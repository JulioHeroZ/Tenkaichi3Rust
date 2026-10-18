#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn Port_acosf(c: &mut Ctx, mut p_x_1: F32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Port_acosf", &PROBE_N, &[(p_x_1).0], false); }
    let r_ = 'f: {
    break 'f F::__ieee754_acosf(c, p_x_1);
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "Port_acosf", &PROBE_N, &[], true); }
    r_
}

pub fn Port_asinf(c: &mut Ctx, mut p_x_1: F32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Port_asinf", &PROBE_N, &[(p_x_1).0], false); }
    let r_ = 'f: {
    break 'f F::__ieee754_asinf(c, p_x_1);
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "Port_asinf", &PROBE_N, &[], true); }
    r_
}

pub fn Port_atan2f(c: &mut Ctx, mut p_y_1: F32, mut p_x_2: F32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Port_atan2f", &PROBE_N, &[(p_y_1).0, (p_x_2).0], false); }
    let r_ = 'f: {
    break 'f F::__ieee754_atan2f(c, p_y_1, p_x_2);
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "Port_atan2f", &PROBE_N, &[], true); }
    r_
}

pub fn Port_powf(c: &mut Ctx, mut p_x_1: F32, mut p_y_2: F32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Port_powf", &PROBE_N, &[(p_x_1).0, (p_y_2).0], false); }
    let r_ = 'f: {
    break 'f F::__ieee754_powf(c, p_x_1, p_y_2);
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "Port_powf", &PROBE_N, &[], true); }
    r_
}

pub fn Port_sqrtf(c: &mut Ctx, mut p_x_1: F32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Port_sqrtf", &PROBE_N, &[(p_x_1).0], false); }
    let r_ = 'f: {
    break 'f F::__ieee754_sqrtf(c, p_x_1);
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "Port_sqrtf", &PROBE_N, &[], true); }
    r_
}
