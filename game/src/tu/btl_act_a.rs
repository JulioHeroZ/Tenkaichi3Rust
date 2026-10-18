#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn BtlAct_WaitHandler(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_WaitHandler", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let mut l_anim_3: i32 = 0;
    let mut l_air_4: i32 = 0;
    let mut l_turn_5: F32 = F32(0);
    let r_ = 'f: {
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        l_anim_3 = 0i32;
        l_air_4 = (rt::flt(F32(0x40a00000), F::BtlAct_GetHeight(c, p_chr_1)) as i32);
        's1: {
            let sv6: i32 = F::BtlAct_GetCurrent(c, p_chr_1);
            match sv6 {
                7i32 => {
                    l_anim_3 = (if ((l_air_4) != 0) { 38i32 } else { 0i32 });
                    break 's1;
                }
                8i32 => {
                    l_anim_3 = (if ((l_air_4) != 0) { 39i32 } else { 1i32 });
                    break 's1;
                }
                9i32 => {
                    l_anim_3 = 406i32;
                    break 's1;
                }
                10i32 => {
                    l_anim_3 = 379i32;
                    break 's1;
                }
                _ => {
                }
            }
        }
        l_turn_5 = F32(0x40490fda);
        F::BtlAnim_Play(c, p_chr_1, l_anim_3, F32(0x3e199999));
        F::BtlChar_SetHeldFlag(c, p_chr_1, ((5i32) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((41i32) as u32));
        { let v_ = F32(0x00000000); let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(152); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(156); c.m.w32(a_, (v_).0); };
        F::BtlMove_TurnYaw(c, p_chr_1, 2i32, l_turn_5);
        F::BtlMove_TurnPitch(c, p_chr_1, 5i32, l_turn_5);
        F::BtlMove_TurnModelYaw(c, p_chr_1, l_turn_5, F32(0x3f800000));
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        F::BtlAnim_AdvanceLoop(c, p_chr_1, 0i32);
        F::BtlMove_Step(c, p_chr_1, 6i32, 6i32, 7i32, F32(0x00000000), F32(0x3f6d0979));
        F::BtlMove_BrakeVertical(c, p_chr_1);
        F::BtlChar_SetFlag(c, p_chr_1, ((90i32) as u32));
        if (((rt::flt(F32(0x40a00000), F::BtlAct_GetHeight(c, p_chr_1)) as i32)) != 0) {
            F::BtlChar_SetFxBit(c, p_chr_1, 58i32);
        } else {
            F::BtlChar_SetFxBit(c, p_chr_1, 59i32);
        }
    }
    if (((((p_phase_2) == (2i32)) as i32)) != 0) {
        if ((F::BtlChar_IsFlagDropped(c, p_chr_1, ((175i32) as u32))) != 0) {
            F::BtlAct_Request(c, p_chr_1, 11i32);
        }
        if ((F::BtlChar_IsFlagDropped(c, p_chr_1, ((176i32) as u32))) != 0) {
            F::BtlAct_Request(c, p_chr_1, 11i32);
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_WaitHandler", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_GetChargeRate(c: &mut Ctx, mut p_chr_1: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_GetChargeRate", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_t_2: F32 = F32(0);
    let mut l_a_3: F32 = F32(0);
    let mut l_b_4: F32 = F32(0);
    let r_ = 'f: {
    l_t_2 = rt::fdiv(rt::i2f((c.m.r32((p_chr_1).wrapping_add(3456)) as i32)), F32(0x47c35000));
    l_a_3 = F::BtlParam_GetChargeRateA(c, p_chr_1);
    l_b_4 = F::BtlParam_GetChargeRateB(c, p_chr_1);
    break 'f F::BtlUtil_MaxF(c, rt::fadd(rt::fmul(rt::fsub(F32(0x3f800000), l_t_2), l_a_3), rt::fmul(l_t_2, l_b_4)), F32(0x3dcccccc));
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_GetChargeRate", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_StepCharge(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_StepCharge", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_rate_2: F32 = F32(0);
    let r_ = 'f: {
    l_rate_2 = F::BtlAct_GetChargeRate(c, p_chr_1);
    { let v_ = { let a0_ = rt::fadd(F32(c.m.r32((p_chr_1).wrapping_add(3452))), rt::fmul(l_rate_2, F32(0x3d360b60))); F::BtlUtil_MinF(c, a0_, F32(0x3f800000)) }; let a_ = (p_chr_1).wrapping_add(3452); c.m.w32(a_, (v_).0); };
    if (((rt::flt(F32(c.m.r32((p_chr_1).wrapping_add(3452))), F32(0x3f800000)) as i32)) != 0) {
        { let v_ = rt::fdiv(F32(c.m.r32((p_chr_1).wrapping_add(3452))), l_rate_2); let a_ = (p_chr_1).wrapping_add(3448); c.m.w32(a_, (v_).0); };
    } else {
        { let v_ = F32(0x3f800000); let a_ = (p_chr_1).wrapping_add(3448); c.m.w32(a_, (v_).0); };
    }
    break 'f (rt::fle(F32(0x3f800000), F32(c.m.r32((p_chr_1).wrapping_add(3448)))) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_StepCharge", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_SetChargeFromAnim(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_SetChargeFromAnim", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = F::BtlAnim_GetProgress(c, p_chr_1); let a_ = (p_chr_1).wrapping_add(3448); c.m.w32(a_, (v_).0); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_SetChargeFromAnim", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_SetChargeAnimDuration(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_SetChargeAnimDuration", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    { let a1_ = { let a0_ = rt::fdiv(F32(0x3fc00000), F::BtlAct_GetChargeRate(c, p_chr_1)); F::BtlUtil_MaxF(c, a0_, F32(0x3f400000)) }; F::BtlAnim_SetDuration(c, p_chr_1, a1_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_SetChargeAnimDuration", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_SetChargeAnimStep(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_SetChargeAnimStep", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_t_2: F32 = F32(0);
    let mut l_a_3: F32 = F32(0);
    let mut l_b_4: F32 = F32(0);
    let r_ = 'f: {
    l_t_2 = rt::fdiv(rt::i2f((c.m.r32((p_chr_1).wrapping_add(3456)) as i32)), F32(0x47c35000));
    l_a_3 = F::BtlParam_GetChargeRateA(c, p_chr_1);
    l_b_4 = F::BtlParam_GetChargeRateB(c, p_chr_1);
    F::BtlAnim_SetStep(c, p_chr_1, rt::fmul(rt::fadd(rt::fmul(rt::fsub(F32(0x3f800000), l_t_2), l_a_3), rt::fmul(l_t_2, l_b_4)), F32(0x40000000)));
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_SetChargeAnimStep", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_CancelOnOppC9(c: &mut Ctx, mut p_chr_1: u32, mut p_after_2: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_CancelOnOppC9", &PROBE_N, &[(p_chr_1) as u32, (p_after_2).0], false); }
    let r_ = 'f: {
    if ((((((F::BtlChar_TestFlag(c, p_chr_1, ((93i32) as u32))) != 0) && (((((F::BtlOpp_GetSeenAction(c, p_chr_1)) == (201i32)) as i32)) != 0)) as i32)) != 0) {
        if ((((!((F::BtlAct_HasQueued(c, p_chr_1)) != 0)) as i32)) != 0) {
            { let _ = F::BtlDecide_Attack(c, p_chr_1, ((((((1i32) as u32)) as u64)) as i32)); };
        }
        if (((rt::flt(p_after_2, F::BtlAnim_GetProgress(c, p_chr_1)) as i32)) != 0) {
            { let a1_ = F::BtlAct_GetQueued(c, p_chr_1); F::BtlAct_Request(c, p_chr_1, a1_) };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_CancelOnOppC9", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_PlayPickedVoice(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_PlayPickedVoice", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_kind_2: i32 = 0;
    let r_ = 'f: {
    l_kind_2 = F::BtlAtk_GetVoiceKind(c, p_chr_1);
    if (((((l_kind_2) >= (0i32)) as i32)) != 0) {
        F::BtlChar_PlayVoice(c, p_chr_1, l_kind_2);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_PlayPickedVoice", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_StartSub19B(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_StartSub19B", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_i_2: i32 = 0;
    let mut l_obj_3: u32 = 0;
    let mut l_mask_4: u32 = 0;
    let mut l_kind_5: i32 = 0;
    let r_ = 'f: {
    l_i_2 = 0i32;
    l_obj_3 = F::BtlChar_GetObj(c, p_chr_1);
    F::BtlAnim_PlaySub(c, p_chr_1, 411i32);
    l_mask_4 = ((F::BtlObjAnim_QueryEvent(c, l_obj_3, ((1i32) as u64), 1i32, ((((6i32) as i64 as u64)) as u32))) as u32);
    {
        'l1: loop {
            if !(((((l_i_2) < (32i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((((l_mask_4) & ((((1i32).wrapping_shl((l_i_2) as u32)) as u32)))) != 0) {
                    l_kind_5 = { let a0_ = F::BtlObjAnim_MaskToNode(c, ((l_mask_4) & ((((1i32).wrapping_shl((l_i_2) as u32)) as u32)))); F::BtlObj_GetNodeSide(c, a0_) };
                    if (((((l_kind_5) == (0i32)) as i32)) != 0) {
                        break 'f 45i32;
                    }
                    if (((((l_kind_5) == (1i32)) as i32)) != 0) {
                        break 'f 31i32;
                    }
                }
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
    break 'f 21i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_StartSub19B", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_DecideAttackFollow(c: &mut Ctx, mut p_chr_1: u32, mut p_atk_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_DecideAttackFollow", &PROBE_N, &[(p_chr_1) as u32, (p_atk_2) as u32], false); }
    let mut l_i_3: i32 = 0;
    let r_ = 'f: {
    if ((F::BtlChar_TestFlag(c, p_chr_1, ((91i32) as u32))) != 0) {
        {
            l_i_3 = 0i32;
            'l1: loop {
                if !(((((l_i_3) < (2i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    if ((((((((c.m.r8(((p_atk_2).wrapping_add(11)).wrapping_add(((l_i_3) as u32))) as i8)) as i32)) >= (0i32)) as i32)) != 0) {
                        if (({ let a1_ = ((((c.m.r8(((p_atk_2).wrapping_add(11)).wrapping_add(((l_i_3) as u32))) as i8)) as i32)).wrapping_add(112i32); F::BtlDecide_QueueAttack(c, p_chr_1, a1_) }) != 0) {
                            { let v_ = 1i32; let a_ = (p_atk_2).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
                        }
                    }
                }
                { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
            }
        }
        if (((((c.m.r32(p_atk_2) as i32)) & (1i32))) != 0) {
            if ((F::BtlDecide_Attack(c, p_chr_1, ((((((524288i32) as u32)) as u64)) as i32))) != 0) {
                { let v_ = 0i32; let a_ = (p_atk_2).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
            }
        }
        if (((((c.m.r32(p_atk_2) as i32)) & (2i32))) != 0) {
            if ((F::BtlDecide_Attack(c, p_chr_1, ((((((1048576i32) as u32)) as u64)) as i32))) != 0) {
                { let v_ = 0i32; let a_ = (p_atk_2).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
            }
        }
        if (((((c.m.r32(p_atk_2) as i32)) & (4i32))) != 0) {
            if ((F::BtlDecide_Attack(c, p_chr_1, ((((((1i32) as u32)) as u64)) as i32))) != 0) {
                { let v_ = 0i32; let a_ = (p_atk_2).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
            }
        }
        if (((((c.m.r32(p_atk_2) as i32)) & (8i32))) != 0) {
            if ((F::BtlInput_TestAction(c, p_chr_1, 52i32, 1i32)) != 0) {
                F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), 70i32);
                { let v_ = 0i32; let a_ = (p_atk_2).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
            }
        }
        if (((((c.m.r32(p_atk_2) as i32)) & (256i32))) != 0) {
            if ((F::BtlAct_QueueComboFinish(c, p_chr_1)) != 0) {
                { let v_ = 0i32; let a_ = (p_atk_2).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
            }
        }
        if (((((c.m.r32(p_atk_2) as i32)) & (512i32))) != 0) {
            if ((F::BtlAct_QueueComboFinish(c, p_chr_1)) != 0) {
                { let v_ = 0i32; let a_ = (p_atk_2).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
            }
        }
        if ((((!((F::BtlChar_TestFlag(c, p_chr_1, ((93i32) as u32))) != 0)) as i32)) != 0) {
            if (((rt::flt(F32(c.m.r32((p_atk_2).wrapping_add(20))), F::BtlAnim_GetProgress(c, p_chr_1)) as i32)) != 0) {
                { let a1_ = F::BtlAct_GetQueued(c, p_chr_1); F::BtlAct_Request(c, p_chr_1, a1_) };
                if (((c.m.r32((p_atk_2).wrapping_add(32)) as i32)) != 0) {
                    if (((((c.m.r32(p_atk_2) as i32)) & (16i32))) != 0) {
                        F::BtlChar_SetHeldFlag(c, p_chr_1, ((221i32) as u32));
                    }
                }
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_DecideAttackFollow", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_RequestAttackEnd(c: &mut Ctx, mut p_chr_1: u32, mut p_atk_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_RequestAttackEnd", &PROBE_N, &[(p_chr_1) as u32, (p_atk_2) as u32], false); }
    let mut l_id_3: i32 = 0;
    let r_ = 'f: {
    l_id_3 = 11i32;
    if ((F::BtlChar_TestFlag(c, p_chr_1, ((92i32) as u32))) != 0) {
        if (((((c.m.r32(p_atk_2) as i32)) & (1024i32))) != 0) {
            l_id_3 = 12i32;
        }
    }
    F::BtlAct_Request(c, p_chr_1, l_id_3);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_RequestAttackEnd", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_RushHandler(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_RushHandler", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let mut l_cancelAt_3: u32 = 0;
    let mut l_pose_4: u32 = 0;
    let mut l_next_5: i32 = 0;
    let mut l_mode_6: i32 = 0;
    let mut l_anim_7: i32 = 0;
    let r_ = 'f: {
    l_cancelAt_3 = ((p_chr_1).wrapping_add(1000)).wrapping_add((((0i32) as u32)).wrapping_mul(4));
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        if ((F::BtlAct_TestPoweredSkill(c, p_chr_1, ((16i32) as u32))) != 0) {
            if ((((((c.m.r32((p_chr_1).wrapping_add(3424)) as i32)) == (4i32)) as i32)) != 0) {
                { let v_ = 5i32; let a_ = (p_chr_1).wrapping_add(3424); c.m.w32(a_, (v_) as u32); };
            }
            if ((((((c.m.r32((p_chr_1).wrapping_add(3424)) as i32)) == (9i32)) as i32)) != 0) {
                { let v_ = 0i32; let a_ = (p_chr_1).wrapping_add(3424); c.m.w32(a_, (v_) as u32); };
            }
        }
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((134i32) as u32))) != 0) {
            if ((((((c.m.r32((p_chr_1).wrapping_add(3424)) as i32)) < (5i32)) as i32)) != 0) {
                { let v_ = 5i32; let a_ = (p_chr_1).wrapping_add(3424); c.m.w32(a_, (v_) as u32); };
            }
            F::BtlChar_ClearFlag(c, p_chr_1, ((134i32) as u32));
        }
        l_anim_7 = ((c.m.r32((p_chr_1).wrapping_add(3424)) as i32)).wrapping_add(55i32);
        { let a_ = (p_chr_1).wrapping_add(3424); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        { let r_ = 10i32; let a_ = (p_chr_1).wrapping_add(3424); let n_ = rt::rem_i32((c.m.r32(a_) as i32), r_); c.m.w32(a_, (n_) as u32); };
        F::BtlAnim_Play(c, p_chr_1, l_anim_7, F32(0x3dcccccc));
        F::BtlMove_BeginRiseToOpponent(c, p_chr_1);
        l_pose_4 = F::BtlChar_GetPos(c, p_chr_1);
        { let v_ = F::BtlAtk_GetLaunchSpeed(c, p_chr_1); let a_ = (l_pose_4).wrapping_add(152); c.m.w32(a_, (v_).0); };
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        l_next_5 = (if ((F::BtlChar_TestFlag(c, p_chr_1, ((14i32) as u32))) != 0) { 38i32 } else { 0i32 });
        's1: {
            let sv8: i32 = F::BtlAnim_GetId(c, p_chr_1);
            match sv8 {
                55i32 | 56i32 | 57i32 | 58i32 | 60i32 | 61i32 | 62i32 | 63i32 => {
                    { let _ = F::BtlAnim_AdvanceThen(c, p_chr_1, l_next_5, F32(0x3e999999), 0i32); };
                    F::BtlChar_SetFlag(c, p_chr_1, ((64i32) as u32));
                    break 's1;
                }
                59i32 | 64i32 => {
                    if ((F::BtlAnim_Advance(c, p_chr_1, 0i32)) != 0) {
                        F::BtlAct_Request(c, p_chr_1, 11i32);
                    }
                    if (((rt::flt(F::BtlAnim_GetFrame(c, p_chr_1), rt::i2f({ let a0_ = F::BtlChar_GetObj(c, p_chr_1); F::BtlObjAnim_QueryEvent(c, a0_, ((2i32) as u64), 0i32, ((((2i32) as i64 as u64)) as u32)) })) as i32)) != 0) {
                        F::BtlChar_SetFlag(c, p_chr_1, ((64i32) as u32));
                    }
                    break 's1;
                }
                0i32 | 38i32 => {
                    { let _ = F::BtlAnim_Advance(c, p_chr_1, 1i32); };
                    if ((((!((F::BtlChar_TestFlag(c, p_chr_1, ((52i32) as u32))) != 0)) as i32)) != 0) {
                        F::BtlAct_Request(c, p_chr_1, 11i32);
                    }
                    F::BtlChar_SetFlag(c, p_chr_1, ((135i32) as u32));
                    break 's1;
                }
                _ => {
                }
            }
        }
        l_mode_6 = 6i32;
        if ((((((F::BtlChar_TestFlag(c, p_chr_1, ((91i32) as u32))) != 0) || ((F::BtlChar_TestFlag(c, p_chr_1, ((93i32) as u32))) != 0)) as i32)) != 0) {
            l_mode_6 = 2i32;
        }
        F::BtlMove_Step(c, p_chr_1, l_mode_6, 5i32, 2i32, F32(0x00000000), F32(0x3f6d0979));
        F::BtlMove_ApplyGravity(c, p_chr_1);
        F::BtlChar_SetFlag(c, p_chr_1, ((213i32) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((136i32) as u32));
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((17i32) as u32))) != 0) {
            F::BtlChar_SetHeldFlag(c, p_chr_1, ((14i32) as u32));
        }
    }
    if (((((p_phase_2) == (2i32)) as i32)) != 0) {
        's2: {
            let sv9: i32 = F::BtlAnim_GetId(c, p_chr_1);
            match sv9 {
                55i32 | 56i32 | 57i32 | 58i32 | 60i32 | 61i32 | 62i32 | 63i32 => {
                    { let _ = F::BtlDecide_Main(c, p_chr_1, ((((((139264i32) as u32)) as u64)) as i32)); };
                    { let _ = F::BtlDecide_Attack(c, p_chr_1, ((((((2101363i32) as u32)) as u64)) as i32)); };
                    if ((F::BtlChar_TestFlag(c, p_chr_1, ((49i32) as u32))) != 0) {
                        { let a1_ = F::BtlAct_GetQueued(c, p_chr_1); F::BtlAct_Request(c, p_chr_1, a1_) };
                    }
                    break 's2;
                }
                59i32 => {
                    if ((F::BtlChar_TestFlag(c, p_chr_1, ((91i32) as u32))) != 0) {
                        if (((rt::flt(F32(0x3ecccccc), F::BtlAnim_GetProgress(c, p_chr_1)) as i32)) != 0) {
                            if (({ let a1_ = (if (((rt::flt(F::BtlAct_GetHeightRatio(c, p_chr_1), F32(0x3f000000)) as i32)) != 0) { 123i32 } else { 124i32 }); F::BtlDecide_QueueAttack(c, p_chr_1, a1_) }) != 0) {
                                { let v_ = F32(0x3f199999); let a_ = l_cancelAt_3; c.m.w32(a_, (v_).0); };
                            }
                        }
                        if ((((((((F::BtlParam_GetFlags2(c, p_chr_1)) as u64)) as i32)) & (2i32))) != 0) {
                            if ((F::BtlInput_TestAction(c, p_chr_1, 42i32, 1i32)) != 0) {
                                F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), 31i32);
                                { let v_ = F32(0x3f000000); let a_ = l_cancelAt_3; c.m.w32(a_, (v_).0); };
                            }
                        }
                        if (((rt::flt(F32(c.m.r32(l_cancelAt_3)), F::BtlAnim_GetProgress(c, p_chr_1)) as i32)) != 0) {
                            { let a1_ = F::BtlAct_GetQueued(c, p_chr_1); F::BtlAct_Request(c, p_chr_1, a1_) };
                        }
                    }
                    break 's2;
                }
                64i32 => {
                    break 's2;
                }
                0i32 | 38i32 => {
                    { let _ = F::BtlDecide_Main(c, p_chr_1, ((((((139264i32) as u32)) as u64)) as i32)); };
                    { let _ = F::BtlDecide_Attack(c, p_chr_1, ((((((2101363i32) as u32)) as u64)) as i32)); };
                    { let a1_ = F::BtlAct_GetQueued(c, p_chr_1); F::BtlAct_Request(c, p_chr_1, a1_) };
                    break 's2;
                }
                _ => {
                }
            }
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_RushHandler", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_RushAutoHandler(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_RushAutoHandler", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let r_ = 'f: {
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        if (((((F::BtlAct_GetPrev(c, p_chr_1)) != (69i32)) as i32)) != 0) {
            F::BtlAnim_Play(c, p_chr_1, 55i32, F32(0x3dcccccc));
        }
        { let a2_ = (if (((rt::flt(F32(c.m.r32((p_chr_1).wrapping_add(1192))), F32(0x00000000)) as i32)) != 0) { 17i32 } else { 18i32 }); F::ChrCam_RequestCut(c, p_chr_1, 1i32, a2_) };
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        's1: {
            let sv3: i32 = F::BtlAnim_GetId(c, p_chr_1);
            match sv3 {
                55i32 | 56i32 | 57i32 | 60i32 | 61i32 | 62i32 | 63i32 => {
                    F::BtlAnim_SetStep(c, p_chr_1, F32(0x40400000));
                    { let _ = { let a1_ = (F::BtlAnim_GetId(c, p_chr_1)).wrapping_add(1i32); F::BtlAnim_AdvanceThen(c, p_chr_1, a1_, F32(0x00000000), 0i32) }; };
                    break 's1;
                }
                58i32 => {
                    F::BtlAnim_SetStep(c, p_chr_1, F32(0x40400000));
                    { let _ = F::BtlAnim_AdvanceThen(c, p_chr_1, 60i32, F32(0x00000000), 0i32); };
                    break 's1;
                }
                64i32 => {
                    if ((F::BtlAnim_Advance(c, p_chr_1, 0i32)) != 0) {
                        F::BtlAct_Request(c, p_chr_1, 11i32);
                    }
                    if ((((!((F::BtlAnim_IsNew(c, p_chr_1)) != 0)) as i32)) != 0) {
                        if ((((((({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(3245); (c.m.r8(a_) as i8) }) as i32)) == ((!((({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(3244); (c.m.r8(a_) as i8) }) as i32))))) as i32)) != 0) {
                            F::BtlChar_SetFlag(c, p_chr_1, ((144i32) as u32));
                        }
                    }
                    break 's1;
                }
                59i32 => {
                    break 's1;
                }
                _ => {
                }
            }
        }
        F::BtlMove_Step(c, p_chr_1, 2i32, 5i32, 2i32, F32(0x00000000), F32(0x3f6d0979));
        F::BtlMove_ApplyGravity(c, p_chr_1);
        F::BtlChar_SetFlag(c, p_chr_1, ((213i32) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((136i32) as u32));
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((17i32) as u32))) != 0) {
            F::BtlChar_SetHeldFlag(c, p_chr_1, ((14i32) as u32));
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_RushAutoHandler", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_RushRapidHandler(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_RushRapidHandler", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let mut l_pose_3: u32 = 0;
    let mut l_next_4: i32 = 0;
    let mut l_can_5: i32 = 0;
    let mut l_go_6: i32 = 0;
    let r_ = 'f: {
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        if (((((F::BtlAct_GetPrev(c, p_chr_1)) != (70i32)) as i32)) != 0) {
            { let v_ = 0i32; let a_ = (p_chr_1).wrapping_add(3428); c.m.w32(a_, (v_) as u32); };
            { let a1_ = F::BtlParam_GetDashSound(c, p_chr_1); F::BtlCharSnd_PlayCommon(c, p_chr_1, a1_) };
            F::BtlChar_PlayVoice(c, p_chr_1, 7i32);
        }
        { let v_ = { let a0_ = (c.m.r32((p_chr_1).wrapping_add(3428)) as i32); F::BtlUtil_Clamp(c, a0_, 0i32, 4i32) }; let a_ = (p_chr_1).wrapping_add(3428); c.m.w32(a_, (v_) as u32); };
        { let a1_ = ((c.m.r32((p_chr_1).wrapping_add(3428)) as i32)).wrapping_add(60i32); F::BtlAnim_Play(c, p_chr_1, a1_, F32(0x3dcccccc)) };
        { let a_ = (p_chr_1).wrapping_add(3428); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        F::BtlMove_BeginRiseToOpponent(c, p_chr_1);
        F::BtlChar_SetHeldFlag(c, p_chr_1, ((14i32) as u32));
        l_pose_3 = F::BtlChar_GetPos(c, p_chr_1);
        { let v_ = F::BtlAtk_GetLaunchSpeed(c, p_chr_1); let a_ = (l_pose_3).wrapping_add(152); c.m.w32(a_, (v_).0); };
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((19i32) as u32))) != 0) {
            { let a2_ = (if (((rt::flt(F32(c.m.r32((p_chr_1).wrapping_add(1192))), F32(0x00000000)) as i32)) != 0) { 26i32 } else { 27i32 }); F::ChrCam_RequestCut(c, p_chr_1, 1i32, a2_) };
        }
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        l_next_4 = (if ((F::BtlChar_TestFlag(c, p_chr_1, ((14i32) as u32))) != 0) { 38i32 } else { 0i32 });
        's1: {
            let sv7: i32 = F::BtlAnim_GetId(c, p_chr_1);
            match sv7 {
                60i32 | 61i32 | 62i32 | 63i32 => {
                    F::BtlAnim_SetStep(c, p_chr_1, F32(0x40400000));
                    F::BtlAnim_SetObjRate(c, p_chr_1, F32(0x40000000));
                    { let _ = F::BtlAnim_AdvanceThen(c, p_chr_1, l_next_4, F32(0x3e999999), 0i32); };
                    F::BtlChar_SetFxBit(c, p_chr_1, 9i32);
                    break 's1;
                }
                64i32 => {
                    if (((rt::flt(F::BtlAnim_GetFrame(c, p_chr_1), rt::i2f({ let a0_ = F::BtlChar_GetObj(c, p_chr_1); F::BtlObjAnim_QueryEvent(c, a0_, ((1i32) as u64), 0i32, ((((0i32) as i64 as u64)) as u32)) })) as i32)) != 0) {
                        F::BtlAnim_SetStep(c, p_chr_1, F32(0x40400000));
                        F::BtlChar_SetFxBit(c, p_chr_1, 9i32);
                    } else {
                        F::ChrCam_EndCut(c, p_chr_1);
                    }
                    if ((F::BtlAnim_Advance(c, p_chr_1, 0i32)) != 0) {
                        F::BtlAct_Request(c, p_chr_1, 11i32);
                    }
                    if ((F::BtlChar_TestFlag(c, p_chr_1, ((91i32) as u32))) != 0) {
                        { let v_ = F32(0x00000000); let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(152); c.m.w32(a_, (v_).0); };
                    }
                    break 's1;
                }
                0i32 | 38i32 => {
                    { let _ = F::BtlAnim_Advance(c, p_chr_1, 1i32); };
                    if ((((!((F::BtlChar_TestFlag(c, p_chr_1, ((52i32) as u32))) != 0)) as i32)) != 0) {
                        F::BtlAct_Request(c, p_chr_1, 11i32);
                    }
                    break 's1;
                }
                _ => {
                }
            }
        }
        F::BtlMove_Step(c, p_chr_1, 2i32, 5i32, 2i32, F32(0x00000000), F32(0x3f6d0979));
        F::BtlMove_ApplyGravity(c, p_chr_1);
        F::BtlChar_SetFlag(c, p_chr_1, ((213i32) as u32));
    }
    if (((((p_phase_2) == (2i32)) as i32)) != 0) {
        l_can_5 = 0i32;
        l_go_6 = 0i32;
        's2: {
            let sv8: i32 = F::BtlAnim_GetId(c, p_chr_1);
            match sv8 {
                60i32 | 61i32 | 62i32 | 63i32 => {
                    l_can_5 = 1i32;
                    if ((F::BtlChar_TestFlag(c, p_chr_1, ((49i32) as u32))) != 0) {
                        l_go_6 = 1i32;
                    }
                    break 's2;
                }
                0i32 | 38i32 => {
                    l_can_5 = 1i32;
                    l_go_6 = 1i32;
                    break 's2;
                }
                _ => {
                }
            }
        }
        if ((l_can_5) != 0) {
            if ((((((((((F::BtlChar_TestFlag(c, p_chr_1, ((6i32) as u32))) != 0) && ((F::BtlAct_TestAttackSkill(c, p_chr_1, 60i32)) != 0)) as i32)) != 0) && ((((!((F::BtlChar_TestMemberUnk70(c, p_chr_1)) != 0)) as i32)) != 0)) as i32)) != 0) {
                { let _ = F::BtlDecide_QueueAttack(c, p_chr_1, 172i32); };
            } else {
                { let _ = F::BtlDecide_QueueAttack(c, p_chr_1, 157i32); };
            }
            if ((F::BtlInput_TestAction(c, p_chr_1, 52i32, 1i32)) != 0) {
                F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), 70i32);
            }
            if ((l_go_6) != 0) {
                if ((F::BtlChar_TestFlag(c, p_chr_1, ((91i32) as u32))) != 0) {
                    { let a1_ = F::BtlAct_GetQueued(c, p_chr_1); F::BtlAct_Request(c, p_chr_1, a1_) };
                }
            }
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_RushRapidHandler", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_SmashChargeHandler(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_SmashChargeHandler", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let mut l_frames_3: u32 = 0;
    let mut l_cancelAt_4: u32 = 0;
    let mut l_pose_5: u32 = 0;
    let mut l_anim_6: i32 = 0;
    let mut l_yawMode_7: i32 = 0;
    let mut l_pitchMode_8: i32 = 0;
    let mut l_charging_9: i32 = 0;
    let mut l_id_10: i32 = 0;
    let mut l_speed_11: F32 = F32(0);
    let mut l_accel_12: F32 = F32(0);
    let mut l_turn_13: F32 = F32(0);
    let r_ = 'f: {
    l_frames_3 = ((p_chr_1).wrapping_add(976)).wrapping_add((((2i32) as u32)).wrapping_mul(4));
    l_cancelAt_4 = ((p_chr_1).wrapping_add(1000)).wrapping_add((((0i32) as u32)).wrapping_mul(4));
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        l_anim_6 = 0i32;
        's1: {
            let sv14: i32 = F::BtlAct_GetCurrent(c, p_chr_1);
            match sv14 {
                71i32 => {
                    l_anim_6 = 80i32;
                    break 's1;
                }
                72i32 => {
                    l_anim_6 = 83i32;
                    break 's1;
                }
                73i32 => {
                    l_anim_6 = 86i32;
                    break 's1;
                }
                74i32 => {
                    l_anim_6 = 89i32;
                    break 's1;
                }
                75i32 => {
                    l_anim_6 = 74i32;
                    break 's1;
                }
                76i32 => {
                    l_anim_6 = 77i32;
                    break 's1;
                }
                _ => {
                }
            }
        }
        F::BtlAnim_Play(c, p_chr_1, l_anim_6, F32(0x3e199999));
        F::BtlMove_BeginRiseToOpponent(c, p_chr_1);
        F::BtlCharSnd_PlayCommon(c, p_chr_1, 46i32);
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        l_speed_11 = F32(0x00000000);
        l_accel_12 = F32(0x3f6d0979);
        l_turn_13 = l_speed_11;
        l_yawMode_7 = 6i32;
        l_pitchMode_8 = 5i32;
        l_charging_9 = 0i32;
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((129i32) as u32))) != 0) {
            l_speed_11 = F32(0x3eed0979);
            l_yawMode_7 = 2i32;
            l_turn_13 = F32(0x3d00adfb);
        }
        's2: {
            let sv15: i32 = F::BtlAnim_GetId(c, p_chr_1);
            match sv15 {
                74i32 | 77i32 | 80i32 | 83i32 | 86i32 | 89i32 => {
                    F::BtlAct_SetChargeAnimStep(c, p_chr_1);
                    if ((((!((F::BtlChar_TestFlag(c, p_chr_1, ((132i32) as u32))) != 0)) as i32)) != 0) {
                        { let _ = { let a1_ = (F::BtlAnim_GetId(c, p_chr_1)).wrapping_add(1i32); F::BtlAnim_AdvanceThen(c, p_chr_1, a1_, F32(0x00000000), 0i32) }; };
                    } else {
                        { let _ = F::BtlAnim_Advance(c, p_chr_1, 0i32); };
                    }
                    l_charging_9 = 1i32;
                    break 's2;
                }
                75i32 | 78i32 | 81i32 | 84i32 | 87i32 | 90i32 => {
                    F::BtlAct_SetChargeAnimDuration(c, p_chr_1);
                    if ((((!((F::BtlChar_TestFlag(c, p_chr_1, ((132i32) as u32))) != 0)) as i32)) != 0) {
                        if ((F::BtlAnim_Advance(c, p_chr_1, 0i32)) != 0) {
                            { let r_ = 1i32; let a_ = ((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                        }
                    } else {
                        { let _ = F::BtlAnim_Advance(c, p_chr_1, 0i32); };
                    }
                    l_charging_9 = 1i32;
                    break 's2;
                }
                76i32 | 79i32 | 82i32 | 85i32 | 88i32 | 91i32 => {
                    if ((F::BtlAnim_IsNew(c, p_chr_1)) != 0) {
                        if (((rt::flt(F32(0x3f7d70a3), F32(c.m.r32((p_chr_1).wrapping_add(3448)))) as i32)) != 0) {
                            F::BtlCharSnd_PlayCommon(c, p_chr_1, 47i32);
                            F::BtlChar_SetVibration(c, p_chr_1, F32(0x3f800000), F32(0x3e999999));
                        }
                        F::BtlAct_PlayPickedVoice(c, p_chr_1);
                        l_pose_5 = F::BtlChar_GetPos(c, p_chr_1);
                        { let v_ = F::BtlAtk_GetLaunchSpeed(c, p_chr_1); let a_ = (l_pose_5).wrapping_add(152); c.m.w32(a_, (v_).0); };
                    }
                    { let a1_ = rt::fadd(F32(c.m.r32((p_chr_1).wrapping_add(3448))), F32(0x3f800000)); F::BtlAnim_SetObjRate(c, p_chr_1, a1_) };
                    if ((F::BtlAnim_Advance(c, p_chr_1, 0i32)) != 0) {
                        F::BtlAct_Request(c, p_chr_1, 11i32);
                    }
                    l_speed_11 = F32(0x00000000);
                    l_yawMode_7 = 6i32;
                    l_turn_13 = l_speed_11;
                    if ((F::BtlMove_IsBlockedByOpponent(c, p_chr_1)) != 0) {
                        if ((((((F::BtlChar_TestFlag(c, p_chr_1, ((91i32) as u32))) != 0) || ((F::BtlChar_TestFlag(c, p_chr_1, ((93i32) as u32))) != 0)) as i32)) != 0) {
                            l_turn_13 = F32(0x40490fda);
                            l_yawMode_7 = 2i32;
                        }
                    }
                    break 's2;
                }
                _ => {
                }
            }
        }
        if ((l_charging_9) != 0) {
            if ((((!((F::BtlChar_TestFlag(c, p_chr_1, ((132i32) as u32))) != 0)) as i32)) != 0) {
                if ((F::BtlAct_StepCharge(c, p_chr_1)) != 0) {
                    F::BtlChar_SetFxBit(c, p_chr_1, 30i32);
                    if ((((((c.m.r32((p_chr_1).wrapping_add(3460)) as i32)) == (0i32)) as i32)) != 0) {
                        F::BtlCharSnd_PlayCommon(c, p_chr_1, 19i32);
                        F::BtlCharSnd_PlayCommon(c, p_chr_1, 15i32);
                    }
                    { let a_ = (p_chr_1).wrapping_add(3460); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                }
            } else {
                if ((((((c.m.r32(l_frames_3) as i32)) == (0i32)) as i32)) != 0) {
                    F::BtlChar_SetFxBit(c, p_chr_1, 12i32);
                    F::BtlCharSnd_PlayCommon(c, p_chr_1, 32i32);
                }
                if ((((((c.m.r32(l_frames_3) as i32)) >= (2i32)) as i32)) != 0) {
                    F::BtlChar_SetFlag(c, p_chr_1, ((11i32) as u32));
                    l_speed_11 = F32(0x4201a12e);
                    l_yawMode_7 = 2i32;
                    l_accel_12 = F32(0x461c4000);
                    l_pitchMode_8 = 4i32;
                    l_turn_13 = F32(0x40490fda);
                }
                if ((((((c.m.r32(l_frames_3) as i32)) >= (6i32)) as i32)) != 0) {
                    { let a1_ = (F::BtlAnim_GetId(c, p_chr_1)).wrapping_add(1i32); F::BtlAnim_Request(c, p_chr_1, a1_, F32(0x00000000)) };
                }
                { let a_ = l_frames_3; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                F::BtlChar_SetFlag(c, p_chr_1, ((66i32) as u32));
                F::BtlChar_SetFlag(c, p_chr_1, ((67i32) as u32));
                F::BtlChar_SetFlag(c, p_chr_1, ((68i32) as u32));
                F::BtlChar_SetFlag(c, p_chr_1, ((69i32) as u32));
                F::BtlChar_SetFlag(c, p_chr_1, ((70i32) as u32));
            }
        }
        F::BtlMove_TurnYaw(c, p_chr_1, l_yawMode_7, l_turn_13);
        F::BtlMove_TurnPitch(c, p_chr_1, l_pitchMode_8, F32(0x40490fda));
        F::BtlMove_TurnModelYaw(c, p_chr_1, F32(0x3f20d97a), F32(0x3e999999));
        F::BtlMove_SetDirection(c, p_chr_1, 3i32);
        F::BtlMove_Advance(c, p_chr_1, l_speed_11, l_accel_12);
        F::BtlMove_ApplyGravity(c, p_chr_1);
        F::BtlChar_SetFlag(c, p_chr_1, ((213i32) as u32));
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((17i32) as u32))) != 0) {
            F::BtlChar_SetHeldFlag(c, p_chr_1, ((14i32) as u32));
        }
    }
    if (((((p_phase_2) == (2i32)) as i32)) != 0) {
        's3: {
            let sv16: i32 = F::BtlAnim_GetId(c, p_chr_1);
            match sv16 {
                74i32 | 75i32 | 77i32 | 78i32 | 80i32 | 81i32 | 83i32 | 84i32 | 86i32 | 87i32 | 89i32 | 90i32 => {
                    if ((((!((F::BtlChar_TestFlag(c, p_chr_1, ((132i32) as u32))) != 0)) as i32)) != 0) {
                        if ((F::BtlInput_TestAction(c, p_chr_1, 63i32, 1i32)) != 0) {
                            { let r_ = 1i32; let a_ = ((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                        }
                        if (((((c.m.r32(((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) & (1i32))) != 0) {
                            if ((((((c.m.r32((p_chr_1).wrapping_add(3460)) as i32)) == (1i32)) as i32)) != 0) {
                                F::BtlChar_SetHeldFlag(c, p_chr_1, ((132i32) as u32));
                            } else {
                                if ((((((((((((((c.m.r32((p_chr_1).wrapping_add(3460)) as i32)) > (0i32)) as i32)) != 0) && ((F::BtlAct_TestPoweredSkill(c, p_chr_1, ((512i32) as u32))) != 0)) as i32)) != 0) && ((F::BtlChar_TestFlag(c, p_chr_1, ((19i32) as u32))) != 0)) as i32)) != 0) {
                                    l_id_10 = 81i32;
                                    's4: {
                                        let sv17: i32 = F::BtlAct_GetCurrent(c, p_chr_1);
                                        match sv17 {
                                            73i32 => {
                                                l_id_10 = 79i32;
                                                break 's4;
                                            }
                                            74i32 => {
                                                l_id_10 = 80i32;
                                                break 's4;
                                            }
                                            71i32 => {
                                                l_id_10 = 77i32;
                                                break 's4;
                                            }
                                            72i32 => {
                                                l_id_10 = 78i32;
                                                break 's4;
                                            }
                                            75i32 => {
                                                l_id_10 = 81i32;
                                                break 's4;
                                            }
                                            76i32 => {
                                                l_id_10 = 82i32;
                                                break 's4;
                                            }
                                            _ => {
                                            }
                                        }
                                    }
                                    F::BtlAct_Request(c, p_chr_1, l_id_10);
                                } else {
                                    { let r_ = 2i32; let a_ = ((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                                }
                            }
                        }
                        if (((((c.m.r32(((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) & (2i32))) != 0) {
                            { let a1_ = (F::BtlAnim_GetId(c, p_chr_1)).wrapping_add(1i32); F::BtlAnim_Request(c, p_chr_1, a1_, F32(0x00000000)) };
                        }
                    }
                    break 's3;
                }
                76i32 | 79i32 | 82i32 | 85i32 | 88i32 | 91i32 => {
                    if ((F::BtlChar_TestFlag(c, p_chr_1, ((91i32) as u32))) != 0) {
                        F::BtlChar_SetFlag(c, p_chr_1, ((61i32) as u32));
                        's5: {
                            let sv18: i32 = F::BtlOpp_GetSeenAction(c, p_chr_1);
                            match sv18 {
                                213i32 => {
                                    if ((((!((F::BtlAct_HasQueued(c, p_chr_1)) != 0)) as i32)) != 0) {
                                        if ((F::BtlDecide_Attack(c, p_chr_1, ((((((1572864i32) as u32)) as u64)) as i32))) != 0) {
                                            { let v_ = F32(0x3f333333); let a_ = l_cancelAt_4; c.m.w32(a_, (v_).0); };
                                        }
                                    }
                                    break 's5;
                                }
                                212i32 => {
                                    if ((((!((F::BtlAct_HasQueued(c, p_chr_1)) != 0)) as i32)) != 0) {
                                        if ((F::BtlDecide_Main(c, p_chr_1, ((((((8i32) as u32)) as u64)) as i32))) != 0) {
                                            { let v_ = F32(0x3f333333); let a_ = l_cancelAt_4; c.m.w32(a_, (v_).0); };
                                        }
                                    }
                                    break 's5;
                                }
                                _ => {
                                    if ((((!((F::BtlAct_HasQueued(c, p_chr_1)) != 0)) as i32)) != 0) {
                                        if ((F::BtlDecide_Attack(c, p_chr_1, ((((((1i32) as u32)) as u64)) as i32))) != 0) {
                                            { let v_ = F32(0x3f199999); let a_ = l_cancelAt_4; c.m.w32(a_, (v_).0); };
                                        }
                                    }
                                    break 's5;
                                }
                            }
                        }
                        if ((F::BtlAct_HasQueued(c, p_chr_1)) != 0) {
                            if (((rt::flt(F32(c.m.r32(l_cancelAt_4)), F::BtlAnim_GetProgress(c, p_chr_1)) as i32)) != 0) {
                                { let a1_ = F::BtlAct_GetQueued(c, p_chr_1); F::BtlAct_Request(c, p_chr_1, a1_) };
                            }
                        }
                    }
                    F::BtlAct_CancelOnOppC9(c, p_chr_1, F32(0x3f199999));
                    break 's3;
                }
                _ => {
                }
            }
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_SmashChargeHandler", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_SmashFullHandler(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_SmashFullHandler", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let mut l_cancelAt_3: u32 = 0;
    let mut l_pose_4: u32 = 0;
    let mut l_cut_5: i32 = 0;
    let mut l_anim_6: i32 = 0;
    let mut l_mode_7: i32 = 0;
    let r_ = 'f: {
    l_cancelAt_3 = ((p_chr_1).wrapping_add(1000)).wrapping_add((((0i32) as u32)).wrapping_mul(4));
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        l_cut_5 = 0i32;
        l_anim_6 = 0i32;
        's1: {
            let sv8: i32 = F::BtlAct_GetCurrent(c, p_chr_1);
            match sv8 {
                77i32 => {
                    l_anim_6 = 80i32;
                    l_cut_5 = 30i32;
                    break 's1;
                }
                78i32 => {
                    l_anim_6 = 83i32;
                    l_cut_5 = 31i32;
                    break 's1;
                }
                79i32 => {
                    l_anim_6 = 86i32;
                    l_cut_5 = 28i32;
                    break 's1;
                }
                80i32 => {
                    l_anim_6 = 89i32;
                    l_cut_5 = 29i32;
                    break 's1;
                }
                81i32 => {
                    l_anim_6 = 74i32;
                    l_cut_5 = 30i32;
                    break 's1;
                }
                82i32 => {
                    l_anim_6 = 77i32;
                    l_cut_5 = 30i32;
                    break 's1;
                }
                _ => {
                }
            }
        }
        F::BtlAnim_Play(c, p_chr_1, l_anim_6, F32(0x00000000));
        F::BtlMove_BeginRiseToOpponent(c, p_chr_1);
        F::BtlChar_PlayVoice(c, p_chr_1, 7i32);
        F::ChrCam_RequestCut(c, p_chr_1, 1i32, l_cut_5);
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        l_mode_7 = 2i32;
        's2: {
            let sv9: i32 = F::BtlAnim_GetId(c, p_chr_1);
            match sv9 {
                75i32 | 78i32 | 81i32 | 84i32 | 87i32 | 90i32 => {
                    F::BtlAnim_SetDuration(c, p_chr_1, F32(0x3e4ccccc));
                    { let _ = { let a1_ = (F::BtlAnim_GetId(c, p_chr_1)).wrapping_add(1i32); F::BtlAnim_AdvanceThen(c, p_chr_1, a1_, F32(0x00000000), 0i32) }; };
                    F::BtlChar_SetFlag(c, p_chr_1, ((293i32) as u32));
                    F::BtlChar_SetFlag(c, p_chr_1, ((298i32) as u32));
                    { let v_ = F32(0x3f800000); let a_ = (p_chr_1).wrapping_add(3448); c.m.w32(a_, (v_).0); };
                    break 's2;
                }
                74i32 | 77i32 | 80i32 | 83i32 | 86i32 | 89i32 => {
                    { let _ = { let a1_ = (F::BtlAnim_GetId(c, p_chr_1)).wrapping_add(1i32); F::BtlAnim_AdvanceThen(c, p_chr_1, a1_, F32(0x00000000), 0i32) }; };
                    F::BtlChar_SetFlag(c, p_chr_1, ((293i32) as u32));
                    F::BtlChar_SetFlag(c, p_chr_1, ((298i32) as u32));
                    { let v_ = F32(0x3f800000); let a_ = (p_chr_1).wrapping_add(3448); c.m.w32(a_, (v_).0); };
                    break 's2;
                }
                76i32 | 79i32 | 82i32 | 85i32 | 88i32 | 91i32 => {
                    if ((F::BtlAnim_Advance(c, p_chr_1, 0i32)) != 0) {
                        F::BtlAct_Request(c, p_chr_1, 11i32);
                    }
                    if ((F::BtlAnim_IsNew(c, p_chr_1)) != 0) {
                        F::ChrCam_EndCut(c, p_chr_1);
                        F::BtlCharSnd_PlayCommon(c, p_chr_1, 47i32);
                        l_pose_4 = F::BtlChar_GetPos(c, p_chr_1);
                        { let v_ = F::BtlAtk_GetLaunchSpeed(c, p_chr_1); let a_ = (l_pose_4).wrapping_add(152); c.m.w32(a_, (v_).0); };
                        F::BtlChar_SetVibration(c, p_chr_1, F32(0x3f800000), F32(0x3e999999));
                    }
                    if ((F::BtlChar_TestFlag(c, p_chr_1, ((92i32) as u32))) != 0) {
                        l_mode_7 = 6i32;
                    }
                    break 's2;
                }
                _ => {
                }
            }
        }
        F::BtlMove_Step(c, p_chr_1, l_mode_7, 5i32, 2i32, F32(0x00000000), F32(0x3f6d0979));
        F::BtlMove_ApplyGravity(c, p_chr_1);
        F::BtlChar_SetFlag(c, p_chr_1, ((213i32) as u32));
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((17i32) as u32))) != 0) {
            F::BtlChar_SetHeldFlag(c, p_chr_1, ((14i32) as u32));
        }
    }
    if (((((p_phase_2) == (2i32)) as i32)) != 0) {
        's3: {
            let sv10: i32 = F::BtlAnim_GetId(c, p_chr_1);
            match sv10 {
                74i32 | 75i32 | 77i32 | 78i32 | 80i32 | 81i32 | 83i32 | 84i32 | 86i32 | 87i32 | 89i32 | 90i32 => {
                    break 's3;
                }
                76i32 | 79i32 | 82i32 | 85i32 | 88i32 | 91i32 => {
                    if ((F::BtlChar_TestFlag(c, p_chr_1, ((91i32) as u32))) != 0) {
                        F::BtlChar_SetFlag(c, p_chr_1, ((61i32) as u32));
                        if (((((F::BtlOpp_GetSeenAction(c, p_chr_1)) == (213i32)) as i32)) != 0) {
                            if ((((!((F::BtlAct_HasQueued(c, p_chr_1)) != 0)) as i32)) != 0) {
                                if ((F::BtlDecide_Attack(c, p_chr_1, ((((((1572864i32) as u32)) as u64)) as i32))) != 0) {
                                    { let v_ = F32(0x3f333333); let a_ = l_cancelAt_3; c.m.w32(a_, (v_).0); };
                                }
                            }
                        }
                        if ((F::BtlAct_HasQueued(c, p_chr_1)) != 0) {
                            if (((rt::flt(F32(c.m.r32(l_cancelAt_3)), F::BtlAnim_GetProgress(c, p_chr_1)) as i32)) != 0) {
                                { let a1_ = F::BtlAct_GetQueued(c, p_chr_1); F::BtlAct_Request(c, p_chr_1, a1_) };
                            }
                        }
                    }
                    break 's3;
                }
                _ => {
                }
            }
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_SmashFullHandler", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_DashSmashHandler(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_DashSmashHandler", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let mut l_anim_3: i32 = 0;
    let mut l_cost_4: i32 = 0;
    let mut l_steer_5: i32 = 0;
    let mut l_speed_6: F32 = F32(0);
    let mut l_accel_7: F32 = F32(0);
    let mut l_closeSpeed_8: F32 = F32(0);
    let mut l_yawAccel_9: F32 = F32(0);
    let mut l_pitchAccel_10: F32 = F32(0);
    let mut l_yawMax_11: F32 = F32(0);
    let mut l_pitchMax_12: F32 = F32(0);
    let mut l_scale_13: F32 = F32(0);
    let r_ = 'f: {
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        l_anim_3 = 0i32;
        's1: {
            let sv14: i32 = F::BtlAct_GetCurrent(c, p_chr_1);
            match sv14 {
                83i32 => {
                    l_anim_3 = 101i32;
                    break 's1;
                }
                84i32 => {
                    l_anim_3 = 104i32;
                    break 's1;
                }
                85i32 => {
                    l_anim_3 = 95i32;
                    break 's1;
                }
                86i32 => {
                    l_anim_3 = 98i32;
                    break 's1;
                }
                87i32 => {
                    l_anim_3 = 92i32;
                    break 's1;
                }
                _ => {
                }
            }
        }
        F::BtlAnim_Play(c, p_chr_1, l_anim_3, F32(0x3e199999));
        F::BtlChar_SetHeldFlag(c, p_chr_1, ((14i32) as u32));
        F::BtlCharSnd_PlayCommon(c, p_chr_1, 46i32);
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        l_speed_6 = F::BtlMoveParam_GetSpeed(c, p_chr_1, 6i32);
        l_closeSpeed_8 = l_speed_6;
        l_cost_4 = F::BtlMoveParam_GetKiCost(c, p_chr_1, 0i32);
        l_accel_7 = F32(0x461c4000);
        l_steer_5 = 1i32;
        's2: {
            let sv15: i32 = F::BtlAnim_GetId(c, p_chr_1);
            match sv15 {
                92i32 | 95i32 | 98i32 | 101i32 | 104i32 => {
                    { let _ = { let a1_ = (F::BtlAnim_GetId(c, p_chr_1)).wrapping_add(1i32); F::BtlAnim_AdvanceThen(c, p_chr_1, a1_, F32(0x00000000), 0i32) }; };
                    F::BtlChar_SetFxBit(c, p_chr_1, 4i32);
                    F::BtlChar_SetFxBit(c, p_chr_1, 9i32);
                    F::BtlChar_SetVibration(c, p_chr_1, F32(0x3f333333), F32(0x3dcccccc));
                    { let _ = F::BtlMember_SpendKi(c, p_chr_1, l_cost_4, 0i32); };
                    F::BtlChar_SetFlag(c, p_chr_1, ((71i32) as u32));
                    break 's2;
                }
                93i32 | 96i32 | 99i32 | 102i32 | 105i32 => {
                    F::BtlAct_SetChargeAnimDuration(c, p_chr_1);
                    { let _ = { let a1_ = (F::BtlAnim_GetId(c, p_chr_1)).wrapping_add(1i32); F::BtlAnim_AdvanceThen(c, p_chr_1, a1_, F32(0x00000000), 0i32) }; };
                    if ((((((c.m.r32((p_chr_1).wrapping_add(3432)) as i32)) > (0i32)) as i32)) != 0) {
                        { let _ = F::BtlAct_StepCharge(c, p_chr_1); };
                        F::BtlChar_SetFxBit(c, p_chr_1, 30i32);
                    } else {
                        if ((F::BtlAct_StepCharge(c, p_chr_1)) != 0) {
                            if ((((((c.m.r32((p_chr_1).wrapping_add(3460)) as i32)) == (0i32)) as i32)) != 0) {
                                F::BtlCharSnd_PlayCommon(c, p_chr_1, 19i32);
                                F::BtlCharSnd_PlayCommon(c, p_chr_1, 15i32);
                            }
                            { let a_ = (p_chr_1).wrapping_add(3460); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        }
                    }
                    F::BtlChar_SetFxBit(c, p_chr_1, 4i32);
                    F::BtlChar_SetFxBit(c, p_chr_1, 9i32);
                    F::BtlChar_SetVibration(c, p_chr_1, F32(0x3f333333), F32(0x3dcccccc));
                    { let _ = F::BtlMember_SpendKi(c, p_chr_1, l_cost_4, 0i32); };
                    F::BtlChar_SetFlag(c, p_chr_1, ((71i32) as u32));
                    break 's2;
                }
                94i32 | 97i32 | 100i32 | 103i32 | 106i32 => {
                    if ((F::BtlAnim_IsNew(c, p_chr_1)) != 0) {
                        F::BtlAct_PlayPickedVoice(c, p_chr_1);
                        F::BtlChar_SetVibration(c, p_chr_1, F32(0x3f800000), F32(0x3e999999));
                    }
                    if ((F::BtlAnim_Advance(c, p_chr_1, 0i32)) != 0) {
                        F::BtlAct_Request(c, p_chr_1, 11i32);
                    }
                    F::BtlAnim_SetObjRate(c, p_chr_1, F32(0x40000000));
                    if ((F::BtlChar_TestFlag(c, p_chr_1, ((91i32) as u32))) != 0) {
                        l_speed_6 = F32(0x00000000);
                        { let v_ = l_speed_6; let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(152); c.m.w32(a_, (v_).0); };
                    }
                    if ((F::BtlChar_TestFlag(c, p_chr_1, ((108i32) as u32))) != 0) {
                        { let r_ = 1i32; let a_ = ((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    }
                    if (((((c.m.r32(((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) & (1i32))) != 0) {
                        l_speed_6 = F32(0x00000000);
                        { let v_ = l_speed_6; let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(152); c.m.w32(a_, (v_).0); };
                    }
                    if ((((((F::BtlChar_TestFlag(c, p_chr_1, ((92i32) as u32))) != 0) || ((F::BtlChar_TestFlag(c, p_chr_1, ((93i32) as u32))) != 0)) as i32)) != 0) {
                        l_speed_6 = F32(0x00000000);
                        l_accel_7 = F32(0x3f6d0979);
                        l_steer_5 = 0i32;
                        if ((F::BtlMove_IsBlockedByOpponent(c, p_chr_1)) != 0) {
                            { let v_ = l_speed_6; let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(152); c.m.w32(a_, (v_).0); };
                        }
                    }
                    break 's2;
                }
                _ => {
                }
            }
        }
        if ((((((F::BtlChar_TestFlag(c, p_chr_1, ((5i32) as u32))) != 0) && ((l_steer_5) != 0)) as i32)) != 0) {
            l_yawAccel_9 = F::BtlMoveParam_GetTurnAccel(c, p_chr_1, 0i32);
            l_pitchAccel_10 = F::BtlMoveParam_GetTurnAccel(c, p_chr_1, 1i32);
            l_yawMax_11 = F::BtlMoveParam_GetTurnMax(c, p_chr_1, 0i32);
            l_pitchMax_12 = F::BtlMoveParam_GetTurnMax(c, p_chr_1, 1i32);
            l_scale_13 = { let a0_ = rt::fmul(F::BtlOpp_GetGapXZ(c, p_chr_1), F32(0x3c23d70a)); F::BtlUtil_MinF(c, a0_, F32(0x3f800000)) };
            F::BtlMove_SteerAtOpponent(c, p_chr_1, l_closeSpeed_8, l_yawAccel_9, l_pitchAccel_10, rt::fmul(l_yawMax_11, l_scale_13), rt::fmul(l_pitchMax_12, l_scale_13), F32(0x40490fda));
            F::BtlMove_TurnModelYaw(c, p_chr_1, F32(0x3f20d97a), F32(0x3f800000));
            F::BtlMove_SetDirection(c, p_chr_1, 3i32);
            F::BtlMove_Advance(c, p_chr_1, l_speed_6, l_accel_7);
            F::BtlMove_ApplyGravity(c, p_chr_1);
        } else {
            F::BtlMove_Step(c, p_chr_1, 6i32, 6i32, 3i32, l_speed_6, l_accel_7);
            F::BtlMove_ApplyGravity(c, p_chr_1);
        }
        F::BtlChar_SetFlag(c, p_chr_1, ((137i32) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((9i32) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((213i32) as u32));
        F::BtlChar_SetFxBit(c, p_chr_1, 58i32);
    }
    if (((((p_phase_2) == (2i32)) as i32)) != 0) {
        's3: {
            let sv16: i32 = F::BtlAnim_GetId(c, p_chr_1);
            match sv16 {
                92i32 | 95i32 | 98i32 | 101i32 | 104i32 => {
                    if ((F::BtlInput_TestAction(c, p_chr_1, 63i32, 1i32)) != 0) {
                        { let r_ = 2i32; let a_ = ((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    }
                    break 's3;
                }
                93i32 | 96i32 | 99i32 | 102i32 | 105i32 => {
                    if ((F::BtlInput_TestAction(c, p_chr_1, 63i32, 1i32)) != 0) {
                        { let r_ = 2i32; let a_ = ((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    }
                    if ((((!((F::BtlChar_TestFlag(c, p_chr_1, ((5i32) as u32))) != 0)) as i32)) != 0) {
                        { let r_ = 2i32; let a_ = ((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    }
                    if (((((c.m.r32(((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) & (2i32))) != 0) {
                        { let a1_ = (F::BtlAnim_GetId(c, p_chr_1)).wrapping_add(1i32); F::BtlAnim_Request(c, p_chr_1, a1_, F32(0x00000000)) };
                    }
                    break 's3;
                }
                94i32 | 97i32 | 100i32 | 103i32 | 106i32 => {
                    if ((F::BtlChar_TestFlag(c, p_chr_1, ((91i32) as u32))) != 0) {
                        F::BtlChar_SetFlag(c, p_chr_1, ((61i32) as u32));
                        's4: {
                            let sv17: i32 = F::BtlOpp_GetSeenAction(c, p_chr_1);
                            match sv17 {
                                213i32 => {
                                    if ((((!((F::BtlAct_HasQueued(c, p_chr_1)) != 0)) as i32)) != 0) {
                                        { let _ = F::BtlDecide_Attack(c, p_chr_1, ((((((1572864i32) as u32)) as u64)) as i32)); };
                                    }
                                    if (((rt::flt(F32(0x3f333333), F::BtlAnim_GetProgress(c, p_chr_1)) as i32)) != 0) {
                                        { let a1_ = F::BtlAct_GetQueued(c, p_chr_1); F::BtlAct_Request(c, p_chr_1, a1_) };
                                    }
                                    break 's4;
                                }
                                198i32 => {
                                    if ((((!((F::BtlAct_HasQueued(c, p_chr_1)) != 0)) as i32)) != 0) {
                                        { let _ = F::BtlDecide_Main(c, p_chr_1, ((((((32768i32) as u32)) as u64)) as i32)); };
                                        { let _ = F::BtlDecide_Attack(c, p_chr_1, ((((((1i32) as u32)) as u64)) as i32)); };
                                    }
                                    if (((rt::flt(F32(0x3f199999), F::BtlAnim_GetProgress(c, p_chr_1)) as i32)) != 0) {
                                        { let a1_ = F::BtlAct_GetQueued(c, p_chr_1); F::BtlAct_Request(c, p_chr_1, a1_) };
                                    }
                                    break 's4;
                                }
                                _ => {
                                }
                            }
                        }
                    }
                    break 's3;
                }
                _ => {
                }
            }
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_DashSmashHandler", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_FlyingKickHandler(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_FlyingKickHandler", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_cancelAt_3: u32 = 0;
    let mut l_steer_4: i32 = 0;
    let mut l_i_5: i32 = 0;
    let mut l_lean_6: F32 = F32(0);
    let mut l_speed_7: F32 = F32(0);
    let mut l_closeSpeed_8: F32 = F32(0);
    let mut l_yawAccel_9: F32 = F32(0);
    let mut l_scale_10: F32 = F32(0);
    let r_ = 'f: {
        l_cancelAt_3 = ((p_chr_1).wrapping_add(1000)).wrapping_add((((0i32) as u32)).wrapping_mul(4));
        if (((((p_phase_2) == (0i32)) as i32)) != 0) {
            F::BtlAnim_Play(c, p_chr_1, 107i32, F32(0x3e199999));
            F::BtlChar_SetHeldFlag(c, p_chr_1, ((14i32) as u32));
            F::BtlCharSnd_PlayCommon(c, p_chr_1, 46i32);
            if ((F::BtlChar_TestFlag(c, p_chr_1, ((5i32) as u32))) != 0) {
                if (((rt::flt({ let a0_ = F::BtlOpp_GetYawFromFacing(c, p_chr_1); rt::ffabs(a0_) }, F::BtlMoveParam_GetAngle58(c, p_chr_1)) as i32)) != 0) {
                    { let r_ = 1i32; let a_ = ((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                }
            }
        }
        if (((((p_phase_2) == (1i32)) as i32)) != 0) {
            l_lean_6 = F32(0x00000000);
            l_speed_7 = { let a1_ = (if ((F::BtlChar_TestFlag(c, p_chr_1, ((17i32) as u32))) != 0) { 5i32 } else { 4i32 }); F::BtlMoveParam_GetSpeed(c, p_chr_1, a1_) };
            l_closeSpeed_8 = l_speed_7;
            l_steer_4 = 0i32;
            if (((((c.m.r32(((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) & (1i32))) != 0) {
                l_steer_4 = 1i32;
            }
            's1: {
                let sv11: i32 = F::BtlAnim_GetId(c, p_chr_1);
                match sv11 {
                    107i32 => {
                        { let _ = F::BtlAnim_AdvanceThen(c, p_chr_1, 108i32, F32(0x00000000), 0i32); };
                        F::BtlChar_SetFxBit(c, p_chr_1, 11i32);
                        l_lean_6 = F32(0x3f800000);
                        break 's1;
                    }
                    108i32 => {
                        if ((F::BtlAnim_AdvanceThen(c, p_chr_1, 109i32, F32(0x00000000), 0i32)) != 0) {
                            F::BtlCharSnd_PlayCommon(c, p_chr_1, 47i32);
                            F::BtlChar_SetVibration(c, p_chr_1, F32(0x3f800000), F32(0x3e999999));
                        }
                        F::BtlAct_SetChargeFromAnim(c, p_chr_1);
                        F::BtlChar_SetFxBit(c, p_chr_1, 11i32);
                        l_lean_6 = F32(0x3f800000);
                        break 's1;
                    }
                    109i32 => {
                        if ((F::BtlAnim_Advance(c, p_chr_1, 0i32)) != 0) {
                            F::BtlAct_Request(c, p_chr_1, 11i32);
                        }
                        if ((F::BtlAnim_IsNew(c, p_chr_1)) != 0) {
                            F::BtlAct_PlayPickedVoice(c, p_chr_1);
                        }
                        if ((F::BtlChar_TestFlag(c, p_chr_1, ((91i32) as u32))) != 0) {
                            l_speed_7 = F32(0x00000000);
                            { let v_ = l_speed_7; let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(152); c.m.w32(a_, (v_).0); };
                        } else {
                            if ((((((F::BtlChar_TestFlag(c, p_chr_1, ((92i32) as u32))) != 0) || ((F::BtlChar_TestFlag(c, p_chr_1, ((93i32) as u32))) != 0)) as i32)) != 0) {
                                l_speed_7 = F32(0x00000000);
                                l_steer_4 = 0i32;
                                if ((F::BtlMove_IsBlockedByOpponent(c, p_chr_1)) != 0) {
                                    { let v_ = F32(0x00000000); let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(152); c.m.w32(a_, (v_).0); };
                                }
                            } else {
                                F::BtlChar_SetFxBit(c, p_chr_1, 11i32);
                            }
                        }
                        l_lean_6 = rt::fsub(F32(0x3f800000), F::BtlAnim_GetProgress(c, p_chr_1));
                        break 's1;
                    }
                    _ => {
                    }
                }
            }
            if ((((((l_steer_4) != 0) && ((F::BtlChar_TestFlag(c, p_chr_1, ((5i32) as u32))) != 0)) as i32)) != 0) {
                l_yawAccel_9 = F32(0x3e20d97a);
                if ((((((c.m.r32((p_chr_1).wrapping_add(2404)) as i32)) == (0i32)) as i32)) != 0) {
                    l_yawAccel_9 = F32(0x40490fda);
                }
                l_scale_10 = { let a0_ = rt::fmul(F::BtlOpp_GetGapXZ(c, p_chr_1), F32(0x3c23d70a)); F::BtlUtil_MinF(c, a0_, F32(0x3f800000)) };
                F::BtlMove_SteerAtOpponent(c, p_chr_1, l_closeSpeed_8, l_yawAccel_9, F32(0x00000000), rt::fmul(l_scale_10, F32(0x3f714637)), F32(0x00000000), F32(0x40490fda));
                F::BtlMove_TurnModelYaw(c, p_chr_1, F32(0x3f20d97a), F32(0x3f000000));
                F::BtlMove_SetDirection(c, p_chr_1, 3i32);
                F::BtlMove_Advance(c, p_chr_1, l_speed_7, F32(0x3f6d0979));
            } else {
                F::BtlMove_Step(c, p_chr_1, 6i32, 6i32, 3i32, l_speed_7, F32(0x3f6d0979));
            }
            F::BtlMove_ApplyGravity(c, p_chr_1);
            F::BtlChar_SetFlag(c, p_chr_1, ((213i32) as u32));
            { let a1_ = rt::fmul({ let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(144); F32(c.m.r32(a_)) }, l_lean_6); F::BtlMove_SetLeanX(c, p_chr_1, a1_) };
            F::BtlChar_SetFxBit(c, p_chr_1, 58i32);
        }
        if (((((p_phase_2) == (2i32)) as i32)) != 0) {
            's2: {
                let sv12: i32 = F::BtlAnim_GetId(c, p_chr_1);
                match sv12 {
                    107i32 => {
                        break 's2;
                    }
                    108i32 => {
                        if ((F::BtlInput_TestAction(c, p_chr_1, 65i32, 1i32)) != 0) {
                            F::BtlAnim_Request(c, p_chr_1, 109i32, F32(0x00000000));
                        }
                        break 's2;
                    }
                    109i32 => {
                        if ((F::BtlChar_TestFlag(c, p_chr_1, ((91i32) as u32))) != 0) {
                            rt::memset(c, fp, 0, 8);
                            { let v_ = 135i32; c.m.w32(fp.wrapping_add(0), (v_) as u32); }
                            { let v_ = 119i32; c.m.w32(fp.wrapping_add(4), (v_) as u32); }
                            {
                                l_i_5 = 0i32;
                                'l3: loop {
                                    if !(((((l_i_5) < (2i32)) as i32)) != 0) { break 'l3; }
                                    'c4: {
                                        if (({ let a1_ = (c.m.r32((fp).wrapping_add((((l_i_5) as u32)).wrapping_mul(4))) as i32); F::BtlDecide_QueueAttack(c, p_chr_1, a1_) }) != 0) {
                                            { let v_ = F32(0x3ecccccc); let a_ = l_cancelAt_3; c.m.w32(a_, (v_).0); };
                                            break 'l3;
                                        }
                                    }
                                    { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
                                }
                            }
                            if ((F::BtlDecide_Attack(c, p_chr_1, ((((((1i32) as u32)) as u64)) as i32))) != 0) {
                                { let v_ = F32(0x3f199999); let a_ = l_cancelAt_3; c.m.w32(a_, (v_).0); };
                            }
                            if ((F::BtlAct_HasQueued(c, p_chr_1)) != 0) {
                                if (((rt::flt(F32(c.m.r32(l_cancelAt_3)), F::BtlAnim_GetProgress(c, p_chr_1)) as i32)) != 0) {
                                    { let a1_ = F::BtlAct_GetQueued(c, p_chr_1); F::BtlAct_Request(c, p_chr_1, a1_) };
                                }
                            }
                        }
                        F::BtlAct_CancelOnOppC9(c, p_chr_1, F32(0x3f199999));
                        break 's2;
                    }
                    _ => {
                    }
                }
            }
        }
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_FlyingKickHandler", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_LiftStrikeHandler(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_LiftStrikeHandler", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let mut l_cancelAt_3: u32 = 0;
    let mut l_pose_4: u32 = 0;
    let mut l_falling_5: i32 = 0;
    let mut l_steer_6: i32 = 0;
    let mut l_speed_7: F32 = F32(0);
    let mut l_turn_8: F32 = F32(0);
    let r_ = 'f: {
    l_cancelAt_3 = ((p_chr_1).wrapping_add(1000)).wrapping_add((((0i32) as u32)).wrapping_mul(4));
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        l_pose_4 = F::BtlChar_GetPos(c, p_chr_1);
        F::BtlAnim_Play(c, p_chr_1, 110i32, F32(0x3e199999));
        F::BtlChar_ClearFlag(c, p_chr_1, ((14i32) as u32));
        F::BtlCharSnd_PlayCommon(c, p_chr_1, 46i32);
        { let v_ = F32(0x00000000); let a_ = (l_pose_4).wrapping_add(156); c.m.w32(a_, (v_).0); };
        if ((((((F::BtlChar_TestFlag(c, p_chr_1, ((5i32) as u32))) != 0) && (((rt::flt({ let a0_ = F::BtlOpp_GetYawFromFacing(c, p_chr_1); rt::ffabs(a0_) }, F::BtlMoveParam_GetAngle58(c, p_chr_1)) as i32)) != 0)) as i32)) != 0) {
            l_turn_8 = F32(0x40490fda);
            F::BtlMove_TurnYaw(c, p_chr_1, 2i32, l_turn_8);
            F::BtlMove_TurnPitch(c, p_chr_1, 1i32, l_turn_8);
            { let v_ = { let a0_ = F32(c.m.r32((l_pose_4).wrapping_add(144))); F::BtlUtil_ClampF(c, a0_, F32(0xbfb4f4aa), F32(0xbef14637)) }; let a_ = (l_pose_4).wrapping_add(144); c.m.w32(a_, (v_).0); };
            { let r_ = 1i32; let a_ = ((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        } else {
            { let a1_ = F32(c.m.r32((l_pose_4).wrapping_add(148))); F::BtlMove_SetHeading(c, p_chr_1, a1_, F32(0xbf20d97a)) };
        }
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        l_speed_7 = F::BtlMoveParam_GetSpeed(c, p_chr_1, 4i32);
        l_falling_5 = 0i32;
        l_steer_6 = 1i32;
        's1: {
            let sv9: i32 = F::BtlAnim_GetId(c, p_chr_1);
            match sv9 {
                110i32 => {
                    { let _ = F::BtlAnim_AdvanceThen(c, p_chr_1, 111i32, F32(0x00000000), 0i32); };
                    break 's1;
                }
                111i32 => {
                    if ((F::BtlAnim_AdvanceThen(c, p_chr_1, 112i32, F32(0x00000000), 0i32)) != 0) {
                        F::BtlCharSnd_PlayCommon(c, p_chr_1, 47i32);
                        F::BtlChar_SetVibration(c, p_chr_1, F32(0x3f800000), F32(0x3e999999));
                    }
                    F::BtlAct_SetChargeFromAnim(c, p_chr_1);
                    break 's1;
                }
                112i32 => {
                    if ((F::BtlAnim_Advance(c, p_chr_1, 0i32)) != 0) {
                        F::BtlAct_Request(c, p_chr_1, 11i32);
                    }
                    if ((F::BtlAnim_IsNew(c, p_chr_1)) != 0) {
                        F::BtlAct_PlayPickedVoice(c, p_chr_1);
                    }
                    l_speed_7 = F32(0x00000000);
                    l_falling_5 = 1i32;
                    if ((F::BtlChar_TestFlag(c, p_chr_1, ((91i32) as u32))) != 0) {
                        { let v_ = l_speed_7; let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(152); c.m.w32(a_, (v_).0); };
                    } else {
                        if ((((((F::BtlChar_TestFlag(c, p_chr_1, ((92i32) as u32))) != 0) || ((F::BtlChar_TestFlag(c, p_chr_1, ((93i32) as u32))) != 0)) as i32)) != 0) {
                            l_steer_6 = 0i32;
                            if ((F::BtlMove_IsBlockedByOpponent(c, p_chr_1)) != 0) {
                                { let v_ = l_speed_7; let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(152); c.m.w32(a_, (v_).0); };
                            }
                        }
                    }
                    break 's1;
                }
                _ => {
                }
            }
        }
        { let a1_ = (if (((((((((c.m.r32(((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) & (1i32))) != 0) && ((l_steer_6) != 0)) as i32)) != 0) { 2i32 } else { 6i32 }); F::BtlMove_Step(c, p_chr_1, a1_, 6i32, 3i32, l_speed_7, F32(0x3f6d0979)) };
        if ((((((l_falling_5) != 0) && ((((!((F::BtlChar_TestFlag(c, p_chr_1, ((14i32) as u32))) != 0)) as i32)) != 0)) as i32)) != 0) {
            F::BtlMove_Fall(c, p_chr_1);
        } else {
            F::BtlMove_BrakeVertical(c, p_chr_1);
        }
        F::BtlChar_SetFlag(c, p_chr_1, ((213i32) as u32));
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((17i32) as u32))) != 0) {
            F::BtlChar_SetHeldFlag(c, p_chr_1, ((14i32) as u32));
        }
        F::BtlChar_SetFxBit(c, p_chr_1, 58i32);
    }
    if (((((p_phase_2) == (2i32)) as i32)) != 0) {
        's2: {
            let sv10: i32 = F::BtlAnim_GetId(c, p_chr_1);
            match sv10 {
                110i32 => {
                    break 's2;
                }
                111i32 => {
                    if ((F::BtlInput_TestAction(c, p_chr_1, 65i32, 1i32)) != 0) {
                        F::BtlAnim_Request(c, p_chr_1, 112i32, F32(0x00000000));
                    }
                    break 's2;
                }
                112i32 => {
                    if ((F::BtlChar_TestFlag(c, p_chr_1, ((91i32) as u32))) != 0) {
                        if ((((((((F::BtlParam_GetFlags2(c, p_chr_1)) as u64)) as i32)) & (4194304i32))) != 0) {
                            F::BtlColl_AddActionBit(c, p_chr_1, 95i32);
                            if ((F::BtlInput_TestAction(c, p_chr_1, 80i32, 1i32)) != 0) {
                                F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), 95i32);
                                { let v_ = F32(0x3e999999); let a_ = l_cancelAt_3; c.m.w32(a_, (v_).0); };
                            }
                        }
                        if ((F::BtlDecide_Attack(c, p_chr_1, ((((((1i32) as u32)) as u64)) as i32))) != 0) {
                            { let v_ = F32(0x3f199999); let a_ = l_cancelAt_3; c.m.w32(a_, (v_).0); };
                        }
                        if ((F::BtlAct_HasQueued(c, p_chr_1)) != 0) {
                            if (((rt::flt(F32(c.m.r32(l_cancelAt_3)), F::BtlAnim_GetProgress(c, p_chr_1)) as i32)) != 0) {
                                { let a1_ = F::BtlAct_GetQueued(c, p_chr_1); F::BtlAct_Request(c, p_chr_1, a1_) };
                            }
                        }
                    }
                    F::BtlAct_CancelOnOppC9(c, p_chr_1, F32(0x3f199999));
                    break 's2;
                }
                _ => {
                }
            }
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_LiftStrikeHandler", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_SmashVanishHandler(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_SmashVanishHandler", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let mut l_anim_3: u32 = 0;
    let mut l_step_4: u32 = 0;
    let mut l_lead_5: u32 = 0;
    let mut l_count_6: u32 = 0;
    let mut l_obj_7: u32 = 0;
    let mut l_move_8: i32 = 0;
    let mut l_a_9: i32 = 0;
    let mut l_turn_10: F32 = F32(0);
    let r_ = 'f: {
    l_anim_3 = ((p_chr_1).wrapping_add(976)).wrapping_add((((4i32) as u32)).wrapping_mul(4));
    l_step_4 = ((p_chr_1).wrapping_add(976)).wrapping_add((((2i32) as u32)).wrapping_mul(4));
    l_lead_5 = ((p_chr_1).wrapping_add(1000)).wrapping_add((((0i32) as u32)).wrapping_mul(4));
    l_count_6 = ((p_chr_1).wrapping_add(976)).wrapping_add((((3i32) as u32)).wrapping_mul(4));
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        's1: {
            let sv11: i32 = F::BtlAct_GetCurrent(c, p_chr_1);
            match sv11 {
                90i32 => {
                    { let v_ = 101i32; let a_ = l_anim_3; c.m.w32(a_, (v_) as u32); };
                    break 's1;
                }
                91i32 => {
                    { let v_ = 104i32; let a_ = l_anim_3; c.m.w32(a_, (v_) as u32); };
                    break 's1;
                }
                92i32 => {
                    { let v_ = 95i32; let a_ = l_anim_3; c.m.w32(a_, (v_) as u32); };
                    break 's1;
                }
                93i32 => {
                    { let v_ = 98i32; let a_ = l_anim_3; c.m.w32(a_, (v_) as u32); };
                    break 's1;
                }
                _ => {
                }
            }
        }
        's2: {
            let sv12: i32 = F::BtlAct_GetPrev(c, p_chr_1);
            match sv12 {
                71i32 | 72i32 | 73i32 | 74i32 | 75i32 | 76i32 => {
                    F::BtlChar_PlayVoice(c, p_chr_1, 29i32);
                    break 's2;
                }
                _ => {
                    F::BtlChar_PlayVoice(c, p_chr_1, 5i32);
                    break 's2;
                }
            }
        }
        F::BtlChar_SetFxBit(c, p_chr_1, 12i32);
        F::BtlCharSnd_PlayCommon(c, p_chr_1, 32i32);
        F::BtlChar_SetSmallVibration(c, p_chr_1, F32(0x3e4ccccc));
        { let v_ = F32(0x00000000); let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(152); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(156); c.m.w32(a_, (v_).0); };
        F::BtlChar_SetHeldFlag(c, p_chr_1, ((14i32) as u32));
        { let a_ = (p_chr_1).wrapping_add(3440); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        { let v_ = F::BtlAct_GetCurrent(c, p_chr_1); let a_ = (p_chr_1).wrapping_add(3472); c.m.w32(a_, (v_) as u32); };
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        l_obj_7 = F::BtlChar_GetObj(c, p_chr_1);
        's3: {
            let sv13: i32 = (c.m.r32(l_step_4) as i32);
            match sv13 {
                0i32 => {
                    { let _ = F::BtlAnim_Advance(c, p_chr_1, 0i32); };
                    { let a_ = l_count_6; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    if ((((((c.m.r32(l_count_6) as i32)) == (1i32)) as i32)) != 0) {
                        { let a1_ = (c.m.r32(l_anim_3) as i32); F::BtlAnim_PlaySub(c, p_chr_1, a1_) };
                        { let r_ = rt::fdiv(rt::fmul(F32(c.m.r32((l_obj_7).wrapping_add(3036))), F32(0x41f00000)), F32(0x42700000)); let a_ = l_lead_5; let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    }
                    if ((((((c.m.r32(l_count_6) as i32)) == (2i32)) as i32)) != 0) {
                        { let a1_ = ((c.m.r32(l_anim_3) as i32)).wrapping_add(2i32); F::BtlAnim_PlaySub(c, p_chr_1, a1_) };
                        l_a_9 = F::BtlObjAnim_QueryEvent(c, l_obj_7, ((1i32) as u64), 1i32, ((((0i32) as i64 as u64)) as u32));
                        { let r_ = F::BtlObjAnim_QueryEvent(c, l_obj_7, ((2i32) as u64), 1i32, ((((0i32) as i64 as u64)) as u32)); let n_ = (l_a_9).wrapping_add(r_); l_a_9 = n_; };
                        { let r_ = rt::fdiv(rt::fmul(rt::fmul(rt::i2f(l_a_9), F32(0x3f000000)), F32(0x41f00000)), F32(0x42700000)); let a_ = l_lead_5; let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                        { let v_ = { let a0_ = rt::fadd(F32(c.m.r32(l_lead_5)), F32(0x3f000000)); F::Port_floorf(c, a0_) }; let a_ = l_lead_5; c.m.w32(a_, (v_).0); };
                    }
                    if ((((((c.m.r32(l_count_6) as i32)) >= (4i32)) as i32)) != 0) {
                        F::BtlChar_SetFlag(c, p_chr_1, ((11i32) as u32));
                    }
                    if ((((((c.m.r32(l_count_6) as i32)) >= (8i32)) as i32)) != 0) {
                        { let v_ = 0i32; let a_ = l_count_6; c.m.w32(a_, (v_) as u32); };
                        { let a_ = l_step_4; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    }
                    break 's3;
                }
                1i32 => {
                    F::BtlMove_SnapToOpponent(c, p_chr_1);
                    F::BtlChar_SetFlag(c, p_chr_1, ((11i32) as u32));
                    F::BtlChar_SetFlag(c, p_chr_1, ((204i32) as u32));
                    { let a_ = l_step_4; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    break 's3;
                }
                2i32 => {
                    l_turn_10 = F32(0x40490fda);
                    { let a1_ = F32(c.m.r32(l_lead_5)); F::BtlMove_WarpAheadOfOpponent(c, p_chr_1, a1_) };
                    F::BtlMove_TurnYaw(c, p_chr_1, 2i32, l_turn_10);
                    F::BtlMove_TurnPitch(c, p_chr_1, 1i32, l_turn_10);
                    F::BtlMove_TurnModelYaw(c, p_chr_1, l_turn_10, F32(0x3f800000));
                    F::BtlMove_SetDirection(c, p_chr_1, 3i32);
                    F::BtlChar_SetFlag(c, p_chr_1, ((63i32) as u32));
                    F::BtlChar_SetFlag(c, p_chr_1, ((11i32) as u32));
                    { let a1_ = (c.m.r32(l_anim_3) as i32); F::BtlAnim_Request(c, p_chr_1, a1_, F32(0x00000000)) };
                    F::BtlChar_SetFxBit(c, p_chr_1, 13i32);
                    F::BtlChar_SetFlag(c, p_chr_1, ((204i32) as u32));
                    F::BtlChar_SetFlag(c, p_chr_1, ((36i32) as u32));
                    { let a_ = l_step_4; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    break 's3;
                }
                3i32 => {
                    l_move_8 = 1i32;
                    's4: {
                        let sv14: i32 = F::BtlAnim_GetId(c, p_chr_1);
                        match sv14 {
                            95i32 | 98i32 | 101i32 | 104i32 => {
                                { let _ = { let a1_ = (F::BtlAnim_GetId(c, p_chr_1)).wrapping_add(2i32); F::BtlAnim_AdvanceThen(c, p_chr_1, a1_, F32(0x00000000), 0i32) }; };
                                { let v_ = F32(0x3f800000); let a_ = (p_chr_1).wrapping_add(3448); c.m.w32(a_, (v_).0); };
                                if (((rt::flt(F::BtlAnim_GetProgress(c, p_chr_1), F32(0x3f333333)) as i32)) != 0) {
                                    F::BtlChar_SetFlag(c, p_chr_1, ((11i32) as u32));
                                }
                                if ((F::BtlAnim_IsNew(c, p_chr_1)) != 0) {
                                    F::BtlChar_SetFlag(c, p_chr_1, ((205i32) as u32));
                                }
                                break 's4;
                            }
                            97i32 | 100i32 | 103i32 | 106i32 => {
                                if ((F::BtlAnim_Advance(c, p_chr_1, 0i32)) != 0) {
                                    's5: {
                                        let sv15: i32 = F::BtlOpp_GetSeenAction(c, p_chr_1);
                                        match sv15 {
                                            137i32 | 138i32 | 139i32 | 140i32 => {
                                                F::BtlAct_Request(c, p_chr_1, 12i32);
                                                break 's5;
                                            }
                                            _ => {
                                                F::BtlAct_Request(c, p_chr_1, 11i32);
                                                break 's5;
                                            }
                                        }
                                    }
                                }
                                F::BtlAnim_SetObjRate(c, p_chr_1, F32(0x40400000));
                                if ((F::BtlChar_TestFlag(c, p_chr_1, ((91i32) as u32))) != 0) {
                                    { let r_ = 1i32; let a_ = ((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                                }
                                l_move_8 = ((!((F::BtlChar_TestFlag(c, p_chr_1, ((92i32) as u32))) != 0)) as i32);
                                break 's4;
                            }
                            96i32 | 99i32 | 102i32 | 105i32 => {
                                break 's4;
                            }
                            _ => {
                            }
                        }
                    }
                    if ((l_move_8) != 0) {
                        F::BtlMove_Step(c, p_chr_1, 2i32, 1i32, 5i32, F32(0x00000000), F32(0x3f6d0979));
                    } else {
                        F::BtlMove_Step(c, p_chr_1, 6i32, 6i32, 5i32, F32(0x00000000), F32(0x3f6d0979));
                    }
                    break 's3;
                }
                _ => {
                }
            }
        }
        F::BtlChar_SetFxBit(c, p_chr_1, 58i32);
        F::BtlChar_SetFlag(c, p_chr_1, ((137i32) as u32));
    }
    if (((((p_phase_2) == (2i32)) as i32)) != 0) {
        if (((((c.m.r32(((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) & (1i32))) != 0) {
            if ((F::BtlChar_TestFlag(c, p_chr_1, ((91i32) as u32))) != 0) {
                F::BtlChar_SetFlag(c, p_chr_1, ((61i32) as u32));
                if (((((F::BtlOpp_GetSeenAction(c, p_chr_1)) == (213i32)) as i32)) != 0) {
                    if ((((!((F::BtlAct_HasQueued(c, p_chr_1)) != 0)) as i32)) != 0) {
                        { let _ = F::BtlDecide_Attack(c, p_chr_1, ((((((1572864i32) as u32)) as u64)) as i32)); };
                    }
                    if (((rt::flt(F32(0x3f333333), F::BtlAnim_GetProgress(c, p_chr_1)) as i32)) != 0) {
                        { let a1_ = F::BtlAct_GetQueued(c, p_chr_1); F::BtlAct_Request(c, p_chr_1, a1_) };
                    }
                }
            }
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_SmashVanishHandler", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_RushFinishHandler(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_RushFinishHandler", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_cancelAt_3: u32 = 0;
    let mut l_obj_4: u32 = 0;
    let mut l_pose_5: u32 = 0;
    let mut l_anim_6: i32 = 0;
    let mut l_mode_7: i32 = 0;
    let mut l_count_8: i32 = 0;
    let mut l_i_9: i32 = 0;
    let mut l_speed_10: F32 = F32(0);
    let mut l_turn_11: F32 = F32(0);
    let r_ = 'f: {
        l_cancelAt_3 = ((p_chr_1).wrapping_add(1000)).wrapping_add((((0i32) as u32)).wrapping_mul(4));
        l_obj_4 = F::BtlChar_GetObj(c, p_chr_1);
        if (((((p_phase_2) == (0i32)) as i32)) != 0) {
            l_anim_6 = 0i32;
            's1: {
                let sv12: i32 = F::BtlAct_GetCurrent(c, p_chr_1);
                match sv12 {
                    94i32 => {
                        l_anim_6 = 65i32;
                        break 's1;
                    }
                    95i32 => {
                        l_anim_6 = 68i32;
                        break 's1;
                    }
                    96i32 => {
                        l_anim_6 = 71i32;
                        break 's1;
                    }
                    97i32 | 98i32 => {
                        l_anim_6 = 144i32;
                        break 's1;
                    }
                    102i32 => {
                        l_anim_6 = 409i32;
                        break 's1;
                    }
                    100i32 => {
                        l_anim_6 = 86i32;
                        F::BtlChar_ToggleHeldFlag(c, p_chr_1, ((143i32) as u32));
                        break 's1;
                    }
                    101i32 => {
                        l_anim_6 = 89i32;
                        F::BtlChar_ToggleHeldFlag(c, p_chr_1, ((143i32) as u32));
                        break 's1;
                    }
                    _ => {
                    }
                }
            }
            F::BtlAnim_Play(c, p_chr_1, l_anim_6, F32(0x3dcccccc));
            F::BtlMove_BeginRiseToOpponent(c, p_chr_1);
            F::BtlCharSnd_PlayCommon(c, p_chr_1, 46i32);
            if (((((F::BtlAct_GetCurrent(c, p_chr_1)) == (102i32)) as i32)) != 0) {
                { let v_ = F::BtlAct_StartSub19B(c, p_chr_1); let a_ = (p_chr_1).wrapping_add(4948); c.m.w32(a_, (v_) as u32); };
            }
        }
        if (((((p_phase_2) == (1i32)) as i32)) != 0) {
            l_speed_10 = F32(0x00000000);
            l_mode_7 = 6i32;
            l_turn_11 = l_speed_10;
            if ((F::BtlChar_TestFlag(c, p_chr_1, ((129i32) as u32))) != 0) {
                l_speed_10 = F32(0x3eed0979);
                l_mode_7 = 2i32;
                l_turn_11 = F32(0x3d00adfb);
            }
            's2: {
                let sv13: i32 = F::BtlAnim_GetId(c, p_chr_1);
                match sv13 {
                    65i32 | 68i32 | 71i32 | 86i32 | 89i32 | 144i32 | 409i32 => {
                        F::BtlAct_SetChargeAnimStep(c, p_chr_1);
                        { let _ = { let a1_ = (F::BtlAnim_GetId(c, p_chr_1)).wrapping_add(1i32); F::BtlAnim_AdvanceThen(c, p_chr_1, a1_, F32(0x00000000), 0i32) }; };
                        break 's2;
                    }
                    66i32 | 69i32 | 72i32 | 87i32 | 90i32 | 145i32 | 410i32 => {
                        F::BtlAct_SetChargeAnimStep(c, p_chr_1);
                        { let _ = { let a1_ = (F::BtlAnim_GetId(c, p_chr_1)).wrapping_add(1i32); F::BtlAnim_AdvanceThen(c, p_chr_1, a1_, F32(0x00000000), 0i32) }; };
                        F::BtlAct_SetChargeFromAnim(c, p_chr_1);
                        if (((((F::BtlAct_GetCurrent(c, p_chr_1)) == (102i32)) as i32)) != 0) {
                            if ((F::BtlAnim_IsNew(c, p_chr_1)) != 0) {
                                F::BtlChar_SetFxBit(c, p_chr_1, 40i32);
                            }
                        }
                        break 's2;
                    }
                    67i32 | 70i32 | 73i32 | 88i32 | 91i32 | 146i32 | 411i32 => {
                        l_speed_10 = F32(0x00000000);
                        l_turn_11 = l_speed_10;
                        l_mode_7 = 6i32;
                        if (((((F::BtlAct_GetCurrent(c, p_chr_1)) == (102i32)) as i32)) != 0) {
                            if ((F::BtlAnim_IsNew(c, p_chr_1)) != 0) {
                                F::BtlAct_PlayPickedVoice(c, p_chr_1);
                                F::BtlChar_SetFxBit(c, p_chr_1, 41i32);
                            }
                            F::BtlChar_SetVibration(c, p_chr_1, F32(0x3f800000), F32(0x3dcccccc));
                            { let _ = { let a1_ = (F::BtlAnim_GetId(c, p_chr_1)).wrapping_add(1i32); F::BtlAnim_AdvanceThen(c, p_chr_1, a1_, F32(0x00000000), 0i32) }; };
                            l_speed_10 = F::BtlAtk_GetLaunchSpeed(c, p_chr_1);
                            if ((F::BtlChar_TestFlag(c, p_chr_1, ((19i32) as u32))) != 0) {
                                F::BtlChar_SetFlag(c, p_chr_1, ((220i32) as u32));
                            }
                        } else {
                            if ((F::BtlAnim_IsNew(c, p_chr_1)) != 0) {
                                F::BtlAct_PlayPickedVoice(c, p_chr_1);
                                l_pose_5 = F::BtlChar_GetPos(c, p_chr_1);
                                { let v_ = F::BtlAtk_GetLaunchSpeed(c, p_chr_1); let a_ = (l_pose_5).wrapping_add(152); c.m.w32(a_, (v_).0); };
                                if (((rt::flt(F32(0x3f7d70a3), F32(c.m.r32((p_chr_1).wrapping_add(3448)))) as i32)) != 0) {
                                    F::BtlChar_SetVibration(c, p_chr_1, F32(0x3f800000), F32(0x3e999999));
                                }
                            }
                            if ((F::BtlAnim_Advance(c, p_chr_1, 0i32)) != 0) {
                                F::BtlAct_Request(c, p_chr_1, 11i32);
                            }
                        }
                        if ((F::BtlMove_IsBlockedByOpponent(c, p_chr_1)) != 0) {
                            if ((((((F::BtlChar_TestFlag(c, p_chr_1, ((91i32) as u32))) != 0) || ((F::BtlChar_TestFlag(c, p_chr_1, ((93i32) as u32))) != 0)) as i32)) != 0) {
                                l_turn_11 = F32(0x40490fda);
                                l_mode_7 = 2i32;
                            }
                        }
                        break 's2;
                    }
                    412i32 => {
                        if ((F::BtlAnim_IsNew(c, p_chr_1)) != 0) {
                            F::BtlChar_SetFxBit(c, p_chr_1, 42i32);
                        }
                        F::BtlAnim_SetDuration(c, p_chr_1, F32(0x3f000000));
                        if ((F::BtlAnim_Advance(c, p_chr_1, 0i32)) != 0) {
                            F::BtlAct_Request(c, p_chr_1, 11i32);
                        }
                        l_speed_10 = F32(0x00000000);
                        l_mode_7 = 6i32;
                        l_turn_11 = l_speed_10;
                        break 's2;
                    }
                    _ => {
                    }
                }
            }
            F::BtlMove_TurnYaw(c, p_chr_1, l_mode_7, l_turn_11);
            F::BtlMove_TurnPitch(c, p_chr_1, 5i32, F32(0x40490fda));
            F::BtlMove_TurnModelYaw(c, p_chr_1, F32(0x3f20d97a), F32(0x3e999999));
            F::BtlMove_SetDirection(c, p_chr_1, 2i32);
            F::BtlMove_Advance(c, p_chr_1, l_speed_10, F32(0x3f6d0979));
            F::BtlMove_ApplyGravity(c, p_chr_1);
            F::BtlChar_SetFlag(c, p_chr_1, ((213i32) as u32));
            if ((F::BtlChar_TestFlag(c, p_chr_1, ((17i32) as u32))) != 0) {
                F::BtlChar_SetHeldFlag(c, p_chr_1, ((14i32) as u32));
            }
        }
        if (((((p_phase_2) == (2i32)) as i32)) != 0) {
            's3: {
                let sv14: i32 = F::BtlAnim_GetId(c, p_chr_1);
                match sv14 {
                    65i32 | 68i32 | 71i32 | 86i32 | 89i32 | 144i32 | 409i32 => {
                        break 's3;
                    }
                    66i32 | 69i32 | 72i32 | 87i32 | 90i32 | 145i32 | 410i32 => {
                        if ((F::BtlInput_TestAction(c, p_chr_1, 67i32, 1i32)) != 0) {
                            { let a1_ = (F::BtlAnim_GetId(c, p_chr_1)).wrapping_add(1i32); F::BtlAnim_Request(c, p_chr_1, a1_, F32(0x00000000)) };
                        }
                        break 's3;
                    }
                    67i32 => {
                        if ((F::BtlChar_TestFlag(c, p_chr_1, ((91i32) as u32))) != 0) {
                            if (((((F::BtlOpp_GetSeenAction(c, p_chr_1)) == (206i32)) as i32)) != 0) {
                                F::BtlColl_AddActionBit(c, p_chr_1, 52i32);
                                if ((F::BtlInput_TestAction(c, p_chr_1, 42i32, 1i32)) != 0) {
                                    l_speed_10 = rt::i2f(F::BtlObjAnim_QueryEvent(c, l_obj_4, ((1i32) as u64), 0i32, ((((0i32) as i64 as u64)) as u32)));
                                    l_turn_11 = F::BtlAnim_GetLength(c, p_chr_1);
                                    { let r_ = F32(0x41a00000); let n_ = rt::fadd(l_speed_10, r_); l_speed_10 = n_; };
                                    { let r_ = l_turn_11; let n_ = rt::fdiv(l_speed_10, r_); l_speed_10 = n_; };
                                    { let v_ = l_speed_10; let a_ = l_cancelAt_3; c.m.w32(a_, (v_).0); };
                                    { let v_ = { let a0_ = F32(c.m.r32(l_cancelAt_3)); F::BtlUtil_ClampF(c, a0_, F32(0x3e4ccccc), F32(0x3f4ccccc)) }; let a_ = l_cancelAt_3; c.m.w32(a_, (v_).0); };
                                    F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), 52i32);
                                }
                            }
                            if ((F::BtlAct_HasQueued(c, p_chr_1)) != 0) {
                                if (((rt::flt(F32(c.m.r32(l_cancelAt_3)), F::BtlAnim_GetProgress(c, p_chr_1)) as i32)) != 0) {
                                    { let a1_ = F::BtlAct_GetQueued(c, p_chr_1); F::BtlAct_Request(c, p_chr_1, a1_) };
                                }
                            }
                        }
                        F::BtlAct_CancelOnOppC9(c, p_chr_1, F32(0x3f199999));
                        break 's3;
                    }
                    70i32 | 73i32 | 88i32 | 91i32 | 146i32 => {
                        if ((F::BtlChar_TestFlag(c, p_chr_1, ((91i32) as u32))) != 0) {
                            l_count_8 = 0i32;
                            's4: {
                                let sv15: i32 = F::BtlAct_GetCurrent(c, p_chr_1);
                                match sv15 {
                                    94i32 => {
                                        break 's4;
                                    }
                                    95i32 => {
                                        { let v_ = 128i32; let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                        { let v_ = 112i32; let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                        l_count_8 = 2i32;
                                        break 's4;
                                    }
                                    96i32 => {
                                        { let v_ = 126i32; let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                        { let v_ = 116i32; let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                        l_count_8 = 2i32;
                                        break 's4;
                                    }
                                    97i32 => {
                                        if (((rt::flt(F32(0x3f7d70a3), F32(c.m.r32((p_chr_1).wrapping_add(3448)))) as i32)) != 0) {
                                            { let v_ = 115i32; let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                            { let v_ = 150i32; let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                            l_count_8 = 2i32;
                                        }
                                        break 's4;
                                    }
                                    98i32 => {
                                        { let v_ = 130i32; let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                        l_count_8 = 1i32;
                                        break 's4;
                                    }
                                    100i32 => {
                                        { let v_ = 153i32; let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                        { let v_ = 133i32; let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                        l_count_8 = 2i32;
                                        break 's4;
                                    }
                                    101i32 => {
                                        { let v_ = 153i32; let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                        { let v_ = 132i32; let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                        l_count_8 = 2i32;
                                        break 's4;
                                    }
                                    _ => {
                                    }
                                }
                            }
                            {
                                l_i_9 = 0i32;
                                'l5: loop {
                                    if !(((((l_i_9) < (l_count_8)) as i32)) != 0) { break 'l5; }
                                    'c6: {
                                        if (({ let a1_ = (c.m.r32((fp).wrapping_add((((l_i_9) as u32)).wrapping_mul(4))) as i32); F::BtlDecide_QueueAttack(c, p_chr_1, a1_) }) != 0) {
                                            { let v_ = F32(0x3ecccccc); let a_ = l_cancelAt_3; c.m.w32(a_, (v_).0); };
                                            break 'l5;
                                        }
                                    }
                                    { let o_ = l_i_9; let n_ = (o_).wrapping_add(1); l_i_9 = n_; };
                                }
                            }
                            if ((F::BtlDecide_Attack(c, p_chr_1, ((((((3i32) as u32)) as u64)) as i32))) != 0) {
                                { let v_ = F32(0x3f199999); let a_ = l_cancelAt_3; c.m.w32(a_, (v_).0); };
                            }
                            if ((F::BtlAct_HasQueued(c, p_chr_1)) != 0) {
                                if (((rt::flt(F32(c.m.r32(l_cancelAt_3)), F::BtlAnim_GetProgress(c, p_chr_1)) as i32)) != 0) {
                                    { let a1_ = F::BtlAct_GetQueued(c, p_chr_1); F::BtlAct_Request(c, p_chr_1, a1_) };
                                }
                            }
                        }
                        F::BtlAct_CancelOnOppC9(c, p_chr_1, F32(0x3f199999));
                        break 's3;
                    }
                    411i32 => {
                        break 's3;
                    }
                    412i32 => {
                        if ((F::BtlChar_TestFlag(c, p_chr_1, ((91i32) as u32))) != 0) {
                            if (({ let a1_ = (if ((((!(((rt::flt(F::BtlAtk_GetLaunchAngleBOf(c, p_chr_1, ((((134i32) as i64 as u64)) as u32)), F32(0x3e32b8c1)) as i32)) != 0)) as i32)) != 0) { 147i32 } else { 163i32 }); F::BtlDecide_QueueAttack(c, p_chr_1, a1_) }) != 0) {
                                { let v_ = F32(0x3f666666); let a_ = l_cancelAt_3; c.m.w32(a_, (v_).0); };
                            }
                            if (((((F::BtlOpp_GetSeenAction(c, p_chr_1)) != (213i32)) as i32)) != 0) {
                                if ((F::BtlDecide_Attack(c, p_chr_1, ((((((3i32) as u32)) as u64)) as i32))) != 0) {
                                    { let v_ = F32(0x3f199999); let a_ = l_cancelAt_3; c.m.w32(a_, (v_).0); };
                                }
                            }
                            if ((F::BtlAct_HasQueued(c, p_chr_1)) != 0) {
                                if (((rt::flt(F32(c.m.r32(l_cancelAt_3)), F::BtlAnim_GetProgress(c, p_chr_1)) as i32)) != 0) {
                                    { let a1_ = F::BtlAct_GetQueued(c, p_chr_1); F::BtlAct_Request(c, p_chr_1, a1_) };
                                }
                            }
                        }
                        F::BtlAct_CancelOnOppC9(c, p_chr_1, F32(0x3f199999));
                        break 's3;
                    }
                    _ => {
                    }
                }
            }
        }
        if (((((p_phase_2) == (3i32)) as i32)) != 0) {
            if (((((F::BtlAct_GetCurrent(c, p_chr_1)) == (102i32)) as i32)) != 0) {
                F::BtlChar_SetFxBit(c, p_chr_1, 42i32);
            }
        }
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_RushFinishHandler", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_Action63(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_Action63", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let r_ = 'f: {
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        { let _ = D::call(c, fid::BtlAnim_Play, &[((p_chr_1) as u64), ((147i32) as i64 as u64), ((F32(0x3dcccccc)).0 as u64)]); };
        { let _ = D::call(c, fid::BtlMove_BeginRiseToOpponent, &[((p_chr_1) as u64)]); };
        { let _ = D::call(c, fid::BtlAct_PlayPickedVoice, &[((p_chr_1) as u64)]); };
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        if ((((D::call(c, fid::BtlAnim_Advance, &[((p_chr_1) as u64), ((0i32) as i64 as u64)])) as i32)) != 0) {
            { let _ = D::call(c, fid::BtlAct_Request, &[((p_chr_1) as u64), ((11i32) as i64 as u64)]); };
        }
        { let _ = D::call(c, fid::BtlMove_Step, &[((p_chr_1) as u64), ((6i32) as i64 as u64), ((5i32) as i64 as u64), ((2i32) as i64 as u64), ((F32(0x00000000)).0 as u64), ((F32(0x3f6d0979)).0 as u64)]); };
        { let _ = D::call(c, fid::BtlMove_ApplyGravity, &[((p_chr_1) as u64)]); };
        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((213i32) as i64 as u64)]); };
        if ((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((17i32) as i64 as u64)])) as i32)) != 0) {
            { let _ = D::call(c, fid::BtlChar_SetHeldFlag, &[((p_chr_1) as u64), ((14i32) as i64 as u64)]); };
        }
    }
    if (((((p_phase_2) == (2i32)) as i32)) != 0) {
        { let _ = ((D::call(c, fid::BtlDecide_Attack, &[((p_chr_1) as u64), ((((3i32) as u32)) as u64)])) as i32); };
        if (((rt::fgt(F32((D::call(c, fid::BtlAnim_GetProgress, &[((p_chr_1) as u64)])) as u32), F32(0x3f666666)) as i32)) != 0) {
            { let a1_ = ((D::call(c, fid::BtlAct_GetQueued, &[((p_chr_1) as u64)])) as i32); { let _ = D::call(c, fid::BtlAct_Request, &[((p_chr_1) as u64), ((a1_) as i64 as u64)]); } };
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_Action63", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_AttackHandler(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_AttackHandler", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let mut l_atk_3: u32 = 0;
    let mut l_part_4: u32 = 0;
    let mut l_pos_5: u32 = 0;
    let mut l_cut_6: i32 = 0;
    let mut l_homing_7: i32 = 0;
    let mut l_speed_8: F32 = F32(0);
    let mut l_accel_9: F32 = F32(0);
    let r_ = 'f: {
    l_atk_3 = (p_chr_1).wrapping_add(3320);
    l_part_4 = ((p_chr_1).wrapping_add(976)).wrapping_add((((2i32) as u32)).wrapping_mul(4));
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        F::BtlAct_LatchAttack(c, p_chr_1);
        { let a1_ = (((c.m.r16(((l_atk_3).wrapping_add(4)).wrapping_add((((0i32) as u32)).wrapping_mul(2))) as i16)) as i32); { let _ = D::call(c, fid::BtlAnim_Play, &[((p_chr_1) as u64), ((a1_) as i64 as u64), ((F32(0x3e199999)).0 as u64)]); } };
        { let _ = D::call(c, fid::BtlMove_BeginRiseToOpponent, &[((p_chr_1) as u64)]); };
        l_pos_5 = ((D::call(c, fid::BtlChar_GetPos, &[((p_chr_1) as u64)])) as u32);
        { let v_ = F32((D::call(c, fid::BtlAtk_GetLaunchSpeed, &[((p_chr_1) as u64)])) as u32); let a_ = (l_pos_5).wrapping_add(152); c.m.w32(a_, (v_).0); };
        { let _ = D::call(c, fid::BtlAct_PlayPickedVoice, &[((p_chr_1) as u64)]); };
        if (((((c.m.r32(l_atk_3) as i32)) & (128i32))) != 0) {
            { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((205i32) as i64 as u64)]); };
        }
        if ((((((((c.m.r8((l_atk_3).wrapping_add(10)) as i8)) as i32)) >= (0i32)) as i32)) != 0) {
            l_cut_6 = 0i32;
            if ((((((((!(({ let a0_ = ((D::call(c, fid::BtlOpp_GetSeenAction, &[((p_chr_1) as u64)])) as i32); F::BtlAct_IsAttackId(c, a0_) }) != 0)) as i32)) != 0) || ((((((c.m.r32((p_chr_1).wrapping_add(2404)) as i32)) < (F::BtlOpp_GetSeenUnk964(c, p_chr_1))) as i32)) != 0)) as i32)) != 0) {
                l_cut_6 = 1i32;
            }
            if ((l_cut_6) != 0) {
                { let a2_ = (((c.m.r8((l_atk_3).wrapping_add(10)) as i8)) as i32); { let _ = D::call(c, fid::ChrCam_RequestCut, &[((p_chr_1) as u64), ((1i32) as i64 as u64), ((a2_) as i64 as u64)]); } };
                { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((39i32) as i64 as u64)]); };
            }
        }
        's1: {
            let sv10: i32 = (((c.m.r8((l_atk_3).wrapping_add(9)) as i8)) as i32);
            match sv10 {
                1i32 | 2i32 | 3i32 => {
                    { let _ = D::call(c, fid::BtlChar_SetFxBit, &[((p_chr_1) as u64), ((13i32) as i64 as u64)]); };
                    break 's1;
                }
                _ => {
                }
            }
        }
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        { let v_ = F32(0x3f800000); let a_ = (p_chr_1).wrapping_add(3448); c.m.w32(a_, (v_).0); };
        if ((((((c.m.r32(l_part_4) as i32)) < (((((c.m.r8((l_atk_3).wrapping_add(8)) as i8)) as i32)).wrapping_sub(1i32))) as i32)) != 0) {
            { let a1_ = rt::fmul(F32(c.m.r32((l_atk_3).wrapping_add(16))), F32(0x40000000)); { let _ = D::call(c, fid::BtlAnim_SetStep, &[((p_chr_1) as u64), ((a1_).0 as u64)]); } };
            if (({ let a1_ = (({ let a_ = ((l_atk_3).wrapping_add(4)).wrapping_add((((((c.m.r32(l_part_4) as i32)).wrapping_add(1i32)) as u32)).wrapping_mul(2)); (c.m.r16(a_) as i16) }) as i32); ((D::call(c, fid::BtlAnim_AdvanceThen, &[((p_chr_1) as u64), ((a1_) as i64 as u64), ((F32(0x00000000)).0 as u64), ((0i32) as i64 as u64)])) as i32) }) != 0) {
                { let a_ = l_part_4; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
            }
        } else {
            if ((((D::call(c, fid::BtlAnim_Advance, &[((p_chr_1) as u64), ((0i32) as i64 as u64)])) as i32)) != 0) {
                { let _ = D::call(c, fid::BtlAct_RequestAttackEnd, &[((p_chr_1) as u64), ((l_atk_3) as u64)]); };
            }
        }
        if (((rt::flt(F32(c.m.r32((l_atk_3).wrapping_add(28))), F32(0x38d1b717)) as i32)) != 0) {
            l_speed_8 = F32(0x00000000);
            l_accel_9 = F32(0x3f6d0979);
            l_homing_7 = 0i32;
        } else {
            l_speed_8 = F32(c.m.r32((l_atk_3).wrapping_add(28)));
            l_accel_9 = F32(0x461c4000);
            l_homing_7 = 1i32;
        }
        if ((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((91i32) as i64 as u64)])) as i32)) != 0) {
            l_speed_8 = F32(0x00000000);
            { let v_ = l_speed_8; let a_ = (((D::call(c, fid::BtlChar_GetPos, &[((p_chr_1) as u64)])) as u32)).wrapping_add(152); c.m.w32(a_, (v_).0); };
        }
        if ((((((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((92i32) as i64 as u64)])) as i32)) != 0) || ((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((93i32) as i64 as u64)])) as i32)) != 0)) as i32)) != 0) {
            l_speed_8 = F32(0x00000000);
            l_accel_9 = F32(0x3f6d0979);
            l_homing_7 = 0i32;
            if ((((D::call(c, fid::BtlMove_IsBlockedByOpponent, &[((p_chr_1) as u64)])) as i32)) != 0) {
                { let v_ = l_speed_8; let a_ = (((D::call(c, fid::BtlChar_GetPos, &[((p_chr_1) as u64)])) as u32)).wrapping_add(152); c.m.w32(a_, (v_).0); };
            }
        }
        if ((l_homing_7) != 0) {
            { let _ = D::call(c, fid::BtlMove_Step, &[((p_chr_1) as u64), ((2i32) as i64 as u64), ((1i32) as i64 as u64), ((3i32) as i64 as u64), ((l_speed_8).0 as u64), ((l_accel_9).0 as u64)]); };
        } else {
            { let _ = D::call(c, fid::BtlMove_Step, &[((p_chr_1) as u64), ((6i32) as i64 as u64), ((6i32) as i64 as u64), ((3i32) as i64 as u64), ((l_speed_8).0 as u64), ((l_accel_9).0 as u64)]); };
        }
        { let _ = D::call(c, fid::BtlMove_ApplyGravity, &[((p_chr_1) as u64)]); };
        { let _ = D::call(c, fid::BtlAnim_SetObjRate, &[((p_chr_1) as u64), ((F32(0x40000000)).0 as u64)]); };
        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((213i32) as i64 as u64)]); };
        { let _ = D::call(c, fid::BtlChar_SetFxBit, &[((p_chr_1) as u64), ((58i32) as i64 as u64)]); };
        if (((((c.m.r32(l_atk_3) as i32)) & (64i32))) != 0) {
            if ((F::BtlChar_IsFlagRaised(c, p_chr_1, ((((91i32) as i64 as u64)) as u32))) != 0) {
                if ((((((((c.m.r8((l_atk_3).wrapping_add(10)) as i8)) as i32)) >= (0i32)) as i32)) != 0) {
                    { let _ = D::call(c, fid::ChrCam_EndCut, &[((p_chr_1) as u64)]); };
                }
            }
        }
    }
    if (((((p_phase_2) == (2i32)) as i32)) != 0) {
        { let _ = D::call(c, fid::BtlAct_DecideAttackFollow, &[((p_chr_1) as u64), ((l_atk_3) as u64)]); };
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_AttackHandler", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_Action67to69(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_Action67to69", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let r_ = 'f: {
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        { let _ = D::call(c, fid::BtlAnim_Play, &[((p_chr_1) as u64), ((247i32) as i64 as u64), ((F32(0x3e199999)).0 as u64)]); };
        { let _ = D::call(c, fid::BtlChar_PlayVoice, &[((p_chr_1) as u64), ((5i32) as i64 as u64)]); };
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        { let _ = { let a1_ = F::BtlParam_GetKiDrain67(c, p_chr_1); ((D::call(c, fid::BtlMember_SpendKi, &[((p_chr_1) as u64), ((a1_) as i64 as u64), ((0i32) as i64 as u64)])) as i32) }; };
        if ((((D::call(c, fid::BtlAnim_Advance, &[((p_chr_1) as u64), ((0i32) as i64 as u64)])) as i32)) != 0) {
            if ((F::BtlMember_IsKiEmpty(c, p_chr_1)) != 0) {
                { let _ = D::call(c, fid::BtlAct_Request, &[((p_chr_1) as u64), ((56i32) as i64 as u64)]); };
            }
            if ((((D::call(c, fid::BtlInput_TestAction, &[((p_chr_1) as u64), ((34i32) as i64 as u64), ((0i32) as i64 as u64)])) as i32)) != 0) {
                { let _ = D::call(c, fid::BtlAct_Request, &[((p_chr_1) as u64), ((11i32) as i64 as u64)]); };
            }
        }
        { let _ = D::call(c, fid::BtlMove_Step, &[((p_chr_1) as u64), ((6i32) as i64 as u64), ((6i32) as i64 as u64), ((7i32) as i64 as u64), ((F32(0x00000000)).0 as u64), ((F32(0x3f6d0979)).0 as u64)]); };
        { let _ = D::call(c, fid::BtlMove_ApplyGravity, &[((p_chr_1) as u64)]); };
        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((213i32) as i64 as u64)]); };
        { let _ = D::call(c, fid::BtlChar_SetVibration, &[((p_chr_1) as u64), ((F32(0x3f333333)).0 as u64), ((F32(0x3dcccccc)).0 as u64)]); };
        if ((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((17i32) as i64 as u64)])) as i32)) != 0) {
            { let _ = D::call(c, fid::BtlChar_SetHeldFlag, &[((p_chr_1) as u64), ((14i32) as i64 as u64)]); };
        }
        if (((((((((((D::call(c, fid::BtlAct_GetCurrent, &[((p_chr_1) as u64)])) as i32)) == (103i32)) as i32)) != 0) && ((((((c.m.r32((p_chr_1).wrapping_add(2404)) as i32)) < (10i32)) as i32)) != 0)) as i32)) != 0) {
            { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((78i32) as i64 as u64)]); };
        }
    }
    if (((((p_phase_2) == (2i32)) as i32)) != 0) {
        if ((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((112i32) as i64 as u64)])) as i32)) != 0) {
            's1: {
                let sv3: i32 = ((D::call(c, fid::BtlAct_GetCurrent, &[((p_chr_1) as u64)])) as i32);
                match sv3 {
                    103i32 | 104i32 => {
                        { let a1_ = { let a1_ = F::BtlParam_GetUnk88(c, p_chr_1, ((((0i32) as i64 as u64)) as u32)); F::BtlAct_GetChainAction(c, p_chr_1, ((((a1_) as i64 as u64)) as u32)) }; { let _ = D::call(c, fid::BtlAct_Request, &[((p_chr_1) as u64), ((a1_) as i64 as u64)]); } };
                        break 's1;
                    }
                    105i32 => {
                        { let a1_ = { let a1_ = F::BtlParam_GetUnk88(c, p_chr_1, ((((1i32) as i64 as u64)) as u32)); F::BtlAct_GetChainAction(c, p_chr_1, ((((a1_) as i64 as u64)) as u32)) }; { let _ = D::call(c, fid::BtlAct_Request, &[((p_chr_1) as u64), ((a1_) as i64 as u64)]); } };
                        break 's1;
                    }
                    _ => {
                    }
                }
            }
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_Action67to69", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_Action6A(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_Action6A", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let r_ = 'f: {
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        { let _ = D::call(c, fid::BtlAnim_Play, &[((p_chr_1) as u64), ((248i32) as i64 as u64), ((F32(0x3e199999)).0 as u64)]); };
        { let v_ = F32(0x411425ec); let a_ = (((D::call(c, fid::BtlChar_GetPos, &[((p_chr_1) as u64)])) as u32)).wrapping_add(152); c.m.w32(a_, (v_).0); };
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        if ((((D::call(c, fid::BtlAnim_Advance, &[((p_chr_1) as u64), ((0i32) as i64 as u64)])) as i32)) != 0) {
            { let _ = D::call(c, fid::BtlAct_Request, &[((p_chr_1) as u64), ((11i32) as i64 as u64)]); };
        }
        { let v_ = F32(0x3f800000); let a_ = (p_chr_1).wrapping_add(3448); c.m.w32(a_, (v_).0); };
        { let _ = D::call(c, fid::BtlMove_Step, &[((p_chr_1) as u64), ((2i32) as i64 as u64), ((5i32) as i64 as u64), ((2i32) as i64 as u64), ((F32(0x00000000)).0 as u64), ((F32(0x3f6d0979)).0 as u64)]); };
        { let _ = D::call(c, fid::BtlMove_ApplyGravity, &[((p_chr_1) as u64)]); };
        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((213i32) as i64 as u64)]); };
        if ((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((17i32) as i64 as u64)])) as i32)) != 0) {
            { let _ = D::call(c, fid::BtlChar_SetHeldFlag, &[((p_chr_1) as u64), ((14i32) as i64 as u64)]); };
        }
    }
    if (((((p_phase_2) == (2i32)) as i32)) != 0) {
        if ((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((91i32) as i64 as u64)])) as i32)) != 0) {
            { let _ = ((D::call(c, fid::BtlDecide_QueueAttack, &[((p_chr_1) as u64), ((162i32) as i64 as u64)])) as i32); };
            if (((rt::fgt(F32((D::call(c, fid::BtlAnim_GetProgress, &[((p_chr_1) as u64)])) as u32), F32(0x3f199999)) as i32)) != 0) {
                { let a1_ = ((D::call(c, fid::BtlAct_GetQueued, &[((p_chr_1) as u64)])) as i32); { let _ = D::call(c, fid::BtlAct_Request, &[((p_chr_1) as u64), ((a1_) as i64 as u64)]); } };
            }
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_Action6A", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_AttackDashHandler(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_AttackDashHandler", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let mut l_atk_3: u32 = 0;
    let mut l_frames_4: u32 = 0;
    let mut l_lean_5: F32 = F32(0);
    let mut l_accel_6: F32 = F32(0);
    let mut l_speed_7: F32 = F32(0);
    let mut l_pitch_8: F32 = F32(0);
    let mut l_end_9: i32 = 0;
    let r_ = 'f: {
    l_atk_3 = (p_chr_1).wrapping_add(3320);
    l_frames_4 = ((p_chr_1).wrapping_add(976)).wrapping_add((((2i32) as u32)).wrapping_mul(4));
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        F::BtlAct_LatchAttack(c, p_chr_1);
        { let _ = D::call(c, fid::BtlAnim_Play, &[((p_chr_1) as u64), ((407i32) as i64 as u64), ((F32(0x00000000)).0 as u64)]); };
        { let _ = D::call(c, fid::BtlAct_PlayPickedVoice, &[((p_chr_1) as u64)]); };
        { let _ = D::call(c, fid::BtlChar_SetHeldFlag, &[((p_chr_1) as u64), ((14i32) as i64 as u64)]); };
        { let a1_ = ((D::call(c, fid::BtlParam_GetDashSound, &[((p_chr_1) as u64)])) as i32); { let _ = D::call(c, fid::BtlCharSnd_PlayCommon, &[((p_chr_1) as u64), ((a1_) as i64 as u64)]); } };
        { let _ = D::call(c, fid::BtlChar_SetHeldFlag, &[((p_chr_1) as u64), ((79i32) as i64 as u64)]); };
        if (((((c.m.r32(l_atk_3) as i32)) & (128i32))) != 0) {
            { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((205i32) as i64 as u64)]); };
        }
        if ((((((((c.m.r8((l_atk_3).wrapping_add(10)) as i8)) as i32)) >= (0i32)) as i32)) != 0) {
            { let a2_ = (((c.m.r8((l_atk_3).wrapping_add(10)) as i8)) as i32); { let _ = D::call(c, fid::ChrCam_RequestCut, &[((p_chr_1) as u64), ((1i32) as i64 as u64), ((a2_) as i64 as u64)]); } };
        }
        's1: {
            let sv10: i32 = (((c.m.r8((l_atk_3).wrapping_add(9)) as i8)) as i32);
            match sv10 {
                1i32 | 2i32 | 3i32 => {
                    { let _ = D::call(c, fid::BtlChar_SetFxBit, &[((p_chr_1) as u64), ((13i32) as i64 as u64)]); };
                    break 's1;
                }
                _ => {
                }
            }
        }
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        l_lean_5 = F32(0x00000000);
        l_accel_6 = F32(0x3f6d0979);
        l_speed_7 = F32(0x00000000);
        's2: {
            let sv11: i32 = ((D::call(c, fid::BtlAnim_GetId, &[((p_chr_1) as u64)])) as i32);
            match sv11 {
                407i32 => {
                    { let _ = D::call(c, fid::BtlAnim_AdvanceLoop, &[((p_chr_1) as u64), ((0i32) as i64 as u64)]); };
                    l_speed_7 = F32(c.m.r32((l_atk_3).wrapping_add(28)));
                    l_lean_5 = F32(0x3f800000);
                    l_accel_6 = F32(0x461c4000);
                    { let _ = D::call(c, fid::BtlChar_SetFxBit, &[((p_chr_1) as u64), ((9i32) as i64 as u64)]); };
                    break 's2;
                }
                408i32 => {
                    { let _ = D::call(c, fid::BtlAnim_SetDuration, &[((p_chr_1) as u64), ((F32(0x3e999999)).0 as u64)]); };
                    if ((((D::call(c, fid::BtlAnim_Advance, &[((p_chr_1) as u64), ((0i32) as i64 as u64)])) as i32)) != 0) {
                        { let _ = D::call(c, fid::BtlAct_RequestAttackEnd, &[((p_chr_1) as u64), ((l_atk_3) as u64)]); };
                    }
                    { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((63i32) as i64 as u64)]); };
                    l_lean_5 = rt::fsub(F32(0x3f800000), F32((D::call(c, fid::BtlAnim_GetProgress, &[((p_chr_1) as u64)])) as u32));
                    break 's2;
                }
                _ => {
                }
            }
        }
        { let _ = D::call(c, fid::BtlMove_Step, &[((p_chr_1) as u64), ((6i32) as i64 as u64), ((6i32) as i64 as u64), ((3i32) as i64 as u64), ((l_speed_7).0 as u64), ((l_accel_6).0 as u64)]); };
        { let _ = D::call(c, fid::BtlMove_ApplyGravity, &[((p_chr_1) as u64)]); };
        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((26i32) as i64 as u64)]); };
        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((213i32) as i64 as u64)]); };
        l_pitch_8 = { let a_ = (((D::call(c, fid::BtlChar_GetPos, &[((p_chr_1) as u64)])) as u32)).wrapping_add(144); F32(c.m.r32(a_)) };
        { let a1_ = rt::fmul({ let a0_ = F::BtlOpp_GetPitchAdjusted(c, p_chr_1); F::BtlUtil_ClampF(c, a0_, rt::fsub(l_pitch_8, F32(0x3f490fda)), rt::fadd(l_pitch_8, F32(0x3f490fda))) }, l_lean_5); { let _ = D::call(c, fid::BtlMove_SetLeanX, &[((p_chr_1) as u64), ((a1_).0 as u64)]); } };
        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((40i32) as i64 as u64)]); };
        { let _ = D::call(c, fid::BtlChar_SetFxBit, &[((p_chr_1) as u64), ((58i32) as i64 as u64)]); };
        if (((((c.m.r32(l_atk_3) as i32)) & (64i32))) != 0) {
            if ((F::BtlChar_IsFlagRaised(c, p_chr_1, ((((91i32) as i64 as u64)) as u32))) != 0) {
                if ((((((((c.m.r8((l_atk_3).wrapping_add(10)) as i8)) as i32)) >= (0i32)) as i32)) != 0) {
                    { let _ = D::call(c, fid::ChrCam_EndCut, &[((p_chr_1) as u64)]); };
                }
            }
        }
    }
    if (((((p_phase_2) == (2i32)) as i32)) != 0) {
        's3: {
            let sv12: i32 = ((D::call(c, fid::BtlAnim_GetId, &[((p_chr_1) as u64)])) as i32);
            match sv12 {
                407i32 => {
                    l_end_9 = 0i32;
                    if ((((({ let a_ = l_frames_4; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); n_ }) > (20i32)) as i32)) != 0) {
                        l_end_9 = 1i32;
                        { let r_ = F32(0x3f000000); let a_ = (((D::call(c, fid::BtlChar_GetPos, &[((p_chr_1) as u64)])) as u32)).wrapping_add(152); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    }
                    if ((((((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((91i32) as i64 as u64)])) as i32)) != 0) || ((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((93i32) as i64 as u64)])) as i32)) != 0)) as i32)) != 0) {
                        l_end_9 = 1i32;
                        { let v_ = F32(0x00000000); let a_ = (((D::call(c, fid::BtlChar_GetPos, &[((p_chr_1) as u64)])) as u32)).wrapping_add(152); c.m.w32(a_, (v_).0); };
                    }
                    if ((l_end_9) != 0) {
                        { let _ = D::call(c, fid::BtlAnim_Request, &[((p_chr_1) as u64), ((408i32) as i64 as u64), ((F32(0x00000000)).0 as u64)]); };
                        { let _ = D::call(c, fid::BtlChar_ClearFlag, &[((p_chr_1) as u64), ((79i32) as i64 as u64)]); };
                    }
                    break 's3;
                }
                408i32 => {
                    { let _ = D::call(c, fid::BtlAct_DecideAttackFollow, &[((p_chr_1) as u64), ((l_atk_3) as u64)]); };
                    break 's3;
                }
                _ => {
                }
            }
        }
    }
    if (((((p_phase_2) == (3i32)) as i32)) != 0) {
        { let _ = D::call(c, fid::BtlChar_ClearFlag, &[((p_chr_1) as u64), ((79i32) as i64 as u64)]); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_AttackDashHandler", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_AttackVolleyDashHandler(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_AttackVolleyDashHandler", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let mut l_atk_3: u32 = 0;
    let mut l_pause_4: u32 = 0;
    let mut l_bursts_5: u32 = 0;
    let mut l_frames_6: u32 = 0;
    let mut l_speed_7: F32 = F32(0);
    let mut l_accel_8: F32 = F32(0);
    let mut l_lean_9: F32 = F32(0);
    let mut l_end_10: i32 = 0;
    let mut l_more_11: i32 = 0;
    let r_ = 'f: {
    l_atk_3 = (p_chr_1).wrapping_add(3320);
    l_pause_4 = ((p_chr_1).wrapping_add(976)).wrapping_add((((3i32) as u32)).wrapping_mul(4));
    l_bursts_5 = ((p_chr_1).wrapping_add(976)).wrapping_add((((4i32) as u32)).wrapping_mul(4));
    l_frames_6 = ((p_chr_1).wrapping_add(976)).wrapping_add((((2i32) as u32)).wrapping_mul(4));
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        F::BtlAct_LatchAttack(c, p_chr_1);
        { let _ = D::call(c, fid::BtlAnim_Play, &[((p_chr_1) as u64), ((45i32) as i64 as u64), ((F32(0x3e199999)).0 as u64)]); };
        { let _ = D::call(c, fid::BtlAct_PlayPickedVoice, &[((p_chr_1) as u64)]); };
        { let _ = D::call(c, fid::BtlChar_SetVibration, &[((p_chr_1) as u64), ((F32(0x3f800000)).0 as u64), ((F32(0x3e999999)).0 as u64)]); };
        { let _ = D::call(c, fid::BtlChar_SetHeldFlag, &[((p_chr_1) as u64), ((14i32) as i64 as u64)]); };
        if (((((c.m.r32(l_atk_3) as i32)) & (128i32))) != 0) {
            { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((205i32) as i64 as u64)]); };
        }
        if ((((((((c.m.r8((l_atk_3).wrapping_add(10)) as i8)) as i32)) >= (0i32)) as i32)) != 0) {
            { let a2_ = (((c.m.r8((l_atk_3).wrapping_add(10)) as i8)) as i32); { let _ = D::call(c, fid::ChrCam_RequestCut, &[((p_chr_1) as u64), ((1i32) as i64 as u64), ((a2_) as i64 as u64)]); } };
        }
        's1: {
            let sv12: i32 = (((c.m.r8((l_atk_3).wrapping_add(9)) as i8)) as i32);
            match sv12 {
                1i32 | 2i32 | 3i32 => {
                    { let _ = D::call(c, fid::BtlChar_SetFxBit, &[((p_chr_1) as u64), ((13i32) as i64 as u64)]); };
                    break 's1;
                }
                _ => {
                }
            }
        }
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        l_speed_7 = F32(0x00000000);
        l_accel_8 = F32(0x3f6d0979);
        l_lean_9 = F32(0x00000000);
        's2: {
            let sv13: i32 = ((D::call(c, fid::BtlAnim_GetId, &[((p_chr_1) as u64)])) as i32);
            match sv13 {
                45i32 => {
                    { let _ = ((D::call(c, fid::BtlAnim_AdvanceThen, &[((p_chr_1) as u64), ((25i32) as i64 as u64), ((F32(0x3dcccccc)).0 as u64), ((0i32) as i64 as u64)])) as i32); };
                    l_speed_7 = F32(0x00000000);
                    break 's2;
                }
                25i32 => {
                    { let _ = D::call(c, fid::BtlAnim_AdvanceLoop, &[((p_chr_1) as u64), ((0i32) as i64 as u64)]); };
                    l_speed_7 = F32(c.m.r32((l_atk_3).wrapping_add(28)));
                    l_accel_8 = F32(0x461c4000);
                    l_lean_9 = F32(0x3f800000);
                    if ((((D::call(c, fid::BtlAnim_IsNew, &[((p_chr_1) as u64)])) as i32)) != 0) {
                        { let a1_ = ((D::call(c, fid::BtlParam_GetDashSound, &[((p_chr_1) as u64)])) as i32); { let _ = D::call(c, fid::BtlCharSnd_PlayCommon, &[((p_chr_1) as u64), ((a1_) as i64 as u64)]); } };
                        { let _ = D::call(c, fid::BtlChar_SetHeldFlag, &[((p_chr_1) as u64), ((79i32) as i64 as u64)]); };
                    }
                    { let _ = D::call(c, fid::BtlChar_SetFxBit, &[((p_chr_1) as u64), ((9i32) as i64 as u64)]); };
                    { let _ = D::call(c, fid::BtlChar_SetFxBit, &[((p_chr_1) as u64), ((4i32) as i64 as u64)]); };
                    break 's2;
                }
                26i32 => {
                    { let _ = D::call(c, fid::BtlAnim_SetDuration, &[((p_chr_1) as u64), ((F32(0x3e999999)).0 as u64)]); };
                    if ((((D::call(c, fid::BtlAnim_Advance, &[((p_chr_1) as u64), ((0i32) as i64 as u64)])) as i32)) != 0) {
                        { let _ = D::call(c, fid::BtlAct_RequestAttackEnd, &[((p_chr_1) as u64), ((l_atk_3) as u64)]); };
                    }
                    l_speed_7 = F32(0x00000000);
                    l_lean_9 = rt::fsub(F32(0x3f800000), F32((D::call(c, fid::BtlAnim_GetProgress, &[((p_chr_1) as u64)])) as u32));
                    { let _ = D::call(c, fid::BtlChar_ClearFlag, &[((p_chr_1) as u64), ((79i32) as i64 as u64)]); };
                    if ((((D::call(c, fid::BtlMove_IsBlockedByOpponent, &[((p_chr_1) as u64)])) as i32)) != 0) {
                        { let v_ = l_speed_7; let a_ = (((D::call(c, fid::BtlChar_GetPos, &[((p_chr_1) as u64)])) as u32)).wrapping_add(152); c.m.w32(a_, (v_).0); };
                    }
                    break 's2;
                }
                _ => {
                }
            }
        }
        { let _ = D::call(c, fid::BtlMove_Step, &[((p_chr_1) as u64), ((2i32) as i64 as u64), ((1i32) as i64 as u64), ((3i32) as i64 as u64), ((l_speed_7).0 as u64), ((l_accel_8).0 as u64)]); };
        { let _ = D::call(c, fid::BtlMove_ApplyGravity, &[((p_chr_1) as u64)]); };
        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((26i32) as i64 as u64)]); };
        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((213i32) as i64 as u64)]); };
        { let a1_ = rt::fmul({ let a_ = (((D::call(c, fid::BtlChar_GetPos, &[((p_chr_1) as u64)])) as u32)).wrapping_add(144); F32(c.m.r32(a_)) }, l_lean_9); { let _ = D::call(c, fid::BtlMove_SetLeanX, &[((p_chr_1) as u64), ((a1_).0 as u64)]); } };
        if (((((c.m.r32(l_atk_3) as i32)) & (64i32))) != 0) {
            if ((F::BtlChar_IsFlagRaised(c, p_chr_1, ((((91i32) as i64 as u64)) as u32))) != 0) {
                if ((((((((c.m.r8((l_atk_3).wrapping_add(10)) as i8)) as i32)) >= (0i32)) as i32)) != 0) {
                    { let _ = D::call(c, fid::ChrCam_EndCut, &[((p_chr_1) as u64)]); };
                }
            }
        }
    }
    if (((((p_phase_2) == (2i32)) as i32)) != 0) {
        's3: {
            let sv14: i32 = ((D::call(c, fid::BtlAnim_GetId, &[((p_chr_1) as u64)])) as i32);
            match sv14 {
                25i32 => {
                    l_end_10 = 0i32;
                    if ((((!((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((79i32) as i64 as u64)])) as i32)) != 0)) as i32)) != 0) {
                        if ((((((c.m.r32(l_pause_4) as i32)) > (0i32)) as i32)) != 0) {
                            if ((((({ let a_ = l_pause_4; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) <= (0i32)) as i32)) != 0) {
                                { let _ = D::call(c, fid::BtlChar_SetHeldFlag, &[((p_chr_1) as u64), ((79i32) as i64 as u64)]); };
                            }
                        } else {
                            { let v_ = 2i32; let a_ = l_pause_4; c.m.w32(a_, (v_) as u32); };
                            l_more_11 = ((({ let a_ = l_bursts_5; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); n_ }) < (4i32)) as i32);
                            l_end_10 = ((!((l_more_11) != 0)) as i32);
                        }
                    }
                    if ((((({ let a_ = l_frames_6; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); n_ }) > (23i32)) as i32)) != 0) {
                        l_end_10 = 1i32;
                    }
                    if ((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((92i32) as i64 as u64)])) as i32)) != 0) {
                        l_end_10 = 1i32;
                    }
                    if ((l_end_10) != 0) {
                        { let _ = D::call(c, fid::BtlAnim_Request, &[((p_chr_1) as u64), ((26i32) as i64 as u64), ((F32(0x3e199999)).0 as u64)]); };
                    }
                    break 's3;
                }
                26i32 => {
                    { let _ = D::call(c, fid::BtlAct_DecideAttackFollow, &[((p_chr_1) as u64), ((l_atk_3) as u64)]); };
                    break 's3;
                }
                45i32 => {
                    break 's3;
                }
                _ => {
                }
            }
        }
    }
    if (((((p_phase_2) == (3i32)) as i32)) != 0) {
        { let _ = D::call(c, fid::BtlChar_ClearFlag, &[((p_chr_1) as u64), ((79i32) as i64 as u64)]); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_AttackVolleyDashHandler", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_Action6Bto6F(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_Action6Bto6F", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let mut l_anim_3: i32 = 0;
    let mut l_homing_4: i32 = 0;
    let mut l_speed_5: F32 = F32(0);
    let mut l_accel_6: F32 = F32(0);
    let r_ = 'f: {
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        l_anim_3 = 0i32;
        's1: {
            let sv7: i32 = ((D::call(c, fid::BtlAct_GetCurrent, &[((p_chr_1) as u64)])) as i32);
            match sv7 {
                107i32 => {
                    l_anim_3 = 103i32;
                    break 's1;
                }
                108i32 => {
                    l_anim_3 = 106i32;
                    break 's1;
                }
                109i32 => {
                    l_anim_3 = 97i32;
                    break 's1;
                }
                110i32 => {
                    l_anim_3 = 100i32;
                    break 's1;
                }
                111i32 => {
                    l_anim_3 = 94i32;
                    break 's1;
                }
                _ => {
                }
            }
        }
        { let _ = D::call(c, fid::BtlAnim_Play, &[((p_chr_1) as u64), ((l_anim_3) as i64 as u64), ((F32(0x3dcccccc)).0 as u64)]); };
        if ((((((c.m.r32((p_chr_1).wrapping_add(3432)) as i32)) > (0i32)) as i32)) != 0) {
            { let _ = D::call(c, fid::ChrCam_RequestCut, &[((p_chr_1) as u64), ((1i32) as i64 as u64), ((32i32) as i64 as u64)]); };
        }
        { let _ = D::call(c, fid::BtlChar_SetVibration, &[((p_chr_1) as u64), ((F32(0x3f800000)).0 as u64), ((F32(0x3e999999)).0 as u64)]); };
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        if ((((D::call(c, fid::BtlAnim_Advance, &[((p_chr_1) as u64), ((0i32) as i64 as u64)])) as i32)) != 0) {
            { let _ = D::call(c, fid::BtlAct_Request, &[((p_chr_1) as u64), ((11i32) as i64 as u64)]); };
        }
        l_speed_5 = F32(0x415e38e2);
        l_accel_6 = F32(0x461c4000);
        l_homing_4 = 1i32;
        if ((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((91i32) as i64 as u64)])) as i32)) != 0) {
            l_speed_5 = F32(0x00000000);
            { let v_ = l_speed_5; let a_ = (((D::call(c, fid::BtlChar_GetPos, &[((p_chr_1) as u64)])) as u32)).wrapping_add(152); c.m.w32(a_, (v_).0); };
        }
        if ((((((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((92i32) as i64 as u64)])) as i32)) != 0) || ((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((93i32) as i64 as u64)])) as i32)) != 0)) as i32)) != 0) {
            l_speed_5 = F32(0x00000000);
            l_accel_6 = F32(0x3f6d0979);
            l_homing_4 = 0i32;
            if ((((D::call(c, fid::BtlMove_IsBlockedByOpponent, &[((p_chr_1) as u64)])) as i32)) != 0) {
                { let v_ = l_speed_5; let a_ = (((D::call(c, fid::BtlChar_GetPos, &[((p_chr_1) as u64)])) as u32)).wrapping_add(152); c.m.w32(a_, (v_).0); };
            }
        }
        if ((l_homing_4) != 0) {
            { let _ = D::call(c, fid::BtlMove_Step, &[((p_chr_1) as u64), ((2i32) as i64 as u64), ((1i32) as i64 as u64), ((3i32) as i64 as u64), ((l_speed_5).0 as u64), ((l_accel_6).0 as u64)]); };
        } else {
            { let _ = D::call(c, fid::BtlMove_Step, &[((p_chr_1) as u64), ((6i32) as i64 as u64), ((6i32) as i64 as u64), ((3i32) as i64 as u64), ((l_speed_5).0 as u64), ((l_accel_6).0 as u64)]); };
        }
        { let _ = D::call(c, fid::BtlMove_ApplyGravity, &[((p_chr_1) as u64)]); };
        { let _ = D::call(c, fid::BtlAnim_SetObjRate, &[((p_chr_1) as u64), ((F32(0x40000000)).0 as u64)]); };
        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((137i32) as i64 as u64)]); };
        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((9i32) as i64 as u64)]); };
        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((213i32) as i64 as u64)]); };
        { let _ = D::call(c, fid::BtlChar_SetFxBit, &[((p_chr_1) as u64), ((58i32) as i64 as u64)]); };
    }
    if (((((p_phase_2) == (2i32)) as i32)) != 0) {
        if ((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((91i32) as i64 as u64)])) as i32)) != 0) {
            { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((61i32) as i64 as u64)]); };
            's2: {
                let sv8: i32 = ((D::call(c, fid::BtlOpp_GetSeenAction, &[((p_chr_1) as u64)])) as i32);
                match sv8 {
                    213i32 => {
                        if ((((!((((D::call(c, fid::BtlAct_HasQueued, &[((p_chr_1) as u64)])) as i32)) != 0)) as i32)) != 0) {
                            { let _ = ((D::call(c, fid::BtlDecide_Attack, &[((p_chr_1) as u64), ((((1572864i32) as u32)) as u64)])) as i32); };
                        }
                        if (((rt::fgt(F32((D::call(c, fid::BtlAnim_GetProgress, &[((p_chr_1) as u64)])) as u32), F32(0x3f333333)) as i32)) != 0) {
                            { let a1_ = ((D::call(c, fid::BtlAct_GetQueued, &[((p_chr_1) as u64)])) as i32); { let _ = D::call(c, fid::BtlAct_Request, &[((p_chr_1) as u64), ((a1_) as i64 as u64)]); } };
                        }
                        break 's2;
                    }
                    198i32 => {
                        if ((((!((((D::call(c, fid::BtlAct_HasQueued, &[((p_chr_1) as u64)])) as i32)) != 0)) as i32)) != 0) {
                            { let _ = ((D::call(c, fid::BtlDecide_Main, &[((p_chr_1) as u64), ((((32768i32) as u32)) as u64)])) as i32); };
                            { let _ = ((D::call(c, fid::BtlDecide_Attack, &[((p_chr_1) as u64), ((((1i32) as u32)) as u64)])) as i32); };
                        }
                        if (((rt::fgt(F32((D::call(c, fid::BtlAnim_GetProgress, &[((p_chr_1) as u64)])) as u32), F32(0x3f199999)) as i32)) != 0) {
                            { let a1_ = ((D::call(c, fid::BtlAct_GetQueued, &[((p_chr_1) as u64)])) as i32); { let _ = D::call(c, fid::BtlAct_Request, &[((p_chr_1) as u64), ((a1_) as i64 as u64)]); } };
                        }
                        break 's2;
                    }
                    _ => {
                    }
                }
            }
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_Action6Bto6F", &PROBE_N, &[], true); }
    r_
}

pub fn BtlActB_NextVariant(c: &mut Ctx, mut p_chr_1: u32, mut p_n_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlActB_NextVariant", &PROBE_N, &[(p_chr_1) as u32, (p_n_2) as u32], false); }
    let r_ = 'f: {
    if (((((p_n_2) < (2i32)) as i32)) != 0) {
        break 'f 0i32;
    }
    { let a_ = (p_chr_1).wrapping_add(4024); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
    { let r_ = p_n_2; let a_ = (p_chr_1).wrapping_add(4024); let n_ = rt::rem_i32((c.m.r32(a_) as i32), r_); c.m.w32(a_, (n_) as u32); };
    break 'f (c.m.r32((p_chr_1).wrapping_add(4024)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlActB_NextVariant", &PROBE_N, &[], true); }
    r_
}

pub fn BtlActB_SetReactionFlags(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlActB_SetReactionFlags", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((213i32) as i64 as u64)]); };
    { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((149i32) as i64 as u64)]); };
    { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((150i32) as i64 as u64)]); };
    if ((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((17i32) as i64 as u64)])) as i32)) != 0) {
        { let _ = D::call(c, fid::BtlChar_SetHeldFlag, &[((p_chr_1) as u64), ((14i32) as i64 as u64)]); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlActB_SetReactionFlags", &PROBE_N, &[], true); }
    r_
}

pub fn BtlActB_PickDownAnim(c: &mut Ctx, mut p_chr_1: u32, mut p_cur_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlActB_PickDownAnim", &PROBE_N, &[(p_chr_1) as u32, (p_cur_2) as u32], false); }
    let mut l_air_3: i32 = 0;
    let mut l_id_4: i32 = 0;
    let r_ = 'f: {
    l_air_3 = F::BtlAct_IsAirMotion(c, p_chr_1, 0i32);
    if ((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((17i32) as i64 as u64)])) as i32)) != 0) {
        l_id_4 = (if ((l_air_3) != 0) { 216i32 } else { 215i32 });
    } else {
        if (((rt::fgt({ let a0_ = (((D::call(c, fid::BtlChar_GetPos, &[((p_chr_1) as u64)])) as u32)).wrapping_add(64); F::Vec3_Length(c, a0_) }, F32(0x4031c71b)) as i32)) != 0) {
            l_id_4 = (if ((l_air_3) != 0) { 211i32 } else { 210i32 });
        } else {
            l_id_4 = (if ((l_air_3) != 0) { 214i32 } else { 213i32 });
        }
    }
    break 'f l_id_4;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlActB_PickDownAnim", &PROBE_N, &[], true); }
    r_
}

pub fn BtlActB_UpdateMashRate(c: &mut Ctx, mut p_chr_1: u32, mut p_rate_2: u32, mut p_scaled_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlActB_UpdateMashRate", &PROBE_N, &[(p_chr_1) as u32, (p_rate_2) as u32, (p_scaled_3) as u32], false); }
    let mut l_add_4: F32 = F32(0);
    let mut l_target_5: F32 = F32(0);
    let r_ = 'f: {
    l_add_4 = F32(0x3f800000);
    if ((p_scaled_3) != 0) {
        if (((rt::fgt(F32((D::call(c, fid::BtlAnim_GetProgress, &[((p_chr_1) as u64)])) as u32), F32(0x3e4ccccc)) as i32)) != 0) {
            l_add_4 = rt::fadd(rt::fmul(rt::fsub(F32(0x3f800000), F32(c.m.r32((p_chr_1).wrapping_add(4040)))), F32(0x3f000000)), F32(0x3f000000));
        }
    }
    l_target_5 = rt::fmul(l_add_4, F32(0x40000000));
    if ((F::BtlChar_IsFree(c, p_chr_1)) != 0) {
        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((231i32) as i64 as u64)]); };
        if ((F::BtlInput_IsPressed(c, p_chr_1, ((1048576i32) as u32))) != 0) {
            { let r_ = l_add_4; let a_ = p_rate_2; let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        }
    }
    { let v_ = { let a0_ = F32(c.m.r32(p_rate_2)); F::BtlUtil_ClampF(c, a0_, F32(0x3f800000), F32(0x40800000)) }; let a_ = p_rate_2; c.m.w32(a_, (v_).0); };
    { let a1_ = F32(c.m.r32(p_rate_2)); { let _ = D::call(c, fid::BtlAnim_SetStep, &[((p_chr_1) as u64), ((a1_).0 as u64)]); } };
    { let v_ = { let a0_ = F32(c.m.r32(p_rate_2)); F::BtlUtil_ApproachF(c, a0_, l_target_5, F32(0x3e088888)) }; let a_ = p_rate_2; c.m.w32(a_, (v_).0); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlActB_UpdateMashRate", &PROBE_N, &[], true); }
    r_
}

pub fn BtlActB_TickMemberChange(c: &mut Ctx, mut p_chr_1: u32, mut p_frames_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlActB_TickMemberChange", &PROBE_N, &[(p_chr_1) as u32, (p_frames_2) as u32], false); }
    let r_ = 'f: {
    if (((((((((F::Battle_GetMode(c)) == (1i32)) as i32)) != 0) && ((((!((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((270i32) as i64 as u64)])) as i32)) != 0)) as i32)) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((!((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((3i32) as i64 as u64)])) as i32)) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((!((F::BtlChar_IsDead(c, p_chr_1)) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((175i32) as i64 as u64)])) as i32)) != 0) {
        break 'f 0i32;
    }
    if (((((F::BtlMember_CountAlive(c, p_chr_1)) > (0i32)) as i32)) != 0) {
        if (((((((((p_frames_2) == (((0i32) as u32))) as i32)) != 0) || ((((({ let a_ = p_frames_2; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); n_ }) > (30i32)) as i32)) != 0)) as i32)) != 0) {
            break 'f 1i32;
        }
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlActB_TickMemberChange", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_HitStaggerHandler(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_HitStaggerHandler", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let mut l_anim_3: i32 = 0;
    let mut l_air_4: i32 = 0;
    let r_ = 'f: {
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        l_anim_3 = 0i32;
        l_air_4 = F::BtlAct_IsAirMotion(c, p_chr_1, 1i32);
        's1: {
            let sv5: i32 = ((D::call(c, fid::BtlAct_GetCurrent, &[((p_chr_1) as u64)])) as i32);
            match sv5 {
                192i32 => {
                    l_anim_3 = (F::BtlActB_NextVariant(c, p_chr_1, 4i32)).wrapping_add((if ((l_air_4) != 0) { 162i32 } else { 158i32 }));
                    break 's1;
                }
                193i32 => {
                    l_anim_3 = (F::BtlActB_NextVariant(c, p_chr_1, 4i32)).wrapping_add((if ((l_air_4) != 0) { 170i32 } else { 166i32 }));
                    break 's1;
                }
                194i32 => {
                    l_anim_3 = (if ((l_air_4) != 0) { 177i32 } else { 179i32 });
                    break 's1;
                }
                195i32 => {
                    l_anim_3 = (if ((l_air_4) != 0) { 177i32 } else { 180i32 });
                    break 's1;
                }
                196i32 => {
                    l_anim_3 = (if ((l_air_4) != 0) { 177i32 } else { 176i32 });
                    break 's1;
                }
                197i32 => {
                    l_anim_3 = (if ((l_air_4) != 0) { 177i32 } else { 178i32 });
                    break 's1;
                }
                198i32 => {
                    l_anim_3 = (if ((l_air_4) != 0) { 177i32 } else { (F::BtlActB_NextVariant(c, p_chr_1, 2i32)).wrapping_add(181i32) });
                    break 's1;
                }
                199i32 => {
                    l_anim_3 = 224i32;
                    break 's1;
                }
                200i32 => {
                    l_anim_3 = 225i32;
                    break 's1;
                }
                201i32 => {
                    l_anim_3 = 184i32;
                    break 's1;
                }
                202i32 => {
                    l_anim_3 = 223i32;
                    break 's1;
                }
                189i32 => {
                    l_anim_3 = 156i32;
                    break 's1;
                }
                _ => {
                }
            }
        }
        { let _ = D::call(c, fid::BtlAnim_Play, &[((p_chr_1) as u64), ((l_anim_3) as i64 as u64), ((F32(0x00000000)).0 as u64)]); };
        { let v_ = F32(0x00000000); let a_ = (((D::call(c, fid::BtlChar_GetPos, &[((p_chr_1) as u64)])) as u32)).wrapping_add(152); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (((D::call(c, fid::BtlChar_GetPos, &[((p_chr_1) as u64)])) as u32)).wrapping_add(156); c.m.w32(a_, (v_).0); };
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        if ((((D::call(c, fid::BtlAnim_Advance, &[((p_chr_1) as u64), ((0i32) as i64 as u64)])) as i32)) != 0) {
            { let a1_ = F::BtlAct_GetIdleFollowUp(c, p_chr_1); { let _ = D::call(c, fid::BtlAct_Request, &[((p_chr_1) as u64), ((a1_) as i64 as u64)]); } };
        }
        { let _ = D::call(c, fid::BtlMove_Step, &[((p_chr_1) as u64), ((6i32) as i64 as u64), ((5i32) as i64 as u64), ((2i32) as i64 as u64), ((F32(0x00000000)).0 as u64), ((F32(0x3f6d0979)).0 as u64)]); };
        { let _ = D::call(c, fid::BtlMove_ApplyGravity, &[((p_chr_1) as u64)]); };
        F::BtlActB_SetReactionFlags(c, p_chr_1);
        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((65i32) as i64 as u64)]); };
        if ((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((15i32) as i64 as u64)])) as i32)) != 0) {
            if (((rt::fgt({ let a0_ = (((D::call(c, fid::BtlChar_GetPos, &[((p_chr_1) as u64)])) as u32)).wrapping_add(64); F::Vec3_Length(c, a0_) }, F32(0x3fed0979)) as i32)) != 0) {
                { let _ = D::call(c, fid::BtlChar_SetFxBit, &[((p_chr_1) as u64), ((50i32) as i64 as u64)]); };
            }
        }
    }
    if (((((p_phase_2) == (2i32)) as i32)) != 0) {
        if ((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((49i32) as i64 as u64)])) as i32)) != 0) {
            { let a1_ = ((D::call(c, fid::BtlAct_GetQueued, &[((p_chr_1) as u64)])) as i32); { let _ = D::call(c, fid::BtlAct_Request, &[((p_chr_1) as u64), ((a1_) as i64 as u64)]); } };
        }
        if ((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((19i32) as i64 as u64)])) as i32)) != 0) {
            if ((((D::call(c, fid::BtlDecide_Main, &[((p_chr_1) as u64), ((((268435456i32) as u32)) as u64)])) as i32)) != 0) {
                { let a1_ = ((D::call(c, fid::BtlAct_GetQueued, &[((p_chr_1) as u64)])) as i32); { let _ = D::call(c, fid::BtlAct_Request, &[((p_chr_1) as u64), ((a1_) as i64 as u64)]); } };
            }
        }
        if ((((D::call(c, fid::BtlDecide_Main, &[((p_chr_1) as u64), ((((8650752i32) as u32)) as u64)])) as i32)) != 0) {
            { let a1_ = ((D::call(c, fid::BtlAct_GetQueued, &[((p_chr_1) as u64)])) as i32); { let _ = D::call(c, fid::BtlAct_Request, &[((p_chr_1) as u64), ((a1_) as i64 as u64)]); } };
        }
    }
    if (((((p_phase_2) == (3i32)) as i32)) != 0) {
        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((51i32) as i64 as u64)]); };
        { let v_ = 15i32; let a_ = (p_chr_1).wrapping_add(4080); c.m.w32(a_, (v_) as u32); };
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_HitStaggerHandler", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_ActionCBtoCC(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_ActionCBtoCC", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let mut l_rate_3: u32 = 0;
    let mut l_anim_4: i32 = 0;
    let mut l_airSaved_5: i32 = 0;
    let mut l_air_6: i32 = 0;
    let mut l_t_7: F32 = F32(0);
    let mut l_lean_8: F32 = F32(0);
    let r_ = 'f: {
    l_rate_3 = ((p_chr_1).wrapping_add(976)).wrapping_add((((6i32) as u32)).wrapping_mul(4));
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        l_anim_4 = 0i32;
        l_airSaved_5 = F::BtlAct_IsAirMotion(c, p_chr_1, 1i32);
        's1: {
            let sv9: i32 = ((D::call(c, fid::BtlAct_GetCurrent, &[((p_chr_1) as u64)])) as i32);
            match sv9 {
                203i32 | 204i32 => {
                    l_anim_4 = (if ((l_airSaved_5) != 0) { 175i32 } else { 174i32 });
                    break 's1;
                }
                _ => {
                }
            }
        }
        { let _ = D::call(c, fid::BtlAnim_Play, &[((p_chr_1) as u64), ((l_anim_4) as i64 as u64), ((F32(0x00000000)).0 as u64)]); };
        { let v_ = F32(0x00000000); let a_ = (((D::call(c, fid::BtlChar_GetPos, &[((p_chr_1) as u64)])) as u32)).wrapping_add(152); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (((D::call(c, fid::BtlChar_GetPos, &[((p_chr_1) as u64)])) as u32)).wrapping_add(156); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x40000000); let a_ = l_rate_3; c.m.w32(a_, (v_).0); };
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        l_air_6 = F::BtlAct_IsAirMotion(c, p_chr_1, 0i32);
        { let a2_ = (((((D::call(c, fid::BtlAct_GetCurrent, &[((p_chr_1) as u64)])) as i32)) == (203i32)) as i32); F::BtlActB_UpdateMashRate(c, p_chr_1, l_rate_3, a2_) };
        if ((((D::call(c, fid::BtlAnim_Advance, &[((p_chr_1) as u64), ((0i32) as i64 as u64)])) as i32)) != 0) {
            { let a1_ = F::BtlAct_GetIdleFollowUp(c, p_chr_1); { let _ = D::call(c, fid::BtlAct_Request, &[((p_chr_1) as u64), ((a1_) as i64 as u64)]); } };
        }
        { let _ = D::call(c, fid::BtlMove_Step, &[((p_chr_1) as u64), ((6i32) as i64 as u64), ((5i32) as i64 as u64), ((2i32) as i64 as u64), ((F32(0x00000000)).0 as u64), ((F32(0x3f6d0979)).0 as u64)]); };
        { let _ = D::call(c, fid::BtlMove_ApplyGravity, &[((p_chr_1) as u64)]); };
        F::BtlActB_SetReactionFlags(c, p_chr_1);
        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((65i32) as i64 as u64)]); };
        if (((((((D::call(c, fid::BtlAct_GetCurrent, &[((p_chr_1) as u64)])) as i32)) == (204i32)) as i32)) != 0) {
            l_t_7 = rt::fsub(F32(0x3f800000), F32((D::call(c, fid::BtlAnim_GetProgress, &[((p_chr_1) as u64)])) as u32));
            l_lean_8 = rt::fmul(l_t_7, F32(0xbfc90fda));
            if ((l_air_6) != 0) {
                l_lean_8 = rt::fmul(l_t_7, F32(0x3f490fda));
            }
            { let _ = D::call(c, fid::BtlMove_SetLeanX, &[((p_chr_1) as u64), ((l_lean_8).0 as u64)]); };
        }
    }
    if (((((p_phase_2) == (2i32)) as i32)) != 0) {
        if ((((D::call(c, fid::BtlDecide_Main, &[((p_chr_1) as u64), ((((268435456i32) as u32)) as u64)])) as i32)) != 0) {
            { let a1_ = ((D::call(c, fid::BtlAct_GetQueued, &[((p_chr_1) as u64)])) as i32); { let _ = D::call(c, fid::BtlAct_Request, &[((p_chr_1) as u64), ((a1_) as i64 as u64)]); } };
        }
        if ((((D::call(c, fid::BtlDecide_Main, &[((p_chr_1) as u64), ((((8388608i32) as u32)) as u64)])) as i32)) != 0) {
            { let a1_ = ((D::call(c, fid::BtlAct_GetQueued, &[((p_chr_1) as u64)])) as i32); { let _ = D::call(c, fid::BtlAct_Request, &[((p_chr_1) as u64), ((a1_) as i64 as u64)]); } };
        }
    }
    if (((((p_phase_2) == (3i32)) as i32)) != 0) {
        { let v_ = 15i32; let a_ = (p_chr_1).wrapping_add(4080); c.m.w32(a_, (v_) as u32); };
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_ActionCBtoCC", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_GroundBounceHandler(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_GroundBounceHandler", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let mut l_rate_3: u32 = 0;
    let mut l_next_4: i32 = 0;
    let mut l_id_5: i32 = 0;
    let r_ = 'f: {
    l_rate_3 = ((p_chr_1).wrapping_add(976)).wrapping_add((((6i32) as u32)).wrapping_mul(4));
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        if ((F::BtlAct_IsAirMotion(c, p_chr_1, 1i32)) != 0) {
            { let r_ = 1i32; let a_ = ((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        }
        { let _ = D::call(c, fid::BtlAnim_Play, &[((p_chr_1) as u64), ((220i32) as i64 as u64), ((F32(0x00000000)).0 as u64)]); };
        { let v_ = F32(0x40000000); let a_ = l_rate_3; c.m.w32(a_, (v_).0); };
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        F::BtlActB_UpdateMashRate(c, p_chr_1, l_rate_3, 1i32);
        's1: {
            let sv6: i32 = ((D::call(c, fid::BtlAnim_GetId, &[((p_chr_1) as u64)])) as i32);
            match sv6 {
                220i32 => {
                    { let _ = ((D::call(c, fid::BtlAnim_AdvanceThen, &[((p_chr_1) as u64), ((221i32) as i64 as u64), ((F32(0x00000000)).0 as u64), ((0i32) as i64 as u64)])) as i32); };
                    break 's1;
                }
                221i32 => {
                    if ((((D::call(c, fid::BtlAnim_Advance, &[((p_chr_1) as u64), ((0i32) as i64 as u64)])) as i32)) != 0) {
                        { let v_ = 1i32; let a_ = ((p_chr_1).wrapping_add(976)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    }
                    break 's1;
                }
                _ => {
                }
            }
        }
        { let a3_ = (if (((((c.m.r32(((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) & (1i32))) != 0) { 2i32 } else { 4i32 }); { let _ = D::call(c, fid::BtlMove_Step, &[((p_chr_1) as u64), ((6i32) as i64 as u64), ((5i32) as i64 as u64), ((a3_) as i64 as u64), ((F32(0x3eed0979)).0 as u64), ((F32(0x3f6d0979)).0 as u64)]); } };
        { let _ = D::call(c, fid::BtlMove_ApplyGravity, &[((p_chr_1) as u64)]); };
        F::BtlActB_SetReactionFlags(c, p_chr_1);
        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((65i32) as i64 as u64)]); };
        if ((F::BtlChar_IsStage4Or27(c)) != 0) {
            if ((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((23i32) as i64 as u64)])) as i32)) != 0) {
                { let _ = D::call(c, fid::BtlChar_SetHeldFlag, &[((p_chr_1) as u64), ((14i32) as i64 as u64)]); };
            }
        }
    }
    if (((((p_phase_2) == (2i32)) as i32)) != 0) {
        if ((((D::call(c, fid::BtlDecide_Main, &[((p_chr_1) as u64), ((((268435456i32) as u32)) as u64)])) as i32)) != 0) {
            { let a1_ = ((D::call(c, fid::BtlAct_GetQueued, &[((p_chr_1) as u64)])) as i32); { let _ = D::call(c, fid::BtlAct_Request, &[((p_chr_1) as u64), ((a1_) as i64 as u64)]); } };
        }
        if (((c.m.r32(((p_chr_1).wrapping_add(976)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) != 0) {
            l_next_4 = 219i32;
            if ((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((14i32) as i64 as u64)])) as i32)) != 0) {
                l_next_4 = (if ((F::BtlChar_IsFree(c, p_chr_1)) != 0) { 230i32 } else { 217i32 });
            } else {
                if ((F::BtlChar_IsFree(c, p_chr_1)) != 0) {
                    if ((((!((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((30i32) as i64 as u64)])) as i32)) != 0)) as i32)) != 0) {
                        l_id_5 = (1i32).wrapping_neg();
                        if ((F::BtlInput_IsHeld(c, p_chr_1, ((32i32) as u32))) != 0) {
                            l_id_5 = 37i32;
                        }
                        if ((F::BtlInput_IsHeld(c, p_chr_1, ((16i32) as u32))) != 0) {
                            l_id_5 = 37i32;
                        }
                        if ((F::BtlInput_IsHeld(c, p_chr_1, ((64i32) as u32))) != 0) {
                            l_id_5 = 226i32;
                        }
                        if ((F::BtlInput_IsHeld(c, p_chr_1, ((128i32) as u32))) != 0) {
                            l_id_5 = 227i32;
                        }
                        if (((((l_id_5) >= (0i32)) as i32)) != 0) {
                            if (((((c.m.r32(((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) & (1i32))) != 0) {
                                { let v_ = { let a0_ = rt::fadd({ let a_ = (((D::call(c, fid::BtlChar_GetPos, &[((p_chr_1) as u64)])) as u32)).wrapping_add(148); F32(c.m.r32(a_)) }, F32(0x40490fda)); F::BtlUtil_WrapAngle(c, a0_) }; let a_ = (((D::call(c, fid::BtlChar_GetPos, &[((p_chr_1) as u64)])) as u32)).wrapping_add(148); c.m.w32(a_, (v_).0); };
                            }
                            l_next_4 = l_id_5;
                        }
                    }
                }
            }
            { let _ = D::call(c, fid::BtlAct_Request, &[((p_chr_1) as u64), ((l_next_4) as i64 as u64)]); };
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_GroundBounceHandler", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_LaunchedHandler(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_LaunchedHandler", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let r_ = 'f: {
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        if ((F::BtlAct_IsAirMotion(c, p_chr_1, 1i32)) != 0) {
            { let r_ = 1i32; let a_ = ((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        }
        { let _ = D::call(c, fid::BtlAnim_Play, &[((p_chr_1) as u64), ((218i32) as i64 as u64), ((F32(0x00000000)).0 as u64)]); };
        { let v_ = rt::fmul(rt::fadd(rt::fmul(F32(c.m.r32((p_chr_1).wrapping_add(4040))), F32(0x3e999999)), F32(0x3f333333)), F32(0xc131c71b)); let a_ = (((D::call(c, fid::BtlChar_GetPos, &[((p_chr_1) as u64)])) as u32)).wrapping_add(156); c.m.w32(a_, (v_).0); };
        { let _ = D::call(c, fid::BtlChar_ClearFlag, &[((p_chr_1) as u64), ((14i32) as i64 as u64)]); };
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        { let _ = D::call(c, fid::BtlMove_Fall, &[((p_chr_1) as u64)]); };
        's1: {
            let sv3: i32 = ((D::call(c, fid::BtlAnim_GetId, &[((p_chr_1) as u64)])) as i32);
            match sv3 {
                218i32 => {
                    { let _ = D::call(c, fid::BtlAnim_AdvanceLoop, &[((p_chr_1) as u64), ((0i32) as i64 as u64)]); };
                    if (((rt::fgt({ let a_ = (((D::call(c, fid::BtlChar_GetPos, &[((p_chr_1) as u64)])) as u32)).wrapping_add(156); F32(c.m.r32(a_)) }, F32(0x00000000)) as i32)) != 0) {
                        { let _ = D::call(c, fid::BtlAnim_Request, &[((p_chr_1) as u64), ((219i32) as i64 as u64), ((F32(0x3ecccccc)).0 as u64)]); };
                    }
                    break 's1;
                }
                219i32 => {
                    { let _ = D::call(c, fid::BtlAnim_AdvanceLoop, &[((p_chr_1) as u64), ((0i32) as i64 as u64)]); };
                    break 's1;
                }
                _ => {
                }
            }
        }
        { let a3_ = (if (((((c.m.r32(((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) & (1i32))) != 0) { 2i32 } else { 4i32 }); { let _ = D::call(c, fid::BtlMove_Step, &[((p_chr_1) as u64), ((6i32) as i64 as u64), ((5i32) as i64 as u64), ((a3_) as i64 as u64), ((F32(0x411425ec)).0 as u64), ((F32(0x3f6d0979)).0 as u64)]); } };
        F::BtlActB_SetReactionFlags(c, p_chr_1);
        { let _ = D::call(c, fid::BtlChar_SetFxBit, &[((p_chr_1) as u64), ((59i32) as i64 as u64)]); };
    }
    if (((((p_phase_2) == (2i32)) as i32)) != 0) {
        if ((((D::call(c, fid::BtlDecide_Main, &[((p_chr_1) as u64), ((((268435456i32) as u32)) as u64)])) as i32)) != 0) {
            { let a1_ = ((D::call(c, fid::BtlAct_GetQueued, &[((p_chr_1) as u64)])) as i32); { let _ = D::call(c, fid::BtlAct_Request, &[((p_chr_1) as u64), ((a1_) as i64 as u64)]); } };
        }
        if (((((((D::call(c, fid::BtlAnim_GetId, &[((p_chr_1) as u64)])) as i32)) == (219i32)) as i32)) != 0) {
            if (((((((rt::flt(F::BtlAct_GetFramesToGround(c, p_chr_1), F32(0x40000000)) as i32)) != 0) || ((F::BtlMove_IsHeadingIntoFloor(c, p_chr_1, F32(0xc0c90fda))) != 0)) as i32)) != 0) {
                if ((((((((!((F::BtlChar_IsStage4Or27(c)) != 0)) as i32)) != 0) || (((rt::flt(F::BtlAct_GetGroundY(c, p_chr_1), F32(0xc1200000)) as i32)) != 0)) as i32)) != 0) {
                    { let _ = D::call(c, fid::BtlAct_Request, &[((p_chr_1) as u64), ((220i32) as i64 as u64)]); };
                    if ((((((((((F::BtlChar_IsFree(c, p_chr_1)) != 0) && ((((!((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((30i32) as i64 as u64)])) as i32)) != 0)) as i32)) != 0)) as i32)) != 0) && ((F::BtlInput_IsHeld(c, p_chr_1, ((240i32) as u32))) != 0)) as i32)) != 0) {
                        if ((((!((F::BtlParam_CanFly(c, p_chr_1)) != 0)) as i32)) != 0) {
                            { let _ = D::call(c, fid::BtlAct_Request, &[((p_chr_1) as u64), ((37i32) as i64 as u64)]); };
                        } else {
                            { let _ = D::call(c, fid::BtlAct_Request, &[((p_chr_1) as u64), ((39i32) as i64 as u64)]); };
                        }
                    }
                } else {
                    if ((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((7i32) as i64 as u64)])) as i32)) != 0) {
                        { let _ = D::call(c, fid::BtlAct_Request, &[((p_chr_1) as u64), ((220i32) as i64 as u64)]); };
                    }
                }
            }
            if ((F::BtlMove_IsHeadingIntoWall(c, p_chr_1, F32(0xc0c90fda))) != 0) {
                if (((({ let a_ = ({ let a_ = (((D::call(c, fid::BtlChar_GetObj, &[((p_chr_1) as u64)])) as u32)).wrapping_add(5728); c.m.r32(a_) }).wrapping_add(98400); (c.m.r32(a_) as i32) }) & (64i32))) != 0) {
                    if ((F::BtlChar_IsFree(c, p_chr_1)) != 0) {
                        { let _ = D::call(c, fid::BtlAct_Request, &[((p_chr_1) as u64), ((232i32) as i64 as u64)]); };
                    } else {
                        { let _ = D::call(c, fid::BtlAct_Request, &[((p_chr_1) as u64), ((231i32) as i64 as u64)]); };
                    }
                } else {
                    if (((((c.m.r32(((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) & (1i32))) != 0) {
                        { let _ = D::call(c, fid::BtlAct_Request, &[((p_chr_1) as u64), ((220i32) as i64 as u64)]); };
                        if ((((((((((F::BtlChar_IsFree(c, p_chr_1)) != 0) && ((((!((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((30i32) as i64 as u64)])) as i32)) != 0)) as i32)) != 0)) as i32)) != 0) && ((F::BtlInput_IsHeld(c, p_chr_1, ((240i32) as u32))) != 0)) as i32)) != 0) {
                            if ((((!((F::BtlParam_CanFly(c, p_chr_1)) != 0)) as i32)) != 0) {
                                { let _ = D::call(c, fid::BtlAct_Request, &[((p_chr_1) as u64), ((37i32) as i64 as u64)]); };
                            } else {
                                { let _ = D::call(c, fid::BtlAct_Request, &[((p_chr_1) as u64), ((39i32) as i64 as u64)]); };
                            }
                        }
                    } else {
                        { let _ = D::call(c, fid::BtlAct_Request, &[((p_chr_1) as u64), ((231i32) as i64 as u64)]); };
                        if ((((((((((F::BtlChar_IsFree(c, p_chr_1)) != 0) && ((((!((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((30i32) as i64 as u64)])) as i32)) != 0)) as i32)) != 0)) as i32)) != 0) && ((F::BtlInput_IsHeld(c, p_chr_1, ((240i32) as u32))) != 0)) as i32)) != 0) {
                            { let _ = D::call(c, fid::BtlAct_Request, &[((p_chr_1) as u64), ((41i32) as i64 as u64)]); };
                        }
                    }
                }
            }
            if ((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((17i32) as i64 as u64)])) as i32)) != 0) {
                { let a1_ = F::BtlAct_GetDownAction(c, p_chr_1); { let _ = D::call(c, fid::BtlAct_Request, &[((p_chr_1) as u64), ((a1_) as i64 as u64)]); } };
            }
            if ((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((22i32) as i64 as u64)])) as i32)) != 0) {
                if ((F::BtlChar_IsFree(c, p_chr_1)) != 0) {
                    { let _ = D::call(c, fid::BtlAct_Request, &[((p_chr_1) as u64), ((230i32) as i64 as u64)]); };
                }
            }
            if ((F::BtlChar_IsStage4Or27(c)) != 0) {
                if ((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((23i32) as i64 as u64)])) as i32)) != 0) {
                    if ((F::BtlChar_IsFree(c, p_chr_1)) != 0) {
                        { let _ = D::call(c, fid::BtlAct_Request, &[((p_chr_1) as u64), ((230i32) as i64 as u64)]); };
                    }
                }
            }
        }
        if ((((((c.m.r32((p_chr_1).wrapping_add(2404)) as i32)) > (8i32)) as i32)) != 0) {
            if ((F::BtlChar_IsFree(c, p_chr_1)) != 0) {
                if ((((D::call(c, fid::BtlInput_TestAction, &[((p_chr_1) as u64), ((47i32) as i64 as u64), ((1i32) as i64 as u64)])) as i32)) != 0) {
                    { let _ = D::call(c, fid::BtlAct_Request, &[((p_chr_1) as u64), ((230i32) as i64 as u64)]); };
                }
                if ((((D::call(c, fid::BtlInput_TestAction, &[((p_chr_1) as u64), ((48i32) as i64 as u64), ((1i32) as i64 as u64)])) as i32)) != 0) {
                    { let _ = D::call(c, fid::BtlChar_SetHeldFlag, &[((p_chr_1) as u64), ((14i32) as i64 as u64)]); };
                    { let _ = D::call(c, fid::BtlAct_Request, &[((p_chr_1) as u64), ((104i32) as i64 as u64)]); };
                }
            }
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_LaunchedHandler", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_ActionCFtoD1(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_ActionCFtoD1", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let mut l_anim_3: i32 = 0;
    let mut l_air_4: i32 = 0;
    let mut l_pos_5: u32 = 0;
    let mut l_speed_6: F32 = F32(0);
    let mut l_end_7: i32 = 0;
    let r_ = 'f: {
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        l_anim_3 = 197i32;
        l_air_4 = F::BtlAct_IsAirMotion(c, p_chr_1, 1i32);
        l_pos_5 = ((D::call(c, fid::BtlChar_GetPos, &[((p_chr_1) as u64)])) as u32);
        's1: {
            let sv8: i32 = ((D::call(c, fid::BtlAct_GetCurrent, &[((p_chr_1) as u64)])) as i32);
            match sv8 {
                207i32 => {
                    l_anim_3 = (if ((l_air_4) != 0) { 188i32 } else { 187i32 });
                    { let _ = D::call(c, fid::BtlChar_ClearFlag, &[((p_chr_1) as u64), ((14i32) as i64 as u64)]); };
                    break 's1;
                }
                208i32 => {
                    if ((l_air_4) != 0) {
                        { let v_ = { let a0_ = rt::fadd(F32(c.m.r32(((l_pos_5).wrapping_add(16)).wrapping_add(4))), F32(0x40490fda)); F::BtlUtil_WrapAngle(c, a0_) }; let a_ = ((l_pos_5).wrapping_add(16)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                        { let v_ = { let a0_ = rt::fadd(F32(c.m.r32((l_pos_5).wrapping_add(148))), F32(0x40490fda)); F::BtlUtil_WrapAngle(c, a0_) }; let a_ = (l_pos_5).wrapping_add(148); c.m.w32(a_, (v_).0); };
                        { let v_ = rt::fneg(F32(c.m.r32((l_pos_5).wrapping_add(144)))); let a_ = (l_pos_5).wrapping_add(144); c.m.w32(a_, (v_).0); };
                    }
                    { let _ = D::call(c, fid::BtlChar_ClearFlag, &[((p_chr_1) as u64), ((14i32) as i64 as u64)]); };
                    break 's1;
                }
                209i32 => {
                    l_anim_3 = (if ((l_air_4) != 0) { 381i32 } else { 380i32 });
                    break 's1;
                }
                _ => {
                }
            }
        }
        { let _ = D::call(c, fid::BtlAnim_Play, &[((p_chr_1) as u64), ((l_anim_3) as i64 as u64), ((F32(0x00000000)).0 as u64)]); };
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        l_speed_6 = F32(0x00000000);
        { let _ = ((D::call(c, fid::BtlAnim_Advance, &[((p_chr_1) as u64), ((0i32) as i64 as u64)])) as i32); };
        's2: {
            let sv9: i32 = ((D::call(c, fid::BtlAct_GetCurrent, &[((p_chr_1) as u64)])) as i32);
            match sv9 {
                207i32 => {
                    l_speed_6 = F32(0x409425ec);
                    break 's2;
                }
                208i32 => {
                    l_speed_6 = F32(0x409425ec);
                    break 's2;
                }
                209i32 => {
                    l_speed_6 = F32(0x00000000);
                    break 's2;
                }
                _ => {
                }
            }
        }
        { let a3_ = (if ((F::BtlAct_IsAirMotion(c, p_chr_1, 0i32)) != 0) { 3i32 } else { 5i32 }); { let _ = D::call(c, fid::BtlMove_Step, &[((p_chr_1) as u64), ((6i32) as i64 as u64), ((6i32) as i64 as u64), ((a3_) as i64 as u64), ((l_speed_6).0 as u64), ((F32(0x3f6d0979)).0 as u64)]); } };
        if (((rt::fgt(F32((D::call(c, fid::BtlAct_GetHeight, &[((p_chr_1) as u64)])) as u32), F32(0x41200000)) as i32)) != 0) {
            { let _ = D::call(c, fid::BtlMove_BrakeVertical, &[((p_chr_1) as u64)]); };
        } else {
            { let _ = D::call(c, fid::BtlMove_Fall, &[((p_chr_1) as u64)]); };
        }
        F::BtlActB_SetReactionFlags(c, p_chr_1);
        { let _ = D::call(c, fid::BtlChar_SetFxBit, &[((p_chr_1) as u64), ((59i32) as i64 as u64)]); };
    }
    if (((((p_phase_2) == (2i32)) as i32)) != 0) {
        if ((F::BtlChar_IsFree(c, p_chr_1)) != 0) {
            if ((((!((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((30i32) as i64 as u64)])) as i32)) != 0)) as i32)) != 0) {
                { let _ = D::call(c, fid::BtlColl_AddActionBit, &[((p_chr_1) as u64), ((37i32) as i64 as u64)]); };
                { let _ = D::call(c, fid::BtlColl_AddActionBit, &[((p_chr_1) as u64), ((38i32) as i64 as u64)]); };
            }
            if ((((D::call(c, fid::BtlDecide_Main, &[((p_chr_1) as u64), ((((268435456i32) as u32)) as u64)])) as i32)) != 0) {
                { let a1_ = ((D::call(c, fid::BtlAct_GetQueued, &[((p_chr_1) as u64)])) as i32); { let _ = D::call(c, fid::BtlAct_Request, &[((p_chr_1) as u64), ((a1_) as i64 as u64)]); } };
            }
        }
        l_end_7 = (((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((49i32) as i64 as u64)])) as i32)) != (0i32)) as i32);
        if (((rt::fgt(F32((D::call(c, fid::BtlAnim_GetProgress, &[((p_chr_1) as u64)])) as u32), F32(0x3f333333)) as i32)) != 0) {
            if ((((!((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((15i32) as i64 as u64)])) as i32)) != 0)) as i32)) != 0) {
                l_end_7 = 1i32;
            }
        }
        if ((l_end_7) != 0) {
            if ((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((15i32) as i64 as u64)])) as i32)) != 0) {
                { let _ = D::call(c, fid::BtlAct_Request, &[((p_chr_1) as u64), ((219i32) as i64 as u64)]); };
            } else {
                { let _ = D::call(c, fid::BtlAct_Request, &[((p_chr_1) as u64), ((217i32) as i64 as u64)]); };
                if ((F::BtlChar_IsStage4Or27(c)) != 0) {
                    if ((F::BtlChar_IsFree(c, p_chr_1)) != 0) {
                        { let _ = D::call(c, fid::BtlAct_Request, &[((p_chr_1) as u64), ((230i32) as i64 as u64)]); };
                    }
                }
            }
            if ((F::BtlChar_IsFree(c, p_chr_1)) != 0) {
                if ((((!((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((30i32) as i64 as u64)])) as i32)) != 0)) as i32)) != 0) {
                    if ((F::BtlInput_IsHeld(c, p_chr_1, ((16i32) as u32))) != 0) {
                        { let _ = D::call(c, fid::BtlChar_SetHeldFlag, &[((p_chr_1) as u64), ((14i32) as i64 as u64)]); };
                        { let _ = D::call(c, fid::BtlAct_Request, &[((p_chr_1) as u64), ((38i32) as i64 as u64)]); };
                    } else {
                        if ((F::BtlInput_IsHeld(c, p_chr_1, ((32i32) as u32))) != 0) {
                            if ((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((15i32) as i64 as u64)])) as i32)) != 0) {
                                { let _ = D::call(c, fid::BtlAct_Request, &[((p_chr_1) as u64), ((37i32) as i64 as u64)]); };
                            } else {
                                { let _ = D::call(c, fid::BtlAct_Request, &[((p_chr_1) as u64), ((230i32) as i64 as u64)]); };
                            }
                        }
                    }
                }
            }
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_ActionCFtoD1", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_ActionD2(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_ActionD2", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let r_ = 'f: {
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        { let _ = D::call(c, fid::BtlAnim_Play, &[((p_chr_1) as u64), ((217i32) as i64 as u64), ((F32(0x00000000)).0 as u64)]); };
        { let v_ = F32(0x00000000); let a_ = (((D::call(c, fid::BtlChar_GetPos, &[((p_chr_1) as u64)])) as u32)).wrapping_add(152); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (((D::call(c, fid::BtlChar_GetPos, &[((p_chr_1) as u64)])) as u32)).wrapping_add(156); c.m.w32(a_, (v_).0); };
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        if ((((D::call(c, fid::BtlAnim_Advance, &[((p_chr_1) as u64), ((0i32) as i64 as u64)])) as i32)) != 0) {
            { let _ = D::call(c, fid::BtlChar_ClearFlag, &[((p_chr_1) as u64), ((14i32) as i64 as u64)]); };
            { let a1_ = F::BtlAct_GetDownAction(c, p_chr_1); { let _ = D::call(c, fid::BtlAct_Request, &[((p_chr_1) as u64), ((a1_) as i64 as u64)]); } };
        }
        if ((((!((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((15i32) as i64 as u64)])) as i32)) != 0)) as i32)) != 0) {
            if (((rt::fgt(F32((D::call(c, fid::BtlAnim_GetProgress, &[((p_chr_1) as u64)])) as u32), F32(0x3ecccccc)) as i32)) != 0) {
                { let _ = D::call(c, fid::BtlAct_Request, &[((p_chr_1) as u64), ((217i32) as i64 as u64)]); };
            }
        }
        { let _ = D::call(c, fid::BtlMove_Step, &[((p_chr_1) as u64), ((6i32) as i64 as u64), ((6i32) as i64 as u64), ((6i32) as i64 as u64), ((F32(0x00000000)).0 as u64), ((F32(0x3f6d0979)).0 as u64)]); };
        { let _ = D::call(c, fid::BtlMove_ApplyGravity, &[((p_chr_1) as u64)]); };
        F::BtlActB_SetReactionFlags(c, p_chr_1);
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_ActionD2", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_KnockBackHandler(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_KnockBackHandler", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let mut l_timer_3: u32 = 0;
    let mut l_speed_4: F32 = F32(0);
    let mut l_height_5: F32 = F32(0);
    let r_ = 'f: {
    l_timer_3 = ((p_chr_1).wrapping_add(976)).wrapping_add((((2i32) as u32)).wrapping_mul(4));
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        { let a1_ = (if ((F::BtlAct_IsAirMotion(c, p_chr_1, 1i32)) != 0) { 192i32 } else { 189i32 }); { let _ = D::call(c, fid::BtlAnim_Play, &[((p_chr_1) as u64), ((a1_) as i64 as u64), ((F32(0x00000000)).0 as u64)]); } };
        { let v_ = rt::f2i(rt::fmul(F32(c.m.r32((p_chr_1).wrapping_add(4040))), F32(0x41700000))); let a_ = l_timer_3; c.m.w32(a_, (v_) as u32); };
        if ((F::BtlChar_IsStage4Or27(c)) != 0) {
            { let v_ = rt::f2i(rt::fmul(rt::i2f((c.m.r32(l_timer_3) as i32)), rt::fsub(F32(0x3f800000), F::BtlMember_GetHealthRatio(c, p_chr_1)))); let a_ = l_timer_3; c.m.w32(a_, (v_) as u32); };
        }
        { let v_ = F32(0x00000000); let a_ = (((D::call(c, fid::BtlChar_GetPos, &[((p_chr_1) as u64)])) as u32)).wrapping_add(156); c.m.w32(a_, (v_).0); };
        { let _ = D::call(c, fid::BtlChar_ClearFlag, &[((p_chr_1) as u64), ((14i32) as i64 as u64)]); };
        's1: {
            let sv6: i32 = ((D::call(c, fid::BtlAct_GetPrev, &[((p_chr_1) as u64)])) as i32);
            match sv6 {
                184i32 | 186i32 => {
                    { let r_ = 1i32; let a_ = ((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    break 's1;
                }
                _ => {
                }
            }
        }
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        l_speed_4 = F32(0x40cf684a);
        if ((F::BtlChar_IsStage4Or27(c)) != 0) {
            l_speed_4 = rt::fmul(rt::fadd(rt::fmul(rt::fsub(F32(0x3f800000), F::BtlMember_GetHealthRatio(c, p_chr_1)), F32(0x3f000000)), F32(0x3f000000)), l_speed_4);
        }
        's2: {
            let sv7: i32 = ((D::call(c, fid::BtlAnim_GetId, &[((p_chr_1) as u64)])) as i32);
            match sv7 {
                189i32 | 192i32 => {
                    { let _ = { let a1_ = (((D::call(c, fid::BtlAnim_GetId, &[((p_chr_1) as u64)])) as i32)).wrapping_add(1i32); ((D::call(c, fid::BtlAnim_AdvanceThen, &[((p_chr_1) as u64), ((a1_) as i64 as u64), ((F32(0x00000000)).0 as u64), ((0i32) as i64 as u64)])) as i32) }; };
                    break 's2;
                }
                190i32 | 193i32 => {
                    if ((((((c.m.r32(l_timer_3) as i32)) > (0i32)) as i32)) != 0) {
                        { let _ = D::call(c, fid::BtlAnim_AdvanceLoop, &[((p_chr_1) as u64), ((0i32) as i64 as u64)]); };
                        { let a_ = l_timer_3; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                    } else {
                        if (({ let a1_ = (((D::call(c, fid::BtlAnim_GetId, &[((p_chr_1) as u64)])) as i32)).wrapping_add(1i32); ((D::call(c, fid::BtlAnim_AdvanceThen, &[((p_chr_1) as u64), ((a1_) as i64 as u64), ((F32(0x00000000)).0 as u64), ((0i32) as i64 as u64)])) as i32) }) != 0) {
                            if ((F::BtlChar_IsFree(c, p_chr_1)) != 0) {
                                if ((((!((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((29i32) as i64 as u64)])) as i32)) != 0)) as i32)) != 0) {
                                    if ((((((F::BtlInput_IsHeld(c, p_chr_1, ((240i32) as u32))) != 0) || ((F::BtlChar_IsStage4Or27(c)) != 0)) as i32)) != 0) {
                                        { let _ = D::call(c, fid::BtlAct_Request, &[((p_chr_1) as u64), ((230i32) as i64 as u64)]); };
                                    }
                                }
                            }
                        }
                    }
                    if ((((((c.m.r32((p_chr_1).wrapping_add(2404)) as i32)) > (0i32)) as i32)) != 0) {
                        { let a1_ = F::BtlAct_IsAirMotion(c, p_chr_1, 0i32); F::BtlMove_DeflectAtStageLimit(c, p_chr_1, a1_) };
                    }
                    break 's2;
                }
                191i32 | 194i32 => {
                    if ((((D::call(c, fid::BtlAnim_Advance, &[((p_chr_1) as u64), ((0i32) as i64 as u64)])) as i32)) != 0) {
                        { let a1_ = F::BtlAct_GetDownAction(c, p_chr_1); { let _ = D::call(c, fid::BtlAct_Request, &[((p_chr_1) as u64), ((a1_) as i64 as u64)]); } };
                        F::BtlCharSnd_PlayBank8(c, p_chr_1, 2i32);
                    }
                    l_height_5 = F32((D::call(c, fid::BtlAct_GetHeight, &[((p_chr_1) as u64)])) as u32);
                    if (((rt::flt({ let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); F::BtlCharApi_GetHeight(c, a0_) }, l_height_5) as i32)) != 0) {
                        if (((rt::fgt(F32((D::call(c, fid::BtlAnim_GetProgress, &[((p_chr_1) as u64)])) as u32), F32(0x3f199999)) as i32)) != 0) {
                            { let _ = D::call(c, fid::BtlAct_Request, &[((p_chr_1) as u64), ((217i32) as i64 as u64)]); };
                        }
                    }
                    break 's2;
                }
                _ => {
                }
            }
        }
        { let a3_ = (if ((F::BtlAct_IsAirMotion(c, p_chr_1, 0i32)) != 0) { 3i32 } else { 5i32 }); { let _ = D::call(c, fid::BtlMove_Step, &[((p_chr_1) as u64), ((6i32) as i64 as u64), ((6i32) as i64 as u64), ((a3_) as i64 as u64), ((l_speed_4).0 as u64), ((F32(0x461c4000)).0 as u64)]); } };
        { let _ = D::call(c, fid::BtlMove_BrakeVertical, &[((p_chr_1) as u64)]); };
        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((27i32) as i64 as u64)]); };
        F::BtlActB_SetReactionFlags(c, p_chr_1);
        { let _ = D::call(c, fid::BtlChar_SetFxBit, &[((p_chr_1) as u64), ((59i32) as i64 as u64)]); };
        if (((((c.m.r32(((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) & (1i32))) != 0) {
            { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((312i32) as i64 as u64)]); };
        }
    }
    if (((((p_phase_2) == (2i32)) as i32)) != 0) {
        if ((((D::call(c, fid::BtlDecide_Main, &[((p_chr_1) as u64), ((((268435456i32) as u32)) as u64)])) as i32)) != 0) {
            { let a1_ = ((D::call(c, fid::BtlAct_GetQueued, &[((p_chr_1) as u64)])) as i32); { let _ = D::call(c, fid::BtlAct_Request, &[((p_chr_1) as u64), ((a1_) as i64 as u64)]); } };
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_KnockBackHandler", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_BlowAwayHandler(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_BlowAwayHandler", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_timer_3: u32 = 0;
    let mut l_mash_4: u32 = 0;
    let mut l_spin_5: u32 = 0;
    let mut l_sel_6: i32 = 0;
    let mut l_air_7: i32 = 0;
    let mut l_hit_8: i32 = 0;
    let mut l_blend_9: F32 = F32(0);
    let mut l_speed_10: F32 = F32(0);
    let r_ = 'f: {
        l_timer_3 = ((p_chr_1).wrapping_add(976)).wrapping_add((((2i32) as u32)).wrapping_mul(4));
        l_mash_4 = ((p_chr_1).wrapping_add(976)).wrapping_add((((3i32) as u32)).wrapping_mul(4));
        l_spin_5 = ((p_chr_1).wrapping_add(976)).wrapping_add((((6i32) as u32)).wrapping_mul(4));
        if (((((p_phase_2) == (0i32)) as i32)) != 0) {
            l_sel_6 = 0i32;
            l_air_7 = F::BtlAct_IsAirMotion(c, p_chr_1, 1i32);
            l_blend_9 = F32(0x00000000);
            if (((((((D::call(c, fid::BtlAct_GetPrev, &[((p_chr_1) as u64)])) as i32)) == (184i32)) as i32)) != 0) {
                l_blend_9 = F32(0x3e199999);
            }
            's1: {
                let sv11: i32 = ((D::call(c, fid::BtlAct_GetCurrent, &[((p_chr_1) as u64)])) as i32);
                match sv11 {
                    213i32 => {
                        { let v_ = 50i32; let a_ = l_timer_3; c.m.w32(a_, (v_) as u32); };
                        l_sel_6 = (if ((l_air_7) != 0) { 196i32 } else { 195i32 });
                        { let _ = D::call(c, fid::BtlCharSnd_PlayCommon, &[((p_chr_1) as u64), ((72i32) as i64 as u64)]); };
                        { let r_ = 3i32; let a_ = ((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                        break 's1;
                    }
                    214i32 => {
                        { let v_ = 15i32; let a_ = l_timer_3; c.m.w32(a_, (v_) as u32); };
                        l_sel_6 = (if ((l_air_7) != 0) { 196i32 } else { 195i32 });
                        { let r_ = 6i32; let a_ = ((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                        break 's1;
                    }
                    223i32 => {
                        { let v_ = 32i32; let a_ = l_timer_3; c.m.w32(a_, (v_) as u32); };
                        l_sel_6 = (if ((l_air_7) != 0) { 199i32 } else { 198i32 });
                        { let r_ = 8i32; let a_ = ((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                        break 's1;
                    }
                    _ => {
                    }
                }
            }
            if ((F::BtlChar_IsStage4Or27(c)) != 0) {
                { let v_ = rt::f2i(rt::fmul(rt::i2f((c.m.r32(l_timer_3) as i32)), rt::fsub(F32(0x3f800000), F::BtlMember_GetHealthRatio(c, p_chr_1)))); let a_ = l_timer_3; c.m.w32(a_, (v_) as u32); };
            }
            { let _ = D::call(c, fid::BtlAnim_Play, &[((p_chr_1) as u64), ((l_sel_6) as i64 as u64), ((l_blend_9).0 as u64)]); };
            { let _ = D::call(c, fid::BtlChar_SetHeldFlag, &[((p_chr_1) as u64), ((14i32) as i64 as u64)]); };
            { let v_ = F32(0x00000000); let a_ = (((D::call(c, fid::BtlChar_GetPos, &[((p_chr_1) as u64)])) as u32)).wrapping_add(156); c.m.w32(a_, (v_).0); };
            's2: {
                let sv12: i32 = ((D::call(c, fid::BtlAct_GetPrev, &[((p_chr_1) as u64)])) as i32);
                match sv12 {
                    184i32 | 186i32 => {
                        { let r_ = 1i32; let a_ = ((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                        break 's2;
                    }
                    _ => {
                    }
                }
            }
        }
        if (((((p_phase_2) == (1i32)) as i32)) != 0) {
            l_air_7 = F::BtlAct_IsAirMotion(c, p_chr_1, 0i32);
            { let _ = D::call(c, fid::BtlAnim_AdvanceLoop, &[((p_chr_1) as u64), ((0i32) as i64 as u64)]); };
            if ((((({ let a_ = l_timer_3; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) < (0i32)) as i32)) != 0) {
                if ((((((F::BtlChar_IsStage4Or27(c)) != 0) && ((F::BtlChar_IsFree(c, p_chr_1)) != 0)) as i32)) != 0) {
                    { let _ = D::call(c, fid::BtlAct_Request, &[((p_chr_1) as u64), ((230i32) as i64 as u64)]); };
                } else {
                    { let _ = D::call(c, fid::BtlAct_Request, &[((p_chr_1) as u64), ((217i32) as i64 as u64)]); };
                }
            }
            if ((((((c.m.r32((p_chr_1).wrapping_add(2404)) as i32)) > (0i32)) as i32)) != 0) {
                { let a1_ = F::BtlAct_IsAirMotion(c, p_chr_1, 0i32); F::BtlMove_DeflectAtStageLimit(c, p_chr_1, a1_) };
            }
            l_sel_6 = (if ((l_air_7) != 0) { 3i32 } else { 5i32 });
            l_speed_10 = F32(0x00000000);
            if ((F::BtlChar_IsStage4Or27(c)) != 0) {
                's3: {
                    let sv13: i32 = ((D::call(c, fid::BtlAct_GetCurrent, &[((p_chr_1) as u64)])) as i32);
                    match sv13 {
                        213i32 => {
                            l_speed_10 = F32(0x411425ec);
                            break 's3;
                        }
                        214i32 => {
                            l_speed_10 = F32(0x411425ec);
                            break 's3;
                        }
                        223i32 => {
                            l_speed_10 = F32(0x411425ec);
                            break 's3;
                        }
                        _ => {
                        }
                    }
                }
            } else {
                's4: {
                    let sv14: i32 = ((D::call(c, fid::BtlAct_GetCurrent, &[((p_chr_1) as u64)])) as i32);
                    match sv14 {
                        213i32 => {
                            l_speed_10 = F32(0x411425ec);
                            break 's4;
                        }
                        214i32 => {
                            l_speed_10 = F32(0x419425ec);
                            break 's4;
                        }
                        223i32 => {
                            l_speed_10 = F32(0x419425ec);
                            break 's4;
                        }
                        _ => {
                        }
                    }
                }
            }
            { let _ = D::call(c, fid::BtlMove_Step, &[((p_chr_1) as u64), ((6i32) as i64 as u64), ((6i32) as i64 as u64), ((l_sel_6) as i64 as u64), ((l_speed_10).0 as u64), ((F32(0x461c4000)).0 as u64)]); };
            { let _ = D::call(c, fid::BtlMove_ApplyGravity, &[((p_chr_1) as u64)]); };
            { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((27i32) as i64 as u64)]); };
            { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((10i32) as i64 as u64)]); };
            { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((42i32) as i64 as u64)]); };
            F::BtlActB_SetReactionFlags(c, p_chr_1);
            { let _ = D::call(c, fid::BtlChar_SetFxBit, &[((p_chr_1) as u64), ((59i32) as i64 as u64)]); };
            if (((((c.m.r32(((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) & (2i32))) != 0) {
                { let a1_ = { let a_ = (((D::call(c, fid::BtlChar_GetPos, &[((p_chr_1) as u64)])) as u32)).wrapping_add(144); F32(c.m.r32(a_)) }; { let _ = D::call(c, fid::BtlMove_SetLeanX, &[((p_chr_1) as u64), ((a1_).0 as u64)]); } };
            }
            if (((((c.m.r32(((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) & (4i32))) != 0) {
                { let v_ = { let a0_ = rt::fadd(F32(c.m.r32(l_spin_5)), F32(0x3f56774e)); F::BtlUtil_WrapAngle(c, a0_) }; let a_ = l_spin_5; c.m.w32(a_, (v_).0); };
                { let a1_ = F32(c.m.r32(l_spin_5)); F::BtlMove_SetLeanZ(c, p_chr_1, a1_) };
            }
            if (((((c.m.r32(((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) & (1i32))) != 0) {
                { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((312i32) as i64 as u64)]); };
            }
        }
        if (((((p_phase_2) == (2i32)) as i32)) != 0) {
            if ((F::BtlChar_IsFree(c, p_chr_1)) != 0) {
                if ((((D::call(c, fid::BtlDecide_Main, &[((p_chr_1) as u64), ((((268435456i32) as u32)) as u64)])) as i32)) != 0) {
                    { let a1_ = ((D::call(c, fid::BtlAct_GetQueued, &[((p_chr_1) as u64)])) as i32); { let _ = D::call(c, fid::BtlAct_Request, &[((p_chr_1) as u64), ((a1_) as i64 as u64)]); } };
                }
                if ((((((((!((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((30i32) as i64 as u64)])) as i32)) != 0)) as i32)) != 0) && ((((((c.m.r32((p_chr_1).wrapping_add(2404)) as i32)) > (8i32)) as i32)) != 0)) as i32)) != 0) {
                    if (({ let a1_ = F::BtlParam_GetRecoverKiCost(c, p_chr_1); F::BtlMember_HasKi(c, p_chr_1, a1_) }) != 0) {
                        { let _ = D::call(c, fid::BtlColl_AddActionBit, &[((p_chr_1) as u64), ((37i32) as i64 as u64)]); };
                        { let _ = D::call(c, fid::BtlColl_AddActionBit, &[((p_chr_1) as u64), ((42i32) as i64 as u64)]); };
                        if ((((D::call(c, fid::BtlInput_TestAction, &[((p_chr_1) as u64), ((40i32) as i64 as u64), ((1i32) as i64 as u64)])) as i32)) != 0) {
                            { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((33i32) as i64 as u64)]); };
                        }
                        if ((((D::call(c, fid::BtlInput_TestAction, &[((p_chr_1) as u64), ((41i32) as i64 as u64), ((1i32) as i64 as u64)])) as i32)) != 0) {
                            { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((34i32) as i64 as u64)]); };
                        }
                    }
                }
                if ((((!((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((29i32) as i64 as u64)])) as i32)) != 0)) as i32)) != 0) {
                    if ((((((D::call(c, fid::BtlParam_GetFlags2, &[((p_chr_1) as u64)])) as i32)) & (64i32))) != 0) {
                        { let _ = D::call(c, fid::BtlColl_AddActionBit, &[((p_chr_1) as u64), ((215i32) as i64 as u64)]); };
                        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((231i32) as i64 as u64)]); };
                        if ((F::BtlInput_IsPressed(c, p_chr_1, ((1048576i32) as u32))) != 0) {
                            { let a_ = l_mash_4; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                            if ((((((c.m.r32(l_mash_4) as i32)) >= ((rt::f2i(rt::fmul(rt::fsub(F32(0x3f800000), F::BtlMember_GetHealthRatio(c, p_chr_1)), F32(0x41400000)))).wrapping_add(4i32))) as i32)) != 0) {
                                { let _ = D::call(c, fid::BtlAct_Request, &[((p_chr_1) as u64), ((215i32) as i64 as u64)]); };
                            }
                        }
                    }
                }
            }
            if ((F::BtlMove_IsHeadingIntoFloor(c, p_chr_1, F32(0x3f9c61a9))) != 0) {
                if ((F::BtlAct_QueueFlag21Action(c, p_chr_1)) != 0) {
                    { let a1_ = ((D::call(c, fid::BtlAct_GetQueued, &[((p_chr_1) as u64)])) as i32); { let _ = D::call(c, fid::BtlAct_Request, &[((p_chr_1) as u64), ((a1_) as i64 as u64)]); } };
                } else {
                    { let _ = F::BtlMember_Damage(c, p_chr_1, 500i32, 0i32); };
                    { let a0_ = ((D::call(c, fid::BtlChar_GetPos, &[((p_chr_1) as u64)])) as u32); F::BtlCharApi_ShakeCamsNear(c, a0_, F32(0x42c80000), F32(0x447a0000), F32(0x40400000), F32(0x3ecccccc)) };
                    { let a0_ = ((D::call(c, fid::BtlChar_GetPos, &[((p_chr_1) as u64)])) as u32); F::BtlCharApi_RumbleNear(c, a0_, F32(0x42c80000), F32(0x447a0000), F32(0x3f4ccccc), F32(0x3e4ccccc)) };
                    if (((((c.m.r32(((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) & (8i32))) != 0) {
                        { let v_ = F32(0x00000000); let a_ = (((D::call(c, fid::BtlChar_GetPos, &[((p_chr_1) as u64)])) as u32)).wrapping_add(152); c.m.w32(a_, (v_).0); };
                    }
                    { let _ = D::call(c, fid::BtlAct_Request, &[((p_chr_1) as u64), ((221i32) as i64 as u64)]); };
                }
            } else {
                if ((F::BtlMove_IsHeadingIntoWall(c, p_chr_1, F32(0x3f9c61a9))) != 0) {
                    if (((({ let a_ = ({ let a_ = (((D::call(c, fid::BtlChar_GetObj, &[((p_chr_1) as u64)])) as u32)).wrapping_add(5728); c.m.r32(a_) }).wrapping_add(98400); (c.m.r32(a_) as i32) }) & (64i32))) != 0) {
                        if ((F::BtlChar_IsFree(c, p_chr_1)) != 0) {
                            { let _ = D::call(c, fid::BtlAct_Request, &[((p_chr_1) as u64), ((232i32) as i64 as u64)]); };
                        } else {
                            { let _ = D::call(c, fid::BtlAct_Request, &[((p_chr_1) as u64), ((231i32) as i64 as u64)]); };
                        }
                    } else {
                        if ((F::BtlAct_QueueFlag21ActionB(c, p_chr_1)) != 0) {
                            { let a1_ = ((D::call(c, fid::BtlAct_GetQueued, &[((p_chr_1) as u64)])) as i32); { let _ = D::call(c, fid::BtlAct_Request, &[((p_chr_1) as u64), ((a1_) as i64 as u64)]); } };
                        } else {
                            { let _ = F::BtlMember_Damage(c, p_chr_1, 500i32, 0i32); };
                            { let a0_ = ((D::call(c, fid::BtlChar_GetPos, &[((p_chr_1) as u64)])) as u32); F::BtlCharApi_ShakeCamsNear(c, a0_, F32(0x42c80000), F32(0x447a0000), F32(0x40400000), F32(0x3ecccccc)) };
                            { let a0_ = ((D::call(c, fid::BtlChar_GetPos, &[((p_chr_1) as u64)])) as u32); F::BtlCharApi_RumbleNear(c, a0_, F32(0x42c80000), F32(0x447a0000), F32(0x3f4ccccc), F32(0x3e4ccccc)) };
                            { let _ = D::call(c, fid::BtlAct_Request, &[((p_chr_1) as u64), ((231i32) as i64 as u64)]); };
                        }
                    }
                } else {
                    if ((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((20i32) as i64 as u64)])) as i32)) != 0) {
                        { let a1_ = ((D::call(c, fid::BtlChar_GetPos, &[((p_chr_1) as u64)])) as u32); F::Vec3_Normalize(c, fp, a1_) };
                        { let a1_ = (((D::call(c, fid::BtlChar_GetPos, &[((p_chr_1) as u64)])) as u32)).wrapping_add(48); F::Vec3_Normalize(c, fp.wrapping_add(16), a1_) };
                        if (((rt::fgt(F::Vec3_Dot(c, fp, fp.wrapping_add(16)), F32(0x3f333333)) as i32)) != 0) {
                            if ((F::BtlChar_IsFree(c, p_chr_1)) != 0) {
                                { let _ = D::call(c, fid::BtlAct_Request, &[((p_chr_1) as u64), ((232i32) as i64 as u64)]); };
                            } else {
                                { let _ = D::call(c, fid::BtlAct_Request, &[((p_chr_1) as u64), ((231i32) as i64 as u64)]); };
                            }
                        }
                    } else {
                        if ((((D::call(c, fid::BtlMove_IsBlockedByOpponent, &[((p_chr_1) as u64)])) as i32)) != 0) {
                            l_sel_6 = F::BtlAct_IsAirMotion(c, p_chr_1, 0i32);
                            if (((rt::flt({ let a0_ = F32((D::call(c, fid::BtlOpp_GetYawFromFacing, &[((p_chr_1) as u64)])) as u32); rt::ffabs(a0_) }, F32(0x3fc90fda)) as i32)) != 0) {
                                l_hit_8 = l_sel_6;
                            } else {
                                l_hit_8 = ((!((l_sel_6) != 0)) as i32);
                            }
                            if ((l_hit_8) != 0) {
                                if ((F::BtlChar_IsFree(c, p_chr_1)) != 0) {
                                    { let _ = D::call(c, fid::BtlAct_Request, &[((p_chr_1) as u64), ((233i32) as i64 as u64)]); };
                                } else {
                                    { let _ = D::call(c, fid::BtlAct_Request, &[((p_chr_1) as u64), ((231i32) as i64 as u64)]); };
                                }
                            }
                        }
                    }
                }
            }
        }
        if (((((p_phase_2) == (3i32)) as i32)) != 0) {
            if (((((c.m.r32(((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) & (1i32))) != 0) {
                F::BtlCharSnd_StopCommon(c, p_chr_1, 72i32);
            }
        }
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_BlowAwayHandler", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_BlowRecoverHandler(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_BlowRecoverHandler", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let mut l_pos_3: u32 = 0;
    let mut l_air_4: i32 = 0;
    let mut l_t_5: F32 = F32(0);
    let r_ = 'f: {
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        { let a1_ = (if ((F::BtlAct_IsAirMotion(c, p_chr_1, 0i32)) != 0) { 398i32 } else { 397i32 }); { let _ = D::call(c, fid::BtlAnim_Play, &[((p_chr_1) as u64), ((a1_) as i64 as u64), ((F32(0x3e199999)).0 as u64)]); } };
        l_pos_3 = ((D::call(c, fid::BtlChar_GetPos, &[((p_chr_1) as u64)])) as u32);
        { let r_ = F32(0x3f000000); let a_ = (l_pos_3).wrapping_add(152); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        { let _ = D::call(c, fid::BtlCharSnd_PlayCommon, &[((p_chr_1) as u64), ((31i32) as i64 as u64)]); };
        { let _ = D::call(c, fid::BtlChar_SetHeldFlag, &[((p_chr_1) as u64), ((14i32) as i64 as u64)]); };
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        l_air_4 = F::BtlAct_IsAirMotion(c, p_chr_1, 0i32);
        if ((((D::call(c, fid::BtlAnim_Advance, &[((p_chr_1) as u64), ((0i32) as i64 as u64)])) as i32)) != 0) {
            if ((F::BtlInput_IsHeld(c, p_chr_1, ((16i32) as u32))) != 0) {
                { let _ = D::call(c, fid::BtlAct_Request, &[((p_chr_1) as u64), ((25i32) as i64 as u64)]); };
            } else {
                { let _ = D::call(c, fid::BtlAct_Request, &[((p_chr_1) as u64), ((11i32) as i64 as u64)]); };
            }
        }
        { let _ = D::call(c, fid::BtlMove_Step, &[((p_chr_1) as u64), ((6i32) as i64 as u64), ((6i32) as i64 as u64), (((if ((l_air_4) != 0) { 3i32 } else { 5i32 })) as i64 as u64), ((F32(0x00000000)).0 as u64), ((F32(0x3eed0979)).0 as u64)]); };
        { let _ = D::call(c, fid::BtlMove_ApplyGravity, &[((p_chr_1) as u64)]); };
        l_t_5 = rt::fsub(F32(0x3f800000), F32((D::call(c, fid::BtlAnim_GetProgress, &[((p_chr_1) as u64)])) as u32));
        { let a1_ = rt::fmul({ let a_ = (((D::call(c, fid::BtlChar_GetPos, &[((p_chr_1) as u64)])) as u32)).wrapping_add(144); F32(c.m.r32(a_)) }, l_t_5); { let _ = D::call(c, fid::BtlMove_SetLeanX, &[((p_chr_1) as u64), ((a1_).0 as u64)]); } };
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_BlowRecoverHandler", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_LieDownHandler(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_LieDownHandler", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let mut l_timer_3: u32 = 0;
    let mut l_cur_4: i32 = 0;
    let mut l_next_5: i32 = 0;
    let mut l_down_6: i32 = 0;
    let mut l_accel_7: F32 = F32(0);
    let mut l_height_8: F32 = F32(0);
    let r_ = 'f: {
    l_timer_3 = ((p_chr_1).wrapping_add(976)).wrapping_add((((2i32) as u32)).wrapping_mul(4));
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        { let a1_ = { let a1_ = ((D::call(c, fid::BtlAnim_GetId, &[((p_chr_1) as u64)])) as i32); F::BtlActB_PickDownAnim(c, p_chr_1, a1_) }; { let _ = D::call(c, fid::BtlAnim_Play, &[((p_chr_1) as u64), ((a1_) as i64 as u64), ((F32(0x3e199999)).0 as u64)]); } };
        if ((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((17i32) as i64 as u64)])) as i32)) != 0) {
            { let _ = D::call(c, fid::BtlChar_SetHeldFlag, &[((p_chr_1) as u64), ((14i32) as i64 as u64)]); };
        } else {
            { let _ = D::call(c, fid::BtlChar_ClearFlag, &[((p_chr_1) as u64), ((14i32) as i64 as u64)]); };
        }
        { let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlEvent_Raise(c, a0_, 34i32) };
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        l_accel_7 = F32(0x3f6d0979);
        l_down_6 = 0i32;
        { let _ = D::call(c, fid::BtlAnim_AdvanceLoop, &[((p_chr_1) as u64), ((0i32) as i64 as u64)]); };
        if ((((!((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((52i32) as i64 as u64)])) as i32)) != 0)) as i32)) != 0) {
            l_cur_4 = ((D::call(c, fid::BtlAnim_GetId, &[((p_chr_1) as u64)])) as i32);
            l_next_5 = F::BtlActB_PickDownAnim(c, p_chr_1, l_cur_4);
            if (((((l_cur_4) != (l_next_5)) as i32)) != 0) {
                { let _ = D::call(c, fid::BtlAnim_Request, &[((p_chr_1) as u64), ((l_next_5) as i64 as u64), ((F32(0x3e199999)).0 as u64)]); };
            }
        }
        's1: {
            let sv9: i32 = ((D::call(c, fid::BtlAnim_GetId, &[((p_chr_1) as u64)])) as i32);
            match sv9 {
                213i32 | 214i32 => {
                    { let _ = D::call(c, fid::BtlMove_Fall, &[((p_chr_1) as u64)]); };
                    l_down_6 = 1i32;
                    break 's1;
                }
                210i32 | 211i32 => {
                    l_accel_7 = F32(0x3e1e0650);
                    { let _ = D::call(c, fid::BtlMove_Fall, &[((p_chr_1) as u64)]); };
                    if ((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((15i32) as i64 as u64)])) as i32)) != 0) {
                        { let _ = D::call(c, fid::BtlChar_SetFxBit, &[((p_chr_1) as u64), ((51i32) as i64 as u64)]); };
                    }
                    break 's1;
                }
                215i32 | 216i32 => {
                    F::BtlMove_MoveVertical(c, p_chr_1, F32(0x3e8e38e2), F32(0x3f6d0979));
                    l_down_6 = 1i32;
                    break 's1;
                }
                _ => {
                }
            }
        }
        { let _ = D::call(c, fid::BtlMove_Step, &[((p_chr_1) as u64), ((6i32) as i64 as u64), ((5i32) as i64 as u64), ((6i32) as i64 as u64), ((F32(0x00000000)).0 as u64), ((l_accel_7).0 as u64)]); };
        if ((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((17i32) as i64 as u64)])) as i32)) != 0) {
            { let _ = D::call(c, fid::BtlChar_SetHeldFlag, &[((p_chr_1) as u64), ((14i32) as i64 as u64)]); };
        }
        if (({ let a_ = (F::BtlMember_GetActiveGauge(c, p_chr_1)).wrapping_add(40); (c.m.r32(a_) as i32) }) != 0) {
            { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((150i32) as i64 as u64)]); };
            if ((F::BtlChar_IsFree(c, p_chr_1)) != 0) {
                { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((231i32) as i64 as u64)]); };
                if ((F::BtlInput_IsPressed(c, p_chr_1, ((1048576i32) as u32))) != 0) {
                    { let a_ = (p_chr_1).wrapping_add(4096); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                }
            }
        }
        if ((F::BtlChar_IsDead(c, p_chr_1)) != 0) {
            { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((311i32) as i64 as u64)]); };
        }
        if ((l_down_6) != 0) {
            if ((F::BtlActB_TickMemberChange(c, p_chr_1, l_timer_3)) != 0) {
                { let _ = D::call(c, fid::BtlAct_Request, &[((p_chr_1) as u64), ((246i32) as i64 as u64)]); };
            }
        }
    }
    if (((((p_phase_2) == (2i32)) as i32)) != 0) {
        if ((((((((((((((((!((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((15i32) as i64 as u64)])) as i32)) != 0)) as i32)) != 0) && ((((!((F::BtlChar_TestPrevFlag(c, p_chr_1, ((((15i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0)) as i32)) != 0) && ((((!((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((31i32) as i64 as u64)])) as i32)) != 0)) as i32)) != 0)) as i32)) != 0) && ((((!((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((17i32) as i64 as u64)])) as i32)) != 0)) as i32)) != 0)) as i32)) != 0) {
            l_height_8 = F32((D::call(c, fid::BtlAct_GetHeight, &[((p_chr_1) as u64)])) as u32);
            if (((rt::flt(rt::fmul({ let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); F::BtlCharApi_GetHeight(c, a0_) }, F32(0x3e999999)), l_height_8) as i32)) != 0) {
                { let _ = D::call(c, fid::BtlAct_Request, &[((p_chr_1) as u64), ((217i32) as i64 as u64)]); };
            }
        }
        if ((F::BtlChar_IsFree(c, p_chr_1)) != 0) {
            if ((((((c.m.r32((p_chr_1).wrapping_add(2404)) as i32)) > (90i32)) as i32)) != 0) {
                if ((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((17i32) as i64 as u64)])) as i32)) != 0) {
                    { let _ = D::call(c, fid::BtlAct_Request, &[((p_chr_1) as u64), ((230i32) as i64 as u64)]); };
                } else {
                    { let _ = D::call(c, fid::BtlAct_Request, &[((p_chr_1) as u64), ((225i32) as i64 as u64)]); };
                }
            }
            if ((((((((!(({ let a_ = (F::BtlMember_GetActiveGauge(c, p_chr_1)).wrapping_add(40); (c.m.r32(a_) as i32) }) != 0)) as i32)) != 0) || ((((((c.m.r32((p_chr_1).wrapping_add(4096)) as i32)) <= (0i32)) as i32)) != 0)) as i32)) != 0) {
                if ((F::BtlAct_CheckRecoveryInput(c, p_chr_1, 0i32)) != 0) {
                    { let a1_ = ((D::call(c, fid::BtlAct_GetQueued, &[((p_chr_1) as u64)])) as i32); { let _ = D::call(c, fid::BtlAct_Request, &[((p_chr_1) as u64), ((a1_) as i64 as u64)]); } };
                }
            }
            if ((F::BtlChar_IsStage4Or27(c)) != 0) {
                if ((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((23i32) as i64 as u64)])) as i32)) != 0) {
                    { let _ = D::call(c, fid::BtlAct_Request, &[((p_chr_1) as u64), ((230i32) as i64 as u64)]); };
                }
            }
        }
        if ((((((c.m.r32((p_chr_1).wrapping_add(4064)) as i32)) > (0i32)) as i32)) != 0) {
            { let _ = D::call(c, fid::BtlAct_Request, &[((p_chr_1) as u64), ((224i32) as i64 as u64)]); };
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_LieDownHandler", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_FallHandler(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_FallHandler", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let mut l_pos_3: u32 = 0;
    let mut l_air_4: i32 = 0;
    let mut l_land_5: i32 = 0;
    let mut l_blend_6: F32 = F32(0);
    let r_ = 'f: {
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        l_blend_6 = F32(0x3e199999);
        l_air_4 = F::BtlAct_IsAirMotion(c, p_chr_1, 1i32);
        if (((((((D::call(c, fid::BtlAct_GetPrev, &[((p_chr_1) as u64)])) as i32)) == (213i32)) as i32)) != 0) {
            l_blend_6 = F32(0x3e999999);
        }
        { let _ = D::call(c, fid::BtlAnim_Play, &[((p_chr_1) as u64), (((if ((l_air_4) != 0) { 199i32 } else { 198i32 })) as i64 as u64), ((l_blend_6).0 as u64)]); };
        { let _ = D::call(c, fid::BtlChar_ClearFlag, &[((p_chr_1) as u64), ((14i32) as i64 as u64)]); };
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        { let _ = D::call(c, fid::BtlAnim_AdvanceLoop, &[((p_chr_1) as u64), ((0i32) as i64 as u64)]); };
        { let _ = D::call(c, fid::BtlMove_Step, &[((p_chr_1) as u64), ((6i32) as i64 as u64), ((5i32) as i64 as u64), ((7i32) as i64 as u64), ((F32(0x00000000)).0 as u64), ((F32(0x3e1e0650)).0 as u64)]); };
        { let _ = D::call(c, fid::BtlMove_Fall, &[((p_chr_1) as u64)]); };
        F::BtlActB_SetReactionFlags(c, p_chr_1);
        if (({ let a_ = (F::BtlMember_GetActiveGauge(c, p_chr_1)).wrapping_add(40); (c.m.r32(a_) as i32) }) != 0) {
            { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((150i32) as i64 as u64)]); };
            if ((F::BtlChar_IsFree(c, p_chr_1)) != 0) {
                if ((((!((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((29i32) as i64 as u64)])) as i32)) != 0)) as i32)) != 0) {
                    { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((231i32) as i64 as u64)]); };
                    if ((F::BtlInput_IsPressed(c, p_chr_1, ((1048576i32) as u32))) != 0) {
                        { let a_ = (p_chr_1).wrapping_add(4096); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                    }
                }
            }
        }
    }
    if (((((p_phase_2) == (2i32)) as i32)) != 0) {
        l_land_5 = 0i32;
        if ((F::BtlChar_IsStage4Or27(c)) != 0) {
            if ((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((15i32) as i64 as u64)])) as i32)) != 0) {
                l_land_5 = 1i32;
            } else {
                if ((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((23i32) as i64 as u64)])) as i32)) != 0) {
                    if ((F::BtlChar_IsFree(c, p_chr_1)) != 0) {
                        { let _ = D::call(c, fid::BtlAct_Request, &[((p_chr_1) as u64), ((230i32) as i64 as u64)]); };
                    }
                }
            }
        } else {
            if (((((((((((rt::flt(F::BtlAct_GetFramesToGround(c, p_chr_1), F32(0x40000000)) as i32)) != 0) || ((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((15i32) as i64 as u64)])) as i32)) != 0)) as i32)) != 0) || ((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((31i32) as i64 as u64)])) as i32)) != 0)) as i32)) != 0) {
                l_land_5 = 1i32;
            }
        }
        if ((l_land_5) != 0) {
            { let _ = D::call(c, fid::BtlAct_Request, &[((p_chr_1) as u64), ((220i32) as i64 as u64)]); };
            if ((((((F::BtlChar_IsFree(c, p_chr_1)) != 0) && ((((!((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((30i32) as i64 as u64)])) as i32)) != 0)) as i32)) != 0)) as i32)) != 0) {
                { let _ = D::call(c, fid::BtlColl_AddActionBit, &[((p_chr_1) as u64), ((37i32) as i64 as u64)]); };
                if ((F::BtlInput_IsHeld(c, p_chr_1, ((240i32) as u32))) != 0) {
                    if ((((!((F::BtlParam_CanFly(c, p_chr_1)) != 0)) as i32)) != 0) {
                        { let _ = D::call(c, fid::BtlAct_Request, &[((p_chr_1) as u64), ((37i32) as i64 as u64)]); };
                    } else {
                        { let _ = D::call(c, fid::BtlAct_Request, &[((p_chr_1) as u64), ((39i32) as i64 as u64)]); };
                    }
                }
            }
            if (((({ let a_ = (((D::call(c, fid::BtlChar_GetPos, &[((p_chr_1) as u64)])) as u32)).wrapping_add(208); (c.m.r32(a_) as i32) }) & (64i32))) != 0) {
                l_pos_3 = ((D::call(c, fid::BtlChar_GetPos, &[((p_chr_1) as u64)])) as u32);
                if (((rt::flt(F::BtlStage_GetBottom(c), F32(c.m.r32((l_pos_3).wrapping_add(4)))) as i32)) != 0) {
                    if ((F::BtlActB_TickMemberChange(c, p_chr_1, ((0i32) as u32))) != 0) {
                        { let _ = D::call(c, fid::BtlAct_Request, &[((p_chr_1) as u64), ((246i32) as i64 as u64)]); };
                    }
                }
            }
        }
        if ((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((17i32) as i64 as u64)])) as i32)) != 0) {
            { let a1_ = F::BtlAct_GetDownAction(c, p_chr_1); { let _ = D::call(c, fid::BtlAct_Request, &[((p_chr_1) as u64), ((a1_) as i64 as u64)]); } };
        }
        if ((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((22i32) as i64 as u64)])) as i32)) != 0) {
            if ((F::BtlChar_IsFree(c, p_chr_1)) != 0) {
                { let _ = D::call(c, fid::BtlAct_Request, &[((p_chr_1) as u64), ((230i32) as i64 as u64)]); };
            }
        }
        if ((F::BtlInput_IsPressed(c, p_chr_1, ((1048576i32) as u32))) != 0) {
            if ((F::BtlChar_IsFree(c, p_chr_1)) != 0) {
                if ((((!((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((29i32) as i64 as u64)])) as i32)) != 0)) as i32)) != 0) {
                    if ((((((((!(({ let a_ = (F::BtlMember_GetActiveGauge(c, p_chr_1)).wrapping_add(40); (c.m.r32(a_) as i32) }) != 0)) as i32)) != 0) || ((((((c.m.r32((p_chr_1).wrapping_add(4096)) as i32)) <= (0i32)) as i32)) != 0)) as i32)) != 0) {
                        { let _ = D::call(c, fid::BtlAct_Request, &[((p_chr_1) as u64), ((230i32) as i64 as u64)]); };
                    }
                }
            }
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_FallHandler", &PROBE_N, &[], true); }
    r_
}
