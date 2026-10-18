//! HD texture packs made for the Wii version of the game under Dolphin (RDSE70): the texture quality of
//! Options > Graphics (host/settings.rs textures): Low = the PS2's own textures; Medium = the 2x pack shown at the
//! original's size (redrawn art, not upscaled); High = the 2x pack ("1080p"); Ultra = the 4x pack ("4K").
//!
//! The Wii game uses the same textures as the PS2's, index for index, so a pack's file is found from the texture
//! the game uploads, as Dolphin names it: `tex1_<w>x<h>_<texture hash>_<palette hash>_<format>.dds`, where
//!   texture hash  XXH64 (seed 0) of the indices in the Wii's tiled layout (blocks of 8 x 8 for 4-bit indices,
//!                 two per byte, the first in the high nibble; 8 x 4 for 8-bit), blocks in rows
//!   palette hash  XXH64 of the palette entries from the lowest index the texture uses to the highest, each as
//!                 the Wii's RGB5A3, big-endian (opaque: 1rrrrrgggggbbbbb; else 0aaarrrrggggbbbb)
//!   format        8 (C4) or 9 (C8)
//! (Dolphin's VideoCommon/TextureInfo.cpp.) Checked on the extracted textures: every sample found its file, and
//! the palette hash matched for every opaque palette; for a palette with see-through colours the Wii's alpha
//! rounding is not known, so the first file of the texture is used.
//!
//! The packs live in assets/textures/2x and assets/textures/4x (any folders below; the installer puts them there,
//! BT3_TEXTURES=<folder> for one more place to look). Files are read and decoded on worker threads; until a
//! replacement is ready the original is shown.
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Mutex, OnceLock};

// ------------------------------------------------------------------------------------------------ XXH64
const P1: u64 = 0x9E37_79B1_85EB_CA87;
const P2: u64 = 0xC2B2_AE3D_27D4_EB4F;
const P3: u64 = 0x1656_67B1_9E37_79F9;
const P4: u64 = 0x85EB_CA77_C2B2_AE63;
const P5: u64 = 0x27D4_EB2F_1656_67C5;

fn round(acc: u64, v: u64) -> u64 {
    acc.wrapping_add(v.wrapping_mul(P2)).rotate_left(31).wrapping_mul(P1)
}

fn merge(acc: u64, v: u64) -> u64 {
    (acc ^ round(0, v)).wrapping_mul(P1).wrapping_add(P4)
}

pub fn xxh64(data: &[u8], seed: u64) -> u64 {
    let r64 = |b: &[u8]| u64::from_le_bytes(b[..8].try_into().unwrap());
    let r32 = |b: &[u8]| u32::from_le_bytes(b[..4].try_into().unwrap()) as u64;
    let n = data.len();
    let mut p = data;
    let mut h = if n >= 32 {
        let mut v = [seed.wrapping_add(P1).wrapping_add(P2), seed.wrapping_add(P2), seed, seed.wrapping_sub(P1)];
        while p.len() >= 32 {
            for (i, x) in v.iter_mut().enumerate() {
                *x = round(*x, r64(&p[i * 8..]));
            }
            p = &p[32..];
        }
        let mut h = v[0].rotate_left(1).wrapping_add(v[1].rotate_left(7)).wrapping_add(v[2].rotate_left(12)).wrapping_add(v[3].rotate_left(18));
        for x in v {
            h = merge(h, x);
        }
        h
    } else {
        seed.wrapping_add(P5)
    };
    h = h.wrapping_add(n as u64);
    while p.len() >= 8 {
        h ^= round(0, r64(p));
        h = h.rotate_left(27).wrapping_mul(P1).wrapping_add(P4);
        p = &p[8..];
    }
    if p.len() >= 4 {
        h ^= r32(p).wrapping_mul(P1);
        h = h.rotate_left(23).wrapping_mul(P2).wrapping_add(P3);
        p = &p[4..];
    }
    for &b in p {
        h ^= (b as u64).wrapping_mul(P5);
        h = h.rotate_left(11).wrapping_mul(P1);
    }
    h ^= h >> 33;
    h = h.wrapping_mul(P2);
    h ^= h >> 29;
    h = h.wrapping_mul(P3);
    h ^ (h >> 32)
}

// ------------------------------------------------------------------------------------------------ names
/// The texture hash of indices `idx` (one per texel, `w` x `h`), 4- or 8-bit, in the Wii's layout.
pub fn texture_hash(idx: &[u8], w: usize, h: usize, bits: u32) -> u64 {
    let (bw, bh) = if bits == 4 { (8, 8) } else { (8, 4) };
    let mut out = Vec::with_capacity(w * h);
    for by in (0..h).step_by(bh) {
        for bx in (0..w).step_by(bw) {
            for y in by..by + bh {
                let row = &idx[y * w + bx..y * w + bx + 8];
                if bits == 4 {
                    for i in (0..8).step_by(2) {
                        out.push((row[i] & 15) << 4 | (row[i + 1] & 15));
                    }
                } else {
                    out.extend_from_slice(row);
                }
            }
        }
    }
    xxh64(&out, 0)
}

/// The palette hash: entries `pal` (RGBA with the PS2's alpha, 0x80 opaque) from the lowest to the highest index
/// of `idx`.
pub fn palette_hash(idx: &[u8], pal: &[[u8; 4]]) -> u64 {
    let lo = *idx.iter().min().unwrap_or(&0) as usize;
    let hi = (*idx.iter().max().unwrap_or(&0) as usize).min(pal.len().saturating_sub(1));
    let mut b = Vec::with_capacity(2 * (hi + 1 - lo));
    for p in &pal[lo..=hi] {
        let (r, g, bl) = (p[0] as u16, p[1] as u16, p[2] as u16);
        let a = (p[3] as u16 * 2).min(255);
        let v = if a >= 255 { 0x8000 | (r >> 3) << 10 | (g >> 3) << 5 | bl >> 3 } else { (a >> 5) << 12 | (r >> 4) << 8 | (g >> 4) << 4 | bl >> 4 };
        b.extend_from_slice(&v.to_be_bytes());
    }
    xxh64(&b, 0)
}

// ------------------------------------------------------------------------------------------------ the packs
/// The files of one pack: texture hash -> (palette hash, file) for each file of the texture.
#[derive(Default)]
struct Index {
    files: HashMap<u64, Vec<(u64, PathBuf)>>,
}

fn index_dir(dir: &Path, idx: &mut Index, depth: u32) {
    let Ok(rd) = std::fs::read_dir(dir) else { return };
    for e in rd.flatten() {
        let p = e.path();
        if p.is_dir() {
            if depth < 4 {
                index_dir(&p, idx, depth + 1);
            }
            continue;
        }
        let Some(name) = p.file_name().and_then(|n| n.to_str()) else { continue };
        let lower = name.to_ascii_lowercase();
        if !lower.starts_with("tex1_") || !lower.ends_with(".dds") {
            continue;
        }
        let mut parts: Vec<&str> = lower[..lower.len() - 4].split('_').collect();
        if parts.get(2) == Some(&"m") {
            parts.remove(2);
        }
        if parts.len() < 4 {
            continue;
        }
        let Ok(tex) = u64::from_str_radix(parts[2], 16) else { continue };
        let tlut = if parts.len() >= 5 { u64::from_str_radix(parts[3], 16).unwrap_or(0) } else { 0 };
        idx.files.entry(tex).or_default().push((tlut, p.clone()));
    }
}

static ROOT: OnceLock<PathBuf> = OnceLock::new();

/// The assets folder (host/harness.rs, at the start).
pub fn set_root(assets: &Path) {
    let _ = ROOT.set(assets.to_path_buf());
}

/// The two packs (2x, 4x), found once.
fn packs() -> &'static [Index; 2] {
    static P: OnceLock<[Index; 2]> = OnceLock::new();
    P.get_or_init(|| {
        let mut out = [Index::default(), Index::default()];
        let mut roots: Vec<PathBuf> = Vec::new();
        if let Some(a) = ROOT.get() {
            roots.push(a.join("textures"));
        }
        if let Ok(t) = std::env::var("BT3_TEXTURES") {
            roots.push(PathBuf::from(t));
        }
        for r in &roots {
            for (k, sub) in ["2x", "4x"].iter().enumerate() {
                index_dir(&r.join(sub), &mut out[k], 0);
            }
        }
        for (k, n) in ["2x", "4x"].iter().enumerate() {
            if !out[k].files.is_empty() {
                eprintln!("bt3: texture pack {n}: {} textures", out[k].files.len());
            }
        }
        out
    })
}

/// Which pack a quality uses and how much its pictures are made smaller: (pack, halvings).
fn source(quality: i32) -> Option<(usize, u32)> {
    let p = packs();
    let has = |k: usize| !p[k].files.is_empty();
    match quality {
        3 if has(1) => Some((1, 0)),
        3 if has(0) => Some((0, 0)),
        2 if has(0) => Some((0, 0)),
        2 if has(1) => Some((1, 1)),
        1 if has(0) => Some((0, 1)),
        1 if has(1) => Some((1, 2)),
        _ => None,
    }
}

/// Whether any pack is there.
pub fn available() -> bool {
    packs().iter().any(|p| !p.files.is_empty())
}

/// The texture's files, as found for one texture: the file (or, when the palette hash matches none of them, all
/// of them: the worker keeps the one whose colours are nearest the original's), and the job's key.
pub struct Found {
    pub key: u64,
    pub files: Vec<PathBuf>,
    pub halvings: u32,
    choice: (u64, u64),
    /// `files` is what to use as it is: one picture, or two grey halves in order (red / green, blue / alpha)
    decided: bool,
}

/// What was chosen for a texture and palette whose palette hash matched none (by colour, on the workers): one
/// file, or two grey halves in order.
static CHOSEN: Mutex<Option<HashMap<(u64, u64), Vec<PathBuf>>>> = Mutex::new(None);

/// The file replacing a texture (its indices and palette), for a texture quality.
pub fn lookup(quality: i32, idx: &[u8], w: usize, h: usize, bits: u32, pal: &[[u8; 4]]) -> Option<Found> {
    let (pack, halvings) = source(quality)?;
    let tex = texture_hash(idx, w, h, bits);
    let files = packs()[pack].files.get(&tex)?;
    let tlut = palette_hash(idx, pal);
    let choice = (tex, tlut);
    let key = xxh64(&[tex.to_le_bytes(), tlut.to_le_bytes(), (halvings as u64 | (pack as u64) << 8).to_le_bytes()].concat(), 0);
    let decided = if files.len() == 1 {
        Some(vec![files[0].1.clone()])
    } else if let Some(f) = files.iter().find(|f| f.0 == tlut) {
        Some(vec![f.1.clone()])
    } else {
        CHOSEN.lock().unwrap().as_ref().and_then(|m| m.get(&choice).cloned())
    };
    Some(match decided {
        Some(files) => Found { key, files, halvings, choice, decided: true },
        None => Found { key, files: files.iter().map(|f| f.1.clone()).collect(), halvings, choice, decided: false },
    })
}

/// A small summary of a picture's look: the average red, green, blue and alpha (0..1) of 8 x 8 cells.
fn thumb(w: usize, h: usize, px: impl Fn(usize, usize) -> [f32; 4]) -> Vec<f32> {
    let mut out = vec![0f32; 8 * 8 * 4];
    for cy in 0..8 {
        for cx in 0..8 {
            let (x0, x1, y0, y1) = (cx * w / 8, ((cx + 1) * w / 8).max(cx * w / 8 + 1), cy * h / 8, ((cy + 1) * h / 8).max(cy * h / 8 + 1));
            let mut acc = [0f32; 4];
            let step = ((x1 - x0) * (y1 - y0) / 64).max(1);
            let mut n = 0f32;
            let mut k = 0;
            for y in y0..y1 {
                for x in x0..x1 {
                    k += 1;
                    if k % step != 0 {
                        continue;
                    }
                    let p = px(x, y);
                    for c in 0..4 {
                        acc[c] += p[c];
                    }
                    n += 1.0;
                }
            }
            let o = (cy * 8 + cx) * 4;
            for c in 0..4 {
                out[o + c] = acc[c] / n.max(1.0);
            }
        }
    }
    out
}

/// The summary of an original texture (RGBA words with the GS's alpha, 0x80 opaque).
pub fn thumb_of_original(px: &[u32], w: usize, h: usize) -> Vec<f32> {
    thumb(w, h, |x, y| {
        let v = px[y * w + x];
        [(v & 255) as f32 / 255.0, (v >> 8 & 255) as f32 / 255.0, (v >> 16 & 255) as f32 / 255.0, ((v >> 24) as f32 / 128.0).min(1.0)]
    })
}

/// The summary of a pack's picture, its alpha as it is (0..1 of 255).
fn thumb_of_picture(p: &Picture) -> Vec<f32> {
    thumb(p.w as usize, p.h as usize, |x, y| {
        let o = (y * p.w as usize + x) * 4;
        [p.rgba[o] as f32 / 255.0, p.rgba[o + 1] as f32 / 255.0, p.rgba[o + 2] as f32 / 255.0, p.rgba[o + 3] as f32 / 255.0]
    })
}

/// Its alpha's scale to the GS's (0x80 opaque): a picture whose alpha stays at or under 130 has the PS2's alpha
/// already (some files of the packs do); else 255 is opaque.
fn alpha_scale_of(p: &Picture) -> f32 {
    let amax = p.rgba.chunks(4).map(|q| q[3]).max().unwrap_or(255);
    if amax <= 130 {
        1.0
    } else {
        0.5
    }
}

fn dist(a: &[f32], b: &[f32]) -> f32 {
    a.iter().zip(b).map(|(x, y)| (x - y) * (x - y)).sum()
}

// ------------------------------------------------------------------------------------------------ DDS
/// A picture decoded from a DDS file: RGBA, the pack's own alpha (255 opaque).
pub struct Picture {
    pub w: u32,
    pub h: u32,
    pub rgba: Vec<u8>,
    /// what its alpha is multiplied by to give the GS's (0x80 opaque): 0.5 for a Wii picture (255 opaque), 1 for
    /// one put together from two halves (`merge_halves`), whose alpha is the PS2's own
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
    Some(Picture { w, h, rgba, alpha_scale: 0.5 })
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
fn grey(p: &Picture) -> bool {
    let n = (p.w * p.h) as usize;
    let step = (n / 4096).max(1);
    let mut sat = 0u64;
    let mut k = 0u64;
    for i in (0..n).step_by(step) {
        let q = &p.rgba[i * 4..i * 4 + 3];
        sat += (*q.iter().max().unwrap() - *q.iter().min().unwrap()) as u64;
        k += 1;
    }
    sat < 10 * k
}

/// A texture whose palette has see-through colours is often, on the Wii, two lookups of the same indices in two
/// intensity / alpha palettes (the Wii has no palette with 8 bits of each of red, green, blue and alpha): one
/// gives red (intensity) and green (alpha), the other blue and the alpha (the PS2's, 0x80 opaque). A pack has
/// both pictures, grey, under two palette hashes; `choose` tells which is which from the original.
fn merge_halves(rg: &Picture, ba: &Picture) -> Option<Picture> {
    if rg.w != ba.w || rg.h != ba.h {
        return None;
    }
    let mut rgba = vec![0u8; rg.rgba.len()];
    for i in 0..(rg.w * rg.h) as usize {
        rgba[i * 4] = rg.rgba[i * 4];
        rgba[i * 4 + 1] = rg.rgba[i * 4 + 3];
        rgba[i * 4 + 2] = ba.rgba[i * 4];
        rgba[i * 4 + 3] = ba.rgba[i * 4 + 3];
    }
    Some(Picture { w: rg.w, h: rg.h, rgba, alpha_scale: 1.0 })
}

/// The picture for a job: the one file, the two halves in order, or the best match among the texture's files
/// (each picture as it is, and each two grey ones put together), judged by the original's summary.
fn choose(f: &Found, thumb: &[f32]) -> Option<Picture> {
    let pics: Vec<(Picture, &PathBuf)> = f
        .files
        .iter()
        .filter_map(|path| match std::fs::read(path).ok().and_then(|d| dds_decode(&d)) {
            Some(p) => Some((p, path)),
            None => {
                eprintln!("bt3: texture pack: cannot read {}", path.display());
                None
            }
        })
        .collect();
    if f.decided || pics.len() < 2 || thumb.len() != 8 * 8 * 4 {
        if f.decided && pics.len() == 2 {
            return merge_halves(&pics[0].0, &pics[1].0);
        }
        return pics.into_iter().next().map(|(mut p, _)| {
            p.alpha_scale = alpha_scale_of(&p);
            p
        });
    }
    let thumbs: Vec<Vec<f32>> = pics.iter().map(|p| thumb_of_picture(&p.0)).collect();
    let scales: Vec<f32> = pics.iter().map(|p| alpha_scale_of(&p.0)).collect();
    let greys: Vec<bool> = pics.iter().map(|p| grey(&p.0)).collect();
    // (error, first, second): second = Some for two halves
    let mut best: Option<(f32, usize, Option<usize>)> = None;
    let mut consider = |e: f32, i: usize, j: Option<usize>| {
        if best.map_or(true, |b| e < b.0) {
            best = Some((e, i, j));
        }
    };
    for (i, t) in thumbs.iter().enumerate() {
        // alpha in the original's units (1 = opaque)
        let k = scales[i] * 255.0 / 128.0;
        let t: Vec<f32> = t.chunks(4).flat_map(|q| [q[0], q[1], q[2], (q[3] * k).min(1.0)]).collect();
        consider(dist(&t, thumb), i, None);
    }
    for i in 0..pics.len() {
        for j in 0..pics.len() {
            if i == j || !greys[i] || !greys[j] {
                continue;
            }
            let (x, y) = (&thumbs[i], &thumbs[j]);
            let t: Vec<f32> = (0..64).flat_map(|c| [x[c * 4], x[c * 4 + 3], y[c * 4], (y[c * 4 + 3] * 255.0 / 128.0).min(1.0)]).collect();
            consider(dist(&t, thumb), i, Some(j));
        }
    }
    let (e, i, j) = best?;
    if std::env::var("BT3_TEX_LOG").is_ok() {
        eprintln!("bt3: texture pack: chose {:?} {:?} (error {e:.3}) of {} files; thumb0 {:?}", pics[i].1.file_name(), j.map(|j| pics[j].1.file_name()), pics.len(), &thumb[..4]);
    }
    let chosen: Vec<PathBuf> = std::iter::once(pics[i].1.clone()).chain(j.map(|j| pics[j].1.clone())).collect();
    CHOSEN.lock().unwrap().get_or_insert_with(HashMap::new).insert(f.choice, chosen);
    match j {
        Some(j) => merge_halves(&pics[i].0, &pics[j].0),
        None => {
            let mut p = pics.into_iter().nth(i)?.0;
            p.alpha_scale = scales[i];
            Some(p)
        }
    }
}

// ------------------------------------------------------------------------------------------------ loading
/// A replacement asked for (the renderer's key for it) and the result.
pub struct Job {
    pub found: Found,
    /// the original's summary (to choose between files)
    pub thumb: Vec<f32>,
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
                let mut picture = choose(f, &job.thumb);
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
    fn xxh64_vectors() {
        assert_eq!(xxh64(b"", 0), 0xEF46DB3751D8E999);
        assert_eq!(xxh64(b"a", 0), 0xD24EC4F1A98C6E5B);
        assert_eq!(xxh64(b"abc", 0), 0x44BC2CF5AD770999);
        assert_eq!(xxh64(b"Nobody inspects the spammish repetition", 0), 0xFBCEA83C8A378BF1);
    }
}
