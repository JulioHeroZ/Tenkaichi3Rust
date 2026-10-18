#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn HudPrompt_UpdateRoot(c: &mut Ctx, mut p_node_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudPrompt_UpdateRoot", &PROBE_N, &[(p_node_1) as u32], false); }
    let r_ = 'f: {
    if ((((((((!(((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0)) as i32)) != 0) && ((((({ let a_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(20); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0)) as i32)) != 0) {
        { let _ = { let a0_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(200); F::Ramp_Step(c, a0_) }; };
    }
    { let a1_ = rt::f2i(rt::fmul({ let a_ = ((c.m.r32(sym::gHudPrompt)).wrapping_add(200)).wrapping_add(16); F32(c.m.r32(a_)) }, F32(0xc3800000))); let a2_ = rt::f2i(rt::fmul({ let a_ = ((c.m.r32(sym::gHudPrompt)).wrapping_add(200)).wrapping_add(16); F32(c.m.r32(a_)) }, F32(0x43600000))); F::HudNode_SetPos(c, p_node_1, a1_, a2_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudPrompt_UpdateRoot", &PROBE_N, &[], true); }
    r_
}

pub fn HudPrompt_ClearIconMark(c: &mut Ctx, mut p_tex_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudPrompt_ClearIconMark", &PROBE_N, &[(p_tex_1) as u32], false); }
    let mut l_t_2: u32 = 0;
    let r_ = 'f: {
    l_t_2 = ({ let a_ = ({ let a_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(12); c.m.r32(a_) }).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((p_tex_1) as u32)).wrapping_mul(64));
    { let v_ = 0i32; let a_ = (l_t_2).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudPrompt_ClearIconMark", &PROBE_N, &[], true); }
    r_
}

pub fn HudPrompt_ClearIconMarks(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudPrompt_ClearIconMarks", &PROBE_N, &[], false); }
    let mut l_n_1: i32 = 0;
    let mut l_i_2: i32 = 0;
    let mut l_t_3: u32 = 0;
    let r_ = 'f: {
    l_n_1 = { let a_ = { let a_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(12); c.m.r32(a_) }; (c.m.r32(a_) as i32) };
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < (l_n_1)) as i32)) != 0) { break 'l1; }
            'c2: {
                l_t_3 = ({ let a_ = ({ let a_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(12); c.m.r32(a_) }).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((l_i_2) as u32)).wrapping_mul(64));
                { let v_ = 0i32; let a_ = (l_t_3).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudPrompt_ClearIconMarks", &PROBE_N, &[], true); }
    r_
}

pub fn HudPrompt_ApplyIconDef(c: &mut Ctx, mut p_spr_1: u32, mut p_def_2: u32, mut p_icon_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudPrompt_ApplyIconDef", &PROBE_N, &[(p_spr_1) as u32, (p_def_2) as u32, (p_icon_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_scale_4: F32 = F32(0);
    let mut l_lo_5: i32 = 0;
    let mut l_hi_6: i32 = 0;
    let mut l_frame_7: i32 = 0;
    let mut l_time_8: u32 = 0;
    let mut l_next_9: i32 = 0;
    let mut l_w_10: i32 = 0;
    let mut l_w_11: i32 = 0;
    let r_ = 'f: {
        l_scale_4 = F32(0x3f800000);
        's1: {
            let sv12: i32 = (c.m.r32((p_icon_3).wrapping_add(12)) as i32);
            match sv12 {
                0i32 => {
                    l_lo_5 = 0i32;
                    l_hi_6 = (if (((((((c.m.r8((p_def_2).wrapping_add(3))) as i32)) < (((c.m.r16((p_def_2).wrapping_add(4))) as i32))) as i32)) != 0) { ((c.m.r8((p_def_2).wrapping_add(3))) as i32) } else { ((c.m.r16((p_def_2).wrapping_add(4))) as i32) });
                    break 's1;
                }
                4i32 => {
                    l_lo_5 = (if (((((((c.m.r8((p_def_2).wrapping_add(3))) as i32)) < (((c.m.r16((p_def_2).wrapping_add(4))) as i32))) as i32)) != 0) { ((c.m.r8((p_def_2).wrapping_add(3))) as i32) } else { ((c.m.r16((p_def_2).wrapping_add(4))) as i32) });
                    l_hi_6 = (if (((((((c.m.r8((p_def_2).wrapping_add(3))) as i32)) < (((c.m.r16((p_def_2).wrapping_add(4))) as i32))) as i32)) != 0) { ((c.m.r8((p_def_2).wrapping_add(3))) as i32) } else { ((c.m.r16((p_def_2).wrapping_add(4))) as i32) });
                    break 's1;
                }
                1i32 => {
                    l_lo_5 = (if (((((((c.m.r8((p_def_2).wrapping_add(3))) as i32)) < (((c.m.r16((p_def_2).wrapping_add(4))) as i32))) as i32)) != 0) { ((c.m.r8((p_def_2).wrapping_add(3))) as i32) } else { ((c.m.r16((p_def_2).wrapping_add(4))) as i32) });
                    l_hi_6 = (((c.m.r16((p_def_2).wrapping_add(4))) as i32)).wrapping_add(1i32);
                    break 's1;
                }
                3i32 => {
                    l_lo_5 = 0i32;
                    l_hi_6 = 0i32;
                    break 's1;
                }
                _ => {
                    l_lo_5 = 0i32;
                    l_hi_6 = (((c.m.r16((p_def_2).wrapping_add(4))) as i32)).wrapping_add(1i32);
                    break 's1;
                }
            }
        }
        l_frame_7 = (c.m.r32((p_icon_3).wrapping_add(4)) as i32);
        if (((((((((l_frame_7) >= (0i32)) as i32)) != 0) && (((((l_frame_7) < (l_lo_5)) as i32)) != 0)) as i32)) != 0) {
            { let v_ = l_lo_5; let a_ = (p_icon_3).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
            l_frame_7 = l_lo_5;
            { let v_ = 0i32; let a_ = (p_icon_3).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        }
        if (((c.m.r32((p_icon_3).wrapping_add(20)) as i32)) != 0) {
            if (((((l_frame_7) < (0i32)) as i32)) != 0) {
                { let v_ = l_lo_5; let a_ = (p_icon_3).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                l_frame_7 = l_lo_5;
                { let v_ = 0i32; let a_ = (p_icon_3).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
            }
        }
        if (((((l_frame_7) < (0i32)) as i32)) != 0) {
            if ((((((c.m.r32((p_icon_3).wrapping_add(8)) as i32)) >= (20i32)) as i32)) != 0) {
                { let v_ = l_lo_5; let a_ = (p_icon_3).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                l_frame_7 = l_lo_5;
                { let v_ = 0i32; let a_ = (p_icon_3).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
            }
        } else {
            l_time_8 = (p_def_2).wrapping_add(6);
            if ((((((c.m.r32((p_icon_3).wrapping_add(8)) as i32)) >= ((((c.m.r8((l_time_8).wrapping_add(((l_frame_7) as u32))) as i8)) as i32))) as i32)) != 0) {
                l_next_9 = (l_frame_7).wrapping_add(1i32);
                { let v_ = l_frame_7; let a_ = (p_icon_3).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
                { let v_ = l_next_9; let a_ = (p_icon_3).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (p_icon_3).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                if (((((((((((((l_next_9) >= (6i32)) as i32)) != 0) || (((((l_next_9) >= (l_hi_6)) as i32)) != 0)) as i32)) != 0) || ((((((((c.m.r8((l_time_8).wrapping_add(((l_next_9) as u32))) as i8)) as i32)) == (0i32)) as i32)) != 0)) as i32)) != 0) {
                    { let v_ = (1i32).wrapping_neg(); let a_ = (p_icon_3).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    l_frame_7 = (1i32).wrapping_neg();
                } else {
                    l_frame_7 = l_next_9;
                }
            }
        }
        if (((((l_frame_7) < (0i32)) as i32)) != 0) {
            {
                l_w_10 = (((c.m.r8((p_def_2).wrapping_add(2))) as i32)).wrapping_add(1i32);
                if (((((l_w_10) >= (((c.m.r8((p_def_2).wrapping_add(1))) as i32))) as i32)) != 0) {
                    l_w_10 = ((c.m.r8((p_def_2).wrapping_add(1))) as i32);
                }
                { let v_ = (rt::rem_i32((c.m.r32((p_icon_3).wrapping_add(16)) as i32), 2i32)).wrapping_mul(((c.m.r8((p_def_2).wrapping_add(1))) as i32)); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = ((rt::rem_i32((c.m.r32((p_icon_3).wrapping_add(16)) as i32), 2i32)).wrapping_mul(((c.m.r8((p_def_2).wrapping_add(1))) as i32))).wrapping_add(rt::f2i(rt::fmul(rt::i2f(l_w_10), l_scale_4))); let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = (rt::div_i32((c.m.r32((p_icon_3).wrapping_add(16)) as i32), 2i32)).wrapping_mul(((c.m.r8((p_def_2).wrapping_add(1))) as i32)); let a_ = (fp).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = ((rt::div_i32((c.m.r32((p_icon_3).wrapping_add(16)) as i32), 2i32)).wrapping_mul(((c.m.r8((p_def_2).wrapping_add(1))) as i32))).wrapping_add(((c.m.r8((p_def_2).wrapping_add(1))) as i32)); let a_ = (fp).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = (rt::rem_i32((c.m.r32((p_icon_3).wrapping_add(16)) as i32), 2i32)).wrapping_mul(((c.m.r8((p_def_2).wrapping_add(1))) as i32)); let a_ = (fp.wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = ((rt::rem_i32((c.m.r32((p_icon_3).wrapping_add(16)) as i32), 2i32)).wrapping_mul(((c.m.r8((p_def_2).wrapping_add(1))) as i32))).wrapping_add(l_w_10); let a_ = (fp.wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = (rt::div_i32((c.m.r32((p_icon_3).wrapping_add(16)) as i32), 2i32)).wrapping_mul(((c.m.r8((p_def_2).wrapping_add(1))) as i32)); let a_ = (fp.wrapping_add(16)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = ((rt::div_i32((c.m.r32((p_icon_3).wrapping_add(16)) as i32), 2i32)).wrapping_mul(((c.m.r8((p_def_2).wrapping_add(1))) as i32))).wrapping_add(((c.m.r8((p_def_2).wrapping_add(1))) as i32)); let a_ = (fp.wrapping_add(16)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            }
        } else {
            {
                l_w_11 = (((c.m.r8((p_def_2).wrapping_add(2))) as i32)).wrapping_add(1i32);
                if (((((l_w_11) >= (((c.m.r8((p_def_2).wrapping_add(1))) as i32))) as i32)) != 0) {
                    l_w_11 = ((c.m.r8((p_def_2).wrapping_add(1))) as i32);
                }
                { let v_ = (rt::rem_i32(l_frame_7, 2i32)).wrapping_mul(((c.m.r8((p_def_2).wrapping_add(1))) as i32)); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = ((rt::rem_i32(l_frame_7, 2i32)).wrapping_mul(((c.m.r8((p_def_2).wrapping_add(1))) as i32))).wrapping_add(rt::f2i(rt::fmul(rt::i2f(l_w_11), l_scale_4))); let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = (rt::div_i32(l_frame_7, 2i32)).wrapping_mul(((c.m.r8((p_def_2).wrapping_add(1))) as i32)); let a_ = (fp).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = ((rt::div_i32(l_frame_7, 2i32)).wrapping_mul(((c.m.r8((p_def_2).wrapping_add(1))) as i32))).wrapping_add(((c.m.r8((p_def_2).wrapping_add(1))) as i32)); let a_ = (fp).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = (rt::rem_i32(l_frame_7, 2i32)).wrapping_mul(((c.m.r8((p_def_2).wrapping_add(1))) as i32)); let a_ = (fp.wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = ((rt::rem_i32(l_frame_7, 2i32)).wrapping_mul(((c.m.r8((p_def_2).wrapping_add(1))) as i32))).wrapping_add(l_w_11); let a_ = (fp.wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = (rt::div_i32(l_frame_7, 2i32)).wrapping_mul(((c.m.r8((p_def_2).wrapping_add(1))) as i32)); let a_ = (fp.wrapping_add(16)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = ((rt::div_i32(l_frame_7, 2i32)).wrapping_mul(((c.m.r8((p_def_2).wrapping_add(1))) as i32))).wrapping_add(((c.m.r8((p_def_2).wrapping_add(1))) as i32)); let a_ = (fp.wrapping_add(16)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            }
        }
        { let a1_ = ((c.m.r8(p_def_2)) as i32); F::HudSprite_SetTex(c, p_spr_1, a1_, 0i32) };
        { let a1_ = (c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32); let a2_ = (c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32); let a3_ = (c.m.r32((fp).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32); let a4_ = (c.m.r32((fp).wrapping_add((((3i32) as u32)).wrapping_mul(4))) as i32); F::HudSprite_SetRect(c, p_spr_1, a1_, a2_, a3_, a4_) };
        F::HudSprite_Center(c, p_spr_1);
        { let a1_ = (c.m.r32((fp.wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32); let a2_ = (c.m.r32((fp.wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32); let a3_ = (c.m.r32((fp.wrapping_add(16)).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32); let a4_ = (c.m.r32((fp.wrapping_add(16)).wrapping_add((((3i32) as u32)).wrapping_mul(4))) as i32); F::HudSprite_SetUv(c, p_spr_1, a1_, a2_, a3_, a4_) };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "HudPrompt_ApplyIconDef", &PROBE_N, &[], true); }
    r_
}

pub fn HudPrompt_ApplyIcon(c: &mut Ctx, mut p_spr_1: u32, mut p_icon_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudPrompt_ApplyIcon", &PROBE_N, &[(p_spr_1) as u32, (p_icon_2) as u32], false); }
    let r_ = 'f: {
    if ((((((c.m.r32(p_icon_2) as i32)) < (16i32)) as i32)) != 0) {
        { let a1_ = (sym::gHudPromptIconDefs).wrapping_add(((((c.m.r32(p_icon_2) as i32)) as u32)).wrapping_mul(12)); F::HudPrompt_ApplyIconDef(c, p_spr_1, a1_, p_icon_2) };
    } else {
        if ((((((c.m.r32(p_icon_2) as i32)) < (33i32)) as i32)) != 0) {
            { let a1_ = (sym::gHudPromptIconDefs16).wrapping_add((((((c.m.r32(p_icon_2) as i32)).wrapping_sub(16i32)) as u32)).wrapping_mul(12)); F::HudPrompt_ApplyIconDef(c, p_spr_1, a1_, p_icon_2) };
        } else {
            { let a1_ = ({ let a_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(16); c.m.r32(a_) }).wrapping_add(((((c.m.r32(p_icon_2) as i32)) as u32)).wrapping_mul(12)); F::HudPrompt_ApplyIconDef(c, p_spr_1, a1_, p_icon_2) };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudPrompt_ApplyIcon", &PROBE_N, &[], true); }
    r_
}

pub fn hud_e_b__HudPrompt_StepRamp(c: &mut Ctx, mut p_ramp_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "hud_e_b__HudPrompt_StepRamp", &PROBE_N, &[(p_ramp_1) as u32], false); }
    let mut l_r_2: i32 = 0;
    let r_ = 'f: {
    if ((((!(((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0)) as i32)) != 0) {
        l_r_2 = F::Ramp_Step(c, p_ramp_1);
    } else {
        l_r_2 = 0i32;
    }
    break 'f l_r_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "hud_e_b__HudPrompt_StepRamp", &PROBE_N, &[], true); }
    r_
}

pub fn HudPrompt_UpdateButton(c: &mut Ctx, mut p_node_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudPrompt_UpdateButton", &PROBE_N, &[(p_node_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(64) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 64);
    let mut l_alpha_2: F32 = F32(0);
    let mut l_side_3: i32 = 0;
    let mut l_base_4: u32 = 0;
    let mut l_icon2_5: u32 = 0;
    let mut l_pulse_6: u32 = 0;
    let mut l_blink_7: u32 = 0;
    let mut l_state_8: u32 = 0;
    let mut l_ramp_9: u32 = 0;
    let mut l_group_10: i32 = 0;
    let mut l_glow_11: u32 = 0;
    let mut l_icon_12: u32 = 0;
    let mut l_t_13: F32 = F32(0);
    let mut l_drop_14: F32 = F32(0);
    let mut l_d_15: F32 = F32(0);
    let mut l_drop_16: F32 = F32(0);
    let mut l_a_17: F32 = F32(0);
    let r_ = 'f: {
        l_alpha_2 = F32(0x3f800000);
        l_side_3 = { let a_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(20); (c.m.r32(a_) as i32) };
        l_base_4 = ({ let a_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((0i32) as u32)).wrapping_mul(28));
        rt::memset(c, fp, 0, 64);
        { let v_ = 22i32; c.m.w32(fp.wrapping_add(0), (v_) as u32); }
        { let v_ = 21i32; c.m.w32(fp.wrapping_add(4), (v_) as u32); }
        { let v_ = 20i32; c.m.w32(fp.wrapping_add(8), (v_) as u32); }
        { let v_ = 23i32; c.m.w32(fp.wrapping_add(12), (v_) as u32); }
        { let v_ = 54i32; c.m.w32(fp.wrapping_add(16), (v_) as u32); }
        { let v_ = 55i32; c.m.w32(fp.wrapping_add(20), (v_) as u32); }
        { let v_ = 69i32; c.m.w32(fp.wrapping_add(24), (v_) as u32); }
        { let v_ = 70i32; c.m.w32(fp.wrapping_add(28), (v_) as u32); }
        { let v_ = 37i32; c.m.w32(fp.wrapping_add(32), (v_) as u32); }
        { let v_ = 43i32; c.m.w32(fp.wrapping_add(36), (v_) as u32); }
        { let v_ = 41i32; c.m.w32(fp.wrapping_add(40), (v_) as u32); }
        { let v_ = 39i32; c.m.w32(fp.wrapping_add(44), (v_) as u32); }
        { let v_ = 40i32; c.m.w32(fp.wrapping_add(48), (v_) as u32); }
        { let v_ = 44i32; c.m.w32(fp.wrapping_add(52), (v_) as u32); }
        { let v_ = 38i32; c.m.w32(fp.wrapping_add(56), (v_) as u32); }
        { let v_ = 42i32; c.m.w32(fp.wrapping_add(60), (v_) as u32); }
        l_state_8 = ((c.m.r32(sym::gHudPrompt)).wrapping_add(24)).wrapping_add((((l_side_3) as u32)).wrapping_mul(4));
        l_pulse_6 = ((c.m.r32(sym::gHudPrompt)).wrapping_add(32)).wrapping_add((((l_side_3) as u32)).wrapping_mul(24));
        l_ramp_9 = ((c.m.r32(sym::gHudPrompt)).wrapping_add(80)).wrapping_add((((l_side_3) as u32)).wrapping_mul(24));
        l_blink_7 = ((c.m.r32(sym::gHudPrompt)).wrapping_add(512)).wrapping_add((((l_side_3) as u32)).wrapping_mul(24));
        l_group_10 = { let a_ = ((c.m.r32(sym::gHudPrompt)).wrapping_add(712)).wrapping_add((((l_side_3) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
        { let a1_ = { let a_ = c.m.r32(sym::gHudPrompt); c.m.r32(a_) }; F::HudSprite_InitTex(c, l_base_4, a1_, 0i32, 0i32) };
        F::HudSprite_Center(c, l_base_4);
        l_glow_11 = (l_base_4).wrapping_add((((3i32) as u32)).wrapping_mul(28));
        l_icon_12 = (l_base_4).wrapping_add((((1i32) as u32)).wrapping_mul(28));
        l_icon2_5 = (l_base_4).wrapping_add((((2i32) as u32)).wrapping_mul(28));
        { let a1_ = { let a_ = c.m.r32(sym::gHudPrompt); c.m.r32(a_) }; F::HudSprite_InitTex(c, l_glow_11, a1_, 7i32, 0i32) };
        F::HudSprite_Center(c, l_glow_11);
        F::HudSprite_Move(c, l_glow_11, 0i32, (24i32).wrapping_neg());
        F::HudNode_Show(c, p_node_1, 1i32);
        F::HudSprite_Show(c, l_icon_12, 1i32);
        F::HudSprite_Show(c, l_icon2_5, 0i32);
        F::HudSprite_Show(c, l_base_4, 1i32);
        F::HudSprite_Show(c, l_glow_11, 1i32);
        's1: {
            let sv18: i32 = (c.m.r32(l_state_8) as i32);
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
                match sv18 { 1i32 => break 'k2, 2i32 => break 'k3, 3i32 => break 'k4, 4i32 => break 'k5, 5i32 => break 'k6, 6i32 => break 'k7, 7i32 => break 'k8, 8i32 => break 'k9, 9i32 => break 'k10, 10i32 => break 'k11, 11i32 => break 'k12, 12i32 => break 'k13, 14i32 => break 'k13, 16i32 => break 'k13, 18i32 => break 'k13, 13i32 => break 'k14, 15i32 => break 'k14, 17i32 => break 'k14, 19i32 => break 'k14, 20i32 => break 'k15, 22i32 => break 'k15, 24i32 => break 'k15, 26i32 => break 'k15, 21i32 => break 'k16, 23i32 => break 'k16, 25i32 => break 'k16, 27i32 => break 'k16, 40i32 => break 'k17, 41i32 => break 'k18, 42i32 => break 'k19, _ => break 'k20, }
            }
            F::HudSprite_Show(c, l_glow_11, 0i32);
            F::Ramp_Start(c, l_ramp_9, F32(0x3e8f5c28), F32(0x00000000), F32(0x3f800000));
            F::Ramp_Start(c, l_pulse_6, F32(0x3dcccccc), F32(0x3f800000), F32(0x00000000));
            F::Ramp_Start(c, l_blink_7, F32(0x3fc00000), F32(0x3f800000), F32(0x00000000));
            { let v_ = 2i32; let a_ = l_state_8; c.m.w32(a_, (v_) as u32); };
            { let v_ = (({ let a_ = ({ let a_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(16); c.m.r32(a_) }).wrapping_add(((({ let a_ = (fp).wrapping_add((((((l_group_10).wrapping_mul(4i32)).wrapping_add({ let a_ = ((c.m.r32(sym::gHudPrompt)).wrapping_add(128)).wrapping_add((((l_side_3) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(12)); c.m.r8(a_) }) as i32); let a_ = (((c.m.r32(sym::gHudPrompt)).wrapping_add(128)).wrapping_add(8)).wrapping_add((((l_side_3) as u32)).wrapping_mul(24)); c.m.w32(a_, (v_) as u32); };
            { let v_ = 0i32; let a_ = ((((c.m.r32(sym::gHudPrompt)).wrapping_add(128)).wrapping_add(8)).wrapping_add((((l_side_3) as u32)).wrapping_mul(24))).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
            { let v_ = 1i32; let a_ = ((((c.m.r32(sym::gHudPrompt)).wrapping_add(128)).wrapping_add(8)).wrapping_add((((l_side_3) as u32)).wrapping_mul(24))).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
            { let v_ = 2i32; let a_ = ((((c.m.r32(sym::gHudPrompt)).wrapping_add(128)).wrapping_add(8)).wrapping_add((((l_side_3) as u32)).wrapping_mul(24))).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
            }
            { let v_ = (({ let a_ = ({ let a_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(16); c.m.r32(a_) }).wrapping_add(((({ let a_ = (fp).wrapping_add((((((l_group_10).wrapping_mul(4i32)).wrapping_add({ let a_ = ((c.m.r32(sym::gHudPrompt)).wrapping_add(128)).wrapping_add((((l_side_3) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(12)); c.m.r8(a_) }) as i32); let a_ = (((c.m.r32(sym::gHudPrompt)).wrapping_add(128)).wrapping_add(8)).wrapping_add((((l_side_3) as u32)).wrapping_mul(24)); c.m.w32(a_, (v_) as u32); };
            F::HudSprite_Show(c, l_glow_11, 0i32);
            if ((F::hud_e_b__HudPrompt_StepRamp(c, l_ramp_9)) != 0) {
                { let v_ = 3i32; let a_ = l_state_8; c.m.w32(a_, (v_) as u32); };
            }
            l_alpha_2 = F32(c.m.r32((l_ramp_9).wrapping_add(16)));
            break 's1;
            }
            { let v_ = (({ let a_ = ({ let a_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(16); c.m.r32(a_) }).wrapping_add(((({ let a_ = (fp).wrapping_add((((((l_group_10).wrapping_mul(4i32)).wrapping_add({ let a_ = ((c.m.r32(sym::gHudPrompt)).wrapping_add(128)).wrapping_add((((l_side_3) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(12)); c.m.r8(a_) }) as i32); let a_ = (((c.m.r32(sym::gHudPrompt)).wrapping_add(128)).wrapping_add(8)).wrapping_add((((l_side_3) as u32)).wrapping_mul(24)); c.m.w32(a_, (v_) as u32); };
            F::HudSprite_Show(c, l_glow_11, 0i32);
            if ((((!(((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0)) as i32)) != 0) {
                { let a_ = ((((c.m.r32(sym::gHudPrompt)).wrapping_add(128)).wrapping_add(8)).wrapping_add((((l_side_3) as u32)).wrapping_mul(24))).wrapping_add(8); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
            }
            break 's1;
            }
            F::Ramp_Start(c, l_ramp_9, F32(0x3e8f5c28), F32(0x00000000), F32(0x3f800000));
            F::Ramp_Start(c, l_pulse_6, F32(0x3dcccccc), F32(0x3f800000), F32(0x00000000));
            F::Ramp_Start(c, l_blink_7, F32(0x3fc00000), F32(0x3f800000), F32(0x00000000));
            { let v_ = 5i32; let a_ = l_state_8; c.m.w32(a_, (v_) as u32); };
            { let v_ = (({ let a_ = ({ let a_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(16); c.m.r32(a_) }).wrapping_add(((({ let a_ = (fp).wrapping_add((((((l_group_10).wrapping_mul(4i32)).wrapping_add({ let a_ = ((c.m.r32(sym::gHudPrompt)).wrapping_add(128)).wrapping_add((((l_side_3) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(12)); c.m.r8(a_) }) as i32); let a_ = (((c.m.r32(sym::gHudPrompt)).wrapping_add(128)).wrapping_add(8)).wrapping_add((((l_side_3) as u32)).wrapping_mul(24)); c.m.w32(a_, (v_) as u32); };
            { let v_ = 0i32; let a_ = ((((c.m.r32(sym::gHudPrompt)).wrapping_add(128)).wrapping_add(8)).wrapping_add((((l_side_3) as u32)).wrapping_mul(24))).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
            { let v_ = 1i32; let a_ = ((((c.m.r32(sym::gHudPrompt)).wrapping_add(128)).wrapping_add(8)).wrapping_add((((l_side_3) as u32)).wrapping_mul(24))).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
            { let v_ = 1i32; let a_ = ((((c.m.r32(sym::gHudPrompt)).wrapping_add(128)).wrapping_add(8)).wrapping_add((((l_side_3) as u32)).wrapping_mul(24))).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
            }
            { let v_ = (({ let a_ = ({ let a_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(16); c.m.r32(a_) }).wrapping_add(((({ let a_ = (fp).wrapping_add((((((l_group_10).wrapping_mul(4i32)).wrapping_add({ let a_ = ((c.m.r32(sym::gHudPrompt)).wrapping_add(128)).wrapping_add((((l_side_3) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(12)); c.m.r8(a_) }) as i32); let a_ = (((c.m.r32(sym::gHudPrompt)).wrapping_add(128)).wrapping_add(8)).wrapping_add((((l_side_3) as u32)).wrapping_mul(24)); c.m.w32(a_, (v_) as u32); };
            if ((F::hud_e_b__HudPrompt_StepRamp(c, l_ramp_9)) != 0) {
                { let v_ = 6i32; let a_ = l_state_8; c.m.w32(a_, (v_) as u32); };
            }
            l_alpha_2 = F32(c.m.r32((l_ramp_9).wrapping_add(16)));
            break 's1;
            }
            { let v_ = (({ let a_ = ({ let a_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(16); c.m.r32(a_) }).wrapping_add(((({ let a_ = (fp).wrapping_add((((((l_group_10).wrapping_mul(4i32)).wrapping_add({ let a_ = ((c.m.r32(sym::gHudPrompt)).wrapping_add(128)).wrapping_add((((l_side_3) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(12)); c.m.r8(a_) }) as i32); let a_ = (((c.m.r32(sym::gHudPrompt)).wrapping_add(128)).wrapping_add(8)).wrapping_add((((l_side_3) as u32)).wrapping_mul(24)); c.m.w32(a_, (v_) as u32); };
            break 's1;
            }
            F::Ramp_Start(c, l_ramp_9, F32(0x3e8f5c28), F32(0x00000000), F32(0x3f800000));
            F::Ramp_Start(c, l_pulse_6, F32(0x3dcccccc), F32(0x3f800000), F32(0x00000000));
            F::Ramp_Start(c, l_blink_7, F32(0x3fc00000), F32(0x3f800000), F32(0x00000000));
            { let v_ = 8i32; let a_ = l_state_8; c.m.w32(a_, (v_) as u32); };
            { let v_ = (({ let a_ = ({ let a_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(16); c.m.r32(a_) }).wrapping_add(((({ let a_ = (fp).wrapping_add((((((l_group_10).wrapping_mul(4i32)).wrapping_add({ let a_ = ((c.m.r32(sym::gHudPrompt)).wrapping_add(128)).wrapping_add((((l_side_3) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(12)); c.m.r8(a_) }) as i32); let a_ = (((c.m.r32(sym::gHudPrompt)).wrapping_add(128)).wrapping_add(8)).wrapping_add((((l_side_3) as u32)).wrapping_mul(24)); c.m.w32(a_, (v_) as u32); };
            { let v_ = 0i32; let a_ = ((((c.m.r32(sym::gHudPrompt)).wrapping_add(128)).wrapping_add(8)).wrapping_add((((l_side_3) as u32)).wrapping_mul(24))).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
            { let v_ = 0i32; let a_ = ((((c.m.r32(sym::gHudPrompt)).wrapping_add(128)).wrapping_add(8)).wrapping_add((((l_side_3) as u32)).wrapping_mul(24))).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
            { let v_ = 1i32; let a_ = ((((c.m.r32(sym::gHudPrompt)).wrapping_add(128)).wrapping_add(8)).wrapping_add((((l_side_3) as u32)).wrapping_mul(24))).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
            }
            { let v_ = (({ let a_ = ({ let a_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(16); c.m.r32(a_) }).wrapping_add(((({ let a_ = (fp).wrapping_add((((((l_group_10).wrapping_mul(4i32)).wrapping_add({ let a_ = ((c.m.r32(sym::gHudPrompt)).wrapping_add(128)).wrapping_add((((l_side_3) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(12)); c.m.r8(a_) }) as i32); let a_ = (((c.m.r32(sym::gHudPrompt)).wrapping_add(128)).wrapping_add(8)).wrapping_add((((l_side_3) as u32)).wrapping_mul(24)); c.m.w32(a_, (v_) as u32); };
            if ((F::hud_e_b__HudPrompt_StepRamp(c, l_ramp_9)) != 0) {
                { let v_ = 9i32; let a_ = l_state_8; c.m.w32(a_, (v_) as u32); };
            }
            l_alpha_2 = F32(c.m.r32((l_ramp_9).wrapping_add(16)));
            break 's1;
            }
            { let v_ = (({ let a_ = ({ let a_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(16); c.m.r32(a_) }).wrapping_add(((({ let a_ = (fp).wrapping_add((((((l_group_10).wrapping_mul(4i32)).wrapping_add({ let a_ = ((c.m.r32(sym::gHudPrompt)).wrapping_add(128)).wrapping_add((((l_side_3) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(12)); c.m.r8(a_) }) as i32); let a_ = (((c.m.r32(sym::gHudPrompt)).wrapping_add(128)).wrapping_add(8)).wrapping_add((((l_side_3) as u32)).wrapping_mul(24)); c.m.w32(a_, (v_) as u32); };
            if ((((!(((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0)) as i32)) != 0) {
                if ((F::Ramp_Step(c, l_blink_7)) != 0) {
                    F::Ramp_Start(c, l_blink_7, F32(0x3fc00000), F32(0x3f800000), F32(0x00000000));
                }
            }
            {
                l_drop_14 = rt::fsub(rt::fmul(F32(c.m.r32((l_blink_7).wrapping_add(16))), F32(0x40266666)), F32(0x3f4ccccc));
                if (((rt::flt(F32(0x3f666666), l_drop_14) as i32)) != 0) {
                    { let v_ = 1i32; let a_ = ((((c.m.r32(sym::gHudPrompt)).wrapping_add(128)).wrapping_add(8)).wrapping_add((((l_side_3) as u32)).wrapping_mul(24))).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                } else {
                    { let v_ = 0i32; let a_ = ((((c.m.r32(sym::gHudPrompt)).wrapping_add(128)).wrapping_add(8)).wrapping_add((((l_side_3) as u32)).wrapping_mul(24))).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                }
                l_drop_14 = (if (((rt::flt(l_drop_14, F32(0x3f800000)) as i32)) != 0) { l_drop_14 } else { F32(0x3f800000) });
                l_d_15 = (if (((rt::flt(F32(0x00000000), l_drop_14) as i32)) != 0) { l_drop_14 } else { F32(0x00000000) });
                F::HudSprite_Move(c, l_glow_11, 0i32, rt::f2i(rt::fmul(rt::fsub(F32(0x3f800000), l_d_15), F32(0xc1800000))));
            }
            break 's1;
            }
            F::HudSprite_Show(c, l_glow_11, 0i32);
            F::Ramp_Start(c, l_ramp_9, F32(0x3e8f5c28), F32(0x00000000), F32(0x3f800000));
            F::Ramp_Start(c, l_pulse_6, F32(0x3dcccccc), F32(0x3f800000), F32(0x00000000));
            { let v_ = (({ let a_ = ((c.m.r32(sym::gHudPrompt)).wrapping_add(128)).wrapping_add((((l_side_3) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as i16); let a_ = (l_icon_12).wrapping_add(4); c.m.w16(a_, (v_) as u16); };
            { let v_ = 11i32; let a_ = l_state_8; c.m.w32(a_, (v_) as u32); };
            }
            F::HudSprite_Show(c, l_glow_11, 0i32);
            if ((F::hud_e_b__HudPrompt_StepRamp(c, l_ramp_9)) != 0) {
                { let a_ = ((c.m.r32(sym::gHudPrompt)).wrapping_add(128)).wrapping_add((((l_side_3) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = (({ let a_ = ((c.m.r32(sym::gHudPrompt)).wrapping_add(128)).wrapping_add((((l_side_3) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as i16); let a_ = (l_icon_12).wrapping_add(4); c.m.w16(a_, (v_) as u16); };
                { let v_ = 12i32; let a_ = l_state_8; c.m.w32(a_, (v_) as u32); };
            }
            l_alpha_2 = F32(c.m.r32((l_ramp_9).wrapping_add(16)));
            break 's1;
            }
            F::Ramp_Start(c, l_ramp_9, F32(0x3cf5c28f), F32(0x00000000), F32(0x3f800000));
            { let a_ = l_state_8; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
            }
            F::HudSprite_Show(c, l_glow_11, 0i32);
            if ((F::hud_e_b__HudPrompt_StepRamp(c, l_ramp_9)) != 0) {
                { let a_ = ((c.m.r32(sym::gHudPrompt)).wrapping_add(128)).wrapping_add((((l_side_3) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = (({ let a_ = ((c.m.r32(sym::gHudPrompt)).wrapping_add(128)).wrapping_add((((l_side_3) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as i16); let a_ = (l_icon_12).wrapping_add(4); c.m.w16(a_, (v_) as u16); };
                { let a_ = l_state_8; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                if ((((((c.m.r32(l_state_8) as i32)) == (20i32)) as i32)) != 0) {
                    { let v_ = 1i32; let a_ = (((c.m.r32(sym::gHudPrompt)).wrapping_add(128)).wrapping_add(56)).wrapping_add((((l_side_3) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                }
            }
            break 's1;
            }
            F::Ramp_Start(c, l_ramp_9, F32(0x3cf5c28f), F32(0x00000000), F32(0x3f800000));
            { let a_ = l_state_8; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
            }
            F::HudSprite_Show(c, l_glow_11, 0i32);
            if ((F::hud_e_b__HudPrompt_StepRamp(c, l_ramp_9)) != 0) {
                { let a_ = ((c.m.r32(sym::gHudPrompt)).wrapping_add(128)).wrapping_add((((l_side_3) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = (({ let a_ = ((c.m.r32(sym::gHudPrompt)).wrapping_add(128)).wrapping_add((((l_side_3) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as i16); let a_ = (l_icon_12).wrapping_add(4); c.m.w16(a_, (v_) as u16); };
                { let a_ = l_state_8; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                if ((((((c.m.r32(l_state_8) as i32)) == (28i32)) as i32)) != 0) {
                    { let v_ = 12i32; let a_ = l_state_8; c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = (((c.m.r32(sym::gHudPrompt)).wrapping_add(128)).wrapping_add(56)).wrapping_add((((l_side_3) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                }
            }
            break 's1;
            }
            F::Ramp_Start(c, l_ramp_9, F32(0x3e8f5c28), F32(0x3f800000), F32(0x00000000));
            { let a_ = l_state_8; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
            }
            F::HudSprite_Show(c, l_glow_11, 0i32);
            if ((F::hud_e_b__HudPrompt_StepRamp(c, l_ramp_9)) != 0) {
                { let v_ = 42i32; let a_ = l_state_8; c.m.w32(a_, (v_) as u32); };
            }
            l_alpha_2 = F32(c.m.r32((l_ramp_9).wrapping_add(16)));
            break 's1;
            }
            F::HudNode_Show(c, p_node_1, 0i32);
            F::HudSprite_Show(c, l_icon_12, 0i32);
            F::HudSprite_Show(c, l_icon2_5, 0i32);
            F::HudSprite_Show(c, l_base_4, 0i32);
            F::HudSprite_Show(c, l_glow_11, 0i32);
            { let v_ = 13i32; let a_ = (((c.m.r32(sym::gHudPrompt)).wrapping_add(128)).wrapping_add(64)).wrapping_add((((l_side_3) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            { let v_ = 0i32; let a_ = l_state_8; c.m.w32(a_, (v_) as u32); };
            break 's1;
            }
            F::HudNode_Show(c, p_node_1, 0i32);
            F::HudSprite_Show(c, l_icon_12, 0i32);
            F::HudSprite_Show(c, l_icon2_5, 0i32);
            F::HudSprite_Show(c, l_base_4, 0i32);
            F::HudSprite_Show(c, l_glow_11, 0i32);
            { let v_ = 13i32; let a_ = (((c.m.r32(sym::gHudPrompt)).wrapping_add(128)).wrapping_add(64)).wrapping_add((((l_side_3) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            { let v_ = 0i32; let a_ = l_state_8; c.m.w32(a_, (v_) as u32); };
            break 's1;
        }
        if ((((({ let a_ = (((c.m.r32(sym::gHudPrompt)).wrapping_add(128)).wrapping_add(64)).wrapping_add((((l_side_3) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) != (12i32)) as i32)) != 0) {
            { let a1_ = ({ let a_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(16); c.m.r32(a_) }).wrapping_add(((({ let a_ = (((c.m.r32(sym::gHudPrompt)).wrapping_add(128)).wrapping_add(8)).wrapping_add((((l_side_3) as u32)).wrapping_mul(24)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(12)); let a2_ = (((c.m.r32(sym::gHudPrompt)).wrapping_add(128)).wrapping_add(8)).wrapping_add((((l_side_3) as u32)).wrapping_mul(24)); F::HudPrompt_ApplyIconDef(c, l_icon_12, a1_, a2_) };
        } else {
            if (((((l_group_10) != (1i32)) as i32)) != 0) {
                { let a1_ = { let a_ = c.m.r32(sym::gHudPrompt); c.m.r32(a_) }; let a2_ = { let a_ = ((c.m.r32(sym::gHudPrompt)).wrapping_add(128)).wrapping_add((((l_side_3) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; F::HudSprite_InitTex(c, l_icon_12, a1_, a2_, 0i32) };
                F::HudSprite_Center(c, l_icon_12);
            } else {
                { let a1_ = { let a_ = c.m.r32(sym::gHudPrompt); c.m.r32(a_) }; let a2_ = ({ let a_ = ((c.m.r32(sym::gHudPrompt)).wrapping_add(128)).wrapping_add((((l_side_3) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_shr((8i32) as u32); F::HudSprite_InitTex(c, l_icon_12, a1_, a2_, 0i32) };
                F::HudSprite_Center(c, l_icon_12);
                { let a1_ = (if ((((({ let a_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(20); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) { (14i32).wrapping_neg() } else { 14i32 }); F::HudSprite_Move(c, l_icon_12, a1_, 0i32) };
                F::HudSprite_Show(c, l_icon_12, 1i32);
                { let a1_ = { let a_ = c.m.r32(sym::gHudPrompt); c.m.r32(a_) }; let a2_ = (((({ let a_ = ((c.m.r32(sym::gHudPrompt)).wrapping_add(128)).wrapping_add((((l_side_3) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u8)) as i32); F::HudSprite_InitTex(c, l_icon2_5, a1_, a2_, 0i32) };
                F::HudSprite_Center(c, l_icon2_5);
                { let a1_ = (if (({ let a_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(20); (c.m.r32(a_) as i32) }) != 0) { (12i32).wrapping_neg() } else { 12i32 }); F::HudSprite_Move(c, l_icon2_5, a1_, 0i32) };
                F::HudSprite_Show(c, l_icon2_5, 1i32);
            }
        }
        if ((F::hud_e_b__HudPrompt_StepRamp(c, l_pulse_6)) != 0) {
            if (((rt::feq(F32(c.m.r32((l_pulse_6).wrapping_add(16))), F32(0x3f800000)) as i32)) != 0) {
                F::Ramp_Start(c, l_pulse_6, F32(0x3dcccccc), F32(0x3f800000), F32(0x00000000));
            } else {
                if (((rt::feq(F32(c.m.r32((l_pulse_6).wrapping_add(16))), F32(0x00000000)) as i32)) != 0) {
                    F::Ramp_Start(c, l_pulse_6, F32(0x3dcccccc), F32(0x00000000), F32(0x3f800000));
                }
            }
        }
        l_t_13 = rt::fadd(rt::fmul(F32(c.m.r32((l_pulse_6).wrapping_add(16))), F32(0x3ecccccc)), F32(0x3f999999));
        if ((((({ let a_ = (((c.m.r32(sym::gHudPrompt)).wrapping_add(128)).wrapping_add(64)).wrapping_add((((l_side_3) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) == (12i32)) as i32)) != 0) {
            { let r_ = F32(0x3f999999); let n_ = rt::fmul(l_t_13, r_); l_t_13 = n_; };
        }
        F::HudSprite_Scale(c, l_base_4, l_t_13, l_t_13);
        l_t_13 = (if ((((({ let a_ = (((c.m.r32(sym::gHudPrompt)).wrapping_add(128)).wrapping_add(64)).wrapping_add((((l_side_3) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) != (12i32)) as i32)) != 0) { F32(0x3fb851eb) } else { F32(0x3f800000) });
        F::HudSprite_Scale(c, l_icon_12, l_t_13, l_t_13);
        F::HudSprite_SetColor(c, l_icon_12, 128i32, 128i32, 128i32, ((((rt::f2i(rt::fmul(l_alpha_2, F32(0x43000000)))) as u8)) as i32));
        F::HudSprite_SetColor(c, l_icon2_5, 128i32, 128i32, 128i32, ((((rt::f2i(rt::fmul(l_alpha_2, F32(0x43000000)))) as u8)) as i32));
        F::HudSprite_SetColor(c, l_base_4, 128i32, 128i32, 128i32, ((((rt::f2i(rt::fmul(l_alpha_2, F32(0x43000000)))) as u8)) as i32));
        {
            l_drop_16 = rt::fsub(rt::fmul(F32(c.m.r32((l_blink_7).wrapping_add(16))), F32(0x40266666)), F32(0x3f4ccccc));
            if (((rt::flt(l_drop_16, F32(0x3f800000)) as i32)) != 0) {
                { let r_ = l_drop_16; let n_ = rt::fmul(l_alpha_2, r_); l_alpha_2 = n_; };
            }
            l_a_17 = rt::fmul(l_alpha_2, (if (((rt::flt(F32(0x00000000), l_alpha_2) as i32)) != 0) { l_alpha_2 } else { F32(0x00000000) }));
            F::HudSprite_SetColor(c, l_glow_11, 128i32, 128i32, 128i32, ((((rt::f2i(rt::fmul(l_a_17, F32(0x43000000)))) as u8)) as i32));
        }
        { let a1_ = (({ let a_ = (((c.m.r32(sym::gHudPrompt)).wrapping_add(128)).wrapping_add(56)).wrapping_add((((l_side_3) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) ^ (l_side_3)); F::HudSprite_SetMirror(c, l_icon_12, a1_) };
        { let a1_ = (({ let a_ = (((c.m.r32(sym::gHudPrompt)).wrapping_add(128)).wrapping_add(56)).wrapping_add((((l_side_3) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) ^ (l_side_3)); F::HudSprite_SetMirror(c, l_icon2_5, a1_) };
        F::HudSprite_Move(c, l_base_4, 1i32, (2i32).wrapping_neg());
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "HudPrompt_UpdateButton", &PROBE_N, &[], true); }
    r_
}

pub fn HudPrompt_UpdateCue(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudPrompt_UpdateCue", &PROBE_N, &[], false); }
    let mut l_ramp_1: u32 = 0;
    let mut l_state_2: u32 = 0;
    let mut l_alpha_3: F32 = F32(0);
    let mut l_side_4: i32 = 0;
    let mut l_icon_5: u32 = 0;
    let mut l_base_6: u32 = 0;
    let mut l_status_7: u32 = 0;
    let mut l_type_8: i32 = 0;
    let mut l_pulse_9: u32 = 0;
    let mut l_def_10: u32 = 0;
    let mut l_s_11: F32 = F32(0);
    let r_ = 'f: {
    l_ramp_1 = (c.m.r32(sym::gHudPrompt)).wrapping_add(660);
    l_state_2 = (c.m.r32(sym::gHudPrompt)).wrapping_add(656);
    l_alpha_3 = F32(0x3f800000);
    l_side_4 = { let a_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(20); (c.m.r32(a_) as i32) };
    l_icon_5 = ({ let a_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((5i32) as u32)).wrapping_mul(28));
    l_base_6 = ({ let a_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((4i32) as u32)).wrapping_mul(28));
    l_status_7 = ((sym::gPad).wrapping_add((((l_side_4) as u32)).wrapping_mul(448))).wrapping_add(384);
    l_type_8 = (c.m.r32((l_status_7).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32);
    l_pulse_9 = (c.m.r32(sym::gHudPrompt)).wrapping_add(684);
    if (((((l_type_8) >= (4i32)) as i32)) != 0) {
        l_type_8 = (c.m.r32((l_status_7).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32);
    }
    if (((((l_side_4) != (0i32)) as i32)) != 0) {
        break 'f;
    }
    l_def_10 = (sym::gHudPromptCueDefs).wrapping_add((((l_type_8) as u32)).wrapping_mul(12));
    { let a1_ = { let a_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(12); c.m.r32(a_) }; let a2_ = ((c.m.r8(l_def_10)) as i32); F::HudSprite_InitTex(c, l_icon_5, a1_, a2_, 0i32) };
    { let a2_ = (if ((((((((c.m.r8((l_def_10).wrapping_add(2))) as i32)).wrapping_add(1i32)) < (((c.m.r8((l_def_10).wrapping_add(1))) as i32))) as i32)) != 0) { (((c.m.r8((l_def_10).wrapping_add(2))) as i32)).wrapping_add(1i32) } else { ((c.m.r8((l_def_10).wrapping_add(1))) as i32) }); let a4_ = ((c.m.r8((l_def_10).wrapping_add(1))) as i32); F::HudSprite_SetRect(c, l_icon_5, 0i32, a2_, 0i32, a4_) };
    { let a2_ = (if ((((((((c.m.r8((l_def_10).wrapping_add(2))) as i32)).wrapping_add(1i32)) < (((c.m.r8((l_def_10).wrapping_add(1))) as i32))) as i32)) != 0) { (((c.m.r8((l_def_10).wrapping_add(2))) as i32)).wrapping_add(1i32) } else { ((c.m.r8((l_def_10).wrapping_add(1))) as i32) }); let a4_ = ((c.m.r8((l_def_10).wrapping_add(1))) as i32); F::HudSprite_SetUv(c, l_icon_5, 0i32, a2_, 0i32, a4_) };
    F::HudSprite_Center(c, l_icon_5);
    { let a1_ = { let a_ = c.m.r32(sym::gHudPrompt); c.m.r32(a_) }; F::HudSprite_InitTex(c, l_base_6, a1_, 8i32, 0i32) };
    F::HudSprite_Center(c, l_base_6);
    F::HudSprite_Show(c, l_icon_5, 1i32);
    F::HudSprite_Show(c, l_base_6, 1i32);
    's1: {
        let sv12: i32 = (c.m.r32(l_state_2) as i32);
        'k6: {
        'k5: {
        'k4: {
        'k3: {
        'k2: {
            match sv12 { 0i32 => break 'k2, 1i32 => break 'k3, 2i32 => break 'k4, 3i32 => break 'k5, 4i32 => break 'k6, _ => break 'k6, }
        }
        F::Ramp_Start(c, l_ramp_1, F32(0x3e8f5c28), F32(0x00000000), F32(0x3f800000));
        F::Ramp_Start(c, l_pulse_9, F32(0x3dcccccc), F32(0x3f800000), F32(0x00000000));
        { let v_ = 1i32; let a_ = l_state_2; c.m.w32(a_, (v_) as u32); };
        }
        if ((F::hud_e_b__HudPrompt_StepRamp(c, l_ramp_1)) != 0) {
            { let v_ = 2i32; let a_ = l_state_2; c.m.w32(a_, (v_) as u32); };
        }
        l_alpha_3 = F32(c.m.r32((l_ramp_1).wrapping_add(16)));
        break 's1;
        }
        break 's1;
        }
        if ((F::hud_e_b__HudPrompt_StepRamp(c, l_ramp_1)) != 0) {
            { let v_ = 4i32; let a_ = l_state_2; c.m.w32(a_, (v_) as u32); };
        }
        l_alpha_3 = F32(c.m.r32((l_ramp_1).wrapping_add(16)));
        break 's1;
        }
        F::HudSprite_Show(c, l_icon_5, 0i32);
        F::HudSprite_Show(c, l_base_6, 0i32);
        { let v_ = 4i32; let a_ = l_state_2; c.m.w32(a_, (v_) as u32); };
        break 's1;
    }
    if ((F::hud_e_b__HudPrompt_StepRamp(c, l_pulse_9)) != 0) {
        if (((rt::feq(F32(c.m.r32((l_pulse_9).wrapping_add(16))), F32(0x3f800000)) as i32)) != 0) {
            F::Ramp_Start(c, l_pulse_9, F32(0x3dcccccc), F32(0x3f800000), F32(0x00000000));
        } else {
            if (((rt::feq(F32(c.m.r32((l_pulse_9).wrapping_add(16))), F32(0x00000000)) as i32)) != 0) {
                F::Ramp_Start(c, l_pulse_9, F32(0x3dcccccc), F32(0x00000000), F32(0x3f800000));
            }
        }
    }
    if (({ let a_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(708); (c.m.r32(a_) as i32) }) != 0) {
        l_s_11 = F32(0x3f999999);
    } else {
        l_s_11 = rt::fmul(rt::fadd(rt::fmul(F32(c.m.r32((l_pulse_9).wrapping_add(16))), F32(0x3ecccccc)), F32(0x3f999999)), F32(0x3f999999));
    }
    F::HudSprite_Scale(c, l_base_6, l_s_11, l_s_11);
    F::HudSprite_Scale(c, l_icon_5, F32(0x3f999999), F32(0x3f999999));
    F::HudSprite_Move(c, l_icon_5, 2i32, 0i32);
    if (({ let a_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(708); (c.m.r32(a_) as i32) }) != 0) {
        { let r_ = F32(0x3f000000); let n_ = rt::fmul(l_alpha_3, r_); l_alpha_3 = n_; };
    }
    F::HudSprite_SetColor(c, l_icon_5, 128i32, 128i32, 128i32, ((((rt::f2i(rt::fmul(l_alpha_3, F32(0x43000000)))) as u8)) as i32));
    F::HudSprite_SetColor(c, l_base_6, 128i32, 128i32, 128i32, ((((rt::f2i(rt::fmul(l_alpha_3, F32(0x43000000)))) as u8)) as i32));
    F::HudSprite_Move(c, l_base_6, 1i32, (2i32).wrapping_neg());
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudPrompt_UpdateCue", &PROBE_N, &[], true); }
    r_
}

pub fn HudPrompt_DrawButton(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudPrompt_DrawButton", &PROBE_N, &[], false); }
    let mut l_icon_1: u32 = 0;
    let r_ = 'f: {
    F::HudPrompt_ClearIconMarks(c);
    { let a0_ = ({ let a_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((0i32) as u32)).wrapping_mul(28)); let a1_ = { let a_ = c.m.r32(sym::gHudPrompt); c.m.r32(a_) }; F::HudSprite_Draw(c, a0_, a1_, 0i32) };
    l_icon_1 = ({ let a_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((1i32) as u32)).wrapping_mul(28));
    if ((((({ let a_ = (((c.m.r32(sym::gHudPrompt)).wrapping_add(128)).wrapping_add(64)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(20); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) != (12i32)) as i32)) != 0) {
        { let a1_ = { let a_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(12); c.m.r32(a_) }; F::HudSprite_DrawAt(c, l_icon_1, a1_, ((((0i32) as i64 as u64)) as u8), 10880i32, 11408i32) };
    } else {
        { let a1_ = { let a_ = c.m.r32(sym::gHudPrompt); c.m.r32(a_) }; F::HudSprite_Draw(c, l_icon_1, a1_, 0i32) };
    }
    { let a0_ = ({ let a_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((2i32) as u32)).wrapping_mul(28)); let a1_ = { let a_ = c.m.r32(sym::gHudPrompt); c.m.r32(a_) }; F::HudSprite_Draw(c, a0_, a1_, 0i32) };
    { let a0_ = ({ let a_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((3i32) as u32)).wrapping_mul(28)); let a1_ = { let a_ = c.m.r32(sym::gHudPrompt); c.m.r32(a_) }; F::HudSprite_Draw(c, a0_, a1_, 0i32) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudPrompt_DrawButton", &PROBE_N, &[], true); }
    r_
}

pub fn HudPrompt_DrawCue(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudPrompt_DrawCue", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if ((((({ let a_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(20); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
        F::HudPrompt_ClearIconMarks(c);
        { let a0_ = ({ let a_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((4i32) as u32)).wrapping_mul(28)); let a1_ = { let a_ = c.m.r32(sym::gHudPrompt); c.m.r32(a_) }; F::HudSprite_Draw(c, a0_, a1_, 0i32) };
        { let a0_ = ({ let a_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((5i32) as u32)).wrapping_mul(28)); let a1_ = { let a_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(12); c.m.r32(a_) }; F::HudSprite_DrawAt(c, a0_, a1_, ((((0i32) as i64 as u64)) as u8), 10880i32, 11408i32) };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudPrompt_DrawCue", &PROBE_N, &[], true); }
    r_
}

pub fn HudPrompt_UpdateCommand(c: &mut Ctx, mut p_node_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudPrompt_UpdateCommand", &PROBE_N, &[(p_node_1) as u32], false); }
    let mut l_side_2: i32 = 0;
    let mut l_spr_3: u32 = 0;
    let mut l_icons_4: u32 = 0;
    let mut l_ramp_5: u32 = 0;
    let mut l_name_6: u32 = 0;
    let mut l_blink_7: u32 = 0;
    let mut l_ghostGlow_8: u32 = 0;
    let mut l_ghost_9: u32 = 0;
    let mut l_x_10: i32 = 0;
    let mut l_state_11: u32 = 0;
    let mut l_count_12: i32 = 0;
    let mut l_glow_13: u32 = 0;
    let mut l_icon_14: u32 = 0;
    let mut l_drop_15: F32 = F32(0);
    let mut l_i_16: i32 = 0;
    let mut l_s_17: F32 = F32(0);
    let r_ = 'f: {
    l_side_2 = { let a_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(20); (c.m.r32(a_) as i32) };
    l_spr_3 = { let a_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(4); c.m.r32(a_) };
    l_x_10 = 0i32;
    l_icons_4 = (((c.m.r32(sym::gHudPrompt)).wrapping_add(288)).wrapping_add(8)).wrapping_add((((l_side_2) as u32)).wrapping_mul(96));
    l_ramp_5 = ((c.m.r32(sym::gHudPrompt)).wrapping_add(240)).wrapping_add((((l_side_2) as u32)).wrapping_mul(24));
    l_name_6 = ((c.m.r32(sym::gHudPrompt)).wrapping_add(608)).wrapping_add((((l_side_2) as u32)).wrapping_mul(24));
    l_blink_7 = ((c.m.r32(sym::gHudPrompt)).wrapping_add(560)).wrapping_add((((l_side_2) as u32)).wrapping_mul(24));
    l_state_11 = (((c.m.r32(sym::gHudPrompt)).wrapping_add(224)).wrapping_add(8)).wrapping_add((((l_side_2) as u32)).wrapping_mul(4));
    l_glow_13 = (l_spr_3).wrapping_add((((10i32) as u32)).wrapping_mul(28));
    l_count_12 = { let a_ = ((c.m.r32(sym::gHudPrompt)).wrapping_add(288)).wrapping_add((((l_side_2) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
    l_ghostGlow_8 = (l_spr_3).wrapping_add((((18i32) as u32)).wrapping_mul(28));
    l_icon_14 = (l_spr_3).wrapping_add((((6i32) as u32)).wrapping_mul(28));
    l_ghost_9 = (l_spr_3).wrapping_add((((14i32) as u32)).wrapping_mul(28));
    {
        l_i_16 = 0i32;
        'l1: loop {
            if !(((((l_i_16) < (4i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let a1_ = { let a_ = c.m.r32(sym::gHudPrompt); c.m.r32(a_) }; F::HudSprite_InitTex(c, (l_glow_13).wrapping_add((((l_i_16) as u32)).wrapping_mul(28)), a1_, 7i32, 0i32) };
                F::HudSprite_Center(c, (l_glow_13).wrapping_add((((l_i_16) as u32)).wrapping_mul(28)));
                F::HudSprite_Move(c, (l_glow_13).wrapping_add((((l_i_16) as u32)).wrapping_mul(28)), 0i32, (22i32).wrapping_neg());
            }
            { let o_ = l_i_16; let n_ = (o_).wrapping_add(1); l_i_16 = n_; };
        }
    }
    {
        l_i_16 = 0i32;
        'l3: loop {
            if !(((((l_i_16) < (4i32)) as i32)) != 0) { break 'l3; }
            'c4: {
                if (((((l_i_16) < (l_count_12)) as i32)) != 0) {
                    F::HudSprite_Show(c, (l_icon_14).wrapping_add((((l_i_16) as u32)).wrapping_mul(28)), 1i32);
                    if ((((((c.m.r32((l_icons_4).wrapping_add((((l_i_16) as u32)).wrapping_mul(24))) as i32)) < (16i32)) as i32)) != 0) {
                        if ((((((((c.m.r32(((l_icons_4).wrapping_add((((l_i_16) as u32)).wrapping_mul(24))).wrapping_add(12)) as i32)) as u32)) < (((2i32) as u32))) as i32)) != 0) {
                            F::HudSprite_Show(c, (l_glow_13).wrapping_add((((l_i_16) as u32)).wrapping_mul(28)), 1i32);
                        } else {
                            F::HudSprite_Show(c, (l_glow_13).wrapping_add((((l_i_16) as u32)).wrapping_mul(28)), 0i32);
                        }
                    } else {
                        F::HudSprite_Show(c, (l_glow_13).wrapping_add((((l_i_16) as u32)).wrapping_mul(28)), 0i32);
                    }
                } else {
                    F::HudSprite_Show(c, (l_icon_14).wrapping_add((((l_i_16) as u32)).wrapping_mul(28)), 0i32);
                    F::HudSprite_Show(c, (l_glow_13).wrapping_add((((l_i_16) as u32)).wrapping_mul(28)), 0i32);
                    F::HudSprite_Show(c, (l_ghost_9).wrapping_add((((l_i_16) as u32)).wrapping_mul(28)), 0i32);
                    F::HudSprite_Show(c, (l_ghostGlow_8).wrapping_add((((l_i_16) as u32)).wrapping_mul(28)), 0i32);
                }
            }
            { let o_ = l_i_16; let n_ = (o_).wrapping_add(1); l_i_16 = n_; };
        }
    }
    's5: {
        let sv18: i32 = (c.m.r32(l_state_11) as i32);
        'k14: {
        'k13: {
        'k12: {
        'k11: {
        'k10: {
        'k9: {
        'k8: {
        'k7: {
        'k6: {
            match sv18 { 0i32 => break 'k6, 1i32 => break 'k7, 2i32 => break 'k8, 3i32 => break 'k9, 5i32 => break 'k10, 6i32 => break 'k11, 4i32 => break 'k12, 7i32 => break 'k12, 9i32 => break 'k13, 8i32 => break 'k14, _ => break 'k14, }
        }
        {
            l_i_16 = 0i32;
            'l15: loop {
                if !(((((l_i_16) < (4i32)) as i32)) != 0) { break 'l15; }
                'c16: {
                    if (((((l_i_16) < (l_count_12)) as i32)) != 0) {
                        { let v_ = 0i32; let a_ = ((l_icons_4).wrapping_add((((l_i_16) as u32)).wrapping_mul(24))).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                        { let v_ = 0i32; let a_ = ((l_icons_4).wrapping_add((((l_i_16) as u32)).wrapping_mul(24))).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                        F::HudSprite_Show(c, (l_icon_14).wrapping_add((((l_i_16) as u32)).wrapping_mul(28)), 1i32);
                        if ((((((c.m.r32((l_icons_4).wrapping_add((((l_i_16) as u32)).wrapping_mul(24))) as i32)) < (16i32)) as i32)) != 0) {
                            if ((((((((c.m.r32(((l_icons_4).wrapping_add((((l_i_16) as u32)).wrapping_mul(24))).wrapping_add(12)) as i32)) as u32)) < (((2i32) as u32))) as i32)) != 0) {
                                F::HudSprite_Show(c, (l_glow_13).wrapping_add((((l_i_16) as u32)).wrapping_mul(28)), 1i32);
                            } else {
                                F::HudSprite_Show(c, (l_glow_13).wrapping_add((((l_i_16) as u32)).wrapping_mul(28)), 0i32);
                            }
                        }
                    } else {
                        F::HudSprite_Show(c, (l_icon_14).wrapping_add((((l_i_16) as u32)).wrapping_mul(28)), 0i32);
                        F::HudSprite_Show(c, (l_glow_13).wrapping_add((((l_i_16) as u32)).wrapping_mul(28)), 0i32);
                    }
                }
                { let o_ = l_i_16; let n_ = (o_).wrapping_add(1); l_i_16 = n_; };
            }
        }
        F::Ramp_Start(c, l_ramp_5, F32(0x3dcccccc), F32(0x00000000), F32(0x3f800000));
        F::Ramp_Start(c, l_name_6, F32(0x3dcccccc), F32(0x00000000), F32(0x3f800000));
        F::Ramp_Start(c, l_blink_7, F32(0x3fc00000), F32(0x3f800000), F32(0x00000000));
        { let a_ = l_state_11; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        }
        if ((((!(((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0)) as i32)) != 0) {
            if ((F::Ramp_Step(c, l_ramp_5)) != 0) {
                { let a_ = l_state_11; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
            }
        }
        break 's5;
        }
        if ((((!(((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0)) as i32)) != 0) {
            if ((F::Ramp_Step(c, l_blink_7)) != 0) {
                F::Ramp_Start(c, l_blink_7, F32(0x3fc00000), F32(0x3f800000), F32(0x00000000));
            }
        }
        if ((((!(((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0)) as i32)) != 0) {
            {
                l_i_16 = 0i32;
                'l17: loop {
                    if !(((((l_i_16) < (l_count_12)) as i32)) != 0) { break 'l17; }
                    'c18: {
                        if ((((((c.m.r32((l_icons_4).wrapping_add((((l_i_16) as u32)).wrapping_mul(24))) as i32)) < (16i32)) as i32)) != 0) {
                            if ((((((c.m.r32(((l_icons_4).wrapping_add((((l_i_16) as u32)).wrapping_mul(24))).wrapping_add(12)) as i32)) == (1i32)) as i32)) != 0) {
                                break 'c18;
                            }
                            if ((((((c.m.r32(((l_icons_4).wrapping_add((((l_i_16) as u32)).wrapping_mul(24))).wrapping_add(12)) as i32)) == (0i32)) as i32)) != 0) {
                                break 'c18;
                            }
                        }
                        { let a_ = ((l_icons_4).wrapping_add((((l_i_16) as u32)).wrapping_mul(24))).wrapping_add(8); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    }
                    { let o_ = l_i_16; let n_ = (o_).wrapping_add(1); l_i_16 = n_; };
                }
            }
        }
        break 's5;
        }
        F::Ramp_Start(c, l_ramp_5, F32(0x3e999999), F32(0x3f800000), F32(0x00000000));
        { let a_ = l_state_11; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        if ((((!(((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0)) as i32)) != 0) {
            if ((F::Ramp_Step(c, l_ramp_5)) != 0) {
                { let v_ = 9i32; let a_ = l_state_11; c.m.w32(a_, (v_) as u32); };
            }
        }
        break 's5;
        }
        F::Ramp_Start(c, l_blink_7, F32(0x3f4ccccc), F32(0x3f800000), F32(0x00000000));
        { let a_ = l_state_11; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        }
        if ((((!(((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0)) as i32)) != 0) {
            if ((F::Ramp_Step(c, l_blink_7)) != 0) {
                F::Ramp_Start(c, l_ramp_5, F32(0x3f000000), F32(0x3f800000), F32(0x00000000));
                { let a_ = l_state_11; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
            }
        }
        break 's5;
        }
        if ((((!(((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0)) as i32)) != 0) {
            if ((F::Ramp_Step(c, l_ramp_5)) != 0) {
                { let v_ = 9i32; let a_ = l_state_11; c.m.w32(a_, (v_) as u32); };
            }
        }
        break 's5;
        }
        { let v_ = (1i32).wrapping_neg(); let a_ = l_state_11; c.m.w32(a_, (v_) as u32); };
        }
        {
            l_i_16 = 0i32;
            'l19: loop {
                if !(((((l_i_16) < (l_count_12)) as i32)) != 0) { break 'l19; }
                'c20: {
                    F::HudSprite_Show(c, (l_icon_14).wrapping_add((((l_i_16) as u32)).wrapping_mul(28)), 0i32);
                }
                { let o_ = l_i_16; let n_ = (o_).wrapping_add(1); l_i_16 = n_; };
            }
        }
        {
            l_i_16 = 0i32;
            'l21: loop {
                if !(((((l_i_16) < (4i32)) as i32)) != 0) { break 'l21; }
                'c22: {
                    F::HudSprite_Show(c, (l_glow_13).wrapping_add((((l_i_16) as u32)).wrapping_mul(28)), 0i32);
                }
                { let o_ = l_i_16; let n_ = (o_).wrapping_add(1); l_i_16 = n_; };
            }
        }
        break 's5;
    }
    l_drop_15 = rt::fsub(rt::fmul(F32(c.m.r32((l_blink_7).wrapping_add(16))), F32(0x40266666)), F32(0x3f4ccccc));
    l_drop_15 = (if (((rt::flt(F32(0x00000000), l_drop_15) as i32)) != 0) { l_drop_15 } else { F32(0x00000000) });
    l_drop_15 = (if (((rt::flt(l_drop_15, F32(0x3f800000)) as i32)) != 0) { l_drop_15 } else { F32(0x3f800000) });
    {
        l_i_16 = 0i32;
        'l23: loop {
            if !(((((l_i_16) < (l_count_12)) as i32)) != 0) { break 'l23; }
            'c24: {
                if ((((((c.m.r32((l_icons_4).wrapping_add((((l_i_16) as u32)).wrapping_mul(24))) as i32)) < (16i32)) as i32)) != 0) {
                    if ((((((c.m.r32(((l_icons_4).wrapping_add((((l_i_16) as u32)).wrapping_mul(24))).wrapping_add(12)) as i32)) == (1i32)) as i32)) != 0) {
                        { let a4_ = ((((rt::f2i(rt::fmul(F32(c.m.r32((l_ramp_5).wrapping_add(16))), F32(0x43000000)))) as u8)) as i32); F::HudSprite_SetColor(c, (l_glow_13).wrapping_add((((l_i_16) as u32)).wrapping_mul(28)), 128i32, 128i32, 128i32, a4_) };
                    } else {
                        if ((((((c.m.r32(((l_icons_4).wrapping_add((((l_i_16) as u32)).wrapping_mul(24))).wrapping_add(12)) as i32)) == (0i32)) as i32)) != 0) {
                            if (((rt::flt(F32(0x3f666666), l_drop_15) as i32)) != 0) {
                                { let v_ = 1i32; let a_ = ((l_icons_4).wrapping_add((((l_i_16) as u32)).wrapping_mul(24))).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                            } else {
                                { let v_ = 0i32; let a_ = ((l_icons_4).wrapping_add((((l_i_16) as u32)).wrapping_mul(24))).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                            }
                            F::HudSprite_Move(c, (l_glow_13).wrapping_add((((l_i_16) as u32)).wrapping_mul(28)), 0i32, rt::f2i(rt::fmul(rt::fsub(F32(0x3f800000), l_drop_15), F32(0xc1800000))));
                            { let a4_ = ((((rt::f2i(rt::fmul(rt::fmul(l_drop_15, F32(c.m.r32((l_ramp_5).wrapping_add(16)))), F32(0x43000000)))) as u8)) as i32); F::HudSprite_SetColor(c, (l_glow_13).wrapping_add((((l_i_16) as u32)).wrapping_mul(28)), 128i32, 128i32, 128i32, a4_) };
                        }
                    }
                }
            }
            { let o_ = l_i_16; let n_ = (o_).wrapping_add(1); l_i_16 = n_; };
        }
    }
    if ((((!(((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0)) as i32)) != 0) {
        { let _ = F::Ramp_Step(c, l_name_6); };
    }
    {
        l_i_16 = 0i32;
        'l25: loop {
            if !(((((l_i_16) < (l_count_12)) as i32)) != 0) { break 'l25; }
            'c26: {
                F::HudPrompt_ApplyIcon(c, (l_icon_14).wrapping_add((((l_i_16) as u32)).wrapping_mul(28)), (l_icons_4).wrapping_add((((l_i_16) as u32)).wrapping_mul(24)));
                if (((((((((c.m.r32(l_state_11) as i32)).wrapping_sub(5i32)) as u32)) < (((2i32) as u32))) as i32)) != 0) {
                    l_s_17 = rt::fadd(rt::fmul(rt::fsub(F32(0x3f800000), F32(c.m.r32((l_blink_7).wrapping_add(16)))), F32(0x3fc00000)), F32(0x3f800000));
                    { let s_ = (l_icon_14).wrapping_add((((l_i_16) as u32)).wrapping_mul(28)); let d_ = (l_ghost_9).wrapping_add((((l_i_16) as u32)).wrapping_mul(28)); rt::memcpy(c, d_, s_, 28); };
                    { let s_ = (l_glow_13).wrapping_add((((l_i_16) as u32)).wrapping_mul(28)); let d_ = (l_ghostGlow_8).wrapping_add((((l_i_16) as u32)).wrapping_mul(28)); rt::memcpy(c, d_, s_, 28); };
                    F::HudSprite_Scale(c, (l_ghost_9).wrapping_add((((l_i_16) as u32)).wrapping_mul(28)), l_s_17, l_s_17);
                    { let a4_ = ((((rt::f2i(rt::fmul(F32(c.m.r32((l_blink_7).wrapping_add(16))), F32(0x43000000)))) as u8)) as i32); F::HudSprite_SetColor(c, (l_ghost_9).wrapping_add((((l_i_16) as u32)).wrapping_mul(28)), 128i32, 128i32, 128i32, a4_) };
                    F::HudSprite_Scale(c, (l_ghostGlow_8).wrapping_add((((l_i_16) as u32)).wrapping_mul(28)), l_s_17, l_s_17);
                    { let a4_ = ((((rt::f2i(rt::fmul(F32(c.m.r32((l_blink_7).wrapping_add(16))), F32(0x43000000)))) as u8)) as i32); F::HudSprite_SetColor(c, (l_ghostGlow_8).wrapping_add((((l_i_16) as u32)).wrapping_mul(28)), 128i32, 128i32, 128i32, a4_) };
                } else {
                    F::HudSprite_Show(c, (l_ghost_9).wrapping_add((((l_i_16) as u32)).wrapping_mul(28)), 0i32);
                    F::HudSprite_Show(c, (l_ghostGlow_8).wrapping_add((((l_i_16) as u32)).wrapping_mul(28)), 0i32);
                }
                F::HudSprite_Move(c, (l_icon_14).wrapping_add((((l_i_16) as u32)).wrapping_mul(28)), l_x_10, 0i32);
                F::HudSprite_Move(c, (l_glow_13).wrapping_add((((l_i_16) as u32)).wrapping_mul(28)), l_x_10, 0i32);
                F::HudSprite_Move(c, (l_ghost_9).wrapping_add((((l_i_16) as u32)).wrapping_mul(28)), l_x_10, 0i32);
                F::HudSprite_Move(c, (l_ghostGlow_8).wrapping_add((((l_i_16) as u32)).wrapping_mul(28)), l_x_10, 0i32);
                if (((((l_i_16) < ((l_count_12).wrapping_sub(1i32))) as i32)) != 0) {
                    { let r_ = ((((c.m.r16((((l_icon_14).wrapping_add((((l_i_16) as u32)).wrapping_mul(28))).wrapping_add(8)).wrapping_add((((1i32) as u32)).wrapping_mul(2))) as i16)) as i32)).wrapping_sub((((c.m.r16((((l_icon_14).wrapping_add((((l_i_16) as u32)).wrapping_mul(28))).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(2))) as i16)) as i32)); let n_ = (l_x_10).wrapping_add(r_); l_x_10 = n_; };
                }
                { let a4_ = ((((rt::f2i(rt::fmul(F32(c.m.r32((l_ramp_5).wrapping_add(16))), F32(0x43000000)))) as u8)) as i32); F::HudSprite_SetColor(c, (l_icon_14).wrapping_add((((l_i_16) as u32)).wrapping_mul(28)), 128i32, 128i32, 128i32, a4_) };
            }
            { let o_ = l_i_16; let n_ = (o_).wrapping_add(1); l_i_16 = n_; };
        }
    }
    {
        l_i_16 = 0i32;
        'l27: loop {
            if !(((((l_i_16) < (l_count_12)) as i32)) != 0) { break 'l27; }
            'c28: {
                if ((((({ let a_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(20); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
                    { let a1_ = (((((c.m.r32((p_node_1).wrapping_add(16)) as i32)).wrapping_mul(2i32)).wrapping_neg()).wrapping_sub(l_x_10)).wrapping_add(512i32); F::HudSprite_Move(c, (l_icon_14).wrapping_add((((l_i_16) as u32)).wrapping_mul(28)), a1_, 0i32) };
                    { let a1_ = (((((c.m.r32((p_node_1).wrapping_add(16)) as i32)).wrapping_mul(2i32)).wrapping_neg()).wrapping_sub(l_x_10)).wrapping_add(512i32); F::HudSprite_Move(c, (l_glow_13).wrapping_add((((l_i_16) as u32)).wrapping_mul(28)), a1_, 0i32) };
                    { let a1_ = (((((c.m.r32((p_node_1).wrapping_add(16)) as i32)).wrapping_mul(2i32)).wrapping_neg()).wrapping_sub(l_x_10)).wrapping_add(512i32); F::HudSprite_Move(c, (l_ghost_9).wrapping_add((((l_i_16) as u32)).wrapping_mul(28)), a1_, 0i32) };
                    { let a1_ = (((((c.m.r32((p_node_1).wrapping_add(16)) as i32)).wrapping_mul(2i32)).wrapping_neg()).wrapping_sub(l_x_10)).wrapping_add(512i32); F::HudSprite_Move(c, (l_ghostGlow_8).wrapping_add((((l_i_16) as u32)).wrapping_mul(28)), a1_, 0i32) };
                }
            }
            { let o_ = l_i_16; let n_ = (o_).wrapping_add(1); l_i_16 = n_; };
        }
    }
    if ((((((c.m.r32(((l_icons_4).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(12)) as i32)) != (3i32)) as i32)) != 0) {
        if ((((({ let a_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(20); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
            { let v_ = (((c.m.r16((((l_icon_14).wrapping_add((((0i32) as u32)).wrapping_mul(28))).wrapping_add(8)).wrapping_add((((1i32) as u32)).wrapping_mul(2))) as i16)) as i32); let a_ = ((c.m.r32(sym::gHudPrompt)).wrapping_add(488)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(20); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(8)); c.m.w32(a_, (v_) as u32); };
        } else {
            { let v_ = (((c.m.r16((((l_icon_14).wrapping_add((((0i32) as u32)).wrapping_mul(28))).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(2))) as i16)) as i32); let a_ = ((c.m.r32(sym::gHudPrompt)).wrapping_add(488)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(20); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(8)); c.m.w32(a_, (v_) as u32); };
        }
        { let v_ = ((((c.m.r16((((l_icon_14).wrapping_add((((0i32) as u32)).wrapping_mul(28))).wrapping_add(8)).wrapping_add((((2i32) as u32)).wrapping_mul(2))) as i16)) as i32)).wrapping_sub(32i32); let a_ = (((c.m.r32(sym::gHudPrompt)).wrapping_add(488)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(20); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(8))).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let r_ = (c.m.r32((p_node_1).wrapping_add(16)) as i32); let a_ = ((c.m.r32(sym::gHudPrompt)).wrapping_add(488)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(20); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(8)); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
        { let r_ = (c.m.r32((p_node_1).wrapping_add(20)) as i32); let a_ = (((c.m.r32(sym::gHudPrompt)).wrapping_add(488)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(20); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(8))).wrapping_add(4); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudPrompt_UpdateCommand", &PROBE_N, &[], true); }
    r_
}

pub fn HudPrompt_DrawCommand(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudPrompt_DrawCommand", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let mut l_spr_2: u32 = 0;
    let r_ = 'f: {
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (4i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                l_spr_2 = ({ let a_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(4); c.m.r32(a_) }).wrapping_add(((((6i32).wrapping_add(l_i_1)) as u32)).wrapping_mul(28));
                { let a0_ = (((c.m.r16((l_spr_2).wrapping_add(4)) as i16)) as i32); F::HudPrompt_ClearIconMark(c, a0_) };
                { let a1_ = { let a_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(12); c.m.r32(a_) }; F::HudSprite_DrawAt(c, l_spr_2, a1_, ((((0i32) as i64 as u64)) as u8), 10880i32, 11408i32) };
                { let a0_ = ({ let a_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(4); c.m.r32(a_) }).wrapping_add(((((10i32).wrapping_add(l_i_1)) as u32)).wrapping_mul(28)); let a1_ = { let a_ = c.m.r32(sym::gHudPrompt); c.m.r32(a_) }; F::HudSprite_Draw(c, a0_, a1_, 0i32) };
                l_spr_2 = ({ let a_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(4); c.m.r32(a_) }).wrapping_add(((((14i32).wrapping_add(l_i_1)) as u32)).wrapping_mul(28));
                { let a0_ = (((c.m.r16((l_spr_2).wrapping_add(4)) as i16)) as i32); F::HudPrompt_ClearIconMark(c, a0_) };
                { let a1_ = { let a_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(12); c.m.r32(a_) }; F::HudSprite_DrawAt(c, l_spr_2, a1_, ((((0i32) as i64 as u64)) as u8), 10880i32, 11408i32) };
                { let a0_ = ({ let a_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(4); c.m.r32(a_) }).wrapping_add(((((18i32).wrapping_add(l_i_1)) as u32)).wrapping_mul(28)); let a1_ = { let a_ = c.m.r32(sym::gHudPrompt); c.m.r32(a_) }; F::HudSprite_Draw(c, a0_, a1_, 0i32) };
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudPrompt_DrawCommand", &PROBE_N, &[], true); }
    r_
}

pub fn HudPrompt_SelectSide(c: &mut Ctx, mut p_side_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudPrompt_SelectSide", &PROBE_N, &[(p_side_1) as u32], false); }
    let mut l_node_2: u32 = 0;
    let r_ = 'f: {
    { let v_ = p_side_1; let a_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
    F::Port_GsMarker(c, (if ((p_side_1) != 0) { 17i32 } else { 16i32 }));
    l_node_2 = ({ let a_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((24i32) as u32)).wrapping_mul(56));
    { let v_ = (((((p_side_1) != (0i32)) as i32)) as u32); let a_ = l_node_2; rt::bf_set(c, a_, 1, 1, (v_) as u64);  };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudPrompt_SelectSide", &PROBE_N, &[], true); }
    r_
}

pub fn HudPrompt_ShowCue(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudPrompt_ShowCue", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if ((((((({ let a_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(656); (c.m.r32(a_) as i32) }) as u32)) >= (((3i32) as u32))) as i32)) != 0) {
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(656); c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudPrompt_ShowCue", &PROBE_N, &[], true); }
    r_
}

pub fn HudPrompt_HideCue(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudPrompt_HideCue", &PROBE_N, &[], false); }
    let mut l_ramp_1: u32 = 0;
    let r_ = 'f: {
    l_ramp_1 = (c.m.r32(sym::gHudPrompt)).wrapping_add(660);
    if ((((((({ let a_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(656); (c.m.r32(a_) as i32) }) as u32)) < (((3i32) as u32))) as i32)) != 0) {
        F::Ramp_Start(c, l_ramp_1, F32(0x3e8f5c28), F32(0x3f800000), F32(0x00000000));
        { let v_ = 3i32; let a_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(656); c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudPrompt_HideCue", &PROBE_N, &[], true); }
    r_
}

pub fn HudPrompt_SetCueDim(c: &mut Ctx, mut p_canAct_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudPrompt_SetCueDim", &PROBE_N, &[(p_canAct_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = (((p_canAct_1) == (0i32)) as i32); let a_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(708); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudPrompt_SetCueDim", &PROBE_N, &[], true); }
    r_
}

pub fn HudPrompt_SetButton(c: &mut Ctx, mut p_side_1: i32, mut p_button_2: i32, mut p_group_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudPrompt_SetButton", &PROBE_N, &[(p_side_1) as u32, (p_button_2) as u32, (p_group_3) as u32], false); }
    let mut l_icon_4: u32 = 0;
    let r_ = 'f: {
    l_icon_4 = ({ let a_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((1i32) as u32)).wrapping_mul(28));
    { let v_ = p_group_3; let a_ = ((c.m.r32(sym::gHudPrompt)).wrapping_add(712)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    if (((((((((((({ let a_ = ((c.m.r32(sym::gHudPrompt)).wrapping_add(24)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_sub(1i32)) as u32)) < (((38i32) as u32))) as i32)) != 0) && ((((({ let a_ = (((c.m.r32(sym::gHudPrompt)).wrapping_add(128)).wrapping_add(64)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) == (p_button_2)) as i32)) != 0)) as i32)) != 0) {
        break 'f;
    }
    if ((((({ let a_ = (((c.m.r32(sym::gHudPrompt)).wrapping_add(224)).wrapping_add(8)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) >= (0i32)) as i32)) != 0) {
        break 'f;
    }
    { let v_ = p_button_2; let a_ = (((c.m.r32(sym::gHudPrompt)).wrapping_add(128)).wrapping_add(64)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    's1: {
        let sv5: i32 = p_button_2;
        match sv5 {
            2i32 => {
                { let v_ = 1i32; let a_ = ((c.m.r32(sym::gHudPrompt)).wrapping_add(24)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = 2i32; let a_ = ((c.m.r32(sym::gHudPrompt)).wrapping_add(128)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (((c.m.r32(sym::gHudPrompt)).wrapping_add(128)).wrapping_add(56)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            3i32 => {
                { let v_ = 1i32; let a_ = ((c.m.r32(sym::gHudPrompt)).wrapping_add(24)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = 3i32; let a_ = ((c.m.r32(sym::gHudPrompt)).wrapping_add(128)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (((c.m.r32(sym::gHudPrompt)).wrapping_add(128)).wrapping_add(56)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            1i32 => {
                { let v_ = 1i32; let a_ = ((c.m.r32(sym::gHudPrompt)).wrapping_add(24)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = 1i32; let a_ = ((c.m.r32(sym::gHudPrompt)).wrapping_add(128)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (((c.m.r32(sym::gHudPrompt)).wrapping_add(128)).wrapping_add(56)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            0i32 => {
                { let v_ = 1i32; let a_ = ((c.m.r32(sym::gHudPrompt)).wrapping_add(24)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = ((c.m.r32(sym::gHudPrompt)).wrapping_add(128)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (((c.m.r32(sym::gHudPrompt)).wrapping_add(128)).wrapping_add(56)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            6i32 => {
                { let v_ = 4i32; let a_ = ((c.m.r32(sym::gHudPrompt)).wrapping_add(24)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = 2i32; let a_ = ((c.m.r32(sym::gHudPrompt)).wrapping_add(128)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (((c.m.r32(sym::gHudPrompt)).wrapping_add(128)).wrapping_add(56)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            7i32 => {
                { let v_ = 4i32; let a_ = ((c.m.r32(sym::gHudPrompt)).wrapping_add(24)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = 3i32; let a_ = ((c.m.r32(sym::gHudPrompt)).wrapping_add(128)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (((c.m.r32(sym::gHudPrompt)).wrapping_add(128)).wrapping_add(56)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            5i32 => {
                { let v_ = 4i32; let a_ = ((c.m.r32(sym::gHudPrompt)).wrapping_add(24)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = 1i32; let a_ = ((c.m.r32(sym::gHudPrompt)).wrapping_add(128)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (((c.m.r32(sym::gHudPrompt)).wrapping_add(128)).wrapping_add(56)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            4i32 => {
                { let v_ = 4i32; let a_ = ((c.m.r32(sym::gHudPrompt)).wrapping_add(24)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = ((c.m.r32(sym::gHudPrompt)).wrapping_add(128)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (((c.m.r32(sym::gHudPrompt)).wrapping_add(128)).wrapping_add(56)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            10i32 => {
                { let v_ = 7i32; let a_ = ((c.m.r32(sym::gHudPrompt)).wrapping_add(24)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = 2i32; let a_ = ((c.m.r32(sym::gHudPrompt)).wrapping_add(128)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (((c.m.r32(sym::gHudPrompt)).wrapping_add(128)).wrapping_add(56)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            11i32 => {
                { let v_ = 7i32; let a_ = ((c.m.r32(sym::gHudPrompt)).wrapping_add(24)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = 3i32; let a_ = ((c.m.r32(sym::gHudPrompt)).wrapping_add(128)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (((c.m.r32(sym::gHudPrompt)).wrapping_add(128)).wrapping_add(56)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            9i32 => {
                { let v_ = 7i32; let a_ = ((c.m.r32(sym::gHudPrompt)).wrapping_add(24)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = 1i32; let a_ = ((c.m.r32(sym::gHudPrompt)).wrapping_add(128)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (((c.m.r32(sym::gHudPrompt)).wrapping_add(128)).wrapping_add(56)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            8i32 => {
                { let v_ = 7i32; let a_ = ((c.m.r32(sym::gHudPrompt)).wrapping_add(24)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = ((c.m.r32(sym::gHudPrompt)).wrapping_add(128)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (((c.m.r32(sym::gHudPrompt)).wrapping_add(128)).wrapping_add(56)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            12i32 => {
                if (((((p_group_3) != (1i32)) as i32)) != 0) {
                    { let a1_ = { let a_ = c.m.r32(sym::gHudPrompt); c.m.r32(a_) }; F::HudSprite_InitTex(c, l_icon_4, a1_, 1i32, 0i32) };
                    { let v_ = 1i32; let a_ = ((c.m.r32(sym::gHudPrompt)).wrapping_add(128)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    F::HudSprite_Center(c, l_icon_4);
                    { let v_ = 10i32; let a_ = ((c.m.r32(sym::gHudPrompt)).wrapping_add(24)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                }
                { let v_ = 0i32; let a_ = (((c.m.r32(sym::gHudPrompt)).wrapping_add(128)).wrapping_add(56)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            _ => {
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudPrompt_SetButton", &PROBE_N, &[], true); }
    r_
}

pub fn HudPrompt_SetCommand(c: &mut Ctx, mut p_side_1: i32, mut p_count_2: i32, mut p_icons_3: u32, mut p_modes_4: u32, mut p_nameIdx_5: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudPrompt_SetCommand", &PROBE_N, &[(p_side_1) as u32, (p_count_2) as u32, (p_icons_3) as u32, (p_modes_4) as u32, (p_nameIdx_5) as u32], false); }
    let mut l_i_6: i32 = 0;
    let mut l_same_7: i32 = 0;
    let mut l_name_8: u32 = 0;
    let r_ = 'f: {
    if (((((p_count_2) <= (0i32)) as i32)) != 0) {
        break 'f;
    }
    if ((((({ let a_ = ((c.m.r32(sym::gHudPrompt)).wrapping_add(24)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) > (0i32)) as i32)) != 0) {
        break 'f;
    }
    if ((((({ let a_ = ((c.m.r32(sym::gHudPrompt)).wrapping_add(224)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) == (1i32)) as i32)) != 0) {
        l_same_7 = 1i32;
        {
            l_i_6 = 0i32;
            'l1: loop {
                if !(((((l_i_6) < (p_count_2)) as i32)) != 0) { break 'l1; }
                'c2: {
                    if ((((((c.m.r32((p_icons_3).wrapping_add((((l_i_6) as u32)).wrapping_mul(4))) as i32)) != ({ let a_ = ((((c.m.r32(sym::gHudPrompt)).wrapping_add(288)).wrapping_add(8)).wrapping_add((((p_side_1) as u32)).wrapping_mul(96))).wrapping_add((((l_i_6) as u32)).wrapping_mul(24)); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                        l_same_7 = 0i32;
                    }
                    if ((((((c.m.r32((p_modes_4).wrapping_add((((l_i_6) as u32)).wrapping_mul(4))) as i32)) != ({ let a_ = (((((c.m.r32(sym::gHudPrompt)).wrapping_add(288)).wrapping_add(8)).wrapping_add((((p_side_1) as u32)).wrapping_mul(96))).wrapping_add((((l_i_6) as u32)).wrapping_mul(24))).wrapping_add(12); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                        l_same_7 = 0i32;
                    }
                }
                { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; };
            }
        }
        if (((((p_nameIdx_5) != ({ let a_ = ((c.m.r32(sym::gHudPrompt)).wrapping_add(504)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) })) as i32)) != 0) {
            l_same_7 = 0i32;
        }
        if ((l_same_7) != 0) {
            break 'f;
        }
    }
    if (((((p_nameIdx_5) < (0i32)) as i32)) != 0) {
        l_name_8 = ((c.m.r32(sym::gHudPrompt)).wrapping_add(608)).wrapping_add((((p_side_1) as u32)).wrapping_mul(24));
        if (((rt::flt(F32(0x00000000), F32(c.m.r32((l_name_8).wrapping_add(16)))) as i32)) != 0) {
            F::Ramp_Start(c, l_name_8, F32(0x3e999999), F32(0x3f800000), F32(0x00000000));
        } else {
            { let v_ = p_nameIdx_5; let a_ = ((c.m.r32(sym::gHudPrompt)).wrapping_add(504)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        }
    } else {
        { let v_ = p_nameIdx_5; let a_ = ((c.m.r32(sym::gHudPrompt)).wrapping_add(504)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    }
    if ((((({ let a_ = ((c.m.r32(sym::gHudPrompt)).wrapping_add(224)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) != (1i32)) as i32)) != 0) {
        { let v_ = 0i32; let a_ = (((c.m.r32(sym::gHudPrompt)).wrapping_add(224)).wrapping_add(8)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    }
    { let v_ = 1i32; let a_ = ((c.m.r32(sym::gHudPrompt)).wrapping_add(224)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    { let v_ = p_count_2; let a_ = ((c.m.r32(sym::gHudPrompt)).wrapping_add(288)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    {
        l_i_6 = 0i32;
        'l3: loop {
            if !(((((l_i_6) < (4i32)) as i32)) != 0) { break 'l3; }
            'c4: {
                if (((((l_i_6) < (p_count_2)) as i32)) != 0) {
                    { let v_ = (c.m.r32((p_icons_3).wrapping_add((((l_i_6) as u32)).wrapping_mul(4))) as i32); let a_ = ((((c.m.r32(sym::gHudPrompt)).wrapping_add(288)).wrapping_add(8)).wrapping_add((((p_side_1) as u32)).wrapping_mul(96))).wrapping_add((((l_i_6) as u32)).wrapping_mul(24)); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = (((((c.m.r32(sym::gHudPrompt)).wrapping_add(288)).wrapping_add(8)).wrapping_add((((p_side_1) as u32)).wrapping_mul(96))).wrapping_add((((l_i_6) as u32)).wrapping_mul(24))).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = (((((c.m.r32(sym::gHudPrompt)).wrapping_add(288)).wrapping_add(8)).wrapping_add((((p_side_1) as u32)).wrapping_mul(96))).wrapping_add((((l_i_6) as u32)).wrapping_mul(24))).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = (c.m.r32((p_modes_4).wrapping_add((((l_i_6) as u32)).wrapping_mul(4))) as i32); let a_ = (((((c.m.r32(sym::gHudPrompt)).wrapping_add(288)).wrapping_add(8)).wrapping_add((((p_side_1) as u32)).wrapping_mul(96))).wrapping_add((((l_i_6) as u32)).wrapping_mul(24))).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = (((((c.m.r32(sym::gHudPrompt)).wrapping_add(288)).wrapping_add(8)).wrapping_add((((p_side_1) as u32)).wrapping_mul(96))).wrapping_add((((l_i_6) as u32)).wrapping_mul(24))).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
                    if ((((((((((((c.m.r32((p_icons_3).wrapping_add((((l_i_6) as u32)).wrapping_mul(4))) as i32)) as u32)) < (((16i32) as u32))) as i32)) != 0) && ((((((c.m.r32((p_modes_4).wrapping_add((((l_i_6) as u32)).wrapping_mul(4))) as i32)) == (2i32)) as i32)) != 0)) as i32)) != 0) {
                        { let v_ = 1i32; let a_ = (((((c.m.r32(sym::gHudPrompt)).wrapping_add(288)).wrapping_add(8)).wrapping_add((((p_side_1) as u32)).wrapping_mul(96))).wrapping_add((((l_i_6) as u32)).wrapping_mul(24))).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
                    }
                } else {
                    { let _ = { let a0_ = ((((c.m.r32(sym::gHudPrompt)).wrapping_add(288)).wrapping_add(8)).wrapping_add((((p_side_1) as u32)).wrapping_mul(96))).wrapping_add((((l_i_6) as u32)).wrapping_mul(24)); F::memset(c, a0_, 0i32, 24u32) }; };
                }
            }
            { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudPrompt_SetCommand", &PROBE_N, &[], true); }
    r_
}

pub fn HudPrompt_ClearCommand(c: &mut Ctx, mut p_side_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudPrompt_ClearCommand", &PROBE_N, &[(p_side_1) as u32], false); }
    let mut l_name_2: u32 = 0;
    let r_ = 'f: {
    l_name_2 = ((c.m.r32(sym::gHudPrompt)).wrapping_add(608)).wrapping_add((((p_side_1) as u32)).wrapping_mul(24));
    if (({ let a_ = ((c.m.r32(sym::gHudPrompt)).wrapping_add(224)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) != 0) {
        { let v_ = 0i32; let a_ = ((c.m.r32(sym::gHudPrompt)).wrapping_add(224)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        if (((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0) {
            { let v_ = 9i32; let a_ = (((c.m.r32(sym::gHudPrompt)).wrapping_add(224)).wrapping_add(8)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            F::Ramp_Stop(c, l_name_2);
        } else {
            { let v_ = 3i32; let a_ = (((c.m.r32(sym::gHudPrompt)).wrapping_add(224)).wrapping_add(8)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            if (((rt::flt(F32(0x00000000), F32(c.m.r32((l_name_2).wrapping_add(16)))) as i32)) != 0) {
                F::Ramp_Start(c, l_name_2, F32(0x3e999999), F32(0x3f800000), F32(0x00000000));
                break 'f;
            }
        }
        { let v_ = (1i32).wrapping_neg(); let a_ = ((c.m.r32(sym::gHudPrompt)).wrapping_add(504)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudPrompt_ClearCommand", &PROBE_N, &[], true); }
    r_
}

pub fn HudPrompt_AcceptCommand(c: &mut Ctx, mut p_side_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudPrompt_AcceptCommand", &PROBE_N, &[(p_side_1) as u32], false); }
    let mut l_on_2: i32 = 0;
    let mut l_name_3: u32 = 0;
    let r_ = 'f: {
    l_on_2 = { let a_ = ((c.m.r32(sym::gHudPrompt)).wrapping_add(224)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
    l_name_3 = ((c.m.r32(sym::gHudPrompt)).wrapping_add(608)).wrapping_add((((p_side_1) as u32)).wrapping_mul(24));
    if ((l_on_2) != 0) {
        { let v_ = 0i32; let a_ = ((c.m.r32(sym::gHudPrompt)).wrapping_add(224)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = 5i32; let a_ = (((c.m.r32(sym::gHudPrompt)).wrapping_add(224)).wrapping_add(8)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        if (((rt::flt(F32(0x00000000), F32(c.m.r32((l_name_3).wrapping_add(16)))) as i32)) != 0) {
            F::Ramp_Start(c, l_name_3, F32(0x3e999999), F32(0x3f800000), F32(0x00000000));
            break 'f;
        }
        { let v_ = (1i32).wrapping_neg(); let a_ = ((c.m.r32(sym::gHudPrompt)).wrapping_add(504)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudPrompt_AcceptCommand", &PROBE_N, &[], true); }
    r_
}

pub fn HudPrompt_SlideOut(c: &mut Ctx, mut p_seconds_1: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudPrompt_SlideOut", &PROBE_N, &[(p_seconds_1).0], false); }
    let r_ = 'f: {
    { let a0_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(200); F::Ramp_Start(c, a0_, p_seconds_1, F32(0x00000000), F32(0x3f800000)) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudPrompt_SlideOut", &PROBE_N, &[], true); }
    r_
}

pub fn HudPrompt_SlideIn(c: &mut Ctx, mut p_seconds_1: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudPrompt_SlideIn", &PROBE_N, &[(p_seconds_1).0], false); }
    let r_ = 'f: {
    { let a0_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(200); F::Ramp_Start(c, a0_, p_seconds_1, F32(0x3f800000), F32(0x00000000)) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudPrompt_SlideIn", &PROBE_N, &[], true); }
    r_
}

pub fn HudPrompt_ClearButton(c: &mut Ctx, mut p_side_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudPrompt_ClearButton", &PROBE_N, &[(p_side_1) as u32], false); }
    let r_ = 'f: {
    if (((((((({ let a_ = ((c.m.r32(sym::gHudPrompt)).wrapping_add(24)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_sub(1i32)) as u32)) < (((38i32) as u32))) as i32)) != 0) {
        { let v_ = 40i32; let a_ = ((c.m.r32(sym::gHudPrompt)).wrapping_add(24)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudPrompt_ClearButton", &PROBE_N, &[], true); }
    r_
}

pub fn HudPrompt_DrawNames(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudPrompt_DrawNames", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let mut l_fade_2: u32 = 0;
    let mut l_alpha_3: F32 = F32(0);
    let mut l_pos_4: u32 = 0;
    let mut l_idx_5: i32 = 0;
    let r_ = 'f: {
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (2i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                l_fade_2 = ((c.m.r32(sym::gHudPrompt)).wrapping_add(608)).wrapping_add((((l_i_1) as u32)).wrapping_mul(24));
                l_alpha_3 = F32(c.m.r32((l_fade_2).wrapping_add(16)));
                l_pos_4 = ((c.m.r32(sym::gHudPrompt)).wrapping_add(488)).wrapping_add((((l_i_1) as u32)).wrapping_mul(8));
                l_idx_5 = { let a_ = ((c.m.r32(sym::gHudPrompt)).wrapping_add(504)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
                if (((rt::fle(l_alpha_3, F32(0x00000000)) as i32)) != 0) {
                    { let v_ = (1i32).wrapping_neg(); let a_ = ((c.m.r32(sym::gHudPrompt)).wrapping_add(504)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    l_idx_5 = (1i32).wrapping_neg();
                }
                if (((((l_idx_5) >= (0i32)) as i32)) != 0) {
                    { let a0_ = { let a_ = (F::BtlCtrl_GetObj(c, l_i_1)).wrapping_add(188); (c.m.r32(a_) as i32) }; F::BtlMenu_SetScript2(c, ((((a0_) as i64 as u64)) as u32)) };
                    { let a0_ = (c.m.r32(l_pos_4) as i32); let a1_ = (c.m.r32((l_pos_4).wrapping_add(4)) as i32); F::BtlText_DrawEntryName(c, a0_, a1_, l_idx_5, l_i_1, l_alpha_3) };
                }
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudPrompt_DrawNames", &PROBE_N, &[], true); }
    r_
}

pub fn HudPrompt_Init(c: &mut Ctx, mut p_out_1: u32, mut p_res_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudPrompt_Init", &PROBE_N, &[(p_out_1) as u32, (p_res_2) as u32], false); }
    let mut l_spr_3: u32 = 0;
    let mut l_node_4: u32 = 0;
    let mut l_i_5: i32 = 0;
    let mut l_y_6: i32 = 0;
    let mut l_rowCount_7: i32 = 0;
    let r_ = 'f: {
    { let v_ = F::Heap_Alloc(c, ((720u32) as i32), ((32i32) as u32), 0i32, 2i32); let a_ = sym::gHudPrompt; c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32(sym::gHudPrompt); F::memset(c, a0_, 0i32, 720u32) }; };
    { let v_ = F::Heap_Alloc(c, (((((22i32) as u32)).wrapping_mul(28u32)) as i32), ((32i32) as u32), 0i32, 2i32); let a_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(4); c.m.w32(a_, v_); };
    { let _ = { let a0_ = { let a_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(4); c.m.r32(a_) }; F::memset(c, a0_, 0i32, (((22i32) as u32)).wrapping_mul(28u32)) }; };
    { let v_ = F::Heap_Alloc(c, (((((27i32) as u32)).wrapping_mul(56u32)) as i32), ((32i32) as u32), 0i32, 2i32); let a_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(8); c.m.w32(a_, v_); };
    { let _ = { let a0_ = { let a_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(8); c.m.r32(a_) }; F::memset(c, a0_, 0i32, (((27i32) as u32)).wrapping_mul(56u32)) }; };
    {
        l_i_5 = 0i32;
        'l1: loop {
            if !(((((l_i_5) < (2i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let v_ = (1i32).wrapping_neg(); let a_ = ((c.m.r32(sym::gHudPrompt)).wrapping_add(24)).wrapping_add((((l_i_5) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = (1i32).wrapping_neg(); let a_ = (((c.m.r32(sym::gHudPrompt)).wrapping_add(224)).wrapping_add(8)).wrapping_add((((l_i_5) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            }
            { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
        }
    }
    l_y_6 = 384i32;
    l_rowCount_7 = 8i32;
    { let v_ = (1i32).wrapping_neg(); let a_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(656); c.m.w32(a_, (v_) as u32); };
    { let v_ = p_res_2; let a_ = c.m.r32(sym::gHudPrompt); c.m.w32(a_, v_); };
    { let v_ = F::FontIcon_GetRes(c); let a_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(12); c.m.w32(a_, v_); };
    { let v_ = F::FontIcon_GetDefs(c); let a_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(16); c.m.w32(a_, v_); };
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((0i32) as u32)).wrapping_mul(28));
    F::HudSprite_InitTex(c, l_spr_3, p_res_2, 0i32, 0i32);
    F::HudSprite_Center(c, l_spr_3);
    F::HudSprite_Show(c, l_spr_3, 0i32);
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((4i32) as u32)).wrapping_mul(28));
    F::HudSprite_InitTex(c, l_spr_3, p_res_2, 8i32, 0i32);
    F::HudSprite_Center(c, l_spr_3);
    F::HudSprite_Show(c, l_spr_3, 0i32);
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((1i32) as u32)).wrapping_mul(28));
    F::HudSprite_InitTex(c, l_spr_3, p_res_2, 1i32, 0i32);
    F::HudSprite_Center(c, l_spr_3);
    F::HudSprite_Show(c, l_spr_3, 0i32);
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((5i32) as u32)).wrapping_mul(28));
    F::HudSprite_InitTex(c, l_spr_3, p_res_2, 1i32, 0i32);
    F::HudSprite_Center(c, l_spr_3);
    F::HudSprite_Show(c, l_spr_3, 0i32);
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((2i32) as u32)).wrapping_mul(28));
    F::HudSprite_InitTex(c, l_spr_3, p_res_2, 1i32, 0i32);
    F::HudSprite_Center(c, l_spr_3);
    F::HudSprite_Show(c, l_spr_3, 0i32);
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((3i32) as u32)).wrapping_mul(28));
    F::HudSprite_InitTex(c, l_spr_3, p_res_2, 7i32, 0i32);
    F::HudSprite_Center(c, l_spr_3);
    F::HudSprite_Show(c, l_spr_3, 0i32);
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((6i32) as u32)).wrapping_mul(28));
    F::HudSprite_SetColor(c, l_spr_3, 255i32, 255i32, 255i32, 128i32);
    F::HudSprite_SetRect(c, l_spr_3, 0i32, 32i32, 0i32, 32i32);
    F::HudSprite_Show(c, l_spr_3, 0i32);
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((7i32) as u32)).wrapping_mul(28));
    F::HudSprite_SetColor(c, l_spr_3, 255i32, 255i32, 255i32, 128i32);
    F::HudSprite_SetRect(c, l_spr_3, 0i32, 32i32, 0i32, 32i32);
    F::HudSprite_Show(c, l_spr_3, 0i32);
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((8i32) as u32)).wrapping_mul(28));
    F::HudSprite_SetColor(c, l_spr_3, 255i32, 255i32, 255i32, 128i32);
    F::HudSprite_SetRect(c, l_spr_3, 0i32, 32i32, 0i32, 32i32);
    F::HudSprite_Show(c, l_spr_3, 0i32);
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((9i32) as u32)).wrapping_mul(28));
    F::HudSprite_SetColor(c, l_spr_3, 255i32, 255i32, 255i32, 128i32);
    F::HudSprite_SetRect(c, l_spr_3, 0i32, 32i32, 0i32, 32i32);
    F::HudSprite_Show(c, l_spr_3, 0i32);
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((10i32) as u32)).wrapping_mul(28));
    F::HudSprite_SetColor(c, l_spr_3, 255i32, 255i32, 255i32, 128i32);
    F::HudSprite_SetRect(c, l_spr_3, 0i32, 32i32, 0i32, 32i32);
    F::HudSprite_Show(c, l_spr_3, 0i32);
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((11i32) as u32)).wrapping_mul(28));
    F::HudSprite_SetColor(c, l_spr_3, 255i32, 255i32, 255i32, 128i32);
    F::HudSprite_SetRect(c, l_spr_3, 0i32, 32i32, 0i32, 32i32);
    F::HudSprite_Show(c, l_spr_3, 0i32);
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((12i32) as u32)).wrapping_mul(28));
    F::HudSprite_SetColor(c, l_spr_3, 255i32, 255i32, 255i32, 128i32);
    F::HudSprite_SetRect(c, l_spr_3, 0i32, 32i32, 0i32, 32i32);
    F::HudSprite_Show(c, l_spr_3, 0i32);
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((13i32) as u32)).wrapping_mul(28));
    F::HudSprite_SetColor(c, l_spr_3, 255i32, 255i32, 255i32, 128i32);
    F::HudSprite_SetRect(c, l_spr_3, 0i32, 32i32, 0i32, 32i32);
    F::HudSprite_Show(c, l_spr_3, 0i32);
    l_node_4 = ({ let a_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((24i32) as u32)).wrapping_mul(56));
    { let v_ = 64i32; let a_ = (l_node_4).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
    { let v_ = l_y_6; let a_ = (l_node_4).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
    { let v_ = ((4i32) as u32); let a_ = (l_node_4).wrapping_add(32); c.m.w32(a_, v_); };
    { let v_ = F::Heap_Alloc(c, (((((4i32) as u32)).wrapping_mul(4u32)) as i32), ((32i32) as u32), 0i32, 2i32); let a_ = (l_node_4).wrapping_add(36); c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32((l_node_4).wrapping_add(36)); let a2_ = (c.m.r32((l_node_4).wrapping_add(32))).wrapping_mul(4u32); F::memset(c, a0_, 0i32, a2_) }; };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((0i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_node_4).wrapping_add(36))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((1i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_node_4).wrapping_add(36))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((2i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_node_4).wrapping_add(36))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((3i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_node_4).wrapping_add(36))).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = fid::HudPrompt_UpdateButton; let a_ = (l_node_4).wrapping_add(48); c.m.w32(a_, v_); };
    { let v_ = fid::HudPrompt_DrawButton; let a_ = (l_node_4).wrapping_add(52); c.m.w32(a_, v_); };
    l_node_4 = ({ let a_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((25i32) as u32)).wrapping_mul(56));
    { let v_ = 256i32; let a_ = (l_node_4).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
    { let v_ = l_y_6; let a_ = (l_node_4).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
    { let v_ = ((2i32) as u32); let a_ = (l_node_4).wrapping_add(32); c.m.w32(a_, v_); };
    { let v_ = F::Heap_Alloc(c, (((((2i32) as u32)).wrapping_mul(4u32)) as i32), ((32i32) as u32), 0i32, 2i32); let a_ = (l_node_4).wrapping_add(36); c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32((l_node_4).wrapping_add(36)); let a2_ = (c.m.r32((l_node_4).wrapping_add(32))).wrapping_mul(4u32); F::memset(c, a0_, 0i32, a2_) }; };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((4i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_node_4).wrapping_add(36))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((5i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_node_4).wrapping_add(36))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = fid::HudPrompt_UpdateCue; let a_ = (l_node_4).wrapping_add(48); c.m.w32(a_, v_); };
    { let v_ = fid::HudPrompt_DrawCue; let a_ = (l_node_4).wrapping_add(52); c.m.w32(a_, v_); };
    l_node_4 = ({ let a_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((26i32) as u32)).wrapping_mul(56));
    { let v_ = 60i32; let a_ = (l_node_4).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
    { let v_ = 400i32; let a_ = (l_node_4).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
    { let v_ = ((l_rowCount_7) as u32); let a_ = (l_node_4).wrapping_add(32); c.m.w32(a_, v_); };
    { let v_ = F::Heap_Alloc(c, (((((8i32) as u32)).wrapping_mul(4u32)) as i32), ((32i32) as u32), 0i32, 2i32); let a_ = (l_node_4).wrapping_add(36); c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32((l_node_4).wrapping_add(36)); let a2_ = (c.m.r32((l_node_4).wrapping_add(32))).wrapping_mul(4u32); F::memset(c, a0_, 0i32, a2_) }; };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((6i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_node_4).wrapping_add(36))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((7i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_node_4).wrapping_add(36))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((8i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_node_4).wrapping_add(36))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((9i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_node_4).wrapping_add(36))).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((10i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_node_4).wrapping_add(36))).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((11i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_node_4).wrapping_add(36))).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((12i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_node_4).wrapping_add(36))).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((13i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_node_4).wrapping_add(36))).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = fid::HudPrompt_UpdateCommand; let a_ = (l_node_4).wrapping_add(48); c.m.w32(a_, v_); };
    { let v_ = fid::HudPrompt_DrawCommand; let a_ = (l_node_4).wrapping_add(52); c.m.w32(a_, v_); };
    l_node_4 = ({ let a_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((23i32) as u32)).wrapping_mul(56));
    { let v_ = 0i32; let a_ = (l_node_4).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
    { let v_ = 0i32; let a_ = (l_node_4).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
    { let v_ = ((3i32) as u32); let a_ = (l_node_4).wrapping_add(40); c.m.w32(a_, v_); };
    { let v_ = F::Heap_Alloc(c, (((((3i32) as u32)).wrapping_mul(4u32)) as i32), ((32i32) as u32), 0i32, 2i32); let a_ = (l_node_4).wrapping_add(44); c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32((l_node_4).wrapping_add(44)); let a2_ = (c.m.r32((l_node_4).wrapping_add(40))).wrapping_mul(4u32); F::memset(c, a0_, 0i32, a2_) }; };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((24i32) as u32)).wrapping_mul(56)); let a_ = (c.m.r32((l_node_4).wrapping_add(44))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((25i32) as u32)).wrapping_mul(56)); let a_ = (c.m.r32((l_node_4).wrapping_add(44))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((26i32) as u32)).wrapping_mul(56)); let a_ = (c.m.r32((l_node_4).wrapping_add(44))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (l_node_4).wrapping_add(52); c.m.w32(a_, v_); };
    { let v_ = fid::HudPrompt_UpdateRoot; let a_ = (l_node_4).wrapping_add(48); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((23i32) as u32)).wrapping_mul(56)); let a_ = p_out_1; c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudPrompt_Init", &PROBE_N, &[], true); }
    r_
}

pub fn HudPrompt_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudPrompt_Term", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let r_ = 'f: {
    if ((((({ let a_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(4); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = { let a_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(4); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
    }
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (27i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((((({ let a_ = (({ let a_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((l_i_1) as u32)).wrapping_mul(56))).wrapping_add(36); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
                    { let a0_ = { let a_ = (({ let a_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((l_i_1) as u32)).wrapping_mul(56))).wrapping_add(36); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
                }
                if ((((({ let a_ = (({ let a_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((l_i_1) as u32)).wrapping_mul(56))).wrapping_add(44); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
                    { let a0_ = { let a_ = (({ let a_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((l_i_1) as u32)).wrapping_mul(56))).wrapping_add(44); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
                }
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
    if ((((({ let a_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(8); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = { let a_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(8); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
    }
    if (((((c.m.r32(sym::gHudPrompt)) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32(sym::gHudPrompt); F::Heap_Free(c, a0_) };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudPrompt_Term", &PROBE_N, &[], true); }
    r_
}

pub fn HudPrompt_Reset(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudPrompt_Reset", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let r_ = 'f: {
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (2i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let v_ = (1i32).wrapping_neg(); let a_ = ((c.m.r32(sym::gHudPrompt)).wrapping_add(24)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let a0_ = ((c.m.r32(sym::gHudPrompt)).wrapping_add(32)).wrapping_add((((l_i_1) as u32)).wrapping_mul(24)); F::Ramp_Stop(c, a0_) };
                { let a0_ = ((c.m.r32(sym::gHudPrompt)).wrapping_add(80)).wrapping_add((((l_i_1) as u32)).wrapping_mul(24)); F::Ramp_Stop(c, a0_) };
                { let a0_ = ((c.m.r32(sym::gHudPrompt)).wrapping_add(240)).wrapping_add((((l_i_1) as u32)).wrapping_mul(24)); F::Ramp_Stop(c, a0_) };
                { let a0_ = ((c.m.r32(sym::gHudPrompt)).wrapping_add(512)).wrapping_add((((l_i_1) as u32)).wrapping_mul(24)); F::Ramp_Stop(c, a0_) };
                { let a0_ = ((c.m.r32(sym::gHudPrompt)).wrapping_add(560)).wrapping_add((((l_i_1) as u32)).wrapping_mul(24)); F::Ramp_Stop(c, a0_) };
                { let v_ = 0i32; let a_ = ((c.m.r32(sym::gHudPrompt)).wrapping_add(128)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (((c.m.r32(sym::gHudPrompt)).wrapping_add(128)).wrapping_add(56)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (((c.m.r32(sym::gHudPrompt)).wrapping_add(128)).wrapping_add(64)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = (1i32).wrapping_neg(); let a_ = (((c.m.r32(sym::gHudPrompt)).wrapping_add(224)).wrapping_add(8)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = ((c.m.r32(sym::gHudPrompt)).wrapping_add(288)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = ((c.m.r32(sym::gHudPrompt)).wrapping_add(224)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = (1i32).wrapping_neg(); let a_ = ((c.m.r32(sym::gHudPrompt)).wrapping_add(504)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = (1i32).wrapping_neg(); let a_ = ((c.m.r32(sym::gHudPrompt)).wrapping_add(488)).wrapping_add((((l_i_1) as u32)).wrapping_mul(8)); c.m.w32(a_, (v_) as u32); };
                { let v_ = (1i32).wrapping_neg(); let a_ = (((c.m.r32(sym::gHudPrompt)).wrapping_add(488)).wrapping_add((((l_i_1) as u32)).wrapping_mul(8))).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                { let v_ = (1i32).wrapping_neg(); let a_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(656); c.m.w32(a_, (v_) as u32); };
                { let a0_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(684); F::Ramp_Stop(c, a0_) };
                { let a0_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(660); F::Ramp_Stop(c, a0_) };
                { let _ = { let a0_ = (((c.m.r32(sym::gHudPrompt)).wrapping_add(128)).wrapping_add(8)).wrapping_add((((l_i_1) as u32)).wrapping_mul(24)); F::memset(c, a0_, 0i32, 24u32) }; };
                { let _ = { let a0_ = (((c.m.r32(sym::gHudPrompt)).wrapping_add(288)).wrapping_add(8)).wrapping_add((((l_i_1) as u32)).wrapping_mul(96)); F::memset(c, a0_, 0i32, (((4i32) as u32)).wrapping_mul(24u32)) }; };
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
    { let a0_ = (c.m.r32(sym::gHudPrompt)).wrapping_add(200); F::Ramp_Stop(c, a0_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudPrompt_Reset", &PROBE_N, &[], true); }
    r_
}
