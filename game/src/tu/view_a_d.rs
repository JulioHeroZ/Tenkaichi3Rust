#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn ChrView_GetModelFile(c: &mut Ctx, mut p_chara_1: i32, mut p_costume_2: i32, mut p_damaged_3: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ChrView_GetModelFile", &PROBE_N, &[(p_chara_1) as u32, (p_costume_2) as u32, (p_damaged_3) as u32], false); }
    let r_ = 'f: {
    if (((((p_damaged_3) != (0i32)) as i32)) != 0) {
        break 'f (((p_chara_1).wrapping_mul(10i32)).wrapping_add(p_costume_2)).wrapping_add((1424i32).wrapping_add(4i32));
    }
    break 'f (((p_chara_1).wrapping_mul(10i32)).wrapping_add(p_costume_2)).wrapping_add(1424i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "ChrView_GetModelFile", &PROBE_N, &[], true); }
    r_
}

pub fn ChrView_GetCharaInfo(c: &mut Ctx, mut p_chara_1: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ChrView_GetCharaInfo", &PROBE_N, &[(p_chara_1) as u32], false); }
    let mut l_table_2: u32 = 0;
    let r_ = 'f: {
    l_table_2 = ({ let a_ = ((c.m.r32(sym::gCommonRes)).wrapping_add(4)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = ((c.m.r32(sym::gCommonRes)).wrapping_add(4)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32));
    if (((((p_chara_1) >= (((161i32) as u32))) as i32)) != 0) {
        break 'f ((0i32) as u32);
    }
    break 'f (l_table_2).wrapping_add((p_chara_1).wrapping_mul(60));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "ChrView_GetCharaInfo", &PROBE_N, &[], true); }
    r_
}

pub fn ChrView_AllocSlot(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ChrView_AllocSlot", &PROBE_N, &[], false); }
    let mut l_slot_1: u32 = 0;
    let r_ = 'f: {
    l_slot_1 = { let a0_ = (c.m.r32(sym::gChrView)).wrapping_add(176); F::SList_PopFront(c, a0_) };
    if (((((l_slot_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f ((0i32) as u32);
    }
    break 'f l_slot_1;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "ChrView_AllocSlot", &PROBE_N, &[], true); }
    r_
}

pub fn ChrView_FreeSlot(c: &mut Ctx, mut p_slot_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ChrView_FreeSlot", &PROBE_N, &[(p_slot_1) as u32], false); }
    let r_ = 'f: {
    if (((((p_slot_1) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = (c.m.r32(sym::gChrView)).wrapping_add(176); F::SList_PushFront(c, a0_, p_slot_1) };
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "ChrView_FreeSlot", &PROBE_N, &[], true); }
    r_
}

pub fn ChrView_Get(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ChrView_Get", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f c.m.r32(sym::gChrView);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "ChrView_Get", &PROBE_N, &[], true); }
    r_
}

pub fn ChrView_GetCur(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ChrView_GetCur", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f { let a_ = (c.m.r32(sym::gChrView)).wrapping_add(60); c.m.r32(a_) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "ChrView_GetCur", &PROBE_N, &[], true); }
    r_
}

pub fn ChrView_SetCur(c: &mut Ctx, mut p_slot_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ChrView_SetCur", &PROBE_N, &[(p_slot_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_slot_1; let a_ = (c.m.r32(sym::gChrView)).wrapping_add(60); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ChrView_SetCur", &PROBE_N, &[], true); }
    r_
}

pub fn ChrView_IsLoading(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ChrView_IsLoading", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f { let a_ = (c.m.r32(sym::gChrView)).wrapping_add(36); (c.m.r32(a_) as i32) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "ChrView_IsLoading", &PROBE_N, &[], true); }
    r_
}

pub fn ChrView_SetLoading(c: &mut Ctx, mut p_loading_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ChrView_SetLoading", &PROBE_N, &[(p_loading_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_loading_1; let a_ = (c.m.r32(sym::gChrView)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ChrView_SetLoading", &PROBE_N, &[], true); }
    r_
}

pub fn ChrView_SetupCamera(c: &mut Ctx, mut p_chara_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ChrView_SetupCamera", &PROBE_N, &[(p_chara_1) as u32], false); }
    let mut l_info_2: u32 = 0;
    let r_ = 'f: {
    l_info_2 = F::ChrView_GetCharaInfo(c, ((p_chara_1) as u32));
    if (((((l_info_2) != (((0i32) as u32))) as i32)) != 0) {
        F::OrbitCam_Reset(c);
        F::OrbitCam_SetFloorClamp(c, 1i32);
        { let a1_ = F32(c.m.r32((l_info_2).wrapping_add(48))); F::OrbitCam_SetTarget(c, F32(0x00000000), a1_, F32(0x00000000)) };
        F::OrbitCam_SetAngles(c, F32(0x00000000), F32(0x40490fda), F32(0x00000000));
        { let a0_ = F32(c.m.r32((l_info_2).wrapping_add(52))); F::OrbitCam_SetDist(c, a0_) };
        if ((((({ let a_ = (c.m.r32(sym::gChrView)).wrapping_add(56); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
            F::OrbitCam_SetDistRange(c, F32(0x40400000), F32(0x42c80000));
        } else {
            { let a0_ = F32(c.m.r32((l_info_2).wrapping_add(52))); let a1_ = F32(c.m.r32((l_info_2).wrapping_add(52))); F::OrbitCam_SetDistRange(c, a0_, a1_) };
        }
        F::OrbitCam_SetPitchLimit(c, F32(0x3f490fda));
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ChrView_SetupCamera", &PROBE_N, &[], true); }
    r_
}

pub fn ChrView_StepLoad(c: &mut Ctx, mut p_job_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ChrView_StepLoad", &PROBE_N, &[(p_job_1) as u32], false); }
    let mut l_file_2: i32 = 0;
    let mut l_old_3: u32 = 0;
    let r_ = 'f: {
    's1: {
        let sv4: i32 = (c.m.r32((p_job_1).wrapping_add(8)) as i32);
        'k6: {
        'k5: {
        'k4: {
        'k3: {
        'k2: {
            match sv4 { 0i32 => break 'k2, 1i32 => break 'k3, 2i32 => break 'k4, 3i32 => break 'k5, _ => break 'k6, }
        }
        F::ChrView_SetLoading(c, 1i32);
        if ((((({ let a_ = (p_job_1).wrapping_add(12); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); o_ }) >= ({ let a_ = (c.m.r32(sym::gChrView)).wrapping_add(44); (c.m.r32(a_) as i32) })) as i32)) != 0) {
            { let a_ = (p_job_1).wrapping_add(8); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        }
        if ((((({ let a_ = (c.m.r32(sym::gChrView)).wrapping_add(44); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
            break 's1;
        }
        }
        l_file_2 = { let a0_ = (c.m.r32((p_job_1).wrapping_add(16)) as i32); let a1_ = (c.m.r32((p_job_1).wrapping_add(20)) as i32); let a2_ = (c.m.r32((p_job_1).wrapping_add(24)) as i32); F::ChrView_GetModelFile(c, a0_, a1_, a2_) };
        { let v_ = F::ChrView_AllocSlot(c); let a_ = (p_job_1).wrapping_add(28); c.m.w32(a_, v_); };
        if (((((c.m.r32((p_job_1).wrapping_add(28))) == (((0i32) as u32))) as i32)) != 0) {
            break 'f 1i32;
        }
        { let a1_ = (c.m.r32((p_job_1).wrapping_add(28))).wrapping_add(8); F::BtlRes_LoadSingle(c, 0i32, a1_, l_file_2) };
        { let a_ = (p_job_1).wrapping_add(8); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        break 's1;
        }
        if (((((F::File_UpdateRequests(c)) == (0i32)) as i32)) != 0) {
            break 'f 0i32;
        }
        { let v_ = { let a1_ = (c.m.r32((p_job_1).wrapping_add(28))).wrapping_add(8); F::BtlObj_Create(c, 3i32, a1_, 1i32) }; let a_ = (c.m.r32((p_job_1).wrapping_add(28))).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = { let a0_ = { let a_ = (c.m.r32((p_job_1).wrapping_add(28))).wrapping_add(4); (c.m.r32(a_) as i32) }; F::BtlObj_Get(c, a0_) }; let a_ = (p_job_1).wrapping_add(32); c.m.w32(a_, v_); };
        { let a0_ = c.m.r32((p_job_1).wrapping_add(32)); let a1_ = { let a_ = (c.m.r32(sym::gChrView)).wrapping_add(40); (c.m.r32(a_) as i32) }; F::BtlObjAnim_PlayModel(c, a0_, a1_, 2i32) };
        { let r_ = (((!(33554432i32))) as u32); let a_ = (c.m.r32((p_job_1).wrapping_add(32))).wrapping_add(((2624i32) as u32)); let n_ = ((c.m.r32(a_)) & (r_)); c.m.w32(a_, n_); };
        { let a0_ = c.m.r32((p_job_1).wrapping_add(32)); F::BtlObj_SetPartVisible(c, a0_, ((0i32) as u32), 0i32) };
        { let v_ = 0i32; let a_ = (p_job_1).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        { let a_ = (p_job_1).wrapping_add(8); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        }
        if ((((({ let a_ = (p_job_1).wrapping_add(12); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); o_ }) >= (3i32)) as i32)) != 0) {
            { let a0_ = (c.m.r32((p_job_1).wrapping_add(16)) as i32); F::ChrView_SetupCamera(c, a0_) };
            { let a0_ = c.m.r32((p_job_1).wrapping_add(32)); F::BtlObj_SetPartVisible(c, a0_, ((0i32) as u32), 1i32) };
            l_old_3 = F::ChrView_GetCur(c);
            if (((((l_old_3) != (((0i32) as u32))) as i32)) != 0) {
                { let _ = { let a0_ = (c.m.r32((l_old_3).wrapping_add(4)) as i32); F::BtlObj_Destroy(c, a0_) }; };
                { let _ = F::ChrView_FreeSlot(c, l_old_3); };
            }
            { let a0_ = c.m.r32((p_job_1).wrapping_add(28)); F::ChrView_SetCur(c, a0_) };
            { let v_ = 1i32; let a_ = (c.m.r32(sym::gChrView)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
            { let v_ = 1i32; let a_ = (c.m.r32(sym::gChrView)).wrapping_add(52); c.m.w32(a_, (v_) as u32); };
            F::StgAmb_Init(c);
            F::ChrView_SetLoading(c, 0i32);
            break 'f 1i32;
        }
        break 's1;
        }
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "ChrView_StepLoad", &PROBE_N, &[], true); }
    r_
}

pub fn ChrView_StepLoadStage(c: &mut Ctx, mut p_job_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ChrView_StepLoadStage", &PROBE_N, &[(p_job_1) as u32], false); }
    let mut l_res_2: u32 = 0;
    let r_ = 'f: {
    l_res_2 = (c.m.r32(sym::gCommonRes)).wrapping_add(32);
    's1: {
        let sv3: i32 = (c.m.r32((p_job_1).wrapping_add(8)) as i32);
        match sv3 {
            0i32 => {
                { let v_ = { let _ = 0i32; F::File_Request(c, 407i32, ((0i32) as u32)) }; let a_ = (l_res_2).wrapping_add(4); c.m.w32(a_, v_); };
                { let a_ = (p_job_1).wrapping_add(8); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            1i32 => {
                break 'f (((F::File_UpdateRequests(c)) != (0i32)) as i32);
            }
            _ => {
                break 'f 1i32;
            }
        }
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "ChrView_StepLoadStage", &PROBE_N, &[], true); }
    r_
}

pub fn ChrView_CancelLoad(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ChrView_CancelLoad", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if (((((F::ChrView_IsLoading(c)) == (1i32)) as i32)) != 0) {
        if ((((({ let a_ = (c.m.r32(sym::gChrView)).wrapping_add(28); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
            if ((((({ let a_ = (c.m.r32(sym::gChrView)).wrapping_add(8); (c.m.r32(a_) as i32) }) >= (3i32)) as i32)) != 0) {
                { let _ = { let a0_ = { let a_ = ({ let a_ = (c.m.r32(sym::gChrView)).wrapping_add(28); c.m.r32(a_) }).wrapping_add(4); (c.m.r32(a_) as i32) }; F::BtlObj_Destroy(c, a0_) }; };
            }
            { let _ = { let a0_ = { let a_ = (c.m.r32(sym::gChrView)).wrapping_add(28); c.m.r32(a_) }; F::ChrView_FreeSlot(c, a0_) }; };
        }
        F::File_CancelRequests(c);
        F::Job_Clear(c);
        F::ChrView_SetLoading(c, 0i32);
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "ChrView_CancelLoad", &PROBE_N, &[], true); }
    r_
}

pub fn ChrView_Hide(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ChrView_Hide", &PROBE_N, &[], false); }
    let mut l_slot_1: u32 = 0;
    let r_ = 'f: {
    l_slot_1 = F::ChrView_GetCur(c);
    { let v_ = 0i32; let a_ = (c.m.r32(sym::gChrView)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
    F::StgAmb_Term(c);
    { let v_ = 0i32; let a_ = (c.m.r32(sym::gChrView)).wrapping_add(52); c.m.w32(a_, (v_) as u32); };
    if (((((F::ChrView_IsLoading(c)) == (1i32)) as i32)) != 0) {
        { let _ = F::ChrView_CancelLoad(c); };
    }
    if (((((l_slot_1) != (((0i32) as u32))) as i32)) != 0) {
        { let _ = { let a0_ = (c.m.r32((l_slot_1).wrapping_add(4)) as i32); F::BtlObj_Destroy(c, a0_) }; };
        { let _ = F::ChrView_FreeSlot(c, l_slot_1); };
        F::ChrView_SetCur(c, ((0i32) as u32));
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "ChrView_Hide", &PROBE_N, &[], true); }
    r_
}

pub fn ChrView_Show(c: &mut Ctx, mut p_chara_1: i32, mut p_costume_2: i32, mut p_damaged_3: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ChrView_Show", &PROBE_N, &[(p_chara_1) as u32, (p_costume_2) as u32, (p_damaged_3) as u32], false); }
    let mut l_job_4: u32 = 0;
    let r_ = 'f: {
    if (((((F::ChrView_IsLoading(c)) == (1i32)) as i32)) != 0) {
        { let _ = F::ChrView_CancelLoad(c); };
    }
    l_job_4 = F::ChrView_Get(c);
    { let _ = F::memset(c, l_job_4, 0i32, 36u32); };
    { let v_ = fid::ChrView_StepLoad; let a_ = (l_job_4).wrapping_add(4); c.m.w32(a_, v_); };
    { let v_ = p_chara_1; let a_ = (l_job_4).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
    { let v_ = p_costume_2; let a_ = (l_job_4).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
    { let v_ = p_damaged_3; let a_ = (l_job_4).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
    { let _ = F::Job_Push(c, l_job_4); };
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "ChrView_Show", &PROBE_N, &[], true); }
    r_
}

pub fn ChrView_Update(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ChrView_Update", &PROBE_N, &[], false); }
    let mut l_view_1: u32 = 0;
    let r_ = 'f: {
    F::OrbitCam_Update(c, 1i32);
    F::Dbg_ProfMark(c, sym::gBattleProf);
    if ((((({ let a_ = (c.m.r32(sym::gChrView)).wrapping_add(48); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
        { let r_ = (!(1i32)); let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(8); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
    } else {
        { let r_ = 1i32; let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(8); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
    F::BtlObj_UpdateAll(c);
    { let _ = F::Dbg_ProfColor(c, sym::gBattleProf, 2164260863u32); };
    if ((F::ChrView_IsVisible(c)) != 0) {
        F::BtlObj_UpdateVisibility(c, 0i32);
        F::BtlObj_FinishVisibility(c, 0i32);
        F::BtlStage_Update(c);
        F::BtlScene_Update(c);
        l_view_1 = c.m.r32(sym::gBtlCamView);
        F::Dbg_ProfMark(c, sym::gBattleProf);
        F::Gfx_MarkPass(c, 1i32);
        F::StgModel_Cull(c, l_view_1);
        F::StgModel_Draw(c, l_view_1);
        { let _ = F::Dbg_ProfColor(c, sym::gBattleProf, 2164211776u32); };
        F::Dbg_ProfMark(c, sym::gBattleProf);
        F::Gfx_MarkPass(c, 3i32);
        F::StgFx_DrawPreNoCheck(c);
        { let _ = F::Dbg_ProfColor(c, sym::gBattleProf, 2164260863u32); };
        F::Dbg_ProfMark(c, sym::gBattleProf);
        F::Gfx_MarkPass(c, 2i32);
        F::BtlObjDraw_Draw(c);
        { let _ = F::Dbg_ProfColor(c, sym::gBattleProf, 2151743296u32); };
        F::Gfx_MarkPass(c, 0i32);
        F::Dbg_ProfMark(c, sym::gBattleProf);
        F::Gfx_MarkPass(c, 4i32);
        F::Gfx_AddDefaultEnv(c);
        F::BtlScene_Draw(c, 1i32);
        { let _ = F::Ot_Draw(c); };
        { let _ = F::Dbg_ProfColor(c, sym::gBattleProf, 2151694591u32); };
        F::Dbg_ProfMark(c, sym::gBattleProf);
        F::Gfx_MarkPass(c, 0i32);
        { let _ = F::Dbg_ProfColor(c, sym::gBattleProf, 2151694591u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ChrView_Update", &PROBE_N, &[], true); }
    r_
}

pub fn ChrView_IsVisible(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ChrView_IsVisible", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f { let a_ = (c.m.r32(sym::gChrView)).wrapping_add(52); (c.m.r32(a_) as i32) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "ChrView_IsVisible", &PROBE_N, &[], true); }
    r_
}

pub fn ChrView_Init(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ChrView_Init", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let mut l_job_2: u32 = 0;
    let r_ = 'f: {
    { let r_ = (((!(256i32))) as u64); let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); let n_ = ((c.m.r64(a_)) & (r_)); c.m.w64(a_, n_); };
    { let v_ = F::Heap_Alloc(c, ((188u32) as i32), ((32i32) as u32), 0i32, 2i32); let a_ = sym::gChrView; c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32(sym::gChrView); F::memset(c, a0_, 0i32, 188u32) }; };
    { let v_ = 7i32; let a_ = (c.m.r32(sym::gChrView)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (2i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let v_ = F::Heap_Alloc(c, 843776i32, ((64i32) as u32), 0i32, 2i32); let a_ = ((((c.m.r32(sym::gChrView)).wrapping_add(64)).wrapping_add((((l_i_1) as u32)).wrapping_mul(56))).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(16)); c.m.w32(a_, v_); };
                { let v_ = 0i32; let a_ = (((((c.m.r32(sym::gChrView)).wrapping_add(64)).wrapping_add((((l_i_1) as u32)).wrapping_mul(56))).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                { let v_ = (1i32).wrapping_neg(); let a_ = (((((c.m.r32(sym::gChrView)).wrapping_add(64)).wrapping_add((((l_i_1) as u32)).wrapping_mul(56))).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = { let a_ = ((((c.m.r32(sym::gChrView)).wrapping_add(64)).wrapping_add((((l_i_1) as u32)).wrapping_mul(56))).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(16)); c.m.r32(a_) }; c.m.w32(a_, (v_) as u32); };
                { let a0_ = (c.m.r32(sym::gChrView)).wrapping_add(176); let a1_ = ((c.m.r32(sym::gChrView)).wrapping_add(64)).wrapping_add((((l_i_1) as u32)).wrapping_mul(56)); F::SList_PushFront(c, a0_, a1_) };
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
    F::BtlObj_Init(c, 0i32);
    F::BtlObj_SetUnk44FC4(c, F32(0x3f800000));
    F::OrbitCam_Init(c);
    F::Job_Clear(c);
    l_job_2 = F::ChrView_Get(c);
    { let _ = F::memset(c, l_job_2, 0i32, 36u32); };
    { let v_ = fid::ChrView_StepLoadStage; let a_ = (l_job_2).wrapping_add(4); c.m.w32(a_, v_); };
    { let _ = F::Job_Push(c, l_job_2); };
    F::Load_RunBlocking(c);
    F::StgModel_InitStage(c);
    F::StgAmb_Term(c);
    F::StgFx_Init(c);
    F::Ot_Init(c);
    F::BtlScene_Init(c, 8i32);
    F::Fade_Start(c, 0i32, 1i32, F32(0x3f800000));
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ChrView_Init", &PROBE_N, &[], true); }
    r_
}

pub fn ChrView_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ChrView_Term", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let r_ = 'f: {
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (2i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let a0_ = { let a_ = ((((c.m.r32(sym::gChrView)).wrapping_add(64)).wrapping_add((((l_i_1) as u32)).wrapping_mul(56))).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(16)); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
    { let a0_ = c.m.r32(sym::gChrView); F::Heap_Free(c, a0_) };
    F::File_CancelRequests(c);
    F::Job_Clear(c);
    F::BtlObj_Term(c);
    F::OrbitCam_Term(c);
    F::StgFx_Term(c);
    F::BtlStage_Term(c);
    F::BtlScene_Term(c);
    F::Ot_Term(c);
    F::Dma_ResetBuffers(c);
    F::Adx_StopAll(c);
    { let a0_ = { let a_ = ((c.m.r32(sym::gCommonRes)).wrapping_add(32)).wrapping_add(4); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
    F::Fade_ResetAll(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ChrView_Term", &PROBE_N, &[], true); }
    r_
}
