#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn Job_Clear(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Job_Clear", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let a0_ = c.m.r32(sym::gJobQueue); F::Queue_Clear(c, a0_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Job_Clear", &PROBE_N, &[], true); }
    r_
}

pub fn Job_Init(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Job_Init", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let v_ = F::Heap_Alloc(c, ((20u32) as i32), ((32i32) as u32), 0i32, 2i32); let a_ = sym::gJobQueue; c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32(sym::gJobQueue); F::memset(c, a0_, 0i32, 20u32) }; };
    { let a0_ = c.m.r32(sym::gJobQueue); F::Queue_Init(c, a0_, 4i32) };
    F::Job_Clear(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Job_Init", &PROBE_N, &[], true); }
    r_
}

pub fn Job_Destroy(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Job_Destroy", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let a0_ = c.m.r32(sym::gJobQueue); F::Queue_Destroy(c, a0_) };
    { let a0_ = c.m.r32(sym::gJobQueue); F::Heap_Free(c, a0_) };
    { let v_ = ((0i32) as u32); let a_ = sym::gJobQueue; c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Job_Destroy", &PROBE_N, &[], true); }
    r_
}

pub fn Job_Run(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Job_Run", &PROBE_N, &[], false); }
    let mut l_job_1: u32 = 0;
    let r_ = 'f: {
    l_job_1 = { let a0_ = c.m.r32(sym::gJobQueue); F::Queue_Peek(c, a0_) };
    if (((((l_job_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((({ let fp_ = c.m.r32((l_job_1).wrapping_add(4)); ((D::call(c, fp_, &[((l_job_1) as u64)])) as i32) }) == (1i32)) as i32)) != 0) {
        { let _ = { let a0_ = c.m.r32(sym::gJobQueue); F::Queue_Pop(c, a0_) }; };
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Job_Run", &PROBE_N, &[], true); }
    r_
}

pub fn Job_Push(c: &mut Ctx, mut p_job_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Job_Push", &PROBE_N, &[(p_job_1) as u32], false); }
    let r_ = 'f: {
    { let _ = { let a0_ = c.m.r32(sym::gJobQueue); F::Queue_Push(c, a0_, p_job_1) }; };
    if ((((({ let a0_ = c.m.r32(sym::gJobQueue); F::Queue_Count(c, a0_) }) != (1i32)) as i32)) != 0) {
        break 'f 0i32;
    }
    { let _ = { let fp_ = c.m.r32((p_job_1).wrapping_add(4)); ((D::call(c, fp_, &[((p_job_1) as u64)])) as i32) }; };
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Job_Push", &PROBE_N, &[], true); }
    r_
}

pub fn Job_GetCount(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Job_GetCount", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f { let a0_ = c.m.r32(sym::gJobQueue); F::Queue_Count(c, a0_) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Job_GetCount", &PROBE_N, &[], true); }
    r_
}
