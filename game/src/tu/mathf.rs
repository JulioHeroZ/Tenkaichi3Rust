#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn Mathf_Sin(c: &mut Ctx, mut p_angle_1: F32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Mathf_Sin", &PROBE_N, &[(p_angle_1).0], false); }
    let r_ = 'f: {
    break 'f { let a0_ = F::Mathf_WrapAngle(c, p_angle_1, F32(0x40490fda)); F::Port_sinf(c, a0_) };
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "Mathf_Sin", &PROBE_N, &[], true); }
    r_
}

pub fn Mathf_Cos(c: &mut Ctx, mut p_angle_1: F32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Mathf_Cos", &PROBE_N, &[(p_angle_1).0], false); }
    let r_ = 'f: {
    break 'f F::Mathf_Sin(c, rt::fadd(p_angle_1, F32(0x3fc90fda)));
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "Mathf_Cos", &PROBE_N, &[], true); }
    r_
}

pub fn Mathf_CosFast(c: &mut Ctx, mut p_angle_1: F32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Mathf_CosFast", &PROBE_N, &[(p_angle_1).0], false); }
    let r_ = 'f: {
    break 'f F::Mathf_SinFast(c, rt::fadd(p_angle_1, F32(0x3fc90fda)));
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "Mathf_CosFast", &PROBE_N, &[], true); }
    r_
}

pub fn Mathf_Tan(c: &mut Ctx, mut p_angle_1: F32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Mathf_Tan", &PROBE_N, &[(p_angle_1).0], false); }
    let r_ = 'f: {
    'l1: while (((rt::flt(p_angle_1, F32(0xc0490fda)) as i32)) != 0) {
        { let r_ = F32(0x40c90fda); let n_ = rt::fadd(p_angle_1, r_); p_angle_1 = n_; };
    }
    'l2: while (((rt::fgt(p_angle_1, F32(0x40490fda)) as i32)) != 0) {
        { let r_ = F32(0x40c90fda); let n_ = rt::fsub(p_angle_1, r_); p_angle_1 = n_; };
    }
    break 'f F::Port_tanf(c, p_angle_1);
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "Mathf_Tan", &PROBE_N, &[], true); }
    r_
}

pub fn Mathf_SinCos(c: &mut Ctx, mut p_out_1: u32, mut p_angle_2: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Mathf_SinCos", &PROBE_N, &[(p_out_1) as u32, (p_angle_2).0], false); }
    let r_ = 'f: {
    { let v_ = F::Mathf_Sin(c, p_angle_2); let a_ = (p_out_1).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = F::Mathf_Cos(c, p_angle_2); let a_ = (p_out_1).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Mathf_SinCos", &PROBE_N, &[], true); }
    r_
}

pub fn Mathf_Asin(c: &mut Ctx, mut p_x_1: F32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Mathf_Asin", &PROBE_N, &[(p_x_1).0], false); }
    let r_ = 'f: {
    break 'f F::Port_asinf(c, (if (((rt::flt(p_x_1, F32(0xbf800000)) as i32)) != 0) { F32(0xbf800000) } else { (if (((rt::fgt(p_x_1, F32(0x3f800000)) as i32)) != 0) { F32(0x3f800000) } else { p_x_1 }) }));
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "Mathf_Asin", &PROBE_N, &[], true); }
    r_
}

pub fn Mathf_Acos(c: &mut Ctx, mut p_x_1: F32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Mathf_Acos", &PROBE_N, &[(p_x_1).0], false); }
    let r_ = 'f: {
    break 'f F::Port_acosf(c, (if (((rt::flt(p_x_1, F32(0xbf800000)) as i32)) != 0) { F32(0xbf800000) } else { (if (((rt::fgt(p_x_1, F32(0x3f800000)) as i32)) != 0) { F32(0x3f800000) } else { p_x_1 }) }));
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "Mathf_Acos", &PROBE_N, &[], true); }
    r_
}

pub fn Mathf_Atan(c: &mut Ctx, mut p_x_1: F32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Mathf_Atan", &PROBE_N, &[(p_x_1).0], false); }
    let r_ = 'f: {
    break 'f F::Port_atanf(c, p_x_1);
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "Mathf_Atan", &PROBE_N, &[], true); }
    r_
}
