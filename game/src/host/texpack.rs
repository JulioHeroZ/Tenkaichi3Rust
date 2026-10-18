//! Texture packs in the naming of PCSX2's texture replacement (packs made for the game under that emulator work as
//! they are; the same as the C port, Tenkaichi3Decomp port/src/gs/gs_texpack.c). The texture quality of
//! Options > Graphics (host/settings.rs textures): Low = the PS2's own textures; Medium = the pack's pictures at
//! half their size; High and Ultra = the pack as it is.
//!
//! A file is named <texture hash>-<palette hash>-<bits>.dds (or .png) (two parts, without the palette hash, for a
//! texture without a palette), the hashes in hexadecimal without leading zeros:
//!   texture hash  XXH3 (64 bits, host/xxh3.rs) over the texture's raw 256-byte blocks of GS memory, block after
//!                 block in row-major block order over the texture's rectangle: the data as it lies in GS memory
//!   palette hash  XXH3 over the palette's 32-bit entries in index order, from the lowest to the highest index the
//!                 texture uses (some files: over the whole palette; both are looked up)
//!   bits          PSM | log2(width) << 6 | log2(height) << 10 | TCC << 14
//! A pack's alpha is the GS's own (0x80 = opaque), like the textures it replaces.
//!
//! The files are looked for at any depth below assets/textures (the installer puts a pack there) and the folder
//! BT3_TEXTURES names; where a name is there twice, the first one found (folders in name order) is used. Files are
//! read and decoded on worker threads; until a replacement is ready the original is shown.
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Mutex, OnceLock};

/// (texture hash, palette hash or 0, bits, has a palette) -> file
type Index = HashMap<(u64, u64, u32, bool), PathBuf>;

/// "1dd4c76113969303-56e3d4469d2ad392-00005e54.dds": (texture hash, palette hash, bits, has a palette)
fn parse_name(name: &str) -> Option<(u64, u64, u32, bool)> {
    let lower = name.to_ascii_lowercase();
    let stem = lower.strip_suffix(".dds").or_else(|| lower.strip_suffix(".png"))?;
    let parts: Vec<&str> = stem.split('-').collect();
    if !(2..=3).contains(&parts.len()) || parts.iter().any(|p| p.is_empty() || p.len() > 16 || !p.bytes().all(|c| c.is_ascii_hexdigit())) {
        return None;
    }
    let tex = u64::from_str_radix(parts[0], 16).ok()?;
    let bits = u32::from_str_radix(parts[parts.len() - 1], 16).ok()?;
    if parts.len() == 3 {
        Some((tex, u64::from_str_radix(parts[1], 16).ok()?, bits, true))
    } else {
        Some((tex, 0, bits, false))
    }
}

fn index_dir(dir: &Path, idx: &mut Index, depth: u32) {
    let Ok(rd) = std::fs::read_dir(dir) else { return };
    let mut entries: Vec<PathBuf> = rd.flatten().map(|e| e.path()).collect();
    entries.sort();
    for p in entries {
        if p.is_dir() {
            if depth < 8 {
                index_dir(&p, idx, depth + 1);
            }
            continue;
        }
        let Some(key) = p.file_name().and_then(|n| n.to_str()).and_then(parse_name) else { continue };
        idx.entry(key).or_insert(p);
    }
}

static ROOT: OnceLock<PathBuf> = OnceLock::new();

/// The assets folder (host/harness.rs, at the start).
pub fn set_root(assets: &Path) {
    let _ = ROOT.set(assets.to_path_buf());
}

/// The pack's files, found once.
fn pack() -> &'static Index {
    static P: OnceLock<Index> = OnceLock::new();
    P.get_or_init(|| {
        let mut idx = Index::new();
        let mut roots: Vec<PathBuf> = Vec::new();
        if let Some(a) = ROOT.get() {
            roots.push(a.join("textures"));
        }
        if let Ok(t) = std::env::var("BT3_TEXTURES") {
            roots.push(PathBuf::from(t));
        }
        for r in &roots {
            index_dir(r, &mut idx, 0);
        }
        if !idx.is_empty() {
            eprintln!("bt3: texture pack: {} replacement textures", idx.len());
        }
        idx
    })
}

/// Whether a pack is there.
pub fn available() -> bool {
    !pack().is_empty()
}

/// How many times a quality makes the pack's pictures smaller (None: the PS2's own textures).
fn halvings(quality: i32) -> Option<u32> {
    match quality {
        1 => Some(1),
        2 | 3 => Some(0),
        _ => None,
    }
}

/// A texture's replacement file, as found.
pub struct Found {
    pub key: u64,
    pub file: PathBuf,
    pub halvings: u32,
}

/// What a texture is, for finding its replacement: its place and format in GS memory (as TEX0 says), the
/// texture's raw blocks there (host/gs Gs::block), and its palette in index order (32-bit entries).
pub struct Query<'a> {
    pub psm: u32,
    pub tw: u32,
    pub th: u32,
    pub tcc: u32,
    pub cpsm: u32,
    /// the 256-byte blocks over the rectangle, in row-major block order
    pub blocks: &'a [u8],
    /// the palette (256 or 16 entries, index order); empty without one
    pub clut: &'a [u32],
}

/// The block a format's texture is made of, in pixels (None: a format packs are not made for here).
pub fn block_size(psm: u32) -> Option<(u32, u32)> {
    match psm {
        0x13 => Some((16, 16)),
        0x14 => Some((32, 16)),
        0x00 => Some((8, 8)),
        _ => None,
    }
}

/// The file replacing a texture, for a texture quality.
pub fn lookup(quality: i32, q: &Query) -> Option<Found> {
    let halvings = halvings(quality)?;
    let idx = pack();
    if idx.is_empty() {
        return None;
    }
    let (bw, bh) = block_size(q.psm)?;
    if q.tw < bw || q.th < bh {
        return None; // smaller than a block: hashed another way there, which is not worked out
    }
    let tex = crate::host::xxh3::hash64(q.blocks);
    let lg = |v: u32| (0..31).find(|&l| (1u32 << l) >= v).unwrap_or(31);
    let (wl, hl) = (lg(q.tw), lg(q.th));
    let bits = q.psm | wl << 6 | hl << 10 | (q.tcc & 1) << 14;
    let file = if q.psm == 0x00 {
        idx.get(&(tex, 0, bits, false))?
    } else {
        if q.cpsm != 0 {
            return None; // 16-bit palettes are expanded with TEXA there: not seen in this game
        }
        // the indices the texture uses, from its blocks
        let (mut lo, mut hi) = (255usize, 0usize);
        for &b in q.blocks {
            if q.psm == 0x13 {
                lo = lo.min(b as usize);
                hi = hi.max(b as usize);
            } else {
                for v in [(b & 15) as usize, (b >> 4) as usize] {
                    lo = lo.min(v);
                    hi = hi.max(v);
                }
            }
        }
        let bytes = |c: &[u32]| c.iter().flat_map(|v| v.to_le_bytes()).collect::<Vec<u8>>();
        let used = crate::host::xxh3::hash64(&bytes(&q.clut[lo.min(hi)..=hi]));
        let whole = crate::host::xxh3::hash64(&bytes(q.clut));
        if std::env::var_os("BT3_TEX_LOG").is_some() {
            let f = idx.get(&(tex, used, bits, true)).or_else(|| idx.get(&(tex, whole, bits, true)));
            eprintln!("texpack: {} {}x{} {:x}-{:x}-{:08x}", if f.is_some() { "found  " } else { "missing" }, q.tw, q.th, tex, used, bits);
        }
        idx.get(&(tex, used, bits, true)).or_else(|| idx.get(&(tex, whole, bits, true)))?
    };
    let key = crate::host::xxh3::hash64(file.to_string_lossy().as_bytes()) ^ halvings as u64;
    Some(Found { key, file: file.clone(), halvings })
}

// ------------------------------------------------------------------------------------------------ DDS
/// A picture decoded from a DDS file: RGBA, the pack's own alpha (255 opaque).
pub struct Picture {
    pub w: u32,
    pub h: u32,
    pub rgba: Vec<u8>,
    /// what its alpha is multiplied by to give the GS's (0x80 opaque): 1, a PCSX2 pack's alpha is the GS's own
    pub alpha_scale: f32,
}

fn rgb565(c: u16) -> [u8; 3] {
    let r = (c >> 11) & 31;
    let g = (c >> 5) & 63;
    let b = c & 31;
    [(r << 3 | r >> 2) as u8, (g << 2 | g >> 4) as u8, (b << 3 | b >> 2) as u8]
}

fn colour_block(b: &[u8], out: &mut [[u8; 4]; 16], dxt1: bool) {
    let c0 = u16::from_le_bytes([b[0], b[1]]);
    let c1 = u16::from_le_bytes([b[2], b[3]]);
    let (a, bb) = (rgb565(c0), rgb565(c1));
    let mix = |x: u8, y: u8, wx: u16, wy: u16| ((x as u16 * wx + y as u16 * wy) / (wx + wy)) as u8;
    let mut pal = [[0u8; 4]; 4];
    pal[0] = [a[0], a[1], a[2], 255];
    pal[1] = [bb[0], bb[1], bb[2], 255];
    if c0 > c1 || !dxt1 {
        pal[2] = [mix(a[0], bb[0], 2, 1), mix(a[1], bb[1], 2, 1), mix(a[2], bb[2], 2, 1), 255];
        pal[3] = [mix(a[0], bb[0], 1, 2), mix(a[1], bb[1], 1, 2), mix(a[2], bb[2], 1, 2), 255];
    } else {
        pal[2] = [mix(a[0], bb[0], 1, 1), mix(a[1], bb[1], 1, 1), mix(a[2], bb[2], 1, 1), 255];
        pal[3] = [0, 0, 0, 0];
    }
    let bits = u32::from_le_bytes([b[4], b[5], b[6], b[7]]);
    for (i, o) in out.iter_mut().enumerate() {
        let p = pal[((bits >> (2 * i)) & 3) as usize];
        o[0] = p[0];
        o[1] = p[1];
        o[2] = p[2];
        if dxt1 {
            o[3] = p[3];
        }
    }
}

fn alpha_block5(b: &[u8], out: &mut [[u8; 4]; 16]) {
    let (a0, a1) = (b[0] as u32, b[1] as u32);
    let mut pal = [0u32; 8];
    pal[0] = a0;
    pal[1] = a1;
    if a0 > a1 {
        for i in 1..7 {
            pal[i + 1] = ((7 - i as u32) * a0 + i as u32 * a1) / 7;
        }
    } else {
        for i in 1..5 {
            pal[i + 1] = ((5 - i as u32) * a0 + i as u32 * a1) / 5;
        }
        pal[6] = 0;
        pal[7] = 255;
    }
    let mut bits = 0u64;
    for i in 0..6 {
        bits |= (b[2 + i] as u64) << (8 * i);
    }
    for (i, o) in out.iter_mut().enumerate() {
        o[3] = pal[((bits >> (3 * i)) & 7) as usize] as u8;
    }
}

/// A DDS file: DXT1 / DXT3 / DXT5 or 32-bit pixels (its first level).
pub fn dds_decode(data: &[u8]) -> Option<Picture> {
    if data.len() < 128 || &data[0..4] != b"DDS " {
        return None;
    }
    let u = |o: usize| u32::from_le_bytes(data[o..o + 4].try_into().unwrap());
    let (h, w) = (u(12), u(16));
    let pf_flags = u(80);
    let fourcc = &data[84..88];
    let body = &data[128..];
    let mut rgba = vec![0u8; (w * h * 4) as usize];
    if pf_flags & 4 != 0 {
        let kind = match fourcc {
            b"DXT1" => 1,
            b"DXT3" => 3,
            b"DXT5" => 5,
            _ => return None,
        };
        let bs = if kind == 1 { 8 } else { 16 };
        let (bw, bh) = (w.div_ceil(4) as usize, h.div_ceil(4) as usize);
        if body.len() < bw * bh * bs {
            return None;
        }
        let mut px = [[0u8; 4]; 16];
        for by in 0..bh {
            for bx in 0..bw {
                let b = &body[(by * bw + bx) * bs..][..bs];
                match kind {
                    1 => colour_block(b, &mut px, true),
                    3 => {
                        colour_block(&b[8..], &mut px, false);
                        for (i, o) in px.iter_mut().enumerate() {
                            let a = (b[i / 2] >> (4 * (i & 1))) & 15;
                            o[3] = a << 4 | a;
                        }
                    }
                    _ => {
                        colour_block(&b[8..], &mut px, false);
                        alpha_block5(b, &mut px);
                    }
                }
                for (i, p) in px.iter().enumerate() {
                    let (x, y) = (bx * 4 + i % 4, by * 4 + i / 4);
                    if x < w as usize && y < h as usize {
                        rgba[(y * w as usize + x) * 4..][..4].copy_from_slice(p);
                    }
                }
            }
        }
    } else if u(88) == 32 {
        let (rm, gm, bm, am) = (u(92), u(96), u(100), u(104));
        let sh = |m: u32| if m == 0 { 0 } else { m.trailing_zeros() };
        if body.len() < (w * h * 4) as usize {
            return None;
        }
        for i in 0..(w * h) as usize {
            let v = u32::from_le_bytes(body[i * 4..i * 4 + 4].try_into().unwrap());
            rgba[i * 4] = ((v & rm) >> sh(rm)) as u8;
            rgba[i * 4 + 1] = ((v & gm) >> sh(gm)) as u8;
            rgba[i * 4 + 2] = ((v & bm) >> sh(bm)) as u8;
            rgba[i * 4 + 3] = if am != 0 { ((v & am) >> sh(am)) as u8 } else { 255 };
        }
    } else {
        return None;
    }
    Some(Picture { w, h, rgba, alpha_scale: 1.0 })
}

/// Half the size (the average of each 2 x 2).
fn halve(p: Picture) -> Picture {
    if p.w < 2 || p.h < 2 {
        return p;
    }
    let (w, h) = (p.w / 2, p.h / 2);
    let mut out = vec![0u8; (w * h * 4) as usize];
    for y in 0..h as usize {
        for x in 0..w as usize {
            for c in 0..4 {
                let s = |dx: usize, dy: usize| p.rgba[((2 * y + dy) * p.w as usize + 2 * x + dx) * 4 + c] as u32;
                out[(y * w as usize + x) * 4 + c] = ((s(0, 0) + s(1, 0) + s(0, 1) + s(1, 1) + 2) / 4) as u8;
            }
        }
    }
    Picture { w, h, rgba: out, alpha_scale: p.alpha_scale }
}

/// Grey (its colour channels equal, near enough).

/// A PNG file (the pack's alpha as it is).
fn png_decode(data: &[u8]) -> Option<Picture> {
    let p = crate::host::assets::png_read(data).ok()?;
    let rgba = match &p.palette {
        Some(pal) => p.data.iter().flat_map(|&i| pal.get(i as usize).copied().unwrap_or([0; 4])).collect(),
        None => p.data,
    };
    if rgba.len() != (p.w * p.h * 4) as usize {
        return None;
    }
    Some(Picture { w: p.w, h: p.h, rgba, alpha_scale: 1.0 })
}

// ------------------------------------------------------------------------------------------------ loading
/// A replacement asked for (the renderer's key for it) and the result.
pub struct Job {
    pub found: Found,
}

pub struct Done {
    pub key: u64,
    pub picture: Option<Picture>,
}

struct Workers {
    tx: Sender<Job>,
    rx: Mutex<Receiver<Done>>,
}

fn workers() -> &'static Workers {
    static W: OnceLock<Workers> = OnceLock::new();
    W.get_or_init(|| {
        let (tx, jobs) = channel::<Job>();
        let (done_tx, rx) = channel::<Done>();
        let jobs = std::sync::Arc::new(Mutex::new(jobs));
        for _ in 0..2 {
            let jobs = jobs.clone();
            let done_tx = done_tx.clone();
            std::thread::spawn(move || loop {
                let job = match jobs.lock().unwrap().recv() {
                    Ok(j) => j,
                    Err(_) => return,
                };
                let f = &job.found;
                let data = std::fs::read(&f.file).ok();
                let is_png = f.file.extension().map_or(false, |e| e.eq_ignore_ascii_case("png"));
                let mut picture = data.as_deref().and_then(|d| if is_png { png_decode(d) } else { dds_decode(d) });
                for _ in 0..f.halvings {
                    picture = picture.map(halve);
                }
                if done_tx.send(Done { key: f.key, picture }).is_err() {
                    return;
                }
            });
        }
        Workers { tx, rx: Mutex::new(rx) }
    })
}

pub fn request(job: Job) {
    let _ = workers().tx.send(job);
}

/// The replacements decoded since the last call.
pub fn finished() -> Vec<Done> {
    let w = workers();
    let rx = w.rx.lock().unwrap();
    rx.try_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn names() {
        assert_eq!(parse_name("1dd4c76113969303-56e3d4469d2ad392-00005e54.dds"), Some((0x1dd4c76113969303, 0x56e3d4469d2ad392, 0x5e54, true)));
        assert_eq!(parse_name("4e0ee2ff8b0877f-156cb9cc3d4e064-00005554.DDS"), Some((0x4e0ee2ff8b0877f, 0x156cb9cc3d4e064, 0x5554, true)));
        assert_eq!(parse_name("abc-00004000.png"), Some((0xabc, 0, 0x4000, false)));
        assert_eq!(parse_name("tex1_64x64_abc_def_9.dds"), None);
    }
}
