#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn Heap_BlockData(c: &mut Ctx, mut p_block_1: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Heap_BlockData", &PROBE_N, &[(p_block_1) as u32], false); }
    let r_ = 'f: {
    break 'f (p_block_1).wrapping_add(32u32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Heap_BlockData", &PROBE_N, &[], true); }
    r_
}

pub fn Heap_InitFreeBlock(c: &mut Ctx, mut p_block_1: u32, mut p_size_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Heap_InitFreeBlock", &PROBE_N, &[(p_block_1) as u32, (p_size_2) as u32], false); }
    let r_ = 'f: {
    { let v_ = ((1397244500i32) as u32); let a_ = p_block_1; c.m.w32(a_, v_); };
    { let v_ = ((32i32) as u32); let a_ = (p_block_1).wrapping_add(12); c.m.w32(a_, v_); };
    { let v_ = (p_size_2).wrapping_sub(32i32); let a_ = (p_block_1).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
    { let v_ = 0i32; let a_ = (p_block_1).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
    { let v_ = 0i32; let a_ = (p_block_1).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
    { let v_ = p_size_2; let a_ = (p_block_1).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
    { let v_ = ((0i32) as u32); let a_ = (p_block_1).wrapping_add(20); c.m.w32(a_, v_); };
    { let v_ = p_size_2; let a_ = ((p_block_1).wrapping_add(((p_size_2) as u32))).wrapping_sub(((4i32) as u32)); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Heap_InitFreeBlock", &PROBE_N, &[], true); }
    r_
}

pub fn Heap_InitUsedBlock(c: &mut Ctx, mut p_block_1: u32, mut p_size_2: i32, mut p_dataSize_3: i32, mut p_align_4: u32, mut p_fromTail_5: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Heap_InitUsedBlock", &PROBE_N, &[(p_block_1) as u32, (p_size_2) as u32, (p_dataSize_3) as u32, (p_align_4) as u32, (p_fromTail_5) as u32], false); }
    let mut l_data_6: u32 = 0;
    let r_ = 'f: {
    l_data_6 = F::Heap_BlockData(c, p_block_1);
    { let v_ = ((1397244500i32) as u32); let a_ = p_block_1; c.m.w32(a_, v_); };
    { let v_ = 1i32; let a_ = (p_block_1).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
    { let v_ = p_fromTail_5; let a_ = (p_block_1).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
    { let v_ = p_align_4; let a_ = (p_block_1).wrapping_add(12); c.m.w32(a_, v_); };
    { let v_ = p_size_2; let a_ = (p_block_1).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
    { let v_ = (l_data_6).wrapping_add(rt::rem_u32((p_align_4).wrapping_sub(rt::rem_u32(l_data_6, p_align_4)), p_align_4)); let a_ = (p_block_1).wrapping_add(20); c.m.w32(a_, v_); };
    { let v_ = p_dataSize_3; let a_ = (p_block_1).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
    { let v_ = p_size_2; let a_ = ((p_block_1).wrapping_add(((p_size_2) as u32))).wrapping_sub(((4i32) as u32)); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Heap_InitUsedBlock", &PROBE_N, &[], true); }
    r_
}

pub fn Heap_PrevBlock(c: &mut Ctx, mut p_block_1: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Heap_PrevBlock", &PROBE_N, &[(p_block_1) as u32], false); }
    let mut l_prev_2: u32 = 0;
    let r_ = 'f: {
    l_prev_2 = ((0i32) as u32);
    if (((((p_block_1) != (((0i32) as u32))) as i32)) != 0) {
        if (((((((((p_block_1) != (c.m.r32((sym::gHeapStart).wrapping_add((((0i32) as u32)).wrapping_mul(4))))) as i32)) != 0) && (((((p_block_1) != (c.m.r32((sym::gHeapStart).wrapping_add((((1i32) as u32)).wrapping_mul(4))))) as i32)) != 0)) as i32)) != 0) {
            l_prev_2 = (p_block_1).wrapping_sub((((c.m.r32((p_block_1).wrapping_sub(((4i32) as u32))) as i32)) as u32));
        }
    }
    break 'f l_prev_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Heap_PrevBlock", &PROBE_N, &[], true); }
    r_
}

pub fn Heap_NextBlock(c: &mut Ctx, mut p_block_1: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Heap_NextBlock", &PROBE_N, &[(p_block_1) as u32], false); }
    let mut l_next_2: u32 = 0;
    let r_ = 'f: {
    l_next_2 = ((0i32) as u32);
    if (((((p_block_1) != (((0i32) as u32))) as i32)) != 0) {
        p_block_1 = (p_block_1).wrapping_add((((c.m.r32((p_block_1).wrapping_add(16)) as i32)) as u32));
        if (((((((((p_block_1) != (c.m.r32((sym::gHeapEnd).wrapping_add((((0i32) as u32)).wrapping_mul(4))))) as i32)) != 0) && (((((p_block_1) != (c.m.r32((sym::gHeapEnd).wrapping_add((((1i32) as u32)).wrapping_mul(4))))) as i32)) != 0)) as i32)) != 0) {
            l_next_2 = p_block_1;
        }
    }
    break 'f l_next_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Heap_NextBlock", &PROBE_N, &[], true); }
    r_
}

pub fn Heap_Create(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Heap_Create", &PROBE_N, &[], false); }
    let mut l_size_1: i32 = 0;
    let mut l_start_2: u32 = 0;
    let r_ = 'f: {
    l_size_1 = (((((32485376i32) as u32)).wrapping_sub(sym::D_3BE71C)) as i32);
    { let r_ = F::Mem_Align4Pad(c, ((l_size_1) as u32)); let n_ = (l_size_1).wrapping_add(r_); l_size_1 = n_; };
    l_start_2 = F::Port_Malloc(c, ((l_size_1) as u32));
    { let v_ = l_size_1; let a_ = (sym::gHeapSize).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    { let v_ = (l_start_2).wrapping_add(((l_size_1) as u32)); let a_ = (sym::gHeapEnd).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = l_start_2; let a_ = (sym::gHeapStart).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    F::Heap_InitFreeBlock(c, l_start_2, l_size_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Heap_Create", &PROBE_N, &[], true); }
    r_
}

pub fn Heap_FindBlock(c: &mut Ctx, mut p_data_1: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Heap_FindBlock", &PROBE_N, &[(p_data_1) as u32], false); }
    let mut l_found_2: u32 = 0;
    let mut l_block_3: u32 = 0;
    let mut l_i_4: i32 = 0;
    let r_ = 'f: {
    l_found_2 = ((0i32) as u32);
    {
        l_i_4 = 0i32;
        'l1: loop {
            if !(((((l_i_4) < (2i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                {
                    l_block_3 = c.m.r32((sym::gHeapStart).wrapping_add((((l_i_4) as u32)).wrapping_mul(4)));
                    'l3: loop {
                        if !(((((l_block_3) != (((0i32) as u32))) as i32)) != 0) { break 'l3; }
                        'c4: {
                            if ((((((((((c.m.r32((l_block_3).wrapping_add(4)) as i32)) == (1i32)) as i32)) != 0) && (((((c.m.r32((l_block_3).wrapping_add(20))) == (p_data_1)) as i32)) != 0)) as i32)) != 0) {
                                l_found_2 = l_block_3;
                                break 'l3;
                            }
                        }
                        l_block_3 = F::Heap_NextBlock(c, l_block_3);
                    }
                }
            }
            { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
        }
    }
    break 'f l_found_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Heap_FindBlock", &PROBE_N, &[], true); }
    r_
}

pub fn Heap_SearchFromHead(c: &mut Ctx, mut p_size_1: i32, mut p_heap_2: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Heap_SearchFromHead", &PROBE_N, &[(p_size_1) as u32, (p_heap_2) as u32], false); }
    let mut l_block_3: u32 = 0;
    let mut l_i_4: i32 = 0;
    let r_ = 'f: {
    l_block_3 = ((0i32) as u32);
    if (((((p_heap_2) != (2i32)) as i32)) != 0) {
        l_block_3 = F::Heap_FindFreeFromHead(c, p_heap_2, p_size_1);
    } else {
        {
            l_i_4 = 0i32;
            'l1: loop {
                if !(((((l_i_4) < (2i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_block_3 = F::Heap_FindFreeFromHead(c, l_i_4, p_size_1);
                    if (((((l_block_3) != (((0i32) as u32))) as i32)) != 0) {
                        break 'l1;
                    }
                }
                { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
            }
        }
    }
    break 'f l_block_3;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Heap_SearchFromHead", &PROBE_N, &[], true); }
    r_
}

pub fn Heap_FindFreeFromHead(c: &mut Ctx, mut p_heap_1: i32, mut p_size_2: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Heap_FindFreeFromHead", &PROBE_N, &[(p_heap_1) as u32, (p_size_2) as u32], false); }
    let mut l_found_3: u32 = 0;
    let mut l_block_4: u32 = 0;
    let r_ = 'f: {
    l_found_3 = ((0i32) as u32);
    {
        l_block_4 = c.m.r32((sym::gHeapStart).wrapping_add((((p_heap_1) as u32)).wrapping_mul(4)));
        'l1: loop {
            if !(((((l_block_4) != (((0i32) as u32))) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((((((((((c.m.r32((l_block_4).wrapping_add(4)) as i32)) == (0i32)) as i32)) != 0) && ((((((c.m.r32((l_block_4).wrapping_add(16)) as i32)) >= (p_size_2)) as i32)) != 0)) as i32)) != 0) {
                    l_found_3 = l_block_4;
                    break 'l1;
                }
            }
            l_block_4 = F::Heap_NextBlock(c, l_block_4);
        }
    }
    break 'f l_found_3;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Heap_FindFreeFromHead", &PROBE_N, &[], true); }
    r_
}

pub fn Heap_SearchFromTail(c: &mut Ctx, mut p_size_1: i32, mut p_heap_2: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Heap_SearchFromTail", &PROBE_N, &[(p_size_1) as u32, (p_heap_2) as u32], false); }
    let mut l_block_3: u32 = 0;
    let mut l_i_4: i32 = 0;
    let r_ = 'f: {
    l_block_3 = ((0i32) as u32);
    if (((((p_heap_2) != (2i32)) as i32)) != 0) {
        l_block_3 = F::Heap_FindFreeFromTail(c, p_heap_2, p_size_1);
    } else {
        {
            l_i_4 = (2i32).wrapping_sub(1i32);
            'l1: loop {
                if !(((((l_i_4) >= (0i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_block_3 = F::Heap_FindFreeFromTail(c, l_i_4, p_size_1);
                    if (((((l_block_3) != (((0i32) as u32))) as i32)) != 0) {
                        break 'l1;
                    }
                }
                { let o_ = l_i_4; let n_ = (o_).wrapping_sub(1); l_i_4 = n_; };
            }
        }
    }
    break 'f l_block_3;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Heap_SearchFromTail", &PROBE_N, &[], true); }
    r_
}

pub fn Heap_FindFreeFromTail(c: &mut Ctx, mut p_heap_1: i32, mut p_size_2: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Heap_FindFreeFromTail", &PROBE_N, &[(p_heap_1) as u32, (p_size_2) as u32], false); }
    let mut l_found_3: u32 = 0;
    let mut l_block_4: u32 = 0;
    let r_ = 'f: {
    l_found_3 = ((0i32) as u32);
    {
        l_block_4 = { let a0_ = c.m.r32((sym::gHeapEnd).wrapping_add((((p_heap_1) as u32)).wrapping_mul(4))); F::Heap_PrevBlock(c, a0_) };
        'l1: loop {
            if !(((((l_block_4) != (((0i32) as u32))) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((((((((((c.m.r32((l_block_4).wrapping_add(4)) as i32)) == (0i32)) as i32)) != 0) && ((((((c.m.r32((l_block_4).wrapping_add(16)) as i32)) >= (p_size_2)) as i32)) != 0)) as i32)) != 0) {
                    l_found_3 = l_block_4;
                    break 'l1;
                }
            }
            l_block_4 = F::Heap_PrevBlock(c, l_block_4);
        }
    }
    break 'f l_found_3;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Heap_FindFreeFromTail", &PROBE_N, &[], true); }
    r_
}

pub fn Heap_AllocBlock(c: &mut Ctx, mut p_size_1: i32, mut p_align_2: u32, mut p_fromTail_3: i32, mut p_heap_4: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Heap_AllocBlock", &PROBE_N, &[(p_size_1) as u32, (p_align_2) as u32, (p_fromTail_3) as u32, (p_heap_4) as u32], false); }
    let mut l_block_5: u32 = 0;
    let mut l_free_6: u32 = 0;
    let mut l_need_7: i32 = 0;
    let r_ = 'f: {
    if (((((p_size_1) <= (0i32)) as i32)) != 0) {
        break 'f ((0i32) as u32);
    }
    l_need_7 = ((((((p_size_1) as u32)).wrapping_add(p_align_2)).wrapping_add(((36i32) as u32))) as i32);
    if (((((p_fromTail_3) == (0i32)) as i32)) != 0) {
        l_block_5 = F::Heap_SearchFromHead(c, l_need_7, p_heap_4);
    } else {
        l_block_5 = F::Heap_SearchFromTail(c, l_need_7, p_heap_4);
    }
    if (((((l_block_5) == (((0i32) as u32))) as i32)) != 0) {
        break 'f ((0i32) as u32);
    }
    if (((((((c.m.r32((l_block_5).wrapping_add(24)) as i32)).wrapping_sub(l_need_7)) > (128i32)) as i32)) != 0) {
        if (((((p_fromTail_3) == (0i32)) as i32)) != 0) {
            { let r_ = F::Mem_Align4Pad(c, (l_block_5).wrapping_add(((l_need_7) as u32))); let n_ = (l_need_7).wrapping_add(r_); l_need_7 = n_; };
            { let a1_ = ((c.m.r32((l_block_5).wrapping_add(16)) as i32)).wrapping_sub(l_need_7); F::Heap_InitFreeBlock(c, (l_block_5).wrapping_add(((l_need_7) as u32)), a1_) };
        } else {
            l_free_6 = l_block_5;
            { let r_ = { let a0_ = (l_block_5).wrapping_add(((((c.m.r32((l_block_5).wrapping_add(16)) as i32)).wrapping_sub(l_need_7)) as u32)); F::Mem_Align4Rem(c, a0_) }; let n_ = (l_need_7).wrapping_add(r_); l_need_7 = n_; };
            l_block_5 = (l_block_5).wrapping_add(((((c.m.r32((l_free_6).wrapping_add(16)) as i32)).wrapping_sub(l_need_7)) as u32));
            { let r_ = l_need_7; let a_ = (l_free_6).wrapping_add(24); let n_ = ((c.m.r32(a_) as i32)).wrapping_sub(r_); c.m.w32(a_, (n_) as u32); };
            { let r_ = l_need_7; let a_ = (l_free_6).wrapping_add(16); let n_ = ((c.m.r32(a_) as i32)).wrapping_sub(r_); c.m.w32(a_, (n_) as u32); };
            { let v_ = (c.m.r32((l_free_6).wrapping_add(16)) as i32); let a_ = ((l_free_6).wrapping_add((((c.m.r32((l_free_6).wrapping_add(16)) as i32)) as u32))).wrapping_sub(((4i32) as u32)); c.m.w32(a_, (v_) as u32); };
        }
    } else {
        l_need_7 = (c.m.r32((l_block_5).wrapping_add(16)) as i32);
    }
    F::Heap_InitUsedBlock(c, l_block_5, l_need_7, p_size_1, p_align_2, p_fromTail_3);
    break 'f l_block_5;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Heap_AllocBlock", &PROBE_N, &[], true); }
    r_
}

pub fn Heap_FreeBlock(c: &mut Ctx, mut p_block_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Heap_FreeBlock", &PROBE_N, &[(p_block_1) as u32], false); }
    let mut l_prev_2: u32 = 0;
    let mut l_next_3: u32 = 0;
    let mut l_merge_4: i32 = 0;
    let r_ = 'f: {
    l_merge_4 = 0i32;
    { let a1_ = (c.m.r32((p_block_1).wrapping_add(16)) as i32); F::Heap_InitFreeBlock(c, p_block_1, a1_) };
    l_prev_2 = F::Heap_PrevBlock(c, p_block_1);
    l_next_3 = F::Heap_NextBlock(c, p_block_1);
    if (((((l_prev_2) != (((0i32) as u32))) as i32)) != 0) {
        l_merge_4 = ((((c.m.r32((l_prev_2).wrapping_add(4)) as i32)) == (0i32)) as i32);
    }
    if (((((((((l_next_3) != (((0i32) as u32))) as i32)) != 0) && ((((((c.m.r32((l_next_3).wrapping_add(4)) as i32)) == (0i32)) as i32)) != 0)) as i32)) != 0) {
        { let r_ = 2i32; let n_ = ((l_merge_4) | (r_)); l_merge_4 = n_; };
    }
    's1: {
        let sv5: i32 = l_merge_4;
        match sv5 {
            0i32 => {
                break 's1;
            }
            1i32 => {
                { let r_ = (c.m.r32((p_block_1).wrapping_add(16)) as i32); let a_ = (l_prev_2).wrapping_add(16); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                { let r_ = (c.m.r32((p_block_1).wrapping_add(16)) as i32); let a_ = (l_prev_2).wrapping_add(24); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                { let v_ = (c.m.r32((l_prev_2).wrapping_add(16)) as i32); let a_ = ((l_prev_2).wrapping_add((((c.m.r32((l_prev_2).wrapping_add(16)) as i32)) as u32))).wrapping_sub(((4i32) as u32)); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            2i32 => {
                { let r_ = (c.m.r32((l_next_3).wrapping_add(16)) as i32); let a_ = (p_block_1).wrapping_add(16); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                { let r_ = (c.m.r32((l_next_3).wrapping_add(16)) as i32); let a_ = (p_block_1).wrapping_add(24); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                { let v_ = (c.m.r32((p_block_1).wrapping_add(16)) as i32); let a_ = ((p_block_1).wrapping_add((((c.m.r32((p_block_1).wrapping_add(16)) as i32)) as u32))).wrapping_sub(((4i32) as u32)); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            3i32 => {
                { let r_ = ((c.m.r32((p_block_1).wrapping_add(16)) as i32)).wrapping_add((c.m.r32((l_next_3).wrapping_add(16)) as i32)); let a_ = (l_prev_2).wrapping_add(16); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                { let r_ = ((c.m.r32((p_block_1).wrapping_add(16)) as i32)).wrapping_add((c.m.r32((l_next_3).wrapping_add(16)) as i32)); let a_ = (l_prev_2).wrapping_add(24); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                { let v_ = (c.m.r32((l_prev_2).wrapping_add(16)) as i32); let a_ = ((l_prev_2).wrapping_add((((c.m.r32((l_prev_2).wrapping_add(16)) as i32)) as u32))).wrapping_sub(((4i32) as u32)); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            _ => {
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Heap_FreeBlock", &PROBE_N, &[], true); }
    r_
}

pub fn Heap_Init(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Heap_Init", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let v_ = 0i32; let a_ = sym::gHeapDebugValue; c.m.w32(a_, (v_) as u32); };
    { let v_ = ((0i32) as u8); let a_ = sym::gHeapDebugFlag; c.m.w8(a_, v_); };
    { let v_ = 0i32; let a_ = sym::gHeapDebugUnused; c.m.w32(a_, (v_) as u32); };
    F::Heap_Create(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Heap_Init", &PROBE_N, &[], true); }
    r_
}

pub fn Heap_Alloc(c: &mut Ctx, mut p_size_1: i32, mut p_align_2: u32, mut p_fromTail_3: i32, mut p_heap_4: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Heap_Alloc", &PROBE_N, &[(p_size_1) as u32, (p_align_2) as u32, (p_fromTail_3) as u32, (p_heap_4) as u32], false); }
    let mut l_block_5: u32 = 0;
    let mut l_data_6: u32 = 0;
    let r_ = 'f: {
    l_block_5 = F::Heap_AllocBlock(c, p_size_1, p_align_2, p_fromTail_3, p_heap_4);
    l_data_6 = ((0i32) as u32);
    if (((((l_block_5) != (((0i32) as u32))) as i32)) != 0) {
        l_data_6 = c.m.r32((l_block_5).wrapping_add(20));
    }
    break 'f l_data_6;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Heap_Alloc", &PROBE_N, &[], true); }
    r_
}

pub fn Heap_Free(c: &mut Ctx, mut p_ptr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Heap_Free", &PROBE_N, &[(p_ptr_1) as u32], false); }
    let mut l_block_2: u32 = 0;
    let r_ = 'f: {
    {
        if ((F::Port_GameBigFree(c, p_ptr_1)) != 0) {
            break 'f;
        }
    }
    l_block_2 = F::Heap_FindBlock(c, p_ptr_1);
    if (((((l_block_2) == (((0i32) as u32))) as i32)) != 0) {
        F::Heap_ReportBadFree(c);
    }
    F::Heap_FreeBlock(c, l_block_2);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Heap_Free", &PROBE_N, &[], true); }
    r_
}

pub fn Heap_SetDebug(c: &mut Ctx, mut p_value_1: i32, mut p_flag_2: u8) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Heap_SetDebug", &PROBE_N, &[(p_value_1) as u32, (p_flag_2) as u32], false); }
    let r_ = 'f: {
    F::Heap_SetDebugValue(c, p_value_1);
    F::Heap_SetDebugFlag(c, p_flag_2);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Heap_SetDebug", &PROBE_N, &[], true); }
    r_
}

pub fn Heap_SetDebugValue(c: &mut Ctx, mut p_value_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Heap_SetDebugValue", &PROBE_N, &[(p_value_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_value_1; let a_ = sym::gHeapDebugValue; c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Heap_SetDebugValue", &PROBE_N, &[], true); }
    r_
}

pub fn Heap_SetDebugFlag(c: &mut Ctx, mut p_flag_1: u8) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Heap_SetDebugFlag", &PROBE_N, &[(p_flag_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_flag_1; let a_ = sym::gHeapDebugFlag; c.m.w8(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Heap_SetDebugFlag", &PROBE_N, &[], true); }
    r_
}

pub fn Heap_GetFreeSize(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Heap_GetFreeSize", &PROBE_N, &[], false); }
    let mut l_total_1: i32 = 0;
    let mut l_block_2: u32 = 0;
    let mut l_i_3: i32 = 0;
    let r_ = 'f: {
    l_total_1 = 0i32;
    {
        l_i_3 = 0i32;
        'l1: loop {
            if !(((((l_i_3) < (2i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                {
                    l_block_2 = c.m.r32((sym::gHeapStart).wrapping_add((((l_i_3) as u32)).wrapping_mul(4)));
                    'l3: loop {
                        if !(((((l_block_2) != (((0i32) as u32))) as i32)) != 0) { break 'l3; }
                        'c4: {
                            if ((((((c.m.r32((l_block_2).wrapping_add(4)) as i32)) == (0i32)) as i32)) != 0) {
                                { let r_ = (c.m.r32((l_block_2).wrapping_add(24)) as i32); let n_ = (l_total_1).wrapping_add(r_); l_total_1 = n_; };
                            }
                        }
                        l_block_2 = F::Heap_NextBlock(c, l_block_2);
                    }
                }
            }
            { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
        }
    }
    break 'f l_total_1;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Heap_GetFreeSize", &PROBE_N, &[], true); }
    r_
}

pub fn Heap_GetUsedSize(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Heap_GetUsedSize", &PROBE_N, &[], false); }
    let mut l_total_1: i32 = 0;
    let mut l_block_2: u32 = 0;
    let mut l_i_3: i32 = 0;
    let r_ = 'f: {
    l_total_1 = 0i32;
    {
        l_i_3 = 0i32;
        'l1: loop {
            if !(((((l_i_3) < (2i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                {
                    l_block_2 = c.m.r32((sym::gHeapStart).wrapping_add((((l_i_3) as u32)).wrapping_mul(4)));
                    'l3: loop {
                        if !(((((l_block_2) != (((0i32) as u32))) as i32)) != 0) { break 'l3; }
                        'c4: {
                            if ((((((c.m.r32((l_block_2).wrapping_add(4)) as i32)) == (1i32)) as i32)) != 0) {
                                { let r_ = (c.m.r32((l_block_2).wrapping_add(24)) as i32); let n_ = (l_total_1).wrapping_add(r_); l_total_1 = n_; };
                            }
                        }
                        l_block_2 = F::Heap_NextBlock(c, l_block_2);
                    }
                }
            }
            { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
        }
    }
    break 'f l_total_1;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Heap_GetUsedSize", &PROBE_N, &[], true); }
    r_
}

pub fn Heap_GetLargestFree(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Heap_GetLargestFree", &PROBE_N, &[], false); }
    let mut l_largest_1: u32 = 0;
    let mut l_block_2: u32 = 0;
    let mut l_i_3: i32 = 0;
    let r_ = 'f: {
    l_largest_1 = ((0i32) as u32);
    {
        l_i_3 = 0i32;
        'l1: loop {
            if !(((((l_i_3) < (2i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                {
                    l_block_2 = c.m.r32((sym::gHeapStart).wrapping_add((((l_i_3) as u32)).wrapping_mul(4)));
                    'l3: loop {
                        if !(((((l_block_2) != (((0i32) as u32))) as i32)) != 0) { break 'l3; }
                        'c4: {
                            if ((((((c.m.r32((l_block_2).wrapping_add(4)) as i32)) == (0i32)) as i32)) != 0) {
                                if (((((l_largest_1) < ((((c.m.r32((l_block_2).wrapping_add(24)) as i32)) as u32))) as i32)) != 0) {
                                    l_largest_1 = (((c.m.r32((l_block_2).wrapping_add(24)) as i32)) as u32);
                                }
                            }
                        }
                        l_block_2 = F::Heap_NextBlock(c, l_block_2);
                    }
                }
            }
            { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
        }
    }
    break 'f l_largest_1;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Heap_GetLargestFree", &PROBE_N, &[], true); }
    r_
}

pub fn Heap_GetTotalSize(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Heap_GetTotalSize", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f ((c.m.r32((sym::gHeapSize).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)).wrapping_add((c.m.r32((sym::gHeapSize).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Heap_GetTotalSize", &PROBE_N, &[], true); }
    r_
}

pub fn Heap_ReportBadFree(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Heap_ReportBadFree", &PROBE_N, &[], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Heap_ReportBadFree", &PROBE_N, &[], true); }
    r_
}

pub fn Heap_GetStart(c: &mut Ctx, mut p_heap_1: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Heap_GetStart", &PROBE_N, &[(p_heap_1) as u32], false); }
    let r_ = 'f: {
    break 'f c.m.r32((sym::gHeapStart).wrapping_add((((p_heap_1) as u32)).wrapping_mul(4)));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Heap_GetStart", &PROBE_N, &[], true); }
    r_
}

pub fn Heap_GetEnd(c: &mut Ctx, mut p_heap_1: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Heap_GetEnd", &PROBE_N, &[(p_heap_1) as u32], false); }
    let r_ = 'f: {
    break 'f c.m.r32((sym::gHeapEnd).wrapping_add((((p_heap_1) as u32)).wrapping_mul(4)));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Heap_GetEnd", &PROBE_N, &[], true); }
    r_
}

pub fn Heap_GetSize(c: &mut Ctx, mut p_heap_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Heap_GetSize", &PROBE_N, &[(p_heap_1) as u32], false); }
    let r_ = 'f: {
    break 'f (c.m.r32((sym::gHeapSize).wrapping_add((((p_heap_1) as u32)).wrapping_mul(4))) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Heap_GetSize", &PROBE_N, &[], true); }
    r_
}

pub fn Heap_GetUnk70(c: &mut Ctx, mut p_heap_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Heap_GetUnk70", &PROBE_N, &[(p_heap_1) as u32], false); }
    let r_ = 'f: {
    break 'f (c.m.r32((sym::gHeapUnk70).wrapping_add((((p_heap_1) as u32)).wrapping_mul(4))) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Heap_GetUnk70", &PROBE_N, &[], true); }
    r_
}

pub fn Heap_GetUnk78(c: &mut Ctx, mut p_heap_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Heap_GetUnk78", &PROBE_N, &[(p_heap_1) as u32], false); }
    let r_ = 'f: {
    break 'f (c.m.r32((sym::gHeapUnk78).wrapping_add((((p_heap_1) as u32)).wrapping_mul(4))) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Heap_GetUnk78", &PROBE_N, &[], true); }
    r_
}
