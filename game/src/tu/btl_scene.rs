#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn BtlScene_Init(c: &mut Ctx, mut p_layerMask_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScene_Init", &PROBE_N, &[(p_layerMask_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = F::Heap_Alloc(c, ((52u32) as i32), ((32i32) as u32), 0i32, 2i32); let a_ = sym::gBtlScene; c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32(sym::gBtlScene); F::memset(c, a0_, 0i32, 52u32) }; };
    { let v_ = p_layerMask_1; let a_ = (c.m.r32(sym::gBtlScene)).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
    if (((((p_layerMask_1) == (0i32)) as i32)) != 0) {
        { let v_ = 31i32; let a_ = (c.m.r32(sym::gBtlScene)).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
    }
    F::BtlScene_SetCharCount(c, 2i32);
    if ((F::Battle_IsSplitScreen(c)) != 0) {
        F::BtlScene_SetSingleView(c, 0i32);
    } else {
        F::BtlScene_SetSingleView(c, 1i32);
    }
    F::BtlPool_Init(c, 0i32);
    F::EftVram_Init(c);
    F::EftHit_Init(c);
    F::EftCam_Init(c);
    F::BtlScene_InitRates(c);
    F::EftSprAnim_InitPool(c);
    { let v_ = F::BtlTaskList_Create(c, ((0i32) as u32), ((((1i32) as i64 as u64)) as u32), ((((0i32) as i64 as u64)) as u32)); let a_ = c.m.r32(sym::gBtlScene); c.m.w32(a_, v_); };
    { let _ = { let a0_ = { let a_ = c.m.r32(sym::gBtlScene); c.m.r32(a_) }; F::BtlTaskList_AddFirst(c, a0_, sym::gBtlSceneRootClass, ((0i32) as u32)) }; };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScene_Init", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScene_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScene_Term", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::BtlScene_Reset(c, 0i32);
    { let a0_ = { let a_ = c.m.r32(sym::gBtlScene); c.m.r32(a_) }; F::BtlTaskList_Destroy(c, a0_) };
    F::EftHit_Term(c);
    F::EftCam_Term(c);
    F::EftVram_Term(c);
    F::BtlScene_TermRates(c);
    F::EftSprAnim_TermPool(c);
    F::BtlPool_Term(c);
    F::BtlPool_SetInitArg(c, 0i32);
    if (((((c.m.r32(sym::gBtlScene)) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32(sym::gBtlScene); F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = sym::gBtlScene; c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScene_Term", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScene_Update(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScene_Update", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::EftHit_BeginFrame(c);
    F::EftCam_Update(c);
    F::BtlScene_UpdateCharScales(c);
    { let a0_ = { let a_ = c.m.r32(sym::gBtlScene); c.m.r32(a_) }; F::BtlTaskList_Update(c, a0_) };
    F::EftHit_ClampToStage(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScene_Update", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScene_PostUpdate(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScene_PostUpdate", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::EftHit_UpdateResults(c);
    { let a0_ = { let a_ = c.m.r32(sym::gBtlScene); c.m.r32(a_) }; F::BtlTaskList_PostUpdate(c, a0_) };
    F::BtlScene_UpdateRecords(c);
    F::EftHit_SpawnImpacts(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScene_PostUpdate", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScene_Reset(c: &mut Ctx, mut p_mode_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScene_Reset", &PROBE_N, &[(p_mode_1) as u32], false); }
    let r_ = 'f: {
    F::EftCam_Clear(c);
    F::BtlScene_ClearStageChangeRequest(c);
    { let v_ = ((0i32) as u32); let a_ = (c.m.r32(sym::gBtlScene)).wrapping_add(48); c.m.w32(a_, v_); };
    's1: {
        let sv2: i32 = p_mode_1;
        match sv2 {
            0i32 => {
                { let a0_ = { let a_ = c.m.r32(sym::gBtlScene); c.m.r32(a_) }; F::BtlTaskList_Reset(c, a0_, 0i32) };
                F::EftHit_Clear(c);
                break 's1;
            }
            1i32 => {
                { let a0_ = { let a_ = c.m.r32(sym::gBtlScene); c.m.r32(a_) }; F::BtlTaskList_Reset(c, a0_, 0i32) };
                F::BtlScene_FreeLayers(c);
                { let a0_ = { let a_ = (c.m.r32(sym::gBtlScene)).wrapping_add(4); c.m.r32(a_) }; F::BtlScene_CreateLayers(c, a0_) };
                break 's1;
            }
            2i32 => {
                { let a0_ = { let a_ = c.m.r32(sym::gBtlScene); c.m.r32(a_) }; F::BtlTaskList_Reset(c, a0_, 2048i32) };
                break 's1;
            }
            3i32 => {
                { let a0_ = { let a_ = c.m.r32(sym::gBtlScene); c.m.r32(a_) }; F::BtlTaskList_Reset(c, a0_, 4096i32) };
                break 's1;
            }
            4i32 => {
                { let a0_ = { let a_ = c.m.r32(sym::gBtlScene); c.m.r32(a_) }; F::BtlTaskList_Reset(c, a0_, 8192i32) };
                break 's1;
            }
            _ => {
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScene_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScene_Draw(c: &mut Ctx, mut p_first_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScene_Draw", &PROBE_N, &[(p_first_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = (((p_first_1) != (1i32)) as i32); let a_ = (c.m.r32(sym::gBtlScene)).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
    if ((p_first_1) != 0) {
        F::EftSurf_BuildPalettes(c);
        if ((F::BtlStage_IsReady(c)) != 0) {
            F::EftVram_UploadAll(c);
        }
        F::EftVram_Reset(c);
    }
    F::EftGfx_UpdateClipPlanes(c);
    { let a0_ = { let a_ = c.m.r32(sym::gBtlScene); c.m.r32(a_) }; F::BtlTaskList_Draw(c, a0_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScene_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScene_GetCommonEntry(c: &mut Ctx, mut p_idx_1: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScene_GetCommonEntry", &PROBE_N, &[(p_idx_1) as u32], false); }
    let mut l_base_2: u32 = 0;
    let r_ = 'f: {
    l_base_2 = { let a_ = ((c.m.r32(sym::gCommonRes)).wrapping_add(4)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) };
    break 'f (l_base_2).wrapping_add((((((c.m.r32((l_base_2).wrapping_add((((p_idx_1) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScene_GetCommonEntry", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScene_GetPackEntry(c: &mut Ctx, mut p_base_1: u32, mut p_idx_2: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScene_GetPackEntry", &PROBE_N, &[(p_base_1) as u32, (p_idx_2) as u32], false); }
    let r_ = 'f: {
    break 'f (p_base_1).wrapping_add((((((c.m.r32((p_base_1).wrapping_add((((p_idx_2) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScene_GetPackEntry", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScene_GetPackEntrySize(c: &mut Ctx, mut p_base_1: u32, mut p_idx_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScene_GetPackEntrySize", &PROBE_N, &[(p_base_1) as u32, (p_idx_2) as u32], false); }
    let r_ = 'f: {
    break 'f ((c.m.r32((p_base_1).wrapping_add(((((p_idx_2).wrapping_add(1i32)) as u32)).wrapping_mul(4))) as i32)).wrapping_sub((c.m.r32((p_base_1).wrapping_add((((p_idx_2) as u32)).wrapping_mul(4))) as i32));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScene_GetPackEntrySize", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScene_TestCharPackBit(c: &mut Ctx, mut p_side_1: i32, mut p_bit_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScene_TestCharPackBit", &PROBE_N, &[(p_side_1) as u32, (p_bit_2) as u32], false); }
    let mut l_pack_3: u32 = 0;
    let mut l_mask_4: u32 = 0;
    let mut l_flags_5: u32 = 0;
    let r_ = 'f: {
    l_pack_3 = F::BtlScene_GetCharPackEntry(c, p_side_1, 1i32);
    l_mask_4 = (((1i32).wrapping_shl(((p_bit_2).wrapping_sub(1i32)) as u32)) as u32);
    l_flags_5 = F::BtlScene_GetPackEntry(c, l_pack_3, 1i32);
    break 'f (((((c.m.r32(l_flags_5)) & (l_mask_4))) != (((0i32) as u32))) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScene_TestCharPackBit", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScene_GetCharPackEntry(c: &mut Ctx, mut p_side_1: i32, mut p_idx_2: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScene_GetCharPackEntry", &PROBE_N, &[(p_side_1) as u32, (p_idx_2) as u32], false); }
    let mut l_base_3: u32 = 0;
    let r_ = 'f: {
    l_base_3 = ((((F::BtlCharApi_GetPlayerObjUnk58(c, p_side_1)) as i64 as u64)) as u32);
    if (((((l_base_3) != (((0i32) as u32))) as i32)) != 0) {
        break 'f (l_base_3).wrapping_add((((((c.m.r32((l_base_3).wrapping_add((((p_idx_2) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32));
    }
    break 'f l_base_3;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScene_GetCharPackEntry", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScene_GetStageData(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScene_GetStageData", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f { let a_ = (c.m.r32(sym::gCommonRes)).wrapping_add(((36i32) as u32)); (c.m.r32(a_) as i32) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScene_GetStageData", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScene_IsTimeStopped(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScene_IsTimeStopped", &PROBE_N, &[], false); }
    let mut l_result_1: i32 = 0;
    let mut l_paused_2: i32 = 0;
    let r_ = 'f: {
    l_result_1 = 1i32;
    l_paused_2 = 0i32;
    if (((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0) {
        l_paused_2 = 1i32;
    }
    if ((F::BtlStage_IsReady(c)) != 0) {
        if ((((!((l_paused_2) != 0)) as i32)) != 0) {
            l_result_1 = 0i32;
        }
    }
    break 'f l_result_1;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScene_IsTimeStopped", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScene_IsCharStopped(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScene_IsCharStopped", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_paused_2: i32 = 0;
    let mut l_result_3: i32 = 0;
    let r_ = 'f: {
    l_paused_2 = 1i32;
    l_result_3 = (((F::BtlCharApi_IsFrozen(c, p_objId_1)) != (0i32)) as i32);
    if (((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0) {
        l_result_3 = l_paused_2;
    }
    break 'f l_result_3;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScene_IsCharStopped", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScene_IsAnyCharStopped(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScene_IsAnyCharStopped", &PROBE_N, &[], false); }
    let mut l_result_1: i32 = 0;
    let r_ = 'f: {
    l_result_1 = (((F::BtlCharApi_AnyFrozen(c)) != (0i32)) as i32);
    if (((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0) {
        l_result_1 = 1i32;
    }
    if ((F::BtlCharApi_IsInRushSequence(c, 0i32)) != 0) {
        l_result_1 = 1i32;
    } else {
        if ((F::BtlCharApi_IsInRushSequence(c, 1i32)) != 0) {
            l_result_1 = 1i32;
        }
    }
    break 'f l_result_1;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScene_IsAnyCharStopped", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScene_SetCharCount(c: &mut Ctx, mut p_count_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScene_SetCharCount", &PROBE_N, &[(p_count_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_count_1; let a_ = sym::gBtlSceneCharCount; c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScene_SetCharCount", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScene_GetCharCount(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScene_GetCharCount", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f (c.m.r32(sym::gBtlSceneCharCount) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScene_GetCharCount", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScene_IsSecondView(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScene_IsSecondView", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f { let a_ = (c.m.r32(sym::gBtlScene)).wrapping_add(32); (c.m.r32(a_) as i32) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScene_IsSecondView", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScene_IsCharInView(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScene_IsCharInView", &PROBE_N, &[(p_objId_1) as u32], false); }
    let r_ = 'f: {
    if ((F::Battle_IsSplitScreen(c)) != 0) {
        if ((F::BtlScene_IsSingleView(c)) != 0) {
            if ((F::EftCam_IsActive(c)) != 0) {
                break 'f 1i32;
            }
            break 'f F::BtlCharApi_IsCamShown(c, p_objId_1);
        }
        break 'f (((F::BtlScene_IsSecondView(c)) == (p_objId_1)) as i32);
    }
    break 'f F::BtlCharApi_IsCamShown(c, p_objId_1);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScene_IsCharInView", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScene_IsStageFlagOn(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScene_IsStageFlagOn", &PROBE_N, &[], false); }
    let mut l_p_1: u32 = 0;
    let r_ = 'f: {
    's1: {
        let sv2: i32 = F::Battle_GetStage(c);
        match sv2 {
            3i32 | 15i32 | 16i32 | 24i32 | 26i32 | 30i32 | 32i32 => {
                break 'f 1i32;
            }
            _ => {
                l_p_1 = F::StgTint_GetWork(c);
                if (((((l_p_1) != (((0i32) as u32))) as i32)) != 0) {
                    break 'f (((((c.m.r8((l_p_1).wrapping_add(((88i32) as u32)))) as i32)) != (0i32)) as i32);
                }
                break 'f 0i32;
            }
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScene_IsStageFlagOn", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScene_GetStageFlag(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScene_GetStageFlag", &PROBE_N, &[], false); }
    let mut l_p_1: u32 = 0;
    let r_ = 'f: {
    l_p_1 = F::StgTint_GetWork(c);
    if (((((l_p_1) != (((0i32) as u32))) as i32)) != 0) {
        break 'f (((((c.m.r8((l_p_1).wrapping_add(((88i32) as u32)))) as i32)) != (0i32)) as i32);
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScene_GetStageFlag", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScene_SetSingleView(c: &mut Ctx, mut p_singleView_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScene_SetSingleView", &PROBE_N, &[(p_singleView_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_singleView_1; let a_ = (c.m.r32(sym::gBtlScene)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScene_SetSingleView", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScene_IsSingleView(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScene_IsSingleView", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if ((((!((F::Battle_IsSplitScreen(c)) != 0)) as i32)) != 0) {
        break 'f 1i32;
    }
    break 'f { let a_ = (c.m.r32(sym::gBtlScene)).wrapping_add(36); (c.m.r32(a_) as i32) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScene_IsSingleView", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScene_FreeLayer0(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScene_FreeLayer0", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::BtlPool_SetCurrent(c, 2i32);
    { let a0_ = { let a_ = ((c.m.r32(sym::gBtlScene)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }; F::BtlTask_Kill(c, a0_) };
    { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gBtlScene)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    F::BtlPool_Reset(c, 2i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScene_FreeLayer0", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScene_CreateLayer0(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScene_CreateLayer0", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if ((((({ let a_ = ((c.m.r32(sym::gBtlScene)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) == (((0i32) as u32))) as i32)) != 0) {
        { let v_ = { let a0_ = { let a_ = (c.m.r32(sym::gBtlScene)).wrapping_add(4); c.m.r32(a_) }; F::BtlTaskList_AddHead(c, a0_, sym::D_002C3550, ((0i32) as u32)) }; let a_ = ((c.m.r32(sym::gBtlScene)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScene_CreateLayer0", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScene_GetGroup(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScene_GetGroup", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f { let a_ = (c.m.r32(sym::gBtlScene)).wrapping_add(4); c.m.r32(a_) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScene_GetGroup", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScene_FreeCharLayer2(c: &mut Ctx, mut p_chr_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScene_FreeCharLayer2", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    F::EftChar_Kill(c, p_chr_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScene_FreeCharLayer2", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScene_CreateCharLayer2(c: &mut Ctx, mut p_chr_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScene_CreateCharLayer2", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    F::EftChar_Create(c, p_chr_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScene_CreateCharLayer2", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScene_FreeChar(c: &mut Ctx, mut p_chr_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScene_FreeChar", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    F::BtlScene_FreeCharLayer2(c, p_chr_1);
    F::EftShot_DestroyChar(c, p_chr_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScene_FreeChar", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScene_CreateChar(c: &mut Ctx, mut p_chr_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScene_CreateChar", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    F::EftShot_CreateChar(c, p_chr_1);
    F::BtlScene_CreateCharLayer2(c, p_chr_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScene_CreateChar", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScene_IsEffectStopped(c: &mut Ctx, mut p_objId_1: i32, mut p_kind_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScene_IsEffectStopped", &PROBE_N, &[(p_objId_1) as u32, (p_kind_2) as u32], false); }
    let mut l_result_3: i32 = 0;
    let r_ = 'f: {
    l_result_3 = 0i32;
    's1: {
        let sv4: i32 = p_kind_2;
        match sv4 {
            0i32 => {
                if ((F::BtlScene_IsAnyCharStopped(c)) != 0) {
                    l_result_3 = 1i32;
                }
                break 's1;
            }
            1i32 => {
                if ((F::BtlScene_IsCharStopped(c, p_objId_1)) != 0) {
                    l_result_3 = 1i32;
                }
                break 's1;
            }
            3i32 => {
                if ((F::BtlScene_IsTimeStopped(c)) != 0) {
                    l_result_3 = 1i32;
                }
                break 's1;
            }
            6i32 => {
                if ((F::BtlScene_IsTimeStopped(c)) != 0) {
                    l_result_3 = 1i32;
                }
                break 's1;
            }
            4i32 => {
                if ((F::BtlScene_IsTimeStopped(c)) != 0) {
                    l_result_3 = 1i32;
                } else {
                    if ((F::BtlCharApi_IsInRushSequence(c, p_objId_1)) != 0) {
                        l_result_3 = 1i32;
                    }
                }
                break 's1;
            }
            _ => {
                l_result_3 = (((F::BtlScene_IsTimeStopped(c)) != (0i32)) as i32);
                break 's1;
            }
        }
    }
    break 'f l_result_3;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScene_IsEffectStopped", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScene_IsEffectHidden(c: &mut Ctx, mut p_objId_1: i32, mut p_kind_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScene_IsEffectHidden", &PROBE_N, &[(p_objId_1) as u32, (p_kind_2) as u32], false); }
    let mut l_result_3: i32 = 0;
    let r_ = 'f: {
    l_result_3 = 0i32;
    if ((F::BtlCharApi_IsHidden(c, p_objId_1)) != 0) {
        's1: {
            let sv4: i32 = p_kind_2;
            match sv4 {
                0i32 => {
                    if ((F::BtlCharApi_AnyFrozen(c)) != 0) {
                        l_result_3 = 1i32;
                    }
                    break 's1;
                }
                1i32 => {
                    if ((((!((F::BtlCharApi_IsInRushSequence(c, p_objId_1)) != 0)) as i32)) != 0) {
                        l_result_3 = 1i32;
                    }
                    break 's1;
                }
                3i32 | 6i32 => {
                    l_result_3 = 1i32;
                    break 's1;
                }
                _ => {
                    if ((F::BtlCharApi_IsFrozen(c, p_objId_1)) != 0) {
                        l_result_3 = 1i32;
                    }
                    if ((((!((F::BtlStage_IsReady(c)) != 0)) as i32)) != 0) {
                        l_result_3 = 1i32;
                    }
                    break 's1;
                }
            }
        }
    }
    if (((((p_kind_2) == (4i32)) as i32)) != 0) {
        if ((F::BtlCharApi_IsHidden(c, p_objId_1)) != 0) {
            l_result_3 = 1i32;
        } else {
            if ((F::BtlCharApi_IsInRushSequence(c, p_objId_1)) != 0) {
                l_result_3 = 1i32;
            }
        }
    }
    if ((F::BtlCharApi_IsChanging(c, p_objId_1)) != 0) {
        's2: {
            let sv5: i32 = p_kind_2;
            match sv5 {
                5i32 | 6i32 => {
                    break 's2;
                }
                _ => {
                    l_result_3 = 1i32;
                    break 's2;
                }
            }
        }
    }
    if (((((p_kind_2) == (0i32)) as i32)) != 0) {
        if ((F::BtlCharApi_AnyCamPriority(c)) != 0) {
            l_result_3 = 1i32;
        } else {
            if ((F::BtlCharApi_IsInRushSequence(c, 0i32)) != 0) {
                l_result_3 = 1i32;
            } else {
                if ((F::BtlCharApi_IsInRushSequence(c, 1i32)) != 0) {
                    l_result_3 = 1i32;
                }
            }
        }
    }
    break 'f l_result_3;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScene_IsEffectHidden", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScene_RequestStageChange(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScene_RequestStageChange", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let r_ = 1i32; let a_ = (c.m.r32(sym::gBtlScene)).wrapping_add(40); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScene_RequestStageChange", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScene_IsStageChangeRequested(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScene_IsStageChangeRequested", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if (((({ let a_ = (c.m.r32(sym::gBtlScene)).wrapping_add(40); (c.m.r32(a_) as i32) }) & (1i32))) != 0) {
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScene_IsStageChangeRequested", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScene_ClearStageChangeRequest(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScene_ClearStageChangeRequest", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let r_ = (!(1i32)); let a_ = (c.m.r32(sym::gBtlScene)).wrapping_add(40); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScene_ClearStageChangeRequest", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScene_CheckStageChange(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScene_CheckStageChange", &PROBE_N, &[], false); }
    let mut l_hit_1: i32 = 0;
    let mut l_target_2: i32 = 0;
    let mut l_found_3: i32 = 0;
    let mut l_list_4: u32 = 0;
    let mut l_rec_5: u32 = 0;
    let mut l_i_6: i32 = 0;
    let mut l_attr_7: i32 = 0;
    let mut l_mode_8: i32 = 0;
    let r_ = 'f: {
    l_hit_1 = 0i32;
    l_target_2 = (1i32).wrapping_neg();
    l_found_3 = 0i32;
    l_list_4 = F::EftHit_GetList(c);
    if (((((l_list_4) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    l_mode_8 = F::Battle_GetMode(c);
    if (((((l_mode_8) < (8i32)) as i32)) != 0) {
        if (((((l_mode_8) >= (4i32)) as i32)) != 0) {
            break 'f;
        }
    }
    if ((((((((!((F::Battle_IsTimeLimitOff(c)) != 0)) as i32)) != 0) && (((((F::Battle_GetTimeLimit(c)) < (10i32)) as i32)) != 0)) as i32)) != 0) {
        break 'f;
    }
    if ((((!((F::BtlGame_IsFighting(c)) != 0)) as i32)) != 0) {
        break 'f;
    }
    {
        l_i_6 = 0i32;
        'l1: loop {
            if !(((((l_i_6) < ((c.m.r32((l_list_4).wrapping_add(25600)) as i32))) as i32)) != 0) { break 'l1; }
            'c2: {
                l_rec_5 = (l_list_4).wrapping_add((((l_i_6) as u32)).wrapping_mul(400));
                l_hit_1 = 0i32;
                l_target_2 = { let a0_ = (c.m.r32(l_rec_5) as i32); F::BtlCharApi_GetOpponentObjId(c, a0_) };
                l_attr_7 = F::EftHit_GetTaskFlags(c, ((((l_i_6) as i64 as u64)) as u32));
                if ((((((c.m.r32((l_rec_5).wrapping_add(12)) as i32)) != (0i32)) as i32)) != 0) {
                    if ((((((((((F::Battle_GetRuleUnk10(c)) != 0) && ((F::EftHit_HasDefFlag200000(c, l_rec_5)) != 0)) as i32)) != 0) && (({ let a0_ = (c.m.r32(l_rec_5) as i32); F::BtlCharApi_IsTargetBelowHalfHp(c, a0_, l_target_2) }) != 0)) as i32)) != 0) {
                        l_hit_1 = 1i32;
                    } else {
                        if (({ let a0_ = (c.m.r32(l_rec_5) as i32); F::BtlCharApi_CanTechniqueFinish(c, a0_, l_target_2) }) != 0) {
                            l_hit_1 = 2i32;
                        } else {
                            l_hit_1 = 0i32;
                        }
                    }
                    if (((((l_hit_1) != (0i32)) as i32)) != 0) {
                        if (((((((l_attr_7) & (3i32))) == (1i32)) as i32)) != 0) {
                            if ((((l_attr_7) & (16384i32))) != 0) {
                                if (((((c.m.r32((l_rec_5).wrapping_add(80)) as i32)) & (16i32))) != 0) {
                                    l_found_3 = 1i32;
                                    if (((((l_hit_1) == (1i32)) as i32)) != 0) {
                                        { let r_ = 2i32; let a_ = (l_rec_5).wrapping_add(80); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                                        { let r_ = 8i32; let a_ = (l_rec_5).wrapping_add(80); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                                    }
                                    break 'l1;
                                }
                            } else {
                                l_found_3 = 1i32;
                                if (((((l_hit_1) == (1i32)) as i32)) != 0) {
                                    { let r_ = 2i32; let a_ = (l_rec_5).wrapping_add(80); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                                    { let r_ = 8i32; let a_ = (l_rec_5).wrapping_add(80); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                                }
                                break 'l1;
                            }
                        }
                    }
                }
            }
            { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; };
        }
    }
    if ((F::BtlScene_IsStageChangeRequested(c)) != 0) {
        F::BtlScene_ClearStageChangeRequest(c);
        l_hit_1 = 1i32;
        l_found_3 = 1i32;
    }
    if ((l_found_3) != 0) {
        if (((((l_hit_1) == (1i32)) as i32)) != 0) {
            F::BtlStage_RequestChange(c);
        } else {
            if (((((l_hit_1) == (2i32)) as i32)) != 0) {
                F::BtlCharApi_SetHeldFlagAB(c, l_target_2);
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScene_CheckStageChange", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScene_InitRates(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScene_InitRates", &PROBE_N, &[], false); }
    let mut l_size_1: i32 = 0;
    let mut l_i_2: i32 = 0;
    let mut l_j_3: i32 = 0;
    let r_ = 'f: {
    l_size_1 = ((((((c.m.r32(sym::gBtlSceneCharCount) as i32)) as u32)).wrapping_mul(28u32)) as i32);
    { let v_ = { let a0_ = F::BtlPool_GetCurrent(c); F::BtlPool_Alloc(c, a0_, l_size_1) }; let a_ = (c.m.r32(sym::gBtlScene)).wrapping_add(44); c.m.w32(a_, v_); };
    { let _ = { let a0_ = { let a_ = (c.m.r32(sym::gBtlScene)).wrapping_add(44); c.m.r32(a_) }; F::memset(c, a0_, 0i32, ((l_size_1) as u32)) }; };
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < ((c.m.r32(sym::gBtlSceneCharCount) as i32))) as i32)) != 0) { break 'l1; }
            'c2: {
                {
                    l_j_3 = 0i32;
                    'l3: loop {
                        if !(((((l_j_3) < (6i32)) as i32)) != 0) { break 'l3; }
                        'c4: {
                            F::BtlScene_SetCharRate(c, l_i_2, l_j_3, F32(0x3f800000));
                        }
                        { let o_ = l_j_3; let n_ = (o_).wrapping_add(1); l_j_3 = n_; };
                    }
                }
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScene_InitRates", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScene_TermRates(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScene_TermRates", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if ((((({ let a_ = (c.m.r32(sym::gBtlScene)).wrapping_add(44); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = F::BtlPool_GetCurrent(c); let a1_ = { let a_ = (c.m.r32(sym::gBtlScene)).wrapping_add(44); c.m.r32(a_) }; F::BtlPool_Free(c, a0_, a1_) };
        { let v_ = ((0i32) as u32); let a_ = (c.m.r32(sym::gBtlScene)).wrapping_add(44); c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScene_TermRates", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScene_SetCharRate(c: &mut Ctx, mut p_idx_1: i32, mut p_slot_2: i32, mut p_value_3: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScene_SetCharRate", &PROBE_N, &[(p_idx_1) as u32, (p_slot_2) as u32, (p_value_3).0], false); }
    let mut l_rate_4: u32 = 0;
    let r_ = 'f: {
    if (((((((((p_idx_1) >= (0i32)) as i32)) != 0) && (((((p_idx_1) <= (((c.m.r32(sym::gBtlSceneCharCount) as i32)).wrapping_sub(1i32))) as i32)) != 0)) as i32)) != 0) {
        l_rate_4 = ({ let a_ = (c.m.r32(sym::gBtlScene)).wrapping_add(44); c.m.r32(a_) }).wrapping_add((((p_idx_1) as u32)).wrapping_mul(28));
        { let v_ = p_value_3; let a_ = (l_rate_4).wrapping_add((((p_slot_2) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScene_SetCharRate", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScene_GetCharRate(c: &mut Ctx, mut p_idx_1: i32, mut p_slot_2: i32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScene_GetCharRate", &PROBE_N, &[(p_idx_1) as u32, (p_slot_2) as u32], false); }
    let mut l_rate_3: u32 = 0;
    let r_ = 'f: {
    if (((((((((p_idx_1) < (0i32)) as i32)) != 0) || (((((p_idx_1) > (((c.m.r32(sym::gBtlSceneCharCount) as i32)).wrapping_sub(1i32))) as i32)) != 0)) as i32)) != 0) {
        break 'f F32(0x3f800000);
    }
    l_rate_3 = ({ let a_ = (c.m.r32(sym::gBtlScene)).wrapping_add(44); c.m.r32(a_) }).wrapping_add((((p_idx_1) as u32)).wrapping_mul(28));
    break 'f F32(c.m.r32((l_rate_3).wrapping_add((((p_slot_2) as u32)).wrapping_mul(4))));
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScene_GetCharRate", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScene_UpdateAllCharScales(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScene_UpdateAllCharScales", &PROBE_N, &[], false); }
    let mut l_rate_1: u32 = 0;
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < ((c.m.r32(sym::gBtlSceneCharCount) as i32))) as i32)) != 0) { break 'l1; }
            'c2: {
                l_rate_1 = ({ let a_ = (c.m.r32(sym::gBtlScene)).wrapping_add(44); c.m.r32(a_) }).wrapping_add((((l_i_2) as u32)).wrapping_mul(28));
                { let v_ = rt::fdiv(F::BtlCharApi_GetHeight(c, l_i_2), F32(0x419acccc)); let a_ = (l_rate_1).wrapping_add(24); c.m.w32(a_, (v_).0); };
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScene_UpdateAllCharScales", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScene_UpdateCharScales(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScene_UpdateCharScales", &PROBE_N, &[], false); }
    let mut l_rate_1: u32 = 0;
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
    if ((((({ let a_ = (c.m.r32(sym::gBtlScene)).wrapping_add(28); (c.m.r32(a_) as i32) }) != (8i32)) as i32)) != 0) {
        {
            l_i_2 = 0i32;
            'l1: loop {
                if !(((((l_i_2) < ((c.m.r32(sym::gBtlSceneCharCount) as i32))) as i32)) != 0) { break 'l1; }
                'c2: {
                    if ((((((F::BtlCharApi_IsFighter(c, l_i_2)) != 0) && ((((!((F::BtlCharApi_IsChanging(c, l_i_2)) != 0)) as i32)) != 0)) as i32)) != 0) {
                        l_rate_1 = ({ let a_ = (c.m.r32(sym::gBtlScene)).wrapping_add(44); c.m.r32(a_) }).wrapping_add((((l_i_2) as u32)).wrapping_mul(28));
                        { let v_ = rt::fdiv(F::BtlCharApi_GetHeight(c, l_i_2), F32(0x419acccc)); let a_ = (l_rate_1).wrapping_add(24); c.m.w32(a_, (v_).0); };
                    }
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScene_UpdateCharScales", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScene_GetCharScale(c: &mut Ctx, mut p_idx_1: i32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScene_GetCharScale", &PROBE_N, &[(p_idx_1) as u32], false); }
    let mut l_rate_2: u32 = 0;
    let r_ = 'f: {
    if (((((((((p_idx_1) < (0i32)) as i32)) != 0) || (((((p_idx_1) > (((c.m.r32(sym::gBtlSceneCharCount) as i32)).wrapping_sub(1i32))) as i32)) != 0)) as i32)) != 0) {
        break 'f F32(0x3f800000);
    }
    l_rate_2 = ({ let a_ = (c.m.r32(sym::gBtlScene)).wrapping_add(44); c.m.r32(a_) }).wrapping_add((((p_idx_1) as u32)).wrapping_mul(28));
    break 'f F32(c.m.r32((l_rate_2).wrapping_add(24)));
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScene_GetCharScale", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSceneRoot_Init(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSceneRoot_Init", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = F::BtlTask_CreateChildList(c, p_task_1, ((((5i32) as i64 as u64)) as u32), ((((0i32) as i64 as u64)) as u32)); let a_ = (c.m.r32(sym::gBtlScene)).wrapping_add(4); c.m.w32(a_, v_); };
    { let a0_ = { let a_ = (c.m.r32(sym::gBtlScene)).wrapping_add(4); c.m.r32(a_) }; F::BtlScene_CreateLayers(c, a0_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSceneRoot_Init", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSceneRoot_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSceneRoot_Term", &PROBE_N, &[], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSceneRoot_Term", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSceneRoot_Update(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSceneRoot_Update", &PROBE_N, &[], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSceneRoot_Update", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScene_CreateLayers(c: &mut Ctx, mut p_group_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScene_CreateLayers", &PROBE_N, &[(p_group_1) as u32], false); }
    let r_ = 'f: {
    if (((({ let a_ = (c.m.r32(sym::gBtlScene)).wrapping_add(28); (c.m.r32(a_) as i32) }) & (8i32))) != 0) {
        { let v_ = { let a0_ = { let a_ = (c.m.r32(sym::gBtlScene)).wrapping_add(4); c.m.r32(a_) }; F::BtlTaskList_AddTail(c, a0_, sym::D_002C3550, ((0i32) as u32)) }; let a_ = ((c.m.r32(sym::gBtlScene)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    }
    if (((({ let a_ = (c.m.r32(sym::gBtlScene)).wrapping_add(28); (c.m.r32(a_) as i32) }) & (4i32))) != 0) {
        { let v_ = { let a0_ = { let a_ = (c.m.r32(sym::gBtlScene)).wrapping_add(4); c.m.r32(a_) }; F::BtlTaskList_AddTail(c, a0_, sym::D_002C36D0, ((0i32) as u32)) }; let a_ = ((c.m.r32(sym::gBtlScene)).wrapping_add(8)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    }
    if (((({ let a_ = (c.m.r32(sym::gBtlScene)).wrapping_add(28); (c.m.r32(a_) as i32) }) & (2i32))) != 0) {
        { let v_ = { let a0_ = { let a_ = (c.m.r32(sym::gBtlScene)).wrapping_add(4); c.m.r32(a_) }; F::BtlTaskList_AddTail(c, a0_, sym::gEftCharRootClass, ((0i32) as u32)) }; let a_ = ((c.m.r32(sym::gBtlScene)).wrapping_add(8)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    }
    if (((({ let a_ = (c.m.r32(sym::gBtlScene)).wrapping_add(28); (c.m.r32(a_) as i32) }) & (1i32))) != 0) {
        { let v_ = { let a0_ = { let a_ = (c.m.r32(sym::gBtlScene)).wrapping_add(4); c.m.r32(a_) }; F::BtlTaskList_AddTail(c, a0_, sym::gEftLayer3Class, ((0i32) as u32)) }; let a_ = ((c.m.r32(sym::gBtlScene)).wrapping_add(8)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    }
    if (((({ let a_ = (c.m.r32(sym::gBtlScene)).wrapping_add(28); (c.m.r32(a_) as i32) }) & (16i32))) != 0) {
        { let v_ = { let a0_ = { let a_ = (c.m.r32(sym::gBtlScene)).wrapping_add(4); c.m.r32(a_) }; F::BtlTaskList_AddTail(c, a0_, sym::D_002C35D0, ((0i32) as u32)) }; let a_ = ((c.m.r32(sym::gBtlScene)).wrapping_add(8)).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScene_CreateLayers", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScene_FreeLayers(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScene_FreeLayers", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if (((({ let a_ = (c.m.r32(sym::gBtlScene)).wrapping_add(28); (c.m.r32(a_) as i32) }) & (1i32))) != 0) {
        { let a0_ = { let a_ = ((c.m.r32(sym::gBtlScene)).wrapping_add(8)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }; F::BtlTask_Kill(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gBtlScene)).wrapping_add(8)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    }
    if (((({ let a_ = (c.m.r32(sym::gBtlScene)).wrapping_add(28); (c.m.r32(a_) as i32) }) & (2i32))) != 0) {
        { let a0_ = { let a_ = ((c.m.r32(sym::gBtlScene)).wrapping_add(8)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }; F::BtlTask_Kill(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gBtlScene)).wrapping_add(8)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    }
    if (((({ let a_ = (c.m.r32(sym::gBtlScene)).wrapping_add(28); (c.m.r32(a_) as i32) }) & (4i32))) != 0) {
        { let a0_ = { let a_ = ((c.m.r32(sym::gBtlScene)).wrapping_add(8)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }; F::BtlTask_Kill(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gBtlScene)).wrapping_add(8)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    }
    if (((({ let a_ = (c.m.r32(sym::gBtlScene)).wrapping_add(28); (c.m.r32(a_) as i32) }) & (8i32))) != 0) {
        { let a0_ = { let a_ = ((c.m.r32(sym::gBtlScene)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }; F::BtlTask_Kill(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gBtlScene)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    }
    if (((({ let a_ = (c.m.r32(sym::gBtlScene)).wrapping_add(28); (c.m.r32(a_) as i32) }) & (16i32))) != 0) {
        { let a0_ = { let a_ = ((c.m.r32(sym::gBtlScene)).wrapping_add(8)).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }; F::BtlTask_Kill(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gBtlScene)).wrapping_add(8)).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScene_FreeLayers", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScene_Rand(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScene_Rand", &PROBE_N, &[], false); }
    let mut l_state_1: u64 = 0;
    let mut l_next_2: i32 = 0;
    let r_ = 'f: {
    l_state_1 = (({ let a_ = (c.m.r32(sym::gBtlScene)).wrapping_add(48); c.m.r32(a_) }) as u64);
    l_next_2 = ((rt::rem_u64(((l_state_1).wrapping_mul(((714025i32) as u64))).wrapping_add(((4096i32) as u64)), ((150889i32) as u64))) as i32);
    if ((((!((F::BtlChars_IsTimeStopped(c)) != 0)) as i32)) != 0) {
        { let v_ = ((l_next_2) as u32); let a_ = (c.m.r32(sym::gBtlScene)).wrapping_add(48); c.m.w32(a_, v_); };
    }
    break 'f l_next_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScene_Rand", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScene_RandF(c: &mut Ctx) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScene_RandF", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f rt::fdiv(rt::u2f(((F::BtlScene_Rand(c)) as u32)), F32(0x48135a40));
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScene_RandF", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScene_RandRangeF(c: &mut Ctx, mut p_a_1: F32, mut p_b_2: F32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScene_RandRangeF", &PROBE_N, &[(p_a_1).0, (p_b_2).0], false); }
    let mut l_tmp_3: F32 = F32(0);
    let r_ = 'f: {
    if (((rt::feq(p_b_2, p_a_1) as i32)) != 0) {
        break 'f p_b_2;
    }
    if (((rt::flt(p_b_2, p_a_1) as i32)) != 0) {
        l_tmp_3 = p_a_1;
        p_a_1 = p_b_2;
        p_b_2 = l_tmp_3;
    }
    break 'f rt::fadd(p_a_1, rt::fmul(F::BtlScene_RandF(c), rt::fsub(p_b_2, p_a_1)));
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScene_RandRangeF", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScene_RandRange(c: &mut Ctx, mut p_a_1: i32, mut p_b_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScene_RandRange", &PROBE_N, &[(p_a_1) as u32, (p_b_2) as u32], false); }
    let mut l_tmp_3: i32 = 0;
    let r_ = 'f: {
    if (((((p_b_2) == (p_a_1)) as i32)) != 0) {
        break 'f p_a_1;
    }
    if (((((p_b_2) < (p_a_1)) as i32)) != 0) {
        l_tmp_3 = p_a_1;
        p_a_1 = p_b_2;
        p_b_2 = l_tmp_3;
    }
    break 'f (((((p_a_1) as u32)).wrapping_add(rt::rem_u32(((F::BtlScene_Rand(c)) as u32), ((((p_b_2).wrapping_sub(p_a_1)).wrapping_add(1i32)) as u32)))) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScene_RandRange", &PROBE_N, &[], true); }
    r_
}

pub fn BtlScene_UpdateRecords(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlScene_UpdateRecords", &PROBE_N, &[], false); }
    let mut l_list_1: u32 = 0;
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
    l_list_1 = F::EftHit_GetList(c);
    if (((((l_list_1) != (((0i32) as u32))) as i32)) != 0) {
        {
            l_i_2 = 0i32;
            'l1: loop {
                if !(((((l_i_2) < ((c.m.r32((l_list_1).wrapping_add(25600)) as i32))) as i32)) != 0) { break 'l1; }
                'c2: {
                    F::EftWater_UpdateBlast(c, (l_list_1).wrapping_add((((l_i_2) as u32)).wrapping_mul(400)));
                    F::EftBubble_OnBlastRecord(c, (l_list_1).wrapping_add((((l_i_2) as u32)).wrapping_mul(400)));
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlScene_UpdateRecords", &PROBE_N, &[], true); }
    r_
}
