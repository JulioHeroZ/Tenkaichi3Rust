#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn StgOctree_Relocate(c: &mut Ctx, mut p_node_1: u32, mut p_base_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgOctree_Relocate", &PROBE_N, &[(p_node_1) as u32, (p_base_2) as u32], false); }
    let mut l_i_3: i32 = 0;
    let mut l_j_4: i32 = 0;
    let mut l_k_5: i32 = 0;
    let r_ = 'f: {
    if (((((c.m.r32(p_node_1)) != (((0i32) as u32))) as i32)) != 0) {
        { let v_ = (p_base_2).wrapping_add((((((c.m.r32(p_node_1)) as i32)) as u32)).wrapping_mul(4)); let a_ = p_node_1; c.m.w32(a_, v_); };
    }
    if (((((c.m.r32((p_node_1).wrapping_add(60))) != (((0i32) as u32))) as i32)) != 0) {
        { let v_ = (p_base_2).wrapping_add((((((c.m.r32((p_node_1).wrapping_add(60))) as i32)) as u32)).wrapping_mul(4)); let a_ = (p_node_1).wrapping_add(60); c.m.w32(a_, v_); };
    }
    {
        l_i_3 = 0i32;
        'l1: loop {
            if !(((((l_i_3) < ((c.m.r32((p_node_1).wrapping_add(56)) as i32))) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((((((((({ let a_ = (c.m.r32((p_node_1).wrapping_add(60))).wrapping_add((((l_i_3) as u32)).wrapping_mul(4)); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) && ((((((({ let a_ = (c.m.r32((p_node_1).wrapping_add(60))).wrapping_add((((l_i_3) as u32)).wrapping_mul(4)); c.m.r32(a_) }) as i32)) != ((1i32).wrapping_neg())) as i32)) != 0)) as i32)) != 0) {
                    { let v_ = (p_base_2).wrapping_add(((((({ let a_ = (c.m.r32((p_node_1).wrapping_add(60))).wrapping_add((((l_i_3) as u32)).wrapping_mul(4)); c.m.r32(a_) }) as i32)) as u32)).wrapping_mul(4)); let a_ = (c.m.r32((p_node_1).wrapping_add(60))).wrapping_add((((l_i_3) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                } else {
                    { let v_ = ((0i32) as u32); let a_ = (c.m.r32((p_node_1).wrapping_add(60))).wrapping_add((((l_i_3) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                }
            }
            { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
        }
    }
    {
        l_i_3 = 0i32;
        'l3: loop {
            if !(((((l_i_3) < (2i32)) as i32)) != 0) { break 'l3; }
            'c4: {
                {
                    l_j_4 = 0i32;
                    'l5: loop {
                        if !(((((l_j_4) < (2i32)) as i32)) != 0) { break 'l5; }
                        'c6: {
                            {
                                l_k_5 = 0i32;
                                'l7: loop {
                                    if !(((((l_k_5) < (2i32)) as i32)) != 0) { break 'l7; }
                                    'c8: {
                                        if (((((c.m.r32(((((p_node_1).wrapping_add(4)).wrapping_add((((l_i_3) as u32)).wrapping_mul(16))).wrapping_add((((l_j_4) as u32)).wrapping_mul(8))).wrapping_add((((l_k_5) as u32)).wrapping_mul(4)))) != (((0i32) as u32))) as i32)) != 0) {
                                            { let v_ = (p_base_2).wrapping_add((((((c.m.r32(((((p_node_1).wrapping_add(4)).wrapping_add((((l_i_3) as u32)).wrapping_mul(16))).wrapping_add((((l_j_4) as u32)).wrapping_mul(8))).wrapping_add((((l_k_5) as u32)).wrapping_mul(4)))) as i32)) as u32)).wrapping_mul(4)); let a_ = ((((p_node_1).wrapping_add(4)).wrapping_add((((l_i_3) as u32)).wrapping_mul(16))).wrapping_add((((l_j_4) as u32)).wrapping_mul(8))).wrapping_add((((l_k_5) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                                            { let a0_ = c.m.r32(((((p_node_1).wrapping_add(4)).wrapping_add((((l_i_3) as u32)).wrapping_mul(16))).wrapping_add((((l_j_4) as u32)).wrapping_mul(8))).wrapping_add((((l_k_5) as u32)).wrapping_mul(4))); F::StgOctree_Relocate(c, a0_, p_base_2) };
                                        }
                                    }
                                    { let o_ = l_k_5; let n_ = (o_).wrapping_add(1); l_k_5 = n_; };
                                }
                            }
                        }
                        { let o_ = l_j_4; let n_ = (o_).wrapping_add(1); l_j_4 = n_; };
                    }
                }
            }
            { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgOctree_Relocate", &PROBE_N, &[], true); }
    r_
}

pub fn BtlStage_Relocate(c: &mut Ctx, mut p_base_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_Relocate", &PROBE_N, &[(p_base_1) as u32], false); }
    let mut l_i_2: u32 = 0;
    let mut l_j_3: i32 = 0;
    let mut l_k_4: i32 = 0;
    let mut l_t_5: u32 = 0;
    let mut l_a_6: u32 = 0;
    let mut l_a1_7: u32 = 0;
    let mut l_a2_8: u32 = 0;
    let mut l_o8_9: i32 = 0;
    let mut l_oC_10: i32 = 0;
    let mut l_b_11: u32 = 0;
    let mut l_b1_12: u32 = 0;
    let mut l_c_13: u32 = 0;
    let mut l_d_14: u32 = 0;
    let mut l_o04_15: i32 = 0;
    let mut l_o0C_16: i32 = 0;
    let mut l_o10_17: i32 = 0;
    let mut l_o18_18: i32 = 0;
    let mut l_d1_19: u32 = 0;
    let mut l_e_20: u32 = 0;
    let r_ = 'f: {
    { let v_ = p_base_1; let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(92); c.m.w32(a_, v_); };
    if (((((c.m.r32(sym::gBtlStage)) != (((0i32) as u32))) as i32)) != 0) {
        { let v_ = (p_base_1).wrapping_add(((((({ let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(20); c.m.r32(a_) }) as i32)) as u32)).wrapping_mul(4)); let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(20); c.m.w32(a_, v_); };
        { let v_ = (p_base_1).wrapping_add(((((({ let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(28); c.m.r32(a_) }) as i32)) as u32)).wrapping_mul(4)); let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(28); c.m.w32(a_, v_); };
        { let v_ = (p_base_1).wrapping_add(((((({ let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(36); c.m.r32(a_) }) as i32)) as u32)).wrapping_mul(4)); let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(36); c.m.w32(a_, v_); };
        { let v_ = (p_base_1).wrapping_add(((((({ let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(44); c.m.r32(a_) }) as i32)) as u32)).wrapping_mul(4)); let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(44); c.m.w32(a_, v_); };
        { let v_ = (p_base_1).wrapping_add(((((({ let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(52); c.m.r32(a_) }) as i32)) as u32)).wrapping_mul(4)); let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(52); c.m.w32(a_, v_); };
        { let v_ = (p_base_1).wrapping_add(((((({ let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(60); c.m.r32(a_) }) as i32)) as u32)).wrapping_mul(4)); let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(60); c.m.w32(a_, v_); };
        { let v_ = (p_base_1).wrapping_add(((((({ let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(64); c.m.r32(a_) }) as i32)) as u32)).wrapping_mul(4)); let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(64); c.m.w32(a_, v_); };
        { let v_ = (p_base_1).wrapping_add(((((({ let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(72); c.m.r32(a_) }) as i32)) as u32)).wrapping_mul(4)); let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(72); c.m.w32(a_, v_); };
        { let v_ = (p_base_1).wrapping_add(((((({ let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(80); c.m.r32(a_) }) as i32)) as u32)).wrapping_mul(4)); let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(80); c.m.w32(a_, v_); };
        {
            l_i_2 = ((0i32) as u32);
            'l1: loop {
                if !(((((l_i_2) < ({ let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(16); c.m.r32(a_) })) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_t_5 = ({ let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(20); c.m.r32(a_) }).wrapping_add((l_i_2).wrapping_mul(16));
                    { let v_ = (p_base_1).wrapping_add((((((c.m.r32((l_t_5).wrapping_add(4))) as i32)) as u32)).wrapping_mul(4)); let a_ = (l_t_5).wrapping_add(4); c.m.w32(a_, v_); };
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        {
            l_i_2 = ((0i32) as u32);
            'l3: loop {
                if !(((((l_i_2) < ({ let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(24); c.m.r32(a_) })) as i32)) != 0) { break 'l3; }
                'c4: {
                    l_a_6 = ({ let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(28); c.m.r32(a_) }).wrapping_add((l_i_2).wrapping_mul(8));
                    { let v_ = (p_base_1).wrapping_add((((((c.m.r32((l_a_6).wrapping_add(4))) as i32)) as u32)).wrapping_mul(4)); let a_ = (l_a_6).wrapping_add(4); c.m.w32(a_, v_); };
                    {
                        l_j_3 = 0i32;
                        'l5: loop {
                            if !(((((l_j_3) < (((c.m.r16((l_a_6).wrapping_add(2))) as i32))) as i32)) != 0) { break 'l5; }
                            'c6: {
                                l_a1_7 = (c.m.r32((l_a_6).wrapping_add(4))).wrapping_add((((l_j_3) as u32)).wrapping_mul(16));
                                if (((((c.m.r32((l_a1_7).wrapping_add(8))) != (((0i32) as u32))) as i32)) != 0) {
                                    { let v_ = (p_base_1).wrapping_add((((((c.m.r32((l_a1_7).wrapping_add(8))) as i32)) as u32)).wrapping_mul(4)); let a_ = (l_a1_7).wrapping_add(8); c.m.w32(a_, v_); };
                                }
                                { let v_ = (p_base_1).wrapping_add((((((c.m.r32((l_a1_7).wrapping_add(4))) as i32)) as u32)).wrapping_mul(4)); let a_ = (l_a1_7).wrapping_add(4); c.m.w32(a_, v_); };
                                {
                                    l_k_4 = 0i32;
                                    'l7: loop {
                                        if !(((((l_k_4) < (((c.m.r16((l_a1_7).wrapping_add(2))) as i32))) as i32)) != 0) { break 'l7; }
                                        'c8: {
                                            l_a2_8 = (c.m.r32((l_a1_7).wrapping_add(4))).wrapping_add((((l_k_4) as u32)).wrapping_mul(16));
                                            l_o8_9 = ((c.m.r32((l_a2_8).wrapping_add(8))) as i32);
                                            l_oC_10 = ((c.m.r32((l_a2_8).wrapping_add(12))) as i32);
                                            { let v_ = (p_base_1).wrapping_add((((l_o8_9) as u32)).wrapping_mul(4)); let a_ = (l_a2_8).wrapping_add(8); c.m.w32(a_, v_); };
                                            { let v_ = (p_base_1).wrapping_add((((l_oC_10) as u32)).wrapping_mul(4)); let a_ = (l_a2_8).wrapping_add(12); c.m.w32(a_, v_); };
                                        }
                                        { let o_ = l_k_4; let n_ = (o_).wrapping_add(1); l_k_4 = n_; };
                                    }
                                }
                            }
                            { let o_ = l_j_3; let n_ = (o_).wrapping_add(1); l_j_3 = n_; };
                        }
                    }
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        {
            l_i_2 = ((0i32) as u32);
            'l9: loop {
                if !(((((l_i_2) < ({ let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(32); c.m.r32(a_) })) as i32)) != 0) { break 'l9; }
                'c10: {
                    l_b_11 = ({ let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(36); c.m.r32(a_) }).wrapping_add((l_i_2).wrapping_mul(8));
                    { let v_ = (p_base_1).wrapping_add((((((c.m.r32((l_b_11).wrapping_add(4))) as i32)) as u32)).wrapping_mul(4)); let a_ = (l_b_11).wrapping_add(4); c.m.w32(a_, v_); };
                    {
                        l_j_3 = 0i32;
                        'l11: loop {
                            if !(((((((l_j_3) as u32)) < (c.m.r32(l_b_11))) as i32)) != 0) { break 'l11; }
                            'c12: {
                                l_b1_12 = (c.m.r32((l_b_11).wrapping_add(4))).wrapping_add((((l_j_3) as u32)).wrapping_mul(32));
                                if (((((c.m.r32((l_b1_12).wrapping_add(8))) != (((0i32) as u32))) as i32)) != 0) {
                                    { let v_ = (p_base_1).wrapping_add((((((c.m.r32((l_b1_12).wrapping_add(8))) as i32)) as u32)).wrapping_mul(4)); let a_ = (l_b1_12).wrapping_add(8); c.m.w32(a_, v_); };
                                }
                                { let v_ = (p_base_1).wrapping_add((((((c.m.r32((l_b1_12).wrapping_add(4))) as i32)) as u32)).wrapping_mul(4)); let a_ = (l_b1_12).wrapping_add(4); c.m.w32(a_, v_); };
                                {
                                    l_k_4 = 0i32;
                                    'l13: loop {
                                        if !(((((l_k_4) < (((c.m.r16(l_b1_12)) as i32))) as i32)) != 0) { break 'l13; }
                                        'c14: {
                                            { let v_ = (p_base_1).wrapping_add(((((({ let a_ = (c.m.r32((l_b1_12).wrapping_add(4))).wrapping_add((((l_k_4) as u32)).wrapping_mul(4)); c.m.r32(a_) }) as i32)) as u32)).wrapping_mul(4)); let a_ = (c.m.r32((l_b1_12).wrapping_add(4))).wrapping_add((((l_k_4) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                                        }
                                        { let o_ = l_k_4; let n_ = (o_).wrapping_add(1); l_k_4 = n_; };
                                    }
                                }
                            }
                            { let o_ = l_j_3; let n_ = (o_).wrapping_add(1); l_j_3 = n_; };
                        }
                    }
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        {
            l_i_2 = ((0i32) as u32);
            'l15: loop {
                if !(((((l_i_2) < ({ let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(48); c.m.r32(a_) })) as i32)) != 0) { break 'l15; }
                'c16: {
                    l_c_13 = ({ let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(52); c.m.r32(a_) }).wrapping_add((l_i_2).wrapping_mul(80));
                    { let v_ = (p_base_1).wrapping_add((((((c.m.r32((l_c_13).wrapping_add(68))) as i32)) as u32)).wrapping_mul(4)); let a_ = (l_c_13).wrapping_add(68); c.m.w32(a_, v_); };
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        {
            l_i_2 = ((0i32) as u32);
            'l17: loop {
                if !(((((l_i_2) < ({ let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(56); c.m.r32(a_) })) as i32)) != 0) { break 'l17; }
                'c18: {
                    l_d_14 = ({ let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(60); c.m.r32(a_) }).wrapping_add((l_i_2).wrapping_mul(64));
                    {
                        l_o04_15 = ((c.m.r32((l_d_14).wrapping_add(4))) as i32);
                        l_o0C_16 = ((c.m.r32((l_d_14).wrapping_add(12))) as i32);
                        l_o10_17 = ((c.m.r32((l_d_14).wrapping_add(16))) as i32);
                        l_o18_18 = ((c.m.r32((l_d_14).wrapping_add(24))) as i32);
                        { let v_ = (p_base_1).wrapping_add((((l_o04_15) as u32)).wrapping_mul(4)); let a_ = (l_d_14).wrapping_add(4); c.m.w32(a_, v_); };
                        { let v_ = (p_base_1).wrapping_add((((l_o0C_16) as u32)).wrapping_mul(4)); let a_ = (l_d_14).wrapping_add(12); c.m.w32(a_, v_); };
                        { let v_ = (p_base_1).wrapping_add((((l_o10_17) as u32)).wrapping_mul(4)); let a_ = (l_d_14).wrapping_add(16); c.m.w32(a_, v_); };
                        { let v_ = (p_base_1).wrapping_add((((l_o18_18) as u32)).wrapping_mul(4)); let a_ = (l_d_14).wrapping_add(24); c.m.w32(a_, v_); };
                    }
                    {
                        l_j_3 = 0i32;
                        'l19: loop {
                            if !(((((l_j_3) < ((c.m.r32((l_d_14).wrapping_add(20)) as i32))) as i32)) != 0) { break 'l19; }
                            'c20: {
                                l_d1_19 = (c.m.r32((l_d_14).wrapping_add(24))).wrapping_add((((l_j_3) as u32)).wrapping_mul(16));
                                { let v_ = (p_base_1).wrapping_add((((((c.m.r32((l_d1_19).wrapping_add(8))) as i32)) as u32)).wrapping_mul(4)); let a_ = (l_d1_19).wrapping_add(8); c.m.w32(a_, v_); };
                            }
                            { let o_ = l_j_3; let n_ = (o_).wrapping_add(1); l_j_3 = n_; };
                        }
                    }
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        {
            l_i_2 = ((0i32) as u32);
            'l21: loop {
                if !(((((l_i_2) < ({ let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(68); c.m.r32(a_) })) as i32)) != 0) { break 'l21; }
                'c22: {
                    l_e_20 = ({ let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(72); c.m.r32(a_) }).wrapping_add((l_i_2).wrapping_mul(80));
                    { let v_ = (p_base_1).wrapping_add((((((c.m.r32((l_e_20).wrapping_add(40))) as i32)) as u32)).wrapping_mul(4)); let a_ = (l_e_20).wrapping_add(40); c.m.w32(a_, v_); };
                    {
                        l_j_3 = 0i32;
                        'l23: loop {
                            if !(((((((l_j_3) as u32)) < (c.m.r32((l_e_20).wrapping_add(36)))) as i32)) != 0) { break 'l23; }
                            'c24: {
                                { let v_ = (p_base_1).wrapping_add(((((({ let a_ = (c.m.r32((l_e_20).wrapping_add(40))).wrapping_add((((l_j_3) as u32)).wrapping_mul(4)); c.m.r32(a_) }) as i32)) as u32)).wrapping_mul(4)); let a_ = (c.m.r32((l_e_20).wrapping_add(40))).wrapping_add((((l_j_3) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                            }
                            { let o_ = l_j_3; let n_ = (o_).wrapping_add(1); l_j_3 = n_; };
                        }
                    }
                    {
                        l_j_3 = 0i32;
                        'l25: loop {
                            if !(((((l_j_3) < (2i32)) as i32)) != 0) { break 'l25; }
                            'c26: {
                                { let v_ = (p_base_1).wrapping_add((((((c.m.r32(((l_e_20).wrapping_add(12)).wrapping_add((((l_j_3) as u32)).wrapping_mul(4)))) as i32)) as u32)).wrapping_mul(4)); let a_ = ((l_e_20).wrapping_add(12)).wrapping_add((((l_j_3) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                                { let v_ = (p_base_1).wrapping_add((((((c.m.r32(((l_e_20).wrapping_add(20)).wrapping_add((((l_j_3) as u32)).wrapping_mul(4)))) as i32)) as u32)).wrapping_mul(4)); let a_ = ((l_e_20).wrapping_add(20)).wrapping_add((((l_j_3) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                                { let v_ = (p_base_1).wrapping_add((((((c.m.r32(((l_e_20).wrapping_add(28)).wrapping_add((((l_j_3) as u32)).wrapping_mul(4)))) as i32)) as u32)).wrapping_mul(4)); let a_ = ((l_e_20).wrapping_add(28)).wrapping_add((((l_j_3) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                            }
                            { let o_ = l_j_3; let n_ = (o_).wrapping_add(1); l_j_3 = n_; };
                        }
                    }
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        { let a0_ = { let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(80); c.m.r32(a_) }; F::StgOctree_Relocate(c, a0_, p_base_1) };
        { let a0_ = (c.m.r32(sym::gBtlStage)).wrapping_add(88); let a2_ = ({ let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(92); c.m.r32(a_) }).wrapping_add(((({ let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(84); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); F::Res_RelocateOffsets(c, a0_, p_base_1, a2_) };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_Relocate", &PROBE_N, &[], true); }
    r_
}
