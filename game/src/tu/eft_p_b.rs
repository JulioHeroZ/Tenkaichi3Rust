#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn eft_p_b__EftPOt_Add(c: &mut Ctx, mut p_p_1: u32, mut p_z_2: i32, mut p_layer_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "eft_p_b__EftPOt_Add", &PROBE_N, &[(p_p_1) as u32, (p_z_2) as u32, (p_layer_3) as u32], false); }
    let mut l_e_4: u32 = 0;
    let r_ = 'f: {
    if (((((p_layer_3) >= (2i32)) as i32)) != 0) {
        { let r_ = 2i32; let n_ = (p_layer_3).wrapping_sub(r_); p_layer_3 = n_; };
    }
    if (((((p_z_2) < (0i32)) as i32)) != 0) {
        l_e_4 = ((c.m.r32(sym::gOtZ)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((p_layer_3) as u32)).wrapping_mul(8));
    } else {
        if (((((p_z_2) >= (4096i32)) as i32)) != 0) {
            l_e_4 = ((c.m.r32(sym::gOtZ)).wrapping_add((((4095i32) as u32)).wrapping_mul(16))).wrapping_add((((p_layer_3) as u32)).wrapping_mul(8));
        } else {
            l_e_4 = ((c.m.r32(sym::gOtZ)).wrapping_add((((p_z_2) as u32)).wrapping_mul(16))).wrapping_add((((p_layer_3) as u32)).wrapping_mul(8));
        }
    }
    { let v_ = p_p_1; let a_ = (c.m.r32((l_e_4).wrapping_add(4))).wrapping_add(4); c.m.w32(a_, v_); };
    { let v_ = p_p_1; let a_ = (l_e_4).wrapping_add(4); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "eft_p_b__EftPOt_Add", &PROBE_N, &[], true); }
    r_
}

pub fn EftGlow_Spline3(c: &mut Ctx, mut p_out_1: u32, mut p_p_2: u32, mut p_t_3: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGlow_Spline3", &PROBE_N, &[(p_out_1) as u32, (p_p_2) as u32, (p_t_3).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(144) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 144);
    let r_ = 'f: {
        F::Vec4_Set(c, (fp).wrapping_add((((0i32) as u32)).wrapping_mul(16)), F32(0x00000000), F32(0x00000000), F32(0x00000000), F32(0x00000000));
        F::Vec4_Set(c, (fp).wrapping_add((((1i32) as u32)).wrapping_mul(16)), F32(0x00000000), F32(0x3f800000), F32(0xc0000000), F32(0x3f800000));
        F::Vec4_Set(c, (fp).wrapping_add((((2i32) as u32)).wrapping_mul(16)), F32(0x00000000), F32(0xc0400000), F32(0x40800000), F32(0xbf800000));
        F::Vec4_Set(c, (fp).wrapping_add((((3i32) as u32)).wrapping_mul(16)), F32(0x00000000), F32(0x40000000), F32(0x00000000), F32(0x00000000));
        F::Vec4_Set(c, (fp.wrapping_add(64)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), F32(0x00000000), F32(0x00000000), F32(0x00000000), F32(0x00000000));
        { let a1_ = F32(c.m.r32((p_p_2).wrapping_add((((0i32) as u32)).wrapping_mul(16)))); let a2_ = F32(c.m.r32(((p_p_2).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4))); let a3_ = F32(c.m.r32(((p_p_2).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8))); let a4_ = F32(c.m.r32(((p_p_2).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(12))); F::Vec4_Set(c, (fp.wrapping_add(64)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), a1_, a2_, a3_, a4_) };
        { let a1_ = F32(c.m.r32((p_p_2).wrapping_add((((1i32) as u32)).wrapping_mul(16)))); let a2_ = F32(c.m.r32(((p_p_2).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(4))); let a3_ = F32(c.m.r32(((p_p_2).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8))); let a4_ = F32(c.m.r32(((p_p_2).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(12))); F::Vec4_Set(c, (fp.wrapping_add(64)).wrapping_add((((2i32) as u32)).wrapping_mul(16)), a1_, a2_, a3_, a4_) };
        { let a1_ = F32(c.m.r32((p_p_2).wrapping_add((((2i32) as u32)).wrapping_mul(16)))); let a2_ = F32(c.m.r32(((p_p_2).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(4))); let a3_ = F32(c.m.r32(((p_p_2).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8))); let a4_ = F32(c.m.r32(((p_p_2).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(12))); F::Vec4_Set(c, (fp.wrapping_add(64)).wrapping_add((((3i32) as u32)).wrapping_mul(16)), a1_, a2_, a3_, a4_) };
        F::Vec4_Set(c, fp.wrapping_add(128), rt::fmul(rt::fmul(p_t_3, p_t_3), p_t_3), rt::fmul(p_t_3, p_t_3), p_t_3, F32(0x3f800000));
        F::Mtx_MulVec4(c, fp.wrapping_add(128), fp, fp.wrapping_add(128));
        F::Mtx_MulVec4(c, fp.wrapping_add(128), fp.wrapping_add(64), fp.wrapping_add(128));
        F::Vec4_Div(c, p_out_1, fp.wrapping_add(128), F32(0x40000000));
        { let v_ = F32(0x3f800000); let a_ = (p_out_1).wrapping_add(12); c.m.w32(a_, (v_).0); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftGlow_Spline3", &PROBE_N, &[], true); }
    r_
}

pub fn EftGlow_BuildTables(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGlow_BuildTables", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(368) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 368);
    let mut l_yMin_1: F32 = F32(0);
    let mut l_yRange_2: F32 = F32(0);
    let mut l_zMin_3: F32 = F32(0);
    let mut l_zRange_4: F32 = F32(0);
    let mut l_i_5: i32 = 0;
    let mut l_j_6: i32 = 0;
    let r_ = 'f: {
        l_yMin_1 = F32(0x41a00000);
        l_yRange_2 = F32(0x42be0000);
        l_zMin_3 = l_yMin_1;
        l_zRange_4 = l_yRange_2;
        {
            l_i_5 = 0i32;
            'l1: loop {
                if !(((((l_i_5) < (13i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let a1_ = ((c.m.r32(sym::gEftGlowCfg)).wrapping_add(448)).wrapping_add((((l_i_5) as u32)).wrapping_mul(16)); let a2_ = { let a_ = (c.m.r32(sym::gEftGlowCfg)).wrapping_add(420); F32(c.m.r32(a_)) }; F::Vec3_Scale(c, (fp).wrapping_add((((l_i_5) as u32)).wrapping_mul(16)), a1_, a2_) };
                }
                { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
            }
        }
        F::Vec4_Copy(c, (fp.wrapping_add(208)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (fp).wrapping_add((((0i32) as u32)).wrapping_mul(16)));
        F::Vec4_Copy(c, (fp.wrapping_add(208)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), (fp).wrapping_add((((1i32) as u32)).wrapping_mul(16)));
        F::Vec4_Copy(c, (fp.wrapping_add(208)).wrapping_add((((2i32) as u32)).wrapping_mul(16)), (fp).wrapping_add((((2i32) as u32)).wrapping_mul(16)));
        F::Vec4_Copy(c, (fp.wrapping_add(208)).wrapping_add((((3i32) as u32)).wrapping_mul(16)), (fp).wrapping_add((((3i32) as u32)).wrapping_mul(16)));
        {
            l_i_5 = 0i32;
            'l3: loop {
                if !(((((l_i_5) < (10i32)) as i32)) != 0) { break 'l3; }
                'c4: {
                    {
                        l_j_6 = 0i32;
                        'l5: loop {
                            if !(((((l_j_6) < (2i32)) as i32)) != 0) { break 'l5; }
                            'c6: {
                                F::EftGlow_Spline3(c, fp.wrapping_add(272), fp.wrapping_add(208), rt::fmul(rt::i2f(l_j_6), F32(0x3f000000)));
                                { let a0_ = ((c.m.r32(sym::gEftGlow)).wrapping_add(32)).wrapping_add((((((l_i_5).wrapping_mul(2i32)).wrapping_add(l_j_6)) as u32)).wrapping_mul(16)); F::Vec4_Copy(c, a0_, fp.wrapping_add(272)) };
                            }
                            { let o_ = l_j_6; let n_ = (o_).wrapping_add(1); l_j_6 = n_; };
                        }
                    }
                    F::Vec4_Copy(c, (fp.wrapping_add(208)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(208)).wrapping_add((((1i32) as u32)).wrapping_mul(16)));
                    F::Vec4_Copy(c, (fp.wrapping_add(208)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(208)).wrapping_add((((2i32) as u32)).wrapping_mul(16)));
                    F::Vec4_Copy(c, (fp.wrapping_add(208)).wrapping_add((((2i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(208)).wrapping_add((((3i32) as u32)).wrapping_mul(16)));
                    F::Vec4_Copy(c, (fp.wrapping_add(208)).wrapping_add((((3i32) as u32)).wrapping_mul(16)), (fp).wrapping_add(((((4i32).wrapping_add(l_i_5)) as u32)).wrapping_mul(16)));
                }
                { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
            }
        }
        {
            l_i_5 = 0i32;
            'l7: loop {
                if !(((((l_i_5) < (6i32)) as i32)) != 0) { break 'l7; }
                'c8: {
                    { let v_ = rt::fdiv(rt::fmul(rt::i2f(l_i_5), F32(0x3f000000)), F32(0x40c00000)); let a_ = (fp.wrapping_add(288)).wrapping_add((((l_i_5) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                }
                { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
            }
        }
        {
            l_i_5 = 6i32;
            'l9: loop {
                if !(((((l_i_5) < (20i32)) as i32)) != 0) { break 'l9; }
                'c10: {
                    { let v_ = rt::fadd(rt::fdiv(rt::fmul(rt::i2f((l_i_5).wrapping_sub(6i32)), F32(0x3f000000)), F32(0x41600000)), F32(0x3f000000)); let a_ = (fp.wrapping_add(288)).wrapping_add((((l_i_5) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                }
                { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
            }
        }
        {
            l_i_5 = 0i32;
            'l11: loop {
                if !(((((l_i_5) < (20i32)) as i32)) != 0) { break 'l11; }
                'c12: {
                    { let v_ = F32(0x00000000); let a_ = ((c.m.r32(sym::gEftGlow)).wrapping_add(352)).wrapping_add((((l_i_5) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fneg({ let a0_ = rt::fdiv(rt::fmul(rt::fadd(l_yMin_1, rt::fmul(F32(c.m.r32((fp.wrapping_add(288)).wrapping_add((((l_i_5) as u32)).wrapping_mul(4)))), l_yRange_2)), F32(0x40490fda)), F32(0x43340000)); F::Port_cosf(c, a0_) }); let a_ = (((c.m.r32(sym::gEftGlow)).wrapping_add(352)).wrapping_add((((l_i_5) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = { let a0_ = rt::fdiv(rt::fmul(rt::fadd(l_zMin_3, rt::fmul(F32(c.m.r32((fp.wrapping_add(288)).wrapping_add((((l_i_5) as u32)).wrapping_mul(4)))), l_zRange_4)), F32(0x40490fda)), F32(0x43340000)); F::Port_sinf(c, a0_) }; let a_ = (((c.m.r32(sym::gEftGlow)).wrapping_add(352)).wrapping_add((((l_i_5) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(0x3f800000); let a_ = (((c.m.r32(sym::gEftGlow)).wrapping_add(352)).wrapping_add((((l_i_5) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
                    { let a0_ = ((c.m.r32(sym::gEftGlow)).wrapping_add(352)).wrapping_add((((l_i_5) as u32)).wrapping_mul(16)); let a1_ = ((c.m.r32(sym::gEftGlow)).wrapping_add(352)).wrapping_add((((l_i_5) as u32)).wrapping_mul(16)); F::Vec3_Normalize(c, a0_, a1_) };
                }
                { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftGlow_BuildTables", &PROBE_N, &[], true); }
    r_
}

pub fn EftGlow_GetShapePoint(c: &mut Ctx, mut p_out_1: u32, mut p_objId_2: i32, mut p_idx_3: i32, mut p_scale_4: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGlow_GetShapePoint", &PROBE_N, &[(p_out_1) as u32, (p_objId_2) as u32, (p_idx_3) as u32, (p_scale_4).0], false); }
    let r_ = 'f: {
    { let a1_ = ((c.m.r32(sym::gEftGlow)).wrapping_add(32)).wrapping_add((((p_idx_3) as u32)).wrapping_mul(16)); F::Vec3_Scale(c, p_out_1, a1_, p_scale_4) };
    { let r_ = { let a_ = (c.m.r32(sym::gEftGlowCfg2)).wrapping_add(8); F32(c.m.r32(a_)) }; let a_ = (p_out_1).wrapping_add(4); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
    { let v_ = F32(0x3f800000); let a_ = (p_out_1).wrapping_add(12); c.m.w32(a_, (v_).0); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftGlow_GetShapePoint", &PROBE_N, &[], true); }
    r_
}

pub fn EftGlow_GetShapeDir(c: &mut Ctx, mut p_out_1: u32, mut p_objId_2: i32, mut p_idx_3: i32, mut p_y_4: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGlow_GetShapeDir", &PROBE_N, &[(p_out_1) as u32, (p_objId_2) as u32, (p_idx_3) as u32, (p_y_4).0], false); }
    let r_ = 'f: {
    { let a1_ = ((c.m.r32(sym::gEftGlow)).wrapping_add(352)).wrapping_add((((p_idx_3) as u32)).wrapping_mul(16)); F::Vec4_Copy(c, p_out_1, a1_) };
    { let r_ = p_y_4; let a_ = (p_out_1).wrapping_add(4); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftGlow_GetShapeDir", &PROBE_N, &[], true); }
    r_
}

pub fn EftGlow_CalcFrame(c: &mut Ctx, mut p_m_1: u32, mut p_objId_2: i32, mut p_kind_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGlow_CalcFrame", &PROBE_N, &[(p_m_1) as u32, (p_objId_2) as u32, (p_kind_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let r_ = 'f: {
        F::Mtx_StoreIdentity(c, p_m_1);
        if (((((p_kind_3) != (5i32)) as i32)) != 0) {
            F::BtlCharApi_GetDir(c, p_objId_2, fp);
            if (((((p_kind_3) == (3i32)) as i32)) != 0) {
                { let r_ = rt::fadd(F::BtlCharApi_GetFallSpeed(c, p_objId_2), F32(0xbf800000)); let a_ = (fp).wrapping_add(4); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            } else {
                if (((((p_kind_3) == (4i32)) as i32)) != 0) {
                    { let r_ = rt::fadd(F::BtlCharApi_GetFallSpeed(c, p_objId_2), F32(0x3f800000)); let a_ = (fp).wrapping_add(4); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                }
            }
            F::Vec3_Normalize(c, fp, fp);
            { let v_ = { let a0_ = F32(c.m.r32((fp).wrapping_add(4))); F::Port_asinf(c, a0_) }; let a_ = fp.wrapping_add(32); c.m.w32(a_, (v_).0); };
            { let a2_ = F32(c.m.r32(fp.wrapping_add(32))); F::Mtx_RotateX(c, p_m_1, p_m_1, a2_) };
            F::BtlCharApi_GetRot(c, p_objId_2, fp.wrapping_add(32));
            { let r_ = F32(0x40490fda); let a_ = (fp.wrapping_add(32)).wrapping_add(4); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            if (((rt::fgt(F32(c.m.r32((fp.wrapping_add(32)).wrapping_add(4))), F32(0x40490fda)) as i32)) != 0) {
                { let r_ = F32(0x40c90fda); let a_ = (fp.wrapping_add(32)).wrapping_add(4); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            }
            { let a2_ = F32(c.m.r32((fp.wrapping_add(32)).wrapping_add(4))); F::Mtx_RotateY(c, p_m_1, p_m_1, a2_) };
        } else {
            F::BtlCharApi_GetNodePos(c, p_objId_2, 46i32, fp.wrapping_add(16));
            F::BtlCharApi_GetNodePos(c, p_objId_2, 3i32, fp);
            F::Vec3_Sub(c, fp, fp, fp.wrapping_add(16));
            F::Vec3_Normalize(c, fp, fp);
            { let v_ = { let a0_ = rt::fneg(F32(c.m.r32((fp).wrapping_add(4)))); F::Port_asinf(c, a0_) }; let a_ = fp.wrapping_add(32); c.m.w32(a_, (v_).0); };
            { let v_ = { let a0_ = F32(c.m.r32(fp)); let a1_ = F32(c.m.r32((fp).wrapping_add(8))); F::Port_atan2f(c, a0_, a1_) }; let a_ = (fp.wrapping_add(32)).wrapping_add(4); c.m.w32(a_, (v_).0); };
            { let a2_ = F32(c.m.r32(fp.wrapping_add(32))); F::Mtx_RotateX(c, p_m_1, p_m_1, a2_) };
            { let a2_ = F32(c.m.r32((fp.wrapping_add(32)).wrapping_add(4))); F::Mtx_RotateY(c, p_m_1, p_m_1, a2_) };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftGlow_CalcFrame", &PROBE_N, &[], true); }
    r_
}

pub fn EftGlow_CalcDrift(c: &mut Ctx, mut p_out_1: u32, mut p_objId_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGlow_CalcDrift", &PROBE_N, &[(p_out_1) as u32, (p_objId_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let mut l_s_3: F32 = F32(0);
    let r_ = 'f: {
        l_s_3 = rt::fadd({ let a_ = (c.m.r32(sym::gEftGlowCfg2)).wrapping_add(12); F32(c.m.r32(a_)) }, rt::fmul({ let a_ = (c.m.r32(sym::gEftGlowCfg2)).wrapping_add(16); F32(c.m.r32(a_)) }, rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff))));
        F::BtlCharApi_GetNodePos(c, p_objId_2, 48i32, fp);
        { let v_ = F32(0x00000000); let a_ = (fp).wrapping_add(12); c.m.w32(a_, (v_).0); };
        F::BtlCharApi_GetNodePos(c, p_objId_2, 3i32, fp.wrapping_add(16));
        { let v_ = F32(0x00000000); let a_ = (fp.wrapping_add(16)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        F::Vec4_Sub(c, fp.wrapping_add(32), fp.wrapping_add(16), fp);
        F::Vec3_Normalize(c, fp.wrapping_add(32), fp.wrapping_add(32));
        F::Vec4_Scale(c, fp.wrapping_add(32), fp.wrapping_add(32), l_s_3);
        F::BtlCharApi_GetFrameMove(c, p_objId_2, p_out_1);
        { let a2_ = { let a_ = (c.m.r32(sym::gEftGlowCfg2)).wrapping_add(20); F32(c.m.r32(a_)) }; F::Vec3_Scale(c, p_out_1, p_out_1, a2_) };
        F::Vec4_Clamp(c, p_out_1, p_out_1, F32(0xbf000000), F32(0x3f000000));
        F::Vec4_Add(c, p_out_1, p_out_1, fp.wrapping_add(32));
        { let v_ = F32(0x00000000); let a_ = (p_out_1).wrapping_add(12); c.m.w32(a_, (v_).0); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftGlow_CalcDrift", &PROBE_N, &[], true); }
    r_
}

pub fn EftGlow_CalcPartFlags(c: &mut Ctx, mut p_e_1: u32, mut p_pair_2: i32, mut p_odd_3: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGlow_CalcPartFlags", &PROBE_N, &[(p_e_1) as u32, (p_pair_2) as u32, (p_odd_3) as u32], false); }
    let mut l_f_4: i32 = 0;
    let mut l_tex_5: i32 = 0;
    let r_ = 'f: {
    l_f_4 = (((c.m.r32(p_e_1) as i32)) & (2i32));
    l_tex_5 = (((c.m.r32(p_e_1) as i32)) & (32i32));
    if ((p_pair_2) != 0) {
        { let r_ = 32i32; let n_ = ((l_f_4) | (r_)); l_f_4 = n_; };
    } else {
        if ((p_odd_3) != 0) {
            { let r_ = 64i32; let n_ = ((l_f_4) | (r_)); l_f_4 = n_; };
        }
    }
    if ((((((c.m.r32((p_e_1).wrapping_add(12)) as i32)) == (3i32)) as i32)) != 0) {
        { let r_ = 128i32; let n_ = ((l_f_4) | (r_)); l_f_4 = n_; };
    } else {
        if ((((((c.m.r32((p_e_1).wrapping_add(12)) as i32)) == (4i32)) as i32)) != 0) {
            { let r_ = 256i32; let n_ = ((l_f_4) | (r_)); l_f_4 = n_; };
        }
    }
    if ((l_tex_5) != 0) {
        { let r_ = 16i32; let n_ = ((l_f_4) | (r_)); l_f_4 = n_; };
    }
    break 'f l_f_4;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftGlow_CalcPartFlags", &PROBE_N, &[], true); }
    r_
}

pub fn EftGlow_GetAlpha(c: &mut Ctx, mut p_e_1: u32, mut p_flags_2: i32, mut p_idx_3: i32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGlow_GetAlpha", &PROBE_N, &[(p_e_1) as u32, (p_flags_2) as u32, (p_idx_3) as u32], false); }
    let mut l_a_4: F32 = F32(0);
    let r_ = 'f: {
    if ((((p_flags_2) & (2i32))) != 0) {
        if ((((p_flags_2) & (32i32))) != 0) {
            l_a_4 = F32(c.m.r32((p_e_1).wrapping_add(68)));
        } else {
            l_a_4 = F32(c.m.r32((p_e_1).wrapping_add(64)));
        }
    } else {
        if ((((p_flags_2) & (32i32))) != 0) {
            l_a_4 = F32(c.m.r32((((p_e_1).wrapping_add(608)).wrapping_add((((p_idx_3) as u32)).wrapping_mul(16))).wrapping_add(12)));
        } else {
            l_a_4 = F32(c.m.r32(((p_e_1).wrapping_add(400)).wrapping_add(12)));
        }
    }
    if ((((p_flags_2) & (8i32))) != 0) {
        { let r_ = F32(0x3f8ccccc); let n_ = rt::fmul(l_a_4, r_); l_a_4 = n_; };
    }
    break 'f l_a_4;
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "EftGlow_GetAlpha", &PROBE_N, &[], true); }
    r_
}

pub fn EftGlow_CalcLength(c: &mut Ctx, mut p_p_1: u32, mut p_stage_2: i32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGlow_CalcLength", &PROBE_N, &[(p_p_1) as u32, (p_stage_2) as u32], false); }
    let mut l_cfg_3: u32 = 0;
    let mut l_r_4: F32 = F32(0);
    let mut l_mul_5: u32 = 0;
    let mut l_side_6: u32 = 0;
    let r_ = 'f: {
    l_cfg_3 = c.m.r32(sym::gEftGlowCfg2);
    l_r_4 = F32(c.m.r32(((l_cfg_3).wrapping_add(60)).wrapping_add((((p_stage_2) as u32)).wrapping_mul(4))));
    l_mul_5 = (l_cfg_3).wrapping_add(84);
    l_side_6 = (l_cfg_3).wrapping_add(104);
    if (((((c.m.r32(p_p_1) as i32)) & (2i32))) != 0) {
        l_r_4 = F32(c.m.r32((l_cfg_3).wrapping_add(72)));
        { let r_ = { let a_ = (((l_cfg_3).wrapping_add(148)).wrapping_add((((c.m.r8((p_p_1).wrapping_add(4))) as u32)).wrapping_mul(16))).wrapping_add((((p_stage_2) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }; let n_ = rt::fmul(l_r_4, r_); l_r_4 = n_; };
    } else {
        if (((((c.m.r32(p_p_1) as i32)) & (32i32))) != 0) {
            { let r_ = F32(c.m.r32((l_side_6).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); let n_ = rt::fmul(l_r_4, r_); l_r_4 = n_; };
            if (((((((((c.m.r32(p_p_1) as i32)) & (128i32))) != 0) || (((((c.m.r32(p_p_1) as i32)) & (256i32))) != 0)) as i32)) != 0) {
                { let r_ = { let a_ = (((c.m.r32(sym::gEftGlowCfg)).wrapping_add(1408)).wrapping_add((((c.m.r8((p_p_1).wrapping_add(13))) as u32)).wrapping_mul(16))).wrapping_add(8); F32(c.m.r32(a_)) }; let n_ = rt::fmul(l_r_4, r_); l_r_4 = n_; };
            } else {
                { let r_ = { let a_ = (((c.m.r32(sym::gEftGlowCfg)).wrapping_add(1168)).wrapping_add((((c.m.r8((p_p_1).wrapping_add(13))) as u32)).wrapping_mul(24))).wrapping_add(16); F32(c.m.r32(a_)) }; let n_ = rt::fmul(l_r_4, r_); l_r_4 = n_; };
            }
        } else {
            if ((((!(((((c.m.r32(p_p_1) as i32)) & (64i32))) != 0)) as i32)) != 0) {
                { let r_ = F32(c.m.r32((l_side_6).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let n_ = rt::fmul(l_r_4, r_); l_r_4 = n_; };
            }
            if (((((((((c.m.r32(p_p_1) as i32)) & (128i32))) != 0) || (((((c.m.r32(p_p_1) as i32)) & (256i32))) != 0)) as i32)) != 0) {
                { let r_ = { let a_ = ((c.m.r32(sym::gEftGlowCfg)).wrapping_add(1408)).wrapping_add((((c.m.r8((p_p_1).wrapping_add(13))) as u32)).wrapping_mul(16)); F32(c.m.r32(a_)) }; let n_ = rt::fmul(l_r_4, r_); l_r_4 = n_; };
            } else {
                { let r_ = { let a_ = (((c.m.r32(sym::gEftGlowCfg)).wrapping_add(1168)).wrapping_add((((c.m.r8((p_p_1).wrapping_add(13))) as u32)).wrapping_mul(24))).wrapping_add(8); F32(c.m.r32(a_)) }; let n_ = rt::fmul(l_r_4, r_); l_r_4 = n_; };
            }
        }
    }
    if (((((c.m.r32(p_p_1) as i32)) & (16i32))) != 0) {
        { let r_ = F32(c.m.r32((l_mul_5).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let n_ = rt::fmul(l_r_4, r_); l_r_4 = n_; };
    }
    if (((((c.m.r32(p_p_1) as i32)) & (8i32))) != 0) {
        { let r_ = F32(c.m.r32((l_mul_5).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let n_ = rt::fmul(l_r_4, r_); l_r_4 = n_; };
    }
    break 'f l_r_4;
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "EftGlow_CalcLength", &PROBE_N, &[], true); }
    r_
}

pub fn EftGlow_CalcWidth(c: &mut Ctx, mut p_p_1: u32, mut p_stage_2: i32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGlow_CalcWidth", &PROBE_N, &[(p_p_1) as u32, (p_stage_2) as u32], false); }
    let mut l_cfg_3: u32 = 0;
    let mut l_r_4: F32 = F32(0);
    let mut l_mul_5: u32 = 0;
    let mut l_side_6: u32 = 0;
    let r_ = 'f: {
    l_cfg_3 = c.m.r32(sym::gEftGlowCfg2);
    l_r_4 = F32(c.m.r32(((l_cfg_3).wrapping_add(44)).wrapping_add((((p_stage_2) as u32)).wrapping_mul(4))));
    l_mul_5 = (l_cfg_3).wrapping_add(76);
    l_side_6 = (l_cfg_3).wrapping_add(92);
    if (((((c.m.r32(p_p_1) as i32)) & (2i32))) != 0) {
        l_r_4 = F32(c.m.r32((l_cfg_3).wrapping_add(56)));
        { let r_ = { let a_ = (((l_cfg_3).wrapping_add(116)).wrapping_add((((c.m.r8((p_p_1).wrapping_add(4))) as u32)).wrapping_mul(16))).wrapping_add((((p_stage_2) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }; let n_ = rt::fmul(l_r_4, r_); l_r_4 = n_; };
    } else {
        if (((((c.m.r32(p_p_1) as i32)) & (32i32))) != 0) {
            { let r_ = F32(c.m.r32((l_side_6).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); let n_ = rt::fmul(l_r_4, r_); l_r_4 = n_; };
            if (((((((((c.m.r32(p_p_1) as i32)) & (128i32))) != 0) || (((((c.m.r32(p_p_1) as i32)) & (256i32))) != 0)) as i32)) != 0) {
                { let r_ = { let a_ = (((c.m.r32(sym::gEftGlowCfg)).wrapping_add(1408)).wrapping_add((((c.m.r8((p_p_1).wrapping_add(13))) as u32)).wrapping_mul(16))).wrapping_add(12); F32(c.m.r32(a_)) }; let n_ = rt::fmul(l_r_4, r_); l_r_4 = n_; };
            } else {
                { let r_ = { let a_ = (((c.m.r32(sym::gEftGlowCfg)).wrapping_add(1168)).wrapping_add((((c.m.r8((p_p_1).wrapping_add(13))) as u32)).wrapping_mul(24))).wrapping_add(20); F32(c.m.r32(a_)) }; let n_ = rt::fmul(l_r_4, r_); l_r_4 = n_; };
            }
        } else {
            if (((((c.m.r32(p_p_1) as i32)) & (64i32))) != 0) {
                { let r_ = F32(c.m.r32((l_side_6).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let n_ = rt::fmul(l_r_4, r_); l_r_4 = n_; };
            } else {
                { let r_ = F32(c.m.r32((l_side_6).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let n_ = rt::fmul(l_r_4, r_); l_r_4 = n_; };
            }
            if (((((((((c.m.r32(p_p_1) as i32)) & (128i32))) != 0) || (((((c.m.r32(p_p_1) as i32)) & (256i32))) != 0)) as i32)) != 0) {
                { let r_ = { let a_ = (((c.m.r32(sym::gEftGlowCfg)).wrapping_add(1408)).wrapping_add((((c.m.r8((p_p_1).wrapping_add(13))) as u32)).wrapping_mul(16))).wrapping_add(4); F32(c.m.r32(a_)) }; let n_ = rt::fmul(l_r_4, r_); l_r_4 = n_; };
            } else {
                { let r_ = { let a_ = (((c.m.r32(sym::gEftGlowCfg)).wrapping_add(1168)).wrapping_add((((c.m.r8((p_p_1).wrapping_add(13))) as u32)).wrapping_mul(24))).wrapping_add(12); F32(c.m.r32(a_)) }; let n_ = rt::fmul(l_r_4, r_); l_r_4 = n_; };
            }
        }
    }
    if (((((c.m.r32(p_p_1) as i32)) & (16i32))) != 0) {
        { let r_ = F32(c.m.r32((l_mul_5).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let n_ = rt::fmul(l_r_4, r_); l_r_4 = n_; };
    }
    if (((((c.m.r32(p_p_1) as i32)) & (8i32))) != 0) {
        { let r_ = F32(c.m.r32((l_mul_5).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let n_ = rt::fmul(l_r_4, r_); l_r_4 = n_; };
    }
    break 'f l_r_4;
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "EftGlow_CalcWidth", &PROBE_N, &[], true); }
    r_
}

pub fn EftGlow_AllocPart(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGlow_AllocPart", &PROBE_N, &[], false); }
    let mut l_p_1: u32 = 0;
    let r_ = 'f: {
    if ((((({ let a_ = (c.m.r32(sym::gEftGlow)).wrapping_add(12); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        l_p_1 = { let a_ = (c.m.r32(sym::gEftGlow)).wrapping_add(12); c.m.r32(a_) };
        { let v_ = c.m.r32((l_p_1).wrapping_add(176)); let a_ = (c.m.r32(sym::gEftGlow)).wrapping_add(12); c.m.w32(a_, v_); };
    } else {
        if ((((({ let a_ = (c.m.r32(sym::gEftGlow)).wrapping_add(20); (c.m.r32(a_) as i32) }) < (70i32)) as i32)) != 0) {
            l_p_1 = ({ let a_ = (c.m.r32(sym::gEftGlow)).wrapping_add(8); c.m.r32(a_) }).wrapping_add(((({ let a_ = (c.m.r32(sym::gEftGlow)).wrapping_add(20); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); o_ }) as u32)).wrapping_mul(192));
        } else {
            break 'f ((0i32) as u32);
        }
    }
    { let v_ = { let a_ = (c.m.r32(sym::gEftGlow)).wrapping_add(16); c.m.r32(a_) }; let a_ = (l_p_1).wrapping_add(176); c.m.w32(a_, v_); };
    { let v_ = l_p_1; let a_ = (c.m.r32(sym::gEftGlow)).wrapping_add(16); c.m.w32(a_, v_); };
    { let v_ = 0i32; let a_ = l_p_1; c.m.w32(a_, (v_) as u32); };
    break 'f l_p_1;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftGlow_AllocPart", &PROBE_N, &[], true); }
    r_
}

pub fn EftGlow_SpawnPart(c: &mut Ctx, mut p_e_1: u32, mut p_objId_2: i32, mut p_angle_3: F32, mut p_scale_4: u32, mut p_odd_5: i32, mut p_pair_6: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGlow_SpawnPart", &PROBE_N, &[(p_e_1) as u32, (p_objId_2) as u32, (p_angle_3).0, (p_scale_4) as u32, (p_odd_5) as u32, (p_pair_6) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_p_7: u32 = 0;
    let mut l_shape_8: i32 = 0;
    let mut l_idx_9: i32 = 0;
    let mut l_k_10: u32 = 0;
    let mut l_alpha_11: F32 = F32(0);
    let mut l_inv_12: F32 = F32(0);
    let mut l_z_13: F32 = F32(0);
    let r_ = 'f: {
        l_p_7 = F::EftGlow_AllocPart(c);
        l_shape_8 = { let a_ = (((c.m.r32(sym::gEftGlowCfg)).wrapping_add(1168)).wrapping_add(((((c.m.r32((p_e_1).wrapping_add(20)) as i32)) as u32)).wrapping_mul(24))).wrapping_add(4); (c.m.r32(a_) as i32) };
        l_idx_9 = rt::rem_i32(F::Port_Rand(c), (c.m.r32((p_e_1).wrapping_add(36)) as i32));
        if (((((l_p_7) == (((0i32) as u32))) as i32)) != 0) {
            break 'f ((0i32) as u32);
        }
        { let v_ = F::EftGlow_CalcPartFlags(c, p_e_1, p_pair_6, p_odd_5); let a_ = l_p_7; c.m.w32(a_, (v_) as u32); };
        if (((((p_pair_6) != (0i32)) as i32)) != 0) {
            { let v_ = ((1i32) as u8); let a_ = (l_p_7).wrapping_add(4); c.m.w8(a_, v_); };
        } else {
            { let v_ = ((0i32) as u8); let a_ = (l_p_7).wrapping_add(4); c.m.w8(a_, v_); };
        }
        if (((((c.m.r32(l_p_7) as i32)) & (2i32))) != 0) {
            { let v_ = ((3i32) as u8); let a_ = (l_p_7).wrapping_add(5); c.m.w8(a_, v_); };
        } else {
            if (((((c.m.r32(l_p_7) as i32)) & (32i32))) != 0) {
                { let v_ = ((2i32) as u8); let a_ = (l_p_7).wrapping_add(5); c.m.w8(a_, v_); };
            } else {
                if (((((c.m.r32(l_p_7) as i32)) & (64i32))) != 0) {
                    { let v_ = ((0i32) as u8); let a_ = (l_p_7).wrapping_add(5); c.m.w8(a_, v_); };
                } else {
                    { let v_ = ((1i32) as u8); let a_ = (l_p_7).wrapping_add(5); c.m.w8(a_, v_); };
                }
            }
        }
        { let v_ = rt::fadd({ let a_ = (c.m.r32(sym::gEftGlowCfg2)).wrapping_add(28); F32(c.m.r32(a_)) }, rt::fmul({ let a_ = (c.m.r32(sym::gEftGlowCfg2)).wrapping_add(32); F32(c.m.r32(a_)) }, rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)))); let a_ = (l_p_7).wrapping_add(16); c.m.w32(a_, (v_).0); };
        if (((((c.m.r32(l_p_7) as i32)) & (8i32))) != 0) {
            { let v_ = rt::fmul(F32(c.m.r32((l_p_7).wrapping_add(16))), F32(0x3f666666)); let a_ = (l_p_7).wrapping_add(16); c.m.w32(a_, (v_).0); };
        }
        { let v_ = F32(c.m.r32((l_p_7).wrapping_add(16))); let a_ = (l_p_7).wrapping_add(20); c.m.w32(a_, (v_).0); };
        { let v_ = ((p_objId_2) as u8); let a_ = (l_p_7).wrapping_add(7); c.m.w8(a_, v_); };
        { let v_ = (((c.m.r32((p_e_1).wrapping_add(20)) as i32)) as u8); let a_ = (l_p_7).wrapping_add(13); c.m.w8(a_, v_); };
        { let v_ = p_angle_3; let a_ = (l_p_7).wrapping_add(56); c.m.w32(a_, (v_).0); };
        l_inv_12 = F32(0x00000000);
        F::Vec4_Set(c, (l_p_7).wrapping_add(128), l_inv_12, l_inv_12, l_inv_12, l_inv_12);
        { let v_ = F::EftGlow_CalcWidth(c, l_p_7, 0i32); let a_ = (l_p_7).wrapping_add(28); c.m.w32(a_, (v_).0); };
        { let v_ = F::EftGlow_CalcLength(c, l_p_7, 0i32); let a_ = (l_p_7).wrapping_add(32); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F::EftGlow_CalcWidth(c, l_p_7, 1i32), F32(c.m.r32((l_p_7).wrapping_add(28)))); let a_ = (l_p_7).wrapping_add(44); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fdiv(F32(c.m.r32((l_p_7).wrapping_add(44))), rt::fmul(F32(c.m.r32((l_p_7).wrapping_add(16))), { let a_ = c.m.r32(sym::gEftGlowCfg2); F32(c.m.r32(a_)) })); let a_ = (l_p_7).wrapping_add(44); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F::EftGlow_CalcLength(c, l_p_7, 1i32), F32(c.m.r32((l_p_7).wrapping_add(32)))); let a_ = (l_p_7).wrapping_add(48); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fdiv(F32(c.m.r32((l_p_7).wrapping_add(48))), rt::fmul(F32(c.m.r32((l_p_7).wrapping_add(16))), { let a_ = c.m.r32(sym::gEftGlowCfg2); F32(c.m.r32(a_)) })); let a_ = (l_p_7).wrapping_add(48); c.m.w32(a_, (v_).0); };
        if (((((c.m.r32(l_p_7) as i32)) & (16i32))) != 0) {
            { let v_ = ((rt::rem_i32(F::Port_Rand(c), (c.m.r32((p_e_1).wrapping_add(44)) as i32))) as u8); let a_ = (l_p_7).wrapping_add(6); c.m.w8(a_, v_); };
        }
        l_alpha_11 = { let a1_ = (c.m.r32(l_p_7) as i32); F::EftGlow_GetAlpha(c, p_e_1, a1_, l_idx_9) };
        l_k_10 = ((c.m.r32(sym::gEftGlowCfg2)).wrapping_add(180)).wrapping_add((((c.m.r8((l_p_7).wrapping_add(5))) as u32)).wrapping_mul(60));
        { let v_ = rt::fadd(F32(c.m.r32((l_k_10).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((l_k_10).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)))); let a_ = (l_p_7).wrapping_add(60); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_k_10).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(((l_k_10).wrapping_add(8)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)))); let a_ = (l_p_7).wrapping_add(52); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(rt::fadd(F32(c.m.r32(((l_k_10).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(((l_k_10).wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)))), F32(c.m.r32((p_e_1).wrapping_add(72)))); let a_ = (l_p_7).wrapping_add(24); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_k_10).wrapping_add(24)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(((l_k_10).wrapping_add(24)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)))); let a_ = (l_p_7).wrapping_add(36); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_k_10).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(((l_k_10).wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)))); let a_ = (l_p_7).wrapping_add(40); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_k_10).wrapping_add(40)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(((l_k_10).wrapping_add(40)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)))); let a_ = ((l_p_7).wrapping_add(112)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(F32(c.m.r32(p_scale_4)), F32(c.m.r32(((l_k_10).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a_ = (l_p_7).wrapping_add(112); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(F32(c.m.r32((p_scale_4).wrapping_add(4))), F32(c.m.r32(((l_k_10).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); let a_ = ((l_p_7).wrapping_add(112)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(F32(c.m.r32((p_scale_4).wrapping_add(8))), F32(c.m.r32(((l_k_10).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(4))))); let a_ = ((l_p_7).wrapping_add(112)).wrapping_add(8); c.m.w32(a_, (v_).0); };
        if (((((c.m.r32(l_p_7) as i32)) & (2i32))) != 0) {
            if (((((c.m.r32(l_p_7) as i32)) & (32i32))) != 0) {
                p_angle_3 = F32(0x3f800000);
                { let v_ = ((1i32) as u8); let a_ = (l_p_7).wrapping_add(12); c.m.w8(a_, v_); };
                F::Vec3_Scale(c, (l_p_7).wrapping_add(80), ((p_e_1).wrapping_add(608)).wrapping_add((((l_idx_9) as u32)).wrapping_mul(16)), p_angle_3);
                { let v_ = l_inv_12; let a_ = ((l_p_7).wrapping_add(80)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                F::Vec4_Sub(c, fp, ((p_e_1).wrapping_add(608)).wrapping_add((((l_idx_9) as u32)).wrapping_mul(16)), (l_p_7).wrapping_add(80));
                { let a2_ = rt::fdiv(p_angle_3, F32(c.m.r32((l_p_7).wrapping_add(20)))); F::Vec3_Scale(c, (l_p_7).wrapping_add(96), fp, a2_) };
                l_inv_12 = rt::fdiv(p_angle_3, { let a_ = (c.m.r32(sym::gEftGlowCfg)).wrapping_add(424); F32(c.m.r32(a_)) });
                { let v_ = rt::fmul(l_alpha_11, l_inv_12); let a_ = ((l_p_7).wrapping_add(96)).wrapping_add(12); c.m.w32(a_, (v_).0); };
            } else {
                { let v_ = ((0i32) as u8); let a_ = (l_p_7).wrapping_add(12); c.m.w8(a_, v_); };
                F::Vec4_Copy(c, (l_p_7).wrapping_add(80), (p_e_1).wrapping_add(400));
                { let v_ = l_inv_12; let a_ = ((l_p_7).wrapping_add(80)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                p_angle_3 = F32(0x3f800000);
                F::Vec4_Sub(c, fp, (p_e_1).wrapping_add(416), (l_p_7).wrapping_add(80));
                { let a2_ = rt::fdiv(p_angle_3, F32(c.m.r32((l_p_7).wrapping_add(20)))); F::Vec3_Scale(c, (l_p_7).wrapping_add(96), fp, a2_) };
                l_inv_12 = rt::fdiv(p_angle_3, { let a_ = (c.m.r32(sym::gEftGlowCfg)).wrapping_add(424); F32(c.m.r32(a_)) });
                { let v_ = rt::fmul(l_alpha_11, l_inv_12); let a_ = ((l_p_7).wrapping_add(96)).wrapping_add(12); c.m.w32(a_, (v_).0); };
            }
            { let v_ = rt::fmul(F32(c.m.r32(((l_p_7).wrapping_add(112)).wrapping_add(8))), rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x3fc00000)), F32(0x3f800000))); let a_ = ((l_p_7).wrapping_add(112)).wrapping_add(8); c.m.w32(a_, (v_).0); };
            { let v_ = (((rt::div_i32(l_shape_8, 2i32)).wrapping_add(rt::rem_i32(F::Port_Rand(c), 2i32))) as u8); let a_ = (l_p_7).wrapping_add(14); c.m.w8(a_, v_); };
        } else {
            if (((((c.m.r32(l_p_7) as i32)) & (32i32))) != 0) {
                { let v_ = ((1i32) as u8); let a_ = (l_p_7).wrapping_add(12); c.m.w8(a_, v_); };
                F::Vec4_Copy(c, (l_p_7).wrapping_add(80), ((p_e_1).wrapping_add(608)).wrapping_add((((l_idx_9) as u32)).wrapping_mul(16)));
                { let v_ = l_inv_12; let a_ = ((l_p_7).wrapping_add(80)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                {
                    l_z_13 = l_inv_12;
                    l_inv_12 = rt::fdiv(F32(0x3f800000), { let a_ = (c.m.r32(sym::gEftGlowCfg)).wrapping_add(424); F32(c.m.r32(a_)) });
                    F::Vec4_Set(c, (l_p_7).wrapping_add(96), l_z_13, l_z_13, l_z_13, rt::fmul(l_alpha_11, l_inv_12));
                }
                if (((((((((c.m.r32((p_e_1).wrapping_add(12)) as i32)).wrapping_sub(3i32)) as u32)) < (((2i32) as u32))) as i32)) != 0) {
                    { let r_ = F32(0x3e999999); let a_ = (l_p_7).wrapping_add(52); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    if (((((((c.m.r8((l_p_7).wrapping_add(13))) as i32)) == (0i32)) as i32)) != 0) {
                        { let v_ = rt::fmul(F32(c.m.r32((l_p_7).wrapping_add(52))), F32(0x3f000000)); let a_ = (l_p_7).wrapping_add(52); c.m.w32(a_, (v_).0); };
                    } else {
                        if (((((((c.m.r8((l_p_7).wrapping_add(13))) as i32)) == (1i32)) as i32)) != 0) {
                            { let v_ = rt::fmul(F32(c.m.r32((l_p_7).wrapping_add(52))), F32(0x3f266666)); let a_ = (l_p_7).wrapping_add(52); c.m.w32(a_, (v_).0); };
                        } else {
                            if ((((((((((((c.m.r8((l_p_7).wrapping_add(13))) as i32)).wrapping_sub(3i32)) as u8)) as i32)) < (2i32)) as i32)) != 0) {
                                { let r_ = F32(0x3f266666); let a_ = ((l_p_7).wrapping_add(112)).wrapping_add(8); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                                { let r_ = F32(0x3f000000); let a_ = (l_p_7).wrapping_add(52); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                            }
                        }
                    }
                }
                { let r_ = F32(0x3dcccccc); let a_ = (l_p_7).wrapping_add(52); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                { let r_ = F32(0x3fa66666); let a_ = (l_p_7).wrapping_add(24); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                { let v_ = (((l_shape_8).wrapping_add(rt::rem_i32(F::Port_Rand(c), 3i32))) as u8); let a_ = (l_p_7).wrapping_add(14); c.m.w8(a_, v_); };
            } else {
                { let v_ = ((0i32) as u8); let a_ = (l_p_7).wrapping_add(12); c.m.w8(a_, v_); };
                p_angle_3 = F32(0x3f800000);
                { let a2_ = rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x3e4ccccc)), p_angle_3); F::Vec3_Scale(c, (l_p_7).wrapping_add(80), (p_e_1).wrapping_add(400), a2_) };
                { let v_ = l_inv_12; let a_ = ((l_p_7).wrapping_add(80)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                l_inv_12 = rt::fdiv(p_angle_3, F32(c.m.r32((l_p_7).wrapping_add(20))));
                F::Vec3_Sub(c, (l_p_7).wrapping_add(96), (p_e_1).wrapping_add(416), (l_p_7).wrapping_add(80));
                F::Vec3_Scale(c, (l_p_7).wrapping_add(96), (l_p_7).wrapping_add(96), l_inv_12);
                l_inv_12 = rt::fdiv(p_angle_3, { let a_ = (c.m.r32(sym::gEftGlowCfg)).wrapping_add(424); F32(c.m.r32(a_)) });
                { let v_ = rt::fmul(l_alpha_11, l_inv_12); let a_ = ((l_p_7).wrapping_add(96)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                if (((((((((c.m.r32((p_e_1).wrapping_add(12)) as i32)).wrapping_sub(3i32)) as u32)) < (((2i32) as u32))) as i32)) != 0) {
                    { let r_ = F32(0x3fd99999); let a_ = (l_p_7).wrapping_add(60); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    { let r_ = F32(0x3f666666); let a_ = (l_p_7).wrapping_add(52); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    if (((((((c.m.r8((l_p_7).wrapping_add(13))) as i32)) == (0i32)) as i32)) != 0) {
                        { let v_ = rt::fmul(F32(c.m.r32((l_p_7).wrapping_add(52))), F32(0x3f000000)); let a_ = (l_p_7).wrapping_add(52); c.m.w32(a_, (v_).0); };
                    } else {
                        if (((((((c.m.r8((l_p_7).wrapping_add(13))) as i32)) == (1i32)) as i32)) != 0) {
                            { let v_ = rt::fmul(F32(c.m.r32((l_p_7).wrapping_add(52))), F32(0x3f266666)); let a_ = (l_p_7).wrapping_add(52); c.m.w32(a_, (v_).0); };
                        } else {
                            if ((((((((((((c.m.r8((l_p_7).wrapping_add(13))) as i32)).wrapping_sub(3i32)) as u8)) as i32)) < (2i32)) as i32)) != 0) {
                                { let r_ = F32(0x3f266666); let a_ = ((l_p_7).wrapping_add(112)).wrapping_add(8); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                                { let r_ = F32(0x3f000000); let a_ = (l_p_7).wrapping_add(52); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                            }
                        }
                    }
                } else {
                    if ((((((((((((c.m.r8((l_p_7).wrapping_add(13))) as i32)).wrapping_sub(3i32)) as u8)) as i32)) < (2i32)) as i32)) != 0) {
                        { let r_ = F32(0x3f333333); let a_ = (l_p_7).wrapping_add(52); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    }
                }
                { let r_ = F32(0x3f599999); let a_ = (l_p_7).wrapping_add(52); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                { let r_ = F32(0x3fcccccc); let a_ = (l_p_7).wrapping_add(24); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                { let v_ = (((l_shape_8).wrapping_add(rt::rem_i32(F::Port_Rand(c), 4i32))) as u8); let a_ = (l_p_7).wrapping_add(14); c.m.w8(a_, v_); };
            }
        }
        if (((((((c.m.r8((l_p_7).wrapping_add(14))) as i32)) >= (20i32)) as i32)) != 0) {
            { let v_ = ((19i32) as u8); let a_ = (l_p_7).wrapping_add(14); c.m.w8(a_, v_); };
        }
        { let a2_ = ((c.m.r8((l_p_7).wrapping_add(14))) as i32); let a3_ = F32(c.m.r32((l_p_7).wrapping_add(60))); F::EftGlow_GetShapeDir(c, (l_p_7).wrapping_add(144), p_objId_2, a2_, a3_) };
        { let v_ = ((0i32) as u8); let a_ = (l_p_7).wrapping_add(8); c.m.w8(a_, v_); };
        { let v_ = ((rt::rem_i32(F::Port_Rand(c), 2i32)) as u8); let a_ = (l_p_7).wrapping_add(9); c.m.w8(a_, v_); };
        { let v_ = ((rt::rem_i32(F::Port_Rand(c), 2i32)) as u8); let a_ = (l_p_7).wrapping_add(10); c.m.w8(a_, v_); };
        { let v_ = ((rt::rem_i32(F::Port_Rand(c), rt::f2i({ let a_ = (c.m.r32(sym::gEftGlowCfg2)).wrapping_add(4); F32(c.m.r32(a_)) }))) as u8); let a_ = (l_p_7).wrapping_add(11); c.m.w8(a_, v_); };
        break 'f l_p_7;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftGlow_SpawnPart", &PROBE_N, &[], true); }
    r_
}

pub fn EftGlow_PairPart(c: &mut Ctx, mut p_e_1: u32, mut p_p_2: u32, mut p_src_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGlow_PairPart", &PROBE_N, &[(p_e_1) as u32, (p_p_2) as u32, (p_src_3) as u32], false); }
    let mut l_idx_4: i32 = 0;
    let mut l_s_5: F32 = F32(0);
    let r_ = 'f: {
    l_idx_4 = rt::rem_i32(F::Port_Rand(c), (c.m.r32((p_e_1).wrapping_add(36)) as i32));
    { let v_ = c.m.r8((p_src_3).wrapping_add(9)); let a_ = (p_p_2).wrapping_add(9); c.m.w8(a_, v_); };
    { let v_ = c.m.r8((p_src_3).wrapping_add(14)); let a_ = (p_p_2).wrapping_add(14); c.m.w8(a_, v_); };
    { let v_ = rt::fmul(F32(c.m.r32((p_src_3).wrapping_add(16))), F32(0x3fc00000)); let a_ = (p_p_2).wrapping_add(16); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fmul(F32(c.m.r32((p_src_3).wrapping_add(20))), F32(0x3fc00000)); let a_ = (p_p_2).wrapping_add(20); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fdiv(rt::fsub(F::EftGlow_CalcWidth(c, p_p_2, 1i32), F32(c.m.r32((p_p_2).wrapping_add(28)))), rt::fmul(F32(c.m.r32((p_p_2).wrapping_add(16))), { let a_ = c.m.r32(sym::gEftGlowCfg2); F32(c.m.r32(a_)) })); let a_ = (p_p_2).wrapping_add(44); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fdiv(rt::fsub(F::EftGlow_CalcLength(c, p_p_2, 1i32), F32(c.m.r32((p_p_2).wrapping_add(32)))), rt::fmul(F32(c.m.r32((p_p_2).wrapping_add(16))), { let a_ = c.m.r32(sym::gEftGlowCfg2); F32(c.m.r32(a_)) })); let a_ = (p_p_2).wrapping_add(48); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((p_src_3).wrapping_add(52))); let a_ = (p_p_2).wrapping_add(52); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((p_src_3).wrapping_add(60))); let a_ = (p_p_2).wrapping_add(60); c.m.w32(a_, (v_).0); };
    F::Vec4_Copy(c, (p_p_2).wrapping_add(144), (p_src_3).wrapping_add(144));
    F::Vec4_Copy(c, (p_p_2).wrapping_add(112), (p_src_3).wrapping_add(112));
    l_s_5 = rt::fmul(F32(c.m.r32((p_e_1).wrapping_add(72))), F32(0x40800000));
    { let r_ = rt::fmul(F32(c.m.r32((p_p_2).wrapping_add(144))), l_s_5); let a_ = (p_p_2).wrapping_add(112); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
    { let r_ = rt::fmul(F32(c.m.r32(((p_p_2).wrapping_add(144)).wrapping_add(4))), l_s_5); let a_ = ((p_p_2).wrapping_add(112)).wrapping_add(4); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
    { let r_ = rt::fmul(F32(c.m.r32(((p_p_2).wrapping_add(144)).wrapping_add(8))), l_s_5); let a_ = ((p_p_2).wrapping_add(112)).wrapping_add(8); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
    { let v_ = rt::fdiv(rt::fsub(F32(c.m.r32(((p_e_1).wrapping_add(608)).wrapping_add((((l_idx_4) as u32)).wrapping_mul(16)))), F32(c.m.r32((p_p_2).wrapping_add(80)))), F32(c.m.r32((p_p_2).wrapping_add(20)))); let a_ = (p_p_2).wrapping_add(96); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fdiv(rt::fsub(F32(c.m.r32((((p_e_1).wrapping_add(608)).wrapping_add((((l_idx_4) as u32)).wrapping_mul(16))).wrapping_add(4))), F32(c.m.r32(((p_p_2).wrapping_add(80)).wrapping_add(4)))), F32(c.m.r32((p_p_2).wrapping_add(20)))); let a_ = ((p_p_2).wrapping_add(96)).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fdiv(rt::fsub(F32(c.m.r32((((p_e_1).wrapping_add(608)).wrapping_add((((l_idx_4) as u32)).wrapping_mul(16))).wrapping_add(8))), F32(c.m.r32(((p_p_2).wrapping_add(80)).wrapping_add(8)))), F32(c.m.r32((p_p_2).wrapping_add(20)))); let a_ = ((p_p_2).wrapping_add(96)).wrapping_add(8); c.m.w32(a_, (v_).0); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftGlow_PairPart", &PROBE_N, &[], true); }
    r_
}

pub fn EftGlow_SpawnNext(c: &mut Ctx, mut p_e_1: u32, mut p_objId_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGlow_SpawnNext", &PROBE_N, &[(p_e_1) as u32, (p_objId_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_n_3: i32 = 0;
    let mut l_angle_4: F32 = F32(0);
    let mut l_step_5: F32 = F32(0);
    let mut l_p_6: u32 = 0;
    let mut l_q_7: u32 = 0;
    let mut l_odd_8: i32 = 0;
    let mut l_pair_9: i32 = 0;
    let mut l_bit_10: i32 = 0;
    let mut l_side_11: i32 = 0;
    let mut l_armL_12: i32 = 0;
    let mut l_armR_13: i32 = 0;
    let r_ = 'f: {
        l_n_3 = (c.m.r32((p_e_1).wrapping_add(24)) as i32);
        l_armL_12 = 3i32;
        l_armR_13 = 4i32;
        if ((((((c.m.r32((p_e_1).wrapping_add(20)) as i32)) == (l_armL_12)) as i32)) != 0) {
            l_step_5 = F32(0xc1c80000);
            l_angle_4 = rt::fadd(rt::fadd(rt::fmul(rt::i2f(l_n_3), l_step_5), F32(0xc2200000)), rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), l_step_5));
            if (((rt::fle(l_angle_4, F32(0xc3340000)) as i32)) != 0) {
                { let r_ = F32(0x43b40000); let n_ = rt::fadd(l_angle_4, r_); l_angle_4 = n_; };
            }
        } else {
            if ((((((c.m.r32((p_e_1).wrapping_add(20)) as i32)) == (l_armR_13)) as i32)) != 0) {
                l_step_5 = F32(0x41c80000);
                l_angle_4 = rt::fadd(rt::fadd(rt::fmul(rt::i2f(l_n_3), l_step_5), F32(0x42200000)), rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), l_step_5));
                if (((rt::fge(l_angle_4, F32(0x43340000)) as i32)) != 0) {
                    { let r_ = F32(0x43b40000); let n_ = rt::fsub(l_angle_4, r_); l_angle_4 = n_; };
                }
            } else {
                if ((((((((((((((c.m.r32((p_e_1).wrapping_add(20)) as i32)) == (3i32)) as i32)) != 0) || ((((((c.m.r32((p_e_1).wrapping_add(20)) as i32)) == (6i32)) as i32)) != 0)) as i32)) != 0) || ((((((c.m.r32((p_e_1).wrapping_add(20)) as i32)) == (8i32)) as i32)) != 0)) as i32)) != 0) {
                    l_step_5 = F32(0xc20c0000);
                    l_angle_4 = rt::fadd(rt::fadd(rt::fmul(rt::i2f(l_n_3), l_step_5), F32(0xc1a00000)), rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), l_step_5));
                    if (((rt::fle(l_angle_4, F32(0xc3340000)) as i32)) != 0) {
                        { let r_ = F32(0x43b40000); let n_ = rt::fadd(l_angle_4, r_); l_angle_4 = n_; };
                    }
                } else {
                    if ((((((((((((((c.m.r32((p_e_1).wrapping_add(20)) as i32)) == (4i32)) as i32)) != 0) || ((((((c.m.r32((p_e_1).wrapping_add(20)) as i32)) == (7i32)) as i32)) != 0)) as i32)) != 0) || ((((((c.m.r32((p_e_1).wrapping_add(20)) as i32)) == (9i32)) as i32)) != 0)) as i32)) != 0) {
                        l_step_5 = F32(0x420c0000);
                        l_angle_4 = rt::fadd(rt::fadd(rt::fmul(rt::i2f(l_n_3), l_step_5), F32(0x41a00000)), rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), l_step_5));
                        if (((rt::fge(l_angle_4, F32(0x43340000)) as i32)) != 0) {
                            { let r_ = F32(0x43b40000); let n_ = rt::fsub(l_angle_4, r_); l_angle_4 = n_; };
                        }
                    } else {
                        l_step_5 = F32(0x41c80000);
                        l_angle_4 = rt::fadd(rt::fadd(rt::fmul(rt::i2f(l_n_3), l_step_5), F32(0xc1c80000)), rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), l_step_5));
                        if (((((l_n_3) >= (2i32)) as i32)) != 0) {
                            { let r_ = F32(0x43020000); let n_ = rt::fadd(l_angle_4, r_); l_angle_4 = n_; };
                        }
                        if (((rt::fge(l_angle_4, F32(0x43340000)) as i32)) != 0) {
                            { let r_ = F32(0x43b40000); let n_ = rt::fsub(l_angle_4, r_); l_angle_4 = n_; };
                        }
                    }
                }
            }
        }
        F::Vec3_Set(c, fp.wrapping_add(16), F32(0x00000000), F32(0x3f800000), F32(0x3e999999));
        F::Vec3_Normalize(c, fp.wrapping_add(16), fp.wrapping_add(16));
        { let a2_ = F32(c.m.r32((p_e_1).wrapping_add(72))); F::Vec3_Scale(c, fp, (p_e_1).wrapping_add(592), a2_) };
        { let v_ = F32(0x3f800000); let a_ = (fp).wrapping_add(12); c.m.w32(a_, (v_).0); };
        l_bit_10 = (1i32).wrapping_shl(((c.m.r32((p_e_1).wrapping_add(20)) as i32)) as u32);
        if (((((c.m.r32((p_e_1).wrapping_add(4)) as i32)) & (l_bit_10))) != 0) {
            { let r_ = (!(l_bit_10)); let a_ = (p_e_1).wrapping_add(4); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
            l_odd_8 = 1i32;
        } else {
            { let r_ = l_bit_10; let a_ = (p_e_1).wrapping_add(4); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            l_odd_8 = 0i32;
        }
        l_side_11 = 0i32;
        if ((((!(((((c.m.r32(p_e_1) as i32)) & (2i32))) != 0)) as i32)) != 0) {
            l_side_11 = l_odd_8;
        }
        l_p_6 = F::EftGlow_SpawnPart(c, p_e_1, p_objId_2, l_angle_4, fp, l_side_11, 0i32);
        if (((((l_p_6) == (((0i32) as u32))) as i32)) != 0) {
            break 'f;
        }
        l_pair_9 = 0i32;
        if ((((((c.m.r32((p_e_1).wrapping_add(48)) as i32)) < (65i32)) as i32)) != 0) {
            if (((((c.m.r32(p_e_1) as i32)) & (2i32))) != 0) {
                l_pair_9 = 1i32;
            } else {
                if ((((!(((((c.m.r32(p_e_1) as i32)) & (8i32))) != 0)) as i32)) != 0) {
                    if ((((((c.m.r32((p_e_1).wrapping_add(24)) as i32)) < (3i32)) as i32)) != 0) {
                        if ((((!((((F::Port_Rand(c)) & (1i32))) != 0)) as i32)) != 0) {
                            l_pair_9 = 1i32;
                        }
                    } else {
                        l_pair_9 = 1i32;
                    }
                } else {
                    l_pair_9 = (((rt::rem_i32(F::Port_Rand(c), 5i32)) == (0i32)) as i32);
                }
            }
            if ((l_pair_9) != 0) {
                l_q_7 = F::EftGlow_SpawnPart(c, p_e_1, p_objId_2, l_angle_4, fp, 0i32, 1i32);
                if (((((l_q_7) != (((0i32) as u32))) as i32)) != 0) {
                    { let r_ = 8i32; let a_ = p_e_1; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    if (((((c.m.r32(p_e_1) as i32)) & (2i32))) != 0) {
                        F::EftGlow_PairPart(c, p_e_1, l_q_7, l_p_6);
                    }
                }
            }
        }
        { let a_ = (p_e_1).wrapping_add(24); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        if ((((((c.m.r32((p_e_1).wrapping_add(24)) as i32)) >= (4i32)) as i32)) != 0) {
            { let v_ = 0i32; let a_ = (p_e_1).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
            if (((((c.m.r32(p_e_1) as i32)) & (2i32))) != 0) {
                { let a_ = (p_e_1).wrapping_add(20); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                if ((((((c.m.r32((p_e_1).wrapping_add(20)) as i32)) < (0i32)) as i32)) != 0) {
                    { let v_ = 7i32; let a_ = (p_e_1).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
                }
            } else {
                { let r_ = (!(8i32)); let a_ = p_e_1; let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_e_1).wrapping_add(20); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                if ((((((c.m.r32((p_e_1).wrapping_add(20)) as i32)) >= (10i32)) as i32)) != 0) {
                    { let v_ = 0i32; let a_ = (p_e_1).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
                    { let v_ = rt::fmul({ let a_ = (c.m.r32(sym::gEftGlowCfg)).wrapping_add(432); F32(c.m.r32(a_)) }, rt::fsub(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)))); let a_ = (p_e_1).wrapping_add(592); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fmul({ let a_ = (c.m.r32(sym::gEftGlowCfg)).wrapping_add(436); F32(c.m.r32(a_)) }, rt::fsub(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)))); let a_ = ((p_e_1).wrapping_add(592)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                }
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftGlow_SpawnNext", &PROBE_N, &[], true); }
    r_
}

pub fn EftGlow_Begin(c: &mut Ctx, mut p_e_1: u32, mut p_arg_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGlow_Begin", &PROBE_N, &[(p_e_1) as u32, (p_arg_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(64) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 64);
    let mut l_i_3: i32 = 0;
    let mut l_j_4: i32 = 0;
    let r_ = 'f: {
        { let v_ = 1i32; let a_ = p_e_1; c.m.w32(a_, (v_) as u32); };
        { let v_ = { let a0_ = (c.m.r32((p_arg_2).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32); F::BtlCharApi_GetActionFxKind(c, a0_) }; let a_ = (p_e_1).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (p_e_1).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
        { let v_ = (c.m.r32((p_arg_2).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32); let a_ = (p_e_1).wrapping_add(76); c.m.w32(a_, (v_) as u32); };
        { let v_ = F32(0x3e40c0c0); let a_ = (p_e_1).wrapping_add(64); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3e008080); let a_ = (p_e_1).wrapping_add(68); c.m.w32(a_, (v_).0); };
        { let v_ = 0i32; let a_ = (p_e_1).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
        {
            l_j_4 = 0i32;
            'l1: loop {
                if !(((((l_j_4) < (10i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let a0_ = (c.m.r32((p_arg_2).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32); let a1_ = { let a_ = ((c.m.r32(sym::gEftGlowCfg)).wrapping_add(1168)).wrapping_add((((l_j_4) as u32)).wrapping_mul(24)); (c.m.r32(a_) as i32) }; F::BtlCharApi_GetNodePos(c, a0_, a1_, ((p_e_1).wrapping_add(432)).wrapping_add((((l_j_4) as u32)).wrapping_mul(16))) };
                }
                { let o_ = l_j_4; let n_ = (o_).wrapping_add(1); l_j_4 = n_; };
            }
        }
        { let v_ = rt::fmul({ let a_ = (c.m.r32(sym::gEftGlowCfg)).wrapping_add(432); F32(c.m.r32(a_)) }, rt::fsub(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)))); let a_ = (p_e_1).wrapping_add(592); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul({ let a_ = (c.m.r32(sym::gEftGlowCfg)).wrapping_add(436); F32(c.m.r32(a_)) }, rt::fsub(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)))); let a_ = ((p_e_1).wrapping_add(592)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = { let a_ = (c.m.r32(sym::gEftGlowCfg)).wrapping_add(440); F32(c.m.r32(a_)) }; let a_ = ((p_e_1).wrapping_add(592)).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let a1_ = (c.m.r32((p_arg_2).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32); let a2_ = (c.m.r32((p_e_1).wrapping_add(12)) as i32); F::EftGlow_CalcFrame(c, fp, a1_, a2_) };
        {
            l_i_3 = 4i32;
            'l3: loop {
                if !(((((l_i_3) >= (0i32)) as i32)) != 0) { break 'l3; }
                'c4: {
                    F::Mtx_Copy(c, ((p_e_1).wrapping_add(80)).wrapping_add((((l_i_3) as u32)).wrapping_mul(64)), fp);
                }
                { let o_ = l_i_3; let n_ = (o_).wrapping_sub(1); l_i_3 = n_; };
            }
        }
        { let v_ = 1i32; let a_ = (p_e_1).wrapping_add(52); c.m.w32(a_, (v_) as u32); };
        { let v_ = { let a_ = (c.m.r32(sym::gEftGlowCfg)).wrapping_add(424); F32(c.m.r32(a_)) }; let a_ = (p_e_1).wrapping_add(56); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (p_e_1).wrapping_add(60); c.m.w32(a_, (v_).0); };
        { let v_ = 0i32; let a_ = (p_e_1).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
        { let v_ = rt::fdiv({ let a0_ = (c.m.r32((p_arg_2).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32); F::BtlCharApi_GetHeight(c, a0_) }, F32(0x419acccc)); let a_ = (p_e_1).wrapping_add(72); c.m.w32(a_, (v_).0); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftGlow_Begin", &PROBE_N, &[], true); }
    r_
}

pub fn EftGlow_UpdateFrames(c: &mut Ctx, mut p_e_1: u32, mut p_objId_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGlow_UpdateFrames", &PROBE_N, &[(p_e_1) as u32, (p_objId_2) as u32], false); }
    let mut l_i_3: i32 = 0;
    let r_ = 'f: {
    if (((((c.m.r32(p_e_1) as i32)) & (2i32))) != 0) {
        {
            l_i_3 = 4i32;
            'l1: loop {
                if !(((((l_i_3) > (0i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    F::Mtx_Copy(c, ((p_e_1).wrapping_add(80)).wrapping_add((((l_i_3) as u32)).wrapping_mul(64)), ((p_e_1).wrapping_add(80)).wrapping_add(((((l_i_3).wrapping_sub(1i32)) as u32)).wrapping_mul(64)));
                }
                { let o_ = l_i_3; let n_ = (o_).wrapping_sub(1); l_i_3 = n_; };
            }
        }
    } else {
        { let a2_ = (c.m.r32((p_e_1).wrapping_add(12)) as i32); F::EftGlow_CalcFrame(c, ((p_e_1).wrapping_add(80)).wrapping_add((((0i32) as u32)).wrapping_mul(64)), p_objId_2, a2_) };
        {
            l_i_3 = 4i32;
            'l3: loop {
                if !(((((l_i_3) > (0i32)) as i32)) != 0) { break 'l3; }
                'c4: {
                    F::Mtx_Copy(c, ((p_e_1).wrapping_add(80)).wrapping_add((((l_i_3) as u32)).wrapping_mul(64)), ((p_e_1).wrapping_add(80)).wrapping_add(((((l_i_3).wrapping_sub(1i32)) as u32)).wrapping_mul(64)));
                }
                { let o_ = l_i_3; let n_ = (o_).wrapping_sub(1); l_i_3 = n_; };
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftGlow_UpdateFrames", &PROBE_N, &[], true); }
    r_
}

pub fn EftGlow_Emit(c: &mut Ctx, mut p_e_1: u32, mut p_objId_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGlow_Emit", &PROBE_N, &[(p_e_1) as u32, (p_objId_2) as u32], false); }
    let mut l_n_3: i32 = 0;
    let mut l_i_4: i32 = 0;
    let r_ = 'f: {
    if ((((((c.m.r32((p_e_1).wrapping_add(48)) as i32)) < (65i32)) as i32)) != 0) {
        l_n_3 = 14i32;
        if (((((((c.m.r32((p_e_1).wrapping_add(48)) as i32)).wrapping_add(l_n_3)) >= (66i32)) as i32)) != 0) {
            l_n_3 = (65i32).wrapping_sub((c.m.r32((p_e_1).wrapping_add(48)) as i32));
        }
        'l1: while (((((l_n_3) > (0i32)) as i32)) != 0) {
            F::EftGlow_SpawnNext(c, p_e_1, p_objId_2);
            { let o_ = l_n_3; let n_ = (o_).wrapping_sub(1); l_n_3 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftGlow_Emit", &PROBE_N, &[], true); }
    r_
}

pub fn EftGlow_Step(c: &mut Ctx, mut p_e_1: u32, mut p_objId_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGlow_Step", &PROBE_N, &[(p_e_1) as u32, (p_objId_2) as u32], false); }
    let mut l_i_3: i32 = 0;
    let mut l_step_4: F32 = F32(0);
    let r_ = 'f: {
    if ((((!(((((c.m.r32(p_e_1) as i32)) & (2i32))) != 0)) as i32)) != 0) {
        { let _ = F::EftAura_IsVisible(c, p_objId_2); };
    }
    {
        l_i_3 = 0i32;
        'l1: loop {
            if !(((((l_i_3) < (10i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let a1_ = { let a_ = ((c.m.r32(sym::gEftGlowCfg)).wrapping_add(1168)).wrapping_add((((l_i_3) as u32)).wrapping_mul(24)); (c.m.r32(a_) as i32) }; F::BtlCharApi_GetNodePos(c, p_objId_2, a1_, ((p_e_1).wrapping_add(432)).wrapping_add((((l_i_3) as u32)).wrapping_mul(16))) };
            }
            { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
        }
    }
    F::EftGlow_UpdateFrames(c, p_e_1, p_objId_2);
    F::EftGlow_Emit(c, p_e_1, p_objId_2);
    if ((((((c.m.r32((p_e_1).wrapping_add(52)) as i32)) != (0i32)) as i32)) != 0) {
        if ((((((c.m.r32((p_e_1).wrapping_add(52)) as i32)) == (1i32)) as i32)) != 0) {
            l_step_4 = rt::fdiv(F32(0x3f800000), { let a_ = (c.m.r32(sym::gEftGlowCfg)).wrapping_add(424); F32(c.m.r32(a_)) });
            { let r_ = l_step_4; let a_ = (p_e_1).wrapping_add(60); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            if (((rt::fge(F32(c.m.r32((p_e_1).wrapping_add(60))), F32(0x3f800000)) as i32)) != 0) {
                { let v_ = F32(0x3f800000); let a_ = (p_e_1).wrapping_add(60); c.m.w32(a_, (v_).0); };
                { let v_ = 0i32; let a_ = (p_e_1).wrapping_add(52); c.m.w32(a_, (v_) as u32); };
            }
        } else {
            l_step_4 = rt::fdiv(F32(0x3f800000), { let a_ = (c.m.r32(sym::gEftGlowCfg)).wrapping_add(428); F32(c.m.r32(a_)) });
            { let r_ = l_step_4; let a_ = (p_e_1).wrapping_add(60); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            if (((rt::fle(F32(c.m.r32((p_e_1).wrapping_add(60))), F32(0x00000000)) as i32)) != 0) {
                { let v_ = F32(0x00000000); let a_ = (p_e_1).wrapping_add(60); c.m.w32(a_, (v_).0); };
                { let r_ = 4i32; let a_ = p_e_1; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_e_1).wrapping_add(52); c.m.w32(a_, (v_) as u32); };
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftGlow_Step", &PROBE_N, &[], true); }
    r_
}

pub fn EftGlow_FreeParts(c: &mut Ctx, mut p_objId_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGlow_FreeParts", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_link_2: u32 = 0;
    let mut l_p_3: u32 = 0;
    let r_ = 'f: {
    l_link_2 = (c.m.r32(sym::gEftGlow)).wrapping_add(16);
    'l1: while (((((c.m.r32(l_link_2)) != (((0i32) as u32))) as i32)) != 0) {
        l_p_3 = c.m.r32(l_link_2);
        if (((((((c.m.r8((l_p_3).wrapping_add(7))) as i32)) != (p_objId_1)) as i32)) != 0) {
            l_link_2 = (l_p_3).wrapping_add(176);
        } else {
            { let v_ = c.m.r32((l_p_3).wrapping_add(176)); let a_ = l_link_2; c.m.w32(a_, v_); };
            { let v_ = { let a_ = (c.m.r32(sym::gEftGlow)).wrapping_add(12); c.m.r32(a_) }; let a_ = (l_p_3).wrapping_add(176); c.m.w32(a_, v_); };
            { let v_ = l_p_3; let a_ = (c.m.r32(sym::gEftGlow)).wrapping_add(12); c.m.w32(a_, v_); };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftGlow_FreeParts", &PROBE_N, &[], true); }
    r_
}

pub fn EftGlow_StepParts(c: &mut Ctx, mut p_e_1: u32, mut p_objId_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGlow_StepParts", &PROBE_N, &[(p_e_1) as u32, (p_objId_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(192) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 192);
    let mut l_link_3: u32 = 0;
    let mut l_p_4: u32 = 0;
    let mut l_inv_5: F32 = F32(0);
    let mut l_d_6: F32 = F32(0);
    let mut l_t_7: F32 = F32(0);
    let r_ = 'f: {
        { let v_ = 0i32; let a_ = (p_e_1).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
        F::EftGlow_CalcDrift(c, fp.wrapping_add(160), p_objId_2);
        l_link_3 = (c.m.r32(sym::gEftGlow)).wrapping_add(16);
        'l1: while (((((c.m.r32(l_link_3)) != (((0i32) as u32))) as i32)) != 0) {
            l_p_4 = c.m.r32(l_link_3);
            if (((((((c.m.r8((l_p_4).wrapping_add(7))) as i32)) == (p_objId_2)) as i32)) != 0) {
                if ((((!(((rt::fgt(F32(c.m.r32((l_p_4).wrapping_add(16))), F32(0x00000000)) as i32)) != 0)) as i32)) != 0) {
                    { let v_ = c.m.r32((l_p_4).wrapping_add(176)); let a_ = l_link_3; c.m.w32(a_, v_); };
                    { let v_ = { let a_ = (c.m.r32(sym::gEftGlow)).wrapping_add(12); c.m.r32(a_) }; let a_ = (l_p_4).wrapping_add(176); c.m.w32(a_, v_); };
                    { let v_ = l_p_4; let a_ = (c.m.r32(sym::gEftGlow)).wrapping_add(12); c.m.w32(a_, v_); };
                } else {
                    { let a1_ = ((p_e_1).wrapping_add(80)).wrapping_add((((c.m.r8((l_p_4).wrapping_add(8))) as u32)).wrapping_mul(64)); F::Mtx_Copy(c, fp.wrapping_add(64), a1_) };
                    { let a_ = (l_p_4).wrapping_add(8); let o_ = c.m.r8(a_); let n_ = (o_).wrapping_add(1); c.m.w8(a_, n_); };
                    if (((((((c.m.r8((l_p_4).wrapping_add(8))) as i32)) >= (5i32)) as i32)) != 0) {
                        { let v_ = ((4i32) as u8); let a_ = (l_p_4).wrapping_add(8); c.m.w8(a_, v_); };
                    }
                    F::Mtx_MulVec4(c, fp.wrapping_add(128), fp.wrapping_add(64), (l_p_4).wrapping_add(112));
                    { let a1_ = ((p_e_1).wrapping_add(432)).wrapping_add((((c.m.r8((l_p_4).wrapping_add(13))) as u32)).wrapping_mul(16)); F::Vec4_Add(c, (l_p_4).wrapping_add(64), a1_, fp.wrapping_add(128)) };
                    { let a2_ = ((c.m.r8((l_p_4).wrapping_add(14))) as i32); let a3_ = rt::fmul(F32(c.m.r32((p_e_1).wrapping_add(72))), F32(c.m.r32((l_p_4).wrapping_add(52)))); F::EftGlow_GetShapePoint(c, (l_p_4).wrapping_add(128), p_objId_2, a2_, a3_) };
                    { let a2_ = ((c.m.r8((l_p_4).wrapping_add(14))) as i32); let a3_ = F32(c.m.r32((l_p_4).wrapping_add(60))); F::EftGlow_GetShapeDir(c, (l_p_4).wrapping_add(144), p_objId_2, a2_, a3_) };
                    F::Mtx_StoreIdentity(c, fp);
                    { let a2_ = rt::fdiv(rt::fmul(F32(c.m.r32((l_p_4).wrapping_add(56))), F32(0x40490fda)), F32(0x43340000)); F::Mtx_RotateZ(c, fp, fp, a2_) };
                    F::Mtx_MulVec4(c, fp.wrapping_add(144), fp, (l_p_4).wrapping_add(128));
                    F::Mtx_MulVec4(c, (l_p_4).wrapping_add(144), fp, (l_p_4).wrapping_add(144));
                    F::Mtx_MulVec4(c, fp.wrapping_add(144), fp.wrapping_add(64), fp.wrapping_add(144));
                    F::Vec4_Add(c, (l_p_4).wrapping_add(64), (l_p_4).wrapping_add(64), fp.wrapping_add(144));
                    { let v_ = F32(0x3f800000); let a_ = ((l_p_4).wrapping_add(64)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                    F::Mtx_MulVec4(c, (l_p_4).wrapping_add(160), fp.wrapping_add(64), (l_p_4).wrapping_add(144));
                    if ((((!(((((c.m.r32(l_p_4) as i32)) & (2i32))) != 0)) as i32)) != 0) {
                        F::Vec4_Add(c, (l_p_4).wrapping_add(160), (l_p_4).wrapping_add(160), fp.wrapping_add(160));
                        F::Vec3_Normalize(c, (l_p_4).wrapping_add(160), (l_p_4).wrapping_add(160));
                    }
                    { let r_ = F32(c.m.r32((l_p_4).wrapping_add(44))); let a_ = (l_p_4).wrapping_add(28); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    { let r_ = F32(c.m.r32((l_p_4).wrapping_add(48))); let a_ = (l_p_4).wrapping_add(32); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    if (((rt::flt(F32(c.m.r32((l_p_4).wrapping_add(28))), F32(0x00000000)) as i32)) != 0) {
                        { let v_ = F32(0x00000000); let a_ = (l_p_4).wrapping_add(28); c.m.w32(a_, (v_).0); };
                    }
                    if (((rt::flt(F32(c.m.r32((l_p_4).wrapping_add(32))), F32(0x00000000)) as i32)) != 0) {
                        { let v_ = F32(0x00000000); let a_ = (l_p_4).wrapping_add(32); c.m.w32(a_, (v_).0); };
                    }
                    if ((((!(((((c.m.r32(l_p_4) as i32)) & (4i32))) != 0)) as i32)) != 0) {
                        if (((rt::flt({ let a_ = c.m.r32(sym::gEftGlowCfg2); F32(c.m.r32(a_)) }, rt::fsub(F32(0x3f800000), rt::fdiv(F32(c.m.r32((l_p_4).wrapping_add(16))), F32(c.m.r32((l_p_4).wrapping_add(20)))))) as i32)) != 0) {
                            { let v_ = rt::fdiv(rt::fsub(F::EftGlow_CalcWidth(c, l_p_4, 2i32), F32(c.m.r32((l_p_4).wrapping_add(28)))), F32(c.m.r32((l_p_4).wrapping_add(16)))); let a_ = (l_p_4).wrapping_add(44); c.m.w32(a_, (v_).0); };
                            { let v_ = rt::fdiv(rt::fsub(F::EftGlow_CalcLength(c, l_p_4, 2i32), F32(c.m.r32((l_p_4).wrapping_add(32)))), F32(c.m.r32((l_p_4).wrapping_add(16)))); let a_ = (l_p_4).wrapping_add(48); c.m.w32(a_, (v_).0); };
                            { let r_ = 4i32; let a_ = l_p_4; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                        }
                    }
                    if ((((!(((rt::flt(rt::fsub(F32(c.m.r32((l_p_4).wrapping_add(20))), F32(c.m.r32((l_p_4).wrapping_add(16)))), { let a_ = (c.m.r32(sym::gEftGlowCfg)).wrapping_add(424); F32(c.m.r32(a_)) }) as i32)) != 0)) as i32)) != 0) {
                        if (((rt::fle(F32(c.m.r32((l_p_4).wrapping_add(16))), { let a_ = (c.m.r32(sym::gEftGlowCfg)).wrapping_add(428); F32(c.m.r32(a_)) }) as i32)) != 0) {
                            if ((((!(((((c.m.r32(l_p_4) as i32)) & (1i32))) != 0)) as i32)) != 0) {
                                l_inv_5 = rt::fdiv(F32(0x3f800000), rt::fadd(F32(c.m.r32((l_p_4).wrapping_add(16))), F32(0x3f800000)));
                                l_d_6 = rt::fsub(F32(c.m.r32(((l_p_4).wrapping_add(80)).wrapping_add(12))), F32(c.m.r32(((p_e_1).wrapping_add(416)).wrapping_add(12))));
                                { let r_ = 1i32; let a_ = l_p_4; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                                { let v_ = rt::fmul(rt::fneg(l_d_6), l_inv_5); let a_ = ((l_p_4).wrapping_add(96)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                            }
                        } else {
                            { let v_ = F32(0x00000000); let a_ = ((l_p_4).wrapping_add(96)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                        }
                    }
                    F::Vec4_Add(c, (l_p_4).wrapping_add(80), (l_p_4).wrapping_add(80), (l_p_4).wrapping_add(96));
                    F::Vec4_Clamp(c, (l_p_4).wrapping_add(80), (l_p_4).wrapping_add(80), F32(0x00000000), F32(0x3f800000));
                    if (((((c.m.r32(l_p_4) as i32)) & (2i32))) != 0) {
                        l_t_7 = rt::fdiv(F32(c.m.r32((l_p_4).wrapping_add(16))), F32(c.m.r32((l_p_4).wrapping_add(20))));
                        { let a2_ = rt::fmul({ let a_ = (c.m.r32(sym::gEftGlowCfg2)).wrapping_add(24); F32(c.m.r32(a_)) }, rt::fmul(rt::fmul(l_t_7, rt::fmul(rt::fmul(l_t_7, l_t_7), l_t_7)), rt::fmul(F32(c.m.r32(((l_p_4).wrapping_add(112)).wrapping_add(12))), F32(c.m.r32((p_e_1).wrapping_add(72)))))); F::Vec3_Scale(c, fp.wrapping_add(176), (l_p_4).wrapping_add(144), a2_) };
                        F::Vec4_Add(c, (l_p_4).wrapping_add(112), (l_p_4).wrapping_add(112), fp.wrapping_add(176));
                    }
                    { let a_ = (l_p_4).wrapping_add(11); let o_ = c.m.r8(a_); let n_ = (o_).wrapping_add(1); c.m.w8(a_, n_); };
                    if (((rt::fge(rt::i2f(((c.m.r8((l_p_4).wrapping_add(11))) as i32)), { let a_ = (c.m.r32(sym::gEftGlowCfg2)).wrapping_add(4); F32(c.m.r32(a_)) }) as i32)) != 0) {
                        { let v_ = ((0i32) as u8); let a_ = (l_p_4).wrapping_add(11); c.m.w8(a_, v_); };
                        { let a_ = (l_p_4).wrapping_add(10); let o_ = c.m.r8(a_); let n_ = (o_).wrapping_add(1); c.m.w8(a_, n_); };
                        if (((((((c.m.r8((l_p_4).wrapping_add(10))) as i32)) >= (2i32)) as i32)) != 0) {
                            { let v_ = ((0i32) as u8); let a_ = (l_p_4).wrapping_add(10); c.m.w8(a_, v_); };
                        }
                    }
                    l_link_3 = (l_p_4).wrapping_add(176);
                    { let r_ = F32(0x3f800000); let a_ = (l_p_4).wrapping_add(16); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    { let a_ = (p_e_1).wrapping_add(48); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                }
            } else {
                l_link_3 = (l_p_4).wrapping_add(176);
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftGlow_StepParts", &PROBE_N, &[], true); }
    r_
}

pub fn EftGlow_MakeFacingMtx(c: &mut Ctx, mut p_m_1: u32, mut p_dir_2: u32, mut p_pos_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGlow_MakeFacingMtx", &PROBE_N, &[(p_m_1) as u32, (p_dir_2) as u32, (p_pos_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(80) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 80);
    let mut l_d_4: F32 = F32(0);
    let mut l_s_5: F32 = F32(0);
    let r_ = 'f: {
        l_s_5 = F32(0x00000000);
        { let v_ = { let a_ = (((c.m.r32(sym::gBtlCamView)).wrapping_add(64)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }; let a_ = fp.wrapping_add(48); c.m.w32(a_, (v_).0); };
        { let v_ = { let a_ = (((c.m.r32(sym::gBtlCamView)).wrapping_add(64)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }; let a_ = (fp.wrapping_add(48)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = { let a_ = (((c.m.r32(sym::gBtlCamView)).wrapping_add(64)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }; let a_ = (fp.wrapping_add(48)).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = (fp.wrapping_add(48)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        l_d_4 = F::Vec3_Dot(c, fp.wrapping_add(48), p_dir_2);
        if (((rt::flt(l_d_4, l_s_5) as i32)) != 0) {
            l_s_5 = rt::fmul(rt::fneg(l_d_4), F32(0x3ecccccc));
            { let a1_ = rt::fneg(F32(c.m.r32(fp.wrapping_add(48)))); let a2_ = rt::fneg(F32(c.m.r32((fp.wrapping_add(48)).wrapping_add(4)))); let a3_ = rt::fneg(F32(c.m.r32((fp.wrapping_add(48)).wrapping_add(8)))); F::Vec3_Set(c, fp.wrapping_add(64), a1_, a2_, a3_) };
        } else {
            l_s_5 = rt::fmul(l_d_4, F32(0x3ecccccc));
            { let a1_ = F32(c.m.r32(fp.wrapping_add(48))); let a2_ = F32(c.m.r32((fp.wrapping_add(48)).wrapping_add(4))); let a3_ = F32(c.m.r32((fp.wrapping_add(48)).wrapping_add(8))); F::Vec3_Set(c, fp.wrapping_add(64), a1_, a2_, a3_) };
        }
        F::Vec4_Sub(c, fp.wrapping_add(64), p_dir_2, fp.wrapping_add(64));
        F::Vec3_Normalize(c, fp.wrapping_add(64), fp.wrapping_add(64));
        F::Vec4_Scale(c, fp.wrapping_add(64), fp.wrapping_add(64), l_s_5);
        F::Vec4_Add(c, fp.wrapping_add(32), p_dir_2, fp.wrapping_add(64));
        F::Vec3_Normalize(c, fp.wrapping_add(32), fp.wrapping_add(32));
        F::Vec3_Cross(c, fp, fp.wrapping_add(48), fp.wrapping_add(32));
        F::Vec3_Normalize(c, fp, fp);
        F::Vec3_Cross(c, fp.wrapping_add(16), fp.wrapping_add(32), fp);
        F::Vec3_Normalize(c, fp.wrapping_add(16), fp.wrapping_add(16));
        F::Mtx_StoreIdentity(c, p_m_1);
        { let v_ = F32(c.m.r32(fp)); let a_ = ((p_m_1).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32(fp.wrapping_add(16))); let a_ = ((p_m_1).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32(fp.wrapping_add(32))); let a_ = ((p_m_1).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((fp).wrapping_add(4))); let a_ = ((p_m_1).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))); let a_ = ((p_m_1).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((fp.wrapping_add(32)).wrapping_add(4))); let a_ = ((p_m_1).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((fp).wrapping_add(8))); let a_ = ((p_m_1).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8))); let a_ = ((p_m_1).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((fp.wrapping_add(32)).wrapping_add(8))); let a_ = ((p_m_1).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32(p_pos_3)); let a_ = ((p_m_1).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((p_pos_3).wrapping_add(4))); let a_ = ((p_m_1).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((p_pos_3).wrapping_add(8))); let a_ = ((p_m_1).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftGlow_MakeFacingMtx", &PROBE_N, &[], true); }
    r_
}

pub fn EftGlow_DrawParts(c: &mut Ctx, mut p_e_1: u32, mut p_objId_2: i32, mut p_alpha_3: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGlow_DrawParts", &PROBE_N, &[(p_e_1) as u32, (p_objId_2) as u32, (p_alpha_3).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(272) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 272);
    let mut l_link_4: u32 = 0;
    let mut l_p_5: u32 = 0;
    let mut l_pkt_6: u32 = 0;
    let mut l_pairTex_7: u32 = 0;
    let mut l_clip_8: i32 = 0;
    let mut l_i_9: i32 = 0;
    let mut l_z_10: i32 = 0;
    let mut l_sx_11: F32 = F32(0);
    let mut l_sz_12: F32 = F32(0);
    let mut l_a_13: F32 = F32(0);
    let mut l_q_14: F32 = F32(0);
    let mut bb_: u32 = 0;
    let r_ = 'f: {
        'fl1: loop {
            match bb_ {
                0 => {
                    { let v_ = ((c.m.r32(sym::gEftGlowCfg)).wrapping_add(656)).wrapping_add((((0i32) as u32)).wrapping_mul(64)); let a_ = (fp.wrapping_add(64)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = ((c.m.r32(sym::gEftGlowCfg)).wrapping_add(656)).wrapping_add((((1i32) as u32)).wrapping_mul(64)); let a_ = (fp.wrapping_add(64)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = (c.m.r32(sym::gEftGlowCfg)).wrapping_add(784); let a_ = (fp.wrapping_add(72)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = (c.m.r32(sym::gEftGlowCfg)).wrapping_add(912); let a_ = (fp.wrapping_add(72)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = (c.m.r32(sym::gEftGlowCfg)).wrapping_add(848); let a_ = (fp.wrapping_add(72)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = (c.m.r32(sym::gEftGlowCfg)).wrapping_add(976); let a_ = (fp.wrapping_add(72)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = ((c.m.r32(sym::gEftGlowCfg)).wrapping_add(1040)).wrapping_add((((0i32) as u32)).wrapping_mul(64)); let a_ = (fp.wrapping_add(88)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = ((c.m.r32(sym::gEftGlowCfg)).wrapping_add(1040)).wrapping_add((((1i32) as u32)).wrapping_mul(64)); let a_ = (fp.wrapping_add(88)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    l_pairTex_7 = (c.m.r32(sym::gEftGlow)).wrapping_add(1256);
                    l_link_4 = (c.m.r32(sym::gEftGlow)).wrapping_add(16);
                    bb_ = 1; continue 'fl1;
                }
                1 => {
                    if (((((c.m.r32(l_link_4)) != (((0i32) as u32))) as i32)) != 0) { bb_ = 2; continue 'fl1; } else { bb_ = 3; continue 'fl1; }
                }
                2 => {
                    l_p_5 = c.m.r32(l_link_4);
                    if (((((((c.m.r8((l_p_5).wrapping_add(7))) as i32)) != (p_objId_2)) as i32)) != 0) { bb_ = 4; continue 'fl1; } else { bb_ = 5; continue 'fl1; }
                }
                3 => {
                    break 'fl1;
                }
                4 => {
                    bb_ = 6; continue 'fl1;
                }
                5 => {
                    l_clip_8 = 0i32;
                    if (((((c.m.r32(l_p_5) as i32)) & (2i32))) != 0) { bb_ = 8; continue 'fl1; } else { bb_ = 9; continue 'fl1; }
                }
                6 => {
                    l_link_4 = (l_p_5).wrapping_add(176);
                    bb_ = 1; continue 'fl1;
                }
                7 => {
                    bb_ = 5; continue 'fl1;
                }
                8 => {
                    l_clip_8 = 1i32;
                    bb_ = 9; continue 'fl1;
                }
                9 => {
                    if ((l_clip_8) != 0) { bb_ = 10; continue 'fl1; } else { bb_ = 11; continue 'fl1; }
                }
                10 => {
                    bb_ = 6; continue 'fl1;
                }
                11 => {
                    F::EftGlow_MakeFacingMtx(c, fp, (l_p_5).wrapping_add(160), (l_p_5).wrapping_add(64));
                    l_sx_11 = rt::fmul(rt::fmul(F32(c.m.r32((l_p_5).wrapping_add(24))), F32(c.m.r32((l_p_5).wrapping_add(28)))), F32(c.m.r32((l_p_5).wrapping_add(36))));
                    l_sz_12 = rt::fmul(F32(c.m.r32((l_p_5).wrapping_add(24))), F32(c.m.r32((l_p_5).wrapping_add(32))));
                    l_i_9 = 0i32;
                    bb_ = 13; continue 'fl1;
                }
                12 => {
                    bb_ = 11; continue 'fl1;
                }
                13 => {
                    if (((((l_i_9) < (4i32)) as i32)) != 0) { bb_ = 14; continue 'fl1; } else { bb_ = 16; continue 'fl1; }
                }
                14 => {
                    { let v_ = rt::fmul({ let a_ = ({ let a_ = (fp.wrapping_add(64)).wrapping_add((((c.m.r8((l_p_5).wrapping_add(4))) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add((((l_i_9) as u32)).wrapping_mul(16)); F32(c.m.r32(a_)) }, l_sx_11); let a_ = fp.wrapping_add(176); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fmul({ let a_ = (({ let a_ = (fp.wrapping_add(64)).wrapping_add((((c.m.r8((l_p_5).wrapping_add(4))) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add((((l_i_9) as u32)).wrapping_mul(16))).wrapping_add(4); F32(c.m.r32(a_)) }, F32(c.m.r32((l_p_5).wrapping_add(24)))); let a_ = (fp.wrapping_add(176)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fmul({ let a_ = (({ let a_ = (fp.wrapping_add(64)).wrapping_add((((c.m.r8((l_p_5).wrapping_add(4))) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add((((l_i_9) as u32)).wrapping_mul(16))).wrapping_add(8); F32(c.m.r32(a_)) }, l_sz_12); let a_ = (fp.wrapping_add(176)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(0x3f800000); let a_ = (fp.wrapping_add(176)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                    if (((((l_i_9) >= (2i32)) as i32)) != 0) { bb_ = 17; continue 'fl1; } else { bb_ = 18; continue 'fl1; }
                }
                15 => {
                    { let o_ = l_i_9; let n_ = (o_).wrapping_add(1); l_i_9 = n_; };
                    bb_ = 13; continue 'fl1;
                }
                16 => {
                    if ((l_clip_8) != 0) { bb_ = 31; continue 'fl1; } else { bb_ = 32; continue 'fl1; }
                }
                17 => {
                    { let r_ = F32(c.m.r32((l_p_5).wrapping_add(40))); let a_ = (fp.wrapping_add(176)).wrapping_add(8); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    bb_ = 18; continue 'fl1;
                }
                18 => {
                    F::Mtx_MulVec4(c, fp.wrapping_add(160), fp, fp.wrapping_add(176));
                    { let _ = F::Vu0Cur_ProjectPoint(c, (fp.wrapping_add(192)).wrapping_add((((l_i_9) as u32)).wrapping_mul(16)), fp.wrapping_add(160)); };
                    if ((((((((c.m.r32((fp.wrapping_add(192)).wrapping_add((((l_i_9) as u32)).wrapping_mul(16))) as i32)) as u32)) > (((65520i32) as u32))) as i32)) != 0) { bb_ = 19; continue 'fl1; } else { bb_ = 20; continue 'fl1; }
                }
                19 => {
                    l_clip_8 = 1i32;
                    bb_ = 16; continue 'fl1;
                }
                20 => {
                    if ((((((((c.m.r32(((fp.wrapping_add(192)).wrapping_add((((l_i_9) as u32)).wrapping_mul(16))).wrapping_add(4)) as i32)) as u32)) > (((65520i32) as u32))) as i32)) != 0) { bb_ = 22; continue 'fl1; } else { bb_ = 23; continue 'fl1; }
                }
                21 => {
                    bb_ = 20; continue 'fl1;
                }
                22 => {
                    l_clip_8 = 1i32;
                    bb_ = 16; continue 'fl1;
                }
                23 => {
                    if ((((((c.m.r32(((fp.wrapping_add(192)).wrapping_add((((l_i_9) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)) < (0i32)) as i32)) != 0) { bb_ = 25; continue 'fl1; } else { bb_ = 26; continue 'fl1; }
                }
                24 => {
                    bb_ = 23; continue 'fl1;
                }
                25 => {
                    l_clip_8 = 1i32;
                    bb_ = 16; continue 'fl1;
                }
                26 => {
                    l_q_14 = rt::fdiv(F32(0x3f800000), rt::i2f((c.m.r32(((fp.wrapping_add(192)).wrapping_add((((l_i_9) as u32)).wrapping_mul(16))).wrapping_add(12)) as i32)));
                    if (((((c.m.r32(l_p_5) as i32)) & (32i32))) != 0) { bb_ = 28; continue 'fl1; } else { bb_ = 30; continue 'fl1; }
                }
                27 => {
                    bb_ = 26; continue 'fl1;
                }
                28 => {
                    { let a1_ = ({ let a_ = (fp.wrapping_add(88)).wrapping_add((((c.m.r8((l_p_5).wrapping_add(9))) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add((((l_i_9) as u32)).wrapping_mul(16)); F::Vec4_Scale(c, (fp.wrapping_add(96)).wrapping_add((((l_i_9) as u32)).wrapping_mul(16)), a1_, l_q_14) };
                    bb_ = 29; continue 'fl1;
                }
                29 => {
                    bb_ = 15; continue 'fl1;
                }
                30 => {
                    { let a1_ = ({ let a_ = (fp.wrapping_add(72)).wrapping_add((((((((c.m.r8((l_p_5).wrapping_add(10))) as i32)).wrapping_mul(2i32)).wrapping_add(((c.m.r8((l_p_5).wrapping_add(9))) as i32))) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add((((l_i_9) as u32)).wrapping_mul(16)); F::Vec4_Scale(c, (fp.wrapping_add(96)).wrapping_add((((l_i_9) as u32)).wrapping_mul(16)), a1_, l_q_14) };
                    bb_ = 29; continue 'fl1;
                }
                31 => {
                    bb_ = 6; continue 'fl1;
                }
                32 => {
                    { let v_ = rt::f2i(rt::fmul(F32(c.m.r32((l_p_5).wrapping_add(80))), F32(0x437f0000))); let a_ = (fp.wrapping_add(256)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    { let v_ = rt::f2i(rt::fmul(F32(c.m.r32(((l_p_5).wrapping_add(80)).wrapping_add(4))), F32(0x437f0000))); let a_ = (fp.wrapping_add(256)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    { let v_ = rt::f2i(rt::fmul(F32(c.m.r32(((l_p_5).wrapping_add(80)).wrapping_add(8))), F32(0x437f0000))); let a_ = (fp.wrapping_add(256)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    l_a_13 = rt::fmul(rt::fmul(rt::fmul(F32(c.m.r32(((l_p_5).wrapping_add(80)).wrapping_add(12))), F32(0x437f0000)), F32(c.m.r32((p_e_1).wrapping_add(60)))), p_alpha_3);
                    l_pkt_6 = c.m.r32(sym::gOtCur);
                    { let v_ = (l_pkt_6).wrapping_add((((1i32) as u32)).wrapping_mul(144)); let a_ = sym::gOtCur; c.m.w32(a_, v_); };
                    if (((((l_pkt_6) == (((0i32) as u32))) as i32)) != 0) { bb_ = 34; continue 'fl1; } else { bb_ = 35; continue 'fl1; }
                }
                33 => {
                    bb_ = 32; continue 'fl1;
                }
                34 => {
                    break 'f;
                }
                35 => {
                    { let v_ = ((92i32) as u64); let a_ = (l_pkt_6).wrapping_add(32); c.m.w64(a_, v_); };
                    { let v_ = ((536870920i32) as u32); let a_ = l_pkt_6; c.m.w32(a_, v_); };
                    { let v_ = ((268435456i32) as u32); let a_ = (l_pkt_6).wrapping_add(8); c.m.w32(a_, v_); };
                    { let v_ = ((1342177288i32) as u32); let a_ = (l_pkt_6).wrapping_add(12); c.m.w32(a_, v_); };
                    { let v_ = 16429131440647602177u64; let a_ = (l_pkt_6).wrapping_add(16); c.m.w64(a_, v_); };
                    { let v_ = ((18599481537995104i64) as u64); let a_ = (l_pkt_6).wrapping_add(24); c.m.w64(a_, v_); };
                    { let v_ = ((0i32) as u32); let a_ = (l_pkt_6).wrapping_add(4); c.m.w32(a_, v_); };
                    { let v_ = (((c.m.r32((fp.wrapping_add(256)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) as u8); let a_ = (((l_pkt_6).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(((0i32) as u32)); c.m.w8(a_, v_); };
                    { let v_ = (((c.m.r32((fp.wrapping_add(256)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) as u8); let a_ = (((l_pkt_6).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(((1i32) as u32)); c.m.w8(a_, v_); };
                    { let v_ = (((c.m.r32((fp.wrapping_add(256)).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32)) as u8); let a_ = (((l_pkt_6).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(((2i32) as u32)); c.m.w8(a_, v_); };
                    { let v_ = ((rt::f2i(l_a_13)) as u8); let a_ = (((l_pkt_6).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(((3i32) as u32)); c.m.w8(a_, v_); };
                    { let v_ = F32(c.m.r32(((fp.wrapping_add(96)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8))); let a_ = (((l_pkt_6).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = (((c.m.r32((fp.wrapping_add(256)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) as u8); let a_ = (((l_pkt_6).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(((0i32) as u32)); c.m.w8(a_, v_); };
                    { let v_ = (((c.m.r32((fp.wrapping_add(256)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) as u8); let a_ = (((l_pkt_6).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(((1i32) as u32)); c.m.w8(a_, v_); };
                    { let v_ = (((c.m.r32((fp.wrapping_add(256)).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32)) as u8); let a_ = (((l_pkt_6).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(((2i32) as u32)); c.m.w8(a_, v_); };
                    { let v_ = ((rt::f2i(l_a_13)) as u8); let a_ = (((l_pkt_6).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(((3i32) as u32)); c.m.w8(a_, v_); };
                    { let v_ = F32(c.m.r32(((fp.wrapping_add(96)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8))); let a_ = (((l_pkt_6).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = (((c.m.r32((fp.wrapping_add(256)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) as u8); let a_ = (((l_pkt_6).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(((0i32) as u32)); c.m.w8(a_, v_); };
                    { let v_ = (((c.m.r32((fp.wrapping_add(256)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) as u8); let a_ = (((l_pkt_6).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(((1i32) as u32)); c.m.w8(a_, v_); };
                    { let v_ = (((c.m.r32((fp.wrapping_add(256)).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32)) as u8); let a_ = (((l_pkt_6).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(((2i32) as u32)); c.m.w8(a_, v_); };
                    { let v_ = ((rt::f2i(l_a_13)) as u8); let a_ = (((l_pkt_6).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(((3i32) as u32)); c.m.w8(a_, v_); };
                    { let v_ = F32(c.m.r32(((fp.wrapping_add(96)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8))); let a_ = (((l_pkt_6).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = (((c.m.r32((fp.wrapping_add(256)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) as u8); let a_ = (((l_pkt_6).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(((0i32) as u32)); c.m.w8(a_, v_); };
                    { let v_ = (((c.m.r32((fp.wrapping_add(256)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) as u8); let a_ = (((l_pkt_6).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(((1i32) as u32)); c.m.w8(a_, v_); };
                    { let v_ = (((c.m.r32((fp.wrapping_add(256)).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32)) as u8); let a_ = (((l_pkt_6).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(((2i32) as u32)); c.m.w8(a_, v_); };
                    { let v_ = ((rt::f2i(l_a_13)) as u8); let a_ = (((l_pkt_6).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(((3i32) as u32)); c.m.w8(a_, v_); };
                    { let v_ = F32(c.m.r32(((fp.wrapping_add(96)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(8))); let a_ = (((l_pkt_6).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(c.m.r32((fp.wrapping_add(96)).wrapping_add((((0i32) as u32)).wrapping_mul(16)))); let a_ = (((l_pkt_6).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(8); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(c.m.r32(((fp.wrapping_add(96)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4))); let a_ = (((l_pkt_6).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(12); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(c.m.r32((fp.wrapping_add(96)).wrapping_add((((1i32) as u32)).wrapping_mul(16)))); let a_ = (((l_pkt_6).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(8); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(c.m.r32(((fp.wrapping_add(96)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(4))); let a_ = (((l_pkt_6).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(12); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(c.m.r32((fp.wrapping_add(96)).wrapping_add((((2i32) as u32)).wrapping_mul(16)))); let a_ = (((l_pkt_6).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(8); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(c.m.r32(((fp.wrapping_add(96)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(4))); let a_ = (((l_pkt_6).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(12); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(c.m.r32((fp.wrapping_add(96)).wrapping_add((((3i32) as u32)).wrapping_mul(16)))); let a_ = (((l_pkt_6).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(8); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(c.m.r32(((fp.wrapping_add(96)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(4))); let a_ = (((l_pkt_6).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(12); c.m.w32(a_, (v_).0); };
                    { let v_ = (((c.m.r32((fp.wrapping_add(192)).wrapping_add((((0i32) as u32)).wrapping_mul(16))) as i32)) as u64); let a_ = (((l_pkt_6).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(16); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                    { let v_ = (((c.m.r32(((fp.wrapping_add(192)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4)) as i32)) as u64); let a_ = ((((l_pkt_6).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                    { let v_ = (((c.m.r32(((fp.wrapping_add(192)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)) as u64); let a_ = ((((l_pkt_6).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
                    { let v_ = ((255i32) as u64); let a_ = ((((l_pkt_6).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
                    { let v_ = (((c.m.r32((fp.wrapping_add(192)).wrapping_add((((1i32) as u32)).wrapping_mul(16))) as i32)) as u64); let a_ = (((l_pkt_6).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(16); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                    { let v_ = (((c.m.r32(((fp.wrapping_add(192)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(4)) as i32)) as u64); let a_ = ((((l_pkt_6).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                    { let v_ = (((c.m.r32(((fp.wrapping_add(192)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)) as u64); let a_ = ((((l_pkt_6).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
                    { let v_ = ((255i32) as u64); let a_ = ((((l_pkt_6).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
                    { let v_ = (((c.m.r32((fp.wrapping_add(192)).wrapping_add((((2i32) as u32)).wrapping_mul(16))) as i32)) as u64); let a_ = (((l_pkt_6).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(16); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                    { let v_ = (((c.m.r32(((fp.wrapping_add(192)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(4)) as i32)) as u64); let a_ = ((((l_pkt_6).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                    { let v_ = (((c.m.r32(((fp.wrapping_add(192)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)) as u64); let a_ = ((((l_pkt_6).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
                    { let v_ = ((255i32) as u64); let a_ = ((((l_pkt_6).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
                    { let v_ = (((c.m.r32((fp.wrapping_add(192)).wrapping_add((((3i32) as u32)).wrapping_mul(16))) as i32)) as u64); let a_ = (((l_pkt_6).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(16); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                    { let v_ = (((c.m.r32(((fp.wrapping_add(192)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(4)) as i32)) as u64); let a_ = ((((l_pkt_6).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                    { let v_ = (((c.m.r32(((fp.wrapping_add(192)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)) as u64); let a_ = ((((l_pkt_6).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
                    { let v_ = ((255i32) as u64); let a_ = ((((l_pkt_6).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
                    if (((((c.m.r32(l_p_5) as i32)) & (32i32))) != 0) { bb_ = 37; continue 'fl1; } else { bb_ = 39; continue 'fl1; }
                }
                36 => {
                    bb_ = 35; continue 'fl1;
                }
                37 => {
                    { let v_ = c.m.r64(l_pairTex_7); let a_ = (l_pkt_6).wrapping_add(40); c.m.w64(a_, v_); };
                    bb_ = 38; continue 'fl1;
                }
                38 => {
                    l_z_10 = (((((c.m.r32(((fp.wrapping_add(192)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)).wrapping_add((c.m.r32(((fp.wrapping_add(192)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32))).wrapping_add((c.m.r32(((fp.wrapping_add(192)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32))).wrapping_add((c.m.r32(((fp.wrapping_add(192)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32))).wrapping_shr((10i32) as u32);
                    if (({ let a0_ = ((c.m.r8((l_p_5).wrapping_add(7))) as i32); F::BtlCharApi_IsCircleDash(c, a0_) }) != 0) { bb_ = 43; continue 'fl1; } else { bb_ = 45; continue 'fl1; }
                }
                39 => {
                    if (((((c.m.r32(p_e_1) as i32)) & (32i32))) != 0) { bb_ = 40; continue 'fl1; } else { bb_ = 42; continue 'fl1; }
                }
                40 => {
                    { let v_ = { let a_ = ((p_e_1).wrapping_add(672)).wrapping_add(((((1i32).wrapping_add(((c.m.r8((l_p_5).wrapping_add(6))) as i32))) as u32)).wrapping_mul(8)); c.m.r64(a_) }; let a_ = (l_pkt_6).wrapping_add(40); c.m.w64(a_, v_); };
                    bb_ = 41; continue 'fl1;
                }
                41 => {
                    bb_ = 38; continue 'fl1;
                }
                42 => {
                    { let v_ = c.m.r64(((p_e_1).wrapping_add(672)).wrapping_add((((0i32) as u32)).wrapping_mul(8))); let a_ = (l_pkt_6).wrapping_add(40); c.m.w64(a_, v_); };
                    bb_ = 41; continue 'fl1;
                }
                43 => {
                    F::eft_p_b__EftPOt_Add(c, l_pkt_6, l_z_10, 1i32);
                    bb_ = 44; continue 'fl1;
                }
                44 => {
                    bb_ = 6; continue 'fl1;
                }
                45 => {
                    if ((F::BtlScene_GetStageFlag(c)) != 0) { bb_ = 46; continue 'fl1; } else { bb_ = 48; continue 'fl1; }
                }
                46 => {
                    F::eft_p_b__EftPOt_Add(c, l_pkt_6, l_z_10, 1i32);
                    bb_ = 47; continue 'fl1;
                }
                47 => {
                    bb_ = 44; continue 'fl1;
                }
                48 => {
                    { let a2_ = ((c.m.r8((l_p_5).wrapping_add(12))) as i32); F::eft_p_b__EftPOt_Add(c, l_pkt_6, l_z_10, a2_) };
                    bb_ = 47; continue 'fl1;
                }
                _ => unreachable!(),
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftGlow_DrawParts", &PROBE_N, &[], true); }
    r_
}
