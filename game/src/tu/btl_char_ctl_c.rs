#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn BtlChar_ResetLook(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_ResetLook", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_look_2: u32 = 0;
    let mut l_obj_3: u32 = 0;
    let r_ = 'f: {
    l_look_2 = (p_chr_1).wrapping_add(5536);
    l_obj_3 = F::BtlChar_GetObj(c, p_chr_1);
    F::Quat_SetIdentity(c, (l_obj_3).wrapping_add(3376));
    F::Quat_SetIdentity(c, (l_obj_3).wrapping_add(3392));
    { let v_ = F32(0x00000000); let a_ = (l_obj_3).wrapping_add(3408); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x00000000); let a_ = (l_obj_3).wrapping_add(3412); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fmul({ let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); F::BtlCharApi_GetHeight(c, a0_) }, F32(0x3f333333)); let a_ = (l_look_2).wrapping_add(32); c.m.w32(a_, (v_).0); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_ResetLook", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChar_UpdateLookOffset(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_UpdateLookOffset", &PROBE_N, &[(p_chr_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_look_2: u32 = 0;
    let mut l_obj_3: u32 = 0;
    let mut l_node_4: u32 = 0;
    let r_ = 'f: {
        l_look_2 = (p_chr_1).wrapping_add(5536);
        l_obj_3 = F::BtlChar_GetObj(c, p_chr_1);
        if ((F::BtlChars_IsTimeStopped(c)) != 0) {
            F::BtlChar_ResetLook(c, p_chr_1);
        } else {
            l_node_4 = F::BtlObj_GetNode(c, l_obj_3, 47i32);
            { let r_ = rt::fmul(rt::fsub(rt::fsub(F32(c.m.r32(((l_obj_3).wrapping_add(2416)).wrapping_add(4))), F32(c.m.r32(((l_node_4).wrapping_add(64)).wrapping_add(4)))), F32(c.m.r32((l_look_2).wrapping_add(32)))), F32(0x3e4ccccc)); let a_ = (l_look_2).wrapping_add(32); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            F::Vec4_Sub(c, fp.wrapping_add(16), (l_obj_3).wrapping_add(2416), (l_node_4).wrapping_add(64));
            F::Vec4_Sub(c, fp, fp.wrapping_add(16), l_look_2);
            F::Vec4_Scale(c, fp, fp, F32(0x3e4ccccc));
            F::Vec4_Add(c, l_look_2, l_look_2, fp);
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_UpdateLookOffset", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChar_SetLookEnabled(c: &mut Ctx, mut p_chr_1: u32, mut p_enabled_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_SetLookEnabled", &PROBE_N, &[(p_chr_1) as u32, (p_enabled_2) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_enabled_2; let a_ = ((p_chr_1).wrapping_add(5536)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_SetLookEnabled", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChar_UpdateLook(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_UpdateLook", &PROBE_N, &[(p_chr_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(352) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 352);
    let mut l_head_2: u32 = 0;
    let mut l_neck_3: u32 = 0;
    let mut l_full_4: i32 = 0;
    let mut l_obj_5: u32 = 0;
    let mut l_look_6: u32 = 0;
    let mut l_active_7: i32 = 0;
    let mut l_headPitch_8: F32 = F32(0);
    let mut l_neckPitch_9: F32 = F32(0);
    let mut l_blend_10: F32 = F32(0);
    let mut l_angle_11: F32 = F32(0);
    let mut l_limA_12: F32 = F32(0);
    let mut l_limB_13: F32 = F32(0);
    let mut l_limC_14: F32 = F32(0);
    let mut l_down_15: F32 = F32(0);
    let mut l_i_16: i32 = 0;
    let mut l_pitch_17: F32 = F32(0);
    let r_ = 'f: {
        l_look_6 = (p_chr_1).wrapping_add(5536);
        l_obj_5 = F::BtlChar_GetObj(c, p_chr_1);
        l_neckPitch_9 = rt::fdiv(rt::fmul(rt::i2f((({ let a_ = (c.m.r32((l_obj_5).wrapping_add(2368))).wrapping_add(851); (c.m.r8(a_) as i8) }) as i32)), F32(0x40490fda)), F32(0x43340000));
        l_headPitch_8 = rt::fdiv(rt::fmul(rt::i2f((({ let a_ = (c.m.r32((l_obj_5).wrapping_add(2368))).wrapping_add(852); (c.m.r8(a_) as i8) }) as i32)), F32(0x40490fda)), F32(0x43340000));
        l_full_4 = 1i32;
        l_active_7 = 1i32;
        if ((((!((F::BtlChar_TestFlag(c, p_chr_1, ((((5i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) {
            l_full_4 = 0i32;
            l_active_7 = 0i32;
        }
        if ((((((c.m.r32((l_look_6).wrapping_add(36)) as i32)) == (0i32)) as i32)) != 0) {
            l_full_4 = 0i32;
            l_active_7 = 1i32;
        }
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((172i32) as i64 as u64)) as u32))) != 0) {
            l_full_4 = 0i32;
            l_active_7 = 0i32;
        }
        l_head_2 = F::BtlObj_GetNode(c, l_obj_5, 46i32);
        l_neck_3 = F::BtlObj_GetNode(c, l_obj_5, 47i32);
        l_blend_10 = F32(0x00000000);
        F::Vec4_Copy(c, fp, (l_head_2).wrapping_add(160));
        F::Vec4_Copy(c, fp.wrapping_add(16), (l_neck_3).wrapping_add(160));
        F::Mtx_Copy(c, fp.wrapping_add(32), (l_head_2).wrapping_add(80));
        F::Mtx_StoreIdentity(c, fp.wrapping_add(176));
        F::Mtx_RotateX(c, fp.wrapping_add(176), fp.wrapping_add(176), l_neckPitch_9);
        F::Mtx_Mul(c, fp.wrapping_add(32), fp.wrapping_add(32), fp.wrapping_add(176));
        F::Mtx_InverseRT(c, fp.wrapping_add(96), fp.wrapping_add(32));
        F::BtlOpp_GetDelta(c, p_chr_1, fp.wrapping_add(240));
        F::Vec4_Add(c, fp.wrapping_add(240), fp.wrapping_add(240), l_look_6);
        F::BtlOpp_GetUnk15A0(c, p_chr_1, fp.wrapping_add(160));
        F::Vec4_Sub(c, fp.wrapping_add(240), fp.wrapping_add(240), fp.wrapping_add(160));
        F::Vec3_Normalize(c, fp.wrapping_add(240), fp.wrapping_add(240));
        { let v_ = l_blend_10; let a_ = (fp.wrapping_add(240)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        if (((c.m.r32((l_look_6).wrapping_add(44)) as i32)) != 0) {
            F::Vec4_Copy(c, (l_look_6).wrapping_add(16), fp.wrapping_add(240));
        } else {
            F::Vec4_Lerp(c, (l_look_6).wrapping_add(16), fp.wrapping_add(240), (l_look_6).wrapping_add(16), F32(0x3e999999));
        }
        { let v_ = F32(0x00000000); let a_ = ((l_look_6).wrapping_add(16)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        if ((l_active_7) != 0) {
            { let a1_ = F32(c.m.r32(((l_look_6).wrapping_add(16)).wrapping_add(12))); let a2_ = F32(c.m.r32(((l_look_6).wrapping_add(16)).wrapping_add(12))); let a4_ = F32(c.m.r32(((l_look_6).wrapping_add(16)).wrapping_add(12))); F::Vec4_Set(c, fp.wrapping_add(272), a1_, a2_, F32(0xbf800000), a4_) };
            F::Mtx_MulVec4(c, fp.wrapping_add(256), fp.wrapping_add(96), (l_look_6).wrapping_add(16));
            F::Vec3_Normalize(c, fp.wrapping_add(256), fp.wrapping_add(256));
            if (((c.m.r32((l_look_6).wrapping_add(40)) as i32)) != 0) {
                l_limB_13 = F32(0xbf800000);
                l_limA_12 = F32(0x3f800000);
                l_limC_14 = F32(0x3fe66666);
            } else {
                l_limA_12 = F32(0x3f199999);
                l_limB_13 = F32(0xbf199999);
                l_limC_14 = F32(0x3fc00000);
            }
            l_angle_11 = { let a0_ = rt::fneg(F32(c.m.r32(fp.wrapping_add(256)))); let a1_ = rt::fneg(F32(c.m.r32((fp.wrapping_add(256)).wrapping_add(8)))); F::Port_atan2f(c, a0_, a1_) };
            l_pitch_17 = rt::fneg({ let a0_ = F32(c.m.r32((fp.wrapping_add(256)).wrapping_add(4))); F::Port_asinf(c, a0_) });
            { let r_ = rt::fmul(F::Port_fabsf(c, l_angle_11), F32(0x3e4ccccc)); let n_ = rt::fadd(l_pitch_17, r_); l_pitch_17 = n_; };
            if (((((((((((((((rt::flt(rt::fneg(l_limC_14), l_angle_11) as i32)) != 0) && (((rt::flt(l_angle_11, l_limC_14) as i32)) != 0)) as i32)) != 0) && (((rt::flt(l_limB_13, l_pitch_17) as i32)) != 0)) as i32)) != 0) && (((rt::flt(l_pitch_17, l_limA_12) as i32)) != 0)) as i32)) != 0) {
                if ((((!(((c.m.r32((l_look_6).wrapping_add(40)) as i32)) != 0)) as i32)) != 0) {
                    F::BtlOpp_GetDelta(c, p_chr_1, fp.wrapping_add(288));
                    l_angle_11 = { let a0_ = F32(c.m.r32(fp.wrapping_add(288))); let a1_ = F32(c.m.r32((fp.wrapping_add(288)).wrapping_add(8))); F::Port_atan2f(c, a0_, a1_) };
                    if (((rt::flt({ let a0_ = { let a0_ = rt::fsub(l_angle_11, { let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(148); F32(c.m.r32(a_)) }); F::BtlUtil_WrapAngle(c, a0_) }; F::Port_fabsf(c, a0_) }, F32(0x3fc90fda)) as i32)) != 0) {
                        { let v_ = 1i32; let a_ = (l_look_6).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
                    }
                }
            } else {
                { let v_ = 0i32; let a_ = (l_look_6).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
            }
            if (((c.m.r32((l_look_6).wrapping_add(40)) as i32)) != 0) {
                l_down_15 = rt::fneg(rt::fdiv(rt::fmul(rt::i2f((({ let a_ = (c.m.r32((l_obj_5).wrapping_add(2368))).wrapping_add(850); c.m.r8(a_) }) as i32)), F32(0x40490fda)), F32(0x43340000)));
                l_limC_14 = rt::fdiv(rt::fmul(rt::i2f((({ let a_ = (c.m.r32((l_obj_5).wrapping_add(2368))).wrapping_add(849); c.m.r8(a_) }) as i32)), F32(0x40490fda)), F32(0x43340000));
                l_limB_13 = rt::fdiv(rt::fmul(rt::i2f((({ let a_ = (c.m.r32((l_obj_5).wrapping_add(2368))).wrapping_add(848); c.m.r8(a_) }) as i32)), F32(0x40490fda)), F32(0x43340000));
                F::Vec4_Set(c, fp.wrapping_add(304), F32(0x00000000), F32(0x3f800000), F32(0x00000000), F32(0x00000000));
                F::Vec3_Cross(c, fp.wrapping_add(288), fp.wrapping_add(256), fp.wrapping_add(304));
                F::Vec3_Normalize(c, fp.wrapping_add(288), fp.wrapping_add(288));
                l_angle_11 = rt::fneg({ let a0_ = F32(c.m.r32((fp.wrapping_add(256)).wrapping_add(4))); F::Port_asinf(c, a0_) });
                { let r_ = rt::fmul(rt::fmul({ let a0_ = { let a0_ = rt::fneg(F32(c.m.r32(fp.wrapping_add(256)))); let a1_ = rt::fneg(F32(c.m.r32((fp.wrapping_add(256)).wrapping_add(8)))); F::Port_atan2f(c, a0_, a1_) }; F::Port_fabsf(c, a0_) }, F32(c.m.r32((fp.wrapping_add(256)).wrapping_add(4)))), F32(0x3e4ccccc)); let n_ = rt::fsub(l_angle_11, r_); l_angle_11 = n_; };
                if (((rt::flt(l_limC_14, l_angle_11) as i32)) != 0) {
                    F::Vec3_RotateAxis(c, fp.wrapping_add(256), fp.wrapping_add(256), fp.wrapping_add(288), rt::fsub(l_angle_11, l_limC_14));
                }
                if (((rt::flt(l_angle_11, l_down_15) as i32)) != 0) {
                    F::Vec3_RotateAxis(c, fp.wrapping_add(256), fp.wrapping_add(256), fp.wrapping_add(288), rt::fsub(l_angle_11, l_down_15));
                }
                F::Vec4_Copy(c, (fp.wrapping_add(320)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(96)).wrapping_add((((1i32) as u32)).wrapping_mul(16)));
                F::Vec4_Set(c, (fp.wrapping_add(320)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), F32(0x00000000), F32(0x3f800000), F32(0x00000000), F32(0x00000000));
                {
                    l_i_16 = 0i32;
                    'l1: loop {
                        if !(((((l_i_16) < (2i32)) as i32)) != 0) { break 'l1; }
                        'c2: {
                            l_angle_11 = { let a0_ = rt::fneg(F32(c.m.r32(fp.wrapping_add(256)))); let a1_ = rt::fneg(F32(c.m.r32((fp.wrapping_add(256)).wrapping_add(8)))); F::Port_atan2f(c, a0_, a1_) };
                            if (((rt::flt(l_angle_11, rt::fneg(l_limB_13)) as i32)) != 0) {
                                F::Vec3_RotateAxis(c, fp.wrapping_add(256), fp.wrapping_add(256), (fp.wrapping_add(320)).wrapping_add((((l_i_16) as u32)).wrapping_mul(16)), rt::fsub(rt::fneg(l_limB_13), l_angle_11));
                            }
                            if (((rt::flt(l_limB_13, l_angle_11) as i32)) != 0) {
                                F::Vec3_RotateAxis(c, fp.wrapping_add(256), fp.wrapping_add(256), (fp.wrapping_add(320)).wrapping_add((((l_i_16) as u32)).wrapping_mul(16)), rt::fsub(l_limB_13, l_angle_11));
                            }
                        }
                        { let o_ = l_i_16; let n_ = (o_).wrapping_add(1); l_i_16 = n_; };
                    }
                }
                F::Quat_FromVectors(c, fp.wrapping_add(288), fp.wrapping_add(272), fp.wrapping_add(256), F32(0x3f000000));
                F::Quat_FromAxisAngle(c, fp.wrapping_add(304), F32(0x3f800000), F32(0x00000000), F32(0x00000000), rt::fmul(l_headPitch_8, F32(0x3f000000)));
                F::Quat_Mul(c, fp.wrapping_add(288), fp.wrapping_add(288), fp.wrapping_add(304));
                F::Quat_FromAxisAngle(c, fp.wrapping_add(304), F32(0x3f800000), F32(0x00000000), F32(0x00000000), rt::fmul(l_neckPitch_9, F32(0x3f000000)));
                F::Quat_Mul(c, fp.wrapping_add(288), fp.wrapping_add(304), fp.wrapping_add(288));
                F::Vec4_Copy(c, (l_obj_5).wrapping_add(3376), fp.wrapping_add(288));
                F::Vec4_Copy(c, (l_obj_5).wrapping_add(3392), fp.wrapping_add(288));
                { let r_ = (((!(1i32))) as u32); let a_ = (l_head_2).wrapping_add(4); let n_ = ((c.m.r32(a_)) & (r_)); c.m.w32(a_, n_); };
                { let r_ = (((!(1i32))) as u32); let a_ = (l_neck_3).wrapping_add(4); let n_ = ((c.m.r32(a_)) & (r_)); c.m.w32(a_, n_); };
                l_blend_10 = F32(0x3f800000);
                if ((((!((l_full_4) != 0)) as i32)) != 0) {
                    l_blend_10 = F32(0x00000000);
                }
            }
        } else {
            { let v_ = 0i32; let a_ = (l_look_6).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
        }
        if (((c.m.r32((l_look_6).wrapping_add(44)) as i32)) != 0) {
            { let v_ = l_blend_10; let a_ = (l_obj_5).wrapping_add(3408); c.m.w32(a_, (v_).0); };
            { let v_ = l_blend_10; let a_ = (l_obj_5).wrapping_add(3412); c.m.w32(a_, (v_).0); };
        } else {
            { let r_ = rt::fmul(rt::fsub(l_blend_10, F32(c.m.r32((l_obj_5).wrapping_add(3408)))), F32(0x3e999999)); let a_ = (l_obj_5).wrapping_add(3408); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            { let r_ = rt::fmul(rt::fsub(l_blend_10, F32(c.m.r32((l_obj_5).wrapping_add(3412)))), F32(0x3e999999)); let a_ = (l_obj_5).wrapping_add(3412); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_UpdateLook", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChar_UpdateLookAlt(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_UpdateLookAlt", &PROBE_N, &[(p_chr_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(224) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 224);
    let mut l_full_2: i32 = 0;
    let mut l_obj_3: u32 = 0;
    let mut l_look_4: u32 = 0;
    let mut l_active_5: i32 = 0;
    let mut l_head_6: u32 = 0;
    let mut l_neck_7: u32 = 0;
    let mut l_headPitch_8: F32 = F32(0);
    let mut l_blend_9: F32 = F32(0);
    let mut l_yaw_10: F32 = F32(0);
    let mut l_pitch_11: F32 = F32(0);
    let mut l_yawMax_12: F32 = F32(0);
    let mut l_upMax_13: F32 = F32(0);
    let mut l_downMax_14: F32 = F32(0);
    let mut l_dir_15: F32 = F32(0);
    let mut l_up_16: F32 = F32(0);
    let mut l_down_17: F32 = F32(0);
    let mut l_side_18: F32 = F32(0);
    let r_ = 'f: {
        l_look_4 = (p_chr_1).wrapping_add(5536);
        l_obj_3 = F::BtlChar_GetObj(c, p_chr_1);
        l_headPitch_8 = rt::fdiv(rt::fmul(rt::i2f((({ let a_ = (c.m.r32((l_obj_3).wrapping_add(2368))).wrapping_add(852); (c.m.r8(a_) as i8) }) as i32)), F32(0x40490fda)), F32(0x43340000));
        l_full_2 = 1i32;
        l_active_5 = 1i32;
        if ((((!((F::BtlChar_TestFlag(c, p_chr_1, ((((5i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) {
            l_full_2 = 0i32;
            l_active_5 = 0i32;
        }
        if ((((((c.m.r32((l_look_4).wrapping_add(36)) as i32)) == (0i32)) as i32)) != 0) {
            l_full_2 = 0i32;
            l_active_5 = 1i32;
        }
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((172i32) as i64 as u64)) as u32))) != 0) {
            l_full_2 = 0i32;
            l_active_5 = 0i32;
        }
        l_head_6 = F::BtlObj_GetNode(c, l_obj_3, 46i32);
        l_neck_7 = F::BtlObj_GetNode(c, l_obj_3, 47i32);
        F::Vec4_Copy(c, fp, (l_head_6).wrapping_add(160));
        l_blend_9 = F32(0x00000000);
        F::Vec4_Copy(c, fp.wrapping_add(16), (l_neck_7).wrapping_add(160));
        F::Mtx_Copy(c, fp.wrapping_add(32), (l_head_6).wrapping_add(80));
        F::Mtx_InverseRT(c, fp.wrapping_add(96), fp.wrapping_add(32));
        F::BtlOpp_GetDelta(c, p_chr_1, fp.wrapping_add(176));
        F::Vec4_Add(c, fp.wrapping_add(176), fp.wrapping_add(176), l_look_4);
        F::BtlOpp_GetUnk15A0(c, p_chr_1, fp.wrapping_add(160));
        F::Vec4_Sub(c, fp.wrapping_add(176), fp.wrapping_add(176), fp.wrapping_add(160));
        F::Vec3_Normalize(c, fp.wrapping_add(176), fp.wrapping_add(176));
        { let v_ = l_blend_9; let a_ = (fp.wrapping_add(176)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        if (((c.m.r32((l_look_4).wrapping_add(44)) as i32)) != 0) {
            F::Vec4_Copy(c, (l_look_4).wrapping_add(16), fp.wrapping_add(176));
        } else {
            F::Vec4_Lerp(c, (l_look_4).wrapping_add(16), fp.wrapping_add(176), (l_look_4).wrapping_add(16), F32(0x3e999999));
        }
        { let v_ = F32(0x00000000); let a_ = ((l_look_4).wrapping_add(16)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        if ((l_active_5) != 0) {
            F::Mtx_MulVec4(c, fp.wrapping_add(192), fp.wrapping_add(96), (l_look_4).wrapping_add(16));
            F::Vec3_Normalize(c, fp.wrapping_add(192), fp.wrapping_add(192));
            if (((c.m.r32((l_look_4).wrapping_add(40)) as i32)) != 0) {
                l_upMax_13 = F32(0x3fa66666);
                l_downMax_14 = F32(0xbfa66666);
                l_yawMax_12 = F32(0x40200000);
            } else {
                l_upMax_13 = F32(0x3f4ccccc);
                l_downMax_14 = F32(0xbf4ccccc);
                l_yawMax_12 = F32(0x3fe66666);
            }
            l_yaw_10 = { let a0_ = rt::fneg(F32(c.m.r32(fp.wrapping_add(192)))); let a1_ = rt::fneg(F32(c.m.r32((fp.wrapping_add(192)).wrapping_add(8)))); F::Port_atan2f(c, a0_, a1_) };
            l_pitch_11 = rt::fneg({ let a0_ = F32(c.m.r32((fp.wrapping_add(192)).wrapping_add(4))); F::Port_asinf(c, a0_) });
            { let r_ = rt::fmul(F::Port_fabsf(c, l_yaw_10), F32(0x3e4ccccc)); let n_ = rt::fadd(l_pitch_11, r_); l_pitch_11 = n_; };
            if (((((((((((((((rt::flt(rt::fneg(l_yawMax_12), l_yaw_10) as i32)) != 0) && (((rt::flt(l_yaw_10, l_yawMax_12) as i32)) != 0)) as i32)) != 0) && (((rt::flt(l_downMax_14, l_pitch_11) as i32)) != 0)) as i32)) != 0) && (((rt::flt(l_pitch_11, l_upMax_13) as i32)) != 0)) as i32)) != 0) {
                if ((((!(((c.m.r32((l_look_4).wrapping_add(40)) as i32)) != 0)) as i32)) != 0) {
                    F::BtlOpp_GetDelta(c, p_chr_1, fp.wrapping_add(208));
                    l_dir_15 = { let a0_ = F32(c.m.r32(fp.wrapping_add(208))); let a1_ = F32(c.m.r32((fp.wrapping_add(208)).wrapping_add(8))); F::Port_atan2f(c, a0_, a1_) };
                    if (((rt::flt({ let a0_ = { let a0_ = rt::fsub(l_dir_15, { let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(148); F32(c.m.r32(a_)) }); F::BtlUtil_WrapAngle(c, a0_) }; F::Port_fabsf(c, a0_) }, F32(0x3fc90fda)) as i32)) != 0) {
                        { let v_ = 1i32; let a_ = (l_look_4).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
                    }
                }
            } else {
                { let v_ = 0i32; let a_ = (l_look_4).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
            }
            if (((c.m.r32((l_look_4).wrapping_add(40)) as i32)) != 0) {
                l_up_16 = rt::fdiv(rt::fmul(rt::i2f((({ let a_ = (c.m.r32((l_obj_3).wrapping_add(2368))).wrapping_add(849); c.m.r8(a_) }) as i32)), F32(0x40490fda)), F32(0x43340000));
                l_down_17 = rt::fneg(rt::fdiv(rt::fmul(rt::i2f((({ let a_ = (c.m.r32((l_obj_3).wrapping_add(2368))).wrapping_add(850); c.m.r8(a_) }) as i32)), F32(0x40490fda)), F32(0x43340000)));
                l_side_18 = rt::fdiv(rt::fmul(rt::i2f((({ let a_ = (c.m.r32((l_obj_3).wrapping_add(2368))).wrapping_add(848); c.m.r8(a_) }) as i32)), F32(0x40490fda)), F32(0x43340000));
                if (((rt::flt(l_up_16, l_pitch_11) as i32)) != 0) {
                    l_pitch_11 = l_up_16;
                }
                if (((rt::flt(l_pitch_11, l_down_17) as i32)) != 0) {
                    l_pitch_11 = l_down_17;
                }
                if (((rt::flt(l_yaw_10, rt::fneg(l_side_18)) as i32)) != 0) {
                    l_yaw_10 = rt::fneg(l_side_18);
                }
                if (((rt::flt(l_side_18, l_yaw_10) as i32)) != 0) {
                    l_yaw_10 = l_side_18;
                }
                F::Quat_FromAxisAngle(c, (l_obj_3).wrapping_add(3376), F32(0x00000000), F32(0x3f800000), F32(0x00000000), l_yaw_10);
                F::Quat_FromAxisAngle(c, (l_obj_3).wrapping_add(3392), F32(0x3f800000), F32(0x00000000), F32(0x00000000), rt::fsub(l_headPitch_8, l_pitch_11));
                { let r_ = (((!(1i32))) as u32); let a_ = (l_head_6).wrapping_add(4); let n_ = ((c.m.r32(a_)) & (r_)); c.m.w32(a_, n_); };
                { let r_ = (((!(1i32))) as u32); let a_ = (l_neck_7).wrapping_add(4); let n_ = ((c.m.r32(a_)) & (r_)); c.m.w32(a_, n_); };
                l_blend_9 = F32(0x3f800000);
                if ((((!((l_full_2) != 0)) as i32)) != 0) {
                    l_blend_9 = F32(0x00000000);
                }
            }
        } else {
            { let v_ = 0i32; let a_ = (l_look_4).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
        }
        if (((c.m.r32((l_look_4).wrapping_add(44)) as i32)) != 0) {
            { let v_ = l_blend_9; let a_ = (l_obj_3).wrapping_add(3408); c.m.w32(a_, (v_).0); };
            { let v_ = l_blend_9; let a_ = (l_obj_3).wrapping_add(3412); c.m.w32(a_, (v_).0); };
        } else {
            { let r_ = rt::fmul(rt::fsub(l_blend_9, F32(c.m.r32((l_obj_3).wrapping_add(3408)))), F32(0x3e999999)); let a_ = (l_obj_3).wrapping_add(3408); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            { let r_ = rt::fmul(rt::fsub(l_blend_9, F32(c.m.r32((l_obj_3).wrapping_add(3412)))), F32(0x3e999999)); let a_ = (l_obj_3).wrapping_add(3412); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_UpdateLookAlt", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChar_UpdateHead(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_UpdateHead", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    if ((((!((((((((F::BtlParam_GetFlags(c, p_chr_1)) as u64)) as i32)) & (32i32))) != 0)) as i32)) != 0) {
        if ((((({ let a_ = F::BtlMember_GetActive(c, p_chr_1); (c.m.r32(a_) as i32) }) == (153i32)) as i32)) != 0) {
            F::BtlChar_UpdateLookAlt(c, p_chr_1);
        } else {
            F::BtlChar_UpdateLook(c, p_chr_1);
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_UpdateHead", &PROBE_N, &[], true); }
    r_
}
