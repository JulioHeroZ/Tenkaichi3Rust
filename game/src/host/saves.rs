//! Saving as a PC game does it, in place of the memory card: three profile slots, the game saved by itself at
//! every point where the PS2 asked "save?", and a "saving" sign in the corner while it happens.
//!
//! The game drives every card operation through McFlow_Start(mode) / McFlow_Update (src/sys/mcflow_a.c): eight
//! dialog state machines over the card layer. They are replaced here (bound in gen/funcs.rs, see
//! tools/translate/c2rs/link.py WRAP) by flows that do the same work on the game's data without the card and its
//! questions, and end the same way (flags 8 = done / 0x100 = refused, then the same callbacks):
//!
//! * save (0, 4) and new save (3): gSaveData (0x4000 bytes, the card format with its checksum) written to the
//!   profile's `system.sav` (the previous one kept as `system.bak`);
//! * boot load (2) and load (1): the slot list; the slot picked is read into gSaveData;
//! * replays (5..9): `replay_<n>.sav` in the profile, in the card's replay file format;
//! * the replay question (8) is asked by the slot list's window.
//!
//! Saves live in Documents\My Games\Tenkaichi3Rust\saves\slot<N>\ (BT3_SAVES=<folder> to put them elsewhere).
#![allow(non_snake_case)]
use crate::rt::prelude::*;
use std::path::PathBuf;
use std::sync::Mutex;
use std::time::{Duration, Instant};

use super::ui::{self, Canvas};

const SLOTS: usize = 3;
const SAVE_SIZE: usize = 0x4000;
const REPLAY_SIZE: usize = 0x1AC00;
/// gMcFlow->state while one of these flows runs (no state of the game's own flows has this value).
const BUSY: u32 = 0x7F;

// McFlow fields (include/sys/mcflow_a.h)
const F_FLAGS: u32 = 0x0C;
const F_MODE: u32 = 0x20;
const F_STATE: u32 = 0x24;
const F_REP_SAVE: u32 = 0x4038;
const F_REP_LOAD: u32 = 0x1EC38;
const F_SLOT_HDR: u32 = 0x39838;
const F_FLOW: u32 = 0x39DE0;
const F_SLOT: u32 = 0x39F88;
// gProgress->flags: bit 2 "continue without saving" chosen at boot, bit 3 a save was loaded
const P_FLAGS: u32 = 0x14;

#[derive(Clone, Copy, PartialEq)]
enum Purpose {
    Boot,
    Load,
    NewGame,
    ReplayAsk,
}

struct Picker {
    purpose: Purpose,
    cursor: usize,
    /// asking whether to overwrite this slot (new game)
    confirm: Option<usize>,
    yes: bool,
    last_buttons: u16,
    /// frames since it opened
    steps: u32,
}

struct State {
    active: Option<usize>,
    /// the flow running: its mode, and whether it ended (then the callback runs at the next McFlow_Update)
    picker: Option<Picker>,
    finished: bool,
    saving_since: Option<Instant>,
    play_from: Option<Instant>,
    /// what the canvas shows now (drawn again only when it changes)
    shown: String,
}

static STATE: Mutex<State> = Mutex::new(State { active: None, picker: None, finished: false, saving_since: None, play_from: None, shown: String::new() });

// ---------------------------------------------------------------------------------------------- the files
fn root() -> PathBuf {
    static ROOT: std::sync::OnceLock<PathBuf> = std::sync::OnceLock::new();
    ROOT.get_or_init(|| match std::env::var("BT3_SAVES") {
        Ok(p) => PathBuf::from(p),
        Err(_) => documents().join("My Games").join("Tenkaichi3Rust").join("saves"),
    })
    .clone()
}

#[cfg(windows)]
pub fn documents() -> PathBuf {
    use std::ffi::c_void;
    #[repr(C)]
    struct Guid(u32, u16, u16, [u8; 8]);
    const FOLDERID_DOCUMENTS: Guid = Guid(0xFDD3_9AD0, 0x238F, 0x46AF, [0xAD, 0xB4, 0x6C, 0x85, 0x48, 0x03, 0x69, 0xC7]);
    #[link(name = "shell32")]
    extern "system" {
        fn SHGetKnownFolderPath(id: *const Guid, flags: u32, token: *mut c_void, out: *mut *mut u16) -> i32;
    }
    #[link(name = "ole32")]
    extern "system" {
        fn CoTaskMemFree(p: *mut c_void);
    }
    unsafe {
        let mut p: *mut u16 = std::ptr::null_mut();
        if SHGetKnownFolderPath(&FOLDERID_DOCUMENTS, 0, std::ptr::null_mut(), &mut p) == 0 && !p.is_null() {
            let mut n = 0;
            while *p.add(n) != 0 {
                n += 1;
            }
            let s = String::from_utf16_lossy(std::slice::from_raw_parts(p, n));
            CoTaskMemFree(p as *mut c_void);
            return PathBuf::from(s);
        }
    }
    PathBuf::from(std::env::var("USERPROFILE").unwrap_or_else(|_| ".".into())).join("Documents")
}

#[cfg(not(windows))]
pub fn documents() -> PathBuf {
    PathBuf::from(std::env::var("HOME").unwrap_or_else(|_| ".".into())).join("Documents")
}

fn slot_dir(slot: usize) -> PathBuf {
    root().join(format!("slot{}", slot + 1))
}

/// What the slot list shows of a profile.
struct Info {
    saved: i64,
    play: u64,
    chars: u32,
    stages: u32,
    money: u32,
}

fn info(slot: usize) -> Option<Info> {
    let d = slot_dir(slot);
    let data = std::fs::read(d.join("system.sav")).ok().filter(|b| b.len() == SAVE_SIZE)?;
    let mut saved = 0i64;
    let mut play = 0u64;
    if let Ok(t) = std::fs::read_to_string(d.join("info.txt")) {
        for l in t.lines() {
            if let Some(v) = l.strip_prefix("saved=") {
                saved = v.trim().parse().unwrap_or(0);
            }
            if let Some(v) = l.strip_prefix("play_seconds=") {
                play = v.trim().parse().unwrap_or(0);
            }
        }
    }
    let rd64 = |o: usize| u64::from_le_bytes(data[o..o + 8].try_into().unwrap());
    let chars = rd64(0xC10).count_ones() + rd64(0xC18).count_ones() + (rd64(0xC20) & ((1u64 << 33) - 1)).count_ones();
    let stages = (rd64(0xC28) & ((1u64 << 35) - 1)).count_ones();
    let money = u32::from_le_bytes(data[0x3028..0x302C].try_into().unwrap());
    Some(Info { saved, play, chars, stages, money })
}

fn now_unix() -> i64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_secs() as i64)
}

/// Writes a file through a temporary one, so a crash never leaves half a save.
fn write_atomic(path: &std::path::Path, data: &[u8]) -> std::io::Result<()> {
    if let Some(d) = path.parent() {
        std::fs::create_dir_all(d)?;
    }
    let tmp = path.with_extension("tmp");
    std::fs::write(&tmp, data)?;
    std::fs::rename(&tmp, path)
}

/// The game's save written to the profile `slot`; the play time and the moment kept beside it.
fn write_system(c: &mut Ctx, slot: usize) {
    let save = c.m.r32(sym::gSaveData);
    F::McCard_Checksum(c, save, SAVE_SIZE as i32, 1);
    let mut data = vec![0u8; SAVE_SIZE];
    c.m.read_bytes(save, &mut data);
    let d = slot_dir(slot);
    let file = d.join("system.sav");
    if file.is_file() {
        let _ = std::fs::copy(&file, d.join("system.bak"));
    }
    let before = info(slot).map_or(0, |i| i.play);
    let play = {
        let mut s = STATE.lock().unwrap();
        let add = s.play_from.map_or(0, |t| t.elapsed().as_secs());
        s.play_from = Some(Instant::now());
        s.saving_since = Some(Instant::now());
        before + add
    };
    match write_atomic(&file, &data) {
        Ok(()) => {
            let _ = write_atomic(&d.join("info.txt"), format!("saved={}\nplay_seconds={}\n", now_unix(), play).as_bytes());
            eprintln!("bt3: saved to {}", file.display());
        }
        Err(e) => eprintln!("bt3: could not save to {}: {e}", file.display()),
    }
}

/// The profile `slot` read into gSaveData (its backup when the save is damaged). Whether it worked.
fn read_system(c: &mut Ctx, slot: usize) -> bool {
    let d = slot_dir(slot);
    for name in ["system.sav", "system.bak"] {
        let Ok(data) = std::fs::read(d.join(name)) else { continue };
        if data.len() != SAVE_SIZE {
            continue;
        }
        // checked as the card flows do: into the work buffer first, then copied
        let save = c.m.r32(sym::gSaveData);
        let mut old = vec![0u8; SAVE_SIZE];
        c.m.read_bytes(save, &mut old);
        c.m.write_bytes(save, &data);
        if F::McCard_Checksum(c, save, SAVE_SIZE as i32, 0) == 0 {
            if name != "system.sav" {
                eprintln!("bt3: {} was damaged; loaded the backup", d.join("system.sav").display());
            }
            let mut s = STATE.lock().unwrap();
            s.play_from = Some(Instant::now());
            return true;
        }
        c.m.write_bytes(save, &old);
    }
    false
}

/// Whether any profile has a save (the title's Load Game item, host/title.rs).
pub fn any_saved() -> bool {
    (0..SLOTS).any(|k| info(k).is_some())
}

fn last_slot() -> Option<usize> {
    (0..SLOTS).filter_map(|s| info(s).map(|i| (s, i.saved))).max_by_key(|x| x.1).map(|x| x.0)
}

// ---------------------------------------------------------------------------------------------- the flows
fn mcflow(c: &Ctx) -> u32 {
    c.m.r32(sym::gMcFlow)
}

fn set_flag(c: &mut Ctx, bit: u32) {
    let w = mcflow(c);
    let f = c.m.r32(w + F_FLAGS);
    c.m.w32(w + F_FLAGS, f | bit);
}

fn progress_flags(c: &mut Ctx, set: u32, clear: u32) {
    let p = c.m.r32(sym::gProgress);
    let f = c.m.r32(p + P_FLAGS);
    c.m.w32(p + P_FLAGS, (f | set) & !clear);
}

fn open_picker(purpose: Purpose) {
    let active = STATE.lock().unwrap().active;
    let cursor = match purpose {
        Purpose::NewGame => (0..SLOTS).find(|&k| info(k).is_none()).unwrap_or(active.unwrap_or(0)),
        _ => active.or_else(last_slot).unwrap_or(0),
    };
    let mut s = STATE.lock().unwrap();
    s.picker = Some(Picker { purpose, cursor, confirm: None, yes: false, last_buttons: 0xFFFF, steps: 0 });
}

fn finish() {
    STATE.lock().unwrap().finished = true;
}

/// Starts flow `mode` (McFlow_Start of src/sys/mcflow_a.c).
pub fn McFlow_Start(c: &mut Ctx, mode: i32) {
    if std::env::var("BT3_MEMCARD").is_ok() {
        return F::__real_McFlow_Start(c, mode);
    }
    eprintln!("bt3: save flow {mode} at blank {} ({})", c.host.vblanks, ["save", "load", "boot load", "new save", "save", "replay save", "replay load", "replay list", "replay question", "replay list"].get(mode as usize).unwrap_or(&"?"));
    let w = mcflow(c);
    c.m.w32(w + F_MODE, mode as u32);
    c.m.w32(w + F_FLOW, 0);
    c.m.w32(w + F_STATE, BUSY);
    STATE.lock().unwrap().finished = false;
    let active = STATE.lock().unwrap().active;
    match mode {
        0 | 4 => {
            // save: no question, to the profile in use
            let slot = active.unwrap_or_else(|| {
                let k = (0..SLOTS).find(|&k| info(k).is_none()).unwrap_or(0);
                STATE.lock().unwrap().active = Some(k);
                k
            });
            write_system(c, slot);
            progress_flags(c, 0, 4);
            set_flag(c, 8);
            finish();
        }
        1 => open_picker(Purpose::Load),
        2 => {
            if (0..SLOTS).all(|k| info(k).is_none()) {
                // nothing saved yet: the first profile, a new game
                STATE.lock().unwrap().active = Some(0);
                set_flag(c, 8);
                finish();
            } else {
                // the profile saved last, without asking (the title's Load Game picks another)
                let k = last_slot().unwrap_or(0);
                STATE.lock().unwrap().active = Some(k);
                if read_system(c, k) {
                    eprintln!("bt3: profile {} loaded", k + 1);
                    F::SndOpt_Apply(c);
                    progress_flags(c, 8, 0);
                }
                set_flag(c, 8);
                finish();
            }
        }
        3 => {
            let p = c.m.r32(sym::gProgress);
            let loaded = c.m.r32(p + P_FLAGS) & 8 != 0;
            eprintln!("bt3: new save: profile {active:?}, a save loaded: {loaded}");
            match active {
                // the profile picked at boot is empty: the new game goes there
                Some(k) if !loaded && info(k).is_none() => new_game(c, k),
                _ => open_picker(Purpose::NewGame),
            }
        }
        5 => {
            let slot = c.m.r32(w + F_SLOT) as usize;
            let mut data = vec![0u8; REPLAY_SIZE];
            c.m.read_bytes(w + F_REP_SAVE, &mut data);
            let file = slot_dir(active.unwrap_or(0)).join(format!("replay_{}.sav", slot + 1));
            match write_atomic(&file, &data) {
                Ok(()) => {
                    STATE.lock().unwrap().saving_since = Some(Instant::now());
                    F::McFlow_CopySlotInfo(c, 0, slot as i32);
                    set_flag(c, 8);
                }
                Err(e) => eprintln!("bt3: could not save the replay to {}: {e}", file.display()),
            }
            finish();
        }
        6 => {
            let slot = c.m.r32(w + F_SLOT) as usize;
            let file = slot_dir(active.unwrap_or(0)).join(format!("replay_{}.sav", slot + 1));
            if let Ok(data) = std::fs::read(&file) {
                if data.len() == REPLAY_SIZE {
                    c.m.write_bytes(w + F_REP_LOAD, &data);
                    if F::McCard_Checksum(c, w + F_REP_LOAD, REPLAY_SIZE as i32, 0) == 0 {
                        F::BattleReplay_Load(c, w + F_REP_LOAD + 0x38, 0x1ABA8);
                        set_flag(c, 8);
                    }
                }
            }
            finish();
        }
        7 | 9 => {
            let f = c.m.r32(w + F_FLAGS);
            c.m.w32(w + F_FLAGS, f & !0x70);
            if mode == 7 {
                F::Progress_ClearTeams(c);
            }
            for i in 0..7u32 {
                let hdr = w + F_SLOT_HDR + i * 0x38;
                let file = slot_dir(active.unwrap_or(0)).join(format!("replay_{}.sav", i + 1));
                let ok = match std::fs::read(&file) {
                    Ok(data) if data.len() == REPLAY_SIZE => {
                        c.m.write_bytes(hdr, &data[..0x38]);
                        F::McCard_Checksum(c, hdr + 8, 0x30, 0) == 0
                    }
                    _ => false,
                };
                if ok {
                    set_flag(c, 0x10);
                    F::McFlow_CopySlotInfo(c, 1, i as i32);
                } else {
                    F::McFlow_ClearSlotInfo(c, 1, i as i32);
                    set_flag(c, 0x20);
                }
            }
            set_flag(c, 8);
            finish();
        }
        8 => open_picker(Purpose::ReplayAsk),
        _ => finish(),
    }
}

fn new_game(c: &mut Ctx, slot: usize) {
    let save = c.m.r32(sym::gSaveData);
    F::Save_SetDefaults(c, save);
    F::McCard_Checksum(c, save, SAVE_SIZE as i32, 1);
    F::SndOpt_Apply(c);
    F::Progress_ClearSession(c);
    {
        let mut s = STATE.lock().unwrap();
        s.active = Some(slot);
        s.play_from = Some(Instant::now());
    }
    // a new profile starts its play time from zero
    let _ = std::fs::remove_file(slot_dir(slot).join("info.txt"));
    write_system(c, slot);
    set_flag(c, 8);
    finish();
}

/// Runs the current flow (McFlow_Update): its state, 0 once finished (the callback has run then).
pub fn McFlow_Update(c: &mut Ctx) -> i32 {
    if std::env::var("BT3_MEMCARD").is_ok() {
        return F::__real_McFlow_Update(c);
    }
    let w = mcflow(c);
    if w == 0 {
        draw(None);
        return 0;
    }
    let state = c.m.r32(w + F_STATE);
    if state != BUSY {
        draw(None);
        return F::__real_McFlow_Update(c);
    }
    if STATE.lock().unwrap().picker.is_some() {
        picker_step(c);
    }
    let finished = STATE.lock().unwrap().finished;
    if !finished {
        if !modal() {
            draw(None);
        }
        return BUSY as i32;
    }
    STATE.lock().unwrap().finished = false;
    c.m.w32(w + F_STATE, 0);
    let mode = c.m.r32(w + F_MODE);
    if mode <= 4 {
        F::McFlow_CallDone(c);
    } else {
        F::McFlow_CallModeDone(c);
    }
    draw(None);
    0
}

/// The card is always there (McFlow_PollCard: 1 = a card in port 0).
pub fn McFlow_PollCard(c: &mut Ctx) -> i32 {
    if std::env::var("BT3_MEMCARD").is_ok() {
        return F::__real_McFlow_PollCard(c);
    }
    1
}

/// Whether the slot list is up (the game then gets no buttons).
pub fn modal() -> bool {
    STATE.lock().unwrap().picker.is_some()
}

// ---------------------------------------------------------------------------------------------- the slot list
const UP: u16 = 1 << 4;
const DOWN: u16 = 1 << 6;
const LEFT: u16 = 1 << 7;
const RIGHT: u16 = 1 << 5;
const START: u16 = 1 << 3;
const CROSS: u16 = 1 << 14;
const CIRCLE: u16 = 1 << 13;
const TRIANGLE: u16 = 1 << 12;

fn picker_step(c: &mut Ctx) {
    let buttons = super::host_buttons(c);
    let mut s = STATE.lock().unwrap();
    let Some(p) = s.picker.as_mut() else { return };
    let pressed = buttons & !p.last_buttons;
    p.last_buttons = buttons;
    if pressed != 0 && std::env::var("BT3_SAVES_VERBOSE").is_ok() {
        eprintln!("bt3: slot list: buttons {pressed:#06x} at blank {}", c.host.vblanks);
    }
    // a few frames before taking buttons, so the press that opened it does not pick at once
    p.steps += 1;
    if p.steps < 8 {
        return;
    }
    let ok = pressed & (CROSS | START) != 0;
    let back = pressed & (CIRCLE | TRIANGLE) != 0;
    let purpose = p.purpose;
    enum Act {
        None,
        Close(Option<usize>, bool),
    }
    let mut act = Act::None;
    if purpose == Purpose::ReplayAsk || p.confirm.is_some() {
        if pressed & (LEFT | RIGHT | UP | DOWN) != 0 {
            p.yes = !p.yes;
        }
        if ok {
            if purpose == Purpose::ReplayAsk {
                act = Act::Close(None, p.yes);
            } else if p.yes {
                act = Act::Close(p.confirm, true);
            } else {
                p.confirm = None;
            }
        } else if back {
            if purpose == Purpose::ReplayAsk {
                act = Act::Close(None, false);
            } else {
                p.confirm = None;
            }
        }
    } else {
        if pressed & UP != 0 {
            p.cursor = (p.cursor + SLOTS - 1) % SLOTS;
        }
        if pressed & DOWN != 0 {
            p.cursor = (p.cursor + 1) % SLOTS;
        }
        if ok {
            let k = p.cursor;
            if purpose == Purpose::NewGame && info(k).is_some() {
                p.confirm = Some(k);
                p.yes = false;
            } else if purpose == Purpose::Load && info(k).is_none() {
                // nothing to load there
            } else {
                act = Act::Close(Some(k), true);
            }
        } else if back && purpose != Purpose::Boot {
            act = Act::Close(None, false);
        }
    }
    let snapshot = s.picker.as_ref().map(|p| (p.purpose, p.cursor, p.confirm, p.yes));
    drop(s);
    match act {
        Act::None => {
            if let Some(v) = snapshot {
                draw(Some(v));
            }
        }
        Act::Close(slot, yes) => {
            STATE.lock().unwrap().picker = None;
            ui::show(None);
            match purpose {
                Purpose::ReplayAsk => {
                    set_flag(c, if yes { 8 } else { 0x100 });
                    finish();
                }
                Purpose::Boot => {
                    let k = slot.unwrap_or(0);
                    STATE.lock().unwrap().active = Some(k);
                    if info(k).is_some() && read_system(c, k) {
                        eprintln!("bt3: profile {} loaded", k + 1);
                        F::SndOpt_Apply(c);
                        progress_flags(c, 8, 0);
                    }
                    set_flag(c, 8);
                    finish();
                }
                Purpose::Load => {
                    if let Some(k) = slot {
                        if read_system(c, k) {
                            STATE.lock().unwrap().active = Some(k);
                            F::SndOpt_Apply(c);
                            F::Progress_ClearSession(c);
                            set_flag(c, 8);
                        }
                    }
                    finish();
                }
                Purpose::NewGame => match slot {
                    Some(k) => new_game(c, k),
                    None => finish(),
                },
            }
        }
    }
}

fn fmt_time(t: i64) -> String {
    #[repr(C)]
    #[derive(Default)]
    struct Tm {
        sec: i32,
        min: i32,
        hour: i32,
        mday: i32,
        mon: i32,
        year: i32,
        rest: [i64; 6], // wday, yday, isdst and what the C library adds
    }
    let mut tm = Tm::default();
    #[cfg(windows)]
    {
        extern "C" {
            fn _localtime64_s(tm: *mut Tm, t: *const i64) -> i32;
        }
        unsafe { _localtime64_s(&mut tm, &t) };
    }
    #[cfg(not(windows))]
    {
        extern "C" {
            fn localtime_r(t: *const i64, tm: *mut Tm) -> *mut Tm;
        }
        unsafe { localtime_r(&t, &mut tm) };
    }
    format!("{:02}/{:02}/{} {:02}:{:02}", tm.mday, tm.mon + 1, tm.year + 1900, tm.hour, tm.min)
}

fn fmt_money(m: u32) -> String {
    let s = m.to_string();
    let mut out = String::new();
    for (i, ch) in s.chars().enumerate() {
        if i > 0 && (s.len() - i) % 3 == 0 {
            out.push('.');
        }
        out.push(ch);
    }
    out
}

const WHITE: [u8; 4] = [255, 255, 255, 255];
const DIM: [u8; 4] = [190, 196, 210, 255];
const ACCENT: [u8; 4] = [255, 160, 40, 255];

/// Draws what is up: the slot list (purpose, cursor, slot asked about, yes chosen), else the saving sign.
fn draw(list: Option<(Purpose, usize, Option<usize>, bool)>) {
    let saving = STATE.lock().unwrap().saving_since.filter(|t| t.elapsed() < Duration::from_millis(1600));
    if list.is_none() && saving.is_none() {
        let mut s = STATE.lock().unwrap();
        if s.picker.is_none() && !s.shown.is_empty() {
            s.shown.clear();
            ui::show(None);
        }
        return;
    }
    let key = format!(
        "{:?} {}",
        list.map(|(p, k, cf, y)| (p as u8, k, cf, y)),
        saving.map_or(0, |t| t.elapsed().as_millis() / 33 + 1)
    );
    {
        let mut s = STATE.lock().unwrap();
        if s.shown == key {
            return;
        }
        s.shown = key;
    }
    let mut cv = Canvas::new();
    if let Some((purpose, cursor, confirm, yes)) = list {
        cv.rect(0, 0, ui::W as i32, ui::H as i32, 0, [8, 10, 18, 190]);
        if purpose == Purpose::ReplayAsk {
            question(&mut cv, "Voltar sem salvar o replay?", "O replay desta luta será perdido.", yes);
        } else {
            let title = match purpose {
                Purpose::Boot => "Escolha um perfil",
                Purpose::Load => "Carregar perfil",
                _ => "Novo jogo: escolha um slot",
            };
            cv.text_centered(640, 120, title, 1.3, WHITE);
            for k in 0..SLOTS {
                let y = 220 + k as i32 * 170;
                let sel = k == cursor;
                cv.rect(240, y, 800, 150, 18, if sel { [60, 46, 26, 235] } else { [28, 32, 46, 220] });
                if sel {
                    cv.rect(240, y, 10, 150, 4, ACCENT);
                }
                cv.text(280, y + 18, &format!("Slot {}", k + 1), 1.0, if sel { ACCENT } else { WHITE });
                match info(k) {
                    Some(i) => {
                        let h = i.play / 3600;
                        let m = (i.play / 60) % 60;
                        let when = if i.saved > 0 { fmt_time(i.saved) } else { "—".into() };
                        cv.text(280, y + 66, &format!("Salvo em {when}   ·   {h}h {m:02}min de jogo"), 0.7, DIM);
                        cv.text(280, y + 102, &format!("Personagens: {}   ·   Cenários: {}   ·   Zeni: {}", i.chars, i.stages, fmt_money(i.money)), 0.7, DIM);
                    }
                    None => {
                        let t = if purpose == Purpose::Load { "Vazio" } else { "Vazio: começar um novo jogo" };
                        cv.text(280, y + 72, t, 0.75, DIM);
                    }
                }
            }
            let hint = if purpose == Purpose::Boot { "↑ ↓  escolher        K / ✕  confirmar" } else { "↑ ↓  escolher        K / ✕  confirmar        L / ○  voltar" };
            cv.text_centered(640, 760, &hint, 0.65, DIM);
            if let Some(k) = confirm {
                cv.rect(0, 0, ui::W as i32, ui::H as i32, 0, [0, 0, 0, 120]);
                question(&mut cv, &format!("Apagar o progresso do Slot {} e começar de novo?", k + 1), "Esta ação não pode ser desfeita.", yes);
            }
        }
    }
    if let Some(t) = saving {
        let phase = t.elapsed().as_secs_f32() * 1.2;
        let a = if t.elapsed() > Duration::from_millis(1300) { ((1600 - t.elapsed().as_millis() as i32).max(0) * 255 / 300) as u8 } else { 255 };
        cv.rect(1000, 806, 250, 60, 30, [10, 12, 20, (a as u32 * 200 / 255) as u8]);
        cv.spinner(1032, 836, 14.0, phase, [ACCENT[0], ACCENT[1], ACCENT[2], a]);
        cv.text(1058, 818, "Salvando...", 0.7, [255, 255, 255, a]);
    }
    ui::show(Some(cv));
}

pub(crate) fn question(cv: &mut Canvas, q: &str, sub: &str, yes: bool) {
    cv.rect(160, 300, 960, 290, 22, [30, 34, 50, 245]);
    let scale = if cv.text_width(q, 0.85) > 900 { 0.85 * 900.0 / cv.text_width(q, 0.85) as f32 } else { 0.85 };
    cv.text_centered(640, 340, q, scale, WHITE);
    cv.text_centered(640, 400, sub, 0.65, DIM);
    for (i, (label, on)) in [("Sim", yes), ("Não", !yes)].into_iter().enumerate() {
        let x = 420 + i as i32 * 260;
        cv.rect(x, 480, 180, 70, 16, if on { [70, 52, 28, 255] } else { [44, 48, 66, 255] });
        cv.text_centered(x + 90, 492, label, 0.85, if on { ACCENT } else { WHITE });
    }
}

/// Once per frame (with or without a flow): keeps the saving sign moving and fading.
pub fn frame(c: &mut Ctx) {
    let _ = c;
    let up = STATE.lock().unwrap().saving_since.is_some_and(|t| t.elapsed() < Duration::from_millis(1700));
    if up && STATE.lock().unwrap().picker.is_none() {
        draw(None);
    } else if !up && !modal() {
        let mut s = STATE.lock().unwrap();
        if s.saving_since.take().is_some() {
            s.shown.clear();
            ui::show(None);
        }
    }
}

/// BT3_UI_PREVIEW=<folder>: the port's screens drawn to PNGs (slot list, new game question, replay question,
/// saving sign) over a grey picture, to look at them without playing.
pub fn preview(dir: &std::path::Path) {
    let _ = std::fs::create_dir_all(dir);
    let shots: [(&str, Option<(Purpose, usize, Option<usize>, bool)>, bool); 5] = [
        ("boot", Some((Purpose::Boot, 0, None, false)), false),
        ("load", Some((Purpose::Load, 1, None, false)), false),
        ("newgame_confirm", Some((Purpose::NewGame, 0, Some(0), false)), false),
        ("replay_question", Some((Purpose::ReplayAsk, 0, None, true)), false),
        ("saving", None, true),
    ];
    for (name, list, saving) in shots {
        {
            let mut s = STATE.lock().unwrap();
            s.shown.clear();
            s.saving_since = if saving { Some(Instant::now() - Duration::from_millis(400)) } else { None };
        }
        draw(list);
        let mut pic = vec![0u8; 640 * 448 * 4];
        for (i, p) in pic.chunks_exact_mut(4).enumerate() {
            let (x, y) = (i % 640, i / 640);
            let v = (60 + (x + y) / 12 % 2 * 30) as u8;
            p.copy_from_slice(&[v, v + 10, v + 30, 255]);
        }
        ui::blend_into(&mut pic, 640, 448);
        let _ = super::gs::write_png(&dir.join(format!("{name}.png")).to_string_lossy(), 640, 448, &pic);
    }
    // the Esc question (gs/gpu.rs), over nothing
    ui::show(None);
    ui::show_question(&ui::snapshot(), "Sair do jogo?", "O progresso desde o último salvamento será perdido.", false);
    let mut pic = vec![90u8; 640 * 448 * 4];
    ui::blend_into(&mut pic, 640, 448);
    let _ = super::gs::write_png(&dir.join("quit_question.png").to_string_lossy(), 640, 448, &pic);
    // the F1 settings, both pages
    let mut m = super::settings::Menu::new();
    for name in ["settings_main", "settings_keys"] {
        ui::show(None);
        m.draw(&ui::snapshot());
        let mut pic = vec![90u8; 640 * 448 * 4];
        ui::blend_into(&mut pic, 640, 448);
        let _ = super::gs::write_png(&dir.join(format!("{name}.png")).to_string_lossy(), 640, 448, &pic);
        // to the key page: down twice, then confirm
        for b in [0u16, 1 << 6, 0, 1 << 6, 0, 1 << 14, 0] {
            m.step(b, 0, false);
        }
    }
}
