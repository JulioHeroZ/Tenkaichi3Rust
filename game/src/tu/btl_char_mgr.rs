#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn BtlChar_BindObject(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_BindObject", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_obj_2: u32 = 0;
    let mut l_i_3: i32 = 0;
    let r_ = 'f: {
    { let v_ = { let a0_ = (c.m.r32(p_chr_1) as i32); F::BattleSide_GetObjId(c, a0_) }; let a_ = (p_chr_1).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
    l_obj_2 = F::BtlChar_GetObj(c, p_chr_1);
    { let v_ = (1i32).wrapping_neg(); let a_ = (p_chr_1).wrapping_add(2432); c.m.w32(a_, (v_) as u32); };
    {
        l_i_3 = 0i32;
        'l1: loop {
            if !(((((l_i_3) < (3i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if (((((c.m.r32(((l_obj_2).wrapping_add(164)).wrapping_add((((l_i_3) as u32)).wrapping_mul(4)))) != (((0i32) as u32))) as i32)) != 0) {
                    { let v_ = (c.m.r32(((l_obj_2).wrapping_add(164)).wrapping_add((((l_i_3) as u32)).wrapping_mul(4)))).wrapping_add((rt::div_u32({ let a_ = (c.m.r32(((l_obj_2).wrapping_add(164)).wrapping_add((((l_i_3) as u32)).wrapping_mul(4)))).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }, ((4i32) as u32))).wrapping_mul(4)); let a_ = ((p_chr_1).wrapping_add(5608)).wrapping_add((((l_i_3) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                }
            }
            { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
        }
    }
    F::BtlChar_ClearFlag(c, p_chr_1, ((((190i32) as i64 as u64)) as u32));
    F::BtlChar_ClearFlagRange(c, p_chr_1, ((((152i32) as i64 as u64)) as u32), ((((153i32) as i64 as u64)) as u32));
    F::BtlObj_InitChains(c, l_obj_2);
    { let _ = F::memset(c, (p_chr_1).wrapping_add(3584), 0i32, ((64i32) as u32)); };
    F::BtlStat_Reset(c, p_chr_1);
    { let v_ = 0i32; let a_ = (c.m.r32((l_obj_2).wrapping_add(5728))).wrapping_add(((98344i32) as u32)); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_BindObject", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChar_CopyMemberBonus(c: &mut Ctx, mut p_chr_1: u32, mut p_idx_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_CopyMemberBonus", &PROBE_N, &[(p_chr_1) as u32, (p_idx_2) as u32], false); }
    let mut l_dst_3: u32 = 0;
    let mut l_src_4: u32 = 0;
    let r_ = 'f: {
    l_dst_3 = ((p_chr_1).wrapping_add(2468)).wrapping_add((((p_idx_2) as u32)).wrapping_mul(164));
    l_src_4 = { let a0_ = (c.m.r32(p_chr_1) as i32); F::BattleSide_GetMember(c, a0_, p_idx_2) };
    { let v_ = (c.m.r32(((l_src_4).wrapping_add(36)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32); let a_ = ((l_dst_3).wrapping_add(12)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    { let v_ = (c.m.r32(((l_src_4).wrapping_add(36)).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32); let a_ = ((l_dst_3).wrapping_add(12)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    { let v_ = (c.m.r32(((l_src_4).wrapping_add(36)).wrapping_add((((3i32) as u32)).wrapping_mul(4))) as i32); let a_ = ((l_dst_3).wrapping_add(12)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    { let v_ = (c.m.r32(((l_src_4).wrapping_add(36)).wrapping_add((((4i32) as u32)).wrapping_mul(4))) as i32); let a_ = ((l_dst_3).wrapping_add(12)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    { let v_ = (c.m.r32(((l_src_4).wrapping_add(36)).wrapping_add((((5i32) as u32)).wrapping_mul(4))) as i32); let a_ = ((l_dst_3).wrapping_add(12)).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    { let v_ = (c.m.r32(((l_src_4).wrapping_add(36)).wrapping_add((((6i32) as u32)).wrapping_mul(4))) as i32); let a_ = ((l_dst_3).wrapping_add(12)).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    { let v_ = (c.m.r32(((l_src_4).wrapping_add(36)).wrapping_add((((7i32) as u32)).wrapping_mul(4))) as i32); let a_ = ((l_dst_3).wrapping_add(12)).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_CopyMemberBonus", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChar_CopyAllBonus(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_CopyAllBonus", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < ((c.m.r32((p_chr_1).wrapping_add(2456)) as i32))) as i32)) != 0) { break 'l1; }
            'c2: {
                F::BtlChar_CopyMemberBonus(c, p_chr_1, l_i_2);
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_CopyAllBonus", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChar_CopyMemberAbility(c: &mut Ctx, mut p_chr_1: u32, mut p_idx_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_CopyMemberAbility", &PROBE_N, &[(p_chr_1) as u32, (p_idx_2) as u32], false); }
    let mut l_dst_3: u32 = 0;
    let mut l_src_4: u32 = 0;
    let mut l_i_5: i32 = 0;
    let r_ = 'f: {
    l_dst_3 = ((p_chr_1).wrapping_add(2468)).wrapping_add((((p_idx_2) as u32)).wrapping_mul(164));
    l_src_4 = { let a0_ = (c.m.r32(p_chr_1) as i32); F::BattleSide_GetMember(c, a0_, p_idx_2) };
    {
        l_i_5 = 0i32;
        'l1: loop {
            if !(((((l_i_5) < (4i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let v_ = (c.m.r32(((l_src_4).wrapping_add(72)).wrapping_add((((l_i_5) as u32)).wrapping_mul(4))) as i32); let a_ = ((l_dst_3).wrapping_add(40)).wrapping_add((((l_i_5) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            }
            { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_CopyMemberAbility", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChar_CopyAllAbility(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_CopyAllAbility", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < ((c.m.r32((p_chr_1).wrapping_add(2456)) as i32))) as i32)) != 0) { break 'l1; }
            'c2: {
                F::BtlChar_CopyMemberAbility(c, p_chr_1, l_i_2);
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_CopyAllAbility", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChar_Reset(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_Reset", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_side_2: i32 = 0;
    let mut l_i_3: i32 = 0;
    let mut l_obj_4: u32 = 0;
    let mut l_m_5: u32 = 0;
    let mut l_mem_6: u32 = 0;
    let mut l_res_7: u32 = 0;
    let r_ = 'f: {
    l_side_2 = (c.m.r32(p_chr_1) as i32);
    F::BtlChar_SetStage(c, p_chr_1, ((((1i32) as i64 as u64)) as u8));
    { let v_ = F::BattleSide_GetOptionA(c, l_side_2); let a_ = (p_chr_1).wrapping_add(1180); c.m.w32(a_, (v_) as u32); };
    { let v_ = (((F::BattleSide_GetOptionB(c, l_side_2)) == (0i32)) as i32); let a_ = (p_chr_1).wrapping_add(1208); c.m.w32(a_, (v_) as u32); };
    { let v_ = ((((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5640); (c.m.r32(a_) as i32) }) & ((2i32).wrapping_shl(((c.m.r32((p_chr_1).wrapping_add(4)) as i32)) as u32)))) != (0i32)) as i32); let a_ = (p_chr_1).wrapping_add(5584); c.m.w32(a_, (v_) as u32); };
    { let v_ = (1i32).wrapping_neg(); let a_ = (p_chr_1).wrapping_add(5520); c.m.w32(a_, (v_) as u32); };
    { let v_ = (1i32).wrapping_neg(); let a_ = (p_chr_1).wrapping_add(5524); c.m.w32(a_, (v_) as u32); };
    F::BtlInput_Init(c, p_chr_1);
    F::BtlAct_ClearQueue(c, p_chr_1);
    { let v_ = F::BattleSide_IsCpu(c, l_side_2); let a_ = (p_chr_1).wrapping_add(4728); c.m.w32(a_, (v_) as u32); };
    F::BtlChar_PlaceAtStart(c, p_chr_1);
    F::BtlReplay_Rewind(c, p_chr_1);
    F::BtlChar_ResetLook(c, p_chr_1);
    F::BtlChar_SetHeldFlag(c, p_chr_1, ((((5i32) as i64 as u64)) as u32));
    l_obj_4 = F::BtlChar_GetObj(c, p_chr_1);
    F::BtlChar_PoseToObj(c, p_chr_1, 1i32);
    F::BtlAnim_Play(c, p_chr_1, 0i32, F32(0x00000000));
    F::BtlObjAnim_SamplePose(c, l_obj_4);
    F::BtlObjPose_CalcMatrices(c, l_obj_4);
    { let v_ = F::BattleSide_GetMemberCount(c, l_side_2); let a_ = (p_chr_1).wrapping_add(2456); c.m.w32(a_, (v_) as u32); };
    { let v_ = F::BattleSide_GetUnk1FC(c, l_side_2); let a_ = (p_chr_1).wrapping_add(4864); c.m.w32(a_, (v_) as u32); };
    { let v_ = F::BattleSide_GetUnk200(c, l_side_2); let a_ = (p_chr_1).wrapping_add(3316); c.m.w32(a_, (v_) as u32); };
    F::BtlChar_CopyAllBonus(c, p_chr_1);
    F::BtlChar_CopyAllAbility(c, p_chr_1);
    {
        l_i_3 = 0i32;
        'l1: loop {
            if !(((((l_i_3) < ((c.m.r32((p_chr_1).wrapping_add(2456)) as i32))) as i32)) != 0) { break 'l1; }
            'c2: {
                l_mem_6 = ((p_chr_1).wrapping_add(2468)).wrapping_add((((l_i_3) as u32)).wrapping_mul(164));
                { let v_ = 1i32; let a_ = (l_mem_6).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                { let v_ = F::BattleSide_GetMemberChara(c, l_side_2, l_i_3); let a_ = l_mem_6; c.m.w32(a_, (v_) as u32); };
                { let v_ = F::BattleSide_GetMemberCostume(c, l_side_2, l_i_3); let a_ = (l_mem_6).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                { let v_ = { let a_ = (F::BattleSide_GetMember(c, l_side_2, l_i_3)).wrapping_add(12); (c.m.r32(a_) as i32) }; let a_ = (l_mem_6).wrapping_add(56); c.m.w32(a_, (v_) as u32); };
                { let v_ = { let a_ = (F::BattleSide_GetMember(c, l_side_2, l_i_3)).wrapping_add(68); (c.m.r32(a_) as i32) }; let a_ = (l_mem_6).wrapping_add(60); c.m.w32(a_, (v_) as u32); };
                l_m_5 = F::BattleSide_GetMember(c, l_side_2, l_i_3);
                { let a3_ = F32(c.m.r32((l_m_5).wrapping_add(16))); let a4_ = F::BattleSide_GetMemberVariant(c, l_side_2, l_i_3); F::BtlMember_LoadParams(c, p_chr_1, l_i_3, 1i32, a3_, a4_) };
            }
            { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
        }
    }
    { let v_ = 0i32; let a_ = (p_chr_1).wrapping_add(2460); c.m.w32(a_, (v_) as u32); };
    if ((F::BtlMember_HasAbility(c, p_chr_1, 27i32)) != 0) {
        { let v_ = 100000i32; let a_ = (p_chr_1).wrapping_add(2460); c.m.w32(a_, (v_) as u32); };
    }
    l_res_7 = F::BattleResult_GetPtr(c);
    { let v_ = 0i32; let a_ = ((l_res_7).wrapping_add(28)).wrapping_add(((((c.m.r32(p_chr_1) as i32)) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    { let v_ = 0i32; let a_ = ((l_res_7).wrapping_add(36)).wrapping_add(((((c.m.r32(p_chr_1) as i32)) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    if ((((({ let a_ = (F::BtlMember_GetActiveGauge(c, p_chr_1)).wrapping_add(32); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
        { let r_ = ((1073741824i32) as u32); let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2624); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChar_OnModelLoaded(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_OnModelLoaded", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_obj_2: u32 = 0;
    let mut l_m_3: u32 = 0;
    let mut l_g_4: u32 = 0;
    let mut l_other_5: u32 = 0;
    let mut l_og_6: u32 = 0;
    let mut l_ratio_7: F32 = F32(0);
    let mut l_i_8: i32 = 0;
    let r_ = 'f: {
    F::BtlChar_BindObject(c, p_chr_1);
    l_obj_2 = F::BtlChar_GetObj(c, p_chr_1);
    F::BtlChar_SetFlag(c, p_chr_1, ((((299i32) as i64 as u64)) as u32));
    F::BtlAnim_FlushRequest(c, p_chr_1);
    F::BtlObjAnim_SamplePose(c, l_obj_2);
    F::BtlChar_PoseToObj(c, p_chr_1, 1i32);
    F::BtlObjPose_CalcMatrices(c, l_obj_2);
    's1: {
        let sv9: i32 = F::BtlAct_GetCurrent(c, p_chr_1);
        match sv9 {
            236i32 | 237i32 | 238i32 | 239i32 | 240i32 => {
                l_m_3 = F::BtlMember_GetActive(c, p_chr_1);
                { let v_ = (c.m.r32((p_chr_1).wrapping_add(4816)) as i32); let a_ = l_m_3; c.m.w32(a_, (v_) as u32); };
                l_g_4 = (l_m_3).wrapping_add(64);
                { let v_ = (c.m.r32((p_chr_1).wrapping_add(4820)) as i32); let a_ = (l_m_3).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                { let v_ = (c.m.r32((p_chr_1).wrapping_add(4832)) as i32); let a_ = (l_g_4).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
                l_ratio_7 = F::BtlMember_GetHealthRatio(c, p_chr_1);
                { let a1_ = F::BtlMember_GetActiveIndex(c, p_chr_1); F::BtlMember_LoadParams(c, p_chr_1, a1_, 0i32, F32(0x00000000), 0i32) };
                { let v_ = rt::f2i(rt::fadd(rt::fmul(rt::i2f((c.m.r32((l_g_4).wrapping_add(4)) as i32)), l_ratio_7), F32(0x3f000000))); let a_ = l_g_4; c.m.w32(a_, (v_) as u32); };
                if ((((F::BtlParam_GetUnkAD(c, p_chr_1)) & (1i32))) != 0) {
                    { let r_ = 5000i32; let a_ = l_g_4; let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                }
                if ((((F::BtlParam_GetUnkAD(c, p_chr_1)) & (2i32))) != 0) {
                    { let r_ = 10000i32; let a_ = l_g_4; let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                }
                if ((((F::BtlParam_GetUnkAD(c, p_chr_1)) & (4i32))) != 0) {
                    { let v_ = (c.m.r32((l_g_4).wrapping_add(16)) as i32); let a_ = (l_g_4).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                }
                { let v_ = { let a0_ = (c.m.r32(l_g_4) as i32); let a2_ = (c.m.r32((l_g_4).wrapping_add(4)) as i32); F::BtlUtil_Clamp(c, a0_, 1i32, a2_) }; let a_ = l_g_4; c.m.w32(a_, (v_) as u32); };
                { let v_ = { let a0_ = (c.m.r32((l_g_4).wrapping_add(20)) as i32); let a2_ = (c.m.r32((l_g_4).wrapping_add(24)) as i32); F::BtlUtil_Clamp(c, a0_, 0i32, a2_) }; let a_ = (l_g_4).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            243i32 | 244i32 | 245i32 | 246i32 | 247i32 | 248i32 => {
                { let a1_ = (c.m.r32((p_chr_1).wrapping_add(3288)) as i32); F::BtlMember_SetActiveIndex(c, p_chr_1, a1_) };
                { let _ = F::BtlMember_GetSwitchTarget(c, p_chr_1); };
                { let v_ = 0i32; let a_ = (p_chr_1).wrapping_add(2460); c.m.w32(a_, (v_) as u32); };
                l_g_4 = (F::BtlMember_GetActive(c, p_chr_1)).wrapping_add(64);
                if ((F::BtlMember_HasAbility(c, p_chr_1, 67i32)) != 0) {
                    F::BtlChar_SetHeldFlag(c, p_chr_1, ((((6i32) as i64 as u64)) as u32));
                    { let v_ = 30000i32; let a_ = (l_g_4).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                    { let v_ = (c.m.r32((l_g_4).wrapping_add(16)) as i32); let a_ = (l_g_4).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                } else {
                    F::BtlChar_ClearFlag(c, p_chr_1, ((((6i32) as i64 as u64)) as u32));
                    { let v_ = 0i32; let a_ = (l_g_4).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                }
                if ((F::BtlMember_HasAbility(c, p_chr_1, 97i32)) != 0) {
                    { let v_ = (c.m.r32((l_g_4).wrapping_add(24)) as i32); let a_ = (l_g_4).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
                }
                if ((F::BtlChar_TestMemberUnk70(c, p_chr_1)) != 0) {
                    F::BtlObj_BindCommonTables(c, l_obj_2);
                }
                break 's1;
            }
            241i32 | 242i32 => {
                l_m_3 = F::BtlMember_GetActive(c, p_chr_1);
                l_g_4 = (l_m_3).wrapping_add(64);
                l_other_5 = { let a1_ = (c.m.r32((p_chr_1).wrapping_add(4856)) as i32); F::BtlMember_Get(c, p_chr_1, a1_) };
                if (((((l_other_5) != (((0i32) as u32))) as i32)) != 0) {
                    l_og_6 = (l_other_5).wrapping_add(64);
                    { let v_ = { let a0_ = ((c.m.r32(((l_m_3).wrapping_add(12)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)).wrapping_add((c.m.r32(((l_other_5).wrapping_add(12)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)); F::BtlUtil_Clamp(c, a0_, (20i32).wrapping_neg(), 40i32) }; let a_ = ((l_m_3).wrapping_add(12)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    { let v_ = { let a0_ = ((c.m.r32(((l_m_3).wrapping_add(12)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)).wrapping_add((c.m.r32(((l_other_5).wrapping_add(12)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)); F::BtlUtil_Clamp(c, a0_, (20i32).wrapping_neg(), 40i32) }; let a_ = ((l_m_3).wrapping_add(12)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    { let v_ = { let a0_ = ((c.m.r32(((l_m_3).wrapping_add(12)).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32)).wrapping_add((c.m.r32(((l_other_5).wrapping_add(12)).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32)); F::BtlUtil_Clamp(c, a0_, (20i32).wrapping_neg(), 40i32) }; let a_ = ((l_m_3).wrapping_add(12)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    { let v_ = { let a0_ = ((c.m.r32(((l_m_3).wrapping_add(12)).wrapping_add((((3i32) as u32)).wrapping_mul(4))) as i32)).wrapping_add((c.m.r32(((l_other_5).wrapping_add(12)).wrapping_add((((3i32) as u32)).wrapping_mul(4))) as i32)); F::BtlUtil_Clamp(c, a0_, (20i32).wrapping_neg(), 40i32) }; let a_ = ((l_m_3).wrapping_add(12)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    { let v_ = { let a0_ = ((c.m.r32(((l_m_3).wrapping_add(12)).wrapping_add((((4i32) as u32)).wrapping_mul(4))) as i32)).wrapping_add((c.m.r32(((l_other_5).wrapping_add(12)).wrapping_add((((4i32) as u32)).wrapping_mul(4))) as i32)); F::BtlUtil_Clamp(c, a0_, (20i32).wrapping_neg(), 40i32) }; let a_ = ((l_m_3).wrapping_add(12)).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    { let v_ = { let a0_ = ((c.m.r32(((l_m_3).wrapping_add(12)).wrapping_add((((5i32) as u32)).wrapping_mul(4))) as i32)).wrapping_add((c.m.r32(((l_other_5).wrapping_add(12)).wrapping_add((((5i32) as u32)).wrapping_mul(4))) as i32)); F::BtlUtil_Clamp(c, a0_, (20i32).wrapping_neg(), 40i32) }; let a_ = ((l_m_3).wrapping_add(12)).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    { let v_ = { let a0_ = ((c.m.r32(((l_m_3).wrapping_add(12)).wrapping_add((((6i32) as u32)).wrapping_mul(4))) as i32)).wrapping_add((c.m.r32(((l_other_5).wrapping_add(12)).wrapping_add((((6i32) as u32)).wrapping_mul(4))) as i32)); F::BtlUtil_Clamp(c, a0_, (20i32).wrapping_neg(), 40i32) }; let a_ = ((l_m_3).wrapping_add(12)).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    {
                        l_i_8 = 0i32;
                        'l2: loop {
                            if !(((((l_i_8) < (4i32)) as i32)) != 0) { break 'l2; }
                            'c3: {
                                { let r_ = (c.m.r32(((l_other_5).wrapping_add(40)).wrapping_add((((l_i_8) as u32)).wrapping_mul(4))) as i32); let a_ = ((l_m_3).wrapping_add(40)).wrapping_add((((l_i_8) as u32)).wrapping_mul(4)); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                            }
                            { let o_ = l_i_8; let n_ = (o_).wrapping_add(1); l_i_8 = n_; };
                        }
                    }
                    { let r_ = (c.m.r32(l_og_6) as i32); let a_ = l_g_4; let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                    { let r_ = (c.m.r32((l_og_6).wrapping_add(4)) as i32); let a_ = (l_g_4).wrapping_add(4); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                    { let v_ = ((({ let a_ = ({ let a_ = ((c.m.r32(sym::gBtlChars)).wrapping_add(32)).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((((((c.m.r32(l_m_3) as i32)).wrapping_mul(4i32)).wrapping_add(3i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_mul(((100000i32) as u32))) as i32); let a_ = (l_g_4).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                    if ((((F::BtlParam_GetUnkAD(c, p_chr_1)) & (1i32))) != 0) {
                        { let r_ = 5000i32; let a_ = l_g_4; let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                    }
                    if ((((F::BtlParam_GetUnkAD(c, p_chr_1)) & (2i32))) != 0) {
                        { let r_ = 10000i32; let a_ = l_g_4; let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                    }
                    if ((((F::BtlParam_GetUnkAD(c, p_chr_1)) & (4i32))) != 0) {
                        { let v_ = (c.m.r32((l_g_4).wrapping_add(16)) as i32); let a_ = (l_g_4).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                    }
                    if ((((F::BtlParam_GetUnkAD(c, p_chr_1)) & (8i32))) != 0) {
                        F::BtlChar_SetHeldFlag(c, p_chr_1, ((((6i32) as i64 as u64)) as u32));
                        { let v_ = 30000i32; let a_ = (l_g_4).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                        { let v_ = (c.m.r32((l_g_4).wrapping_add(16)) as i32); let a_ = (l_g_4).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                    }
                    { let v_ = { let a0_ = (c.m.r32(l_g_4) as i32); let a2_ = (c.m.r32((l_g_4).wrapping_add(4)) as i32); F::BtlUtil_Clamp(c, a0_, 1i32, a2_) }; let a_ = l_g_4; c.m.w32(a_, (v_) as u32); };
                    { let v_ = { let a0_ = (c.m.r32((l_g_4).wrapping_add(20)) as i32); let a2_ = (c.m.r32((l_g_4).wrapping_add(24)) as i32); F::BtlUtil_Clamp(c, a0_, 0i32, a2_) }; let a_ = (l_g_4).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = (l_other_5).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = l_og_6; c.m.w32(a_, (v_) as u32); };
                }
                { let v_ = 1i32; let a_ = (l_g_4).wrapping_add(52); c.m.w32(a_, (v_) as u32); };
                { let v_ = (c.m.r32((p_chr_1).wrapping_add(4816)) as i32); let a_ = l_m_3; c.m.w32(a_, (v_) as u32); };
                { let v_ = (c.m.r32((p_chr_1).wrapping_add(4820)) as i32); let a_ = (l_m_3).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                { let v_ = (c.m.r32((p_chr_1).wrapping_add(4832)) as i32); let a_ = (l_g_4).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            260i32 | 262i32 | 263i32 | 264i32 | 301i32 | 302i32 | 303i32 | 313i32 | 314i32 | 315i32 => {
                l_m_3 = F::BtlMember_GetActive(c, p_chr_1);
                { let v_ = (c.m.r32((p_chr_1).wrapping_add(4816)) as i32); let a_ = l_m_3; c.m.w32(a_, (v_) as u32); };
                l_g_4 = (l_m_3).wrapping_add(64);
                { let v_ = (c.m.r32((p_chr_1).wrapping_add(4820)) as i32); let a_ = (l_m_3).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                { let v_ = (c.m.r32((p_chr_1).wrapping_add(4832)) as i32); let a_ = (l_g_4).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
                if ((F::BtlChar_TestFlag(c, p_chr_1, ((((166i32) as i64 as u64)) as u32))) != 0) {
                    { let v_ = 1i32; let a_ = (l_g_4).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
                    F::BtlObj_BindCommonTables(c, l_obj_2);
                    { let a1_ = F::BtlMember_GetActiveIndex(c, p_chr_1); F::BtlMember_LoadParams(c, p_chr_1, a1_, 0i32, F32(0x00000000), 0i32) };
                    { let v_ = (c.m.r32((l_g_4).wrapping_add(4)) as i32); let a_ = l_g_4; c.m.w32(a_, (v_) as u32); };
                }
                break 's1;
            }
            _ => {
            }
        }
    }
    { let a1_ = F::BtlParam_GetAuraKind(c, p_chr_1); F::BtlObj_SetColorPreset(c, l_obj_2, a1_, (1i32).wrapping_neg()) };
    if ((((({ let a_ = (F::BtlMember_GetActiveGauge(c, p_chr_1)).wrapping_add(32); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
        { let r_ = ((1073741824i32) as u32); let a_ = (l_obj_2).wrapping_add(2624); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
    } else {
        { let r_ = (((!(1073741824i32))) as u32); let a_ = (l_obj_2).wrapping_add(2624); let n_ = ((c.m.r32(a_)) & (r_)); c.m.w32(a_, n_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_OnModelLoaded", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChar_OnStageLoaded(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_OnStageLoaded", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_obj_2: u32 = 0;
    let r_ = 'f: {
    l_obj_2 = F::BtlChar_GetObj(c, p_chr_1);
    { let _ = F::memset(c, (p_chr_1).wrapping_add(16), 0i32, ((720i32) as u32)); };
    F::BtlChar_PlaceAtStart(c, p_chr_1);
    F::BtlChar_ClearFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32));
    F::BtlChar_SetHeldFlag(c, p_chr_1, ((((15i32) as i64 as u64)) as u32));
    { let v_ = 0i32; let a_ = ((p_chr_1).wrapping_add(5588)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    { let v_ = 0i32; let a_ = ((p_chr_1).wrapping_add(5588)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    { let v_ = 0i32; let a_ = ((p_chr_1).wrapping_add(5588)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    { let v_ = 0i32; let a_ = ((p_chr_1).wrapping_add(5588)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    { let v_ = 0i32; let a_ = ((p_chr_1).wrapping_add(5588)).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    F::BtlChar_ResetLook(c, p_chr_1);
    F::BtlChar_PoseToObj(c, p_chr_1, 1i32);
    if (((((((({ let a0_ = F::BtlAnim_GetId(c, p_chr_1); ((((F::BtlAnim_GetFlags(c, ((((a0_) as i64 as u64)) as u32))) as u64)) as i32) }) & (16i32))) != 0) || ((((((c.m.r32((p_chr_1).wrapping_add(4016)) as i32)) >= (3i32)) as i32)) != 0)) as i32)) != 0) {
        F::BtlChar_SetHeldFlag(c, p_chr_1, ((((314i32) as i64 as u64)) as u32));
    } else {
        F::BtlChar_SetHeldFlag(c, p_chr_1, ((((313i32) as i64 as u64)) as u32));
    }
    { let v_ = 1i32; let a_ = (p_chr_1).wrapping_add(4016); c.m.w32(a_, (v_) as u32); };
    F::BtlAnim_Play(c, p_chr_1, 0i32, F32(0x00000000));
    F::BtlObjAnim_SamplePose(c, l_obj_2);
    F::BtlObjPose_CalcMatrices(c, l_obj_2);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_OnStageLoaded", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChar_ResetRound(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_ResetRound", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_prev_2: i32 = 0;
    let r_ = 'f: {
    if ((((((c.m.r32((p_chr_1).wrapping_add(4912)) as i32)) != (0i32)) as i32)) != 0) {
        F::BtlPartner_Release(c, p_chr_1);
    }
    F::BtlChar_ClearFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32));
    F::BtlChar_SetHeldFlag(c, p_chr_1, ((((15i32) as i64 as u64)) as u32));
    F::BtlChar_SetHeldFlag(c, p_chr_1, ((((5i32) as i64 as u64)) as u32));
    F::BtlChar_ClearFlag(c, p_chr_1, ((((293i32) as i64 as u64)) as u32));
    F::BtlChar_PlaceAtStart(c, p_chr_1);
    { let v_ = (c.m.r32((p_chr_1).wrapping_add(2376)) as i32); let a_ = (p_chr_1).wrapping_add(2384); c.m.w32(a_, (v_) as u32); };
    { let v_ = 249i32; let a_ = (p_chr_1).wrapping_add(2376); c.m.w32(a_, (v_) as u32); };
    { let v_ = (1i32).wrapping_neg(); let a_ = (p_chr_1).wrapping_add(2380); c.m.w32(a_, (v_) as u32); };
    F::BtlAct_ClearQueue(c, p_chr_1);
    { let v_ = 1i32; let a_ = (p_chr_1).wrapping_add(4016); c.m.w32(a_, (v_) as u32); };
    { let v_ = F32(0x3f000000); let a_ = (p_chr_1).wrapping_add(3416); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x43fa0000); let a_ = (p_chr_1).wrapping_add(3420); c.m.w32(a_, (v_).0); };
    { let v_ = 0i32; let a_ = (p_chr_1).wrapping_add(4900); c.m.w32(a_, (v_) as u32); };
    { let v_ = 0i32; let a_ = (p_chr_1).wrapping_add(4904); c.m.w32(a_, (v_) as u32); };
    { let v_ = 0i32; let a_ = (p_chr_1).wrapping_add(4896); c.m.w32(a_, (v_) as u32); };
    F::BtlChar_ResetLook(c, p_chr_1);
    if (((((((((F::Battle_GetMode(c)) == (3i32)) as i32)) != 0) && ((((((c.m.r32(p_chr_1) as i32)) == (0i32)) as i32)) != 0)) as i32)) != 0) {
        F::BtlMember_AddHealth(c, p_chr_1, 10000i32);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_ResetRound", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChar_ResetObjWork(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_ResetObjWork", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    { let a0_ = F::BtlChar_GetObj(c, p_chr_1); F::BtlObj_InitChains(c, a0_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_ResetObjWork", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChars_UpdateFreeze(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChars_UpdateFreeze", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_i_1: i32 = 0;
    let mut l_max_2: i32 = 0;
    let mut l_bytes_3: u32 = 0;
    let mut l_chr_4: u32 = 0;
    let mut l_v_5: i32 = 0;
    let r_ = 'f: {
        l_max_2 = 0i32;
        l_bytes_3 = fp;
        { let _ = F::memset(c, l_bytes_3, 0i32, 8u32); };
        {
            l_i_1 = 0i32;
            'l1: loop {
                if !(((((l_i_1) < (F::BtlChar_GetCount(c))) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_chr_4 = F::BtlChar_Get(c, l_i_1);
                    if ((F::BtlChar_TestFlag(c, l_chr_4, ((((293i32) as i64 as u64)) as u32))) != 0) {
                        { let v_ = 2i32; let a_ = (fp).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    } else {
                        if ((F::BtlChar_TestFlag(c, l_chr_4, ((((294i32) as i64 as u64)) as u32))) != 0) {
                            { let v_ = 1i32; let a_ = (fp).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                        }
                    }
                    if (((((l_max_2) < ((c.m.r32((fp).wrapping_add((((l_i_1) as u32)).wrapping_mul(4))) as i32))) as i32)) != 0) {
                        l_max_2 = (c.m.r32((fp).wrapping_add((((l_i_1) as u32)).wrapping_mul(4))) as i32);
                    }
                }
                { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
            }
        }
        if (((((l_max_2) > (0i32)) as i32)) != 0) {
            {
                l_i_1 = 0i32;
                'l3: loop {
                    if !(((((l_i_1) < (F::BtlChar_GetCount(c))) as i32)) != 0) { break 'l3; }
                    'c4: {
                        l_chr_4 = F::BtlChar_Get(c, l_i_1);
                        { let v_ = 1i32; let a_ = (l_chr_4).wrapping_add(4900); c.m.w32(a_, (v_) as u32); };
                        { let v_ = 0i32; let a_ = (l_chr_4).wrapping_add(4904); c.m.w32(a_, (v_) as u32); };
                    }
                    { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
                }
            }
            {
                l_i_1 = 0i32;
                'l5: loop {
                    if !(((((l_i_1) < (F::BtlChar_GetCount(c))) as i32)) != 0) { break 'l5; }
                    'c6: {
                        l_chr_4 = F::BtlChar_Get(c, l_i_1);
                        if (((((l_max_2) == ((c.m.r32((fp).wrapping_add((((l_i_1) as u32)).wrapping_mul(4))) as i32))) as i32)) != 0) {
                            { let v_ = 0i32; let a_ = (l_chr_4).wrapping_add(4900); c.m.w32(a_, (v_) as u32); };
                            { let v_ = 0i32; let a_ = (l_chr_4).wrapping_add(4904); c.m.w32(a_, (v_) as u32); };
                            if (((((l_max_2) >= (2i32)) as i32)) != 0) {
                                break 'l5;
                            }
                        }
                        if ((F::BtlChar_TestFlag(c, l_chr_4, ((((295i32) as i64 as u64)) as u32))) != 0) {
                            if (((((l_max_2) == (1i32)) as i32)) != 0) {
                                { let v_ = 0i32; let a_ = (l_chr_4).wrapping_add(4900); c.m.w32(a_, (v_) as u32); };
                                { let v_ = 0i32; let a_ = (l_chr_4).wrapping_add(4904); c.m.w32(a_, (v_) as u32); };
                            }
                        }
                    }
                    { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
                }
            }
        }
        {
            l_i_1 = 0i32;
            'l7: loop {
                if !(((((l_i_1) < (F::BtlChar_GetCount(c))) as i32)) != 0) { break 'l7; }
                'c8: {
                    l_chr_4 = F::BtlChar_Get(c, l_i_1);
                    if ((((((c.m.r32((l_chr_4).wrapping_add(4896)) as i32)) > (0i32)) as i32)) != 0) {
                        l_v_5 = ((c.m.r32((l_chr_4).wrapping_add(4896)) as i32)).wrapping_sub(1i32);
                        if (((((l_v_5) < (0i32)) as i32)) != 0) {
                            l_v_5 = 0i32;
                        }
                        { let v_ = l_v_5; let a_ = (l_chr_4).wrapping_add(4896); c.m.w32(a_, (v_) as u32); };
                    }
                    if ((((((c.m.r32((l_chr_4).wrapping_add(4900)) as i32)) != (0i32)) as i32)) != 0) {
                        if ((((((c.m.r32((l_chr_4).wrapping_add(4904)) as i32)) != (0i32)) as i32)) != 0) {
                            { let a_ = (l_chr_4).wrapping_add(4904); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                        } else {
                            { let v_ = (c.m.r32((l_chr_4).wrapping_add(4900)) as i32); let a_ = (l_chr_4).wrapping_add(4896); c.m.w32(a_, (v_) as u32); };
                            { let v_ = 0i32; let a_ = (l_chr_4).wrapping_add(4900); c.m.w32(a_, (v_) as u32); };
                        }
                    }
                }
                { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlChars_UpdateFreeze", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChars_UpdateObjFlag2(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChars_UpdateObjFlag2", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let mut l_found_2: i32 = 0;
    let mut l_chr_3: u32 = 0;
    let r_ = 'f: {
    l_found_2 = (1i32).wrapping_neg();
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (F::BtlChar_GetCount(c))) as i32)) != 0) { break 'l1; }
            'c2: {
                if (({ let a0_ = F::BtlChar_Get(c, l_i_1); F::BtlChar_TestFlag(c, a0_, ((((298i32) as i64 as u64)) as u32)) }) != 0) {
                    l_found_2 = l_i_1;
                    break 'l1;
                }
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
    if (((((l_found_2) < (0i32)) as i32)) != 0) {
        {
            l_i_1 = 0i32;
            'l3: loop {
                if !(((((l_i_1) < (F::BtlChar_GetCount(c))) as i32)) != 0) { break 'l3; }
                'c4: {
                    l_chr_3 = F::BtlChar_Get(c, l_i_1);
                    if ((((((F::BtlChar_TestFlag(c, l_chr_3, ((((48i32) as i64 as u64)) as u32))) != 0) || ((F::BtlChar_TestFlag(c, l_chr_3, ((((11i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) {
                        { let r_ = (((!(2i32))) as u32); let a_ = (F::BtlChar_GetObj(c, l_chr_3)).wrapping_add(2624); let n_ = ((c.m.r32(a_)) & (r_)); c.m.w32(a_, n_); };
                    } else {
                        { let r_ = ((2i32) as u32); let a_ = (F::BtlChar_GetObj(c, l_chr_3)).wrapping_add(2624); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
                    }
                }
                { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
            }
        }
    } else {
        {
            l_i_1 = 0i32;
            'l5: loop {
                if !(((((l_i_1) < (F::BtlChar_GetCount(c))) as i32)) != 0) { break 'l5; }
                'c6: {
                    l_chr_3 = F::BtlChar_Get(c, l_i_1);
                    if (((((l_i_1) == (l_found_2)) as i32)) != 0) {
                        if ((((((F::BtlChar_TestFlag(c, l_chr_3, ((((48i32) as i64 as u64)) as u32))) != 0) || ((F::BtlChar_TestFlag(c, l_chr_3, ((((11i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) {
                            { let r_ = (((!(2i32))) as u32); let a_ = (F::BtlChar_GetObj(c, l_chr_3)).wrapping_add(2624); let n_ = ((c.m.r32(a_)) & (r_)); c.m.w32(a_, n_); };
                        } else {
                            { let r_ = ((2i32) as u32); let a_ = (F::BtlChar_GetObj(c, l_chr_3)).wrapping_add(2624); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
                        }
                    } else {
                        { let r_ = (((!(2i32))) as u32); let a_ = (F::BtlChar_GetObj(c, l_chr_3)).wrapping_add(2624); let n_ = ((c.m.r32(a_)) & (r_)); c.m.w32(a_, n_); };
                    }
                }
                { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChars_UpdateObjFlag2", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChars_ClearObjFlag2(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChars_ClearObjFlag2", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let r_ = 'f: {
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (F::BtlChar_GetCount(c))) as i32)) != 0) { break 'l1; }
            'c2: {
                { let r_ = (((!(2i32))) as u32); let a_ = ({ let a0_ = F::BtlChar_Get(c, l_i_1); F::BtlChar_GetObj(c, a0_) }).wrapping_add(2624); let n_ = ((c.m.r32(a_)) & (r_)); c.m.w32(a_, n_); };
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChars_ClearObjFlag2", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChars_UpdateCollision(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChars_UpdateCollision", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let mut l_nobodyFrozen_2: i32 = 0;
    let r_ = 'f: {
    l_nobodyFrozen_2 = 1i32;
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (F::BtlChar_GetCount(c))) as i32)) != 0) { break 'l1; }
            'c2: {
                if (({ let a0_ = F::BtlChar_Get(c, l_i_1); F::BtlChar_IsFrozen(c, a0_) }) != 0) {
                    l_nobodyFrozen_2 = 0i32;
                    break 'l1;
                }
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
    if ((l_nobodyFrozen_2) != 0) {
        F::BtlBodyHit_Update(c);
        F::BtlColl_Update(c);
    } else {
        {
            l_i_1 = 0i32;
            'l3: loop {
                if !(((((l_i_1) < (F::BtlChar_GetCount(c))) as i32)) != 0) { break 'l3; }
                'c4: {
                    { let a0_ = F::BtlChar_Get(c, l_i_1); F::BtlColl_UpdateFrozen(c, a0_) };
                }
                { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChars_UpdateCollision", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChar_RaiseEvents(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_RaiseEvents", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_g_2: u32 = 0;
    let mut l_lost_3: i32 = 0;
    let mut l_tech_4: i32 = 0;
    let mut l_res_5: u32 = 0;
    let mut l_opp_6: i32 = 0;
    let r_ = 'f: {
    l_g_2 = F::BtlMember_GetActiveGauge(c, p_chr_1);
    l_lost_3 = ((c.m.r32((l_g_2).wrapping_add(4)) as i32)).wrapping_sub((c.m.r32(l_g_2) as i32));
    if (((((l_lost_3) >= (70000i32)) as i32)) != 0) {
        { let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlEvent_Raise(c, a0_, 28i32) };
    } else {
        if (((((l_lost_3) >= (60000i32)) as i32)) != 0) {
            { let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlEvent_Raise(c, a0_, 27i32) };
        } else {
            if (((((l_lost_3) >= (50000i32)) as i32)) != 0) {
                { let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlEvent_Raise(c, a0_, 26i32) };
            } else {
                if (((((l_lost_3) >= (40000i32)) as i32)) != 0) {
                    { let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlEvent_Raise(c, a0_, 25i32) };
                } else {
                    if (((((l_lost_3) >= (30000i32)) as i32)) != 0) {
                        { let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlEvent_Raise(c, a0_, 24i32) };
                    } else {
                        if (((((l_lost_3) >= (20000i32)) as i32)) != 0) {
                            { let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlEvent_Raise(c, a0_, 23i32) };
                        } else {
                            if (((((l_lost_3) >= (10000i32)) as i32)) != 0) {
                                { let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlEvent_Raise(c, a0_, 22i32) };
                            }
                        }
                    }
                }
            }
        }
    }
    if ((((((c.m.r32((l_g_2).wrapping_add(12)) as i32)) == ((c.m.r32((l_g_2).wrapping_add(16)) as i32))) as i32)) != 0) {
        { let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlEvent_Raise(c, a0_, 29i32) };
    }
    if ((((((c.m.r32((l_g_2).wrapping_add(20)) as i32)) == ((c.m.r32((l_g_2).wrapping_add(24)) as i32))) as i32)) != 0) {
        { let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlEvent_Raise(c, a0_, 30i32) };
    }
    if ((F::BtlChar_IsFlagRaised(c, p_chr_1, ((((6i32) as i64 as u64)) as u32))) != 0) {
        { let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlEvent_Raise(c, a0_, 31i32) };
    }
    if (({ let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); F::BtlCharApi_EnteredTechnique(c, a0_) }) != 0) {
        's1: {
            let sv7: i32 = F::BtlAct_GetCurrentClass(c, p_chr_1);
            match sv7 {
                2i32 => {
                    { let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlEvent_Raise(c, a0_, 38i32) };
                    break 's1;
                }
                3i32 => {
                    { let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlEvent_Raise(c, a0_, 39i32) };
                    break 's1;
                }
                4i32 => {
                    { let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlEvent_Raise(c, a0_, 40i32) };
                    break 's1;
                }
                _ => {
                }
            }
        }
    }
    if (({ let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); F::BtlCharApi_IsSkillStart(c, a0_) }) != 0) {
        's2: {
            let sv8: i32 = F::BtlAct_GetCurrentClass(c, p_chr_1);
            match sv8 {
                0i32 => {
                    { let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlEvent_Raise(c, a0_, 36i32) };
                    break 's2;
                }
                1i32 => {
                    { let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlEvent_Raise(c, a0_, 37i32) };
                    break 's2;
                }
                _ => {
                }
            }
        }
    }
    if ((F::BtlChar_TestFlag(c, p_chr_1, ((((8i32) as i64 as u64)) as u32))) != 0) {
        's3: {
            let sv9: i32 = F::BtlAct_GetCurrent(c, p_chr_1);
            match sv9 {
                304i32 | 305i32 | 306i32 => {
                    { let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlEvent_Raise(c, a0_, 20i32) };
                    break 's3;
                }
                137i32 | 138i32 | 139i32 | 140i32 => {
                    { let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlEvent_Raise(c, a0_, 66i32) };
                    break 's3;
                }
                65i32 => {
                    { let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlEvent_Raise(c, a0_, 68i32) };
                    break 's3;
                }
                66i32 => {
                    { let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlEvent_Raise(c, a0_, 69i32) };
                    break 's3;
                }
                25i32 => {
                    if ((((((c.m.r32((p_chr_1).wrapping_add(3432)) as i32)) > (0i32)) as i32)) != 0) {
                        { let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlEvent_Raise(c, a0_, 49i32) };
                    }
                    break 's3;
                }
                250i32 | 252i32 => {
                    { let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlEvent_Raise(c, a0_, 21i32) };
                    break 's3;
                }
                67i32 => {
                    { let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlEvent_Raise(c, a0_, 33i32) };
                    break 's3;
                }
                _ => {
                }
            }
        }
    }
    if ((F::BtlChar_IsFlagRaised(c, p_chr_1, ((((91i32) as i64 as u64)) as u32))) != 0) {
        l_tech_4 = F::BtlAtk_GetId(c, p_chr_1);
        's4: {
            let sv10: i32 = l_tech_4;
            match sv10 {
                0i32 | 1i32 | 2i32 | 3i32 | 4i32 | 5i32 | 6i32 | 7i32 | 8i32 | 9i32 | 10i32 => {
                    { let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlEvent_Raise(c, a0_, 61i32) };
                    break 's4;
                }
                103i32 | 104i32 | 105i32 | 106i32 | 107i32 => {
                    { let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlEvent_Raise(c, a0_, 52i32) };
                    break 's4;
                }
                62i32 | 63i32 | 64i32 | 65i32 | 66i32 => {
                    { let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlEvent_Raise(c, a0_, 62i32) };
                    F::BtlChar_SetFrameBits(c, p_chr_1, ((32i32) as u64));
                    if ((((((c.m.r32((p_chr_1).wrapping_add(3440)) as i32)) > (0i32)) as i32)) != 0) {
                        F::BtlChar_SetFrameBits(c, p_chr_1, ((1048576i32) as u64));
                    }
                    break 's4;
                }
                67i32 | 68i32 | 69i32 | 70i32 => {
                    { let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlEvent_Raise(c, a0_, 50i32) };
                    F::BtlChar_SetFrameBits(c, p_chr_1, ((32i32) as u64));
                    if ((((((c.m.r32((p_chr_1).wrapping_add(3432)) as i32)) > (0i32)) as i32)) != 0) {
                        F::BtlChar_SetFrameBits(c, p_chr_1, ((1048576i32) as u64));
                    }
                    break 's4;
                }
                85i32 => {
                    { let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlEvent_Raise(c, a0_, 51i32) };
                    break 's4;
                }
                35i32 | 37i32 | 39i32 => {
                    { let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlEvent_Raise(c, a0_, 64i32) };
                    break 's4;
                }
                41i32 | 43i32 | 45i32 => {
                    { let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlEvent_Raise(c, a0_, 86i32) };
                    break 's4;
                }
                138i32 => {
                    { let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlEvent_Raise(c, a0_, 63i32) };
                    break 's4;
                }
                83i32 => {
                    { let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlEvent_Raise(c, a0_, 75i32) };
                    F::BtlChar_SetFrameBits(c, p_chr_1, ((524288i32) as u64));
                    break 's4;
                }
                16i32 | 82i32 | 88i32 | 96i32 | 101i32 | 111i32 | 114i32 | 116i32 | 123i32 | 125i32 | 144i32 | 146i32 => {
                    { let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlEvent_Raise(c, a0_, 75i32) };
                    break 's4;
                }
                117i32 | 118i32 | 119i32 | 120i32 => {
                    F::BtlChar_SetFrameBits(c, p_chr_1, ((33554432i32) as u64));
                    break 's4;
                }
                _ => {
                }
            }
        }
        's5: {
            let sv11: i32 = l_tech_4;
            match sv11 {
                29i32 | 62i32 | 103i32 => {
                    { let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlEvent_Raise(c, a0_, 55i32) };
                    break 's5;
                }
                30i32 | 63i32 | 104i32 => {
                    { let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlEvent_Raise(c, a0_, 56i32) };
                    break 's5;
                }
                31i32 | 64i32 | 105i32 => {
                    { let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlEvent_Raise(c, a0_, 57i32) };
                    break 's5;
                }
                32i32 | 65i32 | 106i32 => {
                    { let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlEvent_Raise(c, a0_, 58i32) };
                    break 's5;
                }
                33i32 | 66i32 | 107i32 => {
                    { let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlEvent_Raise(c, a0_, 59i32) };
                    break 's5;
                }
                _ => {
                }
            }
        }
    }
    if ((F::BtlChar_TestFlag(c, p_chr_1, ((((112i32) as i64 as u64)) as u32))) != 0) {
        { let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlEvent_Raise(c, a0_, 67i32) };
    }
    if ((F::BtlChar_IsFlagRaised(c, p_chr_1, ((((190i32) as i64 as u64)) as u32))) != 0) {
        { let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlEvent_Raise(c, a0_, 35i32) };
    }
    if ((F::BtlChar_TestFlag(c, p_chr_1, ((((191i32) as i64 as u64)) as u32))) != 0) {
        { let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlEvent_Raise(c, a0_, 71i32) };
    }
    if ((F::BtlChar_TestFlag(c, p_chr_1, ((((193i32) as i64 as u64)) as u32))) != 0) {
        { let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlEvent_Raise(c, a0_, 70i32) };
        F::BtlChar_SetFrameBits(c, p_chr_1, ((8388608i32) as u64));
    }
    if ((F::BtlChar_TestFlag(c, p_chr_1, ((((108i32) as i64 as u64)) as u32))) != 0) {
        { let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlEvent_Raise(c, a0_, 87i32) };
    }
    if ((F::BtlChar_TestFlag(c, p_chr_1, ((((3i32) as i64 as u64)) as u32))) != 0) {
        l_res_5 = F::BattleResult_GetPtr(c);
        l_opp_6 = F::BtlOpp_GetPlayer(c, p_chr_1);
        if ((((((c.m.r32(((l_res_5).wrapping_add(28)).wrapping_add((((l_opp_6) as u32)).wrapping_mul(4))) as i32)) < ((c.m.r32((p_chr_1).wrapping_add(3392)) as i32))) as i32)) != 0) {
            { let v_ = (c.m.r32((p_chr_1).wrapping_add(3392)) as i32); let a_ = ((l_res_5).wrapping_add(28)).wrapping_add((((l_opp_6) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        }
        if ((((((c.m.r32(((l_res_5).wrapping_add(36)).wrapping_add((((l_opp_6) as u32)).wrapping_mul(4))) as i32)) < ((c.m.r32((p_chr_1).wrapping_add(3396)) as i32))) as i32)) != 0) {
            { let v_ = (c.m.r32((p_chr_1).wrapping_add(3396)) as i32); let a_ = ((l_res_5).wrapping_add(36)).wrapping_add((((l_opp_6) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        }
        { let v_ = rt::fmul(F::BtlMember_GetTeamHealthRatio(c, p_chr_1), F32(0x42c80000)); let a_ = ((l_res_5).wrapping_add(44)).wrapping_add(((((c.m.r32(p_chr_1) as i32)) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        if ((((({ let a_ = F::BtlMember_GetActiveGauge(c, p_chr_1); (c.m.r32(a_) as i32) }) == (1i32)) as i32)) != 0) {
            if ((((({ let a0_ = { let a0_ = F::BtlOpp_GetPlayer(c, p_chr_1); F::BtlChar_Get(c, a0_) }; F::BtlMember_CountAlive(c, a0_) }) <= (0i32)) as i32)) != 0) {
                { let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlEvent_Raise(c, a0_, 84i32) };
            }
        }
    }
    if ((((((F::BtlInput_IsPressed(c, p_chr_1, ((8192i32) as u32))) != 0) && (({ let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlCtrl_CanAct(c, a0_) }) != 0)) as i32)) != 0) {
        { let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlEvent_Raise(c, a0_, 80i32) };
    }
    if ((F::BtlChar_TestFlag(c, p_chr_1, ((((7i32) as i64 as u64)) as u32))) != 0) {
        { let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlEvent_Raise(c, a0_, 73i32) };
    }
    if ((F::BtlChar_IsFlagRaised(c, p_chr_1, ((((132i32) as i64 as u64)) as u32))) != 0) {
        F::BtlChar_SetFrameBits(c, p_chr_1, ((67108864i32) as u64));
    }
    if ((F::BtlChar_IsFlagRaised(c, p_chr_1, ((((164i32) as i64 as u64)) as u32))) != 0) {
        F::BtlChar_SetFrameBits(c, p_chr_1, ((16777216i32) as u64));
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_RaiseEvents", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChars_CheckRoundReset(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChars_CheckRoundReset", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let mut l_any_2: i32 = 0;
    let r_ = 'f: {
    l_any_2 = 0i32;
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (F::BtlChar_GetCount(c))) as i32)) != 0) { break 'l1; }
            'c2: {
                if (({ let a0_ = F::BtlChar_Get(c, l_i_1); F::BtlChar_TestFlag(c, a0_, ((((249i32) as i64 as u64)) as u32)) }) != 0) {
                    l_any_2 = 1i32;
                    break 'l1;
                }
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
    if ((l_any_2) != 0) {
        {
            l_i_1 = 0i32;
            'l3: loop {
                if !(((((l_i_1) < (F::BtlChar_GetCount(c))) as i32)) != 0) { break 'l3; }
                'c4: {
                    { let a0_ = F::BtlChar_Get(c, l_i_1); F::BtlChar_ResetRound(c, a0_) };
                }
                { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
            }
        }
        F::BtlScene_Reset(c, 4i32);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChars_CheckRoundReset", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChar_ApplyBonusRequests(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_ApplyBonusRequests", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < (5i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((F::BtlChar_TestFlag(c, p_chr_1, (((((l_i_2).wrapping_add(259i32)) as i64 as u64)) as u32))) != 0) {
                    F::BtlChar_CopyMemberBonus(c, p_chr_1, l_i_2);
                    F::BtlChar_ClearFlag(c, p_chr_1, (((((l_i_2).wrapping_add(259i32)) as i64 as u64)) as u32));
                }
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_ApplyBonusRequests", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChar_ApplyAbilityRequests(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_ApplyAbilityRequests", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < (5i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((F::BtlChar_TestFlag(c, p_chr_1, (((((l_i_2).wrapping_add(264i32)) as i64 as u64)) as u32))) != 0) {
                    F::BtlChar_CopyMemberAbility(c, p_chr_1, l_i_2);
                    F::BtlChar_ClearFlag(c, p_chr_1, (((((l_i_2).wrapping_add(264i32)) as i64 as u64)) as u32));
                }
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_ApplyAbilityRequests", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChar_SampleInput(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_SampleInput", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    if ((((!((F::BtlChar_IsFrozen(c, p_chr_1)) != 0)) as i32)) != 0) {
        F::BtlChar_SetStage(c, p_chr_1, ((((2i32) as i64 as u64)) as u8));
        F::BtlInput_Sample(c, p_chr_1);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_SampleInput", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChar_BeginFrame(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_BeginFrame", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    if ((F::BtlChar_IsFrozen(c, p_chr_1)) != 0) {
        { let _ = p_chr_1; F::BtlInput_Stub(c) };
        break 'f;
    }
    F::BtlChar_SetStage(c, p_chr_1, ((((3i32) as i64 as u64)) as u8));
    { let s_ = (p_chr_1).wrapping_add(16); let d_ = (p_chr_1).wrapping_add(256); rt::memcpy(c, d_, s_, 240); };
    F::BtlChar_ObjToPose(c, p_chr_1);
    { let v_ = 0i32; let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(160); c.m.w32(a_, (v_) as u32); };
    { let v_ = 0i32; let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(164); c.m.w32(a_, (v_) as u32); };
    F::BtlChar_ClearSnapshots(c, p_chr_1);
    { let v_ = (c.m.r32((p_chr_1).wrapping_add(2376)) as i32); let a_ = (p_chr_1).wrapping_add(2384); c.m.w32(a_, (v_) as u32); };
    { let v_ = (c.m.r32((p_chr_1).wrapping_add(2404)) as i32); let a_ = (p_chr_1).wrapping_add(2408); c.m.w32(a_, (v_) as u32); };
    { let v_ = (c.m.r32((p_chr_1).wrapping_add(2420)) as i32); let a_ = (p_chr_1).wrapping_add(2428); c.m.w32(a_, (v_) as u32); };
    F::BtlAnim_SetObjRate(c, p_chr_1, F32(0x3f800000));
    F::BtlAnim_SetRate(c, p_chr_1, F32(0x3f800000));
    { let v_ = 0i32; let a_ = (p_chr_1).wrapping_add(3404); c.m.w32(a_, (v_) as u32); };
    { let v_ = 0i32; let a_ = (p_chr_1).wrapping_add(3408); c.m.w32(a_, (v_) as u32); };
    F::BtlChar_SetLookEnabled(c, p_chr_1, 0i32);
    F::BtlChar_TickVoiceTimers(c, p_chr_1);
    F::BtlColl_ClearActionBits(c, p_chr_1);
    { let s_ = (p_chr_1).wrapping_add(4706); let d_ = (p_chr_1).wrapping_add(4715); rt::memcpy(c, d_, s_, 9); };
    { let _ = F::memset(c, (p_chr_1).wrapping_add(4706), 0i32, 9u32); };
    if ((F::BtlChar_TestFlag(c, p_chr_1, ((((6i32) as i64 as u64)) as u32))) != 0) {
        { let v_ = F::BtlParam_GetUnk70(c, p_chr_1); let a_ = (p_chr_1).wrapping_add(3436); c.m.w32(a_, (v_) as u32); };
        { let v_ = F::BtlParam_GetUnk72(c, p_chr_1); let a_ = (p_chr_1).wrapping_add(3444); c.m.w32(a_, (v_) as u32); };
    } else {
        { let v_ = 1i32; let a_ = (p_chr_1).wrapping_add(3436); c.m.w32(a_, (v_) as u32); };
        { let v_ = 1i32; let a_ = (p_chr_1).wrapping_add(3444); c.m.w32(a_, (v_) as u32); };
    }
    if ((F::BtlMember_HasAbility(c, p_chr_1, 1i32)) != 0) {
        { let r_ = 2i32; let a_ = (p_chr_1).wrapping_add(3436); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
    } else {
        if ((F::BtlMember_HasAbility(c, p_chr_1, 0i32)) != 0) {
            { let r_ = 1i32; let a_ = (p_chr_1).wrapping_add(3436); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
        }
    }
    if ((F::BtlMember_HasAbility(c, p_chr_1, 3i32)) != 0) {
        { let r_ = 2i32; let a_ = (p_chr_1).wrapping_add(3444); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
    } else {
        if ((F::BtlMember_HasAbility(c, p_chr_1, 2i32)) != 0) {
            { let r_ = 1i32; let a_ = (p_chr_1).wrapping_add(3444); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
        }
    }
    { let v_ = ((0i32) as u64); let a_ = (p_chr_1).wrapping_add(5432); c.m.w64(a_, v_); };
    { let v_ = (1i32).wrapping_neg(); let a_ = (p_chr_1).wrapping_add(5520); c.m.w32(a_, (v_) as u32); };
    { let v_ = (1i32).wrapping_neg(); let a_ = (p_chr_1).wrapping_add(5524); c.m.w32(a_, (v_) as u32); };
    F::BtlInput_Update(c, p_chr_1);
    F::BtlChar_ApplyBonusRequests(c, p_chr_1);
    F::BtlChar_ApplyAbilityRequests(c, p_chr_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_BeginFrame", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChar_UpdateSeqFlags(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_UpdateSeqFlags", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    if ((F::BtlChar_IsFrozen(c, p_chr_1)) != 0) {
        break 'f;
    }
    F::BtlChar_ClearFlag(c, p_chr_1, ((((1i32) as i64 as u64)) as u32));
    F::BtlChar_ClearFlag(c, p_chr_1, ((((2i32) as i64 as u64)) as u32));
    F::BtlChar_ClearFlag(c, p_chr_1, ((((3i32) as i64 as u64)) as u32));
    F::BtlChar_ClearFlag(c, p_chr_1, ((((4i32) as i64 as u64)) as u32));
    's1: {
        let sv2: i32 = F::BtlSeq_GetState(c);
        match sv2 {
            0i32 | 1i32 => {
                F::BtlChar_SetHeldFlag(c, p_chr_1, ((((1i32) as i64 as u64)) as u32));
                break 's1;
            }
            2i32 => {
                F::BtlChar_SetHeldFlag(c, p_chr_1, ((((2i32) as i64 as u64)) as u32));
                break 's1;
            }
            3i32 => {
                F::BtlChar_SetHeldFlag(c, p_chr_1, ((((3i32) as i64 as u64)) as u32));
                break 's1;
            }
            4i32 | 5i32 | 6i32 => {
                F::BtlChar_SetHeldFlag(c, p_chr_1, ((((4i32) as i64 as u64)) as u32));
                break 's1;
            }
            _ => {
            }
        }
    }
    F::BtlChar_UpdateLockOn(c, p_chr_1);
    if ((F::BtlChars_IsTimeStopped(c)) != 0) {
        F::BtlChar_SetHeldFlag(c, p_chr_1, ((((309i32) as i64 as u64)) as u32));
    } else {
        F::BtlChar_ClearFlag(c, p_chr_1, ((((309i32) as i64 as u64)) as u32));
    }
    if ((F::BtlChar_IsFlagDropped(c, p_chr_1, ((((309i32) as i64 as u64)) as u32))) != 0) {
        F::BtlChar_ResetObjWork(c, p_chr_1);
    }
    if ((((!((F::BtlChar_TestFlag(c, p_chr_1, ((((175i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) {
        F::BtlChar_ClearFlag(c, p_chr_1, ((((128i32) as i64 as u64)) as u32));
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_UpdateSeqFlags", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChar_UpdateMotion(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_UpdateMotion", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_obj_2: u32 = 0;
    let r_ = 'f: {
    l_obj_2 = F::BtlChar_GetObj(c, p_chr_1);
    if ((F::BtlChar_IsFrozen(c, p_chr_1)) != 0) {
        break 'f;
    }
    F::BtlMove_UpdateAction(c, p_chr_1);
    if ((F::BtlChar_TestFlag(c, p_chr_1, ((((43i32) as i64 as u64)) as u32))) != 0) {
        F::BtlObj_SaveNodePositions(c, l_obj_2, 1i32);
    } else {
        F::BtlObj_SaveNodePositions(c, l_obj_2, 0i32);
    }
    F::BtlChar_PlaceRestart(c, p_chr_1);
    F::BtlChar_PlaceCenter(c, p_chr_1);
    F::BtlChar_PlaceRelative(c, p_chr_1);
    F::BtlChar_PlaceOnPath(c, p_chr_1);
    F::BtlChar_PlaceCenterHigh(c, p_chr_1);
    { let _ = F::BtlChar_PlaceSaved(c, p_chr_1); };
    F::BtlChar_PlaceWarp(c, p_chr_1);
    F::BtlChar_PoseToObj(c, p_chr_1, 1i32);
    F::BtlObjAnim_SamplePose(c, l_obj_2);
    F::BtlObjAnim_UpdateEvents(c, l_obj_2);
    F::BtlChar_UpdateLean(c, p_chr_1);
    F::BtlFx_UpdateObjEvents(c, p_chr_1);
    F::BtlObjPose_CalcMatrices(c, l_obj_2);
    F::BtlChar_ObjToPose(c, p_chr_1);
    F::BtlChar_UpdateLookOffset(c, p_chr_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_UpdateMotion", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChar_UpdateStage6(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_UpdateStage6", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    if ((((!((F::BtlChar_IsFrozen(c, p_chr_1)) != 0)) as i32)) != 0) {
        F::BtlChar_SetStage(c, p_chr_1, ((((6i32) as i64 as u64)) as u8));
        F::BtlOpp_MirrorFlags(c, p_chr_1);
        F::BtlPartner_Update(c, p_chr_1);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_UpdateStage6", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChar_UpdateStage7(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_UpdateStage7", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_obj_2: u32 = 0;
    let r_ = 'f: {
    l_obj_2 = F::BtlChar_GetObj(c, p_chr_1);
    if ((((!((F::BtlChar_IsFrozen(c, p_chr_1)) != 0)) as i32)) != 0) {
        F::BtlChar_SetStage(c, p_chr_1, ((((7i32) as i64 as u64)) as u8));
        { let _ = F::BtlMove_PushOut(c, p_chr_1); };
        F::BtlMove_ApplyOrbit(c, p_chr_1);
        F::BtlChar_PoseToObj(c, p_chr_1, 0i32);
        F::BtlObjPose_CalcMatrices(c, l_obj_2);
        F::BtlChar_ObjToPose(c, p_chr_1);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_UpdateStage7", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChar_UpdateStage8(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_UpdateStage8", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_obj_2: u32 = 0;
    let mut l_flag_3: i32 = 0;
    let mut l_started_4: i32 = 0;
    let r_ = 'f: {
    l_flag_3 = 1i32;
    l_obj_2 = F::BtlChar_GetObj(c, p_chr_1);
    if ((F::BtlChar_IsFrozen(c, p_chr_1)) != 0) {
        break 'f;
    }
    F::BtlChar_SetStage(c, p_chr_1, ((((8i32) as i64 as u64)) as u8));
    F::BtlMove_ClampToStage(c, p_chr_1);
    F::BtlChar_PoseToObj(c, p_chr_1, 0i32);
    F::BtlObjPose_CalcMatrices(c, l_obj_2);
    if ((F::BtlChar_TestFlag(c, p_chr_1, ((((85i32) as i64 as u64)) as u32))) != 0) {
        F::BtlObjBody_Warp(c, l_obj_2, (p_chr_1).wrapping_add(4880));
        l_flag_3 = 0i32;
    }
    l_started_4 = 0i32;
    if (((({ let a_ = (c.m.r32(sym::gBtlChars)).wrapping_add(16); (c.m.r32(a_) as i32) }) & (1i32))) != 0) {
        l_started_4 = 1i32;
    }
    if ((F::BtlChars_IsTimeStopped(c)) != 0) {
        l_started_4 = 0i32;
    }
    { let a2_ = (if (({ let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); F::BtlCharApi_IsChanging(c, a0_) }) != 0) { 0i32 } else { l_started_4 }); F::StgCol_UpdateFighter(c, l_obj_2, l_flag_3, a2_) };
    F::StgGround_UpdateFighter(c, l_obj_2);
    F::BtlChar_ObjToPose(c, p_chr_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_UpdateStage8", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChar_UpdateStage9(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_UpdateStage9", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_obj_2: u32 = 0;
    let r_ = 'f: {
    l_obj_2 = F::BtlChar_GetObj(c, p_chr_1);
    if ((((!((F::BtlChar_IsFrozen(c, p_chr_1)) != 0)) as i32)) != 0) {
        F::BtlChar_SetStage(c, p_chr_1, ((((9i32) as i64 as u64)) as u8));
        F::BtlColl_UpdateGround(c, p_chr_1);
        F::BtlChar_PoseToObj(c, p_chr_1, 0i32);
        F::BtlObjPose_CalcMatrices(c, l_obj_2);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_UpdateStage9", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChar_UpdateCamera(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_UpdateCamera", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_obj_2: u32 = 0;
    let r_ = 'f: {
    l_obj_2 = F::BtlChar_GetObj(c, p_chr_1);
    if ((((!((F::BtlChar_IsFrozen(c, p_chr_1)) != 0)) as i32)) != 0) {
        F::ChrCam_StartCut(c, p_chr_1);
        F::ChrCam_UpdateDemo(c, p_chr_1);
        F::ChrCam_UpdateInput(c, p_chr_1);
        { let a1_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(48); F::BtlObj_SetMoveVec(c, l_obj_2, a1_) };
        F::BtlObj_UpdateChains(c, l_obj_2);
        { let _ = F::BtlChar_UpdateHead(c, p_chr_1); };
        F::BtlObjPose_CalcMatrices(c, l_obj_2);
        F::BtlFx_UpdateAll(c, p_chr_1);
        F::BtlObjHit_BuildVolumes(c, l_obj_2);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_UpdateCamera", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChar_UpdateStage10(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_UpdateStage10", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    if ((((!((F::BtlChar_IsFrozen(c, p_chr_1)) != 0)) as i32)) != 0) {
        F::BtlChar_SetStage(c, p_chr_1, ((((10i32) as i64 as u64)) as u8));
        F::BtlPartner_UpdateEvents(c, p_chr_1);
        F::BtlFx_UpdateAfterHits(c, p_chr_1);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_UpdateStage10", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChar_PostScene(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_PostScene", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_opp_2: u32 = 0;
    let r_ = 'f: {
    if ((F::BtlChar_IsFrozen(c, p_chr_1)) != 0) {
        break 'f;
    }
    F::BtlChar_ObjToPose(c, p_chr_1);
    F::ChrCam_Update(c, p_chr_1);
    F::BtlFx_UpdatePostScene(c, p_chr_1);
    if ((((({ let a_ = F::BtlMember_GetActiveGauge(c, p_chr_1); (c.m.r32(a_) as i32) }) < (10000i32)) as i32)) != 0) {
        { let v_ = 1i32; let a_ = (F::BtlMember_GetActiveGauge(c, p_chr_1)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
        if ((((((((((((((((!((((F::BtlParam_GetUnk0(c, p_chr_1)) & (128i32))) != 0)) as i32)) != 0) && ((((((c.m.r32((p_chr_1).wrapping_add(3604)) as i32)) <= (0i32)) as i32)) != 0)) as i32)) != 0) && ((((((c.m.r32((p_chr_1).wrapping_add(3608)) as i32)) <= (0i32)) as i32)) != 0)) as i32)) != 0) && ((((!((F::BtlChar_TestFlag(c, p_chr_1, ((((6i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0)) as i32)) != 0) {
            { let v_ = 1i32; let a_ = (F::BtlMember_GetActiveGauge(c, p_chr_1)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
        } else {
            { let v_ = 0i32; let a_ = (F::BtlMember_GetActiveGauge(c, p_chr_1)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
        }
    } else {
        { let v_ = 0i32; let a_ = (F::BtlMember_GetActiveGauge(c, p_chr_1)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (F::BtlMember_GetActiveGauge(c, p_chr_1)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
    }
    l_opp_2 = { let a0_ = F::BtlOpp_GetPlayer(c, p_chr_1); F::BtlChar_Get(c, a0_) };
    if ((F::BtlChar_TestFlag(c, p_chr_1, ((((151i32) as i64 as u64)) as u32))) != 0) {
        if ((((!((F::BtlChar_TestFlag(c, p_chr_1, ((((48i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) {
            F::BtlChar_GetSnapDelta(c, p_chr_1, (l_opp_2).wrapping_add(3888), 1i32, 4i32);
        } else {
            F::Vec4_SetZero(c, (l_opp_2).wrapping_add(3888));
        }
    } else {
        F::Vec4_SetZero(c, (l_opp_2).wrapping_add(3888));
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_PostScene", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChar_UpdateLate(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_UpdateLate", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_obj_2: u32 = 0;
    let r_ = 'f: {
    l_obj_2 = F::BtlChar_GetObj(c, p_chr_1);
    if ((F::BtlChar_IsFrozen(c, p_chr_1)) != 0) {
        break 'f;
    }
    F::BtlFx_UpdateLate(c, p_chr_1);
    F::BtlChar_StopVoiceOnFlag(c, p_chr_1);
    F::BtlCharSnd_PlayTechSounds(c, p_chr_1);
    F::BtlMove_UpdateAnimVoice(c, p_chr_1);
    F::BtlStat_ClearFrameMods(c, p_chr_1);
    if ((F::BtlChar_TestMemberUnk70(c, p_chr_1)) != 0) {
        { let v_ = 1i32; let a_ = (c.m.r32((l_obj_2).wrapping_add(5728))).wrapping_add(((98344i32) as u32)); c.m.w32(a_, (v_) as u32); };
        F::BtlStat_SetFrameMod(c, p_chr_1, 0i32, (10i32).wrapping_neg(), 1i32);
        F::BtlStat_SetFrameMod(c, p_chr_1, 1i32, (10i32).wrapping_neg(), 1i32);
        F::BtlStat_SetFrameMod(c, p_chr_1, 2i32, (10i32).wrapping_neg(), 1i32);
        F::BtlStat_SetFrameMod(c, p_chr_1, 3i32, (10i32).wrapping_neg(), 1i32);
    } else {
        { let v_ = 0i32; let a_ = (c.m.r32((l_obj_2).wrapping_add(5728))).wrapping_add(((98344i32) as u32)); c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_UpdateLate", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChar_EndFrame(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_EndFrame", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_pose_2: u32 = 0;
    let r_ = 'f: {
    l_pose_2 = F::BtlChar_GetPos(c, p_chr_1);
    if ((F::BtlChar_IsFrozen(c, p_chr_1)) != 0) {
        break 'f;
    }
    F::BtlChar_SetStage(c, p_chr_1, ((((11i32) as i64 as u64)) as u8));
    if ((((!((F::BtlChar_TestFlag(c, p_chr_1, ((((36i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) {
        F::BtlChar_GetSnapDelta(c, p_chr_1, (l_pose_2).wrapping_add(48), 0i32, 1i32);
        F::BtlChar_GetMoveSince(c, p_chr_1, (l_pose_2).wrapping_add(64), 0i32);
    } else {
        F::Vec4_SetZero(c, (l_pose_2).wrapping_add(48));
        F::Vec4_SetZero(c, (l_pose_2).wrapping_add(64));
    }
    F::BtlChar_RaiseEvents(c, p_chr_1);
    F::BtlChar_UpdateVibration(c, p_chr_1);
    F::BtlChar_SetStage(c, p_chr_1, ((((12i32) as i64 as u64)) as u8));
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_EndFrame", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChar_AllocAll(c: &mut Ctx, mut p_count_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_AllocAll", &PROBE_N, &[(p_count_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = F::Heap_Alloc(c, ((640u32) as i32), ((32i32) as u32), 0i32, 2i32); let a_ = sym::gBtlChars; c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32(sym::gBtlChars); F::memset(c, a0_, 0i32, 640u32) }; };
    { let v_ = F::Heap_Alloc(c, (p_count_1).wrapping_mul(5632i32), ((32i32) as u32), 0i32, 2i32); let a_ = (c.m.r32(sym::gBtlChars)).wrapping_add(4); c.m.w32(a_, v_); };
    { let _ = { let a0_ = { let a_ = (c.m.r32(sym::gBtlChars)).wrapping_add(4); c.m.r32(a_) }; F::memset(c, a0_, 0i32, (((p_count_1).wrapping_mul(5632i32)) as u32)) }; };
    { let v_ = F::Heap_Alloc(c, (p_count_1).wrapping_mul(52i32), ((32i32) as u32), 0i32, 2i32); let a_ = (c.m.r32(sym::gBtlChars)).wrapping_add(8); c.m.w32(a_, v_); };
    { let _ = { let a0_ = { let a_ = (c.m.r32(sym::gBtlChars)).wrapping_add(8); c.m.r32(a_) }; F::memset(c, a0_, 0i32, (((p_count_1).wrapping_mul(52i32)) as u32)) }; };
    { let v_ = F::Heap_Alloc(c, (p_count_1).wrapping_mul(52i32), ((32i32) as u32), 0i32, 2i32); let a_ = (c.m.r32(sym::gBtlChars)).wrapping_add(12); c.m.w32(a_, v_); };
    { let _ = { let a0_ = { let a_ = (c.m.r32(sym::gBtlChars)).wrapping_add(12); c.m.r32(a_) }; F::memset(c, a0_, 0i32, (((p_count_1).wrapping_mul(52i32)) as u32)) }; };
    { let v_ = p_count_1; let a_ = c.m.r32(sym::gBtlChars); c.m.w32(a_, (v_) as u32); };
    { let v_ = ({ let a_ = ((c.m.r32(sym::gCommonRes)).wrapping_add(4)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add((rt::div_u32({ let a_ = ({ let a_ = ((c.m.r32(sym::gCommonRes)).wrapping_add(4)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }, ((4i32) as u32))).wrapping_mul(4)); let a_ = ((c.m.r32(sym::gBtlChars)).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = ((c.m.r32(sym::gCommonRes)).wrapping_add(4)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add((rt::div_u32({ let a_ = ({ let a_ = ((c.m.r32(sym::gCommonRes)).wrapping_add(4)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }, ((4i32) as u32))).wrapping_mul(4)); let a_ = ((c.m.r32(sym::gBtlChars)).wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = ((c.m.r32(sym::gCommonRes)).wrapping_add(4)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add((rt::div_u32({ let a_ = ({ let a_ = ((c.m.r32(sym::gCommonRes)).wrapping_add(4)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }, ((4i32) as u32))).wrapping_mul(4)); let a_ = ((c.m.r32(sym::gBtlChars)).wrapping_add(32)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = ((c.m.r32(sym::gCommonRes)).wrapping_add(4)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add((rt::div_u32({ let a_ = ({ let a_ = ((c.m.r32(sym::gCommonRes)).wrapping_add(4)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }, ((4i32) as u32))).wrapping_mul(4)); let a_ = ((c.m.r32(sym::gBtlChars)).wrapping_add(32)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = ((c.m.r32(sym::gCommonRes)).wrapping_add(4)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add((rt::div_u32({ let a_ = ({ let a_ = ((c.m.r32(sym::gCommonRes)).wrapping_add(4)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add((((8i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }, ((4i32) as u32))).wrapping_mul(4)); let a_ = ((c.m.r32(sym::gBtlChars)).wrapping_add(32)).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = ({ let a_ = ((c.m.r32(sym::gCommonRes)).wrapping_add(4)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add((rt::div_u32({ let a_ = ({ let a_ = ((c.m.r32(sym::gCommonRes)).wrapping_add(4)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add((((9i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }, ((4i32) as u32))).wrapping_mul(4)); let a_ = ((c.m.r32(sym::gBtlChars)).wrapping_add(32)).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    F::BtlChar_ResetAll(c);
    F::BtlCharSnd_Init(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_AllocAll", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChar_FreeAll(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_FreeAll", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let a0_ = { let a_ = (c.m.r32(sym::gBtlChars)).wrapping_add(12); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
    { let a0_ = { let a_ = (c.m.r32(sym::gBtlChars)).wrapping_add(8); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
    { let a0_ = { let a_ = (c.m.r32(sym::gBtlChars)).wrapping_add(4); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
    { let a0_ = c.m.r32(sym::gBtlChars); F::Heap_Free(c, a0_) };
    { let v_ = ((0i32) as u32); let a_ = sym::gBtlChars; c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_FreeAll", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChar_ResetAll(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_ResetAll", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let mut l_chr_2: u32 = 0;
    let mut l_pad_3: i32 = 0;
    let r_ = 'f: {
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (F::BtlChar_GetCount(c))) as i32)) != 0) { break 'l1; }
            'c2: {
                { let a0_ = F::BtlChar_Get(c, l_i_1); F::BtlPartner_Release(c, a0_) };
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
    { let v_ = 0i32; let a_ = (c.m.r32(sym::gBtlChars)).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
    { let v_ = 0i32; let a_ = (c.m.r32(sym::gBtlChars)).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
    { let v_ = 0i32; let a_ = (c.m.r32(sym::gBtlChars)).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
    { let v_ = 0i32; let a_ = (c.m.r32(sym::gBtlChars)).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
    { let _ = { let a0_ = (c.m.r32(sym::gBtlChars)).wrapping_add(160); F::memset(c, a0_, 0i32, ((144i32) as u32)) }; };
    { let _ = { let a0_ = (c.m.r32(sym::gBtlChars)).wrapping_add(312); F::memset(c, a0_, 0i32, ((320i32) as u32)) }; };
    { let _ = { let a0_ = (c.m.r32(sym::gBtlChars)).wrapping_add(64); F::memset(c, a0_, 0i32, ((96i32) as u32)) }; };
    { let v_ = ((0i32) as u64); let a_ = (c.m.r32(sym::gBtlChars)).wrapping_add(304); c.m.w64(a_, v_); };
    { let _ = { let a0_ = { let a_ = (c.m.r32(sym::gBtlChars)).wrapping_add(4); c.m.r32(a_) }; let a2_ = (((F::BtlChar_GetCount(c)).wrapping_mul(5632i32)) as u32); F::memset(c, a0_, 0i32, a2_) }; };
    F::BtlChange_Reset(c);
    F::BtlReplay_ResetViewer(c);
    {
        l_i_1 = 0i32;
        'l3: loop {
            if !(((((l_i_1) < (F::BtlChar_GetCount(c))) as i32)) != 0) { break 'l3; }
            'c4: {
                l_chr_2 = F::BtlChar_Get(c, l_i_1);
                { let v_ = l_i_1; let a_ = l_chr_2; c.m.w32(a_, (v_) as u32); };
                l_pad_3 = F::BattleSide_GetPad(c, l_i_1);
                { let v_ = l_i_1; let a_ = (l_chr_2).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                { let v_ = l_pad_3; let a_ = (l_chr_2).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                F::BtlChar_BindObject(c, l_chr_2);
                F::BtlChar_Reset(c, l_chr_2);
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_ResetAll", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChars_OnModelLoaded(c: &mut Ctx, mut p_side_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChars_OnModelLoaded", &PROBE_N, &[(p_side_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_Get(c, p_side_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        F::BtlChar_OnModelLoaded(c, l_chr_2);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChars_OnModelLoaded", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChars_OnStageLoaded(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChars_OnStageLoaded", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let r_ = 'f: {
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (F::BtlChar_GetCount(c))) as i32)) != 0) { break 'l1; }
            'c2: {
                { let a0_ = F::BtlChar_Get(c, l_i_1); F::BtlChar_OnStageLoaded(c, a0_) };
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChars_OnStageLoaded", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChars_SampleInput(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChars_SampleInput", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let r_ = 'f: {
    if (((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0) {
        break 'f;
    }
    if (((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((8192i32) as u64)))) != 0) {
        break 'f;
    }
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (F::BtlChar_GetCount(c))) as i32)) != 0) { break 'l1; }
            'c2: {
                { let a0_ = F::BtlChar_Get(c, l_i_1); F::BtlChar_SampleInput(c, a0_) };
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChars_SampleInput", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChars_CheckStart(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChars_CheckStart", &PROBE_N, &[], false); }
    let mut l_ret_1: i32 = 0;
    let r_ = 'f: {
    l_ret_1 = 0i32;
    if (((((((((F::BtlSeq_GetState(c)) == (2i32)) as i32)) != 0) || (((((F::BtlSeq_GetState(c)) == (3i32)) as i32)) != 0)) as i32)) != 0) {
        if ((((!(((({ let a_ = (c.m.r32(sym::gBtlChars)).wrapping_add(16); (c.m.r32(a_) as i32) }) & (1i32))) != 0)) as i32)) != 0) {
            if (((((F::Battle_GetMode(c)) != (1i32)) as i32)) != 0) {
                F::BtlChar_ResetAll(c);
                l_ret_1 = 1i32;
            }
        }
        { let r_ = 1i32; let a_ = (c.m.r32(sym::gBtlChars)).wrapping_add(16); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
    break 'f l_ret_1;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChars_CheckStart", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChars_UpdateInput(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChars_UpdateInput", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let r_ = 'f: {
    if (((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0) {
        break 'f;
    }
    if (((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((8192i32) as u64)))) != 0) {
        F::BtlChars_ClearObjFlag2(c);
        break 'f;
    }
    if (((((((({ let a_ = (c.m.r32(sym::gBtlChars)).wrapping_add(16); (c.m.r32(a_) as i32) }) & (1i32))) != 0) && ((((!((F::BtlChars_IsTimeStopped(c)) != 0)) as i32)) != 0)) as i32)) != 0) {
        { let a_ = (c.m.r32(sym::gBtlChars)).wrapping_add(20); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        { let r_ = 1073741823i32; let a_ = (c.m.r32(sym::gBtlChars)).wrapping_add(20); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
        { let v_ = { let a0_ = ({ let a_ = (c.m.r32(sym::gBtlChars)).wrapping_add(28); (c.m.r32(a_) as i32) }).wrapping_sub(1i32); F::BtlUtil_Max(c, a0_, 0i32) }; let a_ = (c.m.r32(sym::gBtlChars)).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
    }
    F::BtlChars_CheckRoundReset(c);
    F::BtlChars_UpdateFreeze(c);
    F::BtlCharSnd_ClearRequests(c);
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (F::BtlChar_GetCount(c))) as i32)) != 0) { break 'l1; }
            'c2: {
                { let a0_ = F::BtlChar_Get(c, l_i_1); F::BtlChar_BeginFrame(c, a0_) };
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
    F::BtlChars_Snapshot(c, 0i32);
    F::BtlChars_UpdateSightFlag(c);
    {
        l_i_1 = 0i32;
        'l3: loop {
            if !(((((l_i_1) < (F::BtlChar_GetCount(c))) as i32)) != 0) { break 'l3; }
            'c4: {
                { let a0_ = F::BtlChar_Get(c, l_i_1); F::BtlChar_UpdateSeqFlags(c, a0_) };
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChars_UpdateInput", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChars_UpdateMain(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChars_UpdateMain", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let r_ = 'f: {
    if (((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0) {
        break 'f;
    }
    if (((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((8192i32) as u64)))) != 0) {
        break 'f;
    }
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (F::BtlChar_GetCount(c))) as i32)) != 0) { break 'l1; }
            'c2: {
                { let a0_ = F::BtlChar_Get(c, l_i_1); F::BtlChar_UpdateMotion(c, a0_) };
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
    {
        l_i_1 = 0i32;
        'l3: loop {
            if !(((((l_i_1) < (F::BtlChar_GetCount(c))) as i32)) != 0) { break 'l3; }
            'c4: {
                { let a0_ = F::BtlChar_Get(c, l_i_1); F::BtlChar_UpdateStage6(c, a0_) };
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
    F::BtlChars_UpdateHold(c);
    F::BtlChars_Snapshot(c, 1i32);
    {
        l_i_1 = 0i32;
        'l5: loop {
            if !(((((l_i_1) < (F::BtlChar_GetCount(c))) as i32)) != 0) { break 'l5; }
            'c6: {
                { let a0_ = F::BtlChar_Get(c, l_i_1); F::BtlChar_UpdateStage7(c, a0_) };
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
    F::BtlChars_Snapshot(c, 2i32);
    {
        l_i_1 = 0i32;
        'l7: loop {
            if !(((((l_i_1) < (F::BtlChar_GetCount(c))) as i32)) != 0) { break 'l7; }
            'c8: {
                { let a0_ = F::BtlChar_Get(c, l_i_1); F::BtlChar_UpdateStage8(c, a0_) };
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
    F::BtlChars_Snapshot(c, 3i32);
    {
        l_i_1 = 0i32;
        'l9: loop {
            if !(((((l_i_1) < (F::BtlChar_GetCount(c))) as i32)) != 0) { break 'l9; }
            'c10: {
                { let a0_ = F::BtlChar_Get(c, l_i_1); F::BtlChar_UpdateStage9(c, a0_) };
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
    F::BtlChars_Snapshot(c, 4i32);
    {
        l_i_1 = 0i32;
        'l11: loop {
            if !(((((l_i_1) < (F::BtlChar_GetCount(c))) as i32)) != 0) { break 'l11; }
            'c12: {
                { let a0_ = F::BtlChar_Get(c, l_i_1); F::BtlChar_UpdateCamera(c, a0_) };
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
    F::BtlReplay_UpdateViewer(c);
    F::BtlFx_UpdateRoster(c);
    F::BtlMembers_UpdateQueuedDamage(c);
    F::BtlChars_UpdateCollision(c);
    {
        l_i_1 = 0i32;
        'l13: loop {
            if !(((((l_i_1) < (F::BtlChar_GetCount(c))) as i32)) != 0) { break 'l13; }
            'c14: {
                { let a0_ = F::BtlChar_Get(c, l_i_1); F::BtlChar_UpdateStage10(c, a0_) };
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
    F::BtlClash_Update(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChars_UpdateMain", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChars_PostScene(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChars_PostScene", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let r_ = 'f: {
    if (((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0) {
        break 'f;
    }
    if (((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((8192i32) as u64)))) != 0) {
        break 'f;
    }
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (F::BtlChar_GetCount(c))) as i32)) != 0) { break 'l1; }
            'c2: {
                { let a0_ = F::BtlChar_Get(c, l_i_1); F::BtlChar_PostScene(c, a0_) };
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChars_PostScene", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChars_EndFrame(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChars_EndFrame", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let r_ = 'f: {
    if (((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((8192i32) as u64)))) != 0) {
        break 'f;
    }
    F::BtlChange_Update(c);
    if (((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0) {
        break 'f;
    }
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (F::BtlChar_GetCount(c))) as i32)) != 0) { break 'l1; }
            'c2: {
                { let a0_ = F::BtlChar_Get(c, l_i_1); F::BtlChar_UpdateLate(c, a0_) };
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
    F::BtlChars_UpdateObjFlag2(c);
    F::BtlCharSnd_PlayRequests(c);
    {
        l_i_1 = 0i32;
        'l3: loop {
            if !(((((l_i_1) < (F::BtlChar_GetCount(c))) as i32)) != 0) { break 'l3; }
            'c4: {
                { let a0_ = F::BtlChar_Get(c, l_i_1); F::BtlChar_EndFrame(c, a0_) };
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
    F::BtlChars_Snapshot(c, 5i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChars_EndFrame", &PROBE_N, &[], true); }
    r_
}
