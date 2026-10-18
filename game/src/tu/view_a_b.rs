#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn MsgWin_Init(c: &mut Ctx, mut p_pack_1: u32, mut p_text_2: u32, mut p_side_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "MsgWin_Init", &PROBE_N, &[(p_pack_1) as u32, (p_text_2) as u32, (p_side_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        { let v_ = ((0i32) as u32); c.m.w32(fp, v_); }
        { let v_ = F::Heap_Alloc(c, ((208u32) as i32), ((32i32) as u32), 0i32, 2i32); let a_ = sym::gMsgWin; c.m.w32(a_, v_); };
        { let _ = { let a0_ = c.m.r32(sym::gMsgWin); F::memset(c, a0_, 0i32, 208u32) }; };
        { let v_ = (p_pack_1).wrapping_add(((c.m.r32((p_pack_1).wrapping_add((((1i32) as u32)).wrapping_mul(4)))).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = { let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }; let a_ = ((c.m.r32(sym::gMsgWin)).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add(((64i32) as u32)); let a_ = ((c.m.r32(sym::gMsgWin)).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let a0_ = (c.m.r32(sym::gMsgWin)).wrapping_add(4); let a1_ = (p_pack_1).wrapping_add(((c.m.r32((p_pack_1).wrapping_add((((2i32) as u32)).wrapping_mul(4)))).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a2_ = (c.m.r32(sym::gMsgWin)).wrapping_add(48); F::Flash_Create(c, a0_, a1_, a2_) };
        { let a0_ = (c.m.r32(sym::gMsgWin)).wrapping_add(4); F::Flash_Play(c, a0_, 1i32) };
        { let a0_ = (c.m.r32(sym::gMsgWin)).wrapping_add(68); F::TextBox_Init(c, a0_, p_text_2, ((5i32) as u32)) };
        { let v_ = p_text_2; let a_ = c.m.r32(sym::gMsgWin); c.m.w32(a_, v_); };
        { let v_ = p_side_3; let a_ = (c.m.r32(sym::gMsgWin)).wrapping_add(64); c.m.w32(a_, (v_) as u32); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "MsgWin_Init", &PROBE_N, &[], true); }
    r_
}

pub fn MsgWin_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "MsgWin_Term", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let a0_ = (c.m.r32(sym::gMsgWin)).wrapping_add(4); F::Flash_Destroy(c, a0_) };
    if (((((c.m.r32(sym::gMsgWin)) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32(sym::gMsgWin); F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = sym::gMsgWin; c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "MsgWin_Term", &PROBE_N, &[], true); }
    r_
}

pub fn MsgWin_Draw(c: &mut Ctx, mut p_unused0_1: i32, mut p_unused1_2: i32, mut p_line_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "MsgWin_Draw", &PROBE_N, &[(p_unused0_1) as u32, (p_unused1_2) as u32, (p_line_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_flash_4: u32 = 0;
    let mut l_i_5: i32 = 0;
    let r_ = 'f: {
        { let a0_ = (c.m.r32(sym::gMsgWin)).wrapping_add(4); F::Flash_Advance(c, a0_) };
        l_flash_4 = (c.m.r32(sym::gMsgWin)).wrapping_add(4);
        if ((((({ let a_ = c.m.r32(sym::gMsgWin); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
            F::Flash_FindLabel(c, l_flash_4, ((0i32) as u32), sym::view_a_b__str0, fp);
            { let a5_ = (c.m.r32(sym::gMsgWin)).wrapping_add(68); F::TextBox_AttachLine(c, l_flash_4, fp, 0i32, 0i32, p_line_3, a5_) };
        }
        {
            l_i_5 = 0i32;
            'l1: loop {
                if !(((((l_i_5) < (1i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let a0_ = ((c.m.r32(sym::gMsgWin)).wrapping_add(4)).wrapping_add((((l_i_5) as u32)).wrapping_mul(44)); F::Flash_Draw(c, a0_) };
                }
                { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
            }
        }
        F::Font_FlushAll(c);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "MsgWin_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn MsgWin_Open(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "MsgWin_Open", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if ((((({ let a_ = (c.m.r32(sym::gMsgWin)).wrapping_add(64); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
        { let a0_ = (c.m.r32(sym::gMsgWin)).wrapping_add(4); F::Flash_GotoLabel(c, a0_, sym::view_a_b__str1, ((((1i32) as i64 as u64)) as u8)) };
    } else {
        { let a0_ = (c.m.r32(sym::gMsgWin)).wrapping_add(4); F::Flash_GotoLabel(c, a0_, sym::view_a_b__str2, ((((1i32) as i64 as u64)) as u8)) };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "MsgWin_Open", &PROBE_N, &[], true); }
    r_
}

pub fn MsgWin_Close(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "MsgWin_Close", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if ((((({ let a_ = (c.m.r32(sym::gMsgWin)).wrapping_add(64); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
        { let a0_ = (c.m.r32(sym::gMsgWin)).wrapping_add(4); F::Flash_GotoLabel(c, a0_, sym::view_a_b__str3, ((((1i32) as i64 as u64)) as u8)) };
    } else {
        { let a0_ = (c.m.r32(sym::gMsgWin)).wrapping_add(4); F::Flash_GotoLabel(c, a0_, sym::view_a_b__str4, ((((1i32) as i64 as u64)) as u8)) };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "MsgWin_Close", &PROBE_N, &[], true); }
    r_
}

pub fn MsgWin_SetText(c: &mut Ctx, mut p_text_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "MsgWin_SetText", &PROBE_N, &[(p_text_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_text_1; let a_ = c.m.r32(sym::gMsgWin); c.m.w32(a_, v_); };
    { let v_ = p_text_1; let a_ = ((c.m.r32(sym::gMsgWin)).wrapping_add(68)).wrapping_add(4); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "MsgWin_SetText", &PROBE_N, &[], true); }
    r_
}

pub fn MsgWin_SetSide(c: &mut Ctx, mut p_side_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "MsgWin_SetSide", &PROBE_N, &[(p_side_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_side_1; let a_ = (c.m.r32(sym::gMsgWin)).wrapping_add(64); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "MsgWin_SetSide", &PROBE_N, &[], true); }
    r_
}

pub fn MsgWin_SetBoxParam(c: &mut Ctx, mut p_a_1: i32, mut p_b_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "MsgWin_SetBoxParam", &PROBE_N, &[(p_a_1) as u32, (p_b_2) as u32], false); }
    let r_ = 'f: {
    { let a0_ = (c.m.r32(sym::gMsgWin)).wrapping_add(68); F::TextBox_SetSpacing(c, a0_, p_a_1, p_b_2) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "MsgWin_SetBoxParam", &PROBE_N, &[], true); }
    r_
}
