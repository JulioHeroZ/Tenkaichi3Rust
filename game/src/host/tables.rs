//! The game's data tables: the bytes of every table run the decompilation leaves to the original executable
//! (gen/data.rs ORIGINAL_RUNS: address and length), read at start-up from `data/tables.txt` of the assets. The
//! installer writes that file from the user's own executable (tools/translate/c2rs/tables_from_elf.py); the program
//! carries none of the game's data, so it can be built and handed out without the disc.
//!
//! tables.txt: `#` comments, then one line per run: its PS2 address and its bytes, in hex.
use std::collections::HashMap;

/// The runs of a tables.txt: (address, bytes) in file order.
pub fn parse(text: &str) -> Result<Vec<(u32, Vec<u8>)>, String> {
    let mut out = Vec::new();
    for (i, line) in text.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let bad = || format!("tables.txt line {}: expected `<address> <hex bytes>`", i + 1);
        let (a, h) = line.split_once(' ').ok_or_else(bad)?;
        let addr = u32::from_str_radix(a, 16).map_err(|_| bad())?;
        let h = h.trim().as_bytes();
        if h.len() % 2 != 0 {
            return Err(bad());
        }
        let hex = |c: u8| (c as char).to_digit(16).map(|v| v as u8);
        let bytes = h.chunks(2).map(|p| Some(hex(p[0])? << 4 | hex(p[1])?)).collect::<Option<Vec<u8>>>().ok_or_else(bad)?;
        out.push((addr, bytes));
    }
    Ok(out)
}

/// The bytes of every run of ORIGINAL_RUNS, in its order, from the runs of a tables.txt.
pub fn run_bytes(runs: &[(u32, Vec<u8>)]) -> Result<Vec<(u32, Vec<u8>)>, String> {
    let mut by: HashMap<(u32, usize), &[u8]> = HashMap::new();
    for (a, b) in runs {
        by.entry((*a, b.len())).or_insert(b);
    }
    crate::gen::data::ORIGINAL_RUNS
        .iter()
        .map(|&(addr, _, _, len)| {
            by.get(&(addr, len as usize)).map(|b| (addr, b.to_vec())).ok_or_else(|| format!("tables.txt has no run 0x{addr:08X} (+{len})"))
        })
        .collect()
}

/// The path of the tables of an assets folder.
pub fn path(assets: &std::path::Path) -> std::path::PathBuf {
    assets.join("data").join("tables.txt")
}

/// Writes every table run into memory; the number of runs.
pub fn load(c: &mut crate::rt::Ctx, assets: &std::path::Path) -> Result<usize, String> {
    let p = path(assets);
    let text = super::vfs::read_to_string(&p).map_err(|e| format!("{}: {e} (the installer writes it from your disc)", p.display()))?;
    let runs = run_bytes(&parse(&text)?)?;
    for (addr, b) in &runs {
        c.m.write_bytes(*addr, b);
    }
    Ok(runs.len())
}

/// Compares the tables with what is in memory now (after the catalog was loaded): the runs that differ.
pub fn compare(c: &crate::rt::Ctx, assets: &std::path::Path) -> usize {
    let runs = match super::vfs::read_to_string(path(assets)).map_err(|e| e.to_string()).and_then(|t| run_bytes(&parse(&t)?)) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("bt3: {e}");
            return usize::MAX;
        }
    };
    let mut bad = 0;
    for (addr, b) in &runs {
        let mut m = vec![0u8; b.len()];
        c.m.read_bytes(*addr, &mut m);
        if m != *b {
            if bad < 5 {
                eprintln!("bt3: table run 0x{addr:08X} (+{}) differs from the catalog", b.len());
            }
            bad += 1;
        }
    }
    bad
}
