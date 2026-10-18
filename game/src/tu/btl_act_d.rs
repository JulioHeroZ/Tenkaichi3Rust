#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn BtlAct_KiBlastB2(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_KiBlastB2", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_r_3: F32 = F32(0);
    let r_ = 'f: {
        if (((((p_phase_2) == (0i32)) as i32)) != 0) {
            F::BtlAnim_Play(c, p_chr_1, 122i32, F32(0x3e199999));
            { let _ = { let a1_ = F::BtlKiBlast_GetKiCost(c, p_chr_1); F::BtlMember_SpendKi(c, p_chr_1, a1_, 0i32) }; };
            F::BtlChar_PlayVoice(c, p_chr_1, 12i32);
            { let v_ = F32(0x00000000); let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(152); c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x00000000); let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(156); c.m.w32(a_, (v_).0); };
            { let r_ = { let a0_ = F::BtlChar_GetObj(c, p_chr_1); F::BtlObjAnim_QueryEvent(c, a0_, ((4i32) as u64), 0i32, ((((3i32) as i64 as u64)) as u32)) }; let a_ = (p_chr_1).wrapping_add(3552); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
            { let v_ = 30i32; let a_ = (p_chr_1).wrapping_add(3556); c.m.w32(a_, (v_) as u32); };
        }
        if (((((p_phase_2) == (1i32)) as i32)) != 0) {
            if ((F::BtlAnim_Advance(c, p_chr_1, 0i32)) != 0) {
                if ((F::BtlChar_TestFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32))) != 0) {
                    F::BtlAct_Request(c, p_chr_1, 11i32);
                } else {
                    F::BtlAct_Request(c, p_chr_1, 17i32);
                }
            }
            F::BtlMove_Step(c, p_chr_1, 2i32, 5i32, 7i32, F32(0x00000000), F32(0x3f6d0979));
            F::BtlMove_BrakeVertical(c, p_chr_1);
            F::BtlChar_SetFlag(c, p_chr_1, ((((213i32) as i64 as u64)) as u32));
            if ((F::BtlChar_TestFlag(c, p_chr_1, ((((5i32) as i64 as u64)) as u32))) != 0) {
                { let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); F::BtlCharApi_GetNodePos(c, a0_, 17i32, fp) };
                { let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); F::BtlCharApi_CalcAimDir(c, a0_, 17i32, fp, (p_chr_1).wrapping_add(3536), F32(0x3ea0d97a), F32(0x3f490fda)) };
            } else {
                l_r_3 = F32(0x3f350481);
                { let v_ = rt::fmul({ let a0_ = { let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(148); F32(c.m.r32(a_)) }; F::Mathf_Sin(c, a0_) }, l_r_3); let a_ = (p_chr_1).wrapping_add(3536); c.m.w32(a_, (v_).0); };
                { let v_ = l_r_3; let a_ = ((p_chr_1).wrapping_add(3536)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fmul({ let a0_ = { let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(148); F32(c.m.r32(a_)) }; F::Mathf_Cos(c, a0_) }, l_r_3); let a_ = ((p_chr_1).wrapping_add(3536)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                { let v_ = F32(0x00000000); let a_ = ((p_chr_1).wrapping_add(3536)).wrapping_add(12); c.m.w32(a_, (v_).0); };
            }
        }
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_KiBlastB2", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_KiBlastChargeB3(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_KiBlastChargeB3", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_r_3: F32 = F32(0);
    let mut l_frame_4: F32 = F32(0);
    let r_ = 'f: {
        if (((((p_phase_2) == (0i32)) as i32)) != 0) {
            F::BtlAnim_Play(c, p_chr_1, 141i32, F32(0x3e199999));
            { let v_ = F::BtlAct_PlaySubAnim(c, p_chr_1, 143i32); let a_ = (p_chr_1).wrapping_add(3568); c.m.w32(a_, (v_) as u32); };
            F::BtlChar_SetFxBit(c, p_chr_1, 23i32);
            F::BtlCharSnd_PlayCommon(c, p_chr_1, 46i32);
            { let a_ = (p_chr_1).wrapping_add(3552); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
            { let v_ = 30i32; let a_ = (p_chr_1).wrapping_add(3556); c.m.w32(a_, (v_) as u32); };
        }
        if (((((p_phase_2) == (1i32)) as i32)) != 0) {
            's1: {
                let sv5: i32 = F::BtlAnim_GetId(c, p_chr_1);
                match sv5 {
                    141i32 => {
                        { let _ = { let a1_ = (F::BtlAnim_GetId(c, p_chr_1)).wrapping_add(1i32); F::BtlAnim_AdvanceThen(c, p_chr_1, a1_, F32(0x00000000), 0i32) }; };
                        F::BtlChar_SetFlag(c, p_chr_1, ((((142i32) as i64 as u64)) as u32));
                        break 's1;
                    }
                    142i32 => {
                        { let _ = { let a1_ = (F::BtlAnim_GetId(c, p_chr_1)).wrapping_add(1i32); F::BtlAnim_AdvanceThen(c, p_chr_1, a1_, F32(0x00000000), 0i32) }; };
                        F::BtlChar_SetFlag(c, p_chr_1, ((((142i32) as i64 as u64)) as u32));
                        { let v_ = F::BtlAnim_GetProgress(c, p_chr_1); let a_ = (p_chr_1).wrapping_add(3564); c.m.w32(a_, (v_).0); };
                        break 's1;
                    }
                    143i32 => {
                        if ((F::BtlAnim_Advance(c, p_chr_1, 0i32)) != 0) {
                            if ((F::BtlChar_TestFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32))) != 0) {
                                F::BtlAct_Request(c, p_chr_1, 11i32);
                            } else {
                                F::BtlAct_Request(c, p_chr_1, 17i32);
                            }
                        }
                        if ((F::BtlAnim_IsNew(c, p_chr_1)) != 0) {
                            F::BtlChar_PlayVoice(c, p_chr_1, 37i32);
                            { let _ = { let a1_ = F::BtlKiBlast_GetKiCost(c, p_chr_1); F::BtlMember_SpendKi(c, p_chr_1, a1_, 0i32) }; };
                        }
                        l_frame_4 = F::BtlAnim_GetFrame(c, p_chr_1);
                        if (((rt::flt(l_frame_4, rt::i2f({ let a0_ = F::BtlChar_GetObj(c, p_chr_1); F::BtlObjAnim_QueryEvent(c, a0_, ((4i32) as u64), 0i32, ((((0i32) as i64 as u64)) as u32)) })) as i32)) != 0) {
                            F::BtlChar_SetFlag(c, p_chr_1, ((((142i32) as i64 as u64)) as u32));
                        }
                        break 's1;
                    }
                    _ => {
                    }
                }
            }
            F::BtlMove_Step(c, p_chr_1, 2i32, 5i32, 7i32, F32(0x00000000), F32(0x3f6d0979));
            F::BtlMove_BrakeVertical(c, p_chr_1);
            F::BtlChar_SetFlag(c, p_chr_1, ((((213i32) as i64 as u64)) as u32));
            if ((F::BtlChar_TestFlag(c, p_chr_1, ((((5i32) as i64 as u64)) as u32))) != 0) {
                { let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); F::BtlCharApi_GetNodePos(c, a0_, 17i32, fp) };
                { let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); F::BtlCharApi_CalcAimDir(c, a0_, 17i32, fp, (p_chr_1).wrapping_add(3536), F32(0x3ea0d97a), F32(0x3f490fda)) };
            } else {
                l_r_3 = F32(0x3f350481);
                { let v_ = rt::fmul({ let a0_ = { let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(148); F32(c.m.r32(a_)) }; F::Mathf_Sin(c, a0_) }, l_r_3); let a_ = (p_chr_1).wrapping_add(3536); c.m.w32(a_, (v_).0); };
                { let v_ = l_r_3; let a_ = ((p_chr_1).wrapping_add(3536)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fmul({ let a0_ = { let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(148); F32(c.m.r32(a_)) }; F::Mathf_Cos(c, a0_) }, l_r_3); let a_ = ((p_chr_1).wrapping_add(3536)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                { let v_ = F32(0x00000000); let a_ = ((p_chr_1).wrapping_add(3536)).wrapping_add(12); c.m.w32(a_, (v_).0); };
            }
        }
        if (((((p_phase_2) == (2i32)) as i32)) != 0) {
            if (((((F::BtlAnim_GetId(c, p_chr_1)) == (142i32)) as i32)) != 0) {
                if ((F::BtlInput_TestAction(c, p_chr_1, 91i32, 1i32)) != 0) {
                    { let a1_ = (F::BtlAnim_GetId(c, p_chr_1)).wrapping_add(1i32); F::BtlAnim_Request(c, p_chr_1, a1_, F32(0x00000000)) };
                }
            }
        }
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_KiBlastChargeB3", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_KiVolley95(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_KiVolley95", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_count_3: u32 = 0;
    let mut l_atk_4: u32 = 0;
    let mut l_next_5: i32 = 0;
    let r_ = 'f: {
        l_count_3 = ((p_chr_1).wrapping_add(976)).wrapping_add((((2i32) as u32)).wrapping_mul(4));
        l_atk_4 = (p_chr_1).wrapping_add(3320);
        if (((((p_phase_2) == (0i32)) as i32)) != 0) {
            F::BtlAct_LatchAttack(c, p_chr_1);
            F::BtlAnim_Play(c, p_chr_1, 113i32, F32(0x3dcccccc));
            F::BtlChar_SetHeldFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32));
            F::BtlChar_PlayVoice(c, p_chr_1, 20i32);
            if (((((c.m.r32(l_atk_4) as i32)) & (128i32))) != 0) {
                F::BtlChar_SetFlag(c, p_chr_1, ((((205i32) as i64 as u64)) as u32));
            }
            if ((((((((c.m.r8((l_atk_4).wrapping_add(10)) as i8)) as i32)) >= (0i32)) as i32)) != 0) {
                { let a2_ = (((c.m.r8((l_atk_4).wrapping_add(10)) as i8)) as i32); F::ChrCam_RequestCut(c, p_chr_1, 1i32, a2_) };
            }
            if ((((((((c.m.r8((l_atk_4).wrapping_add(9)) as i8)) as i32)) < (4i32)) as i32)) != 0) {
                if ((((((((c.m.r8((l_atk_4).wrapping_add(9)) as i8)) as i32)) > (0i32)) as i32)) != 0) {
                    F::BtlChar_SetFxBit(c, p_chr_1, 13i32);
                }
            }
        }
        if (((((p_phase_2) == (1i32)) as i32)) != 0) {
            's1: {
                let sv6: i32 = F::BtlAnim_GetId(c, p_chr_1);
                match sv6 {
                    113i32 | 114i32 => {
                        if ((F::BtlAnim_IsNew(c, p_chr_1)) != 0) {
                            { let a_ = l_count_3; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                            { let _ = { let a1_ = F::BtlKiBlast_GetKiCost(c, p_chr_1); F::BtlMember_SpendKi(c, p_chr_1, a1_, 0i32) }; };
                        }
                        F::BtlAnim_SetDuration(c, p_chr_1, F32(0x3dcccccc));
                        if ((F::BtlAnim_Advance(c, p_chr_1, 0i32)) != 0) {
                            if ((((((c.m.r32(l_count_3) as i32)) >= (10i32)) as i32)) != 0) {
                                F::BtlAct_Request(c, p_chr_1, 11i32);
                            } else {
                                if ((((((((F::BtlParam_GetFlags(c, p_chr_1)) as u64)) as i32)) & (256i32))) != 0) {
                                    l_next_5 = 114i32;
                                } else {
                                    if ((F::BtlChar_TestMemberUnk70(c, p_chr_1)) != 0) {
                                        l_next_5 = 114i32;
                                    } else {
                                        if (((((F::BtlAnim_GetId(c, p_chr_1)) == (113i32)) as i32)) != 0) {
                                            l_next_5 = 114i32;
                                        } else {
                                            l_next_5 = 113i32;
                                        }
                                    }
                                }
                                F::BtlAnim_Request(c, p_chr_1, l_next_5, F32(0x00000000));
                            }
                        }
                        { let a1_ = (F::BtlAnim_GetId(c, p_chr_1)).wrapping_add(2i32); let a2_ = (F::BtlAnim_GetId(c, p_chr_1)).wrapping_add(4i32); F::BtlAct_SetPitchMotion(c, p_chr_1, a1_, a2_, 1i32) };
                        break 's1;
                    }
                    _ => {
                    }
                }
            }
            F::BtlMove_Step(c, p_chr_1, 2i32, 5i32, 7i32, F32(0x00000000), F32(0x3f6d0979));
            F::BtlMove_ApplyGravity(c, p_chr_1);
            if ((F::BtlChar_TestFlag(c, p_chr_1, ((((5i32) as i64 as u64)) as u32))) != 0) {
                { let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); F::BtlCharApi_GetNodePos(c, a0_, 17i32, fp) };
                { let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); F::BtlCharApi_CalcAimDir(c, a0_, 17i32, fp, (p_chr_1).wrapping_add(3536), F32(0xbf20d97a), F32(0x3f20d97a)) };
            } else {
                { let v_ = { let a0_ = { let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(148); F32(c.m.r32(a_)) }; F::Mathf_Sin(c, a0_) }; let a_ = (p_chr_1).wrapping_add(3536); c.m.w32(a_, (v_).0); };
                { let v_ = F32(0x00000000); let a_ = ((p_chr_1).wrapping_add(3536)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                { let v_ = { let a0_ = { let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(148); F32(c.m.r32(a_)) }; F::Mathf_Cos(c, a0_) }; let a_ = ((p_chr_1).wrapping_add(3536)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                { let v_ = F32(0x00000000); let a_ = ((p_chr_1).wrapping_add(3536)).wrapping_add(12); c.m.w32(a_, (v_).0); };
            }
            if (((((c.m.r32(l_atk_4) as i32)) & (64i32))) != 0) {
                if ((F::BtlChar_IsFlagRaised(c, p_chr_1, ((((91i32) as i64 as u64)) as u32))) != 0) {
                    if ((((((((c.m.r8((l_atk_4).wrapping_add(10)) as i8)) as i32)) >= (0i32)) as i32)) != 0) {
                        F::ChrCam_EndCut(c, p_chr_1);
                    }
                }
            }
        }
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_KiVolley95", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_PlayLandFx(c: &mut Ctx, mut p_chr_1: u32, mut p_hard_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_PlayLandFx", &PROBE_N, &[(p_chr_1) as u32, (p_hard_2) as u32], false); }
    let mut l_snd_3: i32 = 0;
    let r_ = 'f: {
    if (((((F::BtlParam_GetUnk2(c, p_chr_1)) == (4i32)) as i32)) != 0) {
        { let a0_ = F::BtlChar_GetPos(c, p_chr_1); F::BtlCharApi_RumbleNear(c, a0_, F32(0x42c80000), F32(0x447a0000), F32(0x3f4ccccc), F32(0x3dcccccc)) };
        { let a0_ = F::BtlChar_GetPos(c, p_chr_1); F::BtlCharApi_ShakeCamsNear(c, a0_, F32(0x42c80000), F32(0x447a0000), F32(0x40400000), F32(0x3e999999)) };
        F::BtlCharSnd_PlayCommon(c, p_chr_1, 37i32);
        F::BtlChar_SetFxBit(c, p_chr_1, 53i32);
        break 'f;
    }
    if ((((((((F::BtlParam_GetFlags(c, p_chr_1)) as u64)) as i32)) & (32768i32))) != 0) {
        l_snd_3 = 11i32;
    } else {
        l_snd_3 = (if (((((p_hard_2) != (0i32)) as i32)) != 0) { 48i32 } else { 51i32 });
    }
    F::BtlCharSnd_PlayCommon(c, p_chr_1, l_snd_3);
    F::BtlChar_SetFxBit(c, p_chr_1, 50i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_PlayLandFx", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_GetIdleMotion(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_GetIdleMotion", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_motion_2: i32 = 0;
    let r_ = 'f: {
    if ((F::BtlChar_TestFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32))) != 0) {
        if ((((!((F::BtlParam_CanFly(c, p_chr_1)) != 0)) as i32)) != 0) {
            if ((((!((F::BtlChar_TestFlag(c, p_chr_1, ((((17i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) {
                l_motion_2 = 395i32;
            } else {
                l_motion_2 = (if ((((({ let a_ = (F::BtlMember_GetActiveGauge(c, p_chr_1)).wrapping_add(40); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) { 39i32 } else { 38i32 });
            }
        } else {
            l_motion_2 = (if ((((({ let a_ = (F::BtlMember_GetActiveGauge(c, p_chr_1)).wrapping_add(40); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) { 39i32 } else { 38i32 });
        }
    } else {
        l_motion_2 = ((({ let a_ = (F::BtlMember_GetActiveGauge(c, p_chr_1)).wrapping_add(40); (c.m.r32(a_) as i32) }) != (0i32)) as i32);
    }
    break 'f l_motion_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_GetIdleMotion", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_NeutralHandler(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_NeutralHandler", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let mut l_timer_3: u32 = 0;
    let mut l_look_4: i32 = 0;
    let mut l_motion_5: i32 = 0;
    let mut l_speed_6: F32 = F32(0);
    let r_ = 'f: {
    l_timer_3 = ((p_chr_1).wrapping_add(976)).wrapping_add((((2i32) as u32)).wrapping_mul(4));
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        if (((rt::flt(F::BtlAct_GetHeight(c, p_chr_1), F32(0x3eed0979)) as i32)) != 0) {
            F::BtlChar_ClearFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32));
        }
        { let a1_ = F::BtlAct_GetIdleMotion(c, p_chr_1); F::BtlAnim_Play(c, p_chr_1, a1_, F32(0x3e199999)) };
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        l_look_4 = 1i32;
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32))) != 0) {
            if ((F::BtlChar_TestFlag(c, p_chr_1, ((((15i32) as i64 as u64)) as u32))) != 0) {
                F::BtlChar_ClearFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32));
            }
        } else {
            if ((((!((F::BtlChar_TestFlag(c, p_chr_1, ((((15i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) {
                if ((F::BtlChar_TestFlag(c, p_chr_1, ((((17i32) as i64 as u64)) as u32))) != 0) {
                    F::BtlChar_SetHeldFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32));
                }
            }
        }
        F::BtlAnim_AdvanceLoop(c, p_chr_1, 0i32);
        if ((F::BtlMove_IsBlockedByOpponent(c, p_chr_1)) != 0) {
            { let v_ = F32(0x00000000); let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(152); c.m.w32(a_, (v_).0); };
        }
        l_speed_6 = F32(0x3f6d0979);
        F::BtlMove_Step(c, p_chr_1, 6i32, 6i32, 7i32, F32(0x00000000), l_speed_6);
        if (((((F::BtlAnim_GetId(c, p_chr_1)) == (395i32)) as i32)) != 0) {
            F::BtlMove_MoveVertical(c, p_chr_1, l_speed_6, l_speed_6);
        } else {
            F::BtlMove_ApplyGravity(c, p_chr_1);
        }
        F::BtlChar_SetFlag(c, p_chr_1, ((((201i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((28i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((37i32) as i64 as u64)) as u32));
        if ((((({ let a_ = (F::BtlMember_GetActiveGauge(c, p_chr_1)).wrapping_add(40); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
            F::BtlChar_SetFlag(c, p_chr_1, ((((150i32) as i64 as u64)) as u32));
            if ((((((((F::BtlParam_GetFlags(c, p_chr_1)) as u64)) as i32)) & (65536i32))) != 0) {
                F::BtlChar_SetFlag(c, p_chr_1, ((((311i32) as i64 as u64)) as u32));
                l_look_4 = 0i32;
            }
        }
        l_motion_5 = F::BtlAct_GetIdleMotion(c, p_chr_1);
        if (((((F::BtlAnim_GetId(c, p_chr_1)) != (l_motion_5)) as i32)) != 0) {
            F::BtlAnim_Request(c, p_chr_1, l_motion_5, F32(0x3e199999));
        }
        if ((l_look_4) != 0) {
            F::BtlChar_SetLookEnabled(c, p_chr_1, 1i32);
        }
    }
    if (((((p_phase_2) == (2i32)) as i32)) != 0) {
        if (((((F::Battle_GetMode(c)) != (1i32)) as i32)) != 0) {
            if ((F::BtlChar_TestFlag(c, p_chr_1, ((((3i32) as i64 as u64)) as u32))) != 0) {
                if ((((({ let a_ = (F::BtlMember_GetActive(c, p_chr_1)).wrapping_add(160); (c.m.r32(a_) as i32) }) <= (0i32)) as i32)) != 0) {
                    if ((((!((F::BtlChar_TestFlag(c, p_chr_1, ((((177i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) {
                        if ((((({ let a_ = l_timer_3; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); n_ }) > (90i32)) as i32)) != 0) {
                            F::BtlAct_Request(c, p_chr_1, 67i32);
                        }
                    }
                }
            }
        }
        { let _ = F::BtlDecide_Main(c, p_chr_1, ((((((586989547i32) as u32)) as u64)) as i32)); };
        { let _ = F::BtlDecide_Attack(c, p_chr_1, ((((((134219087i32) as u32)) as u64)) as i32)); };
        { let _ = F::BtlDecide_Common(c, p_chr_1, 15i32); };
        { let a1_ = F::BtlAct_GetQueued(c, p_chr_1); F::BtlAct_Request(c, p_chr_1, a1_) };
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_NeutralHandler", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_IdleOnceHandler(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_IdleOnceHandler", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let r_ = 'f: {
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        { let a1_ = F::BtlAct_GetIdleMotion(c, p_chr_1); F::BtlAnim_Play(c, p_chr_1, a1_, F32(0x3e199999)) };
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        F::BtlAnim_SetDuration(c, p_chr_1, F32(0x3f800000));
        if ((F::BtlAnim_Advance(c, p_chr_1, 0i32)) != 0) {
            F::BtlAct_Request(c, p_chr_1, 11i32);
        }
        F::BtlMove_Step(c, p_chr_1, 6i32, 6i32, 7i32, F32(0x00000000), F32(0x3f6d0979));
        F::BtlMove_ApplyGravity(c, p_chr_1);
        F::BtlChar_SetFlag(c, p_chr_1, ((((28i32) as i64 as u64)) as u32));
        if ((((({ let a_ = (F::BtlMember_GetActiveGauge(c, p_chr_1)).wrapping_add(40); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
            F::BtlChar_SetFlag(c, p_chr_1, ((((150i32) as i64 as u64)) as u32));
            if ((((((((F::BtlParam_GetFlags(c, p_chr_1)) as u64)) as i32)) & (65536i32))) != 0) {
                F::BtlChar_SetFlag(c, p_chr_1, ((((311i32) as i64 as u64)) as u32));
            }
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_IdleOnceHandler", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_MoveHandler(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_MoveHandler", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let mut l_motion_3: i32 = 0;
    let mut l_subNeg_4: i32 = 0;
    let mut l_subPos_5: i32 = 0;
    let mut l_pitchMode_6: i32 = 0;
    let mut l_speed_7: F32 = F32(0);
    let mut l_side_8: F32 = F32(0);
    let r_ = 'f: {
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        l_motion_3 = 2i32;
        if ((((!((F::BtlParam_CanFly(c, p_chr_1)) != 0)) as i32)) != 0) {
            if ((F::BtlChar_TestFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32))) != 0) {
                if ((((!((F::BtlChar_TestFlag(c, p_chr_1, ((((17i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) {
                    l_motion_3 = 395i32;
                }
            }
        }
        F::BtlAnim_Play(c, p_chr_1, l_motion_3, F32(0x3e199999));
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((17i32) as i64 as u64)) as u32))) != 0) {
            F::BtlCharSnd_PlayCommon(c, p_chr_1, 56i32);
        }
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        l_speed_7 = { let a1_ = F::BtlChar_TestFlag(c, p_chr_1, ((((17i32) as i64 as u64)) as u32)); F::BtlMoveParam_GetSpeed(c, p_chr_1, a1_) };
        l_subNeg_4 = 0i32;
        l_subPos_5 = 0i32;
        's1: {
            let sv9: i32 = F::BtlAnim_GetId(c, p_chr_1);
            match sv9 {
                2i32 => {
                    { let _ = F::BtlAnim_AdvanceThen(c, p_chr_1, 3i32, F32(0x00000000), 0i32); };
                    l_subNeg_4 = 4i32;
                    l_subPos_5 = 6i32;
                    break 's1;
                }
                3i32 => {
                    F::BtlAnim_AdvanceLoop(c, p_chr_1, 0i32);
                    l_subNeg_4 = 5i32;
                    l_subPos_5 = 7i32;
                    break 's1;
                }
                395i32 => {
                    F::BtlAnim_AdvanceLoop(c, p_chr_1, 0i32);
                    l_subNeg_4 = (1i32).wrapping_neg();
                    l_subPos_5 = (1i32).wrapping_neg();
                    break 's1;
                }
                _ => {
                }
            }
        }
        if (((((((((l_subNeg_4) >= (0i32)) as i32)) != 0) && (((((l_subPos_5) >= (0i32)) as i32)) != 0)) as i32)) != 0) {
            l_side_8 = { let a0_ = F::BtlAct_GetRotYRelCam(c, p_chr_1); F::Mathf_Sin(c, a0_) };
            if (((rt::flt(F32(0x00000000), l_side_8) as i32)) != 0) {
                F::BtlAnim_SetUnkC8C(c, p_chr_1, l_side_8);
                F::BtlAnim_PlaySub(c, p_chr_1, l_subPos_5);
            } else {
                F::BtlAnim_SetUnkC8C(c, p_chr_1, rt::fneg(l_side_8));
                F::BtlAnim_PlaySub(c, p_chr_1, l_subNeg_4);
            }
        }
        l_pitchMode_6 = 5i32;
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32))) != 0) {
            if ((F::BtlChar_TestFlag(c, p_chr_1, ((((5i32) as i64 as u64)) as u32))) != 0) {
                l_pitchMode_6 = 2i32;
            }
        }
        F::BtlMove_Step(c, p_chr_1, 0i32, l_pitchMode_6, 3i32, l_speed_7, F32(0x3f6d0979));
        F::BtlMove_ApplyGravity(c, p_chr_1);
        F::BtlChar_SetFlag(c, p_chr_1, ((((201i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((28i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((37i32) as i64 as u64)) as u32));
        F::BtlMove_RequestOrbit(c, p_chr_1, F32(0x41a00000), F32(0x42480000));
        F::BtlChar_SetLookEnabled(c, p_chr_1, 1i32);
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32))) != 0) {
            if ((F::BtlChar_TestFlag(c, p_chr_1, ((((15i32) as i64 as u64)) as u32))) != 0) {
                F::BtlChar_ClearFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32));
            }
        }
        if ((((({ let a_ = (F::BtlMember_GetActiveGauge(c, p_chr_1)).wrapping_add(40); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
            F::BtlChar_SetFlag(c, p_chr_1, ((((150i32) as i64 as u64)) as u32));
        }
        if (((((F::BtlAnim_GetId(c, p_chr_1)) == (395i32)) as i32)) != 0) {
            if ((F::BtlChar_TestFlag(c, p_chr_1, ((((17i32) as i64 as u64)) as u32))) != 0) {
                F::BtlAnim_Request(c, p_chr_1, 3i32, F32(0x3e199999));
            }
        } else {
            if ((((!((F::BtlParam_CanFly(c, p_chr_1)) != 0)) as i32)) != 0) {
                if ((((!((F::BtlChar_TestFlag(c, p_chr_1, ((((17i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) {
                    if ((((!((F::BtlChar_TestFlag(c, p_chr_1, ((((15i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) {
                        F::BtlAnim_Request(c, p_chr_1, 395i32, F32(0x3e199999));
                    }
                }
            }
        }
    }
    if (((((p_phase_2) == (2i32)) as i32)) != 0) {
        if ((F::BtlInput_TestAction(c, p_chr_1, 1i32, 0i32)) != 0) {
            F::BtlAct_Request(c, p_chr_1, 11i32);
        }
        { let _ = F::BtlDecide_Main(c, p_chr_1, ((((((580394987i32) as u32)) as u64)) as i32)); };
        { let _ = F::BtlDecide_Attack(c, p_chr_1, ((((((134218767i32) as u32)) as u64)) as i32)); };
        { let _ = F::BtlDecide_Common(c, p_chr_1, 15i32); };
        { let a1_ = F::BtlAct_GetQueued(c, p_chr_1); F::BtlAct_Request(c, p_chr_1, a1_) };
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_MoveHandler", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_CloseMoveHandler(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_CloseMoveHandler", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let mut l_lean_3: u32 = 0;
    let mut l_motion_4: i32 = 0;
    let mut l_dir_5: F32 = F32(0);
    let mut l_speed_6: F32 = F32(0);
    let r_ = 'f: {
    l_lean_3 = ((p_chr_1).wrapping_add(976)).wrapping_add((((6i32) as u32)).wrapping_mul(4));
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        l_motion_4 = 8i32;
        if ((((!(((rt::flt(F::BtlInput_GetStickY(c, p_chr_1), F32(0x00000000)) as i32)) != 0)) as i32)) != 0) {
            l_motion_4 = 9i32;
        }
        if ((((!((F::BtlParam_CanFly(c, p_chr_1)) != 0)) as i32)) != 0) {
            if ((F::BtlChar_TestFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32))) != 0) {
                if ((((!((F::BtlChar_TestFlag(c, p_chr_1, ((((17i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) {
                    l_motion_4 = 395i32;
                }
            }
        }
        F::BtlAnim_Play(c, p_chr_1, l_motion_4, F32(0x3e199999));
        F::BtlMove_SetDirection(c, p_chr_1, 0i32);
        { let v_ = { let a0_ = F::BtlAct_GetVelDirRelCam(c, p_chr_1); F::Mathf_Sin(c, a0_) }; let a_ = l_lean_3; c.m.w32(a_, (v_).0); };
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32))) != 0) {
            if ((F::BtlChar_TestFlag(c, p_chr_1, ((((15i32) as i64 as u64)) as u32))) != 0) {
                F::BtlChar_ClearFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32));
            }
        }
        if (((((F::BtlAnim_GetId(c, p_chr_1)) != (395i32)) as i32)) != 0) {
            l_dir_5 = F::BtlAct_GetVelDirRelCam(c, p_chr_1);
            if (((((((rt::flt(F32(0xbfc90fda), l_dir_5) as i32)) != 0) && (((rt::flt(l_dir_5, F32(0x3fc90fda)) as i32)) != 0)) as i32)) != 0) {
                if (((((F::BtlAnim_GetId(c, p_chr_1)) == (9i32)) as i32)) != 0) {
                    F::BtlAnim_RequestKeep(c, p_chr_1, 8i32, F32(0x3e199999));
                }
            } else {
                if (((((F::BtlAnim_GetId(c, p_chr_1)) == (8i32)) as i32)) != 0) {
                    F::BtlAnim_RequestKeep(c, p_chr_1, 9i32, F32(0x3e199999));
                }
            }
            { let v_ = { let a0_ = F32(c.m.r32(l_lean_3)); let a1_ = F::Mathf_Sin(c, l_dir_5); F::BtlUtil_ApproachF(c, a0_, a1_, F32(0x3e888888)) }; let a_ = l_lean_3; c.m.w32(a_, (v_).0); };
            if (((rt::flt(F32(0x00000000), F32(c.m.r32(l_lean_3))) as i32)) != 0) {
                { let a1_ = F32(c.m.r32(l_lean_3)); F::BtlAnim_SetUnkC8C(c, p_chr_1, a1_) };
                F::BtlAnim_PlaySub(c, p_chr_1, 11i32);
            } else {
                { let a1_ = rt::fneg(F32(c.m.r32(l_lean_3))); F::BtlAnim_SetUnkC8C(c, p_chr_1, a1_) };
                F::BtlAnim_PlaySub(c, p_chr_1, 10i32);
            }
        }
        F::BtlAnim_AdvanceLoop(c, p_chr_1, 0i32);
        l_speed_6 = { let a1_ = { let a1_ = (F::BtlChar_TestFlag(c, p_chr_1, ((((17i32) as i64 as u64)) as u32))).wrapping_add(2i32); F::BtlMoveParam_GetSpeed(c, p_chr_1, a1_) }; F::BtlAct_ScaleSpeedByApproach(c, p_chr_1, a1_, F32(0x41f00000)) };
        { let a3_ = (((F::BtlChar_TestFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32))) != (0i32)) as i32); F::BtlMove_Step(c, p_chr_1, 2i32, 2i32, a3_, l_speed_6, F32(0x3f6d0979)) };
        F::BtlMove_ApplyGravity(c, p_chr_1);
        F::BtlChar_SetFlag(c, p_chr_1, ((((28i32) as i64 as u64)) as u32));
        F::BtlChar_SetLookEnabled(c, p_chr_1, 1i32);
        F::BtlMove_RequestOrbit(c, p_chr_1, F32(0x41200000), F32(0x41a00000));
        if ((((({ let a_ = (F::BtlMember_GetActiveGauge(c, p_chr_1)).wrapping_add(40); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
            F::BtlChar_SetFlag(c, p_chr_1, ((((150i32) as i64 as u64)) as u32));
        }
        if (((((F::BtlAnim_GetId(c, p_chr_1)) == (395i32)) as i32)) != 0) {
            if ((F::BtlChar_TestFlag(c, p_chr_1, ((((17i32) as i64 as u64)) as u32))) != 0) {
                { let a1_ = (if (((rt::flt(F::BtlInput_GetStickY(c, p_chr_1), F32(0x00000000)) as i32)) != 0) { 8i32 } else { 9i32 }); F::BtlAnim_Request(c, p_chr_1, a1_, F32(0x3e199999)) };
            }
        } else {
            if ((((!((F::BtlParam_CanFly(c, p_chr_1)) != 0)) as i32)) != 0) {
                if ((((!((F::BtlChar_TestFlag(c, p_chr_1, ((((17i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) {
                    if ((((!((F::BtlChar_TestFlag(c, p_chr_1, ((((15i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) {
                        F::BtlAnim_Request(c, p_chr_1, 395i32, F32(0x3e199999));
                    }
                }
            }
        }
    }
    if (((((p_phase_2) == (2i32)) as i32)) != 0) {
        if ((F::BtlInput_TestAction(c, p_chr_1, 2i32, 1i32)) != 0) {
            F::BtlAct_Request(c, p_chr_1, 11i32);
        }
        { let _ = F::BtlDecide_Main(c, p_chr_1, ((((((580698083i32) as u32)) as u64)) as i32)); };
        { let _ = F::BtlDecide_Attack(c, p_chr_1, ((((((134219087i32) as u32)) as u64)) as i32)); };
        { let _ = F::BtlDecide_Common(c, p_chr_1, 15i32); };
        { let a1_ = F::BtlAct_GetQueued(c, p_chr_1); F::BtlAct_Request(c, p_chr_1, a1_) };
    }
    if (((((p_phase_2) == (3i32)) as i32)) != 0) {
        if (((((F::BtlAct_GetRequested(c, p_chr_1)) == (17i32)) as i32)) != 0) {
            break 'f { F::BtlMove_TurnYaw(c, p_chr_1, 0i32, F32(0x40490fda)); 0 };
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_CloseMoveHandler", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_DashMoveHandler(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_DashMoveHandler", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let mut l_subNeg_3: i32 = 0;
    let mut l_subPos_4: i32 = 0;
    let mut l_moving_5: i32 = 0;
    let mut l_pitchMode_6: i32 = 0;
    let mut l_resume_7: i32 = 0;
    let mut l_queued_8: i32 = 0;
    let mut l_ratio_9: F32 = F32(0);
    let mut l_speed_10: F32 = F32(0);
    let mut l_side_11: F32 = F32(0);
    let mut l_toward_12: F32 = F32(0);
    let r_ = 'f: {
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        l_resume_7 = 0i32;
        's1: {
            let sv13: i32 = F::BtlAct_GetPrev(c, p_chr_1);
            match sv13 {
                62i32 | 176i32 | 177i32 => {
                    l_resume_7 = 1i32;
                    break 's1;
                }
                _ => {
                }
            }
        }
        if ((l_resume_7) != 0) {
            F::BtlAnim_Play(c, p_chr_1, 13i32, F32(0x3e800000));
        } else {
            F::BtlAnim_Play(c, p_chr_1, 12i32, F32(0x3e199999));
            F::BtlChar_SetFlag(c, p_chr_1, ((((208i32) as i64 as u64)) as u32));
            F::BtlMove_TurnYaw(c, p_chr_1, 0i32, F32(0x40490fda));
            if ((F::BtlChar_TestFlag(c, p_chr_1, ((((18i32) as i64 as u64)) as u32))) != 0) {
                F::BtlCharSnd_PlayCommon(c, p_chr_1, 35i32);
            } else {
                if ((F::BtlChar_TestFlag(c, p_chr_1, ((((17i32) as i64 as u64)) as u32))) != 0) {
                    F::BtlCharSnd_PlayCommon(c, p_chr_1, 57i32);
                } else {
                    F::BtlCharSnd_PlayCommon(c, p_chr_1, 30i32);
                }
            }
        }
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        l_subNeg_3 = 0i32;
        l_subPos_4 = 0i32;
        l_ratio_9 = F32(0x00000000);
        l_moving_5 = 1i32;
        's2: {
            let sv14: i32 = F::BtlAnim_GetId(c, p_chr_1);
            match sv14 {
                12i32 => {
                    { let _ = F::BtlAnim_AdvanceThen(c, p_chr_1, 13i32, F32(0x3e199999), 0i32); };
                    l_subNeg_3 = 15i32;
                    l_ratio_9 = F::BtlAnim_GetProgress(c, p_chr_1);
                    l_subPos_4 = 18i32;
                    break 's2;
                }
                13i32 => {
                    F::BtlAnim_AdvanceLoop(c, p_chr_1, 0i32);
                    l_subNeg_3 = 16i32;
                    l_ratio_9 = F32(0x3f800000);
                    l_subPos_4 = 19i32;
                    break 's2;
                }
                14i32 => {
                    if ((F::BtlAnim_Advance(c, p_chr_1, 0i32)) != 0) {
                        { let v_ = 1i32; let a_ = ((p_chr_1).wrapping_add(976)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    }
                    l_subNeg_3 = 17i32;
                    l_ratio_9 = rt::fsub(F32(0x3f800000), F::BtlAnim_GetProgress(c, p_chr_1));
                    l_subPos_4 = 20i32;
                    l_moving_5 = 0i32;
                    break 's2;
                }
                21i32 => {
                    if ((F::BtlAnim_IsNew(c, p_chr_1)) != 0) {
                        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((15i32) as i64 as u64)) as u32))) != 0) {
                            F::BtlChar_SetFxBit(c, p_chr_1, 49i32);
                        }
                    }
                    if ((F::BtlAnim_Advance(c, p_chr_1, 0i32)) != 0) {
                        { let v_ = 1i32; let a_ = ((p_chr_1).wrapping_add(976)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    }
                    l_subNeg_3 = 22i32;
                    l_ratio_9 = rt::fsub(F32(0x3f800000), F::BtlAnim_GetProgress(c, p_chr_1));
                    l_subPos_4 = 23i32;
                    l_moving_5 = 0i32;
                    { let r_ = (!(1i32)); let a_ = ((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                    break 's2;
                }
                _ => {
                }
            }
        }
        if ((((!((((((((F::BtlParam_GetFlags(c, p_chr_1)) as u64)) as i32)) & (131072i32))) != 0)) as i32)) != 0) {
            l_side_11 = { let a0_ = F::BtlAct_GetRotYRelCam(c, p_chr_1); F::Mathf_Sin(c, a0_) };
            if (((rt::flt(F32(0x00000000), l_side_11) as i32)) != 0) {
                F::BtlAnim_SetUnkC8C(c, p_chr_1, l_side_11);
                F::BtlAnim_PlaySub(c, p_chr_1, l_subPos_4);
            } else {
                F::BtlAnim_SetUnkC8C(c, p_chr_1, rt::fneg(l_side_11));
                F::BtlAnim_PlaySub(c, p_chr_1, l_subNeg_3);
            }
        }
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((5i32) as i64 as u64)) as u32))) != 0) {
            if ((l_moving_5) != 0) {
                if ((((((((!((F::BtlChar_TestFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) || ((((!((F::BtlInput_IsHeld(c, p_chr_1, ((240i32) as u32))) != 0)) as i32)) != 0)) as i32)) != 0) {
                    F::BtlChar_SetFlag(c, p_chr_1, ((((26i32) as i64 as u64)) as u32));
                }
            }
        }
        l_pitchMode_6 = 5i32;
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32))) != 0) {
            if ((F::BtlChar_TestFlag(c, p_chr_1, ((((5i32) as i64 as u64)) as u32))) != 0) {
                l_pitchMode_6 = 2i32;
            }
        }
        if ((l_moving_5) != 0) {
            l_speed_10 = { let a1_ = (F::BtlChar_TestFlag(c, p_chr_1, ((((17i32) as i64 as u64)) as u32))).wrapping_add(4i32); F::BtlMoveParam_GetSpeed(c, p_chr_1, a1_) };
            { let a2_ = F::BtlMoveParam_GetTurnRate(c, p_chr_1, 0i32); F::BtlMove_TurnYaw(c, p_chr_1, 0i32, a2_) };
            F::BtlMove_TurnPitch(c, p_chr_1, l_pitchMode_6, F32(0x3e20d97a));
            F::BtlMove_TurnModelYaw(c, p_chr_1, F32(0x3f20d97a), F32(0x3e999999));
            F::BtlMove_SetDirection(c, p_chr_1, 3i32);
            F::BtlMove_Advance(c, p_chr_1, l_speed_10, F32(0x461c4000));
            F::BtlMove_ApplyGravity(c, p_chr_1);
            F::BtlChar_SetFxBit(c, p_chr_1, 11i32);
            if ((F::BtlChar_TestFlag(c, p_chr_1, ((((15i32) as i64 as u64)) as u32))) != 0) {
                F::BtlChar_SetFxBit(c, p_chr_1, 48i32);
            }
        } else {
            F::BtlMove_Step(c, p_chr_1, 6i32, 5i32, 3i32, F32(0x00000000), F32(0x3f6d0979));
            F::BtlMove_ApplyGravity(c, p_chr_1);
        }
        l_toward_12 = { let a0_ = F::BtlOpp_GetYawFromFacing(c, p_chr_1); F::Mathf_Cos(c, a0_) };
        if (((rt::flt(F32(0x00000000), l_toward_12) as i32)) != 0) {
            { let a1_ = rt::fmul(rt::fmul({ let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(144); F32(c.m.r32(a_)) }, l_toward_12), l_ratio_9); F::BtlMove_SetLeanX(c, p_chr_1, a1_) };
        }
        F::BtlChar_SetFlag(c, p_chr_1, ((((202i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((37i32) as i64 as u64)) as u32));
        F::BtlMove_RequestOrbit(c, p_chr_1, F32(0x42480000), F32(0x42c80000));
        F::BtlChar_SetFxBit(c, p_chr_1, 58i32);
        if ((((({ let a_ = (F::BtlMember_GetActiveGauge(c, p_chr_1)).wrapping_add(40); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
            F::BtlChar_SetFlag(c, p_chr_1, ((((150i32) as i64 as u64)) as u32));
        }
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((17i32) as i64 as u64)) as u32))) != 0) {
            F::BtlChar_SetHeldFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32));
        }
        if ((F::BtlChar_IsStage4Or27(c)) != 0) {
            if ((F::BtlChar_TestFlag(c, p_chr_1, ((((23i32) as i64 as u64)) as u32))) != 0) {
                F::BtlChar_SetHeldFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32));
            }
        }
    }
    if (((((p_phase_2) == (2i32)) as i32)) != 0) {
        if ((((((c.m.r32(((p_chr_1).wrapping_add(976)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) != (0i32)) as i32)) != 0) {
            F::BtlAct_Request(c, p_chr_1, 11i32);
            if ((((!((F::BtlAct_HasQueued(c, p_chr_1)) != 0)) as i32)) != 0) {
                { let _ = F::BtlDecide_Main(c, p_chr_1, ((((((1i32) as u32)) as u64)) as i32)); };
                { let _ = F::BtlDecide_Common(c, p_chr_1, 1i32); };
            }
            { let a1_ = F::BtlAct_GetQueued(c, p_chr_1); F::BtlAct_Request(c, p_chr_1, a1_) };
        }
        's3: {
            let sv15: i32 = F::BtlAnim_GetId(c, p_chr_1);
            match sv15 {
                12i32 => {
                    if ((F::BtlInput_TestAction(c, p_chr_1, 4i32, 1i32)) != 0) {
                        { let r_ = 1i32; let a_ = ((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    }
                    break 's3;
                }
                13i32 => {
                    if ((F::BtlInput_TestAction(c, p_chr_1, 6i32, 1i32)) != 0) {
                        F::BtlAnim_Request(c, p_chr_1, 14i32, F32(0x3e199999));
                    }
                    if ((F::BtlMove_IsBlockedByOpponent(c, p_chr_1)) != 0) {
                        F::BtlAnim_Request(c, p_chr_1, 14i32, F32(0x3e199999));
                        { let v_ = F32(0x00000000); let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(152); c.m.w32(a_, (v_).0); };
                    }
                    if ((((((F::BtlInput_TestAction(c, p_chr_1, 4i32, 1i32)) != 0) || (((((c.m.r32(((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) & (1i32))) != 0)) as i32)) != 0) {
                        F::BtlAnim_Request(c, p_chr_1, 21i32, F32(0x3e199999));
                    }
                    break 's3;
                }
                14i32 => {
                    if ((F::BtlMove_IsBlockedByOpponent(c, p_chr_1)) != 0) {
                        { let v_ = F32(0x00000000); let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(152); c.m.w32(a_, (v_).0); };
                    }
                    { let _ = F::BtlDecide_Main(c, p_chr_1, ((((((768i32) as u32)) as u64)) as i32)); };
                    break 's3;
                }
                21i32 => {
                    if ((F::BtlAnim_PassedRatio(c, p_chr_1, F32(0x3f000000))) != 0) {
                        if ((F::BtlInput_TestAction(c, p_chr_1, 5i32, 1i32)) != 0) {
                            F::BtlAct_Request(c, p_chr_1, 15i32);
                            { let v_ = { let a0_ = rt::fadd({ let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(172); F32(c.m.r32(a_)) }, F32(0x3dcccccc)); F::BtlUtil_MinF(c, a0_, F32(0x3fc00000)) }; let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(172); c.m.w32(a_, (v_).0); };
                        }
                    }
                    break 's3;
                }
                _ => {
                }
            }
        }
        l_queued_8 = F::BtlDecide_Main(c, p_chr_1, ((((((536878180i32) as u32)) as u64)) as i32));
        if (((((F::BtlAnim_GetId(c, p_chr_1)) != (14i32)) as i32)) != 0) {
            if ((F::BtlInput_TestAction(c, p_chr_1, 64i32, 1i32)) != 0) {
                F::BtlAct_Request(c, p_chr_1, 88i32);
            }
            { let r_ = F::BtlDecide_Attack(c, p_chr_1, ((((((49152i32) as u32)) as u64)) as i32)); let n_ = ((l_queued_8) | (r_)); l_queued_8 = n_; };
            if ((F::BtlInput_TestAction(c, p_chr_1, 44i32, 1i32)) != 0) {
                F::BtlAct_Request(c, p_chr_1, 62i32);
            }
        }
        { let r_ = F::BtlDecide_Common(c, p_chr_1, 14i32); let n_ = ((l_queued_8) | (r_)); l_queued_8 = n_; };
        if ((l_queued_8) != 0) {
            { let a1_ = F::BtlAct_GetQueued(c, p_chr_1); F::BtlAct_Request(c, p_chr_1, a1_) };
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_DashMoveHandler", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_JumpStartHandler(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_JumpStartHandler", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let mut l_height_3: F32 = F32(0);
    let mut l_speed_4: F32 = F32(0);
    let mut l_pose_5: u32 = 0;
    let r_ = 'f: {
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        F::BtlAnim_Play(c, p_chr_1, 31i32, F32(0x3dcccccc));
        F::BtlChar_ClearFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32));
        F::BtlCharSnd_PlayCommon(c, p_chr_1, 49i32);
        { let v_ = 0i32; let a_ = (p_chr_1).wrapping_add(4080); c.m.w32(a_, (v_) as u32); };
        if (((((F::BtlAct_GetCurrent(c, p_chr_1)) == (18i32)) as i32)) != 0) {
            { let r_ = 1i32; let a_ = ((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        }
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        if ((F::BtlAnim_Advance(c, p_chr_1, 1i32)) != 0) {
            { let a1_ = (if (((((c.m.r32(((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) & (1i32))) != 0) { 19i32 } else { 17i32 }); F::BtlAct_Request(c, p_chr_1, a1_) };
        }
        if ((F::BtlAnim_PassedRatio(c, p_chr_1, F32(0x3e999999))) != 0) {
            l_height_3 = { let a1_ = (((c.m.r32(((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) & (1i32)); F::BtlMoveParam_GetHeight(c, p_chr_1, a1_) };
            l_pose_5 = F::BtlChar_GetPos(c, p_chr_1);
            { let v_ = F::BtlMove_CalcJumpSpeed(c, l_height_3); let a_ = (l_pose_5).wrapping_add(156); c.m.w32(a_, (v_).0); };
        }
        l_speed_4 = { let a1_ = (if (((((c.m.r32(((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) & (1i32))) != 0) { 4i32 } else { 13i32 }); F::BtlMoveParam_GetSpeed(c, p_chr_1, a1_) };
        { let a4_ = rt::fmul(l_speed_4, F::BtlInput_GetStickLength(c, p_chr_1)); F::BtlMove_Step(c, p_chr_1, 0i32, 5i32, 3i32, a4_, F32(0x3f6d0979)) };
        F::BtlMove_ApplyGravity(c, p_chr_1);
        F::BtlChar_SetFlag(c, p_chr_1, ((((201i32) as i64 as u64)) as u32));
        F::BtlMove_RequestOrbit(c, p_chr_1, F32(0x41a00000), F32(0x42480000));
        F::BtlChar_SetFlag(c, p_chr_1, ((((26i32) as i64 as u64)) as u32));
        F::BtlChar_SetFxBit(c, p_chr_1, 58i32);
        if ((((({ let a_ = (F::BtlMember_GetActiveGauge(c, p_chr_1)).wrapping_add(40); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
            F::BtlChar_SetFlag(c, p_chr_1, ((((150i32) as i64 as u64)) as u32));
        }
    }
    if (((((p_phase_2) == (2i32)) as i32)) != 0) {
        if ((F::BtlInput_TestAction(c, p_chr_1, 9i32, 1i32)) != 0) {
            { let a1_ = F::BtlAct_GetAction14or16(c, p_chr_1); F::BtlAct_Request(c, p_chr_1, a1_) };
        }
        if ((F::BtlParam_CanFly(c, p_chr_1)) != 0) {
            if ((F::BtlInput_TestAction(c, p_chr_1, 12i32, 1i32)) != 0) {
                { let a1_ = F::BtlAct_GetAction11or15(c, p_chr_1); F::BtlAct_Request(c, p_chr_1, a1_) };
            }
        }
        if ((F::BtlInput_TestAction(c, p_chr_1, 7i32, 1i32)) != 0) {
            F::BtlAct_Request(c, p_chr_1, 15i32);
            F::BtlChar_SetHeldFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32));
        }
        { let _ = F::BtlDecide_Main(c, p_chr_1, ((((((7168i32) as u32)) as u64)) as i32)); };
        { let a1_ = F::BtlAct_GetQueued(c, p_chr_1); F::BtlAct_Request(c, p_chr_1, a1_) };
        if ((F::BtlInput_TestAction(c, p_chr_1, 44i32, 1i32)) != 0) {
            F::BtlAct_Request(c, p_chr_1, 63i32);
        }
        if ((F::BtlDecide_Attack(c, p_chr_1, ((((((458752i32) as u32)) as u64)) as i32))) != 0) {
            { let a1_ = F::BtlAct_GetQueued(c, p_chr_1); F::BtlAct_Request(c, p_chr_1, a1_) };
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_JumpStartHandler", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_JumpAirHandler(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_JumpAirHandler", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let mut l_turn_3: F32 = F32(0);
    let mut l_speed_4: F32 = F32(0);
    let mut l_accel_5: F32 = F32(0);
    let r_ = 'f: {
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        if (((rt::fle(F32(0x00000000), { let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(156); F32(c.m.r32(a_)) }) as i32)) != 0) {
            F::BtlAnim_Play(c, p_chr_1, 33i32, F32(0x3e199999));
        } else {
            F::BtlAnim_Play(c, p_chr_1, 32i32, F32(0x3e199999));
        }
        F::BtlChar_ClearFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32));
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        l_speed_4 = F32(0x00000000);
        l_accel_5 = F32(0x3eed0979);
        l_turn_3 = F::BtlMoveParam_GetTurnRate(c, p_chr_1, 1i32);
        's1: {
            let sv6: i32 = F::BtlAct_GetCurrent(c, p_chr_1);
            match sv6 {
                17i32 => {
                    l_speed_4 = F::BtlMoveParam_GetSpeed(c, p_chr_1, 13i32);
                    break 's1;
                }
                19i32 => {
                    l_speed_4 = F::BtlMoveParam_GetSpeed(c, p_chr_1, 4i32);
                    break 's1;
                }
                _ => {
                }
            }
        }
        's2: {
            let sv7: i32 = F::BtlAnim_GetId(c, p_chr_1);
            match sv7 {
                32i32 => {
                    F::BtlAnim_AdvanceLoop(c, p_chr_1, 0i32);
                    if (((rt::fle(F32(0x00000000), { let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(156); F32(c.m.r32(a_)) }) as i32)) != 0) {
                        F::BtlAnim_Request(c, p_chr_1, 33i32, F32(0x3e199999));
                    }
                    break 's2;
                }
                33i32 => {
                    F::BtlAnim_AdvanceLoop(c, p_chr_1, 0i32);
                    if (((rt::flt(F::BtlAct_GetFramesToGround(c, p_chr_1), F32(0x40400000)) as i32)) != 0) {
                        if ((((((((((((!((F::BtlChar_IsStage4Or27(c)) != 0)) as i32)) != 0) || ((F::BtlChar_TestFlag(c, p_chr_1, ((((7i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) || (((rt::flt(F::BtlAct_GetGroundY(c, p_chr_1), F32(0xc1200000)) as i32)) != 0)) as i32)) != 0) {
                            F::BtlAnim_Request(c, p_chr_1, 34i32, F32(0x3dcccccc));
                        }
                    }
                    break 's2;
                }
                34i32 => {
                    if ((F::BtlAnim_Advance(c, p_chr_1, 0i32)) != 0) {
                        F::BtlAct_Request(c, p_chr_1, 11i32);
                    }
                    l_speed_4 = F32(0x00000000);
                    l_accel_5 = F32(0x3f6d0979);
                    if ((F::BtlAnim_PassedRatio(c, p_chr_1, F32(0x3dcccccc))) != 0) {
                        F::BtlAct_PlayLandFx(c, p_chr_1, 0i32);
                    }
                    break 's2;
                }
                _ => {
                }
            }
        }
        { let r_ = F::BtlInput_GetStickLength(c, p_chr_1); let n_ = rt::fmul(l_speed_4, r_); l_speed_4 = n_; };
        F::BtlMove_TurnYaw(c, p_chr_1, 0i32, l_turn_3);
        F::BtlMove_TurnPitch(c, p_chr_1, 5i32, F32(0x40490fda));
        F::BtlMove_TurnModelYaw(c, p_chr_1, F32(0x3f20d97a), F32(0x3e999999));
        F::BtlMove_SetDirection(c, p_chr_1, 3i32);
        F::BtlMove_Advance(c, p_chr_1, l_speed_4, l_accel_5);
        F::BtlMove_ApplyGravity(c, p_chr_1);
        F::BtlMove_RequestOrbit(c, p_chr_1, F32(0x41a00000), F32(0x42480000));
        F::BtlChar_SetFlag(c, p_chr_1, ((((26i32) as i64 as u64)) as u32));
        F::BtlChar_SetFxBit(c, p_chr_1, 58i32);
        if ((((({ let a_ = (F::BtlMember_GetActiveGauge(c, p_chr_1)).wrapping_add(40); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
            F::BtlChar_SetFlag(c, p_chr_1, ((((150i32) as i64 as u64)) as u32));
        }
    }
    if (((((p_phase_2) == (2i32)) as i32)) != 0) {
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((22i32) as i64 as u64)) as u32))) != 0) {
            F::BtlChar_SetHeldFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32));
            F::BtlAct_Request(c, p_chr_1, 11i32);
        }
        if ((F::BtlChar_IsStage4Or27(c)) != 0) {
            if ((F::BtlChar_TestFlag(c, p_chr_1, ((((23i32) as i64 as u64)) as u32))) != 0) {
                F::BtlChar_SetHeldFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32));
                F::BtlAct_Request(c, p_chr_1, 11i32);
            }
        }
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((17i32) as i64 as u64)) as u32))) != 0) {
            F::BtlChar_SetHeldFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32));
            F::BtlAct_Request(c, p_chr_1, 11i32);
        }
        if ((F::BtlInput_TestAction(c, p_chr_1, 10i32, 1i32)) != 0) {
            { let a1_ = F::BtlAct_GetAction14or16(c, p_chr_1); F::BtlAct_Request(c, p_chr_1, a1_) };
        }
        if ((F::BtlParam_CanFly(c, p_chr_1)) != 0) {
            if ((F::BtlInput_TestAction(c, p_chr_1, 12i32, 1i32)) != 0) {
                { let a1_ = F::BtlAct_GetAction11or15(c, p_chr_1); F::BtlAct_Request(c, p_chr_1, a1_) };
            }
        }
        { let _ = { let a1_ = (((if (((((F::BtlAnim_GetId(c, p_chr_1)) != (34i32)) as i32)) != 0) { 7168i32 } else { 7170i32 })) as u32); F::BtlDecide_Main(c, p_chr_1, ((((a1_) as u64)) as i32)) }; };
        { let a1_ = F::BtlAct_GetQueued(c, p_chr_1); F::BtlAct_Request(c, p_chr_1, a1_) };
        if ((F::BtlInput_TestAction(c, p_chr_1, 7i32, 1i32)) != 0) {
            F::BtlAct_Request(c, p_chr_1, 15i32);
            F::BtlChar_SetHeldFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32));
        }
        if (((((F::BtlAnim_GetId(c, p_chr_1)) != (34i32)) as i32)) != 0) {
            if ((F::BtlInput_TestAction(c, p_chr_1, 44i32, 1i32)) != 0) {
                F::BtlAct_Request(c, p_chr_1, 63i32);
            }
            if ((F::BtlDecide_Attack(c, p_chr_1, ((((((458752i32) as u32)) as u64)) as i32))) != 0) {
                { let a1_ = F::BtlAct_GetQueued(c, p_chr_1); F::BtlAct_Request(c, p_chr_1, a1_) };
            }
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_JumpAirHandler", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_AscendHandler(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_AscendHandler", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let mut l_motion_3: i32 = 0;
    let mut l_speed_4: F32 = F32(0);
    let mut l_vspeed_5: F32 = F32(0);
    let mut l_vaccel_6: F32 = F32(0);
    let r_ = 'f: {
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        l_motion_3 = 43i32;
        if ((((((c.m.r32((p_chr_1).wrapping_add(4080)) as i32)) > (0i32)) as i32)) != 0) {
            l_motion_3 = 46i32;
            F::BtlCharSnd_PlayCommon(c, p_chr_1, 31i32);
        }
        F::BtlAnim_Play(c, p_chr_1, l_motion_3, F32(0x3e199999));
        F::BtlChar_SetHeldFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32));
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        's1: {
            let sv7: i32 = F::BtlAnim_GetId(c, p_chr_1);
            match sv7 {
                46i32 => {
                    F::BtlAnim_SetDuration(c, p_chr_1, F32(0x3eb33333));
                    { let _ = F::BtlAnim_AdvanceThen(c, p_chr_1, 43i32, F32(0x3e199999), 0i32); };
                    F::BtlMove_Step(c, p_chr_1, 6i32, 5i32, 3i32, F32(0x00000000), F32(0x3eed0979));
                    F::BtlMove_ApplyGravity(c, p_chr_1);
                    break 's1;
                }
                43i32 => {
                    if ((F::BtlAnim_IsNew(c, p_chr_1)) != 0) {
                        F::BtlCharSnd_PlayCommon(c, p_chr_1, 24i32);
                    }
                    F::BtlAnim_AdvanceLoop(c, p_chr_1, 0i32);
                    l_speed_4 = rt::fmul({ let a1_ = F::BtlChar_TestFlag(c, p_chr_1, ((((17i32) as i64 as u64)) as u32)); F::BtlMoveParam_GetSpeed(c, p_chr_1, a1_) }, F::BtlInput_GetStickLength(c, p_chr_1));
                    l_vspeed_5 = rt::fneg(F::BtlMoveParam_GetSpeed(c, p_chr_1, 16i32));
                    l_vaccel_6 = F::BtlMoveParam_GetAccel(c, p_chr_1, 0i32);
                    F::BtlMove_Step(c, p_chr_1, 0i32, 5i32, 3i32, l_speed_4, F32(0x3eed0979));
                    F::BtlMove_MoveVertical(c, p_chr_1, l_vspeed_5, l_vaccel_6);
                    break 's1;
                }
                _ => {
                }
            }
        }
        F::BtlChar_SetFlag(c, p_chr_1, ((((201i32) as i64 as u64)) as u32));
        F::BtlMove_RequestOrbit(c, p_chr_1, F32(0x41a00000), F32(0x42480000));
        if ((((({ let a_ = (F::BtlMember_GetActiveGauge(c, p_chr_1)).wrapping_add(40); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
            F::BtlChar_SetFlag(c, p_chr_1, ((((150i32) as i64 as u64)) as u32));
        }
        F::BtlChar_SetFxBit(c, p_chr_1, 58i32);
    }
    if (((((p_phase_2) == (2i32)) as i32)) != 0) {
        if ((F::BtlInput_TestAction(c, p_chr_1, 11i32, 1i32)) != 0) {
            F::BtlAct_Request(c, p_chr_1, 11i32);
            { let _ = F::BtlDecide_Main(c, p_chr_1, ((((((65i32) as u32)) as u64)) as i32)); };
        }
        { let _ = F::BtlDecide_Main(c, p_chr_1, ((((((6144i32) as u32)) as u64)) as i32)); };
        { let _ = F::BtlDecide_Attack(c, p_chr_1, ((((((458752i32) as u32)) as u64)) as i32)); };
        { let a1_ = F::BtlAct_GetQueued(c, p_chr_1); F::BtlAct_Request(c, p_chr_1, a1_) };
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_AscendHandler", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_DescendHandler(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_DescendHandler", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let mut l_motion_3: i32 = 0;
    let mut l_speed_4: F32 = F32(0);
    let mut l_vspeed_5: F32 = F32(0);
    let mut l_vaccel_6: F32 = F32(0);
    let r_ = 'f: {
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        l_motion_3 = 44i32;
        if ((((((c.m.r32((p_chr_1).wrapping_add(4080)) as i32)) > (0i32)) as i32)) != 0) {
            l_motion_3 = 31i32;
            F::BtlCharSnd_PlayCommon(c, p_chr_1, 31i32);
        }
        F::BtlAnim_Play(c, p_chr_1, l_motion_3, F32(0x3e199999));
        F::BtlChar_SetHeldFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32));
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        l_speed_4 = rt::fmul({ let a1_ = F::BtlChar_TestFlag(c, p_chr_1, ((((17i32) as i64 as u64)) as u32)); F::BtlMoveParam_GetSpeed(c, p_chr_1, a1_) }, F::BtlInput_GetStickLength(c, p_chr_1));
        l_vspeed_5 = F::BtlMoveParam_GetSpeed(c, p_chr_1, 17i32);
        l_vaccel_6 = F::BtlMoveParam_GetAccel(c, p_chr_1, 1i32);
        's1: {
            let sv7: i32 = F::BtlAnim_GetId(c, p_chr_1);
            match sv7 {
                31i32 => {
                    F::BtlAnim_SetDuration(c, p_chr_1, F32(0x3eb33333));
                    { let _ = F::BtlAnim_AdvanceThen(c, p_chr_1, 44i32, F32(0x3e199999), 0i32); };
                    break 's1;
                }
                44i32 => {
                    if ((F::BtlAnim_IsNew(c, p_chr_1)) != 0) {
                        F::BtlCharSnd_PlayCommon(c, p_chr_1, 26i32);
                    }
                    F::BtlAnim_AdvanceLoop(c, p_chr_1, 0i32);
                    if (((rt::flt(F::BtlAct_GetFramesToGround(c, p_chr_1), F32(0x3f800000)) as i32)) != 0) {
                        if ((((((((((((!((F::BtlChar_IsStage4Or27(c)) != 0)) as i32)) != 0) || ((F::BtlChar_TestFlag(c, p_chr_1, ((((7i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) || (((rt::flt(F::BtlAct_GetGroundY(c, p_chr_1), F32(0xc1200000)) as i32)) != 0)) as i32)) != 0) {
                            if ((((!(((({ let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(208); (c.m.r32(a_) as i32) }) & (64i32))) != 0)) as i32)) != 0) {
                                F::BtlAnim_Request(c, p_chr_1, 45i32, F32(0x00000000));
                                F::BtlAct_PlayLandFx(c, p_chr_1, 1i32);
                            }
                        }
                    }
                    F::BtlMove_Step(c, p_chr_1, 0i32, 5i32, 3i32, l_speed_4, F32(0x3eed0979));
                    F::BtlMove_MoveVertical(c, p_chr_1, l_vspeed_5, l_vaccel_6);
                    F::BtlChar_SetFlag(c, p_chr_1, ((((201i32) as i64 as u64)) as u32));
                    break 's1;
                }
                45i32 => {
                    F::BtlChar_ClearFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32));
                    if ((F::BtlAnim_Advance(c, p_chr_1, 0i32)) != 0) {
                        { let v_ = 1i32; let a_ = ((p_chr_1).wrapping_add(976)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    }
                    F::BtlMove_Step(c, p_chr_1, 0i32, 5i32, 3i32, F32(0x00000000), F32(0x3f6d0979));
                    F::BtlMove_ApplyGravity(c, p_chr_1);
                    break 's1;
                }
                _ => {
                }
            }
        }
        F::BtlMove_RequestOrbit(c, p_chr_1, F32(0x41a00000), F32(0x42480000));
        if ((((({ let a_ = (F::BtlMember_GetActiveGauge(c, p_chr_1)).wrapping_add(40); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
            F::BtlChar_SetFlag(c, p_chr_1, ((((150i32) as i64 as u64)) as u32));
        }
        F::BtlChar_SetFxBit(c, p_chr_1, 58i32);
    }
    if (((((p_phase_2) == (2i32)) as i32)) != 0) {
        's2: {
            let sv8: i32 = F::BtlAnim_GetId(c, p_chr_1);
            match sv8 {
                31i32 | 44i32 => {
                    if ((F::BtlInput_TestAction(c, p_chr_1, 13i32, 1i32)) != 0) {
                        F::BtlAct_Request(c, p_chr_1, 11i32);
                        { let _ = F::BtlDecide_Main(c, p_chr_1, ((((((33i32) as u32)) as u64)) as i32)); };
                        { let a1_ = F::BtlAct_GetQueued(c, p_chr_1); F::BtlAct_Request(c, p_chr_1, a1_) };
                    }
                    break 's2;
                }
                45i32 => {
                    if ((((((c.m.r32(((p_chr_1).wrapping_add(976)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) != (0i32)) as i32)) != 0) {
                        if ((F::BtlInput_TestAction(c, p_chr_1, 1i32, 1i32)) != 0) {
                            F::BtlAct_Request(c, p_chr_1, 13i32);
                        } else {
                            F::BtlAct_Request(c, p_chr_1, 11i32);
                        }
                    }
                    break 's2;
                }
                _ => {
                }
            }
        }
        { let _ = F::BtlDecide_Main(c, p_chr_1, ((((((6144i32) as u32)) as u64)) as i32)); };
        { let a1_ = F::BtlAct_GetQueued(c, p_chr_1); F::BtlAct_Request(c, p_chr_1, a1_) };
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_DescendHandler", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_HopHandler(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_HopHandler", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let mut l_fade_3: F32 = F32(0);
    let mut l_speed_4: F32 = F32(0);
    let mut l_vspeed_5: F32 = F32(0);
    let mut l_vaccel_6: F32 = F32(0);
    let r_ = 'f: {
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        F::BtlAnim_Play(c, p_chr_1, 396i32, F32(0x3e199999));
        F::BtlChar_SetHeldFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32));
        F::BtlCharSnd_PlayCommon(c, p_chr_1, 31i32);
        { let v_ = F32(0x00000000); let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(156); c.m.w32(a_, (v_).0); };
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        l_fade_3 = F32(0x3f800000);
        l_speed_4 = { let a1_ = F::BtlChar_TestFlag(c, p_chr_1, ((((17i32) as i64 as u64)) as u32)); F::BtlMoveParam_GetSpeed(c, p_chr_1, a1_) };
        l_vspeed_5 = F::BtlMoveParam_GetSpeed(c, p_chr_1, 16i32);
        l_vaccel_6 = F::BtlMoveParam_GetAccel(c, p_chr_1, 0i32);
        { let r_ = F::BtlAnim_GetProgress(c, p_chr_1); let n_ = rt::fsub(l_fade_3, r_); l_fade_3 = n_; };
        { let r_ = rt::fmul(F::BtlInput_GetStickLength(c, p_chr_1), l_fade_3); let n_ = rt::fmul(l_speed_4, r_); l_speed_4 = n_; };
        { let r_ = l_fade_3; let n_ = rt::fmul(l_vspeed_5, r_); l_vspeed_5 = n_; };
        { let r_ = F32(0x3f000000); let n_ = rt::fmul(l_vaccel_6, r_); l_vaccel_6 = n_; };
        if ((F::BtlAnim_Advance(c, p_chr_1, 0i32)) != 0) {
            F::BtlAct_Request(c, p_chr_1, 11i32);
        }
        F::BtlMove_Step(c, p_chr_1, 0i32, 5i32, 2i32, l_speed_4, F32(0x3eed0979));
        F::BtlMove_MoveVertical(c, p_chr_1, rt::fneg(l_vspeed_5), l_vaccel_6);
        F::BtlChar_SetFlag(c, p_chr_1, ((((201i32) as i64 as u64)) as u32));
        F::BtlMove_RequestOrbit(c, p_chr_1, F32(0x41a00000), F32(0x42480000));
        F::BtlChar_SetFxBit(c, p_chr_1, 58i32);
    }
    if (((((p_phase_2) == (2i32)) as i32)) != 0) {
        { let _ = F::BtlDecide_Main(c, p_chr_1, ((((((6144i32) as u32)) as u64)) as i32)); };
        { let _ = F::BtlDecide_Attack(c, p_chr_1, ((((((458752i32) as u32)) as u64)) as i32)); };
        { let a1_ = F::BtlAct_GetQueued(c, p_chr_1); F::BtlAct_Request(c, p_chr_1, a1_) };
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_HopHandler", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_FastAscendHandler(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_FastAscendHandler", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let mut l_vspeed_3: F32 = F32(0);
    let mut l_stop_4: i32 = 0;
    let r_ = 'f: {
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        F::BtlAnim_Play(c, p_chr_1, 46i32, F32(0x3e199999));
        F::BtlChar_SetHeldFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32));
        if ((((((c.m.r32((p_chr_1).wrapping_add(4080)) as i32)) > (0i32)) as i32)) != 0) {
            { let r_ = 1i32; let a_ = ((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        }
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        l_vspeed_3 = rt::fneg(F::BtlMoveParam_GetSpeed(c, p_chr_1, 9i32));
        's1: {
            let sv5: i32 = F::BtlAnim_GetId(c, p_chr_1);
            match sv5 {
                46i32 => {
                    if (((((c.m.r32(((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) & (1i32))) != 0) {
                        F::BtlAnim_SetDuration(c, p_chr_1, F32(0x3f000000));
                    }
                    if ((F::BtlAnim_AdvanceThen(c, p_chr_1, 47i32, F32(0x3e199999), 0i32)) != 0) {
                        { let a1_ = F::BtlParam_GetDashSound(c, p_chr_1); F::BtlCharSnd_PlayCommon(c, p_chr_1, a1_) };
                        F::BtlChar_Vibrate(c, p_chr_1, F32(0x3f4ccccc), F32(0x3e999999));
                    }
                    F::BtlMove_MoveVertical(c, p_chr_1, F32(0x00000000), F32(0x3f6d0979));
                    break 's1;
                }
                47i32 => {
                    F::BtlAnim_AdvanceLoop(c, p_chr_1, 0i32);
                    F::BtlMove_MoveVertical(c, p_chr_1, l_vspeed_3, F32(0x461c4000));
                    F::BtlChar_SetFxBit(c, p_chr_1, 4i32);
                    F::BtlChar_SetFlag(c, p_chr_1, ((((9i32) as i64 as u64)) as u32));
                    F::BtlChar_SetVibration(c, p_chr_1, F32(0x3f333333), F32(0x3dcccccc));
                    break 's1;
                }
                48i32 => {
                    if ((F::BtlAnim_Advance(c, p_chr_1, 0i32)) != 0) {
                        { let v_ = 1i32; let a_ = ((p_chr_1).wrapping_add(976)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    }
                    F::BtlMove_MoveVertical(c, p_chr_1, F32(0x00000000), F32(0x3f6d0979));
                    break 's1;
                }
                _ => {
                }
            }
        }
        F::BtlMove_Step(c, p_chr_1, 6i32, 5i32, 3i32, F32(0x00000000), F32(0x3f6d0979));
        F::BtlChar_SetFxBit(c, p_chr_1, 58i32);
    }
    if (((((p_phase_2) == (2i32)) as i32)) != 0) {
        's2: {
            let sv6: i32 = F::BtlAnim_GetId(c, p_chr_1);
            match sv6 {
                46i32 => {
                    break 's2;
                }
                47i32 => {
                    {
                        l_stop_4 = 0i32;
                        if ((F::BtlInput_TestAction(c, p_chr_1, 16i32, 1i32)) != 0) {
                            l_stop_4 = 1i32;
                        }
                        if (({ let a1_ = F::BtlMoveParam_GetKiCost(c, p_chr_1, 2i32); F::BtlMember_SpendKi(c, p_chr_1, a1_, 0i32) }) != 0) {
                            l_stop_4 = 1i32;
                        }
                        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((21i32) as i64 as u64)) as u32))) != 0) {
                            l_stop_4 = 1i32;
                        }
                        if ((l_stop_4) != 0) {
                            F::BtlAnim_Request(c, p_chr_1, 48i32, F32(0x3e199999));
                        }
                        break 's2;
                    }
                }
                48i32 => {
                    if ((((((c.m.r32(((p_chr_1).wrapping_add(976)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) != (0i32)) as i32)) != 0) {
                        F::BtlAct_Request(c, p_chr_1, 11i32);
                        { let _ = F::BtlDecide_Main(c, p_chr_1, ((((((1i32) as u32)) as u64)) as i32)); };
                        { let a1_ = F::BtlAct_GetQueued(c, p_chr_1); F::BtlAct_Request(c, p_chr_1, a1_) };
                    }
                    break 's2;
                }
                _ => {
                }
            }
        }
        { let _ = F::BtlDecide_Main(c, p_chr_1, ((((((5120i32) as u32)) as u64)) as i32)); };
        { let a1_ = F::BtlAct_GetQueued(c, p_chr_1); F::BtlAct_Request(c, p_chr_1, a1_) };
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_FastAscendHandler", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_FastDescendHandler(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_FastDescendHandler", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let mut l_vspeed_3: F32 = F32(0);
    let mut l_stop_4: i32 = 0;
    let r_ = 'f: {
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        F::BtlAnim_Play(c, p_chr_1, 49i32, F32(0x3e199999));
        F::BtlChar_SetHeldFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32));
        if ((((((c.m.r32((p_chr_1).wrapping_add(4080)) as i32)) > (0i32)) as i32)) != 0) {
            { let r_ = 1i32; let a_ = ((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        }
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        l_vspeed_3 = F::BtlMoveParam_GetSpeed(c, p_chr_1, 9i32);
        's1: {
            let sv5: i32 = F::BtlAnim_GetId(c, p_chr_1);
            match sv5 {
                49i32 => {
                    if (((((c.m.r32(((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) & (1i32))) != 0) {
                        F::BtlAnim_SetDuration(c, p_chr_1, F32(0x3f000000));
                    }
                    if ((F::BtlAnim_AdvanceThen(c, p_chr_1, 50i32, F32(0x3e199999), 0i32)) != 0) {
                        { let a1_ = F::BtlParam_GetDashSound(c, p_chr_1); F::BtlCharSnd_PlayCommon(c, p_chr_1, a1_) };
                        F::BtlChar_Vibrate(c, p_chr_1, F32(0x3f4ccccc), F32(0x3e999999));
                    }
                    F::BtlMove_MoveVertical(c, p_chr_1, F32(0x00000000), F32(0x3f6d0979));
                    break 's1;
                }
                50i32 => {
                    F::BtlAnim_AdvanceLoop(c, p_chr_1, 0i32);
                    F::BtlMove_MoveVertical(c, p_chr_1, l_vspeed_3, F32(0x461c4000));
                    F::BtlChar_SetFxBit(c, p_chr_1, 4i32);
                    F::BtlChar_SetFlag(c, p_chr_1, ((((9i32) as i64 as u64)) as u32));
                    F::BtlChar_SetVibration(c, p_chr_1, F32(0x3f333333), F32(0x3dcccccc));
                    break 's1;
                }
                51i32 => {
                    if ((F::BtlAnim_Advance(c, p_chr_1, 0i32)) != 0) {
                        { let v_ = 1i32; let a_ = ((p_chr_1).wrapping_add(976)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    }
                    F::BtlMove_MoveVertical(c, p_chr_1, F32(0x00000000), F32(0x3f6d0979));
                    break 's1;
                }
                _ => {
                }
            }
        }
        F::BtlMove_Step(c, p_chr_1, 6i32, 5i32, 3i32, F32(0x00000000), F32(0x3f6d0979));
        F::BtlChar_SetFxBit(c, p_chr_1, 58i32);
    }
    if (((((p_phase_2) == (2i32)) as i32)) != 0) {
        's2: {
            let sv6: i32 = F::BtlAnim_GetId(c, p_chr_1);
            match sv6 {
                49i32 => {
                    break 's2;
                }
                50i32 => {
                    {
                        l_stop_4 = 0i32;
                        if ((F::BtlInput_TestAction(c, p_chr_1, 18i32, 1i32)) != 0) {
                            l_stop_4 = 1i32;
                        }
                        if ((((((((!(((({ let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(208); (c.m.r32(a_) as i32) }) & (1073741824i32))) != 0)) as i32)) != 0) || (((({ let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(208); (c.m.r32(a_) as i32) }) & (16777216i32))) != 0)) as i32)) != 0) {
                            if (((rt::flt(F::BtlAct_GetFramesToGround(c, p_chr_1), F32(0x410fffff)) as i32)) != 0) {
                                l_stop_4 = 1i32;
                            }
                        }
                        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((22i32) as i64 as u64)) as u32))) != 0) {
                            l_stop_4 = 1i32;
                        }
                        if (({ let a1_ = F::BtlMoveParam_GetKiCost(c, p_chr_1, 2i32); F::BtlMember_SpendKi(c, p_chr_1, a1_, 0i32) }) != 0) {
                            l_stop_4 = 1i32;
                        }
                        if ((l_stop_4) != 0) {
                            F::BtlAnim_Request(c, p_chr_1, 51i32, F32(0x3e199999));
                        }
                        break 's2;
                    }
                }
                51i32 => {
                    if ((((((c.m.r32(((p_chr_1).wrapping_add(976)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) != (0i32)) as i32)) != 0) {
                        F::BtlAct_Request(c, p_chr_1, 11i32);
                        { let _ = F::BtlDecide_Main(c, p_chr_1, ((((((1i32) as u32)) as u64)) as i32)); };
                        { let a1_ = F::BtlAct_GetQueued(c, p_chr_1); F::BtlAct_Request(c, p_chr_1, a1_) };
                    }
                    break 's2;
                }
                _ => {
                }
            }
        }
        { let _ = F::BtlDecide_Main(c, p_chr_1, ((((((3072i32) as u32)) as u64)) as i32)); };
        { let a1_ = F::BtlAct_GetQueued(c, p_chr_1); F::BtlAct_Request(c, p_chr_1, a1_) };
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_FastDescendHandler", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_HomingDashHandler(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_HomingDashHandler", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let mut l_slowFrames_3: u32 = 0;
    let mut l_prev_4: i32 = 0;
    let mut l_motion_5: i32 = 0;
    let mut l_voice_6: i32 = 0;
    let mut l_moving_7: i32 = 0;
    let mut l_faceOpp_8: i32 = 0;
    let mut l_first_9: i32 = 0;
    let mut l_yawMode_10: i32 = 0;
    let mut l_lean_11: F32 = F32(0);
    let mut l_speed_12: F32 = F32(0);
    let mut l_yawAccel_13: F32 = F32(0);
    let mut l_pitchAccel_14: F32 = F32(0);
    let mut l_yawMax_15: F32 = F32(0);
    let mut l_pitchMax_16: F32 = F32(0);
    let mut l_modelTurn_17: F32 = F32(0);
    let mut l_near_18: F32 = F32(0);
    let mut l_pose_19: u32 = 0;
    let mut l_stop_20: i32 = 0;
    let r_ = 'f: {
    l_slowFrames_3 = ((p_chr_1).wrapping_add(976)).wrapping_add((((2i32) as u32)).wrapping_mul(4));
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        l_voice_6 = (1i32).wrapping_neg();
        l_prev_4 = F::BtlAct_GetPrev(c, p_chr_1);
        if ((F::BtlAct_IsAttackId(c, l_prev_4)) != 0) {
            l_motion_5 = 25i32;
            { let a_ = (p_chr_1).wrapping_add(3432); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        } else {
            's1: {
                let sv21: i32 = l_prev_4;
                'k7: {
                'k6: {
                'k5: {
                'k4: {
                'k3: {
                'k2: {
                    match sv21 { 71i32..=82i32 => break 'k2, 83i32..=87i32 => break 'k3, 90i32..=93i32 => break 'k3, 107i32..=111i32 => break 'k3, 26i32 => break 'k4, 40i32 => break 'k4, 42i32 => break 'k5, 215i32 => break 'k5, 23i32 => break 'k6, 24i32 => break 'k6, _ => break 'k7, }
                }
                l_voice_6 = 28i32;
                }
                { let a_ = (p_chr_1).wrapping_add(3432); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                }
                l_motion_5 = 25i32;
                break 's1;
                }
                l_motion_5 = 25i32;
                break 's1;
                }
                l_motion_5 = 45i32;
                break 's1;
                }
                l_motion_5 = 24i32;
                l_voice_6 = 36i32;
                break 's1;
            }
        }
        F::BtlAnim_Play(c, p_chr_1, l_motion_5, F32(0x3e199999));
        if (((((l_voice_6) >= (0i32)) as i32)) != 0) {
            F::BtlChar_PlayVoice(c, p_chr_1, l_voice_6);
        }
        F::BtlChar_SetHeldFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32));
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        l_lean_11 = F32(0x00000000);
        l_moving_7 = 1i32;
        l_faceOpp_8 = 1i32;
        l_first_9 = 0i32;
        's8: {
            let sv22: i32 = F::BtlAnim_GetId(c, p_chr_1);
            match sv22 {
                24i32 | 45i32 => {
                    { let _ = F::BtlAnim_AdvanceThen(c, p_chr_1, 25i32, F32(0x3e4ccccc), 0i32); };
                    l_lean_11 = F::BtlAnim_GetProgress(c, p_chr_1);
                    l_moving_7 = 0i32;
                    if (((rt::flt(F32(0x3f000000), F::BtlAnim_GetProgress(c, p_chr_1)) as i32)) != 0) {
                        F::BtlChar_SetFlag(c, p_chr_1, ((((71i32) as i64 as u64)) as u32));
                    }
                    break 's8;
                }
                25i32 => {
                    F::BtlAnim_AdvanceLoop(c, p_chr_1, 0i32);
                    if ((F::BtlAnim_IsNew(c, p_chr_1)) != 0) {
                        l_first_9 = 1i32;
                        { let a1_ = F::BtlParam_GetDashSound(c, p_chr_1); F::BtlCharSnd_PlayCommon(c, p_chr_1, a1_) };
                        F::BtlChar_SetFlag(c, p_chr_1, ((((209i32) as i64 as u64)) as u32));
                        F::BtlChar_Vibrate(c, p_chr_1, F32(0x3f4ccccc), F32(0x3e999999));
                        if (((rt::flt(F::BtlAct_GetHeight(c, p_chr_1), F32(0x41a00000)) as i32)) != 0) {
                            F::BtlChar_SetFxBit(c, p_chr_1, 53i32);
                        }
                    }
                    F::BtlChar_SetFlag(c, p_chr_1, ((((71i32) as i64 as u64)) as u32));
                    l_lean_11 = F32(0x3f800000);
                    l_moving_7 = 1i32;
                    F::BtlChar_SetVibration(c, p_chr_1, F32(0x3f333333), F32(0x3dcccccc));
                    break 's8;
                }
                21i32 | 26i32 => {
                    if ((F::BtlAnim_Advance(c, p_chr_1, 0i32)) != 0) {
                        F::BtlAct_Request(c, p_chr_1, 11i32);
                        { let v_ = F32(0x00000000); let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(152); c.m.w32(a_, (v_).0); };
                    }
                    l_moving_7 = 0i32;
                    l_faceOpp_8 = 0i32;
                    l_lean_11 = rt::fsub(F32(0x3f800000), F::BtlAnim_GetProgress(c, p_chr_1));
                    break 's8;
                }
                _ => {
                }
            }
        }
        if ((l_moving_7) != 0) {
            l_speed_12 = F::BtlMoveParam_GetSpeed(c, p_chr_1, 6i32);
            l_yawAccel_13 = F::BtlMoveParam_GetTurnAccel(c, p_chr_1, 0i32);
            l_pitchAccel_14 = F::BtlMoveParam_GetTurnAccel(c, p_chr_1, 1i32);
            l_yawMax_15 = F::BtlMoveParam_GetTurnMax(c, p_chr_1, 0i32);
            l_pitchMax_16 = F::BtlMoveParam_GetTurnMax(c, p_chr_1, 1i32);
            l_modelTurn_17 = F32(0x3f20d97a);
            if ((l_first_9) != 0) {
                l_pitchAccel_14 = F32(0x40490fda);
                l_yawAccel_13 = l_pitchAccel_14;
                l_modelTurn_17 = l_pitchAccel_14;
            }
            l_near_18 = { let a0_ = rt::fmul(F::BtlOpp_GetGapXZ(c, p_chr_1), F32(0x3c23d70a)); F::BtlUtil_MinF(c, a0_, F32(0x3f800000)) };
            F::BtlMove_SteerAtOpponent(c, p_chr_1, l_speed_12, l_yawAccel_13, l_pitchAccel_14, rt::fmul(l_yawMax_15, l_near_18), rt::fmul(l_pitchMax_16, l_near_18), F32(0x40490fda));
            F::BtlMove_TurnModelYaw(c, p_chr_1, l_modelTurn_17, F32(0x3f000000));
            F::BtlMove_SetDirection(c, p_chr_1, 3i32);
            F::BtlMove_Advance(c, p_chr_1, l_speed_12, F32(0x461c4000));
            F::BtlMove_ApplyGravity(c, p_chr_1);
            F::BtlChar_SetFlag(c, p_chr_1, ((((9i32) as i64 as u64)) as u32));
            F::BtlChar_SetFlag(c, p_chr_1, ((((80i32) as i64 as u64)) as u32));
            F::BtlChar_SetFlag(c, p_chr_1, ((((81i32) as i64 as u64)) as u32));
            F::BtlChar_SetFxBit(c, p_chr_1, 4i32);
            F::BtlChar_SetFxBit(c, p_chr_1, 9i32);
        } else {
            l_yawMode_10 = (if ((l_faceOpp_8) != 0) { 2i32 } else { 6i32 });
            if ((F::BtlMove_IsBlockedByOpponent(c, p_chr_1)) != 0) {
                { let v_ = F32(0x00000000); let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(152); c.m.w32(a_, (v_).0); };
            }
            F::BtlMove_Step(c, p_chr_1, l_yawMode_10, 6i32, 7i32, F32(0x00000000), F32(0x3f6d0979));
            F::BtlMove_ApplyGravity(c, p_chr_1);
        }
        F::BtlChar_SetFlag(c, p_chr_1, ((((137i32) as i64 as u64)) as u32));
        { let a1_ = rt::fmul({ let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(144); F32(c.m.r32(a_)) }, l_lean_11); F::BtlMove_SetLeanX(c, p_chr_1, a1_) };
        F::BtlMove_RequestOrbit(c, p_chr_1, F32(0x42480000), F32(0x42c80000));
        F::BtlChar_SetFxBit(c, p_chr_1, 58i32);
    }
    if (((((p_phase_2) == (2i32)) as i32)) != 0) {
        's9: {
            let sv23: i32 = F::BtlAnim_GetId(c, p_chr_1);
            match sv23 {
                24i32 | 45i32 => {
                    break 's9;
                }
                25i32 => {
                    {
                        l_stop_20 = 0i32;
                        if ((F::BtlMove_IsBlockedByOpponent(c, p_chr_1)) != 0) {
                            if (((((F::BtlOpp_GetSeenAction(c, p_chr_1)) != (213i32)) as i32)) != 0) {
                                l_stop_20 = 1i32;
                            }
                        }
                        if (((rt::flt({ let a0_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(64); F::Vec3_Length(c, a0_) }, F32(0x3f6d0979)) as i32)) != 0) {
                            if ((((({ let a_ = l_slowFrames_3; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); n_ }) >= (16i32)) as i32)) != 0) {
                                l_stop_20 = 1i32;
                            }
                        } else {
                            { let v_ = 0i32; let a_ = l_slowFrames_3; c.m.w32(a_, (v_) as u32); };
                        }
                        if ((((!((F::BtlChar_TestFlag(c, p_chr_1, ((((5i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) {
                            l_stop_20 = 1i32;
                        }
                        if (({ let a1_ = F::BtlMoveParam_GetKiCost(c, p_chr_1, 0i32); F::BtlMember_SpendKi(c, p_chr_1, a1_, 0i32) }) != 0) {
                            l_stop_20 = 1i32;
                        }
                        if ((F::BtlInput_TestAction(c, p_chr_1, 22i32, 1i32)) != 0) {
                            F::BtlAnim_Request(c, p_chr_1, 21i32, F32(0x00000000));
                        }
                        if ((F::BtlInput_TestAction(c, p_chr_1, 21i32, 1i32)) != 0) {
                            F::BtlAnim_Request(c, p_chr_1, 21i32, F32(0x00000000));
                            { let r_ = 1i32; let a_ = ((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                        }
                        if ((F::BtlDecide_Main(c, p_chr_1, ((((((134217728i32) as u32)) as u64)) as i32))) != 0) {
                            { let a1_ = F::BtlAct_GetQueued(c, p_chr_1); F::BtlAct_Request(c, p_chr_1, a1_) };
                        }
                        if ((l_stop_20) != 0) {
                            F::BtlAnim_Request(c, p_chr_1, 26i32, F32(0x00000000));
                            l_pose_19 = F::BtlChar_GetPos(c, p_chr_1);
                            { let r_ = F32(0x3f000000); let a_ = (l_pose_19).wrapping_add(152); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                        }
                        break 's9;
                    }
                }
                26i32 => {
                    if ((F::BtlMove_IsBlockedByOpponent(c, p_chr_1)) != 0) {
                        { let v_ = F32(0x00000000); let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(152); c.m.w32(a_, (v_).0); };
                    }
                    break 's9;
                }
                21i32 => {
                    if ((F::BtlInput_TestAction(c, p_chr_1, 24i32, 1i32)) != 0) {
                        { let r_ = 1i32; let a_ = ((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    }
                    if (((rt::flt(F32(0x3f000000), F::BtlAnim_GetProgress(c, p_chr_1)) as i32)) != 0) {
                        if (((((c.m.r32(((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) & (1i32))) != 0) {
                            F::BtlAct_Request(c, p_chr_1, 26i32);
                        }
                    }
                    if ((F::BtlDecide_Main(c, p_chr_1, ((((((134217728i32) as u32)) as u64)) as i32))) != 0) {
                        { let a1_ = F::BtlAct_GetQueued(c, p_chr_1); F::BtlAct_Request(c, p_chr_1, a1_) };
                    }
                    break 's9;
                }
                _ => {
                }
            }
        }
        { let _ = F::BtlDecide_Main(c, p_chr_1, ((((((6144i32) as u32)) as u64)) as i32)); };
        's10: {
            let sv24: i32 = F::BtlAnim_GetId(c, p_chr_1);
            match sv24 {
                25i32 => {
                    { let _ = F::BtlDecide_Attack(c, p_chr_1, ((((((8192i32) as u32)) as u64)) as i32)); };
                    break 's10;
                }
                26i32 => {
                    { let _ = F::BtlDecide_Attack(c, p_chr_1, ((((((3i32) as u32)) as u64)) as i32)); };
                    break 's10;
                }
                _ => {
                }
            }
        }
        { let a1_ = F::BtlAct_GetQueued(c, p_chr_1); F::BtlAct_Request(c, p_chr_1, a1_) };
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_HomingDashHandler", &PROBE_N, &[], true); }
    r_
}
