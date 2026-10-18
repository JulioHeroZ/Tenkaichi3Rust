#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn BtlPool_Init(c: &mut Ctx, mut p_arg_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlPool_Init", &PROBE_N, &[(p_arg_1) as u32], false); }
    let mut l_size_2: i32 = 0;
    let r_ = 'f: {
    { let v_ = p_arg_1; let a_ = sym::gBtlPoolInitArg; c.m.w32(a_, (v_) as u32); };
    { let v_ = F::Heap_Alloc(c, ((156u32) as i32), ((32i32) as u32), 0i32, 2i32); let a_ = sym::gBtlPool; c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32(sym::gBtlPool); F::memset(c, a0_, 0i32, 156u32) }; };
    { let v_ = ((1022i32) as u32); let a_ = (c.m.r32(sym::gBtlPool)).wrapping_add(152); c.m.w32(a_, v_); };
    l_size_2 = F::BtlPool_CalcArenaSize(c);
    if (((((l_size_2) > (0i32)) as i32)) != 0) {
        { let v_ = F::Heap_Alloc(c, l_size_2, ((32i32) as u32), 0i32, 2i32); let a_ = (c.m.r32(sym::gBtlPool)).wrapping_add(144); c.m.w32(a_, v_); };
        { let _ = { let a0_ = { let a_ = (c.m.r32(sym::gBtlPool)).wrapping_add(144); c.m.r32(a_) }; F::memset(c, a0_, 0i32, ((l_size_2) as u32)) }; };
        F::BtlPool_CarveArenas(c);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlPool_Init", &PROBE_N, &[], true); }
    r_
}

pub fn BtlPool_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlPool_Term", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if ((((({ let a_ = (c.m.r32(sym::gBtlPool)).wrapping_add(144); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = { let a_ = (c.m.r32(sym::gBtlPool)).wrapping_add(144); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = (c.m.r32(sym::gBtlPool)).wrapping_add(144); c.m.w32(a_, v_); };
    }
    if (((((c.m.r32(sym::gBtlPool)) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32(sym::gBtlPool); F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = sym::gBtlPool; c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlPool_Term", &PROBE_N, &[], true); }
    r_
}

pub fn BtlPool_Alloc(c: &mut Ctx, mut p_slot_1: i32, mut p_size_2: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlPool_Alloc", &PROBE_N, &[(p_slot_1) as u32, (p_size_2) as u32], false); }
    let mut l_aligned_3: i32 = 0;
    let mut l_s_4: u32 = 0;
    let mut l_ptr_5: u32 = 0;
    let r_ = 'f: {
    l_aligned_3 = F::BtlPool_AlignUp32(c, p_size_2);
    l_s_4 = (c.m.r32(sym::gBtlPool)).wrapping_add((((p_slot_1) as u32)).wrapping_mul(16));
    if (((({ let a_ = (c.m.r32(sym::gBtlPool)).wrapping_add(152); c.m.r32(a_) }) & ((((1i32).wrapping_shl(((p_slot_1).wrapping_add(1i32)) as u32)) as u32)))) != 0) {
        l_ptr_5 = c.m.r32((l_s_4).wrapping_add(4));
        { let r_ = l_aligned_3; let a_ = (l_s_4).wrapping_add(4); let n_ = (c.m.r32(a_)).wrapping_add(((r_) as u32)); c.m.w32(a_, n_); };
        { let r_ = l_aligned_3; let a_ = (l_s_4).wrapping_add(12); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
    } else {
        { let r_ = l_aligned_3; let a_ = (l_s_4).wrapping_add(12); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
        l_ptr_5 = F::Heap_Alloc(c, p_size_2, ((32i32) as u32), 0i32, 2i32);
    }
    break 'f l_ptr_5;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlPool_Alloc", &PROBE_N, &[], true); }
    r_
}

pub fn BtlPool_Free(c: &mut Ctx, mut p_slot_1: i32, mut p_ptr_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlPool_Free", &PROBE_N, &[(p_slot_1) as u32, (p_ptr_2) as u32], false); }
    let r_ = 'f: {
    if ((((!(((({ let a_ = (c.m.r32(sym::gBtlPool)).wrapping_add(152); c.m.r32(a_) }) & ((((1i32).wrapping_shl(((p_slot_1).wrapping_add(1i32)) as u32)) as u32)))) != 0)) as i32)) != 0) {
        if (((((p_ptr_2) != (((0i32) as u32))) as i32)) != 0) {
            F::Heap_Free(c, p_ptr_2);
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlPool_Free", &PROBE_N, &[], true); }
    r_
}

pub fn BtlPool_Reset(c: &mut Ctx, mut p_slot_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlPool_Reset", &PROBE_N, &[(p_slot_1) as u32], false); }
    let mut l_s_2: u32 = 0;
    let r_ = 'f: {
    l_s_2 = (c.m.r32(sym::gBtlPool)).wrapping_add((((p_slot_1) as u32)).wrapping_mul(16));
    if (((({ let a_ = (c.m.r32(sym::gBtlPool)).wrapping_add(152); c.m.r32(a_) }) & ((((1i32).wrapping_shl(((p_slot_1).wrapping_add(1i32)) as u32)) as u32)))) != 0) {
        { let v_ = c.m.r32(l_s_2); let a_ = (l_s_2).wrapping_add(4); c.m.w32(a_, v_); };
        { let v_ = 0i32; let a_ = (l_s_2).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlPool_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn BtlPool_SetCurrent(c: &mut Ctx, mut p_slot_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlPool_SetCurrent", &PROBE_N, &[(p_slot_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_slot_1; let a_ = (c.m.r32(sym::gBtlPool)).wrapping_add(148); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlPool_SetCurrent", &PROBE_N, &[], true); }
    r_
}

pub fn BtlPool_GetCurrent(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlPool_GetCurrent", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f { let a_ = (c.m.r32(sym::gBtlPool)).wrapping_add(148); (c.m.r32(a_) as i32) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlPool_GetCurrent", &PROBE_N, &[], true); }
    r_
}

pub fn BtlPool_SetInitArg(c: &mut Ctx, mut p_arg_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlPool_SetInitArg", &PROBE_N, &[(p_arg_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_arg_1; let a_ = sym::gBtlPoolInitArg; c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlPool_SetInitArg", &PROBE_N, &[], true); }
    r_
}

pub fn BtlPool_CalcArenaSize(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlPool_CalcArenaSize", &PROBE_N, &[], false); }
    let mut l_total_1: i32 = 0;
    let r_ = 'f: {
    l_total_1 = 0i32;
    if (((({ let a_ = (c.m.r32(sym::gBtlPool)).wrapping_add(152); c.m.r32(a_) }) & ((((1i32).wrapping_shl(((0i32).wrapping_add(1i32)) as u32)) as u32)))) != 0) {
        l_total_1 = F::BtlPool_AlignUp32(c, 40960i32);
    }
    if (((({ let a_ = (c.m.r32(sym::gBtlPool)).wrapping_add(152); c.m.r32(a_) }) & ((((1i32).wrapping_shl(((1i32).wrapping_add(1i32)) as u32)) as u32)))) != 0) {
        { let r_ = F::BtlPool_AlignUp32(c, 1126400i32); let n_ = (l_total_1).wrapping_add(r_); l_total_1 = n_; };
    }
    if (((({ let a_ = (c.m.r32(sym::gBtlPool)).wrapping_add(152); c.m.r32(a_) }) & ((((1i32).wrapping_shl(((2i32).wrapping_add(1i32)) as u32)) as u32)))) != 0) {
        { let r_ = F::BtlPool_AlignUp32(c, 122880i32); let n_ = (l_total_1).wrapping_add(r_); l_total_1 = n_; };
    }
    if (((({ let a_ = (c.m.r32(sym::gBtlPool)).wrapping_add(152); c.m.r32(a_) }) & ((((1i32).wrapping_shl(((3i32).wrapping_add(1i32)) as u32)) as u32)))) != 0) {
        { let r_ = F::BtlPool_AlignUp32(c, 5120i32); let n_ = (l_total_1).wrapping_add(r_); l_total_1 = n_; };
    }
    if (((({ let a_ = (c.m.r32(sym::gBtlPool)).wrapping_add(152); c.m.r32(a_) }) & ((((1i32).wrapping_shl(((4i32).wrapping_add(1i32)) as u32)) as u32)))) != 0) {
        { let r_ = F::BtlPool_AlignUp32(c, 102400i32); let n_ = (l_total_1).wrapping_add(r_); l_total_1 = n_; };
    }
    if (((({ let a_ = (c.m.r32(sym::gBtlPool)).wrapping_add(152); c.m.r32(a_) }) & ((((1i32).wrapping_shl(((5i32).wrapping_add(1i32)) as u32)) as u32)))) != 0) {
        { let r_ = F::BtlPool_AlignUp32(c, 102400i32); let n_ = (l_total_1).wrapping_add(r_); l_total_1 = n_; };
    }
    if (((({ let a_ = (c.m.r32(sym::gBtlPool)).wrapping_add(152); c.m.r32(a_) }) & ((((1i32).wrapping_shl(((6i32).wrapping_add(1i32)) as u32)) as u32)))) != 0) {
        { let r_ = F::BtlPool_AlignUp32(c, 1024i32); let n_ = (l_total_1).wrapping_add(r_); l_total_1 = n_; };
    }
    if (((({ let a_ = (c.m.r32(sym::gBtlPool)).wrapping_add(152); c.m.r32(a_) }) & ((((1i32).wrapping_shl(((7i32).wrapping_add(1i32)) as u32)) as u32)))) != 0) {
        { let r_ = F::BtlPool_AlignUp32(c, 102400i32); let n_ = (l_total_1).wrapping_add(r_); l_total_1 = n_; };
    }
    if (((({ let a_ = (c.m.r32(sym::gBtlPool)).wrapping_add(152); c.m.r32(a_) }) & ((((1i32).wrapping_shl(((8i32).wrapping_add(1i32)) as u32)) as u32)))) != 0) {
        { let r_ = F::BtlPool_AlignUp32(c, 102400i32); let n_ = (l_total_1).wrapping_add(r_); l_total_1 = n_; };
    }
    break 'f l_total_1;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlPool_CalcArenaSize", &PROBE_N, &[], true); }
    r_
}

pub fn BtlPool_CarveArenas(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlPool_CarveArenas", &PROBE_N, &[], false); }
    let mut l_mem_1: u32 = 0;
    let r_ = 'f: {
    l_mem_1 = { let a_ = (c.m.r32(sym::gBtlPool)).wrapping_add(144); c.m.r32(a_) };
    if (((({ let a_ = (c.m.r32(sym::gBtlPool)).wrapping_add(152); c.m.r32(a_) }) & ((((1i32).wrapping_shl(((0i32).wrapping_add(1i32)) as u32)) as u32)))) != 0) {
        l_mem_1 = { let a0_ = (c.m.r32(sym::gBtlPool)).wrapping_add((((0i32) as u32)).wrapping_mul(16)); let a1_ = F::BtlPool_AlignUp32(c, 40960i32); F::BtlPool_InitSlot(c, a0_, a1_, l_mem_1) };
    }
    if (((({ let a_ = (c.m.r32(sym::gBtlPool)).wrapping_add(152); c.m.r32(a_) }) & ((((1i32).wrapping_shl(((1i32).wrapping_add(1i32)) as u32)) as u32)))) != 0) {
        l_mem_1 = { let a0_ = (c.m.r32(sym::gBtlPool)).wrapping_add((((1i32) as u32)).wrapping_mul(16)); let a1_ = F::BtlPool_AlignUp32(c, 1126400i32); F::BtlPool_InitSlot(c, a0_, a1_, l_mem_1) };
    }
    if (((({ let a_ = (c.m.r32(sym::gBtlPool)).wrapping_add(152); c.m.r32(a_) }) & ((((1i32).wrapping_shl(((2i32).wrapping_add(1i32)) as u32)) as u32)))) != 0) {
        l_mem_1 = { let a0_ = (c.m.r32(sym::gBtlPool)).wrapping_add((((2i32) as u32)).wrapping_mul(16)); let a1_ = F::BtlPool_AlignUp32(c, 122880i32); F::BtlPool_InitSlot(c, a0_, a1_, l_mem_1) };
    }
    if (((({ let a_ = (c.m.r32(sym::gBtlPool)).wrapping_add(152); c.m.r32(a_) }) & ((((1i32).wrapping_shl(((3i32).wrapping_add(1i32)) as u32)) as u32)))) != 0) {
        l_mem_1 = { let a0_ = (c.m.r32(sym::gBtlPool)).wrapping_add((((3i32) as u32)).wrapping_mul(16)); let a1_ = F::BtlPool_AlignUp32(c, 5120i32); F::BtlPool_InitSlot(c, a0_, a1_, l_mem_1) };
    }
    if (((({ let a_ = (c.m.r32(sym::gBtlPool)).wrapping_add(152); c.m.r32(a_) }) & ((((1i32).wrapping_shl(((4i32).wrapping_add(1i32)) as u32)) as u32)))) != 0) {
        l_mem_1 = { let a0_ = (c.m.r32(sym::gBtlPool)).wrapping_add((((4i32) as u32)).wrapping_mul(16)); let a1_ = F::BtlPool_AlignUp32(c, 102400i32); F::BtlPool_InitSlot(c, a0_, a1_, l_mem_1) };
    }
    if (((({ let a_ = (c.m.r32(sym::gBtlPool)).wrapping_add(152); c.m.r32(a_) }) & ((((1i32).wrapping_shl(((4i32).wrapping_add(1i32)) as u32)) as u32)))) != 0) {
        l_mem_1 = { let a0_ = (c.m.r32(sym::gBtlPool)).wrapping_add((((5i32) as u32)).wrapping_mul(16)); let a1_ = F::BtlPool_AlignUp32(c, 102400i32); F::BtlPool_InitSlot(c, a0_, a1_, l_mem_1) };
    }
    if (((({ let a_ = (c.m.r32(sym::gBtlPool)).wrapping_add(152); c.m.r32(a_) }) & ((((1i32).wrapping_shl(((6i32).wrapping_add(1i32)) as u32)) as u32)))) != 0) {
        l_mem_1 = { let a0_ = (c.m.r32(sym::gBtlPool)).wrapping_add((((6i32) as u32)).wrapping_mul(16)); let a1_ = F::BtlPool_AlignUp32(c, 1024i32); F::BtlPool_InitSlot(c, a0_, a1_, l_mem_1) };
    }
    if (((({ let a_ = (c.m.r32(sym::gBtlPool)).wrapping_add(152); c.m.r32(a_) }) & ((((1i32).wrapping_shl(((7i32).wrapping_add(1i32)) as u32)) as u32)))) != 0) {
        l_mem_1 = { let a0_ = (c.m.r32(sym::gBtlPool)).wrapping_add((((7i32) as u32)).wrapping_mul(16)); let a1_ = F::BtlPool_AlignUp32(c, 102400i32); F::BtlPool_InitSlot(c, a0_, a1_, l_mem_1) };
    }
    if (((({ let a_ = (c.m.r32(sym::gBtlPool)).wrapping_add(152); c.m.r32(a_) }) & ((((1i32).wrapping_shl(((7i32).wrapping_add(1i32)) as u32)) as u32)))) != 0) {
        l_mem_1 = { let a0_ = (c.m.r32(sym::gBtlPool)).wrapping_add((((8i32) as u32)).wrapping_mul(16)); let a1_ = F::BtlPool_AlignUp32(c, 102400i32); F::BtlPool_InitSlot(c, a0_, a1_, l_mem_1) };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlPool_CarveArenas", &PROBE_N, &[], true); }
    r_
}

pub fn BtlPool_InitSlot(c: &mut Ctx, mut p_slot_1: u32, mut p_size_2: i32, mut p_mem_3: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlPool_InitSlot", &PROBE_N, &[(p_slot_1) as u32, (p_size_2) as u32, (p_mem_3) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_size_2; let a_ = (p_slot_1).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
    { let v_ = p_mem_3; let a_ = p_slot_1; c.m.w32(a_, v_); };
    { let v_ = p_mem_3; let a_ = (p_slot_1).wrapping_add(4); c.m.w32(a_, v_); };
    break 'f (p_mem_3).wrapping_add(((p_size_2) as u32));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlPool_InitSlot", &PROBE_N, &[], true); }
    r_
}

pub fn BtlPool_AlignUp32(c: &mut Ctx, mut p_size_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlPool_AlignUp32", &PROBE_N, &[(p_size_1) as u32], false); }
    let r_ = 'f: {
    if ((rt::rem_i32(p_size_1, 32i32)) != 0) {
        p_size_1 = ((rt::div_i32(p_size_1, 32i32)).wrapping_mul(32i32)).wrapping_add(32i32);
    }
    break 'f p_size_1;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlPool_AlignUp32", &PROBE_N, &[], true); }
    r_
}
