#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn BtlMenu_BeginDraw(c: &mut Ctx, mut p_pkt_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMenu_BeginDraw", &PROBE_N, &[(p_pkt_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = F::Dma_BeginDirect(c); let a_ = p_pkt_1; c.m.w32(a_, v_); };
    F::Gfx_PutDefaultEnv(c, p_pkt_1);
    { let v_ = ((1152921504606879746i64) as u64); let a_ = (c.m.r32(p_pkt_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((14i32) as u64); let a_ = (c.m.r32(p_pkt_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let a_ = p_pkt_1; let n_ = (c.m.r32(a_)).wrapping_add((((r_) as u32)).wrapping_mul(8)); c.m.w32(a_, n_); };
    { let v_ = ((0i32) as u64); let a_ = (c.m.r32(p_pkt_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((71i32) as u64); let a_ = (c.m.r32(p_pkt_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let a_ = p_pkt_1; let n_ = (c.m.r32(a_)).wrapping_add((((r_) as u32)).wrapping_mul(8)); c.m.w32(a_, n_); };
    { let v_ = ((5117051104i64) as u64); let a_ = (c.m.r32(p_pkt_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((78i32) as u64); let a_ = (c.m.r32(p_pkt_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let a_ = p_pkt_1; let n_ = (c.m.r32(a_)).wrapping_add((((r_) as u32)).wrapping_mul(8)); c.m.w32(a_, n_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMenu_BeginDraw", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMenu_EndDraw(c: &mut Ctx, mut p_pkt_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMenu_EndDraw", &PROBE_N, &[(p_pkt_1) as u32], false); }
    let r_ = 'f: {
    { let a0_ = c.m.r32(p_pkt_1); F::Dma_EndDirect(c, a0_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMenu_EndDraw", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMenu_PutSprite(c: &mut Ctx, mut p_pkt_1: u32, mut p_x0_2: i32, mut p_y0_3: i32, mut p_x1_4: i32, mut p_y1_5: i32, mut p_u0_6: i32, mut p_v0_7: i32, mut p_u1_8: i32, mut p_v1_9: i32, mut p_color_10: u32, mut p_tex_11: i32, mut p_clut_12: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMenu_PutSprite", &PROBE_N, &[(p_pkt_1) as u32, (p_x0_2) as u32, (p_y0_3) as u32, (p_x1_4) as u32, (p_y1_5) as u32, (p_u0_6) as u32, (p_v0_7) as u32, (p_u1_8) as u32, (p_v1_9) as u32, (p_color_10) as u32, (p_tex_11) as u32, (p_clut_12) as u32], false); }
    let r_ = 'f: {
    { let r_ = 1792i32; let n_ = (p_x0_2).wrapping_add(r_); p_x0_2 = n_; };
    { let r_ = 1824i32; let n_ = (p_y0_3).wrapping_add(r_); p_y0_3 = n_; };
    { let r_ = 1792i32; let n_ = (p_x1_4).wrapping_add(r_); p_x1_4 = n_; };
    { let r_ = 1824i32; let n_ = (p_y1_5).wrapping_add(r_); p_y1_5 = n_; };
    { let v_ = 9511602413006520321u64; let a_ = (c.m.r32(p_pkt_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((1397954288i32) as u64); let a_ = (c.m.r32(p_pkt_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let a_ = p_pkt_1; let n_ = (c.m.r32(a_)).wrapping_add((((r_) as u32)).wrapping_mul(8)); c.m.w32(a_, n_); };
    { let v_ = ((342i32) as u64); let a_ = (c.m.r32(p_pkt_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((0i32) as u64); let a_ = (c.m.r32(p_pkt_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let a_ = p_pkt_1; let n_ = (c.m.r32(a_)).wrapping_add((((r_) as u32)).wrapping_mul(8)); c.m.w32(a_, n_); };
    { let v_ = F::BtlMenu_GetTex0(c, p_tex_11, p_clut_12); let a_ = (c.m.r32(p_pkt_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((((p_color_10) as u64)) | (((4575657221408423936i64) as u64))); let a_ = (c.m.r32(p_pkt_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let a_ = p_pkt_1; let n_ = (c.m.r32(a_)).wrapping_add((((r_) as u32)).wrapping_mul(8)); c.m.w32(a_, n_); };
    { let v_ = (((((((p_u0_6) as i64)).wrapping_shl((4i32) as u32)) | ((((p_v0_7) as i64)).wrapping_shl((20i32) as u32)))) as u64); let a_ = (c.m.r32(p_pkt_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = (((((((((p_x0_2) as i64)).wrapping_shl((4i32) as u32)) | ((((p_y0_3) as i64)).wrapping_shl((20i32) as u32)))) as u64)) | (18446744004990074880u64)); let a_ = (c.m.r32(p_pkt_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let a_ = p_pkt_1; let n_ = (c.m.r32(a_)).wrapping_add((((r_) as u32)).wrapping_mul(8)); c.m.w32(a_, n_); };
    { let v_ = (((((((p_u1_8) as i64)).wrapping_shl((4i32) as u32)) | ((((p_v1_9) as i64)).wrapping_shl((20i32) as u32)))) as u64); let a_ = (c.m.r32(p_pkt_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = (((((((((p_x1_4) as i64)).wrapping_shl((4i32) as u32)) | ((((p_y1_5) as i64)).wrapping_shl((20i32) as u32)))) as u64)) | (18446744004990074880u64)); let a_ = (c.m.r32(p_pkt_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let a_ = p_pkt_1; let n_ = (c.m.r32(a_)).wrapping_add((((r_) as u32)).wrapping_mul(8)); c.m.w32(a_, n_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMenu_PutSprite", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMenu_PutMirrored(c: &mut Ctx, mut p_pkt_1: u32, mut p_x0_2: i32, mut p_x1_3: i32, mut p_y0_4: i32, mut p_y1_5: i32, mut p_color_6: u32, mut p_tex_7: i32, mut p_clut_8: i32, mut p_halves_9: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMenu_PutMirrored", &PROBE_N, &[(p_pkt_1) as u32, (p_x0_2) as u32, (p_x1_3) as u32, (p_y0_4) as u32, (p_y1_5) as u32, (p_color_6) as u32, (p_tex_7) as u32, (p_clut_8) as u32, (p_halves_9) as u32], false); }
    let mut l_hw_10: i32 = 0;
    let mut l_hh_11: i32 = 0;
    let mut l_cx_12: i32 = 0;
    let mut l_cy_13: i32 = 0;
    let mut l_w_14: i32 = 0;
    let mut l_h_15: i32 = 0;
    let r_ = 'f: {
    l_hw_10 = rt::div_i32((p_x1_3).wrapping_sub(p_x0_2), 2i32);
    l_hh_11 = rt::div_i32((p_y1_5).wrapping_sub(p_y0_4), 2i32);
    l_cx_12 = (p_x0_2).wrapping_add(l_hw_10);
    l_cy_13 = (p_y0_4).wrapping_add(l_hh_11);
    { let r_ = 1792i32; let n_ = (l_cx_12).wrapping_add(r_); l_cx_12 = n_; };
    { let r_ = 1824i32; let n_ = (l_cy_13).wrapping_add(r_); l_cy_13 = n_; };
    { let v_ = ((4899916394579132417i64) as u64); let a_ = (c.m.r32(p_pkt_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((5872i32) as u64); let a_ = (c.m.r32(p_pkt_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let a_ = p_pkt_1; let n_ = (c.m.r32(a_)).wrapping_add((((r_) as u32)).wrapping_mul(8)); c.m.w32(a_, n_); };
    { let v_ = ((342i32) as u64); let a_ = (c.m.r32(p_pkt_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((0i32) as u64); let a_ = (c.m.r32(p_pkt_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let a_ = p_pkt_1; let n_ = (c.m.r32(a_)).wrapping_add((((r_) as u32)).wrapping_mul(8)); c.m.w32(a_, n_); };
    { let v_ = F::BtlMenu_GetTex0(c, p_tex_7, p_clut_8); let a_ = (c.m.r32(p_pkt_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((((p_color_6) as u64)) | (((4575657221408423936i64) as u64))); let a_ = (c.m.r32(p_pkt_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let a_ = p_pkt_1; let n_ = (c.m.r32(a_)).wrapping_add((((r_) as u32)).wrapping_mul(8)); c.m.w32(a_, n_); };
    if (((((p_halves_9) == (1i32)) as i32)) != 0) {
        { let v_ = ((4899916394579132418i64) as u64); let a_ = (c.m.r32(p_pkt_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((21331i32) as u64); let a_ = (c.m.r32(p_pkt_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let a_ = p_pkt_1; let n_ = (c.m.r32(a_)).wrapping_add((((r_) as u32)).wrapping_mul(8)); c.m.w32(a_, n_); };
        l_w_14 = (l_hw_10).wrapping_mul(2i32);
        l_h_15 = (l_hh_11).wrapping_mul(2i32);
        { let v_ = ((0i32) as u64); let a_ = (c.m.r32(p_pkt_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((((((((((l_cx_12).wrapping_sub(l_hw_10)) as i64)).wrapping_shl((4i32) as u32)) | (((((l_cy_13).wrapping_sub(l_hh_11)) as i64)).wrapping_shl((20i32) as u32)))) as u64)) | (18446744004990074880u64)); let a_ = (c.m.r32(p_pkt_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let a_ = p_pkt_1; let n_ = (c.m.r32(a_)).wrapping_add((((r_) as u32)).wrapping_mul(8)); c.m.w32(a_, n_); };
        { let v_ = (((((((l_w_14) as i64)).wrapping_shl((4i32) as u32)) | ((((l_h_15) as i64)).wrapping_shl((20i32) as u32)))) as u64); let a_ = (c.m.r32(p_pkt_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = (((((((((l_cx_12) as i64)).wrapping_shl((4i32) as u32)) | (((((l_cy_13).wrapping_add(l_hh_11)) as i64)).wrapping_shl((20i32) as u32)))) as u64)) | (18446744004990074880u64)); let a_ = (c.m.r32(p_pkt_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let a_ = p_pkt_1; let n_ = (c.m.r32(a_)).wrapping_add((((r_) as u32)).wrapping_mul(8)); c.m.w32(a_, n_); };
        { let v_ = ((0i32) as u64); let a_ = (c.m.r32(p_pkt_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((((((((((l_cx_12).wrapping_add(l_hw_10)) as i64)).wrapping_shl((4i32) as u32)) | (((((l_cy_13).wrapping_sub(l_hh_11)) as i64)).wrapping_shl((20i32) as u32)))) as u64)) | (18446744004990074880u64)); let a_ = (c.m.r32(p_pkt_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let a_ = p_pkt_1; let n_ = (c.m.r32(a_)).wrapping_add((((r_) as u32)).wrapping_mul(8)); c.m.w32(a_, n_); };
        { let v_ = (((((((l_w_14) as i64)).wrapping_shl((4i32) as u32)) | ((((l_h_15) as i64)).wrapping_shl((20i32) as u32)))) as u64); let a_ = (c.m.r32(p_pkt_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = (((((((((l_cx_12) as i64)).wrapping_shl((4i32) as u32)) | (((((l_cy_13).wrapping_add(l_hh_11)) as i64)).wrapping_shl((20i32) as u32)))) as u64)) | (18446744004990074880u64)); let a_ = (c.m.r32(p_pkt_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let a_ = p_pkt_1; let n_ = (c.m.r32(a_)).wrapping_add((((r_) as u32)).wrapping_mul(8)); c.m.w32(a_, n_); };
    } else {
        { let v_ = ((4899916394579132420i64) as u64); let a_ = (c.m.r32(p_pkt_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((21331i32) as u64); let a_ = (c.m.r32(p_pkt_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let a_ = p_pkt_1; let n_ = (c.m.r32(a_)).wrapping_add((((r_) as u32)).wrapping_mul(8)); c.m.w32(a_, n_); };
        l_w_14 = l_hw_10;
        l_h_15 = l_hh_11;
        { let v_ = ((0i32) as u64); let a_ = (c.m.r32(p_pkt_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((((((((((l_cx_12).wrapping_sub(l_w_14)) as i64)).wrapping_shl((4i32) as u32)) | (((((l_cy_13).wrapping_sub(l_h_15)) as i64)).wrapping_shl((20i32) as u32)))) as u64)) | (18446744004990074880u64)); let a_ = (c.m.r32(p_pkt_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let a_ = p_pkt_1; let n_ = (c.m.r32(a_)).wrapping_add((((r_) as u32)).wrapping_mul(8)); c.m.w32(a_, n_); };
        { let v_ = (((((((l_w_14) as i64)).wrapping_shl((4i32) as u32)) | ((((l_h_15) as i64)).wrapping_shl((20i32) as u32)))) as u64); let a_ = (c.m.r32(p_pkt_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = (((((((((l_cx_12) as i64)).wrapping_shl((4i32) as u32)) | ((((l_cy_13) as i64)).wrapping_shl((20i32) as u32)))) as u64)) | (18446744004990074880u64)); let a_ = (c.m.r32(p_pkt_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let a_ = p_pkt_1; let n_ = (c.m.r32(a_)).wrapping_add((((r_) as u32)).wrapping_mul(8)); c.m.w32(a_, n_); };
        { let v_ = ((0i32) as u64); let a_ = (c.m.r32(p_pkt_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((((((((((l_cx_12).wrapping_add(l_w_14)) as i64)).wrapping_shl((4i32) as u32)) | (((((l_cy_13).wrapping_sub(l_h_15)) as i64)).wrapping_shl((20i32) as u32)))) as u64)) | (18446744004990074880u64)); let a_ = (c.m.r32(p_pkt_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let a_ = p_pkt_1; let n_ = (c.m.r32(a_)).wrapping_add((((r_) as u32)).wrapping_mul(8)); c.m.w32(a_, n_); };
        { let v_ = (((((((l_w_14) as i64)).wrapping_shl((4i32) as u32)) | ((((l_h_15) as i64)).wrapping_shl((20i32) as u32)))) as u64); let a_ = (c.m.r32(p_pkt_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = (((((((((l_cx_12) as i64)).wrapping_shl((4i32) as u32)) | ((((l_cy_13) as i64)).wrapping_shl((20i32) as u32)))) as u64)) | (18446744004990074880u64)); let a_ = (c.m.r32(p_pkt_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let a_ = p_pkt_1; let n_ = (c.m.r32(a_)).wrapping_add((((r_) as u32)).wrapping_mul(8)); c.m.w32(a_, n_); };
        { let v_ = ((0i32) as u64); let a_ = (c.m.r32(p_pkt_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((((((((((l_cx_12).wrapping_sub(l_w_14)) as i64)).wrapping_shl((4i32) as u32)) | (((((l_cy_13).wrapping_add(l_h_15)) as i64)).wrapping_shl((20i32) as u32)))) as u64)) | (18446744004990074880u64)); let a_ = (c.m.r32(p_pkt_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let a_ = p_pkt_1; let n_ = (c.m.r32(a_)).wrapping_add((((r_) as u32)).wrapping_mul(8)); c.m.w32(a_, n_); };
        { let v_ = (((((((l_w_14) as i64)).wrapping_shl((4i32) as u32)) | ((((l_h_15) as i64)).wrapping_shl((20i32) as u32)))) as u64); let a_ = (c.m.r32(p_pkt_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = (((((((((l_cx_12) as i64)).wrapping_shl((4i32) as u32)) | ((((l_cy_13) as i64)).wrapping_shl((20i32) as u32)))) as u64)) | (18446744004990074880u64)); let a_ = (c.m.r32(p_pkt_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let a_ = p_pkt_1; let n_ = (c.m.r32(a_)).wrapping_add((((r_) as u32)).wrapping_mul(8)); c.m.w32(a_, n_); };
        { let v_ = ((0i32) as u64); let a_ = (c.m.r32(p_pkt_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((((((((((l_cx_12).wrapping_add(l_w_14)) as i64)).wrapping_shl((4i32) as u32)) | (((((l_cy_13).wrapping_add(l_h_15)) as i64)).wrapping_shl((20i32) as u32)))) as u64)) | (18446744004990074880u64)); let a_ = (c.m.r32(p_pkt_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let a_ = p_pkt_1; let n_ = (c.m.r32(a_)).wrapping_add((((r_) as u32)).wrapping_mul(8)); c.m.w32(a_, n_); };
        { let v_ = (((((((l_w_14) as i64)).wrapping_shl((4i32) as u32)) | ((((l_h_15) as i64)).wrapping_shl((20i32) as u32)))) as u64); let a_ = (c.m.r32(p_pkt_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = (((((((((l_cx_12) as i64)).wrapping_shl((4i32) as u32)) | ((((l_cy_13) as i64)).wrapping_shl((20i32) as u32)))) as u64)) | (18446744004990074880u64)); let a_ = (c.m.r32(p_pkt_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let a_ = p_pkt_1; let n_ = (c.m.r32(a_)).wrapping_add((((r_) as u32)).wrapping_mul(8)); c.m.w32(a_, n_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMenu_PutMirrored", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMenu_GetTex0(c: &mut Ctx, mut p_tex_1: i32, mut p_clut_2: i32) -> u64 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMenu_GetTex0", &PROBE_N, &[(p_tex_1) as u32, (p_clut_2) as u32], false); }
    let mut l_c_3: u32 = 0;
    let mut l_t_4: u32 = 0;
    let mut l_cbp_5: i32 = 0;
    let mut l_tbp_6: i32 = 0;
    let mut l_tex0_7: u64 = 0;
    let r_ = 'f: {
    l_c_3 = ({ let a_ = ({ let a_ = c.m.r32(sym::gBtlMenu); c.m.r32(a_) }).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((p_clut_2) as u32)).wrapping_mul(64));
    l_t_4 = ({ let a_ = ({ let a_ = c.m.r32(sym::gBtlMenu); c.m.r32(a_) }).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((p_tex_1) as u32)).wrapping_mul(64));
    l_cbp_5 = (c.m.r32((l_c_3).wrapping_add(36)) as i32);
    l_tbp_6 = (c.m.r32((l_t_4).wrapping_add(32)) as i32);
    l_tex0_7 = c.m.r64((l_t_4).wrapping_add(48));
    { let r_ = ((((l_cbp_5).wrapping_add(10752i32)) as u64)).wrapping_shl((37i32) as u32); let n_ = ((l_tex0_7) | (r_)); l_tex0_7 = n_; };
    { let r_ = (((l_tbp_6).wrapping_add(10816i32)) as u64); let n_ = ((l_tex0_7) | (r_)); l_tex0_7 = n_; };
    { let r_ = ((17179869184i64) as u64); let n_ = ((l_tex0_7) | (r_)); l_tex0_7 = n_; };
    break 'f l_tex0_7;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMenu_GetTex0", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMenu_UploadTex(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMenu_UploadTex", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let a0_ = { let a_ = c.m.r32(sym::gBtlMenu); c.m.r32(a_) }; F::TexFile_UploadAll(c, a0_, 10816i32, 10752i32) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMenu_UploadTex", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMenu_GetDefaultPadType(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMenu_GetDefaultPadType", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMenu_GetDefaultPadType", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMenu_IsSkillList(c: &mut Ctx, mut p_menu_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMenu_IsSkillList", &PROBE_N, &[(p_menu_1) as u32], false); }
    let r_ = 'f: {
    break 'f ((({ let a_ = (c.m.r32(p_menu_1)).wrapping_add(20); (c.m.r32(a_) as i32) }) == (3i32)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMenu_IsSkillList", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMenu_IsControls(c: &mut Ctx, mut p_menu_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMenu_IsControls", &PROBE_N, &[(p_menu_1) as u32], false); }
    let r_ = 'f: {
    break 'f ((({ let a_ = (c.m.r32(p_menu_1)).wrapping_add(20); (c.m.r32(a_) as i32) }) == (4i32)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMenu_IsControls", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMenu_GetText(c: &mut Ctx, mut p_n_1: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMenu_GetText", &PROBE_N, &[(p_n_1) as u32], false); }
    let mut l_file_2: u32 = 0;
    let r_ = 'f: {
    l_file_2 = { let a_ = (c.m.r32(sym::gCommonRes)).wrapping_add(((40i32) as u32)); c.m.r32(a_) };
    break 'f (l_file_2).wrapping_add((rt::div_u32(c.m.r32((l_file_2).wrapping_add((((p_n_1) as u32)).wrapping_mul(4))), ((4i32) as u32))).wrapping_mul(4));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMenu_GetText", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMenu_GetItemText(c: &mut Ctx, mut p_n_1: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMenu_GetItemText", &PROBE_N, &[(p_n_1) as u32], false); }
    let r_ = 'f: {
    break 'f F::BtlMenu_GetText(c, (p_n_1).wrapping_add(8i32));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMenu_GetItemText", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMenu_GetControlsText(c: &mut Ctx, mut p_padType_1: i32, mut p_config_2: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMenu_GetControlsText", &PROBE_N, &[(p_padType_1) as u32, (p_config_2) as u32], false); }
    let mut l_base_3: i32 = 0;
    let r_ = 'f: {
    if (((((p_padType_1) == (0i32)) as i32)) != 0) {
        l_base_3 = 38i32;
    } else {
        if (((((p_padType_1) == (1i32)) as i32)) != 0) {
            l_base_3 = 78i32;
        } else {
            if (((((p_padType_1) == (2i32)) as i32)) != 0) {
                l_base_3 = 118i32;
            } else {
                l_base_3 = 158i32;
            }
        }
    }
    break 'f F::BtlMenu_GetText(c, (l_base_3).wrapping_add(p_config_2));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMenu_GetControlsText", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMenu_FindItem(c: &mut Ctx, mut p_menu_1: u32, mut p_id_2: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMenu_FindItem", &PROBE_N, &[(p_menu_1) as u32, (p_id_2) as u32], false); }
    let mut l_found_3: u32 = 0;
    let mut l_i_4: i32 = 0;
    let r_ = 'f: {
    l_found_3 = ((0i32) as u32);
    {
        l_i_4 = 0i32;
        'l1: loop {
            if !(((((l_i_4) < ((c.m.r32((p_menu_1).wrapping_add(4)) as i32))) as i32)) != 0) { break 'l1; }
            'c2: {
                if (((((p_id_2) == ({ let a_ = ((c.m.r32(p_menu_1)).wrapping_add((((l_i_4) as u32)).wrapping_mul(48))).wrapping_add(4); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                    l_found_3 = (c.m.r32(p_menu_1)).wrapping_add((((l_i_4) as u32)).wrapping_mul(48));
                    break 'l1;
                }
                if ((((({ let a_ = ((c.m.r32(p_menu_1)).wrapping_add((((l_i_4) as u32)).wrapping_mul(48))).wrapping_add(8); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
                    l_found_3 = { let a0_ = { let a_ = ((c.m.r32(p_menu_1)).wrapping_add((((l_i_4) as u32)).wrapping_mul(48))).wrapping_add(8); c.m.r32(a_) }; F::BtlMenu_FindItem(c, a0_, p_id_2) };
                    if (((((l_found_3) != (((0i32) as u32))) as i32)) != 0) {
                        break 'l1;
                    }
                }
            }
            { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
        }
    }
    break 'f l_found_3;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMenu_FindItem", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMenu_FindOwner(c: &mut Ctx, mut p_menu_1: u32, mut p_id_2: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMenu_FindOwner", &PROBE_N, &[(p_menu_1) as u32, (p_id_2) as u32], false); }
    let mut l_found_3: u32 = 0;
    let mut l_i_4: i32 = 0;
    let r_ = 'f: {
    l_found_3 = ((0i32) as u32);
    {
        l_i_4 = 0i32;
        'l1: loop {
            if !(((((l_i_4) < ((c.m.r32((p_menu_1).wrapping_add(4)) as i32))) as i32)) != 0) { break 'l1; }
            'c2: {
                if (((((p_id_2) == ({ let a_ = ((c.m.r32(p_menu_1)).wrapping_add((((l_i_4) as u32)).wrapping_mul(48))).wrapping_add(4); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                    l_found_3 = p_menu_1;
                    break 'l1;
                }
                if ((((({ let a_ = ((c.m.r32(p_menu_1)).wrapping_add((((l_i_4) as u32)).wrapping_mul(48))).wrapping_add(8); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
                    l_found_3 = { let a0_ = { let a_ = ((c.m.r32(p_menu_1)).wrapping_add((((l_i_4) as u32)).wrapping_mul(48))).wrapping_add(8); c.m.r32(a_) }; F::BtlMenu_FindOwner(c, a0_, p_id_2) };
                    if (((((l_found_3) != (((0i32) as u32))) as i32)) != 0) {
                        break 'l1;
                    }
                }
            }
            { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
        }
    }
    break 'f l_found_3;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMenu_FindOwner", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMenu_GetWidth(c: &mut Ctx, mut p_menu_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMenu_GetWidth", &PROBE_N, &[(p_menu_1) as u32], false); }
    let mut l_pad_2: i32 = 0;
    let mut l_w_3: i32 = 0;
    let mut l_i_4: i32 = 0;
    let mut l_n_5: i32 = 0;
    let r_ = 'f: {
    l_pad_2 = { let a_ = (c.m.r32(sym::gBtlMenu)).wrapping_add(12); (c.m.r32(a_) as i32) };
    l_w_3 = 0i32;
    if (((((p_menu_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((F::BtlMenu_IsSkillList(c, p_menu_1)) != 0) {
        l_w_3 = (F::Font_GetGlyphHeight(c)).wrapping_mul(20i32);
    } else {
        if ((F::BtlMenu_IsControls(c, p_menu_1)) != 0) {
            l_w_3 = (F::Font_GetGlyphHeight(c)).wrapping_mul(22i32);
        } else {
            {
                l_i_4 = 0i32;
                'l1: loop {
                    if !(((((l_i_4) < ((c.m.r32((p_menu_1).wrapping_add(4)) as i32))) as i32)) != 0) { break 'l1; }
                    'c2: {
                        l_n_5 = { let a0_ = { let a0_ = (({ let a_ = ((c.m.r32(p_menu_1)).wrapping_add((((l_i_4) as u32)).wrapping_mul(48))).wrapping_add((((l_pad_2) as u32)).wrapping_mul(2)); (c.m.r16(a_) as i16) }) as i32); F::BtlMenu_GetItemText(c, a0_) }; F::Font_GetWidth(c, a0_) };
                        if (((((l_w_3) < (l_n_5)) as i32)) != 0) {
                            l_w_3 = l_n_5;
                        }
                    }
                    { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
                }
            }
            if (((((l_w_3) < (128i32)) as i32)) != 0) {
                l_w_3 = 128i32;
            }
        }
    }
    break 'f (rt::f2i(rt::fmul(rt::i2f(l_w_3), F32(c.m.r32((p_menu_1).wrapping_add(8)))))).wrapping_add(((((c.m.r16((p_menu_1).wrapping_add(12)) as i16)) as i32)).wrapping_mul(2i32));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMenu_GetWidth", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMenu_GetHeight(c: &mut Ctx, mut p_menu_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMenu_GetHeight", &PROBE_N, &[(p_menu_1) as u32], false); }
    let mut l_h_2: i32 = 0;
    let r_ = 'f: {
    if (((((p_menu_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((F::BtlMenu_IsSkillList(c, p_menu_1)) != 0) {
        l_h_2 = (F::Font_GetGlyphHeight(c)).wrapping_mul(18i32);
    } else {
        if ((F::BtlMenu_IsControls(c, p_menu_1)) != 0) {
            l_h_2 = (F::Font_GetGlyphHeight(c)).wrapping_mul(15i32);
        } else {
            l_h_2 = ((c.m.r32((p_menu_1).wrapping_add(4)) as i32)).wrapping_mul(28i32);
        }
    }
    break 'f (rt::f2i(rt::fmul(rt::i2f(l_h_2), F32(c.m.r32((p_menu_1).wrapping_add(8)))))).wrapping_add(((((c.m.r16((p_menu_1).wrapping_add(14)) as i16)) as i32)).wrapping_mul(2i32));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMenu_GetHeight", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMenu_HasOpenSub(c: &mut Ctx, mut p_menu_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMenu_HasOpenSub", &PROBE_N, &[(p_menu_1) as u32], false); }
    let mut l_found_2: i32 = 0;
    let mut l_i_3: i32 = 0;
    let r_ = 'f: {
    l_found_2 = 0i32;
    {
        l_i_3 = 0i32;
        'l1: loop {
            if !(((((l_i_3) < ((c.m.r32((p_menu_1).wrapping_add(4)) as i32))) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((((((((({ let a_ = ((c.m.r32(p_menu_1)).wrapping_add((((l_i_3) as u32)).wrapping_mul(48))).wrapping_add(8); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) && ((((((c.m.r32((p_menu_1).wrapping_add(20)) as i32)) == (l_i_3)) as i32)) != 0)) as i32)) != 0) {
                    l_found_2 = 1i32;
                    break 'l1;
                }
            }
            { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
        }
    }
    break 'f l_found_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMenu_HasOpenSub", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMenu_DrawPart(c: &mut Ctx, mut p_pkt_1: u32, mut p_x_2: i32, mut p_y_3: i32, mut p_w_4: i32, mut p_h_5: i32, mut p_tex_6: i32, mut p_part_7: i32, mut p_centered_8: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMenu_DrawPart", &PROBE_N, &[(p_pkt_1) as u32, (p_x_2) as u32, (p_y_3) as u32, (p_w_4) as u32, (p_h_5) as u32, (p_tex_6) as u32, (p_part_7) as u32, (p_centered_8) as u32], false); }
    let mut l_x0_9: i32 = 0;
    let mut l_y0_10: i32 = 0;
    let mut l_x1_11: i32 = 0;
    let mut l_y1_12: i32 = 0;
    let r_ = 'f: {
    if ((((!((p_centered_8) != 0)) as i32)) != 0) {
        l_x0_9 = p_x_2;
        l_y0_10 = p_y_3;
        l_x1_11 = (l_x0_9).wrapping_add(p_w_4);
        l_y1_12 = (l_y0_10).wrapping_add(p_h_5);
    } else {
        l_x0_9 = (p_x_2).wrapping_sub((p_w_4).wrapping_shr((1i32) as u32));
        l_y0_10 = (p_y_3).wrapping_sub((p_h_5).wrapping_shr((1i32) as u32));
        l_x1_11 = (p_x_2).wrapping_add((p_w_4).wrapping_shr((1i32) as u32));
        l_y1_12 = (p_y_3).wrapping_add((p_h_5).wrapping_shr((1i32) as u32));
    }
    F::BtlMenu_PutSprite(c, p_pkt_1, l_x0_9, l_y0_10, l_x1_11, l_y1_12, 0i32, (p_h_5).wrapping_mul(p_part_7), p_w_4, ((p_part_7).wrapping_add(1i32)).wrapping_mul(p_h_5), 2155905152u32, p_tex_6, p_tex_6);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMenu_DrawPart", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMenu_DrawFrame(c: &mut Ctx, mut p_pkt_1: u32, mut p_x0_2: i32, mut p_x1_3: i32, mut p_y0_4: i32, mut p_y1_5: i32, mut p_kind_6: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMenu_DrawFrame", &PROBE_N, &[(p_pkt_1) as u32, (p_x0_2) as u32, (p_x1_3) as u32, (p_y0_4) as u32, (p_y1_5) as u32, (p_kind_6) as u32], false); }
    let r_ = 'f: {
    F::BtlMenu_PutMirrored(c, p_pkt_1, p_x0_2, p_x1_3, p_y0_4, p_y1_5, 2155905152u32, (p_kind_6).wrapping_add(1i32), (p_kind_6).wrapping_add(1i32), 0i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMenu_DrawFrame", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMenu_DrawPageBar(c: &mut Ctx, mut p_pkt_1: u32, mut p_x0_2: i32, mut p_x1_3: i32, mut p_y0_4: i32, mut p_y1_5: i32, mut p_page_6: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMenu_DrawPageBar", &PROBE_N, &[(p_pkt_1) as u32, (p_x0_2) as u32, (p_x1_3) as u32, (p_y0_4) as u32, (p_y1_5) as u32, (p_page_6) as u32], false); }
    let r_ = 'f: {
    F::BtlMenu_PutMirrored(c, p_pkt_1, (p_x0_2).wrapping_add(4i32), (p_x1_3).wrapping_sub(3i32), p_y0_4, p_y1_5, 2155905152u32, 4i32, (p_page_6).wrapping_add(38i32), 0i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMenu_DrawPageBar", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMenu_DrawPageTab(c: &mut Ctx, mut p_pkt_1: u32, mut p_x_2: i32, mut p_y_3: i32, mut p_page_4: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMenu_DrawPageTab", &PROBE_N, &[(p_pkt_1) as u32, (p_x_2) as u32, (p_y_3) as u32, (p_page_4) as u32], false); }
    let mut l_w_5: i32 = 0;
    let mut l_ofs_6: i32 = 0;
    let r_ = 'f: {
    l_w_5 = 50i32;
    l_ofs_6 = ((p_page_4).wrapping_mul(l_w_5)).wrapping_add(20i32);
    { let r_ = l_ofs_6; let n_ = (p_x_2).wrapping_add(r_); p_x_2 = n_; };
    F::BtlMenu_PutMirrored(c, p_pkt_1, p_x_2, (p_x_2).wrapping_add(l_w_5), (p_y_3).wrapping_add(6i32), (p_y_3).wrapping_add(25i32), 2155905152u32, 3i32, (p_page_4).wrapping_add(38i32), 1i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMenu_DrawPageTab", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMenu_Draw(c: &mut Ctx, mut p_pkt_1: u32, mut p_menu_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMenu_Draw", &PROBE_N, &[(p_pkt_1) as u32, (p_menu_2) as u32], false); }
    let mut l_pad_3: i32 = 0;
    let mut l_list_4: u32 = 0;
    let mut l_padX_5: i32 = 0;
    let mut l_padY_6: i32 = 0;
    let mut l_skills_7: i32 = 0;
    let mut l_i_8: i32 = 0;
    let mut l_w_9: i32 = 0;
    let mut l_h_10: i32 = 0;
    let mut l_x_11: i32 = 0;
    let mut l_y_12: i32 = 0;
    let mut l_x1_13: i32 = 0;
    let mut l_ty_14: i32 = 0;
    let mut l_text_15: i32 = 0;
    let r_ = 'f: {
    l_pad_3 = { let a_ = (c.m.r32(sym::gBtlMenu)).wrapping_add(12); (c.m.r32(a_) as i32) };
    l_list_4 = ((c.m.r32(sym::gBtlMenu)).wrapping_add(32)).wrapping_add((((l_pad_3) as u32)).wrapping_mul(128));
    l_padX_5 = (((c.m.r16((p_menu_2).wrapping_add(12)) as i16)) as i32);
    l_padY_6 = (((c.m.r16((p_menu_2).wrapping_add(14)) as i16)) as i32);
    l_skills_7 = F::BtlMenu_IsSkillList(c, p_menu_2);
    's1: {
        let sv16: i32 = (c.m.r32((p_menu_2).wrapping_add(28)) as i32);
        match sv16 {
            0i32 | 1i32 | 2i32 | 3i32 => {
                l_w_9 = rt::f2i(F32(c.m.r32((p_menu_2).wrapping_add(32))));
                l_h_10 = rt::f2i(F32(c.m.r32((p_menu_2).wrapping_add(36))));
                if (((((((((l_w_9) < (4i32)) as i32)) != 0) && (((((l_h_10) >= (4i32)) as i32)) != 0)) as i32)) != 0) {
                    l_w_9 = 4i32;
                }
                if (((((((((l_h_10) < (4i32)) as i32)) != 0) && (((((l_w_9) >= (4i32)) as i32)) != 0)) as i32)) != 0) {
                    l_h_10 = 4i32;
                }
                l_x_11 = ((512i32).wrapping_sub(l_w_9)).wrapping_shr((1i32) as u32);
                l_y_12 = ((448i32).wrapping_sub(l_h_10)).wrapping_shr((1i32) as u32);
                F::BtlMenu_DrawFrame(c, p_pkt_1, l_x_11, (l_x_11).wrapping_add(l_w_9), l_y_12, (l_y_12).wrapping_add(l_h_10), (((l_skills_7) != (0i32)) as i32));
                break 'f;
            }
            4i32 => {
                if ((((!((F::BtlMenu_HasOpenSub(c, p_menu_2)) != 0)) as i32)) != 0) {
                    l_w_9 = F::BtlMenu_GetWidth(c, p_menu_2);
                    l_h_10 = F::BtlMenu_GetHeight(c, p_menu_2);
                    l_x_11 = ((512i32).wrapping_sub(F::BtlMenu_GetWidth(c, p_menu_2))).wrapping_shr((1i32) as u32);
                    l_y_12 = ((448i32).wrapping_sub(F::BtlMenu_GetHeight(c, p_menu_2))).wrapping_shr((1i32) as u32);
                    if ((l_skills_7) != 0) {
                        l_x1_13 = (l_x_11).wrapping_add(l_w_9);
                        F::BtlMenu_DrawFrame(c, p_pkt_1, l_x_11, l_x1_13, (l_y_12).wrapping_add(268i32), (l_y_12).wrapping_add(l_h_10), 1i32);
                        F::BtlMenu_DrawFrame(c, p_pkt_1, l_x_11, l_x1_13, l_y_12, (l_y_12).wrapping_add(273i32), 1i32);
                        {
                            l_i_8 = ((c.m.r32(l_list_4) as i32)).wrapping_sub(1i32);
                            'l2: loop {
                                if !(((((l_i_8) >= (0i32)) as i32)) != 0) { break 'l2; }
                                'c3: {
                                    F::BtlMenu_DrawPageTab(c, p_pkt_1, l_x_11, l_y_12, l_i_8);
                                }
                                { let o_ = l_i_8; let n_ = (o_).wrapping_sub(1); l_i_8 = n_; };
                            }
                        }
                        { let a5_ = (c.m.r32((l_list_4).wrapping_add(44)) as i32); F::BtlMenu_DrawPageBar(c, p_pkt_1, l_x_11, l_x1_13, (l_y_12).wrapping_add(20i32), (l_y_12).wrapping_add(57i32), a5_) };
                        { let a3_ = (c.m.r32((l_list_4).wrapping_add(44)) as i32); F::BtlMenu_DrawPageTab(c, p_pkt_1, l_x_11, l_y_12, a3_) };
                    } else {
                        F::BtlMenu_DrawFrame(c, p_pkt_1, l_x_11, (l_x_11).wrapping_add(l_w_9), l_y_12, (l_y_12).wrapping_add(l_h_10), 0i32);
                    }
                    if ((l_skills_7) != 0) {
                        F::BtlText_DrawList(c, p_pkt_1, (l_x_11).wrapping_add(l_padX_5), ((l_x_11).wrapping_sub(l_padX_5)).wrapping_add(l_w_9), (l_y_12).wrapping_add(62i32), (l_y_12).wrapping_add(258i32), 0i32);
                        F::BtlText_DrawList(c, p_pkt_1, (l_x_11).wrapping_add(l_padX_5), ((l_x_11).wrapping_sub(l_padX_5)).wrapping_add(l_w_9), (l_y_12).wrapping_add(283i32), ((l_y_12).wrapping_sub(l_padY_6)).wrapping_add(l_h_10), 1i32);
                        F::BtlText_DrawScrollBar(c, p_pkt_1, (l_x_11).wrapping_add(l_padX_5), ((l_x_11).wrapping_sub(l_padX_5)).wrapping_add(l_w_9), (l_y_12).wrapping_add(62i32), (l_y_12).wrapping_add(256i32));
                    } else {
                        if ((F::BtlMenu_IsControls(c, p_menu_2)) != 0) {
                            F::BtlText_DrawControls(c, p_pkt_1, (l_x_11).wrapping_add(l_padX_5), ((l_x_11).wrapping_sub(l_padX_5)).wrapping_add(l_w_9), (l_y_12).wrapping_add(l_padY_6), ((l_y_12).wrapping_sub(l_padY_6)).wrapping_add(l_h_10));
                        } else {
                            l_ty_14 = (l_y_12).wrapping_add(l_padY_6);
                            l_i_8 = 0i32;
                            F::Font_PushStyle(c);
                            F::Font_PushCursor(c);
                            F::Font_SetHome(c, 256i32, (l_ty_14).wrapping_add(42i32));
                            { let a0_ = F32(c.m.r32((p_menu_2).wrapping_add(8))); F::Font_SetScale(c, a0_) };
                            { let a1_ = (28i32).wrapping_sub(F::Font_GetGlyphHeight(c)); F::Font_SetSpacing(c, 0i32, a1_) };
                            F::Font_SetAlign(c, 1i32);
                            F::Font_SetShadowMode(c, 2i32);
                            F::Font_SetShadowColorRGBA(c, ((((0i32) as i64 as u64)) as u8), ((((0i32) as i64 as u64)) as u8), ((((0i32) as i64 as u64)) as u8), 64i32);
                            F::Font_SetShadowOffset(c, 1i32, 2i32);
                            {
                                'l4: loop {
                                    if !(((((l_i_8) < ((c.m.r32((p_menu_2).wrapping_add(4)) as i32))) as i32)) != 0) { break 'l4; }
                                    'c5: {
                                        if ((((((c.m.r32((p_menu_2).wrapping_add(24)) as i32)) == (l_i_8)) as i32)) != 0) {
                                            F::Font_SetColorRGBA(c, ((((244i32) as i64 as u64)) as u8), ((((255i32) as i64 as u64)) as u8), ((((63i32) as i64 as u64)) as u8), 128i32);
                                        } else {
                                            F::Font_SetColorRGBA(c, ((((137i32) as i64 as u64)) as u8), ((((195i32) as i64 as u64)) as u8), ((((229i32) as i64 as u64)) as u8), 128i32);
                                        }
                                        l_text_15 = (({ let a_ = ((c.m.r32(p_menu_2)).wrapping_add((((l_i_8) as u32)).wrapping_mul(48))).wrapping_add((((l_pad_3) as u32)).wrapping_mul(2)); (c.m.r16(a_) as i16) }) as i32);
                                        if (((((l_text_15) >= (0i32)) as i32)) != 0) {
                                            { let a0_ = F::BtlMenu_GetItemText(c, l_text_15); F::Font_Print(c, a0_) };
                                            F::Font_NewLine(c);
                                        } else {
                                            F::BtlMenu_DrawPart(c, p_pkt_1, (l_x_11).wrapping_add(rt::div_i32(l_w_9, 2i32)), (l_ty_14).wrapping_add(14i32), 256i32, 32i32, 0i32, (!(l_text_15)), 1i32);
                                        }
                                    }
                                    { let o_ = l_i_8; let n_ = (o_).wrapping_add(1); l_i_8 = n_; };
                                }
                            }
                            F::Font_PopCursor(c);
                            F::Font_PopStyle(c);
                        }
                    }
                    if ((((((((!((l_skills_7) != 0)) as i32)) != 0) && ((((!((F::BtlMenu_IsControls(c, p_menu_2)) != 0)) as i32)) != 0)) as i32)) != 0) {
                        F::FontIcon_ResetAnim(c);
                    }
                }
                {
                    l_i_8 = 0i32;
                    'l6: loop {
                        if !(((((l_i_8) < ((c.m.r32((p_menu_2).wrapping_add(4)) as i32))) as i32)) != 0) { break 'l6; }
                        'c7: {
                            if ((((((((({ let a_ = ((c.m.r32(p_menu_2)).wrapping_add((((l_i_8) as u32)).wrapping_mul(48))).wrapping_add(8); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) && ((((((c.m.r32((p_menu_2).wrapping_add(20)) as i32)) == (l_i_8)) as i32)) != 0)) as i32)) != 0) {
                                { let a1_ = { let a_ = ((c.m.r32(p_menu_2)).wrapping_add((((l_i_8) as u32)).wrapping_mul(48))).wrapping_add(8); c.m.r32(a_) }; F::BtlMenu_Draw(c, p_pkt_1, a1_) };
                            }
                        }
                        { let o_ = l_i_8; let n_ = (o_).wrapping_add(1); l_i_8 = n_; };
                    }
                }
                break 's1;
            }
            _ => {
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMenu_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMenu_CallItem(c: &mut Ctx, mut p_menu_1: u32, mut p_idx_2: i32, mut p_leaf_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMenu_CallItem", &PROBE_N, &[(p_menu_1) as u32, (p_idx_2) as u32, (p_leaf_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_item_4: u32 = 0;
    let mut l_result_5: i32 = 0;
    let r_ = 'f: {
        l_item_4 = (c.m.r32(p_menu_1)).wrapping_add((((p_idx_2) as u32)).wrapping_mul(48));
        if (((((((c.m.r32((l_item_4).wrapping_add(32)) as i32)) != 0) && (((((c.m.r32((l_item_4).wrapping_add(12))) != (((0i32) as u32))) as i32)) != 0)) as i32)) != 0) {
            { let v_ = p_menu_1; let a_ = fp; c.m.w32(a_, v_); };
            { let v_ = (c.m.r32((l_item_4).wrapping_add(4)) as i32); let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
            { let v_ = (c.m.r32((l_item_4).wrapping_add(16)) as i32); let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
            { let v_ = ((((c.m.r32((l_item_4).wrapping_add(36)) as i32)) == (0i32)) as i32); let a_ = (fp).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
            { let v_ = (c.m.r32((l_item_4).wrapping_add(40)) as i32); let a_ = (fp).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
            l_result_5 = { let fp_ = c.m.r32((l_item_4).wrapping_add(12)); ((D::call(c, fp_, &[((fp) as u64)])) as i32) };
            if (((((l_result_5) != (0i32)) as i32)) != 0) {
                { let v_ = 0i32; let a_ = (l_item_4).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
            }
            { let v_ = 0i32; let a_ = (l_item_4).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
            { let v_ = 1i32; let a_ = (l_item_4).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
            if ((p_leaf_3) != 0) {
                { let v_ = l_result_5; let a_ = (c.m.r32(sym::gBtlMenu)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlMenu_CallItem", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMenu_FixCursor(c: &mut Ctx, mut p_menu_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMenu_FixCursor", &PROBE_N, &[(p_menu_1) as u32], false); }
    let mut l_start_2: i32 = 0;
    let mut l_c_3: i32 = 0;
    let mut l_n_4: i32 = 0;
    let r_ = 'f: {
    l_start_2 = (c.m.r32((p_menu_1).wrapping_add(24)) as i32);
    if ((((((((!((F::BtlMenu_IsSkillList(c, p_menu_1)) != 0)) as i32)) != 0) && ((((!((F::BtlMenu_IsControls(c, p_menu_1)) != 0)) as i32)) != 0)) as i32)) != 0) {
        'l1: loop {
            'c2: {
                l_c_3 = (c.m.r32((p_menu_1).wrapping_add(24)) as i32);
                if (((((l_c_3) < (0i32)) as i32)) != 0) {
                    l_n_4 = ((c.m.r32((p_menu_1).wrapping_add(4)) as i32)).wrapping_sub(1i32);
                    { let v_ = l_n_4; let a_ = (p_menu_1).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                    l_c_3 = l_n_4;
                }
                if (((((l_c_3) >= ((c.m.r32((p_menu_1).wrapping_add(4)) as i32))) as i32)) != 0) {
                    l_n_4 = 0i32;
                    { let v_ = l_n_4; let a_ = (p_menu_1).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                    l_c_3 = l_n_4;
                }
                if ((((!(({ let a_ = ((c.m.r32(p_menu_1)).wrapping_add((((l_c_3) as u32)).wrapping_mul(48))).wrapping_add(28); (c.m.r32(a_) as i32) }) != 0)) as i32)) != 0) {
                    break 'l1;
                }
                l_n_4 = (l_c_3).wrapping_add(1i32);
                { let v_ = l_n_4; let a_ = (p_menu_1).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
            }
            if !(((((l_n_4) != (l_start_2)) as i32)) != 0) { break 'l1; }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMenu_FixCursor", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMenu_SetActive(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMenu_SetActive", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let v_ = 1i32; let a_ = (c.m.r32(sym::gBtlMenu)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMenu_SetActive", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMenu_ClearActive(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMenu_ClearActive", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let v_ = 0i32; let a_ = (c.m.r32(sym::gBtlMenu)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMenu_ClearActive", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMenu_Init(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMenu_Init", &PROBE_N, &[], false); }
    let mut l_file_1: u32 = 0;
    let r_ = 'f: {
    { let v_ = F::Heap_Alloc(c, 288i32, ((32i32) as u32), 0i32, 2i32); let a_ = sym::gBtlMenu; c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32(sym::gBtlMenu); F::memset(c, a0_, 0i32, ((288i32) as u32)) }; };
    F::BtlMenu_SetActive(c);
    l_file_1 = { let a_ = (c.m.r32(sym::gCommonRes)).wrapping_add(((40i32) as u32)); c.m.r32(a_) };
    { let v_ = (l_file_1).wrapping_add((rt::div_u32(c.m.r32((l_file_1).wrapping_add((((7i32) as u32)).wrapping_mul(4))), ((4i32) as u32))).wrapping_mul(4)); let a_ = c.m.r32(sym::gBtlMenu); c.m.w32(a_, v_); };
    { let a0_ = c.m.r32(sym::gBtlMenu); let a1_ = { let a_ = c.m.r32(sym::gBtlMenu); c.m.r32(a_) }; let a2_ = { let a_ = c.m.r32(sym::gBtlMenu); c.m.r32(a_) }; F::Res_RelocateOffsets(c, a0_, a1_, a2_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMenu_Init", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMenu_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMenu_Term", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let a0_ = c.m.r32(sym::gBtlMenu); F::Heap_Free(c, a0_) };
    { let v_ = ((0i32) as u32); let a_ = sym::gBtlMenu; c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMenu_Term", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMenu_DrawAll(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMenu_DrawAll", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_first_1: i32 = 0;
    let r_ = 'f: {
        if (((((({ let a_ = (c.m.r32(sym::gBtlMenu)).wrapping_add(8); (c.m.r32(a_) as i32) }) != 0) && ((((({ let a_ = (c.m.r32(sym::gBtlMenu)).wrapping_add(20); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0)) as i32)) != 0) {
            F::BtlMenu_UploadTex(c);
            l_first_1 = F::Font_GetCmdCount(c);
            F::BtlMenu_BeginDraw(c, fp);
            { let a1_ = { let a_ = (c.m.r32(sym::gBtlMenu)).wrapping_add(20); c.m.r32(a_) }; F::BtlMenu_Draw(c, fp, a1_) };
            F::BtlMenu_EndDraw(c, fp);
            F::Font_Flush(c, l_first_1);
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlMenu_DrawAll", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMenu_GetWork(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMenu_GetWork", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f c.m.r32(sym::gBtlMenu);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMenu_GetWork", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMenu_SetTop(c: &mut Ctx, mut p_menu_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMenu_SetTop", &PROBE_N, &[(p_menu_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_menu_1; let a_ = (c.m.r32(sym::gBtlMenu)).wrapping_add(20); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMenu_SetTop", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMenu_Update(c: &mut Ctx, mut p_which_1: i32, mut p_parent_2: u32, mut p_menu_3: u32, mut p_pad_4: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMenu_Update", &PROBE_N, &[(p_which_1) as u32, (p_parent_2) as u32, (p_menu_3) as u32, (p_pad_4) as u32], false); }
    let mut l_press_5: u32 = 0;
    let mut l_sound_6: i32 = 0;
    let mut l_pw_7: i32 = 0;
    let mut l_ph_8: i32 = 0;
    let mut l_w_9: i32 = 0;
    let mut l_h_10: i32 = 0;
    let mut l_move_11: i32 = 0;
    let mut l_old_12: i32 = 0;
    let mut l_item_13: u32 = 0;
    let mut l_i_14: i32 = 0;
    let mut sv_15: i32 = 0;
    let mut sv_16: i32 = 0;
    let mut bb_: u32 = 0;
    let r_ = 'f: {
    'fl1: loop {
        match bb_ {
            0 => {
                { let v_ = 1i32; let a_ = (c.m.r32(sym::gBtlMenu)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                { let v_ = p_pad_4; let a_ = (c.m.r32(sym::gBtlMenu)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                if (((((F::Battle_GetMode(c)) == (5i32)) as i32)) != 0) { bb_ = 1; continue 'fl1; } else { bb_ = 3; continue 'fl1; }
            }
            1 => {
                { let v_ = F::BtlMenu_GetDefaultPadType(c); let a_ = (c.m.r32(sym::gBtlMenu)).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
                bb_ = 2; continue 'fl1;
            }
            2 => {
                { let a0_ = { let a_ = (c.m.r32(sym::gBtlMenu)).wrapping_add(16); (c.m.r32(a_) as i32) }; F::FontIcon_SetPadType(c, a0_) };
                F::BtlText_CountEntries(c);
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gBtlMenu)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                F::BtlMenu_FixCursor(c, p_menu_3);
                if ((((((c.m.r32((p_menu_3).wrapping_add(20)) as i32)) >= (0i32)) as i32)) != 0) { bb_ = 4; continue 'fl1; } else { bb_ = 5; continue 'fl1; }
            }
            3 => {
                { let v_ = F::Pad_GetLastStatus(c, p_pad_4); let a_ = (c.m.r32(sym::gBtlMenu)).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
                bb_ = 2; continue 'fl1;
            }
            4 => {
                if ((((({ let a_ = ((c.m.r32(p_menu_3)).wrapping_add(((((c.m.r32((p_menu_3).wrapping_add(20)) as i32)) as u32)).wrapping_mul(48))).wrapping_add(8); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) { bb_ = 6; continue 'fl1; } else { bb_ = 7; continue 'fl1; }
            }
            5 => {
                if (((((({ let a_ = ((c.m.r32(p_menu_3)).wrapping_add(((((c.m.r32((p_menu_3).wrapping_add(24)) as i32)) as u32)).wrapping_mul(48))).wrapping_add(32); (c.m.r32(a_) as i32) }) != 0) && ((((({ let a_ = ((c.m.r32(p_menu_3)).wrapping_add(((((c.m.r32((p_menu_3).wrapping_add(24)) as i32)) as u32)).wrapping_mul(48))).wrapping_add(12); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0)) as i32)) != 0) { bb_ = 11; continue 'fl1; } else { bb_ = 12; continue 'fl1; }
            }
            6 => {
                { let a1_ = (c.m.r32((p_menu_3).wrapping_add(20)) as i32); F::BtlMenu_CallItem(c, p_menu_3, a1_, 0i32) };
                if (({ let a2_ = { let a_ = ((c.m.r32(p_menu_3)).wrapping_add(((((c.m.r32((p_menu_3).wrapping_add(20)) as i32)) as u32)).wrapping_mul(48))).wrapping_add(8); c.m.r32(a_) }; F::BtlMenu_Update(c, p_which_1, p_menu_3, a2_, p_pad_4) }) != 0) { bb_ = 8; continue 'fl1; } else { bb_ = 9; continue 'fl1; }
            }
            7 => {
                { let v_ = (1i32).wrapping_neg(); let a_ = (p_menu_3).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
                bb_ = 5; continue 'fl1;
            }
            8 => {
                { let v_ = 0i32; let a_ = ((c.m.r32(p_menu_3)).wrapping_add(((((c.m.r32((p_menu_3).wrapping_add(20)) as i32)) as u32)).wrapping_mul(48))).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
                { let v_ = (1i32).wrapping_neg(); let a_ = (p_menu_3).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
                bb_ = 9; continue 'fl1;
            }
            9 => {
                break 'f 0i32;
            }
            10 => {
                bb_ = 7; continue 'fl1;
            }
            11 => {
                bb_ = 13; continue 'fl1;
            }
            12 => {
                l_press_5 = c.m.r32(((sym::gPad).wrapping_add((((p_pad_4) as u32)).wrapping_mul(448))).wrapping_add(396));
                if ((F::BtlMenu_IsSkillList(c, p_menu_3)) != 0) { bb_ = 15; continue 'fl1; } else { bb_ = 16; continue 'fl1; }
            }
            13 => {
                if ((((({ let a_ = ((c.m.r32(p_menu_3)).wrapping_add(((((c.m.r32((p_menu_3).wrapping_add(24)) as i32)) as u32)).wrapping_mul(48))).wrapping_add(8); c.m.r32(a_) }) == (((0i32) as u32))) as i32)) != 0) { bb_ = 111; continue 'fl1; } else { bb_ = 112; continue 'fl1; }
            }
            14 => {
                bb_ = 12; continue 'fl1;
            }
            15 => {
                { let v_ = 1i32; let a_ = (c.m.r32(sym::gBtlMenu)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                bb_ = 16; continue 'fl1;
            }
            16 => {
                if ((((((F::BtlMenu_IsSkillList(c, p_menu_3)) != 0) || ((F::BtlMenu_IsControls(c, p_menu_3)) != 0)) as i32)) != 0) { bb_ = 17; continue 'fl1; } else { bb_ = 19; continue 'fl1; }
            }
            17 => {
                l_sound_6 = 0i32;
                bb_ = 18; continue 'fl1;
            }
            18 => {
                sv_15 = (c.m.r32((p_menu_3).wrapping_add(28)) as i32);
                bb_ = match sv_15 { 0i32 => 22, 1i32 => 23, 2i32 => 27, 3i32 => 28, 4i32 => 33, _ => 20, }; continue 'fl1;
            }
            19 => {
                l_sound_6 = 1i32;
                bb_ = 18; continue 'fl1;
            }
            20 => {
                bb_ = 13; continue 'fl1;
            }
            21 => {
                bb_ = 22; continue 'fl1;
            }
            22 => {
                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 4i32); };
                { let v_ = 1i32; let a_ = (p_menu_3).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                l_pw_7 = F::BtlMenu_GetWidth(c, p_parent_2);
                l_ph_8 = F::BtlMenu_GetHeight(c, p_parent_2);
                l_w_9 = F::BtlMenu_GetWidth(c, p_menu_3);
                l_h_10 = F::BtlMenu_GetHeight(c, p_menu_3);
                { let v_ = 0i32; let a_ = (p_menu_3).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
                { let v_ = rt::i2f(l_pw_7); let a_ = (p_menu_3).wrapping_add(32); c.m.w32(a_, (v_).0); };
                { let v_ = rt::i2f(l_ph_8); let a_ = (p_menu_3).wrapping_add(36); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fdiv(rt::i2f((l_w_9).wrapping_sub(l_pw_7)), F32(0x40c00000)); let a_ = (p_menu_3).wrapping_add(40); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fdiv(rt::i2f((l_h_10).wrapping_sub(l_ph_8)), F32(0x40c00000)); let a_ = (p_menu_3).wrapping_add(44); c.m.w32(a_, (v_).0); };
                bb_ = 23; continue 'fl1;
            }
            23 => {
                { let r_ = F32(c.m.r32((p_menu_3).wrapping_add(40))); let a_ = (p_menu_3).wrapping_add(32); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                { let r_ = F32(c.m.r32((p_menu_3).wrapping_add(44))); let a_ = (p_menu_3).wrapping_add(36); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                { let a_ = (p_menu_3).wrapping_add(48); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                if ((((((c.m.r32((p_menu_3).wrapping_add(48)) as i32)) >= (6i32)) as i32)) != 0) { bb_ = 24; continue 'fl1; } else { bb_ = 25; continue 'fl1; }
            }
            24 => {
                { let v_ = rt::i2f(F::BtlMenu_GetWidth(c, p_menu_3)); let a_ = (p_menu_3).wrapping_add(32); c.m.w32(a_, (v_).0); };
                { let v_ = rt::i2f(F::BtlMenu_GetHeight(c, p_menu_3)); let a_ = (p_menu_3).wrapping_add(36); c.m.w32(a_, (v_).0); };
                { let v_ = 4i32; let a_ = (p_menu_3).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                bb_ = 25; continue 'fl1;
            }
            25 => {
                bb_ = 20; continue 'fl1;
            }
            26 => {
                bb_ = 27; continue 'fl1;
            }
            27 => {
                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 5i32); };
                { let v_ = 3i32; let a_ = (p_menu_3).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                l_pw_7 = F::BtlMenu_GetWidth(c, p_parent_2);
                l_ph_8 = F::BtlMenu_GetHeight(c, p_parent_2);
                l_w_9 = F::BtlMenu_GetWidth(c, p_menu_3);
                l_h_10 = F::BtlMenu_GetHeight(c, p_menu_3);
                { let v_ = 0i32; let a_ = (p_menu_3).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
                { let v_ = rt::i2f(l_w_9); let a_ = (p_menu_3).wrapping_add(32); c.m.w32(a_, (v_).0); };
                { let v_ = rt::i2f(l_h_10); let a_ = (p_menu_3).wrapping_add(36); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fdiv(rt::i2f((l_pw_7).wrapping_sub(l_w_9)), F32(0x40c00000)); let a_ = (p_menu_3).wrapping_add(40); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fdiv(rt::i2f((l_ph_8).wrapping_sub(l_h_10)), F32(0x40c00000)); let a_ = (p_menu_3).wrapping_add(44); c.m.w32(a_, (v_).0); };
                bb_ = 28; continue 'fl1;
            }
            28 => {
                { let r_ = F32(c.m.r32((p_menu_3).wrapping_add(40))); let a_ = (p_menu_3).wrapping_add(32); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                { let r_ = F32(c.m.r32((p_menu_3).wrapping_add(44))); let a_ = (p_menu_3).wrapping_add(36); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                { let a_ = (p_menu_3).wrapping_add(48); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                if ((((((c.m.r32((p_menu_3).wrapping_add(48)) as i32)) >= (6i32)) as i32)) != 0) { bb_ = 29; continue 'fl1; } else { bb_ = 30; continue 'fl1; }
            }
            29 => {
                { let v_ = rt::i2f(F::BtlMenu_GetWidth(c, p_parent_2)); let a_ = (p_menu_3).wrapping_add(32); c.m.w32(a_, (v_).0); };
                { let v_ = rt::i2f(F::BtlMenu_GetHeight(c, p_parent_2)); let a_ = (p_menu_3).wrapping_add(36); c.m.w32(a_, (v_).0); };
                { let v_ = 0i32; let a_ = (p_menu_3).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                break 'f 1i32;
            }
            30 => {
                bb_ = 20; continue 'fl1;
            }
            31 => {
                bb_ = 30; continue 'fl1;
            }
            32 => {
                bb_ = 33; continue 'fl1;
            }
            33 => {
                if (((((p_pad_4) >= (0i32)) as i32)) != 0) { bb_ = 34; continue 'fl1; } else { bb_ = 35; continue 'fl1; }
            }
            34 => {
                if ((((({ let a_ = (c.m.r32(sym::gBtlMenu)).wrapping_add(20); c.m.r32(a_) }) != (p_menu_3)) as i32)) != 0) { bb_ = 36; continue 'fl1; } else { bb_ = 38; continue 'fl1; }
            }
            35 => {
                l_move_11 = 0i32;
                l_old_12 = (c.m.r32((p_menu_3).wrapping_add(24)) as i32);
                if (((((p_pad_4) >= (0i32)) as i32)) != 0) { bb_ = 45; continue 'fl1; } else { bb_ = 46; continue 'fl1; }
            }
            36 => {
                if ((((l_press_5) & (((1024i32) as u32)))) != 0) { bb_ = 39; continue 'fl1; } else { bb_ = 40; continue 'fl1; }
            }
            37 => {
                bb_ = 35; continue 'fl1;
            }
            38 => {
                if ((((((((l_press_5) & (((((4096i32) | (1024i32))) as u32)))) != 0) && (((((p_which_1) == (0i32)) as i32)) != 0)) as i32)) != 0) { bb_ = 42; continue 'fl1; } else { bb_ = 43; continue 'fl1; }
            }
            39 => {
                { let v_ = 2i32; let a_ = (p_menu_3).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                bb_ = 20; continue 'fl1;
            }
            40 => {
                bb_ = 37; continue 'fl1;
            }
            41 => {
                bb_ = 40; continue 'fl1;
            }
            42 => {
                { let v_ = 2i32; let a_ = (p_menu_3).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                bb_ = 20; continue 'fl1;
            }
            43 => {
                bb_ = 37; continue 'fl1;
            }
            44 => {
                bb_ = 43; continue 'fl1;
            }
            45 => {
                if ((((l_press_5) & (((8i32) as u32)))) != 0) { bb_ = 47; continue 'fl1; } else { bb_ = 48; continue 'fl1; }
            }
            46 => {
                if (((((l_move_11) != (0i32)) as i32)) != 0) { bb_ = 55; continue 'fl1; } else { bb_ = 57; continue 'fl1; }
            }
            47 => {
                if ((l_sound_6) != 0) { bb_ = 49; continue 'fl1; } else { bb_ = 50; continue 'fl1; }
            }
            48 => {
                if ((((l_press_5) & (((4i32) as u32)))) != 0) { bb_ = 51; continue 'fl1; } else { bb_ = 52; continue 'fl1; }
            }
            49 => {
                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                bb_ = 50; continue 'fl1;
            }
            50 => {
                l_move_11 = (1i32).wrapping_neg();
                bb_ = 48; continue 'fl1;
            }
            51 => {
                if ((l_sound_6) != 0) { bb_ = 53; continue 'fl1; } else { bb_ = 54; continue 'fl1; }
            }
            52 => {
                bb_ = 46; continue 'fl1;
            }
            53 => {
                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                bb_ = 54; continue 'fl1;
            }
            54 => {
                l_move_11 = 1i32;
                bb_ = 52; continue 'fl1;
            }
            55 => {
                { let r_ = l_move_11; let a_ = (p_menu_3).wrapping_add(24); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                bb_ = 58; continue 'fl1;
            }
            56 => {
                bb_ = 20; continue 'fl1;
            }
            57 => {
                if (((((p_pad_4) >= (0i32)) as i32)) != 0) { bb_ = 72; continue 'fl1; } else { bb_ = 73; continue 'fl1; }
            }
            58 => {
                bb_ = 59; continue 'fl1;
            }
            59 => {
                if ((((((c.m.r32((p_menu_3).wrapping_add(24)) as i32)) < (0i32)) as i32)) != 0) { bb_ = 62; continue 'fl1; } else { bb_ = 63; continue 'fl1; }
            }
            60 => {
                bb_ = 58; continue 'fl1;
            }
            61 => {
                bb_ = 56; continue 'fl1;
            }
            62 => {
                { let v_ = ((c.m.r32((p_menu_3).wrapping_add(4)) as i32)).wrapping_sub(1i32); let a_ = (p_menu_3).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                bb_ = 63; continue 'fl1;
            }
            63 => {
                if ((((((c.m.r32((p_menu_3).wrapping_add(24)) as i32)) >= ((c.m.r32((p_menu_3).wrapping_add(4)) as i32))) as i32)) != 0) { bb_ = 64; continue 'fl1; } else { bb_ = 65; continue 'fl1; }
            }
            64 => {
                { let v_ = 0i32; let a_ = (p_menu_3).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                bb_ = 65; continue 'fl1;
            }
            65 => {
                if ((((((c.m.r32((p_menu_3).wrapping_add(24)) as i32)) == (l_old_12)) as i32)) != 0) { bb_ = 66; continue 'fl1; } else { bb_ = 67; continue 'fl1; }
            }
            66 => {
                bb_ = 61; continue 'fl1;
            }
            67 => {
                if ((((!(({ let a_ = ((c.m.r32(p_menu_3)).wrapping_add(((((c.m.r32((p_menu_3).wrapping_add(24)) as i32)) as u32)).wrapping_mul(48))).wrapping_add(28); (c.m.r32(a_) as i32) }) != 0)) as i32)) != 0) { bb_ = 69; continue 'fl1; } else { bb_ = 70; continue 'fl1; }
            }
            68 => {
                bb_ = 67; continue 'fl1;
            }
            69 => {
                bb_ = 61; continue 'fl1;
            }
            70 => {
                { let r_ = l_move_11; let a_ = (p_menu_3).wrapping_add(24); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                bb_ = 60; continue 'fl1;
            }
            71 => {
                bb_ = 70; continue 'fl1;
            }
            72 => {
                l_item_13 = (c.m.r32(p_menu_3)).wrapping_add(((((c.m.r32((p_menu_3).wrapping_add(24)) as i32)) as u32)).wrapping_mul(48));
                if ((((l_press_5) & (((512i32) as u32)))) != 0) { bb_ = 74; continue 'fl1; } else { bb_ = 75; continue 'fl1; }
            }
            73 => {
                bb_ = 56; continue 'fl1;
            }
            74 => {
                { let v_ = 1i32; let a_ = (l_item_13).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
                if (((((c.m.r32((l_item_13).wrapping_add(8))) != (((0i32) as u32))) as i32)) != 0) { bb_ = 76; continue 'fl1; } else { bb_ = 78; continue 'fl1; }
            }
            75 => {
                bb_ = 73; continue 'fl1;
            }
            76 => {
                { let v_ = (c.m.r32((p_menu_3).wrapping_add(24)) as i32); let a_ = (p_menu_3).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
                { let _ = { let a2_ = { let a_ = ((c.m.r32(p_menu_3)).wrapping_add(((((c.m.r32((p_menu_3).wrapping_add(20)) as i32)) as u32)).wrapping_mul(48))).wrapping_add(8); c.m.r32(a_) }; F::BtlMenu_Update(c, p_which_1, p_menu_3, a2_, p_pad_4) }; };
                if (((((c.m.r32((l_item_13).wrapping_add(12))) != (((0i32) as u32))) as i32)) != 0) { bb_ = 79; continue 'fl1; } else { bb_ = 80; continue 'fl1; }
            }
            77 => {
                bb_ = 75; continue 'fl1;
            }
            78 => {
                if (((((c.m.r32((l_item_13).wrapping_add(12))) != (((0i32) as u32))) as i32)) != 0) { bb_ = 81; continue 'fl1; } else { bb_ = 83; continue 'fl1; }
            }
            79 => {
                { let v_ = 1i32; let a_ = (l_item_13).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
                { let v_ = 1i32; let a_ = (l_item_13).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
                bb_ = 80; continue 'fl1;
            }
            80 => {
                bb_ = 77; continue 'fl1;
            }
            81 => {
                { let v_ = 1i32; let a_ = (l_item_13).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
                { let v_ = 1i32; let a_ = (l_item_13).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
                bb_ = 82; continue 'fl1;
            }
            82 => {
                bb_ = 77; continue 'fl1;
            }
            83 => {
                sv_16 = (c.m.r32((l_item_13).wrapping_add(20)) as i32);
                bb_ = match sv_16 { 2i32 => 86, 0i32 => 88, _ => 84, }; continue 'fl1;
            }
            84 => {
                bb_ = 82; continue 'fl1;
            }
            85 => {
                bb_ = 86; continue 'fl1;
            }
            86 => {
                { let v_ = 2i32; let a_ = (p_menu_3).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                bb_ = 84; continue 'fl1;
            }
            87 => {
                bb_ = 88; continue 'fl1;
            }
            88 => {
                if ((((((c.m.r32((p_menu_3).wrapping_add(16)) as i32)) == (1i32)) as i32)) != 0) { bb_ = 89; continue 'fl1; } else { bb_ = 91; continue 'fl1; }
            }
            89 => {
                l_i_14 = 0i32;
                bb_ = 92; continue 'fl1;
            }
            90 => {
                bb_ = 84; continue 'fl1;
            }
            91 => {
                if ((((((c.m.r32((p_menu_3).wrapping_add(16)) as i32)) == (2i32)) as i32)) != 0) { bb_ = 96; continue 'fl1; } else { bb_ = 98; continue 'fl1; }
            }
            92 => {
                if (((((l_i_14) < ((c.m.r32((p_menu_3).wrapping_add(4)) as i32))) as i32)) != 0) { bb_ = 93; continue 'fl1; } else { bb_ = 95; continue 'fl1; }
            }
            93 => {
                { let v_ = 0i32; let a_ = ((c.m.r32(p_menu_3)).wrapping_add((((l_i_14) as u32)).wrapping_mul(48))).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                bb_ = 94; continue 'fl1;
            }
            94 => {
                { let o_ = l_i_14; let n_ = (o_).wrapping_add(1); l_i_14 = n_; };
                bb_ = 92; continue 'fl1;
            }
            95 => {
                { let v_ = 1i32; let a_ = (l_item_13).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                bb_ = 90; continue 'fl1;
            }
            96 => {
                { let r_ = 1i32; let a_ = (l_item_13).wrapping_add(24); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
                if (((c.m.r32((l_item_13).wrapping_add(24)) as i32)) != 0) { bb_ = 99; continue 'fl1; } else { bb_ = 100; continue 'fl1; }
            }
            97 => {
                bb_ = 90; continue 'fl1;
            }
            98 => {
                if ((((((c.m.r32((p_menu_3).wrapping_add(16)) as i32)) == (0i32)) as i32)) != 0) { bb_ = 107; continue 'fl1; } else { bb_ = 108; continue 'fl1; }
            }
            99 => {
                l_i_14 = 0i32;
                bb_ = 101; continue 'fl1;
            }
            100 => {
                bb_ = 97; continue 'fl1;
            }
            101 => {
                if (((((l_i_14) < ((c.m.r32((p_menu_3).wrapping_add(4)) as i32))) as i32)) != 0) { bb_ = 102; continue 'fl1; } else { bb_ = 104; continue 'fl1; }
            }
            102 => {
                if (((((l_i_14) != ((c.m.r32((p_menu_3).wrapping_add(24)) as i32))) as i32)) != 0) { bb_ = 105; continue 'fl1; } else { bb_ = 106; continue 'fl1; }
            }
            103 => {
                { let o_ = l_i_14; let n_ = (o_).wrapping_add(1); l_i_14 = n_; };
                bb_ = 101; continue 'fl1;
            }
            104 => {
                bb_ = 100; continue 'fl1;
            }
            105 => {
                { let v_ = 0i32; let a_ = ((c.m.r32(p_menu_3)).wrapping_add((((l_i_14) as u32)).wrapping_mul(48))).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                bb_ = 106; continue 'fl1;
            }
            106 => {
                bb_ = 103; continue 'fl1;
            }
            107 => {
                { let r_ = 1i32; let a_ = (l_item_13).wrapping_add(24); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
                bb_ = 108; continue 'fl1;
            }
            108 => {
                bb_ = 97; continue 'fl1;
            }
            109 => {
                bb_ = 84; continue 'fl1;
            }
            110 => {
                bb_ = 20; continue 'fl1;
            }
            111 => {
                { let a1_ = (c.m.r32((p_menu_3).wrapping_add(24)) as i32); F::BtlMenu_CallItem(c, p_menu_3, a1_, 1i32) };
                bb_ = 112; continue 'fl1;
            }
            112 => {
                break 'f 0i32;
            }
            113 => {
                break 'fl1;
            }
            _ => unreachable!(),
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMenu_Update", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMenu_GetValue(c: &mut Ctx, mut p_menu_1: u32, mut p_id_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMenu_GetValue", &PROBE_N, &[(p_menu_1) as u32, (p_id_2) as u32], false); }
    let mut l_item_3: u32 = 0;
    let r_ = 'f: {
    l_item_3 = F::BtlMenu_FindItem(c, p_menu_1, p_id_2);
    if (((((l_item_3) != (((0i32) as u32))) as i32)) != 0) {
        break 'f (c.m.r32((l_item_3).wrapping_add(24)) as i32);
    }
    break 'f ((l_item_3) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMenu_GetValue", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMenu_SetValue(c: &mut Ctx, mut p_menu_1: u32, mut p_id_2: i32, mut p_value_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMenu_SetValue", &PROBE_N, &[(p_menu_1) as u32, (p_id_2) as u32, (p_value_3) as u32], false); }
    let mut l_item_4: u32 = 0;
    let r_ = 'f: {
    l_item_4 = F::BtlMenu_FindItem(c, p_menu_1, p_id_2);
    if (((((l_item_4) != (((0i32) as u32))) as i32)) != 0) {
        { let v_ = p_value_3; let a_ = (l_item_4).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMenu_SetValue", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMenu_GetPicked(c: &mut Ctx, mut p_menu_1: u32, mut p_id_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMenu_GetPicked", &PROBE_N, &[(p_menu_1) as u32, (p_id_2) as u32], false); }
    let mut l_item_3: u32 = 0;
    let r_ = 'f: {
    l_item_3 = F::BtlMenu_FindItem(c, p_menu_1, p_id_2);
    if (((((l_item_3) != (((0i32) as u32))) as i32)) != 0) {
        break 'f (c.m.r32((l_item_3).wrapping_add(44)) as i32);
    }
    break 'f ((l_item_3) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMenu_GetPicked", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMenu_SetPicked(c: &mut Ctx, mut p_menu_1: u32, mut p_id_2: i32, mut p_picked_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMenu_SetPicked", &PROBE_N, &[(p_menu_1) as u32, (p_id_2) as u32, (p_picked_3) as u32], false); }
    let mut l_item_4: u32 = 0;
    let r_ = 'f: {
    l_item_4 = F::BtlMenu_FindItem(c, p_menu_1, p_id_2);
    if (((((l_item_4) != (((0i32) as u32))) as i32)) != 0) {
        { let v_ = p_picked_3; let a_ = (l_item_4).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMenu_SetPicked", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMenu_SetDisabled(c: &mut Ctx, mut p_menu_1: u32, mut p_id_2: i32, mut p_disabled_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMenu_SetDisabled", &PROBE_N, &[(p_menu_1) as u32, (p_id_2) as u32, (p_disabled_3) as u32], false); }
    let mut l_item_4: u32 = 0;
    let r_ = 'f: {
    l_item_4 = F::BtlMenu_FindItem(c, p_menu_1, p_id_2);
    if (((((l_item_4) != (((0i32) as u32))) as i32)) != 0) {
        { let v_ = p_disabled_3; let a_ = (l_item_4).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMenu_SetDisabled", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMenu_Reset(c: &mut Ctx, mut p_menu_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMenu_Reset", &PROBE_N, &[(p_menu_1) as u32], false); }
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
    if (((((p_menu_1) != (((0i32) as u32))) as i32)) != 0) {
        { let v_ = (1i32).wrapping_neg(); let a_ = (p_menu_1).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (p_menu_1).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (p_menu_1).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
        {
            l_i_2 = 0i32;
            'l1: loop {
                if !(((((l_i_2) < ((c.m.r32((p_menu_1).wrapping_add(4)) as i32))) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let v_ = 0i32; let a_ = ((c.m.r32(p_menu_1)).wrapping_add((((l_i_2) as u32)).wrapping_mul(48))).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = ((c.m.r32(p_menu_1)).wrapping_add((((l_i_2) as u32)).wrapping_mul(48))).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = ((c.m.r32(p_menu_1)).wrapping_add((((l_i_2) as u32)).wrapping_mul(48))).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
                    if ((((({ let a_ = ((c.m.r32(p_menu_1)).wrapping_add((((l_i_2) as u32)).wrapping_mul(48))).wrapping_add(8); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
                        { let a0_ = { let a_ = ((c.m.r32(p_menu_1)).wrapping_add((((l_i_2) as u32)).wrapping_mul(48))).wrapping_add(8); c.m.r32(a_) }; F::BtlMenu_Reset(c, a0_) };
                    }
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gBtlMenu)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMenu_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMenu_FindResult(c: &mut Ctx, mut p_top_1: u32, mut p_menu_2: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMenu_FindResult", &PROBE_N, &[(p_top_1) as u32, (p_menu_2) as u32], false); }
    let mut l_found_3: u32 = 0;
    let mut l_i_4: i32 = 0;
    let r_ = 'f: {
    l_found_3 = ((0i32) as u32);
    {
        l_i_4 = 0i32;
        'l1: loop {
            if !(((((l_i_4) < ((c.m.r32((p_menu_2).wrapping_add(4)) as i32))) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((((((((({ let a_ = ((c.m.r32(p_menu_2)).wrapping_add((((l_i_4) as u32)).wrapping_mul(48))).wrapping_add(20); (c.m.r32(a_) as i32) }) == (1i32)) as i32)) != 0) && (({ let a_ = ((c.m.r32(p_menu_2)).wrapping_add((((l_i_4) as u32)).wrapping_mul(48))).wrapping_add(44); (c.m.r32(a_) as i32) }) != 0)) as i32)) != 0) {
                    if (((((p_top_1) == (((0i32) as u32))) as i32)) != 0) {
                        l_found_3 = (c.m.r32(p_menu_2)).wrapping_add((((l_i_4) as u32)).wrapping_mul(48));
                    } else {
                        l_found_3 = (c.m.r32(p_top_1)).wrapping_add(((((c.m.r32((p_top_1).wrapping_add(24)) as i32)) as u32)).wrapping_mul(48));
                    }
                    break 'l1;
                }
                if ((((({ let a_ = ((c.m.r32(p_menu_2)).wrapping_add((((l_i_4) as u32)).wrapping_mul(48))).wrapping_add(8); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
                    if (((((p_top_1) == (((0i32) as u32))) as i32)) != 0) {
                        p_top_1 = p_menu_2;
                    }
                    l_found_3 = { let a1_ = { let a_ = ((c.m.r32(p_menu_2)).wrapping_add((((l_i_4) as u32)).wrapping_mul(48))).wrapping_add(8); c.m.r32(a_) }; F::BtlMenu_FindResult(c, p_top_1, a1_) };
                    if (((((l_found_3) != (((0i32) as u32))) as i32)) != 0) {
                        break 'l1;
                    }
                }
            }
            { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
        }
    }
    break 'f l_found_3;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMenu_FindResult", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMenu_IsSubOpen(c: &mut Ctx, mut p_menu_1: u32, mut p_id_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMenu_IsSubOpen", &PROBE_N, &[(p_menu_1) as u32, (p_id_2) as u32], false); }
    let mut l_item_3: u32 = 0;
    let mut l_owner_4: u32 = 0;
    let r_ = 'f: {
    l_item_3 = F::BtlMenu_FindItem(c, p_menu_1, p_id_2);
    if (((((l_item_3) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if (((((c.m.r32((l_item_3).wrapping_add(8))) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    l_owner_4 = F::BtlMenu_FindOwner(c, p_menu_1, p_id_2);
    if (((((l_owner_4) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    break 'f ((({ let a_ = ((c.m.r32(l_owner_4)).wrapping_add(((((c.m.r32((l_owner_4).wrapping_add(20)) as i32)) as u32)).wrapping_mul(48))).wrapping_add(4); (c.m.r32(a_) as i32) }) == (p_id_2)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMenu_IsSubOpen", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMenu_SetScript(c: &mut Ctx, mut p_text_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMenu_SetScript", &PROBE_N, &[(p_text_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_text_1; let a_ = (c.m.r32(sym::gBtlMenu)).wrapping_add(24); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMenu_SetScript", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMenu_SetScript2(c: &mut Ctx, mut p_text_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMenu_SetScript2", &PROBE_N, &[(p_text_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_text_1; let a_ = (c.m.r32(sym::gBtlMenu)).wrapping_add(28); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMenu_SetScript2", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMenu_GetScript(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMenu_GetScript", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f { let a_ = (c.m.r32(sym::gBtlMenu)).wrapping_add(24); c.m.r32(a_) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMenu_GetScript", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMenu_GetScript2(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMenu_GetScript2", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f { let a_ = (c.m.r32(sym::gBtlMenu)).wrapping_add(28); c.m.r32(a_) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMenu_GetScript2", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMenu_IsActive(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMenu_IsActive", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f { let a_ = (c.m.r32(sym::gBtlMenu)).wrapping_add(8); (c.m.r32(a_) as i32) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMenu_IsActive", &PROBE_N, &[], true); }
    r_
}
