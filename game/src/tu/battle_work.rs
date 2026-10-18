#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn Battle_GetWork(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Battle_GetWork", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f sym::gBattleWork;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Battle_GetWork", &PROBE_N, &[], true); }
    r_
}

pub fn Battle_ClearWork(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Battle_ClearWork", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let _ = { let a0_ = F::Battle_GetWork(c); F::memset(c, a0_, 0i32, 6656u32) }; };
    F::BattleSetup_Clear(c);
    F::BattleResult_Clear(c);
    F::BtlEvent_ClearAll(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Battle_ClearWork", &PROBE_N, &[], true); }
    r_
}

pub fn Battle_ResetWork(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Battle_ResetWork", &PROBE_N, &[], false); }
    let mut l_ev_1: u32 = 0;
    let r_ = 'f: {
    F::Adx_StopAll(c);
    { let v_ = ((0i32) as u64); let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.w64(a_, v_); };
    F::BtlLoad_Reload(c);
    F::BattleReplay_ClearDataFlag(c);
    F::BattleResult_Clear(c);
    F::BtlEvent_Reset(c);
    { let a0_ = (if (((((((((F::Battle_GetBgm(c)) >= (26i32)) as i32)) != 0) && (((((F::Battle_GetBgm(c)) < ((26i32).wrapping_add(40i32))) as i32)) != 0)) as i32)) != 0) { ((3400i32).wrapping_add(73000i32)).wrapping_add((F::Battle_GetBgm(c)).wrapping_sub(26i32)) } else { (68374i32).wrapping_add(F::Battle_GetBgm(c)) }); F::Bgm_Play(c, a0_) };
    if (((((F::Battle_GetMode(c)) == (1i32)) as i32)) != 0) {
        l_ev_1 = F::Battle_GetEventWork(c);
        F::BtlScript_Restart(c);
        { let v_ = { let a0_ = (c.m.r32(l_ev_1) as i32); F::Gsc_StartMain(c, a0_) }; let a_ = (l_ev_1).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Battle_ResetWork", &PROBE_N, &[], true); }
    r_
}

pub fn Battle_UpdateWork(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Battle_UpdateWork", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if (((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0) {
        break 'f;
    }
    F::BattleResult_CountFrame(c);
    F::BtlEvent_Update(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Battle_UpdateWork", &PROBE_N, &[], true); }
    r_
}

pub fn Battle_GetSetup(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Battle_GetSetup", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f F::Battle_GetWork(c);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Battle_GetSetup", &PROBE_N, &[], true); }
    r_
}

pub fn Battle_GetResult(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Battle_GetResult", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f (F::Battle_GetWork(c)).wrapping_add(6456);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Battle_GetResult", &PROBE_N, &[], true); }
    r_
}

pub fn Battle_GetEventWork(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Battle_GetEventWork", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f (F::Battle_GetWork(c)).wrapping_add(6528);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Battle_GetEventWork", &PROBE_N, &[], true); }
    r_
}

pub fn Battle_GetWork5A8(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Battle_GetWork5A8", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f (F::Battle_GetWork(c)).wrapping_add(1448);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Battle_GetWork5A8", &PROBE_N, &[], true); }
    r_
}

pub fn Battle_Load(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Battle_Load", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::BtlObj_Init(c, 1i32);
    F::Gsc_InitDefault(c, sym::gBtlScriptCommands);
    F::BtlScript_Init(c);
    F::BtlLoad_PushInitialJob(c);
    F::Load_RunBlocking(c);
    { let a1_ = { let a0_ = F::BattleSide_GetModelSlot(c, 0i32); F::BtlObj_CreateChara(c, a0_) }; F::BattleSide_SetObjId(c, 0i32, a1_) };
    { let a1_ = { let a0_ = F::BattleSide_GetModelSlot(c, 1i32); F::BtlObj_CreateChara(c, a0_) }; F::BattleSide_SetObjId(c, 1i32, a1_) };
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Battle_Load", &PROBE_N, &[], true); }
    r_
}

pub fn Battle_Unload(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Battle_Unload", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::BtlObj_Term(c);
    F::BtlLoad_FreeAll(c);
    F::BtlScript_Free(c);
    F::Gsc_Exit(c);
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Battle_Unload", &PROBE_N, &[], true); }
    r_
}
