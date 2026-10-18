#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn Vec3_Hermite(c: &mut Ctx, mut p_out_1: u32, mut p_p0_2: u32, mut p_p1_3: u32, mut p_t0_4: u32, mut p_t1_5: u32, mut p_t_6: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vec3_Hermite", &PROBE_N, &[(p_out_1) as u32, (p_p0_2) as u32, (p_p1_3) as u32, (p_t0_4) as u32, (p_t1_5) as u32, (p_t_6).0], false); }
    let mut l_h0_7: F32 = F32(0);
    let mut l_h1_8: F32 = F32(0);
    let mut l_h2_9: F32 = F32(0);
    let mut l_h3_10: F32 = F32(0);
    let r_ = 'f: {
    l_h0_7 = rt::fmul(rt::fmul(rt::fsub(p_t_6, F32(0x3f800000)), rt::fsub(p_t_6, F32(0x3f800000))), rt::fadd(rt::fmul(F32(0x40000000), p_t_6), F32(0x3f800000)));
    l_h1_8 = rt::fmul(rt::fmul(p_t_6, p_t_6), rt::fsub(F32(0x40400000), rt::fmul(F32(0x40000000), p_t_6)));
    l_h2_9 = rt::fmul(rt::fmul(rt::fsub(F32(0x3f800000), p_t_6), rt::fsub(F32(0x3f800000), p_t_6)), p_t_6);
    l_h3_10 = rt::fmul(rt::fmul(rt::fsub(p_t_6, F32(0x3f800000)), p_t_6), p_t_6);
    { let v_ = rt::fadd(rt::fadd(rt::fadd(rt::fmul(l_h0_7, F32(c.m.r32(p_p0_2))), rt::fmul(l_h1_8, F32(c.m.r32(p_p1_3)))), rt::fmul(l_h2_9, F32(c.m.r32(p_t0_4)))), rt::fmul(l_h3_10, F32(c.m.r32(p_t1_5)))); let a_ = p_out_1; c.m.w32(a_, (v_).0); };
    { let v_ = rt::fadd(rt::fadd(rt::fadd(rt::fmul(l_h0_7, F32(c.m.r32((p_p0_2).wrapping_add(4)))), rt::fmul(l_h1_8, F32(c.m.r32((p_p1_3).wrapping_add(4))))), rt::fmul(l_h2_9, F32(c.m.r32((p_t0_4).wrapping_add(4))))), rt::fmul(l_h3_10, F32(c.m.r32((p_t1_5).wrapping_add(4))))); let a_ = (p_out_1).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fadd(rt::fadd(rt::fadd(rt::fmul(l_h0_7, F32(c.m.r32((p_p0_2).wrapping_add(8)))), rt::fmul(l_h1_8, F32(c.m.r32((p_p1_3).wrapping_add(8))))), rt::fmul(l_h2_9, F32(c.m.r32((p_t0_4).wrapping_add(8))))), rt::fmul(l_h3_10, F32(c.m.r32((p_t1_5).wrapping_add(8))))); let a_ = (p_out_1).wrapping_add(8); c.m.w32(a_, (v_).0); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Vec3_Hermite", &PROBE_N, &[], true); }
    r_
}

pub fn Quat_SetIdentity(c: &mut Ctx, mut p_q_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Quat_SetIdentity", &PROBE_N, &[(p_q_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = F32(0x00000000); let a_ = p_q_1; c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x00000000); let a_ = (p_q_1).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x00000000); let a_ = (p_q_1).wrapping_add(8); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x3f800000); let a_ = (p_q_1).wrapping_add(12); c.m.w32(a_, (v_).0); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Quat_SetIdentity", &PROBE_N, &[], true); }
    r_
}

pub fn Quat_FromAxisAngle(c: &mut Ctx, mut p_out_1: u32, mut p_x_2: F32, mut p_y_3: F32, mut p_z_4: F32, mut p_angle_5: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Quat_FromAxisAngle", &PROBE_N, &[(p_out_1) as u32, (p_x_2).0, (p_y_3).0, (p_z_4).0, (p_angle_5).0], false); }
    let mut l_len_6: F32 = F32(0);
    let mut l_half_7: F32 = F32(0);
    let mut l_s_8: F32 = F32(0);
    let mut l_inv_9: F32 = F32(0);
    let r_ = 'f: {
    l_len_6 = F::Port_sqrtf(c, rt::fadd(rt::fadd(rt::fmul(p_x_2, p_x_2), rt::fmul(p_y_3, p_y_3)), rt::fmul(p_z_4, p_z_4)));
    if (((rt::fgt(l_len_6, F32(0x358637bd)) as i32)) != 0) {
        l_half_7 = rt::fmul(p_angle_5, F32(0x3f000000));
        l_s_8 = F::Port_sinf(c, l_half_7);
        l_inv_9 = rt::fdiv(F32(0x3f800000), l_len_6);
        { let v_ = rt::fmul(rt::fmul(p_x_2, l_inv_9), l_s_8); let a_ = p_out_1; c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(rt::fmul(p_y_3, l_inv_9), l_s_8); let a_ = (p_out_1).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(rt::fmul(p_z_4, l_inv_9), l_s_8); let a_ = (p_out_1).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F::Port_cosf(c, l_half_7); let a_ = (p_out_1).wrapping_add(12); c.m.w32(a_, (v_).0); };
    } else {
        { let v_ = F32(0x00000000); let a_ = p_out_1; c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (p_out_1).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (p_out_1).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = (p_out_1).wrapping_add(12); c.m.w32(a_, (v_).0); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Quat_FromAxisAngle", &PROBE_N, &[], true); }
    r_
}

pub fn Quat_FromVectors(c: &mut Ctx, mut p_out_1: u32, mut p_from_2: u32, mut p_to_3: u32, mut p_t_4: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Quat_FromVectors", &PROBE_N, &[(p_out_1) as u32, (p_from_2) as u32, (p_to_3) as u32, (p_t_4).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let mut l_s_5: F32 = F32(0);
    let mut l_inv_6: F32 = F32(0);
    let r_ = 'f: {
        F::Vec3_Normalize(c, fp, p_from_2);
        F::Vec3_Normalize(c, fp.wrapping_add(16), p_to_3);
        F::Vec3_Cross(c, fp.wrapping_add(32), fp, fp.wrapping_add(16));
        l_s_5 = { let a0_ = rt::fmul(rt::fadd(F::Vec3_Dot(c, fp, fp.wrapping_add(16)), F32(0x3f800000)), F32(0x40000000)); F::Port_sqrtf(c, a0_) };
        l_inv_6 = rt::fdiv(F32(0x3f800000), l_s_5);
        { let v_ = rt::fmul(F32(c.m.r32(fp.wrapping_add(32))), l_inv_6); let a_ = p_out_1; c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(F32(c.m.r32((fp.wrapping_add(32)).wrapping_add(4))), l_inv_6); let a_ = (p_out_1).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(F32(c.m.r32((fp.wrapping_add(32)).wrapping_add(8))), l_inv_6); let a_ = (p_out_1).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(l_s_5, F32(0x3f000000)); let a_ = (p_out_1).wrapping_add(12); c.m.w32(a_, (v_).0); };
        if (((rt::fne(p_t_4, F32(0x3f800000)) as i32)) != 0) {
            F::Quat_SlerpIdentity(c, p_out_1, p_out_1, p_t_4);
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Quat_FromVectors", &PROBE_N, &[], true); }
    r_
}

pub fn Quat_LengthSq(c: &mut Ctx, mut p_q_1: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Quat_LengthSq", &PROBE_N, &[(p_q_1) as u32], false); }
    let r_ = 'f: {
    break 'f rt::fadd(rt::fadd(rt::fadd(rt::fmul(F32(c.m.r32(p_q_1)), F32(c.m.r32(p_q_1))), rt::fmul(F32(c.m.r32((p_q_1).wrapping_add(4))), F32(c.m.r32((p_q_1).wrapping_add(4))))), rt::fmul(F32(c.m.r32((p_q_1).wrapping_add(8))), F32(c.m.r32((p_q_1).wrapping_add(8))))), rt::fmul(F32(c.m.r32((p_q_1).wrapping_add(12))), F32(c.m.r32((p_q_1).wrapping_add(12)))));
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "Quat_LengthSq", &PROBE_N, &[], true); }
    r_
}

pub fn Quat_Length(c: &mut Ctx, mut p_q_1: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Quat_Length", &PROBE_N, &[(p_q_1) as u32], false); }
    let r_ = 'f: {
    break 'f { let a0_ = F::Quat_LengthSq(c, p_q_1); F::Port_sqrtf(c, a0_) };
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "Quat_Length", &PROBE_N, &[], true); }
    r_
}

pub fn Quat_SlerpIdentity(c: &mut Ctx, mut p_out_1: u32, mut p_q_2: u32, mut p_t_3: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Quat_SlerpIdentity", &PROBE_N, &[(p_out_1) as u32, (p_q_2) as u32, (p_t_3).0], false); }
    let mut l_w_4: F32 = F32(0);
    let mut l_angle_5: F32 = F32(0);
    let mut l_s_6: F32 = F32(0);
    let mut l_inv_7: F32 = F32(0);
    let mut l_ka_8: F32 = F32(0);
    let mut l_kb_9: F32 = F32(0);
    let r_ = 'f: {
    l_w_4 = { let a0_ = F32(c.m.r32((p_q_2).wrapping_add(12))); F::Port_fabsf(c, a0_) };
    if (((rt::fgt(l_w_4, F32(0x3f800000)) as i32)) != 0) {
        l_w_4 = F32(0x3f800000);
    }
    l_angle_5 = F::Mathf_Acos(c, l_w_4);
    l_s_6 = F::Port_sinf(c, l_angle_5);
    if (((rt::flt(l_s_6, F32(0x358637bd)) as i32)) != 0) {
        { let v_ = F32(0x00000000); let a_ = p_out_1; c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (p_out_1).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (p_out_1).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = (p_out_1).wrapping_add(12); c.m.w32(a_, (v_).0); };
    } else {
        l_inv_7 = rt::fdiv(F32(0x3f800000), l_s_6);
        l_ka_8 = rt::fmul(F::Port_sinf(c, rt::fmul(l_angle_5, rt::fsub(F32(0x3f800000), p_t_3))), l_inv_7);
        l_kb_9 = rt::fmul(F::Port_sinf(c, rt::fmul(l_angle_5, p_t_3)), l_inv_7);
        { let v_ = rt::fmul(F32(c.m.r32(p_q_2)), l_kb_9); let a_ = p_out_1; c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(F32(c.m.r32((p_q_2).wrapping_add(4))), l_kb_9); let a_ = (p_out_1).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(F32(c.m.r32((p_q_2).wrapping_add(8))), l_kb_9); let a_ = (p_out_1).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(l_ka_8, rt::fmul(F32(c.m.r32((p_q_2).wrapping_add(12))), l_kb_9)); let a_ = (p_out_1).wrapping_add(12); c.m.w32(a_, (v_).0); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Quat_SlerpIdentity", &PROBE_N, &[], true); }
    r_
}

pub fn Quat_GetAngle(c: &mut Ctx, mut p_q_1: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Quat_GetAngle", &PROBE_N, &[(p_q_1) as u32], false); }
    let mut l_w_2: F32 = F32(0);
    let r_ = 'f: {
    l_w_2 = F32(c.m.r32((p_q_1).wrapping_add(12)));
    if (((rt::flt(l_w_2, F32(0xbf800000)) as i32)) != 0) {
        l_w_2 = F32(0xbf800000);
    }
    if (((rt::fgt(l_w_2, F32(0x3f800000)) as i32)) != 0) {
        l_w_2 = F32(0x3f800000);
    }
    break 'f rt::fmul(F::Mathf_Acos(c, l_w_2), F32(0x40000000));
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "Quat_GetAngle", &PROBE_N, &[], true); }
    r_
}

pub fn Quat_Conjugate(c: &mut Ctx, mut p_out_1: u32, mut p_q_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Quat_Conjugate", &PROBE_N, &[(p_out_1) as u32, (p_q_2) as u32], false); }
    let r_ = 'f: {
    { let v_ = rt::fneg(F32(c.m.r32(p_q_2))); let a_ = p_out_1; c.m.w32(a_, (v_).0); };
    { let v_ = rt::fneg(F32(c.m.r32((p_q_2).wrapping_add(4)))); let a_ = (p_out_1).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fneg(F32(c.m.r32((p_q_2).wrapping_add(8)))); let a_ = (p_out_1).wrapping_add(8); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((p_q_2).wrapping_add(12))); let a_ = (p_out_1).wrapping_add(12); c.m.w32(a_, (v_).0); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Quat_Conjugate", &PROBE_N, &[], true); }
    r_
}

pub fn Quat_Inverse(c: &mut Ctx, mut p_out_1: u32, mut p_q_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Quat_Inverse", &PROBE_N, &[(p_out_1) as u32, (p_q_2) as u32], false); }
    let mut l_inv_3: F32 = F32(0);
    let r_ = 'f: {
    l_inv_3 = rt::fdiv(F32(0x3f800000), F::Quat_LengthSq(c, p_q_2));
    { let v_ = rt::fmul(rt::fneg(F32(c.m.r32(p_q_2))), l_inv_3); let a_ = p_out_1; c.m.w32(a_, (v_).0); };
    { let v_ = rt::fmul(rt::fneg(F32(c.m.r32((p_q_2).wrapping_add(4)))), l_inv_3); let a_ = (p_out_1).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fmul(rt::fneg(F32(c.m.r32((p_q_2).wrapping_add(8)))), l_inv_3); let a_ = (p_out_1).wrapping_add(8); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fmul(F32(c.m.r32((p_q_2).wrapping_add(12))), l_inv_3); let a_ = (p_out_1).wrapping_add(12); c.m.w32(a_, (v_).0); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Quat_Inverse", &PROBE_N, &[], true); }
    r_
}

pub fn Quat_Mul(c: &mut Ctx, mut p_out_1: u32, mut p_a_2: u32, mut p_b_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Quat_Mul", &PROBE_N, &[(p_out_1) as u32, (p_a_2) as u32, (p_b_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_x_4: F32 = F32(0);
    let mut l_y_5: F32 = F32(0);
    let mut l_z_6: F32 = F32(0);
    let mut l_w_7: F32 = F32(0);
    let r_ = 'f: {
        l_w_7 = rt::fsub(rt::fsub(rt::fsub(rt::fmul(F32(c.m.r32((p_a_2).wrapping_add(12))), F32(c.m.r32((p_b_3).wrapping_add(12)))), rt::fmul(F32(c.m.r32(p_a_2)), F32(c.m.r32(p_b_3)))), rt::fmul(F32(c.m.r32((p_a_2).wrapping_add(4))), F32(c.m.r32((p_b_3).wrapping_add(4))))), rt::fmul(F32(c.m.r32((p_a_2).wrapping_add(8))), F32(c.m.r32((p_b_3).wrapping_add(8)))));
        l_x_4 = rt::fsub(rt::fadd(rt::fadd(rt::fmul(F32(c.m.r32((p_a_2).wrapping_add(12))), F32(c.m.r32(p_b_3))), rt::fmul(F32(c.m.r32(p_a_2)), F32(c.m.r32((p_b_3).wrapping_add(12))))), rt::fmul(F32(c.m.r32((p_a_2).wrapping_add(4))), F32(c.m.r32((p_b_3).wrapping_add(8))))), rt::fmul(F32(c.m.r32((p_a_2).wrapping_add(8))), F32(c.m.r32((p_b_3).wrapping_add(4)))));
        l_y_5 = rt::fadd(rt::fadd(rt::fsub(rt::fmul(F32(c.m.r32((p_a_2).wrapping_add(12))), F32(c.m.r32((p_b_3).wrapping_add(4)))), rt::fmul(F32(c.m.r32(p_a_2)), F32(c.m.r32((p_b_3).wrapping_add(8))))), rt::fmul(F32(c.m.r32((p_a_2).wrapping_add(4))), F32(c.m.r32((p_b_3).wrapping_add(12))))), rt::fmul(F32(c.m.r32((p_a_2).wrapping_add(8))), F32(c.m.r32(p_b_3))));
        l_z_6 = rt::fadd(rt::fsub(rt::fadd(rt::fmul(F32(c.m.r32((p_a_2).wrapping_add(12))), F32(c.m.r32((p_b_3).wrapping_add(8)))), rt::fmul(F32(c.m.r32(p_a_2)), F32(c.m.r32((p_b_3).wrapping_add(4))))), rt::fmul(F32(c.m.r32((p_a_2).wrapping_add(4))), F32(c.m.r32(p_b_3)))), rt::fmul(F32(c.m.r32((p_a_2).wrapping_add(8))), F32(c.m.r32((p_b_3).wrapping_add(12)))));
        { let v_ = l_x_4; let a_ = p_out_1; c.m.w32(a_, (v_).0); };
        { let v_ = l_y_5; let a_ = (p_out_1).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = l_z_6; let a_ = (p_out_1).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = l_w_7; let a_ = (p_out_1).wrapping_add(12); c.m.w32(a_, (v_).0); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Quat_Mul", &PROBE_N, &[], true); }
    r_
}

pub fn math3d__Quat_Dot(c: &mut Ctx, mut p_a_1: u32, mut p_b_2: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "math3d__Quat_Dot", &PROBE_N, &[(p_a_1) as u32, (p_b_2) as u32], false); }
    let r_ = 'f: {
    break 'f rt::fadd(rt::fadd(rt::fadd(rt::fmul(F32(c.m.r32(p_a_1)), F32(c.m.r32(p_b_2))), rt::fmul(F32(c.m.r32((p_a_1).wrapping_add(4))), F32(c.m.r32((p_b_2).wrapping_add(4))))), rt::fmul(F32(c.m.r32((p_a_1).wrapping_add(8))), F32(c.m.r32((p_b_2).wrapping_add(8))))), rt::fmul(F32(c.m.r32((p_a_1).wrapping_add(12))), F32(c.m.r32((p_b_2).wrapping_add(12)))));
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "math3d__Quat_Dot", &PROBE_N, &[], true); }
    r_
}

pub fn Quat_Slerp(c: &mut Ctx, mut p_out_1: u32, mut p_a_2: u32, mut p_b_3: u32, mut p_t_4: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Quat_Slerp", &PROBE_N, &[(p_out_1) as u32, (p_a_2) as u32, (p_b_3) as u32, (p_t_4).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_dot_5: F32 = F32(0);
    let mut l_c_6: F32 = F32(0);
    let mut l_angle_7: F32 = F32(0);
    let mut l_s_8: F32 = F32(0);
    let mut l_inv_9: F32 = F32(0);
    let mut l_ka_10: F32 = F32(0);
    let mut l_kb_11: F32 = F32(0);
    let r_ = 'f: {
        F::Vec4_Copy(c, fp, p_a_2);
        F::Vec4_Copy(c, fp.wrapping_add(16), p_b_3);
        l_dot_5 = F::math3d__Quat_Dot(c, fp, fp.wrapping_add(16));
        if (((rt::flt(l_dot_5, F32(0x00000000)) as i32)) != 0) {
            { let v_ = rt::fneg(F32(c.m.r32(fp.wrapping_add(16)))); let a_ = fp.wrapping_add(16); c.m.w32(a_, (v_).0); };
            { let v_ = rt::fneg(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4)))); let a_ = (fp.wrapping_add(16)).wrapping_add(4); c.m.w32(a_, (v_).0); };
            { let v_ = rt::fneg(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8)))); let a_ = (fp.wrapping_add(16)).wrapping_add(8); c.m.w32(a_, (v_).0); };
            { let v_ = rt::fneg(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(12)))); let a_ = (fp.wrapping_add(16)).wrapping_add(12); c.m.w32(a_, (v_).0); };
            l_c_6 = F::math3d__Quat_Dot(c, fp, fp.wrapping_add(16));
        } else {
            l_c_6 = l_dot_5;
        }
        if (((rt::fgt(l_c_6, F32(0x3f800000)) as i32)) != 0) {
            l_c_6 = F32(0x3f800000);
        }
        l_angle_7 = F::Mathf_Acos(c, l_c_6);
        l_s_8 = F::Port_sinf(c, l_angle_7);
        if (((rt::flt(l_s_8, F32(0x358637bd)) as i32)) != 0) {
            { let v_ = F32(c.m.r32(fp)); let a_ = p_out_1; c.m.w32(a_, (v_).0); };
            { let v_ = F32(c.m.r32((fp).wrapping_add(4))); let a_ = (p_out_1).wrapping_add(4); c.m.w32(a_, (v_).0); };
            { let v_ = F32(c.m.r32((fp).wrapping_add(8))); let a_ = (p_out_1).wrapping_add(8); c.m.w32(a_, (v_).0); };
            { let v_ = F32(c.m.r32((fp).wrapping_add(12))); let a_ = (p_out_1).wrapping_add(12); c.m.w32(a_, (v_).0); };
        } else {
            l_inv_9 = rt::fdiv(F32(0x3f800000), l_s_8);
            l_ka_10 = rt::fmul(F::Port_sinf(c, rt::fmul(l_angle_7, rt::fsub(F32(0x3f800000), p_t_4))), l_inv_9);
            l_kb_11 = rt::fmul(F::Port_sinf(c, rt::fmul(l_angle_7, p_t_4)), l_inv_9);
            { let v_ = rt::fadd(rt::fmul(F32(c.m.r32(fp)), l_ka_10), rt::fmul(F32(c.m.r32(fp.wrapping_add(16))), l_kb_11)); let a_ = p_out_1; c.m.w32(a_, (v_).0); };
            { let v_ = rt::fadd(rt::fmul(F32(c.m.r32((fp).wrapping_add(4))), l_ka_10), rt::fmul(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))), l_kb_11)); let a_ = (p_out_1).wrapping_add(4); c.m.w32(a_, (v_).0); };
            { let v_ = rt::fadd(rt::fmul(F32(c.m.r32((fp).wrapping_add(8))), l_ka_10), rt::fmul(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8))), l_kb_11)); let a_ = (p_out_1).wrapping_add(8); c.m.w32(a_, (v_).0); };
            { let v_ = rt::fadd(rt::fmul(F32(c.m.r32((fp).wrapping_add(12))), l_ka_10), rt::fmul(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(12))), l_kb_11)); let a_ = (p_out_1).wrapping_add(12); c.m.w32(a_, (v_).0); };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Quat_Slerp", &PROBE_N, &[], true); }
    r_
}

pub fn Quat_ConjugateBy(c: &mut Ctx, mut p_out_1: u32, mut p_q_2: u32, mut p_r_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Quat_ConjugateBy", &PROBE_N, &[(p_out_1) as u32, (p_q_2) as u32, (p_r_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let r_ = 'f: {
        F::Quat_Inverse(c, fp, p_r_3);
        F::Quat_Mul(c, fp.wrapping_add(16), p_q_2, fp);
        F::Quat_Mul(c, fp.wrapping_add(16), p_r_3, fp.wrapping_add(16));
        { let v_ = F32(c.m.r32(fp.wrapping_add(16))); let a_ = p_out_1; c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))); let a_ = (p_out_1).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8))); let a_ = (p_out_1).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(12))); let a_ = (p_out_1).wrapping_add(12); c.m.w32(a_, (v_).0); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Quat_ConjugateBy", &PROBE_N, &[], true); }
    r_
}

pub fn Quat_LimitAngle(c: &mut Ctx, mut p_out_1: u32, mut p_q_2: u32, mut p_maxAngle_3: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Quat_LimitAngle", &PROBE_N, &[(p_out_1) as u32, (p_q_2) as u32, (p_maxAngle_3).0], false); }
    let r_ = 'f: {
    if (((rt::fge(F::Port_fabsf(c, p_maxAngle_3), F32(0x40490fda)) as i32)) != 0) {
        F::Vec4_Copy(c, p_out_1, p_q_2);
        break 'f;
    }
    if (((rt::fgt({ let a0_ = F32(c.m.r32((p_q_2).wrapping_add(12))); F::Port_fabsf(c, a0_) }, F32(0x3f7fffef)) as i32)) != 0) {
        F::Vec4_Copy(c, p_out_1, p_q_2);
        break 'f;
    }
    if (((rt::flt(F::Port_cosf(c, rt::fmul(p_maxAngle_3, F32(0x3f000000))), F32(c.m.r32((p_q_2).wrapping_add(12)))) as i32)) != 0) {
        F::Vec4_Copy(c, p_out_1, p_q_2);
        break 'f;
    }
    { let a1_ = F32(c.m.r32(p_q_2)); let a2_ = F32(c.m.r32((p_q_2).wrapping_add(4))); let a3_ = F32(c.m.r32((p_q_2).wrapping_add(8))); F::Quat_FromAxisAngle(c, p_out_1, a1_, a2_, a3_, p_maxAngle_3) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Quat_LimitAngle", &PROBE_N, &[], true); }
    r_
}

pub fn Quat_Normalize(c: &mut Ctx, mut p_out_1: u32, mut p_q_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Quat_Normalize", &PROBE_N, &[(p_out_1) as u32, (p_q_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_inv_3: F32 = F32(0);
    let r_ = 'f: {
        l_inv_3 = rt::fdiv(F32(0x3f800000), F::Quat_Length(c, p_out_1));
        { let v_ = rt::fmul(F32(c.m.r32(p_q_2)), l_inv_3); let a_ = fp; c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(F32(c.m.r32((p_q_2).wrapping_add(4))), l_inv_3); let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(F32(c.m.r32((p_q_2).wrapping_add(8))), l_inv_3); let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(F32(c.m.r32((p_q_2).wrapping_add(12))), l_inv_3); let a_ = (fp).wrapping_add(12); c.m.w32(a_, (v_).0); };
        F::Vec4_Copy(c, p_out_1, fp);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Quat_Normalize", &PROBE_N, &[], true); }
    r_
}

pub fn Quat_Pack(c: &mut Ctx, mut p_q_1: u32) -> u64 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Quat_Pack", &PROBE_N, &[(p_q_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_largest_2: i32 = 0;
    let mut l_i_3: i32 = 0;
    let mut l_best_4: F32 = F32(0);
    let mut l_packed_5: u64 = 0;
    let mut l_n_6: i32 = 0;
    let r_ = 'f: {
        F::Vec4_Copy(c, fp, p_q_1);
        l_largest_2 = 0i32;
        l_best_4 = { let a0_ = F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); F::Port_fabsf(c, a0_) };
        {
            l_i_3 = 1i32;
            'l1: loop {
                if !(((((l_i_3) < (4i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    if (((rt::flt(l_best_4, { let a0_ = F32(c.m.r32((fp).wrapping_add((((l_i_3) as u32)).wrapping_mul(4)))); F::Port_fabsf(c, a0_) }) as i32)) != 0) {
                        l_best_4 = { let a0_ = F32(c.m.r32((fp).wrapping_add((((l_i_3) as u32)).wrapping_mul(4)))); F::Port_fabsf(c, a0_) };
                        l_largest_2 = l_i_3;
                    }
                }
                { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
            }
        }
        if (((rt::flt(F32(c.m.r32((fp).wrapping_add((((l_largest_2) as u32)).wrapping_mul(4)))), F32(0x00000000)) as i32)) != 0) {
            { let v_ = rt::fneg(F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            { let v_ = rt::fneg(F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            { let v_ = rt::fneg(F32(c.m.r32((fp).wrapping_add((((2i32) as u32)).wrapping_mul(4))))); let a_ = (fp).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            { let v_ = rt::fneg(F32(c.m.r32((fp).wrapping_add((((3i32) as u32)).wrapping_mul(4))))); let a_ = (fp).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        }
        {
            l_i_3 = l_largest_2;
            'l3: loop {
                if !(((((l_i_3) < (3i32)) as i32)) != 0) { break 'l3; }
                'c4: {
                    { let v_ = F32(c.m.r32((fp).wrapping_add(((((l_i_3).wrapping_add(1i32)) as u32)).wrapping_mul(4)))); let a_ = (fp).wrapping_add((((l_i_3) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                }
                { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
            }
        }
        l_packed_5 = ((0i32) as u64);
        {
            l_i_3 = 0i32;
            'l5: loop {
                if !(((((l_i_3) < (3i32)) as i32)) != 0) { break 'l5; }
                'c6: {
                    l_n_6 = rt::f2i(rt::fmul(rt::fadd(rt::fmul(F32(c.m.r32((fp).wrapping_add((((l_i_3) as u32)).wrapping_mul(4)))), F32(0x3f3504f3)), F32(0x3f000000)), F32(0x497ffff0)));
                    if (((((l_n_6) < (0i32)) as i32)) != 0) {
                        l_n_6 = 0i32;
                    }
                    if (((((l_n_6) > (1048575i32)) as i32)) != 0) {
                        l_n_6 = 1048575i32;
                    }
                    { let r_ = (((l_n_6) as u64)).wrapping_shl(((l_i_3).wrapping_mul(20i32)) as u32); let n_ = ((l_packed_5) | (r_)); l_packed_5 = n_; };
                }
                { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
            }
        }
        break 'f ((l_packed_5) | ((((l_largest_2) as u64)).wrapping_shl((60i32) as u32)));
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Quat_Pack", &PROBE_N, &[], true); }
    r_
}

pub fn Quat_Unpack(c: &mut Ctx, mut p_out_1: u32, mut p_packed_2: u64) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Quat_Unpack", &PROBE_N, &[(p_out_1) as u32, (p_packed_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_largest_3: i32 = 0;
    let mut l_i_4: i32 = 0;
    let mut l_sum_5: F32 = F32(0);
    let mut l_w_6: F32 = F32(0);
    let r_ = 'f: {
        l_largest_3 = (((p_packed_2).wrapping_shr((60i32) as u32)) as i32);
        {
            l_i_4 = 0i32;
            'l1: loop {
                if !(((((l_i_4) < (3i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_sum_5 = rt::u2f((((((p_packed_2).wrapping_shr(((l_i_4).wrapping_mul(20i32)) as u32)) as u32)) & (((1048575i32) as u32))));
                    { let v_ = rt::fmul(rt::fsub(rt::fmul(l_sum_5, F32(0x35800008)), F32(0x3f000000)), F32(0x3fb504f3)); let a_ = (fp).wrapping_add((((l_i_4) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                }
                { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
            }
        }
        l_sum_5 = F32(0x00000000);
        {
            l_i_4 = 0i32;
            'l3: loop {
                if !(((((l_i_4) < (3i32)) as i32)) != 0) { break 'l3; }
                'c4: {
                    { let r_ = rt::fmul(F32(c.m.r32((fp).wrapping_add((((l_i_4) as u32)).wrapping_mul(4)))), F32(c.m.r32((fp).wrapping_add((((l_i_4) as u32)).wrapping_mul(4))))); let n_ = rt::fadd(l_sum_5, r_); l_sum_5 = n_; };
                }
                { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
            }
        }
        l_w_6 = F::Port_sqrtf(c, rt::fsub(F32(0x3f800000), l_sum_5));
        {
            l_i_4 = 3i32;
            'l5: loop {
                if !(((((l_i_4) > (l_largest_3)) as i32)) != 0) { break 'l5; }
                'c6: {
                    { let v_ = F32(c.m.r32((fp).wrapping_add(((((l_i_4).wrapping_sub(1i32)) as u32)).wrapping_mul(4)))); let a_ = (fp).wrapping_add((((l_i_4) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                }
                { let o_ = l_i_4; let n_ = (o_).wrapping_sub(1); l_i_4 = n_; };
            }
        }
        { let v_ = l_w_6; let a_ = (fp).wrapping_add((((l_largest_3) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        F::Vec4_Copy(c, p_out_1, fp);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Quat_Unpack", &PROBE_N, &[], true); }
    r_
}

pub fn Quat_ToMtx(c: &mut Ctx, mut p_out_1: u32, mut p_q_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Quat_ToMtx", &PROBE_N, &[(p_out_1) as u32, (p_q_2) as u32], false); }
    let mut l_x_3: F32 = F32(0);
    let mut l_y_4: F32 = F32(0);
    let mut l_z_5: F32 = F32(0);
    let mut l_w_6: F32 = F32(0);
    let mut l_xx_7: F32 = F32(0);
    let mut l_yy_8: F32 = F32(0);
    let r_ = 'f: {
    l_x_3 = F32(c.m.r32(p_q_2));
    l_y_4 = F32(c.m.r32((p_q_2).wrapping_add(4)));
    l_z_5 = F32(c.m.r32((p_q_2).wrapping_add(8)));
    l_w_6 = F32(c.m.r32((p_q_2).wrapping_add(12)));
    l_xx_7 = rt::fmul(l_x_3, l_x_3);
    l_yy_8 = rt::fmul(l_y_4, l_y_4);
    F::Mtx_StoreIdentity(c, p_out_1);
    { let v_ = rt::fsub(rt::fsub(F32(0x3f800000), rt::fmul(F32(0x40000000), l_yy_8)), rt::fmul(F32(0x40000000), rt::fmul(l_z_5, l_z_5))); let a_ = ((p_out_1).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fadd(rt::fmul(rt::fmul(F32(0x40000000), l_x_3), l_y_4), rt::fmul(rt::fmul(F32(0x40000000), l_w_6), l_z_5)); let a_ = ((p_out_1).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fsub(rt::fmul(rt::fmul(F32(0x40000000), l_x_3), l_z_5), rt::fmul(rt::fmul(F32(0x40000000), l_w_6), l_y_4)); let a_ = ((p_out_1).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fsub(rt::fmul(rt::fmul(F32(0x40000000), l_x_3), l_y_4), rt::fmul(rt::fmul(F32(0x40000000), l_w_6), l_z_5)); let a_ = ((p_out_1).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fsub(rt::fsub(F32(0x3f800000), rt::fmul(F32(0x40000000), l_xx_7)), rt::fmul(F32(0x40000000), rt::fmul(l_z_5, l_z_5))); let a_ = ((p_out_1).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fadd(rt::fmul(rt::fmul(F32(0x40000000), l_y_4), l_z_5), rt::fmul(rt::fmul(F32(0x40000000), l_w_6), l_x_3)); let a_ = ((p_out_1).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fadd(rt::fmul(rt::fmul(F32(0x40000000), l_x_3), l_z_5), rt::fmul(rt::fmul(F32(0x40000000), l_w_6), l_y_4)); let a_ = ((p_out_1).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fsub(rt::fmul(rt::fmul(F32(0x40000000), l_y_4), l_z_5), rt::fmul(rt::fmul(F32(0x40000000), l_w_6), l_x_3)); let a_ = ((p_out_1).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fsub(rt::fsub(F32(0x3f800000), rt::fmul(F32(0x40000000), l_xx_7)), rt::fmul(F32(0x40000000), l_yy_8)); let a_ = ((p_out_1).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Quat_ToMtx", &PROBE_N, &[], true); }
    r_
}

pub fn Quat_FromEuler(c: &mut Ctx, mut p_out_1: u32, mut p_angles_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Quat_FromEuler", &PROBE_N, &[(p_out_1) as u32, (p_angles_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let r_ = 'f: {
        { let a4_ = F32(c.m.r32(p_angles_2)); F::Quat_FromAxisAngle(c, fp, F32(0x3f800000), F32(0x00000000), F32(0x00000000), a4_) };
        { let a4_ = F32(c.m.r32((p_angles_2).wrapping_add(4))); F::Quat_FromAxisAngle(c, fp.wrapping_add(16), F32(0x00000000), F32(0x3f800000), F32(0x00000000), a4_) };
        { let a4_ = F32(c.m.r32((p_angles_2).wrapping_add(8))); F::Quat_FromAxisAngle(c, fp.wrapping_add(32), F32(0x00000000), F32(0x00000000), F32(0x3f800000), a4_) };
        F::Quat_Mul(c, p_out_1, fp, fp.wrapping_add(32));
        F::Quat_Mul(c, p_out_1, fp.wrapping_add(16), p_out_1);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Quat_FromEuler", &PROBE_N, &[], true); }
    r_
}

pub fn Mtx_ToEuler(c: &mut Ctx, mut p_out_1: u32, mut p_m_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Mtx_ToEuler", &PROBE_N, &[(p_out_1) as u32, (p_m_2) as u32], false); }
    let mut l_s_3: F32 = F32(0);
    let r_ = 'f: {
    l_s_3 = rt::fneg(F32(c.m.r32(((p_m_2).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))));
    if (((rt::flt(l_s_3, F32(0xbf800000)) as i32)) != 0) {
        l_s_3 = F32(0xbf800000);
    }
    if (((rt::fgt(l_s_3, F32(0x3f800000)) as i32)) != 0) {
        l_s_3 = F32(0x3f800000);
    }
    { let v_ = F::Mathf_Asin(c, l_s_3); let a_ = p_out_1; c.m.w32(a_, (v_).0); };
    if (((rt::flt(F32(c.m.r32(((p_m_2).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(0x3f7ff972)) as i32)) != 0) {
        if (((rt::fgt(F32(c.m.r32(((p_m_2).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(0xbf7ff972)) as i32)) != 0) {
            { let v_ = { let a0_ = F32(c.m.r32(((p_m_2).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a1_ = F32(c.m.r32(((p_m_2).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); F::Port_atan2f(c, a0_, a1_) }; let a_ = (p_out_1).wrapping_add(4); c.m.w32(a_, (v_).0); };
            { let v_ = { let a0_ = F32(c.m.r32(((p_m_2).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a1_ = F32(c.m.r32(((p_m_2).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); F::Port_atan2f(c, a0_, a1_) }; let a_ = (p_out_1).wrapping_add(8); c.m.w32(a_, (v_).0); };
        } else {
            { let v_ = { let a0_ = F32(c.m.r32(((p_m_2).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a1_ = F32(c.m.r32(((p_m_2).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); F::Port_atan2f(c, a0_, a1_) }; let a_ = (p_out_1).wrapping_add(4); c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x00000000); let a_ = (p_out_1).wrapping_add(8); c.m.w32(a_, (v_).0); };
        }
    } else {
        { let v_ = rt::fneg({ let a0_ = F32(c.m.r32(((p_m_2).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a1_ = F32(c.m.r32(((p_m_2).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); F::Port_atan2f(c, a0_, a1_) }); let a_ = (p_out_1).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (p_out_1).wrapping_add(8); c.m.w32(a_, (v_).0); };
    }
    { let v_ = F32(0x00000000); let a_ = (p_out_1).wrapping_add(12); c.m.w32(a_, (v_).0); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Mtx_ToEuler", &PROBE_N, &[], true); }
    r_
}

pub fn Quat_ToEuler(c: &mut Ctx, mut p_out_1: u32, mut p_q_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Quat_ToEuler", &PROBE_N, &[(p_out_1) as u32, (p_q_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(64) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 64);
    let r_ = 'f: {
        F::Quat_ToMtx(c, fp, p_q_2);
        F::Mtx_ToEuler(c, p_out_1, fp);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Quat_ToEuler", &PROBE_N, &[], true); }
    r_
}

pub fn Quat_FromMtx(c: &mut Ctx, mut p_out_1: u32, mut p_m_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Quat_FromMtx", &PROBE_N, &[(p_out_1) as u32, (p_m_2) as u32], false); }
    let mut l_tr_3: F32 = F32(0);
    let mut l_s_4: F32 = F32(0);
    let r_ = 'f: {
    l_tr_3 = rt::fadd(rt::fadd(rt::fadd(F32(c.m.r32(((p_m_2).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(0x3f800000)), F32(c.m.r32(((p_m_2).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4))))), F32(c.m.r32(((p_m_2).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)))));
    if (((rt::fgt(l_tr_3, F32(0x38d1b717)) as i32)) != 0) {
        l_s_4 = rt::fdiv(F32(0x3f000000), F::Port_sqrtf(c, l_tr_3));
        { let v_ = rt::fdiv(F32(0x3e800000), l_s_4); let a_ = (p_out_1).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(rt::fsub(F32(c.m.r32(((p_m_2).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((p_m_2).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4))))), l_s_4); let a_ = p_out_1; c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(rt::fsub(F32(c.m.r32(((p_m_2).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((p_m_2).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4))))), l_s_4); let a_ = (p_out_1).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(rt::fsub(F32(c.m.r32(((p_m_2).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((p_m_2).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4))))), l_s_4); let a_ = (p_out_1).wrapping_add(8); c.m.w32(a_, (v_).0); };
    } else {
        if (((((((rt::fgt(F32(c.m.r32(((p_m_2).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((p_m_2).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4))))) as i32)) != 0) && (((rt::fgt(F32(c.m.r32(((p_m_2).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((p_m_2).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4))))) as i32)) != 0)) as i32)) != 0) {
            l_s_4 = rt::fmul({ let a0_ = rt::fsub(rt::fsub(rt::fadd(F32(c.m.r32(((p_m_2).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(0x3f800000)), F32(c.m.r32(((p_m_2).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4))))), F32(c.m.r32(((p_m_2).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4))))); F::Port_sqrtf(c, a0_) }, F32(0x40000000));
            { let v_ = rt::fmul(l_s_4, F32(0x3e800000)); let a_ = p_out_1; c.m.w32(a_, (v_).0); };
            { let v_ = rt::fdiv(rt::fadd(F32(c.m.r32(((p_m_2).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((p_m_2).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4))))), l_s_4); let a_ = (p_out_1).wrapping_add(4); c.m.w32(a_, (v_).0); };
            { let v_ = rt::fdiv(rt::fadd(F32(c.m.r32(((p_m_2).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((p_m_2).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4))))), l_s_4); let a_ = (p_out_1).wrapping_add(8); c.m.w32(a_, (v_).0); };
            { let v_ = rt::fdiv(rt::fsub(F32(c.m.r32(((p_m_2).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((p_m_2).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4))))), l_s_4); let a_ = (p_out_1).wrapping_add(12); c.m.w32(a_, (v_).0); };
        } else {
            if (((rt::fgt(F32(c.m.r32(((p_m_2).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((p_m_2).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4))))) as i32)) != 0) {
                l_s_4 = rt::fmul({ let a0_ = rt::fsub(rt::fsub(rt::fadd(F32(c.m.r32(((p_m_2).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(0x3f800000)), F32(c.m.r32(((p_m_2).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4))))), F32(c.m.r32(((p_m_2).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4))))); F::Port_sqrtf(c, a0_) }, F32(0x40000000));
                { let v_ = rt::fdiv(rt::fadd(F32(c.m.r32(((p_m_2).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((p_m_2).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4))))), l_s_4); let a_ = p_out_1; c.m.w32(a_, (v_).0); };
                { let v_ = rt::fmul(l_s_4, F32(0x3e800000)); let a_ = (p_out_1).wrapping_add(4); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fdiv(rt::fadd(F32(c.m.r32(((p_m_2).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((p_m_2).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4))))), l_s_4); let a_ = (p_out_1).wrapping_add(8); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fdiv(rt::fsub(F32(c.m.r32(((p_m_2).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((p_m_2).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4))))), l_s_4); let a_ = (p_out_1).wrapping_add(12); c.m.w32(a_, (v_).0); };
            } else {
                l_s_4 = rt::fmul({ let a0_ = rt::fsub(rt::fsub(rt::fadd(F32(c.m.r32(((p_m_2).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), F32(0x3f800000)), F32(c.m.r32(((p_m_2).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4))))), F32(c.m.r32(((p_m_2).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); F::Port_sqrtf(c, a0_) }, F32(0x40000000));
                { let v_ = rt::fdiv(rt::fadd(F32(c.m.r32(((p_m_2).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((p_m_2).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4))))), l_s_4); let a_ = p_out_1; c.m.w32(a_, (v_).0); };
                { let v_ = rt::fdiv(rt::fadd(F32(c.m.r32(((p_m_2).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((p_m_2).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4))))), l_s_4); let a_ = (p_out_1).wrapping_add(4); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fmul(l_s_4, F32(0x3e800000)); let a_ = (p_out_1).wrapping_add(8); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fdiv(rt::fsub(F32(c.m.r32(((p_m_2).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((p_m_2).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4))))), l_s_4); let a_ = (p_out_1).wrapping_add(12); c.m.w32(a_, (v_).0); };
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Quat_FromMtx", &PROBE_N, &[], true); }
    r_
}
