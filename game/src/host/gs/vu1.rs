//! VU1, the PS2's vertex unit: what VIF1 does to it and an interpreter for its programs, so the game's own vertex
//! programs (uploaded by the game with MPG) run unchanged and their XGKICKs become GIF packets for the GS.
//! Rust version of the C port's reference (port/src/gs/gs_vu1.c, interpreter path). Drawing, not simulation:
//! the host's float unit, with infinities and NaNs clamped.

#[derive(Clone, Copy, Default)]
#[repr(C)]
struct Reg {
    u: [u32; 4],
}

impl Reg {
    #[inline(always)]
    fn f(&self, c: usize) -> f32 {
        f32::from_bits(self.u[c])
    }
    #[inline(always)]
    fn setf(&mut self, c: usize, v: f32) {
        self.u[c] = v.to_bits();
    }
}

#[inline(always)]
fn clampf(v: f32) -> f32 {
    let u = v.to_bits();
    if u & 0x7F80_0000 == 0x7F80_0000 {
        f32::from_bits((u & 0x8000_0000) | 0x7F7F_FFFF)
    } else {
        v
    }
}

pub struct Vu1 {
    vf: [Reg; 32],
    acc: Reg,
    vi: [u16; 16],
    q: f32,
    i: f32,
    p: f32,
    q_pending: f32,
    q_ready: u32,
    cycle: u32,
    clip: u32,
    clip_latest: u32,
    clip_queue: [(u32, u32); 8],
    clip_head: usize,
    clip_count: usize,
    pub(super) pc: u32,
    pub mem: Vec<u8>,
    micro: Vec<u8>,
    pub(super) top: u32,
    tops: u32,
    base: u32,
    ofst: u32,
    dbf: u32,
    itop: u32,
    itops: u32,
    mode: u32,
    mask: u32,
    cl: u32,
    wl: u32,
    row: [u32; 4],
    col: [u32; 4],
    unknown: [[u32; 128]; 2],
    /// GIF packets the programs kicked, in order (taken by the GS after each call).
    pub kicked: Vec<u8>,
    pub stat_runs: u32,
    pub stat_instr: u32,
    pub stat_kicks: u32,
    /// Which program is loaded: its size in instructions (an upload at 0 starts a new one).
    pub prog_size: u32,
}

impl Default for Vu1 {
    fn default() -> Self {
        Self::new()
    }
}

impl Vu1 {
    pub fn new() -> Self {
        let mut v = Vu1 {
            vf: [Reg::default(); 32],
            acc: Reg::default(),
            vi: [0; 16],
            q: 0.0,
            i: 0.0,
            p: 0.0,
            q_pending: 0.0,
            q_ready: 0,
            cycle: 0,
            clip: 0,
            clip_latest: 0,
            clip_queue: [(0, 0); 8],
            clip_head: 0,
            clip_count: 0,
            pc: 0,
            mem: vec![0; 0x4000],
            micro: vec![0; 0x4000],
            top: 0,
            tops: 0,
            base: 0,
            ofst: 0,
            dbf: 0,
            itop: 0,
            itops: 0,
            mode: 0,
            mask: 0,
            cl: 0,
            wl: 0,
            row: [0; 4],
            col: [0; 4],
            unknown: [[0; 128]; 2],
            kicked: Vec::new(),
            stat_runs: 0,
            stat_instr: 0,
            stat_kicks: 0,
            prog_size: 0,
        };
        v.vf[0].setf(3, 1.0);
        v
    }

    // ------------------------------------------------------------------------------------------ VIF1 side
    pub fn set_cycle(&mut self, cl: u32, wl: u32) {
        self.cl = cl;
        self.wl = wl;
    }
    pub fn set_base(&mut self, v: u32) {
        self.base = v & 0x3FF;
    }
    pub fn set_offset(&mut self, v: u32) {
        self.ofst = v & 0x3FF;
        self.dbf = 0;
        self.tops = self.base;
    }
    pub fn set_itop(&mut self, v: u32) {
        self.itops = v & 0x3FF;
    }
    pub fn set_mode(&mut self, v: u32) {
        self.mode = v & 3;
    }
    pub fn set_mask(&mut self, v: u32) {
        self.mask = v;
    }
    pub fn set_row(&mut self, v: &[u32]) {
        self.row.copy_from_slice(&v[..4]);
    }
    pub fn set_col(&mut self, v: &[u32]) {
        self.col.copy_from_slice(&v[..4]);
    }

    pub fn program(&mut self, addr: u32, words: &[u32]) {
        self.prog_size = if addr == 0 { words.len() as u32 / 2 } else { self.prog_size + words.len() as u32 / 2 };
        for (i, w) in words.iter().enumerate() {
            let a = ((addr as usize * 8) + i * 4) & 0x3FFC;
            self.micro[a..a + 4].copy_from_slice(&w.to_le_bytes());
        }
    }

    /// Words of data an UNPACK command with these parameters is followed by.
    pub fn unpack_words(&self, cmd: u32, num: u32) -> u32 {
        let vn = (cmd >> 2) & 3;
        let vl = cmd & 3;
        let mut n = if num != 0 { num } else { 256 };
        let bits = if vn == 3 && vl == 3 { 16 } else { (32u32 >> vl) * (vn + 1) };
        if self.wl > self.cl && self.wl != 0 {
            n = self.cl * (n / self.wl) + if n % self.wl > self.cl { self.cl } else { n % self.wl };
        }
        (n * bits + 31) / 32
    }

    pub fn unpack(&mut self, cmd: u32, num: u32, imm: u32, data: &[u32]) {
        let vn = (cmd >> 2) & 3;
        let vl = cmd & 3;
        let masked = (cmd >> 4) & 1 != 0;
        let usn = (imm >> 14) & 1 != 0;
        let mut addr = (imm & 0x3FF) + if (imm >> 15) & 1 != 0 { self.tops } else { 0 };
        let n = if num != 0 { num } else { 256 };
        let cl = if self.cl != 0 { self.cl } else { 1 };
        let wl = if self.wl != 0 { self.wl } else { 1 };
        let src = super::as_bytes(data);
        // the common case: whole 32-bit vectors, unmasked, written as they come
        if vn == 3 && vl == 0 && !masked && self.mode == 0 && wl <= cl {
            let mut cyc = 0u32;
            for k in 0..n as usize {
                if src.len() < k * 16 + 16 {
                    break;
                }
                let at = ((addr & 0x3FF) * 16) as usize;
                self.mem[at..at + 16].copy_from_slice(&src[k * 16..k * 16 + 16]);
                addr += 1;
                cyc += 1;
                if cyc == wl {
                    addr += cl - wl;
                    cyc = 0;
                }
            }
            return;
        }
        let byte = |i: usize| -> u8 { src.get(i).copied().unwrap_or(0) };
        let mut cyc = 0u32;
        let mut bitpos = 0usize;
        for _ in 0..n {
            let mut v = [0u32; 4];
            let have = !(wl > cl && cyc >= cl);
            if have {
                if vn == 3 && vl == 3 {
                    let w = byte(bitpos / 8) as u32 | (byte(bitpos / 8 + 1) as u32) << 8;
                    v = [(w & 31) << 3, ((w >> 5) & 31) << 3, ((w >> 10) & 31) << 3, ((w >> 15) & 1) << 7];
                    bitpos += 16;
                } else {
                    let size = 32usize >> vl;
                    for c in 0..=vn as usize {
                        let mut w = 0u32;
                        for b in 0..size / 8 {
                            w |= (byte(bitpos / 8 + b) as u32) << (8 * b);
                        }
                        if !usn && size == 16 {
                            w = w as u16 as i16 as i32 as u32;
                        }
                        if !usn && size == 8 {
                            w = w as u8 as i8 as i32 as u32;
                        }
                        v[c] = w;
                        bitpos += size;
                    }
                    if vn == 0 {
                        v[1] = v[0];
                        v[2] = v[0];
                        v[3] = v[0];
                    }
                }
            }
            let at = ((addr & 0x3FF) * 16) as usize;
            let mut out = [0u32; 4];
            for c in 0..4 {
                let m = if masked { (self.mask >> (2 * (cyc.min(3) * 4 + c as u32))) & 3 } else { 0 };
                out[c] = match m {
                    0 => match self.mode {
                        1 => v[c].wrapping_add(self.row[c]),
                        2 => {
                            self.row[c] = self.row[c].wrapping_add(v[c]);
                            self.row[c]
                        }
                        _ => v[c],
                    },
                    1 => self.row[c],
                    2 => self.col[cyc.min(3) as usize],
                    _ => u32::from_le_bytes(self.mem[at + c * 4..at + c * 4 + 4].try_into().unwrap()),
                };
            }
            for c in 0..4 {
                self.mem[at + c * 4..at + c * 4 + 4].copy_from_slice(&out[c].to_le_bytes());
            }
            addr += 1;
            cyc += 1;
            if cyc == wl {
                if cl > wl {
                    addr += cl - wl;
                }
                cyc = 0;
            }
        }
    }

    // ------------------------------------------------------------------------------------------ interpreter
    fn kick(&mut self, addr: u32) {
        let mut a = ((addr & 0x3FF) * 16) as usize;
        let mut len = 0usize;
        self.stat_kicks += 1;
        loop {
            let lo = u64::from_le_bytes(self.mem[a & 0x3FFF..(a & 0x3FFF) + 8].try_into().unwrap());
            let nloop = (lo & 0x7FFF) as usize;
            let flg = (lo >> 58) & 3;
            let mut nreg = ((lo >> 60) & 15) as usize;
            if nreg == 0 {
                nreg = 16;
            }
            let qw = 1 + if flg == 0 { nloop * nreg } else if flg == 1 { (nloop * nreg + 1) / 2 } else { nloop };
            if len + qw * 16 > 0x4000 {
                break;
            }
            for _ in 0..qw {
                let s = a & 0x3FFF;
                self.kicked.extend_from_slice(&self.mem[s..s + 16]);
                a += 16;
                len += 16;
            }
            if (lo >> 15) & 1 != 0 {
                break;
            }
        }
    }

    #[inline(always)]
    fn mem_at(a: u32) -> usize {
        ((a & 0x3FF) * 16) as usize
    }

    fn load_q(&self, a: u32) -> Reg {
        let o = Self::mem_at(a);
        let mut r = Reg::default();
        for c in 0..4 {
            r.u[c] = u32::from_le_bytes(self.mem[o + c * 4..o + c * 4 + 4].try_into().unwrap());
        }
        r
    }

    fn store_word(&mut self, a: u32, c: usize, w: u32) {
        let o = Self::mem_at(a) + c * 4;
        self.mem[o..o + 4].copy_from_slice(&w.to_le_bytes());
    }

    fn setvi(&mut self, r: u32, v: u32) {
        if r & 15 != 0 {
            self.vi[(r & 15) as usize] = v as u16;
        }
    }

    /// Runs the program at instruction `start` until its end (E bit).
    pub(super) fn run(&mut self, start: u32) {
        let mut ending = false;
        let mut branch = 0;
        let mut target = 0u32;
        self.pc = start & 0x7FF;
        self.stat_runs += 1;
        for _guard in 0..4_000_000u32 {
            let pcb = (self.pc * 8) as usize;
            let lo = u32::from_le_bytes(self.micro[pcb..pcb + 4].try_into().unwrap());
            let hi = u32::from_le_bytes(self.micro[pcb + 4..pcb + 8].try_into().unwrap());
            let next = (self.pc + 1) & 0x7FF;
            self.cycle = self.cycle.wrapping_add(1);
            self.stat_instr += 1;
            if self.q_ready != 0 && self.cycle >= self.q_ready {
                self.q = self.q_pending;
                self.q_ready = 0;
            }
            while self.clip_count != 0 && self.clip_queue[self.clip_head].0 <= self.cycle {
                self.clip = self.clip_queue[self.clip_head].1;
                self.clip_head = (self.clip_head + 1) & 7;
                self.clip_count -= 1;
            }
            let dest = (hi >> 21) & 15;
            let ft = ((hi >> 16) & 31) as usize;
            let fs = ((hi >> 11) & 31) as usize;
            let fd = ((hi >> 6) & 31) as usize;
            let op = hi & 0x3F;
            let bc = (hi & 3) as usize;
            let mut lower = true;
            if hi & 0x8000_0000 != 0 {
                self.i = f32::from_bits(lo);
                lower = false;
            }
            let dbit = |c: usize| (dest >> (3 - c)) & 1 != 0;
            // ---- upper: result in `res`, committed after the lower instruction has read its operands
            let mut res = Reg::default();
            let mut wr: Option<usize> = None;
            let mut wacc = false;
            let mut wmask = 0u32;
            if hi != 0x0000_02FF {
                let a = self.vf[fs];
                let b = self.vf[ft];
                let acc = self.acc;
                let q = self.q;
                let ii = self.i;
                res = if op >= 0x3C { acc } else { self.vf[fd] };
                macro_rules! each {
                    ($e:expr) => {
                        for c in 0..4 {
                            if dbit(c) {
                                let cc = c;
                                let _ = cc;
                                res.setf(c, $e(c));
                            }
                        }
                    };
                }
                let bcv = b.f(bc);
                if op < 0x3C {
                    let mut to_fd = true;
                    match op {
                        0x00..=0x03 => each!(|c| clampf(a.f(c) + bcv)),
                        0x04..=0x07 => each!(|c| clampf(a.f(c) - bcv)),
                        0x08..=0x0B => each!(|c| clampf(acc.f(c) + a.f(c) * bcv)),
                        0x0C..=0x0F => each!(|c| clampf(acc.f(c) - a.f(c) * bcv)),
                        0x10..=0x13 => each!(|c| if a.f(c) > bcv { a.f(c) } else { bcv }),
                        0x14..=0x17 => each!(|c| if a.f(c) < bcv { a.f(c) } else { bcv }),
                        0x18..=0x1B => each!(|c| clampf(a.f(c) * bcv)),
                        0x1C => each!(|c| clampf(a.f(c) * q)),
                        0x1D => each!(|c| if a.f(c) > ii { a.f(c) } else { ii }),
                        0x1E => each!(|c| clampf(a.f(c) * ii)),
                        0x1F => each!(|c| if a.f(c) < ii { a.f(c) } else { ii }),
                        0x20 => each!(|c| clampf(a.f(c) + q)),
                        0x21 => each!(|c| clampf(acc.f(c) + a.f(c) * q)),
                        0x22 => each!(|c| clampf(a.f(c) + ii)),
                        0x23 => each!(|c| clampf(acc.f(c) + a.f(c) * ii)),
                        0x24 => each!(|c| clampf(a.f(c) - q)),
                        0x25 => each!(|c| clampf(acc.f(c) - a.f(c) * q)),
                        0x26 => each!(|c| clampf(a.f(c) - ii)),
                        0x27 => each!(|c| clampf(acc.f(c) - a.f(c) * ii)),
                        0x28 => each!(|c| clampf(a.f(c) + b.f(c))),
                        0x29 => each!(|c| clampf(acc.f(c) + a.f(c) * b.f(c))),
                        0x2A => each!(|c| clampf(a.f(c) * b.f(c))),
                        0x2B => each!(|c| if a.f(c) > b.f(c) { a.f(c) } else { b.f(c) }),
                        0x2C => each!(|c| clampf(a.f(c) - b.f(c))),
                        0x2D => each!(|c| clampf(acc.f(c) - a.f(c) * b.f(c))),
                        0x2E => {
                            res.setf(0, clampf(acc.f(0) - a.f(1) * b.f(2)));
                            res.setf(1, clampf(acc.f(1) - a.f(2) * b.f(0)));
                            res.setf(2, clampf(acc.f(2) - a.f(0) * b.f(1)));
                            wr = Some(fd);
                            wmask = 0xE;
                            to_fd = false;
                        }
                        0x2F => each!(|c| if a.f(c) < b.f(c) { a.f(c) } else { b.f(c) }),
                        _ => {
                            self.unknown[0][op as usize] += 1;
                            to_fd = false;
                        }
                    }
                    if to_fd {
                        wr = Some(fd);
                        wmask = dest;
                    }
                } else {
                    let sp = ((fd as u32) << 2) | bc as u32;
                    let mut to_acc = true;
                    match sp {
                        0x00..=0x03 => each!(|c| clampf(a.f(c) + bcv)),
                        0x04..=0x07 => each!(|c| clampf(a.f(c) - bcv)),
                        0x08..=0x0B => each!(|c| clampf(acc.f(c) + a.f(c) * bcv)),
                        0x0C..=0x0F => each!(|c| clampf(acc.f(c) - a.f(c) * bcv)),
                        0x10..=0x13 => {
                            let k = [1.0f32, 1.0 / 16.0, 1.0 / 4096.0, 1.0 / 32768.0][(sp - 0x10) as usize];
                            res = self.vf[ft];
                            each!(|c| (a.u[c] as i32) as f32 * k);
                            wr = Some(ft);
                            wmask = dest;
                            to_acc = false;
                        }
                        0x14..=0x17 => {
                            let k = [1.0f64, 16.0, 4096.0, 32768.0][(sp - 0x14) as usize];
                            res = self.vf[ft];
                            for c in 0..4 {
                                if dbit(c) {
                                    let v = a.f(c) as f64 * k;
                                    let i = if v >= 2147483647.0 { 0x7FFF_FFFF } else if v <= -2147483648.0 { i32::MIN } else { v as i32 };
                                    res.u[c] = i as u32;
                                }
                            }
                            wr = Some(ft);
                            wmask = dest;
                            to_acc = false;
                        }
                        0x18..=0x1B => each!(|c| clampf(a.f(c) * bcv)),
                        0x1C => each!(|c| clampf(a.f(c) * q)),
                        0x1D => {
                            res = self.vf[ft];
                            for c in 0..4 {
                                if dbit(c) {
                                    res.u[c] = a.u[c] & 0x7FFF_FFFF;
                                }
                            }
                            wr = Some(ft);
                            wmask = dest;
                            to_acc = false;
                        }
                        0x1E => each!(|c| clampf(a.f(c) * ii)),
                        0x1F => {
                            let w = b.f(3).abs();
                            let f = (a.f(0) > w) as u32
                                | ((a.f(0) < -w) as u32) << 1
                                | ((a.f(1) > w) as u32) << 2
                                | ((a.f(1) < -w) as u32) << 3
                                | ((a.f(2) > w) as u32) << 4
                                | ((a.f(2) < -w) as u32) << 5;
                            let slot = (self.clip_head + self.clip_count) & 7;
                            self.clip_latest = ((self.clip_latest << 6) | f) & 0xFF_FFFF;
                            self.clip_queue[slot] = (self.cycle.wrapping_add(4), self.clip_latest);
                            if self.clip_count < 8 {
                                self.clip_count += 1;
                            }
                            to_acc = false;
                        }
                        0x20 => each!(|c| clampf(a.f(c) + q)),
                        0x21 => each!(|c| clampf(acc.f(c) + a.f(c) * q)),
                        0x22 => each!(|c| clampf(a.f(c) + ii)),
                        0x23 => each!(|c| clampf(acc.f(c) + a.f(c) * ii)),
                        0x24 => each!(|c| clampf(a.f(c) - q)),
                        0x26 => each!(|c| clampf(a.f(c) - ii)),
                        0x28 => each!(|c| clampf(a.f(c) + b.f(c))),
                        0x29 => each!(|c| clampf(acc.f(c) + a.f(c) * b.f(c))),
                        0x2A => each!(|c| clampf(a.f(c) * b.f(c))),
                        0x2C => each!(|c| clampf(a.f(c) - b.f(c))),
                        0x2D => each!(|c| clampf(acc.f(c) - a.f(c) * b.f(c))),
                        0x2E => {
                            res.setf(0, clampf(a.f(1) * b.f(2)));
                            res.setf(1, clampf(a.f(2) * b.f(0)));
                            res.setf(2, clampf(a.f(0) * b.f(1)));
                            wacc = true;
                            wmask = 0xE;
                            to_acc = false;
                        }
                        0x2F => to_acc = false,
                        _ => {
                            self.unknown[0][64 + (sp & 63) as usize] += 1;
                            to_acc = false;
                        }
                    }
                    if to_acc {
                        wacc = true;
                        wmask = dest;
                    }
                }
            }
            // ---- lower instruction
            if lower && lo != 0x8000_033C {
                let lop = lo >> 25;
                let it = (lo >> 16) & 31;
                let is = (lo >> 11) & 31;
                let id = (lo >> 6) & 31;
                let ld = (lo >> 21) & 15;
                let imm11 = ((lo << 21) as i32) >> 21;
                let imm15 = ((lo >> 10) & 0x7800) | (lo & 0x7FF);
                let lbit = |c: usize| (ld >> (3 - c)) & 1 != 0;
                let vi = |s: &Self, r: u32| s.vi[(r & 15) as usize] as u32;
                let rel = |s: &Self| (s.pc as i32 + 1 + imm11) as u32 & 0x7FF;
                match lop {
                    0x00 => {
                        let m = self.load_q((vi(self, is) as i32 + imm11) as u32);
                        if it != 0 {
                            for c in 0..4 {
                                if lbit(c) {
                                    self.vf[it as usize].u[c] = m.u[c];
                                }
                            }
                        }
                    }
                    0x01 => {
                        let a = (vi(self, it) as i32 + imm11) as u32;
                        let s = self.vf[is as usize];
                        for c in 0..4 {
                            if lbit(c) {
                                self.store_word(a, c, s.u[c]);
                            }
                        }
                    }
                    0x04 => {
                        let m = self.load_q((vi(self, is) as i32 + imm11) as u32);
                        for c in 0..4 {
                            if lbit(c) {
                                self.setvi(it, m.u[c]);
                            }
                        }
                    }
                    0x05 => {
                        let a = (vi(self, is) as i32 + imm11) as u32;
                        let w = vi(self, it);
                        for c in 0..4 {
                            if lbit(c) {
                                self.store_word(a, c, w);
                            }
                        }
                    }
                    0x08 => self.setvi(it, vi(self, is).wrapping_add(imm15)),
                    0x09 => self.setvi(it, vi(self, is).wrapping_sub(imm15)),
                    0x10 => self.setvi(1, ((self.clip & 0xFF_FFFF) == (lo & 0xFF_FFFF)) as u32),
                    0x11 => {
                        self.clip = lo & 0xFF_FFFF;
                        self.clip_latest = self.clip;
                        self.clip_count = 0;
                    }
                    0x12 => self.setvi(1, ((self.clip & lo & 0xFF_FFFF) != 0) as u32),
                    0x13 => self.setvi(1, (((self.clip | lo) & 0xFF_FFFF) == 0xFF_FFFF) as u32),
                    0x1C => self.setvi(it, self.clip & 0xFFF),
                    0x20 => {
                        branch = 2;
                        target = rel(self);
                    }
                    0x21 => {
                        self.setvi(it, (self.pc + 2) & 0x7FF);
                        branch = 2;
                        target = rel(self);
                    }
                    0x24 => {
                        branch = 2;
                        target = vi(self, is) & 0x7FF;
                    }
                    0x25 => {
                        let t = vi(self, is) & 0x7FF;
                        self.setvi(it, (self.pc + 2) & 0x7FF);
                        branch = 2;
                        target = t;
                    }
                    0x28 | 0x29 | 0x2C | 0x2D | 0x2E | 0x2F => {
                        let a = vi(self, is) as u16 as i16;
                        let take = match lop {
                            0x28 => vi(self, it) == vi(self, is),
                            0x29 => vi(self, it) != vi(self, is),
                            0x2C => a < 0,
                            0x2D => a > 0,
                            0x2E => a <= 0,
                            _ => a >= 0,
                        };
                        if take {
                            branch = 2;
                            target = rel(self);
                        }
                    }
                    0x40 => {
                        let f = lo & 0x3F;
                        if f < 0x3C {
                            match f {
                                0x30 => self.setvi(id, vi(self, is).wrapping_add(vi(self, it))),
                                0x31 => self.setvi(id, vi(self, is).wrapping_sub(vi(self, it))),
                                0x32 => self.setvi(it, vi(self, is).wrapping_add((((lo << 21) as i32) >> 27) as u32)),
                                0x34 => self.setvi(id, vi(self, is) & vi(self, it)),
                                0x35 => self.setvi(id, vi(self, is) | vi(self, it)),
                                _ => self.unknown[1][f as usize] += 1,
                            }
                        } else {
                            let sp = (id << 2) | (lo & 3);
                            let fsf = ((lo >> 21) & 3) as usize;
                            let ftf = ((lo >> 23) & 3) as usize;
                            let (itu, isu) = (it as usize, is as usize);
                            match sp {
                                0x30 => {
                                    if it != 0 {
                                        let s = self.vf[isu];
                                        for c in 0..4 {
                                            if lbit(c) {
                                                self.vf[itu].u[c] = s.u[c];
                                            }
                                        }
                                    }
                                }
                                0x31 => {
                                    if it != 0 {
                                        let s = self.vf[isu];
                                        let rot = [s.u[1], s.u[2], s.u[3], s.u[0]];
                                        for c in 0..4 {
                                            if lbit(c) {
                                                self.vf[itu].u[c] = rot[c];
                                            }
                                        }
                                    }
                                }
                                0x34 => {
                                    let m = self.load_q(vi(self, is));
                                    if it != 0 {
                                        for c in 0..4 {
                                            if lbit(c) {
                                                self.vf[itu].u[c] = m.u[c];
                                            }
                                        }
                                    }
                                    self.setvi(is, vi(self, is).wrapping_add(1));
                                }
                                0x35 => {
                                    let a = vi(self, it);
                                    let s = self.vf[isu];
                                    for c in 0..4 {
                                        if lbit(c) {
                                            self.store_word(a, c, s.u[c]);
                                        }
                                    }
                                    self.setvi(it, vi(self, it).wrapping_add(1));
                                }
                                0x36 => {
                                    self.setvi(is, vi(self, is).wrapping_sub(1));
                                    let m = self.load_q(vi(self, is));
                                    if it != 0 {
                                        for c in 0..4 {
                                            if lbit(c) {
                                                self.vf[itu].u[c] = m.u[c];
                                            }
                                        }
                                    }
                                }
                                0x37 => {
                                    self.setvi(it, vi(self, it).wrapping_sub(1));
                                    let a = vi(self, it);
                                    let s = self.vf[isu];
                                    for c in 0..4 {
                                        if lbit(c) {
                                            self.store_word(a, c, s.u[c]);
                                        }
                                    }
                                }
                                0x38 => {
                                    let d = self.vf[itu].f(ftf);
                                    let n = self.vf[isu].f(fsf);
                                    self.q_pending = if d != 0.0 {
                                        clampf(n / d)
                                    } else if (n < 0.0) != (d < 0.0) || (n < 0.0 && d == 0.0) {
                                        -3.402_823_5e38
                                    } else {
                                        3.402_823_5e38
                                    };
                                    self.q_ready = self.cycle.wrapping_add(7);
                                }
                                0x39 => {
                                    self.q_pending = self.vf[itu].f(ftf).abs().sqrt();
                                    self.q_ready = self.cycle.wrapping_add(7);
                                }
                                0x3A => {
                                    let d = self.vf[itu].f(ftf).abs().sqrt();
                                    self.q_pending = if d != 0.0 { clampf(self.vf[isu].f(fsf) / d) } else { 3.402_823_5e38 };
                                    self.q_ready = self.cycle.wrapping_add(13);
                                }
                                0x3B => {
                                    if self.q_ready != 0 {
                                        self.q = self.q_pending;
                                        self.q_ready = 0;
                                    }
                                }
                                0x3C => self.setvi(it, self.vf[isu].u[fsf]),
                                0x3D => {
                                    if it != 0 {
                                        let v = vi(self, is) as u16 as i16 as i32 as u32;
                                        for c in 0..4 {
                                            if lbit(c) {
                                                self.vf[itu].u[c] = v;
                                            }
                                        }
                                    }
                                }
                                0x3E => {
                                    let m = self.load_q(vi(self, is));
                                    for c in 0..4 {
                                        if lbit(c) {
                                            self.setvi(it, m.u[c]);
                                        }
                                    }
                                }
                                0x3F => {
                                    let a = vi(self, is);
                                    let w = vi(self, it);
                                    for c in 0..4 {
                                        if lbit(c) {
                                            self.store_word(a, c, w);
                                        }
                                    }
                                }
                                0x64 => {
                                    if it != 0 {
                                        let p = self.p;
                                        for c in 0..4 {
                                            if lbit(c) {
                                                self.vf[itu].setf(c, p);
                                            }
                                        }
                                    }
                                }
                                0x68 => self.setvi(it, self.top),
                                0x69 => self.setvi(it, self.itop),
                                0x6C => self.kick(vi(self, is)),
                                0x7B => {}
                                _ => self.unknown[1][64 + (sp & 63) as usize] += 1,
                            }
                        }
                    }
                    _ => self.unknown[1][(lop & 63) as usize] += 1,
                }
            }
            // ---- commit the upper result
            if let Some(r) = wr {
                if r != 0 {
                    for c in 0..4 {
                        if (wmask >> (3 - c)) & 1 != 0 {
                            self.vf[r].u[c] = res.u[c];
                        }
                    }
                }
            } else if wacc {
                for c in 0..4 {
                    if (wmask >> (3 - c)) & 1 != 0 {
                        self.acc.u[c] = res.u[c];
                    }
                }
            }
            self.vf[0] = Reg::default();
            self.vf[0].setf(3, 1.0);
            // ---- next instruction: branches and the end take effect after one more instruction
            if ending {
                self.pc = next;
                return;
            }
            if hi & 0x4000_0000 != 0 {
                ending = true;
            }
            if branch == 1 {
                self.pc = target;
                branch = 0;
            } else {
                if branch == 2 {
                    branch = 1;
                }
                self.pc = next;
            }
        }
        eprintln!("vu1: program at {start} did not end (pc {})", self.pc);
    }

    /// MSCAL / MSCALF (addr = instruction address) or MSCNT (addr < 0: go on where the last run stopped).
    pub fn call(&mut self, addr: i32) {
        self.swap_buffers();
        let start = if addr < 0 { self.pc } else { addr as u32 };
        self.run(start);
    }

    /// What MSCAL / MSCNT do to the double buffer before the program starts.
    pub fn swap_buffers(&mut self) {
        self.top = self.tops;
        self.itop = self.itops;
        if self.dbf != 0 {
            self.tops = self.base;
            self.dbf = 0;
        } else {
            self.tops = self.base + self.ofst;
            self.dbf = 1;
        }
    }

    pub fn frame_end(&mut self, verbose: bool) {
        for k in 0..2 {
            for i in 0..128 {
                if self.unknown[k][i] != 0 {
                    if verbose {
                        eprintln!(
                            "vu1: unimplemented {} instruction {}{:02x}, {} times",
                            if k == 1 { "lower" } else { "upper" },
                            if i >= 64 { "special " } else { "" },
                            i & 63,
                            self.unknown[k][i]
                        );
                    }
                    self.unknown[k][i] = 0;
                }
            }
        }
        self.stat_runs = 0;
        self.stat_instr = 0;
        self.stat_kicks = 0;
    }
}
