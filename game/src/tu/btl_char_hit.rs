#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn BtlHit_IsFromBehind(c: &mut Ctx, mut p_atk_1: u32, mut p_def_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlHit_IsFromBehind", &PROBE_N, &[(p_atk_1) as u32, (p_def_2) as u32], false); }
    let mut l_pd_3: u32 = 0;
    let mut l_ang_4: F32 = F32(0);
    let r_ = 'f: {
    l_pd_3 = F::BtlChar_GetPos(c, p_def_2);
    l_ang_4 = { let a0_ = rt::fadd(F32(c.m.r32(((l_pd_3).wrapping_add(16)).wrapping_add(4))), { let a_ = (F::BtlChar_GetPos(c, p_def_2)).wrapping_add(100); F32(c.m.r32(a_)) }); F::BtlUtil_WrapAngle(c, a0_) };
    l_ang_4 = { let a0_ = rt::fsub(l_ang_4, { let a_ = (F::BtlChar_GetPos(c, p_atk_1)).wrapping_add(148); F32(c.m.r32(a_)) }); F::BtlUtil_WrapAngle(c, a0_) };
    if (((rt::flt(rt::ffabs(l_ang_4), F32(0x3fc90fda)) as i32)) != 0) {
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlHit_IsFromBehind", &PROBE_N, &[], true); }
    r_
}

pub fn BtlHit_GetVoiceKind(c: &mut Ctx, mut p_reaction_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlHit_GetVoiceKind", &PROBE_N, &[(p_reaction_1) as u32], false); }
    let r_ = 'f: {
    's1: {
        let sv2: i32 = p_reaction_1;
        match sv2 {
            1i32 | 2i32 | 3i32 | 4i32 | 11i32 | 14i32 | 20i32 | 21i32 | 22i32 | 31i32 | 32i32 | 33i32 | 34i32 | 37i32 | 43i32 | 44i32 | 45i32 | 46i32 | 49i32 => {
                break 'f 0i32;
            }
            5i32 | 6i32 | 7i32 | 8i32 | 9i32 | 10i32 | 29i32 | 30i32 | 35i32 | 36i32 | 42i32 | 47i32 | 48i32 => {
                break 'f 1i32;
            }
            15i32 | 16i32 | 17i32 | 19i32 => {
                break 'f 2i32;
            }
            18i32 | 40i32 | 41i32 => {
                break 'f 3i32;
            }
            12i32 | 13i32 | 23i32 | 24i32 | 25i32 | 26i32 | 27i32 | 28i32 | 38i32 | 39i32 => {
                break 'f 4i32;
            }
            _ => {
            }
        }
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlHit_GetVoiceKind", &PROBE_N, &[], true); }
    r_
}

pub fn BtlHit_GetHitNo(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlHit_GetHitNo", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_obj_2: u32 = 0;
    let r_ = 'f: {
    l_obj_2 = F::BtlChar_GetObj(c, p_chr_1);
    if (((({ let a_ = (c.m.r32((l_obj_2).wrapping_add(5728))).wrapping_add(98336); c.m.r32(a_) }) & (((251658240i32) as u32)))) != 0) {
        break 'f (((c.m.r8((l_obj_2).wrapping_add(3245)) as i8)) as i32);
    }
    if ((F::BtlChar_TestFlag(c, p_chr_1, ((((101i32) as i64 as u64)) as u32))) != 0) {
        break 'f (c.m.r32(((p_chr_1).wrapping_add(3888)).wrapping_add(24)) as i32);
    }
    break 'f (1i32).wrapping_neg();
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlHit_GetHitNo", &PROBE_N, &[], true); }
    r_
}

pub fn BtlHit_ApplyGuard(c: &mut Ctx, mut p_atk_1: u32, mut p_def_2: u32, mut p_result_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlHit_ApplyGuard", &PROBE_N, &[(p_atk_1) as u32, (p_def_2) as u32, (p_result_3) as u32], false); }
    let mut l_scale_4: F32 = F32(0);
    let mut l_power_5: F32 = F32(0);
    let mut l_time_6: F32 = F32(0);
    let mut l_pose_7: u32 = 0;
    let r_ = 'f: {
    if ((((((((F::BtlAtk_GetFlags(c, p_atk_1)) as u64)) as i32)) & (65536i32))) != 0) {
        F::BtlChar_SetFxBit(c, p_atk_1, 37i32);
    }
    if ((((((((F::BtlAtk_GetFlags(c, p_atk_1)) as u64)) as i32)) & (131072i32))) != 0) {
        F::BtlChar_SetFxBit(c, p_atk_1, 38i32);
    }
    if ((((((((F::BtlAtk_GetFlags(c, p_atk_1)) as u64)) as i32)) & (262144i32))) != 0) {
        F::BtlChar_SetFxBit(c, p_atk_1, 39i32);
    }
    F::BtlChar_ClearFlag(c, p_atk_1, ((((138i32) as i64 as u64)) as u32));
    F::BtlChar_SetHeldFlag(c, p_atk_1, ((((93i32) as i64 as u64)) as u32));
    's1: {
        let sv8: u32 = p_result_3;
        match sv8 {
            1u32 => {
                F::BtlChar_SetFxBit(c, p_atk_1, 34i32);
                break 's1;
            }
            0u32 | 2u32 | 3u32 | 4u32 => {
                F::BtlChar_SetFxBit(c, p_atk_1, 28i32);
                break 's1;
            }
            5u32 => {
                F::BtlChar_SetFlag(c, p_atk_1, ((((108i32) as i64 as u64)) as u32));
                F::BtlChar_SetHeldFlag(c, p_atk_1, ((((134i32) as i64 as u64)) as u32));
                { let r_ = 50000i32; let a_ = (p_atk_1).wrapping_add(3456); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                if ((((((c.m.r32((p_atk_1).wrapping_add(3456)) as i32)) > (100000i32)) as i32)) != 0) {
                    { let v_ = 100000i32; let a_ = (p_atk_1).wrapping_add(3456); c.m.w32(a_, (v_) as u32); };
                }
                break 's1;
            }
            6u32 => {
                F::BtlChar_SetFxBit(c, p_atk_1, 33i32);
                F::BtlChar_SetFlag(c, p_atk_1, ((((111i32) as i64 as u64)) as u32));
                break 's1;
            }
            7u32 => {
                F::BtlChar_SetFxBit(c, p_atk_1, 36i32);
                F::BtlChar_SetFlag(c, p_atk_1, ((((111i32) as i64 as u64)) as u32));
                { let v_ = 2i32; let a_ = (p_atk_1).wrapping_add(4900); c.m.w32(a_, (v_) as u32); };
                { let v_ = 1i32; let a_ = (p_atk_1).wrapping_add(4904); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            8u32 => {
                F::BtlChar_SetFxBit(c, p_atk_1, 33i32);
                F::BtlChar_SetFlag(c, p_atk_1, ((((111i32) as i64 as u64)) as u32));
                { let v_ = 2i32; let a_ = (p_atk_1).wrapping_add(4900); c.m.w32(a_, (v_) as u32); };
                { let v_ = 1i32; let a_ = (p_atk_1).wrapping_add(4904); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            _ => {
            }
        }
    }
    l_scale_4 = F32(0x3f800000);
    's2: {
        let sv9: u32 = p_result_3;
        match sv9 {
            0u32 | 1u32 => {
                break 's2;
            }
            2u32 => {
                F::BtlChar_SetFlag(c, p_def_2, ((((104i32) as i64 as u64)) as u32));
                break 's2;
            }
            3u32 => {
                F::BtlChar_SetFlag(c, p_def_2, ((((105i32) as i64 as u64)) as u32));
                break 's2;
            }
            4u32 => {
                F::BtlChar_SetFlag(c, p_def_2, ((((106i32) as i64 as u64)) as u32));
                break 's2;
            }
            5u32 => {
                F::BtlChar_SetFxBit(c, p_def_2, 32i32);
                F::BtlChar_SetFlag(c, p_def_2, ((((109i32) as i64 as u64)) as u32));
                { let _ = { let a1_ = F::BtlAtk_GetGuardKiCost(c, p_atk_1); F::BtlMember_SpendKi(c, p_def_2, a1_, 0i32) }; };
                if ((F::BtlMember_IsKiEmpty(c, p_def_2)) != 0) {
                    F::BtlChar_SetHeldFlag(c, p_def_2, ((((190i32) as i64 as u64)) as u32));
                }
                l_scale_4 = F32(0x3f000000);
                break 's2;
            }
            6u32 => {
                F::BtlChar_SetFlag(c, p_def_2, ((((110i32) as i64 as u64)) as u32));
                F::BtlMember_AddBlast(c, p_def_2, 50000i32);
                l_scale_4 = F32(0x3f000000);
                break 's2;
            }
            7u32 => {
                F::BtlChar_SetFlag(c, p_def_2, ((((112i32) as i64 as u64)) as u32));
                F::BtlChar_PlayVoice(c, p_def_2, 18i32);
                { let v_ = 3i32; let a_ = (p_def_2).wrapping_add(4900); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (p_def_2).wrapping_add(4904); c.m.w32(a_, (v_) as u32); };
                break 's2;
            }
            8u32 => {
                F::BtlChar_SetFlag(c, p_def_2, ((((123i32) as i64 as u64)) as u32));
                { let v_ = 3i32; let a_ = (p_def_2).wrapping_add(4900); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (p_def_2).wrapping_add(4904); c.m.w32(a_, (v_) as u32); };
                break 's2;
            }
            _ => {
            }
        }
    }
    l_pose_7 = F::BtlChar_GetPos(c, p_atk_1);
    { let a1_ = F32(c.m.r32((l_pose_7).wrapping_add(148))); let a2_ = rt::fmul(F::BtlAtk_GetPushOnGuard(c, p_atk_1), l_scale_4); F::BtlMove_SetImpulseYaw(c, p_def_2, a1_, a2_) };
    's3: {
        let sv10: i32 = F::BtlAtk_GetGuardFlagSel(c, p_atk_1);
        match sv10 {
            0i32 => {
                F::BtlChar_SetFlag(c, p_def_2, ((((102i32) as i64 as u64)) as u32));
                break 's3;
            }
            1i32 => {
                F::BtlChar_SetFlag(c, p_def_2, ((((103i32) as i64 as u64)) as u32));
                break 's3;
            }
            _ => {
            }
        }
    }
    l_power_5 = F::BtlAtk_GetShakePowerB(c, p_atk_1);
    l_time_6 = F::BtlAtk_GetShakeTimeB(c, p_atk_1);
    if ((((((((F::BtlAtk_GetFlags(c, p_atk_1)) as u64)) as i32)) & (2048i32))) != 0) {
        F::ChrCam_AddShake(c, p_atk_1, l_power_5, l_time_6);
    }
    F::ChrCam_AddShake(c, p_def_2, l_power_5, l_time_6);
    F::BtlChar_Vibrate(c, p_atk_1, rt::fmul(l_power_5, F32(0x3f000000)), rt::fmul(l_time_6, F32(0x3f000000)));
    F::BtlChar_Vibrate(c, p_def_2, rt::fmul(l_power_5, F32(0x3f000000)), rt::fmul(l_time_6, F32(0x3f000000)));
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlHit_ApplyGuard", &PROBE_N, &[], true); }
    r_
}

pub fn BtlHit_CheckNear(c: &mut Ctx, mut p_a_1: u32, mut p_b_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlHit_CheckNear", &PROBE_N, &[(p_a_1) as u32, (p_b_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let mut l_range_3: F32 = F32(0);
    let r_ = 'f: {
        if ((F::BtlChar_TestFlag(c, p_b_2, ((((66i32) as i64 as u64)) as u32))) != 0) {
            break 'f 0i32;
        }
        if ((F::BtlChar_TestFlag(c, p_a_1, ((((74i32) as i64 as u64)) as u32))) != 0) {
            l_range_3 = rt::fmul({ let a0_ = (c.m.r32((p_a_1).wrapping_add(12)) as i32); F::BtlCharApi_GetHeight(c, a0_) }, F32(0x40a00000));
        } else {
            if ((F::BtlChar_TestFlag(c, p_a_1, ((((75i32) as i64 as u64)) as u32))) != 0) {
                l_range_3 = rt::fmul({ let a0_ = (c.m.r32((p_a_1).wrapping_add(12)) as i32); F::BtlCharApi_GetHeight(c, a0_) }, F32(0x40000000));
            } else {
                break 'f 0i32;
            }
        }
        if (((rt::flt(l_range_3, F32(0x41a00000)) as i32)) != 0) {
            l_range_3 = F32(0x41a00000);
        }
        if (((rt::flt(F32(0x43160000), l_range_3) as i32)) != 0) {
            l_range_3 = F32(0x43160000);
        }
        { let a0_ = (c.m.r32((p_a_1).wrapping_add(12)) as i32); F::BtlCharApi_GetNodePos(c, a0_, 3i32, fp) };
        { let a0_ = (c.m.r32((p_b_2).wrapping_add(12)) as i32); F::BtlCharApi_GetNodePos(c, a0_, 3i32, fp.wrapping_add(16)) };
        F::Vec4_Sub(c, fp.wrapping_add(32), fp, fp.wrapping_add(16));
        if (((rt::flt(F::Vec3_Length(c, fp.wrapping_add(32)), l_range_3) as i32)) != 0) {
            F::BtlChar_SetFlag(c, p_a_1, ((((101i32) as i64 as u64)) as u32));
            if ((F::BtlChar_TestFlag(c, p_a_1, ((((75i32) as i64 as u64)) as u32))) != 0) {
                F::BtlChar_ClearFlag(c, p_a_1, ((((75i32) as i64 as u64)) as u32));
            }
        }
        break 'f 0i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlHit_CheckNear", &PROBE_N, &[], true); }
    r_
}

pub fn btl_char_hit__BtlHit_FormIndex(c: &mut Ctx, mut p_n_1: i32, mut p_g_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "btl_char_hit__BtlHit_FormIndex", &PROBE_N, &[(p_n_1) as u32, (p_g_2) as u32], false); }
    let r_ = 'f: {
    break 'f ((p_n_1).wrapping_mul(2i32)).wrapping_add(((((c.m.r32((p_g_2).wrapping_add(32)) as i32)) != (0i32)) as i32));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "btl_char_hit__BtlHit_FormIndex", &PROBE_N, &[], true); }
    r_
}

pub fn BtlHit_CheckGrab(c: &mut Ctx, mut p_a_1: u32, mut p_b_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlHit_CheckGrab", &PROBE_N, &[(p_a_1) as u32, (p_b_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let mut l_dir_3: i32 = 0;
    let mut l_n_4: i32 = 0;
    let mut l_range_5: F32 = F32(0);
    let mut l_flags_6: u32 = 0;
    let mut l_t_7: i32 = 0;
    let mut l_g_8: u32 = 0;
    let mut l_is800_9: i32 = 0;
    let mut bb_: u32 = 0;
    let r_ = 'f: {
        'fl1: loop {
            match bb_ {
                0 => {
                    if ((F::BtlChar_TestFlag(c, p_b_2, ((((66i32) as i64 as u64)) as u32))) != 0) { bb_ = 1; continue 'fl1; } else { bb_ = 2; continue 'fl1; }
                }
                1 => {
                    break 'f 0i32;
                }
                2 => {
                    l_dir_3 = (1i32).wrapping_neg();
                    if ((F::BtlChar_TestFlag(c, p_a_1, ((((76i32) as i64 as u64)) as u32))) != 0) { bb_ = 4; continue 'fl1; } else { bb_ = 5; continue 'fl1; }
                }
                3 => {
                    bb_ = 2; continue 'fl1;
                }
                4 => {
                    l_dir_3 = 0i32;
                    bb_ = 5; continue 'fl1;
                }
                5 => {
                    if ((F::BtlChar_TestFlag(c, p_a_1, ((((77i32) as i64 as u64)) as u32))) != 0) { bb_ = 6; continue 'fl1; } else { bb_ = 7; continue 'fl1; }
                }
                6 => {
                    l_dir_3 = 1i32;
                    bb_ = 7; continue 'fl1;
                }
                7 => {
                    if (((((l_dir_3) < (0i32)) as i32)) != 0) { bb_ = 8; continue 'fl1; } else { bb_ = 9; continue 'fl1; }
                }
                8 => {
                    break 'f 0i32;
                }
                9 => {
                    F::BtlChar_ClearFlag(c, p_a_1, ((((76i32) as i64 as u64)) as u32));
                    F::BtlChar_ClearFlag(c, p_a_1, ((((77i32) as i64 as u64)) as u32));
                    l_n_4 = { let a_ = (F::BtlMember_GetActive(c, p_b_2)).wrapping_add(4); (c.m.r32(a_) as i32) };
                    l_n_4 = { let a1_ = F::BtlMember_GetActiveGauge(c, p_b_2); F::btl_char_hit__BtlHit_FormIndex(c, l_n_4, a1_) };
                    if ((((((((F::BtlParam_GetFlags(c, p_b_2)) as u64)) as i32)) & ((16777216i32).wrapping_shl((l_n_4) as u32)))) != 0) { bb_ = 11; continue 'fl1; } else { bb_ = 12; continue 'fl1; }
                }
                10 => {
                    bb_ = 9; continue 'fl1;
                }
                11 => {
                    break 'f 0i32;
                }
                12 => {
                    if ((F::BtlChar_TestFlag(c, p_b_2, ((((311i32) as i64 as u64)) as u32))) != 0) { bb_ = 14; continue 'fl1; } else { bb_ = 15; continue 'fl1; }
                }
                13 => {
                    bb_ = 12; continue 'fl1;
                }
                14 => {
                    break 'f 0i32;
                }
                15 => {
                    l_range_5 = F32(0x42c80000);
                    { let a0_ = (c.m.r32((p_a_1).wrapping_add(12)) as i32); F::BtlCharApi_GetNodePos(c, a0_, 3i32, fp) };
                    { let a0_ = (c.m.r32((p_b_2).wrapping_add(12)) as i32); F::BtlCharApi_GetNodePos(c, a0_, 3i32, fp.wrapping_add(16)) };
                    F::Vec4_Sub(c, fp.wrapping_add(32), fp, fp.wrapping_add(16));
                    if (((rt::flt(l_range_5, F::Vec3_Length(c, fp.wrapping_add(32))) as i32)) != 0) { bb_ = 17; continue 'fl1; } else { bb_ = 18; continue 'fl1; }
                }
                16 => {
                    bb_ = 15; continue 'fl1;
                }
                17 => {
                    break 'f 0i32;
                }
                18 => {
                    l_flags_6 = { let a0_ = F::BtlAnim_GetId(c, p_b_2); F::BtlAnim_GetFlags(c, ((((a0_) as i64 as u64)) as u32)) };
                    l_is800_9 = ((((l_flags_6) & (((2048i32) as u32)))) as i32);
                    l_t_7 = ((((l_flags_6) & (((32768i32) as u32)))) as i32);
                    if ((((!((l_is800_9) != 0)) as i32)) != 0) { bb_ = 20; continue 'fl1; } else { bb_ = 21; continue 'fl1; }
                }
                19 => {
                    bb_ = 18; continue 'fl1;
                }
                20 => {
                    l_t_7 = F::BtlHit_IsFromBehind(c, p_a_1, p_b_2);
                    bb_ = 21; continue 'fl1;
                }
                21 => {
                    if ((l_t_7) != 0) { bb_ = 22; continue 'fl1; } else { bb_ = 23; continue 'fl1; }
                }
                22 => {
                    l_t_7 = 0i32;
                    bb_ = 24; continue 'fl1;
                }
                23 => {
                    l_t_7 = F::BtlSkill_GetFrames(c, p_a_1, l_dir_3);
                    { let v_ = 0i32; let a_ = ((p_b_2).wrapping_add(4016)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                    { let v_ = l_t_7; let a_ = ((p_b_2).wrapping_add(4016)).wrapping_add(72); c.m.w32(a_, (v_) as u32); };
                    { let v_ = { let a_ = (F::BtlChar_GetPos(c, p_a_1)).wrapping_add(148); F32(c.m.r32(a_)) }; let a_ = ((p_b_2).wrapping_add(4016)).wrapping_add(28); c.m.w32(a_, (v_).0); };
                    F::BtlChar_PlayVoice(c, p_b_2, 0i32);
                    if ((l_is800_9) != 0) { bb_ = 26; continue 'fl1; } else { bb_ = 28; continue 'fl1; }
                }
                24 => {
                    break 'f l_t_7;
                }
                25 => {
                    bb_ = 23; continue 'fl1;
                }
                26 => {
                    { let v_ = 14i32; let a_ = (p_b_2).wrapping_add(4016); c.m.w32(a_, (v_) as u32); };
                    bb_ = 27; continue 'fl1;
                }
                27 => {
                    F::BtlChar_ClearFlag(c, p_b_2, ((((5i32) as i64 as u64)) as u32));
                    l_t_7 = 1i32;
                    bb_ = 24; continue 'fl1;
                }
                28 => {
                    { let v_ = 9i32; let a_ = (p_b_2).wrapping_add(4016); c.m.w32(a_, (v_) as u32); };
                    bb_ = 27; continue 'fl1;
                }
                29 => {
                    break 'fl1;
                }
                _ => unreachable!(),
            }
        }
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlHit_CheckGrab", &PROBE_N, &[], true); }
    r_
}

pub fn BtlHit_Stub(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlHit_Stub", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlHit_Stub", &PROBE_N, &[], true); }
    r_
}

pub fn BtlHit_CheckTouch(c: &mut Ctx, mut p_a_1: u32, mut p_b_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlHit_CheckTouch", &PROBE_N, &[(p_a_1) as u32, (p_b_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_k_3: F32 = F32(0);
    let mut l_hit_4: i32 = 0;
    let mut l_dist_5: F32 = F32(0);
    let mut l_pb_6: u32 = 0;
    let r_ = 'f: {
        if ((F::BtlChar_TestFlag(c, p_b_2, ((((66i32) as i64 as u64)) as u32))) != 0) {
            break 'f 0i32;
        }
        if ((((!((F::BtlChar_TestFlag(c, p_a_1, ((((79i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) {
            break 'f 0i32;
        }
        l_k_3 = F32(0x3f999999);
        l_hit_4 = (((F::BtlMove_IsBlockedByOpponent(c, p_a_1)) != (0i32)) as i32);
        l_pb_6 = F::BtlChar_GetPos(c, p_b_2);
        { let a2_ = F::BtlChar_GetPos(c, p_a_1); F::Vec4_Sub(c, fp, l_pb_6, a2_) };
        l_dist_5 = F::BtlUtil_LengthXZ(c, fp);
        { let r_ = rt::fmul({ let a0_ = (c.m.r32((p_a_1).wrapping_add(12)) as i32); F::BtlCharApi_GetRadius(c, a0_) }, l_k_3); let n_ = rt::fsub(l_dist_5, r_); l_dist_5 = n_; };
        { let r_ = rt::fmul({ let a0_ = (c.m.r32((p_b_2).wrapping_add(12)) as i32); F::BtlCharApi_GetRadius(c, a0_) }, l_k_3); let n_ = rt::fsub(l_dist_5, r_); l_dist_5 = n_; };
        if (((rt::flt(l_dist_5, F32(0x00000000)) as i32)) != 0) {
            { let v_ = rt::fmul({ let a0_ = (c.m.r32((p_a_1).wrapping_add(12)) as i32); F::BtlCharApi_GetHeight(c, a0_) }, l_k_3); let a_ = (fp.wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            { let v_ = rt::fmul({ let a0_ = (c.m.r32((p_b_2).wrapping_add(12)) as i32); F::BtlCharApi_GetHeight(c, a0_) }, l_k_3); let a_ = (fp.wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            if (((((((rt::flt(rt::fneg(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4))))), F32(c.m.r32((fp).wrapping_add(4)))) as i32)) != 0) && (((rt::flt(F32(c.m.r32((fp).wrapping_add(4))), F32(c.m.r32((fp.wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(4))))) as i32)) != 0)) as i32)) != 0) {
                l_hit_4 = 1i32;
            }
        }
        if ((l_hit_4) != 0) {
            F::BtlChar_SetFlag(c, p_a_1, ((((101i32) as i64 as u64)) as u32));
            F::BtlChar_ClearFlag(c, p_a_1, ((((79i32) as i64 as u64)) as u32));
        }
        break 'f l_hit_4;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlHit_CheckTouch", &PROBE_N, &[], true); }
    r_
}

pub fn BtlHit_IsVoid(c: &mut Ctx, mut p_atk_1: u32, mut p_def_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlHit_IsVoid", &PROBE_N, &[(p_atk_1) as u32, (p_def_2) as u32], false); }
    let mut l_k_3: i32 = 0;
    let mut l_behind_4: i32 = 0;
    let mut l_flags_5: u32 = 0;
    let r_ = 'f: {
    l_k_3 = F::BtlAtk_GetId(c, p_atk_1);
    if ((F::BtlChar_TestFlag(c, p_atk_1, ((((186i32) as i64 as u64)) as u32))) != 0) {
        break 'f 1i32;
    }
    if (((((l_k_3) == (85i32)) as i32)) != 0) {
        if ((F::BtlChar_TestFlag(c, p_def_2, ((((67i32) as i64 as u64)) as u32))) != 0) {
            break 'f 1i32;
        }
    } else {
        if ((F::BtlChar_TestFlag(c, p_def_2, ((((66i32) as i64 as u64)) as u32))) != 0) {
            break 'f 1i32;
        }
    }
    l_behind_4 = F::BtlHit_IsFromBehind(c, p_atk_1, p_def_2);
    l_flags_5 = { let a0_ = F::BtlAnim_GetId(c, p_def_2); F::BtlAnim_GetFlags(c, ((((a0_) as i64 as u64)) as u32)) };
    if (((((F::BtlAtk_GetId(c, p_atk_1)) == (86i32)) as i32)) != 0) {
        if ((((l_flags_5) & (((2048i32) as u32)))) != 0) {
            if ((((l_flags_5) & (((32768i32) as u32)))) != 0) {
                break 'f 1i32;
            }
        }
        if ((l_behind_4) != 0) {
            break 'f 1i32;
        }
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlHit_IsVoid", &PROBE_N, &[], true); }
    r_
}

pub fn BtlHit_CheckDodge(c: &mut Ctx, mut p_atk_1: u32, mut p_def_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlHit_CheckDodge", &PROBE_N, &[(p_atk_1) as u32, (p_def_2) as u32], false); }
    let mut l_flags_3: u32 = 0;
    let mut l_st_4: u32 = 0;
    let mut l_lvl_5: i32 = 0;
    let mut l_half_6: i32 = 0;
    let mut l_on_7: i32 = 0;
    let mut l_done_8: i32 = 0;
    let mut l_strike_9: i32 = 0;
    let mut l_free_10: i32 = 0;
    let mut l_counted_11: i32 = 0;
    let mut l_paid_12: i32 = 0;
    let mut l_dir_13: i32 = 0;
    let mut l_d_14: i32 = 0;
    let r_ = 'f: {
    if ((((!((F::BtlChar_IsFree(c, p_def_2)) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((F::BtlChar_TestFlag(c, p_def_2, ((((90i32) as i64 as u64)) as u32))) != 0) {
        break 'f 0i32;
    }
    l_st_4 = { let a0_ = F::BtlAnim_GetId(c, p_def_2); F::BtlAnim_GetFlags(c, ((((a0_) as i64 as u64)) as u32)) };
    l_on_7 = 0i32;
    if ((((l_st_4) & (((177i32) as u32)))) != 0) {
        l_on_7 = 1i32;
    }
    if ((((l_st_4) & (((2097152i32) as u32)))) != 0) {
        l_d_14 = (({ let a_ = (F::BtlChar_GetObj(c, p_def_2)).wrapping_add(3245); (c.m.r8(a_) as i8) }) as i32);
        { let r_ = (!((({ let a_ = (F::BtlChar_GetObj(c, p_def_2)).wrapping_add(3244); (c.m.r8(a_) as i8) }) as i32))); let n_ = ((l_d_14) ^ (r_)); l_d_14 = n_; };
        if (((((l_d_14) == (0i32)) as i32)) != 0) {
            l_on_7 = 1i32;
        }
        if ((F::BtlChar_TestFlag(c, p_def_2, ((((96i32) as i64 as u64)) as u32))) != 0) {
            l_on_7 = 1i32;
        }
    }
    if ((l_on_7) != 0) {
        l_flags_3 = ((((((F::BtlAtk_GetFlags(c, p_atk_1)) as u64)) as i32)) as u32);
        l_done_8 = 0i32;
        l_strike_9 = 0i32;
        l_free_10 = 0i32;
        l_counted_11 = 0i32;
        l_paid_12 = 0i32;
        l_lvl_5 = (c.m.r32((p_def_2).wrapping_add(4200)) as i32);
        l_half_6 = 0i32;
        if ((((l_flags_3) & (((67108864i32) as u32)))) != 0) {
            l_half_6 = 1i32;
        }
        { let r_ = l_half_6; let n_ = (l_lvl_5).wrapping_sub(r_); l_lvl_5 = n_; };
        if (((((((((((((l_lvl_5) > (0i32)) as i32)) != 0) && ((((l_flags_3) & (((1i32) as u32)))) != 0)) as i32)) != 0) && (((((F::BtlHit_GetHitNo(c, p_atk_1)) == (0i32)) as i32)) != 0)) as i32)) != 0) {
            F::BtlChar_SetFlag(c, p_def_2, ((((300i32) as i64 as u64)) as u32));
            l_done_8 = 1i32;
            l_free_10 = 1i32;
            if ((((l_flags_3) & (((1048576i32) as u32)))) != 0) {
                l_paid_12 = (((F::BtlMember_HasKi(c, p_def_2, 20000i32)) != (0i32)) as i32);
            }
        }
        if ((((!((l_done_8) != 0)) as i32)) != 0) {
            if ((((((c.m.r32((p_def_2).wrapping_add(3584)) as i32)) > (0i32)) as i32)) != 0) {
                { let a_ = (p_def_2).wrapping_add(3584); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                l_done_8 = 1i32;
                l_counted_11 = 1i32;
            }
            if ((((((c.m.r32((p_def_2).wrapping_add(3588)) as i32)) > (0i32)) as i32)) != 0) {
                { let a_ = (p_def_2).wrapping_add(3588); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                l_done_8 = 1i32;
                l_strike_9 = 1i32;
                l_counted_11 = 1i32;
            }
            if ((F::BtlChar_TestFlag(c, p_def_2, ((((86i32) as i64 as u64)) as u32))) != 0) {
                l_done_8 = 1i32;
                l_counted_11 = 1i32;
            }
            if ((F::BtlChar_TestFlag(c, p_def_2, ((((87i32) as i64 as u64)) as u32))) != 0) {
                l_done_8 = 1i32;
            }
        }
        if ((l_done_8) != 0) {
            if ((l_paid_12) != 0) {
                F::BtlChar_SetFlag(c, p_def_2, ((((124i32) as i64 as u64)) as u32));
                { let _ = F::BtlMember_SpendKi(c, p_def_2, 20000i32, 0i32); };
            } else {
                if ((l_strike_9) != 0) {
                    F::BtlChar_SetFlag(c, p_def_2, ((((119i32) as i64 as u64)) as u32));
                } else {
                    if ((((l_flags_3) & (((8388608i32) as u32)))) != 0) {
                        l_dir_13 = F::BtlChar_FrameMod(c, 2i32);
                        if ((F::BtlInput_IsHeld(c, p_def_2, ((64i32) as u32))) != 0) {
                            l_dir_13 = 0i32;
                        }
                        if ((F::BtlInput_IsHeld(c, p_def_2, ((128i32) as u32))) != 0) {
                            l_dir_13 = 1i32;
                        }
                    } else {
                        l_dir_13 = F::BtlChar_FrameMod(c, 3i32);
                        if ((F::BtlInput_IsHeld(c, p_def_2, ((64i32) as u32))) != 0) {
                            l_dir_13 = 0i32;
                        }
                        if ((F::BtlInput_IsHeld(c, p_def_2, ((128i32) as u32))) != 0) {
                            l_dir_13 = 1i32;
                        }
                        if ((F::BtlInput_IsHeld(c, p_def_2, ((32i32) as u32))) != 0) {
                            l_dir_13 = 2i32;
                        }
                    }
                    's1: {
                        let sv15: i32 = l_dir_13;
                        match sv15 {
                            0i32 => {
                                F::BtlChar_SetFlag(c, p_def_2, ((((116i32) as i64 as u64)) as u32));
                                break 's1;
                            }
                            1i32 => {
                                F::BtlChar_SetFlag(c, p_def_2, ((((117i32) as i64 as u64)) as u32));
                                break 's1;
                            }
                            2i32 => {
                                F::BtlChar_SetFlag(c, p_def_2, ((((118i32) as i64 as u64)) as u32));
                                break 's1;
                            }
                            _ => {
                            }
                        }
                    }
                    if ((((l_flags_3) & (((16777216i32) as u32)))) != 0) {
                        F::BtlChar_SetFlag(c, p_def_2, ((((120i32) as i64 as u64)) as u32));
                    }
                }
            }
            if ((((((l_free_10) != 0) && ((F::BtlMember_HasAbility(c, p_def_2, 33i32)) != 0)) as i32)) != 0) {
                F::BtlMember_AddBlast(c, p_def_2, 100000i32);
            }
            if ((l_counted_11) != 0) {
                's2: {
                    let sv16: i32 = (c.m.r32((p_def_2).wrapping_add(3596)) as i32);
                    match sv16 {
                        0i32 => {
                            F::BtlChar_SetFlag(c, p_def_2, ((((222i32) as i64 as u64)) as u32));
                            break 's2;
                        }
                        1i32 => {
                            F::BtlChar_SetFlag(c, p_def_2, ((((223i32) as i64 as u64)) as u32));
                            break 's2;
                        }
                        _ => {
                        }
                    }
                }
            }
            break 'f 1i32;
        }
    }
    if ((((((((((((((((((F::BtlChar_TestFlag(c, p_def_2, ((((78i32) as i64 as u64)) as u32))) != 0) && ((((F::BtlParam_GetFlags2(c, p_def_2)) & (((128i32) as u32)))) != 0)) as i32)) != 0) && (((((F::BtlParam_GetUnk2(c, p_atk_1)) != (4i32)) as i32)) != 0)) as i32)) != 0) && ((((!((F::BtlHit_IsFromBehind(c, p_atk_1, p_def_2)) != 0)) as i32)) != 0)) as i32)) != 0) && (((((((F::BtlAtk_GetId(c, p_atk_1)) as u32)) < (((11i32) as u32))) as i32)) != 0)) as i32)) != 0) {
        F::BtlChar_SetFlag(c, p_def_2, ((((125i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_atk_1, ((((126i32) as i64 as u64)) as u32));
        if ((((((({ let a_ = (F::BtlChar_GetObj(c, p_atk_1)).wrapping_add(3245); (c.m.r8(a_) as i8) }) as i32)) == (0i32)) as i32)) != 0) {
            { let _ = F::BtlMember_SpendKi(c, p_atk_1, 2000i32, 0i32); };
        }
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlHit_CheckDodge", &PROBE_N, &[], true); }
    r_
}

pub fn BtlHit_CheckArmor(c: &mut Ctx, mut p_atk_1: u32, mut p_def_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlHit_CheckArmor", &PROBE_N, &[(p_atk_1) as u32, (p_def_2) as u32], false); }
    let mut l_ret_3: i32 = 0;
    let r_ = 'f: {
    if ((((!(((({ let a0_ = F::BtlAnim_GetId(c, p_def_2); F::BtlAnim_GetFlags(c, ((((a0_) as i64 as u64)) as u32)) }) & (((512i32) as u32)))) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    l_ret_3 = 0i32;
    if ((((((((F::BtlAtk_GetFlags(c, p_atk_1)) as u64)) as i32)) & (2i32))) != 0) {
        l_ret_3 = 1i32;
    }
    break 'f l_ret_3;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlHit_CheckArmor", &PROBE_N, &[], true); }
    r_
}

pub fn BtlHit_CheckRepel(c: &mut Ctx, mut p_atk_1: u32, mut p_def_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlHit_CheckRepel", &PROBE_N, &[(p_atk_1) as u32, (p_def_2) as u32], false); }
    let r_ = 'f: {
    if ((((((F::BtlChar_TestFlag(c, p_def_2, ((((89i32) as i64 as u64)) as u32))) != 0) || ((F::BtlChar_TestFlag(c, p_def_2, ((((83i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) {
        F::BtlChar_SetFlag(c, p_atk_1, ((((111i32) as i64 as u64)) as u32));
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlHit_CheckRepel", &PROBE_N, &[], true); }
    r_
}

pub fn BtlHit_CheckGuard(c: &mut Ctx, mut p_atk_1: u32, mut p_def_2: u32, mut p_full_3: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlHit_CheckGuard", &PROBE_N, &[(p_atk_1) as u32, (p_def_2) as u32, (p_full_3) as u32], false); }
    let mut l_guard_4: i32 = 0;
    let mut l_res_5: i32 = 0;
    let r_ = 'f: {
    { let v_ = 1i32; let a_ = p_full_3; c.m.w32(a_, (v_) as u32); };
    if ((((!((F::BtlChar_IsFree(c, p_def_2)) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    l_guard_4 = F::BtlColl_GetGuardKind(c, p_def_2);
    if ((((((((((((((({ let a_ = (F::BtlChar_GetObj(c, p_atk_1)).wrapping_add(3245); (c.m.r8(a_) as i8) }) as i32)) == (0i32)) as i32)) != 0) && (((({ let a0_ = F::BtlAnim_GetId(c, p_def_2); F::BtlAnim_GetFlags(c, ((((a0_) as i64 as u64)) as u32)) }) & (((49i32) as u32)))) != 0)) as i32)) != 0) && ((((((c.m.r32((p_def_2).wrapping_add(4208)) as i32)) > (0i32)) as i32)) != 0)) as i32)) != 0) {
        F::BtlHit_ApplyGuard(c, p_atk_1, p_def_2, ((8i32) as u32));
        { let v_ = 0i32; let a_ = p_full_3; c.m.w32(a_, (v_) as u32); };
        break 'f 1i32;
    }
    if (((((l_guard_4) != (6i32)) as i32)) != 0) {
        if ((F::BtlHit_IsFromBehind(c, p_atk_1, p_def_2)) != 0) {
            break 'f 0i32;
        }
    }
    if ((((((F::BtlChar_TestFlag(c, p_def_2, ((((64i32) as i64 as u64)) as u32))) != 0) && (((((((F::BtlAtk_GetId(c, p_atk_1)) as u32)) < (((11i32) as u32))) as i32)) != 0)) as i32)) != 0) {
        F::BtlHit_ApplyGuard(c, p_atk_1, p_def_2, ((1i32) as u32));
        break 'f 1i32;
    }
    if ((((((((((F::BtlChar_TestFlag(c, p_def_2, ((((65i32) as i64 as u64)) as u32))) != 0) && ((((((((F::BtlAtk_GetFlags(c, p_atk_1)) as u64)) as i32)) & (16i32))) != 0)) as i32)) != 0) && ((F::BtlInput_IsCmdHeld(c, p_def_2, ((16i32) as u32))) != 0)) as i32)) != 0) {
        if ((F::BtlInput_IsHeld(c, p_def_2, ((16i32) as u32))) != 0) {
            's1: {
                let sv6: i32 = F::BtlAtk_GetGuardKindB(c, p_atk_1);
                match sv6 {
                    0i32 | 2i32 => {
                        F::BtlHit_ApplyGuard(c, p_atk_1, p_def_2, ((2i32) as u32));
                        break 'f 1i32;
                    }
                    _ => {
                    }
                }
            }
        } else {
            if ((F::BtlInput_IsHeld(c, p_def_2, ((32i32) as u32))) != 0) {
                's2: {
                    let sv7: i32 = F::BtlAtk_GetGuardKindC(c, p_atk_1);
                    match sv7 {
                        0i32 | 2i32 => {
                            F::BtlHit_ApplyGuard(c, p_atk_1, p_def_2, ((3i32) as u32));
                            break 'f 1i32;
                        }
                        _ => {
                        }
                    }
                }
            } else {
                's3: {
                    let sv8: i32 = F::BtlAtk_GetGuardKindA(c, p_atk_1);
                    match sv8 {
                        0i32 | 2i32 => {
                            F::BtlHit_ApplyGuard(c, p_atk_1, p_def_2, ((4i32) as u32));
                            break 'f 1i32;
                        }
                        _ => {
                        }
                    }
                }
            }
        }
    }
    if (((((l_guard_4) == (0i32)) as i32)) != 0) {
        break 'f 0i32;
    }
    l_res_5 = 0i32;
    's4: {
        let sv9: i32 = l_guard_4;
        match sv9 {
            1i32 => {
                l_res_5 = F::BtlAtk_GetGuardKindA(c, p_atk_1);
                break 's4;
            }
            2i32 => {
                l_res_5 = F::BtlAtk_GetGuardKindB(c, p_atk_1);
                break 's4;
            }
            3i32 => {
                l_res_5 = F::BtlAtk_GetGuardKindC(c, p_atk_1);
                break 's4;
            }
            5i32 => {
                l_res_5 = F::BtlAtk_GetGuardKindE(c, p_atk_1);
                break 's4;
            }
            4i32 => {
                l_res_5 = F::BtlAtk_GetBestGuardKind(c, p_atk_1);
                break 's4;
            }
            6i32 => {
                l_res_5 = 0i32;
                break 's4;
            }
            _ => {
            }
        }
    }
    if (((((l_res_5) == (3i32)) as i32)) != 0) {
        break 'f 0i32;
    }
    's5: {
        let sv10: i32 = l_res_5;
        match sv10 {
            0i32 => {
                F::BtlHit_ApplyGuard(c, p_atk_1, p_def_2, ((0i32) as u32));
                break 's5;
            }
            1i32 => {
                F::BtlHit_ApplyGuard(c, p_atk_1, p_def_2, ((5i32) as u32));
                break 's5;
            }
            2i32 => {
                F::BtlHit_ApplyGuard(c, p_atk_1, p_def_2, ((6i32) as u32));
                break 's5;
            }
            4i32 => {
                F::BtlHit_ApplyGuard(c, p_atk_1, p_def_2, ((7i32) as u32));
                break 's5;
            }
            _ => {
            }
        }
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlHit_CheckGuard", &PROBE_N, &[], true); }
    r_
}

pub fn BtlHit_CheckThrowBreak(c: &mut Ctx, mut p_atk_1: u32, mut p_def_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlHit_CheckThrowBreak", &PROBE_N, &[(p_atk_1) as u32, (p_def_2) as u32], false); }
    let mut l_st_3: u32 = 0;
    let r_ = 'f: {
    if (((((F::BtlAtk_GetId(c, p_atk_1)) != (85i32)) as i32)) != 0) {
        break 'f 0i32;
    }
    if (((((F::BtlParam_GetUnk2(c, p_def_2)) == (4i32)) as i32)) != 0) {
        F::BtlChar_SetFlag(c, p_atk_1, ((((122i32) as i64 as u64)) as u32));
        F::BtlCharSnd_PlayCommon(c, p_def_2, 68i32);
        F::BtlChar_PlayVoice(c, p_atk_1, 0i32);
        break 'f 1i32;
    }
    if ((((((((((((F::BtlParam_GetFlags(c, p_def_2)) as u64)) as i32)) & (16384i32))) != 0) && ((((((((F::BtlAtk_GetFlags(c, p_atk_1)) as u64)) as i32)) & (16384i32))) != 0)) as i32)) != 0) {
        F::BtlChar_SetFlag(c, p_atk_1, ((((122i32) as i64 as u64)) as u32));
        F::BtlCharSnd_PlayCommon(c, p_def_2, 68i32);
        F::BtlChar_PlayVoice(c, p_atk_1, 0i32);
        break 'f 1i32;
    }
    if ((((!((F::BtlChar_IsFree(c, p_def_2)) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    l_st_3 = { let a0_ = F::BtlAnim_GetId(c, p_def_2); F::BtlAnim_GetFlags(c, ((((a0_) as i64 as u64)) as u32)) };
    if ((((((((((((((((l_st_3) & (((49i32) as u32)))) != 0) && ((((!((((l_st_3) & (((2048i32) as u32)))) != 0)) as i32)) != 0)) as i32)) != 0) && ((((!((F::BtlHit_IsFromBehind(c, p_atk_1, p_def_2)) != 0)) as i32)) != 0)) as i32)) != 0) && ((((((c.m.r32((p_def_2).wrapping_add(4216)) as i32)) > (0i32)) as i32)) != 0)) as i32)) != 0) {
        F::BtlChar_SetFlag(c, p_atk_1, ((((122i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_def_2, ((((121i32) as i64 as u64)) as u32));
        F::BtlCharSnd_PlayCommon(c, p_def_2, 68i32);
        F::BtlChar_PlayVoice(c, p_atk_1, 0i32);
        F::BtlChar_PlayVoice(c, p_def_2, 17i32);
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlHit_CheckThrowBreak", &PROBE_N, &[], true); }
    r_
}

pub fn BtlHit_BeginFrame(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlHit_BeginFrame", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_st_2: u32 = 0;
    let mut l_react_3: u32 = 0;
    let mut l_obj_4: u32 = 0;
    let r_ = 'f: {
    l_st_2 = (p_chr_1).wrapping_add(3888);
    l_react_3 = (p_chr_1).wrapping_add(4016);
    l_obj_4 = F::BtlChar_GetObj(c, p_chr_1);
    { let v_ = 1i32; let a_ = l_react_3; c.m.w32(a_, (v_) as u32); };
    if ((F::BtlChar_TestFlag(c, p_chr_1, ((((43i32) as i64 as u64)) as u32))) != 0) {
        F::BtlChar_ClearFlag(c, p_chr_1, ((((96i32) as i64 as u64)) as u32));
    }
    if ((((((((c.m.r8((l_obj_4).wrapping_add(3245)) as i8)) as i32)) >= (0i32)) as i32)) != 0) {
        if ((((((c.m.r32((l_st_2).wrapping_add(20)) as i32)) != ((((c.m.r8((l_obj_4).wrapping_add(3245)) as i8)) as i32))) as i32)) != 0) {
            F::BtlChar_ClearFlag(c, p_chr_1, ((((96i32) as i64 as u64)) as u32));
        }
    } else {
        if ((((((c.m.r32((l_st_2).wrapping_add(20)) as i32)) >= (0i32)) as i32)) != 0) {
            if ((((((((((((!((F::BtlChar_TestFlag(c, p_chr_1, ((((96i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) && ((((!((F::BtlChar_TestFlag(c, p_chr_1, ((((43i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0)) as i32)) != 0) && ((((((((c.m.r8((l_obj_4).wrapping_add(3245)) as i8)) as i32)) == ((!((((c.m.r8((l_obj_4).wrapping_add(3244)) as i8)) as i32))))) as i32)) != 0)) as i32)) != 0) {
                F::BtlChar_SetHeldFlag(c, p_chr_1, ((((92i32) as i64 as u64)) as u32));
            }
        }
        F::BtlChar_ClearFlag(c, p_chr_1, ((((96i32) as i64 as u64)) as u32));
    }
    { let v_ = (((c.m.r8((l_obj_4).wrapping_add(3245)) as i8)) as i32); let a_ = (l_st_2).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlHit_BeginFrame", &PROBE_N, &[], true); }
    r_
}

pub fn BtlHit_CheckClash(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlHit_CheckClash", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let mut l_a_2: u32 = 0;
    let mut l_b_3: u32 = 0;
    let mut l_same_4: i32 = 0;
    let mut l_mixed_5: i32 = 0;
    let r_ = 'f: {
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (F::BtlChar_GetCount(c))) as i32)) != 0) { break 'l1; }
            'c2: {
                l_a_2 = F::BtlChar_Get(c, l_i_1);
                l_b_3 = { let a0_ = F::BtlOpp_GetPlayer(c, l_a_2); F::BtlChar_Get(c, a0_) };
                if ((F::BtlChar_TestFlag(c, l_a_2, ((((3i32) as i64 as u64)) as u32))) != 0) {
                    l_same_4 = 0i32;
                    if ((((((F::BtlChar_TestFlag(c, l_a_2, ((((71i32) as i64 as u64)) as u32))) != 0) && ((F::BtlChar_TestFlag(c, l_b_3, ((((71i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) {
                        l_same_4 = 1i32;
                    }
                    l_mixed_5 = 0i32;
                    if ((((((F::BtlChar_TestFlag(c, l_a_2, ((((72i32) as i64 as u64)) as u32))) != 0) && ((F::BtlChar_TestFlag(c, l_b_3, ((((72i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) {
                        l_mixed_5 = 1i32;
                    }
                    if ((((((F::BtlChar_TestFlag(c, l_a_2, ((((71i32) as i64 as u64)) as u32))) != 0) && ((F::BtlChar_TestFlag(c, l_b_3, ((((72i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) {
                        l_mixed_5 = 1i32;
                    }
                    if ((((((F::BtlChar_TestFlag(c, l_a_2, ((((72i32) as i64 as u64)) as u32))) != 0) && ((F::BtlChar_TestFlag(c, l_b_3, ((((71i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) {
                        l_mixed_5 = 1i32;
                    }
                    if ((((((((((l_same_4) != 0) || ((l_mixed_5) != 0)) as i32)) != 0) && ((((((F::BtlMove_IsBlockedByOpponent(c, l_a_2)) != 0) || ((F::BtlMove_IsBlockedByOpponent(c, l_b_3)) != 0)) as i32)) != 0)) as i32)) != 0) {
                        if ((l_same_4) != 0) {
                            F::BtlChar_SetFlag(c, l_a_2, ((((97i32) as i64 as u64)) as u32));
                            F::BtlChar_SetFlag(c, l_b_3, ((((97i32) as i64 as u64)) as u32));
                            { let v_ = 6i32; let a_ = (l_a_2).wrapping_add(4900); c.m.w32(a_, (v_) as u32); };
                            { let v_ = 6i32; let a_ = (l_b_3).wrapping_add(4900); c.m.w32(a_, (v_) as u32); };
                            { let v_ = 0i32; let a_ = (l_a_2).wrapping_add(4904); c.m.w32(a_, (v_) as u32); };
                            { let v_ = 0i32; let a_ = (l_b_3).wrapping_add(4904); c.m.w32(a_, (v_) as u32); };
                            F::BtlCharSnd_PlayCommon(c, l_a_2, 79i32);
                            F::BtlCharSnd_PlayCommon(c, l_b_3, 75i32);
                        } else {
                            F::BtlChar_SetFlag(c, l_a_2, ((((98i32) as i64 as u64)) as u32));
                            F::BtlChar_SetFlag(c, l_b_3, ((((98i32) as i64 as u64)) as u32));
                            { let v_ = 2i32; let a_ = (l_a_2).wrapping_add(4900); c.m.w32(a_, (v_) as u32); };
                            { let v_ = 2i32; let a_ = (l_b_3).wrapping_add(4900); c.m.w32(a_, (v_) as u32); };
                            { let v_ = 0i32; let a_ = (l_a_2).wrapping_add(4904); c.m.w32(a_, (v_) as u32); };
                            { let v_ = 0i32; let a_ = (l_b_3).wrapping_add(4904); c.m.w32(a_, (v_) as u32); };
                            F::BtlCharSnd_PlayCommon(c, l_a_2, 79i32);
                            F::BtlCharSnd_PlayCommon(c, l_b_3, 75i32);
                            F::BtlChar_SetFxBit(c, l_a_2, 44i32);
                        }
                        F::BtlChar_SetFlag(c, l_a_2, ((((68i32) as i64 as u64)) as u32));
                        F::BtlChar_SetFlag(c, l_a_2, ((((69i32) as i64 as u64)) as u32));
                        F::BtlChar_SetFlag(c, l_a_2, ((((70i32) as i64 as u64)) as u32));
                        F::BtlChar_SetFlag(c, l_b_3, ((((68i32) as i64 as u64)) as u32));
                        F::BtlChar_SetFlag(c, l_b_3, ((((69i32) as i64 as u64)) as u32));
                        F::BtlChar_SetFlag(c, l_b_3, ((((70i32) as i64 as u64)) as u32));
                        break 'f 1i32;
                    }
                }
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlHit_CheckClash", &PROBE_N, &[], true); }
    r_
}

pub fn BtlHit_CheckRush(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlHit_CheckRush", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let mut l_a_2: u32 = 0;
    let mut l_b_3: u32 = 0;
    let mut l_st_4: u32 = 0;
    let mut l_tech_5: i32 = 0;
    let mut l_tf_6: u32 = 0;
    let mut l_flag_7: i32 = 0;
    let mut l_n0_8: i32 = 0;
    let mut l_n1_9: i32 = 0;
    let mut sv_10: i32 = 0;
    let mut bb_: u32 = 0;
    let r_ = 'f: {
    'fl1: loop {
        match bb_ {
            0 => {
                l_i_1 = 0i32;
                bb_ = 1; continue 'fl1;
            }
            1 => {
                if (((((l_i_1) < (F::BtlChar_GetCount(c))) as i32)) != 0) { bb_ = 2; continue 'fl1; } else { bb_ = 4; continue 'fl1; }
            }
            2 => {
                l_a_2 = F::BtlChar_Get(c, l_i_1);
                l_b_3 = { let a0_ = F::BtlOpp_GetPlayer(c, l_a_2); F::BtlChar_Get(c, a0_) };
                if ((((!((F::BtlChar_TestFlag(c, l_a_2, ((((73i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) { bb_ = 5; continue 'fl1; } else { bb_ = 6; continue 'fl1; }
            }
            3 => {
                { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
                bb_ = 1; continue 'fl1;
            }
            4 => {
                break 'f 0i32;
            }
            5 => {
                bb_ = 3; continue 'fl1;
            }
            6 => {
                if ((((!((F::BtlMove_IsBlockedByOpponent(c, l_a_2)) != 0)) as i32)) != 0) { bb_ = 8; continue 'fl1; } else { bb_ = 9; continue 'fl1; }
            }
            7 => {
                bb_ = 6; continue 'fl1;
            }
            8 => {
                bb_ = 3; continue 'fl1;
            }
            9 => {
                { let _ = { let a0_ = F::BtlAnim_GetId(c, l_a_2); F::BtlAnim_GetFlags(c, ((((a0_) as i64 as u64)) as u32)) }; };
                l_st_4 = { let a0_ = F::BtlAnim_GetId(c, l_b_3); F::BtlAnim_GetFlags(c, ((((a0_) as i64 as u64)) as u32)) };
                l_tech_5 = F::BtlAct_GetCurrentClass(c, l_a_2);
                l_tf_6 = F::BtlSuper_GetFlags(c, l_a_2, l_tech_5);
                if ((((((F::BtlChar_TestFlag(c, l_b_3, ((((66i32) as i64 as u64)) as u32))) != 0) || ((((l_st_4) & (((2048i32) as u32)))) != 0)) as i32)) != 0) { bb_ = 11; continue 'fl1; } else { bb_ = 12; continue 'fl1; }
            }
            10 => {
                bb_ = 9; continue 'fl1;
            }
            11 => {
                bb_ = 13; continue 'fl1;
            }
            12 => {
                if ((((((((((F::BtlChar_TestFlag(c, l_b_3, ((((73i32) as i64 as u64)) as u32))) != 0) && ((F::BtlChar_TestFlag(c, l_b_3, ((((165i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) && ((((!((F::BtlChar_TestFlag(c, l_a_2, ((((165i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0)) as i32)) != 0) { bb_ = 15; continue 'fl1; } else { bb_ = 16; continue 'fl1; }
            }
            13 => {
                F::BtlChar_SetFlag(c, l_a_2, ((((99i32) as i64 as u64)) as u32));
                bb_ = 3; continue 'fl1;
            }
            14 => {
                bb_ = 12; continue 'fl1;
            }
            15 => {
                bb_ = 13; continue 'fl1;
            }
            16 => {
                if (((((((((((((F::BtlParam_GetUnk2(c, l_b_3)) == (4i32)) as i32)) != 0) && ((((!((((l_tf_6) & (((134217728i32) as u32)))) != 0)) as i32)) != 0)) as i32)) != 0) || ((F::BtlChar_TestFlag(c, l_b_3, ((((89i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) { bb_ = 18; continue 'fl1; } else { bb_ = 19; continue 'fl1; }
            }
            17 => {
                bb_ = 16; continue 'fl1;
            }
            18 => {
                F::BtlChar_SetFlag(c, l_a_2, ((((100i32) as i64 as u64)) as u32));
                F::BtlChar_SetFlag(c, l_a_2, ((((297i32) as i64 as u64)) as u32));
                bb_ = 3; continue 'fl1;
            }
            19 => {
                if ((((((F::BtlChar_IsFree(c, l_b_3)) != 0) && ((((((c.m.r32((l_b_3).wrapping_add(4220)) as i32)) > (0i32)) as i32)) != 0)) as i32)) != 0) { bb_ = 21; continue 'fl1; } else { bb_ = 22; continue 'fl1; }
            }
            20 => {
                bb_ = 19; continue 'fl1;
            }
            21 => {
                if ((F::BtlChar_TestFlag(c, l_a_2, ((((165i32) as i64 as u64)) as u32))) != 0) { bb_ = 23; continue 'fl1; } else { bb_ = 25; continue 'fl1; }
            }
            22 => {
                if ((((((F::BtlChar_IsFree(c, l_b_3)) != 0) && ((((!((F::BtlChar_TestFlag(c, l_b_3, ((((90i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0)) as i32)) != 0) { bb_ = 37; continue 'fl1; } else { bb_ = 38; continue 'fl1; }
            }
            23 => {
                if ((((((((l_st_4) & (((49i32) as u32)))) != 0) && ((((!((F::BtlHit_IsFromBehind(c, l_a_2, l_b_3)) != 0)) as i32)) != 0)) as i32)) != 0) { bb_ = 26; continue 'fl1; } else { bb_ = 27; continue 'fl1; }
            }
            24 => {
                bb_ = 22; continue 'fl1;
            }
            25 => {
                if ((((((((l_st_4) & (((177i32) as u32)))) != 0) && ((((!((F::BtlChar_TestFlag(c, l_b_3, ((((90i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0)) as i32)) != 0) { bb_ = 29; continue 'fl1; } else { bb_ = 30; continue 'fl1; }
            }
            26 => {
                F::BtlChar_SetFlag(c, l_a_2, ((((122i32) as i64 as u64)) as u32));
                F::BtlChar_SetFlag(c, l_b_3, ((((121i32) as i64 as u64)) as u32));
                F::BtlCharSnd_PlayCommon(c, l_b_3, 68i32);
                F::BtlChar_PlayVoice(c, l_a_2, 0i32);
                F::BtlChar_PlayVoice(c, l_b_3, 17i32);
                bb_ = 3; continue 'fl1;
            }
            27 => {
                bb_ = 24; continue 'fl1;
            }
            28 => {
                bb_ = 27; continue 'fl1;
            }
            29 => {
                F::BtlChar_SetFlag(c, l_a_2, ((((99i32) as i64 as u64)) as u32));
                if ((F::BtlChar_FrameMod(c, 2i32)) != 0) { bb_ = 31; continue 'fl1; } else { bb_ = 33; continue 'fl1; }
            }
            30 => {
                bb_ = 24; continue 'fl1;
            }
            31 => {
                l_flag_7 = 116i32;
                bb_ = 32; continue 'fl1;
            }
            32 => {
                F::BtlChar_SetFlag(c, l_b_3, ((((l_flag_7) as i64 as u64)) as u32));
                if ((F::BtlMember_HasAbility(c, l_b_3, 34i32)) != 0) { bb_ = 34; continue 'fl1; } else { bb_ = 35; continue 'fl1; }
            }
            33 => {
                l_flag_7 = 117i32;
                bb_ = 32; continue 'fl1;
            }
            34 => {
                F::BtlMember_AddBlast(c, l_b_3, 100000i32);
                bb_ = 35; continue 'fl1;
            }
            35 => {
                bb_ = 3; continue 'fl1;
            }
            36 => {
                bb_ = 30; continue 'fl1;
            }
            37 => {
                l_n0_8 = 0i32;
                l_n1_9 = 0i32;
                if ((((((c.m.r32((l_b_3).wrapping_add(3584)) as i32)) > (0i32)) as i32)) != 0) { bb_ = 39; continue 'fl1; } else { bb_ = 40; continue 'fl1; }
            }
            38 => {
                if ((((((((((((((F::BtlColl_IsGuarding(c, l_b_3)) != 0) && ((((!((F::BtlHit_IsFromBehind(c, l_a_2, l_b_3)) != 0)) as i32)) != 0)) as i32)) != 0) && ((((!((((l_tf_6) & (((1i32) as u32)))) != 0)) as i32)) != 0)) as i32)) != 0) && ((((!((F::BtlChar_TestFlag(c, l_a_2, ((((165i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0)) as i32)) != 0) { bb_ = 63; continue 'fl1; } else { bb_ = 64; continue 'fl1; }
            }
            39 => {
                l_n0_8 = 1i32;
                bb_ = 40; continue 'fl1;
            }
            40 => {
                if ((((((c.m.r32((l_b_3).wrapping_add(3588)) as i32)) > (0i32)) as i32)) != 0) { bb_ = 41; continue 'fl1; } else { bb_ = 42; continue 'fl1; }
            }
            41 => {
                l_n1_9 = 1i32;
                bb_ = 42; continue 'fl1;
            }
            42 => {
                if ((((((((((l_n0_8) != 0) || ((l_n1_9) != 0)) as i32)) != 0) && ((((l_st_4) & (((177i32) as u32)))) != 0)) as i32)) != 0) { bb_ = 43; continue 'fl1; } else { bb_ = 44; continue 'fl1; }
            }
            43 => {
                if ((l_n0_8) != 0) { bb_ = 45; continue 'fl1; } else { bb_ = 46; continue 'fl1; }
            }
            44 => {
                bb_ = 38; continue 'fl1;
            }
            45 => {
                { let a_ = (l_b_3).wrapping_add(3584); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                bb_ = 46; continue 'fl1;
            }
            46 => {
                if ((l_n1_9) != 0) { bb_ = 47; continue 'fl1; } else { bb_ = 48; continue 'fl1; }
            }
            47 => {
                { let a_ = (l_b_3).wrapping_add(3588); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                bb_ = 48; continue 'fl1;
            }
            48 => {
                F::BtlChar_SetFlag(c, l_a_2, ((((99i32) as i64 as u64)) as u32));
                if ((l_n0_8) != 0) { bb_ = 49; continue 'fl1; } else { bb_ = 50; continue 'fl1; }
            }
            49 => {
                if ((F::BtlChar_FrameMod(c, 2i32)) != 0) { bb_ = 51; continue 'fl1; } else { bb_ = 53; continue 'fl1; }
            }
            50 => {
                if ((l_n1_9) != 0) { bb_ = 54; continue 'fl1; } else { bb_ = 55; continue 'fl1; }
            }
            51 => {
                l_flag_7 = 116i32;
                bb_ = 52; continue 'fl1;
            }
            52 => {
                F::BtlChar_SetFlag(c, l_b_3, ((((l_flag_7) as i64 as u64)) as u32));
                bb_ = 50; continue 'fl1;
            }
            53 => {
                l_flag_7 = 117i32;
                bb_ = 52; continue 'fl1;
            }
            54 => {
                F::BtlChar_SetFlag(c, l_b_3, ((((119i32) as i64 as u64)) as u32));
                bb_ = 55; continue 'fl1;
            }
            55 => {
                sv_10 = (c.m.r32((l_b_3).wrapping_add(3596)) as i32);
                bb_ = match sv_10 { 0i32 => 58, 1i32 => 60, _ => 56, }; continue 'fl1;
            }
            56 => {
                bb_ = 3; continue 'fl1;
            }
            57 => {
                bb_ = 58; continue 'fl1;
            }
            58 => {
                F::BtlChar_SetFlag(c, l_b_3, ((((222i32) as i64 as u64)) as u32));
                bb_ = 56; continue 'fl1;
            }
            59 => {
                bb_ = 60; continue 'fl1;
            }
            60 => {
                F::BtlChar_SetFlag(c, l_b_3, ((((223i32) as i64 as u64)) as u32));
                bb_ = 56; continue 'fl1;
            }
            61 => {
                bb_ = 56; continue 'fl1;
            }
            62 => {
                bb_ = 44; continue 'fl1;
            }
            63 => {
                F::BtlChar_SetFlag(c, l_a_2, ((((111i32) as i64 as u64)) as u32));
                F::BtlChar_SetFlag(c, l_a_2, ((((297i32) as i64 as u64)) as u32));
                F::BtlChar_SetFxBit(c, l_b_3, 32i32);
                F::BtlChar_SetFlag(c, l_b_3, ((((109i32) as i64 as u64)) as u32));
                F::BtlCharSnd_PlayCommon(c, l_a_2, 66i32);
                F::BtlChar_SetFlag(c, l_b_3, ((((148i32) as i64 as u64)) as u32));
                { let v_ = { let a0_ = rt::fadd({ let a_ = (F::BtlChar_GetPos(c, l_a_2)).wrapping_add(148); F32(c.m.r32(a_)) }, F32(0x40490fda)); F::BtlUtil_WrapAngle(c, a0_) }; let a_ = ((l_b_3).wrapping_add(4016)).wrapping_add(32); c.m.w32(a_, (v_).0); };
                { let v_ = F32(0x00000000); let a_ = ((l_b_3).wrapping_add(4016)).wrapping_add(36); c.m.w32(a_, (v_).0); };
                { let _ = F::BtlMember_SpendKi(c, l_b_3, 20000i32, 0i32); };
                if ((F::BtlMember_IsKiEmpty(c, l_b_3)) != 0) { bb_ = 65; continue 'fl1; } else { bb_ = 66; continue 'fl1; }
            }
            64 => {
                if ((((!((F::BtlChar_TestFlag(c, l_a_2, ((((3i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) { bb_ = 68; continue 'fl1; } else { bb_ = 69; continue 'fl1; }
            }
            65 => {
                F::BtlChar_SetHeldFlag(c, l_b_3, ((((190i32) as i64 as u64)) as u32));
                bb_ = 66; continue 'fl1;
            }
            66 => {
                bb_ = 3; continue 'fl1;
            }
            67 => {
                bb_ = 64; continue 'fl1;
            }
            68 => {
                bb_ = 13; continue 'fl1;
            }
            69 => {
                if ((F::BtlColl_StartThrow(c, l_a_2, l_b_3, l_tech_5, l_tf_6)) != 0) { bb_ = 71; continue 'fl1; } else { bb_ = 72; continue 'fl1; }
            }
            70 => {
                bb_ = 69; continue 'fl1;
            }
            71 => {
                break 'f 1i32;
            }
            72 => {
                bb_ = 3; continue 'fl1;
            }
            73 => {
                bb_ = 72; continue 'fl1;
            }
            74 => {
                break 'fl1;
            }
            _ => unreachable!(),
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlHit_CheckRush", &PROBE_N, &[], true); }
    r_
}

pub fn BtlHit_CheckProximityAll(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlHit_CheckProximityAll", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let mut l_j_2: i32 = 0;
    let mut l_a_3: u32 = 0;
    let mut l_b_4: u32 = 0;
    let r_ = 'f: {
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (F::BtlChar_GetCount(c))) as i32)) != 0) { break 'l1; }
            'c2: {
                {
                    l_j_2 = 0i32;
                    'l3: loop {
                        if !(((((l_j_2) < (F::BtlChar_GetCount(c))) as i32)) != 0) { break 'l3; }
                        'c4: {
                            if (((((l_i_1) != (l_j_2)) as i32)) != 0) {
                                l_a_3 = F::BtlChar_Get(c, l_i_1);
                                l_b_4 = F::BtlChar_Get(c, l_j_2);
                                { let _ = F::BtlHit_CheckNear(c, l_a_3, l_b_4); };
                                { let _ = F::BtlHit_CheckGrab(c, l_a_3, l_b_4); };
                                { let _ = F::BtlHit_CheckTouch(c, l_a_3, l_b_4); };
                            }
                        }
                        { let o_ = l_j_2; let n_ = (o_).wrapping_add(1); l_j_2 = n_; };
                    }
                }
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlHit_CheckProximityAll", &PROBE_N, &[], true); }
    r_
}

pub fn BtlHit_TestHit(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlHit_TestHit", &PROBE_N, &[(p_chr_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_st_2: u32 = 0;
    let mut l_obj_3: u32 = 0;
    let mut l_ok_4: i32 = 0;
    let mut l_oppIdx_5: i32 = 0;
    let mut l_opp_6: u32 = 0;
    let r_ = 'f: {
        l_st_2 = (p_chr_1).wrapping_add(3888);
        l_obj_3 = F::BtlChar_GetObj(c, p_chr_1);
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((96i32) as i64 as u64)) as u32))) != 0) {
            break 'f;
        }
        if (((((F::BtlAtk_GetId(c, p_chr_1)) == ((1i32).wrapping_neg())) as i32)) != 0) {
            break 'f;
        }
        if (((((((({ let a_ = (c.m.r32((l_obj_3).wrapping_add(5728))).wrapping_add(98336); c.m.r32(a_) }) & (((251658240i32) as u32)))) != 0) || ((F::BtlChar_TestFlag(c, p_chr_1, ((((101i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) {
            l_ok_4 = 0i32;
            l_oppIdx_5 = F::BtlOpp_GetPlayer(c, p_chr_1);
            l_opp_6 = F::BtlChar_Get(c, l_oppIdx_5);
            if (((rt::flt({ let a0_ = F::BtlOpp_GetYawFromFacing(c, p_chr_1); rt::ffabs(a0_) }, F32(0x3fc90fda)) as i32)) != 0) {
                l_ok_4 = 1i32;
            }
            if ((((((((F::BtlAtk_GetFlags(c, p_chr_1)) as u64)) as i32)) & (8i32))) != 0) {
                l_ok_4 = 1i32;
            }
            if ((l_ok_4) != 0) {
                F::BtlChar_SetHeldFlag(c, p_chr_1, ((((96i32) as i64 as u64)) as u32));
                if ((((((((((((((F::BtlHit_IsVoid(c, p_chr_1, l_opp_6)) != 0) || ((F::BtlHit_CheckDodge(c, p_chr_1, l_opp_6)) != 0)) as i32)) != 0) || ((F::BtlHit_CheckArmor(c, p_chr_1, l_opp_6)) != 0)) as i32)) != 0) || ((F::BtlHit_CheckRepel(c, p_chr_1, l_opp_6)) != 0)) as i32)) != 0) {
                    F::BtlChar_SetHeldFlag(c, p_chr_1, ((((92i32) as i64 as u64)) as u32));
                } else {
                    if ((F::BtlHit_CheckGuard(c, p_chr_1, l_opp_6, fp)) != 0) {
                        if (((c.m.r32(fp) as i32)) != 0) {
                            { let a1_ = F::BtlAtk_GetGuardDamage(c, p_chr_1); { let _ = F::BtlMember_Damage(c, l_opp_6, a1_, 12i32); } };
                        }
                    } else {
                        if ((((!((F::BtlHit_CheckThrowBreak(c, p_chr_1, l_opp_6)) != 0)) as i32)) != 0) {
                            { let v_ = l_oppIdx_5; let a_ = (l_st_2).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
                        }
                    }
                }
            }
        }
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((101i32) as i64 as u64)) as u32))) != 0) {
            { let a_ = ((p_chr_1).wrapping_add(3888)).wrapping_add(24); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlHit_TestHit", &PROBE_N, &[], true); }
    r_
}

pub fn BtlHit_ResolveTrades(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlHit_ResolveTrades", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let mut l_j_2: i32 = 0;
    let mut l_a_3: u32 = 0;
    let mut l_b_4: u32 = 0;
    let mut l_sa_5: u32 = 0;
    let mut l_sb_6: u32 = 0;
    let mut l_pa_7: i32 = 0;
    let mut l_pb_8: i32 = 0;
    let r_ = 'f: {
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (F::BtlChar_GetCount(c))) as i32)) != 0) { break 'l1; }
            'c2: {
                {
                    l_j_2 = (l_i_1).wrapping_add(1i32);
                    'l3: loop {
                        if !(((((l_j_2) < (F::BtlChar_GetCount(c))) as i32)) != 0) { break 'l3; }
                        'c4: {
                            l_a_3 = F::BtlChar_Get(c, l_i_1);
                            l_b_4 = F::BtlChar_Get(c, l_j_2);
                            l_sa_5 = (l_a_3).wrapping_add(3888);
                            l_sb_6 = (l_b_4).wrapping_add(3888);
                            if ((((((((((c.m.r32((l_sa_5).wrapping_add(16)) as i32)) == ((c.m.r32(l_b_4) as i32))) as i32)) != 0) && ((((((c.m.r32((l_sb_6).wrapping_add(16)) as i32)) == ((c.m.r32(l_a_3) as i32))) as i32)) != 0)) as i32)) != 0) {
                                l_pa_7 = F::BtlAtk_GetPriority(c, l_a_3);
                                l_pb_8 = F::BtlAtk_GetPriority(c, l_b_4);
                                if (((((l_pa_7) > (l_pb_8)) as i32)) != 0) {
                                    { let v_ = (1i32).wrapping_neg(); let a_ = (l_sb_6).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
                                } else {
                                    if (((((l_pa_7) < (l_pb_8)) as i32)) != 0) {
                                        { let v_ = (1i32).wrapping_neg(); let a_ = (l_sa_5).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
                                    } else {
                                        { let v_ = (1i32).wrapping_neg(); let a_ = (l_sa_5).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
                                        { let v_ = (1i32).wrapping_neg(); let a_ = (l_sb_6).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
                                        F::BtlChar_SetFxBit(c, l_a_3, 34i32);
                                        F::BtlChar_SetFxBit(c, l_b_4, 34i32);
                                        F::ChrCam_AddShake(c, l_a_3, F32(0x40000000), F32(0x3e999999));
                                        F::ChrCam_AddShake(c, l_b_4, F32(0x40000000), F32(0x3e999999));
                                        F::BtlChar_Vibrate(c, l_a_3, F32(0x3f800000), F32(0x3dcccccc));
                                        F::BtlChar_Vibrate(c, l_b_4, F32(0x3f800000), F32(0x3dcccccc));
                                        F::BtlChar_SetHeldFlag(c, l_a_3, ((((93i32) as i64 as u64)) as u32));
                                        F::BtlChar_SetHeldFlag(c, l_b_4, ((((93i32) as i64 as u64)) as u32));
                                    }
                                }
                            }
                        }
                        { let o_ = l_j_2; let n_ = (o_).wrapping_add(1); l_j_2 = n_; };
                    }
                }
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlHit_ResolveTrades", &PROBE_N, &[], true); }
    r_
}

pub fn BtlHit_ApplyHit(c: &mut Ctx, mut p_atk_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlHit_ApplyHit", &PROBE_N, &[(p_atk_1) as u32], false); }
    let mut l_drain_2: i32 = 0;
    let mut l_launch_3: i32 = 0;
    let mut l_def_4: u32 = 0;
    let mut l_dobj_5: u32 = 0;
    let mut l_kind_6: i32 = 0;
    let mut l_af_7: i32 = 0;
    let mut l_state_8: i32 = 0;
    let mut l_st_9: u32 = 0;
    let mut l_yaw_10: F32 = F32(0);
    let mut l_back_11: i32 = 0;
    let mut l_react_12: i32 = 0;
    let mut l_power_13: F32 = F32(0);
    let mut l_time_14: F32 = F32(0);
    let mut l_a_15: F32 = F32(0);
    let mut l_b_16: F32 = F32(0);
    let mut l_lvl_17: i32 = 0;
    let mut l_la_18: F32 = F32(0);
    let mut l_lb_19: F32 = F32(0);
    let mut l_v_20: i32 = 0;
    let mut l_type_21: i32 = 0;
    let mut l_dmg_22: i32 = 0;
    let r_ = 'f: {
    l_drain_2 = 0i32;
    l_launch_3 = 0i32;
    if ((((((c.m.r32(((p_atk_1).wrapping_add(3888)).wrapping_add(16)) as i32)) < (0i32)) as i32)) != 0) {
        break 'f;
    }
    l_def_4 = { let a0_ = (c.m.r32(((p_atk_1).wrapping_add(3888)).wrapping_add(16)) as i32); F::BtlChar_Get(c, a0_) };
    l_dobj_5 = F::BtlChar_GetObj(c, l_def_4);
    l_kind_6 = F::BtlAtk_GetId(c, p_atk_1);
    l_af_7 = ((((F::BtlAtk_GetFlags(c, p_atk_1)) as u64)) as i32);
    l_state_8 = F::BtlAnim_GetId(c, l_def_4);
    l_st_9 = F::BtlAnim_GetFlags(c, ((((l_state_8) as i64 as u64)) as u32));
    if ((((l_af_7) & (268435456i32))) != 0) {
        l_yaw_10 = F::BtlOpp_GetYaw(c, p_atk_1);
    } else {
        l_yaw_10 = { let a_ = (F::BtlChar_GetPos(c, p_atk_1)).wrapping_add(148); F32(c.m.r32(a_)) };
    }
    l_back_11 = F::BtlHit_IsFromBehind(c, p_atk_1, l_def_4);
    if ((((l_st_9) & (((4096i32) as u32)))) != 0) {
        l_react_12 = F::BtlAtk_GetReactionD(c, p_atk_1);
    } else {
        if ((((l_st_9) & (((8192i32) as u32)))) != 0) {
            l_react_12 = F::BtlAtk_GetReactionE(c, p_atk_1);
        } else {
            if ((((((((l_st_9) & (((16384i32) as u32)))) != 0) || (((((F::BtlAct_GetCurrent(c, l_def_4)) == (223i32)) as i32)) != 0)) as i32)) != 0) {
                l_react_12 = F::BtlAtk_GetReactionC(c, p_atk_1);
            } else {
                if ((((((((((((l_state_8).wrapping_sub(174i32)) as u32)) < (((2i32) as u32))) as i32)) != 0) || ((((l_st_9) & (((512i32) as u32)))) != 0)) as i32)) != 0) {
                    l_react_12 = F::BtlAtk_GetReactionG(c, p_atk_1);
                } else {
                    l_lvl_17 = 0i32;
                    if ((((((((F::BtlParam_GetFlags(c, l_def_4)) as u64)) as i32)) & (4i32))) != 0) {
                        l_lvl_17 = 1i32;
                    }
                    if ((((((((F::BtlParam_GetFlags(c, l_def_4)) as u64)) as i32)) & (8i32))) != 0) {
                        { let o_ = l_lvl_17; let n_ = (o_).wrapping_sub(1); l_lvl_17 = n_; };
                    }
                    if ((((((((F::BtlParam_GetFlags(c, p_atk_1)) as u64)) as i32)) & (4i32))) != 0) {
                        { let o_ = l_lvl_17; let n_ = (o_).wrapping_sub(1); l_lvl_17 = n_; };
                    }
                    if ((((((((F::BtlParam_GetFlags(c, p_atk_1)) as u64)) as i32)) & (8i32))) != 0) {
                        { let o_ = l_lvl_17; let n_ = (o_).wrapping_add(1); l_lvl_17 = n_; };
                    }
                    if ((F::BtlAct_TestPoweredSkill(c, l_def_4, ((((64i32) as i64 as u64)) as u32))) != 0) {
                        { let o_ = l_lvl_17; let n_ = (o_).wrapping_add(1); l_lvl_17 = n_; };
                    }
                    if ((((((c.m.r32((l_def_4).wrapping_add(3604)) as i32)) > (0i32)) as i32)) != 0) {
                        { let o_ = l_lvl_17; let n_ = (o_).wrapping_add(1); l_lvl_17 = n_; };
                    }
                    if ((F::BtlMember_HasAbility(c, l_def_4, 70i32)) != 0) {
                        { let o_ = l_lvl_17; let n_ = (o_).wrapping_add(1); l_lvl_17 = n_; };
                    }
                    if ((F::BtlAct_TestPoweredSkill(c, p_atk_1, ((((256i32) as i64 as u64)) as u32))) != 0) {
                        { let o_ = l_lvl_17; let n_ = (o_).wrapping_sub(1); l_lvl_17 = n_; };
                    }
                    if ((F::BtlMember_HasAbility(c, p_atk_1, 71i32)) != 0) {
                        { let o_ = l_lvl_17; let n_ = (o_).wrapping_sub(1); l_lvl_17 = n_; };
                    } else {
                        if ((F::BtlMember_HasAbility(c, p_atk_1, 112i32)) != 0) {
                            if ((F::BtlChar_TestFlag(c, p_atk_1, ((((6i32) as i64 as u64)) as u32))) != 0) {
                                { let o_ = l_lvl_17; let n_ = (o_).wrapping_sub(1); l_lvl_17 = n_; };
                            }
                        }
                    }
                    { let r_ = F::BtlAtk_GetUnk2C(c, p_atk_1); let n_ = (l_lvl_17).wrapping_sub(r_); l_lvl_17 = n_; };
                    if (((((((((l_lvl_17) > (0i32)) as i32)) != 0) && ((((l_af_7) & (4i32))) != 0)) as i32)) != 0) {
                        l_react_12 = 2i32;
                        if ((((((((F::BtlParam_GetFlags(c, l_def_4)) as u64)) as i32)) & (4i32))) != 0) {
                            l_drain_2 = 1i32;
                        }
                    } else {
                        if ((((l_st_9) & (((2048i32) as u32)))) != 0) {
                            l_react_12 = F::BtlAtk_GetReactionB(c, p_atk_1);
                        } else {
                            if ((((l_st_9) & (((256i32) as u32)))) != 0) {
                                if ((((((l_back_11) != 0) || ((((((F::BtlObjAnim_QueryEvent(c, l_dobj_5, ((1i32) as u64), 0i32, ((((3i32) as i64 as u64)) as u32))) != 0) && ((((((((c.m.r8((l_dobj_5).wrapping_add(3245)) as i8)) as i32)) == ((!((((c.m.r8((l_dobj_5).wrapping_add(3244)) as i8)) as i32))))) as i32)) != 0)) as i32)) != 0)) as i32)) != 0) {
                                    l_react_12 = F::BtlAtk_GetReaction(c, p_atk_1);
                                } else {
                                    l_react_12 = F::BtlAtk_GetReactionF(c, p_atk_1);
                                }
                            } else {
                                l_react_12 = F::BtlAtk_GetReaction(c, p_atk_1);
                            }
                        }
                    }
                }
            }
        }
    }
    if (((((l_react_12) == (1i32)) as i32)) != 0) {
        F::BtlChar_SetHeldFlag(c, p_atk_1, ((((92i32) as i64 as u64)) as u32));
        break 'f;
    }
    if (((((l_react_12) == (20i32)) as i32)) != 0) {
        if ((((((((F::BtlParam_GetFlags(c, p_atk_1)) as u64)) as i32)) & (512i32))) != 0) {
            l_react_12 = 37i32;
        }
    }
    's1: {
        let sv23: i32 = l_react_12;
        match sv23 {
            20i32 | 21i32 | 22i32 | 37i32 | 49i32 => {
                l_launch_3 = 1i32;
                break 's1;
            }
            _ => {
                if ((((((c.m.r32((l_def_4).wrapping_add(3608)) as i32)) > (0i32)) as i32)) != 0) {
                    l_react_12 = 2i32;
                }
                break 's1;
            }
        }
    }
    { let v_ = l_react_12; let a_ = (l_def_4).wrapping_add(4016); c.m.w32(a_, (v_) as u32); };
    if (((((l_react_12) != (2i32)) as i32)) != 0) {
        { let v_ = F::BtlAtk_GetReactionSub(c, p_atk_1); let a_ = ((l_def_4).wrapping_add(4016)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        if ((F::BtlChar_TestMemberUnk70(c, p_atk_1)) != 0) {
            { let v_ = 19i32; let a_ = ((l_def_4).wrapping_add(4016)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        }
        { let v_ = (((l_af_7).wrapping_shr((10i32) as u32)) & (1i32)); let a_ = ((l_def_4).wrapping_add(4016)).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
        if (((((l_react_12) == (20i32)) as i32)) != 0) {
            { let v_ = F32(0x3f800000); let a_ = ((l_def_4).wrapping_add(4016)).wrapping_add(24); c.m.w32(a_, (v_).0); };
        } else {
            if ((((l_af_7) & (33554432i32))) != 0) {
                { let v_ = F32(c.m.r32((p_atk_1).wrapping_add(3448))); let a_ = ((l_def_4).wrapping_add(4016)).wrapping_add(24); c.m.w32(a_, (v_).0); };
            } else {
                { let v_ = rt::fmul(F32(c.m.r32((p_atk_1).wrapping_add(3448))), F::BtlParam_GetHitReactScale(c, l_def_4)); let a_ = ((l_def_4).wrapping_add(4016)).wrapping_add(24); c.m.w32(a_, (v_).0); };
            }
        }
        if ((((l_af_7) & (64i32))) != 0) {
            F::BtlChar_SetHeldFlag(c, l_def_4, ((((14i32) as i64 as u64)) as u32));
        }
        if ((((l_af_7) & (4194304i32))) != 0) {
            F::BtlChar_SetHeldFlag(c, l_def_4, ((((30i32) as i64 as u64)) as u32));
        }
        if ((((((c.m.r32(((l_def_4).wrapping_add(4016)).wrapping_add(20)) as i32)) == (0i32)) as i32)) != 0) {
            { let a1_ = { let a0_ = (c.m.r32((l_def_4).wrapping_add(4016)) as i32); F::BtlHit_GetVoiceKind(c, a0_) }; F::BtlChar_PlayVoice(c, l_def_4, a1_) };
        }
    } else {
        { let v_ = 3i32; let a_ = ((l_def_4).wrapping_add(4016)).wrapping_add(56); c.m.w32(a_, (v_) as u32); };
        if ((l_drain_2) != 0) {
            { let _ = F::BtlMember_SpendKi(c, l_def_4, 2000i32, 0i32); };
        }
    }
    if ((F::BtlAtk_GetHitFxKind(c, p_atk_1)) != 0) {
        F::BtlChar_SetFxBit(c, l_def_4, 29i32);
    }
    l_power_13 = F::BtlAtk_GetShakePower(c, p_atk_1);
    l_time_14 = F::BtlAtk_GetShakeTime(c, p_atk_1);
    if ((((l_af_7) & (2048i32))) != 0) {
        F::ChrCam_AddShake(c, p_atk_1, l_power_13, l_time_14);
    }
    F::ChrCam_AddShake(c, l_def_4, l_power_13, l_time_14);
    F::BtlChar_Vibrate(c, p_atk_1, rt::fmul(l_power_13, F32(0x3f000000)), rt::fmul(l_time_14, F32(0x3f000000)));
    F::BtlChar_Vibrate(c, l_def_4, rt::fmul(l_power_13, F32(0x3f000000)), rt::fmul(l_time_14, F32(0x3f000000)));
    if ((((!((((l_af_7) & (524288i32))) != 0)) as i32)) != 0) {
        F::BtlChar_SetFxBit(c, p_atk_1, 27i32);
    }
    if ((((l_af_7) & (65536i32))) != 0) {
        F::BtlChar_SetFxBit(c, p_atk_1, 37i32);
    }
    if ((((l_af_7) & (131072i32))) != 0) {
        F::BtlChar_SetFxBit(c, p_atk_1, 38i32);
    }
    if ((((l_af_7) & (262144i32))) != 0) {
        F::BtlChar_SetFxBit(c, p_atk_1, 39i32);
    }
    F::BtlChar_ClearFlag(c, p_atk_1, ((((138i32) as i64 as u64)) as u32));
    F::BtlChar_SetHeldFlag(c, p_atk_1, ((((91i32) as i64 as u64)) as u32));
    if ((((l_st_9) & (((2i32) as u32)))) != 0) {
        F::BtlChar_SetFlag(c, p_atk_1, ((((113i32) as i64 as u64)) as u32));
    }
    if ((((l_af_7) & (512i32))) != 0) {
        F::BtlChar_SetHeldFlag(c, p_atk_1, ((((134i32) as i64 as u64)) as u32));
    }
    if ((((l_af_7) & (32i32))) != 0) {
        F::BtlMove_AimVerticalAtOpponent(c, p_atk_1, 0i32);
    }
    if (((((l_react_12) == (49i32)) as i32)) != 0) {
        F::BtlChar_SetFlag(c, p_atk_1, ((((127i32) as i64 as u64)) as u32));
    }
    if (((((l_react_12) != (2i32)) as i32)) != 0) {
        if ((l_launch_3) != 0) {
            l_la_18 = F::BtlAtk_GetLaunchAngleA(c, p_atk_1);
            l_lb_19 = F::BtlAtk_GetLaunchAngleB(c, p_atk_1);
            { let v_ = (((l_af_7).wrapping_shr((12i32) as u32)) & (1i32)); let a_ = ((l_def_4).wrapping_add(4016)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
            if ((((((F::BtlChar_TestMemberUnk70(c, p_atk_1)) != 0) && (((((l_kind_6) == (85i32)) as i32)) != 0)) as i32)) != 0) {
                { let v_ = 0i32; let a_ = ((l_def_4).wrapping_add(4016)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                l_la_18 = F32(0x00000000);
                l_lb_19 = F32(0xbf490fd9);
            }
            { let v_ = l_yaw_10; let a_ = ((l_def_4).wrapping_add(4016)).wrapping_add(28); c.m.w32(a_, (v_).0); };
            { let v_ = l_la_18; let a_ = ((l_def_4).wrapping_add(4016)).wrapping_add(40); c.m.w32(a_, (v_).0); };
            { let v_ = l_lb_19; let a_ = ((l_def_4).wrapping_add(4016)).wrapping_add(44); c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x00000000); let a_ = ((l_def_4).wrapping_add(4016)).wrapping_add(32); c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x00000000); let a_ = ((l_def_4).wrapping_add(4016)).wrapping_add(36); c.m.w32(a_, (v_).0); };
            F::BtlChar_SetFlag(c, p_atk_1, ((((68i32) as i64 as u64)) as u32));
            F::BtlChar_SetFlag(c, l_def_4, ((((68i32) as i64 as u64)) as u32));
            F::BtlChar_SetFlag(c, p_atk_1, ((((69i32) as i64 as u64)) as u32));
            F::BtlChar_SetFlag(c, l_def_4, ((((69i32) as i64 as u64)) as u32));
            F::BtlChar_SetFlag(c, p_atk_1, ((((70i32) as i64 as u64)) as u32));
            F::BtlChar_SetFlag(c, l_def_4, ((((70i32) as i64 as u64)) as u32));
        } else {
            { let v_ = l_back_11; let a_ = ((l_def_4).wrapping_add(4016)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
            { let v_ = l_yaw_10; let a_ = ((l_def_4).wrapping_add(4016)).wrapping_add(28); c.m.w32(a_, (v_).0); };
            { let a1_ = F32(c.m.r32(((l_def_4).wrapping_add(4016)).wrapping_add(28))); let a2_ = F::BtlAtk_GetPushOnHit(c, p_atk_1); F::BtlMove_SetImpulseYaw(c, l_def_4, a1_, a2_) };
            if ((((l_af_7) & (256i32))) != 0) {
                F::BtlChar_SetFlag(c, l_def_4, ((((148i32) as i64 as u64)) as u32));
                l_a_15 = F::BtlAtk_GetLaunchAngleA(c, p_atk_1);
                l_b_16 = F::BtlAtk_GetLaunchAngleB(c, p_atk_1);
                if ((((l_af_7) & (2097152i32))) != 0) {
                    l_a_15 = F32(0x00000000);
                    l_b_16 = { let a_ = (F::BtlChar_GetPos(c, p_atk_1)).wrapping_add(144); F32(c.m.r32(a_)) };
                }
                if (((c.m.r32(((l_def_4).wrapping_add(4016)).wrapping_add(12)) as i32)) != 0) {
                    { let v_ = { let a0_ = rt::fadd(F32(c.m.r32(((l_def_4).wrapping_add(4016)).wrapping_add(28))), l_a_15); F::BtlUtil_WrapAngle(c, a0_) }; let a_ = ((l_def_4).wrapping_add(4016)).wrapping_add(32); c.m.w32(a_, (v_).0); };
                    { let v_ = l_b_16; let a_ = ((l_def_4).wrapping_add(4016)).wrapping_add(36); c.m.w32(a_, (v_).0); };
                } else {
                    { let v_ = { let a0_ = rt::fadd({ let a0_ = rt::fadd(F32(c.m.r32(((l_def_4).wrapping_add(4016)).wrapping_add(28))), l_a_15); F::BtlUtil_WrapAngle(c, a0_) }, F32(0x40490fda)); F::BtlUtil_WrapAngle(c, a0_) }; let a_ = ((l_def_4).wrapping_add(4016)).wrapping_add(32); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fneg(l_b_16); let a_ = ((l_def_4).wrapping_add(4016)).wrapping_add(36); c.m.w32(a_, (v_).0); };
                }
            }
            { let v_ = F32(0x00000000); let a_ = ((l_def_4).wrapping_add(4016)).wrapping_add(40); c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x00000000); let a_ = ((l_def_4).wrapping_add(4016)).wrapping_add(44); c.m.w32(a_, (v_).0); };
        }
    }
    if (((((l_kind_6) == (85i32)) as i32)) != 0) {
        l_v_20 = (((((((F::BtlParam_GetFlags(c, p_atk_1)) as u64)) as i32)).wrapping_shr((19i32) as u32)) & (1i32));
        { let v_ = l_v_20; let a_ = ((p_atk_1).wrapping_add(4016)).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
        { let v_ = l_v_20; let a_ = ((l_def_4).wrapping_add(4016)).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
    } else {
        { let v_ = 0i32; let a_ = ((p_atk_1).wrapping_add(4016)).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = ((l_def_4).wrapping_add(4016)).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
    }
    's2: {
        let sv24: i32 = l_react_12;
        match sv24 {
            20i32 | 22i32 | 37i32 => {
                { let v_ = F::BtlAtk_GetDamage(c, p_atk_1); let a_ = ((l_def_4).wrapping_add(4016)).wrapping_add(60); c.m.w32(a_, (v_) as u32); };
                break 's2;
            }
            _ => {
                {
                    l_type_21 = 0i32;
                    l_dmg_22 = F::BtlAtk_GetDamage(c, p_atk_1);
                    's3: {
                        let sv25: i32 = l_kind_6;
                        match sv25 {
                            117i32 | 118i32 | 119i32 | 120i32 => {
                                { let r_ = rt::div_i32((l_dmg_22).wrapping_mul({ let a0_ = (((c.m.r32((p_atk_1).wrapping_add(3464)) as i32)).wrapping_add((c.m.r32((l_def_4).wrapping_add(3464)) as i32))).wrapping_sub(1i32); F::BtlUtil_Clamp(c, a0_, 0i32, 5i32) }), 5i32); let n_ = (l_dmg_22).wrapping_add(r_); l_dmg_22 = n_; };
                                break 's3;
                            }
                            _ => {
                                if (((c.m.r32(((l_def_4).wrapping_add(4016)).wrapping_add(12)) as i32)) != 0) {
                                    if ((((((((!((((l_af_7) & (134217728i32))) != 0)) as i32)) != 0) && ((((!((((l_st_9) & (((2048i32) as u32)))) != 0)) as i32)) != 0)) as i32)) != 0) {
                                        { let r_ = rt::div_i32(l_dmg_22, 5i32); let n_ = (l_dmg_22).wrapping_add(r_); l_dmg_22 = n_; };
                                    }
                                }
                                break 's3;
                            }
                        }
                    }
                    's4: {
                        let sv26: i32 = l_kind_6;
                        match sv26 {
                            67i32 | 68i32 | 69i32 | 70i32 | 71i32 => {
                                l_type_21 = 134217728i32;
                                break 's4;
                            }
                            62i32 | 63i32 | 64i32 | 65i32 | 66i32 => {
                                l_type_21 = 67108864i32;
                                break 's4;
                            }
                            97i32 | 98i32 => {
                                l_type_21 = (2097152i32).wrapping_add((c.m.r32((p_atk_1).wrapping_add(3596)) as i32));
                                break 's4;
                            }
                            _ => {
                            }
                        }
                    }
                    { let _ = F::BtlMember_Damage(c, l_def_4, l_dmg_22, l_type_21); };
                    break 's2;
                }
            }
        }
    }
    if ((((!((F::BtlChar_TestFlag(c, p_atk_1, ((((152i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) {
        { let a1_ = F::BtlAtk_GetKiGain(c, p_atk_1); { let _ = F::BtlMember_AddKi(c, p_atk_1, a1_); } };
    }
    { let r_ = F::BtlAtk_GetUnk10(c, p_atk_1); let a_ = (p_atk_1).wrapping_add(3456); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
    if ((((((c.m.r32((p_atk_1).wrapping_add(3456)) as i32)) > (100000i32)) as i32)) != 0) {
        { let v_ = 100000i32; let a_ = (p_atk_1).wrapping_add(3456); c.m.w32(a_, (v_) as u32); };
    }
    F::BtlChar_RaiseFirstClash(c, p_atk_1);
    { let a0_ = rt::fmul(F::BtlAtk_GetShakeTime(c, p_atk_1), F32(0x40000000)); F::BtlChar_AddStageTimer(c, a0_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlHit_ApplyHit", &PROBE_N, &[], true); }
    r_
}

pub fn BtlColl_Update(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlColl_Update", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (F::BtlChar_GetCount(c))) as i32)) != 0) { break 'l1; }
            'c2: {
                { let a0_ = F::BtlChar_Get(c, l_i_1); F::BtlHit_BeginFrame(c, a0_) };
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
    if ((F::BtlHit_CheckClash(c)) != 0) {
        break 'f;
    }
    if ((((!((F::BtlHit_CheckRush(c)) != 0)) as i32)) != 0) {
        F::BtlHit_CheckProximityAll(c);
        {
            l_i_1 = 0i32;
            'l3: loop {
                if !(((((l_i_1) < (F::BtlChar_GetCount(c))) as i32)) != 0) { break 'l3; }
                'c4: {
                    l_chr_2 = F::BtlChar_Get(c, l_i_1);
                    F::BtlColl_UpdateFrozen(c, l_chr_2);
                    F::BtlHit_TestHit(c, l_chr_2);
                }
                { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
            }
        }
        F::BtlHit_ResolveTrades(c);
        {
            l_i_1 = 0i32;
            'l5: loop {
                if !(((((l_i_1) < (F::BtlChar_GetCount(c))) as i32)) != 0) { break 'l5; }
                'c6: {
                    { let a0_ = F::BtlChar_Get(c, l_i_1); F::BtlHit_ApplyHit(c, a0_) };
                }
                { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlColl_Update", &PROBE_N, &[], true); }
    r_
}

pub fn BtlColl_UpdateFrozen(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlColl_UpdateFrozen", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = (1i32).wrapping_neg(); let a_ = ((p_chr_1).wrapping_add(3888)).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlColl_UpdateFrozen", &PROBE_N, &[], true); }
    r_
}

pub fn BtlColl_GetGuardKind(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlColl_GetGuardKind", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    's1: {
        let sv2: i32 = F::BtlAnim_GetId(c, p_chr_1);
        match sv2 {
            239i32 | 240i32 => {
                break 'f 1i32;
            }
            241i32 | 242i32 => {
                break 'f 2i32;
            }
            243i32 | 244i32 => {
                break 'f 3i32;
            }
            236i32 | 245i32 => {
                break 'f 4i32;
            }
            247i32 => {
                break 'f 5i32;
            }
            237i32 | 238i32 | 246i32 => {
                break 'f 0i32;
            }
            _ => {
            }
        }
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlColl_GetGuardKind", &PROBE_N, &[], true); }
    r_
}

pub fn BtlColl_IsGuarding(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlColl_IsGuarding", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_kind_2: i32 = 0;
    let r_ = 'f: {
    l_kind_2 = F::BtlColl_GetGuardKind(c, p_chr_1);
    's1: {
        let sv3: i32 = l_kind_2;
        match sv3 {
            1i32..=4i32 => {
                break 'f 1i32;
            }
            _ => {
            }
        }
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlColl_IsGuarding", &PROBE_N, &[], true); }
    r_
}
