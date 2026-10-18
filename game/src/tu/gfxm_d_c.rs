#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn ObjSeam_Transform(c: &mut Ctx, mut p_obj_1: u32, mut p_view_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ObjSeam_Transform", &PROBE_N, &[(p_obj_1) as u32, (p_view_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(208) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 208);
    let mut l_sec_3: u32 = 0;
    let mut l_vtx_4: u32 = 0;
    let mut l_i_5: i32 = 0;
    let mut l_part_6: u32 = 0;
    let mut l_node_7: u32 = 0;
    let r_ = 'f: {
        if ((((({ let a_ = (c.m.r32(((p_obj_1).wrapping_add(24)).wrapping_add(40))).wrapping_add(112); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
            l_sec_3 = (c.m.r32(((p_obj_1).wrapping_add(24)).wrapping_add(40))).wrapping_add((({ let a_ = (c.m.r32(((p_obj_1).wrapping_add(24)).wrapping_add(40))).wrapping_add(112); (c.m.r32(a_) as i32) }) as u32));
            l_vtx_4 = (c.m.r32(((p_obj_1).wrapping_add(24)).wrapping_add(40))).wrapping_add((((c.m.r32((l_sec_3).wrapping_add(8)) as i32)) as u32));
            F::Mtx_MakeNormalLight(c, (fp).wrapping_add(128), (p_view_2).wrapping_add(16), sym::D_002EC2C0, sym::D_002EC2C0);
            { let v_ = F32(0x3f000000); let a_ = ((fp).wrapping_add(192)).wrapping_add(12); c.m.w32(a_, (v_).0); };
            {
                l_i_5 = 0i32;
                'l1: loop {
                    if !(((((l_i_5) < ((c.m.r32(l_sec_3) as i32))) as i32)) != 0) { break 'l1; }
                    'c2: {
                        l_part_6 = { let a1_ = (c.m.r32(((l_vtx_4).wrapping_add((((l_i_5) as u32)).wrapping_mul(112))).wrapping_add(28)) as i32); F::BtlObj_FindBound(c, p_obj_1, a1_) };
                        l_node_7 = { let a1_ = (c.m.r32(((l_vtx_4).wrapping_add((((l_i_5) as u32)).wrapping_mul(112))).wrapping_add(28)) as i32); F::BtlObj_GetNode(c, p_obj_1, a1_) };
                        F::ObjSeam_TransformVtx(c, (l_vtx_4).wrapping_add((((l_i_5) as u32)).wrapping_mul(112)), fp, (l_node_7).wrapping_add(16), (l_part_6).wrapping_add(16));
                    }
                    { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
                }
            }
            F::Vu0_InitAxisRegs(c);
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "ObjSeam_Transform", &PROBE_N, &[], true); }
    r_
}

pub fn ObjSeam_Draw(c: &mut Ctx, mut p_obj_1: u32, mut p_state_2: u32, mut p_view_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ObjSeam_Draw", &PROBE_N, &[(p_obj_1) as u32, (p_state_2) as u32, (p_view_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_mdl_4: u32 = 0;
    let mut l_model_5: u32 = 0;
    let mut l_sec_6: u32 = 0;
    let mut l_tri_7: u32 = 0;
    let mut l_vtx_8: u32 = 0;
    let mut l_rawBase_9: u64 = 0;
    let mut l_rawShade_10: u64 = 0;
    let mut l_tex0Base_11: u64 = 0;
    let mut l_tex0Shade_12: u64 = 0;
    let mut l_regBase_13: u64 = 0;
    let mut l_regShade_14: u64 = 0;
    let mut l_p_15: u32 = 0;
    let mut l_tag_16: u32 = 0;
    let mut l_pass_17: i32 = 0;
    let mut l_i_18: i32 = 0;
    let mut l_n_19: i32 = 0;
    let mut l_tbpBase_20: i32 = 0;
    let mut l_cbpBase_21: i32 = 0;
    let mut l_tbpShade_22: i32 = 0;
    let mut l_cbpShade_23: i32 = 0;
    let mut l_tbp_24: i32 = 0;
    let mut l_clutTbp_25: i32 = 0;
    let r_ = 'f: {
        rt::memset(c, fp, 0, 16);
        rt::memset(c, fp.wrapping_add(0), 0, 16);
        { let v_ = 128i32; c.m.w32(fp.wrapping_add(0).wrapping_add(0), (v_) as u32); }
        { let v_ = 128i32; c.m.w32(fp.wrapping_add(0).wrapping_add(4), (v_) as u32); }
        { let v_ = 128i32; c.m.w32(fp.wrapping_add(0).wrapping_add(8), (v_) as u32); }
        { let v_ = 128i32; c.m.w32(fp.wrapping_add(0).wrapping_add(12), (v_) as u32); }
        if ((((({ let a_ = (c.m.r32(((p_obj_1).wrapping_add(24)).wrapping_add(40))).wrapping_add(112); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
            l_sec_6 = (c.m.r32(((p_obj_1).wrapping_add(24)).wrapping_add(40))).wrapping_add((({ let a_ = (c.m.r32(((p_obj_1).wrapping_add(24)).wrapping_add(40))).wrapping_add(112); (c.m.r32(a_) as i32) }) as u32));
            l_mdl_4 = (p_obj_1).wrapping_add(24);
            l_model_5 = c.m.r32((l_mdl_4).wrapping_add(40));
            l_tri_7 = (l_model_5).wrapping_add((((c.m.r32((l_sec_6).wrapping_add(12)) as i32)) as u32));
            l_tbp_24 = (c.m.r32((l_model_5).wrapping_add(88)) as i32);
            l_clutTbp_25 = (c.m.r32((l_model_5).wrapping_add(96)) as i32);
            l_rawBase_9 = c.m.r64((l_tri_7).wrapping_add(16));
            l_rawShade_10 = c.m.r64((l_tri_7).wrapping_add(24));
            l_tbpBase_20 = (((((l_rawBase_9) & (((16383i32) as u64)))) as i32)).wrapping_add(l_tbp_24);
            l_cbpBase_21 = ((((((l_rawBase_9).wrapping_shr((37i32) as u32)) & (((16383i32) as u64)))) as i32)).wrapping_add((c.m.r32((l_model_5).wrapping_add(92)) as i32));
            l_tbpShade_22 = (((((l_rawShade_10) & (((16383i32) as u64)))) as i32)).wrapping_add(15680i32);
            l_cbpShade_23 = ((((((l_rawShade_10).wrapping_shr((37i32) as u32)) & (((16383i32) as u64)))) as i32)).wrapping_add(l_clutTbp_25);
            l_tex0Base_11 = ((((l_rawBase_9) & ((((!(16383i32))) as u64)))) | (((l_tbpBase_20) as u64)));
            l_tex0Shade_12 = ((((l_rawShade_10) & ((((!(16383i32))) as u64)))) | (((l_tbpShade_22) as u64)));
            l_tex0Base_11 = ((((l_tex0Base_11) & (18444492411334819839u64))) | ((((l_cbpBase_21) as u64)).wrapping_shl((37i32) as u32)));
            l_tex0Shade_12 = ((((l_tex0Shade_12) & (18444492411334819839u64))) | ((((l_cbpShade_23) as u64)).wrapping_shl((37i32) as u32)));
            l_vtx_8 = (l_model_5).wrapping_add((((c.m.r32((l_sec_6).wrapping_add(8)) as i32)) as u32));
            l_p_15 = F::Dma_BeginDirect(c);
            l_regBase_13 = ((l_tex0Base_11) | (((17179869184i64) as u64)));
            l_regShade_14 = ((l_tex0Shade_12) | (((17179869184i64) as u64)));
            {
                l_pass_17 = 0i32;
                'l1: loop {
                    if !(((((l_pass_17) < (2i32)) as i32)) != 0) { break 'l1; }
                    'c2: {
                        if (((((l_pass_17) == (0i32)) as i32)) != 0) {
                            { let v_ = rt::f2i(F32(c.m.r32((p_view_3).wrapping_add(32)))); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                            { let v_ = rt::f2i(F32(c.m.r32(((p_view_3).wrapping_add(32)).wrapping_add(4)))); let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                            { let v_ = rt::f2i(F32(c.m.r32(((p_view_3).wrapping_add(32)).wrapping_add(8)))); let a_ = (fp).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                            { let v_ = ((((((((2i32) as u64)) | ((((0i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (l_p_15).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                            { let v_ = ((14i32) as u64); let a_ = (l_p_15).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                            { let r_ = 2i32; let n_ = (l_p_15).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_15 = n_; };
                            { let v_ = ((0i32) as u64); let a_ = (l_p_15).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                            { let v_ = ((63i32) as u64); let a_ = (l_p_15).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                            { let r_ = 2i32; let n_ = (l_p_15).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_15 = n_; };
                            { let v_ = l_regBase_13; let a_ = (l_p_15).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                            { let v_ = ((6i32) as u64); let a_ = (l_p_15).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                            { let r_ = 2i32; let n_ = (l_p_15).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_15 = n_; };
                            l_tag_16 = l_p_15;
                            { let r_ = 2i32; let n_ = (l_p_15).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_15 = n_; };
                            {
                                { l_i_18 = 0i32; l_n_19 = 0i32; };
                                'l3: loop {
                                    if !(((((l_i_18) < ((c.m.r32((l_sec_6).wrapping_add(4)) as i32))) as i32)) != 0) { break 'l3; }
                                    'c4: {
                                        if (((({ let a_ = (((l_vtx_8).wrapping_add(((((c.m.r32(((l_tri_7).wrapping_add((((l_i_18) as u32)).wrapping_mul(32))).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(112))).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) & (32768i32))) != 0) {
                                            break 'c4;
                                        }
                                        if (((({ let a_ = (((l_vtx_8).wrapping_add(((((c.m.r32(((l_tri_7).wrapping_add((((l_i_18) as u32)).wrapping_mul(32))).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(112))).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) & (32768i32))) != 0) {
                                            break 'c4;
                                        }
                                        if (((({ let a_ = (((l_vtx_8).wrapping_add(((((c.m.r32(((l_tri_7).wrapping_add((((l_i_18) as u32)).wrapping_mul(32))).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(112))).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) & (32768i32))) != 0) {
                                            break 'c4;
                                        }
                                        { let s_ = ((l_vtx_8).wrapping_add(((((c.m.r32(((l_tri_7).wrapping_add((((l_i_18) as u32)).wrapping_mul(32))).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(112))).wrapping_add(80); let d_ = l_p_15; rt::memcpy(c, d_, s_, 16); };
                                        { let r_ = 2i32; let n_ = (l_p_15).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_15 = n_; };
                                        { let s_ = fp; let d_ = l_p_15; rt::memcpy(c, d_, s_, 16); };
                                        { let r_ = 2i32; let n_ = (l_p_15).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_15 = n_; };
                                        { let s_ = ((l_vtx_8).wrapping_add(((((c.m.r32(((l_tri_7).wrapping_add((((l_i_18) as u32)).wrapping_mul(32))).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(112))).wrapping_add(48); let d_ = l_p_15; rt::memcpy(c, d_, s_, 16); };
                                        { let r_ = 2i32; let n_ = (l_p_15).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_15 = n_; };
                                        { let s_ = ((l_vtx_8).wrapping_add(((((c.m.r32(((l_tri_7).wrapping_add((((l_i_18) as u32)).wrapping_mul(32))).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(112))).wrapping_add(80); let d_ = l_p_15; rt::memcpy(c, d_, s_, 16); };
                                        { let r_ = 2i32; let n_ = (l_p_15).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_15 = n_; };
                                        { let s_ = fp; let d_ = l_p_15; rt::memcpy(c, d_, s_, 16); };
                                        { let r_ = 2i32; let n_ = (l_p_15).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_15 = n_; };
                                        { let s_ = ((l_vtx_8).wrapping_add(((((c.m.r32(((l_tri_7).wrapping_add((((l_i_18) as u32)).wrapping_mul(32))).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(112))).wrapping_add(48); let d_ = l_p_15; rt::memcpy(c, d_, s_, 16); };
                                        { let r_ = 2i32; let n_ = (l_p_15).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_15 = n_; };
                                        { let s_ = ((l_vtx_8).wrapping_add(((((c.m.r32(((l_tri_7).wrapping_add((((l_i_18) as u32)).wrapping_mul(32))).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(112))).wrapping_add(80); let d_ = l_p_15; rt::memcpy(c, d_, s_, 16); };
                                        { let r_ = 2i32; let n_ = (l_p_15).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_15 = n_; };
                                        { let s_ = fp; let d_ = l_p_15; rt::memcpy(c, d_, s_, 16); };
                                        { let r_ = 2i32; let n_ = (l_p_15).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_15 = n_; };
                                        { let s_ = ((l_vtx_8).wrapping_add(((((c.m.r32(((l_tri_7).wrapping_add((((l_i_18) as u32)).wrapping_mul(32))).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(112))).wrapping_add(48); let d_ = l_p_15; rt::memcpy(c, d_, s_, 16); };
                                        { let r_ = 2i32; let n_ = (l_p_15).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_15 = n_; };
                                        { let o_ = l_n_19; let n_ = (o_).wrapping_add(1); l_n_19 = n_; };
                                    }
                                    { let o_ = l_i_18; let n_ = (o_).wrapping_add(1); l_i_18 = n_; };
                                }
                            }
                            { let v_ = (((((((l_n_19).wrapping_mul(3i32)) | (32768i32))) as u64)) | ((((49335i32) as u64)).wrapping_shl((46i32) as u32))); let a_ = (l_tag_16).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                            { let v_ = ((1042i32) as u64); let a_ = (l_tag_16).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                        } else {
                            if (((((l_pass_17) == (1i32)) as i32)) != 0) {
                                { let v_ = ((((((((2i32) as u64)) | ((((0i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (l_p_15).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                                { let v_ = ((14i32) as u64); let a_ = (l_p_15).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                                { let r_ = 2i32; let n_ = (l_p_15).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_15 = n_; };
                                { let v_ = ((0i32) as u64); let a_ = (l_p_15).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                                { let v_ = ((63i32) as u64); let a_ = (l_p_15).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                                { let r_ = 2i32; let n_ = (l_p_15).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_15 = n_; };
                                { let v_ = l_regShade_14; let a_ = (l_p_15).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                                { let v_ = ((7i32) as u64); let a_ = (l_p_15).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                                { let r_ = 2i32; let n_ = (l_p_15).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_15 = n_; };
                                l_tag_16 = l_p_15;
                                { let r_ = 2i32; let n_ = (l_p_15).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_15 = n_; };
                                {
                                    { l_i_18 = 0i32; l_n_19 = 0i32; };
                                    'l5: loop {
                                        if !(((((l_i_18) < ((c.m.r32((l_sec_6).wrapping_add(4)) as i32))) as i32)) != 0) { break 'l5; }
                                        'c6: {
                                            if (((({ let a_ = (((l_vtx_8).wrapping_add(((((c.m.r32(((l_tri_7).wrapping_add((((l_i_18) as u32)).wrapping_mul(32))).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(112))).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) & (32768i32))) != 0) {
                                                break 'c6;
                                            }
                                            if (((({ let a_ = (((l_vtx_8).wrapping_add(((((c.m.r32(((l_tri_7).wrapping_add((((l_i_18) as u32)).wrapping_mul(32))).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(112))).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) & (32768i32))) != 0) {
                                                break 'c6;
                                            }
                                            if (((({ let a_ = (((l_vtx_8).wrapping_add(((((c.m.r32(((l_tri_7).wrapping_add((((l_i_18) as u32)).wrapping_mul(32))).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(112))).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) & (32768i32))) != 0) {
                                                break 'c6;
                                            }
                                            { let s_ = ((l_vtx_8).wrapping_add(((((c.m.r32(((l_tri_7).wrapping_add((((l_i_18) as u32)).wrapping_mul(32))).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(112))).wrapping_add(96); let d_ = l_p_15; rt::memcpy(c, d_, s_, 16); };
                                            { let r_ = 2i32; let n_ = (l_p_15).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_15 = n_; };
                                            { let s_ = fp; let d_ = l_p_15; rt::memcpy(c, d_, s_, 16); };
                                            { let r_ = 2i32; let n_ = (l_p_15).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_15 = n_; };
                                            { let s_ = ((l_vtx_8).wrapping_add(((((c.m.r32(((l_tri_7).wrapping_add((((l_i_18) as u32)).wrapping_mul(32))).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(112))).wrapping_add(48); let d_ = l_p_15; rt::memcpy(c, d_, s_, 16); };
                                            { let r_ = 2i32; let n_ = (l_p_15).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_15 = n_; };
                                            { let s_ = ((l_vtx_8).wrapping_add(((((c.m.r32(((l_tri_7).wrapping_add((((l_i_18) as u32)).wrapping_mul(32))).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(112))).wrapping_add(96); let d_ = l_p_15; rt::memcpy(c, d_, s_, 16); };
                                            { let r_ = 2i32; let n_ = (l_p_15).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_15 = n_; };
                                            { let s_ = fp; let d_ = l_p_15; rt::memcpy(c, d_, s_, 16); };
                                            { let r_ = 2i32; let n_ = (l_p_15).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_15 = n_; };
                                            { let s_ = ((l_vtx_8).wrapping_add(((((c.m.r32(((l_tri_7).wrapping_add((((l_i_18) as u32)).wrapping_mul(32))).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(112))).wrapping_add(48); let d_ = l_p_15; rt::memcpy(c, d_, s_, 16); };
                                            { let r_ = 2i32; let n_ = (l_p_15).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_15 = n_; };
                                            { let s_ = ((l_vtx_8).wrapping_add(((((c.m.r32(((l_tri_7).wrapping_add((((l_i_18) as u32)).wrapping_mul(32))).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(112))).wrapping_add(96); let d_ = l_p_15; rt::memcpy(c, d_, s_, 16); };
                                            { let r_ = 2i32; let n_ = (l_p_15).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_15 = n_; };
                                            { let s_ = fp; let d_ = l_p_15; rt::memcpy(c, d_, s_, 16); };
                                            { let r_ = 2i32; let n_ = (l_p_15).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_15 = n_; };
                                            { let s_ = ((l_vtx_8).wrapping_add(((((c.m.r32(((l_tri_7).wrapping_add((((l_i_18) as u32)).wrapping_mul(32))).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(112))).wrapping_add(48); let d_ = l_p_15; rt::memcpy(c, d_, s_, 16); };
                                            { let r_ = 2i32; let n_ = (l_p_15).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_15 = n_; };
                                            { let o_ = l_n_19; let n_ = (o_).wrapping_add(1); l_n_19 = n_; };
                                        }
                                        { let o_ = l_i_18; let n_ = (o_).wrapping_add(1); l_i_18 = n_; };
                                    }
                                }
                                { let v_ = (((((((l_n_19).wrapping_mul(3i32)) | (32768i32))) as u64)) | ((((50359i32) as u64)).wrapping_shl((46i32) as u32))); let a_ = (l_tag_16).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                                { let v_ = ((1042i32) as u64); let a_ = (l_tag_16).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                            }
                        }
                    }
                    { let o_ = l_pass_17; let n_ = (o_).wrapping_add(1); l_pass_17 = n_; };
                }
            }
            F::Dma_EndDirect(c, l_p_15);
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "ObjSeam_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn ObjDraw_MakeFacingMtx(c: &mut Ctx, mut p_m_1: u32, mut p_dir_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ObjDraw_MakeFacingMtx", &PROBE_N, &[(p_m_1) as u32, (p_dir_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let r_ = 'f: {
        rt::memset(c, fp, 0, 16);
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(0), (v_).0); }
        { let v_ = F32(0xbf800000); c.m.w32(fp.wrapping_add(4), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(8), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(12), (v_).0); }
        F::Vec3_Cross(c, fp.wrapping_add(16), p_dir_2, fp);
        F::Vec3_Normalize(c, fp.wrapping_add(16), fp.wrapping_add(16));
        F::Vec3_Cross(c, fp, fp.wrapping_add(16), p_dir_2);
        F::Vec3_Normalize(c, fp, fp);
        F::Mtx_StoreIdentity(c, p_m_1);
        { let v_ = F32(c.m.r32(fp.wrapping_add(16))); let a_ = ((p_m_1).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32(fp)); let a_ = ((p_m_1).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32(p_dir_2)); let a_ = ((p_m_1).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))); let a_ = ((p_m_1).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((fp).wrapping_add(4))); let a_ = ((p_m_1).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((p_dir_2).wrapping_add(4))); let a_ = ((p_m_1).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8))); let a_ = ((p_m_1).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((fp).wrapping_add(8))); let a_ = ((p_m_1).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((p_dir_2).wrapping_add(8))); let a_ = ((p_m_1).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "ObjDraw_MakeFacingMtx", &PROBE_N, &[], true); }
    r_
}

pub fn ObjDraw_UploadTextures(c: &mut Ctx, mut p_obj_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ObjDraw_UploadTextures", &PROBE_N, &[(p_obj_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let mut l_mdl_2: u32 = 0;
    let mut l_file_3: u32 = 0;
    let mut l_e_4: u32 = 0;
    let mut l_face_5: u32 = 0;
    let mut l_i_6: i32 = 0;
    let mut l_j_7: i32 = 0;
    let mut l_tbp_8: i32 = 0;
    let mut l_cbp_9: i32 = 0;
    let mut l_count_10: u8 = 0;
    let mut l_first_11: u8 = 0;
    let r_ = 'f: {
        rt::memset(c, fp, 0, 48);
        { let v_ = ((((268435456i32) | (2i32))) as u32); c.m.w32(fp.wrapping_add(0), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(4), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(8), v_); }
        { let v_ = ((((1342177280i32) | (2i32))) as u32); c.m.w32(fp.wrapping_add(12), v_); }
        { let v_ = ((((32768i32) | (1i32))) as u32); c.m.w32(fp.wrapping_add(16), v_); }
        { let v_ = ((268435456i32) as u32); c.m.w32(fp.wrapping_add(20), v_); }
        { let v_ = ((14i32) as u32); c.m.w32(fp.wrapping_add(24), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(28), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(32), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(36), v_); }
        { let v_ = ((80i32) as u32); c.m.w32(fp.wrapping_add(40), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(44), v_); }
        l_mdl_2 = (p_obj_1).wrapping_add(24);
        l_i_6 = 0i32;
        l_file_3 = c.m.r32(((p_obj_1).wrapping_add(24)).wrapping_add(48));
        l_first_11 = { let a_ = (c.m.r32(((p_obj_1).wrapping_add(24)).wrapping_add(40))).wrapping_add(78); c.m.r8(a_) };
        l_count_10 = { let a_ = (c.m.r32(((p_obj_1).wrapping_add(24)).wrapping_add(40))).wrapping_add(79); c.m.r8(a_) };
        {
            'l1: loop {
                if !(((((((l_i_6) as u32)) < (c.m.r32(l_file_3))) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_e_4 = (c.m.r32((l_file_3).wrapping_add(16))).wrapping_add((((l_i_6) as u32)).wrapping_mul(64));
                    if (((((((((((((l_i_6) >= (((l_first_11) as i32))) as i32)) != 0) && (((((l_i_6) < ((((l_first_11) as i32)).wrapping_add(((l_count_10) as i32)))) as i32)) != 0)) as i32)) != 0) && (((((l_i_6) != ((({ let a_ = (c.m.r32(((p_obj_1).wrapping_add(24)).wrapping_add(40))).wrapping_add(76); c.m.r8(a_) }) as i32))) as i32)) != 0)) as i32)) != 0) {
                        l_tbp_8 = ((c.m.r32((l_e_4).wrapping_add(32)) as i32)).wrapping_add({ let a_ = (c.m.r32((l_mdl_2).wrapping_add(40))).wrapping_add(88); (c.m.r32(a_) as i32) });
                        l_cbp_9 = ((c.m.r32((l_e_4).wrapping_add(36)) as i32)).wrapping_add({ let a_ = (c.m.r32((l_mdl_2).wrapping_add(40))).wrapping_add(92); (c.m.r32(a_) as i32) });
                        { let v_ = ((0i32) as u32); let a_ = (fp).wrapping_add((((8i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                        { let v_ = ((((((((l_tbp_8) as u64)).wrapping_shl((32i32) as u32)) | ((((c.m.r32((l_e_4).wrapping_add(24))) as u64)).wrapping_shl((48i32) as u32)))).wrapping_shr((32i32) as u32)) as u32); let a_ = (fp).wrapping_add((((9i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                        F::Dma_AddData(c, fp, 48i32);
                        { let a0_ = c.m.r32((l_e_4).wrapping_add(56)); let a1_ = (c.m.r32((l_e_4).wrapping_add(8)) as i32); F::Dma_AddRef(c, a0_, a1_) };
                        { let v_ = ((0i32) as u32); let a_ = (fp).wrapping_add((((8i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                        { let v_ = ((((((((l_cbp_9) as u64)).wrapping_shl((32i32) as u32)) | ((((c.m.r32((l_e_4).wrapping_add(28))) as u64)).wrapping_shl((48i32) as u32)))).wrapping_shr((32i32) as u32)) as u32); let a_ = (fp).wrapping_add((((9i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                        F::Dma_AddData(c, fp, 48i32);
                        { let a0_ = c.m.r32((l_e_4).wrapping_add(60)); let a1_ = (c.m.r32((l_e_4).wrapping_add(12)) as i32); F::Dma_AddRef(c, a0_, a1_) };
                    } else {
                        if (((((((((((((l_i_6) >= (((l_first_11) as i32))) as i32)) != 0) && (((((l_i_6) < ((((l_first_11) as i32)).wrapping_add(((l_count_10) as i32)))) as i32)) != 0)) as i32)) != 0) && (((((l_i_6) == ((({ let a_ = (c.m.r32(((p_obj_1).wrapping_add(24)).wrapping_add(40))).wrapping_add(76); c.m.r8(a_) }) as i32))) as i32)) != 0)) as i32)) != 0) {
                            l_face_5 = F::BtlObj_GetFaceTexture(c, p_obj_1);
                            l_tbp_8 = ((c.m.r32((l_e_4).wrapping_add(32)) as i32)).wrapping_add({ let a_ = (c.m.r32((l_mdl_2).wrapping_add(40))).wrapping_add(88); (c.m.r32(a_) as i32) });
                            l_cbp_9 = ((c.m.r32((l_e_4).wrapping_add(36)) as i32)).wrapping_add({ let a_ = (c.m.r32((l_mdl_2).wrapping_add(40))).wrapping_add(92); (c.m.r32(a_) as i32) });
                            if (((((l_face_5) != (((0i32) as u32))) as i32)) != 0) {
                                l_e_4 = l_face_5;
                                { let v_ = ((0i32) as u32); let a_ = (fp).wrapping_add((((8i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                                { let v_ = ((((((((l_tbp_8) as u64)).wrapping_shl((32i32) as u32)) | ((((c.m.r32((l_e_4).wrapping_add(24))) as u64)).wrapping_shl((48i32) as u32)))).wrapping_shr((32i32) as u32)) as u32); let a_ = (fp).wrapping_add((((9i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                                F::Dma_AddData(c, fp, 48i32);
                                { let a0_ = c.m.r32((l_e_4).wrapping_add(56)); let a1_ = (c.m.r32((l_e_4).wrapping_add(8)) as i32); F::Dma_AddRef(c, a0_, a1_) };
                                { let v_ = ((0i32) as u32); let a_ = (fp).wrapping_add((((8i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                                { let v_ = ((((((((l_cbp_9) as u64)).wrapping_shl((32i32) as u32)) | ((((c.m.r32((l_e_4).wrapping_add(28))) as u64)).wrapping_shl((48i32) as u32)))).wrapping_shr((32i32) as u32)) as u32); let a_ = (fp).wrapping_add((((9i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                                F::Dma_AddData(c, fp, 48i32);
                                { let a0_ = c.m.r32((l_e_4).wrapping_add(60)); let a1_ = (c.m.r32((l_e_4).wrapping_add(12)) as i32); F::Dma_AddRef(c, a0_, a1_) };
                            } else {
                                { let v_ = ((0i32) as u32); let a_ = (fp).wrapping_add((((8i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                                { let v_ = ((((((((l_tbp_8) as u64)).wrapping_shl((32i32) as u32)) | ((((c.m.r32((l_e_4).wrapping_add(24))) as u64)).wrapping_shl((48i32) as u32)))).wrapping_shr((32i32) as u32)) as u32); let a_ = (fp).wrapping_add((((9i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                                F::Dma_AddData(c, fp, 48i32);
                                { let a0_ = c.m.r32((l_e_4).wrapping_add(56)); let a1_ = (c.m.r32((l_e_4).wrapping_add(8)) as i32); F::Dma_AddRef(c, a0_, a1_) };
                                { let v_ = ((0i32) as u32); let a_ = (fp).wrapping_add((((8i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                                { let v_ = ((((((((l_cbp_9) as u64)).wrapping_shl((32i32) as u32)) | ((((c.m.r32((l_e_4).wrapping_add(28))) as u64)).wrapping_shl((48i32) as u32)))).wrapping_shr((32i32) as u32)) as u32); let a_ = (fp).wrapping_add((((9i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                                F::Dma_AddData(c, fp, 48i32);
                                { let a0_ = c.m.r32((l_e_4).wrapping_add(60)); let a1_ = (c.m.r32((l_e_4).wrapping_add(12)) as i32); F::Dma_AddRef(c, a0_, a1_) };
                            }
                        }
                    }
                }
                { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; };
            }
        }
        if (((((((((rt::bf_get_u(c, ((p_obj_1).wrapping_add(2624)).wrapping_add(2), 0, 1) as u32)) as i32)) != 0) || (((((rt::bf_get_u(c, ((p_obj_1).wrapping_add(2624)).wrapping_add(2), 1, 1) as u32)) as i32)) != 0)) as i32)) != 0) {
            {
                l_j_7 = 0i32;
                'l3: loop {
                    if !(((((l_j_7) < ((({ let a_ = (c.m.r32(((p_obj_1).wrapping_add(24)).wrapping_add(40))).wrapping_add(81); c.m.r8(a_) }) as i32))) as i32)) != 0) { break 'l3; }
                    'c4: {
                        { let a0_ = c.m.r32(((p_obj_1).wrapping_add(24)).wrapping_add(48)); let a1_ = (({ let a_ = (c.m.r32((l_mdl_2).wrapping_add(40))).wrapping_add(80); c.m.r8(a_) }) as i32); let a2_ = (({ let a_ = (c.m.r32((l_mdl_2).wrapping_add(40))).wrapping_add(81); c.m.r8(a_) }) as i32); let a3_ = { let a_ = (c.m.r32((l_mdl_2).wrapping_add(40))).wrapping_add(96); (c.m.r32(a_) as i32) }; F::TexFile_UploadRange(c, a0_, a1_, a2_, a3_, (1i32).wrapping_neg()) };
                    }
                    { let o_ = l_j_7; let n_ = (o_).wrapping_add(1); l_j_7 = n_; };
                }
            }
        } else {
            if ((((c.m.r32((p_obj_1).wrapping_add(2624))) & (((524288i32) as u32)))) != 0) {
                {
                    l_j_7 = 0i32;
                    'l5: loop {
                        if !(((((l_j_7) < ((({ let a_ = (c.m.r32(((p_obj_1).wrapping_add(24)).wrapping_add(40))).wrapping_add(81); c.m.r8(a_) }) as i32))) as i32)) != 0) { break 'l5; }
                        'c6: {
                            { let a0_ = ({ let a_ = (c.m.r32((l_mdl_2).wrapping_add(40))).wrapping_add(96); (c.m.r32(a_) as i32) }).wrapping_add((l_j_7).wrapping_mul(4i32)); F::BtlObjLight_FindRes1(c, a0_) };
                        }
                        { let o_ = l_j_7; let n_ = (o_).wrapping_add(1); l_j_7 = n_; };
                    }
                }
            } else {
                if ((((c.m.r32((p_obj_1).wrapping_add(2624))) & (((262144i32) as u32)))) != 0) {
                    {
                        l_j_7 = 0i32;
                        'l7: loop {
                            if !(((((l_j_7) < ((({ let a_ = (c.m.r32(((p_obj_1).wrapping_add(24)).wrapping_add(40))).wrapping_add(81); c.m.r8(a_) }) as i32))) as i32)) != 0) { break 'l7; }
                            'c8: {
                                { let a0_ = c.m.r32(((p_obj_1).wrapping_add(24)).wrapping_add(48)); let a1_ = (({ let a_ = (c.m.r32((l_mdl_2).wrapping_add(40))).wrapping_add(80); c.m.r8(a_) }) as i32); let a2_ = (({ let a_ = (c.m.r32((l_mdl_2).wrapping_add(40))).wrapping_add(81); c.m.r8(a_) }) as i32); let a3_ = { let a_ = (c.m.r32((l_mdl_2).wrapping_add(40))).wrapping_add(96); (c.m.r32(a_) as i32) }; F::TexFile_UploadRange(c, a0_, a1_, a2_, a3_, (1i32).wrapping_neg()) };
                            }
                            { let o_ = l_j_7; let n_ = (o_).wrapping_add(1); l_j_7 = n_; };
                        }
                    }
                } else {
                    if ((((c.m.r32((p_obj_1).wrapping_add(2624))) & (((4194304i32) as u32)))) != 0) {
                        {
                            l_j_7 = 0i32;
                            'l9: loop {
                                if !(((((l_j_7) < ((({ let a_ = (c.m.r32(((p_obj_1).wrapping_add(24)).wrapping_add(40))).wrapping_add(81); c.m.r8(a_) }) as i32))) as i32)) != 0) { break 'l9; }
                                'c10: {
                                    { let a0_ = c.m.r32(((p_obj_1).wrapping_add(24)).wrapping_add(48)); let a1_ = (({ let a_ = (c.m.r32((l_mdl_2).wrapping_add(40))).wrapping_add(80); c.m.r8(a_) }) as i32); let a2_ = (({ let a_ = (c.m.r32((l_mdl_2).wrapping_add(40))).wrapping_add(81); c.m.r8(a_) }) as i32); let a3_ = { let a_ = (c.m.r32((l_mdl_2).wrapping_add(40))).wrapping_add(96); (c.m.r32(a_) as i32) }; F::TexFile_UploadRange(c, a0_, a1_, a2_, a3_, (1i32).wrapping_neg()) };
                                }
                                { let o_ = l_j_7; let n_ = (o_).wrapping_add(1); l_j_7 = n_; };
                            }
                        }
                    }
                }
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "ObjDraw_UploadTextures", &PROBE_N, &[], true); }
    r_
}

pub fn gfxm_d_c__ObjDraw_GetDimAlpha(c: &mut Ctx, mut p_mdl_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "gfxm_d_c__ObjDraw_GetDimAlpha", &PROBE_N, &[(p_mdl_1) as u32], false); }
    let r_ = 'f: {
    if (((((c.m.r32((p_mdl_1).wrapping_add(2308))) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 244i32;
    }
    break 'f ((((((({ let a_ = (c.m.r32((p_mdl_1).wrapping_add(2308))).wrapping_add(((3i32) as u32)); c.m.r8(a_) }) as i32)).wrapping_add(244i32)) as u8)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "gfxm_d_c__ObjDraw_GetDimAlpha", &PROBE_N, &[], true); }
    r_
}

pub fn ObjDraw_AddClutPass(c: &mut Ctx, mut p_obj_1: u32, mut p_state_2: u32, mut p_view_3: u32, mut p_dim_4: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ObjDraw_AddClutPass", &PROBE_N, &[(p_obj_1) as u32, (p_state_2) as u32, (p_view_3) as u32, (p_dim_4) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_p_5: u32 = 0;
    let mut l_mdl_6: u32 = 0;
    let mut l_a_7: i32 = 0;
    let mut l_a_8: i32 = 0;
    let mut l_a_9: i32 = 0;
    let r_ = 'f: {
        l_p_5 = F::Dma_BeginDirect(c);
        { let v_ = ((((((((11i32) as u64)) | ((((0i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (l_p_5).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((14i32) as u64); let a_ = (l_p_5).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_5).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_5 = n_; };
        l_mdl_6 = (p_obj_1).wrapping_add(24);
        { let v_ = ((((((({ let a_ = (c.m.r32((l_mdl_6).wrapping_add(40))).wrapping_add(92); (c.m.r32(a_) as i32) }) as u32)).wrapping_shr((5i32) as u32)) | (((65536i32) as u32)))) as u64); let a_ = (l_p_5).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((76i32) as u64); let a_ = (l_p_5).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_5).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_5 = n_; };
        { let v_ = ((((((224i32) as u64)) | ((((49i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((32i32) as u32))); let a_ = (l_p_5).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((78i32) as u64); let a_ = (l_p_5).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_5).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_5 = n_; };
        { let v_ = ((((32256i32) as u64)) | ((((32256i32) as u64)).wrapping_shl((32i32) as u32))); let a_ = (l_p_5).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((24i32) as u64); let a_ = (l_p_5).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_5).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_5 = n_; };
        { let v_ = ((((((((0i32) as u64)) | ((((63i32) as u64)).wrapping_shl((16i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((32i32) as u32)))) | ((((63i32) as u64)).wrapping_shl((48i32) as u32))); let a_ = (l_p_5).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((64i32) as u64); let a_ = (l_p_5).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_5).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_5 = n_; };
        { let v_ = ((1082332790794i64) as u64); let a_ = (l_p_5).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((8i32) as u64); let a_ = (l_p_5).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_5).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_5 = n_; };
        { let v_ = ((196608i32) as u64); let a_ = (l_p_5).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((71i32) as u64); let a_ = (l_p_5).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_5).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_5 = n_; };
        { let v_ = ((549755813988i64) as u64); let a_ = (l_p_5).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((66i32) as u64); let a_ = (l_p_5).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_5).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_5 = n_; };
        { let v_ = ((0i32) as u64); let a_ = (l_p_5).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((20i32) as u64); let a_ = (l_p_5).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_5).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_5 = n_; };
        { let v_ = ((0i32) as u64); let a_ = (l_p_5).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((63i32) as u64); let a_ = (l_p_5).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_5).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_5 = n_; };
        { let v_ = ((342i32) as u64); let a_ = (l_p_5).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((0i32) as u64); let a_ = (l_p_5).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_5).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_5 = n_; };
        { let v_ = ((4575657223564329088i64) as u64); let a_ = (l_p_5).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((1i32) as u64); let a_ = (l_p_5).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_5).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_5 = n_; };
        if ((p_dim_4) != 0) {
            l_a_7 = F::gfxm_d_c__ObjDraw_GetDimAlpha(c, l_mdl_6);
            { let v_ = ((((((((3i32) as u64)) | ((((0i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (l_p_5).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let v_ = ((14i32) as u64); let a_ = (l_p_5).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let r_ = 2i32; let n_ = (l_p_5).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_5 = n_; };
            { let v_ = ((70i32) as u64); let a_ = (l_p_5).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let v_ = ((0i32) as u64); let a_ = (l_p_5).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let r_ = 2i32; let n_ = (l_p_5).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_5 = n_; };
            { let v_ = (((((l_a_7) as u64)).wrapping_shl((24i32) as u32)) | (((4575657221408424191i64) as u64))); let a_ = (l_p_5).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let v_ = ((1i32) as u64); let a_ = (l_p_5).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let r_ = 2i32; let n_ = (l_p_5).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_5 = n_; };
            { let v_ = ((68i32) as u64); let a_ = (l_p_5).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let v_ = ((66i32) as u64); let a_ = (l_p_5).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let r_ = 2i32; let n_ = (l_p_5).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_5 = n_; };
            { let v_ = ((2594073385365405697i64) as u64); let a_ = (l_p_5).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let v_ = ((85i32) as u64); let a_ = (l_p_5).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let r_ = 2i32; let n_ = (l_p_5).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_5 = n_; };
            { let v_ = ((2113961472i32) as u64); let a_ = (l_p_5).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let v_ = ((2181071360u32) as u64); let a_ = (l_p_5).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let r_ = 2i32; let n_ = (l_p_5).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_5 = n_; };
        }
        if ((F::BtlObjFade_Get(c, p_obj_1, fp.wrapping_add(20))) != 0) {
            l_a_8 = rt::f2i(rt::fmul(F32(c.m.r32(fp.wrapping_add(20))), F32(0x428c0000)));
            { let v_ = ((((((((4i32) as u64)) | ((((0i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (l_p_5).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let v_ = ((14i32) as u64); let a_ = (l_p_5).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let r_ = 2i32; let n_ = (l_p_5).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_5 = n_; };
            { let v_ = ((((((((({ let a_ = (c.m.r32((l_mdl_6).wrapping_add(40))).wrapping_add(92); (c.m.r32(a_) as i32) }) as u32)).wrapping_shr((5i32) as u32)) | (((65536i32) as u32)))) as u64)) | (18374686479671623680u64)); let a_ = (l_p_5).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let v_ = ((76i32) as u64); let a_ = (l_p_5).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let r_ = 2i32; let n_ = (l_p_5).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_5 = n_; };
            { let v_ = ((70i32) as u64); let a_ = (l_p_5).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let v_ = ((0i32) as u64); let a_ = (l_p_5).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let r_ = 2i32; let n_ = (l_p_5).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_5 = n_; };
            { let v_ = ((4575657221415126271i64) as u64); let a_ = (l_p_5).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let v_ = ((1i32) as u64); let a_ = (l_p_5).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let r_ = 2i32; let n_ = (l_p_5).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_5 = n_; };
            { let v_ = (((((l_a_8) as u64)).wrapping_shl((32i32) as u32)) | (((100i32) as u64))); let a_ = (l_p_5).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let v_ = ((66i32) as u64); let a_ = (l_p_5).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let r_ = 2i32; let n_ = (l_p_5).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_5 = n_; };
            { let v_ = ((2594073385365405697i64) as u64); let a_ = (l_p_5).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let v_ = ((85i32) as u64); let a_ = (l_p_5).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let r_ = 2i32; let n_ = (l_p_5).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_5 = n_; };
            { let v_ = ((2113961472i32) as u64); let a_ = (l_p_5).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let v_ = ((2181071360u32) as u64); let a_ = (l_p_5).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let r_ = 2i32; let n_ = (l_p_5).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_5 = n_; };
        }
        if ((((c.m.r32((p_obj_1).wrapping_add(2624))) & (((4194304i32) as u32)))) != 0) {
            { let v_ = ((rt::f2u(F32(c.m.r32((p_state_2).wrapping_add(204))))) as u8); let a_ = (fp.wrapping_add(16)).wrapping_add(((0i32) as u32)); c.m.w8(a_, v_); };
            { let v_ = ((rt::f2u(F32(c.m.r32(((p_state_2).wrapping_add(204)).wrapping_add(4))))) as u8); let a_ = (fp.wrapping_add(16)).wrapping_add(((1i32) as u32)); c.m.w8(a_, v_); };
            { let v_ = ((rt::f2u(F32(c.m.r32(((p_state_2).wrapping_add(204)).wrapping_add(8))))) as u8); let a_ = (fp.wrapping_add(16)).wrapping_add(((2i32) as u32)); c.m.w8(a_, v_); };
            { let v_ = ((rt::f2u(F32(c.m.r32(((p_state_2).wrapping_add(204)).wrapping_add(12))))) as u8); let a_ = (fp.wrapping_add(16)).wrapping_add(((3i32) as u32)); c.m.w8(a_, v_); };
            { let v_ = ((((((((4i32) as u64)) | ((((0i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (l_p_5).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let v_ = ((14i32) as u64); let a_ = (l_p_5).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let r_ = 2i32; let n_ = (l_p_5).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_5 = n_; };
            { let v_ = ((((((((({ let a_ = (c.m.r32((l_mdl_6).wrapping_add(40))).wrapping_add(92); (c.m.r32(a_) as i32) }) as u32)).wrapping_shr((5i32) as u32)) | (((65536i32) as u32)))) as u64)) | (18374686479671623680u64)); let a_ = (l_p_5).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let v_ = ((76i32) as u64); let a_ = (l_p_5).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let r_ = 2i32; let n_ = (l_p_5).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_5 = n_; };
            { let v_ = ((70i32) as u64); let a_ = (l_p_5).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let v_ = ((0i32) as u64); let a_ = (l_p_5).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let r_ = 2i32; let n_ = (l_p_5).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_5 = n_; };
            { let v_ = ((((((((c.m.r8((fp.wrapping_add(16)).wrapping_add(((0i32) as u32)))) as u64)) | ((((c.m.r8((fp.wrapping_add(16)).wrapping_add(((1i32) as u32)))) as u64)).wrapping_shl((8i32) as u32)))) | ((((c.m.r8((fp.wrapping_add(16)).wrapping_add(((2i32) as u32)))) as u64)).wrapping_shl((16i32) as u32)))) | (((4575657221408423936i64) as u64))); let a_ = (l_p_5).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let v_ = ((1i32) as u64); let a_ = (l_p_5).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let r_ = 2i32; let n_ = (l_p_5).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_5 = n_; };
            { let v_ = (((((c.m.r8((fp.wrapping_add(16)).wrapping_add(((3i32) as u32)))) as u64)).wrapping_shl((32i32) as u32)) | (((104i32) as u64))); let a_ = (l_p_5).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let v_ = ((66i32) as u64); let a_ = (l_p_5).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let r_ = 2i32; let n_ = (l_p_5).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_5 = n_; };
            { let v_ = ((2594073385365405697i64) as u64); let a_ = (l_p_5).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let v_ = ((85i32) as u64); let a_ = (l_p_5).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let r_ = 2i32; let n_ = (l_p_5).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_5 = n_; };
            { let v_ = ((2113961472i32) as u64); let a_ = (l_p_5).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let v_ = ((2181071360u32) as u64); let a_ = (l_p_5).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let r_ = 2i32; let n_ = (l_p_5).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_5 = n_; };
        }
        if ((F::BtlObjFlash_GetColor(c, p_obj_1, fp, fp.wrapping_add(20))) != 0) {
            l_a_9 = rt::f2i(rt::fmul(F32(c.m.r32(fp.wrapping_add(20))), F32(0x43000000)));
            if (((((l_a_9) > (128i32)) as i32)) != 0) {
                l_a_9 = 128i32;
            }
            { let v_ = ((((((((4i32) as u64)) | ((((0i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (l_p_5).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let v_ = ((14i32) as u64); let a_ = (l_p_5).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let r_ = 2i32; let n_ = (l_p_5).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_5 = n_; };
            { let v_ = ((((((((({ let a_ = (c.m.r32((l_mdl_6).wrapping_add(40))).wrapping_add(92); (c.m.r32(a_) as i32) }) as u32)).wrapping_shr((5i32) as u32)) | (((65536i32) as u32)))) as u64)) | (18374686479671623680u64)); let a_ = (l_p_5).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let v_ = ((76i32) as u64); let a_ = (l_p_5).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let r_ = 2i32; let n_ = (l_p_5).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_5 = n_; };
            { let v_ = ((70i32) as u64); let a_ = (l_p_5).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let v_ = ((0i32) as u64); let a_ = (l_p_5).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let r_ = 2i32; let n_ = (l_p_5).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_5 = n_; };
            { let v_ = ((((((((rt::f2u(F32(c.m.r32(fp)))) as u8)) as u64)) | ((((((((rt::f2u(F32(c.m.r32((fp).wrapping_add(8))))) as u8)) as u64)).wrapping_shl((16i32) as u32)) | ((((((rt::f2u(F32(c.m.r32((fp).wrapping_add(4))))) as u8)) as u64)).wrapping_shl((8i32) as u32)))))) | (((4575657221408423936i64) as u64))); let a_ = (l_p_5).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let v_ = ((1i32) as u64); let a_ = (l_p_5).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let r_ = 2i32; let n_ = (l_p_5).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_5 = n_; };
            { let v_ = (((((l_a_9) as u64)).wrapping_shl((32i32) as u32)) | (((104i32) as u64))); let a_ = (l_p_5).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let v_ = ((66i32) as u64); let a_ = (l_p_5).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let r_ = 2i32; let n_ = (l_p_5).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_5 = n_; };
            { let v_ = ((2594073385365405697i64) as u64); let a_ = (l_p_5).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let v_ = ((85i32) as u64); let a_ = (l_p_5).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let r_ = 2i32; let n_ = (l_p_5).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_5 = n_; };
            { let v_ = ((2113961472i32) as u64); let a_ = (l_p_5).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let v_ = ((2181071360u32) as u64); let a_ = (l_p_5).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let r_ = 2i32; let n_ = (l_p_5).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_5 = n_; };
        }
        F::Dma_EndDirect(c, l_p_5);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "ObjDraw_AddClutPass", &PROBE_N, &[], true); }
    r_
}

pub fn ObjDraw_DrawParts(c: &mut Ctx, mut p_obj_1: u32, mut p_view_2: u32, mut p_mode_3: i32, mut p_tex0_4: u64, mut p_fbmsk_5: u32, mut p_noMask_6: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ObjDraw_DrawParts", &PROBE_N, &[(p_obj_1) as u32, (p_view_2) as u32, (p_mode_3) as u32, (p_tex0_4) as u32, (p_fbmsk_5) as u32, (p_noMask_6) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(224) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 224);
    let mut l_mdl_7: u32 = 0;
    let mut l_headNode_8: u32 = 0;
    let mut l_headPart_9: u32 = 0;
    let mut l_part_10: u32 = 0;
    let mut l_node_11: u32 = 0;
    let mut l_k_12: F32 = F32(0);
    let mut l_pkt_13: u32 = 0;
    let mut l_pkt_14: u32 = 0;
    let mut l_pkt_15: u32 = 0;
    let mut l_pkt_16: u32 = 0;
    let r_ = 'f: {
        l_headNode_8 = ((0i32) as u32);
        l_headPart_9 = ((0i32) as u32);
        if (((((p_mode_3) == (1i32)) as i32)) != 0) {
            { let v_ = { let a_ = (((c.m.r32(sym::gBtlCamView)).wrapping_add(64)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }; let a_ = fp.wrapping_add(192); c.m.w32(a_, (v_).0); };
            { let v_ = { let a_ = (((c.m.r32(sym::gBtlCamView)).wrapping_add(64)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }; let a_ = (fp.wrapping_add(192)).wrapping_add(4); c.m.w32(a_, (v_).0); };
            { let v_ = { let a_ = (((c.m.r32(sym::gBtlCamView)).wrapping_add(64)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }; let a_ = (fp.wrapping_add(192)).wrapping_add(8); c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x3f800000); let a_ = (fp.wrapping_add(192)).wrapping_add(12); c.m.w32(a_, (v_).0); };
            F::ObjDraw_MakeFacingMtx(c, fp, fp.wrapping_add(192));
            F::Mtx_InverseRT(c, fp, fp);
        }
        if (((((p_mode_3) == (2i32)) as i32)) != 0) {
            F::Vec4_Copy(c, fp.wrapping_add(208), (p_view_2).wrapping_add(48));
        }
        l_part_10 = c.m.r32(((p_obj_1).wrapping_add(24)).wrapping_add(44));
        l_mdl_7 = (p_obj_1).wrapping_add(24);
        l_k_12 = F32(0x43000000);
        'l1: while ((1i32) != 0) {
            if ((c.m.r16((l_part_10).wrapping_add(8))) != 0) {
                l_node_11 = { let a1_ = ((c.m.r16((l_part_10).wrapping_add(10))) as i32); F::BtlObj_GetNode(c, p_obj_1, a1_) };
                if ((c.m.r8((l_node_11).wrapping_add(12))) != 0) {
                    if (((((p_mode_3) == (0i32)) as i32)) != 0) {
                        if (((((((c.m.r16((l_part_10).wrapping_add(10))) as i32)) == (51i32)) as i32)) != 0) {
                            l_headNode_8 = l_node_11;
                            l_headPart_9 = l_part_10;
                        } else {
                            l_pkt_13 = { let a0_ = (if (((((((((((c.m.r16((l_part_10).wrapping_add(10))) as i32)) == (48i32)) as i32)) != 0) && (((((F::BtlObj_GetSubState(c, p_obj_1)) != ((1i32).wrapping_neg())) as i32)) != 0)) as i32)) != 0) { { let a_ = ((l_mdl_7).wrapping_add(68)).wrapping_add((((F::BtlObj_GetSubState(c, p_obj_1)) as u32)).wrapping_mul(4)); c.m.r32(a_) } } else { (l_part_10).wrapping_add(96) }); F::Vu1Pkt_CallProg0(c, a0_) };
                            F::Mtx_Copy(c, (l_pkt_13).wrapping_add(80), (l_node_11).wrapping_add(80));
                            F::Mtx_Copy(c, (l_pkt_13).wrapping_add(16), (l_node_11).wrapping_add(16));
                            F::Vec4_Copy(c, (l_pkt_13).wrapping_add(144), (l_part_10).wrapping_add(16));
                            F::Vec4_Copy(c, (l_pkt_13).wrapping_add(160), (l_part_10).wrapping_add(32));
                            F::Mtx_MakeNormalLight(c, (l_pkt_13).wrapping_add(176), (p_view_2).wrapping_add(16), sym::D_002EC2C0, sym::D_002EC2C0);
                            F::Vec4_Copy(c, (l_pkt_13).wrapping_add(368), (p_view_2).wrapping_add(32));
                            F::Vec4_Copy(c, (l_pkt_13).wrapping_add(384), sym::D_002C3440);
                            F::Vu0Screen_StoreMtx(c, (l_pkt_13).wrapping_add(240));
                            F::Vu0Clip_StoreMtx(c, (l_pkt_13).wrapping_add(304));
                        }
                    } else {
                        if (((((p_mode_3) == (1i32)) as i32)) != 0) {
                            if ((((((c.m.r16((l_part_10).wrapping_add(12))) as i32)) & (1i32))) != 0) {
                                l_pkt_14 = F::Vu1Pkt_CallProg1(c, l_part_10);
                                { let v_ = ((p_tex0_4) as i32); let a_ = ((l_pkt_14).wrapping_add(432)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                { let v_ = (((p_tex0_4).wrapping_shr((32i32) as u32)) as i32); let a_ = ((l_pkt_14).wrapping_add(432)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                { let v_ = 7i32; let a_ = (l_pkt_14).wrapping_add(440); c.m.w32(a_, (v_) as u32); };
                                { let v_ = 0i32; let a_ = (l_pkt_14).wrapping_add(444); c.m.w32(a_, (v_) as u32); };
                                F::Mtx_Copy(c, (l_pkt_14).wrapping_add(80), (l_node_11).wrapping_add(80));
                                F::Mtx_Copy(c, (l_pkt_14).wrapping_add(16), (l_node_11).wrapping_add(16));
                                F::Vec4_Copy(c, (l_pkt_14).wrapping_add(144), (l_part_10).wrapping_add(16));
                                F::Vec4_Copy(c, (l_pkt_14).wrapping_add(160), (l_part_10).wrapping_add(32));
                                { let v_ = l_k_12; let a_ = (l_pkt_14).wrapping_add(368); c.m.w32(a_, (v_).0); };
                                { let v_ = l_k_12; let a_ = ((l_pkt_14).wrapping_add(368)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                                { let v_ = l_k_12; let a_ = ((l_pkt_14).wrapping_add(368)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                                { let v_ = F32(c.m.r32((p_view_2).wrapping_add(64))); let a_ = ((l_pkt_14).wrapping_add(368)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                                { let v_ = l_k_12; let a_ = (l_pkt_14).wrapping_add(384); c.m.w32(a_, (v_).0); };
                                { let v_ = l_k_12; let a_ = ((l_pkt_14).wrapping_add(384)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                                { let v_ = l_k_12; let a_ = ((l_pkt_14).wrapping_add(384)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                                { let v_ = l_k_12; let a_ = ((l_pkt_14).wrapping_add(384)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                                F::Mtx_Copy(c, fp.wrapping_add(64), (l_node_11).wrapping_add(16));
                                { let v_ = F32(0x00000000); let a_ = ((fp.wrapping_add(64)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                                { let v_ = F32(0x00000000); let a_ = ((fp.wrapping_add(64)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                                { let v_ = F32(0x00000000); let a_ = ((fp.wrapping_add(64)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                                F::Mtx_Mul(c, fp.wrapping_add(128), fp, fp.wrapping_add(64));
                                F::Mtx_Copy(c, (l_pkt_14).wrapping_add(176), fp.wrapping_add(128));
                                { let v_ = F32(0x00000000); let a_ = (((l_pkt_14).wrapping_add(176)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                                { let v_ = F32(0x00000000); let a_ = (((l_pkt_14).wrapping_add(176)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                                { let v_ = F32(0x00000000); let a_ = (((l_pkt_14).wrapping_add(176)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                                F::Vu0Screen_StoreMtx(c, (l_pkt_14).wrapping_add(240));
                                F::Vu0Clip_StoreMtx(c, (l_pkt_14).wrapping_add(304));
                            }
                        } else {
                            if (((((p_mode_3) == (2i32)) as i32)) != 0) {
                                l_pkt_15 = F::Vu1Pkt_CallProg2(c, l_part_10);
                                F::Mtx_Copy(c, (l_pkt_15).wrapping_add(80), (l_node_11).wrapping_add(80));
                                F::Mtx_Copy(c, (l_pkt_15).wrapping_add(16), (l_node_11).wrapping_add(16));
                                F::Vec4_Copy(c, (l_pkt_15).wrapping_add(144), (l_part_10).wrapping_add(16));
                                F::Vec4_Copy(c, (l_pkt_15).wrapping_add(160), (l_part_10).wrapping_add(32));
                                F::Vec4_Copy(c, (l_pkt_15).wrapping_add(176), fp.wrapping_add(208));
                                { let v_ = l_k_12; let a_ = (((l_pkt_15).wrapping_add(176)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                            }
                        }
                    }
                }
            }
            if ((c.m.r16((l_part_10).wrapping_add(6))) != 0) {
                break 'l1;
            }
            l_part_10 = (l_part_10).wrapping_add((((c.m.r32(l_part_10) as i32)) as u32));
        }
        if (((((l_headNode_8) != (((0i32) as u32))) as i32)) != 0) {
            if ((((!((p_noMask_6) != 0)) as i32)) != 0) {
                F::ObjGs_AddFrameMaskOpaque(c, p_fbmsk_5);
            }
            l_pkt_16 = F::Vu1Pkt_CallProg0(c, (l_headPart_9).wrapping_add(96));
            F::Mtx_Copy(c, (l_pkt_16).wrapping_add(80), (l_headNode_8).wrapping_add(80));
            F::Mtx_Copy(c, (l_pkt_16).wrapping_add(16), (l_headNode_8).wrapping_add(16));
            F::Vec4_Copy(c, (l_pkt_16).wrapping_add(144), (l_headPart_9).wrapping_add(16));
            F::Vec4_Copy(c, (l_pkt_16).wrapping_add(160), (l_headPart_9).wrapping_add(32));
            F::Mtx_MakeNormalLight(c, (l_pkt_16).wrapping_add(176), (p_view_2).wrapping_add(16), sym::D_002EC2C0, sym::D_002EC2C0);
            F::Vec4_Copy(c, (l_pkt_16).wrapping_add(368), (p_view_2).wrapping_add(32));
            F::Vec4_Copy(c, (l_pkt_16).wrapping_add(384), sym::D_002C3440);
            F::Vu0Screen_StoreMtx(c, (l_pkt_16).wrapping_add(240));
            F::Vu0Clip_StoreMtx(c, (l_pkt_16).wrapping_add(304));
            if ((((!((p_noMask_6) != 0)) as i32)) != 0) {
                F::ObjGs_AddFrameMaskBlend(c, p_fbmsk_5);
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "ObjDraw_DrawParts", &PROBE_N, &[], true); }
    r_
}

pub fn ObjDraw_SetLight(c: &mut Ctx, mut p_obj_1: u32, mut p_view_2: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ObjDraw_SetLight", &PROBE_N, &[(p_obj_1) as u32, (p_view_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(112) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 112);
    let mut l_mask_3: u32 = 0;
    let mut l_flags_4: i32 = 0;
    let mut l_k_5: F32 = F32(0);
    let mut l_node_6: u32 = 0;
    let mut l_dst_7: u32 = 0;
    let mut l_node_8: u32 = 0;
    let mut l_dst_9: u32 = 0;
    let mut l_k_10: F32 = F32(0);
    let r_ = 'f: {
        l_mask_3 = 4278190080u32;
        if (((((((((rt::bf_get_u(c, (p_obj_1).wrapping_add(2624), 1, 1) as u32)) as i32)) != 0) && (((((rt::bf_get_u(c, (p_obj_1).wrapping_add(2624), 2, 1) as u32)) as i32)) != 0)) as i32)) != 0) {
            l_mask_3 = ((0i32) as u32);
        }
        l_flags_4 = (c.m.r32(p_view_2) as i32);
        if ((((l_flags_4) & (65536i32))) != 0) {
            F::BtlObjLight_GetDir(c, (p_view_2).wrapping_add(16));
            F::ObjGs_AddFrame1(c, l_mask_3, 0i32);
            F::ObjGs_AddFrame2Alpha(c, 4278190080u32, ((549755813986i64) as u64));
            F::ObjGs_AddZbufBoth(c, 224i32, 0i32);
        } else {
            if ((((l_flags_4) & (131072i32))) != 0) {
                { let v_ = { let a_ = (((c.m.r32(sym::gBtlCamView)).wrapping_add(64)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }; let a_ = (p_view_2).wrapping_add(16); c.m.w32(a_, (v_).0); };
                { let v_ = { let a_ = (((c.m.r32(sym::gBtlCamView)).wrapping_add(64)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }; let a_ = ((p_view_2).wrapping_add(16)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                { let v_ = { let a_ = (((c.m.r32(sym::gBtlCamView)).wrapping_add(64)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }; let a_ = ((p_view_2).wrapping_add(16)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                { let v_ = F32(0x3f800000); let a_ = ((p_view_2).wrapping_add(16)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                F::ObjGs_AddFrame1(c, l_mask_3, 0i32);
                F::ObjGs_AddFrame2Alpha(c, 4278190080u32, ((549755813986i64) as u64));
                F::ObjGs_AddZbufBoth(c, 224i32, 0i32);
            } else {
                if ((((l_flags_4) & (4194304i32))) != 0) {
                    if (((({ let a_ = (c.m.r32(((p_obj_1).wrapping_add(24)).wrapping_add(40))).wrapping_add(12); c.m.r32(a_) }) & (((16777216i32) as u32)))) != 0) {
                        { let v_ = { let a_ = (((c.m.r32(sym::gBtlCamView)).wrapping_add(64)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }; let a_ = (p_view_2).wrapping_add(16); c.m.w32(a_, (v_).0); };
                        { let v_ = { let a_ = (((c.m.r32(sym::gBtlCamView)).wrapping_add(64)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }; let a_ = ((p_view_2).wrapping_add(16)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                        { let v_ = { let a_ = (((c.m.r32(sym::gBtlCamView)).wrapping_add(64)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }; let a_ = ((p_view_2).wrapping_add(16)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                        { let v_ = F32(0x3f800000); let a_ = ((p_view_2).wrapping_add(16)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                        F::ObjGs_AddFrame1(c, l_mask_3, 0i32);
                    } else {
                        l_k_5 = F32(0x3f800000);
                        l_node_6 = F::BtlObj_GetNode(c, p_obj_1, 3i32);
                        { let a1_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(64); F::Mtx_InverseRT(c, fp.wrapping_add(32), a1_) };
                        l_dst_7 = (p_view_2).wrapping_add(16);
                        F::Vec4_Sub(c, fp, (fp.wrapping_add(32)).wrapping_add((((3i32) as u32)).wrapping_mul(16)), ((l_node_6).wrapping_add(16)).wrapping_add((((3i32) as u32)).wrapping_mul(16)));
                        F::Vec3_Normalize(c, fp, fp);
                        { let r_ = rt::fmul({ let a_ = (((c.m.r32(sym::gBtlCamView)).wrapping_add(64)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }, F32(0x3f333333)); let a_ = fp; let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                        { let v_ = { let a_ = (((c.m.r32(sym::gBtlCamView)).wrapping_add(64)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }; let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
                        { let r_ = rt::fmul({ let a_ = (((c.m.r32(sym::gBtlCamView)).wrapping_add(64)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }, F32(0x3f333333)); let a_ = (fp).wrapping_add(8); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                        { let v_ = l_k_5; let a_ = (fp).wrapping_add(12); c.m.w32(a_, (v_).0); };
                        F::Vec3_Normalize(c, fp, fp);
                        F::Vec3_Scale(c, fp, fp, l_k_5);
                        F::BtlObjLight_GetDir(c, fp.wrapping_add(16));
                        { let r_ = rt::fsub(F32(0x3f800000), l_k_5); let a_ = fp.wrapping_add(16); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                        { let r_ = rt::fsub(F32(0x3f800000), l_k_5); let a_ = (fp.wrapping_add(16)).wrapping_add(4); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                        { let r_ = rt::fsub(F32(0x3f800000), l_k_5); let a_ = (fp.wrapping_add(16)).wrapping_add(8); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                        { let v_ = l_k_5; let a_ = (fp.wrapping_add(16)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                        F::Vec4_Add(c, l_dst_7, fp, fp.wrapping_add(16));
                        F::Vec3_Normalize(c, l_dst_7, l_dst_7);
                        F::ObjGs_AddFrame1(c, l_mask_3, 0i32);
                    }
                    F::ObjGs_AddFrame2Alpha(c, 4278190080u32, ((549755813986i64) as u64));
                    F::ObjGs_AddZbufBoth(c, 224i32, 0i32);
                } else {
                    if ((((l_flags_4) & (524288i32))) != 0) {
                        { let v_ = rt::fsub(rt::fneg({ let a_ = (((c.m.r32(sym::gBtlCamView)).wrapping_add(64)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }), rt::fmul({ let a_ = (((c.m.r32(sym::gBtlCamView)).wrapping_add(64)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }, F32(0xbf333333))); let a_ = (p_view_2).wrapping_add(16); c.m.w32(a_, (v_).0); };
                        { let v_ = rt::fsub(rt::fneg({ let a_ = (((c.m.r32(sym::gBtlCamView)).wrapping_add(64)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }), rt::fmul({ let a_ = (((c.m.r32(sym::gBtlCamView)).wrapping_add(64)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }, F32(0xbf333333))); let a_ = ((p_view_2).wrapping_add(16)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                        { let v_ = rt::fsub(rt::fneg({ let a_ = (((c.m.r32(sym::gBtlCamView)).wrapping_add(64)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }), rt::fmul({ let a_ = (((c.m.r32(sym::gBtlCamView)).wrapping_add(64)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }, F32(0xbf333333))); let a_ = ((p_view_2).wrapping_add(16)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                        { let v_ = F32(0x3f800000); let a_ = ((p_view_2).wrapping_add(16)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                        F::Vec3_Normalize(c, (p_view_2).wrapping_add(16), (p_view_2).wrapping_add(16));
                        F::ObjGs_AddFrame1(c, l_mask_3, 0i32);
                        F::ObjGs_AddFrame2Alpha(c, 4278190080u32, ((549755813960i64) as u64));
                        F::ObjGs_AddZbufBoth(c, 224i32, 0i32);
                    } else {
                        if ((((l_flags_4) & (262144i32))) != 0) {
                            if ((((!((F::BtlObjFade_Get(c, p_obj_1, fp.wrapping_add(96))) != 0)) as i32)) != 0) {
                                { let v_ = F32(0x00000000); let a_ = fp.wrapping_add(96); c.m.w32(a_, (v_).0); };
                            }
                            l_node_8 = F::BtlObj_GetNode(c, p_obj_1, 3i32);
                            l_k_10 = F32(0x3f800000);
                            { let a1_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(64); F::Mtx_InverseRT(c, fp.wrapping_add(32), a1_) };
                            F::Vec4_Sub(c, fp, (fp.wrapping_add(32)).wrapping_add((((3i32) as u32)).wrapping_mul(16)), ((l_node_8).wrapping_add(16)).wrapping_add((((3i32) as u32)).wrapping_mul(16)));
                            l_dst_9 = (p_view_2).wrapping_add(16);
                            F::Vec3_Normalize(c, fp, fp);
                            { let r_ = rt::fmul({ let a_ = (((c.m.r32(sym::gBtlCamView)).wrapping_add(64)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }, F32(0x3f333333)); let a_ = fp; let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                            { let v_ = { let a_ = (((c.m.r32(sym::gBtlCamView)).wrapping_add(64)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }; let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
                            { let r_ = rt::fmul({ let a_ = (((c.m.r32(sym::gBtlCamView)).wrapping_add(64)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }, F32(0x3f333333)); let a_ = (fp).wrapping_add(8); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                            { let v_ = l_k_10; let a_ = (fp).wrapping_add(12); c.m.w32(a_, (v_).0); };
                            F::Vec3_Normalize(c, fp, fp);
                            { let a2_ = F32(c.m.r32(fp.wrapping_add(96))); F::Vec3_Scale(c, fp, fp, a2_) };
                            F::BtlObjLight_GetDir(c, fp.wrapping_add(16));
                            { let r_ = rt::fsub(l_k_10, F32(c.m.r32(fp.wrapping_add(96)))); let a_ = fp.wrapping_add(16); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                            { let r_ = rt::fsub(l_k_10, F32(c.m.r32(fp.wrapping_add(96)))); let a_ = (fp.wrapping_add(16)).wrapping_add(4); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                            { let r_ = rt::fsub(l_k_10, F32(c.m.r32(fp.wrapping_add(96)))); let a_ = (fp.wrapping_add(16)).wrapping_add(8); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                            { let v_ = l_k_10; let a_ = (fp.wrapping_add(16)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                            F::Vec4_Add(c, l_dst_9, fp, fp.wrapping_add(16));
                            F::Vec3_Normalize(c, l_dst_9, l_dst_9);
                            F::ObjGs_AddFrame1(c, l_mask_3, 0i32);
                            F::ObjGs_AddFrame2Alpha(c, 4278190080u32, ((549755813986i64) as u64));
                            F::ObjGs_AddZbufBoth(c, 224i32, 0i32);
                        } else {
                            if ((((l_flags_4) & (1048576i32))) != 0) {
                                F::BtlObjLight_GetDir(c, (p_view_2).wrapping_add(16));
                                { let r_ = ((16777215i32) as u32); let n_ = ((l_mask_3) | (r_)); l_mask_3 = n_; };
                                F::ObjGs_AddFrame1(c, ((l_mask_3) | (((16777215i32) as u32))), 0i32);
                                F::ObjGs_AddFrame2Alpha(c, 4294967295u32, ((549755813986i64) as u64));
                                F::ObjGs_AddZbufBoth(c, 224i32, 0i32);
                            } else {
                                F::BtlObjLight_GetDir(c, (p_view_2).wrapping_add(16));
                                F::ObjGs_AddFrame1(c, ((16777215i32) as u32), 1i32);
                                F::ObjGs_AddFrame2Alpha(c, 4294967295u32, ((549755813986i64) as u64));
                                F::ObjGs_AddZbufBoth(c, 224i32, 1i32);
                            }
                        }
                    }
                }
            }
        }
        break 'f l_mask_3;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "ObjDraw_SetLight", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObjDraw_DrawModel(c: &mut Ctx, mut p_obj_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObjDraw_DrawModel", &PROBE_N, &[(p_obj_1) as u32], false); }
    let mut l_state_2: u32 = 0;
    let mut l_view_3: u32 = 0;
    let mut l_mask_4: u32 = 0;
    let r_ = 'f: {
    l_state_2 = (p_obj_1).wrapping_add(2624);
    l_view_3 = c.m.r32(((p_obj_1).wrapping_add(2624)).wrapping_add(176));
    F::ObjDraw_UploadTextures(c, p_obj_1);
    F::BtlObjDraw_SetEnv(c);
    F::ObjGs_AddTexEnv(c);
    { let a3_ = ((((c.m.r32(l_view_3) as i32)).wrapping_shr((21i32) as u32)) & (1i32)); F::ObjDraw_AddClutPass(c, p_obj_1, l_state_2, l_view_3, a3_) };
    F::BtlObjDraw_SetEnv(c);
    F::ObjGs_AddTexEnv(c);
    l_mask_4 = F::ObjDraw_SetLight(c, p_obj_1, l_view_3);
    { let _ = F::Vu1Pkt_LoadProg0(c); };
    F::ObjSeam_Transform(c, p_obj_1, l_view_3);
    { let a5_ = ((((c.m.r32(l_view_3) as i32)).wrapping_shr((21i32) as u32)) & (1i32)); F::ObjDraw_DrawParts(c, p_obj_1, l_view_3, 0i32, ((0i32) as u64), l_mask_4, a5_) };
    F::ObjSeam_Draw(c, p_obj_1, l_state_2, l_view_3);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObjDraw_DrawModel", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObjDraw_DrawModelFade(c: &mut Ctx, mut p_obj_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObjDraw_DrawModelFade", &PROBE_N, &[(p_obj_1) as u32], false); }
    let mut l_mdl_2: u32 = 0;
    let mut l_view_3: u32 = 0;
    let mut l_pkt_4: u32 = 0;
    let mut l_tex0_5: u64 = 0;
    let mut l_e_6: u32 = 0;
    let r_ = 'f: {
    l_mdl_2 = (p_obj_1).wrapping_add(24);
    l_view_3 = c.m.r32(((p_obj_1).wrapping_add(2624)).wrapping_add(176));
    { let a0_ = c.m.r32((l_mdl_2).wrapping_add(48)); let a1_ = (({ let a_ = (c.m.r32(((p_obj_1).wrapping_add(24)).wrapping_add(40))).wrapping_add(77); c.m.r8(a_) }) as i32); let a2_ = { let a_ = (c.m.r32((l_mdl_2).wrapping_add(40))).wrapping_add(100); (c.m.r32(a_) as i32) }; let a3_ = { let a_ = (c.m.r32((l_mdl_2).wrapping_add(40))).wrapping_add(104); (c.m.r32(a_) as i32) }; F::TexFile_UploadOne(c, a0_, a1_, a2_, a3_) };
    F::Dma_AddTexFlush(c);
    { let a0_ = c.m.r32((l_mdl_2).wrapping_add(48)); let a1_ = (({ let a_ = (c.m.r32((l_mdl_2).wrapping_add(40))).wrapping_add(82); c.m.r8(a_) }) as i32); let a2_ = (({ let a_ = (c.m.r32((l_mdl_2).wrapping_add(40))).wrapping_add(83); c.m.r8(a_) }) as i32); let a3_ = { let a_ = (c.m.r32((l_mdl_2).wrapping_add(40))).wrapping_add(88); (c.m.r32(a_) as i32) }; let a4_ = { let a_ = (c.m.r32((l_mdl_2).wrapping_add(40))).wrapping_add(92); (c.m.r32(a_) as i32) }; F::TexFile_UploadRange(c, a0_, a1_, a2_, a3_, a4_) };
    F::Dma_AddTexFlush(c);
    {
        l_e_6 = ({ let a_ = (c.m.r32((l_mdl_2).wrapping_add(48))).wrapping_add(16); c.m.r32(a_) }).wrapping_add(((({ let a_ = (c.m.r32((l_mdl_2).wrapping_add(40))).wrapping_add(77); c.m.r8(a_) }) as u32)).wrapping_mul(64));
        l_tex0_5 = c.m.r64((l_e_6).wrapping_add(48));
        { let r_ = ((((({ let a_ = (c.m.r32((l_mdl_2).wrapping_add(40))).wrapping_add(104); (c.m.r32(a_) as i32) }) as u32)) as u64)).wrapping_shl((37i32) as u32); let n_ = ((l_tex0_5) | (r_)); l_tex0_5 = n_; };
        { let r_ = (((({ let a_ = (c.m.r32((l_mdl_2).wrapping_add(40))).wrapping_add(100); (c.m.r32(a_) as i32) }) as u32)) as u64); let n_ = ((l_tex0_5) | (r_)); l_tex0_5 = n_; };
        { let r_ = ((51539607552i64) as u64); let n_ = ((l_tex0_5) | (r_)); l_tex0_5 = n_; };
    }
    F::ObjGs_AddTexEnv(c);
    F::ObjGs_AddFadeEnv(c);
    F::Dma_AddZbuf(c, 224i32, 1i32);
    l_pkt_4 = F::Vu1Pkt_LoadProg1(c);
    F::Vu0Screen_StoreMtx(c, (l_pkt_4).wrapping_add(240));
    F::Vu0Clip_StoreMtx(c, (l_pkt_4).wrapping_add(304));
    { let a1_ = rt::f2i(F32(c.m.r32((l_view_3).wrapping_add(64)))); F::ObjGs_AddFadeTex(c, l_tex0_5, a1_) };
    { let a5_ = ((((c.m.r32(l_view_3) as i32)).wrapping_shr((21i32) as u32)) & (1i32)); F::ObjDraw_DrawParts(c, p_obj_1, l_view_3, 1i32, l_tex0_5, ((0i32) as u32), a5_) };
    F::Dma_AddZbuf(c, 224i32, 0i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObjDraw_DrawModelFade", &PROBE_N, &[], true); }
    r_
}

pub fn ObjDraw_DrawPartsFlat(c: &mut Ctx, mut p_obj_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ObjDraw_DrawPartsFlat", &PROBE_N, &[(p_obj_1) as u32], false); }
    let r_ = 'f: {
    { let a1_ = c.m.r32(((p_obj_1).wrapping_add(2624)).wrapping_add(176)); F::ObjDraw_DrawParts(c, p_obj_1, a1_, 2i32, ((0i32) as u64), ((0i32) as u32), 0i32) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ObjDraw_DrawPartsFlat", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObjDraw_End(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObjDraw_End", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::BtlObjDraw_SetEnv(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObjDraw_End", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObjDraw_SetEnv(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObjDraw_SetEnv", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(352) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 352);
    let r_ = 'f: {
        rt::memset(c, fp, 0, 352);
        { let v_ = ((((268435456i32) | (21i32))) as u32); c.m.w32(fp.wrapping_add(0), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(4), v_); }
        { let v_ = ((268435456i32) as u32); c.m.w32(fp.wrapping_add(8), v_); }
        { let v_ = ((((1342177280i32) | (21i32))) as u32); c.m.w32(fp.wrapping_add(12), v_); }
        { let v_ = ((((32768i32) | (20i32))) as u32); c.m.w32(fp.wrapping_add(16), v_); }
        { let v_ = ((268435456i32) as u32); c.m.w32(fp.wrapping_add(20), v_); }
        { let v_ = ((14i32) as u32); c.m.w32(fp.wrapping_add(24), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(28), v_); }
        { let v_ = (((if ((((!(((((c.m.r32((sym::gGfx).wrapping_add(564)) as i32)) & (1i32))) != 0)) as i32)) != 0) { ((((((((112i32) as u64)) | ((((8i32) as u64)).wrapping_shl((16i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((32i32) as u32))) } else { ((((((((0i32) as u64)) | ((((8i32) as u64)).wrapping_shl((16i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((32i32) as u32))) })) as u32); c.m.w32(fp.wrapping_add(32), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(36), v_); }
        { let v_ = ((76i32) as u32); c.m.w32(fp.wrapping_add(40), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(44), v_); }
        { let v_ = (((if ((((!(((((c.m.r32((sym::gGfx).wrapping_add(564)) as i32)) & (1i32))) != 0)) as i32)) != 0) { ((((((((112i32) as u64)) | ((((8i32) as u64)).wrapping_shl((16i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((32i32) as u32))) } else { ((((((((0i32) as u64)) | ((((8i32) as u64)).wrapping_shl((16i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((32i32) as u32))) })) as u32); c.m.w32(fp.wrapping_add(48), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(52), v_); }
        { let v_ = ((77i32) as u32); c.m.w32(fp.wrapping_add(56), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(60), v_); }
        { let v_ = ((((((((224i32) as u64)) | ((((49i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((32i32) as u32)))) as u32); c.m.w32(fp.wrapping_add(64), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(68), v_); }
        { let v_ = ((78i32) as u32); c.m.w32(fp.wrapping_add(72), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(76), v_); }
        { let v_ = ((((((((224i32) as u64)) | ((((49i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((32i32) as u32)))) as u32); c.m.w32(fp.wrapping_add(80), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(84), v_); }
        { let v_ = ((79i32) as u32); c.m.w32(fp.wrapping_add(88), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(92), v_); }
        { let v_ = ((28672i32) as u32); c.m.w32(fp.wrapping_add(96), v_); }
        { let v_ = ((29184i32) as u32); c.m.w32(fp.wrapping_add(100), v_); }
        { let v_ = ((24i32) as u32); c.m.w32(fp.wrapping_add(104), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(108), v_); }
        { let v_ = ((28672i32) as u32); c.m.w32(fp.wrapping_add(112), v_); }
        { let v_ = ((29184i32) as u32); c.m.w32(fp.wrapping_add(116), v_); }
        { let v_ = ((25i32) as u32); c.m.w32(fp.wrapping_add(120), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(124), v_); }
        { let v_ = (((((((((({ let a_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(512); (c.m.r32(a_) as i32) }) as u64)) | (((({ let a_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(516); (c.m.r32(a_) as i32) }) as u64)).wrapping_shl((16i32) as u32)))) | (((({ let a_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(520); (c.m.r32(a_) as i32) }) as u64)).wrapping_shl((32i32) as u32)))) | (((({ let a_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(524); (c.m.r32(a_) as i32) }) as u64)).wrapping_shl((48i32) as u32)))) as u32); c.m.w32(fp.wrapping_add(128), v_); }
        { let v_ = ((((((((((({ let a_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(512); (c.m.r32(a_) as i32) }) as u64)) | (((({ let a_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(516); (c.m.r32(a_) as i32) }) as u64)).wrapping_shl((16i32) as u32)))) | (((({ let a_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(520); (c.m.r32(a_) as i32) }) as u64)).wrapping_shl((32i32) as u32)))) | (((({ let a_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(524); (c.m.r32(a_) as i32) }) as u64)).wrapping_shl((48i32) as u32)))).wrapping_shr((32i32) as u32)) as u32); c.m.w32(fp.wrapping_add(132), v_); }
        { let v_ = ((64i32) as u32); c.m.w32(fp.wrapping_add(136), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(140), v_); }
        { let v_ = (((((((((({ let a_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(512); (c.m.r32(a_) as i32) }) as u64)) | (((({ let a_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(516); (c.m.r32(a_) as i32) }) as u64)).wrapping_shl((16i32) as u32)))) | (((({ let a_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(520); (c.m.r32(a_) as i32) }) as u64)).wrapping_shl((32i32) as u32)))) | (((({ let a_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(524); (c.m.r32(a_) as i32) }) as u64)).wrapping_shl((48i32) as u32)))) as u32); c.m.w32(fp.wrapping_add(144), v_); }
        { let v_ = ((((((((((({ let a_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(512); (c.m.r32(a_) as i32) }) as u64)) | (((({ let a_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(516); (c.m.r32(a_) as i32) }) as u64)).wrapping_shl((16i32) as u32)))) | (((({ let a_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(520); (c.m.r32(a_) as i32) }) as u64)).wrapping_shl((32i32) as u32)))) | (((({ let a_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(524); (c.m.r32(a_) as i32) }) as u64)).wrapping_shl((48i32) as u32)))).wrapping_shr((32i32) as u32)) as u32); c.m.w32(fp.wrapping_add(148), v_); }
        { let v_ = ((65i32) as u32); c.m.w32(fp.wrapping_add(152), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(156), v_); }
        { let v_ = ((68i32) as u32); c.m.w32(fp.wrapping_add(160), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(164), v_); }
        { let v_ = ((66i32) as u32); c.m.w32(fp.wrapping_add(168), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(172), v_); }
        { let v_ = ((68i32) as u32); c.m.w32(fp.wrapping_add(176), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(180), v_); }
        { let v_ = ((67i32) as u32); c.m.w32(fp.wrapping_add(184), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(188), v_); }
        { let v_ = ((327680i32) as u32); c.m.w32(fp.wrapping_add(192), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(196), v_); }
        { let v_ = ((71i32) as u32); c.m.w32(fp.wrapping_add(200), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(204), v_); }
        { let v_ = ((327680i32) as u32); c.m.w32(fp.wrapping_add(208), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(212), v_); }
        { let v_ = ((72i32) as u32); c.m.w32(fp.wrapping_add(216), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(220), v_); }
        { let v_ = ((96i32) as u32); c.m.w32(fp.wrapping_add(224), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(228), v_); }
        { let v_ = ((20i32) as u32); c.m.w32(fp.wrapping_add(232), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(236), v_); }
        { let v_ = ((96i32) as u32); c.m.w32(fp.wrapping_add(240), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(244), v_); }
        { let v_ = ((21i32) as u32); c.m.w32(fp.wrapping_add(248), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(252), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(256), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(260), v_); }
        { let v_ = ((74i32) as u32); c.m.w32(fp.wrapping_add(264), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(268), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(272), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(276), v_); }
        { let v_ = ((75i32) as u32); c.m.w32(fp.wrapping_add(280), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(284), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(288), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(292), v_); }
        { let v_ = ((8i32) as u32); c.m.w32(fp.wrapping_add(296), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(300), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(304), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(308), v_); }
        { let v_ = ((9i32) as u32); c.m.w32(fp.wrapping_add(312), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(316), v_); }
        { let v_ = ((1i32) as u32); c.m.w32(fp.wrapping_add(320), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(324), v_); }
        { let v_ = ((70i32) as u32); c.m.w32(fp.wrapping_add(328), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(332), v_); }
        { let v_ = ((1i32) as u32); c.m.w32(fp.wrapping_add(336), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(340), v_); }
        { let v_ = ((26i32) as u32); c.m.w32(fp.wrapping_add(344), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(348), v_); }
        F::Dma_AddData(c, fp, ((352u32) as i32));
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlObjDraw_SetEnv", &PROBE_N, &[], true); }
    r_
}

pub fn ObjGs_AddTexEnv(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ObjGs_AddTexEnv", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(192) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 192);
    let r_ = 'f: {
        rt::memset(c, fp, 0, 192);
        { let v_ = ((((268435456i32) | (11i32))) as u32); c.m.w32(fp.wrapping_add(0), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(4), v_); }
        { let v_ = ((268435456i32) as u32); c.m.w32(fp.wrapping_add(8), v_); }
        { let v_ = ((((1342177280i32) | (11i32))) as u32); c.m.w32(fp.wrapping_add(12), v_); }
        { let v_ = ((((32768i32) | (10i32))) as u32); c.m.w32(fp.wrapping_add(16), v_); }
        { let v_ = ((268435456i32) as u32); c.m.w32(fp.wrapping_add(20), v_); }
        { let v_ = ((14i32) as u32); c.m.w32(fp.wrapping_add(24), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(28), v_); }
        { let v_ = ((100i32) as u32); c.m.w32(fp.wrapping_add(32), v_); }
        { let v_ = ((128i32) as u32); c.m.w32(fp.wrapping_add(36), v_); }
        { let v_ = ((66i32) as u32); c.m.w32(fp.wrapping_add(40), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(44), v_); }
        { let v_ = ((327680i32) as u32); c.m.w32(fp.wrapping_add(48), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(52), v_); }
        { let v_ = ((71i32) as u32); c.m.w32(fp.wrapping_add(56), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(60), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(64), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(68), v_); }
        { let v_ = ((74i32) as u32); c.m.w32(fp.wrapping_add(72), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(76), v_); }
        { let v_ = ((96i32) as u32); c.m.w32(fp.wrapping_add(80), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(84), v_); }
        { let v_ = ((20i32) as u32); c.m.w32(fp.wrapping_add(88), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(92), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(96), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(100), v_); }
        { let v_ = ((8i32) as u32); c.m.w32(fp.wrapping_add(104), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(108), v_); }
        { let v_ = ((327680i32) as u32); c.m.w32(fp.wrapping_add(112), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(116), v_); }
        { let v_ = ((72i32) as u32); c.m.w32(fp.wrapping_add(120), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(124), v_); }
        { let v_ = ((96i32) as u32); c.m.w32(fp.wrapping_add(128), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(132), v_); }
        { let v_ = ((21i32) as u32); c.m.w32(fp.wrapping_add(136), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(140), v_); }
        { let v_ = ((5i32) as u32); c.m.w32(fp.wrapping_add(144), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(148), v_); }
        { let v_ = ((9i32) as u32); c.m.w32(fp.wrapping_add(152), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(156), v_); }
        { let v_ = ((1i32) as u32); c.m.w32(fp.wrapping_add(160), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(164), v_); }
        { let v_ = ((70i32) as u32); c.m.w32(fp.wrapping_add(168), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(172), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(176), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(180), v_); }
        { let v_ = ((75i32) as u32); c.m.w32(fp.wrapping_add(184), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(188), v_); }
        F::Dma_AddData(c, fp, ((192u32) as i32));
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "ObjGs_AddTexEnv", &PROBE_N, &[], true); }
    r_
}

pub fn ObjGs_AddFrame1(c: &mut Ctx, mut p_fbmsk_1: u32, mut p_unused_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ObjGs_AddFrame1", &PROBE_N, &[(p_fbmsk_1) as u32, (p_unused_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(96) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 96);
    let r_ = 'f: {
        rt::memset(c, fp, 0, 96);
        { let v_ = ((((268435456i32) | (5i32))) as u32); c.m.w32(fp.wrapping_add(0), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(4), v_); }
        { let v_ = ((268435456i32) as u32); c.m.w32(fp.wrapping_add(8), v_); }
        { let v_ = ((((1342177280i32) | (5i32))) as u32); c.m.w32(fp.wrapping_add(12), v_); }
        { let v_ = ((((32768i32) | (4i32))) as u32); c.m.w32(fp.wrapping_add(16), v_); }
        { let v_ = ((268435456i32) as u32); c.m.w32(fp.wrapping_add(20), v_); }
        { let v_ = ((14i32) as u32); c.m.w32(fp.wrapping_add(24), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(28), v_); }
        { let v_ = (((((((((((if (((((c.m.r32((sym::gGfx).wrapping_add(564)) as i32)) & (1i32))) != 0) { 0i32 } else { 112i32 })) as u64)) | ((((8i32) as u64)).wrapping_shl((16i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((p_fbmsk_1) as u64)).wrapping_shl((32i32) as u32)))) as u32); c.m.w32(fp.wrapping_add(32), v_); }
        { let v_ = ((((((((((((if (((((c.m.r32((sym::gGfx).wrapping_add(564)) as i32)) & (1i32))) != 0) { 0i32 } else { 112i32 })) as u64)) | ((((8i32) as u64)).wrapping_shl((16i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((p_fbmsk_1) as u64)).wrapping_shl((32i32) as u32)))).wrapping_shr((32i32) as u32)) as u32); c.m.w32(fp.wrapping_add(36), v_); }
        { let v_ = ((76i32) as u32); c.m.w32(fp.wrapping_add(40), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(44), v_); }
        { let v_ = ((100i32) as u32); c.m.w32(fp.wrapping_add(48), v_); }
        { let v_ = ((128i32) as u32); c.m.w32(fp.wrapping_add(52), v_); }
        { let v_ = ((66i32) as u32); c.m.w32(fp.wrapping_add(56), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(60), v_); }
        { let v_ = ((327680i32) as u32); c.m.w32(fp.wrapping_add(64), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(68), v_); }
        { let v_ = ((71i32) as u32); c.m.w32(fp.wrapping_add(72), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(76), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(80), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(84), v_); }
        { let v_ = ((74i32) as u32); c.m.w32(fp.wrapping_add(88), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(92), v_); }
        F::Dma_AddData(c, fp, ((96u32) as i32));
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "ObjGs_AddFrame1", &PROBE_N, &[], true); }
    r_
}

pub fn ObjGs_AddFrame2Alpha(c: &mut Ctx, mut p_fbmsk_1: u32, mut p_alpha_2: u64) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ObjGs_AddFrame2Alpha", &PROBE_N, &[(p_fbmsk_1) as u32, (p_alpha_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(64) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 64);
    let r_ = 'f: {
        rt::memset(c, fp, 0, 64);
        { let v_ = ((((268435456i32) | (3i32))) as u32); c.m.w32(fp.wrapping_add(0), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(4), v_); }
        { let v_ = ((268435456i32) as u32); c.m.w32(fp.wrapping_add(8), v_); }
        { let v_ = ((((1342177280i32) | (3i32))) as u32); c.m.w32(fp.wrapping_add(12), v_); }
        { let v_ = ((((32768i32) | (2i32))) as u32); c.m.w32(fp.wrapping_add(16), v_); }
        { let v_ = ((268435456i32) as u32); c.m.w32(fp.wrapping_add(20), v_); }
        { let v_ = ((14i32) as u32); c.m.w32(fp.wrapping_add(24), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(28), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(32), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(36), v_); }
        { let v_ = ((67i32) as u32); c.m.w32(fp.wrapping_add(40), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(44), v_); }
        { let v_ = (((((((((((if (((((c.m.r32((sym::gGfx).wrapping_add(564)) as i32)) & (1i32))) != 0) { 0i32 } else { 112i32 })) as u64)) | ((((8i32) as u64)).wrapping_shl((16i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((p_fbmsk_1) as u64)).wrapping_shl((32i32) as u32)))) as u32); c.m.w32(fp.wrapping_add(48), v_); }
        { let v_ = ((((((((((((if (((((c.m.r32((sym::gGfx).wrapping_add(564)) as i32)) & (1i32))) != 0) { 0i32 } else { 112i32 })) as u64)) | ((((8i32) as u64)).wrapping_shl((16i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((p_fbmsk_1) as u64)).wrapping_shl((32i32) as u32)))).wrapping_shr((32i32) as u32)) as u32); c.m.w32(fp.wrapping_add(52), v_); }
        { let v_ = ((77i32) as u32); c.m.w32(fp.wrapping_add(56), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(60), v_); }
        { let v_ = ((p_alpha_2) as u32); let a_ = (fp).wrapping_add((((8i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = (((p_alpha_2).wrapping_shr((32i32) as u32)) as u32); let a_ = (fp).wrapping_add((((9i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        F::Dma_AddData(c, fp, ((64u32) as i32));
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "ObjGs_AddFrame2Alpha", &PROBE_N, &[], true); }
    r_
}

pub fn ObjGs_AddFadeEnv(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ObjGs_AddFadeEnv", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(336) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 336);
    let r_ = 'f: {
        rt::memset(c, fp, 0, 336);
        { let v_ = ((((268435456i32) | (20i32))) as u32); c.m.w32(fp.wrapping_add(0), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(4), v_); }
        { let v_ = ((268435456i32) as u32); c.m.w32(fp.wrapping_add(8), v_); }
        { let v_ = ((((1342177280i32) | (20i32))) as u32); c.m.w32(fp.wrapping_add(12), v_); }
        { let v_ = ((((32768i32) | (19i32))) as u32); c.m.w32(fp.wrapping_add(16), v_); }
        { let v_ = ((268435456i32) as u32); c.m.w32(fp.wrapping_add(20), v_); }
        { let v_ = ((14i32) as u32); c.m.w32(fp.wrapping_add(24), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(28), v_); }
        { let v_ = (((if ((((!(((((c.m.r32((sym::gGfx).wrapping_add(564)) as i32)) & (1i32))) != 0)) as i32)) != 0) { ((((((((112i32) as u64)) | ((((8i32) as u64)).wrapping_shl((16i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((32i32) as u32))) } else { ((((((((0i32) as u64)) | ((((8i32) as u64)).wrapping_shl((16i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((32i32) as u32))) })) as u32); c.m.w32(fp.wrapping_add(32), v_); }
        { let v_ = ((16777215i32) as u32); c.m.w32(fp.wrapping_add(36), v_); }
        { let v_ = ((76i32) as u32); c.m.w32(fp.wrapping_add(40), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(44), v_); }
        { let v_ = (((if ((((!(((((c.m.r32((sym::gGfx).wrapping_add(564)) as i32)) & (1i32))) != 0)) as i32)) != 0) { ((((((((112i32) as u64)) | ((((8i32) as u64)).wrapping_shl((16i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((32i32) as u32))) } else { ((((((((0i32) as u64)) | ((((8i32) as u64)).wrapping_shl((16i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((32i32) as u32))) })) as u32); c.m.w32(fp.wrapping_add(48), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(52), v_); }
        { let v_ = ((77i32) as u32); c.m.w32(fp.wrapping_add(56), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(60), v_); }
        { let v_ = ((((((((224i32) as u64)) | ((((49i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((32i32) as u32)))) as u32); c.m.w32(fp.wrapping_add(64), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(68), v_); }
        { let v_ = ((78i32) as u32); c.m.w32(fp.wrapping_add(72), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(76), v_); }
        { let v_ = ((((((((224i32) as u64)) | ((((49i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((32i32) as u32)))) as u32); c.m.w32(fp.wrapping_add(80), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(84), v_); }
        { let v_ = ((79i32) as u32); c.m.w32(fp.wrapping_add(88), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(92), v_); }
        { let v_ = ((28672i32) as u32); c.m.w32(fp.wrapping_add(96), v_); }
        { let v_ = ((29184i32) as u32); c.m.w32(fp.wrapping_add(100), v_); }
        { let v_ = ((24i32) as u32); c.m.w32(fp.wrapping_add(104), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(108), v_); }
        { let v_ = ((28672i32) as u32); c.m.w32(fp.wrapping_add(112), v_); }
        { let v_ = ((29184i32) as u32); c.m.w32(fp.wrapping_add(116), v_); }
        { let v_ = ((25i32) as u32); c.m.w32(fp.wrapping_add(120), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(124), v_); }
        { let v_ = (((((((((({ let a_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(512); (c.m.r32(a_) as i32) }) as u64)) | (((({ let a_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(516); (c.m.r32(a_) as i32) }) as u64)).wrapping_shl((16i32) as u32)))) | (((({ let a_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(520); (c.m.r32(a_) as i32) }) as u64)).wrapping_shl((32i32) as u32)))) | (((({ let a_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(524); (c.m.r32(a_) as i32) }) as u64)).wrapping_shl((48i32) as u32)))) as u32); c.m.w32(fp.wrapping_add(128), v_); }
        { let v_ = ((((((((((({ let a_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(512); (c.m.r32(a_) as i32) }) as u64)) | (((({ let a_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(516); (c.m.r32(a_) as i32) }) as u64)).wrapping_shl((16i32) as u32)))) | (((({ let a_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(520); (c.m.r32(a_) as i32) }) as u64)).wrapping_shl((32i32) as u32)))) | (((({ let a_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(524); (c.m.r32(a_) as i32) }) as u64)).wrapping_shl((48i32) as u32)))).wrapping_shr((32i32) as u32)) as u32); c.m.w32(fp.wrapping_add(132), v_); }
        { let v_ = ((64i32) as u32); c.m.w32(fp.wrapping_add(136), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(140), v_); }
        { let v_ = (((((((((({ let a_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(512); (c.m.r32(a_) as i32) }) as u64)) | (((({ let a_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(516); (c.m.r32(a_) as i32) }) as u64)).wrapping_shl((16i32) as u32)))) | (((({ let a_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(520); (c.m.r32(a_) as i32) }) as u64)).wrapping_shl((32i32) as u32)))) | (((({ let a_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(524); (c.m.r32(a_) as i32) }) as u64)).wrapping_shl((48i32) as u32)))) as u32); c.m.w32(fp.wrapping_add(144), v_); }
        { let v_ = ((((((((((({ let a_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(512); (c.m.r32(a_) as i32) }) as u64)) | (((({ let a_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(516); (c.m.r32(a_) as i32) }) as u64)).wrapping_shl((16i32) as u32)))) | (((({ let a_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(520); (c.m.r32(a_) as i32) }) as u64)).wrapping_shl((32i32) as u32)))) | (((({ let a_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(524); (c.m.r32(a_) as i32) }) as u64)).wrapping_shl((48i32) as u32)))).wrapping_shr((32i32) as u32)) as u32); c.m.w32(fp.wrapping_add(148), v_); }
        { let v_ = ((65i32) as u32); c.m.w32(fp.wrapping_add(152), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(156), v_); }
        { let v_ = ((68i32) as u32); c.m.w32(fp.wrapping_add(160), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(164), v_); }
        { let v_ = ((66i32) as u32); c.m.w32(fp.wrapping_add(168), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(172), v_); }
        { let v_ = ((88i32) as u32); c.m.w32(fp.wrapping_add(176), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(180), v_); }
        { let v_ = ((67i32) as u32); c.m.w32(fp.wrapping_add(184), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(188), v_); }
        { let v_ = ((327680i32) as u32); c.m.w32(fp.wrapping_add(192), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(196), v_); }
        { let v_ = ((71i32) as u32); c.m.w32(fp.wrapping_add(200), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(204), v_); }
        { let v_ = ((327680i32) as u32); c.m.w32(fp.wrapping_add(208), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(212), v_); }
        { let v_ = ((72i32) as u32); c.m.w32(fp.wrapping_add(216), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(220), v_); }
        { let v_ = ((96i32) as u32); c.m.w32(fp.wrapping_add(224), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(228), v_); }
        { let v_ = ((20i32) as u32); c.m.w32(fp.wrapping_add(232), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(236), v_); }
        { let v_ = ((96i32) as u32); c.m.w32(fp.wrapping_add(240), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(244), v_); }
        { let v_ = ((21i32) as u32); c.m.w32(fp.wrapping_add(248), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(252), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(256), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(260), v_); }
        { let v_ = ((74i32) as u32); c.m.w32(fp.wrapping_add(264), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(268), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(272), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(276), v_); }
        { let v_ = ((75i32) as u32); c.m.w32(fp.wrapping_add(280), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(284), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(288), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(292), v_); }
        { let v_ = ((8i32) as u32); c.m.w32(fp.wrapping_add(296), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(300), v_); }
        { let v_ = ((5i32) as u32); c.m.w32(fp.wrapping_add(304), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(308), v_); }
        { let v_ = ((9i32) as u32); c.m.w32(fp.wrapping_add(312), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(316), v_); }
        { let v_ = ((1i32) as u32); c.m.w32(fp.wrapping_add(320), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(324), v_); }
        { let v_ = ((70i32) as u32); c.m.w32(fp.wrapping_add(328), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(332), v_); }
        F::Dma_AddData(c, fp, ((336u32) as i32));
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "ObjGs_AddFadeEnv", &PROBE_N, &[], true); }
    r_
}

pub fn ObjGs_AddFadeTex(c: &mut Ctx, mut p_tex0_1: u64, mut p_aref_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ObjGs_AddFadeTex", &PROBE_N, &[(p_tex0_1) as u32, (p_aref_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(80) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 80);
    let r_ = 'f: {
        rt::memset(c, fp, 0, 80);
        { let v_ = ((((268435456i32) | (4i32))) as u32); c.m.w32(fp.wrapping_add(0), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(4), v_); }
        { let v_ = ((268435456i32) as u32); c.m.w32(fp.wrapping_add(8), v_); }
        { let v_ = ((((1342177280i32) | (4i32))) as u32); c.m.w32(fp.wrapping_add(12), v_); }
        { let v_ = ((((32768i32) | (3i32))) as u32); c.m.w32(fp.wrapping_add(16), v_); }
        { let v_ = ((268435456i32) as u32); c.m.w32(fp.wrapping_add(20), v_); }
        { let v_ = ((14i32) as u32); c.m.w32(fp.wrapping_add(24), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(28), v_); }
        { let v_ = (((((((p_aref_2) as u64)).wrapping_shl((4i32) as u32)) | (((327684i32) as u64)))) as u32); c.m.w32(fp.wrapping_add(32), v_); }
        { let v_ = ((((((((p_aref_2) as u64)).wrapping_shl((4i32) as u32)) | (((327684i32) as u64)))).wrapping_shr((32i32) as u32)) as u32); c.m.w32(fp.wrapping_add(36), v_); }
        { let v_ = ((71i32) as u32); c.m.w32(fp.wrapping_add(40), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(44), v_); }
        { let v_ = ((p_tex0_1) as u32); c.m.w32(fp.wrapping_add(48), v_); }
        { let v_ = (((p_tex0_1).wrapping_shr((32i32) as u32)) as u32); c.m.w32(fp.wrapping_add(52), v_); }
        { let v_ = ((7i32) as u32); c.m.w32(fp.wrapping_add(56), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(60), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(64), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(68), v_); }
        { let v_ = ((63i32) as u32); c.m.w32(fp.wrapping_add(72), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(76), v_); }
        F::Dma_AddData(c, fp, ((80u32) as i32));
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "ObjGs_AddFadeTex", &PROBE_N, &[], true); }
    r_
}

pub fn ObjGs_AddZbufBoth(c: &mut Ctx, mut p_zbp_1: i32, mut p_zmsk_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ObjGs_AddZbufBoth", &PROBE_N, &[(p_zbp_1) as u32, (p_zmsk_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(64) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 64);
    let r_ = 'f: {
        rt::memset(c, fp, 0, 64);
        { let v_ = ((((268435456i32) | (3i32))) as u32); c.m.w32(fp.wrapping_add(0), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(4), v_); }
        { let v_ = ((268435456i32) as u32); c.m.w32(fp.wrapping_add(8), v_); }
        { let v_ = ((((1342177280i32) | (3i32))) as u32); c.m.w32(fp.wrapping_add(12), v_); }
        { let v_ = ((((32768i32) | (2i32))) as u32); c.m.w32(fp.wrapping_add(16), v_); }
        { let v_ = ((268435456i32) as u32); c.m.w32(fp.wrapping_add(20), v_); }
        { let v_ = ((14i32) as u32); c.m.w32(fp.wrapping_add(24), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(28), v_); }
        { let v_ = ((((((((p_zbp_1) as u64)) | ((((49i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((p_zmsk_2) as u64)).wrapping_shl((32i32) as u32)))) as u32); c.m.w32(fp.wrapping_add(32), v_); }
        { let v_ = (((((((((p_zbp_1) as u64)) | ((((49i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((p_zmsk_2) as u64)).wrapping_shl((32i32) as u32)))).wrapping_shr((32i32) as u32)) as u32); c.m.w32(fp.wrapping_add(36), v_); }
        { let v_ = ((78i32) as u32); c.m.w32(fp.wrapping_add(40), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(44), v_); }
        { let v_ = ((((((((p_zbp_1) as u64)) | ((((49i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((p_zmsk_2) as u64)).wrapping_shl((32i32) as u32)))) as u32); c.m.w32(fp.wrapping_add(48), v_); }
        { let v_ = (((((((((p_zbp_1) as u64)) | ((((49i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((p_zmsk_2) as u64)).wrapping_shl((32i32) as u32)))).wrapping_shr((32i32) as u32)) as u32); c.m.w32(fp.wrapping_add(52), v_); }
        { let v_ = ((79i32) as u32); c.m.w32(fp.wrapping_add(56), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(60), v_); }
        F::Dma_AddData(c, fp, ((64u32) as i32));
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "ObjGs_AddZbufBoth", &PROBE_N, &[], true); }
    r_
}
