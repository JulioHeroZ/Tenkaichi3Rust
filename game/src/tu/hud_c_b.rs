#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn hud_c_b__HudCombo_Zero(c: &mut Ctx, mut p_p_1: u32, mut p_size_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "hud_c_b__HudCombo_Zero", &PROBE_N, &[(p_p_1) as u32, (p_size_2) as u32], false); }
    let r_ = 'f: {
    { let _ = F::memset(c, p_p_1, 0i32, p_size_2); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "hud_c_b__HudCombo_Zero", &PROBE_N, &[], true); }
    r_
}

pub fn HudCombo_UpdateRoot(c: &mut Ctx, mut p_node_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudCombo_UpdateRoot", &PROBE_N, &[(p_node_1) as u32], false); }
    let r_ = 'f: {
    if ((((((((!(((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0)) as i32)) != 0) && ((((({ let a_ = (c.m.r32(sym::gHudCombo)).wrapping_add(12); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0)) as i32)) != 0) {
        { let _ = { let a0_ = (c.m.r32(sym::gHudCombo)).wrapping_add(344); F::Ramp_Step(c, a0_) }; };
    }
    if ((((({ let a_ = (c.m.r32(sym::gHudCombo)).wrapping_add(12); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
        { let a1_ = rt::f2i(rt::fmul({ let a_ = ((c.m.r32(sym::gHudCombo)).wrapping_add(344)).wrapping_add(16); F32(c.m.r32(a_)) }, F32(0xc3800000))); let a2_ = rt::f2i(rt::fmul({ let a_ = ((c.m.r32(sym::gHudCombo)).wrapping_add(344)).wrapping_add(16); F32(c.m.r32(a_)) }, F32(0x00000000))); F::HudNode_SetPos(c, p_node_1, a1_, a2_) };
    } else {
        { let a1_ = rt::f2i(rt::fmul({ let a_ = ((c.m.r32(sym::gHudCombo)).wrapping_add(344)).wrapping_add(16); F32(c.m.r32(a_)) }, F32(0x43800000))); let a2_ = rt::f2i(rt::fmul({ let a_ = ((c.m.r32(sym::gHudCombo)).wrapping_add(344)).wrapping_add(16); F32(c.m.r32(a_)) }, F32(0x00000000))); F::HudNode_SetPos(c, p_node_1, a1_, a2_) };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudCombo_UpdateRoot", &PROBE_N, &[], true); }
    r_
}

pub fn HudCombo_UpdateMessage(c: &mut Ctx, mut p_node_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudCombo_UpdateMessage", &PROBE_N, &[(p_node_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_msg_2: i32 = 0;
    let mut l_ramp_3: u32 = 0;
    let mut l_spr_4: u32 = 0;
    let mut l_width_5: F32 = F32(0);
    let mut l_done_6: i32 = 0;
    let r_ = 'f: {
        l_msg_2 = { let a_ = ((c.m.r32(sym::gHudCombo)).wrapping_add(48)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudCombo)).wrapping_add(12); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
        l_ramp_3 = ((c.m.r32(sym::gHudCombo)).wrapping_add(104)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudCombo)).wrapping_add(12); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(24));
        F::hud_c_b__HudCombo_Zero(c, fp, 8u32);
        l_spr_4 = ({ let a_ = (c.m.r32(sym::gHudCombo)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((0i32) as u32)).wrapping_mul(28));
        's1: {
            let sv7: i32 = l_msg_2;
            match sv7 {
                0i32 => {
                    { let a1_ = { let a_ = c.m.r32(sym::gHudCombo); c.m.r32(a_) }; F::HudSprite_InitTex(c, l_spr_4, a1_, 3i32, 0i32) };
                    break 's1;
                }
                1i32 => {
                    { let a1_ = { let a_ = c.m.r32(sym::gHudCombo); c.m.r32(a_) }; F::HudSprite_InitTex(c, l_spr_4, a1_, 4i32, 0i32) };
                    break 's1;
                }
                2i32 => {
                    { let a1_ = { let a_ = c.m.r32(sym::gHudCombo); c.m.r32(a_) }; F::HudSprite_InitTex(c, l_spr_4, a1_, 5i32, 0i32) };
                    break 's1;
                }
                3i32 => {
                    { let a1_ = { let a_ = c.m.r32(sym::gHudCombo); c.m.r32(a_) }; F::HudSprite_InitTex(c, l_spr_4, a1_, 6i32, 0i32) };
                    break 's1;
                }
                4i32 => {
                    { let a1_ = { let a_ = c.m.r32(sym::gHudCombo); c.m.r32(a_) }; F::HudSprite_InitTex(c, l_spr_4, a1_, 7i32, 0i32) };
                    break 's1;
                }
                _ => {
                }
            }
        }
        l_width_5 = rt::fneg(rt::i2f(((((c.m.r16((l_spr_4).wrapping_add(18)) as i16)) as i32)).wrapping_sub((((c.m.r16((l_spr_4).wrapping_add(16)) as i16)) as i32))));
        F::HudSprite_Show(c, l_spr_4, 1i32);
        's2: {
            let sv8: i32 = { let a_ = ((c.m.r32(sym::gHudCombo)).wrapping_add(16)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudCombo)).wrapping_add(12); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
            'k7: {
            'k6: {
            'k5: {
            'k4: {
            'k3: {
                match sv8 { 0i32 => break 'k3, 1i32 => break 'k4, 2i32 => break 'k5, 3i32 => break 'k6, 4i32 => break 'k7, _ => break 'k7, }
            }
            F::Ramp_Start(c, l_ramp_3, F32(0x3dcccccc), F32(0x3f800000), F32(0x00000000));
            { let a_ = ((c.m.r32(sym::gHudCombo)).wrapping_add(16)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudCombo)).wrapping_add(12); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
            }
            if ((((!(((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0)) as i32)) != 0) {
                l_done_6 = F::Ramp_Step(c, l_ramp_3);
            } else {
                l_done_6 = 0i32;
            }
            if ((l_done_6) != 0) {
                F::Ramp_Start(c, l_ramp_3, F32(0x3f999999), F32(0x00000000), F32(0x3f800000));
                { let a_ = ((c.m.r32(sym::gHudCombo)).wrapping_add(16)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudCombo)).wrapping_add(12); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = F32(0x00000000); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            } else {
                { let v_ = rt::fmul(l_width_5, F32(c.m.r32((l_ramp_3).wrapping_add(16)))); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            }
            break 's2;
            }
            if ((((!(((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0)) as i32)) != 0) {
                l_done_6 = F::Ramp_Step(c, l_ramp_3);
            } else {
                l_done_6 = 0i32;
            }
            if ((l_done_6) != 0) {
                F::Ramp_Start(c, l_ramp_3, F32(0x3dcccccc), F32(0x00000000), F32(0x3f800000));
                { let a_ = ((c.m.r32(sym::gHudCombo)).wrapping_add(16)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudCombo)).wrapping_add(12); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
            }
            break 's2;
            }
            if ((((!(((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0)) as i32)) != 0) {
                l_done_6 = F::Ramp_Step(c, l_ramp_3);
            } else {
                l_done_6 = 0i32;
            }
            if ((l_done_6) != 0) {
                F::HudSprite_Show(c, l_spr_4, 0i32);
                { let a_ = ((c.m.r32(sym::gHudCombo)).wrapping_add(16)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudCombo)).wrapping_add(12); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
            } else {
                { let v_ = rt::fmul(l_width_5, F32(c.m.r32((l_ramp_3).wrapping_add(16)))); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            }
            break 's2;
            }
            F::HudSprite_Show(c, l_spr_4, 0i32);
            break 'f;
        }
        { let r_ = F32(0x40a00000); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        if (((((l_msg_2) == (4i32)) as i32)) != 0) {
            { let r_ = F32(0x439b0000); let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        }
        if ((((({ let a_ = (c.m.r32(sym::gHudCombo)).wrapping_add(12); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
            { let a1_ = rt::f2i(F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a2_ = rt::f2i(F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); F::HudNode_SetOfs(c, p_node_1, a1_, a2_) };
        } else {
            { let a1_ = (372i32).wrapping_sub(rt::f2i(F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)))))); let a2_ = rt::f2i(F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); F::HudNode_SetOfs(c, p_node_1, a1_, a2_) };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "HudCombo_UpdateMessage", &PROBE_N, &[], true); }
    r_
}

pub fn HudCombo_UpdateText(c: &mut Ctx, mut p_node_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudCombo_UpdateText", &PROBE_N, &[(p_node_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_alpha_2: i32 = 0;
    let mut l_ramp_3: u32 = 0;
    let mut l_spr_4: u32 = 0;
    let mut l_width_5: F32 = F32(0);
    let mut l_done_6: i32 = 0;
    let r_ = 'f: {
        l_alpha_2 = 128i32;
        l_ramp_3 = ((c.m.r32(sym::gHudCombo)).wrapping_add(152)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudCombo)).wrapping_add(12); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(24));
        F::hud_c_b__HudCombo_Zero(c, fp, 8u32);
        l_spr_4 = ({ let a_ = (c.m.r32(sym::gHudCombo)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((1i32) as u32)).wrapping_mul(28));
        l_width_5 = rt::fneg(rt::i2f(((((c.m.r16((l_spr_4).wrapping_add(18)) as i16)) as i32)).wrapping_sub((((c.m.r16((l_spr_4).wrapping_add(16)) as i16)) as i32))));
        F::HudSprite_Show(c, l_spr_4, 1i32);
        's1: {
            let sv7: i32 = { let a_ = ((c.m.r32(sym::gHudCombo)).wrapping_add(40)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudCombo)).wrapping_add(12); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
            'k6: {
            'k5: {
            'k4: {
            'k3: {
            'k2: {
                match sv7 { 0i32 => break 'k2, 1i32 => break 'k3, 2i32 => break 'k4, 3i32 => break 'k5, 4i32 => break 'k6, _ => break 'k6, }
            }
            F::Ramp_Start(c, l_ramp_3, F32(0x3dcccccc), F32(0x00000000), F32(0x3f800000));
            { let a_ = ((c.m.r32(sym::gHudCombo)).wrapping_add(40)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudCombo)).wrapping_add(12); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
            }
            if ((((!(((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0)) as i32)) != 0) {
                l_done_6 = F::Ramp_Step(c, l_ramp_3);
            } else {
                l_done_6 = 0i32;
            }
            if ((l_done_6) != 0) {
                F::Ramp_Start(c, l_ramp_3, F32(0x3f999999), F32(0x00000000), F32(0x3f800000));
                l_alpha_2 = 128i32;
                { let a_ = ((c.m.r32(sym::gHudCombo)).wrapping_add(40)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudCombo)).wrapping_add(12); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = F32(0x00000000); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            } else {
                { let v_ = rt::fmul(l_width_5, F32(c.m.r32((l_ramp_3).wrapping_add(16)))); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                l_alpha_2 = ((((rt::f2i(rt::fmul(F32(c.m.r32((l_ramp_3).wrapping_add(16))), F32(0x43000000)))) as u8)) as i32);
            }
            break 's1;
            }
            if ((((!(((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0)) as i32)) != 0) {
                l_done_6 = F::Ramp_Step(c, l_ramp_3);
            } else {
                l_done_6 = 0i32;
            }
            if ((l_done_6) != 0) {
                F::Ramp_Start(c, l_ramp_3, F32(0x3dcccccc), F32(0x3f800000), F32(0x00000000));
                { let a_ = ((c.m.r32(sym::gHudCombo)).wrapping_add(40)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudCombo)).wrapping_add(12); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
            }
            break 's1;
            }
            if ((((!(((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0)) as i32)) != 0) {
                l_done_6 = F::Ramp_Step(c, l_ramp_3);
            } else {
                l_done_6 = 0i32;
            }
            if ((l_done_6) != 0) {
                F::HudSprite_Show(c, l_spr_4, 0i32);
                l_alpha_2 = 0i32;
                { let a_ = ((c.m.r32(sym::gHudCombo)).wrapping_add(40)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudCombo)).wrapping_add(12); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
            } else {
                { let v_ = rt::fmul(l_width_5, F32(c.m.r32((l_ramp_3).wrapping_add(16)))); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                l_alpha_2 = ((((rt::f2i(rt::fmul(F32(c.m.r32((l_ramp_3).wrapping_add(16))), F32(0x43000000)))) as u8)) as i32);
            }
            break 's1;
            }
            F::HudSprite_Show(c, l_spr_4, 0i32);
            break 'f;
        }
        F::HudSprite_SetColor(c, l_spr_4, 128i32, 128i32, 128i32, l_alpha_2);
        { let r_ = F32(0x41700000); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        if ((((({ let a_ = (c.m.r32(sym::gHudCombo)).wrapping_add(12); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
            { let a1_ = rt::f2i(F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a2_ = rt::f2i(F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); F::HudNode_SetOfs(c, p_node_1, a1_, a2_) };
        } else {
            { let a1_ = (512i32).wrapping_sub(rt::f2i(F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)))))); let a2_ = rt::f2i(F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); F::HudNode_SetOfs(c, p_node_1, a1_, a2_) };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "HudCombo_UpdateText", &PROBE_N, &[], true); }
    r_
}

pub fn HudCombo_SetHitDigits(c: &mut Ctx, mut p_node_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudCombo_SetHitDigits", &PROBE_N, &[(p_node_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(112) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 112);
    let mut l_hits_2: u32 = 0;
    let mut l_i_3: i32 = 0;
    let mut l_spr_4: u32 = 0;
    let mut l_digit_5: u32 = 0;
    let mut l_step_6: i32 = 0;
    let r_ = 'f: {
        rt::memset(c, fp, 0, 24);
        { let v_ = ((1i32) as u32); c.m.w32(fp.wrapping_add(0), v_); }
        { let v_ = ((10i32) as u32); c.m.w32(fp.wrapping_add(4), v_); }
        { let v_ = ((100i32) as u32); c.m.w32(fp.wrapping_add(8), v_); }
        { let v_ = ((1000i32) as u32); c.m.w32(fp.wrapping_add(12), v_); }
        { let v_ = ((10000i32) as u32); c.m.w32(fp.wrapping_add(16), v_); }
        { let v_ = ((100000i32) as u32); c.m.w32(fp.wrapping_add(20), v_); }
        rt::memset(c, fp.wrapping_add(24), 0, 80);
        rt::memset(c, fp.wrapping_add(24).wrapping_add(0), 0, 8);
        { let v_ = ((1i32) as u16); c.m.w16(fp.wrapping_add(24).wrapping_add(0).wrapping_add(0), v_); }
        { let v_ = ((31i32) as u16); c.m.w16(fp.wrapping_add(24).wrapping_add(0).wrapping_add(2), v_); }
        { let v_ = ((0i32) as u16); c.m.w16(fp.wrapping_add(24).wrapping_add(0).wrapping_add(4), v_); }
        { let v_ = ((32i32) as u16); c.m.w16(fp.wrapping_add(24).wrapping_add(0).wrapping_add(6), v_); }
        rt::memset(c, fp.wrapping_add(24).wrapping_add(8), 0, 8);
        { let v_ = ((33i32) as u16); c.m.w16(fp.wrapping_add(24).wrapping_add(8).wrapping_add(0), v_); }
        { let v_ = ((63i32) as u16); c.m.w16(fp.wrapping_add(24).wrapping_add(8).wrapping_add(2), v_); }
        { let v_ = ((0i32) as u16); c.m.w16(fp.wrapping_add(24).wrapping_add(8).wrapping_add(4), v_); }
        { let v_ = ((32i32) as u16); c.m.w16(fp.wrapping_add(24).wrapping_add(8).wrapping_add(6), v_); }
        rt::memset(c, fp.wrapping_add(24).wrapping_add(16), 0, 8);
        { let v_ = ((65i32) as u16); c.m.w16(fp.wrapping_add(24).wrapping_add(16).wrapping_add(0), v_); }
        { let v_ = ((95i32) as u16); c.m.w16(fp.wrapping_add(24).wrapping_add(16).wrapping_add(2), v_); }
        { let v_ = ((0i32) as u16); c.m.w16(fp.wrapping_add(24).wrapping_add(16).wrapping_add(4), v_); }
        { let v_ = ((32i32) as u16); c.m.w16(fp.wrapping_add(24).wrapping_add(16).wrapping_add(6), v_); }
        rt::memset(c, fp.wrapping_add(24).wrapping_add(24), 0, 8);
        { let v_ = ((97i32) as u16); c.m.w16(fp.wrapping_add(24).wrapping_add(24).wrapping_add(0), v_); }
        { let v_ = ((127i32) as u16); c.m.w16(fp.wrapping_add(24).wrapping_add(24).wrapping_add(2), v_); }
        { let v_ = ((0i32) as u16); c.m.w16(fp.wrapping_add(24).wrapping_add(24).wrapping_add(4), v_); }
        { let v_ = ((32i32) as u16); c.m.w16(fp.wrapping_add(24).wrapping_add(24).wrapping_add(6), v_); }
        rt::memset(c, fp.wrapping_add(24).wrapping_add(32), 0, 8);
        { let v_ = ((1i32) as u16); c.m.w16(fp.wrapping_add(24).wrapping_add(32).wrapping_add(0), v_); }
        { let v_ = ((31i32) as u16); c.m.w16(fp.wrapping_add(24).wrapping_add(32).wrapping_add(2), v_); }
        { let v_ = ((32i32) as u16); c.m.w16(fp.wrapping_add(24).wrapping_add(32).wrapping_add(4), v_); }
        { let v_ = ((64i32) as u16); c.m.w16(fp.wrapping_add(24).wrapping_add(32).wrapping_add(6), v_); }
        rt::memset(c, fp.wrapping_add(24).wrapping_add(40), 0, 8);
        { let v_ = ((33i32) as u16); c.m.w16(fp.wrapping_add(24).wrapping_add(40).wrapping_add(0), v_); }
        { let v_ = ((63i32) as u16); c.m.w16(fp.wrapping_add(24).wrapping_add(40).wrapping_add(2), v_); }
        { let v_ = ((32i32) as u16); c.m.w16(fp.wrapping_add(24).wrapping_add(40).wrapping_add(4), v_); }
        { let v_ = ((64i32) as u16); c.m.w16(fp.wrapping_add(24).wrapping_add(40).wrapping_add(6), v_); }
        rt::memset(c, fp.wrapping_add(24).wrapping_add(48), 0, 8);
        { let v_ = ((65i32) as u16); c.m.w16(fp.wrapping_add(24).wrapping_add(48).wrapping_add(0), v_); }
        { let v_ = ((95i32) as u16); c.m.w16(fp.wrapping_add(24).wrapping_add(48).wrapping_add(2), v_); }
        { let v_ = ((32i32) as u16); c.m.w16(fp.wrapping_add(24).wrapping_add(48).wrapping_add(4), v_); }
        { let v_ = ((64i32) as u16); c.m.w16(fp.wrapping_add(24).wrapping_add(48).wrapping_add(6), v_); }
        rt::memset(c, fp.wrapping_add(24).wrapping_add(56), 0, 8);
        { let v_ = ((97i32) as u16); c.m.w16(fp.wrapping_add(24).wrapping_add(56).wrapping_add(0), v_); }
        { let v_ = ((127i32) as u16); c.m.w16(fp.wrapping_add(24).wrapping_add(56).wrapping_add(2), v_); }
        { let v_ = ((32i32) as u16); c.m.w16(fp.wrapping_add(24).wrapping_add(56).wrapping_add(4), v_); }
        { let v_ = ((64i32) as u16); c.m.w16(fp.wrapping_add(24).wrapping_add(56).wrapping_add(6), v_); }
        rt::memset(c, fp.wrapping_add(24).wrapping_add(64), 0, 8);
        { let v_ = ((1i32) as u16); c.m.w16(fp.wrapping_add(24).wrapping_add(64).wrapping_add(0), v_); }
        { let v_ = ((31i32) as u16); c.m.w16(fp.wrapping_add(24).wrapping_add(64).wrapping_add(2), v_); }
        { let v_ = ((64i32) as u16); c.m.w16(fp.wrapping_add(24).wrapping_add(64).wrapping_add(4), v_); }
        { let v_ = ((96i32) as u16); c.m.w16(fp.wrapping_add(24).wrapping_add(64).wrapping_add(6), v_); }
        rt::memset(c, fp.wrapping_add(24).wrapping_add(72), 0, 8);
        { let v_ = ((33i32) as u16); c.m.w16(fp.wrapping_add(24).wrapping_add(72).wrapping_add(0), v_); }
        { let v_ = ((63i32) as u16); c.m.w16(fp.wrapping_add(24).wrapping_add(72).wrapping_add(2), v_); }
        { let v_ = ((64i32) as u16); c.m.w16(fp.wrapping_add(24).wrapping_add(72).wrapping_add(4), v_); }
        { let v_ = ((96i32) as u16); c.m.w16(fp.wrapping_add(24).wrapping_add(72).wrapping_add(6), v_); }
        F::hud_c_b__HudCombo_Zero(c, fp.wrapping_add(104), 8u32);
        l_hits_2 = { let a_ = ((c.m.r32(sym::gHudCombo)).wrapping_add(72)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudCombo)).wrapping_add(12); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.r32(a_) };
        { let v_ = 36i32; let a_ = (fp.wrapping_add(104)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (fp.wrapping_add(104)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        {
            l_i_3 = 0i32;
            'l1: loop {
                if !(((((((l_i_3) as u32)) < (((3i32) as u32))) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_spr_4 = { let a_ = (c.m.r32((p_node_1).wrapping_add(36))).wrapping_add(((((l_i_3).wrapping_add(1i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) };
                    { let a1_ = { let a_ = c.m.r32(sym::gHudCombo); c.m.r32(a_) }; F::HudSprite_InitTex(c, l_spr_4, a1_, 0i32, 0i32) };
                    F::HudSprite_SetRect(c, l_spr_4, 1i32, 30i32, (32i32).wrapping_neg(), 0i32);
                    l_digit_5 = rt::div_u32(rt::rem_u32(l_hits_2, c.m.r32((fp).wrapping_add(((((l_i_3).wrapping_add(1i32)) as u32)).wrapping_mul(4)))), c.m.r32((fp).wrapping_add((((l_i_3) as u32)).wrapping_mul(4))));
                    F::HudSprite_Show(c, l_spr_4, 1i32);
                    {
                        l_step_6 = (22i32).wrapping_neg();
                        { let a1_ = ((l_step_6).wrapping_mul(l_i_3)).wrapping_add((c.m.r32((fp.wrapping_add(104)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)); let a2_ = (c.m.r32((fp.wrapping_add(104)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32); F::HudSprite_Move(c, l_spr_4, a1_, a2_) };
                    }
                    { let a1_ = ((c.m.r16(((fp.wrapping_add(24)).wrapping_add((l_digit_5).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(2)))) as i32); let a2_ = ((c.m.r16(((fp.wrapping_add(24)).wrapping_add((l_digit_5).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(2)))) as i32); let a3_ = ((c.m.r16(((fp.wrapping_add(24)).wrapping_add((l_digit_5).wrapping_mul(8))).wrapping_add((((2i32) as u32)).wrapping_mul(2)))) as i32); let a4_ = ((c.m.r16(((fp.wrapping_add(24)).wrapping_add((l_digit_5).wrapping_mul(8))).wrapping_add((((3i32) as u32)).wrapping_mul(2)))) as i32); F::HudSprite_SetUv(c, l_spr_4, a1_, a2_, a3_, a4_) };
                }
                { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
            }
        }
        l_i_3 = 2i32;
        'l3: loop {
            'c4: {
                l_spr_4 = { let a_ = (c.m.r32((p_node_1).wrapping_add(36))).wrapping_add(((((l_i_3).wrapping_add(1i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) };
                { let o_ = l_i_3; let n_ = (o_).wrapping_sub(1); l_i_3 = n_; };
                if ((((((((((((c.m.r16((l_spr_4).wrapping_add(16)) as i16)) as i32)) != (((c.m.r16(((fp.wrapping_add(24)).wrapping_add((((0i32) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(2)))) as i32))) as i32)) != 0) || ((((((((c.m.r16((l_spr_4).wrapping_add(20)) as i16)) as i32)) != (((c.m.r16(((fp.wrapping_add(24)).wrapping_add((((0i32) as u32)).wrapping_mul(8))).wrapping_add((((2i32) as u32)).wrapping_mul(2)))) as i32))) as i32)) != 0)) as i32)) != 0) {
                    break 'l3;
                }
                F::HudSprite_Show(c, l_spr_4, 0i32);
            }
            if !(((((l_i_3) > (0i32)) as i32)) != 0) { break 'l3; }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "HudCombo_SetHitDigits", &PROBE_N, &[], true); }
    r_
}

pub fn HudCombo_SetDamageDigits(c: &mut Ctx, mut p_node_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudCombo_SetDamageDigits", &PROBE_N, &[(p_node_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(112) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 112);
    let mut l_ramp_2: u32 = 0;
    let mut l_value_3: u32 = 0;
    let mut l_i_4: i32 = 0;
    let mut l_spr_5: u32 = 0;
    let mut l_digit_6: u32 = 0;
    let mut l_step_7: i32 = 0;
    let r_ = 'f: {
        l_ramp_2 = ((c.m.r32(sym::gHudCombo)).wrapping_add(296)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudCombo)).wrapping_add(12); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(24));
        rt::memset(c, fp, 0, 24);
        { let v_ = ((1i32) as u32); c.m.w32(fp.wrapping_add(0), v_); }
        { let v_ = ((10i32) as u32); c.m.w32(fp.wrapping_add(4), v_); }
        { let v_ = ((100i32) as u32); c.m.w32(fp.wrapping_add(8), v_); }
        { let v_ = ((1000i32) as u32); c.m.w32(fp.wrapping_add(12), v_); }
        { let v_ = ((10000i32) as u32); c.m.w32(fp.wrapping_add(16), v_); }
        { let v_ = ((100000i32) as u32); c.m.w32(fp.wrapping_add(20), v_); }
        rt::memset(c, fp.wrapping_add(24), 0, 80);
        rt::memset(c, fp.wrapping_add(24).wrapping_add(0), 0, 8);
        { let v_ = ((0i32) as u16); c.m.w16(fp.wrapping_add(24).wrapping_add(0).wrapping_add(0), v_); }
        { let v_ = ((32i32) as u16); c.m.w16(fp.wrapping_add(24).wrapping_add(0).wrapping_add(2), v_); }
        { let v_ = ((0i32) as u16); c.m.w16(fp.wrapping_add(24).wrapping_add(0).wrapping_add(4), v_); }
        { let v_ = ((32i32) as u16); c.m.w16(fp.wrapping_add(24).wrapping_add(0).wrapping_add(6), v_); }
        rt::memset(c, fp.wrapping_add(24).wrapping_add(8), 0, 8);
        { let v_ = ((32i32) as u16); c.m.w16(fp.wrapping_add(24).wrapping_add(8).wrapping_add(0), v_); }
        { let v_ = ((64i32) as u16); c.m.w16(fp.wrapping_add(24).wrapping_add(8).wrapping_add(2), v_); }
        { let v_ = ((0i32) as u16); c.m.w16(fp.wrapping_add(24).wrapping_add(8).wrapping_add(4), v_); }
        { let v_ = ((32i32) as u16); c.m.w16(fp.wrapping_add(24).wrapping_add(8).wrapping_add(6), v_); }
        rt::memset(c, fp.wrapping_add(24).wrapping_add(16), 0, 8);
        { let v_ = ((64i32) as u16); c.m.w16(fp.wrapping_add(24).wrapping_add(16).wrapping_add(0), v_); }
        { let v_ = ((96i32) as u16); c.m.w16(fp.wrapping_add(24).wrapping_add(16).wrapping_add(2), v_); }
        { let v_ = ((0i32) as u16); c.m.w16(fp.wrapping_add(24).wrapping_add(16).wrapping_add(4), v_); }
        { let v_ = ((32i32) as u16); c.m.w16(fp.wrapping_add(24).wrapping_add(16).wrapping_add(6), v_); }
        rt::memset(c, fp.wrapping_add(24).wrapping_add(24), 0, 8);
        { let v_ = ((96i32) as u16); c.m.w16(fp.wrapping_add(24).wrapping_add(24).wrapping_add(0), v_); }
        { let v_ = ((128i32) as u16); c.m.w16(fp.wrapping_add(24).wrapping_add(24).wrapping_add(2), v_); }
        { let v_ = ((0i32) as u16); c.m.w16(fp.wrapping_add(24).wrapping_add(24).wrapping_add(4), v_); }
        { let v_ = ((32i32) as u16); c.m.w16(fp.wrapping_add(24).wrapping_add(24).wrapping_add(6), v_); }
        rt::memset(c, fp.wrapping_add(24).wrapping_add(32), 0, 8);
        { let v_ = ((0i32) as u16); c.m.w16(fp.wrapping_add(24).wrapping_add(32).wrapping_add(0), v_); }
        { let v_ = ((32i32) as u16); c.m.w16(fp.wrapping_add(24).wrapping_add(32).wrapping_add(2), v_); }
        { let v_ = ((32i32) as u16); c.m.w16(fp.wrapping_add(24).wrapping_add(32).wrapping_add(4), v_); }
        { let v_ = ((64i32) as u16); c.m.w16(fp.wrapping_add(24).wrapping_add(32).wrapping_add(6), v_); }
        rt::memset(c, fp.wrapping_add(24).wrapping_add(40), 0, 8);
        { let v_ = ((32i32) as u16); c.m.w16(fp.wrapping_add(24).wrapping_add(40).wrapping_add(0), v_); }
        { let v_ = ((64i32) as u16); c.m.w16(fp.wrapping_add(24).wrapping_add(40).wrapping_add(2), v_); }
        { let v_ = ((32i32) as u16); c.m.w16(fp.wrapping_add(24).wrapping_add(40).wrapping_add(4), v_); }
        { let v_ = ((64i32) as u16); c.m.w16(fp.wrapping_add(24).wrapping_add(40).wrapping_add(6), v_); }
        rt::memset(c, fp.wrapping_add(24).wrapping_add(48), 0, 8);
        { let v_ = ((64i32) as u16); c.m.w16(fp.wrapping_add(24).wrapping_add(48).wrapping_add(0), v_); }
        { let v_ = ((96i32) as u16); c.m.w16(fp.wrapping_add(24).wrapping_add(48).wrapping_add(2), v_); }
        { let v_ = ((32i32) as u16); c.m.w16(fp.wrapping_add(24).wrapping_add(48).wrapping_add(4), v_); }
        { let v_ = ((64i32) as u16); c.m.w16(fp.wrapping_add(24).wrapping_add(48).wrapping_add(6), v_); }
        rt::memset(c, fp.wrapping_add(24).wrapping_add(56), 0, 8);
        { let v_ = ((96i32) as u16); c.m.w16(fp.wrapping_add(24).wrapping_add(56).wrapping_add(0), v_); }
        { let v_ = ((128i32) as u16); c.m.w16(fp.wrapping_add(24).wrapping_add(56).wrapping_add(2), v_); }
        { let v_ = ((32i32) as u16); c.m.w16(fp.wrapping_add(24).wrapping_add(56).wrapping_add(4), v_); }
        { let v_ = ((64i32) as u16); c.m.w16(fp.wrapping_add(24).wrapping_add(56).wrapping_add(6), v_); }
        rt::memset(c, fp.wrapping_add(24).wrapping_add(64), 0, 8);
        { let v_ = ((0i32) as u16); c.m.w16(fp.wrapping_add(24).wrapping_add(64).wrapping_add(0), v_); }
        { let v_ = ((32i32) as u16); c.m.w16(fp.wrapping_add(24).wrapping_add(64).wrapping_add(2), v_); }
        { let v_ = ((64i32) as u16); c.m.w16(fp.wrapping_add(24).wrapping_add(64).wrapping_add(4), v_); }
        { let v_ = ((96i32) as u16); c.m.w16(fp.wrapping_add(24).wrapping_add(64).wrapping_add(6), v_); }
        rt::memset(c, fp.wrapping_add(24).wrapping_add(72), 0, 8);
        { let v_ = ((32i32) as u16); c.m.w16(fp.wrapping_add(24).wrapping_add(72).wrapping_add(0), v_); }
        { let v_ = ((64i32) as u16); c.m.w16(fp.wrapping_add(24).wrapping_add(72).wrapping_add(2), v_); }
        { let v_ = ((64i32) as u16); c.m.w16(fp.wrapping_add(24).wrapping_add(72).wrapping_add(4), v_); }
        { let v_ = ((96i32) as u16); c.m.w16(fp.wrapping_add(24).wrapping_add(72).wrapping_add(6), v_); }
        F::hud_c_b__HudCombo_Zero(c, fp.wrapping_add(104), 8u32);
        if ((((!(((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0)) as i32)) != 0) {
            { let _ = F::Ramp_Step(c, l_ramp_2); };
        }
        if ((((({ let a_ = ((c.m.r32(sym::gHudCombo)).wrapping_add(56)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudCombo)).wrapping_add(12); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) < (2i32)) as i32)) != 0) {
            l_value_3 = (({ let a_ = ((c.m.r32(sym::gHudCombo)).wrapping_add(56)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudCombo)).wrapping_add(12); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32);
        } else {
            l_value_3 = (({ let a_ = ((c.m.r32(sym::gHudCombo)).wrapping_add(64)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudCombo)).wrapping_add(12); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32);
            l_value_3 = rt::f2u(rt::fadd(rt::u2f(l_value_3), rt::fmul(rt::i2f(({ let a_ = ((c.m.r32(sym::gHudCombo)).wrapping_add(56)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudCombo)).wrapping_add(12); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_sub({ let a_ = ((c.m.r32(sym::gHudCombo)).wrapping_add(64)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudCombo)).wrapping_add(12); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) })), F32(c.m.r32((l_ramp_2).wrapping_add(16))))));
        }
        if (((((l_value_3) > (((99999i32) as u32))) as i32)) != 0) {
            l_value_3 = ((99999i32) as u32);
        }
        { let v_ = 96i32; let a_ = (fp.wrapping_add(104)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (fp.wrapping_add(104)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        {
            l_i_4 = 0i32;
            'l1: loop {
                if !(((((((l_i_4) as u32)) < (((5i32) as u32))) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_spr_5 = { let a_ = (c.m.r32((p_node_1).wrapping_add(36))).wrapping_add(((((l_i_4).wrapping_add(1i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) };
                    { let a1_ = { let a_ = c.m.r32(sym::gHudCombo); c.m.r32(a_) }; F::HudSprite_InitTex(c, l_spr_5, a1_, 1i32, 0i32) };
                    F::HudSprite_SetRect(c, l_spr_5, 1i32, 31i32, 1i32, 31i32);
                    l_digit_6 = rt::div_u32(rt::rem_u32(l_value_3, c.m.r32((fp).wrapping_add(((((l_i_4).wrapping_add(1i32)) as u32)).wrapping_mul(4)))), c.m.r32((fp).wrapping_add((((l_i_4) as u32)).wrapping_mul(4))));
                    if ((((((((((((((((!(((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0)) as i32)) != 0) && (((((l_i_4) == (0i32)) as i32)) != 0)) as i32)) != 0) && ((((((l_value_3).wrapping_sub(((100i32) as u32))) <= (((99898i32) as u32))) as i32)) != 0)) as i32)) != 0) && (((rt::fne(F32(c.m.r32((l_ramp_2).wrapping_add(16))), F32(c.m.r32((l_ramp_2).wrapping_add(20)))) as i32)) != 0)) as i32)) != 0) {
                        'l3: loop {
                            'c4: {
                                l_digit_6 = ((F::Rand_IntRange(c, 0i32, 9i32)) as u32);
                            }
                            if !(((((l_digit_6) == ((({ let a_ = ((c.m.r32(sym::gHudCombo)).wrapping_add(368)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudCombo)).wrapping_add(12); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(2)); c.m.r16(a_) }) as u32))) as i32)) != 0) { break 'l3; }
                        }
                        { let v_ = ((l_digit_6) as u16); let a_ = ((c.m.r32(sym::gHudCombo)).wrapping_add(368)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudCombo)).wrapping_add(12); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(2)); c.m.w16(a_, v_); };
                    }
                    F::HudSprite_Show(c, l_spr_5, 1i32);
                    {
                        l_step_7 = (12i32).wrapping_neg();
                        { let a1_ = ((l_step_7).wrapping_mul(l_i_4)).wrapping_add((c.m.r32((fp.wrapping_add(104)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)); let a2_ = (c.m.r32((fp.wrapping_add(104)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32); F::HudSprite_Move(c, l_spr_5, a1_, a2_) };
                    }
                    { let a1_ = (((c.m.r16(((fp.wrapping_add(24)).wrapping_add((l_digit_6).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(2)))) as i32)).wrapping_add(1i32); let a2_ = (((c.m.r16(((fp.wrapping_add(24)).wrapping_add((l_digit_6).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(2)))) as i32)).wrapping_sub(1i32); let a3_ = (((c.m.r16(((fp.wrapping_add(24)).wrapping_add((l_digit_6).wrapping_mul(8))).wrapping_add((((2i32) as u32)).wrapping_mul(2)))) as i32)).wrapping_add(1i32); let a4_ = (((c.m.r16(((fp.wrapping_add(24)).wrapping_add((l_digit_6).wrapping_mul(8))).wrapping_add((((3i32) as u32)).wrapping_mul(2)))) as i32)).wrapping_sub(1i32); F::HudSprite_SetUv(c, l_spr_5, a1_, a2_, a3_, a4_) };
                }
                { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
            }
        }
        l_i_4 = 4i32;
        'l5: loop {
            'c6: {
                l_spr_5 = { let a_ = (c.m.r32((p_node_1).wrapping_add(36))).wrapping_add(((((l_i_4).wrapping_add(1i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) };
                { let o_ = l_i_4; let n_ = (o_).wrapping_sub(1); l_i_4 = n_; };
                if ((((((((((((c.m.r16((l_spr_5).wrapping_add(16)) as i16)) as i32)) != (1i32)) as i32)) != 0) || ((((((((c.m.r16((l_spr_5).wrapping_add(20)) as i16)) as i32)) != (1i32)) as i32)) != 0)) as i32)) != 0) {
                    break 'l5;
                }
                F::HudSprite_Show(c, l_spr_5, 0i32);
            }
            if !(((((l_i_4) > (0i32)) as i32)) != 0) { break 'l5; }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "HudCombo_SetDamageDigits", &PROBE_N, &[], true); }
    r_
}

pub fn HudCombo_UpdateDamage(c: &mut Ctx, mut p_node_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudCombo_UpdateDamage", &PROBE_N, &[(p_node_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_ramp_2: u32 = 0;
    let mut l_spr_3: u32 = 0;
    let mut l_width_4: F32 = F32(0);
    let mut l_done_5: i32 = 0;
    let r_ = 'f: {
        l_ramp_2 = ((c.m.r32(sym::gHudCombo)).wrapping_add(200)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudCombo)).wrapping_add(12); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(24));
        F::hud_c_b__HudCombo_Zero(c, fp, 8u32);
        l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudCombo)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((2i32) as u32)).wrapping_mul(28));
        l_width_4 = rt::fneg(rt::i2f(((((c.m.r16((l_spr_3).wrapping_add(18)) as i16)) as i32)).wrapping_sub((((c.m.r16((l_spr_3).wrapping_add(16)) as i16)) as i32))));
        F::HudCombo_SetDamageDigits(c, p_node_1);
        F::HudNode_Show(c, p_node_1, 1i32);
        's1: {
            let sv6: i32 = { let a_ = ((c.m.r32(sym::gHudCombo)).wrapping_add(24)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudCombo)).wrapping_add(12); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
            'k9: {
            'k8: {
            'k7: {
            'k6: {
            'k5: {
            'k4: {
            'k3: {
            'k2: {
                match sv6 { 0i32 => break 'k2, 1i32 => break 'k3, 4i32 => break 'k4, 5i32 => break 'k5, 7i32 => break 'k6, 8i32 => break 'k7, 10i32 => break 'k8, 2i32 => break 'k9, 3i32 => break 'k9, 6i32 => break 'k9, 9i32 => break 'k9, _ => break 's1, }
            }
            F::HudNode_Show(c, p_node_1, 1i32);
            F::Ramp_Start(c, l_ramp_2, F32(0x3dcccccc), F32(0x3f800000), F32(0x00000000));
            { let a_ = ((c.m.r32(sym::gHudCombo)).wrapping_add(24)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudCombo)).wrapping_add(12); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
            }
            if ((((!(((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0)) as i32)) != 0) {
                l_done_5 = F::Ramp_Step(c, l_ramp_2);
            } else {
                l_done_5 = 0i32;
            }
            if ((l_done_5) != 0) {
                { let v_ = 3i32; let a_ = ((c.m.r32(sym::gHudCombo)).wrapping_add(24)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudCombo)).wrapping_add(12); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = F32(0x00000000); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            } else {
                { let v_ = rt::fmul(l_width_4, F32(c.m.r32((l_ramp_2).wrapping_add(16)))); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            }
            break 's1;
            }
            F::Ramp_Start(c, l_ramp_2, F32(0x3dcccccc), F32(0x00000000), F32(0x3f800000));
            { let a_ = ((c.m.r32(sym::gHudCombo)).wrapping_add(24)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudCombo)).wrapping_add(12); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
            }
            if ((((!(((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0)) as i32)) != 0) {
                l_done_5 = F::Ramp_Step(c, l_ramp_2);
            } else {
                l_done_5 = 0i32;
            }
            if ((l_done_5) != 0) {
                if (((rt::feq(F32(c.m.r32((l_ramp_2).wrapping_add(16))), F32(0x3f800000)) as i32)) != 0) {
                    F::Ramp_Start(c, l_ramp_2, F32(0x3dcccccc), F32(0x3f800000), F32(0x00000000));
                } else {
                    if (((rt::feq(F32(c.m.r32((l_ramp_2).wrapping_add(16))), F32(0x00000000)) as i32)) != 0) {
                        { let v_ = 3i32; let a_ = ((c.m.r32(sym::gHudCombo)).wrapping_add(24)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudCombo)).wrapping_add(12); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    }
                }
            }
            { let r_ = rt::fmul(F32(c.m.r32((l_ramp_2).wrapping_add(16))), F32(0x41200000)); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            break 's1;
            }
            F::Ramp_Start(c, l_ramp_2, F32(0x3dcccccc), F32(0x00000000), F32(0x3f800000));
            { let a_ = ((c.m.r32(sym::gHudCombo)).wrapping_add(24)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudCombo)).wrapping_add(12); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
            }
            if ((((!(((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0)) as i32)) != 0) {
                l_done_5 = F::Ramp_Step(c, l_ramp_2);
            } else {
                l_done_5 = 0i32;
            }
            if ((l_done_5) != 0) {
                F::HudNode_Show(c, p_node_1, 0i32);
                { let v_ = 10i32; let a_ = ((c.m.r32(sym::gHudCombo)).wrapping_add(24)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudCombo)).wrapping_add(12); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            } else {
                { let v_ = rt::fmul(l_width_4, F32(c.m.r32((l_ramp_2).wrapping_add(16)))); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            }
            break 's1;
            }
            F::HudNode_Show(c, p_node_1, 0i32);
            { let v_ = 0i32; let a_ = ((c.m.r32(sym::gHudCombo)).wrapping_add(64)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudCombo)).wrapping_add(12); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            { let v_ = 0i32; let a_ = ((c.m.r32(sym::gHudCombo)).wrapping_add(56)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudCombo)).wrapping_add(12); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            break 'f;
            }
            break 's1;
        }
        { let r_ = F32(0x40a00000); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        if ((((({ let a_ = (c.m.r32(sym::gHudCombo)).wrapping_add(12); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
            { let a1_ = rt::f2i(F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a2_ = rt::f2i(F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); F::HudNode_SetOfs(c, p_node_1, a1_, a2_) };
        } else {
            { let a1_ = (372i32).wrapping_sub(rt::f2i(F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)))))); let a2_ = rt::f2i(F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); F::HudNode_SetOfs(c, p_node_1, a1_, a2_) };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "HudCombo_UpdateDamage", &PROBE_N, &[], true); }
    r_
}

pub fn HudCombo_UpdateHits(c: &mut Ctx, mut p_node_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudCombo_UpdateHits", &PROBE_N, &[(p_node_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_scale_2: F32 = F32(0);
    let mut l_ramp_3: u32 = 0;
    let mut l_spr_4: u32 = 0;
    let mut l_width_5: F32 = F32(0);
    let mut l_i_6: i32 = 0;
    let mut l_done_7: i32 = 0;
    let r_ = 'f: {
        l_scale_2 = F32(0x3f800000);
        l_ramp_3 = ((c.m.r32(sym::gHudCombo)).wrapping_add(248)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudCombo)).wrapping_add(12); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(24));
        F::hud_c_b__HudCombo_Zero(c, fp, 8u32);
        l_spr_4 = ({ let a_ = (c.m.r32(sym::gHudCombo)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((8i32) as u32)).wrapping_mul(28));
        l_width_5 = rt::fneg(rt::i2f(((((c.m.r16((l_spr_4).wrapping_add(18)) as i16)) as i32)).wrapping_sub((((c.m.r16((l_spr_4).wrapping_add(16)) as i16)) as i32))));
        F::HudCombo_SetHitDigits(c, p_node_1);
        F::HudNode_Show(c, p_node_1, 1i32);
        's1: {
            let sv8: i32 = { let a_ = ((c.m.r32(sym::gHudCombo)).wrapping_add(32)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudCombo)).wrapping_add(12); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
            'k9: {
            'k8: {
            'k7: {
            'k6: {
            'k5: {
            'k4: {
            'k3: {
            'k2: {
                match sv8 { 0i32 => break 'k2, 1i32 => break 'k3, 4i32 => break 'k4, 5i32 => break 'k5, 7i32 => break 'k6, 8i32 => break 'k7, 10i32 => break 'k8, 2i32 => break 'k9, 3i32 => break 'k9, 6i32 => break 'k9, 9i32 => break 'k9, _ => break 's1, }
            }
            F::HudNode_Show(c, p_node_1, 1i32);
            F::Ramp_Start(c, l_ramp_3, F32(0x3dcccccc), F32(0x3f800000), F32(0x00000000));
            { let a_ = ((c.m.r32(sym::gHudCombo)).wrapping_add(32)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudCombo)).wrapping_add(12); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
            }
            if ((((!(((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0)) as i32)) != 0) {
                l_done_7 = F::Ramp_Step(c, l_ramp_3);
            } else {
                l_done_7 = 0i32;
            }
            if ((l_done_7) != 0) {
                F::Ramp_Start(c, l_ramp_3, F32(0x3f999999), F32(0x00000000), F32(0x3f800000));
                { let v_ = 3i32; let a_ = ((c.m.r32(sym::gHudCombo)).wrapping_add(32)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudCombo)).wrapping_add(12); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = F32(0x00000000); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            } else {
                { let v_ = rt::fmul(l_width_5, F32(c.m.r32((l_ramp_3).wrapping_add(16)))); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            }
            break 's1;
            }
            F::Ramp_Start(c, l_ramp_3, F32(0x3dcccccc), F32(0x00000000), F32(0x3f800000));
            { let a_ = ((c.m.r32(sym::gHudCombo)).wrapping_add(32)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudCombo)).wrapping_add(12); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
            }
            if ((((!(((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0)) as i32)) != 0) {
                l_done_7 = F::Ramp_Step(c, l_ramp_3);
            } else {
                l_done_7 = 0i32;
            }
            if ((l_done_7) != 0) {
                if (((rt::feq(F32(c.m.r32((l_ramp_3).wrapping_add(16))), F32(0x3f800000)) as i32)) != 0) {
                    F::Ramp_Start(c, l_ramp_3, F32(0x3dcccccc), F32(0x3f800000), F32(0x00000000));
                } else {
                    if (((rt::feq(F32(c.m.r32((l_ramp_3).wrapping_add(16))), F32(0x00000000)) as i32)) != 0) {
                        { let v_ = 3i32; let a_ = ((c.m.r32(sym::gHudCombo)).wrapping_add(32)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudCombo)).wrapping_add(12); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    }
                }
            }
            l_scale_2 = rt::fadd(rt::fmul(F32(c.m.r32((l_ramp_3).wrapping_add(16))), F32(0x3f000000)), F32(0x3f800000));
            break 's1;
            }
            F::Ramp_Start(c, l_ramp_3, F32(0x3dcccccc), F32(0x00000000), F32(0x3f800000));
            { let a_ = ((c.m.r32(sym::gHudCombo)).wrapping_add(32)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudCombo)).wrapping_add(12); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
            }
            if ((((!(((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0)) as i32)) != 0) {
                l_done_7 = F::Ramp_Step(c, l_ramp_3);
            } else {
                l_done_7 = 0i32;
            }
            if ((l_done_7) != 0) {
                F::HudNode_Show(c, p_node_1, 0i32);
                { let v_ = 10i32; let a_ = ((c.m.r32(sym::gHudCombo)).wrapping_add(32)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudCombo)).wrapping_add(12); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            } else {
                { let v_ = rt::fmul(l_width_5, F32(c.m.r32((l_ramp_3).wrapping_add(16)))); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            }
            break 's1;
            }
            F::HudNode_Show(c, p_node_1, 0i32);
            break 'f;
            }
            break 's1;
        }
        { let r_ = F32(0x40a00000); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        if ((((({ let a_ = (c.m.r32(sym::gHudCombo)).wrapping_add(12); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
            { let a1_ = rt::f2i(F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a2_ = rt::f2i(F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); F::HudNode_SetOfs(c, p_node_1, a1_, a2_) };
        } else {
            { let a1_ = (372i32).wrapping_sub(rt::f2i(F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)))))); let a2_ = rt::f2i(F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); F::HudNode_SetOfs(c, p_node_1, a1_, a2_) };
        }
        {
            l_i_6 = 0i32;
            'l10: loop {
                if !(((((l_i_6) < (3i32)) as i32)) != 0) { break 'l10; }
                'c11: {
                    { let a0_ = { let a_ = (c.m.r32((p_node_1).wrapping_add(36))).wrapping_add(((((l_i_6).wrapping_add(1i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) }; F::HudSprite_Scale(c, a0_, F32(0x3f800000), l_scale_2) };
                }
                { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "HudCombo_UpdateHits", &PROBE_N, &[], true); }
    r_
}

pub fn HudCombo_DrawSprites(c: &mut Ctx, mut p_node_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudCombo_DrawSprites", &PROBE_N, &[(p_node_1) as u32], false); }
    let mut l_i_2: u32 = 0;
    let r_ = 'f: {
    {
        l_i_2 = ((0i32) as u32);
        'l1: loop {
            if !(((((l_i_2) < (c.m.r32((p_node_1).wrapping_add(32)))) as i32)) != 0) { break 'l1; }
            'c2: {
                { let a0_ = { let a_ = (c.m.r32((p_node_1).wrapping_add(36))).wrapping_add((l_i_2).wrapping_mul(4)); c.m.r32(a_) }; let a1_ = { let a_ = c.m.r32(sym::gHudCombo); c.m.r32(a_) }; F::HudSprite_Draw(c, a0_, a1_, 0i32) };
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudCombo_DrawSprites", &PROBE_N, &[], true); }
    r_
}

pub fn HudCombo_StoreTextPos(c: &mut Ctx, mut p_node_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudCombo_StoreTextPos", &PROBE_N, &[(p_node_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(64) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 64);
    let mut l_spr_2: u32 = 0;
    let mut l_alpha_3: F32 = F32(0);
    let r_ = 'f: {
        l_spr_2 = { let a_ = (c.m.r32((p_node_1).wrapping_add(36))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) };
        l_alpha_3 = rt::fdiv(rt::i2f(((c.m.r8((l_spr_2).wrapping_add(27))) as i32)), F32(0x43000000));
        if ((((c.m.r32(l_spr_2)) & (((1i32) as u32)))) != 0) {
            { let v_ = (1i32).wrapping_neg(); let a_ = (((c.m.r32(sym::gHudCombo)).wrapping_add(372)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudCombo)).wrapping_add(12); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            { let v_ = (1i32).wrapping_neg(); let a_ = (((c.m.r32(sym::gHudCombo)).wrapping_add(372)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudCombo)).wrapping_add(12); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        } else {
            F::Vu0Cur_StoreMtx(c, fp);
            { let v_ = l_alpha_3; let a_ = ((c.m.r32(sym::gHudCombo)).wrapping_add(388)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudCombo)).wrapping_add(12); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            { let v_ = rt::f2i(F32(c.m.r32(((fp).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a_ = (((c.m.r32(sym::gHudCombo)).wrapping_add(372)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudCombo)).wrapping_add(12); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            { let v_ = rt::f2i(F32(c.m.r32(((fp).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); let a_ = (((c.m.r32(sym::gHudCombo)).wrapping_add(372)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudCombo)).wrapping_add(12); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "HudCombo_StoreTextPos", &PROBE_N, &[], true); }
    r_
}

pub fn HudCombo_SelectSide(c: &mut Ctx, mut p_side_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudCombo_SelectSide", &PROBE_N, &[(p_side_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_side_1; let a_ = (c.m.r32(sym::gHudCombo)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
    F::Port_GsMarker(c, (if ((p_side_1) != 0) { 17i32 } else { 16i32 }));
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudCombo_SelectSide", &PROBE_N, &[], true); }
    r_
}

pub fn HudCombo_SetMessage(c: &mut Ctx, mut p_side_1: i32, mut p_message_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudCombo_SetMessage", &PROBE_N, &[(p_side_1) as u32, (p_message_2) as u32], false); }
    let r_ = 'f: {
    { let v_ = 0i32; let a_ = ((c.m.r32(sym::gHudCombo)).wrapping_add(16)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    { let v_ = p_message_2; let a_ = ((c.m.r32(sym::gHudCombo)).wrapping_add(48)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudCombo_SetMessage", &PROBE_N, &[], true); }
    r_
}

pub fn HudCombo_SetText(c: &mut Ctx, mut p_side_1: i32, mut p_text_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudCombo_SetText", &PROBE_N, &[(p_side_1) as u32, (p_text_2) as u32], false); }
    let r_ = 'f: {
    { let v_ = 0i32; let a_ = ((c.m.r32(sym::gHudCombo)).wrapping_add(40)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    { let v_ = p_text_2; let a_ = ((c.m.r32(sym::gHudCombo)).wrapping_add(96)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudCombo_SetText", &PROBE_N, &[], true); }
    r_
}

pub fn HudCombo_SetDamage(c: &mut Ctx, mut p_side_1: i32, mut p_damage_2: i32, mut p_isNew_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudCombo_SetDamage", &PROBE_N, &[(p_side_1) as u32, (p_damage_2) as u32, (p_isNew_3) as u32], false); }
    let mut l_w_4: u32 = 0;
    let mut l_seconds_5: F32 = F32(0);
    let r_ = 'f: {
    if (((((p_damage_2) > (199998i32)) as i32)) != 0) {
        p_damage_2 = 199998i32;
    }
    l_w_4 = c.m.r32(sym::gHudCombo);
    if (((((p_damage_2) < (0i32)) as i32)) != 0) {
        if (((((((((c.m.r32(((l_w_4).wrapping_add(24)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4))) as i32)).wrapping_sub(7i32)) as u32)) < (((4i32) as u32))) as i32)) != 0) {
            break 'f;
        }
        { let v_ = 7i32; let a_ = ((l_w_4).wrapping_add(24)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    } else {
        l_seconds_5 = F32(0x3f199999);
        if ((((((p_isNew_3) != 0) && ((((({ let a_ = ((c.m.r32(sym::gHudCombo)).wrapping_add(80)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) <= ({ let a_ = ((c.m.r32(sym::gHudCombo)).wrapping_add(88)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) })) as i32)) != 0)) as i32)) != 0) {
            { let v_ = 0i32; let a_ = ((c.m.r32(sym::gHudCombo)).wrapping_add(64)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        } else {
            if (((rt::fne({ let a_ = (((c.m.r32(sym::gHudCombo)).wrapping_add(296)).wrapping_add((((p_side_1) as u32)).wrapping_mul(24))).wrapping_add(16); F32(c.m.r32(a_)) }, { let a_ = (((c.m.r32(sym::gHudCombo)).wrapping_add(296)).wrapping_add((((p_side_1) as u32)).wrapping_mul(24))).wrapping_add(20); F32(c.m.r32(a_)) }) as i32)) != 0) {
                { let r_ = rt::f2i(rt::fmul({ let a_ = (((c.m.r32(sym::gHudCombo)).wrapping_add(296)).wrapping_add((((p_side_1) as u32)).wrapping_mul(24))).wrapping_add(16); F32(c.m.r32(a_)) }, rt::i2f(({ let a_ = ((c.m.r32(sym::gHudCombo)).wrapping_add(56)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_sub({ let a_ = ((c.m.r32(sym::gHudCombo)).wrapping_add(64)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) })))); let a_ = ((c.m.r32(sym::gHudCombo)).wrapping_add(64)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
            } else {
                { let v_ = { let a_ = ((c.m.r32(sym::gHudCombo)).wrapping_add(56)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ((c.m.r32(sym::gHudCombo)).wrapping_add(64)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            }
        }
        { let v_ = p_damage_2; let a_ = ((c.m.r32(sym::gHudCombo)).wrapping_add(56)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        if (((((({ let a_ = ((c.m.r32(sym::gHudCombo)).wrapping_add(56)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_sub({ let a_ = ((c.m.r32(sym::gHudCombo)).wrapping_add(64)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) })) > (6000i32)) as i32)) != 0) {
            l_seconds_5 = rt::fdiv(rt::i2f(({ let a_ = ((c.m.r32(sym::gHudCombo)).wrapping_add(56)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_sub({ let a_ = ((c.m.r32(sym::gHudCombo)).wrapping_add(64)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) })), F32(0x46147000));
        }
        if (((rt::fgt(l_seconds_5, F32(0x3fc00000)) as i32)) != 0) {
            l_seconds_5 = F32(0x3fc00000);
        }
        { let a0_ = ((c.m.r32(sym::gHudCombo)).wrapping_add(296)).wrapping_add((((p_side_1) as u32)).wrapping_mul(24)); F::Ramp_Start(c, a0_, l_seconds_5, F32(0x00000000), F32(0x3f800000)) };
        if ((((((({ let a_ = ((c.m.r32(sym::gHudCombo)).wrapping_add(24)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)) < (((6i32) as u32))) as i32)) != 0) {
            { let v_ = 4i32; let a_ = ((c.m.r32(sym::gHudCombo)).wrapping_add(24)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        } else {
            { let v_ = 0i32; let a_ = ((c.m.r32(sym::gHudCombo)).wrapping_add(24)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        }
    }
    { let a0_ = ({ let a_ = (c.m.r32(sym::gHudCombo)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((2i32) as u32)).wrapping_mul(28)); F::HudSprite_Show(c, a0_, 1i32) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudCombo_SetDamage", &PROBE_N, &[], true); }
    r_
}

pub fn HudCombo_SetHits(c: &mut Ctx, mut p_side_1: i32, mut p_hits_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudCombo_SetHits", &PROBE_N, &[(p_side_1) as u32, (p_hits_2) as u32], false); }
    let r_ = 'f: {
    { let v_ = ((p_hits_2) as i32); let a_ = ((c.m.r32(sym::gHudCombo)).wrapping_add(80)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    if (((((p_hits_2) > (((999i32) as u32))) as i32)) != 0) {
        p_hits_2 = ((999i32) as u32);
    }
    if (((((p_hits_2) < (((2i32) as u32))) as i32)) != 0) {
        if (((((((({ let a_ = ((c.m.r32(sym::gHudCombo)).wrapping_add(32)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_sub(7i32)) as u32)) < (((4i32) as u32))) as i32)) != 0) {
            break 'f;
        }
        { let v_ = 7i32; let a_ = ((c.m.r32(sym::gHudCombo)).wrapping_add(32)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    } else {
        if (((((p_hits_2) < ({ let a_ = ((c.m.r32(sym::gHudCombo)).wrapping_add(72)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.r32(a_) })) as i32)) != 0) {
            { let v_ = 0i32; let a_ = ((c.m.r32(sym::gHudCombo)).wrapping_add(32)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        } else {
            if ((((((({ let a_ = ((c.m.r32(sym::gHudCombo)).wrapping_add(32)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)) < (((6i32) as u32))) as i32)) != 0) {
                { let v_ = 4i32; let a_ = ((c.m.r32(sym::gHudCombo)).wrapping_add(32)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            } else {
                { let v_ = 0i32; let a_ = ((c.m.r32(sym::gHudCombo)).wrapping_add(32)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            }
        }
        { let v_ = (({ let a_ = ((c.m.r32(sym::gHudCombo)).wrapping_add(72)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }) as i32); let a_ = ((c.m.r32(sym::gHudCombo)).wrapping_add(88)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = p_hits_2; let a_ = ((c.m.r32(sym::gHudCombo)).wrapping_add(72)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    }
    { let a0_ = ({ let a_ = (c.m.r32(sym::gHudCombo)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((8i32) as u32)).wrapping_mul(28)); F::HudSprite_Show(c, a0_, 1i32) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudCombo_SetHits", &PROBE_N, &[], true); }
    r_
}

pub fn HudCombo_SlideOut(c: &mut Ctx, mut p_seconds_1: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudCombo_SlideOut", &PROBE_N, &[(p_seconds_1).0], false); }
    let r_ = 'f: {
    { let a0_ = (c.m.r32(sym::gHudCombo)).wrapping_add(344); F::Ramp_Start(c, a0_, p_seconds_1, F32(0x00000000), F32(0x3f800000)) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudCombo_SlideOut", &PROBE_N, &[], true); }
    r_
}

pub fn HudCombo_SlideIn(c: &mut Ctx, mut p_seconds_1: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudCombo_SlideIn", &PROBE_N, &[(p_seconds_1).0], false); }
    let r_ = 'f: {
    { let a0_ = (c.m.r32(sym::gHudCombo)).wrapping_add(344); F::Ramp_Start(c, a0_, p_seconds_1, F32(0x3f800000), F32(0x00000000)) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudCombo_SlideIn", &PROBE_N, &[], true); }
    r_
}

pub fn HudCombo_DrawText(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudCombo_DrawText", &PROBE_N, &[], false); }
    let mut l_side_1: i32 = 0;
    let mut l_pos_2: u32 = 0;
    let mut l_alpha_3: F32 = F32(0);
    let r_ = 'f: {
    {
        l_side_1 = 0i32;
        'l1: loop {
            if !(((((l_side_1) < (2i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                l_pos_2 = ((c.m.r32(sym::gHudCombo)).wrapping_add(372)).wrapping_add((((l_side_1) as u32)).wrapping_mul(8));
                l_alpha_3 = { let a_ = ((c.m.r32(sym::gHudCombo)).wrapping_add(388)).wrapping_add((((l_side_1) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) };
                if ((((((((((c.m.r32((l_pos_2).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) >= (0i32)) as i32)) != 0) && ((((((c.m.r32((l_pos_2).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) >= (0i32)) as i32)) != 0)) as i32)) != 0) {
                    { let a0_ = { let a_ = (F::BtlCtrl_GetObj(c, l_side_1)).wrapping_add(188); c.m.r32(a_) }; F::BtlMenu_SetScript2(c, a0_) };
                    { let a0_ = (c.m.r32((l_pos_2).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32); let a1_ = (c.m.r32((l_pos_2).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32); let a2_ = { let a_ = ((c.m.r32(sym::gHudCombo)).wrapping_add(96)).wrapping_add((((l_side_1) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; F::BtlText_DrawEntryName(c, a0_, a1_, a2_, l_side_1, l_alpha_3) };
                }
            }
            { let o_ = l_side_1; let n_ = (o_).wrapping_add(1); l_side_1 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudCombo_DrawText", &PROBE_N, &[], true); }
    r_
}

pub fn HudCombo_Init(c: &mut Ctx, mut p_out_1: u32, mut p_res_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudCombo_Init", &PROBE_N, &[(p_out_1) as u32, (p_res_2) as u32], false); }
    let mut l_i_3: i32 = 0;
    let mut l_spr_4: u32 = 0;
    let mut l_node_5: u32 = 0;
    let r_ = 'f: {
    { let v_ = F::Heap_Alloc(c, ((396u32) as i32), ((32i32) as u32), 0i32, 2i32); let a_ = sym::gHudCombo; c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32(sym::gHudCombo); F::memset(c, a0_, 0i32, 396u32) }; };
    { let v_ = F::Heap_Alloc(c, (((((12i32) as u32)).wrapping_mul(28u32)) as i32), ((32i32) as u32), 0i32, 2i32); let a_ = (c.m.r32(sym::gHudCombo)).wrapping_add(4); c.m.w32(a_, v_); };
    { let _ = { let a0_ = { let a_ = (c.m.r32(sym::gHudCombo)).wrapping_add(4); c.m.r32(a_) }; F::memset(c, a0_, 0i32, (((12i32) as u32)).wrapping_mul(28u32)) }; };
    { let v_ = F::Heap_Alloc(c, (((((6i32) as u32)).wrapping_mul(56u32)) as i32), ((32i32) as u32), 0i32, 2i32); let a_ = (c.m.r32(sym::gHudCombo)).wrapping_add(8); c.m.w32(a_, v_); };
    { let _ = { let a0_ = { let a_ = (c.m.r32(sym::gHudCombo)).wrapping_add(8); c.m.r32(a_) }; F::memset(c, a0_, 0i32, (((6i32) as u32)).wrapping_mul(56u32)) }; };
    {
        l_i_3 = 0i32;
        'l1: loop {
            if !(((((l_i_3) < (2i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let v_ = (1i32).wrapping_neg(); let a_ = ((c.m.r32(sym::gHudCombo)).wrapping_add(48)).wrapping_add((((l_i_3) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = (1i32).wrapping_neg(); let a_ = ((c.m.r32(sym::gHudCombo)).wrapping_add(16)).wrapping_add((((l_i_3) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = (1i32).wrapping_neg(); let a_ = ((c.m.r32(sym::gHudCombo)).wrapping_add(40)).wrapping_add((((l_i_3) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = 10i32; let a_ = ((c.m.r32(sym::gHudCombo)).wrapping_add(24)).wrapping_add((((l_i_3) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = 10i32; let a_ = ((c.m.r32(sym::gHudCombo)).wrapping_add(32)).wrapping_add((((l_i_3) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            }
            { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
        }
    }
    { let v_ = p_res_2; let a_ = c.m.r32(sym::gHudCombo); c.m.w32(a_, v_); };
    l_spr_4 = ({ let a_ = (c.m.r32(sym::gHudCombo)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((0i32) as u32)).wrapping_mul(28));
    F::HudSprite_InitTex(c, l_spr_4, p_res_2, 2i32, 0i32);
    F::HudSprite_Show(c, l_spr_4, 0i32);
    l_spr_4 = ({ let a_ = (c.m.r32(sym::gHudCombo)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((1i32) as u32)).wrapping_mul(28));
    F::HudSprite_InitPlain(c, l_spr_4, ((128i32) as u8), ((128i32) as u8), ((128i32) as u8), ((128i32) as u8));
    F::HudSprite_SetRect(c, l_spr_4, 0i32, 128i32, 0i32, 32i32);
    F::HudSprite_Show(c, l_spr_4, 0i32);
    l_spr_4 = ({ let a_ = (c.m.r32(sym::gHudCombo)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((2i32) as u32)).wrapping_mul(28));
    F::HudSprite_InitTex(c, l_spr_4, p_res_2, 2i32, 0i32);
    F::HudSprite_SetRect(c, l_spr_4, 0i32, 128i32, 0i32, 32i32);
    F::HudSprite_SetUv(c, l_spr_4, 0i32, 128i32, 32i32, 64i32);
    F::HudSprite_Show(c, l_spr_4, 0i32);
    l_spr_4 = ({ let a_ = (c.m.r32(sym::gHudCombo)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((3i32) as u32)).wrapping_mul(28));
    F::HudSprite_InitTex(c, l_spr_4, p_res_2, 0i32, 0i32);
    F::HudSprite_Show(c, l_spr_4, 0i32);
    l_spr_4 = ({ let a_ = (c.m.r32(sym::gHudCombo)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((4i32) as u32)).wrapping_mul(28));
    F::HudSprite_InitTex(c, l_spr_4, p_res_2, 0i32, 0i32);
    F::HudSprite_Show(c, l_spr_4, 0i32);
    l_spr_4 = ({ let a_ = (c.m.r32(sym::gHudCombo)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((5i32) as u32)).wrapping_mul(28));
    F::HudSprite_InitTex(c, l_spr_4, p_res_2, 0i32, 0i32);
    F::HudSprite_Show(c, l_spr_4, 0i32);
    l_spr_4 = ({ let a_ = (c.m.r32(sym::gHudCombo)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((6i32) as u32)).wrapping_mul(28));
    F::HudSprite_InitTex(c, l_spr_4, p_res_2, 0i32, 0i32);
    F::HudSprite_Show(c, l_spr_4, 0i32);
    l_spr_4 = ({ let a_ = (c.m.r32(sym::gHudCombo)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((7i32) as u32)).wrapping_mul(28));
    F::HudSprite_InitTex(c, l_spr_4, p_res_2, 0i32, 0i32);
    F::HudSprite_Show(c, l_spr_4, 0i32);
    l_spr_4 = ({ let a_ = (c.m.r32(sym::gHudCombo)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((8i32) as u32)).wrapping_mul(28));
    F::HudSprite_InitTex(c, l_spr_4, p_res_2, 2i32, 0i32);
    F::HudSprite_SetRect(c, l_spr_4, 0i32, 128i32, (30i32).wrapping_neg(), 2i32);
    F::HudSprite_SetUv(c, l_spr_4, 0i32, 128i32, 2i32, 32i32);
    F::HudSprite_Show(c, l_spr_4, 0i32);
    l_spr_4 = ({ let a_ = (c.m.r32(sym::gHudCombo)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((9i32) as u32)).wrapping_mul(28));
    F::HudSprite_InitTex(c, l_spr_4, p_res_2, 0i32, 0i32);
    F::HudSprite_Show(c, l_spr_4, 0i32);
    l_spr_4 = ({ let a_ = (c.m.r32(sym::gHudCombo)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((10i32) as u32)).wrapping_mul(28));
    F::HudSprite_InitTex(c, l_spr_4, p_res_2, 0i32, 0i32);
    F::HudSprite_Show(c, l_spr_4, 0i32);
    l_spr_4 = ({ let a_ = (c.m.r32(sym::gHudCombo)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((11i32) as u32)).wrapping_mul(28));
    F::HudSprite_InitTex(c, l_spr_4, p_res_2, 0i32, 0i32);
    F::HudSprite_Show(c, l_spr_4, 0i32);
    l_node_5 = ({ let a_ = (c.m.r32(sym::gHudCombo)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((2i32) as u32)).wrapping_mul(56));
    F::HudNode_SetPos(c, l_node_5, 0i32, 16i32);
    { let v_ = ((1i32) as u32); let a_ = (l_node_5).wrapping_add(32); c.m.w32(a_, v_); };
    { let v_ = { let a0_ = (((c.m.r32((l_node_5).wrapping_add(32))).wrapping_mul(4u32)) as i32); F::Heap_Alloc(c, a0_, ((32i32) as u32), 0i32, 2i32) }; let a_ = (l_node_5).wrapping_add(36); c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32((l_node_5).wrapping_add(36)); let a2_ = (c.m.r32((l_node_5).wrapping_add(32))).wrapping_mul(4u32); F::memset(c, a0_, 0i32, a2_) }; };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudCombo)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((0i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_node_5).wrapping_add(36))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = fid::HudCombo_UpdateMessage; let a_ = (l_node_5).wrapping_add(48); c.m.w32(a_, v_); };
    { let v_ = fid::HudCombo_DrawSprites; let a_ = (l_node_5).wrapping_add(52); c.m.w32(a_, v_); };
    l_node_5 = ({ let a_ = (c.m.r32(sym::gHudCombo)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((3i32) as u32)).wrapping_mul(56));
    F::HudNode_SetPos(c, l_node_5, 0i32, 4i32);
    { let v_ = ((1i32) as u32); let a_ = (l_node_5).wrapping_add(32); c.m.w32(a_, v_); };
    { let v_ = { let a0_ = (((c.m.r32((l_node_5).wrapping_add(32))).wrapping_mul(4u32)) as i32); F::Heap_Alloc(c, a0_, ((32i32) as u32), 0i32, 2i32) }; let a_ = (l_node_5).wrapping_add(36); c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32((l_node_5).wrapping_add(36)); let a2_ = (c.m.r32((l_node_5).wrapping_add(32))).wrapping_mul(4u32); F::memset(c, a0_, 0i32, a2_) }; };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudCombo)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((1i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_node_5).wrapping_add(36))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = fid::HudCombo_UpdateText; let a_ = (l_node_5).wrapping_add(48); c.m.w32(a_, v_); };
    { let v_ = fid::HudCombo_StoreTextPos; let a_ = (l_node_5).wrapping_add(52); c.m.w32(a_, v_); };
    l_node_5 = ({ let a_ = (c.m.r32(sym::gHudCombo)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((4i32) as u32)).wrapping_mul(56));
    F::HudNode_SetPos(c, l_node_5, 0i32, 80i32);
    { let v_ = ((6i32) as u32); let a_ = (l_node_5).wrapping_add(32); c.m.w32(a_, v_); };
    { let v_ = { let a0_ = (((c.m.r32((l_node_5).wrapping_add(32))).wrapping_mul(4u32)) as i32); F::Heap_Alloc(c, a0_, ((32i32) as u32), 0i32, 2i32) }; let a_ = (l_node_5).wrapping_add(36); c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32((l_node_5).wrapping_add(36)); let a2_ = (c.m.r32((l_node_5).wrapping_add(32))).wrapping_mul(4u32); F::memset(c, a0_, 0i32, a2_) }; };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudCombo)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((2i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_node_5).wrapping_add(36))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudCombo)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((3i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_node_5).wrapping_add(36))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudCombo)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((4i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_node_5).wrapping_add(36))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudCombo)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((5i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_node_5).wrapping_add(36))).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudCombo)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((6i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_node_5).wrapping_add(36))).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudCombo)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((7i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_node_5).wrapping_add(36))).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = fid::HudCombo_UpdateDamage; let a_ = (l_node_5).wrapping_add(48); c.m.w32(a_, v_); };
    { let v_ = fid::HudCombo_DrawSprites; let a_ = (l_node_5).wrapping_add(52); c.m.w32(a_, v_); };
    F::HudNode_Show(c, l_node_5, 0i32);
    l_node_5 = ({ let a_ = (c.m.r32(sym::gHudCombo)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((5i32) as u32)).wrapping_mul(56));
    F::HudNode_SetPos(c, l_node_5, 0i32, 80i32);
    { let v_ = ((4i32) as u32); let a_ = (l_node_5).wrapping_add(32); c.m.w32(a_, v_); };
    { let v_ = { let a0_ = (((c.m.r32((l_node_5).wrapping_add(32))).wrapping_mul(4u32)) as i32); F::Heap_Alloc(c, a0_, ((32i32) as u32), 0i32, 2i32) }; let a_ = (l_node_5).wrapping_add(36); c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32((l_node_5).wrapping_add(36)); let a2_ = (c.m.r32((l_node_5).wrapping_add(32))).wrapping_mul(4u32); F::memset(c, a0_, 0i32, a2_) }; };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudCombo)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((8i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_node_5).wrapping_add(36))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudCombo)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((9i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_node_5).wrapping_add(36))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudCombo)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((10i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_node_5).wrapping_add(36))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudCombo)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((11i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_node_5).wrapping_add(36))).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = fid::HudCombo_UpdateHits; let a_ = (l_node_5).wrapping_add(48); c.m.w32(a_, v_); };
    { let v_ = fid::HudCombo_DrawSprites; let a_ = (l_node_5).wrapping_add(52); c.m.w32(a_, v_); };
    F::HudNode_Show(c, l_node_5, 0i32);
    l_node_5 = ({ let a_ = (c.m.r32(sym::gHudCombo)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((1i32) as u32)).wrapping_mul(56));
    F::HudNode_SetPos(c, l_node_5, 0i32, 85i32);
    { let v_ = ((0i32) as u32); let a_ = (l_node_5).wrapping_add(32); c.m.w32(a_, v_); };
    { let v_ = ((4i32) as u32); let a_ = (l_node_5).wrapping_add(40); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (l_node_5).wrapping_add(48); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (l_node_5).wrapping_add(52); c.m.w32(a_, v_); };
    { let v_ = { let a0_ = (((c.m.r32((l_node_5).wrapping_add(40))).wrapping_mul(4u32)) as i32); F::Heap_Alloc(c, a0_, ((32i32) as u32), 0i32, 2i32) }; let a_ = (l_node_5).wrapping_add(44); c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32((l_node_5).wrapping_add(44)); let a2_ = (c.m.r32((l_node_5).wrapping_add(40))).wrapping_mul(4u32); F::memset(c, a0_, 0i32, a2_) }; };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudCombo)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((2i32) as u32)).wrapping_mul(56)); let a_ = (c.m.r32((l_node_5).wrapping_add(44))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudCombo)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((3i32) as u32)).wrapping_mul(56)); let a_ = (c.m.r32((l_node_5).wrapping_add(44))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudCombo)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((4i32) as u32)).wrapping_mul(56)); let a_ = (c.m.r32((l_node_5).wrapping_add(44))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudCombo)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((5i32) as u32)).wrapping_mul(56)); let a_ = (c.m.r32((l_node_5).wrapping_add(44))).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    l_node_5 = ({ let a_ = (c.m.r32(sym::gHudCombo)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((0i32) as u32)).wrapping_mul(56));
    F::HudNode_SetPos(c, l_node_5, 0i32, 0i32);
    { let v_ = ((0i32) as u32); let a_ = (l_node_5).wrapping_add(32); c.m.w32(a_, v_); };
    { let v_ = ((1i32) as u32); let a_ = (l_node_5).wrapping_add(40); c.m.w32(a_, v_); };
    { let v_ = fid::HudCombo_UpdateRoot; let a_ = (l_node_5).wrapping_add(48); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (l_node_5).wrapping_add(52); c.m.w32(a_, v_); };
    { let v_ = { let a0_ = (((c.m.r32((l_node_5).wrapping_add(40))).wrapping_mul(4u32)) as i32); F::Heap_Alloc(c, a0_, ((32i32) as u32), 0i32, 2i32) }; let a_ = (l_node_5).wrapping_add(44); c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32((l_node_5).wrapping_add(44)); let a2_ = (c.m.r32((l_node_5).wrapping_add(40))).wrapping_mul(4u32); F::memset(c, a0_, 0i32, a2_) }; };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudCombo)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((1i32) as u32)).wrapping_mul(56)); let a_ = (c.m.r32((l_node_5).wrapping_add(44))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = { let a_ = (c.m.r32(sym::gHudCombo)).wrapping_add(8); c.m.r32(a_) }; let a_ = p_out_1; c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudCombo_Init", &PROBE_N, &[], true); }
    r_
}

pub fn HudCombo_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudCombo_Term", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let r_ = 'f: {
    if ((((({ let a_ = (c.m.r32(sym::gHudCombo)).wrapping_add(4); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = { let a_ = (c.m.r32(sym::gHudCombo)).wrapping_add(4); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
    }
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (6i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((((({ let a_ = (({ let a_ = (c.m.r32(sym::gHudCombo)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((l_i_1) as u32)).wrapping_mul(56))).wrapping_add(36); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
                    { let a0_ = { let a_ = (({ let a_ = (c.m.r32(sym::gHudCombo)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((l_i_1) as u32)).wrapping_mul(56))).wrapping_add(36); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
                }
                if ((((({ let a_ = (({ let a_ = (c.m.r32(sym::gHudCombo)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((l_i_1) as u32)).wrapping_mul(56))).wrapping_add(44); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
                    { let a0_ = { let a_ = (({ let a_ = (c.m.r32(sym::gHudCombo)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((l_i_1) as u32)).wrapping_mul(56))).wrapping_add(44); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
                }
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
    if ((((({ let a_ = (c.m.r32(sym::gHudCombo)).wrapping_add(8); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = { let a_ = (c.m.r32(sym::gHudCombo)).wrapping_add(8); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
    }
    if (((((c.m.r32(sym::gHudCombo)) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32(sym::gHudCombo); F::Heap_Free(c, a0_) };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudCombo_Term", &PROBE_N, &[], true); }
    r_
}

pub fn HudCombo_Reset(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudCombo_Reset", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let r_ = 'f: {
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (2i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let v_ = (1i32).wrapping_neg(); let a_ = ((c.m.r32(sym::gHudCombo)).wrapping_add(16)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = 10i32; let a_ = ((c.m.r32(sym::gHudCombo)).wrapping_add(24)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = 10i32; let a_ = ((c.m.r32(sym::gHudCombo)).wrapping_add(32)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = (1i32).wrapping_neg(); let a_ = ((c.m.r32(sym::gHudCombo)).wrapping_add(40)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = (1i32).wrapping_neg(); let a_ = ((c.m.r32(sym::gHudCombo)).wrapping_add(48)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = ((c.m.r32(sym::gHudCombo)).wrapping_add(56)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = ((c.m.r32(sym::gHudCombo)).wrapping_add(64)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gHudCombo)).wrapping_add(72)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                { let v_ = 0i32; let a_ = ((c.m.r32(sym::gHudCombo)).wrapping_add(80)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = ((c.m.r32(sym::gHudCombo)).wrapping_add(88)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = ((c.m.r32(sym::gHudCombo)).wrapping_add(96)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let _ = { let a0_ = ((c.m.r32(sym::gHudCombo)).wrapping_add(104)).wrapping_add((((l_i_1) as u32)).wrapping_mul(24)); F::memset(c, a0_, 0i32, 24u32) }; };
                { let _ = { let a0_ = ((c.m.r32(sym::gHudCombo)).wrapping_add(152)).wrapping_add((((l_i_1) as u32)).wrapping_mul(24)); F::memset(c, a0_, 0i32, 24u32) }; };
                { let _ = { let a0_ = ((c.m.r32(sym::gHudCombo)).wrapping_add(200)).wrapping_add((((l_i_1) as u32)).wrapping_mul(24)); F::memset(c, a0_, 0i32, 24u32) }; };
                { let _ = { let a0_ = ((c.m.r32(sym::gHudCombo)).wrapping_add(248)).wrapping_add((((l_i_1) as u32)).wrapping_mul(24)); F::memset(c, a0_, 0i32, 24u32) }; };
                { let _ = { let a0_ = ((c.m.r32(sym::gHudCombo)).wrapping_add(296)).wrapping_add((((l_i_1) as u32)).wrapping_mul(24)); F::memset(c, a0_, 0i32, 24u32) }; };
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
    { let _ = { let a0_ = (c.m.r32(sym::gHudCombo)).wrapping_add(344); F::memset(c, a0_, 0i32, 24u32) }; };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudCombo_Reset", &PROBE_N, &[], true); }
    r_
}
