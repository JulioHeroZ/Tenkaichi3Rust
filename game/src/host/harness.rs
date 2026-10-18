//! Start-up and the headless harness of the C port (port/src/headless.c): the game's own main loop, with the
//! menu overlay's Progress_Main replaced by an attract-demo or replay battle set-up when BT3_DEMO / BT3_REPLAY is
//! given, and the per-frame trace of the fight (BT3_TRACE=<n>).
#![allow(non_snake_case)]
use crate::gen::funcs as F;
use crate::gen::sym;
use crate::rt::Ctx;

pub fn wrap_Progress_Main(c: &mut Ctx, arg: i32) -> i32 {
    // Optional original-movie viewer after the game's normal subsystem setup.
    // It does not load/save a profile or change the selected story/mode.
    if let Ok(movie) = std::env::var("BT3_MOVIE_PREVIEW") {
        c.host.menu_mode = true;
        F::File_LoadPartitionNw(c, 2);
        match movie.as_str() {
            "opening" => F::Movie_PlayOpening(c),
            "ending" => F::Movie_PlayEnding(c),
            _ => { eprintln!("bt3: BT3_MOVIE_PREVIEW must be opening or ending"); std::process::exit(2); }
        }
        println!("bt3: original movie preview completed: {movie}");
        std::process::exit(0);
    }
    let replay = std::env::var("BT3_REPLAY").ok();
    if replay.is_none() && std::env::var("BT3_DEMO").is_err() {
        c.host.menu_mode = true;
        let r = F::__real_Progress_Main(c, arg);
        c.host.menu_mode = false;
        c.host.from_menu = true;
        return r;
    }
    let n = c.host.battles;
    c.host.battles += 1;
    if n > 0 {
        println!("bt3: battle finished");
        std::process::exit(0);
    }
    // the third archive (streams: music, voices) is opened by the boot menu's FirstRun_Main, which a demo or a
    // replay skips: open it here, as the boot does (only the file layer sees this)
    F::File_LoadPartitionNw(c, 2);
    if let Some(path) = replay {
        const SIZE: usize = 0x1AC00;
        let data = std::fs::read(&path).unwrap_or_default();
        if data.len() != SIZE {
            eprintln!("bt3: cannot read replay {path} (0x{SIZE:X} bytes expected)");
            std::process::exit(2);
        }
        let buf = super::low_alloc(c, SIZE as u32);
        c.m.write_bytes(buf, &data);
        F::Battle_ClearWork(c);
        if F::BattleReplay_Load(c, buf + 0x38, 0x1ABA8) == 0 {
            eprintln!("bt3: {path} is not a replay this version accepts");
            std::process::exit(2);
        }
        println!("bt3: replay {path}");
    } else {
        F::Demo_SetupBattle(c);
        println!("bt3: attract-demo battle");
    }
    0
}

/// BT3_AT=<tick>,...: when the battle clock first shows one of these ticks, write the game heap
/// (PS2 0x3BE730..0x1EFB014) to heap_<tick>.bin, as the C port does (for comparing memory).
fn at_dump(c: &mut Ctx) {
    let Ok(list) = std::env::var("BT3_AT") else { return };
    let seq = c.m.r32(sym::gBtlSeq);
    if (c.host.battles == 0 && !c.host.from_menu) || seq == 0 || (c.m.r32(seq) as i32) < 2 {
        return;
    }
    let clock = F::BtlSeq_GetClock(c);
    let now = c.m.r32(clock);
    if now.wrapping_add(1) == c.host.at_last {
        return;
    }
    c.host.at_last = now.wrapping_add(1);
    for t in list.split(',') {
        if t.trim().parse::<u32>().ok() == Some(now) {
            let mut buf = vec![0u8; (0x1EFB014 - 0x3BE730) as usize];
            c.m.read_bytes(0x3BE730, &mut buf);
            let _ = std::fs::write(format!("heap_{now}.bin"), &buf);
            println!("bt3: tick {now} (vblank {}): heap_{now}.bin", c.host.vblanks);
        }
    }
}

fn debug_vb(c: &mut Ctx) {
    let Some(n) = std::env::var("BT3_DEBUG_VB").ok().and_then(|v| v.parse::<u32>().ok()) else { return };
    if c.host.vblanks != n && n != 0 {
        return;
    }
    let fx = |b: u32| f32::from_bits(b);
    for id in 0..2 {
        let chr = F::BtlChar_FindByObjId(c, id);
        if chr == 0 {
            continue;
        }
        let pos = F::BtlChar_GetPos(c, chr);
        let v: Vec<f32> = (0..8).map(|k| fx(c.m.r32(pos + 4 * k))).collect();
        println!("dbg vb {} obj {id} chr {chr:08x} pos@{pos:08x} {:?} ram_host={:#x}", c.host.vblanks, v, c.m.ram_ptr());
    }
}

pub fn trace(c: &mut Ctx) {
    debug_vb(c);
    at_dump(c);
    if c.host.trace_every == 0 {
        c.host.trace_every = std::env::var("BT3_TRACE").ok().and_then(|v| v.parse().ok()).unwrap_or(-1);
    }
    let every = c.host.trace_every;
    let vb = c.host.vblanks;
    let seq = c.m.r32(sym::gBtlSeq);
    if every <= 0 || vb % every as u32 != 0 || c.host.battles == 0 || seq == 0 {
        return;
    }
    let state = c.m.r32(seq) as i32;
    if state < 2 {
        return;
    }
    let clock = F::BtlSeq_GetClock(c);
    let cl: Vec<u32> = (0..4).map(|i| c.m.r32(clock + 4 * i)).collect();
    let mut p = [[0u32; 3]; 2];
    let tmp = c.sp.wrapping_sub(64) & !15;
    for i in 0..2 {
        F::BtlCharApi_GetPos(c, i as i32, tmp);
        for k in 0..3 {
            p[i][k] = c.m.r32(tmp + 4 * k as u32);
        }
    }
    let hp0 = F::BtlCharApi_GetHp(c, 0);
    let hp1 = F::BtlCharApi_GetHp(c, 1);
    println!(
        "vb {:7} seq {} clock {:08x} {:08x} {:08x} {:08x} | hp {:6} {:6} | p0 {:08x} {:08x} {:08x} | p1 {:08x} {:08x} {:08x}",
        vb, state, cl[0], cl[1], cl[2], cl[3], hp0, hp1, p[0][0], p[0][1], p[0][2], p[1][0], p[1][1], p[1][2]
    );
}

/// Finds a file next to the program or in the project's asset folders.
fn find(rel: &str) -> Option<std::path::PathBuf> {
    let mut bases: Vec<std::path::PathBuf> = vec![std::env::current_dir().unwrap_or_default()];
    if let Ok(exe) = std::env::current_exe() {
        let mut d = exe.parent().map(|p| p.to_path_buf());
        while let Some(p) = d {
            bases.push(p.clone());
            d = p.parent().map(|q| q.to_path_buf());
        }
    }
    bases.into_iter().map(|b| b.join(rel)).find(|p| p.exists())
}

pub fn run_main() -> i32 {
    // Opened by itself (a double click on the installed game): the assets folder next to the program (or above it,
    // or in the current folder), and only the PC resources, as jogar.cmd sets them.
    if std::env::var_os("BT3_ASSETS").is_none() && std::env::var_os("BT3_DATA").is_none() {
        if let Some(a) = find("assets").filter(|a| a.join("files").is_dir() || a.join("files.bt3pak").is_file()) {
            std::env::set_var("BT3_ASSETS", &a);
            if std::env::var_os("BT3_VERIFY_FILES").is_none() && std::env::var_os("BT3_ORIGINAL_FILES").is_none() {
                std::env::set_var("BT3_PC_ONLY", "1");
            }
        }
    }
    if std::env::var_os("BT3_PC_ONLY").is_some() {
        if std::env::var_os("BT3_ASSETS").is_none() {
            eprintln!("bt3: BT3_PC_ONLY requires BT3_ASSETS=<PC assets folder>");
            return 2;
        }
        for name in ["BT3_ORIGINAL_FILES","BT3_ORIGINAL_AUDIO","BT3_TABLES"] {
            if std::env::var_os(name).is_some() {
                eprintln!("bt3: {name} is incompatible with BT3_PC_ONLY; remove the diagnostic override");
                return 2;
            }
        }
        eprintln!("bt3: PC-only resources: original file access disabled");
    }
    if std::env::var("BT3_PROBE").is_ok() {
        crate::rt::PROBING.store(true, std::sync::atomic::Ordering::Relaxed);
    }
    if let Some(w) = std::env::var("BT3_WATCH").ok().and_then(|v| u32::from_str_radix(v.trim_start_matches("0x"), 16).ok()) {
        crate::rt::mem::WATCH.store(w, std::sync::atomic::Ordering::Relaxed);
    }
    let mut c = Ctx::new();
    if let Ok(d) = std::env::var("BT3_UI_PREVIEW") {
        super::saves::preview(std::path::Path::new(&d));
        return 0;
    }
    if std::env::var("BT3_VERIFY_FILES").is_ok() {
        let raw = std::path::PathBuf::from(std::env::var("BT3_DATA").unwrap_or_else(|_| "gamedata".into()));
        let assets = std::env::var("BT3_ASSETS").map(std::path::PathBuf::from).unwrap_or_else(|_| raw.join("..").join(".."));
        if let Err(e) = super::vfs::mount(&assets) {
            eprintln!("bt3: {e}");
            return 2;
        }
        return if super::assets::verify(&assets, &raw) == 0 { 0 } else { 1 };
    }
    if std::env::var_os("BT3_VERIFY_PC_TABLES").is_some() {
        let assets=super::files::assets_root(&c);
        let raw=std::path::PathBuf::from(std::env::var("BT3_DATA").unwrap_or_else(|_|"gamedata".into()));
        return if super::assets::verify_pc_tables(&assets,&raw)==0 {0}else{1};
    }
    // the game's data tables: data/tables.txt of the assets (host/tables.rs). BT3_TABLES=<catalog> loads the PC
    // catalog instead (the extraction's own file) and checks tables.txt against it.
    let assets_dir = super::files::assets_root(&c);
    // the player install keeps the open files in packs (assets/*.bt3pak)
    match super::vfs::mount(&assets_dir) {
        Ok(0) => {}
        Ok(n) => eprintln!("bt3: {n} files in the packs of {}", assets_dir.display()),
        Err(e) => {
            eprintln!("bt3: {e}");
            return 2;
        }
    }
    match std::env::var("BT3_TABLES").ok().map(std::fs::read) {
        Some(Ok(bytes)) => match super::files::load_original_tables(&mut c, &bytes) {
            Ok(n) => {
                let bad = super::tables::compare(&c, &assets_dir);
                eprintln!("bt3: {n} original data tables loaded from the PC catalog; tables.txt differs in {bad}");
            }
            Err(e) => {
                eprintln!("bt3: {e}");
                return 2;
            }
        },
        Some(Err(e)) => {
            eprintln!("bt3: BT3_TABLES: {e}");
            return 2;
        }
        None => match super::tables::load(&mut c, &assets_dir) {
            Ok(n) => eprintln!("bt3: {n} data tables from {}", super::tables::path(&assets_dir).display()),
            Err(e) => {
                eprintln!("bt3: {e}");
                return 2;
            }
        },
    }
    if let Ok(d) = std::env::var("BT3_DATA") {
        c.host.data_root = d.into();
    } else if std::env::var_os("BT3_PC_ONLY").is_none() {
        if let Some(d) = find("assets/_legacy/raw") {
        c.host.data_root = d;
        }
    }
    F::main(&mut c)
}

/// Differential-replay probe: BT3_PROBE=<name>:<n>[,<name>:<n>...] writes the whole RAM and the arguments
/// before and after the n-th call (1-based) of each probed function to probe_<name>_<n>_{pre,post}.bin and
/// probe_<name>_<n>.args (the caller's frame stack pointer, then the arguments); .counts / .counts_post hold
/// how many times every function had been entered at those two moments and .children the calls it made.
/// The run ends after the last dump unless BT3_PROBE_KEEP is set.
pub fn probe(c: &mut Ctx, name: &'static str, cnt: &'static crate::rt::ProbeCount, args: &[u32], post: bool) {
    use std::cell::RefCell;
    use std::collections::{HashMap, HashSet};
    use std::sync::atomic::Ordering::Relaxed;
    #[derive(Default)]
    struct St {
        want: HashMap<String, HashSet<u32>>,
        left: usize,
        seen: Vec<(&'static str, &'static crate::rt::ProbeCount)>,
        open: Vec<(&'static str, u32, u32, usize, Vec<(&'static str, u32)>)>, // name, depth, n, stack depth, children
        stack: usize,
    }
    thread_local!(static ST: RefCell<Option<St>> = RefCell::new(None));
    let n;
    let d;
    if !post {
        n = cnt.n.fetch_add(1, Relaxed) + 1;
        d = cnt.depth.fetch_add(1, Relaxed) + 1;
    } else {
        n = 0;
        d = cnt.depth.fetch_sub(1, Relaxed);
    }
    ST.with(|st| {
        let mut st = st.borrow_mut();
        if st.is_none() {
            let mut s = St::default();
            for spec in std::env::var("BT3_PROBE").unwrap_or_default().split(',') {
                let (nm, k) = spec.split_once(':').unwrap_or((spec, "1"));
                s.want.entry(nm.to_string()).or_default().insert(k.parse().unwrap_or(1));
            }
            s.left = s.want.values().map(|v| v.len()).sum();
            *st = Some(s);
        }
        let s = st.as_mut().unwrap();
        let counts_text = |s: &St| s.seen.iter().map(|(k, v)| format!("{k} {}\n", v.n.load(Relaxed))).collect::<String>();
        let dump = |c: &mut Ctx, path: String| {
            let mut ram = vec![0u8; crate::rt::mem::RAM as usize];
            c.m.read_bytes(0, &mut ram);
            std::fs::write(path, &ram).ok();
        };
        if !post {
            if n == 1 {
                s.seen.push((name, cnt));
            }
            s.stack += 1;
            let sd = s.stack;
            for o in s.open.iter_mut() {
                if o.3 + 1 == sd {
                    o.4.push((name, n));
                }
            }
            if s.want.get(name).map_or(false, |v| v.contains(&n)) {
                s.open.push((name, d, n, sd, Vec::new()));
                dump(c, format!("probe_{name}_{n}_pre.bin"));
                let a: Vec<String> = args.iter().map(|a| format!("{a:#x}")).collect();
                std::fs::write(format!("probe_{name}_{n}.args"), format!("{:#x} {}\n", c.sp, a.join(" "))).ok();
                std::fs::write(format!("probe_{name}_{n}.counts"), counts_text(s)).ok();
            }
        } else {
            s.stack = s.stack.saturating_sub(1);
            if s.open.is_empty() {
                return;
            }
            if let Some(i) = s.open.iter().position(|o| o.0 == name && o.1 == d) {
                let (_, _, n, _, kids) = s.open.remove(i);
                let k: String = kids.iter().map(|(a, b)| format!("{a} {b}\n")).collect();
                std::fs::write(format!("probe_{name}_{n}.children"), k).ok();
                dump(c, format!("probe_{name}_{n}_post.bin"));
                std::fs::write(format!("probe_{name}_{n}.counts_post"), counts_text(s)).ok();
                eprintln!("bt3: probe {name} #{n} dumped");
                s.left -= 1;
                if s.left == 0 && std::env::var("BT3_PROBE_KEEP").is_err() {
                    std::process::exit(0);
                }
            }
        }
    });
}
