#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn Mem_LoadWord(c: &mut Ctx, mut p_p_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Mem_LoadWord", &PROBE_N, &[(p_p_1) as u32], false); }
    let r_ = 'f: {
    break 'f (c.m.r32(p_p_1) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Mem_LoadWord", &PROBE_N, &[], true); }
    r_
}

pub fn Mem_LoadFloat(c: &mut Ctx, mut p_p_1: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Mem_LoadFloat", &PROBE_N, &[(p_p_1) as u32], false); }
    let r_ = 'f: {
    break 'f F32(c.m.r32(p_p_1));
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "Mem_LoadFloat", &PROBE_N, &[], true); }
    r_
}

pub fn Mem_LoadHalf(c: &mut Ctx, mut p_p_1: u32) -> u16 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Mem_LoadHalf", &PROBE_N, &[(p_p_1) as u32], false); }
    let r_ = 'f: {
    break 'f c.m.r16(p_p_1);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Mem_LoadHalf", &PROBE_N, &[], true); }
    r_
}

pub fn Mem_ReadS32(c: &mut Ctx, mut p_p_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Mem_ReadS32", &PROBE_N, &[(p_p_1) as u32], false); }
    let r_ = 'f: {
    break 'f F::Mem_LoadWord(c, p_p_1);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Mem_ReadS32", &PROBE_N, &[], true); }
    r_
}

pub fn Mem_ReadU32(c: &mut Ctx, mut p_p_1: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Mem_ReadU32", &PROBE_N, &[(p_p_1) as u32], false); }
    let r_ = 'f: {
    break 'f ((F::Mem_LoadWord(c, p_p_1)) as u32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Mem_ReadU32", &PROBE_N, &[], true); }
    r_
}

pub fn Mem_ReadF32(c: &mut Ctx, mut p_p_1: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Mem_ReadF32", &PROBE_N, &[(p_p_1) as u32], false); }
    let r_ = 'f: {
    break 'f F::Mem_LoadFloat(c, p_p_1);
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "Mem_ReadF32", &PROBE_N, &[], true); }
    r_
}

pub fn Mem_ReadU16(c: &mut Ctx, mut p_p_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Mem_ReadU16", &PROBE_N, &[(p_p_1) as u32], false); }
    let r_ = 'f: {
    break 'f ((F::Mem_LoadHalf(c, p_p_1)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Mem_ReadU16", &PROBE_N, &[], true); }
    r_
}

pub fn Mem_ReadS16(c: &mut Ctx, mut p_p_1: u32) -> i16 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Mem_ReadS16", &PROBE_N, &[(p_p_1) as u32], false); }
    let r_ = 'f: {
    break 'f ((F::Mem_LoadHalf(c, p_p_1)) as i16);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Mem_ReadS16", &PROBE_N, &[], true); }
    r_
}
