#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn BtlChange_Alloc(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChange_Alloc", &PROBE_N, &[], false); }
    let mut l_q_1: u32 = 0;
    let mut l_w_2: i32 = 0;
    let r_ = 'f: {
    l_q_1 = (c.m.r32(sym::gBtlChars)).wrapping_add(312);
    l_w_2 = (c.m.r32((l_q_1).wrapping_add(292)) as i32);
    if (((((rt::rem_i32((l_w_2).wrapping_add(1i32), 8i32)) == ((c.m.r32((l_q_1).wrapping_add(296)) as i32))) as i32)) != 0) {
        break 'f ((0i32) as u32);
    }
    { let v_ = rt::rem_i32((l_w_2).wrapping_add(1i32), 8i32); let a_ = (l_q_1).wrapping_add(292); c.m.w32(a_, (v_) as u32); };
    break 'f (l_q_1).wrapping_add((((l_w_2) as u32)).wrapping_mul(36));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChange_Alloc", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChange_Pop(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChange_Pop", &PROBE_N, &[], false); }
    let mut l_q_1: u32 = 0;
    let mut l_r_2: i32 = 0;
    let r_ = 'f: {
    l_q_1 = (c.m.r32(sym::gBtlChars)).wrapping_add(312);
    l_r_2 = (c.m.r32((l_q_1).wrapping_add(296)) as i32);
    if ((((((c.m.r32((l_q_1).wrapping_add(292)) as i32)) == (l_r_2)) as i32)) != 0) {
        break 'f ((0i32) as u32);
    }
    { let v_ = rt::rem_i32((l_r_2).wrapping_add(1i32), 8i32); let a_ = (l_q_1).wrapping_add(296); c.m.w32(a_, (v_) as u32); };
    break 'f (l_q_1).wrapping_add((((l_r_2) as u32)).wrapping_mul(36));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChange_Pop", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChange_Reset(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChange_Reset", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let _ = { let a0_ = (c.m.r32(sym::gBtlChars)).wrapping_add(312); F::memset(c, a0_, 0i32, 320u32) }; };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChange_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChange_Update(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChange_Update", &PROBE_N, &[], false); }
    let mut l_q_1: u32 = 0;
    let r_ = 'f: {
    l_q_1 = (c.m.r32(sym::gBtlChars)).wrapping_add(312);
    if ((((((c.m.r32((l_q_1).wrapping_add(300)) as i32)) == (5i32)) as i32)) != 0) {
        { let v_ = 0i32; let a_ = (l_q_1).wrapping_add(300); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (l_q_1).wrapping_add(304); c.m.w32(a_, (v_) as u32); };
        { let v_ = ((0i32) as u32); let a_ = (l_q_1).wrapping_add(288); c.m.w32(a_, v_); };
    }
    if (((((c.m.r32((l_q_1).wrapping_add(288))) == (((0i32) as u32))) as i32)) != 0) {
        { let v_ = F::BtlChange_Pop(c); let a_ = (l_q_1).wrapping_add(288); c.m.w32(a_, v_); };
        if (((((c.m.r32((l_q_1).wrapping_add(288))) == (((0i32) as u32))) as i32)) != 0) {
            { let v_ = 0i32; let a_ = (l_q_1).wrapping_add(316); c.m.w32(a_, (v_) as u32); };
        } else {
            { let v_ = 1i32; let a_ = (l_q_1).wrapping_add(300); c.m.w32(a_, (v_) as u32); };
            { let v_ = 0i32; let a_ = (l_q_1).wrapping_add(304); c.m.w32(a_, (v_) as u32); };
            { let v_ = 1i32; let a_ = (l_q_1).wrapping_add(316); c.m.w32(a_, (v_) as u32); };
        }
    } else {
        { let v_ = 1i32; let a_ = (l_q_1).wrapping_add(316); c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChange_Update", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChange_RequestChara(c: &mut Ctx, mut p_player_1: i32, mut p_chara_2: i32, mut p_costume_3: i32, mut p_variant_4: i32, mut p_animChara_5: i32, mut p_unk18_6: i32, mut p_voiceChara_7: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChange_RequestChara", &PROBE_N, &[(p_player_1) as u32, (p_chara_2) as u32, (p_costume_3) as u32, (p_variant_4) as u32, (p_animChara_5) as u32, (p_unk18_6) as u32, (p_voiceChara_7) as u32], false); }
    let mut l_req_8: u32 = 0;
    let r_ = 'f: {
    l_req_8 = F::BtlChange_Alloc(c);
    if (((((l_req_8) != (((0i32) as u32))) as i32)) != 0) {
        { let v_ = p_player_1; let a_ = l_req_8; c.m.w32(a_, (v_) as u32); };
        { let v_ = p_chara_2; let a_ = (l_req_8).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = p_costume_3; let a_ = (l_req_8).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        { let v_ = p_variant_4; let a_ = (l_req_8).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
        { let v_ = p_animChara_5; let a_ = (l_req_8).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
        { let v_ = p_unk18_6; let a_ = (l_req_8).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
        { let v_ = p_voiceChara_7; let a_ = (l_req_8).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (l_req_8).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (l_req_8).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChange_RequestChara", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChange_RequestObject(c: &mut Ctx, mut p_player_1: i32, mut p_id_2: i32, mut p_costume_3: i32, mut p_variant_4: i32, mut p_slot_5: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChange_RequestObject", &PROBE_N, &[(p_player_1) as u32, (p_id_2) as u32, (p_costume_3) as u32, (p_variant_4) as u32, (p_slot_5) as u32], false); }
    let mut l_req_6: u32 = 0;
    let r_ = 'f: {
    l_req_6 = F::BtlChange_Alloc(c);
    if (((((l_req_6) != (((0i32) as u32))) as i32)) != 0) {
        { let v_ = p_player_1; let a_ = l_req_6; c.m.w32(a_, (v_) as u32); };
        { let v_ = 1i32; let a_ = (l_req_6).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = p_id_2; let a_ = (l_req_6).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = p_costume_3; let a_ = (l_req_6).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        { let v_ = p_variant_4; let a_ = (l_req_6).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
        { let v_ = (1i32).wrapping_neg(); let a_ = (l_req_6).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
        { let v_ = (1i32).wrapping_neg(); let a_ = (l_req_6).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
        { let v_ = (1i32).wrapping_neg(); let a_ = (l_req_6).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
        { let v_ = p_slot_5; let a_ = (l_req_6).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChange_RequestObject", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChange_SetTaken(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChange_SetTaken", &PROBE_N, &[], false); }
    let mut l_q_1: u32 = 0;
    let r_ = 'f: {
    l_q_1 = (c.m.r32(sym::gBtlChars)).wrapping_add(312);
    if (((((c.m.r32((l_q_1).wrapping_add(288))) != (((0i32) as u32))) as i32)) != 0) {
        { let v_ = 2i32; let a_ = (l_q_1).wrapping_add(300); c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChange_SetTaken", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChange_SetLoaded(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChange_SetLoaded", &PROBE_N, &[], false); }
    let mut l_q_1: u32 = 0;
    let r_ = 'f: {
    l_q_1 = (c.m.r32(sym::gBtlChars)).wrapping_add(312);
    if (((((c.m.r32((l_q_1).wrapping_add(288))) != (((0i32) as u32))) as i32)) != 0) {
        { let v_ = 3i32; let a_ = (l_q_1).wrapping_add(300); c.m.w32(a_, (v_) as u32); };
        { let v_ = 1i32; let a_ = (l_q_1).wrapping_add(304); c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChange_SetLoaded", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChange_IsLoadedFor(c: &mut Ctx, mut p_player_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChange_IsLoadedFor", &PROBE_N, &[(p_player_1) as u32], false); }
    let mut l_q_2: u32 = 0;
    let r_ = 'f: {
    l_q_2 = (c.m.r32(sym::gBtlChars)).wrapping_add(312);
    if (((((c.m.r32((l_q_2).wrapping_add(288))) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((({ let a_ = c.m.r32((l_q_2).wrapping_add(288)); (c.m.r32(a_) as i32) }) != (p_player_1)) as i32)) != 0) {
        break 'f 0i32;
    }
    break 'f (c.m.r32((l_q_2).wrapping_add(304)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChange_IsLoadedFor", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChange_IsLoaded(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChange_IsLoaded", &PROBE_N, &[], false); }
    let mut l_q_1: u32 = 0;
    let r_ = 'f: {
    l_q_1 = (c.m.r32(sym::gBtlChars)).wrapping_add(312);
    if (((((c.m.r32((l_q_1).wrapping_add(288))) != (((0i32) as u32))) as i32)) != 0) {
        break 'f (c.m.r32((l_q_1).wrapping_add(304)) as i32);
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChange_IsLoaded", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChars_IsTimeStopped(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChars_IsTimeStopped", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f { let a_ = ((c.m.r32(sym::gBtlChars)).wrapping_add(312)).wrapping_add(316); (c.m.r32(a_) as i32) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChars_IsTimeStopped", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChange_GetPlayer(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChange_GetPlayer", &PROBE_N, &[], false); }
    let mut l_cur_1: u32 = 0;
    let r_ = 'f: {
    l_cur_1 = { let a_ = ((c.m.r32(sym::gBtlChars)).wrapping_add(312)).wrapping_add(288); c.m.r32(a_) };
    if (((((l_cur_1) != (((0i32) as u32))) as i32)) != 0) {
        break 'f (c.m.r32(l_cur_1) as i32);
    }
    break 'f (1i32).wrapping_neg();
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChange_GetPlayer", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChange_SetReady(c: &mut Ctx, mut p_player_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChange_SetReady", &PROBE_N, &[(p_player_1) as u32], false); }
    let mut l_q_2: u32 = 0;
    let r_ = 'f: {
    l_q_2 = (c.m.r32(sym::gBtlChars)).wrapping_add(312);
    if (((((((((c.m.r32((l_q_2).wrapping_add(288))) != (((0i32) as u32))) as i32)) != 0) && ((((({ let a_ = c.m.r32((l_q_2).wrapping_add(288)); (c.m.r32(a_) as i32) }) == (p_player_1)) as i32)) != 0)) as i32)) != 0) {
        { let v_ = 4i32; let a_ = (l_q_2).wrapping_add(300); c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChange_SetReady", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChange_SetDone(c: &mut Ctx, mut p_player_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChange_SetDone", &PROBE_N, &[(p_player_1) as u32], false); }
    let mut l_q_2: u32 = 0;
    let r_ = 'f: {
    l_q_2 = (c.m.r32(sym::gBtlChars)).wrapping_add(312);
    if (((((((((c.m.r32((l_q_2).wrapping_add(288))) != (((0i32) as u32))) as i32)) != 0) && ((((({ let a_ = c.m.r32((l_q_2).wrapping_add(288)); (c.m.r32(a_) as i32) }) == (p_player_1)) as i32)) != 0)) as i32)) != 0) {
        { let v_ = 5i32; let a_ = (l_q_2).wrapping_add(300); c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChange_SetDone", &PROBE_N, &[], true); }
    r_
}
