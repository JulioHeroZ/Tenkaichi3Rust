//! The Graphics Synthesizer: the frame's display list, run in software into the emulated GS memory.
//!
//! The game's drawing code (translated) produces what it produced on the PS2: one DMA chain per frame on the VIF1
//! channel, carrying GIF packets (register writes, primitives, texture uploads) for the GS and data and programs
//! for the vertex unit (vu1.rs). This module walks the chain and draws:
//!     DMA chain -> VIF1 commands -> GIF packets -> GS registers and vertices -> rasteriser -> GS memory.
//! GS memory is emulated with its real layout (4 MB in pages / blocks / columns per pixel format), because the
//! game uploads textures pre-swizzled and draws into memory it later reads as a texture.
//! Rust version of the C port's software reference (port/src/gs/gs_core.c). Drawing, not simulation: host floats.
//!
//! BT3_GS=1 turns it on (headless); BT3_SHOT=<n> writes every n-th frame to shots/frame_<n>.png.
#![allow(clippy::too_many_arguments)]
pub mod gl;
pub mod gpu;
mod hle;
pub mod platform;
pub mod vu1;

use crate::rt::Ctx;

const SURF_W: i32 = 1024;
const SURF_H: i32 = 1024;

const BLOCK32: [[u32; 8]; 4] = [[0, 1, 4, 5, 16, 17, 20, 21], [2, 3, 6, 7, 18, 19, 22, 23], [8, 9, 12, 13, 24, 25, 28, 29], [10, 11, 14, 15, 26, 27, 30, 31]];
const BLOCK16: [[u32; 4]; 8] = [[0, 2, 8, 10], [1, 3, 9, 11], [4, 6, 12, 14], [5, 7, 13, 15], [16, 18, 24, 26], [17, 19, 25, 27], [20, 22, 28, 30], [21, 23, 29, 31]];
const BLOCK16S: [[u32; 4]; 8] = [[0, 2, 16, 18], [1, 3, 17, 19], [8, 10, 24, 26], [9, 11, 25, 27], [4, 6, 20, 22], [5, 7, 21, 23], [12, 14, 28, 30], [13, 15, 29, 31]];
const COL32: [[u32; 8]; 8] = [
    [0, 1, 4, 5, 8, 9, 12, 13],
    [2, 3, 6, 7, 10, 11, 14, 15],
    [16, 17, 20, 21, 24, 25, 28, 29],
    [18, 19, 22, 23, 26, 27, 30, 31],
    [32, 33, 36, 37, 40, 41, 44, 45],
    [34, 35, 38, 39, 42, 43, 46, 47],
    [48, 49, 52, 53, 56, 57, 60, 61],
    [50, 51, 54, 55, 58, 59, 62, 63],
];
const COL16: [[u32; 16]; 8] = [
    [0, 2, 8, 10, 16, 18, 24, 26, 1, 3, 9, 11, 17, 19, 25, 27],
    [4, 6, 12, 14, 20, 22, 28, 30, 5, 7, 13, 15, 21, 23, 29, 31],
    [32, 34, 40, 42, 48, 50, 56, 58, 33, 35, 41, 43, 49, 51, 57, 59],
    [36, 38, 44, 46, 52, 54, 60, 62, 37, 39, 45, 47, 53, 55, 61, 63],
    [64, 66, 72, 74, 80, 82, 88, 90, 65, 67, 73, 75, 81, 83, 89, 91],
    [68, 70, 76, 78, 84, 86, 92, 94, 69, 71, 77, 79, 85, 87, 93, 95],
    [96, 98, 104, 106, 112, 114, 120, 122, 97, 99, 105, 107, 113, 115, 121, 123],
    [100, 102, 108, 110, 116, 118, 124, 126, 101, 103, 109, 111, 117, 119, 125, 127],
];
const COL8_FIRST: [[u32; 16]; 4] = [
    [0, 4, 16, 20, 32, 36, 48, 52, 2, 6, 18, 22, 34, 38, 50, 54],
    [8, 12, 24, 28, 40, 44, 56, 60, 10, 14, 26, 30, 42, 46, 58, 62],
    [33, 37, 49, 53, 1, 5, 17, 21, 35, 39, 51, 55, 3, 7, 19, 23],
    [41, 45, 57, 61, 9, 13, 25, 29, 43, 47, 59, 63, 11, 15, 27, 31],
];
const COL4_FIRST: [[u32; 32]; 4] = [
    [0, 8, 32, 40, 64, 72, 96, 104, 2, 10, 34, 42, 66, 74, 98, 106, 4, 12, 36, 44, 68, 76, 100, 108, 6, 14, 38, 46, 70, 78, 102, 110],
    [16, 24, 48, 56, 80, 88, 112, 120, 18, 26, 50, 58, 82, 90, 114, 122, 20, 28, 52, 60, 84, 92, 116, 124, 22, 30, 54, 62, 86, 94, 118, 126],
    [65, 73, 97, 105, 1, 9, 33, 41, 67, 75, 99, 107, 3, 11, 35, 43, 69, 77, 101, 109, 5, 13, 37, 45, 71, 79, 103, 111, 7, 15, 39, 47],
    [81, 89, 113, 121, 17, 25, 49, 57, 83, 91, 115, 123, 19, 27, 51, 59, 85, 93, 117, 125, 21, 29, 53, 61, 87, 95, 119, 127, 23, 31, 55, 63],
];

#[derive(Clone, Copy, Default)]
struct Vertex {
    x: f32,
    y: f32,
    z: u32,
    s: f32,
    t: f32,
    q: f32,
    u: i32,
    v: i32,
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

#[derive(Default)]
struct State {
    prim: u64,
    prim_raw: u64,
    rgbaq: u64,
    st: u64,
    uv: u64,
    texa: u64,
    prmodecont: u64,
    prmode: u64,
    tex0: [u64; 2],
    tex1: [u64; 2],
    clamp: [u64; 2],
    xyoffset: [u64; 2],
    scissor: [u64; 2],
    alpha: [u64; 2],
    test: [u64; 2],
    frame: [u64; 2],
    zbuf: [u64; 2],
    fba: [u64; 2],
    bitbltbuf: u64,
    trxpos: u64,
    trxreg: u64,
    trxdir: u64,
    q: f32,
    vtx: [Vertex; 3],
    vcount: usize,
    strip: u32,
    transfer: bool,
    tx: u32,
    ty: u32,
    tw: u32,
    th: u32,
    tpos: u32,
}

struct Target {
    fbp: u32,
    drawn: u32,
}

pub struct Gs {
    vram: Vec<u32>,
    col8: [[u32; 16]; 16],
    col4: [[u32; 32]; 16],
    in32: Vec<[u16; 64]>,      // [32]
    in16: Vec<Vec<[u16; 64]>>, // [2][64]
    in8: Vec<[u16; 128]>,      // [64]
    in4: Vec<[u16; 128]>,      // [128]
    s: State,
    targets: Vec<Target>,
    pub frame: u32,
    pub main_fbp: i32,
    shown: u64,
    shot_every: u32,
    verbose: bool,
    // GIF tag in progress (persists across runs)
    tag_lo: u64,
    tag_hi: u64,
    loops: u32,
    reg: u32,
    // VIF1 command in progress
    direct: u32,
    pend: u32,
    need: u32,
    buf: Vec<u32>,
    pub vu: vu1::Vu1,
    stat: [u32; 8],
    /// The last finished frame as RGBA (640 x 448), for a window.
    pub picture: Vec<u8>,
    pub picture_frame: u32,
    /// Bumped whenever an upload writes into that 8 KB page of GS memory (0: never uploaded to: a frame buffer).
    page_gen: Vec<u32>,
    page_hashes: Vec<(u32, u32, bool)>,
    /// The GPU back end (BT3_GS=gpu / gpu-headless): primitives go there instead of the rasteriser.
    pub gpu: Option<Box<gpu::Gpu>>,
    /// Seconds spent: [whole GS, VU1 programs, GPU capture (in kicks), frame end]; wall clock of the last report.
    pub times: [f64; 4],
    wall: std::time::Instant,
    vu_instr: u64,
    vu_interp: bool,
    hle_p4: u32,
    hle_p6: u32,
}

/// Whether the renderer runs (BT3_GS set and not none/off/0).
/// BT3_GS=gpu: a window; gpu-headless: an offscreen GL context (tests).
pub fn gpu_mode() -> Option<bool> {
    match std::env::var("BT3_GS").as_deref() {
        Ok("gpu") => Some(false),
        Ok("gpu-headless") => Some(true),
        _ => None,
    }
}

pub fn enabled() -> bool {
    match std::env::var("BT3_GS") {
        Ok(v) => !(v == "none" || v == "off" || v == "0"),
        Err(_) => false,
    }
}

impl Gs {
    pub fn new() -> Box<Gs> {
        let mut g = Box::new(Gs {
            vram: vec![0; 1 << 20],
            col8: [[0; 16]; 16],
            col4: [[0; 32]; 16],
            in32: vec![[0; 64]; 32],
            in16: vec![vec![[0; 64]; 64]; 2],
            in8: vec![[0; 128]; 64],
            in4: vec![[0; 128]; 128],
            s: State { prmodecont: 1, ..Default::default() },
            targets: Vec::new(),
            frame: 0,
            main_fbp: -1,
            shown: 0,
            shot_every: std::env::var("BT3_SHOT").ok().and_then(|v| v.parse().ok()).unwrap_or(0),
            verbose: std::env::var("BT3_GS_VERBOSE").is_ok(),
            tag_lo: 0,
            tag_hi: 0,
            loops: 0,
            reg: 0,
            direct: 0,
            pend: 0,
            need: 0,
            buf: Vec::new(),
            vu: vu1::Vu1::new(),
            stat: [0; 8],
            picture: vec![0; 640 * 448 * 4],
            picture_frame: 0,
            page_gen: vec![0; 512],
            page_hashes: vec![(0, 0, false); 512],
            gpu: None,
            times: [0.0; 4],
            wall: std::time::Instant::now(),
            vu_instr: 0,
            vu_interp: std::env::var("BT3_VU_INTERP").is_ok(),
            hle_p4: 0,
            hle_p6: 0,
        });
        g.tables_init();
        if let Some(headless) = gpu_mode() {
            match gpu::Gpu::new(headless) {
                Ok(p) => g.gpu = Some(p),
                Err(e) => eprintln!("bt3: GPU renderer unavailable ({e}); drawing in software"),
            }
        }
        g
    }

    fn tables_init(&mut self) {
        for y in 0..128usize {
            for x in 0..128usize {
                if y < 32 && x < 64 {
                    self.in32[y][x] = ((BLOCK32[(y >> 3) & 3][(x >> 3) & 7] << 6) + COL32[y & 7][x & 7]) as u16;
                }
                if y < 64 && x < 64 {
                    self.in16[0][y][x] = ((BLOCK16[(y >> 3) & 7][(x >> 4) & 3] << 7) + COL16[y & 7][x & 15]) as u16;
                    self.in16[1][y][x] = ((BLOCK16S[(y >> 3) & 7][(x >> 4) & 3] << 7) + COL16[y & 7][x & 15]) as u16;
                }
            }
        }
        for y in 0..16usize {
            let col = y >> 2;
            for x in 0..16usize {
                let sx = if col & 1 != 0 { x ^ 4 } else { x };
                self.col8[y][x] = COL8_FIRST[y & 3][sx] + col as u32 * 64;
            }
            for x in 0..32usize {
                let sx = if col & 1 != 0 { x ^ 4 } else { x };
                self.col4[y][x] = COL4_FIRST[y & 3][sx] + col as u32 * 128;
            }
        }
        for y in 0..128usize {
            for x in 0..128usize {
                if y < 64 {
                    self.in8[y][x] = ((BLOCK32[(y >> 4) & 3][(x >> 4) & 7] << 8) + self.col8[y & 15][x & 15]) as u16;
                }
                self.in4[y][x] = ((BLOCK16[(y >> 4) & 7][(x >> 5) & 3] << 9) + self.col4[y & 15][x & 31]) as u16;
            }
        }
    }

    // ------------------------------------------------------------------------------------------ GS memory
    /// Reads (write = None) or writes the pixel (x, y) of a buffer at bp, width bw, format psm.
    fn vram_rw(&mut self, bp: u32, bw: u32, psm: u32, x: u32, y: u32, write: Option<u32>) -> u32 {
        let bw = if bw == 0 { 1 } else { bw };
        let (mut psm, mut zx) = (psm, 0u32);
        if psm & 0x30 == 0x30 {
            zx = 0x18;
            psm &= 0x0F;
        }
        match psm {
            0x02 | 0x0A => {
                let blk = if psm == 0x0A { BLOCK16S } else { BLOCK16 };
                let a = ((bp
                    .wrapping_add(((y >> 1) & !0x1F).wrapping_mul(bw))
                    .wrapping_add((x >> 1) & !0x1F)
                    .wrapping_add(blk[((y >> 3) & 7) as usize][((x >> 4) & 3) as usize] ^ zx))
                    << 7)
                    .wrapping_add(COL16[(y & 7) as usize][(x & 15) as usize]);
                let w = &mut self.vram[((a >> 1) & 0xFFFFF) as usize];
                let sh = (a & 1) * 16;
                if let Some(v) = write {
                    *w = (*w & !(0xFFFF << sh)) | (v & 0xFFFF) << sh;
                    return 0;
                }
                (*w >> sh) & 0xFFFF
            }
            0x13 => {
                let a = ((bp
                    .wrapping_add(((y >> 1) & !0x1F).wrapping_mul(bw >> 1))
                    .wrapping_add((x >> 2) & !0x1F)
                    .wrapping_add(BLOCK32[((y >> 4) & 3) as usize][((x >> 4) & 7) as usize]))
                    << 8)
                    .wrapping_add(self.col8[(y & 15) as usize][(x & 15) as usize]);
                let w = &mut self.vram[((a >> 2) & 0xFFFFF) as usize];
                let sh = (a & 3) * 8;
                if let Some(v) = write {
                    *w = (*w & !(0xFF << sh)) | (v & 0xFF) << sh;
                    return 0;
                }
                (*w >> sh) & 0xFF
            }
            0x14 => {
                let a = ((bp
                    .wrapping_add(((y >> 2) & !0x1F).wrapping_mul(bw >> 1))
                    .wrapping_add((x >> 2) & !0x1F)
                    .wrapping_add(BLOCK16[((y >> 4) & 7) as usize][((x >> 5) & 3) as usize]))
                    << 9)
                    .wrapping_add(self.col4[(y & 15) as usize][(x & 31) as usize]);
                let w = &mut self.vram[((a >> 3) & 0xFFFFF) as usize];
                let sh = (a & 7) * 4;
                if let Some(v) = write {
                    *w = (*w & !(0xF << sh)) | (v & 0xF) << sh;
                    return 0;
                }
                (*w >> sh) & 0xF
            }
            _ => {
                let a = ((bp
                    .wrapping_add((y & !0x1F).wrapping_mul(bw))
                    .wrapping_add((x >> 1) & !0x1F)
                    .wrapping_add(BLOCK32[((y >> 3) & 3) as usize][((x >> 3) & 7) as usize] ^ zx))
                    << 6)
                    .wrapping_add(COL32[(y & 7) as usize][(x & 7) as usize]);
                let w = &mut self.vram[(a & 0xFFFFF) as usize];
                match psm {
                    0x01 => {
                        if let Some(v) = write {
                            *w = (*w & 0xFF00_0000) | (v & 0xFF_FFFF);
                            return 0;
                        }
                        *w & 0xFF_FFFF
                    }
                    0x1B => {
                        if let Some(v) = write {
                            *w = (*w & 0x00FF_FFFF) | v << 24;
                            return 0;
                        }
                        *w >> 24
                    }
                    0x24 => {
                        if let Some(v) = write {
                            *w = (*w & 0xF0FF_FFFF) | (v & 15) << 24;
                            return 0;
                        }
                        (*w >> 24) & 15
                    }
                    0x2C => {
                        if let Some(v) = write {
                            *w = (*w & 0x0FFF_FFFF) | (v & 15) << 28;
                            return 0;
                        }
                        *w >> 28
                    }
                    _ => {
                        if let Some(v) = write {
                            *w = v;
                            return 0;
                        }
                        *w
                    }
                }
            }
        }
    }

    /// A hash of one 8 KB page of GS memory, recomputed only after an upload touched the page (the GPU's texture
    /// cache is keyed on these, so a texture uploaded again unchanged is recognised instead of decoded again).
    fn page_hash(&mut self, page: u32) -> u32 {
        let page = (page & 511) as usize;
        let (h, seen, valid) = self.page_hashes[page];
        if valid && seen == self.page_gen[page] {
            return h;
        }
        let w = &self.vram[page * 2048..page * 2048 + 2048];
        const MUL: [u64; 4] = [0xA0761D6478BD642F, 0xE7037ED1A0B428DB, 0x8EBC6AF09C88C6E3, 0x589965CC75374CC3];
        let mut l: [u64; 4] = [0x9E3779B97F4A7C15, 0xC2B2AE3D27D4EB4F, 0x165667B19E3779F9, 0x27D4EB2F165667C5];
        let mut i = 0;
        while i < 2048 {
            for j in 0..4 {
                let q = w[i + j * 4] as u64 | (w[i + j * 4 + 1] as u64) << 32;
                let b = w[i + j * 4 + 2] as u64 | (w[i + j * 4 + 3] as u64) << 32;
                let m = ((l[j] ^ q) as u128) * ((MUL[j] ^ b) as u128);
                l[j] = (m as u64) ^ ((m >> 64) as u64) ^ q;
            }
            i += 16;
        }
        let q = (l[0] ^ l[1].rotate_left(13) ^ l[2].rotate_left(29) ^ l[3].rotate_left(47)).wrapping_mul(0x9FB21C651E98DF25);
        let h = (q ^ q >> 32) as u32;
        self.page_hashes[page] = (h, self.page_gen[page], true);
        h
    }

    /// A primitive for the GPU back end (type 0 point, 1 line, 3 triangle, 6 sprite).
    fn gpu_draw(&mut self, ty: u32, ctx: usize, v: &[Vertex]) {
        let mut g = self.gpu.take().unwrap();
        g.draw(self, ty, ctx, v);
        self.gpu = Some(g);
    }

    #[inline(always)]
    fn rd(&mut self, bp: u32, bw: u32, psm: u32, x: u32, y: u32) -> u32 {
        self.vram_rw(bp, bw, psm, x, y, None)
    }

    /// The raw 256 bytes of GS memory of the block holding pixel (x, y) of a texture (PSMT8, PSMT4, else PSMCT32),
    /// appended to `out` (host/texpack.rs hashes them as PCSX2 does).
    pub(crate) fn block_into(&self, bp: u32, bw: u32, psm: u32, x: u32, y: u32, out: &mut Vec<u8>) {
        let bw = if bw == 0 { 1 } else { bw };
        let bn = match psm {
            0x13 => bp
                .wrapping_add(((y >> 1) & !0x1F).wrapping_mul(bw >> 1))
                .wrapping_add((x >> 2) & !0x1F)
                .wrapping_add(BLOCK32[((y >> 4) & 3) as usize][((x >> 4) & 7) as usize]),
            0x14 => bp
                .wrapping_add(((y >> 2) & !0x1F).wrapping_mul(bw >> 1))
                .wrapping_add((x >> 2) & !0x1F)
                .wrapping_add(BLOCK16[((y >> 4) & 7) as usize][((x >> 5) & 3) as usize]),
            _ => bp
                .wrapping_add((y & !0x1F).wrapping_mul(bw))
                .wrapping_add((x >> 1) & !0x1F)
                .wrapping_add(BLOCK32[((y >> 3) & 3) as usize][((x >> 3) & 7) as usize]),
        };
        let w = ((bn & 0x3FFF) << 6) as usize;
        for v in &self.vram[w..w + 64] {
            out.extend_from_slice(&v.to_le_bytes());
        }
    }

    fn target_index(&mut self, fbp: u32) -> usize {
        if let Some(i) = self.targets.iter().position(|t| t.fbp == fbp) {
            return i;
        }
        if self.targets.len() == 16 {
            return 0;
        }
        self.targets.push(Target { fbp, drawn: 0 });
        self.targets.len() - 1
    }

    // ------------------------------------------------------------------------------------------ textures
    fn psm_bits(psm: u32) -> u32 {
        let psm = if psm & 0x30 == 0x30 { psm & 0x0F } else { psm };
        match psm {
            0x00 => 32,
            0x01 => 24,
            0x02 | 0x0A => 16,
            0x13 | 0x1B => 8,
            0x14 | 0x24 | 0x2C => 4,
            _ => 32,
        }
    }

    /// A stored colour of format psm as R | G << 8 | B << 16 | A << 24 (TEXA supplies alpha where needed).
    fn expand(&self, c: u32, psm: u32) -> u32 {
        let ta0 = (self.s.texa & 0xFF) as u32;
        let ta1 = ((self.s.texa >> 32) & 0xFF) as u32;
        let aem = (self.s.texa >> 15) & 1 != 0;
        let psm = if psm & 0x30 == 0x30 { psm & 0x0F } else { psm };
        if psm == 0x00 {
            return c;
        }
        if psm == 0x01 {
            let c = c & 0xFF_FFFF;
            return c | (if aem && c == 0 { 0 } else { ta0 }) << 24;
        }
        let r = (c & 31) << 3;
        let g = ((c >> 5) & 31) << 3;
        let b = ((c >> 10) & 31) << 3;
        let a = if c & 0x8000 != 0 { ta1 } else if aem && (c & 0x7FFF) == 0 { 0 } else { ta0 };
        r | g << 8 | b << 16 | a << 24
    }

    fn texel(&mut self, ctx: usize, u: i32, v: i32) -> u32 {
        let t0 = self.s.tex0[ctx];
        let cl = self.s.clamp[ctx];
        let tbp = (t0 & 0x3FFF) as u32;
        let tbw = ((t0 >> 14) & 0x3F) as u32;
        let psm = ((t0 >> 20) & 0x3F) as u32;
        let tw = 1i32 << ((t0 >> 26) & 15);
        let th = 1i32 << ((t0 >> 30) & 15);
        let cbp = ((t0 >> 37) & 0x3FFF) as u32;
        let cpsm = ((t0 >> 51) & 15) as u32;
        let wms = cl & 3;
        let wmt = (cl >> 2) & 3;
        let minu = ((cl >> 4) & 0x3FF) as i32;
        let maxu = ((cl >> 14) & 0x3FF) as i32;
        let minv = ((cl >> 24) & 0x3FF) as i32;
        let maxv = ((cl >> 34) & 0x3FF) as i32;
        let u = match wms {
            0 => u & (tw - 1),
            1 => u.clamp(0, tw - 1),
            2 => {
                if u < minu {
                    minu
                } else if u > maxu {
                    maxu
                } else {
                    u
                }
            }
            _ => (u & minu) | maxu,
        };
        let v = match wmt {
            0 => v & (th - 1),
            1 => v.clamp(0, th - 1),
            2 => {
                if v < minv {
                    minv
                } else if v > maxv {
                    maxv
                } else {
                    v
                }
            }
            _ => (v & minv) | maxv,
        };
        let c = self.rd(tbp, tbw, psm, u as u32, v as u32);
        let bits = Self::psm_bits(psm);
        if bits > 8 {
            return self.expand(c, psm);
        }
        if bits == 8 {
            let c = (c & 0xE7) | ((c & 8) << 1) | ((c & 0x10) >> 1);
            let p = self.rd(cbp, 1, cpsm, c & 15, c >> 4);
            return self.expand(p, cpsm);
        }
        let p = self.rd(cbp, 1, cpsm, c & 7, c >> 3);
        self.expand(p, cpsm)
    }

    // ------------------------------------------------------------------------------------------ pixels
    fn pixel(&mut self, ti: usize, ctx: usize, x: i32, y: i32, z: u32, src: u32) {
        let test = self.s.test[ctx];
        let al = self.s.alpha[ctx];
        let zb = self.s.zbuf[ctx];
        let pr = self.s.prim;
        let sc = self.s.scissor[ctx];
        let fr = self.s.frame[ctx];
        let fbp = ((fr & 0x1FF) << 5) as u32;
        let fbw = ((fr >> 16) & 0x3F) as u32;
        let fpsm = ((fr >> 24) & 0x3F) as u32;
        let mut write_rgb = true;
        let mut write_z = (zb >> 32) & 1 == 0;
        if x < (sc & 0x7FF) as i32
            || x > ((sc >> 16) & 0x7FF) as i32
            || y < ((sc >> 32) & 0x7FF) as i32
            || y > ((sc >> 48) & 0x7FF) as i32
            || x < 0
            || y < 0
            || x >= SURF_W
            || y >= SURF_H
        {
            return;
        }
        let sa = src >> 24;
        if test & 1 != 0 {
            let r = ((test >> 4) & 0xFF) as u32;
            let pass = match (test >> 1) & 7 {
                0 => false,
                1 => true,
                2 => sa < r,
                3 => sa <= r,
                4 => sa == r,
                5 => sa >= r,
                6 => sa > r,
                _ => sa != r,
            };
            if !pass {
                match (test >> 12) & 3 {
                    0 => return,
                    1 => write_z = false,
                    2 => write_rgb = false,
                    _ => write_z = false,
                }
            }
        }
        let zbp = ((zb & 0x1FF) << 5) as u32;
        let zpsm = 0x30 | ((zb >> 24) & 15) as u32;
        let zmax = match Self::psm_bits(zpsm) {
            32 => 0xFFFF_FFFF,
            24 => 0xFF_FFFF,
            _ => 0xFFFF,
        };
        let z = z.min(zmax);
        if (test >> 16) & 1 != 0 {
            let zd = self.rd(zbp, fbw, zpsm, x as u32, y as u32);
            match (test >> 17) & 3 {
                0 => return,
                1 => {}
                2 => {
                    if z < zd {
                        return;
                    }
                }
                _ => {
                    if z <= zd {
                        return;
                    }
                }
            }
        }
        let mut d = self.rd(fbp, fbw, fpsm, x as u32, y as u32);
        if Self::psm_bits(fpsm) == 16 {
            d = (d & 31) << 3 | ((d >> 5) & 31) << 11 | ((d >> 10) & 31) << 19 | if d & 0x8000 != 0 { 0x8000_0000 } else { 0 };
        } else if fpsm == 1 {
            d |= 0x8000_0000;
        }
        let (sr, sg, sb) = (src & 0xFF, (src >> 8) & 0xFF, (src >> 16) & 0xFF);
        let mut out = [sr, sg, sb];
        if (pr >> 6) & 1 != 0 {
            let s = [sr as i32, sg as i32, sb as i32];
            let c = match (al >> 4) & 3 {
                0 => sa,
                1 => d >> 24,
                _ => ((al >> 32) & 0xFF) as u32,
            } as i32;
            for i in 0..3 {
                let dc = ((d >> (8 * i)) & 0xFF) as i32;
                let a = match al & 3 {
                    0 => s[i],
                    1 => dc,
                    _ => 0,
                };
                let b = match (al >> 2) & 3 {
                    0 => s[i],
                    1 => dc,
                    _ => 0,
                };
                let dd = match (al >> 6) & 3 {
                    0 => s[i],
                    1 => dc,
                    _ => 0,
                };
                let v = (((a - b) * c) >> 7) + dd;
                out[i] = v.clamp(0, 255) as u32;
            }
        }
        if write_rgb {
            let mut c = out[0] | out[1] << 8 | out[2] << 16 | sa << 24;
            let mut m = (fr >> 32) as u32;
            if Self::psm_bits(fpsm) == 16 {
                c = out[0] >> 3 | (out[1] >> 3) << 5 | (out[2] >> 3) << 10 | (sa & 0x80) << 8;
                m = ((m >> 3) & 0x1F) | ((m >> 6) & 0x3E0) | ((m >> 9) & 0x7C00) | ((m >> 16) & 0x8000);
            }
            if m != 0 {
                let old = self.rd(fbp, fbw, fpsm, x as u32, y as u32);
                c = (old & m) | (c & !m);
            }
            self.vram_rw(fbp, fbw, fpsm, x as u32, y as u32, Some(c));
            self.targets[ti].drawn += 1;
        }
        if write_z {
            self.vram_rw(zbp, fbw, zpsm, x as u32, y as u32, Some(z));
        }
    }

    /// Vertex colour and texture combined (TEX0.TFX / TCC).
    fn shade(&mut self, ctx: usize, vc: u32, s: f32, t: f32, q: f32, u: i32, v: i32) -> u32 {
        let pr = self.s.prim;
        let t0 = self.s.tex0[ctx];
        let tfx = (t0 >> 35) & 3;
        let tcc = (t0 >> 34) & 1;
        if (pr >> 4) & 1 == 0 {
            return vc;
        }
        let tc = if (pr >> 8) & 1 != 0 {
            self.texel(ctx, u >> 4, v >> 4)
        } else {
            let w = if q != 0.0 { 1.0 / q } else { 1.0 };
            let tu = (s * w * (1u32 << ((t0 >> 26) & 15)) as f32) as i32;
            let tv = (t * w * (1u32 << ((t0 >> 30) & 15)) as f32) as i32;
            self.texel(ctx, tu, tv)
        };
        let mut out = 0u32;
        for i in 0..3 {
            let cv = (vc >> (8 * i)) & 0xFF;
            let ct = (tc >> (8 * i)) & 0xFF;
            let av = vc >> 24;
            let c = match tfx {
                0 => (ct * cv) >> 7,
                1 => ct,
                _ => ((ct * cv) >> 7) + av,
            };
            out |= c.min(255) << (8 * i);
        }
        let av = vc >> 24;
        let at = tc >> 24;
        let a = if tcc == 0 {
            av
        } else if tfx == 0 {
            (at * av) >> 7
        } else if tfx == 2 {
            at + av
        } else {
            at
        };
        out | a.min(255) << 24
    }

    // ------------------------------------------------------------------------------------------ primitives
    fn vcol(v: &Vertex) -> u32 {
        v.r as u32 | (v.g as u32) << 8 | (v.b as u32) << 16 | (v.a as u32) << 24
    }

    fn draw_sprite(&mut self, ti: usize, ctx: usize, a: Vertex, b: Vertex) {
        let (mut a, mut b) = (a, b);
        let col = Self::vcol(&b);
        if a.x > b.x {
            std::mem::swap(&mut a, &mut b);
        }
        let (x0, x1, y0, y1) = (a.x, b.x, a.y, b.y);
        let xa = (x0 + 0.9375) as i32;
        let xb = (x1 + 0.9375) as i32;
        let (ya, yb) = if y0 <= y1 { ((y0 + 0.9375) as i32, (y1 + 0.9375) as i32) } else { ((y1 + 0.9375) as i32, (y0 + 0.9375) as i32) };
        let xa = xa.max(-1);
        let xb = xb.min(SURF_W + 1);
        let ya = ya.max(-1);
        let yb = yb.min(SURF_H + 1);
        for y in ya..yb {
            let fy = if y1 != y0 { (y as f32 - y0) / (y1 - y0) } else { 0.0 };
            for x in xa..xb {
                let fx = if x1 != x0 { (x as f32 - x0) / (x1 - x0) } else { 0.0 };
                let c = self.shade(
                    ctx,
                    col,
                    a.s + (b.s - a.s) * fx,
                    a.t + (b.t - a.t) * fy,
                    b.q,
                    (a.u as f32 + (b.u - a.u) as f32 * fx) as i32,
                    (a.v as f32 + (b.v - a.v) as f32 * fy) as i32,
                );
                self.pixel(ti, ctx, x, y, b.z, c);
            }
        }
    }

    fn edge(a: &Vertex, b: &Vertex, x: f32, y: f32) -> f32 {
        (b.x - a.x) * (y - a.y) - (b.y - a.y) * (x - a.x)
    }

    fn draw_triangle(&mut self, ti: usize, ctx: usize, a: Vertex, b: Vertex, c: Vertex) {
        let area = Self::edge(&a, &b, c.x, c.y);
        let gouraud = (self.s.prim >> 3) & 1 != 0;
        if area == 0.0 {
            return;
        }
        let minx = a.x.min(b.x).min(c.x).max(0.0);
        let maxx = a.x.max(b.x).max(c.x).min((SURF_W - 1) as f32);
        let miny = a.y.min(b.y).min(c.y).max(0.0);
        let maxy = a.y.max(b.y).max(c.y).min((SURF_H - 1) as f32);
        if minx > maxx || miny > maxy {
            return;
        }
        for y in miny as i32..=maxy as i32 {
            for x in minx as i32..=maxx as i32 {
                let w0 = Self::edge(&b, &c, x as f32, y as f32) as f64 / area as f64;
                let w1 = Self::edge(&c, &a, x as f32, y as f32) as f64 / area as f64;
                let w2 = 1.0 - w0 - w1;
                if w0 < 0.0 || w1 < 0.0 || w2 < -1e-9 {
                    continue;
                }
                let col = if gouraud {
                    let m = |p: u8, q: u8, r: u8| (p as f64 * w0 + q as f64 * w1 + r as f64 * w2 + 0.5) as u32;
                    m(a.r, b.r, c.r) | m(a.g, b.g, c.g) << 8 | m(a.b, b.b, c.b) << 16 | m(a.a, b.a, c.a) << 24
                } else {
                    Self::vcol(&c)
                };
                let z = (a.z as f64 * w0 + b.z as f64 * w1 + c.z as f64 * w2 + 0.5) as u32;
                let s = (a.s as f64 * w0 + b.s as f64 * w1 + c.s as f64 * w2) as f32;
                let t = (a.t as f64 * w0 + b.t as f64 * w1 + c.t as f64 * w2) as f32;
                let q = (a.q as f64 * w0 + b.q as f64 * w1 + c.q as f64 * w2) as f32;
                let u = (a.u as f64 * w0 + b.u as f64 * w1 + c.u as f64 * w2) as i32;
                let v = (a.v as f64 * w0 + b.v as f64 * w1 + c.v as f64 * w2) as i32;
                let px = self.shade(ctx, col, s, t, q, u, v);
                self.pixel(ti, ctx, x, y, z, px);
            }
        }
    }

    fn draw_line(&mut self, ti: usize, ctx: usize, a: Vertex, b: Vertex) {
        let dx = b.x - a.x;
        let dy = b.y - a.y;
        let n = dx.abs().max(dy.abs()) as i32;
        let n = n.min(4096);
        for i in 0..=n {
            let f = if n != 0 { i as f32 / n as f32 } else { 0.0 };
            self.pixel(ti, ctx, (a.x + dx * f) as i32, (a.y + dy * f) as i32, b.z, Self::vcol(&b));
        }
    }

    /// A vertex arrives (XYZ2 / XYZF2 with kick, XYZ3 / XYZF3 without).
    fn vertex(&mut self, x: u32, y: u32, z: u32, kick: bool) {
        let ctx = ((self.s.prim >> 9) & 1) as usize;
        let ty = self.s.prim & 7;
        self.stat[5] += 1;
        let v = Vertex {
            x: (x as i32 as f32 - (self.s.xyoffset[ctx] & 0xFFFF) as f32) / 16.0,
            y: (y as i32 as f32 - ((self.s.xyoffset[ctx] >> 32) & 0xFFFF) as f32) / 16.0,
            z,
            r: (self.s.rgbaq & 0xFF) as u8,
            g: ((self.s.rgbaq >> 8) & 0xFF) as u8,
            b: ((self.s.rgbaq >> 16) & 0xFF) as u8,
            a: ((self.s.rgbaq >> 24) & 0xFF) as u8,
            s: f32::from_bits(self.s.st as u32),
            t: f32::from_bits((self.s.st >> 32) as u32),
            q: self.s.q,
            u: (self.s.uv & 0x3FFF) as i32,
            v: ((self.s.uv >> 16) & 0x3FFF) as i32,
        };
        if self.s.vcount < 3 {
            self.s.vtx[self.s.vcount] = v;
            self.s.vcount += 1;
        } else {
            self.s.vtx[0] = self.s.vtx[1];
            self.s.vtx[1] = self.s.vtx[2];
            self.s.vtx[2] = v;
        }
        self.s.strip += 1;
        let vt = self.s.vtx;
        if self.gpu.is_some() {
            let fb = (self.s.frame[ctx] & 0x1FF) as usize;
            match ty {
                0 => {
                    if kick {
                        self.gpu_draw(0, ctx, &[v]);
                    }
                    self.s.vcount = 0;
                }
                1 | 2 => {
                    if self.s.vcount == 2 {
                        if kick {
                            self.gpu_draw(1, ctx, &vt[..2]);
                        }
                        if ty == 2 {
                            self.s.vtx[0] = self.s.vtx[1];
                            self.s.vcount = 1;
                        } else {
                            self.s.vcount = 0;
                        }
                    }
                }
                3 => {
                    if self.s.vcount == 3 {
                        if kick {
                            self.gpu_draw(3, ctx, &vt);
                        }
                        self.s.vcount = 0;
                    }
                }
                4 => {
                    if self.s.vcount == 3 && kick {
                        self.gpu_draw(3, ctx, &vt);
                    }
                }
                5 => {
                    if self.s.vcount == 3 {
                        if kick {
                            self.gpu_draw(3, ctx, &vt);
                        }
                        self.s.vtx[1] = self.s.vtx[2];
                        self.s.vcount = 2;
                    }
                }
                6 => {
                    if self.s.vcount == 2 {
                        // a rectangle drawn into texture memory (a 64-pixel-wide buffer at a page the game uploads
                        // to): the fighters' palettes painted over, which has to happen in GS memory
                        if kick && ((self.s.frame[ctx] >> 16) & 0x3F) == 1 && self.page_gen[fb] != 0 {
                            let ti = self.target_index(fb as u32);
                            self.draw_sprite(ti, ctx, vt[0], vt[1]);
                            self.page_gen[fb] += 1;
                            self.page_gen[(fb + 1) & 511] += 1;
                        } else if kick {
                            self.gpu_draw(6, ctx, &vt[..2]);
                        }
                        self.s.vcount = 0;
                    }
                }
                _ => {}
            }
            return;
        }
        let ti = self.target_index((self.s.frame[ctx] & 0x1FF) as u32);
        match ty {
            0 => {
                if kick {
                    let c = Self::vcol(&v);
                    self.pixel(ti, ctx, v.x as i32, v.y as i32, v.z, c);
                }
                self.s.vcount = 0;
            }
            1 => {
                if self.s.vcount == 2 {
                    if kick {
                        self.draw_line(ti, ctx, vt[0], vt[1]);
                    }
                    self.s.vcount = 0;
                }
            }
            2 => {
                if self.s.vcount == 2 {
                    if kick {
                        self.draw_line(ti, ctx, vt[0], vt[1]);
                    }
                    self.s.vtx[0] = self.s.vtx[1];
                    self.s.vcount = 1;
                }
            }
            3 => {
                if self.s.vcount == 3 {
                    if kick {
                        self.draw_triangle(ti, ctx, vt[0], vt[1], vt[2]);
                    }
                    self.s.vcount = 0;
                }
            }
            4 => {
                if self.s.vcount == 3 && kick {
                    self.draw_triangle(ti, ctx, vt[0], vt[1], vt[2]);
                }
            }
            5 => {
                if self.s.vcount == 3 {
                    if kick {
                        self.draw_triangle(ti, ctx, vt[0], vt[1], vt[2]);
                    }
                    self.s.vtx[1] = self.s.vtx[2];
                    self.s.vcount = 2;
                }
            }
            6 => {
                if self.s.vcount == 2 {
                    if kick {
                        self.draw_sprite(ti, ctx, vt[0], vt[1]);
                    }
                    self.s.vcount = 0;
                }
            }
            _ => {}
        }
    }

    // ------------------------------------------------------------------------------------------ registers
    fn transfer_begin(&mut self) {
        let s = &mut self.s;
        s.transfer = s.trxdir & 3 == 0;
        s.tx = ((s.trxpos >> 32) & 0x7FF) as u32;
        s.ty = ((s.trxpos >> 48) & 0x7FF) as u32;
        s.tw = (s.trxreg & 0xFFF) as u32;
        s.th = ((s.trxreg >> 32) & 0xFFF) as u32;
        s.tpos = 0;
    }

    fn transfer_data(&mut self, p: &[u8]) {
        let dbp = ((self.s.bitbltbuf >> 32) & 0x3FFF) as u32;
        let dbw = ((self.s.bitbltbuf >> 48) & 0x3F) as u32;
        let dpsm = ((self.s.bitbltbuf >> 56) & 0x3F) as u32;
        if !self.s.transfer || self.s.tw == 0 {
            return;
        }
        let bytes = p.len() as u32;
        let bits = Self::psm_bits(dpsm);
        let total = self.s.tw * self.s.th;
        if dbp == 0 && dbw == 8 && bits == 32 {
            if let Some(g) = self.gpu.as_mut() {
                let bit = if self.s.ty < 448 { 1 } else { 2 };
                if g.fb_uploads & bit == 0 {
                    g.fb_upload(bit == 2);
                }
                g.fb_uploads |= bit;
            }
        }
        {
            let rowbytes = (if dbw != 0 { dbw } else { 1 }) * 64 * bits / 8;
            let first = dbp / 32 + (self.s.ty * rowbytes) / 8192;
            let last = dbp / 32 + ((self.s.ty + self.s.th) * rowbytes + 8191) / 8192;
            for pg in first..=last {
                self.page_gen[(pg & 511) as usize] = self.page_gen[(pg & 511) as usize].wrapping_add(1);
            }
        }
        let mut i = 0u32;
        if matches!(dpsm, 0x00 | 0x02 | 0x0A | 0x13 | 0x14) {
            let bw = if dbw != 0 { dbw } else { 1 };
            let count = bytes * 8 / bits;
            while i < count && self.s.tpos < total {
                let col = self.s.tpos % self.s.tw;
                let y = self.s.ty + self.s.tpos / self.s.tw;
                let mut x = self.s.tx + col;
                let run = (self.s.tw - col).min(count - i);
                match dpsm {
                    0x13 => {
                        let base = (dbp + (y >> 6) * 32 * (bw >> 1)) << 8;
                        for j in 0..run {
                            let a = (base + ((x >> 7) << 13) + self.in8[(y & 63) as usize][(x & 127) as usize] as u32) & 0x3F_FFFF;
                            let w = &mut self.vram[(a >> 2) as usize];
                            let sh = (a & 3) * 8;
                            *w = (*w & !(0xFF << sh)) | (p[(i + j) as usize] as u32) << sh;
                            x += 1;
                        }
                    }
                    0x14 => {
                        let base = (dbp + (y >> 7) * 32 * (bw >> 1)) << 9;
                        for j in 0..run {
                            let a = base + ((x >> 7) << 14) + self.in4[(y & 127) as usize][(x & 127) as usize] as u32;
                            let k = i + j;
                            let c = (p[(k >> 1) as usize] as u32 >> ((k & 1) * 4)) & 15;
                            let ba = (a >> 1) & 0x3F_FFFF;
                            let w = &mut self.vram[(ba >> 2) as usize];
                            let bsh = (ba & 3) * 8;
                            let mut byte = (*w >> bsh) & 0xFF;
                            byte = if a & 1 != 0 { (byte & 0x0F) | c << 4 } else { (byte & 0xF0) | c };
                            *w = (*w & !(0xFF << bsh)) | byte << bsh;
                            x += 1;
                        }
                    }
                    0x00 => {
                        let base = (dbp + (y >> 5) * 32 * bw) << 6;
                        for j in 0..run {
                            let a = (base + ((x >> 6) << 11) + self.in32[(y & 31) as usize][(x & 63) as usize] as u32) & 0xFFFFF;
                            let o = ((i + j) * 4) as usize;
                            self.vram[a as usize] = u32::from_le_bytes(p[o..o + 4].try_into().unwrap());
                            x += 1;
                        }
                    }
                    _ => {
                        let sw = (dpsm == 0x0A) as usize;
                        let base = (dbp + (y >> 6) * 32 * bw) << 7;
                        for j in 0..run {
                            let a = (base + ((x >> 6) << 12) + self.in16[sw][(y & 63) as usize][(x & 63) as usize] as u32) & 0x1F_FFFF;
                            let o = ((i + j) * 2) as usize;
                            let h = u16::from_le_bytes([p[o], p[o + 1]]) as u32;
                            let w = &mut self.vram[(a >> 1) as usize];
                            let sh = (a & 1) * 16;
                            *w = (*w & !(0xFFFF << sh)) | h << sh;
                            x += 1;
                        }
                    }
                }
                i += run;
                self.s.tpos += run;
            }
        }
        while i * bits < bytes * 8 && self.s.tpos < total {
            let x = self.s.tx + self.s.tpos % self.s.tw;
            let y = self.s.ty + self.s.tpos / self.s.tw;
            let iu = i as usize;
            let c = match bits {
                32 => u32::from_le_bytes(p[iu * 4..iu * 4 + 4].try_into().unwrap()),
                24 => p[iu * 3] as u32 | (p[iu * 3 + 1] as u32) << 8 | (p[iu * 3 + 2] as u32) << 16,
                16 => p[iu * 2] as u32 | (p[iu * 2 + 1] as u32) << 8,
                8 => p[iu] as u32,
                _ => (p[iu / 2] as u32 >> ((i & 1) * 4)) & 15,
            };
            self.vram_rw(dbp, dbw, dpsm, x, y, Some(c));
            self.s.tpos += 1;
            i += 1;
        }
    }

    fn prim_update(&mut self) {
        self.s.prim = if self.s.prmodecont & 1 != 0 { self.s.prim_raw } else { (self.s.prim_raw & 7) | (self.s.prmode & 0x7F8) };
    }

    pub fn reg_write(&mut self, addr: u32, d: u64) {
        self.stat[4] += 1;
        match addr {
            0x00 => {
                self.s.prim_raw = d;
                self.prim_update();
                self.s.vcount = 0;
                self.s.strip = 0;
            }
            0x01 => {
                self.s.rgbaq = d;
                self.s.q = f32::from_bits((d >> 32) as u32);
            }
            0x02 => self.s.st = d,
            0x03 => self.s.uv = d,
            0x04 => self.vertex((d & 0xFFFF) as u32, ((d >> 16) & 0xFFFF) as u32, ((d >> 32) & 0xFF_FFFF) as u32, true),
            0x05 => self.vertex((d & 0xFFFF) as u32, ((d >> 16) & 0xFFFF) as u32, (d >> 32) as u32, true),
            0x06 | 0x07 => self.s.tex0[(addr - 6) as usize] = d,
            0x08 | 0x09 => self.s.clamp[(addr - 8) as usize] = d,
            0x0C => self.vertex((d & 0xFFFF) as u32, ((d >> 16) & 0xFFFF) as u32, ((d >> 32) & 0xFF_FFFF) as u32, false),
            0x0D => self.vertex((d & 0xFFFF) as u32, ((d >> 16) & 0xFFFF) as u32, (d >> 32) as u32, false),
            0x14 | 0x15 => self.s.tex1[(addr - 0x14) as usize] = d,
            0x16 | 0x17 => {
                let i = (addr - 0x16) as usize;
                const M: u64 = 0x1FFF_FFE0_03F0_0000;
                self.s.tex0[i] = (self.s.tex0[i] & !M) | (d & M);
            }
            0x18 | 0x19 => self.s.xyoffset[(addr - 0x18) as usize] = d,
            0x1A => {
                self.s.prmodecont = d;
                self.prim_update();
            }
            0x1B => {
                self.s.prmode = d;
                self.prim_update();
            }
            0x3B => self.s.texa = d,
            0x7F => {
                // not a GS register: the port's marker (Port_GsMarker)
                if let Some(mut g) = self.gpu.take() {
                    g.native(self, d as i32);
                    self.gpu = Some(g);
                }
            }
            0x40 | 0x41 => self.s.scissor[(addr - 0x40) as usize] = d,
            0x42 | 0x43 => self.s.alpha[(addr - 0x42) as usize] = d,
            0x47 | 0x48 => self.s.test[(addr - 0x47) as usize] = d,
            0x4A | 0x4B => self.s.fba[(addr - 0x4A) as usize] = d & 1,
            0x4C | 0x4D => self.s.frame[(addr - 0x4C) as usize] = d,
            0x4E | 0x4F => self.s.zbuf[(addr - 0x4E) as usize] = d,
            0x50 => self.s.bitbltbuf = d,
            0x51 => self.s.trxpos = d,
            0x52 => self.s.trxreg = d,
            0x53 => {
                self.s.trxdir = d;
                self.transfer_begin();
            }
            _ => {}
        }
    }

    // ------------------------------------------------------------------------------------------ GIF
    /// One run of GIF data (whole quadwords).
    pub fn gif(&mut self, p: &[u8]) {
        let q64 = |o: usize| u64::from_le_bytes(p[o..o + 8].try_into().unwrap());
        let mut o = 0usize;
        let mut qwc = p.len() / 16;
        while qwc != 0 {
            if self.loops == 0 {
                self.stat[3] += 1;
                self.tag_lo = q64(o);
                self.tag_hi = q64(o + 8);
                o += 16;
                qwc -= 1;
                self.loops = (self.tag_lo & 0x7FFF) as u32;
                self.reg = 0;
                if self.loops != 0 && (self.tag_lo >> 46) & 1 != 0 && (self.tag_lo >> 58) & 3 == 0 {
                    self.reg_write(0, (self.tag_lo >> 47) & 0x7FF);
                }
                continue;
            }
            let flg = (self.tag_lo >> 58) & 3;
            let mut nreg = ((self.tag_lo >> 60) & 15) as u32;
            if nreg == 0 {
                nreg = 16;
            }
            if flg == 0 {
                let r = ((self.tag_hi >> (4 * self.reg)) & 15) as u32;
                let lo = q64(o);
                let hi = q64(o + 8);
                match r {
                    0x0 => self.reg_write(0, lo & 0x7FF),
                    0x1 => {
                        let v = (lo & 0xFF) | ((lo >> 32) & 0xFF) << 8 | (hi & 0xFF) << 16 | ((hi >> 32) & 0xFF) << 24 | (self.s.q.to_bits() as u64) << 32;
                        self.reg_write(1, v);
                    }
                    0x2 => {
                        self.s.st = lo;
                        self.s.q = f32::from_bits(hi as u32);
                    }
                    0x3 => self.reg_write(3, (lo & 0x3FFF) | ((lo >> 32) & 0x3FFF) << 16),
                    0x4 => self.vertex((lo & 0xFFFF) as u32, ((lo >> 32) & 0xFFFF) as u32, ((hi >> 4) & 0xFF_FFFF) as u32, (hi >> 47) & 1 == 0),
                    0x5 => self.vertex((lo & 0xFFFF) as u32, ((lo >> 32) & 0xFFFF) as u32, hi as u32, (hi >> 47) & 1 == 0),
                    0xC => self.vertex((lo & 0xFFFF) as u32, ((lo >> 32) & 0xFFFF) as u32, ((hi >> 4) & 0xFF_FFFF) as u32, false),
                    0xD => self.vertex((lo & 0xFFFF) as u32, ((lo >> 32) & 0xFFFF) as u32, hi as u32, false),
                    0xE => self.reg_write((hi & 0xFF) as u32, lo),
                    0xF => {}
                    _ => self.reg_write(r, lo),
                }
                o += 16;
                qwc -= 1;
                self.reg += 1;
                if self.reg == nreg {
                    self.reg = 0;
                    self.loops -= 1;
                }
            } else if flg == 1 {
                for half in 0..2 {
                    if self.loops == 0 {
                        break;
                    }
                    let r = ((self.tag_hi >> (4 * self.reg)) & 15) as u32;
                    if r != 0xE && r != 0xF {
                        self.reg_write(r, q64(o + half * 8));
                    }
                    self.reg += 1;
                    if self.reg == nreg {
                        self.reg = 0;
                        self.loops -= 1;
                    }
                }
                o += 16;
                qwc -= 1;
            } else {
                let n = (self.loops as usize).min(qwc);
                self.stat[6] += n as u32;
                self.transfer_data(&p[o..o + n * 16]);
                o += n * 16;
                qwc -= n;
                self.loops -= n as u32;
            }
        }
    }

    fn take_kicks(&mut self) {
        if !self.vu.kicked.is_empty() {
            let k = std::mem::take(&mut self.vu.kicked);
            self.gif(&k);
        }
    }

    fn vif_cmd_data(&mut self, pend: u32, b: &[u32]) {
        let c = (pend >> 24) & 0x7F;
        let pn = (pend >> 16) & 0xFF;
        match c {
            0x20 => self.vu.set_mask(b[0]),
            0x30 => self.vu.set_row(b),
            0x31 => self.vu.set_col(b),
            0x4A => self.vu.program(pend & 0xFFFF, b),
            _ => self.vu.unpack(c, pn, pend & 0xFFFF, b),
        }
    }

    // ------------------------------------------------------------------------------------------ VIF1
    /// One run of VIF data (32-bit words): DIRECT data to the GS, everything else to VU1.
    fn vif(&mut self, w: &[u32]) {
        let count = w.len();
        let mut i = 0usize;
        while i < count {
            if self.direct != 0 {
                let qwc = ((count - i) / 4).min(self.direct as usize);
                if qwc == 0 {
                    break;
                }
                self.stat[2] += qwc as u32;
                self.gif(as_bytes(&w[i..i + qwc * 4]));
                i += qwc * 4;
                self.direct -= qwc as u32;
                continue;
            }
            if self.need != 0 {
                let need = self.need as usize;
                let n = (count - i).min(need - self.buf.len());
                self.buf.extend_from_slice(&w[i..i + n]);
                i += n;
                if self.buf.len() == need {
                    let b = std::mem::take(&mut self.buf);
                    self.vif_cmd_data(self.pend, &b);
                    self.need = 0;
                }
                continue;
            }
            let code = w[i];
            i += 1;
            let cmd = (code >> 24) & 0x7F;
            let num = (code >> 16) & 0xFF;
            let imm = code & 0xFFFF;
            self.stat[1] += 1;
            self.pend = code;
            match cmd {
                0x01 => self.vu.set_cycle(imm & 0xFF, (imm >> 8) & 0xFF),
                0x02 => self.vu.set_offset(imm),
                0x03 => self.vu.set_base(imm),
                0x04 => self.vu.set_itop(imm),
                0x05 => self.vu.set_mode(imm),
                0x14 | 0x15 | 0x17 => {
                    let t = std::time::Instant::now();
                    self.vu_call(if cmd == 0x17 { -1 } else { imm as i32 });
                    self.times[1] += t.elapsed().as_secs_f64();
                    self.take_kicks();
                }
                0x20 => self.need = 1,
                0x30 | 0x31 => self.need = 4,
                0x4A => self.need = (if num != 0 { num } else { 256 }) * 2,
                0x50 | 0x51 => self.direct = if imm != 0 { imm } else { 65536 },
                _ => {
                    if cmd >= 0x60 {
                        self.need = self.vu.unpack_words(cmd, num);
                    }
                }
            }
            if self.need != 0 {
                self.buf.clear();
            }
        }
    }

    // ------------------------------------------------------------------------------------------ DMA and frames
    fn words(c: &Ctx, a: u32, n: u32) -> Vec<u32> {
        let mut w = vec![0u32; n as usize];
        // (the host is little-endian, as the PS2: the words' bytes are memory's bytes)
        c.m.read_bytes(a, unsafe { std::slice::from_raw_parts_mut(w.as_mut_ptr() as *mut u8, w.len() * 4) });
        w
    }

    /// Executes one frame's list: the DMA chain at `tadr` on VIF1, then the end of the frame.
    pub fn run_chain(&mut self, c: &Ctx, mut tadr: u32, tte: bool) {
        let mut stack = [0u32; 2];
        let mut sp = 0usize;
        for _guard in 0..100_000 {
            let tag = Self::words(c, tadr, 4);
            let qwc = tag[0] & 0xFFFF;
            let id = (tag[0] >> 28) & 7;
            let addr = tag[1];
            let mut end = false;
            let data_addr = match id {
                0 => {
                    end = true;
                    addr
                }
                1 => {
                    let d = tadr + 16;
                    tadr = tadr + 16 + qwc * 16;
                    d
                }
                2 => {
                    let d = tadr + 16;
                    tadr = addr;
                    d
                }
                3 | 4 => {
                    let d = addr;
                    tadr += 16;
                    d
                }
                5 => {
                    let d = tadr + 16;
                    if sp < 2 {
                        stack[sp] = tadr + 16 + qwc * 16;
                        sp += 1;
                    }
                    tadr = addr;
                    d
                }
                6 => {
                    let d = tadr + 16;
                    if sp > 0 {
                        sp -= 1;
                        tadr = stack[sp];
                    } else {
                        end = true;
                    }
                    d
                }
                _ => {
                    end = true;
                    tadr + 16
                }
            };
            self.stat[0] += 1;
            if tte {
                self.vif(&tag[2..4]);
            }
            let data = Self::words(c, data_addr, qwc * 4);
            self.vif(&data);
            if end {
                break;
            }
        }
        if self.verbose && (self.frame < 4 || self.frame % 60 == 0) {
            eprintln!(
                "gs: frame {}: {} DMA tags, {} VIF codes, {} DIRECT qwords, {} GIF tags, {} register writes, {} vertices, {} image qwords, {} targets",
                self.frame, self.stat[0], self.stat[1], self.stat[2], self.stat[3], self.stat[4], self.stat[5], self.stat[6], self.targets.len()
            );
        }
        self.stat = [0; 8];
        self.vu_instr += self.vu.stat_instr as u64;
        self.vu.frame_end(self.verbose);
        self.end_frame();
    }

    /// A transfer on the GIF channel itself: `qwc` quadwords at `addr`, or a source chain starting there.
    pub fn gif_channel(&mut self, c: &Ctx, mut addr: u32, qwc: u32, chain: bool) {
        let read = |a: u32, n: u32| {
            let mut b = vec![0u8; n as usize * 16];
            c.m.read_bytes(a, &mut b);
            b
        };
        if !chain {
            let b = read(addr, qwc);
            self.gif(&b);
            return;
        }
        for _guard in 0..100_000 {
            let tag = Self::words(c, addr, 4);
            let n = tag[0] & 0xFFFF;
            let id = (tag[0] >> 28) & 7;
            let r = tag[1];
            if id == 0 || id == 3 || id == 4 {
                let b = read(r, n);
                self.gif(&b);
                addr += 16;
            } else {
                let b = read(addr + 16, n);
                self.gif(&b);
                addr = if id == 2 { r } else { addr + 16 + n * 16 };
            }
            if id == 0 || id == 7 {
                break;
            }
        }
    }

    /// The finished frame: the picture of the buffer the frame was drawn into, and a screenshot when asked.
    fn end_frame(&mut self) {
        if self.verbose && self.frame % 30 == 29 {
            let w = self.wall.elapsed().as_secs_f64();
            let t = self.times;
            eprintln!(
                "gs: frames {}..{}: {:.1} ms a frame: GS {:.1} (VU1 {:.1}, {} instructions), frame end {:.1}, the game {:.1}",
                self.frame - 29,
                self.frame,
                w / 30.0 * 1e3,
                (t[0] - t[3]) / 30.0 * 1e3,
                t[1] / 30.0 * 1e3,
                self.vu_instr / 30,
                t[3] / 30.0 * 1e3,
                (w - t[0]) / 30.0 * 1e3
            );
            self.times = [0.0; 4];
            self.vu_instr = 0;
            self.wall = std::time::Instant::now();
        }
        let te = std::time::Instant::now();
        if let Some(mut g) = self.gpu.take() {
            g.frame_end(self);
            self.gpu = Some(g);
            self.frame += 1;
            self.times[3] += te.elapsed().as_secs_f64();
            return;
        }
        let mut best: Option<usize> = None;
        for (i, t) in self.targets.iter().enumerate() {
            let pick = if self.main_fbp >= 0 { t.fbp == self.main_fbp as u32 } else { best.map_or(true, |b| t.drawn > self.targets[b].drawn) };
            if pick {
                best = Some(i);
            }
        }
        if let Some(b) = best {
            let fbp = self.targets[b].fbp;
            let fr = if (self.s.frame[0] & 0x1FF) as u32 == fbp {
                self.s.frame[0]
            } else if (self.s.frame[1] & 0x1FF) as u32 == fbp {
                self.s.frame[1]
            } else {
                self.shown
            };
            self.shown = fr;
            let fbw = ((fr >> 16) & 0x3F) as u32;
            let fpsm = ((fr >> 24) & 0x3F) as u32;
            let w = if fbw != 0 { fbw } else { 10 };
            // the buffer (512 pixels wide in a fight) fills the 640-pixel-wide picture, as on the TV
            let src_w = (w * 64).min(640);
            for y in 0..448u32 {
                for x in 0..640u32 {
                    let sx = x * src_w / 640;
                    let mut px = self.rd(fbp << 5, w, fpsm, sx, y);
                    if Self::psm_bits(fpsm) == 16 {
                        px = (px & 31) << 3 | ((px >> 5) & 31) << 11 | ((px >> 10) & 31) << 19;
                    }
                    let o = ((y * 640 + x) * 4) as usize;
                    self.picture[o] = px as u8;
                    self.picture[o + 1] = (px >> 8) as u8;
                    self.picture[o + 2] = (px >> 16) as u8;
                    self.picture[o + 3] = 255;
                }
            }
            self.picture_frame = self.frame;
            if self.shot_every > 0 && self.frame % self.shot_every == 0 {
                let _ = std::fs::create_dir_all("shots");
                let _ = write_png(&format!("shots/frame_{:05}.png", self.frame), 640, 448, &self.picture);
            }
        }
        for t in self.targets.iter_mut() {
            t.drawn = 0;
        }
        self.frame += 1;
    }
}

/// Words as their bytes (little-endian host, as the PS2).
#[inline(always)]
fn as_bytes(w: &[u32]) -> &[u8] {
    unsafe { std::slice::from_raw_parts(w.as_ptr() as *const u8, w.len() * 4) }
}

// ---------------------------------------------------------------------------------------------- PNG
fn crc32(data: &[u8]) -> u32 {
    let mut table = [0u32; 256];
    for (n, e) in table.iter_mut().enumerate() {
        let mut c = n as u32;
        for _ in 0..8 {
            c = if c & 1 != 0 { 0xEDB8_8320 ^ (c >> 1) } else { c >> 1 };
        }
        *e = c;
    }
    let mut c = 0xFFFF_FFFFu32;
    for &b in data {
        c = table[((c ^ b as u32) & 0xFF) as usize] ^ (c >> 8);
    }
    c ^ 0xFFFF_FFFF
}

/// An RGBA picture as a PNG file (stored deflate blocks: no compression, no dependencies).
pub fn write_png(path: &str, w: u32, h: u32, rgba: &[u8]) -> std::io::Result<()> {
    let mut raw = Vec::with_capacity(((w * 4 + 1) * h) as usize);
    for y in 0..h as usize {
        raw.push(0);
        raw.extend_from_slice(&rgba[y * w as usize * 4..(y + 1) * w as usize * 4]);
    }
    let mut z = vec![0x78, 0x01];
    let mut a: u32 = 1;
    let mut b: u32 = 0;
    for &x in &raw {
        a = (a + x as u32) % 65521;
        b = (b + a) % 65521;
    }
    for (i, chunk) in raw.chunks(65535).enumerate() {
        let last = (i + 1) * 65535 >= raw.len();
        z.push(last as u8);
        let n = chunk.len() as u16;
        z.extend_from_slice(&n.to_le_bytes());
        z.extend_from_slice(&(!n).to_le_bytes());
        z.extend_from_slice(chunk);
    }
    z.extend_from_slice(&((b << 16) | a).to_be_bytes());
    let mut out = vec![0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A];
    let mut chunk = |kind: &[u8], data: &[u8]| {
        out.extend_from_slice(&(data.len() as u32).to_be_bytes());
        let mut c = kind.to_vec();
        c.extend_from_slice(data);
        out.extend_from_slice(&c);
        out.extend_from_slice(&crc32(&c).to_be_bytes());
    };
    let mut ihdr = Vec::new();
    ihdr.extend_from_slice(&w.to_be_bytes());
    ihdr.extend_from_slice(&h.to_be_bytes());
    ihdr.extend_from_slice(&[8, 6, 0, 0, 0]);
    chunk(b"IHDR", &ihdr);
    chunk(b"IDAT", &z);
    chunk(b"IEND", &[]);
    std::fs::write(path, out)
}
