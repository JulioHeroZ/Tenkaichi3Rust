//! The game's files: CRI ADXF over the three archives, served from a folder of loose files
//! (<data>/pzs3us1/00042.bin is file 42 of partition 1), as port/src/plat_file.c.
#![allow(non_snake_case)]
use crate::rt::Ctx;
use std::io::{Read, Seek, SeekFrom};

const SECTOR: u32 = 2048;
const STAT_STOP: i32 = 1;
const STAT_READEND: i32 = 3;
// PortFile in the game's memory: rel[128], sizeSct, posSct, stat
const F_SIZE: u32 = 128;
const F_POS: u32 = 132;
const F_STAT: u32 = 136;

fn root(c: &Ctx) -> std::path::PathBuf {
    if !c.host.data_root.as_os_str().is_empty() {
        return c.host.data_root.clone();
    }
    std::path::PathBuf::from(std::env::var("BT3_DATA").unwrap_or_else(|_| "gamedata".into()))
}

fn host_path(c: &Ctx, rel: &str) -> Option<std::path::PathBuf> {
    if std::env::var_os("BT3_PC_ONLY").is_some() {
        return None;
    }
    let r = root(c);
    let m = r.join("mods").join(rel);
    if m.is_file() {
        return Some(m);
    }
    let p = r.join(rel);
    if p.is_file() { Some(p) } else { None }
}

/// A file of the game rebuilt from the open-format extraction (assets/files), unless a mod replaces it.
pub fn rebuilt(c: &Ctx, rel: &str) -> Option<std::sync::Arc<Vec<u8>>> {
    if root(c).join("mods").join(rel).is_file() {
        return None;
    }
    super::assets::rebuilt(&assets_root(c), rel)
}

fn open_rel(c: &mut Ctx, rel: &str) -> u32 {
    let size = if let Some(b) = rebuilt(c, rel) {
        b.len() as u32
    } else {
        let Some(path) = host_path(c, rel) else {
            if std::env::var_os("BT3_PC_ONLY").is_some() {
                panic!("bt3: PC asset missing or rebuild failed: {rel}; raw fallback is disabled");
            }
            eprintln!("bt3: file missing: {rel}");
            return 0;
        };
        std::fs::metadata(&path).map(|m| m.len()).unwrap_or(0) as u32
    };
    let f = super::low_alloc(c, 140);
    let mut name = rel.as_bytes().to_vec();
    name.truncate(127);
    c.m.write_bytes(f, &name);
    c.m.w32(f + F_SIZE, (size + SECTOR - 1) / SECTOR);
    c.m.w32(f + F_STAT, STAT_STOP as u32);
    f
}

/// Where a file of the game is on this computer: file `flid` of partition `ptid`, or the loose file `fname`
/// (a disc path, as ADXF_Open takes it). Port_FilePath of plat_file.c.
pub fn file_path(c: &Ctx, ptid: i32, flid: i32, fname: Option<&[u8]>) -> Option<std::path::PathBuf> {
    host_path(c, &file_rel(c, ptid, flid, fname))
}

/// The open-format assets made from the data (BT3_ASSETS, else the folder two levels above the data root:
/// assets/_legacy/raw -> assets).
pub fn assets_root(c: &Ctx) -> std::path::PathBuf {
    if let Ok(a) = std::env::var("BT3_ASSETS") {
        return a.into();
    }
    root(c).join("..").join("..")
}

/// A file's path under the data root (as file_path finds it).
pub fn file_rel(c: &Ctx, ptid: i32, flid: i32, fname: Option<&[u8]>) -> String {
    match fname {
        Some(name) => {
            let mut rel = String::from("disc/DATA/");
            for &b in name.iter().take_while(|&&b| b != b';') {
                rel.push(if b == b'\\' { '/' } else { (b as char).to_ascii_uppercase() });
            }
            rel
        }
        None => format!("{}/{:05}.bin", c.host.part_dir[(ptid & 7) as usize], flid),
    }
}

pub fn ADXF_LoadPartitionNw(c: &mut Ctx, ptid: i32, fname: u32, _dir: u32, _info: u32) -> i32 {
    let name = c.m.cstr(fname);
    let base = name.rsplit(|&b| b == b'/' || b == b'\\').next().unwrap_or(&name);
    let stem: String = base.iter().take_while(|&&b| b != b'.').map(|&b| (b as char).to_ascii_lowercase()).collect();
    c.host.part_dir[(ptid & 7) as usize] = stem;
    0
}

pub fn ADXF_GetPtStat(c: &mut Ctx, ptid: i32) -> i32 {
    if c.host.part_dir[(ptid & 7) as usize].is_empty() { STAT_STOP } else { STAT_READEND }
}

pub fn ADXF_OpenAfs(c: &mut Ctx, ptid: i32, flid: i32) -> u32 {
    let rel = format!("{}/{:05}.bin", c.host.part_dir[(ptid & 7) as usize], flid);
    open_rel(c, &rel)
}

pub fn ADXF_Open(c: &mut Ctx, fname: u32, _atr: u32) -> u32 {
    let name = c.m.cstr(fname);
    let mut rel = String::from("disc/DATA/");
    for &b in name.iter().take_while(|&&b| b != b';') {
        rel.push(if b == b'\\' { '/' } else { (b as char).to_ascii_uppercase() });
    }
    open_rel(c, &rel)
}

pub fn ADXF_Close(c: &mut Ctx, h: u32) {
    if h != 0 {
        super::low_free(c, h);
    }
}

pub fn ADXF_GetFsizeSct(c: &mut Ctx, h: u32) -> i32 { c.m.r32(h + F_SIZE) as i32 }
pub fn ADXF_Stop(c: &mut Ctx, h: u32) -> i32 { c.m.w32(h + F_STAT, STAT_STOP as u32); 0 }
pub fn ADXF_GetStat(c: &mut Ctx, h: u32) -> i32 { c.m.r32(h + F_STAT) as i32 }
pub fn ADXF_Tell(c: &mut Ctx, h: u32) -> i32 { c.m.r32(h + F_POS) as i32 }

pub fn ADXF_ReadNw(c: &mut Ctx, h: u32, nsct: i32, buf: u32) -> i32 {
    let size = c.m.r32(h + F_SIZE) as i32;
    let pos = c.m.r32(h + F_POS) as i32;
    let n = nsct.min(size - pos).max(0);
    let rel = String::from_utf8_lossy(&c.m.cstr(h)).to_string();
    let mut data = vec![0u8; (n as u32 * SECTOR) as usize];
    if let Some(b) = rebuilt(c, &rel) {
        let start = (pos as usize * SECTOR as usize).min(b.len());
        let end = (start + data.len()).min(b.len());
        data[..end - start].copy_from_slice(&b[start..end]);
    } else if let Some(p) = host_path(c, &rel) {
        if let Ok(mut f) = std::fs::File::open(p) {
            let _ = f.seek(SeekFrom::Start(pos as u64 * SECTOR as u64));
            let mut got = 0;
            while got < data.len() {
                match f.read(&mut data[got..]) {
                    Ok(0) | Err(_) => break,
                    Ok(k) => got += k,
                }
            }
        }
    } else if std::env::var_os("BT3_PC_ONLY").is_some() {
        panic!("bt3: PC asset disappeared during read: {rel}");
    }
    c.m.write_bytes(buf, &data);
    c.m.w32(h + F_POS, (pos + n) as u32);
    c.m.w32(h + F_STAT, STAT_READEND as u32);
    n
}

/// The game's original data tables, from the PC catalog made once from the disc (assets/data/runtime/source-tables.bt3).
pub fn load_original_tables(c: &mut Ctx, catalog: &[u8]) -> Result<usize, String> {
    if catalog.get(..8) != Some(b"BT3RSRC1") {
        return Err("not a BT3RSRC1 catalog".into());
    }
    let w = |o: usize| u32::from_le_bytes(catalog[o..o + 4].try_into().unwrap());
    let count = w(8) as usize;
    let mut known: std::collections::HashMap<(u32, u32), (usize, u32)> = Default::default();
    let mut ranges = Vec::new();
    let mut at = 20;
    for _ in 0..count {
        let (src, off, len) = (w(at), w(at + 4), w(at + 8));
        at += 12;
        ranges.push((src, off, len, at));
        known.insert((src, off), (at, len));
        at += len as usize;
    }
    let mut done = 0;
    for &(addr, src, off, len) in crate::gen::data::ORIGINAL_RUNS {
        // find a catalog range holding [off, off + len)
        let r = ranges.iter().find(|&&(s, o, l, _)| s == src && o <= off && off + len <= o + l);
        match r {
            Some(&(_, o, _, data)) => {
                let start = data + (off - o) as usize;
                let bytes = catalog[start..start + len as usize].to_vec();
                c.m.write_bytes(addr, &bytes);
                done += 1;
            }
            None => return Err(format!("catalog has no bytes for source {src} offset 0x{off:X} (+{len})")),
        }
    }
    Ok(done)
}
