#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn MdlTex_RebaseChain(c: &mut Ctx, mut p_single_1: i32, mut p_chain_2: u32, mut p_tbp_3: i32, mut p_cbp_4: i32, mut p_tbp2_5: i32, mut p_cbp2_6: i32, mut p_minTbp_7: u32, mut p_minCbp_8: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "MdlTex_RebaseChain", &PROBE_N, &[(p_single_1) as u32, (p_chain_2) as u32, (p_tbp_3) as u32, (p_cbp_4) as u32, (p_tbp2_5) as u32, (p_cbp2_6) as u32, (p_minTbp_7) as u32, (p_minCbp_8) as u32], false); }
    let mut l_p_9: u32 = 0;
    let mut l_mesh_10: u32 = 0;
    let mut l_v_11: u32 = 0;
    let r_ = 'f: {
    if (((((p_single_1) != (0i32)) as i32)) != 0) {
        {
            l_p_9 = p_chain_2;
            'l1: loop {
                if !(((((c.m.r32(l_p_9)) != (((1879048192i32) as u32))) as i32)) != 0) { break 'l1; }
                'c2: {
                    if (((((c.m.r32(l_p_9)) == (((1812299776i32) as u32))) as i32)) != 0) {
                        if (((((((c.m.r32((l_p_9).wrapping_add((((5i32) as u32)).wrapping_mul(4)))) & (((16383i32) as u32)))) < (p_minTbp_7)) as i32)) != 0) {
                            p_minTbp_7 = ((c.m.r32((l_p_9).wrapping_add((((5i32) as u32)).wrapping_mul(4)))) & (((16383i32) as u32)));
                        }
                        if ((((((((c.m.r32((l_p_9).wrapping_add((((6i32) as u32)).wrapping_mul(4)))).wrapping_shr((5i32) as u32)) & (((16383i32) as u32)))) < (p_minCbp_8)) as i32)) != 0) {
                            p_minCbp_8 = (((c.m.r32((l_p_9).wrapping_add((((6i32) as u32)).wrapping_mul(4)))).wrapping_shr((5i32) as u32)) & (((16383i32) as u32)));
                        }
                    }
                }
                { let o_ = l_p_9; let n_ = (o_).wrapping_add(4); l_p_9 = n_; };
            }
        }
        {
            l_p_9 = p_chain_2;
            'l3: loop {
                if !(((((c.m.r32(l_p_9)) != (((1879048192i32) as u32))) as i32)) != 0) { break 'l3; }
                'c4: {
                    if (((((c.m.r32(l_p_9)) == (((1812299776i32) as u32))) as i32)) != 0) {
                        l_v_11 = ((((c.m.r32((l_p_9).wrapping_add((((5i32) as u32)).wrapping_mul(4)))) & (((16383i32) as u32)))).wrapping_sub(p_minTbp_7)).wrapping_add(((p_tbp_3) as u32));
                        { let v_ = ((((c.m.r32((l_p_9).wrapping_add((((5i32) as u32)).wrapping_mul(4)))) & ((((!(16383i32))) as u32)))) | (l_v_11)); let a_ = (l_p_9).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                        l_v_11 = (((((c.m.r32((l_p_9).wrapping_add((((6i32) as u32)).wrapping_mul(4)))).wrapping_shr((5i32) as u32)) & (((16383i32) as u32)))).wrapping_sub(p_minCbp_8)).wrapping_add(((p_cbp_4) as u32));
                        { let v_ = ((((c.m.r32((l_p_9).wrapping_add((((6i32) as u32)).wrapping_mul(4)))) & (4294443039u32))) | ((l_v_11).wrapping_shl((5i32) as u32))); let a_ = (l_p_9).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                        l_v_11 = ((((c.m.r32((l_p_9).wrapping_add((((10i32) as u32)).wrapping_mul(4)))).wrapping_shr((5i32) as u32)) & (((16383i32) as u32)))).wrapping_add(((p_cbp2_6) as u32));
                        { let v_ = ((((c.m.r32((l_p_9).wrapping_add((((9i32) as u32)).wrapping_mul(4)))) & ((((!(16383i32))) as u32)))) | (((p_tbp2_5) as u32))); let a_ = (l_p_9).wrapping_add((((9i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                        { let v_ = ((((c.m.r32((l_p_9).wrapping_add((((10i32) as u32)).wrapping_mul(4)))) & (4294443039u32))) | ((l_v_11).wrapping_shl((5i32) as u32))); let a_ = (l_p_9).wrapping_add((((10i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    }
                }
                { let o_ = l_p_9; let n_ = (o_).wrapping_add(4); l_p_9 = n_; };
            }
        }
    } else {
        l_mesh_10 = p_chain_2;
        {
            'l5: loop {
                'c6: {
                    if (((((((c.m.r16((l_mesh_10).wrapping_add(8))) as i32)) != (0i32)) as i32)) != 0) {
                        {
                            l_p_9 = (l_mesh_10).wrapping_add(96);
                            'l7: loop {
                                if !(((((c.m.r32(l_p_9)) != (((1879048192i32) as u32))) as i32)) != 0) { break 'l7; }
                                'c8: {
                                    if (((((c.m.r32(l_p_9)) == (((1812299776i32) as u32))) as i32)) != 0) {
                                        if (((((((c.m.r32((l_p_9).wrapping_add((((5i32) as u32)).wrapping_mul(4)))) & (((16383i32) as u32)))) < (p_minTbp_7)) as i32)) != 0) {
                                            p_minTbp_7 = ((c.m.r32((l_p_9).wrapping_add((((5i32) as u32)).wrapping_mul(4)))) & (((16383i32) as u32)));
                                        }
                                        if ((((((((c.m.r32((l_p_9).wrapping_add((((6i32) as u32)).wrapping_mul(4)))).wrapping_shr((5i32) as u32)) & (((16383i32) as u32)))) < (p_minCbp_8)) as i32)) != 0) {
                                            p_minCbp_8 = (((c.m.r32((l_p_9).wrapping_add((((6i32) as u32)).wrapping_mul(4)))).wrapping_shr((5i32) as u32)) & (((16383i32) as u32)));
                                        }
                                    }
                                }
                                { let o_ = l_p_9; let n_ = (o_).wrapping_add(4); l_p_9 = n_; };
                            }
                        }
                    }
                    if (((((((c.m.r16((l_mesh_10).wrapping_add(6))) as i32)) != (0i32)) as i32)) != 0) {
                        break 'l5;
                    }
                    l_mesh_10 = (l_mesh_10).wrapping_add((((c.m.r32(l_mesh_10) as i32)) as u32));
                }
            }
        }
        l_mesh_10 = p_chain_2;
        {
            'l9: loop {
                'c10: {
                    if (((((((c.m.r16((l_mesh_10).wrapping_add(8))) as i32)) != (0i32)) as i32)) != 0) {
                        {
                            l_p_9 = (l_mesh_10).wrapping_add(96);
                            'l11: loop {
                                if !(((((c.m.r32(l_p_9)) != (((1879048192i32) as u32))) as i32)) != 0) { break 'l11; }
                                'c12: {
                                    if (((((c.m.r32(l_p_9)) == (((1812299776i32) as u32))) as i32)) != 0) {
                                        l_v_11 = ((((c.m.r32((l_p_9).wrapping_add((((5i32) as u32)).wrapping_mul(4)))) & (((16383i32) as u32)))).wrapping_sub(p_minTbp_7)).wrapping_add(((p_tbp_3) as u32));
                                        { let v_ = ((((c.m.r32((l_p_9).wrapping_add((((5i32) as u32)).wrapping_mul(4)))) & ((((!(16383i32))) as u32)))) | (l_v_11)); let a_ = (l_p_9).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                                        l_v_11 = (((((c.m.r32((l_p_9).wrapping_add((((6i32) as u32)).wrapping_mul(4)))).wrapping_shr((5i32) as u32)) & (((16383i32) as u32)))).wrapping_sub(p_minCbp_8)).wrapping_add(((p_cbp_4) as u32));
                                        { let v_ = ((((c.m.r32((l_p_9).wrapping_add((((6i32) as u32)).wrapping_mul(4)))) & (4294443039u32))) | ((l_v_11).wrapping_shl((5i32) as u32))); let a_ = (l_p_9).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                                        l_v_11 = ((((c.m.r32((l_p_9).wrapping_add((((10i32) as u32)).wrapping_mul(4)))).wrapping_shr((5i32) as u32)) & (((16383i32) as u32)))).wrapping_add(((p_cbp2_6) as u32));
                                        { let v_ = ((((c.m.r32((l_p_9).wrapping_add((((9i32) as u32)).wrapping_mul(4)))) & ((((!(16383i32))) as u32)))) | (((p_tbp2_5) as u32))); let a_ = (l_p_9).wrapping_add((((9i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                                        { let v_ = ((((c.m.r32((l_p_9).wrapping_add((((10i32) as u32)).wrapping_mul(4)))) & (4294443039u32))) | ((l_v_11).wrapping_shl((5i32) as u32))); let a_ = (l_p_9).wrapping_add((((10i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                                    }
                                }
                                { let o_ = l_p_9; let n_ = (o_).wrapping_add(4); l_p_9 = n_; };
                            }
                        }
                    }
                    if (((((((c.m.r16((l_mesh_10).wrapping_add(6))) as i32)) != (0i32)) as i32)) != 0) {
                        break 'l9;
                    }
                    l_mesh_10 = (l_mesh_10).wrapping_add((((c.m.r32(l_mesh_10) as i32)) as u32));
                }
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "MdlTex_RebaseChain", &PROBE_N, &[], true); }
    r_
}
