#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn Heap_CountBlocks(c: &mut Ctx, mut p_heap_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Heap_CountBlocks", &PROBE_N, &[(p_heap_1) as u32], false); }
    let r_ = 'f: {
    break 'f (F::Heap_CountFreeBlocks(c, p_heap_1)).wrapping_add(F::Heap_CountUsedBlocks(c, p_heap_1));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Heap_CountBlocks", &PROBE_N, &[], true); }
    r_
}

pub fn Heap_CountFreeBlocks(c: &mut Ctx, mut p_heap_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Heap_CountFreeBlocks", &PROBE_N, &[(p_heap_1) as u32], false); }
    let mut l_count_2: i32 = 0;
    let mut l_block_3: u32 = 0;
    let r_ = 'f: {
    l_count_2 = 0i32;
    {
        l_block_3 = c.m.r32((sym::gHeapStart).wrapping_add((((p_heap_1) as u32)).wrapping_mul(4)));
        'l1: loop {
            if !(((((l_block_3) != (((0i32) as u32))) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((((((c.m.r32((l_block_3).wrapping_add(4)) as i32)) == (0i32)) as i32)) != 0) {
                    { let o_ = l_count_2; let n_ = (o_).wrapping_add(1); l_count_2 = n_; };
                }
            }
            l_block_3 = F::Heap_NextBlock(c, l_block_3);
        }
    }
    break 'f l_count_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Heap_CountFreeBlocks", &PROBE_N, &[], true); }
    r_
}

pub fn Heap_CountUsedBlocks(c: &mut Ctx, mut p_heap_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Heap_CountUsedBlocks", &PROBE_N, &[(p_heap_1) as u32], false); }
    let mut l_count_2: i32 = 0;
    let mut l_block_3: u32 = 0;
    let r_ = 'f: {
    l_count_2 = 0i32;
    {
        l_block_3 = c.m.r32((sym::gHeapStart).wrapping_add((((p_heap_1) as u32)).wrapping_mul(4)));
        'l1: loop {
            if !(((((l_block_3) != (((0i32) as u32))) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((((((c.m.r32((l_block_3).wrapping_add(4)) as i32)) == (1i32)) as i32)) != 0) {
                    { let o_ = l_count_2; let n_ = (o_).wrapping_add(1); l_count_2 = n_; };
                }
            }
            l_block_3 = F::Heap_NextBlock(c, l_block_3);
        }
    }
    break 'f l_count_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Heap_CountUsedBlocks", &PROBE_N, &[], true); }
    r_
}
