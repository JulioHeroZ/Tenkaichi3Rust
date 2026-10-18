#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn Shop_CheckStockLevel(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Shop_CheckStockLevel", &PROBE_N, &[], false); }
    let mut l_total_1: i32 = 0;
    let mut l_owned_2: i32 = 0;
    let mut l_i_3: i32 = 0;
    let mut l_flags_4: i32 = 0;
    let r_ = 'f: {
    l_total_1 = 0i32;
    l_owned_2 = 0i32;
    if ((((!(((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(4104); (c.m.r32(a_) as i32) }) & (2i32))) != 0)) as i32)) != 0) {
        {
            l_i_3 = 0i32;
            'l1: loop {
                if !(((((l_i_3) < (350i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_flags_4 = { let a_ = (({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(12004); c.m.r32(a_) }).wrapping_add((((l_i_3) as u32)).wrapping_mul(40))).wrapping_add(20); (c.m.r32(a_) as i32) };
                    if ((((l_flags_4) & (5i32))) != 0) {
                        break 'c2;
                    }
                    if ((((!((((l_flags_4) & (64i32))) != 0)) as i32)) != 0) {
                        break 'c2;
                    }
                    if ((((l_flags_4) & (256i32))) != 0) {
                        break 'c2;
                    }
                    { let o_ = l_total_1; let n_ = (o_).wrapping_add(1); l_total_1 = n_; };
                    { let r_ = (((({ let a_ = ((c.m.r32(sym::gSaveData)).wrapping_add(11976)).wrapping_add(((l_i_3) as u32)); c.m.r8(a_) }) as i32)) & (1i32)); let n_ = (l_owned_2).wrapping_add(r_); l_owned_2 = n_; };
                }
                { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
            }
        }
        if (((((l_owned_2) == ((l_total_1).wrapping_sub(1i32))) as i32)) != 0) {
            { let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(4108); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
            if ((((((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(4108); (c.m.r32(a_) as i32) }) as u32)) >= (((6i32) as u32))) as i32)) != 0) {
                { let v_ = 5i32; let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(4108); c.m.w32(a_, (v_) as u32); };
            }
            { let r_ = 1i32; let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(4104); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            { let r_ = 2i32; let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(4104); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Shop_CheckStockLevel", &PROBE_N, &[], true); }
    r_
}

pub fn Shop_IsSoldOut(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Shop_IsSoldOut", &PROBE_N, &[], false); }
    let mut l_ret_1: i32 = 0;
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
    l_ret_1 = 1i32;
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < ({ let a_ = ((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) })) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((((!(((((((({ let a_ = ((c.m.r32(sym::gSaveData)).wrapping_add(11976)).wrapping_add((({ let a_ = (((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(1400))).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)); c.m.r8(a_) }) as i32)) & (1i32))) as u8)) != 0)) as i32)) != 0) {
                    l_ret_1 = 0i32;
                    break 'l1;
                }
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
    break 'f l_ret_1;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Shop_IsSoldOut", &PROBE_N, &[], true); }
    r_
}

pub fn Shop_CanBuy(c: &mut Ctx, mut p_item_1: i32, mut p_table_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Shop_CanBuy", &PROBE_N, &[(p_item_1) as u32, (p_table_2) as u32], false); }
    let mut l_ret_3: i32 = 0;
    let r_ = 'f: {
    l_ret_3 = 0i32;
    if ((((!(((((((({ let a_ = ((c.m.r32(sym::gSaveData)).wrapping_add(11976)).wrapping_add(((p_item_1) as u32)); c.m.r8(a_) }) as i32)) & (1i32))) as u8)) != 0)) as i32)) != 0) {
        if ((((!(((((c.m.r32(((p_table_2).wrapping_add((((p_item_1) as u32)).wrapping_mul(40))).wrapping_add(20)) as i32)) & (5i32))) != 0)) as i32)) != 0) {
            if (((((c.m.r32(((p_table_2).wrapping_add((((p_item_1) as u32)).wrapping_mul(40))).wrapping_add(20)) as i32)) & (32i32))) != 0) {
                l_ret_3 = 1i32;
            }
        }
    }
    break 'f l_ret_3;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Shop_CanBuy", &PROBE_N, &[], true); }
    r_
}

pub fn Shop_PlayVoice(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Shop_PlayVoice", &PROBE_N, &[], false); }
    let r_ = 'f: {
    's1: {
        let sv1: i32 = { let a_ = ((c.m.r32(sym::gShop)).wrapping_add(564)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
        match sv1 {
            0i32 => {
                { let v_ = (({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(604); (c.m.r32(a_) as i32) }).wrapping_mul(15i32)).wrapping_add(1i32); let a_ = (c.m.r32(sym::gShop)).wrapping_add(600); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            1i32 => {
                { let v_ = (({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(604); (c.m.r32(a_) as i32) }).wrapping_mul(15i32)).wrapping_add(3i32); let a_ = (c.m.r32(sym::gShop)).wrapping_add(600); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            _ => {
            }
        }
    }
    { let a0_ = { let a_ = (c.m.r32(sym::gShop)).wrapping_add(12); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gShop)).wrapping_add(600); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34523i32, a2_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Shop_PlayVoice", &PROBE_N, &[], true); }
    r_
}

pub fn Shop_SetListScissor(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Shop_SetListScissor", &PROBE_N, &[], false); }
    let r_ = 'f: {
    's1: {
        let sv1: i32 = { let a_ = ((c.m.r32(sym::gShop)).wrapping_add(564)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
        match sv1 {
            0i32 => {
                F::Sprite_SetScissor(c, 0i32, 512i32, 134i32, 278i32);
                break 's1;
            }
            1i32 => {
                F::Sprite_SetScissor(c, 0i32, 512i32, 134i32, 383i32);
                break 's1;
            }
            _ => {
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Shop_SetListScissor", &PROBE_N, &[], true); }
    r_
}

pub fn Shop_SetWindowScissor(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Shop_SetWindowScissor", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::Sprite_SetScissor(c, 164i32, 449i32, 61i32, 301i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Shop_SetWindowScissor", &PROBE_N, &[], true); }
    r_
}

pub fn Shop_ResetScissor(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Shop_ResetScissor", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::Sprite_SetScissor(c, 0i32, 511i32, 0i32, 447i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Shop_ResetScissor", &PROBE_N, &[], true); }
    r_
}

pub fn Shop_SwapGuide(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Shop_SwapGuide", &PROBE_N, &[], false); }
    let mut l_res_1: u32 = 0;
    let r_ = 'f: {
    { let r_ = 1i32; let a_ = (c.m.r32(sym::gShop)).wrapping_add(604); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
    l_res_1 = { let a_ = ((c.m.r32(sym::gShop)).wrapping_add(288)).wrapping_add(((({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(604); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.r32(a_) };
    's1: {
        let sv2: i32 = { let a_ = (c.m.r32(sym::gShop)).wrapping_add(604); (c.m.r32(a_) as i32) };
        match sv2 {
            0i32 => {
                { let v_ = (c.m.r32((l_res_1).wrapping_add(16))).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gShop)).wrapping_add(356)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                { let v_ = (c.m.r32((l_res_1).wrapping_add(16))).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gShop)).wrapping_add(356)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                { let v_ = (c.m.r32((l_res_1).wrapping_add(16))).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gShop)).wrapping_add(356)).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gShop)).wrapping_add(356)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gShop)).wrapping_add(356)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                break 's1;
            }
            1i32 => {
                { let v_ = (c.m.r32((l_res_1).wrapping_add(16))).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gShop)).wrapping_add(356)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                { let v_ = (c.m.r32((l_res_1).wrapping_add(16))).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gShop)).wrapping_add(356)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                { let v_ = (c.m.r32((l_res_1).wrapping_add(16))).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gShop)).wrapping_add(356)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gShop)).wrapping_add(356)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gShop)).wrapping_add(356)).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                break 's1;
            }
            _ => {
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Shop_SwapGuide", &PROBE_N, &[], true); }
    r_
}

pub fn Shop_Init(c: &mut Ctx, mut p_section_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Shop_Init", &PROBE_N, &[(p_section_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_k_2: i32 = 0;
    let mut l_i_3: i32 = 0;
    let mut l_flags_4: i32 = 0;
    let r_ = 'f: {
        { let v_ = ((0i32) as u32); c.m.w32(fp, v_); }
        l_k_2 = 0i32;
        { let v_ = F::Heap_Alloc(c, ((13968u32) as i32), ((32i32) as u32), 0i32, 2i32); let a_ = sym::gShop; c.m.w32(a_, v_); };
        { let _ = { let a0_ = c.m.r32(sym::gShop); F::memset(c, a0_, 0i32, 13968u32) }; };
        { let v_ = (c.m.r32(sym::gMenuArc7)).wrapping_add((({ let a_ = (c.m.r32(sym::gMenuArc7)).wrapping_add((((p_section_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = c.m.r32(sym::gShop); c.m.w32(a_, v_); };
        { let v_ = { let a0_ = { let a_ = c.m.r32(sym::gShop); c.m.r32(a_) }; F::Sprite_Unpack(c, a0_, ((0i32) as u32), ((0i32) as u32)) }; let a_ = (c.m.r32(sym::gShop)).wrapping_add(4); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = c.m.r32(fp); let a_ = (c.m.r32(sym::gShop)).wrapping_add(284); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((31i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gShop)).wrapping_add(296)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gShop)).wrapping_add(296)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gShop)).wrapping_add(296)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gShop)).wrapping_add(296)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gShop)).wrapping_add(296)).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gShop)).wrapping_add(296)).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gShop)).wrapping_add(296)).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gShop)).wrapping_add(296)).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gShop)).wrapping_add(296)).wrapping_add((((8i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gShop)).wrapping_add(296)).wrapping_add((((9i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gShop)).wrapping_add(296)).wrapping_add((((13i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gShop)).wrapping_add(296)).wrapping_add((((14i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gShop)).wrapping_add(296)).wrapping_add((((10i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gShop)).wrapping_add(296)).wrapping_add((((12i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gShop)).wrapping_add(296)).wrapping_add((((11i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let a0_ = ((c.m.r32(sym::gShop)).wrapping_add(20)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); let a1_ = ({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a2_ = (c.m.r32(sym::gShop)).wrapping_add(296); F::Flash_Create(c, a0_, a1_, a2_) };
        { let a0_ = ((c.m.r32(sym::gShop)).wrapping_add(20)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_Play(c, a0_, 1i32) };
        { let v_ = ({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = c.m.r32(fp); let a_ = ((c.m.r32(sym::gShop)).wrapping_add(288)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gShop)).wrapping_add(356)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gShop)).wrapping_add(356)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gShop)).wrapping_add(356)).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((8i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = c.m.r32(fp); let a_ = ((c.m.r32(sym::gShop)).wrapping_add(288)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let a0_ = ((c.m.r32(sym::gShop)).wrapping_add(20)).wrapping_add((((3i32) as u32)).wrapping_mul(44)); let a1_ = ({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((9i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a2_ = (c.m.r32(sym::gShop)).wrapping_add(356); F::Flash_Create(c, a0_, a1_, a2_) };
        { let a0_ = ((c.m.r32(sym::gShop)).wrapping_add(20)).wrapping_add((((3i32) as u32)).wrapping_mul(44)); F::Flash_Play(c, a0_, 1i32) };
        { let v_ = ({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((10i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gShop)).wrapping_add(376)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((11i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gShop)).wrapping_add(376)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gShop)).wrapping_add(376)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gShop)).wrapping_add(376)).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gShop)).wrapping_add(460)).wrapping_add((((14i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gShop)).wrapping_add(460)).wrapping_add((((15i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gShop)).wrapping_add(460)).wrapping_add((((16i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((12i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gShop)).wrapping_add(376)).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gShop)).wrapping_add(376)).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gShop)).wrapping_add(376)).wrapping_add((((18i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((13i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gShop)).wrapping_add(376)).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gShop)).wrapping_add(376)).wrapping_add((((10i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gShop)).wrapping_add(376)).wrapping_add((((8i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gShop)).wrapping_add(376)).wrapping_add((((11i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((4i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gShop)).wrapping_add(376)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gShop)).wrapping_add(460)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gShop)).wrapping_add(460)).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gShop)).wrapping_add(460)).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gShop)).wrapping_add(460)).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((4i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gShop)).wrapping_add(460)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((14i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gShop)).wrapping_add(376)).wrapping_add((((12i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gShop)).wrapping_add(376)).wrapping_add((((9i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gShop)).wrapping_add(376)).wrapping_add((((19i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gShop)).wrapping_add(460)).wrapping_add((((8i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gShop)).wrapping_add(460)).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gShop)).wrapping_add(556)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((15i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gShop)).wrapping_add(376)).wrapping_add((((17i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gShop)).wrapping_add(376)).wrapping_add((((16i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gShop)).wrapping_add(460)).wrapping_add((((13i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gShop)).wrapping_add(460)).wrapping_add((((12i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((16i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gShop)).wrapping_add(376)).wrapping_add((((13i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gShop)).wrapping_add(376)).wrapping_add((((14i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gShop)).wrapping_add(460)).wrapping_add((((9i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gShop)).wrapping_add(460)).wrapping_add((((10i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((24i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gShop)).wrapping_add(376)).wrapping_add((((20i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gShop)).wrapping_add(460)).wrapping_add((((17i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let a0_ = ((c.m.r32(sym::gShop)).wrapping_add(20)).wrapping_add((((1i32) as u32)).wrapping_mul(44)); let a1_ = ({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((17i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a2_ = (c.m.r32(sym::gShop)).wrapping_add(376); F::Flash_Create(c, a0_, a1_, a2_) };
        { let a0_ = ((c.m.r32(sym::gShop)).wrapping_add(20)).wrapping_add((((1i32) as u32)).wrapping_mul(44)); F::Flash_Play(c, a0_, 1i32) };
        { let v_ = ({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((18i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gShop)).wrapping_add(460)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gShop)).wrapping_add(460)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let a0_ = ((c.m.r32(sym::gShop)).wrapping_add(20)).wrapping_add((((2i32) as u32)).wrapping_mul(44)); let a1_ = ({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((19i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a2_ = (c.m.r32(sym::gShop)).wrapping_add(460); F::Flash_Create(c, a0_, a1_, a2_) };
        { let a0_ = ((c.m.r32(sym::gShop)).wrapping_add(20)).wrapping_add((((2i32) as u32)).wrapping_mul(44)); F::Flash_Play(c, a0_, 1i32) };
        { let v_ = ({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((27i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gShop)).wrapping_add(532)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gShop)).wrapping_add(532)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gShop)).wrapping_add(532)).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gShop)).wrapping_add(532)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((28i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gShop)).wrapping_add(532)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gShop)).wrapping_add(532)).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let a0_ = ((c.m.r32(sym::gShop)).wrapping_add(20)).wrapping_add((((4i32) as u32)).wrapping_mul(44)); let a1_ = ({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((29i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a2_ = (c.m.r32(sym::gShop)).wrapping_add(532); F::Flash_Create(c, a0_, a1_, a2_) };
        { let a0_ = ((c.m.r32(sym::gShop)).wrapping_add(20)).wrapping_add((((4i32) as u32)).wrapping_mul(44)); F::Flash_Play(c, a0_, 1i32) };
        { let a0_ = ((c.m.r32(sym::gShop)).wrapping_add(20)).wrapping_add((((5i32) as u32)).wrapping_mul(44)); let a1_ = ({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((33i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a2_ = (c.m.r32(sym::gShop)).wrapping_add(556); F::Flash_Create(c, a0_, a1_, a2_) };
        { let a0_ = ((c.m.r32(sym::gShop)).wrapping_add(20)).wrapping_add((((5i32) as u32)).wrapping_mul(44)); F::Flash_Play(c, a0_, 1i32) };
        { let v_ = ({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((20i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let a0_ = ({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((22i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a1_ = c.m.r32(fp); F::IconWin_Init(c, a0_, a1_) };
        F::IconWin_Open(c);
        { let v_ = ({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((32i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gShop)).wrapping_add(8); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((26i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gShop)).wrapping_add(12); c.m.w32(a_, v_); };
        { let a0_ = ({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((21i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a1_ = { let a_ = (c.m.r32(sym::gShop)).wrapping_add(8); c.m.r32(a_) }; let _ = 0i32; F::MsgWin_Init(c, a0_, a1_, 1i32) };
        F::MsgWin_Open(c);
        { let a0_ = ({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((25i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); F::ItemHelp_Init(c, a0_) };
        { let v_ = ({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((30i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gShop)).wrapping_add(16); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = ((c.m.r32(sym::gCommonRes)).wrapping_add(4)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = ((c.m.r32(sym::gCommonRes)).wrapping_add(4)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gShop)).wrapping_add(12004); c.m.w32(a_, v_); };
        F::Shop_CheckStockLevel(c);
        {
            'l1: loop {
                if !(((((l_k_2) < (2i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    {
                        l_i_3 = 0i32;
                        'l3: loop {
                            if !(((((l_i_3) < (350i32)) as i32)) != 0) { break 'l3; }
                            'c4: {
                                l_flags_4 = { let a_ = (({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(12004); c.m.r32(a_) }).wrapping_add((((l_i_3) as u32)).wrapping_mul(40))).wrapping_add(20); (c.m.r32(a_) as i32) };
                                if ((((l_flags_4) & (5i32))) != 0) {
                                    break 'c4;
                                }
                                if ((((((({ let a_ = ({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(12004); c.m.r32(a_) }).wrapping_add((((l_i_3) as u32)).wrapping_mul(40)); c.m.r8(a_) }) as i32)) >= (3i32)) as i32)) != 0) {
                                    break 'c4;
                                }
                                if ((((!((((l_flags_4) & (64i32))) != 0)) as i32)) != 0) {
                                    break 'c4;
                                }
                                if (((((l_k_2) == (0i32)) as i32)) != 0) {
                                    if ((((!((((l_flags_4) & (32i32))) != 0)) as i32)) != 0) {
                                        break 'c4;
                                    }
                                    if ((((((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(4108); (c.m.r32(a_) as i32) }) as u32)) < ({ let a_ = (({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(12004); c.m.r32(a_) }).wrapping_add((((l_i_3) as u32)).wrapping_mul(40))).wrapping_add(8); c.m.r32(a_) })) as i32)) != 0) {
                                        break 'c4;
                                    }
                                }
                                { let v_ = l_i_3; let a_ = (((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_2) as u32)).wrapping_mul(5680))).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(1400))).wrapping_add(((({ let a_ = ((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_2) as u32)).wrapping_mul(5680))).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                { let a_ = ((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_2) as u32)).wrapping_mul(5680))).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                                { let v_ = l_i_3; let a_ = (((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_2) as u32)).wrapping_mul(5680))).wrapping_add(48)).wrapping_add((((((({ let a_ = ({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(12004); c.m.r32(a_) }).wrapping_add((((l_i_3) as u32)).wrapping_mul(40)); c.m.r8(a_) }) as i32)).wrapping_add(1i32)) as u32)).wrapping_mul(1400))).wrapping_add(((({ let a_ = ((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_2) as u32)).wrapping_mul(5680))).wrapping_add(32)).wrapping_add((((((({ let a_ = ({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(12004); c.m.r32(a_) }).wrapping_add((((l_i_3) as u32)).wrapping_mul(40)); c.m.r8(a_) }) as i32)).wrapping_add(1i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                { let a_ = ((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_2) as u32)).wrapping_mul(5680))).wrapping_add(32)).wrapping_add((((((({ let a_ = ({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(12004); c.m.r32(a_) }).wrapping_add((((l_i_3) as u32)).wrapping_mul(40)); c.m.r8(a_) }) as i32)).wrapping_add(1i32)) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                                's5: {
                                    let sv5: i32 = l_k_2;
                                    match sv5 {
                                        0i32 => {
                                            break 's5;
                                        }
                                        1i32 => {
                                            if ((((!(((({ let a_ = (({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(12004); c.m.r32(a_) }).wrapping_add((((l_i_3) as u32)).wrapping_mul(40))).wrapping_add(20); (c.m.r32(a_) as i32) }) & (256i32))) != 0)) as i32)) != 0) {
                                                { let a_ = (c.m.r32(sym::gShop)).wrapping_add(628); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                                                if (((((((({ let a_ = ((c.m.r32(sym::gSaveData)).wrapping_add(11976)).wrapping_add(((l_i_3) as u32)); c.m.r8(a_) }) as i32)) & (1i32))) as u8)) != 0) {
                                                    { let a_ = (c.m.r32(sym::gShop)).wrapping_add(632); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                                                }
                                            }
                                            break 's5;
                                        }
                                        _ => {
                                        }
                                    }
                                }
                            }
                            { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
                        }
                    }
                    { let v_ = (if (((((l_k_2) != (0i32)) as i32)) != 0) { 7i32 } else { 4i32 }); let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_2) as u32)).wrapping_mul(5680))).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
                    { let v_ = (if (((((l_k_2) != (0i32)) as i32)) != 0) { F32(0x40e8f5c2) } else { F32(0x40800000) }); let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_2) as u32)).wrapping_mul(5680))).wrapping_add(24); c.m.w32(a_, (v_).0); };
                    {
                        l_i_3 = 0i32;
                        'l6: loop {
                            if !(((((l_i_3) < (4i32)) as i32)) != 0) { break 'l6; }
                            'c7: {
                                { let v_ = 0i32; let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_2) as u32)).wrapping_mul(5680))).wrapping_add((((l_i_3) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                { let v_ = { let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_2) as u32)).wrapping_mul(5680))).wrapping_add((((l_i_3) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_2) as u32)).wrapping_mul(5680))).wrapping_add(5648)).wrapping_add((((l_i_3) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                { let v_ = (({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_2) as u32)).wrapping_mul(5680))).wrapping_add((((l_i_3) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_2) as u32)).wrapping_mul(5680))).wrapping_add(20); (c.m.r32(a_) as i32) })).wrapping_sub(1i32); let a_ = ((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_2) as u32)).wrapping_mul(5680))).wrapping_add(5664)).wrapping_add((((l_i_3) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                            }
                            { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
                        }
                    }
                }
                { let o_ = l_k_2; let n_ = (o_).wrapping_add(1); l_k_2 = n_; };
            }
        }
        { let v_ = rt::f2i(rt::fmul(rt::fdiv(rt::i2f({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(632); (c.m.r32(a_) as i32) }), rt::i2f({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(628); (c.m.r32(a_) as i32) })), F32(0x42c80000))); let a_ = (c.m.r32(sym::gShop)).wrapping_add(636); c.m.w32(a_, (v_) as u32); };
        if ((((((((({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(632); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) && ((((({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(636); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0)) as i32)) != 0) {
            { let v_ = 1i32; let a_ = (c.m.r32(sym::gShop)).wrapping_add(636); c.m.w32(a_, (v_) as u32); };
        }
        { let v_ = ((F::Rand_Range(c, ((2i32) as u32))) as i32); let a_ = (c.m.r32(sym::gShop)).wrapping_add(604); c.m.w32(a_, (v_) as u32); };
        F::Shop_SwapGuide(c);
        { let v_ = (1i32).wrapping_neg(); let a_ = (c.m.r32(sym::gShop)).wrapping_add(600); c.m.w32(a_, (v_) as u32); };
        {
            l_k_2 = 0i32;
            'l8: loop {
                if !(((((l_k_2) < (14i32)) as i32)) != 0) { break 'l8; }
                'c9: {
                    { let a0_ = ((c.m.r32(sym::gShop)).wrapping_add(12008)).wrapping_add((((l_k_2) as u32)).wrapping_mul(140)); let a1_ = { let a_ = (c.m.r32(sym::gShop)).wrapping_add(16); c.m.r32(a_) }; F::TextBox_Init(c, a0_, a1_, ((6i32) as u32)) };
                    { let a0_ = ((c.m.r32(sym::gShop)).wrapping_add(12008)).wrapping_add((((l_k_2) as u32)).wrapping_mul(140)); F::TextBox_SetUnk80(c, a0_, 1i32) };
                    if (((((l_k_2) < (8i32)) as i32)) != 0) {
                        { let a0_ = ((c.m.r32(sym::gShop)).wrapping_add(12008)).wrapping_add((((l_k_2) as u32)).wrapping_mul(140)); F::TextBox_SetRect(c, a0_, 0i32, 512i32, 134i32, 383i32) };
                    } else {
                        if (((((l_k_2) < (13i32)) as i32)) != 0) {
                            { let a0_ = ((c.m.r32(sym::gShop)).wrapping_add(12008)).wrapping_add((((l_k_2) as u32)).wrapping_mul(140)); F::TextBox_SetRect(c, a0_, 0i32, 512i32, 134i32, 278i32) };
                        }
                    }
                }
                { let o_ = l_k_2; let n_ = (o_).wrapping_add(1); l_k_2 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Shop_Init", &PROBE_N, &[], true); }
    r_
}

pub fn Shop_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Shop_Term", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let r_ = 'f: {
    F::ItemHelp_Term(c);
    F::MsgWin_Term(c);
    F::IconWin_Term(c);
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (6i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let a0_ = ((c.m.r32(sym::gShop)).wrapping_add(20)).wrapping_add((((l_i_1) as u32)).wrapping_mul(44)); F::Flash_Destroy(c, a0_) };
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
    if ((((({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(4); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = { let a_ = (c.m.r32(sym::gShop)).wrapping_add(4); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = (c.m.r32(sym::gShop)).wrapping_add(4); c.m.w32(a_, v_); };
    }
    if (((((c.m.r32(sym::gShop)) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32(sym::gShop); F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = sym::gShop; c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Shop_Term", &PROBE_N, &[], true); }
    r_
}

pub fn Shop_Draw(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Shop_Draw", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(96) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 96);
    let mut l_flash_1: u32 = 0;
    let mut l_i_2: i32 = 0;
    let mut l_k_3: i32 = 0;
    let mut l_count_4: i32 = 0;
    let mut l_rows_5: i32 = 0;
    let mut l_pos_6: i32 = 0;
    let mut l_scale_7: F32 = F32(0);
    let r_ = 'f: {
        { let a0_ = { let a_ = (c.m.r32(sym::gShop)).wrapping_add(284); c.m.r32(a_) }; F::Sprite_DrawPicture(c, a0_, 0i32, 0i32, 128i32) };
        l_flash_1 = ((c.m.r32(sym::gShop)).wrapping_add(20)).wrapping_add((((0i32) as u32)).wrapping_mul(44));
        {
            l_i_2 = 0i32;
            'l1: loop {
                if !(((((l_i_2) < (2i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = (l_i_2).wrapping_mul(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 512i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = ((c.m.r32((fp.wrapping_add(8)).wrapping_add(4)) as i32)).wrapping_add(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                    { let _ = F::sprintf(c, fp.wrapping_add(28), sym::menu_v_d__str0, &[(((l_i_2).wrapping_add(1i32)) as i64 as u64)]); };
                    F::Flash_FindLabel(c, l_flash_1, fp.wrapping_add(28), sym::menu_v_d__str1, fp);
                    F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(8));
                    F::Flash_FindLabel(c, l_flash_1, fp.wrapping_add(28), sym::menu_v_d__str2, fp);
                    F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(8));
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        { let a4_ = { let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(12328); (c.m.r32(a_) as i32) }; F::Num_Draw(c, l_flash_1, sym::menu_v_d__str3, 0i32, 7i32, a4_, 32i32, 32i32, 0i32) };
        { let a4_ = { let a_ = (c.m.r32(sym::gShop)).wrapping_add(636); (c.m.r32(a_) as i32) }; F::Num_Draw(c, l_flash_1, sym::menu_v_d__str4, 0i32, 3i32, a4_, 32i32, 32i32, 0i32) };
        F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_v_d__str5, fp);
        F::Flash_ClipSetCallbackA(c, l_flash_1, fp, fid::Shop_SetWindowScissor, ((0i32) as u32));
        F::Flash_ClipSetCallbackB(c, l_flash_1, fp, fid::Shop_ResetScissor, ((0i32) as u32));
        { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = 512i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 128i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        { let a3_ = (c.m.r32(sym::gShop)).wrapping_add(640); F::FlashAnim_Scroll(c, l_flash_1, fp, fp.wrapping_add(8), a3_, ((0i32) as u32), F32(0xbe11a2b3), F32(0x00000000)) };
        F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_v_d__str6, fp);
        F::Flash_ClipSetCallbackA(c, l_flash_1, fp, fid::Shop_SetWindowScissor, ((0i32) as u32));
        F::Flash_ClipSetCallbackB(c, l_flash_1, fp, fid::Shop_ResetScissor, ((0i32) as u32));
        l_flash_1 = ((c.m.r32(sym::gShop)).wrapping_add(20)).wrapping_add((((3i32) as u32)).wrapping_mul(44));
        { let a2_ = (if ((((({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(604); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) { sym::menu_v_d__str8 } else { sym::menu_v_d__str9 }); F::Flash_FindLabel(c, l_flash_1, sym::menu_v_d__str7, a2_, fp) };
        { let a2_ = (c.m.r32(sym::gShop)).wrapping_add(608); F::FlashAnim_Blink(c, l_flash_1, fp, a2_, 0i32) };
        { let a2_ = (if ((((({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(604); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) { sym::menu_v_d__str11 } else { sym::menu_v_d__str12 }); F::Flash_FindLabel(c, l_flash_1, sym::menu_v_d__str10, a2_, fp) };
        { let a2_ = (c.m.r32(sym::gShop)).wrapping_add(612); F::FlashAnim_Talk(c, l_flash_1, fp, a2_, 0i32) };
        {
            l_k_3 = 0i32;
            'l3: loop {
                if !(((((l_k_3) < (2i32)) as i32)) != 0) { break 'l3; }
                'c4: {
                    if ((((!(((({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(560); (c.m.r32(a_) as i32) }) & ((16i32).wrapping_shl((l_k_3) as u32)))) != 0)) as i32)) != 0) {
                        break 'c4;
                    }
                    l_flash_1 = (if (((((l_k_3) != (0i32)) as i32)) != 0) { ((c.m.r32(sym::gShop)).wrapping_add(20)).wrapping_add((((2i32) as u32)).wrapping_mul(44)) } else { ((c.m.r32(sym::gShop)).wrapping_add(20)).wrapping_add((((1i32) as u32)).wrapping_mul(44)) });
                    {
                        l_i_2 = 0i32;
                        'l5: loop {
                            if !(((((l_i_2) < (4i32)) as i32)) != 0) { break 'l5; }
                            'c6: {
                                { let v_ = (l_i_2).wrapping_mul(64i32); let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                                { let v_ = 0i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                                { let v_ = ((c.m.r32(fp.wrapping_add(8)) as i32)).wrapping_add(64i32); let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                                { let v_ = 64i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                                { let _ = F::sprintf(c, fp.wrapping_add(28), sym::menu_v_d__str13, &[((l_i_2) as i64 as u64)]); };
                                F::Flash_FindLabel(c, l_flash_1, fp.wrapping_add(28), sym::menu_v_d__str14, fp);
                                F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(8));
                                F::Flash_FindLabel(c, l_flash_1, fp.wrapping_add(28), sym::menu_v_d__str15, fp);
                                F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(8));
                                F::Flash_FindLabel(c, l_flash_1, fp.wrapping_add(28), sym::menu_v_d__str16, fp);
                                { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
                            }
                            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
                        }
                    }
                    { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = ({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }).wrapping_mul(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 256i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = ((c.m.r32((fp.wrapping_add(8)).wrapping_add(4)) as i32)).wrapping_add(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                    F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_v_d__str17, fp);
                    F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(8));
                    if (((((l_k_3) == (0i32)) as i32)) != 0) {
                        F::Flash_FindLabel(c, l_flash_1, sym::menu_v_d__str18, sym::menu_v_d__str19, fp);
                        F::Flash_ClipSetTex(c, l_flash_1, fp, 2i32);
                        { let v_ = ((({ let a_ = ({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(12004); c.m.r32(a_) }).wrapping_add(((({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(624); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(40)); c.m.r8(a_) }) as i32)).wrapping_mul(64i32); let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                        { let v_ = 0i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                        { let v_ = ((c.m.r32(fp.wrapping_add(8)) as i32)).wrapping_add(64i32); let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                        { let v_ = 64i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                        F::Flash_FindLabel(c, l_flash_1, sym::menu_v_d__str20, sym::menu_v_d__str21, fp);
                        F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(8));
                        F::Flash_FindLabel(c, l_flash_1, sym::menu_v_d__str22, sym::menu_v_d__str23, fp);
                        if ((((((({ let a_ = (({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(12004); c.m.r32(a_) }).wrapping_add(((({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(624); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(40))).wrapping_add(3); c.m.r8(a_) }) as i32)) != (0i32)) as i32)) != 0) {
                            { let v_ = (((({ let a_ = (({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(12004); c.m.r32(a_) }).wrapping_add(((({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(624); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(40))).wrapping_add(3); c.m.r8(a_) }) as i32)).wrapping_mul(32i32)).wrapping_sub(32i32); let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                            { let v_ = ({ let a0_ = { let a_ = (c.m.r32(sym::gShop)).wrapping_add(624); (c.m.r32(a_) as i32) }; let a1_ = { let a_ = (c.m.r32(sym::gShop)).wrapping_add(12004); c.m.r32(a_) }; F::ItemTbl_GetClass(c, a0_, a1_) }).wrapping_mul(42i32); let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                            { let v_ = ((c.m.r32(fp.wrapping_add(8)) as i32)).wrapping_add(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                            { let v_ = ((c.m.r32((fp.wrapping_add(8)).wrapping_add(4)) as i32)).wrapping_add(42i32); let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                            F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(8));
                            { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((1i32) as i64 as u64)) as u8)); };
                        } else {
                            { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
                        }
                        F::Flash_FindLabel(c, l_flash_1, sym::menu_v_d__str24, sym::menu_v_d__str25, fp);
                        { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
                        F::Flash_FindLabel(c, l_flash_1, sym::menu_v_d__str26, sym::menu_v_d__str27, fp);
                        { let a4_ = { let a_ = (c.m.r32(sym::gShop)).wrapping_add(624); (c.m.r32(a_) as i32) }; let a5_ = ((c.m.r32(sym::gShop)).wrapping_add(12008)).wrapping_add((((13i32) as u32)).wrapping_mul(140)); F::TextBox_AttachLine(c, l_flash_1, fp, 0i32, 0i32, a4_, a5_) };
                    }
                    {
                        l_i_2 = 0i32;
                        'l7: loop {
                            if !(((((l_i_2) < ({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(20); (c.m.r32(a_) as i32) })) as i32)) != 0) { break 'l7; }
                            'c8: {
                                { let _ = F::sprintf(c, fp.wrapping_add(28), sym::menu_v_d__str28, &[(((l_i_2).wrapping_add(1i32)) as i64 as u64)]); };
                                F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), fp.wrapping_add(28), fp);
                                { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 1024i32, (((((((l_k_3) != (0i32)) as i32)) as i64 as u64)) as u8)); };
                                if (((((((((l_i_2) == (0i32)) as i32)) != 0) || (((((l_i_2) == (({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(20); (c.m.r32(a_) as i32) }).wrapping_sub(1i32))) as i32)) != 0)) as i32)) != 0) {
                                    F::Flash_ClipSetCallbackA(c, l_flash_1, fp, fid::Shop_SetListScissor, ((0i32) as u32));
                                    F::Flash_ClipSetCallbackB(c, l_flash_1, fp, fid::Shop_ResetScissor, ((0i32) as u32));
                                }
                                's9: {
                                    let sv8: i32 = l_k_3;
                                    match sv8 {
                                        0i32 => {
                                            if (((((({ let a_ = ((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(5648)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(l_i_2)) >= ({ let a_ = ((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(32)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                                                F::Flash_ClipSetColor(c, l_flash_1, fp, F32(0x3f800000));
                                                F::Flash_FindLabel(c, l_flash_1, fp.wrapping_add(28), sym::menu_v_d__str29, fp);
                                                F::Flash_ClipSetTex(c, l_flash_1, fp, 7i32);
                                                F::Flash_FindLabel(c, l_flash_1, fp.wrapping_add(28), sym::menu_v_d__str30, fp);
                                                { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
                                                F::Flash_FindLabel(c, l_flash_1, fp.wrapping_add(28), sym::menu_v_d__str31, fp);
                                                { let a5_ = ((c.m.r32(sym::gShop)).wrapping_add(12008)).wrapping_add(((((8i32).wrapping_add(l_i_2)) as u32)).wrapping_mul(140)); F::TextBox_AttachLine(c, l_flash_1, fp, 0i32, 0i32, (1i32).wrapping_neg(), a5_) };
                                                F::Flash_FindLabel(c, l_flash_1, fp.wrapping_add(28), sym::menu_v_d__str32, fp);
                                                { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
                                            } else {
                                                if (({ let a0_ = { let a_ = (((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(48)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(1400))).wrapping_add((((({ let a_ = ((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(5648)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(l_i_2)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a1_ = { let a_ = (c.m.r32(sym::gShop)).wrapping_add(12004); c.m.r32(a_) }; F::Shop_CanBuy(c, a0_, a1_) }) != 0) {
                                                    F::Flash_ClipSetColor(c, l_flash_1, fp, F32(0x3f800000));
                                                } else {
                                                    F::Flash_ClipSetColor(c, l_flash_1, fp, F32(0x3ecccccc));
                                                }
                                                F::Flash_FindLabel(c, l_flash_1, fp.wrapping_add(28), sym::menu_v_d__str33, fp);
                                                F::Flash_ClipSetTex(c, l_flash_1, fp, 2i32);
                                                F::Flash_FindLabel(c, l_flash_1, fp.wrapping_add(28), sym::menu_v_d__str34, fp);
                                                if ((((((({ let a_ = (({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(12004); c.m.r32(a_) }).wrapping_add(((({ let a_ = (((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(48)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(1400))).wrapping_add((((({ let a_ = ((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(5648)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(l_i_2)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(40))).wrapping_add(3); c.m.r8(a_) }) as i32)) != (0i32)) as i32)) != 0) {
                                                    { let v_ = (((({ let a_ = (({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(12004); c.m.r32(a_) }).wrapping_add(((({ let a_ = (((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(48)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(1400))).wrapping_add((((({ let a_ = ((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(5648)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(l_i_2)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(40))).wrapping_add(3); c.m.r8(a_) }) as i32)).wrapping_mul(32i32)).wrapping_sub(32i32); let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                                                    { let v_ = ({ let a0_ = { let a_ = (((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(48)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(1400))).wrapping_add((((({ let a_ = ((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(5648)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(l_i_2)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a1_ = { let a_ = (c.m.r32(sym::gShop)).wrapping_add(12004); c.m.r32(a_) }; F::ItemTbl_GetClass(c, a0_, a1_) }).wrapping_mul(42i32); let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                                                    { let v_ = ((c.m.r32(fp.wrapping_add(8)) as i32)).wrapping_add(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                                                    { let v_ = ((c.m.r32((fp.wrapping_add(8)).wrapping_add(4)) as i32)).wrapping_add(42i32); let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                                                    F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(8));
                                                    { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((1i32) as i64 as u64)) as u8)); };
                                                } else {
                                                    { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
                                                }
                                                F::Flash_FindLabel(c, l_flash_1, fp.wrapping_add(28), sym::menu_v_d__str35, fp);
                                                { let a4_ = { let a_ = (((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(48)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(1400))).wrapping_add((((({ let a_ = ((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(5648)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(l_i_2)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a5_ = ((c.m.r32(sym::gShop)).wrapping_add(12008)).wrapping_add(((((8i32).wrapping_add(l_i_2)) as u32)).wrapping_mul(140)); F::TextBox_AttachLine(c, l_flash_1, fp, 0i32, 0i32, a4_, a5_) };
                                                { let v_ = ((({ let a_ = ({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(12004); c.m.r32(a_) }).wrapping_add(((({ let a_ = (((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(48)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(1400))).wrapping_add((((({ let a_ = ((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(5648)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(l_i_2)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(40)); c.m.r8(a_) }) as i32)).wrapping_mul(64i32); let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                                                { let v_ = 0i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                                                { let v_ = ((c.m.r32(fp.wrapping_add(8)) as i32)).wrapping_add(64i32); let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                                                { let v_ = 64i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                                                F::Flash_FindLabel(c, l_flash_1, fp.wrapping_add(28), sym::menu_v_d__str36, fp);
                                                F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(8));
                                                { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((1i32) as i64 as u64)) as u8)); };
                                            }
                                            break 's9;
                                        }
                                        1i32 => {
                                            F::Flash_FindLabel(c, l_flash_1, fp.wrapping_add(28), sym::menu_v_d__str37, fp);
                                            if (((((((({ let a_ = ((c.m.r32(sym::gSaveData)).wrapping_add(11976)).wrapping_add((({ let a_ = (((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(48)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(1400))).wrapping_add((((({ let a_ = ((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(5648)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(l_i_2)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)); c.m.r8(a_) }) as i32)) & (1i32))) as u8)) != 0) {
                                                F::Flash_ClipSetTex(c, l_flash_1, fp, 2i32);
                                                F::Flash_FindLabel(c, l_flash_1, fp.wrapping_add(28), sym::menu_v_d__str38, fp);
                                                { let a4_ = { let a_ = (((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(48)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(1400))).wrapping_add((((({ let a_ = ((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(5648)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(l_i_2)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a5_ = ((c.m.r32(sym::gShop)).wrapping_add(12008)).wrapping_add((((l_i_2) as u32)).wrapping_mul(140)); F::TextBox_AttachLine(c, l_flash_1, fp, 0i32, 0i32, a4_, a5_) };
                                            } else {
                                                F::Flash_ClipSetTex(c, l_flash_1, fp, 7i32);
                                                F::Flash_FindLabel(c, l_flash_1, fp.wrapping_add(28), sym::menu_v_d__str39, fp);
                                                { let a5_ = ((c.m.r32(sym::gShop)).wrapping_add(12008)).wrapping_add((((l_i_2) as u32)).wrapping_mul(140)); F::TextBox_AttachLine(c, l_flash_1, fp, 0i32, 0i32, 350i32, a5_) };
                                            }
                                            F::Flash_FindLabel(c, l_flash_1, fp.wrapping_add(28), sym::menu_v_d__str40, fp);
                                            if ((((((((((({ let a_ = (({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(12004); c.m.r32(a_) }).wrapping_add(((({ let a_ = (((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(48)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(1400))).wrapping_add((((({ let a_ = ((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(5648)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(l_i_2)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(40))).wrapping_add(3); c.m.r8(a_) }) as i32)) != (0i32)) as i32)) != 0) && (((((((((({ let a_ = ((c.m.r32(sym::gSaveData)).wrapping_add(11976)).wrapping_add((({ let a_ = (((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(48)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(1400))).wrapping_add((((({ let a_ = ((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(5648)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(l_i_2)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)); c.m.r8(a_) }) as i32)) & (1i32))) as u8)) as i32)) != 0)) as i32)) != 0) {
                                                { let v_ = (((({ let a_ = (({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(12004); c.m.r32(a_) }).wrapping_add(((({ let a_ = (((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(48)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(1400))).wrapping_add((((({ let a_ = ((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(5648)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(l_i_2)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(40))).wrapping_add(3); c.m.r8(a_) }) as i32)).wrapping_mul(32i32)).wrapping_sub(32i32); let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                                                { let v_ = ({ let a0_ = { let a_ = (((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(48)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(1400))).wrapping_add((((({ let a_ = ((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(5648)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(l_i_2)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a1_ = { let a_ = (c.m.r32(sym::gShop)).wrapping_add(12004); c.m.r32(a_) }; F::ItemTbl_GetClass(c, a0_, a1_) }).wrapping_mul(42i32); let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                                                { let v_ = ((c.m.r32(fp.wrapping_add(8)) as i32)).wrapping_add(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                                                { let v_ = ((c.m.r32((fp.wrapping_add(8)).wrapping_add(4)) as i32)).wrapping_add(42i32); let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                                                F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(8));
                                                { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((1i32) as i64 as u64)) as u8)); };
                                            } else {
                                                { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
                                            }
                                            { let v_ = ((({ let a_ = ({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(12004); c.m.r32(a_) }).wrapping_add(((({ let a_ = (((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(48)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(1400))).wrapping_add((((({ let a_ = ((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(5648)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(l_i_2)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(40)); c.m.r8(a_) }) as i32)).wrapping_mul(64i32); let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                                            { let v_ = 0i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                                            { let v_ = ((c.m.r32(fp.wrapping_add(8)) as i32)).wrapping_add(64i32); let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                                            { let v_ = 64i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                                            F::Flash_FindLabel(c, l_flash_1, fp.wrapping_add(28), sym::menu_v_d__str41, fp);
                                            F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(8));
                                            break 's9;
                                        }
                                        _ => {
                                        }
                                    }
                                }
                                F::Flash_FindLabel(c, l_flash_1, fp.wrapping_add(28), sym::menu_v_d__str42, fp);
                                if (((((l_k_3) != (0i32)) as i32)) != 0) {
                                    if (((((({ let a_ = ((c.m.r32(sym::gSaveData)).wrapping_add(11976)).wrapping_add((({ let a_ = (((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(48)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(1400))).wrapping_add((((({ let a_ = ((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(5648)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(l_i_2)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)); c.m.r8(a_) }) as i32)) & (2i32))) != 0) {
                                        { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((1i32) as i64 as u64)) as u8)); };
                                    } else {
                                        { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
                                    }
                                } else {
                                    { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
                                }
                            }
                            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
                        }
                    }
                    { let _ = { let a2_ = ({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(20); (c.m.r32(a_) as i32) }).wrapping_add(1i32); F::sprintf(c, fp.wrapping_add(28), sym::menu_v_d__str43, &[((a2_) as i64 as u64)]) }; };
                    F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), fp.wrapping_add(28), fp);
                    { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 1024i32, (((((((l_k_3) != (0i32)) as i32)) as i64 as u64)) as u8)); };
                    F::Flash_ClipSetCallbackA(c, l_flash_1, fp, fid::Shop_SetListScissor, ((0i32) as u32));
                    F::Flash_ClipSetCallbackB(c, l_flash_1, fp, fid::Shop_ResetScissor, ((0i32) as u32));
                    's10: {
                        let sv9: i32 = l_k_3;
                        match sv9 {
                            0i32 => {
                                if ((((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(28); (c.m.r32(a_) as i32) }) >= ({ let a_ = ((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(32)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                                    F::Flash_FindLabel(c, l_flash_1, fp.wrapping_add(28), sym::menu_v_d__str44, fp);
                                    F::Flash_ClipSetTex(c, l_flash_1, fp, 7i32);
                                } else {
                                    if (({ let a0_ = { let a_ = (((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(48)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(1400))).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(28); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a1_ = { let a_ = (c.m.r32(sym::gShop)).wrapping_add(12004); c.m.r32(a_) }; F::Shop_CanBuy(c, a0_, a1_) }) != 0) {
                                        F::Flash_ClipSetColor(c, l_flash_1, fp, F32(0x3f800000));
                                    } else {
                                        F::Flash_ClipSetColor(c, l_flash_1, fp, F32(0x3ecccccc));
                                    }
                                    F::Flash_FindLabel(c, l_flash_1, fp.wrapping_add(28), sym::menu_v_d__str45, fp);
                                    F::Flash_ClipSetTex(c, l_flash_1, fp, 2i32);
                                }
                                F::Flash_FindLabel(c, l_flash_1, fp.wrapping_add(28), sym::menu_v_d__str46, fp);
                                { let a4_ = { let a_ = (((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(48)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(1400))).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(28); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a5_ = ((c.m.r32(sym::gShop)).wrapping_add(12008)).wrapping_add(((((8i32).wrapping_add({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(20); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(140)); F::TextBox_AttachLine(c, l_flash_1, fp, 0i32, 0i32, a4_, a5_) };
                                F::Flash_FindLabel(c, l_flash_1, fp.wrapping_add(28), sym::menu_v_d__str47, fp);
                                if ((((((({ let a_ = (({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(12004); c.m.r32(a_) }).wrapping_add(((({ let a_ = (((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(48)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(1400))).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(28); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(40))).wrapping_add(3); c.m.r8(a_) }) as i32)) != (0i32)) as i32)) != 0) {
                                    { let v_ = (((({ let a_ = (({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(12004); c.m.r32(a_) }).wrapping_add(((({ let a_ = (((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(48)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(1400))).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(28); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(40))).wrapping_add(3); c.m.r8(a_) }) as i32)).wrapping_mul(32i32)).wrapping_sub(32i32); let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                                    { let v_ = ({ let a0_ = { let a_ = (((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(48)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(1400))).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(28); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a1_ = { let a_ = (c.m.r32(sym::gShop)).wrapping_add(12004); c.m.r32(a_) }; F::ItemTbl_GetClass(c, a0_, a1_) }).wrapping_mul(42i32); let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                                    { let v_ = ((c.m.r32(fp.wrapping_add(8)) as i32)).wrapping_add(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                                    { let v_ = ((c.m.r32((fp.wrapping_add(8)).wrapping_add(4)) as i32)).wrapping_add(42i32); let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                                    F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(8));
                                    { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((1i32) as i64 as u64)) as u8)); };
                                } else {
                                    { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
                                }
                                break 's10;
                            }
                            1i32 => {
                                F::Flash_FindLabel(c, l_flash_1, fp.wrapping_add(28), sym::menu_v_d__str48, fp);
                                if (((((((({ let a_ = ((c.m.r32(sym::gSaveData)).wrapping_add(11976)).wrapping_add((({ let a_ = (((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(48)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(1400))).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(28); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)); c.m.r8(a_) }) as i32)) & (1i32))) as u8)) != 0) {
                                    F::Flash_ClipSetTex(c, l_flash_1, fp, 2i32);
                                    F::Flash_FindLabel(c, l_flash_1, fp.wrapping_add(28), sym::menu_v_d__str49, fp);
                                    { let a4_ = { let a_ = (((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(48)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(1400))).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(28); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a5_ = ((c.m.r32(sym::gShop)).wrapping_add(12008)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(20); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(140)); F::TextBox_AttachLine(c, l_flash_1, fp, 0i32, 0i32, a4_, a5_) };
                                } else {
                                    F::Flash_ClipSetTex(c, l_flash_1, fp, 7i32);
                                    F::Flash_FindLabel(c, l_flash_1, fp.wrapping_add(28), sym::menu_v_d__str50, fp);
                                    { let a5_ = ((c.m.r32(sym::gShop)).wrapping_add(12008)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(20); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(140)); F::TextBox_AttachLine(c, l_flash_1, fp, 0i32, 0i32, 350i32, a5_) };
                                }
                                F::Flash_FindLabel(c, l_flash_1, fp.wrapping_add(28), sym::menu_v_d__str51, fp);
                                if ((((((((((({ let a_ = (({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(12004); c.m.r32(a_) }).wrapping_add(((({ let a_ = (((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(48)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(1400))).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(28); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(40))).wrapping_add(3); c.m.r8(a_) }) as i32)) != (0i32)) as i32)) != 0) && (((((((((({ let a_ = ((c.m.r32(sym::gSaveData)).wrapping_add(11976)).wrapping_add((({ let a_ = (((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(48)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(1400))).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(28); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)); c.m.r8(a_) }) as i32)) & (1i32))) as u8)) as i32)) != 0)) as i32)) != 0) {
                                    { let v_ = (((({ let a_ = (({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(12004); c.m.r32(a_) }).wrapping_add(((({ let a_ = (((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(48)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(1400))).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(28); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(40))).wrapping_add(3); c.m.r8(a_) }) as i32)).wrapping_mul(32i32)).wrapping_sub(32i32); let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                                    { let v_ = ({ let a0_ = { let a_ = (((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(48)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(1400))).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(28); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a1_ = { let a_ = (c.m.r32(sym::gShop)).wrapping_add(12004); c.m.r32(a_) }; F::ItemTbl_GetClass(c, a0_, a1_) }).wrapping_mul(42i32); let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                                    { let v_ = ((c.m.r32(fp.wrapping_add(8)) as i32)).wrapping_add(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                                    { let v_ = ((c.m.r32((fp.wrapping_add(8)).wrapping_add(4)) as i32)).wrapping_add(42i32); let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                                    F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(8));
                                    { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((1i32) as i64 as u64)) as u8)); };
                                } else {
                                    { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
                                }
                                break 's10;
                            }
                            _ => {
                            }
                        }
                    }
                    { let v_ = ((({ let a_ = ({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(12004); c.m.r32(a_) }).wrapping_add(((({ let a_ = (((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(48)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(1400))).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(28); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(40)); c.m.r8(a_) }) as i32)).wrapping_mul(64i32); let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    { let v_ = ((c.m.r32(fp.wrapping_add(8)) as i32)).wrapping_add(64i32); let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 64i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                    F::Flash_FindLabel(c, l_flash_1, fp.wrapping_add(28), sym::menu_v_d__str52, fp);
                    F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(8));
                    F::Flash_FindLabel(c, l_flash_1, fp.wrapping_add(28), sym::menu_v_d__str53, fp);
                    if (((((l_k_3) != (0i32)) as i32)) != 0) {
                        if (((((({ let a_ = ((c.m.r32(sym::gSaveData)).wrapping_add(11976)).wrapping_add((({ let a_ = (((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(48)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(1400))).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(28); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)); c.m.r8(a_) }) as i32)) & (2i32))) != 0) {
                            { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((1i32) as i64 as u64)) as u8)); };
                        } else {
                            { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
                        }
                    } else {
                        { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
                    }
                    { let a4_ = { let a_ = (({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(12004); c.m.r32(a_) }).wrapping_add(((({ let a_ = (((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(48)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(1400))).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(40))).wrapping_add(4); (c.m.r32(a_) as i32) }; F::Num_Draw(c, l_flash_1, sym::menu_v_d__str54, 0i32, 7i32, a4_, 32i32, 32i32, 0i32) };
                    {
                        l_i_2 = 0i32;
                        'l11: loop {
                            if !(((((l_i_2) < (2i32)) as i32)) != 0) { break 'l11; }
                            'c12: {
                                { let v_ = (l_i_2).wrapping_mul(32i32); let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                                { let v_ = 32i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                                { let v_ = ((c.m.r32(fp.wrapping_add(8)) as i32)).wrapping_add(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                                { let v_ = 64i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                                F::Flash_FindLabel(c, l_flash_1, sym::menu_v_d__str55, (if (((((l_i_2) != (0i32)) as i32)) != 0) { sym::menu_v_d__str56 } else { sym::menu_v_d__str57 }), fp);
                                F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(8));
                                if (((((l_i_2) == (0i32)) as i32)) != 0) {
                                    { let a3_ = ((({ let a_ = ((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(5648)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) != (0i32)) as i32); { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((a3_) as i64 as u64)) as u8)); } };
                                } else {
                                    if ((((((((({ let a_ = ((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(32)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) < ({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(20); (c.m.r32(a_) as i32) })) as i32)) != 0) || ((((({ let a_ = ((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(5664)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) == (({ let a_ = ((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(32)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_sub(1i32))) as i32)) != 0)) as i32)) != 0) {
                                        { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
                                    } else {
                                        { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((1i32) as i64 as u64)) as u8)); };
                                    }
                                }
                            }
                            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
                        }
                    }
                    l_rows_5 = { let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(20); (c.m.r32(a_) as i32) };
                    l_count_4 = { let a_ = ((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(32)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
                    if (((((l_count_4) < (l_rows_5)) as i32)) != 0) {
                        l_count_4 = l_rows_5;
                    }
                    l_pos_6 = rt::f2i(rt::fdiv(rt::fmul(rt::fmul({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(24); F32(c.m.r32(a_)) }, F32(0x42000000)), rt::i2f({ let a_ = ((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(5648)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) })), rt::i2f(l_count_4)));
                    l_scale_7 = rt::fmul(rt::fdiv(rt::i2f(l_rows_5), rt::i2f(l_count_4)), { let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((l_k_3) as u32)).wrapping_mul(5680))).wrapping_add(24); F32(c.m.r32(a_)) });
                    F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_v_d__str58, fp);
                    F::Flash_ClipSetScale(c, l_flash_1, fp, F32(0x3f800000), l_scale_7);
                    F::Flash_ClipSetOffset(c, l_flash_1, fp, 0i32, l_pos_6);
                }
                { let o_ = l_k_3; let n_ = (o_).wrapping_add(1); l_k_3 = n_; };
            }
        }
        l_flash_1 = ((c.m.r32(sym::gShop)).wrapping_add(20)).wrapping_add((((4i32) as u32)).wrapping_mul(44));
        {
            l_i_2 = 0i32;
            'l13: loop {
                if !(((((l_i_2) < (3i32)) as i32)) != 0) { break 'l13; }
                'c14: {
                    { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = (l_i_2).wrapping_mul(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 256i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = ((c.m.r32((fp.wrapping_add(8)).wrapping_add(4)) as i32)).wrapping_add(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                    { let _ = F::sprintf(c, fp.wrapping_add(28), sym::menu_v_d__str59, &[(((l_i_2).wrapping_add(1i32)) as i64 as u64)]); };
                    F::Flash_FindLabel(c, l_flash_1, fp.wrapping_add(28), sym::menu_v_d__str60, fp);
                    F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(8));
                    F::Flash_FindLabel(c, l_flash_1, fp.wrapping_add(28), sym::menu_v_d__str61, fp);
                    F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(8));
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        F::IconWin_Draw(c);
        if (((({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(560); (c.m.r32(a_) as i32) }) & (128i32))) != 0) {
            { let a2_ = { let a_ = (c.m.r32(sym::gShop)).wrapping_add(600); (c.m.r32(a_) as i32) }; F::MsgWin_Draw(c, 0i32, 0i32, a2_) };
            { let a0_ = ((c.m.r32(sym::gShop)).wrapping_add(20)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_Draw(c, a0_) };
            { let a0_ = ((c.m.r32(sym::gShop)).wrapping_add(20)).wrapping_add((((2i32) as u32)).wrapping_mul(44)); F::Flash_Draw(c, a0_) };
            { let a0_ = ((c.m.r32(sym::gShop)).wrapping_add(20)).wrapping_add((((1i32) as u32)).wrapping_mul(44)); F::Flash_Draw(c, a0_) };
            { let a0_ = ((c.m.r32(sym::gShop)).wrapping_add(20)).wrapping_add((((3i32) as u32)).wrapping_mul(44)); F::Flash_Draw(c, a0_) };
        } else {
            { let a0_ = ((c.m.r32(sym::gShop)).wrapping_add(20)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_Draw(c, a0_) };
            { let a0_ = ((c.m.r32(sym::gShop)).wrapping_add(20)).wrapping_add((((2i32) as u32)).wrapping_mul(44)); F::Flash_Draw(c, a0_) };
            { let a0_ = ((c.m.r32(sym::gShop)).wrapping_add(20)).wrapping_add((((1i32) as u32)).wrapping_mul(44)); F::Flash_Draw(c, a0_) };
            { let a0_ = ((c.m.r32(sym::gShop)).wrapping_add(20)).wrapping_add((((3i32) as u32)).wrapping_mul(44)); F::Flash_Draw(c, a0_) };
            { let a2_ = { let a_ = (c.m.r32(sym::gShop)).wrapping_add(600); (c.m.r32(a_) as i32) }; F::MsgWin_Draw(c, 0i32, 0i32, a2_) };
        }
        F::Font_FlushAll(c);
        { let a0_ = ((c.m.r32(sym::gShop)).wrapping_add(20)).wrapping_add((((4i32) as u32)).wrapping_mul(44)); F::Flash_Draw(c, a0_) };
        { let a0_ = ((c.m.r32(sym::gShop)).wrapping_add(20)).wrapping_add((((5i32) as u32)).wrapping_mul(44)); F::Flash_Draw(c, a0_) };
        { let a0_ = { let a_ = (c.m.r32(sym::gShop)).wrapping_add(620); (c.m.r32(a_) as i32) }; F::ItemHelp_Draw(c, a0_) };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Shop_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn Shop_SetPlate(c: &mut Ctx, mut p_flash_1: i32, mut p_level_2: i32, mut p_tab_3: i32, mut p_label_4: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Shop_SetPlate", &PROBE_N, &[(p_flash_1) as u32, (p_level_2) as u32, (p_tab_3) as u32, (p_label_4) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(80) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 80);
    let mut l_fl_5: u32 = 0;
    let r_ = 'f: {
        l_fl_5 = ((c.m.r32(sym::gShop)).wrapping_add(20)).wrapping_add((((p_flash_1) as u32)).wrapping_mul(44));
        's1: {
            let sv6: i32 = p_level_2;
            match sv6 {
                0i32 => {
                    { let _ = { let a2_ = ({ let a_ = ((c.m.r32(sym::gShop)).wrapping_add(564)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(1i32); F::sprintf(c, fp.wrapping_add(8), sym::menu_v_d__str62, &[((a2_) as i64 as u64)]) }; };
                    break 's1;
                }
                1i32 => {
                    if (((((p_tab_3) != (0i32)) as i32)) != 0) {
                        { let _ = { let a2_ = { let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }; F::sprintf(c, fp.wrapping_add(8), sym::menu_v_d__str63, &[((a2_) as i64 as u64)]) }; };
                    } else {
                        { let _ = { let a2_ = (({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_sub({ let a_ = ((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(5648)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) })).wrapping_add(1i32); F::sprintf(c, fp.wrapping_add(8), sym::menu_v_d__str64, &[((a2_) as i64 as u64)]) }; };
                    }
                    break 's1;
                }
                2i32 => {
                    if (((((p_tab_3) != (0i32)) as i32)) != 0) {
                        { let _ = { let a2_ = { let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }; F::sprintf(c, fp.wrapping_add(8), sym::menu_v_d__str65, &[((a2_) as i64 as u64)]) }; };
                    } else {
                        { let _ = { let a2_ = (({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_sub({ let a_ = ((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(5648)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) })).wrapping_add(1i32); F::sprintf(c, fp.wrapping_add(8), sym::menu_v_d__str66, &[((a2_) as i64 as u64)]) }; };
                    }
                    break 's1;
                }
                4i32 => {
                    { let _ = { let a2_ = ({ let a_ = ((c.m.r32(sym::gShop)).wrapping_add(564)).wrapping_add((((4i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(1i32); F::sprintf(c, fp.wrapping_add(8), sym::menu_v_d__str67, &[((a2_) as i64 as u64)]) }; };
                    break 's1;
                }
                _ => {
                }
            }
        }
        F::Flash_FindLabel(c, l_fl_5, ((0i32) as u32), fp.wrapping_add(8), fp);
        F::Flash_ClipGotoLabel(c, l_fl_5, fp, p_label_4);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Shop_SetPlate", &PROBE_N, &[], true); }
    r_
}

pub fn Shop_Update(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Shop_Update", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let r_ = 'f: {
    if ((((((((({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(596); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) && (((({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(560); (c.m.r32(a_) as i32) }) & (8i32))) != 0)) as i32)) != 0) {
        if ((((((((({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(600); (c.m.r32(a_) as i32) }) == ((({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(604); (c.m.r32(a_) as i32) }).wrapping_mul(15i32)).wrapping_add(0i32))) as i32)) != 0) || ((((({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(600); (c.m.r32(a_) as i32) }) == ((({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(604); (c.m.r32(a_) as i32) }).wrapping_mul(15i32)).wrapping_add(2i32))) as i32)) != 0)) as i32)) != 0) {
            if (((((F::Voice_GetStat(c, 0i32)) == (5i32)) as i32)) != 0) {
                F::Shop_PlayVoice(c);
            }
        }
    }
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (6i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let a0_ = ((c.m.r32(sym::gShop)).wrapping_add(20)).wrapping_add((((l_i_1) as u32)).wrapping_mul(44)); F::Flash_Advance(c, a0_) };
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Shop_Update", &PROBE_N, &[], true); }
    r_
}

pub fn Shop_Input(c: &mut Ctx, mut p_result_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Shop_Input", &PROBE_N, &[(p_result_1) as u32], false); }
    let r_ = 'f: {
    if ((((!(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(20)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add(8); c.m.r32(a_) }) & (((2i32) as u32)))) != 0)) as i32)) != 0) {
        break 'f;
    }
    if ((((!(((({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(560); (c.m.r32(a_) as i32) }) & (4i32))) != 0)) as i32)) != 0) {
        F::Shop_SetPlate(c, 0i32, 0i32, 0i32, sym::menu_v_d__str68);
        { let r_ = 4i32; let a_ = (c.m.r32(sym::gShop)).wrapping_add(560); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
    's1: {
        let sv2: i32 = { let a_ = (c.m.r32(sym::gShop)).wrapping_add(592); (c.m.r32(a_) as i32) };
        match sv2 {
            0i32 => {
                if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((8i32) as u32)))) != 0) {
                    { let v_ = 0i32; let a_ = (c.m.r32(sym::gShop)).wrapping_add(616); c.m.w32(a_, (v_) as u32); };
                    F::Shop_SetPlate(c, 0i32, 0i32, 0i32, sym::menu_v_d__str69);
                    { let a_ = ((c.m.r32(sym::gShop)).wrapping_add(564)).wrapping_add(((({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(592); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                    if ((((({ let a_ = ((c.m.r32(sym::gShop)).wrapping_add(564)).wrapping_add(((({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(592); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) < (0i32)) as i32)) != 0) {
                        { let v_ = 1i32; let a_ = ((c.m.r32(sym::gShop)).wrapping_add(564)).wrapping_add(((({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(592); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    }
                    F::Shop_SetPlate(c, 0i32, 0i32, 0i32, sym::menu_v_d__str70);
                    F::Shop_PlayVoice(c);
                    { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                } else {
                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((4i32) as u32)))) != 0) {
                        { let v_ = 0i32; let a_ = (c.m.r32(sym::gShop)).wrapping_add(616); c.m.w32(a_, (v_) as u32); };
                        F::Shop_SetPlate(c, 0i32, 0i32, 0i32, sym::menu_v_d__str71);
                        { let a_ = ((c.m.r32(sym::gShop)).wrapping_add(564)).wrapping_add(((({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(592); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        if ((((({ let a_ = ((c.m.r32(sym::gShop)).wrapping_add(564)).wrapping_add(((({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(592); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) >= (2i32)) as i32)) != 0) {
                            { let v_ = 0i32; let a_ = ((c.m.r32(sym::gShop)).wrapping_add(564)).wrapping_add(((({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(592); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                        }
                        F::Shop_SetPlate(c, 0i32, 0i32, 0i32, sym::menu_v_d__str72);
                        F::Shop_PlayVoice(c);
                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                    } else {
                        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                            { let v_ = 0i32; let a_ = (c.m.r32(sym::gShop)).wrapping_add(616); c.m.w32(a_, (v_) as u32); };
                            F::Shop_SetPlate(c, 0i32, 0i32, 0i32, sym::menu_v_d__str73);
                            's2: {
                                let sv3: i32 = { let a_ = ((c.m.r32(sym::gShop)).wrapping_add(564)).wrapping_add(((({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(592); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
                                match sv3 {
                                    0i32 => {
                                        if ((((({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(636); (c.m.r32(a_) as i32) }) == (100i32)) as i32)) != 0) {
                                            { let v_ = (({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(604); (c.m.r32(a_) as i32) }).wrapping_mul(15i32)).wrapping_add(12i32); let a_ = (c.m.r32(sym::gShop)).wrapping_add(600); c.m.w32(a_, (v_) as u32); };
                                            { let a0_ = { let a_ = (c.m.r32(sym::gShop)).wrapping_add(12); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gShop)).wrapping_add(600); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34523i32, a2_) };
                                            { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 7i32); };
                                        } else {
                                            if ((F::Shop_IsSoldOut(c)) != 0) {
                                                { let v_ = (({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(604); (c.m.r32(a_) as i32) }).wrapping_mul(15i32)).wrapping_add(10i32); let a_ = (c.m.r32(sym::gShop)).wrapping_add(600); c.m.w32(a_, (v_) as u32); };
                                                { let a0_ = { let a_ = (c.m.r32(sym::gShop)).wrapping_add(12); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gShop)).wrapping_add(600); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34523i32, a2_) };
                                                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 7i32); };
                                            } else {
                                                { let a0_ = ((c.m.r32(sym::gShop)).wrapping_add(20)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_v_d__str74, ((((1i32) as i64 as u64)) as u8)) };
                                                { let a0_ = ((c.m.r32(sym::gShop)).wrapping_add(20)).wrapping_add((((1i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_v_d__str75, ((((1i32) as i64 as u64)) as u8)) };
                                                { let a0_ = ((c.m.r32(sym::gShop)).wrapping_add(20)).wrapping_add((((3i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_v_d__str76, ((((1i32) as i64 as u64)) as u8)) };
                                                { let v_ = 151i32; let a_ = (c.m.r32(sym::gShop)).wrapping_add(596); c.m.w32(a_, (v_) as u32); };
                                                F::Shop_SetPlate(c, 1i32, 1i32, 0i32, sym::menu_v_d__str77);
                                                F::Shop_SetPlate(c, 1i32, 1i32, 1i32, sym::menu_v_d__str78);
                                                { let r_ = 16i32; let a_ = (c.m.r32(sym::gShop)).wrapping_add(560); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                                                { let r_ = (!(32i32)); let a_ = (c.m.r32(sym::gShop)).wrapping_add(560); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                                                { let v_ = (({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(604); (c.m.r32(a_) as i32) }).wrapping_mul(15i32)).wrapping_add(5i32); let a_ = (c.m.r32(sym::gShop)).wrapping_add(600); c.m.w32(a_, (v_) as u32); };
                                                { let a0_ = { let a_ = (c.m.r32(sym::gShop)).wrapping_add(12); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gShop)).wrapping_add(600); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34523i32, a2_) };
                                                { let v_ = 1i32; let a_ = (c.m.r32(sym::gShop)).wrapping_add(592); c.m.w32(a_, (v_) as u32); };
                                                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                                            }
                                        }
                                        break 's2;
                                    }
                                    1i32 => {
                                        { let a0_ = ((c.m.r32(sym::gShop)).wrapping_add(20)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_v_d__str79, ((((1i32) as i64 as u64)) as u8)) };
                                        { let a0_ = ((c.m.r32(sym::gShop)).wrapping_add(20)).wrapping_add((((2i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_v_d__str80, ((((1i32) as i64 as u64)) as u8)) };
                                        F::MsgWin_Close(c);
                                        { let a0_ = ((c.m.r32(sym::gShop)).wrapping_add(20)).wrapping_add((((3i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_v_d__str81, ((((1i32) as i64 as u64)) as u8)) };
                                        { let r_ = 32i32; let a_ = (c.m.r32(sym::gShop)).wrapping_add(560); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                                        { let r_ = (!(16i32)); let a_ = (c.m.r32(sym::gShop)).wrapping_add(560); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                                        F::Shop_SetPlate(c, 2i32, 2i32, 0i32, sym::menu_v_d__str82);
                                        F::Shop_SetPlate(c, 2i32, 2i32, 1i32, sym::menu_v_d__str83);
                                        { let v_ = 2i32; let a_ = (c.m.r32(sym::gShop)).wrapping_add(592); c.m.w32(a_, (v_) as u32); };
                                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                                        break 's2;
                                    }
                                    _ => {
                                    }
                                }
                            }
                        } else {
                            if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1024i32) as u32)))) != 0) {
                                { let v_ = 0i32; let a_ = (c.m.r32(sym::gShop)).wrapping_add(616); c.m.w32(a_, (v_) as u32); };
                                { let v_ = 51i32; let a_ = (c.m.r32(sym::gShop)).wrapping_add(596); c.m.w32(a_, (v_) as u32); };
                                { let v_ = 0i32; let a_ = p_result_1; c.m.w32(a_, (v_) as u32); };
                                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 2i32); };
                            } else {
                                { let a_ = (c.m.r32(sym::gShop)).wrapping_add(616); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                                if ((((({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(616); (c.m.r32(a_) as i32) }) == (3600i32)) as i32)) != 0) {
                                    { let v_ = 1i32; let a_ = (c.m.r32(sym::gShop)).wrapping_add(596); c.m.w32(a_, (v_) as u32); };
                                    { let v_ = 0i32; let a_ = (c.m.r32(sym::gShop)).wrapping_add(616); c.m.w32(a_, (v_) as u32); };
                                }
                            }
                        }
                    }
                }
                break 's1;
            }
            1i32 => {
                if (((if ((((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(384)) as i32)) != (1i32)) as i32)) != 0) { ((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((16384i32) as u32))) } else { ((((((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((8i32) as u32)))) != 0) && ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(392))) & (((2048i32) as u32)))) != 0)) as i32)) as u32) })) != 0) {
                    if ((((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) > (0i32)) as i32)) != 0) {
                        F::Shop_SetPlate(c, 1i32, 1i32, 0i32, sym::menu_v_d__str84);
                        { let r_ = 4i32; let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); let n_ = ((c.m.r32(a_) as i32)).wrapping_sub(r_); c.m.w32(a_, (n_) as u32); };
                        if ((((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) < ({ let a_ = ((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(5648)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                            if ((((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) < (4i32)) as i32)) != 0) {
                                { let v_ = { let a_ = ((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(5648)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                                { let v_ = 0i32; let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                { let v_ = 0i32; let a_ = ((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(5648)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                { let v_ = 3i32; let a_ = ((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(5664)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                            } else {
                                { let v_ = { let a_ = ((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(5648)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                                { let r_ = 4i32; let a_ = ((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(5648)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); let n_ = ((c.m.r32(a_) as i32)).wrapping_sub(r_); c.m.w32(a_, (n_) as u32); };
                                { let r_ = 4i32; let a_ = ((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(5664)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); let n_ = ((c.m.r32(a_) as i32)).wrapping_sub(r_); c.m.w32(a_, (n_) as u32); };
                            }
                        }
                        F::Shop_SetPlate(c, 1i32, 1i32, 0i32, sym::menu_v_d__str85);
                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                    }
                } else {
                    if (((if ((((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(384)) as i32)) != (1i32)) as i32)) != 0) { ((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((65536i32) as u32))) } else { ((((((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((4i32) as u32)))) != 0) && ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(392))) & (((2048i32) as u32)))) != 0)) as i32)) as u32) })) != 0) {
                        if ((((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) < (({ let a_ = ((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(32)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_sub(1i32))) as i32)) != 0) {
                            F::Shop_SetPlate(c, 1i32, 1i32, 0i32, sym::menu_v_d__str86);
                            { let r_ = 4i32; let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                            if ((((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) > (({ let a_ = ((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(32)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_sub(1i32))) as i32)) != 0) {
                                { let v_ = ({ let a_ = ((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(32)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_sub(1i32); let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                            }
                            if ((((({ let a_ = ((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(5664)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) < ({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                                if ((((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) > (({ let a_ = ((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(32)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_sub(5i32))) as i32)) != 0) {
                                    { let v_ = ({ let a_ = ((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(32)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_sub(1i32); let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                    { let v_ = { let a_ = ((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(5664)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                                    { let v_ = ({ let a_ = ((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(32)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_sub(4i32); let a_ = ((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(5648)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                    { let v_ = ({ let a_ = ((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(32)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_sub(1i32); let a_ = ((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(5664)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                } else {
                                    { let v_ = { let a_ = ((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(5664)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                                    { let r_ = 4i32; let a_ = ((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(5648)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                                    { let r_ = 4i32; let a_ = ((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(5664)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                                }
                            }
                            F::Shop_SetPlate(c, 1i32, 1i32, 0i32, sym::menu_v_d__str87);
                            { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                        }
                    } else {
                        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((8i32) as u32)))) != 0) {
                            if ((((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) > (0i32)) as i32)) != 0) {
                                F::Shop_SetPlate(c, 1i32, 1i32, 0i32, sym::menu_v_d__str88);
                                { let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                                if ((((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) < ({ let a_ = ((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(5648)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                                    { let a0_ = ((c.m.r32(sym::gShop)).wrapping_add(20)).wrapping_add((((1i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_v_d__str89, ((((1i32) as i64 as u64)) as u8)) };
                                    { let v_ = { let a_ = ((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(5664)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                                    { let a_ = ((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(5648)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                                    { let a_ = ((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(5664)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                                }
                                F::Shop_SetPlate(c, 1i32, 1i32, 0i32, sym::menu_v_d__str90);
                                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                            }
                        } else {
                            if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((4i32) as u32)))) != 0) {
                                if ((((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) < (({ let a_ = ((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(32)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_sub(1i32))) as i32)) != 0) {
                                    F::Shop_SetPlate(c, 1i32, 1i32, 0i32, sym::menu_v_d__str91);
                                    { let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                                    if ((((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) > ({ let a_ = ((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(5664)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                                        { let a0_ = ((c.m.r32(sym::gShop)).wrapping_add(20)).wrapping_add((((1i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_v_d__str92, ((((1i32) as i64 as u64)) as u8)) };
                                        { let v_ = { let a_ = ((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(5648)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                                        { let a_ = ((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(5648)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                                        { let a_ = ((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(5664)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                                    }
                                    F::Shop_SetPlate(c, 1i32, 1i32, 0i32, sym::menu_v_d__str93);
                                    { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                                }
                            } else {
                                if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((1i32) as u32)))) != 0) {
                                    F::Shop_SetPlate(c, 1i32, 1i32, 0i32, sym::menu_v_d__str94);
                                    F::Shop_SetPlate(c, 1i32, 1i32, 1i32, sym::menu_v_d__str95);
                                    { let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(16); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                                    if ((((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) < (0i32)) as i32)) != 0) {
                                        { let v_ = (4i32).wrapping_sub(1i32); let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
                                    }
                                    F::Shop_SetPlate(c, 1i32, 1i32, 0i32, sym::menu_v_d__str96);
                                    F::Shop_SetPlate(c, 1i32, 1i32, 1i32, sym::menu_v_d__str97);
                                    { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                                } else {
                                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((2i32) as u32)))) != 0) {
                                        F::Shop_SetPlate(c, 1i32, 1i32, 0i32, sym::menu_v_d__str98);
                                        F::Shop_SetPlate(c, 1i32, 1i32, 1i32, sym::menu_v_d__str99);
                                        { let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(16); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                                        if ((((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) >= (4i32)) as i32)) != 0) {
                                            { let v_ = 0i32; let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
                                        }
                                        F::Shop_SetPlate(c, 1i32, 1i32, 0i32, sym::menu_v_d__str100);
                                        F::Shop_SetPlate(c, 1i32, 1i32, 1i32, sym::menu_v_d__str101);
                                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                                    } else {
                                        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                                            F::Shop_SetPlate(c, 1i32, 1i32, 0i32, sym::menu_v_d__str102);
                                            if (({ let a0_ = { let a_ = (((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(48)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(1400))).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a1_ = { let a_ = (c.m.r32(sym::gShop)).wrapping_add(12004); c.m.r32(a_) }; F::Shop_CanBuy(c, a0_, a1_) }) != 0) {
                                                { let v_ = 1i32; let a_ = ((c.m.r32(sym::gShop)).wrapping_add(564)).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                                F::Shop_SetPlate(c, 4i32, 4i32, 0i32, sym::menu_v_d__str103);
                                                { let a0_ = ((c.m.r32(sym::gShop)).wrapping_add(20)).wrapping_add((((4i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_v_d__str104, ((((1i32) as i64 as u64)) as u8)) };
                                                { let v_ = (({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(604); (c.m.r32(a_) as i32) }).wrapping_mul(15i32)).wrapping_add(6i32); let a_ = (c.m.r32(sym::gShop)).wrapping_add(600); c.m.w32(a_, (v_) as u32); };
                                                { let a0_ = { let a_ = (c.m.r32(sym::gShop)).wrapping_add(12); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gShop)).wrapping_add(600); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34523i32, a2_) };
                                                { let v_ = 4i32; let a_ = (c.m.r32(sym::gShop)).wrapping_add(592); c.m.w32(a_, (v_) as u32); };
                                                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                                            } else {
                                                { let v_ = (({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(604); (c.m.r32(a_) as i32) }).wrapping_mul(15i32)).wrapping_add(11i32); let a_ = (c.m.r32(sym::gShop)).wrapping_add(600); c.m.w32(a_, (v_) as u32); };
                                                { let a0_ = { let a_ = (c.m.r32(sym::gShop)).wrapping_add(12); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gShop)).wrapping_add(600); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34523i32, a2_) };
                                                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 7i32); };
                                            }
                                        } else {
                                            if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1024i32) as u32)))) != 0) {
                                                { let a0_ = ((c.m.r32(sym::gShop)).wrapping_add(20)).wrapping_add((((1i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_v_d__str105, ((((1i32) as i64 as u64)) as u8)) };
                                                { let a0_ = ((c.m.r32(sym::gShop)).wrapping_add(20)).wrapping_add((((3i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_v_d__str106, ((((1i32) as i64 as u64)) as u8)) };
                                                { let a0_ = ((c.m.r32(sym::gShop)).wrapping_add(20)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_v_d__str107, ((((1i32) as i64 as u64)) as u8)) };
                                                { let v_ = 151i32; let a_ = (c.m.r32(sym::gShop)).wrapping_add(596); c.m.w32(a_, (v_) as u32); };
                                                F::Shop_SetPlate(c, 1i32, 1i32, 0i32, sym::menu_v_d__str108);
                                                F::Shop_SetPlate(c, 1i32, 1i32, 1i32, sym::menu_v_d__str109);
                                                { let v_ = (({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(604); (c.m.r32(a_) as i32) }).wrapping_mul(15i32)).wrapping_add(4i32); let a_ = (c.m.r32(sym::gShop)).wrapping_add(600); c.m.w32(a_, (v_) as u32); };
                                                { let a0_ = { let a_ = (c.m.r32(sym::gShop)).wrapping_add(12); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gShop)).wrapping_add(600); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34523i32, a2_) };
                                                { let v_ = 0i32; let a_ = (c.m.r32(sym::gShop)).wrapping_add(592); c.m.w32(a_, (v_) as u32); };
                                                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 2i32); };
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                break 's1;
            }
            2i32 => {
                if (((if ((((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(384)) as i32)) != (1i32)) as i32)) != 0) { ((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((16384i32) as u32))) } else { ((((((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((8i32) as u32)))) != 0) && ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(392))) & (((2048i32) as u32)))) != 0)) as i32)) as u32) })) != 0) {
                    if ((((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) > (0i32)) as i32)) != 0) {
                        F::Shop_SetPlate(c, 2i32, 2i32, 0i32, sym::menu_v_d__str110);
                        { let r_ = 7i32; let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); let n_ = ((c.m.r32(a_) as i32)).wrapping_sub(r_); c.m.w32(a_, (n_) as u32); };
                        if ((((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) < ({ let a_ = ((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(5648)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                            if ((((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) < (7i32)) as i32)) != 0) {
                                { let v_ = { let a_ = ((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(5648)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                                { let v_ = 0i32; let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                { let v_ = 0i32; let a_ = ((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(5648)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                { let v_ = 6i32; let a_ = ((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(5664)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                            } else {
                                { let v_ = { let a_ = ((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(5648)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                                { let r_ = 7i32; let a_ = ((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(5648)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); let n_ = ((c.m.r32(a_) as i32)).wrapping_sub(r_); c.m.w32(a_, (n_) as u32); };
                                { let r_ = 7i32; let a_ = ((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(5664)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); let n_ = ((c.m.r32(a_) as i32)).wrapping_sub(r_); c.m.w32(a_, (n_) as u32); };
                            }
                        }
                        F::Shop_SetPlate(c, 2i32, 2i32, 0i32, sym::menu_v_d__str111);
                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                    }
                } else {
                    if (((if ((((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(384)) as i32)) != (1i32)) as i32)) != 0) { ((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((65536i32) as u32))) } else { ((((((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((4i32) as u32)))) != 0) && ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(392))) & (((2048i32) as u32)))) != 0)) as i32)) as u32) })) != 0) {
                        if ((((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) < (({ let a_ = ((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(32)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_sub(1i32))) as i32)) != 0) {
                            F::Shop_SetPlate(c, 2i32, 2i32, 0i32, sym::menu_v_d__str112);
                            { let r_ = 7i32; let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                            if ((((({ let a_ = ((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(5664)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) < ({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                                if ((((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) > (({ let a_ = ((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(32)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_sub(8i32))) as i32)) != 0) {
                                    { let v_ = ({ let a_ = ((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(32)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_sub(1i32); let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                    { let v_ = { let a_ = ((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(5664)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                                    { let v_ = ({ let a_ = ((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(32)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_sub(7i32); let a_ = ((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(5648)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                    { let v_ = ({ let a_ = ((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(32)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_sub(1i32); let a_ = ((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(5664)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                } else {
                                    { let v_ = { let a_ = ((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(5664)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                                    { let r_ = 7i32; let a_ = ((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(5648)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                                    { let r_ = 7i32; let a_ = ((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(5664)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                                }
                            }
                            F::Shop_SetPlate(c, 2i32, 2i32, 0i32, sym::menu_v_d__str113);
                            { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                        }
                    } else {
                        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((8i32) as u32)))) != 0) {
                            if ((((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) > (0i32)) as i32)) != 0) {
                                F::Shop_SetPlate(c, 2i32, 2i32, 0i32, sym::menu_v_d__str114);
                                { let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                                if ((((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) < ({ let a_ = ((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(5648)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                                    { let a0_ = ((c.m.r32(sym::gShop)).wrapping_add(20)).wrapping_add((((2i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_v_d__str115, ((((1i32) as i64 as u64)) as u8)) };
                                    { let v_ = { let a_ = ((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(5664)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                                    { let a_ = ((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(5648)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                                    { let a_ = ((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(5664)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                                }
                                F::Shop_SetPlate(c, 2i32, 2i32, 0i32, sym::menu_v_d__str116);
                                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                            }
                        } else {
                            if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((4i32) as u32)))) != 0) {
                                if ((((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) < (({ let a_ = ((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(32)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_sub(1i32))) as i32)) != 0) {
                                    F::Shop_SetPlate(c, 2i32, 2i32, 0i32, sym::menu_v_d__str117);
                                    { let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                                    if ((((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) > ({ let a_ = ((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(5664)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                                        { let a0_ = ((c.m.r32(sym::gShop)).wrapping_add(20)).wrapping_add((((2i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_v_d__str118, ((((1i32) as i64 as u64)) as u8)) };
                                        { let v_ = { let a_ = ((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(5648)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                                        { let a_ = ((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(5648)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                                        { let a_ = ((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(5664)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                                    }
                                    F::Shop_SetPlate(c, 2i32, 2i32, 0i32, sym::menu_v_d__str119);
                                    { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                                }
                            } else {
                                if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((1i32) as u32)))) != 0) {
                                    F::Shop_SetPlate(c, 2i32, 2i32, 0i32, sym::menu_v_d__str120);
                                    F::Shop_SetPlate(c, 2i32, 2i32, 1i32, sym::menu_v_d__str121);
                                    { let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(16); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                                    if ((((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) < (0i32)) as i32)) != 0) {
                                        { let v_ = (4i32).wrapping_sub(1i32); let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
                                    }
                                    F::Shop_SetPlate(c, 2i32, 2i32, 0i32, sym::menu_v_d__str122);
                                    F::Shop_SetPlate(c, 2i32, 2i32, 1i32, sym::menu_v_d__str123);
                                    { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                                } else {
                                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((2i32) as u32)))) != 0) {
                                        F::Shop_SetPlate(c, 2i32, 2i32, 0i32, sym::menu_v_d__str124);
                                        F::Shop_SetPlate(c, 2i32, 2i32, 1i32, sym::menu_v_d__str125);
                                        { let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(16); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                                        if ((((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) >= (4i32)) as i32)) != 0) {
                                            { let v_ = 0i32; let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
                                        }
                                        F::Shop_SetPlate(c, 2i32, 2i32, 0i32, sym::menu_v_d__str126);
                                        F::Shop_SetPlate(c, 2i32, 2i32, 1i32, sym::menu_v_d__str127);
                                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                                    } else {
                                        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                                            F::Shop_SetPlate(c, 2i32, 2i32, 0i32, sym::menu_v_d__str128);
                                            if (((((((({ let a_ = ((c.m.r32(sym::gSaveData)).wrapping_add(11976)).wrapping_add((({ let a_ = (((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(48)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(1400))).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)); c.m.r8(a_) }) as i32)) & (1i32))) as u8)) != 0) {
                                                F::ItemHelp_Open(c);
                                                { let v_ = { let a_ = (c.m.r32(sym::gShop)).wrapping_add(592); (c.m.r32(a_) as i32) }; let a_ = ((c.m.r32(sym::gShop)).wrapping_add(564)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                                { let v_ = { let a_ = (((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(48)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(1400))).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((1i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gShop)).wrapping_add(620); c.m.w32(a_, (v_) as u32); };
                                                { let r_ = (!(2i32)); let a_ = ((c.m.r32(sym::gSaveData)).wrapping_add(11976)).wrapping_add((({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(620); (c.m.r32(a_) as i32) }) as u32)); let n_ = ((((((c.m.r8(a_)) as i32)) & (r_))) as u8); c.m.w8(a_, n_); };
                                                { let v_ = 3i32; let a_ = (c.m.r32(sym::gShop)).wrapping_add(592); c.m.w32(a_, (v_) as u32); };
                                                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                                            } else {
                                                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 7i32); };
                                            }
                                        } else {
                                            if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1024i32) as u32)))) != 0) {
                                                { let a0_ = ((c.m.r32(sym::gShop)).wrapping_add(20)).wrapping_add((((2i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_v_d__str129, ((((1i32) as i64 as u64)) as u8)) };
                                                F::MsgWin_Open(c);
                                                { let a0_ = ((c.m.r32(sym::gShop)).wrapping_add(20)).wrapping_add((((3i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_v_d__str130, ((((1i32) as i64 as u64)) as u8)) };
                                                { let a0_ = ((c.m.r32(sym::gShop)).wrapping_add(20)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_v_d__str131, ((((1i32) as i64 as u64)) as u8)) };
                                                F::Shop_SetPlate(c, 2i32, 2i32, 0i32, sym::menu_v_d__str132);
                                                F::Shop_SetPlate(c, 2i32, 2i32, 1i32, sym::menu_v_d__str133);
                                                { let v_ = (({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(604); (c.m.r32(a_) as i32) }).wrapping_mul(15i32)).wrapping_add(4i32); let a_ = (c.m.r32(sym::gShop)).wrapping_add(600); c.m.w32(a_, (v_) as u32); };
                                                { let a0_ = { let a_ = (c.m.r32(sym::gShop)).wrapping_add(12); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gShop)).wrapping_add(600); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34523i32, a2_) };
                                                { let v_ = 0i32; let a_ = (c.m.r32(sym::gShop)).wrapping_add(592); c.m.w32(a_, (v_) as u32); };
                                                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 2i32); };
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                break 's1;
            }
            3i32 => {
                if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((((512i32) | (1024i32))) as u32)))) != 0) {
                    F::ItemHelp_Close(c);
                    's3: {
                        let sv4: i32 = { let a_ = ((c.m.r32(sym::gShop)).wrapping_add(564)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
                        match sv4 {
                            2i32 => {
                                F::Shop_SetPlate(c, 2i32, 2i32, 0i32, sym::menu_v_d__str134);
                                F::Shop_SetPlate(c, 2i32, 2i32, 1i32, sym::menu_v_d__str135);
                                break 's3;
                            }
                            4i32 => {
                                F::Shop_SetPlate(c, 4i32, 4i32, 0i32, sym::menu_v_d__str136);
                                break 's3;
                            }
                            _ => {
                            }
                        }
                    }
                    { let v_ = { let a_ = ((c.m.r32(sym::gShop)).wrapping_add(564)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gShop)).wrapping_add(592); c.m.w32(a_, (v_) as u32); };
                    { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 2i32); };
                }
                break 's1;
            }
            4i32 => {
                if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((8i32) as u32)))) != 0) {
                    F::Shop_SetPlate(c, 4i32, 4i32, 0i32, sym::menu_v_d__str137);
                    { let a_ = ((c.m.r32(sym::gShop)).wrapping_add(564)).wrapping_add(((({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(592); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                    if ((((({ let a_ = ((c.m.r32(sym::gShop)).wrapping_add(564)).wrapping_add(((({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(592); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) < (0i32)) as i32)) != 0) {
                        { let v_ = 2i32; let a_ = ((c.m.r32(sym::gShop)).wrapping_add(564)).wrapping_add(((({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(592); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    }
                    F::Shop_SetPlate(c, 4i32, 4i32, 0i32, sym::menu_v_d__str138);
                    { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                } else {
                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((4i32) as u32)))) != 0) {
                        F::Shop_SetPlate(c, 4i32, 4i32, 0i32, sym::menu_v_d__str139);
                        { let a_ = ((c.m.r32(sym::gShop)).wrapping_add(564)).wrapping_add(((({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(592); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        if ((((({ let a_ = ((c.m.r32(sym::gShop)).wrapping_add(564)).wrapping_add(((({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(592); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) >= (3i32)) as i32)) != 0) {
                            { let v_ = 0i32; let a_ = ((c.m.r32(sym::gShop)).wrapping_add(564)).wrapping_add(((({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(592); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                        }
                        F::Shop_SetPlate(c, 4i32, 4i32, 0i32, sym::menu_v_d__str140);
                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                    } else {
                        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                            's4: {
                                let sv5: i32 = { let a_ = ((c.m.r32(sym::gShop)).wrapping_add(564)).wrapping_add(((({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(592); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
                                match sv5 {
                                    0i32 => {
                                        if ((((((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(12328); (c.m.r32(a_) as i32) }) as u32)) < ((({ let a_ = (({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(12004); c.m.r32(a_) }).wrapping_add(((({ let a_ = (((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(48)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(1400))).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(40))).wrapping_add(4); (c.m.r32(a_) as i32) }) as u32))) as i32)) != 0) {
                                            { let v_ = (({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(604); (c.m.r32(a_) as i32) }).wrapping_mul(15i32)).wrapping_add(8i32); let a_ = (c.m.r32(sym::gShop)).wrapping_add(600); c.m.w32(a_, (v_) as u32); };
                                            { let a0_ = { let a_ = (c.m.r32(sym::gShop)).wrapping_add(12); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gShop)).wrapping_add(600); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34523i32, a2_) };
                                            { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 7i32); };
                                        } else {
                                            F::Shop_SetPlate(c, 4i32, 4i32, 0i32, sym::menu_v_d__str141);
                                            { let a0_ = ((c.m.r32(sym::gShop)).wrapping_add(20)).wrapping_add((((1i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_v_d__str142, ((((1i32) as i64 as u64)) as u8)) };
                                            { let a0_ = ((c.m.r32(sym::gShop)).wrapping_add(20)).wrapping_add((((4i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_v_d__str143, ((((1i32) as i64 as u64)) as u8)) };
                                            { let v_ = { let a_ = (((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(48)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(1400))).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gShop)).wrapping_add(624); c.m.w32(a_, (v_) as u32); };
                                            { let a0_ = { let a_ = (c.m.r32(sym::gShop)).wrapping_add(624); (c.m.r32(a_) as i32) }; F::Save_AddItem(c, a0_) };
                                            { let r_ = { let a_ = (({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(12004); c.m.r32(a_) }).wrapping_add(((({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(624); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(40))).wrapping_add(4); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(12328); let n_ = ((c.m.r32(a_) as i32)).wrapping_sub(r_); c.m.w32(a_, (n_) as u32); };
                                            { let a_ = (c.m.r32(sym::gShop)).wrapping_add(632); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                                            { let v_ = rt::f2i(rt::fmul(rt::fdiv(rt::i2f({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(632); (c.m.r32(a_) as i32) }), rt::i2f({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(628); (c.m.r32(a_) as i32) })), F32(0x42c80000))); let a_ = (c.m.r32(sym::gShop)).wrapping_add(636); c.m.w32(a_, (v_) as u32); };
                                            if ((((((((({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(632); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) && ((((({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(636); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0)) as i32)) != 0) {
                                                { let v_ = 1i32; let a_ = (c.m.r32(sym::gShop)).wrapping_add(636); c.m.w32(a_, (v_) as u32); };
                                            }
                                            { let r_ = 1i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(20); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                                            { let v_ = 5i32; let a_ = (c.m.r32(sym::gShop)).wrapping_add(592); c.m.w32(a_, (v_) as u32); };
                                            { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                                        }
                                        break 's4;
                                    }
                                    1i32 => {
                                        { let a0_ = ((c.m.r32(sym::gShop)).wrapping_add(20)).wrapping_add((((4i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_v_d__str144, ((((1i32) as i64 as u64)) as u8)) };
                                        F::Shop_SetPlate(c, 4i32, 4i32, 0i32, sym::menu_v_d__str145);
                                        { let v_ = 1i32; let a_ = (c.m.r32(sym::gShop)).wrapping_add(592); c.m.w32(a_, (v_) as u32); };
                                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 2i32); };
                                        break 's4;
                                    }
                                    2i32 => {
                                        F::Shop_SetPlate(c, 4i32, 4i32, 0i32, sym::menu_v_d__str146);
                                        F::ItemHelp_Open(c);
                                        { let v_ = { let a_ = (c.m.r32(sym::gShop)).wrapping_add(592); (c.m.r32(a_) as i32) }; let a_ = ((c.m.r32(sym::gShop)).wrapping_add(564)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                        { let v_ = { let a_ = (((((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(48)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(1400))).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(644)).wrapping_add((((0i32) as u32)).wrapping_mul(5680))).wrapping_add(16); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gShop)).wrapping_add(620); c.m.w32(a_, (v_) as u32); };
                                        { let v_ = 3i32; let a_ = (c.m.r32(sym::gShop)).wrapping_add(592); c.m.w32(a_, (v_) as u32); };
                                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                                        break 's4;
                                    }
                                    _ => {
                                    }
                                }
                            }
                        } else {
                            if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1024i32) as u32)))) != 0) {
                                { let a0_ = ((c.m.r32(sym::gShop)).wrapping_add(20)).wrapping_add((((4i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_v_d__str147, ((((1i32) as i64 as u64)) as u8)) };
                                F::Shop_SetPlate(c, 4i32, 4i32, 0i32, sym::menu_v_d__str148);
                                { let v_ = 1i32; let a_ = (c.m.r32(sym::gShop)).wrapping_add(592); c.m.w32(a_, (v_) as u32); };
                                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 2i32); };
                            }
                        }
                    }
                }
                break 's1;
            }
            5i32 => {
                if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                    if ((((({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(636); (c.m.r32(a_) as i32) }) == (100i32)) as i32)) != 0) {
                        { let v_ = 101i32; let a_ = (c.m.r32(sym::gShop)).wrapping_add(596); c.m.w32(a_, (v_) as u32); };
                    } else {
                        { let v_ = (({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(604); (c.m.r32(a_) as i32) }).wrapping_mul(15i32)).wrapping_add(7i32); let a_ = (c.m.r32(sym::gShop)).wrapping_add(600); c.m.w32(a_, (v_) as u32); };
                        { let a0_ = { let a_ = (c.m.r32(sym::gShop)).wrapping_add(12); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gShop)).wrapping_add(600); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34523i32, a2_) };
                    }
                    { let a0_ = ((c.m.r32(sym::gShop)).wrapping_add(20)).wrapping_add((((1i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_v_d__str149, ((((1i32) as i64 as u64)) as u8)) };
                    { let v_ = 1i32; let a_ = (c.m.r32(sym::gShop)).wrapping_add(592); c.m.w32(a_, (v_) as u32); };
                    { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                }
                break 's1;
            }
            _ => {
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Shop_Input", &PROBE_N, &[], true); }
    r_
}

pub fn Shop_UpdateSeq(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Shop_UpdateSeq", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if ((((({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(596); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
        break 'f;
    }
    's1: {
        let sv1: i32 = { let a_ = (c.m.r32(sym::gShop)).wrapping_add(596); (c.m.r32(a_) as i32) };
        'k20: {
        'k19: {
        'k18: {
        'k17: {
        'k16: {
        'k15: {
        'k14: {
        'k13: {
        'k12: {
        'k11: {
        'k10: {
        'k9: {
        'k8: {
        'k7: {
        'k6: {
        'k5: {
        'k4: {
        'k3: {
        'k2: {
            match sv1 { 1i32 => break 'k2, 2i32 => break 'k3, 3i32 => break 'k4, 4i32 => break 'k5, 5i32 => break 'k6, 6i32 => break 'k7, 7i32 => break 'k8, 51i32 => break 'k9, 52i32 => break 'k10, 53i32 => break 'k11, 101i32 => break 'k12, 102i32 => break 'k13, 103i32 => break 'k14, 104i32 => break 'k15, 105i32 => break 'k16, 106i32 => break 'k17, 151i32 => break 'k18, 152i32 => break 'k19, 153i32 => break 'k20, _ => break 's1, }
        }
        { let a0_ = ((c.m.r32(sym::gShop)).wrapping_add(20)).wrapping_add((((3i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_v_d__str150, ((((1i32) as i64 as u64)) as u8)) };
        { let a_ = (c.m.r32(sym::gShop)).wrapping_add(596); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        break 's1;
        }
        if (((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(20)).wrapping_add((((3i32) as u32)).wrapping_mul(44))).wrapping_add(12); c.m.r32(a_) }) & (((1i32) as u32)))) != 0) {
            { let a_ = (c.m.r32(sym::gShop)).wrapping_add(596); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        }
        break 's1;
        }
        { let v_ = (({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(604); (c.m.r32(a_) as i32) }).wrapping_mul(15i32)).wrapping_add(13i32); let a_ = (c.m.r32(sym::gShop)).wrapping_add(600); c.m.w32(a_, (v_) as u32); };
        { let a0_ = { let a_ = (c.m.r32(sym::gShop)).wrapping_add(12); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gShop)).wrapping_add(600); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34523i32, a2_) };
        { let a_ = (c.m.r32(sym::gShop)).wrapping_add(596); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        break 's1;
        }
        if (((((F::Voice_GetStat(c, 0i32)) == (5i32)) as i32)) != 0) {
            { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 35i32); };
            F::Shop_SwapGuide(c);
            { let a_ = (c.m.r32(sym::gShop)).wrapping_add(596); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        }
        break 's1;
        }
        { let a0_ = ((c.m.r32(sym::gShop)).wrapping_add(20)).wrapping_add((((3i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_v_d__str151, ((((1i32) as i64 as u64)) as u8)) };
        { let a_ = (c.m.r32(sym::gShop)).wrapping_add(596); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        break 's1;
        }
        if (((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(20)).wrapping_add((((3i32) as u32)).wrapping_mul(44))).wrapping_add(12); c.m.r32(a_) }) & (((1i32) as u32)))) != 0) {
            { let v_ = (({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(604); (c.m.r32(a_) as i32) }).wrapping_mul(15i32)).wrapping_add(0i32); let a_ = (c.m.r32(sym::gShop)).wrapping_add(600); c.m.w32(a_, (v_) as u32); };
            { let a0_ = { let a_ = (c.m.r32(sym::gShop)).wrapping_add(12); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gShop)).wrapping_add(600); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34523i32, a2_) };
            { let a_ = (c.m.r32(sym::gShop)).wrapping_add(596); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        }
        break 's1;
        }
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gShop)).wrapping_add(596); c.m.w32(a_, (v_) as u32); };
        break 's1;
        }
        { let v_ = (({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(604); (c.m.r32(a_) as i32) }).wrapping_mul(15i32)).wrapping_add(14i32); let a_ = (c.m.r32(sym::gShop)).wrapping_add(600); c.m.w32(a_, (v_) as u32); };
        { let a0_ = { let a_ = (c.m.r32(sym::gShop)).wrapping_add(12); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gShop)).wrapping_add(600); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34523i32, a2_) };
        { let a_ = (c.m.r32(sym::gShop)).wrapping_add(596); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        break 's1;
        }
        if (((((F::Voice_GetStat(c, 0i32)) == (5i32)) as i32)) != 0) {
            { let a_ = (c.m.r32(sym::gShop)).wrapping_add(596); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        } else {
            if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                { let a_ = (c.m.r32(sym::gShop)).wrapping_add(596); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
            } else {
                if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1024i32) as u32)))) != 0) {
                    { let a_ = (c.m.r32(sym::gShop)).wrapping_add(596); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 2i32); };
                }
            }
        }
        break 's1;
        }
        F::ColorFade_StartOut(c, 0i32, 0i32, 0i32, 20i32);
        { let a0_ = ((c.m.r32(sym::gShop)).wrapping_add(20)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_v_d__str152, ((((1i32) as i64 as u64)) as u8)) };
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gShop)).wrapping_add(596); c.m.w32(a_, (v_) as u32); };
        break 's1;
        }
        { let a_ = (c.m.r32(sym::gShop)).wrapping_add(596); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        }
        if (((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(20)).wrapping_add((((1i32) as u32)).wrapping_mul(44))).wrapping_add(12); c.m.r32(a_) }) & (((1i32) as u32)))) != 0) {
            { let a0_ = ((c.m.r32(sym::gShop)).wrapping_add(20)).wrapping_add((((5i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_v_d__str153, ((((1i32) as i64 as u64)) as u8)) };
            { let a_ = (c.m.r32(sym::gShop)).wrapping_add(596); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        }
        break 's1;
        }
        if (((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(20)).wrapping_add((((5i32) as u32)).wrapping_mul(44))).wrapping_add(12); c.m.r32(a_) }) & (((2i32) as u32)))) != 0) {
            { let v_ = (({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(604); (c.m.r32(a_) as i32) }).wrapping_mul(15i32)).wrapping_add(9i32); let a_ = (c.m.r32(sym::gShop)).wrapping_add(600); c.m.w32(a_, (v_) as u32); };
            { let a0_ = { let a_ = (c.m.r32(sym::gShop)).wrapping_add(12); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gShop)).wrapping_add(600); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34523i32, a2_) };
            { let a_ = (c.m.r32(sym::gShop)).wrapping_add(596); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        }
        break 's1;
        }
        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
            { let a0_ = ((c.m.r32(sym::gShop)).wrapping_add(20)).wrapping_add((((5i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_v_d__str154, ((((1i32) as i64 as u64)) as u8)) };
            { let a_ = (c.m.r32(sym::gShop)).wrapping_add(596); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
            { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
        }
        break 's1;
        }
        if (((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(20)).wrapping_add((((5i32) as u32)).wrapping_mul(44))).wrapping_add(12); c.m.r32(a_) }) & (((2i32) as u32)))) != 0) {
            { let a_ = (c.m.r32(sym::gShop)).wrapping_add(596); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        }
        break 's1;
        }
        { let a0_ = ((c.m.r32(sym::gShop)).wrapping_add(20)).wrapping_add((((1i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_v_d__str155, ((((1i32) as i64 as u64)) as u8)) };
        { let a0_ = ((c.m.r32(sym::gShop)).wrapping_add(20)).wrapping_add((((3i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_v_d__str156, ((((1i32) as i64 as u64)) as u8)) };
        { let a0_ = ((c.m.r32(sym::gShop)).wrapping_add(20)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_v_d__str157, ((((1i32) as i64 as u64)) as u8)) };
        F::Shop_SetPlate(c, 1i32, 1i32, 0i32, sym::menu_v_d__str158);
        F::Shop_SetPlate(c, 1i32, 1i32, 1i32, sym::menu_v_d__str159);
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gShop)).wrapping_add(592); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gShop)).wrapping_add(596); c.m.w32(a_, (v_) as u32); };
        break 's1;
        }
        { let a_ = (c.m.r32(sym::gShop)).wrapping_add(596); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        }
        if (((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(20)).wrapping_add((((3i32) as u32)).wrapping_mul(44))).wrapping_add(12); c.m.r32(a_) }) & (((2i32) as u32)))) != 0) {
            { let a_ = (c.m.r32(sym::gShop)).wrapping_add(596); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        }
        break 's1;
        }
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gShop)).wrapping_add(596); c.m.w32(a_, (v_) as u32); };
        if (((({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(560); (c.m.r32(a_) as i32) }) & (128i32))) != 0) {
            { let r_ = 128i32; let a_ = (c.m.r32(sym::gShop)).wrapping_add(560); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
        } else {
            { let r_ = 128i32; let a_ = (c.m.r32(sym::gShop)).wrapping_add(560); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        }
        break 's1;
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Shop_UpdateSeq", &PROBE_N, &[], true); }
    r_
}

pub fn Shop_Run(c: &mut Ctx, mut p_section_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Shop_Run", &PROBE_N, &[(p_section_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        { let v_ = 1i32; c.m.w32(fp, (v_) as u32); }
        F::Shop_Init(c, p_section_1);
        F::ColorFade_StartIn(c, 0i32, 0i32, 0i32, 20i32);
        'l1: while ((1i32) != 0) {
            F::Gfx_BeginFrame(c);
            F::Pad_Update(c);
            F::Snd_Update(c);
            F::ColorFade_Update(c);
            if ((((!(((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(20); (c.m.r32(a_) as i32) }) & (256i32))) != 0)) as i32)) != 0) {
                F::Shop_Update(c);
                F::Shop_UpdateSeq(c);
            }
            F::Shop_Draw(c);
            F::ColorFade_Draw(c);
            F::Gfx_EndFrame(c, 1i32);
            F::Dma_Flush(c);
            F::File_Stub264D90(c);
            if (((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(20); (c.m.r32(a_) as i32) }) & (256i32))) != 0) {
                continue 'l1;
            }
            if ((F::ColorFade_IsInDone(c)) != 0) {
                if ((((((((!(((({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(560); (c.m.r32(a_) as i32) }) & (8i32))) != 0)) as i32)) != 0) && (((({ let a_ = (((c.m.r32(sym::gShop)).wrapping_add(20)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add(8); c.m.r32(a_) }) & (((2i32) as u32)))) != 0)) as i32)) != 0) {
                    { let r_ = 8i32; let a_ = (c.m.r32(sym::gShop)).wrapping_add(560); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    if (((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(4104); (c.m.r32(a_) as i32) }) & (1i32))) != 0) {
                        { let v_ = (({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(604); (c.m.r32(a_) as i32) }).wrapping_mul(15i32)).wrapping_add(2i32); let a_ = (c.m.r32(sym::gShop)).wrapping_add(600); c.m.w32(a_, (v_) as u32); };
                        { let r_ = (!(1i32)); let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(4104); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                    } else {
                        { let v_ = (({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(604); (c.m.r32(a_) as i32) }).wrapping_mul(15i32)).wrapping_add(0i32); let a_ = (c.m.r32(sym::gShop)).wrapping_add(600); c.m.w32(a_, (v_) as u32); };
                    }
                    { let a0_ = { let a_ = (c.m.r32(sym::gShop)).wrapping_add(12); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gShop)).wrapping_add(600); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34523i32, a2_) };
                }
            }
            if ((F::ColorFade_IsFadingOut(c)) != 0) {
                F::Voice_FadeOutStep(c, 0i32);
                continue 'l1;
            }
            if ((F::ColorFade_IsOutDone(c)) != 0) {
                break 'l1;
            }
            if (((({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(560); (c.m.r32(a_) as i32) }) & (2i32))) != 0) {
                if ((((({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(588); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) == ((1i32).wrapping_neg())) as i32)) != 0) {
                    F::ColorFade_StartOut(c, 0i32, 0i32, 0i32, 20i32);
                }
            } else {
                if ((((({ let a_ = (c.m.r32(sym::gShop)).wrapping_add(596); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
                    F::Shop_Input(c, fp);
                }
            }
        }
        F::Shop_Term(c);
        F::Dma_ResetBuffers(c);
        break 'f (c.m.r32(fp) as i32);
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Shop_Run", &PROBE_N, &[], true); }
    r_
}
