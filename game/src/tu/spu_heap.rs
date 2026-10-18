#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn SpuBlock_Clear(c: &mut Ctx, mut p_block_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SpuBlock_Clear", &PROBE_N, &[(p_block_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = 0i32; let a_ = p_block_1; c.m.w32(a_, (v_) as u32); };
    { let v_ = 0i32; let a_ = (p_block_1).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
    { let v_ = 0i32; let a_ = (p_block_1).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "SpuBlock_Clear", &PROBE_N, &[], true); }
    r_
}

pub fn SpuBlock_SetFree(c: &mut Ctx, mut p_block_1: u32, mut p_addr_2: i32, mut p_size_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SpuBlock_SetFree", &PROBE_N, &[(p_block_1) as u32, (p_addr_2) as u32, (p_size_3) as u32], false); }
    let r_ = 'f: {
    if (((((p_size_3) <= (0i32)) as i32)) != 0) {
        F::SpuBlock_Clear(c, p_block_1);
        break 'f;
    }
    { let v_ = p_size_3; let a_ = (p_block_1).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
    { let v_ = p_addr_2; let a_ = (p_block_1).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
    { let v_ = 1i32; let a_ = p_block_1; c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "SpuBlock_SetFree", &PROBE_N, &[], true); }
    r_
}

pub fn SpuBlock_SetUsed(c: &mut Ctx, mut p_block_1: u32, mut p_addr_2: i32, mut p_size_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SpuBlock_SetUsed", &PROBE_N, &[(p_block_1) as u32, (p_addr_2) as u32, (p_size_3) as u32], false); }
    let r_ = 'f: {
    { let v_ = 2i32; let a_ = p_block_1; c.m.w32(a_, (v_) as u32); };
    { let v_ = p_addr_2; let a_ = (p_block_1).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
    { let v_ = p_size_3; let a_ = (p_block_1).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "SpuBlock_SetUsed", &PROBE_N, &[], true); }
    r_
}

pub fn SpuHeap_FindBlock(c: &mut Ctx, mut p_addr_1: i32, mut p_byEnd_2: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SpuHeap_FindBlock", &PROBE_N, &[(p_addr_1) as u32, (p_byEnd_2) as u32], false); }
    let mut l_found_3: u32 = 0;
    let mut l_block_4: u32 = 0;
    let mut l_key_5: i32 = 0;
    let mut l_i_6: i32 = 0;
    let r_ = 'f: {
    l_found_3 = ((0i32) as u32);
    l_block_4 = sym::gSpuHeapBlocks;
    l_key_5 = 0i32;
    {
        l_i_6 = 0i32;
        'l1: loop {
            if !(((((l_i_6) < (17i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                's3: {
                    let sv7: i32 = p_byEnd_2;
                    match sv7 {
                        0i32 => {
                            l_key_5 = (c.m.r32((l_block_4).wrapping_add(4)) as i32);
                            break 's3;
                        }
                        1i32 => {
                            l_key_5 = ((c.m.r32((l_block_4).wrapping_add(4)) as i32)).wrapping_add((c.m.r32((l_block_4).wrapping_add(8)) as i32));
                            break 's3;
                        }
                        _ => {
                        }
                    }
                }
                if (((((l_key_5) == (p_addr_1)) as i32)) != 0) {
                    l_found_3 = l_block_4;
                    break 'l1;
                }
            }
            { { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; }; { let o_ = l_block_4; let n_ = (o_).wrapping_add(12); l_block_4 = n_; }; };
        }
    }
    break 'f l_found_3;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "SpuHeap_FindBlock", &PROBE_N, &[], true); }
    r_
}

pub fn SpuHeap_Init(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SpuHeap_Init", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let _ = F::memset(c, sym::gSpuHeapBlocks, 0i32, 204u32); };
    F::SpuBlock_SetFree(c, (sym::gSpuHeapBlocks).wrapping_add((((0i32) as u32)).wrapping_mul(12)), 20496i32, 2035664i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "SpuHeap_Init", &PROBE_N, &[], true); }
    r_
}

pub fn SpuHeap_Alloc(c: &mut Ctx, mut p_size_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SpuHeap_Alloc", &PROBE_N, &[(p_size_1) as u32], false); }
    let mut l_addr_2: i32 = 0;
    let mut l_fit_3: u32 = 0;
    let mut l_used_4: u32 = 0;
    let mut l_block_5: u32 = 0;
    let mut l_i_6: i32 = 0;
    let r_ = 'f: {
    l_addr_2 = 20496i32;
    l_fit_3 = ((0i32) as u32);
    l_used_4 = ((0i32) as u32);
    { let r_ = F::Mem_Align16Pad(c, ((p_size_1) as u32)); let n_ = (p_size_1).wrapping_add(r_); p_size_1 = n_; };
    'l1: while ((((({ l_block_5 = F::SpuHeap_FindBlock(c, l_addr_2, 0i32); l_block_5 }) != (((0i32) as u32))) as i32)) != 0) {
        if ((((((((((c.m.r32(l_block_5) as i32)) == (1i32)) as i32)) != 0) && ((((((c.m.r32((l_block_5).wrapping_add(8)) as i32)) >= (p_size_1)) as i32)) != 0)) as i32)) != 0) {
            l_fit_3 = l_block_5;
            break 'l1;
        }
        { let r_ = (c.m.r32((l_block_5).wrapping_add(8)) as i32); let n_ = (l_addr_2).wrapping_add(r_); l_addr_2 = n_; };
    }
    {
        { l_i_6 = 0i32; l_block_5 = sym::gSpuHeapBlocks; };
        'l2: loop {
            if !(((((l_i_6) < (17i32)) as i32)) != 0) { break 'l2; }
            'c3: {
                if ((((((c.m.r32(l_block_5) as i32)) == (0i32)) as i32)) != 0) {
                    l_used_4 = l_block_5;
                    break 'l2;
                }
            }
            { { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; }; { let o_ = l_block_5; let n_ = (o_).wrapping_add(12); l_block_5 = n_; }; };
        }
    }
    { let a1_ = (c.m.r32((l_fit_3).wrapping_add(4)) as i32); F::SpuBlock_SetUsed(c, l_used_4, a1_, p_size_1) };
    { let a1_ = ((c.m.r32((l_fit_3).wrapping_add(4)) as i32)).wrapping_add((c.m.r32((l_used_4).wrapping_add(8)) as i32)); let a2_ = ((c.m.r32((l_fit_3).wrapping_add(8)) as i32)).wrapping_sub((c.m.r32((l_used_4).wrapping_add(8)) as i32)); F::SpuBlock_SetFree(c, l_fit_3, a1_, a2_) };
    break 'f (c.m.r32((l_used_4).wrapping_add(4)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "SpuHeap_Alloc", &PROBE_N, &[], true); }
    r_
}

pub fn SpuHeap_Free(c: &mut Ctx, mut p_addr_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SpuHeap_Free", &PROBE_N, &[(p_addr_1) as u32], false); }
    let mut l_block_2: u32 = 0;
    let mut l_prev_3: u32 = 0;
    let mut l_next_4: u32 = 0;
    let mut l_end_5: i32 = 0;
    let mut l_merge_6: i32 = 0;
    let r_ = 'f: {
    l_block_2 = F::SpuHeap_FindBlock(c, p_addr_1, 0i32);
    l_end_5 = ((c.m.r32((l_block_2).wrapping_add(4)) as i32)).wrapping_add((c.m.r32((l_block_2).wrapping_add(8)) as i32));
    l_merge_6 = 0i32;
    l_prev_3 = F::SpuHeap_FindBlock(c, p_addr_1, 1i32);
    l_next_4 = F::SpuHeap_FindBlock(c, l_end_5, 0i32);
    if (((((l_prev_3) != (((0i32) as u32))) as i32)) != 0) {
        l_merge_6 = ((((c.m.r32(l_prev_3) as i32)) == (1i32)) as i32);
    }
    if (((((((((l_next_4) != (((0i32) as u32))) as i32)) != 0) && ((((((c.m.r32(l_next_4) as i32)) == (1i32)) as i32)) != 0)) as i32)) != 0) {
        { let r_ = 2i32; let n_ = ((l_merge_6) | (r_)); l_merge_6 = n_; };
    }
    's1: {
        let sv7: i32 = l_merge_6;
        match sv7 {
            0i32 => {
                { let a1_ = (c.m.r32((l_block_2).wrapping_add(4)) as i32); let a2_ = (c.m.r32((l_block_2).wrapping_add(8)) as i32); F::SpuBlock_SetFree(c, l_block_2, a1_, a2_) };
                break 's1;
            }
            1i32 => {
                { let a1_ = (c.m.r32((l_prev_3).wrapping_add(4)) as i32); let a2_ = ((c.m.r32((l_prev_3).wrapping_add(8)) as i32)).wrapping_add((c.m.r32((l_block_2).wrapping_add(8)) as i32)); F::SpuBlock_SetFree(c, l_prev_3, a1_, a2_) };
                F::SpuBlock_Clear(c, l_block_2);
                break 's1;
            }
            2i32 => {
                { let a1_ = (c.m.r32((l_block_2).wrapping_add(4)) as i32); let a2_ = ((c.m.r32((l_block_2).wrapping_add(8)) as i32)).wrapping_add((c.m.r32((l_next_4).wrapping_add(8)) as i32)); F::SpuBlock_SetFree(c, l_block_2, a1_, a2_) };
                F::SpuBlock_Clear(c, l_next_4);
                break 's1;
            }
            3i32 => {
                { let a1_ = (c.m.r32((l_prev_3).wrapping_add(4)) as i32); let a2_ = (((c.m.r32((l_prev_3).wrapping_add(8)) as i32)).wrapping_add((c.m.r32((l_block_2).wrapping_add(8)) as i32))).wrapping_add((c.m.r32((l_next_4).wrapping_add(8)) as i32)); F::SpuBlock_SetFree(c, l_prev_3, a1_, a2_) };
                F::SpuBlock_Clear(c, l_block_2);
                F::SpuBlock_Clear(c, l_next_4);
                break 's1;
            }
            _ => {
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "SpuHeap_Free", &PROBE_N, &[], true); }
    r_
}

pub fn SpuHeap_GetUsedSize(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SpuHeap_GetUsedSize", &PROBE_N, &[], false); }
    let mut l_total_1: i32 = 0;
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
    l_total_1 = 0i32;
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < (17i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((((((c.m.r32((sym::gSpuHeapBlocks).wrapping_add((((l_i_2) as u32)).wrapping_mul(12))) as i32)) == (2i32)) as i32)) != 0) {
                    { let r_ = (c.m.r32(((sym::gSpuHeapBlocks).wrapping_add((((l_i_2) as u32)).wrapping_mul(12))).wrapping_add(8)) as i32); let n_ = (l_total_1).wrapping_add(r_); l_total_1 = n_; };
                }
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
    break 'f l_total_1;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "SpuHeap_GetUsedSize", &PROBE_N, &[], true); }
    r_
}

pub fn SpuHeap_GetFreeSize(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SpuHeap_GetFreeSize", &PROBE_N, &[], false); }
    let mut l_total_1: i32 = 0;
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
    l_total_1 = 0i32;
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < (17i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((((((c.m.r32((sym::gSpuHeapBlocks).wrapping_add((((l_i_2) as u32)).wrapping_mul(12))) as i32)) == (1i32)) as i32)) != 0) {
                    { let r_ = (c.m.r32(((sym::gSpuHeapBlocks).wrapping_add((((l_i_2) as u32)).wrapping_mul(12))).wrapping_add(8)) as i32); let n_ = (l_total_1).wrapping_add(r_); l_total_1 = n_; };
                }
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
    break 'f l_total_1;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "SpuHeap_GetFreeSize", &PROBE_N, &[], true); }
    r_
}

pub fn SpuHeap_GetMaxFree(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SpuHeap_GetMaxFree", &PROBE_N, &[], false); }
    let mut l_max_1: i32 = 0;
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
    l_max_1 = 0i32;
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < (17i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((((((c.m.r32((sym::gSpuHeapBlocks).wrapping_add((((l_i_2) as u32)).wrapping_mul(12))) as i32)) == (1i32)) as i32)) != 0) {
                    if (((((l_max_1) < ((c.m.r32(((sym::gSpuHeapBlocks).wrapping_add((((l_i_2) as u32)).wrapping_mul(12))).wrapping_add(8)) as i32))) as i32)) != 0) {
                        l_max_1 = (c.m.r32(((sym::gSpuHeapBlocks).wrapping_add((((l_i_2) as u32)).wrapping_mul(12))).wrapping_add(8)) as i32);
                    }
                }
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
    break 'f l_max_1;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "SpuHeap_GetMaxFree", &PROBE_N, &[], true); }
    r_
}

pub fn SpuHeap_Walk(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SpuHeap_Walk", &PROBE_N, &[], false); }
    let mut l_addr_1: i32 = 0;
    let mut l_block_2: u32 = 0;
    let r_ = 'f: {
    l_addr_1 = 20496i32;
    'l1: while ((((({ l_block_2 = F::SpuHeap_FindBlock(c, l_addr_1, 0i32); l_block_2 }) != (((0i32) as u32))) as i32)) != 0) {
        { let r_ = (c.m.r32((l_block_2).wrapping_add(8)) as i32); let n_ = (l_addr_1).wrapping_add(r_); l_addr_1 = n_; };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "SpuHeap_Walk", &PROBE_N, &[], true); }
    r_
}
