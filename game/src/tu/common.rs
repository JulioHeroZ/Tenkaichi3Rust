#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn Common_LoadBoot(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Common_LoadBoot", &PROBE_N, &[], false); }
    let mut l_res_1: u32 = 0;
    let r_ = 'f: {
    l_res_1 = c.m.r32(sym::gCommonRes);
    { let _ = F::memset(c, l_res_1, 0i32, ((32i32) as u32)); };
    { let v_ = F::File_LoadSync(c, 5i32, ((0i32) as u32), 0i32); let a_ = l_res_1; c.m.w32(a_, v_); };
    { let v_ = F::File_GetSize(c, 2i32); let a_ = ((l_res_1).wrapping_add(20)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    { let v_ = { let a0_ = (c.m.r32(((l_res_1).wrapping_add(20)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32); F::Heap_Alloc(c, a0_, ((64i32) as u32), 0i32, 2i32) }; let a_ = ((l_res_1).wrapping_add(4)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = F::File_GetSize(c, 3i32); let a_ = ((l_res_1).wrapping_add(20)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    { let v_ = { let a0_ = (c.m.r32(((l_res_1).wrapping_add(20)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32); F::Heap_Alloc(c, a0_, ((64i32) as u32), 0i32, 2i32) }; let a_ = ((l_res_1).wrapping_add(4)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = F::File_GetSize(c, 4i32); let a_ = ((l_res_1).wrapping_add(20)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    { let v_ = { let a0_ = (c.m.r32(((l_res_1).wrapping_add(20)).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32); F::Heap_Alloc(c, a0_, ((64i32) as u32), 0i32, 2i32) }; let a_ = ((l_res_1).wrapping_add(4)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Common_LoadBoot", &PROBE_N, &[], true); }
    r_
}

pub fn Common_Reload(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Common_Reload", &PROBE_N, &[], false); }
    let mut l_res_1: u32 = 0;
    let mut l_bank_2: u32 = 0;
    let r_ = 'f: {
    l_res_1 = c.m.r32(sym::gCommonRes);
    F::File_CancelRequests(c);
    { let v_ = { let a1_ = c.m.r32(((l_res_1).wrapping_add(4)).wrapping_add((((0i32) as u32)).wrapping_mul(4))); let a2_ = (c.m.r32(((l_res_1).wrapping_add(20)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32); ((D::call(c, fid::File_Request, &[((2i32) as i64 as u64), ((a1_) as u64), ((a2_) as i64 as u64)])) as u32) }; let a_ = ((l_res_1).wrapping_add(4)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = { let a1_ = c.m.r32(((l_res_1).wrapping_add(4)).wrapping_add((((1i32) as u32)).wrapping_mul(4))); let a2_ = (c.m.r32(((l_res_1).wrapping_add(20)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32); ((D::call(c, fid::File_Request, &[((3i32) as i64 as u64), ((a1_) as u64), ((a2_) as i64 as u64)])) as u32) }; let a_ = ((l_res_1).wrapping_add(4)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = { let a1_ = c.m.r32(((l_res_1).wrapping_add(4)).wrapping_add((((2i32) as u32)).wrapping_mul(4))); let a2_ = (c.m.r32(((l_res_1).wrapping_add(20)).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32); ((D::call(c, fid::File_Request, &[((4i32) as i64 as u64), ((a1_) as u64), ((a2_) as i64 as u64)])) as u32) }; let a_ = ((l_res_1).wrapping_add(4)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    l_bank_2 = ((D::call(c, fid::File_Request, &[((331i32) as i64 as u64), ((((0i32) as u32)) as u64), ((0i32) as i64 as u64)])) as u32);
    'l1: while ((((!((F::File_UpdateRequests(c)) != 0)) as i32)) != 0) {
    }
    F::File_CancelRequests(c);
    F::Snd_LoadBank(c, ((((1i32) as i64 as u64)) as u32), l_bank_2, 1i32);
    F::Heap_Free(c, l_bank_2);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Common_Reload", &PROBE_N, &[], true); }
    r_
}

pub fn Common_Init(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Common_Init", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let v_ = F::Heap_Alloc(c, ((112u32) as i32), ((32i32) as u32), 0i32, 2i32); let a_ = sym::gCommonRes; c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32(sym::gCommonRes); F::memset(c, a0_, 0i32, 112u32) }; };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Common_Init", &PROBE_N, &[], true); }
    r_
}
