#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn Vec3_ScaleNeg(c: &mut Ctx, mut p_out_1: u32, mut p_v_2: u32, mut p_scale_3: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vec3_ScaleNeg", &PROBE_N, &[(p_out_1) as u32, (p_v_2) as u32, (p_scale_3).0], false); }
    let r_ = 'f: {
    p_scale_3 = rt::fneg(p_scale_3);
    { let v_ = rt::fmul(p_scale_3, F32(c.m.r32(p_v_2))); let a_ = p_out_1; c.m.w32(a_, (v_).0); };
    { let v_ = rt::fmul(p_scale_3, F32(c.m.r32((p_v_2).wrapping_add(4)))); let a_ = (p_out_1).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fmul(p_scale_3, F32(c.m.r32((p_v_2).wrapping_add(8)))); let a_ = (p_out_1).wrapping_add(8); c.m.w32(a_, (v_).0); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Vec3_ScaleNeg", &PROBE_N, &[], true); }
    r_
}

pub fn Rigid_EulerAccel(c: &mut Ctx, mut p_out_1: u32, mut p_angVel_2: u32, mut p_torque_3: u32, mut p_inertia_4: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Rigid_EulerAccel", &PROBE_N, &[(p_out_1) as u32, (p_angVel_2) as u32, (p_torque_3) as u32, (p_inertia_4) as u32], false); }
    let r_ = 'f: {
    { let v_ = rt::fdiv(rt::fadd(F32(c.m.r32(p_torque_3)), rt::fmul(rt::fmul(rt::fsub(F32(c.m.r32((p_inertia_4).wrapping_add(4))), F32(c.m.r32((p_inertia_4).wrapping_add(8)))), F32(c.m.r32((p_angVel_2).wrapping_add(4)))), F32(c.m.r32((p_angVel_2).wrapping_add(8))))), F32(c.m.r32(p_inertia_4))); let a_ = p_out_1; c.m.w32(a_, (v_).0); };
    { let v_ = rt::fdiv(rt::fadd(F32(c.m.r32((p_torque_3).wrapping_add(4))), rt::fmul(rt::fmul(rt::fsub(F32(c.m.r32((p_inertia_4).wrapping_add(8))), F32(c.m.r32(p_inertia_4))), F32(c.m.r32((p_angVel_2).wrapping_add(8)))), F32(c.m.r32(p_angVel_2)))), F32(c.m.r32((p_inertia_4).wrapping_add(4)))); let a_ = (p_out_1).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fdiv(rt::fadd(F32(c.m.r32((p_torque_3).wrapping_add(8))), rt::fmul(rt::fmul(rt::fsub(F32(c.m.r32(p_inertia_4)), F32(c.m.r32((p_inertia_4).wrapping_add(4)))), F32(c.m.r32(p_angVel_2))), F32(c.m.r32((p_angVel_2).wrapping_add(4))))), F32(c.m.r32((p_inertia_4).wrapping_add(8)))); let a_ = (p_out_1).wrapping_add(8); c.m.w32(a_, (v_).0); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Rigid_EulerAccel", &PROBE_N, &[], true); }
    r_
}

pub fn Mat3_SetSkew(c: &mut Ctx, mut p_out_1: u32, mut p_v_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Mat3_SetSkew", &PROBE_N, &[(p_out_1) as u32, (p_v_2) as u32], false); }
    let r_ = 'f: {
    { let v_ = F32(0x00000000); let a_ = ((p_out_1).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((p_v_2).wrapping_add(8))); let a_ = ((p_out_1).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fneg(F32(c.m.r32((p_v_2).wrapping_add(4)))); let a_ = ((p_out_1).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fneg(F32(c.m.r32((p_v_2).wrapping_add(8)))); let a_ = ((p_out_1).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x00000000); let a_ = ((p_out_1).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32(p_v_2)); let a_ = ((p_out_1).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((p_v_2).wrapping_add(4))); let a_ = ((p_out_1).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fneg(F32(c.m.r32(p_v_2))); let a_ = ((p_out_1).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x00000000); let a_ = ((p_out_1).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Mat3_SetSkew", &PROBE_N, &[], true); }
    r_
}

pub fn Mat3_Mul(c: &mut Ctx, mut p_out_1: u32, mut p_a_2: u32, mut p_b_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Mat3_Mul", &PROBE_N, &[(p_out_1) as u32, (p_a_2) as u32, (p_b_3) as u32], false); }
    let r_ = 'f: {
    { let v_ = rt::fadd(rt::fadd(rt::fmul(F32(c.m.r32(((p_a_2).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((p_b_3).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4))))), rt::fmul(F32(c.m.r32(((p_a_2).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((p_b_3).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))))), rt::fmul(F32(c.m.r32(((p_a_2).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((p_b_3).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))))); let a_ = ((p_out_1).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fadd(rt::fadd(rt::fmul(F32(c.m.r32(((p_a_2).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((p_b_3).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4))))), rt::fmul(F32(c.m.r32(((p_a_2).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((p_b_3).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))))), rt::fmul(F32(c.m.r32(((p_a_2).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((p_b_3).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))))); let a_ = ((p_out_1).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fadd(rt::fadd(rt::fmul(F32(c.m.r32(((p_a_2).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((p_b_3).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4))))), rt::fmul(F32(c.m.r32(((p_a_2).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((p_b_3).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)))))), rt::fmul(F32(c.m.r32(((p_a_2).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((p_b_3).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)))))); let a_ = ((p_out_1).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fadd(rt::fadd(rt::fmul(F32(c.m.r32(((p_a_2).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((p_b_3).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4))))), rt::fmul(F32(c.m.r32(((p_a_2).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((p_b_3).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))))), rt::fmul(F32(c.m.r32(((p_a_2).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((p_b_3).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))))); let a_ = ((p_out_1).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fadd(rt::fadd(rt::fmul(F32(c.m.r32(((p_a_2).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((p_b_3).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4))))), rt::fmul(F32(c.m.r32(((p_a_2).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((p_b_3).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))))), rt::fmul(F32(c.m.r32(((p_a_2).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((p_b_3).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))))); let a_ = ((p_out_1).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fadd(rt::fadd(rt::fmul(F32(c.m.r32(((p_a_2).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((p_b_3).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4))))), rt::fmul(F32(c.m.r32(((p_a_2).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((p_b_3).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)))))), rt::fmul(F32(c.m.r32(((p_a_2).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((p_b_3).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)))))); let a_ = ((p_out_1).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fadd(rt::fadd(rt::fmul(F32(c.m.r32(((p_a_2).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((p_b_3).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4))))), rt::fmul(F32(c.m.r32(((p_a_2).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((p_b_3).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))))), rt::fmul(F32(c.m.r32(((p_a_2).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((p_b_3).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))))); let a_ = ((p_out_1).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fadd(rt::fadd(rt::fmul(F32(c.m.r32(((p_a_2).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((p_b_3).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4))))), rt::fmul(F32(c.m.r32(((p_a_2).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((p_b_3).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))))), rt::fmul(F32(c.m.r32(((p_a_2).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((p_b_3).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))))); let a_ = ((p_out_1).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fadd(rt::fadd(rt::fmul(F32(c.m.r32(((p_a_2).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((p_b_3).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4))))), rt::fmul(F32(c.m.r32(((p_a_2).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((p_b_3).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)))))), rt::fmul(F32(c.m.r32(((p_a_2).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((p_b_3).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)))))); let a_ = ((p_out_1).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Mat3_Mul", &PROBE_N, &[], true); }
    r_
}

pub fn Mat3_Add(c: &mut Ctx, mut p_out_1: u32, mut p_a_2: u32, mut p_b_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Mat3_Add", &PROBE_N, &[(p_out_1) as u32, (p_a_2) as u32, (p_b_3) as u32], false); }
    let r_ = 'f: {
    { let v_ = rt::fadd(F32(c.m.r32(((p_a_2).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((p_b_3).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a_ = ((p_out_1).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fadd(F32(c.m.r32(((p_a_2).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((p_b_3).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); let a_ = ((p_out_1).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fadd(F32(c.m.r32(((p_a_2).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((p_b_3).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4))))); let a_ = ((p_out_1).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fadd(F32(c.m.r32(((p_a_2).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((p_b_3).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a_ = ((p_out_1).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fadd(F32(c.m.r32(((p_a_2).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((p_b_3).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); let a_ = ((p_out_1).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fadd(F32(c.m.r32(((p_a_2).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((p_b_3).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4))))); let a_ = ((p_out_1).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fadd(F32(c.m.r32(((p_a_2).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((p_b_3).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a_ = ((p_out_1).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fadd(F32(c.m.r32(((p_a_2).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((p_b_3).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); let a_ = ((p_out_1).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fadd(F32(c.m.r32(((p_a_2).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((p_b_3).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4))))); let a_ = ((p_out_1).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Mat3_Add", &PROBE_N, &[], true); }
    r_
}

pub fn Mat3_MulVecCols(c: &mut Ctx, mut p_out_1: u32, mut p_v_2: u32, mut p_m_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Mat3_MulVecCols", &PROBE_N, &[(p_out_1) as u32, (p_v_2) as u32, (p_m_3) as u32], false); }
    let r_ = 'f: {
    { let v_ = rt::fadd(rt::fadd(rt::fmul(F32(c.m.r32(p_v_2)), F32(c.m.r32(((p_m_3).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4))))), rt::fmul(F32(c.m.r32((p_v_2).wrapping_add(4))), F32(c.m.r32(((p_m_3).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))))), rt::fmul(F32(c.m.r32((p_v_2).wrapping_add(8))), F32(c.m.r32(((p_m_3).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)))))); let a_ = p_out_1; c.m.w32(a_, (v_).0); };
    { let v_ = rt::fadd(rt::fadd(rt::fmul(F32(c.m.r32(p_v_2)), F32(c.m.r32(((p_m_3).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4))))), rt::fmul(F32(c.m.r32((p_v_2).wrapping_add(4))), F32(c.m.r32(((p_m_3).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))))), rt::fmul(F32(c.m.r32((p_v_2).wrapping_add(8))), F32(c.m.r32(((p_m_3).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)))))); let a_ = (p_out_1).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fadd(rt::fadd(rt::fmul(F32(c.m.r32(p_v_2)), F32(c.m.r32(((p_m_3).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4))))), rt::fmul(F32(c.m.r32((p_v_2).wrapping_add(4))), F32(c.m.r32(((p_m_3).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))))), rt::fmul(F32(c.m.r32((p_v_2).wrapping_add(8))), F32(c.m.r32(((p_m_3).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)))))); let a_ = (p_out_1).wrapping_add(8); c.m.w32(a_, (v_).0); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Mat3_MulVecCols", &PROBE_N, &[], true); }
    r_
}

pub fn Mat3_MulVecRows(c: &mut Ctx, mut p_out_1: u32, mut p_v_2: u32, mut p_m_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Mat3_MulVecRows", &PROBE_N, &[(p_out_1) as u32, (p_v_2) as u32, (p_m_3) as u32], false); }
    let r_ = 'f: {
    { let v_ = rt::fadd(rt::fadd(rt::fmul(F32(c.m.r32(p_v_2)), F32(c.m.r32(((p_m_3).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4))))), rt::fmul(F32(c.m.r32((p_v_2).wrapping_add(4))), F32(c.m.r32(((p_m_3).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))))), rt::fmul(F32(c.m.r32((p_v_2).wrapping_add(8))), F32(c.m.r32(((p_m_3).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))))); let a_ = p_out_1; c.m.w32(a_, (v_).0); };
    { let v_ = rt::fadd(rt::fadd(rt::fmul(F32(c.m.r32(p_v_2)), F32(c.m.r32(((p_m_3).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4))))), rt::fmul(F32(c.m.r32((p_v_2).wrapping_add(4))), F32(c.m.r32(((p_m_3).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))))), rt::fmul(F32(c.m.r32((p_v_2).wrapping_add(8))), F32(c.m.r32(((p_m_3).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))))); let a_ = (p_out_1).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fadd(rt::fadd(rt::fmul(F32(c.m.r32(p_v_2)), F32(c.m.r32(((p_m_3).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4))))), rt::fmul(F32(c.m.r32((p_v_2).wrapping_add(4))), F32(c.m.r32(((p_m_3).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)))))), rt::fmul(F32(c.m.r32((p_v_2).wrapping_add(8))), F32(c.m.r32(((p_m_3).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)))))); let a_ = (p_out_1).wrapping_add(8); c.m.w32(a_, (v_).0); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Mat3_MulVecRows", &PROBE_N, &[], true); }
    r_
}

pub fn Mat3_Orthonormalize(c: &mut Ctx, mut p_m_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Mat3_Orthonormalize", &PROBE_N, &[(p_m_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let r_ = 'f: {
        { let v_ = F32(c.m.r32(((p_m_1).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = fp; c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32(((p_m_1).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32(((p_m_1).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_).0); };
        F::Vec3_Normalize(c, fp, fp);
        { let v_ = F32(c.m.r32(fp)); let a_ = ((p_m_1).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((fp).wrapping_add(4))); let a_ = ((p_m_1).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((fp).wrapping_add(8))); let a_ = ((p_m_1).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32(((p_m_1).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = fp.wrapping_add(16); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32(((p_m_1).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a_ = (fp.wrapping_add(16)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32(((p_m_1).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); let a_ = (fp.wrapping_add(16)).wrapping_add(8); c.m.w32(a_, (v_).0); };
        F::Vec3_Cross(c, fp.wrapping_add(32), fp.wrapping_add(16), fp);
        F::Vec3_Normalize(c, fp.wrapping_add(32), fp.wrapping_add(32));
        { let v_ = F32(c.m.r32(fp.wrapping_add(32))); let a_ = ((p_m_1).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((fp.wrapping_add(32)).wrapping_add(4))); let a_ = ((p_m_1).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((fp.wrapping_add(32)).wrapping_add(8))); let a_ = ((p_m_1).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        F::Vec3_Cross(c, fp.wrapping_add(16), fp, fp.wrapping_add(32));
        { let v_ = F32(c.m.r32(fp.wrapping_add(16))); let a_ = ((p_m_1).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))); let a_ = ((p_m_1).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8))); let a_ = ((p_m_1).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Mat3_Orthonormalize", &PROBE_N, &[], true); }
    r_
}

pub fn Rigid_Stub266CC0(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Rigid_Stub266CC0", &PROBE_N, &[], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Rigid_Stub266CC0", &PROBE_N, &[], true); }
    r_
}

pub fn Rigid_AddForceAt(c: &mut Ctx, mut p_body_1: u32, mut p_point_2: u32, mut p_force_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Rigid_AddForceAt", &PROBE_N, &[(p_body_1) as u32, (p_point_2) as u32, (p_force_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let r_ = 'f: {
        F::Vec4_Sub(c, fp.wrapping_add(16), p_point_2, (p_body_1).wrapping_add(48));
        F::Vec3_Cross(c, fp, fp.wrapping_add(16), p_force_3);
        { let r_ = F32(c.m.r32(p_force_3)); let a_ = (p_body_1).wrapping_add(176); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        { let r_ = F32(c.m.r32((p_force_3).wrapping_add(4))); let a_ = ((p_body_1).wrapping_add(176)).wrapping_add(4); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        { let r_ = F32(c.m.r32((p_force_3).wrapping_add(8))); let a_ = ((p_body_1).wrapping_add(176)).wrapping_add(8); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        { let r_ = F32(c.m.r32(fp)); let a_ = (p_body_1).wrapping_add(192); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        { let r_ = F32(c.m.r32((fp).wrapping_add(4))); let a_ = ((p_body_1).wrapping_add(192)).wrapping_add(4); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        { let r_ = F32(c.m.r32((fp).wrapping_add(8))); let a_ = ((p_body_1).wrapping_add(192)).wrapping_add(8); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Rigid_AddForceAt", &PROBE_N, &[], true); }
    r_
}

pub fn Rigid_Integrate(c: &mut Ctx, mut p_body_1: u32, mut p_dt_2: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Rigid_Integrate", &PROBE_N, &[(p_body_1) as u32, (p_dt_2).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_k_3: F32 = F32(0);
    let r_ = 'f: {
        F::Rigid_Stub266CC0(c);
        l_k_3 = rt::fdiv(p_dt_2, F32(c.m.r32((p_body_1).wrapping_add(8))));
        { let r_ = rt::fmul(F32(c.m.r32((p_body_1).wrapping_add(176))), l_k_3); let a_ = (p_body_1).wrapping_add(144); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        { let r_ = rt::fmul(F32(c.m.r32(((p_body_1).wrapping_add(176)).wrapping_add(4))), l_k_3); let a_ = ((p_body_1).wrapping_add(144)).wrapping_add(4); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        { let r_ = rt::fmul(F32(c.m.r32(((p_body_1).wrapping_add(176)).wrapping_add(8))), l_k_3); let a_ = ((p_body_1).wrapping_add(144)).wrapping_add(8); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        F::Mat3_MulVecCols(c, fp, (p_body_1).wrapping_add(192), (p_body_1).wrapping_add(80));
        F::Rigid_EulerAccel(c, fp.wrapping_add(16), (p_body_1).wrapping_add(208), fp, (p_body_1).wrapping_add(16));
        { let r_ = rt::fmul(rt::fmul(p_dt_2, F32(c.m.r32(fp.wrapping_add(16)))), F32(0x3f7fbe76)); let a_ = (p_body_1).wrapping_add(208); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        { let r_ = rt::fmul(rt::fmul(p_dt_2, F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4)))), F32(0x3f7fbe76)); let a_ = ((p_body_1).wrapping_add(208)).wrapping_add(4); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        { let r_ = rt::fmul(rt::fmul(p_dt_2, F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8)))), F32(0x3f7fbe76)); let a_ = ((p_body_1).wrapping_add(208)).wrapping_add(8); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        F::Mat3_MulVecRows(c, (p_body_1).wrapping_add(160), (p_body_1).wrapping_add(208), (p_body_1).wrapping_add(80));
        { let r_ = { let a0_ = F::Vec3_Dot(c, (p_body_1).wrapping_add(176), (p_body_1).wrapping_add(176)); F::Port_sqrtf(c, a0_) }; let a_ = (p_body_1).wrapping_add(536); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        { let v_ = F32(0x00000000); let a_ = (p_body_1).wrapping_add(176); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = ((p_body_1).wrapping_add(176)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = ((p_body_1).wrapping_add(176)).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = ((p_body_1).wrapping_add(176)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (p_body_1).wrapping_add(192); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = ((p_body_1).wrapping_add(192)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = ((p_body_1).wrapping_add(192)).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = ((p_body_1).wrapping_add(192)).wrapping_add(12); c.m.w32(a_, (v_).0); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Rigid_Integrate", &PROBE_N, &[], true); }
    r_
}

pub fn Rigid_GetMatrix(c: &mut Ctx, mut p_out_1: u32, mut p_body_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Rigid_GetMatrix", &PROBE_N, &[(p_out_1) as u32, (p_body_2) as u32], false); }
    let r_ = 'f: {
    F::Mtx_StoreIdentity(c, p_out_1);
    { let v_ = F32(c.m.r32((((p_body_2).wrapping_add(80)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = ((p_out_1).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((((p_body_2).wrapping_add(80)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a_ = ((p_out_1).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((((p_body_2).wrapping_add(80)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); let a_ = ((p_out_1).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((((p_body_2).wrapping_add(80)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = ((p_out_1).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((((p_body_2).wrapping_add(80)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a_ = ((p_out_1).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((((p_body_2).wrapping_add(80)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); let a_ = ((p_out_1).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((((p_body_2).wrapping_add(80)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = ((p_out_1).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((((p_body_2).wrapping_add(80)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a_ = ((p_out_1).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((((p_body_2).wrapping_add(80)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); let a_ = ((p_out_1).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((p_body_2).wrapping_add(64))); let a_ = ((p_out_1).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32(((p_body_2).wrapping_add(64)).wrapping_add(4))); let a_ = ((p_out_1).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32(((p_body_2).wrapping_add(64)).wrapping_add(8))); let a_ = ((p_out_1).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Rigid_GetMatrix", &PROBE_N, &[], true); }
    r_
}

pub fn Rigid_AddGravity(c: &mut Ctx, mut p_body_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Rigid_AddGravity", &PROBE_N, &[(p_body_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        { let v_ = F32(0x00000000); let a_ = fp; c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(F32(c.m.r32((p_body_1).wrapping_add(8))), F32(0x411ccccc)); let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_).0); };
        F::Rigid_AddForceAt(c, p_body_1, (p_body_1).wrapping_add(48), fp);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Rigid_AddGravity", &PROBE_N, &[], true); }
    r_
}

pub fn Rigid_Damp(c: &mut Ctx, mut p_body_1: u32, mut p_k_2: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Rigid_Damp", &PROBE_N, &[(p_body_1) as u32, (p_k_2).0], false); }
    let r_ = 'f: {
    { let r_ = rt::fmul(F32(c.m.r32((p_body_1).wrapping_add(144))), p_k_2); let a_ = (p_body_1).wrapping_add(144); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
    { let r_ = rt::fmul(F32(c.m.r32(((p_body_1).wrapping_add(144)).wrapping_add(4))), p_k_2); let a_ = ((p_body_1).wrapping_add(144)).wrapping_add(4); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
    { let r_ = rt::fmul(F32(c.m.r32(((p_body_1).wrapping_add(144)).wrapping_add(8))), p_k_2); let a_ = ((p_body_1).wrapping_add(144)).wrapping_add(8); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
    { let r_ = rt::fmul(F32(c.m.r32((p_body_1).wrapping_add(208))), p_k_2); let a_ = (p_body_1).wrapping_add(208); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
    { let r_ = rt::fmul(F32(c.m.r32(((p_body_1).wrapping_add(208)).wrapping_add(4))), p_k_2); let a_ = ((p_body_1).wrapping_add(208)).wrapping_add(4); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
    { let r_ = rt::fmul(F32(c.m.r32(((p_body_1).wrapping_add(208)).wrapping_add(8))), p_k_2); let a_ = ((p_body_1).wrapping_add(208)).wrapping_add(8); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Rigid_Damp", &PROBE_N, &[], true); }
    r_
}

pub fn Rigid_UpdateCenter(c: &mut Ctx, mut p_body_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Rigid_UpdateCenter", &PROBE_N, &[(p_body_1) as u32], false); }
    let r_ = 'f: {
    F::Mat3_MulVecRows(c, (p_body_1).wrapping_add(48), (p_body_1).wrapping_add(32), (p_body_1).wrapping_add(80));
    { let r_ = F32(c.m.r32((p_body_1).wrapping_add(64))); let a_ = (p_body_1).wrapping_add(48); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
    { let r_ = F32(c.m.r32(((p_body_1).wrapping_add(64)).wrapping_add(4))); let a_ = ((p_body_1).wrapping_add(48)).wrapping_add(4); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
    { let r_ = F32(c.m.r32(((p_body_1).wrapping_add(64)).wrapping_add(8))); let a_ = ((p_body_1).wrapping_add(48)).wrapping_add(8); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Rigid_UpdateCenter", &PROBE_N, &[], true); }
    r_
}

pub fn Rigid_AddContact(c: &mut Ctx, mut p_body_1: u32, mut p_point_2: u32, mut p_normal_3: u32, mut p_restitution_4: F32, mut p_friction_5: F32, mut p_depth_6: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Rigid_AddContact", &PROBE_N, &[(p_body_1) as u32, (p_point_2) as u32, (p_normal_3) as u32, (p_restitution_4).0, (p_friction_5).0, (p_depth_6).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(128) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 128);
    let mut l_damp_7: F32 = F32(0);
    let mut l_spring_8: F32 = F32(0);
    let mut l_mag_9: F32 = F32(0);
    let mut l_d_10: F32 = F32(0);
    let r_ = 'f: {
        l_damp_7 = rt::fmul(rt::fmul(rt::fsub(F32(0x3f800000), p_restitution_4), F32(0x419ccccc)), F32(c.m.r32((p_body_1).wrapping_add(8))));
        l_spring_8 = rt::fmul(F32(c.m.r32((p_body_1).wrapping_add(8))), F32(0x4312ffff));
        F::Vec4_Sub(c, fp, p_point_2, (p_body_1).wrapping_add(48));
        F::Vec3_Cross(c, fp.wrapping_add(16), (p_body_1).wrapping_add(160), fp);
        { let r_ = F32(c.m.r32((p_body_1).wrapping_add(144))); let a_ = fp.wrapping_add(16); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        { let r_ = F32(c.m.r32(((p_body_1).wrapping_add(144)).wrapping_add(4))); let a_ = (fp.wrapping_add(16)).wrapping_add(4); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        { let r_ = F32(c.m.r32(((p_body_1).wrapping_add(144)).wrapping_add(8))); let a_ = (fp.wrapping_add(16)).wrapping_add(8); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        l_mag_9 = rt::fsub(rt::fmul(l_spring_8, p_depth_6), rt::fmul(l_damp_7, F::Vec3_Dot(c, fp.wrapping_add(16), p_normal_3)));
        if (((rt::flt(l_mag_9, F32(0x00000000)) as i32)) != 0) {
            break 'f;
        }
        { let v_ = rt::fmul(l_mag_9, F32(c.m.r32(p_normal_3))); let a_ = fp.wrapping_add(32); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(l_mag_9, F32(c.m.r32((p_normal_3).wrapping_add(4)))); let a_ = (fp.wrapping_add(32)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(l_mag_9, F32(c.m.r32((p_normal_3).wrapping_add(8)))); let a_ = (fp.wrapping_add(32)).wrapping_add(8); c.m.w32(a_, (v_).0); };
        F::Vec4_Sub(c, fp.wrapping_add(64), p_point_2, (p_body_1).wrapping_add(48));
        F::Vec3_Cross(c, fp.wrapping_add(48), fp.wrapping_add(64), fp.wrapping_add(32));
        { let _ = F::Vec3_Dot(c, fp.wrapping_add(48), fp.wrapping_add(48)); };
        { let r_ = F32(c.m.r32(fp.wrapping_add(32))); let a_ = (p_body_1).wrapping_add(176); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        { let r_ = F32(c.m.r32((fp.wrapping_add(32)).wrapping_add(4))); let a_ = ((p_body_1).wrapping_add(176)).wrapping_add(4); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        { let r_ = F32(c.m.r32((fp.wrapping_add(32)).wrapping_add(8))); let a_ = ((p_body_1).wrapping_add(176)).wrapping_add(8); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        { let r_ = F32(c.m.r32(fp.wrapping_add(48))); let a_ = (p_body_1).wrapping_add(192); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        { let r_ = F32(c.m.r32((fp.wrapping_add(48)).wrapping_add(4))); let a_ = ((p_body_1).wrapping_add(192)).wrapping_add(4); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        { let r_ = F32(c.m.r32((fp.wrapping_add(48)).wrapping_add(8))); let a_ = ((p_body_1).wrapping_add(192)).wrapping_add(8); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        if (((rt::flt(F32(0x00000000), p_friction_5) as i32)) != 0) {
            l_d_10 = F::Vec3_Dot(c, fp.wrapping_add(16), p_normal_3);
            { let v_ = rt::fsub(F32(c.m.r32(fp.wrapping_add(16))), rt::fmul(l_d_10, F32(c.m.r32(p_normal_3)))); let a_ = fp.wrapping_add(96); c.m.w32(a_, (v_).0); };
            { let v_ = rt::fsub(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))), rt::fmul(l_d_10, F32(c.m.r32((p_normal_3).wrapping_add(4))))); let a_ = (fp.wrapping_add(96)).wrapping_add(4); c.m.w32(a_, (v_).0); };
            { let v_ = rt::fsub(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8))), rt::fmul(l_d_10, F32(c.m.r32((p_normal_3).wrapping_add(8))))); let a_ = (fp.wrapping_add(96)).wrapping_add(8); c.m.w32(a_, (v_).0); };
            F::Vec3_Normalize(c, fp.wrapping_add(96), fp.wrapping_add(96));
            F::Vec3_ScaleNeg(c, fp.wrapping_add(112), fp.wrapping_add(96), rt::fmul(l_mag_9, F32(0x3f666666)));
            F::Vec3_Cross(c, fp.wrapping_add(80), fp.wrapping_add(64), fp.wrapping_add(112));
            { let _ = { let a0_ = F::Vec3_Dot(c, fp.wrapping_add(80), fp.wrapping_add(80)); F::Mathf_Sqrt(c, a0_) }; };
            { let r_ = F32(c.m.r32(fp.wrapping_add(112))); let a_ = (p_body_1).wrapping_add(176); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            { let r_ = F32(c.m.r32((fp.wrapping_add(112)).wrapping_add(4))); let a_ = ((p_body_1).wrapping_add(176)).wrapping_add(4); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            { let r_ = F32(c.m.r32((fp.wrapping_add(112)).wrapping_add(8))); let a_ = ((p_body_1).wrapping_add(176)).wrapping_add(8); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            { let r_ = F32(c.m.r32(fp.wrapping_add(80))); let a_ = (p_body_1).wrapping_add(192); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            { let r_ = F32(c.m.r32((fp.wrapping_add(80)).wrapping_add(4))); let a_ = ((p_body_1).wrapping_add(192)).wrapping_add(4); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            { let r_ = F32(c.m.r32((fp.wrapping_add(80)).wrapping_add(8))); let a_ = ((p_body_1).wrapping_add(192)).wrapping_add(8); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Rigid_AddContact", &PROBE_N, &[], true); }
    r_
}

pub fn Rigid_Stub267390(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Rigid_Stub267390", &PROBE_N, &[], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Rigid_Stub267390", &PROBE_N, &[], true); }
    r_
}

pub fn Rigid_Step(c: &mut Ctx, mut p_body_1: u32, mut p_dt_2: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Rigid_Step", &PROBE_N, &[(p_body_1) as u32, (p_dt_2).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(224) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 224);
    let r_ = 'f: {
        F::Rigid_Integrate(c, p_body_1, p_dt_2);
        { let v_ = rt::fadd(F32(c.m.r32((p_body_1).wrapping_add(64))), rt::fmul(p_dt_2, F32(c.m.r32((p_body_1).wrapping_add(144))))); let a_ = fp; c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((p_body_1).wrapping_add(64)).wrapping_add(4))), rt::fmul(p_dt_2, F32(c.m.r32(((p_body_1).wrapping_add(144)).wrapping_add(4))))); let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((p_body_1).wrapping_add(64)).wrapping_add(8))), rt::fmul(p_dt_2, F32(c.m.r32(((p_body_1).wrapping_add(144)).wrapping_add(8))))); let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(p_dt_2, F32(c.m.r32((p_body_1).wrapping_add(160)))); let a_ = fp.wrapping_add(16); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(p_dt_2, F32(c.m.r32(((p_body_1).wrapping_add(160)).wrapping_add(4)))); let a_ = (fp.wrapping_add(16)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(p_dt_2, F32(c.m.r32(((p_body_1).wrapping_add(160)).wrapping_add(8)))); let a_ = (fp.wrapping_add(16)).wrapping_add(8); c.m.w32(a_, (v_).0); };
        F::Mat3_SetSkew(c, fp.wrapping_add(96), fp.wrapping_add(16));
        F::Mat3_Mul(c, fp.wrapping_add(160), (p_body_1).wrapping_add(80), fp.wrapping_add(96));
        F::Mat3_Add(c, fp.wrapping_add(32), (p_body_1).wrapping_add(80), fp.wrapping_add(160));
        F::Mat3_Orthonormalize(c, fp.wrapping_add(32));
        F::Vec4_Copy(c, (p_body_1).wrapping_add(64), fp);
        F::Mtx_Copy(c, (p_body_1).wrapping_add(80), fp.wrapping_add(32));
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Rigid_Step", &PROBE_N, &[], true); }
    r_
}

pub fn Rigid_SetTransform(c: &mut Ctx, mut p_body_1: u32, mut p_pos_2: u32, mut p_rot_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Rigid_SetTransform", &PROBE_N, &[(p_body_1) as u32, (p_pos_2) as u32, (p_rot_3) as u32], false); }
    let r_ = 'f: {
    F::Vec4_Copy(c, (p_body_1).wrapping_add(64), p_pos_2);
    F::Mtx_Copy(c, (p_body_1).wrapping_add(80), p_rot_3);
    F::Vec4_Copy(c, (p_body_1).wrapping_add(240), (p_body_1).wrapping_add(64));
    F::Mtx_Copy(c, (p_body_1).wrapping_add(256), (p_body_1).wrapping_add(80));
    { let v_ = F32(0x00000000); let a_ = (p_body_1).wrapping_add(144); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x00000000); let a_ = ((p_body_1).wrapping_add(144)).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x00000000); let a_ = ((p_body_1).wrapping_add(144)).wrapping_add(8); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x3f800000); let a_ = ((p_body_1).wrapping_add(144)).wrapping_add(12); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x00000000); let a_ = (p_body_1).wrapping_add(176); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x00000000); let a_ = ((p_body_1).wrapping_add(176)).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x00000000); let a_ = ((p_body_1).wrapping_add(176)).wrapping_add(8); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x3f800000); let a_ = ((p_body_1).wrapping_add(176)).wrapping_add(12); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x00000000); let a_ = (p_body_1).wrapping_add(192); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x00000000); let a_ = ((p_body_1).wrapping_add(192)).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x00000000); let a_ = ((p_body_1).wrapping_add(192)).wrapping_add(8); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x3f800000); let a_ = ((p_body_1).wrapping_add(192)).wrapping_add(12); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x00000000); let a_ = (p_body_1).wrapping_add(160); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x00000000); let a_ = ((p_body_1).wrapping_add(160)).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x00000000); let a_ = ((p_body_1).wrapping_add(160)).wrapping_add(8); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x3f800000); let a_ = ((p_body_1).wrapping_add(160)).wrapping_add(12); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x00000000); let a_ = (p_body_1).wrapping_add(208); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x00000000); let a_ = ((p_body_1).wrapping_add(208)).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x00000000); let a_ = ((p_body_1).wrapping_add(208)).wrapping_add(8); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x3f800000); let a_ = ((p_body_1).wrapping_add(208)).wrapping_add(12); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x00000000); let a_ = (p_body_1).wrapping_add(320); c.m.w32(a_, (v_).0); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Rigid_SetTransform", &PROBE_N, &[], true); }
    r_
}

pub fn Rigid_SetTransform2(c: &mut Ctx, mut p_body_1: u32, mut p_pos_2: u32, mut p_rot_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Rigid_SetTransform2", &PROBE_N, &[(p_body_1) as u32, (p_pos_2) as u32, (p_rot_3) as u32], false); }
    let r_ = 'f: {
    F::Rigid_SetTransform(c, p_body_1, p_pos_2, p_rot_3);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Rigid_SetTransform2", &PROBE_N, &[], true); }
    r_
}

pub fn Rigid_BeginFrame(c: &mut Ctx, mut p_body_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Rigid_BeginFrame", &PROBE_N, &[(p_body_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = (c.m.r32(p_body_1) as i32); let a_ = (p_body_1).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
    F::Vec4_Copy(c, (p_body_1).wrapping_add(240), (p_body_1).wrapping_add(64));
    F::Mtx_Copy(c, (p_body_1).wrapping_add(256), (p_body_1).wrapping_add(80));
    { let r_ = 255i32; let a_ = p_body_1; let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
    { let v_ = ((0i32) as i64); let a_ = (p_body_1).wrapping_add(528); c.m.w64(a_, (v_) as u64); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Rigid_BeginFrame", &PROBE_N, &[], true); }
    r_
}

pub fn Rigid_SetVelocity(c: &mut Ctx, mut p_body_1: u32, mut p_vel_2: u32, mut p_angVel_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Rigid_SetVelocity", &PROBE_N, &[(p_body_1) as u32, (p_vel_2) as u32, (p_angVel_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let r_ = 'f: {
        F::Vec4_Copy(c, fp.wrapping_add(32), p_angVel_3);
        F::Mat3_MulVecRows(c, fp, fp.wrapping_add(32), (p_body_1).wrapping_add(80));
        F::Mat3_MulVecCols(c, fp.wrapping_add(16), fp, (p_body_1).wrapping_add(80));
        { let r_ = F32(c.m.r32(fp.wrapping_add(16))); let a_ = (p_body_1).wrapping_add(208); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        { let r_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))); let a_ = ((p_body_1).wrapping_add(208)).wrapping_add(4); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        { let r_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8))); let a_ = ((p_body_1).wrapping_add(208)).wrapping_add(8); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        { let v_ = F32(0x00000000); let a_ = fp.wrapping_add(32); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (fp.wrapping_add(32)).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((p_angVel_3).wrapping_add(4))); let a_ = (fp.wrapping_add(32)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        F::Mat3_MulVecCols(c, fp.wrapping_add(16), fp.wrapping_add(32), (p_body_1).wrapping_add(80));
        { let r_ = F32(c.m.r32(fp.wrapping_add(16))); let a_ = (p_body_1).wrapping_add(208); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        { let r_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))); let a_ = ((p_body_1).wrapping_add(208)).wrapping_add(4); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        { let r_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8))); let a_ = ((p_body_1).wrapping_add(208)).wrapping_add(8); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        { let v_ = F32(c.m.r32(p_vel_2)); let a_ = (p_body_1).wrapping_add(144); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((p_vel_2).wrapping_add(4))); let a_ = ((p_body_1).wrapping_add(144)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((p_vel_2).wrapping_add(8))); let a_ = ((p_body_1).wrapping_add(144)).wrapping_add(8); c.m.w32(a_, (v_).0); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Rigid_SetVelocity", &PROBE_N, &[], true); }
    r_
}

pub fn Rigid_CheckRest(c: &mut Ctx, mut p_body_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Rigid_CheckRest", &PROBE_N, &[(p_body_1) as u32], false); }
    let mut l_sum_2: F32 = F32(0);
    let mut l_ret_3: i32 = 0;
    let r_ = 'f: {
    { let v_ = F::Vec3_Dist(c, (p_body_1).wrapping_add(64), (p_body_1).wrapping_add(240)); let a_ = (p_body_1).wrapping_add(320); c.m.w32(a_, (v_).0); };
    l_sum_2 = (if (((rt::flt(F32(c.m.r32((p_body_1).wrapping_add(160))), F32(0x00000000)) as i32)) != 0) { rt::fneg(F32(c.m.r32((p_body_1).wrapping_add(160)))) } else { F32(c.m.r32((p_body_1).wrapping_add(160))) });
    { let r_ = (if (((rt::flt(F32(c.m.r32(((p_body_1).wrapping_add(160)).wrapping_add(4))), F32(0x00000000)) as i32)) != 0) { rt::fneg(F32(c.m.r32(((p_body_1).wrapping_add(160)).wrapping_add(4)))) } else { F32(c.m.r32(((p_body_1).wrapping_add(160)).wrapping_add(4))) }); let n_ = rt::fadd(l_sum_2, r_); l_sum_2 = n_; };
    { let r_ = (if (((rt::flt(F32(c.m.r32(((p_body_1).wrapping_add(160)).wrapping_add(8))), F32(0x00000000)) as i32)) != 0) { rt::fneg(F32(c.m.r32(((p_body_1).wrapping_add(160)).wrapping_add(8)))) } else { F32(c.m.r32(((p_body_1).wrapping_add(160)).wrapping_add(8))) }); let n_ = rt::fadd(l_sum_2, r_); l_sum_2 = n_; };
    if (((rt::flt({ let a0_ = F32(c.m.r32((p_body_1).wrapping_add(160))); rt::ffabs(a0_) }, F32(0x3dcccccc)) as i32)) != 0) {
        { let r_ = F32(0x3e4ccccc); let a_ = (p_body_1).wrapping_add(160); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
    }
    if (((rt::flt({ let a0_ = F32(c.m.r32(((p_body_1).wrapping_add(160)).wrapping_add(4))); rt::ffabs(a0_) }, F32(0x3dcccccc)) as i32)) != 0) {
        { let r_ = F32(0x3e4ccccc); let a_ = ((p_body_1).wrapping_add(160)).wrapping_add(4); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
    }
    if (((rt::flt({ let a0_ = F32(c.m.r32(((p_body_1).wrapping_add(160)).wrapping_add(8))); rt::ffabs(a0_) }, F32(0x3dcccccc)) as i32)) != 0) {
        { let r_ = F32(0x3e4ccccc); let a_ = ((p_body_1).wrapping_add(160)).wrapping_add(8); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
    }
    if (((rt::flt({ let a0_ = F32(c.m.r32((p_body_1).wrapping_add(208))); rt::ffabs(a0_) }, F32(0x3dcccccc)) as i32)) != 0) {
        { let r_ = F32(0x3e4ccccc); let a_ = (p_body_1).wrapping_add(208); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
    }
    if (((rt::flt({ let a0_ = F32(c.m.r32(((p_body_1).wrapping_add(208)).wrapping_add(4))); rt::ffabs(a0_) }, F32(0x3dcccccc)) as i32)) != 0) {
        { let r_ = F32(0x3e4ccccc); let a_ = ((p_body_1).wrapping_add(208)).wrapping_add(4); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
    }
    if (((rt::flt({ let a0_ = F32(c.m.r32(((p_body_1).wrapping_add(208)).wrapping_add(8))); rt::ffabs(a0_) }, F32(0x3dcccccc)) as i32)) != 0) {
        { let r_ = F32(0x3e4ccccc); let a_ = ((p_body_1).wrapping_add(208)).wrapping_add(8); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
    }
    if (((rt::flt({ let a0_ = F32(c.m.r32((p_body_1).wrapping_add(160))); rt::ffabs(a0_) }, F32(0x3e199999)) as i32)) != 0) {
        { let r_ = F32(0x3f333333); let a_ = (p_body_1).wrapping_add(160); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
    }
    if (((rt::flt({ let a0_ = F32(c.m.r32(((p_body_1).wrapping_add(160)).wrapping_add(4))); rt::ffabs(a0_) }, F32(0x3e199999)) as i32)) != 0) {
        { let r_ = F32(0x3f333333); let a_ = ((p_body_1).wrapping_add(160)).wrapping_add(4); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
    }
    if (((rt::flt({ let a0_ = F32(c.m.r32(((p_body_1).wrapping_add(160)).wrapping_add(8))); rt::ffabs(a0_) }, F32(0x3e199999)) as i32)) != 0) {
        { let r_ = F32(0x3f333333); let a_ = ((p_body_1).wrapping_add(160)).wrapping_add(8); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
    }
    if (((rt::flt({ let a0_ = F32(c.m.r32((p_body_1).wrapping_add(208))); rt::ffabs(a0_) }, F32(0x3e800000)) as i32)) != 0) {
        { let r_ = F32(0x3f333333); let a_ = (p_body_1).wrapping_add(208); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
    }
    if (((rt::flt({ let a0_ = F32(c.m.r32(((p_body_1).wrapping_add(208)).wrapping_add(4))); rt::ffabs(a0_) }, F32(0x3e800000)) as i32)) != 0) {
        { let r_ = F32(0x3f333333); let a_ = ((p_body_1).wrapping_add(208)).wrapping_add(4); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
    }
    if (((rt::flt({ let a0_ = F32(c.m.r32(((p_body_1).wrapping_add(208)).wrapping_add(8))); rt::ffabs(a0_) }, F32(0x3e800000)) as i32)) != 0) {
        { let r_ = F32(0x3f333333); let a_ = ((p_body_1).wrapping_add(208)).wrapping_add(8); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
    }
    l_ret_3 = 0i32;
    if (((((((rt::fle(F32(c.m.r32((p_body_1).wrapping_add(320))), F32(0x3c75c28f)) as i32)) != 0) && (((rt::flt(l_sum_2, F32(0x3e4ccccc)) as i32)) != 0)) as i32)) != 0) {
        F::Vec4_Copy(c, (p_body_1).wrapping_add(64), (p_body_1).wrapping_add(240));
        F::Mtx_Copy(c, (p_body_1).wrapping_add(80), (p_body_1).wrapping_add(256));
        l_ret_3 = 1i32;
    }
    break 'f l_ret_3;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Rigid_CheckRest", &PROBE_N, &[], true); }
    r_
}

pub fn Rigid_SyncSpheres(c: &mut Ctx, mut p_body_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Rigid_SyncSpheres", &PROBE_N, &[(p_body_1) as u32], false); }
    let r_ = 'f: {
    F::Vec4_Copy(c, (p_body_1).wrapping_add(464), (p_body_1).wrapping_add(64));
    F::Vec4_Copy(c, (p_body_1).wrapping_add(496), (p_body_1).wrapping_add(64));
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Rigid_SyncSpheres", &PROBE_N, &[], true); }
    r_
}

pub fn Rigid_Init(c: &mut Ctx, mut p_body_1: u32, mut p_user_2: i32, mut p_radius_3: F32, mut p_density_4: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Rigid_Init", &PROBE_N, &[(p_body_1) as u32, (p_user_2) as u32, (p_radius_3).0, (p_density_4).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_inertia_5: F32 = F32(0);
    let r_ = 'f: {
        rt::memset(c, fp, 0, 16);
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(0), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(4), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(8), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(12), (v_).0); }
        { let o_ = p_body_1; let n_ = (o_).wrapping_add(544); p_body_1 = n_; };
        { let o_ = p_body_1; let n_ = (o_).wrapping_sub(544); p_body_1 = n_; };
        { let _ = F::memset(c, p_body_1, 0i32, 544u32); };
        { let v_ = 1i32; let a_ = p_body_1; c.m.w32(a_, (v_) as u32); };
        F::ColSphere_Set(c, (p_body_1).wrapping_add(496), fp, p_radius_3);
        F::Vec4_Copy(c, (p_body_1).wrapping_add(32), fp);
        { let v_ = rt::fmul(rt::fmul(rt::fmul(rt::fmul(p_radius_3, F32(0x4016cbe3)), p_radius_3), p_radius_3), p_density_4); let a_ = (p_body_1).wrapping_add(8); c.m.w32(a_, (v_).0); };
        l_inertia_5 = rt::fmul(rt::fdiv(rt::fmul(rt::fmul(rt::fmul(F32(c.m.r32((p_body_1).wrapping_add(8))), p_radius_3), p_radius_3), F32(0x40000000)), F32(0x40a00000)), F32(0x41a00000));
        { let v_ = l_inertia_5; let a_ = (p_body_1).wrapping_add(16); c.m.w32(a_, (v_).0); };
        { let v_ = l_inertia_5; let a_ = ((p_body_1).wrapping_add(16)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = l_inertia_5; let a_ = ((p_body_1).wrapping_add(16)).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = ((p_body_1).wrapping_add(16)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let s_ = (p_body_1).wrapping_add(496); let d_ = (p_body_1).wrapping_add(464); rt::memcpy(c, d_, s_, 32); };
        { let v_ = p_user_2; let a_ = (p_body_1).wrapping_add(228); c.m.w32(a_, (v_) as u32); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Rigid_Init", &PROBE_N, &[], true); }
    r_
}
