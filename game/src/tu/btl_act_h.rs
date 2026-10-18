#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn BtlActThrow_SetReleaseHeading(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlActThrow_SetReleaseHeading", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_pose_2: u32 = 0;
    let mut l_yaw_3: F32 = F32(0);
    let mut l_pitch_4: F32 = F32(0);
    let r_ = 'f: {
    l_pose_2 = F::BtlChar_GetPos(c, p_chr_1);
    if ((((((c.m.r32(((p_chr_1).wrapping_add(4016)).wrapping_add(12)) as i32)) != (0i32)) as i32)) != 0) {
        l_yaw_3 = { let a0_ = rt::fadd({ let a0_ = rt::fadd(F32(c.m.r32((l_pose_2).wrapping_add(148))), F32(c.m.r32(((p_chr_1).wrapping_add(4016)).wrapping_add(40)))); F::BtlUtil_WrapAngle(c, a0_) }, F32(0x40490fda)); F::BtlUtil_WrapAngle(c, a0_) };
        l_pitch_4 = F32(c.m.r32(((p_chr_1).wrapping_add(4016)).wrapping_add(44)));
    } else {
        l_yaw_3 = { let a0_ = rt::fadd(F32(c.m.r32((l_pose_2).wrapping_add(148))), F32(c.m.r32(((p_chr_1).wrapping_add(4016)).wrapping_add(40)))); F::BtlUtil_WrapAngle(c, a0_) };
        l_pitch_4 = rt::fneg(F32(c.m.r32(((p_chr_1).wrapping_add(4016)).wrapping_add(44))));
    }
    { let a2_ = rt::fmul(l_pitch_4, { let a0_ = F32(c.m.r32(((l_pose_2).wrapping_add(96)).wrapping_add(4))); F::Mathf_Cos(c, a0_) }); F::BtlMove_SetHeading(c, p_chr_1, l_yaw_3, a2_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlActThrow_SetReleaseHeading", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_GrabDash(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_GrabDash", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let mut l_yawMode_3: i32 = 0;
    let r_ = 'f: {
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        { let a1_ = (if (((((F::BtlAct_GetCurrent(c, p_chr_1)) == (182i32)) as i32)) != 0) { 390i32 } else { 148i32 }); F::BtlAnim_Play(c, p_chr_1, a1_, F32(0x3e199999)) };
        F::BtlMove_BeginRiseToOpponent(c, p_chr_1);
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        l_yawMode_3 = 6i32;
        's1: {
            let sv4: i32 = F::BtlAct_GetCurrent(c, p_chr_1);
            match sv4 {
                181i32 => {
                    l_yawMode_3 = 2i32;
                    break 's1;
                }
                180i32 => {
                    l_yawMode_3 = 6i32;
                    break 's1;
                }
                182i32 => {
                    l_yawMode_3 = 6i32;
                    break 's1;
                }
                _ => {
                    break 's1;
                }
            }
        }
        's2: {
            let sv5: i32 = F::BtlAnim_GetId(c, p_chr_1);
            match sv5 {
                148i32 => {
                    { let _ = F::BtlAnim_AdvanceThen(c, p_chr_1, 149i32, F32(0x00000000), 0i32); };
                    break 's2;
                }
                390i32 => {
                    { let _ = F::BtlAnim_AdvanceThen(c, p_chr_1, 391i32, F32(0x00000000), 0i32); };
                    break 's2;
                }
                149i32 | 157i32 | 391i32 => {
                    if ((F::BtlAnim_Advance(c, p_chr_1, 0i32)) != 0) {
                        F::BtlAct_Request(c, p_chr_1, 11i32);
                    }
                    break 's2;
                }
                _ => {
                }
            }
        }
        F::BtlMove_Step(c, p_chr_1, l_yawMode_3, 5i32, 2i32, F32(0x00000000), F32(0x3f6d0979));
        F::BtlMove_ApplyGravity(c, p_chr_1);
        F::BtlChar_SetFlag(c, p_chr_1, ((((213i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((146i32) as i64 as u64)) as u32));
        if (((((F::BtlAct_GetCurrent(c, p_chr_1)) == (182i32)) as i32)) != 0) {
            F::BtlAnim_SetObjRate(c, p_chr_1, F32(0x40000000));
        }
    }
    if (((((p_phase_2) == (2i32)) as i32)) != 0) {
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((122i32) as i64 as u64)) as u32))) != 0) {
            F::BtlAnim_Request(c, p_chr_1, 157i32, F32(0x00000000));
        }
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((91i32) as i64 as u64)) as u32))) != 0) {
            's3: {
                let sv6: i32 = F::BtlAct_GetCurrent(c, p_chr_1);
                match sv6 {
                    180i32 | 181i32 => {
                        if ((((F::BtlParam_GetFlags(c, p_chr_1)) & (((512i32) as u32)))) != 0) {
                            F::BtlAct_Request(c, p_chr_1, 187i32);
                        } else {
                            F::BtlAct_Request(c, p_chr_1, 183i32);
                        }
                        break 's3;
                    }
                    182i32 => {
                        F::BtlAct_Request(c, p_chr_1, 185i32);
                        break 's3;
                    }
                    _ => {
                    }
                }
            }
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_GrabDash", &PROBE_N, &[], true); }
    r_
}
