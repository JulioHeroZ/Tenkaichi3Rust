#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn Battle_Restart(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Battle_Restart", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::Snd_StopBankAndResume(c, 60i32);
    F::Battle_ResetWork(c);
    F::BtlScene_Reset(c, 0i32);
    F::StgFx_Reset(c);
    F::BtlGame_Reset(c);
    F::BtlStage_Reset(c);
    F::BtlChar_ResetAll(c);
    F::Fade_ResetAll(c);
    F::Fade_Start(c, 0i32, 1i32, F32(0x3f800000));
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Battle_Restart", &PROBE_N, &[], true); }
    r_
}

pub fn Battle_Init(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Battle_Init", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let _ = F::Battle_Load(c); };
    F::StgFx_Init(c);
    F::StgModel_InitStage(c);
    F::BtlChar_AllocAll(c, 2i32);
    F::BtlAiMgr_Init(c);
    F::BtlCam_Init(c);
    { let _ = F::StgNav_Init(c); };
    F::BtlGame_Init(c);
    F::Ot_Init(c);
    F::BtlScene_Init(c, 0i32);
    { let _ = F::Battle_Restart(c); };
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Battle_Init", &PROBE_N, &[], true); }
    r_
}

pub fn Battle_Term(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Battle_Term", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::BattleResult_Finish(c);
    F::Adx_StopAll(c);
    F::BtlChar_FreeAll(c);
    F::BtlCam_Term(c);
    F::BtlScene_Term(c);
    F::Ot_Term(c);
    F::StgFx_Term(c);
    F::BtlStage_Term(c);
    F::BtlAiMgr_Term(c);
    F::StgNav_Term(c);
    F::BtlGame_Term(c);
    { let _ = F::Battle_Unload(c); };
    F::Fade_ResetAll(c);
    F::Dma_ResetBuffers(c);
    F::Job_Clear(c);
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Battle_Term", &PROBE_N, &[], true); }
    r_
}

pub fn Battle_Update(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Battle_Update", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let mut l_singleView_2: i32 = 0;
    let r_ = 'f: {
    F::BtlChars_UpdateInput(c);
    F::BtlChars_UpdateMain(c);
    F::BtlScene_Update(c);
    F::EftDet_Update(c);
    F::BtlScene_PostUpdate(c);
    F::BtlChars_PostScene(c);
    F::BtlScene_CheckStageChange(c);
    F::BtlLoad_PollCharaRequest(c);
    F::BtlLoad_PollObjectRequest(c);
    F::BtlStage_Update(c);
    F::BtlCam_SelectView(c, 0i32);
    F::BtlCam_UpdateView(c, 0i32);
    F::BtlCam_SelectView(c, 1i32);
    F::BtlCam_UpdateView(c, 1i32);
    l_singleView_2 = F::BtlCam_UpdateOverride(c);
    F::BtlChars_EndFrame(c);
    F::BtlObj_UpdateAll(c);
    if ((((((F::Battle_IsSplitScreen(c)) != 0) && ((((!((l_singleView_2) != 0)) as i32)) != 0)) as i32)) != 0) {
        {
            l_i_1 = 0i32;
            'l1: loop {
                if !(((((l_i_1) < (2i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    F::BtlCam_SelectView(c, l_i_1);
                    F::BtlCam_ApplyView(c, 0i32);
                    F::BtlObj_UpdateVisibility(c, l_i_1);
                }
                { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
            }
        }
        F::BtlObj_FinishVisibility(c, 1i32);
    } else {
        F::BtlObj_UpdateVisibility(c, 0i32);
        F::BtlObj_FinishVisibility(c, 0i32);
    }
    F::BtlScene_SetSingleView(c, l_singleView_2);
    break 'f (((l_singleView_2) == (0i32)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Battle_Update", &PROBE_N, &[], true); }
    r_
}

pub fn Battle_Draw(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Battle_Draw", &PROBE_N, &[], false); }
    let mut l_prof_1: u32 = 0;
    let mut l_view_2: i32 = 0;
    let r_ = 'f: {
    l_prof_1 = sym::gBattleProf;
    l_view_2 = (c.m.r32(sym::gBtlCamView) as i32);
    F::Dbg_ProfMark(c, l_prof_1);
    F::Gfx_MarkPass(c, 1i32);
    F::StgModel_Cull(c, ((((l_view_2) as i64 as u64)) as u32));
    F::StgModel_Draw(c, ((((l_view_2) as i64 as u64)) as u32));
    { let _ = F::Dbg_ProfColor(c, l_prof_1, 2164211776u32); };
    F::Dbg_ProfMark(c, l_prof_1);
    F::Gfx_MarkPass(c, 3i32);
    F::StgFx_DrawPre(c);
    { let _ = F::Dbg_ProfColor(c, l_prof_1, 2164260863u32); };
    F::Dbg_ProfMark(c, l_prof_1);
    F::Gfx_MarkPass(c, 2i32);
    F::BtlObjDraw_Draw(c);
    { let _ = F::Dbg_ProfColor(c, l_prof_1, 2151743296u32); };
    F::Dbg_ProfMark(c, l_prof_1);
    F::Gfx_MarkPass(c, 3i32);
    F::StgFx_DrawNop(c);
    { let _ = F::Dbg_ProfColor(c, l_prof_1, 2164260863u32); };
    F::Dbg_ProfMark(c, l_prof_1);
    F::Gfx_MarkPass(c, 4i32);
    F::Gfx_AddDefaultEnv(c);
    F::BtlScene_Draw(c, 1i32);
    { let _ = F::Ot_Draw(c); };
    { let _ = F::Dbg_ProfColor(c, l_prof_1, 2151694591u32); };
    F::Dbg_ProfMark(c, l_prof_1);
    F::Gfx_MarkPass(c, 3i32);
    F::StgFx_DrawPost(c);
    { let _ = F::Dbg_ProfColor(c, l_prof_1, 2164260863u32); };
    F::Dbg_ProfMark(c, l_prof_1);
    F::Gfx_MarkPass(c, 0i32);
    F::BtlGame_Draw(c);
    { let _ = F::Dbg_ProfColor(c, l_prof_1, 2164211967u32); };
    F::Dbg_ProfMark(c, l_prof_1);
    F::Gfx_MarkPass(c, 0i32);
    F::Font_FlushAll(c);
    F::FontIcon_Stub23D1E0(c);
    { let _ = F::Dbg_ProfColor(c, l_prof_1, 2164211967u32); };
    F::Dbg_ProfMark(c, l_prof_1);
    F::Gfx_MarkPass(c, 3i32);
    F::StgFx_DrawOverlay(c);
    { let _ = F::Dbg_ProfColor(c, l_prof_1, 2164260863u32); };
    F::Gfx_MarkPass(c, 0i32);
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Battle_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn Battle_DrawSplit(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Battle_DrawSplit", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let r_ = 'f: {
    F::Dbg_ProfMark(c, sym::gBattleProf);
    F::Gfx_MarkPass(c, 1i32);
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (2i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                F::BtlCam_SelectView(c, l_i_1);
                F::BtlCam_ApplyView(c, 1i32);
                { let a0_ = (c.m.r32(sym::gBtlCamView) as i32); F::StgModel_Cull(c, ((((a0_) as i64 as u64)) as u32)) };
                { let a0_ = (c.m.r32(sym::gBtlCamView) as i32); F::StgModel_Draw(c, ((((a0_) as i64 as u64)) as u32)) };
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
    { let _ = F::Dbg_ProfColor(c, sym::gBattleProf, 2164211776u32); };
    F::Dbg_ProfMark(c, sym::gBattleProf);
    F::Gfx_MarkPass(c, 3i32);
    F::StgFx_DrawPre(c);
    { let _ = F::Dbg_ProfColor(c, sym::gBattleProf, 2164260863u32); };
    F::Dbg_ProfMark(c, sym::gBattleProf);
    F::Gfx_MarkPass(c, 2i32);
    F::BtlObjDraw_Draw(c);
    { let _ = F::Dbg_ProfColor(c, sym::gBattleProf, 2151743296u32); };
    F::Dbg_ProfMark(c, sym::gBattleProf);
    F::Gfx_MarkPass(c, 3i32);
    F::StgFx_DrawNop(c);
    { let _ = F::Dbg_ProfColor(c, sym::gBattleProf, 2164260863u32); };
    F::Dbg_ProfMark(c, sym::gBattleProf);
    F::Gfx_MarkPass(c, 4i32);
    F::Gfx_AddDefaultEnv(c);
    {
        l_i_1 = 0i32;
        'l3: loop {
            if !(((((l_i_1) < (2i32)) as i32)) != 0) { break 'l3; }
            'c4: {
                F::BtlCam_SelectView(c, l_i_1);
                F::BtlCam_ApplyView(c, 1i32);
                F::BtlScene_Draw(c, (((l_i_1) == (0i32)) as i32));
                { let _ = F::Ot_Draw(c); };
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
    { let _ = F::Dbg_ProfColor(c, sym::gBattleProf, 2151694591u32); };
    F::Dbg_ProfMark(c, sym::gBattleProf);
    F::Gfx_MarkPass(c, 3i32);
    F::StgFx_DrawPost(c);
    { let _ = F::Dbg_ProfColor(c, sym::gBattleProf, 2164260863u32); };
    F::Dbg_ProfMark(c, sym::gBattleProf);
    F::Gfx_MarkPass(c, 0i32);
    F::BtlGame_Draw(c);
    { let _ = F::Dbg_ProfColor(c, sym::gBattleProf, 2164211967u32); };
    F::Dbg_ProfMark(c, sym::gBattleProf);
    F::Gfx_MarkPass(c, 0i32);
    F::Font_FlushAll(c);
    F::FontIcon_Stub23D1E0(c);
    { let _ = F::Dbg_ProfColor(c, sym::gBattleProf, 2164211967u32); };
    F::Dbg_ProfMark(c, sym::gBattleProf);
    F::Gfx_MarkPass(c, 3i32);
    F::StgFx_DrawOverlay(c);
    { let _ = F::Dbg_ProfColor(c, sym::gBattleProf, 2164260863u32); };
    F::Gfx_MarkPass(c, 0i32);
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Battle_DrawSplit", &PROBE_N, &[], true); }
    r_
}

pub fn Battle_Loop(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Battle_Loop", &PROBE_N, &[], false); }
    let mut l_draw_1: i32 = 0;
    let mut l_done_2: i32 = 0;
    let r_ = 'f: {
    'l1: loop {
        'c2: {
            if (((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((32768i32) as u64)))) != 0) {
                { let _ = F::Battle_Restart(c); };
                { let r_ = (((!(32768i32))) as u64); let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); let n_ = ((c.m.r64(a_)) & (r_)); c.m.w64(a_, n_); };
            }
            { let _ = F::Job_Run(c); };
            F::Gfx_BeginFrame(c);
            if ((((!(((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0)) as i32)) != 0) {
                F::Gsc_Update(c);
                F::BtlScript_Update(c);
            }
            { let _ = F::BtlChars_CheckStart(c); };
            F::Pad_Update(c);
            F::Snd_Update(c);
            F::Snd_SendFighters(c);
            F::BtlGame_PreUpdate(c);
            F::Battle_UpdateWork(c);
            F::BtlAiMgr_Update(c);
            F::BtlChars_SampleInput(c);
            l_draw_1 = F::Battle_Update(c);
            l_done_2 = F::BtlGame_Update(c);
            if ((((((F::Battle_IsSplitScreen(c)) != 0) && ((l_draw_1) != 0)) as i32)) != 0) {
                { let _ = F::Battle_DrawSplit(c); };
            } else {
                { let _ = F::Battle_Draw(c); };
            }
            F::Gfx_EndFrame(c, 2i32);
            F::Dma_Flush(c);
        }
        if !((((!((l_done_2) != 0)) as i32)) != 0) { break 'l1; }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Battle_Loop", &PROBE_N, &[], true); }
    r_
}

pub fn Battle_Main(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Battle_Main", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let v_ = 1i32; let a_ = (F::Battle_GetWork(c)).wrapping_add(6648); c.m.w32(a_, (v_) as u32); };
    { let _ = F::Battle_Init(c); };
    F::Battle_Loop(c);
    { let _ = F::Battle_Term(c); };
    { let v_ = 0i32; let a_ = (F::Battle_GetWork(c)).wrapping_add(6648); c.m.w32(a_, (v_) as u32); };
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Battle_Main", &PROBE_N, &[], true); }
    r_
}
