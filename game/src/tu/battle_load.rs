#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn BtlJob_GetPool(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlJob_GetPool", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f sym::gBtlJobPool;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlJob_GetPool", &PROBE_N, &[], true); }
    r_
}

pub fn BtlJob_InitPool(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlJob_InitPool", &PROBE_N, &[], false); }
    let mut l_pool_1: u32 = 0;
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
    l_pool_1 = F::BtlJob_GetPool(c);
    { let _ = F::memset(c, l_pool_1, 0i32, 524u32); };
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < (8i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                F::SList_PushFront(c, (l_pool_1).wrapping_add(512), (l_pool_1).wrapping_add((((l_i_2) as u32)).wrapping_mul(64)));
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlJob_InitPool", &PROBE_N, &[], true); }
    r_
}

pub fn BtlJob_Alloc(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlJob_Alloc", &PROBE_N, &[], false); }
    let mut l_job_1: u32 = 0;
    let r_ = 'f: {
    l_job_1 = { let a0_ = (F::BtlJob_GetPool(c)).wrapping_add(512); F::SList_PopFront(c, a0_) };
    if (((((l_job_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f ((0i32) as u32);
    }
    { let _ = F::memset(c, l_job_1, 0i32, 64u32); };
    break 'f l_job_1;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlJob_Alloc", &PROBE_N, &[], true); }
    r_
}

pub fn BtlJob_Free(c: &mut Ctx, mut p_job_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlJob_Free", &PROBE_N, &[(p_job_1) as u32], false); }
    let mut l_pool_2: u32 = 0;
    let r_ = 'f: {
    l_pool_2 = F::BtlJob_GetPool(c);
    if (((((p_job_1) != (((0i32) as u32))) as i32)) != 0) {
        F::SList_PushFront(c, (l_pool_2).wrapping_add(512), p_job_1);
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlJob_Free", &PROBE_N, &[], true); }
    r_
}

pub fn BtlJob_GetFreeCount(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlJob_GetFreeCount", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f { let a0_ = (F::BtlJob_GetPool(c)).wrapping_add(512); F::SList_GetCount(c, a0_) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlJob_GetFreeCount", &PROBE_N, &[], true); }
    r_
}

pub fn BtlLoad_BeginStageSwap(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlLoad_BeginStageSwap", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::EftStage_ResetAll(c);
    F::BtlStage_Term(c);
    F::StgFx_SetDisabled(c, 1i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlLoad_BeginStageSwap", &PROBE_N, &[], true); }
    r_
}

pub fn BtlLoad_EndStageSwap(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlLoad_EndStageSwap", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::StgModel_InitStage(c);
    F::BtlChars_OnStageLoaded(c);
    F::StgNav_Rebind(c);
    F::EftStage_Recreate(c);
    F::StgFx_Reset(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlLoad_EndStageSwap", &PROBE_N, &[], true); }
    r_
}

pub fn BtlLoad_StepStageReload(c: &mut Ctx, mut p_job_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlLoad_StepStageReload", &PROBE_N, &[(p_job_1) as u32], false); }
    let mut l_res_2: u32 = 0;
    let mut l_id_3: i32 = 0;
    let r_ = 'f: {
    l_res_2 = (c.m.r32(sym::gCommonRes)).wrapping_add(((32i32) as u32));
    's1: {
        let sv4: i32 = (c.m.r32((p_job_1).wrapping_add(8)) as i32);
        match sv4 {
            0i32 => {
                { let r_ = ((8192i32) as u64); let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
                { let a_ = (p_job_1).wrapping_add(8); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            1i32 => {
                F::BtlLoad_BeginStageSwap(c);
                if (((((F::Battle_GetStage(c)) >= (36i32)) as i32)) != 0) {
                    l_id_3 = (if (((((F::Battle_GetStage(c)) >= (36i32)) as i32)) != 0) { ((3400i32).wrapping_add(70000i32)).wrapping_add((F::Battle_GetStage(c)).wrapping_sub(36i32)) } else { (F::Battle_GetStage(c)).wrapping_add(369i32) });
                } else {
                    if ((F::Battle_IsSplitScreen(c)) != 0) {
                        l_id_3 = (F::Battle_GetStage(c)).wrapping_add(408i32);
                    } else {
                        l_id_3 = (F::Battle_GetStage(c)).wrapping_add(369i32);
                    }
                }
                { let v_ = { let a1_ = c.m.r32((l_res_2).wrapping_add(4)); let a2_ = (c.m.r32((l_res_2).wrapping_add(44)) as i32); ((D::call(c, fid::File_Request, &[((l_id_3) as i64 as u64), ((a1_) as u64), ((a2_) as i64 as u64)])) as u32) }; let a_ = (l_res_2).wrapping_add(4); c.m.w32(a_, v_); };
                { let v_ = { let a0_ = ((if (((((F::Battle_GetStage(c)) >= (36i32)) as i32)) != 0) { 0i32 } else { F::Battle_GetStage(c) })).wrapping_add(334i32); let a1_ = c.m.r32((l_res_2).wrapping_add(12)); let a2_ = (c.m.r32((l_res_2).wrapping_add(52)) as i32); ((D::call(c, fid::File_Request, &[((a0_) as i64 as u64), ((a1_) as u64), ((a2_) as i64 as u64)])) as u32) }; let a_ = (l_res_2).wrapping_add(12); c.m.w32(a_, v_); };
                { let a_ = (p_job_1).wrapping_add(8); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 'f 0i32;
            }
            2i32 => {
                if ((((!((F::File_UpdateRequests(c)) != 0)) as i32)) != 0) {
                    break 'f 0i32;
                }
                { let a1_ = c.m.r32((l_res_2).wrapping_add(12)); F::Snd_ReloadBank(c, ((((8i32) as i64 as u64)) as u32), a1_, 0i32) };
                { let a_ = (p_job_1).wrapping_add(8); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            3i32 => {
                if (((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0) {
                    break 'f 0i32;
                }
                { let r_ = (((!(8192i32))) as u64); let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); let n_ = ((c.m.r64(a_)) & (r_)); c.m.w64(a_, n_); };
                F::BtlLoad_EndStageSwap(c);
                { let _ = F::BtlJob_Free(c, p_job_1); };
                break 'f 1i32;
            }
            _ => {
                break 'f 1i32;
            }
        }
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlLoad_StepStageReload", &PROBE_N, &[], true); }
    r_
}

pub fn BtlLoad_StepStageChange(c: &mut Ctx, mut p_job_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlLoad_StepStageChange", &PROBE_N, &[(p_job_1) as u32], false); }
    let mut l_res_2: u32 = 0;
    let r_ = 'f: {
    l_res_2 = (c.m.r32(sym::gCommonRes)).wrapping_add(((32i32) as u32));
    's1: {
        let sv3: i32 = (c.m.r32((p_job_1).wrapping_add(8)) as i32);
        'k12: {
        'k11: {
        'k10: {
        'k9: {
        'k8: {
        'k7: {
        'k6: {
        'k5: {
        'k4: {
        'k3: {
        'k2: {
            match sv3 { 0i32 => break 'k2, 1i32 => break 'k3, 2i32 => break 'k4, 3i32 => break 'k5, 4i32 => break 'k6, 5i32 => break 'k7, 6i32 => break 'k8, 7i32 => break 'k9, 8i32 => break 'k10, 9i32 => break 'k11, _ => break 'k12, }
        }
        { let r_ = ((8192i32) as u64); let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
        { let a_ = (p_job_1).wrapping_add(8); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        break 's1;
        }
        if ((((((c.m.r32((p_job_1).wrapping_add(12)) as i32)) == (3i32)) as i32)) != 0) {
            { let v_ = ((D::call(c, fid::File_Request, &[((447i32) as i64 as u64), ((((0i32) as u32)) as u64), ((0i32) as i64 as u64)])) as u32); let a_ = (l_res_2).wrapping_add(20); c.m.w32(a_, v_); };
        } else {
            { let v_ = ((D::call(c, fid::File_Request, &[((448i32) as i64 as u64), ((((0i32) as u32)) as u64), ((0i32) as i64 as u64)])) as u32); let a_ = (l_res_2).wrapping_add(20); c.m.w32(a_, v_); };
        }
        F::BtlLoad_BeginStageSwap(c);
        { let a_ = (p_job_1).wrapping_add(8); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        break 's1;
        }
        if ((((!((F::File_UpdateRequests(c)) != 0)) as i32)) != 0) {
            break 'f 0i32;
        }
        { let a_ = (p_job_1).wrapping_add(8); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        break 's1;
        }
        if (((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0) {
            break 'f 0i32;
        }
        F::EftBurst_Start(c);
        { let a_ = (p_job_1).wrapping_add(8); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        break 'f 0i32;
        }
        { let v_ = { let a0_ = (if (((((F::Battle_GetStage(c)) >= (36i32)) as i32)) != 0) { ((3400i32).wrapping_add(70000i32)).wrapping_add((F::Battle_GetStage(c)).wrapping_sub(36i32)) } else { (F::Battle_GetStage(c)).wrapping_add(369i32) }); let a1_ = c.m.r32((l_res_2).wrapping_add(4)); let a2_ = (c.m.r32((l_res_2).wrapping_add(44)) as i32); ((D::call(c, fid::File_Request, &[((a0_) as i64 as u64), ((a1_) as u64), ((a2_) as i64 as u64)])) as u32) }; let a_ = (l_res_2).wrapping_add(4); c.m.w32(a_, v_); };
        { let v_ = { let a0_ = ((if (((((F::Battle_GetStage(c)) >= (36i32)) as i32)) != 0) { 0i32 } else { F::Battle_GetStage(c) })).wrapping_add(334i32); let a1_ = c.m.r32((l_res_2).wrapping_add(12)); let a2_ = (c.m.r32((l_res_2).wrapping_add(52)) as i32); ((D::call(c, fid::File_Request, &[((a0_) as i64 as u64), ((a1_) as u64), ((a2_) as i64 as u64)])) as u32) }; let a_ = (l_res_2).wrapping_add(12); c.m.w32(a_, v_); };
        { let a_ = (p_job_1).wrapping_add(8); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        break 'f 0i32;
        }
        if ((((!((F::File_UpdateRequests(c)) != 0)) as i32)) != 0) {
            break 'f 0i32;
        }
        { let a1_ = c.m.r32((l_res_2).wrapping_add(12)); F::Snd_ReloadBank(c, ((((8i32) as i64 as u64)) as u32), a1_, 0i32) };
        { let a_ = (p_job_1).wrapping_add(8); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        }
        if (((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0) {
            break 'f 0i32;
        }
        if ((F::EftBurst_IsBusy(c)) != 0) {
            break 'f 0i32;
        }
        F::Fade_Start(c, 2i32, 0i32, F32(0x3f800000));
        { let a_ = (p_job_1).wrapping_add(8); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        break 's1;
        }
        if ((F::Fade_IsDone(c, 2i32)) != 0) {
            F::EftBurst_End(c);
            { let a_ = (p_job_1).wrapping_add(8); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        }
        break 's1;
        }
        if (((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0) {
            break 'f 0i32;
        }
        { let r_ = (((!(8192i32))) as u64); let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); let n_ = ((c.m.r64(a_)) & (r_)); c.m.w64(a_, n_); };
        F::BtlLoad_EndStageSwap(c);
        if (((((c.m.r32((l_res_2).wrapping_add(20))) != (((0i32) as u32))) as i32)) != 0) {
            { let a0_ = c.m.r32((l_res_2).wrapping_add(20)); F::Heap_Free(c, a0_) };
            { let v_ = ((0i32) as u32); let a_ = (l_res_2).wrapping_add(20); c.m.w32(a_, v_); };
        }
        { let a_ = (p_job_1).wrapping_add(8); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        break 's1;
        }
        F::Fade_Start(c, 2i32, 1i32, F32(0x3f800000));
        { let _ = F::BtlJob_Free(c, p_job_1); };
        break 'f 1i32;
        }
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlLoad_StepStageChange", &PROBE_N, &[], true); }
    r_
}

pub fn BtlLoad_StepObject(c: &mut Ctx, mut p_job_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlLoad_StepObject", &PROBE_N, &[(p_job_1) as u32], false); }
    let mut l_id_2: i32 = 0;
    let mut l_ready_3: i32 = 0;
    let r_ = 'f: {
    's1: {
        let sv4: i32 = (c.m.r32((p_job_1).wrapping_add(8)) as i32);
        match sv4 {
            0i32 => {
                { let r_ = ((4096i32) as u64); let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
                l_id_2 = ((c.m.r32((p_job_1).wrapping_add(20)) as i32)).wrapping_add(3100i32);
                if ((((((c.m.r32((p_job_1).wrapping_add(20)) as i32)) < (256i32)) as i32)) != 0) {
                    if ((((((c.m.r32((p_job_1).wrapping_add(28)) as i32)) != (0i32)) as i32)) != 0) {
                        l_id_2 = ((((c.m.r32((p_job_1).wrapping_add(20)) as i32)).wrapping_mul(10i32)).wrapping_add((c.m.r32((p_job_1).wrapping_add(24)) as i32))).wrapping_add((1424i32).wrapping_add(4i32));
                    } else {
                        l_id_2 = ((((c.m.r32((p_job_1).wrapping_add(20)) as i32)).wrapping_mul(10i32)).wrapping_add((c.m.r32((p_job_1).wrapping_add(24)) as i32))).wrapping_add(1424i32);
                    }
                }
                { let v_ = F::BtlRes_Request(c, 0i32, l_id_2, (1i32).wrapping_neg(), (1i32).wrapping_neg()); let a_ = sym::gBtlLoadHandle; c.m.w32(a_, (v_) as u32); };
                { let a_ = (p_job_1).wrapping_add(8); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 'f 0i32;
            }
            1i32 => {
                if ((((!((F::File_UpdateRequests(c)) != 0)) as i32)) != 0) {
                    break 'f 0i32;
                }
                's2: {
                    let sv5: i32 = (c.m.r32((p_job_1).wrapping_add(12)) as i32);
                    match sv5 {
                        0i32 => {
                            break 's2;
                        }
                        2i32 => {
                            F::BtlChange_NotifyLoaded(c);
                            break 's2;
                        }
                        _ => {
                        }
                    }
                }
                { let a_ = (p_job_1).wrapping_add(8); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            2i32 => {
                l_ready_3 = 1i32;
                if (((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0) {
                    break 'f 0i32;
                }
                's3: {
                    let sv6: i32 = (c.m.r32((p_job_1).wrapping_add(12)) as i32);
                    match sv6 {
                        0i32 => {
                            break 's3;
                        }
                        2i32 => {
                            l_ready_3 = F::BtlChange_IsReady(c);
                            break 's3;
                        }
                        _ => {
                        }
                    }
                }
                if ((((!((l_ready_3) != 0)) as i32)) != 0) {
                    break 'f 0i32;
                }
                's4: {
                    let sv7: i32 = (c.m.r32((p_job_1).wrapping_add(12)) as i32);
                    match sv7 {
                        0i32 => {
                            { let v_ = { let a1_ = { let a0_ = (c.m.r32(sym::gBtlLoadHandle) as i32); ((((F::BtlRes_GetSlot(c, a0_)) as u64)) as i32) }; F::BtlObj_Create(c, 2i32, ((((a1_) as i64 as u64)) as u32), 1i32) }; let a_ = sym::gBtlLoadObj; c.m.w32(a_, (v_) as u32); };
                            { let a0_ = { let a0_ = (c.m.r32(sym::gBtlLoadObj) as i32); ((((F::BtlObj_Get(c, a0_)) as u64)) as i32) }; F::BtlObjAnim_PlayModel(c, ((((a0_) as i64 as u64)) as u32), 0i32, 2i32) };
                            break 's4;
                        }
                        2i32 => {
                            { let v_ = { let a0_ = (c.m.r32((p_job_1).wrapping_add(36)) as i32); let a1_ = { let a0_ = (c.m.r32(sym::gBtlLoadHandle) as i32); ((((F::BtlRes_GetSlot(c, a0_)) as u64)) as i32) }; F::BtlObj_Create(c, a0_, ((((a1_) as i64 as u64)) as u32), 1i32) }; let a_ = sym::gBtlLoadObj; c.m.w32(a_, (v_) as u32); };
                            { let a0_ = (c.m.r32((p_job_1).wrapping_add(16)) as i32); let a1_ = (c.m.r32(sym::gBtlLoadHandle) as i32); let a2_ = (c.m.r32(sym::gBtlLoadObj) as i32); F::BtlCtrl_AttachPartner(c, a0_, a1_, a2_) };
                            break 's4;
                        }
                        _ => {
                        }
                    }
                }
                { let r_ = (((!(4096i32))) as u64); let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); let n_ = ((c.m.r64(a_)) & (r_)); c.m.w64(a_, n_); };
                { let _ = F::BtlJob_Free(c, p_job_1); };
                break 'f 1i32;
            }
            _ => {
                break 'f 1i32;
            }
        }
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlLoad_StepObject", &PROBE_N, &[], true); }
    r_
}

pub fn BtlLoad_StepChara(c: &mut Ctx, mut p_job_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlLoad_StepChara", &PROBE_N, &[(p_job_1) as u32], false); }
    let mut l_res_2: u32 = 0;
    let mut l_model_3: i32 = 0;
    let mut l_voice_4: i32 = 0;
    let mut l_file8_5: i32 = 0;
    let mut l_file9_6: i32 = 0;
    let mut l_member_7: i32 = 0;
    let mut l_ready_8: i32 = 0;
    let mut l_m_9: u32 = 0;
    let mut l_next_10: u32 = 0;
    let r_ = 'f: {
    l_res_2 = (c.m.r32(sym::gCommonRes)).wrapping_add(((32i32) as u32));
    's1: {
        let sv11: i32 = (c.m.r32((p_job_1).wrapping_add(8)) as i32);
        match sv11 {
            0i32 => {
                if ((((((c.m.r32((p_job_1).wrapping_add(40)) as i32)) != (0i32)) as i32)) != 0) {
                    l_model_3 = ((((c.m.r32((p_job_1).wrapping_add(20)) as i32)).wrapping_mul(10i32)).wrapping_add((c.m.r32((p_job_1).wrapping_add(36)) as i32))).wrapping_add((1424i32).wrapping_add(4i32));
                } else {
                    l_model_3 = ((((c.m.r32((p_job_1).wrapping_add(20)) as i32)).wrapping_mul(10i32)).wrapping_add((c.m.r32((p_job_1).wrapping_add(36)) as i32))).wrapping_add(1424i32);
                }
                if ((((((c.m.r32((p_job_1).wrapping_add(44)) as i32)) == (0i32)) as i32)) != 0) {
                    l_file8_5 = (((c.m.r32((p_job_1).wrapping_add(24)) as i32)).wrapping_mul(10i32)).wrapping_add((1424i32).wrapping_add(8i32));
                    l_file9_6 = (((c.m.r32((p_job_1).wrapping_add(28)) as i32)).wrapping_mul(10i32)).wrapping_add((1424i32).wrapping_add(9i32));
                    l_voice_4 = ((c.m.r32((p_job_1).wrapping_add(32)) as i32)).wrapping_add((if (((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5640); (c.m.r32(a_) as i32) }) & (1i32))) != 0) { 3195i32 } else { 3034i32 }));
                    { let a0_ = { let a0_ = (c.m.r32((p_job_1).wrapping_add(16)) as i32); F::BattleSide_GetModelSlot(c, a0_) }; F::BtlRes_Reload(c, a0_, l_model_3, l_file8_5, l_file9_6) };
                    { let v_ = { let a1_ = c.m.r32((l_res_2).wrapping_add(12)); let a2_ = (c.m.r32((l_res_2).wrapping_add(52)) as i32); ((D::call(c, fid::File_Request, &[((l_voice_4) as i64 as u64), ((a1_) as u64), ((a2_) as i64 as u64)])) as u32) }; let a_ = (l_res_2).wrapping_add(12); c.m.w32(a_, v_); };
                    if ((((!(({ let a0_ = (c.m.r32((p_job_1).wrapping_add(16)) as i32); F::BtlCtrl_IsSwitching(c, a0_) }) != 0)) as i32)) != 0) {
                        l_member_7 = 0i32;
                        if ((((((c.m.r32((p_job_1).wrapping_add(48)) as i32)) == (0i32)) as i32)) != 0) {
                            l_member_7 = { let a0_ = (c.m.r32((p_job_1).wrapping_add(16)) as i32); F::BtlSide_GetActiveMember(c, a0_) };
                        }
                        l_next_10 = { let a0_ = (c.m.r32((p_job_1).wrapping_add(16)) as i32); F::BattleSide_GetMember(c, a0_, l_member_7) };
                        { let v_ = { let a0_ = ((((c.m.r32((p_job_1).wrapping_add(20)) as i32)).wrapping_mul(2i32)).wrapping_add((c.m.r32((p_job_1).wrapping_add(16)) as i32))).wrapping_add(8i32); let a1_ = c.m.r32(((l_next_10).wrapping_add(92)).wrapping_add((((1i32) as u32)).wrapping_mul(4))); ((D::call(c, fid::File_Request, &[((a0_) as i64 as u64), ((a1_) as u64), ((4096i32) as i64 as u64)])) as u32) }; let a_ = ((l_next_10).wrapping_add(92)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    }
                } else {
                    { let a0_ = { let a0_ = (c.m.r32((p_job_1).wrapping_add(16)) as i32); F::BattleSide_GetModelSlot(c, a0_) }; F::BtlRes_Reload(c, a0_, l_model_3, (1i32).wrapping_neg(), (1i32).wrapping_neg()) };
                }
                { let r_ = ((2048i32) as u64); let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
                { let a_ = (p_job_1).wrapping_add(8); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            1i32 => {
                if ((((!((F::File_UpdateRequests(c)) != 0)) as i32)) != 0) {
                    break 'f 0i32;
                }
                's2: {
                    let sv12: i32 = (c.m.r32((p_job_1).wrapping_add(12)) as i32);
                    match sv12 {
                        0i32 => {
                            break 's2;
                        }
                        1i32 => {
                            F::BtlChange_NotifyLoaded(c);
                            break 's2;
                        }
                        _ => {
                        }
                    }
                }
                { let a_ = (p_job_1).wrapping_add(8); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            2i32 => {
                l_ready_8 = 1i32;
                's3: {
                    let sv13: i32 = (c.m.r32((p_job_1).wrapping_add(12)) as i32);
                    match sv13 {
                        0i32 => {
                            break 's3;
                        }
                        1i32 => {
                            l_ready_8 = F::BtlChange_IsReady(c);
                            break 's3;
                        }
                        _ => {
                        }
                    }
                }
                if ((((!((l_ready_8) != 0)) as i32)) != 0) {
                    break 's1;
                }
                { let a_ = (p_job_1).wrapping_add(8); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                F::BtlRes_CommitReload2(c);
                if ((((((c.m.r32((p_job_1).wrapping_add(44)) as i32)) == (0i32)) as i32)) != 0) {
                    if ((((((c.m.r32((p_job_1).wrapping_add(16)) as i32)) == (0i32)) as i32)) != 0) {
                        { let a1_ = c.m.r32((l_res_2).wrapping_add(12)); F::Snd_ReloadBank(c, ((((16i32) as i64 as u64)) as u32), a1_, 0i32) };
                    } else {
                        { let a1_ = c.m.r32((l_res_2).wrapping_add(12)); F::Snd_ReloadBank(c, ((((32i32) as i64 as u64)) as u32), a1_, 0i32) };
                    }
                }
                { let a0_ = { let a0_ = (c.m.r32((p_job_1).wrapping_add(16)) as i32); F::BattleSide_GetObjId(c, a0_) }; let a1_ = { let a0_ = (c.m.r32((p_job_1).wrapping_add(16)) as i32); F::BattleSide_GetModelSlot(c, a0_) }; F::BtlObj_Rebind(c, a0_, a1_) };
                's4: {
                    let sv14: i32 = (c.m.r32((p_job_1).wrapping_add(12)) as i32);
                    match sv14 {
                        0i32 => {
                            { let a0_ = { let a0_ = { let a0_ = (c.m.r32((p_job_1).wrapping_add(16)) as i32); F::BattleSide_GetObjId(c, a0_) }; ((((F::BtlObj_Get(c, a0_)) as u64)) as i32) }; F::BtlObjAnim_PlayAuto(c, ((((a0_) as i64 as u64)) as u32), 0i32, 2i32) };
                            break 's4;
                        }
                        1i32 => {
                            { let a0_ = (c.m.r32((p_job_1).wrapping_add(16)) as i32); F::BtlChars_OnModelLoaded(c, a0_) };
                            break 's4;
                        }
                        _ => {
                        }
                    }
                }
                if ((((((c.m.r32((p_job_1).wrapping_add(44)) as i32)) == (0i32)) as i32)) != 0) {
                    { let a0_ = (c.m.r32((p_job_1).wrapping_add(16)) as i32); F::BtlAiMgr_ResetSide(c, a0_) };
                    { let a0_ = (c.m.r32((p_job_1).wrapping_add(16)) as i32); F::BtlScene_CreateChar(c, a0_) };
                    if ((((!(({ let a0_ = (c.m.r32((p_job_1).wrapping_add(16)) as i32); F::BtlCtrl_IsSwitching(c, a0_) }) != 0)) as i32)) != 0) {
                        l_m_9 = { let a0_ = (c.m.r32((p_job_1).wrapping_add(16)) as i32); let a1_ = { let a0_ = (c.m.r32((p_job_1).wrapping_add(16)) as i32); F::BtlSide_GetActiveMember(c, a0_) }; F::BattleSide_GetMember(c, a0_, a1_) };
                        if (((((l_m_9) != (((0i32) as u32))) as i32)) != 0) {
                            { let a1_ = c.m.r32(((l_m_9).wrapping_add(92)).wrapping_add((((1i32) as u32)).wrapping_mul(4))); let a2_ = c.m.r32(((l_m_9).wrapping_add(92)).wrapping_add((((1i32) as u32)).wrapping_mul(4))); F::Res_RelocateOffsets(c, ((l_m_9).wrapping_add(92)).wrapping_add((((1i32) as u32)).wrapping_mul(4)), a1_, a2_) };
                            { let v_ = c.m.r32(((l_m_9).wrapping_add(92)).wrapping_add((((1i32) as u32)).wrapping_mul(4))); let a_ = (l_m_9).wrapping_add(88); c.m.w32(a_, v_); };
                        }
                    }
                }
                { let _ = F::BtlJob_Free(c, p_job_1); };
                { let r_ = (((!(2048i32))) as u64); let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); let n_ = ((c.m.r64(a_)) & (r_)); c.m.w64(a_, n_); };
                break 'f 1i32;
            }
            _ => {
                break 'f 1i32;
            }
        }
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlLoad_StepChara", &PROBE_N, &[], true); }
    r_
}

pub fn BtlLoad_StepInitial(c: &mut Ctx, mut p_job_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlLoad_StepInitial", &PROBE_N, &[(p_job_1) as u32], false); }
    let mut l_res_2: u32 = 0;
    let mut l_m_3: u32 = 0;
    let mut l_ev_4: u32 = 0;
    let mut l_side_5: i32 = 0;
    let mut l_i_6: i32 = 0;
    let mut l_id_7: i32 = 0;
    let r_ = 'f: {
    l_res_2 = (c.m.r32(sym::gCommonRes)).wrapping_add(((32i32) as u32));
    's1: {
        let sv8: i32 = (c.m.r32((p_job_1).wrapping_add(8)) as i32);
        'k10: {
        'k9: {
        'k8: {
        'k7: {
        'k6: {
        'k5: {
        'k4: {
        'k3: {
        'k2: {
            match sv8 { 0i32 => break 'k2, 1i32 => break 'k3, 2i32 => break 'k4, 3i32 => break 'k5, 4i32 => break 'k6, 5i32 => break 'k7, 90i32 => break 'k8, 91i32 => break 'k9, _ => break 'k10, }
        }
        { let v_ = ((D::call(c, fid::File_Request, &[((330i32) as i64 as u64), ((((0i32) as u32)) as u64), ((0i32) as i64 as u64)])) as u32); let a_ = (l_res_2).wrapping_add(24); c.m.w32(a_, v_); };
        { let v_ = { let a0_ = ((if (((((F::Battle_GetStartStage(c)) >= (36i32)) as i32)) != 0) { 0i32 } else { F::Battle_GetStartStage(c) })).wrapping_add(334i32); ((D::call(c, fid::File_Request, &[((a0_) as i64 as u64), ((((0i32) as u32)) as u64), ((0i32) as i64 as u64)])) as u32) }; let a_ = (l_res_2).wrapping_add(28); c.m.w32(a_, v_); };
        { let v_ = { let a0_ = (F::BattleSide_GetStartChara(c, 0i32)).wrapping_add((if (((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5640); (c.m.r32(a_) as i32) }) & (1i32))) != 0) { 3195i32 } else { 3034i32 })); ((D::call(c, fid::File_Request, &[((a0_) as i64 as u64), ((((0i32) as u32)) as u64), ((0i32) as i64 as u64)])) as u32) }; let a_ = ((l_res_2).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = { let a0_ = (F::BattleSide_GetStartChara(c, 1i32)).wrapping_add((if (((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5640); (c.m.r32(a_) as i32) }) & (1i32))) != 0) { 3195i32 } else { 3034i32 })); ((D::call(c, fid::File_Request, &[((a0_) as i64 as u64), ((((0i32) as u32)) as u64), ((0i32) as i64 as u64)])) as u32) }; let a_ = ((l_res_2).wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let a_ = (p_job_1).wrapping_add(8); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        break 'f 0i32;
        }
        if ((((!((F::File_UpdateRequests(c)) != 0)) as i32)) != 0) {
            break 'f 0i32;
        }
        { let a_ = (p_job_1).wrapping_add(8); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        }
        { let a1_ = c.m.r32((l_res_2).wrapping_add(24)); F::Snd_LoadBank(c, ((((4i32) as i64 as u64)) as u32), a1_, 0i32) };
        { let a1_ = c.m.r32((l_res_2).wrapping_add(28)); F::Snd_LoadBank(c, ((((8i32) as i64 as u64)) as u32), a1_, 0i32) };
        { let a1_ = c.m.r32(((l_res_2).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(4))); F::Snd_LoadBank(c, ((((16i32) as i64 as u64)) as u32), a1_, 0i32) };
        { let a1_ = c.m.r32(((l_res_2).wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(4))); F::Snd_LoadBank(c, ((((32i32) as i64 as u64)) as u32), a1_, 0i32) };
        { let a_ = (p_job_1).wrapping_add(8); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        break 's1;
        }
        if ((((!((F::Snd_IsUploadDone(c)) != 0)) as i32)) != 0) {
            break 's1;
        }
        { let a0_ = c.m.r32((l_res_2).wrapping_add(24)); F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = (l_res_2).wrapping_add(24); c.m.w32(a_, v_); };
        { let a0_ = c.m.r32((l_res_2).wrapping_add(28)); F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = (l_res_2).wrapping_add(28); c.m.w32(a_, v_); };
        { let a0_ = c.m.r32(((l_res_2).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(4))); F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = ((l_res_2).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let a0_ = c.m.r32(((l_res_2).wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(4))); F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = ((l_res_2).wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let a_ = (p_job_1).wrapping_add(8); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        break 's1;
        }
        { let a1_ = { let a1_ = F::BattleSide_GetStartChara(c, 0i32); let a2_ = F::BattleSide_GetStartCostume(c, 0i32); let a3_ = F::BattleSide_GetStartVariant(c, 0i32); F::BtlObj_RequestCharaModel(c, 0i32, a1_, a2_, a3_) }; F::BattleSide_SetModelSlot(c, 0i32, a1_) };
        { let a1_ = { let a1_ = F::BattleSide_GetStartChara(c, 1i32); let a2_ = F::BattleSide_GetStartCostume(c, 1i32); let a3_ = F::BattleSide_GetStartVariant(c, 1i32); F::BtlObj_RequestCharaModel(c, 1i32, a1_, a2_, a3_) }; F::BattleSide_SetModelSlot(c, 1i32, a1_) };
        {
            { let v_ = F::Port_StageBufSize(c); let a_ = (l_res_2).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
        }
        {
            { let v_ = ((0i32) as u32); let a_ = (l_res_2).wrapping_add(4); c.m.w32(a_, v_); };
            if ((((((c.m.r32((l_res_2).wrapping_add(44)) as i32)) > (7124992i32)) as i32)) != 0) {
                { let v_ = { let a0_ = (c.m.r32((l_res_2).wrapping_add(44)) as i32); F::Port_GameBigAlloc(c, a0_) }; let a_ = (l_res_2).wrapping_add(4); c.m.w32(a_, v_); };
            }
            if (((((c.m.r32((l_res_2).wrapping_add(4))) == (((0i32) as u32))) as i32)) != 0) {
                { let v_ = { let a0_ = (c.m.r32((l_res_2).wrapping_add(44)) as i32); F::Heap_Alloc(c, a0_, ((64i32) as u32), 0i32, 2i32) }; let a_ = (l_res_2).wrapping_add(4); c.m.w32(a_, v_); };
            }
        }
        { let _ = { let a0_ = c.m.r32((l_res_2).wrapping_add(4)); let a2_ = (((c.m.r32((l_res_2).wrapping_add(44)) as i32)) as u32); F::memset(c, a0_, 0i32, a2_) }; };
        { let v_ = 591872i32; let a_ = (l_res_2).wrapping_add(52); c.m.w32(a_, (v_) as u32); };
        { let v_ = { let a0_ = (c.m.r32((l_res_2).wrapping_add(52)) as i32); F::Heap_Alloc(c, a0_, ((64i32) as u32), 0i32, 2i32) }; let a_ = (l_res_2).wrapping_add(12); c.m.w32(a_, v_); };
        { let _ = { let a0_ = c.m.r32((l_res_2).wrapping_add(12)); let a2_ = (((c.m.r32((l_res_2).wrapping_add(52)) as i32)) as u32); F::memset(c, a0_, 0i32, a2_) }; };
        {
            l_side_5 = 0i32;
            'l11: loop {
                if !(((((l_side_5) < (2i32)) as i32)) != 0) { break 'l11; }
                'c12: {
                    {
                        l_i_6 = 0i32;
                        'l13: loop {
                            if !(((((l_i_6) < (5i32)) as i32)) != 0) { break 'l13; }
                            'c14: {
                                l_m_3 = F::BattleSide_GetMember(c, l_side_5, l_i_6);
                                { let v_ = F::Heap_Alloc(c, 4096i32, ((64i32) as u32), 0i32, 2i32); let a_ = ((l_m_3).wrapping_add(92)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                                { let v_ = F::Heap_Alloc(c, 4096i32, ((64i32) as u32), 0i32, 2i32); let a_ = ((l_m_3).wrapping_add(92)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                                { let v_ = c.m.r32(((l_m_3).wrapping_add(92)).wrapping_add((((0i32) as u32)).wrapping_mul(4))); let a_ = (l_m_3).wrapping_add(88); c.m.w32(a_, v_); };
                                { let _ = { let a0_ = c.m.r32(((l_m_3).wrapping_add(92)).wrapping_add((((0i32) as u32)).wrapping_mul(4))); F::memset(c, a0_, 0i32, ((4096i32) as u32)) }; };
                                { let _ = { let a0_ = c.m.r32(((l_m_3).wrapping_add(92)).wrapping_add((((1i32) as u32)).wrapping_mul(4))); F::memset(c, a0_, 0i32, ((4096i32) as u32)) }; };
                            }
                            { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; };
                        }
                    }
                }
                { let o_ = l_side_5; let n_ = (o_).wrapping_add(1); l_side_5 = n_; };
            }
        }
        if (((((F::Battle_GetStartStage(c)) >= (36i32)) as i32)) != 0) {
            l_id_7 = (if (((((F::Battle_GetStartStage(c)) >= (36i32)) as i32)) != 0) { ((3400i32).wrapping_add(70000i32)).wrapping_add((F::Battle_GetStartStage(c)).wrapping_sub(36i32)) } else { (F::Battle_GetStartStage(c)).wrapping_add(369i32) });
        } else {
            if ((F::Battle_IsSplitScreen(c)) != 0) {
                l_id_7 = (F::Battle_GetStartStage(c)).wrapping_add(408i32);
            } else {
                l_id_7 = (F::Battle_GetStartStage(c)).wrapping_add(369i32);
            }
        }
        { let v_ = { let a1_ = c.m.r32((l_res_2).wrapping_add(4)); let a2_ = (c.m.r32((l_res_2).wrapping_add(44)) as i32); ((D::call(c, fid::File_Request, &[((l_id_7) as i64 as u64), ((a1_) as u64), ((a2_) as i64 as u64)])) as u32) }; let a_ = (l_res_2).wrapping_add(4); c.m.w32(a_, v_); };
        {
            l_side_5 = 0i32;
            'l15: loop {
                if !(((((l_side_5) < (2i32)) as i32)) != 0) { break 'l15; }
                'c16: {
                    {
                        l_i_6 = 0i32;
                        'l17: loop {
                            if !(((((((l_i_6) as u32)) < (((F::BattleSide_GetMemberCount(c, l_side_5)) as u32))) as i32)) != 0) { break 'l17; }
                            'c18: {
                                l_m_3 = F::BattleSide_GetMember(c, l_side_5, l_i_6);
                                { let v_ = { let a0_ = ((((c.m.r32(l_m_3) as i32)).wrapping_mul(2i32)).wrapping_add(l_side_5)).wrapping_add(8i32); let a1_ = c.m.r32(((l_m_3).wrapping_add(92)).wrapping_add((((0i32) as u32)).wrapping_mul(4))); ((D::call(c, fid::File_Request, &[((a0_) as i64 as u64), ((a1_) as u64), ((4096i32) as i64 as u64)])) as u32) }; let a_ = ((l_m_3).wrapping_add(92)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                            }
                            { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; };
                        }
                    }
                }
                { let o_ = l_side_5; let n_ = (o_).wrapping_add(1); l_side_5 = n_; };
            }
        }
        { let v_ = { let a0_ = ({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(6i32); ((D::call(c, fid::File_Request, &[((a0_) as i64 as u64), ((((0i32) as u32)) as u64), ((0i32) as i64 as u64)])) as u32) }; let a_ = (l_res_2).wrapping_add(8); c.m.w32(a_, v_); };
        { let a_ = (p_job_1).wrapping_add(8); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        break 's1;
        }
        if ((((!((F::File_UpdateRequests(c)) != 0)) as i32)) != 0) {
            break 'f 0i32;
        }
        {
            l_side_5 = 0i32;
            'l19: loop {
                if !(((((l_side_5) < (2i32)) as i32)) != 0) { break 'l19; }
                'c20: {
                    {
                        l_i_6 = 0i32;
                        'l21: loop {
                            if !(((((((l_i_6) as u32)) < (((F::BattleSide_GetMemberCount(c, l_side_5)) as u32))) as i32)) != 0) { break 'l21; }
                            'c22: {
                                l_m_3 = F::BattleSide_GetMember(c, l_side_5, l_i_6);
                                { let a1_ = c.m.r32(((l_m_3).wrapping_add(92)).wrapping_add((((0i32) as u32)).wrapping_mul(4))); let a2_ = c.m.r32(((l_m_3).wrapping_add(92)).wrapping_add((((0i32) as u32)).wrapping_mul(4))); F::Res_RelocateOffsets(c, ((l_m_3).wrapping_add(92)).wrapping_add((((0i32) as u32)).wrapping_mul(4)), a1_, a2_) };
                            }
                            { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; };
                        }
                    }
                }
                { let o_ = l_side_5; let n_ = (o_).wrapping_add(1); l_side_5 = n_; };
            }
        }
        { let _ = F::BtlJob_Free(c, p_job_1); };
        break 'f 1i32;
        }
        { let v_ = { let a0_ = ((c.m.r32((p_job_1).wrapping_add(12)) as i32)).wrapping_add(511i32); ((D::call(c, fid::File_Request, &[((a0_) as i64 as u64), ((((0i32) as u32)) as u64), ((0i32) as i64 as u64)])) as u32) }; let a_ = (l_res_2).wrapping_add(16); c.m.w32(a_, v_); };
        { let a_ = (p_job_1).wrapping_add(8); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        break 'f 0i32;
        }
        if ((((!((F::File_UpdateRequests(c)) != 0)) as i32)) != 0) {
            break 'f 0i32;
        }
        l_ev_4 = F::Battle_GetEventWork(c);
        { let v_ = { let a0_ = c.m.r32((l_res_2).wrapping_add(16)); F::Gsc_LoadFile(c, a0_) }; let a_ = l_ev_4; c.m.w32(a_, (v_) as u32); };
        { let a0_ = (c.m.r32(l_ev_4) as i32); F::Gsc_RunAction(c, a0_, 1000i32) };
        { let a0_ = (c.m.r32(l_ev_4) as i32); F::BtlScript_ScanEvents(c, a0_) };
        F::BtlScript_Nop(c);
        F::BattleSetup_Finish(c);
        { let v_ = 0i32; let a_ = (p_job_1).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        break 's1;
        }
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlLoad_StepInitial", &PROBE_N, &[], true); }
    r_
}

pub fn BtlLoad_RequestStageReload(c: &mut Ctx, mut p_stage_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlLoad_RequestStageReload", &PROBE_N, &[(p_stage_1) as u32], false); }
    let mut l_job_2: u32 = 0;
    let r_ = 'f: {
    if (((((((((((((((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((2048i32) as u64)))) != 0) || (((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((4096i32) as u64)))) != 0)) as i32)) != 0) || (((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((8192i32) as u64)))) != 0)) as i32)) != 0) || (((((F::Battle_GetStage(c)) == (p_stage_1)) as i32)) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    l_job_2 = F::BtlJob_Alloc(c);
    { let _ = F::memset(c, l_job_2, 0i32, 64u32); };
    { let v_ = fid::BtlLoad_StepStageReload; let a_ = (l_job_2).wrapping_add(4); c.m.w32(a_, v_); };
    { let v_ = 0i32; let a_ = (l_job_2).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
    { let v_ = p_stage_1; let a_ = (l_job_2).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
    F::Battle_SetStage(c, p_stage_1);
    { let _ = F::Job_Push(c, l_job_2); };
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlLoad_RequestStageReload", &PROBE_N, &[], true); }
    r_
}

pub fn BtlLoad_RequestStageChange(c: &mut Ctx, mut p_stage_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlLoad_RequestStageChange", &PROBE_N, &[(p_stage_1) as u32], false); }
    let mut l_job_2: u32 = 0;
    let r_ = 'f: {
    if (((((((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((2048i32) as u64)))) != 0) || (((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((4096i32) as u64)))) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    if (((((((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((8192i32) as u64)))) != 0) || (((((((((p_stage_1) >= (0i32)) as i32)) != 0) && (((((F::Battle_GetStage(c)) == (p_stage_1)) as i32)) != 0)) as i32)) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    if (((((p_stage_1) < (0i32)) as i32)) != 0) {
        break 'f 0i32;
    }
    l_job_2 = F::BtlJob_Alloc(c);
    { let _ = F::memset(c, l_job_2, 0i32, 64u32); };
    { let v_ = fid::BtlLoad_StepStageChange; let a_ = (l_job_2).wrapping_add(4); c.m.w32(a_, v_); };
    { let v_ = 0i32; let a_ = (l_job_2).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
    { let v_ = p_stage_1; let a_ = (l_job_2).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
    F::Battle_SetStage(c, p_stage_1);
    { let _ = F::Job_Push(c, l_job_2); };
    F::Fade_Start(c, 2i32, 1i32, F32(0x3f800000));
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlLoad_RequestStageChange", &PROBE_N, &[], true); }
    r_
}

pub fn BtlLoad_PollObjectRequest(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlLoad_PollObjectRequest", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_side_1: i32 = 0;
    let mut l_job_2: u32 = 0;
    let r_ = 'f: {
        {
            l_side_1 = 0i32;
            'l1: loop {
                if !(((((l_side_1) < (2i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    if ((F::BtlChange_IsPendingType1(c, l_side_1)) != 0) {
                        F::BtlChange_GetArgs(c, fp, fp.wrapping_add(4), fp.wrapping_add(8), ((0i32) as u32), ((0i32) as u32), ((0i32) as u32), fp.wrapping_add(12));
                        l_job_2 = F::BtlJob_Alloc(c);
                        { let _ = F::memset(c, l_job_2, 0i32, 64u32); };
                        { let v_ = fid::BtlLoad_StepObject; let a_ = (l_job_2).wrapping_add(4); c.m.w32(a_, v_); };
                        { let v_ = l_side_1; let a_ = (l_job_2).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
                        { let v_ = 2i32; let a_ = (l_job_2).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                        { let v_ = (c.m.r32(fp.wrapping_add(12)) as i32); let a_ = (l_job_2).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                        { let v_ = (c.m.r32(fp) as i32); let a_ = (l_job_2).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
                        { let v_ = (c.m.r32(fp.wrapping_add(4)) as i32); let a_ = (l_job_2).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                        { let v_ = (c.m.r32(fp.wrapping_add(8)) as i32); let a_ = (l_job_2).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                        { let _ = F::Job_Push(c, l_job_2); };
                        F::BtlChange_NotifyTaken(c);
                    }
                }
                { let o_ = l_side_1; let n_ = (o_).wrapping_add(1); l_side_1 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlLoad_PollObjectRequest", &PROBE_N, &[], true); }
    r_
}

pub fn BtlLoad_PollCharaRequest(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlLoad_PollCharaRequest", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_side_1: i32 = 0;
    let mut l_modelOnly_2: i32 = 0;
    let mut l_job_3: u32 = 0;
    let r_ = 'f: {
        {
            l_side_1 = 0i32;
            'l1: loop {
                if !(((((l_side_1) < (2i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    if ((F::BtlChange_IsPendingType0(c, l_side_1)) != 0) {
                        F::BtlChange_GetArgs(c, fp, fp.wrapping_add(4), fp.wrapping_add(8), fp.wrapping_add(12), fp.wrapping_add(16), fp.wrapping_add(20), ((0i32) as u32));
                        if ((((((((((((((c.m.r32(fp.wrapping_add(12)) as i32)) < (0i32)) as i32)) != 0) && ((((((c.m.r32(fp.wrapping_add(16)) as i32)) < (0i32)) as i32)) != 0)) as i32)) != 0) && ((((((c.m.r32(fp.wrapping_add(20)) as i32)) < (0i32)) as i32)) != 0)) as i32)) != 0) {
                            l_modelOnly_2 = 1i32;
                        } else {
                            l_modelOnly_2 = 0i32;
                        }
                        l_job_3 = F::BtlJob_Alloc(c);
                        { let _ = F::memset(c, l_job_3, 0i32, 64u32); };
                        { let v_ = l_side_1; let a_ = (l_job_3).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
                        { let v_ = 1i32; let a_ = (l_job_3).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                        { let v_ = fid::BtlLoad_StepChara; let a_ = (l_job_3).wrapping_add(4); c.m.w32(a_, v_); };
                        { let v_ = (c.m.r32(fp) as i32); let a_ = (l_job_3).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
                        { let v_ = (c.m.r32(fp.wrapping_add(12)) as i32); let a_ = (l_job_3).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                        { let v_ = (c.m.r32(fp.wrapping_add(16)) as i32); let a_ = (l_job_3).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                        { let v_ = (c.m.r32(fp.wrapping_add(20)) as i32); let a_ = (l_job_3).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
                        { let v_ = (c.m.r32(fp.wrapping_add(4)) as i32); let a_ = (l_job_3).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                        { let v_ = (c.m.r32(fp.wrapping_add(8)) as i32); let a_ = (l_job_3).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
                        { let v_ = l_modelOnly_2; let a_ = (l_job_3).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
                        { let v_ = 0i32; let a_ = (l_job_3).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
                        if (((((l_modelOnly_2) == (0i32)) as i32)) != 0) {
                            F::BtlScene_FreeChar(c, l_side_1);
                        }
                        { let a0_ = (c.m.r32((l_job_3).wrapping_add(16)) as i32); let a1_ = (c.m.r32((l_job_3).wrapping_add(20)) as i32); let a2_ = (c.m.r32((l_job_3).wrapping_add(36)) as i32); let a3_ = (c.m.r32((l_job_3).wrapping_add(40)) as i32); F::BattleSide_SetForm(c, a0_, a1_, a2_, a3_) };
                        { let _ = F::Job_Push(c, l_job_3); };
                        F::BtlChange_NotifyTaken(c);
                    }
                }
                { let o_ = l_side_1; let n_ = (o_).wrapping_add(1); l_side_1 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlLoad_PollCharaRequest", &PROBE_N, &[], true); }
    r_
}

pub fn BtlLoad_Reload(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlLoad_Reload", &PROBE_N, &[], false); }
    let mut l_res_1: u32 = 0;
    let mut l_side_2: i32 = 0;
    let mut l_i_3: i32 = 0;
    let mut l_chara_4: i32 = 0;
    let mut l_costume_5: i32 = 0;
    let mut l_variant_6: i32 = 0;
    let mut l_job_7: u32 = 0;
    let mut l_m_8: u32 = 0;
    let r_ = 'f: {
    l_res_1 = (c.m.r32(sym::gCommonRes)).wrapping_add(((32i32) as u32));
    if (((((((((l_res_1) != (((0i32) as u32))) as i32)) != 0) && (((((c.m.r32((l_res_1).wrapping_add(20))) != (((0i32) as u32))) as i32)) != 0)) as i32)) != 0) {
        { let r_ = (((!(8192i32))) as u64); let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); let n_ = ((c.m.r64(a_)) & (r_)); c.m.w64(a_, n_); };
        { let a0_ = c.m.r32((l_res_1).wrapping_add(20)); F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = (l_res_1).wrapping_add(20); c.m.w32(a_, v_); };
    }
    F::BtlRes_CommitReloadEx(c);
    F::Job_Clear(c);
    F::BtlJob_InitPool(c);
    { let _ = { let a0_ = F::Battle_GetStartStage(c); ((D::call(c, fid::BtlLoad_RequestStageReload, &[((a0_) as i64 as u64), ((0i32) as i64 as u64)])) as i32) }; };
    {
        l_side_2 = 0i32;
        'l1: loop {
            if !(((((l_side_2) < (2i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((F::BattleSide_IsFormChanged(c, l_side_2)) != 0) {
                    l_chara_4 = F::BattleSide_GetStartChara(c, l_side_2);
                    l_costume_5 = F::BattleSide_GetStartCostume(c, l_side_2);
                    l_variant_6 = F::BattleSide_GetStartVariant(c, l_side_2);
                    F::BattleSide_SetForm(c, l_side_2, l_chara_4, l_costume_5, l_variant_6);
                    l_job_7 = F::BtlJob_Alloc(c);
                    { let _ = F::memset(c, l_job_7, 0i32, 64u32); };
                    { let v_ = fid::BtlLoad_StepChara; let a_ = (l_job_7).wrapping_add(4); c.m.w32(a_, v_); };
                    { let v_ = l_side_2; let a_ = (l_job_7).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = (l_job_7).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                    { let v_ = l_chara_4; let a_ = (l_job_7).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
                    { let v_ = l_chara_4; let a_ = (l_job_7).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                    { let v_ = l_chara_4; let a_ = (l_job_7).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                    { let v_ = l_chara_4; let a_ = (l_job_7).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
                    { let v_ = l_costume_5; let a_ = (l_job_7).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                    { let v_ = l_variant_6; let a_ = (l_job_7).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = (l_job_7).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 1i32; let a_ = (l_job_7).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
                    F::BtlScene_FreeChar(c, l_side_2);
                    { let _ = F::Job_Push(c, l_job_7); };
                } else {
                    { let a0_ = F::BattleSide_GetObjId(c, l_side_2); let a1_ = F::BattleSide_GetModelSlot(c, l_side_2); F::BtlObj_Rebind(c, a0_, a1_) };
                }
            }
            { let o_ = l_side_2; let n_ = (o_).wrapping_add(1); l_side_2 = n_; };
        }
    }
    if (((((F::BtlJob_GetFreeCount(c)) != (8i32)) as i32)) != 0) {
        F::Load_RunBlocking(c);
    }
    {
        l_side_2 = 0i32;
        'l3: loop {
            if !(((((l_side_2) < (2i32)) as i32)) != 0) { break 'l3; }
            'c4: {
                {
                    l_i_3 = 0i32;
                    'l5: loop {
                        if !(((((((l_i_3) as u32)) < (((F::BattleSide_GetMemberCount(c, l_side_2)) as u32))) as i32)) != 0) { break 'l5; }
                        'c6: {
                            l_m_8 = F::BattleSide_GetMember(c, l_side_2, l_i_3);
                            { let v_ = c.m.r32(((l_m_8).wrapping_add(92)).wrapping_add((((0i32) as u32)).wrapping_mul(4))); let a_ = (l_m_8).wrapping_add(88); c.m.w32(a_, v_); };
                        }
                        { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
                    }
                }
            }
            { let o_ = l_side_2; let n_ = (o_).wrapping_add(1); l_side_2 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlLoad_Reload", &PROBE_N, &[], true); }
    r_
}

pub fn BtlLoad_PushInitialJob(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlLoad_PushInitialJob", &PROBE_N, &[], false); }
    let mut l_job_1: u32 = 0;
    let r_ = 'f: {
    F::BtlJob_InitPool(c);
    l_job_1 = F::BtlJob_Alloc(c);
    { let _ = F::memset(c, l_job_1, 0i32, 64u32); };
    { let v_ = fid::BtlLoad_StepInitial; let a_ = (l_job_1).wrapping_add(4); c.m.w32(a_, v_); };
    { let v_ = F::Battle_GetScript(c); let a_ = (l_job_1).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
    if ((((((c.m.r32((l_job_1).wrapping_add(12)) as i32)) >= (0i32)) as i32)) != 0) {
        { let v_ = 90i32; let a_ = (l_job_1).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
    }
    { let _ = F::Job_Push(c, l_job_1); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlLoad_PushInitialJob", &PROBE_N, &[], true); }
    r_
}

pub fn BtlLoad_FreeAll(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlLoad_FreeAll", &PROBE_N, &[], false); }
    let mut l_res_1: u32 = 0;
    let mut l_ev_2: u32 = 0;
    let mut l_m_3: u32 = 0;
    let mut l_side_4: i32 = 0;
    let mut l_i_5: i32 = 0;
    let r_ = 'f: {
    l_res_1 = (c.m.r32(sym::gCommonRes)).wrapping_add(((32i32) as u32));
    l_ev_2 = F::Battle_GetEventWork(c);
    if (((((F::Battle_GetMode(c)) == (1i32)) as i32)) != 0) {
        { let a0_ = (c.m.r32(l_ev_2) as i32); F::Gsc_UnloadFile(c, a0_) };
        { let a0_ = c.m.r32((l_res_1).wrapping_add(16)); F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = (l_res_1).wrapping_add(16); c.m.w32(a_, v_); };
    }
    {
        l_side_4 = 0i32;
        'l1: loop {
            if !(((((l_side_4) < (2i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                {
                    l_i_5 = 0i32;
                    'l3: loop {
                        if !(((((l_i_5) < (5i32)) as i32)) != 0) { break 'l3; }
                        'c4: {
                            l_m_3 = F::BattleSide_GetMember(c, l_side_4, l_i_5);
                            { let a0_ = c.m.r32(((l_m_3).wrapping_add(92)).wrapping_add((((0i32) as u32)).wrapping_mul(4))); F::Heap_Free(c, a0_) };
                            { let v_ = ((0i32) as u32); let a_ = ((l_m_3).wrapping_add(92)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                            { let a0_ = c.m.r32(((l_m_3).wrapping_add(92)).wrapping_add((((1i32) as u32)).wrapping_mul(4))); F::Heap_Free(c, a0_) };
                            { let v_ = ((0i32) as u32); let a_ = ((l_m_3).wrapping_add(92)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                        }
                        { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
                    }
                }
            }
            { let o_ = l_side_4; let n_ = (o_).wrapping_add(1); l_side_4 = n_; };
        }
    }
    { let a0_ = c.m.r32((l_res_1).wrapping_add(12)); F::Heap_Free(c, a0_) };
    { let v_ = ((0i32) as u32); let a_ = (l_res_1).wrapping_add(12); c.m.w32(a_, v_); };
    { let a0_ = c.m.r32((l_res_1).wrapping_add(8)); F::Heap_Free(c, a0_) };
    { let v_ = ((0i32) as u32); let a_ = (l_res_1).wrapping_add(8); c.m.w32(a_, v_); };
    { let a0_ = c.m.r32((l_res_1).wrapping_add(4)); F::Heap_Free(c, a0_) };
    { let v_ = ((0i32) as u32); let a_ = (l_res_1).wrapping_add(4); c.m.w32(a_, v_); };
    if (((((((((l_res_1) != (((0i32) as u32))) as i32)) != 0) && (((((c.m.r32((l_res_1).wrapping_add(20))) != (((0i32) as u32))) as i32)) != 0)) as i32)) != 0) {
        { let r_ = (((!(8192i32))) as u64); let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); let n_ = ((c.m.r64(a_)) & (r_)); c.m.w64(a_, n_); };
        { let a0_ = c.m.r32((l_res_1).wrapping_add(20)); F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = (l_res_1).wrapping_add(20); c.m.w32(a_, v_); };
    }
    F::Snd_Reset(c);
    F::Snd_UnloadBank(c, ((((4i32) as i64 as u64)) as u32));
    F::Snd_UnloadBank(c, ((((8i32) as i64 as u64)) as u32));
    F::Snd_UnloadBank(c, ((((16i32) as i64 as u64)) as u32));
    F::Snd_UnloadBank(c, ((((32i32) as i64 as u64)) as u32));
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlLoad_FreeAll", &PROBE_N, &[], true); }
    r_
}

pub fn BattleResult_Clear(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BattleResult_Clear", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let _ = { let a0_ = F::Battle_GetResult(c); F::memset(c, a0_, 0i32, 72u32) }; };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BattleResult_Clear", &PROBE_N, &[], true); }
    r_
}

pub fn BattleResult_Set(c: &mut Ctx, mut p_flags_1: i32, mut p_reason_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BattleResult_Set", &PROBE_N, &[(p_flags_1) as u32, (p_reason_2) as u32], false); }
    let mut l_result_3: u32 = 0;
    let r_ = 'f: {
    l_result_3 = F::Battle_GetResult(c);
    { let v_ = p_flags_1; let a_ = l_result_3; c.m.w32(a_, (v_) as u32); };
    { let v_ = p_reason_2; let a_ = (l_result_3).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
    if ((((p_reason_2) & (32768i32))) != 0) {
        F::BattleReplay_SetActive(c, 0i32);
    }
    if ((((p_reason_2) & (65536i32))) != 0) {
        F::BattleReplay_SetActive(c, 1i32);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BattleResult_Set", &PROBE_N, &[], true); }
    r_
}

pub fn BattleResult_IsAborted(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BattleResult_IsAborted", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if (((({ let a_ = F::Battle_GetResult(c); (c.m.r32(a_) as i32) }) & (8i32))) != 0) {
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BattleResult_IsAborted", &PROBE_N, &[], true); }
    r_
}

pub fn Battle_IsRematchRequested(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Battle_IsRematchRequested", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f ((((({ let a_ = (F::Battle_GetResult(c)).wrapping_add(4); (c.m.r32(a_) as i32) }) & (98304i32))) != (0i32)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Battle_IsRematchRequested", &PROBE_N, &[], true); }
    r_
}

pub fn BattleResult_HasWinner(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BattleResult_HasWinner", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f ((((({ let a_ = F::Battle_GetResult(c); (c.m.r32(a_) as i32) }) & (3i32))) != (0i32)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BattleResult_HasWinner", &PROBE_N, &[], true); }
    r_
}

pub fn BattleResult_IsPlayerWin(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BattleResult_IsPlayerWin", &PROBE_N, &[], false); }
    let mut l_result_1: u32 = 0;
    let r_ = 'f: {
    l_result_1 = F::Battle_GetResult(c);
    if ((((((((((F::Battle_IsSplitScreen(c)) != 0) || (((((F::Battle_GetMode(c)) == (8i32)) as i32)) != 0)) as i32)) != 0) || (((((F::Battle_GetMode(c)) == (1i32)) as i32)) != 0)) as i32)) != 0) {
        break 'f 1i32;
    }
    if (((((c.m.r32(l_result_1) as i32)) & (1i32))) != 0) {
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BattleResult_IsPlayerWin", &PROBE_N, &[], true); }
    r_
}

pub fn BattleResult_IsReasonBit20(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BattleResult_IsReasonBit20", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f ((({ let a_ = (F::Battle_GetResult(c)).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_shr((20i32) as u32)) & (1i32));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BattleResult_IsReasonBit20", &PROBE_N, &[], true); }
    r_
}

pub fn BattleResult_GetWinnerSide(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BattleResult_GetWinnerSide", &PROBE_N, &[], false); }
    let mut l_winner_1: i32 = 0;
    let r_ = 'f: {
    l_winner_1 = { let a_ = F::Battle_GetResult(c); (c.m.r32(a_) as i32) };
    if ((((l_winner_1) & (1i32))) != 0) {
        break 'f 0i32;
    }
    if ((((l_winner_1) & (2i32))) != 0) {
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BattleResult_GetWinnerSide", &PROBE_N, &[], true); }
    r_
}

pub fn BattleResult_IsKo(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BattleResult_IsKo", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f (({ let a_ = (F::Battle_GetResult(c)).wrapping_add(4); (c.m.r32(a_) as i32) }) & (1i32));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BattleResult_IsKo", &PROBE_N, &[], true); }
    r_
}

pub fn BattleResult_IsTimeUp(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BattleResult_IsTimeUp", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f ((({ let a_ = (F::Battle_GetResult(c)).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_shr((1i32) as u32)) & (1i32));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BattleResult_IsTimeUp", &PROBE_N, &[], true); }
    r_
}

pub fn BattleResult_IsReasonBit2(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BattleResult_IsReasonBit2", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f ((({ let a_ = (F::Battle_GetResult(c)).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_shr((2i32) as u32)) & (1i32));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BattleResult_IsReasonBit2", &PROBE_N, &[], true); }
    r_
}

pub fn BattleResult_IsReasonBit18(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BattleResult_IsReasonBit18", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f ((({ let a_ = (F::Battle_GetResult(c)).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_shr((18i32) as u32)) & (1i32));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BattleResult_IsReasonBit18", &PROBE_N, &[], true); }
    r_
}

pub fn BattleResult_IsReasonBit5or6(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BattleResult_IsReasonBit5or6", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f ((((({ let a_ = (F::Battle_GetResult(c)).wrapping_add(4); (c.m.r32(a_) as i32) }) & (96i32))) != (0i32)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BattleResult_IsReasonBit5or6", &PROBE_N, &[], true); }
    r_
}

pub fn BattleResult_IsWinnerEvent59Clear(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BattleResult_IsWinnerEvent59Clear", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f ((({ let a0_ = F::BattleResult_GetWinnerSide(c); F::BtlEvent_WasRaised(c, a0_, 89i32) }) == (0i32)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BattleResult_IsWinnerEvent59Clear", &PROBE_N, &[], true); }
    r_
}

pub fn BattleResult_IsEvent3CSet(c: &mut Ctx, mut p_side_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BattleResult_IsEvent3CSet", &PROBE_N, &[(p_side_1) as u32], false); }
    let r_ = 'f: {
    break 'f (((F::BtlEvent_WasRaised(c, p_side_1, 60i32)) != (0i32)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BattleResult_IsEvent3CSet", &PROBE_N, &[], true); }
    r_
}

pub fn BattleResult_CollectEvents(c: &mut Ctx, mut p_result_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BattleResult_CollectEvents", &PROBE_N, &[(p_result_1) as u32], false); }
    let mut l_i_2: u32 = 0;
    let mut l_count_3: u32 = 0;
    let mut l_sum_4: u16 = 0;
    let mut l_j_5: i32 = 0;
    let mut l_m_6: u32 = 0;
    let r_ = 'f: {
    { let v_ = ((0i32) as u64); let a_ = (p_result_1).wrapping_add(16); c.m.w64(a_, v_); };
    if ((F::BtlEvent_WasRaised(c, 0i32, 81i32)) != 0) {
        { let r_ = ((1i32) as u64); let a_ = (p_result_1).wrapping_add(16); let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
    }
    if ((F::BtlEvent_WasRaised(c, 0i32, 82i32)) != 0) {
        { let r_ = ((2i32) as u64); let a_ = (p_result_1).wrapping_add(16); let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
    }
    if ((F::BtlEvent_WasRaised(c, 0i32, 83i32)) != 0) {
        { let r_ = ((4i32) as u64); let a_ = (p_result_1).wrapping_add(16); let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
    }
    if ((((!((F::BtlEvent_WasRaised(c, 0i32, 89i32)) != 0)) as i32)) != 0) {
        { let r_ = ((8i32) as u64); let a_ = (p_result_1).wrapping_add(16); let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
    }
    if ((F::BtlEvent_WasRaised(c, 0i32, 84i32)) != 0) {
        { let r_ = ((16i32) as u64); let a_ = (p_result_1).wrapping_add(16); let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
    }
    if ((((((((((((((((((F::BtlEvent_WasRaised(c, 0i32, 55i32)) != 0) && ((F::BtlEvent_WasRaised(c, 0i32, 56i32)) != 0)) as i32)) != 0) && ((F::BtlEvent_WasRaised(c, 0i32, 57i32)) != 0)) as i32)) != 0) && ((F::BtlEvent_WasRaised(c, 0i32, 58i32)) != 0)) as i32)) != 0) && ((F::BtlEvent_WasRaised(c, 0i32, 59i32)) != 0)) as i32)) != 0) {
        { let r_ = ((256i32) as u64); let a_ = (p_result_1).wrapping_add(16); let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
    }
    if ((F::BtlEvent_WasRaised(c, 0i32, 85i32)) != 0) {
        { let r_ = ((512i32) as u64); let a_ = (p_result_1).wrapping_add(16); let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
    }
    if ((F::BtlEvent_WasRaised(c, 0i32, 87i32)) != 0) {
        { let r_ = ((1024i32) as u64); let a_ = (p_result_1).wrapping_add(16); let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
    }
    if ((F::BtlEvent_WasRaised(c, 0i32, 52i32)) != 0) {
        { let r_ = ((8192i32) as u64); let a_ = (p_result_1).wrapping_add(16); let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
    }
    if ((F::BtlEvent_WasRaised(c, 0i32, 62i32)) != 0) {
        { let r_ = ((32768i32) as u64); let a_ = (p_result_1).wrapping_add(16); let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
    }
    if ((F::BtlEvent_WasRaised(c, 0i32, 50i32)) != 0) {
        { let r_ = ((65536i32) as u64); let a_ = (p_result_1).wrapping_add(16); let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
    }
    if ((F::BtlEvent_WasRaised(c, 0i32, 63i32)) != 0) {
        { let r_ = ((131072i32) as u64); let a_ = (p_result_1).wrapping_add(16); let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
    }
    if ((F::BtlEvent_WasRaised(c, 0i32, 64i32)) != 0) {
        { let r_ = ((262144i32) as u64); let a_ = (p_result_1).wrapping_add(16); let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
    }
    if ((F::BtlEvent_WasRaised(c, 0i32, 86i32)) != 0) {
        { let r_ = ((524288i32) as u64); let a_ = (p_result_1).wrapping_add(16); let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
    }
    if ((F::BtlEvent_WasRaised(c, 0i32, 54i32)) != 0) {
        { let r_ = ((16777216i32) as u64); let a_ = (p_result_1).wrapping_add(16); let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
    }
    if ((F::BtlEvent_WasRaised(c, 0i32, 36i32)) != 0) {
        { let r_ = ((33554432i32) as u64); let a_ = (p_result_1).wrapping_add(16); let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
    }
    if ((F::BtlEvent_WasRaised(c, 0i32, 37i32)) != 0) {
        { let r_ = ((33554432i32) as u64); let a_ = (p_result_1).wrapping_add(16); let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
    }
    if ((F::BtlEvent_WasRaised(c, 0i32, 38i32)) != 0) {
        { let r_ = ((67108864i32) as u64); let a_ = (p_result_1).wrapping_add(16); let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
    }
    if ((F::BtlEvent_WasRaised(c, 0i32, 39i32)) != 0) {
        { let r_ = ((67108864i32) as u64); let a_ = (p_result_1).wrapping_add(16); let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
    }
    if ((F::BtlEvent_WasRaised(c, 0i32, 40i32)) != 0) {
        { let r_ = ((134217728i32) as u64); let a_ = (p_result_1).wrapping_add(16); let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
    }
    if ((F::BtlEvent_WasRaised(c, 0i32, 31i32)) != 0) {
        { let r_ = ((268435456i32) as u64); let a_ = (p_result_1).wrapping_add(16); let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
    }
    if ((F::BtlEvent_WasRaised(c, 0i32, 51i32)) != 0) {
        { let r_ = (1u64).wrapping_shl((31i32) as u32); let a_ = (p_result_1).wrapping_add(16); let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
    }
    if ((F::BtlEvent_WasRaised(c, 0i32, 65i32)) != 0) {
        { let r_ = (1u64).wrapping_shl((32i32) as u32); let a_ = (p_result_1).wrapping_add(16); let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
    }
    if ((F::BtlEvent_WasRaised(c, 0i32, 69i32)) != 0) {
        { let r_ = (1u64).wrapping_shl((33i32) as u32); let a_ = (p_result_1).wrapping_add(16); let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
    }
    if ((F::BtlEvent_WasRaised(c, 0i32, 66i32)) != 0) {
        { let r_ = (1u64).wrapping_shl((34i32) as u32); let a_ = (p_result_1).wrapping_add(16); let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
    }
    if ((F::BtlEvent_WasRaised(c, 0i32, 67i32)) != 0) {
        { let r_ = (1u64).wrapping_shl((35i32) as u32); let a_ = (p_result_1).wrapping_add(16); let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
    }
    if ((F::BtlEvent_WasRaised(c, 0i32, 68i32)) != 0) {
        { let r_ = (1u64).wrapping_shl((36i32) as u32); let a_ = (p_result_1).wrapping_add(16); let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
    }
    if ((((!((F::BtlEvent_WasRaised(c, 0i32, 34i32)) != 0)) as i32)) != 0) {
        { let r_ = (1u64).wrapping_shl((37i32) as u32); let a_ = (p_result_1).wrapping_add(16); let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
    }
    if ((F::BtlEvent_WasRaised(c, 0i32, 88i32)) != 0) {
        { let r_ = (1u64).wrapping_shl((39i32) as u32); let a_ = (p_result_1).wrapping_add(16); let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
    }
    if ((((((((!((F::BtlEvent_WasRaised(c, 0i32, 78i32)) != 0)) as i32)) != 0) && (((((((F::BattleSide_GetMemberCount(c, 0i32)) as u32)) >= (((2i32) as u32))) as i32)) != 0)) as i32)) != 0) {
        { let r_ = (1u64).wrapping_shl((40i32) as u32); let a_ = (p_result_1).wrapping_add(16); let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
    }
    if ((F::BtlEvent_WasRaised(c, 0i32, 33i32)) != 0) {
        { let r_ = (1u64).wrapping_shl((41i32) as u32); let a_ = (p_result_1).wrapping_add(16); let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
    }
    if ((F::BtlEvent_WasRaised(c, 0i32, 60i32)) != 0) {
        { let r_ = (1u64).wrapping_shl((42i32) as u32); let a_ = (p_result_1).wrapping_add(16); let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
    }
    if ((F::BtlEvent_WasRaised(c, 0i32, 30i32)) != 0) {
        { let r_ = (1u64).wrapping_shl((44i32) as u32); let a_ = (p_result_1).wrapping_add(16); let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
    }
    l_count_3 = ((F::BattleSide_GetMemberCount(c, 0i32)) as u32);
    l_sum_4 = ((0i32) as u16);
    {
        l_i_2 = ((0i32) as u32);
        'l1: loop {
            if !(((((l_i_2) < (l_count_3)) as i32)) != 0) { break 'l1; }
            'c2: {
                l_m_6 = F::BattleSide_GetMember(c, 0i32, ((l_i_2) as i32));
                if (((((l_m_6) != (((0i32) as u32))) as i32)) != 0) {
                    {
                        l_j_5 = 0i32;
                        'l3: loop {
                            if !(((((l_j_5) < (8i32)) as i32)) != 0) { break 'l3; }
                            'c4: {
                                { let r_ = ((c.m.r16(((l_m_6).wrapping_add(20)).wrapping_add((((l_j_5) as u32)).wrapping_mul(2)))) as i32); let n_ = (((((l_sum_4) as i32)).wrapping_add(r_)) as u16); l_sum_4 = n_; };
                            }
                            { let o_ = l_j_5; let n_ = (o_).wrapping_add(1); l_j_5 = n_; };
                        }
                    }
                }
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
    if (((((((l_sum_4) as i32)) == (0i32)) as i32)) != 0) {
        { let r_ = (1u64).wrapping_shl((45i32) as u32); let a_ = (p_result_1).wrapping_add(16); let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
    }
    if ((F::BtlEvent_WasRaised(c, 0i32, 70i32)) != 0) {
        { let r_ = (1u64).wrapping_shl((46i32) as u32); let a_ = (p_result_1).wrapping_add(16); let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
    }
    if ((F::BtlEvent_WasRaised(c, 0i32, 71i32)) != 0) {
        { let r_ = (1u64).wrapping_shl((47i32) as u32); let a_ = (p_result_1).wrapping_add(16); let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BattleResult_CollectEvents", &PROBE_N, &[], true); }
    r_
}

pub fn battle_load__Side(c: &mut Ctx, mut p_n_1: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "battle_load__Side", &PROBE_N, &[(p_n_1) as u32], false); }
    let mut l_setup_2: u32 = 0;
    let r_ = 'f: {
    l_setup_2 = F::Battle_GetSetup(c);
    break 'f ((l_setup_2).wrapping_add(192)).wrapping_add((((p_n_1) as u32)).wrapping_mul(624));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "battle_load__Side", &PROBE_N, &[], true); }
    r_
}

pub fn battle_load__Member(c: &mut Ctx, mut p_n_1: i32, mut p_i_2: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "battle_load__Member", &PROBE_N, &[(p_n_1) as u32, (p_i_2) as u32], false); }
    let mut l_side_3: u32 = 0;
    let r_ = 'f: {
    l_side_3 = F::battle_load__Side(c, p_n_1);
    break 'f ((l_side_3).wrapping_add(4)).wrapping_add((((p_i_2) as u32)).wrapping_mul(100));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "battle_load__Member", &PROBE_N, &[], true); }
    r_
}

pub fn BattleResult_CountFrame(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BattleResult_CountFrame", &PROBE_N, &[], false); }
    let mut l_res_1: u32 = 0;
    let r_ = 'f: {
    l_res_1 = F::Battle_GetResult(c);
    if ((F::BtlEvent_IsNew(c, 0i32, 72i32)) != 0) {
        { let a_ = (l_res_1).wrapping_add(24); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BattleResult_CountFrame", &PROBE_N, &[], true); }
    r_
}

pub fn BattleResult_Stub1291B8(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BattleResult_Stub1291B8", &PROBE_N, &[], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BattleResult_Stub1291B8", &PROBE_N, &[], true); }
    r_
}

pub fn BattleResult_Finish(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BattleResult_Finish", &PROBE_N, &[], false); }
    let mut l_res_1: u32 = 0;
    let r_ = 'f: {
    l_res_1 = F::Battle_GetResult(c);
    F::BattleResult_CollectEvents(c, l_res_1);
    { let s_ = F::BtlSeq_GetClock(c); let d_ = (l_res_1).wrapping_add(52); rt::memcpy(c, d_, s_, 16); };
    if (((((c.m.r32(l_res_1) as i32)) & (8i32))) != 0) {
        { let v_ = 1i32; let a_ = (l_res_1).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
    }
    if (((((c.m.r32((l_res_1).wrapping_add(4)) as i32)) & (32i32))) != 0) {
        { let r_ = 2i32; let a_ = l_res_1; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
    if (((((c.m.r32((l_res_1).wrapping_add(4)) as i32)) & (64i32))) != 0) {
        { let r_ = 1i32; let a_ = l_res_1; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BattleResult_Finish", &PROBE_N, &[], true); }
    r_
}

pub fn BattleResult_GetFlags(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BattleResult_GetFlags", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f { let a_ = F::Battle_GetResult(c); (c.m.r32(a_) as i32) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BattleResult_GetFlags", &PROBE_N, &[], true); }
    r_
}

pub fn BattleResult_GetReason(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BattleResult_GetReason", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f { let a_ = (F::Battle_GetResult(c)).wrapping_add(4); (c.m.r32(a_) as i32) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BattleResult_GetReason", &PROBE_N, &[], true); }
    r_
}

pub fn BattleResult_GetEventSummary(c: &mut Ctx) -> u64 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BattleResult_GetEventSummary", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f { let a_ = (F::Battle_GetResult(c)).wrapping_add(16); c.m.r64(a_) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BattleResult_GetEventSummary", &PROBE_N, &[], true); }
    r_
}

pub fn BattleResult_GetPtr(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BattleResult_GetPtr", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f F::Battle_GetResult(c);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BattleResult_GetPtr", &PROBE_N, &[], true); }
    r_
}

pub fn BtlClock_ToSeconds(c: &mut Ctx, mut p_clock_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlClock_ToSeconds", &PROBE_N, &[(p_clock_1) as u32], false); }
    let r_ = 'f: {
    break 'f ((((((c.m.r16((p_clock_1).wrapping_add(4)) as i16)) as i32)).wrapping_mul(3600i32)).wrapping_add(((((c.m.r16((p_clock_1).wrapping_add(6)) as i16)) as i32)).wrapping_mul(60i32))).wrapping_add((((c.m.r16((p_clock_1).wrapping_add(8)) as i16)) as i32));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlClock_ToSeconds", &PROBE_N, &[], true); }
    r_
}

pub fn BtlEvent_RaiseTimeEvents(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlEvent_RaiseTimeEvents", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(80) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 80);
    let mut l_sec_1: i32 = 0;
    let mut l_i_2: u32 = 0;
    let r_ = 'f: {
        rt::memset(c, fp, 0, 76);
        { let v_ = 10i32; c.m.w32(fp.wrapping_add(0), (v_) as u32); }
        { let v_ = 15i32; c.m.w32(fp.wrapping_add(4), (v_) as u32); }
        { let v_ = 20i32; c.m.w32(fp.wrapping_add(8), (v_) as u32); }
        { let v_ = 25i32; c.m.w32(fp.wrapping_add(12), (v_) as u32); }
        { let v_ = 30i32; c.m.w32(fp.wrapping_add(16), (v_) as u32); }
        { let v_ = 35i32; c.m.w32(fp.wrapping_add(20), (v_) as u32); }
        { let v_ = 40i32; c.m.w32(fp.wrapping_add(24), (v_) as u32); }
        { let v_ = 45i32; c.m.w32(fp.wrapping_add(28), (v_) as u32); }
        { let v_ = 50i32; c.m.w32(fp.wrapping_add(32), (v_) as u32); }
        { let v_ = 55i32; c.m.w32(fp.wrapping_add(36), (v_) as u32); }
        { let v_ = 60i32; c.m.w32(fp.wrapping_add(40), (v_) as u32); }
        { let v_ = 70i32; c.m.w32(fp.wrapping_add(44), (v_) as u32); }
        { let v_ = 80i32; c.m.w32(fp.wrapping_add(48), (v_) as u32); }
        { let v_ = 90i32; c.m.w32(fp.wrapping_add(52), (v_) as u32); }
        { let v_ = 100i32; c.m.w32(fp.wrapping_add(56), (v_) as u32); }
        { let v_ = 120i32; c.m.w32(fp.wrapping_add(60), (v_) as u32); }
        { let v_ = 140i32; c.m.w32(fp.wrapping_add(64), (v_) as u32); }
        { let v_ = 160i32; c.m.w32(fp.wrapping_add(68), (v_) as u32); }
        { let v_ = 180i32; c.m.w32(fp.wrapping_add(72), (v_) as u32); }
        l_sec_1 = { let a0_ = F::BtlSeq_GetSubClock(c); F::BtlClock_ToSeconds(c, a0_) };
        {
            l_i_2 = ((0i32) as u32);
            'l1: loop {
                if !(((((l_i_2) < (((19i32) as u32))) as i32)) != 0) { break 'l1; }
                'c2: {
                    if (((((l_sec_1) >= ((c.m.r32((fp).wrapping_add((l_i_2).wrapping_mul(4))) as i32))) as i32)) != 0) {
                        F::BtlEvent_Raise(c, 0i32, ((l_i_2) as i32));
                        F::BtlEvent_Raise(c, 1i32, ((l_i_2) as i32));
                    }
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlEvent_RaiseTimeEvents", &PROBE_N, &[], true); }
    r_
}

pub fn BtlEvent_UpdateRequests(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlEvent_UpdateRequests", &PROBE_N, &[], false); }
    let mut l_ev_1: u32 = 0;
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
    l_ev_1 = F::Battle_GetEventWork(c);
    if ((((((c.m.r32((l_ev_1).wrapping_add(104)) as i32)) != (0i32)) as i32)) != 0) {
        if (((((F::BtlFacade_AreBothInterruptible(c)) != (0i32)) as i32)) != 0) {
            F::BtlEvent_Raise(c, 0i32, 76i32);
            F::BtlEvent_Raise(c, 1i32, 76i32);
            F::BtlFacade_ClearFlag200(c);
            { let v_ = 0i32; let a_ = (l_ev_1).wrapping_add(104); c.m.w32(a_, (v_) as u32); };
        }
    }
    if ((((((c.m.r32((l_ev_1).wrapping_add(108)) as i32)) == (1i32)) as i32)) != 0) {
        if ((((((((((((((F::BtlEvent_IsNew(c, 0i32, 77i32)) != 0) || ((F::BtlEvent_IsNew(c, 1i32, 77i32)) != 0)) as i32)) != 0) || ((F::BtlEvent_IsNew(c, 0i32, 78i32)) != 0)) as i32)) != 0) || ((F::BtlEvent_IsNew(c, 1i32, 78i32)) != 0)) as i32)) != 0) {
            { let v_ = 0i32; let a_ = (l_ev_1).wrapping_add(108); c.m.w32(a_, (v_) as u32); };
        }
    }
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < (2i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((F::BtlEvent_IsNew(c, l_i_2, 79i32)) != 0) {
                    F::BtlFacade_ClearFixedCamera(c);
                }
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlEvent_UpdateRequests", &PROBE_N, &[], true); }
    r_
}

pub fn BtlEventSet_Rotate(c: &mut Ctx, mut p_set_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlEventSet_Rotate", &PROBE_N, &[(p_set_1) as u32], false); }
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < (2i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let v_ = c.m.r64((p_set_1).wrapping_add((((l_i_2) as u32)).wrapping_mul(8))); let a_ = ((p_set_1).wrapping_add(16)).wrapping_add((((l_i_2) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                { let r_ = c.m.r64((p_set_1).wrapping_add((((l_i_2) as u32)).wrapping_mul(8))); let a_ = ((p_set_1).wrapping_add(32)).wrapping_add((((l_i_2) as u32)).wrapping_mul(8)); let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
                { let v_ = ((0i32) as u64); let a_ = (p_set_1).wrapping_add((((l_i_2) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlEventSet_Rotate", &PROBE_N, &[], true); }
    r_
}

pub fn BtlEvent_SetBit(c: &mut Ctx, mut p_bits_1: u32, mut p_n_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlEvent_SetBit", &PROBE_N, &[(p_bits_1) as u32, (p_n_2) as u32], false); }
    let mut l_mask_3: u64 = 0;
    let r_ = 'f: {
    l_mask_3 = ((1i32) as u64);
    { let r_ = rt::div_i32(p_n_2, 64i32); let n_ = (p_bits_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); p_bits_1 = n_; };
    { let r_ = (l_mask_3).wrapping_shl((rt::rem_i32(p_n_2, 64i32)) as u32); let a_ = p_bits_1; let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlEvent_SetBit", &PROBE_N, &[], true); }
    r_
}

pub fn BtlEventSet_IsNew(c: &mut Ctx, mut p_set_1: u32, mut p_n_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlEventSet_IsNew", &PROBE_N, &[(p_set_1) as u32, (p_n_2) as u32], false); }
    let r_ = 'f: {
    break 'f (((((((c.m.r64((p_set_1).wrapping_add((((rt::div_i32(p_n_2, 64i32)) as u32)).wrapping_mul(8)))) & ((!(c.m.r64(((p_set_1).wrapping_add(16)).wrapping_add((((rt::div_i32(p_n_2, 64i32)) as u32)).wrapping_mul(8)))))))).wrapping_shr((rt::rem_i32(p_n_2, 64i32)) as u32)) & (((1i32) as u64)))) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlEventSet_IsNew", &PROBE_N, &[], true); }
    r_
}

pub fn BtlEventSet_WasRaised(c: &mut Ctx, mut p_set_1: u32, mut p_n_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlEventSet_WasRaised", &PROBE_N, &[(p_set_1) as u32, (p_n_2) as u32], false); }
    let r_ = 'f: {
    break 'f (((((c.m.r64(((p_set_1).wrapping_add(32)).wrapping_add((((rt::div_i32(p_n_2, 64i32)) as u32)).wrapping_mul(8)))).wrapping_shr((rt::rem_i32(p_n_2, 64i32)) as u32)) & (((1i32) as u64)))) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlEventSet_WasRaised", &PROBE_N, &[], true); }
    r_
}

pub fn BtlEvent_ClearAll(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlEvent_ClearAll", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let _ = { let a0_ = F::Battle_GetEventWork(c); F::memset(c, a0_, 0i32, 112u32) }; };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlEvent_ClearAll", &PROBE_N, &[], true); }
    r_
}

pub fn BtlEvent_Reset(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlEvent_Reset", &PROBE_N, &[], false); }
    let mut l_keep_1: i32 = 0;
    let r_ = 'f: {
    l_keep_1 = { let a_ = F::Battle_GetEventWork(c); (c.m.r32(a_) as i32) };
    F::BtlEvent_ClearAll(c);
    { let v_ = l_keep_1; let a_ = F::Battle_GetEventWork(c); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlEvent_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn BtlEvent_Raise(c: &mut Ctx, mut p_side_1: i32, mut p_ev_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlEvent_Raise", &PROBE_N, &[(p_side_1) as u32, (p_ev_2) as u32], false); }
    let r_ = 'f: {
    { let a0_ = ((F::Battle_GetEventWork(c)).wrapping_add(8)).wrapping_add((((p_side_1) as u32)).wrapping_mul(48)); F::BtlEvent_SetBit(c, a0_, p_ev_2) };
    's1: {
        let sv3: i32 = p_ev_2;
        match sv3 {
            90i32 | 91i32 => {
                F::BtlScript_StartWaitEvents(c);
                break 's1;
            }
            _ => {
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlEvent_Raise", &PROBE_N, &[], true); }
    r_
}

pub fn BtlEvent_IsNew(c: &mut Ctx, mut p_side_1: i32, mut p_ev_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlEvent_IsNew", &PROBE_N, &[(p_side_1) as u32, (p_ev_2) as u32], false); }
    let r_ = 'f: {
    break 'f { let a0_ = ((F::Battle_GetEventWork(c)).wrapping_add(8)).wrapping_add((((p_side_1) as u32)).wrapping_mul(48)); F::BtlEventSet_IsNew(c, a0_, p_ev_2) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlEvent_IsNew", &PROBE_N, &[], true); }
    r_
}

pub fn BtlEvent_WasRaised(c: &mut Ctx, mut p_side_1: i32, mut p_ev_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlEvent_WasRaised", &PROBE_N, &[(p_side_1) as u32, (p_ev_2) as u32], false); }
    let r_ = 'f: {
    break 'f { let a0_ = ((F::Battle_GetEventWork(c)).wrapping_add(8)).wrapping_add((((p_side_1) as u32)).wrapping_mul(48)); F::BtlEventSet_WasRaised(c, a0_, p_ev_2) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlEvent_WasRaised", &PROBE_N, &[], true); }
    r_
}

pub fn BtlEvent_Update(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlEvent_Update", &PROBE_N, &[], false); }
    let mut l_ev_1: u32 = 0;
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
    l_ev_1 = F::Battle_GetEventWork(c);
    F::BtlEvent_UpdateRequests(c);
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < (2i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                F::BtlEventSet_Rotate(c, ((l_ev_1).wrapping_add(8)).wrapping_add((((l_i_2) as u32)).wrapping_mul(48)));
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
    F::BtlEvent_RaiseTimeEvents(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlEvent_Update", &PROBE_N, &[], true); }
    r_
}

pub fn BtlEvent_BeginInterrupt(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlEvent_BeginInterrupt", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let v_ = 1i32; let a_ = (F::Battle_GetEventWork(c)).wrapping_add(104); c.m.w32(a_, (v_) as u32); };
    F::BtlFacade_SetFlag200(c);
    F::Hud_SlideOut(c, F32(0x3f800000));
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlEvent_BeginInterrupt", &PROBE_N, &[], true); }
    r_
}

pub fn BtlEvent_EndInterrupt(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlEvent_EndInterrupt", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::Hud_SlideIn(c, F32(0x3f800000));
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlEvent_EndInterrupt", &PROBE_N, &[], true); }
    r_
}

pub fn BtlEvent_SetWaitOff(c: &mut Ctx, mut p_off_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlEvent_SetWaitOff", &PROBE_N, &[(p_off_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = (((p_off_1) == (0i32)) as i32); let a_ = (F::Battle_GetEventWork(c)).wrapping_add(108); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlEvent_SetWaitOff", &PROBE_N, &[], true); }
    r_
}

pub fn BtlEvent_IsWaitOff(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlEvent_IsWaitOff", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f ((({ let a_ = (F::Battle_GetEventWork(c)).wrapping_add(108); (c.m.r32(a_) as i32) }) == (0i32)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlEvent_IsWaitOff", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMember_ClampBonus(c: &mut Ctx, mut p_m_1: u32, mut p_wide_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMember_ClampBonus", &PROBE_N, &[(p_m_1) as u32, (p_wide_2) as u32], false); }
    let mut l_min_3: i32 = 0;
    let mut l_max_4: i32 = 0;
    let r_ = 'f: {
    if (((((p_wide_2) != (0i32)) as i32)) != 0) {
        l_min_3 = (20i32).wrapping_neg();
        l_max_4 = 60i32;
    } else {
        l_min_3 = (20i32).wrapping_neg();
        l_max_4 = 20i32;
    }
    if ((((((c.m.r32(((p_m_1).wrapping_add(36)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) > (l_max_4)) as i32)) != 0) {
        { let v_ = l_max_4; let a_ = ((p_m_1).wrapping_add(36)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    } else {
        if ((((((c.m.r32(((p_m_1).wrapping_add(36)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) < ((20i32).wrapping_neg())) as i32)) != 0) {
            { let v_ = l_min_3; let a_ = ((p_m_1).wrapping_add(36)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        }
    }
    if ((((((c.m.r32(((p_m_1).wrapping_add(36)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) > (l_max_4)) as i32)) != 0) {
        { let v_ = l_max_4; let a_ = ((p_m_1).wrapping_add(36)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    } else {
        if ((((((c.m.r32(((p_m_1).wrapping_add(36)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) < ((20i32).wrapping_neg())) as i32)) != 0) {
            { let v_ = l_min_3; let a_ = ((p_m_1).wrapping_add(36)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        }
    }
    if ((((((c.m.r32(((p_m_1).wrapping_add(36)).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32)) > (l_max_4)) as i32)) != 0) {
        { let v_ = l_max_4; let a_ = ((p_m_1).wrapping_add(36)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    } else {
        if ((((((c.m.r32(((p_m_1).wrapping_add(36)).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32)) < ((20i32).wrapping_neg())) as i32)) != 0) {
            { let v_ = l_min_3; let a_ = ((p_m_1).wrapping_add(36)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        }
    }
    if ((((((c.m.r32(((p_m_1).wrapping_add(36)).wrapping_add((((3i32) as u32)).wrapping_mul(4))) as i32)) > (l_max_4)) as i32)) != 0) {
        { let v_ = l_max_4; let a_ = ((p_m_1).wrapping_add(36)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    } else {
        if ((((((c.m.r32(((p_m_1).wrapping_add(36)).wrapping_add((((3i32) as u32)).wrapping_mul(4))) as i32)) < ((20i32).wrapping_neg())) as i32)) != 0) {
            { let v_ = l_min_3; let a_ = ((p_m_1).wrapping_add(36)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        }
    }
    if ((((((c.m.r32(((p_m_1).wrapping_add(36)).wrapping_add((((4i32) as u32)).wrapping_mul(4))) as i32)) > (l_max_4)) as i32)) != 0) {
        { let v_ = l_max_4; let a_ = ((p_m_1).wrapping_add(36)).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    } else {
        if ((((((c.m.r32(((p_m_1).wrapping_add(36)).wrapping_add((((4i32) as u32)).wrapping_mul(4))) as i32)) < ((20i32).wrapping_neg())) as i32)) != 0) {
            { let v_ = l_min_3; let a_ = ((p_m_1).wrapping_add(36)).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        }
    }
    if ((((((c.m.r32(((p_m_1).wrapping_add(36)).wrapping_add((((5i32) as u32)).wrapping_mul(4))) as i32)) > (l_max_4)) as i32)) != 0) {
        { let v_ = l_max_4; let a_ = ((p_m_1).wrapping_add(36)).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    } else {
        if ((((((c.m.r32(((p_m_1).wrapping_add(36)).wrapping_add((((5i32) as u32)).wrapping_mul(4))) as i32)) < ((20i32).wrapping_neg())) as i32)) != 0) {
            { let v_ = l_min_3; let a_ = ((p_m_1).wrapping_add(36)).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        }
    }
    if ((((((c.m.r32(((p_m_1).wrapping_add(36)).wrapping_add((((6i32) as u32)).wrapping_mul(4))) as i32)) > (l_max_4)) as i32)) != 0) {
        { let v_ = l_max_4; let a_ = ((p_m_1).wrapping_add(36)).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    } else {
        if ((((((c.m.r32(((p_m_1).wrapping_add(36)).wrapping_add((((6i32) as u32)).wrapping_mul(4))) as i32)) < ((20i32).wrapping_neg())) as i32)) != 0) {
            { let v_ = l_min_3; let a_ = ((p_m_1).wrapping_add(36)).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        }
    }
    if ((((((c.m.r32(((p_m_1).wrapping_add(36)).wrapping_add((((7i32) as u32)).wrapping_mul(4))) as i32)) > (l_max_4)) as i32)) != 0) {
        { let v_ = l_max_4; let a_ = ((p_m_1).wrapping_add(36)).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    } else {
        if ((((((c.m.r32(((p_m_1).wrapping_add(36)).wrapping_add((((7i32) as u32)).wrapping_mul(4))) as i32)) < ((20i32).wrapping_neg())) as i32)) != 0) {
            { let v_ = l_min_3; let a_ = ((p_m_1).wrapping_add(36)).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMember_ClampBonus", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMember_ApplyItems(c: &mut Ctx, mut p_m_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMember_ApplyItems", &PROBE_N, &[(p_m_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
        { let a3_ = (c.m.r32(p_m_1) as i32); F::ItemSet_GetStats(c, (p_m_1).wrapping_add(20), fp.wrapping_add(16), fp, a3_) };
        { let v_ = 0i32; let a_ = ((p_m_1).wrapping_add(36)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = (c.m.r32((fp.wrapping_add(16)).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32); let a_ = ((p_m_1).wrapping_add(36)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = (c.m.r32((fp.wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32); let a_ = ((p_m_1).wrapping_add(36)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = (c.m.r32((fp.wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32); let a_ = ((p_m_1).wrapping_add(36)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = (c.m.r32((fp.wrapping_add(16)).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32); let a_ = ((p_m_1).wrapping_add(36)).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = (c.m.r32((fp.wrapping_add(16)).wrapping_add((((3i32) as u32)).wrapping_mul(4))) as i32); let a_ = ((p_m_1).wrapping_add(36)).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = (c.m.r32((fp.wrapping_add(16)).wrapping_add((((3i32) as u32)).wrapping_mul(4))) as i32); let a_ = ((p_m_1).wrapping_add(36)).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = (c.m.r32((fp.wrapping_add(16)).wrapping_add((((3i32) as u32)).wrapping_mul(4))) as i32); let a_ = ((p_m_1).wrapping_add(36)).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = (c.m.r32((fp.wrapping_add(16)).wrapping_add((((4i32) as u32)).wrapping_mul(4))) as i32); let a_ = (p_m_1).wrapping_add(68); c.m.w32(a_, (v_) as u32); };
        {
            l_i_2 = 0i32;
            'l1: loop {
                if !(((((l_i_2) < (4i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let v_ = (c.m.r32((fp).wrapping_add((((l_i_2) as u32)).wrapping_mul(4))) as i32); let a_ = ((p_m_1).wrapping_add(72)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlMember_ApplyItems", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMember_Init(c: &mut Ctx, mut p_m_1: u32, mut p_chara_2: i32, mut p_costume_3: i32, mut p_variant_4: i32, mut p_cpuLevel_5: i32, mut p_health_6: F32, mut p_items_7: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMember_Init", &PROBE_N, &[(p_m_1) as u32, (p_chara_2) as u32, (p_costume_3) as u32, (p_variant_4) as u32, (p_cpuLevel_5) as u32, (p_health_6).0, (p_items_7) as u32], false); }
    let r_ = 'f: {
    { let _ = F::memset(c, p_m_1, 0i32, 100u32); };
    if (((((p_items_7) != (((0i32) as u32))) as i32)) != 0) {
        { let s_ = p_items_7; let d_ = (p_m_1).wrapping_add(20); rt::memcpy(c, d_, s_, 16); };
    }
    F::BtlMember_ApplyItems(c, p_m_1);
    { let v_ = p_chara_2; let a_ = p_m_1; c.m.w32(a_, (v_) as u32); };
    { let v_ = p_costume_3; let a_ = (p_m_1).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
    { let v_ = p_variant_4; let a_ = (p_m_1).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
    { let v_ = p_cpuLevel_5; let a_ = (p_m_1).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
    { let v_ = p_health_6; let a_ = (p_m_1).wrapping_add(16); c.m.w32(a_, (v_).0); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMember_Init", &PROBE_N, &[], true); }
    r_
}

pub fn BattleSetup_FixForMode(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BattleSetup_FixForMode", &PROBE_N, &[], false); }
    let mut l_rule_1: u32 = 0;
    let mut l_side_2: u32 = 0;
    let mut l_i_3: i32 = 0;
    let mut l_j_4: i32 = 0;
    let mut l_m_5: u32 = 0;
    let r_ = 'f: {
    l_rule_1 = (F::Battle_GetSetup(c)).wrapping_add(8);
    if (((((((((F::Battle_GetMode(c)) != (0i32)) as i32)) != 0) && (((((F::Battle_GetMode(c)) != (4i32)) as i32)) != 0)) as i32)) != 0) {
        if ((((((c.m.r32((l_rule_1).wrapping_add(28)) as i32)) == (1i32)) as i32)) != 0) {
            { let v_ = 0i32; let a_ = (l_rule_1).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
        }
    }
    if (((((F::Battle_GetMode(c)) == (9i32)) as i32)) != 0) {
        if ((((((c.m.r32((l_rule_1).wrapping_add(8)) as i32)) != (0i32)) as i32)) != 0) {
            { let v_ = 0i32; let a_ = (l_rule_1).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        }
    }
    if (((((F::Battle_GetMode(c)) == (7i32)) as i32)) != 0) {
        if ((((((c.m.r32((l_rule_1).wrapping_add(8)) as i32)) != (5i32)) as i32)) != 0) {
            { let v_ = 5i32; let a_ = (l_rule_1).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        }
    }
    {
        l_i_3 = 0i32;
        'l1: loop {
            if !(((((l_i_3) < (2i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                l_side_2 = ((F::Battle_GetSetup(c)).wrapping_add(192)).wrapping_add((((l_i_3) as u32)).wrapping_mul(624));
                's3: {
                    let sv6: i32 = F::Battle_GetMode(c);
                    match sv6 {
                        2i32 => {
                            break 's3;
                        }
                        9i32 => {
                            if ((((((c.m.r32((l_side_2).wrapping_add(520)) as i32)) != (2i32)) as i32)) != 0) {
                                { let v_ = 2i32; let a_ = (l_side_2).wrapping_add(520); c.m.w32(a_, (v_) as u32); };
                                {
                                    l_j_4 = 0i32;
                                    'l4: loop {
                                        if !(((((l_j_4) < (5i32)) as i32)) != 0) { break 'l4; }
                                        'c5: {
                                            l_m_5 = ((((F::Battle_GetSetup(c)).wrapping_add(192)).wrapping_add((((l_i_3) as u32)).wrapping_mul(624))).wrapping_add(4)).wrapping_add((((l_j_4) as u32)).wrapping_mul(100));
                                            { let v_ = 29i32; let a_ = (l_m_5).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                                            { let v_ = 0i32; let a_ = (l_m_5).wrapping_add(68); c.m.w32(a_, (v_) as u32); };
                                        }
                                        { let o_ = l_j_4; let n_ = (o_).wrapping_add(1); l_j_4 = n_; };
                                    }
                                }
                            }
                            break 's3;
                        }
                        6i32 => {
                            if ((((((c.m.r32(l_side_2) as i32)) != (1i32)) as i32)) != 0) {
                                { let v_ = 1i32; let a_ = l_side_2; c.m.w32(a_, (v_) as u32); };
                            }
                            if (((((l_i_3) == (0i32)) as i32)) != 0) {
                                if ((((((c.m.r32((l_side_2).wrapping_add(520)) as i32)) != (0i32)) as i32)) != 0) {
                                    { let v_ = 0i32; let a_ = (l_side_2).wrapping_add(520); c.m.w32(a_, (v_) as u32); };
                                }
                            }
                            if (((((l_i_3) == (1i32)) as i32)) != 0) {
                                if ((((((c.m.r32((l_side_2).wrapping_add(520)) as i32)) != (2i32)) as i32)) != 0) {
                                    { let v_ = 2i32; let a_ = (l_side_2).wrapping_add(520); c.m.w32(a_, (v_) as u32); };
                                }
                                if ((((((c.m.r32((((l_side_2).wrapping_add(4)).wrapping_add((((0i32) as u32)).wrapping_mul(100))).wrapping_add(12)) as i32)) != ((1i32).wrapping_neg())) as i32)) != 0) {
                                    { let v_ = (1i32).wrapping_neg(); let a_ = (((l_side_2).wrapping_add(4)).wrapping_add((((0i32) as u32)).wrapping_mul(100))).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                                }
                            }
                            break 's3;
                        }
                        4i32 => {
                            if ((((((c.m.r32(l_side_2) as i32)) != (1i32)) as i32)) != 0) {
                                { let v_ = 1i32; let a_ = l_side_2; c.m.w32(a_, (v_) as u32); };
                            }
                            break 's3;
                        }
                        3i32 => {
                            if (((((l_i_3) == (0i32)) as i32)) != 0) {
                                if ((((((c.m.r32(l_side_2) as i32)) != (1i32)) as i32)) != 0) {
                                    { let v_ = 1i32; let a_ = l_side_2; c.m.w32(a_, (v_) as u32); };
                                }
                                if ((((((c.m.r32((l_side_2).wrapping_add(520)) as i32)) != (0i32)) as i32)) != 0) {
                                    { let v_ = 0i32; let a_ = (l_side_2).wrapping_add(520); c.m.w32(a_, (v_) as u32); };
                                }
                            }
                            if (((((l_i_3) == (1i32)) as i32)) != 0) {
                                if ((((((c.m.r32((l_side_2).wrapping_add(520)) as i32)) != (2i32)) as i32)) != 0) {
                                    { let v_ = 2i32; let a_ = (l_side_2).wrapping_add(520); c.m.w32(a_, (v_) as u32); };
                                }
                            }
                            break 's3;
                        }
                        7i32 => {
                            if ((((((c.m.r32((l_side_2).wrapping_add(520)) as i32)) != (2i32)) as i32)) != 0) {
                                { let v_ = 2i32; let a_ = (l_side_2).wrapping_add(520); c.m.w32(a_, (v_) as u32); };
                            }
                            break 's3;
                        }
                        _ => {
                        }
                    }
                }
            }
            { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
        }
    }
    if (((((F::Battle_GetMode(c)) == (9i32)) as i32)) != 0) {
        { let v_ = 6i32; let a_ = (F::Battle_GetSetup(c)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BattleSetup_FixForMode", &PROBE_N, &[], true); }
    r_
}

pub fn BattleSetup_InitCharaBits(c: &mut Ctx, mut p_dst_1: u32, mut p_src_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BattleSetup_InitCharaBits", &PROBE_N, &[(p_dst_1) as u32, (p_src_2) as u32], false); }
    let r_ = 'f: {
    if (((((p_src_2) != (((0i32) as u32))) as i32)) != 0) {
        { let s_ = p_src_2; let d_ = p_dst_1; rt::memcpy(c, d_, s_, 64); };
    } else {
        { let _ = F::memset(c, p_dst_1, 0i32, 64u32); };
        { let v_ = { let a_ = ((c.m.r32(sym::gSaveData)).wrapping_add(3088)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.r64(a_) }; let a_ = (p_dst_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = { let a_ = ((c.m.r32(sym::gSaveData)).wrapping_add(3088)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.r64(a_) }; let a_ = (p_dst_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = { let a_ = ((c.m.r32(sym::gSaveData)).wrapping_add(3088)).wrapping_add((((2i32) as u32)).wrapping_mul(8)); c.m.r64(a_) }; let a_ = (p_dst_1).wrapping_add((((2i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BattleSetup_InitCharaBits", &PROBE_N, &[], true); }
    r_
}

pub fn BattleSetup_DefaultOptions(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BattleSetup_DefaultOptions", &PROBE_N, &[], false); }
    let mut l_opt_1: u32 = 0;
    let r_ = 'f: {
    l_opt_1 = (F::Battle_GetSetup(c)).wrapping_add(44);
    if ((((((c.m.r32(l_opt_1) as i32)) == (0i32)) as i32)) != 0) {
        { let _ = F::memset(c, l_opt_1, 0i32, 144u32); };
        { let v_ = { let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5780); (c.m.r32(a_) as i32) }; let a_ = ((l_opt_1).wrapping_add(4)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = { let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5780); (c.m.r32(a_) as i32) }; let a_ = ((l_opt_1).wrapping_add(4)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = { let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5784); (c.m.r32(a_) as i32) }; let a_ = ((l_opt_1).wrapping_add(12)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = { let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5784); (c.m.r32(a_) as i32) }; let a_ = ((l_opt_1).wrapping_add(12)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BattleSetup_DefaultOptions", &PROBE_N, &[], true); }
    r_
}

pub fn BattleSetup_Clear(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BattleSetup_Clear", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let _ = { let a0_ = F::Battle_GetSetup(c); F::memset(c, a0_, 0i32, 1448u32) }; };
    { let v_ = ((98i32) as u8); let a_ = (F::Battle_GetSetup(c)).wrapping_add(((0i32) as u32)); c.m.w8(a_, v_); };
    { let v_ = ((116i32) as u8); let a_ = (F::Battle_GetSetup(c)).wrapping_add(((1i32) as u32)); c.m.w8(a_, v_); };
    { let v_ = ((108i32) as u8); let a_ = (F::Battle_GetSetup(c)).wrapping_add(((2i32) as u32)); c.m.w8(a_, v_); };
    { let v_ = ((115i32) as u8); let a_ = (F::Battle_GetSetup(c)).wrapping_add(((3i32) as u32)); c.m.w8(a_, v_); };
    { let v_ = 7i32; let a_ = (F::Battle_GetSetup(c)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BattleSetup_Clear", &PROBE_N, &[], true); }
    r_
}

pub fn BattleReplay_ClearDataFlag(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BattleReplay_ClearDataFlag", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let r_ = (!(1i32)); let a_ = (F::BattleReplay_GetData(c)).wrapping_add(108016); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BattleReplay_ClearDataFlag", &PROBE_N, &[], true); }
    r_
}

pub fn BattleSetup_SetScript(c: &mut Ctx, mut p_script_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BattleSetup_SetScript", &PROBE_N, &[(p_script_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = (p_script_1).wrapping_add(1i32); let a_ = (F::Battle_GetSetup(c)).wrapping_add(188); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BattleSetup_SetScript", &PROBE_N, &[], true); }
    r_
}

pub fn BattleSetup_SetOptions(c: &mut Ctx, mut p_optA0_1: i32, mut p_optA1_2: i32, mut p_optB0_3: i32, mut p_optB1_4: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BattleSetup_SetOptions", &PROBE_N, &[(p_optA0_1) as u32, (p_optA1_2) as u32, (p_optB0_3) as u32, (p_optB1_4) as u32], false); }
    let mut l_opt_5: u32 = 0;
    let r_ = 'f: {
    l_opt_5 = (F::Battle_GetSetup(c)).wrapping_add(44);
    { let _ = F::memset(c, l_opt_5, 0i32, 144u32); };
    { let v_ = 1i32; let a_ = l_opt_5; c.m.w32(a_, (v_) as u32); };
    { let v_ = p_optA0_1; let a_ = ((l_opt_5).wrapping_add(4)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    { let v_ = p_optA1_2; let a_ = ((l_opt_5).wrapping_add(4)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    { let v_ = p_optB0_3; let a_ = ((l_opt_5).wrapping_add(12)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    { let v_ = p_optB1_4; let a_ = ((l_opt_5).wrapping_add(12)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BattleSetup_SetOptions", &PROBE_N, &[], true); }
    r_
}

pub fn BattleSetup_SetOption14(c: &mut Ctx, mut p_val_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BattleSetup_SetOption14", &PROBE_N, &[(p_val_1) as u32], false); }
    let mut l_opt_2: u32 = 0;
    let r_ = 'f: {
    l_opt_2 = (F::Battle_GetSetup(c)).wrapping_add(44);
    { let _ = F::memset(c, l_opt_2, 0i32, 144u32); };
    { let v_ = 1i32; let a_ = l_opt_2; c.m.w32(a_, (v_) as u32); };
    { let v_ = p_val_1; let a_ = (l_opt_2).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
    { let v_ = { let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5780); (c.m.r32(a_) as i32) }; let a_ = ((l_opt_2).wrapping_add(4)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    { let v_ = { let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5780); (c.m.r32(a_) as i32) }; let a_ = ((l_opt_2).wrapping_add(4)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    { let v_ = { let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5784); (c.m.r32(a_) as i32) }; let a_ = ((l_opt_2).wrapping_add(12)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    { let v_ = { let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5784); (c.m.r32(a_) as i32) }; let a_ = ((l_opt_2).wrapping_add(12)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BattleSetup_SetOption14", &PROBE_N, &[], true); }
    r_
}

pub fn BattleSetup_SetRule(c: &mut Ctx, mut p_screenMode_1: i32, mut p_mode_2: i32, mut p_bgm_3: i32, mut p_timeLimit_4: i32, mut p_announcer_5: i32, mut p_stage_6: i32, mut p_unk10_7: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BattleSetup_SetRule", &PROBE_N, &[(p_screenMode_1) as u32, (p_mode_2) as u32, (p_bgm_3) as u32, (p_timeLimit_4) as u32, (p_announcer_5) as u32, (p_stage_6) as u32, (p_unk10_7) as u32], false); }
    let mut l_rule_8: u32 = 0;
    let r_ = 'f: {
    l_rule_8 = (F::Battle_GetSetup(c)).wrapping_add(8);
    {
        if (((((F::Port_TestScreen(c)) >= (0i32)) as i32)) != 0) {
            p_screenMode_1 = F::Port_TestScreen(c);
        }
    }
    { let v_ = p_screenMode_1; let a_ = (l_rule_8).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
    { let v_ = p_mode_2; let a_ = l_rule_8; c.m.w32(a_, (v_) as u32); };
    {
        if (((((F::Port_TestBgm(c)) >= (0i32)) as i32)) != 0) {
            p_bgm_3 = F::Port_TestBgm(c);
        }
    }
    if (((((p_bgm_3) == (24i32)) as i32)) != 0) {
        { let v_ = rt::rem_i32(F::Port_Rand(c), 24i32); let a_ = (l_rule_8).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
    } else {
        { let v_ = p_bgm_3; let a_ = (l_rule_8).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
    }
    { let v_ = p_timeLimit_4; let a_ = (l_rule_8).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
    { let v_ = p_announcer_5; let a_ = (l_rule_8).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
    { let v_ = p_unk10_7; let a_ = (l_rule_8).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
    {
        if (((((F::Port_TestStage(c)) >= (0i32)) as i32)) != 0) {
            p_stage_6 = F::Port_TestStage(c);
        }
    }
    { let v_ = p_stage_6; let a_ = (l_rule_8).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
    { let v_ = p_stage_6; let a_ = (l_rule_8).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BattleSetup_SetRule", &PROBE_N, &[], true); }
    r_
}

pub fn BattleSetup_SetSide(c: &mut Ctx, mut p_sideNo_1: i32, mut p_control_2: i32, mut p_pad_3: i32, mut p_memberCount_4: i32, mut p_unk1FC_5: i32, mut p_unk200_6: i32, mut p_lead_7: i32, mut p_bits_8: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BattleSetup_SetSide", &PROBE_N, &[(p_sideNo_1) as u32, (p_control_2) as u32, (p_pad_3) as u32, (p_memberCount_4) as u32, (p_unk1FC_5) as u32, (p_unk200_6) as u32, (p_lead_7) as u32, (p_bits_8) as u32], false); }
    let mut l_side_9: u32 = 0;
    let r_ = 'f: {
    l_side_9 = F::battle_load__Side(c, p_sideNo_1);
    { let v_ = p_control_2; let a_ = (l_side_9).wrapping_add(520); c.m.w32(a_, (v_) as u32); };
    { let v_ = p_pad_3; let a_ = (l_side_9).wrapping_add(516); c.m.w32(a_, (v_) as u32); };
    { let v_ = p_memberCount_4; let a_ = l_side_9; c.m.w32(a_, (v_) as u32); };
    { let v_ = p_unk200_6; let a_ = (l_side_9).wrapping_add(512); c.m.w32(a_, (v_) as u32); };
    { let v_ = p_unk1FC_5; let a_ = (l_side_9).wrapping_add(508); c.m.w32(a_, (v_) as u32); };
    { let v_ = p_lead_7; let a_ = (l_side_9).wrapping_add(504); c.m.w32(a_, (v_) as u32); };
    F::BattleSetup_InitCharaBits(c, (l_side_9).wrapping_add(528), p_bits_8);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BattleSetup_SetSide", &PROBE_N, &[], true); }
    r_
}

pub fn BattleSetup_SetMemberByItemIds(c: &mut Ctx, mut p_sideNo_1: i32, mut p_idx_2: i32, mut p_chara_3: i32, mut p_costume_4: i32, mut p_variant_5: i32, mut p_cpuLevel_6: i32, mut p_health_7: F32, mut p_itemIds_8: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BattleSetup_SetMemberByItemIds", &PROBE_N, &[(p_sideNo_1) as u32, (p_idx_2) as u32, (p_chara_3) as u32, (p_costume_4) as u32, (p_variant_5) as u32, (p_cpuLevel_6) as u32, (p_health_7).0, (p_itemIds_8) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_m_9: u32 = 0;
    let mut l_i_10: i32 = 0;
    let r_ = 'f: {
        l_m_9 = F::battle_load__Member(c, p_sideNo_1, p_idx_2);
        { let _ = F::memset(c, fp, 0i32, 16u32); };
        {
            l_i_10 = 0i32;
            'l1: loop {
                if !(((((l_i_10) < (8i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let v_ = (((((c.m.r16((p_itemIds_8).wrapping_add((((l_i_10) as u32)).wrapping_mul(4)))) as i32)).wrapping_add(1i32)) as u16); let a_ = (fp).wrapping_add((((l_i_10) as u32)).wrapping_mul(2)); c.m.w16(a_, v_); };
                }
                { let o_ = l_i_10; let n_ = (o_).wrapping_add(1); l_i_10 = n_; };
            }
        }
        F::BtlMember_Init(c, l_m_9, p_chara_3, p_costume_4, p_variant_5, p_cpuLevel_6, p_health_7, fp);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BattleSetup_SetMemberByItemIds", &PROBE_N, &[], true); }
    r_
}

pub fn BattleSetup_SetMember(c: &mut Ctx, mut p_sideNo_1: i32, mut p_idx_2: i32, mut p_chara_3: i32, mut p_costume_4: i32, mut p_variant_5: i32, mut p_cpuLevel_6: i32, mut p_health_7: F32, mut p_items_8: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BattleSetup_SetMember", &PROBE_N, &[(p_sideNo_1) as u32, (p_idx_2) as u32, (p_chara_3) as u32, (p_costume_4) as u32, (p_variant_5) as u32, (p_cpuLevel_6) as u32, (p_health_7).0, (p_items_8) as u32], false); }
    let r_ = 'f: {
    { let a0_ = F::battle_load__Member(c, p_sideNo_1, p_idx_2); F::BtlMember_Init(c, a0_, p_chara_3, p_costume_4, p_variant_5, p_cpuLevel_6, p_health_7, p_items_8) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BattleSetup_SetMember", &PROBE_N, &[], true); }
    r_
}

pub fn BattleSetup_SetPoolMember(c: &mut Ctx, mut p_count_1: i32, mut p_idx_2: i32, mut p_chara_3: i32, mut p_costume_4: i32, mut p_variant_5: i32, mut p_cpuLevel_6: i32, mut p_health_7: F32, mut p_items_8: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BattleSetup_SetPoolMember", &PROBE_N, &[(p_count_1) as u32, (p_idx_2) as u32, (p_chara_3) as u32, (p_costume_4) as u32, (p_variant_5) as u32, (p_cpuLevel_6) as u32, (p_health_7).0, (p_items_8) as u32], false); }
    let mut l_pool_9: u32 = 0;
    let r_ = 'f: {
    l_pool_9 = F::Battle_GetWork5A8(c);
    { let v_ = p_count_1; let a_ = (l_pool_9).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
    { let v_ = 0i32; let a_ = l_pool_9; c.m.w32(a_, (v_) as u32); };
    F::BtlMember_Init(c, ((l_pool_9).wrapping_add(8)).wrapping_add((((p_idx_2) as u32)).wrapping_mul(100)), p_chara_3, p_costume_4, p_variant_5, p_cpuLevel_6, p_health_7, p_items_8);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BattleSetup_SetPoolMember", &PROBE_N, &[], true); }
    r_
}

pub fn BattleSetup_Finish(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BattleSetup_Finish", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::BattleSetup_FinishEx(c, 0i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BattleSetup_Finish", &PROBE_N, &[], true); }
    r_
}

pub fn BattleSetup_FinishEx(c: &mut Ctx, mut p_wide_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BattleSetup_FinishEx", &PROBE_N, &[(p_wide_1) as u32], false); }
    let mut l_cpuWide_2: i32 = 0;
    let mut l_i_3: i32 = 0;
    let mut l_j_4: i32 = 0;
    let mut l_side_5: u32 = 0;
    let mut l_m_6: u32 = 0;
    let mut l_rule_7: u32 = 0;
    let mut l_setup_8: u32 = 0;
    let mut l_pool_9: u32 = 0;
    let mut l_mask_10: i32 = 0;
    let r_ = 'f: {
    F::BattleSetup_FixForMode(c);
    's1: {
        let sv11: i32 = F::Battle_GetMode(c);
        match sv11 {
            1i32 | 2i32 | 3i32 | 4i32 => {
                l_cpuWide_2 = 1i32;
                break 's1;
            }
            _ => {
                l_cpuWide_2 = 0i32;
                break 's1;
            }
        }
    }
    if (((((F::Battle_GetMode(c)) == (3i32)) as i32)) != 0) {
        l_pool_9 = F::Battle_GetWork5A8(c);
        { let v_ = 0i32; let a_ = l_pool_9; c.m.w32(a_, (v_) as u32); };
        {
            l_j_4 = 0i32;
            'l2: loop {
                if !(((((l_j_4) < (50i32)) as i32)) != 0) { break 'l2; }
                'c3: {
                    F::BtlMember_ClampBonus(c, ((l_pool_9).wrapping_add(8)).wrapping_add((((l_j_4) as u32)).wrapping_mul(100)), 1i32);
                }
                { let o_ = l_j_4; let n_ = (o_).wrapping_add(1); l_j_4 = n_; };
            }
        }
        { let s_ = ((l_pool_9).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(100)); let d_ = ((((F::Battle_GetSetup(c)).wrapping_add(192)).wrapping_add((((1i32) as u32)).wrapping_mul(624))).wrapping_add(4)).wrapping_add((((0i32) as u32)).wrapping_mul(100)); rt::memcpy(c, d_, s_, 100); };
    }
    {
        l_i_3 = 0i32;
        'l4: loop {
            if !(((((l_i_3) < (2i32)) as i32)) != 0) { break 'l4; }
            'c5: {
                l_setup_8 = F::Battle_GetSetup(c);
                l_side_5 = ((l_setup_8).wrapping_add(192)).wrapping_add((((l_i_3) as u32)).wrapping_mul(624));
                { let v_ = { let a_ = ((l_side_5).wrapping_add(4)).wrapping_add(((((c.m.r32((l_side_5).wrapping_add(504)) as i32)) as u32)).wrapping_mul(100)); (c.m.r32(a_) as i32) }; let a_ = (l_side_5).wrapping_add(592); c.m.w32(a_, (v_) as u32); };
                { let v_ = { let a_ = (((l_side_5).wrapping_add(4)).wrapping_add(((((c.m.r32((l_side_5).wrapping_add(504)) as i32)) as u32)).wrapping_mul(100))).wrapping_add(4); (c.m.r32(a_) as i32) }; let a_ = ((l_side_5).wrapping_add(592)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                { let v_ = { let a_ = (((l_side_5).wrapping_add(4)).wrapping_add(((((c.m.r32((l_side_5).wrapping_add(504)) as i32)) as u32)).wrapping_mul(100))).wrapping_add(8); (c.m.r32(a_) as i32) }; let a_ = ((l_side_5).wrapping_add(592)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                { let v_ = (c.m.r32((l_side_5).wrapping_add(592)) as i32); let a_ = (l_side_5).wrapping_add(604); c.m.w32(a_, (v_) as u32); };
                { let v_ = (c.m.r32(((l_side_5).wrapping_add(592)).wrapping_add(4)) as i32); let a_ = ((l_side_5).wrapping_add(604)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                { let v_ = (c.m.r32(((l_side_5).wrapping_add(592)).wrapping_add(8)) as i32); let a_ = ((l_side_5).wrapping_add(604)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                's6: {
                    let sv12: i32 = F::Battle_GetMode(c);
                    'k9: {
                    'k8: {
                    'k7: {
                        match sv12 { 2i32 => break 'k7, 3i32 => break 'k7, 4i32 => break 'k7, 7i32 => break 'k7, 9i32 => break 'k7, 1i32 => break 'k8, 5i32 => break 'k9, 6i32 => break 'k9, 8i32 => break 'k9, _ => break 's6, }
                    }
                    if ((((((c.m.r32((l_side_5).wrapping_add(520)) as i32)) != (2i32)) as i32)) != 0) {
                        break 's6;
                    }
                    }
                    { let v_ = (((1i32).wrapping_neg()) as u64); let a_ = ((l_side_5).wrapping_add(528)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                    { let v_ = (((1i32).wrapping_neg()) as u64); let a_ = ((l_side_5).wrapping_add(528)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                    { let v_ = (((1i32).wrapping_neg()) as u64); let a_ = ((l_side_5).wrapping_add(528)).wrapping_add((((2i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                    break 's6;
                    }
                    break 's6;
                }
                {
                    l_j_4 = 0i32;
                    'l10: loop {
                        if !(((((l_j_4) < (5i32)) as i32)) != 0) { break 'l10; }
                        'c11: {
                            l_m_6 = ((((F::Battle_GetSetup(c)).wrapping_add(192)).wrapping_add((((l_i_3) as u32)).wrapping_mul(624))).wrapping_add(4)).wrapping_add((((l_j_4) as u32)).wrapping_mul(100));
                            if ((((((c.m.r32((l_side_5).wrapping_add(520)) as i32)) == (2i32)) as i32)) != 0) {
                                if ((l_cpuWide_2) != 0) {
                                    F::BtlMember_ClampBonus(c, l_m_6, 1i32);
                                } else {
                                    F::BtlMember_ClampBonus(c, l_m_6, 0i32);
                                }
                            } else {
                                if ((p_wide_1) != 0) {
                                    F::BtlMember_ClampBonus(c, l_m_6, 1i32);
                                } else {
                                    F::BtlMember_ClampBonus(c, l_m_6, 0i32);
                                }
                            }
                        }
                        { let o_ = l_j_4; let n_ = (o_).wrapping_add(1); l_j_4 = n_; };
                    }
                }
            }
            { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
        }
    }
    l_rule_7 = (F::Battle_GetSetup(c)).wrapping_add(8);
    { let v_ = 0i32; let a_ = (l_rule_7).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
    if (((((F::Battle_GetMode(c)) == (1i32)) as i32)) != 0) {
        {
            l_i_3 = 0i32;
            'l12: loop {
                if !(((((l_i_3) < (7i32)) as i32)) != 0) { break 'l12; }
                'c13: {
                    l_mask_10 = (1i32).wrapping_shl((l_i_3) as u32);
                    if ((((!(((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(8); (c.m.r32(a_) as i32) }) & (l_mask_10))) != 0)) as i32)) != 0) {
                        { let v_ = (l_i_3).wrapping_add(1i32); let a_ = (l_rule_7).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                        break 'l12;
                    }
                }
                { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
            }
        }
    }
    F::BattleSetup_DefaultOptions(c);
    { let v_ = 1i32; let a_ = (F::Battle_GetSetup(c)).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
    { let _ = F::memset(c, sym::gBattleReplay, 0i32, 109480u32); };
    { let s_ = F::Battle_GetSetup(c); let d_ = sym::gBattleReplay; rt::memcpy(c, d_, s_, 1448); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BattleSetup_FinishEx", &PROBE_N, &[], true); }
    r_
}

pub fn BattleReplay_GetBuffer(c: &mut Ctx, mut p_size_1: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BattleReplay_GetBuffer", &PROBE_N, &[(p_size_1) as u32], false); }
    let mut l_rep_2: u32 = 0;
    let r_ = 'f: {
    if (((((p_size_1) != (((0i32) as u32))) as i32)) != 0) {
        { let v_ = ((109480u32) as i32); let a_ = p_size_1; c.m.w32(a_, (v_) as u32); };
    }
    l_rep_2 = sym::gBattleReplay;
    { let v_ = 0i32; let a_ = (l_rep_2).wrapping_add(109472); c.m.w32(a_, (v_) as u32); };
    { let v_ = 0i32; let a_ = (l_rep_2).wrapping_add(109476); c.m.w32(a_, (v_) as u32); };
    break 'f l_rep_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BattleReplay_GetBuffer", &PROBE_N, &[], true); }
    r_
}

pub fn BattleReplay_Load(c: &mut Ctx, mut p_buf_1: u32, mut p_size_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BattleReplay_Load", &PROBE_N, &[(p_buf_1) as u32, (p_size_2) as u32], false); }
    let r_ = 'f: {
    if (((((p_buf_1) != (((0i32) as u32))) as i32)) != 0) {
        if (((((((p_size_2) as u32)) == (109480u32)) as i32)) != 0) {
            if (((((c.m.r64(p_buf_1)) == ((((((7i32) as u64)).wrapping_shl((32i32) as u32)) | (((1936487522i32) as u64))))) as i32)) != 0) {
                { let s_ = p_buf_1; let d_ = F::Battle_GetSetup(c); rt::memcpy(c, d_, s_, 1448); };
                { let _ = F::memset(c, sym::gBattleReplay, 0i32, 109480u32); };
                { let s_ = p_buf_1; let d_ = sym::gBattleReplay; rt::memcpy(c, d_, s_, 1448); };
                { let _ = F::memcpy(c, (sym::gBattleReplay).wrapping_add(1448), (p_buf_1).wrapping_add(1448), (108016u32).wrapping_add(4u32)); };
                { let v_ = 1i32; let a_ = (sym::gBattleReplay).wrapping_add(109468); c.m.w32(a_, (v_) as u32); };
                { let v_ = 1i32; let a_ = (sym::gBattleReplay).wrapping_add(109472); c.m.w32(a_, (v_) as u32); };
                break 'f 1i32;
            }
        }
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BattleReplay_Load", &PROBE_N, &[], true); }
    r_
}

pub fn BattleReplay_IsActive(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BattleReplay_IsActive", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f (c.m.r32((sym::gBattleReplay).wrapping_add(109468)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BattleReplay_IsActive", &PROBE_N, &[], true); }
    r_
}

pub fn BattleReplay_IsLoaded(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BattleReplay_IsLoaded", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f (c.m.r32((sym::gBattleReplay).wrapping_add(109472)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BattleReplay_IsLoaded", &PROBE_N, &[], true); }
    r_
}

pub fn BattleReplay_GetData(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BattleReplay_GetData", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f (sym::gBattleReplay).wrapping_add(1448);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BattleReplay_GetData", &PROBE_N, &[], true); }
    r_
}

pub fn BattleReplay_TestDataFlag(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BattleReplay_TestDataFlag", &PROBE_N, &[], false); }
    let mut l_data_1: u32 = 0;
    let r_ = 'f: {
    l_data_1 = F::BattleReplay_GetData(c);
    if (((((F::BattleReplay_IsActive(c)) == (0i32)) as i32)) != 0) {
        break 'f 1i32;
    }
    if ((((((((c.m.r32((l_data_1).wrapping_add(108016)) as i32)) & (1i32))) != (0i32)) as i32)) != 0) {
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BattleReplay_TestDataFlag", &PROBE_N, &[], true); }
    r_
}

pub fn BattleReplay_SetActive(c: &mut Ctx, mut p_active_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BattleReplay_SetActive", &PROBE_N, &[(p_active_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_active_1; let a_ = (sym::gBattleReplay).wrapping_add(109468); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BattleReplay_SetActive", &PROBE_N, &[], true); }
    r_
}

pub fn Battle_GetHumanSide(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Battle_GetHumanSide", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let r_ = 'f: {
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (2i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((((({ let a_ = (F::battle_load__Side(c, l_i_1)).wrapping_add(520); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
                    break 'f l_i_1;
                }
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
    break 'f (1i32).wrapping_neg();
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Battle_GetHumanSide", &PROBE_N, &[], true); }
    r_
}

pub fn Battle_GetOption14(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Battle_GetOption14", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f { let a_ = ((F::Battle_GetSetup(c)).wrapping_add(44)).wrapping_add(20); (c.m.r32(a_) as i32) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Battle_GetOption14", &PROBE_N, &[], true); }
    r_
}

pub fn Battle_GetScript(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Battle_GetScript", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f ({ let a_ = (F::Battle_GetSetup(c)).wrapping_add(188); (c.m.r32(a_) as i32) }).wrapping_sub(1i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Battle_GetScript", &PROBE_N, &[], true); }
    r_
}

pub fn Battle_IsSplitScreen(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Battle_IsSplitScreen", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f ((({ let a_ = ((F::Battle_GetSetup(c)).wrapping_add(8)).wrapping_add(28); (c.m.r32(a_) as i32) }) == (1i32)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Battle_IsSplitScreen", &PROBE_N, &[], true); }
    r_
}

pub fn Battle_GetMode(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Battle_GetMode", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f { let a_ = (F::Battle_GetSetup(c)).wrapping_add(8); (c.m.r32(a_) as i32) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Battle_GetMode", &PROBE_N, &[], true); }
    r_
}

pub fn Battle_GetTimeLimit(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Battle_GetTimeLimit", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f (({ let a_ = (sym::gBattleTimeLimitTbl).wrapping_add(((({ let a_ = ((F::Battle_GetSetup(c)).wrapping_add(8)).wrapping_add(8); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(2)); (c.m.r16(a_) as i16) }) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Battle_GetTimeLimit", &PROBE_N, &[], true); }
    r_
}

pub fn Battle_IsTimeLimitOff(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Battle_IsTimeLimitOff", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f ((({ let a_ = ((F::Battle_GetSetup(c)).wrapping_add(8)).wrapping_add(8); (c.m.r32(a_) as i32) }) == (0i32)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Battle_IsTimeLimitOff", &PROBE_N, &[], true); }
    r_
}

pub fn Battle_GetAnnouncer(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Battle_GetAnnouncer", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f { let a_ = ((F::Battle_GetSetup(c)).wrapping_add(8)).wrapping_add(12); (c.m.r32(a_) as i32) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Battle_GetAnnouncer", &PROBE_N, &[], true); }
    r_
}

pub fn Battle_GetBgm(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Battle_GetBgm", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f { let a_ = ((F::Battle_GetSetup(c)).wrapping_add(8)).wrapping_add(4); (c.m.r32(a_) as i32) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Battle_GetBgm", &PROBE_N, &[], true); }
    r_
}

pub fn Battle_GetStartStage(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Battle_GetStartStage", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f { let a_ = ((F::Battle_GetSetup(c)).wrapping_add(8)).wrapping_add(20); (c.m.r32(a_) as i32) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Battle_GetStartStage", &PROBE_N, &[], true); }
    r_
}

pub fn Battle_GetStage(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Battle_GetStage", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f { let a_ = ((F::Battle_GetSetup(c)).wrapping_add(8)).wrapping_add(32); (c.m.r32(a_) as i32) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Battle_GetStage", &PROBE_N, &[], true); }
    r_
}

pub fn Battle_SetStage(c: &mut Ctx, mut p_stage_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Battle_SetStage", &PROBE_N, &[(p_stage_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_stage_1; let a_ = ((F::Battle_GetSetup(c)).wrapping_add(8)).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Battle_SetStage", &PROBE_N, &[], true); }
    r_
}

pub fn Battle_IsStageChanged(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Battle_IsStageChanged", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f (((F::Battle_GetStartStage(c)) != (F::Battle_GetStage(c))) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Battle_IsStageChanged", &PROBE_N, &[], true); }
    r_
}

pub fn Battle_ResetStage(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Battle_ResetStage", &PROBE_N, &[], false); }
    let mut l_rule_1: u32 = 0;
    let r_ = 'f: {
    l_rule_1 = (F::Battle_GetSetup(c)).wrapping_add(8);
    { let v_ = { let a_ = ((F::Battle_GetSetup(c)).wrapping_add(8)).wrapping_add(20); (c.m.r32(a_) as i32) }; let a_ = (l_rule_1).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Battle_ResetStage", &PROBE_N, &[], true); }
    r_
}

pub fn Battle_GetRuleUnk10(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Battle_GetRuleUnk10", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f { let a_ = ((F::Battle_GetSetup(c)).wrapping_add(8)).wrapping_add(16); (c.m.r32(a_) as i32) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Battle_GetRuleUnk10", &PROBE_N, &[], true); }
    r_
}

pub fn BattleSide_GetOptionA(c: &mut Ctx, mut p_side_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BattleSide_GetOptionA", &PROBE_N, &[(p_side_1) as u32], false); }
    let mut l_p_2: u32 = 0;
    let r_ = 'f: {
    l_p_2 = ((F::Battle_GetSetup(c)).wrapping_add(44)).wrapping_add(4);
    break 'f (c.m.r32((l_p_2).wrapping_add((((p_side_1) as u32)).wrapping_mul(4))) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BattleSide_GetOptionA", &PROBE_N, &[], true); }
    r_
}

pub fn BattleSide_GetOptionB(c: &mut Ctx, mut p_side_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BattleSide_GetOptionB", &PROBE_N, &[(p_side_1) as u32], false); }
    let mut l_p_2: u32 = 0;
    let r_ = 'f: {
    l_p_2 = ((F::Battle_GetSetup(c)).wrapping_add(44)).wrapping_add(12);
    break 'f (c.m.r32((l_p_2).wrapping_add((((p_side_1) as u32)).wrapping_mul(4))) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BattleSide_GetOptionB", &PROBE_N, &[], true); }
    r_
}

pub fn BattleSide_GetPad(c: &mut Ctx, mut p_side_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BattleSide_GetPad", &PROBE_N, &[(p_side_1) as u32], false); }
    let r_ = 'f: {
    break 'f { let a_ = (F::battle_load__Side(c, p_side_1)).wrapping_add(516); (c.m.r32(a_) as i32) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BattleSide_GetPad", &PROBE_N, &[], true); }
    r_
}

pub fn BattleSide_GetControl(c: &mut Ctx, mut p_side_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BattleSide_GetControl", &PROBE_N, &[(p_side_1) as u32], false); }
    let r_ = 'f: {
    break 'f { let a_ = (F::battle_load__Side(c, p_side_1)).wrapping_add(520); (c.m.r32(a_) as i32) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BattleSide_GetControl", &PROBE_N, &[], true); }
    r_
}

pub fn BattleSide_IsCpu(c: &mut Ctx, mut p_side_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BattleSide_IsCpu", &PROBE_N, &[(p_side_1) as u32], false); }
    let r_ = 'f: {
    break 'f ((({ let a_ = (F::battle_load__Side(c, p_side_1)).wrapping_add(520); (c.m.r32(a_) as i32) }) == (2i32)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BattleSide_IsCpu", &PROBE_N, &[], true); }
    r_
}

pub fn BattleSide_GetUnk200(c: &mut Ctx, mut p_side_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BattleSide_GetUnk200", &PROBE_N, &[(p_side_1) as u32], false); }
    let r_ = 'f: {
    break 'f { let a_ = (F::battle_load__Side(c, p_side_1)).wrapping_add(512); (c.m.r32(a_) as i32) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BattleSide_GetUnk200", &PROBE_N, &[], true); }
    r_
}

pub fn BattleSide_GetUnk1FC(c: &mut Ctx, mut p_side_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BattleSide_GetUnk1FC", &PROBE_N, &[(p_side_1) as u32], false); }
    let r_ = 'f: {
    break 'f { let a_ = (F::battle_load__Side(c, p_side_1)).wrapping_add(508); (c.m.r32(a_) as i32) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BattleSide_GetUnk1FC", &PROBE_N, &[], true); }
    r_
}

pub fn BattleSide_GetStartChara(c: &mut Ctx, mut p_side_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BattleSide_GetStartChara", &PROBE_N, &[(p_side_1) as u32], false); }
    let r_ = 'f: {
    break 'f { let a_ = (F::battle_load__Side(c, p_side_1)).wrapping_add(592); (c.m.r32(a_) as i32) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BattleSide_GetStartChara", &PROBE_N, &[], true); }
    r_
}

pub fn BattleSide_GetStartCostume(c: &mut Ctx, mut p_side_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BattleSide_GetStartCostume", &PROBE_N, &[(p_side_1) as u32], false); }
    let r_ = 'f: {
    break 'f { let a_ = ((F::battle_load__Side(c, p_side_1)).wrapping_add(592)).wrapping_add(4); (c.m.r32(a_) as i32) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BattleSide_GetStartCostume", &PROBE_N, &[], true); }
    r_
}

pub fn BattleSide_GetStartVariant(c: &mut Ctx, mut p_side_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BattleSide_GetStartVariant", &PROBE_N, &[(p_side_1) as u32], false); }
    let r_ = 'f: {
    break 'f { let a_ = ((F::battle_load__Side(c, p_side_1)).wrapping_add(592)).wrapping_add(8); (c.m.r32(a_) as i32) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BattleSide_GetStartVariant", &PROBE_N, &[], true); }
    r_
}

pub fn BattleSide_GetChara(c: &mut Ctx, mut p_side_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BattleSide_GetChara", &PROBE_N, &[(p_side_1) as u32], false); }
    let r_ = 'f: {
    break 'f { let a_ = (F::battle_load__Side(c, p_side_1)).wrapping_add(604); (c.m.r32(a_) as i32) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BattleSide_GetChara", &PROBE_N, &[], true); }
    r_
}

pub fn BattleSide_SetChara(c: &mut Ctx, mut p_side_1: i32, mut p_chara_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BattleSide_SetChara", &PROBE_N, &[(p_side_1) as u32, (p_chara_2) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_chara_2; let a_ = (F::battle_load__Side(c, p_side_1)).wrapping_add(604); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BattleSide_SetChara", &PROBE_N, &[], true); }
    r_
}

pub fn BattleSide_IsCharaChanged(c: &mut Ctx, mut p_side_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BattleSide_IsCharaChanged", &PROBE_N, &[(p_side_1) as u32], false); }
    let r_ = 'f: {
    break 'f (((F::BattleSide_GetStartChara(c, p_side_1)) != (F::BattleSide_GetChara(c, p_side_1))) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BattleSide_IsCharaChanged", &PROBE_N, &[], true); }
    r_
}

pub fn BattleSide_ResetChara(c: &mut Ctx, mut p_side_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BattleSide_ResetChara", &PROBE_N, &[(p_side_1) as u32], false); }
    let r_ = 'f: {
    { let a1_ = F::BattleSide_GetStartChara(c, p_side_1); F::BattleSide_SetChara(c, p_side_1, a1_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BattleSide_ResetChara", &PROBE_N, &[], true); }
    r_
}

pub fn BattleSide_GetMemberChara(c: &mut Ctx, mut p_side_1: i32, mut p_idx_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BattleSide_GetMemberChara", &PROBE_N, &[(p_side_1) as u32, (p_idx_2) as u32], false); }
    let r_ = 'f: {
    break 'f { let a_ = F::battle_load__Member(c, p_side_1, p_idx_2); (c.m.r32(a_) as i32) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BattleSide_GetMemberChara", &PROBE_N, &[], true); }
    r_
}

pub fn BattleSide_GetMemberCostume(c: &mut Ctx, mut p_side_1: i32, mut p_idx_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BattleSide_GetMemberCostume", &PROBE_N, &[(p_side_1) as u32, (p_idx_2) as u32], false); }
    let r_ = 'f: {
    break 'f { let a_ = (F::battle_load__Member(c, p_side_1, p_idx_2)).wrapping_add(4); (c.m.r32(a_) as i32) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BattleSide_GetMemberCostume", &PROBE_N, &[], true); }
    r_
}

pub fn BattleSide_GetMemberVariant(c: &mut Ctx, mut p_side_1: i32, mut p_idx_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BattleSide_GetMemberVariant", &PROBE_N, &[(p_side_1) as u32, (p_idx_2) as u32], false); }
    let r_ = 'f: {
    break 'f { let a_ = (F::battle_load__Member(c, p_side_1, p_idx_2)).wrapping_add(8); (c.m.r32(a_) as i32) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BattleSide_GetMemberVariant", &PROBE_N, &[], true); }
    r_
}

pub fn BattleSide_GetMemberCount(c: &mut Ctx, mut p_side_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BattleSide_GetMemberCount", &PROBE_N, &[(p_side_1) as u32], false); }
    let r_ = 'f: {
    break 'f { let a_ = F::battle_load__Side(c, p_side_1); (c.m.r32(a_) as i32) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BattleSide_GetMemberCount", &PROBE_N, &[], true); }
    r_
}

pub fn BattleSide_GetObjId(c: &mut Ctx, mut p_side_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BattleSide_GetObjId", &PROBE_N, &[(p_side_1) as u32], false); }
    let r_ = 'f: {
    break 'f { let a_ = (F::battle_load__Side(c, p_side_1)).wrapping_add(616); (c.m.r32(a_) as i32) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BattleSide_GetObjId", &PROBE_N, &[], true); }
    r_
}

pub fn BattleSide_GetModelSlot(c: &mut Ctx, mut p_side_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BattleSide_GetModelSlot", &PROBE_N, &[(p_side_1) as u32], false); }
    let r_ = 'f: {
    break 'f { let a_ = (F::battle_load__Side(c, p_side_1)).wrapping_add(620); (c.m.r32(a_) as i32) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BattleSide_GetModelSlot", &PROBE_N, &[], true); }
    r_
}

pub fn BattleSide_SetObjId(c: &mut Ctx, mut p_side_1: i32, mut p_objId_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BattleSide_SetObjId", &PROBE_N, &[(p_side_1) as u32, (p_objId_2) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_objId_2; let a_ = (F::battle_load__Side(c, p_side_1)).wrapping_add(616); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BattleSide_SetObjId", &PROBE_N, &[], true); }
    r_
}

pub fn BattleSide_SetModelSlot(c: &mut Ctx, mut p_side_1: i32, mut p_slot_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BattleSide_SetModelSlot", &PROBE_N, &[(p_side_1) as u32, (p_slot_2) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_slot_2; let a_ = (F::battle_load__Side(c, p_side_1)).wrapping_add(620); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BattleSide_SetModelSlot", &PROBE_N, &[], true); }
    r_
}

pub fn BattleSide_SetForm(c: &mut Ctx, mut p_side_1: i32, mut p_chara_2: i32, mut p_costume_3: i32, mut p_variant_4: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BattleSide_SetForm", &PROBE_N, &[(p_side_1) as u32, (p_chara_2) as u32, (p_costume_3) as u32, (p_variant_4) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_chara_2; let a_ = (F::battle_load__Side(c, p_side_1)).wrapping_add(604); c.m.w32(a_, (v_) as u32); };
    { let v_ = p_costume_3; let a_ = ((F::battle_load__Side(c, p_side_1)).wrapping_add(604)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
    { let v_ = p_variant_4; let a_ = ((F::battle_load__Side(c, p_side_1)).wrapping_add(604)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BattleSide_SetForm", &PROBE_N, &[], true); }
    r_
}

pub fn BattleSide_IsFormChanged(c: &mut Ctx, mut p_side_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BattleSide_IsFormChanged", &PROBE_N, &[(p_side_1) as u32], false); }
    let r_ = 'f: {
    if ((((({ let a_ = (F::battle_load__Side(c, p_side_1)).wrapping_add(604); (c.m.r32(a_) as i32) }) != ({ let a_ = (F::battle_load__Side(c, p_side_1)).wrapping_add(592); (c.m.r32(a_) as i32) })) as i32)) != 0) {
        break 'f 1i32;
    }
    if ((((({ let a_ = ((F::battle_load__Side(c, p_side_1)).wrapping_add(604)).wrapping_add(4); (c.m.r32(a_) as i32) }) != ({ let a_ = ((F::battle_load__Side(c, p_side_1)).wrapping_add(592)).wrapping_add(4); (c.m.r32(a_) as i32) })) as i32)) != 0) {
        break 'f 1i32;
    }
    break 'f ((({ let a_ = ((F::battle_load__Side(c, p_side_1)).wrapping_add(604)).wrapping_add(8); (c.m.r32(a_) as i32) }) != ({ let a_ = ((F::battle_load__Side(c, p_side_1)).wrapping_add(592)).wrapping_add(8); (c.m.r32(a_) as i32) })) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BattleSide_IsFormChanged", &PROBE_N, &[], true); }
    r_
}

pub fn BattleSide_GetMember(c: &mut Ctx, mut p_side_1: i32, mut p_idx_2: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BattleSide_GetMember", &PROBE_N, &[(p_side_1) as u32, (p_idx_2) as u32], false); }
    let r_ = 'f: {
    break 'f F::battle_load__Member(c, p_side_1, p_idx_2);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BattleSide_GetMember", &PROBE_N, &[], true); }
    r_
}

pub fn BattleSide_SetMemberItems(c: &mut Ctx, mut p_side_1: i32, mut p_idx_2: i32, mut p_items_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BattleSide_SetMemberItems", &PROBE_N, &[(p_side_1) as u32, (p_idx_2) as u32, (p_items_3) as u32], false); }
    let mut l_m_4: u32 = 0;
    let r_ = 'f: {
    l_m_4 = F::BattleSide_GetMember(c, p_side_1, p_idx_2);
    if (((((p_items_3) != (((0i32) as u32))) as i32)) != 0) {
        { let s_ = p_items_3; let d_ = (l_m_4).wrapping_add(20); rt::memcpy(c, d_, s_, 16); };
        F::BtlMember_ApplyItems(c, l_m_4);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BattleSide_SetMemberItems", &PROBE_N, &[], true); }
    r_
}

pub fn BattleSide_IsCharaUsable(c: &mut Ctx, mut p_side_1: i32, mut p_chara_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BattleSide_IsCharaUsable", &PROBE_N, &[(p_side_1) as u32, (p_chara_2) as u32], false); }
    let mut l_bits_3: u32 = 0;
    let r_ = 'f: {
    l_bits_3 = (F::battle_load__Side(c, p_side_1)).wrapping_add(528);
    break 'f (((((c.m.r64((l_bits_3).wrapping_add((((rt::div_i32(p_chara_2, 64i32)) as u32)).wrapping_mul(8)))).wrapping_shr((rt::rem_i32(p_chara_2, 64i32)) as u32)) & (((1i32) as u64)))) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BattleSide_IsCharaUsable", &PROBE_N, &[], true); }
    r_
}
