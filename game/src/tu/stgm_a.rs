#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn StgModel_DrawAnims(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgModel_DrawAnims", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(96) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 96);
    let mut l_count_1: i32 = 0;
    let mut l_i_2: i32 = 0;
    let mut l_j_3: i32 = 0;
    let mut l_anim_4: i32 = 0;
    let mut l_uploaded_5: i32 = 0;
    let mut l_pkt_6: u32 = 0;
    let mut l_data_7: u32 = 0;
    let mut l_def_8: u32 = 0;
    let mut l_tex_9: u32 = 0;
    let mut l_node_10: u32 = 0;
    let mut l_mdl_11: u32 = 0;
    let mut l_track_12: u32 = 0;
    let r_ = 'f: {
        if (((((c.m.r32(sym::gStgAnimTable)) == (((0i32) as u32))) as i32)) != 0) {
            break 'f;
        }
        if (((({ let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(8); (c.m.r32(a_) as i32) }) & (1i32))) != 0) {
            break 'f;
        }
        l_pkt_6 = F::Vu1Pkt_LoadProg8(c);
        F::Vu0Screen_StoreMtx(c, (l_pkt_6).wrapping_add(((176i32) as u32)));
        F::Vu0Clip_StoreMtx(c, (l_pkt_6).wrapping_add(((240i32) as u32)));
        F::StgModel_SetAnimDrawEnv(c);
        l_count_1 = { let a0_ = c.m.r32(sym::gStgAnimTable); F::StgOfsTable_GetCount(c, a0_) };
        l_data_7 = { let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(4); c.m.r32(a_) };
        l_anim_4 = 1i32;
        {
            l_i_2 = 0i32;
            'l1: loop {
                if !(((((l_i_2) < (l_count_1)) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_uploaded_5 = 0i32;
                    l_tex_9 = { let a0_ = c.m.r32(sym::gStgAnimTable); ((((F::StgOfsTable_Get(c, a0_, l_i_2)) as i64 as u64)) as u32) };
                    l_node_10 = { let a0_ = c.m.r32(sym::gStgAnimTable); ((((F::StgOfsTable_Get(c, a0_, (l_i_2).wrapping_add(1i32))) as i64 as u64)) as u32) };
                    l_mdl_11 = (l_node_10).wrapping_add((rt::div_u32(c.m.r32((l_node_10).wrapping_add((((0i32) as u32)).wrapping_mul(4))), ((4i32) as u32))).wrapping_mul(((4i32) as u32)));
                    l_track_12 = { let a0_ = c.m.r32(sym::gStgAnimTable); ((((F::StgOfsTable_Get(c, a0_, (l_i_2).wrapping_add(2i32))) as i64 as u64)) as u32) };
                    l_def_8 = c.m.r32((l_data_7).wrapping_add(12));
                    {
                        l_j_3 = 0i32;
                        'l3: loop {
                            if !(((((l_j_3) < ((c.m.r32((l_data_7).wrapping_add(8)) as i32))) as i32)) != 0) { break 'l3; }
                            'c4: {
                                if ((((((c.m.r32((l_def_8).wrapping_add(8)) as i32)) != (l_anim_4)) as i32)) != 0) {
                                    { let o_ = l_def_8; let n_ = (o_).wrapping_add(64); l_def_8 = n_; };
                                    break 'c4;
                                }
                                if ((((((((!(((((c.m.r32((l_def_8).wrapping_add(4)) as i32)) & (1073741824i32))) != 0)) as i32)) != 0) && ((F::BtlStage_IsObjBroken(c, l_j_3)) != 0)) as i32)) != 0) {
                                    { let o_ = l_def_8; let n_ = (o_).wrapping_add(64); l_def_8 = n_; };
                                    break 'c4;
                                }
                                { let v_ = rt::fdiv(rt::fmul(F32(c.m.r32((l_def_8).wrapping_add(48))), F32(0x40490fda)), F32(0x43340000)); let a_ = fp.wrapping_add(80); c.m.w32(a_, (v_).0); };
                                { let v_ = rt::fdiv(rt::fmul(F32(c.m.r32(((l_def_8).wrapping_add(48)).wrapping_add(4))), F32(0x40490fda)), F32(0x43340000)); let a_ = (fp.wrapping_add(80)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                                { let v_ = rt::fdiv(rt::fmul(F32(c.m.r32(((l_def_8).wrapping_add(48)).wrapping_add(8))), F32(0x40490fda)), F32(0x43340000)); let a_ = (fp.wrapping_add(80)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                                { let v_ = F32(0x3f800000); let a_ = (fp.wrapping_add(80)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                                { let v_ = F32(c.m.r32((l_def_8).wrapping_add(32))); let a_ = fp.wrapping_add(64); c.m.w32(a_, (v_).0); };
                                { let v_ = F32(c.m.r32(((l_def_8).wrapping_add(32)).wrapping_add(4))); let a_ = (fp.wrapping_add(64)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                                { let v_ = F32(c.m.r32(((l_def_8).wrapping_add(32)).wrapping_add(8))); let a_ = (fp.wrapping_add(64)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                                { let v_ = F32(0x3f800000); let a_ = (fp.wrapping_add(64)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                                { let v_ = F32(0x3f800000); let a_ = ((l_def_8).wrapping_add(32)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                                F::Vu0Cur_LoadIdentity(c);
                                F::Vu0Cur_RotateZXY(c, fp.wrapping_add(80));
                                F::Vu0Cur_Translate(c, fp.wrapping_add(64));
                                F::Vu0Cur_StoreMtx(c, fp);
                                { let a3_ = F32(c.m.r32((l_def_8).wrapping_add(28))); F::Vu1Node_Animate(c, l_mdl_11, l_track_12, fp, a3_) };
                                { let o_ = l_def_8; let n_ = (o_).wrapping_add(64); l_def_8 = n_; };
                                if ((((!((l_uploaded_5) != 0)) as i32)) != 0) {
                                    F::TexFile_UploadPacked(c, l_tex_9, 13440i32, 14080i32);
                                    l_uploaded_5 = 1i32;
                                }
                                F::Vu1Node_Draw(c, l_mdl_11);
                            }
                            { let o_ = l_j_3; let n_ = (o_).wrapping_add(1); l_j_3 = n_; };
                        }
                    }
                }
                { { let r_ = 3i32; let n_ = (l_i_2).wrapping_add(r_); l_i_2 = n_; }; { let o_ = l_anim_4; let n_ = (o_).wrapping_add(1); l_anim_4 = n_; }; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "StgModel_DrawAnims", &PROBE_N, &[], true); }
    r_
}

pub fn StgModel_CullCell(c: &mut Ctx, mut p_cell_1: u32, mut p_size_2: F32, mut p_vis_3: i32, mut p_fr_4: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgModel_CullCell", &PROBE_N, &[(p_cell_1) as u32, (p_size_2).0, (p_vis_3) as u32, (p_fr_4) as u32], false); }
    let mut l_tbl_5: u32 = 0;
    let mut l_half_6: F32 = F32(0);
    let mut l_i_7: i32 = 0;
    let mut l_j_8: i32 = 0;
    let mut l_k_9: i32 = 0;
    let mut l_part_10: u32 = 0;
    let mut l_part_11: u32 = 0;
    let r_ = 'f: {
    l_tbl_5 = { let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(44); c.m.r32(a_) };
    l_half_6 = rt::fdiv(rt::fadd(p_size_2, p_size_2), rt::i2f((1i32).wrapping_shl(((c.m.r32((p_cell_1).wrapping_add(52)) as i32)) as u32)));
    if (((((p_vis_3) == (1i32)) as i32)) != 0) {
        p_vis_3 = { let a1_ = F32(c.m.r32((p_cell_1).wrapping_add(40))); let a2_ = F32(c.m.r32((p_cell_1).wrapping_add(44))); let a3_ = F32(c.m.r32((p_cell_1).wrapping_add(48))); F::StgFrustum_TestBox(c, p_fr_4, a1_, a2_, a3_, l_half_6) };
    }
    { let v_ = p_vis_3; let a_ = (p_cell_1).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
    if (((((p_vis_3) == (2i32)) as i32)) != 0) {
        break 'f;
    }
    if (((((p_vis_3) == (0i32)) as i32)) != 0) {
        {
            l_i_7 = 0i32;
            'l1: loop {
                if !(((((l_i_7) < ((c.m.r32((p_cell_1).wrapping_add(56)) as i32))) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_part_10 = { let a_ = (c.m.r32((p_cell_1).wrapping_add(60))).wrapping_add((((l_i_7) as u32)).wrapping_mul(4)); c.m.r32(a_) };
                    if (((((l_part_10) == (((0i32) as u32))) as i32)) != 0) {
                        break 'c2;
                    }
                    if ((((!((((((((c.m.r16((l_part_10).wrapping_add(2))) as i32)) & (1i32))) as u16)) != 0)) as i32)) != 0) {
                        break 'c2;
                    }
                    {
                        l_j_8 = 0i32;
                        'l3: loop {
                            if !(((((l_j_8) < (((c.m.r16(l_part_10)) as i32))) as i32)) != 0) { break 'l3; }
                            'c4: {
                                { let v_ = ((1i32) as u8); let a_ = (l_tbl_5).wrapping_add((({ let a_ = { let a_ = (c.m.r32((l_part_10).wrapping_add(4))).wrapping_add((((l_j_8) as u32)).wrapping_mul(4)); c.m.r32(a_) }; c.m.r16(a_) }) as u32)); c.m.w8(a_, v_); };
                            }
                            { let o_ = l_j_8; let n_ = (o_).wrapping_add(1); l_j_8 = n_; };
                        }
                    }
                }
                { let o_ = l_i_7; let n_ = (o_).wrapping_add(1); l_i_7 = n_; };
            }
        }
    } else {
        {
            l_i_7 = 0i32;
            'l5: loop {
                if !(((((l_i_7) < ((c.m.r32((p_cell_1).wrapping_add(56)) as i32))) as i32)) != 0) { break 'l5; }
                'c6: {
                    l_part_11 = { let a_ = (c.m.r32((p_cell_1).wrapping_add(60))).wrapping_add((((l_i_7) as u32)).wrapping_mul(4)); c.m.r32(a_) };
                    if (((((l_part_11) == (((0i32) as u32))) as i32)) != 0) {
                        break 'c6;
                    }
                    if ((((!((((((((c.m.r16((l_part_11).wrapping_add(2))) as i32)) & (1i32))) as u16)) != 0)) as i32)) != 0) {
                        break 'c6;
                    }
                    if (((((F::StgFrustum_TestPart(c, l_part_11, p_fr_4)) == (2i32)) as i32)) != 0) {
                        break 'c6;
                    }
                    {
                        l_j_8 = 0i32;
                        'l7: loop {
                            if !(((((l_j_8) < (((c.m.r16(l_part_11)) as i32))) as i32)) != 0) { break 'l7; }
                            'c8: {
                                { let v_ = ((1i32) as u8); let a_ = (l_tbl_5).wrapping_add((({ let a_ = { let a_ = (c.m.r32((l_part_11).wrapping_add(4))).wrapping_add((((l_j_8) as u32)).wrapping_mul(4)); c.m.r32(a_) }; c.m.r16(a_) }) as u32)); c.m.w8(a_, v_); };
                            }
                            { let o_ = l_j_8; let n_ = (o_).wrapping_add(1); l_j_8 = n_; };
                        }
                    }
                }
                { let o_ = l_i_7; let n_ = (o_).wrapping_add(1); l_i_7 = n_; };
            }
        }
    }
    {
        l_k_9 = 0i32;
        'l9: loop {
            if !(((((l_k_9) < (2i32)) as i32)) != 0) { break 'l9; }
            'c10: {
                {
                    l_j_8 = 0i32;
                    'l11: loop {
                        if !(((((l_j_8) < (2i32)) as i32)) != 0) { break 'l11; }
                        'c12: {
                            {
                                l_i_7 = 0i32;
                                'l13: loop {
                                    if !(((((l_i_7) < (2i32)) as i32)) != 0) { break 'l13; }
                                    'c14: {
                                        if (((((c.m.r32(((((p_cell_1).wrapping_add(4)).wrapping_add((((l_k_9) as u32)).wrapping_mul(16))).wrapping_add((((l_j_8) as u32)).wrapping_mul(8))).wrapping_add((((l_i_7) as u32)).wrapping_mul(4)))) != (((0i32) as u32))) as i32)) != 0) {
                                            { let a0_ = c.m.r32(((((p_cell_1).wrapping_add(4)).wrapping_add((((l_k_9) as u32)).wrapping_mul(16))).wrapping_add((((l_j_8) as u32)).wrapping_mul(8))).wrapping_add((((l_i_7) as u32)).wrapping_mul(4))); F::StgModel_CullCell(c, a0_, p_size_2, p_vis_3, p_fr_4) };
                                        }
                                    }
                                    { let o_ = l_i_7; let n_ = (o_).wrapping_add(1); l_i_7 = n_; };
                                }
                            }
                        }
                        { let o_ = l_j_8; let n_ = (o_).wrapping_add(1); l_j_8 = n_; };
                    }
                }
            }
            { let o_ = l_k_9; let n_ = (o_).wrapping_add(1); l_k_9 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgModel_CullCell", &PROBE_N, &[], true); }
    r_
}

pub fn StgModel_Cull(c: &mut Ctx, mut p_view_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgModel_Cull", &PROBE_N, &[(p_view_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(208) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 208);
    let mut l_tbl_2: u32 = 0;
    let mut l_list_3: u32 = 0;
    let mut l_root_4: u32 = 0;
    let mut l_part_5: u32 = 0;
    let mut l_i_6: u32 = 0;
    let mut l_j_7: i32 = 0;
    let mut l_parts_8: u32 = 0;
    let r_ = 'f: {
        if (((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((8192i32) as u64)))) != 0) {
            break 'f;
        }
        l_tbl_2 = { let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(44); c.m.r32(a_) };
        l_list_3 = { let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(36); c.m.r32(a_) };
        l_root_4 = { let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(80); c.m.r32(a_) };
        { let _ = { let a2_ = (({ let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(40); (c.m.r32(a_) as i32) }) as u32); F::memset(c, l_tbl_2, 0i32, a2_) }; };
        if (((({ let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(8); (c.m.r32(a_) as i32) }) & (1i32))) != 0) {
            break 'f;
        }
        F::StgFrustum_Build(c, p_view_1, fp);
        { let a1_ = { let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(76); F32(c.m.r32(a_)) }; F::StgModel_CullCell(c, l_root_4, a1_, 1i32, fp) };
        if (((((c.m.r32(l_list_3)) != (((0i32) as u32))) as i32)) != 0) {
            l_parts_8 = c.m.r32((l_list_3).wrapping_add(4));
            {
                l_i_6 = ((0i32) as u32);
                'l1: loop {
                    if !(((((l_i_6) < (c.m.r32(l_list_3))) as i32)) != 0) { break 'l1; }
                    'c2: {
                        l_part_5 = (l_parts_8).wrapping_add((l_i_6).wrapping_mul(32));
                        {
                            l_j_7 = 0i32;
                            'l3: loop {
                                if !(((((l_j_7) < (((c.m.r16(l_part_5)) as i32))) as i32)) != 0) { break 'l3; }
                                'c4: {
                                    { let v_ = ((1i32) as u8); let a_ = (l_tbl_2).wrapping_add((({ let a_ = { let a_ = (c.m.r32((l_part_5).wrapping_add(4))).wrapping_add((((l_j_7) as u32)).wrapping_mul(4)); c.m.r32(a_) }; c.m.r16(a_) }) as u32)); c.m.w8(a_, v_); };
                                }
                                { let o_ = l_j_7; let n_ = (o_).wrapping_add(1); l_j_7 = n_; };
                            }
                        }
                    }
                    { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; };
                }
            }
        }
        F::StgModel_FadeDebris(c);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "StgModel_Cull", &PROBE_N, &[], true); }
    r_
}

pub fn StgModel_FadeDebris(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgModel_FadeDebris", &PROBE_N, &[], false); }
    let mut l_objs_1: u32 = 0;
    let mut l_tbl_2: u32 = 0;
    let mut l_mesh_3: u32 = 0;
    let mut l_piece_4: u32 = 0;
    let mut l_node_5: u32 = 0;
    let mut l_i_6: u32 = 0;
    let mut l_j_7: u32 = 0;
    let mut l_k_8: i32 = 0;
    let mut l_ratio_9: F32 = F32(0);
    let mut l_a_10: F32 = F32(0);
    let mut l_alpha_11: u8 = 0;
    let r_ = 'f: {
    l_objs_1 = { let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(72); c.m.r32(a_) };
    l_tbl_2 = { let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(44); c.m.r32(a_) };
    {
        l_i_6 = ((0i32) as u32);
        'l1: loop {
            if !(((((l_i_6) < ({ let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(68); c.m.r32(a_) })) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((((!(((((c.m.r32((l_objs_1).wrapping_add((l_i_6).wrapping_mul(80))) as i32)) & (2i32))) != 0)) as i32)) != 0) {
                    break 'c2;
                }
                l_ratio_9 = { let a0_ = rt::i2f((c.m.r32(((l_objs_1).wrapping_add((l_i_6).wrapping_mul(80))).wrapping_add(48)) as i32)); let a1_ = rt::i2f((c.m.r32(((l_objs_1).wrapping_add((l_i_6).wrapping_mul(80))).wrapping_add(44)) as i32)); F::Stg_FadeRatio(c, a0_, a1_) };
                {
                    l_j_7 = ((0i32) as u32);
                    'l3: loop {
                        if !(((((l_j_7) < (c.m.r32(((l_objs_1).wrapping_add((l_i_6).wrapping_mul(80))).wrapping_add(36)))) as i32)) != 0) { break 'l3; }
                        'c4: {
                            l_piece_4 = { let a_ = (c.m.r32(((l_objs_1).wrapping_add((l_i_6).wrapping_mul(80))).wrapping_add(40))).wrapping_add((l_j_7).wrapping_mul(4)); c.m.r32(a_) };
                            l_node_5 = c.m.r32((l_piece_4).wrapping_add(8));
                            if (((((l_node_5) == (((0i32) as u32))) as i32)) != 0) {
                                break 'c4;
                            }
                            if ((((((c.m.r32((l_node_5).wrapping_add(76)) as i32)) < (0i32)) as i32)) != 0) {
                                break 'c4;
                            }
                            l_a_10 = F::Stg_FadeByCamDist(c, (l_node_5).wrapping_add(48), l_ratio_9);
                            if (((rt::flt(l_a_10, F32(0x3f800000)) as i32)) != 0) {
                                l_alpha_11 = ((rt::f2u(rt::fmul(l_a_10, F32(0x43000000)))) as u8);
                            } else {
                                l_alpha_11 = ((128i32) as u8);
                            }
                            if (((((((l_alpha_11) as i32)) == (0i32)) as i32)) != 0) {
                                break 'c4;
                            }
                            {
                                l_k_8 = 0i32;
                                'l5: loop {
                                    if !(((((l_k_8) < (((c.m.r16(l_piece_4)) as i32))) as i32)) != 0) { break 'l5; }
                                    'c6: {
                                        l_mesh_3 = { let a_ = (c.m.r32((l_piece_4).wrapping_add(4))).wrapping_add((((l_k_8) as u32)).wrapping_mul(4)); c.m.r32(a_) };
                                        { let v_ = l_alpha_11; let a_ = (l_mesh_3).wrapping_add(4); c.m.w8(a_, v_); };
                                        { let v_ = ((1i32) as u8); let a_ = (l_tbl_2).wrapping_add(((c.m.r16(l_mesh_3)) as u32)); c.m.w8(a_, v_); };
                                    }
                                    { let o_ = l_k_8; let n_ = (o_).wrapping_add(1); l_k_8 = n_; };
                                }
                            }
                        }
                        { let o_ = l_j_7; let n_ = (o_).wrapping_add(1); l_j_7 = n_; };
                    }
                }
            }
            { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgModel_FadeDebris", &PROBE_N, &[], true); }
    r_
}

pub fn StgModel_SetTexture(c: &mut Ctx, mut p_index_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgModel_SetTexture", &PROBE_N, &[(p_index_1) as u32], false); }
    let r_ = 'f: {
    F::Dma_AddTexFlush(c);
    { let a0_ = { let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(88); c.m.r32(a_) }; F::TexFile_UploadOne(c, a0_, p_index_1, 10752i32, 12992i32) };
    { let a0_ = (({ let a_ = (({ let a_ = ({ let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(88); c.m.r32(a_) }).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((p_index_1) as u32)).wrapping_mul(64))).wrapping_add(48); c.m.r64(a_) }) | (((1785624063388160i64) as u64))); F::Dma_AddTex0(c, a0_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgModel_SetTexture", &PROBE_N, &[], true); }
    r_
}

pub fn StgModel_DrawDebris(c: &mut Ctx, mut p_drawn_1: i32, mut p_vis_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgModel_DrawDebris", &PROBE_N, &[(p_drawn_1) as u32, (p_vis_2) as u32], false); }
    let mut l_mdl_3: u32 = 0;
    let mut l_mats_4: u32 = 0;
    let mut l_mat_5: u32 = 0;
    let mut l_mesh_6: u32 = 0;
    let mut l_i_7: i32 = 0;
    let mut l_j_8: i32 = 0;
    let mut l_chain_9: u32 = 0;
    let r_ = 'f: {
    l_mdl_3 = { let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(28); c.m.r32(a_) };
    if (((((((c.m.r16(((l_mdl_3).wrapping_add((((6i32) as u32)).wrapping_mul(8))).wrapping_add(2))) as i32)) != (0i32)) as i32)) != 0) {
        { let _ = F::Vu1Pkt_LoadProg7(c); };
        F::StgModel_SetDrawEnv(c);
        F::StgModel_SetTestDebris(c);
        l_mats_4 = c.m.r32(((l_mdl_3).wrapping_add((((6i32) as u32)).wrapping_mul(8))).wrapping_add(4));
        p_drawn_1 = 0i32;
        {
            l_i_7 = 0i32;
            'l1: loop {
                if !(((((l_i_7) < (((c.m.r16(((l_mdl_3).wrapping_add((((6i32) as u32)).wrapping_mul(8))).wrapping_add(2))) as i32))) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_chain_9 = ((0i32) as u32);
                    l_mesh_6 = c.m.r32(((l_mats_4).wrapping_add((((l_i_7) as u32)).wrapping_mul(16))).wrapping_add(4));
                    {
                        l_j_8 = 0i32;
                        'l3: loop {
                            if !(((((l_j_8) < (((c.m.r16(((l_mats_4).wrapping_add((((l_i_7) as u32)).wrapping_mul(16))).wrapping_add(2))) as i32))) as i32)) != 0) { break 'l3; }
                            'c4: {
                                if ((((((({ let a_ = (p_vis_2).wrapping_add(((c.m.r16((l_mesh_6).wrapping_add((((l_j_8) as u32)).wrapping_mul(16)))) as u32)); c.m.r8(a_) }) as i32)) != (0i32)) as i32)) != 0) {
                                    if (((((l_chain_9) == (((0i32) as u32))) as i32)) != 0) {
                                        { let a0_ = ((c.m.r16((l_mats_4).wrapping_add((((l_i_7) as u32)).wrapping_mul(16)))) as i32); F::StgModel_SetTexture(c, a0_) };
                                    }
                                    l_chain_9 = c.m.r32(((l_mesh_6).wrapping_add((((l_j_8) as u32)).wrapping_mul(16))).wrapping_add(8));
                                    { let a0_ = c.m.r32(((l_mesh_6).wrapping_add((((l_j_8) as u32)).wrapping_mul(16))).wrapping_add(12)); F::Vu0Cur_LoadMtx(c, a0_) };
                                    { let _ = { let a1_ = rt::i2f((c.m.r8(((l_mesh_6).wrapping_add((((l_j_8) as u32)).wrapping_mul(16))).wrapping_add(4))) as i32); F::Vu1Pkt_CallProg7(c, l_chain_9, a1_) }; };
                                }
                            }
                            { let o_ = l_j_8; let n_ = (o_).wrapping_add(1); l_j_8 = n_; };
                        }
                    }
                }
                { let o_ = l_i_7; let n_ = (o_).wrapping_add(1); l_i_7 = n_; };
            }
        }
        F::StgModel_SetTestDefault(c);
    }
    break 'f p_drawn_1;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "StgModel_DrawDebris", &PROBE_N, &[], true); }
    r_
}

pub fn StgModel_Draw(c: &mut Ctx, mut p_view_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgModel_Draw", &PROBE_N, &[(p_view_1) as u32], false); }
    let mut l_vis_2: u32 = 0;
    let mut l_prog_3: i32 = 0;
    let mut l_blend_4: i32 = 0;
    let mut l_mtx_5: u32 = 0;
    let mut l_mdl_6: u32 = 0;
    let mut l_mats_7: u32 = 0;
    let mut l_mesh_8: u32 = 0;
    let mut l_anim_9: u32 = 0;
    let mut l_frames_10: u32 = 0;
    let mut l_i_11: i32 = 0;
    let mut l_j_12: i32 = 0;
    let mut l_k_13: i32 = 0;
    let mut l_chain_14: u32 = 0;
    let r_ = 'f: {
    l_prog_3 = (1i32).wrapping_neg();
    l_blend_4 = 0i32;
    if (((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((8192i32) as u64)))) != 0) {
        break 'f;
    }
    if (((({ let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(8); (c.m.r32(a_) as i32) }) & (1i32))) != 0) {
        break 'f;
    }
    l_mdl_6 = { let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(28); c.m.r32(a_) };
    l_vis_2 = { let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(44); c.m.r32(a_) };
    F::StgModel_SetDrawEnv(c);
    if (((((((c.m.r16(((l_mdl_6).wrapping_add((((0i32) as u32)).wrapping_mul(8))).wrapping_add(2))) as i32)) != (0i32)) as i32)) != 0) {
        l_prog_3 = 0i32;
        F::Vu1Pkt_LoadProg4(c, (p_view_1).wrapping_add(256));
        l_mats_7 = c.m.r32(((l_mdl_6).wrapping_add((((0i32) as u32)).wrapping_mul(8))).wrapping_add(4));
        F::Dma_AddZbuf(c, 224i32, 1i32);
        {
            l_i_11 = 0i32;
            'l1: loop {
                if !(((((l_i_11) < (((c.m.r16(((l_mdl_6).wrapping_add((((0i32) as u32)).wrapping_mul(8))).wrapping_add(2))) as i32))) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_chain_14 = ((0i32) as u32);
                    l_mesh_8 = c.m.r32(((l_mats_7).wrapping_add((((l_i_11) as u32)).wrapping_mul(16))).wrapping_add(4));
                    {
                        l_j_12 = 0i32;
                        'l3: loop {
                            if !(((((l_j_12) < (((c.m.r16(((l_mats_7).wrapping_add((((l_i_11) as u32)).wrapping_mul(16))).wrapping_add(2))) as i32))) as i32)) != 0) { break 'l3; }
                            'c4: {
                                if ((((((({ let a_ = (l_vis_2).wrapping_add(((c.m.r16((l_mesh_8).wrapping_add((((l_j_12) as u32)).wrapping_mul(16)))) as u32)); c.m.r8(a_) }) as i32)) != (0i32)) as i32)) != 0) {
                                    if (((((l_chain_14) == (((0i32) as u32))) as i32)) != 0) {
                                        { let a0_ = ((c.m.r16((l_mats_7).wrapping_add((((l_i_11) as u32)).wrapping_mul(16)))) as i32); F::StgModel_SetTexture(c, a0_) };
                                    }
                                    l_chain_14 = c.m.r32(((l_mesh_8).wrapping_add((((l_j_12) as u32)).wrapping_mul(16))).wrapping_add(8));
                                    F::Vu1Pkt_CallProg4(c, l_chain_14);
                                }
                            }
                            { let o_ = l_j_12; let n_ = (o_).wrapping_add(1); l_j_12 = n_; };
                        }
                    }
                }
                { let o_ = l_i_11; let n_ = (o_).wrapping_add(1); l_i_11 = n_; };
            }
        }
        F::Dma_AddZbuf(c, 224i32, 0i32);
    }
    if ((((!(((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((36028797018963968i64) as u64)))) != 0)) as i32)) != 0) {
        F::Dma_AddZbuf(c, 224i32, 1i32);
        F::EftStageScroll_Draw(c);
        F::Dma_AddZbuf(c, 224i32, 0i32);
    }
    if (((((((c.m.r16(((l_mdl_6).wrapping_add((((1i32) as u32)).wrapping_mul(8))).wrapping_add(2))) as i32)) != (0i32)) as i32)) != 0) {
        if (((((l_prog_3) != (0i32)) as i32)) != 0) {
            l_prog_3 = 0i32;
            F::Vu1Pkt_LoadProg4(c, (p_view_1).wrapping_add(256));
        }
        l_mats_7 = c.m.r32(((l_mdl_6).wrapping_add((((1i32) as u32)).wrapping_mul(8))).wrapping_add(4));
        F::Dma_AddZbuf(c, 224i32, 0i32);
        {
            l_i_11 = 0i32;
            'l5: loop {
                if !(((((l_i_11) < (((c.m.r16(((l_mdl_6).wrapping_add((((1i32) as u32)).wrapping_mul(8))).wrapping_add(2))) as i32))) as i32)) != 0) { break 'l5; }
                'c6: {
                    l_chain_14 = ((0i32) as u32);
                    l_mesh_8 = c.m.r32(((l_mats_7).wrapping_add((((l_i_11) as u32)).wrapping_mul(16))).wrapping_add(4));
                    {
                        l_j_12 = 0i32;
                        'l7: loop {
                            if !(((((l_j_12) < (((c.m.r16(((l_mats_7).wrapping_add((((l_i_11) as u32)).wrapping_mul(16))).wrapping_add(2))) as i32))) as i32)) != 0) { break 'l7; }
                            'c8: {
                                if ((((((({ let a_ = (l_vis_2).wrapping_add(((c.m.r16((l_mesh_8).wrapping_add((((l_j_12) as u32)).wrapping_mul(16)))) as u32)); c.m.r8(a_) }) as i32)) != (0i32)) as i32)) != 0) {
                                    if (((((l_chain_14) == (((0i32) as u32))) as i32)) != 0) {
                                        { let a0_ = ((c.m.r16((l_mats_7).wrapping_add((((l_i_11) as u32)).wrapping_mul(16)))) as i32); F::StgModel_SetTexture(c, a0_) };
                                    }
                                    l_chain_14 = c.m.r32(((l_mesh_8).wrapping_add((((l_j_12) as u32)).wrapping_mul(16))).wrapping_add(8));
                                    F::Vu1Pkt_CallProg4(c, l_chain_14);
                                }
                            }
                            { let o_ = l_j_12; let n_ = (o_).wrapping_add(1); l_j_12 = n_; };
                        }
                    }
                }
                { let o_ = l_i_11; let n_ = (o_).wrapping_add(1); l_i_11 = n_; };
            }
        }
    }
    F::StgModel_SetTestOpaque(c);
    if (((((((c.m.r16(((l_mdl_6).wrapping_add((((2i32) as u32)).wrapping_mul(8))).wrapping_add(2))) as i32)) != (0i32)) as i32)) != 0) {
        l_mtx_5 = (p_view_1).wrapping_add(256);
        F::Vu1Pkt_LoadProg4(c, l_mtx_5);
        l_mats_7 = c.m.r32(((l_mdl_6).wrapping_add((((2i32) as u32)).wrapping_mul(8))).wrapping_add(4));
        {
            l_i_11 = 0i32;
            'l9: loop {
                if !(((((l_i_11) < (((c.m.r16(((l_mdl_6).wrapping_add((((2i32) as u32)).wrapping_mul(8))).wrapping_add(2))) as i32))) as i32)) != 0) { break 'l9; }
                'c10: {
                    l_chain_14 = ((0i32) as u32);
                    l_mesh_8 = c.m.r32(((l_mats_7).wrapping_add((((l_i_11) as u32)).wrapping_mul(16))).wrapping_add(4));
                    {
                        l_j_12 = 0i32;
                        'l11: loop {
                            if !(((((l_j_12) < (((c.m.r16(((l_mats_7).wrapping_add((((l_i_11) as u32)).wrapping_mul(16))).wrapping_add(2))) as i32))) as i32)) != 0) { break 'l11; }
                            'c12: {
                                if ((((((({ let a_ = (l_vis_2).wrapping_add(((c.m.r16((l_mesh_8).wrapping_add((((l_j_12) as u32)).wrapping_mul(16)))) as u32)); c.m.r8(a_) }) as i32)) == (0i32)) as i32)) != 0) {
                                    break 'c12;
                                }
                                if (((((l_chain_14) == (((0i32) as u32))) as i32)) != 0) {
                                    l_anim_9 = c.m.r32(((l_mats_7).wrapping_add((((l_i_11) as u32)).wrapping_mul(16))).wrapping_add(8));
                                    if (((((l_anim_9) != (((0i32) as u32))) as i32)) != 0) {
                                        if ((((((((((c.m.r16(l_anim_9)) as i32)) & (4i32))) != 0) && ((((!(((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0)) as i32)) != 0)) as i32)) != 0) {
                                            l_frames_10 = c.m.r32((l_anim_9).wrapping_add(4));
                                            {
                                                l_k_13 = 0i32;
                                                'l13: loop {
                                                    if !(((((l_k_13) < (((c.m.r16((l_anim_9).wrapping_add(2))) as i32))) as i32)) != 0) { break 'l13; }
                                                    'c14: {
                                                        if ((((((((c.m.r32((l_frames_10).wrapping_add((((l_k_13) as u32)).wrapping_mul(4))) as i32)) as u32)) == (((((c.m.r16(((l_mats_7).wrapping_add((((l_i_11) as u32)).wrapping_mul(16))).wrapping_add(12))) as u32)) & (4294934527u32)))) as i32)) != 0) {
                                                            if ((((((((c.m.r16(l_anim_9)) as i32)) & (1i32))) as u16)) != 0) {
                                                                if (((((l_k_13) == ((((c.m.r16((l_anim_9).wrapping_add(2))) as i32)).wrapping_sub(1i32))) as i32)) != 0) {
                                                                    { let v_ = (((c.m.r32((l_frames_10).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) as u16); let a_ = ((l_mats_7).wrapping_add((((l_i_11) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w16(a_, v_); };
                                                                } else {
                                                                    { let v_ = (((c.m.r32((l_frames_10).wrapping_add(((((l_k_13).wrapping_add(1i32)) as u32)).wrapping_mul(4))) as i32)) as u16); let a_ = ((l_mats_7).wrapping_add((((l_i_11) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w16(a_, v_); };
                                                                }
                                                            } else {
                                                                if ((((((c.m.r16(((l_mats_7).wrapping_add((((l_i_11) as u32)).wrapping_mul(16))).wrapping_add(12))) as i32)) & (32768i32))) != 0) {
                                                                    if (((((l_k_13) == (0i32)) as i32)) != 0) {
                                                                        { let v_ = (((c.m.r32((l_frames_10).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) as u16); let a_ = ((l_mats_7).wrapping_add((((l_i_11) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w16(a_, v_); };
                                                                    } else {
                                                                        { let v_ = (((((((((c.m.r32((l_frames_10).wrapping_add(((((l_k_13).wrapping_sub(1i32)) as u32)).wrapping_mul(4))) as i32)) as u16)) as i32)) | (32768i32))) as u16); let a_ = ((l_mats_7).wrapping_add((((l_i_11) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w16(a_, v_); };
                                                                    }
                                                                } else {
                                                                    if (((((l_k_13) == ((((c.m.r16((l_anim_9).wrapping_add(2))) as i32)).wrapping_sub(1i32))) as i32)) != 0) {
                                                                        { let v_ = (((((((({ let a_ = (l_frames_10).wrapping_add(((((((c.m.r16((l_anim_9).wrapping_add(2))) as i32)).wrapping_sub(2i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u16)) as i32)) | (32768i32))) as u16); let a_ = ((l_mats_7).wrapping_add((((l_i_11) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w16(a_, v_); };
                                                                    } else {
                                                                        { let v_ = (((c.m.r32((l_frames_10).wrapping_add(((((l_k_13).wrapping_add(1i32)) as u32)).wrapping_mul(4))) as i32)) as u16); let a_ = ((l_mats_7).wrapping_add((((l_i_11) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w16(a_, v_); };
                                                                    }
                                                                }
                                                            }
                                                            break 'l13;
                                                        }
                                                    }
                                                    { let o_ = l_k_13; let n_ = (o_).wrapping_add(1); l_k_13 = n_; };
                                                }
                                            }
                                        }
                                        { let a0_ = ((c.m.r8(((l_mats_7).wrapping_add((((l_i_11) as u32)).wrapping_mul(16))).wrapping_add(12))) as i32); F::StgModel_SetTexture(c, a0_) };
                                    } else {
                                        { let a0_ = ((c.m.r16((l_mats_7).wrapping_add((((l_i_11) as u32)).wrapping_mul(16)))) as i32); F::StgModel_SetTexture(c, a0_) };
                                    }
                                }
                                l_chain_14 = c.m.r32(((l_mesh_8).wrapping_add((((l_j_12) as u32)).wrapping_mul(16))).wrapping_add(8));
                                if (((((((c.m.r8(((l_mesh_8).wrapping_add((((l_j_12) as u32)).wrapping_mul(16))).wrapping_add(3))) as i32)) != (l_prog_3)) as i32)) != 0) {
                                    's15: {
                                        let sv15: i32 = ((c.m.r8(((l_mesh_8).wrapping_add((((l_j_12) as u32)).wrapping_mul(16))).wrapping_add(3))) as i32);
                                        match sv15 {
                                            0i32 => {
                                                F::Vu1Pkt_LoadProg4(c, l_mtx_5);
                                                break 's15;
                                            }
                                            1i32 => {
                                                break 's15;
                                            }
                                            _ => {
                                            }
                                        }
                                    }
                                    l_prog_3 = ((c.m.r8(((l_mesh_8).wrapping_add((((l_j_12) as u32)).wrapping_mul(16))).wrapping_add(3))) as i32);
                                }
                                if (((((((c.m.r8(((l_mesh_8).wrapping_add((((l_j_12) as u32)).wrapping_mul(16))).wrapping_add(3))) as i32)) == (0i32)) as i32)) != 0) {
                                    F::Vu1Pkt_CallProg4(c, l_chain_14);
                                }
                            }
                            { let o_ = l_j_12; let n_ = (o_).wrapping_add(1); l_j_12 = n_; };
                        }
                    }
                }
                { let o_ = l_i_11; let n_ = (o_).wrapping_add(1); l_i_11 = n_; };
            }
        }
    }
    F::StgModel_DrawAnims(c);
    if (((((F::BtlStage_GetId(c)) == (8i32)) as i32)) != 0) {
        l_blend_4 = F::StgModel_DrawDebris(c, 0i32, l_vis_2);
    }
    if (((((((((F::BtlStage_GetId(c)) != (5i32)) as i32)) != 0) && (((((((c.m.r16(((l_mdl_6).wrapping_add((((3i32) as u32)).wrapping_mul(8))).wrapping_add(2))) as i32)) != (0i32)) as i32)) != 0)) as i32)) != 0) {
        l_blend_4 = 1i32;
        F::Vu1Pkt_LoadProg4(c, (p_view_1).wrapping_add(256));
        F::Dma_AddZbuf(c, 224i32, 1i32);
        F::Dma_AddFillRect(c, 1792i32, 1824i32, 512i32, 448i32, ((0i32) as u32));
        l_mats_7 = c.m.r32(((l_mdl_6).wrapping_add((((3i32) as u32)).wrapping_mul(8))).wrapping_add(4));
        F::StgModel_SetBlendEnv(c);
        F::Dma_AddZbuf(c, 224i32, 0i32);
        {
            l_i_11 = 0i32;
            'l16: loop {
                if !(((((l_i_11) < (((c.m.r16(((l_mdl_6).wrapping_add((((3i32) as u32)).wrapping_mul(8))).wrapping_add(2))) as i32))) as i32)) != 0) { break 'l16; }
                'c17: {
                    l_chain_14 = ((0i32) as u32);
                    l_mesh_8 = c.m.r32(((l_mats_7).wrapping_add((((l_i_11) as u32)).wrapping_mul(16))).wrapping_add(4));
                    {
                        l_j_12 = 0i32;
                        'l18: loop {
                            if !(((((l_j_12) < (((c.m.r16(((l_mats_7).wrapping_add((((l_i_11) as u32)).wrapping_mul(16))).wrapping_add(2))) as i32))) as i32)) != 0) { break 'l18; }
                            'c19: {
                                if ((((((({ let a_ = (l_vis_2).wrapping_add(((c.m.r16((l_mesh_8).wrapping_add((((l_j_12) as u32)).wrapping_mul(16)))) as u32)); c.m.r8(a_) }) as i32)) != (0i32)) as i32)) != 0) {
                                    if (((((l_chain_14) == (((0i32) as u32))) as i32)) != 0) {
                                        { let a0_ = ((c.m.r16((l_mats_7).wrapping_add((((l_i_11) as u32)).wrapping_mul(16)))) as i32); F::StgModel_SetTexture(c, a0_) };
                                    }
                                    l_chain_14 = c.m.r32(((l_mesh_8).wrapping_add((((l_j_12) as u32)).wrapping_mul(16))).wrapping_add(8));
                                    F::Vu1Pkt_CallProg4(c, l_chain_14);
                                }
                            }
                            { let o_ = l_j_12; let n_ = (o_).wrapping_add(1); l_j_12 = n_; };
                        }
                    }
                }
                { let o_ = l_i_11; let n_ = (o_).wrapping_add(1); l_i_11 = n_; };
            }
        }
        F::Dma_AddZbuf(c, 224i32, 0i32);
    }
    if (((((((c.m.r16(((l_mdl_6).wrapping_add((((4i32) as u32)).wrapping_mul(8))).wrapping_add(2))) as i32)) != (0i32)) as i32)) != 0) {
        l_blend_4 = 1i32;
        F::Vu1Pkt_LoadProg4(c, (p_view_1).wrapping_add(256));
        F::Dma_AddZbuf(c, 224i32, 1i32);
        F::Dma_AddFillRect(c, 1792i32, 1824i32, 512i32, 448i32, ((0i32) as u32));
        l_mats_7 = c.m.r32(((l_mdl_6).wrapping_add((((4i32) as u32)).wrapping_mul(8))).wrapping_add(4));
        F::Dma_AddZbuf(c, 224i32, 1i32);
        F::StgModel_SetBlendEnv(c);
        {
            l_i_11 = 0i32;
            'l20: loop {
                if !(((((l_i_11) < (((c.m.r16(((l_mdl_6).wrapping_add((((4i32) as u32)).wrapping_mul(8))).wrapping_add(2))) as i32))) as i32)) != 0) { break 'l20; }
                'c21: {
                    l_chain_14 = ((0i32) as u32);
                    l_mesh_8 = c.m.r32(((l_mats_7).wrapping_add((((l_i_11) as u32)).wrapping_mul(16))).wrapping_add(4));
                    {
                        l_j_12 = 0i32;
                        'l22: loop {
                            if !(((((l_j_12) < (((c.m.r16(((l_mats_7).wrapping_add((((l_i_11) as u32)).wrapping_mul(16))).wrapping_add(2))) as i32))) as i32)) != 0) { break 'l22; }
                            'c23: {
                                if ((((((({ let a_ = (l_vis_2).wrapping_add(((c.m.r16((l_mesh_8).wrapping_add((((l_j_12) as u32)).wrapping_mul(16)))) as u32)); c.m.r8(a_) }) as i32)) != (0i32)) as i32)) != 0) {
                                    if (((((l_chain_14) == (((0i32) as u32))) as i32)) != 0) {
                                        { let a0_ = ((c.m.r16((l_mats_7).wrapping_add((((l_i_11) as u32)).wrapping_mul(16)))) as i32); F::StgModel_SetTexture(c, a0_) };
                                    }
                                    l_chain_14 = c.m.r32(((l_mesh_8).wrapping_add((((l_j_12) as u32)).wrapping_mul(16))).wrapping_add(8));
                                    F::Vu1Pkt_CallProg4(c, l_chain_14);
                                }
                            }
                            { let o_ = l_j_12; let n_ = (o_).wrapping_add(1); l_j_12 = n_; };
                        }
                    }
                }
                { let o_ = l_i_11; let n_ = (o_).wrapping_add(1); l_i_11 = n_; };
            }
        }
        F::Dma_AddZbuf(c, 224i32, 0i32);
    }
    if (((((((c.m.r16(((l_mdl_6).wrapping_add((((5i32) as u32)).wrapping_mul(8))).wrapping_add(2))) as i32)) != (0i32)) as i32)) != 0) {
        l_blend_4 = 1i32;
        F::Vu1Pkt_LoadProg4(c, (p_view_1).wrapping_add(256));
        F::Dma_AddZbuf(c, 224i32, 1i32);
        F::Dma_AddFillRect(c, 1792i32, 1824i32, 512i32, 448i32, ((0i32) as u32));
        l_mats_7 = c.m.r32(((l_mdl_6).wrapping_add((((5i32) as u32)).wrapping_mul(8))).wrapping_add(4));
        F::StgModel_SetBlendEnv(c);
        {
            l_i_11 = 0i32;
            'l24: loop {
                if !(((((l_i_11) < (((c.m.r16(((l_mdl_6).wrapping_add((((5i32) as u32)).wrapping_mul(8))).wrapping_add(2))) as i32))) as i32)) != 0) { break 'l24; }
                'c25: {
                    l_chain_14 = ((0i32) as u32);
                    l_mesh_8 = c.m.r32(((l_mats_7).wrapping_add((((l_i_11) as u32)).wrapping_mul(16))).wrapping_add(4));
                    {
                        l_j_12 = 0i32;
                        'l26: loop {
                            if !(((((l_j_12) < (((c.m.r16(((l_mats_7).wrapping_add((((l_i_11) as u32)).wrapping_mul(16))).wrapping_add(2))) as i32))) as i32)) != 0) { break 'l26; }
                            'c27: {
                                if ((((((({ let a_ = (l_vis_2).wrapping_add(((c.m.r16((l_mesh_8).wrapping_add((((l_j_12) as u32)).wrapping_mul(16)))) as u32)); c.m.r8(a_) }) as i32)) != (0i32)) as i32)) != 0) {
                                    if (((((l_chain_14) == (((0i32) as u32))) as i32)) != 0) {
                                        { let a0_ = ((c.m.r16((l_mats_7).wrapping_add((((l_i_11) as u32)).wrapping_mul(16)))) as i32); F::StgModel_SetTexture(c, a0_) };
                                    }
                                    l_chain_14 = c.m.r32(((l_mesh_8).wrapping_add((((l_j_12) as u32)).wrapping_mul(16))).wrapping_add(8));
                                    F::Vu1Pkt_CallProg4(c, l_chain_14);
                                }
                            }
                            { let o_ = l_j_12; let n_ = (o_).wrapping_add(1); l_j_12 = n_; };
                        }
                    }
                }
                { let o_ = l_i_11; let n_ = (o_).wrapping_add(1); l_i_11 = n_; };
            }
        }
        F::Dma_AddZbuf(c, 224i32, 0i32);
    }
    if (((((F::BtlStage_GetId(c)) != (8i32)) as i32)) != 0) {
        l_blend_4 = F::StgModel_DrawDebris(c, l_blend_4, l_vis_2);
    }
    F::Dma_AddZbuf(c, 224i32, 0i32);
    if (((((l_blend_4) != (0i32)) as i32)) != 0) {
        F::StgModel_SetDrawEnv(c);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgModel_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn StgModel_SetDrawEnv(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgModel_SetDrawEnv", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(160) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 160);
    let r_ = 'f: {
        rt::memset(c, fp, 0, 160);
        { let v_ = ((268435465i32) as u32); c.m.w32(fp.wrapping_add(0), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(4), v_); }
        { let v_ = ((268435456i32) as u32); c.m.w32(fp.wrapping_add(8), v_); }
        { let v_ = ((1342177289i32) as u32); c.m.w32(fp.wrapping_add(12), v_); }
        { let v_ = ((32776i32) as u32); c.m.w32(fp.wrapping_add(16), v_); }
        { let v_ = ((268435456i32) as u32); c.m.w32(fp.wrapping_add(20), v_); }
        { let v_ = ((14i32) as u32); c.m.w32(fp.wrapping_add(24), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(28), v_); }
        { let v_ = (((if ((((!(((((c.m.r32((sym::gGfx).wrapping_add(564)) as i32)) & (1i32))) != 0)) as i32)) != 0) { 524400i32 } else { 524288i32 })) as u32); c.m.w32(fp.wrapping_add(32), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(36), v_); }
        { let v_ = ((76i32) as u32); c.m.w32(fp.wrapping_add(40), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(44), v_); }
        { let v_ = ((822083808i32) as u32); c.m.w32(fp.wrapping_add(48), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(52), v_); }
        { let v_ = ((78i32) as u32); c.m.w32(fp.wrapping_add(56), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(60), v_); }
        { let v_ = ((68i32) as u32); c.m.w32(fp.wrapping_add(64), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(68), v_); }
        { let v_ = ((66i32) as u32); c.m.w32(fp.wrapping_add(72), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(76), v_); }
        { let v_ = ((327707i32) as u32); c.m.w32(fp.wrapping_add(80), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(84), v_); }
        { let v_ = ((71i32) as u32); c.m.w32(fp.wrapping_add(88), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(92), v_); }
        { let v_ = ((96i32) as u32); c.m.w32(fp.wrapping_add(96), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(100), v_); }
        { let v_ = ((20i32) as u32); c.m.w32(fp.wrapping_add(104), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(108), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(112), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(116), v_); }
        { let v_ = ((74i32) as u32); c.m.w32(fp.wrapping_add(120), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(124), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(128), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(132), v_); }
        { let v_ = ((8i32) as u32); c.m.w32(fp.wrapping_add(136), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(140), v_); }
        { let v_ = ((1i32) as u32); c.m.w32(fp.wrapping_add(144), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(148), v_); }
        { let v_ = ((70i32) as u32); c.m.w32(fp.wrapping_add(152), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(156), v_); }
        F::Dma_AddData(c, fp, 160i32);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "StgModel_SetDrawEnv", &PROBE_N, &[], true); }
    r_
}

pub fn StgModel_SetBlendEnv(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgModel_SetBlendEnv", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(80) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 80);
    let r_ = 'f: {
        rt::memset(c, fp, 0, 80);
        { let v_ = ((268435460i32) as u32); c.m.w32(fp.wrapping_add(0), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(4), v_); }
        { let v_ = ((268435456i32) as u32); c.m.w32(fp.wrapping_add(8), v_); }
        { let v_ = ((1342177284i32) as u32); c.m.w32(fp.wrapping_add(12), v_); }
        { let v_ = ((32771i32) as u32); c.m.w32(fp.wrapping_add(16), v_); }
        { let v_ = ((268435456i32) as u32); c.m.w32(fp.wrapping_add(20), v_); }
        { let v_ = ((14i32) as u32); c.m.w32(fp.wrapping_add(24), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(28), v_); }
        { let v_ = ((822083808i32) as u32); c.m.w32(fp.wrapping_add(32), v_); }
        { let v_ = ((1i32) as u32); c.m.w32(fp.wrapping_add(36), v_); }
        { let v_ = ((78i32) as u32); c.m.w32(fp.wrapping_add(40), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(44), v_); }
        { let v_ = ((344064i32) as u32); c.m.w32(fp.wrapping_add(48), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(52), v_); }
        { let v_ = ((71i32) as u32); c.m.w32(fp.wrapping_add(56), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(60), v_); }
        { let v_ = ((1i32) as u32); c.m.w32(fp.wrapping_add(64), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(68), v_); }
        { let v_ = ((74i32) as u32); c.m.w32(fp.wrapping_add(72), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(76), v_); }
        F::Dma_AddData(c, fp, 80i32);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "StgModel_SetBlendEnv", &PROBE_N, &[], true); }
    r_
}

pub fn StgModel_SetTestOpaque(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgModel_SetTestOpaque", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let r_ = 'f: {
        rt::memset(c, fp, 0, 48);
        { let v_ = ((268435458i32) as u32); c.m.w32(fp.wrapping_add(0), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(4), v_); }
        { let v_ = ((268435456i32) as u32); c.m.w32(fp.wrapping_add(8), v_); }
        { let v_ = ((1342177282i32) as u32); c.m.w32(fp.wrapping_add(12), v_); }
        { let v_ = ((32769i32) as u32); c.m.w32(fp.wrapping_add(16), v_); }
        { let v_ = ((268435456i32) as u32); c.m.w32(fp.wrapping_add(20), v_); }
        { let v_ = ((14i32) as u32); c.m.w32(fp.wrapping_add(24), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(28), v_); }
        { let v_ = ((329723i32) as u32); c.m.w32(fp.wrapping_add(32), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(36), v_); }
        { let v_ = ((71i32) as u32); c.m.w32(fp.wrapping_add(40), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(44), v_); }
        F::Dma_AddData(c, fp, 48i32);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "StgModel_SetTestOpaque", &PROBE_N, &[], true); }
    r_
}

pub fn StgModel_SetTestDebris(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgModel_SetTestDebris", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let r_ = 'f: {
        rt::memset(c, fp, 0, 48);
        { let v_ = ((268435458i32) as u32); c.m.w32(fp.wrapping_add(0), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(4), v_); }
        { let v_ = ((268435456i32) as u32); c.m.w32(fp.wrapping_add(8), v_); }
        { let v_ = ((1342177282i32) as u32); c.m.w32(fp.wrapping_add(12), v_); }
        { let v_ = ((32769i32) as u32); c.m.w32(fp.wrapping_add(16), v_); }
        { let v_ = ((268435456i32) as u32); c.m.w32(fp.wrapping_add(20), v_); }
        { let v_ = ((14i32) as u32); c.m.w32(fp.wrapping_add(24), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(28), v_); }
        { let v_ = ((333833i32) as u32); c.m.w32(fp.wrapping_add(32), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(36), v_); }
        { let v_ = ((71i32) as u32); c.m.w32(fp.wrapping_add(40), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(44), v_); }
        F::Dma_AddData(c, fp, 48i32);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "StgModel_SetTestDebris", &PROBE_N, &[], true); }
    r_
}

pub fn StgModel_SetTestDefault(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgModel_SetTestDefault", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let r_ = 'f: {
        rt::memset(c, fp, 0, 48);
        { let v_ = ((268435458i32) as u32); c.m.w32(fp.wrapping_add(0), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(4), v_); }
        { let v_ = ((268435456i32) as u32); c.m.w32(fp.wrapping_add(8), v_); }
        { let v_ = ((1342177282i32) as u32); c.m.w32(fp.wrapping_add(12), v_); }
        { let v_ = ((32769i32) as u32); c.m.w32(fp.wrapping_add(16), v_); }
        { let v_ = ((268435456i32) as u32); c.m.w32(fp.wrapping_add(20), v_); }
        { let v_ = ((14i32) as u32); c.m.w32(fp.wrapping_add(24), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(28), v_); }
        { let v_ = ((327707i32) as u32); c.m.w32(fp.wrapping_add(32), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(36), v_); }
        { let v_ = ((71i32) as u32); c.m.w32(fp.wrapping_add(40), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(44), v_); }
        F::Dma_AddData(c, fp, 48i32);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "StgModel_SetTestDefault", &PROBE_N, &[], true); }
    r_
}

pub fn StgModel_SetAnimDrawEnv(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgModel_SetAnimDrawEnv", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(160) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 160);
    let r_ = 'f: {
        rt::memset(c, fp, 0, 160);
        { let v_ = ((268435465i32) as u32); c.m.w32(fp.wrapping_add(0), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(4), v_); }
        { let v_ = ((268435456i32) as u32); c.m.w32(fp.wrapping_add(8), v_); }
        { let v_ = ((1342177289i32) as u32); c.m.w32(fp.wrapping_add(12), v_); }
        { let v_ = ((32776i32) as u32); c.m.w32(fp.wrapping_add(16), v_); }
        { let v_ = ((268435456i32) as u32); c.m.w32(fp.wrapping_add(20), v_); }
        { let v_ = ((14i32) as u32); c.m.w32(fp.wrapping_add(24), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(28), v_); }
        { let v_ = (((if ((((!(((((c.m.r32((sym::gGfx).wrapping_add(564)) as i32)) & (1i32))) != 0)) as i32)) != 0) { 524400i32 } else { 524288i32 })) as u32); c.m.w32(fp.wrapping_add(32), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(36), v_); }
        { let v_ = ((76i32) as u32); c.m.w32(fp.wrapping_add(40), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(44), v_); }
        { let v_ = ((822083808i32) as u32); c.m.w32(fp.wrapping_add(48), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(52), v_); }
        { let v_ = ((78i32) as u32); c.m.w32(fp.wrapping_add(56), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(60), v_); }
        { let v_ = ((68i32) as u32); c.m.w32(fp.wrapping_add(64), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(68), v_); }
        { let v_ = ((66i32) as u32); c.m.w32(fp.wrapping_add(72), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(76), v_); }
        { let v_ = ((329723i32) as u32); c.m.w32(fp.wrapping_add(80), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(84), v_); }
        { let v_ = ((71i32) as u32); c.m.w32(fp.wrapping_add(88), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(92), v_); }
        { let v_ = ((96i32) as u32); c.m.w32(fp.wrapping_add(96), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(100), v_); }
        { let v_ = ((20i32) as u32); c.m.w32(fp.wrapping_add(104), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(108), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(112), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(116), v_); }
        { let v_ = ((74i32) as u32); c.m.w32(fp.wrapping_add(120), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(124), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(128), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(132), v_); }
        { let v_ = ((8i32) as u32); c.m.w32(fp.wrapping_add(136), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(140), v_); }
        { let v_ = ((1i32) as u32); c.m.w32(fp.wrapping_add(144), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(148), v_); }
        { let v_ = ((70i32) as u32); c.m.w32(fp.wrapping_add(152), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(156), v_); }
        F::Dma_AddData(c, fp, 160i32);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "StgModel_SetAnimDrawEnv", &PROBE_N, &[], true); }
    r_
}
