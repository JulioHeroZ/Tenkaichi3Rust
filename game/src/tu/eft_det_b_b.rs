#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn StgNav_IsReady(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgNav_IsReady", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if (((((c.m.r32(sym::gStgNav)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((!(({ let a_ = (c.m.r32(sym::gStgNav)).wrapping_add(4); (c.m.r32(a_) as i32) }) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    break 'f (((F::BtlStage_IsReady(c)) != (0i32)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "StgNav_IsReady", &PROBE_N, &[], true); }
    r_
}

pub fn StgNav_Bind(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgNav_Bind", &PROBE_N, &[], false); }
    let mut l_file_1: u32 = 0;
    let mut l_start_2: u32 = 0;
    let mut l_end_3: u32 = 0;
    let mut l_data_4: u32 = 0;
    let r_ = 'f: {
    if ((((({ let a_ = (c.m.r32(sym::gStgNav)).wrapping_add(4); (c.m.r32(a_) as i32) }) == (1i32)) as i32)) != 0) {
        break 'f (1i32).wrapping_neg();
    }
    l_file_1 = { let a_ = (c.m.r32(sym::gCommonRes)).wrapping_add(36); c.m.r32(a_) };
    l_start_2 = c.m.r32((l_file_1).wrapping_add((((16i32) as u32)).wrapping_mul(4)));
    l_end_3 = c.m.r32((l_file_1).wrapping_add((((17i32) as u32)).wrapping_mul(4)));
    l_data_4 = (l_file_1).wrapping_add((rt::div_u32(l_start_2, ((4i32) as u32))).wrapping_mul(4));
    if (((((l_end_3) == (l_start_2)) as i32)) != 0) {
        break 'f (2i32).wrapping_neg();
    }
    { let v_ = l_data_4; let a_ = c.m.r32(sym::gStgNav); c.m.w32(a_, v_); };
    { let v_ = (l_data_4).wrapping_add(((16i32) as u32)); let a_ = ({ let a_ = c.m.r32(sym::gStgNav); c.m.r32(a_) }).wrapping_add(12); c.m.w32(a_, v_); };
    if ((((({ let a_ = { let a_ = c.m.r32(sym::gStgNav); c.m.r32(a_) }; (c.m.r32(a_) as i32) }) != (1i32)) as i32)) != 0) {
        break 'f (3i32).wrapping_neg();
    }
    { let v_ = { let a_ = { let a_ = c.m.r32(sym::gStgNav); c.m.r32(a_) }; (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gStgNav)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "StgNav_Bind", &PROBE_N, &[], true); }
    r_
}

pub fn StgNav_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgNav_Term", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if (((((c.m.r32(sym::gStgNav)) != (((0i32) as u32))) as i32)) != 0) {
        if ((((({ let a_ = (c.m.r32(sym::gStgNav)).wrapping_add(12); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
            { let a0_ = { let a_ = (c.m.r32(sym::gStgNav)).wrapping_add(12); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
            { let v_ = ((0i32) as u32); let a_ = (c.m.r32(sym::gStgNav)).wrapping_add(12); c.m.w32(a_, v_); };
        }
        if ((((({ let a_ = (c.m.r32(sym::gStgNav)).wrapping_add(16); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
            { let a0_ = { let a_ = (c.m.r32(sym::gStgNav)).wrapping_add(16); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
            { let v_ = ((0i32) as u32); let a_ = (c.m.r32(sym::gStgNav)).wrapping_add(16); c.m.w32(a_, v_); };
        }
        if (((((c.m.r32(sym::gStgNav)) != (((0i32) as u32))) as i32)) != 0) {
            { let a0_ = c.m.r32(sym::gStgNav); F::Heap_Free(c, a0_) };
            { let v_ = ((0i32) as u32); let a_ = sym::gStgNav; c.m.w32(a_, v_); };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgNav_Term", &PROBE_N, &[], true); }
    r_
}

pub fn StgNav_Rebind(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgNav_Rebind", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if ((F::StgNav_IsReady(c)) != 0) {
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gStgNav)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        if (((((F::StgNav_Bind(c)) == (0i32)) as i32)) != 0) {
            F::StgNav_BlockObj(c, (1i32).wrapping_neg());
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgNav_Rebind", &PROBE_N, &[], true); }
    r_
}

pub fn StgNav_Init(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgNav_Init", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if (((((c.m.r32(sym::gStgNav)) == (((0i32) as u32))) as i32)) != 0) {
        { let v_ = F::Heap_Alloc(c, ((20u32) as i32), ((32i32) as u32), 0i32, 2i32); let a_ = sym::gStgNav; c.m.w32(a_, v_); };
        { let _ = { let a0_ = c.m.r32(sym::gStgNav); F::memset(c, a0_, 0i32, 20u32) }; };
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gStgNav)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = ((0i32) as u32); let a_ = c.m.r32(sym::gStgNav); c.m.w32(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = (c.m.r32(sym::gStgNav)).wrapping_add(12); c.m.w32(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = (c.m.r32(sym::gStgNav)).wrapping_add(16); c.m.w32(a_, v_); };
        { let v_ = F::Battle_GetStage(c); let a_ = (c.m.r32(sym::gStgNav)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = F::Heap_Alloc(c, 3072i32, ((32i32) as u32), 0i32, 2i32); let a_ = (c.m.r32(sym::gStgNav)).wrapping_add(12); c.m.w32(a_, v_); };
        { let _ = { let a0_ = { let a_ = (c.m.r32(sym::gStgNav)).wrapping_add(12); c.m.r32(a_) }; F::memset(c, a0_, 0i32, ((3072i32) as u32)) }; };
        { let v_ = F::Heap_Alloc(c, 3072i32, ((32i32) as u32), 0i32, 2i32); let a_ = (c.m.r32(sym::gStgNav)).wrapping_add(16); c.m.w32(a_, v_); };
        { let _ = { let a0_ = { let a_ = (c.m.r32(sym::gStgNav)).wrapping_add(16); c.m.r32(a_) }; F::memset(c, a0_, 0i32, ((3072i32) as u32)) }; };
        { let _ = F::StgNav_Bind(c); };
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "StgNav_Init", &PROBE_N, &[], true); }
    r_
}

pub fn StgNav_FindNearest(c: &mut Ctx, mut p_pos_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgNav_FindNearest", &PROBE_N, &[(p_pos_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_best_2: i32 = 0;
    let mut l_bestDist_3: F32 = F32(0);
    let mut l_i_4: i32 = 0;
    let mut l_node_5: u32 = 0;
    let mut l_dist_6: F32 = F32(0);
    let r_ = 'f: {
        l_best_2 = (1i32).wrapping_neg();
        l_bestDist_3 = F32(0x461c4000);
        {
            l_i_4 = 0i32;
            'l1: loop {
                if !(((((l_i_4) < ({ let a_ = ({ let a_ = c.m.r32(sym::gStgNav); c.m.r32(a_) }).wrapping_add(4); (c.m.r32(a_) as i32) })) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_node_5 = ({ let a_ = ({ let a_ = c.m.r32(sym::gStgNav); c.m.r32(a_) }).wrapping_add(12); c.m.r32(a_) }).wrapping_add((((l_i_4) as u32)).wrapping_mul(48));
                    if (((((c.m.r32((l_node_5).wrapping_add(16)) as i32)) & (1i32))) != 0) {
                        break 'c2;
                    }
                    F::Vec3_Sub(c, fp, p_pos_1, l_node_5);
                    l_dist_6 = F::Vec3_Length(c, fp);
                    if (((rt::flt(l_bestDist_3, l_dist_6) as i32)) != 0) {
                        break 'c2;
                    }
                    l_bestDist_3 = l_dist_6;
                    l_best_2 = l_i_4;
                }
                { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
            }
        }
        break 'f l_best_2;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "StgNav_FindNearest", &PROBE_N, &[], true); }
    r_
}

pub fn StgNavNode_Clear(c: &mut Ctx, mut p_node_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgNavNode_Clear", &PROBE_N, &[(p_node_1) as u32], false); }
    let mut l_i_2: i32 = 0;
    let mut l_none_3: i16 = 0;
    let r_ = 'f: {
    F::Vec4_Set(c, p_node_1, F32(0x00000000), F32(0x00000000), F32(0x00000000), F32(0x00000000));
    { let v_ = 0i32; let a_ = (p_node_1).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
    { let v_ = (1i32).wrapping_neg(); let a_ = (p_node_1).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
    {
        l_i_2 = 2i32;
        'l1: loop {
            if !(((((l_i_2) >= (0i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let v_ = 0i32; let a_ = ((p_node_1).wrapping_add(24)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_sub(1); l_i_2 = n_; };
        }
    }
    l_none_3 = (((1i32).wrapping_neg()) as i16);
    {
        l_i_2 = 7i32;
        'l3: loop {
            if !(((((l_i_2) >= (0i32)) as i32)) != 0) { break 'l3; }
            'c4: {
                { let v_ = l_none_3; let a_ = ((p_node_1).wrapping_add(32)).wrapping_add((((l_i_2) as u32)).wrapping_mul(2)); c.m.w16(a_, (v_) as u16); };
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_sub(1); l_i_2 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgNavNode_Clear", &PROBE_N, &[], true); }
    r_
}

pub fn StgNav_SwapEntries(c: &mut Ctx, mut p_a_1: u32, mut p_b_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgNav_SwapEntries", &PROBE_N, &[(p_a_1) as u32, (p_b_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        { let s_ = p_a_1; let d_ = fp; rt::memcpy(c, d_, s_, 12); };
        { let s_ = p_b_2; let d_ = p_a_1; rt::memcpy(c, d_, s_, 12); };
        { let s_ = fp; let d_ = p_b_2; rt::memcpy(c, d_, s_, 12); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "StgNav_SwapEntries", &PROBE_N, &[], true); }
    r_
}

pub fn StgNav_SortEntries(c: &mut Ctx, mut p_lo_1: i32, mut p_hi_2: i32, mut p_list_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgNav_SortEntries", &PROBE_N, &[(p_lo_1) as u32, (p_hi_2) as u32, (p_list_3) as u32], false); }
    let mut l_last_4: i32 = 0;
    let mut l_i_5: i32 = 0;
    let r_ = 'f: {
    if (((((p_lo_1) >= (p_hi_2)) as i32)) != 0) {
        break 'f;
    }
    F::StgNav_SwapEntries(c, (p_list_3).wrapping_add((((p_lo_1) as u32)).wrapping_mul(12)), (p_list_3).wrapping_add((((rt::div_i32((p_lo_1).wrapping_add(p_hi_2), 2i32)) as u32)).wrapping_mul(12)));
    l_last_4 = p_lo_1;
    {
        l_i_5 = (p_lo_1).wrapping_add(1i32);
        'l1: loop {
            if !(((((l_i_5) <= (p_hi_2)) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((((((c.m.r32(((p_list_3).wrapping_add((((l_i_5) as u32)).wrapping_mul(12))).wrapping_add(8)) as i32)) > ((c.m.r32(((p_list_3).wrapping_add((((p_lo_1) as u32)).wrapping_mul(12))).wrapping_add(8)) as i32))) as i32)) != 0) {
                    F::StgNav_SwapEntries(c, (p_list_3).wrapping_add(((({ let o_ = l_last_4; let n_ = (o_).wrapping_add(1); l_last_4 = n_; n_ }) as u32)).wrapping_mul(12)), (p_list_3).wrapping_add((((l_i_5) as u32)).wrapping_mul(12)));
                }
            }
            { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
        }
    }
    F::StgNav_SwapEntries(c, (p_list_3).wrapping_add((((l_last_4) as u32)).wrapping_mul(12)), (p_list_3).wrapping_add((((p_lo_1) as u32)).wrapping_mul(12)));
    F::StgNav_SortEntries(c, p_lo_1, (l_last_4).wrapping_sub(1i32), p_list_3);
    F::StgNav_SortEntries(c, (l_last_4).wrapping_add(1i32), p_hi_2, p_list_3);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgNav_SortEntries", &PROBE_N, &[], true); }
    r_
}

pub fn StgNav_FindPath(c: &mut Ctx, mut p_from_1: u32, mut p_to_2: u32, mut p_path_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgNav_FindPath", &PROBE_N, &[(p_from_1) as u32, (p_to_2) as u32, (p_path_3) as u32], false); }
    let mut l_closedCount_4: i32 = 0;
    let mut l_openCount_5: i32 = 0;
    let mut l_start_6: i32 = 0;
    let mut l_goal_7: i32 = 0;
    let mut l_e_8: u32 = 0;
    let mut l_p_9: u32 = 0;
    let mut l_node_10: u32 = 0;
    let mut l_n_11: u32 = 0;
    let mut l_found_12: i32 = 0;
    let mut l_i_13: i32 = 0;
    let mut l_j_14: i32 = 0;
    let r_ = 'f: {
    l_closedCount_4 = 0i32;
    { let v_ = 0i32; let a_ = (p_path_3).wrapping_add(256); c.m.w32(a_, (v_) as u32); };
    if ((((!((F::StgNav_IsReady(c)) != 0)) as i32)) != 0) {
        break 'f;
    }
    l_start_6 = F::StgNav_FindNearest(c, p_from_1);
    l_goal_7 = F::StgNav_FindNearest(c, p_to_2);
    if (((((l_start_6) == ((1i32).wrapping_neg())) as i32)) != 0) {
        break 'f;
    }
    if (((((l_goal_7) == ((1i32).wrapping_neg())) as i32)) != 0) {
        break 'f;
    }
    l_openCount_5 = 1i32;
    l_e_8 = { let a_ = (c.m.r32(sym::gStgNav)).wrapping_add(12); c.m.r32(a_) };
    { let v_ = l_start_6; let a_ = l_e_8; c.m.w32(a_, (v_) as u32); };
    { let v_ = ((0i32) as u32); let a_ = (l_e_8).wrapping_add(4); c.m.w32(a_, v_); };
    { let v_ = 0i32; let a_ = (l_e_8).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
    'l1: loop {
        'c2: {
            if ((((((c.m.r32(l_e_8) as i32)) == (l_goal_7)) as i32)) != 0) {
                { let s_ = p_to_2; let d_ = (p_path_3).wrapping_add(((({ let a_ = (p_path_3).wrapping_add(256); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); o_ }) as u32)).wrapping_mul(16)); rt::memcpy(c, d_, s_, 16); };
                if ((((((c.m.r32((p_path_3).wrapping_add(256)) as i32)) >= (16i32)) as i32)) != 0) {
                    break 'f;
                }
                l_n_11 = ({ let a_ = ({ let a_ = c.m.r32(sym::gStgNav); c.m.r32(a_) }).wrapping_add(12); c.m.r32(a_) }).wrapping_add(((((c.m.r32(l_e_8) as i32)) as u32)).wrapping_mul(48));
                { let s_ = l_n_11; let d_ = (p_path_3).wrapping_add(((({ let a_ = (p_path_3).wrapping_add(256); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); o_ }) as u32)).wrapping_mul(16)); rt::memcpy(c, d_, s_, 16); };
                l_p_9 = l_e_8;
                'l3: while (((((c.m.r32((l_p_9).wrapping_add(4))) != (((0i32) as u32))) as i32)) != 0) {
                    l_p_9 = c.m.r32((l_p_9).wrapping_add(4));
                    if ((((((c.m.r32((p_path_3).wrapping_add(256)) as i32)) >= (16i32)) as i32)) != 0) {
                        break 'f;
                    }
                    l_n_11 = ({ let a_ = ({ let a_ = c.m.r32(sym::gStgNav); c.m.r32(a_) }).wrapping_add(12); c.m.r32(a_) }).wrapping_add(((((c.m.r32(l_p_9) as i32)) as u32)).wrapping_mul(48));
                    { let s_ = l_n_11; let d_ = (p_path_3).wrapping_add(((({ let a_ = (p_path_3).wrapping_add(256); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); o_ }) as u32)).wrapping_mul(16)); rt::memcpy(c, d_, s_, 16); };
                }
                break 'f;
            }
            { let s_ = l_e_8; let d_ = ({ let a_ = (c.m.r32(sym::gStgNav)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((l_closedCount_4) as u32)).wrapping_mul(12)); rt::memcpy(c, d_, s_, 12); };
            { let o_ = l_closedCount_4; let n_ = (o_).wrapping_add(1); l_closedCount_4 = n_; };
            { let o_ = l_openCount_5; let n_ = (o_).wrapping_sub(1); l_openCount_5 = n_; };
            l_node_10 = ({ let a_ = ({ let a_ = c.m.r32(sym::gStgNav); c.m.r32(a_) }).wrapping_add(12); c.m.r32(a_) }).wrapping_add(((((c.m.r32(l_e_8) as i32)) as u32)).wrapping_mul(48));
            {
                l_i_13 = 0i32;
                'l4: loop {
                    if !(((((l_i_13) < (8i32)) as i32)) != 0) { break 'l4; }
                    'c5: {
                        if ((((((((c.m.r16(((l_node_10).wrapping_add(32)).wrapping_add((((l_i_13) as u32)).wrapping_mul(2))) as i16)) as i32)) == ((1i32).wrapping_neg())) as i32)) != 0) {
                            break 'c5;
                        }
                        if (((((c.m.r32((l_node_10).wrapping_add(16)) as i32)) & (1i32))) != 0) {
                            break 'c5;
                        }
                        l_found_12 = 0i32;
                        {
                            l_j_14 = 0i32;
                            'l6: loop {
                                if !(((((l_j_14) < (l_closedCount_4)) as i32)) != 0) { break 'l6; }
                                'c7: {
                                    if ((((({ let a_ = ({ let a_ = (c.m.r32(sym::gStgNav)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((l_j_14) as u32)).wrapping_mul(12)); (c.m.r32(a_) as i32) }) == ((((c.m.r16(((l_node_10).wrapping_add(32)).wrapping_add((((l_i_13) as u32)).wrapping_mul(2))) as i16)) as i32))) as i32)) != 0) {
                                        l_found_12 = 1i32;
                                        break 'l6;
                                    }
                                }
                                { let o_ = l_j_14; let n_ = (o_).wrapping_add(1); l_j_14 = n_; };
                            }
                        }
                        if (((((l_found_12) == (1i32)) as i32)) != 0) {
                            break 'c5;
                        }
                        {
                            l_j_14 = 0i32;
                            'l8: loop {
                                if !(((((l_j_14) < (l_openCount_5)) as i32)) != 0) { break 'l8; }
                                'c9: {
                                    if ((((({ let a_ = ({ let a_ = (c.m.r32(sym::gStgNav)).wrapping_add(12); c.m.r32(a_) }).wrapping_add((((l_j_14) as u32)).wrapping_mul(12)); (c.m.r32(a_) as i32) }) == ((((c.m.r16(((l_node_10).wrapping_add(32)).wrapping_add((((l_i_13) as u32)).wrapping_mul(2))) as i16)) as i32))) as i32)) != 0) {
                                        l_found_12 = 1i32;
                                        break 'l8;
                                    }
                                }
                                { let o_ = l_j_14; let n_ = (o_).wrapping_add(1); l_j_14 = n_; };
                            }
                        }
                        if (((((l_found_12) == (1i32)) as i32)) != 0) {
                            break 'c5;
                        }
                        { let v_ = (((c.m.r16(((l_node_10).wrapping_add(32)).wrapping_add((((l_i_13) as u32)).wrapping_mul(2))) as i16)) as i32); let a_ = ({ let a_ = (c.m.r32(sym::gStgNav)).wrapping_add(12); c.m.r32(a_) }).wrapping_add((((l_openCount_5) as u32)).wrapping_mul(12)); c.m.w32(a_, (v_) as u32); };
                        { let v_ = ({ let a_ = (c.m.r32(sym::gStgNav)).wrapping_add(16); c.m.r32(a_) }).wrapping_add(((((l_closedCount_4).wrapping_sub(1i32)) as u32)).wrapping_mul(12)); let a_ = (({ let a_ = (c.m.r32(sym::gStgNav)).wrapping_add(12); c.m.r32(a_) }).wrapping_add((((l_openCount_5) as u32)).wrapping_mul(12))).wrapping_add(4); c.m.w32(a_, v_); };
                        { let v_ = ((c.m.r32((l_e_8).wrapping_add(8)) as i32)).wrapping_add(1i32); let a_ = (({ let a_ = (c.m.r32(sym::gStgNav)).wrapping_add(12); c.m.r32(a_) }).wrapping_add((((l_openCount_5) as u32)).wrapping_mul(12))).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                        { let o_ = l_openCount_5; let n_ = (o_).wrapping_add(1); l_openCount_5 = n_; };
                    }
                    { let o_ = l_i_13; let n_ = (o_).wrapping_add(1); l_i_13 = n_; };
                }
            }
            { let a2_ = { let a_ = (c.m.r32(sym::gStgNav)).wrapping_add(12); c.m.r32(a_) }; F::StgNav_SortEntries(c, 0i32, (l_openCount_5).wrapping_sub(1i32), a2_) };
            l_e_8 = ({ let a_ = (c.m.r32(sym::gStgNav)).wrapping_add(12); c.m.r32(a_) }).wrapping_add(((((l_openCount_5).wrapping_sub(1i32)) as u32)).wrapping_mul(12));
        }
        if !(((((l_openCount_5) != (0i32)) as i32)) != 0) { break 'l1; }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgNav_FindPath", &PROBE_N, &[], true); }
    r_
}

pub fn StgNav_UnblockObj(c: &mut Ctx, mut p_idx_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgNav_UnblockObj", &PROBE_N, &[(p_idx_1) as u32], false); }
    let mut l_node_2: u32 = 0;
    let r_ = 'f: {
    if ((((!((F::StgNav_IsReady(c)) != 0)) as i32)) != 0) {
        break 'f;
    }
    {
        l_node_2 = { let a_ = ({ let a_ = c.m.r32(sym::gStgNav); c.m.r32(a_) }).wrapping_add(12); c.m.r32(a_) };
        'l1: loop {
            if !(((((l_node_2) != (({ let a_ = ({ let a_ = c.m.r32(sym::gStgNav); c.m.r32(a_) }).wrapping_add(12); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = c.m.r32(sym::gStgNav); c.m.r32(a_) }).wrapping_add(4); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48)))) as i32)) != 0) { break 'l1; }
            'c2: {
                if (((((c.m.r32((l_node_2).wrapping_add(16)) as i32)) & (2i32))) != 0) {
                    if (((((((((p_idx_1) == ((1i32).wrapping_neg())) as i32)) != 0) || (((((p_idx_1) == ((c.m.r32((l_node_2).wrapping_add(20)) as i32))) as i32)) != 0)) as i32)) != 0) {
                        { let r_ = (!(2i32)); let a_ = (l_node_2).wrapping_add(16); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                    }
                }
            }
            { let o_ = l_node_2; let n_ = (o_).wrapping_add(48); l_node_2 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgNav_UnblockObj", &PROBE_N, &[], true); }
    r_
}

pub fn StgNav_BlockObj(c: &mut Ctx, mut p_idx_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgNav_BlockObj", &PROBE_N, &[(p_idx_1) as u32], false); }
    let mut l_node_2: u32 = 0;
    let r_ = 'f: {
    if ((((!((F::StgNav_IsReady(c)) != 0)) as i32)) != 0) {
        break 'f;
    }
    {
        l_node_2 = { let a_ = ({ let a_ = c.m.r32(sym::gStgNav); c.m.r32(a_) }).wrapping_add(12); c.m.r32(a_) };
        'l1: loop {
            if !(((((l_node_2) != (({ let a_ = ({ let a_ = c.m.r32(sym::gStgNav); c.m.r32(a_) }).wrapping_add(12); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = c.m.r32(sym::gStgNav); c.m.r32(a_) }).wrapping_add(4); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48)))) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((((((c.m.r32((l_node_2).wrapping_add(20)) as i32)) != ((1i32).wrapping_neg())) as i32)) != 0) {
                    if (((((((((p_idx_1) == ((1i32).wrapping_neg())) as i32)) != 0) || (((((p_idx_1) == ((c.m.r32((l_node_2).wrapping_add(20)) as i32))) as i32)) != 0)) as i32)) != 0) {
                        { let r_ = 2i32; let a_ = (l_node_2).wrapping_add(16); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    }
                }
            }
            { let o_ = l_node_2; let n_ = (o_).wrapping_add(48); l_node_2 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgNav_BlockObj", &PROBE_N, &[], true); }
    r_
}
