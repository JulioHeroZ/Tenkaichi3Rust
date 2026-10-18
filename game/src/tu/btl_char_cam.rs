#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn ChrCam_GetOffset(c: &mut Ctx, mut p_chr_1: u32, mut p_out_2: u32, mut p_mode_3: i32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ChrCam_GetOffset", &PROBE_N, &[(p_chr_1) as u32, (p_out_2) as u32, (p_mode_3) as u32], false); }
    let mut l_scale_4: F32 = F32(0);
    let mut l_dist_5: F32 = F32(0);
    let mut l_pitch_6: F32 = F32(0);
    let r_ = 'f: {
    l_scale_4 = { let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); F::BtlCharApi_GetHeight(c, a0_) };
    l_pitch_6 = F32(0x00000000);
    l_dist_5 = rt::fadd(rt::fmul(l_scale_4, F32(0x3f000000)), F32(0x3f800000));
    if (((rt::flt(l_dist_5, F32(0x41200000)) as i32)) != 0) {
        l_dist_5 = F32(0x41200000);
    }
    's1: {
        let sv7: i32 = p_mode_3;
        match sv7 {
            1i32 => {
                { let v_ = F32(0x00000000); let a_ = p_out_2; c.m.w32(a_, (v_).0); };
                { let v_ = rt::fmul(rt::fneg(l_dist_5), F32(0x3fc00000)); let a_ = (p_out_2).wrapping_add(4); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fmul(rt::fneg(l_dist_5), F32(0x40333333)); let a_ = (p_out_2).wrapping_add(8); c.m.w32(a_, (v_).0); };
                l_pitch_6 = rt::fadd(rt::fmul(l_scale_4, F32(0xba83126e)), F32(0xbd75c28f));
                break 's1;
            }
            2i32 => {
                { let v_ = F32(0x00000000); let a_ = p_out_2; c.m.w32(a_, (v_).0); };
                { let v_ = rt::fmul(rt::fneg(l_dist_5), F32(0x40000000)); let a_ = (p_out_2).wrapping_add(4); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fmul(rt::fneg(l_dist_5), F32(0x40533333)); let a_ = (p_out_2).wrapping_add(8); c.m.w32(a_, (v_).0); };
                l_pitch_6 = rt::fadd(rt::fmul(l_scale_4, F32(0xba83126e)), F32(0xbdcccccc));
                break 's1;
            }
            0i32 => {
                { let v_ = F32(0x00000000); let a_ = p_out_2; c.m.w32(a_, (v_).0); };
                { let v_ = rt::fmul(rt::fneg(l_dist_5), F32(0x3f999999)); let a_ = (p_out_2).wrapping_add(4); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fmul(rt::fneg(l_dist_5), F32(0x40199999)); let a_ = (p_out_2).wrapping_add(8); c.m.w32(a_, (v_).0); };
                l_pitch_6 = rt::fadd(rt::fmul(l_scale_4, F32(0xba83126e)), F32(0xbd4ccccc));
                break 's1;
            }
            _ => {
            }
        }
    }
    { let v_ = F32(0x3f800000); let a_ = (p_out_2).wrapping_add(12); c.m.w32(a_, (v_).0); };
    break 'f l_pitch_6;
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "ChrCam_GetOffset", &PROBE_N, &[], true); }
    r_
}

pub fn ChrCam_CalcRate(c: &mut Ctx, mut p_chr_1: u32, mut p_eye_2: u32, mut p_target_3: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ChrCam_CalcRate", &PROBE_N, &[(p_chr_1) as u32, (p_eye_2) as u32, (p_target_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_len_4: F32 = F32(0);
    let mut l_rate_5: F32 = F32(0);
    let r_ = 'f: {
        F::Vec4_Sub(c, fp, p_target_3, p_eye_2);
        l_len_4 = F::Vec3_Length(c, fp);
        l_rate_5 = rt::fadd(rt::fmul(rt::fdiv(l_len_4, { let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); F::BtlCharApi_GetHeight(c, a0_) }), F32(0x3e4ccccc)), F32(0x3e4ccccc));
        if (((rt::flt(F32(0x3f800000), l_rate_5) as i32)) != 0) {
            l_rate_5 = F32(0x3f800000);
        }
        break 'f l_rate_5;
       F32(0)
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "ChrCam_CalcRate", &PROBE_N, &[], true); }
    r_
}

pub fn ChrCam_GetRate(c: &mut Ctx, mut p_chr_1: u32, mut p_eye_2: u32, mut p_target_3: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ChrCam_GetRate", &PROBE_N, &[(p_chr_1) as u32, (p_eye_2) as u32, (p_target_3) as u32], false); }
    let mut l_cam_4: u32 = 0;
    let r_ = 'f: {
    l_cam_4 = (p_chr_1).wrapping_add(1056);
    { let v_ = { let a0_ = F32(c.m.r32((l_cam_4).wrapping_add(148))); let a1_ = F::ChrCam_CalcRate(c, p_chr_1, p_eye_2, p_target_3); F::BtlUtil_ApproachF(c, a0_, a1_, F32(0x3c5a740c)) }; let a_ = (l_cam_4).wrapping_add(148); c.m.w32(a_, (v_).0); };
    if ((F::BtlChar_TestFlag(c, p_chr_1, ((((207i32) as i64 as u64)) as u32))) != 0) {
        { let v_ = F32(0x3e999999); let a_ = (l_cam_4).wrapping_add(148); c.m.w32(a_, (v_).0); };
    } else {
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((208i32) as i64 as u64)) as u32))) != 0) {
            { let v_ = F32(0x3e800000); let a_ = (l_cam_4).wrapping_add(148); c.m.w32(a_, (v_).0); };
        } else {
            if ((F::BtlChar_TestFlag(c, p_chr_1, ((((209i32) as i64 as u64)) as u32))) != 0) {
                { let v_ = F32(0x3e4ccccc); let a_ = (l_cam_4).wrapping_add(148); c.m.w32(a_, (v_).0); };
            } else {
                if ((F::BtlChars_IsTimeStopped(c)) != 0) {
                    { let v_ = F32(0x3f800000); let a_ = (l_cam_4).wrapping_add(148); c.m.w32(a_, (v_).0); };
                }
            }
        }
    }
    break 'f F32(c.m.r32((l_cam_4).wrapping_add(148)));
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "ChrCam_GetRate", &PROBE_N, &[], true); }
    r_
}

pub fn ChrCam_GetBob(c: &mut Ctx, mut p_chr_1: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ChrCam_GetBob", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_cam_2: u32 = 0;
    let r_ = 'f: {
    l_cam_2 = (p_chr_1).wrapping_add(1056);
    if ((((((F::BtlChars_IsTimeStopped(c)) != 0) || ((((((c.m.r32((l_cam_2).wrapping_add(152)) as i32)) == (0i32)) as i32)) != 0)) as i32)) != 0) {
        { let v_ = F32(0x00000000); let a_ = (l_cam_2).wrapping_add(144); c.m.w32(a_, (v_).0); };
        break 'f F32(c.m.r32((l_cam_2).wrapping_add(144)));
    }
    if ((F::BtlChar_TestFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32))) != 0) {
        { let v_ = { let a0_ = rt::fadd(F32(c.m.r32((l_cam_2).wrapping_add(144))), F32(0x3d2b92a5)); F::BtlUtil_WrapAngle(c, a0_) }; let a_ = (l_cam_2).wrapping_add(144); c.m.w32(a_, (v_).0); };
    } else {
        if (((rt::flt(F32(0x3fc90fda), F32(c.m.r32((l_cam_2).wrapping_add(144)))) as i32)) != 0) {
            { let v_ = rt::fsub(F32(0x40490fda), F32(c.m.r32((l_cam_2).wrapping_add(144)))); let a_ = (l_cam_2).wrapping_add(144); c.m.w32(a_, (v_).0); };
        }
        if (((rt::flt(F32(c.m.r32((l_cam_2).wrapping_add(144))), F32(0xbfc90fda)) as i32)) != 0) {
            { let v_ = rt::fsub(F32(0xc0490fda), F32(c.m.r32((l_cam_2).wrapping_add(144)))); let a_ = (l_cam_2).wrapping_add(144); c.m.w32(a_, (v_).0); };
        }
        { let v_ = { let a0_ = F32(c.m.r32((l_cam_2).wrapping_add(144))); F::BtlUtil_ApproachF(c, a0_, F32(0x00000000), F32(0x3dd6774e)) }; let a_ = (l_cam_2).wrapping_add(144); c.m.w32(a_, (v_).0); };
    }
    break 'f rt::fmul({ let a0_ = F32(c.m.r32((l_cam_2).wrapping_add(144))); F::Mathf_Sin(c, a0_) }, F32(0x3fc00000));
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "ChrCam_GetBob", &PROBE_N, &[], true); }
    r_
}
