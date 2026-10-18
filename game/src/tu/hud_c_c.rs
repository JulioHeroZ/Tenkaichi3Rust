#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn HudGfx_CallBegin(c: &mut Ctx, mut p_fn_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudGfx_CallBegin", &PROBE_N, &[(p_fn_1) as u32], false); }
    let r_ = 'f: {
    { let _ = D::call(c, p_fn_1, &[((0i32) as i64 as u64)]); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudGfx_CallBegin", &PROBE_N, &[], true); }
    r_
}

pub fn HudGfx_CallEnd(c: &mut Ctx, mut p_fn_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudGfx_CallEnd", &PROBE_N, &[(p_fn_1) as u32], false); }
    let r_ = 'f: {
    { let _ = D::call(c, p_fn_1, &[((0i32) as i64 as u64)]); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudGfx_CallEnd", &PROBE_N, &[], true); }
    r_
}

pub fn HudSprite_Show(c: &mut Ctx, mut p_spr_1: u32, mut p_show_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudSprite_Show", &PROBE_N, &[(p_spr_1) as u32, (p_show_2) as u32], false); }
    let r_ = 'f: {
    { let v_ = (((((p_show_2) == (0i32)) as i32)) as u32); let a_ = p_spr_1; rt::bf_set(c, a_, 0, 1, (v_) as u64);  };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudSprite_Show", &PROBE_N, &[], true); }
    r_
}

pub fn HudSprite_SetMirror(c: &mut Ctx, mut p_spr_1: u32, mut p_mirror_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudSprite_SetMirror", &PROBE_N, &[(p_spr_1) as u32, (p_mirror_2) as u32], false); }
    let r_ = 'f: {
    { let v_ = ((p_mirror_2) as u32); let a_ = p_spr_1; rt::bf_set(c, a_, 1, 1, (v_) as u64);  };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudSprite_SetMirror", &PROBE_N, &[], true); }
    r_
}

pub fn HudSprite_SetRect(c: &mut Ctx, mut p_spr_1: u32, mut p_x0_2: i32, mut p_x1_3: i32, mut p_y0_4: i32, mut p_y1_5: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudSprite_SetRect", &PROBE_N, &[(p_spr_1) as u32, (p_x0_2) as u32, (p_x1_3) as u32, (p_y0_4) as u32, (p_y1_5) as u32], false); }
    let r_ = 'f: {
    { let v_ = ((p_x0_2) as i16); let a_ = (p_spr_1).wrapping_add(8); c.m.w16(a_, (v_) as u16); };
    { let v_ = ((p_x1_3) as i16); let a_ = (p_spr_1).wrapping_add(10); c.m.w16(a_, (v_) as u16); };
    { let v_ = ((p_y0_4) as i16); let a_ = (p_spr_1).wrapping_add(12); c.m.w16(a_, (v_) as u16); };
    { let v_ = ((p_y1_5) as i16); let a_ = (p_spr_1).wrapping_add(14); c.m.w16(a_, (v_) as u16); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudSprite_SetRect", &PROBE_N, &[], true); }
    r_
}

pub fn HudSprite_SetUv(c: &mut Ctx, mut p_spr_1: u32, mut p_u0_2: i32, mut p_u1_3: i32, mut p_v0_4: i32, mut p_v1_5: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudSprite_SetUv", &PROBE_N, &[(p_spr_1) as u32, (p_u0_2) as u32, (p_u1_3) as u32, (p_v0_4) as u32, (p_v1_5) as u32], false); }
    let r_ = 'f: {
    { let v_ = ((p_u0_2) as i16); let a_ = (p_spr_1).wrapping_add(16); c.m.w16(a_, (v_) as u16); };
    { let v_ = ((p_u1_3) as i16); let a_ = (p_spr_1).wrapping_add(18); c.m.w16(a_, (v_) as u16); };
    { let v_ = ((p_v0_4) as i16); let a_ = (p_spr_1).wrapping_add(20); c.m.w16(a_, (v_) as u16); };
    { let v_ = ((p_v1_5) as i16); let a_ = (p_spr_1).wrapping_add(22); c.m.w16(a_, (v_) as u16); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudSprite_SetUv", &PROBE_N, &[], true); }
    r_
}

pub fn HudSprite_SetTex(c: &mut Ctx, mut p_spr_1: u32, mut p_tex_2: i32, mut p_sub_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudSprite_SetTex", &PROBE_N, &[(p_spr_1) as u32, (p_tex_2) as u32, (p_sub_3) as u32], false); }
    let r_ = 'f: {
    { let v_ = ((p_tex_2) as i16); let a_ = (p_spr_1).wrapping_add(4); c.m.w16(a_, (v_) as u16); };
    { let v_ = ((p_sub_3) as u16); let a_ = (p_spr_1).wrapping_add(6); c.m.w16(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudSprite_SetTex", &PROBE_N, &[], true); }
    r_
}

pub fn HudSprite_SetColor(c: &mut Ctx, mut p_spr_1: u32, mut p_r_2: i32, mut p_g_3: i32, mut p_b_4: i32, mut p_a_5: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudSprite_SetColor", &PROBE_N, &[(p_spr_1) as u32, (p_r_2) as u32, (p_g_3) as u32, (p_b_4) as u32, (p_a_5) as u32], false); }
    let r_ = 'f: {
    { let v_ = ((p_r_2) as u8); let a_ = (p_spr_1).wrapping_add(24); c.m.w8(a_, v_); };
    { let v_ = ((p_g_3) as u8); let a_ = (p_spr_1).wrapping_add(25); c.m.w8(a_, v_); };
    { let v_ = ((p_b_4) as u8); let a_ = (p_spr_1).wrapping_add(26); c.m.w8(a_, v_); };
    { let v_ = ((p_a_5) as u8); let a_ = (p_spr_1).wrapping_add(27); c.m.w8(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudSprite_SetColor", &PROBE_N, &[], true); }
    r_
}

pub fn HudSprite_InitPlain(c: &mut Ctx, mut p_spr_1: u32, mut p_r_2: u8, mut p_g_3: u8, mut p_b_4: u8, mut p_a_5: u8) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudSprite_InitPlain", &PROBE_N, &[(p_spr_1) as u32, (p_r_2) as u32, (p_g_3) as u32, (p_b_4) as u32, (p_a_5) as u32], false); }
    let r_ = 'f: {
    { let v_ = (((1i32).wrapping_neg()) as i16); let a_ = (p_spr_1).wrapping_add(4); c.m.w16(a_, (v_) as u16); };
    { let _ = F::memset(c, (p_spr_1).wrapping_add(16), 0i32, ((8i32) as u32)); };
    { let v_ = p_r_2; let a_ = (p_spr_1).wrapping_add(24); c.m.w8(a_, v_); };
    { let v_ = p_g_3; let a_ = (p_spr_1).wrapping_add(25); c.m.w8(a_, v_); };
    { let v_ = p_b_4; let a_ = (p_spr_1).wrapping_add(26); c.m.w8(a_, v_); };
    { let v_ = p_a_5; let a_ = (p_spr_1).wrapping_add(27); c.m.w8(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudSprite_InitPlain", &PROBE_N, &[], true); }
    r_
}

pub fn HudSprite_Move(c: &mut Ctx, mut p_spr_1: u32, mut p_dx_2: i32, mut p_dy_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudSprite_Move", &PROBE_N, &[(p_spr_1) as u32, (p_dx_2) as u32, (p_dy_3) as u32], false); }
    let r_ = 'f: {
    { let r_ = p_dx_2; let a_ = (p_spr_1).wrapping_add(8); let n_ = ((((((c.m.r16(a_) as i16)) as i32)).wrapping_add(r_)) as i16); c.m.w16(a_, (n_) as u16); };
    { let r_ = p_dx_2; let a_ = (p_spr_1).wrapping_add(10); let n_ = ((((((c.m.r16(a_) as i16)) as i32)).wrapping_add(r_)) as i16); c.m.w16(a_, (n_) as u16); };
    { let r_ = p_dy_3; let a_ = (p_spr_1).wrapping_add(12); let n_ = ((((((c.m.r16(a_) as i16)) as i32)).wrapping_add(r_)) as i16); c.m.w16(a_, (n_) as u16); };
    { let r_ = p_dy_3; let a_ = (p_spr_1).wrapping_add(14); let n_ = ((((((c.m.r16(a_) as i16)) as i32)).wrapping_add(r_)) as i16); c.m.w16(a_, (n_) as u16); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudSprite_Move", &PROBE_N, &[], true); }
    r_
}

pub fn HudSprite_MoveUv(c: &mut Ctx, mut p_spr_1: u32, mut p_du_2: i32, mut p_dv_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudSprite_MoveUv", &PROBE_N, &[(p_spr_1) as u32, (p_du_2) as u32, (p_dv_3) as u32], false); }
    let r_ = 'f: {
    { let r_ = p_du_2; let a_ = (p_spr_1).wrapping_add(16); let n_ = ((((((c.m.r16(a_) as i16)) as i32)).wrapping_add(r_)) as i16); c.m.w16(a_, (n_) as u16); };
    { let r_ = p_du_2; let a_ = (p_spr_1).wrapping_add(18); let n_ = ((((((c.m.r16(a_) as i16)) as i32)).wrapping_add(r_)) as i16); c.m.w16(a_, (n_) as u16); };
    { let r_ = p_dv_3; let a_ = (p_spr_1).wrapping_add(20); let n_ = ((((((c.m.r16(a_) as i16)) as i32)).wrapping_add(r_)) as i16); c.m.w16(a_, (n_) as u16); };
    { let r_ = p_dv_3; let a_ = (p_spr_1).wrapping_add(22); let n_ = ((((((c.m.r16(a_) as i16)) as i32)).wrapping_add(r_)) as i16); c.m.w16(a_, (n_) as u16); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudSprite_MoveUv", &PROBE_N, &[], true); }
    r_
}

pub fn HudSprite_FlipU(c: &mut Ctx, mut p_spr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudSprite_FlipU", &PROBE_N, &[(p_spr_1) as u32], false); }
    let mut l_t_2: i16 = 0;
    let r_ = 'f: {
    l_t_2 = (c.m.r16((p_spr_1).wrapping_add(16)) as i16);
    { let v_ = (c.m.r16((p_spr_1).wrapping_add(18)) as i16); let a_ = (p_spr_1).wrapping_add(16); c.m.w16(a_, (v_) as u16); };
    { let v_ = l_t_2; let a_ = (p_spr_1).wrapping_add(18); c.m.w16(a_, (v_) as u16); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudSprite_FlipU", &PROBE_N, &[], true); }
    r_
}

pub fn HudSprite_FlipV(c: &mut Ctx, mut p_spr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudSprite_FlipV", &PROBE_N, &[(p_spr_1) as u32], false); }
    let mut l_t_2: i16 = 0;
    let r_ = 'f: {
    l_t_2 = (c.m.r16((p_spr_1).wrapping_add(20)) as i16);
    { let v_ = (c.m.r16((p_spr_1).wrapping_add(22)) as i16); let a_ = (p_spr_1).wrapping_add(20); c.m.w16(a_, (v_) as u16); };
    { let v_ = l_t_2; let a_ = (p_spr_1).wrapping_add(22); c.m.w16(a_, (v_) as u16); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudSprite_FlipV", &PROBE_N, &[], true); }
    r_
}

pub fn HudSprite_Scale(c: &mut Ctx, mut p_spr_1: u32, mut p_sx_2: F32, mut p_sy_3: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudSprite_Scale", &PROBE_N, &[(p_spr_1) as u32, (p_sx_2).0, (p_sy_3).0], false); }
    let r_ = 'f: {
    { let v_ = ((rt::f2i(rt::fmul(rt::i2f((((c.m.r16((p_spr_1).wrapping_add(8)) as i16)) as i32)), p_sx_2))) as i16); let a_ = (p_spr_1).wrapping_add(8); c.m.w16(a_, (v_) as u16); };
    { let v_ = ((rt::f2i(rt::fmul(rt::i2f((((c.m.r16((p_spr_1).wrapping_add(10)) as i16)) as i32)), p_sx_2))) as i16); let a_ = (p_spr_1).wrapping_add(10); c.m.w16(a_, (v_) as u16); };
    { let v_ = ((rt::f2i(rt::fmul(rt::i2f((((c.m.r16((p_spr_1).wrapping_add(12)) as i16)) as i32)), p_sy_3))) as i16); let a_ = (p_spr_1).wrapping_add(12); c.m.w16(a_, (v_) as u16); };
    { let v_ = ((rt::f2i(rt::fmul(rt::i2f((((c.m.r16((p_spr_1).wrapping_add(14)) as i16)) as i32)), p_sy_3))) as i16); let a_ = (p_spr_1).wrapping_add(14); c.m.w16(a_, (v_) as u16); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudSprite_Scale", &PROBE_N, &[], true); }
    r_
}

pub fn HudSprite_Center(c: &mut Ctx, mut p_spr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudSprite_Center", &PROBE_N, &[(p_spr_1) as u32], false); }
    let mut l_cx_2: i32 = 0;
    let mut l_cy_3: i32 = 0;
    let r_ = 'f: {
    l_cx_2 = rt::div_i32(((((c.m.r16((p_spr_1).wrapping_add(8)) as i16)) as i32)).wrapping_add((((c.m.r16((p_spr_1).wrapping_add(10)) as i16)) as i32)), 2i32);
    l_cy_3 = rt::div_i32(((((c.m.r16((p_spr_1).wrapping_add(12)) as i16)) as i32)).wrapping_add((((c.m.r16((p_spr_1).wrapping_add(14)) as i16)) as i32)), 2i32);
    { let r_ = l_cx_2; let a_ = (p_spr_1).wrapping_add(8); let n_ = ((((((c.m.r16(a_) as i16)) as i32)).wrapping_sub(r_)) as i16); c.m.w16(a_, (n_) as u16); };
    { let r_ = l_cx_2; let a_ = (p_spr_1).wrapping_add(10); let n_ = ((((((c.m.r16(a_) as i16)) as i32)).wrapping_sub(r_)) as i16); c.m.w16(a_, (n_) as u16); };
    { let r_ = l_cy_3; let a_ = (p_spr_1).wrapping_add(12); let n_ = ((((((c.m.r16(a_) as i16)) as i32)).wrapping_sub(r_)) as i16); c.m.w16(a_, (n_) as u16); };
    { let r_ = l_cy_3; let a_ = (p_spr_1).wrapping_add(14); let n_ = ((((((c.m.r16(a_) as i16)) as i32)).wrapping_sub(r_)) as i16); c.m.w16(a_, (n_) as u16); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudSprite_Center", &PROBE_N, &[], true); }
    r_
}

pub fn HudSprite_InitTex(c: &mut Ctx, mut p_spr_1: u32, mut p_res_2: u32, mut p_tex_3: i32, mut p_sub_4: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudSprite_InitTex", &PROBE_N, &[(p_spr_1) as u32, (p_res_2) as u32, (p_tex_3) as u32, (p_sub_4) as u32], false); }
    let mut l_t_5: u32 = 0;
    let mut l_w_6: i32 = 0;
    let mut l_h_7: i32 = 0;
    let r_ = 'f: {
    l_t_5 = (c.m.r32((p_res_2).wrapping_add(16))).wrapping_add((((p_tex_3) as u32)).wrapping_mul(64));
    if (((((l_t_5) != (((0i32) as u32))) as i32)) != 0) {
        l_w_6 = (1i32).wrapping_shl(((((((c.m.r64((l_t_5).wrapping_add(48))).wrapping_shr((26i32) as u32)) & (((15i32) as u64)))) as i32)) as u32);
        l_h_7 = (1i32).wrapping_shl(((((((c.m.r64((l_t_5).wrapping_add(48))).wrapping_shr((30i32) as u32)) & (((15i32) as u64)))) as i32)) as u32);
        F::HudSprite_SetTex(c, p_spr_1, p_tex_3, p_sub_4);
        F::HudSprite_SetRect(c, p_spr_1, 0i32, l_w_6, 0i32, l_h_7);
        F::HudSprite_SetUv(c, p_spr_1, 0i32, l_w_6, 0i32, l_h_7);
        F::HudSprite_SetColor(c, p_spr_1, 128i32, 128i32, 128i32, 128i32);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudSprite_InitTex", &PROBE_N, &[], true); }
    r_
}

pub fn HudRes_UploadTex(c: &mut Ctx, mut p_res_1: u32, mut p_tex_2: i32, mut p_sub_3: i32, mut p_tbp_4: i32, mut p_cbp_5: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudRes_UploadTex", &PROBE_N, &[(p_res_1) as u32, (p_tex_2) as u32, (p_sub_3) as u32, (p_tbp_4) as u32, (p_cbp_5) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let mut l_t_6: u32 = 0;
    let mut l_imageWidth_7: i32 = 0;
    let mut l_clutWidth_8: i32 = 0;
    let mut l_block_9: i32 = 0;
    let mut l_block_10: i32 = 0;
    let mut l_block_11: i32 = 0;
    let r_ = 'f: {
        rt::memset(c, fp, 0, 48);
        { let v_ = ((268435458i32) as u32); c.m.w32(fp.wrapping_add(0), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(4), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(8), v_); }
        { let v_ = ((1342177282i32) as u32); c.m.w32(fp.wrapping_add(12), v_); }
        { let v_ = ((32769i32) as u32); c.m.w32(fp.wrapping_add(16), v_); }
        { let v_ = ((268435456i32) as u32); c.m.w32(fp.wrapping_add(20), v_); }
        { let v_ = ((14i32) as u32); c.m.w32(fp.wrapping_add(24), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(28), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(32), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(36), v_); }
        { let v_ = ((80i32) as u32); c.m.w32(fp.wrapping_add(40), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(44), v_); }
        l_t_6 = (c.m.r32((p_res_1).wrapping_add(16))).wrapping_add((((p_tex_2) as u32)).wrapping_mul(64));
        l_imageWidth_7 = (c.m.r32((l_t_6).wrapping_add(24)) as i32);
        l_clutWidth_8 = (c.m.r32((l_t_6).wrapping_add(28)) as i32);
        if ((((!(((c.m.r32((l_t_6).wrapping_add(40)) as i32)) != 0)) as i32)) != 0) {
            { let v_ = 1i32; let a_ = (l_t_6).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
            if ((((((c.m.r32((l_t_6).wrapping_add(8)) as i32)) != (0i32)) as i32)) != 0) {
                l_block_9 = ((c.m.r32((l_t_6).wrapping_add(32)) as i32)).wrapping_add(p_tbp_4);
                { let v_ = (((((((((l_block_9) as i64)).wrapping_shl((32i32) as u32)) as u64)) | ((((l_imageWidth_7) as u64)).wrapping_shl((48i32) as u32)))) as u32); let a_ = (fp).wrapping_add((((8i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                { let v_ = ((((((((((l_block_9) as i64)).wrapping_shl((32i32) as u32)) as u64)) | ((((l_imageWidth_7) as u64)).wrapping_shl((48i32) as u32)))).wrapping_shr((32i32) as u32)) as u32); let a_ = (fp).wrapping_add((((9i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                F::Dma_AddData(c, fp, ((48u32) as i32));
                { let a0_ = c.m.r32((l_t_6).wrapping_add(56)); let a1_ = (c.m.r32((l_t_6).wrapping_add(8)) as i32); F::Dma_AddRef(c, a0_, a1_) };
            }
            if ((((((c.m.r32((l_t_6).wrapping_add(12)) as i32)) != (0i32)) as i32)) != 0) {
                l_block_10 = ((c.m.r32((l_t_6).wrapping_add(36)) as i32)).wrapping_add(p_cbp_5);
                { let v_ = (((((((((l_block_10) as i64)).wrapping_shl((32i32) as u32)) as u64)) | ((((l_clutWidth_8) as u64)).wrapping_shl((48i32) as u32)))) as u32); let a_ = (fp).wrapping_add((((8i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                { let v_ = ((((((((((l_block_10) as i64)).wrapping_shl((32i32) as u32)) as u64)) | ((((l_clutWidth_8) as u64)).wrapping_shl((48i32) as u32)))).wrapping_shr((32i32) as u32)) as u32); let a_ = (fp).wrapping_add((((9i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                F::Dma_AddData(c, fp, ((48u32) as i32));
                { let a0_ = c.m.r32((l_t_6).wrapping_add(60)); let a1_ = (c.m.r32((l_t_6).wrapping_add(12)) as i32); F::Dma_AddRef(c, a0_, a1_) };
            }
        }
        l_t_6 = (c.m.r32((p_res_1).wrapping_add(16))).wrapping_add(((((p_tex_2).wrapping_add(p_sub_3)) as u32)).wrapping_mul(64));
        if ((((!(((c.m.r32((l_t_6).wrapping_add(40)) as i32)) != 0)) as i32)) != 0) {
            { let v_ = 1i32; let a_ = (l_t_6).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
            if ((((((c.m.r32((l_t_6).wrapping_add(12)) as i32)) != (0i32)) as i32)) != 0) {
                l_block_11 = ((c.m.r32((l_t_6).wrapping_add(36)) as i32)).wrapping_add(p_cbp_5);
                { let v_ = (((((((((l_block_11) as i64)).wrapping_shl((32i32) as u32)) as u64)) | ((((l_clutWidth_8) as u64)).wrapping_shl((48i32) as u32)))) as u32); let a_ = (fp).wrapping_add((((8i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                { let v_ = ((((((((((l_block_11) as i64)).wrapping_shl((32i32) as u32)) as u64)) | ((((l_clutWidth_8) as u64)).wrapping_shl((48i32) as u32)))).wrapping_shr((32i32) as u32)) as u32); let a_ = (fp).wrapping_add((((9i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                F::Dma_AddData(c, fp, ((48u32) as i32));
                { let a0_ = c.m.r32((l_t_6).wrapping_add(60)); let a1_ = (c.m.r32((l_t_6).wrapping_add(12)) as i32); F::Dma_AddRef(c, a0_, a1_) };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "HudRes_UploadTex", &PROBE_N, &[], true); }
    r_
}

pub fn HudSprite_DrawAlphaClear(c: &mut Ctx, mut p_spr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudSprite_DrawAlphaClear", &PROBE_N, &[(p_spr_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(64) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 64);
    let mut l_p_2: u32 = 0;
    let r_ = 'f: {
        { let v_ = rt::i2f(((c.m.r16((p_spr_1).wrapping_add(8)) as i16)) as i32); let a_ = ((fp).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::i2f(((c.m.r16((p_spr_1).wrapping_add(12)) as i16)) as i32); let a_ = ((fp).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = ((fp).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::i2f(((c.m.r16((p_spr_1).wrapping_add(10)) as i16)) as i32); let a_ = ((fp).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::i2f(((c.m.r16((p_spr_1).wrapping_add(12)) as i16)) as i32); let a_ = ((fp).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = ((fp).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::i2f(((c.m.r16((p_spr_1).wrapping_add(8)) as i16)) as i32); let a_ = ((fp).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::i2f(((c.m.r16((p_spr_1).wrapping_add(14)) as i16)) as i32); let a_ = ((fp).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = ((fp).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::i2f(((c.m.r16((p_spr_1).wrapping_add(10)) as i16)) as i32); let a_ = ((fp).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::i2f(((c.m.r16((p_spr_1).wrapping_add(14)) as i16)) as i32); let a_ = ((fp).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = ((fp).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        F::Vu0Cur_MulVec4(c, (fp).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (fp).wrapping_add((((0i32) as u32)).wrapping_mul(16)));
        F::Vu0Cur_MulVec4(c, (fp).wrapping_add((((1i32) as u32)).wrapping_mul(16)), (fp).wrapping_add((((1i32) as u32)).wrapping_mul(16)));
        F::Vu0Cur_MulVec4(c, (fp).wrapping_add((((2i32) as u32)).wrapping_mul(16)), (fp).wrapping_add((((2i32) as u32)).wrapping_mul(16)));
        F::Vu0Cur_MulVec4(c, (fp).wrapping_add((((3i32) as u32)).wrapping_mul(16)), (fp).wrapping_add((((3i32) as u32)).wrapping_mul(16)));
        { let v_ = rt::i2f({ let a0_ = rt::f2i(F32(c.m.r32(((fp).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); F::Sprite_ConvY(c, a0_) }); let a_ = ((fp).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::i2f({ let a0_ = rt::f2i(F32(c.m.r32(((fp).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); F::Sprite_ConvY(c, a0_) }); let a_ = ((fp).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::i2f({ let a0_ = rt::f2i(F32(c.m.r32(((fp).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); F::Sprite_ConvY(c, a0_) }); let a_ = ((fp).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::i2f({ let a0_ = rt::f2i(F32(c.m.r32(((fp).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); F::Sprite_ConvY(c, a0_) }); let a_ = ((fp).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        l_p_2 = F::Dma_BeginDirect(c);
        { let v_ = ((((((((5i32) as u64)) | ((((0i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (l_p_2).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((14i32) as u64); let a_ = (l_p_2).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_2).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_2 = n_; };
        { let v_ = ((68i32) as u64); let a_ = (l_p_2).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((66i32) as u64); let a_ = (l_p_2).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_2).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_2 = n_; };
        { let v_ = ((196608i32) as u64); let a_ = (l_p_2).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((71i32) as u64); let a_ = (l_p_2).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_2).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_2 = n_; };
        { let v_ = ((0i32) as u64); let a_ = (l_p_2).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((74i32) as u64); let a_ = (l_p_2).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_2).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_2 = n_; };
        { let v_ = ((4575657221408423936i64) as u64); let a_ = (l_p_2).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((1i32) as u64); let a_ = (l_p_2).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_2).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_2 = n_; };
        { let v_ = ((68i32) as u64); let a_ = (l_p_2).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((0i32) as u64); let a_ = (l_p_2).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_2).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_2 = n_; };
        { let v_ = ((((((((1i32) as u64)) | ((((1i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((4i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (l_p_2).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((21845i32) as u64); let a_ = (l_p_2).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_2).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_2 = n_; };
        { let v_ = ((((((rt::f2i(F32(c.m.r32(((fp).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))))).wrapping_shl((4i32) as u32)).wrapping_add(28672i32)) as u64)) | (((((({ let a0_ = rt::f2i(F32(c.m.r32(((fp).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); F::Sprite_ConvY(c, a0_) }).wrapping_shl((4i32) as u32)).wrapping_add(29184i32)) as u64)).wrapping_shl((16i32) as u32))); let a_ = (l_p_2).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((((((rt::f2i(F32(c.m.r32(((fp).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))))).wrapping_shl((4i32) as u32)).wrapping_add(28672i32)) as u64)) | (((((({ let a0_ = rt::f2i(F32(c.m.r32(((fp).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); F::Sprite_ConvY(c, a0_) }).wrapping_shl((4i32) as u32)).wrapping_add(29184i32)) as u64)).wrapping_shl((16i32) as u32))); let a_ = (l_p_2).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_2).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_2 = n_; };
        { let v_ = ((((((rt::f2i(F32(c.m.r32(((fp).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))))).wrapping_shl((4i32) as u32)).wrapping_add(28672i32)) as u64)) | (((((({ let a0_ = rt::f2i(F32(c.m.r32(((fp).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); F::Sprite_ConvY(c, a0_) }).wrapping_shl((4i32) as u32)).wrapping_add(29184i32)) as u64)).wrapping_shl((16i32) as u32))); let a_ = (l_p_2).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((((((rt::f2i(F32(c.m.r32(((fp).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))))).wrapping_shl((4i32) as u32)).wrapping_add(28672i32)) as u64)) | (((((({ let a0_ = rt::f2i(F32(c.m.r32(((fp).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); F::Sprite_ConvY(c, a0_) }).wrapping_shl((4i32) as u32)).wrapping_add(29184i32)) as u64)).wrapping_shl((16i32) as u32))); let a_ = (l_p_2).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_2).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_2 = n_; };
        F::Dma_EndDirect(c, l_p_2);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "HudSprite_DrawAlphaClear", &PROBE_N, &[], true); }
    r_
}

pub fn HudSprite_DrawAlphaSet(c: &mut Ctx, mut p_spr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudSprite_DrawAlphaSet", &PROBE_N, &[(p_spr_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(64) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 64);
    let mut l_p_2: u32 = 0;
    let r_ = 'f: {
        { let v_ = rt::i2f(((c.m.r16((p_spr_1).wrapping_add(8)) as i16)) as i32); let a_ = ((fp).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::i2f(((c.m.r16((p_spr_1).wrapping_add(12)) as i16)) as i32); let a_ = ((fp).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = ((fp).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::i2f(((c.m.r16((p_spr_1).wrapping_add(10)) as i16)) as i32); let a_ = ((fp).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::i2f(((c.m.r16((p_spr_1).wrapping_add(12)) as i16)) as i32); let a_ = ((fp).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = ((fp).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::i2f(((c.m.r16((p_spr_1).wrapping_add(8)) as i16)) as i32); let a_ = ((fp).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::i2f(((c.m.r16((p_spr_1).wrapping_add(14)) as i16)) as i32); let a_ = ((fp).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = ((fp).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::i2f(((c.m.r16((p_spr_1).wrapping_add(10)) as i16)) as i32); let a_ = ((fp).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::i2f(((c.m.r16((p_spr_1).wrapping_add(14)) as i16)) as i32); let a_ = ((fp).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = ((fp).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        F::Vu0Cur_MulVec4(c, (fp).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (fp).wrapping_add((((0i32) as u32)).wrapping_mul(16)));
        F::Vu0Cur_MulVec4(c, (fp).wrapping_add((((1i32) as u32)).wrapping_mul(16)), (fp).wrapping_add((((1i32) as u32)).wrapping_mul(16)));
        F::Vu0Cur_MulVec4(c, (fp).wrapping_add((((2i32) as u32)).wrapping_mul(16)), (fp).wrapping_add((((2i32) as u32)).wrapping_mul(16)));
        F::Vu0Cur_MulVec4(c, (fp).wrapping_add((((3i32) as u32)).wrapping_mul(16)), (fp).wrapping_add((((3i32) as u32)).wrapping_mul(16)));
        { let v_ = rt::i2f({ let a0_ = rt::f2i(F32(c.m.r32(((fp).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); F::Sprite_ConvY(c, a0_) }); let a_ = ((fp).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::i2f({ let a0_ = rt::f2i(F32(c.m.r32(((fp).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); F::Sprite_ConvY(c, a0_) }); let a_ = ((fp).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::i2f({ let a0_ = rt::f2i(F32(c.m.r32(((fp).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); F::Sprite_ConvY(c, a0_) }); let a_ = ((fp).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::i2f({ let a0_ = rt::f2i(F32(c.m.r32(((fp).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); F::Sprite_ConvY(c, a0_) }); let a_ = ((fp).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        l_p_2 = F::Dma_BeginDirect(c);
        { let v_ = ((((((((5i32) as u64)) | ((((0i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (l_p_2).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((14i32) as u64); let a_ = (l_p_2).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_2).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_2 = n_; };
        { let v_ = ((68i32) as u64); let a_ = (l_p_2).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((66i32) as u64); let a_ = (l_p_2).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_2).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_2 = n_; };
        { let v_ = ((196608i32) as u64); let a_ = (l_p_2).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((71i32) as u64); let a_ = (l_p_2).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_2).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_2 = n_; };
        { let v_ = ((0i32) as u64); let a_ = (l_p_2).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((74i32) as u64); let a_ = (l_p_2).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_2).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_2 = n_; };
        { let v_ = ((4575657225686614016i64) as u64); let a_ = (l_p_2).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((1i32) as u64); let a_ = (l_p_2).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_2).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_2 = n_; };
        { let v_ = ((68i32) as u64); let a_ = (l_p_2).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((0i32) as u64); let a_ = (l_p_2).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_2).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_2 = n_; };
        { let v_ = ((((((((1i32) as u64)) | ((((1i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((4i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (l_p_2).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((21845i32) as u64); let a_ = (l_p_2).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_2).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_2 = n_; };
        { let v_ = ((((((rt::f2i(F32(c.m.r32(((fp).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))))).wrapping_shl((4i32) as u32)).wrapping_add(28672i32)) as u64)) | (((((({ let a0_ = rt::f2i(F32(c.m.r32(((fp).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); F::Sprite_ConvY(c, a0_) }).wrapping_shl((4i32) as u32)).wrapping_add(29184i32)) as u64)).wrapping_shl((16i32) as u32))); let a_ = (l_p_2).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((((((rt::f2i(F32(c.m.r32(((fp).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))))).wrapping_shl((4i32) as u32)).wrapping_add(28672i32)) as u64)) | (((((({ let a0_ = rt::f2i(F32(c.m.r32(((fp).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); F::Sprite_ConvY(c, a0_) }).wrapping_shl((4i32) as u32)).wrapping_add(29184i32)) as u64)).wrapping_shl((16i32) as u32))); let a_ = (l_p_2).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_2).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_2 = n_; };
        { let v_ = ((((((rt::f2i(F32(c.m.r32(((fp).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))))).wrapping_shl((4i32) as u32)).wrapping_add(28672i32)) as u64)) | (((((({ let a0_ = rt::f2i(F32(c.m.r32(((fp).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); F::Sprite_ConvY(c, a0_) }).wrapping_shl((4i32) as u32)).wrapping_add(29184i32)) as u64)).wrapping_shl((16i32) as u32))); let a_ = (l_p_2).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((((((rt::f2i(F32(c.m.r32(((fp).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))))).wrapping_shl((4i32) as u32)).wrapping_add(28672i32)) as u64)) | (((((({ let a0_ = rt::f2i(F32(c.m.r32(((fp).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); F::Sprite_ConvY(c, a0_) }).wrapping_shl((4i32) as u32)).wrapping_add(29184i32)) as u64)).wrapping_shl((16i32) as u32))); let a_ = (l_p_2).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_2).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_2 = n_; };
        F::Dma_EndDirect(c, l_p_2);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "HudSprite_DrawAlphaSet", &PROBE_N, &[], true); }
    r_
}

pub fn HudGfx_ClearAlpha(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudGfx_ClearAlpha", &PROBE_N, &[], false); }
    let mut l_p_1: u32 = 0;
    let mut l_i_2: i32 = 0;
    let mut l_n_3: i32 = 0;
    let mut l_tag_4: u32 = 0;
    let r_ = 'f: {
    l_p_1 = F::Dma_BeginDirect(c);
    { let v_ = ((((((((3i32) as u64)) | ((((0i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (l_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((14i32) as u64); let a_ = (l_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_1 = n_; };
    { let v_ = ((4575657221408423936i64) as u64); let a_ = (l_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((1i32) as u64); let a_ = (l_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_1 = n_; };
    { let v_ = ((((((224i32) as u64)) | ((((49i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((32i32) as u32))); let a_ = (l_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((78i32) as u64); let a_ = (l_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_1 = n_; };
    { let v_ = ((70i32) as u64); let a_ = (l_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((0i32) as u64); let a_ = (l_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_1 = n_; };
    l_tag_4 = l_p_1;
    { let r_ = 2i32; let n_ = (l_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_1 = n_; };
    l_i_2 = 0i32;
    {
        l_n_3 = 0i32;
        'l1: loop {
            if !(((((l_n_3) < (16i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let v_ = (((((((l_i_2).wrapping_mul(32i32)).wrapping_add(1792i32)).wrapping_shl((4i32) as u32)) | (((1824i32).wrapping_shl((4i32) as u32)).wrapping_shl((16i32) as u32)))) as u64); let a_ = (l_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                { let v_ = ((((((((((l_i_2).wrapping_mul(32i32)).wrapping_add(1792i32)).wrapping_add(32i32)).wrapping_shl((4i32) as u32)) as u64)) | ((((((1824i32).wrapping_add(448i32)).wrapping_shl((4i32) as u32)) as u64)).wrapping_shl((16i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((32i32) as u32))); let a_ = (l_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                { let r_ = 2i32; let n_ = (l_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_1 = n_; };
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
            { let o_ = l_n_3; let n_ = (o_).wrapping_add(1); l_n_3 = n_; };
        }
    }
    { let v_ = ((((((((16i32) as u64)) | ((((1i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((2i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (l_tag_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((((5i32) | ((5i32).wrapping_shl((4i32) as u32)))) as u64); let a_ = (l_tag_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((((((((2i32) as u64)) | ((((0i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (l_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((14i32) as u64); let a_ = (l_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_1 = n_; };
    { let v_ = ((4575657223564329088i64) as u64); let a_ = (l_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((1i32) as u64); let a_ = (l_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_1 = n_; };
    { let v_ = ((((((224i32) as u64)) | ((((49i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((32i32) as u32))); let a_ = (l_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((78i32) as u64); let a_ = (l_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_1 = n_; };
    F::Dma_EndDirect(c, l_p_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudGfx_ClearAlpha", &PROBE_N, &[], true); }
    r_
}

pub fn HudGfx_SetEnv(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudGfx_SetEnv", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(352) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 352);
    let r_ = 'f: {
        rt::memset(c, fp, 0, 352);
        { let v_ = ((268435477i32) as u32); c.m.w32(fp.wrapping_add(0), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(4), v_); }
        { let v_ = ((268435456i32) as u32); c.m.w32(fp.wrapping_add(8), v_); }
        { let v_ = ((1342177301i32) as u32); c.m.w32(fp.wrapping_add(12), v_); }
        { let v_ = ((32788i32) as u32); c.m.w32(fp.wrapping_add(16), v_); }
        { let v_ = ((268435456i32) as u32); c.m.w32(fp.wrapping_add(20), v_); }
        { let v_ = ((14i32) as u32); c.m.w32(fp.wrapping_add(24), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(28), v_); }
        { let v_ = (((if ((((!(((((c.m.r32((sym::gGfx).wrapping_add(564)) as i32)) & (1i32))) != 0)) as i32)) != 0) { 524400i32 } else { 524288i32 })) as u32); c.m.w32(fp.wrapping_add(32), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(36), v_); }
        { let v_ = ((76i32) as u32); c.m.w32(fp.wrapping_add(40), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(44), v_); }
        { let v_ = (((if ((((!(((((c.m.r32((sym::gGfx).wrapping_add(564)) as i32)) & (1i32))) != 0)) as i32)) != 0) { 524400i32 } else { 524288i32 })) as u32); c.m.w32(fp.wrapping_add(48), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(52), v_); }
        { let v_ = ((77i32) as u32); c.m.w32(fp.wrapping_add(56), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(60), v_); }
        { let v_ = ((822083808i32) as u32); c.m.w32(fp.wrapping_add(64), v_); }
        { let v_ = ((1i32) as u32); c.m.w32(fp.wrapping_add(68), v_); }
        { let v_ = ((78i32) as u32); c.m.w32(fp.wrapping_add(72), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(76), v_); }
        { let v_ = ((822083808i32) as u32); c.m.w32(fp.wrapping_add(80), v_); }
        { let v_ = ((1i32) as u32); c.m.w32(fp.wrapping_add(84), v_); }
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
        { let v_ = ((33488896i32) as u32); c.m.w32(fp.wrapping_add(128), v_); }
        { let v_ = ((29294592i32) as u32); c.m.w32(fp.wrapping_add(132), v_); }
        { let v_ = ((64i32) as u32); c.m.w32(fp.wrapping_add(136), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(140), v_); }
        { let v_ = ((33488896i32) as u32); c.m.w32(fp.wrapping_add(144), v_); }
        { let v_ = ((29294592i32) as u32); c.m.w32(fp.wrapping_add(148), v_); }
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
        { let v_ = ((196608i32) as u32); c.m.w32(fp.wrapping_add(192), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(196), v_); }
        { let v_ = ((71i32) as u32); c.m.w32(fp.wrapping_add(200), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(204), v_); }
        { let v_ = ((196608i32) as u32); c.m.w32(fp.wrapping_add(208), v_); }
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
    if rt::probing() { crate::host::harness::probe(c, "HudGfx_SetEnv", &PROBE_N, &[], true); }
    r_
}

pub fn HudSprite_DrawAt(c: &mut Ctx, mut p_spr_1: u32, mut p_res_2: u32, mut p_ctx2_3: u8, mut p_tbp_4: i32, mut p_cbp_5: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudSprite_DrawAt", &PROBE_N, &[(p_spr_1) as u32, (p_res_2) as u32, (p_ctx2_3) as u32, (p_tbp_4) as u32, (p_cbp_5) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(96) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 96);
    let mut l_t_6: u32 = 0;
    let mut l_pal_7: u32 = 0;
    let mut l_p_8: u32 = 0;
    let mut l_tag_9: u32 = 0;
    let mut l_ctx_10: i32 = 0;
    let mut l_block_11: i32 = 0;
    let mut l_clut_12: u64 = 0;
    let r_ = 'f: {
        l_t_6 = ((0i32) as u32);
        l_pal_7 = ((0i32) as u32);
        l_ctx_10 = (((((p_ctx2_3) as i32)) != (0i32)) as i32);
        if ((((c.m.r32(p_spr_1)) & (((1i32) as u32)))) != 0) {
            break 'f;
        }
        { let _ = F::memset(c, fp, 0i32, 64u32); };
        if ((((((((c.m.r16((p_spr_1).wrapping_add(4)) as i16)) as i32)) >= (0i32)) as i32)) != 0) {
            { let a1_ = (((c.m.r16((p_spr_1).wrapping_add(4)) as i16)) as i32); let a2_ = ((c.m.r16((p_spr_1).wrapping_add(6))) as i32); F::HudRes_UploadTex(c, p_res_2, a1_, a2_, p_tbp_4, p_cbp_5) };
            l_t_6 = (c.m.r32((p_res_2).wrapping_add(16))).wrapping_add(((((c.m.r16((p_spr_1).wrapping_add(4)) as i16)) as u32)).wrapping_mul(64));
            l_pal_7 = (c.m.r32((p_res_2).wrapping_add(16))).wrapping_add((((((((c.m.r16((p_spr_1).wrapping_add(4)) as i16)) as i32)).wrapping_add(((c.m.r16((p_spr_1).wrapping_add(6))) as i32))) as u32)).wrapping_mul(64));
        }
        l_p_8 = F::Dma_BeginDirect(c);
        l_tag_9 = l_p_8;
        { let r_ = 2i32; let n_ = (l_p_8).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_8 = n_; };
        { let v_ = rt::i2f(((c.m.r16((p_spr_1).wrapping_add(8)) as i16)) as i32); let a_ = ((fp).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::i2f(((c.m.r16((p_spr_1).wrapping_add(12)) as i16)) as i32); let a_ = ((fp).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = ((fp).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::i2f(((c.m.r16((p_spr_1).wrapping_add(10)) as i16)) as i32); let a_ = ((fp).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::i2f(((c.m.r16((p_spr_1).wrapping_add(12)) as i16)) as i32); let a_ = ((fp).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = ((fp).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::i2f(((c.m.r16((p_spr_1).wrapping_add(8)) as i16)) as i32); let a_ = ((fp).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::i2f(((c.m.r16((p_spr_1).wrapping_add(14)) as i16)) as i32); let a_ = ((fp).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = ((fp).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::i2f(((c.m.r16((p_spr_1).wrapping_add(10)) as i16)) as i32); let a_ = ((fp).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::i2f(((c.m.r16((p_spr_1).wrapping_add(14)) as i16)) as i32); let a_ = ((fp).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = ((fp).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        F::Vu0Cur_MulVec4(c, (fp).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (fp).wrapping_add((((0i32) as u32)).wrapping_mul(16)));
        F::Vu0Cur_MulVec4(c, (fp).wrapping_add((((1i32) as u32)).wrapping_mul(16)), (fp).wrapping_add((((1i32) as u32)).wrapping_mul(16)));
        F::Vu0Cur_MulVec4(c, (fp).wrapping_add((((2i32) as u32)).wrapping_mul(16)), (fp).wrapping_add((((2i32) as u32)).wrapping_mul(16)));
        F::Vu0Cur_MulVec4(c, (fp).wrapping_add((((3i32) as u32)).wrapping_mul(16)), (fp).wrapping_add((((3i32) as u32)).wrapping_mul(16)));
        { let v_ = rt::i2f({ let a0_ = rt::f2i(F32(c.m.r32(((fp).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); F::Sprite_ConvY(c, a0_) }); let a_ = ((fp).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::i2f({ let a0_ = rt::f2i(F32(c.m.r32(((fp).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); F::Sprite_ConvY(c, a0_) }); let a_ = ((fp).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::i2f({ let a0_ = rt::f2i(F32(c.m.r32(((fp).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); F::Sprite_ConvY(c, a0_) }); let a_ = ((fp).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::i2f({ let a0_ = rt::f2i(F32(c.m.r32(((fp).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); F::Sprite_ConvY(c, a0_) }); let a_ = ((fp).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        if ((((c.m.r32(p_spr_1)) & (((2i32) as u32)))) != 0) {
            { let v_ = (((c.m.r16((p_spr_1).wrapping_add(18)) as i16)) as i32); let a_ = ((fp.wrapping_add(64)).wrapping_add((((0i32) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            { let v_ = (((c.m.r16((p_spr_1).wrapping_add(18)) as i16)) as i32); let a_ = ((fp.wrapping_add(64)).wrapping_add((((2i32) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            { let v_ = (((c.m.r16((p_spr_1).wrapping_add(16)) as i16)) as i32); let a_ = ((fp.wrapping_add(64)).wrapping_add((((1i32) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            { let v_ = (((c.m.r16((p_spr_1).wrapping_add(16)) as i16)) as i32); let a_ = ((fp.wrapping_add(64)).wrapping_add((((3i32) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            { let v_ = (((c.m.r16((p_spr_1).wrapping_add(20)) as i16)) as i32); let a_ = ((fp.wrapping_add(64)).wrapping_add((((0i32) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            { let v_ = (((c.m.r16((p_spr_1).wrapping_add(20)) as i16)) as i32); let a_ = ((fp.wrapping_add(64)).wrapping_add((((1i32) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            { let v_ = (((c.m.r16((p_spr_1).wrapping_add(22)) as i16)) as i32); let a_ = ((fp.wrapping_add(64)).wrapping_add((((2i32) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            { let v_ = (((c.m.r16((p_spr_1).wrapping_add(22)) as i16)) as i32); let a_ = ((fp.wrapping_add(64)).wrapping_add((((3i32) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        } else {
            { let v_ = (((c.m.r16((p_spr_1).wrapping_add(16)) as i16)) as i32); let a_ = ((fp.wrapping_add(64)).wrapping_add((((0i32) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            { let v_ = (((c.m.r16((p_spr_1).wrapping_add(16)) as i16)) as i32); let a_ = ((fp.wrapping_add(64)).wrapping_add((((2i32) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            { let v_ = (((c.m.r16((p_spr_1).wrapping_add(18)) as i16)) as i32); let a_ = ((fp.wrapping_add(64)).wrapping_add((((1i32) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            { let v_ = (((c.m.r16((p_spr_1).wrapping_add(18)) as i16)) as i32); let a_ = ((fp.wrapping_add(64)).wrapping_add((((3i32) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            { let v_ = (((c.m.r16((p_spr_1).wrapping_add(20)) as i16)) as i32); let a_ = ((fp.wrapping_add(64)).wrapping_add((((0i32) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            { let v_ = (((c.m.r16((p_spr_1).wrapping_add(20)) as i16)) as i32); let a_ = ((fp.wrapping_add(64)).wrapping_add((((1i32) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            { let v_ = (((c.m.r16((p_spr_1).wrapping_add(22)) as i16)) as i32); let a_ = ((fp.wrapping_add(64)).wrapping_add((((2i32) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            { let v_ = (((c.m.r16((p_spr_1).wrapping_add(22)) as i16)) as i32); let a_ = ((fp.wrapping_add(64)).wrapping_add((((3i32) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        }
        if ((((((((c.m.r16((p_spr_1).wrapping_add(4)) as i16)) as i32)) >= (0i32)) as i32)) != 0) {
            l_block_11 = ((c.m.r32((l_t_6).wrapping_add(32)) as i32)).wrapping_add(p_tbp_4);
            l_clut_12 = (((((c.m.r32((l_pal_7).wrapping_add(36)) as i32)).wrapping_add(p_cbp_5)) as u64)).wrapping_shl((37i32) as u32);
            { let v_ = ((((((((c.m.r8((p_spr_1).wrapping_add(24))) as u64)) | ((((c.m.r8((p_spr_1).wrapping_add(25))) as u64)).wrapping_shl((8i32) as u32)))) | ((((c.m.r8((p_spr_1).wrapping_add(26))) as u64)).wrapping_shl((16i32) as u32)))) | ((((c.m.r8((p_spr_1).wrapping_add(27))) as u64)).wrapping_shl((24i32) as u32))); let a_ = (l_p_8).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let v_ = ((((((c.m.r64((l_t_6).wrapping_add(48))) | (l_clut_12))) | (((l_block_11) as u64)))) | ((((32768i32) as u64)).wrapping_shl((19i32) as u32))); let a_ = (l_p_8).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let r_ = 2i32; let n_ = (l_p_8).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_8 = n_; };
            { let v_ = ((((((c.m.r64((l_t_6).wrapping_add(48))) | (l_clut_12))) | (((l_block_11) as u64)))) | ((((32768i32) as u64)).wrapping_shl((19i32) as u32))); let a_ = (l_p_8).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let v_ = (((((l_ctx_10) as u64)).wrapping_shl((9i32) as u32)) | (((340i32) as u64))); let a_ = (l_p_8).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let r_ = 2i32; let n_ = (l_p_8).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_8 = n_; };
        } else {
            { let v_ = ((((((((c.m.r8((p_spr_1).wrapping_add(24))) as u64)) | ((((c.m.r8((p_spr_1).wrapping_add(25))) as u64)).wrapping_shl((8i32) as u32)))) | ((((c.m.r8((p_spr_1).wrapping_add(26))) as u64)).wrapping_shl((16i32) as u32)))) | ((((c.m.r8((p_spr_1).wrapping_add(27))) as u64)).wrapping_shl((24i32) as u32))); let a_ = (l_p_8).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let v_ = (((((l_ctx_10) as u64)).wrapping_shl((9i32) as u32)) | (((68i32) as u64))); let a_ = (l_p_8).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let r_ = 2i32; let n_ = (l_p_8).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_8 = n_; };
        }
        { let v_ = (((((((c.m.r32(((fp.wrapping_add(64)).wrapping_add((((0i32) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)).wrapping_shl((4i32) as u32)).wrapping_add(8i32)) as u64)) | (((((((c.m.r32(((fp.wrapping_add(64)).wrapping_add((((0i32) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)).wrapping_shl((4i32) as u32)).wrapping_add(8i32)) as u64)).wrapping_shl((16i32) as u32))); let a_ = (l_p_8).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((((((rt::f2i(F32(c.m.r32(((fp).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))))).wrapping_shl((4i32) as u32)).wrapping_add(28672i32)) as u64)) | ((((((rt::f2i(F32(c.m.r32(((fp).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))))).wrapping_shl((4i32) as u32)).wrapping_add(29184i32)) as u64)).wrapping_shl((16i32) as u32))); let a_ = (l_p_8).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_8).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_8 = n_; };
        { let v_ = (((((((c.m.r32(((fp.wrapping_add(64)).wrapping_add((((1i32) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)).wrapping_shl((4i32) as u32)).wrapping_add(8i32)) as u64)) | (((((((c.m.r32(((fp.wrapping_add(64)).wrapping_add((((1i32) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)).wrapping_shl((4i32) as u32)).wrapping_add(8i32)) as u64)).wrapping_shl((16i32) as u32))); let a_ = (l_p_8).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((((((rt::f2i(F32(c.m.r32(((fp).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))))).wrapping_shl((4i32) as u32)).wrapping_add(28672i32)) as u64)) | ((((((rt::f2i(F32(c.m.r32(((fp).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))))).wrapping_shl((4i32) as u32)).wrapping_add(29184i32)) as u64)).wrapping_shl((16i32) as u32))); let a_ = (l_p_8).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_8).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_8 = n_; };
        { let v_ = (((((((c.m.r32(((fp.wrapping_add(64)).wrapping_add((((2i32) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)).wrapping_shl((4i32) as u32)).wrapping_add(8i32)) as u64)) | (((((((c.m.r32(((fp.wrapping_add(64)).wrapping_add((((2i32) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)).wrapping_shl((4i32) as u32)).wrapping_add(8i32)) as u64)).wrapping_shl((16i32) as u32))); let a_ = (l_p_8).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((((((rt::f2i(F32(c.m.r32(((fp).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))))).wrapping_shl((4i32) as u32)).wrapping_add(28672i32)) as u64)) | ((((((rt::f2i(F32(c.m.r32(((fp).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))))).wrapping_shl((4i32) as u32)).wrapping_add(29184i32)) as u64)).wrapping_shl((16i32) as u32))); let a_ = (l_p_8).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_8).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_8 = n_; };
        { let v_ = (((((((c.m.r32(((fp.wrapping_add(64)).wrapping_add((((3i32) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)).wrapping_shl((4i32) as u32)).wrapping_add(8i32)) as u64)) | (((((((c.m.r32(((fp.wrapping_add(64)).wrapping_add((((3i32) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)).wrapping_shl((4i32) as u32)).wrapping_add(8i32)) as u64)).wrapping_shl((16i32) as u32))); let a_ = (l_p_8).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((((((rt::f2i(F32(c.m.r32(((fp).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))))).wrapping_shl((4i32) as u32)).wrapping_add(28672i32)) as u64)) | ((((((rt::f2i(F32(c.m.r32(((fp).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))))).wrapping_shl((4i32) as u32)).wrapping_add(29184i32)) as u64)).wrapping_shl((16i32) as u32))); let a_ = (l_p_8).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_8).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_8 = n_; };
        if ((((((((c.m.r16((p_spr_1).wrapping_add(4)) as i16)) as i32)) >= (0i32)) as i32)) != 0) {
            { let v_ = ((((((((1i32) as u64)) | ((((1i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((12i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (l_tag_9).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let v_ = ((91617345341041i64) as u64); let a_ = (l_tag_9).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        } else {
            { let v_ = ((((((((1i32) as u64)) | ((((1i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((10i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (l_tag_9).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let v_ = ((357880255233i64) as u64); let a_ = (l_tag_9).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        }
        F::Dma_EndDirect(c, l_p_8);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "HudSprite_DrawAt", &PROBE_N, &[], true); }
    r_
}

pub fn HudSprite_DrawGradient(c: &mut Ctx, mut p_spr_1: u32, mut p_res_2: u32, mut p_ctx2_3: u8, mut p_colorL_4: i32, mut p_colorR_5: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudSprite_DrawGradient", &PROBE_N, &[(p_spr_1) as u32, (p_res_2) as u32, (p_ctx2_3) as u32, (p_colorL_4) as u32, (p_colorR_5) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(96) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 96);
    let mut l_t_6: u32 = 0;
    let mut l_pal_7: u32 = 0;
    let mut l_p_8: u32 = 0;
    let mut l_tag_9: u32 = 0;
    let mut l_ctx_10: i32 = 0;
    let mut l_block_11: i32 = 0;
    let mut l_clut_12: u64 = 0;
    let r_ = 'f: {
        l_t_6 = ((0i32) as u32);
        l_pal_7 = ((0i32) as u32);
        l_ctx_10 = (((((p_ctx2_3) as i32)) != (0i32)) as i32);
        if ((((c.m.r32(p_spr_1)) & (((1i32) as u32)))) != 0) {
            break 'f;
        }
        { let _ = F::memset(c, fp, 0i32, 64u32); };
        if ((((((((c.m.r16((p_spr_1).wrapping_add(4)) as i16)) as i32)) >= (0i32)) as i32)) != 0) {
            { let a1_ = (((c.m.r16((p_spr_1).wrapping_add(4)) as i16)) as i32); let a2_ = ((c.m.r16((p_spr_1).wrapping_add(6))) as i32); F::HudRes_UploadTex(c, p_res_2, a1_, a2_, 10752i32, 11392i32) };
            l_t_6 = (c.m.r32((p_res_2).wrapping_add(16))).wrapping_add(((((c.m.r16((p_spr_1).wrapping_add(4)) as i16)) as u32)).wrapping_mul(64));
            l_pal_7 = (c.m.r32((p_res_2).wrapping_add(16))).wrapping_add((((((((c.m.r16((p_spr_1).wrapping_add(4)) as i16)) as i32)).wrapping_add(((c.m.r16((p_spr_1).wrapping_add(6))) as i32))) as u32)).wrapping_mul(64));
        }
        l_p_8 = F::Dma_BeginDirect(c);
        l_tag_9 = l_p_8;
        { let r_ = 2i32; let n_ = (l_p_8).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_8 = n_; };
        { let v_ = rt::i2f(((c.m.r16((p_spr_1).wrapping_add(8)) as i16)) as i32); let a_ = ((fp).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::i2f(((c.m.r16((p_spr_1).wrapping_add(12)) as i16)) as i32); let a_ = ((fp).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = ((fp).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::i2f(((c.m.r16((p_spr_1).wrapping_add(10)) as i16)) as i32); let a_ = ((fp).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::i2f(((c.m.r16((p_spr_1).wrapping_add(12)) as i16)) as i32); let a_ = ((fp).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = ((fp).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::i2f(((c.m.r16((p_spr_1).wrapping_add(8)) as i16)) as i32); let a_ = ((fp).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::i2f(((c.m.r16((p_spr_1).wrapping_add(14)) as i16)) as i32); let a_ = ((fp).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = ((fp).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::i2f(((c.m.r16((p_spr_1).wrapping_add(10)) as i16)) as i32); let a_ = ((fp).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::i2f(((c.m.r16((p_spr_1).wrapping_add(14)) as i16)) as i32); let a_ = ((fp).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = ((fp).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        F::Vu0Cur_MulVec4(c, (fp).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (fp).wrapping_add((((0i32) as u32)).wrapping_mul(16)));
        F::Vu0Cur_MulVec4(c, (fp).wrapping_add((((1i32) as u32)).wrapping_mul(16)), (fp).wrapping_add((((1i32) as u32)).wrapping_mul(16)));
        F::Vu0Cur_MulVec4(c, (fp).wrapping_add((((2i32) as u32)).wrapping_mul(16)), (fp).wrapping_add((((2i32) as u32)).wrapping_mul(16)));
        F::Vu0Cur_MulVec4(c, (fp).wrapping_add((((3i32) as u32)).wrapping_mul(16)), (fp).wrapping_add((((3i32) as u32)).wrapping_mul(16)));
        { let v_ = rt::i2f({ let a0_ = rt::f2i(F32(c.m.r32(((fp).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); F::Sprite_ConvY(c, a0_) }); let a_ = ((fp).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::i2f({ let a0_ = rt::f2i(F32(c.m.r32(((fp).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); F::Sprite_ConvY(c, a0_) }); let a_ = ((fp).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::i2f({ let a0_ = rt::f2i(F32(c.m.r32(((fp).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); F::Sprite_ConvY(c, a0_) }); let a_ = ((fp).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::i2f({ let a0_ = rt::f2i(F32(c.m.r32(((fp).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); F::Sprite_ConvY(c, a0_) }); let a_ = ((fp).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        if ((((c.m.r32(p_spr_1)) & (((2i32) as u32)))) != 0) {
            { let v_ = (((c.m.r16((p_spr_1).wrapping_add(18)) as i16)) as i32); let a_ = ((fp.wrapping_add(64)).wrapping_add((((0i32) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            { let v_ = (((c.m.r16((p_spr_1).wrapping_add(18)) as i16)) as i32); let a_ = ((fp.wrapping_add(64)).wrapping_add((((2i32) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            { let v_ = (((c.m.r16((p_spr_1).wrapping_add(16)) as i16)) as i32); let a_ = ((fp.wrapping_add(64)).wrapping_add((((1i32) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            { let v_ = (((c.m.r16((p_spr_1).wrapping_add(16)) as i16)) as i32); let a_ = ((fp.wrapping_add(64)).wrapping_add((((3i32) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            { let v_ = (((c.m.r16((p_spr_1).wrapping_add(20)) as i16)) as i32); let a_ = ((fp.wrapping_add(64)).wrapping_add((((0i32) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            { let v_ = (((c.m.r16((p_spr_1).wrapping_add(20)) as i16)) as i32); let a_ = ((fp.wrapping_add(64)).wrapping_add((((1i32) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            { let v_ = (((c.m.r16((p_spr_1).wrapping_add(22)) as i16)) as i32); let a_ = ((fp.wrapping_add(64)).wrapping_add((((2i32) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            { let v_ = (((c.m.r16((p_spr_1).wrapping_add(22)) as i16)) as i32); let a_ = ((fp.wrapping_add(64)).wrapping_add((((3i32) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        } else {
            { let v_ = (((c.m.r16((p_spr_1).wrapping_add(16)) as i16)) as i32); let a_ = ((fp.wrapping_add(64)).wrapping_add((((0i32) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            { let v_ = (((c.m.r16((p_spr_1).wrapping_add(16)) as i16)) as i32); let a_ = ((fp.wrapping_add(64)).wrapping_add((((2i32) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            { let v_ = (((c.m.r16((p_spr_1).wrapping_add(18)) as i16)) as i32); let a_ = ((fp.wrapping_add(64)).wrapping_add((((1i32) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            { let v_ = (((c.m.r16((p_spr_1).wrapping_add(18)) as i16)) as i32); let a_ = ((fp.wrapping_add(64)).wrapping_add((((3i32) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            { let v_ = (((c.m.r16((p_spr_1).wrapping_add(20)) as i16)) as i32); let a_ = ((fp.wrapping_add(64)).wrapping_add((((0i32) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            { let v_ = (((c.m.r16((p_spr_1).wrapping_add(20)) as i16)) as i32); let a_ = ((fp.wrapping_add(64)).wrapping_add((((1i32) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            { let v_ = (((c.m.r16((p_spr_1).wrapping_add(22)) as i16)) as i32); let a_ = ((fp.wrapping_add(64)).wrapping_add((((2i32) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            { let v_ = (((c.m.r16((p_spr_1).wrapping_add(22)) as i16)) as i32); let a_ = ((fp.wrapping_add(64)).wrapping_add((((3i32) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        }
        if ((((((((c.m.r16((p_spr_1).wrapping_add(4)) as i16)) as i32)) >= (0i32)) as i32)) != 0) {
            l_block_11 = ((c.m.r32((l_t_6).wrapping_add(32)) as i32)).wrapping_add(10752i32);
            l_clut_12 = (((((c.m.r32((l_pal_7).wrapping_add(36)) as i32)).wrapping_add(11392i32)) as u64)).wrapping_shl((37i32) as u32);
            { let v_ = ((((((c.m.r64((l_t_6).wrapping_add(48))) | (l_clut_12))) | (((l_block_11) as u64)))) | ((((32768i32) as u64)).wrapping_shl((19i32) as u32))); let a_ = (l_p_8).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let v_ = ((((((c.m.r64((l_t_6).wrapping_add(48))) | (l_clut_12))) | (((l_block_11) as u64)))) | ((((32768i32) as u64)).wrapping_shl((19i32) as u32))); let a_ = (l_p_8).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let r_ = 2i32; let n_ = (l_p_8).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_8 = n_; };
            { let v_ = (((((l_ctx_10) as u64)).wrapping_shl((9i32) as u32)) | (((340i32) as u64))); let a_ = (l_p_8).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let v_ = (((((((((((p_colorL_4).wrapping_shr((24i32) as u32)) & (255i32))) as u64)) | (((((((p_colorL_4).wrapping_shr((16i32) as u32)) & (255i32))) as u64)).wrapping_shl((8i32) as u32)))) | (((((((p_colorL_4).wrapping_shr((8i32) as u32)) & (255i32))) as u64)).wrapping_shl((16i32) as u32)))) | ((((((p_colorL_4) & (255i32))) as u64)).wrapping_shl((24i32) as u32))); let a_ = (l_p_8).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let r_ = 2i32; let n_ = (l_p_8).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_8 = n_; };
        } else {
            { let v_ = (((((l_ctx_10) as u64)).wrapping_shl((9i32) as u32)) | (((68i32) as u64))); let a_ = (l_p_8).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let v_ = (((((((((((p_colorL_4).wrapping_shr((24i32) as u32)) & (255i32))) as u64)) | (((((((p_colorL_4).wrapping_shr((16i32) as u32)) & (255i32))) as u64)).wrapping_shl((8i32) as u32)))) | (((((((p_colorL_4).wrapping_shr((8i32) as u32)) & (255i32))) as u64)).wrapping_shl((16i32) as u32)))) | ((((((p_colorL_4) & (255i32))) as u64)).wrapping_shl((24i32) as u32))); let a_ = (l_p_8).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let r_ = 2i32; let n_ = (l_p_8).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_8 = n_; };
        }
        { let v_ = (((((((c.m.r32(((fp.wrapping_add(64)).wrapping_add((((0i32) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)).wrapping_shl((4i32) as u32)).wrapping_add(8i32)) as u64)) | (((((((c.m.r32(((fp.wrapping_add(64)).wrapping_add((((0i32) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)).wrapping_shl((4i32) as u32)).wrapping_add(8i32)) as u64)).wrapping_shl((16i32) as u32))); let a_ = (l_p_8).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((((((rt::f2i(F32(c.m.r32(((fp).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))))).wrapping_shl((4i32) as u32)).wrapping_add(28672i32)) as u64)) | ((((((rt::f2i(F32(c.m.r32(((fp).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))))).wrapping_shl((4i32) as u32)).wrapping_add(29184i32)) as u64)).wrapping_shl((16i32) as u32))); let a_ = (l_p_8).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_8).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_8 = n_; };
        { let v_ = (((((((((((p_colorR_5).wrapping_shr((24i32) as u32)) & (255i32))) as u64)) | (((((((p_colorR_5).wrapping_shr((16i32) as u32)) & (255i32))) as u64)).wrapping_shl((8i32) as u32)))) | (((((((p_colorR_5).wrapping_shr((8i32) as u32)) & (255i32))) as u64)).wrapping_shl((16i32) as u32)))) | ((((((p_colorR_5) & (255i32))) as u64)).wrapping_shl((24i32) as u32))); let a_ = (l_p_8).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = (((((((c.m.r32(((fp.wrapping_add(64)).wrapping_add((((1i32) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)).wrapping_shl((4i32) as u32)).wrapping_add(8i32)) as u64)) | (((((((c.m.r32(((fp.wrapping_add(64)).wrapping_add((((1i32) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)).wrapping_shl((4i32) as u32)).wrapping_add(8i32)) as u64)).wrapping_shl((16i32) as u32))); let a_ = (l_p_8).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_8).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_8 = n_; };
        { let v_ = ((((((rt::f2i(F32(c.m.r32(((fp).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))))).wrapping_shl((4i32) as u32)).wrapping_add(28672i32)) as u64)) | ((((((rt::f2i(F32(c.m.r32(((fp).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))))).wrapping_shl((4i32) as u32)).wrapping_add(29184i32)) as u64)).wrapping_shl((16i32) as u32))); let a_ = (l_p_8).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = (((((((((((p_colorL_4).wrapping_shr((24i32) as u32)) & (255i32))) as u64)) | (((((((p_colorL_4).wrapping_shr((16i32) as u32)) & (255i32))) as u64)).wrapping_shl((8i32) as u32)))) | (((((((p_colorL_4).wrapping_shr((8i32) as u32)) & (255i32))) as u64)).wrapping_shl((16i32) as u32)))) | ((((((p_colorL_4) & (255i32))) as u64)).wrapping_shl((24i32) as u32))); let a_ = (l_p_8).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_8).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_8 = n_; };
        { let v_ = (((((((c.m.r32(((fp.wrapping_add(64)).wrapping_add((((2i32) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)).wrapping_shl((4i32) as u32)).wrapping_add(8i32)) as u64)) | (((((((c.m.r32(((fp.wrapping_add(64)).wrapping_add((((2i32) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)).wrapping_shl((4i32) as u32)).wrapping_add(8i32)) as u64)).wrapping_shl((16i32) as u32))); let a_ = (l_p_8).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((((((rt::f2i(F32(c.m.r32(((fp).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))))).wrapping_shl((4i32) as u32)).wrapping_add(28672i32)) as u64)) | ((((((rt::f2i(F32(c.m.r32(((fp).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))))).wrapping_shl((4i32) as u32)).wrapping_add(29184i32)) as u64)).wrapping_shl((16i32) as u32))); let a_ = (l_p_8).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_8).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_8 = n_; };
        { let v_ = (((((((((((p_colorR_5).wrapping_shr((24i32) as u32)) & (255i32))) as u64)) | (((((((p_colorR_5).wrapping_shr((16i32) as u32)) & (255i32))) as u64)).wrapping_shl((8i32) as u32)))) | (((((((p_colorR_5).wrapping_shr((8i32) as u32)) & (255i32))) as u64)).wrapping_shl((16i32) as u32)))) | ((((((p_colorR_5) & (255i32))) as u64)).wrapping_shl((24i32) as u32))); let a_ = (l_p_8).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = (((((((c.m.r32(((fp.wrapping_add(64)).wrapping_add((((3i32) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)).wrapping_shl((4i32) as u32)).wrapping_add(8i32)) as u64)) | (((((((c.m.r32(((fp.wrapping_add(64)).wrapping_add((((3i32) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)).wrapping_shl((4i32) as u32)).wrapping_add(8i32)) as u64)).wrapping_shl((16i32) as u32))); let a_ = (l_p_8).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_8).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_8 = n_; };
        { let v_ = ((((((rt::f2i(F32(c.m.r32(((fp).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))))).wrapping_shl((4i32) as u32)).wrapping_add(28672i32)) as u64)) | ((((((rt::f2i(F32(c.m.r32(((fp).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))))).wrapping_shl((4i32) as u32)).wrapping_add(29184i32)) as u64)).wrapping_shl((16i32) as u32))); let a_ = (l_p_8).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((0i32) as u64); let a_ = (l_p_8).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_8).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_8 = n_; };
        if ((((((((c.m.r16((p_spr_1).wrapping_add(4)) as i16)) as i32)) >= (0i32)) as i32)) != 0) {
            { let v_ = ((((((((1i32) as u64)) | ((((1i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((15i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (l_tag_9).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let v_ = ((374171594535407719i64) as u64); let a_ = (l_tag_9).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        } else {
            { let v_ = ((((((((1i32) as u64)) | ((((1i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((13i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (l_tag_9).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let v_ = ((1461607791153936i64) as u64); let a_ = (l_tag_9).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        }
        F::Dma_EndDirect(c, l_p_8);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "HudSprite_DrawGradient", &PROBE_N, &[], true); }
    r_
}
