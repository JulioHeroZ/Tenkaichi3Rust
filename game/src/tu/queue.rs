#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn Queue_AllocNode(c: &mut Ctx, mut p_queue_1: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Queue_AllocNode", &PROBE_N, &[(p_queue_1) as u32], false); }
    let mut l_node_2: u32 = 0;
    let r_ = 'f: {
    if (((((c.m.r32((p_queue_1).wrapping_add(8))) != (((0i32) as u32))) as i32)) != 0) {
        l_node_2 = c.m.r32((p_queue_1).wrapping_add(8));
        { let v_ = c.m.r32(l_node_2); let a_ = (p_queue_1).wrapping_add(8); c.m.w32(a_, v_); };
    } else {
        if ((((((c.m.r32((p_queue_1).wrapping_add(16)) as i32)) < (((c.m.r32(p_queue_1) as i32)).wrapping_sub(1i32))) as i32)) != 0) {
            l_node_2 = (c.m.r32((p_queue_1).wrapping_add(4))).wrapping_add(((({ let a_ = (p_queue_1).wrapping_add(16); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); o_ }) as u32)).wrapping_mul(8));
        } else {
            break 'f ((0i32) as u32);
        }
    }
    break 'f l_node_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Queue_AllocNode", &PROBE_N, &[], true); }
    r_
}

pub fn Queue_GetTail(c: &mut Ctx, mut p_queue_1: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Queue_GetTail", &PROBE_N, &[(p_queue_1) as u32], false); }
    let mut l_tail_2: u32 = 0;
    let mut l_node_3: u32 = 0;
    let r_ = 'f: {
    l_tail_2 = ((0i32) as u32);
    l_node_3 = c.m.r32((p_queue_1).wrapping_add(12));
    'l1: while (((((l_node_3) != (((0i32) as u32))) as i32)) != 0) {
        l_tail_2 = l_node_3;
        l_node_3 = c.m.r32(l_node_3);
    }
    break 'f l_tail_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Queue_GetTail", &PROBE_N, &[], true); }
    r_
}

pub fn Queue_Init(c: &mut Ctx, mut p_queue_1: u32, mut p_capacity_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Queue_Init", &PROBE_N, &[(p_queue_1) as u32, (p_capacity_2) as u32], false); }
    let r_ = 'f: {
    { let _ = F::memset(c, p_queue_1, 0i32, 20u32); };
    { let v_ = p_capacity_2; let a_ = p_queue_1; c.m.w32(a_, (v_) as u32); };
    { let v_ = F::Heap_Alloc(c, (((((p_capacity_2) as u32)).wrapping_mul(8u32)) as i32), ((32i32) as u32), 0i32, 2i32); let a_ = (p_queue_1).wrapping_add(4); c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32((p_queue_1).wrapping_add(4)); let a2_ = ((((c.m.r32(p_queue_1) as i32)) as u32)).wrapping_mul(8u32); F::memset(c, a0_, 0i32, a2_) }; };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Queue_Init", &PROBE_N, &[], true); }
    r_
}

pub fn Queue_Destroy(c: &mut Ctx, mut p_queue_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Queue_Destroy", &PROBE_N, &[(p_queue_1) as u32], false); }
    let r_ = 'f: {
    if (((((c.m.r32((p_queue_1).wrapping_add(4))) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32((p_queue_1).wrapping_add(4)); F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = (p_queue_1).wrapping_add(4); c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Queue_Destroy", &PROBE_N, &[], true); }
    r_
}

pub fn Queue_Clear(c: &mut Ctx, mut p_queue_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Queue_Clear", &PROBE_N, &[(p_queue_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = ((0i32) as u32); let a_ = (p_queue_1).wrapping_add(8); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (p_queue_1).wrapping_add(12); c.m.w32(a_, v_); };
    { let v_ = 0i32; let a_ = (p_queue_1).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Queue_Clear", &PROBE_N, &[], true); }
    r_
}

pub fn Queue_Push(c: &mut Ctx, mut p_queue_1: u32, mut p_value_2: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Queue_Push", &PROBE_N, &[(p_queue_1) as u32, (p_value_2) as u32], false); }
    let mut l_node_3: u32 = 0;
    let mut l_tail_4: u32 = 0;
    let r_ = 'f: {
    l_node_3 = F::Queue_AllocNode(c, p_queue_1);
    if (((((l_node_3) == (((0i32) as u32))) as i32)) != 0) {
        break 'f ((0i32) as u32);
    }
    if (((((p_value_2) != (((0i32) as u32))) as i32)) != 0) {
        { let v_ = p_value_2; let a_ = (l_node_3).wrapping_add(4); c.m.w32(a_, v_); };
    }
    l_tail_4 = F::Queue_GetTail(c, p_queue_1);
    if (((((l_tail_4) == (((0i32) as u32))) as i32)) != 0) {
        { let v_ = l_node_3; let a_ = (p_queue_1).wrapping_add(12); c.m.w32(a_, v_); };
    } else {
        { let v_ = l_node_3; let a_ = l_tail_4; c.m.w32(a_, v_); };
    }
    { let v_ = ((0i32) as u32); let a_ = l_node_3; c.m.w32(a_, v_); };
    break 'f c.m.r32((l_node_3).wrapping_add(4));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Queue_Push", &PROBE_N, &[], true); }
    r_
}

pub fn Queue_Peek(c: &mut Ctx, mut p_queue_1: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Queue_Peek", &PROBE_N, &[(p_queue_1) as u32], false); }
    let mut l_node_2: u32 = 0;
    let mut l_value_3: u32 = 0;
    let r_ = 'f: {
    l_node_2 = c.m.r32((p_queue_1).wrapping_add(12));
    l_value_3 = ((0i32) as u32);
    if (((((l_node_2) != (((0i32) as u32))) as i32)) != 0) {
        l_value_3 = c.m.r32((l_node_2).wrapping_add(4));
    }
    break 'f l_value_3;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Queue_Peek", &PROBE_N, &[], true); }
    r_
}

pub fn Queue_Pop(c: &mut Ctx, mut p_queue_1: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Queue_Pop", &PROBE_N, &[(p_queue_1) as u32], false); }
    let mut l_node_2: u32 = 0;
    let mut l_value_3: u32 = 0;
    let r_ = 'f: {
    l_node_2 = c.m.r32((p_queue_1).wrapping_add(12));
    l_value_3 = ((0i32) as u32);
    if (((((l_node_2) != (((0i32) as u32))) as i32)) != 0) {
        if (((((c.m.r32(l_node_2)) != (((0i32) as u32))) as i32)) != 0) {
            { let v_ = c.m.r32(l_node_2); let a_ = (p_queue_1).wrapping_add(12); c.m.w32(a_, v_); };
            { let v_ = c.m.r32((p_queue_1).wrapping_add(8)); let a_ = l_node_2; c.m.w32(a_, v_); };
            { let v_ = l_node_2; let a_ = (p_queue_1).wrapping_add(8); c.m.w32(a_, v_); };
        } else {
            { let v_ = ((0i32) as u32); let a_ = (p_queue_1).wrapping_add(12); c.m.w32(a_, v_); };
            { let v_ = c.m.r32((p_queue_1).wrapping_add(8)); let a_ = l_node_2; c.m.w32(a_, v_); };
            { let v_ = l_node_2; let a_ = (p_queue_1).wrapping_add(8); c.m.w32(a_, v_); };
        }
        l_value_3 = c.m.r32((l_node_2).wrapping_add(4));
    }
    break 'f l_value_3;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Queue_Pop", &PROBE_N, &[], true); }
    r_
}

pub fn Queue_Count(c: &mut Ctx, mut p_queue_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Queue_Count", &PROBE_N, &[(p_queue_1) as u32], false); }
    let mut l_count_2: i32 = 0;
    let mut l_node_3: u32 = 0;
    let mut l_next_4: u32 = 0;
    let r_ = 'f: {
    l_count_2 = 0i32;
    l_node_3 = c.m.r32((p_queue_1).wrapping_add(12));
    if (((((l_node_3) != (((0i32) as u32))) as i32)) != 0) {
        'l1: loop {
            'c2: {
                l_next_4 = c.m.r32(l_node_3);
                { let o_ = l_count_2; let n_ = (o_).wrapping_add(1); l_count_2 = n_; };
                l_node_3 = l_next_4;
            }
            if !(((((l_next_4) != (((0i32) as u32))) as i32)) != 0) { break 'l1; }
        }
    }
    break 'f l_count_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Queue_Count", &PROBE_N, &[], true); }
    r_
}
