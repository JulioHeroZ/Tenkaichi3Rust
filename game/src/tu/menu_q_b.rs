#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn SimDay_GetTrainRank(c: &mut Ctx, mut p_kind_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SimDay_GetTrainRank", &PROBE_N, &[(p_kind_1) as u32], false); }
    let mut l_turn_2: i32 = 0;
    let mut l_i_3: i32 = 0;
    let mut l_last_4: i32 = 0;
    let r_ = 'f: {
    l_turn_2 = { let a_ = ((c.m.r32(sym::gProgress)).wrapping_add(1612)).wrapping_add(40); (c.m.r32(a_) as i32) };
    {
        l_i_3 = 0i32;
        'l1: loop {
            if !(((((l_i_3) < (4i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                's3: {
                    let sv5: i32 = p_kind_1;
                    match sv5 {
                        0i32 => {
                            l_last_4 = ((c.m.r32(((sym::gSimTrain0).wrapping_add(((((l_i_3).wrapping_add(1i32)) as u32)).wrapping_mul(24))).wrapping_add(20)) as i32)).wrapping_sub(1i32);
                            if (((((l_turn_2) < (l_last_4)) as i32)) != 0) {
                                break 'f l_i_3;
                            }
                            break 's3;
                        }
                        1i32 => {
                            l_last_4 = ((c.m.r32(((sym::gSimTrain1).wrapping_add(((((l_i_3).wrapping_add(1i32)) as u32)).wrapping_mul(24))).wrapping_add(20)) as i32)).wrapping_sub(1i32);
                            if (((((l_turn_2) < (l_last_4)) as i32)) != 0) {
                                break 'f l_i_3;
                            }
                            break 's3;
                        }
                        2i32 => {
                            l_last_4 = ((c.m.r32(((sym::gSimTrain2).wrapping_add(((((l_i_3).wrapping_add(1i32)) as u32)).wrapping_mul(24))).wrapping_add(20)) as i32)).wrapping_sub(1i32);
                            if (((((l_turn_2) < (l_last_4)) as i32)) != 0) {
                                break 'f l_i_3;
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
    break 'f 4i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "SimDay_GetTrainRank", &PROBE_N, &[], true); }
    r_
}

pub fn SimDay_SetFaceA(c: &mut Ctx, mut p_n_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SimDay_SetFaceA", &PROBE_N, &[(p_n_1) as u32], false); }
    let r_ = 'f: {
    if (((((p_n_1) == (12i32)) as i32)) != 0) {
        { let v_ = { let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(936); c.m.r32(a_) }; let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(344)).wrapping_add((((32i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    } else {
        { let v_ = ({ let a_ = ({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(928); c.m.r32(a_) }).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((p_n_1).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(344)).wrapping_add((((32i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "SimDay_SetFaceA", &PROBE_N, &[], true); }
    r_
}

pub fn SimDay_SetFaceB(c: &mut Ctx, mut p_n_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SimDay_SetFaceB", &PROBE_N, &[(p_n_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = ({ let a_ = ({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(932); c.m.r32(a_) }).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((p_n_1).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(344)).wrapping_add((((33i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "SimDay_SetFaceB", &PROBE_N, &[], true); }
    r_
}

pub fn SimDay_SetupBattle(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SimDay_SetupBattle", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_round_1: i32 = 0;
    let mut l_rd_2: u32 = 0;
    let mut l_announcer_3: i32 = 0;
    let mut l_unk4_4: i32 = 0;
    let mut l_timeLimit_5: i32 = 0;
    let mut l_stage_6: i32 = 0;
    let mut l_bgm_7: i32 = 0;
    let mut l_unk14_8: i32 = 0;
    let mut l_en_9: u32 = 0;
    let mut l_i_10: i32 = 0;
    let mut l_n_11: i32 = 0;
    let mut l_color_12: i32 = 0;
    let mut l_pool_13: i32 = 0;
    let mut l_slot_14: i32 = 0;
    let mut l_id_15: i32 = 0;
    let r_ = 'f: {
        l_round_1 = rt::div_i32({ let a_ = ((c.m.r32(sym::gProgress)).wrapping_add(1612)).wrapping_add(40); (c.m.r32(a_) as i32) }, 10i32);
        l_rd_2 = ({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(2892); c.m.r32(a_) }).wrapping_add((((l_round_1) as u32)).wrapping_mul(28));
        l_announcer_3 = (c.m.r32(l_rd_2) as i32);
        l_unk4_4 = ((((c.m.r32((l_rd_2).wrapping_add(4)) as i32)) != (0i32)) as i32);
        l_timeLimit_5 = (c.m.r32((l_rd_2).wrapping_add(8)) as i32);
        l_stage_6 = (c.m.r32((l_rd_2).wrapping_add(12)) as i32);
        l_bgm_7 = (c.m.r32((l_rd_2).wrapping_add(16)) as i32);
        l_unk14_8 = ((((c.m.r32((l_rd_2).wrapping_add(20)) as i32)) != (0i32)) as i32);
        l_en_9 = ({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(2900); c.m.r32(a_) }).wrapping_add(((((c.m.r32((l_rd_2).wrapping_add(24)) as i32)) as u32)).wrapping_mul(44));
        if (((((l_announcer_3) == (998i32)) as i32)) != 0) {
            l_announcer_3 = ((F::Rand_Range(c, ((8i32) as u32))) as i32);
        }
        if (((((l_stage_6) == (998i32)) as i32)) != 0) {
            l_stage_6 = ((F::Rand_Range(c, ((35i32) as u32))) as i32);
        }
        if (((((l_bgm_7) == (998i32)) as i32)) != 0) {
            l_bgm_7 = 24i32;
        }
        F::Battle_ClearWork(c);
        F::BattleSetup_SetRule(c, 0i32, 2i32, l_bgm_7, l_timeLimit_5, l_announcer_3, l_stage_6, l_unk4_4);
        if ((((((c.m.r32(l_en_9) as i32)) == (998i32)) as i32)) != 0) {
            l_n_11 = 0i32;
            l_pool_13 = 0i32;
            's1: {
                let sv16: i32 = l_round_1;
                match sv16 {
                    0i32 => {
                        break 's1;
                    }
                    3i32 => {
                        l_pool_13 = 1i32;
                        break 's1;
                    }
                    5i32 => {
                        l_pool_13 = 2i32;
                        break 's1;
                    }
                    _ => {
                    }
                }
            }
            {
                l_i_10 = 0i32;
                'l2: loop {
                    if !(((((l_i_10) < (16i32)) as i32)) != 0) { break 'l2; }
                    'c3: {
                        if ((((({ let a_ = (({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(2896); c.m.r32(a_) }).wrapping_add((((l_pool_13) as u32)).wrapping_mul(64))).wrapping_add((((l_i_10) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) == (999i32)) as i32)) != 0) {
                            break 'l2;
                        }
                        { let o_ = l_n_11; let n_ = (o_).wrapping_add(1); l_n_11 = n_; };
                    }
                    { let o_ = l_i_10; let n_ = (o_).wrapping_add(1); l_i_10 = n_; };
                }
            }
            { let v_ = { let a_ = (({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(2896); c.m.r32(a_) }).wrapping_add((((l_pool_13) as u32)).wrapping_mul(64))).wrapping_add((F::Rand_Range(c, ((l_n_11) as u32))).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(2904); c.m.w32(a_, (v_) as u32); };
        } else {
            { let v_ = (c.m.r32(l_en_9) as i32); let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(2904); c.m.w32(a_, (v_) as u32); };
        }
        { let _ = F::memset(c, fp, 0i32, 16u32); };
        { let v_ = ((({ let a_ = (((c.m.r32(sym::gProgress)).wrapping_add(1612)).wrapping_add(4)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(196i32)) as u16); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(2)); c.m.w16(a_, v_); };
        { let v_ = ((({ let a_ = (((c.m.r32(sym::gProgress)).wrapping_add(1612)).wrapping_add(4)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(272i32)) as u16); let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(2)); c.m.w16(a_, v_); };
        {
            l_i_10 = 0i32;
            'l4: loop {
                if !(((((l_i_10) < (3i32)) as i32)) != 0) { break 'l4; }
                'c5: {
                    l_slot_14 = rt::rem_i32(({ let a_ = ((c.m.r32(sym::gProgress)).wrapping_add(1612)).wrapping_add(36); (c.m.r32(a_) as i32) }).wrapping_add(l_i_10), 3i32);
                    if ((((({ let a_ = (((c.m.r32(sym::gProgress)).wrapping_add(1612)).wrapping_add(24)).wrapping_add((((l_slot_14) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) != ((1i32).wrapping_neg())) as i32)) != 0) {
                        { let v_ = ((({ let a_ = (((c.m.r32(sym::gProgress)).wrapping_add(1612)).wrapping_add(24)).wrapping_add((((l_slot_14) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(1i32)) as u16); let a_ = (fp).wrapping_add(((((2i32).wrapping_add(l_i_10)) as u32)).wrapping_mul(2)); c.m.w16(a_, v_); };
                    }
                }
                { let o_ = l_i_10; let n_ = (o_).wrapping_add(1); l_i_10 = n_; };
            }
        }
        { let _ = F::memset(c, fp.wrapping_add(16), 0i32, 16u32); };
        l_n_11 = 0i32;
        {
            l_i_10 = 0i32;
            'l6: loop {
                if !(((((l_i_10) < (7i32)) as i32)) != 0) { break 'l6; }
                'c7: {
                    l_id_15 = (c.m.r32(((l_en_9).wrapping_add(16)).wrapping_add((((l_i_10) as u32)).wrapping_mul(4))) as i32);
                    if (((((l_id_15) != (999i32)) as i32)) != 0) {
                        { let v_ = (((l_id_15).wrapping_add(1i32)) as u16); let a_ = (fp.wrapping_add(16)).wrapping_add(((({ let o_ = l_n_11; let n_ = (o_).wrapping_add(1); l_n_11 = n_; o_ }) as u32)).wrapping_mul(2)); c.m.w16(a_, v_); };
                    }
                }
                { let o_ = l_i_10; let n_ = (o_).wrapping_add(1); l_i_10 = n_; };
            }
        }
        if ((((((c.m.r32((l_en_9).wrapping_add(12)) as i32)) == (999i32)) as i32)) != 0) {
            { let v_ = ((0i32) as u16); let a_ = (fp.wrapping_add(16)).wrapping_add((((7i32) as u32)).wrapping_mul(2)); c.m.w16(a_, v_); };
        } else {
            { let v_ = ((((c.m.r32((l_en_9).wrapping_add(12)) as i32)).wrapping_add(1i32)) as u16); let a_ = (fp.wrapping_add(16)).wrapping_add((((7i32) as u32)).wrapping_mul(2)); c.m.w16(a_, v_); };
        }
        F::BattleSetup_SetSide(c, 0i32, 0i32, 0i32, 1i32, 1i32, 1i32, 0i32, ((((0i32) as i64 as u64)) as u32));
        F::BattleSetup_SetSide(c, 1i32, 2i32, 1i32, 1i32, l_unk14_8, 1i32, 0i32, ((((0i32) as i64 as u64)) as u32));
        { let a2_ = { let a_ = ((c.m.r32(sym::gProgress)).wrapping_add(1088)).wrapping_add(28); (c.m.r32(a_) as i32) }; let a3_ = { let a_ = ((c.m.r32(sym::gProgress)).wrapping_add(1088)).wrapping_add(24); (c.m.r32(a_) as i32) }; let a6_ = rt::i2f({ let a_ = (((c.m.r32(sym::gProgress)).wrapping_add(1612)).wrapping_add(4)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }); F::BattleSetup_SetMember(c, 0i32, 0i32, a2_, a3_, 0i32, 0i32, a6_, fp) };
        l_color_12 = (c.m.r32((l_en_9).wrapping_add(4)) as i32);
        if ((((({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(2904); (c.m.r32(a_) as i32) }) == ({ let a_ = ((c.m.r32(sym::gProgress)).wrapping_add(1088)).wrapping_add(28); (c.m.r32(a_) as i32) })) as i32)) != 0) {
            if ((((({ let a_ = ((c.m.r32(sym::gProgress)).wrapping_add(1088)).wrapping_add(24); (c.m.r32(a_) as i32) }) == (l_color_12)) as i32)) != 0) {
                l_color_12 = (((l_color_12) == (0i32)) as i32);
            }
        }
        { let a2_ = { let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(2904); (c.m.r32(a_) as i32) }; let a5_ = (c.m.r32((l_en_9).wrapping_add(8)) as i32); F::BattleSetup_SetMember(c, 1i32, 0i32, a2_, l_color_12, 0i32, a5_, F32(0x42c80000), fp.wrapping_add(16)) };
        F::BattleSetup_FinishEx(c, 1i32);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "SimDay_SetupBattle", &PROBE_N, &[], true); }
    r_
}

pub fn SimDay_IsEventItem(c: &mut Ctx, mut p_item_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SimDay_IsEventItem", &PROBE_N, &[(p_item_1) as u32], false); }
    let mut l_i_2: u32 = 0;
    let r_ = 'f: {
    {
        l_i_2 = ((0i32) as u32);
        'l1: loop {
            if !(((((l_i_2) < (((42i32) as u32))) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((((((c.m.r32((sym::gSimEventItems).wrapping_add((l_i_2).wrapping_mul(4))) as i32)) == (p_item_1)) as i32)) != 0) {
                    break 'f 1i32;
                }
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "SimDay_IsEventItem", &PROBE_N, &[], true); }
    r_
}

pub fn SimDay_ListItems(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SimDay_ListItems", &PROBE_N, &[], false); }
    let mut l_any_1: i32 = 0;
    let mut l_i_2: i32 = 0;
    let mut l_all_3: i32 = 0;
    let r_ = 'f: {
    l_any_1 = 0i32;
    { let v_ = 0i32; let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(2880); c.m.w32(a_, (v_) as u32); };
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < (350i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if (((((((((((((({ let a_ = (((c.m.r32(sym::gSaveData)).wrapping_add(8)).wrapping_add(11968)).wrapping_add(((l_i_2) as u32)); c.m.r8(a_) }) as i32)) & (1i32))) as u8)) as i32)) != 0) && ((F::SimDay_IsEventItem(c, l_i_2)) != 0)) as i32)) != 0) {
                    l_any_1 = 1i32;
                    { let v_ = l_i_2; let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(1480)).wrapping_add(((({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(2880); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    { let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(2880); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                }
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
    l_i_2 = { let a_ = ((c.m.r32(sym::gSaveData)).wrapping_add(8)).wrapping_add(516); (c.m.r32(a_) as i32) };
    l_all_3 = (((l_i_2) == (20i32)) as i32);
    if ((((!((l_all_3) != 0)) as i32)) != 0) {
        if ((((!((l_any_1) != 0)) as i32)) != 0) {
            { let v_ = 0i32; let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(2884); c.m.w32(a_, (v_) as u32); };
        } else {
            { let v_ = 1i32; let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(2884); c.m.w32(a_, (v_) as u32); };
        }
    } else {
        if ((((!((l_any_1) != 0)) as i32)) != 0) {
            { let v_ = 2i32; let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(2884); c.m.w32(a_, (v_) as u32); };
        } else {
            { let v_ = 3i32; let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(2884); c.m.w32(a_, (v_) as u32); };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "SimDay_ListItems", &PROBE_N, &[], true); }
    r_
}

pub fn SimDay_AddChange(c: &mut Ctx, mut p_stat_1: i32, mut p_amount_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SimDay_AddChange", &PROBE_N, &[(p_stat_1) as u32, (p_amount_2) as u32], false); }
    let r_ = 'f: {
    { let r_ = (1i32).wrapping_shl((p_stat_1) as u32); let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(2960); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    if (((((p_stat_1) == (3i32)) as i32)) != 0) {
        { let v_ = rt::div_i32(p_amount_2, 100i32); let a_ = (((c.m.r32(sym::gSimDay)).wrapping_add(2960)).wrapping_add(8)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    } else {
        { let v_ = p_amount_2; let a_ = (((c.m.r32(sym::gSimDay)).wrapping_add(2960)).wrapping_add(8)).wrapping_add((((p_stat_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "SimDay_AddChange", &PROBE_N, &[], true); }
    r_
}

pub fn SimDay_GetStat(c: &mut Ctx, mut p_stat_1: i32, mut p_preview_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SimDay_GetStat", &PROBE_N, &[(p_stat_1) as u32, (p_preview_2) as u32], false); }
    let r_ = 'f: {
    if ((p_preview_2) != 0) {
        break 'f { let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(1460)).wrapping_add((((p_stat_1) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
    }
    if (((((p_stat_1) == (3i32)) as i32)) != 0) {
        break 'f ({ let a_ = (((c.m.r32(sym::gProgress)).wrapping_add(1612)).wrapping_add(4)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_mul(100i32);
    }
    break 'f { let a_ = (((c.m.r32(sym::gProgress)).wrapping_add(1612)).wrapping_add(4)).wrapping_add((((p_stat_1) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "SimDay_GetStat", &PROBE_N, &[], true); }
    r_
}

pub fn SimDay_GiveItem(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SimDay_GiveItem", &PROBE_N, &[], false); }
    let mut l_n_1: i32 = 0;
    let r_ = 'f: {
    l_n_1 = (({ let a0_ = (({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(2880); (c.m.r32(a_) as i32) }) as u32); F::Rand_Range(c, a0_) }) as i32);
    { let v_ = { let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(1480)).wrapping_add((((l_n_1) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (((c.m.r32(sym::gProgress)).wrapping_add(1612)).wrapping_add(24)).wrapping_add(((({ let a_ = ((c.m.r32(sym::gProgress)).wrapping_add(1612)).wrapping_add(36); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    { let v_ = rt::rem_i32(({ let a_ = ((c.m.r32(sym::gProgress)).wrapping_add(1612)).wrapping_add(36); (c.m.r32(a_) as i32) }).wrapping_add(1i32), 3i32); let a_ = ((c.m.r32(sym::gProgress)).wrapping_add(1612)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
    F::SimDay_Cmd(c, 2i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "SimDay_GiveItem", &PROBE_N, &[], true); }
    r_
}

pub fn SimDay_TakeItem(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SimDay_TakeItem", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let mut l_slot_2: i32 = 0;
    let r_ = 'f: {
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (3i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                l_slot_2 = rt::rem_i32(({ let a_ = ((c.m.r32(sym::gProgress)).wrapping_add(1612)).wrapping_add(36); (c.m.r32(a_) as i32) }).wrapping_add(l_i_1), 3i32);
                if ((((({ let a_ = (((c.m.r32(sym::gProgress)).wrapping_add(1612)).wrapping_add(24)).wrapping_add((((l_slot_2) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) != ((1i32).wrapping_neg())) as i32)) != 0) {
                    { let v_ = (1i32).wrapping_neg(); let a_ = (((c.m.r32(sym::gProgress)).wrapping_add(1612)).wrapping_add(24)).wrapping_add((((l_slot_2) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    F::SimDay_Cmd(c, 3i32);
                    break 'f;
                }
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "SimDay_TakeItem", &PROBE_N, &[], true); }
    r_
}

pub fn SimDay_CountItems(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SimDay_CountItems", &PROBE_N, &[], false); }
    let mut l_n_1: i32 = 0;
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
    l_n_1 = 0i32;
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < (3i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((((({ let a_ = (((c.m.r32(sym::gProgress)).wrapping_add(1612)).wrapping_add(24)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) != ((1i32).wrapping_neg())) as i32)) != 0) {
                    { let o_ = l_n_1; let n_ = (o_).wrapping_add(1); l_n_1 = n_; };
                }
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
    break 'f l_n_1;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "SimDay_CountItems", &PROBE_N, &[], true); }
    r_
}

pub fn SimDay_CountItems2(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SimDay_CountItems2", &PROBE_N, &[], false); }
    let mut l_n_1: i32 = 0;
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
    l_n_1 = 0i32;
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < (3i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((((({ let a_ = (((c.m.r32(sym::gProgress)).wrapping_add(1612)).wrapping_add(24)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) != ((1i32).wrapping_neg())) as i32)) != 0) {
                    { let o_ = l_n_1; let n_ = (o_).wrapping_add(1); l_n_1 = n_; };
                }
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
    break 'f l_n_1;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "SimDay_CountItems2", &PROBE_N, &[], true); }
    r_
}

pub fn SimDay_SetButton(c: &mut Ctx, mut p_on_1: i32, mut p_button_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SimDay_SetButton", &PROBE_N, &[(p_on_1) as u32, (p_button_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(80) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 80);
    let mut l_flash_3: u32 = 0;
    let r_ = 'f: {
        l_flash_3 = ((c.m.r32(sym::gSimDay)).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(44));
        { let _ = F::sprintf(c, fp.wrapping_add(8), sym::menu_q_b__str0, &[((p_button_2) as i64 as u64)]); };
        F::Flash_FindLabel(c, l_flash_3, ((0i32) as u32), fp.wrapping_add(8), fp);
        if ((p_on_1) != 0) {
            if ((((({ let a_ = ((c.m.r32(sym::gProgress)).wrapping_add(1612)).wrapping_add(52); (c.m.r32(a_) as i32) }).wrapping_shr((p_button_2) as u32)) & (1i32))) != 0) {
                F::Flash_ClipGotoLabel(c, l_flash_3, fp, sym::menu_q_b__str1);
                { let r_ = (!((1i32).wrapping_shl((p_button_2) as u32))); let a_ = ((c.m.r32(sym::gProgress)).wrapping_add(1612)).wrapping_add(52); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
            }
        } else {
            if ((((!((((({ let a_ = ((c.m.r32(sym::gProgress)).wrapping_add(1612)).wrapping_add(52); (c.m.r32(a_) as i32) }).wrapping_shr((p_button_2) as u32)) & (1i32))) != 0)) as i32)) != 0) {
                F::Flash_ClipGotoLabel(c, l_flash_3, fp, sym::menu_q_b__str2);
                { let r_ = (1i32).wrapping_shl((p_button_2) as u32); let a_ = ((c.m.r32(sym::gProgress)).wrapping_add(1612)).wrapping_add(52); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "SimDay_SetButton", &PROBE_N, &[], true); }
    r_
}

pub fn SimDay_PassWait(c: &mut Ctx, mut p_turns_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SimDay_PassWait", &PROBE_N, &[(p_turns_1) as u32], false); }
    let r_ = 'f: {
    if ((((({ let a_ = ((c.m.r32(sym::gProgress)).wrapping_add(1612)).wrapping_add(48); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
        { let r_ = p_turns_1; let a_ = ((c.m.r32(sym::gProgress)).wrapping_add(1612)).wrapping_add(48); let n_ = ((c.m.r32(a_) as i32)).wrapping_sub(r_); c.m.w32(a_, (n_) as u32); };
        if ((((({ let a_ = ((c.m.r32(sym::gProgress)).wrapping_add(1612)).wrapping_add(48); (c.m.r32(a_) as i32) }) < (0i32)) as i32)) != 0) {
            { let v_ = 0i32; let a_ = ((c.m.r32(sym::gProgress)).wrapping_add(1612)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "SimDay_PassWait", &PROBE_N, &[], true); }
    r_
}

pub fn SimDay_NextChange(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SimDay_NextChange", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let r_ = 'f: {
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (4i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((((({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(2960); (c.m.r32(a_) as i32) }).wrapping_shr((l_i_1) as u32)) & (1i32))) != 0) {
                    break 'f l_i_1;
                }
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
    break 'f (1i32).wrapping_neg();
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "SimDay_NextChange", &PROBE_N, &[], true); }
    r_
}

pub fn SimDay_ShowChange(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SimDay_ShowChange", &PROBE_N, &[], false); }
    let mut l_stat_1: i32 = 0;
    let mut l_step_2: i32 = 0;
    let mut l_neg_3: i32 = 0;
    let r_ = 'f: {
    l_stat_1 = F::SimDay_NextChange(c);
    if (((((l_stat_1) == ((1i32).wrapping_neg())) as i32)) != 0) {
        break 'f;
    }
    { let v_ = l_stat_1; let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(2960)).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
    's1: {
        let sv4: i32 = l_stat_1;
        match sv4 {
            0i32 | 1i32 => {
                {
                    l_step_2 = (((F::Rand_Range(c, ((2i32) as u32))).wrapping_add(((1i32) as u32))) as i32);
                    if ((((({ let a_ = (((c.m.r32(sym::gSimDay)).wrapping_add(2960)).wrapping_add(8)).wrapping_add((((l_stat_1) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
                        if ((((({ let a_ = (((c.m.r32(sym::gSimDay)).wrapping_add(2960)).wrapping_add(8)).wrapping_add((((l_stat_1) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) > (0i32)) as i32)) != 0) {
                            if ((((({ let a_ = (((c.m.r32(sym::gSimDay)).wrapping_add(2960)).wrapping_add(8)).wrapping_add((((l_stat_1) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) < (l_step_2)) as i32)) != 0) {
                                l_step_2 = { let a_ = (((c.m.r32(sym::gSimDay)).wrapping_add(2960)).wrapping_add(8)).wrapping_add((((l_stat_1) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
                            }
                            { let r_ = l_step_2; let a_ = (((c.m.r32(sym::gSimDay)).wrapping_add(2960)).wrapping_add(8)).wrapping_add((((l_stat_1) as u32)).wrapping_mul(4)); let n_ = ((c.m.r32(a_) as i32)).wrapping_sub(r_); c.m.w32(a_, (n_) as u32); };
                        } else {
                            l_neg_3 = (l_step_2).wrapping_neg();
                            if (((((l_neg_3) < ({ let a_ = (((c.m.r32(sym::gSimDay)).wrapping_add(2960)).wrapping_add(8)).wrapping_add((((l_stat_1) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                                l_step_2 = ({ let a_ = (((c.m.r32(sym::gSimDay)).wrapping_add(2960)).wrapping_add(8)).wrapping_add((((l_stat_1) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_neg();
                                l_neg_3 = { let a_ = (((c.m.r32(sym::gSimDay)).wrapping_add(2960)).wrapping_add(8)).wrapping_add((((l_stat_1) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
                            }
                            { let r_ = l_step_2; let a_ = (((c.m.r32(sym::gSimDay)).wrapping_add(2960)).wrapping_add(8)).wrapping_add((((l_stat_1) as u32)).wrapping_mul(4)); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                            l_step_2 = l_neg_3;
                        }
                        { let v_ = (l_step_2).wrapping_add(2i32); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(2960)).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                    } else {
                        { let v_ = 2i32; let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(2960)).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                        { let v_ = 0i32; let a_ = (((c.m.r32(sym::gSimDay)).wrapping_add(2960)).wrapping_add(8)).wrapping_add((((l_stat_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    }
                    { let v_ = ((F::Rand_Range(c, ((3i32) as u32))) as i32); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(2960)).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
                    if ((((({ let a_ = (((c.m.r32(sym::gSimDay)).wrapping_add(2960)).wrapping_add(8)).wrapping_add((((l_stat_1) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
                        { let r_ = (!((1i32).wrapping_shl((l_stat_1) as u32))); let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(2960); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                    }
                    F::SimDay_Cmd(c, 27i32);
                    break 's1;
                }
            }
            2i32 => {
                F::SimDay_Cmd(c, 29i32);
                { let v_ = { let a_ = (((c.m.r32(sym::gSimDay)).wrapping_add(2960)).wrapping_add(8)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(2960)).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                { let r_ = (!(4i32)); let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(2960); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            3i32 => {
                F::SimDay_Cmd(c, 31i32);
                { let v_ = { let a_ = (((c.m.r32(sym::gSimDay)).wrapping_add(2960)).wrapping_add(8)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(2960)).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                { let r_ = (!(8i32)); let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(2960); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            _ => {
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "SimDay_ShowChange", &PROBE_N, &[], true); }
    r_
}

pub fn SimDay_SetMenuAlpha(c: &mut Ctx, mut p_text_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SimDay_SetMenuAlpha", &PROBE_N, &[(p_text_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(80) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 80);
    let mut l_flash_2: u32 = 0;
    let mut l_nameAlpha_3: F32 = F32(0);
    let mut l_plateAlpha_4: F32 = F32(0);
    let mut l_i_5: i32 = 0;
    let r_ = 'f: {
        l_flash_2 = ((c.m.r32(sym::gSimDay)).wrapping_add(32)).wrapping_add((((5i32) as u32)).wrapping_mul(44));
        if ((p_text_1) != 0) {
            l_nameAlpha_3 = F32(0x3f800000);
            l_plateAlpha_4 = F32(0x00000000);
        } else {
            l_nameAlpha_3 = F32(0x00000000);
            l_plateAlpha_4 = F32(0x3f800000);
        }
        {
            l_i_5 = 0i32;
            'l1: loop {
                if !(((((l_i_5) < (3i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let _ = F::sprintf(c, fp.wrapping_add(8), sym::menu_q_b__str3, &[(((l_i_5).wrapping_add(1i32)) as i64 as u64)]); };
                    F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), fp.wrapping_add(8), fp);
                    F::Flash_ClipSetAlpha(c, l_flash_2, fp, l_nameAlpha_3);
                }
                { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
            }
        }
        {
            l_i_5 = 0i32;
            'l3: loop {
                if !(((((l_i_5) < (3i32)) as i32)) != 0) { break 'l3; }
                'c4: {
                    { let _ = F::sprintf(c, fp.wrapping_add(8), sym::menu_q_b__str4, &[(((l_i_5).wrapping_add(1i32)) as i64 as u64)]); };
                    F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), fp.wrapping_add(8), fp);
                    F::Flash_ClipSetAlpha(c, l_flash_2, fp, l_plateAlpha_4);
                }
                { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "SimDay_SetMenuAlpha", &PROBE_N, &[], true); }
    r_
}

pub fn SimDay_PlayBgm(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SimDay_PlayBgm", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let a0_ = ({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1448); (c.m.r32(a_) as i32) }).wrapping_add(68374i32); F::Bgm_Play(c, a0_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "SimDay_PlayBgm", &PROBE_N, &[], true); }
    r_
}

pub fn SimDay_Cmd(c: &mut Ctx, mut p_cmd_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SimDay_Cmd", &PROBE_N, &[(p_cmd_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(80) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 80);
    let mut l_flash_2: u32 = 0;
    let mut l_day_3: i32 = 0;
    let mut l_i_4: i32 = 0;
    let mut l_cur_5: i32 = 0;
    let r_ = 'f: {
        l_flash_2 = ((c.m.r32(sym::gSimDay)).wrapping_add(32)).wrapping_add((((4i32) as u32)).wrapping_mul(44));
        l_day_3 = { let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1456); (c.m.r32(a_) as i32) };
        if (((((p_cmd_1) < (54i32)) as i32)) != 0) {
            l_flash_2 = ((c.m.r32(sym::gSimDay)).wrapping_add(32)).wrapping_add((((3i32) as u32)).wrapping_mul(44));
            if (((((p_cmd_1) < (44i32)) as i32)) != 0) {
                l_flash_2 = ((c.m.r32(sym::gSimDay)).wrapping_add(32)).wrapping_add((((2i32) as u32)).wrapping_mul(44));
                if (((((p_cmd_1) < (35i32)) as i32)) != 0) {
                    l_flash_2 = ((c.m.r32(sym::gSimDay)).wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(44));
                    if (((((p_cmd_1) < (24i32)) as i32)) != 0) {
                        if (((((p_cmd_1) >= (19i32)) as i32)) != 0) {
                            l_flash_2 = ((c.m.r32(sym::gSimDay)).wrapping_add(32)).wrapping_add((((5i32) as u32)).wrapping_mul(44));
                        } else {
                            l_flash_2 = ((c.m.r32(sym::gSimDay)).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(44));
                        }
                    }
                }
            }
        }
        's1: {
            let sv6: i32 = p_cmd_1;
            match sv6 {
                0i32 => {
                    {
                        l_i_4 = 0i32;
                        'l2: loop {
                            if !(((((l_i_4) <= ({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1612); (c.m.r32(a_) as i32) })) as i32)) != 0) { break 'l2; }
                            'c3: {
                                { let _ = F::sprintf(c, fp.wrapping_add(8), sym::menu_q_b__str5, &[(((6i32).wrapping_sub(l_i_4)) as i64 as u64)]); };
                                F::Flash_FindLabel(c, l_flash_2, sym::menu_q_b__str6, fp.wrapping_add(8), fp);
                                F::Flash_ClipGotoLabel(c, l_flash_2, fp, sym::menu_q_b__str7);
                            }
                            { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
                        }
                    }
                    break 's1;
                }
                1i32 => {
                    {
                        l_i_4 = 0i32;
                        'l4: loop {
                            if !(((((l_i_4) <= ({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1612); (c.m.r32(a_) as i32) })) as i32)) != 0) { break 'l4; }
                            'c5: {
                                { let _ = F::sprintf(c, fp.wrapping_add(8), sym::menu_q_b__str8, &[(((6i32).wrapping_sub(l_i_4)) as i64 as u64)]); };
                                F::Flash_FindLabel(c, l_flash_2, sym::menu_q_b__str9, fp.wrapping_add(8), fp);
                                F::Flash_ClipGotoLabel(c, l_flash_2, fp, sym::menu_q_b__str10);
                            }
                            { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
                        }
                    }
                    break 's1;
                }
                2i32 => {
                    {
                        l_i_4 = 0i32;
                        'l6: loop {
                            if !(((((l_i_4) < (3i32)) as i32)) != 0) { break 'l6; }
                            'c7: {
                                if ((((!((((({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(2996); (c.m.r32(a_) as i32) }).wrapping_shr((l_i_4) as u32)) & (1i32))) != 0)) as i32)) != 0) {
                                    { let _ = F::sprintf(c, fp.wrapping_add(8), sym::menu_q_b__str11, &[((l_i_4) as i64 as u64)]); };
                                    F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), fp.wrapping_add(8), fp);
                                    F::Flash_ClipGotoLabel(c, l_flash_2, fp, sym::menu_q_b__str12);
                                    { let r_ = (1i32).wrapping_shl((l_i_4) as u32); let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(2996); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                                    break 'l6;
                                }
                            }
                            { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
                        }
                    }
                    break 's1;
                }
                3i32 => {
                    {
                        l_i_4 = (3i32).wrapping_sub(1i32);
                        'l8: loop {
                            if !(((((l_i_4) >= (0i32)) as i32)) != 0) { break 'l8; }
                            'c9: {
                                if ((((({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(2996); (c.m.r32(a_) as i32) }).wrapping_shr((l_i_4) as u32)) & (1i32))) != 0) {
                                    { let _ = F::sprintf(c, fp.wrapping_add(8), sym::menu_q_b__str13, &[((l_i_4) as i64 as u64)]); };
                                    F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), fp.wrapping_add(8), fp);
                                    F::Flash_ClipGotoLabel(c, l_flash_2, fp, sym::menu_q_b__str14);
                                    { let r_ = (!((1i32).wrapping_shl((l_i_4) as u32))); let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(2996); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                                    break 'l8;
                                }
                            }
                            { let o_ = l_i_4; let n_ = (o_).wrapping_sub(1); l_i_4 = n_; };
                        }
                    }
                    break 's1;
                }
                4i32 => {
                    F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), sym::menu_q_b__str15, fp);
                    F::Flash_ClipGotoLabel(c, l_flash_2, fp, sym::menu_q_b__str16);
                    break 's1;
                }
                5i32 => {
                    F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), sym::menu_q_b__str17, fp);
                    if (((({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1360); (c.m.r32(a_) as i32) }) & (32i32))) != 0) {
                        { let r_ = (!(32i32)); let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                        F::Flash_ClipGotoLabel(c, l_flash_2, fp, sym::menu_q_b__str18);
                    } else {
                        F::Flash_ClipGotoLabel(c, l_flash_2, fp, sym::menu_q_b__str19);
                    }
                    { let r_ = 64i32; let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    break 's1;
                }
                6i32 => {
                    { let a0_ = { let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(2940); (c.m.r32(a_) as i32) }; F::SimDay_SetFaceB(c, a0_) };
                    F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), sym::menu_q_b__str20, fp);
                    F::Flash_ClipGotoLabel(c, l_flash_2, fp, sym::menu_q_b__str21);
                    { let r_ = 32i32; let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    break 's1;
                }
                7i32 => {
                    F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), sym::menu_q_b__str22, fp);
                    F::Flash_ClipGotoLabel(c, l_flash_2, fp, sym::menu_q_b__str23);
                    { let r_ = (!(32i32)); let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                    break 's1;
                }
                8i32 => {
                    F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), sym::menu_q_b__str24, fp);
                    F::Flash_ClipGotoLabel(c, l_flash_2, fp, sym::menu_q_b__str25);
                    F::Flash_FindLabel(c, l_flash_2, sym::menu_q_b__str26, sym::menu_q_b__str27, fp);
                    F::Flash_ClipGotoLabel(c, l_flash_2, fp, sym::menu_q_b__str28);
                    break 's1;
                }
                9i32 => {
                    {
                        l_i_4 = 0i32;
                        'l10: loop {
                            if !(((((l_i_4) <= (l_day_3)) as i32)) != 0) { break 'l10; }
                            'c11: {
                                { let _ = F::sprintf(c, fp.wrapping_add(8), sym::menu_q_b__str29, &[((l_i_4) as i64 as u64)]); };
                                F::Flash_FindLabel(c, l_flash_2, sym::menu_q_b__str30, fp.wrapping_add(8), fp);
                                F::Flash_ClipGotoLabel(c, l_flash_2, fp, sym::menu_q_b__str31);
                            }
                            { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
                        }
                    }
                    break 's1;
                }
                10i32 => {
                    if (((((l_day_3) == ((10i32).wrapping_sub(1i32))) as i32)) != 0) {
                        F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), sym::menu_q_b__str32, fp);
                        F::Flash_ClipGotoLabel(c, l_flash_2, fp, sym::menu_q_b__str33);
                    }
                    break 's1;
                }
                11i32 => {
                    {
                        l_i_4 = 0i32;
                        'l12: loop {
                            if !(((((l_i_4) <= (l_day_3)) as i32)) != 0) { break 'l12; }
                            'c13: {
                                { let _ = F::sprintf(c, fp.wrapping_add(8), sym::menu_q_b__str34, &[((l_i_4) as i64 as u64)]); };
                                F::Flash_FindLabel(c, l_flash_2, sym::menu_q_b__str35, fp.wrapping_add(8), fp);
                                F::Flash_ClipGotoLabel(c, l_flash_2, fp, sym::menu_q_b__str36);
                            }
                            { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
                        }
                    }
                    F::Flash_FindLabel(c, l_flash_2, sym::menu_q_b__str37, sym::menu_q_b__str38, fp);
                    F::Flash_ClipGotoLabel(c, l_flash_2, fp, sym::menu_q_b__str39);
                    break 's1;
                }
                12i32 => {
                    F::Flash_GotoLabel(c, l_flash_2, sym::menu_q_b__str40, ((((1i32) as i64 as u64)) as u8));
                    break 's1;
                }
                13i32 => {
                    F::Flash_GotoLabel(c, l_flash_2, sym::menu_q_b__str41, ((((1i32) as i64 as u64)) as u8));
                    break 's1;
                }
                14i32 => {
                    F::Flash_GotoLabel(c, l_flash_2, sym::menu_q_b__str42, ((((1i32) as i64 as u64)) as u8));
                    F::SimDay_ClipGoto(c, 0i32, 1i32, sym::menu_q_b__str43);
                    { let v_ = 0i32; let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(1368)).wrapping_add(((({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1424); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    F::SimDay_ClipGoto(c, 0i32, 1i32, sym::menu_q_b__str44);
                    break 's1;
                }
                15i32 => {
                    F::Flash_GotoLabel(c, l_flash_2, sym::menu_q_b__str45, ((((1i32) as i64 as u64)) as u8));
                    break 's1;
                }
                16i32 => {
                    F::Flash_GotoLabel(c, l_flash_2, sym::menu_q_b__str46, ((((1i32) as i64 as u64)) as u8));
                    { let v_ = 0i32; let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1424); c.m.w32(a_, (v_) as u32); };
                    break 's1;
                }
                17i32 => {
                    { let v_ = 2i32; let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1424); c.m.w32(a_, (v_) as u32); };
                    F::Flash_GotoLabel(c, l_flash_2, sym::menu_q_b__str47, ((((1i32) as i64 as u64)) as u8));
                    F::SimDay_ClipGoto(c, 0i32, 2i32, sym::menu_q_b__str48);
                    { let v_ = 0i32; let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(1368)).wrapping_add(((({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1424); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    F::SimDay_ClipGoto(c, 0i32, 2i32, sym::menu_q_b__str49);
                    break 's1;
                }
                18i32 => {
                    F::Flash_GotoLabel(c, l_flash_2, sym::menu_q_b__str50, ((((1i32) as i64 as u64)) as u8));
                    { let v_ = 3i32; let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1424); c.m.w32(a_, (v_) as u32); };
                    break 's1;
                }
                19i32 => {
                    break 's1;
                }
                20i32 => {
                    {
                        if ((((({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1424); (c.m.r32(a_) as i32) }) != (9i32)) as i32)) != 0) {
                            { let v_ = { let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1424); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1428); c.m.w32(a_, (v_) as u32); };
                        }
                        { let v_ = 6i32; let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1424); c.m.w32(a_, (v_) as u32); };
                        F::Flash_GotoLabel(c, l_flash_2, sym::menu_q_b__str51, ((((1i32) as i64 as u64)) as u8));
                        l_cur_5 = { let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(1368)).wrapping_add(((({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1424); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
                        {
                            l_i_4 = 0i32;
                            'l14: loop {
                                if !(((((l_i_4) < (3i32)) as i32)) != 0) { break 'l14; }
                                'c15: {
                                    { let v_ = l_i_4; let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(1368)).wrapping_add(((({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1424); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                    if (((((l_i_4) == (l_cur_5)) as i32)) != 0) {
                                        F::SimDay_ClipGoto(c, 5i32, 6i32, sym::menu_q_b__str52);
                                    } else {
                                        F::SimDay_ClipGoto(c, 5i32, 6i32, sym::menu_q_b__str53);
                                    }
                                }
                                { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
                            }
                        }
                        { let v_ = l_cur_5; let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(1368)).wrapping_add(((({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1424); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                        { let r_ = 8192i32; let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 4i32); };
                        { let v_ = 0i32; let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1444); c.m.w32(a_, (v_) as u32); };
                        break 's1;
                    }
                }
                21i32 => {
                    F::Flash_GotoLabel(c, l_flash_2, sym::menu_q_b__str54, ((((1i32) as i64 as u64)) as u8));
                    { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 5i32); };
                    break 's1;
                }
                22i32 => {
                    F::Flash_GotoLabel(c, l_flash_2, sym::menu_q_b__str55, ((((1i32) as i64 as u64)) as u8));
                    { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 4i32); };
                    break 's1;
                }
                23i32 => {
                    F::Flash_GotoLabel(c, l_flash_2, sym::menu_q_b__str56, ((((1i32) as i64 as u64)) as u8));
                    { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 5i32); };
                    break 's1;
                }
                24i32 => {
                    break 's1;
                }
                25i32 => {
                    F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), sym::menu_q_b__str57, fp);
                    F::Flash_ClipGotoLabel(c, l_flash_2, fp, sym::menu_q_b__str58);
                    { let v_ = 4i32; let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1424); c.m.w32(a_, (v_) as u32); };
                    { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 25i32); };
                    break 's1;
                }
                26i32 => {
                    F::SimDay_ShowChange(c);
                    { let v_ = 4i32; let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1424); c.m.w32(a_, (v_) as u32); };
                    break 's1;
                }
                27i32 => {
                    F::Flash_GotoLabel(c, l_flash_2, sym::menu_q_b__str59, ((((1i32) as i64 as u64)) as u8));
                    break 's1;
                }
                28i32 => {
                    {
                        l_i_4 = 0i32;
                        'l16: loop {
                            if !(((((l_i_4) < (3i32)) as i32)) != 0) { break 'l16; }
                            'c17: {
                                { let _ = F::sprintf(c, fp.wrapping_add(8), sym::menu_q_b__str60, &[((l_i_4) as i64 as u64)]); };
                                F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), fp.wrapping_add(8), fp);
                                F::Flash_ClipGotoLabel(c, l_flash_2, fp, sym::menu_q_b__str61);
                            }
                            { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
                        }
                    }
                    break 's1;
                }
                29i32 => {
                    F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), sym::menu_q_b__str62, fp);
                    F::Flash_ClipGotoLabel(c, l_flash_2, fp, sym::menu_q_b__str63);
                    break 's1;
                }
                30i32 => {
                    F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), sym::menu_q_b__str64, fp);
                    F::Flash_ClipGotoLabel(c, l_flash_2, fp, sym::menu_q_b__str65);
                    break 's1;
                }
                31i32 => {
                    F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), sym::menu_q_b__str66, fp);
                    F::Flash_ClipGotoLabel(c, l_flash_2, fp, sym::menu_q_b__str67);
                    break 's1;
                }
                32i32 => {
                    F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), sym::menu_q_b__str68, fp);
                    F::Flash_ClipGotoLabel(c, l_flash_2, fp, sym::menu_q_b__str69);
                    break 's1;
                }
                33i32 => {
                    if ((((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1612); (c.m.r32(a_) as i32) }) == (6i32)) as i32)) != 0) {
                        { let r_ = (!(1024i32)); let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                    } else {
                        l_i_4 = ({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1612); (c.m.r32(a_) as i32) }).wrapping_add(1i32);
                        'l18: loop {
                            'c19: {
                                { let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1612); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                                { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
                            }
                            if !((((((((({ let a_ = (({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(2912); c.m.r32(a_) }).wrapping_add((((l_i_4) as u32)).wrapping_mul(8))).wrapping_add(4); (c.m.r32(a_) as i32) }) < ({ let a_ = (((c.m.r32(sym::gProgress)).wrapping_add(1612)).wrapping_add(4)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) })) as i32)) != 0) && ((((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1612); (c.m.r32(a_) as i32) }) < (6i32)) as i32)) != 0)) as i32)) != 0) { break 'l18; }
                        }
                        { let r_ = 16i32; let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(2960); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                        F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), sym::menu_q_b__str70, fp);
                        F::Flash_ClipGotoLabel(c, l_flash_2, fp, sym::menu_q_b__str71);
                        { let v_ = 87i32; let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                        { let v_ = 4i32; let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1424); c.m.w32(a_, (v_) as u32); };
                        { let r_ = 1024i32; let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    }
                    break 's1;
                }
                34i32 => {
                    break 's1;
                }
                35i32 => {
                    break 's1;
                }
                36i32 => {
                    F::Flash_Play(c, l_flash_2, 1i32);
                    break 's1;
                }
                37i32 => {
                    {
                        l_i_4 = 0i32;
                        'l20: loop {
                            if !(((((l_i_4) < ({ let a_ = (sym::gSimTrain0).wrapping_add(((({ let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(2944)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(24)); (c.m.r32(a_) as i32) })) as i32)) != 0) { break 'l20; }
                            'c21: {
                                { let _ = F::sprintf(c, fp.wrapping_add(8), sym::menu_q_b__str72, &[((l_i_4) as i64 as u64)]); };
                                F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), fp.wrapping_add(8), fp);
                                F::Flash_ClipGotoLabel(c, l_flash_2, fp, sym::menu_q_b__str73);
                            }
                            { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
                        }
                    }
                    break 's1;
                }
                38i32 => {
                    {
                        l_i_4 = 0i32;
                        'l22: loop {
                            if !(((((l_i_4) < ({ let a_ = (sym::gSimTrain0).wrapping_add(((({ let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(2944)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(24)); (c.m.r32(a_) as i32) })) as i32)) != 0) { break 'l22; }
                            'c23: {
                                { let _ = F::sprintf(c, fp.wrapping_add(8), sym::menu_q_b__str74, &[((l_i_4) as i64 as u64)]); };
                                F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), fp.wrapping_add(8), fp);
                                F::Flash_ClipGotoLabel(c, l_flash_2, fp, sym::menu_q_b__str75);
                            }
                            { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
                        }
                    }
                    break 's1;
                }
                39i32 => {
                    { let _ = { let a2_ = { let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(1368)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; F::sprintf(c, fp.wrapping_add(8), sym::menu_q_b__str76, &[((a2_) as i64 as u64)]) }; };
                    F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), fp.wrapping_add(8), fp);
                    F::Flash_ClipGotoLabel(c, l_flash_2, fp, sym::menu_q_b__str77);
                    break 's1;
                }
                40i32 => {
                    F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), sym::menu_q_b__str78, fp);
                    F::Flash_ClipGotoLabel(c, l_flash_2, fp, sym::menu_q_b__str79);
                    break 's1;
                }
                41i32 => {
                    F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), sym::menu_q_b__str80, fp);
                    F::Flash_ClipGotoLabel(c, l_flash_2, fp, sym::menu_q_b__str81);
                    break 's1;
                }
                42i32 => {
                    F::Flash_GotoLabel(c, l_flash_2, sym::menu_q_b__str82, ((((1i32) as i64 as u64)) as u8));
                    break 's1;
                }
                43i32 => {
                    F::Flash_Reset(c, l_flash_2, 0i32);
                    F::Flash_Stop(c, l_flash_2);
                    break 's1;
                }
                44i32 => {
                    break 's1;
                }
                45i32 => {
                    F::Flash_Play(c, l_flash_2, 1i32);
                    break 's1;
                }
                46i32 => {
                    F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), sym::menu_q_b__str83, fp);
                    F::Flash_ClipGotoLabel(c, l_flash_2, fp, sym::menu_q_b__str84);
                    break 's1;
                }
                47i32 => {
                    F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), sym::menu_q_b__str85, fp);
                    F::Flash_ClipGotoLabel(c, l_flash_2, fp, sym::menu_q_b__str86);
                    break 's1;
                }
                48i32 => {
                    {
                        l_i_4 = 0i32;
                        'l24: loop {
                            if !(((((l_i_4) < (5i32)) as i32)) != 0) { break 'l24; }
                            'c25: {
                                { let _ = F::sprintf(c, fp.wrapping_add(8), sym::menu_q_b__str87, &[((l_i_4) as i64 as u64)]); };
                                F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), fp.wrapping_add(8), fp);
                                F::Flash_ClipGotoLabel(c, l_flash_2, fp, sym::menu_q_b__str88);
                            }
                            { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
                        }
                    }
                    break 's1;
                }
                49i32 => {
                    { let _ = { let a2_ = { let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(1368)).wrapping_add(((({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1424); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; F::sprintf(c, fp.wrapping_add(8), sym::menu_q_b__str89, &[((a2_) as i64 as u64)]) }; };
                    F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), fp.wrapping_add(8), fp);
                    F::Flash_ClipGotoLabel(c, l_flash_2, fp, sym::menu_q_b__str90);
                    break 's1;
                }
                50i32 => {
                    F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), sym::menu_q_b__str91, fp);
                    F::Flash_ClipGotoLabel(c, l_flash_2, fp, sym::menu_q_b__str92);
                    break 's1;
                }
                51i32 => {
                    F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), sym::menu_q_b__str93, fp);
                    F::Flash_ClipGotoLabel(c, l_flash_2, fp, sym::menu_q_b__str94);
                    break 's1;
                }
                52i32 => {
                    F::Flash_GotoLabel(c, l_flash_2, sym::menu_q_b__str95, ((((1i32) as i64 as u64)) as u8));
                    break 's1;
                }
                53i32 => {
                    F::Flash_Reset(c, l_flash_2, 0i32);
                    F::Flash_Stop(c, l_flash_2);
                    break 's1;
                }
                54i32 => {
                    break 's1;
                }
                55i32 => {
                    F::Flash_Play(c, l_flash_2, 1i32);
                    break 's1;
                }
                56i32 => {
                    F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), sym::menu_q_b__str96, fp);
                    F::Flash_ClipGotoLabel(c, l_flash_2, fp, sym::menu_q_b__str97);
                    break 's1;
                }
                57i32 => {
                    F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), sym::menu_q_b__str98, fp);
                    F::Flash_ClipGotoLabel(c, l_flash_2, fp, sym::menu_q_b__str99);
                    F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), sym::menu_q_b__str100, fp);
                    F::Flash_ClipGotoLabel(c, l_flash_2, fp, sym::menu_q_b__str101);
                    break 's1;
                }
                58i32 => {
                    F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), sym::menu_q_b__str102, fp);
                    F::Flash_ClipGotoLabel(c, l_flash_2, fp, sym::menu_q_b__str103);
                    F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), sym::menu_q_b__str104, fp);
                    F::Flash_ClipGotoLabel(c, l_flash_2, fp, sym::menu_q_b__str105);
                    break 's1;
                }
                59i32 => {
                    F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), sym::menu_q_b__str106, fp);
                    F::Flash_ClipGotoLabel(c, l_flash_2, fp, sym::menu_q_b__str107);
                    break 's1;
                }
                60i32 => {
                    F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), sym::menu_q_b__str108, fp);
                    F::Flash_ClipGotoLabel(c, l_flash_2, fp, sym::menu_q_b__str109);
                    break 's1;
                }
                61i32 => {
                    F::Flash_GotoLabel(c, l_flash_2, sym::menu_q_b__str110, ((((1i32) as i64 as u64)) as u8));
                    break 's1;
                }
                62i32 => {
                    F::Flash_Reset(c, l_flash_2, 0i32);
                    F::Flash_Stop(c, l_flash_2);
                    break 's1;
                }
                _ => {
                }
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "SimDay_Cmd", &PROBE_N, &[], true); }
    r_
}

pub fn SimDay_PickEvent(c: &mut Ctx, mut p_unused_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SimDay_PickEvent", &PROBE_N, &[(p_unused_1) as u32], false); }
    let mut l_r_2: i32 = 0;
    let mut l_sum_3: i32 = 0;
    let mut l_i_4: i32 = 0;
    let mut l_w_5: i32 = 0;
    let r_ = 'f: {
    l_r_2 = ((F::Rand_Range(c, ((256i32) as u32))) as i32);
    { let v_ = 0i32; let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(2920)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    { let v_ = 0i32; let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(2920)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    's1: {
        let sv6: i32 = { let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(1368)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
        match sv6 {
            0i32 => {
                { let v_ = { let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(1368)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(2916); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            1i32 => {
                l_sum_3 = 0i32;
                {
                    l_i_4 = 0i32;
                    'l2: loop {
                        if !(((((l_i_4) < (32i32)) as i32)) != 0) { break 'l2; }
                        'c3: {
                            l_w_5 = { let a_ = (({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(2888); c.m.r32(a_) }).wrapping_add((((l_i_4) as u32)).wrapping_mul(20))).wrapping_add(((({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(2884); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
                            if (((((l_w_5) == (0i32)) as i32)) != 0) {
                                break 'c3;
                            }
                            { let r_ = l_w_5; let n_ = (l_sum_3).wrapping_add(r_); l_sum_3 = n_; };
                            if (((((l_r_2) < (l_sum_3)) as i32)) != 0) {
                                break 'l2;
                            }
                        }
                        { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
                    }
                }
                { let v_ = (l_i_4).wrapping_add(5i32); let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(2916); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            2i32 => {
                { let v_ = 3i32; let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(2916); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            3i32 => {
                { let v_ = 4i32; let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(2916); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            _ => {
            }
        }
    }
    { let v_ = 0i32; let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(2956); c.m.w32(a_, (v_) as u32); };
    if ((((({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(2916); (c.m.r32(a_) as i32) }) == ({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(2928); (c.m.r32(a_) as i32) })) as i32)) != 0) {
        { let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(2932); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        if ((((({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(2932); (c.m.r32(a_) as i32) }) >= (3i32)) as i32)) != 0) {
            { let v_ = 2i32; let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(2932); c.m.w32(a_, (v_) as u32); };
        }
    } else {
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(2932); c.m.w32(a_, (v_) as u32); };
    }
    { let v_ = { let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(2916); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(2928); c.m.w32(a_, (v_) as u32); };
    { let v_ = 0i32; let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(1368)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    { let v_ = (1i32).wrapping_neg(); let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "SimDay_PickEvent", &PROBE_N, &[], true); }
    r_
}

pub fn SimDay_RefreshBoard(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SimDay_RefreshBoard", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let r_ = 'f: {
    F::SimDay_Cmd(c, 0i32);
    F::SimDay_Cmd(c, 8i32);
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (3i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((((({ let a_ = (((c.m.r32(sym::gProgress)).wrapping_add(1612)).wrapping_add(24)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) != ((1i32).wrapping_neg())) as i32)) != 0) {
                    F::SimDay_Cmd(c, 2i32);
                }
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
    { let v_ = ({ let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(1368)).wrapping_add(((({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1424); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(80i32); let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
    if ((((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1612); (c.m.r32(a_) as i32) }) < ((6i32).wrapping_add(1i32))) as i32)) != 0) {
        if ((((({ let a_ = (({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(2912); c.m.r32(a_) }).wrapping_add((((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1612); (c.m.r32(a_) as i32) }).wrapping_add(1i32)) as u32)).wrapping_mul(8))).wrapping_add(4); (c.m.r32(a_) as i32) }) < ({ let a_ = (((c.m.r32(sym::gProgress)).wrapping_add(1612)).wrapping_add(4)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) })) as i32)) != 0) {
            { let v_ = 1i32; let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(3000); c.m.w32(a_, (v_) as u32); };
            F::SimDay_Cmd(c, 33i32);
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "SimDay_RefreshBoard", &PROBE_N, &[], true); }
    r_
}

pub fn SimDay_Init(c: &mut Ctx, mut p_section_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SimDay_Init", &PROBE_N, &[(p_section_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_i_2: i32 = 0;
    let mut l_check_3: u32 = 0;
    let mut l_hp_4: i32 = 0;
    let r_ = 'f: {
        { let v_ = ((0i32) as u32); c.m.w32(fp, v_); }
        if (((((c.m.r32(sym::gMenuArc3)) != (((0i32) as u32))) as i32)) != 0) {
            { let a0_ = c.m.r32(sym::gMenuArc3); F::Heap_Free(c, a0_) };
            { let v_ = ((0i32) as u32); let a_ = sym::gMenuArc3; c.m.w32(a_, v_); };
        }
        { let v_ = F::Heap_Alloc(c, ((3004u32) as i32), ((32i32) as u32), 0i32, 2i32); let a_ = sym::gSimDay; c.m.w32(a_, v_); };
        { let _ = { let a0_ = c.m.r32(sym::gSimDay); F::memset(c, a0_, 0i32, 3004u32) }; };
        {
            l_i_2 = 0i32;
            'l1: loop {
                if !(((((l_i_2) < (3i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let v_ = F::SimDay_GetTrainRank(c, l_i_2); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(2944)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        { let v_ = { let a0_ = ({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_add(26i32); F::File_LoadSync(c, a0_, ((0i32) as u32), 0i32) }; let a_ = c.m.r32(sym::gSimDay); c.m.w32(a_, v_); };
        l_check_3 = { let a_ = c.m.r32(sym::gSimDay); c.m.r32(a_) };
        { let v_ = { let a0_ = { let a_ = c.m.r32(sym::gSimDay); c.m.r32(a_) }; F::Sprite_Unpack(c, a0_, ((0i32) as u32), ((0i32) as u32)) }; let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(4); c.m.w32(a_, v_); };
        l_check_3 = { let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(4); c.m.r32(a_) };
        { let v_ = F::Heap_Alloc(c, 92160i32, ((64i32) as u32), 0i32, 2i32); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = F::Heap_Alloc(c, 92160i32, ((64i32) as u32), 0i32, 2i32); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(8)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = F::Heap_Alloc(c, 133120i32, ((32i32) as u32), 0i32, 2i32); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = F::Heap_Alloc(c, 133120i32, ((32i32) as u32), 0i32, 2i32); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = c.m.r32(fp); let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(340); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(344)).wrapping_add((((55i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(344)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(344)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(344)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((4i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(344)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((5i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(344)).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((6i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(344)).wrapping_add((((49i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((7i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(344)).wrapping_add((((51i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((8i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(344)).wrapping_add((((50i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((9i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(344)).wrapping_add((((57i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((10i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(344)).wrapping_add((((45i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((11i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(344)).wrapping_add((((44i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((12i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(344)).wrapping_add((((46i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((13i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(344)).wrapping_add((((56i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((14i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(344)).wrapping_add((((58i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((15i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(344)).wrapping_add((((59i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((16i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(344)).wrapping_add((((60i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((17i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(344)).wrapping_add((((62i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((18i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(344)).wrapping_add((((61i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((19i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(344)).wrapping_add((((54i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((20i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(344)).wrapping_add((((19i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((21i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(344)).wrapping_add((((20i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((22i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(344)).wrapping_add((((8i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((23i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(344)).wrapping_add((((9i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((24i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(344)).wrapping_add((((18i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((25i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(344)).wrapping_add((((11i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((26i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(344)).wrapping_add((((21i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((27i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(344)).wrapping_add((((22i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((28i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(344)).wrapping_add((((10i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((29i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(344)).wrapping_add((((27i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((30i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(344)).wrapping_add((((28i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((31i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(344)).wrapping_add((((16i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((32i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(344)).wrapping_add((((17i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((33i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(344)).wrapping_add((((14i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((34i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(344)).wrapping_add((((15i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((35i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(344)).wrapping_add((((12i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((36i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(344)).wrapping_add((((13i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((37i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(344)).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((38i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(344)).wrapping_add((((23i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((39i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(344)).wrapping_add((((24i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((40i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(344)).wrapping_add((((25i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((41i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(344)).wrapping_add((((26i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((42i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(344)).wrapping_add((((29i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((43i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(344)).wrapping_add((((30i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((44i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(344)).wrapping_add((((35i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((45i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(344)).wrapping_add((((34i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((46i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(344)).wrapping_add((((38i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((47i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(344)).wrapping_add((((31i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((48i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(344)).wrapping_add((((37i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((49i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(344)).wrapping_add((((36i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((50i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(344)).wrapping_add((((47i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((51i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(344)).wrapping_add((((48i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((52i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(344)).wrapping_add((((52i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((53i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(344)).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((54i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(344)).wrapping_add((((40i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((55i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(344)).wrapping_add((((41i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((56i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(344)).wrapping_add((((43i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((57i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(344)).wrapping_add((((42i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((58i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(344)).wrapping_add((((39i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(344)).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(344)).wrapping_add((((32i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((59i32).wrapping_mul(64i32)) as u32)); let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(936); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((31i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(344)).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((8i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add(((({ let a_ = ((c.m.r32(sym::gProgress)).wrapping_add(1088)).wrapping_add(28); (c.m.r32(a_) as i32) }).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(344)).wrapping_add((((53i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(928); c.m.w32(a_, v_); };
        { let a0_ = (c.m.r32(sym::gSimDay)).wrapping_add(928); let a1_ = { let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(928); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(928); c.m.r32(a_) }; F::Res_RelocateOffsets(c, a0_, a1_, a2_) };
        F::SimDay_SetFaceA(c, 0i32);
        { let v_ = ({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(932); c.m.w32(a_, v_); };
        { let a0_ = (c.m.r32(sym::gSimDay)).wrapping_add(932); let a1_ = { let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(932); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(932); c.m.r32(a_) }; F::Res_RelocateOffsets(c, a0_, a1_, a2_) };
        F::SimDay_SetFaceB(c, 0i32);
        { let v_ = ({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((23i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(596)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(596)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(596)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(596)).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((4i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(596)).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(596)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((19i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(620)).wrapping_add((((9i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(620)).wrapping_add((((10i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(620)).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(620)).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((4i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(620)).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((5i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(620)).wrapping_add((((8i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((6i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(620)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((7i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(620)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((20i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(620)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((21i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(620)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((22i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(924); c.m.w32(a_, v_); };
        { let a0_ = (c.m.r32(sym::gSimDay)).wrapping_add(924); let a1_ = { let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(924); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(924); c.m.r32(a_) }; F::Res_RelocateOffsets(c, a0_, a1_, a2_) };
        { let v_ = ({ let a_ = ({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(924); c.m.r32(a_) }).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(620)).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((16i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(664)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((5i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(664)).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((6i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(664)).wrapping_add((((8i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((7i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(664)).wrapping_add((((10i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((8i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(664)).wrapping_add((((9i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((9i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(664)).wrapping_add((((11i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((10i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(664)).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((12i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(664)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((13i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(664)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((14i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(664)).wrapping_add((((12i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((19i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(664)).wrapping_add((((13i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((27i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(664)).wrapping_add((((15i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(728)).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(728)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((12i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(728)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((13i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(728)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((14i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(728)).wrapping_add((((11i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((19i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(728)).wrapping_add((((13i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((22i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(728)).wrapping_add((((12i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((23i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(728)).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((24i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(728)).wrapping_add((((8i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((25i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(728)).wrapping_add((((9i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((26i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(728)).wrapping_add((((10i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(788)).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(788)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((4i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(788)).wrapping_add((((9i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((11i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(788)).wrapping_add((((12i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((12i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(788)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((13i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(788)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((15i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(788)).wrapping_add((((8i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((16i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(788)).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((17i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(788)).wrapping_add((((13i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((18i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(788)).wrapping_add((((11i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((19i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(788)).wrapping_add((((14i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((20i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(788)).wrapping_add((((16i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((17i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(664)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(664)).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(664)).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(664)).wrapping_add((((14i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(728)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(728)).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(728)).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(728)).wrapping_add((((14i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(788)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(788)).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(788)).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(788)).wrapping_add((((15i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((4i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(788)).wrapping_add((((10i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((5i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(788)).wrapping_add((((17i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(860)).wrapping_add((((9i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(860)).wrapping_add((((10i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(860)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(860)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((4i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(860)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((5i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(860)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((6i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(860)).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((7i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(860)).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((8i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(860)).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((9i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(860)).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((10i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(860)).wrapping_add((((11i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((11i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(860)).wrapping_add((((12i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((12i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(860)).wrapping_add((((8i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((13i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(860)).wrapping_add((((15i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let a0_ = ((c.m.r32(sym::gSimDay)).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); let a1_ = ({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a2_ = (c.m.r32(sym::gSimDay)).wrapping_add(344); F::Flash_Create(c, a0_, a1_, a2_) };
        { let a0_ = ((c.m.r32(sym::gSimDay)).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_Play(c, a0_, 1i32) };
        { let a0_ = ((c.m.r32(sym::gSimDay)).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_Advance(c, a0_) };
        { let a0_ = ((c.m.r32(sym::gSimDay)).wrapping_add(32)).wrapping_add((((5i32) as u32)).wrapping_mul(44)); let a1_ = ({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((15i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a2_ = (c.m.r32(sym::gSimDay)).wrapping_add(596); F::Flash_Create(c, a0_, a1_, a2_) };
        { let a0_ = ((c.m.r32(sym::gSimDay)).wrapping_add(32)).wrapping_add((((5i32) as u32)).wrapping_mul(44)); F::Flash_Play(c, a0_, 1i32) };
        { let a0_ = ((c.m.r32(sym::gSimDay)).wrapping_add(32)).wrapping_add((((6i32) as u32)).wrapping_mul(44)); let a1_ = ({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a2_ = (c.m.r32(sym::gSimDay)).wrapping_add(860); F::Flash_Create(c, a0_, a1_, a2_) };
        { let a0_ = ((c.m.r32(sym::gSimDay)).wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(44)); let a1_ = ({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((18i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a2_ = (c.m.r32(sym::gSimDay)).wrapping_add(620); F::Flash_Create(c, a0_, a1_, a2_) };
        { let a0_ = ((c.m.r32(sym::gSimDay)).wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(44)); F::Flash_Play(c, a0_, 1i32) };
        { let a0_ = ((c.m.r32(sym::gSimDay)).wrapping_add(32)).wrapping_add((((2i32) as u32)).wrapping_mul(44)); let a1_ = ({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((({ let a_ = (sym::gSimTrain0).wrapping_add(((({ let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(2944)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(24)); (c.m.r32(a_) as i32) }).wrapping_add(5i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a2_ = (c.m.r32(sym::gSimDay)).wrapping_add(664); F::Flash_Create(c, a0_, a1_, a2_) };
        { let a0_ = ((c.m.r32(sym::gSimDay)).wrapping_add(32)).wrapping_add((((3i32) as u32)).wrapping_mul(44)); let a1_ = ({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((13i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a2_ = (c.m.r32(sym::gSimDay)).wrapping_add(728); F::Flash_Create(c, a0_, a1_, a2_) };
        { let a0_ = ((c.m.r32(sym::gSimDay)).wrapping_add(32)).wrapping_add((((4i32) as u32)).wrapping_mul(44)); let a1_ = ({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((14i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a2_ = (c.m.r32(sym::gSimDay)).wrapping_add(788); F::Flash_Create(c, a0_, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((24i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(24); c.m.w32(a_, v_); };
        { let a0_ = ({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((9i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a1_ = { let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(24); c.m.r32(a_) }; let _ = 0i32; F::MsgWin_Init(c, a0_, a1_, 1i32) };
        F::MsgWin_Open(c);
        { let v_ = ({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((32i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(28); c.m.w32(a_, v_); };
        {
            l_i_2 = 0i32;
            'l3: loop {
                if !(((((l_i_2) < (3i32)) as i32)) != 0) { break 'l3; }
                'c4: {
                    { let a0_ = ((c.m.r32(sym::gSimDay)).wrapping_add(940)).wrapping_add((((l_i_2) as u32)).wrapping_mul(140)); let a1_ = { let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(28); c.m.r32(a_) }; F::TextBox_Init(c, a0_, a1_, ((0i32) as u32)) };
                    { let a0_ = ((c.m.r32(sym::gSimDay)).wrapping_add(940)).wrapping_add((((l_i_2) as u32)).wrapping_mul(140)); F::TextBox_SetUnk50(c, a0_, 1i32) };
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        { let a0_ = ({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((33i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a1_ = { let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(24); c.m.r32(a_) }; F::Dialog_Init(c, a0_, a1_, 0i32) };
        { let v_ = ({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((28i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(2888); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((25i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(2892); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((27i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(2896); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((26i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(2900); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((29i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(2908); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((30i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(2912); c.m.w32(a_, v_); };
        { let v_ = (1i32).wrapping_neg(); let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
        { let v_ = (1i32).wrapping_neg(); let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(2928); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1444); c.m.w32(a_, (v_) as u32); };
        if ((((({ let a_ = ((c.m.r32(sym::gProgress)).wrapping_add(1612)).wrapping_add(40); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
            l_hp_4 = rt::f2i({ let a_ = ((F::BattleResult_GetPtr(c)).wrapping_add(44)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) });
            if (((((l_hp_4) == (0i32)) as i32)) != 0) {
                l_hp_4 = 1i32;
            }
            { let v_ = l_hp_4; let a_ = (((c.m.r32(sym::gProgress)).wrapping_add(1612)).wrapping_add(4)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            { let v_ = rt::rem_i32({ let a_ = ((c.m.r32(sym::gProgress)).wrapping_add(1612)).wrapping_add(40); (c.m.r32(a_) as i32) }, 10i32); let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1456); c.m.w32(a_, (v_) as u32); };
            F::SimDay_PassWait(c, 1i32);
            if ((((({ let a_ = ((c.m.r32(sym::gProgress)).wrapping_add(1612)).wrapping_add(48); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
                F::SimDay_SetButton(c, 0i32, 3i32);
            } else {
                F::SimDay_SetButton(c, 1i32, 3i32);
            }
            if ((((({ let a_ = (((c.m.r32(sym::gProgress)).wrapping_add(1612)).wrapping_add(4)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) < (21i32)) as i32)) != 0) {
                F::SimDay_SetButton(c, 0i32, 0i32);
                F::SimDay_SetButton(c, 0i32, 1i32);
                { let v_ = 2i32; let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(1368)).wrapping_add(((({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1424); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            }
        }
        F::SimDay_ClipGoto(c, 0i32, 0i32, sym::menu_q_b__str111);
        F::SimDay_ListItems(c);
        { let v_ = { let a_ = (sym::gSimBgm).wrapping_add((((rt::div_i32({ let a_ = ((c.m.r32(sym::gProgress)).wrapping_add(1612)).wrapping_add(40); (c.m.r32(a_) as i32) }, 10i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1448); c.m.w32(a_, (v_) as u32); };
        F::SimDay_PlayBgm(c);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "SimDay_Init", &PROBE_N, &[], true); }
    r_
}

pub fn SimDay_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SimDay_Term", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let r_ = 'f: {
    F::MsgWin_Term(c);
    F::Dialog_Term(c);
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (7i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let a0_ = ((c.m.r32(sym::gSimDay)).wrapping_add(32)).wrapping_add((((l_i_1) as u32)).wrapping_mul(44)); F::Flash_Destroy(c, a0_) };
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
    if ((((({ let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = { let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    }
    if ((((({ let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(8)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = { let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(8)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(8)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    }
    if ((((({ let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = { let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    }
    if ((((({ let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = { let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    }
    if ((((({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(4); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = { let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(4); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(4); c.m.w32(a_, v_); };
    }
    if ((((({ let a_ = c.m.r32(sym::gSimDay); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = { let a_ = c.m.r32(sym::gSimDay); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = c.m.r32(sym::gSimDay); c.m.w32(a_, v_); };
    }
    if (((((c.m.r32(sym::gSimDay)) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32(sym::gSimDay); F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = sym::gSimDay; c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "SimDay_Term", &PROBE_N, &[], true); }
    r_
}

pub fn SimDay_Draw(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SimDay_Draw", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(144) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 144);
    let mut l_flash_1: u32 = 0;
    let mut l_i_2: i32 = 0;
    let mut l_n_3: i32 = 0;
    let mut l_done_4: i32 = 0;
    let mut l_slot_5: i32 = 0;
    let r_ = 'f: {
        l_i_2 = 0i32;
        { let a0_ = { let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(340); c.m.r32(a_) }; F::Sprite_DrawPicture(c, a0_, 0i32, 0i32, 128i32) };
        l_flash_1 = ((c.m.r32(sym::gSimDay)).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(44));
        F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_q_b__str112, fp);
        { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 1024i32, ((((1i32) as i64 as u64)) as u8)); };
        { let _ = F::sprintf(c, fp.wrapping_add(28), sym::menu_q_b__str113, &[((0i32) as i64 as u64)]); };
        F::Flash_FindLabel(c, l_flash_1, sym::menu_q_b__str114, fp.wrapping_add(28), fp);
        F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_q_b__str115, fp);
        { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 1024i32, ((((1i32) as i64 as u64)) as u8)); };
        { let a5_ = ({ let a_ = ((c.m.r32(sym::gProgress)).wrapping_add(1612)).wrapping_add(40); (c.m.r32(a_) as i32) }).wrapping_add(1i32); F::Num_DrawChild(c, l_flash_1, sym::menu_q_b__str116, sym::menu_q_b__str117, 0i32, 3i32, a5_, 32i32, 32i32, 1i32, 0i32) };
        F::Flash_FindLabel(c, l_flash_1, sym::menu_q_b__str118, sym::menu_q_b__str119, fp);
        { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 128i32, ((((1i32) as i64 as u64)) as u8)); };
        F::Flash_FindLabel(c, l_flash_1, sym::menu_q_b__str120, sym::menu_q_b__str121, fp);
        { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 256i32, ((((1i32) as i64 as u64)) as u8)); };
        { let a2_ = rt::f2i(rt::fmul(rt::i2f(({ let a_ = (((c.m.r32(sym::gProgress)).wrapping_add(1612)).wrapping_add(4)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_sub(100i32)), F32(0x3fc66666))); F::Flash_ClipSetOffset(c, l_flash_1, fp, a2_, 0i32) };
        { let a5_ = { let a_ = (((c.m.r32(sym::gProgress)).wrapping_add(1612)).wrapping_add(4)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; F::Num_DrawChild(c, l_flash_1, sym::menu_q_b__str122, sym::menu_q_b__str123, 0i32, 3i32, a5_, 32i32, 32i32, 1i32, 0i32) };
        if ((((({ let a_ = (((c.m.r32(sym::gProgress)).wrapping_add(1612)).wrapping_add(4)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) >= (0i32)) as i32)) != 0) {
            {
                l_i_2 = 0i32;
                'l1: loop {
                    if !(((((l_i_2) < (2i32)) as i32)) != 0) { break 'l1; }
                    'c2: {
                        { let _ = F::sprintf(c, fp.wrapping_add(28), sym::menu_q_b__str124, &[(((l_i_2).wrapping_add(2i32)) as i64 as u64)]); };
                        F::Flash_FindLabel(c, l_flash_1, sym::menu_q_b__str125, fp.wrapping_add(28), fp);
                        F::Flash_ClipSetTex(c, l_flash_1, fp, 0i32);
                    }
                    { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
                }
            }
            { let a5_ = { let a_ = (((c.m.r32(sym::gProgress)).wrapping_add(1612)).wrapping_add(4)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; F::Num_DrawChild(c, l_flash_1, sym::menu_q_b__str126, sym::menu_q_b__str127, 2i32, 2i32, a5_, 32i32, 32i32, 1i32, 0i32) };
        } else {
            {
                l_i_2 = 0i32;
                'l3: loop {
                    if !(((((l_i_2) < (2i32)) as i32)) != 0) { break 'l3; }
                    'c4: {
                        { let _ = F::sprintf(c, fp.wrapping_add(28), sym::menu_q_b__str128, &[(((l_i_2).wrapping_add(2i32)) as i64 as u64)]); };
                        F::Flash_FindLabel(c, l_flash_1, sym::menu_q_b__str129, fp.wrapping_add(28), fp);
                        F::Flash_ClipSetTex(c, l_flash_1, fp, 1i32);
                    }
                    { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
                }
            }
            { let a5_ = ({ let a_ = (((c.m.r32(sym::gProgress)).wrapping_add(1612)).wrapping_add(4)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_neg(); F::Num_DrawChild(c, l_flash_1, sym::menu_q_b__str130, sym::menu_q_b__str131, 2i32, 2i32, a5_, 32i32, 32i32, 1i32, 0i32) };
        }
        { let a5_ = { let a_ = ({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(2912); c.m.r32(a_) }).wrapping_add(((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1612); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(8)); (c.m.r32(a_) as i32) }; F::Num_DrawChild(c, l_flash_1, sym::menu_q_b__str132, sym::menu_q_b__str133, 0i32, 2i32, a5_, 32i32, 32i32, 1i32, 0i32) };
        if ((((({ let a_ = (((c.m.r32(sym::gProgress)).wrapping_add(1612)).wrapping_add(4)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) >= (0i32)) as i32)) != 0) {
            {
                l_i_2 = 0i32;
                'l5: loop {
                    if !(((((l_i_2) < (2i32)) as i32)) != 0) { break 'l5; }
                    'c6: {
                        { let _ = F::sprintf(c, fp.wrapping_add(28), sym::menu_q_b__str134, &[(((l_i_2).wrapping_add(2i32)) as i64 as u64)]); };
                        F::Flash_FindLabel(c, l_flash_1, sym::menu_q_b__str135, fp.wrapping_add(28), fp);
                        F::Flash_ClipSetTex(c, l_flash_1, fp, 0i32);
                    }
                    { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
                }
            }
            { let a5_ = { let a_ = (((c.m.r32(sym::gProgress)).wrapping_add(1612)).wrapping_add(4)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; F::Num_DrawChild(c, l_flash_1, sym::menu_q_b__str136, sym::menu_q_b__str137, 2i32, 2i32, a5_, 32i32, 32i32, 1i32, 0i32) };
        } else {
            {
                l_i_2 = 0i32;
                'l7: loop {
                    if !(((((l_i_2) < (2i32)) as i32)) != 0) { break 'l7; }
                    'c8: {
                        { let _ = F::sprintf(c, fp.wrapping_add(28), sym::menu_q_b__str138, &[(((l_i_2).wrapping_add(2i32)) as i64 as u64)]); };
                        F::Flash_FindLabel(c, l_flash_1, sym::menu_q_b__str139, fp.wrapping_add(28), fp);
                        F::Flash_ClipSetTex(c, l_flash_1, fp, 1i32);
                    }
                    { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
                }
            }
            { let a5_ = { let a_ = (((c.m.r32(sym::gProgress)).wrapping_add(1612)).wrapping_add(4)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; F::Num_DrawChild(c, l_flash_1, sym::menu_q_b__str140, sym::menu_q_b__str141, 2i32, 2i32, a5_, 32i32, 32i32, 1i32, 0i32) };
        }
        { let a5_ = { let a_ = ({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(2912); c.m.r32(a_) }).wrapping_add(((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1612); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(8)); (c.m.r32(a_) as i32) }; F::Num_DrawChild(c, l_flash_1, sym::menu_q_b__str142, sym::menu_q_b__str143, 0i32, 2i32, a5_, 32i32, 32i32, 1i32, 0i32) };
        { let a5_ = { let a_ = (((c.m.r32(sym::gProgress)).wrapping_add(1612)).wrapping_add(4)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; F::Num_DrawChild(c, l_flash_1, sym::menu_q_b__str144, sym::menu_q_b__str145, 2i32, 7i32, a5_, 32i32, 32i32, 1i32, 0i32) };
        l_flash_1 = ((c.m.r32(sym::gSimDay)).wrapping_add(32)).wrapping_add((((5i32) as u32)).wrapping_mul(44));
        { let a0_ = { let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1444); (c.m.r32(a_) as i32) }; F::SimDay_SetMenuAlpha(c, a0_) };
        {
            l_i_2 = 0i32;
            'l9: loop {
                if !(((((l_i_2) < (3i32)) as i32)) != 0) { break 'l9; }
                'c10: {
                    { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = (l_i_2).wrapping_shl((5i32) as u32); let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 512i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = ((l_i_2).wrapping_shl((5i32) as u32)).wrapping_add(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                    { let _ = F::sprintf(c, fp.wrapping_add(28), sym::menu_q_b__str146, &[(((l_i_2).wrapping_add(1i32)) as i64 as u64)]); };
                    F::Flash_FindLabel(c, l_flash_1, fp.wrapping_add(28), sym::menu_q_b__str147, fp);
                    F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(8));
                    if (((((((((l_i_2) == (1i32)) as i32)) != 0) && (((((F::SimDay_CountItems(c)) == (0i32)) as i32)) != 0)) as i32)) != 0) {
                        F::Flash_ClipSetColor(c, l_flash_1, fp, F32(0x3ecccccc));
                    }
                    F::Flash_FindLabel(c, l_flash_1, fp.wrapping_add(28), sym::menu_q_b__str148, fp);
                    F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(8));
                    if (((((((((l_i_2) == (1i32)) as i32)) != 0) && (((((F::SimDay_CountItems(c)) == (0i32)) as i32)) != 0)) as i32)) != 0) {
                        F::Flash_ClipSetColor(c, l_flash_1, fp, F32(0x3ecccccc));
                    }
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        l_n_3 = 0i32;
        {
            l_i_2 = 0i32;
            'l11: loop {
                if !(((((l_i_2) < (3i32)) as i32)) != 0) { break 'l11; }
                'c12: {
                    l_slot_5 = rt::rem_i32(({ let a_ = ((c.m.r32(sym::gProgress)).wrapping_add(1612)).wrapping_add(36); (c.m.r32(a_) as i32) }).wrapping_add(l_i_2), 3i32);
                    if ((((({ let a_ = (((c.m.r32(sym::gProgress)).wrapping_add(1612)).wrapping_add(24)).wrapping_add((((l_slot_5) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) != ((1i32).wrapping_neg())) as i32)) != 0) {
                        { let _ = F::sprintf(c, fp.wrapping_add(28), sym::menu_q_b__str149, &[(((l_n_3).wrapping_add(1i32)) as i64 as u64)]); };
                        F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), fp.wrapping_add(28), fp);
                        { let a4_ = { let a_ = (((c.m.r32(sym::gProgress)).wrapping_add(1612)).wrapping_add(24)).wrapping_add((((l_slot_5) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a5_ = ((c.m.r32(sym::gSimDay)).wrapping_add(940)).wrapping_add((((l_n_3) as u32)).wrapping_mul(140)); F::TextBox_AttachLine(c, l_flash_1, fp, 0i32, 0i32, a4_, a5_) };
                        { let o_ = l_n_3; let n_ = (o_).wrapping_add(1); l_n_3 = n_; };
                    }
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        l_flash_1 = ((c.m.r32(sym::gSimDay)).wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(44));
        's13: {
            let sv6: i32 = { let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(2960)).wrapping_add(24); (c.m.r32(a_) as i32) };
            match sv6 {
                0i32 | 1i32 => {
                    {
                        l_i_2 = 0i32;
                        'l14: loop {
                            if !(((((l_i_2) < (3i32)) as i32)) != 0) { break 'l14; }
                            'c15: {
                                { let _ = F::sprintf(c, fp.wrapping_add(28), sym::menu_q_b__str150, &[((l_i_2) as i64 as u64)]); };
                                F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), fp.wrapping_add(28), fp);
                                if ((((({ let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(2960)).wrapping_add(32); (c.m.r32(a_) as i32) }) == (l_i_2)) as i32)) != 0) {
                                    { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((1i32) as i64 as u64)) as u8)); };
                                    { let a0_ = ((c.m.r32(sym::gSimDay)).wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(44)); F::Flash_FindLabel(c, a0_, ((0i32) as u32), fp.wrapping_add(28), fp) };
                                    { let a0_ = ((c.m.r32(sym::gSimDay)).wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(44)); let a2_ = { let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(2960)).wrapping_add(24); (c.m.r32(a_) as i32) }; F::Flash_ClipSetTex(c, a0_, fp, a2_) };
                                    { let a0_ = ((c.m.r32(sym::gSimDay)).wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(44)); F::Flash_FindLabel(c, a0_, fp.wrapping_add(28), sym::menu_q_b__str151, fp) };
                                    { let a0_ = ((c.m.r32(sym::gSimDay)).wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(44)); let a2_ = { let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(2960)).wrapping_add(28); (c.m.r32(a_) as i32) }; F::Flash_ClipSetTex(c, a0_, fp, a2_) };
                                } else {
                                    { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
                                }
                            }
                            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
                        }
                    }
                    break 's13;
                }
                2i32 => {
                    F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_q_b__str152, fp);
                    { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 1024i32, ((((1i32) as i64 as u64)) as u8)); };
                    { let a5_ = { let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(2960)).wrapping_add(28); (c.m.r32(a_) as i32) }; F::Num_DrawChild(c, l_flash_1, sym::menu_q_b__str153, sym::menu_q_b__str154, 0i32, 5i32, a5_, 32i32, 32i32, 0i32, 0i32) };
                    if ((((({ let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(2960)).wrapping_add(28); (c.m.r32(a_) as i32) }) < (0i32)) as i32)) != 0) {
                        { let v_ = 32i32; let a_ = fp.wrapping_add(92); c.m.w32(a_, (v_) as u32); };
                        { let v_ = 0i32; let a_ = (fp.wrapping_add(92)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                        { let v_ = 64i32; let a_ = (fp.wrapping_add(92)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                        { let v_ = 32i32; let a_ = (fp.wrapping_add(92)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                        F::Flash_FindLabel(c, l_flash_1, sym::menu_q_b__str155, sym::menu_q_b__str156, fp);
                        F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(92));
                        { let v_ = 32i32; let a_ = fp.wrapping_add(92); c.m.w32(a_, (v_) as u32); };
                        { let v_ = 0i32; let a_ = (fp.wrapping_add(92)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                        { let v_ = 64i32; let a_ = (fp.wrapping_add(92)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                        { let v_ = 32i32; let a_ = (fp.wrapping_add(92)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                        F::Flash_FindLabel(c, l_flash_1, sym::menu_q_b__str157, sym::menu_q_b__str158, fp);
                        F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(92));
                        { let v_ = ({ let a_ = ({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(924); c.m.r32(a_) }).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(620)).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    } else {
                        { let v_ = ({ let a_ = ({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(924); c.m.r32(a_) }).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(620)).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    }
                    break 's13;
                }
                3i32 => {
                    F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_q_b__str159, fp);
                    { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 1024i32, ((((1i32) as i64 as u64)) as u8)); };
                    { let a5_ = ({ let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(2960)).wrapping_add(28); (c.m.r32(a_) as i32) }).wrapping_mul(100i32); F::Num_DrawChild(c, l_flash_1, sym::menu_q_b__str160, sym::menu_q_b__str161, 0i32, 9i32, a5_, 32i32, 32i32, 0i32, 0i32) };
                    if ((((({ let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(2960)).wrapping_add(28); (c.m.r32(a_) as i32) }) < (0i32)) as i32)) != 0) {
                        { let v_ = 32i32; let a_ = fp.wrapping_add(112); c.m.w32(a_, (v_) as u32); };
                        { let v_ = 0i32; let a_ = (fp.wrapping_add(112)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                        { let v_ = 64i32; let a_ = (fp.wrapping_add(112)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                        { let v_ = 32i32; let a_ = (fp.wrapping_add(112)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                        F::Flash_FindLabel(c, l_flash_1, sym::menu_q_b__str162, sym::menu_q_b__str163, fp);
                        F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(112));
                        { let v_ = 32i32; let a_ = fp.wrapping_add(112); c.m.w32(a_, (v_) as u32); };
                        { let v_ = 0i32; let a_ = (fp.wrapping_add(112)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                        { let v_ = 64i32; let a_ = (fp.wrapping_add(112)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                        { let v_ = 32i32; let a_ = (fp.wrapping_add(112)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                        F::Flash_FindLabel(c, l_flash_1, sym::menu_q_b__str164, sym::menu_q_b__str165, fp);
                        F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(112));
                        { let v_ = ({ let a_ = ({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(924); c.m.r32(a_) }).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(620)).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    } else {
                        { let v_ = ({ let a_ = ({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(924); c.m.r32(a_) }).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(620)).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    }
                    break 's13;
                }
                _ => {
                }
            }
        }
        if ((((({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1424); (c.m.r32(a_) as i32) }) == (10i32)) as i32)) != 0) {
            { let a0_ = ((c.m.r32(sym::gSimDay)).wrapping_add(32)).wrapping_add((((6i32) as u32)).wrapping_mul(44)); let a4_ = (rt::div_i32({ let a_ = ((c.m.r32(sym::gProgress)).wrapping_add(1612)).wrapping_add(40); (c.m.r32(a_) as i32) }, 10i32)).wrapping_add(1i32); F::Num_Draw(c, a0_, sym::menu_q_b__str166, 0i32, 2i32, a4_, 64i32, 64i32, 1i32) };
        }
        l_done_4 = 0i32;
        {
            l_i_2 = 0i32;
            'l16: loop {
                if !(((((l_i_2) < (7i32)) as i32)) != 0) { break 'l16; }
                'c17: {
                    { let a0_ = ((c.m.r32(sym::gSimDay)).wrapping_add(32)).wrapping_add((((l_i_2) as u32)).wrapping_mul(44)); F::Flash_Draw(c, a0_) };
                    if (((((((((l_i_2) == (4i32)) as i32)) != 0) && ((((!((l_done_4) != 0)) as i32)) != 0)) as i32)) != 0) {
                        l_done_4 = 1i32;
                        { let a2_ = { let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1440); (c.m.r32(a_) as i32) }; F::MsgWin_Draw(c, 0i32, 0i32, a2_) };
                    }
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        F::Dialog_Draw(c, 1i32);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "SimDay_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn SimDay_Update(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SimDay_Update", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let mut l_stat_2: i32 = 0;
    let mut l_moved_3: i32 = 0;
    let r_ = 'f: {
    if (((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(20); (c.m.r32(a_) as i32) }) & (256i32))) != 0) {
        break 'f;
    }
    if ((((({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1420); (c.m.r32(a_) as i32) }) > (0i32)) as i32)) != 0) {
        { let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1420); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
    }
    if (((({ let a_ = (((c.m.r32(sym::gSimDay)).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add(12); c.m.r32(a_) }) & (((32i32) as u32)))) != 0) {
        F::SimDay_Cmd(c, 4i32);
    }
    if (((({ let a_ = (((c.m.r32(sym::gSimDay)).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add(12); c.m.r32(a_) }) & (((1i32) as u32)))) != 0) {
        's1: {
            let sv4: i32 = { let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1424); (c.m.r32(a_) as i32) };
            match sv4 {
                0i32 => {
                    if ((((({ let a_ = ((c.m.r32(sym::gProgress)).wrapping_add(1612)).wrapping_add(48); (c.m.r32(a_) as i32) }) == (30i32)) as i32)) != 0) {
                        F::SimDay_SetButton(c, 0i32, 3i32);
                    }
                    if (((({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1360); (c.m.r32(a_) as i32) }) & (256i32))) != 0) {
                        { let r_ = (!(256i32)); let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                    } else {
                        if (((({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1360); (c.m.r32(a_) as i32) }) & (512i32))) != 0) {
                            { let r_ = (!(512i32)); let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                            if ((((({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1456); (c.m.r32(a_) as i32) }) >= (5i32)) as i32)) != 0) {
                                { let v_ = (({ let a_ = ((c.m.r32(sym::gProgress)).wrapping_add(1612)).wrapping_add(40); (c.m.r32(a_) as i32) }).wrapping_sub({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1456); (c.m.r32(a_) as i32) })).wrapping_add(9i32); let a_ = ((c.m.r32(sym::gProgress)).wrapping_add(1612)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
                            } else {
                                { let r_ = 5i32; let a_ = ((c.m.r32(sym::gProgress)).wrapping_add(1612)).wrapping_add(40); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                            }
                            F::SimDay_PassWait(c, 5i32);
                        } else {
                            { let a_ = ((c.m.r32(sym::gProgress)).wrapping_add(1612)).wrapping_add(40); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                            F::SimDay_PassWait(c, 1i32);
                        }
                    }
                    { let v_ = rt::rem_i32({ let a_ = ((c.m.r32(sym::gProgress)).wrapping_add(1612)).wrapping_add(40); (c.m.r32(a_) as i32) }, 10i32); let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1456); c.m.w32(a_, (v_) as u32); };
                    F::SimDay_Cmd(c, 8i32);
                    F::SimDay_Cmd(c, 12i32);
                    F::SimDay_SetFaceA(c, 0i32);
                    { let v_ = ({ let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(1368)).wrapping_add(((({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1424); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(80i32); let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                    {
                        l_i_1 = 0i32;
                        'l2: loop {
                            if !(((((l_i_1) < (3i32)) as i32)) != 0) { break 'l2; }
                            'c3: {
                                { let v_ = F::SimDay_GetTrainRank(c, l_i_1); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(2944)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                            }
                            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
                        }
                    }
                    break 's1;
                }
                3i32 => {
                    { let a0_ = { let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(2936); (c.m.r32(a_) as i32) }; F::SimDay_SetFaceA(c, a0_) };
                    { let a0_ = { let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(2940); (c.m.r32(a_) as i32) }; F::SimDay_SetFaceB(c, a0_) };
                    break 's1;
                }
                5i32 => {
                    F::SimDay_SetFaceA(c, 12i32);
                    F::SimDay_Cmd(c, 10i32);
                    { let v_ = 88i32; let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                    break 's1;
                }
                _ => {
                }
            }
        }
        { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 18i32); };
    }
    if (((({ let a_ = (((c.m.r32(sym::gSimDay)).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add(12); c.m.r32(a_) }) & (((4i32) as u32)))) != 0) {
        { let r_ = (!(64i32)); let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
    }
    if (((({ let a_ = (((c.m.r32(sym::gSimDay)).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add(12); c.m.r32(a_) }) & (((8i32) as u32)))) != 0) {
        F::SimDay_Cmd(c, 9i32);
    }
    if (((({ let a_ = (((c.m.r32(sym::gSimDay)).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add(12); c.m.r32(a_) }) & (((2i32) as u32)))) != 0) {
        F::SimDay_Cmd(c, 11i32);
    }
    if (((({ let a_ = (((c.m.r32(sym::gSimDay)).wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(44))).wrapping_add(12); c.m.r32(a_) }) & (((2i32) as u32)))) != 0) {
        's4: {
            let sv5: i32 = { let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(2960)).wrapping_add(24); (c.m.r32(a_) as i32) };
            match sv5 {
                0i32 | 1i32 => {
                    F::SimDay_Cmd(c, 28i32);
                    { let r_ = ({ let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(2960)).wrapping_add(28); (c.m.r32(a_) as i32) }).wrapping_sub(2i32); let a_ = (((c.m.r32(sym::gProgress)).wrapping_add(1612)).wrapping_add(4)).wrapping_add(((({ let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(2960)).wrapping_add(24); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                    if ((((({ let a_ = (((c.m.r32(sym::gProgress)).wrapping_add(1612)).wrapping_add(4)).wrapping_add(((({ let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(2960)).wrapping_add(24); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) < (0i32)) as i32)) != 0) {
                        { let v_ = 0i32; let a_ = (((c.m.r32(sym::gProgress)).wrapping_add(1612)).wrapping_add(4)).wrapping_add(((({ let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(2960)).wrapping_add(24); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    }
                    if ((((({ let a_ = ({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(2912); c.m.r32(a_) }).wrapping_add(((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1612); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(8)); (c.m.r32(a_) as i32) }) < ({ let a_ = (((c.m.r32(sym::gProgress)).wrapping_add(1612)).wrapping_add(4)).wrapping_add(((({ let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(2960)).wrapping_add(24); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                        { let v_ = { let a_ = ({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(2912); c.m.r32(a_) }).wrapping_add(((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1612); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(8)); (c.m.r32(a_) as i32) }; let a_ = (((c.m.r32(sym::gProgress)).wrapping_add(1612)).wrapping_add(4)).wrapping_add(((({ let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(2960)).wrapping_add(24); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    }
                    if (((((({ let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(2960)).wrapping_add(28); (c.m.r32(a_) as i32) }).wrapping_sub(2i32)) < (0i32)) as i32)) != 0) {
                        { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 37i32); };
                    } else {
                        { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 36i32); };
                    }
                    break 's4;
                }
                2i32 => {
                    if ((((({ let a_ = (((c.m.r32(sym::gSimDay)).wrapping_add(2960)).wrapping_add(8)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) < (0i32)) as i32)) != 0) {
                        { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 27i32); };
                    } else {
                        { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 26i32); };
                    }
                    { let v_ = 1i32; let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(2960)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    break 's4;
                }
                3i32 => {
                    { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 28i32); };
                    { let v_ = 1i32; let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(2960)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    break 's4;
                }
                _ => {
                }
            }
        }
    }
    if (((({ let a_ = (((c.m.r32(sym::gSimDay)).wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(44))).wrapping_add(12); c.m.r32(a_) }) & (((1i32) as u32)))) != 0) {
        if (((((F::SimDay_NextChange(c)) == ((1i32).wrapping_neg())) as i32)) != 0) {
            { let v_ = 3i32; let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1424); c.m.w32(a_, (v_) as u32); };
        } else {
            F::SimDay_Cmd(c, 26i32);
        }
    }
    if (((({ let a_ = (((c.m.r32(sym::gSimDay)).wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(44))).wrapping_add(12); c.m.r32(a_) }) & (((4i32) as u32)))) != 0) {
        { let v_ = 3i32; let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1424); c.m.w32(a_, (v_) as u32); };
    }
    if (((({ let a_ = (((c.m.r32(sym::gSimDay)).wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(44))).wrapping_add(12); c.m.r32(a_) }) & (((8i32) as u32)))) != 0) {
        { let r_ = (!(16i32)); let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(2960); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
        F::SimDay_Cmd(c, 0i32);
        if (((({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1360); (c.m.r32(a_) as i32) }) & (1024i32))) != 0) {
            if (({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(3000); (c.m.r32(a_) as i32) }) != 0) {
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1424); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(3000); c.m.w32(a_, (v_) as u32); };
                { let v_ = ({ let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(1368)).wrapping_add(((({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1424); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(80i32); let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
            } else {
                { let v_ = 3i32; let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1424); c.m.w32(a_, (v_) as u32); };
            }
            { let r_ = (!(1024i32)); let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
        }
    }
    if (((({ let a_ = (((c.m.r32(sym::gSimDay)).wrapping_add(32)).wrapping_add((((5i32) as u32)).wrapping_mul(44))).wrapping_add(12); c.m.r32(a_) }) & (((1i32) as u32)))) != 0) {
        if ((((({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1424); (c.m.r32(a_) as i32) }) == (6i32)) as i32)) != 0) {
            { let v_ = 0i32; let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1444); c.m.w32(a_, (v_) as u32); };
        } else {
            { let v_ = 1i32; let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1444); c.m.w32(a_, (v_) as u32); };
        }
    }
    if (((({ let a_ = (((c.m.r32(sym::gSimDay)).wrapping_add(32)).wrapping_add((((5i32) as u32)).wrapping_mul(44))).wrapping_add(8); c.m.r32(a_) }) & (((8i32) as u32)))) != 0) {
        if ((((({ let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(1368)).wrapping_add(((({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1424); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) == (2i32)) as i32)) != 0) {
            { let v_ = 8i32; let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1424); c.m.w32(a_, (v_) as u32); };
            F::Dialog_SetChoices(c, 1i32);
            F::Dialog_SetCursor(c, 1i32);
            F::Dialog_Start(c, 0i32);
        } else {
            { let v_ = { let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1428); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1424); c.m.w32(a_, (v_) as u32); };
            { let r_ = (!(8192i32)); let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
        }
    }
    if (({ let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(2960)).wrapping_add(4); (c.m.r32(a_) as i32) }) != 0) {
        l_stat_2 = { let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(2960)).wrapping_add(24); (c.m.r32(a_) as i32) };
        if ((((({ let a_ = (((c.m.r32(sym::gSimDay)).wrapping_add(2960)).wrapping_add(8)).wrapping_add((((l_stat_2) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) > (0i32)) as i32)) != 0) {
            's5: {
                let sv6: i32 = l_stat_2;
                match sv6 {
                    2i32 => {
                        if ((((({ let a_ = (((c.m.r32(sym::gSimDay)).wrapping_add(2960)).wrapping_add(8)).wrapping_add((((l_stat_2) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) >= (2i32)) as i32)) != 0) {
                            { let a_ = (((c.m.r32(sym::gProgress)).wrapping_add(1612)).wrapping_add(4)).wrapping_add((((l_stat_2) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                            { let a_ = (((c.m.r32(sym::gSimDay)).wrapping_add(2960)).wrapping_add(8)).wrapping_add((((l_stat_2) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                        } else {
                            { let r_ = { let a_ = (((c.m.r32(sym::gSimDay)).wrapping_add(2960)).wrapping_add(8)).wrapping_add((((l_stat_2) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (((c.m.r32(sym::gProgress)).wrapping_add(1612)).wrapping_add(4)).wrapping_add((((l_stat_2) as u32)).wrapping_mul(4)); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                            { let v_ = 0i32; let a_ = (((c.m.r32(sym::gSimDay)).wrapping_add(2960)).wrapping_add(8)).wrapping_add((((l_stat_2) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                        }
                        if ((((({ let a_ = (((c.m.r32(sym::gProgress)).wrapping_add(1612)).wrapping_add(4)).wrapping_add((((l_stat_2) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) > (100i32)) as i32)) != 0) {
                            { let v_ = 100i32; let a_ = (((c.m.r32(sym::gProgress)).wrapping_add(1612)).wrapping_add(4)).wrapping_add((((l_stat_2) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                        }
                        if ((((({ let a_ = (((c.m.r32(sym::gProgress)).wrapping_add(1612)).wrapping_add(4)).wrapping_add((((l_stat_2) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) > (20i32)) as i32)) != 0) {
                            if (((({ let a_ = ((c.m.r32(sym::gProgress)).wrapping_add(1612)).wrapping_add(52); (c.m.r32(a_) as i32) }) & (1i32))) != 0) {
                                F::SimDay_SetButton(c, 1i32, 0i32);
                            }
                            if (((({ let a_ = ((c.m.r32(sym::gProgress)).wrapping_add(1612)).wrapping_add(52); (c.m.r32(a_) as i32) }) & (2i32))) != 0) {
                                F::SimDay_SetButton(c, 1i32, 1i32);
                            }
                            if ((((((({ let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(1368)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)) < (((2i32) as u32))) as i32)) != 0) {
                                F::SimDay_ClipGoto(c, 0i32, 0i32, sym::menu_q_b__str167);
                            }
                        }
                        break 's5;
                    }
                    3i32 => {
                        if ((((({ let a_ = (((c.m.r32(sym::gSimDay)).wrapping_add(2960)).wrapping_add(8)).wrapping_add((((l_stat_2) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) >= (8i32)) as i32)) != 0) {
                            { let r_ = 7i32; let a_ = (((c.m.r32(sym::gProgress)).wrapping_add(1612)).wrapping_add(4)).wrapping_add((((l_stat_2) as u32)).wrapping_mul(4)); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                            { let r_ = 7i32; let a_ = (((c.m.r32(sym::gSimDay)).wrapping_add(2960)).wrapping_add(8)).wrapping_add((((l_stat_2) as u32)).wrapping_mul(4)); let n_ = ((c.m.r32(a_) as i32)).wrapping_sub(r_); c.m.w32(a_, (n_) as u32); };
                        } else {
                            { let r_ = { let a_ = (((c.m.r32(sym::gSimDay)).wrapping_add(2960)).wrapping_add(8)).wrapping_add((((l_stat_2) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (((c.m.r32(sym::gProgress)).wrapping_add(1612)).wrapping_add(4)).wrapping_add((((l_stat_2) as u32)).wrapping_mul(4)); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                            { let v_ = 0i32; let a_ = (((c.m.r32(sym::gSimDay)).wrapping_add(2960)).wrapping_add(8)).wrapping_add((((l_stat_2) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                        }
                        if ((((({ let a_ = (((c.m.r32(sym::gProgress)).wrapping_add(1612)).wrapping_add(4)).wrapping_add((((l_stat_2) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) > (65535i32)) as i32)) != 0) {
                            { let v_ = 65535i32; let a_ = (((c.m.r32(sym::gProgress)).wrapping_add(1612)).wrapping_add(4)).wrapping_add((((l_stat_2) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                        }
                        break 's5;
                    }
                    _ => {
                    }
                }
            }
            if (((((l_stat_2) == (3i32)) as i32)) != 0) {
                if ((((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1612); (c.m.r32(a_) as i32) }) < ((6i32).wrapping_add(1i32))) as i32)) != 0) {
                    if ((((({ let a_ = (({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(2912); c.m.r32(a_) }).wrapping_add((((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1612); (c.m.r32(a_) as i32) }).wrapping_add(1i32)) as u32)).wrapping_mul(8))).wrapping_add(4); (c.m.r32(a_) as i32) }) < ({ let a_ = (((c.m.r32(sym::gProgress)).wrapping_add(1612)).wrapping_add(4)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                        F::SimDay_Cmd(c, 33i32);
                    }
                }
            }
        } else {
            if ((((({ let a_ = (((c.m.r32(sym::gSimDay)).wrapping_add(2960)).wrapping_add(8)).wrapping_add((((l_stat_2) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) < (0i32)) as i32)) != 0) {
                's6: {
                    let sv7: i32 = l_stat_2;
                    match sv7 {
                        2i32 => {
                            if ((((({ let a_ = (((c.m.r32(sym::gSimDay)).wrapping_add(2960)).wrapping_add(8)).wrapping_add((((l_stat_2) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) < ((1i32).wrapping_neg())) as i32)) != 0) {
                                { let a_ = (((c.m.r32(sym::gProgress)).wrapping_add(1612)).wrapping_add(4)).wrapping_add((((l_stat_2) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                                { let a_ = (((c.m.r32(sym::gSimDay)).wrapping_add(2960)).wrapping_add(8)).wrapping_add((((l_stat_2) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                            } else {
                                { let r_ = { let a_ = (((c.m.r32(sym::gSimDay)).wrapping_add(2960)).wrapping_add(8)).wrapping_add((((l_stat_2) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (((c.m.r32(sym::gProgress)).wrapping_add(1612)).wrapping_add(4)).wrapping_add((((l_stat_2) as u32)).wrapping_mul(4)); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                                { let v_ = 0i32; let a_ = (((c.m.r32(sym::gSimDay)).wrapping_add(2960)).wrapping_add(8)).wrapping_add((((l_stat_2) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                            }
                            if ((((({ let a_ = (((c.m.r32(sym::gProgress)).wrapping_add(1612)).wrapping_add(4)).wrapping_add((((l_stat_2) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) <= (0i32)) as i32)) != 0) {
                                { let v_ = 1i32; let a_ = (((c.m.r32(sym::gProgress)).wrapping_add(1612)).wrapping_add(4)).wrapping_add((((l_stat_2) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                            }
                            if ((((({ let a_ = (((c.m.r32(sym::gProgress)).wrapping_add(1612)).wrapping_add(4)).wrapping_add((((l_stat_2) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) < (20i32)) as i32)) != 0) {
                                { let v_ = 20i32; let a_ = (((c.m.r32(sym::gProgress)).wrapping_add(1612)).wrapping_add(4)).wrapping_add((((l_stat_2) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                            }
                            if ((((({ let a_ = (((c.m.r32(sym::gProgress)).wrapping_add(1612)).wrapping_add(4)).wrapping_add((((l_stat_2) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) < (21i32)) as i32)) != 0) {
                                l_moved_3 = 0i32;
                                if ((((!(((({ let a_ = ((c.m.r32(sym::gProgress)).wrapping_add(1612)).wrapping_add(52); (c.m.r32(a_) as i32) }) & (1i32))) != 0)) as i32)) != 0) {
                                    F::SimDay_SetButton(c, 0i32, 0i32);
                                    l_moved_3 = 1i32;
                                }
                                if ((((!(((({ let a_ = ((c.m.r32(sym::gProgress)).wrapping_add(1612)).wrapping_add(52); (c.m.r32(a_) as i32) }) & (2i32))) != 0)) as i32)) != 0) {
                                    F::SimDay_SetButton(c, 0i32, 1i32);
                                    l_moved_3 = 1i32;
                                }
                                if ((l_moved_3) != 0) {
                                    { let v_ = 2i32; let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(1368)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                    F::SimDay_ClipGoto(c, 0i32, 0i32, sym::menu_q_b__str168);
                                }
                            }
                            break 's6;
                        }
                        3i32 => {
                            if ((((({ let a_ = (((c.m.r32(sym::gSimDay)).wrapping_add(2960)).wrapping_add(8)).wrapping_add((((l_stat_2) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) < ((7i32).wrapping_neg())) as i32)) != 0) {
                                { let r_ = 7i32; let a_ = (((c.m.r32(sym::gProgress)).wrapping_add(1612)).wrapping_add(4)).wrapping_add((((l_stat_2) as u32)).wrapping_mul(4)); let n_ = ((c.m.r32(a_) as i32)).wrapping_sub(r_); c.m.w32(a_, (n_) as u32); };
                                { let r_ = 7i32; let a_ = (((c.m.r32(sym::gSimDay)).wrapping_add(2960)).wrapping_add(8)).wrapping_add((((l_stat_2) as u32)).wrapping_mul(4)); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                            } else {
                                { let r_ = { let a_ = (((c.m.r32(sym::gSimDay)).wrapping_add(2960)).wrapping_add(8)).wrapping_add((((l_stat_2) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (((c.m.r32(sym::gProgress)).wrapping_add(1612)).wrapping_add(4)).wrapping_add((((l_stat_2) as u32)).wrapping_mul(4)); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                                { let v_ = 0i32; let a_ = (((c.m.r32(sym::gSimDay)).wrapping_add(2960)).wrapping_add(8)).wrapping_add((((l_stat_2) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                            }
                            if ((((({ let a_ = (((c.m.r32(sym::gProgress)).wrapping_add(1612)).wrapping_add(4)).wrapping_add((((l_stat_2) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) < (0i32)) as i32)) != 0) {
                                { let v_ = 0i32; let a_ = (((c.m.r32(sym::gProgress)).wrapping_add(1612)).wrapping_add(4)).wrapping_add((((l_stat_2) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                            }
                            break 's6;
                        }
                        _ => {
                        }
                    }
                }
            }
        }
        if ((((((((({ let a_ = (((c.m.r32(sym::gSimDay)).wrapping_add(2960)).wrapping_add(8)).wrapping_add((((l_stat_2) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) && ((((!(((({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(2960); (c.m.r32(a_) as i32) }) & (16i32))) != 0)) as i32)) != 0)) as i32)) != 0) {
            's7: {
                let sv8: i32 = l_stat_2;
                match sv8 {
                    2i32 => {
                        F::SimDay_Cmd(c, 30i32);
                        break 's7;
                    }
                    3i32 => {
                        F::SimDay_Cmd(c, 32i32);
                        break 's7;
                    }
                    _ => {
                    }
                }
            }
            { let v_ = 0i32; let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(2960)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        }
    }
    {
        l_i_1 = 0i32;
        'l8: loop {
            if !(((((l_i_1) < (7i32)) as i32)) != 0) { break 'l8; }
            'c9: {
                if ((((((((!(((({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1360); (c.m.r32(a_) as i32) }) & (8192i32))) != 0)) as i32)) != 0) || (((((l_i_1) == (5i32)) as i32)) != 0)) as i32)) != 0) {
                    { let a0_ = ((c.m.r32(sym::gSimDay)).wrapping_add(32)).wrapping_add((((l_i_1) as u32)).wrapping_mul(44)); F::Flash_Advance(c, a0_) };
                }
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "SimDay_Update", &PROBE_N, &[], true); }
    r_
}

pub fn SimDay_UpdateTalk(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SimDay_UpdateTalk", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if ((((({ let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(1432)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
        break 'f;
    }
    if (((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(20); (c.m.r32(a_) as i32) }) & (256i32))) != 0) {
        break 'f;
    }
    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
        { let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(1432)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
    } else {
        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1024i32) as u32)))) != 0) {
            { let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(1432)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
            { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 2i32); };
        } else {
            if ((((((((((((({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1440); (c.m.r32(a_) as i32) }) != ((1i32).wrapping_neg())) as i32)) != 0) && (((((F::Voice_GetStat(c, 0i32)) != (5i32)) as i32)) != 0)) as i32)) != 0) && ((((({ let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(1432)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) == ({ let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(1432)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) })) as i32)) != 0)) as i32)) != 0) {
                break 'f;
            }
        }
    }
    's1: {
        let sv1: i32 = { let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(1432)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
        match sv1 {
            1i32 => {
                { let v_ = 25i32; let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            2i32 => {
                { let v_ = 26i32; let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            3i32 => {
                { let v_ = 27i32; let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            4i32 => {
                { let v_ = 28i32; let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            5i32 => {
                { let v_ = 29i32; let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            _ => {
            }
        }
    }
    { let a2_ = { let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1440); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, ((0i32) as u32), 34661i32, a2_) };
    { let v_ = 0i32; let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(1432)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    { let v_ = (1i32).wrapping_neg(); let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
    { let v_ = { let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(1432)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(1432)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "SimDay_UpdateTalk", &PROBE_N, &[], true); }
    r_
}

pub fn SimDay_Input(c: &mut Ctx, mut p_result_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SimDay_Input", &PROBE_N, &[(p_result_1) as u32], false); }
    let mut l_ok_2: i32 = 0;
    let mut l_cancel_3: i32 = 0;
    let mut l_up_4: i32 = 0;
    let mut l_down_5: i32 = 0;
    let mut l_start_6: i32 = 0;
    let mut l_row_7: i32 = 0;
    let mut l_answer_8: i32 = 0;
    let r_ = 'f: {
    l_ok_2 = ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) as i32);
    l_cancel_3 = ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1024i32) as u32)))) as i32);
    l_up_4 = ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((8i32) as u32)))) as i32);
    l_down_5 = ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((4i32) as u32)))) as i32);
    l_start_6 = ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((4096i32) as u32)))) as i32);
    if ((((!(((({ let a_ = (((c.m.r32(sym::gSimDay)).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add(8); c.m.r32(a_) }) & (((2i32) as u32)))) != 0)) as i32)) != 0) {
        break 'f;
    }
    if ((((((((({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1424); (c.m.r32(a_) as i32) }) == (6i32)) as i32)) != 0) || ((((({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1424); (c.m.r32(a_) as i32) }) == (7i32)) as i32)) != 0)) as i32)) != 0) {
        if ((((!(((({ let a_ = (((c.m.r32(sym::gSimDay)).wrapping_add(32)).wrapping_add((((5i32) as u32)).wrapping_mul(44))).wrapping_add(8); c.m.r32(a_) }) & (((2i32) as u32)))) != 0)) as i32)) != 0) {
            break 'f;
        }
    }
    if (((({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1360); (c.m.r32(a_) as i32) }) & (64i32))) != 0) {
        break 'f;
    }
    if ((((!(((({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1360); (c.m.r32(a_) as i32) }) & (4i32))) != 0)) as i32)) != 0) {
        F::SimDay_RefreshBoard(c);
        { let r_ = 4i32; let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
    if ((((({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1424); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
        if ((l_start_6) != 0) {
            F::SimDay_Cmd(c, 20i32);
            break 'f;
        }
    }
    's1: {
        let sv9: i32 = { let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1424); (c.m.r32(a_) as i32) };
        match sv9 {
            0i32 => {
                if ((l_up_4) != 0) {
                    F::SimDay_ClipGoto(c, 0i32, 0i32, sym::menu_q_b__str169);
                    'l2: loop {
                        'c3: {
                            { let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(1368)).wrapping_add(((({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1424); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                            if ((((({ let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(1368)).wrapping_add(((({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1424); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) < (0i32)) as i32)) != 0) {
                                { let v_ = 3i32; let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(1368)).wrapping_add(((({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1424); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                            }
                        }
                        if !((((({ let a_ = ((c.m.r32(sym::gProgress)).wrapping_add(1612)).wrapping_add(52); (c.m.r32(a_) as i32) }).wrapping_shr(({ let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(1368)).wrapping_add(((({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1424); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)) & (1i32))) != 0) { break 'l2; }
                    }
                    F::SimDay_ClipGoto(c, 0i32, 0i32, sym::menu_q_b__str170);
                    { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                    { let v_ = ({ let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(1368)).wrapping_add(((({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1424); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(80i32); let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                } else {
                    if ((l_down_5) != 0) {
                        F::SimDay_ClipGoto(c, 0i32, 0i32, sym::menu_q_b__str171);
                        'l4: loop {
                            'c5: {
                                { let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(1368)).wrapping_add(((({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1424); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                                if ((((({ let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(1368)).wrapping_add(((({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1424); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) >= (4i32)) as i32)) != 0) {
                                    { let v_ = 0i32; let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(1368)).wrapping_add(((({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1424); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                }
                            }
                            if !((((({ let a_ = ((c.m.r32(sym::gProgress)).wrapping_add(1612)).wrapping_add(52); (c.m.r32(a_) as i32) }).wrapping_shr(({ let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(1368)).wrapping_add(((({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1424); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)) & (1i32))) != 0) { break 'l4; }
                        }
                        F::SimDay_ClipGoto(c, 0i32, 0i32, sym::menu_q_b__str172);
                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                        { let v_ = ({ let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(1368)).wrapping_add(((({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1424); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(80i32); let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                    } else {
                        if ((l_ok_2) != 0) {
                            if ((((({ let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(1368)).wrapping_add(((({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1424); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
                                's6: {
                                    let sv10: i32 = { let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(1368)).wrapping_add(((({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1424); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
                                    match sv10 {
                                        1i32 => {
                                            F::SimDay_PickEvent(c, 0i32);
                                            break 's6;
                                        }
                                        2i32 => {
                                            F::SimDay_PickEvent(c, 0i32);
                                            break 's6;
                                        }
                                        3i32 => {
                                            { let v_ = 30i32; let a_ = ((c.m.r32(sym::gProgress)).wrapping_add(1612)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
                                            F::SimDay_PickEvent(c, 0i32);
                                            { let v_ = 0i32; let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(1368)).wrapping_add(((({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1424); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                            F::SimDay_ClipGoto(c, 0i32, 0i32, sym::menu_q_b__str173);
                                            break 's6;
                                        }
                                        _ => {
                                        }
                                    }
                                }
                                F::SimDay_Cmd(c, 13i32);
                                { let v_ = 3i32; let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1424); c.m.w32(a_, (v_) as u32); };
                            } else {
                                { let v_ = 1i32; let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1424); c.m.w32(a_, (v_) as u32); };
                                F::SimDay_Cmd(c, 14i32);
                                { let v_ = ({ let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(1368)).wrapping_add(((({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1424); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(84i32); let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                            }
                            { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                        }
                    }
                }
                break 's1;
            }
            1i32 => {
                if ((l_up_4) != 0) {
                    F::SimDay_ClipGoto(c, 0i32, 1i32, sym::menu_q_b__str174);
                    if ((((({ let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(1368)).wrapping_add(((({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1424); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) < (0i32)) as i32)) != 0) {
                        { let v_ = 2i32; let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(1368)).wrapping_add(((({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1424); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    }
                    F::SimDay_ClipGoto(c, 0i32, 1i32, sym::menu_q_b__str175);
                    { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                    { let v_ = ({ let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(1368)).wrapping_add(((({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1424); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(84i32); let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                } else {
                    if ((l_down_5) != 0) {
                        F::SimDay_ClipGoto(c, 0i32, 1i32, sym::menu_q_b__str176);
                        if ((((({ let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(1368)).wrapping_add(((({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1424); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); n_ }) >= (3i32)) as i32)) != 0) {
                            { let v_ = 0i32; let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(1368)).wrapping_add(((({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1424); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                        }
                        F::SimDay_ClipGoto(c, 0i32, 1i32, sym::menu_q_b__str177);
                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                        { let v_ = ({ let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(1368)).wrapping_add(((({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1424); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(84i32); let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                    } else {
                        if ((l_ok_2) != 0) {
                            { let v_ = 3i32; let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1424); c.m.w32(a_, (v_) as u32); };
                            F::SimDay_Cmd(c, 15i32);
                            F::SimDay_PickEvent(c, 1i32);
                            { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                        } else {
                            if ((l_cancel_3) != 0) {
                                F::SimDay_Cmd(c, 16i32);
                                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 2i32); };
                                { let v_ = ({ let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(1368)).wrapping_add(((({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1424); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(80i32); let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                            }
                        }
                    }
                }
                break 's1;
            }
            6i32 => {
                if ((l_up_4) != 0) {
                    F::SimDay_ClipGoto(c, 5i32, 6i32, sym::menu_q_b__str178);
                    if ((((({ let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(1368)).wrapping_add(((({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1424); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) < (0i32)) as i32)) != 0) {
                        { let v_ = 2i32; let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(1368)).wrapping_add(((({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1424); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    }
                    if ((((((((({ let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(1368)).wrapping_add(((({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1424); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) == (1i32)) as i32)) != 0) && ((((!((F::SimDay_CountItems(c)) != 0)) as i32)) != 0)) as i32)) != 0) {
                        { let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(1368)).wrapping_add(((({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1424); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                    }
                    F::SimDay_ClipGoto(c, 5i32, 6i32, sym::menu_q_b__str179);
                    { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                } else {
                    if ((l_down_5) != 0) {
                        F::SimDay_ClipGoto(c, 5i32, 6i32, sym::menu_q_b__str180);
                        if ((((({ let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(1368)).wrapping_add(((({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1424); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); n_ }) >= (3i32)) as i32)) != 0) {
                            { let v_ = 0i32; let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(1368)).wrapping_add(((({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1424); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                        }
                        if ((((((((({ let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(1368)).wrapping_add(((({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1424); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) == (1i32)) as i32)) != 0) && ((((!((F::SimDay_CountItems(c)) != 0)) as i32)) != 0)) as i32)) != 0) {
                            { let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(1368)).wrapping_add(((({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1424); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        }
                        F::SimDay_ClipGoto(c, 5i32, 6i32, sym::menu_q_b__str181);
                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                    } else {
                        if ((l_ok_2) != 0) {
                            l_row_7 = { let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(1368)).wrapping_add(((({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1424); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
                            if (((((l_row_7) == (1i32)) as i32)) != 0) {
                                if ((F::SimDay_CountItems(c)) != 0) {
                                    F::SimDay_Cmd(c, 22i32);
                                    { let v_ = 7i32; let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1424); c.m.w32(a_, (v_) as u32); };
                                    { let v_ = l_row_7; let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1444); c.m.w32(a_, (v_) as u32); };
                                    { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                                } else {
                                    { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 7i32); };
                                }
                            } else {
                                F::SimDay_Cmd(c, 21i32);
                                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                            }
                        } else {
                            if ((l_start_6) != 0) {
                                { let v_ = 0i32; let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(1368)).wrapping_add(((({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1424); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                F::SimDay_Cmd(c, 21i32);
                            }
                        }
                    }
                }
                break 's1;
            }
            7i32 => {
                if ((l_ok_2) != 0) {
                    { let v_ = 0i32; let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1444); c.m.w32(a_, (v_) as u32); };
                    F::SimDay_Cmd(c, 23i32);
                    { let v_ = 6i32; let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1424); c.m.w32(a_, (v_) as u32); };
                }
                break 's1;
            }
            8i32 => {
                {
                    F::Dialog_SetMsg(c, 14i32);
                    l_answer_8 = F::Dialog_Input(c, 0i32);
                    if (((((l_answer_8) > (0i32)) as i32)) != 0) {
                        { let v_ = 12i32; let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1424); c.m.w32(a_, (v_) as u32); };
                        F::Dialog_SetChoices(c, 0i32);
                        F::Dialog_Start(c, 1i32);
                    } else {
                        if (((((l_answer_8) < (0i32)) as i32)) != 0) {
                            { let v_ = 9i32; let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1424); c.m.w32(a_, (v_) as u32); };
                            F::Dialog_SetChoices(c, 0i32);
                            F::Dialog_Start(c, 1i32);
                        }
                    }
                    break 's1;
                }
            }
            9i32 => {
                if ((F::Dialog_IsClosed(c)) != 0) {
                    F::SimDay_Cmd(c, 20i32);
                }
                break 's1;
            }
            2i32 => {
                if ((l_up_4) != 0) {
                    F::SimDay_ClipGoto(c, 0i32, 2i32, sym::menu_q_b__str182);
                    if ((((({ let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(1368)).wrapping_add(((({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1424); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) < (0i32)) as i32)) != 0) {
                        { let v_ = 1i32; let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(1368)).wrapping_add(((({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1424); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    }
                    F::SimDay_ClipGoto(c, 0i32, 2i32, sym::menu_q_b__str183);
                    { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                } else {
                    if ((l_down_5) != 0) {
                        F::SimDay_ClipGoto(c, 0i32, 2i32, sym::menu_q_b__str184);
                        if ((((({ let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(1368)).wrapping_add(((({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1424); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); n_ }) >= (2i32)) as i32)) != 0) {
                            { let v_ = 0i32; let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(1368)).wrapping_add(((({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1424); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                        }
                        F::SimDay_ClipGoto(c, 0i32, 2i32, sym::menu_q_b__str185);
                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                    } else {
                        if ((l_ok_2) != 0) {
                            if ((((({ let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(1368)).wrapping_add(((({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1424); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
                                { let v_ = 1i32; let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(2956); c.m.w32(a_, (v_) as u32); };
                            } else {
                                { let v_ = 0i32; let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(2956); c.m.w32(a_, (v_) as u32); };
                            }
                            F::SimDay_Cmd(c, 18i32);
                            { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                        }
                    }
                }
                break 's1;
            }
            3i32 => {
                if (((({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1360); (c.m.r32(a_) as i32) }) & (128i32))) != 0) {
                    if ((l_ok_2) != 0) {
                        { let r_ = (!(128i32)); let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                    }
                } else {
                    if (({ let a0_ = c.m.r32(sym::gSimDay); let a1_ = (({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(2916); (c.m.r32(a_) as i32) }) as u32); F::SimEvent_Run(c, a0_, a1_) }) != 0) {
                        if ((((({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1456); (c.m.r32(a_) as i32) }) == ((10i32).wrapping_sub(1i32))) as i32)) != 0) {
                            { let v_ = 5i32; let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1424); c.m.w32(a_, (v_) as u32); };
                            F::SimDay_SetupBattle(c);
                            { let v_ = 1i32; let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1364); c.m.w32(a_, (v_) as u32); };
                        } else {
                            { let v_ = 0i32; let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1424); c.m.w32(a_, (v_) as u32); };
                        }
                        F::SimDay_ListItems(c);
                        F::SimDay_Cmd(c, 5i32);
                    }
                }
                break 's1;
            }
            5i32 => {
                if ((((!(((({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1360); (c.m.r32(a_) as i32) }) & (16i32))) != 0)) as i32)) != 0) {
                    break 'f;
                }
                if ((l_ok_2) != 0) {
                    { let a0_ = ((c.m.r32(sym::gSimDay)).wrapping_add(32)).wrapping_add((((6i32) as u32)).wrapping_mul(44)); F::Flash_Play(c, a0_, 1i32) };
                    { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 20i32); };
                    { let v_ = 10i32; let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1424); c.m.w32(a_, (v_) as u32); };
                    { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                    if ((((({ let a_ = ((c.m.r32(sym::gProgress)).wrapping_add(1612)).wrapping_add(40); (c.m.r32(a_) as i32) }) == (69i32)) as i32)) != 0) {
                        { let v_ = 5i32; let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(1432)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    } else {
                        if ((((({ let a_ = ((c.m.r32(sym::gProgress)).wrapping_add(1088)).wrapping_add(28); (c.m.r32(a_) as i32) }) == (102i32)) as i32)) != 0) {
                            { let v_ = 4i32; let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(1432)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                        } else {
                            { let v_ = (((F::Rand_Range(c, ((3i32) as u32))).wrapping_add(((1i32) as u32))) as i32); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(1432)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                        }
                    }
                }
                break 's1;
            }
            10i32 => {
                { let v_ = 11i32; let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1424); c.m.w32(a_, (v_) as u32); };
                { let v_ = 300i32; let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1452); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            11i32 => {
                { let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1452); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                if ((((((l_ok_2) != 0) || ((((({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1452); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0)) as i32)) != 0) {
                    F::Bgm_FadeOutStep(c);
                    F::Voice_StopWithLip(c);
                    { let r_ = 1i32; let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    { let r_ = 2i32; let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    { let v_ = 15i32; let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1420); c.m.w32(a_, (v_) as u32); };
                    if ((l_ok_2) != 0) {
                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                    }
                }
                break 's1;
            }
            12i32 => {
                F::ColorFade_StartOut(c, 0i32, 0i32, 0i32, 20i32);
                { let v_ = 0i32; let a_ = p_result_1; c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            _ => {
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "SimDay_Input", &PROBE_N, &[], true); }
    r_
}

pub fn SimDay_ClipGoto(c: &mut Ctx, mut p_movie_1: i32, mut p_kind_2: i32, mut p_label_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SimDay_ClipGoto", &PROBE_N, &[(p_movie_1) as u32, (p_kind_2) as u32, (p_label_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(80) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 80);
    let mut l_flash_4: u32 = 0;
    let r_ = 'f: {
        l_flash_4 = ((c.m.r32(sym::gSimDay)).wrapping_add(32)).wrapping_add((((p_movie_1) as u32)).wrapping_mul(44));
        's1: {
            let sv5: i32 = p_kind_2;
            match sv5 {
                0i32 => {
                    { let _ = { let a2_ = { let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(1368)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; F::sprintf(c, fp.wrapping_add(8), sym::menu_q_b__str186, &[((a2_) as i64 as u64)]) }; };
                    break 's1;
                }
                6i32 => {
                    { let _ = { let a2_ = ({ let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(1368)).wrapping_add((((6i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(1i32); F::sprintf(c, fp.wrapping_add(8), sym::menu_q_b__str187, &[((a2_) as i64 as u64)]) }; };
                    break 's1;
                }
                1i32 => {
                    { let _ = { let a2_ = ({ let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(1368)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(6i32); F::sprintf(c, fp.wrapping_add(8), sym::menu_q_b__str188, &[((a2_) as i64 as u64)]) }; };
                    break 's1;
                }
                2i32 => {
                    { let _ = { let a2_ = ({ let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(1368)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(4i32); F::sprintf(c, fp.wrapping_add(8), sym::menu_q_b__str189, &[((a2_) as i64 as u64)]) }; };
                    break 's1;
                }
                _ => {
                }
            }
        }
        F::Flash_FindLabel(c, l_flash_4, ((0i32) as u32), fp.wrapping_add(8), fp);
        F::Flash_ClipGotoLabel(c, l_flash_4, fp, p_label_3);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "SimDay_ClipGoto", &PROBE_N, &[], true); }
    r_
}

pub fn SimDay_UpdateFaceLoad(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SimDay_UpdateFaceLoad", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        rt::memset(c, fp, 0, 8);
        { let v_ = 0i32; c.m.w32(fp.wrapping_add(0), (v_) as u32); }
        's1: {
            let sv1: i32 = { let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1364); (c.m.r32(a_) as i32) };
            match sv1 {
                1i32 => {
                    { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(860)).wrapping_add((((14i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(860)).wrapping_add((((13i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = { let a_ = ((c.m.r32(sym::gProgress)).wrapping_add(1088)).wrapping_add(28); (c.m.r32(a_) as i32) }; let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    { let v_ = { let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(2904); (c.m.r32(a_) as i32) }; let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    F::File_CancelRequests(c);
                    { let _ = { let a0_ = ((c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)).wrapping_add(761i32); let a1_ = { let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }; let _ = 92160i32; F::File_Request(c, a0_, a1_) }; };
                    { let _ = { let a0_ = ((c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)).wrapping_add(761i32); let a1_ = { let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(8)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }; let _ = 92160i32; F::File_Request(c, a0_, a1_) }; };
                    { let v_ = 2i32; let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1364); c.m.w32(a_, (v_) as u32); };
                    break 's1;
                }
                2i32 => {
                    if ((F::File_UpdateRequests(c)) != 0) {
                        { let v_ = 3i32; let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1364); c.m.w32(a_, (v_) as u32); };
                    }
                    break 's1;
                }
                3i32 => {
                    { let _ = { let a0_ = { let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }; let a1_ = { let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }; F::Sprite_Unpack(c, a0_, a1_, ((0i32) as u32)) }; };
                    { let _ = { let a0_ = { let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(8)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }; let a1_ = { let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }; F::Sprite_Unpack(c, a0_, a1_, ((0i32) as u32)) }; };
                    { let v_ = { let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }; let a_ = fp.wrapping_add(8); c.m.w32(a_, v_); };
                    { let a1_ = c.m.r32(fp.wrapping_add(8)); let a2_ = c.m.r32(fp.wrapping_add(8)); F::Res_RelocateOffsets(c, fp.wrapping_add(8), a1_, a2_) };
                    { let v_ = { let a_ = (c.m.r32(fp.wrapping_add(8))).wrapping_add(16); c.m.r32(a_) }; let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(860)).wrapping_add((((14i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = { let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }; let a_ = fp.wrapping_add(8); c.m.w32(a_, v_); };
                    { let a1_ = c.m.r32(fp.wrapping_add(8)); let a2_ = c.m.r32(fp.wrapping_add(8)); F::Res_RelocateOffsets(c, fp.wrapping_add(8), a1_, a2_) };
                    { let v_ = { let a_ = (c.m.r32(fp.wrapping_add(8))).wrapping_add(16); c.m.r32(a_) }; let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(860)).wrapping_add((((13i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let r_ = 16i32; let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    { let v_ = 0i32; let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1364); c.m.w32(a_, (v_) as u32); };
                    break 's1;
                }
                _ => {
                }
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "SimDay_UpdateFaceLoad", &PROBE_N, &[], true); }
    r_
}

pub fn SimDay_Run(c: &mut Ctx, mut p_section_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SimDay_Run", &PROBE_N, &[(p_section_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        { let v_ = 1i32; c.m.w32(fp, (v_) as u32); }
        F::SimDay_Init(c, p_section_1);
        F::ColorFade_StartIn(c, 0i32, 0i32, 0i32, 20i32);
        'l1: while ((1i32) != 0) {
            F::Gfx_BeginFrame(c);
            F::Pad_Update(c);
            F::Snd_Update(c);
            F::ColorFade_Update(c);
            F::SimDay_UpdateFaceLoad(c);
            F::SimDay_Update(c);
            F::SimDay_UpdateTalk(c);
            F::SimDay_Draw(c);
            F::ColorFade_Draw(c);
            F::Gfx_EndFrame(c, 1i32);
            F::Dma_Flush(c);
            F::File_Stub264D90(c);
            if ((F::ColorFade_IsInDone(c)) != 0) {
                if ((((((((!(((({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1360); (c.m.r32(a_) as i32) }) & (8i32))) != 0)) as i32)) != 0) && (((({ let a_ = (((c.m.r32(sym::gSimDay)).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add(8); c.m.r32(a_) }) & (((2i32) as u32)))) != 0)) as i32)) != 0) {
                    { let r_ = 8i32; let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                }
            }
            if ((F::ColorFade_IsFadingOut(c)) != 0) {
                F::Bgm_FadeOutStep(c);
                F::Voice_FadeOutStep(c, 0i32);
                continue 'l1;
            }
            if ((F::ColorFade_IsOutDone(c)) != 0) {
                if ((((({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1364); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
                    continue 'l1;
                }
                break 'l1;
            }
            if (((({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1360); (c.m.r32(a_) as i32) }) & (2i32))) != 0) {
                if ((((({ let a_ = (c.m.r32(sym::gSimDay)).wrapping_add(1420); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
                    F::ColorFade_StartOut(c, 0i32, 0i32, 0i32, 20i32);
                }
            } else {
                if ((((({ let a_ = ((c.m.r32(sym::gSimDay)).wrapping_add(1432)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
                    F::SimDay_Input(c, fp);
                }
            }
        }
        { let v_ = 0i32; let a_ = ((c.m.r32(sym::gProgress)).wrapping_add(1612)).wrapping_add(52); c.m.w32(a_, (v_) as u32); };
        F::SimDay_Term(c);
        F::Dma_ResetBuffers(c);
        break 'f (c.m.r32(fp) as i32);
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "SimDay_Run", &PROBE_N, &[], true); }
    r_
}
