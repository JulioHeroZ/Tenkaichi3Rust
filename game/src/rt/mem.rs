//! The PS2 address space: 4 GB of 64 KB pages allocated on first touch.
const PAGE_BITS: u32 = 16;

/// BT3_WATCH=<addr>: every write touching these 4 bytes is reported with the C functions on the stack.
pub static WATCH: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);

#[cold]
#[inline(never)]
fn watch_hit(m: &Mem, a: u32, n: u32) {
    let w = WATCH.load(std::sync::atomic::Ordering::Relaxed);
    let bt = std::backtrace::Backtrace::force_capture().to_string();
    let names: Vec<String> = bt.lines().map(|l| l.trim()).filter(|l| !l.starts_with("at ") && !l.contains("rt::mem") && !l.contains("std::") && !l.contains("core::"))
        .map(|l| l.split(": ").last().unwrap_or(l).rsplit("::").next().unwrap_or(l).to_string()).take(8).collect();
    eprintln!("watch {w:08x} <- write {a:08x}+{n}: {} by {}", 0, names.join(" < "));
    let _ = m;
}

#[inline(always)]
fn watched(a: u32, n: u32) -> bool {
    let w = WATCH.load(std::sync::atomic::Ordering::Relaxed);
    w != 0 && w.wrapping_sub(a) < n
}
const PAGE: usize = 1 << PAGE_BITS;

/// The PS2's 32 MB of RAM, addressed directly (most accesses); everything above goes through pages.
pub const RAM: u32 = 0x0400_0000;

pub struct Mem {
    ram: Vec<u8>,
    pages: Vec<Option<Box<[u8; PAGE]>>>,
    /// Pages written since the last call to `take_dirty` are not tracked: whole-memory compares are rare.
    pub reads_unmapped: u64,
}

impl Default for Mem {
    fn default() -> Self {
        Self::new()
    }
}

impl Mem {
    pub fn new() -> Self {
        let mut pages = Vec::with_capacity(1 << (32 - PAGE_BITS));
        pages.resize_with(1 << (32 - PAGE_BITS), || None);
        Mem { ram: vec![0u8; RAM as usize + 16], pages, reads_unmapped: 0 }
    }

    #[inline(always)]
    fn page(&self, a: u32) -> Option<&[u8; PAGE]> {
        self.pages[(a >> PAGE_BITS) as usize].as_deref()
    }

    #[inline(always)]
    fn page_mut(&mut self, a: u32) -> &mut [u8; PAGE] {
        let slot = &mut self.pages[(a >> PAGE_BITS) as usize];
        if slot.is_none() {
            *slot = Some(vec![0u8; PAGE].into_boxed_slice().try_into().unwrap());
        }
        slot.as_deref_mut().unwrap()
    }

    #[inline(always)]
    pub fn r8(&self, a: u32) -> u8 {
        if a < RAM {
            return self.ram[a as usize];
        }
        match self.page(a) {
            Some(p) => p[(a as usize) & (PAGE - 1)],
            None => 0,
        }
    }

    #[inline(always)]
    pub fn w8(&mut self, a: u32, v: u8) {
        if watched(a, 1) { watch_hit(self, a, 1); }
        if a < RAM {
            self.ram[a as usize] = v;
            return;
        }
        self.page_mut(a)[(a as usize) & (PAGE - 1)] = v;
    }

    #[inline(always)]
    fn read<const N: usize>(&self, a: u32) -> [u8; N] {
        if a < RAM {
            let a = a as usize;
            return self.ram[a..a + N].try_into().unwrap();
        }
        let o = (a as usize) & (PAGE - 1);
        if o + N <= PAGE {
            if let Some(p) = self.page(a) {
                let mut b = [0u8; N];
                b.copy_from_slice(&p[o..o + N]);
                return b;
            }
            return [0u8; N];
        }
        let mut b = [0u8; N];
        for (i, x) in b.iter_mut().enumerate() {
            *x = self.r8(a.wrapping_add(i as u32));
        }
        b
    }

    #[inline(always)]
    fn write<const N: usize>(&mut self, a: u32, b: [u8; N]) {
        if watched(a, N as u32) { watch_hit(self, a, N as u32); }
        if a < RAM {
            let a = a as usize;
            self.ram[a..a + N].copy_from_slice(&b);
            return;
        }
        let o = (a as usize) & (PAGE - 1);
        if o + N <= PAGE {
            self.page_mut(a)[o..o + N].copy_from_slice(&b);
            return;
        }
        for (i, x) in b.iter().enumerate() {
            self.w8(a.wrapping_add(i as u32), *x);
        }
    }

    #[inline(always)]
    pub fn r16(&self, a: u32) -> u16 { u16::from_le_bytes(self.read(a)) }
    #[inline(always)]
    pub fn r32(&self, a: u32) -> u32 { u32::from_le_bytes(self.read(a)) }
    #[inline(always)]
    pub fn r64(&self, a: u32) -> u64 { u64::from_le_bytes(self.read(a)) }
    #[inline(always)]
    pub fn r128(&self, a: u32) -> u128 { u128::from_le_bytes(self.read(a)) }
    #[inline(always)]
    pub fn w16(&mut self, a: u32, v: u16) { self.write(a, v.to_le_bytes()) }
    #[inline(always)]
    pub fn w32(&mut self, a: u32, v: u32) { self.write(a, v.to_le_bytes()) }
    #[inline(always)]
    pub fn w64(&mut self, a: u32, v: u64) { self.write(a, v.to_le_bytes()) }
    #[inline(always)]
    pub fn w128(&mut self, a: u32, v: u128) { self.write(a, v.to_le_bytes()) }

    /// Host address of the RAM block (for debugger watchpoints).
    pub fn ram_ptr(&self) -> usize { self.ram.as_ptr() as usize }

    #[inline]
    pub fn read_bytes(&self, a: u32, out: &mut [u8]) {
        let n = out.len();
        if (a as u64) + (n as u64) <= RAM as u64 {
            out.copy_from_slice(&self.ram[a as usize..a as usize + n]);
            return;
        }
        let mut off = 0usize;
        while off < n {
            let addr = a.wrapping_add(off as u32);
            if addr < RAM {
                let k = ((RAM - addr) as usize).min(n - off);
                out[off..off + k].copy_from_slice(&self.ram[addr as usize..addr as usize + k]);
                off += k;
                continue;
            }
            let o = (addr as usize) & (PAGE - 1);
            let k = (PAGE - o).min(n - off);
            match self.page(addr) {
                Some(p) => out[off..off + k].copy_from_slice(&p[o..o + k]),
                None => out[off..off + k].fill(0),
            }
            off += k;
        }
    }

    pub fn write_bytes(&mut self, a: u32, data: &[u8]) {
        let n = data.len();
        if watched(a, n as u32) { watch_hit(self, a, n as u32); }
        if (a as u64) + (n as u64) <= RAM as u64 {
            self.ram[a as usize..a as usize + n].copy_from_slice(data);
            return;
        }
        let mut off = 0usize;
        while off < n {
            let addr = a.wrapping_add(off as u32);
            if addr < RAM {
                let k = ((RAM - addr) as usize).min(n - off);
                self.ram[addr as usize..addr as usize + k].copy_from_slice(&data[off..off + k]);
                off += k;
                continue;
            }
            let o = (addr as usize) & (PAGE - 1);
            let k = (PAGE - o).min(n - off);
            self.page_mut(addr)[o..o + k].copy_from_slice(&data[off..off + k]);
            off += k;
        }
    }

    #[inline]
    pub fn copy(&mut self, dst: u32, src: u32, n: u32) {
        if n == 0 || dst == src {
            return;
        }
        if watched(dst, n) { watch_hit(self, dst, n); }
        if (src as u64) + (n as u64) <= RAM as u64 && (dst as u64) + (n as u64) <= RAM as u64 {
            self.ram.copy_within(src as usize..(src + n) as usize, dst as usize);
            return;
        }
        // through a bounce buffer, in chunks; overlapping ranges copied as memmove
        let mut buf = [0u8; 4096];
        let chunks: Vec<u32> = (0..n).step_by(4096).collect();
        let order: Box<dyn Iterator<Item = &u32>> = if dst > src { Box::new(chunks.iter().rev()) } else { Box::new(chunks.iter()) };
        for &off in order {
            let k = (n - off).min(4096) as usize;
            self.read_bytes(src.wrapping_add(off), &mut buf[..k]);
            self.write_bytes(dst.wrapping_add(off), &buf[..k]);
        }
    }

    #[inline]
    pub fn fill(&mut self, dst: u32, v: u8, n: u32) {
        if watched(dst, n) { watch_hit(self, dst, n); }
        if (dst as u64) + (n as u64) <= RAM as u64 {
            self.ram[dst as usize..(dst + n) as usize].fill(v);
            return;
        }
        let mut off = 0u32;
        while off < n {
            let addr = dst.wrapping_add(off);
            if addr < RAM {
                let k = (RAM - addr).min(n - off);
                self.ram[addr as usize..(addr + k) as usize].fill(v);
                off += k;
                continue;
            }
            let o = (addr as usize) & (PAGE - 1);
            let k = ((PAGE - o) as u32).min(n - off);
            self.page_mut(addr)[o..o + k as usize].fill(v);
            off += k;
        }
    }

    /// NUL-terminated string at `a` (latin-1 bytes).
    pub fn cstr(&self, a: u32) -> Vec<u8> {
        let mut v = Vec::new();
        let mut p = a;
        loop {
            let b = self.r8(p);
            if b == 0 || v.len() > 1 << 20 {
                break;
            }
            v.push(b);
            p = p.wrapping_add(1);
        }
        v
    }
}
