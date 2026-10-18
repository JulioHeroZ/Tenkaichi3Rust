#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn Dma_InitBuffers(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Dma_InitBuffers", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let v_ = F::Heap_Alloc(c, 1048576i32, ((32i32) as u32), 0i32, 2i32); let a_ = (sym::gDmaBuf).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = F::Heap_Alloc(c, 1048576i32, ((32i32) as u32), 0i32, 2i32); let a_ = (sym::gDmaBuf).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = (c.m.r32((sym::gDmaBuf).wrapping_add((((0i32) as u32)).wrapping_mul(4)))).wrapping_add(((1048576i32) as u32)); let a_ = (sym::gDmaBufEnd).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = (c.m.r32((sym::gDmaBuf).wrapping_add((((1i32) as u32)).wrapping_mul(4)))).wrapping_add(((1048576i32) as u32)); let a_ = (sym::gDmaBufEnd).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = 1048576i32; let a_ = sym::gDmaBufSize; c.m.w32(a_, (v_) as u32); };
    { let v_ = c.m.r32((sym::gDmaBuf).wrapping_add((((0i32) as u32)).wrapping_mul(4))); let a_ = sym::gDmaCur; c.m.w32(a_, v_); };
    { let v_ = 0i32; let a_ = sym::gDmaBufIdx; c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Dma_InitBuffers", &PROBE_N, &[], true); }
    r_
}

pub fn Dma_AddData(c: &mut Ctx, mut p_src_1: u32, mut p_size_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Dma_AddData", &PROBE_N, &[(p_src_1) as u32, (p_size_2) as u32], false); }
    let r_ = 'f: {
    { let _ = { let a0_ = c.m.r32(sym::gDmaCur); F::memcpy(c, a0_, p_src_1, ((p_size_2) as u32)) }; };
    { let v_ = (c.m.r32(sym::gDmaCur)).wrapping_add((((rt::div_i32(p_size_2, 16i32)).wrapping_mul(16i32)) as u32)); let a_ = sym::gDmaCur; c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Dma_AddData", &PROBE_N, &[], true); }
    r_
}

pub fn Dma_AddRef(c: &mut Ctx, mut p_addr_1: u32, mut p_size_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Dma_AddRef", &PROBE_N, &[(p_addr_1) as u32, (p_size_2) as u32], false); }
    let mut l_tag_3: u32 = 0;
    let r_ = 'f: {
    l_tag_3 = c.m.r32(sym::gDmaCur);
    { let v_ = (((805306368i32).wrapping_add(rt::div_i32(p_size_2, 16i32))) as u32); let a_ = (l_tag_3).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = p_addr_1; let a_ = (l_tag_3).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (l_tag_3).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (l_tag_3).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let r_ = 4i32; let n_ = (l_tag_3).wrapping_add((((r_) as u32)).wrapping_mul(4)); l_tag_3 = n_; };
    { let v_ = l_tag_3; let a_ = sym::gDmaCur; c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Dma_AddRef", &PROBE_N, &[], true); }
    r_
}

pub fn Dma_AddNext(c: &mut Ctx, mut p_addr_1: u32, mut p_retTag_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Dma_AddNext", &PROBE_N, &[(p_addr_1) as u32, (p_retTag_2) as u32], false); }
    let mut l_tag_3: u32 = 0;
    let r_ = 'f: {
    l_tag_3 = c.m.r32(sym::gDmaCur);
    { let v_ = ((536870912i32) as u32); let a_ = (l_tag_3).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = p_addr_1; let a_ = (l_tag_3).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (l_tag_3).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (l_tag_3).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let r_ = 4i32; let n_ = (l_tag_3).wrapping_add((((r_) as u32)).wrapping_mul(4)); l_tag_3 = n_; };
    { let v_ = l_tag_3; let a_ = (p_retTag_2).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = l_tag_3; let a_ = sym::gDmaCur; c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Dma_AddNext", &PROBE_N, &[], true); }
    r_
}

pub fn Dma_Flush(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Dma_Flush", &PROBE_N, &[], false); }
    let mut l_p_1: u32 = 0;
    let r_ = 'f: {
    l_p_1 = c.m.r32(sym::gDmaCur);
    { let v_ = ((((268435456i32) | (2i32))) as u32); let a_ = (l_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (l_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((268435456i32) as u32); let a_ = (l_p_1).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((((1342177280i32) | (2i32))) as u32); let a_ = (l_p_1).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let r_ = 4i32; let n_ = (l_p_1).wrapping_add((((r_) as u32)).wrapping_mul(4)); l_p_1 = n_; };
    { let v_ = ((((((((1i32) as u64)) | ((((1i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (l_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((14i32) as u64); let a_ = (l_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 4i32; let n_ = (l_p_1).wrapping_add((((r_) as u32)).wrapping_mul(4)); l_p_1 = n_; };
    { let v_ = ((0i32) as u64); let a_ = (l_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((97i32) as u64); let a_ = (l_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 4i32; let n_ = (l_p_1).wrapping_add((((r_) as u32)).wrapping_mul(4)); l_p_1 = n_; };
    { let v_ = ((1879048192i32) as u32); let a_ = (l_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (l_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (l_p_1).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (l_p_1).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let a0_ = { let a_ = (sym::gDmaBuf).wrapping_add(((((c.m.r32(sym::gDmaBufIdx) as i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) }; F::Dma_SendVif1(c, a0_) };
    { let r_ = 1i32; let a_ = sym::gDmaBufIdx; let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
    { let v_ = { let a_ = (sym::gDmaBuf).wrapping_add(((((c.m.r32(sym::gDmaBufIdx) as i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) }; let a_ = sym::gDmaCur; c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Dma_Flush", &PROBE_N, &[], true); }
    r_
}

pub fn Dma_Alloc(c: &mut Ctx, mut p_size_1: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Dma_Alloc", &PROBE_N, &[(p_size_1) as u32], false); }
    let mut l_p_2: u32 = 0;
    let r_ = 'f: {
    l_p_2 = c.m.r32(sym::gDmaCur);
    { let v_ = (c.m.r32(sym::gDmaCur)).wrapping_add((((rt::div_i32(p_size_1, 16i32)).wrapping_mul(16i32)) as u32)); let a_ = sym::gDmaCur; c.m.w32(a_, v_); };
    break 'f l_p_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Dma_Alloc", &PROBE_N, &[], true); }
    r_
}

pub fn Dma_BeginDirect(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Dma_BeginDirect", &PROBE_N, &[], false); }
    let mut l_tag_1: u32 = 0;
    let r_ = 'f: {
    l_tag_1 = c.m.r32(sym::gDmaCur);
    { let v_ = ((0i32) as u64); let a_ = l_tag_1; c.m.w64(a_, v_); };
    { let v_ = ((0i32) as u64); let a_ = (l_tag_1).wrapping_add(8); c.m.w64(a_, v_); };
    break 'f (l_tag_1).wrapping_add((((1i32) as u32)).wrapping_mul(16));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Dma_BeginDirect", &PROBE_N, &[], true); }
    r_
}

pub fn Dma_EndDirect(c: &mut Ctx, mut p_end_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Dma_EndDirect", &PROBE_N, &[(p_end_1) as u32], false); }
    let mut l_tag_2: u32 = 0;
    let mut l_qwc_3: i32 = 0;
    let r_ = 'f: {
    l_tag_2 = c.m.r32(sym::gDmaCur);
    if (((((p_end_1) != ((l_tag_2).wrapping_add((((4i32) as u32)).wrapping_mul(4)))) as i32)) != 0) {
        l_qwc_3 = ((((p_end_1).wrapping_sub(l_tag_2) as i32)).wrapping_shr((4i32) as u32)).wrapping_sub(1i32);
        { let v_ = ((((268435456i32) | (l_qwc_3))) as u32); let a_ = (l_tag_2).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = (l_tag_2).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ((268435456i32) as u32); let a_ = (l_tag_2).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ((((1342177280i32) | (l_qwc_3))) as u32); let a_ = (l_tag_2).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        F::FlushCache(c, 0i32);
        { let v_ = p_end_1; let a_ = sym::gDmaCur; c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Dma_EndDirect", &PROBE_N, &[], true); }
    r_
}

pub fn Dma_ResetBuffers(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Dma_ResetBuffers", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let v_ = ((1879048192i32) as u32); let a_ = (c.m.r32((sym::gDmaBuf).wrapping_add((((0i32) as u32)).wrapping_mul(4)))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (c.m.r32((sym::gDmaBuf).wrapping_add((((0i32) as u32)).wrapping_mul(4)))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (c.m.r32((sym::gDmaBuf).wrapping_add((((0i32) as u32)).wrapping_mul(4)))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (c.m.r32((sym::gDmaBuf).wrapping_add((((0i32) as u32)).wrapping_mul(4)))).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((1879048192i32) as u32); let a_ = (c.m.r32((sym::gDmaBuf).wrapping_add((((1i32) as u32)).wrapping_mul(4)))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (c.m.r32((sym::gDmaBuf).wrapping_add((((1i32) as u32)).wrapping_mul(4)))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (c.m.r32((sym::gDmaBuf).wrapping_add((((1i32) as u32)).wrapping_mul(4)))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (c.m.r32((sym::gDmaBuf).wrapping_add((((1i32) as u32)).wrapping_mul(4)))).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = { let a_ = (sym::gDmaBuf).wrapping_add(((((c.m.r32(sym::gDmaBufIdx) as i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) }; let a_ = sym::gDmaCur; c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Dma_ResetBuffers", &PROBE_N, &[], true); }
    r_
}

pub fn Dma_GetBufSize(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Dma_GetBufSize", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f (c.m.r32(sym::gDmaBufSize) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Dma_GetBufSize", &PROBE_N, &[], true); }
    r_
}

pub fn Dma_GetUsedSize(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Dma_GetUsedSize", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f (((c.m.r32(sym::gDmaCur)).wrapping_sub({ let a_ = (sym::gDmaBuf).wrapping_add(((((c.m.r32(sym::gDmaBufIdx) as i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) })) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Dma_GetUsedSize", &PROBE_N, &[], true); }
    r_
}

pub fn Dma_GetFreeSize(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Dma_GetFreeSize", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f (F::Dma_GetBufSize(c)).wrapping_sub(F::Dma_GetUsedSize(c));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Dma_GetFreeSize", &PROBE_N, &[], true); }
    r_
}

pub fn Dma_LendBufferTails(c: &mut Ctx, mut p_tail0_1: u32, mut p_tail1_2: u32, mut p_size_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Dma_LendBufferTails", &PROBE_N, &[(p_tail0_1) as u32, (p_tail1_2) as u32, (p_size_3) as u32], false); }
    let r_ = 'f: {
    { let v_ = 331776i32; let a_ = sym::gDmaBufSize; c.m.w32(a_, (v_) as u32); };
    { let v_ = (c.m.r32((sym::gDmaBuf).wrapping_add((((0i32) as u32)).wrapping_mul(4)))).wrapping_add(((331776i32) as u32)); let a_ = (sym::gDmaBufEnd).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = (c.m.r32((sym::gDmaBuf).wrapping_add((((1i32) as u32)).wrapping_mul(4)))).wrapping_add(((331776i32) as u32)); let a_ = (sym::gDmaBufEnd).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = c.m.r32((sym::gDmaBufEnd).wrapping_add((((0i32) as u32)).wrapping_mul(4))); let a_ = p_tail0_1; c.m.w32(a_, v_); };
    { let v_ = c.m.r32((sym::gDmaBufEnd).wrapping_add((((1i32) as u32)).wrapping_mul(4))); let a_ = p_tail1_2; c.m.w32(a_, v_); };
    { let v_ = (1048576i32).wrapping_sub(331776i32); let a_ = p_size_3; c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Dma_LendBufferTails", &PROBE_N, &[], true); }
    r_
}

pub fn Dma_ReclaimBufferTails(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Dma_ReclaimBufferTails", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let v_ = 1048576i32; let a_ = sym::gDmaBufSize; c.m.w32(a_, (v_) as u32); };
    { let v_ = (c.m.r32((sym::gDmaBuf).wrapping_add((((0i32) as u32)).wrapping_mul(4)))).wrapping_add(((1048576i32) as u32)); let a_ = (sym::gDmaBufEnd).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = (c.m.r32((sym::gDmaBuf).wrapping_add((((1i32) as u32)).wrapping_mul(4)))).wrapping_add(((1048576i32) as u32)); let a_ = (sym::gDmaBufEnd).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Dma_ReclaimBufferTails", &PROBE_N, &[], true); }
    r_
}

pub fn Dma_InitController(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Dma_InitController", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let v_ = ((1i32) as u32); let a_ = ((268492800i32) as u32); c.m.w32(a_, v_); };
    { let v_ = 2214526978u32; let a_ = ((268492832i32) as u32); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = ((268492816i32) as u32); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Dma_InitController", &PROBE_N, &[], true); }
    r_
}

pub fn Dma_WaitCond(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Dma_WaitCond", &PROBE_N, &[], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Dma_WaitCond", &PROBE_N, &[], true); }
    r_
}

pub fn Dma_WaitVu1(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Dma_WaitVu1", &PROBE_N, &[], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Dma_WaitVu1", &PROBE_N, &[], true); }
    r_
}

pub fn Dma_SendVif1(c: &mut Ctx, mut p_chain_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Dma_SendVif1", &PROBE_N, &[(p_chain_1) as u32], false); }
    let r_ = 'f: {
    if (((c.m.r32(sym::gDmaVif1Started) as i32)) != 0) {
        F::Dma_WaitCond(c);
        { let v_ = 1i32; let a_ = sym::gDmaVif1Started; c.m.w32(a_, (v_) as u32); };
    }
    'l1: while ((((c.m.r32(((268450816i32) as u32))) & (((520093699i32) as u32)))) != 0) {
    }
    F::Dma_WaitVu1(c);
    'l2: while ((((c.m.r32(((268447776i32) as u32))) & (((3072i32) as u32)))) != 0) {
    }
    { let v_ = ((0i32) as u32); let a_ = ((268472352i32) as u32); c.m.w32(a_, v_); };
    { let v_ = p_chain_1; let a_ = ((268472368i32) as u32); c.m.w32(a_, v_); };
    { let v_ = ((2i32) as u32); let a_ = ((268492816i32) as u32); c.m.w32(a_, v_); };
    F::FlushCache(c, 0i32);
    { let v_ = ((325i32) as u32); let a_ = F::Port_DmaChcr(c, 1i32); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Dma_SendVif1", &PROBE_N, &[], true); }
    r_
}

pub fn Dma_SendVif1Sync(c: &mut Ctx, mut p_chain_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Dma_SendVif1Sync", &PROBE_N, &[(p_chain_1) as u32], false); }
    let r_ = 'f: {
    F::Dma_WaitCond(c);
    'l1: while ((((c.m.r32(((268450816i32) as u32))) & (((520093699i32) as u32)))) != 0) {
    }
    F::Dma_WaitVu1(c);
    'l2: while ((((c.m.r32(((268447776i32) as u32))) & (((3072i32) as u32)))) != 0) {
    }
    { let v_ = ((0i32) as u32); let a_ = ((268472352i32) as u32); c.m.w32(a_, v_); };
    { let v_ = p_chain_1; let a_ = ((268472368i32) as u32); c.m.w32(a_, v_); };
    { let v_ = ((2i32) as u32); let a_ = ((268492816i32) as u32); c.m.w32(a_, v_); };
    F::FlushCache(c, 0i32);
    { let v_ = ((325i32) as u32); let a_ = F::Port_DmaChcr(c, 1i32); c.m.w32(a_, v_); };
    F::Dma_WaitCond(c);
    'l3: while ((((c.m.r32(((268450816i32) as u32))) & (((520093699i32) as u32)))) != 0) {
    }
    F::Dma_WaitVu1(c);
    'l4: while ((((c.m.r32(((268447776i32) as u32))) & (((3072i32) as u32)))) != 0) {
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Dma_SendVif1Sync", &PROBE_N, &[], true); }
    r_
}

pub fn Dma_SendVif1Spr(c: &mut Ctx, mut p_chain_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Dma_SendVif1Spr", &PROBE_N, &[(p_chain_1) as u32], false); }
    let r_ = 'f: {
    F::Dma_WaitCond(c);
    'l1: while ((((c.m.r32(((268450816i32) as u32))) & (((520093699i32) as u32)))) != 0) {
    }
    F::Dma_WaitVu1(c);
    'l2: while ((((c.m.r32(((268447776i32) as u32))) & (((3072i32) as u32)))) != 0) {
    }
    { let v_ = ((0i32) as u32); let a_ = ((268472352i32) as u32); c.m.w32(a_, v_); };
    { let v_ = ((p_chain_1) | (2147483648u32)); let a_ = ((268472368i32) as u32); c.m.w32(a_, v_); };
    { let v_ = ((2i32) as u32); let a_ = ((268492816i32) as u32); c.m.w32(a_, v_); };
    F::FlushCache(c, 0i32);
    { let v_ = ((325i32) as u32); let a_ = F::Port_DmaChcr(c, 1i32); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Dma_SendVif1Spr", &PROBE_N, &[], true); }
    r_
}

pub fn Dma_SendVif1SprSync(c: &mut Ctx, mut p_chain_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Dma_SendVif1SprSync", &PROBE_N, &[(p_chain_1) as u32], false); }
    let r_ = 'f: {
    F::Dma_WaitCond(c);
    'l1: while ((((c.m.r32(((268450816i32) as u32))) & (((520093699i32) as u32)))) != 0) {
    }
    F::Dma_WaitVu1(c);
    'l2: while ((((c.m.r32(((268447776i32) as u32))) & (((3072i32) as u32)))) != 0) {
    }
    { let v_ = ((0i32) as u32); let a_ = ((268472352i32) as u32); c.m.w32(a_, v_); };
    { let v_ = ((p_chain_1) | (2147483648u32)); let a_ = ((268472368i32) as u32); c.m.w32(a_, v_); };
    { let v_ = ((2i32) as u32); let a_ = ((268492816i32) as u32); c.m.w32(a_, v_); };
    F::FlushCache(c, 0i32);
    { let v_ = ((325i32) as u32); let a_ = F::Port_DmaChcr(c, 1i32); c.m.w32(a_, v_); };
    F::Dma_WaitCond(c);
    'l3: while ((((c.m.r32(((268450816i32) as u32))) & (((520093699i32) as u32)))) != 0) {
    }
    F::Dma_WaitVu1(c);
    'l4: while ((((c.m.r32(((268447776i32) as u32))) & (((3072i32) as u32)))) != 0) {
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Dma_SendVif1SprSync", &PROBE_N, &[], true); }
    r_
}

pub fn Dma_SendGifChain(c: &mut Ctx, mut p_chain_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Dma_SendGifChain", &PROBE_N, &[(p_chain_1) as u32], false); }
    let r_ = 'f: {
    'l1: loop {
        'c2: {
            'l3: while (((({ let a_ = F::Port_DmaChcr(c, 2i32); c.m.r32(a_) }) & (((256i32) as u32)))) != 0) {
            }
        }
        if !((((c.m.r32(((268447776i32) as u32))) & (((3072i32) as u32)))) != 0) { break 'l1; }
    }
    { let v_ = ((0i32) as u32); let a_ = ((268476448i32) as u32); c.m.w32(a_, v_); };
    { let v_ = p_chain_1; let a_ = ((268476464i32) as u32); c.m.w32(a_, v_); };
    { let v_ = ((4i32) as u32); let a_ = ((268492816i32) as u32); c.m.w32(a_, v_); };
    F::FlushCache(c, 0i32);
    { let v_ = ((325i32) as u32); let a_ = F::Port_DmaChcr(c, 2i32); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Dma_SendGifChain", &PROBE_N, &[], true); }
    r_
}

pub fn Dma_SendGifChainSync(c: &mut Ctx, mut p_chain_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Dma_SendGifChainSync", &PROBE_N, &[(p_chain_1) as u32], false); }
    let r_ = 'f: {
    'l1: loop {
        'c2: {
            'l3: while (((({ let a_ = F::Port_DmaChcr(c, 2i32); c.m.r32(a_) }) & (((256i32) as u32)))) != 0) {
            }
        }
        if !((((c.m.r32(((268447776i32) as u32))) & (((3072i32) as u32)))) != 0) { break 'l1; }
    }
    { let v_ = ((0i32) as u32); let a_ = ((268476448i32) as u32); c.m.w32(a_, v_); };
    { let v_ = p_chain_1; let a_ = ((268476464i32) as u32); c.m.w32(a_, v_); };
    { let v_ = ((4i32) as u32); let a_ = ((268492816i32) as u32); c.m.w32(a_, v_); };
    F::FlushCache(c, 0i32);
    { let v_ = ((325i32) as u32); let a_ = F::Port_DmaChcr(c, 2i32); c.m.w32(a_, v_); };
    'l4: loop {
        'c5: {
            'l6: while (((({ let a_ = F::Port_DmaChcr(c, 2i32); c.m.r32(a_) }) & (((256i32) as u32)))) != 0) {
            }
        }
        if !((((c.m.r32(((268447776i32) as u32))) & (((3072i32) as u32)))) != 0) { break 'l4; }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Dma_SendGifChainSync", &PROBE_N, &[], true); }
    r_
}

pub fn Dma_SendGifNoWait(c: &mut Ctx, mut p_addr_1: u32, mut p_qwc_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Dma_SendGifNoWait", &PROBE_N, &[(p_addr_1) as u32, (p_qwc_2) as u32], false); }
    let r_ = 'f: {
    'l1: loop {
        'c2: {
            'l3: while (((({ let a_ = F::Port_DmaChcr(c, 2i32); c.m.r32(a_) }) & (((256i32) as u32)))) != 0) {
            }
        }
        if !((((c.m.r32(((268447776i32) as u32))) & (((3072i32) as u32)))) != 0) { break 'l1; }
    }
    { let v_ = p_qwc_2; let a_ = ((268476448i32) as u32); c.m.w32(a_, v_); };
    { let v_ = p_addr_1; let a_ = ((268476432i32) as u32); c.m.w32(a_, v_); };
    F::FlushCache(c, 0i32);
    { let v_ = ((257i32) as u32); let a_ = F::Port_DmaChcr(c, 2i32); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Dma_SendGifNoWait", &PROBE_N, &[], true); }
    r_
}

pub fn Dma_SendGif(c: &mut Ctx, mut p_addr_1: u32, mut p_qwc_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Dma_SendGif", &PROBE_N, &[(p_addr_1) as u32, (p_qwc_2) as u32], false); }
    let r_ = 'f: {
    'l1: loop {
        'c2: {
            'l3: while (((({ let a_ = F::Port_DmaChcr(c, 2i32); c.m.r32(a_) }) & (((256i32) as u32)))) != 0) {
            }
        }
        if !((((c.m.r32(((268447776i32) as u32))) & (((3072i32) as u32)))) != 0) { break 'l1; }
    }
    { let v_ = p_qwc_2; let a_ = ((268476448i32) as u32); c.m.w32(a_, v_); };
    { let v_ = p_addr_1; let a_ = ((268476432i32) as u32); c.m.w32(a_, v_); };
    F::FlushCache(c, 0i32);
    { let v_ = ((257i32) as u32); let a_ = F::Port_DmaChcr(c, 2i32); c.m.w32(a_, v_); };
    'l4: loop {
        'c5: {
            'l6: while (((({ let a_ = F::Port_DmaChcr(c, 2i32); c.m.r32(a_) }) & (((256i32) as u32)))) != 0) {
            }
        }
        if !((((c.m.r32(((268447776i32) as u32))) & (((3072i32) as u32)))) != 0) { break 'l4; }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Dma_SendGif", &PROBE_N, &[], true); }
    r_
}

pub fn Dma_WaitGif(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Dma_WaitGif", &PROBE_N, &[], false); }
    let r_ = 'f: {
    'l1: loop {
        'c2: {
            'l3: while (((({ let a_ = F::Port_DmaChcr(c, 2i32); c.m.r32(a_) }) & (((256i32) as u32)))) != 0) {
            }
        }
        if !((((c.m.r32(((268447776i32) as u32))) & (((3072i32) as u32)))) != 0) { break 'l1; }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Dma_WaitGif", &PROBE_N, &[], true); }
    r_
}

pub fn Dma_SendToIop(c: &mut Ctx, mut p_iopAddr_1: u32, mut p_data_2: u32, mut p_size_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Dma_SendToIop", &PROBE_N, &[(p_iopAddr_1) as u32, (p_data_2) as u32, (p_size_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_id_4: u32 = 0;
    let r_ = 'f: {
        { let v_ = p_iopAddr_1; let a_ = (fp).wrapping_add(4); c.m.w32(a_, v_); };
        { let v_ = p_data_2; let a_ = fp; c.m.w32(a_, v_); };
        { let v_ = p_size_3; let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (fp).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        F::FlushCache(c, 0i32);
        l_id_4 = F::sceSifSetDma(c, fp, 1i32);
        'l1: while (((((F::sceSifDmaStat(c, l_id_4)) >= (0i32)) as i32)) != 0) {
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Dma_SendToIop", &PROBE_N, &[], true); }
    r_
}

pub fn Dma_AddTexFlush(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Dma_AddTexFlush", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let r_ = 'f: {
        rt::memset(c, fp, 0, 48);
        { let v_ = ((((268435456i32) | (2i32))) as u32); c.m.w32(fp.wrapping_add(0), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(4), v_); }
        { let v_ = ((268435456i32) as u32); c.m.w32(fp.wrapping_add(8), v_); }
        { let v_ = ((((1342177280i32) | (2i32))) as u32); c.m.w32(fp.wrapping_add(12), v_); }
        { let v_ = ((((32768i32) | (1i32))) as u32); c.m.w32(fp.wrapping_add(16), v_); }
        { let v_ = ((268435456i32) as u32); c.m.w32(fp.wrapping_add(20), v_); }
        { let v_ = ((14i32) as u32); c.m.w32(fp.wrapping_add(24), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(28), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(32), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(36), v_); }
        { let v_ = ((63i32) as u32); c.m.w32(fp.wrapping_add(40), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(44), v_); }
        F::Dma_AddData(c, fp, ((48u32) as i32));
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Dma_AddTexFlush", &PROBE_N, &[], true); }
    r_
}

pub fn Dma_AddFrame(c: &mut Ctx, mut p_fbp_1: i32, mut p_fbw_2: i32, mut p_fbmsk_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Dma_AddFrame", &PROBE_N, &[(p_fbp_1) as u32, (p_fbw_2) as u32, (p_fbmsk_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(64) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 64);
    let mut l_frame_4: u64 = 0;
    let r_ = 'f: {
        l_frame_4 = ((((((((p_fbp_1) as u64)) | ((((p_fbw_2) as u64)).wrapping_shl((16i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((p_fbmsk_3) as u64)).wrapping_shl((32i32) as u32)));
        rt::memset(c, fp, 0, 64);
        { let v_ = ((((268435456i32) | (3i32))) as u32); c.m.w32(fp.wrapping_add(0), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(4), v_); }
        { let v_ = ((268435456i32) as u32); c.m.w32(fp.wrapping_add(8), v_); }
        { let v_ = ((((1342177280i32) | (3i32))) as u32); c.m.w32(fp.wrapping_add(12), v_); }
        { let v_ = ((((32768i32) | (2i32))) as u32); c.m.w32(fp.wrapping_add(16), v_); }
        { let v_ = ((268435456i32) as u32); c.m.w32(fp.wrapping_add(20), v_); }
        { let v_ = ((14i32) as u32); c.m.w32(fp.wrapping_add(24), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(28), v_); }
        { let v_ = ((l_frame_4) as u32); c.m.w32(fp.wrapping_add(32), v_); }
        { let v_ = (((l_frame_4).wrapping_shr((32i32) as u32)) as u32); c.m.w32(fp.wrapping_add(36), v_); }
        { let v_ = ((76i32) as u32); c.m.w32(fp.wrapping_add(40), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(44), v_); }
        { let v_ = ((l_frame_4) as u32); c.m.w32(fp.wrapping_add(48), v_); }
        { let v_ = (((l_frame_4).wrapping_shr((32i32) as u32)) as u32); c.m.w32(fp.wrapping_add(52), v_); }
        { let v_ = ((77i32) as u32); c.m.w32(fp.wrapping_add(56), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(60), v_); }
        F::Dma_AddData(c, fp, ((64u32) as i32));
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Dma_AddFrame", &PROBE_N, &[], true); }
    r_
}

pub fn Dma_AddClamp(c: &mut Ctx, mut p_wms_1: i32, mut p_wmt_2: i32, mut p_minu_3: i32, mut p_maxu_4: i32, mut p_minv_5: i32, mut p_maxv_6: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Dma_AddClamp", &PROBE_N, &[(p_wms_1) as u32, (p_wmt_2) as u32, (p_minu_3) as u32, (p_maxu_4) as u32, (p_minv_5) as u32, (p_maxv_6) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(64) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 64);
    let mut l_clamp_7: u64 = 0;
    let r_ = 'f: {
        l_clamp_7 = ((((((((((((p_wms_1) as u64)) | ((((p_wmt_2) as u64)).wrapping_shl((2i32) as u32)))) | ((((p_minu_3) as u64)).wrapping_shl((4i32) as u32)))) | ((((p_maxu_4) as u64)).wrapping_shl((14i32) as u32)))) | ((((p_minv_5) as u64)).wrapping_shl((24i32) as u32)))) | ((((p_maxv_6) as u64)).wrapping_shl((34i32) as u32)));
        rt::memset(c, fp, 0, 64);
        { let v_ = ((((268435456i32) | (3i32))) as u32); c.m.w32(fp.wrapping_add(0), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(4), v_); }
        { let v_ = ((268435456i32) as u32); c.m.w32(fp.wrapping_add(8), v_); }
        { let v_ = ((((1342177280i32) | (3i32))) as u32); c.m.w32(fp.wrapping_add(12), v_); }
        { let v_ = ((((32768i32) | (2i32))) as u32); c.m.w32(fp.wrapping_add(16), v_); }
        { let v_ = ((268435456i32) as u32); c.m.w32(fp.wrapping_add(20), v_); }
        { let v_ = ((14i32) as u32); c.m.w32(fp.wrapping_add(24), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(28), v_); }
        { let v_ = ((l_clamp_7) as u32); c.m.w32(fp.wrapping_add(32), v_); }
        { let v_ = (((l_clamp_7).wrapping_shr((32i32) as u32)) as u32); c.m.w32(fp.wrapping_add(36), v_); }
        { let v_ = ((8i32) as u32); c.m.w32(fp.wrapping_add(40), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(44), v_); }
        { let v_ = ((l_clamp_7) as u32); c.m.w32(fp.wrapping_add(48), v_); }
        { let v_ = (((l_clamp_7).wrapping_shr((32i32) as u32)) as u32); c.m.w32(fp.wrapping_add(52), v_); }
        { let v_ = ((9i32) as u32); c.m.w32(fp.wrapping_add(56), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(60), v_); }
        F::Dma_AddData(c, fp, ((64u32) as i32));
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Dma_AddClamp", &PROBE_N, &[], true); }
    r_
}

pub fn Dma_AddScissor(c: &mut Ctx, mut p_x0_1: i32, mut p_x1_2: i32, mut p_y0_3: i32, mut p_y1_4: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Dma_AddScissor", &PROBE_N, &[(p_x0_1) as u32, (p_x1_2) as u32, (p_y0_3) as u32, (p_y1_4) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(64) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 64);
    let mut l_scissor_5: u64 = 0;
    let r_ = 'f: {
        l_scissor_5 = ((((((((p_x0_1) as u64)) | ((((p_x1_2) as u64)).wrapping_shl((16i32) as u32)))) | ((((p_y0_3) as u64)).wrapping_shl((32i32) as u32)))) | ((((p_y1_4) as u64)).wrapping_shl((48i32) as u32)));
        rt::memset(c, fp, 0, 64);
        { let v_ = ((((268435456i32) | (3i32))) as u32); c.m.w32(fp.wrapping_add(0), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(4), v_); }
        { let v_ = ((268435456i32) as u32); c.m.w32(fp.wrapping_add(8), v_); }
        { let v_ = ((((1342177280i32) | (3i32))) as u32); c.m.w32(fp.wrapping_add(12), v_); }
        { let v_ = ((((32768i32) | (2i32))) as u32); c.m.w32(fp.wrapping_add(16), v_); }
        { let v_ = ((268435456i32) as u32); c.m.w32(fp.wrapping_add(20), v_); }
        { let v_ = ((14i32) as u32); c.m.w32(fp.wrapping_add(24), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(28), v_); }
        { let v_ = ((l_scissor_5) as u32); c.m.w32(fp.wrapping_add(32), v_); }
        { let v_ = (((l_scissor_5).wrapping_shr((32i32) as u32)) as u32); c.m.w32(fp.wrapping_add(36), v_); }
        { let v_ = ((64i32) as u32); c.m.w32(fp.wrapping_add(40), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(44), v_); }
        { let v_ = ((l_scissor_5) as u32); c.m.w32(fp.wrapping_add(48), v_); }
        { let v_ = (((l_scissor_5).wrapping_shr((32i32) as u32)) as u32); c.m.w32(fp.wrapping_add(52), v_); }
        { let v_ = ((65i32) as u32); c.m.w32(fp.wrapping_add(56), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(60), v_); }
        F::Dma_AddData(c, fp, ((64u32) as i32));
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Dma_AddScissor", &PROBE_N, &[], true); }
    r_
}

pub fn Dma_AddXyOffset(c: &mut Ctx, mut p_x_1: i32, mut p_y_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Dma_AddXyOffset", &PROBE_N, &[(p_x_1) as u32, (p_y_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(64) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 64);
    let r_ = 'f: {
        rt::memset(c, fp, 0, 64);
        { let v_ = ((((268435456i32) | (3i32))) as u32); c.m.w32(fp.wrapping_add(0), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(4), v_); }
        { let v_ = ((268435456i32) as u32); c.m.w32(fp.wrapping_add(8), v_); }
        { let v_ = ((((1342177280i32) | (3i32))) as u32); c.m.w32(fp.wrapping_add(12), v_); }
        { let v_ = ((((32768i32) | (2i32))) as u32); c.m.w32(fp.wrapping_add(16), v_); }
        { let v_ = ((268435456i32) as u32); c.m.w32(fp.wrapping_add(20), v_); }
        { let v_ = ((14i32) as u32); c.m.w32(fp.wrapping_add(24), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(28), v_); }
        { let v_ = (((((((p_x_1).wrapping_shl((4i32) as u32)) as u64)) | (((((p_y_2).wrapping_shl((4i32) as u32)) as u64)).wrapping_shl((32i32) as u32)))) as u32); c.m.w32(fp.wrapping_add(32), v_); }
        { let v_ = ((((((((p_x_1).wrapping_shl((4i32) as u32)) as u64)) | (((((p_y_2).wrapping_shl((4i32) as u32)) as u64)).wrapping_shl((32i32) as u32)))).wrapping_shr((32i32) as u32)) as u32); c.m.w32(fp.wrapping_add(36), v_); }
        { let v_ = ((24i32) as u32); c.m.w32(fp.wrapping_add(40), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(44), v_); }
        { let v_ = (((((((p_x_1).wrapping_shl((4i32) as u32)) as u64)) | (((((p_y_2).wrapping_shl((4i32) as u32)) as u64)).wrapping_shl((32i32) as u32)))) as u32); c.m.w32(fp.wrapping_add(48), v_); }
        { let v_ = ((((((((p_x_1).wrapping_shl((4i32) as u32)) as u64)) | (((((p_y_2).wrapping_shl((4i32) as u32)) as u64)).wrapping_shl((32i32) as u32)))).wrapping_shr((32i32) as u32)) as u32); c.m.w32(fp.wrapping_add(52), v_); }
        { let v_ = ((25i32) as u32); c.m.w32(fp.wrapping_add(56), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(60), v_); }
        F::Dma_AddData(c, fp, ((64u32) as i32));
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Dma_AddXyOffset", &PROBE_N, &[], true); }
    r_
}

pub fn Dma_AddZbuf(c: &mut Ctx, mut p_zbp_1: i32, mut p_zmsk_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Dma_AddZbuf", &PROBE_N, &[(p_zbp_1) as u32, (p_zmsk_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let r_ = 'f: {
        rt::memset(c, fp, 0, 48);
        { let v_ = ((((268435456i32) | (2i32))) as u32); c.m.w32(fp.wrapping_add(0), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(4), v_); }
        { let v_ = ((268435456i32) as u32); c.m.w32(fp.wrapping_add(8), v_); }
        { let v_ = ((((1342177280i32) | (2i32))) as u32); c.m.w32(fp.wrapping_add(12), v_); }
        { let v_ = ((((32768i32) | (1i32))) as u32); c.m.w32(fp.wrapping_add(16), v_); }
        { let v_ = ((268435456i32) as u32); c.m.w32(fp.wrapping_add(20), v_); }
        { let v_ = ((14i32) as u32); c.m.w32(fp.wrapping_add(24), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(28), v_); }
        { let v_ = ((((((((p_zbp_1) as u64)) | ((((49i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((p_zmsk_2) as u64)).wrapping_shl((32i32) as u32)))) as u32); c.m.w32(fp.wrapping_add(32), v_); }
        { let v_ = (((((((((p_zbp_1) as u64)) | ((((49i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((p_zmsk_2) as u64)).wrapping_shl((32i32) as u32)))).wrapping_shr((32i32) as u32)) as u32); c.m.w32(fp.wrapping_add(36), v_); }
        { let v_ = ((78i32) as u32); c.m.w32(fp.wrapping_add(40), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(44), v_); }
        F::Dma_AddData(c, fp, ((48u32) as i32));
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Dma_AddZbuf", &PROBE_N, &[], true); }
    r_
}

pub fn Dma_AddTex0(c: &mut Ctx, mut p_tex0_1: u64) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Dma_AddTex0", &PROBE_N, &[(p_tex0_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let r_ = 'f: {
        rt::memset(c, fp, 0, 48);
        { let v_ = ((((268435456i32) | (2i32))) as u32); c.m.w32(fp.wrapping_add(0), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(4), v_); }
        { let v_ = ((268435456i32) as u32); c.m.w32(fp.wrapping_add(8), v_); }
        { let v_ = ((((1342177280i32) | (2i32))) as u32); c.m.w32(fp.wrapping_add(12), v_); }
        { let v_ = ((((32768i32) | (1i32))) as u32); c.m.w32(fp.wrapping_add(16), v_); }
        { let v_ = ((268435456i32) as u32); c.m.w32(fp.wrapping_add(20), v_); }
        { let v_ = ((14i32) as u32); c.m.w32(fp.wrapping_add(24), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(28), v_); }
        { let v_ = ((p_tex0_1) as u32); c.m.w32(fp.wrapping_add(32), v_); }
        { let v_ = (((p_tex0_1).wrapping_shr((32i32) as u32)) as u32); c.m.w32(fp.wrapping_add(36), v_); }
        { let v_ = ((6i32) as u32); c.m.w32(fp.wrapping_add(40), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(44), v_); }
        F::Dma_AddData(c, fp, ((48u32) as i32));
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Dma_AddTex0", &PROBE_N, &[], true); }
    r_
}

pub fn Dma_AddTex1(c: &mut Ctx, mut p_tex1_1: u64) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Dma_AddTex1", &PROBE_N, &[(p_tex1_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let r_ = 'f: {
        rt::memset(c, fp, 0, 48);
        { let v_ = ((((268435456i32) | (2i32))) as u32); c.m.w32(fp.wrapping_add(0), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(4), v_); }
        { let v_ = ((268435456i32) as u32); c.m.w32(fp.wrapping_add(8), v_); }
        { let v_ = ((((1342177280i32) | (2i32))) as u32); c.m.w32(fp.wrapping_add(12), v_); }
        { let v_ = ((((32768i32) | (1i32))) as u32); c.m.w32(fp.wrapping_add(16), v_); }
        { let v_ = ((268435456i32) as u32); c.m.w32(fp.wrapping_add(20), v_); }
        { let v_ = ((14i32) as u32); c.m.w32(fp.wrapping_add(24), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(28), v_); }
        { let v_ = ((p_tex1_1) as u32); c.m.w32(fp.wrapping_add(32), v_); }
        { let v_ = (((p_tex1_1).wrapping_shr((32i32) as u32)) as u32); c.m.w32(fp.wrapping_add(36), v_); }
        { let v_ = ((20i32) as u32); c.m.w32(fp.wrapping_add(40), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(44), v_); }
        F::Dma_AddData(c, fp, ((48u32) as i32));
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Dma_AddTex1", &PROBE_N, &[], true); }
    r_
}

pub fn Dma_AddFba(c: &mut Ctx, mut p_fba_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Dma_AddFba", &PROBE_N, &[(p_fba_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let r_ = 'f: {
        rt::memset(c, fp, 0, 48);
        { let v_ = ((((268435456i32) | (2i32))) as u32); c.m.w32(fp.wrapping_add(0), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(4), v_); }
        { let v_ = ((268435456i32) as u32); c.m.w32(fp.wrapping_add(8), v_); }
        { let v_ = ((((1342177280i32) | (2i32))) as u32); c.m.w32(fp.wrapping_add(12), v_); }
        { let v_ = ((((32768i32) | (1i32))) as u32); c.m.w32(fp.wrapping_add(16), v_); }
        { let v_ = ((268435456i32) as u32); c.m.w32(fp.wrapping_add(20), v_); }
        { let v_ = ((14i32) as u32); c.m.w32(fp.wrapping_add(24), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(28), v_); }
        { let v_ = ((((p_fba_1) as u64)) as u32); c.m.w32(fp.wrapping_add(32), v_); }
        { let v_ = (((((p_fba_1) as u64)).wrapping_shr((32i32) as u32)) as u32); c.m.w32(fp.wrapping_add(36), v_); }
        { let v_ = ((74i32) as u32); c.m.w32(fp.wrapping_add(40), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(44), v_); }
        F::Dma_AddData(c, fp, ((48u32) as i32));
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Dma_AddFba", &PROBE_N, &[], true); }
    r_
}

pub fn Dma_AddColClamp(c: &mut Ctx, mut p_clamp_1: u64) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Dma_AddColClamp", &PROBE_N, &[(p_clamp_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let r_ = 'f: {
        rt::memset(c, fp, 0, 48);
        { let v_ = ((((268435456i32) | (2i32))) as u32); c.m.w32(fp.wrapping_add(0), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(4), v_); }
        { let v_ = ((268435456i32) as u32); c.m.w32(fp.wrapping_add(8), v_); }
        { let v_ = ((((1342177280i32) | (2i32))) as u32); c.m.w32(fp.wrapping_add(12), v_); }
        { let v_ = ((((32768i32) | (1i32))) as u32); c.m.w32(fp.wrapping_add(16), v_); }
        { let v_ = ((268435456i32) as u32); c.m.w32(fp.wrapping_add(20), v_); }
        { let v_ = ((14i32) as u32); c.m.w32(fp.wrapping_add(24), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(28), v_); }
        { let v_ = ((p_clamp_1) as u32); c.m.w32(fp.wrapping_add(32), v_); }
        { let v_ = (((p_clamp_1).wrapping_shr((32i32) as u32)) as u32); c.m.w32(fp.wrapping_add(36), v_); }
        { let v_ = ((70i32) as u32); c.m.w32(fp.wrapping_add(40), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(44), v_); }
        F::Dma_AddData(c, fp, ((48u32) as i32));
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Dma_AddColClamp", &PROBE_N, &[], true); }
    r_
}

pub fn Dma_AddAlpha(c: &mut Ctx, mut p_alpha_1: u64) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Dma_AddAlpha", &PROBE_N, &[(p_alpha_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let r_ = 'f: {
        rt::memset(c, fp, 0, 48);
        { let v_ = ((((268435456i32) | (2i32))) as u32); c.m.w32(fp.wrapping_add(0), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(4), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(8), v_); }
        { let v_ = ((((1342177280i32) | (2i32))) as u32); c.m.w32(fp.wrapping_add(12), v_); }
        { let v_ = ((((32768i32) | (1i32))) as u32); c.m.w32(fp.wrapping_add(16), v_); }
        { let v_ = ((268435456i32) as u32); c.m.w32(fp.wrapping_add(20), v_); }
        { let v_ = ((14i32) as u32); c.m.w32(fp.wrapping_add(24), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(28), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(32), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(36), v_); }
        { let v_ = ((66i32) as u32); c.m.w32(fp.wrapping_add(40), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(44), v_); }
        { let v_ = ((p_alpha_1) as u32); let a_ = (fp).wrapping_add((((8i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = (((p_alpha_1).wrapping_shr((32i32) as u32)) as u32); let a_ = (fp).wrapping_add((((9i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        F::Dma_AddData(c, fp, ((48u32) as i32));
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Dma_AddAlpha", &PROBE_N, &[], true); }
    r_
}

pub fn Dma_Stub101870(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Dma_Stub101870", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Dma_Stub101870", &PROBE_N, &[], true); }
    r_
}

pub fn Dma_AddFillRect(c: &mut Ctx, mut p_x_1: i32, mut p_y_2: i32, mut p_w_3: i32, mut p_h_4: i32, mut p_rgba_5: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Dma_AddFillRect", &PROBE_N, &[(p_x_1) as u32, (p_y_2) as u32, (p_w_3) as u32, (p_h_4) as u32, (p_rgba_5) as u32], false); }
    let mut l_n_6: i32 = 0;
    let mut l_p_7: u32 = 0;
    let mut l_tag_8: u32 = 0;
    let mut l_i_9: i32 = 0;
    let r_ = 'f: {
    l_n_6 = rt::div_i32(p_w_3, 32i32);
    l_p_7 = F::Dma_BeginDirect(c);
    { let v_ = ((((((((5i32) as u64)) | ((((0i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (l_p_7).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((14i32) as u64); let a_ = (l_p_7).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_7).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_7 = n_; };
    { let v_ = ((68i32) as u64); let a_ = (l_p_7).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((66i32) as u64); let a_ = (l_p_7).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_7).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_7 = n_; };
    { let v_ = ((196608i32) as u64); let a_ = (l_p_7).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((71i32) as u64); let a_ = (l_p_7).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_7).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_7 = n_; };
    { let v_ = ((0i32) as u64); let a_ = (l_p_7).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((74i32) as u64); let a_ = (l_p_7).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_7).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_7 = n_; };
    { let v_ = ((70i32) as u64); let a_ = (l_p_7).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((0i32) as u64); let a_ = (l_p_7).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_7).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_7 = n_; };
    { let v_ = ((((p_rgba_5) as u64)) | ((1065353216u64).wrapping_shl((32i32) as u32))); let a_ = (l_p_7).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((1i32) as u64); let a_ = (l_p_7).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_7).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_7 = n_; };
    l_tag_8 = l_p_7;
    { let r_ = 2i32; let n_ = (l_p_7).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_7 = n_; };
    {
        l_i_9 = 0i32;
        'l1: loop {
            if !(((((l_i_9) < (l_n_6)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let v_ = ((((((((p_x_1).wrapping_shl((4i32) as u32)).wrapping_add(((l_i_9).wrapping_mul(32i32)).wrapping_shl((4i32) as u32))) as u64)) | (((((p_y_2).wrapping_shl((4i32) as u32)) as u64)).wrapping_shl((16i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((32i32) as u32))); let a_ = (l_p_7).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                { let v_ = ((((((((p_x_1).wrapping_shl((4i32) as u32)).wrapping_add((((l_i_9).wrapping_mul(32i32)).wrapping_add(32i32)).wrapping_shl((4i32) as u32))) as u64)) | ((((((p_y_2).wrapping_add(p_h_4)).wrapping_shl((4i32) as u32)) as u64)).wrapping_shl((16i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((32i32) as u32))); let a_ = (l_p_7).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                { let r_ = 2i32; let n_ = (l_p_7).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_7 = n_; };
            }
            { let o_ = l_i_9; let n_ = (o_).wrapping_add(1); l_i_9 = n_; };
        }
    }
    { let v_ = ((((((((l_n_6) as u64)) | ((((1i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((2i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (l_tag_8).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((((5i32) | ((5i32).wrapping_shl((4i32) as u32)))) as u64); let a_ = (l_tag_8).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    F::Dma_EndDirect(c, l_p_7);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Dma_AddFillRect", &PROBE_N, &[], true); }
    r_
}

pub fn Dma_PutTexStrips(c: &mut Ctx, mut p_pp_1: u32, mut p_x0_2: i32, mut p_y0_3: i32, mut p_x1_4: i32, mut p_y1_5: i32, mut p_xFrac_6: i32, mut p_yFrac_7: i32, mut p_u0_8: i32, mut p_v0_9: i32, mut p_u1_10: i32, mut p_v1_11: i32, mut p_uFrac_12: i32, mut p_vFrac_13: i32, mut p_rgba_14: u32, mut p_blend_15: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Dma_PutTexStrips", &PROBE_N, &[(p_pp_1) as u32, (p_x0_2) as u32, (p_y0_3) as u32, (p_x1_4) as u32, (p_y1_5) as u32, (p_xFrac_6) as u32, (p_yFrac_7) as u32, (p_u0_8) as u32, (p_v0_9) as u32, (p_u1_10) as u32, (p_v1_11) as u32, (p_uFrac_12) as u32, (p_vFrac_13) as u32, (p_rgba_14) as u32, (p_blend_15) as u32], false); }
    let mut l_step_16: i32 = 0;
    let mut l_n_17: i32 = 0;
    let mut l_w_18: i32 = 0;
    let mut l_i_19: i32 = 0;
    let mut l_dx_20: i32 = 0;
    let mut l_dx_21: F32 = F32(0);
    let r_ = 'f: {
    l_step_16 = 32i32;
    l_n_17 = rt::div_i32((p_u1_10).wrapping_sub(p_u0_8), l_step_16);
    l_w_18 = (p_x1_4).wrapping_sub(p_x0_2);
    { let v_ = ((((((((2i32) as u64)) | ((((0i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (c.m.r32(p_pp_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((14i32) as u64); let a_ = (c.m.r32(p_pp_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let a_ = p_pp_1; let o_ = c.m.r32(a_); let n_ = (o_).wrapping_add(16); c.m.w32(a_, n_); };
    { let v_ = (((if ((p_blend_15) != 0) { 342i32 } else { 278i32 })) as u64); let a_ = (c.m.r32(p_pp_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((0i32) as u64); let a_ = (c.m.r32(p_pp_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let a_ = p_pp_1; let o_ = c.m.r32(a_); let n_ = (o_).wrapping_add(16); c.m.w32(a_, n_); };
    { let v_ = ((((p_rgba_14) as u64)) | ((1065353216u64).wrapping_shl((32i32) as u32))); let a_ = (c.m.r32(p_pp_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((1i32) as u64); let a_ = (c.m.r32(p_pp_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let a_ = p_pp_1; let o_ = c.m.r32(a_); let n_ = (o_).wrapping_add(16); c.m.w32(a_, n_); };
    { let v_ = ((((((((l_n_17) as u64)) | ((((1i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((4i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (c.m.r32(p_pp_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((21331i32) as u64); let a_ = (c.m.r32(p_pp_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let a_ = p_pp_1; let o_ = c.m.r32(a_); let n_ = (o_).wrapping_add(16); c.m.w32(a_, n_); };
    if (((((rt::rem_i32(l_w_18, l_n_17)) == (0i32)) as i32)) != 0) {
        l_dx_20 = rt::div_i32(l_w_18, l_n_17);
        {
            l_i_19 = 0i32;
            'l1: loop {
                if !(((((l_i_19) < (l_n_17)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let v_ = (((((((p_u0_8).wrapping_add((l_i_19).wrapping_mul(l_step_16))).wrapping_shl((4i32) as u32)).wrapping_add(p_uFrac_12)) as u64)) | ((((((p_v0_9).wrapping_shl((4i32) as u32)).wrapping_add(p_vFrac_13)) as u64)).wrapping_shl((16i32) as u32))); let a_ = (c.m.r32(p_pp_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                    { let v_ = ((((((((((p_x0_2).wrapping_add((l_i_19).wrapping_mul(l_dx_20))).wrapping_shl((4i32) as u32)).wrapping_add(p_xFrac_6)).wrapping_add(28672i32)) as u64)) | (((((((p_y0_3).wrapping_add(1824i32)).wrapping_shl((4i32) as u32)).wrapping_add(p_yFrac_7)) as u64)).wrapping_shl((16i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((32i32) as u32))); let a_ = (c.m.r32(p_pp_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                    { let a_ = p_pp_1; let o_ = c.m.r32(a_); let n_ = (o_).wrapping_add(16); c.m.w32(a_, n_); };
                    { let v_ = (((((((p_u0_8).wrapping_add(((l_i_19).wrapping_add(1i32)).wrapping_mul(l_step_16))).wrapping_shl((4i32) as u32)).wrapping_add(p_uFrac_12)) as u64)) | ((((((p_v1_11).wrapping_shl((4i32) as u32)).wrapping_add(p_vFrac_13)) as u64)).wrapping_shl((16i32) as u32))); let a_ = (c.m.r32(p_pp_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                    { let v_ = ((((((((((p_x0_2).wrapping_add(((l_i_19).wrapping_add(1i32)).wrapping_mul(l_dx_20))).wrapping_shl((4i32) as u32)).wrapping_add(p_xFrac_6)).wrapping_add(28672i32)) as u64)) | (((((((p_y1_5).wrapping_add(1824i32)).wrapping_shl((4i32) as u32)).wrapping_add(p_yFrac_7)) as u64)).wrapping_shl((16i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((32i32) as u32))); let a_ = (c.m.r32(p_pp_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                    { let a_ = p_pp_1; let o_ = c.m.r32(a_); let n_ = (o_).wrapping_add(16); c.m.w32(a_, n_); };
                }
                { let o_ = l_i_19; let n_ = (o_).wrapping_add(1); l_i_19 = n_; };
            }
        }
    } else {
        l_dx_21 = rt::fdiv(rt::i2f((l_w_18).wrapping_shl((4i32) as u32)), rt::i2f(l_n_17));
        {
            l_i_19 = 0i32;
            'l3: loop {
                if !(((((l_i_19) < (l_n_17)) as i32)) != 0) { break 'l3; }
                'c4: {
                    { let v_ = (((((((p_u0_8).wrapping_add((l_i_19).wrapping_mul(l_step_16))).wrapping_shl((4i32) as u32)).wrapping_add(p_uFrac_12)) as u64)) | ((((((p_v0_9).wrapping_shl((4i32) as u32)).wrapping_add(p_vFrac_13)) as u64)).wrapping_shl((16i32) as u32))); let a_ = (c.m.r32(p_pp_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                    { let v_ = ((((((((p_x0_2).wrapping_add((p_xFrac_6).wrapping_add(rt::f2i(rt::fmul(l_dx_21, rt::i2f(l_i_19)))))).wrapping_add(28672i32)) as u64)) | (((((((p_y0_3).wrapping_add(1824i32)).wrapping_shl((4i32) as u32)).wrapping_add(p_yFrac_7)) as u64)).wrapping_shl((16i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((32i32) as u32))); let a_ = (c.m.r32(p_pp_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                    { let a_ = p_pp_1; let o_ = c.m.r32(a_); let n_ = (o_).wrapping_add(16); c.m.w32(a_, n_); };
                    { let v_ = (((((((p_u0_8).wrapping_add(((l_i_19).wrapping_add(1i32)).wrapping_mul(l_step_16))).wrapping_shl((4i32) as u32)).wrapping_add(p_uFrac_12)) as u64)) | ((((((p_v1_11).wrapping_shl((4i32) as u32)).wrapping_add(p_vFrac_13)) as u64)).wrapping_shl((16i32) as u32))); let a_ = (c.m.r32(p_pp_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                    { let v_ = ((((((((p_x0_2).wrapping_add((p_xFrac_6).wrapping_add(rt::f2i(rt::fmul(l_dx_21, rt::i2f((l_i_19).wrapping_add(1i32))))))).wrapping_add(28672i32)) as u64)) | (((((((p_y1_5).wrapping_add(1824i32)).wrapping_shl((4i32) as u32)).wrapping_add(p_yFrac_7)) as u64)).wrapping_shl((16i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((32i32) as u32))); let a_ = (c.m.r32(p_pp_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                    { let a_ = p_pp_1; let o_ = c.m.r32(a_); let n_ = (o_).wrapping_add(16); c.m.w32(a_, n_); };
                }
                { let o_ = l_i_19; let n_ = (o_).wrapping_add(1); l_i_19 = n_; };
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Dma_PutTexStrips", &PROBE_N, &[], true); }
    r_
}
