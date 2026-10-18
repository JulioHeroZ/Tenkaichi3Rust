#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn EftLayer3_Init(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftLayer3_Init", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
    F::BtlPool_SetCurrent(c, 1i32);
    { let v_ = { let a0_ = F::BtlPool_GetCurrent(c); F::BtlPool_Alloc(c, a0_, ((4u32) as i32)) }; let a_ = sym::gEftLayer3; c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = c.m.r32(sym::gEftLayer3); c.m.w32(a_, v_); };
    { let v_ = F::BtlTask_CreateChildList(c, p_task_1, ((((31i32) as i64 as u64)) as u32), ((((0i32) as i64 as u64)) as u32)); let a_ = c.m.r32(sym::gEftLayer3); c.m.w32(a_, v_); };
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < (31i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if (((((c.m.r32(((sym::gEftLayer3Classes).wrapping_add((((l_i_2) as u32)).wrapping_mul(8))).wrapping_add(4)) as i32)) & (1i32))) != 0) {
                    { let _ = { let a0_ = { let a_ = c.m.r32(sym::gEftLayer3); c.m.r32(a_) }; let a1_ = c.m.r32((sym::gEftLayer3Classes).wrapping_add((((l_i_2) as u32)).wrapping_mul(8))); F::BtlTaskList_AddTail(c, a0_, a1_, ((0i32) as u32)) }; };
                }
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftLayer3_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftLayer3_Term(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftLayer3_Term", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    F::BtlPool_SetCurrent(c, 1i32);
    { let a0_ = F::BtlPool_GetCurrent(c); let a1_ = c.m.r32(sym::gEftLayer3); F::BtlPool_Free(c, a0_, a1_) };
    { let v_ = ((0i32) as u32); let a_ = sym::gEftLayer3; c.m.w32(a_, v_); };
    F::BtlPool_Reset(c, 1i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftLayer3_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftLayer3_Update(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftLayer3_Update", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftLayer3_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftLayer3_Reset(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftLayer3_Reset", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftLayer3_Reset", &PROBE_N, &[], true); }
    r_
}
