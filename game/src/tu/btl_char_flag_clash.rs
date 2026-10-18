#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn BtlClash_BothInClashA(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlClash_BothInClashA", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if ((((((F::BtlCharApi_IsInClashA(c, 0i32)) != 0) && ((F::BtlCharApi_IsInClashA(c, 1i32)) != 0)) as i32)) != 0) {
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlClash_BothInClashA", &PROBE_N, &[], true); }
    r_
}

pub fn BtlClash_HoldClashA(c: &mut Ctx, mut p_player_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlClash_HoldClashA", &PROBE_N, &[(p_player_1) as u32], false); }
    let r_ = 'f: {
    if ((F::BtlCharApi_IsInClashA(c, p_player_1)) != 0) {
        { let a0_ = F::BtlChar_Get(c, p_player_1); F::BtlChar_SetHeldFlag(c, a0_, ((195i32) as u32)) };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlClash_HoldClashA", &PROBE_N, &[], true); }
    r_
}

pub fn BtlClash_BothInActionFA(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlClash_BothInActionFA", &PROBE_N, &[], false); }
    let mut l_action_1: i32 = 0;
    let r_ = 'f: {
    l_action_1 = { let a0_ = F::BtlChar_Get(c, 0i32); F::BtlAct_GetCurrent(c, a0_) };
    if (((((l_action_1) != (250i32)) as i32)) != 0) {
        break 'f 0i32;
    }
    break 'f ((({ let a0_ = F::BtlChar_Get(c, 1i32); F::BtlAct_GetCurrent(c, a0_) }) == (l_action_1)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlClash_BothInActionFA", &PROBE_N, &[], true); }
    r_
}

pub fn BtlClash_EndActionFA(c: &mut Ctx, mut p_player_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlClash_EndActionFA", &PROBE_N, &[(p_player_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_Get(c, p_player_1);
    if (((((F::BtlAct_GetCurrent(c, l_chr_2)) == (250i32)) as i32)) != 0) {
        F::BtlChar_SetFlag(c, l_chr_2, ((191i32) as u32));
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlClash_EndActionFA", &PROBE_N, &[], true); }
    r_
}

pub fn BtlClash_BothInActionFB(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlClash_BothInActionFB", &PROBE_N, &[], false); }
    let mut l_action_1: i32 = 0;
    let r_ = 'f: {
    l_action_1 = { let a0_ = F::BtlChar_Get(c, 0i32); F::BtlAct_GetCurrent(c, a0_) };
    if (((((l_action_1) != (251i32)) as i32)) != 0) {
        break 'f 0i32;
    }
    break 'f ((({ let a0_ = F::BtlChar_Get(c, 1i32); F::BtlAct_GetCurrent(c, a0_) }) == (l_action_1)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlClash_BothInActionFB", &PROBE_N, &[], true); }
    r_
}

pub fn BtlClash_BothInActionFC(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlClash_BothInActionFC", &PROBE_N, &[], false); }
    let mut l_action_1: i32 = 0;
    let r_ = 'f: {
    l_action_1 = { let a0_ = F::BtlChar_Get(c, 0i32); F::BtlAct_GetCurrent(c, a0_) };
    if (((((l_action_1) != (252i32)) as i32)) != 0) {
        break 'f 0i32;
    }
    break 'f ((({ let a0_ = F::BtlChar_Get(c, 1i32); F::BtlAct_GetCurrent(c, a0_) }) == (l_action_1)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlClash_BothInActionFC", &PROBE_N, &[], true); }
    r_
}

pub fn BtlClash_EndActionFC(c: &mut Ctx, mut p_player_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlClash_EndActionFC", &PROBE_N, &[(p_player_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_Get(c, p_player_1);
    if (((((F::BtlAct_GetCurrent(c, l_chr_2)) == (252i32)) as i32)) != 0) {
        F::BtlChar_SetFlag(c, l_chr_2, ((198i32) as u32));
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlClash_EndActionFC", &PROBE_N, &[], true); }
    r_
}

pub fn BtlClash_SetOrbitCut(c: &mut Ctx, mut p_player_1: i32, mut p_type_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlClash_SetOrbitCut", &PROBE_N, &[(p_player_1) as u32, (p_type_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_chr_3: u32 = 0;
    let mut l_nodeA_4: i32 = 0;
    let mut l_nodeB_5: i32 = 0;
    let mut l_pitch_6: F32 = F32(0);
    let mut l_yaw_7: F32 = F32(0);
    let mut l_dist_8: F32 = F32(0);
    let r_ = 'f: {
        l_nodeA_4 = 3i32;
        l_nodeB_5 = 17i32;
        l_chr_3 = F::BtlChar_Get(c, p_player_1);
        l_pitch_6 = F32(0x00000000);
        F::BtlOpp_GetDelta(c, l_chr_3, fp);
        l_yaw_7 = F32(0x00000000);
        l_dist_8 = F32(0x00000000);
        F::Vec3_Normalize(c, fp, fp);
        's1: {
            let sv9: i32 = p_type_2;
            match sv9 {
                0i32 => {
                    if (((((p_player_1) == (0i32)) as i32)) != 0) {
                        l_dist_8 = F32(0x40199999);
                    } else {
                        l_dist_8 = F32(0xc0199999);
                    }
                    l_pitch_6 = F32(0x00000000);
                    l_yaw_7 = { let a0_ = rt::fsub({ let a_ = (F::BtlChar_GetPos(c, l_chr_3)).wrapping_add(148); F32(c.m.r32(a_)) }, l_dist_8); F::BtlUtil_WrapAngle(c, a0_) };
                    l_dist_8 = { let a0_ = (c.m.r32((l_chr_3).wrapping_add(12)) as i32); F::BtlCharApi_GetHeight(c, a0_) };
                    { let r_ = F::EftStruggle_GetCamDist(c, p_player_1, p_type_2); let n_ = rt::fadd(l_dist_8, r_); l_dist_8 = n_; };
                    break 's1;
                }
                1i32 => {
                    if (((((p_player_1) == (0i32)) as i32)) != 0) {
                        l_dist_8 = F32(0x3f000000);
                    } else {
                        l_dist_8 = F32(0xbf000000);
                    }
                    l_pitch_6 = F32(0x00000000);
                    l_yaw_7 = { let a0_ = rt::fsub({ let a_ = (F::BtlChar_GetPos(c, l_chr_3)).wrapping_add(148); F32(c.m.r32(a_)) }, l_dist_8); F::BtlUtil_WrapAngle(c, a0_) };
                    l_dist_8 = { let a0_ = (c.m.r32((l_chr_3).wrapping_add(12)) as i32); F::BtlCharApi_GetHeight(c, a0_) };
                    { let r_ = F::EftStruggle_GetCamDist(c, p_player_1, p_type_2); let n_ = rt::fadd(l_dist_8, r_); l_dist_8 = n_; };
                    break 's1;
                }
                2i32 => {
                    l_dist_8 = F32(0x40490fda);
                    l_nodeA_4 = 48i32;
                    l_nodeB_5 = 48i32;
                    l_yaw_7 = { let a0_ = rt::fsub({ let a_ = (F::BtlChar_GetPos(c, l_chr_3)).wrapping_add(148); F32(c.m.r32(a_)) }, l_dist_8); F::BtlUtil_WrapAngle(c, a0_) };
                    l_pitch_6 = rt::fsub({ let a0_ = F32(c.m.r32((fp).wrapping_add(4))); F::Mathf_Asin(c, a0_) }, F32(0x3e4ccccc));
                    l_dist_8 = rt::fmul({ let a0_ = (c.m.r32((l_chr_3).wrapping_add(12)) as i32); F::BtlCharApi_GetHeight(c, a0_) }, F32(0x3ecccccc));
                    { let r_ = F::EftStruggle_GetCamDist(c, p_player_1, p_type_2); let n_ = rt::fadd(l_dist_8, r_); l_dist_8 = n_; };
                    break 's1;
                }
                _ => {
                }
            }
        }
        F::ChrCam_SetCut(c, l_chr_3, sym::D_002EC2A0, sym::D_002EC2A0, sym::D_002EC2A0, sym::D_002EC2A0, sym::D_002EC2A0, sym::D_002EC2A0, l_nodeA_4, l_yaw_7, F32(0x00000000), l_pitch_6, F32(0x00000000), l_dist_8, F32(0x00000000), l_nodeA_4, l_nodeB_5, l_nodeB_5, 1i32, 69i32);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlClash_SetOrbitCut", &PROBE_N, &[], true); }
    r_
}

pub fn BtlClash_SetMidCut(c: &mut Ctx, mut p_mid_1: u32, mut p_bias_2: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlClash_SetMidCut", &PROBE_N, &[(p_mid_1) as u32, (p_bias_2).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(64) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 64);
    let mut l_chr_3: u32 = 0;
    let mut l_ang_4: F32 = F32(0);
    let mut l_half_5: F32 = F32(0);
    let mut l_base_6: F32 = F32(0);
    let mut l_yaw_7: F32 = F32(0);
    let mut l_pitch_8: F32 = F32(0);
    let r_ = 'f: {
        l_chr_3 = F::BtlChar_Get(c, 0i32);
        l_ang_4 = F32(0x3fc90fda);
        F::BtlOpp_GetDelta(c, l_chr_3, fp);
        l_half_5 = F32(0x3f000000);
        l_ang_4 = rt::fmul(p_bias_2, l_ang_4);
        l_base_6 = F32(0x42480000);
        F::Vec3_Normalize(c, fp, fp);
        F::BtlCharApi_GetNodePos(c, 0i32, 17i32, fp.wrapping_add(32));
        l_ang_4 = rt::fmul(l_ang_4, l_half_5);
        F::BtlCharApi_GetNodePos(c, 1i32, 17i32, fp.wrapping_add(48));
        F::Vec4_Lerp(c, fp.wrapping_add(16), fp.wrapping_add(48), fp.wrapping_add(32), rt::fadd(rt::fmul(p_bias_2, F32(0x3f199999)), l_half_5));
        l_ang_4 = rt::fadd(l_ang_4, F32(0xbfc90fda));
        l_yaw_7 = { let a0_ = rt::fadd({ let a0_ = F32(c.m.r32(fp)); let a1_ = F32(c.m.r32((fp).wrapping_add(8))); F::Port_atan2f(c, a0_, a1_) }, l_ang_4); F::BtlUtil_WrapAngle(c, a0_) };
        l_pitch_8 = rt::fneg({ let a0_ = F32(c.m.r32((fp).wrapping_add(4))); F::Mathf_Asin(c, a0_) });
        { let r_ = p_bias_2; let n_ = rt::fmul(l_pitch_8, r_); l_pitch_8 = n_; };
        { let a12_ = rt::fadd(F::EftStruggle_GetMidDist(c), l_base_6); F::ChrCam_SetCut(c, l_chr_3, p_mid_1, sym::D_002EC2A0, p_mid_1, sym::D_002EC2A0, fp.wrapping_add(16), sym::D_002EC2A0, (1i32).wrapping_neg(), l_yaw_7, F32(0x00000000), l_pitch_8, F32(0x00000000), a12_, F32(0x00000000), (1i32).wrapping_neg(), (1i32).wrapping_neg(), (1i32).wrapping_neg(), 1i32, 69i32) };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlClash_SetMidCut", &PROBE_N, &[], true); }
    r_
}

pub fn BtlClash_SetPathCut(c: &mut Ctx, mut p_n_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlClash_SetPathCut", &PROBE_N, &[(p_n_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_path_2: u32 = 0;
    let mut l_p_3: u32 = 0;
    let r_ = 'f: {
        l_path_2 = F::BtlStage_GetPath(c, p_n_1);
        if (((((l_path_2) != (((0i32) as u32))) as i32)) != 0) {
            l_p_3 = c.m.r32((l_path_2).wrapping_add(4));
            { let v_ = F32(c.m.r32((l_p_3).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = fp; c.m.w32(a_, (v_).0); };
            { let v_ = F32(c.m.r32((l_p_3).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
            { let v_ = F32(c.m.r32((l_p_3).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x3f800000); let a_ = (fp).wrapping_add(12); c.m.w32(a_, (v_).0); };
            { let v_ = rt::fadd(F32(c.m.r32(fp)), { let a0_ = F32(c.m.r32((l_p_3).wrapping_add((((5i32) as u32)).wrapping_mul(4)))); F::Port_sinf(c, a0_) }); let a_ = fp.wrapping_add(16); c.m.w32(a_, (v_).0); };
            { let v_ = rt::fsub(F32(c.m.r32((fp).wrapping_add(4))), { let a0_ = F32(c.m.r32((l_p_3).wrapping_add((((4i32) as u32)).wrapping_mul(4)))); F::Port_sinf(c, a0_) }); let a_ = (fp.wrapping_add(16)).wrapping_add(4); c.m.w32(a_, (v_).0); };
            { let v_ = rt::fadd(F32(c.m.r32((fp).wrapping_add(8))), { let a0_ = F32(c.m.r32((l_p_3).wrapping_add((((5i32) as u32)).wrapping_mul(4)))); F::Port_cosf(c, a0_) }); let a_ = (fp.wrapping_add(16)).wrapping_add(8); c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x3f800000); let a_ = (fp.wrapping_add(16)).wrapping_add(12); c.m.w32(a_, (v_).0); };
            { let a0_ = F::BtlChar_Get(c, 0i32); F::ChrCam_SetCut(c, a0_, fp, sym::D_002EC2A0, fp, sym::D_002EC2A0, fp.wrapping_add(16), sym::D_002EC2A0, (1i32).wrapping_neg(), F32(0x00000000), F32(0x00000000), F32(0x00000000), F32(0x00000000), F32(0x00000000), F32(0x00000000), (1i32).wrapping_neg(), (1i32).wrapping_neg(), (1i32).wrapping_neg(), 1i32, 69i32) };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlClash_SetPathCut", &PROBE_N, &[], true); }
    r_
}

pub fn BtlClash_CalcBias(c: &mut Ctx, mut p_lead_1: i32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlClash_CalcBias", &PROBE_N, &[(p_lead_1) as u32], false); }
    let mut l_x_2: F32 = F32(0);
    let r_ = 'f: {
    l_x_2 = rt::fmul(rt::i2f(p_lead_1), F32(0x3d4ccccc));
    break 'f F::BtlUtil_ClampF(c, rt::fdiv(l_x_2, rt::fadd(rt::ffabs(l_x_2), F32(0x3f800000))), F32(0xbf4ccccc), F32(0x3f4ccccc));
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlClash_CalcBias", &PROBE_N, &[], true); }
    r_
}

pub fn BtlClash_UpdateA(c: &mut Ctx, mut p_state_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlClash_UpdateA", &PROBE_N, &[(p_state_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_roster_2: u32 = 0;
    let mut l_chr0_3: u32 = 0;
    let mut l_chr1_4: u32 = 0;
    let mut l_count0_5: i32 = 0;
    let mut l_c_6: u32 = 0;
    let mut l_count1_7: i32 = 0;
    let mut l_winner_8: u32 = 0;
    let mut l_loser_9: u32 = 0;
    let mut l_flags_10: i32 = 0;
    let mut l_cls_11: i32 = 0;
    let mut l_loserCls_12: i32 = 0;
    let mut l_damage_13: i32 = 0;
    let r_ = 'f: {
        l_roster_2 = c.m.r32(sym::gBtlChars);
        l_chr0_3 = F::BtlChar_Get(c, 0i32);
        l_chr1_4 = F::BtlChar_Get(c, 1i32);
        l_count0_5 = (c.m.r32((l_chr0_3).wrapping_add(3660)) as i32);
        l_c_6 = (l_roster_2).wrapping_add(80);
        l_count1_7 = (c.m.r32((l_chr1_4).wrapping_add(3660)) as i32);
        's1: {
            let sv14: i32 = p_state_1;
            match sv14 {
                1i32 => {
                    { let v_ = 0i32; let a_ = l_c_6; c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = (l_c_6).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let o_ = p_state_1; let n_ = (o_).wrapping_add(1); p_state_1 = n_; };
                    F::BtlCharSnd_PlayStream(c, l_chr0_3, 36144i32);
                    break 's1;
                }
                2i32 => {
                    if ((((((c.m.r32(l_c_6) as i32)) == (0i32)) as i32)) != 0) {
                        F::BtlChar_PlayVoice(c, l_chr0_3, 20i32);
                    }
                    if ((((((c.m.r32(l_c_6) as i32)) == (30i32)) as i32)) != 0) {
                        F::BtlChar_PlayVoice(c, l_chr1_4, 20i32);
                    }
                    if ((((((c.m.r32(l_c_6) as i32)) < (30i32)) as i32)) != 0) {
                        F::BtlClash_SetOrbitCut(c, 0i32, 0i32);
                    } else {
                        if ((((((c.m.r32(l_c_6) as i32)) < (60i32)) as i32)) != 0) {
                            F::BtlClash_SetOrbitCut(c, 1i32, 0i32);
                        } else {
                            if (((((l_count1_7) < (l_count0_5)) as i32)) != 0) {
                                { let a_ = (l_c_6).wrapping_add(8); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                            }
                            if (((((l_count0_5) < (l_count1_7)) as i32)) != 0) {
                                { let a_ = (l_c_6).wrapping_add(8); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                            }
                            if (((rt::flt(F32(c.m.r32((l_c_6).wrapping_add(12))), F32(0xbf23d70a)) as i32)) != 0) {
                                F::BtlClash_SetOrbitCut(c, 0i32, 1i32);
                            } else {
                                if (((rt::flt(F32(0x3f23d70a), F32(c.m.r32((l_c_6).wrapping_add(12)))) as i32)) != 0) {
                                    F::BtlClash_SetOrbitCut(c, 1i32, 1i32);
                                } else {
                                    { let a1_ = F32(c.m.r32((l_c_6).wrapping_add(12))); F::BtlClash_SetMidCut(c, (l_c_6).wrapping_add(16), a1_) };
                                }
                            }
                        }
                    }
                    { let a_ = l_c_6; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    if ((((((c.m.r32(l_c_6) as i32)) >= (106i32)) as i32)) != 0) {
                        { let v_ = 0i32; let a_ = l_c_6; c.m.w32(a_, (v_) as u32); };
                        { let o_ = p_state_1; let n_ = (o_).wrapping_add(1); p_state_1 = n_; };
                    }
                    break 's1;
                }
                3i32 => {
                    if ((((((c.m.r32(l_c_6) as i32)) == (0i32)) as i32)) != 0) {
                        if ((((((c.m.r32((l_c_6).wrapping_add(8)) as i32)) == (0i32)) as i32)) != 0) {
                            { let v_ = (if ((F::BtlChar_FrameMod(c, 2i32)) != 0) { 1i32 } else { (1i32).wrapping_neg() }); let a_ = (l_c_6).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                        }
                        if ((((((c.m.r32((l_c_6).wrapping_add(8)) as i32)) > (0i32)) as i32)) != 0) {
                            F::BtlChar_PlayVoice(c, l_chr0_3, 21i32);
                        } else {
                            F::BtlChar_PlayVoice(c, l_chr1_4, 21i32);
                        }
                    }
                    if ((((((c.m.r32((l_c_6).wrapping_add(8)) as i32)) > (0i32)) as i32)) != 0) {
                        F::BtlClash_SetOrbitCut(c, 0i32, 2i32);
                    } else {
                        F::BtlClash_SetOrbitCut(c, 1i32, 2i32);
                    }
                    { let a_ = l_c_6; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    if ((((((c.m.r32(l_c_6) as i32)) >= (16i32)) as i32)) != 0) {
                        { let v_ = 0i32; let a_ = l_c_6; c.m.w32(a_, (v_) as u32); };
                        { let o_ = p_state_1; let n_ = (o_).wrapping_add(1); p_state_1 = n_; };
                    }
                    break 's1;
                }
                4i32 => {
                    if ((((((c.m.r32((l_c_6).wrapping_add(8)) as i32)) > (0i32)) as i32)) != 0) {
                        { let r_ = 4i32; let a_ = (l_c_6).wrapping_add(8); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                    } else {
                        { let r_ = 4i32; let a_ = (l_c_6).wrapping_add(8); let n_ = ((c.m.r32(a_) as i32)).wrapping_sub(r_); c.m.w32(a_, (n_) as u32); };
                    }
                    { let a1_ = F32(c.m.r32((l_c_6).wrapping_add(12))); F::BtlClash_SetMidCut(c, (l_c_6).wrapping_add(16), a1_) };
                    if ((((({ let a0_ = (c.m.r32((l_c_6).wrapping_add(8)) as i32); (a0_).wrapping_abs() }) >= (71i32)) as i32)) != 0) {
                        { let v_ = 0i32; let a_ = l_c_6; c.m.w32(a_, (v_) as u32); };
                        { let o_ = p_state_1; let n_ = (o_).wrapping_add(1); p_state_1 = n_; };
                    }
                    break 's1;
                }
                5i32 => {
                    if ((((((c.m.r32(l_c_6) as i32)) == (0i32)) as i32)) != 0) {
                        l_flags_10 = 0i32;
                        l_winner_8 = l_chr0_3;
                        l_loser_9 = l_chr1_4;
                        if ((((!((((((c.m.r32((l_c_6).wrapping_add(8)) as i32)) > (0i32)) as i32)) != 0)) as i32)) != 0) {
                            if ((((((c.m.r32((l_c_6).wrapping_add(8)) as i32)) < (0i32)) as i32)) != 0) {
                                l_winner_8 = l_chr1_4;
                                l_loser_9 = l_chr0_3;
                            } else {
                                if (((((F::BtlChar_FrameMod(c, 2i32)) == (0i32)) as i32)) != 0) {
                                    l_winner_8 = l_chr1_4;
                                    l_loser_9 = l_chr0_3;
                                }
                            }
                        }
                        l_cls_11 = F::BtlAct_GetCurrentClass(c, l_winner_8);
                        l_loserCls_12 = F::BtlAct_GetCurrentClass(c, l_loser_9);
                        F::BtlChar_SetHeldFlag(c, l_winner_8, ((193i32) as u32));
                        F::BtlChar_SetHeldFlag(c, l_winner_8, ((195i32) as u32));
                        F::BtlChar_SetHeldFlag(c, l_loser_9, ((194i32) as u32));
                        if ((F::BtlSuper_IsThrow(c, l_winner_8, l_cls_11)) != 0) {
                            { let a3_ = ((((F::BtlSuper_GetFlags(c, l_winner_8, l_cls_11)) as u64)) as i32); { let _ = F::BtlColl_StartThrow(c, l_winner_8, l_loser_9, l_cls_11, ((((a3_) as i64 as u64)) as u32)); } };
                            F::BtlChar_SetHeldFlag(c, l_winner_8, ((160i32) as u32));
                            { let a0_ = (c.m.r32((l_winner_8).wrapping_add(12)) as i32); F::EftStruggle_End(c, a0_) };
                        } else {
                            l_damage_13 = F::BtlSuper_GetDamage(c, l_winner_8, l_cls_11, 0i32, 1i32);
                            { let r_ = rt::div_i32(F::BtlSuper_GetDamage(c, l_loser_9, l_loserCls_12, 0i32, 1i32), 2i32); let n_ = (l_damage_13).wrapping_add(r_); l_damage_13 = n_; };
                            's2: {
                                let sv15: i32 = l_cls_11;
                                match sv15 {
                                    2i32 => {
                                        l_flags_10 = 8388608i32;
                                        break 's2;
                                    }
                                    3i32 => {
                                        l_flags_10 = 16777216i32;
                                        break 's2;
                                    }
                                    4i32 => {
                                        l_flags_10 = 33554432i32;
                                        break 's2;
                                    }
                                    _ => {
                                    }
                                }
                            }
                            { let _ = F::BtlMember_Damage(c, l_loser_9, l_damage_13, l_flags_10); };
                            F::BtlChar_PlayVoice(c, l_loser_9, 22i32);
                            F::BtlCharSnd_PlayStream(c, l_loser_9, 36145i32);
                            F::EftStruggle_End(c, (1i32).wrapping_neg());
                        }
                        F::BtlChar_RaiseFirstClash(c, l_winner_8);
                        { let v_ = (c.m.r32(l_winner_8) as i32); let a_ = (l_c_6).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    }
                    if ((((((c.m.r32((l_c_6).wrapping_add(4)) as i32)) == (0i32)) as i32)) != 0) {
                        F::BtlClash_SetOrbitCut(c, 1i32, 0i32);
                    } else {
                        F::BtlClash_SetOrbitCut(c, 0i32, 0i32);
                    }
                    { let v_ = 0i32; let a_ = (l_c_6).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    p_state_1 = 0i32;
                    break 's1;
                }
                _ => {
                }
            }
        }
        { let v_ = { let a0_ = (c.m.r32((l_c_6).wrapping_add(8)) as i32); F::BtlClash_CalcBias(c, a0_) }; let a_ = (l_c_6).wrapping_add(12); c.m.w32(a_, (v_).0); };
        F::BtlCharApi_GetNodePos(c, 0i32, 17i32, fp);
        F::BtlCharApi_GetNodePos(c, 1i32, 17i32, fp.wrapping_add(16));
        { let a3_ = rt::fadd(rt::fmul(F32(c.m.r32((l_c_6).wrapping_add(12))), F32(0x3f000000)), F32(0x3f000000)); F::Vec4_Lerp(c, (l_c_6).wrapping_add(16), fp.wrapping_add(16), fp, a3_) };
        break 'f p_state_1;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlClash_UpdateA", &PROBE_N, &[], true); }
    r_
}

pub fn BtlClash_UpdateB(c: &mut Ctx, mut p_state_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlClash_UpdateB", &PROBE_N, &[(p_state_1) as u32], false); }
    let mut l_c_2: u32 = 0;
    let mut l_chr0_3: u32 = 0;
    let mut l_chr1_4: u32 = 0;
    let mut l_win_5: i32 = 0;
    let mut l_lose_6: i32 = 0;
    let mut l_winner_7: u32 = 0;
    let mut l_loser_8: u32 = 0;
    let r_ = 'f: {
    l_c_2 = (c.m.r32(sym::gBtlChars)).wrapping_add(112);
    l_chr0_3 = F::BtlChar_Get(c, 0i32);
    l_chr1_4 = F::BtlChar_Get(c, 1i32);
    's1: {
        let sv9: i32 = p_state_1;
        match sv9 {
            6i32 => {
                { let v_ = 0i32; let a_ = l_c_2; c.m.w32(a_, (v_) as u32); };
                p_state_1 = 7i32;
                F::ChrCam_RequestCut(c, l_chr0_3, 1i32, 6i32);
                break 's1;
            }
            7i32 => {
                if ((((((c.m.r32(l_c_2) as i32)) == (0i32)) as i32)) != 0) {
                    F::BtlChar_PlayVoice(c, l_chr0_3, 24i32);
                }
                if ((((((c.m.r32(l_c_2) as i32)) == (15i32)) as i32)) != 0) {
                    F::BtlChar_PlayVoice(c, l_chr1_4, 24i32);
                }
                { let a_ = l_c_2; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                if ((((((c.m.r32(l_c_2) as i32)) >= (76i32)) as i32)) != 0) {
                    { let v_ = 0i32; let a_ = l_c_2; c.m.w32(a_, (v_) as u32); };
                    p_state_1 = 8i32;
                }
                if ((((!((F::BtlClash_BothInActionFA(c)) != 0)) as i32)) != 0) {
                    p_state_1 = 0i32;
                    F::BtlClash_EndActionFA(c, 0i32);
                    F::BtlClash_EndActionFA(c, 1i32);
                }
                break 's1;
            }
            8i32 => {
                if ((((((c.m.r32(l_c_2) as i32)) == (0i32)) as i32)) != 0) {
                    if ((((((c.m.r32((l_chr1_4).wrapping_add(3664)) as i32)) < ((c.m.r32((l_chr0_3).wrapping_add(3664)) as i32))) as i32)) != 0) {
                        l_win_5 = 0i32;
                    } else {
                        if ((((((c.m.r32((l_chr0_3).wrapping_add(3664)) as i32)) < ((c.m.r32((l_chr1_4).wrapping_add(3664)) as i32))) as i32)) != 0) {
                            l_win_5 = 1i32;
                        } else {
                            l_win_5 = (((F::BtlChar_FrameMod(c, 2i32)) == (0i32)) as i32);
                        }
                    }
                    l_lose_6 = ((!((l_win_5) != 0)) as i32);
                    l_winner_7 = F::BtlChar_Get(c, l_win_5);
                    l_loser_8 = F::BtlChar_Get(c, l_lose_6);
                    F::BtlChar_SetFlag(c, l_winner_7, ((191i32) as u32));
                    F::BtlChar_SetFlag(c, l_loser_8, ((192i32) as u32));
                    F::BtlChar_PlayVoice(c, l_winner_7, 25i32);
                    F::BtlChar_RaiseFirstClash(c, l_winner_7);
                    F::ChrCam_EndCut(c, l_chr0_3);
                    { let v_ = l_win_5; let a_ = (l_c_2).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                }
                { let a_ = l_c_2; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                if ((((!((F::BtlClash_BothInActionFA(c)) != 0)) as i32)) != 0) {
                    p_state_1 = 0i32;
                }
                break 's1;
            }
            _ => {
            }
        }
    }
    break 'f p_state_1;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlClash_UpdateB", &PROBE_N, &[], true); }
    r_
}

pub fn BtlClash_UpdateC(c: &mut Ctx, mut p_state_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlClash_UpdateC", &PROBE_N, &[(p_state_1) as u32], false); }
    let mut l_c_2: u32 = 0;
    let mut l_chr0_3: u32 = 0;
    let mut l_chr1_4: u32 = 0;
    let mut l_path_5: u32 = 0;
    let mut l_winner_6: u32 = 0;
    let mut l_loser_7: u32 = 0;
    let mut l_prev_8: i32 = 0;
    let mut l_n_9: i32 = 0;
    let mut l_prev_10: i32 = 0;
    let r_ = 'f: {
    l_c_2 = (c.m.r32(sym::gBtlChars)).wrapping_add(124);
    l_chr0_3 = F::BtlChar_Get(c, 0i32);
    l_chr1_4 = F::BtlChar_Get(c, 1i32);
    's1: {
        let sv11: i32 = p_state_1;
        match sv11 {
            9i32 => {
                { let v_ = 0i32; let a_ = (l_c_2).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                { let v_ = (((rt::rem_u32((((((F::BtlChar_Rand(c)) as i64 as u64)) as u32)).wrapping_shr((2i32) as u32), (((F::BtlStage_GetPathCount(c)).wrapping_sub(1i32)) as u32))).wrapping_add(((1i32) as u32))) as i32); let a_ = (l_c_2).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                { let v_ = (if (((((rt::rem_u32((((((F::BtlChar_Rand(c)) as i64 as u64)) as u32)).wrapping_shr((2i32) as u32), ((10i32) as u32))) < (((7i32) as u32))) as i32)) != 0) { 2i32 } else { 1i32 }); let a_ = (l_c_2).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
                l_path_5 = { let a0_ = (c.m.r32((l_c_2).wrapping_add(8)) as i32); F::BtlStage_GetPath(c, a0_) };
                if (((((l_path_5) != (((0i32) as u32))) as i32)) != 0) {
                    { let v_ = (((rt::rem_u32((((((F::BtlChar_Rand(c)) as i64 as u64)) as u32)).wrapping_shr((2i32) as u32), ((((c.m.r32(l_path_5) as i32)).wrapping_sub((c.m.r32((l_path_5).wrapping_add(8)) as i32))) as u32))).wrapping_add((((c.m.r32((l_path_5).wrapping_add(8)) as i32)) as u32))) as i32); let a_ = (l_c_2).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                } else {
                    { let v_ = 0i32; let a_ = (l_c_2).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                }
                { let v_ = (1i32).wrapping_neg(); let a_ = (l_c_2).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                { let v_ = (((((((((F::BtlChar_Rand(c)) as i64 as u64)) as u32)).wrapping_shr((2i32) as u32)) & (((3i32) as u32)))) as i32); let a_ = (l_c_2).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
                if (((((F::Battle_GetMode(c)) == (1i32)) as i32)) != 0) {
                    { let v_ = { let a_ = (c.m.r32(sym::gProgress)).wrapping_add(60); (c.m.r32(a_) as i32) }; let a_ = l_c_2; c.m.w32(a_, (v_) as u32); };
                } else {
                    { let v_ = 1i32; let a_ = l_c_2; c.m.w32(a_, (v_) as u32); };
                }
                { let o_ = p_state_1; let n_ = (o_).wrapping_add(1); p_state_1 = n_; };
                break 's1;
            }
            10i32 => {
                if ((F::BtlClash_BothInActionFC(c)) != 0) {
                    { let a0_ = (c.m.r32((l_c_2).wrapping_add(8)) as i32); F::BtlClash_SetPathCut(c, a0_) };
                }
                if ((((((F::BtlChar_TestFlag(c, l_chr0_3, ((196i32) as u32))) != 0) && ((F::BtlChar_TestFlag(c, l_chr1_4, ((196i32) as u32))) != 0)) as i32)) != 0) {
                    if ((((((c.m.r32((l_chr0_3).wrapping_add(3672)) as i32)) >= (0i32)) as i32)) != 0) {
                        F::BtlAct_CountAndMarkOpponent(c, l_chr0_3);
                    } else {
                        F::BtlAct_CountAndMarkOpponent(c, l_chr1_4);
                    }
                    { let a_ = (l_c_2).wrapping_add(4); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    if ((((((((((c.m.r32((l_c_2).wrapping_add(4)) as i32)) >= (7i32)) as i32)) != 0) && ((((((c.m.r32((l_chr0_3).wrapping_add(3664)) as i32)) != ((c.m.r32((l_chr1_4).wrapping_add(3664)) as i32))) as i32)) != 0)) as i32)) != 0) {
                        if ((((((c.m.r32((l_chr1_4).wrapping_add(3664)) as i32)) < ((c.m.r32((l_chr0_3).wrapping_add(3664)) as i32))) as i32)) != 0) {
                            l_winner_6 = l_chr0_3;
                            l_loser_7 = l_chr1_4;
                        } else {
                            l_winner_6 = l_chr1_4;
                            l_loser_7 = l_chr0_3;
                        }
                        F::BtlChar_SetFlag(c, l_winner_6, ((199i32) as u32));
                        { let o_ = p_state_1; let n_ = (o_).wrapping_add(1); p_state_1 = n_; };
                        F::BtlChar_SetFlag(c, l_loser_7, ((200i32) as u32));
                    } else {
                        { let a_ = (l_c_2).wrapping_add(16); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                        if ((((((c.m.r32((l_c_2).wrapping_add(16)) as i32)) <= (0i32)) as i32)) != 0) {
                            l_prev_8 = (c.m.r32((l_c_2).wrapping_add(8)) as i32);
                            l_n_9 = ((rt::rem_u32((((((F::BtlChar_Rand(c)) as i64 as u64)) as u32)).wrapping_shr((2i32) as u32), (((F::BtlStage_GetPathCount(c)).wrapping_sub(2i32)) as u32))) as i32);
                            { let v_ = (l_n_9).wrapping_add(1i32); let a_ = (l_c_2).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                            if ((((((c.m.r32((l_c_2).wrapping_add(8)) as i32)) >= (l_prev_8)) as i32)) != 0) {
                                { let v_ = (l_n_9).wrapping_add(2i32); let a_ = (l_c_2).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                            }
                            { let v_ = (if (((((rt::rem_u32((((((F::BtlChar_Rand(c)) as i64 as u64)) as u32)).wrapping_shr((2i32) as u32), ((10i32) as u32))) < (((7i32) as u32))) as i32)) != 0) { 2i32 } else { 1i32 }); let a_ = (l_c_2).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
                            { let v_ = 0i32; let a_ = (l_c_2).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                            l_path_5 = { let a0_ = (c.m.r32((l_c_2).wrapping_add(8)) as i32); F::BtlStage_GetPath(c, a0_) };
                            if (((((l_path_5) != (((0i32) as u32))) as i32)) != 0) {
                                { let v_ = (((rt::rem_u32((((((F::BtlChar_Rand(c)) as i64 as u64)) as u32)).wrapping_shr((2i32) as u32), ((((c.m.r32(l_path_5) as i32)).wrapping_sub((c.m.r32((l_path_5).wrapping_add(8)) as i32))) as u32))).wrapping_add((((c.m.r32((l_path_5).wrapping_add(8)) as i32)) as u32))) as i32); let a_ = (l_c_2).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                            }
                        } else {
                            l_path_5 = { let a0_ = (c.m.r32((l_c_2).wrapping_add(8)) as i32); F::BtlStage_GetPath(c, a0_) };
                            if (((((l_path_5) != (((0i32) as u32))) as i32)) != 0) {
                                l_prev_10 = (c.m.r32((l_c_2).wrapping_add(12)) as i32);
                                { let v_ = (((rt::rem_u32((((((F::BtlChar_Rand(c)) as i64 as u64)) as u32)).wrapping_shr((2i32) as u32), (((((c.m.r32(l_path_5) as i32)).wrapping_sub((c.m.r32((l_path_5).wrapping_add(8)) as i32))).wrapping_sub(1i32)) as u32))).wrapping_add((((c.m.r32((l_path_5).wrapping_add(8)) as i32)) as u32))) as i32); let a_ = (l_c_2).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                                if ((((((c.m.r32((l_c_2).wrapping_add(12)) as i32)) >= (l_prev_10)) as i32)) != 0) {
                                    { let a_ = (l_c_2).wrapping_add(12); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                                }
                            } else {
                                { let v_ = 0i32; let a_ = (l_c_2).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                            }
                        }
                        { let v_ = (c.m.r32((l_c_2).wrapping_add(20)) as i32); let a_ = (l_c_2).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                        { let v_ = (((((((((F::BtlChar_Rand(c)) as i64 as u64)) as u32)).wrapping_shr((2i32) as u32)) & (((3i32) as u32)))) as i32); let a_ = (l_c_2).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
                        F::BtlChar_SetFlag(c, l_chr0_3, ((197i32) as u32));
                        F::BtlChar_SetFlag(c, l_chr1_4, ((197i32) as u32));
                    }
                }
                if ((((((((!((F::BtlClash_BothInActionFB(c)) != 0)) as i32)) != 0) && ((((!((F::BtlClash_BothInActionFC(c)) != 0)) as i32)) != 0)) as i32)) != 0) {
                    p_state_1 = 0i32;
                    F::BtlClash_EndActionFC(c, 0i32);
                    F::BtlClash_EndActionFC(c, 1i32);
                }
                break 's1;
            }
            11i32 => {
                if ((((!((F::BtlClash_BothInActionFC(c)) != 0)) as i32)) != 0) {
                    p_state_1 = 0i32;
                }
                break 's1;
            }
            _ => {
            }
        }
    }
    break 'f p_state_1;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlClash_UpdateC", &PROBE_N, &[], true); }
    r_
}

pub fn BtlClash_Update(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlClash_Update", &PROBE_N, &[], false); }
    let mut l_state_1: u32 = 0;
    let r_ = 'f: {
    l_state_1 = (c.m.r32(sym::gBtlChars)).wrapping_add(64);
    if ((((((c.m.r32(l_state_1) as i32)) == (0i32)) as i32)) != 0) {
        if ((F::BtlClash_BothInClashA(c)) != 0) {
            { let v_ = 1i32; let a_ = l_state_1; c.m.w32(a_, (v_) as u32); };
        }
        if ((F::BtlClash_BothInActionFA(c)) != 0) {
            { let v_ = 6i32; let a_ = l_state_1; c.m.w32(a_, (v_) as u32); };
        }
        if ((F::BtlClash_BothInActionFB(c)) != 0) {
            { let v_ = 9i32; let a_ = l_state_1; c.m.w32(a_, (v_) as u32); };
        }
    }
    's1: {
        let sv2: i32 = (c.m.r32(l_state_1) as i32);
        match sv2 {
            0i32 => {
                F::BtlClash_HoldClashA(c, 0i32);
                F::BtlClash_HoldClashA(c, 1i32);
                F::BtlClash_EndActionFA(c, 0i32);
                F::BtlClash_EndActionFA(c, 1i32);
                break 's1;
            }
            1i32 | 2i32 | 3i32 | 4i32 | 5i32 => {
                { let v_ = { let a0_ = (c.m.r32(l_state_1) as i32); F::BtlClash_UpdateA(c, a0_) }; let a_ = l_state_1; c.m.w32(a_, (v_) as u32); };
                if ((((!((F::BtlClash_BothInClashA(c)) != 0)) as i32)) != 0) {
                    F::BtlClash_HoldClashA(c, 0i32);
                    F::BtlClash_HoldClashA(c, 1i32);
                    { let v_ = 0i32; let a_ = l_state_1; c.m.w32(a_, (v_) as u32); };
                }
                break 's1;
            }
            6i32 | 7i32 | 8i32 => {
                { let v_ = { let a0_ = (c.m.r32(l_state_1) as i32); F::BtlClash_UpdateB(c, a0_) }; let a_ = l_state_1; c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            9i32 | 10i32 | 11i32 => {
                { let v_ = { let a0_ = (c.m.r32(l_state_1) as i32); F::BtlClash_UpdateC(c, a0_) }; let a_ = l_state_1; c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            _ => {
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlClash_Update", &PROBE_N, &[], true); }
    r_
}
