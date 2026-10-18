#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn UbMenu_CursorUp(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "UbMenu_CursorUp", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if ((((({ let a_ = ((c.m.r32(sym::gUbMenu)).wrapping_add(136)).wrapping_add(((({ let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(152); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) < (0i32)) as i32)) != 0) {
        { let v_ = (4i32).wrapping_sub(1i32); let a_ = ((c.m.r32(sym::gUbMenu)).wrapping_add(136)).wrapping_add(((({ let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(152); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "UbMenu_CursorUp", &PROBE_N, &[], true); }
    r_
}

pub fn UbMenu_CursorDown(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "UbMenu_CursorDown", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if ((((({ let a_ = ((c.m.r32(sym::gUbMenu)).wrapping_add(136)).wrapping_add(((({ let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(152); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); n_ }) >= (4i32)) as i32)) != 0) {
        { let v_ = 0i32; let a_ = ((c.m.r32(sym::gUbMenu)).wrapping_add(136)).wrapping_add(((({ let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(152); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "UbMenu_CursorDown", &PROBE_N, &[], true); }
    r_
}

pub fn UbMenu_Init(c: &mut Ctx, mut p_section_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "UbMenu_Init", &PROBE_N, &[(p_section_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
        { let v_ = ((0i32) as u32); c.m.w32(fp, v_); }
        { let v_ = F::Heap_Alloc(c, ((200u32) as i32), ((32i32) as u32), 0i32, 2i32); let a_ = sym::gUbMenu; c.m.w32(a_, v_); };
        { let _ = { let a0_ = c.m.r32(sym::gUbMenu); F::memset(c, a0_, 0i32, 200u32) }; };
        { let v_ = (c.m.r32(sym::gMenuArc3)).wrapping_add((({ let a_ = (c.m.r32(sym::gMenuArc3)).wrapping_add((((p_section_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = c.m.r32(sym::gUbMenu); c.m.w32(a_, v_); };
        { let v_ = { let a0_ = { let a_ = c.m.r32(sym::gUbMenu); c.m.r32(a_) }; F::Sprite_Unpack(c, a0_, ((0i32) as u32), ((0i32) as u32)) }; let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(4); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = c.m.r32(fp); let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(60); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbMenu)).wrapping_add(64)).wrapping_add((((8i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbMenu)).wrapping_add(64)).wrapping_add((((9i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbMenu)).wrapping_add(64)).wrapping_add((((10i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((5i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbMenu)).wrapping_add(64)).wrapping_add((((11i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((6i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbMenu)).wrapping_add(64)).wrapping_add((((12i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((8i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbMenu)).wrapping_add(64)).wrapping_add((((13i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbMenu)).wrapping_add(64)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbMenu)).wrapping_add(64)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbMenu)).wrapping_add(64)).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbMenu)).wrapping_add(64)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((4i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbMenu)).wrapping_add(64)).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((5i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbMenu)).wrapping_add(64)).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((6i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbMenu)).wrapping_add(64)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((7i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbMenu)).wrapping_add(64)).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((8i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbMenu)).wrapping_add(64)).wrapping_add((((14i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((9i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbMenu)).wrapping_add(64)).wrapping_add((((15i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((10i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbMenu)).wrapping_add(64)).wrapping_add((((16i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let a0_ = ((c.m.r32(sym::gUbMenu)).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); let a1_ = ({ let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a2_ = (c.m.r32(sym::gUbMenu)).wrapping_add(64); F::Flash_Create(c, a0_, a1_, a2_) };
        { let a0_ = ((c.m.r32(sym::gUbMenu)).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_Play(c, a0_, 1i32) };
        {
            l_i_2 = 0i32;
            'l1: loop {
                if !(((((l_i_2) < (2i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let v_ = rt::rem_i32(F::Rand_Libc(c), 32i32); let a_ = ((c.m.r32(sym::gUbMenu)).wrapping_add(176)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        { let v_ = ({ let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((8i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(8); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((9i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(12); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let a0_ = ({ let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a1_ = c.m.r32(fp); F::IconWin_Init(c, a0_, a1_) };
        F::IconWin_Open(c);
        { let a0_ = ({ let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a1_ = { let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(8); c.m.r32(a_) }; let _ = 0i32; F::MsgWin_Init(c, a0_, a1_, 0i32) };
        F::MsgWin_Open(c);
        { let v_ = (1i32).wrapping_neg(); let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(168); c.m.w32(a_, (v_) as u32); };
        { let v_ = (({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(520); (c.m.r32(a_) as i32) }) & (1i32)); let a_ = sym::gUbMenuPlate3Open; c.m.w32(a_, (v_) as u32); };
        { let v_ = { let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1592); (c.m.r32(a_) as i32) }; let a_ = ((c.m.r32(sym::gUbMenu)).wrapping_add(136)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(164); c.m.w32(a_, (v_) as u32); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "UbMenu_Init", &PROBE_N, &[], true); }
    r_
}

pub fn UbMenu_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "UbMenu_Term", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let r_ = 'f: {
    F::MsgWin_Term(c);
    F::IconWin_Term(c);
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (1i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let a0_ = ((c.m.r32(sym::gUbMenu)).wrapping_add(16)).wrapping_add((((l_i_1) as u32)).wrapping_mul(44)); F::Flash_Destroy(c, a0_) };
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
    if ((((({ let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(4); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = { let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(4); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(4); c.m.w32(a_, v_); };
    }
    if (((((c.m.r32(sym::gUbMenu)) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32(sym::gUbMenu); F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = sym::gUbMenu; c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "UbMenu_Term", &PROBE_N, &[], true); }
    r_
}

pub fn UbMenu_Draw(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "UbMenu_Draw", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(96) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 96);
    let mut l_flash_1: u32 = 0;
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
        { let a0_ = { let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(60); c.m.r32(a_) }; F::Sprite_DrawPicture(c, a0_, 0i32, 0i32, 128i32) };
        l_flash_1 = ((c.m.r32(sym::gUbMenu)).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(44));
        {
            l_i_2 = 0i32;
            'l1: loop {
                if !(((((l_i_2) < (2i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    F::Flash_FindLabel(c, l_flash_1, (if (((((l_i_2) != (0i32)) as i32)) != 0) { sym::menu_p_c__str0 } else { sym::menu_p_c__str1 }), (if (((((l_i_2) != (0i32)) as i32)) != 0) { sym::menu_p_c__str2 } else { sym::menu_p_c__str3 }), fp);
                    { let a2_ = ((c.m.r32(sym::gUbMenu)).wrapping_add(176)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); F::FlashAnim_Blink(c, l_flash_1, fp, a2_, 0i32) };
                    F::Flash_FindLabel(c, l_flash_1, (if (((((l_i_2) != (0i32)) as i32)) != 0) { sym::menu_p_c__str4 } else { sym::menu_p_c__str5 }), (if (((((l_i_2) != (0i32)) as i32)) != 0) { sym::menu_p_c__str6 } else { sym::menu_p_c__str7 }), fp);
                    if ((((({ let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(172); (c.m.r32(a_) as i32) }) == (l_i_2)) as i32)) != 0) {
                        { let a2_ = ((c.m.r32(sym::gUbMenu)).wrapping_add(184)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); F::FlashAnim_Talk(c, l_flash_1, fp, a2_, 0i32) };
                    } else {
                        F::FlashAnim_ShowNext2(c, l_flash_1, fp, 0i32);
                    }
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        { let v_ = ({ let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(196); (c.m.r32(a_) as i32) }).wrapping_shl((6i32) as u32); let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = ((c.m.r32(fp.wrapping_add(8)) as i32)).wrapping_add(64i32); let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 64i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_p_c__str8, fp);
        F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(8));
        { let r_ = 1i32; let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(196); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
        {
            l_i_2 = 0i32;
            'l3: loop {
                if !(((((l_i_2) < (4i32)) as i32)) != 0) { break 'l3; }
                'c4: {
                    { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = (l_i_2).wrapping_shl((5i32) as u32); let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 512i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = ((c.m.r32((fp.wrapping_add(8)).wrapping_add(4)) as i32)).wrapping_add(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                    if (((((((((l_i_2) == (2i32)) as i32)) != 0) && ((((((c.m.r32(sym::gUbMenuPlate3Open) as i32)) == (0i32)) as i32)) != 0)) as i32)) != 0) {
                        { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                        { let v_ = (4i32).wrapping_shl((5i32) as u32); let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                        { let v_ = 512i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                        { let v_ = ((c.m.r32((fp.wrapping_add(8)).wrapping_add(4)) as i32)).wrapping_add(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                    }
                    { let _ = F::sprintf(c, fp.wrapping_add(28), sym::menu_p_c__str9, &[(((l_i_2).wrapping_add(1i32)) as i64 as u64)]); };
                    F::Flash_FindLabel(c, l_flash_1, fp.wrapping_add(28), sym::menu_p_c__str10, fp);
                    F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(8));
                    F::Flash_FindLabel(c, l_flash_1, fp.wrapping_add(28), sym::menu_p_c__str11, fp);
                    F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(8));
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        {
            l_i_2 = 0i32;
            'l5: loop {
                if !(((((l_i_2) < (1i32)) as i32)) != 0) { break 'l5; }
                'c6: {
                    { let a0_ = ((c.m.r32(sym::gUbMenu)).wrapping_add(16)).wrapping_add((((l_i_2) as u32)).wrapping_mul(44)); F::Flash_Draw(c, a0_) };
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        F::IconWin_Draw(c);
        { let a2_ = { let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(168); (c.m.r32(a_) as i32) }; F::MsgWin_Draw(c, 0i32, 0i32, a2_) };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "UbMenu_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn UbMenu_Update(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "UbMenu_Update", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let r_ = 'f: {
    if (((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(20); (c.m.r32(a_) as i32) }) & (256i32))) != 0) {
        break 'f;
    }
    if ((((({ let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(140); (c.m.r32(a_) as i32) }) > (0i32)) as i32)) != 0) {
        { let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(140); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
    }
    if (((((((({ let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(132); (c.m.r32(a_) as i32) }) & (8i32))) != 0) && ((((((({ let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(168); (c.m.r32(a_) as i32) }) as u32)) < (((2i32) as u32))) as i32)) != 0)) as i32)) != 0) {
        if (((((F::Voice_GetStat(c, 0i32)) == (5i32)) as i32)) != 0) {
            { let v_ = ({ let a_ = ((c.m.r32(sym::gUbMenu)).wrapping_add(136)).wrapping_add(((({ let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(152); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(2i32); let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(168); c.m.w32(a_, (v_) as u32); };
            if ((((((((({ let a_ = ((c.m.r32(sym::gUbMenu)).wrapping_add(136)).wrapping_add(((({ let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(152); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) == (1i32)) as i32)) != 0) || ((((({ let a_ = ((c.m.r32(sym::gUbMenu)).wrapping_add(136)).wrapping_add(((({ let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(152); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) == (3i32)) as i32)) != 0)) as i32)) != 0) {
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(172); c.m.w32(a_, (v_) as u32); };
            } else {
                { let v_ = 1i32; let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(172); c.m.w32(a_, (v_) as u32); };
            }
            { let a0_ = { let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(12); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(168); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34661i32, a2_) };
        }
    }
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (1i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let a0_ = ((c.m.r32(sym::gUbMenu)).wrapping_add(16)).wrapping_add((((l_i_1) as u32)).wrapping_mul(44)); F::Flash_Advance(c, a0_) };
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "UbMenu_Update", &PROBE_N, &[], true); }
    r_
}

pub fn UbMenu_UpdateVoice(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "UbMenu_UpdateVoice", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if (((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(20); (c.m.r32(a_) as i32) }) & (256i32))) != 0) {
        break 'f;
    }
    if ((((({ let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(164); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
        if ((((({ let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(156); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
            break 'f;
        }
        if ((((((((((((({ let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(168); (c.m.r32(a_) as i32) }) != ((1i32).wrapping_neg())) as i32)) != 0) && (((((F::Voice_GetStat(c, 0i32)) != (5i32)) as i32)) != 0)) as i32)) != 0) && ((((({ let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(156); (c.m.r32(a_) as i32) }) == ({ let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(160); (c.m.r32(a_) as i32) })) as i32)) != 0)) as i32)) != 0) {
            if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                { let _ = ((D::call(c, fid::Snd_PlaySe, &[((((1i32) as u32)) as u64), ((1i32) as i64 as u64)])) as i32); };
                { let v_ = 1i32; let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(164); c.m.w32(a_, (v_) as u32); };
            }
            break 'f;
        }
    } else {
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(164); c.m.w32(a_, (v_) as u32); };
    }
    's1: {
        let sv1: i32 = { let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(156); (c.m.r32(a_) as i32) };
        match sv1 {
            1i32 => {
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(168); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(172); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(156); c.m.w32(a_, (v_) as u32); };
                { let a0_ = { let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(12); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(168); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34661i32, a2_) };
                break 's1;
            }
            11i32 => {
                { let v_ = 1i32; let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(168); c.m.w32(a_, (v_) as u32); };
                { let v_ = 1i32; let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(172); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(156); c.m.w32(a_, (v_) as u32); };
                { let a0_ = { let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(12); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(168); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34661i32, a2_) };
                break 's1;
            }
            21i32 => {
                { let v_ = 7i32; let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(168); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(172); c.m.w32(a_, (v_) as u32); };
                { let a0_ = { let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(12); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(168); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34661i32, a2_) };
                { let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(156); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            22i32 => {
                { let v_ = 8i32; let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(168); c.m.w32(a_, (v_) as u32); };
                { let v_ = 1i32; let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(172); c.m.w32(a_, (v_) as u32); };
                { let a0_ = { let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(12); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(168); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34661i32, a2_) };
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(156); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            41i32 => {
                { let v_ = 9i32; let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(168); c.m.w32(a_, (v_) as u32); };
                { let v_ = 1i32; let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(172); c.m.w32(a_, (v_) as u32); };
                { let a0_ = { let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(12); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(168); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34661i32, a2_) };
                { let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(156); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            42i32 => {
                { let v_ = 10i32; let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(168); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(172); c.m.w32(a_, (v_) as u32); };
                { let a0_ = { let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(12); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(168); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34661i32, a2_) };
                { let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(156); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            43i32 => {
                { let v_ = 11i32; let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(168); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(172); c.m.w32(a_, (v_) as u32); };
                { let a0_ = { let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(12); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(168); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34661i32, a2_) };
                { let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(156); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            44i32 => {
                { let v_ = 12i32; let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(168); c.m.w32(a_, (v_) as u32); };
                { let v_ = 1i32; let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(172); c.m.w32(a_, (v_) as u32); };
                { let a0_ = { let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(12); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(168); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34661i32, a2_) };
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(156); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            61i32 => {
                { let v_ = 13i32; let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(168); c.m.w32(a_, (v_) as u32); };
                { let v_ = 1i32; let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(172); c.m.w32(a_, (v_) as u32); };
                { let a0_ = { let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(12); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(168); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34661i32, a2_) };
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(156); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            81i32 => {
                { let v_ = 14i32; let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(168); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(172); c.m.w32(a_, (v_) as u32); };
                { let a0_ = { let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(12); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(168); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34661i32, a2_) };
                { let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(156); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            101i32 => {
                { let v_ = 15i32; let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(168); c.m.w32(a_, (v_) as u32); };
                { let v_ = 1i32; let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(172); c.m.w32(a_, (v_) as u32); };
                { let a0_ = { let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(12); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(168); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34661i32, a2_) };
                { let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(156); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            121i32 => {
                { let v_ = 16i32; let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(168); c.m.w32(a_, (v_) as u32); };
                { let v_ = 1i32; let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(172); c.m.w32(a_, (v_) as u32); };
                { let a0_ = { let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(12); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(168); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34661i32, a2_) };
                { let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(156); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            82i32 | 102i32 | 122i32 => {
                F::ColorFade_StartOut(c, 0i32, 0i32, 0i32, 20i32);
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(156); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            _ => {
            }
        }
    }
    { let v_ = { let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(156); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(160); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "UbMenu_UpdateVoice", &PROBE_N, &[], true); }
    r_
}

pub fn UbMenu_Input(c: &mut Ctx, mut p_result_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "UbMenu_Input", &PROBE_N, &[(p_result_1) as u32], false); }
    let mut l_up_2: i32 = 0;
    let mut l_down_3: i32 = 0;
    let mut l_confirm_4: i32 = 0;
    let mut l_cancel_5: i32 = 0;
    let r_ = 'f: {
    l_up_2 = ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((8i32) as u32)))) as i32);
    l_down_3 = ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((4i32) as u32)))) as i32);
    l_confirm_4 = ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) as i32);
    l_cancel_5 = ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1024i32) as u32)))) as i32);
    if ((((!(((({ let a_ = (((c.m.r32(sym::gUbMenu)).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add(8); c.m.r32(a_) }) & (((2i32) as u32)))) != 0)) as i32)) != 0) {
        break 'f;
    }
    if ((((!(((({ let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(132); (c.m.r32(a_) as i32) }) & (4i32))) != 0)) as i32)) != 0) {
        F::UbMenu_ClipGoto(c, 0i32, 0i32, sym::menu_p_c__str12);
        if (((((F::Rand_Range(c, ((2i32) as u32))) != (((0i32) as u32))) as i32)) != 0) {
            { let v_ = 1i32; let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(156); c.m.w32(a_, (v_) as u32); };
        } else {
            { let v_ = 11i32; let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(156); c.m.w32(a_, (v_) as u32); };
        }
        { let r_ = 4i32; let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(132); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
    's1: {
        let sv6: i32 = { let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(152); (c.m.r32(a_) as i32) };
        match sv6 {
            0i32 => {
                if ((l_up_2) != 0) {
                    F::UbMenu_ClipGoto(c, 0i32, 0i32, sym::menu_p_c__str13);
                    F::UbMenu_CursorUp(c);
                    if ((((((((({ let a_ = ((c.m.r32(sym::gUbMenu)).wrapping_add(136)).wrapping_add(((({ let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(152); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) == (2i32)) as i32)) != 0) && ((((((c.m.r32(sym::gUbMenuPlate3Open) as i32)) == (0i32)) as i32)) != 0)) as i32)) != 0) {
                        F::UbMenu_CursorUp(c);
                    }
                    F::UbMenu_ClipGoto(c, 0i32, 0i32, sym::menu_p_c__str14);
                    { let v_ = ({ let a_ = ((c.m.r32(sym::gUbMenu)).wrapping_add(136)).wrapping_add(((({ let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(152); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(2i32); let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(168); c.m.w32(a_, (v_) as u32); };
                    if ((((((((({ let a_ = ((c.m.r32(sym::gUbMenu)).wrapping_add(136)).wrapping_add(((({ let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(152); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) == (1i32)) as i32)) != 0) || ((((({ let a_ = ((c.m.r32(sym::gUbMenu)).wrapping_add(136)).wrapping_add(((({ let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(152); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) == (3i32)) as i32)) != 0)) as i32)) != 0) {
                        { let v_ = 0i32; let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(172); c.m.w32(a_, (v_) as u32); };
                    } else {
                        { let v_ = 1i32; let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(172); c.m.w32(a_, (v_) as u32); };
                    }
                    { let a0_ = { let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(12); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(168); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34661i32, a2_) };
                    { let _ = ((D::call(c, fid::Snd_PlaySe, &[((((1i32) as u32)) as u64), ((0i32) as i64 as u64)])) as i32); };
                    { let v_ = 0i32; let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(192); c.m.w32(a_, (v_) as u32); };
                } else {
                    if ((l_down_3) != 0) {
                        F::UbMenu_ClipGoto(c, 0i32, 0i32, sym::menu_p_c__str15);
                        F::UbMenu_CursorDown(c);
                        if ((((((((({ let a_ = ((c.m.r32(sym::gUbMenu)).wrapping_add(136)).wrapping_add(((({ let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(152); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) == (2i32)) as i32)) != 0) && ((((((c.m.r32(sym::gUbMenuPlate3Open) as i32)) == (0i32)) as i32)) != 0)) as i32)) != 0) {
                            F::UbMenu_CursorDown(c);
                        }
                        F::UbMenu_ClipGoto(c, 0i32, 0i32, sym::menu_p_c__str16);
                        { let v_ = ({ let a_ = ((c.m.r32(sym::gUbMenu)).wrapping_add(136)).wrapping_add(((({ let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(152); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(2i32); let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(168); c.m.w32(a_, (v_) as u32); };
                        if ((((((((({ let a_ = ((c.m.r32(sym::gUbMenu)).wrapping_add(136)).wrapping_add(((({ let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(152); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) == (1i32)) as i32)) != 0) || ((((({ let a_ = ((c.m.r32(sym::gUbMenu)).wrapping_add(136)).wrapping_add(((({ let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(152); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) == (3i32)) as i32)) != 0)) as i32)) != 0) {
                            { let v_ = 0i32; let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(172); c.m.w32(a_, (v_) as u32); };
                        } else {
                            { let v_ = 1i32; let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(172); c.m.w32(a_, (v_) as u32); };
                        }
                        { let a0_ = { let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(12); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(168); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34661i32, a2_) };
                        { let _ = ((D::call(c, fid::Snd_PlaySe, &[((((1i32) as u32)) as u64), ((0i32) as i64 as u64)])) as i32); };
                        { let v_ = 0i32; let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(192); c.m.w32(a_, (v_) as u32); };
                    } else {
                        if ((l_confirm_4) != 0) {
                            { let r_ = 1i32; let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(132); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                            { let r_ = 2i32; let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(132); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                            { let v_ = 15i32; let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(140); c.m.w32(a_, (v_) as u32); };
                            { let v_ = ({ let a_ = ((c.m.r32(sym::gUbMenu)).wrapping_add(136)).wrapping_add(((({ let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(152); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(1i32); let a_ = p_result_1; c.m.w32(a_, (v_) as u32); };
                            { let _ = ((D::call(c, fid::Snd_PlaySe, &[((((1i32) as u32)) as u64), ((1i32) as i64 as u64)])) as i32); };
                            { let v_ = 0i32; let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(192); c.m.w32(a_, (v_) as u32); };
                            F::Voice_StopWithLip(c);
                        } else {
                            if ((l_cancel_5) != 0) {
                                { let v_ = 0i32; let a_ = p_result_1; c.m.w32(a_, (v_) as u32); };
                                { let v_ = { let a_ = (sym::menu_p_c__sUbMenuLeaveScript).wrapping_add((F::Rand_Range(c, ((3i32) as u32))).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(156); c.m.w32(a_, (v_) as u32); };
                                { let a0_ = ((c.m.r32(sym::gUbMenu)).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_p_c__str17, ((((1i32) as i64 as u64)) as u8)) };
                                { let _ = ((D::call(c, fid::Snd_PlaySe, &[((((1i32) as u32)) as u64), ((2i32) as i64 as u64)])) as i32); };
                                { let v_ = 0i32; let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(192); c.m.w32(a_, (v_) as u32); };
                            } else {
                                { let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(192); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                                if ((((({ let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(192); (c.m.r32(a_) as i32) }) == (1800i32)) as i32)) != 0) {
                                    { let v_ = { let a_ = (sym::menu_p_c__sUbMenuIdleScript).wrapping_add((F::Rand_Range(c, ((3i32) as u32))).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(156); c.m.w32(a_, (v_) as u32); };
                                    { let v_ = 0i32; let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(192); c.m.w32(a_, (v_) as u32); };
                                }
                            }
                        }
                    }
                }
                break 's1;
            }
            _ => {
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "UbMenu_Input", &PROBE_N, &[], true); }
    r_
}

pub fn UbMenu_ClipGoto(c: &mut Ctx, mut p_movie_1: i32, mut p_level_2: i32, mut p_label_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "UbMenu_ClipGoto", &PROBE_N, &[(p_movie_1) as u32, (p_level_2) as u32, (p_label_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(80) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 80);
    let mut l_flash_4: u32 = 0;
    let r_ = 'f: {
        l_flash_4 = ((c.m.r32(sym::gUbMenu)).wrapping_add(16)).wrapping_add((((p_movie_1) as u32)).wrapping_mul(44));
        { let _ = { let a2_ = ({ let a_ = ((c.m.r32(sym::gUbMenu)).wrapping_add(136)).wrapping_add((((p_level_2) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(1i32); F::sprintf(c, fp.wrapping_add(8), sym::menu_p_c__str18, &[((a2_) as i64 as u64)]) }; };
        F::Flash_FindLabel(c, l_flash_4, ((0i32) as u32), fp.wrapping_add(8), fp);
        F::Flash_ClipGotoLabel(c, l_flash_4, fp, p_label_3);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "UbMenu_ClipGoto", &PROBE_N, &[], true); }
    r_
}

pub fn UbMenu_Run(c: &mut Ctx, mut p_section_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "UbMenu_Run", &PROBE_N, &[(p_section_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        { let v_ = 1i32; c.m.w32(fp, (v_) as u32); }
        F::UbMenu_Init(c, p_section_1);
        F::ColorFade_StartIn(c, 0i32, 0i32, 0i32, 20i32);
        'l1: while ((1i32) != 0) {
            F::Gfx_BeginFrame(c);
            F::Pad_Update(c);
            F::Snd_Update(c);
            F::ColorFade_Update(c);
            F::UbMenu_Update(c);
            F::UbMenu_UpdateVoice(c);
            F::UbMenu_Draw(c);
            F::ColorFade_Draw(c);
            F::Gfx_EndFrame(c, 1i32);
            F::Dma_Flush(c);
            F::File_Stub264D90(c);
            if ((F::ColorFade_IsInDone(c)) != 0) {
                if ((((((((!(((({ let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(132); (c.m.r32(a_) as i32) }) & (8i32))) != 0)) as i32)) != 0) && (((({ let a_ = (((c.m.r32(sym::gUbMenu)).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add(8); c.m.r32(a_) }) & (((2i32) as u32)))) != 0)) as i32)) != 0) {
                    { let r_ = 8i32; let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(132); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                }
            }
            if ((F::ColorFade_IsFadingOut(c)) != 0) {
                F::Voice_FadeOutStep(c, 0i32);
                F::Bgm_FadeOutStep(c);
                continue 'l1;
            }
            if ((F::ColorFade_IsOutDone(c)) != 0) {
                break 'l1;
            }
            if (((({ let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(132); (c.m.r32(a_) as i32) }) & (2i32))) != 0) {
                if ((((({ let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(140); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
                    F::ColorFade_StartOut(c, 0i32, 0i32, 0i32, 20i32);
                    { let v_ = { let a_ = ((c.m.r32(sym::gUbMenu)).wrapping_add(136)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1592); c.m.w32(a_, (v_) as u32); };
                }
            } else {
                if ((((({ let a_ = (c.m.r32(sym::gUbMenu)).wrapping_add(156); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
                    F::UbMenu_Input(c, fp);
                }
            }
        }
        F::UbMenu_Term(c);
        F::Dma_ResetBuffers(c);
        break 'f (c.m.r32(fp) as i32);
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "UbMenu_Run", &PROBE_N, &[], true); }
    r_
}
