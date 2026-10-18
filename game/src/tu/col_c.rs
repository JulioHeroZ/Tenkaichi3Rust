#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn Font_IsInGsRange(c: &mut Ctx, mut p_x0_1: i32, mut p_y0_2: i32, mut p_x1_3: i32, mut p_y1_4: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Font_IsInGsRange", &PROBE_N, &[(p_x0_1) as u32, (p_y0_2) as u32, (p_x1_3) as u32, (p_y1_4) as u32], false); }
    let r_ = 'f: {
    break 'f (((((((((((((((p_x0_1) > (0i32)) as i32)) != 0) && (((((p_x1_3) < (4096i32)) as i32)) != 0)) as i32)) != 0) && (((((p_y0_2) > (0i32)) as i32)) != 0)) as i32)) != 0) && (((((p_y1_4) < (4096i32)) as i32)) != 0)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Font_IsInGsRange", &PROBE_N, &[], true); }
    r_
}

pub fn Font_IsInClip(c: &mut Ctx, mut p_clip_1: u32, mut p_x0_2: i32, mut p_y0_3: i32, mut p_x1_4: i32, mut p_y1_5: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Font_IsInClip", &PROBE_N, &[(p_clip_1) as u32, (p_x0_2) as u32, (p_y0_3) as u32, (p_x1_4) as u32, (p_y1_5) as u32], false); }
    let r_ = 'f: {
    break 'f (((((((((((((((((c.m.r16((p_clip_1).wrapping_add(48))) as i32)) < (p_x1_4)) as i32)) != 0) && (((((p_x0_2) < (((c.m.r16((p_clip_1).wrapping_add(52))) as i32))) as i32)) != 0)) as i32)) != 0) && (((((((c.m.r16((p_clip_1).wrapping_add(50))) as i32)) < (p_y1_5)) as i32)) != 0)) as i32)) != 0) && (((((p_y0_3) < (((c.m.r16((p_clip_1).wrapping_add(54))) as i32))) as i32)) != 0)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Font_IsInClip", &PROBE_N, &[], true); }
    r_
}

pub fn Font_BeginPacket(c: &mut Ctx, mut p_out_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Font_BeginPacket", &PROBE_N, &[(p_out_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = F::Dma_BeginDirect(c); let a_ = p_out_1; c.m.w32(a_, v_); };
    F::Gfx_PutDefaultEnv(c, p_out_1);
    { let v_ = ((1152921504606879749i64) as u64); let a_ = (c.m.r32(p_out_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((14i32) as u64); let a_ = (c.m.r32(p_out_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let a_ = p_out_1; let n_ = (c.m.r32(a_)).wrapping_add((((r_) as u32)).wrapping_mul(8)); c.m.w32(a_, n_); };
    { let v_ = ((0i32) as u64); let a_ = (c.m.r32(p_out_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((8i32) as u64); let a_ = (c.m.r32(p_out_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let a_ = p_out_1; let n_ = (c.m.r32(a_)).wrapping_add((((r_) as u32)).wrapping_mul(8)); c.m.w32(a_, n_); };
    { let v_ = ((0i32) as u64); let a_ = (c.m.r32(p_out_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((71i32) as u64); let a_ = (c.m.r32(p_out_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let a_ = p_out_1; let n_ = (c.m.r32(a_)).wrapping_add((((r_) as u32)).wrapping_mul(8)); c.m.w32(a_, n_); };
    { let v_ = ((549755846784i64) as u64); let a_ = (c.m.r32(p_out_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((59i32) as u64); let a_ = (c.m.r32(p_out_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let a_ = p_out_1; let n_ = (c.m.r32(a_)).wrapping_add((((r_) as u32)).wrapping_mul(8)); c.m.w32(a_, n_); };
    { let v_ = ((5117051104i64) as u64); let a_ = (c.m.r32(p_out_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((78i32) as u64); let a_ = (c.m.r32(p_out_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let a_ = p_out_1; let n_ = (c.m.r32(a_)).wrapping_add((((r_) as u32)).wrapping_mul(8)); c.m.w32(a_, n_); };
    { let v_ = ((342i32) as u64); let a_ = (c.m.r32(p_out_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((0i32) as u64); let a_ = (c.m.r32(p_out_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let a_ = p_out_1; let n_ = (c.m.r32(a_)).wrapping_add((((r_) as u32)).wrapping_mul(8)); c.m.w32(a_, n_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Font_BeginPacket", &PROBE_N, &[], true); }
    r_
}

pub fn Font_EndPacket(c: &mut Ctx, mut p_out_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Font_EndPacket", &PROBE_N, &[(p_out_1) as u32], false); }
    let r_ = 'f: {
    { let a0_ = c.m.r32(p_out_1); F::Dma_EndDirect(c, a0_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Font_EndPacket", &PROBE_N, &[], true); }
    r_
}

pub fn Font_PutSprite(c: &mut Ctx, mut p_out_1: u32, mut p_x0_2: i32, mut p_y0_3: i32, mut p_x1_4: i32, mut p_y1_5: i32, mut p_u0_6: i32, mut p_v0_7: i32, mut p_u1_8: i32, mut p_v1_9: i32, mut p_rgba_10: u32, mut p_tex0_11: u64) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Font_PutSprite", &PROBE_N, &[(p_out_1) as u32, (p_x0_2) as u32, (p_y0_3) as u32, (p_x1_4) as u32, (p_y1_5) as u32, (p_u0_6) as u32, (p_v0_7) as u32, (p_u1_8) as u32, (p_v1_9) as u32, (p_rgba_10) as u32, (p_tex0_11) as u32], false); }
    let r_ = 'f: {
    { let r_ = 1792i32; let n_ = (p_x0_2).wrapping_add(r_); p_x0_2 = n_; };
    { let r_ = 1792i32; let n_ = (p_x1_4).wrapping_add(r_); p_x1_4 = n_; };
    { let r_ = 1824i32; let n_ = (p_y0_3).wrapping_add(r_); p_y0_3 = n_; };
    { let r_ = 1824i32; let n_ = (p_y1_5).wrapping_add(r_); p_y1_5 = n_; };
    if ((F::Font_IsInGsRange(c, p_x0_2, p_y0_3, p_x1_4, p_y1_5)) != 0) {
        { let v_ = p_tex0_11; let a_ = (c.m.r32(p_out_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((((p_rgba_10) as u64)) | ((((1065353216i32) as u64)).wrapping_shl((32i32) as u32))); let a_ = (c.m.r32(p_out_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let a_ = p_out_1; let n_ = (c.m.r32(a_)).wrapping_add((((r_) as u32)).wrapping_mul(8)); c.m.w32(a_, n_); };
        { let v_ = ((((((p_u0_6).wrapping_mul(16i32)).wrapping_add(8i32)) as u64)) | ((((((p_v0_7).wrapping_mul(16i32)).wrapping_add(8i32)) as u64)).wrapping_shl((16i32) as u32))); let a_ = (c.m.r32(p_out_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = (((((((p_x0_2) as u64)).wrapping_shl((4i32) as u32)) | ((((p_y0_3) as u64)).wrapping_shl((20i32) as u32)))) | ((((268435440i32) as u64)).wrapping_shl((32i32) as u32))); let a_ = (c.m.r32(p_out_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let a_ = p_out_1; let n_ = (c.m.r32(a_)).wrapping_add((((r_) as u32)).wrapping_mul(8)); c.m.w32(a_, n_); };
        { let v_ = ((((((p_u1_8).wrapping_mul(16i32)).wrapping_add(8i32)) as u64)) | ((((((p_v1_9).wrapping_mul(16i32)).wrapping_add(8i32)) as u64)).wrapping_shl((16i32) as u32))); let a_ = (c.m.r32(p_out_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = (((((((p_x1_4) as u64)).wrapping_shl((4i32) as u32)) | ((((p_y1_5) as u64)).wrapping_shl((20i32) as u32)))) | ((((268435440i32) as u64)).wrapping_shl((32i32) as u32))); let a_ = (c.m.r32(p_out_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let a_ = p_out_1; let n_ = (c.m.r32(a_)).wrapping_add((((r_) as u32)).wrapping_mul(8)); c.m.w32(a_, n_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Font_PutSprite", &PROBE_N, &[], true); }
    r_
}

pub fn Font_PutScissor(c: &mut Ctx, mut p_pkt_1: u32, mut p_cmd_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Font_PutScissor", &PROBE_N, &[(p_pkt_1) as u32, (p_cmd_2) as u32], false); }
    let mut l_x0_3: i32 = 0;
    let mut l_y0_4: i32 = 0;
    let mut l_x1_5: i32 = 0;
    let mut l_y1_6: i32 = 0;
    let r_ = 'f: {
    l_x0_3 = ((c.m.r16((p_cmd_2).wrapping_add(48))) as i32);
    l_y0_4 = ((c.m.r16((p_cmd_2).wrapping_add(50))) as i32);
    l_x1_5 = (((c.m.r16((p_cmd_2).wrapping_add(52))) as i32)).wrapping_sub(1i32);
    l_y1_6 = (((c.m.r16((p_cmd_2).wrapping_add(54))) as i32)).wrapping_sub(1i32);
    { let v_ = ((1152921504606879745i64) as u64); let a_ = (c.m.r32(p_pkt_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((14i32) as u64); let a_ = (c.m.r32(p_pkt_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let a_ = p_pkt_1; let n_ = (c.m.r32(a_)).wrapping_add((((r_) as u32)).wrapping_mul(8)); c.m.w32(a_, n_); };
    { let v_ = ((((((((l_x0_3) as u64)) | ((((l_x1_5) as u64)).wrapping_shl((16i32) as u32)))) | ((((l_y0_4) as u64)).wrapping_shl((32i32) as u32)))) | ((((l_y1_6) as u64)).wrapping_shl((48i32) as u32))); let a_ = (c.m.r32(p_pkt_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((64i32) as u64); let a_ = (c.m.r32(p_pkt_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let a_ = p_pkt_1; let n_ = (c.m.r32(a_)).wrapping_add((((r_) as u32)).wrapping_mul(8)); c.m.w32(a_, n_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Font_PutScissor", &PROBE_N, &[], true); }
    r_
}

pub fn Font_OpenSpriteTag(c: &mut Ctx, mut p_pkt_1: u32, mut p_tag_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Font_OpenSpriteTag", &PROBE_N, &[(p_pkt_1) as u32, (p_tag_2) as u32], false); }
    let r_ = 'f: {
    { let v_ = c.m.r32(p_pkt_1); let a_ = p_tag_2; c.m.w32(a_, v_); };
    { let v_ = ((7205759403792826368i64) as u64); let a_ = (c.m.r32(p_pkt_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((5460758i32) as u64); let a_ = (c.m.r32(p_pkt_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let a_ = p_pkt_1; let n_ = (c.m.r32(a_)).wrapping_add((((r_) as u32)).wrapping_mul(8)); c.m.w32(a_, n_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Font_OpenSpriteTag", &PROBE_N, &[], true); }
    r_
}

pub fn Font_CloseSpriteTag(c: &mut Ctx, mut p_pkt_1: u32, mut p_tag_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Font_CloseSpriteTag", &PROBE_N, &[(p_pkt_1) as u32, (p_tag_2) as u32], false); }
    let r_ = 'f: {
    { let v_ = ((rt::div_i32(((((c.m.r32(p_pkt_1)).wrapping_sub(c.m.r32(p_tag_2)) as i32)).wrapping_div(8)).wrapping_sub(2i32), 6i32)) as u16); let a_ = c.m.r32(p_tag_2); c.m.w16(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Font_CloseSpriteTag", &PROBE_N, &[], true); }
    r_
}

pub fn Font_MakeTex0(c: &mut Ctx, mut p_slot_1: u32, mut p_clut_2: i32) -> u64 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Font_MakeTex0", &PROBE_N, &[(p_slot_1) as u32, (p_clut_2) as u32], false); }
    let mut l_tex0_3: u64 = 0;
    let r_ = 'f: {
    l_tex0_3 = { let a_ = ({ let a_ = (c.m.r32(p_slot_1)).wrapping_add(12); c.m.r32(a_) }).wrapping_add(48); c.m.r64(a_) };
    { let r_ = (((((c.m.r32((p_slot_1).wrapping_add(8)) as i32)).wrapping_add(p_clut_2)) as u64)).wrapping_shl((37i32) as u32); let n_ = ((l_tex0_3) | (r_)); l_tex0_3 = n_; };
    { let r_ = ((c.m.r32((p_slot_1).wrapping_add(4))) as u64); let n_ = ((l_tex0_3) | (r_)); l_tex0_3 = n_; };
    break 'f ((l_tex0_3) | (((17179869184i64) as u64)));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Font_MakeTex0", &PROBE_N, &[], true); }
    r_
}

pub fn Font_UploadTexture(c: &mut Ctx, mut p_slot_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Font_UploadTexture", &PROBE_N, &[(p_slot_1) as u32], false); }
    let r_ = 'f: {
    { let a0_ = { let a_ = (c.m.r32(p_slot_1)).wrapping_add(12); c.m.r32(a_) }; let a1_ = c.m.r32((p_slot_1).wrapping_add(4)); let a2_ = (c.m.r32((p_slot_1).wrapping_add(8)) as i32); F::Tex_Upload(c, a0_, ((((a1_) as u64)) as i32), a2_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Font_UploadTexture", &PROBE_N, &[], true); }
    r_
}

pub fn Font_Init(c: &mut Ctx, mut p_unused_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Font_Init", &PROBE_N, &[(p_unused_1) as u32], false); }
    let r_ = 'f: {
    F::Font_InitEx(c, p_unused_1, 120i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Font_Init", &PROBE_N, &[], true); }
    r_
}

pub fn Font_InitEx(c: &mut Ctx, mut p_unused_1: i32, mut p_maxCmds_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Font_InitEx", &PROBE_N, &[(p_unused_1) as u32, (p_maxCmds_2) as u32], false); }
    let mut l_boot_3: u32 = 0;
    let r_ = 'f: {
    { let v_ = 1i32; let a_ = sym::gFontReady; c.m.w32(a_, (v_) as u32); };
    { let v_ = 1i32; let a_ = sym::gFontEnabled; c.m.w32(a_, (v_) as u32); };
    { let v_ = 16i32; let a_ = sym::gFontCursor; c.m.w32(a_, (v_) as u32); };
    { let v_ = 16i32; let a_ = (sym::gFontCursor).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
    { let v_ = 16i32; let a_ = (sym::gFontCursor).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
    { let v_ = 16i32; let a_ = (sym::gFontCursor).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
    { let v_ = F32(0x3f800000); let a_ = sym::gFontStyle; c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x3f800000); let a_ = (sym::gFontStyle).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x3f800000); let a_ = (sym::gFontStyle).wrapping_add(8); c.m.w32(a_, (v_).0); };
    { let v_ = 2164260863u32; let a_ = (sym::gFontStyle).wrapping_add(12); c.m.w32(a_, v_); };
    { let v_ = 2155905152u32; let a_ = (sym::gFontStyle).wrapping_add(16); c.m.w32(a_, v_); };
    { let v_ = 0i32; let a_ = (sym::gFontStyle).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
    { let v_ = 0i32; let a_ = (sym::gFontStyle).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
    { let v_ = ((0i32) as u32); let a_ = (sym::gFontStyle).wrapping_add(28); c.m.w32(a_, v_); };
    { let v_ = 0i32; let a_ = (sym::gFontStyle).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
    { let v_ = ((0i32) as u32); let a_ = (sym::gFontStyle).wrapping_add(36); c.m.w32(a_, v_); };
    { let v_ = 2155905152u32; let a_ = (sym::gFontStyle).wrapping_add(40); c.m.w32(a_, v_); };
    { let v_ = ((1i32) as u16); let a_ = (sym::gFontStyle).wrapping_add(44); c.m.w16(a_, v_); };
    { let v_ = ((2i32) as u16); let a_ = (sym::gFontStyle).wrapping_add(46); c.m.w16(a_, v_); };
    { let v_ = ((0i32) as u16); let a_ = (sym::gFontStyle).wrapping_add(48); c.m.w16(a_, v_); };
    { let v_ = ((0i32) as u16); let a_ = (sym::gFontStyle).wrapping_add(50); c.m.w16(a_, v_); };
    { let v_ = ((511i32) as u16); let a_ = (sym::gFontStyle).wrapping_add(52); c.m.w16(a_, v_); };
    { let v_ = ((447i32) as u16); let a_ = (sym::gFontStyle).wrapping_add(54); c.m.w16(a_, v_); };
    { let v_ = 0i32; let a_ = (sym::gFontStyle).wrapping_add(56); c.m.w32(a_, (v_) as u32); };
    { let v_ = ((0i32) as u32); let a_ = (sym::gFontStyle).wrapping_add(60); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (sym::gFontStyle).wrapping_add(64); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (sym::gFontStyle).wrapping_add(68); c.m.w32(a_, v_); };
    { let v_ = F::Heap_Alloc(c, (((((7i32) as u32)).wrapping_mul(20u32)) as i32), ((32i32) as u32), 0i32, 2i32); let a_ = sym::gFontSlots; c.m.w32(a_, v_); };
    { let v_ = F::Heap_Alloc(c, (((((5i32) as u32)).wrapping_mul(16u32)) as i32), ((32i32) as u32), 0i32, 2i32); let a_ = sym::gFontCursorStack; c.m.w32(a_, v_); };
    { let v_ = F::Heap_Alloc(c, (((((5i32) as u32)).wrapping_mul(72u32)) as i32), ((32i32) as u32), 0i32, 2i32); let a_ = sym::gFontStyleStack; c.m.w32(a_, v_); };
    { let v_ = F::Heap_Alloc(c, (((((p_maxCmds_2) as u32)).wrapping_mul(84u32)) as i32), ((32i32) as u32), 0i32, 2i32); let a_ = sym::gFontCmds; c.m.w32(a_, v_); };
    { let v_ = 0i32; let a_ = sym::gFontCursorDepth; c.m.w32(a_, (v_) as u32); };
    { let v_ = p_maxCmds_2; let a_ = sym::gFontCmdMax; c.m.w32(a_, (v_) as u32); };
    { let v_ = 0i32; let a_ = sym::gFontStyleDepth; c.m.w32(a_, (v_) as u32); };
    { let v_ = 0i32; let a_ = sym::gFontCmdCount; c.m.w32(a_, (v_) as u32); };
    { let _ = { let a0_ = c.m.r32(sym::gFontSlots); F::memset(c, a0_, 0i32, (((7i32) as u32)).wrapping_mul(20u32)) }; };
    l_boot_3 = { let a_ = c.m.r32(sym::gCommonRes); c.m.r32(a_) };
    { let a1_ = (l_boot_3).wrapping_add(((c.m.r32((l_boot_3).wrapping_add((((1i32) as u32)).wrapping_mul(4)))).wrapping_shr((2i32) as u32)).wrapping_mul(4)); F::Font_SetFont(c, 0i32, a1_) };
    { let a1_ = (l_boot_3).wrapping_add(((c.m.r32((l_boot_3).wrapping_add((((2i32) as u32)).wrapping_mul(4)))).wrapping_shr((2i32) as u32)).wrapping_mul(4)); F::Font_SetFont(c, 1i32, a1_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Font_InitEx", &PROBE_N, &[], true); }
    r_
}

pub fn Font_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Font_Term", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if (((((c.m.r32(sym::gFontSlots)) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32(sym::gFontSlots); F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = sym::gFontSlots; c.m.w32(a_, v_); };
    }
    if (((((c.m.r32(sym::gFontCursorStack)) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32(sym::gFontCursorStack); F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = sym::gFontCursorStack; c.m.w32(a_, v_); };
    }
    if (((((c.m.r32(sym::gFontStyleStack)) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32(sym::gFontStyleStack); F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = sym::gFontStyleStack; c.m.w32(a_, v_); };
    }
    if (((((c.m.r32(sym::gFontCmds)) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32(sym::gFontCmds); F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = sym::gFontCmds; c.m.w32(a_, v_); };
    }
    { let v_ = 0i32; let a_ = sym::gFontReady; c.m.w32(a_, (v_) as u32); };
    { let v_ = 0i32; let a_ = sym::gFontEnabled; c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Font_Term", &PROBE_N, &[], true); }
    r_
}

pub fn Font_DrawCharNow(c: &mut Ctx, mut p_pkt_1: u32, mut p_cmd_2: u32, mut p_x_3: u32, mut p_y_4: u32, mut p_ch_5: u16) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Font_DrawCharNow", &PROBE_N, &[(p_pkt_1) as u32, (p_cmd_2) as u32, (p_x_3) as u32, (p_y_4) as u32, (p_ch_5) as u32], false); }
    let r_ = 'f: {
    if (((((((((((p_ch_5) as i32)) != (0i32)) as i32)) != 0) && ((((!((F::Font_IsNewline(c, p_ch_5)) != 0)) as i32)) != 0)) as i32)) != 0) {
        break 'f F::Font_DrawChar(c, p_pkt_1, p_cmd_2, p_x_3, p_y_4, p_ch_5, ((0i32) as u32));
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Font_DrawCharNow", &PROBE_N, &[], true); }
    r_
}

pub fn Font_FlushAll(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Font_FlushAll", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::Font_Flush(c, 0i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Font_FlushAll", &PROBE_N, &[], true); }
    r_
}

pub fn Font_Flush(c: &mut Ctx, mut p_first_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Font_Flush", &PROBE_N, &[(p_first_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_vram_2: i32 = 0;
    let mut l_slot_3: u32 = 0;
    let mut l_cmd_4: u32 = 0;
    let mut l_i_5: i32 = 0;
    let r_ = 'f: {
        l_vram_2 = 14392i32;
        if (((((((((((c.m.r32(sym::gFontReady) as i32)) != 0) && (((c.m.r32(sym::gFontEnabled) as i32)) != 0)) as i32)) != 0) && (((((p_first_1) < ((c.m.r32(sym::gFontCmdCount) as i32))) as i32)) != 0)) as i32)) != 0) {
            l_slot_3 = c.m.r32(sym::gFontSlots);
            {
                l_i_5 = 0i32;
                'l1: loop {
                    if !(((((l_i_5) < (7i32)) as i32)) != 0) { break 'l1; }
                    'c2: {
                        if (((((c.m.r32(l_slot_3)) != (((0i32) as u32))) as i32)) != 0) {
                            F::Font_UploadTexture(c, l_slot_3);
                        }
                    }
                    { { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; }; { let o_ = l_slot_3; let n_ = (o_).wrapping_add(20); l_slot_3 = n_; }; };
                }
            }
            l_cmd_4 = (c.m.r32(sym::gFontCmds)).wrapping_add((((p_first_1) as u32)).wrapping_mul(84));
            {
                l_i_5 = p_first_1;
                'l3: loop {
                    if !(((((l_i_5) < ((c.m.r32(sym::gFontCmdCount) as i32))) as i32)) != 0) { break 'l3; }
                    'c4: {
                        if (((((c.m.r32((l_cmd_4).wrapping_add(68))) != (((0i32) as u32))) as i32)) != 0) {
                            { let v_ = l_vram_2; let a_ = (l_cmd_4).wrapping_add(76); c.m.w32(a_, (v_) as u32); };
                            { let r_ = { let fp_ = c.m.r32((l_cmd_4).wrapping_add(68)); ((D::call(c, fp_, &[((l_vram_2) as i64 as u64)])) as i32) }; let n_ = (l_vram_2).wrapping_add(r_); l_vram_2 = n_; };
                        }
                    }
                    { { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; }; { let o_ = l_cmd_4; let n_ = (o_).wrapping_add(84); l_cmd_4 = n_; }; };
                }
            }
            { let _ = D::call(c, fid::Font_BeginPacket, &[((fp) as u64)]); };
            l_cmd_4 = (c.m.r32(sym::gFontCmds)).wrapping_add((((p_first_1) as u32)).wrapping_mul(84));
            {
                l_i_5 = p_first_1;
                'l5: loop {
                    if !(((((l_i_5) < ((c.m.r32(sym::gFontCmdCount) as i32))) as i32)) != 0) { break 'l5; }
                    'c6: {
                        F::Font_PutScissor(c, fp, l_cmd_4);
                        F::Font_OpenSpriteTag(c, fp, fp.wrapping_add(4));
                        F::Font_DrawCmd(c, fp, l_cmd_4);
                        F::Font_CloseSpriteTag(c, fp, fp.wrapping_add(4));
                    }
                    { { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; }; { let o_ = l_cmd_4; let n_ = (o_).wrapping_add(84); l_cmd_4 = n_; }; };
                }
            }
            { let _ = D::call(c, fid::Font_EndPacket, &[((fp) as u64)]); };
            F::Font_Home(c);
            F::Font_SetCmdCount(c, p_first_1);
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Font_Flush", &PROBE_N, &[], true); }
    r_
}

pub fn Font_GetCmdCount(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Font_GetCmdCount", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f (c.m.r32(sym::gFontCmdCount) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Font_GetCmdCount", &PROBE_N, &[], true); }
    r_
}

pub fn Font_SetEnabled(c: &mut Ctx, mut p_enabled_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Font_SetEnabled", &PROBE_N, &[(p_enabled_1) as u32], false); }
    let r_ = 'f: {
    if (((c.m.r32(sym::gFontReady) as i32)) != 0) {
        { let v_ = p_enabled_1; let a_ = sym::gFontEnabled; c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Font_SetEnabled", &PROBE_N, &[], true); }
    r_
}

pub fn Font_IsEnabled(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Font_IsEnabled", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f (c.m.r32(sym::gFontEnabled) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Font_IsEnabled", &PROBE_N, &[], true); }
    r_
}

pub fn Font_GetLineHeight(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Font_GetLineHeight", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f (c.m.r32(sym::gFontLineHeight) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Font_GetLineHeight", &PROBE_N, &[], true); }
    r_
}

pub fn Font_GetGlyphHeight(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Font_GetGlyphHeight", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f (c.m.r32(sym::gFontLineHeight) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Font_GetGlyphHeight", &PROBE_N, &[], true); }
    r_
}

pub fn Font_MeasureWidthEx(c: &mut Ctx, mut p_str_1: u32, mut p_kind_2: i32, mut p_scale_3: F32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Font_MeasureWidthEx", &PROBE_N, &[(p_str_1) as u32, (p_kind_2) as u32, (p_scale_3).0], false); }
    let mut l_style_4: u32 = 0;
    let mut l_max_5: i32 = 0;
    let mut l_width_6: i32 = 0;
    let mut l_gap_7: i32 = 0;
    let mut l_ch_8: u16 = 0;
    let mut l_glyph_9: u32 = 0;
    let r_ = 'f: {
    l_style_4 = sym::gFontStyle;
    l_max_5 = 0i32;
    l_width_6 = 0i32;
    l_gap_7 = ((c.m.r32(sym::gFontGlyphGap) as i32)).wrapping_add((c.m.r32((l_style_4).wrapping_add(20)) as i32));
    'l1: while ((1i32) != 0) {
        l_ch_8 = F::Font_ReadChar(c, p_kind_2, p_str_1);
        p_str_1 = (p_str_1).wrapping_add(((F::Font_CharSize(c, p_kind_2)) as u32));
        if (((((((l_ch_8) as i32)) == (0i32)) as i32)) != 0) {
            break 'l1;
        }
        if ((F::Font_IsNewline(c, l_ch_8)) != 0) {
            if (((((c.m.r32((l_style_4).wrapping_add(32)) as i32)) & (1i32))) != 0) {
                break 'l1;
            }
            l_width_6 = 0i32;
        } else {
            l_glyph_9 = F::Font_FindGlyph(c, l_ch_8, ((0i32) as u32));
            if (((((l_glyph_9) != (((0i32) as u32))) as i32)) != 0) {
                { let r_ = (rt::f2i(rt::fmul(rt::fmul(rt::fmul(rt::i2f((((((c.m.r16((l_glyph_9).wrapping_add(2))) as i32)).wrapping_shr((10i32) as u32)) & (31i32))), F32(c.m.r32(l_style_4))), p_scale_3), F32(c.m.r32((l_style_4).wrapping_add(8)))))).wrapping_add(l_gap_7); let n_ = (l_width_6).wrapping_add(r_); l_width_6 = n_; };
                if (((((l_max_5) < (l_width_6)) as i32)) != 0) {
                    l_max_5 = l_width_6;
                }
            }
        }
    }
    break 'f (l_max_5).wrapping_sub(l_gap_7);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Font_MeasureWidthEx", &PROBE_N, &[], true); }
    r_
}

pub fn Font_GetWidth(c: &mut Ctx, mut p_str_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Font_GetWidth", &PROBE_N, &[(p_str_1) as u32], false); }
    let r_ = 'f: {
    break 'f F::Font_MeasureWidthEx(c, p_str_1, 0i32, F32(0x3f800000));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Font_GetWidth", &PROBE_N, &[], true); }
    r_
}

pub fn Font_GetWidthAscii(c: &mut Ctx, mut p_str_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Font_GetWidthAscii", &PROBE_N, &[(p_str_1) as u32], false); }
    let r_ = 'f: {
    break 'f F::Font_MeasureWidthEx(c, p_str_1, 1i32, F32(0x3f800000));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Font_GetWidthAscii", &PROBE_N, &[], true); }
    r_
}

pub fn Font_FitScale(c: &mut Ctx, mut p_str_1: u32, mut p_width_2: i32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Font_FitScale", &PROBE_N, &[(p_str_1) as u32, (p_width_2) as u32], false); }
    let mut l_step_3: F32 = F32(0);
    let mut l_scale_4: F32 = F32(0);
    let r_ = 'f: {
    l_step_3 = F32(0x3c23d70a);
    l_scale_4 = F32(0x3f800000);
    'l1: while (((rt::flt(l_step_3, l_scale_4) as i32)) != 0) {
        if (((((F::Font_MeasureWidthEx(c, p_str_1, 0i32, l_scale_4)) < (p_width_2)) as i32)) != 0) {
            break 'l1;
        }
        { let r_ = l_step_3; let n_ = rt::fsub(l_scale_4, r_); l_scale_4 = n_; };
    }
    break 'f l_scale_4;
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "Font_FitScale", &PROBE_N, &[], true); }
    r_
}

pub fn Font_CountLinesEx(c: &mut Ctx, mut p_str_1: u32, mut p_kind_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Font_CountLinesEx", &PROBE_N, &[(p_str_1) as u32, (p_kind_2) as u32], false); }
    let mut l_lines_3: i32 = 0;
    let mut l_ch_4: u16 = 0;
    let r_ = 'f: {
    l_lines_3 = 1i32;
    {
        'l1: loop {
            'c2: {
                l_ch_4 = F::Font_ReadChar(c, p_kind_2, p_str_1);
                p_str_1 = (p_str_1).wrapping_add(((F::Font_CharSize(c, p_kind_2)) as u32));
                if (((((((l_ch_4) as i32)) == (0i32)) as i32)) != 0) {
                    break 'l1;
                }
                if ((F::Font_IsNewline(c, l_ch_4)) != 0) {
                    { let o_ = l_lines_3; let n_ = (o_).wrapping_add(1); l_lines_3 = n_; };
                }
            }
        }
    }
    break 'f l_lines_3;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Font_CountLinesEx", &PROBE_N, &[], true); }
    r_
}

pub fn Font_GetHeight(c: &mut Ctx, mut p_str_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Font_GetHeight", &PROBE_N, &[(p_str_1) as u32], false); }
    let mut l_style_2: u32 = 0;
    let mut l_lines_3: i32 = 0;
    let r_ = 'f: {
    l_style_2 = sym::gFontStyle;
    l_lines_3 = F::Font_CountLinesEx(c, p_str_1, 0i32);
    break 'f ((l_lines_3).wrapping_mul((rt::f2i(rt::fmul(rt::fmul(rt::i2f(F::Font_GetGlyphHeight(c)), F32(c.m.r32((l_style_2).wrapping_add(4)))), F32(c.m.r32((l_style_2).wrapping_add(8)))))).wrapping_add((c.m.r32((l_style_2).wrapping_add(24)) as i32)))).wrapping_sub((c.m.r32((l_style_2).wrapping_add(24)) as i32));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Font_GetHeight", &PROBE_N, &[], true); }
    r_
}

pub fn Font_GetHeightAscii(c: &mut Ctx, mut p_str_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Font_GetHeightAscii", &PROBE_N, &[(p_str_1) as u32], false); }
    let mut l_lines_2: i32 = 0;
    let r_ = 'f: {
    l_lines_2 = F::Font_CountLinesEx(c, p_str_1, 1i32);
    break 'f (l_lines_2).wrapping_mul(F::Font_GetGlyphHeight(c));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Font_GetHeightAscii", &PROBE_N, &[], true); }
    r_
}

pub fn Font_CountLines(c: &mut Ctx, mut p_str_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Font_CountLines", &PROBE_N, &[(p_str_1) as u32], false); }
    let r_ = 'f: {
    break 'f F::Font_CountLinesEx(c, p_str_1, 0i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Font_CountLines", &PROBE_N, &[], true); }
    r_
}

pub fn Font_CountLinesAscii(c: &mut Ctx, mut p_str_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Font_CountLinesAscii", &PROBE_N, &[(p_str_1) as u32], false); }
    let r_ = 'f: {
    break 'f F::Font_CountLinesEx(c, p_str_1, 1i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Font_CountLinesAscii", &PROBE_N, &[], true); }
    r_
}

pub fn Font_AddCmd(c: &mut Ctx, mut p_arg_1: i32, mut p_digits_2: i32, mut p_kind_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Font_AddCmd", &PROBE_N, &[(p_arg_1) as u32, (p_digits_2) as u32, (p_kind_3) as u32], false); }
    let r_ = 'f: {
    { let a0_ = { let a2_ = (c.m.r32((sym::gFontCursor).wrapping_add(8)) as i32); let a3_ = (c.m.r32((sym::gFontCursor).wrapping_add(12)) as i32); let a4_ = (c.m.r32(sym::gFontCursor) as i32); F::Font_NewCmd(c, p_arg_1, p_digits_2, a2_, a3_, a4_, p_kind_3) }; F::Font_AdvanceCursor(c, a0_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Font_AddCmd", &PROBE_N, &[], true); }
    r_
}

pub fn Font_AddCmdAt(c: &mut Ctx, mut p_x_1: i32, mut p_y_2: i32, mut p_arg_3: i32, mut p_digits_4: i32, mut p_kind_5: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Font_AddCmdAt", &PROBE_N, &[(p_x_1) as u32, (p_y_2) as u32, (p_arg_3) as u32, (p_digits_4) as u32, (p_kind_5) as u32], false); }
    let r_ = 'f: {
    break 'f F::Font_NewCmd(c, p_arg_3, p_digits_4, p_x_1, p_y_2, p_x_1, p_kind_5);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Font_AddCmdAt", &PROBE_N, &[], true); }
    r_
}

pub fn Font_Print(c: &mut Ctx, mut p_str_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Font_Print", &PROBE_N, &[(p_str_1) as u32], false); }
    let r_ = 'f: {
    F::Font_AddCmd(c, ((p_str_1) as i32), 0i32, 0i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Font_Print", &PROBE_N, &[], true); }
    r_
}

pub fn Font_PrintAt(c: &mut Ctx, mut p_x_1: i32, mut p_y_2: i32, mut p_str_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Font_PrintAt", &PROBE_N, &[(p_x_1) as u32, (p_y_2) as u32, (p_str_3) as u32], false); }
    let r_ = 'f: {
    { let _ = F::Font_AddCmdAt(c, p_x_1, p_y_2, ((p_str_3) as i32), 0i32, 0i32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Font_PrintAt", &PROBE_N, &[], true); }
    r_
}

pub fn Font_PutChar(c: &mut Ctx, mut p_ch_1: u16) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Font_PutChar", &PROBE_N, &[(p_ch_1) as u32], false); }
    let r_ = 'f: {
    F::Font_AddCmd(c, ((p_ch_1) as i32), 0i32, 2i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Font_PutChar", &PROBE_N, &[], true); }
    r_
}

pub fn Font_PutCharAt(c: &mut Ctx, mut p_x_1: i32, mut p_y_2: i32, mut p_ch_3: u16) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Font_PutCharAt", &PROBE_N, &[(p_x_1) as u32, (p_y_2) as u32, (p_ch_3) as u32], false); }
    let r_ = 'f: {
    { let _ = F::Font_AddCmdAt(c, p_x_1, p_y_2, ((p_ch_3) as i32), 0i32, 2i32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Font_PutCharAt", &PROBE_N, &[], true); }
    r_
}

pub fn Font_PrintAscii(c: &mut Ctx, mut p_str_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Font_PrintAscii", &PROBE_N, &[(p_str_1) as u32], false); }
    let r_ = 'f: {
    F::Font_AddCmd(c, ((p_str_1) as i32), 0i32, 1i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Font_PrintAscii", &PROBE_N, &[], true); }
    r_
}

pub fn Font_PrintAsciiAt(c: &mut Ctx, mut p_x_1: i32, mut p_y_2: i32, mut p_str_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Font_PrintAsciiAt", &PROBE_N, &[(p_x_1) as u32, (p_y_2) as u32, (p_str_3) as u32], false); }
    let r_ = 'f: {
    { let _ = F::Font_AddCmdAt(c, p_x_1, p_y_2, ((p_str_3) as i32), 0i32, 1i32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Font_PrintAsciiAt", &PROBE_N, &[], true); }
    r_
}

pub fn Font_PutAsciiChar(c: &mut Ctx, mut p_ch_1: i8) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Font_PutAsciiChar", &PROBE_N, &[(p_ch_1) as u32], false); }
    let r_ = 'f: {
    F::Font_AddCmd(c, ((p_ch_1) as i32), 0i32, 3i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Font_PutAsciiChar", &PROBE_N, &[], true); }
    r_
}

pub fn Font_PutAsciiCharAt(c: &mut Ctx, mut p_x_1: i32, mut p_y_2: i32, mut p_ch_3: i8) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Font_PutAsciiCharAt", &PROBE_N, &[(p_x_1) as u32, (p_y_2) as u32, (p_ch_3) as u32], false); }
    let r_ = 'f: {
    { let _ = F::Font_AddCmdAt(c, p_x_1, p_y_2, ((p_ch_3) as i32), 0i32, 3i32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Font_PutAsciiCharAt", &PROBE_N, &[], true); }
    r_
}

pub fn Font_PrintNum(c: &mut Ctx, mut p_value_1: i32, mut p_digits_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Font_PrintNum", &PROBE_N, &[(p_value_1) as u32, (p_digits_2) as u32], false); }
    let r_ = 'f: {
    if (((((p_digits_2) == (0i32)) as i32)) != 0) {
        p_digits_2 = F::Font_CountDigits(c, p_value_1);
    }
    F::Font_AddCmd(c, p_value_1, p_digits_2, 4i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Font_PrintNum", &PROBE_N, &[], true); }
    r_
}

pub fn Font_PrintNumAt(c: &mut Ctx, mut p_x_1: i32, mut p_y_2: i32, mut p_value_3: i32, mut p_digits_4: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Font_PrintNumAt", &PROBE_N, &[(p_x_1) as u32, (p_y_2) as u32, (p_value_3) as u32, (p_digits_4) as u32], false); }
    let r_ = 'f: {
    if (((((p_digits_4) == (0i32)) as i32)) != 0) {
        p_digits_4 = F::Font_CountDigits(c, p_value_3);
    }
    { let _ = F::Font_AddCmdAt(c, p_x_1, p_y_2, p_value_3, p_digits_4, 4i32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Font_PrintNumAt", &PROBE_N, &[], true); }
    r_
}

pub fn Font_SetCursorState(c: &mut Ctx, mut p_cursor_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Font_SetCursorState", &PROBE_N, &[(p_cursor_1) as u32], false); }
    let r_ = 'f: {
    { let s_ = p_cursor_1; let d_ = sym::gFontCursor; rt::memcpy(c, d_, s_, 16); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Font_SetCursorState", &PROBE_N, &[], true); }
    r_
}

pub fn Font_PushCursor(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Font_PushCursor", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let s_ = sym::gFontCursor; let d_ = (c.m.r32(sym::gFontCursorStack)).wrapping_add(((((c.m.r32(sym::gFontCursorDepth) as i32)) as u32)).wrapping_mul(16)); rt::memcpy(c, d_, s_, 16); };
    { let a_ = sym::gFontCursorDepth; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Font_PushCursor", &PROBE_N, &[], true); }
    r_
}

pub fn Font_PopCursor(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Font_PopCursor", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let a_ = sym::gFontCursorDepth; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
    { let s_ = (c.m.r32(sym::gFontCursorStack)).wrapping_add(((((c.m.r32(sym::gFontCursorDepth) as i32)) as u32)).wrapping_mul(16)); let d_ = sym::gFontCursor; rt::memcpy(c, d_, s_, 16); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Font_PopCursor", &PROBE_N, &[], true); }
    r_
}

pub fn Font_SetHome(c: &mut Ctx, mut p_x_1: i32, mut p_y_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Font_SetHome", &PROBE_N, &[(p_x_1) as u32, (p_y_2) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_x_1; let a_ = sym::gFontCursor; c.m.w32(a_, (v_) as u32); };
    { let v_ = p_y_2; let a_ = (sym::gFontCursor).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
    F::Font_Home(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Font_SetHome", &PROBE_N, &[], true); }
    r_
}

pub fn Font_GetHome(c: &mut Ctx, mut p_x_1: u32, mut p_y_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Font_GetHome", &PROBE_N, &[(p_x_1) as u32, (p_y_2) as u32], false); }
    let r_ = 'f: {
    { let v_ = (c.m.r32(sym::gFontCursor) as i32); let a_ = p_x_1; c.m.w32(a_, (v_) as u32); };
    { let v_ = (c.m.r32((sym::gFontCursor).wrapping_add(4)) as i32); let a_ = p_y_2; c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Font_GetHome", &PROBE_N, &[], true); }
    r_
}

pub fn Font_SetCursor(c: &mut Ctx, mut p_x_1: i32, mut p_y_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Font_SetCursor", &PROBE_N, &[(p_x_1) as u32, (p_y_2) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_x_1; let a_ = (sym::gFontCursor).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
    { let v_ = p_y_2; let a_ = (sym::gFontCursor).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Font_SetCursor", &PROBE_N, &[], true); }
    r_
}

pub fn Font_GetCursor(c: &mut Ctx, mut p_x_1: u32, mut p_y_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Font_GetCursor", &PROBE_N, &[(p_x_1) as u32, (p_y_2) as u32], false); }
    let r_ = 'f: {
    { let v_ = (c.m.r32((sym::gFontCursor).wrapping_add(8)) as i32); let a_ = p_x_1; c.m.w32(a_, (v_) as u32); };
    { let v_ = (c.m.r32((sym::gFontCursor).wrapping_add(12)) as i32); let a_ = p_y_2; c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Font_GetCursor", &PROBE_N, &[], true); }
    r_
}

pub fn Font_Home(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Font_Home", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let v_ = (c.m.r32(sym::gFontCursor) as i32); let a_ = (sym::gFontCursor).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
    { let v_ = (c.m.r32((sym::gFontCursor).wrapping_add(4)) as i32); let a_ = (sym::gFontCursor).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Font_Home", &PROBE_N, &[], true); }
    r_
}

pub fn Font_NewLine(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Font_NewLine", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let v_ = (c.m.r32(sym::gFontCursor) as i32); let a_ = (sym::gFontCursor).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
    { let r_ = rt::f2i(rt::fmul(rt::i2f(((c.m.r32(sym::gFontLineHeight) as i32)).wrapping_add((c.m.r32((sym::gFontStyle).wrapping_add(24)) as i32))), rt::fmul(F32(c.m.r32((sym::gFontStyle).wrapping_add(4))), F32(c.m.r32((sym::gFontStyle).wrapping_add(8)))))); let a_ = (sym::gFontCursor).wrapping_add(12); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Font_NewLine", &PROBE_N, &[], true); }
    r_
}

pub fn Font_SetStyle(c: &mut Ctx, mut p_style_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Font_SetStyle", &PROBE_N, &[(p_style_1) as u32], false); }
    let r_ = 'f: {
    { let s_ = p_style_1; let d_ = sym::gFontStyle; rt::memcpy(c, d_, s_, 72); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Font_SetStyle", &PROBE_N, &[], true); }
    r_
}

pub fn Font_GetStyle(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Font_GetStyle", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f sym::gFontStyle;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Font_GetStyle", &PROBE_N, &[], true); }
    r_
}

pub fn Font_PushStyle(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Font_PushStyle", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let s_ = sym::gFontStyle; let d_ = (c.m.r32(sym::gFontStyleStack)).wrapping_add(((((c.m.r32(sym::gFontStyleDepth) as i32)) as u32)).wrapping_mul(72)); rt::memcpy(c, d_, s_, 72); };
    { let a_ = sym::gFontStyleDepth; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Font_PushStyle", &PROBE_N, &[], true); }
    r_
}

pub fn Font_PopStyle(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Font_PopStyle", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let a_ = sym::gFontStyleDepth; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
    { let s_ = (c.m.r32(sym::gFontStyleStack)).wrapping_add(((((c.m.r32(sym::gFontStyleDepth) as i32)) as u32)).wrapping_mul(72)); let d_ = sym::gFontStyle; rt::memcpy(c, d_, s_, 72); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Font_PopStyle", &PROBE_N, &[], true); }
    r_
}

pub fn Font_SetColor(c: &mut Ctx, mut p_color_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Font_SetColor", &PROBE_N, &[(p_color_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_color_1; let a_ = (sym::gFontStyle).wrapping_add(12); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Font_SetColor", &PROBE_N, &[], true); }
    r_
}

pub fn Font_GetColor(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Font_GetColor", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f c.m.r32((sym::gFontStyle).wrapping_add(12));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Font_GetColor", &PROBE_N, &[], true); }
    r_
}

pub fn Font_SetScale(c: &mut Ctx, mut p_scale_1: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Font_SetScale", &PROBE_N, &[(p_scale_1).0], false); }
    let r_ = 'f: {
    { let v_ = p_scale_1; let a_ = (sym::gFontStyle).wrapping_add(8); c.m.w32(a_, (v_).0); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Font_SetScale", &PROBE_N, &[], true); }
    r_
}

pub fn Font_GetScale(c: &mut Ctx) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Font_GetScale", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f F32(c.m.r32((sym::gFontStyle).wrapping_add(8)));
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "Font_GetScale", &PROBE_N, &[], true); }
    r_
}

pub fn Font_SetScaleXY(c: &mut Ctx, mut p_x_1: F32, mut p_y_2: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Font_SetScaleXY", &PROBE_N, &[(p_x_1).0, (p_y_2).0], false); }
    let r_ = 'f: {
    { let v_ = p_x_1; let a_ = sym::gFontStyle; c.m.w32(a_, (v_).0); };
    { let v_ = p_y_2; let a_ = (sym::gFontStyle).wrapping_add(4); c.m.w32(a_, (v_).0); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Font_SetScaleXY", &PROBE_N, &[], true); }
    r_
}

pub fn Font_GetScaleXY(c: &mut Ctx, mut p_x_1: u32, mut p_y_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Font_GetScaleXY", &PROBE_N, &[(p_x_1) as u32, (p_y_2) as u32], false); }
    let r_ = 'f: {
    { let v_ = F32(c.m.r32(sym::gFontStyle)); let a_ = p_x_1; c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((sym::gFontStyle).wrapping_add(4))); let a_ = p_y_2; c.m.w32(a_, (v_).0); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Font_GetScaleXY", &PROBE_N, &[], true); }
    r_
}

pub fn Font_SetSpacing(c: &mut Ctx, mut p_x_1: i32, mut p_y_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Font_SetSpacing", &PROBE_N, &[(p_x_1) as u32, (p_y_2) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_x_1; let a_ = (sym::gFontStyle).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
    { let v_ = p_y_2; let a_ = (sym::gFontStyle).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Font_SetSpacing", &PROBE_N, &[], true); }
    r_
}

pub fn Font_GetSpacing(c: &mut Ctx, mut p_x_1: u32, mut p_y_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Font_GetSpacing", &PROBE_N, &[(p_x_1) as u32, (p_y_2) as u32], false); }
    let r_ = 'f: {
    { let v_ = (c.m.r32((sym::gFontStyle).wrapping_add(20)) as i32); let a_ = p_x_1; c.m.w32(a_, (v_) as u32); };
    { let v_ = (c.m.r32((sym::gFontStyle).wrapping_add(24)) as i32); let a_ = p_y_2; c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Font_GetSpacing", &PROBE_N, &[], true); }
    r_
}

pub fn Font_SetClip(c: &mut Ctx, mut p_x0_1: i32, mut p_y0_2: i32, mut p_x1_3: i32, mut p_y1_4: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Font_SetClip", &PROBE_N, &[(p_x0_1) as u32, (p_y0_2) as u32, (p_x1_3) as u32, (p_y1_4) as u32], false); }
    let r_ = 'f: {
    { let v_ = ((p_x0_1) as u16); let a_ = (sym::gFontStyle).wrapping_add(48); c.m.w16(a_, v_); };
    { let v_ = ((p_y0_2) as u16); let a_ = (sym::gFontStyle).wrapping_add(50); c.m.w16(a_, v_); };
    { let v_ = ((p_x1_3) as u16); let a_ = (sym::gFontStyle).wrapping_add(52); c.m.w16(a_, v_); };
    { let v_ = ((p_y1_4) as u16); let a_ = (sym::gFontStyle).wrapping_add(54); c.m.w16(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Font_SetClip", &PROBE_N, &[], true); }
    r_
}

pub fn Font_GetClip(c: &mut Ctx, mut p_x0_1: u32, mut p_y0_2: u32, mut p_x1_3: u32, mut p_y1_4: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Font_GetClip", &PROBE_N, &[(p_x0_1) as u32, (p_y0_2) as u32, (p_x1_3) as u32, (p_y1_4) as u32], false); }
    let r_ = 'f: {
    { let v_ = ((c.m.r16((sym::gFontStyle).wrapping_add(48))) as i32); let a_ = p_x0_1; c.m.w32(a_, (v_) as u32); };
    { let v_ = ((c.m.r16((sym::gFontStyle).wrapping_add(50))) as i32); let a_ = p_y0_2; c.m.w32(a_, (v_) as u32); };
    { let v_ = ((c.m.r16((sym::gFontStyle).wrapping_add(52))) as i32); let a_ = p_x1_3; c.m.w32(a_, (v_) as u32); };
    { let v_ = ((c.m.r16((sym::gFontStyle).wrapping_add(54))) as i32); let a_ = p_y1_4; c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Font_GetClip", &PROBE_N, &[], true); }
    r_
}

pub fn Font_SetAlign(c: &mut Ctx, mut p_align_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Font_SetAlign", &PROBE_N, &[(p_align_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = ((p_align_1) as u32); let a_ = (sym::gFontStyle).wrapping_add(28); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Font_SetAlign", &PROBE_N, &[], true); }
    r_
}

pub fn Font_GetAlign(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Font_GetAlign", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f ((c.m.r32((sym::gFontStyle).wrapping_add(28))) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Font_GetAlign", &PROBE_N, &[], true); }
    r_
}

pub fn Font_SetFlags(c: &mut Ctx, mut p_flags_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Font_SetFlags", &PROBE_N, &[(p_flags_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_flags_1; let a_ = (sym::gFontStyle).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Font_SetFlags", &PROBE_N, &[], true); }
    r_
}

pub fn Font_GetFlags(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Font_GetFlags", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f (c.m.r32((sym::gFontStyle).wrapping_add(32)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Font_GetFlags", &PROBE_N, &[], true); }
    r_
}

pub fn Font_SetShadowMode(c: &mut Ctx, mut p_mode_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Font_SetShadowMode", &PROBE_N, &[(p_mode_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = ((p_mode_1) as u32); let a_ = (sym::gFontStyle).wrapping_add(36); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Font_SetShadowMode", &PROBE_N, &[], true); }
    r_
}

pub fn Font_GetShadowMode(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Font_GetShadowMode", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f ((c.m.r32((sym::gFontStyle).wrapping_add(36))) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Font_GetShadowMode", &PROBE_N, &[], true); }
    r_
}

pub fn Font_SetShadowColor(c: &mut Ctx, mut p_color_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Font_SetShadowColor", &PROBE_N, &[(p_color_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_color_1; let a_ = (sym::gFontStyle).wrapping_add(40); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Font_SetShadowColor", &PROBE_N, &[], true); }
    r_
}

pub fn Font_GetShadowColor(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Font_GetShadowColor", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f c.m.r32((sym::gFontStyle).wrapping_add(40));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Font_GetShadowColor", &PROBE_N, &[], true); }
    r_
}

pub fn Font_SetShadowOffset(c: &mut Ctx, mut p_dx_1: i32, mut p_dy_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Font_SetShadowOffset", &PROBE_N, &[(p_dx_1) as u32, (p_dy_2) as u32], false); }
    let r_ = 'f: {
    { let v_ = ((p_dx_1) as u16); let a_ = (sym::gFontStyle).wrapping_add(44); c.m.w16(a_, v_); };
    { let v_ = ((p_dy_2) as u16); let a_ = (sym::gFontStyle).wrapping_add(46); c.m.w16(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Font_SetShadowOffset", &PROBE_N, &[], true); }
    r_
}

pub fn Font_GetShadowOffset(c: &mut Ctx, mut p_dx_1: u32, mut p_dy_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Font_GetShadowOffset", &PROBE_N, &[(p_dx_1) as u32, (p_dy_2) as u32], false); }
    let r_ = 'f: {
    { let v_ = ((c.m.r16((sym::gFontStyle).wrapping_add(44))) as i32); let a_ = p_dx_1; c.m.w32(a_, (v_) as u32); };
    { let v_ = ((c.m.r16((sym::gFontStyle).wrapping_add(46))) as i32); let a_ = p_dy_2; c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Font_GetShadowOffset", &PROBE_N, &[], true); }
    r_
}

pub fn Font_SetColorRGBA(c: &mut Ctx, mut p_r_1: u8, mut p_g_2: u8, mut p_b_3: u8, mut p_a_4: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Font_SetColorRGBA", &PROBE_N, &[(p_r_1) as u32, (p_g_2) as u32, (p_b_3) as u32, (p_a_4) as u32], false); }
    let r_ = 'f: {
    { let v_ = ((((((((p_r_1) as i32)) | ((p_a_4).wrapping_shl((24i32) as u32)))) | ((((((p_b_3) as i32)).wrapping_shl((16i32) as u32)) | ((((p_g_2) as i32)).wrapping_shl((8i32) as u32)))))) as u32); let a_ = (sym::gFontStyle).wrapping_add(12); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Font_SetColorRGBA", &PROBE_N, &[], true); }
    r_
}

pub fn Font_SetColor2RGBA(c: &mut Ctx, mut p_r_1: u8, mut p_g_2: u8, mut p_b_3: u8, mut p_a_4: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Font_SetColor2RGBA", &PROBE_N, &[(p_r_1) as u32, (p_g_2) as u32, (p_b_3) as u32, (p_a_4) as u32], false); }
    let r_ = 'f: {
    { let v_ = ((((((((p_r_1) as i32)) | ((p_a_4).wrapping_shl((24i32) as u32)))) | ((((((p_b_3) as i32)).wrapping_shl((16i32) as u32)) | ((((p_g_2) as i32)).wrapping_shl((8i32) as u32)))))) as u32); let a_ = (sym::gFontStyle).wrapping_add(16); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Font_SetColor2RGBA", &PROBE_N, &[], true); }
    r_
}

pub fn Font_SetShadowColorRGBA(c: &mut Ctx, mut p_r_1: u8, mut p_g_2: u8, mut p_b_3: u8, mut p_a_4: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Font_SetShadowColorRGBA", &PROBE_N, &[(p_r_1) as u32, (p_g_2) as u32, (p_b_3) as u32, (p_a_4) as u32], false); }
    let r_ = 'f: {
    { let v_ = ((((((((p_r_1) as i32)) | ((p_a_4).wrapping_shl((24i32) as u32)))) | ((((((p_b_3) as i32)).wrapping_shl((16i32) as u32)) | ((((p_g_2) as i32)).wrapping_shl((8i32) as u32)))))) as u32); let a_ = (sym::gFontStyle).wrapping_add(40); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Font_SetShadowColorRGBA", &PROBE_N, &[], true); }
    r_
}

pub fn Font_SetBeginCallback(c: &mut Ctx, mut p_fn_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Font_SetBeginCallback", &PROBE_N, &[(p_fn_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_fn_1; let a_ = (sym::gFontStyle).wrapping_add(60); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Font_SetBeginCallback", &PROBE_N, &[], true); }
    r_
}

pub fn Font_SetTagCallback(c: &mut Ctx, mut p_fn_1: u32, mut p_arg_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Font_SetTagCallback", &PROBE_N, &[(p_fn_1) as u32, (p_arg_2) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_fn_1; let a_ = (sym::gFontStyle).wrapping_add(64); c.m.w32(a_, v_); };
    { let v_ = p_arg_2; let a_ = (sym::gFontStyle).wrapping_add(68); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Font_SetTagCallback", &PROBE_N, &[], true); }
    r_
}

pub fn Font_ChangeUserFlags(c: &mut Ctx, mut p_mask_1: i32, mut p_set_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Font_ChangeUserFlags", &PROBE_N, &[(p_mask_1) as u32, (p_set_2) as u32], false); }
    let r_ = 'f: {
    { let v_ = (((((c.m.r32((sym::gFontStyle).wrapping_add(56)) as i32)) & (p_mask_1))) | (p_set_2)); let a_ = (sym::gFontStyle).wrapping_add(56); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Font_ChangeUserFlags", &PROBE_N, &[], true); }
    r_
}

pub fn Font_GetUserFlags(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Font_GetUserFlags", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f (c.m.r32((sym::gFontStyle).wrapping_add(56)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Font_GetUserFlags", &PROBE_N, &[], true); }
    r_
}

pub fn Font_DrawCmd(c: &mut Ctx, mut p_pkt_1: u32, mut p_cmd_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Font_DrawCmd", &PROBE_N, &[(p_pkt_1) as u32, (p_cmd_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(96) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 96);
    let mut l_kind_3: i32 = 0;
    let mut l_vram_4: i32 = 0;
    let mut l_ch_5: u16 = 0;
    let mut l_div_6: i32 = 0;
    let mut l_i_7: i32 = 0;
    let r_ = 'f: {
        { let v_ = (((c.m.r16((p_cmd_2).wrapping_add(8)) as i16)) as i32); let a_ = fp.wrapping_add(84); c.m.w32(a_, (v_) as u32); };
        { let v_ = (((c.m.r16((p_cmd_2).wrapping_add(10)) as i16)) as i32); let a_ = fp.wrapping_add(88); c.m.w32(a_, (v_) as u32); };
        l_kind_3 = (c.m.r32(p_cmd_2) as i32);
        l_vram_4 = (c.m.r32((p_cmd_2).wrapping_add(76)) as i32);
        if (((((p_pkt_1) != (((0i32) as u32))) as i32)) != 0) {
            { let s_ = p_cmd_2; let d_ = fp; rt::memcpy(c, d_, s_, 84); };
            's1: {
                let sv8: i32 = l_kind_3;
                match sv8 {
                    0i32 | 1i32 => {
                        { let v_ = (((c.m.r32((p_cmd_2).wrapping_add(4)) as i32)) as u32); let a_ = fp.wrapping_add(92); c.m.w32(a_, v_); };
                        { let v_ = { let a1_ = (c.m.r32(fp.wrapping_add(84)) as i32); let a2_ = c.m.r32(fp.wrapping_add(92)); F::Font_AlignLine(c, p_cmd_2, a1_, a2_) }; let a_ = fp.wrapping_add(84); c.m.w32(a_, (v_) as u32); };
                        'l2: loop {
                            'c3: {
                                if (((((((((l_kind_3) == (0i32)) as i32)) != 0) && (((((c.m.r32((p_cmd_2).wrapping_add(72))) != (((0i32) as u32))) as i32)) != 0)) as i32)) != 0) {
                                    { let fp_ = c.m.r32((p_cmd_2).wrapping_add(72)); { let _ = D::call(c, fp_, &[((p_pkt_1) as u64), ((p_cmd_2) as u64), ((fp) as u64), ((fp.wrapping_add(84)) as u64), ((fp.wrapping_add(88)) as u64), ((fp.wrapping_add(92)) as u64), ((l_vram_4) as i64 as u64), ((1i32) as i64 as u64)]); } };
                                }
                                l_ch_5 = { let a1_ = c.m.r32(fp.wrapping_add(92)); F::Font_ReadChar(c, l_kind_3, a1_) };
                                { let v_ = (c.m.r32(fp.wrapping_add(92))).wrapping_add(((F::Font_CharSize(c, l_kind_3)) as u32)); let a_ = fp.wrapping_add(92); c.m.w32(a_, v_); };
                            }
                            if !((((!(({ let a5_ = c.m.r32(fp.wrapping_add(92)); F::Font_DrawChar(c, p_pkt_1, p_cmd_2, fp.wrapping_add(84), fp.wrapping_add(88), l_ch_5, a5_) }) != 0)) as i32)) != 0) { break 'l2; }
                        }
                        break 's1;
                    }
                    2i32 | 3i32 => {
                        l_ch_5 = (((c.m.r32((p_cmd_2).wrapping_add(4)) as i32)) as u16);
                        { let v_ = { let a1_ = (c.m.r32(fp.wrapping_add(84)) as i32); F::Font_AlignLine(c, p_cmd_2, a1_, ((0i32) as u32)) }; let a_ = fp.wrapping_add(84); c.m.w32(a_, (v_) as u32); };
                        { let _ = F::Font_DrawChar(c, p_pkt_1, p_cmd_2, fp.wrapping_add(84), fp.wrapping_add(88), l_ch_5, ((0i32) as u32)); };
                        break 's1;
                    }
                    4i32 => {
                        { let v_ = { let a1_ = (c.m.r32(fp.wrapping_add(84)) as i32); F::Font_AlignLine(c, p_cmd_2, a1_, ((0i32) as u32)) }; let a_ = fp.wrapping_add(84); c.m.w32(a_, (v_) as u32); };
                        l_div_6 = { let a0_ = (((c.m.r16((p_cmd_2).wrapping_add(14)) as i16)) as i32); F::Font_Pow10(c, a0_) };
                        {
                            l_i_7 = 0i32;
                            'l4: loop {
                                if !(((((l_i_7) < ((((c.m.r16((p_cmd_2).wrapping_add(14)) as i16)) as i32))) as i32)) != 0) { break 'l4; }
                                'c5: {
                                    { let _ = { let a4_ = (((rt::rem_i32(rt::div_i32((c.m.r32((p_cmd_2).wrapping_add(4)) as i32), l_div_6), 10i32)).wrapping_add(48i32)) as u16); F::Font_DrawChar(c, p_pkt_1, p_cmd_2, fp.wrapping_add(84), fp.wrapping_add(88), a4_, ((0i32) as u32)) }; };
                                    { let r_ = 10i32; let n_ = rt::div_i32(l_div_6, r_); l_div_6 = n_; };
                                }
                                { let o_ = l_i_7; let n_ = (o_).wrapping_add(1); l_i_7 = n_; };
                            }
                        }
                        break 's1;
                    }
                    _ => {
                    }
                }
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Font_DrawCmd", &PROBE_N, &[], true); }
    r_
}

pub fn Font_DrawChar(c: &mut Ctx, mut p_pkt_1: u32, mut p_cmd_2: u32, mut p_x_3: u32, mut p_y_4: u32, mut p_ch_5: u16, mut p_next_6: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Font_DrawChar", &PROBE_N, &[(p_pkt_1) as u32, (p_cmd_2) as u32, (p_x_3) as u32, (p_y_4) as u32, (p_ch_5) as u32, (p_next_6) as u32], false); }
    let mut l_done_7: i32 = 0;
    let r_ = 'f: {
    l_done_7 = 0i32;
    if (((((((p_ch_5) as i32)) == (0i32)) as i32)) != 0) {
        l_done_7 = 1i32;
    } else {
        if ((F::Font_IsNewline(c, p_ch_5)) != 0) {
            { let v_ = { let a1_ = (((c.m.r16((p_cmd_2).wrapping_add(12)) as i16)) as i32); F::Font_AlignLine(c, p_cmd_2, a1_, p_next_6) }; let a_ = p_x_3; c.m.w32(a_, (v_) as u32); };
            { let r_ = rt::f2i(rt::fmul(rt::i2f(((c.m.r32(sym::gFontLineHeight) as i32)).wrapping_add(((c.m.r16((p_cmd_2).wrapping_add(26))) as i32))), F32(c.m.r32((p_cmd_2).wrapping_add(20))))); let a_ = p_y_4; let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
            if (((((c.m.r32((p_cmd_2).wrapping_add(64)) as i32)) & (1i32))) != 0) {
                l_done_7 = 1i32;
            }
        } else {
            { let r_ = { let a2_ = (c.m.r32(p_x_3) as i32); let a3_ = (c.m.r32(p_y_4) as i32); F::Font_DrawGlyph(c, p_pkt_1, p_cmd_2, a2_, a3_, p_ch_5) }; let a_ = p_x_3; let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
        }
    }
    break 'f l_done_7;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Font_DrawChar", &PROBE_N, &[], true); }
    r_
}

pub fn Font_AlignLine(c: &mut Ctx, mut p_cmd_1: u32, mut p_x_2: i32, mut p_str_3: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Font_AlignLine", &PROBE_N, &[(p_cmd_1) as u32, (p_x_2) as u32, (p_str_3) as u32], false); }
    let r_ = 'f: {
    's1: {
        let sv4: u32 = c.m.r32((p_cmd_1).wrapping_add(60));
        match sv4 {
            1u32 => {
                { let r_ = rt::div_i32(F::Font_MeasureLine(c, p_cmd_1, p_str_3), 2i32); let n_ = (p_x_2).wrapping_sub(r_); p_x_2 = n_; };
                break 's1;
            }
            0u32 => {
                break 's1;
            }
            2u32 => {
                { let r_ = F::Font_MeasureLine(c, p_cmd_1, p_str_3); let n_ = (p_x_2).wrapping_sub(r_); p_x_2 = n_; };
                break 's1;
            }
            _ => {
            }
        }
    }
    break 'f p_x_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Font_AlignLine", &PROBE_N, &[], true); }
    r_
}

pub fn Font_MeasureLine(c: &mut Ctx, mut p_cmd_1: u32, mut p_str_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Font_MeasureLine", &PROBE_N, &[(p_cmd_1) as u32, (p_str_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_kind_3: i32 = 0;
    let mut l_ch_4: u16 = 0;
    let mut l_div_5: i32 = 0;
    let mut l_i_6: i32 = 0;
    let r_ = 'f: {
        l_kind_3 = (c.m.r32(p_cmd_1) as i32);
        { let v_ = 0i32; let a_ = fp; c.m.w32(a_, (v_) as u32); };
        's1: {
            let sv7: i32 = l_kind_3;
            match sv7 {
                0i32 | 1i32 => {
                    'l2: loop {
                        'c3: {
                            l_ch_4 = F::Font_ReadChar(c, l_kind_3, p_str_2);
                            p_str_2 = (p_str_2).wrapping_add(((F::Font_CharSize(c, l_kind_3)) as u32));
                        }
                        if !((((!((F::Font_MeasureChar(c, p_cmd_1, fp, l_ch_4)) != 0)) as i32)) != 0) { break 'l2; }
                    }
                    break 's1;
                }
                2i32 | 3i32 => {
                    { let _ = { let a2_ = (((c.m.r32((p_cmd_1).wrapping_add(4)) as i32)) as u16); F::Font_MeasureChar(c, p_cmd_1, fp, a2_) }; };
                    break 's1;
                }
                4i32 => {
                    l_div_5 = { let a0_ = (((c.m.r16((p_cmd_1).wrapping_add(14)) as i16)) as i32); F::Font_Pow10(c, a0_) };
                    {
                        l_i_6 = 0i32;
                        'l4: loop {
                            if !(((((l_i_6) < ((((c.m.r16((p_cmd_1).wrapping_add(14)) as i16)) as i32))) as i32)) != 0) { break 'l4; }
                            'c5: {
                                { let _ = { let a2_ = (((rt::rem_i32(rt::div_i32((c.m.r32((p_cmd_1).wrapping_add(4)) as i32), l_div_5), 10i32)).wrapping_add(48i32)) as u16); F::Font_MeasureChar(c, p_cmd_1, fp, a2_) }; };
                                { let r_ = 10i32; let n_ = rt::div_i32(l_div_5, r_); l_div_5 = n_; };
                            }
                            { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; };
                        }
                    }
                    break 's1;
                }
                _ => {
                }
            }
        }
        break 'f (c.m.r32(fp) as i32);
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Font_MeasureLine", &PROBE_N, &[], true); }
    r_
}

pub fn Font_MeasureChar(c: &mut Ctx, mut p_cmd_1: u32, mut p_width_2: u32, mut p_ch_3: u16) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Font_MeasureChar", &PROBE_N, &[(p_cmd_1) as u32, (p_width_2) as u32, (p_ch_3) as u32], false); }
    let mut l_done_4: i32 = 0;
    let mut l_glyph_5: u32 = 0;
    let r_ = 'f: {
    l_done_4 = 0i32;
    if (((((((p_ch_3) as i32)) == (0i32)) as i32)) != 0) {
        l_done_4 = 1i32;
    } else {
        if ((F::Font_IsNewline(c, p_ch_3)) != 0) {
            l_done_4 = 1i32;
        } else {
            l_glyph_5 = F::Font_FindGlyph(c, p_ch_3, ((0i32) as u32));
            if (((((l_glyph_5) != (((0i32) as u32))) as i32)) != 0) {
                { let r_ = (rt::f2i(rt::fmul(rt::i2f((((((c.m.r16((l_glyph_5).wrapping_add(2))) as i32)).wrapping_shr((10i32) as u32)) & (31i32))), F32(c.m.r32((p_cmd_1).wrapping_add(16)))))).wrapping_add((((c.m.r16((p_cmd_1).wrapping_add(24))) as i32)).wrapping_add((c.m.r32(sym::gFontGlyphGap) as i32))); let a_ = p_width_2; let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
            }
        }
    }
    break 'f l_done_4;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Font_MeasureChar", &PROBE_N, &[], true); }
    r_
}

pub fn Font_AdvanceCursor(c: &mut Ctx, mut p_cmd_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Font_AdvanceCursor", &PROBE_N, &[(p_cmd_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_kind_2: i32 = 0;
    let mut l_str_3: u32 = 0;
    let mut l_ch_4: u16 = 0;
    let mut l_div_5: i32 = 0;
    let mut l_i_6: i32 = 0;
    let r_ = 'f: {
        l_kind_2 = (c.m.r32(p_cmd_1) as i32);
        { let v_ = (c.m.r32((sym::gFontCursor).wrapping_add(8)) as i32); let a_ = fp; c.m.w32(a_, (v_) as u32); };
        { let v_ = (c.m.r32((sym::gFontCursor).wrapping_add(12)) as i32); let a_ = fp.wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        's1: {
            let sv7: i32 = l_kind_2;
            match sv7 {
                0i32 | 1i32 => {
                    l_str_3 = (((c.m.r32((p_cmd_1).wrapping_add(4)) as i32)) as u32);
                    'l2: loop {
                        'c3: {
                            l_ch_4 = F::Font_ReadChar(c, l_kind_2, l_str_3);
                            l_str_3 = (l_str_3).wrapping_add(((F::Font_CharSize(c, l_kind_2)) as u32));
                        }
                        if !((((!((F::Font_AdvanceChar(c, p_cmd_1, fp, fp.wrapping_add(4), l_ch_4)) != 0)) as i32)) != 0) { break 'l2; }
                    }
                    break 's1;
                }
                2i32 | 3i32 => {
                    { let _ = { let a3_ = (((c.m.r32((p_cmd_1).wrapping_add(4)) as i32)) as u16); F::Font_AdvanceChar(c, p_cmd_1, fp, fp.wrapping_add(4), a3_) }; };
                    break 's1;
                }
                4i32 => {
                    l_div_5 = { let a0_ = (((c.m.r16((p_cmd_1).wrapping_add(14)) as i16)) as i32); F::Font_Pow10(c, a0_) };
                    {
                        l_i_6 = 0i32;
                        'l4: loop {
                            if !(((((l_i_6) < ((((c.m.r16((p_cmd_1).wrapping_add(14)) as i16)) as i32))) as i32)) != 0) { break 'l4; }
                            'c5: {
                                { let _ = { let a3_ = (((rt::rem_i32(rt::div_i32((c.m.r32((p_cmd_1).wrapping_add(4)) as i32), l_div_5), 10i32)).wrapping_add(48i32)) as u16); F::Font_AdvanceChar(c, p_cmd_1, fp, fp.wrapping_add(4), a3_) }; };
                                { let r_ = 10i32; let n_ = rt::div_i32(l_div_5, r_); l_div_5 = n_; };
                            }
                            { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; };
                        }
                    }
                    break 's1;
                }
                _ => {
                }
            }
        }
        { let v_ = (c.m.r32(fp) as i32); let a_ = (sym::gFontCursor).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = (c.m.r32(fp.wrapping_add(4)) as i32); let a_ = (sym::gFontCursor).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Font_AdvanceCursor", &PROBE_N, &[], true); }
    r_
}

pub fn Font_AdvanceChar(c: &mut Ctx, mut p_cmd_1: u32, mut p_x_2: u32, mut p_y_3: u32, mut p_ch_4: u16) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Font_AdvanceChar", &PROBE_N, &[(p_cmd_1) as u32, (p_x_2) as u32, (p_y_3) as u32, (p_ch_4) as u32], false); }
    let mut l_done_5: i32 = 0;
    let mut l_glyph_6: u32 = 0;
    let r_ = 'f: {
    l_done_5 = 0i32;
    if (((((((p_ch_4) as i32)) == (0i32)) as i32)) != 0) {
        l_done_5 = 1i32;
    } else {
        if ((F::Font_IsNewline(c, p_ch_4)) != 0) {
            { let v_ = (c.m.r32(sym::gFontCursor) as i32); let a_ = p_x_2; c.m.w32(a_, (v_) as u32); };
            { let r_ = rt::f2i(rt::fmul(rt::i2f(((c.m.r32(sym::gFontLineHeight) as i32)).wrapping_add(((c.m.r16((p_cmd_1).wrapping_add(26))) as i32))), F32(c.m.r32((p_cmd_1).wrapping_add(20))))); let a_ = p_y_3; let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
            if (((((c.m.r32((p_cmd_1).wrapping_add(64)) as i32)) & (1i32))) != 0) {
                l_done_5 = 1i32;
            }
        } else {
            l_glyph_6 = F::Font_FindGlyph(c, p_ch_4, ((0i32) as u32));
            if (((((l_glyph_6) != (((0i32) as u32))) as i32)) != 0) {
                { let r_ = (rt::f2i(rt::fmul(rt::i2f((((((c.m.r16((l_glyph_6).wrapping_add(2))) as i32)).wrapping_shr((10i32) as u32)) & (31i32))), F32(c.m.r32((p_cmd_1).wrapping_add(16)))))).wrapping_add((((c.m.r16((p_cmd_1).wrapping_add(24))) as i32)).wrapping_add((c.m.r32(sym::gFontGlyphGap) as i32))); let a_ = p_x_2; let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
            }
        }
    }
    break 'f l_done_5;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Font_AdvanceChar", &PROBE_N, &[], true); }
    r_
}

pub fn Font_CharSize(c: &mut Ctx, mut p_kind_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Font_CharSize", &PROBE_N, &[(p_kind_1) as u32], false); }
    let mut l_size_2: i32 = 0;
    let r_ = 'f: {
    's1: {
        let sv3: i32 = p_kind_1;
        match sv3 {
            0i32 => {
                l_size_2 = 2i32;
                break 's1;
            }
            1i32 => {
                l_size_2 = 1i32;
                break 's1;
            }
            _ => {
                l_size_2 = 2i32;
                break 's1;
            }
        }
    }
    break 'f l_size_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Font_CharSize", &PROBE_N, &[], true); }
    r_
}

pub fn Font_ReadChar(c: &mut Ctx, mut p_kind_1: i32, mut p_p_2: u32) -> u16 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Font_ReadChar", &PROBE_N, &[(p_kind_1) as u32, (p_p_2) as u32], false); }
    let r_ = 'f: {
    's1: {
        let sv3: i32 = p_kind_1;
        match sv3 {
            0i32 => {
                break 'f c.m.r16(p_p_2);
            }
            1i32 => {
                break 'f (((c.m.r8(p_p_2) as i8)) as u16);
            }
            _ => {
                break 'f c.m.r16(p_p_2);
            }
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Font_ReadChar", &PROBE_N, &[], true); }
    r_
}

pub fn Font_IsNewline(c: &mut Ctx, mut p_ch_1: u16) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Font_IsNewline", &PROBE_N, &[(p_ch_1) as u32], false); }
    let mut l_result_2: i32 = 0;
    let r_ = 'f: {
    l_result_2 = 0i32;
    if (((((((((((p_ch_1) as i32)) == (10i32)) as i32)) != 0) || (((((((p_ch_1) as i32)) == (13i32)) as i32)) != 0)) as i32)) != 0) {
        l_result_2 = 1i32;
    }
    break 'f l_result_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Font_IsNewline", &PROBE_N, &[], true); }
    r_
}

pub fn Font_CountDigits(c: &mut Ctx, mut p_value_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Font_CountDigits", &PROBE_N, &[(p_value_1) as u32], false); }
    let mut l_limit_2: i32 = 0;
    let mut l_n_3: i32 = 0;
    let r_ = 'f: {
    l_limit_2 = 10i32;
    {
        l_n_3 = 1i32;
        'l1: loop {
            if !(((((l_n_3) < (10i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if (((((p_value_1) < (l_limit_2)) as i32)) != 0) {
                    break 'l1;
                }
                { let r_ = 10i32; let n_ = (l_limit_2).wrapping_mul(r_); l_limit_2 = n_; };
            }
            { let o_ = l_n_3; let n_ = (o_).wrapping_add(1); l_n_3 = n_; };
        }
    }
    break 'f l_n_3;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Font_CountDigits", &PROBE_N, &[], true); }
    r_
}

pub fn Font_Pow10(c: &mut Ctx, mut p_digits_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Font_Pow10", &PROBE_N, &[(p_digits_1) as u32], false); }
    let mut l_result_2: i32 = 0;
    let mut l_i_3: i32 = 0;
    let r_ = 'f: {
    l_result_2 = 1i32;
    {
        l_i_3 = 1i32;
        'l1: loop {
            if !(((((l_i_3) < (p_digits_1)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let r_ = 10i32; let n_ = (l_result_2).wrapping_mul(r_); l_result_2 = n_; };
            }
            { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
        }
    }
    break 'f l_result_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Font_Pow10", &PROBE_N, &[], true); }
    r_
}

pub fn Font_FindGlyph(c: &mut Ctx, mut p_ch_1: u16, mut p_slotOut_2: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Font_FindGlyph", &PROBE_N, &[(p_ch_1) as u32, (p_slotOut_2) as u32], false); }
    let mut l_glyph_3: u32 = 0;
    let mut l_slot_4: u32 = 0;
    let mut l_i_5: i32 = 0;
    let r_ = 'f: {
    l_glyph_3 = ((0i32) as u32);
    l_slot_4 = c.m.r32(sym::gFontSlots);
    {
        l_i_5 = 0i32;
        'l1: loop {
            if !(((((l_i_5) < (7i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if (((((c.m.r32(l_slot_4)) != (((0i32) as u32))) as i32)) != 0) {
                    l_glyph_3 = F::Font_SearchGlyph(c, l_slot_4, p_ch_1);
                    if (((((l_glyph_3) != (((0i32) as u32))) as i32)) != 0) {
                        if (((((p_slotOut_2) != (((0i32) as u32))) as i32)) != 0) {
                            { let v_ = l_slot_4; let a_ = p_slotOut_2; c.m.w32(a_, v_); };
                        }
                        break 'l1;
                    }
                }
            }
            { { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; }; { let o_ = l_slot_4; let n_ = (o_).wrapping_add(20); l_slot_4 = n_; }; };
        }
    }
    if (((((((((l_glyph_3) == (((0i32) as u32))) as i32)) != 0) && (((((((p_ch_1) as i32)) == (160i32)) as i32)) != 0)) as i32)) != 0) {
        l_glyph_3 = F::Font_FindGlyph(c, ((32i32) as u16), p_slotOut_2);
    }
    break 'f l_glyph_3;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Font_FindGlyph", &PROBE_N, &[], true); }
    r_
}

pub fn Font_SearchGlyph(c: &mut Ctx, mut p_slot_1: u32, mut p_ch_2: u16) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Font_SearchGlyph", &PROBE_N, &[(p_slot_1) as u32, (p_ch_2) as u32], false); }
    let mut l_lo_3: i32 = 0;
    let mut l_hi_4: i32 = 0;
    let mut l_glyphs_5: u32 = 0;
    let mut l_glyph_6: u32 = 0;
    let mut l_mid_7: i32 = 0;
    let r_ = 'f: {
    l_lo_3 = 0i32;
    l_hi_4 = ({ let a_ = (c.m.r32(p_slot_1)).wrapping_add(24); (c.m.r32(a_) as i32) }).wrapping_sub(1i32);
    l_glyphs_5 = { let a_ = (c.m.r32(p_slot_1)).wrapping_add(20); c.m.r32(a_) };
    'l1: while (((((l_lo_3) <= (l_hi_4)) as i32)) != 0) {
        l_mid_7 = (l_lo_3).wrapping_add(rt::div_i32((l_hi_4).wrapping_sub(l_lo_3), 2i32));
        l_glyph_6 = ((((l_mid_7) as u32)).wrapping_mul(6u32)).wrapping_add(l_glyphs_5);
        if (((((((c.m.r16(l_glyph_6)) as i32)) < (((p_ch_2) as i32))) as i32)) != 0) {
            l_lo_3 = (l_mid_7).wrapping_add(1i32);
        } else {
            if (((((((p_ch_2) as i32)) < (((c.m.r16(l_glyph_6)) as i32))) as i32)) != 0) {
                l_hi_4 = (l_mid_7).wrapping_sub(1i32);
            } else {
                if (((((((c.m.r16(l_glyph_6)) as i32)) == (((p_ch_2) as i32))) as i32)) != 0) {
                    break 'f l_glyph_6;
                }
            }
        }
    }
    break 'f ((0i32) as u32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Font_SearchGlyph", &PROBE_N, &[], true); }
    r_
}

pub fn Font_AllocCmd(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Font_AllocCmd", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f (c.m.r32(sym::gFontCmds)).wrapping_add(((({ let a_ = sym::gFontCmdCount; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); o_ }) as u32)).wrapping_mul(84));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Font_AllocCmd", &PROBE_N, &[], true); }
    r_
}

pub fn Font_SetCmdCount(c: &mut Ctx, mut p_count_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Font_SetCmdCount", &PROBE_N, &[(p_count_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_count_1; let a_ = sym::gFontCmdCount; c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Font_SetCmdCount", &PROBE_N, &[], true); }
    r_
}

pub fn Font_NewCmd(c: &mut Ctx, mut p_arg_1: i32, mut p_digits_2: i32, mut p_x_3: i32, mut p_y_4: i32, mut p_homeX_5: i32, mut p_kind_6: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Font_NewCmd", &PROBE_N, &[(p_arg_1) as u32, (p_digits_2) as u32, (p_x_3) as u32, (p_y_4) as u32, (p_homeX_5) as u32, (p_kind_6) as u32], false); }
    let mut l_cmd_7: u32 = 0;
    let r_ = 'f: {
    l_cmd_7 = F::Font_AllocCmd(c);
    { let v_ = p_arg_1; let a_ = (l_cmd_7).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
    { let v_ = ((p_x_3) as i16); let a_ = (l_cmd_7).wrapping_add(8); c.m.w16(a_, (v_) as u16); };
    { let v_ = ((p_y_4) as i16); let a_ = (l_cmd_7).wrapping_add(10); c.m.w16(a_, (v_) as u16); };
    { let v_ = ((p_digits_2) as i16); let a_ = (l_cmd_7).wrapping_add(14); c.m.w16(a_, (v_) as u16); };
    { let v_ = ((p_homeX_5) as i16); let a_ = (l_cmd_7).wrapping_add(12); c.m.w16(a_, (v_) as u16); };
    { let v_ = p_kind_6; let a_ = l_cmd_7; c.m.w32(a_, (v_) as u32); };
    { let v_ = rt::fmul(F32(c.m.r32(sym::gFontStyle)), F32(c.m.r32((sym::gFontStyle).wrapping_add(8)))); let a_ = (l_cmd_7).wrapping_add(16); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fmul(F32(c.m.r32((sym::gFontStyle).wrapping_add(4))), F32(c.m.r32((sym::gFontStyle).wrapping_add(8)))); let a_ = (l_cmd_7).wrapping_add(20); c.m.w32(a_, (v_).0); };
    { let v_ = (((c.m.r32((sym::gFontStyle).wrapping_add(20)) as i32)) as u16); let a_ = (l_cmd_7).wrapping_add(24); c.m.w16(a_, v_); };
    { let v_ = (((c.m.r32((sym::gFontStyle).wrapping_add(24)) as i32)) as u16); let a_ = (l_cmd_7).wrapping_add(26); c.m.w16(a_, v_); };
    { let v_ = c.m.r32((sym::gFontStyle).wrapping_add(12)); let a_ = (l_cmd_7).wrapping_add(28); c.m.w32(a_, v_); };
    { let v_ = c.m.r32((sym::gFontStyle).wrapping_add(16)); let a_ = (l_cmd_7).wrapping_add(32); c.m.w32(a_, v_); };
    { let v_ = c.m.r32((sym::gFontStyle).wrapping_add(28)); let a_ = (l_cmd_7).wrapping_add(60); c.m.w32(a_, v_); };
    { let v_ = (c.m.r32((sym::gFontStyle).wrapping_add(32)) as i32); let a_ = (l_cmd_7).wrapping_add(64); c.m.w32(a_, (v_) as u32); };
    { let v_ = c.m.r16((sym::gFontStyle).wrapping_add(48)); let a_ = (l_cmd_7).wrapping_add(48); c.m.w16(a_, v_); };
    { let v_ = c.m.r16((sym::gFontStyle).wrapping_add(50)); let a_ = (l_cmd_7).wrapping_add(50); c.m.w16(a_, v_); };
    { let v_ = c.m.r16((sym::gFontStyle).wrapping_add(52)); let a_ = (l_cmd_7).wrapping_add(52); c.m.w16(a_, v_); };
    { let v_ = c.m.r16((sym::gFontStyle).wrapping_add(54)); let a_ = (l_cmd_7).wrapping_add(54); c.m.w16(a_, v_); };
    { let v_ = (c.m.r32((sym::gFontStyle).wrapping_add(56)) as i32); let a_ = (l_cmd_7).wrapping_add(56); c.m.w32(a_, (v_) as u32); };
    { let v_ = c.m.r32((sym::gFontStyle).wrapping_add(36)); let a_ = (l_cmd_7).wrapping_add(36); c.m.w32(a_, v_); };
    { let v_ = c.m.r32((sym::gFontStyle).wrapping_add(40)); let a_ = (l_cmd_7).wrapping_add(40); c.m.w32(a_, v_); };
    { let v_ = c.m.r16((sym::gFontStyle).wrapping_add(44)); let a_ = (l_cmd_7).wrapping_add(44); c.m.w16(a_, v_); };
    { let v_ = c.m.r16((sym::gFontStyle).wrapping_add(46)); let a_ = (l_cmd_7).wrapping_add(46); c.m.w16(a_, v_); };
    { let v_ = c.m.r32((sym::gFontStyle).wrapping_add(60)); let a_ = (l_cmd_7).wrapping_add(68); c.m.w32(a_, v_); };
    { let v_ = c.m.r32((sym::gFontStyle).wrapping_add(64)); let a_ = (l_cmd_7).wrapping_add(72); c.m.w32(a_, v_); };
    { let v_ = c.m.r32((sym::gFontStyle).wrapping_add(68)); let a_ = (l_cmd_7).wrapping_add(80); c.m.w32(a_, v_); };
    { let v_ = (1i32).wrapping_neg(); let a_ = (l_cmd_7).wrapping_add(76); c.m.w32(a_, (v_) as u32); };
    break 'f l_cmd_7;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Font_NewCmd", &PROBE_N, &[], true); }
    r_
}

pub fn Font_SetFont(c: &mut Ctx, mut p_slot_1: i32, mut p_data_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Font_SetFont", &PROBE_N, &[(p_slot_1) as u32, (p_data_2) as u32], false); }
    let mut l_tex_3: u32 = 0;
    let mut l_s_4: u32 = 0;
    let r_ = 'f: {
    if (((((c.m.r32((p_data_2).wrapping_add(12))) < (p_data_2)) as i32)) != 0) {
        l_tex_3 = (c.m.r32((p_data_2).wrapping_add(12))).wrapping_add(p_data_2);
        { let v_ = l_tex_3; let a_ = (p_data_2).wrapping_add(12); c.m.w32(a_, v_); };
        { let v_ = (c.m.r32((p_data_2).wrapping_add(20))).wrapping_add(p_data_2); let a_ = (p_data_2).wrapping_add(20); c.m.w32(a_, v_); };
        { let v_ = ((c.m.r32(l_tex_3) as i32)).wrapping_add(((p_data_2) as i32)); let a_ = (l_tex_3).wrapping_add(56); c.m.w32(a_, (v_) as u32); };
        { let v_ = ((c.m.r32((l_tex_3).wrapping_add(4)) as i32)).wrapping_add(((p_data_2) as i32)); let a_ = (l_tex_3).wrapping_add(60); c.m.w32(a_, (v_) as u32); };
    }
    l_s_4 = (c.m.r32(sym::gFontSlots)).wrapping_add((((p_slot_1) as u32)).wrapping_mul(20));
    { let v_ = ((p_slot_1).wrapping_mul(520i32)).wrapping_add(10752i32); let a_ = (l_s_4).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
    { let v_ = ((((p_slot_1).wrapping_mul(520i32)).wrapping_add(10760i32)) as u32); let a_ = (l_s_4).wrapping_add(4); c.m.w32(a_, v_); };
    { let v_ = p_data_2; let a_ = l_s_4; c.m.w32(a_, v_); };
    { let v_ = (c.m.r32((p_data_2).wrapping_add(28)) as i32); let a_ = (l_s_4).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
    { let v_ = (c.m.r32((p_data_2).wrapping_add(32)) as i32); let a_ = (l_s_4).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
    { let v_ = 0i32; let a_ = sym::gFontGlyphGap; c.m.w32(a_, (v_) as u32); };
    { let v_ = (c.m.r32((p_data_2).wrapping_add(32)) as i32); let a_ = sym::gFontLineHeight; c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Font_SetFont", &PROBE_N, &[], true); }
    r_
}

pub fn Font_DrawGlyph(c: &mut Ctx, mut p_pkt_1: u32, mut p_cmd_2: u32, mut p_x_3: i32, mut p_y_4: i32, mut p_ch_5: u16) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Font_DrawGlyph", &PROBE_N, &[(p_pkt_1) as u32, (p_cmd_2) as u32, (p_x_3) as u32, (p_y_4) as u32, (p_ch_5) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_color_6: u32 = 0;
    let mut l_shadowColor_7: u32 = 0;
    let mut l_glyph_8: u32 = 0;
    let mut l_tex0_9: u64 = 0;
    let mut l_u0_10: i32 = 0;
    let mut l_v0_11: i32 = 0;
    let mut l_u1_12: i32 = 0;
    let mut l_v1_13: i32 = 0;
    let mut l_w_14: i32 = 0;
    let mut l_h_15: i32 = 0;
    let mut l_x1_16: i32 = 0;
    let mut l_y1_17: i32 = 0;
    let mut l_sx0_18: i32 = 0;
    let mut l_sy0_19: i32 = 0;
    let mut l_sx1_20: i32 = 0;
    let mut l_sy1_21: i32 = 0;
    let mut l_dx_22: i32 = 0;
    let mut l_dy_23: i32 = 0;
    let mut l_i_24: i32 = 0;
    let r_ = 'f: {
        l_color_6 = c.m.r32((p_cmd_2).wrapping_add(28));
        l_shadowColor_7 = c.m.r32((p_cmd_2).wrapping_add(40));
        l_glyph_8 = F::Font_FindGlyph(c, p_ch_5, fp);
        if (((((l_glyph_8) == (((0i32) as u32))) as i32)) != 0) {
            break 'f 0i32;
        }
        l_sx0_18 = 0i32;
        l_sy0_19 = 0i32;
        l_w_14 = (((((c.m.r16((l_glyph_8).wrapping_add(2))) as i32)).wrapping_shr((10i32) as u32)) & (31i32));
        l_h_15 = (((((c.m.r16((l_glyph_8).wrapping_add(4))) as i32)).wrapping_shr((10i32) as u32)) & (31i32));
        l_u0_10 = ((((c.m.r16((l_glyph_8).wrapping_add(2))) as i32)) & (1023i32));
        l_v0_11 = ((((c.m.r16((l_glyph_8).wrapping_add(4))) as i32)) & (1023i32));
        l_x1_16 = (p_x_3).wrapping_add(rt::f2i(rt::fmul(rt::i2f(l_w_14), F32(c.m.r32((p_cmd_2).wrapping_add(16))))));
        l_y1_17 = (p_y_4).wrapping_add(rt::f2i(rt::fmul(rt::i2f(l_h_15), F32(c.m.r32((p_cmd_2).wrapping_add(20))))));
        l_u1_12 = (l_u0_10).wrapping_add(l_w_14);
        l_v1_13 = (l_v0_11).wrapping_add(l_h_15);
        l_sx1_20 = 0i32;
        l_sy1_21 = 0i32;
        l_tex0_9 = { let a0_ = c.m.r32(fp); let a1_ = (((((c.m.r16((l_glyph_8).wrapping_add(4))) as i32)).wrapping_shr((15i32) as u32)).wrapping_shl((1i32) as u32)).wrapping_add((((c.m.r16((l_glyph_8).wrapping_add(2))) as i32)).wrapping_shr((15i32) as u32)); F::Font_MakeTex0(c, a0_, a1_) };
        l_dx_22 = ((c.m.r16((p_cmd_2).wrapping_add(44))) as i32);
        l_dy_23 = ((c.m.r16((p_cmd_2).wrapping_add(46))) as i32);
        's1: {
            let sv25: u32 = c.m.r32((p_cmd_2).wrapping_add(36));
            match sv25 {
                1u32 => {
                    l_sx0_18 = (p_x_3).wrapping_add(l_dx_22);
                    l_sy0_19 = (p_y_4).wrapping_add(l_dy_23);
                    l_sx1_20 = (l_x1_16).wrapping_add(l_dx_22);
                    l_sy1_21 = (l_y1_17).wrapping_add(l_dy_23);
                    if ((((D::call(c, fid::Font_IsInClip, &[((p_cmd_2) as u64), ((l_sx0_18) as i64 as u64), ((l_sy0_19) as i64 as u64), ((l_sx1_20) as i64 as u64), ((l_sy1_21) as i64 as u64)])) as i32)) != 0) {
                        { let _ = D::call(c, fid::Font_PutSprite, &[((p_pkt_1) as u64), ((l_sx0_18) as i64 as u64), ((l_sy0_19) as i64 as u64), ((l_sx1_20) as i64 as u64), ((l_sy1_21) as i64 as u64), ((l_u0_10) as i64 as u64), ((l_v0_11) as i64 as u64), ((l_u1_12) as i64 as u64), ((l_v1_13) as i64 as u64), ((l_shadowColor_7) as u64), ((l_tex0_9) as u64)]); };
                    }
                    break 's1;
                }
                2u32 => {
                    {
                        l_i_24 = 0i32;
                        'l2: loop {
                            if !(((((l_i_24) < (8i32)) as i32)) != 0) { break 'l2; }
                            'c3: {
                                's4: {
                                    let sv26: i32 = l_i_24;
                                    match sv26 {
                                        0i32 => {
                                            l_sx0_18 = (p_x_3).wrapping_add(l_dx_22);
                                            l_sy0_19 = (p_y_4).wrapping_add(l_dy_23);
                                            l_sx1_20 = (l_x1_16).wrapping_add(l_dx_22);
                                            l_sy1_21 = (l_y1_17).wrapping_add(l_dy_23);
                                            break 's4;
                                        }
                                        1i32 => {
                                            l_sx0_18 = (p_x_3).wrapping_sub(l_dx_22);
                                            l_sy0_19 = (p_y_4).wrapping_add(l_dy_23);
                                            l_sx1_20 = (l_x1_16).wrapping_sub(l_dx_22);
                                            l_sy1_21 = (l_y1_17).wrapping_add(l_dy_23);
                                            break 's4;
                                        }
                                        2i32 => {
                                            l_sx0_18 = (p_x_3).wrapping_add(l_dx_22);
                                            l_sy0_19 = (p_y_4).wrapping_sub(l_dy_23);
                                            l_sx1_20 = (l_x1_16).wrapping_add(l_dx_22);
                                            l_sy1_21 = (l_y1_17).wrapping_sub(l_dy_23);
                                            break 's4;
                                        }
                                        3i32 => {
                                            l_sx0_18 = (p_x_3).wrapping_sub(l_dx_22);
                                            l_sy0_19 = (p_y_4).wrapping_sub(l_dy_23);
                                            l_sx1_20 = (l_x1_16).wrapping_sub(l_dx_22);
                                            l_sy1_21 = (l_y1_17).wrapping_sub(l_dy_23);
                                            break 's4;
                                        }
                                        4i32 => {
                                            l_sx0_18 = (p_x_3).wrapping_add(l_dx_22);
                                            l_sy0_19 = p_y_4;
                                            l_sx1_20 = (l_x1_16).wrapping_add(l_dx_22);
                                            l_sy1_21 = l_y1_17;
                                            break 's4;
                                        }
                                        5i32 => {
                                            l_sx0_18 = (p_x_3).wrapping_sub(l_dx_22);
                                            l_sy0_19 = p_y_4;
                                            l_sx1_20 = (l_x1_16).wrapping_sub(l_dx_22);
                                            l_sy1_21 = l_y1_17;
                                            break 's4;
                                        }
                                        6i32 => {
                                            l_sx0_18 = p_x_3;
                                            l_sy0_19 = (p_y_4).wrapping_add(l_dy_23);
                                            l_sx1_20 = l_x1_16;
                                            l_sy1_21 = (l_y1_17).wrapping_add(l_dy_23);
                                            break 's4;
                                        }
                                        7i32 => {
                                            l_sx0_18 = p_x_3;
                                            l_sy0_19 = (p_y_4).wrapping_sub(l_dy_23);
                                            l_sx1_20 = l_x1_16;
                                            l_sy1_21 = (l_y1_17).wrapping_sub(l_dy_23);
                                            break 's4;
                                        }
                                        _ => {
                                        }
                                    }
                                }
                                if ((((D::call(c, fid::Font_IsInClip, &[((p_cmd_2) as u64), ((l_sx0_18) as i64 as u64), ((l_sy0_19) as i64 as u64), ((l_sx1_20) as i64 as u64), ((l_sy1_21) as i64 as u64)])) as i32)) != 0) {
                                    { let _ = D::call(c, fid::Font_PutSprite, &[((p_pkt_1) as u64), ((l_sx0_18) as i64 as u64), ((l_sy0_19) as i64 as u64), ((l_sx1_20) as i64 as u64), ((l_sy1_21) as i64 as u64), ((l_u0_10) as i64 as u64), ((l_v0_11) as i64 as u64), ((l_u1_12) as i64 as u64), ((l_v1_13) as i64 as u64), ((l_shadowColor_7) as u64), ((l_tex0_9) as u64)]); };
                                }
                            }
                            { let o_ = l_i_24; let n_ = (o_).wrapping_add(1); l_i_24 = n_; };
                        }
                    }
                    break 's1;
                }
                3u32 => {
                    {
                        l_i_24 = 0i32;
                        'l5: loop {
                            if !(((((l_i_24) < (4i32)) as i32)) != 0) { break 'l5; }
                            'c6: {
                                's7: {
                                    let sv27: i32 = l_i_24;
                                    match sv27 {
                                        0i32 => {
                                            l_sx0_18 = (p_x_3).wrapping_sub(l_dx_22);
                                            l_sy0_19 = (p_y_4).wrapping_sub(l_dy_23);
                                            l_sx1_20 = (l_x1_16).wrapping_sub(l_dx_22);
                                            l_sy1_21 = (l_y1_17).wrapping_add(l_dy_23);
                                            break 's7;
                                        }
                                        1i32 => {
                                            l_sx0_18 = (p_x_3).wrapping_add(l_dx_22);
                                            l_sy0_19 = (p_y_4).wrapping_sub(l_dy_23);
                                            l_sx1_20 = (l_x1_16).wrapping_add(l_dx_22);
                                            l_sy1_21 = (l_y1_17).wrapping_add(l_dy_23);
                                            break 's7;
                                        }
                                        2i32 => {
                                            l_sx0_18 = (p_x_3).wrapping_sub(l_dx_22);
                                            l_sy0_19 = (p_y_4).wrapping_sub(l_dy_23);
                                            l_sx1_20 = (l_x1_16).wrapping_add(l_dx_22);
                                            l_sy1_21 = (l_y1_17).wrapping_sub(l_dy_23);
                                            break 's7;
                                        }
                                        3i32 => {
                                            l_sx0_18 = (p_x_3).wrapping_sub(l_dx_22);
                                            l_sy0_19 = (p_y_4).wrapping_add(l_dy_23);
                                            l_sx1_20 = (l_x1_16).wrapping_add(l_dx_22);
                                            l_sy1_21 = (l_y1_17).wrapping_add(l_dy_23);
                                            break 's7;
                                        }
                                        _ => {
                                        }
                                    }
                                }
                                if ((((D::call(c, fid::Font_IsInClip, &[((p_cmd_2) as u64), ((l_sx0_18) as i64 as u64), ((l_sy0_19) as i64 as u64), ((l_sx1_20) as i64 as u64), ((l_sy1_21) as i64 as u64)])) as i32)) != 0) {
                                    { let _ = D::call(c, fid::Font_PutSprite, &[((p_pkt_1) as u64), ((l_sx0_18) as i64 as u64), ((l_sy0_19) as i64 as u64), ((l_sx1_20) as i64 as u64), ((l_sy1_21) as i64 as u64), ((l_u0_10) as i64 as u64), ((l_v0_11) as i64 as u64), ((l_u1_12) as i64 as u64), ((l_v1_13) as i64 as u64), ((l_shadowColor_7) as u64), ((l_tex0_9) as u64)]); };
                                }
                            }
                            { let o_ = l_i_24; let n_ = (o_).wrapping_add(1); l_i_24 = n_; };
                        }
                    }
                    break 's1;
                }
                _ => {
                }
            }
        }
        if ((((D::call(c, fid::Font_IsInClip, &[((p_cmd_2) as u64), ((p_x_3) as i64 as u64), ((p_y_4) as i64 as u64), ((l_x1_16) as i64 as u64), ((l_y1_17) as i64 as u64)])) as i32)) != 0) {
            { let _ = D::call(c, fid::Font_PutSprite, &[((p_pkt_1) as u64), ((p_x_3) as i64 as u64), ((p_y_4) as i64 as u64), ((l_x1_16) as i64 as u64), ((l_y1_17) as i64 as u64), ((l_u0_10) as i64 as u64), ((l_v0_11) as i64 as u64), ((l_u1_12) as i64 as u64), ((l_v1_13) as i64 as u64), ((l_color_6) as u64), ((l_tex0_9) as u64)]); };
        }
        break 'f ((rt::f2i(rt::fmul(rt::i2f((((((c.m.r16((l_glyph_8).wrapping_add(2))) as i32)).wrapping_shr((10i32) as u32)) & (31i32))), F32(c.m.r32((p_cmd_2).wrapping_add(16)))))).wrapping_add((c.m.r32(sym::gFontGlyphGap) as i32))).wrapping_add(((c.m.r16((p_cmd_2).wrapping_add(24))) as i32));
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Font_DrawGlyph", &PROBE_N, &[], true); }
    r_
}
