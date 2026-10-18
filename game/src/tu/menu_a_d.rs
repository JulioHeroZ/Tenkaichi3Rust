#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn ModeMenu_GetLine(c: &mut Ctx, mut p_menu_1: i32, mut p_item_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ModeMenu_GetLine", &PROBE_N, &[(p_menu_1) as u32, (p_item_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_line_3: i32 = 0;
    let mut l_i_4: i32 = 0;
    let r_ = 'f: {
        rt::memset(c, fp, 0, 32);
        { let v_ = 3i32; c.m.w32(fp.wrapping_add(0), (v_) as u32); }
        { let v_ = 4i32; c.m.w32(fp.wrapping_add(4), (v_) as u32); }
        { let v_ = 7i32; c.m.w32(fp.wrapping_add(8), (v_) as u32); }
        { let v_ = 5i32; c.m.w32(fp.wrapping_add(12), (v_) as u32); }
        { let v_ = 16i32; c.m.w32(fp.wrapping_add(16), (v_) as u32); }
        { let v_ = 5i32; c.m.w32(fp.wrapping_add(20), (v_) as u32); }
        { let v_ = 4i32; c.m.w32(fp.wrapping_add(24), (v_) as u32); }
        { let v_ = 4i32; c.m.w32(fp.wrapping_add(28), (v_) as u32); }
        l_line_3 = 0i32;
        {
            l_i_4 = 0i32;
            'l1: loop {
                if !(((((l_i_4) < (p_menu_1)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let r_ = (c.m.r32((fp).wrapping_add((((l_i_4) as u32)).wrapping_mul(4))) as i32); let n_ = (l_line_3).wrapping_add(r_); l_line_3 = n_; };
                }
                { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
            }
        }
        { let r_ = p_item_2; let n_ = (l_line_3).wrapping_add(r_); l_line_3 = n_; };
        's3: {
            let sv5: i32 = l_line_3;
            match sv5 {
                20i32 => {
                    l_line_3 = 21i32;
                    break 's3;
                }
                21i32 => {
                    l_line_3 = 22i32;
                    break 's3;
                }
                22i32 => {
                    l_line_3 = 23i32;
                    break 's3;
                }
                23i32 => {
                    l_line_3 = 20i32;
                    break 's3;
                }
                _ => {
                }
            }
        }
        break 'f l_line_3;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "ModeMenu_GetLine", &PROBE_N, &[], true); }
    r_
}

pub fn ModeMenu_UpdateImage(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ModeMenu_UpdateImage", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        's1: {
            let sv1: i32 = { let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(396); (c.m.r32(a_) as i32) };
            match sv1 {
                5i32 => {
                    { let v_ = 6i32; let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(396); c.m.w32(a_, (v_) as u32); };
                    break 's1;
                }
                6i32 => {
                    if (((({ let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(244); (c.m.r32(a_) as i32) }) & (64i32))) != 0) {
                        { let r_ = 64i32; let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(244); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
                    }
                    { let v_ = 1i32; let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(396); c.m.w32(a_, (v_) as u32); };
                    break 's1;
                }
                1i32 => {
                    F::File_CancelRequests(c);
                    { let _ = { let a0_ = ({ let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(272); (c.m.r32(a_) as i32) }).wrapping_add({ let a_ = ((c.m.r32(sym::gModeMenu)).wrapping_add(296)).wrapping_add(((({ let a_ = ((c.m.r32(sym::gModeMenu)).wrapping_add(248)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }); let a1_ = { let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(16); c.m.r32(a_) }; let _ = 114688i32; F::File_Request(c, a0_, a1_) }; };
                    { let v_ = 2i32; let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(396); c.m.w32(a_, (v_) as u32); };
                    break 's1;
                }
                2i32 => {
                    if ((F::File_UpdateRequests(c)) != 0) {
                        { let v_ = 3i32; let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(396); c.m.w32(a_, (v_) as u32); };
                    }
                    break 's1;
                }
                3i32 => {
                    { let _ = { let a0_ = { let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(16); c.m.r32(a_) }; let a1_ = { let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(20); c.m.r32(a_) }; F::Sprite_Unpack(c, a0_, a1_, ((0i32) as u32)) }; };
                    { let v_ = { let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(20); c.m.r32(a_) }; let a_ = fp; c.m.w32(a_, v_); };
                    { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gModeMenu)).wrapping_add(80)).wrapping_add((((9i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let r_ = 128i32; let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(244); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    { let v_ = 4i32; let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(396); c.m.w32(a_, (v_) as u32); };
                    break 's1;
                }
                4i32 => {
                    if (((({ let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(244); (c.m.r32(a_) as i32) }) & (64i32))) != 0) {
                        { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gModeMenu)).wrapping_add(80)).wrapping_add((((9i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                        { let v_ = 6i32; let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(396); c.m.w32(a_, (v_) as u32); };
                    }
                    break 's1;
                }
                _ => {
                }
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "ModeMenu_UpdateImage", &PROBE_N, &[], true); }
    r_
}

pub fn ModeMenu_ChangeImage(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ModeMenu_ChangeImage", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if (((({ let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(244); (c.m.r32(a_) as i32) }) & (128i32))) != 0) {
        { let r_ = 128i32; let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(244); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
    } else {
        { let v_ = 5i32; let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(396); c.m.w32(a_, (v_) as u32); };
    }
    { let r_ = 64i32; let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(244); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ModeMenu_ChangeImage", &PROBE_N, &[], true); }
    r_
}

pub fn ModeMenu_Init(c: &mut Ctx, mut p_section_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ModeMenu_Init", &PROBE_N, &[(p_section_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_i_2: i32 = 0;
    let mut l_slot_3: u32 = 0;
    let r_ = 'f: {
        { let v_ = ((0i32) as u32); c.m.w32(fp, v_); }
        { let v_ = F::Heap_Alloc(c, 828i32, ((32i32) as u32), 0i32, 2i32); let a_ = sym::gModeMenu; c.m.w32(a_, v_); };
        { let _ = { let a0_ = c.m.r32(sym::gModeMenu); F::memset(c, a0_, 0i32, ((828i32) as u32)) }; };
        { let v_ = (c.m.r32(sym::gMenuArc2)).wrapping_add((({ let a_ = (c.m.r32(sym::gMenuArc2)).wrapping_add((((p_section_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = c.m.r32(sym::gModeMenu); c.m.w32(a_, v_); };
        { let v_ = { let a0_ = { let a_ = c.m.r32(sym::gModeMenu); c.m.r32(a_) }; F::Sprite_Unpack(c, a0_, ((0i32) as u32), ((0i32) as u32)) }; let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(4); c.m.w32(a_, v_); };
        { let v_ = { let a0_ = (({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_add({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(52); (c.m.r32(a_) as i32) })).wrapping_add(16i32); F::File_LoadSync(c, a0_, ((0i32) as u32), 0i32) }; let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(8); c.m.w32(a_, v_); };
        { let v_ = { let a0_ = { let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(8); c.m.r32(a_) }; F::Sprite_Unpack(c, a0_, ((0i32) as u32), ((0i32) as u32)) }; let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(12); c.m.w32(a_, v_); };
        { let a0_ = { let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(12); c.m.r32(a_) }; F::ModeBg_Init(c, a0_) };
        { let v_ = ({ let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(12); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(12); c.m.r32(a_) }).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        's1: {
            let sv4: i32 = { let a_ = (c.m.r32(sym::gProgress)).wrapping_add(52); (c.m.r32(a_) as i32) };
            match sv4 {
                0i32 => {
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gModeMenu)).wrapping_add(80)).wrapping_add((((10i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gModeMenu)).wrapping_add(80)).wrapping_add((((11i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gModeMenu)).wrapping_add(80)).wrapping_add((((12i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    break 's1;
                }
                1i32 => {
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gModeMenu)).wrapping_add(80)).wrapping_add((((13i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gModeMenu)).wrapping_add(80)).wrapping_add((((14i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gModeMenu)).wrapping_add(80)).wrapping_add((((15i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    break 's1;
                }
                2i32 => {
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gModeMenu)).wrapping_add(80)).wrapping_add((((16i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gModeMenu)).wrapping_add(80)).wrapping_add((((17i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gModeMenu)).wrapping_add(80)).wrapping_add((((18i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    break 's1;
                }
                3i32 => {
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gModeMenu)).wrapping_add(80)).wrapping_add((((19i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gModeMenu)).wrapping_add(80)).wrapping_add((((21i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gModeMenu)).wrapping_add(80)).wrapping_add((((20i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    break 's1;
                }
                4i32 => {
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gModeMenu)).wrapping_add(80)).wrapping_add((((22i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gModeMenu)).wrapping_add(80)).wrapping_add((((23i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gModeMenu)).wrapping_add(80)).wrapping_add((((24i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    break 's1;
                }
                5i32 => {
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gModeMenu)).wrapping_add(80)).wrapping_add((((25i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gModeMenu)).wrapping_add(80)).wrapping_add((((26i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gModeMenu)).wrapping_add(80)).wrapping_add((((27i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    break 's1;
                }
                6i32 => {
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gModeMenu)).wrapping_add(80)).wrapping_add((((28i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gModeMenu)).wrapping_add(80)).wrapping_add((((29i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gModeMenu)).wrapping_add(80)).wrapping_add((((30i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    break 's1;
                }
                7i32 => {
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gModeMenu)).wrapping_add(80)).wrapping_add((((31i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gModeMenu)).wrapping_add(80)).wrapping_add((((32i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gModeMenu)).wrapping_add(80)).wrapping_add((((33i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    break 's1;
                }
                _ => {
                }
            }
        }
        { let v_ = ({ let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gModeMenu)).wrapping_add(80)).wrapping_add((((8i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gModeMenu)).wrapping_add(80)).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gModeMenu)).wrapping_add(80)).wrapping_add((((34i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((4i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gModeMenu)).wrapping_add(80)).wrapping_add((((38i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((5i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gModeMenu)).wrapping_add(80)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((6i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gModeMenu)).wrapping_add(80)).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((7i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gModeMenu)).wrapping_add(80)).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((8i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gModeMenu)).wrapping_add(80)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gModeMenu)).wrapping_add(80)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(12); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(12); c.m.r32(a_) }).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gModeMenu)).wrapping_add(80)).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gModeMenu)).wrapping_add(80)).wrapping_add((((36i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(12); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(12); c.m.r32(a_) }).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gModeMenu)).wrapping_add(80)).wrapping_add((((37i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gModeMenu)).wrapping_add(80)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gModeMenu)).wrapping_add(80)).wrapping_add((((35i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let a0_ = ((c.m.r32(sym::gModeMenu)).wrapping_add(36)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); let a1_ = ({ let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a2_ = (c.m.r32(sym::gModeMenu)).wrapping_add(80); F::Flash_Create(c, a0_, a1_, a2_) };
        { let a0_ = ((c.m.r32(sym::gModeMenu)).wrapping_add(36)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_Play(c, a0_, 1i32) };
        { let v_ = ({ let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(12); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(12); c.m.r32(a_) }).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(24); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(12); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(12); c.m.r32(a_) }).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(28); c.m.w32(a_, v_); };
        { let a0_ = ({ let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((8i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a1_ = { let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(24); c.m.r32(a_) }; let _ = 0i32; F::MsgWin_Init(c, a0_, a1_, 1i32) };
        { let v_ = ({ let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(32); c.m.w32(a_, v_); };
        's2: {
            let sv5: i32 = { let a_ = (c.m.r32(sym::gProgress)).wrapping_add(52); (c.m.r32(a_) as i32) };
            match sv5 {
                0i32 => {
                    { let v_ = 3i32; let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(376); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 34103i32; let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(264); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 19i32; let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(268); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 974i32; let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(272); c.m.w32(a_, (v_) as u32); };
                    break 's2;
                }
                1i32 => {
                    { let v_ = 4i32; let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(376); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 33827i32; let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(264); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 12i32; let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(268); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 977i32; let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(272); c.m.w32(a_, (v_) as u32); };
                    break 's2;
                }
                2i32 => {
                    { let v_ = 7i32; let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(376); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 33716i32; let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(264); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 18i32; let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(268); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 981i32; let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(272); c.m.w32(a_, (v_) as u32); };
                    break 's2;
                }
                3i32 => {
                    { let v_ = 5i32; let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(376); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 33755i32; let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(264); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 21i32; let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(268); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 988i32; let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(272); c.m.w32(a_, (v_) as u32); };
                    break 's2;
                }
                4i32 => {
                    { let v_ = 16i32; let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(376); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 33924i32; let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(264); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 19i32; let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(268); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 993i32; let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(272); c.m.w32(a_, (v_) as u32); };
                    break 's2;
                }
                5i32 => {
                    { let v_ = 5i32; let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(376); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 33856i32; let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(264); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 20i32; let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(268); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 1009i32; let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(272); c.m.w32(a_, (v_) as u32); };
                    break 's2;
                }
                6i32 => {
                    { let v_ = 4i32; let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(376); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 33794i32; let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(264); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 16i32; let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(268); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 1014i32; let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(272); c.m.w32(a_, (v_) as u32); };
                    break 's2;
                }
                7i32 => {
                    { let v_ = 4i32; let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(376); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 33894i32; let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(264); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 14i32; let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(268); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 1018i32; let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(272); c.m.w32(a_, (v_) as u32); };
                    break 's2;
                }
                _ => {
                }
            }
        }
        {
            l_i_2 = 0i32;
            'l3: loop {
                if !(((((l_i_2) < ({ let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(376); (c.m.r32(a_) as i32) })) as i32)) != 0) { break 'l3; }
                'c4: {
                    l_slot_3 = ((c.m.r32(sym::gSaveData)).wrapping_add(16)).wrapping_add(((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(52); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(16));
                    if (((((c.m.r32(((l_slot_3).wrapping_add(4)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) & ((((1u32).wrapping_shl((l_i_2) as u32)) as i32)))) != 0) {
                        { let v_ = l_i_2; let a_ = ((c.m.r32(sym::gModeMenu)).wrapping_add(296)).wrapping_add(((({ let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(360); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); o_ }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    }
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        {
            l_i_2 = { let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(360); (c.m.r32(a_) as i32) };
            'l5: loop {
                if !(((((l_i_2) < (3i32)) as i32)) != 0) { break 'l5; }
                'c6: {
                    { let v_ = { let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(376); (c.m.r32(a_) as i32) }; let a_ = ((c.m.r32(sym::gModeMenu)).wrapping_add(296)).wrapping_add(((({ let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(360); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); o_ }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        { let v_ = ({ let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(12); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(12); c.m.r32(a_) }).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(824); c.m.w32(a_, v_); };
        if ((((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(52); (c.m.r32(a_) as i32) }) == (7i32)) as i32)) != 0) {
            { let v_ = 2i32; let a_ = { let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(824); c.m.r32(a_) }; c.m.w32(a_, (v_) as u32); };
        }
        if ((((((((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(28); (c.m.r32(a_) as i32) }) == (10i32)) as i32)) != 0) || ((((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(28); (c.m.r32(a_) as i32) }) == (8i32)) as i32)) != 0)) as i32)) != 0) {
            {
                l_i_2 = 0i32;
                'l7: loop {
                    if !(((((l_i_2) < ({ let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(360); (c.m.r32(a_) as i32) })) as i32)) != 0) { break 'l7; }
                    'c8: {
                        if ((((({ let a_ = ((c.m.r32(sym::gModeMenu)).wrapping_add(296)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) == ({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(56); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                            { let v_ = l_i_2; let a_ = ((c.m.r32(sym::gModeMenu)).wrapping_add(248)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                            break 'l7;
                        }
                    }
                    { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
                }
            }
        }
        { let v_ = { let a_ = ((c.m.r32(sym::gModeMenu)).wrapping_add(248)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(364); c.m.w32(a_, (v_) as u32); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(364); (c.m.r32(a_) as i32) }).wrapping_add(2i32); let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(368); c.m.w32(a_, (v_) as u32); };
        'l9: while ((((({ let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(368); (c.m.r32(a_) as i32) }) >= ({ let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(360); (c.m.r32(a_) as i32) })) as i32)) != 0) {
            { let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(364); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
            { let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(368); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
        }
        { let v_ = F::Heap_Alloc(c, 114688i32, ((64i32) as u32), 0i32, 2i32); let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(16); c.m.w32(a_, v_); };
        { let v_ = F::Heap_Alloc(c, 133120i32, ((32i32) as u32), 0i32, 2i32); let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(20); c.m.w32(a_, v_); };
        { let v_ = 1i32; let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(396); c.m.w32(a_, (v_) as u32); };
        { let v_ = (1i32).wrapping_neg(); let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(280); c.m.w32(a_, (v_) as u32); };
        { let v_ = ((F::Rand_Range(c, ((32i32) as u32))) as i32); let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(284); c.m.w32(a_, (v_) as u32); };
        {
            l_i_2 = 0i32;
            'l10: loop {
                if !(((((l_i_2) < (3i32)) as i32)) != 0) { break 'l10; }
                'c11: {
                    { let a0_ = ((c.m.r32(sym::gModeMenu)).wrapping_add(404)).wrapping_add((((l_i_2) as u32)).wrapping_mul(140)); let a1_ = { let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(32); c.m.r32(a_) }; F::TextBox_Init(c, a0_, a1_, ((0i32) as u32)) };
                    { let a0_ = ((c.m.r32(sym::gModeMenu)).wrapping_add(404)).wrapping_add((((l_i_2) as u32)).wrapping_mul(140)); F::TextBox_SetUnk80(c, a0_, 1i32) };
                    { let a0_ = ((c.m.r32(sym::gModeMenu)).wrapping_add(404)).wrapping_add((((l_i_2) as u32)).wrapping_mul(140)); F::TextBox_SetUnk50(c, a0_, 1i32) };
                    { let a0_ = ((c.m.r32(sym::gModeMenu)).wrapping_add(404)).wrapping_add((((l_i_2) as u32)).wrapping_mul(140)); F::TextBox_SetRect(c, a0_, 0i32, 512i32, 0i32, 417i32) };
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        's12: {
            let sv6: i32 = { let a_ = (c.m.r32(sym::gProgress)).wrapping_add(52); (c.m.r32(a_) as i32) };
            match sv6 {
                0i32 => {
                    F::Bgm_Play(c, 68380i32);
                    F::StreamSe_PlayDefault(c, 0i32, 68572i32);
                    break 's12;
                }
                1i32 => {
                    F::Bgm_Play(c, 68380i32);
                    F::StreamSe_PlayDefault(c, 0i32, 68573i32);
                    break 's12;
                }
                2i32 => {
                    F::Bgm_Play(c, 68380i32);
                    F::StreamSe_PlayDefault(c, 0i32, 68574i32);
                    break 's12;
                }
                3i32 | 4i32 => {
                    F::Bgm_Play(c, 68380i32);
                    break 's12;
                }
                5i32 => {
                    F::Bgm_Play(c, 68382i32);
                    break 's12;
                }
                6i32 => {
                    F::Bgm_Play(c, 68392i32);
                    break 's12;
                }
                7i32 => {
                    F::Bgm_Play(c, 68380i32);
                    break 's12;
                }
                _ => {
                }
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "ModeMenu_Init", &PROBE_N, &[], true); }
    r_
}

pub fn ModeMenu_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ModeMenu_Term", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let r_ = 'f: {
    F::ModeBg_Term(c);
    F::MsgWin_Term(c);
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (1i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let a0_ = ((c.m.r32(sym::gModeMenu)).wrapping_add(36)).wrapping_add((((l_i_1) as u32)).wrapping_mul(44)); F::Flash_Destroy(c, a0_) };
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
    if ((((({ let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(20); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = { let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(20); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(20); c.m.w32(a_, v_); };
    }
    if ((((({ let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(16); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = { let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(16); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(16); c.m.w32(a_, v_); };
    }
    if ((((({ let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(12); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = { let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(12); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(12); c.m.w32(a_, v_); };
    }
    if ((((({ let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(8); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = { let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(8); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(8); c.m.w32(a_, v_); };
    }
    if ((((({ let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(4); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = { let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(4); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(4); c.m.w32(a_, v_); };
    }
    if (((((c.m.r32(sym::gModeMenu)) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32(sym::gModeMenu); F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = sym::gModeMenu; c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ModeMenu_Term", &PROBE_N, &[], true); }
    r_
}

pub fn ModeMenu_Draw(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ModeMenu_Draw", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(160) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 160);
    let mut l_i_1: i32 = 0;
    let mut l_flash_2: u32 = 0;
    let r_ = 'f: {
        F::ModeBg_Draw(c);
        l_flash_2 = ((c.m.r32(sym::gModeMenu)).wrapping_add(36)).wrapping_add((((0i32) as u32)).wrapping_mul(44));
        { let _ = { let a2_ = { let a_ = (c.m.r32(sym::gProgress)).wrapping_add(52); (c.m.r32(a_) as i32) }; F::sprintf(c, fp.wrapping_add(28), sym::menu_a_d__str0, &[((a2_) as i64 as u64)]) }; };
        { let _ = { let a2_ = { let a_ = (c.m.r32(sym::gProgress)).wrapping_add(52); (c.m.r32(a_) as i32) }; F::sprintf(c, fp.wrapping_add(92), sym::menu_a_d__str1, &[((a2_) as i64 as u64)]) }; };
        F::Flash_FindLabel(c, l_flash_2, fp.wrapping_add(28), fp.wrapping_add(92), fp);
        { let a2_ = (c.m.r32(sym::gModeMenu)).wrapping_add(284); F::FlashAnim_Blink(c, l_flash_2, fp, a2_, 0i32) };
        { let _ = { let a2_ = { let a_ = (c.m.r32(sym::gProgress)).wrapping_add(52); (c.m.r32(a_) as i32) }; F::sprintf(c, fp.wrapping_add(92), sym::menu_a_d__str2, &[((a2_) as i64 as u64)]) }; };
        F::Flash_FindLabel(c, l_flash_2, fp.wrapping_add(28), fp.wrapping_add(92), fp);
        { let a2_ = (c.m.r32(sym::gModeMenu)).wrapping_add(288); F::FlashAnim_Talk(c, l_flash_2, fp, a2_, 0i32) };
        if (((({ let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(244); (c.m.r32(a_) as i32) }) & (128i32))) != 0) {
            { let r_ = F32(0x3d999999); let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(400); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            if (((rt::fge({ let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(400); F32(c.m.r32(a_)) }, F32(0x3f800000)) as i32)) != 0) {
                { let v_ = F32(0x3f800000); let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(400); c.m.w32(a_, (v_).0); };
            }
        } else {
            { let v_ = F32(0x00000000); let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(400); c.m.w32(a_, (v_).0); };
        }
        F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), sym::menu_a_d__str3, fp);
        { let _ = F::Flash_ClipSetFlags(c, l_flash_2, fp, 128i32, ((((1i32) as i64 as u64)) as u8)); };
        F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), sym::menu_a_d__str4, fp);
        { let a2_ = { let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(400); F32(c.m.r32(a_)) }; F::Flash_ClipSetAlpha(c, l_flash_2, fp, a2_) };
        { let _ = F::Flash_ClipSetFlags(c, l_flash_2, fp, 256i32, ((((1i32) as i64 as u64)) as u8)); };
        { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = (rt::rem_i32({ let a_ = ((c.m.r32(sym::gModeMenu)).wrapping_add(296)).wrapping_add(((({ let a_ = ((c.m.r32(sym::gModeMenu)).wrapping_add(248)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }, 4i32)).wrapping_mul(64i32); let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = 512i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = ((c.m.r32((fp.wrapping_add(8)).wrapping_add(4)) as i32)).wrapping_add(64i32); let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), sym::menu_a_d__str5, fp);
        { let a2_ = { let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(400); F32(c.m.r32(a_)) }; F::Flash_ClipSetAlpha(c, l_flash_2, fp, a2_) };
        F::Flash_ClipSetUv(c, l_flash_2, fp, fp.wrapping_add(8));
        { let a2_ = rt::div_i32({ let a_ = ((c.m.r32(sym::gModeMenu)).wrapping_add(296)).wrapping_add(((({ let a_ = ((c.m.r32(sym::gModeMenu)).wrapping_add(248)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }, 4i32); F::Flash_ClipSetTex(c, l_flash_2, fp, a2_) };
        F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), sym::menu_a_d__str6, fp);
        if (((({ let a_ = ((((c.m.r32(sym::gSaveData)).wrapping_add(16)).wrapping_add(((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(52); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(16))).wrapping_add(4)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) & ((((1u32).wrapping_shl(({ let a_ = ((c.m.r32(sym::gModeMenu)).wrapping_add(296)).wrapping_add(((({ let a_ = ((c.m.r32(sym::gModeMenu)).wrapping_add(248)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)) as i32)))) != 0) {
            { let _ = F::Flash_ClipSetFlags(c, l_flash_2, fp, 2i32, ((((1i32) as i64 as u64)) as u8)); };
            { let a2_ = { let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(400); F32(c.m.r32(a_)) }; F::Flash_ClipSetAlpha(c, l_flash_2, fp, a2_) };
        } else {
            { let _ = F::Flash_ClipSetFlags(c, l_flash_2, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
        }
        {
            l_i_1 = 0i32;
            'l1: loop {
                if !(((((l_i_1) < (2i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let v_ = 0i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    { let v_ = (l_i_1).wrapping_mul(32i32); let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = ((c.m.r32(fp.wrapping_add(8)) as i32)).wrapping_add(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 32i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                    's3: {
                        let sv3: i32 = l_i_1;
                        match sv3 {
                            0i32 => {
                                F::Flash_FindLabel(c, l_flash_2, sym::menu_a_d__str7, sym::menu_a_d__str8, fp);
                                { let a3_ = ((({ let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(364); (c.m.r32(a_) as i32) }) != (0i32)) as i32); { let _ = F::Flash_ClipSetFlags(c, l_flash_2, fp, 2i32, ((((a3_) as i64 as u64)) as u8)); } };
                                break 's3;
                            }
                            1i32 => {
                                F::Flash_FindLabel(c, l_flash_2, sym::menu_a_d__str9, sym::menu_a_d__str10, fp);
                                { let a3_ = ((({ let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(368); (c.m.r32(a_) as i32) }) != (({ let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(360); (c.m.r32(a_) as i32) }).wrapping_sub(1i32))) as i32); { let _ = F::Flash_ClipSetFlags(c, l_flash_2, fp, 2i32, ((((a3_) as i64 as u64)) as u8)); } };
                                break 's3;
                            }
                            _ => {
                            }
                        }
                    }
                    F::Flash_ClipSetUv(c, l_flash_2, fp, fp.wrapping_add(8));
                }
                { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
            }
        }
        {
            l_i_1 = 0i32;
            'l4: loop {
                if !(((((l_i_1) < (3i32)) as i32)) != 0) { break 'l4; }
                'c5: {
                    { let _ = F::sprintf(c, fp.wrapping_add(28), sym::menu_a_d__str11, &[(((l_i_1).wrapping_add(1i32)) as i64 as u64)]); };
                    F::Flash_FindLabel(c, l_flash_2, fp.wrapping_add(28), sym::menu_a_d__str12, fp);
                    if ((((((((({ let a_ = ((c.m.r32(sym::gModeMenu)).wrapping_add(296)).wrapping_add((((({ let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(364); (c.m.r32(a_) as i32) }).wrapping_add(l_i_1)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) != ({ let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(376); (c.m.r32(a_) as i32) })) as i32)) != 0) && (((({ let a_ = ((((c.m.r32(sym::gSaveData)).wrapping_add(16)).wrapping_add(((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(52); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(16))).wrapping_add(4)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) & ((((1u32).wrapping_shl(({ let a_ = ((c.m.r32(sym::gModeMenu)).wrapping_add(296)).wrapping_add((((({ let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(364); (c.m.r32(a_) as i32) }).wrapping_add(l_i_1)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)) as i32)))) != 0)) as i32)) != 0) {
                        { let _ = F::Flash_ClipSetFlags(c, l_flash_2, fp, 2i32, ((((1i32) as i64 as u64)) as u8)); };
                    } else {
                        { let _ = F::Flash_ClipSetFlags(c, l_flash_2, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
                    }
                    F::Flash_FindLabel(c, l_flash_2, fp.wrapping_add(28), sym::menu_a_d__str13, fp);
                    { let a2_ = { let a_ = ((c.m.r32(sym::gModeMenu)).wrapping_add(296)).wrapping_add((((({ let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(364); (c.m.r32(a_) as i32) }).wrapping_add(l_i_1)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; F::Flash_ClipSetTex(c, l_flash_2, fp, a2_) };
                }
                { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
            }
        }
        {
            l_i_1 = 0i32;
            'l6: loop {
                if !(((((l_i_1) < (2i32)) as i32)) != 0) { break 'l6; }
                'c7: {
                    { let _ = F::sprintf(c, fp.wrapping_add(28), sym::menu_a_d__str14, &[(((l_i_1).wrapping_add(4i32)) as i64 as u64)]); };
                    F::Flash_FindLabel(c, l_flash_2, fp.wrapping_add(28), sym::menu_a_d__str15, fp);
                    if ((((((((({ let a_ = ((c.m.r32(sym::gModeMenu)).wrapping_add(296)).wrapping_add(((({ let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(372); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) != ({ let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(376); (c.m.r32(a_) as i32) })) as i32)) != 0) && (((({ let a_ = ((((c.m.r32(sym::gSaveData)).wrapping_add(16)).wrapping_add(((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(52); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(16))).wrapping_add(4)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) & ((((1u32).wrapping_shl(({ let a_ = ((c.m.r32(sym::gModeMenu)).wrapping_add(296)).wrapping_add(((({ let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(372); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)) as i32)))) != 0)) as i32)) != 0) {
                        { let _ = F::Flash_ClipSetFlags(c, l_flash_2, fp, 2i32, ((((1i32) as i64 as u64)) as u8)); };
                    } else {
                        { let _ = F::Flash_ClipSetFlags(c, l_flash_2, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
                    }
                    F::Flash_FindLabel(c, l_flash_2, fp.wrapping_add(28), sym::menu_a_d__str16, fp);
                    { let a2_ = { let a_ = ((c.m.r32(sym::gModeMenu)).wrapping_add(296)).wrapping_add(((({ let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(372); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; F::Flash_ClipSetTex(c, l_flash_2, fp, a2_) };
                }
                { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
            }
        }
        {
            l_i_1 = 0i32;
            'l8: loop {
                if !(((((l_i_1) < (3i32)) as i32)) != 0) { break 'l8; }
                'c9: {
                    { let _ = F::sprintf(c, fp.wrapping_add(28), sym::menu_a_d__str17, &[(((l_i_1).wrapping_add(1i32)) as i64 as u64)]); };
                    F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), fp.wrapping_add(28), fp);
                    if (((((l_i_1) >= ({ let a_ = ({ let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(824); c.m.r32(a_) }).wrapping_add(((({ let a_ = ((c.m.r32(sym::gModeMenu)).wrapping_add(296)).wrapping_add(((({ let a_ = ((c.m.r32(sym::gModeMenu)).wrapping_add(248)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(32)); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                        { let a3_ = (rt::f2i({ let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(380); F32(c.m.r32(a_)) })).wrapping_neg(); let a5_ = ((c.m.r32(sym::gModeMenu)).wrapping_add(404)).wrapping_add((((l_i_1) as u32)).wrapping_mul(140)); F::TextBox_AttachLine(c, l_flash_2, fp, 0i32, a3_, (1i32).wrapping_neg(), a5_) };
                    } else {
                        { let a3_ = (rt::f2i({ let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(380); F32(c.m.r32(a_)) })).wrapping_neg(); let a4_ = ({ let a_ = (({ let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(824); c.m.r32(a_) }).wrapping_add(((({ let a_ = ((c.m.r32(sym::gModeMenu)).wrapping_add(296)).wrapping_add(((({ let a_ = ((c.m.r32(sym::gModeMenu)).wrapping_add(248)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(32))).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_add(l_i_1); let a5_ = ((c.m.r32(sym::gModeMenu)).wrapping_add(404)).wrapping_add((((l_i_1) as u32)).wrapping_mul(140)); F::TextBox_AttachLine(c, l_flash_2, fp, 0i32, a3_, a4_, a5_) };
                    }
                }
                { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
            }
        }
        F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), sym::menu_a_d__str18, fp);
        { let a3_ = ((({ let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(244); (c.m.r32(a_) as i32) }).wrapping_shr((5i32) as u32)) & (1i32)); { let _ = F::Flash_ClipSetFlags(c, l_flash_2, fp, 2i32, ((((a3_) as i64 as u64)) as u8)); } };
        {
            l_i_1 = 0i32;
            'l10: loop {
                if !(((((l_i_1) < (1i32)) as i32)) != 0) { break 'l10; }
                'c11: {
                    { let a0_ = ((c.m.r32(sym::gModeMenu)).wrapping_add(36)).wrapping_add((((l_i_1) as u32)).wrapping_mul(44)); F::Flash_Draw(c, a0_) };
                }
                { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
            }
        }
        F::Font_FlushAll(c);
        { let a2_ = { let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(280); (c.m.r32(a_) as i32) }; F::MsgWin_Draw(c, 0i32, 0i32, a2_) };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "ModeMenu_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn ModeMenu_Update(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ModeMenu_Update", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let r_ = 'f: {
    if (((((((({ let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(244); (c.m.r32(a_) as i32) }) & (8i32))) != 0) && ((((((((({ let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(280); (c.m.r32(a_) as i32) }) == ({ let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(268); (c.m.r32(a_) as i32) })) as i32)) != 0) || ((((({ let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(280); (c.m.r32(a_) as i32) }) == (({ let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(268); (c.m.r32(a_) as i32) }).wrapping_add(3i32))) as i32)) != 0)) as i32)) != 0)) as i32)) != 0) {
        if (((((F::Voice_GetStat(c, 0i32)) == (5i32)) as i32)) != 0) {
            { let v_ = (({ let a_ = ((c.m.r32(sym::gModeMenu)).wrapping_add(296)).wrapping_add(((({ let a_ = ((c.m.r32(sym::gModeMenu)).wrapping_add(248)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add({ let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(268); (c.m.r32(a_) as i32) })).wrapping_add(4i32); let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(280); c.m.w32(a_, (v_) as u32); };
            { let a0_ = { let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(28); c.m.r32(a_) }; let a1_ = { let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(264); (c.m.r32(a_) as i32) }; let a2_ = { let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(280); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, a1_, a2_) };
        }
    }
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (1i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let a0_ = ((c.m.r32(sym::gModeMenu)).wrapping_add(36)).wrapping_add((((l_i_1) as u32)).wrapping_mul(44)); F::Flash_Advance(c, a0_) };
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
    if ((((((((!(((({ let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(244); (c.m.r32(a_) as i32) }) & (4i32))) != 0)) as i32)) != 0) && (((({ let a_ = (((c.m.r32(sym::gModeMenu)).wrapping_add(36)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add(12); c.m.r32(a_) }) & (((2i32) as u32)))) != 0)) as i32)) != 0) {
        F::MsgWin_Open(c);
    }
    if (((({ let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(244); (c.m.r32(a_) as i32) }) & (16i32))) != 0) {
        { let r_ = F32(0x3e6eeeee); let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(380); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        if (((rt::fge({ let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(380); F32(c.m.r32(a_)) }, { let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(384); F32(c.m.r32(a_)) }) as i32)) != 0) {
            { let v_ = { let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(384); F32(c.m.r32(a_)) }; let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(380); c.m.w32(a_, (v_).0); };
            { let r_ = (!(16i32)); let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(244); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
            { let r_ = 32i32; let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(244); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ModeMenu_Update", &PROBE_N, &[], true); }
    r_
}

pub fn ModeMenu_PlateGoto(c: &mut Ctx, mut p_movie_1: i32, mut p_kind_2: i32, mut p_label_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ModeMenu_PlateGoto", &PROBE_N, &[(p_movie_1) as u32, (p_kind_2) as u32, (p_label_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(80) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 80);
    let mut l_flash_4: u32 = 0;
    let r_ = 'f: {
        l_flash_4 = ((c.m.r32(sym::gModeMenu)).wrapping_add(36)).wrapping_add((((p_movie_1) as u32)).wrapping_mul(44));
        if (((((p_kind_2) == (0i32)) as i32)) != 0) {
            { let _ = { let a2_ = (({ let a_ = ((c.m.r32(sym::gModeMenu)).wrapping_add(248)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_sub({ let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(364); (c.m.r32(a_) as i32) })).wrapping_add(1i32); F::sprintf(c, fp.wrapping_add(8), sym::menu_a_d__str19, &[((a2_) as i64 as u64)]) }; };
            F::Flash_FindLabel(c, l_flash_4, ((0i32) as u32), fp.wrapping_add(8), fp);
            F::Flash_ClipGotoLabel(c, l_flash_4, fp, p_label_3);
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "ModeMenu_PlateGoto", &PROBE_N, &[], true); }
    r_
}

pub fn ModeMenu_Input(c: &mut Ctx, mut p_result_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ModeMenu_Input", &PROBE_N, &[(p_result_1) as u32], false); }
    let r_ = 'f: {
    if ((((!(((({ let a_ = (((c.m.r32(sym::gModeMenu)).wrapping_add(36)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add(8); c.m.r32(a_) }) & (((2i32) as u32)))) != 0)) as i32)) != 0) {
        break 'f;
    }
    if ((((!(((({ let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(244); (c.m.r32(a_) as i32) }) & (4i32))) != 0)) as i32)) != 0) {
        F::ModeMenu_PlateGoto(c, 0i32, 0i32, sym::menu_a_d__str20);
        { let r_ = 4i32; let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(244); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
    's1: {
        let sv2: i32 = { let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(260); (c.m.r32(a_) as i32) };
        match sv2 {
            0i32 => {
                if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((1i32) as u32)))) != 0) {
                    { let v_ = 0i32; let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(292); c.m.w32(a_, (v_) as u32); };
                    if ((((((((({ let a_ = ((c.m.r32(sym::gModeMenu)).wrapping_add(248)).wrapping_add(((({ let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(260); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) > (0i32)) as i32)) != 0) && ((((({ let a_ = ((c.m.r32(sym::gModeMenu)).wrapping_add(296)).wrapping_add((((({ let a_ = ((c.m.r32(sym::gModeMenu)).wrapping_add(248)).wrapping_add(((({ let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(260); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_sub(1i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) != ({ let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(376); (c.m.r32(a_) as i32) })) as i32)) != 0)) as i32)) != 0) {
                        F::ModeMenu_PlateGoto(c, 0i32, 0i32, sym::menu_a_d__str21);
                        { let a_ = ((c.m.r32(sym::gModeMenu)).wrapping_add(248)).wrapping_add(((({ let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(260); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                        if ((((({ let a_ = ((c.m.r32(sym::gModeMenu)).wrapping_add(248)).wrapping_add(((({ let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(260); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) < ({ let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(364); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                            { let a0_ = ((c.m.r32(sym::gModeMenu)).wrapping_add(36)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_a_d__str22, ((((1i32) as i64 as u64)) as u8)) };
                            { let v_ = { let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(368); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(372); c.m.w32(a_, (v_) as u32); };
                            { let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(364); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                            { let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(368); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                        }
                        F::ModeMenu_PlateGoto(c, 0i32, 0i32, sym::menu_a_d__str23);
                        { let v_ = (({ let a_ = ((c.m.r32(sym::gModeMenu)).wrapping_add(296)).wrapping_add(((({ let a_ = ((c.m.r32(sym::gModeMenu)).wrapping_add(248)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add({ let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(268); (c.m.r32(a_) as i32) })).wrapping_add(4i32); let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(280); c.m.w32(a_, (v_) as u32); };
                        { let a0_ = { let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(28); c.m.r32(a_) }; let a1_ = { let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(264); (c.m.r32(a_) as i32) }; let a2_ = { let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(280); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, a1_, a2_) };
                        F::ModeMenu_ChangeImage(c);
                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                    }
                } else {
                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((2i32) as u32)))) != 0) {
                        { let v_ = 0i32; let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(292); c.m.w32(a_, (v_) as u32); };
                        if ((((((((({ let a_ = ((c.m.r32(sym::gModeMenu)).wrapping_add(248)).wrapping_add(((({ let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(260); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) < (({ let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(360); (c.m.r32(a_) as i32) }).wrapping_sub(1i32))) as i32)) != 0) && ((((({ let a_ = ((c.m.r32(sym::gModeMenu)).wrapping_add(296)).wrapping_add((((({ let a_ = ((c.m.r32(sym::gModeMenu)).wrapping_add(248)).wrapping_add(((({ let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(260); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(1i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) != ({ let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(376); (c.m.r32(a_) as i32) })) as i32)) != 0)) as i32)) != 0) {
                            F::ModeMenu_PlateGoto(c, 0i32, 0i32, sym::menu_a_d__str24);
                            { let a_ = ((c.m.r32(sym::gModeMenu)).wrapping_add(248)).wrapping_add(((({ let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(260); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                            if ((((({ let a_ = ((c.m.r32(sym::gModeMenu)).wrapping_add(248)).wrapping_add(((({ let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(260); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) > ({ let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(368); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                                { let a0_ = ((c.m.r32(sym::gModeMenu)).wrapping_add(36)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_a_d__str25, ((((1i32) as i64 as u64)) as u8)) };
                                { let v_ = { let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(364); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(372); c.m.w32(a_, (v_) as u32); };
                                { let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(364); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                                { let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(368); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                            }
                            F::ModeMenu_PlateGoto(c, 0i32, 0i32, sym::menu_a_d__str26);
                            { let v_ = (({ let a_ = ((c.m.r32(sym::gModeMenu)).wrapping_add(296)).wrapping_add(((({ let a_ = ((c.m.r32(sym::gModeMenu)).wrapping_add(248)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add({ let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(268); (c.m.r32(a_) as i32) })).wrapping_add(4i32); let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(280); c.m.w32(a_, (v_) as u32); };
                            { let a0_ = { let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(28); c.m.r32(a_) }; let a1_ = { let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(264); (c.m.r32(a_) as i32) }; let a2_ = { let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(280); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, a1_, a2_) };
                            F::ModeMenu_ChangeImage(c);
                            { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                        }
                    } else {
                        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                            { let v_ = 0i32; let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(292); c.m.w32(a_, (v_) as u32); };
                            F::ModeMenu_PlateGoto(c, 0i32, 0i32, sym::menu_a_d__str27);
                            { let v_ = 1i32; let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(276); c.m.w32(a_, (v_) as u32); };
                            { let v_ = F32(0x00000000); let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(380); c.m.w32(a_, (v_).0); };
                            { let v_ = rt::fadd(rt::fmul(rt::i2f({ let a_ = ({ let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(824); c.m.r32(a_) }).wrapping_add(((({ let a_ = ((c.m.r32(sym::gModeMenu)).wrapping_add(296)).wrapping_add(((({ let a_ = ((c.m.r32(sym::gModeMenu)).wrapping_add(248)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(32)); (c.m.r32(a_) as i32) }), F32(0x42200000)), F32(0x43200000)); let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(384); c.m.w32(a_, (v_).0); };
                            { let r_ = (!(32i32)); let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(244); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                            { let v_ = 1i32; let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(260); c.m.w32(a_, (v_) as u32); };
                            { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                        } else {
                            if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1024i32) as u32)))) != 0) {
                                { let v_ = 0i32; let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(292); c.m.w32(a_, (v_) as u32); };
                                F::ColorFade_StartOut(c, 0i32, 0i32, 0i32, 20i32);
                                { let v_ = 0i32; let a_ = p_result_1; c.m.w32(a_, (v_) as u32); };
                                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 2i32); };
                            }
                        }
                    }
                }
                break 's1;
            }
            1i32 => {
                if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((4608i32) as u32)))) != 0) {
                    { let r_ = 1i32; let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(244); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    { let r_ = 2i32; let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(244); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    { let v_ = 15i32; let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(256); c.m.w32(a_, (v_) as u32); };
                    { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                } else {
                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1024i32) as u32)))) != 0) {
                        F::ModeMenu_PlateGoto(c, 0i32, 0i32, sym::menu_a_d__str28);
                        { let a0_ = ((c.m.r32(sym::gModeMenu)).wrapping_add(36)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_a_d__str29, ((((1i32) as i64 as u64)) as u8)) };
                        F::MsgWin_Open(c);
                        { let v_ = (({ let a_ = ((c.m.r32(sym::gModeMenu)).wrapping_add(248)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add({ let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(268); (c.m.r32(a_) as i32) })).wrapping_add(4i32); let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(280); c.m.w32(a_, (v_) as u32); };
                        { let a0_ = { let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(28); c.m.r32(a_) }; let a1_ = { let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(264); (c.m.r32(a_) as i32) }; let a2_ = { let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(280); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, a1_, a2_) };
                        { let v_ = 0i32; let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(260); c.m.w32(a_, (v_) as u32); };
                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 2i32); };
                    }
                }
                break 's1;
            }
            _ => {
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ModeMenu_Input", &PROBE_N, &[], true); }
    r_
}

pub fn ModeMenu_UpdateStep(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ModeMenu_UpdateStep", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if ((((({ let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(276); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
        break 'f;
    }
    's1: {
        let sv1: i32 = { let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(276); (c.m.r32(a_) as i32) };
        match sv1 {
            1i32 => {
                { let v_ = ({ let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(268); (c.m.r32(a_) as i32) }).wrapping_add(1i32); let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(280); c.m.w32(a_, (v_) as u32); };
                { let a0_ = { let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(28); c.m.r32(a_) }; let a1_ = { let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(264); (c.m.r32(a_) as i32) }; let a2_ = { let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(280); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, a1_, a2_) };
                { let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(276); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            2i32 => {
                if (((((F::Voice_GetStat(c, 0i32)) == (5i32)) as i32)) != 0) {
                    { let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(276); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                } else {
                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                        { let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(276); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        F::Voice_StopWithLip(c);
                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                    }
                }
                break 's1;
            }
            3i32 => {
                { let a0_ = ((c.m.r32(sym::gModeMenu)).wrapping_add(36)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_a_d__str30, ((((1i32) as i64 as u64)) as u8)) };
                F::MsgWin_Close(c);
                { let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(276); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            4i32 => {
                { let v_ = 51i32; let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(276); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            51i32 => {
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(388); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(392); c.m.w32(a_, (v_) as u32); };
                { let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(276); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            52i32 => {
                if (((({ let a_ = (((c.m.r32(sym::gModeMenu)).wrapping_add(36)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add(12); c.m.r32(a_) }) & (((1i32) as u32)))) != 0) {
                    { let r_ = 16i32; let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(244); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    { let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(276); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                }
                break 's1;
            }
            53i32 => {
                if ((((({ let a_ = ((({ let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(824); c.m.r32(a_) }).wrapping_add(((({ let a_ = ((c.m.r32(sym::gModeMenu)).wrapping_add(296)).wrapping_add(((({ let a_ = ((c.m.r32(sym::gModeMenu)).wrapping_add(248)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(32))).wrapping_add(8)).wrapping_add(((({ let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(392); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) <= ({ let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(388); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); o_ })) as i32)) != 0) {
                    { let a2_ = ({ let a_ = (({ let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(824); c.m.r32(a_) }).wrapping_add(((({ let a_ = ((c.m.r32(sym::gModeMenu)).wrapping_add(296)).wrapping_add(((({ let a_ = ((c.m.r32(sym::gModeMenu)).wrapping_add(248)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(32))).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_add({ let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(392); (c.m.r32(a_) as i32) }); F::Voice_PlayWithSubtitle(c, ((0i32) as u32), 33974i32, a2_) };
                    { let v_ = 0i32; let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(388); c.m.w32(a_, (v_) as u32); };
                    { let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(276); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                }
                break 's1;
            }
            54i32 => {
                if (((((F::Voice_GetStat(c, 0i32)) == (5i32)) as i32)) != 0) {
                    if ((((({ let a_ = ({ let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(824); c.m.r32(a_) }).wrapping_add(((({ let a_ = ((c.m.r32(sym::gModeMenu)).wrapping_add(296)).wrapping_add(((({ let a_ = ((c.m.r32(sym::gModeMenu)).wrapping_add(248)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(32)); (c.m.r32(a_) as i32) }) <= ({ let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(392); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); n_ })) as i32)) != 0) {
                        { let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(276); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    } else {
                        { let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(276); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                    }
                } else {
                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((4096i32) as u32)))) != 0) {
                        { let v_ = 101i32; let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(276); c.m.w32(a_, (v_) as u32); };
                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                    } else {
                        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1024i32) as u32)))) != 0) {
                            F::ModeMenu_PlateGoto(c, 0i32, 0i32, sym::menu_a_d__str31);
                            { let a0_ = ((c.m.r32(sym::gModeMenu)).wrapping_add(36)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_a_d__str32, ((((1i32) as i64 as u64)) as u8)) };
                            F::MsgWin_Open(c);
                            { let v_ = (({ let a_ = ((c.m.r32(sym::gModeMenu)).wrapping_add(248)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add({ let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(268); (c.m.r32(a_) as i32) })).wrapping_add(4i32); let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(280); c.m.w32(a_, (v_) as u32); };
                            { let a0_ = { let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(28); c.m.r32(a_) }; let a1_ = { let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(264); (c.m.r32(a_) as i32) }; let a2_ = { let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(280); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, a1_, a2_) };
                            { let v_ = 0i32; let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(388); c.m.w32(a_, (v_) as u32); };
                            { let v_ = 0i32; let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(392); c.m.w32(a_, (v_) as u32); };
                            { let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(276); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                            { let v_ = 0i32; let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(260); c.m.w32(a_, (v_) as u32); };
                            { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 2i32); };
                        }
                    }
                }
                break 's1;
            }
            55i32 => {
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(276); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            101i32 => {
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(276); c.m.w32(a_, (v_) as u32); };
                { let r_ = 1i32; let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(244); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let r_ = 2i32; let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(244); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 15i32; let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(256); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            _ => {
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ModeMenu_UpdateStep", &PROBE_N, &[], true); }
    r_
}

pub fn ModeMenu_Run(c: &mut Ctx, mut p_section_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ModeMenu_Run", &PROBE_N, &[(p_section_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_slot_2: u32 = 0;
    let r_ = 'f: {
        { let v_ = 1i32; c.m.w32(fp, (v_) as u32); }
        F::ModeMenu_Init(c, p_section_1);
        if ((((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(28); (c.m.r32(a_) as i32) }) == (6i32)) as i32)) != 0) {
            F::ColorFade_StartIn(c, 255i32, 255i32, 255i32, 20i32);
        } else {
            F::ColorFade_StartIn(c, 0i32, 0i32, 0i32, 20i32);
        }
        'l1: while ((1i32) != 0) {
            F::Gfx_BeginFrame(c);
            F::ModeMenu_UpdateImage(c);
            F::Pad_Update(c);
            F::Snd_Update(c);
            F::ColorFade_Update(c);
            if ((((!(((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(20); (c.m.r32(a_) as i32) }) & (256i32))) != 0)) as i32)) != 0) {
                F::ModeMenu_Update(c);
                F::ModeMenu_UpdateStep(c);
            }
            F::ModeMenu_Draw(c);
            F::ColorFade_Draw(c);
            F::Gfx_EndFrame(c, 1i32);
            F::Dma_Flush(c);
            F::File_Stub264D90(c);
            if (((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(20); (c.m.r32(a_) as i32) }) & (256i32))) != 0) {
                continue 'l1;
            }
            if ((F::ColorFade_IsInDone(c)) != 0) {
                if ((((((((!(((({ let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(244); (c.m.r32(a_) as i32) }) & (8i32))) != 0)) as i32)) != 0) && (((({ let a_ = (((c.m.r32(sym::gModeMenu)).wrapping_add(36)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add(8); c.m.r32(a_) }) & (((2i32) as u32)))) != 0)) as i32)) != 0) {
                    { let r_ = 8i32; let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(244); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    {
                        l_slot_2 = ((c.m.r32(sym::gSaveData)).wrapping_add(16)).wrapping_add(((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(52); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(16));
                        if (((((c.m.r32(l_slot_2) as i32)) & (64i32))) != 0) {
                            { let v_ = ({ let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(268); (c.m.r32(a_) as i32) }).wrapping_add(3i32); let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(280); c.m.w32(a_, (v_) as u32); };
                            l_slot_2 = (c.m.r32(sym::gSaveData)).wrapping_add(16);
                            { let r_ = { let a_ = (c.m.r32(sym::gProgress)).wrapping_add(52); (c.m.r32(a_) as i32) }; let n_ = (l_slot_2).wrapping_add((((r_) as u32)).wrapping_mul(16)); l_slot_2 = n_; };
                            { let r_ = (!(64i32)); let a_ = l_slot_2; let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                        } else {
                            { let v_ = { let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(268); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(280); c.m.w32(a_, (v_) as u32); };
                        }
                    }
                    { let a0_ = { let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(28); c.m.r32(a_) }; let a1_ = { let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(264); (c.m.r32(a_) as i32) }; let a2_ = { let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(280); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, a1_, a2_) };
                }
            }
            if ((F::ColorFade_IsFadingOut(c)) != 0) {
                F::Voice_FadeOutStep(c, 0i32);
                F::Bgm_FadeOutStep(c);
                F::StreamSe_FadeOutStep(c, 0i32);
                continue 'l1;
            }
            if ((F::ColorFade_IsOutDone(c)) != 0) {
                if ((((({ let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(396); (c.m.r32(a_) as i32) }) != (4i32)) as i32)) != 0) {
                    continue 'l1;
                }
                break 'l1;
            } else {
                if (((({ let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(244); (c.m.r32(a_) as i32) }) & (2i32))) != 0) {
                    if ((((({ let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(256); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) == ((1i32).wrapping_neg())) as i32)) != 0) {
                        F::ColorFade_StartOut(c, 0i32, 0i32, 0i32, 20i32);
                        { let v_ = { let a_ = ((c.m.r32(sym::gModeMenu)).wrapping_add(296)).wrapping_add(((({ let a_ = ((c.m.r32(sym::gModeMenu)).wrapping_add(248)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(56); c.m.w32(a_, (v_) as u32); };
                    }
                } else {
                    if ((((({ let a_ = (c.m.r32(sym::gModeMenu)).wrapping_add(276); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
                        F::ModeMenu_Input(c, fp);
                    }
                }
            }
        }
        if (((c.m.r32(fp) as i32)) != 0) {
            { let r_ = (!((1i32).wrapping_shl(({ let a_ = ((c.m.r32(sym::gModeMenu)).wrapping_add(296)).wrapping_add(((({ let a_ = ((c.m.r32(sym::gModeMenu)).wrapping_add(248)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32))); let a_ = ((((c.m.r32(sym::gSaveData)).wrapping_add(16)).wrapping_add(((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(52); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(16))).wrapping_add(4)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
            F::BattleSetup_Clear(c);
            { let a0_ = { let a0_ = { let a_ = (c.m.r32(sym::gProgress)).wrapping_add(52); (c.m.r32(a_) as i32) }; let a1_ = { let a_ = ((c.m.r32(sym::gModeMenu)).wrapping_add(296)).wrapping_add(((({ let a_ = ((c.m.r32(sym::gModeMenu)).wrapping_add(248)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; F::ModeMenu_GetLine(c, a0_, a1_) }; F::BattleSetup_SetScript(c, a0_) };
        }
        F::ModeMenu_Term(c);
        F::Dma_ResetBuffers(c);
        break 'f (c.m.r32(fp) as i32);
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "ModeMenu_Run", &PROBE_N, &[], true); }
    r_
}
