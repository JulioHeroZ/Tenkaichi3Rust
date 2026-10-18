#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn BtlText_DrawControls(c: &mut Ctx, mut p_pkt_1: u32, mut p_x0_2: i32, mut p_x1_3: i32, mut p_y0_4: i32, mut p_y1_5: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlText_DrawControls", &PROBE_N, &[(p_pkt_1) as u32, (p_x0_2) as u32, (p_x1_3) as u32, (p_y0_4) as u32, (p_y1_5) as u32], false); }
    let mut l_config_6: i32 = 0;
    let mut l_padType_7: i32 = 0;
    let r_ = 'f: {
    l_config_6 = { let a_ = (c.m.r32(sym::gProgress)).wrapping_add(2036); (c.m.r32(a_) as i32) };
    l_padType_7 = F::BtlMenu_GetDefaultPadType(c);
    F::Font_PushStyle(c);
    F::Font_SetShadowMode(c, 2i32);
    F::Font_SetSpacing(c, 1i32, 4i32);
    F::BtlMenu_DrawPart(c, p_pkt_1, (p_x0_2).wrapping_add(rt::div_i32((p_x1_3).wrapping_sub(p_x0_2), 2i32)), (p_y0_4).wrapping_add(14i32), 256i32, 32i32, 0i32, 4i32, 1i32);
    F::Font_SetScale(c, F32(0x3f800000));
    F::Font_SetColorRGBA(c, ((((255i32) as i64 as u64)) as u8), ((((255i32) as i64 as u64)) as u8), ((((255i32) as i64 as u64)) as u8), 128i32);
    F::Font_SetShadowColorRGBA(c, ((((0i32) as i64 as u64)) as u8), ((((16i32) as i64 as u64)) as u8), ((((16i32) as i64 as u64)) as u8), 64i32);
    { let a2_ = F::BtlMenu_GetControlsText(c, l_padType_7, l_config_6); F::Font_PrintAt(c, (p_x0_2).wrapping_add(16i32), (p_y0_4).wrapping_add(42i32), a2_) };
    F::Font_PopStyle(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlText_DrawControls", &PROBE_N, &[], true); }
    r_
}

pub fn BtlText_IsOnScreen(c: &mut Ctx, mut p_x0_1: i32, mut p_y0_2: i32, mut p_x1_3: i32, mut p_y1_4: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlText_IsOnScreen", &PROBE_N, &[(p_x0_1) as u32, (p_y0_2) as u32, (p_x1_3) as u32, (p_y1_4) as u32], false); }
    let mut l_result_5: i32 = 0;
    let r_ = 'f: {
    l_result_5 = 0i32;
    if (((((((((((((((((p_x0_1) > (0i32)) as i32)) != 0) && (((((p_x1_3) < (4096i32)) as i32)) != 0)) as i32)) != 0) && (((((p_y0_2) > (0i32)) as i32)) != 0)) as i32)) != 0) && (((((p_y1_4) < (4096i32)) as i32)) != 0)) as i32)) != 0) {
        l_result_5 = 1i32;
    }
    break 'f l_result_5;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlText_IsOnScreen", &PROBE_N, &[], true); }
    r_
}

pub fn BtlText_PutSprite(c: &mut Ctx, mut p_pkt_1: u32, mut p_x0_2: i32, mut p_y0_3: i32, mut p_x1_4: i32, mut p_y1_5: i32, mut p_u0_6: i32, mut p_v0_7: i32, mut p_u1_8: i32, mut p_v1_9: i32, mut p_color_10: u32, mut p_part_11: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlText_PutSprite", &PROBE_N, &[(p_pkt_1) as u32, (p_x0_2) as u32, (p_y0_3) as u32, (p_x1_4) as u32, (p_y1_5) as u32, (p_u0_6) as u32, (p_v0_7) as u32, (p_u1_8) as u32, (p_v1_9) as u32, (p_color_10) as u32, (p_part_11) as u32], false); }
    let r_ = 'f: {
    { let r_ = 1792i32; let n_ = (p_x0_2).wrapping_add(r_); p_x0_2 = n_; };
    { let r_ = 1792i32; let n_ = (p_x1_4).wrapping_add(r_); p_x1_4 = n_; };
    { let r_ = 1824i32; let n_ = (p_y0_3).wrapping_add(r_); p_y0_3 = n_; };
    { let r_ = 1824i32; let n_ = (p_y1_5).wrapping_add(r_); p_y1_5 = n_; };
    if ((F::BtlText_IsOnScreen(c, p_x0_2, p_y0_3, p_x1_4, p_y1_5)) != 0) {
        { let v_ = 9511602413006520321u64; let a_ = (c.m.r32(p_pkt_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((1397954288i32) as u64); let a_ = (c.m.r32(p_pkt_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let o_ = p_y0_3; let n_ = (o_).wrapping_add(1); p_y0_3 = n_; };
        { let o_ = p_y0_3; let n_ = (o_).wrapping_sub(1); p_y0_3 = n_; };
        { let r_ = 2i32; let a_ = p_pkt_1; let n_ = (c.m.r32(a_)).wrapping_add((((r_) as u32)).wrapping_mul(8)); c.m.w32(a_, n_); };
        { let v_ = ((342i32) as u64); let a_ = (c.m.r32(p_pkt_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((0i32) as u64); let a_ = (c.m.r32(p_pkt_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let a_ = p_pkt_1; let n_ = (c.m.r32(a_)).wrapping_add((((r_) as u32)).wrapping_mul(8)); c.m.w32(a_, n_); };
        { let v_ = F::BtlText_GetTex0(c, p_part_11); let a_ = (c.m.r32(p_pkt_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((((p_color_10) as u64)) | (((4575657221408423936i64) as u64))); let a_ = (c.m.r32(p_pkt_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let a_ = p_pkt_1; let n_ = (c.m.r32(a_)).wrapping_add((((r_) as u32)).wrapping_mul(8)); c.m.w32(a_, n_); };
        { let v_ = (((((((p_u0_6) as i64)).wrapping_shl((4i32) as u32)) | ((((p_v0_7) as i64)).wrapping_shl((20i32) as u32)))) as u64); let a_ = (c.m.r32(p_pkt_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = (((((((((p_x0_2) as i64)).wrapping_shl((4i32) as u32)) | ((((p_y0_3) as i64)).wrapping_shl((20i32) as u32)))) as u64)) | (18446744004990074880u64)); let a_ = (c.m.r32(p_pkt_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let a_ = p_pkt_1; let n_ = (c.m.r32(a_)).wrapping_add((((r_) as u32)).wrapping_mul(8)); c.m.w32(a_, n_); };
        { let v_ = (((((((p_u1_8) as i64)).wrapping_shl((4i32) as u32)) | ((((p_v1_9) as i64)).wrapping_shl((20i32) as u32)))) as u64); let a_ = (c.m.r32(p_pkt_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = (((((((((p_x1_4) as i64)).wrapping_shl((4i32) as u32)) | ((((p_y1_5) as i64)).wrapping_shl((20i32) as u32)))) as u64)) | (18446744004990074880u64)); let a_ = (c.m.r32(p_pkt_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let a_ = p_pkt_1; let n_ = (c.m.r32(a_)).wrapping_add((((r_) as u32)).wrapping_mul(8)); c.m.w32(a_, n_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlText_PutSprite", &PROBE_N, &[], true); }
    r_
}

pub fn BtlText_GetTex0(c: &mut Ctx, mut p_part_1: i32) -> u64 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlText_GetTex0", &PROBE_N, &[(p_part_1) as u32], false); }
    let mut l_t_2: u32 = 0;
    let mut l_cbp_3: i32 = 0;
    let mut l_tbp_4: i32 = 0;
    let mut l_tex0_5: u64 = 0;
    let r_ = 'f: {
    l_t_2 = ({ let a_ = ({ let a_ = F::BtlMenu_GetWork(c); c.m.r32(a_) }).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((p_part_1) as u32)).wrapping_mul(64));
    l_cbp_3 = (c.m.r32((l_t_2).wrapping_add(36)) as i32);
    l_tbp_4 = (c.m.r32((l_t_2).wrapping_add(32)) as i32);
    l_tex0_5 = c.m.r64((l_t_2).wrapping_add(48));
    { let r_ = ((((l_cbp_3).wrapping_add(10752i32)) as u64)).wrapping_shl((37i32) as u32); let n_ = ((l_tex0_5) | (r_)); l_tex0_5 = n_; };
    { let r_ = (((l_tbp_4).wrapping_add(10816i32)) as u64); let n_ = ((l_tex0_5) | (r_)); l_tex0_5 = n_; };
    { let r_ = ((17179869184i64) as u64); let n_ = ((l_tex0_5) | (r_)); l_tex0_5 = n_; };
    break 'f l_tex0_5;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlText_GetTex0", &PROBE_N, &[], true); }
    r_
}

pub fn BtlText_PutScissor(c: &mut Ctx, mut p_pkt_1: u32, mut p_x0_2: i32, mut p_y0_3: i32, mut p_x1_4: i32, mut p_y1_5: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlText_PutScissor", &PROBE_N, &[(p_pkt_1) as u32, (p_x0_2) as u32, (p_y0_3) as u32, (p_x1_4) as u32, (p_y1_5) as u32], false); }
    let r_ = 'f: {
    { let v_ = ((1152921504606879745i64) as u64); let a_ = (c.m.r32(p_pkt_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((14i32) as u64); let a_ = (c.m.r32(p_pkt_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let a_ = p_pkt_1; let n_ = (c.m.r32(a_)).wrapping_add((((r_) as u32)).wrapping_mul(8)); c.m.w32(a_, n_); };
    { let v_ = ((((((((((p_x0_2) as i64)) | ((((p_x1_4) as i64)).wrapping_shl((16i32) as u32)))) | ((((p_y0_3) as i64)).wrapping_shl((32i32) as u32)))) | ((((p_y1_5) as i64)).wrapping_shl((48i32) as u32)))) as u64); let a_ = (c.m.r32(p_pkt_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((64i32) as u64); let a_ = (c.m.r32(p_pkt_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let a_ = p_pkt_1; let n_ = (c.m.r32(a_)).wrapping_add((((r_) as u32)).wrapping_mul(8)); c.m.w32(a_, n_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlText_PutScissor", &PROBE_N, &[], true); }
    r_
}

pub fn BtlText_IsNewline(c: &mut Ctx, mut p_c_1: u16) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlText_IsNewline", &PROBE_N, &[(p_c_1) as u32], false); }
    let mut l_result_2: i32 = 0;
    let r_ = 'f: {
    l_result_2 = 0i32;
    if (((((((((((p_c_1) as i32)) == (10i32)) as i32)) != 0) || (((((((p_c_1) as i32)) == (13i32)) as i32)) != 0)) as i32)) != 0) {
        l_result_2 = 1i32;
    }
    break 'f l_result_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlText_IsNewline", &PROBE_N, &[], true); }
    r_
}

pub fn BtlText_NextLine(c: &mut Ctx, mut p_p_1: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlText_NextLine", &PROBE_N, &[(p_p_1) as u32], false); }
    let r_ = 'f: {
    'l1: while (((((((c.m.r16(p_p_1)) as i32)) != (0i32)) as i32)) != 0) {
        if (({ let a0_ = c.m.r16(p_p_1); F::BtlText_IsNewline(c, a0_) }) != 0) {
            { let o_ = p_p_1; let n_ = (o_).wrapping_add(2); p_p_1 = n_; };
            break 'l1;
        }
        { let o_ = p_p_1; let n_ = (o_).wrapping_add(2); p_p_1 = n_; };
    }
    break 'f p_p_1;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlText_NextLine", &PROBE_N, &[], true); }
    r_
}
