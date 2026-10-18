#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn Sys_InitIopHeap(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Sys_InitIopHeap", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let _ = F::sceSifInitIopHeap(c); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Sys_InitIopHeap", &PROBE_N, &[], true); }
    r_
}

pub fn IopHeap_Alloc(c: &mut Ctx, mut p_size_1: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "IopHeap_Alloc", &PROBE_N, &[(p_size_1) as u32], false); }
    let r_ = 'f: {
    break 'f F::sceSifAllocIopHeap(c, p_size_1);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "IopHeap_Alloc", &PROBE_N, &[], true); }
    r_
}

pub fn IopHeap_Free(c: &mut Ctx, mut p_addr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "IopHeap_Free", &PROBE_N, &[(p_addr_1) as u32], false); }
    let r_ = 'f: {
    break 'f F::sceSifFreeIopHeap(c, p_addr_1);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "IopHeap_Free", &PROBE_N, &[], true); }
    r_
}

pub fn IopHeap_GetTotalFree(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "IopHeap_GetTotalFree", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f F::sceSifQueryTotalFreeMemSize(c);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "IopHeap_GetTotalFree", &PROBE_N, &[], true); }
    r_
}

pub fn IopHeap_GetMaxFree(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "IopHeap_GetMaxFree", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f F::sceSifQueryMaxFreeMemSize(c);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "IopHeap_GetMaxFree", &PROBE_N, &[], true); }
    r_
}

pub fn IopHeap_PrintFree(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "IopHeap_PrintFree", &PROBE_N, &[], false); }
    let mut l_total_1: i32 = 0;
    let mut l_max_2: i32 = 0;
    let r_ = 'f: {
    l_total_1 = F::IopHeap_GetTotalFree(c);
    l_max_2 = F::IopHeap_GetMaxFree(c);
    if (((((l_total_1) != (0i32)) as i32)) != 0) {
        { let r_ = 1024i32; let n_ = rt::div_i32(l_total_1, r_); l_total_1 = n_; };
        { let r_ = 1024i32; let n_ = rt::div_i32(l_max_2, r_); l_max_2 = n_; };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "IopHeap_PrintFree", &PROBE_N, &[], true); }
    r_
}
