#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn EftPrim_DrawBillboard(c: &mut Ctx, mut p_pos_1: u32, mut p_color_2: u32, mut p_layer_3: i32, mut p_noDepth_4: i32, mut p_tex0_5: u64, mut p_w_6: F32, mut p_h_7: F32, mut p_u0_8: F32, mut p_v0_9: F32, mut p_u1_10: F32, mut p_v1_11: F32, mut p_rot_12: F32, mut p_zScale_13: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftPrim_DrawBillboard", &PROBE_N, &[(p_pos_1) as u32, (p_color_2) as u32, (p_layer_3) as u32, (p_noDepth_4) as u32, (p_tex0_5) as u32, (p_w_6).0, (p_h_7).0, (p_u0_8).0, (p_v0_9).0, (p_u1_10).0, (p_v1_11).0, (p_rot_12).0, (p_zScale_13).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(160) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 160);
    let mut l_scale_14: F32 = F32(0);
    let mut l_sw_15: i32 = 0;
    let mut l_sh_16: i32 = 0;
    let mut l_z_17: i32 = 0;
    let mut l_oz_18: i32 = 0;
    let mut l_x0_19: i32 = 0;
    let mut l_y0_20: i32 = 0;
    let mut l_zz_21: i32 = 0;
    let mut l_x1_22: i32 = 0;
    let mut l_y1_23: i32 = 0;
    let mut l_x2_24: i32 = 0;
    let mut l_y2_25: i32 = 0;
    let mut l_x3_26: i32 = 0;
    let mut l_y3_27: i32 = 0;
    let mut l_abe_28: i32 = 0;
    let mut l_ctx_29: i32 = 0;
    let mut l_l_30: i32 = 0;
    let mut l_alpha_31: u32 = 0;
    let mut l_p_32: u32 = 0;
    let mut l_e_33: u32 = 0;
    let r_ = 'f: {
        l_scale_14 = { let a_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(564); F32(c.m.r32(a_)) };
        l_abe_28 = 1i32;
        { let a2_ = rt::fmul(F32(0x3f955555), F::Port_WideFactor(c)); F::Vec4_Set(c, fp.wrapping_add(64), F32(0x3f800000), a2_, F32(0x3f800000), F32(0x3f800000)) };
        { let v_ = F32(0x3f800000); let a_ = (p_pos_1).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let _ = F::Vu0Cur_ProjectPoint(c, fp.wrapping_add(112), p_pos_1); };
        l_sw_15 = rt::f2i(p_w_6);
        l_sh_16 = rt::f2i(p_h_7);
        { let r_ = F32(0x45800000); let n_ = rt::fmul(l_scale_14, r_); l_scale_14 = n_; };
        l_z_17 = ((c.m.r32((fp.wrapping_add(112)).wrapping_add(8)) as i32)).wrapping_shr((8i32) as u32);
        { let r_ = rt::i2f((c.m.r32((fp.wrapping_add(112)).wrapping_add(12)) as i32)); let n_ = rt::fdiv(l_scale_14, r_); l_scale_14 = n_; };
        l_sw_15 = ((l_sw_15).wrapping_mul(rt::f2i(l_scale_14))).wrapping_shr((12i32) as u32);
        l_sh_16 = ((l_sh_16).wrapping_mul(rt::f2i(l_scale_14))).wrapping_shr((12i32) as u32);
        if (((((((((l_sw_15) < (2i32)) as i32)) != 0) || (((((l_sh_16) < (2i32)) as i32)) != 0)) as i32)) != 0) {
            break 'f;
        }
        p_w_6 = rt::i2f(l_sw_15);
        p_h_7 = rt::i2f(l_sh_16);
        F::Vec4_Set(c, fp.wrapping_add(80), p_w_6, p_h_7, F32(0x00000000), F32(0x3f800000));
        F::Vec4_Set(c, fp.wrapping_add(96), rt::fneg(p_w_6), p_h_7, F32(0x00000000), F32(0x3f800000));
        F::Mtx_StoreIdentity(c, fp);
        F::Mtx_RotateZ(c, fp, fp, p_rot_12);
        F::Mtx_MulVec4(c, fp.wrapping_add(80), fp, fp.wrapping_add(80));
        F::Mtx_MulVec4(c, fp.wrapping_add(96), fp, fp.wrapping_add(96));
        F::Vec4_Mul(c, fp.wrapping_add(80), fp.wrapping_add(80), fp.wrapping_add(64));
        F::Vec4_Mul(c, fp.wrapping_add(96), fp.wrapping_add(96), fp.wrapping_add(64));
        F::Vec4_ToInt(c, fp.wrapping_add(128), fp.wrapping_add(80));
        F::Vec4_ToInt(c, fp.wrapping_add(144), fp.wrapping_add(96));
        if (({ let a0_ = ((c.m.r32(fp.wrapping_add(112)) as i32)).wrapping_sub((c.m.r32(fp.wrapping_add(128)) as i32)); let a1_ = ((c.m.r32((fp.wrapping_add(112)).wrapping_add(4)) as i32)).wrapping_sub((c.m.r32((fp.wrapping_add(128)).wrapping_add(4)) as i32)); let a2_ = (c.m.r32((fp.wrapping_add(112)).wrapping_add(8)) as i32); F::EftPrim_IsOffScreen(c, a0_, a1_, a2_) }) != 0) {
            break 'f;
        }
        if (({ let a0_ = ((c.m.r32(fp.wrapping_add(112)) as i32)).wrapping_add((c.m.r32(fp.wrapping_add(144)) as i32)); let a1_ = ((c.m.r32((fp.wrapping_add(112)).wrapping_add(4)) as i32)).wrapping_add((c.m.r32((fp.wrapping_add(144)).wrapping_add(4)) as i32)); let a2_ = (c.m.r32((fp.wrapping_add(112)).wrapping_add(8)) as i32); F::EftPrim_IsOffScreen(c, a0_, a1_, a2_) }) != 0) {
            break 'f;
        }
        if (({ let a0_ = ((c.m.r32(fp.wrapping_add(112)) as i32)).wrapping_sub((c.m.r32(fp.wrapping_add(144)) as i32)); let a1_ = ((c.m.r32((fp.wrapping_add(112)).wrapping_add(4)) as i32)).wrapping_sub((c.m.r32((fp.wrapping_add(144)).wrapping_add(4)) as i32)); let a2_ = (c.m.r32((fp.wrapping_add(112)).wrapping_add(8)) as i32); F::EftPrim_IsOffScreen(c, a0_, a1_, a2_) }) != 0) {
            break 'f;
        }
        if (({ let a0_ = ((c.m.r32(fp.wrapping_add(112)) as i32)).wrapping_add((c.m.r32(fp.wrapping_add(128)) as i32)); let a1_ = ((c.m.r32((fp.wrapping_add(112)).wrapping_add(4)) as i32)).wrapping_add((c.m.r32((fp.wrapping_add(128)).wrapping_add(4)) as i32)); let a2_ = (c.m.r32((fp.wrapping_add(112)).wrapping_add(8)) as i32); F::EftPrim_IsOffScreen(c, a0_, a1_, a2_) }) != 0) {
            break 'f;
        }
        if ((p_noDepth_4) != 0) {
            { let v_ = 16777215i32; let a_ = (fp.wrapping_add(112)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        }
        l_p_32 = c.m.r32(sym::gOtCur);
        { let v_ = (l_p_32).wrapping_add((((1i32) as u32)).wrapping_mul(128)); let a_ = sym::gOtCur; c.m.w32(a_, v_); };
        l_x0_19 = ((c.m.r32(fp.wrapping_add(112)) as i32)).wrapping_sub((c.m.r32(fp.wrapping_add(128)) as i32));
        l_y0_20 = ((c.m.r32((fp.wrapping_add(112)).wrapping_add(4)) as i32)).wrapping_sub((c.m.r32((fp.wrapping_add(128)).wrapping_add(4)) as i32));
        l_x1_22 = ((c.m.r32(fp.wrapping_add(112)) as i32)).wrapping_add((c.m.r32(fp.wrapping_add(144)) as i32));
        l_y1_23 = ((c.m.r32((fp.wrapping_add(112)).wrapping_add(4)) as i32)).wrapping_add((c.m.r32((fp.wrapping_add(144)).wrapping_add(4)) as i32));
        l_x2_24 = ((c.m.r32(fp.wrapping_add(112)) as i32)).wrapping_sub((c.m.r32(fp.wrapping_add(144)) as i32));
        l_y2_25 = ((c.m.r32((fp.wrapping_add(112)).wrapping_add(4)) as i32)).wrapping_sub((c.m.r32((fp.wrapping_add(144)).wrapping_add(4)) as i32));
        l_x3_26 = ((c.m.r32(fp.wrapping_add(112)) as i32)).wrapping_add((c.m.r32(fp.wrapping_add(128)) as i32));
        l_y3_27 = ((c.m.r32((fp.wrapping_add(112)).wrapping_add(4)) as i32)).wrapping_add((c.m.r32((fp.wrapping_add(128)).wrapping_add(4)) as i32));
        l_zz_21 = (c.m.r32((fp.wrapping_add(112)).wrapping_add(8)) as i32);
        if (((((l_p_32) == (((0i32) as u32))) as i32)) != 0) {
            break 'f;
        }
        l_ctx_29 = (((p_layer_3) >= (2i32)) as i32);
        { let v_ = (((((((l_abe_28) as u64)).wrapping_shl((6i32) as u32)) | ((((l_ctx_29) as u64)).wrapping_shl((9i32) as u32)))) | (((20i32) as u64))); let a_ = (l_p_32).wrapping_add(32); c.m.w64(a_, v_); };
        { let v_ = ((536870919i32) as u32); let a_ = l_p_32; c.m.w32(a_, v_); };
        { let v_ = ((268435456i32) as u32); let a_ = (l_p_32).wrapping_add(8); c.m.w32(a_, v_); };
        { let v_ = ((1342177287i32) as u32); let a_ = (l_p_32).wrapping_add(12); c.m.w32(a_, v_); };
        { let v_ = 14123288431433908225u64; let a_ = (l_p_32).wrapping_add(16); c.m.w64(a_, v_); };
        { let v_ = (((268436062347616i64).wrapping_add((((l_ctx_29).wrapping_shl((4i32) as u32)) as i64))) as u64); let a_ = (l_p_32).wrapping_add(24); c.m.w64(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = (l_p_32).wrapping_add(4); c.m.w32(a_, v_); };
        { let v_ = ((rt::f2i(F32(c.m.r32(p_color_2)))) as u8); let a_ = (l_p_32).wrapping_add(48); c.m.w8(a_, v_); };
        { let v_ = ((rt::f2i(F32(c.m.r32((p_color_2).wrapping_add(4))))) as u8); let a_ = ((l_p_32).wrapping_add(48)).wrapping_add(1); c.m.w8(a_, v_); };
        { let v_ = ((rt::f2i(F32(c.m.r32((p_color_2).wrapping_add(8))))) as u8); let a_ = ((l_p_32).wrapping_add(48)).wrapping_add(2); c.m.w8(a_, v_); };
        l_alpha_31 = rt::f2u(F32(c.m.r32((p_color_2).wrapping_add(12))));
        { let v_ = ((l_x0_19) as u64); let a_ = (l_p_32).wrapping_add(64); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = ((l_y0_20) as u64); let a_ = ((l_p_32).wrapping_add(64)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        l_l_30 = p_layer_3;
        { let v_ = ((l_x1_22) as u64); let a_ = (l_p_32).wrapping_add(80); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = ((l_y1_23) as u64); let a_ = ((l_p_32).wrapping_add(80)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = ((l_x2_24) as u64); let a_ = (l_p_32).wrapping_add(96); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = ((l_y2_25) as u64); let a_ = ((l_p_32).wrapping_add(96)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = ((l_x3_26) as u64); let a_ = (l_p_32).wrapping_add(112); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = ((l_y3_27) as u64); let a_ = ((l_p_32).wrapping_add(112)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        l_oz_18 = rt::f2i(rt::fmul(rt::i2f(l_z_17), p_zScale_13));
        { let v_ = ((l_zz_21) as u64); let a_ = ((l_p_32).wrapping_add(64)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
        { let v_ = ((l_zz_21) as u64); let a_ = ((l_p_32).wrapping_add(80)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
        { let v_ = ((l_zz_21) as u64); let a_ = ((l_p_32).wrapping_add(96)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
        { let v_ = ((l_zz_21) as u64); let a_ = ((l_p_32).wrapping_add(112)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
        if (((((l_l_30) >= (2i32)) as i32)) != 0) {
            { let r_ = 2i32; let n_ = (l_l_30).wrapping_sub(r_); l_l_30 = n_; };
        }
        { let v_ = ((l_alpha_31) as u8); let a_ = ((l_p_32).wrapping_add(48)).wrapping_add(3); c.m.w8(a_, v_); };
        { let v_ = F32(0x3f800000); let a_ = ((l_p_32).wrapping_add(48)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = p_u0_8; let a_ = (l_p_32).wrapping_add(56); c.m.w32(a_, (v_).0); };
        { let v_ = p_v0_9; let a_ = ((l_p_32).wrapping_add(56)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = p_u0_8; let a_ = (l_p_32).wrapping_add(72); c.m.w32(a_, (v_).0); };
        { let v_ = p_v1_11; let a_ = ((l_p_32).wrapping_add(72)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = p_u1_10; let a_ = (l_p_32).wrapping_add(88); c.m.w32(a_, (v_).0); };
        { let v_ = p_v0_9; let a_ = ((l_p_32).wrapping_add(88)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = p_u1_10; let a_ = (l_p_32).wrapping_add(104); c.m.w32(a_, (v_).0); };
        { let v_ = p_v1_11; let a_ = ((l_p_32).wrapping_add(104)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = ((255i32) as u64); let a_ = ((l_p_32).wrapping_add(64)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
        { let v_ = ((255i32) as u64); let a_ = ((l_p_32).wrapping_add(80)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
        { let v_ = ((255i32) as u64); let a_ = ((l_p_32).wrapping_add(96)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
        { let v_ = ((255i32) as u64); let a_ = ((l_p_32).wrapping_add(112)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
        { let v_ = p_tex0_5; let a_ = (l_p_32).wrapping_add(40); c.m.w64(a_, v_); };
        if (((((l_oz_18) < (0i32)) as i32)) != 0) {
            l_e_33 = ((c.m.r32(sym::gOtZ)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((l_l_30) as u32)).wrapping_mul(8));
        } else {
            if (((((l_oz_18) >= (4096i32)) as i32)) != 0) {
                l_e_33 = ((c.m.r32(sym::gOtZ)).wrapping_add((((4095i32) as u32)).wrapping_mul(16))).wrapping_add((((l_l_30) as u32)).wrapping_mul(8));
            } else {
                l_e_33 = ((c.m.r32(sym::gOtZ)).wrapping_add((((l_oz_18) as u32)).wrapping_mul(16))).wrapping_add((((l_l_30) as u32)).wrapping_mul(8));
            }
        }
        { let v_ = l_p_32; let a_ = (c.m.r32((l_e_33).wrapping_add(4))).wrapping_add(4); c.m.w32(a_, v_); };
        { let v_ = l_p_32; let a_ = (l_e_33).wrapping_add(4); c.m.w32(a_, v_); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftPrim_DrawBillboard", &PROBE_N, &[], true); }
    r_
}

pub fn EftPrim_DrawQuadDepth(c: &mut Ctx, mut p_pos_1: u32, mut p_color_2: u32, mut p_arg2_3: i32, mut p_arg3_4: i32, mut p_tex0_5: u64, mut p_w_6: F32, mut p_h_7: F32, mut p_u0_8: F32, mut p_v0_9: F32, mut p_u1_10: F32, mut p_v1_11: F32, mut p_rot_12: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftPrim_DrawQuadDepth", &PROBE_N, &[(p_pos_1) as u32, (p_color_2) as u32, (p_arg2_3) as u32, (p_arg3_4) as u32, (p_tex0_5) as u32, (p_w_6).0, (p_h_7).0, (p_u0_8).0, (p_v0_9).0, (p_u1_10).0, (p_v1_11).0, (p_rot_12).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(704) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 704);
    let mut l_i_13: i32 = 0;
    let mut l_z_14: i32 = 0;
    let r_ = 'f: {
        F::Vec4_Set(c, (fp.wrapping_add(560)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), rt::fneg(p_w_6), rt::fneg(p_h_7), F32(0x00000000), F32(0x3f800000));
        F::Vec4_Set(c, (fp.wrapping_add(560)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), p_w_6, rt::fneg(p_h_7), F32(0x00000000), F32(0x3f800000));
        F::Vec4_Set(c, (fp.wrapping_add(560)).wrapping_add((((2i32) as u32)).wrapping_mul(16)), rt::fneg(p_w_6), p_h_7, F32(0x00000000), F32(0x3f800000));
        F::Vec4_Set(c, (fp.wrapping_add(560)).wrapping_add((((3i32) as u32)).wrapping_mul(16)), p_w_6, p_h_7, F32(0x00000000), F32(0x3f800000));
        F::Mtx_StoreIdentity(c, fp.wrapping_add(432));
        F::Mtx_RotateZ(c, fp.wrapping_add(432), fp.wrapping_add(432), p_rot_12);
        { let a1_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(64); F::Mtx_InverseRT(c, fp.wrapping_add(496), a1_) };
        { let v_ = { let v_ = { let v_ = F32(0x00000000); let a_ = ((fp.wrapping_add(496)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); v_ }; let a_ = ((fp.wrapping_add(496)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); v_ }; let a_ = ((fp.wrapping_add(496)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        F::Mtx_Mul(c, fp.wrapping_add(432), fp.wrapping_add(496), fp.wrapping_add(432));
        {
            l_i_13 = 0i32;
            'l1: loop {
                if !(((((l_i_13) < (4i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    F::Mtx_MulVec4(c, (fp.wrapping_add(560)).wrapping_add((((l_i_13) as u32)).wrapping_mul(16)), fp.wrapping_add(432), (fp.wrapping_add(560)).wrapping_add((((l_i_13) as u32)).wrapping_mul(16)));
                    F::Vec3_Add(c, (fp.wrapping_add(560)).wrapping_add((((l_i_13) as u32)).wrapping_mul(16)), (fp.wrapping_add(560)).wrapping_add((((l_i_13) as u32)).wrapping_mul(16)), p_pos_1);
                    { let v_ = F32(0x3f800000); let a_ = ((fp.wrapping_add(560)).wrapping_add((((l_i_13) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
                }
                { let o_ = l_i_13; let n_ = (o_).wrapping_add(1); l_i_13 = n_; };
            }
        }
        { let _ = F::Vu0Cur_ProjectPoint(c, fp.wrapping_add(688), p_pos_1); };
        l_z_14 = ((c.m.r32((fp.wrapping_add(688)).wrapping_add(8)) as i32)).wrapping_shr((8i32) as u32);
        if ((((((c.m.r32((fp.wrapping_add(688)).wrapping_add(8)) as i32)) >= (0i32)) as i32)) != 0) {
            F::Vec4_Set(c, (fp.wrapping_add(624)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), p_u0_8, p_v0_9, F32(0x3f800000), F32(0x00000000));
            F::Vec4_Set(c, (fp.wrapping_add(624)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), p_u1_10, p_v0_9, F32(0x3f800000), F32(0x00000000));
            F::Vec4_Set(c, (fp.wrapping_add(624)).wrapping_add((((2i32) as u32)).wrapping_mul(16)), p_u0_8, p_v1_11, F32(0x3f800000), F32(0x00000000));
            F::Vec4_Set(c, (fp.wrapping_add(624)).wrapping_add((((3i32) as u32)).wrapping_mul(16)), p_u1_10, p_v1_11, F32(0x3f800000), F32(0x00000000));
            {
                l_i_13 = 0i32;
                'l3: loop {
                    if !(((((l_i_13) < (2i32)) as i32)) != 0) { break 'l3; }
                    'c4: {
                        F::ClipVtx_Set(c, (fp).wrapping_add((((0i32) as u32)).wrapping_mul(48)), (fp.wrapping_add(560)).wrapping_add((((l_i_13) as u32)).wrapping_mul(16)), (fp.wrapping_add(624)).wrapping_add((((l_i_13) as u32)).wrapping_mul(16)), p_color_2);
                        F::ClipVtx_Set(c, (fp).wrapping_add((((1i32) as u32)).wrapping_mul(48)), (fp.wrapping_add(560)).wrapping_add(((((l_i_13).wrapping_add(1i32)) as u32)).wrapping_mul(16)), (fp.wrapping_add(624)).wrapping_add(((((l_i_13).wrapping_add(1i32)) as u32)).wrapping_mul(16)), p_color_2);
                        F::ClipVtx_Set(c, (fp).wrapping_add((((2i32) as u32)).wrapping_mul(48)), (fp.wrapping_add(560)).wrapping_add(((((l_i_13).wrapping_add(2i32)) as u32)).wrapping_mul(16)), (fp.wrapping_add(624)).wrapping_add(((((l_i_13).wrapping_add(2i32)) as u32)).wrapping_mul(16)), p_color_2);
                        F::EftGfx_DrawPolyFixedZ(c, fp, p_arg2_3, 0i32, 0i32, p_arg3_4, 0i32, p_tex0_5, l_z_14);
                    }
                    { let o_ = l_i_13; let n_ = (o_).wrapping_add(1); l_i_13 = n_; };
                }
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftPrim_DrawQuadDepth", &PROBE_N, &[], true); }
    r_
}

pub fn EftPrim_DrawQuadDepthScaled(c: &mut Ctx, mut p_pos_1: u32, mut p_color_2: u32, mut p_w_3: F32, mut p_h_4: F32, mut p_u0_5: F32, mut p_v0_6: F32, mut p_u1_7: F32, mut p_v1_8: F32, mut p_rot_9: F32, mut p_arg2_10: i32, mut p_arg3_11: i32, mut p_tex0_12: u64, mut p_zScale_13: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftPrim_DrawQuadDepthScaled", &PROBE_N, &[(p_pos_1) as u32, (p_color_2) as u32, (p_w_3).0, (p_h_4).0, (p_u0_5).0, (p_v0_6).0, (p_u1_7).0, (p_v1_8).0, (p_rot_9).0, (p_arg2_10) as u32, (p_arg3_11) as u32, (p_tex0_12) as u32, (p_zScale_13).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(704) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 704);
    let mut l_i_14: i32 = 0;
    let mut l_z_15: i32 = 0;
    let r_ = 'f: {
        F::Vec4_Set(c, (fp.wrapping_add(560)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), rt::fneg(p_w_3), rt::fneg(p_h_4), F32(0x00000000), F32(0x3f800000));
        F::Vec4_Set(c, (fp.wrapping_add(560)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), p_w_3, rt::fneg(p_h_4), F32(0x00000000), F32(0x3f800000));
        F::Vec4_Set(c, (fp.wrapping_add(560)).wrapping_add((((2i32) as u32)).wrapping_mul(16)), rt::fneg(p_w_3), p_h_4, F32(0x00000000), F32(0x3f800000));
        F::Vec4_Set(c, (fp.wrapping_add(560)).wrapping_add((((3i32) as u32)).wrapping_mul(16)), p_w_3, p_h_4, F32(0x00000000), F32(0x3f800000));
        F::Mtx_StoreIdentity(c, fp.wrapping_add(432));
        F::Mtx_RotateZ(c, fp.wrapping_add(432), fp.wrapping_add(432), p_rot_9);
        { let a1_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(64); F::Mtx_InverseRT(c, fp.wrapping_add(496), a1_) };
        { let v_ = { let v_ = { let v_ = F32(0x00000000); let a_ = ((fp.wrapping_add(496)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); v_ }; let a_ = ((fp.wrapping_add(496)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); v_ }; let a_ = ((fp.wrapping_add(496)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        F::Mtx_Mul(c, fp.wrapping_add(432), fp.wrapping_add(496), fp.wrapping_add(432));
        {
            l_i_14 = 0i32;
            'l1: loop {
                if !(((((l_i_14) < (4i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    F::Mtx_MulVec4(c, (fp.wrapping_add(560)).wrapping_add((((l_i_14) as u32)).wrapping_mul(16)), fp.wrapping_add(432), (fp.wrapping_add(560)).wrapping_add((((l_i_14) as u32)).wrapping_mul(16)));
                    F::Vec3_Add(c, (fp.wrapping_add(560)).wrapping_add((((l_i_14) as u32)).wrapping_mul(16)), (fp.wrapping_add(560)).wrapping_add((((l_i_14) as u32)).wrapping_mul(16)), p_pos_1);
                    { let v_ = F32(0x3f800000); let a_ = ((fp.wrapping_add(560)).wrapping_add((((l_i_14) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
                }
                { let o_ = l_i_14; let n_ = (o_).wrapping_add(1); l_i_14 = n_; };
            }
        }
        { let _ = F::Vu0Cur_ProjectPoint(c, fp.wrapping_add(688), p_pos_1); };
        l_z_15 = ((c.m.r32((fp.wrapping_add(688)).wrapping_add(8)) as i32)).wrapping_shr((8i32) as u32);
        if ((((((c.m.r32((fp.wrapping_add(688)).wrapping_add(8)) as i32)) >= (0i32)) as i32)) != 0) {
            F::Vec4_Set(c, (fp.wrapping_add(624)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), p_u0_5, p_v0_6, F32(0x3f800000), F32(0x00000000));
            F::Vec4_Set(c, (fp.wrapping_add(624)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), p_u1_7, p_v0_6, F32(0x3f800000), F32(0x00000000));
            F::Vec4_Set(c, (fp.wrapping_add(624)).wrapping_add((((2i32) as u32)).wrapping_mul(16)), p_u0_5, p_v1_8, F32(0x3f800000), F32(0x00000000));
            F::Vec4_Set(c, (fp.wrapping_add(624)).wrapping_add((((3i32) as u32)).wrapping_mul(16)), p_u1_7, p_v1_8, F32(0x3f800000), F32(0x00000000));
            {
                l_i_14 = 0i32;
                'l3: loop {
                    if !(((((l_i_14) < (2i32)) as i32)) != 0) { break 'l3; }
                    'c4: {
                        F::ClipVtx_Set(c, (fp).wrapping_add((((0i32) as u32)).wrapping_mul(48)), (fp.wrapping_add(560)).wrapping_add((((l_i_14) as u32)).wrapping_mul(16)), (fp.wrapping_add(624)).wrapping_add((((l_i_14) as u32)).wrapping_mul(16)), p_color_2);
                        F::ClipVtx_Set(c, (fp).wrapping_add((((1i32) as u32)).wrapping_mul(48)), (fp.wrapping_add(560)).wrapping_add(((((l_i_14).wrapping_add(1i32)) as u32)).wrapping_mul(16)), (fp.wrapping_add(624)).wrapping_add(((((l_i_14).wrapping_add(1i32)) as u32)).wrapping_mul(16)), p_color_2);
                        F::ClipVtx_Set(c, (fp).wrapping_add((((2i32) as u32)).wrapping_mul(48)), (fp.wrapping_add(560)).wrapping_add(((((l_i_14).wrapping_add(2i32)) as u32)).wrapping_mul(16)), (fp.wrapping_add(624)).wrapping_add(((((l_i_14).wrapping_add(2i32)) as u32)).wrapping_mul(16)), p_color_2);
                        F::EftGfx_DrawPolyFixedZ(c, fp, p_arg2_10, 0i32, 0i32, p_arg3_11, 0i32, p_tex0_12, rt::f2i(rt::fmul(rt::i2f(l_z_15), p_zScale_13)));
                    }
                    { let o_ = l_i_14; let n_ = (o_).wrapping_add(1); l_i_14 = n_; };
                }
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftPrim_DrawQuadDepthScaled", &PROBE_N, &[], true); }
    r_
}

pub fn EftPrim_DrawTriangle(c: &mut Ctx, mut p_v0_1: u32, mut p_v1_2: u32, mut p_v2_3: u32, mut p_c0_4: u32, mut p_c1_5: u32, mut p_c2_6: u32, mut p_uv0_7: u32, mut p_uv1_8: u32, mut p_uv2_9: u32, mut p_unk1_10: i32, mut p_unk2_11: i32, mut p_layer_12: i32, mut p_z_13: i32, mut p_tex0_14: u64) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftPrim_DrawTriangle", &PROBE_N, &[(p_v0_1) as u32, (p_v1_2) as u32, (p_v2_3) as u32, (p_c0_4) as u32, (p_c1_5) as u32, (p_c2_6) as u32, (p_uv0_7) as u32, (p_uv1_8) as u32, (p_uv2_9) as u32, (p_unk1_10) as u32, (p_unk2_11) as u32, (p_layer_12) as u32, (p_z_13) as u32, (p_tex0_14) as u32], false); }
    let mut l_abe_15: i32 = 0;
    let mut l_ctx_16: i32 = 0;
    let mut l_l_17: i32 = 0;
    let mut l_p_18: u32 = 0;
    let mut l_e_19: u32 = 0;
    let r_ = 'f: {
    l_abe_15 = 1i32;
    if (((((((((((rt::fle(F32(c.m.r32((p_c0_4).wrapping_add(12))), F32(0x3c23d70a)) as i32)) != 0) && (((rt::fle(F32(c.m.r32((p_c1_5).wrapping_add(12))), F32(0x3c23d70a)) as i32)) != 0)) as i32)) != 0) && (((rt::fle(F32(c.m.r32((p_c2_6).wrapping_add(12))), F32(0x3c23d70a)) as i32)) != 0)) as i32)) != 0) {
        break 'f;
    }
    if (((((p_layer_12) < (0i32)) as i32)) != 0) {
        p_layer_12 = 0i32;
        l_abe_15 = 0i32;
    }
    l_ctx_16 = (((p_layer_12) >= (2i32)) as i32);
    l_p_18 = c.m.r32(sym::gOtCur);
    { let v_ = (l_p_18).wrapping_add((((1i32) as u32)).wrapping_mul(128)); let a_ = sym::gOtCur; c.m.w32(a_, v_); };
    { let v_ = (((((((l_abe_15) as u64)).wrapping_shl((6i32) as u32)) | ((((l_ctx_16) as u64)).wrapping_shl((9i32) as u32)))) | (((27i32) as u64))); let a_ = (l_p_18).wrapping_add(32); c.m.w64(a_, v_); };
    { let v_ = ((536870919i32) as u32); let a_ = l_p_18; c.m.w32(a_, v_); };
    { let v_ = ((268435456i32) as u32); let a_ = (l_p_18).wrapping_add(8); c.m.w32(a_, v_); };
    { let v_ = ((1342177287i32) as u32); let a_ = (l_p_18).wrapping_add(12); c.m.w32(a_, v_); };
    { let v_ = 14123288431433908225u64; let a_ = (l_p_18).wrapping_add(16); c.m.w64(a_, v_); };
    { let v_ = (((268423679713632i64).wrapping_add((((l_ctx_16).wrapping_shl((4i32) as u32)) as i64))) as u64); let a_ = (l_p_18).wrapping_add(24); c.m.w64(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (l_p_18).wrapping_add(4); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u64); let a_ = (l_p_18).wrapping_add(120); c.m.w64(a_, v_); };
    { let v_ = ((rt::f2i(F32(c.m.r32(p_c0_4)))) as u8); let a_ = (l_p_18).wrapping_add(48); c.m.w8(a_, v_); };
    { let v_ = ((rt::f2i(F32(c.m.r32((p_c0_4).wrapping_add(4))))) as u8); let a_ = ((l_p_18).wrapping_add(48)).wrapping_add(1); c.m.w8(a_, v_); };
    { let v_ = ((rt::f2i(F32(c.m.r32((p_c0_4).wrapping_add(8))))) as u8); let a_ = ((l_p_18).wrapping_add(48)).wrapping_add(2); c.m.w8(a_, v_); };
    { let v_ = ((rt::f2i(F32(c.m.r32((p_c0_4).wrapping_add(12))))) as u8); let a_ = ((l_p_18).wrapping_add(48)).wrapping_add(3); c.m.w8(a_, v_); };
    { let v_ = F32(c.m.r32((p_uv0_7).wrapping_add(8))); let a_ = ((l_p_18).wrapping_add(48)).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = ((rt::f2i(F32(c.m.r32(p_c1_5)))) as u8); let a_ = (l_p_18).wrapping_add(72); c.m.w8(a_, v_); };
    { let v_ = ((rt::f2i(F32(c.m.r32((p_c1_5).wrapping_add(4))))) as u8); let a_ = ((l_p_18).wrapping_add(72)).wrapping_add(1); c.m.w8(a_, v_); };
    { let v_ = ((rt::f2i(F32(c.m.r32((p_c1_5).wrapping_add(8))))) as u8); let a_ = ((l_p_18).wrapping_add(72)).wrapping_add(2); c.m.w8(a_, v_); };
    { let v_ = ((rt::f2i(F32(c.m.r32((p_c1_5).wrapping_add(12))))) as u8); let a_ = ((l_p_18).wrapping_add(72)).wrapping_add(3); c.m.w8(a_, v_); };
    { let v_ = F32(c.m.r32((p_uv1_8).wrapping_add(8))); let a_ = ((l_p_18).wrapping_add(72)).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = ((rt::f2i(F32(c.m.r32(p_c2_6)))) as u8); let a_ = (l_p_18).wrapping_add(96); c.m.w8(a_, v_); };
    { let v_ = ((rt::f2i(F32(c.m.r32((p_c2_6).wrapping_add(4))))) as u8); let a_ = ((l_p_18).wrapping_add(96)).wrapping_add(1); c.m.w8(a_, v_); };
    { let v_ = ((rt::f2i(F32(c.m.r32((p_c2_6).wrapping_add(8))))) as u8); let a_ = ((l_p_18).wrapping_add(96)).wrapping_add(2); c.m.w8(a_, v_); };
    { let v_ = ((rt::f2i(F32(c.m.r32((p_c2_6).wrapping_add(12))))) as u8); let a_ = ((l_p_18).wrapping_add(96)).wrapping_add(3); c.m.w8(a_, v_); };
    l_l_17 = p_layer_12;
    if (((((l_l_17) >= (2i32)) as i32)) != 0) {
        { let r_ = 2i32; let n_ = (l_l_17).wrapping_sub(r_); l_l_17 = n_; };
    }
    { let v_ = F32(c.m.r32((p_uv2_9).wrapping_add(8))); let a_ = ((l_p_18).wrapping_add(96)).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = p_tex0_14; let a_ = (l_p_18).wrapping_add(40); c.m.w64(a_, v_); };
    { let v_ = F32(c.m.r32(p_uv0_7)); let a_ = (l_p_18).wrapping_add(56); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((p_uv0_7).wrapping_add(4))); let a_ = ((l_p_18).wrapping_add(56)).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32(p_uv1_8)); let a_ = (l_p_18).wrapping_add(80); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((p_uv1_8).wrapping_add(4))); let a_ = ((l_p_18).wrapping_add(80)).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32(p_uv2_9)); let a_ = (l_p_18).wrapping_add(104); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((p_uv2_9).wrapping_add(4))); let a_ = ((l_p_18).wrapping_add(104)).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = (((c.m.r32(p_v0_1) as i32)) as u64); let a_ = (l_p_18).wrapping_add(64); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
    { let v_ = (((c.m.r32((p_v0_1).wrapping_add(4)) as i32)) as u64); let a_ = ((l_p_18).wrapping_add(64)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
    { let v_ = (((c.m.r32((p_v0_1).wrapping_add(8)) as i32)) as u64); let a_ = ((l_p_18).wrapping_add(64)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
    { let v_ = ((255i32) as u64); let a_ = ((l_p_18).wrapping_add(64)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
    { let v_ = (((c.m.r32(p_v1_2) as i32)) as u64); let a_ = (l_p_18).wrapping_add(88); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
    { let v_ = (((c.m.r32((p_v1_2).wrapping_add(4)) as i32)) as u64); let a_ = ((l_p_18).wrapping_add(88)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
    { let v_ = (((c.m.r32((p_v1_2).wrapping_add(8)) as i32)) as u64); let a_ = ((l_p_18).wrapping_add(88)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
    { let v_ = ((255i32) as u64); let a_ = ((l_p_18).wrapping_add(88)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
    { let v_ = (((c.m.r32(p_v2_3) as i32)) as u64); let a_ = (l_p_18).wrapping_add(112); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
    { let v_ = (((c.m.r32((p_v2_3).wrapping_add(4)) as i32)) as u64); let a_ = ((l_p_18).wrapping_add(112)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
    { let v_ = (((c.m.r32((p_v2_3).wrapping_add(8)) as i32)) as u64); let a_ = ((l_p_18).wrapping_add(112)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
    { let v_ = ((255i32) as u64); let a_ = ((l_p_18).wrapping_add(112)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
    if (((((p_z_13) < (0i32)) as i32)) != 0) {
        l_e_19 = ((c.m.r32(sym::gOtZ)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((l_l_17) as u32)).wrapping_mul(8));
    } else {
        if (((((p_z_13) >= (4096i32)) as i32)) != 0) {
            l_e_19 = ((c.m.r32(sym::gOtZ)).wrapping_add((((4095i32) as u32)).wrapping_mul(16))).wrapping_add((((l_l_17) as u32)).wrapping_mul(8));
        } else {
            l_e_19 = ((c.m.r32(sym::gOtZ)).wrapping_add((((p_z_13) as u32)).wrapping_mul(16))).wrapping_add((((l_l_17) as u32)).wrapping_mul(8));
        }
    }
    { let v_ = l_p_18; let a_ = (c.m.r32((l_e_19).wrapping_add(4))).wrapping_add(4); c.m.w32(a_, v_); };
    { let v_ = l_p_18; let a_ = (l_e_19).wrapping_add(4); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftPrim_DrawTriangle", &PROBE_N, &[], true); }
    r_
}

pub fn EftPrim_IsOffScreen(c: &mut Ctx, mut p_x_1: i32, mut p_y_2: i32, mut p_z_3: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftPrim_IsOffScreen", &PROBE_N, &[(p_x_1) as u32, (p_y_2) as u32, (p_z_3) as u32], false); }
    let r_ = 'f: {
    if (((((p_z_3) <= (0i32)) as i32)) != 0) {
        break 'f 1i32;
    } else {
        if (((((p_x_1) > (65519i32)) as i32)) != 0) {
            break 'f 1i32;
        } else {
            if (((((p_x_1) <= (0i32)) as i32)) != 0) {
                break 'f 1i32;
            } else {
                if (((((p_y_2) > (65519i32)) as i32)) != 0) {
                    break 'f 1i32;
                } else {
                    if (((((p_y_2) <= (0i32)) as i32)) != 0) {
                        break 'f 1i32;
                    }
                }
            }
        }
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftPrim_IsOffScreen", &PROBE_N, &[], true); }
    r_
}

pub fn EftBubble_StartBurst(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBubble_StartBurst", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let mut l_n_3: F32 = F32(0);
    let r_ = 'f: {
    if (((((c.m.r32(sym::gEftBubble)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    l_chr_2 = ({ let a_ = c.m.r32(sym::gEftBubble); c.m.r32(a_) }).wrapping_add((((p_objId_1) as u32)).wrapping_mul(16));
    if (((rt::bf_get_u(c, (l_chr_2).wrapping_add(12), 1, 1) as u8)) != 0) {
        break 'f 0i32;
    }
    l_n_3 = rt::fmul(F::BtlCharApi_GetSpeed(c, p_objId_1), F32(0x40a00000));
    { let v_ = p_objId_1; let a_ = l_chr_2; c.m.w32(a_, (v_) as u32); };
    if (((rt::flt(l_n_3, F32(0x42200000)) as i32)) != 0) {
        { let v_ = F32(0x42200000); let a_ = (l_chr_2).wrapping_add(8); c.m.w32(a_, (v_).0); };
    } else {
        if (((rt::fgt(l_n_3, F32(0x42700000)) as i32)) != 0) {
            { let v_ = F32(0x42700000); let a_ = (l_chr_2).wrapping_add(8); c.m.w32(a_, (v_).0); };
        } else {
            { let v_ = l_n_3; let a_ = (l_chr_2).wrapping_add(8); c.m.w32(a_, (v_).0); };
        }
    }
    { let v_ = 0i32; let a_ = (l_chr_2).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
    { let v_ = ((1i32) as u8); let a_ = (l_chr_2).wrapping_add(12); rt::bf_set(c, a_, 1, 1, (v_) as u64);  };
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBubble_StartBurst", &PROBE_N, &[], true); }
    r_
}

pub fn EftBubble_OnBlastRecord(c: &mut Ctx, mut p_rec_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBubble_OnBlastRecord", &PROBE_N, &[(p_rec_1) as u32], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBubble_OnBlastRecord", &PROBE_N, &[], true); }
    r_
}

pub fn EftBubble_Init(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBubble_Init", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_stage_2: u32 = 0;
    let r_ = 'f: {
    if (((((c.m.r32(sym::gEftBubble)) == (((0i32) as u32))) as i32)) != 0) {
        { let v_ = { let a0_ = F::BtlPool_GetCurrent(c); F::BtlPool_Alloc(c, a0_, ((624u32) as i32)) }; let a_ = sym::gEftBubble; c.m.w32(a_, v_); };
        { let _ = { let a0_ = c.m.r32(sym::gEftBubble); F::memset(c, a0_, 0i32, 624u32) }; };
    }
    if ((((({ let a_ = c.m.r32(sym::gEftBubble); c.m.r32(a_) }) == (((0i32) as u32))) as i32)) != 0) {
        { let v_ = { let a0_ = F::BtlPool_GetCurrent(c); F::BtlPool_Alloc(c, a0_, (((((2i32) as u32)).wrapping_mul(16u32)) as i32)) }; let a_ = c.m.r32(sym::gEftBubble); c.m.w32(a_, v_); };
        { let _ = { let a0_ = { let a_ = c.m.r32(sym::gEftBubble); c.m.r32(a_) }; F::memset(c, a0_, 0i32, (((2i32) as u32)).wrapping_mul(16u32)) }; };
    }
    if ((((({ let a_ = (c.m.r32(sym::gEftBubble)).wrapping_add(4); c.m.r32(a_) }) == (((0i32) as u32))) as i32)) != 0) {
        { let v_ = { let a0_ = F::BtlPool_GetCurrent(c); F::BtlPool_Alloc(c, a0_, (((((100i32) as u32)).wrapping_mul(112u32)) as i32)) }; let a_ = (c.m.r32(sym::gEftBubble)).wrapping_add(4); c.m.w32(a_, v_); };
        { let _ = { let a0_ = { let a_ = (c.m.r32(sym::gEftBubble)).wrapping_add(4); c.m.r32(a_) }; F::memset(c, a0_, 0i32, (((100i32) as u32)).wrapping_mul(112u32)) }; };
    }
    l_stage_2 = ((((F::BtlScene_GetStageData(c)) as i64 as u64)) as u32);
    if (((((F::BtlScene_GetPackEntrySize(c, l_stage_2, 18i32)) > (0i32)) as i32)) != 0) {
        { let a0_ = (c.m.r32(sym::gEftBubble)).wrapping_add(104); let a1_ = F::BtlScene_GetPackEntry(c, l_stage_2, 18i32); F::EftTexSet_Load32(c, a0_, a1_) };
        if ((((({ let a_ = ((c.m.r32(sym::gEftBubble)).wrapping_add(104)).wrapping_add(512); (c.m.r32(a_) as i32) }) < (6i32)) as i32)) != 0) {
            F::BtlTask_SetDead(c, p_task_1);
        }
    } else {
        F::BtlTask_SetDead(c, p_task_1);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBubble_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftBubble_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBubble_Term", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if ((((({ let a_ = (c.m.r32(sym::gEftBubble)).wrapping_add(4); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = F::BtlPool_GetCurrent(c); let a1_ = { let a_ = (c.m.r32(sym::gEftBubble)).wrapping_add(4); c.m.r32(a_) }; F::BtlPool_Free(c, a0_, a1_) };
        { let v_ = ((0i32) as u32); let a_ = (c.m.r32(sym::gEftBubble)).wrapping_add(4); c.m.w32(a_, v_); };
    }
    if ((((({ let a_ = c.m.r32(sym::gEftBubble); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = F::BtlPool_GetCurrent(c); let a1_ = { let a_ = c.m.r32(sym::gEftBubble); c.m.r32(a_) }; F::BtlPool_Free(c, a0_, a1_) };
        { let v_ = ((0i32) as u32); let a_ = c.m.r32(sym::gEftBubble); c.m.w32(a_, v_); };
    }
    if (((((c.m.r32(sym::gEftBubble)) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = F::BtlPool_GetCurrent(c); let a1_ = c.m.r32(sym::gEftBubble); F::BtlPool_Free(c, a0_, a1_) };
        { let v_ = ((0i32) as u32); let a_ = sym::gEftBubble; c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBubble_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftBubble_Update(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBubble_Update", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::EftBubble_UpdateChrs(c);
    F::EftBubble_UpdateAmbient(c);
    { let a0_ = (c.m.r32(sym::gEftBubble)).wrapping_add(8); let a1_ = (c.m.r32(sym::gEftBubble)).wrapping_add(12); F::EftBubble_UpdateList(c, a0_, a1_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBubble_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftBubble_PostUpdate(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBubble_PostUpdate", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::EftBubble_SetTexMode(c, 1i32, 0i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBubble_PostUpdate", &PROBE_N, &[], true); }
    r_
}

pub fn EftBubble_Draw(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBubble_Draw", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if ((F::EftStage_IsDrawOn(c)) != 0) {
        if ((F::BtlStage_IsReady(c)) != 0) {
            { let a0_ = (c.m.r32(sym::gEftBubble)).wrapping_add(32); let a1_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(64); F::Mtx_InverseRT(c, a0_, a1_) };
            { let v_ = { let v_ = { let v_ = F32(0x00000000); let a_ = (((c.m.r32(sym::gEftBubble)).wrapping_add(32)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); v_ }; let a_ = (((c.m.r32(sym::gEftBubble)).wrapping_add(32)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); v_ }; let a_ = (((c.m.r32(sym::gEftBubble)).wrapping_add(32)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            { let a0_ = { let a_ = (c.m.r32(sym::gEftBubble)).wrapping_add(8); c.m.r32(a_) }; F::EftBubble_DrawList(c, a0_) };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBubble_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn EftBubble_EmitBody(c: &mut Ctx, mut p_objId_1: i32, mut p_chance_2: i32, mut p_view_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBubble_EmitBody", &PROBE_N, &[(p_objId_1) as u32, (p_chance_2) as u32, (p_view_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(256) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 256);
    let mut l_spacing_4: F32 = F32(0);
    let mut l_len_5: F32 = F32(0);
    let mut l_size_6: F32 = F32(0);
    let mut l_amp_7: F32 = F32(0);
    let mut l_rise_8: F32 = F32(0);
    let mut l_life_9: F32 = F32(0);
    let mut l_i_10: i32 = 0;
    let mut l_j_11: i32 = 0;
    let mut l_n_12: i32 = 0;
    let r_ = 'f: {
        rt::memset(c, fp, 0, 160);
        rt::memset(c, fp.wrapping_add(0), 0, 8);
        { let v_ = 11i32; c.m.w32(fp.wrapping_add(0).wrapping_add(0), (v_) as u32); }
        { let v_ = 10i32; c.m.w32(fp.wrapping_add(0).wrapping_add(4), (v_) as u32); }
        rt::memset(c, fp.wrapping_add(8), 0, 8);
        { let v_ = 10i32; c.m.w32(fp.wrapping_add(8).wrapping_add(0), (v_) as u32); }
        { let v_ = 9i32; c.m.w32(fp.wrapping_add(8).wrapping_add(4), (v_) as u32); }
        rt::memset(c, fp.wrapping_add(16), 0, 8);
        { let v_ = 9i32; c.m.w32(fp.wrapping_add(16).wrapping_add(0), (v_) as u32); }
        { let v_ = 8i32; c.m.w32(fp.wrapping_add(16).wrapping_add(4), (v_) as u32); }
        rt::memset(c, fp.wrapping_add(24), 0, 8);
        { let v_ = 8i32; c.m.w32(fp.wrapping_add(24).wrapping_add(0), (v_) as u32); }
        { let v_ = (1i32).wrapping_neg(); c.m.w32(fp.wrapping_add(24).wrapping_add(4), (v_) as u32); }
        rt::memset(c, fp.wrapping_add(32), 0, 8);
        { let v_ = 15i32; c.m.w32(fp.wrapping_add(32).wrapping_add(0), (v_) as u32); }
        { let v_ = 14i32; c.m.w32(fp.wrapping_add(32).wrapping_add(4), (v_) as u32); }
        rt::memset(c, fp.wrapping_add(40), 0, 8);
        { let v_ = 14i32; c.m.w32(fp.wrapping_add(40).wrapping_add(0), (v_) as u32); }
        { let v_ = 13i32; c.m.w32(fp.wrapping_add(40).wrapping_add(4), (v_) as u32); }
        rt::memset(c, fp.wrapping_add(48), 0, 8);
        { let v_ = 13i32; c.m.w32(fp.wrapping_add(48).wrapping_add(0), (v_) as u32); }
        { let v_ = 12i32; c.m.w32(fp.wrapping_add(48).wrapping_add(4), (v_) as u32); }
        rt::memset(c, fp.wrapping_add(56), 0, 8);
        { let v_ = 12i32; c.m.w32(fp.wrapping_add(56).wrapping_add(0), (v_) as u32); }
        { let v_ = (1i32).wrapping_neg(); c.m.w32(fp.wrapping_add(56).wrapping_add(4), (v_) as u32); }
        rt::memset(c, fp.wrapping_add(64), 0, 8);
        { let v_ = 18i32; c.m.w32(fp.wrapping_add(64).wrapping_add(0), (v_) as u32); }
        { let v_ = 19i32; c.m.w32(fp.wrapping_add(64).wrapping_add(4), (v_) as u32); }
        rt::memset(c, fp.wrapping_add(72), 0, 8);
        { let v_ = 19i32; c.m.w32(fp.wrapping_add(72).wrapping_add(0), (v_) as u32); }
        { let v_ = 20i32; c.m.w32(fp.wrapping_add(72).wrapping_add(4), (v_) as u32); }
        rt::memset(c, fp.wrapping_add(80), 0, 8);
        { let v_ = 20i32; c.m.w32(fp.wrapping_add(80).wrapping_add(0), (v_) as u32); }
        { let v_ = 21i32; c.m.w32(fp.wrapping_add(80).wrapping_add(4), (v_) as u32); }
        rt::memset(c, fp.wrapping_add(88), 0, 8);
        { let v_ = 21i32; c.m.w32(fp.wrapping_add(88).wrapping_add(0), (v_) as u32); }
        { let v_ = (1i32).wrapping_neg(); c.m.w32(fp.wrapping_add(88).wrapping_add(4), (v_) as u32); }
        rt::memset(c, fp.wrapping_add(96), 0, 8);
        { let v_ = 32i32; c.m.w32(fp.wrapping_add(96).wrapping_add(0), (v_) as u32); }
        { let v_ = 33i32; c.m.w32(fp.wrapping_add(96).wrapping_add(4), (v_) as u32); }
        rt::memset(c, fp.wrapping_add(104), 0, 8);
        { let v_ = 33i32; c.m.w32(fp.wrapping_add(104).wrapping_add(0), (v_) as u32); }
        { let v_ = 34i32; c.m.w32(fp.wrapping_add(104).wrapping_add(4), (v_) as u32); }
        rt::memset(c, fp.wrapping_add(112), 0, 8);
        { let v_ = 34i32; c.m.w32(fp.wrapping_add(112).wrapping_add(0), (v_) as u32); }
        { let v_ = 35i32; c.m.w32(fp.wrapping_add(112).wrapping_add(4), (v_) as u32); }
        rt::memset(c, fp.wrapping_add(120), 0, 8);
        { let v_ = 35i32; c.m.w32(fp.wrapping_add(120).wrapping_add(0), (v_) as u32); }
        { let v_ = (1i32).wrapping_neg(); c.m.w32(fp.wrapping_add(120).wrapping_add(4), (v_) as u32); }
        rt::memset(c, fp.wrapping_add(128), 0, 8);
        { let v_ = 3i32; c.m.w32(fp.wrapping_add(128).wrapping_add(0), (v_) as u32); }
        { let v_ = 16i32; c.m.w32(fp.wrapping_add(128).wrapping_add(4), (v_) as u32); }
        rt::memset(c, fp.wrapping_add(136), 0, 8);
        { let v_ = 16i32; c.m.w32(fp.wrapping_add(136).wrapping_add(0), (v_) as u32); }
        { let v_ = 17i32; c.m.w32(fp.wrapping_add(136).wrapping_add(4), (v_) as u32); }
        rt::memset(c, fp.wrapping_add(144), 0, 8);
        { let v_ = 17i32; c.m.w32(fp.wrapping_add(144).wrapping_add(0), (v_) as u32); }
        { let v_ = 48i32; c.m.w32(fp.wrapping_add(144).wrapping_add(4), (v_) as u32); }
        rt::memset(c, fp.wrapping_add(152), 0, 8);
        { let v_ = 48i32; c.m.w32(fp.wrapping_add(152).wrapping_add(0), (v_) as u32); }
        { let v_ = (1i32).wrapping_neg(); c.m.w32(fp.wrapping_add(152).wrapping_add(4), (v_) as u32); }
        F::BtlCharApi_GetDir(c, p_objId_1, fp.wrapping_add(176));
        l_spacing_4 = rt::fmul(F::BtlCharApi_GetHeight(c, p_objId_1), F32(0x3e800000));
        if (((rt::flt(l_spacing_4, F32(0x40c00000)) as i32)) != 0) {
            l_spacing_4 = F32(0x40c00000);
        }
        {
            l_i_10 = 0i32;
            'l1: loop {
                if !(((((l_i_10) < (20i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let a1_ = (c.m.r32((fp).wrapping_add((((l_i_10) as u32)).wrapping_mul(8))) as i32); F::BtlCharApi_GetNodePos(c, p_objId_1, a1_, fp.wrapping_add(160)) };
                    F::Vec4_Copy(c, fp.wrapping_add(192), fp.wrapping_add(160));
                    if ((((((c.m.r32(((fp).wrapping_add((((l_i_10) as u32)).wrapping_mul(8))).wrapping_add(4)) as i32)) >= (0i32)) as i32)) != 0) {
                        { let a1_ = (c.m.r32(((fp).wrapping_add((((l_i_10) as u32)).wrapping_mul(8))).wrapping_add(4)) as i32); F::BtlCharApi_GetNodePos(c, p_objId_1, a1_, fp.wrapping_add(224)) };
                        F::Vec4_Sub(c, fp.wrapping_add(224), fp.wrapping_add(224), fp.wrapping_add(160));
                        l_len_5 = F::Vec3_Length(c, fp.wrapping_add(224));
                        if (((rt::flt(l_spacing_4, l_len_5) as i32)) != 0) {
                            l_n_12 = rt::f2i(rt::fadd(rt::fdiv(l_len_5, l_spacing_4), F32(0x3f800000)));
                            F::Vec3_Scale(c, fp.wrapping_add(208), fp.wrapping_add(224), rt::fdiv(F32(0x3f800000), rt::i2f(l_n_12)));
                        } else {
                            F::Vec4_Set(c, fp.wrapping_add(208), F32(0x00000000), F32(0x00000000), F32(0x00000000), F32(0x3f800000));
                            l_n_12 = 1i32;
                        }
                    } else {
                        F::Vec4_Set(c, fp.wrapping_add(208), F32(0x00000000), F32(0x00000000), F32(0x00000000), F32(0x3f800000));
                        l_n_12 = 1i32;
                    }
                    {
                        l_j_11 = 0i32;
                        'l3: loop {
                            if !(((((l_j_11) < (l_n_12)) as i32)) != 0) { break 'l3; }
                            'c4: {
                                if (((((rt::rem_i32(F::Port_Rand(c), p_chance_2)) == (0i32)) as i32)) != 0) {
                                    l_size_6 = rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x3ecccccc)), F32(0x3ecccccc));
                                    l_amp_7 = rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x3e4ccccc)), F32(0x3dcccccc));
                                    l_rise_8 = rt::fsub(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0xbd4ccccc)), F32(0x3d4ccccc));
                                    l_life_9 = rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x3ecccccc)), F32(0x3f199999));
                                    { let r_ = rt::fmul(F::BtlCharApi_GetHeight(c, p_objId_1), F32(0x3d4ccccc)); let n_ = rt::fmul(l_size_6, r_); l_size_6 = n_; };
                                    { let r_ = rt::fmul(F::BtlCharApi_GetHeight(c, p_objId_1), F32(0x3d4ccccc)); let n_ = rt::fmul(l_amp_7, r_); l_amp_7 = n_; };
                                    { let r_ = rt::fmul(F::BtlCharApi_GetHeight(c, p_objId_1), F32(0x3d4ccccc)); let n_ = rt::fmul(l_rise_8, r_); l_rise_8 = n_; };
                                    { let a2_ = rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x3e999999)), F32(0x3f333333)); F::Vec4_Scale(c, fp.wrapping_add(240), fp.wrapping_add(176), a2_) };
                                    { let v_ = rt::fadd(F32(c.m.r32(fp.wrapping_add(192))), rt::fmul(rt::i2f(l_j_11), F32(c.m.r32(fp.wrapping_add(208))))); let a_ = fp.wrapping_add(160); c.m.w32(a_, (v_).0); };
                                    { let v_ = rt::fadd(F32(c.m.r32((fp.wrapping_add(192)).wrapping_add(4))), rt::fmul(rt::i2f(l_j_11), F32(c.m.r32((fp.wrapping_add(208)).wrapping_add(4))))); let a_ = (fp.wrapping_add(160)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                                    { let v_ = rt::fadd(F32(c.m.r32((fp.wrapping_add(192)).wrapping_add(8))), rt::fmul(rt::i2f(l_j_11), F32(c.m.r32((fp.wrapping_add(208)).wrapping_add(8))))); let a_ = (fp.wrapping_add(160)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                                    { let a5_ = rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x43b40000)); let a8_ = rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x40490fda)); F::EftBubble_Add(c, fp.wrapping_add(160), fp.wrapping_add(240), F32(0xc1000000), l_size_6, F32(0x43000000), a5_, l_rise_8, F32(0x40c00000), a8_, l_amp_7, ((128i32) as u8), ((128i32) as u8), ((128i32) as u8), p_objId_1, p_view_3, ((5i32) as u8), F32(0x3e800000), l_life_9, ((0i32) as u8)) };
                                }
                            }
                            { let o_ = l_j_11; let n_ = (o_).wrapping_add(1); l_j_11 = n_; };
                        }
                    }
                }
                { let o_ = l_i_10; let n_ = (o_).wrapping_add(1); l_i_10 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftBubble_EmitBody", &PROBE_N, &[], true); }
    r_
}

pub fn EftBubble_EmitBurst(c: &mut Ctx, mut p_objId_1: i32, mut p_count_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBubble_EmitBurst", &PROBE_N, &[(p_objId_1) as u32, (p_count_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let mut l_size_3: F32 = F32(0);
    let mut l_rise_4: F32 = F32(0);
    let mut l_amp_5: F32 = F32(0);
    let mut l_dist_6: F32 = F32(0);
    let mut l_rot_7: F32 = F32(0);
    let mut l_phase_8: F32 = F32(0);
    let mut l_i_9: i32 = 0;
    let r_ = 'f: {
        F::BtlCharApi_GetPos(c, p_objId_1, fp);
        F::BtlCharApi_GetDir(c, p_objId_1, fp.wrapping_add(16));
        {
            l_i_9 = 0i32;
            'l1: loop {
                if !(((((l_i_9) < (p_count_2)) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_size_3 = rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x40900000)), F32(0x40400000));
                    l_rise_4 = rt::fsub(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0xbe4ccccc)), F32(0x3e4ccccc));
                    l_amp_5 = rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x3f000000)), F32(0x3e999999));
                    l_dist_6 = rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x41200000));
                    { let r_ = rt::fmul(F::BtlCharApi_GetHeight(c, p_objId_1), F32(0x3d4ccccc)); let n_ = rt::fmul(l_size_3, r_); l_size_3 = n_; };
                    { let r_ = rt::fmul(F::BtlCharApi_GetHeight(c, p_objId_1), F32(0x3d4ccccc)); let n_ = rt::fmul(l_rise_4, r_); l_rise_4 = n_; };
                    { let r_ = rt::fmul(F::BtlCharApi_GetHeight(c, p_objId_1), F32(0x3d4ccccc)); let n_ = rt::fmul(l_amp_5, r_); l_amp_5 = n_; };
                    { let r_ = rt::fmul(F::BtlCharApi_GetHeight(c, p_objId_1), F32(0x3d4ccccc)); let n_ = rt::fmul(l_dist_6, r_); l_dist_6 = n_; };
                    F::EftBubble_RandDir(c, fp.wrapping_add(32));
                    F::Vec4_Scale(c, fp.wrapping_add(32), fp.wrapping_add(32), l_dist_6);
                    F::Vec4_Add(c, fp.wrapping_add(32), fp.wrapping_add(32), fp);
                    { let a2_ = rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x3e999999)), F32(0x3f333333)); F::Vec4_Scale(c, fp.wrapping_add(16), fp.wrapping_add(16), a2_) };
                    l_rot_7 = rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x43b40000));
                    l_phase_8 = rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x40490fda));
                    { let a15_ = (((if ((((F::Port_Rand(c)) & (3i32))) != 0) { 7i32 } else { 8i32 })) as u8); F::EftBubble_Add(c, fp.wrapping_add(32), fp.wrapping_add(16), F32(0xc1000000), l_size_3, F32(0x42e00000), l_rot_7, l_rise_4, F32(0x40c00000), l_phase_8, l_amp_5, ((128i32) as u8), ((128i32) as u8), ((128i32) as u8), p_objId_1, (1i32).wrapping_neg(), a15_, F32(0x3f000000), F32(0xbf800000), ((0i32) as u8)) };
                }
                { let o_ = l_i_9; let n_ = (o_).wrapping_add(1); l_i_9 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftBubble_EmitBurst", &PROBE_N, &[], true); }
    r_
}

pub fn EftBubble_EmitCamera(c: &mut Ctx, mut p_view_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBubble_EmitCamera", &PROBE_N, &[(p_view_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(96) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 96);
    let mut l_v_2: u32 = 0;
    let mut l_x_3: F32 = F32(0);
    let mut l_y_4: F32 = F32(0);
    let mut l_size_5: F32 = F32(0);
    let mut l_tex_6: i32 = 0;
    let r_ = 'f: {
        l_v_2 = ((c.m.r32(sym::gBtlCam)).wrapping_add(1824)).wrapping_add((((p_view_1) as u32)).wrapping_mul(656));
        F::Mtx_InverseRT(c, fp.wrapping_add(32), (l_v_2).wrapping_add(64));
        F::Vec4_Scale(c, fp, (fp.wrapping_add(32)).wrapping_add((((2i32) as u32)).wrapping_mul(16)), F32(0x41200000));
        F::Vec4_Add(c, fp, fp, (l_v_2).wrapping_add(608));
        { let r_ = F32(0x41a00000); let a_ = (fp).wrapping_add(4); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        l_x_3 = rt::fmul(rt::i2f((rt::rem_i32(F::Port_Rand(c), 2001i32)).wrapping_sub(1000i32)), F32(0x3c75c28f));
        l_y_4 = rt::fmul(rt::i2f((rt::rem_i32(F::Port_Rand(c), 2001i32)).wrapping_sub(1000i32)), F32(0x3c75c28f));
        F::Vec4_Scale(c, fp.wrapping_add(16), (fp.wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), l_x_3);
        F::Vec4_Add(c, fp, fp, fp.wrapping_add(16));
        F::Vec4_Scale(c, fp.wrapping_add(16), (fp.wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), l_y_4);
        F::Vec4_Add(c, fp, fp, fp.wrapping_add(16));
        if (((((rt::rem_i32(F::Port_Rand(c), 6i32)) != (0i32)) as i32)) != 0) {
            l_tex_6 = 5i32;
            l_size_5 = rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x3e199999)), F32(0x3e199999));
        } else {
            l_tex_6 = 6i32;
            l_size_5 = rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x3fc00000)), F32(0x3fc00000));
        }
        { let a8_ = rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x40490fda)); F::EftBubble_Add(c, fp, sym::gEftBubbleZeroVel, F32(0x00000000), l_size_5, F32(0x43000000), F32(0x00000000), F32(0xbe999999), F32(0x00000000), a8_, F32(0x3dcccccc), ((128i32) as u8), ((128i32) as u8), ((128i32) as u8), (1i32).wrapping_neg(), p_view_1, ((l_tex_6) as u8), F32(0x3e800000), F32(0xbf800000), ((0i32) as u8)) };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftBubble_EmitCamera", &PROBE_N, &[], true); }
    r_
}

pub fn EftBubble_UpdateChrs(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBubble_UpdateChrs", &PROBE_N, &[], false); }
    let mut l_chr_1: u32 = 0;
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
    if (((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0) {
        break 'f;
    }
    l_chr_1 = { let a_ = c.m.r32(sym::gEftBubble); c.m.r32(a_) };
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < (2i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if (((rt::bf_get_u(c, (l_chr_1).wrapping_add(12), 0, 1) as u8)) != 0) {
                    F::EftBubble_UpdateTrail(c, l_chr_1);
                }
                if (((rt::bf_get_u(c, (l_chr_1).wrapping_add(12), 1, 1) as u8)) != 0) {
                    F::EftBubble_UpdateBurst(c, l_chr_1);
                }
            }
            { { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; }; { let o_ = l_chr_1; let n_ = (o_).wrapping_add(16); l_chr_1 = n_; }; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBubble_UpdateChrs", &PROBE_N, &[], true); }
    r_
}

pub fn EftBubble_UpdateTrail(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBubble_UpdateTrail", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    { let a0_ = (c.m.r32(p_chr_1) as i32); let a2_ = (c.m.r32(p_chr_1) as i32); F::EftBubble_EmitBody(c, a0_, 20i32, a2_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBubble_UpdateTrail", &PROBE_N, &[], true); }
    r_
}

pub fn EftBubble_UpdateBurst(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBubble_UpdateBurst", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    if (((rt::fgt(F32(c.m.r32((p_chr_1).wrapping_add(8))), F32(0x00000000)) as i32)) != 0) {
        { let a0_ = (c.m.r32(p_chr_1) as i32); let a1_ = rt::f2i(F32(c.m.r32((p_chr_1).wrapping_add(8)))); F::EftBubble_EmitBurst(c, a0_, a1_) };
        { let r_ = F32(0x41000000); let a_ = (p_chr_1).wrapping_add(8); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
    } else {
        { let v_ = F32(0x00000000); let a_ = (p_chr_1).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = ((0i32) as u8); let a_ = (p_chr_1).wrapping_add(12); rt::bf_set(c, a_, 1, 1, (v_) as u64);  };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBubble_UpdateBurst", &PROBE_N, &[], true); }
    r_
}

pub fn EftBubble_UpdateAmbient(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBubble_UpdateAmbient", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_chr_1: u32 = 0;
    let mut l_views_2: i32 = 0;
    let mut l_i_3: i32 = 0;
    let r_ = 'f: {
        { let v_ = F32(0x00000000); c.m.w32(fp, (v_).0); }
        if (((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0) {
            break 'f;
        }
        if ((F::Battle_IsSplitScreen(c)) != 0) {
            break 'f;
        }
        { let _ = F::EftWater_GetSurfaceY(c, fp); };
        l_views_2 = (if ((F::Battle_IsSplitScreen(c)) != 0) { 2i32 } else { 1i32 });
        l_chr_1 = { let a_ = c.m.r32(sym::gEftBubble); c.m.r32(a_) };
        {
            l_i_3 = 0i32;
            'l1: loop {
                if !(((((l_i_3) < (l_views_2)) as i32)) != 0) { break 'l1; }
                'c2: {
                    if ((((!(((rt::flt({ let a_ = ((((c.m.r32(sym::gBtlCam)).wrapping_add((((l_i_3) as u32)).wrapping_mul(656u32))).wrapping_add(1824)).wrapping_add(608)).wrapping_add(4); F32(c.m.r32(a_)) }, F32(c.m.r32(fp))) as i32)) != 0)) as i32)) != 0) {
                        { let r_ = (if ((((F::Port_Rand(c)) & (3i32))) != 0) { F32(0x3d4ccccc) } else { F32(0x3dcccccc) }); let a_ = ((c.m.r32(sym::gEftBubble)).wrapping_add(96)).wrapping_add((((l_i_3) as u32)).wrapping_mul(4)); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                        if (((rt::fgt({ let a_ = ((c.m.r32(sym::gEftBubble)).wrapping_add(96)).wrapping_add((((l_i_3) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }, F32(0x3f800000)) as i32)) != 0) {
                            { let r_ = F32(0x3f800000); let a_ = ((c.m.r32(sym::gEftBubble)).wrapping_add(96)).wrapping_add((((l_i_3) as u32)).wrapping_mul(4)); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                            F::EftBubble_EmitCamera(c, l_i_3);
                        }
                        if ((((!(((rt::bf_get_u(c, ((l_chr_1).wrapping_add((((l_i_3) as u32)).wrapping_mul(16))).wrapping_add(12), 0, 1) as u8)) != 0)) as i32)) != 0) {
                            if (({ let a0_ = (c.m.r32((l_chr_1).wrapping_add((((l_i_3) as u32)).wrapping_mul(16))) as i32); F::BtlCharApi_IsInWater(c, a0_) }) != 0) {
                                { let a0_ = (c.m.r32((l_chr_1).wrapping_add((((l_i_3) as u32)).wrapping_mul(16))) as i32); F::EftBubble_EmitBody(c, a0_, 120i32, l_i_3) };
                            }
                        }
                    }
                }
                { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftBubble_UpdateAmbient", &PROBE_N, &[], true); }
    r_
}

pub fn EftBubble_AllocSlot(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBubble_AllocSlot", &PROBE_N, &[], false); }
    let mut l_b_1: u32 = 0;
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
    l_b_1 = { let a_ = (c.m.r32(sym::gEftBubble)).wrapping_add(4); c.m.r32(a_) };
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < (100i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((((!(((c.m.r32((l_b_1).wrapping_add(88)) as i32)) != 0)) as i32)) != 0) {
                    { let _ = F::memset(c, l_b_1, 0i32, 112u32); };
                    break 'f l_b_1;
                }
            }
            { { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; }; { let o_ = l_b_1; let n_ = (o_).wrapping_add(112); l_b_1 = n_; }; };
        }
    }
    break 'f ((0i32) as u32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBubble_AllocSlot", &PROBE_N, &[], true); }
    r_
}

pub fn EftBubble_Add(c: &mut Ctx, p_pos_1: u32, p_vel_2: u32, mut p_drag_3: F32, mut p_size_4: F32, mut p_alpha_5: F32, mut p_rot_6: F32, mut p_rise_7: F32, mut p_riseGrow_8: F32, mut p_phase_9: F32, mut p_amp_10: F32, mut p_r_11: u8, mut p_g_12: u8, mut p_b_13: u8, mut p_objId_14: i32, mut p_view_15: i32, mut p_tex_16: u8, mut p_phaseStep_17: F32, mut p_life_18: F32, mut p_layer_19: u8) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBubble_Add", &PROBE_N, &[(p_pos_1) as u32, (p_vel_2) as u32, (p_drag_3).0, (p_size_4).0, (p_alpha_5).0, (p_rot_6).0, (p_rise_7).0, (p_riseGrow_8).0, (p_phase_9).0, (p_amp_10).0, (p_r_11) as u32, (p_g_12) as u32, (p_b_13) as u32, (p_objId_14) as u32, (p_view_15) as u32, (p_tex_16) as u32, (p_phaseStep_17).0, (p_life_18).0, (p_layer_19) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_bub_20: u32 = 0;
    rt::memcpy(c, fp, p_pos_1, 16);
    rt::memcpy(c, fp.wrapping_add(16), p_vel_2, 16);
    let r_ = 'f: {
        l_bub_20 = F::EftBubble_AllocSlot(c);
        if (((((l_bub_20) != (((0i32) as u32))) as i32)) != 0) {
            if ((((({ let a_ = (c.m.r32(sym::gEftBubble)).wrapping_add(8); c.m.r32(a_) }) == (((0i32) as u32))) as i32)) != 0) {
                { let v_ = l_bub_20; let a_ = (c.m.r32(sym::gEftBubble)).wrapping_add(8); c.m.w32(a_, v_); };
                { let v_ = ((0i32) as u32); let a_ = (l_bub_20).wrapping_add(92); c.m.w32(a_, v_); };
                { let v_ = ((0i32) as u32); let a_ = (l_bub_20).wrapping_add(96); c.m.w32(a_, v_); };
            } else {
                { let v_ = { let a_ = (c.m.r32(sym::gEftBubble)).wrapping_add(12); c.m.r32(a_) }; let a_ = (l_bub_20).wrapping_add(92); c.m.w32(a_, v_); };
                { let v_ = ((0i32) as u32); let a_ = (l_bub_20).wrapping_add(96); c.m.w32(a_, v_); };
            }
            if ((((({ let a_ = (c.m.r32(sym::gEftBubble)).wrapping_add(12); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
                { let v_ = l_bub_20; let a_ = ({ let a_ = (c.m.r32(sym::gEftBubble)).wrapping_add(12); c.m.r32(a_) }).wrapping_add(96); c.m.w32(a_, v_); };
            }
            { let v_ = l_bub_20; let a_ = (c.m.r32(sym::gEftBubble)).wrapping_add(12); c.m.w32(a_, v_); };
            F::Vec4_Copy(c, l_bub_20, fp);
            F::Vec4_Copy(c, (l_bub_20).wrapping_add(16), fp.wrapping_add(16));
            if (((rt::feq(p_drag_3, F32(0x00000000)) as i32)) != 0) {
                { let v_ = F32(0x00000000); let a_ = (l_bub_20).wrapping_add(32); c.m.w32(a_, (v_).0); };
            } else {
                { let v_ = rt::fdiv(F32(0x3f800000), p_drag_3); let a_ = (l_bub_20).wrapping_add(32); c.m.w32(a_, (v_).0); };
            }
            { let v_ = p_rot_6; let a_ = (l_bub_20).wrapping_add(40); c.m.w32(a_, (v_).0); };
            { let v_ = p_size_4; let a_ = (l_bub_20).wrapping_add(36); c.m.w32(a_, (v_).0); };
            { let v_ = p_alpha_5; let a_ = (l_bub_20).wrapping_add(44); c.m.w32(a_, (v_).0); };
            if (((rt::fle(p_life_18, F32(0x00000000)) as i32)) != 0) {
                { let v_ = F32(0x00000000); let a_ = (l_bub_20).wrapping_add(48); c.m.w32(a_, (v_).0); };
            } else {
                { let v_ = rt::fdiv(rt::fneg(p_alpha_5), rt::fmul(p_life_18, F32(0x41f00000))); let a_ = (l_bub_20).wrapping_add(48); c.m.w32(a_, (v_).0); };
            }
            { let v_ = p_rise_7; let a_ = (l_bub_20).wrapping_add(52); c.m.w32(a_, (v_).0); };
            if (((rt::feq(p_riseGrow_8, F32(0x00000000)) as i32)) != 0) {
                { let v_ = F32(0x00000000); let a_ = (l_bub_20).wrapping_add(56); c.m.w32(a_, (v_).0); };
            } else {
                { let v_ = rt::fdiv(F32(0x3f800000), p_riseGrow_8); let a_ = (l_bub_20).wrapping_add(56); c.m.w32(a_, (v_).0); };
            }
            { let v_ = p_phase_9; let a_ = (l_bub_20).wrapping_add(60); c.m.w32(a_, (v_).0); };
            { let v_ = p_amp_10; let a_ = (l_bub_20).wrapping_add(64); c.m.w32(a_, (v_).0); };
            { let v_ = p_phaseStep_17; let a_ = (l_bub_20).wrapping_add(68); c.m.w32(a_, (v_).0); };
            { let v_ = p_r_11; let a_ = (l_bub_20).wrapping_add(80); c.m.w8(a_, v_); };
            { let v_ = p_g_12; let a_ = (l_bub_20).wrapping_add(81); c.m.w8(a_, v_); };
            { let v_ = p_b_13; let a_ = (l_bub_20).wrapping_add(82); c.m.w8(a_, v_); };
            { let v_ = p_objId_14; let a_ = (l_bub_20).wrapping_add(72); c.m.w32(a_, (v_) as u32); };
            { let v_ = p_view_15; let a_ = (l_bub_20).wrapping_add(76); c.m.w32(a_, (v_) as u32); };
            { let v_ = p_tex_16; let a_ = (l_bub_20).wrapping_add(83); c.m.w8(a_, v_); };
            { let v_ = p_layer_19; let a_ = (l_bub_20).wrapping_add(84); c.m.w8(a_, v_); };
            { let v_ = 1i32; let a_ = (l_bub_20).wrapping_add(88); c.m.w32(a_, (v_) as u32); };
            { let a_ = (c.m.r32(sym::gEftBubble)).wrapping_add(16); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftBubble_Add", &PROBE_N, &[], true); }
    r_
}

pub fn EftBubble_UpdateList(c: &mut Ctx, mut p_head_1: u32, mut p_tail_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBubble_UpdateList", &PROBE_N, &[(p_head_1) as u32, (p_tail_2) as u32], false); }
    let mut l_b_3: u32 = 0;
    let mut l_cur_4: u32 = 0;
    let mut l_dead_5: i32 = 0;
    let r_ = 'f: {
    if ((((({ let a_ = (c.m.r32(sym::gEftBubble)).wrapping_add(16); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
        break 'f;
    }
    if (((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0) {
        break 'f;
    }
    l_b_3 = c.m.r32(p_head_1);
    'l1: while (((((l_b_3) != (((0i32) as u32))) as i32)) != 0) {
        l_cur_4 = l_b_3;
        l_dead_5 = F::EftBubble_Step(c, l_b_3);
        l_b_3 = c.m.r32((l_b_3).wrapping_add(96));
        if ((l_dead_5) != 0) {
            { let v_ = 0i32; let a_ = (l_cur_4).wrapping_add(88); c.m.w32(a_, (v_) as u32); };
            if (((((c.m.r32((l_cur_4).wrapping_add(92))) == (((0i32) as u32))) as i32)) != 0) {
                { let v_ = c.m.r32((l_cur_4).wrapping_add(96)); let a_ = p_head_1; c.m.w32(a_, v_); };
            } else {
                { let v_ = c.m.r32((l_cur_4).wrapping_add(96)); let a_ = (c.m.r32((l_cur_4).wrapping_add(92))).wrapping_add(96); c.m.w32(a_, v_); };
            }
            if (((((c.m.r32((l_cur_4).wrapping_add(96))) == (((0i32) as u32))) as i32)) != 0) {
                { let v_ = c.m.r32((l_cur_4).wrapping_add(92)); let a_ = p_tail_2; c.m.w32(a_, v_); };
            } else {
                { let v_ = c.m.r32((l_cur_4).wrapping_add(92)); let a_ = (c.m.r32((l_cur_4).wrapping_add(96))).wrapping_add(92); c.m.w32(a_, v_); };
            }
            { let v_ = ((0i32) as u32); let a_ = (l_cur_4).wrapping_add(92); c.m.w32(a_, v_); };
            { let v_ = ((0i32) as u32); let a_ = (l_cur_4).wrapping_add(96); c.m.w32(a_, v_); };
            { let a_ = (c.m.r32(sym::gEftBubble)).wrapping_add(16); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBubble_UpdateList", &PROBE_N, &[], true); }
    r_
}

pub fn EftBubble_Step(c: &mut Ctx, mut p_b_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBubble_Step", &PROBE_N, &[(p_b_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        F::Vec4_Add(c, p_b_1, p_b_1, (p_b_1).wrapping_add(16));
        { let r_ = rt::fmul(F32(c.m.r32((p_b_1).wrapping_add(16))), F32(c.m.r32((p_b_1).wrapping_add(32)))); let a_ = (p_b_1).wrapping_add(16); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        { let r_ = F32(c.m.r32((p_b_1).wrapping_add(52))); let a_ = (p_b_1).wrapping_add(4); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        { let r_ = rt::fmul(F32(c.m.r32(((p_b_1).wrapping_add(16)).wrapping_add(4))), F32(c.m.r32((p_b_1).wrapping_add(32)))); let a_ = ((p_b_1).wrapping_add(16)).wrapping_add(4); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        { let r_ = rt::fmul(F32(c.m.r32((p_b_1).wrapping_add(52))), F32(c.m.r32((p_b_1).wrapping_add(56)))); let a_ = (p_b_1).wrapping_add(52); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        { let r_ = rt::fmul(F32(c.m.r32(((p_b_1).wrapping_add(16)).wrapping_add(8))), F32(c.m.r32((p_b_1).wrapping_add(32)))); let a_ = ((p_b_1).wrapping_add(16)).wrapping_add(8); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        { let r_ = rt::fmul({ let a0_ = F32(c.m.r32((p_b_1).wrapping_add(60))); F::Mathf_CosFast(c, a0_) }, F32(c.m.r32((p_b_1).wrapping_add(64)))); let a_ = p_b_1; let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        { let r_ = rt::fmul({ let a0_ = F32(c.m.r32((p_b_1).wrapping_add(60))); F::Mathf_SinFast(c, a0_) }, F32(c.m.r32((p_b_1).wrapping_add(64)))); let a_ = (p_b_1).wrapping_add(8); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        { let r_ = F32(c.m.r32((p_b_1).wrapping_add(68))); let a_ = (p_b_1).wrapping_add(60); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        if (((rt::fgt(F32(c.m.r32((p_b_1).wrapping_add(60))), F32(0x40490fda)) as i32)) != 0) {
            { let r_ = F32(0x40c90fda); let a_ = (p_b_1).wrapping_add(60); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        }
        { let r_ = F32(c.m.r32((p_b_1).wrapping_add(48))); let a_ = (p_b_1).wrapping_add(44); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        if (((rt::fle(F32(c.m.r32((p_b_1).wrapping_add(44))), F32(0x00000000)) as i32)) != 0) {
            { let v_ = F32(0x00000000); let a_ = (p_b_1).wrapping_add(44); c.m.w32(a_, (v_).0); };
            break 'f 1i32;
        }
        if ((((!((F::EftWater_GetSurfaceY(c, fp)) != 0)) as i32)) != 0) {
            { let v_ = F32(0x40a00000); let a_ = fp; c.m.w32(a_, (v_).0); };
        }
        if (((rt::flt(rt::fsub(F32(c.m.r32((p_b_1).wrapping_add(4))), rt::fmul(F32(c.m.r32((p_b_1).wrapping_add(36))), F32(0x3f000000))), F32(c.m.r32(fp))) as i32)) != 0) {
            break 'f 1i32;
        }
        if ((((((c.m.r32((p_b_1).wrapping_add(76)) as i32)) != ((1i32).wrapping_neg())) as i32)) != 0) {
            if (((rt::flt({ let a_ = ((((c.m.r32(sym::gBtlCam)).wrapping_add(1824)).wrapping_add(((((c.m.r32((p_b_1).wrapping_add(76)) as i32)) as u32)).wrapping_mul(656))).wrapping_add(608)).wrapping_add(4); F32(c.m.r32(a_)) }, F32(c.m.r32(fp))) as i32)) != 0) {
                break 'f 1i32;
            }
        }
        break 'f 0i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftBubble_Step", &PROBE_N, &[], true); }
    r_
}

pub fn EftBubble_DrawList(c: &mut Ctx, mut p_b_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBubble_DrawList", &PROBE_N, &[(p_b_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(80) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 80);
    let r_ = 'f: {
        { let a1_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(320); F::Mtx_Copy(c, fp, a1_) };
        {
            'l1: loop {
                if !(((((p_b_1) != (((0i32) as u32))) as i32)) != 0) { break 'l1; }
                'c2: {
                    if (((c.m.r32((p_b_1).wrapping_add(88)) as i32)) != 0) {
                        F::Vec4_Copy(c, fp.wrapping_add(64), p_b_1);
                        if ((((((((((c.m.r32((p_b_1).wrapping_add(76)) as i32)) == (F::BtlScene_IsSecondView(c))) as i32)) != 0) || ((((((c.m.r32((p_b_1).wrapping_add(76)) as i32)) == ((1i32).wrapping_neg())) as i32)) != 0)) as i32)) != 0) {
                            { let a2_ = F32(c.m.r32((p_b_1).wrapping_add(36))); let a3_ = F32(c.m.r32((p_b_1).wrapping_add(40))); let a4_ = c.m.r8((p_b_1).wrapping_add(80)); let a5_ = c.m.r8((p_b_1).wrapping_add(81)); let a6_ = c.m.r8((p_b_1).wrapping_add(82)); let a7_ = ((rt::f2u(F32(c.m.r32((p_b_1).wrapping_add(44))))) as u8); let a8_ = ((c.m.r32(sym::gEftBubble)).wrapping_add(104)).wrapping_add((((c.m.r8((p_b_1).wrapping_add(83))) as u32)).wrapping_mul(16)); let a9_ = c.m.r8((p_b_1).wrapping_add(84)); F::EftBubble_DrawOne(c, fp.wrapping_add(64), fp, a2_, a3_, a4_, a5_, a6_, a7_, a8_, a9_) };
                        }
                    }
                }
                p_b_1 = c.m.r32((p_b_1).wrapping_add(96));
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftBubble_DrawList", &PROBE_N, &[], true); }
    r_
}

pub fn EftBubble_IsOnScreen(c: &mut Ctx, p_v_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBubble_IsOnScreen", &PROBE_N, &[(p_v_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    rt::memcpy(c, fp, p_v_1, 16);
    let r_ = 'f: {
        if ((((((c.m.r32((fp).wrapping_add(8)) as i32)) <= (0i32)) as i32)) != 0) {
            break 'f 0i32;
        }
        if ((((((c.m.r32(fp) as i32)) > (38911i32)) as i32)) != 0) {
            break 'f 0i32;
        }
        if ((((((c.m.r32(fp) as i32)) <= (26624i32)) as i32)) != 0) {
            break 'f 0i32;
        }
        if ((((((c.m.r32((fp).wrapping_add(4)) as i32)) > (38143i32)) as i32)) != 0) {
            break 'f 0i32;
        }
        if ((((((c.m.r32((fp).wrapping_add(4)) as i32)) <= (27392i32)) as i32)) != 0) {
            break 'f 0i32;
        }
        break 'f 1i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftBubble_IsOnScreen", &PROBE_N, &[], true); }
    r_
}

pub fn EftBubble_RandDir(c: &mut Ctx, p_v_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBubble_RandDir", &PROBE_N, &[(p_v_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(80) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 80);
    let mut l_a_2: F32 = F32(0);
    rt::memcpy(c, fp, p_v_1, 16);
    let r_ = 'f: {
        l_a_2 = rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x40c90fda));
        { let v_ = F::Mathf_SinFast(c, l_a_2); let a_ = fp; c.m.w32(a_, (v_).0); };
        { let v_ = F::Mathf_CosFast(c, l_a_2); let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = { let v_ = F32(0x00000000); let a_ = (fp).wrapping_add(12); c.m.w32(a_, (v_).0); v_ }; let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_).0); };
        l_a_2 = rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x40c90fda));
        F::Mtx_StoreIdentity(c, fp.wrapping_add(16));
        { let a2_ = F::EftMath_WrapAngle(c, l_a_2); F::Mtx_RotateY(c, fp.wrapping_add(16), fp.wrapping_add(16), a2_) };
        F::Mtx_MulVec4(c, fp, fp.wrapping_add(16), fp);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftBubble_RandDir", &PROBE_N, &[], true); }
    r_
}

pub fn EftBubble_DrawOne(c: &mut Ctx, p_pos_1: u32, p_m_2: u32, mut p_size_3: F32, mut p_rot_4: F32, mut p_r_5: u8, mut p_g_6: u8, mut p_b_7: u8, mut p_a_8: u8, mut p_tex_9: u32, mut p_layer_10: u8) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBubble_DrawOne", &PROBE_N, &[(p_pos_1) as u32, (p_m_2) as u32, (p_size_3).0, (p_rot_4).0, (p_r_5) as u32, (p_g_6) as u32, (p_b_7) as u32, (p_a_8) as u32, (p_tex_9) as u32, (p_layer_10) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(336) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 336);
    let mut l_half_11: F32 = F32(0);
    let mut l_i_12: i32 = 0;
    let mut l_z_13: i32 = 0;
    let mut l_l_14: i32 = 0;
    let mut l_p_15: u32 = 0;
    let mut l_e_16: u32 = 0;
    rt::memcpy(c, fp, p_pos_1, 16);
    rt::memcpy(c, fp.wrapping_add(16), p_m_2, 64);
    let r_ = 'f: {
        { let a1_ = (c.m.r32(sym::gEftBubble)).wrapping_add(32); F::Mtx_Copy(c, fp.wrapping_add(80), a1_) };
        if (((rt::fne(p_rot_4, F32(0x00000000)) as i32)) != 0) {
            F::Mtx_StoreIdentity(c, fp.wrapping_add(272));
            { let a2_ = F::EftMath_WrapAngle(c, rt::fdiv(rt::fmul(p_rot_4, F32(0x40490fda)), F32(0x43340000))); F::Mtx_RotateZ(c, fp.wrapping_add(272), fp.wrapping_add(272), a2_) };
            F::Mtx_Mul(c, fp.wrapping_add(80), fp.wrapping_add(80), fp.wrapping_add(272));
        }
        l_half_11 = rt::fmul(p_size_3, F32(0x3f000000));
        F::Vec4_Copy(c, (fp.wrapping_add(80)).wrapping_add((((3i32) as u32)).wrapping_mul(16)), fp);
        F::Vec4_Set(c, (fp.wrapping_add(144)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), rt::fneg(l_half_11), rt::fneg(l_half_11), F32(0x00000000), F32(0x3f800000));
        F::Vec4_Set(c, (fp.wrapping_add(144)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), l_half_11, rt::fneg(l_half_11), F32(0x00000000), F32(0x3f800000));
        F::Vec4_Set(c, (fp.wrapping_add(144)).wrapping_add((((2i32) as u32)).wrapping_mul(16)), rt::fneg(l_half_11), l_half_11, F32(0x00000000), F32(0x3f800000));
        F::Vec4_Set(c, (fp.wrapping_add(144)).wrapping_add((((3i32) as u32)).wrapping_mul(16)), l_half_11, l_half_11, F32(0x00000000), F32(0x3f800000));
        {
            l_i_12 = 0i32;
            'l1: loop {
                if !(((((l_i_12) < (4i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    F::Mtx_MulVec4(c, (fp.wrapping_add(144)).wrapping_add((((l_i_12) as u32)).wrapping_mul(16)), fp.wrapping_add(80), (fp.wrapping_add(144)).wrapping_add((((l_i_12) as u32)).wrapping_mul(16)));
                    { let _ = F::Mtx_ProjectPoint(c, (fp.wrapping_add(208)).wrapping_add((((l_i_12) as u32)).wrapping_mul(16)), fp.wrapping_add(16), (fp.wrapping_add(144)).wrapping_add((((l_i_12) as u32)).wrapping_mul(16))); };
                    if ((((!((F::EftBubble_IsOnScreen(c, (fp.wrapping_add(208)).wrapping_add((((l_i_12) as u32)).wrapping_mul(16)))) != 0)) as i32)) != 0) {
                        break 'f;
                    }
                }
                { let o_ = l_i_12; let n_ = (o_).wrapping_add(1); l_i_12 = n_; };
            }
        }
        l_p_15 = c.m.r32(sym::gOtCur);
        { let v_ = (l_p_15).wrapping_add((((1i32) as u32)).wrapping_mul(128)); let a_ = sym::gOtCur; c.m.w32(a_, v_); };
        if (((((l_p_15) == (((0i32) as u32))) as i32)) != 0) {
            break 'f;
        }
        l_l_14 = ((p_layer_10) as i32);
        if (((((l_l_14) >= (2i32)) as i32)) != 0) {
            { let r_ = 2i32; let n_ = (l_l_14).wrapping_sub(r_); l_l_14 = n_; };
        }
        { let v_ = ((84i32) as u64); let a_ = (l_p_15).wrapping_add(32); c.m.w64(a_, v_); };
        { let v_ = ((536870919i32) as u32); let a_ = l_p_15; c.m.w32(a_, v_); };
        { let v_ = ((268435456i32) as u32); let a_ = (l_p_15).wrapping_add(8); c.m.w32(a_, v_); };
        { let v_ = ((1342177287i32) as u32); let a_ = (l_p_15).wrapping_add(12); c.m.w32(a_, v_); };
        { let v_ = 14123288431433908225u64; let a_ = (l_p_15).wrapping_add(16); c.m.w64(a_, v_); };
        { let v_ = ((268436062347616i64) as u64); let a_ = (l_p_15).wrapping_add(24); c.m.w64(a_, v_); };
        { let v_ = p_r_5; let a_ = (l_p_15).wrapping_add(48); c.m.w8(a_, v_); };
        { let v_ = p_g_6; let a_ = ((l_p_15).wrapping_add(48)).wrapping_add(1); c.m.w8(a_, v_); };
        { let v_ = p_b_7; let a_ = ((l_p_15).wrapping_add(48)).wrapping_add(2); c.m.w8(a_, v_); };
        { let v_ = p_a_8; let a_ = ((l_p_15).wrapping_add(48)).wrapping_add(3); c.m.w8(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = (l_p_15).wrapping_add(4); c.m.w32(a_, v_); };
        { let v_ = F32(0x3f800000); let a_ = ((l_p_15).wrapping_add(48)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (l_p_15).wrapping_add(56); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = ((l_p_15).wrapping_add(56)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = (l_p_15).wrapping_add(72); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = ((l_p_15).wrapping_add(72)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (l_p_15).wrapping_add(88); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = ((l_p_15).wrapping_add(88)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = (l_p_15).wrapping_add(104); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = ((l_p_15).wrapping_add(104)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = (((c.m.r32((fp.wrapping_add(208)).wrapping_add((((0i32) as u32)).wrapping_mul(16))) as i32)) as u64); let a_ = (l_p_15).wrapping_add(64); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = (((c.m.r32(((fp.wrapping_add(208)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4)) as i32)) as u64); let a_ = ((l_p_15).wrapping_add(64)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = (((c.m.r32(((fp.wrapping_add(208)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)) as u64); let a_ = ((l_p_15).wrapping_add(64)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
        { let v_ = ((255i32) as u64); let a_ = ((l_p_15).wrapping_add(64)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
        { let v_ = (((c.m.r32((fp.wrapping_add(208)).wrapping_add((((1i32) as u32)).wrapping_mul(16))) as i32)) as u64); let a_ = (l_p_15).wrapping_add(80); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = (((c.m.r32(((fp.wrapping_add(208)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(4)) as i32)) as u64); let a_ = ((l_p_15).wrapping_add(80)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = (((c.m.r32(((fp.wrapping_add(208)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)) as u64); let a_ = ((l_p_15).wrapping_add(80)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
        { let v_ = ((255i32) as u64); let a_ = ((l_p_15).wrapping_add(80)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
        { let v_ = (((c.m.r32((fp.wrapping_add(208)).wrapping_add((((2i32) as u32)).wrapping_mul(16))) as i32)) as u64); let a_ = (l_p_15).wrapping_add(96); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = (((c.m.r32(((fp.wrapping_add(208)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(4)) as i32)) as u64); let a_ = ((l_p_15).wrapping_add(96)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = (((c.m.r32(((fp.wrapping_add(208)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)) as u64); let a_ = ((l_p_15).wrapping_add(96)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
        { let v_ = ((255i32) as u64); let a_ = ((l_p_15).wrapping_add(96)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
        { let v_ = (((c.m.r32((fp.wrapping_add(208)).wrapping_add((((3i32) as u32)).wrapping_mul(16))) as i32)) as u64); let a_ = (l_p_15).wrapping_add(112); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = (((c.m.r32(((fp.wrapping_add(208)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(4)) as i32)) as u64); let a_ = ((l_p_15).wrapping_add(112)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = (((c.m.r32(((fp.wrapping_add(208)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)) as u64); let a_ = ((l_p_15).wrapping_add(112)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
        { let v_ = ((255i32) as u64); let a_ = ((l_p_15).wrapping_add(112)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
        l_z_13 = ((((c.m.r32(((fp.wrapping_add(208)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)).wrapping_add((c.m.r32(((fp.wrapping_add(208)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32))).wrapping_add(((c.m.r32(((fp.wrapping_add(208)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)).wrapping_add((c.m.r32(((fp.wrapping_add(208)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)))).wrapping_shr((10i32) as u32);
        { let v_ = c.m.r64(p_tex_9); let a_ = (l_p_15).wrapping_add(40); c.m.w64(a_, v_); };
        if (((((l_z_13) < (0i32)) as i32)) != 0) {
            l_e_16 = ((c.m.r32(sym::gOtZ)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((l_l_14) as u32)).wrapping_mul(8));
        } else {
            if (((((l_z_13) >= (4096i32)) as i32)) != 0) {
                l_e_16 = ((c.m.r32(sym::gOtZ)).wrapping_add((((4095i32) as u32)).wrapping_mul(16))).wrapping_add((((l_l_14) as u32)).wrapping_mul(8));
            } else {
                l_e_16 = ((c.m.r32(sym::gOtZ)).wrapping_add((((l_z_13) as u32)).wrapping_mul(16))).wrapping_add((((l_l_14) as u32)).wrapping_mul(8));
            }
        }
        { let v_ = l_p_15; let a_ = (c.m.r32((l_e_16).wrapping_add(4))).wrapping_add(4); c.m.w32(a_, v_); };
        { let v_ = l_p_15; let a_ = (l_e_16).wrapping_add(4); c.m.w32(a_, v_); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftBubble_DrawOne", &PROBE_N, &[], true); }
    r_
}

pub fn EftBubble_SetTexMode(c: &mut Ctx, mut p_a_1: i32, mut p_b_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBubble_SetTexMode", &PROBE_N, &[(p_a_1) as u32, (p_b_2) as u32], false); }
    let mut l_i_3: i32 = 0;
    let r_ = 'f: {
    if ((((({ let a_ = (c.m.r32(sym::gEftBubble)).wrapping_add(16); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
        {
            l_i_3 = 5i32;
            'l1: loop {
                if !(((((l_i_3) < (9i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let v_ = { let a0_ = ((c.m.r32(sym::gEftBubble)).wrapping_add(104)).wrapping_add((((l_i_3) as u32)).wrapping_mul(16)); F::EftVram_AddTex(c, a0_, p_a_1, p_b_2) }; let a_ = ((c.m.r32(sym::gEftBubble)).wrapping_add(104)).wrapping_add((((l_i_3) as u32)).wrapping_mul(16)); c.m.w64(a_, v_); };
                }
                { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBubble_SetTexMode", &PROBE_N, &[], true); }
    r_
}

pub fn EftStageScroll_Init(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftStageScroll_Init", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let v_ = { let a0_ = F::BtlPool_GetCurrent(c); F::BtlPool_Alloc(c, a0_, ((720u32) as i32)) }; let a_ = sym::gEftStageScroll; c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32(sym::gEftStageScroll); F::memset(c, a0_, 0i32, 720u32) }; };
    F::EftStageScroll_Load(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftStageScroll_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftStageScroll_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftStageScroll_Term", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let a0_ = F::BtlPool_GetCurrent(c); let a1_ = c.m.r32(sym::gEftStageScroll); F::BtlPool_Free(c, a0_, a1_) };
    { let v_ = ((0i32) as u32); let a_ = sym::gEftStageScroll; c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftStageScroll_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftStageScroll_Update(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftStageScroll_Update", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if (((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0) {
        break 'f;
    }
    if ((((!((F::BtlStage_IsReady(c)) != 0)) as i32)) != 0) {
        break 'f;
    }
    if ((((!(({ let a_ = (c.m.r32(sym::gEftStageScroll)).wrapping_add(704); (c.m.r32(a_) as i32) }) != 0)) as i32)) != 0) {
        break 'f;
    }
    { let r_ = rt::fmul({ let a_ = (c.m.r32(sym::gEftStageScroll)).wrapping_add(672); F32(c.m.r32(a_)) }, { let a_ = (c.m.r32(sym::gEftStageScroll)).wrapping_add(688); F32(c.m.r32(a_)) }); let a_ = (c.m.r32(sym::gEftStageScroll)).wrapping_add(680); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
    { let r_ = rt::fmul({ let a_ = (c.m.r32(sym::gEftStageScroll)).wrapping_add(676); F32(c.m.r32(a_)) }, { let a_ = (c.m.r32(sym::gEftStageScroll)).wrapping_add(688); F32(c.m.r32(a_)) }); let a_ = (c.m.r32(sym::gEftStageScroll)).wrapping_add(684); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
    if (((rt::fgt({ let a_ = (c.m.r32(sym::gEftStageScroll)).wrapping_add(680); F32(c.m.r32(a_)) }, { let a_ = (c.m.r32(sym::gEftStageScroll)).wrapping_add(692); F32(c.m.r32(a_)) }) as i32)) != 0) {
        { let v_ = rt::fsub({ let a_ = (c.m.r32(sym::gEftStageScroll)).wrapping_add(680); F32(c.m.r32(a_)) }, { let a_ = (c.m.r32(sym::gEftStageScroll)).wrapping_add(692); F32(c.m.r32(a_)) }); let a_ = (c.m.r32(sym::gEftStageScroll)).wrapping_add(680); c.m.w32(a_, (v_).0); };
    } else {
        if (((rt::flt({ let a_ = (c.m.r32(sym::gEftStageScroll)).wrapping_add(680); F32(c.m.r32(a_)) }, rt::fneg({ let a_ = (c.m.r32(sym::gEftStageScroll)).wrapping_add(692); F32(c.m.r32(a_)) })) as i32)) != 0) {
            { let v_ = rt::fadd({ let a_ = (c.m.r32(sym::gEftStageScroll)).wrapping_add(680); F32(c.m.r32(a_)) }, { let a_ = (c.m.r32(sym::gEftStageScroll)).wrapping_add(692); F32(c.m.r32(a_)) }); let a_ = (c.m.r32(sym::gEftStageScroll)).wrapping_add(680); c.m.w32(a_, (v_).0); };
        }
    }
    if (((rt::fgt({ let a_ = (c.m.r32(sym::gEftStageScroll)).wrapping_add(684); F32(c.m.r32(a_)) }, { let a_ = (c.m.r32(sym::gEftStageScroll)).wrapping_add(692); F32(c.m.r32(a_)) }) as i32)) != 0) {
        { let v_ = rt::fsub({ let a_ = (c.m.r32(sym::gEftStageScroll)).wrapping_add(684); F32(c.m.r32(a_)) }, { let a_ = (c.m.r32(sym::gEftStageScroll)).wrapping_add(692); F32(c.m.r32(a_)) }); let a_ = (c.m.r32(sym::gEftStageScroll)).wrapping_add(684); c.m.w32(a_, (v_).0); };
    } else {
        if (((rt::flt({ let a_ = (c.m.r32(sym::gEftStageScroll)).wrapping_add(684); F32(c.m.r32(a_)) }, rt::fneg({ let a_ = (c.m.r32(sym::gEftStageScroll)).wrapping_add(692); F32(c.m.r32(a_)) })) as i32)) != 0) {
            { let v_ = rt::fadd({ let a_ = (c.m.r32(sym::gEftStageScroll)).wrapping_add(684); F32(c.m.r32(a_)) }, { let a_ = (c.m.r32(sym::gEftStageScroll)).wrapping_add(692); F32(c.m.r32(a_)) }); let a_ = (c.m.r32(sym::gEftStageScroll)).wrapping_add(684); c.m.w32(a_, (v_).0); };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftStageScroll_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftStageScroll_Reset(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftStageScroll_Reset", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::EftStageScroll_Load(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftStageScroll_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn EftStageScroll_DrawStub(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftStageScroll_DrawStub", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if ((((!((F::BtlStage_IsReady(c)) != 0)) as i32)) != 0) {
        break 'f;
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftStageScroll_DrawStub", &PROBE_N, &[], true); }
    r_
}

pub fn EftStageScroll_Draw(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftStageScroll_Draw", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(64) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 64);
    let mut l_p_1: u32 = 0;
    let r_ = 'f: {
        if (((((c.m.r32(sym::gEftStageScroll)) == (((0i32) as u32))) as i32)) != 0) {
            break 'f;
        }
        if ((((!((F::BtlStage_IsReady(c)) != 0)) as i32)) != 0) {
            break 'f;
        }
        F::EftGfx_UpdateClipPlanes(c);
        if ((((!(({ let a_ = (c.m.r32(sym::gEftStageScroll)).wrapping_add(704); (c.m.r32(a_) as i32) }) != 0)) as i32)) != 0) {
            break 'f;
        }
        if ((((!((F::EftStage_IsDrawOn(c)) != 0)) as i32)) != 0) {
            break 'f;
        }
        F::EftStageScroll_PlaceTextures(c, 1i32, 0i32);
        l_p_1 = F::Dma_BeginDirect(c);
        { let v_ = ((1152921504606879747i64) as u64); let a_ = (l_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((14i32) as u64); let a_ = (l_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_1 = n_; };
        { let v_ = ((68i32) as u64); let a_ = (l_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((66i32) as u64); let a_ = (l_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_1 = n_; };
        { let v_ = ((327680i32) as u64); let a_ = (l_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((71i32) as u64); let a_ = (l_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_1 = n_; };
        { let v_ = ((0i32) as u64); let a_ = (l_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((8i32) as u64); let a_ = (l_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_1 = n_; };
        F::Dma_EndDirect(c, l_p_1);
        F::Mtx_StoreIdentity(c, fp);
        { let v_ = F32(0x00000000); let a_ = ((fp).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = ((fp).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = ((fp).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let a0_ = (c.m.r32(sym::gEftStageScroll)).wrapping_add(528); F::EftMesh_SetRepeat(c, a0_, 1i32) };
        { let a0_ = (c.m.r32(sym::gEftStageScroll)).wrapping_add(528); let a1_ = { let a_ = (c.m.r32(sym::gEftStageScroll)).wrapping_add(692); F32(c.m.r32(a_)) }; F::EftMesh_SetUvScale(c, a0_, a1_) };
        { let a0_ = (c.m.r32(sym::gEftStageScroll)).wrapping_add(528); let a1_ = { let a_ = (c.m.r32(sym::gEftStageScroll)).wrapping_add(680); F32(c.m.r32(a_)) }; let a2_ = { let a_ = (c.m.r32(sym::gEftStageScroll)).wrapping_add(684); F32(c.m.r32(a_)) }; F::EftMesh_SetUvOfs(c, a0_, a1_, a2_) };
        { let a0_ = (c.m.r32(sym::gEftStageScroll)).wrapping_add(528); F::EftMesh_SetMtx(c, a0_, fp) };
        { let a0_ = (c.m.r32(sym::gEftStageScroll)).wrapping_add(528); F::EftMesh_DrawNow(c, a0_, 1i32) };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftStageScroll_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn EftStageScroll_PlaceTextures(c: &mut Ctx, mut p_tcc_1: i32, mut p_tfx_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftStageScroll_PlaceTextures", &PROBE_N, &[(p_tcc_1) as u32, (p_tfx_2) as u32], false); }
    let mut l_tbp_3: i32 = 0;
    let mut l_cbp_4: i32 = 0;
    let mut l_set_5: u32 = 0;
    let mut l_i_6: i32 = 0;
    let r_ = 'f: {
    l_tbp_3 = 10784i32;
    l_cbp_4 = 10752i32;
    l_set_5 = (c.m.r32(sym::gEftStageScroll)).wrapping_add(8);
    {
        l_i_6 = 0i32;
        'l1: loop {
            if !(((((l_i_6) < ((c.m.r32((l_set_5).wrapping_add(512)) as i32))) as i32)) != 0) { break 'l1; }
            'c2: {
                { let v_ = ((((((((c.m.r64((l_set_5).wrapping_add((((l_i_6) as u32)).wrapping_mul(16)))) & (18444492291075719168u64))) | ((((p_tcc_1) as u64)).wrapping_shl((34i32) as u32)))) | ((((((p_tfx_2) as u64)).wrapping_shl((35i32) as u32)) | (((((l_tbp_3) as u32)) as u64)))))) | ((((l_cbp_4) as u64)).wrapping_shl((37i32) as u32))); let a_ = (l_set_5).wrapping_add((((l_i_6) as u32)).wrapping_mul(16)); c.m.w64(a_, v_); };
                { let a0_ = c.m.r32(((l_set_5).wrapping_add((((l_i_6) as u32)).wrapping_mul(16))).wrapping_add(8)); F::Tex_Upload(c, a0_, l_tbp_3, l_cbp_4) };
                { let r_ = { let a_ = (c.m.r32(((l_set_5).wrapping_add((((l_i_6) as u32)).wrapping_mul(16))).wrapping_add(8))).wrapping_add(16); (c.m.r32(a_) as i32) }; let n_ = (l_tbp_3).wrapping_add(r_); l_tbp_3 = n_; };
                { let r_ = { let a_ = (c.m.r32(((l_set_5).wrapping_add((((l_i_6) as u32)).wrapping_mul(16))).wrapping_add(8))).wrapping_add(20); (c.m.r32(a_) as i32) }; let n_ = (l_cbp_4).wrapping_add(r_); l_cbp_4 = n_; };
            }
            { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftStageScroll_PlaceTextures", &PROBE_N, &[], true); }
    r_
}

pub fn EftStageScroll_Load(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftStageScroll_Load", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_file_1: u32 = 0;
    let mut l_p_2: u32 = 0;
    let mut l_texPack_3: u32 = 0;
    let mut l_deg_4: F32 = F32(0);
    let mut l_angle_5: F32 = F32(0);
    let r_ = 'f: {
        l_file_1 = { let a_ = (c.m.r32(sym::gCommonRes)).wrapping_add(36); c.m.r32(a_) };
        l_p_2 = (l_file_1).wrapping_add((rt::div_u32(c.m.r32((l_file_1).wrapping_add(((28i32) as u32))), ((4i32) as u32))).wrapping_mul(((4i32) as u32)));
        l_texPack_3 = (l_file_1).wrapping_add((rt::div_u32(c.m.r32((l_file_1).wrapping_add(((24i32) as u32))), ((4i32) as u32))).wrapping_mul(((4i32) as u32)));
        { let v_ = 1i32; let a_ = (c.m.r32(sym::gEftStageScroll)).wrapping_add(704); c.m.w32(a_, (v_) as u32); };
        { { let _ = { rt::memcpy(c, fp.wrapping_add(16), l_p_2, (((4i32) as u32)) as u32); fp.wrapping_add(16) }; }; { let r_ = 4i32; let n_ = (l_p_2).wrapping_add(((r_) as u32)); l_p_2 = n_; }; };
        { let v_ = rt::i2f((c.m.r32(fp.wrapping_add(16)) as i32)); let a_ = (c.m.r32(sym::gEftStageScroll)).wrapping_add(692); c.m.w32(a_, (v_).0); };
        { { let _ = { rt::memcpy(c, fp.wrapping_add(16), l_p_2, (((4i32) as u32)) as u32); fp.wrapping_add(16) }; }; { let r_ = 4i32; let n_ = (l_p_2).wrapping_add(((r_) as u32)); l_p_2 = n_; }; };
        { let v_ = rt::i2f((c.m.r32(fp.wrapping_add(16)) as i32)); let a_ = (c.m.r32(sym::gEftStageScroll)).wrapping_add(696); c.m.w32(a_, (v_).0); };
        { { let _ = { rt::memcpy(c, fp.wrapping_add(16), l_p_2, (((4i32) as u32)) as u32); fp.wrapping_add(16) }; }; { let r_ = 4i32; let n_ = (l_p_2).wrapping_add(((r_) as u32)); l_p_2 = n_; }; };
        { let v_ = rt::fmul(rt::fmul(rt::i2f((c.m.r32(fp.wrapping_add(16)) as i32)), F32(0x3727c5ac)), { let a_ = (c.m.r32(sym::gEftStageScroll)).wrapping_add(692); F32(c.m.r32(a_)) }); let a_ = (c.m.r32(sym::gEftStageScroll)).wrapping_add(688); c.m.w32(a_, (v_).0); };
        { { let _ = { rt::memcpy(c, fp.wrapping_add(16), l_p_2, (((4i32) as u32)) as u32); fp.wrapping_add(16) }; }; { let r_ = 4i32; let n_ = (l_p_2).wrapping_add(((r_) as u32)); l_p_2 = n_; }; };
        { let v_ = rt::i2f((c.m.r32(fp.wrapping_add(16)) as i32)); let a_ = (c.m.r32(sym::gEftStageScroll)).wrapping_add(700); c.m.w32(a_, (v_).0); };
        if (((rt::flt({ let a_ = (c.m.r32(sym::gEftStageScroll)).wrapping_add(700); F32(c.m.r32(a_)) }, F32(0x00000000)) as i32)) != 0) {
            l_deg_4 = rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x43b40000));
        } else {
            l_deg_4 = { let a_ = (c.m.r32(sym::gEftStageScroll)).wrapping_add(700); F32(c.m.r32(a_)) };
        }
        l_angle_5 = F::EftMath_WrapAngle(c, rt::fdiv(rt::fmul(l_deg_4, F32(0x40490fda)), F32(0x43340000)));
        { let v_ = F::Port_cosf(c, l_angle_5); let a_ = fp; c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = F::Port_sinf(c, l_angle_5); let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = (fp).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32(fp)); let a_ = (c.m.r32(sym::gEftStageScroll)).wrapping_add(672); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((fp).wrapping_add(8))); let a_ = (c.m.r32(sym::gEftStageScroll)).wrapping_add(676); c.m.w32(a_, (v_).0); };
        { let a0_ = (c.m.r32(sym::gEftStageScroll)).wrapping_add(8); F::EftTexSet_Load32(c, a0_, l_texPack_3) };
        { let a0_ = (c.m.r32(sym::gEftStageScroll)).wrapping_add(528); F::EftMesh_Init(c, a0_, l_p_2) };
        { let a0_ = (c.m.r32(sym::gEftStageScroll)).wrapping_add(528); let a1_ = (c.m.r32(sym::gEftStageScroll)).wrapping_add(8); F::EftMesh_SetTex(c, a0_, a1_) };
        { let a0_ = (c.m.r32(sym::gEftStageScroll)).wrapping_add(528); F::EftMesh_SetClip(c, a0_, 1i32) };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftStageScroll_Load", &PROBE_N, &[], true); }
    r_
}

pub fn EftGeyser_Create(c: &mut Ctx, mut p_param_1: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGeyser_Create", &PROBE_N, &[(p_param_1) as u32], false); }
    let r_ = 'f: {
    if (((((c.m.r32(sym::gEftGeyserMgr)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f ((0i32) as u32);
    }
    break 'f { let a0_ = { let a_ = c.m.r32(sym::gEftGeyserMgr); c.m.r32(a_) }; F::BtlTaskList_AddTail(c, a0_, sym::gEftGeyserClass, p_param_1) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftGeyser_Create", &PROBE_N, &[], true); }
    r_
}

pub fn EftGeyser_GetWork(c: &mut Ctx, mut p_task_1: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGeyser_GetWork", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f ((0i32) as u32);
    }
    if ((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) != (fid::EftGeyser_Update)) as i32)) != 0) {
        break 'f ((0i32) as u32);
    }
    if (((rt::bf_get_u(c, p_task_1, 0, 1) as u8)) != 0) {
        break 'f ((0i32) as u32);
    }
    break 'f c.m.r32((p_task_1).wrapping_add(56));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftGeyser_GetWork", &PROBE_N, &[], true); }
    r_
}

pub fn EftGeyser_Kill(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGeyser_Kill", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    if ((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) != (fid::EftGeyser_Update)) as i32)) != 0) {
        break 'f;
    }
    if (((rt::bf_get_u(c, p_task_1, 0, 1) as u8)) != 0) {
        break 'f;
    }
    F::BtlTask_SetDead(c, p_task_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftGeyser_Kill", &PROBE_N, &[], true); }
    r_
}

pub fn EftGeyser_MgrInit(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGeyser_MgrInit", &PROBE_N, &[(p_task_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(112) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 112);
    let mut l_stage_2: u32 = 0;
    let mut l_count_3: i32 = 0;
    let mut l_res_4: u32 = 0;
    let mut l_rec_5: u32 = 0;
    let mut l_i_6: i32 = 0;
    let r_ = 'f: {
        l_stage_2 = ((((F::BtlScene_GetStageData(c)) as i64 as u64)) as u32);
        l_count_3 = ((((F::BtlStage_GetFxResA(c)) as u64)) as i32);
        { let v_ = { let a0_ = F::BtlPool_GetCurrent(c); F::BtlPool_Alloc(c, a0_, ((528u32) as i32)) }; let a_ = sym::gEftGeyserMgr; c.m.w32(a_, v_); };
        { let _ = { let a0_ = c.m.r32(sym::gEftGeyserMgr); F::memset(c, a0_, 0i32, 528u32) }; };
        { let v_ = F::BtlTask_CreateChildList(c, p_task_1, (((((l_count_3).wrapping_add(1i32)) as i64 as u64)) as u32), ((((((256u32) as i32)) as i64 as u64)) as u32)); let a_ = c.m.r32(sym::gEftGeyserMgr); c.m.w32(a_, v_); };
        { let a0_ = (c.m.r32(sym::gEftGeyserMgr)).wrapping_add(8); let a1_ = F::BtlScene_GetPackEntry(c, l_stage_2, 18i32); F::EftTexSet_Load32(c, a0_, a1_) };
        l_res_4 = F::BtlStage_GetFxResA2(c);
        if (((((l_res_4) != (((0i32) as u32))) as i32)) != 0) {
            {
                l_i_6 = 0i32;
                'l1: loop {
                    if !(((((l_i_6) < (l_count_3)) as i32)) != 0) { break 'l1; }
                    'c2: {
                        l_rec_5 = (l_res_4).wrapping_add((((l_i_6) as u32)).wrapping_mul(48));
                        { let _ = F::memset(c, fp, 0i32, 112u32); };
                        { let a1_ = rt::i2f((c.m.r8((l_rec_5).wrapping_add(30))) as i32); let a2_ = rt::i2f((c.m.r8((l_rec_5).wrapping_add(31))) as i32); let a3_ = rt::i2f((c.m.r8((l_rec_5).wrapping_add(32))) as i32); let a4_ = rt::i2f((c.m.r8((l_rec_5).wrapping_add(33))) as i32); F::Vec4_Set(c, (fp).wrapping_add(32), a1_, a2_, a3_, a4_) };
                        F::Vec4_Copy(c, (fp).wrapping_add(48), (fp).wrapping_add(32));
                        { let v_ = F32(c.m.r32((l_rec_5).wrapping_add(12))); let a_ = (fp).wrapping_add(64); c.m.w32(a_, (v_).0); };
                        { let v_ = F32(c.m.r32((l_rec_5).wrapping_add(16))); let a_ = (fp).wrapping_add(68); c.m.w32(a_, (v_).0); };
                        { let v_ = F32(0x3f800000); let a_ = (fp).wrapping_add(72); c.m.w32(a_, (v_).0); };
                        { let v_ = F32(0x3f800000); let a_ = (fp).wrapping_add(76); c.m.w32(a_, (v_).0); };
                        { let v_ = F32(0x40000000); let a_ = (fp).wrapping_add(80); c.m.w32(a_, (v_).0); };
                        { let v_ = F32(c.m.r32((l_rec_5).wrapping_add(20))); let a_ = (fp).wrapping_add(84); c.m.w32(a_, (v_).0); };
                        { let v_ = ((c.m.r16((l_rec_5).wrapping_add(24))) as i32); let a_ = (fp).wrapping_add(88); c.m.w32(a_, (v_) as u32); };
                        { let v_ = ((c.m.r16((l_rec_5).wrapping_add(26))) as i32); let a_ = (fp).wrapping_add(92); c.m.w32(a_, (v_) as u32); };
                        { let v_ = ((c.m.r16((l_rec_5).wrapping_add(28))) as i32); let a_ = (fp).wrapping_add(96); c.m.w32(a_, (v_) as u32); };
                        { let v_ = F32(c.m.r32(l_rec_5)); let a_ = fp; c.m.w32(a_, (v_).0); };
                        { let v_ = F32(c.m.r32((l_rec_5).wrapping_add(4))); let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
                        { let v_ = F32(c.m.r32((l_rec_5).wrapping_add(8))); let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_).0); };
                        { let v_ = F32(0x3f800000); let a_ = (fp).wrapping_add(12); c.m.w32(a_, (v_).0); };
                        F::Vec4_Copy(c, (fp).wrapping_add(16), fp);
                        { let v_ = rt::fadd(F32(c.m.r32((fp).wrapping_add(4))), F32(c.m.r32((fp).wrapping_add(68)))); let a_ = ((fp).wrapping_add(16)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                        { let _ = F::EftGeyser_Create(c, fp); };
                    }
                    { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; };
                }
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftGeyser_MgrInit", &PROBE_N, &[], true); }
    r_
}

pub fn EftGeyser_MgrTerm(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGeyser_MgrTerm", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let a0_ = F::BtlPool_GetCurrent(c); let a1_ = c.m.r32(sym::gEftGeyserMgr); F::BtlPool_Free(c, a0_, a1_) };
    { let v_ = ((0i32) as u32); let a_ = sym::gEftGeyserMgr; c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftGeyser_MgrTerm", &PROBE_N, &[], true); }
    r_
}

pub fn EftGeyser_MgrUpdate(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGeyser_MgrUpdate", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if ((((({ let a_ = ({ let a_ = c.m.r32(sym::gEftGeyserMgr); c.m.r32(a_) }).wrapping_add((((1i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
        { let v_ = { let a0_ = ((c.m.r32(sym::gEftGeyserMgr)).wrapping_add(8)).wrapping_add((((1i32) as u32)).wrapping_mul(16)); F::EftVram_AddTex(c, a0_, 1i32, 0i32) }; let a_ = ((c.m.r32(sym::gEftGeyserMgr)).wrapping_add(8)).wrapping_add((((1i32) as u32)).wrapping_mul(16)); c.m.w64(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftGeyser_MgrUpdate", &PROBE_N, &[], true); }
    r_
}

pub fn EftGeyser_Init(c: &mut Ctx, mut p_task_1: u32, mut p_param_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGeyser_Init", &PROBE_N, &[(p_task_1) as u32, (p_param_2) as u32], false); }
    let mut l_w_3: u32 = 0;
    let r_ = 'f: {
    l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
    { let _ = F::memset(c, l_w_3, 0i32, 256u32); };
    { let s_ = p_param_2; let d_ = l_w_3; rt::memcpy(c, d_, s_, 112); };
    { let s_ = p_param_2; let d_ = (l_w_3).wrapping_add(112); rt::memcpy(c, d_, s_, 112); };
    { let v_ = (c.m.r32(sym::gEftGeyserMgr)).wrapping_add(8); let a_ = (l_w_3).wrapping_add(232); c.m.w32(a_, v_); };
    { let v_ = rt::rem_i32(F::Port_Rand(c), 180i32); let a_ = (l_w_3).wrapping_add(244); c.m.w32(a_, (v_) as u32); };
    F::EftGeyser_Restart(c, p_task_1, l_w_3);
    F::EftGeyser_StartSmoke(c, p_task_1);
    F::EftGeyser_StartSteam(c, p_task_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftGeyser_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftGeyser_Term(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGeyser_Term", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    if (((((c.m.r32((l_w_2).wrapping_add(240))) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32((l_w_2).wrapping_add(240)); F::EftSteam_Stop(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = (l_w_2).wrapping_add(240); c.m.w32(a_, v_); };
    }
    if (((((c.m.r32((l_w_2).wrapping_add(236))) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32((l_w_2).wrapping_add(236)); F::EftSmoke_Stop(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = (l_w_2).wrapping_add(236); c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftGeyser_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftGeyser_ResetStub(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGeyser_ResetStub", &PROBE_N, &[], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftGeyser_ResetStub", &PROBE_N, &[], true); }
    r_
}

pub fn EftGeyser_Update(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGeyser_Update", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    if ((F::BtlScene_IsTimeStopped(c)) != 0) {
        break 'f;
    }
    's1: {
        let sv3: i32 = ((c.m.r8((p_task_1).wrapping_add(1))) as i32);
        'k7: {
        'k6: {
        'k5: {
        'k4: {
        'k3: {
        'k2: {
            match sv3 { 0i32 => break 'k2, 1i32 => break 'k3, 2i32 => break 'k4, 3i32 => break 'k5, 4i32 => break 'k6, 5i32 => break 'k7, _ => break 's1, }
        }
        if ((((({ let a_ = (l_w_2).wrapping_add(244); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) <= (0i32)) as i32)) != 0) {
            { let v_ = ((1i32) as u8); let a_ = (p_task_1).wrapping_add(1); c.m.w8(a_, v_); };
        }
        break 's1;
        }
        F::EftGeyser_Restart(c, p_task_1, l_w_2);
        { let a_ = (p_task_1).wrapping_add(1); let o_ = c.m.r8(a_); let n_ = (o_).wrapping_add(1); c.m.w8(a_, n_); };
        }
        { let v_ = F32(c.m.r32(((l_w_2).wrapping_add(112)).wrapping_add(4))); let a_ = (((l_w_2).wrapping_add(112)).wrapping_add(16)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let a_ = (p_task_1).wrapping_add(1); let o_ = c.m.r8(a_); let n_ = (o_).wrapping_add(1); c.m.w8(a_, n_); };
        break 's1;
        }
        { let r_ = F32(c.m.r32((l_w_2).wrapping_add(224))); let a_ = (((l_w_2).wrapping_add(112)).wrapping_add(48)).wrapping_add(12); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        if (((rt::fgt(F32(c.m.r32((((l_w_2).wrapping_add(112)).wrapping_add(48)).wrapping_add(12))), F32(c.m.r32(((l_w_2).wrapping_add(32)).wrapping_add(12)))) as i32)) != 0) {
            { let v_ = F32(c.m.r32(((l_w_2).wrapping_add(32)).wrapping_add(12))); let a_ = (((l_w_2).wrapping_add(112)).wrapping_add(48)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        }
        if ((((({ let a_ = ((l_w_2).wrapping_add(112)).wrapping_add(88); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) <= (0i32)) as i32)) != 0) {
            { let a0_ = c.m.r32((l_w_2).wrapping_add(240)); F::EftSteam_SetPaused(c, a0_, 1i32) };
            { let v_ = 0i32; let a_ = ((l_w_2).wrapping_add(112)).wrapping_add(88); c.m.w32(a_, (v_) as u32); };
            { let a_ = (p_task_1).wrapping_add(1); let o_ = c.m.r8(a_); let n_ = (o_).wrapping_add(1); c.m.w8(a_, n_); };
        }
        break 's1;
        }
        { let r_ = F32(c.m.r32((l_w_2).wrapping_add(224))); let a_ = (((l_w_2).wrapping_add(112)).wrapping_add(48)).wrapping_add(12); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        { let r_ = F32(c.m.r32(((l_w_2).wrapping_add(112)).wrapping_add(80))); let a_ = (((l_w_2).wrapping_add(112)).wrapping_add(16)).wrapping_add(4); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        if (((rt::fle(F32(c.m.r32((((l_w_2).wrapping_add(112)).wrapping_add(48)).wrapping_add(12))), F32(0x00000000)) as i32)) != 0) {
            { let v_ = F32(0x00000000); let a_ = (((l_w_2).wrapping_add(112)).wrapping_add(48)).wrapping_add(12); c.m.w32(a_, (v_).0); };
            { let a_ = (p_task_1).wrapping_add(1); let o_ = c.m.r8(a_); let n_ = (o_).wrapping_add(1); c.m.w8(a_, n_); };
        }
        break 's1;
        }
        if ((((({ let a_ = ((l_w_2).wrapping_add(112)).wrapping_add(96); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) <= (0i32)) as i32)) != 0) {
            { let v_ = 0i32; let a_ = ((l_w_2).wrapping_add(112)).wrapping_add(96); c.m.w32(a_, (v_) as u32); };
            { let v_ = ((1i32) as u8); let a_ = (p_task_1).wrapping_add(1); c.m.w8(a_, v_); };
        }
        break 's1;
    }
    { let r_ = F32(c.m.r32(((l_w_2).wrapping_add(112)).wrapping_add(84))); let a_ = (l_w_2).wrapping_add(228); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
    if (((rt::fgt(F32(c.m.r32((l_w_2).wrapping_add(228))), F32(0x3f800000)) as i32)) != 0) {
        { let r_ = F32(0x3f800000); let a_ = (l_w_2).wrapping_add(228); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
    }
    if (((rt::flt(F32(c.m.r32((l_w_2).wrapping_add(228))), F32(0x00000000)) as i32)) != 0) {
        { let r_ = F32(0x3f800000); let a_ = (l_w_2).wrapping_add(228); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftGeyser_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftGeyser_Draw(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGeyser_Draw", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    if (((({ let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(8); (c.m.r32(a_) as i32) }) & (1i32))) != 0) {
        break 'f;
    }
    if ((((!((F::EftStage_IsDrawOn(c)) != 0)) as i32)) != 0) {
        break 'f;
    }
    F::Vu0Cur_Push(c);
    { let a0_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(320); F::Vu0Cur_LoadMtx(c, a0_) };
    F::EftGeyser_DrawColumn(c, p_task_1);
    F::Vu0Cur_Pop(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftGeyser_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn EftGeyser_Restart(c: &mut Ctx, mut p_task_1: u32, mut p_w_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGeyser_Restart", &PROBE_N, &[(p_task_1) as u32, (p_w_2) as u32], false); }
    let r_ = 'f: {
    { let s_ = p_w_2; let d_ = (p_w_2).wrapping_add(112); rt::memcpy(c, d_, s_, 112); };
    F::Vec4_Copy(c, ((p_w_2).wrapping_add(112)).wrapping_add(48), ((p_w_2).wrapping_add(112)).wrapping_add(32));
    { let v_ = F32(0x00000000); let a_ = (((p_w_2).wrapping_add(112)).wrapping_add(48)).wrapping_add(12); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fdiv(F32(c.m.r32(((p_w_2).wrapping_add(32)).wrapping_add(12))), rt::i2f((c.m.r32(((p_w_2).wrapping_add(112)).wrapping_add(92)) as i32))); let a_ = (p_w_2).wrapping_add(224); c.m.w32(a_, (v_).0); };
    { let a0_ = c.m.r32((p_w_2).wrapping_add(240)); F::EftSteam_SetPaused(c, a0_, 0i32) };
    { let a0_ = c.m.r32((p_w_2).wrapping_add(240)); F::EftSteam_SetPos(c, a0_, p_w_2) };
    { let a0_ = c.m.r32((p_w_2).wrapping_add(236)); F::EftSmoke_SetPos(c, a0_, p_w_2) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftGeyser_Restart", &PROBE_N, &[], true); }
    r_
}

pub fn EftGeyser_DrawColumn(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftGeyser_DrawColumn", &PROBE_N, &[(p_task_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(464) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 464);
    let mut l_first_2: i32 = 0;
    let mut l_w_3: u32 = 0;
    let mut l_v_4: F32 = F32(0);
    let mut l_dv_5: F32 = F32(0);
    let mut l_vPrev_6: F32 = F32(0);
    let mut l_width_7: F32 = F32(0);
    let mut l_texV_8: F32 = F32(0);
    let mut l_fade_9: F32 = F32(0);
    let mut l_i_10: i32 = 0;
    let mut l_z_11: i32 = 0;
    let mut l_abe_12: i32 = 0;
    let mut l_tex0_13: u64 = 0;
    let mut l_q_14: u32 = 0;
    let mut l_p_15: u32 = 0;
    let mut l_e_16: u32 = 0;
    let r_ = 'f: {
        rt::memset(c, fp.wrapping_add(400), 0, 64);
        rt::memset(c, fp.wrapping_add(400).wrapping_add(0), 0, 16);
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(400).wrapping_add(0).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(400).wrapping_add(0).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(400).wrapping_add(0).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(400).wrapping_add(0).wrapping_add(12), (v_).0); }
        rt::memset(c, fp.wrapping_add(400).wrapping_add(16), 0, 16);
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(400).wrapping_add(16).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(400).wrapping_add(16).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(400).wrapping_add(16).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(400).wrapping_add(16).wrapping_add(12), (v_).0); }
        rt::memset(c, fp.wrapping_add(400).wrapping_add(32), 0, 16);
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(400).wrapping_add(32).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(400).wrapping_add(32).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(400).wrapping_add(32).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(400).wrapping_add(32).wrapping_add(12), (v_).0); }
        rt::memset(c, fp.wrapping_add(400).wrapping_add(48), 0, 16);
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(400).wrapping_add(48).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(400).wrapping_add(48).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(400).wrapping_add(48).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(400).wrapping_add(48).wrapping_add(12), (v_).0); }
        l_first_2 = 1i32;
        l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
        l_v_4 = F32(0x3eaaaaaa);
        l_dv_5 = l_v_4;
        l_vPrev_6 = F32(0x00000000);
        l_width_7 = F32(c.m.r32(((l_w_3).wrapping_add(112)).wrapping_add(64)));
        l_texV_8 = F32(c.m.r32((l_w_3).wrapping_add(228)));
        l_fade_9 = F::EftStage_GetTintScale(c);
        { let a1_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(544); F::Vec4_Copy(c, fp.wrapping_add(64), a1_) };
        {
            l_i_10 = 0i32;
            'l1: loop {
                if !(((((l_i_10) < (4i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    F::Vec4_Copy(c, (fp.wrapping_add(336)).wrapping_add((((l_i_10) as u32)).wrapping_mul(16)), (fp.wrapping_add(400)).wrapping_add((((l_i_10) as u32)).wrapping_mul(16)));
                }
                { let o_ = l_i_10; let n_ = (o_).wrapping_add(1); l_i_10 = n_; };
            }
        }
        {
            l_i_10 = 0i32;
            'l3: loop {
                if !(((((l_i_10) < (4i32)) as i32)) != 0) { break 'l3; }
                'c4: {
                    { let v_ = F32(c.m.r32(((l_w_3).wrapping_add(112)).wrapping_add(16))); let a_ = (fp).wrapping_add((((l_i_10) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fsub(F32(c.m.r32((((l_w_3).wrapping_add(112)).wrapping_add(16)).wrapping_add(4))), rt::fmul(rt::fmul(F32(c.m.r32(((l_w_3).wrapping_add(112)).wrapping_add(68))), F32(0x3e800000)), rt::i2f(l_i_10))); let a_ = ((fp).wrapping_add((((l_i_10) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(c.m.r32((((l_w_3).wrapping_add(112)).wrapping_add(16)).wrapping_add(8))); let a_ = ((fp).wrapping_add((((l_i_10) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(0x3f800000); let a_ = ((fp).wrapping_add((((l_i_10) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
                }
                { let o_ = l_i_10; let n_ = (o_).wrapping_add(1); l_i_10 = n_; };
            }
        }
        F::Vec3_Scale(c, (fp.wrapping_add(96)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), ((l_w_3).wrapping_add(112)).wrapping_add(48), l_fade_9);
        F::Vec3_Scale(c, (fp.wrapping_add(96)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), ((l_w_3).wrapping_add(112)).wrapping_add(48), l_fade_9);
        F::Vec3_Scale(c, (fp.wrapping_add(96)).wrapping_add((((2i32) as u32)).wrapping_mul(16)), ((l_w_3).wrapping_add(112)).wrapping_add(48), l_fade_9);
        F::Vec3_Scale(c, (fp.wrapping_add(96)).wrapping_add((((3i32) as u32)).wrapping_mul(16)), ((l_w_3).wrapping_add(112)).wrapping_add(48), l_fade_9);
        {
            l_i_10 = 0i32;
            'l5: loop {
                if !(((((l_i_10) < (3i32)) as i32)) != 0) { break 'l5; }
                'c6: {
                    F::Vec4_Sub(c, fp.wrapping_add(160), (fp).wrapping_add(((((l_i_10).wrapping_add(1i32)) as u32)).wrapping_mul(16)), (fp).wrapping_add((((l_i_10) as u32)).wrapping_mul(16)));
                    F::Vec4_Sub(c, fp.wrapping_add(80), (fp).wrapping_add((((l_i_10) as u32)).wrapping_mul(16)), fp.wrapping_add(64));
                    F::Vec3_Cross(c, fp.wrapping_add(160), fp.wrapping_add(160), fp.wrapping_add(80));
                    F::Vec3_Normalize(c, fp.wrapping_add(160), fp.wrapping_add(160));
                    if ((l_first_2) != 0) {
                        F::Vec3_ScaleAdd(c, (fp.wrapping_add(272)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), fp.wrapping_add(160), (fp).wrapping_add((((l_i_10) as u32)).wrapping_mul(16)), l_width_7);
                        F::Vec3_ScaleAdd(c, (fp.wrapping_add(272)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), fp.wrapping_add(160), (fp).wrapping_add((((l_i_10) as u32)).wrapping_mul(16)), rt::fneg(l_width_7));
                        l_first_2 = 0i32;
                    } else {
                        F::Vec4_Copy(c, (fp.wrapping_add(272)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(176)).wrapping_add((((0i32) as u32)).wrapping_mul(16)));
                        F::Vec4_Copy(c, (fp.wrapping_add(272)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(176)).wrapping_add((((1i32) as u32)).wrapping_mul(16)));
                    }
                    F::Vec3_ScaleAdd(c, (fp.wrapping_add(272)).wrapping_add((((2i32) as u32)).wrapping_mul(16)), fp.wrapping_add(160), (fp).wrapping_add(((((l_i_10).wrapping_add(1i32)) as u32)).wrapping_mul(16)), l_width_7);
                    F::Vec3_ScaleAdd(c, (fp.wrapping_add(272)).wrapping_add((((3i32) as u32)).wrapping_mul(16)), fp.wrapping_add(160), (fp).wrapping_add(((((l_i_10).wrapping_add(1i32)) as u32)).wrapping_mul(16)), rt::fneg(l_width_7));
                    F::Vec4_Copy(c, (fp.wrapping_add(176)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(272)).wrapping_add((((2i32) as u32)).wrapping_mul(16)));
                    F::Vec4_Copy(c, (fp.wrapping_add(176)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(272)).wrapping_add((((3i32) as u32)).wrapping_mul(16)));
                    { let v_ = { let v_ = { let v_ = { let v_ = F32(0x3f800000); let a_ = ((fp.wrapping_add(272)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); v_ }; let a_ = ((fp.wrapping_add(272)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); v_ }; let a_ = ((fp.wrapping_add(272)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); v_ }; let a_ = ((fp.wrapping_add(272)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
                    if ((F::Vu0Cur_ProjectPoints(c, fp.wrapping_add(208), fp.wrapping_add(272), 4i32)) != 0) {
                        { let v_ = rt::fadd(l_vPrev_6, l_texV_8); let a_ = ((fp.wrapping_add(336)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                        { let v_ = rt::fadd(l_v_4, l_texV_8); let a_ = ((fp.wrapping_add(336)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                        { let v_ = rt::fadd(l_vPrev_6, l_texV_8); let a_ = ((fp.wrapping_add(336)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                        l_z_11 = ((((c.m.r32(((fp.wrapping_add(208)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)).wrapping_add((c.m.r32(((fp.wrapping_add(208)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32))).wrapping_add(((c.m.r32(((fp.wrapping_add(208)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)).wrapping_add((c.m.r32(((fp.wrapping_add(208)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)))).wrapping_shr((10i32) as u32);
                        { let v_ = rt::fadd(l_v_4, l_texV_8); let a_ = ((fp.wrapping_add(336)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                        if (((((l_i_10) == (2i32)) as i32)) != 0) {
                            { let v_ = F32(0x00000000); let a_ = ((fp.wrapping_add(96)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
                            { let v_ = F32(0x00000000); let a_ = ((fp.wrapping_add(96)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
                        }
                        l_abe_12 = 1i32;
                        l_tex0_13 = { let a_ = (c.m.r32((l_w_3).wrapping_add(232))).wrapping_add((((1i32) as u32)).wrapping_mul(16)); c.m.r64(a_) };
                        if ((((!(((((((((((rt::fle(F32(c.m.r32(((fp.wrapping_add(96)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(12))), F32(0x00000000)) as i32)) != 0) && (((rt::fle(F32(c.m.r32(((fp.wrapping_add(96)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(12))), F32(0x00000000)) as i32)) != 0)) as i32)) != 0) && (((rt::fle(F32(c.m.r32(((fp.wrapping_add(96)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(12))), F32(0x00000000)) as i32)) != 0)) as i32)) != 0)) as i32)) != 0) {
                            if (((((l_z_11) < (0i32)) as i32)) != 0) {
                                l_e_16 = ((c.m.r32(sym::gOtZ)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(8));
                            } else {
                                if (((((l_z_11) >= (4096i32)) as i32)) != 0) {
                                    l_e_16 = ((c.m.r32(sym::gOtZ)).wrapping_add((((4095i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(8));
                                } else {
                                    l_e_16 = ((c.m.r32(sym::gOtZ)).wrapping_add((((l_z_11) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(8));
                                }
                            }
                            l_q_14 = c.m.r32(sym::gOtCur);
                            { let v_ = (l_q_14).wrapping_add((((1i32) as u32)).wrapping_mul(64)); let a_ = sym::gOtCur; c.m.w32(a_, v_); };
                            { let v_ = ((536870915i32) as u32); let a_ = l_q_14; c.m.w32(a_, v_); };
                            { let v_ = ((1342177283i32) as u32); let a_ = (l_q_14).wrapping_add(12); c.m.w32(a_, v_); };
                            { let v_ = ((1152921504606879746i64) as u64); let a_ = (l_q_14).wrapping_add(16); c.m.w64(a_, v_); };
                            { let v_ = ((14i32) as u64); let a_ = (l_q_14).wrapping_add(24); c.m.w64(a_, v_); };
                            { let v_ = ((8i32) as u64); let a_ = (l_q_14).wrapping_add(40); c.m.w64(a_, v_); };
                            { let v_ = ((9i32) as u64); let a_ = (l_q_14).wrapping_add(56); c.m.w64(a_, v_); };
                            { let v_ = ((0i32) as u32); let a_ = (l_q_14).wrapping_add(4); c.m.w32(a_, v_); };
                            { let v_ = ((268435456i32) as u32); let a_ = (l_q_14).wrapping_add(8); c.m.w32(a_, v_); };
                            { let v_ = ((0i32) as u64); let a_ = (l_q_14).wrapping_add(32); c.m.w64(a_, v_); };
                            { let v_ = ((0i32) as u64); let a_ = (l_q_14).wrapping_add(48); c.m.w64(a_, v_); };
                            { let v_ = l_q_14; let a_ = (c.m.r32((l_e_16).wrapping_add(4))).wrapping_add(4); c.m.w32(a_, v_); };
                            { let v_ = l_q_14; let a_ = (l_e_16).wrapping_add(4); c.m.w32(a_, v_); };
                            l_p_15 = c.m.r32(sym::gOtCur);
                            { let v_ = (l_p_15).wrapping_add((((1i32) as u32)).wrapping_mul(144)); let a_ = sym::gOtCur; c.m.w32(a_, v_); };
                            { let v_ = ((268435456i32) as u32); let a_ = (l_p_15).wrapping_add(8); c.m.w32(a_, v_); };
                            { let v_ = (((((l_abe_12) as u64)).wrapping_shl((6i32) as u32)) | (((28i32) as u64))); let a_ = (l_p_15).wrapping_add(32); c.m.w64(a_, v_); };
                            { let v_ = ((536870920i32) as u32); let a_ = l_p_15; c.m.w32(a_, v_); };
                            { let v_ = ((1342177288i32) as u32); let a_ = (l_p_15).wrapping_add(12); c.m.w32(a_, v_); };
                            { let v_ = 16429131440647602177u64; let a_ = (l_p_15).wrapping_add(16); c.m.w64(a_, v_); };
                            { let v_ = ((18599481537995104i64) as u64); let a_ = (l_p_15).wrapping_add(24); c.m.w64(a_, v_); };
                            { let v_ = ((0i32) as u32); let a_ = (l_p_15).wrapping_add(4); c.m.w32(a_, v_); };
                            { let v_ = ((rt::f2i(F32(c.m.r32((fp.wrapping_add(96)).wrapping_add((((0i32) as u32)).wrapping_mul(16)))))) as u8); let a_ = (l_p_15).wrapping_add(48); c.m.w8(a_, v_); };
                            { let v_ = ((rt::f2i(F32(c.m.r32(((fp.wrapping_add(96)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4))))) as u8); let a_ = ((l_p_15).wrapping_add(48)).wrapping_add(1); c.m.w8(a_, v_); };
                            { let v_ = ((rt::f2i(F32(c.m.r32(((fp.wrapping_add(96)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8))))) as u8); let a_ = ((l_p_15).wrapping_add(48)).wrapping_add(2); c.m.w8(a_, v_); };
                            { let v_ = ((rt::f2i(F32(c.m.r32(((fp.wrapping_add(96)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(12))))) as u8); let a_ = ((l_p_15).wrapping_add(48)).wrapping_add(3); c.m.w8(a_, v_); };
                            { let v_ = F32(c.m.r32(((fp.wrapping_add(336)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8))); let a_ = ((l_p_15).wrapping_add(48)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                            { let v_ = ((rt::f2i(F32(c.m.r32((fp.wrapping_add(96)).wrapping_add((((1i32) as u32)).wrapping_mul(16)))))) as u8); let a_ = (l_p_15).wrapping_add(72); c.m.w8(a_, v_); };
                            { let v_ = ((rt::f2i(F32(c.m.r32(((fp.wrapping_add(96)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(4))))) as u8); let a_ = ((l_p_15).wrapping_add(72)).wrapping_add(1); c.m.w8(a_, v_); };
                            { let v_ = ((rt::f2i(F32(c.m.r32(((fp.wrapping_add(96)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8))))) as u8); let a_ = ((l_p_15).wrapping_add(72)).wrapping_add(2); c.m.w8(a_, v_); };
                            { let v_ = ((rt::f2i(F32(c.m.r32(((fp.wrapping_add(96)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(12))))) as u8); let a_ = ((l_p_15).wrapping_add(72)).wrapping_add(3); c.m.w8(a_, v_); };
                            { let v_ = F32(c.m.r32(((fp.wrapping_add(336)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8))); let a_ = ((l_p_15).wrapping_add(72)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                            { let v_ = ((rt::f2i(F32(c.m.r32((fp.wrapping_add(96)).wrapping_add((((2i32) as u32)).wrapping_mul(16)))))) as u8); let a_ = (l_p_15).wrapping_add(96); c.m.w8(a_, v_); };
                            { let v_ = ((rt::f2i(F32(c.m.r32(((fp.wrapping_add(96)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(4))))) as u8); let a_ = ((l_p_15).wrapping_add(96)).wrapping_add(1); c.m.w8(a_, v_); };
                            { let v_ = ((rt::f2i(F32(c.m.r32(((fp.wrapping_add(96)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8))))) as u8); let a_ = ((l_p_15).wrapping_add(96)).wrapping_add(2); c.m.w8(a_, v_); };
                            { let v_ = ((rt::f2i(F32(c.m.r32(((fp.wrapping_add(96)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(12))))) as u8); let a_ = ((l_p_15).wrapping_add(96)).wrapping_add(3); c.m.w8(a_, v_); };
                            { let v_ = F32(c.m.r32(((fp.wrapping_add(336)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8))); let a_ = ((l_p_15).wrapping_add(96)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                            { let v_ = ((rt::f2i(F32(c.m.r32((fp.wrapping_add(96)).wrapping_add((((3i32) as u32)).wrapping_mul(16)))))) as u8); let a_ = (l_p_15).wrapping_add(120); c.m.w8(a_, v_); };
                            { let v_ = ((rt::f2i(F32(c.m.r32(((fp.wrapping_add(96)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(4))))) as u8); let a_ = ((l_p_15).wrapping_add(120)).wrapping_add(1); c.m.w8(a_, v_); };
                            { let v_ = ((rt::f2i(F32(c.m.r32(((fp.wrapping_add(96)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(8))))) as u8); let a_ = ((l_p_15).wrapping_add(120)).wrapping_add(2); c.m.w8(a_, v_); };
                            { let v_ = ((rt::f2i(F32(c.m.r32(((fp.wrapping_add(96)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(12))))) as u8); let a_ = ((l_p_15).wrapping_add(120)).wrapping_add(3); c.m.w8(a_, v_); };
                            { let v_ = l_tex0_13; let a_ = (l_p_15).wrapping_add(40); c.m.w64(a_, v_); };
                            { let v_ = F32(c.m.r32(((fp.wrapping_add(336)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(8))); let a_ = ((l_p_15).wrapping_add(120)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                            { let v_ = F32(c.m.r32((fp.wrapping_add(336)).wrapping_add((((0i32) as u32)).wrapping_mul(16)))); let a_ = (l_p_15).wrapping_add(56); c.m.w32(a_, (v_).0); };
                            { let v_ = F32(c.m.r32(((fp.wrapping_add(336)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4))); let a_ = ((l_p_15).wrapping_add(56)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                            { let v_ = F32(c.m.r32((fp.wrapping_add(336)).wrapping_add((((1i32) as u32)).wrapping_mul(16)))); let a_ = (l_p_15).wrapping_add(80); c.m.w32(a_, (v_).0); };
                            { let v_ = F32(c.m.r32(((fp.wrapping_add(336)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(4))); let a_ = ((l_p_15).wrapping_add(80)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                            { let v_ = F32(c.m.r32((fp.wrapping_add(336)).wrapping_add((((2i32) as u32)).wrapping_mul(16)))); let a_ = (l_p_15).wrapping_add(104); c.m.w32(a_, (v_).0); };
                            { let v_ = F32(c.m.r32(((fp.wrapping_add(336)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(4))); let a_ = ((l_p_15).wrapping_add(104)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                            { let v_ = F32(c.m.r32((fp.wrapping_add(336)).wrapping_add((((3i32) as u32)).wrapping_mul(16)))); let a_ = (l_p_15).wrapping_add(128); c.m.w32(a_, (v_).0); };
                            { let v_ = F32(c.m.r32(((fp.wrapping_add(336)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(4))); let a_ = ((l_p_15).wrapping_add(128)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                            { let v_ = (((c.m.r32((fp.wrapping_add(208)).wrapping_add((((0i32) as u32)).wrapping_mul(16))) as i32)) as u64); let a_ = (l_p_15).wrapping_add(64); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                            { let v_ = (((c.m.r32(((fp.wrapping_add(208)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4)) as i32)) as u64); let a_ = ((l_p_15).wrapping_add(64)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                            { let v_ = (((c.m.r32(((fp.wrapping_add(208)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)) as u64); let a_ = ((l_p_15).wrapping_add(64)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
                            { let v_ = ((255i32) as u64); let a_ = ((l_p_15).wrapping_add(64)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
                            { let v_ = (((c.m.r32((fp.wrapping_add(208)).wrapping_add((((1i32) as u32)).wrapping_mul(16))) as i32)) as u64); let a_ = (l_p_15).wrapping_add(88); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                            { let v_ = (((c.m.r32(((fp.wrapping_add(208)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(4)) as i32)) as u64); let a_ = ((l_p_15).wrapping_add(88)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                            { let v_ = (((c.m.r32(((fp.wrapping_add(208)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)) as u64); let a_ = ((l_p_15).wrapping_add(88)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
                            { let v_ = ((255i32) as u64); let a_ = ((l_p_15).wrapping_add(88)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
                            { let v_ = (((c.m.r32((fp.wrapping_add(208)).wrapping_add((((2i32) as u32)).wrapping_mul(16))) as i32)) as u64); let a_ = (l_p_15).wrapping_add(112); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                            { let v_ = (((c.m.r32(((fp.wrapping_add(208)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(4)) as i32)) as u64); let a_ = ((l_p_15).wrapping_add(112)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                            { let v_ = (((c.m.r32(((fp.wrapping_add(208)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)) as u64); let a_ = ((l_p_15).wrapping_add(112)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
                            { let v_ = ((255i32) as u64); let a_ = ((l_p_15).wrapping_add(112)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
                            { let v_ = (((c.m.r32((fp.wrapping_add(208)).wrapping_add((((3i32) as u32)).wrapping_mul(16))) as i32)) as u64); let a_ = (l_p_15).wrapping_add(136); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                            { let v_ = (((c.m.r32(((fp.wrapping_add(208)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(4)) as i32)) as u64); let a_ = ((l_p_15).wrapping_add(136)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                            { let v_ = (((c.m.r32(((fp.wrapping_add(208)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)) as u64); let a_ = ((l_p_15).wrapping_add(136)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
                            { let v_ = ((255i32) as u64); let a_ = ((l_p_15).wrapping_add(136)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
                            if (((((l_z_11) < (0i32)) as i32)) != 0) {
                                l_e_16 = ((c.m.r32(sym::gOtZ)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(8));
                            } else {
                                if (((((l_z_11) >= (4096i32)) as i32)) != 0) {
                                    l_e_16 = ((c.m.r32(sym::gOtZ)).wrapping_add((((4095i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(8));
                                } else {
                                    l_e_16 = ((c.m.r32(sym::gOtZ)).wrapping_add((((l_z_11) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(8));
                                }
                            }
                            { let v_ = l_p_15; let a_ = (c.m.r32((l_e_16).wrapping_add(4))).wrapping_add(4); c.m.w32(a_, v_); };
                            { let v_ = l_p_15; let a_ = (l_e_16).wrapping_add(4); c.m.w32(a_, v_); };
                        }
                        { let r_ = l_dv_5; let n_ = rt::fadd(l_vPrev_6, r_); l_vPrev_6 = n_; };
                        { let r_ = l_dv_5; let n_ = rt::fadd(l_v_4, r_); l_v_4 = n_; };
                        if (((((l_i_10) >= (2i32)) as i32)) != 0) {
                            l_v_4 = F32(0x3f800000);
                        }
                    }
                }
                { let o_ = l_i_10; let n_ = (o_).wrapping_add(1); l_i_10 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftGeyser_DrawColumn", &PROBE_N, &[], true); }
    r_
}
