//! Two more items on the title screen: "Load Game", which opens the save slot list (host/saves.rs) to pick the
//! profile to play, and "Options", the game's options screen (host/options.rs) straight from the title. The title's
//! menu is a Flash movie with two plates (src/menu/menu_a_c.c: mc_menu_plate_1 and _2, each with a lit and an unlit
//! text clip that show one 32-pixel row of a 512 x 64 text texture). The other items are the unlit plate drawn
//! again, lower down, from its draw-over callback (FlashSlot_Draw calls it with the plate's drawing state), and
//! the plates swap roles so that the lit one is always under the cursor: the lit plate shows the item under the
//! cursor (plate 1 for New Game, plate 2 for the others, moved to the item's row), the unlit plate the first of the
//! others, its copies the rest. The four rows start one row higher than the game's two, so that they fit above
//! the bottom of the screen.
//!
//! The texts "Load Game" and "Options" are drawn by tools/assets/make_title_load.py in the style of the game's
//! texts (title_load.png: lit and unlit row of each). They become a second texture of the two text images (512 x
//! 64: Load Game, Options), in the game's own palette, so the plates pick them with texture index 1.
//!
//! Title_Init / Title_Term / Title_Draw / Title_Input are bound here (tools/translate/c2rs/link.py),
//! and TextBox_DrawClip, whose address is the callback (a draw-over callback of the battle's text boxes otherwise).
use crate::rt::prelude::*;
use std::sync::Mutex;

/// The callback's argument that means "the third plate" (never an address).
const MAGIC: u32 = 0x7E57_0001;

// Title (include/menu/menu_a.h)
const T_FLASH: u32 = 0x0C;
const T_BG: u32 = 0x38;
const T_TEX: u32 = 0x3C;
const T_CURSOR: u32 = 0xAC;
const T_COUNT: u32 = 0xB4;
const T_STATE: u32 = 0xB8;
const T_IDLE: u32 = 0xC4;
const T_CLOUD: u32 = 0xC8;
// the text images of the plates: unlit, lit
const TEX_OFF: u32 = 6;
const TEX_ON: u32 = 9;
// gPad[0]
const PAD_PRESSED: u32 = 0x18C;
const PAD_REPEAT: u32 = 0x190;
const BTN_UP: u32 = 8;
const BTN_DOWN: u32 = 4;
const BTN_OK: u32 = 0x200;
// Flash, FlashClip, FlashProp (include/sys/gfxm_c.h)
const FL_CLIPS: u32 = 0x24;
const CLIP_SIZE: u32 = 0xE4;
const CLIP_TL: u32 = 0xCC;
const PROP_SIZE: usize = 0x6C;
const PROP_X: u32 = 0x04;
const PROP_Y: u32 = 0x08;
const PROP_COLOR: u32 = 0x18;
const OV_POS: u32 = 0x02;
const OV_COLOR: u32 = 0x20;

// my block of the game's heap
const B_TAB_ON: u32 = 0x000;
const B_TAB_OFF: u32 = 0x080;
const B_REF: u32 = 0x100;
const B_UV: u32 = 0x110;
const B_POS: u32 = 0x130;
const B_PROP: u32 = 0x140;
const B_STR: u32 = 0x200;
const B_PKT_ON: u32 = 0x400;
const PKT_SIZE: u32 = 0x60 + 512 * 64;
const B_PKT_OFF: u32 = B_PKT_ON + 0x8080;
const B_SIZE: u32 = B_PKT_OFF + 0x8080;
// the items
const ITEMS: usize = 4;
const ITEM_OPTIONS: usize = 3;


const NAMES: [&str; 9] = ["mc_menu_plate_1", "mc_menu_plate_2", "mc_menu_text_off", "mc_menu_text_on", "mc_kumo_1", "mc_kumo_2", "fl_on_start", "fl_off_start", "fl_ok"];
const S_PLATE: usize = 0;
const S_TEXT_OFF: usize = 2;
const S_TEXT_ON: usize = 3;
const S_KUMO: usize = 4;
const S_ON: usize = 6;
const S_OFF: usize = 7;
const S_OK: usize = 8;

struct State {
    block: u32,
    /// the item under the cursor: 0 New Game, 1 Continue, 2 Load Game, 3 Options
    cursor: usize,
    /// the plate unlit, the item it shows, and its copies: item and how much lower (for the callback)
    copy: Option<(usize, usize, [(usize, f32); 2])>,
}

static STATE: Mutex<State> = Mutex::new(State { block: 0, cursor: 0, copy: None });

fn title(c: &Ctx) -> u32 {
    c.m.r32(sym::gTitle)
}

fn name(block: u32, k: usize) -> u32 {
    block + B_STR + 32 * k as u32
}

/// Which items can be chosen: New Game always, Continue with a save loaded, Load Game with any save, Options.
fn available(c: &Ctx) -> [bool; ITEMS] {
    let t = title(c);
    [true, c.m.r32(t + T_COUNT) >= 2, super::saves::any_saved(), true]
}

/// The plate that is lit for an item under the cursor, and the three others in order.
fn roles(cursor: usize) -> (usize, [usize; 3]) {
    let lit = if cursor == 0 { 0 } else { 1 };
    let mut others = [0; 3];
    let mut k = 0;
    for i in 0..ITEMS {
        if i != cursor {
            others[k] = i;
            k += 1;
        }
    }
    (lit, others)
}

/// The game's Title_Init, then the third item's texture and names.
pub fn Title_Init(c: &mut Ctx, section: i32) {
    F::__real_Title_Init(c, section);
    let t = title(c);
    let block = F::Heap_Alloc(c, B_SIZE as i32, 0x40, 0, 2);
    if block == 0 {
        eprintln!("bt3: no room for the title's load item");
        return;
    }
    for (k, n) in NAMES.iter().enumerate() {
        let mut b = n.as_bytes().to_vec();
        b.push(0);
        c.m.write_bytes(name(block, k), &b);
    }
    let png = super::assets::png_read(include_bytes!("title_load.png")).expect("title_load.png");
    for (img, tab, pkt, rows) in [(TEX_ON, B_TAB_ON, B_PKT_ON, [0usize, 2]), (TEX_OFF, B_TAB_OFF, B_PKT_OFF, [1usize, 3])] {
        let orig = c.m.r32(t + T_TEX + 4 * img);
        make_texture(c, orig, block + tab, block + pkt, &png, rows);
        c.m.w32(t + T_TEX + 4 * img, block + tab);
    }
    let mut s = STATE.lock().unwrap();
    s.block = block;
    s.cursor = c.m.r32(t + T_CURSOR) as usize;
    s.copy = None;
}

/// A table of two textures: the game's (entry 0) and mine (entry 1: 512 x 64, rows `rows` of the picture, 8-bit,
/// the same palette).
fn make_texture(c: &mut Ctx, orig: u32, tab: u32, pkt: u32, png: &super::assets::Png, rows: [usize; 2]) {
    let mut e = [0u8; 64];
    c.m.read_bytes(orig, &mut e);
    c.m.write_bytes(tab, &e);
    let w32 = |e: &mut [u8; 64], o: usize, v: u32| e[o..o + 4].copy_from_slice(&v.to_le_bytes());
    let r32 = |e: &[u8; 64], o: usize| u32::from_le_bytes(e[o..o + 4].try_into().unwrap());
    let mut e1 = e;
    w32(&mut e1, 0x08, PKT_SIZE); // pixSize
    w32(&mut e1, 0x10, 512 * 64 / 256); // tbpStep: GS blocks of 256 bytes
    w32(&mut e1, 0x18, 8 | 0x13 << 8); // pixBlt: DBW 8 (512 pixels), DPSM PSMT8
    let tex0 = u64::from_le_bytes(e[0x30..0x38].try_into().unwrap());
    let tex0 = (tex0 & !(15 << 30)) | 6 << 30; // TH: 64 rows
    e1[0x30..0x38].copy_from_slice(&tex0.to_le_bytes());
    w32(&mut e1, 0x38, pkt);
    c.m.write_bytes(tab + 64, &e1);
    // the upload: the game's packet header (VIF DIRECT, BITBLT registers, IMAGE tag) for 512 x 64 bytes
    let opix = r32(&e, 0x38);
    let mut h = [0u8; 0x60];
    c.m.read_bytes(opix, &mut h);
    h[0x0C..0x10].copy_from_slice(&(0x5000_0000 | (PKT_SIZE / 16 - 1)).to_le_bytes());
    h[0x30..0x34].copy_from_slice(&512u32.to_le_bytes());
    h[0x34..0x38].copy_from_slice(&64u32.to_le_bytes());
    let tag = u32::from_le_bytes(h[0x50..0x54].try_into().unwrap());
    h[0x50..0x54].copy_from_slice(&((tag & !0x7FFF) | (512 * 64 / 16)).to_le_bytes());
    c.m.write_bytes(pkt, &h);
    // the pixels: the nearest colours of the palette (256 colours, CSM1: index bits 3 and 4 swapped in memory)
    let clut = r32(&e, 0x3C) + 0x60;
    let pal: Vec<[i32; 4]> = (0..256u32)
        .map(|k| {
            let s = (k & !0x18) | (k & 8) << 1 | (k & 0x10) >> 1;
            let v = c.m.r32(clut + 4 * s);
            [(v & 255) as i32, (v >> 8 & 255) as i32, (v >> 16 & 255) as i32, (v >> 24 & 255) as i32]
        })
        .collect();
    let mut cache = std::collections::HashMap::new();
    let mut px = vec![0u8; 512 * 64];
    for y in 0..64 {
        for x in 0..512 {
            let o = ((rows[y / 32] * 32 + y % 32) * png.w as usize + x) * 4;
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

/// The game's Title_Term, then my block freed.
pub fn Title_Term(c: &mut Ctx) {
    F::__real_Title_Term(c);
    let block = std::mem::take(&mut STATE.lock().unwrap().block);
    if block != 0 {
        F::Heap_Free(c, block);
    }
}

fn find(c: &mut Ctx, block: u32, parent: Option<usize>, n: usize) -> bool {
    let flash = title(c) + T_FLASH;
    F::Flash_FindLabel(c, flash, parent.map_or(0, |p| name(block, p)), name(block, n), block + B_REF);
    c.m.r32(block + B_REF) as i32 >= 0
}

/// Shows item `item` on plate `plate`'s two text clips.
fn set_text(c: &mut Ctx, block: u32, plate: usize, item: usize) {
    let flash = title(c) + T_FLASH;
    let (row, tex) = if item >= 2 { ((item as u32 - 2) * 32, 1) } else { (item as u32 * 32, 0) };
    for (k, v) in [0u32, row, 512, row + 32].into_iter().enumerate() {
        c.m.w32(block + B_UV + 4 * k as u32, v);
    }
    for n in [S_TEXT_OFF, S_TEXT_ON] {
        if find(c, block, Some(S_PLATE + plate), n) {
            F::Flash_ClipSetUv(c, flash, block + B_REF, block + B_UV);
            F::Flash_ClipSetTex(c, flash, block + B_REF, tex);
        }
    }
}

/// Title_Draw with three items (the game's: the picture, the clouds, the two plates, the movie).
pub fn Title_Draw(c: &mut Ctx) {
    let block = STATE.lock().unwrap().block;
    if block == 0 {
        return F::__real_Title_Draw(c);
    }
    let t = title(c);
    let flash = t + T_FLASH;
    let bg = c.m.r32(t + T_BG);
    F::Sprite_DrawPicture(c, bg, 0, 0, 0x80);
    for i in 0..2u32 {
        for (k, v) in [0u32, 0, 0x200, 0x100].into_iter().enumerate() {
            c.m.w32(block + B_UV + 4 * k as u32, v);
        }
        find(c, block, None, S_KUMO + i as usize);
        let dx = if i != 0 { -1.0666667f32 } else { -0.71111111f32 };
        F::FlashAnim_Scroll(c, flash, block + B_REF, block + B_UV, t + T_CLOUD + 4 * i, 0, F32(dx.to_bits()), F32(0));
    }
    let avail = available(c);
    let cursor = STATE.lock().unwrap().cursor;
    let (lit, others) = roles(cursor);
    let unlit = 1 - lit;
    // the rows' spacing: the plates' places in the movie, without the offsets set here
    let mut ys = [0i32; 2];
    for p in 0..2 {
        if find(c, block, None, S_PLATE + p) {
            F::Flash_ClipSetOffset(c, flash, block + B_REF, 0, 0);
            F::Flash_ClipGetPos(c, flash, block + B_REF, block + B_POS, block + B_POS + 4);
            ys[p] = c.m.r32(block + B_POS + 4) as i32;
        }
    }
    let dy = ys[1] - ys[0];
    let shown = |p: usize| if p == lit { cursor } else { others[0] };
    for p in 0..2 {
        let item = shown(p);
        if find(c, block, None, S_PLATE + p) {
            // the plate moved to its item's row, the rows starting one higher than the game's
            let off = (item as i32 - p as i32 - 1) * dy;
            F::Flash_ClipSetOffset(c, flash, block + B_REF, 0, off);
            let col = if avail[item] { 1.0f32 } else { 0.5 };
            F::Flash_ClipSetColor(c, flash, block + B_REF, F32(col.to_bits()));
            let (cb, arg) = if p == unlit { (fid::TextBox_DrawClip, MAGIC) } else { (0, 0) };
            F::Flash_ClipSetCallbackC(c, flash, block + B_REF, cb, arg);
        }
        set_text(c, block, p, item);
    }
    let rel = |i: usize| (i as f32 - others[0] as f32) * dy as f32;
    STATE.lock().unwrap().copy = Some((unlit, others[0], [(others[1], rel(others[1])), (others[2], rel(others[2]))]));
    F::Flash_Draw(c, flash);
    STATE.lock().unwrap().copy = None;
}

/// The draw-over callback of the unlit plate: the plate once more for each of the other items, lower down.
fn draw_copy(c: &mut Ctx, prop: u32) {
    let (block, copy) = {
        let s = STATE.lock().unwrap();
        (s.block, s.copy)
    };
    let Some((plate, own, copies)) = copy else { return };
    let avail = available(c);
    let mut p = [0u8; PROP_SIZE];
    c.m.read_bytes(prop, &mut p);
    for (item, delta) in copies {
        c.m.write_bytes(block + B_PROP, &p);
        let flags = c.m.r32(block + B_PROP);
        let (x, y) = if flags & OV_POS != 0 {
            (c.m.r32(block + B_PROP + PROP_X), f32::from_bits(c.m.r32(block + B_PROP + PROP_Y)))
        } else {
            (0, 0.0)
        };
        c.m.w32(block + B_PROP, flags | OV_POS | OV_COLOR);
        c.m.w32(block + B_PROP + PROP_X, x);
        c.m.w32(block + B_PROP + PROP_Y, (y + delta).to_bits());
        c.m.w32(block + B_PROP + PROP_COLOR, (if avail[item] { 1.0f32 } else { 0.5 }).to_bits());
        set_text(c, block, plate, item);
        if find(c, block, None, S_PLATE + plate) {
            let clips = c.m.r32(c.m.r32(sym::gTitle) + T_FLASH + FL_CLIPS);
            let clip = c.m.r32(clips) + c.m.r32(block + B_REF) * CLIP_SIZE;
            F::FlashTl_Draw(c, clip + CLIP_TL, block + B_PROP);
        }
    }
    set_text(c, block, plate, own);
}

/// TextBox_DrawClip, or the third plate when called with MAGIC.
pub fn TextBox_DrawClip(c: &mut Ctx, draw: u32, prop: u32) -> i32 {
    if draw == MAGIC {
        draw_copy(c, prop);
        return 0;
    }
    if draw == super::options::MAGIC {
        super::options::draw_copy(c, prop);
        return 0;
    }
    F::__real_TextBox_DrawClip(c, draw, prop)
}

/// Title_Input with three items: up / down and choosing Load Game are handled here, the rest by the game's.
pub fn Title_Input(c: &mut Ctx, result: u32) {
    let block = STATE.lock().unwrap().block;
    let t = title(c);
    if block == 0 || c.m.r32(t + T_STATE) != 2 {
        F::__real_Title_Input(c, result);
        if block != 0 && c.m.r32(t + T_STATE) == 2 {
            // the menu just came up: the game's cursor (Continue with a save loaded, else New Game)
            STATE.lock().unwrap().cursor = c.m.r32(t + T_CURSOR) as usize;
        }
        return;
    }
    let pad = sym::gPad;
    let pressed = c.m.r32(pad + PAD_PRESSED);
    let repeat = c.m.r32(pad + PAD_REPEAT);
    let avail = available(c);
    let cursor = STATE.lock().unwrap().cursor;
    let step = if repeat & BTN_UP != 0 { 2 } else if repeat & BTN_DOWN != 0 { 1 } else { 0 };
    let mut mask = BTN_UP | BTN_DOWN;
    if step != 0 {
        c.m.w32(t + T_IDLE, 0);
        let step = if step == 2 { ITEMS - 1 } else { 1 };
        let mut next = cursor;
        for _ in 0..ITEMS {
            next = (next + step) % ITEMS;
            if avail[next] {
                break;
            }
        }
        if next != cursor {
            F::Title_ClipGoto(c, 0, 2, name(block, S_OFF));
            STATE.lock().unwrap().cursor = next;
            c.m.w32(t + T_CURSOR, roles(next).0 as u32);
            F::Title_ClipGoto(c, 0, 2, name(block, S_ON));
            let _ = F::Snd_PlaySe(c, 1, 0);
        }
    } else if pressed & BTN_OK != 0 && cursor == 2 {
        // Load Game: the slot list; once a profile is loaded the title leaves for the main menu as Continue
        mask |= BTN_OK;
        c.m.w32(t + T_IDLE, 0);
        F::Title_ClipGoto(c, 0, 2, name(block, S_OK));
        F::McFlow_Start(c, 1);
        F::McFlow_SetDoneCb(c, 0, fid::Title_OnCardDone, result as i32);
        F::McFlow_SetDoneCb(c, 1, fid::Title_OnCardCancel, 0);
        let _ = F::Snd_PlaySe(c, 1, 1);
    } else if pressed & BTN_OK != 0 && cursor == ITEM_OPTIONS {
        // Options: the options screen runs here, over the title (which waits), then the title goes on
        mask |= BTN_OK;
        c.m.w32(t + T_IDLE, 0);
        let _ = F::Snd_PlaySe(c, 1, 1);
        run_options(c);
        F::Title_ClipGoto(c, 0, 2, name(block, S_ON));
    }
    c.m.w32(pad + PAD_PRESSED, pressed & !mask);
    c.m.w32(pad + PAD_REPEAT, repeat & !mask);
    F::__real_Title_Input(c, result);
    c.m.w32(pad + PAD_PRESSED, pressed);
    c.m.w32(pad + PAD_REPEAT, repeat);
}

/// The options screen from the title, as the main menu's mode 62 runs it (src/menu/menu_x_b.c OptMode_Main): its
/// archive loaded, its music, its own frame loop (Option_Run), then the title's music and a fade back in.
fn run_options(c: &mut Ctx) {
    const BGM_TITLE: i32 = 0x10B16;
    const BGM_OPTION: i32 = 0x10B18;
    F::Adx_StopAll(c);
    let base = c.m.r32(c.m.r32(sym::gProgress) + 4) as i32;
    if c.m.r32(sym::gMenuArc10) == 0 {
        let a = F::File_LoadSync(c, base + 5, 0, 0);
        c.m.w32(sym::gMenuArc10, a);
    }
    if c.m.r32(sym::gMenuArc10) == 0 {
        eprintln!("bt3: no room for the options screen over the title");
        return;
    }
    F::Bgm_Play(c, BGM_OPTION);
    // the options screen sets up a save flow of its own (Option_Init: McFlow_Init) and frees it when it closes
    // (McFlow_Term: gMcFlow = NULL), which took the title's away: New Game then saved but never left the title.
    // The title's flow is put back afterwards.
    let title_flow = c.m.r32(sym::gMcFlow);
    super::options::set_from_title(true);
    F::Option_Run(c, 1);
    super::options::set_from_title(false);
    c.m.w32(sym::gMcFlow, title_flow);
    F::Adx_StopAll(c);
    let a10 = c.m.r32(sym::gMenuArc10);
    if a10 != 0 {
        F::Heap_Free(c, a10);
        c.m.w32(sym::gMenuArc10, 0);
    }
    F::Bgm_Play(c, BGM_TITLE);
    F::ColorFade_StartIn(c, 0, 0, 0, 20);
}
