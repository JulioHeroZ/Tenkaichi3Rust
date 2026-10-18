#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn BtlOpp_GetTargetPos(c: &mut Ctx, mut p_chr_1: u32, mut p_out_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlOpp_GetTargetPos", &PROBE_N, &[(p_chr_1) as u32, (p_out_2) as u32], false); }
    let r_ = 'f: {
    { let a0_ = (if ((((((c.m.r32(p_chr_1) as i32)) == (0i32)) as i32)) != 0) { F::BtlChar_Get(c, 1i32) } else { F::BtlChar_Get(c, 0i32) }); F::BtlChar_GetSnapPos(c, a0_, p_out_2, 6i32) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlOpp_GetTargetPos", &PROBE_N, &[], true); }
    r_
}

pub fn BtlOpp_GetSnapPos(c: &mut Ctx, mut p_chr_1: u32, mut p_out_2: u32, mut p_slot_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlOpp_GetSnapPos", &PROBE_N, &[(p_chr_1) as u32, (p_out_2) as u32, (p_slot_3) as u32], false); }
    let r_ = 'f: {
    { let a0_ = (if ((((((c.m.r32(p_chr_1) as i32)) == (0i32)) as i32)) != 0) { F::BtlChar_Get(c, 1i32) } else { F::BtlChar_Get(c, 0i32) }); F::BtlChar_GetSnapPos(c, a0_, p_out_2, p_slot_3) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlOpp_GetSnapPos", &PROBE_N, &[], true); }
    r_
}

pub fn BtlOpp_GetTargetRot(c: &mut Ctx, mut p_chr_1: u32, mut p_out_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlOpp_GetTargetRot", &PROBE_N, &[(p_chr_1) as u32, (p_out_2) as u32], false); }
    let r_ = 'f: {
    { let a0_ = (if ((((((c.m.r32(p_chr_1) as i32)) == (0i32)) as i32)) != 0) { F::BtlChar_Get(c, 1i32) } else { F::BtlChar_Get(c, 0i32) }); F::BtlChar_GetSnapRot(c, a0_, p_out_2, 6i32) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlOpp_GetTargetRot", &PROBE_N, &[], true); }
    r_
}

pub fn BtlOpp_GetSnapRot(c: &mut Ctx, mut p_chr_1: u32, mut p_out_2: u32, mut p_slot_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlOpp_GetSnapRot", &PROBE_N, &[(p_chr_1) as u32, (p_out_2) as u32, (p_slot_3) as u32], false); }
    let r_ = 'f: {
    { let a0_ = (if ((((((c.m.r32(p_chr_1) as i32)) == (0i32)) as i32)) != 0) { F::BtlChar_Get(c, 1i32) } else { F::BtlChar_Get(c, 0i32) }); F::BtlChar_GetSnapRot(c, a0_, p_out_2, p_slot_3) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlOpp_GetSnapRot", &PROBE_N, &[], true); }
    r_
}

pub fn BtlOpp_GetTargetYaw(c: &mut Ctx, mut p_chr_1: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlOpp_GetTargetYaw", &PROBE_N, &[(p_chr_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        { let a0_ = (if ((((((c.m.r32(p_chr_1) as i32)) == (0i32)) as i32)) != 0) { F::BtlChar_Get(c, 1i32) } else { F::BtlChar_Get(c, 0i32) }); F::BtlChar_GetSnapRot(c, a0_, fp, 6i32) };
        break 'f F32(c.m.r32((fp).wrapping_add(4)));
       F32(0)
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlOpp_GetTargetYaw", &PROBE_N, &[], true); }
    r_
}

pub fn BtlOpp_GetDelta(c: &mut Ctx, mut p_chr_1: u32, mut p_out_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlOpp_GetDelta", &PROBE_N, &[(p_chr_1) as u32, (p_out_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        F::BtlOpp_GetTargetPos(c, p_chr_1, fp);
        { let a2_ = F::BtlChar_GetPos(c, p_chr_1); F::Vec4_Sub(c, p_out_2, fp, a2_) };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlOpp_GetDelta", &PROBE_N, &[], true); }
    r_
}

pub fn BtlOpp_GetPoseVec30(c: &mut Ctx, mut p_chr_1: u32, mut p_out_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlOpp_GetPoseVec30", &PROBE_N, &[(p_chr_1) as u32, (p_out_2) as u32], false); }
    let r_ = 'f: {
    { let a1_ = ({ let a0_ = (if ((((((c.m.r32(p_chr_1) as i32)) == (0i32)) as i32)) != 0) { F::BtlChar_Get(c, 1i32) } else { F::BtlChar_Get(c, 0i32) }); F::BtlChar_GetPos(c, a0_) }).wrapping_add(48); F::Vec4_Copy(c, p_out_2, a1_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlOpp_GetPoseVec30", &PROBE_N, &[], true); }
    r_
}

pub fn BtlOpp_GetVelocity(c: &mut Ctx, mut p_chr_1: u32, mut p_out_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlOpp_GetVelocity", &PROBE_N, &[(p_chr_1) as u32, (p_out_2) as u32], false); }
    let r_ = 'f: {
    { let a1_ = ({ let a0_ = (if ((((((c.m.r32(p_chr_1) as i32)) == (0i32)) as i32)) != 0) { F::BtlChar_Get(c, 1i32) } else { F::BtlChar_Get(c, 0i32) }); F::BtlChar_GetPos(c, a0_) }).wrapping_add(64); F::Vec4_Copy(c, p_out_2, a1_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlOpp_GetVelocity", &PROBE_N, &[], true); }
    r_
}

pub fn BtlOpp_GetDistance(c: &mut Ctx, mut p_chr_1: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlOpp_GetDistance", &PROBE_N, &[(p_chr_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        F::BtlOpp_GetDelta(c, p_chr_1, fp);
        break 'f F::Vec3_Length(c, fp);
       F32(0)
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlOpp_GetDistance", &PROBE_N, &[], true); }
    r_
}

pub fn BtlOpp_GetDistanceXZ(c: &mut Ctx, mut p_chr_1: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlOpp_GetDistanceXZ", &PROBE_N, &[(p_chr_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        F::BtlOpp_GetDelta(c, p_chr_1, fp);
        break 'f F::BtlUtil_LengthXZ(c, fp);
       F32(0)
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlOpp_GetDistanceXZ", &PROBE_N, &[], true); }
    r_
}

pub fn BtlOpp_GetGapXZ(c: &mut Ctx, mut p_chr_1: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlOpp_GetGapXZ", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_d_2: F32 = F32(0);
    let r_ = 'f: {
    l_d_2 = F::BtlOpp_GetDistanceXZ(c, p_chr_1);
    { let r_ = { let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); F::BtlCharApi_GetRadius(c, a0_) }; let n_ = rt::fsub(l_d_2, r_); l_d_2 = n_; };
    { let r_ = F::BtlOpp_GetRadius(c, p_chr_1); let n_ = rt::fsub(l_d_2, r_); l_d_2 = n_; };
    break 'f F::BtlUtil_MaxF(c, l_d_2, F32(0x00000000));
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlOpp_GetGapXZ", &PROBE_N, &[], true); }
    r_
}

pub fn BtlOpp_GetYaw(c: &mut Ctx, mut p_chr_1: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlOpp_GetYaw", &PROBE_N, &[(p_chr_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        F::BtlOpp_GetDelta(c, p_chr_1, fp);
        if (((((((rt::flt({ let a0_ = F32(c.m.r32(fp)); rt::ffabs(a0_) }, F32(0x38d1b717)) as i32)) != 0) && (((rt::flt({ let a0_ = F32(c.m.r32((fp).wrapping_add(8))); rt::ffabs(a0_) }, F32(0x38d1b717)) as i32)) != 0)) as i32)) != 0) {
            break 'f F32(0x00000000);
        }
        break 'f { let a0_ = F32(c.m.r32(fp)); let a1_ = F32(c.m.r32((fp).wrapping_add(8))); F::Port_atan2f(c, a0_, a1_) };
       F32(0)
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlOpp_GetYaw", &PROBE_N, &[], true); }
    r_
}

pub fn BtlOpp_GetYawFromFacing(c: &mut Ctx, mut p_chr_1: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlOpp_GetYawFromFacing", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_yaw_2: F32 = F32(0);
    let mut l_pose_3: u32 = 0;
    let r_ = 'f: {
    l_yaw_2 = F::BtlOpp_GetYaw(c, p_chr_1);
    l_pose_3 = F::BtlChar_GetPos(c, p_chr_1);
    break 'f { let a0_ = rt::fsub(l_yaw_2, { let a0_ = rt::fadd(F32(c.m.r32(((l_pose_3).wrapping_add(16)).wrapping_add(4))), { let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(100); F32(c.m.r32(a_)) }); F::BtlUtil_WrapAngle(c, a0_) }); F::BtlUtil_WrapAngle(c, a0_) };
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlOpp_GetYawFromFacing", &PROBE_N, &[], true); }
    r_
}

pub fn BtlOpp_GetYawFromItsFacing(c: &mut Ctx, mut p_chr_1: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlOpp_GetYawFromItsFacing", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_yaw_2: F32 = F32(0);
    let r_ = 'f: {
    l_yaw_2 = F::BtlOpp_GetYaw(c, p_chr_1);
    break 'f { let a0_ = rt::fsub(l_yaw_2, F::BtlOpp_GetTargetYaw(c, p_chr_1)); F::BtlUtil_WrapAngle(c, a0_) };
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlOpp_GetYawFromItsFacing", &PROBE_N, &[], true); }
    r_
}

pub fn BtlOpp_GetPitch(c: &mut Ctx, mut p_chr_1: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlOpp_GetPitch", &PROBE_N, &[(p_chr_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_len_2: F32 = F32(0);
    let mut l_pitch_3: F32 = F32(0);
    let r_ = 'f: {
        F::BtlOpp_GetDelta(c, p_chr_1, fp);
        l_len_2 = F::Vec3_Length(c, fp);
        l_pitch_3 = F32(0x00000000);
        if (((rt::flt(F32(0x3a83126e), l_len_2) as i32)) != 0) {
            F::Vec4_Scale(c, fp, fp, rt::fdiv(F32(0x3f800000), l_len_2));
            l_pitch_3 = rt::fneg({ let a0_ = F32(c.m.r32((fp).wrapping_add(4))); F::Mathf_Asin(c, a0_) });
        }
        break 'f l_pitch_3;
       F32(0)
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlOpp_GetPitch", &PROBE_N, &[], true); }
    r_
}

pub fn BtlOpp_GetPitchAdjusted(c: &mut Ctx, mut p_chr_1: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlOpp_GetPitchAdjusted", &PROBE_N, &[(p_chr_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_len_2: F32 = F32(0);
    let mut l_pitch_3: F32 = F32(0);
    let r_ = 'f: {
        F::BtlOpp_GetDelta(c, p_chr_1, fp);
        { let r_ = F::BtlOpp_GetHalfHeightDiff(c, p_chr_1); let a_ = (fp).wrapping_add(4); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        l_len_2 = F::Vec3_Length(c, fp);
        l_pitch_3 = F32(0x00000000);
        if (((rt::flt(F32(0x3a83126e), l_len_2) as i32)) != 0) {
            F::Vec4_Scale(c, fp, fp, rt::fdiv(F32(0x3f800000), l_len_2));
            l_pitch_3 = rt::fneg({ let a0_ = F32(c.m.r32((fp).wrapping_add(4))); F::Mathf_Asin(c, a0_) });
        }
        break 'f l_pitch_3;
       F32(0)
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlOpp_GetPitchAdjusted", &PROBE_N, &[], true); }
    r_
}

pub fn BtlOpp_GetHalfHeightDiff(c: &mut Ctx, mut p_chr_1: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlOpp_GetHalfHeightDiff", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_opp_2: u32 = 0;
    let mut l_own_3: F32 = F32(0);
    let r_ = 'f: {
    l_opp_2 = (if ((((((c.m.r32(p_chr_1) as i32)) == (0i32)) as i32)) != 0) { F::BtlChar_Get(c, 1i32) } else { F::BtlChar_Get(c, 0i32) });
    l_own_3 = { let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); F::BtlCharApi_GetHeight(c, a0_) };
    break 'f rt::fsub(rt::fmul(l_own_3, F32(0x3f000000)), rt::fmul({ let a0_ = (c.m.r32((l_opp_2).wrapping_add(12)) as i32); F::BtlCharApi_GetHeight(c, a0_) }, F32(0x3f000000)));
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlOpp_GetHalfHeightDiff", &PROBE_N, &[], true); }
    r_
}

pub fn BtlOpp_GetRadius(c: &mut Ctx, mut p_chr_1: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlOpp_GetRadius", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f { let a0_ = { let a_ = ((if ((((((c.m.r32(p_chr_1) as i32)) == (0i32)) as i32)) != 0) { F::BtlChar_Get(c, 1i32) } else { F::BtlChar_Get(c, 0i32) })).wrapping_add(12); (c.m.r32(a_) as i32) }; F::BtlCharApi_GetRadius(c, a0_) };
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlOpp_GetRadius", &PROBE_N, &[], true); }
    r_
}

pub fn BtlOpp_GetParamByte2(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlOpp_GetParamByte2", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f { let a0_ = (if ((((((c.m.r32(p_chr_1) as i32)) == (0i32)) as i32)) != 0) { F::BtlChar_Get(c, 1i32) } else { F::BtlChar_Get(c, 0i32) }); F::BtlParam_GetUnk2(c, a0_) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlOpp_GetParamByte2", &PROBE_N, &[], true); }
    r_
}

pub fn BtlOpp_GetHeight(c: &mut Ctx, mut p_chr_1: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlOpp_GetHeight", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f { let a0_ = { let a_ = ((if ((((((c.m.r32(p_chr_1) as i32)) == (0i32)) as i32)) != 0) { F::BtlChar_Get(c, 1i32) } else { F::BtlChar_Get(c, 0i32) })).wrapping_add(12); (c.m.r32(a_) as i32) }; F::BtlCharApi_GetHeight(c, a0_) };
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlOpp_GetHeight", &PROBE_N, &[], true); }
    r_
}

pub fn BtlOpp_GetObjUnkFFC(c: &mut Ctx, mut p_chr_1: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlOpp_GetObjUnkFFC", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f { let a0_ = { let a_ = ((if ((((((c.m.r32(p_chr_1) as i32)) == (0i32)) as i32)) != 0) { F::BtlChar_Get(c, 1i32) } else { F::BtlChar_Get(c, 0i32) })).wrapping_add(12); (c.m.r32(a_) as i32) }; F::BtlCharApi_GetCenterHeight(c, a0_) };
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlOpp_GetObjUnkFFC", &PROBE_N, &[], true); }
    r_
}

pub fn BtlOpp_GetParamWord0(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlOpp_GetParamWord0", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f { let a0_ = (if ((((((c.m.r32(p_chr_1) as i32)) == (0i32)) as i32)) != 0) { F::BtlChar_Get(c, 1i32) } else { F::BtlChar_Get(c, 0i32) }); F::BtlParam_GetUnk0(c, a0_) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlOpp_GetParamWord0", &PROBE_N, &[], true); }
    r_
}

pub fn BtlOpp_GetPlayer(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlOpp_GetPlayer", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f { let a_ = (if ((((((c.m.r32(p_chr_1) as i32)) == (0i32)) as i32)) != 0) { F::BtlChar_Get(c, 1i32) } else { F::BtlChar_Get(c, 0i32) }); (c.m.r32(a_) as i32) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlOpp_GetPlayer", &PROBE_N, &[], true); }
    r_
}

pub fn BtlOpp_GetObjId(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlOpp_GetObjId", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f { let a_ = ((if ((((((c.m.r32(p_chr_1) as i32)) == (0i32)) as i32)) != 0) { F::BtlChar_Get(c, 1i32) } else { F::BtlChar_Get(c, 0i32) })).wrapping_add(12); (c.m.r32(a_) as i32) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlOpp_GetObjId", &PROBE_N, &[], true); }
    r_
}

pub fn BtlOpp_GetObj(c: &mut Ctx, mut p_chr_1: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlOpp_GetObj", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f { let a0_ = (if ((((((c.m.r32(p_chr_1) as i32)) == (0i32)) as i32)) != 0) { F::BtlChar_Get(c, 1i32) } else { F::BtlChar_Get(c, 0i32) }); F::BtlChar_GetObj(c, a0_) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlOpp_GetObj", &PROBE_N, &[], true); }
    r_
}

pub fn BtlOpp_GetSeenAction(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlOpp_GetSeenAction", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_opp_2: u32 = 0;
    let r_ = 'f: {
    l_opp_2 = (if ((((((c.m.r32(p_chr_1) as i32)) == (0i32)) as i32)) != 0) { F::BtlChar_Get(c, 1i32) } else { F::BtlChar_Get(c, 0i32) });
    if (((((((((F::BtlChar_GetStage(c, p_chr_1)) < (((6i32) as u32))) as i32)) != 0) || (((((F::BtlChar_GetStage(c, l_opp_2)) < (((6i32) as u32))) as i32)) != 0)) as i32)) != 0) {
        break 'f (c.m.r32((l_opp_2).wrapping_add(2384)) as i32);
    }
    break 'f F::BtlAct_GetCurrent(c, l_opp_2);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlOpp_GetSeenAction", &PROBE_N, &[], true); }
    r_
}

pub fn BtlOpp_GetSeenUnk964(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlOpp_GetSeenUnk964", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_opp_2: u32 = 0;
    let r_ = 'f: {
    l_opp_2 = (if ((((((c.m.r32(p_chr_1) as i32)) == (0i32)) as i32)) != 0) { F::BtlChar_Get(c, 1i32) } else { F::BtlChar_Get(c, 0i32) });
    if (((((((((F::BtlChar_GetStage(c, p_chr_1)) < (((6i32) as u32))) as i32)) != 0) || (((((F::BtlChar_GetStage(c, l_opp_2)) < (((6i32) as u32))) as i32)) != 0)) as i32)) != 0) {
        break 'f (c.m.r32((l_opp_2).wrapping_add(2408)) as i32);
    }
    break 'f (c.m.r32((l_opp_2).wrapping_add(2404)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlOpp_GetSeenUnk964", &PROBE_N, &[], true); }
    r_
}

pub fn BtlOpp_GetObjUnkFF8(c: &mut Ctx, mut p_chr_1: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlOpp_GetObjUnkFF8", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f { let a0_ = { let a_ = ((if ((((((c.m.r32(p_chr_1) as i32)) == (0i32)) as i32)) != 0) { F::BtlChar_Get(c, 1i32) } else { F::BtlChar_Get(c, 0i32) })).wrapping_add(12); (c.m.r32(a_) as i32) }; F::BtlCharApi_GetBodyUnkFF8(c, a0_) };
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlOpp_GetObjUnkFF8", &PROBE_N, &[], true); }
    r_
}

pub fn BtlOpp_GetUnk15A0(c: &mut Ctx, mut p_chr_1: u32, mut p_out_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlOpp_GetUnk15A0", &PROBE_N, &[(p_chr_1) as u32, (p_out_2) as u32], false); }
    let mut l_opp_3: u32 = 0;
    let r_ = 'f: {
    l_opp_3 = (if ((((((c.m.r32(p_chr_1) as i32)) == (0i32)) as i32)) != 0) { F::BtlChar_Get(c, 1i32) } else { F::BtlChar_Get(c, 0i32) });
    F::Vec4_Copy(c, p_out_2, (l_opp_3).wrapping_add(5536));
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlOpp_GetUnk15A0", &PROBE_N, &[], true); }
    r_
}

pub fn BtlOpp_GetNodePos(c: &mut Ctx, mut p_chr_1: u32, mut p_node_2: i32, mut p_out_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlOpp_GetNodePos", &PROBE_N, &[(p_chr_1) as u32, (p_node_2) as u32, (p_out_3) as u32], false); }
    let r_ = 'f: {
    { let a0_ = { let a_ = ((if ((((((c.m.r32(p_chr_1) as i32)) == (0i32)) as i32)) != 0) { F::BtlChar_Get(c, 1i32) } else { F::BtlChar_Get(c, 0i32) })).wrapping_add(12); (c.m.r32(a_) as i32) }; F::BtlCharApi_GetNodePos(c, a0_, p_node_2, p_out_3) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlOpp_GetNodePos", &PROBE_N, &[], true); }
    r_
}

pub fn BtlOpp_GetObjVecFA0(c: &mut Ctx, mut p_chr_1: u32, mut p_out_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlOpp_GetObjVecFA0", &PROBE_N, &[(p_chr_1) as u32, (p_out_2) as u32], false); }
    let r_ = 'f: {
    { let a1_ = { let a_ = ({ let a0_ = { let a_ = ((if ((((((c.m.r32(p_chr_1) as i32)) == (0i32)) as i32)) != 0) { F::BtlChar_Get(c, 1i32) } else { F::BtlChar_Get(c, 0i32) })).wrapping_add(12); (c.m.r32(a_) as i32) }; F::BtlObj_Get(c, a0_) }).wrapping_add(((4000i32) as u32)); c.m.r32(a_) }; F::Vec4_Copy(c, p_out_2, a1_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlOpp_GetObjVecFA0", &PROBE_N, &[], true); }
    r_
}

pub fn BtlOpp_GetUnkD88(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlOpp_GetUnkD88", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f { let a_ = ((if ((((((c.m.r32(p_chr_1) as i32)) == (0i32)) as i32)) != 0) { F::BtlChar_Get(c, 1i32) } else { F::BtlChar_Get(c, 0i32) })).wrapping_add(3464); (c.m.r32(a_) as i32) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlOpp_GetUnkD88", &PROBE_N, &[], true); }
    r_
}

pub fn BtlOpp_GetClosingTime(c: &mut Ctx, mut p_chr_1: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlOpp_GetClosingTime", &PROBE_N, &[(p_chr_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(96) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 96);
    let mut l_dist_2: F32 = F32(0);
    let mut l_speed_3: F32 = F32(0);
    let mut l_rel_4: F32 = F32(0);
    let mut l_eps_5: F32 = F32(0);
    let r_ = 'f: {
        { let a1_ = F::BtlChar_GetPos(c, p_chr_1); F::Vec4_Copy(c, fp, a1_) };
        l_eps_5 = F32(0x3a83126e);
        F::BtlOpp_GetTargetPos(c, p_chr_1, fp.wrapping_add(32));
        F::Vec4_Sub(c, fp.wrapping_add(64), fp.wrapping_add(32), fp);
        l_dist_2 = F::Vec3_Length(c, fp.wrapping_add(64));
        if (((rt::flt(l_dist_2, l_eps_5) as i32)) != 0) {
            break 'f F32(0x00000000);
        }
        F::Vec4_Scale(c, fp.wrapping_add(80), fp.wrapping_add(64), rt::fdiv(F32(0x3f800000), l_dist_2));
        { let a1_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(64); F::Vec4_Copy(c, fp.wrapping_add(16), a1_) };
        F::BtlOpp_GetVelocity(c, p_chr_1, fp.wrapping_add(48));
        l_speed_3 = F::Vec3_Dot(c, fp.wrapping_add(16), fp.wrapping_add(80));
        if (((rt::flt(l_speed_3, F32(0x00000000)) as i32)) != 0) {
            break 'f F32(0x4b189680);
        }
        l_rel_4 = rt::fsub(l_speed_3, F::Vec3_Dot(c, fp.wrapping_add(48), fp.wrapping_add(80)));
        if (((rt::flt(l_rel_4, l_eps_5) as i32)) != 0) {
            break 'f F32(0x4b189680);
        }
        break 'f rt::fdiv(l_dist_2, l_rel_4);
       F32(0)
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlOpp_GetClosingTime", &PROBE_N, &[], true); }
    r_
}

pub fn BtlOpp_HasAbility(c: &mut Ctx, mut p_chr_1: u32, mut p_arg_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlOpp_HasAbility", &PROBE_N, &[(p_chr_1) as u32, (p_arg_2) as u32], false); }
    let r_ = 'f: {
    break 'f { let a0_ = (if ((((((c.m.r32(p_chr_1) as i32)) == (0i32)) as i32)) != 0) { F::BtlChar_Get(c, 1i32) } else { F::BtlChar_Get(c, 0i32) }); F::BtlMember_HasAbility(c, a0_, p_arg_2) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlOpp_HasAbility", &PROBE_N, &[], true); }
    r_
}

pub fn BtlOpp_GetUnk20F318(c: &mut Ctx, mut p_chr_1: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlOpp_GetUnk20F318", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f { let a0_ = (if ((((((c.m.r32(p_chr_1) as i32)) == (0i32)) as i32)) != 0) { F::BtlChar_Get(c, 1i32) } else { F::BtlChar_Get(c, 0i32) }); F::BtlParam_GetUnkC7Scale(c, a0_) };
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlOpp_GetUnk20F318", &PROBE_N, &[], true); }
    r_
}

pub fn BtlOpp_GetClashCountB(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlOpp_GetClashCountB", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f { let a_ = ((if ((((((c.m.r32(p_chr_1) as i32)) == (0i32)) as i32)) != 0) { F::BtlChar_Get(c, 1i32) } else { F::BtlChar_Get(c, 0i32) })).wrapping_add(3664); (c.m.r32(a_) as i32) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlOpp_GetClashCountB", &PROBE_N, &[], true); }
    r_
}

pub fn BtlOpp_MirrorFlags(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlOpp_MirrorFlags", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_opp_2: u32 = 0;
    let r_ = 'f: {
    if ((((((c.m.r32(p_chr_1) as i32)) == (0i32)) as i32)) != 0) {
        l_opp_2 = F::BtlChar_Get(c, 1i32);
    } else {
        l_opp_2 = F::BtlChar_Get(c, 0i32);
    }
    F::BtlChar_ClearFlagRange(c, p_chr_1, ((172i32) as u32), ((185i32) as u32));
    if ((F::BtlChar_TestFlag(c, l_opp_2, ((11i32) as u32))) != 0) {
        F::BtlChar_SetHeldFlag(c, p_chr_1, ((172i32) as u32));
    }
    if ((F::BtlChar_TestFlag(c, l_opp_2, ((26i32) as u32))) != 0) {
        F::BtlChar_SetHeldFlag(c, p_chr_1, ((173i32) as u32));
    }
    if ((F::BtlChar_TestFlag(c, l_opp_2, ((63i32) as u32))) != 0) {
        F::BtlChar_SetHeldFlag(c, p_chr_1, ((174i32) as u32));
    }
    if ((F::BtlChar_TestFlag(c, l_opp_2, ((133i32) as u32))) != 0) {
        F::BtlChar_SetHeldFlag(c, p_chr_1, ((179i32) as u32));
    }
    if (({ let a0_ = F::BtlAct_GetCurrent(c, l_opp_2); F::BtlAct_IsTechniqueId(c, a0_) }) != 0) {
        F::BtlChar_SetHeldFlag(c, p_chr_1, ((175i32) as u32));
    }
    if (({ let a0_ = (c.m.r32((l_opp_2).wrapping_add(12)) as i32); F::BtlCharApi_IsInSkill(c, a0_) }) != 0) {
        F::BtlChar_SetHeldFlag(c, p_chr_1, ((176i32) as u32));
    }
    if ((F::BtlChar_IsDead(c, l_opp_2)) != 0) {
        F::BtlChar_SetHeldFlag(c, p_chr_1, ((177i32) as u32));
    }
    if ((F::BtlChar_TestFlag(c, l_opp_2, ((204i32) as u32))) != 0) {
        F::BtlChar_SetHeldFlag(c, p_chr_1, ((180i32) as u32));
    }
    if ((F::BtlChar_TestFlag(c, l_opp_2, ((38i32) as u32))) != 0) {
        F::BtlChar_SetHeldFlag(c, p_chr_1, ((178i32) as u32));
    }
    if ((F::BtlChar_TestFlag(c, l_opp_2, ((39i32) as u32))) != 0) {
        F::ChrCam_EndCut(c, p_chr_1);
    }
    if ((F::BtlChar_TestFlag(c, l_opp_2, ((144i32) as u32))) != 0) {
        F::BtlChar_SetFlag(c, p_chr_1, ((181i32) as u32));
    }
    if ((F::BtlChar_TestFlag(c, l_opp_2, ((22i32) as u32))) != 0) {
        F::BtlChar_SetFlag(c, p_chr_1, ((182i32) as u32));
    }
    if ((F::BtlChar_TestFlag(c, l_opp_2, ((23i32) as u32))) != 0) {
        F::BtlChar_SetFlag(c, p_chr_1, ((183i32) as u32));
    }
    if ((F::BtlChar_TestFlag(c, l_opp_2, ((40i32) as u32))) != 0) {
        F::BtlChar_SetFlag(c, p_chr_1, ((214i32) as u32));
    }
    if ((F::BtlChar_TestFlag(c, l_opp_2, ((220i32) as u32))) != 0) {
        F::BtlChar_SetFlag(c, p_chr_1, ((184i32) as u32));
    }
    if ((F::BtlChar_TestFlag(c, l_opp_2, ((146i32) as u32))) != 0) {
        F::BtlChar_SetFlag(c, p_chr_1, ((185i32) as u32));
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlOpp_MirrorFlags", &PROBE_N, &[], true); }
    r_
}
