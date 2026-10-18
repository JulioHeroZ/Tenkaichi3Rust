#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn Rand_FloatRange(c: &mut Ctx, mut p_a_1: F32, mut p_b_2: F32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Rand_FloatRange", &PROBE_N, &[(p_a_1).0, (p_b_2).0], false); }
    let mut l_tmp_3: F32 = F32(0);
    let r_ = 'f: {
    if (((rt::feq(p_b_2, p_a_1) as i32)) != 0) {
        break 'f p_b_2;
    }
    if (((rt::flt(p_b_2, p_a_1) as i32)) != 0) {
        l_tmp_3 = p_a_1;
        p_a_1 = p_b_2;
        p_b_2 = l_tmp_3;
    }
    break 'f rt::fadd(p_a_1, rt::fmul(F::Rand_Float01(c), rt::fsub(p_b_2, p_a_1)));
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "Rand_FloatRange", &PROBE_N, &[], true); }
    r_
}

pub fn Rand_Libc(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Rand_Libc", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f F::Port_Rand(c);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Rand_Libc", &PROBE_N, &[], true); }
    r_
}

pub fn Rand_IntRange(c: &mut Ctx, mut p_a_1: i32, mut p_b_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Rand_IntRange", &PROBE_N, &[(p_a_1) as u32, (p_b_2) as u32], false); }
    let mut l_tmp_3: i32 = 0;
    let r_ = 'f: {
    if (((((p_b_2) == (p_a_1)) as i32)) != 0) {
        break 'f p_a_1;
    }
    if (((((p_b_2) < (p_a_1)) as i32)) != 0) {
        l_tmp_3 = p_a_1;
        p_a_1 = p_b_2;
        p_b_2 = l_tmp_3;
    }
    break 'f (p_a_1).wrapping_add(rt::rem_i32(F::Rand_Libc(c), ((p_b_2).wrapping_sub(p_a_1)).wrapping_add(1i32)));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Rand_IntRange", &PROBE_N, &[], true); }
    r_
}
