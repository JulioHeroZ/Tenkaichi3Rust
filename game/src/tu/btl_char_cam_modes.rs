#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn ChrCam_CalcFixed(c: &mut Ctx, mut p_chr_1: u32, mut p_eye_2: u32, mut p_rot_3: u32, mut p_target_4: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ChrCam_CalcFixed", &PROBE_N, &[(p_chr_1) as u32, (p_eye_2) as u32, (p_rot_3) as u32, (p_target_4) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(128) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 128);
    let mut l_cam_5: u32 = 0;
    let mut l_scale_6: F32 = F32(0);
    let mut l_len_7: F32 = F32(0);
    let mut l_a_8: F32 = F32(0);
    let mut l_b_9: F32 = F32(0);
    let mut l_horiz_10: F32 = F32(0);
    let mut l_yaw_11: F32 = F32(0);
    let mut l_camYaw_12: F32 = F32(0);
    let mut l_sign_13: F32 = F32(0);
    let mut l_s_14: F32 = F32(0);
    let mut l_c_15: F32 = F32(0);
    let mut l_t_16: F32 = F32(0);
    let mut l_y_17: F32 = F32(0);
    let mut l_maxPitch_18: F32 = F32(0);
    let mut l_pitch_19: F32 = F32(0);
    let mut l_k_20: F32 = F32(0);
    let mut l_base_21: F32 = F32(0);
    let r_ = 'f: {
        l_cam_5 = (p_chr_1).wrapping_add(1056);
        l_scale_6 = { let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); F::BtlCharApi_GetHeight(c, a0_) };
        { let a1_ = F::BtlChar_GetPos(c, p_chr_1); F::Vec4_Copy(c, fp, a1_) };
        F::BtlOpp_GetTargetPos(c, p_chr_1, fp.wrapping_add(16));
        F::Vec4_Sub(c, fp.wrapping_add(32), fp.wrapping_add(16), fp);
        { let r_ = F::BtlOpp_GetHalfHeightDiff(c, p_chr_1); let a_ = (fp.wrapping_add(32)).wrapping_add(4); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        l_len_7 = F::Vec3_Length(c, fp.wrapping_add(32));
        if (((rt::flt(l_len_7, F32(0x3c23d70a)) as i32)) != 0) {
            F::Vec4_Set(c, fp.wrapping_add(48), F32(0x00000000), F32(0x00000000), F32(0x3f800000), F32(0x00000000));
        } else {
            F::Vec4_Scale(c, fp.wrapping_add(48), fp.wrapping_add(32), rt::fdiv(F32(0x3f800000), l_len_7));
        }
        l_y_17 = rt::fneg(rt::fmul(l_scale_6, F32(0x3f333333)));
        l_maxPitch_18 = F32(0x3f8cbe4b);
        l_k_20 = F32(0x3ecccccc);
        l_base_21 = F32(0xbdcccccc);
        F::Vec4_Set(c, p_target_4, F32(0x00000000), l_y_17, F32(0x00000000), F32(0x3f800000));
        { let v_ = F32(0x00000000); let a_ = fp.wrapping_add(64); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(rt::fmul(rt::fneg({ let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); F::BtlCharApi_GetHeight(c, a0_) }), l_k_20), F32(0x41200000)); let a_ = (fp.wrapping_add(64)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0xc20c0000); let a_ = (fp.wrapping_add(64)).wrapping_add(8); c.m.w32(a_, (v_).0); };
        F::Vec4_Sub(c, fp.wrapping_add(64), fp.wrapping_add(64), p_target_4);
        l_yaw_11 = F32(c.m.r32((l_cam_5).wrapping_add(128)));
        l_camYaw_12 = l_yaw_11;
        if (((rt::flt(F32(0x3f800000), F32(c.m.r32((fp.wrapping_add(48)).wrapping_add(4)))) as i32)) != 0) {
            { let v_ = F32(0x3f800000); let a_ = (fp.wrapping_add(48)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        }
        if (((rt::flt(F32(c.m.r32((fp.wrapping_add(48)).wrapping_add(4))), F32(0xbf800000)) as i32)) != 0) {
            { let v_ = F32(0xbf800000); let a_ = (fp.wrapping_add(48)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        }
        l_a_8 = rt::fdiv(rt::fneg({ let a0_ = F32(c.m.r32((fp.wrapping_add(48)).wrapping_add(4))); F::Mathf_Asin(c, a0_) }), l_maxPitch_18);
        if (((rt::flt(F32(0x3f800000), l_a_8) as i32)) != 0) {
            l_a_8 = F32(0x3f800000);
        }
        if (((rt::flt(l_a_8, F32(0xbf800000)) as i32)) != 0) {
            l_a_8 = F32(0xbf800000);
        }
        l_b_9 = l_a_8;
        if (((rt::flt(l_b_9, F32(0x00000000)) as i32)) != 0) {
            l_b_9 = F32(0x00000000);
        }
        l_b_9 = rt::fsub(rt::fmul(l_b_9, F32(0x40000000)), F32(0x3f800000));
        l_a_8 = rt::fadd(rt::fmul(rt::fneg(l_a_8), rt::ffabs(l_a_8)), rt::fmul(l_a_8, F32(0x40000000)));
        l_b_9 = rt::fadd(rt::fmul(rt::fneg(l_b_9), rt::ffabs(l_b_9)), rt::fmul(l_b_9, F32(0x40000000)));
        l_pitch_19 = rt::fmul(rt::fmul(l_a_8, l_maxPitch_18), l_k_20);
        l_b_9 = rt::fadd(rt::fmul(l_b_9, F32(0x3f000000)), F32(0x3f000000));
        F::Vec4_Set(c, fp.wrapping_add(96), l_pitch_19, F32(0x00000000), F32(0x00000000), F32(0x00000000));
        { let v_ = F32(0x00000000); let a_ = (fp.wrapping_add(64)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        F::Vec4_RotateEuler(c, fp.wrapping_add(64), fp.wrapping_add(96), fp.wrapping_add(64));
        F::Vec4_Scale(c, fp.wrapping_add(64), fp.wrapping_add(64), rt::fsub(F32(0x3f800000), rt::fmul(l_b_9, F32(0x3e999999))));
        l_base_21 = rt::fadd(rt::fsub(l_pitch_19, rt::fmul(l_b_9, l_base_21)), l_base_21);
        l_horiz_10 = F::BtlUtil_LengthXZ(c, fp.wrapping_add(32));
        l_sign_13 = F32(0x3f800000);
        if (((rt::flt(F32(c.m.r32((l_cam_5).wrapping_add(136))), F32(0x00000000)) as i32)) != 0) {
            l_sign_13 = F32(0xbf800000);
        }
        F::Vec4_Set(c, fp.wrapping_add(112), F32(0x00000000), rt::fmul(l_sign_13, F32(0x3fa0d97a)), F32(0x00000000), F32(0x00000000));
        { let v_ = F32(0x00000000); let a_ = (fp.wrapping_add(64)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        F::Vec4_RotateEuler(c, fp.wrapping_add(64), fp.wrapping_add(112), fp.wrapping_add(64));
        l_t_16 = { let a0_ = rt::fneg(F32(c.m.r32(fp.wrapping_add(64)))); let a1_ = rt::fsub(rt::fmul(l_horiz_10, F32(0x3f000000)), F32(c.m.r32((fp.wrapping_add(64)).wrapping_add(8)))); F::Port_atan2f(c, a0_, a1_) };
        { let v_ = l_t_16; let a_ = (l_cam_5).wrapping_add(140); c.m.w32(a_, (v_).0); };
        l_yaw_11 = F::BtlUtil_WrapAngle(c, rt::fsub(l_yaw_11, l_t_16));
        { let r_ = F32(c.m.r32(fp)); let a_ = p_target_4; let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        { let r_ = F32(c.m.r32((fp).wrapping_add(4))); let a_ = (p_target_4).wrapping_add(4); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        { let r_ = F32(c.m.r32((fp).wrapping_add(8))); let a_ = (p_target_4).wrapping_add(8); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        { let v_ = F32(0x3f800000); let a_ = (p_target_4).wrapping_add(12); c.m.w32(a_, (v_).0); };
        l_s_14 = F::Mathf_Sin(c, l_camYaw_12);
        l_c_15 = F::Mathf_Cos(c, l_camYaw_12);
        { let v_ = rt::fadd(rt::fmul(rt::fneg(l_c_15), F32(c.m.r32(fp.wrapping_add(64)))), rt::fmul(l_s_14, F32(c.m.r32((fp.wrapping_add(64)).wrapping_add(8))))); let a_ = fp.wrapping_add(80); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((fp.wrapping_add(64)).wrapping_add(4))); let a_ = (fp.wrapping_add(80)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(rt::fmul(l_s_14, F32(c.m.r32(fp.wrapping_add(64)))), rt::fmul(l_c_15, F32(c.m.r32((fp.wrapping_add(64)).wrapping_add(8))))); let a_ = (fp.wrapping_add(80)).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = (fp.wrapping_add(80)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(p_target_4)), F32(c.m.r32(fp.wrapping_add(80)))); let a_ = p_eye_2; c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32((p_target_4).wrapping_add(4))), F32(c.m.r32((fp.wrapping_add(80)).wrapping_add(4)))); let a_ = (p_eye_2).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32((p_target_4).wrapping_add(8))), F32(c.m.r32((fp.wrapping_add(80)).wrapping_add(8)))); let a_ = (p_eye_2).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = (p_eye_2).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let v_ = l_base_21; let a_ = p_rot_3; c.m.w32(a_, (v_).0); };
        { let v_ = l_yaw_11; let a_ = (p_rot_3).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (p_rot_3).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (p_rot_3).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let r_ = F::ChrCam_GetBob(c, p_chr_1); let a_ = (p_eye_2).wrapping_add(4); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "ChrCam_CalcFixed", &PROBE_N, &[], true); }
    r_
}

pub fn ChrCam_CalcLockOn(c: &mut Ctx, mut p_chr_1: u32, mut p_eye_2: u32, mut p_rot_3: u32, mut p_target_4: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ChrCam_CalcLockOn", &PROBE_N, &[(p_chr_1) as u32, (p_eye_2) as u32, (p_rot_3) as u32, (p_target_4) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(128) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 128);
    let mut l_cam_5: u32 = 0;
    let mut l_scale_6: F32 = F32(0);
    let mut l_len_7: F32 = F32(0);
    let mut l_a_8: F32 = F32(0);
    let mut l_b_9: F32 = F32(0);
    let mut l_u_10: F32 = F32(0);
    let mut l_horiz_11: F32 = F32(0);
    let mut l_ofsLen_12: F32 = F32(0);
    let mut l_yaw_13: F32 = F32(0);
    let mut l_camYaw_14: F32 = F32(0);
    let mut l_s_15: F32 = F32(0);
    let mut l_c_16: F32 = F32(0);
    let mut l_maxPitch_17: F32 = F32(0);
    let mut l_pitch_18: F32 = F32(0);
    let mut l_base_19: F32 = F32(0);
    let mut l_floorY_20: F32 = F32(0);
    let mut l_limit_21: F32 = F32(0);
    let mut l_width_22: F32 = F32(0);
    let mut l_fov_23: F32 = F32(0);
    let mut l_half_24: F32 = F32(0);
    let mut l_tn_25: F32 = F32(0);
    let mut l_spread_26: F32 = F32(0);
    let mut l_side_27: F32 = F32(0);
    let mut l_turn_28: F32 = F32(0);
    let mut l_d_29: F32 = F32(0);
    let mut l_m_30: F32 = F32(0);
    let r_ = 'f: {
        l_cam_5 = (p_chr_1).wrapping_add(1056);
        l_scale_6 = { let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); F::BtlCharApi_GetHeight(c, a0_) };
        { let a1_ = F::BtlChar_GetPos(c, p_chr_1); F::Vec4_Copy(c, fp, a1_) };
        l_floorY_20 = rt::fadd(F::BtlStage_GetBottom(c), rt::fmul(l_scale_6, F32(0x3f000000)));
        if (((rt::flt(l_floorY_20, F32(c.m.r32((fp).wrapping_add(4)))) as i32)) != 0) {
            { let v_ = l_floorY_20; let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
        }
        F::BtlOpp_GetTargetPos(c, p_chr_1, fp.wrapping_add(16));
        F::Vec4_Sub(c, fp.wrapping_add(32), fp.wrapping_add(16), fp);
        { let r_ = F::BtlOpp_GetHalfHeightDiff(c, p_chr_1); let a_ = (fp.wrapping_add(32)).wrapping_add(4); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        l_len_7 = F::Vec3_Length(c, fp.wrapping_add(32));
        if (((rt::flt(l_len_7, F32(0x3c23d70a)) as i32)) != 0) {
            F::Vec4_Set(c, fp.wrapping_add(48), F32(0x00000000), F32(0x00000000), F32(0x3f800000), F32(0x00000000));
        } else {
            F::Vec4_Scale(c, fp.wrapping_add(48), fp.wrapping_add(32), rt::fdiv(F32(0x3f800000), l_len_7));
        }
        F::Vec4_Set(c, p_target_4, F32(0x00000000), rt::fneg(rt::fmul(l_scale_6, F32(0x3f333333))), F32(0x00000000), F32(0x3f800000));
        l_base_19 = { let a2_ = (c.m.r32((l_cam_5).wrapping_add(124)) as i32); F::ChrCam_GetOffset(c, p_chr_1, fp.wrapping_add(64), a2_) };
        F::Vec4_Sub(c, fp.wrapping_add(64), fp.wrapping_add(64), p_target_4);
        l_yaw_13 = F32(c.m.r32((l_cam_5).wrapping_add(128)));
        l_camYaw_14 = l_yaw_13;
        if (((rt::flt(l_len_7, F32(0x00000000)) as i32)) != 0) {
            l_u_10 = F32(0x00000000);
        } else {
            if (((rt::flt(l_len_7, F32(0x42c80000)) as i32)) != 0) {
                l_u_10 = rt::fdiv(rt::fsub(l_len_7, F32(0x00000000)), F32(0x42c80000));
            } else {
                l_u_10 = F32(0x3f800000);
            }
        }
        l_u_10 = rt::fsub(rt::fmul(l_u_10, F32(0x40000000)), F32(0x3f800000));
        l_u_10 = rt::fadd(rt::fmul(rt::fneg(l_u_10), rt::ffabs(l_u_10)), rt::fmul(l_u_10, F32(0x40000000)));
        l_u_10 = rt::fadd(rt::fmul(l_u_10, F32(0x3f000000)), F32(0x3f000000));
        l_u_10 = rt::fadd(rt::fmul(l_u_10, F32(0x3e4ccccc)), F32(0x3f4ccccc));
        F::Vec4_Scale(c, fp.wrapping_add(64), fp.wrapping_add(64), l_u_10);
        F::Vec4_Scale(c, p_target_4, p_target_4, rt::fsub(F32(0x3f800000), rt::fmul(rt::fsub(F32(0x3f800000), l_u_10), F32(0x3f000000))));
        l_maxPitch_17 = F32(0x3f8cbe4b);
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((214i32) as i64 as u64)) as u32))) != 0) {
            l_maxPitch_17 = F32(0x40470d21);
        }
        if (((rt::flt(F32(0x3f800000), F32(c.m.r32((fp.wrapping_add(48)).wrapping_add(4)))) as i32)) != 0) {
            { let v_ = F32(0x3f800000); let a_ = (fp.wrapping_add(48)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        }
        if (((rt::flt(F32(c.m.r32((fp.wrapping_add(48)).wrapping_add(4))), F32(0xbf800000)) as i32)) != 0) {
            { let v_ = F32(0xbf800000); let a_ = (fp.wrapping_add(48)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        }
        l_a_8 = rt::fdiv(rt::fneg({ let a0_ = F32(c.m.r32((fp.wrapping_add(48)).wrapping_add(4))); F::Mathf_Asin(c, a0_) }), l_maxPitch_17);
        if (((rt::flt(F32(0x3f800000), l_a_8) as i32)) != 0) {
            l_a_8 = F32(0x3f800000);
        }
        if (((rt::flt(l_a_8, F32(0xbf800000)) as i32)) != 0) {
            l_a_8 = F32(0xbf800000);
        }
        l_b_9 = l_a_8;
        if (((rt::flt(l_b_9, F32(0x00000000)) as i32)) != 0) {
            l_b_9 = F32(0x00000000);
        }
        l_b_9 = rt::fsub(rt::fmul(l_b_9, F32(0x40000000)), F32(0x3f800000));
        l_a_8 = rt::fadd(rt::fmul(rt::fneg(l_a_8), rt::ffabs(l_a_8)), rt::fmul(l_a_8, F32(0x40000000)));
        l_b_9 = rt::fadd(rt::fmul(rt::fneg(l_b_9), rt::ffabs(l_b_9)), rt::fmul(l_b_9, F32(0x40000000)));
        l_pitch_18 = rt::fmul(rt::fmul(l_maxPitch_17, l_a_8), F32(0x3ecccccc));
        l_b_9 = rt::fadd(rt::fmul(l_b_9, F32(0x3f000000)), F32(0x3f000000));
        F::Vec4_Set(c, fp.wrapping_add(96), l_pitch_18, F32(0x00000000), F32(0x00000000), F32(0x00000000));
        { let v_ = F32(0x00000000); let a_ = (fp.wrapping_add(64)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        F::Vec4_RotateEuler(c, fp.wrapping_add(64), fp.wrapping_add(96), fp.wrapping_add(64));
        F::Vec4_Scale(c, fp.wrapping_add(64), fp.wrapping_add(64), rt::fsub(F32(0x3f800000), rt::fmul(l_b_9, F32(0x3e999999))));
        l_base_19 = rt::fadd(l_base_19, rt::fsub(l_pitch_18, rt::fmul(l_base_19, l_b_9)));
        l_limit_21 = F::ChrCam_GetSideLimit(c, p_chr_1);
        l_horiz_11 = F::BtlUtil_LengthXZ(c, fp.wrapping_add(32));
        l_ofsLen_12 = F::BtlUtil_LengthXZ(c, fp.wrapping_add(64));
        l_width_22 = (if ((F::Battle_IsSplitScreen(c)) != 0) { F32(0x43800000) } else { F32(0x44000000) });
        l_fov_23 = rt::fmul(F::Mathf_Atan(c, rt::fdiv(rt::fmul(l_width_22, F32(0x3f000000)), F32(0x43d88000))), F32(0x40000000));
        l_half_24 = rt::fmul(l_fov_23, F32(0x3f333333));
        l_tn_25 = F::Mathf_Tan(c, l_half_24);
        if (((rt::flt(F32(0x3c23d70a), l_horiz_11) as i32)) != 0) {
            l_spread_26 = F::Mathf_Atan(c, rt::fmul(l_tn_25, rt::fdiv(l_ofsLen_12, l_horiz_11)));
        } else {
            l_spread_26 = F32(0x3fc90fda);
            if ((((!(((rt::flt(F32(0x00000000), rt::fmul(rt::fmul(l_tn_25, l_ofsLen_12), l_horiz_11)) as i32)) != 0)) as i32)) != 0) {
                l_spread_26 = F32(0xbfc90fda);
            }
        }
        l_d_29 = rt::fsub(l_limit_21, l_fov_23);
        l_side_27 = rt::fdiv(F32(c.m.r32((l_cam_5).wrapping_add(136))), l_limit_21);
        l_m_30 = rt::fmul(l_spread_26, l_d_29);
        l_m_30 = rt::fdiv(l_m_30, l_limit_21);
        l_turn_28 = rt::fadd(l_half_24, l_m_30);
        F::Vec4_Set(c, fp.wrapping_add(112), F32(0x00000000), rt::fmul(l_turn_28, l_side_27), F32(0x00000000), F32(0x00000000));
        { let v_ = F32(0x00000000); let a_ = (fp.wrapping_add(64)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        F::Vec4_RotateEuler(c, fp.wrapping_add(64), fp.wrapping_add(112), fp.wrapping_add(64));
        l_turn_28 = rt::fadd(l_turn_28, rt::fmul(l_spread_26, rt::fdiv(l_limit_21, F32(0x3fc90fda))));
        l_turn_28 = rt::fmul(l_turn_28, F32(0x3f000000));
        l_turn_28 = rt::fmul(l_turn_28, l_side_27);
        { let v_ = l_turn_28; let a_ = (l_cam_5).wrapping_add(140); c.m.w32(a_, (v_).0); };
        l_yaw_13 = F::BtlUtil_WrapAngle(c, rt::fsub(l_yaw_13, l_turn_28));
        { let r_ = F32(c.m.r32(fp)); let a_ = p_target_4; let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        { let r_ = F32(c.m.r32((fp).wrapping_add(4))); let a_ = (p_target_4).wrapping_add(4); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        { let r_ = F32(c.m.r32((fp).wrapping_add(8))); let a_ = (p_target_4).wrapping_add(8); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        { let v_ = F32(0x3f800000); let a_ = (p_target_4).wrapping_add(12); c.m.w32(a_, (v_).0); };
        l_s_15 = F::Mathf_Sin(c, l_camYaw_14);
        l_c_16 = F::Mathf_Cos(c, l_camYaw_14);
        { let v_ = rt::fadd(rt::fmul(rt::fneg(l_c_16), F32(c.m.r32(fp.wrapping_add(64)))), rt::fmul(l_s_15, F32(c.m.r32((fp.wrapping_add(64)).wrapping_add(8))))); let a_ = fp.wrapping_add(80); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((fp.wrapping_add(64)).wrapping_add(4))); let a_ = (fp.wrapping_add(80)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(rt::fmul(l_s_15, F32(c.m.r32(fp.wrapping_add(64)))), rt::fmul(l_c_16, F32(c.m.r32((fp.wrapping_add(64)).wrapping_add(8))))); let a_ = (fp.wrapping_add(80)).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = (fp.wrapping_add(80)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(p_target_4)), F32(c.m.r32(fp.wrapping_add(80)))); let a_ = p_eye_2; c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32((p_target_4).wrapping_add(4))), F32(c.m.r32((fp.wrapping_add(80)).wrapping_add(4)))); let a_ = (p_eye_2).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32((p_target_4).wrapping_add(8))), F32(c.m.r32((fp.wrapping_add(80)).wrapping_add(8)))); let a_ = (p_eye_2).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = (p_eye_2).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let v_ = l_base_19; let a_ = p_rot_3; c.m.w32(a_, (v_).0); };
        { let v_ = l_yaw_13; let a_ = (p_rot_3).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (p_rot_3).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (p_rot_3).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let r_ = F::ChrCam_GetBob(c, p_chr_1); let a_ = (p_eye_2).wrapping_add(4); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "ChrCam_CalcLockOn", &PROBE_N, &[], true); }
    r_
}

pub fn ChrCam_CalcFree(c: &mut Ctx, mut p_chr_1: u32, mut p_eye_2: u32, mut p_rot_3: u32, mut p_target_4: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ChrCam_CalcFree", &PROBE_N, &[(p_chr_1) as u32, (p_eye_2) as u32, (p_rot_3) as u32, (p_target_4) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let mut l_cam_5: u32 = 0;
    let mut l_one_6: F32 = F32(0);
    let mut l_scale_7: F32 = F32(0);
    let mut l_pitch_8: F32 = F32(0);
    let mut l_yaw_9: F32 = F32(0);
    let mut l_s_10: F32 = F32(0);
    let mut l_c_11: F32 = F32(0);
    let mut l_body_12: u32 = 0;
    let r_ = 'f: {
        l_cam_5 = (p_chr_1).wrapping_add(1056);
        l_body_12 = F::BtlChar_GetPos(c, p_chr_1);
        l_one_6 = F32(0x3f800000);
        l_scale_7 = { let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); F::BtlCharApi_GetHeight(c, a0_) };
        F::Vec4_Copy(c, fp, l_body_12);
        { let r_ = F32(0x3f333333); let n_ = rt::fmul(l_scale_7, r_); l_scale_7 = n_; };
        { let v_ = l_one_6; let a_ = (fp.wrapping_add(32)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = fp.wrapping_add(32); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (fp.wrapping_add(32)).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fneg(l_scale_7); let a_ = (fp.wrapping_add(32)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        l_pitch_8 = { let a2_ = (c.m.r32((l_cam_5).wrapping_add(124)) as i32); F::ChrCam_GetOffset(c, p_chr_1, fp.wrapping_add(16), a2_) };
        l_yaw_9 = F32(c.m.r32((l_cam_5).wrapping_add(128)));
        l_s_10 = F::Mathf_Sin(c, l_yaw_9);
        { let v_ = rt::fadd(rt::fadd(F32(c.m.r32(fp)), rt::fmul(l_s_10, rt::fsub(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8))), F32(c.m.r32((fp.wrapping_add(32)).wrapping_add(8)))))), F32(c.m.r32((fp.wrapping_add(32)).wrapping_add(8)))); let a_ = p_eye_2; c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(rt::fadd(F32(c.m.r32((fp).wrapping_add(4))), rt::fsub(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))), F32(c.m.r32((fp.wrapping_add(32)).wrapping_add(4))))), F32(c.m.r32((fp.wrapping_add(32)).wrapping_add(4)))); let a_ = (p_eye_2).wrapping_add(4); c.m.w32(a_, (v_).0); };
        l_c_11 = F::Mathf_Cos(c, l_yaw_9);
        { let v_ = l_one_6; let a_ = (p_eye_2).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(rt::fadd(F32(c.m.r32((fp).wrapping_add(8))), rt::fmul(l_c_11, rt::fsub(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8))), F32(c.m.r32((fp.wrapping_add(32)).wrapping_add(8)))))), F32(c.m.r32((fp.wrapping_add(32)).wrapping_add(8)))); let a_ = (p_eye_2).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(fp.wrapping_add(32))), F32(c.m.r32(fp))); let a_ = p_target_4; c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32((fp.wrapping_add(32)).wrapping_add(4))), F32(c.m.r32((fp).wrapping_add(4)))); let a_ = (p_target_4).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32((fp.wrapping_add(32)).wrapping_add(8))), F32(c.m.r32((fp).wrapping_add(8)))); let a_ = (p_target_4).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = l_one_6; let a_ = (p_target_4).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let v_ = l_pitch_8; let a_ = p_rot_3; c.m.w32(a_, (v_).0); };
        { let v_ = l_yaw_9; let a_ = (p_rot_3).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (p_rot_3).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (p_rot_3).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let r_ = F::ChrCam_GetBob(c, p_chr_1); let a_ = (p_eye_2).wrapping_add(4); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "ChrCam_CalcFree", &PROBE_N, &[], true); }
    r_
}

pub fn ChrCam_KeepBehindHead(c: &mut Ctx, mut p_chr_1: u32, mut p_eye_2: u32, mut p_rot_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ChrCam_KeepBehindHead", &PROBE_N, &[(p_chr_1) as u32, (p_eye_2) as u32, (p_rot_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(64) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 64);
    let mut l_one_4: F32 = F32(0);
    let mut l_margin_5: F32 = F32(0);
    let mut l_d_6: F32 = F32(0);
    let r_ = 'f: {
        l_one_4 = F32(0x3f800000);
        { let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); F::BtlCharApi_GetNodePos(c, a0_, 48i32, fp.wrapping_add(16)) };
        l_margin_5 = rt::fmul({ let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); F::BtlCharApi_GetHeight(c, a0_) }, F32(0x3e199999));
        { let v_ = { let a0_ = F32(c.m.r32((p_rot_3).wrapping_add(4))); F::Mathf_Sin(c, a0_) }; let a_ = fp.wrapping_add(32); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (fp.wrapping_add(32)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32((p_rot_3).wrapping_add(4))); F::Mathf_Cos(c, a0_) }; let a_ = (fp.wrapping_add(32)).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32(fp.wrapping_add(32))); let a_ = fp; c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((fp.wrapping_add(32)).wrapping_add(4))); let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((fp.wrapping_add(32)).wrapping_add(8))); let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (fp.wrapping_add(32)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fneg(F::Vec3_Dot(c, fp.wrapping_add(32), fp.wrapping_add(16))); let a_ = (fp).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(rt::fadd(F32(c.m.r32((fp).wrapping_add(12))), rt::fmul(l_margin_5, rt::fadd({ let a0_ = F32(c.m.r32(p_rot_3)); F::Mathf_Cos(c, a0_) }, l_one_4))), F32(0x41200000)); let a_ = (fp).wrapping_add(12); c.m.w32(a_, (v_).0); };
        l_d_6 = rt::fadd(F::Vec3_Dot(c, fp, p_eye_2), F32(c.m.r32((fp).wrapping_add(12))));
        if (((rt::flt(F32(0x00000000), l_d_6) as i32)) != 0) {
            F::Vec4_Scale(c, fp.wrapping_add(48), fp, l_d_6);
            F::Vec4_Sub(c, p_eye_2, p_eye_2, fp.wrapping_add(48));
            { let v_ = l_one_4; let a_ = (p_eye_2).wrapping_add(12); c.m.w32(a_, (v_).0); };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "ChrCam_KeepBehindHead", &PROBE_N, &[], true); }
    r_
}

pub fn ChrCam_UpdateLockOnYaw(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ChrCam_UpdateLockOnYaw", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_cam_2: u32 = 0;
    let mut l_step_3: F32 = F32(0);
    let mut l_limit_4: F32 = F32(0);
    let mut l_d_5: F32 = F32(0);
    let mut bb_: u32 = 0;
    let r_ = 'f: {
    'fl1: loop {
        match bb_ {
            0 => {
                l_step_3 = F::BtlOpp_GetYaw(c, p_chr_1);
                l_cam_2 = (p_chr_1).wrapping_add(1056);
                l_step_3 = { let a0_ = rt::fsub(l_step_3, F32(c.m.r32((l_cam_2).wrapping_add(128)))); F::BtlUtil_WrapAngle(c, a0_) };
                if ((((!((F::BtlChar_TestFlag(c, p_chr_1, ((((205i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) { bb_ = 1; continue 'fl1; } else { bb_ = 2; continue 'fl1; }
            }
            1 => {
                { let r_ = F32(0x3e800000); let n_ = rt::fmul(l_step_3, r_); l_step_3 = n_; };
                bb_ = 2; continue 'fl1;
            }
            2 => {
                { let v_ = { let a0_ = rt::fadd(F32(c.m.r32((l_cam_2).wrapping_add(128))), l_step_3); F::BtlUtil_WrapAngle(c, a0_) }; let a_ = (l_cam_2).wrapping_add(128); c.m.w32(a_, (v_).0); };
                { let v_ = l_step_3; let a_ = (l_cam_2).wrapping_add(132); c.m.w32(a_, (v_).0); };
                if ((F::BtlChar_TestFlag(c, p_chr_1, ((((210i32) as i64 as u64)) as u32))) != 0) { bb_ = 3; continue 'fl1; } else { bb_ = 4; continue 'fl1; }
            }
            3 => {
                break 'f;
            }
            4 => {
                l_limit_4 = F::ChrCam_GetSideLimit(c, p_chr_1);
                if ((F::BtlChar_IsFlagRaised(c, p_chr_1, ((((5i32) as i64 as u64)) as u32))) != 0) { bb_ = 6; continue 'fl1; } else { bb_ = 7; continue 'fl1; }
            }
            5 => {
                bb_ = 4; continue 'fl1;
            }
            6 => {
                bb_ = 8; continue 'fl1;
            }
            7 => {
                if ((F::BtlInput_IsHeld(c, p_chr_1, ((256i32) as u32))) != 0) { bb_ = 10; continue 'fl1; } else { bb_ = 12; continue 'fl1; }
            }
            8 => {
                if (((rt::flt(F32(0x00000000), F32(c.m.r32((l_cam_2).wrapping_add(136)))) as i32)) != 0) { bb_ = 19; continue 'fl1; } else { bb_ = 21; continue 'fl1; }
            }
            9 => {
                bb_ = 7; continue 'fl1;
            }
            10 => {
                { let v_ = rt::fneg(l_limit_4); let a_ = (l_cam_2).wrapping_add(136); c.m.w32(a_, (v_).0); };
                bb_ = 11; continue 'fl1;
            }
            11 => {
                break 'fl1;
            }
            12 => {
                if ((F::BtlInput_IsHeld(c, p_chr_1, ((1024i32) as u32))) != 0) { bb_ = 13; continue 'fl1; } else { bb_ = 15; continue 'fl1; }
            }
            13 => {
                { let v_ = l_limit_4; let a_ = (l_cam_2).wrapping_add(136); c.m.w32(a_, (v_).0); };
                bb_ = 14; continue 'fl1;
            }
            14 => {
                bb_ = 11; continue 'fl1;
            }
            15 => {
                if ((F::BtlChar_TestFlag(c, p_chr_1, ((((213i32) as i64 as u64)) as u32))) != 0) { bb_ = 16; continue 'fl1; } else { bb_ = 18; continue 'fl1; }
            }
            16 => {
                bb_ = 8; continue 'fl1;
            }
            17 => {
                bb_ = 14; continue 'fl1;
            }
            18 => {
                l_d_5 = F32(c.m.r32((l_cam_2).wrapping_add(132)));
                if (((rt::flt(F32(0x3fc90fda), l_d_5) as i32)) != 0) { bb_ = 22; continue 'fl1; } else { bb_ = 23; continue 'fl1; }
            }
            19 => {
                { let v_ = l_limit_4; let a_ = (l_cam_2).wrapping_add(136); c.m.w32(a_, (v_).0); };
                bb_ = 20; continue 'fl1;
            }
            20 => {
                bb_ = 17; continue 'fl1;
            }
            21 => {
                { let v_ = rt::fneg(l_limit_4); let a_ = (l_cam_2).wrapping_add(136); c.m.w32(a_, (v_).0); };
                bb_ = 20; continue 'fl1;
            }
            22 => {
                { let r_ = F32(0x40490fda); let n_ = rt::fsub(l_d_5, r_); l_d_5 = n_; };
                bb_ = 23; continue 'fl1;
            }
            23 => {
                if (((rt::flt(l_d_5, F32(0xbfc90fda)) as i32)) != 0) { bb_ = 24; continue 'fl1; } else { bb_ = 25; continue 'fl1; }
            }
            24 => {
                { let r_ = F32(0x40490fda); let n_ = rt::fadd(l_d_5, r_); l_d_5 = n_; };
                bb_ = 25; continue 'fl1;
            }
            25 => {
                { let r_ = l_d_5; let a_ = (l_cam_2).wrapping_add(136); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                if (((rt::flt(F32(c.m.r32((l_cam_2).wrapping_add(136))), rt::fneg(l_limit_4)) as i32)) != 0) { bb_ = 26; continue 'fl1; } else { bb_ = 27; continue 'fl1; }
            }
            26 => {
                { let v_ = rt::fneg(l_limit_4); let a_ = (l_cam_2).wrapping_add(136); c.m.w32(a_, (v_).0); };
                bb_ = 27; continue 'fl1;
            }
            27 => {
                if (((rt::flt(l_limit_4, F32(c.m.r32((l_cam_2).wrapping_add(136)))) as i32)) != 0) { bb_ = 28; continue 'fl1; } else { bb_ = 29; continue 'fl1; }
            }
            28 => {
                { let v_ = l_limit_4; let a_ = (l_cam_2).wrapping_add(136); c.m.w32(a_, (v_).0); };
                bb_ = 29; continue 'fl1;
            }
            29 => {
                bb_ = 17; continue 'fl1;
            }
            _ => unreachable!(),
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ChrCam_UpdateLockOnYaw", &PROBE_N, &[], true); }
    r_
}

pub fn ChrCam_UpdateFreeYaw(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ChrCam_UpdateFreeYaw", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_cam_2: u32 = 0;
    let mut l_d_3: F32 = F32(0);
    let r_ = 'f: {
    l_cam_2 = (p_chr_1).wrapping_add(1056);
    if ((F::BtlChar_TestFlag(c, p_chr_1, ((((201i32) as i64 as u64)) as u32))) != 0) {
        { let v_ = { let a0_ = rt::fadd(F32(c.m.r32((l_cam_2).wrapping_add(128))), rt::fmul(F::BtlInput_GetStickX(c, p_chr_1), F32(0x3d088888))); F::BtlUtil_WrapAngle(c, a0_) }; let a_ = (l_cam_2).wrapping_add(128); c.m.w32(a_, (v_).0); };
        if ((F::BtlInput_TestAction(c, p_chr_1, 97i32, 1i32)) != 0) {
            { let r_ = F32(0x3d56774e); let a_ = (l_cam_2).wrapping_add(128); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        }
        if ((F::BtlInput_TestAction(c, p_chr_1, 98i32, 1i32)) != 0) {
            { let r_ = F32(0x3d56774e); let a_ = (l_cam_2).wrapping_add(128); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        }
    } else {
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((202i32) as i64 as u64)) as u32))) != 0) {
            l_d_3 = { let a0_ = rt::fsub({ let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(148); F32(c.m.r32(a_)) }, F32(c.m.r32((l_cam_2).wrapping_add(128)))); F::BtlUtil_WrapAngle(c, a0_) };
            { let v_ = { let a0_ = { let a0_ = F32(c.m.r32((l_cam_2).wrapping_add(128))); let a1_ = rt::fadd(F32(c.m.r32((l_cam_2).wrapping_add(128))), l_d_3); let a2_ = { let a0_ = rt::fmul(F::Mathf_Sin(c, rt::ffabs(l_d_3)), F32(0x3d4ccccc)); F::BtlUtil_MinF(c, a0_, F32(0x3d088888)) }; F::BtlUtil_ApproachF(c, a0_, a1_, a2_) }; F::BtlUtil_WrapAngle(c, a0_) }; let a_ = (l_cam_2).wrapping_add(128); c.m.w32(a_, (v_).0); };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ChrCam_UpdateFreeYaw", &PROBE_N, &[], true); }
    r_
}

pub fn ChrCam_TurnToOpponent(c: &mut Ctx, mut p_chr_1: u32, mut p_rot_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ChrCam_TurnToOpponent", &PROBE_N, &[(p_chr_1) as u32, (p_rot_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(64) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 64);
    let mut l_cam_3: u32 = 0;
    let mut l_t_4: F32 = F32(0);
    let mut l_yaw_5: F32 = F32(0);
    let mut l_dist_6: F32 = F32(0);
    let r_ = 'f: {
        F::Vec4_Copy(c, fp, ((p_chr_1).wrapping_add(1056)).wrapping_add(16));
        l_cam_3 = (p_chr_1).wrapping_add(1056);
        { let a1_ = F::BtlChar_GetPos(c, p_chr_1); F::Vec4_Copy(c, fp.wrapping_add(16), a1_) };
        F::BtlOpp_GetTargetPos(c, p_chr_1, fp.wrapping_add(32));
        F::Vec4_Sub(c, fp.wrapping_add(48), fp.wrapping_add(32), fp);
        l_yaw_5 = { let a0_ = F32(c.m.r32(fp.wrapping_add(48))); let a1_ = F32(c.m.r32((fp.wrapping_add(48)).wrapping_add(8))); F::Port_atan2f(c, a0_, a1_) };
        l_dist_6 = F::BtlOpp_GetDistance(c, p_chr_1);
        if (((rt::flt(l_dist_6, F32(0x42480000)) as i32)) != 0) {
            l_t_4 = F32(0x3f800000);
        } else {
            if (((rt::flt(l_dist_6, F32(0x447a0000)) as i32)) != 0) {
                l_t_4 = rt::fsub(F32(0x3f800000), rt::fdiv(rt::fsub(l_dist_6, F32(0x42480000)), F32(0x446d8000)));
            } else {
                l_t_4 = F32(0x00000000);
            }
        }
        l_t_4 = rt::fsub(rt::fmul(l_t_4, F32(0x40000000)), F32(0x3f800000));
        l_t_4 = rt::fadd(rt::fmul(rt::fneg(l_t_4), rt::ffabs(l_t_4)), rt::fmul(l_t_4, F32(0x40000000)));
        l_t_4 = rt::fadd(rt::fmul(l_t_4, F32(0x3f000000)), F32(0x3f000000));
        { let r_ = rt::fsub(F32(0x3f800000), F32(c.m.r32((l_cam_3).wrapping_add(120)))); let n_ = rt::fmul(l_t_4, r_); l_t_4 = n_; };
        if (((rt::flt(F32(0x00000000), l_t_4) as i32)) != 0) {
            { let v_ = { let a0_ = rt::fadd(F32(c.m.r32((p_rot_2).wrapping_add(4))), rt::fmul({ let a0_ = rt::fsub(l_yaw_5, F32(c.m.r32((p_rot_2).wrapping_add(4)))); F::BtlUtil_WrapAngle(c, a0_) }, l_t_4)); F::BtlUtil_WrapAngle(c, a0_) }; let a_ = (p_rot_2).wrapping_add(4); c.m.w32(a_, (v_).0); };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "ChrCam_TurnToOpponent", &PROBE_N, &[], true); }
    r_
}

pub fn ChrCam_UpdateInput(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ChrCam_UpdateInput", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_dst_2: u32 = 0;
    let r_ = 'f: {
    l_dst_2 = ((p_chr_1).wrapping_add(1056)).wrapping_add(64);
    if ((F::BtlInput_IsPressed(c, p_chr_1, ((8388608i32) as u32))) != 0) {
        { let a_ = ((p_chr_1).wrapping_add(1056)).wrapping_add(124); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        { let r_ = 3i32; let a_ = ((p_chr_1).wrapping_add(1056)).wrapping_add(124); let n_ = rt::rem_i32((c.m.r32(a_) as i32), r_); c.m.w32(a_, (n_) as u32); };
    }
    if ((F::BtlChar_TestFlag(c, p_chr_1, ((((5i32) as i64 as u64)) as u32))) != 0) {
        F::ChrCam_UpdateLockOnYaw(c, p_chr_1);
    } else {
        F::ChrCam_UpdateFreeYaw(c, p_chr_1);
    }
    { let _ = F::BtlAnim_GetId(c, p_chr_1); };
    { let a1_ = { let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(((4000i32) as u32)); c.m.r32(a_) }; F::Vec4_Copy(c, l_dst_2, a1_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ChrCam_UpdateInput", &PROBE_N, &[], true); }
    r_
}

pub fn ChrCam_Update(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ChrCam_Update", &PROBE_N, &[(p_chr_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(96) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 96);
    let mut l_cam_2: u32 = 0;
    let mut l_keepBehind_3: i32 = 0;
    let mut l_turn_4: i32 = 0;
    let mut l_smooth_5: i32 = 0;
    let mut l_limit_6: F32 = F32(0);
    let mut l_len_7: F32 = F32(0);
    let mut l_rate_8: F32 = F32(0);
    let mut l_k_9: F32 = F32(0);
    let mut l_k_10: F32 = F32(0);
    let mut bb_: u32 = 0;
    let r_ = 'f: {
        'fl1: loop {
            match bb_ {
                0 => {
                    l_cam_2 = (p_chr_1).wrapping_add(1056);
                    l_keepBehind_3 = 0i32;
                    l_turn_4 = 0i32;
                    if ((F::BtlChar_TestFlag(c, p_chr_1, ((((204i32) as i64 as u64)) as u32))) != 0) { bb_ = 1; continue 'fl1; } else { bb_ = 3; continue 'fl1; }
                }
                1 => {
                    F::Vec4_Copy(c, fp, ((p_chr_1).wrapping_add(1056)).wrapping_add(16));
                    F::Vec4_Copy(c, fp.wrapping_add(16), ((p_chr_1).wrapping_add(1056)).wrapping_add(32));
                    F::Vec4_Copy(c, fp.wrapping_add(32), ((p_chr_1).wrapping_add(1056)).wrapping_add(48));
                    if ((F::BtlChar_TestPrevFlag(c, p_chr_1, ((((211i32) as i64 as u64)) as u32))) != 0) { bb_ = 4; continue 'fl1; } else { bb_ = 5; continue 'fl1; }
                }
                2 => {
                    bb_ = 6; continue 'fl1;
                }
                3 => {
                    if ((F::ChrCam_IsCutActive(c, p_chr_1)) != 0) { bb_ = 8; continue 'fl1; } else { bb_ = 10; continue 'fl1; }
                }
                4 => {
                    F::BtlChar_SetFlag(c, p_chr_1, ((((211i32) as i64 as u64)) as u32));
                    bb_ = 6; continue 'fl1;
                }
                5 => {
                    bb_ = 2; continue 'fl1;
                }
                6 => {
                    F::CamShake_Calc(c, (l_cam_2).wrapping_add(80), fp.wrapping_add(48), fp.wrapping_add(64));
                    F::Vec4_Add(c, fp, fp, fp.wrapping_add(48));
                    F::CamShake_Tick(c, (l_cam_2).wrapping_add(80));
                    l_limit_6 = F::BtlStage_GetRadius(c);
                    l_len_7 = F::BtlUtil_LengthXZ(c, fp);
                    if (((rt::flt(l_limit_6, l_len_7) as i32)) != 0) { bb_ = 18; continue 'fl1; } else { bb_ = 19; continue 'fl1; }
                }
                7 => {
                    bb_ = 5; continue 'fl1;
                }
                8 => {
                    F::ChrCam_CalcCut(c, p_chr_1, fp, fp.wrapping_add(16), fp.wrapping_add(32));
                    bb_ = 6; continue 'fl1;
                }
                9 => {
                    bb_ = 2; continue 'fl1;
                }
                10 => {
                    if ((((((F::BtlChar_TestFlag(c, p_chr_1, ((((220i32) as i64 as u64)) as u32))) != 0) || ((F::BtlChar_TestFlag(c, p_chr_1, ((((184i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) { bb_ = 12; continue 'fl1; } else { bb_ = 14; continue 'fl1; }
                }
                11 => {
                    bb_ = 9; continue 'fl1;
                }
                12 => {
                    F::ChrCam_CalcFixed(c, p_chr_1, fp, fp.wrapping_add(16), fp.wrapping_add(32));
                    l_turn_4 = 1i32;
                    bb_ = 13; continue 'fl1;
                }
                13 => {
                    bb_ = 9; continue 'fl1;
                }
                14 => {
                    if ((F::BtlChar_TestFlag(c, p_chr_1, ((((5i32) as i64 as u64)) as u32))) != 0) { bb_ = 15; continue 'fl1; } else { bb_ = 17; continue 'fl1; }
                }
                15 => {
                    F::ChrCam_CalcLockOn(c, p_chr_1, fp, fp.wrapping_add(16), fp.wrapping_add(32));
                    l_keepBehind_3 = 1i32;
                    l_turn_4 = 1i32;
                    bb_ = 16; continue 'fl1;
                }
                16 => {
                    bb_ = 13; continue 'fl1;
                }
                17 => {
                    F::ChrCam_CalcFree(c, p_chr_1, fp, fp.wrapping_add(16), fp.wrapping_add(32));
                    l_keepBehind_3 = 1i32;
                    bb_ = 16; continue 'fl1;
                }
                18 => {
                    l_k_9 = rt::fdiv(l_limit_6, l_len_7);
                    { let r_ = l_k_9; let a_ = fp; let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    { let r_ = l_k_9; let a_ = (fp).wrapping_add(8); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    bb_ = 19; continue 'fl1;
                }
                19 => {
                    l_len_7 = F::BtlUtil_LengthXZ(c, fp.wrapping_add(32));
                    if (((rt::flt(l_limit_6, l_len_7) as i32)) != 0) { bb_ = 20; continue 'fl1; } else { bb_ = 21; continue 'fl1; }
                }
                20 => {
                    l_k_10 = rt::fdiv(l_limit_6, l_len_7);
                    { let r_ = l_k_10; let a_ = fp.wrapping_add(32); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    { let r_ = l_k_10; let a_ = (fp.wrapping_add(32)).wrapping_add(8); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    bb_ = 21; continue 'fl1;
                }
                21 => {
                    l_smooth_5 = (((F::BtlChar_TestFlag(c, p_chr_1, ((((205i32) as i64 as u64)) as u32))) == (0i32)) as i32);
                    if ((F::BtlChar_TestPrevFlag(c, p_chr_1, ((((206i32) as i64 as u64)) as u32))) != 0) { bb_ = 22; continue 'fl1; } else { bb_ = 23; continue 'fl1; }
                }
                22 => {
                    l_smooth_5 = 0i32;
                    bb_ = 23; continue 'fl1;
                }
                23 => {
                    l_rate_8 = F::ChrCam_GetRate(c, p_chr_1, fp, fp.wrapping_add(32));
                    if ((l_smooth_5) != 0) { bb_ = 24; continue 'fl1; } else { bb_ = 26; continue 'fl1; }
                }
                24 => {
                    F::Vec4_Lerp(c, l_cam_2, fp, l_cam_2, l_rate_8);
                    bb_ = 25; continue 'fl1;
                }
                25 => {
                    F::Vec4_Copy(c, (l_cam_2).wrapping_add(48), fp.wrapping_add(32));
                    if ((l_keepBehind_3) != 0) { bb_ = 27; continue 'fl1; } else { bb_ = 28; continue 'fl1; }
                }
                26 => {
                    F::Vec4_Copy(c, l_cam_2, fp);
                    { let v_ = F::ChrCam_CalcRate(c, p_chr_1, fp, fp.wrapping_add(32)); let a_ = (l_cam_2).wrapping_add(148); c.m.w32(a_, (v_).0); };
                    bb_ = 25; continue 'fl1;
                }
                27 => {
                    F::ChrCam_KeepBehindHead(c, p_chr_1, l_cam_2, fp.wrapping_add(16));
                    bb_ = 28; continue 'fl1;
                }
                28 => {
                    { let v_ = F::BtlCam_TraceStage(c, (l_cam_2).wrapping_add(16), l_cam_2, (l_cam_2).wrapping_add(48), (l_cam_2).wrapping_add(120), (l_cam_2).wrapping_add(112)); let a_ = (l_cam_2).wrapping_add(116); c.m.w32(a_, (v_) as u32); };
                    if (((((((c.m.r32((l_cam_2).wrapping_add(116)) as i32)) != 0) && ((l_turn_4) != 0)) as i32)) != 0) { bb_ = 29; continue 'fl1; } else { bb_ = 30; continue 'fl1; }
                }
                29 => {
                    F::ChrCam_TurnToOpponent(c, p_chr_1, fp.wrapping_add(16));
                    bb_ = 30; continue 'fl1;
                }
                30 => {
                    if ((l_smooth_5) != 0) { bb_ = 31; continue 'fl1; } else { bb_ = 33; continue 'fl1; }
                }
                31 => {
                    F::Vec4_Sub(c, fp.wrapping_add(80), fp.wrapping_add(16), (l_cam_2).wrapping_add(32));
                    F::BtlUtil_WrapAngles(c, fp.wrapping_add(80), fp.wrapping_add(80));
                    F::Vec4_Scale(c, fp.wrapping_add(80), fp.wrapping_add(80), l_rate_8);
                    F::Vec4_Add(c, (l_cam_2).wrapping_add(32), (l_cam_2).wrapping_add(32), fp.wrapping_add(80));
                    F::BtlUtil_WrapAngles(c, (l_cam_2).wrapping_add(32), (l_cam_2).wrapping_add(32));
                    bb_ = 32; continue 'fl1;
                }
                32 => {
                    F::Vec4_Add(c, (l_cam_2).wrapping_add(32), (l_cam_2).wrapping_add(32), fp.wrapping_add(64));
                    F::BtlUtil_WrapAngles(c, (l_cam_2).wrapping_add(32), (l_cam_2).wrapping_add(32));
                    { let v_ = F32(0x3f800000); let a_ = (l_cam_2).wrapping_add(12); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(0x00000000); let a_ = ((l_cam_2).wrapping_add(32)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(0x3f800000); let a_ = ((l_cam_2).wrapping_add(16)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(0x3f800000); let a_ = ((l_cam_2).wrapping_add(48)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                    break 'fl1;
                }
                33 => {
                    F::Vec4_Copy(c, (l_cam_2).wrapping_add(32), fp.wrapping_add(16));
                    bb_ = 32; continue 'fl1;
                }
                _ => unreachable!(),
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "ChrCam_Update", &PROBE_N, &[], true); }
    r_
}

pub fn ChrCam_GetSideLimit(c: &mut Ctx, mut p_chr_1: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ChrCam_GetSideLimit", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_a_2: F32 = F32(0);
    let r_ = 'f: {
    l_a_2 = { let a0_ = { let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); F::BtlCharApi_GetBodyUnk1000(c, a0_) }; F::Mathf_Atan(c, a0_) };
    if (((rt::flt(l_a_2, F32(0x3f490fda)) as i32)) != 0) {
        l_a_2 = F32(0x3f490fda);
    }
    break 'f l_a_2;
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "ChrCam_GetSideLimit", &PROBE_N, &[], true); }
    r_
}

pub fn ChrCam_AddShake(c: &mut Ctx, mut p_chr_1: u32, mut p_strength_2: F32, mut p_time_3: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ChrCam_AddShake", &PROBE_N, &[(p_chr_1) as u32, (p_strength_2).0, (p_time_3).0], false); }
    let r_ = 'f: {
    if ((((((c.m.r32(((p_chr_1).wrapping_add(1056)).wrapping_add(152)) as i32)) != (0i32)) as i32)) != 0) {
        F::CamShake_Add(c, ((p_chr_1).wrapping_add(1056)).wrapping_add(80), p_strength_2, p_time_3);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ChrCam_AddShake", &PROBE_N, &[], true); }
    r_
}

pub fn ChrCam_UpdateDemo(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ChrCam_UpdateDemo", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    if ((F::BtlChar_TestFlag(c, p_chr_1, ((((218i32) as i64 as u64)) as u32))) != 0) {
        F::DemoCam_Stop(c);
    }
    if ((F::BtlChar_TestFlag(c, p_chr_1, ((((215i32) as i64 as u64)) as u32))) != 0) {
        { let a0_ = (c.m.r32(p_chr_1) as i32); F::DemoCam_PlayCharAnim0(c, a0_) };
    }
    if ((F::BtlChar_TestFlag(c, p_chr_1, ((((216i32) as i64 as u64)) as u32))) != 0) {
        { let a0_ = (c.m.r32(p_chr_1) as i32); F::DemoCam_PlayCharAnim1(c, a0_) };
    }
    if ((F::BtlChar_TestFlag(c, p_chr_1, ((((217i32) as i64 as u64)) as u32))) != 0) {
        { let a0_ = (c.m.r32(p_chr_1) as i32); F::DemoCam_PlayCharAnim2(c, a0_) };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ChrCam_UpdateDemo", &PROBE_N, &[], true); }
    r_
}

pub fn ChrCam_SetCut(c: &mut Ctx, mut p_chr_1: u32, mut p_vecA_2: u32, mut p_vecADelta_3: u32, mut p_vecB_4: u32, mut p_vecBDelta_5: u32, mut p_vecC_6: u32, mut p_vecCDelta_7: u32, mut p_unk88_8: i32, mut p_valA_9: F32, mut p_valADelta_10: F32, mut p_valB_11: F32, mut p_valBDelta_12: F32, mut p_valC_13: F32, mut p_valCDelta_14: F32, mut p_unk8C_15: i32, mut p_unk90_16: i32, mut p_unk94_17: i32, mut p_time_18: i32, mut p_flags_19: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ChrCam_SetCut", &PROBE_N, &[(p_chr_1) as u32, (p_vecA_2) as u32, (p_vecADelta_3) as u32, (p_vecB_4) as u32, (p_vecBDelta_5) as u32, (p_vecC_6) as u32, (p_vecCDelta_7) as u32, (p_unk88_8) as u32, (p_valA_9).0, (p_valADelta_10).0, (p_valB_11).0, (p_valBDelta_12).0, (p_valC_13).0, (p_valCDelta_14).0, (p_unk8C_15) as u32, (p_unk90_16) as u32, (p_unk94_17) as u32, (p_time_18) as u32, (p_flags_19) as u32], false); }
    let mut l_cut_20: u32 = 0;
    let r_ = 'f: {
    l_cut_20 = (p_chr_1).wrapping_add(1216);
    F::Vec4_Copy(c, ((p_chr_1).wrapping_add(1216)).wrapping_add(16), p_vecA_2);
    F::Vec4_Copy(c, ((p_chr_1).wrapping_add(1216)).wrapping_add(32), p_vecADelta_3);
    F::Vec4_Copy(c, ((p_chr_1).wrapping_add(1216)).wrapping_add(48), p_vecB_4);
    F::Vec4_Copy(c, ((p_chr_1).wrapping_add(1216)).wrapping_add(64), p_vecBDelta_5);
    F::Vec4_Copy(c, ((p_chr_1).wrapping_add(1216)).wrapping_add(80), p_vecC_6);
    F::Vec4_Copy(c, ((p_chr_1).wrapping_add(1216)).wrapping_add(96), p_vecCDelta_7);
    { let v_ = p_unk88_8; let a_ = (l_cut_20).wrapping_add(136); c.m.w32(a_, (v_) as u32); };
    { let v_ = p_unk8C_15; let a_ = (l_cut_20).wrapping_add(140); c.m.w32(a_, (v_) as u32); };
    { let v_ = p_unk90_16; let a_ = (l_cut_20).wrapping_add(144); c.m.w32(a_, (v_) as u32); };
    { let v_ = p_unk94_17; let a_ = (l_cut_20).wrapping_add(148); c.m.w32(a_, (v_) as u32); };
    { let v_ = p_valA_9; let a_ = (l_cut_20).wrapping_add(112); c.m.w32(a_, (v_).0); };
    { let v_ = p_valADelta_10; let a_ = (l_cut_20).wrapping_add(116); c.m.w32(a_, (v_).0); };
    { let v_ = p_valB_11; let a_ = (l_cut_20).wrapping_add(120); c.m.w32(a_, (v_).0); };
    { let v_ = p_valBDelta_12; let a_ = (l_cut_20).wrapping_add(124); c.m.w32(a_, (v_).0); };
    { let v_ = p_valC_13; let a_ = (l_cut_20).wrapping_add(128); c.m.w32(a_, (v_).0); };
    { let v_ = p_valCDelta_14; let a_ = (l_cut_20).wrapping_add(132); c.m.w32(a_, (v_).0); };
    { let v_ = p_time_18; let a_ = (l_cut_20).wrapping_add(152); c.m.w32(a_, (v_) as u32); };
    { let v_ = p_time_18; let a_ = (l_cut_20).wrapping_add(156); c.m.w32(a_, (v_) as u32); };
    { let v_ = p_flags_19; let a_ = (l_cut_20).wrapping_add(160); c.m.w32(a_, (v_) as u32); };
    if ((((p_flags_19) & (1i32))) != 0) {
        F::BtlChar_SetFlag(c, p_chr_1, ((((205i32) as i64 as u64)) as u32));
    }
    if ((((p_flags_19) & (8i32))) != 0) {
        F::BtlChar_SetFlag(c, p_chr_1, ((((209i32) as i64 as u64)) as u32));
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ChrCam_SetCut", &PROBE_N, &[], true); }
    r_
}

pub fn ChrCam_BlendToCut(c: &mut Ctx, mut p_chr_1: u32, mut p_vecADelta_2: u32, mut p_vecBDelta_3: u32, mut p_vecCDelta_4: u32, mut p_unk8C_5: i32, mut p_unk94_6: i32, mut p_valADelta_7: F32, mut p_valBDelta_8: F32, mut p_valCDelta_9: F32, mut p_time_10: i32, mut p_flags_11: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ChrCam_BlendToCut", &PROBE_N, &[(p_chr_1) as u32, (p_vecADelta_2) as u32, (p_vecBDelta_3) as u32, (p_vecCDelta_4) as u32, (p_unk8C_5) as u32, (p_unk94_6) as u32, (p_valADelta_7).0, (p_valBDelta_8).0, (p_valCDelta_9).0, (p_time_10) as u32, (p_flags_11) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(64) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 64);
    let mut l_valA_12: F32 = F32(0);
    let mut l_valB_13: F32 = F32(0);
    let mut l_valC_14: F32 = F32(0);
    let mut l_pitch_15: F32 = F32(0);
    let mut l_cut_16: u32 = 0;
    let mut l_cam_17: u32 = 0;
    let mut l_pb_18: u32 = 0;
    let mut l_pc_19: u32 = 0;
    let mut l_unk88_20: i32 = 0;
    let mut l_unk90_21: i32 = 0;
    let mut l_t_22: F32 = F32(0);
    let r_ = 'f: {
        l_cut_16 = (p_chr_1).wrapping_add(1216);
        l_t_22 = F32(0x00000000);
        if ((((((((((c.m.r32((l_cut_16).wrapping_add(152)) as i32)) > (0i32)) as i32)) != 0) || (((((c.m.r32((l_cut_16).wrapping_add(160)) as i32)) & (128i32))) != 0)) as i32)) != 0) {
            if ((((((c.m.r32((l_cut_16).wrapping_add(156)) as i32)) > (0i32)) as i32)) != 0) {
                l_t_22 = rt::fsub(F32(0x3f800000), rt::fdiv(rt::i2f((c.m.r32((l_cut_16).wrapping_add(152)) as i32)), rt::i2f((c.m.r32((l_cut_16).wrapping_add(156)) as i32))));
            }
            F::Vec4_Scale(c, fp, ((p_chr_1).wrapping_add(1216)).wrapping_add(32), l_t_22);
            F::Vec4_Add(c, fp, ((p_chr_1).wrapping_add(1216)).wrapping_add(16), fp);
            l_pb_18 = fp.wrapping_add(16);
            F::Vec4_Scale(c, l_pb_18, ((p_chr_1).wrapping_add(1216)).wrapping_add(64), l_t_22);
            F::Vec4_Add(c, l_pb_18, ((p_chr_1).wrapping_add(1216)).wrapping_add(48), l_pb_18);
            l_pc_19 = fp.wrapping_add(32);
            F::Vec4_Scale(c, l_pc_19, ((p_chr_1).wrapping_add(1216)).wrapping_add(96), l_t_22);
            F::Vec4_Add(c, l_pc_19, ((p_chr_1).wrapping_add(1216)).wrapping_add(80), l_pc_19);
            l_unk88_20 = (c.m.r32((l_cut_16).wrapping_add(136)) as i32);
            l_valA_12 = rt::fadd(F32(c.m.r32((l_cut_16).wrapping_add(112))), rt::fmul(F32(c.m.r32((l_cut_16).wrapping_add(116))), l_t_22));
            l_valB_13 = rt::fadd(F32(c.m.r32((l_cut_16).wrapping_add(120))), rt::fmul(F32(c.m.r32((l_cut_16).wrapping_add(124))), l_t_22));
            l_unk90_21 = (c.m.r32((l_cut_16).wrapping_add(144)) as i32);
            l_valC_14 = rt::fadd(F32(c.m.r32((l_cut_16).wrapping_add(128))), rt::fmul(F32(c.m.r32((l_cut_16).wrapping_add(132))), l_t_22));
        } else {
            l_cam_17 = (p_chr_1).wrapping_add(1056);
            l_unk90_21 = (1i32).wrapping_neg();
            l_unk88_20 = (1i32).wrapping_neg();
            l_pb_18 = fp.wrapping_add(16);
            l_valB_13 = F32(c.m.r32((l_cam_17).wrapping_add(32)));
            l_valA_12 = F32(c.m.r32(((l_cam_17).wrapping_add(32)).wrapping_add(4)));
            F::Vec4_Copy(c, l_pb_18, ((p_chr_1).wrapping_add(1056)).wrapping_add(48));
            l_pitch_15 = rt::fneg(l_valB_13);
            F::Vec4_Sub(c, fp.wrapping_add(48), ((p_chr_1).wrapping_add(1056)).wrapping_add(48), l_cam_17);
            l_valC_14 = F::Vec3_Length(c, fp.wrapping_add(48));
            { let v_ = rt::fmul(F::Mathf_Cos(c, l_pitch_15), F::Mathf_Sin(c, l_valA_12)); let a_ = fp.wrapping_add(48); c.m.w32(a_, (v_).0); };
            { let v_ = F::Mathf_Sin(c, l_pitch_15); let a_ = (fp.wrapping_add(48)).wrapping_add(4); c.m.w32(a_, (v_).0); };
            { let v_ = rt::fmul(F::Mathf_Cos(c, l_pitch_15), F::Mathf_Cos(c, l_valA_12)); let a_ = (fp.wrapping_add(48)).wrapping_add(8); c.m.w32(a_, (v_).0); };
            F::Vec4_Scale(c, fp.wrapping_add(48), fp.wrapping_add(48), l_valC_14);
            F::Vec4_Add(c, fp, l_cam_17, fp.wrapping_add(48));
            l_pc_19 = fp.wrapping_add(32);
            F::Vec4_Copy(c, l_pc_19, fp);
        }
        { let r_ = (!(9i32)); let n_ = ((p_flags_11) & (r_)); p_flags_11 = n_; };
        F::ChrCam_SetCut(c, p_chr_1, fp, p_vecADelta_2, l_pb_18, p_vecBDelta_3, l_pc_19, p_vecCDelta_4, l_unk88_20, l_valA_12, p_valADelta_7, l_valB_13, p_valBDelta_8, l_valC_14, p_valCDelta_9, p_unk8C_5, l_unk90_21, p_unk94_6, p_time_10, p_flags_11);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "ChrCam_BlendToCut", &PROBE_N, &[], true); }
    r_
}

pub fn ChrCam_RequestCut(c: &mut Ctx, mut p_chr_1: u32, mut p_table_2: i32, mut p_index_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ChrCam_RequestCut", &PROBE_N, &[(p_chr_1) as u32, (p_table_2) as u32, (p_index_3) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_table_2; let a_ = (p_chr_1).wrapping_add(1216); c.m.w32(a_, (v_) as u32); };
    { let v_ = p_index_3; let a_ = ((p_chr_1).wrapping_add(1216)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
    F::BtlChar_SetFlag(c, p_chr_1, ((((219i32) as i64 as u64)) as u32));
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ChrCam_RequestCut", &PROBE_N, &[], true); }
    r_
}

pub fn ChrCam_StartCut(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ChrCam_StartCut", &PROBE_N, &[(p_chr_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(240) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 240);
    let mut l_def_2: u32 = 0;
    let mut l_unk88_3: i32 = 0;
    let mut l_obj_4: u32 = 0;
    let mut l_flags_5: u16 = 0;
    let mut l_unk8C_6: i32 = 0;
    let mut l_unk90_7: i32 = 0;
    let mut l_unk94_8: i32 = 0;
    let mut l_yaw_9: F32 = F32(0);
    let mut l_scale_10: F32 = F32(0);
    let mut l_pi_11: F32 = F32(0);
    let mut l_half_12: F32 = F32(0);
    let mut l_valA_13: F32 = F32(0);
    let mut l_valADelta_14: F32 = F32(0);
    let mut l_valB_15: F32 = F32(0);
    let mut l_valBDelta_16: F32 = F32(0);
    let mut l_valC_17: F32 = F32(0);
    let mut l_valCDelta_18: F32 = F32(0);
    let mut l_time_19: i32 = 0;
    let mut l_cutFlags_20: i32 = 0;
    let r_ = 'f: {
        l_def_2 = ((0i32) as u32);
        l_obj_4 = F::BtlChar_GetObj(c, p_chr_1);
        if ((((!((F::BtlChar_TestFlag(c, p_chr_1, ((((219i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) {
            break 'f;
        }
        's1: {
            let sv21: i32 = (c.m.r32((p_chr_1).wrapping_add(1216)) as i32);
            match sv21 {
                0i32 => {
                    l_def_2 = (c.m.r32((l_obj_4).wrapping_add(((2360i32) as u32)))).wrapping_add(((((c.m.r32(((p_chr_1).wrapping_add(1216)).wrapping_add(4)) as i32)) as u32)).wrapping_mul(36));
                    break 's1;
                }
                1i32 => {
                    l_def_2 = ({ let a_ = (c.m.r32(sym::gBtlChars)).wrapping_add(36); c.m.r32(a_) }).wrapping_add(((((c.m.r32(((p_chr_1).wrapping_add(1216)).wrapping_add(4)) as i32)) as u32)).wrapping_mul(36));
                    break 's1;
                }
                _ => {
                }
            }
        }
        if (((((l_def_2) == (((0i32) as u32))) as i32)) != 0) {
            break 'f;
        }
        l_flags_5 = c.m.r16((l_def_2).wrapping_add(34));
        if ((((((l_flags_5) as i32)) & (2i32))) != 0) {
            F::BtlOpp_GetTargetRot(c, p_chr_1, fp.wrapping_add(80));
            l_yaw_9 = F32(c.m.r32((fp.wrapping_add(80)).wrapping_add(4)));
            l_scale_10 = F::BtlOpp_GetHeight(c, p_chr_1);
            if ((((((l_flags_5) as i32)) & (1i32))) != 0) {
                F::Vec4_SetZeroW1(c, fp);
                F::Vec4_SetZeroW1(c, fp.wrapping_add(16));
                l_unk88_3 = ((((c.m.r8(l_def_2)) as i32)) | (1073741824i32));
                l_unk8C_6 = ((((c.m.r8((l_def_2).wrapping_add(1))) as i32)) | (1073741824i32));
            } else {
                l_unk88_3 = (1i32).wrapping_neg();
                { let a1_ = ((c.m.r8(l_def_2)) as i32); F::BtlOpp_GetNodePos(c, p_chr_1, a1_, fp) };
                { let a1_ = ((c.m.r8((l_def_2).wrapping_add(1))) as i32); F::BtlOpp_GetNodePos(c, p_chr_1, a1_, fp.wrapping_add(96)) };
                F::Vec4_Sub(c, fp.wrapping_add(16), fp.wrapping_add(96), fp);
                l_unk8C_6 = (1i32).wrapping_neg();
            }
            { let a1_ = { let a_ = (F::BtlOpp_GetObj(c, p_chr_1)).wrapping_add(((4000i32) as u32)); c.m.r32(a_) }; F::Vec4_Copy(c, fp.wrapping_add(32), a1_) };
        } else {
            if ((((((l_flags_5) as i32)) & (128i32))) != 0) {
                l_yaw_9 = { let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(148); F32(c.m.r32(a_)) };
                { let v_ = { let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); F::BtlCharApi_GetHeight(c, a0_) }; let a_ = (fp.wrapping_add(160)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                { let v_ = F::BtlOpp_GetHeight(c, p_chr_1); let a_ = (fp.wrapping_add(160)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                l_scale_10 = (if (((rt::flt(F32(c.m.r32((fp.wrapping_add(160)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((fp.wrapping_add(160)).wrapping_add((((0i32) as u32)).wrapping_mul(4))))) as i32)) != 0) { F32(c.m.r32((fp.wrapping_add(160)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))) } else { F32(c.m.r32((fp.wrapping_add(160)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))) });
                l_unk88_3 = (1i32).wrapping_neg();
                { let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); let a1_ = ((c.m.r8(l_def_2)) as i32); F::BtlCharApi_GetNodePos(c, a0_, a1_, fp.wrapping_add(112)) };
                l_half_12 = F32(0x3f000000);
                { let a1_ = ((c.m.r8(l_def_2)) as i32); F::BtlOpp_GetNodePos(c, p_chr_1, a1_, fp.wrapping_add(128)) };
                F::Vec4_Lerp(c, fp, fp.wrapping_add(112), fp.wrapping_add(128), l_half_12);
                l_unk8C_6 = (1i32).wrapping_neg();
                { let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); let a1_ = ((c.m.r8((l_def_2).wrapping_add(1))) as i32); F::BtlCharApi_GetNodePos(c, a0_, a1_, fp.wrapping_add(112)) };
                { let a1_ = ((c.m.r8((l_def_2).wrapping_add(1))) as i32); F::BtlOpp_GetNodePos(c, p_chr_1, a1_, fp.wrapping_add(128)) };
                F::Vec4_Lerp(c, fp.wrapping_add(144), fp.wrapping_add(112), fp.wrapping_add(128), l_half_12);
                F::Vec4_Sub(c, fp.wrapping_add(16), fp.wrapping_add(144), fp);
                F::Vec4_Copy(c, fp.wrapping_add(32), fp);
                if ((((((l_flags_5) as i32)) & (1i32))) != 0) {
                    F::Vec4_SetZeroW1(c, fp);
                    F::Vec4_SetZeroW1(c, fp.wrapping_add(16));
                    l_unk88_3 = ((((c.m.r8(l_def_2)) as i32)) | (536870912i32));
                    l_unk8C_6 = ((((c.m.r8((l_def_2).wrapping_add(1))) as i32)) | (536870912i32));
                }
            } else {
                l_yaw_9 = { let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(148); F32(c.m.r32(a_)) };
                l_scale_10 = { let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); F::BtlCharApi_GetHeight(c, a0_) };
                if ((((((l_flags_5) as i32)) & (1i32))) != 0) {
                    F::Vec4_SetZeroW1(c, fp);
                    F::Vec4_SetZeroW1(c, fp.wrapping_add(16));
                    l_unk88_3 = ((c.m.r8(l_def_2)) as i32);
                    l_unk8C_6 = ((c.m.r8((l_def_2).wrapping_add(1))) as i32);
                } else {
                    l_unk88_3 = (1i32).wrapping_neg();
                    { let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); let a1_ = ((c.m.r8(l_def_2)) as i32); F::BtlCharApi_GetNodePos(c, a0_, a1_, fp) };
                    l_unk8C_6 = (1i32).wrapping_neg();
                    { let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); let a1_ = ((c.m.r8((l_def_2).wrapping_add(1))) as i32); F::BtlCharApi_GetNodePos(c, a0_, a1_, fp.wrapping_add(168)) };
                    F::Vec4_Sub(c, fp.wrapping_add(16), fp.wrapping_add(168), fp);
                }
                { let a1_ = { let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(((4000i32) as u32)); c.m.r32(a_) }; F::Vec4_Copy(c, fp.wrapping_add(32), a1_) };
            }
        }
        if ((((((l_flags_5) as i32)) & (8i32))) != 0) {
            if ((((((l_flags_5) as i32)) & (4i32))) != 0) {
                F::Vec4_SetZeroW1(c, fp.wrapping_add(48));
                l_unk90_7 = ((((c.m.r8((l_def_2).wrapping_add(2))) as i32)) | (1073741824i32));
            } else {
                { let a1_ = ((c.m.r8((l_def_2).wrapping_add(2))) as i32); F::BtlOpp_GetNodePos(c, p_chr_1, a1_, fp.wrapping_add(48)) };
                l_unk90_7 = (1i32).wrapping_neg();
            }
        } else {
            if ((((((l_flags_5) as i32)) & (256i32))) != 0) {
                if ((((((l_flags_5) as i32)) & (4i32))) != 0) {
                    F::Vec4_SetZeroW1(c, fp.wrapping_add(48));
                    l_unk90_7 = ((((c.m.r8((l_def_2).wrapping_add(2))) as i32)) | (536870912i32));
                } else {
                    { let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); F::BtlCharApi_GetNodePos(c, a0_, 3i32, fp.wrapping_add(184)) };
                    F::BtlOpp_GetNodePos(c, p_chr_1, 3i32, fp.wrapping_add(200));
                    l_unk90_7 = (1i32).wrapping_neg();
                    F::Vec4_Lerp(c, fp.wrapping_add(48), fp.wrapping_add(184), fp.wrapping_add(200), F32(0x3f000000));
                }
            } else {
                if ((((((l_flags_5) as i32)) & (4i32))) != 0) {
                    F::Vec4_SetZeroW1(c, fp.wrapping_add(48));
                    l_unk90_7 = ((c.m.r8((l_def_2).wrapping_add(2))) as i32);
                } else {
                    { let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); let a1_ = ((c.m.r8((l_def_2).wrapping_add(2))) as i32); F::BtlCharApi_GetNodePos(c, a0_, a1_, fp.wrapping_add(48)) };
                    l_unk90_7 = (1i32).wrapping_neg();
                }
            }
        }
        if ((((((l_flags_5) as i32)) & (32i32))) != 0) {
            if ((((((l_flags_5) as i32)) & (16i32))) != 0) {
                F::Vec4_SetZeroW1(c, fp.wrapping_add(64));
                l_unk94_8 = ((((c.m.r8((l_def_2).wrapping_add(3))) as i32)) | (1073741824i32));
            } else {
                { let a1_ = ((c.m.r8((l_def_2).wrapping_add(3))) as i32); F::BtlOpp_GetNodePos(c, p_chr_1, a1_, fp.wrapping_add(184)) };
                F::Vec4_Sub(c, fp.wrapping_add(64), fp.wrapping_add(184), fp.wrapping_add(48));
                l_unk94_8 = (1i32).wrapping_neg();
            }
        } else {
            if ((((((l_flags_5) as i32)) & (512i32))) != 0) {
                if ((((((l_flags_5) as i32)) & (16i32))) != 0) {
                    F::Vec4_SetZeroW1(c, fp.wrapping_add(64));
                    l_unk94_8 = ((((c.m.r8((l_def_2).wrapping_add(3))) as i32)) | (536870912i32));
                } else {
                    { let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); F::BtlCharApi_GetNodePos(c, a0_, 3i32, fp.wrapping_add(184)) };
                    F::BtlOpp_GetNodePos(c, p_chr_1, 3i32, fp.wrapping_add(200));
                    l_unk94_8 = (1i32).wrapping_neg();
                    F::Vec4_Lerp(c, fp.wrapping_add(216), fp.wrapping_add(184), fp.wrapping_add(200), F32(0x3f000000));
                    F::Vec4_Sub(c, fp.wrapping_add(64), fp.wrapping_add(216), fp.wrapping_add(48));
                }
            } else {
                if ((((((l_flags_5) as i32)) & (16i32))) != 0) {
                    F::Vec4_SetZeroW1(c, fp.wrapping_add(64));
                    l_unk94_8 = ((c.m.r8((l_def_2).wrapping_add(3))) as i32);
                } else {
                    { let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); let a1_ = ((c.m.r8((l_def_2).wrapping_add(3))) as i32); F::BtlCharApi_GetNodePos(c, a0_, a1_, fp.wrapping_add(184)) };
                    l_unk94_8 = (1i32).wrapping_neg();
                    F::Vec4_Sub(c, fp.wrapping_add(64), fp.wrapping_add(184), fp.wrapping_add(48));
                }
            }
        }
        if ((((((l_flags_5) as i32)) & (1024i32))) != 0) {
            l_scale_10 = F::BtlUtil_MaxF(c, l_scale_10, F32(0x41700000));
        }
        l_pi_11 = F32(0x40490fda);
        l_valA_13 = { let a0_ = rt::fadd(l_yaw_9, rt::fdiv(rt::fmul(F32(c.m.r32((l_def_2).wrapping_add(4))), l_pi_11), F32(0x43340000))); F::BtlUtil_WrapAngle(c, a0_) };
        l_valADelta_14 = rt::fdiv(rt::fmul(F32(c.m.r32((l_def_2).wrapping_add(8))), l_pi_11), F32(0x43340000));
        l_valB_15 = rt::fdiv(rt::fmul(F32(c.m.r32((l_def_2).wrapping_add(12))), l_pi_11), F32(0x43340000));
        l_valBDelta_16 = rt::fdiv(rt::fmul(F32(c.m.r32((l_def_2).wrapping_add(16))), l_pi_11), F32(0x43340000));
        l_valC_17 = rt::fmul(F32(c.m.r32((l_def_2).wrapping_add(20))), l_scale_10);
        l_valCDelta_18 = rt::fmul(F32(c.m.r32((l_def_2).wrapping_add(24))), l_scale_10);
        l_time_19 = rt::f2i(rt::fmul(F32(c.m.r32((l_def_2).wrapping_add(28))), F32(0x41f00000)));
        if (((((l_time_19) <= (0i32)) as i32)) != 0) {
            l_time_19 = 1i32;
        }
        l_cutFlags_20 = ((c.m.r16((l_def_2).wrapping_add(32))) as i32);
        if ((((((l_flags_5) as i32)) & (64i32))) != 0) {
            F::ChrCam_BlendToCut(c, p_chr_1, fp.wrapping_add(16), sym::D_002EC2A0, fp.wrapping_add(64), l_unk8C_6, l_unk94_8, l_valADelta_14, l_valBDelta_16, l_valCDelta_18, l_time_19, l_cutFlags_20);
        } else {
            F::ChrCam_SetCut(c, p_chr_1, fp, fp.wrapping_add(16), fp.wrapping_add(32), sym::D_002EC2A0, fp.wrapping_add(48), fp.wrapping_add(64), l_unk88_3, l_valA_13, l_valADelta_14, l_valB_15, l_valBDelta_16, l_valC_17, l_valCDelta_18, l_unk8C_6, l_unk90_7, l_unk94_8, l_time_19, l_cutFlags_20);
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "ChrCam_StartCut", &PROBE_N, &[], true); }
    r_
}

pub fn ChrCam_EndCut(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ChrCam_EndCut", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_cut_2: u32 = 0;
    let r_ = 'f: {
    l_cut_2 = (p_chr_1).wrapping_add(1216);
    if (((((c.m.r32((l_cut_2).wrapping_add(160)) as i32)) & (4i32))) != 0) {
        F::BtlChar_SetFlag(c, p_chr_1, ((((205i32) as i64 as u64)) as u32));
    }
    if (((((c.m.r32((l_cut_2).wrapping_add(160)) as i32)) & (32i32))) != 0) {
        F::BtlChar_SetFlag(c, p_chr_1, ((((209i32) as i64 as u64)) as u32));
    }
    { let v_ = 0i32; let a_ = (l_cut_2).wrapping_add(152); c.m.w32(a_, (v_) as u32); };
    { let v_ = 0i32; let a_ = (l_cut_2).wrapping_add(160); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ChrCam_EndCut", &PROBE_N, &[], true); }
    r_
}

pub fn ChrCam_IsCutActive(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ChrCam_IsCutActive", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_cut_2: u32 = 0;
    let r_ = 'f: {
    l_cut_2 = (p_chr_1).wrapping_add(1216);
    if ((((((((((c.m.r32((l_cut_2).wrapping_add(152)) as i32)) > (0i32)) as i32)) != 0) || (((((c.m.r32((l_cut_2).wrapping_add(160)) as i32)) & (128i32))) != 0)) as i32)) != 0) {
        break 'f 1i32;
    }
    break 'f (((F::BtlChar_TestFlag(c, p_chr_1, ((((219i32) as i64 as u64)) as u32))) != (0i32)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "ChrCam_IsCutActive", &PROBE_N, &[], true); }
    r_
}
