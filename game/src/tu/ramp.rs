#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn Ramp_Stop(c: &mut Ctx, mut p_ramp_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Ramp_Stop", &PROBE_N, &[(p_ramp_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = (1i32).wrapping_neg(); let a_ = (p_ramp_1).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Ramp_Stop", &PROBE_N, &[], true); }
    r_
}

pub fn Ramp_Start(c: &mut Ctx, mut p_ramp_1: u32, mut p_seconds_2: F32, mut p_from_3: F32, mut p_to_4: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Ramp_Start", &PROBE_N, &[(p_ramp_1) as u32, (p_seconds_2).0, (p_from_3).0, (p_to_4).0], false); }
    let r_ = 'f: {
    { let v_ = 0i32; let a_ = (p_ramp_1).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
    { let v_ = rt::fmul(p_seconds_2, F32(0x41f00000)); let a_ = (p_ramp_1).wrapping_add(8); c.m.w32(a_, (v_).0); };
    { let v_ = p_from_3; let a_ = (p_ramp_1).wrapping_add(16); c.m.w32(a_, (v_).0); };
    { let v_ = p_to_4; let a_ = (p_ramp_1).wrapping_add(20); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fdiv(rt::fsub(p_to_4, p_from_3), F32(c.m.r32((p_ramp_1).wrapping_add(8)))); let a_ = (p_ramp_1).wrapping_add(12); c.m.w32(a_, (v_).0); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Ramp_Start", &PROBE_N, &[], true); }
    r_
}

pub fn Ramp_Step(c: &mut Ctx, mut p_ramp_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Ramp_Step", &PROBE_N, &[(p_ramp_1) as u32], false); }
    let r_ = 'f: {
    if ((((((c.m.r32((p_ramp_1).wrapping_add(4)) as i32)) == ((1i32).wrapping_neg())) as i32)) != 0) {
        break 'f 1i32;
    }
    { let r_ = F32(0x3f800000); let a_ = (p_ramp_1).wrapping_add(8); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
    if (((rt::fle(F32(c.m.r32((p_ramp_1).wrapping_add(8))), F32(0x38d1b717)) as i32)) != 0) {
        { let v_ = F32(c.m.r32((p_ramp_1).wrapping_add(20))); let a_ = (p_ramp_1).wrapping_add(16); c.m.w32(a_, (v_).0); };
        break 'f 1i32;
    }
    { let r_ = F32(c.m.r32((p_ramp_1).wrapping_add(12))); let a_ = (p_ramp_1).wrapping_add(16); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
    if (((rt::flt(F32(0x00000000), F32(c.m.r32((p_ramp_1).wrapping_add(12)))) as i32)) != 0) {
        if (((rt::flt(F32(c.m.r32((p_ramp_1).wrapping_add(20))), F32(c.m.r32((p_ramp_1).wrapping_add(16)))) as i32)) != 0) {
            { let v_ = F32(c.m.r32((p_ramp_1).wrapping_add(20))); let a_ = (p_ramp_1).wrapping_add(16); c.m.w32(a_, (v_).0); };
        }
    } else {
        if (((rt::flt(F32(c.m.r32((p_ramp_1).wrapping_add(16))), F32(c.m.r32((p_ramp_1).wrapping_add(20)))) as i32)) != 0) {
            { let v_ = F32(c.m.r32((p_ramp_1).wrapping_add(20))); let a_ = (p_ramp_1).wrapping_add(16); c.m.w32(a_, (v_).0); };
        }
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Ramp_Step", &PROBE_N, &[], true); }
    r_
}
