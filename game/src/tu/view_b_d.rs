#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn view_b_d__ShenScene_PlaySe(c: &mut Ctx, mut p_id_1: i32, mut p_volume_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "view_b_d__ShenScene_PlaySe", &PROBE_N, &[(p_id_1) as u32, (p_volume_2) as u32], false); }
    let r_ = 'f: {
    { let _ = F::Snd_PlaySeEx(c, ((2i32) as u32), p_id_1, p_volume_2, 0i32, 0i32); };
    { let _ = F::Snd_PlaySeEx(c, ((2i32) as u32), p_id_1, p_volume_2, 0i32, 0i32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "view_b_d__ShenScene_PlaySe", &PROBE_N, &[], true); }
    r_
}

pub fn view_b_d__ShenScene_SetBlur(c: &mut Ctx, mut p_a3_1: u8, mut p_a0_2: u8, mut p_a1_3: u8, mut p_a2_4: u8) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "view_b_d__ShenScene_SetBlur", &PROBE_N, &[(p_a3_1) as u32, (p_a0_2) as u32, (p_a1_3) as u32, (p_a2_4) as u32], false); }
    let r_ = 'f: {
    F::StgBlur_SetColor3Rgba(c, 0i32, ((128i32) as u8), ((128i32) as u8), ((128i32) as u8), p_a3_1);
    F::StgBlur_SetColor0Rgba(c, 0i32, ((128i32) as u8), ((128i32) as u8), ((128i32) as u8), p_a0_2);
    F::StgBlur_SetColor1Rgba(c, 0i32, ((128i32) as u8), ((128i32) as u8), ((128i32) as u8), p_a1_3);
    F::StgBlur_SetColor2Rgba(c, 0i32, ((128i32) as u8), ((128i32) as u8), ((128i32) as u8), p_a2_4);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "view_b_d__ShenScene_SetBlur", &PROBE_N, &[], true); }
    r_
}

pub fn ShenScene_StepSeq(c: &mut Ctx, mut p_seq_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ShenScene_StepSeq", &PROBE_N, &[(p_seq_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(176) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 176);
    let mut l_lo_2: F32 = F32(0);
    let mut l_hi_3: F32 = F32(0);
    let mut l_actor_4: u32 = 0;
    let mut l_ramp_5: u32 = 0;
    let mut l_work_6: u32 = 0;
    let mut l_shake_7: F32 = F32(0);
    let mut l_zero_8: F32 = F32(0);
    let mut l_half_9: F32 = F32(0);
    let mut l_dragon_10: i32 = 0;
    let mut l_frame_11: i32 = 0;
    let mut l_volume_12: i32 = 0;
    let mut l_dragon_13: u32 = 0;
    let mut l_balls_14: u32 = 0;
    let mut l_w_15: u32 = 0;
    let mut l_balls_16: u32 = 0;
    let mut l_level_17: F32 = F32(0);
    let r_ = 'f: {
        's1: {
            let sv18: i32 = (c.m.r32(p_seq_1) as i32);
            'k9: {
            'k8: {
            'k7: {
            'k6: {
            'k5: {
            'k4: {
            'k3: {
            'k2: {
                match sv18 { 0i32 => break 'k2, 1i32 => break 'k3, 2i32 => break 'k4, 3i32 => break 'k5, 4i32 => break 'k6, 5i32 => break 'k7, 6i32 => break 'k8, _ => break 'k9, }
            }
            {
                F::Mtx_StoreIdentity(c, fp);
                { let _ = F::ShenScene_NextCam(c, p_seq_1); };
                l_dragon_13 = ((c.m.r32(sym::gShenScene)).wrapping_add(12)).wrapping_add((((0i32) as u32)).wrapping_mul(40));
                { let r_ = (((!(33554432i32))) as u32); let a_ = (c.m.r32((l_dragon_13).wrapping_add(12))).wrapping_add(((2624i32) as u32)); let n_ = ((c.m.r32(a_)) & (r_)); c.m.w32(a_, n_); };
                { let a0_ = c.m.r32((l_dragon_13).wrapping_add(12)); F::BtlObjXf_SetMtx(c, a0_, fp) };
                { let a0_ = c.m.r32((l_dragon_13).wrapping_add(12)); F::BtlObjAnim_PlayModel(c, a0_, 0i32, 2i32) };
                { let a0_ = c.m.r32((l_dragon_13).wrapping_add(12)); F::BtlObj_SetUnkB30(c, a0_, 64i32) };
                l_w_15 = c.m.r32(sym::gShenScene);
                l_balls_14 = ((l_w_15).wrapping_add(12)).wrapping_add((((1i32) as u32)).wrapping_mul(40));
                { let r_ = (((!(33554432i32))) as u32); let a_ = (c.m.r32((l_balls_14).wrapping_add(12))).wrapping_add(((2624i32) as u32)); let n_ = ((c.m.r32(a_)) & (r_)); c.m.w32(a_, n_); };
                { let a0_ = c.m.r32((l_balls_14).wrapping_add(12)); F::BtlObjXf_SetMtx(c, a0_, fp) };
                { let a0_ = c.m.r32((l_balls_14).wrapping_add(12)); F::BtlObjAnim_PlayModel(c, a0_, 0i32, 2i32) };
                F::Ramp_Start(c, (((l_w_15).wrapping_add(12)).wrapping_add((((1i32) as u32)).wrapping_mul(40))).wrapping_add(16), F32(0x3f000000), F32(0x00000000), F32(0x42fe0000));
                { let a0_ = c.m.r32((l_balls_14).wrapping_add(12)); let a1_ = ((((rt::f2u(F32(c.m.r32(((l_balls_14).wrapping_add(16)).wrapping_add(16))))) as u8)) as i32); F::BtlObj_SetUnkB30(c, a0_, a1_) };
                { let a_ = p_seq_1; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            }
            l_ramp_5 = (((c.m.r32(sym::gShenScene)).wrapping_add(12)).wrapping_add((((1i32) as u32)).wrapping_mul(40))).wrapping_add(16);
            l_actor_4 = ((c.m.r32(sym::gShenScene)).wrapping_add(12)).wrapping_add((((1i32) as u32)).wrapping_mul(40));
            if ((F::Ramp_Step(c, l_ramp_5)) != 0) {
                if (((rt::feq(F32(c.m.r32(((l_actor_4).wrapping_add(16)).wrapping_add(16))), F32(0x00000000)) as i32)) != 0) {
                    F::Ramp_Start(c, l_ramp_5, F32(0x3f000000), F32(0x00000000), F32(0x42fe0000));
                } else {
                    F::Ramp_Start(c, l_ramp_5, F32(0x3f000000), F32(0x42fe0000), F32(0x00000000));
                }
            }
            l_lo_2 = F32(0x00000000);
            l_hi_3 = F32(0x3f800000);
            { let a0_ = c.m.r32((l_actor_4).wrapping_add(12)); let a1_ = ((((rt::f2u(F32(c.m.r32(((l_actor_4).wrapping_add(16)).wrapping_add(16))))) as u8)) as i32); F::BtlObj_SetUnkB30(c, a0_, a1_) };
            if (((rt::fge(F::DemoCam_GetTime(c), F32(0x43b10000)) as i32)) != 0) {
                { let s_ = sym::view_b_d__ShenScene_StepSeq__c; let d_ = fp.wrapping_add(64); rt::memcpy(c, d_, s_, 16); };
                F::view_b_d__ShenScene_SetBlur(c, ((64i32) as u8), ((0i32) as u8), ((rt::f2u(l_lo_2)) as u8), ((rt::f2u(rt::fmul(l_hi_3, F32(0x43000000)))) as u8));
                F::StgBlur_SetCenter(c, 0i32, fp.wrapping_add(64), 0i32);
            }
            if ((((!((F::ShenScene_IsCamEnd(c)) != 0)) as i32)) != 0) {
                break 's1;
            }
            { let a_ = p_seq_1; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
            }
            {
                l_work_6 = c.m.r32(sym::gShenScene);
                l_zero_8 = F32(0x00000000);
                l_half_9 = F32(0x3f000000);
                { let a0_ = c.m.r32((((l_work_6).wrapping_add(12)).wrapping_add((((0i32) as u32)).wrapping_mul(40))).wrapping_add(12)); F::BtlObjAnim_PlayModel(c, a0_, 1i32, 2i32) };
                F::Ramp_Start(c, (((l_work_6).wrapping_add(12)).wrapping_add((((0i32) as u32)).wrapping_mul(40))).wrapping_add(16), F32(0x40000000), F32(0x42800000), l_zero_8);
                { let r_ = (((!(2i32))) as u32); let a_ = ({ let a_ = (((c.m.r32(sym::gShenScene)).wrapping_add(12)).wrapping_add((((1i32) as u32)).wrapping_mul(40))).wrapping_add(12); c.m.r32(a_) }).wrapping_add(((2624i32) as u32)); let n_ = ((c.m.r32(a_)) & (r_)); c.m.w32(a_, n_); };
                {
                    rt::memset(c, fp.wrapping_add(80), 0, 48);
                    rt::memset(c, fp.wrapping_add(80).wrapping_add(0), 0, 16);
                    { let v_ = F32(0x3f5c28f5); c.m.w32(fp.wrapping_add(80).wrapping_add(0).wrapping_add(0), (v_).0); }
                    { let v_ = F32(0xc2b5f958); c.m.w32(fp.wrapping_add(80).wrapping_add(0).wrapping_add(4), (v_).0); }
                    { let v_ = F32(0xc198e353); c.m.w32(fp.wrapping_add(80).wrapping_add(0).wrapping_add(8), (v_).0); }
                    { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(80).wrapping_add(0).wrapping_add(12), (v_).0); }
                    rt::memset(c, fp.wrapping_add(80).wrapping_add(16), 0, 16);
                    { let v_ = F32(0xbf641893); c.m.w32(fp.wrapping_add(80).wrapping_add(16).wrapping_add(0), (v_).0); }
                    { let v_ = F32(0xc329bc6a); c.m.w32(fp.wrapping_add(80).wrapping_add(16).wrapping_add(4), (v_).0); }
                    { let v_ = F32(0xc2051581); c.m.w32(fp.wrapping_add(80).wrapping_add(16).wrapping_add(8), (v_).0); }
                    { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(80).wrapping_add(16).wrapping_add(12), (v_).0); }
                    rt::memset(c, fp.wrapping_add(80).wrapping_add(32), 0, 16);
                    { let v_ = F32(0xbecc49ba); c.m.w32(fp.wrapping_add(80).wrapping_add(32).wrapping_add(0), (v_).0); }
                    { let v_ = F32(0xc023020c); c.m.w32(fp.wrapping_add(80).wrapping_add(32).wrapping_add(4), (v_).0); }
                    { let v_ = F32(0xc125ba5e); c.m.w32(fp.wrapping_add(80).wrapping_add(32).wrapping_add(8), (v_).0); }
                    { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(80).wrapping_add(32).wrapping_add(12), (v_).0); }
                    rt::memset(c, fp.wrapping_add(128), 0, 48);
                    rt::memset(c, fp.wrapping_add(128).wrapping_add(0), 0, 16);
                    { let v_ = F32(0x412b851e); c.m.w32(fp.wrapping_add(128).wrapping_add(0).wrapping_add(0), (v_).0); }
                    { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(128).wrapping_add(0).wrapping_add(4), (v_).0); }
                    { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(128).wrapping_add(0).wrapping_add(8), (v_).0); }
                    { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(128).wrapping_add(0).wrapping_add(12), (v_).0); }
                    rt::memset(c, fp.wrapping_add(128).wrapping_add(16), 0, 16);
                    { let v_ = F32(0x41863958); c.m.w32(fp.wrapping_add(128).wrapping_add(16).wrapping_add(0), (v_).0); }
                    { let v_ = F32(0xbf52b020); c.m.w32(fp.wrapping_add(128).wrapping_add(16).wrapping_add(4), (v_).0); }
                    { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(128).wrapping_add(16).wrapping_add(8), (v_).0); }
                    { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(128).wrapping_add(16).wrapping_add(12), (v_).0); }
                    rt::memset(c, fp.wrapping_add(128).wrapping_add(32), 0, 16);
                    { let v_ = F32(0x40a48b43); c.m.w32(fp.wrapping_add(128).wrapping_add(32).wrapping_add(0), (v_).0); }
                    { let v_ = F32(0x40646a7e); c.m.w32(fp.wrapping_add(128).wrapping_add(32).wrapping_add(4), (v_).0); }
                    { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(128).wrapping_add(32).wrapping_add(8), (v_).0); }
                    { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(128).wrapping_add(32).wrapping_add(12), (v_).0); }
                    l_dragon_10 = { let a_ = (c.m.r32(sym::gShenScene)).wrapping_add(124); (c.m.r32(a_) as i32) };
                    { let v_ = rt::fdiv(rt::fmul(F32(c.m.r32((fp.wrapping_add(128)).wrapping_add((((l_dragon_10) as u32)).wrapping_mul(16)))), F32(0x40490fda)), F32(0x43340000)); let a_ = (fp.wrapping_add(128)).wrapping_add((((l_dragon_10) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fdiv(rt::fmul(F32(c.m.r32(((fp.wrapping_add(128)).wrapping_add((((l_dragon_10) as u32)).wrapping_mul(16))).wrapping_add(4))), F32(0x40490fda)), F32(0x43340000)); let a_ = ((fp.wrapping_add(128)).wrapping_add((((l_dragon_10) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fdiv(rt::fmul(F32(c.m.r32(((fp.wrapping_add(128)).wrapping_add((((l_dragon_10) as u32)).wrapping_mul(16))).wrapping_add(8))), F32(0x40490fda)), F32(0x43340000)); let a_ = ((fp.wrapping_add(128)).wrapping_add((((l_dragon_10) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_).0); };
                    F::DemoCam_SetFixedPose(c, (fp.wrapping_add(80)).wrapping_add((((l_dragon_10) as u32)).wrapping_mul(16)), (fp.wrapping_add(128)).wrapping_add((((l_dragon_10) as u32)).wrapping_mul(16)));
                    F::DemoCam_AddShake(c, F32(0x3f800000));
                    { let v_ = l_zero_8; let a_ = ((fp.wrapping_add(80)).wrapping_add((((l_dragon_10) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_).0); };
                    { let v_ = l_zero_8; let a_ = (fp.wrapping_add(80)).wrapping_add((((l_dragon_10) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
                    { let _ = F::ScrWarp_Spawn(c, 0i32, (fp.wrapping_add(80)).wrapping_add((((l_dragon_10) as u32)).wrapping_mul(16)), F32(0x40733333), F32(0x41200000), F32(0x42480000), F32(0x41200000), l_half_9); };
                }
                F::ScrXfade_RequestCapture(c);
                F::ScrXfade_Start(c, 1i32, l_half_9);
                F::ShenScene_SetState(c, 1i32);
                { let a_ = p_seq_1; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
            }
            }
            l_ramp_5 = (((c.m.r32(sym::gShenScene)).wrapping_add(12)).wrapping_add((((0i32) as u32)).wrapping_mul(40))).wrapping_add(16);
            l_actor_4 = ((c.m.r32(sym::gShenScene)).wrapping_add(12)).wrapping_add((((0i32) as u32)).wrapping_mul(40));
            if ((F::Ramp_Step(c, l_ramp_5)) != 0) {
                if (((rt::feq(F32(c.m.r32(((l_actor_4).wrapping_add(16)).wrapping_add(16))), F32(0x00000000)) as i32)) != 0) {
                    F::Ramp_Start(c, l_ramp_5, F32(0x40000000), F32(0x00000000), F32(0x42800000));
                } else {
                    F::Ramp_Start(c, l_ramp_5, F32(0x40000000), F32(0x42800000), F32(0x00000000));
                }
            }
            { let a0_ = c.m.r32((l_actor_4).wrapping_add(12)); let a1_ = ((((rt::f2u(F32(c.m.r32(((l_actor_4).wrapping_add(16)).wrapping_add(16))))) as u8)) as i32); F::BtlObj_SetUnkB30(c, a0_, a1_) };
            {
                { let s_ = sym::view_b_d__ShenScene_StepSeq__c_2; let d_ = fp.wrapping_add(64); rt::memcpy(c, d_, s_, 16); };
                l_shake_7 = F::DemoCam_GetShakeTime(c);
                F::view_b_d__ShenScene_SetBlur(c, ((rt::f2u(rt::fmul(l_shake_7, F32(0x42c00000)))) as u8), ((rt::f2u(rt::fmul(l_shake_7, F32(0x00000000)))) as u8), ((rt::f2u(rt::fmul(l_shake_7, F32(0x42500000)))) as u8), ((rt::f2u(rt::fmul(l_shake_7, F32(0x43000000)))) as u8));
                F::StgBlur_SetCenter(c, 0i32, fp.wrapping_add(64), 0i32);
            }
            if (((((F::ShenScene_GetState(c)) != (3i32)) as i32)) != 0) {
                break 's1;
            }
            F::ShenScene_SetState(c, 0i32);
            F::ScrXfade_RequestCapture(c);
            F::ScrXfade_Start(c, 1i32, F32(0x3f800000));
            { let a_ = p_seq_1; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
            break 's1;
            }
            F::DemoCam_ClearFixed(c);
            { let _ = F::ShenScene_NextCam(c, p_seq_1); };
            { let r_ = (((!(2i32))) as u32); let a_ = ({ let a_ = (((c.m.r32(sym::gShenScene)).wrapping_add(12)).wrapping_add((((0i32) as u32)).wrapping_mul(40))).wrapping_add(12); c.m.r32(a_) }).wrapping_add(((2624i32) as u32)); let n_ = ((c.m.r32(a_)) & (r_)); c.m.w32(a_, n_); };
            {
                l_balls_16 = ((c.m.r32(sym::gShenScene)).wrapping_add(12)).wrapping_add((((1i32) as u32)).wrapping_mul(40));
                { let r_ = ((2i32) as u32); let a_ = (c.m.r32((l_balls_16).wrapping_add(12))).wrapping_add(((2624i32) as u32)); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
                { let a0_ = c.m.r32((l_balls_16).wrapping_add(12)); F::BtlObjAnim_PlayModel(c, a0_, 1i32, 1i32) };
                { let a0_ = c.m.r32((l_balls_16).wrapping_add(12)); F::BtlObj_SetUnkB30(c, a0_, 127i32) };
            }
            { let a_ = p_seq_1; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
            }
            if ((F::ShenScene_IsCamEnd(c)) != 0) {
                F::Fade_Start(c, 0i32, 0i32, F32(0x3f800000));
                { let a_ = p_seq_1; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
            }
            break 's1;
            }
            if ((F::Fade_IsDone(c, 0i32)) != 0) {
                break 'f 1i32;
            }
            break 's1;
            }
            break 'f 1i32;
        }
        l_frame_11 = { let a_ = (c.m.r32(sym::gShenScene)).wrapping_add(132); (c.m.r32(a_) as i32) };
        's10: {
            let sv19: i32 = (c.m.r32((p_seq_1).wrapping_add(4)) as i32);
            match sv19 {
                0i32 => {
                    if ((((!((((((((c.m.r32(p_seq_1) as i32)) as u32)) < (((2i32) as u32))) as i32)) != 0)) as i32)) != 0) {
                        break 's10;
                    }
                    if (((rt::flt(F::DemoCam_GetTime(c), F32(0x43b10000)) as i32)) != 0) {
                        l_level_17 = rt::fsub(F32(0x3f800000), rt::fdiv(F::DemoCam_GetTime(c), F32(0x43d20000)));
                        if (((((rt::rem_i32(l_frame_11, 27i32)) == (0i32)) as i32)) != 0) {
                            l_volume_12 = rt::f2i(rt::fmul(l_level_17, F32(0x42fe0000)));
                            F::view_b_d__ShenScene_PlaySe(c, 61i32, l_volume_12);
                        }
                    } else {
                        F::view_b_d__ShenScene_PlaySe(c, 62i32, 127i32);
                        { let a_ = (p_seq_1).wrapping_add(4); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    }
                    break 's10;
                }
                1i32 => {
                    if ((((((c.m.r32(p_seq_1) as i32)) == (4i32)) as i32)) != 0) {
                        F::view_b_d__ShenScene_PlaySe(c, 63i32, 127i32);
                        { let a_ = (p_seq_1).wrapping_add(4); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    }
                    break 's10;
                }
                2i32 => {
                    if (((rt::fge(F::DemoCam_GetTime(c), F32(0x43a10000)) as i32)) != 0) {
                        F::view_b_d__ShenScene_PlaySe(c, 64i32, 127i32);
                        { let a_ = (p_seq_1).wrapping_add(4); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    }
                    break 's10;
                }
                _ => {
                }
            }
        }
        break 'f 0i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "ShenScene_StepSeq", &PROBE_N, &[], true); }
    r_
}

pub fn ShenScene_StepLoad(c: &mut Ctx, mut p_job_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ShenScene_StepLoad", &PROBE_N, &[(p_job_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_res_2: u32 = 0;
    let mut l_actor_3: u32 = 0;
    let mut l_i_4: i32 = 0;
    let mut l_slot_5: u32 = 0;
    let mut l_work_6: u32 = 0;
    let mut l_a_7: u32 = 0;
    let r_ = 'f: {
        l_res_2 = (c.m.r32(sym::gCommonRes)).wrapping_add(32);
        's1: {
            let sv8: i32 = (c.m.r32((p_job_1).wrapping_add(8)) as i32);
            match sv8 {
                0i32 => {
                    {
                        rt::memset(c, fp, 0, 24);
                        rt::memset(c, fp.wrapping_add(0), 0, 8);
                        { let v_ = 3388i32; c.m.w32(fp.wrapping_add(0).wrapping_add(0), (v_) as u32); }
                        { let v_ = 3398i32; c.m.w32(fp.wrapping_add(0).wrapping_add(4), (v_) as u32); }
                        rt::memset(c, fp.wrapping_add(8), 0, 8);
                        { let v_ = 3389i32; c.m.w32(fp.wrapping_add(8).wrapping_add(0), (v_) as u32); }
                        { let v_ = 3398i32; c.m.w32(fp.wrapping_add(8).wrapping_add(4), (v_) as u32); }
                        rt::memset(c, fp.wrapping_add(16), 0, 8);
                        { let v_ = 3397i32; c.m.w32(fp.wrapping_add(16).wrapping_add(0), (v_) as u32); }
                        { let v_ = 3399i32; c.m.w32(fp.wrapping_add(16).wrapping_add(4), (v_) as u32); }
                        { let v_ = { let a0_ = ({ let a_ = (c.m.r32(sym::gShenScene)).wrapping_add(124); (c.m.r32(a_) as i32) }).wrapping_add(404i32); let _ = 0i32; F::File_Request(c, a0_, ((0i32) as u32)) }; let a_ = (l_res_2).wrapping_add(4); c.m.w32(a_, v_); };
                        {
                            l_i_4 = 0i32;
                            'l2: loop {
                                if !(((((l_i_4) < (2i32)) as i32)) != 0) { break 'l2; }
                                'c3: {
                                    l_a_7 = ((c.m.r32(sym::gShenScene)).wrapping_add(12)).wrapping_add((((l_i_4) as u32)).wrapping_mul(40));
                                    { let v_ = { let a1_ = { let a_ = ((fp).wrapping_add(((({ let a_ = (c.m.r32(sym::gShenScene)).wrapping_add(124); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(8))).wrapping_add((((l_i_4) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; F::BtlRes_Request(c, 0i32, a1_, (1i32).wrapping_neg(), (1i32).wrapping_neg()) }; let a_ = (l_a_7).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                                }
                                { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
                            }
                        }
                        { let a_ = (p_job_1).wrapping_add(8); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        break 's1;
                    }
                }
                1i32 => {
                    if (((((F::File_UpdateRequests(c)) == (0i32)) as i32)) != 0) {
                        break 'f 0i32;
                    }
                    {
                        l_i_4 = 0i32;
                        'l4: loop {
                            if !(((((l_i_4) < (2i32)) as i32)) != 0) { break 'l4; }
                            'c5: {
                                l_actor_3 = ((c.m.r32(sym::gShenScene)).wrapping_add(12)).wrapping_add((((l_i_4) as u32)).wrapping_mul(40));
                                l_slot_5 = { let a0_ = (c.m.r32((l_actor_3).wrapping_add(8)) as i32); F::BtlRes_GetSlot(c, a0_) };
                                if (((((l_i_4) == (0i32)) as i32)) != 0) {
                                    { let v_ = F::BtlObj_Create(c, 1i32, l_slot_5, 1i32); let a_ = (l_actor_3).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                                } else {
                                    { let v_ = F::BtlObj_Create(c, 3i32, l_slot_5, 1i32); let a_ = (l_actor_3).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                                }
                                { let v_ = { let a0_ = (c.m.r32((l_actor_3).wrapping_add(8)) as i32); F::BtlObj_Get(c, a0_) }; let a_ = (l_actor_3).wrapping_add(12); c.m.w32(a_, v_); };
                            }
                            { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
                        }
                    }
                    break 'f 1i32;
                }
                _ => {
                    break 'f 1i32;
                }
            }
        }
        break 'f 0i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "ShenScene_StepLoad", &PROBE_N, &[], true); }
    r_
}

pub fn ShenScene_InitCams(c: &mut Ctx, mut p_seq_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ShenScene_InitCams", &PROBE_N, &[(p_seq_1) as u32], false); }
    let mut l_actor_2: u32 = 0;
    let mut l_cams_3: u32 = 0;
    let mut l_i_4: i32 = 0;
    let r_ = 'f: {
    l_actor_2 = ((c.m.r32(sym::gShenScene)).wrapping_add(12)).wrapping_add((((0i32) as u32)).wrapping_mul(40));
    l_cams_3 = (p_seq_1).wrapping_add(12);
    {
        l_i_4 = 0i32;
        'l1: loop {
            if !(((((l_i_4) < (2i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let v_ = { let a_ = ((c.m.r32((l_actor_2).wrapping_add(12))).wrapping_add(((2312i32) as u32))).wrapping_add((((l_i_4) as u32)).wrapping_mul(4)); c.m.r32(a_) }; let a_ = ((p_seq_1).wrapping_add(12)).wrapping_add((((l_i_4) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
            }
            { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
        }
    }
    { let v_ = 0i32; let a_ = (p_seq_1).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
    { let v_ = c.m.r32(l_cams_3); let a_ = (p_seq_1).wrapping_add(8); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ShenScene_InitCams", &PROBE_N, &[], true); }
    r_
}

pub fn ShenScene_NextCam(c: &mut Ctx, mut p_seq_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ShenScene_NextCam", &PROBE_N, &[(p_seq_1) as u32], false); }
    let r_ = 'f: {
    if (((((c.m.r32((p_seq_1).wrapping_add(8))) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32((p_seq_1).wrapping_add(8)); F::DemoCam_SetAnim(c, a0_) };
        F::DemoCam_SetTime(c, F32(0x00000000));
        F::DemoCam_StartAuto(c);
        { let a_ = (p_seq_1).wrapping_add(20); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        if ((((((c.m.r32((p_seq_1).wrapping_add(20)) as i32)) >= (2i32)) as i32)) != 0) {
            { let v_ = ((0i32) as u32); let a_ = (p_seq_1).wrapping_add(8); c.m.w32(a_, v_); };
        } else {
            { let v_ = { let a_ = ((p_seq_1).wrapping_add(12)).wrapping_add(((((c.m.r32((p_seq_1).wrapping_add(20)) as i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) }; let a_ = (p_seq_1).wrapping_add(8); c.m.w32(a_, v_); };
        }
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "ShenScene_NextCam", &PROBE_N, &[], true); }
    r_
}

pub fn ShenScene_IsCamEnd(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ShenScene_IsCamEnd", &PROBE_N, &[], false); }
    let mut l_time_1: F32 = F32(0);
    let r_ = 'f: {
    l_time_1 = F::DemoCam_GetTime(c);
    if (((rt::fge(l_time_1, F::DemoCam_GetLength(c)) as i32)) != 0) {
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "ShenScene_IsCamEnd", &PROBE_N, &[], true); }
    r_
}

pub fn ShenScene_RunSeq(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ShenScene_RunSeq", &PROBE_N, &[], false); }
    let mut l_seq_1: u32 = 0;
    let r_ = 'f: {
    l_seq_1 = (c.m.r32(sym::gShenScene)).wrapping_add(92);
    if (((((c.m.r32((l_seq_1).wrapping_add(24))) != (((0i32) as u32))) as i32)) != 0) {
        if ((((({ let fp_ = c.m.r32((l_seq_1).wrapping_add(24)); ((D::call(c, fp_, &[((l_seq_1) as u64)])) as i32) }) == (0i32)) as i32)) != 0) {
            break 'f 0i32;
        }
        { let v_ = ((0i32) as u32); let a_ = (l_seq_1).wrapping_add(24); c.m.w32(a_, v_); };
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "ShenScene_RunSeq", &PROBE_N, &[], true); }
    r_
}

pub fn ShenScene_StartSeq(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ShenScene_StartSeq", &PROBE_N, &[], false); }
    let mut l_seq_1: u32 = 0;
    let r_ = 'f: {
    l_seq_1 = (c.m.r32(sym::gShenScene)).wrapping_add(92);
    { let _ = F::memset(c, l_seq_1, 0i32, 32u32); };
    { let v_ = fid::ShenScene_StepSeq; let a_ = (l_seq_1).wrapping_add(24); c.m.w32(a_, v_); };
    F::ShenScene_InitCams(c, l_seq_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ShenScene_StartSeq", &PROBE_N, &[], true); }
    r_
}

pub fn ShenScene_Nop(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ShenScene_Nop", &PROBE_N, &[], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ShenScene_Nop", &PROBE_N, &[], true); }
    r_
}

pub fn ShenScene_GetState(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ShenScene_GetState", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f { let a_ = ((c.m.r32(sym::gShenScene)).wrapping_add(92)).wrapping_add(28); (c.m.r32(a_) as i32) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "ShenScene_GetState", &PROBE_N, &[], true); }
    r_
}

pub fn ShenScene_SetState(c: &mut Ctx, mut p_state_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ShenScene_SetState", &PROBE_N, &[(p_state_1) as u32], false); }
    let mut l_seq_2: u32 = 0;
    let r_ = 'f: {
    l_seq_2 = (c.m.r32(sym::gShenScene)).wrapping_add(92);
    if (((((((((p_state_1) == (3i32)) as i32)) != 0) && (((((F::ShenScene_GetState(c)) != (1i32)) as i32)) != 0)) as i32)) != 0) {
        break 'f;
    }
    { let v_ = p_state_1; let a_ = (l_seq_2).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ShenScene_SetState", &PROBE_N, &[], true); }
    r_
}

pub fn ShenScene_Nop2(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ShenScene_Nop2", &PROBE_N, &[], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ShenScene_Nop2", &PROBE_N, &[], true); }
    r_
}

pub fn ShenScene_Update(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ShenScene_Update", &PROBE_N, &[], false); }
    let mut l_view_1: u32 = 0;
    let r_ = 'f: {
    if ((((!(((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0)) as i32)) != 0) {
        if ((F::ShenScene_RunSeq(c)) != 0) {
            F::ShenScene_SetState(c, 2i32);
        }
    }
    if ((F::DemoCam_IsActive(c)) != 0) {
        { let _ = F::DemoCam_Update(c); };
    }
    if ((((!(((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0)) as i32)) != 0) {
        F::Dbg_ProfMark(c, sym::gBattleProf);
        F::BtlObj_UpdateAll(c);
        F::BtlObj_UpdateVisibility(c, 0i32);
        F::BtlObj_FinishVisibility(c, 0i32);
        F::BtlStage_Update(c);
        F::BtlScene_Update(c);
        { let r_ = F32(0x40000000); let a_ = (c.m.r32(sym::gShenScene)).wrapping_add(128); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        { let a_ = (c.m.r32(sym::gShenScene)).wrapping_add(132); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        { let _ = F::Dbg_ProfColor(c, sym::gBattleProf, 2164260863u32); };
    }
    l_view_1 = c.m.r32(sym::gBtlCamView);
    F::Dbg_ProfMark(c, sym::gBattleProf);
    F::Gfx_MarkPass(c, 1i32);
    F::StgModel_Cull(c, l_view_1);
    F::StgModel_Draw(c, l_view_1);
    { let _ = F::Dbg_ProfColor(c, sym::gBattleProf, 2164211776u32); };
    F::Dbg_ProfMark(c, sym::gBattleProf);
    F::Gfx_MarkPass(c, 3i32);
    F::StgFx_DrawPreNoCheck(c);
    { let _ = F::Dbg_ProfColor(c, sym::gBattleProf, 2164260863u32); };
    F::Dbg_ProfMark(c, sym::gBattleProf);
    F::Gfx_MarkPass(c, 2i32);
    F::BtlObjDraw_Draw(c);
    { let _ = F::Dbg_ProfColor(c, sym::gBattleProf, 2151743296u32); };
    F::Dbg_ProfMark(c, sym::gBattleProf);
    F::Gfx_MarkPass(c, 3i32);
    F::StgFx_DrawNop(c);
    { let _ = F::Dbg_ProfColor(c, sym::gBattleProf, 2164260863u32); };
    F::Dbg_ProfMark(c, sym::gBattleProf);
    F::Gfx_MarkPass(c, 4i32);
    F::Gfx_AddDefaultEnv(c);
    F::BtlScene_Draw(c, 1i32);
    { let _ = F::Ot_Draw(c); };
    { let _ = F::Dbg_ProfColor(c, sym::gBattleProf, 2151694591u32); };
    F::Dbg_ProfMark(c, sym::gBattleProf);
    F::Gfx_MarkPass(c, 3i32);
    F::StgFx_DrawPost(c);
    { let _ = F::Dbg_ProfColor(c, sym::gBattleProf, 2164260863u32); };
    F::Dbg_ProfMark(c, sym::gBattleProf);
    F::Gfx_MarkPass(c, 3i32);
    F::StgFx_DrawOverlay(c);
    { let _ = F::Dbg_ProfColor(c, sym::gBattleProf, 2164260863u32); };
    F::Gfx_MarkPass(c, 0i32);
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "ShenScene_Update", &PROBE_N, &[], true); }
    r_
}

pub fn ShenScene_Init(c: &mut Ctx, mut p_dragon_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ShenScene_Init", &PROBE_N, &[(p_dragon_1) as u32], false); }
    let mut l_job_2: u32 = 0;
    let r_ = 'f: {
    { let r_ = (((!(256i32))) as u64); let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); let n_ = ((c.m.r64(a_)) & (r_)); c.m.w64(a_, n_); };
    { let v_ = F::Heap_Alloc(c, ((136u32) as i32), ((32i32) as u32), 0i32, 2i32); let a_ = sym::gShenScene; c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32(sym::gShenScene); F::memset(c, a0_, 0i32, 136u32) }; };
    { let v_ = p_dragon_1; let a_ = (c.m.r32(sym::gShenScene)).wrapping_add(124); c.m.w32(a_, (v_) as u32); };
    F::BtlObj_Init(c, 0i32);
    F::BtlObj_SetUnk44FC4(c, F32(0x40000000));
    F::Job_Clear(c);
    l_job_2 = c.m.r32(sym::gShenScene);
    { let _ = F::memset(c, l_job_2, 0i32, 12u32); };
    { let v_ = fid::ShenScene_StepLoad; let a_ = (l_job_2).wrapping_add(4); c.m.w32(a_, v_); };
    { let _ = F::Job_Push(c, l_job_2); };
    F::Load_RunBlocking(c);
    F::Battle_ClearWork(c);
    F::DemoCam_Init(c);
    F::StgModel_InitStage(c);
    F::StgFx_Init(c);
    F::Ot_Init(c);
    F::BtlScene_Init(c, 8i32);
    F::ShenScene_StartSeq(c);
    F::Fade_Start(c, 0i32, 1i32, F32(0x3f800000));
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ShenScene_Init", &PROBE_N, &[], true); }
    r_
}

pub fn ShenScene_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ShenScene_Term", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::ShenScene_Nop(c);
    { let a0_ = c.m.r32(sym::gShenScene); F::Heap_Free(c, a0_) };
    F::File_CancelRequests(c);
    F::Job_Clear(c);
    F::BtlObj_Term(c);
    F::DemoCam_Term(c);
    F::StgFx_Term(c);
    F::BtlStage_Term(c);
    F::BtlScene_Term(c);
    F::Ot_Term(c);
    F::Dma_ResetBuffers(c);
    F::Adx_StopAll(c);
    { let a0_ = { let a_ = ((c.m.r32(sym::gCommonRes)).wrapping_add(32)).wrapping_add(4); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
    F::Fade_ResetAll(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ShenScene_Term", &PROBE_N, &[], true); }
    r_
}
