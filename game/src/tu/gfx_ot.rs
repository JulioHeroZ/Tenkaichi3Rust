#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn Ot_Init(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Ot_Init", &PROBE_N, &[], false); }
    let mut l_buf_1: u32 = 0;
    let r_ = 'f: {
    { let v_ = F::Heap_Alloc(c, ((72u32) as i32), ((32i32) as u32), 0i32, 2i32); let a_ = sym::gOt; c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32(sym::gOt); F::memset(c, a0_, 0i32, 72u32) }; };
    { let v_ = F::Ot_Align64(c, 430079i32); let a_ = (c.m.r32(sym::gOt)).wrapping_add(64); c.m.w32(a_, (v_) as u32); };
    { let r_ = 40960i32; let a_ = (c.m.r32(sym::gOt)).wrapping_add(64); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
    l_buf_1 = { let a0_ = { let a0_ = ((((({ let a_ = (c.m.r32(sym::gOt)).wrapping_add(64); (c.m.r32(a_) as i32) }) as u32)).wrapping_add((((((4096i32).wrapping_add(2i32)).wrapping_mul(2i32)) as u32)).wrapping_mul(8u32))) as i32); F::Ot_Align64(c, a0_) }; F::Heap_Alloc(c, a0_, ((32i32) as u32), 0i32, 2i32) };
    { let _ = F::memset(c, l_buf_1, 0i32, ((4i32) as u32)); };
    { let a1_ = { let a_ = (c.m.r32(sym::gOt)).wrapping_add(64); (c.m.r32(a_) as i32) }; F::Ot_SetBuffer(c, l_buf_1, a1_) };
    F::Ot_ResetCursor(c);
    F::Ot_InitLayers(c);
    { let _ = { let a0_ = c.m.r32(sym::gOtFirst); F::Ot_ResetEntries(c, a0_, ((4096i32).wrapping_add(2i32)).wrapping_mul(2i32)) }; };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Ot_Init", &PROBE_N, &[], true); }
    r_
}

pub fn Ot_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Ot_Term", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::Ot_FreeLayers(c);
    { let a0_ = { let a_ = (c.m.r32(sym::gOt)).wrapping_add(56); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
    { let a0_ = c.m.r32(sym::gOt); F::Heap_Free(c, a0_) };
    { let v_ = ((0i32) as u32); let a_ = sym::gOt; c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Ot_Term", &PROBE_N, &[], true); }
    r_
}

pub fn Ot_SetDefaultAlpha(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Ot_SetDefaultAlpha", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let v_ = ((68i32) as u64); let a_ = ((c.m.r32(sym::gOt)).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((72i32) as u64); let a_ = ((c.m.r32(sym::gOt)).wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((66i32) as u64); let a_ = ((c.m.r32(sym::gOt)).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((73i32) as u64); let a_ = ((c.m.r32(sym::gOt)).wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Ot_SetDefaultAlpha", &PROBE_N, &[], true); }
    r_
}

pub fn Ot_ResetCursor(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Ot_ResetCursor", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let v_ = { let a_ = (c.m.r32(sym::gOt)).wrapping_add(56); c.m.r32(a_) }; let a_ = sym::gOtCur; c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Ot_ResetCursor", &PROBE_N, &[], true); }
    r_
}

pub fn Ot_Reset(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Ot_Reset", &PROBE_N, &[], false); }
    let mut l_slots_1: u32 = 0;
    let r_ = 'f: {
    { let v_ = 0i32; let a_ = (c.m.r32(sym::gOt)).wrapping_add(60); c.m.w32(a_, (v_) as u32); };
    l_slots_1 = (c.m.r32(sym::gOt)).wrapping_add(((({ let a_ = (c.m.r32(sym::gOt)).wrapping_add(60); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4));
    { let v_ = c.m.r32((l_slots_1).wrapping_add((((0i32) as u32)).wrapping_mul(4))); let a_ = sym::gOtFirst; c.m.w32(a_, v_); };
    { let v_ = c.m.r32((l_slots_1).wrapping_add((((1i32) as u32)).wrapping_mul(4))); let a_ = sym::gOtZ; c.m.w32(a_, v_); };
    { let v_ = c.m.r32((l_slots_1).wrapping_add((((2i32) as u32)).wrapping_mul(4))); let a_ = sym::gOtLast; c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32(sym::gOtFirst); F::Ot_ResetEntries(c, a0_, ((4096i32).wrapping_add(2i32)).wrapping_mul(2i32)) }; };
    F::Ot_ResetCursor(c);
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Ot_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn Ot_ResetEntries(c: &mut Ctx, mut p_entry_1: u32, mut p_count_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Ot_ResetEntries", &PROBE_N, &[(p_entry_1) as u32, (p_count_2) as u32], false); }
    let mut l_tail_3: u32 = 0;
    let mut l_i_4: i32 = 0;
    let r_ = 'f: {
    l_tail_3 = (p_entry_1).wrapping_add(4);
    {
        l_i_4 = 0i32;
        'l1: loop {
            if !(((((l_i_4) < (p_count_2)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let v_ = c.m.r32(p_entry_1); let a_ = l_tail_3; c.m.w32(a_, v_); };
                { let o_ = p_entry_1; let n_ = (o_).wrapping_add(8); p_entry_1 = n_; };
            }
            { { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; }; { let r_ = 2i32; let n_ = (l_tail_3).wrapping_add((((r_) as u32)).wrapping_mul(4)); l_tail_3 = n_; }; };
        }
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Ot_ResetEntries", &PROBE_N, &[], true); }
    r_
}

pub fn Ot_SetLayerAlpha(c: &mut Ctx, mut p_layer_1: i32, mut p_alpha_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Ot_SetLayerAlpha", &PROBE_N, &[(p_layer_1) as u32, (p_alpha_2) as u32], false); }
    let r_ = 'f: {
    { let v_ = c.m.r64(p_alpha_2); let a_ = ((c.m.r32(sym::gOt)).wrapping_add(16)).wrapping_add((((p_layer_1) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    F::Ot_BuildHeads(c, p_layer_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Ot_SetLayerAlpha", &PROBE_N, &[], true); }
    r_
}

pub fn Ot_Add(c: &mut Ctx, mut p_prim_1: u32, mut p_entry_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Ot_Add", &PROBE_N, &[(p_prim_1) as u32, (p_entry_2) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_prim_1; let a_ = (c.m.r32((p_entry_2).wrapping_add(4))).wrapping_add(4); c.m.w32(a_, v_); };
    { let v_ = p_prim_1; let a_ = (p_entry_2).wrapping_add(4); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Ot_Add", &PROBE_N, &[], true); }
    r_
}

pub fn Ot_AddChain(c: &mut Ctx, mut p_first_1: u32, mut p_last_2: u32, mut p_entry_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Ot_AddChain", &PROBE_N, &[(p_first_1) as u32, (p_last_2) as u32, (p_entry_3) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_first_1; let a_ = (c.m.r32((p_entry_3).wrapping_add(4))).wrapping_add(4); c.m.w32(a_, v_); };
    { let v_ = p_last_2; let a_ = (p_entry_3).wrapping_add(4); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Ot_AddChain", &PROBE_N, &[], true); }
    r_
}

pub fn Ot_Link(c: &mut Ctx, mut p_entry_1: u32, mut p_count_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Ot_Link", &PROBE_N, &[(p_entry_1) as u32, (p_count_2) as u32], false); }
    let mut l_i_3: i32 = 0;
    let r_ = 'f: {
    { let v_ = ((0i32) as u32); let a_ = sym::gOtHead; c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = sym::gOtTail; c.m.w32(a_, v_); };
    {
        l_i_3 = 0i32;
        'l1: loop {
            if !(((((((((l_i_3) < (p_count_2)) as i32)) != 0) && (((((c.m.r32(sym::gOtHead)) == (((0i32) as u32))) as i32)) != 0)) as i32)) != 0) { break 'l1; }
            'c2: {
                if (((((c.m.r32((p_entry_1).wrapping_add(4))) != (c.m.r32(p_entry_1))) as i32)) != 0) {
                    { let v_ = c.m.r32((p_entry_1).wrapping_add(4)); let a_ = sym::gOtTail; c.m.w32(a_, v_); };
                    { let v_ = c.m.r32(p_entry_1); let a_ = sym::gOtHead; c.m.w32(a_, v_); };
                }
            }
            { { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; }; { let o_ = p_entry_1; let n_ = (o_).wrapping_add(8); p_entry_1 = n_; }; };
        }
    }
    {
        'l3: loop {
            if !(((((l_i_3) < (p_count_2)) as i32)) != 0) { break 'l3; }
            'c4: {
                if (((((c.m.r32((p_entry_1).wrapping_add(4))) != (c.m.r32(p_entry_1))) as i32)) != 0) {
                    { let v_ = c.m.r32(p_entry_1); let a_ = (c.m.r32(sym::gOtTail)).wrapping_add(4); c.m.w32(a_, v_); };
                    { let v_ = c.m.r32((p_entry_1).wrapping_add(4)); let a_ = sym::gOtTail; c.m.w32(a_, v_); };
                }
            }
            { { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; }; { let o_ = p_entry_1; let n_ = (o_).wrapping_add(8); p_entry_1 = n_; }; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Ot_Link", &PROBE_N, &[], true); }
    r_
}

pub fn Ot_Draw(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Ot_Draw", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let a0_ = c.m.r32(sym::gOtFirst); F::Ot_Link(c, a0_, ((4096i32).wrapping_add(2i32)).wrapping_mul(2i32)) };
    if ((F::BtlStage_IsReady(c)) != 0) {
        { let _ = F::Ot_Send(c); };
    }
    break 'f F::Ot_Reset(c);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Ot_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn Ot_Send(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Ot_Send", &PROBE_N, &[], false); }
    let mut l_prim_1: u32 = 0;
    let mut l_dst_2: u32 = 0;
    let r_ = 'f: {
    if (((((c.m.r32(sym::gOtTail)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    F::Ot_AddEnv(c, 0i32);
    l_dst_2 = F::Dma_BeginDirect(c);
    {
        l_prim_1 = c.m.r32(sym::gOtHead);
        'l1: loop {
            if !(((((l_prim_1) != (((0i32) as u32))) as i32)) != 0) { break 'l1; }
            'c2: {
                { let _ = { let a2_ = (((((c.m.r16(l_prim_1)) as i32)).wrapping_shl((4i32) as u32)) as u32); F::memcpy(c, l_dst_2, (l_prim_1).wrapping_add((((1i32) as u32)).wrapping_mul(16)), a2_) }; };
                { let r_ = (((c.m.r16(l_prim_1)) as i32)).wrapping_shl((4i32) as u32); let n_ = (l_dst_2).wrapping_add(((r_) as u32)); l_dst_2 = n_; };
            }
            l_prim_1 = c.m.r32((l_prim_1).wrapping_add(4));
        }
    }
    F::Dma_EndDirect(c, l_dst_2);
    F::Ot_AddRestoreEnv(c);
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Ot_Send", &PROBE_N, &[], true); }
    r_
}

pub fn Ot_AddEnv(c: &mut Ctx, mut p_test_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Ot_AddEnv", &PROBE_N, &[(p_test_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(368) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 368);
    let mut l_tst_2: i32 = 0;
    let r_ = 'f: {
        l_tst_2 = ((p_test_1) | (327692i32));
        rt::memset(c, fp, 0, 368);
        { let v_ = ((((268435456i32) | (22i32))) as u32); c.m.w32(fp.wrapping_add(0), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(4), v_); }
        { let v_ = ((268435456i32) as u32); c.m.w32(fp.wrapping_add(8), v_); }
        { let v_ = ((((1342177280i32) | (22i32))) as u32); c.m.w32(fp.wrapping_add(12), v_); }
        { let v_ = ((((32768i32) | (21i32))) as u32); c.m.w32(fp.wrapping_add(16), v_); }
        { let v_ = ((268435456i32) as u32); c.m.w32(fp.wrapping_add(20), v_); }
        { let v_ = ((14i32) as u32); c.m.w32(fp.wrapping_add(24), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(28), v_); }
        { let v_ = (((if ((((!(((((c.m.r32((sym::gGfx).wrapping_add(564)) as i32)) & (1i32))) != 0)) as i32)) != 0) { ((((((((112i32) as u64)) | ((((8i32) as u64)).wrapping_shl((16i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((32i32) as u32))) } else { ((((((((0i32) as u64)) | ((((8i32) as u64)).wrapping_shl((16i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((32i32) as u32))) })) as u32); c.m.w32(fp.wrapping_add(32), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(36), v_); }
        { let v_ = ((76i32) as u32); c.m.w32(fp.wrapping_add(40), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(44), v_); }
        { let v_ = (((if ((((!(((((c.m.r32((sym::gGfx).wrapping_add(564)) as i32)) & (1i32))) != 0)) as i32)) != 0) { ((((((((112i32) as u64)) | ((((8i32) as u64)).wrapping_shl((16i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((32i32) as u32))) } else { ((((((((0i32) as u64)) | ((((8i32) as u64)).wrapping_shl((16i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((32i32) as u32))) })) as u32); c.m.w32(fp.wrapping_add(48), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(52), v_); }
        { let v_ = ((77i32) as u32); c.m.w32(fp.wrapping_add(56), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(60), v_); }
        { let v_ = ((((((((224i32) as u64)) | ((((49i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((32i32) as u32)))) as u32); c.m.w32(fp.wrapping_add(64), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(68), v_); }
        { let v_ = ((78i32) as u32); c.m.w32(fp.wrapping_add(72), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(76), v_); }
        { let v_ = ((((((((224i32) as u64)) | ((((49i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((32i32) as u32)))) as u32); c.m.w32(fp.wrapping_add(80), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(84), v_); }
        { let v_ = ((79i32) as u32); c.m.w32(fp.wrapping_add(88), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(92), v_); }
        { let v_ = ((28672i32) as u32); c.m.w32(fp.wrapping_add(96), v_); }
        { let v_ = ((29184i32) as u32); c.m.w32(fp.wrapping_add(100), v_); }
        { let v_ = ((24i32) as u32); c.m.w32(fp.wrapping_add(104), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(108), v_); }
        { let v_ = ((28672i32) as u32); c.m.w32(fp.wrapping_add(112), v_); }
        { let v_ = ((29184i32) as u32); c.m.w32(fp.wrapping_add(116), v_); }
        { let v_ = ((25i32) as u32); c.m.w32(fp.wrapping_add(120), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(124), v_); }
        { let v_ = (((((((((({ let a_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(512); (c.m.r32(a_) as i32) }) as u64)) | (((({ let a_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(516); (c.m.r32(a_) as i32) }) as u64)).wrapping_shl((16i32) as u32)))) | (((({ let a_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(520); (c.m.r32(a_) as i32) }) as u64)).wrapping_shl((32i32) as u32)))) | (((({ let a_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(524); (c.m.r32(a_) as i32) }) as u64)).wrapping_shl((48i32) as u32)))) as u32); c.m.w32(fp.wrapping_add(128), v_); }
        { let v_ = ((((((((((({ let a_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(512); (c.m.r32(a_) as i32) }) as u64)) | (((({ let a_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(516); (c.m.r32(a_) as i32) }) as u64)).wrapping_shl((16i32) as u32)))) | (((({ let a_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(520); (c.m.r32(a_) as i32) }) as u64)).wrapping_shl((32i32) as u32)))) | (((({ let a_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(524); (c.m.r32(a_) as i32) }) as u64)).wrapping_shl((48i32) as u32)))).wrapping_shr((32i32) as u32)) as u32); c.m.w32(fp.wrapping_add(132), v_); }
        { let v_ = ((64i32) as u32); c.m.w32(fp.wrapping_add(136), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(140), v_); }
        { let v_ = (((((((((({ let a_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(512); (c.m.r32(a_) as i32) }) as u64)) | (((({ let a_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(516); (c.m.r32(a_) as i32) }) as u64)).wrapping_shl((16i32) as u32)))) | (((({ let a_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(520); (c.m.r32(a_) as i32) }) as u64)).wrapping_shl((32i32) as u32)))) | (((({ let a_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(524); (c.m.r32(a_) as i32) }) as u64)).wrapping_shl((48i32) as u32)))) as u32); c.m.w32(fp.wrapping_add(144), v_); }
        { let v_ = ((((((((((({ let a_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(512); (c.m.r32(a_) as i32) }) as u64)) | (((({ let a_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(516); (c.m.r32(a_) as i32) }) as u64)).wrapping_shl((16i32) as u32)))) | (((({ let a_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(520); (c.m.r32(a_) as i32) }) as u64)).wrapping_shl((32i32) as u32)))) | (((({ let a_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(524); (c.m.r32(a_) as i32) }) as u64)).wrapping_shl((48i32) as u32)))).wrapping_shr((32i32) as u32)) as u32); c.m.w32(fp.wrapping_add(148), v_); }
        { let v_ = ((65i32) as u32); c.m.w32(fp.wrapping_add(152), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(156), v_); }
        { let v_ = ((68i32) as u32); c.m.w32(fp.wrapping_add(160), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(164), v_); }
        { let v_ = ((66i32) as u32); c.m.w32(fp.wrapping_add(168), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(172), v_); }
        { let v_ = ((68i32) as u32); c.m.w32(fp.wrapping_add(176), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(180), v_); }
        { let v_ = ((67i32) as u32); c.m.w32(fp.wrapping_add(184), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(188), v_); }
        { let v_ = ((((((((224i32) as u64)) | ((((49i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((32i32) as u32)))) as u32); c.m.w32(fp.wrapping_add(192), v_); }
        { let v_ = ((1i32) as u32); c.m.w32(fp.wrapping_add(196), v_); }
        { let v_ = ((78i32) as u32); c.m.w32(fp.wrapping_add(200), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(204), v_); }
        { let v_ = ((((((((224i32) as u64)) | ((((49i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((32i32) as u32)))) as u32); c.m.w32(fp.wrapping_add(208), v_); }
        { let v_ = ((1i32) as u32); c.m.w32(fp.wrapping_add(212), v_); }
        { let v_ = ((79i32) as u32); c.m.w32(fp.wrapping_add(216), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(220), v_); }
        { let v_ = ((l_tst_2) as u32); c.m.w32(fp.wrapping_add(224), v_); }
        { let v_ = (((((l_tst_2) as u64)).wrapping_shr((32i32) as u32)) as u32); c.m.w32(fp.wrapping_add(228), v_); }
        { let v_ = ((71i32) as u32); c.m.w32(fp.wrapping_add(232), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(236), v_); }
        { let v_ = ((l_tst_2) as u32); c.m.w32(fp.wrapping_add(240), v_); }
        { let v_ = (((((l_tst_2) as u64)).wrapping_shr((32i32) as u32)) as u32); c.m.w32(fp.wrapping_add(244), v_); }
        { let v_ = ((72i32) as u32); c.m.w32(fp.wrapping_add(248), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(252), v_); }
        { let v_ = ((96i32) as u32); c.m.w32(fp.wrapping_add(256), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(260), v_); }
        { let v_ = ((20i32) as u32); c.m.w32(fp.wrapping_add(264), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(268), v_); }
        { let v_ = ((96i32) as u32); c.m.w32(fp.wrapping_add(272), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(276), v_); }
        { let v_ = (((20i32).wrapping_add(1i32)) as u32); c.m.w32(fp.wrapping_add(280), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(284), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(288), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(292), v_); }
        { let v_ = ((8i32) as u32); c.m.w32(fp.wrapping_add(296), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(300), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(304), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(308), v_); }
        { let v_ = ((9i32) as u32); c.m.w32(fp.wrapping_add(312), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(316), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(320), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(324), v_); }
        { let v_ = ((74i32) as u32); c.m.w32(fp.wrapping_add(328), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(332), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(336), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(340), v_); }
        { let v_ = ((75i32) as u32); c.m.w32(fp.wrapping_add(344), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(348), v_); }
        { let v_ = ((1i32) as u32); c.m.w32(fp.wrapping_add(352), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(356), v_); }
        { let v_ = ((70i32) as u32); c.m.w32(fp.wrapping_add(360), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(364), v_); }
        F::Dma_AddData(c, fp, ((368u32) as i32));
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Ot_AddEnv", &PROBE_N, &[], true); }
    r_
}

pub fn Ot_AddRestoreEnv(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Ot_AddRestoreEnv", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(144) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 144);
    let r_ = 'f: {
        rt::memset(c, fp, 0, 144);
        { let v_ = ((((268435456i32) | (8i32))) as u32); c.m.w32(fp.wrapping_add(0), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(4), v_); }
        { let v_ = ((268435456i32) as u32); c.m.w32(fp.wrapping_add(8), v_); }
        { let v_ = ((((1342177280i32) | (8i32))) as u32); c.m.w32(fp.wrapping_add(12), v_); }
        { let v_ = ((((32768i32) | (7i32))) as u32); c.m.w32(fp.wrapping_add(16), v_); }
        { let v_ = ((268435456i32) as u32); c.m.w32(fp.wrapping_add(20), v_); }
        { let v_ = ((14i32) as u32); c.m.w32(fp.wrapping_add(24), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(28), v_); }
        { let v_ = ((68i32) as u32); c.m.w32(fp.wrapping_add(32), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(36), v_); }
        { let v_ = ((66i32) as u32); c.m.w32(fp.wrapping_add(40), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(44), v_); }
        { let v_ = ((68i32) as u32); c.m.w32(fp.wrapping_add(48), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(52), v_); }
        { let v_ = ((67i32) as u32); c.m.w32(fp.wrapping_add(56), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(60), v_); }
        { let v_ = ((((((((224i32) as u64)) | ((((49i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((32i32) as u32)))) as u32); c.m.w32(fp.wrapping_add(64), v_); }
        { let v_ = ((1i32) as u32); c.m.w32(fp.wrapping_add(68), v_); }
        { let v_ = ((78i32) as u32); c.m.w32(fp.wrapping_add(72), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(76), v_); }
        { let v_ = ((458752i32) as u32); c.m.w32(fp.wrapping_add(80), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(84), v_); }
        { let v_ = ((71i32) as u32); c.m.w32(fp.wrapping_add(88), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(92), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(96), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(100), v_); }
        { let v_ = ((20i32) as u32); c.m.w32(fp.wrapping_add(104), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(108), v_); }
        { let v_ = ((1i32) as u32); c.m.w32(fp.wrapping_add(112), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(116), v_); }
        { let v_ = ((70i32) as u32); c.m.w32(fp.wrapping_add(120), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(124), v_); }
        { let v_ = ((5i32) as u32); c.m.w32(fp.wrapping_add(128), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(132), v_); }
        { let v_ = ((8i32) as u32); c.m.w32(fp.wrapping_add(136), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(140), v_); }
        F::Dma_AddData(c, fp, ((144u32) as i32));
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Ot_AddRestoreEnv", &PROBE_N, &[], true); }
    r_
}

pub fn Ot_Stub102C60(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Ot_Stub102C60", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Ot_Stub102C60", &PROBE_N, &[], true); }
    r_
}

pub fn Ot_Stub102C68(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Ot_Stub102C68", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Ot_Stub102C68", &PROBE_N, &[], true); }
    r_
}

pub fn Ot_GetCapacity(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Ot_GetCapacity", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if (((((c.m.r32(sym::gOtCur)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if (((((c.m.r32(sym::gOt)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((({ let a_ = (c.m.r32(sym::gOt)).wrapping_add(56); c.m.r32(a_) }) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    break 'f ({ let a_ = (c.m.r32(sym::gOt)).wrapping_add(64); (c.m.r32(a_) as i32) }).wrapping_sub(16i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Ot_GetCapacity", &PROBE_N, &[], true); }
    r_
}

pub fn Ot_SetBuffer(c: &mut Ctx, mut p_buf_1: u32, mut p_size_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Ot_SetBuffer", &PROBE_N, &[(p_buf_1) as u32, (p_size_2) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_buf_1; let a_ = (c.m.r32(sym::gOt)).wrapping_add(56); c.m.w32(a_, v_); };
    { let v_ = (p_buf_1).wrapping_add(((p_size_2) as u32)); let a_ = (c.m.r32(sym::gOt)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = (c.m.r32(sym::gOt)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add((((1i32) as u32)).wrapping_mul(16)); let a_ = (c.m.r32(sym::gOt)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = (c.m.r32(sym::gOt)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add((((4096i32) as u32)).wrapping_mul(16)); let a_ = (c.m.r32(sym::gOt)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = { let a_ = (c.m.r32(sym::gOt)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }; let a_ = sym::gOtFirst; c.m.w32(a_, v_); };
    { let v_ = { let a_ = (c.m.r32(sym::gOt)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }; let a_ = sym::gOtZ; c.m.w32(a_, v_); };
    { let v_ = { let a_ = (c.m.r32(sym::gOt)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }; let a_ = sym::gOtLast; c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Ot_SetBuffer", &PROBE_N, &[], true); }
    r_
}

pub fn Ot_InitLayers(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Ot_InitLayers", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let r_ = 'f: {
    F::Ot_SetDefaultAlpha(c);
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (2i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let v_ = F::Heap_Alloc(c, ((4096i32).wrapping_add(2i32)).wrapping_mul(64i32), ((32i32) as u32), 0i32, 2i32); let a_ = ((c.m.r32(sym::gOt)).wrapping_add(48)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                { let _ = { let a0_ = { let a_ = ((c.m.r32(sym::gOt)).wrapping_add(48)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }; F::memset(c, a0_, 0i32, ((((4096i32).wrapping_add(2i32)).wrapping_mul(64i32)) as u32)) }; };
                F::Ot_BuildHeads(c, l_i_1);
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Ot_InitLayers", &PROBE_N, &[], true); }
    r_
}

pub fn Ot_FreeLayers(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Ot_FreeLayers", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let r_ = 'f: {
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (2i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((((({ let a_ = ((c.m.r32(sym::gOt)).wrapping_add(48)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
                    { let a0_ = { let a_ = ((c.m.r32(sym::gOt)).wrapping_add(48)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
                    { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gOt)).wrapping_add(48)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                }
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Ot_FreeLayers", &PROBE_N, &[], true); }
    r_
}

pub fn Ot_NewHead(c: &mut Ctx, mut p_layer_1: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Ot_NewHead", &PROBE_N, &[(p_layer_1) as u32], false); }
    let mut l_p_2: u32 = 0;
    let r_ = 'f: {
    l_p_2 = c.m.r32(sym::gOtCur);
    { let v_ = (l_p_2).wrapping_add((((rt::div_i32(64i32, 4i32)) as u32)).wrapping_mul(4)); let a_ = sym::gOtCur; c.m.w32(a_, v_); };
    { let v_ = ((((536870912i32) | (3i32))) as u32); let a_ = (l_p_2).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((268435456i32) as u32); let a_ = (l_p_2).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((((1342177280i32) | (3i32))) as u32); let a_ = (l_p_2).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ((((((((2i32) as u64)) | ((((1i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (l_p_2).wrapping_add((((2i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((14i32) as u64); let a_ = (l_p_2).wrapping_add((((3i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((66i32) as u64); let a_ = (l_p_2).wrapping_add((((5i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((67i32) as u64); let a_ = (l_p_2).wrapping_add((((7i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (l_p_2).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = { let a_ = ((c.m.r32(sym::gOt)).wrapping_add(16)).wrapping_add((((p_layer_1) as u32)).wrapping_mul(8)); c.m.r64(a_) }; let a_ = (l_p_2).wrapping_add((((4i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = { let a_ = ((c.m.r32(sym::gOt)).wrapping_add(32)).wrapping_add((((p_layer_1) as u32)).wrapping_mul(8)); c.m.r64(a_) }; let a_ = (l_p_2).wrapping_add((((6i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    break 'f l_p_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Ot_NewHead", &PROBE_N, &[], true); }
    r_
}

pub fn Ot_BuildHeads(c: &mut Ctx, mut p_layer_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Ot_BuildHeads", &PROBE_N, &[(p_layer_1) as u32], false); }
    let mut l_saved_2: u32 = 0;
    let mut l_slot_3: u32 = 0;
    let mut l_i_4: i32 = 0;
    let r_ = 'f: {
    l_saved_2 = c.m.r32(sym::gOtCur);
    { let v_ = { let a_ = ((c.m.r32(sym::gOt)).wrapping_add(48)).wrapping_add((((p_layer_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }; let a_ = sym::gOtCur; c.m.w32(a_, v_); };
    l_slot_3 = { let a_ = (c.m.r32(sym::gOt)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) };
    {
        l_i_4 = 0i32;
        'l1: loop {
            if !(((((l_i_4) < ((4096i32).wrapping_add(2i32))) as i32)) != 0) { break 'l1; }
            'c2: {
                { let v_ = F::Ot_NewHead(c, p_layer_1); let a_ = (l_slot_3).wrapping_add((((p_layer_1) as u32)).wrapping_mul(8)); c.m.w32(a_, v_); };
                { let o_ = l_slot_3; let n_ = (o_).wrapping_add(16); l_slot_3 = n_; };
            }
            { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
        }
    }
    { let v_ = l_saved_2; let a_ = sym::gOtCur; c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Ot_BuildHeads", &PROBE_N, &[], true); }
    r_
}

pub fn Ot_Align64(c: &mut Ctx, mut p_size_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Ot_Align64", &PROBE_N, &[(p_size_1) as u32], false); }
    let r_ = 'f: {
    if ((((p_size_1) & (63i32))) != 0) {
        p_size_1 = ((rt::div_i32(p_size_1, 64i32)).wrapping_mul(64i32)).wrapping_add(64i32);
    }
    break 'f p_size_1;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Ot_Align64", &PROBE_N, &[], true); }
    r_
}
