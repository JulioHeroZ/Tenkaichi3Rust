#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn BtlAct_DragonDashHandler(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_DragonDashHandler", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let mut l_counter_3: u32 = 0;
    let mut l_flag_4: i32 = 0;
    let mut l_speed_5: F32 = F32(0);
    let mut l_turn_6: F32 = F32(0);
    let mut l_vspeed_7: F32 = F32(0);
    let mut l_anim_8: i32 = 0;
    let r_ = 'f: {
    l_counter_3 = ((p_chr_1).wrapping_add(976)).wrapping_add((((2i32) as u32)).wrapping_mul(4));
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        's1: {
            let sv9: i32 = F::BtlAct_GetPrev(c, p_chr_1);
            match sv9 {
                25i32 | 26i32 => {
                    l_anim_8 = 25i32;
                    break 's1;
                }
                _ => {
                    l_anim_8 = 45i32;
                    break 's1;
                }
            }
        }
        if ((((((c.m.r32((p_chr_1).wrapping_add(4080)) as i32)) > (0i32)) as i32)) != 0) {
            l_anim_8 = 24i32;
        }
        F::BtlAnim_Play(c, p_chr_1, l_anim_8, F32(0x3e199999));
        F::BtlChar_SetHeldFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32));
        F::BtlMove_TurnYaw(c, p_chr_1, 0i32, F32(0x40490fda));
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        l_flag_4 = 1i32;
        's2: {
            let sv10: i32 = F::BtlAnim_GetId(c, p_chr_1);
            match sv10 {
                24i32 | 45i32 => {
                    { let _ = F::BtlAnim_AdvanceThen(c, p_chr_1, 25i32, F32(0x3e4ccccc), 0i32); };
                    l_flag_4 = 0i32;
                    break 's2;
                }
                25i32 => {
                    F::BtlAnim_AdvanceLoop(c, p_chr_1, 0i32);
                    if ((F::BtlAnim_IsNew(c, p_chr_1)) != 0) {
                        { let a1_ = F::BtlParam_GetDashSound(c, p_chr_1); F::BtlCharSnd_PlayCommon(c, p_chr_1, a1_) };
                        F::BtlChar_Vibrate(c, p_chr_1, F32(0x3f4ccccc), F32(0x3e999999));
                    } else {
                        F::BtlChar_SetVibration(c, p_chr_1, F32(0x3f333333), F32(0x3dcccccc));
                    }
                    l_flag_4 = 1i32;
                    break 's2;
                }
                21i32 | 26i32 => {
                    if ((F::BtlAnim_Advance(c, p_chr_1, 0i32)) != 0) {
                        F::BtlAct_Request(c, p_chr_1, 11i32);
                        { let v_ = F32(0x00000000); let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(152); c.m.w32(a_, (v_).0); };
                    }
                    l_flag_4 = 0i32;
                    break 's2;
                }
                _ => {
                }
            }
        }
        if ((l_flag_4) != 0) {
            l_speed_5 = F::BtlMoveParam_GetSpeed(c, p_chr_1, 7i32);
            l_turn_6 = F::BtlMoveParam_GetTurnRate(c, p_chr_1, 2i32);
            l_vspeed_7 = F::BtlMoveParam_GetSpeed(c, p_chr_1, 8i32);
            F::BtlMove_TurnYaw(c, p_chr_1, 0i32, l_turn_6);
            F::BtlMove_TurnPitch(c, p_chr_1, 5i32, F32(0x40490fda));
            F::BtlMove_TurnModelYaw(c, p_chr_1, F32(0x3f20d97a), F32(0x3e999999));
            F::BtlMove_SetDirection(c, p_chr_1, 3i32);
            F::BtlMove_Advance(c, p_chr_1, l_speed_5, F32(0x461c4000));
            if ((F::BtlInput_IsHeld(c, p_chr_1, ((2048i32) as u32))) != 0) {
                F::BtlMove_MoveVertical(c, p_chr_1, rt::fneg(l_vspeed_7), F32(0x3f6d0979));
            } else {
                if ((F::BtlInput_IsHeld(c, p_chr_1, ((4096i32) as u32))) != 0) {
                    F::BtlMove_MoveVertical(c, p_chr_1, l_vspeed_7, F32(0x3f6d0979));
                } else {
                    F::BtlMove_ApplyGravity(c, p_chr_1);
                }
            }
            F::BtlChar_SetFlag(c, p_chr_1, ((((9i32) as i64 as u64)) as u32));
            F::BtlChar_SetFlag(c, p_chr_1, ((((80i32) as i64 as u64)) as u32));
            F::BtlChar_SetFlag(c, p_chr_1, ((((81i32) as i64 as u64)) as u32));
            F::BtlChar_SetFxBit(c, p_chr_1, 4i32);
        } else {
            F::BtlMove_Step(c, p_chr_1, 6i32, 5i32, 3i32, F32(0x00000000), F32(0x3f6d0979));
            F::BtlMove_ApplyGravity(c, p_chr_1);
        }
        F::BtlMove_RequestOrbit(c, p_chr_1, F32(0x42480000), F32(0x42c80000));
        F::BtlChar_SetFlag(c, p_chr_1, ((((26i32) as i64 as u64)) as u32));
        F::BtlChar_SetFxBit(c, p_chr_1, 58i32);
    }
    if (((((p_phase_2) == (2i32)) as i32)) != 0) {
        's3: {
            let sv11: i32 = F::BtlAnim_GetId(c, p_chr_1);
            match sv11 {
                24i32 | 45i32 => {
                    break 's3;
                }
                25i32 => {
                    l_flag_4 = (((F::BtlMove_IsBlockedByOpponent(c, p_chr_1)) != (0i32)) as i32);
                    if (((rt::flt({ let a0_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(64); F::Vec3_Length(c, a0_) }, F32(0x3f6d0979)) as i32)) != 0) {
                        if ((((({ let a_ = l_counter_3; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); n_ }) >= (16i32)) as i32)) != 0) {
                            l_flag_4 = 1i32;
                        }
                    } else {
                        { let v_ = 0i32; let a_ = l_counter_3; c.m.w32(a_, (v_) as u32); };
                    }
                    if (({ let a1_ = F::BtlMoveParam_GetKiCost(c, p_chr_1, 1i32); F::BtlMember_SpendKi(c, p_chr_1, a1_, 0i32) }) != 0) {
                        l_flag_4 = 1i32;
                    }
                    if ((F::BtlInput_TestAction(c, p_chr_1, 22i32, 1i32)) != 0) {
                        F::BtlAnim_Request(c, p_chr_1, 21i32, F32(0x00000000));
                    }
                    if ((F::BtlInput_TestAction(c, p_chr_1, 20i32, 1i32)) != 0) {
                        F::BtlAnim_Request(c, p_chr_1, 21i32, F32(0x00000000));
                        { let r_ = 1i32; let a_ = ((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    }
                    if ((F::BtlInput_TestAction(c, p_chr_1, 21i32, 1i32)) != 0) {
                        F::BtlAnim_Request(c, p_chr_1, 21i32, F32(0x00000000));
                        { let r_ = 2i32; let a_ = ((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    }
                    if ((l_flag_4) != 0) {
                        F::BtlAnim_Request(c, p_chr_1, 26i32, F32(0x00000000));
                        { let r_ = F32(0x3f000000); let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(152); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    }
                    break 's3;
                }
                26i32 => {
                    if ((F::BtlMove_IsBlockedByOpponent(c, p_chr_1)) != 0) {
                        { let v_ = F32(0x00000000); let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(152); c.m.w32(a_, (v_).0); };
                    }
                    break 's3;
                }
                21i32 => {
                    if ((F::BtlInput_TestAction(c, p_chr_1, 23i32, 1i32)) != 0) {
                        { let r_ = 1i32; let a_ = ((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    }
                    if ((F::BtlInput_TestAction(c, p_chr_1, 24i32, 1i32)) != 0) {
                        { let r_ = 2i32; let a_ = ((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    }
                    if (((rt::fgt(F::BtlAnim_GetProgress(c, p_chr_1), F32(0x3f000000)) as i32)) != 0) {
                        if (((((c.m.r32(((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) & (1i32))) != 0) {
                            if ((F::BtlChar_TestFlag(c, p_chr_1, ((((5i32) as i64 as u64)) as u32))) != 0) {
                                F::BtlAct_Request(c, p_chr_1, 25i32);
                            }
                        }
                        if (((((c.m.r32(((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) & (2i32))) != 0) {
                            F::BtlAct_Request(c, p_chr_1, 26i32);
                        }
                    }
                    break 's3;
                }
                _ => {
                }
            }
        }
        { let _ = F::BtlDecide_Main(c, p_chr_1, 6144i32); };
        { let a1_ = F::BtlAct_GetQueued(c, p_chr_1); F::BtlAct_Request(c, p_chr_1, a1_) };
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_DragonDashHandler", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_StepHandler(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_StepHandler", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let mut l_voice_3: i32 = 0;
    let mut l_anim_4: i32 = 0;
    let mut l_yaw_5: F32 = F32(0);
    let mut l_speed_6: F32 = F32(0);
    let r_ = 'f: {
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        l_anim_4 = 0i32;
        l_voice_3 = (1i32).wrapping_neg();
        if (((rt::flt(F::BtlAct_GetHeight(c, p_chr_1), F32(0x40a00000)) as i32)) != 0) {
            F::BtlChar_ClearFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32));
        }
        l_yaw_5 = F::BtlOpp_GetYaw(c, p_chr_1);
        's1: {
            let sv7: i32 = F::BtlAct_GetCurrent(c, p_chr_1);
            match sv7 {
                27i32 => {
                    l_anim_4 = (if ((F::BtlChar_TestFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32))) != 0) { 40i32 } else { 28i32 });
                    l_voice_3 = 9i32;
                    l_yaw_5 = F::BtlUtil_WrapAngle(c, rt::fadd(l_yaw_5, F32(0x40490fda)));
                    break 's1;
                }
                28i32 => {
                    l_anim_4 = (if ((F::BtlChar_TestFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32))) != 0) { 41i32 } else { 29i32 });
                    l_voice_3 = 8i32;
                    l_yaw_5 = F::BtlUtil_WrapAngle(c, rt::fsub(l_yaw_5, F32(0x3fc90fda)));
                    break 's1;
                }
                29i32 => {
                    l_anim_4 = (if ((F::BtlChar_TestFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32))) != 0) { 42i32 } else { 30i32 });
                    l_voice_3 = 8i32;
                    l_yaw_5 = F::BtlUtil_WrapAngle(c, rt::fadd(l_yaw_5, F32(0x3fc90fda)));
                    break 's1;
                }
                _ => {
                }
            }
        }
        F::BtlAnim_Play(c, p_chr_1, l_anim_4, F32(0x3e199999));
        { let v_ = F::Mathf_Sin(c, l_yaw_5); let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(128); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = ((F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(128)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = F::Mathf_Cos(c, l_yaw_5); let a_ = ((F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(128)).wrapping_add(8); c.m.w32(a_, (v_).0); };
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32))) != 0) {
            F::BtlCharSnd_PlayCommon(c, p_chr_1, 31i32);
        } else {
            if (((((F::BtlAct_GetCurrent(c, p_chr_1)) == (27i32)) as i32)) != 0) {
                F::BtlCharSnd_PlayCommon(c, p_chr_1, 31i32);
            } else {
                F::BtlCharSnd_PlayBank8(c, p_chr_1, 1i32);
            }
        }
        if (((((l_voice_3) >= (0i32)) as i32)) != 0) {
            F::BtlChar_PlayVoice(c, p_chr_1, l_voice_3);
        }
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        l_speed_6 = F32(0x00000000);
        l_yaw_5 = F::BtlAnim_GetProgress(c, p_chr_1);
        if ((F::BtlAnim_Advance(c, p_chr_1, 0i32)) != 0) {
            { let v_ = 1i32; let a_ = ((p_chr_1).wrapping_add(976)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        }
        if (((rt::flt(F::BtlAnim_GetProgress(c, p_chr_1), F32(0x3e4ccccc)) as i32)) != 0) {
            F::BtlChar_SetFlag(c, p_chr_1, ((((66i32) as i64 as u64)) as u32));
        }
        's2: {
            let sv8: i32 = F::BtlAnim_GetId(c, p_chr_1);
            match sv8 {
                28i32 | 40i32 => {
                    if (((((((rt::flt(F32(0x3dcccccc), l_yaw_5) as i32)) != 0) && (((rt::flt(l_yaw_5, F32(0x3f199999)) as i32)) != 0)) as i32)) != 0) {
                        l_speed_6 = F::BtlMoveParam_GetSpeed(c, p_chr_1, 11i32);
                    } else {
                        l_speed_6 = F32(0x00000000);
                    }
                    break 's2;
                }
                29i32 | 30i32 | 41i32 | 42i32 => {
                    if (((((((rt::flt(F32(0x3dcccccc), l_yaw_5) as i32)) != 0) && (((rt::flt(l_yaw_5, F32(0x3f199999)) as i32)) != 0)) as i32)) != 0) {
                        l_yaw_5 = F::BtlMoveParam_GetSpeed(c, p_chr_1, 12i32);
                        l_speed_6 = F::BtlAnim_GetLength(c, p_chr_1);
                        { let r_ = F::BtlAnim_GetStep(c, p_chr_1); let n_ = rt::fdiv(l_speed_6, r_); l_speed_6 = n_; };
                        l_speed_6 = F::BtlAct_ScaleSpeedByDist(c, p_chr_1, l_yaw_5, F32(0x3fc90fda), rt::fmul(l_speed_6, F32(0x3effffff)));
                    } else {
                        l_speed_6 = F32(0x00000000);
                    }
                    break 's2;
                }
                _ => {
                }
            }
        }
        F::BtlMove_Step(c, p_chr_1, 2i32, 5i32, 6i32, l_speed_6, F32(0x3f6d0979));
        F::BtlMove_ApplyGravity(c, p_chr_1);
        F::BtlChar_SetFlag(c, p_chr_1, ((((136i32) as i64 as u64)) as u32));
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((17i32) as i64 as u64)) as u32))) != 0) {
            F::BtlChar_SetHeldFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32));
        }
        if (({ let a_ = (F::BtlMember_GetActiveGauge(c, p_chr_1)).wrapping_add(40); (c.m.r32(a_) as i32) }) != 0) {
            F::BtlChar_SetFlag(c, p_chr_1, ((((150i32) as i64 as u64)) as u32));
        }
        F::BtlMove_RequestOrbit(c, p_chr_1, F32(0x40a00000), F32(0x41200000));
    }
    if (((((p_phase_2) == (2i32)) as i32)) != 0) {
        if (((((F::BtlAnim_GetId(c, p_chr_1)) == (28i32)) as i32)) != 0) {
            l_yaw_5 = F::BtlParam_GetUnk6C(c, p_chr_1);
        } else {
            l_yaw_5 = F::BtlParam_GetUnk68(c, p_chr_1);
        }
        if (((c.m.r32(((p_chr_1).wrapping_add(976)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) != 0) {
            F::BtlAct_Request(c, p_chr_1, 11i32);
            l_anim_4 = 1i32;
            if ((F::BtlInput_IsHeld(c, p_chr_1, ((32i32) as u32))) != 0) {
                l_anim_4 = 17i32;
            }
            { let _ = F::BtlDecide_Main(c, p_chr_1, l_anim_4); };
            { let a1_ = F::BtlAct_GetQueued(c, p_chr_1); F::BtlAct_Request(c, p_chr_1, a1_) };
        }
        if ((((((((!((F::BtlAct_HasQueued(c, p_chr_1)) != 0)) as i32)) != 0) && (((rt::fgt(F::BtlAnim_GetProgress(c, p_chr_1), F32(0x3ecccccc)) as i32)) != 0)) as i32)) != 0) {
            { let _ = F::BtlDecide_Main(c, p_chr_1, 81920i32); };
            { let _ = F::BtlDecide_Attack(c, p_chr_1, 131i32); };
        }
        if (((rt::flt(l_yaw_5, F::BtlAnim_GetProgress(c, p_chr_1)) as i32)) != 0) {
            { let a1_ = F::BtlAct_GetQueued(c, p_chr_1); F::BtlAct_Request(c, p_chr_1, a1_) };
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_StepHandler", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_StepInHandler(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_StepInHandler", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let mut l_speed_3: F32 = F32(0);
    let mut l_accel_4: F32 = F32(0);
    let r_ = 'f: {
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        F::BtlAnim_Play(c, p_chr_1, 27i32, F32(0x3e199999));
        F::BtlChar_SetHeldFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32));
        F::BtlCharSnd_PlayCommon(c, p_chr_1, 31i32);
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        if ((F::BtlAnim_Advance(c, p_chr_1, 0i32)) != 0) {
            F::BtlAct_Request(c, p_chr_1, 11i32);
        }
        if (((rt::flt(F::BtlAnim_GetProgress(c, p_chr_1), F32(0x3e999999)) as i32)) != 0) {
            F::BtlChar_SetFlag(c, p_chr_1, ((((66i32) as i64 as u64)) as u32));
        }
        if (((rt::flt(F::BtlAnim_GetProgress(c, p_chr_1), F32(0x3f333333)) as i32)) != 0) {
            l_speed_3 = F::BtlMoveParam_GetSpeed(c, p_chr_1, 10i32);
            l_accel_4 = F32(0x461c4000);
        } else {
            l_speed_3 = F32(0x00000000);
            l_accel_4 = F32(0x3f6d0979);
        }
        F::BtlMove_Step(c, p_chr_1, 2i32, 1i32, 3i32, l_speed_3, l_accel_4);
        F::BtlMove_ApplyGravity(c, p_chr_1);
        F::BtlChar_SetFlag(c, p_chr_1, ((((136i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((213i32) as i64 as u64)) as u32));
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((17i32) as i64 as u64)) as u32))) != 0) {
            F::BtlChar_SetHeldFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32));
        }
        if (({ let a_ = (F::BtlMember_GetActiveGauge(c, p_chr_1)).wrapping_add(40); (c.m.r32(a_) as i32) }) != 0) {
            F::BtlChar_SetFlag(c, p_chr_1, ((((150i32) as i64 as u64)) as u32));
        }
    }
    if (((((p_phase_2) == (2i32)) as i32)) != 0) {
        's1: {
            let sv5: i32 = F::BtlAct_GetCurrent(c, p_chr_1);
            match sv5 {
                30i32 => {
                    { let _ = F::BtlDecide_Attack(c, p_chr_1, 29362179i32); };
                    if (((rt::fgt(F::BtlAnim_GetProgress(c, p_chr_1), F32(0x3f000000)) as i32)) != 0) {
                        { let a1_ = F::BtlAct_GetQueued(c, p_chr_1); F::BtlAct_Request(c, p_chr_1, a1_) };
                    }
                    if ((F::BtlDecide_Attack(c, p_chr_1, 64i32)) != 0) {
                        { let a1_ = F::BtlAct_GetQueued(c, p_chr_1); F::BtlAct_Request(c, p_chr_1, a1_) };
                    }
                    break 's1;
                }
                31i32 => {
                    { let _ = F::BtlDecide_Attack(c, p_chr_1, 33554480i32); };
                    { let _ = F::BtlDecide_QueueAttack(c, p_chr_1, 143i32); };
                    { let _ = F::BtlDecide_QueueAttack(c, p_chr_1, 141i32); };
                    { let _ = F::BtlDecide_QueueAttack(c, p_chr_1, 142i32); };
                    if (((rt::fgt(F::BtlAnim_GetProgress(c, p_chr_1), F32(0x3e999999)) as i32)) != 0) {
                        { let a1_ = F::BtlAct_GetQueued(c, p_chr_1); F::BtlAct_Request(c, p_chr_1, a1_) };
                    }
                    break 's1;
                }
                _ => {
                }
            }
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_StepInHandler", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_VanishStepHandler(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_VanishStepHandler", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let mut l_counter_3: u32 = 0;
    let mut l_pose_4: u32 = 0;
    let mut l_yaw_5: F32 = F32(0);
    let mut l_c_6: F32 = F32(0);
    let mut l_sn_7: F32 = F32(0);
    let r_ = 'f: {
        l_counter_3 = ((p_chr_1).wrapping_add(976)).wrapping_add((((2i32) as u32)).wrapping_mul(4));
        l_pose_4 = F::BtlChar_GetPos(c, p_chr_1);
        if (((((p_phase_2) == (0i32)) as i32)) != 0) {
            F::BtlAnim_Play(c, p_chr_1, 246i32, F32(0x3e199999));
            if (((((F::BtlAct_GetPrev(c, p_chr_1)) == (68i32)) as i32)) != 0) {
                F::BtlChar_SetHeldFlag(c, p_chr_1, ((((131i32) as i64 as u64)) as u32));
                { let _ = { let a1_ = F::BtlParam_GetAmountA(c, p_chr_1); F::BtlMember_SpendKi(c, p_chr_1, a1_, 0i32) }; };
            }
            if ((((((((((F::BtlChar_TestFlag(c, p_chr_1, ((((116i32) as i64 as u64)) as u32))) != 0) || ((F::BtlChar_TestFlag(c, p_chr_1, ((((117i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) || ((F::BtlChar_TestFlag(c, p_chr_1, ((((118i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) {
                { let r_ = 1i32; let a_ = ((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                F::BtlChar_SetFlag(c, p_chr_1, ((((300i32) as i64 as u64)) as u32));
                { let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlEvent_Raise(c, a0_, 65i32) };
                F::BtlChar_SetFrameBits(c, p_chr_1, ((32768i32) as u64));
                F::BtlChar_AddStageTimer(c, F32(0x40000000));
            }
            { let v_ = 7i32; let a_ = l_counter_3; c.m.w32(a_, (v_) as u32); };
            F::BtlChar_SetFxBit(c, p_chr_1, 12i32);
            F::BtlChar_SetHeldFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32));
            F::BtlCharSnd_PlayCommon(c, p_chr_1, 32i32);
            { let v_ = F32(0x00000000); let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(152); c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x00000000); let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(156); c.m.w32(a_, (v_).0); };
            's1: {
                let sv8: i32 = F::BtlAct_GetCurrent(c, p_chr_1);
                match sv8 {
                    33i32 | 34i32 => {
                        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((19i32) as i64 as u64)) as u32))) != 0) {
                            F::BtlOpp_GetTargetRot(c, p_chr_1, fp);
                            if (((rt::fgt({ let a0_ = { let a0_ = rt::fsub(F32(c.m.r32((l_pose_4).wrapping_add(148))), F32(c.m.r32((fp).wrapping_add(4)))); F::BtlUtil_WrapAngle(c, a0_) }; rt::ffabs(a0_) }, F32(0x3fc90fda)) as i32)) != 0) {
                                F::BtlChar_PlayVoice(c, p_chr_1, 29i32);
                            }
                        }
                        break 's1;
                    }
                    _ => {
                    }
                }
            }
            if ((F::BtlChar_TestFlag(c, p_chr_1, ((((120i32) as i64 as u64)) as u32))) != 0) {
                { let r_ = 2i32; let a_ = ((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            }
        }
        if (((((p_phase_2) == (1i32)) as i32)) != 0) {
            if ((F::BtlAnim_Advance(c, p_chr_1, 0i32)) != 0) {
                { let r_ = 1i32; let a_ = ((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            }
            if ((((!(((((c.m.r32(((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) & (1i32))) != 0)) as i32)) != 0) {
                F::BtlMove_Step(c, p_chr_1, 2i32, 5i32, 7i32, F32(0x00000000), F32(0x3f6d0979));
                F::BtlMove_ApplyGravity(c, p_chr_1);
            } else {
                if ((((((c.m.r32(l_counter_3) as i32)) == (7i32)) as i32)) != 0) {
                    F::Vec4_SetZero(c, fp);
                    F::BtlOpp_GetDelta(c, p_chr_1, fp.wrapping_add(16));
                    F::Vec3_Normalize(c, fp.wrapping_add(16), fp.wrapping_add(16));
                    l_yaw_5 = rt::fneg(F::BtlOpp_GetYaw(c, p_chr_1));
                    's2: {
                        let sv9: i32 = F::BtlAct_GetCurrent(c, p_chr_1);
                        match sv9 {
                            33i32 => {
                                { let v_ = F32(0xbf800000); let a_ = fp; c.m.w32(a_, (v_).0); };
                                break 's2;
                            }
                            34i32 => {
                                { let v_ = F32(0x3f800000); let a_ = fp; c.m.w32(a_, (v_).0); };
                                break 's2;
                            }
                            35i32 => {
                                { let v_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))); let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
                                { let v_ = { let a0_ = rt::fsub(F32(0x3f800000), rt::fmul(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))), F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))))); F::Port_sqrtf(c, a0_) }; let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_).0); };
                                break 's2;
                            }
                            32i32 => {
                                { let v_ = F32(0xbf800000); let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_).0); };
                                break 's2;
                            }
                            _ => {
                            }
                        }
                    }
                    F::Vec3_Normalize(c, fp, fp);
                    l_c_6 = F::Mathf_Cos(c, l_yaw_5);
                    l_sn_7 = F::Mathf_Sin(c, l_yaw_5);
                    { let v_ = rt::fsub(rt::fmul(F32(c.m.r32(fp)), l_c_6), rt::fmul(F32(c.m.r32((fp).wrapping_add(8))), l_sn_7)); let a_ = (l_pose_4).wrapping_add(128); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(c.m.r32((fp).wrapping_add(4))); let a_ = ((l_pose_4).wrapping_add(128)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fadd(rt::fmul(F32(c.m.r32(fp)), l_sn_7), rt::fmul(F32(c.m.r32((fp).wrapping_add(8))), l_c_6)); let a_ = ((l_pose_4).wrapping_add(128)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                }
                { let a_ = l_counter_3; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                if ((((((c.m.r32(l_counter_3) as i32)) >= (2i32)) as i32)) != 0) {
                    l_c_6 = { let a1_ = F::BtlMoveParam_GetSpeed(c, p_chr_1, 18i32); F::BtlAct_ScaleSpeedByDist(c, p_chr_1, a1_, F32(0x40490fda), F32(0x40c00000)) };
                } else {
                    if ((((((c.m.r32(l_counter_3) as i32)) == (1i32)) as i32)) != 0) {
                        l_c_6 = F32(0x00000000);
                        F::BtlChar_SetFxBit(c, p_chr_1, 13i32);
                    } else {
                        l_c_6 = F32(0x00000000);
                    }
                }
                F::BtlMove_Step(c, p_chr_1, 2i32, 5i32, 7i32, l_c_6, F32(0x461c4000));
                F::BtlMove_ApplyGravity(c, p_chr_1);
                if ((((!(((((c.m.r32(((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) & (2i32))) != 0)) as i32)) != 0) {
                    if ((F::BtlChar_TestFlag(c, p_chr_1, ((((5i32) as i64 as u64)) as u32))) != 0) {
                        F::BtlOpp_GetVelocity(c, p_chr_1, fp.wrapping_add(32));
                        F::Vec4_Add(c, l_pose_4, l_pose_4, fp.wrapping_add(32));
                    }
                }
                F::BtlChar_SetFlag(c, p_chr_1, ((((11i32) as i64 as u64)) as u32));
            }
            F::BtlMove_RequestOrbit(c, p_chr_1, F32(0x40a00000), F32(0x41200000));
            F::BtlChar_SetFlag(c, p_chr_1, ((((136i32) as i64 as u64)) as u32));
            F::BtlChar_SetFxBit(c, p_chr_1, 58i32);
            F::BtlChar_SetFlag(c, p_chr_1, ((((66i32) as i64 as u64)) as u32));
            F::BtlChar_SetFlag(c, p_chr_1, ((((67i32) as i64 as u64)) as u32));
            F::BtlChar_SetFlag(c, p_chr_1, ((((68i32) as i64 as u64)) as u32));
            F::BtlChar_SetFlag(c, p_chr_1, ((((69i32) as i64 as u64)) as u32));
            F::BtlChar_SetFlag(c, p_chr_1, ((((70i32) as i64 as u64)) as u32));
            if ((F::BtlChar_TestFlag(c, p_chr_1, ((((131i32) as i64 as u64)) as u32))) != 0) {
                F::BtlChar_SetFlag(c, p_chr_1, ((((133i32) as i64 as u64)) as u32));
            }
        }
        if (((((p_phase_2) == (2i32)) as i32)) != 0) {
            if ((((!((F::BtlAct_HasQueued(c, p_chr_1)) != 0)) as i32)) != 0) {
                { let _ = F::BtlDecide_Attack(c, p_chr_1, 3i32); };
            }
            if ((((((c.m.r32(l_counter_3) as i32)) <= (0i32)) as i32)) != 0) {
                F::BtlAct_Request(c, p_chr_1, 11i32);
                { let a1_ = F::BtlAct_GetQueued(c, p_chr_1); F::BtlAct_Request(c, p_chr_1, a1_) };
            }
        }
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_VanishStepHandler", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_VanishBehindHandler(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_VanishBehindHandler", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let mut l_counter_3: u32 = 0;
    let r_ = 'f: {
    l_counter_3 = ((p_chr_1).wrapping_add(976)).wrapping_add((((2i32) as u32)).wrapping_mul(4));
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        F::BtlAnim_Play(c, p_chr_1, 246i32, F32(0x3e199999));
        { let _ = { let a1_ = F::BtlParam_GetAmountB(c, p_chr_1); F::BtlMember_SpendKi(c, p_chr_1, a1_, 0i32) }; };
        F::BtlChar_SetFxBit(c, p_chr_1, 12i32);
        F::BtlChar_SetHeldFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32));
        F::BtlCharSnd_PlayCommon(c, p_chr_1, 32i32);
        F::BtlChar_PlayVoice(c, p_chr_1, 29i32);
        { let v_ = F32(0x00000000); let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(152); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(156); c.m.w32(a_, (v_).0); };
        { let v_ = 7i32; let a_ = l_counter_3; c.m.w32(a_, (v_) as u32); };
        F::BtlChar_SetSmallVibration(c, p_chr_1, F32(0x3e4ccccc));
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        if ((((!((F::BtlAnim_Advance(c, p_chr_1, 0i32)) != 0)) as i32)) != 0) {
            F::BtlMove_Step(c, p_chr_1, 2i32, 5i32, 7i32, F32(0x00000000), F32(0x3f6d0979));
            F::BtlMove_ApplyGravity(c, p_chr_1);
        } else {
            { let a_ = l_counter_3; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
            if ((((((c.m.r32(l_counter_3) as i32)) >= (2i32)) as i32)) != 0) {
                { let v_ = F32(0x00000000); let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(152); c.m.w32(a_, (v_).0); };
                { let v_ = F32(0x00000000); let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(156); c.m.w32(a_, (v_).0); };
            } else {
                if ((((((c.m.r32(l_counter_3) as i32)) == (1i32)) as i32)) != 0) {
                    F::BtlChar_SetFxBit(c, p_chr_1, 13i32);
                    if ((((!((F::BtlChar_TestFlag(c, p_chr_1, ((((186i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) {
                        F::BtlMove_WarpBehindOpponent(c, p_chr_1, F32(0x00000000));
                        F::BtlMove_TurnYaw(c, p_chr_1, 2i32, F32(0x40490fda));
                        F::BtlMove_TurnPitch(c, p_chr_1, 1i32, F32(0x40490fda));
                        F::BtlMove_TurnModelYaw(c, p_chr_1, F32(0x40490fda), F32(0x3f800000));
                        F::BtlMove_SetDirection(c, p_chr_1, 3i32);
                        F::BtlChar_SetFlag(c, p_chr_1, ((((63i32) as i64 as u64)) as u32));
                        F::BtlChar_SetFlag(c, p_chr_1, ((((11i32) as i64 as u64)) as u32));
                        F::BtlChar_SetFlag(c, p_chr_1, ((((204i32) as i64 as u64)) as u32));
                        F::BtlChar_SetFlag(c, p_chr_1, ((((36i32) as i64 as u64)) as u32));
                    }
                }
            }
            F::BtlChar_SetFlag(c, p_chr_1, ((((11i32) as i64 as u64)) as u32));
        }
        F::BtlChar_SetFlag(c, p_chr_1, ((((66i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((67i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((68i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((69i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((70i32) as i64 as u64)) as u32));
    }
    if (((((p_phase_2) == (2i32)) as i32)) != 0) {
        if ((((!((F::BtlAct_HasQueued(c, p_chr_1)) != 0)) as i32)) != 0) {
            { let _ = F::BtlDecide_Attack(c, p_chr_1, 3i32); };
        }
        if ((((((c.m.r32(l_counter_3) as i32)) <= (0i32)) as i32)) != 0) {
            F::BtlAct_Request(c, p_chr_1, 11i32);
            { let a1_ = F::BtlAct_GetQueued(c, p_chr_1); F::BtlAct_Request(c, p_chr_1, a1_) };
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_VanishBehindHandler", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_RecoverHandler(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_RecoverHandler", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let mut l_pose_3: u32 = 0;
    let mut l_anim_4: i32 = 0;
    let r_ = 'f: {
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        l_pose_3 = F::BtlChar_GetPos(c, p_chr_1);
        l_anim_4 = 254i32;
        if ((F::BtlAct_IsAirMotion(c, p_chr_1, 0i32)) != 0) {
            { let v_ = { let a0_ = rt::fadd(F32(c.m.r32((l_pose_3).wrapping_add(148))), F32(0x40490fda)); F::BtlUtil_WrapAngle(c, a0_) }; let a_ = (l_pose_3).wrapping_add(148); c.m.w32(a_, (v_).0); };
        }
        's1: {
            let sv5: i32 = F::BtlAct_GetCurrent(c, p_chr_1);
            match sv5 {
                37i32 | 38i32 => {
                    l_anim_4 = 254i32;
                    break 's1;
                }
                39i32 => {
                    l_anim_4 = 255i32;
                    break 's1;
                }
                41i32 => {
                    l_anim_4 = 256i32;
                    { let v_ = F32(0x00000000); let a_ = (l_pose_3).wrapping_add(152); c.m.w32(a_, (v_).0); };
                    { let v_ = { let v_ = { let a0_ = rt::fadd(F32(c.m.r32((l_pose_3).wrapping_add(148))), F32(0x40490fda)); F::BtlUtil_WrapAngle(c, a0_) }; let a_ = (l_pose_3).wrapping_add(148); c.m.w32(a_, (v_).0); v_ }; let a_ = ((l_pose_3).wrapping_add(16)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    break 's1;
                }
                40i32 => {
                    l_anim_4 = 257i32;
                    break 's1;
                }
                42i32 => {
                    l_anim_4 = 258i32;
                    { let v_ = F32(0x00000000); let a_ = (l_pose_3).wrapping_add(152); c.m.w32(a_, (v_).0); };
                    { let v_ = { let v_ = { let a0_ = rt::fadd(F32(c.m.r32((l_pose_3).wrapping_add(148))), F32(0x40490fda)); F::BtlUtil_WrapAngle(c, a0_) }; let a_ = (l_pose_3).wrapping_add(148); c.m.w32(a_, (v_).0); v_ }; let a_ = ((l_pose_3).wrapping_add(16)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    break 's1;
                }
                _ => {
                }
            }
        }
        F::BtlAnim_Play(c, p_chr_1, l_anim_4, F32(0x00000000));
        F::BtlChar_SetHeldFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32));
        { let _ = { let a1_ = F::BtlParam_GetRecoverKiCost(c, p_chr_1); F::BtlMember_SpendKi(c, p_chr_1, a1_, 0i32) }; };
        F::BtlCharSnd_PlayCommon(c, p_chr_1, 33i32);
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        's2: {
            let sv6: i32 = F::BtlAct_GetCurrent(c, p_chr_1);
            match sv6 {
                38i32 => {
                    { let _ = F::BtlAnim_Advance(c, p_chr_1, 0i32); };
                    if (((rt::fgt(F::BtlAnim_GetProgress(c, p_chr_1), F32(0x3ecccccc)) as i32)) != 0) {
                        F::BtlAct_Request(c, p_chr_1, 15i32);
                    }
                    break 's2;
                }
                37i32 | 39i32 | 41i32 => {
                    if ((F::BtlAnim_Advance(c, p_chr_1, 0i32)) != 0) {
                        F::BtlAct_Request(c, p_chr_1, 11i32);
                    }
                    break 's2;
                }
                40i32 | 42i32 => {
                    if ((F::BtlAnim_Advance(c, p_chr_1, 0i32)) != 0) {
                        F::BtlAct_Request(c, p_chr_1, 25i32);
                    }
                    break 's2;
                }
                _ => {
                }
            }
        }
        F::BtlMove_Step(c, p_chr_1, 6i32, 6i32, 7i32, F32(0x00000000), F32(0x3f6d0979));
        F::BtlMove_ApplyGravity(c, p_chr_1);
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((17i32) as i64 as u64)) as u32))) != 0) {
            F::BtlChar_SetHeldFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32));
        }
    }
    if (((((p_phase_2) == (3i32)) as i32)) != 0) {
        F::BtlChar_SetFlag(c, p_chr_1, ((((51i32) as i64 as u64)) as u32));
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_RecoverHandler", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_PlayAttackPart(c: &mut Ctx, mut p_chr_1: u32, mut p_atk_2: u32, mut p_frame_3: i32, mut p_time_4: F32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_PlayAttackPart", &PROBE_N, &[(p_chr_1) as u32, (p_atk_2) as u32, (p_frame_3) as u32, (p_time_4).0], false); }
    let mut l_obj_5: u32 = 0;
    let mut l_n_6: i32 = 0;
    let r_ = 'f: {
    l_obj_5 = F::BtlChar_GetObj(c, p_chr_1);
    if (((((p_frame_3) < (((((c.m.r8((p_atk_2).wrapping_add(8)) as i8)) as i32)).wrapping_sub(1i32))) as i32)) != 0) {
        { let a1_ = (((c.m.r16(((p_atk_2).wrapping_add(4)).wrapping_add((((p_frame_3) as u32)).wrapping_mul(2))) as i16)) as i32); F::BtlAnim_PlaySub(c, p_chr_1, a1_) };
        { let r_ = rt::fdiv(rt::fdiv(rt::fmul(F32(c.m.r32((l_obj_5).wrapping_add(3036))), F32(0x41f00000)), F32(0x42700000)), F32(c.m.r32((p_atk_2).wrapping_add(16)))); let n_ = rt::fadd(p_time_4, r_); p_time_4 = n_; };
    } else {
        if (((((p_frame_3) == (((((c.m.r8((p_atk_2).wrapping_add(8)) as i8)) as i32)).wrapping_sub(1i32))) as i32)) != 0) {
            { let a1_ = (((c.m.r16(((p_atk_2).wrapping_add(4)).wrapping_add((((p_frame_3) as u32)).wrapping_mul(2))) as i16)) as i32); F::BtlAnim_PlaySub(c, p_chr_1, a1_) };
            l_n_6 = F::BtlObjAnim_QueryEvent(c, l_obj_5, ((1i32) as u64), 1i32, ((((0i32) as i64 as u64)) as u32));
            { let r_ = F::BtlObjAnim_QueryEvent(c, l_obj_5, ((2i32) as u64), 1i32, ((((0i32) as i64 as u64)) as u32)); let n_ = (l_n_6).wrapping_add(r_); l_n_6 = n_; };
            { let r_ = rt::fdiv(rt::fmul(rt::fmul(rt::i2f(l_n_6), F32(0x3f000000)), F32(0x41f00000)), F32(0x42700000)); let n_ = rt::fadd(p_time_4, r_); p_time_4 = n_; };
            p_time_4 = F::Port_floorf(c, rt::fadd(p_time_4, F32(0x3f000000)));
        }
    }
    break 'f p_time_4;
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_PlayAttackPart", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_VanishAttackHandler(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_VanishAttackHandler", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let mut l_timer_3: u32 = 0;
    let mut l_step_4: u32 = 0;
    let r_ = 'f: {
    l_timer_3 = ((p_chr_1).wrapping_add(976)).wrapping_add((((3i32) as u32)).wrapping_mul(4));
    l_step_4 = ((p_chr_1).wrapping_add(976)).wrapping_add((((2i32) as u32)).wrapping_mul(4));
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        F::BtlChar_SetFxBit(c, p_chr_1, 12i32);
        F::BtlCharSnd_PlayCommon(c, p_chr_1, 32i32);
        F::BtlChar_SetSmallVibration(c, p_chr_1, F32(0x3e4ccccc));
        F::BtlChar_SetHeldFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32));
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        { let _ = F::BtlAnim_AdvanceThen(c, p_chr_1, 38i32, F32(0x3e199999), 0i32); };
        { let v_ = { let a2_ = (c.m.r32((p_chr_1).wrapping_add(2404)) as i32); let a3_ = F32(c.m.r32(((p_chr_1).wrapping_add(3356)).wrapping_add(24))); F::BtlAct_PlayAttackPart(c, p_chr_1, (p_chr_1).wrapping_add(3356), a2_, a3_) }; let a_ = ((p_chr_1).wrapping_add(3356)).wrapping_add(24); c.m.w32(a_, (v_).0); };
        's1: {
            let sv5: i32 = (c.m.r32(l_step_4) as i32);
            match sv5 {
                0i32 => {
                    if ((((({ let a_ = l_timer_3; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); n_ }) >= (4i32)) as i32)) != 0) {
                        F::BtlChar_SetFlag(c, p_chr_1, ((((11i32) as i64 as u64)) as u32));
                    }
                    if ((((((c.m.r32(l_timer_3) as i32)) >= (6i32)) as i32)) != 0) {
                        { let v_ = 0i32; let a_ = l_timer_3; c.m.w32(a_, (v_) as u32); };
                        { let a_ = l_step_4; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    }
                    break 's1;
                }
                1i32 => {
                    F::BtlMove_SnapToOpponent(c, p_chr_1);
                    F::BtlChar_SetFlag(c, p_chr_1, ((((11i32) as i64 as u64)) as u32));
                    F::BtlChar_SetFlag(c, p_chr_1, ((((204i32) as i64 as u64)) as u32));
                    if ((F::BtlAct_HasQueued(c, p_chr_1)) != 0) {
                        { let a1_ = F::BtlAct_GetQueued(c, p_chr_1); F::BtlAct_Request(c, p_chr_1, a1_) };
                    } else {
                        F::BtlAct_Request(c, p_chr_1, 11i32);
                    }
                    break 's1;
                }
                _ => {
                }
            }
        }
    }
    if (((((p_phase_2) == (3i32)) as i32)) != 0) {
        F::BtlChar_SetHeldFlag(c, p_chr_1, ((((221i32) as i64 as u64)) as u32));
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_VanishAttackHandler", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_VanishAttackQuickHandler(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_VanishAttackQuickHandler", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let mut l_step_3: u32 = 0;
    let r_ = 'f: {
    l_step_3 = ((p_chr_1).wrapping_add(976)).wrapping_add((((2i32) as u32)).wrapping_mul(4));
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        F::BtlChar_SetFxBit(c, p_chr_1, 12i32);
        F::BtlCharSnd_PlayCommon(c, p_chr_1, 32i32);
        F::BtlChar_SetSmallVibration(c, p_chr_1, F32(0x3e4ccccc));
        F::BtlChar_SetHeldFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32));
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        { let _ = F::BtlAnim_AdvanceThen(c, p_chr_1, 38i32, F32(0x3e199999), 0i32); };
        { let v_ = { let a2_ = (c.m.r32((p_chr_1).wrapping_add(2404)) as i32); let a3_ = F32(c.m.r32(((p_chr_1).wrapping_add(3356)).wrapping_add(24))); F::BtlAct_PlayAttackPart(c, p_chr_1, (p_chr_1).wrapping_add(3356), a2_, a3_) }; let a_ = ((p_chr_1).wrapping_add(3356)).wrapping_add(24); c.m.w32(a_, (v_).0); };
        's1: {
            let sv4: i32 = (c.m.r32(l_step_3) as i32);
            match sv4 {
                0i32 => {
                    F::BtlChar_SetFlag(c, p_chr_1, ((((11i32) as i64 as u64)) as u32));
                    { let a_ = l_step_3; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    break 's1;
                }
                1i32 => {
                    F::BtlMove_SnapToOpponent(c, p_chr_1);
                    F::BtlChar_SetFlag(c, p_chr_1, ((((11i32) as i64 as u64)) as u32));
                    F::BtlChar_SetFlag(c, p_chr_1, ((((204i32) as i64 as u64)) as u32));
                    if ((F::BtlAct_HasQueued(c, p_chr_1)) != 0) {
                        { let a1_ = F::BtlAct_GetQueued(c, p_chr_1); F::BtlAct_Request(c, p_chr_1, a1_) };
                    } else {
                        F::BtlAct_Request(c, p_chr_1, 11i32);
                    }
                    break 's1;
                }
                _ => {
                }
            }
        }
    }
    if (((((p_phase_2) == (3i32)) as i32)) != 0) {
        F::BtlChar_SetHeldFlag(c, p_chr_1, ((((221i32) as i64 as u64)) as u32));
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_VanishAttackQuickHandler", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_WarpBehindAttackHandler(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_WarpBehindAttackHandler", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let r_ = 'f: {
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        { let v_ = F32(0x00000000); let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(152); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(156); c.m.w32(a_, (v_).0); };
        if ((((!((F::BtlChar_TestFlag(c, p_chr_1, ((((186i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) {
            F::BtlMove_WarpBehindOpponent(c, p_chr_1, F32(0x00000000));
            F::BtlMove_TurnYaw(c, p_chr_1, 2i32, F32(0x40490fda));
            F::BtlMove_TurnPitch(c, p_chr_1, 1i32, F32(0x40490fda));
            F::BtlMove_TurnModelYaw(c, p_chr_1, F32(0x40490fda), F32(0x3f800000));
            F::BtlMove_SetDirection(c, p_chr_1, 3i32);
            F::BtlChar_SetFlag(c, p_chr_1, ((((63i32) as i64 as u64)) as u32));
            F::BtlChar_SetFlag(c, p_chr_1, ((((11i32) as i64 as u64)) as u32));
            F::BtlChar_SetFlag(c, p_chr_1, ((((204i32) as i64 as u64)) as u32));
            F::BtlChar_SetFlag(c, p_chr_1, ((((36i32) as i64 as u64)) as u32));
        }
        if ((F::BtlAct_HasQueued(c, p_chr_1)) != 0) {
            { let a1_ = F::BtlAct_GetQueued(c, p_chr_1); F::BtlAct_Request(c, p_chr_1, a1_) };
        } else {
            F::BtlAct_Request(c, p_chr_1, 11i32);
        }
    }
    if (((((p_phase_2) == (3i32)) as i32)) != 0) {
        F::BtlChar_SetHeldFlag(c, p_chr_1, ((((221i32) as i64 as u64)) as u32));
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_WarpBehindAttackHandler", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_WarpAheadAttackHandler(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_WarpAheadAttackHandler", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let r_ = 'f: {
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        { let v_ = F32(0x00000000); let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(152); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(156); c.m.w32(a_, (v_).0); };
        if ((((!((F::BtlChar_TestFlag(c, p_chr_1, ((((186i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) {
            { let a1_ = F32(c.m.r32(((p_chr_1).wrapping_add(3356)).wrapping_add(24))); F::BtlMove_WarpAheadOfOpponent(c, p_chr_1, a1_) };
            F::BtlMove_TurnYaw(c, p_chr_1, 2i32, F32(0x40490fda));
            F::BtlMove_TurnPitch(c, p_chr_1, 1i32, F32(0x40490fda));
            F::BtlMove_TurnModelYaw(c, p_chr_1, F32(0x40490fda), F32(0x3f800000));
            F::BtlMove_SetDirection(c, p_chr_1, 3i32);
            F::BtlChar_SetFlag(c, p_chr_1, ((((63i32) as i64 as u64)) as u32));
            F::BtlChar_SetFlag(c, p_chr_1, ((((11i32) as i64 as u64)) as u32));
            F::BtlChar_SetFlag(c, p_chr_1, ((((204i32) as i64 as u64)) as u32));
            F::BtlChar_SetFlag(c, p_chr_1, ((((36i32) as i64 as u64)) as u32));
        }
        if ((F::BtlAct_HasQueued(c, p_chr_1)) != 0) {
            { let a1_ = F::BtlAct_GetQueued(c, p_chr_1); F::BtlAct_Request(c, p_chr_1, a1_) };
        } else {
            F::BtlAct_Request(c, p_chr_1, 11i32);
        }
    }
    if (((((p_phase_2) == (3i32)) as i32)) != 0) {
        F::BtlChar_SetHeldFlag(c, p_chr_1, ((((221i32) as i64 as u64)) as u32));
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_WarpAheadAttackHandler", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_SlideToAttackHandler(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_SlideToAttackHandler", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_step_3: u32 = 0;
    let mut l_counter_4: u32 = 0;
    let mut l_face_5: u32 = 0;
    let mut l_pose_6: u32 = 0;
    let mut l_dist_7: F32 = F32(0);
    let mut l_angle_8: F32 = F32(0);
    let mut l_extra_9: F32 = F32(0);
    let mut l_frames_10: i32 = 0;
    let r_ = 'f: {
        l_step_3 = ((p_chr_1).wrapping_add(976)).wrapping_add((((12i32) as u32)).wrapping_mul(4));
        l_counter_4 = ((p_chr_1).wrapping_add(976)).wrapping_add((((2i32) as u32)).wrapping_mul(4));
        l_face_5 = ((p_chr_1).wrapping_add(976)).wrapping_add((((16i32) as u32)).wrapping_mul(4));
        l_pose_6 = F::BtlChar_GetPos(c, p_chr_1);
        if (((((p_phase_2) == (0i32)) as i32)) != 0) {
            l_dist_7 = F32(0x00000000);
            l_angle_8 = F32(0x00000000);
            l_extra_9 = F32(0x00000000);
            l_frames_10 = 4i32;
            's1: {
                let sv11: i32 = F::BtlAct_GetCurrent(c, p_chr_1);
                match sv11 {
                    47i32 => {
                        l_angle_8 = F32(0xbf1c61a9);
                        l_dist_7 = F32(0x41300000);
                        l_extra_9 = F32(0x3f800000);
                        l_frames_10 = 4i32;
                        break 's1;
                    }
                    48i32 => {
                        l_angle_8 = F32(0xbf9c61a9);
                        l_dist_7 = F32(0x41000000);
                        break 's1;
                    }
                    _ => {
                    }
                }
            }
            { let a3_ = F32(c.m.r32(((p_chr_1).wrapping_add(3356)).wrapping_add(28))); F::BtlMove_CalcApproachPoint(c, p_chr_1, fp, fp.wrapping_add(16), a3_, l_angle_8, l_dist_7, rt::fadd(rt::fadd(l_dist_7, rt::i2f(l_frames_10)), l_extra_9)) };
            F::Vec4_Sub(c, l_step_3, fp, l_pose_6);
            F::Vec4_Scale(c, l_step_3, l_step_3, F32(0x3e800000));
            F::Vec4_Copy(c, l_face_5, fp.wrapping_add(16));
        }
        if (((((p_phase_2) == (1i32)) as i32)) != 0) {
            { let v_ = F32(0x00000000); let a_ = (l_pose_6).wrapping_add(152); c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x00000000); let a_ = (l_pose_6).wrapping_add(156); c.m.w32(a_, (v_).0); };
            F::Vec4_Add(c, l_pose_6, l_pose_6, l_step_3);
            if ((((({ let a_ = l_counter_4; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); n_ }) == (4i32)) as i32)) != 0) {
                if ((F::BtlAct_HasQueued(c, p_chr_1)) != 0) {
                    { let a1_ = F::BtlAct_GetQueued(c, p_chr_1); F::BtlAct_Request(c, p_chr_1, a1_) };
                } else {
                    F::BtlAct_Request(c, p_chr_1, 11i32);
                }
            }
            F::BtlChar_SetFlag(c, p_chr_1, ((((63i32) as i64 as u64)) as u32));
            F::BtlChar_SetFlag(c, p_chr_1, ((((11i32) as i64 as u64)) as u32));
            F::BtlChar_SetFlag(c, p_chr_1, ((((204i32) as i64 as u64)) as u32));
            F::BtlChar_SetFlag(c, p_chr_1, ((((36i32) as i64 as u64)) as u32));
        }
        if (((((p_phase_2) == (3i32)) as i32)) != 0) {
            F::BtlMove_TurnToPoint(c, p_chr_1, l_face_5, F32(0x40490fda), F32(0x40490fda));
            F::BtlChar_SetHeldFlag(c, p_chr_1, ((((221i32) as i64 as u64)) as u32));
        }
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_SlideToAttackHandler", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_RushToAttackHandler(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_RushToAttackHandler", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let mut l_lead_3: u32 = 0;
    let mut l_counter_4: u32 = 0;
    let mut l_reach_5: i32 = 0;
    let mut l_stuck_6: i32 = 0;
    let mut l_speed_7: F32 = F32(0);
    let mut l_accel_8: F32 = F32(0);
    let r_ = 'f: {
    l_lead_3 = ((p_chr_1).wrapping_add(976)).wrapping_add((((6i32) as u32)).wrapping_mul(4));
    l_counter_4 = ((p_chr_1).wrapping_add(976)).wrapping_add((((2i32) as u32)).wrapping_mul(4));
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        F::BtlAnim_Play(c, p_chr_1, 13i32, F32(0x3e199999));
        F::BtlChar_SetHeldFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((209i32) as i64 as u64)) as u32));
        { let a1_ = F::BtlParam_GetDashSound(c, p_chr_1); F::BtlCharSnd_PlayCommon(c, p_chr_1, a1_) };
        F::BtlChar_SetVibration(c, p_chr_1, F32(0x3f4ccccc), F32(0x3e4ccccc));
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        l_speed_7 = F32(0x00000000);
        { let v_ = { let a2_ = (c.m.r32((p_chr_1).wrapping_add(2404)) as i32); let a3_ = F32(c.m.r32(l_lead_3)); F::BtlAct_PlayAttackPart(c, p_chr_1, (p_chr_1).wrapping_add(3356), a2_, a3_) }; let a_ = l_lead_3; c.m.w32(a_, (v_).0); };
        l_accel_8 = F32(0x3f6d0979);
        's1: {
            let sv9: i32 = F::BtlAnim_GetId(c, p_chr_1);
            match sv9 {
                13i32 => {
                    F::BtlAnim_AdvanceLoop(c, p_chr_1, 0i32);
                    l_speed_7 = F32(c.m.r32(((p_chr_1).wrapping_add(3356)).wrapping_add(28)));
                    l_accel_8 = F32(0x461c4000);
                    F::BtlChar_SetFxBit(c, p_chr_1, 4i32);
                    if ((F::BtlChar_TestFlag(c, p_chr_1, ((((15i32) as i64 as u64)) as u32))) != 0) {
                        F::BtlChar_SetFxBit(c, p_chr_1, 48i32);
                    }
                    F::BtlChar_SetFlag(c, p_chr_1, ((((9i32) as i64 as u64)) as u32));
                    F::BtlChar_SetFlag(c, p_chr_1, ((((80i32) as i64 as u64)) as u32));
                    F::BtlChar_SetFlag(c, p_chr_1, ((((81i32) as i64 as u64)) as u32));
                    F::BtlChar_SetFxBit(c, p_chr_1, 9i32);
                    break 's1;
                }
                14i32 => {
                    if ((F::BtlAnim_Advance(c, p_chr_1, 0i32)) != 0) {
                        F::BtlAct_Request(c, p_chr_1, 11i32);
                    }
                    break 's1;
                }
                _ => {
                }
            }
        }
        F::BtlMove_Step(c, p_chr_1, 2i32, 1i32, 3i32, l_speed_7, l_accel_8);
        F::BtlMove_ApplyGravity(c, p_chr_1);
    }
    if (((((p_phase_2) == (2i32)) as i32)) != 0) {
        if (((((F::BtlAnim_GetId(c, p_chr_1)) == (13i32)) as i32)) != 0) {
            l_reach_5 = 0i32;
            l_stuck_6 = 0i32;
            if (((rt::flt(F::BtlOpp_GetClosingTime(c, p_chr_1), F32(c.m.r32(l_lead_3))) as i32)) != 0) {
                l_reach_5 = 1i32;
            }
            if ((F::BtlMove_IsBlockedByOpponent(c, p_chr_1)) != 0) {
                l_reach_5 = 1i32;
            }
            if (((rt::flt({ let a0_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(64); F::Vec3_Length(c, a0_) }, F32(0x3f6d0979)) as i32)) != 0) {
                if ((((({ let a_ = l_counter_4; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); n_ }) >= (8i32)) as i32)) != 0) {
                    l_stuck_6 = 1i32;
                }
            } else {
                { let v_ = 0i32; let a_ = l_counter_4; c.m.w32(a_, (v_) as u32); };
            }
            if ((((!((F::BtlChar_TestFlag(c, p_chr_1, ((((5i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) {
                l_reach_5 = 1i32;
            }
            if ((((((l_reach_5) != 0) || ((l_stuck_6) != 0)) as i32)) != 0) {
                { let v_ = F32(0x00000000); let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(152); c.m.w32(a_, (v_).0); };
                if ((((((F::BtlAct_HasQueued(c, p_chr_1)) != 0) && ((((!((l_stuck_6) != 0)) as i32)) != 0)) as i32)) != 0) {
                    { let a1_ = F::BtlAct_GetQueued(c, p_chr_1); F::BtlAct_Request(c, p_chr_1, a1_) };
                } else {
                    F::BtlAnim_Request(c, p_chr_1, 14i32, F32(0x00000000));
                }
            }
        }
    }
    if (((((p_phase_2) == (3i32)) as i32)) != 0) {
        F::BtlChar_SetHeldFlag(c, p_chr_1, ((((221i32) as i64 as u64)) as u32));
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_RushToAttackHandler", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_HopBackAttackHandler(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_HopBackAttackHandler", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let mut l_speed_3: F32 = F32(0);
    let r_ = 'f: {
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        F::BtlAnim_Play(c, p_chr_1, 40i32, F32(0x3e199999));
        F::BtlChar_SetHeldFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32));
        F::BtlCharSnd_PlayCommon(c, p_chr_1, 31i32);
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        l_speed_3 = F::BtlMoveParam_GetSpeed(c, p_chr_1, 11i32);
        { let _ = F::BtlAnim_Advance(c, p_chr_1, 0i32); };
        if (((rt::fgt(F::BtlAnim_GetProgress(c, p_chr_1), F32(0x3f000000)) as i32)) != 0) {
            if ((F::BtlAct_HasQueued(c, p_chr_1)) != 0) {
                { let a1_ = F::BtlAct_GetQueued(c, p_chr_1); F::BtlAct_Request(c, p_chr_1, a1_) };
            } else {
                F::BtlAct_Request(c, p_chr_1, 11i32);
            }
        }
        F::BtlMove_Step(c, p_chr_1, 2i32, 5i32, 4i32, l_speed_3, F32(0x3f6d0979));
        F::BtlMove_ApplyGravity(c, p_chr_1);
    }
    if (((((p_phase_2) == (3i32)) as i32)) != 0) {
        F::BtlChar_SetHeldFlag(c, p_chr_1, ((((221i32) as i64 as u64)) as u32));
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_HopBackAttackHandler", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_CircleDashHandler(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_CircleDashHandler", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let mut l_counter_3: u32 = 0;
    let mut l_baseYaw_4: u32 = 0;
    let mut l_flag_5: i32 = 0;
    let mut l_lean_6: F32 = F32(0);
    let mut l_speed_7: F32 = F32(0);
    let mut l_pose_8: u32 = 0;
    let r_ = 'f: {
    l_counter_3 = ((p_chr_1).wrapping_add(976)).wrapping_add((((2i32) as u32)).wrapping_mul(4));
    l_baseYaw_4 = ((p_chr_1).wrapping_add(976)).wrapping_add((((6i32) as u32)).wrapping_mul(4));
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        l_pose_8 = F::BtlChar_GetPos(c, p_chr_1);
        F::BtlAnim_Play(c, p_chr_1, 45i32, F32(0x3e199999));
        F::BtlChar_SetHeldFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32));
        F::BtlChar_PlayVoice(c, p_chr_1, 5i32);
        F::BtlChar_SetFxBit(c, p_chr_1, 6i32);
        { let v_ = F32(0x00000000); let a_ = (l_pose_8).wrapping_add(152); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (l_pose_8).wrapping_add(156); c.m.w32(a_, (v_).0); };
        if (((rt::flt(F32(0x00000000), F::BtlOpp_GetYawFromItsFacing(c, p_chr_1)) as i32)) != 0) {
            { let r_ = 1i32; let a_ = ((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        }
        { let v_ = { let a0_ = rt::fadd(F32(c.m.r32(((l_pose_8).wrapping_add(16)).wrapping_add(4))), F32(0x40490fda)); F::BtlUtil_WrapAngle(c, a0_) }; let a_ = l_baseYaw_4; c.m.w32(a_, (v_).0); };
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        l_flag_5 = 0i32;
        l_lean_6 = F32(0x00000000);
        's1: {
            let sv9: i32 = F::BtlAnim_GetId(c, p_chr_1);
            match sv9 {
                45i32 => {
                    { let _ = F::BtlAnim_AdvanceThen(c, p_chr_1, 25i32, F32(0x3e199999), 0i32); };
                    l_lean_6 = F32(0x3f800000);
                    break 's1;
                }
                25i32 => {
                    F::BtlAnim_AdvanceLoop(c, p_chr_1, 0i32);
                    l_lean_6 = F32(0x3f800000);
                    l_flag_5 = 1i32;
                    if ((F::BtlAnim_IsNew(c, p_chr_1)) != 0) {
                        { let a1_ = F::BtlParam_GetDashSound(c, p_chr_1); F::BtlCharSnd_PlayCommon(c, p_chr_1, a1_) };
                        F::BtlChar_Vibrate(c, p_chr_1, l_lean_6, F32(0x3e999999));
                        F::ChrCam_RequestCut(c, p_chr_1, 1i32, 4i32);
                    }
                    break 's1;
                }
                26i32 => {
                    F::BtlAnim_SetDuration(c, p_chr_1, F32(0x3e999999));
                    if ((F::BtlAnim_Advance(c, p_chr_1, 0i32)) != 0) {
                        F::BtlAct_Request(c, p_chr_1, 11i32);
                    }
                    l_lean_6 = rt::fsub(F32(0x3f800000), F::BtlAnim_GetProgress(c, p_chr_1));
                    break 's1;
                }
                _ => {
                }
            }
        }
        if ((l_flag_5) != 0) {
            l_speed_7 = F::BtlMoveParam_GetSpeed(c, p_chr_1, 19i32);
            if (({ let a1_ = (((c.m.r32(((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) & (1i32)); let a2_ = (((F::BtlOpp_GetSeenAction(c, p_chr_1)) == (213i32)) as i32); let a3_ = F32(c.m.r32(l_baseYaw_4)); F::BtlMove_CircleOpponent(c, p_chr_1, a1_, a2_, a3_, l_speed_7) }) != 0) {
                { let r_ = 2i32; let a_ = ((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            }
            F::BtlChar_SetFlag(c, p_chr_1, ((((9i32) as i64 as u64)) as u32));
            F::BtlChar_SetFlag(c, p_chr_1, ((((80i32) as i64 as u64)) as u32));
            F::BtlChar_SetFlag(c, p_chr_1, ((((81i32) as i64 as u64)) as u32));
            F::BtlChar_SetFxBit(c, p_chr_1, 4i32);
            F::BtlChar_SetFlag(c, p_chr_1, ((((71i32) as i64 as u64)) as u32));
        } else {
            { let a1_ = (if (((((c.m.r32(((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) & (2i32))) != 0) { 2i32 } else { 6i32 }); F::BtlMove_Step(c, p_chr_1, a1_, 6i32, 7i32, F32(0x00000000), F32(0x3f6d0979)) };
            F::BtlMove_ApplyGravity(c, p_chr_1);
        }
        { let a1_ = rt::fmul({ let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(144); F32(c.m.r32(a_)) }, l_lean_6); F::BtlMove_SetLeanX(c, p_chr_1, a1_) };
        F::BtlChar_SetFlag(c, p_chr_1, ((((137i32) as i64 as u64)) as u32));
    }
    if (((((p_phase_2) == (2i32)) as i32)) != 0) {
        l_flag_5 = 0i32;
        's2: {
            let sv10: i32 = F::BtlAnim_GetId(c, p_chr_1);
            match sv10 {
                45i32 => {
                    if ((F::BtlMove_IsBlockedByOpponent(c, p_chr_1)) != 0) {
                        l_flag_5 = 1i32;
                        { let v_ = F32(0x00000000); let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(152); c.m.w32(a_, (v_).0); };
                    }
                    break 's2;
                }
                25i32 => {
                    if ((F::BtlMove_IsBlockedByOpponent(c, p_chr_1)) != 0) {
                        l_flag_5 = 1i32;
                        { let v_ = F32(0x00000000); let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(152); c.m.w32(a_, (v_).0); };
                    }
                    if (((((c.m.r32(((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) & (2i32))) != 0) {
                        l_flag_5 = 1i32;
                        { let v_ = F32(0x00000000); let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(152); c.m.w32(a_, (v_).0); };
                    }
                    if (({ let a1_ = F::BtlMoveParam_GetKiCost(c, p_chr_1, 3i32); F::BtlMember_SpendKi(c, p_chr_1, a1_, 0i32) }) != 0) {
                        l_flag_5 = 1i32;
                        { let r_ = F32(0x3e4ccccc); let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(152); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    }
                    if ((((!((F::BtlChar_TestFlag(c, p_chr_1, ((((5i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) {
                        l_flag_5 = 1i32;
                    }
                    if (((rt::flt({ let a0_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(64); F::Vec3_Length(c, a0_) }, F32(0x411425ec)) as i32)) != 0) {
                        if ((((({ let a_ = l_counter_3; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); n_ }) >= (6i32)) as i32)) != 0) {
                            l_flag_5 = 1i32;
                            { let v_ = F32(0x00000000); let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(152); c.m.w32(a_, (v_).0); };
                        }
                    } else {
                        { let v_ = 0i32; let a_ = l_counter_3; c.m.w32(a_, (v_) as u32); };
                    }
                    break 's2;
                }
                26i32 => {
                    break 's2;
                }
                _ => {
                }
            }
        }
        if ((l_flag_5) != 0) {
            F::BtlAnim_Request(c, p_chr_1, 26i32, F32(0x3e199999));
        }
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((19i32) as i64 as u64)) as u32))) != 0) {
            l_speed_7 = F32(0x3f490fda);
            if (((((F::BtlAnim_GetId(c, p_chr_1)) == (26i32)) as i32)) != 0) {
                l_speed_7 = F32(0x3fc90fda);
            }
            if (((rt::flt({ let a0_ = { let a0_ = rt::fsub(F::BtlOpp_GetYaw(c, p_chr_1), F32(c.m.r32(l_baseYaw_4))); F::BtlUtil_WrapAngle(c, a0_) }; rt::ffabs(a0_) }, l_speed_7) as i32)) != 0) {
                if ((F::BtlDecide_QueueAttack(c, p_chr_1, 144i32)) != 0) {
                    { let a1_ = F::BtlAct_GetQueued(c, p_chr_1); F::BtlAct_Request(c, p_chr_1, a1_) };
                }
                if ((F::BtlDecide_Attack(c, p_chr_1, 67108864i32)) != 0) {
                    { let a1_ = F::BtlAct_GetQueued(c, p_chr_1); F::BtlAct_Request(c, p_chr_1, a1_) };
                }
            }
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_CircleDashHandler", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_RushDashHandler(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_RushDashHandler", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let mut l_fast_3: i32 = 0;
    let mut l_yawMode_4: i32 = 0;
    let mut l_accel_5: F32 = F32(0);
    let mut l_pitchMode_6: i32 = 0;
    let mut l_speed_7: F32 = F32(0);
    let r_ = 'f: {
    l_fast_3 = (((((((F::BtlParam_GetFlags(c, p_chr_1)) as u64)) as i32)).wrapping_shr((20i32) as u32)) & (1i32));
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        F::BtlAnim_Play(c, p_chr_1, 35i32, F32(0x3e199999));
        F::BtlChar_SetHeldFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32));
        if ((l_fast_3) != 0) {
            F::BtlChar_SetFxBit(c, p_chr_1, 14i32);
            F::BtlCharSnd_PlayCommon(c, p_chr_1, 32i32);
        } else {
            F::BtlCharSnd_PlayCommon(c, p_chr_1, 49i32);
        }
        F::BtlChar_SetSmallVibration(c, p_chr_1, F32(0x3e4ccccc));
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        l_yawMode_4 = 2i32;
        l_accel_5 = F32(0x461c4000);
        l_pitchMode_6 = 1i32;
        if ((l_fast_3) != 0) {
            l_speed_7 = F32(0x00000000);
            F::BtlChar_SetFlag(c, p_chr_1, ((((11i32) as i64 as u64)) as u32));
            F::BtlChar_SetFlag(c, p_chr_1, ((((66i32) as i64 as u64)) as u32));
            F::BtlChar_SetFlag(c, p_chr_1, ((((67i32) as i64 as u64)) as u32));
            F::BtlChar_SetFlag(c, p_chr_1, ((((68i32) as i64 as u64)) as u32));
            F::BtlChar_SetFlag(c, p_chr_1, ((((69i32) as i64 as u64)) as u32));
            F::BtlChar_SetFlag(c, p_chr_1, ((((70i32) as i64 as u64)) as u32));
            if ((((((c.m.r32((p_chr_1).wrapping_add(2404)) as i32)) != (0i32)) as i32)) != 0) {
                l_speed_7 = F32(0x41a2f683);
            }
        } else {
            l_speed_7 = F32(0x41855554);
        }
        's1: {
            let sv8: i32 = F::BtlAnim_GetId(c, p_chr_1);
            match sv8 {
                35i32 => {
                    { let _ = F::BtlAnim_AdvanceThen(c, p_chr_1, 36i32, F32(0x3e199999), 0i32); };
                    break 's1;
                }
                36i32 => {
                    F::BtlAnim_AdvanceLoop(c, p_chr_1, 0i32);
                    break 's1;
                }
                37i32 => {
                    F::BtlAnim_SetDuration(c, p_chr_1, F32(0x3dcccccc));
                    if ((F::BtlAnim_Advance(c, p_chr_1, 0i32)) != 0) {
                        F::BtlAct_Request(c, p_chr_1, 11i32);
                        { let v_ = F32(0x00000000); let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(152); c.m.w32(a_, (v_).0); };
                        { let v_ = F32(0x00000000); let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(156); c.m.w32(a_, (v_).0); };
                    }
                    l_speed_7 = F32(0x00000000);
                    l_yawMode_4 = 6i32;
                    l_accel_5 = F32(0x3f6d0979);
                    l_pitchMode_6 = 6i32;
                    break 's1;
                }
                _ => {
                }
            }
        }
        F::BtlMove_Step(c, p_chr_1, l_yawMode_4, l_pitchMode_6, 3i32, l_speed_7, l_accel_5);
        F::BtlMove_ApplyGravity(c, p_chr_1);
        F::BtlChar_SetFxBit(c, p_chr_1, 9i32);
        F::BtlChar_SetFxBit(c, p_chr_1, 58i32);
    }
    if (((((p_phase_2) == (2i32)) as i32)) != 0) {
        if (((((F::BtlAnim_GetId(c, p_chr_1)) == (36i32)) as i32)) != 0) {
            if ((F::BtlMove_IsBlockedByOpponent(c, p_chr_1)) != 0) {
                F::BtlAnim_Request(c, p_chr_1, 37i32, F32(0x00000000));
            }
        }
        if ((F::BtlDecide_QueueAttack(c, p_chr_1, 156i32)) != 0) {
            { let a1_ = F::BtlAct_GetQueued(c, p_chr_1); F::BtlAct_Request(c, p_chr_1, a1_) };
        }
        if ((F::BtlInput_TestAction(c, p_chr_1, 52i32, 1i32)) != 0) {
            F::BtlAct_Request(c, p_chr_1, 70i32);
        }
        if ((F::BtlInput_TestAction(c, p_chr_1, 93i32, 1i32)) != 0) {
            F::BtlAct_Request(c, p_chr_1, 181i32);
        }
    }
    if (((((p_phase_2) == (3i32)) as i32)) != 0) {
        if ((l_fast_3) != 0) {
            F::BtlChar_SetFxBit(c, p_chr_1, 13i32);
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_RushDashHandler", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_VanishDashHandler(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_VanishDashHandler", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let mut l_flag_3: i32 = 0;
    let r_ = 'f: {
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        F::BtlAnim_Play(c, p_chr_1, 12i32, F32(0x3e199999));
        F::BtlChar_SetFxBit(c, p_chr_1, 14i32);
        F::BtlChar_SetHeldFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32));
        F::BtlCharSnd_PlayCommon(c, p_chr_1, 32i32);
        F::BtlMove_TurnYaw(c, p_chr_1, 0i32, F32(0x40490fda));
        F::BtlMove_TurnPitch(c, p_chr_1, 2i32, F32(0x40490fda));
        F::BtlMove_TurnModelYaw(c, p_chr_1, F32(0x40490fda), F32(0x3f800000));
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        l_flag_3 = 1i32;
        's1: {
            let sv4: i32 = F::BtlAnim_GetId(c, p_chr_1);
            match sv4 {
                12i32 => {
                    F::BtlAnim_SetDuration(c, p_chr_1, F32(0x3d4ccccc));
                    { let _ = F::BtlAnim_AdvanceThen(c, p_chr_1, 13i32, F32(0x3e199999), 0i32); };
                    if ((F::BtlAnim_IsNew(c, p_chr_1)) != 0) {
                        l_flag_3 = 0i32;
                    }
                    break 's1;
                }
                13i32 => {
                    F::BtlAnim_AdvanceLoop(c, p_chr_1, 0i32);
                    F::BtlChar_SetFlag(c, p_chr_1, ((((11i32) as i64 as u64)) as u32));
                    break 's1;
                }
                14i32 => {
                    if ((F::BtlAnim_Advance(c, p_chr_1, 0i32)) != 0) {
                        F::BtlAct_Request(c, p_chr_1, 11i32);
                    }
                    if ((F::BtlAnim_IsNew(c, p_chr_1)) != 0) {
                        F::BtlChar_SetFxBit(c, p_chr_1, 13i32);
                        { let v_ = F32(0x411425ec); let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(152); c.m.w32(a_, (v_).0); };
                    }
                    if ((F::BtlMove_IsBlockedByOpponent(c, p_chr_1)) != 0) {
                        { let v_ = F32(0x00000000); let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(152); c.m.w32(a_, (v_).0); };
                    }
                    l_flag_3 = 0i32;
                    break 's1;
                }
                _ => {
                }
            }
        }
        if ((l_flag_3) != 0) {
            F::BtlMove_Step(c, p_chr_1, 6i32, 6i32, 3i32, F32(0x419425ec), F32(0x461c4000));
            F::BtlChar_SetFlag(c, p_chr_1, ((((9i32) as i64 as u64)) as u32));
            F::BtlChar_SetFlag(c, p_chr_1, ((((66i32) as i64 as u64)) as u32));
            F::BtlChar_SetFlag(c, p_chr_1, ((((67i32) as i64 as u64)) as u32));
            F::BtlChar_SetFlag(c, p_chr_1, ((((68i32) as i64 as u64)) as u32));
            F::BtlChar_SetFlag(c, p_chr_1, ((((69i32) as i64 as u64)) as u32));
            F::BtlChar_SetFlag(c, p_chr_1, ((((70i32) as i64 as u64)) as u32));
            F::BtlChar_SetFlag(c, p_chr_1, ((((71i32) as i64 as u64)) as u32));
        } else {
            F::BtlMove_Step(c, p_chr_1, 6i32, 6i32, 3i32, F32(0x00000000), F32(0x3f6d0979));
        }
        F::BtlMove_ApplyGravity(c, p_chr_1);
        F::BtlMove_RequestOrbit(c, p_chr_1, F32(0x42480000), F32(0x42c80000));
        F::BtlChar_SetFlag(c, p_chr_1, ((((188i32) as i64 as u64)) as u32));
    }
    if (((((p_phase_2) == (2i32)) as i32)) != 0) {
        's2: {
            let sv5: i32 = F::BtlAnim_GetId(c, p_chr_1);
            match sv5 {
                12i32 | 13i32 => {
                    l_flag_3 = 1i32;
                    if ((((((c.m.r32((p_chr_1).wrapping_add(2404)) as i32)) < (13i32)) as i32)) != 0) {
                        l_flag_3 = 0i32;
                    }
                    if ((F::BtlMove_IsBlockedByOpponent(c, p_chr_1)) != 0) {
                        if (((rt::flt({ let a0_ = F::BtlOpp_GetYawFromFacing(c, p_chr_1); rt::ffabs(a0_) }, F32(0x3f490fda)) as i32)) != 0) {
                            l_flag_3 = 1i32;
                        }
                    }
                    if ((l_flag_3) != 0) {
                        F::BtlAnim_Request(c, p_chr_1, 14i32, F32(0x3e199999));
                    }
                    break 's2;
                }
                14i32 => {
                    { let _ = F::BtlDecide_Attack(c, p_chr_1, 3i32); };
                    { let a1_ = F::BtlAct_GetQueued(c, p_chr_1); F::BtlAct_Request(c, p_chr_1, a1_) };
                    break 's2;
                }
                _ => {
                }
            }
        }
        if ((((((c.m.r32((p_chr_1).wrapping_add(2404)) as i32)) >= (7i32)) as i32)) != 0) {
            { let _ = F::BtlDecide_Main(c, p_chr_1, 536870912i32); };
            { let a1_ = F::BtlAct_GetQueued(c, p_chr_1); F::BtlAct_Request(c, p_chr_1, a1_) };
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_VanishDashHandler", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_ClashBlowsHandler(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_ClashBlowsHandler", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let mut l_bonus_3: u32 = 0;
    let r_ = 'f: {
    l_bonus_3 = ((p_chr_1).wrapping_add(976)).wrapping_add((((2i32) as u32)).wrapping_mul(4));
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        { let a1_ = (if ((((((c.m.r32(p_chr_1) as i32)) == (0i32)) as i32)) != 0) { 371i32 } else { 372i32 }); F::BtlAnim_Play(c, p_chr_1, a1_, F32(0x3e199999)) };
        { let v_ = 0i32; let a_ = (p_chr_1).wrapping_add(3664); c.m.w32(a_, (v_) as u32); };
        if (({ let a0_ = F::BtlAct_GetPrev(c, p_chr_1); F::BtlAct_IsTechniqueId(c, a0_) }) != 0) {
            { let r_ = 1i32; let a_ = ((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            's1: {
                let sv4: i32 = F::BtlAct_GetPrevClass(c, p_chr_1);
                match sv4 {
                    2i32 => {
                        { let v_ = rt::div_i32(F::BtlSuper_GetKiCost(c, p_chr_1, 2i32), 2i32); let a_ = l_bonus_3; c.m.w32(a_, (v_) as u32); };
                        break 's1;
                    }
                    3i32 => {
                        { let v_ = rt::div_i32(F::BtlSuper_GetKiCost(c, p_chr_1, 3i32), 2i32); let a_ = l_bonus_3; c.m.w32(a_, (v_) as u32); };
                        break 's1;
                    }
                    4i32 => {
                        { let v_ = 100000i32; let a_ = l_bonus_3; c.m.w32(a_, (v_) as u32); };
                        break 's1;
                    }
                    _ => {
                    }
                }
            }
        }
        F::BtlChar_ClearFlagRange(c, p_chr_1, ((191i32) as u32), ((195i32) as u32));
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        's2: {
            let sv5: i32 = F::BtlAnim_GetId(c, p_chr_1);
            match sv5 {
                371i32 | 372i32 => {
                    F::BtlAnim_AdvanceLoop(c, p_chr_1, 0i32);
                    F::BtlChar_SetFlag(c, p_chr_1, ((((230i32) as i64 as u64)) as u32));
                    F::BtlChar_SetFlag(c, p_chr_1, ((((237i32) as i64 as u64)) as u32));
                    if ((F::BtlInput_TestAction(c, p_chr_1, 51i32, 1i32)) != 0) {
                        F::BtlAct_CountAndMarkOpponent(c, p_chr_1);
                    } else {
                        if ((F::BtlMember_HasAbility(c, p_chr_1, 106i32)) != 0) {
                            if (((((rt::rem_i32((c.m.r32((p_chr_1).wrapping_add(2404)) as i32), 28i32)) == (0i32)) as i32)) != 0) {
                                F::BtlAct_CountAndMarkOpponent(c, p_chr_1);
                            }
                        } else {
                            if ((F::BtlMember_HasAbility(c, p_chr_1, 105i32)) != 0) {
                                if (((((rt::rem_i32((c.m.r32((p_chr_1).wrapping_add(2404)) as i32), 8i32)) == (0i32)) as i32)) != 0) {
                                    F::BtlAct_CountAndMarkOpponent(c, p_chr_1);
                                }
                            } else {
                                if ((F::BtlMember_HasAbility(c, p_chr_1, 104i32)) != 0) {
                                    if (((((rt::rem_i32((c.m.r32((p_chr_1).wrapping_add(2404)) as i32), 5i32)) == (0i32)) as i32)) != 0) {
                                        F::BtlAct_CountAndMarkOpponent(c, p_chr_1);
                                    }
                                } else {
                                    if (((((rt::rem_i32((c.m.r32((p_chr_1).wrapping_add(2404)) as i32), 4i32)) == (0i32)) as i32)) != 0) {
                                        F::BtlAct_CountAndMarkOpponent(c, p_chr_1);
                                    }
                                }
                            }
                        }
                    }
                    if (((((c.m.r32(((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) & (1i32))) != 0) {
                        if (((((rt::rem_i32((c.m.r32((p_chr_1).wrapping_add(2404)) as i32), 15i32)) == (0i32)) as i32)) != 0) {
                            F::BtlAct_CountAndMarkOpponent(c, p_chr_1);
                        }
                    }
                    if ((F::BtlMember_HasAbility(c, p_chr_1, 20i32)) != 0) {
                        if (((((rt::rem_i32((c.m.r32((p_chr_1).wrapping_add(2404)) as i32), 5i32)) == (0i32)) as i32)) != 0) {
                            F::BtlAct_CountAndMarkOpponent(c, p_chr_1);
                        }
                    } else {
                        if ((F::BtlMember_HasAbility(c, p_chr_1, 19i32)) != 0) {
                            if (((((rt::rem_i32((c.m.r32((p_chr_1).wrapping_add(2404)) as i32), 11i32)) == (0i32)) as i32)) != 0) {
                                F::BtlAct_CountAndMarkOpponent(c, p_chr_1);
                            }
                        } else {
                            if ((F::BtlMember_HasAbility(c, p_chr_1, 18i32)) != 0) {
                                if (((((rt::rem_i32((c.m.r32((p_chr_1).wrapping_add(2404)) as i32), 17i32)) == (0i32)) as i32)) != 0) {
                                    F::BtlAct_CountAndMarkOpponent(c, p_chr_1);
                                }
                            }
                        }
                    }
                    { let v_ = 30i32; let a_ = (p_chr_1).wrapping_add(3400); c.m.w32(a_, (v_) as u32); };
                    if ((F::BtlChar_TestFlag(c, p_chr_1, ((((191i32) as i64 as u64)) as u32))) != 0) {
                        F::BtlAnim_Request(c, p_chr_1, 373i32, F32(0x3e199999));
                        F::BtlAct_CountAndMarkOpponent(c, p_chr_1);
                        { let v_ = 0i32; let a_ = (p_chr_1).wrapping_add(3400); c.m.w32(a_, (v_) as u32); };
                        if ((((((c.m.r32(l_bonus_3) as i32)) > (0i32)) as i32)) != 0) {
                            { let _ = { let a1_ = (c.m.r32(l_bonus_3) as i32); F::BtlMember_AddKi(c, p_chr_1, a1_) }; };
                            if ((F::BtlChar_TestFlag(c, p_chr_1, ((((6i32) as i64 as u64)) as u32))) != 0) {
                                F::BtlMember_AddMaxPower(c, p_chr_1, 30000i32);
                            }
                        }
                    }
                    if ((F::BtlChar_TestFlag(c, p_chr_1, ((((192i32) as i64 as u64)) as u32))) != 0) {
                        F::BtlAnim_Request(c, p_chr_1, 374i32, F32(0x3e199999));
                    }
                    break 's2;
                }
                373i32 => {
                    if ((F::BtlAnim_Advance(c, p_chr_1, 0i32)) != 0) {
                        F::BtlAct_Request(c, p_chr_1, 11i32);
                    }
                    break 's2;
                }
                374i32 => {
                    if ((F::BtlAnim_Advance(c, p_chr_1, 0i32)) != 0) {
                        F::BtlAct_Request(c, p_chr_1, 213i32);
                    }
                    if ((F::BtlAnim_PassedRatio(c, p_chr_1, F32(0x3f000000))) != 0) {
                        { let _ = { let a1_ = (F::BtlOpp_GetClashCountB(c, p_chr_1)).wrapping_mul(60i32); F::BtlMember_Damage(c, p_chr_1, a1_, 8i32) }; };
                        F::BtlChar_PlayVoice(c, p_chr_1, 26i32);
                    }
                    F::BtlChar_SetFlag(c, p_chr_1, ((((149i32) as i64 as u64)) as u32));
                    F::BtlChar_SetFlag(c, p_chr_1, ((((150i32) as i64 as u64)) as u32));
                    break 's2;
                }
                _ => {
                }
            }
        }
        F::BtlMove_SteerAtOpponent(c, p_chr_1, F32(0x3f6d0979), F32(0x00000000), F32(0x00000000), F32(0x00000000), F32(0x00000000), F32(0x3f490fda));
        F::BtlMove_TurnModelYaw(c, p_chr_1, F32(0x3f20d97a), F32(0x3e999999));
        { let v_ = { let a0_ = rt::fadd({ let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(148); F32(c.m.r32(a_)) }, F32(0x3f000000)); F::BtlUtil_WrapAngle(c, a0_) }; let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(148); c.m.w32(a_, (v_).0); };
        F::BtlMove_SetDirection(c, p_chr_1, 3i32);
        F::BtlMove_Advance(c, p_chr_1, F32(0x3f6d0979), F32(0x461c4000));
        F::BtlMove_ApplyGravity(c, p_chr_1);
        F::BtlChar_SetFlag(c, p_chr_1, ((((187i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((296i32) as i64 as u64)) as u32));
        F::BtlChar_Vibrate(c, p_chr_1, F32(0x3f800000), F32(0x3dcccccc));
        F::BtlChar_SetFlag(c, p_chr_1, ((((210i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((308i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((238i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((315i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((66i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((67i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((68i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((69i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((70i32) as i64 as u64)) as u32));
    }
    if (((((p_phase_2) == (3i32)) as i32)) != 0) {
        { let v_ = 0i32; let a_ = (p_chr_1).wrapping_add(3664); c.m.w32(a_, (v_) as u32); };
        F::BtlChar_ClearFlagRange(c, p_chr_1, ((191i32) as u32), ((195i32) as u32));
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_ClashBlowsHandler", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_ClashVanishHandler(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_ClashVanishHandler", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let mut l_counter_3: u32 = 0;
    let r_ = 'f: {
    l_counter_3 = ((p_chr_1).wrapping_add(976)).wrapping_add((((2i32) as u32)).wrapping_mul(4));
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        F::BtlAnim_Play(c, p_chr_1, 246i32, F32(0x3dcccccc));
        { let v_ = 0i32; let a_ = (p_chr_1).wrapping_add(3664); c.m.w32(a_, (v_) as u32); };
        F::BtlChar_SetFxBit(c, p_chr_1, 12i32);
        F::BtlCharSnd_PlayCommon(c, p_chr_1, 32i32);
        { let v_ = F32(0x00000000); let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(152); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(156); c.m.w32(a_, (v_).0); };
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        if ((F::BtlAnim_Advance(c, p_chr_1, 0i32)) != 0) {
            F::BtlChar_SetFlag(c, p_chr_1, ((((11i32) as i64 as u64)) as u32));
            if ((((({ let a_ = l_counter_3; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); n_ }) >= (7i32)) as i32)) != 0) {
                F::BtlAct_Request(c, p_chr_1, 252i32);
            }
        }
        F::BtlMove_Step(c, p_chr_1, 2i32, 5i32, 2i32, F32(0x00000000), F32(0x3f6d0979));
        F::BtlMove_ApplyGravity(c, p_chr_1);
        F::BtlChar_SetFlag(c, p_chr_1, ((((187i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((296i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((66i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((67i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((68i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((69i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((70i32) as i64 as u64)) as u32));
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_ClashVanishHandler", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_ClashExchangeHandler(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_ClashExchangeHandler", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let mut l_c_3: u32 = 0;
    let mut l_counter_4: u32 = 0;
    let mut l_wait_5: i32 = 0;
    let mut l_duration_6: F32 = F32(0);
    let mut l_speed_7: F32 = F32(0);
    let mut l_anim_8: i32 = 0;
    let r_ = 'f: {
    l_c_3 = (c.m.r32(sym::gBtlChars)).wrapping_add(124);
    l_counter_4 = ((p_chr_1).wrapping_add(976)).wrapping_add((((2i32) as u32)).wrapping_mul(4));
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        l_anim_8 = 0i32;
        if (((((F::BtlAct_GetPrev(c, p_chr_1)) != (252i32)) as i32)) != 0) {
            { let v_ = 0i32; let a_ = (p_chr_1).wrapping_add(3668); c.m.w32(a_, (v_) as u32); };
        } else {
            { let a_ = (p_chr_1).wrapping_add(3668); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        }
        's1: {
            let sv9: i32 = rt::rem_i32((c.m.r32((p_chr_1).wrapping_add(3668)) as i32), 4i32);
            match sv9 {
                0i32 => {
                    l_anim_8 = 76i32;
                    break 's1;
                }
                1i32 => {
                    l_anim_8 = 59i32;
                    break 's1;
                }
                2i32 => {
                    l_anim_8 = 79i32;
                    break 's1;
                }
                3i32 => {
                    l_anim_8 = 64i32;
                    break 's1;
                }
                _ => {
                }
            }
        }
        F::BtlAnim_Play(c, p_chr_1, l_anim_8, F32(0x3e199999));
        F::BtlAnim_EnableHandle(c, p_chr_1);
        { let v_ = F32(0x00000000); let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(152); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(156); c.m.w32(a_, (v_).0); };
        F::BtlChar_SetHeldFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32));
        F::BtlChar_RequestPlaceOnPath(c, p_chr_1);
        { let v_ = (1i32).wrapping_neg(); let a_ = (p_chr_1).wrapping_add(3672); c.m.w32(a_, (v_) as u32); };
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        l_duration_6 = F32(0x3ecccccc);
        l_speed_7 = F32(0x00000000);
        l_wait_5 = 15i32;
        's2: {
            let sv10: i32 = (c.m.r32(l_c_3) as i32);
            match sv10 {
                0i32 => {
                    l_duration_6 = F32(0x3f000000);
                    l_wait_5 = 18i32;
                    break 's2;
                }
                1i32 => {
                    break 's2;
                }
                2i32 => {
                    l_wait_5 = 12i32;
                    l_duration_6 = F32(0x3e999999);
                    break 's2;
                }
                _ => {
                }
            }
        }
        's3: {
            let sv11: i32 = F::BtlAnim_GetId(c, p_chr_1);
            match sv11 {
                59i32 | 64i32 | 76i32 | 79i32 => {
                    if ((((((c.m.r32(l_counter_4) as i32)) < (l_wait_5)) as i32)) != 0) {
                        { let a_ = l_counter_4; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        F::BtlChar_SetFlag(c, p_chr_1, ((((11i32) as i64 as u64)) as u32));
                    } else {
                        F::BtlAnim_SetDuration(c, p_chr_1, l_duration_6);
                        { let _ = F::BtlAnim_Advance(c, p_chr_1, 0i32); };
                        if (((rt::flt(F::BtlAnim_GetProgress(c, p_chr_1), F32(0x3dcccccc)) as i32)) != 0) {
                            F::BtlChar_SetFlag(c, p_chr_1, ((((11i32) as i64 as u64)) as u32));
                        } else {
                            l_speed_7 = F32(0x419425ec);
                        }
                        if ((F::BtlAnim_PassedRatio(c, p_chr_1, F32(0x3dcccccc))) != 0) {
                            F::BtlChar_SetFxBit(c, p_chr_1, 12i32);
                            F::BtlChar_Vibrate(c, p_chr_1, F32(0x3f800000), F32(0x3e999999));
                            if ((((((c.m.r32(p_chr_1) as i32)) == (0i32)) as i32)) != 0) {
                                F::BtlChar_SetFxBit(c, p_chr_1, 45i32);
                                F::BtlCharSnd_PlayCommon(c, p_chr_1, 79i32);
                                F::BtlCharSnd_PlayCommon(c, p_chr_1, 75i32);
                            }
                        }
                        if (((rt::fgt(F::BtlAnim_GetProgress(c, p_chr_1), F32(0x3f333333)) as i32)) != 0) {
                            F::BtlChar_SetFlag(c, p_chr_1, ((((11i32) as i64 as u64)) as u32));
                            F::BtlChar_SetFlag(c, p_chr_1, ((((196i32) as i64 as u64)) as u32));
                        }
                    }
                    if ((((((c.m.r32((p_chr_1).wrapping_add(2404)) as i32)) > (0i32)) as i32)) != 0) {
                        { let a1_ = ((c.m.r32((l_c_3).wrapping_add(20)) as i32)).wrapping_add(233i32); F::BtlChar_SetFlag(c, p_chr_1, ((((a1_) as i64 as u64)) as u32)) };
                        if ((((((c.m.r32((p_chr_1).wrapping_add(3672)) as i32)) == ((1i32).wrapping_neg())) as i32)) != 0) {
                            if ((F::BtlInput_IsPressed(c, p_chr_1, ((2013265920i32) as u32))) != 0) {
                                if (({ let a1_ = (((134217728i32).wrapping_shl(((c.m.r32((l_c_3).wrapping_add(20)) as i32)) as u32)) as u32); F::BtlInput_IsPressed(c, p_chr_1, a1_) }) != 0) {
                                    { let v_ = (c.m.r32((p_chr_1).wrapping_add(2404)) as i32); let a_ = (p_chr_1).wrapping_add(3672); c.m.w32(a_, (v_) as u32); };
                                    if ((((!(((c.m.r32((p_chr_1).wrapping_add(4728)) as i32)) != 0)) as i32)) != 0) {
                                        F::BtlCharSnd_PlayCommonFar(c, p_chr_1, 20i32);
                                    }
                                } else {
                                    if ((((((((((c.m.r32((l_c_3).wrapping_add(24)) as i32)) < (0i32)) as i32)) != 0) || ((((!(({ let a1_ = (((134217728i32).wrapping_shl(((c.m.r32((l_c_3).wrapping_add(24)) as i32)) as u32)) as u32); F::BtlInput_IsPressed(c, p_chr_1, a1_) }) != 0)) as i32)) != 0)) as i32)) != 0) {
                                        { let v_ = (2i32).wrapping_neg(); let a_ = (p_chr_1).wrapping_add(3672); c.m.w32(a_, (v_) as u32); };
                                    }
                                }
                            }
                        }
                    }
                    F::BtlChar_SetFlag(c, p_chr_1, ((((24i32) as i64 as u64)) as u32));
                    { let v_ = 30i32; let a_ = (p_chr_1).wrapping_add(3400); c.m.w32(a_, (v_) as u32); };
                    break 's3;
                }
                373i32 => {
                    if ((F::BtlAnim_Advance(c, p_chr_1, 0i32)) != 0) {
                        F::BtlAct_Request(c, p_chr_1, 11i32);
                    }
                    break 's3;
                }
                374i32 => {
                    if ((F::BtlAnim_Advance(c, p_chr_1, 0i32)) != 0) {
                        F::BtlAct_Request(c, p_chr_1, 213i32);
                    }
                    if ((F::BtlAnim_PassedRatio(c, p_chr_1, F32(0x3f000000))) != 0) {
                        { let _ = { let a1_ = (F::BtlOpp_GetClashCountB(c, p_chr_1)).wrapping_mul(600i32); F::BtlMember_Damage(c, p_chr_1, a1_, 8i32) }; };
                        F::BtlChar_PlayVoice(c, p_chr_1, 26i32);
                    }
                    F::BtlChar_SetFlag(c, p_chr_1, ((((149i32) as i64 as u64)) as u32));
                    F::BtlChar_SetFlag(c, p_chr_1, ((((150i32) as i64 as u64)) as u32));
                    break 's3;
                }
                _ => {
                }
            }
        }
        F::BtlMove_Step(c, p_chr_1, 2i32, 5i32, 2i32, l_speed_7, F32(0x461c4000));
        F::BtlMove_ApplyGravity(c, p_chr_1);
        F::BtlChar_SetFlag(c, p_chr_1, ((((187i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((296i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((308i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((238i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((66i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((67i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((68i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((69i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((70i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((145i32) as i64 as u64)) as u32));
    }
    if (((((p_phase_2) == (2i32)) as i32)) != 0) {
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((197i32) as i64 as u64)) as u32))) != 0) {
            F::BtlAct_Request(c, p_chr_1, 252i32);
        }
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((198i32) as i64 as u64)) as u32))) != 0) {
            F::BtlAct_Request(c, p_chr_1, 11i32);
        }
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((199i32) as i64 as u64)) as u32))) != 0) {
            F::BtlAnim_Request(c, p_chr_1, 373i32, F32(0x00000000));
            { let v_ = 0i32; let a_ = (p_chr_1).wrapping_add(3400); c.m.w32(a_, (v_) as u32); };
        }
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((200i32) as i64 as u64)) as u32))) != 0) {
            F::BtlAnim_Request(c, p_chr_1, 374i32, F32(0x00000000));
        }
    }
    if (((((p_phase_2) == (3i32)) as i32)) != 0) {
        if (((((F::BtlAct_GetRequested(c, p_chr_1)) != (252i32)) as i32)) != 0) {
            { let v_ = 0i32; let a_ = (p_chr_1).wrapping_add(3664); c.m.w32(a_, (v_) as u32); };
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_ClashExchangeHandler", &PROBE_N, &[], true); }
    r_
}
