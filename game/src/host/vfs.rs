//! The game's open files as the loader reads them: loose in the assets folder (developer install, mods) or inside
//! the packs of the player install (`assets/*.bt3pak`, made by tools/install/pack.py). A loose file always wins
//! over the same path in a pack, so a mod is a file put in its place.
//!
//! A pack: "BT3PAK1\0", u32 entry count, u64 offset of the index; the entries' bytes; the index: per entry u16 path
//! length, the path (UTF-8, `/`, relative to the folder of the pack), u64 offset, u32 stored size, u32 size,
//! u8 method (0 stored, 1 zlib).
use std::collections::{HashMap, HashSet};
use std::io::{Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use std::sync::Mutex;

struct Entry {
    pack: usize,
    offset: u64,
    stored: u32,
    size: u32,
    zlib: bool,
}

#[derive(Default)]
struct Packs {
    /// folders whose packs are loaded: as given and canonical
    roots: Vec<[PathBuf; 2]>,
    files: Vec<Mutex<std::fs::File>>,
    /// root index, path relative to it -> entry
    entries: HashMap<(usize, String), Entry>,
    /// root index, folder relative to it -> its children (name, is a folder)
    dirs: HashMap<(usize, String), Vec<(String, bool)>>,
}

static PACKS: Mutex<Option<Packs>> = Mutex::new(None);

/// Loads the packs of an assets folder (its *.bt3pak); the number of files they hold.
pub fn mount(root: &Path) -> Result<usize, String> {
    let Ok(canon) = root.canonicalize() else { return Ok(0) };
    let mut g = PACKS.lock().unwrap();
    let packs = g.get_or_insert_with(Packs::default);
    if packs.roots.iter().any(|r| r[1] == canon) {
        return Ok(0);
    }
    let mut names: Vec<PathBuf> = match std::fs::read_dir(&canon) {
        Ok(it) => it.flatten().map(|e| e.path()).filter(|p| p.extension().is_some_and(|x| x == "bt3pak")).collect(),
        Err(_) => return Ok(0),
    };
    if names.is_empty() {
        return Ok(0);
    }
    names.sort();
    let r = packs.roots.len();
    packs.roots.push([root.to_path_buf(), canon]);
    let mut n = 0;
    let mut seen_dirs: HashSet<(usize, String, String)> = HashSet::new();
    for name in names {
        let mut f = std::fs::File::open(&name).map_err(|e| format!("{}: {e}", name.display()))?;
        let mut head = [0u8; 20];
        f.read_exact(&mut head).map_err(|e| format!("{}: {e}", name.display()))?;
        if &head[..8] != b"BT3PAK1\0" {
            return Err(format!("{}: not a BT3PAK1 pack", name.display()));
        }
        let count = u32::from_le_bytes(head[8..12].try_into().unwrap()) as usize;
        let at = u64::from_le_bytes(head[12..20].try_into().unwrap());
        let mut index = Vec::new();
        f.seek(SeekFrom::Start(at)).and_then(|_| f.read_to_end(&mut index)).map_err(|e| format!("{}: {e}", name.display()))?;
        let pack = packs.files.len();
        let mut p = 0usize;
        let bad = || format!("{}: damaged index", name.display());
        for _ in 0..count {
            let len = u16::from_le_bytes(index.get(p..p + 2).ok_or_else(bad)?.try_into().unwrap()) as usize;
            let path = std::str::from_utf8(index.get(p + 2..p + 2 + len).ok_or_else(bad)?).map_err(|_| bad())?.to_string();
            p += 2 + len;
            let rest = index.get(p..p + 17).ok_or_else(bad)?;
            p += 17;
            let e = Entry {
                pack,
                offset: u64::from_le_bytes(rest[0..8].try_into().unwrap()),
                stored: u32::from_le_bytes(rest[8..12].try_into().unwrap()),
                size: u32::from_le_bytes(rest[12..16].try_into().unwrap()),
                zlib: rest[16] == 1,
            };
            // the folders on the way, for listings
            let mut child = path.as_str();
            let mut is_dir = false;
            loop {
                let (parent, leaf) = match child.rfind('/') {
                    Some(i) => (&child[..i], &child[i + 1..]),
                    None => ("", child),
                };
                if seen_dirs.insert((r, parent.to_string(), leaf.to_string())) {
                    packs.dirs.entry((r, parent.to_string())).or_default().push((leaf.to_string(), is_dir));
                } else if is_dir {
                    break;
                }
                if parent.is_empty() {
                    break;
                }
                child = parent;
                is_dir = true;
            }
            packs.entries.insert((r, path), e);
            n += 1;
        }
        packs.files.push(Mutex::new(f));
    }
    Ok(n)
}

/// (root index, path relative to it with `/`) of a path inside a folder with packs.
fn locate(packs: &Packs, p: &Path) -> Option<(usize, String)> {
    for (i, r) in packs.roots.iter().enumerate() {
        for base in r {
            if let Ok(rel) = p.strip_prefix(base) {
                // `a/b/../c` -> `a/c` (glTF links between folders)
                let mut parts: Vec<String> = Vec::new();
                for c in rel.components() {
                    match c {
                        std::path::Component::ParentDir => {
                            parts.pop()?;
                        }
                        std::path::Component::Normal(x) => parts.push(x.to_string_lossy().to_string()),
                        _ => {}
                    }
                }
                return Some((i, parts.join("/")));
            }
        }
    }
    None
}

fn from_pack(p: &Path) -> Option<std::io::Result<Vec<u8>>> {
    let g = PACKS.lock().unwrap();
    let packs = g.as_ref()?;
    let key = locate(packs, p)?;
    let e = packs.entries.get(&key)?;
    let mut f = packs.files[e.pack].lock().unwrap();
    let mut buf = vec![0u8; e.stored as usize];
    if let Err(err) = f.seek(SeekFrom::Start(e.offset)).and_then(|_| f.read_exact(&mut buf)) {
        return Some(Err(err));
    }
    if e.zlib {
        buf = match super::assets::inflate_zlib(&buf) {
            Ok(b) => b,
            Err(m) => return Some(Err(std::io::Error::new(std::io::ErrorKind::InvalidData, m))),
        };
    }
    if buf.len() != e.size as usize {
        return Some(Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "pack entry of the wrong size")));
    }
    Some(Ok(buf))
}

/// A file's bytes: loose, else from a pack.
pub fn read(p: impl AsRef<Path>) -> std::io::Result<Vec<u8>> {
    let p = p.as_ref();
    match std::fs::read(p) {
        Ok(b) => Ok(b),
        Err(e) => from_pack(p).unwrap_or(Err(e)),
    }
}

pub fn read_to_string(p: impl AsRef<Path>) -> std::io::Result<String> {
    String::from_utf8(read(p)?).map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
}

fn pack_kind(p: &Path) -> Option<bool> {
    let g = PACKS.lock().unwrap();
    let packs = g.as_ref()?;
    let key = locate(packs, p)?;
    if packs.entries.contains_key(&key) {
        Some(false)
    } else if packs.dirs.contains_key(&key) || key.1.is_empty() {
        Some(true)
    } else {
        None
    }
}

/// is_file / is_dir / exists that see the packs too.
pub trait VPath {
    fn vis_file(&self) -> bool;
    fn vis_dir(&self) -> bool;
    fn vexists(&self) -> bool;
}

impl<T: AsRef<Path> + ?Sized> VPath for T {
    fn vis_file(&self) -> bool {
        let p = self.as_ref();
        p.is_file() || pack_kind(p) == Some(false)
    }
    fn vis_dir(&self) -> bool {
        let p = self.as_ref();
        p.is_dir() || pack_kind(p) == Some(true)
    }
    fn vexists(&self) -> bool {
        let p = self.as_ref();
        p.exists() || pack_kind(p).is_some()
    }
}

/// The entries of a folder (path, name, is a folder), loose and in packs, sorted by name.
pub fn read_dir(p: impl AsRef<Path>) -> std::io::Result<Vec<(PathBuf, String, bool)>> {
    let p = p.as_ref();
    let mut out: Vec<(PathBuf, String, bool)> = Vec::new();
    let mut names = HashSet::new();
    let mut found = false;
    if let Ok(it) = std::fs::read_dir(p) {
        found = true;
        for e in it.flatten() {
            let name = e.file_name().to_string_lossy().to_string();
            let path = e.path();
            let d = path.is_dir();
            names.insert(name.clone());
            out.push((path, name, d));
        }
    }
    {
        let g = PACKS.lock().unwrap();
        if let Some(packs) = g.as_ref() {
            if let Some(key) = locate(packs, p) {
                if let Some(children) = packs.dirs.get(&key) {
                    found = true;
                    for (name, d) in children {
                        if names.insert(name.clone()) {
                            out.push((p.join(name), name.clone(), *d));
                        }
                    }
                }
            }
        }
    }
    if !found {
        return Err(std::io::Error::new(std::io::ErrorKind::NotFound, format!("{}: no such folder", p.display())));
    }
    out.sort_by(|a, b| a.1.cmp(&b.1));
    Ok(out)
}
