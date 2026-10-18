#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn BtlFacade_ShakeCamera(c: &mut Ctx, mut p_time_1: F32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_ShakeCamera", &PROBE_N, &[(p_time_1).0], false); }
    let r_ = 'f: {
    F::DemoCam_AddShake(c, p_time_1);
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_ShakeCamera", &PROBE_N, &[], true); }
    r_
}

pub fn BtlFacade_SetFixedCamera(c: &mut Ctx, mut p_pos_1: u32, mut p_rot_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_SetFixedCamera", &PROBE_N, &[(p_pos_1) as u32, (p_rot_2) as u32], false); }
    let r_ = 'f: {
    F::DemoCam_SetFixedPose(c, p_pos_1, p_rot_2);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_SetFixedCamera", &PROBE_N, &[], true); }
    r_
}

pub fn BtlFacade_GetFixedCamera(c: &mut Ctx, mut p_pos_1: u32, mut p_rot_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_GetFixedCamera", &PROBE_N, &[(p_pos_1) as u32, (p_rot_2) as u32], false); }
    let r_ = 'f: {
    F::DemoCam_GetFixedPose(c, p_pos_1, p_rot_2);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_GetFixedCamera", &PROBE_N, &[], true); }
    r_
}

pub fn BtlFacade_ClearFixedCamera(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_ClearFixedCamera", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::DemoCam_ClearFixed(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_ClearFixedCamera", &PROBE_N, &[], true); }
    r_
}

pub fn BtlFacade_SetObjSubState14(c: &mut Ctx, mut p_side_1: i32, mut p_arg_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_SetObjSubState14", &PROBE_N, &[(p_side_1) as u32, (p_arg_2) as u32], false); }
    let r_ = 'f: {
    { let a0_ = { let a0_ = F::BattleSide_GetObjId(c, p_side_1); F::BtlObj_Get(c, a0_) }; F::BtlObj_SetSubState(c, a0_, 14i32, p_arg_2) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_SetObjSubState14", &PROBE_N, &[], true); }
    r_
}

pub fn BtlFacade_EndObjSubState3(c: &mut Ctx, mut p_side_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_EndObjSubState3", &PROBE_N, &[(p_side_1) as u32], false); }
    let mut l_obj_2: u32 = 0;
    let r_ = 'f: {
    l_obj_2 = { let a0_ = F::BattleSide_GetObjId(c, p_side_1); F::BtlObj_Get(c, a0_) };
    if (((((F::BtlObj_GetSubState(c, l_obj_2)) == (3i32)) as i32)) != 0) {
        F::BtlObj_SetSubState(c, l_obj_2, 0i32, (1i32).wrapping_neg());
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_EndObjSubState3", &PROBE_N, &[], true); }
    r_
}

pub fn BtlFacade_SetCpuParam8(c: &mut Ctx, mut p_arg_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_SetCpuParam8", &PROBE_N, &[(p_arg_1) as u32], false); }
    let r_ = 'f: {
    F::BtlAiMgr_SetLevel(c, 0i32, p_arg_1);
    F::BtlAiMgr_SetLevel(c, 1i32, p_arg_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_SetCpuParam8", &PROBE_N, &[], true); }
    r_
}

pub fn BtlFacade_SetCpuParam4(c: &mut Ctx, mut p_arg_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_SetCpuParam4", &PROBE_N, &[(p_arg_1) as u32], false); }
    let r_ = 'f: {
    F::BtlAiMgr_SetType(c, 0i32, p_arg_1);
    F::BtlAiMgr_SetType(c, 1i32, p_arg_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_SetCpuParam4", &PROBE_N, &[], true); }
    r_
}

pub fn BtlFacade_SetCharPos(c: &mut Ctx, mut p_side_1: i32, mut p_pos_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_SetCharPos", &PROBE_N, &[(p_side_1) as u32, (p_pos_2) as u32], false); }
    let r_ = 'f: {
    F::BtlCtrl_SetPos(c, p_side_1, p_pos_2);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_SetCharPos", &PROBE_N, &[], true); }
    r_
}

pub fn BtlFacade_SetCharRot(c: &mut Ctx, mut p_side_1: i32, mut p_dir_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_SetCharRot", &PROBE_N, &[(p_side_1) as u32, (p_dir_2) as u32], false); }
    let r_ = 'f: {
    F::BtlCtrl_SetRot(c, p_side_1, p_dir_2);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_SetCharRot", &PROBE_N, &[], true); }
    r_
}

pub fn BtlFacade_PlayCharMotion(c: &mut Ctx, mut p_side_1: i32, mut p_type_2: i32, mut p_mode_3: i32, mut p_value_4: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_PlayCharMotion", &PROBE_N, &[(p_side_1) as u32, (p_type_2) as u32, (p_mode_3) as u32, (p_value_4).0], false); }
    let r_ = 'f: {
    F::BtlCtrl_PlayMotion(c, p_side_1, p_type_2, p_mode_3, p_value_4);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_PlayCharMotion", &PROBE_N, &[], true); }
    r_
}

pub fn BtlFacade_IsCharMotionPlaying(c: &mut Ctx, mut p_side_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_IsCharMotionPlaying", &PROBE_N, &[(p_side_1) as u32], false); }
    let r_ = 'f: {
    break 'f F::BtlCtrl_IsMotionPlaying(c, p_side_1);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_IsCharMotionPlaying", &PROBE_N, &[], true); }
    r_
}

pub fn BtlFacade_IsCharAction4(c: &mut Ctx, mut p_side_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_IsCharAction4", &PROBE_N, &[(p_side_1) as u32], false); }
    let r_ = 'f: {
    break 'f F::BtlCtrl_IsAction4(c, p_side_1);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_IsCharAction4", &PROBE_N, &[], true); }
    r_
}

pub fn BtlFacade_StopCharMotion(c: &mut Ctx, mut p_side_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_StopCharMotion", &PROBE_N, &[(p_side_1) as u32], false); }
    let r_ = 'f: {
    F::BtlCtrl_StopMotion(c, p_side_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_StopCharMotion", &PROBE_N, &[], true); }
    r_
}

pub fn BtlFacade_SetCtrlFE(c: &mut Ctx, mut p_side_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_SetCtrlFE", &PROBE_N, &[(p_side_1) as u32], false); }
    let r_ = 'f: {
    F::BtlCtrl_SetAuraOn(c, p_side_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_SetCtrlFE", &PROBE_N, &[], true); }
    r_
}

pub fn BtlFacade_SetCtrlFF(c: &mut Ctx, mut p_side_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_SetCtrlFF", &PROBE_N, &[(p_side_1) as u32], false); }
    let r_ = 'f: {
    F::BtlCtrl_SetAuraOff(c, p_side_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_SetCtrlFF", &PROBE_N, &[], true); }
    r_
}

pub fn BtlFacade_SetCtrl101(c: &mut Ctx, mut p_side_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_SetCtrl101", &PROBE_N, &[(p_side_1) as u32], false); }
    let r_ = 'f: {
    { let _ = F::BtlCtrl_SetChargeFx(c, p_side_1, 1i32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_SetCtrl101", &PROBE_N, &[], true); }
    r_
}

pub fn BtlFacade_ClearCtrl101(c: &mut Ctx, mut p_side_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_ClearCtrl101", &PROBE_N, &[(p_side_1) as u32], false); }
    let r_ = 'f: {
    { let _ = F::BtlCtrl_SetChargeFx(c, p_side_1, 0i32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_ClearCtrl101", &PROBE_N, &[], true); }
    r_
}

pub fn BtlFacade_SetCtrl102(c: &mut Ctx, mut p_side_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_SetCtrl102", &PROBE_N, &[(p_side_1) as u32], false); }
    let r_ = 'f: {
    F::BtlCtrl_BurstChargeFx(c, p_side_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_SetCtrl102", &PROBE_N, &[], true); }
    r_
}

pub fn BtlFacade_ClearCtrl100(c: &mut Ctx, mut p_side_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_ClearCtrl100", &PROBE_N, &[(p_side_1) as u32], false); }
    let r_ = 'f: {
    F::BtlCtrl_ClearHidden(c, p_side_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_ClearCtrl100", &PROBE_N, &[], true); }
    r_
}

pub fn BtlFacade_SetCtrl100(c: &mut Ctx, mut p_side_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_SetCtrl100", &PROBE_N, &[(p_side_1) as u32], false); }
    let r_ = 'f: {
    F::BtlCtrl_SetHidden(c, p_side_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_SetCtrl100", &PROBE_N, &[], true); }
    r_
}

pub fn BtlFacade_SetCtrl10D(c: &mut Ctx, mut p_side_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_SetCtrl10D", &PROBE_N, &[(p_side_1) as u32], false); }
    let r_ = 'f: {
    F::BtlCtrl_SetFlag10D(c, p_side_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_SetCtrl10D", &PROBE_N, &[], true); }
    r_
}

pub fn BtlFacade_ClearCtrl10D(c: &mut Ctx, mut p_side_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_ClearCtrl10D", &PROBE_N, &[(p_side_1) as u32], false); }
    let r_ = 'f: {
    F::BtlCtrl_ClearFlag10D(c, p_side_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_ClearCtrl10D", &PROBE_N, &[], true); }
    r_
}

pub fn BtlFacade_GetActiveMember(c: &mut Ctx, mut p_side_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_GetActiveMember", &PROBE_N, &[(p_side_1) as u32], false); }
    let r_ = 'f: {
    break 'f F::BtlCtrl_GetActiveMember(c, p_side_1);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_GetActiveMember", &PROBE_N, &[], true); }
    r_
}

pub fn BtlFacade_NotifyMemberItems(c: &mut Ctx, mut p_side_1: i32, mut p_member_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_NotifyMemberItems", &PROBE_N, &[(p_side_1) as u32, (p_member_2) as u32], false); }
    let r_ = 'f: {
    F::BtlCtrl_ReloadMemberBonus(c, p_side_1, ((((p_member_2) as i64 as u64)) as u32));
    F::BtlCtrl_ReloadMemberAbilities(c, p_side_1, ((((p_member_2) as i64 as u64)) as u32));
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_NotifyMemberItems", &PROBE_N, &[], true); }
    r_
}

pub fn BtlFacade_SetMemberItems(c: &mut Ctx, mut p_side_1: i32, mut p_member_2: i32, mut p_src_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_SetMemberItems", &PROBE_N, &[(p_side_1) as u32, (p_member_2) as u32, (p_src_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_i_4: i32 = 0;
    let r_ = 'f: {
        { let _ = F::memset(c, fp, 0i32, 16u32); };
        {
            l_i_4 = 0i32;
            'l1: loop {
                if !(((((l_i_4) < (8i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let v_ = (((((c.m.r16((p_src_3).wrapping_add((((l_i_4) as u32)).wrapping_mul(4)))) as i32)).wrapping_add(1i32)) as u16); let a_ = (fp).wrapping_add((((l_i_4) as u32)).wrapping_mul(2)); c.m.w16(a_, v_); };
                }
                { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
            }
        }
        F::BattleSide_SetMemberItems(c, p_side_1, p_member_2, fp);
        if (((((p_member_2) == ((1i32).wrapping_neg())) as i32)) != 0) {
            { let a1_ = F::BtlCtrl_GetActiveMember(c, p_side_1); F::BtlFacade_NotifyMemberItems(c, p_side_1, a1_) };
        } else {
            F::BtlFacade_NotifyMemberItems(c, p_side_1, p_member_2);
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_SetMemberItems", &PROBE_N, &[], true); }
    r_
}

pub fn BtlFacade_AddHp(c: &mut Ctx, mut p_side_1: i32, mut p_member_2: i32, mut p_value_3: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_AddHp", &PROBE_N, &[(p_side_1) as u32, (p_member_2) as u32, (p_value_3).0], false); }
    let r_ = 'f: {
    if (((((p_member_2) == ((1i32).wrapping_neg())) as i32)) != 0) {
        { let a1_ = F::BtlCtrl_GetActiveMember(c, p_side_1); { let _ = F::BtlCtrl_AddHp(c, p_side_1, a1_, p_value_3); } };
    } else {
        { let _ = F::BtlCtrl_AddHp(c, p_side_1, p_member_2, p_value_3); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_AddHp", &PROBE_N, &[], true); }
    r_
}

pub fn BtlFacade_RaiseHp(c: &mut Ctx, mut p_side_1: i32, mut p_member_2: i32, mut p_value_3: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_RaiseHp", &PROBE_N, &[(p_side_1) as u32, (p_member_2) as u32, (p_value_3).0], false); }
    let r_ = 'f: {
    if (((((p_member_2) == ((1i32).wrapping_neg())) as i32)) != 0) {
        { let a1_ = F::BtlCtrl_GetActiveMember(c, p_side_1); F::BtlCtrl_RaiseHp(c, p_side_1, a1_, p_value_3) };
    } else {
        F::BtlCtrl_RaiseHp(c, p_side_1, p_member_2, p_value_3);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_RaiseHp", &PROBE_N, &[], true); }
    r_
}

pub fn BtlFacade_LowerHp(c: &mut Ctx, mut p_side_1: i32, mut p_member_2: i32, mut p_value_3: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_LowerHp", &PROBE_N, &[(p_side_1) as u32, (p_member_2) as u32, (p_value_3).0], false); }
    let r_ = 'f: {
    if (((rt::fle(p_value_3, F32(0x00000000)) as i32)) != 0) {
        if (((((p_side_1) == (0i32)) as i32)) != 0) {
            F::BtlEvent_Raise(c, p_side_1, 74i32);
            F::BtlEvent_Raise(c, 1i32, 72i32);
        } else {
            F::BtlEvent_Raise(c, 0i32, 72i32);
            F::BtlEvent_Raise(c, 1i32, 74i32);
        }
    }
    if (((((p_member_2) == ((1i32).wrapping_neg())) as i32)) != 0) {
        { let a1_ = F::BtlCtrl_GetActiveMember(c, p_side_1); F::BtlCtrl_LowerHp(c, p_side_1, a1_, p_value_3) };
    } else {
        F::BtlCtrl_LowerHp(c, p_side_1, p_member_2, p_value_3);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_LowerHp", &PROBE_N, &[], true); }
    r_
}

pub fn BtlFacade_AddGaugeC(c: &mut Ctx, mut p_side_1: i32, mut p_member_2: i32, mut p_value_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_AddGaugeC", &PROBE_N, &[(p_side_1) as u32, (p_member_2) as u32, (p_value_3) as u32], false); }
    let r_ = 'f: {
    if (((((p_member_2) == ((1i32).wrapping_neg())) as i32)) != 0) {
        { let a1_ = F::BtlCtrl_GetActiveMember(c, p_side_1); { let _ = F::BtlCtrl_AddKi(c, p_side_1, a1_, p_value_3); } };
    } else {
        { let _ = F::BtlCtrl_AddKi(c, p_side_1, p_member_2, p_value_3); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_AddGaugeC", &PROBE_N, &[], true); }
    r_
}

pub fn BtlFacade_RaiseGaugeC(c: &mut Ctx, mut p_side_1: i32, mut p_member_2: i32, mut p_value_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_RaiseGaugeC", &PROBE_N, &[(p_side_1) as u32, (p_member_2) as u32, (p_value_3) as u32], false); }
    let r_ = 'f: {
    if (((((p_member_2) == ((1i32).wrapping_neg())) as i32)) != 0) {
        { let a1_ = F::BtlCtrl_GetActiveMember(c, p_side_1); F::BtlCtrl_RaiseKi(c, p_side_1, a1_, p_value_3) };
    } else {
        F::BtlCtrl_RaiseKi(c, p_side_1, p_member_2, p_value_3);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_RaiseGaugeC", &PROBE_N, &[], true); }
    r_
}

pub fn BtlFacade_LowerGaugeC(c: &mut Ctx, mut p_side_1: i32, mut p_member_2: i32, mut p_value_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_LowerGaugeC", &PROBE_N, &[(p_side_1) as u32, (p_member_2) as u32, (p_value_3) as u32], false); }
    let r_ = 'f: {
    if (((((p_member_2) == ((1i32).wrapping_neg())) as i32)) != 0) {
        { let a1_ = F::BtlCtrl_GetActiveMember(c, p_side_1); F::BtlCtrl_LowerKi(c, p_side_1, a1_, p_value_3) };
    } else {
        F::BtlCtrl_LowerKi(c, p_side_1, p_member_2, p_value_3);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_LowerGaugeC", &PROBE_N, &[], true); }
    r_
}

pub fn BtlFacade_AddGauge14(c: &mut Ctx, mut p_side_1: i32, mut p_member_2: i32, mut p_value_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_AddGauge14", &PROBE_N, &[(p_side_1) as u32, (p_member_2) as u32, (p_value_3) as u32], false); }
    let r_ = 'f: {
    if (((((p_member_2) == ((1i32).wrapping_neg())) as i32)) != 0) {
        { let a1_ = F::BtlCtrl_GetActiveMember(c, p_side_1); { let _ = F::BtlCtrl_AddBlast(c, p_side_1, a1_, p_value_3); } };
    } else {
        { let _ = F::BtlCtrl_AddBlast(c, p_side_1, p_member_2, p_value_3); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_AddGauge14", &PROBE_N, &[], true); }
    r_
}

pub fn BtlFacade_RaiseGauge14(c: &mut Ctx, mut p_side_1: i32, mut p_member_2: i32, mut p_value_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_RaiseGauge14", &PROBE_N, &[(p_side_1) as u32, (p_member_2) as u32, (p_value_3) as u32], false); }
    let r_ = 'f: {
    if (((((p_member_2) == ((1i32).wrapping_neg())) as i32)) != 0) {
        { let a1_ = F::BtlCtrl_GetActiveMember(c, p_side_1); F::BtlCtrl_RaiseBlast(c, p_side_1, a1_, p_value_3) };
    } else {
        F::BtlCtrl_RaiseBlast(c, p_side_1, p_member_2, p_value_3);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_RaiseGauge14", &PROBE_N, &[], true); }
    r_
}

pub fn BtlFacade_LowerGauge14(c: &mut Ctx, mut p_side_1: i32, mut p_member_2: i32, mut p_value_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_LowerGauge14", &PROBE_N, &[(p_side_1) as u32, (p_member_2) as u32, (p_value_3) as u32], false); }
    let r_ = 'f: {
    if (((((p_member_2) == ((1i32).wrapping_neg())) as i32)) != 0) {
        { let a1_ = F::BtlCtrl_GetActiveMember(c, p_side_1); F::BtlCtrl_LowerBlast(c, p_side_1, a1_, p_value_3) };
    } else {
        F::BtlCtrl_LowerBlast(c, p_side_1, p_member_2, p_value_3);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_LowerGauge14", &PROBE_N, &[], true); }
    r_
}

pub fn BtlFacade_SetMaxPower(c: &mut Ctx, mut p_side_1: i32, mut p_on_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_SetMaxPower", &PROBE_N, &[(p_side_1) as u32, (p_on_2) as u32], false); }
    let r_ = 'f: {
    { let _ = F::BtlCtrl_SetMaxPower(c, p_side_1, p_on_2); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_SetMaxPower", &PROBE_N, &[], true); }
    r_
}

pub fn BtlFacade_Transform(c: &mut Ctx, mut p_side_1: i32, mut p_id_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_Transform", &PROBE_N, &[(p_side_1) as u32, (p_id_2) as u32], false); }
    let r_ = 'f: {
    if ((F::BtlCtrl_Transform(c, p_side_1, p_id_2)) != 0) {
        F::BtlEvent_SetWaitOff(c, 0i32);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_Transform", &PROBE_N, &[], true); }
    r_
}

pub fn BtlFacade_Fuse(c: &mut Ctx, mut p_side_1: i32, mut p_id_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_Fuse", &PROBE_N, &[(p_side_1) as u32, (p_id_2) as u32], false); }
    let r_ = 'f: {
    if ((F::BtlCtrl_Fuse(c, p_side_1, p_id_2)) != 0) {
        F::BtlEvent_SetWaitOff(c, 0i32);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_Fuse", &PROBE_N, &[], true); }
    r_
}

pub fn BtlFacade_ChangeMember(c: &mut Ctx, mut p_side_1: i32, mut p_member_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_ChangeMember", &PROBE_N, &[(p_side_1) as u32, (p_member_2) as u32], false); }
    let r_ = 'f: {
    if ((F::BtlCtrl_ChangeMember(c, p_side_1, p_member_2)) != 0) {
        F::BtlEvent_SetWaitOff(c, 0i32);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_ChangeMember", &PROBE_N, &[], true); }
    r_
}

pub fn BtlFacade_PulseCtrl10D(c: &mut Ctx, mut p_side_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_PulseCtrl10D", &PROBE_N, &[(p_side_1) as u32], false); }
    let r_ = 'f: {
    F::BtlFacade_ClearCtrl10D(c, p_side_1);
    F::BtlFacade_SetCtrl10D(c, p_side_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_PulseCtrl10D", &PROBE_N, &[], true); }
    r_
}

pub fn BtlFacade_AreBothInterruptible(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_AreBothInterruptible", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if ((((((F::BtlCtrl_IsInterruptible(c, 0i32)) != 0) && ((F::BtlCtrl_IsInterruptible(c, 1i32)) != 0)) as i32)) != 0) {
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_AreBothInterruptible", &PROBE_N, &[], true); }
    r_
}

pub fn BtlFacade_CanCharAct(c: &mut Ctx, mut p_side_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_CanCharAct", &PROBE_N, &[(p_side_1) as u32], false); }
    let r_ = 'f: {
    break 'f (((F::BtlCtrl_CanAct(c, p_side_1)) != (0i32)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_CanCharAct", &PROBE_N, &[], true); }
    r_
}

pub fn BtlFacade_ForceAction01(c: &mut Ctx, mut p_side_1: i32, mut p_arg_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_ForceAction01", &PROBE_N, &[(p_side_1) as u32, (p_arg_2) as u32], false); }
    let r_ = 'f: {
    { let _ = F::BtlCtrl_ForceReaction(c, (((p_side_1) == (0i32)) as i32), ((((0i32) as i64 as u64)) as u32)); };
    { let _ = F::BtlCtrl_UseTechnique(c, p_side_1, (((p_arg_2) != (0i32)) as i32)); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_ForceAction01", &PROBE_N, &[], true); }
    r_
}

pub fn BtlFacade_ForceAction23(c: &mut Ctx, mut p_side_1: i32, mut p_arg_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_ForceAction23", &PROBE_N, &[(p_side_1) as u32, (p_arg_2) as u32], false); }
    let r_ = 'f: {
    { let _ = F::BtlCtrl_ForceReaction(c, (((p_side_1) == (0i32)) as i32), ((((0i32) as i64 as u64)) as u32)); };
    { let _ = F::BtlCtrl_UseTechnique(c, p_side_1, (if (((((p_arg_2) == (0i32)) as i32)) != 0) { 2i32 } else { 3i32 })); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_ForceAction23", &PROBE_N, &[], true); }
    r_
}

pub fn BtlFacade_ForceAction4(c: &mut Ctx, mut p_side_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_ForceAction4", &PROBE_N, &[(p_side_1) as u32], false); }
    let r_ = 'f: {
    { let _ = F::BtlCtrl_ForceReaction(c, (((p_side_1) == (0i32)) as i32), ((((0i32) as i64 as u64)) as u32)); };
    { let _ = F::BtlCtrl_UseTechnique(c, p_side_1, 4i32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_ForceAction4", &PROBE_N, &[], true); }
    r_
}

pub fn BtlFacade_Nop(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_Nop", &PROBE_N, &[], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_Nop", &PROBE_N, &[], true); }
    r_
}

pub fn BtlFacade_ForceBoth111(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_ForceBoth111", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let r_ = 'f: {
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (2i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let _ = F::BtlCtrl_ForceFlag11x(c, l_i_1, 1i32); };
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_ForceBoth111", &PROBE_N, &[], true); }
    r_
}

pub fn BtlFacade_ForceBoth110(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_ForceBoth110", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let r_ = 'f: {
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (2i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let _ = F::BtlCtrl_ForceFlag11x(c, l_i_1, 0i32); };
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_ForceBoth110", &PROBE_N, &[], true); }
    r_
}

pub fn BtlFacade_ForceActions(c: &mut Ctx, mut p_arg0_1: i32, mut p_arg1_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_ForceActions", &PROBE_N, &[(p_arg0_1) as u32, (p_arg1_2) as u32], false); }
    let mut l_i_3: i32 = 0;
    let r_ = 'f: {
    {
        l_i_3 = 0i32;
        'l1: loop {
            if !(((((l_i_3) < (2i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let _ = F::BtlCtrl_UseTechnique(c, l_i_3, (if (((((l_i_3) == (0i32)) as i32)) != 0) { p_arg0_1 } else { p_arg1_2 })); };
            }
            { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_ForceActions", &PROBE_N, &[], true); }
    r_
}

pub fn BtlFacade_CallSeqExtra(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_CallSeqExtra", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let _ = F::BtlSeq_CallExtra(c); };
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_CallSeqExtra", &PROBE_N, &[], true); }
    r_
}

pub fn BtlFacade_CallSeqExtra2(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_CallSeqExtra2", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let _ = F::BtlSeq_CallExtra(c); };
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_CallSeqExtra2", &PROBE_N, &[], true); }
    r_
}

pub fn BtlFacade_ResetSubClock(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_ResetSubClock", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::BtlSeq_ResetSubClock(c);
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_ResetSubClock", &PROBE_N, &[], true); }
    r_
}

pub fn BtlFacade_SetEndCheckOff(c: &mut Ctx, mut p_off_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_SetEndCheckOff", &PROBE_N, &[(p_off_1) as u32], false); }
    let r_ = 'f: {
    F::BtlSeq_SetEndCheckOff(c, p_off_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_SetEndCheckOff", &PROBE_N, &[], true); }
    r_
}

pub fn BtlFacade_GetEndCheckOff(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_GetEndCheckOff", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f F::BtlSeq_GetEndCheckOff(c);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_GetEndCheckOff", &PROBE_N, &[], true); }
    r_
}

pub fn BtlFacade_RequestStageChange(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_RequestStageChange", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::BtlStage_RequestChange(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_RequestStageChange", &PROBE_N, &[], true); }
    r_
}

pub fn BtlFacade_SetFlag200(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_SetFlag200", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let r_ = ((512i32) as u64); let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_SetFlag200", &PROBE_N, &[], true); }
    r_
}

pub fn BtlFacade_ClearFlag200(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_ClearFlag200", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let r_ = (((!(512i32))) as u64); let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); let n_ = ((c.m.r64(a_)) & (r_)); c.m.w64(a_, n_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_ClearFlag200", &PROBE_N, &[], true); }
    r_
}

pub fn BtlFacade_SetResult(c: &mut Ctx, mut p_kind_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_SetResult", &PROBE_N, &[(p_kind_1) as u32], false); }
    let r_ = 'f: {
    's1: {
        let sv2: i32 = p_kind_1;
        match sv2 {
            0i32 => {
                F::BattleResult_Set(c, 2i32, 1i32);
                break 's1;
            }
            1i32 => {
                F::BattleResult_Set(c, 1i32, 1i32);
                break 's1;
            }
            2i32 => {
                F::BattleResult_Set(c, 2i32, 4i32);
                break 's1;
            }
            3i32 => {
                F::BattleResult_Set(c, 1i32, 4i32);
                break 's1;
            }
            _ => {
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_SetResult", &PROBE_N, &[], true); }
    r_
}

pub fn BtlFacade_IsNotFinished(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_IsNotFinished", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f (((F::BtlSeq_IsFinish(c)) == (0i32)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_IsNotFinished", &PROBE_N, &[], true); }
    r_
}

pub fn BtlFacade_BeginInterrupt(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_BeginInterrupt", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::BtlEvent_BeginInterrupt(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_BeginInterrupt", &PROBE_N, &[], true); }
    r_
}

pub fn BtlFacade_EndInterrupt(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_EndInterrupt", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::BtlEvent_EndInterrupt(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_EndInterrupt", &PROBE_N, &[], true); }
    r_
}

pub fn BtlFacade_BeginWait(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_BeginWait", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::BtlEvent_SetWaitOff(c, 1i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_BeginWait", &PROBE_N, &[], true); }
    r_
}

pub fn BtlFacade_IsWaitOff(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_IsWaitOff", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f F::BtlEvent_IsWaitOff(c);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlFacade_IsWaitOff", &PROBE_N, &[], true); }
    r_
}
