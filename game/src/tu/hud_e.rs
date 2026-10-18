#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn HudNotice_DrawBanner(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudNotice_DrawBanner", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::HudGfx_ClearAlpha(c);
    { let a0_ = ({ let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((0i32) as u32)).wrapping_mul(28)); let a1_ = { let a_ = c.m.r32(sym::gHudNotice); c.m.r32(a_) }; F::HudSprite_Draw(c, a0_, a1_, 0i32) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudNotice_DrawBanner", &PROBE_N, &[], true); }
    r_
}

pub fn HudNotice_UpdateBanner(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudNotice_UpdateBanner", &PROBE_N, &[], false); }
    let mut l_ramp_1: u32 = 0;
    let mut l_spr_2: u32 = 0;
    let mut l_streak_3: u32 = 0;
    let mut l_done_4: i32 = 0;
    let mut l_t_5: F32 = F32(0);
    let r_ = 'f: {
    l_ramp_1 = (c.m.r32(sym::gHudNotice)).wrapping_add(20);
    l_spr_2 = ({ let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((0i32) as u32)).wrapping_mul(28));
    l_streak_3 = (l_spr_2).wrapping_add((((13i32) as u32)).wrapping_mul(28));
    's1: {
        let sv6: i32 = { let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(12); (c.m.r32(a_) as i32) };
        'k7: {
        'k6: {
        'k5: {
        'k4: {
        'k3: {
        'k2: {
            match sv6 { 0i32 => break 'k2, 1i32 => break 'k3, 2i32 => break 'k4, 3i32 => break 'k5, 4i32 => break 'k6, 5i32 => break 'k7, _ => break 's1, }
        }
        F::HudSprite_Show(c, l_spr_2, 1i32);
        { let a1_ = { let a_ = c.m.r32(sym::gHudNotice); c.m.r32(a_) }; F::HudSprite_InitTex(c, l_spr_2, a1_, 6i32, 0i32) };
        F::HudSprite_Center(c, l_spr_2);
        F::HudSprite_Show(c, l_streak_3, 0i32);
        F::Ramp_Start(c, l_ramp_1, F32(0x3ecccccc), F32(0x3f800000), F32(0x00000000));
        { let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(12); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        }
        l_done_4 = F::Ramp_Step(c, l_ramp_1);
        l_t_5 = { let a0_ = rt::fmul(F32(c.m.r32((l_ramp_1).wrapping_add(16))), F32(0x3fc90fda)); F::Mathf_Sin(c, a0_) };
        { let a1_ = { let a_ = c.m.r32(sym::gHudNotice); c.m.r32(a_) }; F::HudSprite_InitTex(c, l_spr_2, a1_, 6i32, 0i32) };
        F::HudSprite_Center(c, l_spr_2);
        F::HudSprite_Move(c, l_spr_2, 0i32, rt::f2i(rt::fmul(l_t_5, F32(0xc2c80000))));
        l_t_5 = rt::fsub(F32(0x3f800000), l_t_5);
        F::HudSprite_SetColor(c, l_spr_2, 128i32, 128i32, 128i32, ((((rt::f2u(rt::fmul(l_t_5, F32(0x43000000)))) as u8)) as i32));
        if ((l_done_4) != 0) {
            { let a0_ = (c.m.r32(sym::gHudNotice)).wrapping_add(20); F::Ramp_Start(c, a0_, F32(0x3eb851eb), F32(0x00000000), F32(0x3f800000)) };
            { let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(12); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        }
        break 's1;
        }
        l_done_4 = F::Ramp_Step(c, l_ramp_1);
        l_t_5 = rt::fmul({ let a0_ = rt::fmul(F32(c.m.r32((l_ramp_1).wrapping_add(16))), F32(0x40490fda)); F::Mathf_Sin(c, a0_) }, F32(0x3dcccccc));
        { let a1_ = { let a_ = c.m.r32(sym::gHudNotice); c.m.r32(a_) }; F::HudSprite_InitTex(c, l_spr_2, a1_, 6i32, 0i32) };
        F::HudSprite_Center(c, l_spr_2);
        F::HudSprite_Move(c, l_spr_2, 0i32, rt::f2i(rt::fmul(l_t_5, F32(0xc2c80000))));
        l_t_5 = rt::fsub(F32(0x3f800000), l_t_5);
        F::HudSprite_SetColor(c, l_spr_2, 128i32, 128i32, 128i32, ((((rt::f2u(rt::fmul(l_t_5, F32(0x43000000)))) as u8)) as i32));
        if ((l_done_4) != 0) {
            F::Ramp_Start(c, l_ramp_1, F32(0x3fdd70a3), F32(0x3f800000), F32(0x00000000));
            { let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(12); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        }
        break 's1;
        }
        if ((F::Ramp_Step(c, l_ramp_1)) != 0) {
            F::Ramp_Start(c, l_ramp_1, F32(0x3ecccccc), F32(0x3f800000), F32(0x00000000));
            { let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(12); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        }
        break 's1;
        }
        l_done_4 = F::Ramp_Step(c, l_ramp_1);
        if ((l_done_4) != 0) {
            { let v_ = ((0i32) as u8); let a_ = (l_spr_2).wrapping_add(27); c.m.w8(a_, v_); };
            { let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(12); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        } else {
            l_t_5 = { let a0_ = F32(c.m.r32((l_ramp_1).wrapping_add(16))); F::Port_powf(c, a0_, F32(0x40400000)) };
            { let v_ = ((rt::f2u(rt::fmul(l_t_5, F32(0x43000000)))) as u8); let a_ = (l_spr_2).wrapping_add(27); c.m.w8(a_, v_); };
            l_t_5 = rt::fadd(rt::fmul(l_t_5, F32(0x3f000000)), F32(0x3f000000));
            F::HudSprite_Scale(c, l_spr_2, l_t_5, l_t_5);
        }
        if ((l_done_4) != 0) {
        }
        break 's1;
        }
        F::HudSprite_Show(c, l_spr_2, 0i32);
        F::HudSprite_Show(c, l_streak_3, 0i32);
        break 's1;
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudNotice_UpdateBanner", &PROBE_N, &[], true); }
    r_
}

pub fn HudNotice_GetVoiceBase(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudNotice_GetVoiceBase", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if (((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5640); (c.m.r32(a_) as i32) }) & (1i32))) != 0) {
        break 'f 68460i32;
    }
    break 'f 68404i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "HudNotice_GetVoiceBase", &PROBE_N, &[], true); }
    r_
}

pub fn HudNotice_Show(c: &mut Ctx, mut p_id_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudNotice_Show", &PROBE_N, &[(p_id_1) as u32], false); }
    let mut l_node_2: u32 = 0;
    let mut l_i_3: i32 = 0;
    let r_ = 'f: {
    { let a0_ = ({ let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((13i32) as u32)).wrapping_mul(28)); F::HudSprite_Show(c, a0_, 0i32) };
    {
        l_i_3 = 0i32;
        'l1: loop {
            if !(((((l_i_3) < (12i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let a0_ = ({ let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(4); c.m.r32(a_) }).wrapping_add(((((l_i_3).wrapping_add(1i32)) as u32)).wrapping_mul(28)); F::HudSprite_Show(c, a0_, 0i32) };
            }
            { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
        }
    }
    { let a0_ = ({ let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((0i32) as u32)).wrapping_mul(28)); F::HudSprite_Show(c, a0_, 0i32) };
    l_node_2 = ({ let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((1i32) as u32)).wrapping_mul(56));
    { let v_ = F32(0x00000000); let a_ = (l_node_2).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x00000000); let a_ = (l_node_2).wrapping_add(8); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x00000000); let a_ = (l_node_2).wrapping_add(12); c.m.w32(a_, (v_).0); };
    { let v_ = 0i32; let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
    's3: {
        let sv4: i32 = p_id_1;
        match sv4 {
            0i32 => {
                F::HudNode_SetPos(c, l_node_2, 256i32, 224i32);
                { let v_ = fid::HudNotice_UpdateReady; let a_ = (l_node_2).wrapping_add(48); c.m.w32(a_, v_); };
                { let v_ = fid::HudNotice_DrawReady; let a_ = (l_node_2).wrapping_add(52); c.m.w32(a_, v_); };
                { let a1_ = (F::HudNotice_GetVoiceBase(c)).wrapping_add((F::Battle_GetAnnouncer(c)).wrapping_mul(7i32)); F::StreamSe_PlayDefault(c, 0i32, a1_) };
                break 's3;
            }
            1i32 => {
                F::HudNode_SetPos(c, l_node_2, 256i32, 224i32);
                { let v_ = fid::HudNotice_UpdateFight; let a_ = (l_node_2).wrapping_add(48); c.m.w32(a_, v_); };
                { let v_ = fid::HudNotice_DrawFight; let a_ = (l_node_2).wrapping_add(52); c.m.w32(a_, v_); };
                { let a1_ = ((F::HudNotice_GetVoiceBase(c)).wrapping_add((F::Battle_GetAnnouncer(c)).wrapping_mul(7i32))).wrapping_add(1i32); F::StreamSe_PlayDefault(c, 0i32, a1_) };
                break 's3;
            }
            2i32 => {
                F::HudNode_SetPos(c, l_node_2, 256i32, 224i32);
                { let v_ = fid::HudNotice_UpdateKo; let a_ = (l_node_2).wrapping_add(48); c.m.w32(a_, v_); };
                { let v_ = fid::HudNotice_DrawKo; let a_ = (l_node_2).wrapping_add(52); c.m.w32(a_, v_); };
                { let a1_ = ((F::HudNotice_GetVoiceBase(c)).wrapping_add((F::Battle_GetAnnouncer(c)).wrapping_mul(7i32))).wrapping_add(2i32); F::StreamSe_PlayDefault(c, 0i32, a1_) };
                break 's3;
            }
            3i32 => {
                F::HudNode_SetPos(c, l_node_2, 256i32, 224i32);
                { let v_ = fid::HudNotice_UpdateBandA; let a_ = (l_node_2).wrapping_add(48); c.m.w32(a_, v_); };
                { let v_ = fid::HudNotice_DrawBandA; let a_ = (l_node_2).wrapping_add(52); c.m.w32(a_, v_); };
                { let a1_ = ((F::HudNotice_GetVoiceBase(c)).wrapping_add((F::Battle_GetAnnouncer(c)).wrapping_mul(7i32))).wrapping_add(3i32); F::StreamSe_PlayDefault(c, 0i32, a1_) };
                break 's3;
            }
            4i32 => {
                F::HudNode_SetPos(c, l_node_2, 256i32, 224i32);
                { let v_ = fid::HudNotice_UpdateSpinA; let a_ = (l_node_2).wrapping_add(48); c.m.w32(a_, v_); };
                { let v_ = fid::HudNotice_DrawSpin; let a_ = (l_node_2).wrapping_add(52); c.m.w32(a_, v_); };
                { let a1_ = ((F::HudNotice_GetVoiceBase(c)).wrapping_add((F::Battle_GetAnnouncer(c)).wrapping_mul(7i32))).wrapping_add(4i32); F::StreamSe_PlayDefault(c, 0i32, a1_) };
                break 's3;
            }
            5i32 => {
                F::HudNode_SetPos(c, l_node_2, 256i32, 224i32);
                { let v_ = fid::HudNotice_UpdateSpinB; let a_ = (l_node_2).wrapping_add(48); c.m.w32(a_, v_); };
                { let v_ = fid::HudNotice_DrawSpin; let a_ = (l_node_2).wrapping_add(52); c.m.w32(a_, v_); };
                { let a1_ = ((F::HudNotice_GetVoiceBase(c)).wrapping_add((F::Battle_GetAnnouncer(c)).wrapping_mul(7i32))).wrapping_add(5i32); F::StreamSe_PlayDefault(c, 0i32, a1_) };
                break 's3;
            }
            6i32 => {
                F::HudNode_SetPos(c, l_node_2, 256i32, 352i32);
                { let v_ = fid::HudNotice_UpdateTrail; let a_ = (l_node_2).wrapping_add(48); c.m.w32(a_, v_); };
                { let v_ = fid::HudNotice_DrawTrail; let a_ = (l_node_2).wrapping_add(52); c.m.w32(a_, v_); };
                break 's3;
            }
            7i32 => {
                F::HudNode_SetPos(c, l_node_2, 256i32, 352i32);
                { let v_ = fid::HudNotice_UpdateBanner; let a_ = (l_node_2).wrapping_add(48); c.m.w32(a_, v_); };
                { let v_ = fid::HudNotice_DrawBanner; let a_ = (l_node_2).wrapping_add(52); c.m.w32(a_, v_); };
                break 's3;
            }
            8i32 => {
                F::HudNode_SetPos(c, l_node_2, 256i32, 224i32);
                { let v_ = fid::HudNotice_UpdateBandB; let a_ = (l_node_2).wrapping_add(48); c.m.w32(a_, v_); };
                { let v_ = fid::HudNotice_DrawBandB; let a_ = (l_node_2).wrapping_add(52); c.m.w32(a_, v_); };
                if ((F::Rand_Range(c, ((2i32) as u32))) != 0) {
                    { let a1_ = ((F::HudNotice_GetVoiceBase(c)).wrapping_add((F::Battle_GetAnnouncer(c)).wrapping_mul(7i32))).wrapping_add(2i32); F::StreamSe_PlayDefault(c, 0i32, a1_) };
                } else {
                    { let a1_ = ((F::HudNotice_GetVoiceBase(c)).wrapping_add((F::Battle_GetAnnouncer(c)).wrapping_mul(7i32))).wrapping_add(3i32); F::StreamSe_PlayDefault(c, 0i32, a1_) };
                }
                break 's3;
            }
            _ => {
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudNotice_Show", &PROBE_N, &[], true); }
    r_
}

pub fn HudNotice_ShowReplayMark(c: &mut Ctx, mut p_on_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudNotice_ShowReplayMark", &PROBE_N, &[(p_on_1) as u32], false); }
    let r_ = 'f: {
    { let a0_ = ({ let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((15i32) as u32)).wrapping_mul(28)); F::HudSprite_Show(c, a0_, p_on_1) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudNotice_ShowReplayMark", &PROBE_N, &[], true); }
    r_
}

pub fn HudNotice_Init(c: &mut Ctx, mut p_out_1: u32, mut p_res_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudNotice_Init", &PROBE_N, &[(p_out_1) as u32, (p_res_2) as u32], false); }
    let mut l_spr_3: u32 = 0;
    let mut l_node_4: u32 = 0;
    let mut l_one_5: i32 = 0;
    let mut l_cx_6: i32 = 0;
    let mut l_cy_7: i32 = 0;
    let r_ = 'f: {
    l_one_5 = 1i32;
    l_cx_6 = 256i32;
    l_cy_7 = 224i32;
    { let v_ = F::Heap_Alloc(c, ((116u32) as i32), ((32i32) as u32), 0i32, 2i32); let a_ = sym::gHudNotice; c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32(sym::gHudNotice); F::memset(c, a0_, 0i32, 116u32) }; };
    { let v_ = F::Heap_Alloc(c, (((((16i32) as u32)).wrapping_mul(28u32)) as i32), ((32i32) as u32), 0i32, 2i32); let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(4); c.m.w32(a_, v_); };
    { let _ = { let a0_ = { let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(4); c.m.r32(a_) }; F::memset(c, a0_, 0i32, (((16i32) as u32)).wrapping_mul(28u32)) }; };
    { let v_ = F::Heap_Alloc(c, (((((4i32) as u32)).wrapping_mul(56u32)) as i32), ((32i32) as u32), 0i32, 2i32); let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(8); c.m.w32(a_, v_); };
    { let _ = { let a0_ = { let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(8); c.m.r32(a_) }; F::memset(c, a0_, 0i32, (((4i32) as u32)).wrapping_mul(56u32)) }; };
    { let v_ = (1i32).wrapping_neg(); let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
    { let v_ = p_res_2; let a_ = c.m.r32(sym::gHudNotice); c.m.w32(a_, v_); };
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((0i32) as u32)).wrapping_mul(28));
    F::HudSprite_InitTex(c, l_spr_3, p_res_2, 0i32, 0i32);
    F::HudSprite_Center(c, l_spr_3);
    F::HudSprite_Show(c, l_spr_3, 0i32);
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((1i32) as u32)).wrapping_mul(28));
    F::HudSprite_InitTex(c, l_spr_3, p_res_2, 0i32, 0i32);
    F::HudSprite_Center(c, l_spr_3);
    F::HudSprite_Show(c, l_spr_3, 0i32);
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((13i32) as u32)).wrapping_mul(28));
    F::HudSprite_InitTex(c, l_spr_3, p_res_2, 7i32, 0i32);
    F::HudSprite_Center(c, l_spr_3);
    F::HudSprite_Show(c, l_spr_3, 0i32);
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((14i32) as u32)).wrapping_mul(28));
    F::HudSprite_Show(c, l_spr_3, 0i32);
    F::HudSprite_InitPlain(c, l_spr_3, ((((128i32) as i64 as u64)) as u8), ((((128i32) as i64 as u64)) as u8), ((((128i32) as i64 as u64)) as u8), ((((128i32) as i64 as u64)) as u8));
    F::HudSprite_Center(c, l_spr_3);
    l_spr_3 = ({ let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((15i32) as u32)).wrapping_mul(28));
    F::HudSprite_Show(c, l_spr_3, 0i32);
    F::HudSprite_InitTex(c, l_spr_3, p_res_2, 8i32, 0i32);
    F::HudSprite_Center(c, l_spr_3);
    l_node_4 = ({ let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((2i32) as u32)).wrapping_mul(56));
    { let v_ = 0i32; let a_ = (l_node_4).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
    { let v_ = 0i32; let a_ = (l_node_4).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
    { let v_ = ((l_one_5) as u32); let a_ = (l_node_4).wrapping_add(32); c.m.w32(a_, v_); };
    { let v_ = F::Heap_Alloc(c, (((((1i32) as u32)).wrapping_mul(4u32)) as i32), ((32i32) as u32), 0i32, 2i32); let a_ = (l_node_4).wrapping_add(36); c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32((l_node_4).wrapping_add(36)); let a2_ = (c.m.r32((l_node_4).wrapping_add(32))).wrapping_mul(4u32); F::memset(c, a0_, 0i32, a2_) }; };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((14i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_node_4).wrapping_add(36))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (l_node_4).wrapping_add(48); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (l_node_4).wrapping_add(52); c.m.w32(a_, v_); };
    l_node_4 = ({ let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((3i32) as u32)).wrapping_mul(56));
    { let v_ = l_cx_6; let a_ = (l_node_4).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
    { let v_ = 50i32; let a_ = (l_node_4).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
    { let v_ = ((l_one_5) as u32); let a_ = (l_node_4).wrapping_add(32); c.m.w32(a_, v_); };
    { let v_ = F::Heap_Alloc(c, (((((1i32) as u32)).wrapping_mul(4u32)) as i32), ((32i32) as u32), 0i32, 2i32); let a_ = (l_node_4).wrapping_add(36); c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32((l_node_4).wrapping_add(36)); let a2_ = (c.m.r32((l_node_4).wrapping_add(32))).wrapping_mul(4u32); F::memset(c, a0_, 0i32, a2_) }; };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((15i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_node_4).wrapping_add(36))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (l_node_4).wrapping_add(48); c.m.w32(a_, v_); };
    { let v_ = fid::HudNotice_DrawNode; let a_ = (l_node_4).wrapping_add(52); c.m.w32(a_, v_); };
    l_node_4 = ({ let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((1i32) as u32)).wrapping_mul(56));
    { let v_ = l_cx_6; let a_ = (l_node_4).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
    { let v_ = l_cy_7; let a_ = (l_node_4).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
    { let v_ = ((3i32) as u32); let a_ = (l_node_4).wrapping_add(32); c.m.w32(a_, v_); };
    { let v_ = F::Heap_Alloc(c, (((((3i32) as u32)).wrapping_mul(4u32)) as i32), ((32i32) as u32), 0i32, 2i32); let a_ = (l_node_4).wrapping_add(36); c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32((l_node_4).wrapping_add(36)); let a2_ = (c.m.r32((l_node_4).wrapping_add(32))).wrapping_mul(4u32); F::memset(c, a0_, 0i32, a2_) }; };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((0i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_node_4).wrapping_add(36))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((1i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_node_4).wrapping_add(36))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((13i32) as u32)).wrapping_mul(28)); let a_ = (c.m.r32((l_node_4).wrapping_add(36))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = fid::HudNotice_UpdateReady; let a_ = (l_node_4).wrapping_add(48); c.m.w32(a_, v_); };
    { let v_ = fid::HudNotice_DrawSpin; let a_ = (l_node_4).wrapping_add(52); c.m.w32(a_, v_); };
    l_node_4 = ({ let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((0i32) as u32)).wrapping_mul(56));
    { let v_ = 0i32; let a_ = (l_node_4).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
    { let v_ = 0i32; let a_ = (l_node_4).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
    { let v_ = ((2i32) as u32); let a_ = (l_node_4).wrapping_add(40); c.m.w32(a_, v_); };
    { let v_ = F::Heap_Alloc(c, (((((2i32) as u32)).wrapping_mul(4u32)) as i32), ((32i32) as u32), 0i32, 2i32); let a_ = (l_node_4).wrapping_add(44); c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32((l_node_4).wrapping_add(44)); let a2_ = (c.m.r32((l_node_4).wrapping_add(40))).wrapping_mul(4u32); F::memset(c, a0_, 0i32, a2_) }; };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((1i32) as u32)).wrapping_mul(56)); let a_ = (c.m.r32((l_node_4).wrapping_add(44))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((3i32) as u32)).wrapping_mul(56)); let a_ = (c.m.r32((l_node_4).wrapping_add(44))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (l_node_4).wrapping_add(48); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (l_node_4).wrapping_add(52); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((0i32) as u32)).wrapping_mul(56)); let a_ = p_out_1; c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudNotice_Init", &PROBE_N, &[], true); }
    r_
}

pub fn HudNotice_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudNotice_Term", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let r_ = 'f: {
    if ((((({ let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(4); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = { let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(4); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
    }
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (4i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((((({ let a_ = (({ let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((l_i_1) as u32)).wrapping_mul(56))).wrapping_add(36); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
                    { let a0_ = { let a_ = (({ let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((l_i_1) as u32)).wrapping_mul(56))).wrapping_add(36); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
                }
                if ((((({ let a_ = (({ let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((l_i_1) as u32)).wrapping_mul(56))).wrapping_add(44); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
                    { let a0_ = { let a_ = (({ let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((l_i_1) as u32)).wrapping_mul(56))).wrapping_add(44); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
                }
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
    if ((((({ let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(8); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = { let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(8); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
    }
    if (((((c.m.r32(sym::gHudNotice)) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32(sym::gHudNotice); F::Heap_Free(c, a0_) };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudNotice_Term", &PROBE_N, &[], true); }
    r_
}

pub fn HudNotice_Reset(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudNotice_Reset", &PROBE_N, &[], false); }
    let mut l_node_1: u32 = 0;
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
    { let v_ = (1i32).wrapping_neg(); let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
    { let _ = { let a0_ = (c.m.r32(sym::gHudNotice)).wrapping_add(20); F::memset(c, a0_, 0i32, 24u32) }; };
    { let _ = { let a0_ = (c.m.r32(sym::gHudNotice)).wrapping_add(44); F::memset(c, a0_, 0i32, 24u32) }; };
    { let a0_ = ({ let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((13i32) as u32)).wrapping_mul(28)); F::HudSprite_Show(c, a0_, 0i32) };
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < (12i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let a0_ = ({ let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(4); c.m.r32(a_) }).wrapping_add(((((l_i_2).wrapping_add(1i32)) as u32)).wrapping_mul(28)); F::HudSprite_Show(c, a0_, 0i32) };
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
    { let a0_ = ({ let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((0i32) as u32)).wrapping_mul(28)); F::HudSprite_Show(c, a0_, 0i32) };
    l_node_1 = ({ let a_ = (c.m.r32(sym::gHudNotice)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((1i32) as u32)).wrapping_mul(56));
    { let v_ = F32(0x00000000); let a_ = (l_node_1).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x00000000); let a_ = (l_node_1).wrapping_add(8); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x00000000); let a_ = (l_node_1).wrapping_add(12); c.m.w32(a_, (v_).0); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudNotice_Reset", &PROBE_N, &[], true); }
    r_
}
