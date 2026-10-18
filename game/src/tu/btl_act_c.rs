#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn BtlAct_DownHandler(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_DownHandler", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let mut l_air_3: i32 = 0;
    let mut l_anim_4: i32 = 0;
    let mut l_common_5: i32 = 0;
    let mut l_bank8_6: i32 = 0;
    let mut l_voice_7: i32 = 0;
    let mut l_fx_8: i32 = 0;
    let mut l_clear_9: i32 = 0;
    let r_ = 'f: {
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        l_common_5 = (1i32).wrapping_neg();
        l_air_3 = F::BtlAct_IsAirMotion(c, p_chr_1, 0i32);
        l_anim_4 = 0i32;
        l_bank8_6 = (1i32).wrapping_neg();
        l_voice_7 = (1i32).wrapping_neg();
        l_fx_8 = (1i32).wrapping_neg();
        l_clear_9 = 1i32;
        's1: {
            let sv10: i32 = F::BtlAct_GetCurrent(c, p_chr_1);
            match sv10 {
                218i32 => {
                    l_anim_4 = (if ((l_air_3) != 0) { 186i32 } else { 185i32 });
                    l_clear_9 = 0i32;
                    break 's1;
                }
                219i32 => {
                    l_anim_4 = (if ((l_air_3) != 0) { 203i32 } else { 202i32 });
                    l_bank8_6 = 2i32;
                    l_fx_8 = 52i32;
                    break 's1;
                }
                220i32 => {
                    l_anim_4 = (if ((l_air_3) != 0) { 205i32 } else { 204i32 });
                    l_bank8_6 = 3i32;
                    l_voice_7 = 0i32;
                    l_fx_8 = 52i32;
                    break 's1;
                }
                221i32 => {
                    l_anim_4 = (if ((l_air_3) != 0) { 207i32 } else { 206i32 });
                    l_common_5 = 37i32;
                    l_voice_7 = 1i32;
                    l_fx_8 = 53i32;
                    break 's1;
                }
                222i32 => {
                    l_anim_4 = (if ((l_air_3) != 0) { 203i32 } else { 202i32 });
                    l_common_5 = 37i32;
                    l_voice_7 = 1i32;
                    l_fx_8 = 53i32;
                    break 's1;
                }
                _ => {
                }
            }
        }
        F::BtlAnim_Play(c, p_chr_1, l_anim_4, F32(0x00000000));
        if ((l_clear_9) != 0) {
            F::BtlChar_ClearFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32));
        }
        if (((((l_voice_7) >= (0i32)) as i32)) != 0) {
            F::BtlChar_PlayVoice(c, p_chr_1, l_voice_7);
        }
        if (((((l_common_5) >= (0i32)) as i32)) != 0) {
            F::BtlCharSnd_PlayCommon(c, p_chr_1, l_common_5);
        }
        if (((((l_bank8_6) >= (0i32)) as i32)) != 0) {
            F::BtlCharSnd_PlayBank8(c, p_chr_1, l_bank8_6);
        }
        if (((((l_fx_8) >= (0i32)) as i32)) != 0) {
            F::BtlChar_SetFxBit(c, p_chr_1, l_fx_8);
        }
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        if ((F::BtlAnim_Advance(c, p_chr_1, 0i32)) != 0) {
            's2: {
                let sv11: i32 = F::BtlAct_GetCurrent(c, p_chr_1);
                match sv11 {
                    218i32 | 219i32 | 222i32 => {
                        { let a1_ = F::BtlAct_GetDownAction(c, p_chr_1); F::BtlAct_Request(c, p_chr_1, a1_) };
                        break 's2;
                    }
                    220i32 | 221i32 => {
                        F::BtlAct_Request(c, p_chr_1, 219i32);
                        break 's2;
                    }
                    _ => {
                    }
                }
            }
            if ((((((((((F::BtlChar_IsStage4Or27(c)) != 0) && ((F::BtlChar_IsFree(c, p_chr_1)) != 0)) as i32)) != 0) && ((F::BtlChar_TestFlag(c, p_chr_1, ((((23i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) {
                F::BtlAct_Request(c, p_chr_1, 230i32);
            }
        }
        F::BtlMove_Step(c, p_chr_1, 6i32, 5i32, 6i32, F32(0x00000000), F32(0x3e1e0650));
        F::BtlMove_ApplyGravity(c, p_chr_1);
        F::BtlActB_SetReactionFlags(c, p_chr_1);
        if ((((((((({ let a_ = (F::BtlMember_GetActiveGauge(c, p_chr_1)).wrapping_add(40); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) && ((F::BtlChar_IsFree(c, p_chr_1)) != 0)) as i32)) != 0) {
            F::BtlChar_SetFlag(c, p_chr_1, ((((231i32) as i64 as u64)) as u32));
            if ((F::BtlInput_IsPressed(c, p_chr_1, ((1048576i32) as u32))) != 0) {
                { let a_ = (p_chr_1).wrapping_add(4096); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
            }
        }
    }
    if (((((p_phase_2) == (2i32)) as i32)) != 0) {
        if ((F::BtlChar_IsFree(c, p_chr_1)) != 0) {
            if ((((((((({ let a_ = (F::BtlMember_GetActiveGauge(c, p_chr_1)).wrapping_add(40); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) || ((((((c.m.r32((p_chr_1).wrapping_add(4096)) as i32)) <= (0i32)) as i32)) != 0)) as i32)) != 0) {
                if ((((((c.m.r32((p_chr_1).wrapping_add(3396)) as i32)) >= (10i32)) as i32)) != 0) {
                    if ((F::BtlAct_CheckRecoveryInput(c, p_chr_1, 0i32)) != 0) {
                        { let a1_ = F::BtlAct_GetQueued(c, p_chr_1); F::BtlAct_Request(c, p_chr_1, a1_) };
                    }
                }
            }
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_DownHandler", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_GetUpHandler(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_GetUpHandler", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let mut l_anim_3: i32 = 0;
    let mut l_air_4: i32 = 0;
    let mut l_pose_5: u32 = 0;
    let r_ = 'f: {
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        l_anim_3 = 0i32;
        l_air_4 = F::BtlAct_IsAirMotion(c, p_chr_1, 1i32);
        's1: {
            let sv6: i32 = F::BtlAct_GetCurrent(c, p_chr_1);
            match sv6 {
                225i32 => {
                    l_anim_3 = (if ((l_air_4) != 0) { 227i32 } else { 226i32 });
                    break 's1;
                }
                226i32 => {
                    l_anim_3 = (if ((l_air_4) != 0) { 229i32 } else { 228i32 });
                    break 's1;
                }
                227i32 => {
                    l_anim_3 = (if ((l_air_4) != 0) { 231i32 } else { 230i32 });
                    break 's1;
                }
                228i32 => {
                    l_anim_3 = 254i32;
                    break 's1;
                }
                229i32 => {
                    l_anim_3 = 254i32;
                    l_pose_5 = F::BtlChar_GetPos(c, p_chr_1);
                    { let v_ = { let a0_ = rt::fadd({ let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(148); F32(c.m.r32(a_)) }, F32(0x40490fda)); F::BtlUtil_WrapAngle(c, a0_) }; let a_ = (l_pose_5).wrapping_add(148); c.m.w32(a_, (v_).0); };
                    break 's1;
                }
                _ => {
                }
            }
        }
        F::BtlAnim_Play(c, p_chr_1, l_anim_3, F32(0x3e199999));
        if (((((F::BtlAct_GetCurrent(c, p_chr_1)) != (225i32)) as i32)) != 0) {
            F::BtlCharSnd_PlayCommon(c, p_chr_1, 31i32);
        }
        if (((((F::BtlAct_GetCurrent(c, p_chr_1)) == (225i32)) as i32)) != 0) {
            if ((((((((({ let a_ = (F::BtlMember_GetActiveGauge(c, p_chr_1)).wrapping_add(32); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) && ((((({ let a_ = (F::BtlMember_GetActiveGauge(c, p_chr_1)).wrapping_add(44); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0)) as i32)) != 0) {
                { let a1_ = (if ((F::BtlChar_FrameMod(c, 2i32)) != 0) { 32i32 } else { 33i32 }); F::BtlChar_PlayVoice(c, p_chr_1, a1_) };
                { let v_ = 1i32; let a_ = (F::BtlMember_GetActiveGauge(c, p_chr_1)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            }
        }
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        if ((F::BtlAnim_Advance(c, p_chr_1, 0i32)) != 0) {
            { let a1_ = F::BtlAct_GetIdleFollowUp(c, p_chr_1); F::BtlAct_Request(c, p_chr_1, a1_) };
        }
        F::BtlMove_Step(c, p_chr_1, 6i32, 5i32, 6i32, F32(0x00000000), F32(0x3eed0979));
        F::BtlMove_ApplyGravity(c, p_chr_1);
        F::BtlChar_SetFlag(c, p_chr_1, ((((65i32) as i64 as u64)) as u32));
        F::BtlMove_RequestOrbit(c, p_chr_1, F32(0x41a00000), F32(0x42480000));
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((17i32) as i64 as u64)) as u32))) != 0) {
            F::BtlChar_SetHeldFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32));
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_GetUpHandler", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_ActionE6(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_ActionE6", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let mut l_blend_3: F32 = F32(0);
    let mut l_air_4: i32 = 0;
    let mut l_anim_5: i32 = 0;
    let r_ = 'f: {
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        l_blend_3 = F32(0x3e199999);
        l_air_4 = F::BtlAct_IsAirMotion(c, p_chr_1, 1i32);
        if (((((F::BtlAct_GetPrev(c, p_chr_1)) == (212i32)) as i32)) != 0) {
            l_blend_3 = F32(0x00000000);
        }
        l_anim_5 = (if ((l_air_4) != 0) { 235i32 } else { 234i32 });
        F::BtlAnim_Play(c, p_chr_1, l_anim_5, l_blend_3);
        F::BtlChar_SetHeldFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32));
        F::BtlCharSnd_PlayCommon(c, p_chr_1, 33i32);
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((5i32) as i64 as u64)) as u32))) != 0) {
            if ((((F::BtlAnim_GetFlags(c, ((l_anim_5) as u32))) & (((32768i32) as u32)))) != 0) {
                F::BtlMove_TurnYaw(c, p_chr_1, 3i32, F32(0x40c90fda));
            } else {
                F::BtlMove_TurnYaw(c, p_chr_1, 2i32, F32(0x40c90fda));
            }
        }
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        if ((F::BtlAnim_Advance(c, p_chr_1, 0i32)) != 0) {
            { let a1_ = F::BtlAct_GetIdleFollowUp(c, p_chr_1); F::BtlAct_Request(c, p_chr_1, a1_) };
        }
        F::BtlMove_Step(c, p_chr_1, 6i32, 5i32, 7i32, F32(0x00000000), F32(0x3eed0979));
        F::BtlMove_ApplyGravity(c, p_chr_1);
        F::BtlChar_SetFlag(c, p_chr_1, ((((65i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((149i32) as i64 as u64)) as u32));
    }
    if (((((p_phase_2) == (2i32)) as i32)) != 0) {
        if (((rt::flt(F32(0x3ecccccc), F::BtlAnim_GetProgress(c, p_chr_1)) as i32)) != 0) {
            if ((((!((F::BtlAct_HasQueued(c, p_chr_1)) != 0)) as i32)) != 0) {
                { let _ = F::BtlDecide_Main(c, p_chr_1, 531560i32); };
                { let _ = F::BtlDecide_Attack(c, p_chr_1, 15i32); };
                { let _ = F::BtlDecide_Common(c, p_chr_1, 6i32); };
            }
            { let a1_ = F::BtlAct_GetQueued(c, p_chr_1); F::BtlAct_Request(c, p_chr_1, a1_) };
        }
    }
    if (((((p_phase_2) == (3i32)) as i32)) != 0) {
        break 'f { F::BtlChar_SetFlag(c, p_chr_1, ((((51i32) as i64 as u64)) as u32)); 0 };
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_ActionE6", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_ActionE7(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_ActionE7", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let mut l_zero_3: F32 = F32(0);
    let r_ = 'f: {
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        l_zero_3 = F32(0x00000000);
        { let a1_ = (if ((F::BtlAct_IsAirMotion(c, p_chr_1, 0i32)) != 0) { 201i32 } else { 200i32 }); F::BtlAnim_Play(c, p_chr_1, a1_, l_zero_3) };
        F::BtlChar_ClearFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32));
        { let v_ = l_zero_3; let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(152); c.m.w32(a_, (v_).0); };
        F::BtlChar_PlayVoice(c, p_chr_1, 0i32);
        F::BtlCharSnd_PlayCommon(c, p_chr_1, 50i32);
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        if ((F::BtlAnim_Advance(c, p_chr_1, 0i32)) != 0) {
            if ((F::BtlChar_TestFlag(c, p_chr_1, ((((15i32) as i64 as u64)) as u32))) != 0) {
                F::BtlAct_Request(c, p_chr_1, 219i32);
            } else {
                if ((F::BtlChar_TestFlag(c, p_chr_1, ((((17i32) as i64 as u64)) as u32))) != 0) {
                    { let a1_ = F::BtlAct_GetDownAction(c, p_chr_1); F::BtlAct_Request(c, p_chr_1, a1_) };
                } else {
                    F::BtlAct_Request(c, p_chr_1, 217i32);
                }
            }
        }
        F::BtlMove_Step(c, p_chr_1, 6i32, 5i32, 6i32, F32(0x00000000), F32(0x3f6d0979));
        F::BtlMove_ApplyGravity(c, p_chr_1);
        F::BtlActB_SetReactionFlags(c, p_chr_1);
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_ActionE7", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_ActionE8E9(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_ActionE8E9", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let mut l_air_3: i32 = 0;
    let mut l_zero_4: F32 = F32(0);
    let r_ = 'f: {
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        l_air_3 = F::BtlAct_IsAirMotion(c, p_chr_1, 0i32);
        l_zero_4 = F32(0x00000000);
        F::BtlAnim_Play(c, p_chr_1, (if ((l_air_3) != 0) { 209i32 } else { 208i32 }), l_zero_4);
        's1: {
            let sv5: i32 = F::BtlAct_GetCurrent(c, p_chr_1);
            match sv5 {
                232i32 => {
                    F::BtlMove_TurnYaw(c, p_chr_1, (if ((l_air_3) != 0) { 8i32 } else { 7i32 }), F32(0x40490fda));
                    F::BtlMove_SetDirection(c, p_chr_1, 2i32);
                    { let v_ = F32(0x40cf684a); let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(152); c.m.w32(a_, (v_).0); };
                    break 's1;
                }
                233i32 => {
                    { let v_ = l_zero_4; let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(152); c.m.w32(a_, (v_).0); };
                    break 's1;
                }
                _ => {
                }
            }
        }
        F::BtlChar_SetHeldFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32));
        F::BtlChar_PlayVoice(c, p_chr_1, 0i32);
        F::BtlCharSnd_PlayBank8(c, p_chr_1, 9i32);
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        if ((F::BtlAnim_Advance(c, p_chr_1, 0i32)) != 0) {
            F::BtlAct_Request(c, p_chr_1, 11i32);
        }
        F::BtlMove_Step(c, p_chr_1, 6i32, 5i32, 2i32, F32(0x00000000), F32(0x3f6d0979));
        F::BtlMove_ApplyGravity(c, p_chr_1);
        F::BtlActB_SetReactionFlags(c, p_chr_1);
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_ActionE8E9", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_ActionEA(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_ActionEA", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let r_ = 'f: {
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        F::BtlAnim_Play(c, p_chr_1, 212i32, F32(0x3e199999));
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        F::BtlAnim_AdvanceLoop(c, p_chr_1, 0i32);
        F::BtlMove_Step(c, p_chr_1, 6i32, 6i32, 7i32, F32(0x00000000), F32(0x3f6d0979));
        F::BtlMove_ApplyGravity(c, p_chr_1);
        F::BtlActB_SetReactionFlags(c, p_chr_1);
    }
    if (((((p_phase_2) == (2i32)) as i32)) != 0) {
        if ((((!((F::BtlChar_TestFlag(c, p_chr_1, ((((190i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) {
            F::BtlAct_Request(c, p_chr_1, 11i32);
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_ActionEA", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_ActionEB(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_ActionEB", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let mut l_work_3: u32 = 0;
    let r_ = 'f: {
    l_work_3 = ((p_chr_1).wrapping_add(976)).wrapping_add((((2i32) as u32)).wrapping_mul(4));
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        F::BtlAnim_Play(c, p_chr_1, 0i32, F32(0x00000000));
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        F::BtlAnim_AdvanceLoop(c, p_chr_1, 0i32);
        F::BtlMove_Step(c, p_chr_1, 6i32, 6i32, 7i32, F32(0x00000000), F32(0x3f6d0979));
        F::BtlMove_BrakeVertical(c, p_chr_1);
        F::BtlChar_SetFlag(c, p_chr_1, ((((11i32) as i64 as u64)) as u32));
        if ((F::BtlActB_TickMemberChange(c, p_chr_1, l_work_3)) != 0) {
            F::BtlAct_Request(c, p_chr_1, 246i32);
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_ActionEB", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_StunHandler(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_StunHandler", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let r_ = 'f: {
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        F::BtlAnim_Play(c, p_chr_1, 183i32, F32(0x00000000));
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        F::BtlAnim_AdvanceLoop(c, p_chr_1, 0i32);
        F::BtlMove_Step(c, p_chr_1, 6i32, 6i32, 7i32, F32(0x00000000), F32(0x3f6d0979));
        F::BtlMove_ApplyGravity(c, p_chr_1);
        F::BtlActB_SetReactionFlags(c, p_chr_1);
    }
    if (((((p_phase_2) == (2i32)) as i32)) != 0) {
        if ((((((c.m.r32((p_chr_1).wrapping_add(4064)) as i32)) <= (0i32)) as i32)) != 0) {
            { let a1_ = F::BtlAct_GetIdleFollowUp(c, p_chr_1); F::BtlAct_Request(c, p_chr_1, a1_) };
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_StunHandler", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_AirStunHandler(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_AirStunHandler", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let mut l_accel_3: F32 = F32(0);
    let r_ = 'f: {
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        { let a1_ = (if ((F::BtlAct_IsAirMotion(c, p_chr_1, 0i32)) != 0) { 211i32 } else { 210i32 }); F::BtlAnim_Play(c, p_chr_1, a1_, F32(0x00000000)) };
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        F::BtlAnim_AdvanceLoop(c, p_chr_1, 0i32);
        l_accel_3 = F32(0x3f6d0979);
        F::BtlMove_Step(c, p_chr_1, 6i32, 6i32, 7i32, F32(0x00000000), l_accel_3);
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((17i32) as i64 as u64)) as u32))) != 0) {
            F::BtlMove_MoveVertical(c, p_chr_1, F32(0x3e8e38e2), l_accel_3);
        } else {
            F::BtlMove_Fall(c, p_chr_1);
        }
        F::BtlActB_SetReactionFlags(c, p_chr_1);
        F::BtlChar_SetFlag(c, p_chr_1, ((((42i32) as i64 as u64)) as u32));
    }
    if (((((p_phase_2) == (2i32)) as i32)) != 0) {
        if ((((((c.m.r32((p_chr_1).wrapping_add(4064)) as i32)) <= (0i32)) as i32)) != 0) {
            { let a1_ = F::BtlAct_GetDownAction(c, p_chr_1); F::BtlAct_Request(c, p_chr_1, a1_) };
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_AirStunHandler", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_GrowSearchWindow(c: &mut Ctx, mut p_chr_1: u32, mut p_limit_2: F32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_GrowSearchWindow", &PROBE_N, &[(p_chr_1) as u32, (p_limit_2).0], false); }
    let mut l_slow_3: i32 = 0;
    let mut l_dAngle_4: F32 = F32(0);
    let mut l_dRange_5: F32 = F32(0);
    let mut l_old_6: F32 = F32(0);
    let r_ = 'f: {
    l_slow_3 = (((F::BtlOpp_GetParamWord0(c, p_chr_1)).wrapping_shr((7i32) as u32)) & (1i32));
    l_dAngle_4 = F::BtlParam_GetTypeValueA(c, p_chr_1);
    l_dRange_5 = F::BtlParam_GetTypeValueB(c, p_chr_1);
    if ((l_slow_3) != 0) {
        { let r_ = F32(0x3f199999); let n_ = rt::fmul(l_dAngle_4, r_); l_dAngle_4 = n_; };
        { let r_ = F32(0x3f199999); let n_ = rt::fmul(l_dRange_5, r_); l_dRange_5 = n_; };
    }
    l_old_6 = F32(c.m.r32((p_chr_1).wrapping_add(3416)));
    { let v_ = rt::fadd(l_old_6, l_dAngle_4); let a_ = (p_chr_1).wrapping_add(3416); c.m.w32(a_, (v_).0); };
    if (((rt::flt(F32(0x40490fda), F32(c.m.r32((p_chr_1).wrapping_add(3416)))) as i32)) != 0) {
        { let v_ = F32(0x40490fda); let a_ = (p_chr_1).wrapping_add(3416); c.m.w32(a_, (v_).0); };
    }
    { let r_ = l_dRange_5; let a_ = (p_chr_1).wrapping_add(3420); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
    if (((rt::flt(F32(0x49742400), F32(c.m.r32((p_chr_1).wrapping_add(3420)))) as i32)) != 0) {
        { let v_ = F32(0x49742400); let a_ = (p_chr_1).wrapping_add(3420); c.m.w32(a_, (v_).0); };
    }
    if (((((((rt::flt(l_old_6, p_limit_2) as i32)) != 0) && (((rt::fle(p_limit_2, F32(c.m.r32((p_chr_1).wrapping_add(3416)))) as i32)) != 0)) as i32)) != 0) {
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_GrowSearchWindow", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_IsOppInSearchWindow(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_IsOppInSearchWindow", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_ret_2: i32 = 0;
    let mut l_flag_3: i32 = 0;
    let mut l_dist_4: F32 = F32(0);
    let mut l_yaw_5: F32 = F32(0);
    let r_ = 'f: {
    l_ret_2 = 0i32;
    if ((F::BtlChar_TestFlag(c, p_chr_1, ((((178i32) as i64 as u64)) as u32))) != 0) {
        break 'f 0i32;
    }
    { let _ = F::BtlOpp_GetParamWord0(c, p_chr_1); };
    l_flag_3 = F::BtlChar_TestFlag(c, p_chr_1, ((((186i32) as i64 as u64)) as u32));
    l_dist_4 = F::BtlOpp_GetDistanceXZ(c, p_chr_1);
    l_yaw_5 = { let a0_ = F::BtlOpp_GetYawFromFacing(c, p_chr_1); rt::ffabs(a0_) };
    if (((rt::fle(l_yaw_5, F32(c.m.r32((p_chr_1).wrapping_add(3416)))) as i32)) != 0) {
        if (((rt::flt(l_dist_4, rt::fmul(F32(c.m.r32((p_chr_1).wrapping_add(3420))), rt::fadd(rt::fmul(rt::fsub(F32(0x3f800000), rt::fdiv(l_yaw_5, F32(0x40490fda))), F32(0x3f599999)), F32(0x3e199999)))) as i32)) != 0) {
            l_ret_2 = (((l_flag_3) == (0i32)) as i32);
            if ((F::BtlMember_HasAbility(c, p_chr_1, 108i32)) != 0) {
                if (((((F::BtlAnim_GetId(c, p_chr_1)) == (382i32)) as i32)) != 0) {
                    l_ret_2 = 1i32;
                }
            }
        }
    }
    break 'f l_ret_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_IsOppInSearchWindow", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_SearchHandler(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_SearchHandler", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let mut l_grow_3: i32 = 0;
    let r_ = 'f: {
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        if ((F::BtlParam_IsType2to4(c, p_chr_1)) != 0) {
            F::BtlAnim_Play(c, p_chr_1, 383i32, F32(0x3e199999));
        } else {
            F::BtlAnim_Play(c, p_chr_1, 382i32, F32(0x3e199999));
        }
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        l_grow_3 = 0i32;
        's1: {
            let sv4: i32 = F::BtlAnim_GetId(c, p_chr_1);
            match sv4 {
                383i32 => {
                    { let _ = F::BtlAnim_AdvanceThen(c, p_chr_1, 382i32, F32(0x3e199999), 0i32); };
                    if (((rt::flt(F32(0x3f000000), F::BtlAnim_GetProgress(c, p_chr_1)) as i32)) != 0) {
                        l_grow_3 = 1i32;
                    }
                    break 's1;
                }
                382i32 => {
                    F::BtlAnim_AdvanceLoop(c, p_chr_1, 0i32);
                    l_grow_3 = 1i32;
                    break 's1;
                }
                _ => {
                }
            }
        }
        if ((l_grow_3) != 0) {
            if ((F::BtlAct_GrowSearchWindow(c, p_chr_1, F32(0x3fc90fda))) != 0) {
                F::BtlChar_PlayVoice(c, p_chr_1, 38i32);
            }
        }
        if ((F::BtlAct_IsOppInSearchWindow(c, p_chr_1)) != 0) {
            F::BtlChar_SetHeldFlag(c, p_chr_1, ((((5i32) as i64 as u64)) as u32));
            F::BtlMove_TurnYaw(c, p_chr_1, 2i32, F32(0x40490fda));
            F::BtlChar_PlayVoice(c, p_chr_1, 39i32);
            F::BtlCharSnd_PlayCommon(c, p_chr_1, 44i32);
            if ((((!((F::BtlChar_TestFlag(c, p_chr_1, ((((186i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) {
                F::BtlChar_SetFxBit(c, p_chr_1, 21i32);
            }
        }
        F::BtlMove_Step(c, p_chr_1, 6i32, 6i32, 7i32, F32(0x00000000), F32(0x3f6d0979));
        F::BtlMove_ApplyGravity(c, p_chr_1);
        F::BtlChar_SetFlag(c, p_chr_1, ((((201i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((27i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((28i32) as i64 as u64)) as u32));
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((17i32) as i64 as u64)) as u32))) != 0) {
            F::BtlChar_SetHeldFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32));
        }
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((15i32) as i64 as u64)) as u32))) != 0) {
            F::BtlChar_ClearFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32));
        }
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((5i32) as i64 as u64)) as u32))) != 0) {
            F::BtlAct_Request(c, p_chr_1, 11i32);
        }
    }
    if (((((p_phase_2) == (2i32)) as i32)) != 0) {
        { let _ = F::BtlDecide_Main(c, p_chr_1, 4726763i32); };
        { let _ = F::BtlDecide_Attack(c, p_chr_1, 3i32); };
        { let _ = F::BtlDecide_Common(c, p_chr_1, 6i32); };
        { let a1_ = F::BtlAct_GetQueued(c, p_chr_1); F::BtlAct_Request(c, p_chr_1, a1_) };
    }
    if (((((p_phase_2) == (3i32)) as i32)) != 0) {
        { let v_ = F32(0x3f000000); let a_ = (p_chr_1).wrapping_add(3416); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x43fa0000); let a_ = (p_chr_1).wrapping_add(3420); c.m.w32(a_, (v_).0); };
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_SearchHandler", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_ChargeHandler(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_ChargeHandler", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_yaw_3: F32 = F32(0);
    let mut l_accel_4: F32 = F32(0);
    let mut l_amount_5: i32 = 0;
    let r_ = 'f: {
        if (((((p_phase_2) == (0i32)) as i32)) != 0) {
            F::BtlAnim_Play(c, p_chr_1, 52i32, F32(0x3e199999));
        }
        if (((((p_phase_2) == (1i32)) as i32)) != 0) {
            's1: {
                let sv6: i32 = F::BtlAnim_GetId(c, p_chr_1);
                match sv6 {
                    52i32 => {
                        if ((F::BtlAnim_AdvanceThen(c, p_chr_1, 53i32, F32(0x3e199999), 1i32)) != 0) {
                            { let a1_ = F::BtlParam_GetChargeStartSound(c, p_chr_1); F::BtlCharSnd_PlayCommon(c, p_chr_1, a1_) };
                            F::BtlChar_PlayVoice(c, p_chr_1, 13i32);
                        }
                        break 's1;
                    }
                    53i32 => {
                        F::BtlAnim_AdvanceLoop(c, p_chr_1, 0i32);
                        { let a1_ = F::BtlParam_GetChargeLoopSound(c, p_chr_1); F::BtlCharSnd_PlayCommon(c, p_chr_1, a1_) };
                        F::BtlChar_SetFxBit(c, p_chr_1, 7i32);
                        F::BtlChar_SetFlag(c, p_chr_1, ((((12i32) as i64 as u64)) as u32));
                        if (((((F::BtlParam_GetUnk0(c, p_chr_1)) != (128i32)) as i32)) != 0) {
                            l_yaw_3 = { let a_ = ((F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(16)).wrapping_add(4); F32(c.m.r32(a_)) };
                            { let v_ = rt::fneg(F::Mathf_Sin(c, l_yaw_3)); let a_ = fp; c.m.w32(a_, (v_).0); };
                            { let v_ = F32(0xbf800000); let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
                            { let v_ = rt::fneg(F::Mathf_Cos(c, l_yaw_3)); let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_).0); };
                            F::BtlAct_PushDir(c, p_chr_1, fp, F32(0x3e999999), F32(0x40400000));
                        }
                        F::BtlChar_SetVibration(c, p_chr_1, F32(0x3f4ccccc), F32(0x3dcccccc));
                        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((15i32) as i64 as u64)) as u32))) != 0) {
                            F::BtlChar_SetFxBit(c, p_chr_1, 54i32);
                        }
                        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((17i32) as i64 as u64)) as u32))) != 0) {
                            l_amount_5 = F::BtlParam_GetRateB(c, p_chr_1);
                        } else {
                            l_amount_5 = F::BtlParam_GetRateA(c, p_chr_1);
                        }
                        { let _ = F::BtlMember_AddKi(c, p_chr_1, l_amount_5); };
                        if ((F::BtlMember_IsKiFull(c, p_chr_1)) != 0) {
                            if ((F::BtlMember_HasBlast(c, p_chr_1, 100000i32)) != 0) {
                                { let a1_ = F::BtlParam_GetStepA(c, p_chr_1); F::BtlMember_AddMaxPower(c, p_chr_1, a1_) };
                                F::BtlChar_SetFlag(c, p_chr_1, ((((187i32) as i64 as u64)) as u32));
                            }
                            F::BtlStat_ClearPenalty(c, p_chr_1);
                        }
                        if ((F::BtlMember_IsMaxPowerFull(c, p_chr_1)) != 0) {
                            F::BtlAnim_Request(c, p_chr_1, 54i32, F32(0x3e199999));
                            F::BtlChar_SetFxBit(c, p_chr_1, 8i32);
                            F::BtlChar_SetFlag(c, p_chr_1, ((((13i32) as i64 as u64)) as u32));
                            F::BtlMember_SubBlast(c, p_chr_1, 100000i32);
                            { let a1_ = F::BtlParam_GetMaxPowerSound(c, p_chr_1); F::BtlCharSnd_PlayCommon(c, p_chr_1, a1_) };
                            F::BtlChar_PlayVoice(c, p_chr_1, 19i32);
                            F::BtlChar_SetHeldFlag(c, p_chr_1, ((((6i32) as i64 as u64)) as u32));
                            F::ChrCam_AddShake(c, p_chr_1, F32(0x40800000), F32(0x3e999999));
                            F::BtlChar_Vibrate(c, p_chr_1, F32(0x3f4ccccc), F32(0x3ecccccc));
                        }
                        break 's1;
                    }
                    54i32 => {
                        if ((F::BtlAnim_Advance(c, p_chr_1, 0i32)) != 0) {
                            { let a1_ = F::BtlAct_GetIdleFollowUp(c, p_chr_1); F::BtlAct_Request(c, p_chr_1, a1_) };
                        }
                        if ((((!((((F::BtlParam_GetFlags(c, p_chr_1)) & (((16i32) as u32)))) != 0)) as i32)) != 0) {
                            if ((F::BtlAnim_PassedRatio(c, p_chr_1, F32(0x3e800000))) != 0) {
                                F::BtlChar_SetFlag(c, p_chr_1, ((((74i32) as i64 as u64)) as u32));
                            }
                        }
                        F::BtlChar_SetFlag(c, p_chr_1, ((((187i32) as i64 as u64)) as u32));
                        F::BtlChar_SetFlag(c, p_chr_1, ((((89i32) as i64 as u64)) as u32));
                        break 's1;
                    }
                    _ => {
                    }
                }
            }
            l_accel_4 = F32(0x3f6d0979);
            F::BtlMove_Step(c, p_chr_1, 6i32, 6i32, 7i32, F32(0x00000000), l_accel_4);
            if ((((((((((((!((F::BtlParam_CanFly(c, p_chr_1)) != 0)) as i32)) != 0) && ((F::BtlChar_TestFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) && ((((!((F::BtlChar_TestFlag(c, p_chr_1, ((((17i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0)) as i32)) != 0) {
                F::BtlMove_MoveVertical(c, p_chr_1, l_accel_4, l_accel_4);
            } else {
                F::BtlMove_ApplyGravity(c, p_chr_1);
            }
            F::BtlChar_SetFlag(c, p_chr_1, ((((201i32) as i64 as u64)) as u32));
            F::BtlChar_SetFlag(c, p_chr_1, ((((28i32) as i64 as u64)) as u32));
            if ((F::BtlChar_TestFlag(c, p_chr_1, ((((17i32) as i64 as u64)) as u32))) != 0) {
                F::BtlChar_SetHeldFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32));
            }
            if ((F::BtlChar_TestFlag(c, p_chr_1, ((((15i32) as i64 as u64)) as u32))) != 0) {
                F::BtlChar_ClearFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32));
            }
        }
        if (((((p_phase_2) == (2i32)) as i32)) != 0) {
            if (((((((((F::BtlAnim_GetId(c, p_chr_1)) != (54i32)) as i32)) != 0) && (((((F::BtlAnim_GetNextId(c, p_chr_1)) != (54i32)) as i32)) != 0)) as i32)) != 0) {
                if ((F::BtlInput_TestAction(c, p_chr_1, 19i32, 0i32)) != 0) {
                    F::BtlAct_Request(c, p_chr_1, 11i32);
                }
                { let _ = F::BtlDecide_Main(c, p_chr_1, 13638656i32); };
                { let _ = F::BtlDecide_Common(c, p_chr_1, 6i32); };
                { let a1_ = F::BtlAct_GetQueued(c, p_chr_1); F::BtlAct_Request(c, p_chr_1, a1_) };
            }
        }
        if (((((p_phase_2) == (3i32)) as i32)) != 0) {
            F::BtlChar_PlayVoiceSingle(c, p_chr_1, 13i32);
        }
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_ChargeHandler", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_GuardHandler(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_GuardHandler", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let mut l_work_3: u32 = 0;
    let mut l_blend_4: F32 = F32(0);
    let mut l_accel_5: F32 = F32(0);
    let mut l_anim_6: i32 = 0;
    let mut l_want_7: i32 = 0;
    let r_ = 'f: {
    l_work_3 = ((p_chr_1).wrapping_add(976)).wrapping_add((((2i32) as u32)).wrapping_mul(4));
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        l_blend_4 = F32(0x00000000);
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((104i32) as i64 as u64)) as u32))) != 0) {
            l_anim_6 = 242i32;
        } else {
            if ((F::BtlChar_TestFlag(c, p_chr_1, ((((105i32) as i64 as u64)) as u32))) != 0) {
                l_anim_6 = 244i32;
            } else {
                if ((F::BtlChar_TestFlag(c, p_chr_1, ((((106i32) as i64 as u64)) as u32))) != 0) {
                    l_anim_6 = 240i32;
                } else {
                    l_blend_4 = F32(0x3e199999);
                    if ((F::BtlInput_TestAction(c, p_chr_1, 36i32, 1i32)) != 0) {
                        l_anim_6 = 241i32;
                    } else {
                        l_anim_6 = (if ((F::BtlInput_TestAction(c, p_chr_1, 37i32, 1i32)) != 0) { 243i32 } else { 239i32 });
                    }
                }
            }
        }
        F::BtlAnim_Play(c, p_chr_1, l_anim_6, l_blend_4);
        F::BtlMove_TurnYaw(c, p_chr_1, 2i32, F32(0x40490fda));
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        's1: {
            let sv8: i32 = F::BtlAnim_GetId(c, p_chr_1);
            match sv8 {
                239i32 | 241i32 | 243i32 => {
                    F::BtlAnim_AdvanceLoop(c, p_chr_1, 0i32);
                    break 's1;
                }
                240i32 | 242i32 | 244i32 => {
                    { let _ = { let a1_ = (F::BtlAnim_GetId(c, p_chr_1)).wrapping_sub(1i32); F::BtlAnim_AdvanceThen(c, p_chr_1, a1_, F32(0x00000000), 0i32) }; };
                    F::BtlChar_SetFlag(c, p_chr_1, ((((149i32) as i64 as u64)) as u32));
                    if ((p_phase_2) != 0) {
                        F::BtlChar_SetFlag(c, p_chr_1, ((((65i32) as i64 as u64)) as u32));
                    } else {
                        F::BtlChar_SetFlag(c, p_chr_1, ((((65i32) as i64 as u64)) as u32));
                    }
                    break 's1;
                }
                245i32 => {
                    { let _ = F::BtlAnim_AdvanceThen(c, p_chr_1, 239i32, F32(0x00000000), 0i32); };
                    F::BtlChar_SetFlag(c, p_chr_1, ((((149i32) as i64 as u64)) as u32));
                    F::BtlChar_SetFlag(c, p_chr_1, ((((65i32) as i64 as u64)) as u32));
                    if ((F::BtlAnim_IsNew(c, p_chr_1)) != 0) {
                        F::BtlChar_PlayVoice(c, p_chr_1, 27i32);
                    }
                    break 's1;
                }
                _ => {
                }
            }
        }
        l_accel_5 = F32(0x3f6d0979);
        F::BtlMove_Step(c, p_chr_1, 6i32, 5i32, 2i32, F32(0x00000000), l_accel_5);
        if ((((((((((((!((F::BtlParam_CanFly(c, p_chr_1)) != 0)) as i32)) != 0) && ((F::BtlChar_TestFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) && ((((!((F::BtlChar_TestFlag(c, p_chr_1, ((((17i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0)) as i32)) != 0) {
            F::BtlMove_MoveVertical(c, p_chr_1, l_accel_5, l_accel_5);
        } else {
            F::BtlMove_ApplyGravity(c, p_chr_1);
        }
        F::BtlChar_SetFlag(c, p_chr_1, ((((201i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((213i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((28i32) as i64 as u64)) as u32));
        F::BtlChar_SetLookEnabled(c, p_chr_1, 1i32);
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((15i32) as i64 as u64)) as u32))) != 0) {
            if (((rt::flt(F32(0x3fed0979), { let a0_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(64); F::Vec3_Length(c, a0_) }) as i32)) != 0) {
                F::BtlChar_SetFxBit(c, p_chr_1, 50i32);
            }
        }
        if ((((((F::BtlChar_TestFlag(c, p_chr_1, ((((17i32) as i64 as u64)) as u32))) != 0) && ((((!((F::BtlChar_TestFlag(c, p_chr_1, ((((15i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0)) as i32)) != 0) {
            F::BtlChar_SetHeldFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32));
        }
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((15i32) as i64 as u64)) as u32))) != 0) {
            F::BtlChar_ClearFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32));
        }
    }
    if (((((p_phase_2) == (2i32)) as i32)) != 0) {
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((102i32) as i64 as u64)) as u32))) != 0) {
            's2: {
                let sv9: i32 = F::BtlAnim_GetId(c, p_chr_1);
                match sv9 {
                    239i32 | 241i32 | 243i32 => {
                        { let a1_ = (F::BtlAnim_GetId(c, p_chr_1)).wrapping_add(1i32); F::BtlAnim_Request(c, p_chr_1, a1_, F32(0x00000000)) };
                        break 's2;
                    }
                    240i32 | 242i32 | 244i32 => {
                        { let a1_ = F::BtlAnim_GetId(c, p_chr_1); F::BtlAnim_Request(c, p_chr_1, a1_, F32(0x00000000)) };
                        break 's2;
                    }
                    245i32 => {
                        F::BtlAnim_Request(c, p_chr_1, 240i32, F32(0x00000000));
                        break 's2;
                    }
                    _ => {
                    }
                }
            }
        } else {
            if ((F::BtlChar_TestFlag(c, p_chr_1, ((((103i32) as i64 as u64)) as u32))) != 0) {
                F::BtlAnim_Request(c, p_chr_1, 245i32, F32(0x00000000));
            } else {
                's3: {
                    let sv10: i32 = F::BtlAnim_GetId(c, p_chr_1);
                    match sv10 {
                        239i32 | 241i32 | 243i32 => {
                            if ((F::BtlInput_TestAction(c, p_chr_1, 34i32, 0i32)) != 0) {
                                F::BtlAct_Request(c, p_chr_1, 11i32);
                            }
                            l_want_7 = 239i32;
                            if ((F::BtlInput_TestAction(c, p_chr_1, 36i32, 1i32)) != 0) {
                                l_want_7 = 241i32;
                            }
                            if ((F::BtlInput_TestAction(c, p_chr_1, 37i32, 1i32)) != 0) {
                                l_want_7 = 243i32;
                            }
                            if (((((F::BtlAnim_GetId(c, p_chr_1)) == (l_want_7)) as i32)) != 0) {
                                { let v_ = 0i32; let a_ = l_work_3; c.m.w32(a_, (v_) as u32); };
                            } else {
                                { let a_ = l_work_3; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                            }
                            if ((((((c.m.r32(l_work_3) as i32)) >= (3i32)) as i32)) != 0) {
                                F::BtlAnim_Request(c, p_chr_1, l_want_7, F32(0x3e199999));
                                { let v_ = 0i32; let a_ = l_work_3; c.m.w32(a_, (v_) as u32); };
                            }
                            break 's3;
                        }
                        240i32 | 242i32 | 244i32 | 245i32 => {
                            break 's3;
                        }
                        _ => {
                        }
                    }
                }
                { let _ = F::BtlDecide_Main(c, p_chr_1, 16777216i32); };
                { let _ = F::BtlDecide_Attack(c, p_chr_1, 1024i32); };
                { let _ = F::BtlDecide_Common(c, p_chr_1, 6i32); };
                { let a1_ = F::BtlAct_GetQueued(c, p_chr_1); F::BtlAct_Request(c, p_chr_1, a1_) };
            }
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_GuardHandler", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_Action39(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_Action39", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let r_ = 'f: {
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        F::BtlAnim_Play(c, p_chr_1, 236i32, F32(0x00000000));
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        if ((F::BtlAnim_Advance(c, p_chr_1, 0i32)) != 0) {
            { let a1_ = F::BtlAct_GetEvasionAttack(c, p_chr_1); { let _ = F::BtlDecide_QueueAttack(c, p_chr_1, a1_); } };
            { let a1_ = F::BtlAct_GetQueued(c, p_chr_1); F::BtlAct_Request(c, p_chr_1, a1_) };
        }
        F::BtlMove_Step(c, p_chr_1, 6i32, 5i32, 2i32, F32(0x00000000), F32(0x3f6d0979));
        F::BtlMove_ApplyGravity(c, p_chr_1);
        F::BtlChar_SetFlag(c, p_chr_1, ((((201i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((213i32) as i64 as u64)) as u32));
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((15i32) as i64 as u64)) as u32))) != 0) {
            if (((rt::flt(F32(0x3fed0979), { let a0_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(64); F::Vec3_Length(c, a0_) }) as i32)) != 0) {
                F::BtlChar_SetFxBit(c, p_chr_1, 50i32);
            }
        }
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((17i32) as i64 as u64)) as u32))) != 0) {
            F::BtlChar_SetHeldFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32));
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_Action39", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_Action3A(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_Action3A", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let r_ = 'f: {
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        F::BtlAnim_Play(c, p_chr_1, 238i32, F32(0x3e199999));
        if ((((((F::BtlChar_TestFlag(c, p_chr_1, ((((5i32) as i64 as u64)) as u32))) != 0) && ((F::BtlChar_TestFlag(c, p_chr_1, ((((19i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) {
            F::BtlChar_PlayVoice(c, p_chr_1, 30i32);
        }
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        if ((F::BtlAnim_Advance(c, p_chr_1, 0i32)) != 0) {
            F::BtlAct_Request(c, p_chr_1, 11i32);
            { let a1_ = F::BtlAct_GetQueued(c, p_chr_1); F::BtlAct_Request(c, p_chr_1, a1_) };
        }
        F::BtlMove_Step(c, p_chr_1, 3i32, 5i32, 2i32, F32(0x00000000), F32(0x3f6d0979));
        F::BtlMove_ApplyGravity(c, p_chr_1);
        F::BtlChar_SetFlag(c, p_chr_1, ((((213i32) as i64 as u64)) as u32));
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((17i32) as i64 as u64)) as u32))) != 0) {
            F::BtlChar_SetHeldFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32));
        }
    }
    if (((((p_phase_2) == (3i32)) as i32)) != 0) {
        break 'f { F::BtlChar_SetFlag(c, p_chr_1, ((((51i32) as i64 as u64)) as u32)); 0 };
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_Action3A", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_Action3B(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_Action3B", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let mut l_work_3: u32 = 0;
    let r_ = 'f: {
    l_work_3 = ((p_chr_1).wrapping_add(976)).wrapping_add((((2i32) as u32)).wrapping_mul(4));
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        F::BtlAnim_Play(c, p_chr_1, 249i32, F32(0x3e199999));
        { let v_ = 4i32; let a_ = l_work_3; c.m.w32(a_, (v_) as u32); };
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        { let _ = F::BtlAnim_Advance(c, p_chr_1, 0i32); };
        F::BtlMove_Step(c, p_chr_1, 6i32, 6i32, 7i32, F32(0x00000000), F32(0x3f6d0979));
        F::BtlMove_ApplyGravity(c, p_chr_1);
        F::BtlChar_SetFlag(c, p_chr_1, ((((28i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((80i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((81i32) as i64 as u64)) as u32));
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((17i32) as i64 as u64)) as u32))) != 0) {
            F::BtlChar_SetHeldFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32));
        }
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((15i32) as i64 as u64)) as u32))) != 0) {
            F::BtlChar_ClearFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32));
        }
    }
    if (((((p_phase_2) == (2i32)) as i32)) != 0) {
        if ((F::BtlInput_TestAction(c, p_chr_1, 43i32, 1i32)) != 0) {
            { let v_ = 4i32; let a_ = l_work_3; c.m.w32(a_, (v_) as u32); };
        }
        if ((F::BtlInput_TestAction(c, p_chr_1, 34i32, 0i32)) != 0) {
            { let v_ = 0i32; let a_ = l_work_3; c.m.w32(a_, (v_) as u32); };
        }
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((49i32) as i64 as u64)) as u32))) != 0) {
            if ((((({ let a_ = l_work_3; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) <= (0i32)) as i32)) != 0) {
                F::BtlAct_Request(c, p_chr_1, 11i32);
            } else {
                { let a1_ = (if (((((F::BtlAnim_GetId(c, p_chr_1)) == (249i32)) as i32)) != 0) { 250i32 } else { 249i32 }); F::BtlAnim_Request(c, p_chr_1, a1_, F32(0x00000000)) };
            }
        }
        { let _ = F::BtlDecide_Common(c, p_chr_1, 6i32); };
        { let a1_ = F::BtlAct_GetQueued(c, p_chr_1); F::BtlAct_Request(c, p_chr_1, a1_) };
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_Action3B", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_Action3C(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_Action3C", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let r_ = 'f: {
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        F::BtlAnim_Play(c, p_chr_1, 253i32, F32(0x3e199999));
        F::BtlChar_PlayVoice(c, p_chr_1, 5i32);
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        if ((F::BtlAnim_Advance(c, p_chr_1, 0i32)) != 0) {
            F::BtlAct_Request(c, p_chr_1, 11i32);
        }
        F::BtlMove_Step(c, p_chr_1, 6i32, 6i32, 7i32, F32(0x00000000), F32(0x3f6d0979));
        F::BtlMove_ApplyGravity(c, p_chr_1);
        F::BtlChar_SetFlag(c, p_chr_1, ((((28i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((82i32) as i64 as u64)) as u32));
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((17i32) as i64 as u64)) as u32))) != 0) {
            F::BtlChar_SetHeldFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32));
        }
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((15i32) as i64 as u64)) as u32))) != 0) {
            F::BtlChar_ClearFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32));
        }
    }
    if (((((p_phase_2) == (2i32)) as i32)) != 0) {
        { let _ = F::BtlDecide_Main(c, p_chr_1, 7274i32); };
        { let a1_ = F::BtlAct_GetQueued(c, p_chr_1); F::BtlAct_Request(c, p_chr_1, a1_) };
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_Action3C", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_Action3D(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_Action3D", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let r_ = 'f: {
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        F::BtlAnim_Play(c, p_chr_1, 249i32, F32(0x3e199999));
        F::BtlCharSnd_PlayCommon(c, p_chr_1, 31i32);
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        's1: {
            let sv3: i32 = F::BtlAnim_GetId(c, p_chr_1);
            match sv3 {
                249i32 => {
                    { let _ = F::BtlAnim_AdvanceThen(c, p_chr_1, 250i32, F32(0x00000000), 0i32); };
                    break 's1;
                }
                250i32 => {
                    F::BtlAnim_AdvanceLoop(c, p_chr_1, 0i32);
                    break 's1;
                }
                _ => {
                }
            }
        }
        F::BtlMove_Step(c, p_chr_1, 6i32, 6i32, 7i32, F32(0x00000000), F32(0x3f6d0979));
        F::BtlMove_ApplyGravity(c, p_chr_1);
        F::BtlChar_SetFlag(c, p_chr_1, ((((28i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((84i32) as i64 as u64)) as u32));
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((17i32) as i64 as u64)) as u32))) != 0) {
            F::BtlChar_SetHeldFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32));
        }
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((15i32) as i64 as u64)) as u32))) != 0) {
            F::BtlChar_ClearFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32));
        }
    }
    if (((((p_phase_2) == (2i32)) as i32)) != 0) {
        if ((F::BtlInput_TestAction(c, p_chr_1, 34i32, 0i32)) != 0) {
            F::BtlAct_Request(c, p_chr_1, 11i32);
        }
        { let _ = F::BtlDecide_Common(c, p_chr_1, 6i32); };
        { let a1_ = F::BtlAct_GetQueued(c, p_chr_1); F::BtlAct_Request(c, p_chr_1, a1_) };
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_Action3D", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_Action3E(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_Action3E", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let mut l_speed_3: F32 = F32(0);
    let mut l_c_4: F32 = F32(0);
    let r_ = 'f: {
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        F::BtlAnim_Play(c, p_chr_1, 251i32, F32(0x3e199999));
        F::BtlChar_SetHeldFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32));
        F::BtlChar_PlayVoice(c, p_chr_1, 5i32);
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        if ((F::BtlAnim_Advance(c, p_chr_1, 0i32)) != 0) {
            F::BtlAct_Request(c, p_chr_1, 15i32);
        }
        l_speed_3 = { let a1_ = (F::BtlChar_TestFlag(c, p_chr_1, ((((17i32) as i64 as u64)) as u32))).wrapping_add(4i32); F::BtlMoveParam_GetSpeed(c, p_chr_1, a1_) };
        { let a2_ = F::BtlMoveParam_GetTurnRate(c, p_chr_1, 0i32); F::BtlMove_TurnYaw(c, p_chr_1, 0i32, a2_) };
        F::BtlMove_TurnPitch(c, p_chr_1, 2i32, F32(0x3e20d97a));
        F::BtlMove_TurnModelYaw(c, p_chr_1, F32(0x3f20d97a), F32(0x3e999999));
        F::BtlMove_SetDirection(c, p_chr_1, 3i32);
        F::BtlMove_Advance(c, p_chr_1, l_speed_3, F32(0x461c4000));
        F::BtlMove_ApplyGravity(c, p_chr_1);
        if ((((((F::BtlChar_TestFlag(c, p_chr_1, ((((5i32) as i64 as u64)) as u32))) != 0) && ((((!((F::BtlInput_IsHeld(c, p_chr_1, ((240i32) as u32))) != 0)) as i32)) != 0)) as i32)) != 0) {
            F::BtlChar_SetFlag(c, p_chr_1, ((((26i32) as i64 as u64)) as u32));
        }
        l_c_4 = { let a0_ = F::BtlOpp_GetYawFromFacing(c, p_chr_1); F::Mathf_Cos(c, a0_) };
        if (((rt::flt(F32(0x00000000), l_c_4) as i32)) != 0) {
            { let a1_ = rt::fmul({ let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(144); F32(c.m.r32(a_)) }, l_c_4); F::BtlMove_SetLeanX(c, p_chr_1, a1_) };
        }
        F::BtlChar_SetFlag(c, p_chr_1, ((((202i32) as i64 as u64)) as u32));
        F::BtlMove_RequestOrbit(c, p_chr_1, F32(0x42480000), F32(0x42c80000));
        F::BtlChar_SetFlag(c, p_chr_1, ((((80i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((81i32) as i64 as u64)) as u32));
        F::BtlChar_SetFxBit(c, p_chr_1, 11i32);
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((17i32) as i64 as u64)) as u32))) != 0) {
            F::BtlChar_SetHeldFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32));
        }
    }
    if (((((p_phase_2) == (2i32)) as i32)) != 0) {
        if (((((((rt::flt(F32(0x3ecccccc), F::BtlAnim_GetProgress(c, p_chr_1)) as i32)) != 0) && ((F::BtlInput_TestAction(c, p_chr_1, 44i32, 1i32)) != 0)) as i32)) != 0) {
            F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), 62i32);
        }
        if ((F::BtlAnim_PassedRatio(c, p_chr_1, F32(0x3f333333))) != 0) {
            { let a1_ = F::BtlAct_GetQueued(c, p_chr_1); F::BtlAct_Request(c, p_chr_1, a1_) };
        }
        if ((F::BtlInput_TestAction(c, p_chr_1, 64i32, 1i32)) != 0) {
            F::BtlAct_Request(c, p_chr_1, 88i32);
        }
        if ((F::BtlDecide_Attack(c, p_chr_1, 49152i32)) != 0) {
            { let a1_ = F::BtlAct_GetQueued(c, p_chr_1); F::BtlAct_Request(c, p_chr_1, a1_) };
        }
        if ((F::BtlDecide_Common(c, p_chr_1, 6i32)) != 0) {
            { let a1_ = F::BtlAct_GetQueued(c, p_chr_1); F::BtlAct_Request(c, p_chr_1, a1_) };
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_Action3E", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_Action3F(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_Action3F", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let r_ = 'f: {
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        F::BtlAnim_Play(c, p_chr_1, 252i32, F32(0x3e199999));
        F::BtlChar_ClearFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32));
        F::BtlChar_PlayVoice(c, p_chr_1, 5i32);
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        if ((F::BtlAnim_Advance(c, p_chr_1, 0i32)) != 0) {
            F::BtlAct_Request(c, p_chr_1, 17i32);
        }
        F::BtlMove_Step(c, p_chr_1, 2i32, 5i32, 7i32, F32(0x00000000), F32(0x3f6d0979));
        F::BtlMove_ApplyGravity(c, p_chr_1);
        F::BtlChar_SetFlag(c, p_chr_1, ((((213i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((80i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((81i32) as i64 as u64)) as u32));
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((17i32) as i64 as u64)) as u32))) != 0) {
            F::BtlChar_SetHeldFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32));
        }
    }
    if (((((p_phase_2) == (2i32)) as i32)) != 0) {
        if (((((((rt::flt(F32(0x3ecccccc), F::BtlAnim_GetProgress(c, p_chr_1)) as i32)) != 0) && ((F::BtlInput_TestAction(c, p_chr_1, 44i32, 1i32)) != 0)) as i32)) != 0) {
            F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), 63i32);
        }
        if ((F::BtlAnim_PassedRatio(c, p_chr_1, F32(0x3f333333))) != 0) {
            { let a1_ = F::BtlAct_GetQueued(c, p_chr_1); F::BtlAct_Request(c, p_chr_1, a1_) };
        }
        if ((F::BtlInput_TestAction(c, p_chr_1, 64i32, 1i32)) != 0) {
            F::BtlAct_Request(c, p_chr_1, 89i32);
        }
        if ((F::BtlDecide_Attack(c, p_chr_1, 393216i32)) != 0) {
            { let a1_ = F::BtlAct_GetQueued(c, p_chr_1); F::BtlAct_Request(c, p_chr_1, a1_) };
        }
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((15i32) as i64 as u64)) as u32))) != 0) {
            F::BtlAct_Request(c, p_chr_1, 11i32);
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_Action3F", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_Action40(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_Action40", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let r_ = 'f: {
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        F::BtlAnim_Play(c, p_chr_1, 222i32, F32(0x3e199999));
        F::BtlCharSnd_PlayCommon(c, p_chr_1, 31i32);
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        if ((F::BtlAnim_Advance(c, p_chr_1, 0i32)) != 0) {
            F::BtlAct_Request(c, p_chr_1, 11i32);
        }
        F::BtlMove_Step(c, p_chr_1, 6i32, 5i32, 3i32, F32(0x00000000), F32(0x3f6d0979));
        F::BtlMove_ApplyGravity(c, p_chr_1);
        F::BtlChar_SetFlag(c, p_chr_1, ((((213i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((66i32) as i64 as u64)) as u32));
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((17i32) as i64 as u64)) as u32))) != 0) {
            F::BtlChar_SetHeldFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32));
        }
    }
    if (((((p_phase_2) == (2i32)) as i32)) != 0) {
        if (((((((rt::flt(F32(0x3f199999), F::BtlAnim_GetProgress(c, p_chr_1)) as i32)) != 0) && ((F::BtlAct_HasQueued(c, p_chr_1)) != 0)) as i32)) != 0) {
            { let a1_ = F::BtlAct_GetQueued(c, p_chr_1); F::BtlAct_Request(c, p_chr_1, a1_) };
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_Action40", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_Action41(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_Action41", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let mut l_idle_3: i32 = 0;
    let r_ = 'f: {
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        if (((((F::BtlAct_GetPrev(c, p_chr_1)) != (65i32)) as i32)) != 0) {
            { let v_ = 0i32; let a_ = (p_chr_1).wrapping_add(3468); c.m.w32(a_, (v_) as u32); };
        }
        { let a1_ = ((c.m.r32((p_chr_1).wrapping_add(3468)) as i32)).wrapping_add(403i32); F::BtlAnim_Play(c, p_chr_1, a1_, F32(0x3e199999)) };
        { let a_ = (p_chr_1).wrapping_add(3468); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        { let r_ = 3i32; let a_ = (p_chr_1).wrapping_add(3468); let n_ = rt::rem_i32((c.m.r32(a_) as i32), r_); c.m.w32(a_, (n_) as u32); };
        F::BtlCharSnd_PlayCommon(c, p_chr_1, 31i32);
        { let a2_ = (if (((rt::flt(F32(c.m.r32((p_chr_1).wrapping_add(1192))), F32(0x00000000)) as i32)) != 0) { 17i32 } else { 18i32 }); F::ChrCam_RequestCut(c, p_chr_1, 1i32, a2_) };
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        l_idle_3 = (if ((F::BtlChar_TestFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32))) != 0) { 38i32 } else { 0i32 });
        's1: {
            let sv4: i32 = F::BtlAnim_GetId(c, p_chr_1);
            match sv4 {
                403i32 | 404i32 | 405i32 => {
                    { let _ = F::BtlAnim_AdvanceThen(c, p_chr_1, l_idle_3, F32(0x3e199999), 0i32); };
                    break 's1;
                }
                0i32 | 38i32 => {
                    F::BtlAnim_AdvanceLoop(c, p_chr_1, 0i32);
                    break 's1;
                }
                _ => {
                }
            }
        }
        F::BtlMove_Step(c, p_chr_1, 2i32, 5i32, 3i32, F32(0x00000000), F32(0x3f6d0979));
        F::BtlMove_ApplyGravity(c, p_chr_1);
        F::BtlChar_SetFlag(c, p_chr_1, ((((213i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((78i32) as i64 as u64)) as u32));
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((17i32) as i64 as u64)) as u32))) != 0) {
            F::BtlChar_SetHeldFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32));
        }
    }
    if (((((p_phase_2) == (2i32)) as i32)) != 0) {
        's2: {
            let sv5: i32 = F::BtlAnim_GetId(c, p_chr_1);
            match sv5 {
                0i32 | 38i32 => {
                    if ((((((F::BtlChar_TestFlag(c, p_chr_1, ((((181i32) as i64 as u64)) as u32))) != 0) || (((((F::BtlOpp_GetSeenAction(c, p_chr_1)) != (69i32)) as i32)) != 0)) as i32)) != 0) {
                        F::BtlAct_Request(c, p_chr_1, 106i32);
                    }
                    break 's2;
                }
                403i32 | 404i32 | 405i32 => {
                    break 's2;
                }
                _ => {
                }
            }
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_Action41", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_Action42(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_Action42", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let r_ = 'f: {
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        F::BtlAnim_Play(c, p_chr_1, 402i32, F32(0x3e199999));
        F::BtlChar_SetHeldFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32));
        { let v_ = F32(0x00000000); let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(152); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(156); c.m.w32(a_, (v_).0); };
        { let _ = F::BtlMember_Damage(c, p_chr_1, 5000i32, 1067i32); };
        F::BtlChar_SetHeldFlag(c, p_chr_1, ((((75i32) as i64 as u64)) as u32));
        F::BtlCharSnd_PlayCommon(c, p_chr_1, 36i32);
        F::BtlChar_PlayVoice(c, p_chr_1, 6i32);
        F::BtlChar_SetFxBit(c, p_chr_1, 43i32);
        F::BtlChar_Vibrate(c, p_chr_1, F32(0x3f4ccccc), F32(0x3ecccccc));
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        F::BtlAnim_SetDuration(c, p_chr_1, F32(0x3f555555));
        if ((F::BtlAnim_Advance(c, p_chr_1, 0i32)) != 0) {
            F::BtlAct_Request(c, p_chr_1, 11i32);
        }
        if ((F::BtlAnim_PassedRatio(c, p_chr_1, F32(0x3f000000))) != 0) {
            F::BtlChar_ClearFlag(c, p_chr_1, ((((75i32) as i64 as u64)) as u32));
        }
        F::BtlMove_Step(c, p_chr_1, 6i32, 6i32, 7i32, F32(0x00000000), F32(0x3f6d0979));
        F::BtlMove_ApplyGravity(c, p_chr_1);
        F::BtlChar_SetFlag(c, p_chr_1, ((((201i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((213i32) as i64 as u64)) as u32));
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((17i32) as i64 as u64)) as u32))) != 0) {
            F::BtlChar_SetHeldFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32));
        }
    }
    if (((((p_phase_2) == (3i32)) as i32)) != 0) {
        break 'f { F::BtlChar_ClearFlag(c, p_chr_1, ((((75i32) as i64 as u64)) as u32)); 0 };
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_Action42", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_Action43(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_Action43", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let r_ = 'f: {
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        F::BtlAnim_Play(c, p_chr_1, 389i32, F32(0x3e199999));
        F::BtlChar_PlayVoice(c, p_chr_1, 35i32);
        { let a_ = (F::BtlMember_GetActive(c, p_chr_1)).wrapping_add(160); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        if ((F::BtlAnim_Advance(c, p_chr_1, 0i32)) != 0) {
            F::BtlAct_Request(c, p_chr_1, 11i32);
        }
        F::BtlMove_Step(c, p_chr_1, 6i32, 5i32, 2i32, F32(0x00000000), F32(0x3f6d0979));
        F::BtlMove_ApplyGravity(c, p_chr_1);
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((17i32) as i64 as u64)) as u32))) != 0) {
            F::BtlChar_SetHeldFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32));
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_Action43", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_PlaySubAnim(c: &mut Ctx, mut p_chr_1: u32, mut p_anim_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_PlaySubAnim", &PROBE_N, &[(p_chr_1) as u32, (p_anim_2) as u32], false); }
    let mut l_obj_3: u32 = 0;
    let r_ = 'f: {
    l_obj_3 = F::BtlChar_GetObj(c, p_chr_1);
    F::BtlAnim_PlaySub(c, p_chr_1, p_anim_2);
    break 'f { let a0_ = F::BtlObjAnim_QueryEvent(c, l_obj_3, ((4i32) as u64), 1i32, ((((6i32) as i64 as u64)) as u32)); F::BtlObjAnim_MaskToNode(c, ((((a0_) as i64 as u64)) as u32)) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_PlaySubAnim", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_KiBlastHandler(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_KiBlastHandler", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_idle_3: i32 = 0;
    let mut l_zero_4: F32 = F32(0);
    let r_ = 'f: {
        if (((((p_phase_2) == (0i32)) as i32)) != 0) {
            if (((((F::BtlAct_GetPrev(c, p_chr_1)) != (174i32)) as i32)) != 0) {
                { let v_ = 0i32; let a_ = (p_chr_1).wrapping_add(3560); c.m.w32(a_, (v_) as u32); };
            }
            if ((((((c.m.r32((p_chr_1).wrapping_add(3560)) as i32)) == (0i32)) as i32)) != 0) {
                F::BtlAnim_Play(c, p_chr_1, 113i32, F32(0x3dcccccc));
                { let v_ = 1i32; let a_ = (p_chr_1).wrapping_add(3560); c.m.w32(a_, (v_) as u32); };
            } else {
                F::BtlAnim_Play(c, p_chr_1, 114i32, F32(0x3dcccccc));
                if ((((((((!((F::BtlChar_TestMemberUnk70(c, p_chr_1)) != 0)) as i32)) != 0) && ((((!((((F::BtlParam_GetFlags(c, p_chr_1)) & (((256i32) as u32)))) != 0)) as i32)) != 0)) as i32)) != 0) {
                    { let v_ = 0i32; let a_ = (p_chr_1).wrapping_add(3560); c.m.w32(a_, (v_) as u32); };
                }
            }
            { let _ = { let a1_ = F::BtlKiBlast_GetKiCost(c, p_chr_1); F::BtlMember_SpendKi(c, p_chr_1, a1_, 0i32) }; };
            F::BtlChar_PlayVoice(c, p_chr_1, 12i32);
            { let r_ = F::BtlKiBlast_GetHitsOf(c, p_chr_1, ((((0i32) as i64 as u64)) as u32)); let a_ = (p_chr_1).wrapping_add(3552); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
            { let v_ = 30i32; let a_ = (p_chr_1).wrapping_add(3556); c.m.w32(a_, (v_) as u32); };
        }
        if (((((p_phase_2) == (1i32)) as i32)) != 0) {
            l_idle_3 = (if ((F::BtlChar_TestFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32))) != 0) { 38i32 } else { 0i32 });
            's1: {
                let sv5: i32 = F::BtlAnim_GetId(c, p_chr_1);
                match sv5 {
                    113i32 | 114i32 => {
                        { let _ = F::BtlAnim_AdvanceThen(c, p_chr_1, l_idle_3, F32(0x3e4ccccc), 0i32); };
                        { let a1_ = (F::BtlAnim_GetId(c, p_chr_1)).wrapping_add(2i32); let a2_ = (F::BtlAnim_GetId(c, p_chr_1)).wrapping_add(4i32); F::BtlAct_SetPitchMotion(c, p_chr_1, a1_, a2_, 1i32) };
                        break 's1;
                    }
                    0i32 | 38i32 => {
                        { let _ = F::BtlAnim_Advance(c, p_chr_1, 1i32); };
                        if ((((!((F::BtlChar_TestFlag(c, p_chr_1, ((((52i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) {
                            F::BtlAct_Request(c, p_chr_1, 11i32);
                        }
                        F::BtlChar_SetFlag(c, p_chr_1, ((((135i32) as i64 as u64)) as u32));
                        break 's1;
                    }
                    _ => {
                    }
                }
            }
            l_zero_4 = F32(0x00000000);
            F::BtlMove_Step(c, p_chr_1, 2i32, 5i32, 7i32, l_zero_4, F32(0x3f6d0979));
            F::BtlMove_ApplyGravity(c, p_chr_1);
            F::BtlChar_SetFlag(c, p_chr_1, ((((213i32) as i64 as u64)) as u32));
            if ((F::BtlChar_TestFlag(c, p_chr_1, ((((17i32) as i64 as u64)) as u32))) != 0) {
                F::BtlChar_SetHeldFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32));
            }
            if ((F::BtlChar_TestFlag(c, p_chr_1, ((((5i32) as i64 as u64)) as u32))) != 0) {
                { let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); F::BtlCharApi_GetNodePos(c, a0_, 17i32, fp) };
                { let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); F::BtlCharApi_CalcAimDir(c, a0_, 17i32, fp, (p_chr_1).wrapping_add(3536), F32(0xbf20d97a), F32(0x3f20d97a)) };
            } else {
                { let v_ = { let a0_ = { let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(148); F32(c.m.r32(a_)) }; F::Mathf_Sin(c, a0_) }; let a_ = (p_chr_1).wrapping_add(3536); c.m.w32(a_, (v_).0); };
                { let v_ = l_zero_4; let a_ = ((p_chr_1).wrapping_add(3536)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                { let v_ = { let a0_ = { let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(148); F32(c.m.r32(a_)) }; F::Mathf_Cos(c, a0_) }; let a_ = ((p_chr_1).wrapping_add(3536)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                { let v_ = l_zero_4; let a_ = ((p_chr_1).wrapping_add(3536)).wrapping_add(12); c.m.w32(a_, (v_).0); };
            }
        }
        if (((((p_phase_2) == (2i32)) as i32)) != 0) {
            { let _ = F::BtlDecide_Attack(c, p_chr_1, 12i32); };
            's2: {
                let sv6: i32 = F::BtlAnim_GetId(c, p_chr_1);
                match sv6 {
                    113i32 | 114i32 => {
                        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((49i32) as i64 as u64)) as u32))) != 0) {
                            { let a1_ = F::BtlAct_GetQueued(c, p_chr_1); F::BtlAct_Request(c, p_chr_1, a1_) };
                        }
                        break 's2;
                    }
                    0i32 | 38i32 => {
                        { let _ = F::BtlDecide_Main(c, p_chr_1, 531690i32); };
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
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_KiBlastHandler", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_ChargedKiBlastHandler(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_ChargedKiBlastHandler", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_frame_3: F32 = F32(0);
    let mut l_zero_4: F32 = F32(0);
    let r_ = 'f: {
        if (((((p_phase_2) == (0i32)) as i32)) != 0) {
            F::BtlAnim_Play(c, p_chr_1, 123i32, F32(0x3e199999));
            { let v_ = F::BtlAct_PlaySubAnim(c, p_chr_1, 125i32); let a_ = (p_chr_1).wrapping_add(3568); c.m.w32(a_, (v_) as u32); };
            F::BtlChar_SetFxBit(c, p_chr_1, 23i32);
            F::BtlCharSnd_PlayCommon(c, p_chr_1, 46i32);
            { let a_ = (p_chr_1).wrapping_add(3552); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
            { let v_ = 30i32; let a_ = (p_chr_1).wrapping_add(3556); c.m.w32(a_, (v_) as u32); };
        }
        if (((((p_phase_2) == (1i32)) as i32)) != 0) {
            's1: {
                let sv5: i32 = F::BtlAnim_GetId(c, p_chr_1);
                match sv5 {
                    123i32 => {
                        { let _ = F::BtlAnim_AdvanceThen(c, p_chr_1, 124i32, F32(0x00000000), 0i32); };
                        F::BtlChar_SetFlag(c, p_chr_1, ((((142i32) as i64 as u64)) as u32));
                        break 's1;
                    }
                    124i32 => {
                        { let _ = F::BtlAnim_AdvanceThen(c, p_chr_1, 125i32, F32(0x00000000), 0i32); };
                        F::BtlChar_SetFlag(c, p_chr_1, ((((142i32) as i64 as u64)) as u32));
                        { let v_ = F::BtlAnim_GetProgress(c, p_chr_1); let a_ = (p_chr_1).wrapping_add(3564); c.m.w32(a_, (v_).0); };
                        break 's1;
                    }
                    125i32 => {
                        if ((F::BtlAnim_Advance(c, p_chr_1, 0i32)) != 0) {
                            F::BtlAct_Request(c, p_chr_1, 11i32);
                        }
                        l_frame_3 = F::BtlAnim_GetFrame(c, p_chr_1);
                        if (((rt::flt(l_frame_3, rt::i2f({ let a0_ = F::BtlChar_GetObj(c, p_chr_1); F::BtlObjAnim_QueryEvent(c, a0_, ((4i32) as u64), 0i32, ((((0i32) as i64 as u64)) as u32)) })) as i32)) != 0) {
                            F::BtlChar_SetFlag(c, p_chr_1, ((((142i32) as i64 as u64)) as u32));
                        }
                        if ((F::BtlAnim_IsNew(c, p_chr_1)) != 0) {
                            { let _ = { let a1_ = F::BtlKiBlast_GetKiCost(c, p_chr_1); F::BtlMember_SpendKi(c, p_chr_1, a1_, 0i32) }; };
                            F::BtlChar_PlayVoice(c, p_chr_1, 37i32);
                        }
                        break 's1;
                    }
                    _ => {
                    }
                }
            }
            l_zero_4 = F32(0x00000000);
            F::BtlMove_Step(c, p_chr_1, 2i32, 5i32, 7i32, l_zero_4, F32(0x3f6d0979));
            F::BtlMove_ApplyGravity(c, p_chr_1);
            F::BtlChar_SetFlag(c, p_chr_1, ((((213i32) as i64 as u64)) as u32));
            if ((F::BtlChar_TestFlag(c, p_chr_1, ((((17i32) as i64 as u64)) as u32))) != 0) {
                F::BtlChar_SetHeldFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32));
            }
            { let a1_ = (F::BtlAnim_GetId(c, p_chr_1)).wrapping_add(3i32); let a2_ = (F::BtlAnim_GetId(c, p_chr_1)).wrapping_add(6i32); F::BtlAct_SetPitchMotion(c, p_chr_1, a1_, a2_, 1i32) };
            if ((F::BtlChar_TestFlag(c, p_chr_1, ((((5i32) as i64 as u64)) as u32))) != 0) {
                { let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); F::BtlCharApi_GetNodePos(c, a0_, 17i32, fp) };
                { let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); let a1_ = (c.m.r32((p_chr_1).wrapping_add(3568)) as i32); F::BtlCharApi_CalcAimDir(c, a0_, a1_, fp, (p_chr_1).wrapping_add(3536), F32(0xbf20d97a), F32(0x3f20d97a)) };
            } else {
                { let v_ = { let a0_ = { let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(148); F32(c.m.r32(a_)) }; F::Mathf_Sin(c, a0_) }; let a_ = (p_chr_1).wrapping_add(3536); c.m.w32(a_, (v_).0); };
                { let v_ = l_zero_4; let a_ = ((p_chr_1).wrapping_add(3536)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                { let v_ = { let a0_ = { let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(148); F32(c.m.r32(a_)) }; F::Mathf_Cos(c, a0_) }; let a_ = ((p_chr_1).wrapping_add(3536)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                { let v_ = l_zero_4; let a_ = ((p_chr_1).wrapping_add(3536)).wrapping_add(12); c.m.w32(a_, (v_).0); };
            }
        }
        if (((((p_phase_2) == (2i32)) as i32)) != 0) {
            if (((((F::BtlAnim_GetId(c, p_chr_1)) == (124i32)) as i32)) != 0) {
                if ((F::BtlInput_TestAction(c, p_chr_1, 91i32, 1i32)) != 0) {
                    F::BtlAnim_Request(c, p_chr_1, 125i32, F32(0x00000000));
                }
            }
        }
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_ChargedKiBlastHandler", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_DashKiBlastHandler(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_DashKiBlastHandler", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let mut l_anim_3: i32 = 0;
    let mut l_dir_4: u32 = 0;
    let mut l_speed_5: F32 = F32(0);
    let mut l_yawAccel_6: F32 = F32(0);
    let mut l_max_7: F32 = F32(0);
    let mut l_tmp_8: F32 = F32(0);
    let mut l_tmp2_9: F32 = F32(0);
    let r_ = 'f: {
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        l_anim_3 = 0i32;
        's1: {
            let sv10: i32 = { let a0_ = F::BtlAct_GetFacingRelCam(c, p_chr_1); F::BtlUtil_AngleToSector(c, a0_) };
            match sv10 {
                0i32 => {
                    l_anim_3 = 119i32;
                    if ((F::BtlChar_TestFlag(c, p_chr_1, ((((5i32) as i64 as u64)) as u32))) != 0) {
                        { let r_ = 1i32; let a_ = ((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                        F::BtlChar_SetHeldFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32));
                    }
                    break 's1;
                }
                2i32 => {
                    l_anim_3 = 120i32;
                    break 's1;
                }
                1i32 => {
                    l_anim_3 = 119i32;
                    break 's1;
                }
                3i32 => {
                    l_anim_3 = 121i32;
                    break 's1;
                }
                _ => {
                }
            }
        }
        F::BtlAnim_Play(c, p_chr_1, l_anim_3, F32(0x3e199999));
        { let _ = { let a1_ = F::BtlKiBlast_GetKiCost(c, p_chr_1); F::BtlMember_SpendKi(c, p_chr_1, a1_, 0i32) }; };
        F::BtlChar_PlayVoice(c, p_chr_1, 12i32);
        { let r_ = { let a0_ = F::BtlChar_GetObj(c, p_chr_1); F::BtlObjAnim_QueryEvent(c, a0_, ((4i32) as u64), 0i32, ((((3i32) as i64 as u64)) as u32)) }; let a_ = (p_chr_1).wrapping_add(3552); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
        { let v_ = 30i32; let a_ = (p_chr_1).wrapping_add(3556); c.m.w32(a_, (v_) as u32); };
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        if ((F::BtlAnim_Advance(c, p_chr_1, 0i32)) != 0) {
            F::BtlAct_Request(c, p_chr_1, 15i32);
        }
        l_dir_4 = (p_chr_1).wrapping_add(3536);
        { let a1_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(128); F::Vec4_Copy(c, l_dir_4, a1_) };
        's2: {
            let sv11: i32 = F::BtlAnim_GetId(c, p_chr_1);
            match sv11 {
                119i32 => {
                    break 's2;
                }
                120i32 => {
                    l_tmp_8 = rt::fneg(F32(c.m.r32(l_dir_4)));
                    l_tmp2_9 = F32(c.m.r32(((p_chr_1).wrapping_add(3536)).wrapping_add(8)));
                    { let v_ = l_tmp_8; let a_ = ((p_chr_1).wrapping_add(3536)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                    { let v_ = l_tmp2_9; let a_ = l_dir_4; c.m.w32(a_, (v_).0); };
                    break 's2;
                }
                121i32 => {
                    l_tmp_8 = rt::fneg(F32(c.m.r32(((p_chr_1).wrapping_add(3536)).wrapping_add(8))));
                    l_tmp2_9 = F32(c.m.r32(l_dir_4));
                    { let v_ = l_tmp_8; let a_ = l_dir_4; c.m.w32(a_, (v_).0); };
                    { let v_ = l_tmp2_9; let a_ = ((p_chr_1).wrapping_add(3536)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                    break 's2;
                }
                _ => {
                }
            }
        }
        if (((((((((c.m.r32(((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) & (1i32))) != 0) && ((F::BtlChar_TestFlag(c, p_chr_1, ((((5i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) {
            l_yawAccel_6 = F32(0x3e20d97a);
            l_speed_5 = { let a1_ = (F::BtlChar_TestFlag(c, p_chr_1, ((((17i32) as i64 as u64)) as u32))).wrapping_add(4i32); F::BtlMoveParam_GetSpeed(c, p_chr_1, a1_) };
            l_max_7 = F32(0x3f714637);
            if ((((((c.m.r32((p_chr_1).wrapping_add(2404)) as i32)) == (0i32)) as i32)) != 0) {
                l_yawAccel_6 = F32(0x40490fda);
            }
            { let a4_ = rt::fmul({ let a0_ = rt::fmul(F::BtlOpp_GetGapXZ(c, p_chr_1), F32(0x3c23d70a)); F::BtlUtil_MinF(c, a0_, F32(0x3f800000)) }, l_max_7); F::BtlMove_SteerAtOpponent(c, p_chr_1, l_speed_5, l_yawAccel_6, F32(0x00000000), a4_, F32(0x00000000), l_max_7) };
            F::BtlMove_TurnModelYaw(c, p_chr_1, F32(0x3f20d97a), F32(0x3f000000));
            F::BtlMove_SetDirection(c, p_chr_1, 3i32);
            F::BtlMove_Advance(c, p_chr_1, l_speed_5, F32(0x3fed0979));
        } else {
            l_speed_5 = { let a1_ = (F::BtlChar_TestFlag(c, p_chr_1, ((((17i32) as i64 as u64)) as u32))).wrapping_add(4i32); F::BtlMoveParam_GetSpeed(c, p_chr_1, a1_) };
            { let a2_ = F::BtlMoveParam_GetTurnRate(c, p_chr_1, 0i32); F::BtlMove_TurnYaw(c, p_chr_1, 0i32, a2_) };
            F::BtlMove_TurnPitch(c, p_chr_1, 5i32, F32(0x3e20d97a));
            F::BtlMove_TurnModelYaw(c, p_chr_1, F32(0x3f20d97a), F32(0x3e999999));
            F::BtlMove_SetDirection(c, p_chr_1, 3i32);
            F::BtlMove_Advance(c, p_chr_1, l_speed_5, F32(0x461c4000));
        }
        F::BtlMove_ApplyGravity(c, p_chr_1);
        F::BtlMove_RequestOrbit(c, p_chr_1, F32(0x41a00000), F32(0x42480000));
        F::BtlChar_SetFlag(c, p_chr_1, ((((202i32) as i64 as u64)) as u32));
        F::BtlChar_SetFxBit(c, p_chr_1, 11i32);
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((17i32) as i64 as u64)) as u32))) != 0) {
            F::BtlChar_SetHeldFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32));
        }
        if ((((((F::BtlChar_TestFlag(c, p_chr_1, ((((5i32) as i64 as u64)) as u32))) != 0) && (((((c.m.r32(((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) & (1i32))) != 0)) as i32)) != 0) {
            { let a1_ = { let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(144); F32(c.m.r32(a_)) }; F::BtlMove_SetLeanX(c, p_chr_1, a1_) };
        }
    }
    if (((((p_phase_2) == (2i32)) as i32)) != 0) {
        if (((rt::flt(F32(0x3f000000), F::BtlAnim_GetProgress(c, p_chr_1)) as i32)) != 0) {
            { let _ = F::BtlDecide_Attack(c, p_chr_1, 49152i32); };
            if ((F::BtlInput_TestAction(c, p_chr_1, 44i32, 1i32)) != 0) {
                F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), 62i32);
            }
            if ((F::BtlChar_TestFlag(c, p_chr_1, ((((49i32) as i64 as u64)) as u32))) != 0) {
                { let a1_ = F::BtlAct_GetQueued(c, p_chr_1); F::BtlAct_Request(c, p_chr_1, a1_) };
            }
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_DashKiBlastHandler", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_DashChargedKiBlastHandler(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_DashChargedKiBlastHandler", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let mut l_anim_3: i32 = 0;
    let mut l_dir_4: u32 = 0;
    let mut l_speed_5: F32 = F32(0);
    let mut l_yawAccel_6: F32 = F32(0);
    let mut l_max_7: F32 = F32(0);
    let mut l_tmp_8: F32 = F32(0);
    let mut l_tmp2_9: F32 = F32(0);
    let mut l_frame_10: F32 = F32(0);
    let r_ = 'f: {
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        l_anim_3 = 0i32;
        's1: {
            let sv11: i32 = { let a0_ = F::BtlAct_GetFacingRelCam(c, p_chr_1); F::BtlUtil_AngleToSector(c, a0_) };
            match sv11 {
                0i32 => {
                    l_anim_3 = 132i32;
                    if ((F::BtlChar_TestFlag(c, p_chr_1, ((((5i32) as i64 as u64)) as u32))) != 0) {
                        { let r_ = 1i32; let a_ = ((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                        F::BtlChar_SetHeldFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32));
                    }
                    break 's1;
                }
                2i32 => {
                    l_anim_3 = 135i32;
                    break 's1;
                }
                1i32 => {
                    l_anim_3 = 132i32;
                    break 's1;
                }
                3i32 => {
                    l_anim_3 = 138i32;
                    break 's1;
                }
                _ => {
                }
            }
        }
        F::BtlAnim_Play(c, p_chr_1, l_anim_3, F32(0x3e199999));
        { let v_ = F::BtlAct_PlaySubAnim(c, p_chr_1, (l_anim_3).wrapping_add(2i32)); let a_ = (p_chr_1).wrapping_add(3568); c.m.w32(a_, (v_) as u32); };
        F::BtlChar_SetFxBit(c, p_chr_1, 23i32);
        F::BtlCharSnd_PlayCommon(c, p_chr_1, 46i32);
        { let a_ = (p_chr_1).wrapping_add(3552); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        { let v_ = 30i32; let a_ = (p_chr_1).wrapping_add(3556); c.m.w32(a_, (v_) as u32); };
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        's2: {
            let sv12: i32 = F::BtlAnim_GetId(c, p_chr_1);
            match sv12 {
                132i32 | 135i32 | 138i32 => {
                    { let _ = { let a1_ = (F::BtlAnim_GetId(c, p_chr_1)).wrapping_add(1i32); F::BtlAnim_AdvanceThen(c, p_chr_1, a1_, F32(0x00000000), 0i32) }; };
                    F::BtlChar_SetFlag(c, p_chr_1, ((((142i32) as i64 as u64)) as u32));
                    break 's2;
                }
                133i32 | 136i32 | 139i32 => {
                    { let _ = { let a1_ = (F::BtlAnim_GetId(c, p_chr_1)).wrapping_add(1i32); F::BtlAnim_AdvanceThen(c, p_chr_1, a1_, F32(0x00000000), 0i32) }; };
                    F::BtlChar_SetFlag(c, p_chr_1, ((((142i32) as i64 as u64)) as u32));
                    { let v_ = F::BtlAnim_GetProgress(c, p_chr_1); let a_ = (p_chr_1).wrapping_add(3564); c.m.w32(a_, (v_).0); };
                    break 's2;
                }
                134i32 | 137i32 | 140i32 => {
                    if ((F::BtlAnim_Advance(c, p_chr_1, 0i32)) != 0) {
                        F::BtlAct_Request(c, p_chr_1, 15i32);
                    }
                    if ((F::BtlAnim_IsNew(c, p_chr_1)) != 0) {
                        F::BtlChar_PlayVoice(c, p_chr_1, 37i32);
                        { let _ = { let a1_ = F::BtlKiBlast_GetKiCost(c, p_chr_1); F::BtlMember_SpendKi(c, p_chr_1, a1_, 0i32) }; };
                    }
                    l_frame_10 = F::BtlAnim_GetFrame(c, p_chr_1);
                    if (((rt::flt(l_frame_10, rt::i2f({ let a0_ = F::BtlChar_GetObj(c, p_chr_1); F::BtlObjAnim_QueryEvent(c, a0_, ((4i32) as u64), 0i32, ((((0i32) as i64 as u64)) as u32)) })) as i32)) != 0) {
                        F::BtlChar_SetFlag(c, p_chr_1, ((((142i32) as i64 as u64)) as u32));
                    }
                    break 's2;
                }
                _ => {
                }
            }
        }
        l_dir_4 = (p_chr_1).wrapping_add(3536);
        { let a1_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(128); F::Vec4_Copy(c, l_dir_4, a1_) };
        's3: {
            let sv13: i32 = F::BtlAnim_GetId(c, p_chr_1);
            match sv13 {
                137i32 => {
                    l_tmp_8 = rt::fneg(F32(c.m.r32(l_dir_4)));
                    l_tmp2_9 = F32(c.m.r32(((p_chr_1).wrapping_add(3536)).wrapping_add(8)));
                    { let v_ = l_tmp_8; let a_ = ((p_chr_1).wrapping_add(3536)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                    { let v_ = l_tmp2_9; let a_ = l_dir_4; c.m.w32(a_, (v_).0); };
                    break 's3;
                }
                140i32 => {
                    l_tmp_8 = rt::fneg(F32(c.m.r32(((p_chr_1).wrapping_add(3536)).wrapping_add(8))));
                    l_tmp2_9 = F32(c.m.r32(l_dir_4));
                    { let v_ = l_tmp_8; let a_ = l_dir_4; c.m.w32(a_, (v_).0); };
                    { let v_ = l_tmp2_9; let a_ = ((p_chr_1).wrapping_add(3536)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                    break 's3;
                }
                _ => {
                }
            }
        }
        if (((((((((c.m.r32(((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) & (1i32))) != 0) && ((F::BtlChar_TestFlag(c, p_chr_1, ((((5i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) {
            l_yawAccel_6 = F32(0x3e20d97a);
            l_speed_5 = { let a1_ = (F::BtlChar_TestFlag(c, p_chr_1, ((((17i32) as i64 as u64)) as u32))).wrapping_add(4i32); F::BtlMoveParam_GetSpeed(c, p_chr_1, a1_) };
            l_max_7 = F32(0x3f714637);
            if ((((((c.m.r32((p_chr_1).wrapping_add(2404)) as i32)) == (0i32)) as i32)) != 0) {
                l_yawAccel_6 = F32(0x40490fda);
            }
            { let a4_ = rt::fmul({ let a0_ = rt::fmul(F::BtlOpp_GetGapXZ(c, p_chr_1), F32(0x3c23d70a)); F::BtlUtil_MinF(c, a0_, F32(0x3f800000)) }, l_max_7); F::BtlMove_SteerAtOpponent(c, p_chr_1, l_speed_5, l_yawAccel_6, F32(0x00000000), a4_, F32(0x00000000), l_max_7) };
            F::BtlMove_TurnModelYaw(c, p_chr_1, F32(0x3f20d97a), F32(0x3f000000));
            F::BtlMove_SetDirection(c, p_chr_1, 3i32);
            F::BtlMove_Advance(c, p_chr_1, l_speed_5, F32(0x3fed0979));
        } else {
            l_speed_5 = { let a1_ = (F::BtlChar_TestFlag(c, p_chr_1, ((((17i32) as i64 as u64)) as u32))).wrapping_add(4i32); F::BtlMoveParam_GetSpeed(c, p_chr_1, a1_) };
            { let a2_ = F::BtlMoveParam_GetTurnRate(c, p_chr_1, 0i32); F::BtlMove_TurnYaw(c, p_chr_1, 0i32, a2_) };
            F::BtlMove_TurnPitch(c, p_chr_1, 5i32, F32(0x3e20d97a));
            F::BtlMove_TurnModelYaw(c, p_chr_1, F32(0x3f20d97a), F32(0x3e999999));
            F::BtlMove_SetDirection(c, p_chr_1, 3i32);
            F::BtlMove_Advance(c, p_chr_1, l_speed_5, F32(0x461c4000));
        }
        F::BtlMove_ApplyGravity(c, p_chr_1);
        F::BtlMove_RequestOrbit(c, p_chr_1, F32(0x41a00000), F32(0x42480000));
        F::BtlChar_SetFlag(c, p_chr_1, ((((202i32) as i64 as u64)) as u32));
        F::BtlChar_SetFxBit(c, p_chr_1, 11i32);
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((17i32) as i64 as u64)) as u32))) != 0) {
            F::BtlChar_SetHeldFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32));
        }
        if ((((((F::BtlChar_TestFlag(c, p_chr_1, ((((5i32) as i64 as u64)) as u32))) != 0) && (((((c.m.r32(((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) & (1i32))) != 0)) as i32)) != 0) {
            { let a1_ = { let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(144); F32(c.m.r32(a_)) }; F::BtlMove_SetLeanX(c, p_chr_1, a1_) };
        }
    }
    if (((((p_phase_2) == (2i32)) as i32)) != 0) {
        's4: {
            let sv14: i32 = F::BtlAnim_GetId(c, p_chr_1);
            match sv14 {
                132i32 | 135i32 | 138i32 => {
                    break 's4;
                }
                133i32 | 136i32 | 139i32 => {
                    if ((F::BtlInput_TestAction(c, p_chr_1, 91i32, 1i32)) != 0) {
                        { let a1_ = (F::BtlAnim_GetId(c, p_chr_1)).wrapping_add(1i32); F::BtlAnim_Request(c, p_chr_1, a1_, F32(0x00000000)) };
                    }
                    break 's4;
                }
                134i32 | 137i32 | 140i32 => {
                    break 's4;
                }
                _ => {
                }
            }
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_DashChargedKiBlastHandler", &PROBE_N, &[], true); }
    r_
}
