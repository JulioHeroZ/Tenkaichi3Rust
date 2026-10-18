//! The game's options screen (src/menu/menu_x_c.c, mode 62) with the PC's own page, "Graphics": resolution,
//! aspect ratio, texture quality and full screen (host/settings.rs), in the screen's own style.
//!
//! * The top page gets a fifth row, "Graphics", in the empty place between Controller and EXIT. The movie has four
//!   row plates there; the fifth is a plate drawn once more, one row lower, from its draw-over callback (the way
//!   host/title.rs adds Load Game). With the cursor on Graphics the game's cursor stays on Controller (plate 4 lit)
//!   and that plate moves one row down and shows "Graphics", while the copy, drawn at row 4, shows "Controller".
//! * Choosing it opens a page made from the Screen page's movie (page 1: three row plates, a hidden fourth, the
//!   bottom plate "Default"), its four rows showing the settings with their values; left / right (or confirm)
//!   change a value, which applies at once; Default restores the defaults; back returns to the top page.
//! * The texts (options_labels.png, tools/assets/make_option_labels.py) are drawn in the style of the game's
//!   plate texts; each row is a setting with one of its values. They become two more textures of the plates'
//!   text images (lit and off), in the game's own palettes, which the plates pick with texture index 2 / 3.
//!
//! Option_Init / Option_Input / Option_Term and Flash_Draw are bound here (tools/translate/c2rs/link.py); the
//! page's title in the icon window is mine too (options_header.png).
#![allow(non_snake_case)]
use crate::gen::fid;
use crate::gen::funcs as F;
use crate::gen::sym;
use crate::host::settings;
use crate::rt::prelude::*;
use std::sync::Mutex;

/// The callback's argument that means "the options screen's extra plate" (never an address).
pub const MAGIC: u32 = 0x7E57_0002;

// Option (include/menu/menu_x.h)
const O_FLASH: u32 = 0x08;
const O_TEX: u32 = 0x38;
const O_CURSOR: u32 = 0x104;
const O_STATE: u32 = 0x108;
const O_PAGE: u32 = 0x10C;
const O_VOICE: u32 = 0x12C;
const O_FROM: u32 = 0x13C;
// MFlash.flags: the movie accepts input
const FL_FLAGS: u32 = 0x08;
const MFLASH_PAD: u32 = 2;
// the slots of Option.tex holding the plates' text images: off (#0, #1 of section 16) and lit (#2, #3)
const TEX_OFF: u32 = 3;
const TEX_ON: u32 = 7;
// states, rows and clips (menu_x.h)
const ST_TOP: u32 = 0;
const ST_SCREEN: u32 = 2;
const ITEM_CTRL: u32 = 4;
const ITEM_EXIT: u32 = 5;
const CLIP_ROW: i32 = 0;
const CLIP_ROW_SCREEN: i32 = 1;
const CLIP_BOTTOM: i32 = 3;
// gPad[0]
const PAD_PRESSED: u32 = 0x18C;
const PAD_REPEAT: u32 = 0x190;
const B_LEFT: u32 = 1;
const B_RIGHT: u32 = 2;
const B_DOWN: u32 = 4;
const B_UP: u32 = 8;
const B_CROSS: u32 = 0x200;
const B_TRIANGLE: u32 = 0x400;
const B_ALL: u32 = B_LEFT | B_RIGHT | B_DOWN | B_UP | B_CROSS | B_TRIANGLE | 0x1000 | 0x800 | 0x100;
// Flash, FlashClip, FlashProp (include/sys/gfxm_c.h), as host/title.rs
const FL_CLIPS: u32 = 0x24;
const CLIP_SIZE: u32 = 0xE4;
const CLIP_TL: u32 = 0xCC;
const PROP_SIZE: usize = 0x6C;
const PROP_X: u32 = 0x04;
const PROP_Y: u32 = 0x08;
const OV_POS: u32 = 0x02;

// my block of the game's heap
const B_REF: u32 = 0x000;
const B_UV: u32 = 0x010;
const B_POS: u32 = 0x030;
const B_PROP: u32 = 0x040;
const B_STR: u32 = 0x100;
const B_TAB_OFF: u32 = 0x600;
const B_TAB_ON: u32 = 0x700;
const B_PKT: u32 = 0x800;
const TEX_H: u32 = 256;
const PKT_SIZE: u32 = 0x60 + 512 * TEX_H;
const PKT_STEP: u32 = (PKT_SIZE + 0x7F) & !0x7F;
const B_HDR: u32 = B_PKT + 4 * PKT_STEP; // the page title's texture entry, then its packet
const B_HDR_PKT: u32 = B_HDR + 0x80;
const B_SIZE: u32 = B_HDR_PKT + 0x60 + 512 * 64;
// IconWin (include/battle/view_a.h): the icons' resource, the shown icon
const IW_ICONS: u32 = 0x2C;
const IW_TEX4: u32 = 0x40;
const ICON_SCREEN: u32 = 2; // the Screen page's title: texture 1 + 1 of the icons

const NAMES: [&str; 14] = [
    "mc_menu_plate_1", "mc_menu_plate_2", "mc_menu_plate_3", "mc_menu_plate_4", "mc_menu_plate_5", "mc_menu_plate_6",
    "mc_menu_plate_7", "mc_menu_plate_8", "mc_menu_text_off", "mc_menu_text_on", "fl_on_start", "fl_off_start",
    "fl_menu_change", "mc_bottom_plate_1",
];
const S_PLATE: usize = 0; // + plate - 1
const S_TEXT_OFF: usize = 8;
const S_TEXT_ON: usize = 9;
const S_ON: usize = 10;
const S_OFF: usize = 11;
const S_CHANGE: usize = 12;
const S_BOTTOM: usize = 13;

// the rows of options_labels.png (tools/assets/make_option_labels.py)
const ROW_GRAPHICS: u32 = 0;
const ROW_RESOLUTION: u32 = 1;
const ROW_ASPECT: u32 = 5;
const ROW_TEXTURES: u32 = 9;
const ROW_DISPLAY: u32 = 13;

struct State {
    block: u32,
    /// top page: the cursor is on Graphics (the game's cursor on Controller)
    gfx_row: bool,
    /// the Graphics page is open (the game's page 1, state 2)
    gfx_page: bool,
    /// the top page came back from the Graphics page (its outgoing plates show the Graphics rows)
    from_gfx: bool,
    /// Graphics page: the row under the cursor, 0..3 settings, 4 Default
    row: u32,
    /// the message window was closed for the Graphics row
    msg_closed: bool,
    /// the extra plate: which plate is drawn again, how much lower, with which text (texture index, row)
    copy: Option<(u32, f32, (i32, u32))>,
}

/// The screen was opened from the title (host/title.rs): no profile may be loaded yet, so leaving it does not run
/// the game's save flow (the game's settings stay in the save data in memory; mine are in settings.ini).
static FROM_TITLE: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

pub fn set_from_title(on: bool) {
    FROM_TITLE.store(on, std::sync::atomic::Ordering::Relaxed);
}

const O_DIRTY: u32 = 0x198;

static STATE: Mutex<State> = Mutex::new(State { block: 0, gfx_row: false, gfx_page: false, from_gfx: false, row: 0, msg_closed: false, copy: None });

fn opt(c: &Ctx) -> u32 {
    c.m.r32(sym::gOption)
}

fn flash(c: &Ctx) -> u32 {
    opt(c) + O_FLASH
}

fn name(block: u32, k: usize) -> u32 {
    block + B_STR + 32 * k as u32
}

/// The label row of a Graphics page row (0..3) for the current settings.
fn setting_row(i: u32) -> u32 {
    let st = settings::get();
    match i {
        0 => ROW_RESOLUTION + settings::RESOLUTIONS.iter().position(|r| r.0 == st.resolution).unwrap_or(1) as u32,
        1 => ROW_ASPECT + settings::ASPECTS.iter().position(|a| a.0 == settings::aspect_milli()).unwrap_or(2) as u32,
        2 => ROW_TEXTURES + st.textures.clamp(0, 3) as u32,
        _ => ROW_DISPLAY + st.fullscreen as u32,
    }
}

/// A texture of the label rows `row0..row0 + 8`, in the palette of the game's texture `orig` (its entry copied to
/// `dst`, its upload packet made at `pkt`): 512 x 256, 8-bit (as host/title.rs make_texture).
fn make_texture(c: &mut Ctx, orig: u32, dst: u32, pkt: u32, png: &super::assets::Png, row0: usize) {
    make_texture_h(c, orig, dst, pkt, png, row0 * 32, TEX_H);
}

/// The same for `h` rows of pixels of the picture from pixel row `y0` (h = 2^n).
fn make_texture_h(c: &mut Ctx, orig: u32, dst: u32, pkt: u32, png: &super::assets::Png, y0: usize, h: u32) {
    let pkt_size = 0x60 + 512 * h;
    let mut e = [0u8; 64];
    c.m.read_bytes(orig, &mut e);
    let w32 = |e: &mut [u8; 64], o: usize, v: u32| e[o..o + 4].copy_from_slice(&v.to_le_bytes());
    let r32 = |e: &[u8; 64], o: usize| u32::from_le_bytes(e[o..o + 4].try_into().unwrap());
    let mut e1 = e;
    w32(&mut e1, 0x08, pkt_size); // pixSize
    w32(&mut e1, 0x10, 512 * h / 256); // tbpStep: GS blocks of 256 bytes
    w32(&mut e1, 0x18, 8 | 0x13 << 8); // pixBlt: DBW 8 (512 pixels), DPSM PSMT8
    let tex0 = u64::from_le_bytes(e[0x30..0x38].try_into().unwrap());
    let tex0 = (tex0 & !(15 << 30) & !(15 << 26)) | (h.trailing_zeros() as u64) << 30 | 9 << 26; // 512 x h
    e1[0x30..0x38].copy_from_slice(&tex0.to_le_bytes());
    w32(&mut e1, 0x38, pkt);
    c.m.write_bytes(dst, &e1);
    let opix = r32(&e, 0x38);
    let mut hd = [0u8; 0x60];
    c.m.read_bytes(opix, &mut hd);
    hd[0x0C..0x10].copy_from_slice(&(0x5000_0000 | (pkt_size / 16 - 1)).to_le_bytes());
    hd[0x30..0x34].copy_from_slice(&512u32.to_le_bytes());
    hd[0x34..0x38].copy_from_slice(&h.to_le_bytes());
    let tag = u32::from_le_bytes(hd[0x50..0x54].try_into().unwrap());
    hd[0x50..0x54].copy_from_slice(&((tag & !0x7FFF) | (512 * h / 16)).to_le_bytes());
    c.m.write_bytes(pkt, &hd);
    let clut = r32(&e, 0x3C) + 0x60;
    let pal: Vec<[i32; 4]> = (0..256u32)
        .map(|k| {
            let s = (k & !0x18) | (k & 8) << 1 | (k & 0x10) >> 1;
            let v = c.m.r32(clut + 4 * s);
            [(v & 255) as i32, (v >> 8 & 255) as i32, (v >> 16 & 255) as i32, (v >> 24 & 255) as i32]
        })
        .collect();
    let mut cache = std::collections::HashMap::new();
    let mut px = vec![0u8; (512 * h) as usize];
    for y in 0..h as usize {
        for x in 0..512 {
            let o = ((y0 + y) * png.w as usize + x) * 4;
            let p = &png.data[o..o + 4];
            let key = u32::from_le_bytes(p.try_into().unwrap());
            px[y * 512 + x] = *cache.entry(key).or_insert_with(|| {
                let a = (p[3] as i32 * 128 + 127) / 255;
                (0..256)
                    .min_by_key(|&k| {
                        let q = pal[k];
                        let dc: i32 = (0..3).map(|i| (q[i] - p[i] as i32).pow(2)).sum();
                        dc * a.min(q[3]) / 128 + 16 * (q[3] - a).pow(2)
                    })
                    .unwrap() as u8
            });
        }
    }
    c.m.write_bytes(pkt + 0x60, &px);
}

/// The game's Option_Init, then the labels' textures and names.
pub fn Option_Init(c: &mut Ctx, section: i32) -> i32 {
    let r = F::__real_Option_Init(c, section);
    let o = opt(c);
    *STATE.lock().unwrap() = State { block: 0, gfx_row: false, gfx_page: false, from_gfx: false, row: 0, msg_closed: false, copy: None };
    let block = F::Heap_Alloc(c, B_SIZE as i32, 0x80, 0, 2);
    if block == 0 {
        eprintln!("bt3: no room for the options screen's Graphics page");
        return r;
    }
    for (k, n) in NAMES.iter().enumerate() {
        let mut b = n.as_bytes().to_vec();
        b.push(0);
        c.m.write_bytes(name(block, k), &b);
    }
    let png = super::assets::png_read(include_bytes!("options_labels.png")).expect("options_labels.png");
    // the tables: the game's two textures of the image, then mine (rows 0..7, 8..15)
    for (slot, tab, png_base, k) in [(TEX_OFF, B_TAB_OFF, 16usize, 0u32), (TEX_ON, B_TAB_ON, 0usize, 2u32)] {
        let orig = c.m.r32(o + O_TEX + 4 * slot);
        let mut e = [0u8; 128];
        c.m.read_bytes(orig, &mut e);
        c.m.write_bytes(block + tab, &e);
        for half in 0..2u32 {
            let pkt = block + B_PKT + (k + half) * PKT_STEP;
            make_texture(c, orig, block + tab + 128 + 64 * half, pkt, &png, png_base + 8 * half as usize);
        }
        c.m.w32(o + O_TEX + 4 * slot, block + tab);
    }
    // the page title "Graphics", in the palette of the Screen page's
    let iw = c.m.r32(sym::gIconWin);
    if iw != 0 {
        let icons = c.m.r32(iw + IW_ICONS);
        let orig = c.m.r32(icons + 0x10) + 64 * ICON_SCREEN;
        let hdr = super::assets::png_read(include_bytes!("options_header.png")).expect("options_header.png");
        make_texture_h(c, orig, block + B_HDR, block + B_HDR_PKT, &hdr, 0, 64);
    }
    STATE.lock().unwrap().block = block;
    r
}

/// The game's Option_Term, then my block freed.
pub fn Option_Term(c: &mut Ctx) {
    F::__real_Option_Term(c);
    let block = std::mem::take(&mut STATE.lock().unwrap().block);
    if block != 0 {
        F::Heap_Free(c, block);
    }
}

fn find(c: &mut Ctx, block: u32, parent: Option<usize>, n: usize) -> bool {
    let fl = flash(c);
    F::Flash_FindLabel(c, fl, parent.map_or(0, |p| name(block, p)), name(block, n), block + B_REF);
    c.m.r32(block + B_REF) as i32 >= 0
}

/// Plate `plate` (1..8) shows texture `tex`'s row `row` (32 pixels high, 512 wide) on its two text clips.
fn set_text(c: &mut Ctx, block: u32, plate: u32, (tex, row): (i32, u32)) {
    let fl = flash(c);
    for (k, v) in [0u32, row * 32, 512, row * 32 + 32].into_iter().enumerate() {
        c.m.w32(block + B_UV + 4 * k as u32, v);
    }
    for n in [S_TEXT_OFF, S_TEXT_ON] {
        if find(c, block, Some(S_PLATE + plate as usize - 1), n) {
            F::Flash_ClipSetUv(c, fl, block + B_REF, block + B_UV);
            F::Flash_ClipSetTex(c, fl, block + B_REF, tex);
        }
    }
}

/// One of my label rows as (texture index, row of that texture).
fn label(row: u32) -> (i32, u32) {
    (2 + (row / 8) as i32, row % 8)
}

fn plate_ref(c: &mut Ctx, block: u32, plate: u32) -> bool {
    find(c, block, None, S_PLATE + plate as usize - 1)
}

/// What the screen shows of my rows, set just before its movie is drawn (Option_Draw's last step).
fn before_draw(c: &mut Ctx) {
    let (block, gfx_row, gfx_page, from_gfx) = {
        let s = STATE.lock().unwrap();
        (s.block, s.gfx_row, s.gfx_page, s.from_gfx)
    };
    if block == 0 {
        return;
    }
    let o = opt(c);
    let fl = flash(c);
    let page = c.m.r32(o + O_PAGE);
    let cursor = c.m.r32(o + O_CURSOR);
    // no offsets or callbacks left from the last frame
    for p in 1..=4 {
        if plate_ref(c, block, p) {
            F::Flash_ClipSetOffset(c, fl, block + B_REF, 0, 0);
            F::Flash_ClipSetCallbackC(c, fl, block + B_REF, 0, 0);
        }
    }
    if find(c, block, None, S_BOTTOM) {
        F::Flash_ClipSetOffset(c, fl, block + B_REF, 0, 0);
    }
    STATE.lock().unwrap().copy = None;
    if page == 0 {
        // the row spacing: plates 3 and 4 where the movie puts them
        let mut ys = [0f32; 2];
        for (i, p) in [3u32, 4].into_iter().enumerate() {
            if plate_ref(c, block, p) {
                F::Flash_ClipGetPos(c, fl, block + B_REF, block + B_POS, block + B_POS + 4);
                ys[i] = c.m.r32(block + B_POS + 4) as i32 as f32;
            }
        }
        let dy = ys[1] - ys[0];
        if dy.abs() < 4.0 {
            return;
        }
        // EXIT a little lower, so that the fifth row fits between it and Controller
        if find(c, block, None, S_BOTTOM) {
            F::Flash_ClipGetPos(c, fl, block + B_REF, block + B_POS, block + B_POS + 4);
            let y = c.m.r32(block + B_POS + 4) as i32 as f32;
            let shift = (ys[1] + 2.0 * dy - y).max(0.0);
            F::Flash_ClipSetOffset(c, fl, block + B_REF, 0, shift as i32);
        }
        let copy = if gfx_row {
            // plate 4 (lit) one row lower with Graphics; plate 3 again at row 4 with Controller
            if plate_ref(c, block, 4) {
                F::Flash_ClipSetOffset(c, fl, block + B_REF, 0, dy as i32);
            }
            set_text(c, block, 4, label(ROW_GRAPHICS));
            (3u32, dy, (0, 3u32))
        } else {
            // an unlit plate again at row 5 with Graphics
            let p = if cursor == 4 { 3 } else { 4 };
            (p, (5 - p) as f32 * dy, label(ROW_GRAPHICS))
        };
        if plate_ref(c, block, copy.0) {
            F::Flash_ClipSetCallbackC(c, fl, block + B_REF, fid::TextBox_DrawClip, MAGIC);
        }
        STATE.lock().unwrap().copy = Some(copy);
        if from_gfx {
            // the outgoing plates of the page change show the Graphics page's rows
            for i in 0..4u32 {
                set_text(c, block, 5 + i, label(setting_row(i)));
            }
            if plate_ref(c, block, 8) {
                F::Flash_ClipSetFlags(c, fl, block + B_REF, 2, 1);
            }
        }
    } else if page == 1 && gfx_page {
        for i in 0..4u32 {
            set_text(c, block, 1 + i, label(setting_row(i)));
        }
        if plate_ref(c, block, 4) {
            F::Flash_ClipSetFlags(c, fl, block + B_REF, 2, 1);
        }
    }
}

/// The game's Flash_Draw; for the options screen's movie my rows are set first.
pub fn Flash_Draw(c: &mut Ctx, fl: u32) {
    let o = opt(c);
    let (block, gfx_page) = {
        let s = STATE.lock().unwrap();
        (s.block, s.gfx_page)
    };
    if o != 0 && fl == o + O_FLASH && block != 0 {
        before_draw(c);
    }
    // the icon window over the Graphics page: its title (Option_Draw has just set the Screen page's)
    if o != 0 && block != 0 && gfx_page && fl == c.m.r32(sym::gIconWin) {
        c.m.w32(fl + IW_TEX4, block + B_HDR);
    }
    F::__real_Flash_Draw(c, fl);
}

/// The draw-over callback of the plate chosen in `before_draw`: the plate once more, lower, with another text.
pub fn draw_copy(c: &mut Ctx, prop: u32) {
    let (block, copy) = {
        let s = STATE.lock().unwrap();
        (s.block, s.copy)
    };
    let Some((plate, delta, text)) = copy else { return };
    if block == 0 {
        return;
    }
    let mut p = [0u8; PROP_SIZE];
    c.m.read_bytes(prop, &mut p);
    c.m.write_bytes(block + B_PROP, &p);
    let flags = c.m.r32(block + B_PROP);
    let (x, y) = if flags & OV_POS != 0 {
        (c.m.r32(block + B_PROP + PROP_X), f32::from_bits(c.m.r32(block + B_PROP + PROP_Y)))
    } else {
        (0, 0.0)
    };
    c.m.w32(block + B_PROP, flags | OV_POS);
    c.m.w32(block + B_PROP + PROP_X, x);
    c.m.w32(block + B_PROP + PROP_Y, (y + delta).to_bits());
    // the plate's own text, put back after (the game set it this frame: a row of texture 0)
    let own = (0, plate - 1);
    set_text(c, block, plate, text);
    if plate_ref(c, block, plate) {
        let clips = c.m.r32(flash(c) + FL_CLIPS);
        let clip = c.m.r32(clips) + c.m.r32(block + B_REF) * CLIP_SIZE;
        F::FlashTl_Draw(c, clip + CLIP_TL, block + B_PROP);
    }
    set_text(c, block, plate, own);
}

fn plate_goto(c: &mut Ctx, clip: i32, label: usize) {
    let block = STATE.lock().unwrap().block;
    F::Option_PlateGoto(c, 0, clip, name(block, label));
}

/// Graphics page: plate of row `row` (0..3, 4 = Default) to a label.
fn row_goto(c: &mut Ctx, row: u32, label: usize) {
    let o = opt(c);
    if row < 4 {
        c.m.w32(o + O_CURSOR, 7 + row);
        plate_goto(c, CLIP_ROW_SCREEN, label);
    } else {
        c.m.w32(o + O_CURSOR, 10);
        plate_goto(c, CLIP_BOTTOM, label);
    }
}

/// A setting of the Graphics page one step on (`d` = 1) or back (-1).
fn change(row: u32, d: i32) {
    let mut st = settings::get();
    let step = |i: usize, n: usize| ((i as i32 + d).rem_euclid(n as i32)) as usize;
    match row {
        0 => {
            let i = settings::RESOLUTIONS.iter().position(|r| r.0 == st.resolution).unwrap_or(1);
            st.resolution = settings::RESOLUTIONS[step(i, 4)].0;
        }
        1 => {
            let i = settings::ASPECTS.iter().position(|a| a.0 == settings::aspect_milli()).unwrap_or(2);
            st.aspect = settings::ASPECTS[step(i, 4)].0;
        }
        2 => st.textures = step(st.textures.clamp(0, 3) as usize, 4) as i32,
        _ => st.fullscreen = !st.fullscreen,
    }
    settings::set(st);
}

/// Option_Input with the Graphics row and page: what concerns them is handled here (and hidden from the game's
/// input), the rest by the game's.
pub fn Option_Input(c: &mut Ctx) -> i32 {
    let o = opt(c);
    if FROM_TITLE.load(std::sync::atomic::Ordering::Relaxed) && o != 0 {
        c.m.w32(o + O_DIRTY, 0);
    }
    let block = STATE.lock().unwrap().block;
    if block == 0 || c.m.r32(flash(c) + FL_FLAGS) & MFLASH_PAD == 0 {
        return F::__real_Option_Input(c);
    }
    let pad = sym::gPad;
    let pressed = c.m.r32(pad + PAD_PRESSED);
    let repeat = c.m.r32(pad + PAD_REPEAT);
    let state = c.m.r32(o + O_STATE);
    let cursor = c.m.r32(o + O_CURSOR);
    let (gfx_row, gfx_page, row) = {
        let s = STATE.lock().unwrap();
        (s.gfx_row, s.gfx_page, s.row)
    };
    let mut mask = 0u32;
    let mut after_gfx_row = None;
    if gfx_page && state == ST_SCREEN && c.m.r32(o + O_PAGE) == 1 {
        mask = B_ALL;
        if repeat & (B_UP | B_DOWN) != 0 {
            let next = if repeat & B_UP != 0 { (row + 4) % 5 } else { (row + 1) % 5 };
            row_goto(c, row, S_OFF);
            row_goto(c, next, S_ON);
            STATE.lock().unwrap().row = next;
            let _ = F::Snd_PlaySe(c, 1, 0);
        } else if pressed & (B_LEFT | B_RIGHT) != 0 && row < 4 {
            change(row, if pressed & B_LEFT != 0 { -1 } else { 1 });
            let _ = F::Snd_PlaySe(c, 1, 0);
        } else if pressed & B_CROSS != 0 {
            if row < 4 {
                change(row, 1);
                let _ = F::Snd_PlaySe(c, 1, 0);
            } else {
                let keys = settings::get().keys;
                settings::set(settings::Settings { keys, ..Default::default() });
                let _ = F::Snd_PlaySe(c, 1, 1);
            }
        } else if pressed & B_TRIANGLE != 0 {
            // back to the top page, the cursor on Graphics (as the game's way back from the Screen page)
            row_goto(c, row, S_OFF);
            plate_goto(c, CLIP_BOTTOM, S_OFF);
            c.m.w32(o + O_STATE, ST_TOP);
            c.m.w32(o + O_CURSOR, ITEM_CTRL);
            c.m.w32(o + O_VOICE, ITEM_CTRL);
            c.m.w32(o + O_PAGE, 0);
            c.m.w32(o + O_FROM, 1);
            plate_goto(c, CLIP_ROW, S_ON);
            let fl = flash(c);
            F::Flash_GotoLabel(c, fl, name(block, S_CHANGE), 1);
            let _ = F::Snd_PlaySe(c, 1, 2);
            let mut s = STATE.lock().unwrap();
            s.gfx_page = false;
            s.gfx_row = true;
            s.from_gfx = true;
        }
    } else if state == ST_TOP && c.m.r32(o + O_PAGE) == 0 {
        if gfx_row {
            if repeat & B_UP != 0 {
                mask |= B_UP | B_DOWN;
                STATE.lock().unwrap().gfx_row = false;
                let _ = F::Snd_PlaySe(c, 1, 0);
            } else if repeat & B_DOWN != 0 || pressed & B_TRIANGLE != 0 {
                // the game's: from Controller to EXIT
                STATE.lock().unwrap().gfx_row = false;
            } else if pressed & B_CROSS != 0 {
                // the Graphics page, as the game opens the Screen page
                mask |= B_ALL;
                let fl = flash(c);
                F::Flash_GotoLabel(c, fl, name(block, S_CHANGE), 1);
                plate_goto(c, CLIP_ROW, S_OFF);
                c.m.w32(o + O_FROM, 0);
                c.m.w32(o + O_PAGE, 1);
                c.m.w32(o + O_STATE, ST_SCREEN);
                row_goto(c, 0, S_ON);
                let _ = F::Snd_PlaySe(c, 1, 1);
                let mut s = STATE.lock().unwrap();
                s.gfx_page = true;
                s.row = 0;
                s.from_gfx = false;
            } else {
                mask |= B_CROSS;
            }
        } else if cursor == ITEM_CTRL && repeat & B_DOWN != 0 {
            mask |= B_UP | B_DOWN;
            STATE.lock().unwrap().gfx_row = true;
            let _ = F::Snd_PlaySe(c, 1, 0);
        } else if cursor == ITEM_EXIT && repeat & B_UP != 0 {
            // the game's: from EXIT to Controller, shown as Graphics
            after_gfx_row = Some(true);
        }
    }
    if c.m.r32(o + O_PAGE) != 0 && !STATE.lock().unwrap().gfx_page {
        STATE.lock().unwrap().from_gfx = false;
    }
    c.m.w32(pad + PAD_PRESSED, pressed & !mask);
    c.m.w32(pad + PAD_REPEAT, repeat & !mask);
    let r = F::__real_Option_Input(c);
    c.m.w32(pad + PAD_PRESSED, pressed);
    c.m.w32(pad + PAD_REPEAT, repeat);
    if let Some(v) = after_gfx_row {
        STATE.lock().unwrap().gfx_row = v;
    }
    // the message window has no line for the Graphics row: closed while the cursor is on it
    let on_gfx = {
        let s = STATE.lock().unwrap();
        s.gfx_row && !s.gfx_page
    };
    let mut s = STATE.lock().unwrap();
    if on_gfx != s.msg_closed && !s.gfx_page {
        s.msg_closed = on_gfx;
        drop(s);
        if on_gfx { F::MsgWin_Close(c) } else { F::MsgWin_Open(c) }
    }
    r
}
