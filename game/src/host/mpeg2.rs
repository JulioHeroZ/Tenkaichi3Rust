//! An MPEG-2 video decoder for the game's movies (assets/video/*.m2v, tools/assets/pss_video.py: the video of the
//! disc's PSS files, unchanged), in Rust without libraries. The movies of Tenkaichi 3 are intra-coded only (every
//! picture an I picture, frame structure, 4:2:0, Main profile), so that is what this decodes: ISO/IEC 13818-2 sequence
//! header and extensions, picture coding extension (intra VLC format, alternate scan, q scale type, DC precision,
//! field DCT), slices, intra macroblocks, inverse quantisation with saturation and mismatch control, and an
//! IEEE 1180 precise inverse DCT. A stream with P or B pictures is refused with an error. The pictures are turned into
//! RGB as the PS2's IPU does (ITU-R BT.601, each chroma sample for its 2x2 pixels).

pub struct Decoder {
    data: Vec<u8>,
    pos: usize,
    pub width: usize,
    pub height: usize,
    intra_q: [u8; 64],
    mpeg2: bool,
    // picture coding extension
    dc_precision: u32,
    frame_dct: bool,
    q_scale_type: bool,
    intra_vlc: bool,
    alternate: bool,
    y: Vec<u8>,
    cb: Vec<u8>,
    cr: Vec<u8>,
}

const ZIGZAG: [u8; 64] = [
    0, 1, 8, 16, 9, 2, 3, 10, 17, 24, 32, 25, 18, 11, 4, 5, 12, 19, 26, 33, 40, 48, 41, 34, 27, 20, 13, 6, 7, 14, 21, 28,
    35, 42, 49, 56, 57, 50, 43, 36, 29, 22, 15, 23, 30, 37, 44, 51, 58, 59, 52, 45, 38, 31, 39, 46, 53, 60, 61, 54, 47,
    55, 62, 63,
];
const ALTERNATE: [u8; 64] = [
    0, 8, 16, 24, 1, 9, 2, 10, 17, 25, 32, 40, 48, 56, 57, 49, 41, 33, 26, 18, 3, 11, 4, 12, 19, 27, 34, 42, 50, 58, 35,
    43, 51, 59, 20, 28, 5, 13, 6, 14, 21, 29, 36, 44, 52, 60, 37, 45, 53, 61, 22, 30, 7, 15, 23, 31, 38, 46, 54, 62, 39,
    47, 55, 63,
];
const DEFAULT_INTRA: [u8; 64] = [
    8, 16, 19, 22, 26, 27, 29, 34, 16, 16, 22, 24, 27, 29, 34, 37, 19, 22, 26, 27, 29, 34, 34, 38, 22, 22, 26, 27, 29, 34,
    37, 40, 22, 26, 27, 29, 32, 35, 40, 48, 26, 27, 29, 32, 35, 40, 48, 58, 26, 27, 29, 34, 38, 46, 56, 69, 27, 29, 35,
    38, 46, 56, 69, 83,
];
const NONLINEAR_Q: [u8; 32] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 10, 12, 14, 16, 18, 20, 22, 24, 28, 32, 36, 40, 44, 48, 52, 56, 64, 72, 80, 88, 96, 104, 112,
];
// dct_dc_size_luminance / _chrominance (tables B.12, B.13): (code, length) per size 0..11
const DC_LUM: [(u16, u8); 12] =
    [(0x4, 3), (0x0, 2), (0x1, 2), (0x5, 3), (0x6, 3), (0xe, 4), (0x1e, 5), (0x3e, 6), (0x7e, 7), (0xfe, 8), (0x1fe, 9), (0x1ff, 9)];
const DC_CHROMA: [(u16, u8); 12] =
    [(0x0, 2), (0x1, 2), (0x2, 2), (0x6, 3), (0xe, 4), (0x1e, 5), (0x3e, 6), (0x7e, 7), (0xfe, 8), (0x1fe, 9), (0x3fe, 10), (0x3ff, 10)];
// macroblock_address_increment (table B.1): increment i + 1
const MB_INCR: [(u16, u8); 33] = [
    (0x1, 1), (0x3, 3), (0x2, 3), (0x3, 4), (0x2, 4), (0x3, 5), (0x2, 5), (0x7, 7), (0x6, 7), (0xb, 8), (0xa, 8),
    (0x9, 8), (0x8, 8), (0x7, 8), (0x6, 8), (0x17, 10), (0x16, 10), (0x15, 10), (0x14, 10), (0x13, 10), (0x12, 10),
    (0x23, 11), (0x22, 11), (0x21, 11), (0x20, 11), (0x1f, 11), (0x1e, 11), (0x1d, 11), (0x1c, 11), (0x1b, 11),
    (0x1a, 11), (0x19, 11), (0x18, 11),
];
// DCT coefficients, tables B.14 and B.15 (the sign bit follows the code): (code, length) per (run, level) of RUN/LEVEL
const B14: [(u16, u8); 111] = [
    (0x3, 2), (0x4, 4), (0x5, 5), (0x6, 7), (0x26, 8), (0x21, 8), (0xa, 10), (0x1d, 12), (0x18, 12), (0x13, 12), (0x10, 12),
    (0x1a, 13), (0x19, 13), (0x18, 13), (0x17, 13), (0x1f, 14), (0x1e, 14), (0x1d, 14), (0x1c, 14), (0x1b, 14), (0x1a, 14),
    (0x19, 14), (0x18, 14), (0x17, 14), (0x16, 14), (0x15, 14), (0x14, 14), (0x13, 14), (0x12, 14), (0x11, 14), (0x10, 14),
    (0x18, 15), (0x17, 15), (0x16, 15), (0x15, 15), (0x14, 15), (0x13, 15), (0x12, 15), (0x11, 15), (0x10, 15), (0x3, 3),
    (0x6, 6), (0x25, 8), (0xc, 10), (0x1b, 12), (0x16, 13), (0x15, 13), (0x1f, 15), (0x1e, 15), (0x1d, 15), (0x1c, 15),
    (0x1b, 15), (0x1a, 15), (0x19, 15), (0x13, 16), (0x12, 16), (0x11, 16), (0x10, 16), (0x5, 4), (0x4, 7), (0xb, 10),
    (0x14, 12), (0x14, 13), (0x7, 5), (0x24, 8), (0x1c, 12), (0x13, 13), (0x6, 5), (0xf, 10), (0x12, 12), (0x7, 6), (0x9, 10),
    (0x12, 13), (0x5, 6), (0x1e, 12), (0x14, 16), (0x4, 6), (0x15, 12), (0x7, 7), (0x11, 12), (0x5, 7), (0x11, 13), (0x27, 8),
    (0x10, 13), (0x23, 8), (0x1a, 16), (0x22, 8), (0x19, 16), (0x20, 8), (0x18, 16), (0xe, 10), (0x17, 16), (0xd, 10),
    (0x16, 16), (0x8, 10), (0x15, 16), (0x1f, 12), (0x1a, 12), (0x19, 12), (0x17, 12), (0x16, 12), (0x1f, 13), (0x1e, 13),
    (0x1d, 13), (0x1c, 13), (0x1b, 13), (0x1f, 16), (0x1e, 16), (0x1d, 16), (0x1c, 16), (0x1b, 16),
];
const B15: [(u16, u8); 111] = [
    (0x02, 2), (0x06, 3), (0x07, 4), (0x1c, 5), (0x1d, 5), (0x05, 6), (0x04, 6), (0x7b, 7), (0x7c, 7), (0x23, 8), (0x22, 8),
    (0xfa, 8), (0xfb, 8), (0xfe, 8), (0xff, 8), (0x1f, 14), (0x1e, 14), (0x1d, 14), (0x1c, 14), (0x1b, 14), (0x1a, 14),
    (0x19, 14), (0x18, 14), (0x17, 14), (0x16, 14), (0x15, 14), (0x14, 14), (0x13, 14), (0x12, 14), (0x11, 14), (0x10, 14),
    (0x18, 15), (0x17, 15), (0x16, 15), (0x15, 15), (0x14, 15), (0x13, 15), (0x12, 15), (0x11, 15), (0x10, 15), (0x02, 3),
    (0x06, 5), (0x79, 7), (0x27, 8), (0x20, 8), (0x16, 13), (0x15, 13), (0x1f, 15), (0x1e, 15), (0x1d, 15), (0x1c, 15),
    (0x1b, 15), (0x1a, 15), (0x19, 15), (0x13, 16), (0x12, 16), (0x11, 16), (0x10, 16), (0x05, 5), (0x07, 7), (0xfc, 8),
    (0x0c, 10), (0x14, 13), (0x07, 5), (0x26, 8), (0x1c, 12), (0x13, 13), (0x06, 6), (0xfd, 8), (0x12, 12), (0x07, 6),
    (0x04, 9), (0x12, 13), (0x06, 7), (0x1e, 12), (0x14, 16), (0x04, 7), (0x15, 12), (0x05, 7), (0x11, 12), (0x78, 7),
    (0x11, 13), (0x7a, 7), (0x10, 13), (0x21, 8), (0x1a, 16), (0x25, 8), (0x19, 16), (0x24, 8), (0x18, 16), (0x05, 9),
    (0x17, 16), (0x07, 9), (0x16, 16), (0x0d, 10), (0x15, 16), (0x1f, 12), (0x1a, 12), (0x19, 12), (0x17, 12), (0x16, 12),
    (0x1f, 13), (0x1e, 13), (0x1d, 13), (0x1c, 13), (0x1b, 13), (0x1f, 16), (0x1e, 16), (0x1d, 16), (0x1c, 16), (0x1b, 16),
];
const LEVEL: [u8; 111] = [
    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33,
    34, 35, 36, 37, 38, 39, 40, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 1, 2, 3, 4, 5, 1, 2, 3, 4, 1, 2,
    3, 1, 2, 3, 1, 2, 3, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
];
const RUN: [u8; 111] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 5, 5, 5, 6, 6, 6, 7, 7, 8, 8, 9, 9,
    10, 10, 11, 11, 12, 12, 13, 13, 14, 14, 15, 15, 16, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31,
];

/// A variable-length code table looked up by its longest code: (index, length) per value of the next `bits` bits.
struct Vlc {
    bits: u32,
    table: Vec<(u16, u8)>,
}

impl Vlc {
    fn new(codes: &[(u16, u8)], extra: &[(u16, u8, u16)]) -> Vlc {
        let bits = codes.iter().map(|c| c.1).chain(extra.iter().map(|c| c.1)).max().unwrap() as u32;
        let mut table = vec![(u16::MAX, 0u8); 1 << bits];
        let mut put = |code: u16, len: u8, value: u16| {
            let shift = bits - len as u32;
            let first = (code as usize) << shift;
            for e in &mut table[first..first + (1 << shift)] {
                *e = (value, len);
            }
        };
        for (i, &(code, len)) in codes.iter().enumerate() {
            put(code, len, i as u16);
        }
        for &(code, len, value) in extra {
            put(code, len, value);
        }
        Vlc { bits, table }
    }
}

struct Tables {
    dc_lum: Vlc,
    dc_chroma: Vlc,
    mb_incr: Vlc,
    b14: Vlc,
    b15: Vlc,
    idct: [[f64; 8]; 8],
}

const ESCAPE: u16 = 1000;
const EOB: u16 = 1001;
const MB_ESCAPE: u16 = 100;
const MB_STUFFING: u16 = 101;

fn tables() -> &'static Tables {
    static T: std::sync::OnceLock<Tables> = std::sync::OnceLock::new();
    T.get_or_init(|| {
        let mut idct = [[0.0; 8]; 8];
        for (x, row) in idct.iter_mut().enumerate() {
            for (u, v) in row.iter_mut().enumerate() {
                let c = if u == 0 { std::f64::consts::FRAC_1_SQRT_2 } else { 1.0 };
                *v = c / 2.0 * (((2 * x + 1) * u) as f64 * std::f64::consts::PI / 16.0).cos();
            }
        }
        Tables {
            dc_lum: Vlc::new(&DC_LUM, &[]),
            dc_chroma: Vlc::new(&DC_CHROMA, &[]),
            mb_incr: Vlc::new(&MB_INCR, &[(0x8, 11, MB_ESCAPE), (0xf, 11, MB_STUFFING)]),
            b14: Vlc::new(&B14, &[(0x1, 6, ESCAPE), (0x2, 2, EOB)]),
            b15: Vlc::new(&B15, &[(0x1, 6, ESCAPE), (0x6, 4, EOB)]),
            idct,
        }
    })
}

/// Bits of a byte slice, most significant first.
struct Bits<'a> {
    d: &'a [u8],
    bit: usize,
}

impl Bits<'_> {
    fn peek(&self, n: u32) -> u32 {
        let mut v: u64 = 0;
        let byte = self.bit >> 3;
        for k in 0..8 {
            v = v << 8 | *self.d.get(byte + k).unwrap_or(&0) as u64;
        }
        ((v << (self.bit & 7)) >> (64 - n)) as u32
    }
    fn get(&mut self, n: u32) -> u32 {
        let v = self.peek(n);
        self.bit += n as usize;
        v
    }
    fn vlc(&mut self, t: &Vlc) -> Result<u16, String> {
        let (v, len) = t.table[self.peek(t.bits) as usize];
        if len == 0 {
            return Err(format!("bad variable-length code at byte {}", self.bit / 8));
        }
        self.bit += len as usize;
        Ok(v)
    }
    fn at_start_code(&self) -> bool {
        self.peek(23) == 0
    }
}

impl Decoder {
    pub fn new(data: Vec<u8>) -> Result<Decoder, String> {
        let mut d = Decoder {
            data,
            pos: 0,
            width: 0,
            height: 0,
            intra_q: DEFAULT_INTRA,
            mpeg2: false,
            dc_precision: 0,
            frame_dct: true,
            q_scale_type: false,
            intra_vlc: false,
            alternate: false,
            y: Vec::new(),
            cb: Vec::new(),
            cr: Vec::new(),
        };
        // the sequence header comes first
        match d.next_start_code() {
            Some(0xB3) => d.sequence_header()?,
            _ => return Err("not an MPEG video stream (no sequence header)".into()),
        }
        Ok(d)
    }

    /// The code of the next start code (00 00 01 xx); pos left after it.
    fn next_start_code(&mut self) -> Option<u8> {
        let d = &self.data;
        let mut i = self.pos;
        while i + 3 < d.len() {
            if d[i] == 0 && d[i + 1] == 0 && d[i + 2] == 1 {
                self.pos = i + 4;
                return Some(d[i + 3]);
            }
            i += 1;
        }
        self.pos = d.len();
        None
    }

    fn sequence_header(&mut self) -> Result<(), String> {
        let mut b = Bits { d: &self.data[self.pos..], bit: 0 };
        self.width = b.get(12) as usize;
        self.height = b.get(12) as usize;
        b.get(4 + 4 + 18 + 1 + 10 + 1);
        self.intra_q = DEFAULT_INTRA;
        if b.get(1) == 1 {
            for i in 0..64 {
                self.intra_q[ZIGZAG[i] as usize] = b.get(8) as u8;
            }
        }
        if b.get(1) == 1 {
            b.get(64 * 8); // non-intra matrix: not used by intra pictures
        }
        if self.width == 0 || self.height == 0 || self.width > 4096 || self.height > 4096 {
            return Err("bad picture size".into());
        }
        let mbw = self.width.div_ceil(16) * 16;
        let mbh = self.height.div_ceil(16) * 16;
        self.y = vec![0; mbw * mbh];
        self.cb = vec![0; mbw * mbh / 4];
        self.cr = vec![0; mbw * mbh / 4];
        Ok(())
    }

    fn extension(&mut self) -> Result<(), String> {
        let mut b = Bits { d: &self.data[self.pos..], bit: 0 };
        match b.get(4) {
            1 => {
                // sequence extension: profile, progressive, chroma format, size extensions
                self.mpeg2 = true;
                b.get(8 + 1);
                if b.get(2) != 1 {
                    return Err("only 4:2:0 video".into());
                }
            }
            3 => {
                // quant matrix extension
                if b.get(1) == 1 {
                    for i in 0..64 {
                        self.intra_q[ZIGZAG[i] as usize] = b.get(8) as u8;
                    }
                }
            }
            8 => {
                // picture coding extension
                b.get(16); // f codes
                self.dc_precision = b.get(2);
                if b.get(2) != 3 {
                    return Err("only frame pictures".into());
                }
                b.get(1); // top field first
                self.frame_dct = b.get(1) == 1;
                b.get(1); // concealment motion vectors
                self.q_scale_type = b.get(1) == 1;
                self.intra_vlc = b.get(1) == 1;
                self.alternate = b.get(1) == 1;
            }
            _ => {}
        }
        Ok(())
    }

    /// Decodes the next picture into the planes; false at the end of the stream.
    pub fn next_picture(&mut self) -> Result<bool, String> {
        // to the next picture start code
        loop {
            match self.next_start_code() {
                None | Some(0xB7) => return Ok(false),
                Some(0xB3) => self.sequence_header()?,
                Some(0xB5) => self.extension()?,
                Some(0x00) => break,
                _ => {}
            }
        }
        let kind = Bits { d: &self.data[self.pos..], bit: 0 }.peek(13) & 7;
        if kind != 1 {
            return Err(format!("picture of type {kind}: only intra-coded (I) pictures are supported"));
        }
        // its extensions, then the slices
        loop {
            match self.next_start_code() {
                Some(0xB5) => self.extension()?,
                Some(code @ 0x01..=0xAF) => self.slice(code as usize - 1)?,
                Some(_) => {
                    self.pos -= 4; // the next picture or sequence: read by the next call
                    break;
                }
                None => break,
            }
        }
        Ok(true)
    }

    fn slice(&mut self, row: usize) -> Result<(), String> {
        let t = tables();
        let data = std::mem::take(&mut self.data);
        let result = (|| {
            let mut b = Bits { d: &data[self.pos..], bit: 0 };
            let mbw = self.width.div_ceil(16);
            let mut q = self.quant(b.get(5));
            if b.get(1) == 1 {
                b.get(1 + 1 + 6);
                while b.get(1) == 1 {
                    b.get(8);
                }
            }
            let reset = 1i32 << (7 + self.dc_precision);
            let mut dc = [reset; 3];
            let mut addr = row as isize * mbw as isize - 1;
            loop {
                // macroblock address increment
                let mut incr = 0;
                loop {
                    match b.vlc(&t.mb_incr)? {
                        MB_ESCAPE => incr += 33,
                        MB_STUFFING => {}
                        v => {
                            incr += v as isize + 1;
                            break;
                        }
                    }
                }
                addr += incr;
                if incr > 1 {
                    dc = [reset; 3];
                }
                // macroblock type of an I picture: 1 = intra, 01 = intra with a new quantiser scale
                if b.get(1) == 0 {
                    if b.get(1) != 1 {
                        return Err("bad macroblock type".to_string());
                    }
                    q = self.quant(b.get(5));
                }
                let field = !self.frame_dct && b.get(1) == 1;
                let (mx, my) = ((addr as usize % mbw) * 16, (addr as usize / mbw) * 16);
                for blk in 0..6 {
                    let mut f = [0i32; 64];
                    let c = if blk < 4 { 0 } else { blk - 3 };
                    // DC
                    let size = b.vlc(if c == 0 { &t.dc_lum } else { &t.dc_chroma })? as u32;
                    if size > 0 {
                        let mut diff = b.get(size) as i32;
                        if diff < 1 << (size - 1) {
                            diff -= (1 << size) - 1;
                        }
                        dc[c] += diff;
                    }
                    f[0] = dc[c] << (3 - self.dc_precision);
                    // AC
                    let scan = if self.alternate { &ALTERNATE } else { &ZIGZAG };
                    let vlc = if self.intra_vlc { &t.b15 } else { &t.b14 };
                    let mut i = 0usize;
                    loop {
                        let v = b.vlc(vlc)?;
                        let (run, level) = match v {
                            EOB => break,
                            ESCAPE => {
                                let run = b.get(6) as usize;
                                let mut level = b.get(12) as i32;
                                if level >= 2048 {
                                    level -= 4096;
                                }
                                if level == 0 || level == -2048 {
                                    return Err("bad escape level".to_string());
                                }
                                (run, level)
                            }
                            v => {
                                let l = LEVEL[v as usize] as i32;
                                (RUN[v as usize] as usize, if b.get(1) == 1 { -l } else { l })
                            }
                        };
                        i += run + 1;
                        if i > 63 {
                            return Err("too many coefficients in a block".to_string());
                        }
                        let k = scan[i] as usize;
                        let v = level * self.intra_q[k] as i32 * q * 2 / 32;
                        f[k] = v.clamp(-2048, 2047);
                    }
                    // mismatch control
                    if f.iter().sum::<i32>() & 1 == 0 {
                        f[63] ^= 1;
                    }
                    let px = idct(&f, &t.idct);
                    // into the picture
                    if c == 0 {
                        let stride = self.y.len() / (self.height.div_ceil(16) * 16);
                        let (bx, by, step) = if field {
                            (mx + (blk & 1) * 8, my + (blk >> 1), 2)
                        } else {
                            (mx + (blk & 1) * 8, my + (blk >> 1) * 8, 1)
                        };
                        for r in 0..8 {
                            let o = (by + r * step) * stride + bx;
                            self.y[o..o + 8].copy_from_slice(&px[r * 8..r * 8 + 8]);
                        }
                    } else {
                        let plane = if c == 1 { &mut self.cb } else { &mut self.cr };
                        let stride = self.width.div_ceil(16) * 8;
                        for r in 0..8 {
                            let o = (my / 2 + r) * stride + mx / 2;
                            plane[o..o + 8].copy_from_slice(&px[r * 8..r * 8 + 8]);
                        }
                    }
                }
                if b.at_start_code() {
                    break;
                }
            }
            Ok(())
        })();
        self.data = data;
        result
    }

    fn quant(&self, code: u32) -> i32 {
        if self.q_scale_type {
            NONLINEAR_Q[code as usize] as i32
        } else {
            code as i32 * 2
        }
    }

    /// The picture as RGBA (alpha 255), `width` x `height`.
    pub fn rgba(&self, out: &mut [u8]) {
        let ys = self.y.len() / (self.height.div_ceil(16) * 16);
        let cs = ys / 2;
        for yy in 0..self.height {
            for xx in 0..self.width {
                let y = self.y[yy * ys + xx] as i32 - 16;
                let u = self.cb[(yy / 2) * cs + xx / 2] as i32 - 128;
                let v = self.cr[(yy / 2) * cs + xx / 2] as i32 - 128;
                // BT.601 limited range, 16.16 fixed point
                let l = 76309 * y + 32768;
                let r = (l + 104597 * v) >> 16;
                let g = (l - 25675 * u - 53279 * v) >> 16;
                let bl = (l + 132201 * u) >> 16;
                let o = (yy * self.width + xx) * 4;
                out[o] = r.clamp(0, 255) as u8;
                out[o + 1] = g.clamp(0, 255) as u8;
                out[o + 2] = bl.clamp(0, 255) as u8;
                out[o + 3] = 255;
            }
        }
    }

    /// The planes of the picture: Y (stride `width` rounded up to 16), Cb and Cr (half).
    pub fn planes(&self) -> (&[u8], &[u8], &[u8]) {
        (&self.y, &self.cb, &self.cr)
    }
}

/// The inverse DCT of a block of coefficients (raster order), in double precision, rounded and clamped to 0..255
/// (the pixels of an intra block).
fn idct(f: &[i32; 64], c: &[[f64; 8]; 8]) -> [u8; 64] {
    let mut tmp = [0f64; 64];
    // rows: tmp[v][x] = sum_u c[x][u] F[v][u]
    for v in 0..8 {
        for x in 0..8 {
            let mut s = 0.0;
            for u in 0..8 {
                s += c[x][u] * f[v * 8 + u] as f64;
            }
            tmp[v * 8 + x] = s;
        }
    }
    let mut out = [0u8; 64];
    for y in 0..8 {
        for x in 0..8 {
            let mut s = 0.0;
            for v in 0..8 {
                s += c[y][v] * tmp[v * 8 + x];
            }
            out[y * 8 + x] = (s.round() as i32).clamp(-256, 255).clamp(0, 255) as u8;
        }
    }
    out
}
