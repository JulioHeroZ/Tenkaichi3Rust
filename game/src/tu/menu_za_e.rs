#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn DcSave_OnDone(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcSave_OnDone", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let r_ = 1i32; let a_ = c.m.r32(sym::gDcSave); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "DcSave_OnDone", &PROBE_N, &[], true); }
    r_
}

pub fn DcSave_Init(c: &mut Ctx, mut p_pack_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcSave_Init", &PROBE_N, &[(p_pack_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = F::Heap_Alloc(c, ((8u32) as i32), ((32i32) as u32), 0i32, 2i32); let a_ = sym::gDcSave; c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32(sym::gDcSave); F::memset(c, a0_, 0i32, 8u32) }; };
    F::Dialog_Init(c, p_pack_1, ((0i32) as u32), 0i32);
    { let _ = 1i32; F::McFlow_Init(c) };
    F::McFlow_SetDoneCb(c, 0i32, fid::DcSave_OnDone, ((((((0i32) as u32)) as u64)) as i32));
    F::McFlow_SetDoneCb(c, 1i32, fid::DcSave_OnDone, ((((((0i32) as u32)) as u64)) as i32));
       
    };
    if rt::probing() { crate::host::harness::probe(c, "DcSave_Init", &PROBE_N, &[], true); }
    r_
}

pub fn DcSave_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcSave_Term", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if (((({ let a_ = c.m.r32(sym::gDcSave); (c.m.r32(a_) as i32) }) & (4i32))) != 0) {
        { let r_ = 1i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(20); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
    }
    F::Dialog_Term(c);
    F::McFlow_Term(c);
    if (((((c.m.r32(sym::gDcSave)) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32(sym::gDcSave); F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = sym::gDcSave; c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "DcSave_Term", &PROBE_N, &[], true); }
    r_
}

pub fn DcSave_Start(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcSave_Start", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::McFlow_Start(c, 0i32);
    { let r_ = 4i32; let a_ = c.m.r32(sym::gDcSave); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "DcSave_Start", &PROBE_N, &[], true); }
    r_
}

pub fn DcSave_Update(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcSave_Update", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if (((({ let a_ = c.m.r32(sym::gDcSave); (c.m.r32(a_) as i32) }) & (4i32))) != 0) {
        { let v_ = F::McFlow_Update(c); let a_ = (c.m.r32(sym::gDcSave)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "DcSave_Update", &PROBE_N, &[], true); }
    r_
}

pub fn DcSave_GetState(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcSave_GetState", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if (((({ let a_ = c.m.r32(sym::gDcSave); (c.m.r32(a_) as i32) }) & (4i32))) != 0) {
        break 'f { let a_ = (c.m.r32(sym::gDcSave)).wrapping_add(4); (c.m.r32(a_) as i32) };
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "DcSave_GetState", &PROBE_N, &[], true); }
    r_
}

pub fn DcSave_IsDone(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcSave_IsDone", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if ((((!(((({ let a_ = c.m.r32(sym::gDcSave); (c.m.r32(a_) as i32) }) & (4i32))) != 0)) as i32)) != 0) {
        break 'f 1i32;
    }
    break 'f (({ let a_ = c.m.r32(sym::gDcSave); (c.m.r32(a_) as i32) }) & (1i32));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "DcSave_IsDone", &PROBE_N, &[], true); }
    r_
}

pub fn DcSave_IsStarted(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DcSave_IsStarted", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f (({ let a_ = c.m.r32(sym::gDcSave); (c.m.r32(a_) as i32) }) & (4i32));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "DcSave_IsStarted", &PROBE_N, &[], true); }
    r_
}
