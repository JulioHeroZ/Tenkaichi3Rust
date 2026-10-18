#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn EftSpr_DrawFlat(c: &mut Ctx, mut p_r_1: i32, mut p_g_2: i32, mut p_b_3: i32, mut p_a_4: i32, mut p_u0_5: F32, mut p_v0_6: F32, mut p_u1_7: F32, mut p_v1_8: F32, mut p_pos_9: u32, mut p_w_10: u32, mut p_h_11: u32, mut p_ctx_12: i32, mut p_layer_13: i32, mut p_tex0_14: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSpr_DrawFlat", &PROBE_N, &[(p_r_1) as u32, (p_g_2) as u32, (p_b_3) as u32, (p_a_4) as u32, (p_u0_5).0, (p_v0_6).0, (p_u1_7).0, (p_v1_8).0, (p_pos_9) as u32, (p_w_10) as u32, (p_h_11) as u32, (p_ctx_12) as u32, (p_layer_13) as u32, (p_tex0_14) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_p_15: u32 = 0;
    let mut l_e_16: u32 = 0;
    let mut l_scale_17: F32 = F32(0);
    let mut l_k_18: u32 = 0;
    let mut l_z_19: i32 = 0;
    let mut l_l_20: i32 = 0;
    let mut l_c_21: i8 = 0;
    let mut l_abe_22: i64 = 0;
    let r_ = 'f: {
        l_scale_17 = { let a_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(564); F32(c.m.r32(a_)) };
        l_abe_22 = ((1i32) as i64);
        if (((((((((p_w_10) == (((0i32) as u32))) as i32)) != 0) || (((((p_h_11) == (((0i32) as u32))) as i32)) != 0)) as i32)) != 0) {
            break 'f;
        }
        { let _ = { let a1_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(320); F::Mtx_ProjectPoint(c, fp, a1_, p_pos_9) }; };
        l_k_18 = rt::f2u(rt::fdiv(rt::fmul(l_scale_17, F32(0x45800000)), rt::i2f((c.m.r32((fp).wrapping_add(12)) as i32))));
        p_w_10 = ((p_w_10).wrapping_mul(l_k_18)).wrapping_shr((12i32) as u32);
        p_h_11 = ((p_h_11).wrapping_mul(l_k_18)).wrapping_shr((12i32) as u32);
        if (((((((((p_w_10) < (((2i32) as u32))) as i32)) != 0) || (((((p_h_11) < (((2i32) as u32))) as i32)) != 0)) as i32)) != 0) {
            break 'f;
        }
        if (({ let a0_ = ((((((c.m.r32(fp) as i32)) as u32)).wrapping_sub(p_w_10)) as i32); let a1_ = ((((((c.m.r32((fp).wrapping_add(4)) as i32)) as u32)).wrapping_sub(p_h_11)) as i32); let a2_ = (c.m.r32((fp).wrapping_add(8)) as i32); F::EftSpr_IsOffScreen(c, a0_, a1_, a2_) }) != 0) {
            break 'f;
        }
        if (({ let a0_ = ((((((c.m.r32(fp) as i32)) as u32)).wrapping_sub(p_w_10)) as i32); let a1_ = ((((((c.m.r32((fp).wrapping_add(4)) as i32)) as u32)).wrapping_add(p_h_11)) as i32); let a2_ = (c.m.r32((fp).wrapping_add(8)) as i32); F::EftSpr_IsOffScreen(c, a0_, a1_, a2_) }) != 0) {
            break 'f;
        }
        if (({ let a0_ = ((((((c.m.r32(fp) as i32)) as u32)).wrapping_add(p_w_10)) as i32); let a1_ = ((((((c.m.r32((fp).wrapping_add(4)) as i32)) as u32)).wrapping_add(p_h_11)) as i32); let a2_ = (c.m.r32((fp).wrapping_add(8)) as i32); F::EftSpr_IsOffScreen(c, a0_, a1_, a2_) }) != 0) {
            break 'f;
        }
        if (({ let a0_ = ((((((c.m.r32(fp) as i32)) as u32)).wrapping_add(p_w_10)) as i32); let a1_ = ((((((c.m.r32((fp).wrapping_add(4)) as i32)) as u32)).wrapping_sub(p_h_11)) as i32); let a2_ = (c.m.r32((fp).wrapping_add(8)) as i32); F::EftSpr_IsOffScreen(c, a0_, a1_, a2_) }) != 0) {
            break 'f;
        }
        l_p_15 = c.m.r32(sym::gOtCur);
        { let v_ = (l_p_15).wrapping_add((((1i32) as u32)).wrapping_mul(128)); let a_ = sym::gOtCur; c.m.w32(a_, v_); };
        l_c_21 = ((p_ctx_12) as i8);
        { let v_ = (((((((((l_c_21) as i64)).wrapping_shl((9i32) as u32)) | ((l_abe_22).wrapping_shl((6i32) as u32)))) | (((20i32) as i64)))) as u64); let a_ = (l_p_15).wrapping_add(32); c.m.w64(a_, v_); };
        { let v_ = ((536870919i32) as u32); let a_ = l_p_15; c.m.w32(a_, v_); };
        { let v_ = ((268435456i32) as u32); let a_ = (l_p_15).wrapping_add(8); c.m.w32(a_, v_); };
        { let v_ = ((1342177287i32) as u32); let a_ = (l_p_15).wrapping_add(12); c.m.w32(a_, v_); };
        { let v_ = 14123288431433908225u64; let a_ = (l_p_15).wrapping_add(16); c.m.w64(a_, v_); };
        { let v_ = ((((((((l_c_21) as i32)).wrapping_shl((4i32) as u32)) as i64)).wrapping_add(268436062347616i64)) as u64); let a_ = (l_p_15).wrapping_add(24); c.m.w64(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = (l_p_15).wrapping_add(4); c.m.w32(a_, v_); };
        l_l_20 = p_layer_13;
        if (((((l_l_20) >= (2i32)) as i32)) != 0) {
            { let r_ = 2i32; let n_ = (l_l_20).wrapping_sub(r_); l_l_20 = n_; };
        }
        { let v_ = c.m.r64(p_tex0_14); let a_ = (l_p_15).wrapping_add(40); c.m.w64(a_, v_); };
        { let v_ = ((p_r_1) as u8); let a_ = (l_p_15).wrapping_add(48); c.m.w8(a_, v_); };
        { let v_ = ((p_g_2) as u8); let a_ = (l_p_15).wrapping_add(49); c.m.w8(a_, v_); };
        { let v_ = ((p_b_3) as u8); let a_ = (l_p_15).wrapping_add(50); c.m.w8(a_, v_); };
        { let v_ = ((p_a_4) as u8); let a_ = (l_p_15).wrapping_add(51); c.m.w8(a_, v_); };
        { let v_ = F32(0x3f800000); let a_ = (l_p_15).wrapping_add(52); c.m.w32(a_, (v_).0); };
        { let v_ = p_u0_5; let a_ = ((l_p_15).wrapping_add(56)).wrapping_add((((0i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
        { let v_ = p_v0_6; let a_ = (((l_p_15).wrapping_add(56)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = p_u0_5; let a_ = ((l_p_15).wrapping_add(56)).wrapping_add((((1i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
        { let v_ = p_v1_8; let a_ = (((l_p_15).wrapping_add(56)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = p_u1_7; let a_ = ((l_p_15).wrapping_add(56)).wrapping_add((((2i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
        { let v_ = p_v0_6; let a_ = (((l_p_15).wrapping_add(56)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = p_u1_7; let a_ = ((l_p_15).wrapping_add(56)).wrapping_add((((3i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
        { let v_ = p_v1_8; let a_ = (((l_p_15).wrapping_add(56)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = ((((((c.m.r32(fp) as i32)) as u32)).wrapping_sub(p_w_10)) as u64); let a_ = (((l_p_15).wrapping_add(56)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = ((((((c.m.r32((fp).wrapping_add(4)) as i32)) as u32)).wrapping_sub(p_h_11)) as u64); let a_ = ((((l_p_15).wrapping_add(56)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = (((c.m.r32((fp).wrapping_add(8)) as i32)) as u64); let a_ = ((((l_p_15).wrapping_add(56)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
        { let v_ = ((255i32) as u64); let a_ = ((((l_p_15).wrapping_add(56)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
        { let v_ = ((((((c.m.r32(fp) as i32)) as u32)).wrapping_add(p_w_10)) as u64); let a_ = (((l_p_15).wrapping_add(56)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = ((((((c.m.r32((fp).wrapping_add(4)) as i32)) as u32)).wrapping_sub(p_h_11)) as u64); let a_ = ((((l_p_15).wrapping_add(56)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = (((c.m.r32((fp).wrapping_add(8)) as i32)) as u64); let a_ = ((((l_p_15).wrapping_add(56)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
        { let v_ = ((255i32) as u64); let a_ = ((((l_p_15).wrapping_add(56)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
        { let v_ = ((((((c.m.r32(fp) as i32)) as u32)).wrapping_sub(p_w_10)) as u64); let a_ = (((l_p_15).wrapping_add(56)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = ((((((c.m.r32((fp).wrapping_add(4)) as i32)) as u32)).wrapping_add(p_h_11)) as u64); let a_ = ((((l_p_15).wrapping_add(56)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = (((c.m.r32((fp).wrapping_add(8)) as i32)) as u64); let a_ = ((((l_p_15).wrapping_add(56)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
        { let v_ = ((255i32) as u64); let a_ = ((((l_p_15).wrapping_add(56)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
        { let v_ = ((((((c.m.r32(fp) as i32)) as u32)).wrapping_add(p_w_10)) as u64); let a_ = (((l_p_15).wrapping_add(56)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(8); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = ((((((c.m.r32((fp).wrapping_add(4)) as i32)) as u32)).wrapping_add(p_h_11)) as u64); let a_ = ((((l_p_15).wrapping_add(56)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(8)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = (((c.m.r32((fp).wrapping_add(8)) as i32)) as u64); let a_ = ((((l_p_15).wrapping_add(56)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(8)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
        { let v_ = ((255i32) as u64); let a_ = ((((l_p_15).wrapping_add(56)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(8)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
        l_z_19 = ((c.m.r32((fp).wrapping_add(8)) as i32)).wrapping_shr((8i32) as u32);
        if (((((l_z_19) < (0i32)) as i32)) != 0) {
            l_e_16 = ((c.m.r32(sym::gOtZ)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((l_l_20) as u32)).wrapping_mul(8));
        } else {
            if (((((l_z_19) >= (4096i32)) as i32)) != 0) {
                l_e_16 = ((c.m.r32(sym::gOtZ)).wrapping_add((((4095i32) as u32)).wrapping_mul(16))).wrapping_add((((l_l_20) as u32)).wrapping_mul(8));
            } else {
                l_e_16 = ((c.m.r32(sym::gOtZ)).wrapping_add((((l_z_19) as u32)).wrapping_mul(16))).wrapping_add((((l_l_20) as u32)).wrapping_mul(8));
            }
        }
        { let v_ = l_p_15; let a_ = (c.m.r32((l_e_16).wrapping_add(4))).wrapping_add(4); c.m.w32(a_, v_); };
        { let v_ = l_p_15; let a_ = (l_e_16).wrapping_add(4); c.m.w32(a_, v_); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftSpr_DrawFlat", &PROBE_N, &[], true); }
    r_
}

pub fn EftSpr_DrawRot(c: &mut Ctx, mut p_r_1: u8, mut p_g_2: u8, mut p_b_3: u8, mut p_a_4: u8, mut p_x_5: F32, mut p_y_6: F32, mut p_z_7: F32, mut p_u0_8: F32, mut p_v0_9: F32, mut p_u1_10: F32, mut p_v1_11: F32, mut p_rot_12: F32, mut p_ofsX_13: i32, mut p_ofsY_14: i32, mut p_w_15: u32, mut p_h_16: u32, mut p_unused_17: i32, mut p_size_18: u32, mut p_ctx_19: i32, mut p_layer_20: i32, mut p_tex0_21: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSpr_DrawRot", &PROBE_N, &[(p_r_1) as u32, (p_g_2) as u32, (p_b_3) as u32, (p_a_4) as u32, (p_x_5).0, (p_y_6).0, (p_z_7).0, (p_u0_8).0, (p_v0_9).0, (p_u1_10).0, (p_v1_11).0, (p_rot_12).0, (p_ofsX_13) as u32, (p_ofsY_14) as u32, (p_w_15) as u32, (p_h_16) as u32, (p_unused_17) as u32, (p_size_18) as u32, (p_ctx_19) as u32, (p_layer_20) as u32, (p_tex0_21) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(224) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 224);
    let mut l_p_22: u32 = 0;
    let mut l_e_23: u32 = 0;
    let mut l_scale_24: F32 = F32(0);
    let mut l_k_25: u32 = 0;
    let mut l_ox_26: i32 = 0;
    let mut l_oy_27: i32 = 0;
    let mut l_i_28: i32 = 0;
    let mut l_sz_29: i32 = 0;
    let mut l_l_30: i32 = 0;
    let mut l_cc_31: i8 = 0;
    let mut l_abe_32: i64 = 0;
    let r_ = 'f: {
        l_scale_24 = { let a_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(564); F32(c.m.r32(a_)) };
        l_abe_32 = ((1i32) as i64);
        F::Vec4_Set(c, fp.wrapping_add(80), p_x_5, p_y_6, p_z_7, F32(0x3f800000));
        { let _ = { let a1_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(320); F::Mtx_ProjectPoint(c, fp, a1_, fp.wrapping_add(80)) }; };
        l_k_25 = rt::f2u(rt::fdiv(rt::fmul(rt::u2f(p_size_18), l_scale_24), rt::i2f((c.m.r32((fp).wrapping_add(12)) as i32))));
        p_w_15 = ((p_w_15).wrapping_mul(l_k_25)).wrapping_shr((12i32) as u32);
        p_h_16 = ((p_h_16).wrapping_mul(l_k_25)).wrapping_shr((12i32) as u32);
        if (((((((((p_w_15) < (((2i32) as u32))) as i32)) != 0) || (((((p_h_16) < (((2i32) as u32))) as i32)) != 0)) as i32)) != 0) {
            break 'f;
        }
        if (((rt::fgt(p_rot_12, F32(0x40490fda)) as i32)) != 0) {
            { let r_ = F32(0x40c90fda); let n_ = rt::fsub(p_rot_12, r_); p_rot_12 = n_; };
        } else {
            if (((rt::flt(p_rot_12, F32(0xc0490fda)) as i32)) != 0) {
                { let r_ = F32(0x40c90fda); let n_ = rt::fadd(p_rot_12, r_); p_rot_12 = n_; };
            }
        }
        F::Mtx_StoreIdentity(c, fp.wrapping_add(160));
        F::Mtx_RotateZ(c, fp.wrapping_add(160), fp.wrapping_add(160), p_rot_12);
        if (((((p_ofsX_13) >= (0i32)) as i32)) != 0) {
            l_ox_26 = ((((((p_ofsX_13) as u32)).wrapping_mul(l_k_25)) as i32)).wrapping_shr((12i32) as u32);
        } else {
            l_ox_26 = ((((((((p_ofsX_13).wrapping_neg()) as u32)).wrapping_mul(l_k_25)) as i32)).wrapping_shr((12i32) as u32)).wrapping_neg();
        }
        if (((((p_ofsY_14) >= (0i32)) as i32)) != 0) {
            l_oy_27 = ((((((p_ofsY_14) as u32)).wrapping_mul(l_k_25)) as i32)).wrapping_shr((12i32) as u32);
        } else {
            l_oy_27 = ((((((((p_ofsY_14).wrapping_neg()) as u32)).wrapping_mul(l_k_25)) as i32)).wrapping_shr((12i32) as u32)).wrapping_neg();
        }
        { let v_ = (((((l_ox_26) as u32)).wrapping_sub(p_w_15)) as i32); let a_ = (fp.wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_) as u32); };
        { let v_ = (((((l_oy_27) as u32)).wrapping_sub(p_h_16)) as i32); let a_ = ((fp.wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = ((fp.wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 1i32; let a_ = ((fp.wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        { let v_ = (((((l_ox_26) as u32)).wrapping_sub(p_w_15)) as i32); let a_ = (fp.wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_) as u32); };
        { let v_ = (((p_h_16).wrapping_add(((l_oy_27) as u32))) as i32); let a_ = ((fp.wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = ((fp.wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 1i32; let a_ = ((fp.wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        { let v_ = (((p_w_15).wrapping_add(((l_ox_26) as u32))) as i32); let a_ = (fp.wrapping_add(16)).wrapping_add((((2i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_) as u32); };
        { let v_ = (((((l_oy_27) as u32)).wrapping_sub(p_h_16)) as i32); let a_ = ((fp.wrapping_add(16)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = ((fp.wrapping_add(16)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 1i32; let a_ = ((fp.wrapping_add(16)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        { let v_ = (((p_w_15).wrapping_add(((l_ox_26) as u32))) as i32); let a_ = (fp.wrapping_add(16)).wrapping_add((((3i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_) as u32); };
        { let v_ = (((p_h_16).wrapping_add(((l_oy_27) as u32))) as i32); let a_ = ((fp.wrapping_add(16)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = ((fp.wrapping_add(16)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 1i32; let a_ = ((fp.wrapping_add(16)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        {
            l_i_28 = 0i32;
            'l1: loop {
                if !(((((l_i_28) < (4i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let v_ = rt::i2f((c.m.r32(((fp.wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((l_i_28).wrapping_shl((4i32) as u32)) as u32))) as i32)); let a_ = ((fp.wrapping_add(96)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((l_i_28).wrapping_shl((4i32) as u32)) as u32)); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::i2f((c.m.r32((((fp.wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4)).wrapping_add((((l_i_28).wrapping_shl((4i32) as u32)) as u32))) as i32)); let a_ = (((fp.wrapping_add(96)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4)).wrapping_add((((l_i_28).wrapping_shl((4i32) as u32)) as u32)); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::i2f((c.m.r32((((fp.wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8)).wrapping_add((((l_i_28).wrapping_shl((4i32) as u32)) as u32))) as i32)); let a_ = (((fp.wrapping_add(96)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8)).wrapping_add((((l_i_28).wrapping_shl((4i32) as u32)) as u32)); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::i2f((c.m.r32((((fp.wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(12)).wrapping_add((((l_i_28).wrapping_shl((4i32) as u32)) as u32))) as i32)); let a_ = (((fp.wrapping_add(96)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(12)).wrapping_add((((l_i_28).wrapping_shl((4i32) as u32)) as u32)); c.m.w32(a_, (v_).0); };
                    F::Mtx_MulVec4(c, (fp.wrapping_add(96)).wrapping_add((((l_i_28) as u32)).wrapping_mul(16)), fp.wrapping_add(160), (fp.wrapping_add(96)).wrapping_add((((l_i_28) as u32)).wrapping_mul(16)));
                    F::Vec4_Mul(c, (fp.wrapping_add(96)).wrapping_add((((l_i_28) as u32)).wrapping_mul(16)), (fp.wrapping_add(96)).wrapping_add((((l_i_28) as u32)).wrapping_mul(16)), sym::gEftSprUvScale);
                    F::Vec4_ToInt(c, (fp.wrapping_add(16)).wrapping_add((((l_i_28) as u32)).wrapping_mul(16)), (fp.wrapping_add(96)).wrapping_add((((l_i_28) as u32)).wrapping_mul(16)));
                }
                { let o_ = l_i_28; let n_ = (o_).wrapping_add(1); l_i_28 = n_; };
            }
        }
        if (({ let a0_ = ((c.m.r32(fp) as i32)).wrapping_add((c.m.r32((fp.wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(16))) as i32)); let a1_ = ((c.m.r32((fp).wrapping_add(4)) as i32)).wrapping_add((c.m.r32(((fp.wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4)) as i32)); let a2_ = (c.m.r32((fp).wrapping_add(8)) as i32); F::EftSpr_IsOffScreen(c, a0_, a1_, a2_) }) != 0) {
            break 'f;
        }
        if (({ let a0_ = ((c.m.r32(fp) as i32)).wrapping_add((c.m.r32((fp.wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(16))) as i32)); let a1_ = ((c.m.r32((fp).wrapping_add(4)) as i32)).wrapping_add((c.m.r32(((fp.wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(4)) as i32)); let a2_ = (c.m.r32((fp).wrapping_add(8)) as i32); F::EftSpr_IsOffScreen(c, a0_, a1_, a2_) }) != 0) {
            break 'f;
        }
        if (({ let a0_ = ((c.m.r32(fp) as i32)).wrapping_add((c.m.r32((fp.wrapping_add(16)).wrapping_add((((2i32) as u32)).wrapping_mul(16))) as i32)); let a1_ = ((c.m.r32((fp).wrapping_add(4)) as i32)).wrapping_add((c.m.r32(((fp.wrapping_add(16)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(4)) as i32)); let a2_ = (c.m.r32((fp).wrapping_add(8)) as i32); F::EftSpr_IsOffScreen(c, a0_, a1_, a2_) }) != 0) {
            break 'f;
        }
        if (({ let a0_ = ((c.m.r32(fp) as i32)).wrapping_add((c.m.r32((fp.wrapping_add(16)).wrapping_add((((3i32) as u32)).wrapping_mul(16))) as i32)); let a1_ = ((c.m.r32((fp).wrapping_add(4)) as i32)).wrapping_add((c.m.r32(((fp.wrapping_add(16)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(4)) as i32)); let a2_ = (c.m.r32((fp).wrapping_add(8)) as i32); F::EftSpr_IsOffScreen(c, a0_, a1_, a2_) }) != 0) {
            break 'f;
        }
        l_p_22 = c.m.r32(sym::gOtCur);
        { let v_ = (l_p_22).wrapping_add((((1i32) as u32)).wrapping_mul(128)); let a_ = sym::gOtCur; c.m.w32(a_, v_); };
        l_cc_31 = ((p_ctx_19) as i8);
        { let v_ = (((((((((l_cc_31) as i64)).wrapping_shl((9i32) as u32)) | ((l_abe_32).wrapping_shl((6i32) as u32)))) | (((20i32) as i64)))) as u64); let a_ = (l_p_22).wrapping_add(32); c.m.w64(a_, v_); };
        { let v_ = ((536870919i32) as u32); let a_ = l_p_22; c.m.w32(a_, v_); };
        { let v_ = ((268435456i32) as u32); let a_ = (l_p_22).wrapping_add(8); c.m.w32(a_, v_); };
        { let v_ = ((1342177287i32) as u32); let a_ = (l_p_22).wrapping_add(12); c.m.w32(a_, v_); };
        { let v_ = 14123288431433908225u64; let a_ = (l_p_22).wrapping_add(16); c.m.w64(a_, v_); };
        { let v_ = ((((((((l_cc_31) as i32)).wrapping_shl((4i32) as u32)) as i64)).wrapping_add(268436062347616i64)) as u64); let a_ = (l_p_22).wrapping_add(24); c.m.w64(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = (l_p_22).wrapping_add(4); c.m.w32(a_, v_); };
        l_l_30 = p_layer_20;
        if (((((l_l_30) >= (2i32)) as i32)) != 0) {
            { let r_ = 2i32; let n_ = (l_l_30).wrapping_sub(r_); l_l_30 = n_; };
        }
        { let v_ = c.m.r64(p_tex0_21); let a_ = (l_p_22).wrapping_add(40); c.m.w64(a_, v_); };
        { let v_ = p_r_1; let a_ = (l_p_22).wrapping_add(48); c.m.w8(a_, v_); };
        { let v_ = p_g_2; let a_ = (l_p_22).wrapping_add(49); c.m.w8(a_, v_); };
        { let v_ = p_b_3; let a_ = (l_p_22).wrapping_add(50); c.m.w8(a_, v_); };
        { let v_ = p_a_4; let a_ = (l_p_22).wrapping_add(51); c.m.w8(a_, v_); };
        { let v_ = F32(0x3f800000); let a_ = (l_p_22).wrapping_add(52); c.m.w32(a_, (v_).0); };
        { let v_ = p_u0_8; let a_ = ((l_p_22).wrapping_add(56)).wrapping_add((((0i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
        { let v_ = p_v0_9; let a_ = (((l_p_22).wrapping_add(56)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = p_u0_8; let a_ = ((l_p_22).wrapping_add(56)).wrapping_add((((1i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
        { let v_ = p_v1_11; let a_ = (((l_p_22).wrapping_add(56)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = p_u1_10; let a_ = ((l_p_22).wrapping_add(56)).wrapping_add((((2i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
        { let v_ = p_v0_9; let a_ = (((l_p_22).wrapping_add(56)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = p_u1_10; let a_ = ((l_p_22).wrapping_add(56)).wrapping_add((((3i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
        { let v_ = p_v1_11; let a_ = (((l_p_22).wrapping_add(56)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = ((((c.m.r32(fp) as i32)).wrapping_add((c.m.r32((fp.wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(16))) as i32))) as u64); let a_ = (((l_p_22).wrapping_add(56)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = ((((c.m.r32((fp).wrapping_add(4)) as i32)).wrapping_add((c.m.r32(((fp.wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4)) as i32))) as u64); let a_ = ((((l_p_22).wrapping_add(56)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = (((c.m.r32((fp).wrapping_add(8)) as i32)) as u64); let a_ = ((((l_p_22).wrapping_add(56)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
        { let v_ = ((255i32) as u64); let a_ = ((((l_p_22).wrapping_add(56)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
        { let v_ = ((((c.m.r32(fp) as i32)).wrapping_add((c.m.r32((fp.wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(16))) as i32))) as u64); let a_ = (((l_p_22).wrapping_add(56)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = ((((c.m.r32((fp).wrapping_add(4)) as i32)).wrapping_add((c.m.r32(((fp.wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(4)) as i32))) as u64); let a_ = ((((l_p_22).wrapping_add(56)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = (((c.m.r32((fp).wrapping_add(8)) as i32)) as u64); let a_ = ((((l_p_22).wrapping_add(56)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
        { let v_ = ((255i32) as u64); let a_ = ((((l_p_22).wrapping_add(56)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
        { let v_ = ((((c.m.r32(fp) as i32)).wrapping_add((c.m.r32((fp.wrapping_add(16)).wrapping_add((((2i32) as u32)).wrapping_mul(16))) as i32))) as u64); let a_ = (((l_p_22).wrapping_add(56)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = ((((c.m.r32((fp).wrapping_add(4)) as i32)).wrapping_add((c.m.r32(((fp.wrapping_add(16)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(4)) as i32))) as u64); let a_ = ((((l_p_22).wrapping_add(56)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = (((c.m.r32((fp).wrapping_add(8)) as i32)) as u64); let a_ = ((((l_p_22).wrapping_add(56)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
        { let v_ = ((255i32) as u64); let a_ = ((((l_p_22).wrapping_add(56)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
        { let v_ = ((((c.m.r32(fp) as i32)).wrapping_add((c.m.r32((fp.wrapping_add(16)).wrapping_add((((3i32) as u32)).wrapping_mul(16))) as i32))) as u64); let a_ = (((l_p_22).wrapping_add(56)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(8); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = ((((c.m.r32((fp).wrapping_add(4)) as i32)).wrapping_add((c.m.r32(((fp.wrapping_add(16)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(4)) as i32))) as u64); let a_ = ((((l_p_22).wrapping_add(56)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(8)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = (((c.m.r32((fp).wrapping_add(8)) as i32)) as u64); let a_ = ((((l_p_22).wrapping_add(56)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(8)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
        { let v_ = ((255i32) as u64); let a_ = ((((l_p_22).wrapping_add(56)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(8)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
        l_sz_29 = ((c.m.r32((fp).wrapping_add(8)) as i32)).wrapping_shr((8i32) as u32);
        if (((((l_sz_29) < (0i32)) as i32)) != 0) {
            l_e_23 = ((c.m.r32(sym::gOtZ)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((l_l_30) as u32)).wrapping_mul(8));
        } else {
            if (((((l_sz_29) >= (4096i32)) as i32)) != 0) {
                l_e_23 = ((c.m.r32(sym::gOtZ)).wrapping_add((((4095i32) as u32)).wrapping_mul(16))).wrapping_add((((l_l_30) as u32)).wrapping_mul(8));
            } else {
                l_e_23 = ((c.m.r32(sym::gOtZ)).wrapping_add((((l_sz_29) as u32)).wrapping_mul(16))).wrapping_add((((l_l_30) as u32)).wrapping_mul(8));
            }
        }
        { let v_ = l_p_22; let a_ = (c.m.r32((l_e_23).wrapping_add(4))).wrapping_add(4); c.m.w32(a_, v_); };
        { let v_ = l_p_22; let a_ = (l_e_23).wrapping_add(4); c.m.w32(a_, v_); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftSpr_DrawRot", &PROBE_N, &[], true); }
    r_
}
