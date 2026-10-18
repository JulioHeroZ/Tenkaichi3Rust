//! The PS2 system software and PC services the game calls (the counterpart of the C port's port/src): memory,
//! files, the vertical blank, pads, the second processor's services, sound players, the GS kick-off calls.
//! Headless for now: no picture, no sound, the pads idle, as the C port's headless build.
#![allow(non_snake_case, unused_variables)]
pub mod libc;
pub mod files;
pub mod assets;
pub mod resource_transport;
pub mod harness;
pub mod native;
pub mod snd;
pub mod tables;
pub mod vfs;
pub mod gs;
pub mod ui;
pub mod saves;
pub mod title;
pub mod options;
pub mod texpack;
pub mod xxh3;
pub mod settings;
pub mod movie;
pub mod mpeg2;

pub use libc::*;
pub use files::*;
pub use harness::{wrap_Progress_Main, run_main};
use crate::rt::{Ctx, F32};
use crate::gen::funcs as F;

pub const HEAP_FIRST: u32 = 0x003B_E730;
pub const HEAP_END: u32 = 0x0200_0000;
pub const LOW_BASE: u32 = 0x1300_0000;
pub const LOW_SIZE: u32 = 0x0C00_0000;

#[derive(Default)]
pub struct HostState {
    pub movie: Option<movie::Decoder>,
    pub heap_next: u32,
    pub low_next: u32,
    pub low_free: Vec<(u32, Vec<u32>)>,
    pub vsync_handler: u32,
    pub vblanks: u32,
    /// Sound (with the window): the device, whether it was looked for, and the ADX players' handles in order.
    pub snd: Option<Box<snd::Sound>>,
    pub snd_checked: bool,
    pub adx_handles: Vec<u32>,
    /// Real-time pacing (a window, or BT3_PACED): whether, and the next blank's moment.
    pub paced: Option<bool>,
    pub pace_next: Option<std::time::Instant>,
    pub pad_sockets: i32,
    pub part_dir: [String; 8],
    pub data_root: std::path::PathBuf,
    pub battles: u32,
    pub from_menu: bool,
    pub trace_every: i32,
    pub demo: bool,
    pub menu_mode: bool,
    pub exit_at: u32,
    pub at_last: u32,
    /// The GS renderer (BT3_GS=1), created on the first display list.
    pub gs: Option<Box<gs::Gs>>,
    pub gs_checked: bool,
}

// ------------------------------------------------------------------ memory
pub fn Port_Malloc(c: &mut Ctx, size: u32) -> u32 {
    if c.host.heap_next == 0 {
        c.host.heap_next = HEAP_FIRST;
    }
    let p = (c.host.heap_next + 15) & !15;
    if size > HEAP_END - p {
        eprintln!("bt3: malloc(0x{size:X}) does not fit in the game heap");
        std::process::exit(2);
    }
    c.host.heap_next = p + size;
    p
}

/// Port_LowAlloc: blocks handed to the game by address (file handles), zeroed, reused by size.
pub fn low_alloc(c: &mut Ctx, size: u32) -> u32 {
    if c.host.low_next == 0 {
        c.host.low_next = LOW_BASE;
    }
    let mut total = (size + 16 + 255) & !255;
    if total > 4096 {
        total = (total + 4095) & !4095;
    }
    for (sz, list) in c.host.low_free.iter_mut() {
        if *sz == total {
            if let Some(p) = list.pop() {
                c.m.fill(p, 0, total);
                c.m.w64(p, total as u64);
                return p + 16;
            }
        }
    }
    let p = c.host.low_next;
    if p + total > LOW_BASE + LOW_SIZE {
        eprintln!("bt3: no memory for {size} bytes in the port's region");
        std::process::exit(2);
    }
    c.host.low_next += total;
    c.m.fill(p, 0, total);
    c.m.w64(p, total as u64);
    p + 16
}

pub fn low_free(c: &mut Ctx, addr: u32) {
    if addr == 0 {
        return;
    }
    let p = addr - 16;
    let total = c.m.r64(p) as u32;
    if let Some((_, list)) = c.host.low_free.iter_mut().find(|(s, _)| *s == total) {
        list.push(p);
    } else {
        c.host.low_free.push((total, vec![p]));
    }
}

pub fn low_string(c: &mut Ctx, s: &[u8]) -> u32 {
    let p = low_alloc(c, s.len() as u32 + 1);
    c.m.write_bytes(p, s);
    p
}

pub fn Port_GameBigAlloc(c: &mut Ctx, size: i32) -> u32 { low_alloc(c, size as u32) }
pub fn Port_GameBigFree(c: &mut Ctx, addr: u32) -> i32 {
    if addr >= LOW_BASE && addr - LOW_BASE < LOW_SIZE {
        low_free(c, addr);
        return 1;
    }
    0
}

// ------------------------------------------------------------------ vector unit primitives (REF_VU0_EXTERN_ARITH)
pub fn RefVu0_AddBits(_c: &mut Ctx, a: u32, b: u32) -> u32 { crate::rt::float::vu_add(a, b) }
pub fn RefVu0_MulBits(_c: &mut Ctx, a: u32, b: u32) -> u32 { crate::rt::float::vu_mul(a, b) }
pub fn RefVu0_DivBits(_c: &mut Ctx, a: u32, b: u32) -> u32 { crate::rt::float::vu_div(a, b) }
/// The SSE shortcut of plat_fastvec.c: 0 = not taken, the caller does the exact long way.
pub fn Port_FastMtxApply(_c: &mut Ctx, _a: u32, _b: u32, _d: u32) -> i32 { 0 }

// ------------------------------------------------------------------ the vertical blank
pub fn sceGsSyncVCallback(c: &mut Ctx, handler: u32) -> u32 {
    let old = c.host.vsync_handler;
    c.host.vsync_handler = handler;
    old
}

/// Real-time pacing at the vertical blank, as the C port's vblank_wait (plat_stub.c): 59.94 per second, on a fixed
/// grid. The game waits for one blank per menu frame and two per fight frame. A blank already passed (but by less
/// than a tenth of a second) is not waited for and the grid is kept, so a fight frame whose work takes between one
/// and two periods still lasts two; further behind, a new grid starts from now.
fn vblank_wait(c: &mut Ctx) {
    const PERIOD: std::time::Duration = std::time::Duration::from_nanos(16_683_350);
    const LATE: std::time::Duration = std::time::Duration::from_millis(100);
    let t = std::time::Instant::now();
    match c.host.pace_next {
        Some(next) if t >= next && t - next < LATE => c.host.pace_next = Some(next + PERIOD),
        Some(next) if next > t && next - t < LATE => {
            // sleep to within a millisecond and a half, then give the rest away in small slices
            let margin = std::time::Duration::from_micros(1500);
            if next - t > margin {
                std::thread::sleep(next - t - margin);
            }
            while std::time::Instant::now() < next {
                std::thread::yield_now();
            }
            c.host.pace_next = Some(next + PERIOD);
        }
        _ => {
            c.host.pace_next = Some(t + PERIOD);
            PACE_RESETS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        }
    }
    // BT3_GS_VERBOSE: once a second, how the blanks came
    use std::sync::atomic::Ordering::Relaxed;
    if PACE_COUNT.fetch_add(1, Relaxed) == 59 {
        PACE_COUNT.store(0, Relaxed);
        if std::env::var("BT3_GS_VERBOSE").is_ok() {
            let start = PACE_START.with(|s| s.replace(Some(std::time::Instant::now())));
            if let Some(st) = start {
                eprintln!("pace: 60 vertical blanks in {:.1} ms (1001.1 expected), {} new grids", st.elapsed().as_secs_f64() * 1e3, PACE_RESETS.swap(0, Relaxed));
            }
        }
    }
}

static PACE_COUNT: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
static PACE_RESETS: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
thread_local! { static PACE_START: std::cell::Cell<Option<std::time::Instant>> = const { std::cell::Cell::new(None) }; }

pub fn port_vblank(c: &mut Ctx) {
    saves::frame(c);
    if c.host.paced.is_none() {
        let window = c.host.gs.as_ref().and_then(|g| g.gpu.as_ref()).map_or(false, |g| g.surface.has_window());
        c.host.paced = Some((window || std::env::var("BT3_PACED").is_ok()) && std::env::var("BT3_UNCAPPED").is_err());
    }
    if c.host.paced == Some(true) {
        vblank_wait(c);
    }
    c.host.vblanks = c.host.vblanks.wrapping_add(1);
    if c.host.exit_at == 0 {
        c.host.exit_at = std::env::var("BT3_EXIT_AT").ok().and_then(|v| v.parse().ok()).unwrap_or(u32::MAX);
    }
    if c.host.vblanks >= c.host.exit_at {
        println!("bt3: stopped at vertical blank {}", c.host.vblanks);
        std::process::exit(0);
    }
    F::Port_AdxTick(c);
    snd::tick(c);
    harness::trace(c);
    let h = c.host.vsync_handler;
    if h != 0 {
        crate::gen::dispatch::call(c, h, &[0]);
    }
}

pub fn sceGsSyncV(c: &mut Ctx, _mode: i32) -> i32 { port_vblank(c); 0 }
pub fn ADXM_WaitVsync(c: &mut Ctx) { port_vblank(c); }

// ------------------------------------------------------------------ GS (headless: nothing is drawn)
pub fn sceGsResetGraph(_c: &mut Ctx, _a: i16, _b: i16, _d: i16, _e: i16) {}
pub fn sceGsResetPath(_c: &mut Ctx) {}
pub fn sceGsSetDefStoreImage(_c: &mut Ctx, _p: u32, _a: i16, _b: i16, _d: i16, _e: i16, _f: i16, _g: i16, _h: i16) {}
pub fn sceGsExecStoreImage(_c: &mut Ctx, _a: u32, _b: u32) -> i32 { 0 }
pub fn sceGsSyncPath(_c: &mut Ctx, _a: i32, _b: u16) -> i32 { 0 }
/// libgraph's double buffer: for each of the two buffers a GIF packet "drawing environment + clear sprite" that
/// sceGsSwapDBuff sends (the game patches the sprite's corners and colour). Layout: disp[2][5], then for each
/// buffer a GIF tag, 8 A+D register writes (environment) and 6 (clear). As port/src/gs/gs_sony.c.
pub fn sceGsSetDefDBuff(c: &mut Ctx, p: u32, psm: i16, w: i16, h: i16, ztest: i16, zpsm: i16, clear: i16) -> i32 {
    let (psm, w, h, ztest, zpsm) = (psm as u64, w as i64, h as i64, ztest as u64, zpsm as u64);
    let bytes = if psm & 2 != 0 { 2 } else { 4 };
    let pages = ((w * h * bytes + 8191) / 8192) as u64;
    c.m.fill(p, 0, 560);
    let test = if ztest != 0 { (1u64 << 16) | ((ztest & 3) << 17) } else { 0 };
    for (k, fbp) in [(0u32, 0u64), (1, pages)] {
        let tag = p + 80 + k * 240;
        let reg = |i: u32| tag + 16 + i * 16;
        c.m.w64(tag, (if clear != 0 { 14 } else { 8 }) | (1 << 15) | (1 << 60));
        c.m.w64(tag + 8, 0xE);
        let half_w = (w / 2) as u64;
        let half_h = (h / 2) as u64;
        let regs: [(u64, u64); 14] = [
            (fbp | (((w + 63) / 64) as u64) << 16 | psm << 24, 0x4C),
            (pages * 2 | (zpsm & 15) << 24 | ((ztest == 0) as u64) << 32, 0x4E),
            (((2048 - half_w) << 4) | ((2048 - half_h) << 4) << 32, 0x18),
            (((w - 1) as u64) << 16 | ((h - 1) as u64) << 48, 0x40),
            (1, 0x1A),
            (1, 0x46),
            (if psm & 2 != 0 { 1 } else { 0 }, 0x45),
            (test, 0x47),
            ((1 << 16) | (1 << 17), 0x47),
            (6, 0x00),
            (0, 0x01),
            (((2048 - half_w) << 4) | ((2048 - half_h) << 4) << 16, 0x05),
            (((2048 + half_w) << 4) | ((2048 + half_h) << 4) << 16, 0x05),
            (test, 0x47),
        ];
        for (i, (v, a)) in regs.iter().enumerate() {
            c.m.w64(reg(i as u32), *v);
            c.m.w64(reg(i as u32) + 8, *a);
        }
    }
    0
}
pub fn sceGsPutDrawEnv(c: &mut Ctx, giftag: u32) -> i32 {
    let qwc = 1 + (c.m.r64(giftag) & 0x7FFF) as u32;
    Port_GsGifChannel(c, giftag, qwc, 0);
    0
}
pub fn sceGsSwapDBuff(c: &mut Ctx, p: u32, id: i32) {
    let tag = p + 80 + if id & 1 != 0 { 240 } else { 0 };
    let fbp = (c.m.r64(tag + 16) & 0x1FF) as i32;
    if let Some(g) = gs_get(c) {
        g.main_fbp = fbp;
    }
    sceGsPutDrawEnv(c, tag);
}
/// The renderer, when BT3_GS asks for one.
pub fn gs_get(c: &mut Ctx) -> Option<&mut gs::Gs> {
    if !c.host.gs_checked {
        c.host.gs_checked = true;
        if gs::enabled() {
            c.host.gs = Some(gs::Gs::new());
        }
    }
    c.host.gs.as_deref_mut()
}
pub fn Port_GsGifChannel(c: &mut Ctx, addr: u32, qwc: u32, chain: i32) {
    if gs_get(c).is_none() {
        return;
    }
    let mut g = c.host.gs.take().unwrap();
    let t = std::time::Instant::now();
    g.gif_channel(c, addr, qwc, chain != 0);
    g.times[0] += t.elapsed().as_secs_f64();
    c.host.gs = Some(g);
}
pub fn Port_GsVif1Chain(c: &mut Ctx, tadr: u32, tte: i32) {
    if gs_get(c).is_none() {
        return;
    }
    let mut g = c.host.gs.take().unwrap();
    let t = std::time::Instant::now();
    g.run_chain(c, tadr, tte != 0);
    g.times[0] += t.elapsed().as_secs_f64();
    c.host.gs = Some(g);
}
pub fn Port_NameFontSheets(_c: &mut Ctx, _a: i32, _b: i32, _d: i32) {}

// ------------------------------------------------------------------ pads (headless: idle)
pub fn sceDbcInit(_c: &mut Ctx) -> i32 { 1 }
pub fn scePad2Init(_c: &mut Ctx, _m: i32) -> i32 { 1 }
pub fn scePad2CreateSocket(c: &mut Ctx, _a: u32, _b: u32) -> i32 { let s = c.host.pad_sockets; c.host.pad_sockets += 1; s }
pub fn scePad2GetState(_c: &mut Ctx, _s: i32) -> i32 { 1 }
pub fn scePad2GetButtonProfile(c: &mut Ctx, _s: i32, p: u32) -> i32 {
    c.m.write_bytes(p, &[0xFF, 0xFF, 0xFF, 0x03]);
    4
}
fn pad_script(c: &Ctx) -> u16 {
    static EV: std::sync::OnceLock<Vec<(u32, u16)>> = std::sync::OnceLock::new();
    let ev = EV.get_or_init(|| {
        let names = [("start", 1u16 << 3), ("select", 1), ("up", 1 << 4), ("down", 1 << 6), ("left", 1 << 7), ("right", 1 << 5),
            ("cross", 1 << 14), ("circle", 1 << 13), ("square", 1 << 15), ("triangle", 1 << 12), ("l1", 1 << 10), ("r1", 1 << 11),
            ("l2", 1 << 8), ("r2", 1 << 9)];
        std::env::var("BT3_PAD").unwrap_or_default().split(',').filter_map(|e| {
            let (f, n) = e.trim().split_once(':')?;
            Some((f.parse().ok()?, names.iter().find(|x| x.0 == n)?.1))
        }).collect()
    });
    if ev.is_empty() {
        return 0;
    }
    let frame = c.host.gs.as_ref().map_or(c.host.vblanks, |g| g.frame);
    ev.iter().filter(|&&(f, _)| frame >= f && frame < f + 4).fold(0, |a, &(_, b)| a | b)
}

/// The buttons of the first controller pressed now (PS2 bits, active high): the scripted ones (BT3_PAD), else the
/// keyboard / gamepad of the window. For the port's own screens (host/saves.rs).
pub fn host_buttons(c: &Ctx) -> u16 {
    let b = pad_script(c);
    if b != 0 {
        return b;
    }
    c.host.gs.as_ref().and_then(|g| g.gpu.as_ref()).filter(|g| g.surface.has_window()).map_or(0, |g| g.input.buttons)
}

pub fn scePad2Read(c: &mut Ctx, socket: i32, data: u32) -> i32 {
    // while the port's slot list is up, the game sees no button
    if saves::modal() {
        let mut d = [0u8; 18];
        d[0] = 0xFF;
        d[1] = 0xFF;
        d[2..6].fill(0x80);
        c.m.write_bytes(data, &d);
        return 18;
    }
    let mut d = [0u8; 18];
    d[0] = 0xFF;
    d[1] = 0xFF;
    d[2..6].fill(0x80);
    // BT3_PAD="frame:button,...": scripted presses for player 1, each held for 4 frames (the renderer's frame
    // counter), as the C port's script_buttons: for tests without a person at the keyboard
    if socket == 0 {
        let btn = pad_script(c);
        d[0] &= !(btn as u8);
        d[1] &= !((btn >> 8) as u8);
    }
    // with a window, the first controller port is the keyboard / the first gamepad (buttons active low)
    if socket == 0 && d[0] == 0xFF && d[1] == 0xFF {
        if let Some(inp) = c.host.gs.as_ref().and_then(|g| g.gpu.as_ref()).filter(|g| g.surface.has_window()).map(|g| g.input) {
            d[0] = !(inp.buttons as u8);
            d[1] = !((inp.buttons >> 8) as u8);
            d[2] = inp.rx;
            d[3] = inp.ry;
            d[4] = inp.lx;
            d[5] = inp.ly;
        }
    }
    c.m.write_bytes(data, &d);
    18
}
pub fn sceVibGetProfile(_c: &mut Ctx, _a: i32, _b: u32) -> i32 { 0 }
/// The controller's motors (Pad_SendActuator: one byte, large motor power << 1 | small motor), to the window's
/// gamepad (gs/platform.rs). Only controller 1 has a gamepad here.
pub fn sceVibSetActParam(c: &mut Ctx, socket: i32, _profile_size: i32, _profile: u32, size: i32, data: u32) -> i32 {
    if socket == 0 && size > 0 && data != 0 {
        gs::platform::RUMBLE.store(c.m.r8(data) as u32, std::sync::atomic::Ordering::Relaxed);
    }
    0
}

// ------------------------------------------------------------------ second processor, sound driver
pub fn sceSifInitIopHeap(_c: &mut Ctx) -> i32 { 0 }
pub fn sceSifAllocIopHeap(c: &mut Ctx, size: i32) -> u32 { low_alloc(c, size as u32) }
pub fn sceSifFreeIopHeap(c: &mut Ctx, addr: u32) -> i32 { low_free(c, addr); 0 }
pub fn sceSifQueryTotalFreeMemSize(_c: &mut Ctx) -> i32 { 0x100000 }
pub fn sceSifQueryMaxFreeMemSize(_c: &mut Ctx) -> i32 { 0x100000 }
pub fn sceSifBindRpc(c: &mut Ctx, client: u32, _id: i32, _mode: i32) -> i32 { c.m.w32(client + 0x24, client); 0 }
pub fn func_002B4AF0(_c: &mut Ctx, _a: u32) -> i32 { 0 }
pub fn sceSifDmaStat(_c: &mut Ctx, _id: u32) -> i32 { -1 }
pub fn sceSdRemoteInit(_c: &mut Ctx) -> i32 { 0 }
pub fn func_00296B48(_c: &mut Ctx, _a: i32) -> i32 { 0 }
pub fn sceSifSetDma(c: &mut Ctx, d: u32, count: i32) -> u32 {
    for i in 0..count.max(0) as u32 {
        let e = d + i * 16;
        let (data, addr, size) = (c.m.r32(e), c.m.r32(e + 4), c.m.r32(e + 8) as i32);
        if addr != 0 && data != 0 && size > 0 {
            c.m.copy(addr, data, size as u32);
        }
    }
    1
}
/// sceSdRemote: 0x80D0 transfers to the sound chip (headless: nothing plays), 0x80F0 asks whether it finished.
pub fn func_002967C0(c: &mut Ctx, _block: i32, cmd: i32, rest: &[u64]) -> i32 {
    // rest: core, mode, iopAddr, spuAddr, size
    if cmd == 0x80D0 && rest.len() >= 5 {
        snd::spu_upload(c, rest[2] as u32, rest[3] as u32, rest[4] as u32);
    }
    if cmd == 0x80F0 { 1 } else { 0 }
}
pub fn sceSifCallRpc(c: &mut Ctx, _client: u32, fno: i32, _mode: i32, send: u32, _ss: i32, recv: u32, rs: i32, _cb: u32, _arg: u32) -> i32 {
    if recv != 0 && rs >= 4 {
        c.m.w32(recv, 0);
    }
    snd::rpc(c, fno, send);
    0
}

// ------------------------------------------------------------------ CRI ADX stream players (headless: stopped)
pub fn ADXT_Init(_c: &mut Ctx) {}
pub fn ADXT_Create(c: &mut Ctx, _maxnch: i32, work: u32, _ws: i32) -> u32 {
    let n = c.m.r32(crate::gen::sym::gPortAdxCount);
    c.m.w32(crate::gen::sym::gPortAdxCount, n + 1);
    let h = low_alloc(c, 16);
    snd::adxt_create(c, h);
    h
}
pub fn ADXT_SetReloadSct(_c: &mut Ctx, _a: u32, _n: i32) {}
pub fn ADXT_SetSvrFreq(_c: &mut Ctx, _a: u32, _f: i32) {}
pub fn ADXT_StartAfs(c: &mut Ctx, a: u32, p: i32, f: i32) { snd::adxt_start(c, a, p, f, None); }
pub fn ADXT_StartFname(c: &mut Ctx, a: u32, f: u32) { let n = c.m.cstr(f); snd::adxt_start(c, a, 0, 0, Some(n)); }
pub fn ADXT_Stop(c: &mut Ctx, a: u32) { snd::adxt_stop(c, a); }
pub fn ADXT_Pause(c: &mut Ctx, a: u32, sw: i32) { snd::adxt_pause(c, a, sw != 0); }
pub fn ADXT_SetOutVol(c: &mut Ctx, a: u32, vol: i32) { c.m.w32(a, vol as u32); snd::adxt_volume(c, a, vol); }
pub fn ADXT_GetOutVol(c: &mut Ctx, a: u32) -> i32 { c.m.r32(a) as i32 }
pub fn ADXT_SetOutPan(c: &mut Ctx, a: u32, ch: i32, pan: i32) { snd::adxt_pan(c, a, ch, pan); }
pub fn ADXT_SetOutputMono(c: &mut Ctx, f: i32) { snd::adxt_mono(c, f != 0); }
pub fn ADXT_GetStat(c: &mut Ctx, a: u32) -> i32 { snd::adxt_stat(c, a) }
pub fn func_00277A70(_c: &mut Ctx, _a: u32) -> i32 { 0 }
pub fn ADXPS2_ExecVint(_c: &mut Ctx, _m: i32) {}
pub fn ADXPS2_SetupDvdFs(_c: &mut Ctx, _p: u32) {}
pub fn ADXPS2_LoadFcacheDvd(_c: &mut Ctx, _p: u32) {}
pub fn ADXPS2_SetupThrd(_c: &mut Ctx, _p: u32, _u: i32) {}
pub fn ADXM_ExecMain(_c: &mut Ctx) {}
pub fn ADXERR_EntryErrFunc(_c: &mut Ctx, _f: u32, _o: u32) {}

// ------------------------------------------------------------------ MPEG movies (headless: none)
pub fn sceMpegInit(_c: &mut Ctx) -> i32 { 0 }
pub fn sceMpegCreate(_c: &mut Ctx, _m: u32, _w: u32, _s: i32) -> i32 { 0 }
pub fn sceMpegDelete(_c: &mut Ctx, _m: u32) -> i32 { 0 }
pub fn sceMpegReset(_c: &mut Ctx, _m: u32) -> i32 { 0 }
pub fn sceMpegAddCallback(_c: &mut Ctx, _m: u32, _t: i32, _f: u32, _d: u32) -> u32 { 0 }
pub fn sceMpegAddStrCallback(_c: &mut Ctx, _m: u32, _t: i32, _n: i32, _f: u32, _d: u32) -> u32 { 0 }
pub fn sceMpegDemuxPss(_c: &mut Ctx, _m: u32, _p: u32, _s: i32) -> i32 { 0 }
pub fn sceMpegGetPicture(_c: &mut Ctx, _m: u32, _r: u32, _n: i32) -> i32 { 0 }
pub fn sceMpegIsEnd(_c: &mut Ctx, _m: u32) -> i32 { 1 }

// ------------------------------------------------------------------ CD / DVD
pub fn sceCdGetDiskTypeSafe(_c: &mut Ctx) -> i32 { 0x14 }
pub fn sceCdStatus(_c: &mut Ctx) -> i32 { 10 }
pub fn sceCdSearchFile(_c: &mut Ctx, _f: u32, _n: u32) -> i32 { 1 }
pub fn sceOpen(_c: &mut Ctx, _n: u32, _f: i32) -> i32 { 3 }
pub fn sceLseek(_c: &mut Ctx, _fd: i32, _o: i32, _w: i32) -> i32 { 0 }
pub fn sceRead(_c: &mut Ctx, _fd: i32, _b: u32, size: i32) -> i32 { size }
pub fn sceClose(_c: &mut Ctx, _fd: i32) -> i32 { 0 }

// ------------------------------------------------------------------ PC options of the port
/// The picture's width over the console's 4:3 width: 4/3 at 16:9. The game's projection, culling and effect
/// proportions take it (btl_cam, stg_a, eft_*). In the menus it is 1: their pages are composed for 4:3 and are
/// shown at 4:3 in the middle (gs/gpu.rs MENU_MODE).
pub fn Port_WideFactor(c: &mut Ctx) -> F32 {
    if settings::wide() && !c.host.menu_mode {
        F32((settings::aspect_milli() as f32 * 3.0 / 4000.0).to_bits())
    } else {
        F32(0x3F80_0000)
    }
}
pub fn Port_StageBufSize(_c: &mut Ctx) -> i32 { 0 }
pub fn Port_TestBgm(_c: &mut Ctx) -> i32 { -1 }
pub fn Port_TestScreen(_c: &mut Ctx) -> i32 { -1 }
pub fn Port_TestStage(_c: &mut Ctx) -> i32 { -1 }
pub fn Port_SongDebug(_c: &mut Ctx, _a: i32, _b: i32, _d: i32, _e: i32, _f: i32, _g: i32) {}
pub fn Port_NetLeave(_c: &mut Ctx) {}
pub fn Port_NetMenuListed(_c: &mut Ctx) -> i32 { 1 }
pub fn Port_NetMenuOpen(_c: &mut Ctx) {}
pub fn Port_NetSession(_c: &mut Ctx) -> i32 { 0 }
pub fn Port_NetView(_c: &mut Ctx) -> i32 { -1 }

// ------------------------------------------------------------------ memory card (none yet)
pub fn func_002A1A48(_c: &mut Ctx) -> i32 { 0 }
pub fn func_002A25B8(_c: &mut Ctx, _p: i32, _s: i32, _t: u32, _f: u32, _fm: u32) -> i32 { 0 }
pub fn func_002A1E58(_c: &mut Ctx, _p: i32, _s: i32, _n: u32, _m: i32) -> i32 { 0 }
pub fn func_002A1F80(_c: &mut Ctx, _p: i32, _s: i32, _n: u32) -> i32 { 0 }
pub fn func_002A1FB8(_c: &mut Ctx, _fd: i32) -> i32 { 0 }
pub fn func_002A2078(_c: &mut Ctx, _fd: i32, _o: i32, _w: i32) -> i32 { 0 }
pub fn func_002A2208(_c: &mut Ctx, _fd: i32, _b: u32, _s: i32) -> i32 { 0 }
pub fn func_002A2320(_c: &mut Ctx, _fd: i32, _b: u32, _s: i32) -> i32 { 0 }
pub fn func_002A27A8(_c: &mut Ctx, _p: i32, _s: i32, _n: u32, _m: i32, _max: i32, _t: u32) -> i32 { 0 }
pub fn func_002A2AC8(_c: &mut Ctx, _p: i32, _s: i32) -> i32 { 0 }
pub fn func_002A2498(c: &mut Ctx, _mode: i32, cmd: u32, result: u32) -> i32 {
    if cmd != 0 { c.m.w32(cmd, 0); }
    if result != 0 { c.m.w32(result, 0); }
    1
}
