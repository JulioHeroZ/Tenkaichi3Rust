#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn ChrTbl_WrapCostume(c: &mut Ctx, mut p_chara_1: i32, mut p_costume_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ChrTbl_WrapCostume", &PROBE_N, &[(p_chara_1) as u32, (p_costume_2) as u32], false); }
    let mut l_file_3: u32 = 0;
    let mut l_table_4: u32 = 0;
    let r_ = 'f: {
    l_file_3 = { let a_ = ((c.m.r32(sym::gCommonRes)).wrapping_add(4)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.r32(a_) };
    l_table_4 = (l_file_3).wrapping_add(((c.m.r32((l_file_3).wrapping_add(4))).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32));
    if (((((((((p_costume_2) != (((0i32) as u32))) as i32)) != 0) && ((((!((((((c.m.r32(p_costume_2) as i32)) < (((c.m.r16(((l_table_4).wrapping_add((((p_chara_1) as u32)).wrapping_mul(60))).wrapping_add(10))) as i32))) as i32)) != 0)) as i32)) != 0)) as i32)) != 0) {
        { let v_ = 0i32; let a_ = p_costume_2; c.m.w32(a_, (v_) as u32); };
    }
    break 'f ((c.m.r16(((l_table_4).wrapping_add((((p_chara_1) as u32)).wrapping_mul(60))).wrapping_add(10))) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "ChrTbl_WrapCostume", &PROBE_N, &[], true); }
    r_
}

pub fn ChrTbl_GetCost(c: &mut Ctx, mut p_chara_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ChrTbl_GetCost", &PROBE_N, &[(p_chara_1) as u32], false); }
    let mut l_file_2: u32 = 0;
    let mut l_table_3: u32 = 0;
    let r_ = 'f: {
    l_file_2 = { let a_ = ((c.m.r32(sym::gCommonRes)).wrapping_add(4)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.r32(a_) };
    l_table_3 = (l_file_2).wrapping_add(((c.m.r32((l_file_2).wrapping_add(4))).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32));
    break 'f ((c.m.r16(((l_table_3).wrapping_add((((p_chara_1) as u32)).wrapping_mul(60))).wrapping_add(12))) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "ChrTbl_GetCost", &PROBE_N, &[], true); }
    r_
}

pub fn ChrTbl_GetLevel(c: &mut Ctx, mut p_chara_1: i32, mut p_slot_2: i32, mut p_fromRec_3: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ChrTbl_GetLevel", &PROBE_N, &[(p_chara_1) as u32, (p_slot_2) as u32, (p_fromRec_3) as u32], false); }
    let mut l_level_4: i32 = 0;
    let mut l_file_5: u32 = 0;
    let mut l_table_6: u32 = 0;
    let r_ = 'f: {
    if (((((p_fromRec_3) != (0i32)) as i32)) != 0) {
        l_level_4 = (({ let a_ = (((c.m.r32(sym::gSaveData)).wrapping_add(11584)).wrapping_add((((p_slot_2) as u32)).wrapping_mul(28))).wrapping_add(20); c.m.r16(a_) }) as i32);
    } else {
        l_level_4 = (({ let a_ = (((c.m.r32(sym::gSaveData)).wrapping_add(6152)).wrapping_add((((p_slot_2) as u32)).wrapping_mul(56))).wrapping_add(52); c.m.r16(a_) }) as i32);
    }
    l_file_5 = { let a_ = ((c.m.r32(sym::gCommonRes)).wrapping_add(4)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.r32(a_) };
    l_table_6 = (l_file_5).wrapping_add(((c.m.r32((l_file_5).wrapping_add(4))).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32));
    break 'f (((c.m.r16(((l_table_6).wrapping_add((((p_chara_1) as u32)).wrapping_mul(60))).wrapping_add(14))) as i32)).wrapping_add(l_level_4);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "ChrTbl_GetLevel", &PROBE_N, &[], true); }
    r_
}

pub fn ChrTbl_GetExp(c: &mut Ctx, mut p_chara_1: i32, mut p_slot_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ChrTbl_GetExp", &PROBE_N, &[(p_chara_1) as u32, (p_slot_2) as u32], false); }
    let mut l_file_3: u32 = 0;
    let mut l_table_4: u32 = 0;
    let r_ = 'f: {
    l_file_3 = { let a_ = ((c.m.r32(sym::gCommonRes)).wrapping_add(4)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.r32(a_) };
    l_table_4 = (l_file_3).wrapping_add(((c.m.r32((l_file_3).wrapping_add(4))).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32));
    break 'f { let a_ = (((l_table_4).wrapping_add((((p_chara_1) as u32)).wrapping_mul(60))).wrapping_add(16)).wrapping_add(((({ let a_ = (((c.m.r32(sym::gSaveData)).wrapping_add(6152)).wrapping_add((((p_slot_2) as u32)).wrapping_mul(56))).wrapping_add(52); c.m.r16(a_) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "ChrTbl_GetExp", &PROBE_N, &[], true); }
    r_
}

pub fn ChrTbl_GetMaxExp(c: &mut Ctx, mut p_chara_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ChrTbl_GetMaxExp", &PROBE_N, &[(p_chara_1) as u32], false); }
    let mut l_file_2: u32 = 0;
    let mut l_table_3: u32 = 0;
    let mut l_i_4: i32 = 0;
    let mut l_max_5: i32 = 0;
    let r_ = 'f: {
    l_file_2 = { let a_ = ((c.m.r32(sym::gCommonRes)).wrapping_add(4)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.r32(a_) };
    l_table_3 = (l_file_2).wrapping_add(((c.m.r32((l_file_2).wrapping_add(4))).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32));
    l_max_5 = 0i32;
    {
        l_i_4 = 0i32;
        'l1: loop {
            if !(((((l_i_4) < (7i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((((((c.m.r32((((l_table_3).wrapping_add((((p_chara_1) as u32)).wrapping_mul(60))).wrapping_add(16)).wrapping_add((((l_i_4) as u32)).wrapping_mul(4))) as i32)) != (0i32)) as i32)) != 0) {
                    l_max_5 = (c.m.r32((((l_table_3).wrapping_add((((p_chara_1) as u32)).wrapping_mul(60))).wrapping_add(16)).wrapping_add((((l_i_4) as u32)).wrapping_mul(4))) as i32);
                } else {
                    break 'l1;
                }
            }
            { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
        }
    }
    break 'f l_max_5;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "ChrTbl_GetMaxExp", &PROBE_N, &[], true); }
    r_
}

pub fn ItemSet_Fit(c: &mut Ctx, mut p_ids_1: u32, mut p_table_2: u32, mut p_capacity_3: i32, mut p_used_4: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ItemSet_Fit", &PROBE_N, &[(p_ids_1) as u32, (p_table_2) as u32, (p_capacity_3) as u32, (p_used_4) as u32], false); }
    let mut l_total_5: i32 = 0;
    let mut l_i_6: i32 = 0;
    let mut l_j_7: i32 = 0;
    let mut l_sum_8: i32 = 0;
    let mut l_p_9: u32 = 0;
    let r_ = 'f: {
    l_total_5 = 0i32;
    if (((((p_used_4) != (((0i32) as u32))) as i32)) != 0) {
        { let v_ = 0i32; let a_ = p_used_4; c.m.w32(a_, (v_) as u32); };
    }
    {
        l_i_6 = 0i32;
        'l1: loop {
            if !(((((l_i_6) < (7i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                l_p_9 = (p_ids_1).wrapping_add((((l_i_6) as u32)).wrapping_mul(2));
                if (((((((c.m.r16(l_p_9)) as i32)) != (0i32)) as i32)) != 0) {
                    l_sum_8 = (l_total_5).wrapping_add((({ let a_ = ((p_table_2).wrapping_add(((((((c.m.r16(l_p_9)) as i32)).wrapping_sub(1i32)) as u32)).wrapping_mul(40))).wrapping_add(3); c.m.r8(a_) }) as i32));
                    if ((((!(((((p_capacity_3) < (l_sum_8)) as i32)) != 0)) as i32)) != 0) {
                        l_total_5 = l_sum_8;
                        if (((((p_used_4) != (((0i32) as u32))) as i32)) != 0) {
                            { let v_ = (l_i_6).wrapping_add(1i32); let a_ = p_used_4; c.m.w32(a_, (v_) as u32); };
                        }
                    } else {
                        {
                            l_j_7 = l_i_6;
                            'l3: loop {
                                if !(((((l_j_7) < (7i32)) as i32)) != 0) { break 'l3; }
                                'c4: {
                                    { let v_ = ((0i32) as u16); let a_ = (p_ids_1).wrapping_add((((l_j_7) as u32)).wrapping_mul(2)); c.m.w16(a_, v_); };
                                }
                                { let o_ = l_j_7; let n_ = (o_).wrapping_add(1); l_j_7 = n_; };
                            }
                        }
                        break 'l1;
                    }
                }
            }
            { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; };
        }
    }
    if (((((p_used_4) != (((0i32) as u32))) as i32)) != 0) {
        { let r_ = (p_capacity_3).wrapping_sub(l_total_5); let a_ = p_used_4; let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
    }
    break 'f l_total_5;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "ItemSet_Fit", &PROBE_N, &[], true); }
    r_
}

pub fn ItemSet_GetBonus(c: &mut Ctx, mut p_ids_1: u32, mut p_table_2: u32, mut p_out_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ItemSet_GetBonus", &PROBE_N, &[(p_ids_1) as u32, (p_table_2) as u32, (p_out_3) as u32], false); }
    let mut l_i_4: i32 = 0;
    let mut l_k_5: i32 = 0;
    let mut l_idx_6: i32 = 0;
    let mut l_div_7: i32 = 0;
    let r_ = 'f: {
    { let _ = F::memset(c, p_out_3, 0i32, ((20i32) as u32)); };
    {
        l_i_4 = 0i32;
        'l1: loop {
            if !(((((l_i_4) < (8i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if (((((((c.m.r16((p_ids_1).wrapping_add((((l_i_4) as u32)).wrapping_mul(2)))) as i32)) != (0i32)) as i32)) != 0) {
                    l_idx_6 = (((c.m.r16((p_ids_1).wrapping_add((((l_i_4) as u32)).wrapping_mul(2)))) as i32)).wrapping_sub(1i32);
                    if (((((((c.m.r8((p_table_2).wrapping_add((((l_idx_6) as u32)).wrapping_mul(40)))) as i32)) == (1i32)) as i32)) != 0) {
                        { let r_ = ((c.m.r8(((p_table_2).wrapping_add((((l_idx_6) as u32)).wrapping_mul(40))).wrapping_add(3))) as i32); let a_ = (p_out_3).wrapping_add((((0i32) as u32)).wrapping_mul(4)); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                    }
                    {
                        l_k_5 = 0i32;
                        'l3: loop {
                            if !(((((l_k_5) < (4i32)) as i32)) != 0) { break 'l3; }
                            'c4: {
                                { let r_ = (((c.m.r16((((p_table_2).wrapping_add((((l_idx_6) as u32)).wrapping_mul(40))).wrapping_add(12)).wrapping_add((((l_k_5) as u32)).wrapping_mul(2))) as i16)) as i32); let a_ = (p_out_3).wrapping_add(((((1i32).wrapping_add(l_k_5)) as u32)).wrapping_mul(4)); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                            }
                            { let o_ = l_k_5; let n_ = (o_).wrapping_add(1); l_k_5 = n_; };
                        }
                    }
                }
            }
            { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
        }
    }
    {
        l_k_5 = 0i32;
        'l5: loop {
            if !(((((l_k_5) < (5i32)) as i32)) != 0) { break 'l5; }
            'c6: {
                if (((((l_k_5) == (0i32)) as i32)) != 0) {
                    if ((((((c.m.r32((p_out_3).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) >= (8i32)) as i32)) != 0) {
                        { let v_ = 7i32; let a_ = (p_out_3).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    }
                } else {
                    if ((((((c.m.r32((p_out_3).wrapping_add((((l_k_5) as u32)).wrapping_mul(4))) as i32)) > (20i32)) as i32)) != 0) {
                        { let v_ = 20i32; let a_ = (p_out_3).wrapping_add((((l_k_5) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    } else {
                        if ((((((c.m.r32((p_out_3).wrapping_add((((l_k_5) as u32)).wrapping_mul(4))) as i32)) < ((20i32).wrapping_neg())) as i32)) != 0) {
                            { let v_ = (20i32).wrapping_neg(); let a_ = (p_out_3).wrapping_add((((l_k_5) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                        }
                    }
                }
            }
            { let o_ = l_k_5; let n_ = (o_).wrapping_add(1); l_k_5 = n_; };
        }
    }
    l_div_7 = 5i32;
    {
        l_k_5 = 0i32;
        'l7: loop {
            if !(((((l_k_5) < (4i32)) as i32)) != 0) { break 'l7; }
            'c8: {
                { let r_ = l_div_7; let a_ = (p_out_3).wrapping_add(((((1i32).wrapping_add(l_k_5)) as u32)).wrapping_mul(4)); let n_ = rt::div_i32((c.m.r32(a_) as i32), r_); c.m.w32(a_, (n_) as u32); };
                if ((((((c.m.r32((p_out_3).wrapping_add(((((1i32).wrapping_add(l_k_5)) as u32)).wrapping_mul(4))) as i32)) > (3i32)) as i32)) != 0) {
                    { let v_ = 3i32; let a_ = (p_out_3).wrapping_add(((((1i32).wrapping_add(l_k_5)) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                } else {
                    if ((((((c.m.r32((p_out_3).wrapping_add(((((1i32).wrapping_add(l_k_5)) as u32)).wrapping_mul(4))) as i32)) < ((3i32).wrapping_neg())) as i32)) != 0) {
                        { let v_ = (3i32).wrapping_neg(); let a_ = (p_out_3).wrapping_add(((((1i32).wrapping_add(l_k_5)) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    }
                }
            }
            { let o_ = l_k_5; let n_ = (o_).wrapping_add(1); l_k_5 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ItemSet_GetBonus", &PROBE_N, &[], true); }
    r_
}

pub fn ItemSet_GetStats(c: &mut Ctx, mut p_ids_1: u32, mut p_stats_2: u32, mut p_ability_3: u32, mut p_chara_4: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ItemSet_GetStats", &PROBE_N, &[(p_ids_1) as u32, (p_stats_2) as u32, (p_ability_3) as u32, (p_chara_4) as u32], false); }
    let mut l_file_5: u32 = 0;
    let mut l_items_6: u32 = 0;
    let mut l_charas_7: u32 = 0;
    let mut l_i_8: i32 = 0;
    let mut l_k_9: i32 = 0;
    let mut l_m_10: i32 = 0;
    let mut l_id_11: i32 = 0;
    let mut l_idx_12: i32 = 0;
    let r_ = 'f: {
    l_file_5 = { let a_ = ((c.m.r32(sym::gCommonRes)).wrapping_add(4)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.r32(a_) };
    l_items_6 = (l_file_5).wrapping_add(((c.m.r32((l_file_5).wrapping_add(8))).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32));
    l_charas_7 = (l_file_5).wrapping_add(((c.m.r32((l_file_5).wrapping_add(4))).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32));
    { let _ = F::memset(c, p_stats_2, 0i32, ((20i32) as u32)); };
    { let _ = F::memset(c, p_ability_3, 0i32, ((16i32) as u32)); };
    {
        l_i_8 = 0i32;
        'l1: loop {
            if !(((((l_i_8) < (8i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if (((((((c.m.r16((p_ids_1).wrapping_add((((l_i_8) as u32)).wrapping_mul(2)))) as i32)) != (0i32)) as i32)) != 0) {
                    l_id_11 = ((c.m.r16((p_ids_1).wrapping_add((((l_i_8) as u32)).wrapping_mul(2)))) as i32);
                    l_idx_12 = (l_id_11).wrapping_sub(1i32);
                    if (((((((c.m.r8((l_items_6).wrapping_add((((l_idx_12) as u32)).wrapping_mul(40)))) as i32)) == (2i32)) as i32)) != 0) {
                        { let v_ = (l_id_11).wrapping_sub(135i32); let a_ = (p_stats_2).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    } else {
                        {
                            l_k_9 = 0i32;
                            'l3: loop {
                                if !(((((l_k_9) < (4i32)) as i32)) != 0) { break 'l3; }
                                'c4: {
                                    { let r_ = (((c.m.r16((((l_items_6).wrapping_add((((l_idx_12) as u32)).wrapping_mul(40))).wrapping_add(12)).wrapping_add((((l_k_9) as u32)).wrapping_mul(2))) as i16)) as i32); let a_ = (p_stats_2).wrapping_add((((l_k_9) as u32)).wrapping_mul(4)); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                                    {
                                        l_m_10 = 0i32;
                                        'l5: loop {
                                            if !(((((l_m_10) < (4i32)) as i32)) != 0) { break 'l5; }
                                            'c6: {
                                                { let r_ = (c.m.r32((((l_items_6).wrapping_add((((l_idx_12) as u32)).wrapping_mul(40))).wrapping_add(24)).wrapping_add((((l_m_10) as u32)).wrapping_mul(4))) as i32); let a_ = (p_ability_3).wrapping_add((((l_m_10) as u32)).wrapping_mul(4)); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                                            }
                                            { let o_ = l_m_10; let n_ = (o_).wrapping_add(1); l_m_10 = n_; };
                                        }
                                    }
                                }
                                { let o_ = l_k_9; let n_ = (o_).wrapping_add(1); l_k_9 = n_; };
                            }
                        }
                    }
                }
            }
            { let o_ = l_i_8; let n_ = (o_).wrapping_add(1); l_i_8 = n_; };
        }
    }
    if ((((((c.m.r32((p_stats_2).wrapping_add((((4i32) as u32)).wrapping_mul(4))) as i32)) == (0i32)) as i32)) != 0) {
        if (((((p_chara_4) < (0i32)) as i32)) != 0) {
            { let v_ = 0i32; let a_ = (p_stats_2).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        } else {
            { let v_ = (c.m.r32((l_charas_7).wrapping_add((((p_chara_4) as u32)).wrapping_mul(60))) as i32); let a_ = (p_stats_2).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ItemSet_GetStats", &PROBE_N, &[], true); }
    r_
}

pub fn ItemSet_GetStatsList(c: &mut Ctx, mut p_ids_1: u32, mut p_count_2: i32, mut p_stats_3: u32, mut p_ability_4: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ItemSet_GetStatsList", &PROBE_N, &[(p_ids_1) as u32, (p_count_2) as u32, (p_stats_3) as u32, (p_ability_4) as u32], false); }
    let mut l_file_5: u32 = 0;
    let mut l_items_6: u32 = 0;
    let mut l_i_7: i32 = 0;
    let mut l_k_8: i32 = 0;
    let mut l_m_9: i32 = 0;
    let mut l_id_10: i32 = 0;
    let mut l_idx_11: i32 = 0;
    let r_ = 'f: {
    l_file_5 = { let a_ = ((c.m.r32(sym::gCommonRes)).wrapping_add(4)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.r32(a_) };
    l_items_6 = (l_file_5).wrapping_add(((c.m.r32((l_file_5).wrapping_add(8))).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32));
    { let _ = F::memset(c, p_stats_3, 0i32, ((20i32) as u32)); };
    { let _ = F::memset(c, p_ability_4, 0i32, ((16i32) as u32)); };
    {
        l_i_7 = 0i32;
        'l1: loop {
            if !(((((l_i_7) < (p_count_2)) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((((((c.m.r32((p_ids_1).wrapping_add((((l_i_7) as u32)).wrapping_mul(4))) as i32)) != (0i32)) as i32)) != 0) {
                    l_id_10 = (c.m.r32((p_ids_1).wrapping_add((((l_i_7) as u32)).wrapping_mul(4))) as i32);
                    l_idx_11 = (l_id_10).wrapping_sub(1i32);
                    if (((((((c.m.r8((l_items_6).wrapping_add((((l_idx_11) as u32)).wrapping_mul(40)))) as i32)) == (2i32)) as i32)) != 0) {
                        { let v_ = (l_id_10).wrapping_sub(135i32); let a_ = (p_stats_3).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    } else {
                        {
                            l_k_8 = 0i32;
                            'l3: loop {
                                if !(((((l_k_8) < (4i32)) as i32)) != 0) { break 'l3; }
                                'c4: {
                                    { let r_ = (((c.m.r16((((l_items_6).wrapping_add((((l_idx_11) as u32)).wrapping_mul(40))).wrapping_add(12)).wrapping_add((((l_k_8) as u32)).wrapping_mul(2))) as i16)) as i32); let a_ = (p_stats_3).wrapping_add((((l_k_8) as u32)).wrapping_mul(4)); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                                    {
                                        l_m_9 = 0i32;
                                        'l5: loop {
                                            if !(((((l_m_9) < (4i32)) as i32)) != 0) { break 'l5; }
                                            'c6: {
                                                { let r_ = (c.m.r32((((l_items_6).wrapping_add((((l_idx_11) as u32)).wrapping_mul(40))).wrapping_add(24)).wrapping_add((((l_m_9) as u32)).wrapping_mul(4))) as i32); let a_ = (p_ability_4).wrapping_add((((l_m_9) as u32)).wrapping_mul(4)); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                                            }
                                            { let o_ = l_m_9; let n_ = (o_).wrapping_add(1); l_m_9 = n_; };
                                        }
                                    }
                                }
                                { let o_ = l_k_8; let n_ = (o_).wrapping_add(1); l_k_8 = n_; };
                            }
                        }
                    }
                }
            }
            { let o_ = l_i_7; let n_ = (o_).wrapping_add(1); l_i_7 = n_; };
        }
    }
    if ((((((c.m.r32((p_stats_3).wrapping_add((((4i32) as u32)).wrapping_mul(4))) as i32)) == (0i32)) as i32)) != 0) {
        { let v_ = 0i32; let a_ = (p_stats_3).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ItemSet_GetStatsList", &PROBE_N, &[], true); }
    r_
}

pub fn ItemTbl_CanEquip(c: &mut Ctx, mut p_item_1: i32, mut p_chara_2: i32, mut p_table_3: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ItemTbl_CanEquip", &PROBE_N, &[(p_item_1) as u32, (p_chara_2) as u32, (p_table_3) as u32], false); }
    let mut l_flags_4: u32 = 0;
    let mut l_file_5: u32 = 0;
    let mut l_charas_6: u32 = 0;
    let r_ = 'f: {
    l_flags_4 = (((c.m.r32(((p_table_3).wrapping_add((((p_item_1) as u32)).wrapping_mul(40))).wrapping_add(20)) as i32)) as u32);
    if ((((l_flags_4) & (((((1i32) | (4i32))) as u32)))) != 0) {
        break 'f 0i32;
    }
    l_file_5 = { let a_ = ((c.m.r32(sym::gCommonRes)).wrapping_add(4)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.r32(a_) };
    l_charas_6 = (l_file_5).wrapping_add(((c.m.r32((l_file_5).wrapping_add(4))).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32));
    if ((((((c.m.r16(((l_charas_6).wrapping_add((((p_chara_2) as u32)).wrapping_mul(60))).wrapping_add(8))) as i32)) & (1i32))) != 0) {
        if ((((l_flags_4) & (((8i32) as u32)))) != 0) {
            break 'f 1i32;
        }
        break 'f 0i32;
    }
    if ((((l_flags_4) & (((16i32) as u32)))) != 0) {
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "ItemTbl_CanEquip", &PROBE_N, &[], true); }
    r_
}

pub fn ItemTbl_GetClass(c: &mut Ctx, mut p_item_1: i32, mut p_table_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ItemTbl_GetClass", &PROBE_N, &[(p_item_1) as u32, (p_table_2) as u32], false); }
    let mut l_flags_3: u32 = 0;
    let r_ = 'f: {
    l_flags_3 = (((c.m.r32(((p_table_2).wrapping_add((((p_item_1) as u32)).wrapping_mul(40))).wrapping_add(20)) as i32)) as u32);
    if ((((l_flags_3) & (((8i32) as u32)))) != 0) {
        if ((((l_flags_3) & (((16i32) as u32)))) != 0) {
            break 'f 1i32;
        }
        break 'f 0i32;
    }
    break 'f 2i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "ItemTbl_GetClass", &PROBE_N, &[], true); }
    r_
}
