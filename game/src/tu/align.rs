#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn Mem_Align4Rem(c: &mut Ctx, mut p_value_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Mem_Align4Rem", &PROBE_N, &[(p_value_1) as u32], false); }
    let r_ = 'f: {
    break 'f ((((p_value_1) & (((3i32) as u32)))) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Mem_Align4Rem", &PROBE_N, &[], true); }
    r_
}

pub fn Mem_Align8Rem(c: &mut Ctx, mut p_value_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Mem_Align8Rem", &PROBE_N, &[(p_value_1) as u32], false); }
    let r_ = 'f: {
    break 'f ((((p_value_1) & (((7i32) as u32)))) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Mem_Align8Rem", &PROBE_N, &[], true); }
    r_
}

pub fn Mem_Align16Rem(c: &mut Ctx, mut p_value_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Mem_Align16Rem", &PROBE_N, &[(p_value_1) as u32], false); }
    let r_ = 'f: {
    break 'f ((((p_value_1) & (((15i32) as u32)))) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Mem_Align16Rem", &PROBE_N, &[], true); }
    r_
}

pub fn Mem_Align32Rem(c: &mut Ctx, mut p_value_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Mem_Align32Rem", &PROBE_N, &[(p_value_1) as u32], false); }
    let r_ = 'f: {
    break 'f ((((p_value_1) & (((31i32) as u32)))) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Mem_Align32Rem", &PROBE_N, &[], true); }
    r_
}

pub fn Mem_Align64Rem(c: &mut Ctx, mut p_value_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Mem_Align64Rem", &PROBE_N, &[(p_value_1) as u32], false); }
    let r_ = 'f: {
    break 'f ((((p_value_1) & (((63i32) as u32)))) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Mem_Align64Rem", &PROBE_N, &[], true); }
    r_
}

pub fn Mem_Align128Rem(c: &mut Ctx, mut p_value_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Mem_Align128Rem", &PROBE_N, &[(p_value_1) as u32], false); }
    let r_ = 'f: {
    break 'f ((((p_value_1) & (((127i32) as u32)))) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Mem_Align128Rem", &PROBE_N, &[], true); }
    r_
}

pub fn Mem_Align4Pad(c: &mut Ctx, mut p_value_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Mem_Align4Pad", &PROBE_N, &[(p_value_1) as u32], false); }
    let r_ = 'f: {
    break 'f (((((((p_value_1) & (((3i32) as u32)))).wrapping_neg()) & (((3i32) as u32)))) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Mem_Align4Pad", &PROBE_N, &[], true); }
    r_
}

pub fn Mem_Align8Pad(c: &mut Ctx, mut p_value_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Mem_Align8Pad", &PROBE_N, &[(p_value_1) as u32], false); }
    let r_ = 'f: {
    break 'f (((((((p_value_1) & (((7i32) as u32)))).wrapping_neg()) & (((7i32) as u32)))) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Mem_Align8Pad", &PROBE_N, &[], true); }
    r_
}

pub fn Mem_Align16Pad(c: &mut Ctx, mut p_value_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Mem_Align16Pad", &PROBE_N, &[(p_value_1) as u32], false); }
    let r_ = 'f: {
    break 'f (((((((p_value_1) & (((15i32) as u32)))).wrapping_neg()) & (((15i32) as u32)))) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Mem_Align16Pad", &PROBE_N, &[], true); }
    r_
}

pub fn Mem_Align32Pad(c: &mut Ctx, mut p_value_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Mem_Align32Pad", &PROBE_N, &[(p_value_1) as u32], false); }
    let r_ = 'f: {
    break 'f (((((((p_value_1) & (((31i32) as u32)))).wrapping_neg()) & (((31i32) as u32)))) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Mem_Align32Pad", &PROBE_N, &[], true); }
    r_
}

pub fn Mem_Align64Pad(c: &mut Ctx, mut p_value_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Mem_Align64Pad", &PROBE_N, &[(p_value_1) as u32], false); }
    let r_ = 'f: {
    break 'f (((((((p_value_1) & (((63i32) as u32)))).wrapping_neg()) & (((63i32) as u32)))) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Mem_Align64Pad", &PROBE_N, &[], true); }
    r_
}

pub fn Mem_Align128Pad(c: &mut Ctx, mut p_value_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Mem_Align128Pad", &PROBE_N, &[(p_value_1) as u32], false); }
    let r_ = 'f: {
    break 'f (((((((p_value_1) & (((127i32) as u32)))).wrapping_neg()) & (((127i32) as u32)))) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Mem_Align128Pad", &PROBE_N, &[], true); }
    r_
}

pub fn Mem_Align4Up(c: &mut Ctx, mut p_value_1: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Mem_Align4Up", &PROBE_N, &[(p_value_1) as u32], false); }
    let r_ = 'f: {
    break 'f (p_value_1).wrapping_add(((F::Mem_Align4Pad(c, p_value_1)) as u32));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Mem_Align4Up", &PROBE_N, &[], true); }
    r_
}

pub fn Mem_Align8Up(c: &mut Ctx, mut p_value_1: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Mem_Align8Up", &PROBE_N, &[(p_value_1) as u32], false); }
    let r_ = 'f: {
    break 'f (p_value_1).wrapping_add(((F::Mem_Align8Pad(c, p_value_1)) as u32));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Mem_Align8Up", &PROBE_N, &[], true); }
    r_
}

pub fn Mem_Align16Up(c: &mut Ctx, mut p_value_1: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Mem_Align16Up", &PROBE_N, &[(p_value_1) as u32], false); }
    let r_ = 'f: {
    break 'f (p_value_1).wrapping_add(((F::Mem_Align16Pad(c, p_value_1)) as u32));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Mem_Align16Up", &PROBE_N, &[], true); }
    r_
}

pub fn Mem_Align32Up(c: &mut Ctx, mut p_value_1: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Mem_Align32Up", &PROBE_N, &[(p_value_1) as u32], false); }
    let r_ = 'f: {
    break 'f (p_value_1).wrapping_add(((F::Mem_Align32Pad(c, p_value_1)) as u32));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Mem_Align32Up", &PROBE_N, &[], true); }
    r_
}

pub fn Mem_Align64Up(c: &mut Ctx, mut p_value_1: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Mem_Align64Up", &PROBE_N, &[(p_value_1) as u32], false); }
    let r_ = 'f: {
    break 'f (p_value_1).wrapping_add(((F::Mem_Align64Pad(c, p_value_1)) as u32));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Mem_Align64Up", &PROBE_N, &[], true); }
    r_
}

pub fn Mem_Align128Up(c: &mut Ctx, mut p_value_1: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Mem_Align128Up", &PROBE_N, &[(p_value_1) as u32], false); }
    let r_ = 'f: {
    break 'f (p_value_1).wrapping_add(((F::Mem_Align128Pad(c, p_value_1)) as u32));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Mem_Align128Up", &PROBE_N, &[], true); }
    r_
}
