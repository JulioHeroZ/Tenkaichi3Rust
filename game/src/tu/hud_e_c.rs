#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn HudTimer_UpdateRoot(c: &mut Ctx, mut p_node_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudTimer_UpdateRoot", &PROBE_N, &[(p_node_1) as u32], false); }
    let r_ = 'f: {
    if ((((!(((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0)) as i32)) != 0) {
        { let _ = { let a0_ = (c.m.r32(sym::gHudTimer)).wrapping_add(44); F::Ramp_Step(c, a0_) }; };
    }
    { let a1_ = rt::f2i(rt::fmul({ let a_ = ((c.m.r32(sym::gHudTimer)).wrapping_add(44)).wrapping_add(16); F32(c.m.r32(a_)) }, F32(0x00000000))); let a2_ = rt::f2i(rt::fmul({ let a_ = ((c.m.r32(sym::gHudTimer)).wrapping_add(44)).wrapping_add(16); F32(c.m.r32(a_)) }, F32(0xc3600000))); F::HudNode_SetPos(c, p_node_1, a1_, a2_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudTimer_UpdateRoot", &PROBE_N, &[], true); }
    r_
}

pub fn HudTimer_UpdateDigits(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudTimer_UpdateDigits", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_x_1: i32 = 0;
    let mut l_pulse_2: u32 = 0;
    let mut l_value_3: i32 = 0;
    let mut l_spr_4: u32 = 0;
    let mut l_d_5: i32 = 0;
    let mut l_w_6: i32 = 0;
    let mut l_hundred_7: i32 = 0;
    let mut l_hundred_8: i32 = 0;
    let mut l_ten_9: i32 = 0;
    let mut l_ten_10: i32 = 0;
    let mut l_scale_11: F32 = F32(0);
    let r_ = 'f: {
        l_x_1 = 0i32;
        l_pulse_2 = (c.m.r32(sym::gHudTimer)).wrapping_add(20);
        l_value_3 = { let a_ = (c.m.r32(sym::gHudTimer)).wrapping_add(12); (c.m.r32(a_) as i32) };
        if (((((l_value_3) >= (1000i32)) as i32)) != 0) {
            l_value_3 = 999i32;
        }
        if (((((l_value_3) < (0i32)) as i32)) != 0) {
            l_spr_4 = ({ let a_ = (c.m.r32(sym::gHudTimer)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((1i32) as u32)).wrapping_mul(28));
            F::HudSprite_Show(c, l_spr_4, 1i32);
            F::HudSprite_SetRect(c, l_spr_4, 0i32, 32i32, (32i32).wrapping_neg(), 0i32);
            F::HudSprite_SetUv(c, l_spr_4, 96i32, 128i32, 64i32, 96i32);
            F::HudSprite_Move(c, l_spr_4, 1i32, 0i32);
            l_spr_4 = ({ let a_ = (c.m.r32(sym::gHudTimer)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((2i32) as u32)).wrapping_mul(28));
            F::HudSprite_Show(c, l_spr_4, 1i32);
            F::HudSprite_SetRect(c, l_spr_4, (32i32).wrapping_neg(), 0i32, (32i32).wrapping_neg(), 0i32);
            F::HudSprite_SetUv(c, l_spr_4, 64i32, 96i32, 64i32, 96i32);
            F::HudSprite_Move(c, l_spr_4, 1i32, 0i32);
            { let a0_ = ({ let a_ = (c.m.r32(sym::gHudTimer)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((3i32) as u32)).wrapping_mul(28)); F::HudSprite_Show(c, a0_, 0i32) };
        } else {
            l_spr_4 = ({ let a_ = (c.m.r32(sym::gHudTimer)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((3i32) as u32)).wrapping_mul(28));
            {
                l_hundred_7 = 100i32;
                l_d_5 = rt::div_i32(l_value_3, l_hundred_7);
            }
            if (((((l_d_5) == (0i32)) as i32)) != 0) {
                F::HudSprite_Show(c, l_spr_4, 0i32);
            } else {
                F::HudSprite_Show(c, l_spr_4, 1i32);
                F::HudSprite_SetRect(c, l_spr_4, 0i32, 32i32, (32i32).wrapping_neg(), 0i32);
                F::HudSprite_SetUv(c, l_spr_4, (rt::rem_i32(l_d_5, 4i32)).wrapping_mul(32i32), ((rt::rem_i32(l_d_5, 4i32)).wrapping_mul(32i32)).wrapping_add(32i32), (rt::div_i32(l_d_5, 4i32)).wrapping_mul(32i32), ((rt::div_i32(l_d_5, 4i32)).wrapping_mul(32i32)).wrapping_add(32i32));
                l_x_1 = 11i32;
            }
            l_spr_4 = ({ let a_ = (c.m.r32(sym::gHudTimer)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((2i32) as u32)).wrapping_mul(28));
            {
                l_hundred_8 = 100i32;
                l_ten_9 = 10i32;
                l_d_5 = rt::div_i32(rt::rem_i32(l_value_3, l_hundred_8), l_ten_9);
            }
            if (((((((((l_x_1) == (0i32)) as i32)) != 0) && (((((l_d_5) == (0i32)) as i32)) != 0)) as i32)) != 0) {
                F::HudSprite_Show(c, l_spr_4, 0i32);
            } else {
                F::HudSprite_Show(c, l_spr_4, 1i32);
                F::HudSprite_SetRect(c, l_spr_4, l_x_1, (l_x_1).wrapping_add(32i32), (32i32).wrapping_neg(), 0i32);
                F::HudSprite_SetUv(c, l_spr_4, (rt::rem_i32(l_d_5, 4i32)).wrapping_mul(32i32), ((rt::rem_i32(l_d_5, 4i32)).wrapping_mul(32i32)).wrapping_add(32i32), (rt::div_i32(l_d_5, 4i32)).wrapping_mul(32i32), ((rt::div_i32(l_d_5, 4i32)).wrapping_mul(32i32)).wrapping_add(32i32));
                { let r_ = 11i32; let n_ = (l_x_1).wrapping_add(r_); l_x_1 = n_; };
            }
            l_spr_4 = ({ let a_ = (c.m.r32(sym::gHudTimer)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((1i32) as u32)).wrapping_mul(28));
            {
                l_ten_10 = 10i32;
                l_d_5 = rt::rem_i32(l_value_3, l_ten_10);
            }
            F::HudSprite_Show(c, l_spr_4, 1i32);
            F::HudSprite_SetRect(c, l_spr_4, l_x_1, (l_x_1).wrapping_add(32i32), (32i32).wrapping_neg(), 0i32);
            F::HudSprite_SetUv(c, l_spr_4, (rt::rem_i32(l_d_5, 4i32)).wrapping_mul(32i32), ((rt::rem_i32(l_d_5, 4i32)).wrapping_mul(32i32)).wrapping_add(32i32), (rt::div_i32(l_d_5, 4i32)).wrapping_mul(32i32), ((rt::div_i32(l_d_5, 4i32)).wrapping_mul(32i32)).wrapping_add(32i32));
            { let a0_ = ({ let a_ = (c.m.r32(sym::gHudTimer)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((1i32) as u32)).wrapping_mul(28)); F::HudSprite_Move(c, a0_, { l_w_6 = rt::div_i32(((l_x_1).wrapping_add(32i32)).wrapping_neg(), 2i32); l_w_6 }, 0i32) };
            { let a0_ = ({ let a_ = (c.m.r32(sym::gHudTimer)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((2i32) as u32)).wrapping_mul(28)); F::HudSprite_Move(c, a0_, l_w_6, 0i32) };
            { let a0_ = ({ let a_ = (c.m.r32(sym::gHudTimer)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((3i32) as u32)).wrapping_mul(28)); F::HudSprite_Move(c, a0_, l_w_6, 0i32) };
        }
        if (({ let a_ = (c.m.r32(sym::gHudTimer)).wrapping_add(68); (c.m.r32(a_) as i32) }) != 0) {
            if (((((((l_value_3) as u32)) < (((4i32) as u32))) as i32)) != 0) {
                rt::memset(c, fp, 0, 8);
                { let v_ = 0i32; c.m.w32(fp.wrapping_add(0), (v_) as u32); }
                { let v_ = 16i32; c.m.w32(fp.wrapping_add(4), (v_) as u32); }
                if (((((l_value_3) != ({ let a_ = (c.m.r32(sym::gHudTimer)).wrapping_add(16); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                    F::Ramp_Start(c, l_pulse_2, F32(0x3f000000), F32(0x3f800000), F32(0x00000000));
                }
                l_scale_11 = rt::fadd(F32(c.m.r32((l_pulse_2).wrapping_add(16))), F32(0x3f800000));
                l_spr_4 = ({ let a_ = (c.m.r32(sym::gHudTimer)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((1i32) as u32)).wrapping_mul(28));
                F::HudSprite_SetTex(c, l_spr_4, 0i32, 2i32);
                { let a1_ = (c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32); let a2_ = (c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32); F::HudSprite_Move(c, l_spr_4, a1_, a2_) };
                F::HudSprite_Scale(c, l_spr_4, l_scale_11, l_scale_11);
                { let a1_ = ((c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)).wrapping_neg(); let a2_ = ((c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)).wrapping_neg(); F::HudSprite_Move(c, l_spr_4, a1_, a2_) };
                l_spr_4 = ({ let a_ = (c.m.r32(sym::gHudTimer)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((2i32) as u32)).wrapping_mul(28));
                F::HudSprite_SetTex(c, l_spr_4, 0i32, 2i32);
                { let a1_ = (c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32); let a2_ = (c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32); F::HudSprite_Move(c, l_spr_4, a1_, a2_) };
                F::HudSprite_Scale(c, l_spr_4, l_scale_11, l_scale_11);
                { let a1_ = ((c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)).wrapping_neg(); let a2_ = ((c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)).wrapping_neg(); F::HudSprite_Move(c, l_spr_4, a1_, a2_) };
                l_spr_4 = ({ let a_ = (c.m.r32(sym::gHudTimer)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((3i32) as u32)).wrapping_mul(28));
                F::HudSprite_SetTex(c, l_spr_4, 0i32, 2i32);
                { let a1_ = (c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32); let a2_ = (c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32); F::HudSprite_Move(c, l_spr_4, a1_, a2_) };
                F::HudSprite_Scale(c, l_spr_4, l_scale_11, l_scale_11);
                { let a1_ = ((c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)).wrapping_neg(); let a2_ = ((c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)).wrapping_neg(); F::HudSprite_Move(c, l_spr_4, a1_, a2_) };
                if ((((!(((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0)) as i32)) != 0) {
                    { let _ = F::Ramp_Step(c, l_pulse_2); };
                }
            } else {
                if (((((((l_value_3) as u32)) < (((10i32) as u32))) as i32)) != 0) {
                    { let a0_ = ({ let a_ = (c.m.r32(sym::gHudTimer)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((1i32) as u32)).wrapping_mul(28)); F::HudSprite_SetTex(c, a0_, 0i32, 1i32) };
                    { let a0_ = ({ let a_ = (c.m.r32(sym::gHudTimer)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((2i32) as u32)).wrapping_mul(28)); F::HudSprite_SetTex(c, a0_, 0i32, 1i32) };
                    { let a0_ = ({ let a_ = (c.m.r32(sym::gHudTimer)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((3i32) as u32)).wrapping_mul(28)); F::HudSprite_SetTex(c, a0_, 0i32, 1i32) };
                } else {
                    { let a0_ = ({ let a_ = (c.m.r32(sym::gHudTimer)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((1i32) as u32)).wrapping_mul(28)); F::HudSprite_SetTex(c, a0_, 0i32, 0i32) };
                    { let a0_ = ({ let a_ = (c.m.r32(sym::gHudTimer)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((2i32) as u32)).wrapping_mul(28)); F::HudSprite_SetTex(c, a0_, 0i32, 0i32) };
                    { let a0_ = ({ let a_ = (c.m.r32(sym::gHudTimer)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((3i32) as u32)).wrapping_mul(28)); F::HudSprite_SetTex(c, a0_, 0i32, 0i32) };
                }
            }
        } else {
            { let a0_ = ({ let a_ = (c.m.r32(sym::gHudTimer)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((1i32) as u32)).wrapping_mul(28)); F::HudSprite_SetTex(c, a0_, 0i32, 0i32) };
            { let a0_ = ({ let a_ = (c.m.r32(sym::gHudTimer)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((2i32) as u32)).wrapping_mul(28)); F::HudSprite_SetTex(c, a0_, 0i32, 0i32) };
            { let a0_ = ({ let a_ = (c.m.r32(sym::gHudTimer)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((3i32) as u32)).wrapping_mul(28)); F::HudSprite_SetTex(c, a0_, 0i32, 0i32) };
        }
        { let a0_ = ({ let a_ = (c.m.r32(sym::gHudTimer)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((1i32) as u32)).wrapping_mul(28)); F::HudSprite_Move(c, a0_, (2i32).wrapping_neg(), 4i32) };
        { let a0_ = ({ let a_ = (c.m.r32(sym::gHudTimer)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((2i32) as u32)).wrapping_mul(28)); F::HudSprite_Move(c, a0_, (2i32).wrapping_neg(), 4i32) };
        { let a0_ = ({ let a_ = (c.m.r32(sym::gHudTimer)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((3i32) as u32)).wrapping_mul(28)); F::HudSprite_Move(c, a0_, (2i32).wrapping_neg(), 4i32) };
        { let a0_ = ({ let a_ = (c.m.r32(sym::gHudTimer)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((5i32) as u32)).wrapping_mul(28)); let a1_ = { let a_ = (c.m.r32(sym::gHudTimer)).wrapping_add(72); (c.m.r32(a_) as i32) }; F::HudSprite_Show(c, a0_, a1_) };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "HudTimer_UpdateDigits", &PROBE_N, &[], true); }
    r_
}

pub fn HudTimer_Draw(c: &mut Ctx, mut p_node_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudTimer_Draw", &PROBE_N, &[(p_node_1) as u32], false); }
    let mut l_i_2: u32 = 0;
    let r_ = 'f: {
    {
        l_i_2 = ((0i32) as u32);
        'l1: loop {
            if !(((((l_i_2) < (c.m.r32((p_node_1).wrapping_add(32)))) as i32)) != 0) { break 'l1; }
            'c2: {
                { let a0_ = { let a_ = (c.m.r32((p_node_1).wrapping_add(36))).wrapping_add((l_i_2).wrapping_mul(4)); c.m.r32(a_) }; let a1_ = { let a_ = c.m.r32(sym::gHudTimer); c.m.r32(a_) }; F::HudSprite_Draw(c, a0_, a1_, 0i32) };
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudTimer_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn HudTimer_SlideOut(c: &mut Ctx, mut p_seconds_1: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudTimer_SlideOut", &PROBE_N, &[(p_seconds_1).0], false); }
    let r_ = 'f: {
    { let a0_ = (c.m.r32(sym::gHudTimer)).wrapping_add(44); F::Ramp_Start(c, a0_, p_seconds_1, F32(0x00000000), F32(0x3f800000)) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudTimer_SlideOut", &PROBE_N, &[], true); }
    r_
}

pub fn HudTimer_SlideIn(c: &mut Ctx, mut p_seconds_1: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudTimer_SlideIn", &PROBE_N, &[(p_seconds_1).0], false); }
    let r_ = 'f: {
    { let a0_ = (c.m.r32(sym::gHudTimer)).wrapping_add(44); F::Ramp_Start(c, a0_, p_seconds_1, F32(0x3f800000), F32(0x00000000)) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudTimer_SlideIn", &PROBE_N, &[], true); }
    r_
}

pub fn HudTimer_Init(c: &mut Ctx, mut p_out_1: u32, mut p_res_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudTimer_Init", &PROBE_N, &[(p_out_1) as u32, (p_res_2) as u32], false); }
    let mut l_spr_3: u32 = 0;
    let mut l_node_4: u32 = 0;
    let r_ = 'f: {
    { let v_ = F::Heap_Alloc(c, ((76u32) as i32), ((32i32) as u32), 0i32, 2i32); let a_ = sym::gHudTimer; c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32(sym::gHudTimer); F::memset(c, a0_, 0i32, 76u32) }; };
    { let v_ = F::Heap_Alloc(c, (((((6i32) as u32)).wrapping_mul(28u32)) as i32), ((32i32) as u32), 0i32, 2i32); let a_ = (c.m.r32(sym::gHudTimer)).wrapping_add(4); c.m.w32(a_, v_); };
    { let _ = { let a0_ = { let a_ = (c.m.r32(sym::gHudTimer)).wrapping_add(4); c.m.r32(a_) }; F::memset(c, a0_, 0i32, (((6i32) as u32)).wrapping_mul(28u32)) }; };
    { let v_ = F::Heap_Alloc(c, (((((2i32) as u32)).wrapping_mul(56u32)) as i32), ((32i32) as u32), 0i32, 2i32); let a_ = (c.m.r32(sym::gHudTimer)).wrapping_add(8); c.m.w32(a_, v_); };
    { let _ = { let a0_ = { let a_ = (c.m.r32(sym::gHudTimer)).wrapping_add(8); c.m.r32(a_) }; F::memset(c, a0_, 0i32, (((2i32) as u32)).wrapping_mul(56u32)) }; };
    { let v_ = p_res_2; let a_ = c.m.r32(sym::gHudTimer); c.m.w32(a_, v_); };
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudTimer)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((0i32) as u32)).wrapping_mul(28));
    F::HudSprite_InitTex(c, l_spr_3, p_res_2, 3i32, 0i32);
    F::HudSprite_SetRect(c, l_spr_3, 0i32, 64i32, 0i32, 30i32);
    F::HudSprite_SetUv(c, l_spr_3, 0i32, 64i32, 0i32, 30i32);
    F::HudSprite_Center(c, l_spr_3);
    F::HudSprite_Move(c, l_spr_3, 0i32, (7i32).wrapping_neg());
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudTimer)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((1i32) as u32)).wrapping_mul(28));
    F::HudSprite_InitTex(c, l_spr_3, p_res_2, 0i32, 0i32);
    F::HudSprite_SetRect(c, l_spr_3, (10i32).wrapping_neg(), 22i32, (16i32).wrapping_neg(), 16i32);
    F::HudSprite_SetUv(c, l_spr_3, 0i32, 32i32, 0i32, 32i32);
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudTimer)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((5i32) as u32)).wrapping_mul(28));
    F::HudSprite_InitTex(c, l_spr_3, p_res_2, 5i32, 0i32);
    F::HudSprite_Center(c, l_spr_3);
    F::HudSprite_Move(c, l_spr_3, 0i32, 16i32);
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudTimer)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((2i32) as u32)).wrapping_mul(28));
    F::HudSprite_InitTex(c, l_spr_3, p_res_2, 0i32, 0i32);
    F::HudSprite_SetRect(c, l_spr_3, (22i32).wrapping_neg(), 10i32, (16i32).wrapping_neg(), 16i32);
    F::HudSprite_SetUv(c, l_spr_3, 0i32, 32i32, 0i32, 32i32);
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudTimer)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((3i32) as u32)).wrapping_mul(28));
    F::HudSprite_InitTex(c, l_spr_3, p_res_2, 0i32, 0i32);
    F::HudSprite_SetRect(c, l_spr_3, (22i32).wrapping_neg(), 10i32, (16i32).wrapping_neg(), 16i32);
    F::HudSprite_SetUv(c, l_spr_3, 0i32, 32i32, 0i32, 32i32);
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudTimer)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((4i32) as u32)).wrapping_mul(28));
    F::HudSprite_InitTex(c, l_spr_3, p_res_2, 4i32, 0i32);
    F::HudSprite_Center(c, l_spr_3);
    F::HudSprite_Show(c, l_spr_3, 0i32);
    F::HudSprite_Move(c, l_spr_3, 0i32, 26i32);
    l_node_4 = ({ let a_ = (c.m.r32(sym::gHudTimer)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((1i32) as u32)).wrapping_mul(56));
    { let v_ = 256i32; let a_ = (l_node_4).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
    { let v_ = 37i32; let a_ = (l_node_4).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
    { let v_ = ((6i32) as u32); let a_ = (l_node_4).wrapping_add(32); c.m.w32(a_, v_); };
    { let v_ = { let a0_ = (((c.m.r32((l_node_4).wrapping_add(32))).wrapping_mul(4u32)) as i32); F::Heap_Alloc(c, a0_, ((32i32) as u32), 0i32, 2i32) }; let a_ = (l_node_4).wrapping_add(36); c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32((l_node_4).wrapping_add(36)); let a2_ = (c.m.r32((l_node_4).wrapping_add(32))).wrapping_mul(4u32); F::memset(c, a0_, 0i32, a2_) }; };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudTimer)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((0i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_node_4).wrapping_add(36))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudTimer)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((1i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_node_4).wrapping_add(36))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudTimer)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((2i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_node_4).wrapping_add(36))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudTimer)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((3i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_node_4).wrapping_add(36))).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudTimer)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((4i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_node_4).wrapping_add(36))).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudTimer)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((5i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_node_4).wrapping_add(36))).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = fid::HudTimer_UpdateDigits; let a_ = (l_node_4).wrapping_add(48); c.m.w32(a_, v_); };
    { let v_ = fid::HudTimer_Draw; let a_ = (l_node_4).wrapping_add(52); c.m.w32(a_, v_); };
    l_node_4 = ({ let a_ = (c.m.r32(sym::gHudTimer)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((0i32) as u32)).wrapping_mul(56));
    { let v_ = 0i32; let a_ = (l_node_4).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
    { let v_ = 0i32; let a_ = (l_node_4).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
    { let v_ = ((1i32) as u32); let a_ = (l_node_4).wrapping_add(40); c.m.w32(a_, v_); };
    { let v_ = { let a0_ = (((c.m.r32((l_node_4).wrapping_add(40))).wrapping_mul(4u32)) as i32); F::Heap_Alloc(c, a0_, ((32i32) as u32), 0i32, 2i32) }; let a_ = (l_node_4).wrapping_add(44); c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32((l_node_4).wrapping_add(44)); let a2_ = (c.m.r32((l_node_4).wrapping_add(40))).wrapping_mul(4u32); F::memset(c, a0_, 0i32, a2_) }; };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudTimer)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((1i32) as u32)).wrapping_mul(56)); let a_ = (c.m.r32((l_node_4).wrapping_add(44))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = fid::HudTimer_UpdateRoot; let a_ = (l_node_4).wrapping_add(48); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (l_node_4).wrapping_add(52); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudTimer)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((0i32) as u32)).wrapping_mul(56)); let a_ = p_out_1; c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudTimer_Init", &PROBE_N, &[], true); }
    r_
}

pub fn HudTimer_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudTimer_Term", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let r_ = 'f: {
    if ((((({ let a_ = (c.m.r32(sym::gHudTimer)).wrapping_add(4); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = { let a_ = (c.m.r32(sym::gHudTimer)).wrapping_add(4); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
    }
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (2i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((((({ let a_ = (({ let a_ = (c.m.r32(sym::gHudTimer)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((l_i_1) as u32)).wrapping_mul(56))).wrapping_add(36); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
                    { let a0_ = { let a_ = (({ let a_ = (c.m.r32(sym::gHudTimer)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((l_i_1) as u32)).wrapping_mul(56))).wrapping_add(36); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
                }
                if ((((({ let a_ = (({ let a_ = (c.m.r32(sym::gHudTimer)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((l_i_1) as u32)).wrapping_mul(56))).wrapping_add(44); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
                    { let a0_ = { let a_ = (({ let a_ = (c.m.r32(sym::gHudTimer)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((l_i_1) as u32)).wrapping_mul(56))).wrapping_add(44); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
                }
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
    if ((((({ let a_ = (c.m.r32(sym::gHudTimer)).wrapping_add(8); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = { let a_ = (c.m.r32(sym::gHudTimer)).wrapping_add(8); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
    }
    if (((((c.m.r32(sym::gHudTimer)) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32(sym::gHudTimer); F::Heap_Free(c, a0_) };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudTimer_Term", &PROBE_N, &[], true); }
    r_
}

pub fn HudTimer_SetTime(c: &mut Ctx, mut p_seconds_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudTimer_SetTime", &PROBE_N, &[(p_seconds_1) as u32], false); }
    let mut l_spr_2: u32 = 0;
    let r_ = 'f: {
    { let v_ = { let a_ = (c.m.r32(sym::gHudTimer)).wrapping_add(12); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gHudTimer)).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
    { let v_ = p_seconds_1; let a_ = (c.m.r32(sym::gHudTimer)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
    { let v_ = 1i32; let a_ = (c.m.r32(sym::gHudTimer)).wrapping_add(68); c.m.w32(a_, (v_) as u32); };
    l_spr_2 = ({ let a_ = (c.m.r32(sym::gHudTimer)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((0i32) as u32)).wrapping_mul(28));
    F::HudSprite_SetRect(c, l_spr_2, 0i32, 64i32, 0i32, 30i32);
    F::HudSprite_SetUv(c, l_spr_2, 0i32, 64i32, 0i32, 30i32);
    F::HudSprite_Center(c, l_spr_2);
    F::HudSprite_Move(c, l_spr_2, 0i32, (7i32).wrapping_neg());
    { let a0_ = ({ let a_ = (c.m.r32(sym::gHudTimer)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((4i32) as u32)).wrapping_mul(28)); F::HudSprite_Show(c, a0_, 0i32) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudTimer_SetTime", &PROBE_N, &[], true); }
    r_
}

pub fn HudTimer_SetCount(c: &mut Ctx, mut p_count_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudTimer_SetCount", &PROBE_N, &[(p_count_1) as u32], false); }
    let mut l_spr_2: u32 = 0;
    let r_ = 'f: {
    { let v_ = p_count_1; let a_ = (c.m.r32(sym::gHudTimer)).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
    { let v_ = p_count_1; let a_ = (c.m.r32(sym::gHudTimer)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
    { let v_ = 0i32; let a_ = (c.m.r32(sym::gHudTimer)).wrapping_add(68); c.m.w32(a_, (v_) as u32); };
    l_spr_2 = ({ let a_ = (c.m.r32(sym::gHudTimer)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((0i32) as u32)).wrapping_mul(28));
    F::HudSprite_SetRect(c, l_spr_2, 0i32, 64i32, 0i32, 32i32);
    F::HudSprite_SetUv(c, l_spr_2, 0i32, 64i32, 32i32, 64i32);
    F::HudSprite_Center(c, l_spr_2);
    F::HudSprite_Move(c, l_spr_2, 0i32, (6i32).wrapping_neg());
    { let a0_ = ({ let a_ = (c.m.r32(sym::gHudTimer)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((4i32) as u32)).wrapping_mul(28)); F::HudSprite_Show(c, a0_, 1i32) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudTimer_SetCount", &PROBE_N, &[], true); }
    r_
}

pub fn HudTimer_ShowMark(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudTimer_ShowMark", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let v_ = 1i32; let a_ = (c.m.r32(sym::gHudTimer)).wrapping_add(72); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudTimer_ShowMark", &PROBE_N, &[], true); }
    r_
}

pub fn HudTimer_ShowMark2(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudTimer_ShowMark2", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let v_ = 1i32; let a_ = (c.m.r32(sym::gHudTimer)).wrapping_add(72); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudTimer_ShowMark2", &PROBE_N, &[], true); }
    r_
}

pub fn HudTimer_Reset(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudTimer_Reset", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let v_ = 0i32; let a_ = (c.m.r32(sym::gHudTimer)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
    { let v_ = 0i32; let a_ = (c.m.r32(sym::gHudTimer)).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
    { let v_ = 0i32; let a_ = (c.m.r32(sym::gHudTimer)).wrapping_add(72); c.m.w32(a_, (v_) as u32); };
    { let _ = { let a0_ = (c.m.r32(sym::gHudTimer)).wrapping_add(20); F::memset(c, a0_, 0i32, 24u32) }; };
    { let _ = { let a0_ = (c.m.r32(sym::gHudTimer)).wrapping_add(44); F::memset(c, a0_, 0i32, 24u32) }; };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudTimer_Reset", &PROBE_N, &[], true); }
    r_
}
