//! What the installer's programs share: the payload of the one-file installer (Instalar Tenkaichi 3.exe carries the
//! Python, the extraction tools and the game's program after its own bytes, written by tools/install/empacotar.py)
//! and the zlib inflate it needs.
//!
//! The payload, after the executable: the entries' bytes (zlib), then the index (per entry: u16 path length, the path
//! with `/`, u64 offset from the payload start, u32 stored size, u32 size, u8 method: 0 stored, 1 zlib), then the
//! trailer: u64 payload start, u64 index offset from the payload start, u32 entry count, "BT3SFX1\0".
use std::path::Path;

#[path = "../../../game/src/host/mpeg2.rs"]
pub mod mpeg2;
pub mod icon;

pub struct Entry {
    pub path: String,
    pub offset: u64,
    pub stored: u32,
    pub size: u32,
    pub zlib: bool,
}

/// The entries of the payload of an executable's bytes, and where it starts; None when it has none.
pub fn payload(exe: &[u8]) -> Option<(usize, Vec<Entry>)> {
    let n = exe.len();
    if n < 28 || &exe[n - 8..] != b"BT3SFX1\0" {
        return None;
    }
    let start = u64::from_le_bytes(exe[n - 28..n - 20].try_into().ok()?) as usize;
    let index = start + u64::from_le_bytes(exe[n - 20..n - 12].try_into().ok()?) as usize;
    let count = u32::from_le_bytes(exe[n - 12..n - 8].try_into().ok()?) as usize;
    let mut p = index;
    let mut out = Vec::with_capacity(count);
    for _ in 0..count {
        let len = u16::from_le_bytes(exe.get(p..p + 2)?.try_into().ok()?) as usize;
        let path = std::str::from_utf8(exe.get(p + 2..p + 2 + len)?).ok()?.to_string();
        p += 2 + len;
        let r = exe.get(p..p + 17)?;
        p += 17;
        out.push(Entry {
            path,
            offset: u64::from_le_bytes(r[0..8].try_into().ok()?),
            stored: u32::from_le_bytes(r[8..12].try_into().ok()?),
            size: u32::from_le_bytes(r[12..16].try_into().ok()?),
            zlib: r[16] == 1,
        });
    }
    Some((start, out))
}

/// Writes the payload's files under `dir`; `step` is told each file written (done, total).
pub fn extract(exe: &[u8], dir: &Path, step: &mut dyn FnMut(usize, usize)) -> Result<usize, String> {
    let (start, entries) = payload(exe).ok_or("este instalador não traz os arquivos de instalação")?;
    for (i, e) in entries.iter().enumerate() {
        if e.path.split('/').any(|c| c == ".." || c.is_empty()) {
            return Err(format!("caminho inválido no instalador: {}", e.path));
        }
        let at = start + e.offset as usize;
        let raw = exe.get(at..at + e.stored as usize).ok_or("instalador danificado")?;
        let data = if e.zlib { inflate_zlib(raw)? } else { raw.to_vec() };
        if data.len() != e.size as usize {
            return Err(format!("instalador danificado: {}", e.path));
        }
        let target = dir.join(&e.path);
        if let Some(parent) = target.parent() {
            std::fs::create_dir_all(parent).map_err(|x| format!("{}: {x}", parent.display()))?;
        }
        std::fs::write(&target, &data).map_err(|x| format!("{}: {x}", target.display()))?;
        step(i + 1, entries.len());
    }
    Ok(entries.len())
}

// ---------------------------------------------------------------------------------------------- inflate
struct Bits<'a> {
    d: &'a [u8],
    pos: usize,
    bit: u32,
    nbits: u32,
}

impl<'a> Bits<'a> {
    fn need(&mut self, n: u32) -> Result<(), String> {
        while self.nbits < n {
            let b = *self.d.get(self.pos).ok_or("deflate data ends early")? as u32;
            self.pos += 1;
            self.bit |= b << self.nbits;
            self.nbits += 8;
        }
        Ok(())
    }
    fn take(&mut self, n: u32) -> Result<u32, String> {
        if n == 0 {
            return Ok(0);
        }
        self.need(n)?;
        let v = self.bit & ((1u32 << n) - 1);
        self.bit >>= n;
        self.nbits -= n;
        Ok(v)
    }
}

/// Canonical Huffman table: (counts per length, symbols in order).
struct Huff {
    count: [u16; 16],
    sym: Vec<u16>,
}

impl Huff {
    fn new(lengths: &[u8]) -> Huff {
        let mut count = [0u16; 16];
        for &l in lengths {
            count[l as usize] += 1;
        }
        count[0] = 0;
        let mut offs = [0u16; 16];
        for i in 1..16 {
            offs[i] = offs[i - 1] + count[i - 1];
        }
        let mut sym = vec![0u16; lengths.len()];
        for (s, &l) in lengths.iter().enumerate() {
            if l != 0 {
                sym[offs[l as usize] as usize] = s as u16;
                offs[l as usize] += 1;
            }
        }
        Huff { count, sym }
    }
    fn decode(&self, b: &mut Bits) -> Result<u16, String> {
        let (mut code, mut first, mut index) = (0i32, 0i32, 0i32);
        for len in 1..16 {
            code |= b.take(1)? as i32;
            let count = self.count[len] as i32;
            if code - count < first {
                return Ok(self.sym[(index + (code - first)) as usize]);
            }
            index += count;
            first += count;
            first <<= 1;
            code <<= 1;
        }
        Err("bad Huffman code".into())
    }
}

const LBASE: [u16; 29] = [3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 15, 17, 19, 23, 27, 31, 35, 43, 51, 59, 67, 83, 99, 115, 131, 163, 195, 227, 258];
const LEXT: [u8; 29] = [0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 0];
const DBASE: [u16; 30] = [1, 2, 3, 4, 5, 7, 9, 13, 17, 25, 33, 49, 65, 97, 129, 193, 257, 385, 513, 769, 1025, 1537, 2049, 3073, 4097, 6145, 8193, 12289, 16385, 24577];
const DEXT: [u8; 30] = [0, 0, 0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 11, 11, 12, 12, 13, 13];

/// zlib (RFC 1950) around deflate (RFC 1951).
pub fn inflate_zlib(d: &[u8]) -> Result<Vec<u8>, String> {
    if d.len() < 2 || (d[0] & 15) != 8 {
        return Err("not zlib data".into());
    }
    let mut b = Bits { d: &d[2..], pos: 0, bit: 0, nbits: 0 };
    let mut out = Vec::new();
    loop {
        let last = b.take(1)?;
        let kind = b.take(2)?;
        match kind {
            0 => {
                b.bit = 0;
                b.nbits = 0;
                let p = b.pos;
                let len = u16::from_le_bytes([*b.d.get(p).ok_or("eof")?, *b.d.get(p + 1).ok_or("eof")?]) as usize;
                let start = p + 4;
                out.extend_from_slice(b.d.get(start..start + len).ok_or("stored block past the end")?);
                b.pos = start + len;
            }
            1 | 2 => {
                let (lit, dist) = if kind == 1 {
                    let mut l = [0u8; 288];
                    for (i, x) in l.iter_mut().enumerate() {
                        *x = if i < 144 { 8 } else if i < 256 { 9 } else if i < 280 { 7 } else { 8 };
                    }
                    (Huff::new(&l), Huff::new(&[5u8; 30]))
                } else {
                    let hlit = b.take(5)? as usize + 257;
                    let hdist = b.take(5)? as usize + 1;
                    let hclen = b.take(4)? as usize + 4;
                    const ORDER: [usize; 19] = [16, 17, 18, 0, 8, 7, 9, 6, 10, 5, 11, 4, 12, 3, 13, 2, 14, 1, 15];
                    let mut cl = [0u8; 19];
                    for &i in ORDER.iter().take(hclen) {
                        cl[i] = b.take(3)? as u8;
                    }
                    let ch = Huff::new(&cl);
                    let mut lens = vec![0u8; hlit + hdist];
                    let mut i = 0;
                    while i < hlit + hdist {
                        let s = ch.decode(&mut b)?;
                        match s {
                            0..=15 => {
                                lens[i] = s as u8;
                                i += 1;
                            }
                            16 => {
                                let prev = *lens.get(i.wrapping_sub(1)).ok_or("repeat with nothing before")?;
                                for _ in 0..3 + b.take(2)? {
                                    *lens.get_mut(i).ok_or("too many lengths")? = prev;
                                    i += 1;
                                }
                            }
                            17 => i += 3 + b.take(3)? as usize,
                            _ => i += 11 + b.take(7)? as usize,
                        }
                    }
                    if i > hlit + hdist {
                        return Err("too many lengths".into());
                    }
                    (Huff::new(&lens[..hlit]), Huff::new(&lens[hlit..]))
                };
                loop {
                    let s = lit.decode(&mut b)?;
                    if s < 256 {
                        out.push(s as u8);
                    } else if s == 256 {
                        break;
                    } else {
                        let k = (s - 257) as usize;
                        let len = *LBASE.get(k).ok_or("bad length")? as usize + b.take(*LEXT.get(k).unwrap_or(&0) as u32)? as usize;
                        let ds = dist.decode(&mut b)? as usize;
                        let dd = *DBASE.get(ds).ok_or("bad distance")? as usize + b.take(*DEXT.get(ds).unwrap_or(&0) as u32)? as usize;
                        if dd > out.len() {
                            return Err("distance too far back".into());
                        }
                        let start = out.len() - dd;
                        for j in 0..len {
                            let v = out[start + j];
                            out.push(v);
                        }
                    }
                }
            }
            _ => return Err("bad block type".into()),
        }
        if last == 1 {
            break;
        }
    }
    Ok(out)
}
