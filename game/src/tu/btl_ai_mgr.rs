#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn btl_ai_mgr__BtlAiMove_Div(c: &mut Ctx, mut p_a_1: i32, mut p_b_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "btl_ai_mgr__BtlAiMove_Div", &PROBE_N, &[(p_a_1) as u32, (p_b_2) as u32], false); }
    let r_ = 'f: {
    break 'f rt::div_i32(p_a_1, p_b_2);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "btl_ai_mgr__BtlAiMove_Div", &PROBE_N, &[], true); }
    r_
}

pub fn btl_ai_mgr__BtlAiMove_Mod(c: &mut Ctx, mut p_a_1: i32, mut p_b_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "btl_ai_mgr__BtlAiMove_Mod", &PROBE_N, &[(p_a_1) as u32, (p_b_2) as u32], false); }
    let r_ = 'f: {
    break 'f rt::rem_i32(p_a_1, p_b_2);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "btl_ai_mgr__BtlAiMove_Mod", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiMgr_Reset(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiMgr_Reset", &PROBE_N, &[], false); }
    let mut l_s_1: u32 = 0;
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
    if (((({ let a_ = (c.m.r32(sym::gBtlAi)).wrapping_add(2644); (c.m.r32(a_) as i32) }) & (1i32))) != 0) {
        if ((((({ let a_ = c.m.r32(sym::gBtlAi); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
            { let a0_ = { let a_ = c.m.r32(sym::gBtlAi); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
            { let v_ = ((0i32) as u32); let a_ = c.m.r32(sym::gBtlAi); c.m.w32(a_, v_); };
        }
    }
    {
        l_s_1 = (c.m.r32(sym::gBtlAi)).wrapping_add(16);
        'l1: loop {
            if !(((((l_s_1) != (((c.m.r32(sym::gBtlAi)).wrapping_add(16)).wrapping_add((((2i32) as u32)).wrapping_mul(1312)))) as i32)) != 0) { break 'l1; }
            'c2: {
                if (((((c.m.r32((l_s_1).wrapping_add(36)) as i32)) & (1i32))) != 0) {
                    if (((((c.m.r32((l_s_1).wrapping_add(24))) != (((0i32) as u32))) as i32)) != 0) {
                        { let a0_ = c.m.r32((l_s_1).wrapping_add(24)); F::Heap_Free(c, a0_) };
                        { let v_ = ((0i32) as u32); let a_ = (l_s_1).wrapping_add(24); c.m.w32(a_, v_); };
                    }
                }
            }
            { let o_ = l_s_1; let n_ = (o_).wrapping_add(1312); l_s_1 = n_; };
        }
    }
    F::AiThink_BindData(c, 0i32);
    {
        l_i_2 = 0i32;
        'l3: loop {
            if !(((((l_i_2) < (2i32)) as i32)) != 0) { break 'l3; }
            'c4: {
                F::AiThink_ResetSide(c, l_i_2, 0i32);
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiMgr_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiMgr_ResetSide(c: &mut Ctx, mut p_side_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiMgr_ResetSide", &PROBE_N, &[(p_side_1) as u32], false); }
    let r_ = 'f: {
    F::AiThink_ResetSide(c, p_side_1, 0i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiMgr_ResetSide", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiMgr_Init(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiMgr_Init", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if (((((c.m.r32(sym::gBtlAi)) == (((0i32) as u32))) as i32)) != 0) {
        { let _ = { let a0_ = { let v_ = F::Heap_Alloc(c, 2656i32, ((32i32) as u32), 0i32, 2i32); let a_ = sym::gBtlAi; c.m.w32(a_, v_); v_ }; F::memset(c, a0_, 0i32, ((2656i32) as u32)) }; };
    }
    F::BtlAiMgr_Reset(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiMgr_Init", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiMgr_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiMgr_Term", &PROBE_N, &[], false); }
    let mut l_s_1: u32 = 0;
    let r_ = 'f: {
    if (((({ let a_ = (c.m.r32(sym::gBtlAi)).wrapping_add(2644); (c.m.r32(a_) as i32) }) & (1i32))) != 0) {
        if ((((({ let a_ = c.m.r32(sym::gBtlAi); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
            { let a0_ = { let a_ = c.m.r32(sym::gBtlAi); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
            { let v_ = ((0i32) as u32); let a_ = c.m.r32(sym::gBtlAi); c.m.w32(a_, v_); };
        }
    }
    {
        l_s_1 = (c.m.r32(sym::gBtlAi)).wrapping_add(16);
        'l1: loop {
            if !(((((l_s_1) != (((c.m.r32(sym::gBtlAi)).wrapping_add(16)).wrapping_add((((2i32) as u32)).wrapping_mul(1312)))) as i32)) != 0) { break 'l1; }
            'c2: {
                if (((((c.m.r32((l_s_1).wrapping_add(36)) as i32)) & (1i32))) != 0) {
                    if (((((c.m.r32((l_s_1).wrapping_add(24))) != (((0i32) as u32))) as i32)) != 0) {
                        { let a0_ = c.m.r32((l_s_1).wrapping_add(24)); F::Heap_Free(c, a0_) };
                        { let v_ = ((0i32) as u32); let a_ = (l_s_1).wrapping_add(24); c.m.w32(a_, v_); };
                    }
                }
            }
            { let o_ = l_s_1; let n_ = (o_).wrapping_add(1312); l_s_1 = n_; };
        }
    }
    if (((((c.m.r32(sym::gBtlAi)) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32(sym::gBtlAi); F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = sym::gBtlAi; c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiMgr_Term", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiMgr_BuildRates(c: &mut Ctx, mut p_s_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiMgr_BuildRates", &PROBE_N, &[(p_s_1) as u32], false); }
    let mut l_tbl_2: u32 = 0;
    let mut l_i_3: i32 = 0;
    let mut l_r_4: i32 = 0;
    let r_ = 'f: {
    l_tbl_2 = { let a_ = (({ let a_ = c.m.r32(sym::gBtlAi); c.m.r32(a_) }).wrapping_add(200)).wrapping_add(((((c.m.r32((p_s_1).wrapping_add(4)) as i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) };
    {
        l_i_3 = 0i32;
        'l1: loop {
            if !(((((l_i_3) < (14i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let v_ = ((0i32) as i8); let a_ = (((p_s_1).wrapping_add(744)).wrapping_add(88)).wrapping_add(((l_i_3) as u32)); c.m.w8(a_, (v_) as u8); };
                l_r_4 = F::AiThink_FindBasicColumn(c, l_i_3, 3i32);
                if (((((l_r_4) != ((1i32).wrapping_neg())) as i32)) != 0) {
                    { let v_ = ((F::BtlAi_GetPairRate(c, p_s_1, ((l_i_3) as u32), l_r_4, (l_tbl_2).wrapping_add(((4i32) as u32)), (l_tbl_2).wrapping_add(((708i32) as u32)), 0i32)) as i8); let a_ = (((p_s_1).wrapping_add(744)).wrapping_add(88)).wrapping_add(((l_i_3) as u32)); c.m.w8(a_, (v_) as u8); };
                }
            }
            { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiMgr_BuildRates", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiMgr_GetType(c: &mut Ctx, mut p_side_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiMgr_GetType", &PROBE_N, &[(p_side_1) as u32], false); }
    let r_ = 'f: {
    break 'f { let a_ = (((c.m.r32(sym::gBtlAi)).wrapping_add(16)).wrapping_add((((p_side_1) as u32)).wrapping_mul(1312))).wrapping_add(4); (c.m.r32(a_) as i32) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiMgr_GetType", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiMgr_SetType(c: &mut Ctx, mut p_side_1: i32, mut p_aiType_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiMgr_SetType", &PROBE_N, &[(p_side_1) as u32, (p_aiType_2) as u32], false); }
    let mut l_s_3: u32 = 0;
    let r_ = 'f: {
    l_s_3 = ((c.m.r32(sym::gBtlAi)).wrapping_add(16)).wrapping_add((((p_side_1) as u32)).wrapping_mul(1312));
    { let v_ = p_aiType_2; let a_ = (l_s_3).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
    F::BtlAiMgr_BuildRates(c, l_s_3);
    F::AiThink_BuildTotals(c, l_s_3);
    { let _ = F::memset(c, ((l_s_3).wrapping_add(744)).wrapping_add(172), 0i32, ((392i32) as u32)); };
    F::BtlAiSeq_Reset(c, (l_s_3).wrapping_add(40));
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiMgr_SetType", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiMgr_GetLevel(c: &mut Ctx, mut p_side_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiMgr_GetLevel", &PROBE_N, &[(p_side_1) as u32], false); }
    let r_ = 'f: {
    break 'f { let a_ = (((c.m.r32(sym::gBtlAi)).wrapping_add(16)).wrapping_add((((p_side_1) as u32)).wrapping_mul(1312))).wrapping_add(8); (c.m.r32(a_) as i32) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiMgr_GetLevel", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiMgr_SetLevel(c: &mut Ctx, mut p_side_1: i32, mut p_cpuLevel_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiMgr_SetLevel", &PROBE_N, &[(p_side_1) as u32, (p_cpuLevel_2) as u32], false); }
    let mut l_s_3: u32 = 0;
    let r_ = 'f: {
    l_s_3 = ((c.m.r32(sym::gBtlAi)).wrapping_add(16)).wrapping_add((((p_side_1) as u32)).wrapping_mul(1312));
    { let v_ = p_cpuLevel_2; let a_ = (l_s_3).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
    F::BtlAiMgr_BuildRates(c, l_s_3);
    F::AiThink_BuildTotals(c, l_s_3);
    { let _ = F::memset(c, ((l_s_3).wrapping_add(744)).wrapping_add(172), 0i32, ((392i32) as u32)); };
    F::BtlAiSeq_Reset(c, (l_s_3).wrapping_add(40));
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiMgr_SetLevel", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiMgr_IsIdleState(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiMgr_IsIdleState", &PROBE_N, &[], false); }
    let mut l_state_1: i32 = 0;
    let r_ = 'f: {
    l_state_1 = F::BtlSeq_GetState(c);
    if (((((((((l_state_1) == (2i32)) as i32)) != 0) || (((((l_state_1) == (3i32)) as i32)) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiMgr_IsIdleState", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiMgr_UpdateSight(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiMgr_UpdateSight", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(80) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 80);
    let r_ = 'f: {
        F::BtlCharApi_GetPos(c, 0i32, fp);
        { let r_ = rt::fmul(F::BtlCharApi_GetHeight(c, 0i32), F32(0x3f000000)); let a_ = (fp).wrapping_add(4); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        F::BtlCharApi_GetPos(c, 1i32, fp.wrapping_add(16));
        { let r_ = rt::fmul(F::BtlCharApi_GetHeight(c, 1i32), F32(0x3f000000)); let a_ = (fp.wrapping_add(16)).wrapping_add(4); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        F::Vec3_Sub(c, fp.wrapping_add(32), fp, fp.wrapping_add(16));
        { let v_ = F::Vec3_Length(c, fp.wrapping_add(32)); let a_ = (c.m.r32(sym::gBtlAi)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let s_ = fp; let d_ = fp.wrapping_add(48); rt::memcpy(c, d_, s_, 16); };
        { let s_ = fp.wrapping_add(16); let d_ = (fp.wrapping_add(48)).wrapping_add(16); rt::memcpy(c, d_, s_, 16); };
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gBtlAi)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        if ((F::StgCol_TraceSegment(c, fp.wrapping_add(48))) != 0) {
            { let r_ = 1i32; let a_ = (c.m.r32(sym::gBtlAi)).wrapping_add(12); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            if ((((({ let a_ = (F::StgCol_GetHit(c)).wrapping_add(64); (c.m.r32(a_) as i32) }) != ((1i32).wrapping_neg())) as i32)) != 0) {
                { let r_ = 2i32; let a_ = (c.m.r32(sym::gBtlAi)).wrapping_add(12); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlAiMgr_UpdateSight", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiMgr_GetFrame(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiMgr_GetFrame", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f { let a_ = (c.m.r32(sym::gBtlAi)).wrapping_add(2640); (c.m.r32(a_) as i32) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiMgr_GetFrame", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiMgr_Update(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiMgr_Update", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let mut l_s_2: u32 = 0;
    let r_ = 'f: {
    if (((((c.m.r32(sym::gBtlAi)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    if (((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0) {
        break 'f;
    }
    if (((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((4096i32) as u64)))) != 0) {
        break 'f;
    }
    if ((((!((F::BtlStage_IsReady(c)) != 0)) as i32)) != 0) {
        break 'f;
    }
    if ((F::BtlAiMgr_IsIdleState(c)) != 0) {
        break 'f;
    }
    F::BtlAiMgr_UpdateSight(c);
    { let a_ = (c.m.r32(sym::gBtlAi)).wrapping_add(2640); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((((((l_i_1) < (F::BtlChar_GetCount(c))) as i32)) != 0) && (((((l_i_1) < (2i32)) as i32)) != 0)) as i32)) != 0) { break 'l1; }
            'c2: {
                l_s_2 = ((c.m.r32(sym::gBtlAi)).wrapping_add(16)).wrapping_add((((l_i_1) as u32)).wrapping_mul(1312));
                if ((((((F::BtlCharApi_IsInputInjected(c, l_i_1)) != 0) && (((((c.m.r32((l_s_2).wrapping_add(24))) != (((0i32) as u32))) as i32)) != 0)) as i32)) != 0) {
                    F::BtlAiSense_Update(c, l_s_2);
                    F::AiThink_Think(c, l_s_2);
                    F::BtlAi_RunSeq(c, l_s_2);
                    F::BtlAi_SendInput(c, l_s_2);
                }
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiMgr_Update", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiMove_PickDir(c: &mut Ctx, mut p_s_1: u32, mut p_mode_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiMove_PickDir", &PROBE_N, &[(p_s_1) as u32, (p_mode_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(112) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 112);
    let mut l_list_3: u32 = 0;
    let mut l_n_4: i32 = 0;
    let mut l_top_5: F32 = F32(0);
    let mut l_rad_6: F32 = F32(0);
    let mut bb_: u32 = 0;
    let r_ = 'f: {
        'fl1: loop {
            match bb_ {
                0 => {
                    l_n_4 = 1i32;
                    l_top_5 = rt::fadd(F::BtlStage_GetTop(c), F32(0x41a00000));
                    l_rad_6 = rt::fsub(F::BtlStage_GetInnerRadius(c), F32(0x42c80000));
                    { let a0_ = (c.m.r32(p_s_1) as i32); F::BtlCharApi_GetPos(c, a0_, fp) };
                    { let a0_ = (((c.m.r32(p_s_1) as i32)) ^ (1i32)); F::BtlCharApi_GetPos(c, a0_, fp.wrapping_add(16)) };
                    l_list_3 = fp.wrapping_add(80);
                    { let v_ = 0i32; let a_ = (l_list_3).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    if (((((p_mode_2) == (2i32)) as i32)) != 0) { bb_ = 1; continue 'fl1; } else { bb_ = 2; continue 'fl1; }
                }
                1 => {
                    if (((((((rt::flt(l_top_5, F32(c.m.r32((fp).wrapping_add(4)))) as i32)) != 0) || (((rt::flt(l_top_5, F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4)))) as i32)) != 0)) as i32)) != 0) { bb_ = 3; continue 'fl1; } else { bb_ = 4; continue 'fl1; }
                }
                2 => {
                    if (((rt::flt(l_rad_6, F::Vec3_Length(c, fp)) as i32)) != 0) { bb_ = 7; continue 'fl1; } else { bb_ = 9; continue 'fl1; }
                }
                3 => {
                    { let v_ = 1i32; let a_ = (fp.wrapping_add(80)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    l_n_4 = 2i32;
                    bb_ = 4; continue 'fl1;
                }
                4 => {
                    if (((((((rt::flt(F32(c.m.r32((fp).wrapping_add(4))), rt::fsub({ let a0_ = (c.m.r32(p_s_1) as i32); F::BtlCharApi_GetGroundY(c, a0_) }, F32(0x41a00000))) as i32)) != 0) || (((rt::flt(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))), rt::fsub({ let a0_ = (((c.m.r32(p_s_1) as i32)) ^ (1i32)); F::BtlCharApi_GetGroundY(c, a0_) }, F32(0x41a00000))) as i32)) != 0)) as i32)) != 0) { bb_ = 5; continue 'fl1; } else { bb_ = 6; continue 'fl1; }
                }
                5 => {
                    { let v_ = 2i32; let a_ = (l_list_3).wrapping_add(((({ let o_ = l_n_4; let n_ = (o_).wrapping_add(1); l_n_4 = n_; o_ }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    bb_ = 6; continue 'fl1;
                }
                6 => {
                    bb_ = 2; continue 'fl1;
                }
                7 => {
                    F::Vec3_Sub(c, fp.wrapping_add(32), fp.wrapping_add(16), fp);
                    { let v_ = F32(0x00000000); let a_ = (fp.wrapping_add(32)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    F::Vec3_Normalize(c, fp.wrapping_add(32), fp.wrapping_add(32));
                    F::Vec3_Normalize(c, fp.wrapping_add(48), fp);
                    if (((((((rt::flt(F32(0x00000000), F::Vec3_Length(c, fp.wrapping_add(32))) as i32)) != 0) && (((rt::flt(F32(0x00000000), F::Vec3_Length(c, fp.wrapping_add(48))) as i32)) != 0)) as i32)) != 0) { bb_ = 10; continue 'fl1; } else { bb_ = 11; continue 'fl1; }
                }
                8 => {
                    break 'f { let a_ = (l_list_3).wrapping_add((F::Rand_Range(c, ((l_n_4) as u32))).wrapping_mul(4)); (c.m.r32(a_) as i32) };
                }
                9 => {
                    bb_ = 14; continue 'fl1;
                }
                10 => {
                    if (((rt::flt(F32(0x00000000), F::Vec3_Dot(c, fp.wrapping_add(48), fp.wrapping_add(32))) as i32)) != 0) { bb_ = 12; continue 'fl1; } else { bb_ = 13; continue 'fl1; }
                }
                11 => {
                    bb_ = 8; continue 'fl1;
                }
                12 => {
                    bb_ = 14; continue 'fl1;
                }
                13 => {
                    F::Vec3_RotateY(c, fp.wrapping_add(64), fp.wrapping_add(32), F32(0xbf860a91));
                    F::Vec3_Normalize(c, fp.wrapping_add(64), fp.wrapping_add(64));
                    if (((rt::flt(F32(0x00000000), F::Vec3_Dot(c, fp.wrapping_add(48), fp.wrapping_add(64))) as i32)) != 0) { bb_ = 16; continue 'fl1; } else { bb_ = 17; continue 'fl1; }
                }
                14 => {
                    { let v_ = 3i32; let a_ = (l_list_3).wrapping_add(((({ let o_ = l_n_4; let n_ = (o_).wrapping_add(1); l_n_4 = n_; o_ }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 4i32; let a_ = (l_list_3).wrapping_add(((({ let o_ = l_n_4; let n_ = (o_).wrapping_add(1); l_n_4 = n_; o_ }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    bb_ = 8; continue 'fl1;
                }
                15 => {
                    bb_ = 13; continue 'fl1;
                }
                16 => {
                    { let v_ = 3i32; let a_ = (l_list_3).wrapping_add(((({ let o_ = l_n_4; let n_ = (o_).wrapping_add(1); l_n_4 = n_; o_ }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    bb_ = 17; continue 'fl1;
                }
                17 => {
                    F::Vec3_RotateY(c, fp.wrapping_add(64), fp.wrapping_add(32), F32(0x3f860a91));
                    F::Vec3_Normalize(c, fp.wrapping_add(64), fp.wrapping_add(64));
                    if (((rt::flt(F32(0x00000000), F::Vec3_Dot(c, fp.wrapping_add(48), fp.wrapping_add(64))) as i32)) != 0) { bb_ = 18; continue 'fl1; } else { bb_ = 19; continue 'fl1; }
                }
                18 => {
                    { let v_ = 4i32; let a_ = (l_list_3).wrapping_add(((({ let o_ = l_n_4; let n_ = (o_).wrapping_add(1); l_n_4 = n_; o_ }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    bb_ = 19; continue 'fl1;
                }
                19 => {
                    bb_ = 11; continue 'fl1;
                }
                20 => {
                    break 'fl1;
                }
                _ => unreachable!(),
            }
        }
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlAiMove_PickDir", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiMove_CalcTarget(c: &mut Ctx, mut p_s_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiMove_CalcTarget", &PROBE_N, &[(p_s_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(128) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 128);
    let mut l_act_2: u32 = 0;
    let mut l_m_3: u32 = 0;
    let mut l_rad_4: F32 = F32(0);
    let mut l_special_5: i32 = 0;
    let mut l_reach_6: F32 = F32(0);
    let r_ = 'f: {
        l_act_2 = (p_s_1).wrapping_add(40);
        l_m_3 = (p_s_1).wrapping_add(192);
        l_rad_4 = F::BtlStage_GetRadius(c);
        l_special_5 = F::BtlChar_IsStage4Or27(c);
        { let a0_ = (c.m.r32(p_s_1) as i32); F::BtlCharApi_GetPos(c, a0_, fp) };
        { let s_ = fp; let d_ = fp.wrapping_add(32); rt::memcpy(c, d_, s_, 16); };
        { let a0_ = (((c.m.r32(p_s_1) as i32)) ^ (1i32)); F::BtlCharApi_GetPos(c, a0_, fp.wrapping_add(16)) };
        { let v_ = F32(c.m.r32((fp).wrapping_add(4))); let a_ = (fp.wrapping_add(32)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        F::Vec3_Sub(c, fp.wrapping_add(48), fp.wrapping_add(32), fp.wrapping_add(16));
        F::Vec3_Normalize(c, fp.wrapping_add(48), fp.wrapping_add(48));
        { let a2_ = { let a_ = (l_m_3).wrapping_add(((((c.m.r32((l_m_3).wrapping_add(308)) as i32)) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }; F::Vec4_Scale(c, fp.wrapping_add(32), fp.wrapping_add(48), a2_) };
        if (((((((((l_special_5) != (0i32)) as i32)) != 0) && ((((((((c.m.r32((l_m_3).wrapping_add(308)) as i32)) as u32)) >= (((2i32) as u32))) as i32)) != 0)) as i32)) != 0) {
            F::Vec4_SetZero(c, (l_m_3).wrapping_add(320));
            { let v_ = F32(c.m.r32((fp).wrapping_add(4))); let a_ = ((l_m_3).wrapping_add(320)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        } else {
            if ((((((c.m.r32((l_m_3).wrapping_add(308)) as i32)) == (4i32)) as i32)) != 0) {
                { let a0_ = (c.m.r32(p_s_1) as i32); { let _ = F::BtlStage_GetStartPlace(c, a0_, (l_m_3).wrapping_add(320), fp.wrapping_add(80), 0i32); } };
                { let r_ = F32(0x41a00000); let a_ = ((l_m_3).wrapping_add(320)).wrapping_add(4); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                { let v_ = rt::i2f((((F::Rand_Range(c, ((400i32) as u32))).wrapping_sub(((200i32) as u32))) as i32)); let a_ = (l_m_3).wrapping_add(320); c.m.w32(a_, (v_).0); };
                { let v_ = rt::i2f((((F::Rand_Range(c, ((400i32) as u32))).wrapping_sub(((200i32) as u32))) as i32)); let a_ = ((l_m_3).wrapping_add(320)).wrapping_add(8); c.m.w32(a_, (v_).0); };
            } else {
                F::Vec3_Add(c, (l_m_3).wrapping_add(320), fp.wrapping_add(16), fp.wrapping_add(32));
            }
        }
        { let r_ = rt::fmul({ let a0_ = (c.m.r32(p_s_1) as i32); F::BtlCharApi_GetHeight(c, a0_) }, F32(0x3f000000)); let a_ = (fp).wrapping_add(4); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        { let r_ = (!(((8i32) | (16i32)))); let a_ = l_act_2; let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
        if (((rt::flt(l_rad_4, F::Vec3_Length(c, (l_m_3).wrapping_add(320))) as i32)) != 0) {
            { let a2_ = rt::fsub(l_rad_4, F::Vec3_Length(c, fp)); F::Vec4_Scale(c, fp.wrapping_add(32), fp.wrapping_add(48), a2_) };
            F::Vec3_Add(c, (l_m_3).wrapping_add(320), fp, fp.wrapping_add(32));
        }
        { let s_ = fp; let d_ = fp.wrapping_add(96); rt::memcpy(c, d_, s_, 16); };
        { let s_ = (l_m_3).wrapping_add(320); let d_ = (fp.wrapping_add(96)).wrapping_add(16); rt::memcpy(c, d_, s_, 16); };
        if ((F::StgCol_TraceSegment(c, fp.wrapping_add(96))) != 0) {
            { let r_ = 8i32; let a_ = l_act_2; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            if ((((({ let a_ = (F::StgCol_GetHit(c)).wrapping_add(64); (c.m.r32(a_) as i32) }) != ((1i32).wrapping_neg())) as i32)) != 0) {
                { let r_ = 16i32; let a_ = l_act_2; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            }
        }
        l_reach_6 = { let a0_ = (c.m.r32(p_s_1) as i32); F::BtlCharApi_GetRadius(c, a0_) };
        { let r_ = rt::fmul({ let a0_ = (c.m.r32(p_s_1) as i32); F::BtlCharApi_GetSpeed(c, a0_) }, F32(0x40400000)); let n_ = rt::fadd(l_reach_6, r_); l_reach_6 = n_; };
        { let a0_ = (c.m.r32(p_s_1) as i32); F::BtlCharApi_GetVelocity(c, a0_, fp.wrapping_add(48)) };
        F::Vec3_Normalize(c, fp.wrapping_add(48), fp.wrapping_add(48));
        F::Vec4_Scale(c, fp.wrapping_add(32), fp.wrapping_add(48), l_reach_6);
        F::Vec3_Add(c, fp.wrapping_add(64), fp, fp.wrapping_add(32));
        if (((rt::flt(l_rad_4, F::Vec3_Length(c, fp.wrapping_add(64))) as i32)) != 0) {
            { let a2_ = rt::fsub(l_rad_4, F::Vec3_Length(c, fp)); F::Vec4_Scale(c, fp.wrapping_add(32), fp.wrapping_add(48), a2_) };
            F::Vec3_Add(c, fp.wrapping_add(64), fp, fp.wrapping_add(32));
        }
        { let r_ = (!(4i32)); let a_ = l_act_2; let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
        { let v_ = F32(c.m.r32((fp).wrapping_add(4))); let a_ = (fp.wrapping_add(64)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let s_ = fp; let d_ = fp.wrapping_add(96); rt::memcpy(c, d_, s_, 16); };
        { let s_ = fp.wrapping_add(64); let d_ = (fp.wrapping_add(96)).wrapping_add(16); rt::memcpy(c, d_, s_, 16); };
        if ((F::StgCol_TraceSegment(c, fp.wrapping_add(96))) != 0) {
            { let r_ = 4i32; let a_ = l_act_2; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlAiMove_CalcTarget", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiMove_IsNear(c: &mut Ctx, mut p_s_1: u32, mut p_p_2: u32, mut p_flat_3: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiMove_IsNear", &PROBE_N, &[(p_s_1) as u32, (p_p_2) as u32, (p_flat_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_m_4: u32 = 0;
    let mut l_len_5: F32 = F32(0);
    let r_ = 'f: {
        l_m_4 = (p_s_1).wrapping_add(192);
        { let a0_ = (c.m.r32(p_s_1) as i32); F::BtlCharApi_GetPos(c, a0_, fp) };
        F::Vec3_Sub(c, fp.wrapping_add(16), fp, p_p_2);
        if ((((((((((c.m.r32((l_m_4).wrapping_add(308)) as i32)) != (1i32)) as i32)) != 0) || (((((p_flat_3) != (0i32)) as i32)) != 0)) as i32)) != 0) {
            { let v_ = F32(0x00000000); let a_ = (fp.wrapping_add(16)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        }
        l_len_5 = F::Vec3_Length(c, fp.wrapping_add(16));
        if (((rt::flt(l_len_5, F32(0x41a00000)) as i32)) != 0) {
            break 'f 1i32;
        }
        if (((rt::flt(l_len_5, rt::fmul({ let a0_ = (c.m.r32(p_s_1) as i32); F::BtlCharApi_GetSpeed(c, a0_) }, F32(0x40a00000))) as i32)) != 0) {
            break 'f 1i32;
        }
        break 'f 0i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlAiMove_IsNear", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiMove_BuildPath(c: &mut Ctx, mut p_s_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiMove_BuildPath", &PROBE_N, &[(p_s_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_path_2: u32 = 0;
    let mut l_m_3: u32 = 0;
    let r_ = 'f: {
        l_path_2 = ((p_s_1).wrapping_add(192)).wrapping_add(32);
        l_m_3 = (p_s_1).wrapping_add(192);
        { let a0_ = (c.m.r32(p_s_1) as i32); F::BtlCharApi_GetPos(c, a0_, fp) };
        F::StgNav_FindPath(c, fp, ((p_s_1).wrapping_add(192)).wrapping_add(320), l_path_2);
        { let v_ = 60i32; let a_ = (l_m_3).wrapping_add(304); c.m.w32(a_, (v_) as u32); };
        if ((((((c.m.r32((l_path_2).wrapping_add(256)) as i32)) > (0i32)) as i32)) != 0) {
            { let a1_ = (l_path_2).wrapping_add((((((c.m.r32((l_path_2).wrapping_add(256)) as i32)).wrapping_sub(1i32)) as u32)).wrapping_mul(16)); F::Vec3_Sub(c, fp.wrapping_add(16), a1_, fp) };
            { let v_ = F::Vec3_Length(c, fp.wrapping_add(16)); let a_ = (l_m_3).wrapping_add(336); c.m.w32(a_, (v_).0); };
            break 'f 0i32;
        }
        { let v_ = F32(0x00000000); let a_ = (l_m_3).wrapping_add(336); c.m.w32(a_, (v_).0); };
        break 'f 1i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlAiMove_BuildPath", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiMove_Check(c: &mut Ctx, mut p_s_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiMove_Check", &PROBE_N, &[(p_s_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(80) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 80);
    let mut l_skip_2: i32 = 0;
    let mut l_m_3: u32 = 0;
    let mut l_act_4: u32 = 0;
    let mut l_path_5: u32 = 0;
    let mut l_state_6: i32 = 0;
    let r_ = 'f: {
        l_skip_2 = 0i32;
        l_m_3 = (p_s_1).wrapping_add(192);
        l_act_4 = (p_s_1).wrapping_add(40);
        l_path_5 = ((p_s_1).wrapping_add(192)).wrapping_add(32);
        l_state_6 = { let a0_ = (c.m.r32(p_s_1) as i32); F::BtlCharApi_GetUnk974(c, a0_) };
        if ((((((c.m.r32((l_m_3).wrapping_add(304)) as i32)) > (0i32)) as i32)) != 0) {
            { let a_ = (l_m_3).wrapping_add(304); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
        }
        if (((((c.m.r32(l_act_4) as i32)) & (2i32))) != 0) {
            if ((((!(((((c.m.r32(l_act_4) as i32)) & (8i32))) != 0)) as i32)) != 0) {
                break 'f 0i32;
            }
            if ((((((c.m.r32((l_path_5).wrapping_add(256)) as i32)) == (0i32)) as i32)) != 0) {
                break 'f 0i32;
            }
            { let a0_ = (c.m.r32(p_s_1) as i32); F::BtlCharApi_GetPos(c, a0_, fp) };
            if ((((((c.m.r32((l_path_5).wrapping_add(256)) as i32)) >= (2i32)) as i32)) != 0) {
                { let s_ = fp; let d_ = fp.wrapping_add(48); rt::memcpy(c, d_, s_, 16); };
                { let s_ = (l_path_5).wrapping_add((((((c.m.r32((l_path_5).wrapping_add(256)) as i32)).wrapping_sub(2i32)) as u32)).wrapping_mul(16)); let d_ = (fp.wrapping_add(48)).wrapping_add(16); rt::memcpy(c, d_, s_, 16); };
                l_skip_2 = (((F::StgCol_TraceSegment(c, fp.wrapping_add(48))) == (0i32)) as i32);
            }
            if ((((((l_skip_2) != 0) || (({ let a1_ = (l_path_5).wrapping_add((((((c.m.r32((l_path_5).wrapping_add(256)) as i32)).wrapping_sub(1i32)) as u32)).wrapping_mul(16)); F::BtlAiMove_IsNear(c, p_s_1, a1_, 1i32) }) != 0)) as i32)) != 0) {
                { let a_ = (l_path_5).wrapping_add(256); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (l_m_3).wrapping_add(372); c.m.w32(a_, (v_) as u32); };
                if ((((((c.m.r32((l_path_5).wrapping_add(256)) as i32)) == (0i32)) as i32)) != 0) {
                    break 'f 1i32;
                }
                if ((((((((l_state_6).wrapping_sub(31i32)) as u32)) < (((4i32) as u32))) as i32)) != 0) {
                    break 'f 4i32;
                }
                { let a1_ = (l_path_5).wrapping_add((((((c.m.r32((l_path_5).wrapping_add(256)) as i32)).wrapping_sub(1i32)) as u32)).wrapping_mul(16)); F::Vec3_Sub(c, fp.wrapping_add(32), a1_, fp) };
                { let v_ = F::Vec3_Length(c, fp.wrapping_add(32)); let a_ = (l_m_3).wrapping_add(336); c.m.w32(a_, (v_).0); };
            }
            if ((((((c.m.r32((l_m_3).wrapping_add(304)) as i32)) == (0i32)) as i32)) != 0) {
                { let a0_ = (((c.m.r32(p_s_1) as i32)) ^ (1i32)); F::BtlCharApi_GetPos(c, a0_, fp.wrapping_add(16)) };
                F::Vec3_Sub(c, fp.wrapping_add(32), fp.wrapping_add(16), l_path_5);
                if (((rt::flt(F32(0x42480000), F::Vec3_Length(c, fp.wrapping_add(32))) as i32)) != 0) {
                    if ((F::BtlAiMove_BuildPath(c, p_s_1)) != 0) {
                        break 'f 0i32;
                    }
                }
            }
            break 'f 3i32;
        }
        if ((F::BtlAiMove_IsNear(c, p_s_1, (l_m_3).wrapping_add(320), 0i32)) != 0) {
            if ((((!(((({ let a_ = (c.m.r32(sym::gBtlAi)).wrapping_add(12); (c.m.r32(a_) as i32) }) & (1i32))) != 0)) as i32)) != 0) {
                break 'f 1i32;
            }
        }
        if ((((!(((((c.m.r32(l_act_4) as i32)) & (8i32))) != 0)) as i32)) != 0) {
            break 'f 2i32;
        }
        if (((((((((c.m.r32(l_act_4) as i32)) & (16i32))) != 0) && ((((((c.m.r32((l_m_3).wrapping_add(340)) as i32)) == (2i32)) as i32)) != 0)) as i32)) != 0) {
            break 'f 2i32;
        }
        break 'f (if ((F::BtlAiMove_BuildPath(c, p_s_1)) != 0) { 0i32 } else { 3i32 });
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlAiMove_Check", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiMove_Steer(c: &mut Ctx, mut p_s_1: u32, mut p_to_2: u32, mut p_kind_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiMove_Steer", &PROBE_N, &[(p_s_1) as u32, (p_to_2) as u32, (p_kind_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let mut l_flags_4: u32 = 0;
    let mut l_cls_5: i32 = 0;
    let mut l_hold_6: i32 = 0;
    let mut l_m_7: u32 = 0;
    let mut l_press_8: i32 = 0;
    let mut l_chr_9: u32 = 0;
    let mut l_tbl_10: u32 = 0;
    let mut l_side_11: i32 = 0;
    let mut l_mgr_12: u32 = 0;
    let mut l_fly_13: i32 = 0;
    let mut l_dy_14: F32 = F32(0);
    let mut l_len_15: F32 = F32(0);
    let mut l_yaw_16: F32 = F32(0);
    let mut l_x_17: F32 = F32(0);
    let mut l_y_18: F32 = F32(0);
    let r_ = 'f: {
        l_hold_6 = 0i32;
        l_m_7 = (p_s_1).wrapping_add(192);
        l_press_8 = 0i32;
        l_chr_9 = { let a0_ = (c.m.r32(p_s_1) as i32); F::BtlChar_Get(c, a0_) };
        l_mgr_12 = c.m.r32(sym::gBtlAi);
        l_side_11 = (c.m.r32(p_s_1) as i32);
        l_flags_4 = (p_s_1).wrapping_add(40);
        l_tbl_10 = { let a_ = (c.m.r32(l_mgr_12)).wrapping_add(164); c.m.r32(a_) };
        l_cls_5 = (({ let a_ = ((l_tbl_10).wrapping_add(904)).wrapping_add(((F::BtlCharApi_GetUnk974(c, l_side_11)) as u32)); (c.m.r8(a_) as i8) }) as i32);
        l_fly_13 = { let a0_ = (c.m.r32(p_s_1) as i32); F::BtlCharApi_TestFlag0F(c, a0_) };
        { let a0_ = (c.m.r32(p_s_1) as i32); F::BtlCharApi_GetPos(c, a0_, fp) };
        { let a0_ = (((c.m.r32(p_s_1) as i32)) ^ (1i32)); F::BtlCharApi_GetPos(c, a0_, fp.wrapping_add(32)) };
        l_dy_14 = rt::fsub(F32(c.m.r32((p_to_2).wrapping_add(4))), F32(c.m.r32((fp).wrapping_add(4))));
        F::Vec3_Sub(c, fp.wrapping_add(16), p_to_2, fp);
        { let v_ = { let v_ = F32(0x00000000); let a_ = (fp.wrapping_add(16)).wrapping_add(12); c.m.w32(a_, (v_).0); v_ }; let a_ = (fp.wrapping_add(16)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        l_len_15 = F::Vec3_Length(c, fp.wrapping_add(16));
        if (((rt::flt(l_len_15, { let a0_ = (c.m.r32(p_s_1) as i32); F::BtlCharApi_GetRadius(c, a0_) }) as i32)) != 0) {
            l_len_15 = { let a0_ = (c.m.r32(p_s_1) as i32); F::BtlCharApi_GetRadius(c, a0_) };
        }
        F::Vec3_Normalize(c, fp.wrapping_add(16), fp.wrapping_add(16));
        l_yaw_16 = F32(c.m.r32((l_chr_9).wrapping_add(1092)));
        l_x_17 = rt::fsub(rt::fmul(F::Mathf_Cos(c, l_yaw_16), F32(c.m.r32(fp.wrapping_add(16)))), rt::fmul(F::Mathf_Sin(c, l_yaw_16), F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8)))));
        l_y_18 = rt::fneg(rt::fadd(rt::fmul(F::Mathf_Sin(c, l_yaw_16), F32(c.m.r32(fp.wrapping_add(16)))), rt::fmul(F::Mathf_Cos(c, l_yaw_16), F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8))))));
        if (((rt::flt(F32(0x3f800000), l_x_17) as i32)) != 0) {
            l_x_17 = F32(0x3f800000);
        } else {
            if (((rt::flt(l_x_17, F32(0xbf800000)) as i32)) != 0) {
                l_x_17 = F32(0xbf800000);
            }
        }
        if (((rt::flt(F32(0x3f800000), l_y_18) as i32)) != 0) {
            l_y_18 = F32(0x3f800000);
        } else {
            if (((rt::flt(l_y_18, F32(0xbf800000)) as i32)) != 0) {
                l_y_18 = F32(0xbf800000);
            }
        }
        if (((((({ let a0_ = (c.m.r32(p_s_1) as i32); F::BtlCharApi_IsClose(c, a0_) }) != 0) || (((((l_fly_13) != (0i32)) as i32)) != 0)) as i32)) != 0) {
            l_press_8 = 2i32;
        } else {
            l_hold_6 = 2i32;
        }
        if (((((p_kind_3) == (0i32)) as i32)) != 0) {
            if (((rt::flt(l_dy_14, F32(0xc1f00000)) as i32)) != 0) {
                if (((((((((l_fly_13) == (0i32)) as i32)) != 0) || ((((!(((rt::flt(F32(c.m.r32((fp).wrapping_add(4))), F32(c.m.r32((fp.wrapping_add(32)).wrapping_add(4)))) as i32)) != 0)) as i32)) != 0)) as i32)) != 0) {
                    { let r_ = 2048i32; let n_ = ((l_hold_6) | (r_)); l_hold_6 = n_; };
                }
            } else {
                if (((rt::flt(F32(0x41f00000), l_dy_14) as i32)) != 0) {
                    { let r_ = 4096i32; let n_ = ((l_hold_6) | (r_)); l_hold_6 = n_; };
                }
            }
        }
        if (((((c.m.r32(l_flags_4) as i32)) & (4i32))) != 0) {
            if ((((!((((l_hold_6) & (4096i32))) != 0)) as i32)) != 0) {
                { let r_ = 2048i32; let n_ = ((l_hold_6) | (r_)); l_hold_6 = n_; };
            }
        }
        if (((((l_cls_5) == (11i32)) as i32)) != 0) {
            { let r_ = (!(2i32)); let n_ = ((l_press_8) & (r_)); l_press_8 = n_; };
        }
        if (((((({ let a0_ = (c.m.r32(p_s_1) as i32); F::BtlCharApi_TestFlag0F(c, a0_) }) != 0) && ((((!(({ let a0_ = (((c.m.r32(p_s_1) as i32)) ^ (1i32)); F::BtlCharApi_TestFlag0F(c, a0_) }) != 0)) as i32)) != 0)) as i32)) != 0) {
            if (((rt::fgt(F32(c.m.r32((fp).wrapping_add(4))), F32(c.m.r32((fp.wrapping_add(32)).wrapping_add(4)))) as i32)) != 0) {
                { let r_ = 2048i32; let n_ = ((l_hold_6) | (r_)); l_hold_6 = n_; };
            }
        }
        if (((((((((((rt::flt(l_len_15, F32(c.m.r32((l_m_7).wrapping_add((((2i32) as u32)).wrapping_mul(4))))) as i32)) != 0) && (((((p_kind_3) == (1i32)) as i32)) != 0)) as i32)) != 0) && ((((((((c.m.r32((l_m_7).wrapping_add(308)) as i32)) as u32)) < (((2i32) as u32))) as i32)) != 0)) as i32)) != 0) {
            l_y_18 = F32(0x00000000);
            { let r_ = 16i32; let n_ = ((l_hold_6) | (r_)); l_hold_6 = n_; };
            l_x_17 = l_y_18;
        }
        if (({ let a0_ = (c.m.r32(p_s_1) as i32); F::BtlCharApi_IsClose(c, a0_) }) != 0) {
            if ((((!((((l_hold_6) & (2048i32))) != 0)) as i32)) != 0) {
                if ((((((((c.m.r32((l_m_7).wrapping_add(308)) as i32)) as u32)) >= (((2i32) as u32))) as i32)) != 0) {
                    { let r_ = 512i32; let n_ = ((l_hold_6) | (r_)); l_hold_6 = n_; };
                }
                if (((rt::fgt((if (((rt::flt(l_x_17, F32(0x00000000)) as i32)) != 0) { rt::fneg(l_x_17) } else { l_x_17 }), (if (((rt::flt(l_y_18, F32(0x00000000)) as i32)) != 0) { rt::fneg(l_y_18) } else { l_y_18 })) as i32)) != 0) {
                    l_y_18 = F32(0x00000000);
                } else {
                    l_x_17 = F32(0x00000000);
                }
            }
        }
        F::BtlAiPad_Set(c, p_s_1, l_hold_6, l_press_8, 0i32, ((0i32) as i16), l_x_17, l_y_18);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlAiMove_Steer", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiMove_Hold(c: &mut Ctx, mut p_s_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiMove_Hold", &PROBE_N, &[(p_s_1) as u32], false); }
    let mut l_m_2: u32 = 0;
    let mut l_act_3: u32 = 0;
    let mut l_y_4: F32 = F32(0);
    let mut l_x_5: F32 = F32(0);
    let mut l_tbl_6: u32 = 0;
    let mut l_hold_7: i32 = 0;
    let mut l_cls_8: i32 = 0;
    let mut l_state_9: i32 = 0;
    let r_ = 'f: {
    l_m_2 = (p_s_1).wrapping_add(192);
    l_act_3 = (p_s_1).wrapping_add(40);
    l_y_4 = F32(0x00000000);
    l_tbl_6 = { let a_ = ({ let a_ = c.m.r32(sym::gBtlAi); c.m.r32(a_) }).wrapping_add(164); c.m.r32(a_) };
    l_state_9 = { let a0_ = (c.m.r32(p_s_1) as i32); F::BtlCharApi_GetUnk974(c, a0_) };
    l_hold_7 = 2i32;
    if ((((((c.m.r32((l_m_2).wrapping_add(340)) as i32)) == (2i32)) as i32)) != 0) {
        l_hold_7 = ((2i32) | (512i32));
    }
    if (((((c.m.r32(l_act_3) as i32)) & (4i32))) != 0) {
        { let r_ = 2048i32; let n_ = ((l_hold_7) | (r_)); l_hold_7 = n_; };
    }
    l_cls_8 = (((c.m.r8(((l_tbl_6).wrapping_add(904)).wrapping_add(((l_state_9) as u32))) as i8)) as i32);
    l_x_5 = l_y_4;
    if (((((l_cls_8) == (10i32)) as i32)) != 0) {
        l_y_4 = F32(0xbf800000);
    }
    F::BtlAiPad_Set(c, p_s_1, l_hold_7, 0i32, 0i32, ((0i32) as i16), l_x_5, l_y_4);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiMove_Hold", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiMove_Init(c: &mut Ctx, mut p_s_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiMove_Init", &PROBE_N, &[(p_s_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = 1i32; let a_ = ((p_s_1).wrapping_add(40)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
    F::BtlAiMove_Start(c, p_s_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiMove_Init", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiMove_Start(c: &mut Ctx, mut p_s_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiMove_Start", &PROBE_N, &[(p_s_1) as u32], false); }
    let mut l_act_2: u32 = 0;
    let mut l_m_3: u32 = 0;
    let mut l_e_4: u32 = 0;
    let mut l_v_5: i32 = 0;
    let mut bb_: u32 = 0;
    let r_ = 'f: {
    'fl1: loop {
        match bb_ {
            0 => {
                l_act_2 = (p_s_1).wrapping_add(40);
                l_m_3 = (p_s_1).wrapping_add(192);
                l_e_4 = ((l_act_2).wrapping_add(20)).wrapping_add((((((c.m.r32((l_act_2).wrapping_add(4)) as i32)).wrapping_sub(1i32)) as u32)).wrapping_mul(8));
                l_v_5 = { let a0_ = (((c.m.r8((l_e_4).wrapping_add(4)) as i8)) as i32); F::btl_ai_mgr__BtlAiMove_Div(c, a0_, 10i32) };
                F::BtlAiPad_Clear(c, (p_s_1).wrapping_add(616), 0i32);
                if (((((l_v_5) == (1i32)) as i32)) != 0) { bb_ = 1; continue 'fl1; } else { bb_ = 2; continue 'fl1; }
            }
            1 => {
                bb_ = 3; continue 'fl1;
            }
            2 => {
                if (((((l_v_5) == (2i32)) as i32)) != 0) { bb_ = 5; continue 'fl1; } else { bb_ = 6; continue 'fl1; }
            }
            3 => {
                { let v_ = l_v_5; let a_ = (l_m_3).wrapping_add(340); c.m.w32(a_, (v_) as u32); };
                bb_ = 6; continue 'fl1;
            }
            4 => {
                bb_ = 2; continue 'fl1;
            }
            5 => {
                bb_ = 3; continue 'fl1;
            }
            6 => {
                l_v_5 = { let a0_ = (((c.m.r8((l_e_4).wrapping_add(4)) as i8)) as i32); F::btl_ai_mgr__BtlAiMove_Mod(c, a0_, 10i32) };
                { let v_ = l_v_5; let a_ = (l_m_3).wrapping_add(308); c.m.w32(a_, (v_) as u32); };
                { let v_ = { let a1_ = (c.m.r32((l_m_3).wrapping_add(340)) as i32); F::BtlAiMove_PickDir(c, p_s_1, a1_) }; let a_ = (l_m_3).wrapping_add(344); c.m.w32(a_, (v_) as u32); };
                { let v_ = { let a_ = (c.m.r32(sym::gBtlAi)).wrapping_add(4); F32(c.m.r32(a_)) }; let a_ = (l_m_3).wrapping_add(336); c.m.w32(a_, (v_).0); };
                { let r_ = (!(2i32)); let a_ = l_act_2; let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (l_m_3).wrapping_add(372); c.m.w32(a_, (v_) as u32); };
                F::BtlAiMove_CalcTarget(c, p_s_1);
                { let v_ = 2i32; let a_ = (l_act_2).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                F::BtlAiMove_Run(c, p_s_1);
                break 'fl1;
            }
            _ => unreachable!(),
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiMove_Start", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiMove_Run(c: &mut Ctx, mut p_s_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiMove_Run", &PROBE_N, &[(p_s_1) as u32], false); }
    let mut l_m_2: u32 = 0;
    let mut l_act_3: u32 = 0;
    let mut l_path_4: u32 = 0;
    let r_ = 'f: {
    l_m_2 = (p_s_1).wrapping_add(192);
    l_act_3 = (p_s_1).wrapping_add(40);
    l_path_4 = ((p_s_1).wrapping_add(192)).wrapping_add(32);
    's1: {
        let sv5: i32 = F::BtlAiMove_Check(c, p_s_1);
        match sv5 {
            0i32 => {
                F::BtlAiMove_CalcTarget(c, p_s_1);
                { let v_ = 1i32; let a_ = (l_act_3).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            1i32 => {
                { let v_ = 3i32; let a_ = (l_act_3).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            2i32 => {
                { let r_ = (!(2i32)); let a_ = l_act_3; let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                F::BtlAiPad_Clear(c, (p_s_1).wrapping_add(616), 1i32);
                if ((((((c.m.r32((l_m_2).wrapping_add(308)) as i32)) == (0i32)) as i32)) != 0) {
                    F::BtlAiMove_Hold(c, p_s_1);
                } else {
                    F::BtlAiMove_Steer(c, p_s_1, (l_m_2).wrapping_add(320), 1i32);
                }
                break 's1;
            }
            3i32 => {
                { let r_ = 2i32; let a_ = l_act_3; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a1_ = (l_path_4).wrapping_add((((((c.m.r32((l_path_4).wrapping_add(256)) as i32)).wrapping_sub(1i32)) as u32)).wrapping_mul(16)); F::BtlAiMove_Steer(c, p_s_1, a1_, 0i32) };
                break 's1;
            }
            4i32 => {
                F::BtlAiPad_Set(c, p_s_1, 0i32, 0i32, 2048i32, ((0i32) as i16), F32(0x00000000), F32(0x00000000));
                break 's1;
            }
            _ => {
            }
        }
    }
    if (((((c.m.r32((l_m_2).wrapping_add(348)) as i32)) & (1i32))) != 0) {
        { let v_ = 3i32; let a_ = (l_act_3).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let r_ = (!(1i32)); let a_ = (l_m_2).wrapping_add(348); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
    }
    if ((((({ let a_ = (l_m_2).wrapping_add(372); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); n_ }) > (60i32)) as i32)) != 0) {
        { let v_ = 3i32; let a_ = (l_act_3).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
    }
    if ((((((c.m.r32((l_m_2).wrapping_add(308)) as i32)) == (4i32)) as i32)) != 0) {
        if (((rt::flt({ let a_ = (c.m.r32(sym::gBtlAi)).wrapping_add(4); F32(c.m.r32(a_)) }, F32(c.m.r32((l_m_2).wrapping_add((((1i32) as u32)).wrapping_mul(4))))) as i32)) != 0) {
            { let v_ = 3i32; let a_ = (l_act_3).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiMove_Run", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAiMove_End(c: &mut Ctx, mut p_s_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAiMove_End", &PROBE_N, &[(p_s_1) as u32], false); }
    let mut l_act_2: u32 = 0;
    let mut l_tbl_3: u32 = 0;
    let mut l_state_4: i32 = 0;
    let mut l_cls_5: i32 = 0;
    let r_ = 'f: {
    l_act_2 = (p_s_1).wrapping_add(40);
    l_tbl_3 = { let a_ = ({ let a_ = c.m.r32(sym::gBtlAi); c.m.r32(a_) }).wrapping_add(164); c.m.r32(a_) };
    l_state_4 = { let a0_ = (c.m.r32(p_s_1) as i32); F::BtlCharApi_GetUnk974(c, a0_) };
    l_cls_5 = (((c.m.r8(((l_tbl_3).wrapping_add(904)).wrapping_add(((l_state_4) as u32))) as i8)) as i32);
    F::BtlAiPad_Clear(c, (p_s_1).wrapping_add(616), 1i32);
    if (((((l_cls_5) == (10i32)) as i32)) != 0) {
        F::BtlAiPad_Set(c, p_s_1, 0i32, 2i32, 0i32, ((0i32) as i16), F32(0x00000000), F32(0x00000000));
    }
    if ((((((((l_state_4).wrapping_sub(31i32)) as u32)) < (((4i32) as u32))) as i32)) != 0) {
        F::BtlAiPad_Set(c, p_s_1, 2048i32, 0i32, 0i32, ((0i32) as i16), F32(0x00000000), F32(0x00000000));
    }
    if (((((l_cls_5) == (0i32)) as i32)) != 0) {
        { let a_ = (l_act_2).wrapping_add(4); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAiMove_End", &PROBE_N, &[], true); }
    r_
}
