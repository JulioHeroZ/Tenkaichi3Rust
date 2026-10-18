#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn __ieee754_powf(c: &mut Ctx, mut p_x_1: F32, mut p_y_2: F32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "__ieee754_powf", &PROBE_N, &[(p_x_1).0, (p_y_2).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(96) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 96);
    let mut l_z_3: F32 = F32(0);
    let mut l_ax_4: F32 = F32(0);
    let mut l_z_h_5: F32 = F32(0);
    let mut l_z_l_6: F32 = F32(0);
    let mut l_p_h_7: F32 = F32(0);
    let mut l_p_l_8: F32 = F32(0);
    let mut l_y1_9: F32 = F32(0);
    let mut l_t1_10: F32 = F32(0);
    let mut l_t2_11: F32 = F32(0);
    let mut l_r_12: F32 = F32(0);
    let mut l_s_13: F32 = F32(0);
    let mut l_t_14: F32 = F32(0);
    let mut l_u_15: F32 = F32(0);
    let mut l_v_16: F32 = F32(0);
    let mut l_w_17: F32 = F32(0);
    let mut l_i_18: i32 = 0;
    let mut l_j_19: i32 = 0;
    let mut l_k_20: i32 = 0;
    let mut l_yisint_21: i32 = 0;
    let mut l_n_22: i32 = 0;
    let mut l_hx_23: i32 = 0;
    let mut l_hy_24: i32 = 0;
    let mut l_ix_25: i32 = 0;
    let mut l_iy_26: i32 = 0;
    let mut l_is_27: i32 = 0;
    let mut l_s2_28: F32 = F32(0);
    let mut l_s_h_29: F32 = F32(0);
    let mut l_s_l_30: F32 = F32(0);
    let mut l_t_h_31: F32 = F32(0);
    let mut l_t_l_32: F32 = F32(0);
    let r_ = 'f: {
        'l1: loop {
            'c2: {
                { let v_ = p_x_1; let a_ = fp; c.m.w32(a_, (v_).0); };
                l_hx_23 = ((c.m.r32(fp)) as i32);
            }
            if !((0i32) != 0) { break 'l1; }
        }
        'l3: loop {
            'c4: {
                { let v_ = p_y_2; let a_ = fp.wrapping_add(4); c.m.w32(a_, (v_).0); };
                l_hy_24 = ((c.m.r32(fp.wrapping_add(4))) as i32);
            }
            if !((0i32) != 0) { break 'l3; }
        }
        l_ix_25 = ((l_hx_23) & (2147483647i32));
        l_iy_26 = ((l_hy_24) & (2147483647i32));
        if (((((l_iy_26) == (0i32)) as i32)) != 0) {
            break 'f F32(0x3f800000);
        }
        if (((((((((l_ix_25) > (2139095040i32)) as i32)) != 0) || (((((l_iy_26) > (2139095040i32)) as i32)) != 0)) as i32)) != 0) {
            break 'f rt::fadd(p_x_1, p_y_2);
        }
        l_yisint_21 = 0i32;
        if (((((l_hx_23) < (0i32)) as i32)) != 0) {
            if (((((l_iy_26) >= (1266679808i32)) as i32)) != 0) {
                l_yisint_21 = 2i32;
            } else {
                if (((((l_iy_26) >= (1065353216i32)) as i32)) != 0) {
                    l_k_20 = ((l_iy_26).wrapping_shr((23i32) as u32)).wrapping_sub(127i32);
                    l_j_19 = (l_iy_26).wrapping_shr(((23i32).wrapping_sub(l_k_20)) as u32);
                    if ((((((l_j_19).wrapping_shl(((23i32).wrapping_sub(l_k_20)) as u32)) == (l_iy_26)) as i32)) != 0) {
                        l_yisint_21 = (2i32).wrapping_sub(((l_j_19) & (1i32)));
                    }
                }
            }
        }
        if (((((l_iy_26) == (2139095040i32)) as i32)) != 0) {
            if (((((l_ix_25) == (1065353216i32)) as i32)) != 0) {
                break 'f rt::fsub(p_y_2, p_y_2);
            } else {
                if (((((l_ix_25) > (1065353216i32)) as i32)) != 0) {
                    break 'f (if (((((l_hy_24) >= (0i32)) as i32)) != 0) { p_y_2 } else { F32(0x00000000) });
                } else {
                    break 'f (if (((((l_hy_24) < (0i32)) as i32)) != 0) { rt::fneg(p_y_2) } else { F32(0x00000000) });
                }
            }
        }
        if (((((l_iy_26) == (1065353216i32)) as i32)) != 0) {
            if (((((l_hy_24) < (0i32)) as i32)) != 0) {
                break 'f rt::fdiv(F32(0x3f800000), p_x_1);
            } else {
                break 'f p_x_1;
            }
        }
        if (((((l_hy_24) == (1073741824i32)) as i32)) != 0) {
            break 'f rt::fmul(p_x_1, p_x_1);
        }
        if (((((l_hy_24) == (1056964608i32)) as i32)) != 0) {
            if (((((l_hx_23) >= (0i32)) as i32)) != 0) {
                break 'f F::__ieee754_sqrtf(c, p_x_1);
            }
        }
        l_ax_4 = F::Port_fabsf(c, p_x_1);
        if (((((((((((((l_ix_25) == (2139095040i32)) as i32)) != 0) || (((((l_ix_25) == (0i32)) as i32)) != 0)) as i32)) != 0) || (((((l_ix_25) == (1065353216i32)) as i32)) != 0)) as i32)) != 0) {
            l_z_3 = l_ax_4;
            if (((((l_hy_24) < (0i32)) as i32)) != 0) {
                l_z_3 = rt::fdiv(F32(0x3f800000), l_z_3);
            }
            if (((((l_hx_23) < (0i32)) as i32)) != 0) {
                if ((((((((l_ix_25).wrapping_sub(1065353216i32)) | (l_yisint_21))) == (0i32)) as i32)) != 0) {
                    l_z_3 = rt::fdiv(rt::fsub(l_z_3, l_z_3), rt::fsub(l_z_3, l_z_3));
                } else {
                    if (((((l_yisint_21) == (1i32)) as i32)) != 0) {
                        l_z_3 = rt::fneg(l_z_3);
                    }
                }
            }
            break 'f l_z_3;
        }
        if (((((((((((l_hx_23) as u32)).wrapping_shr((31i32) as u32)).wrapping_sub(((1i32) as u32))) | (((l_yisint_21) as u32)))) == (((0i32) as u32))) as i32)) != 0) {
            break 'f rt::fdiv(rt::fsub(p_x_1, p_x_1), rt::fsub(p_x_1, p_x_1));
        }
        if (((((l_iy_26) > (1291845632i32)) as i32)) != 0) {
            if (((((l_ix_25) < (1065353208i32)) as i32)) != 0) {
                break 'f (if (((((l_hy_24) < (0i32)) as i32)) != 0) { rt::fmul(F32(0x7149f2ca), F32(0x7149f2ca)) } else { F32(0x00000000) });
            }
            if (((((l_ix_25) > (1065353223i32)) as i32)) != 0) {
                break 'f (if (((((l_hy_24) > (0i32)) as i32)) != 0) { rt::fmul(F32(0x7149f2ca), F32(0x7149f2ca)) } else { F32(0x00000000) });
            }
            l_t_14 = rt::fsub(p_x_1, F32(0x3f800000));
            l_w_17 = rt::fmul(rt::fmul(l_t_14, l_t_14), rt::fsub(F32(0x3f000000), rt::fmul(l_t_14, rt::fsub(F32(0x3eaaaaaa), rt::fmul(l_t_14, F32(0x3e800000))))));
            l_u_15 = rt::fmul(F32(0x3fb8aa00), l_t_14);
            l_v_16 = rt::fsub(rt::fmul(l_t_14, F32(0x36eca570)), rt::fmul(l_w_17, F32(0x3fb8aa3b)));
            l_t1_10 = rt::fadd(l_u_15, l_v_16);
            'l5: loop {
                'c6: {
                    { let v_ = l_t1_10; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_).0); };
                    l_is_27 = ((c.m.r32(fp.wrapping_add(8))) as i32);
                }
                if !((0i32) != 0) { break 'l5; }
            }
            'l7: loop {
                'c8: {
                    { let v_ = ((((l_is_27) as u32)) & (4294963200u32)); let a_ = fp.wrapping_add(12); c.m.w32(a_, v_); };
                    l_t1_10 = F32(c.m.r32(fp.wrapping_add(12)));
                }
                if !((0i32) != 0) { break 'l7; }
            }
            l_t2_11 = rt::fsub(l_v_16, rt::fsub(l_t1_10, l_u_15));
        } else {
            l_n_22 = 0i32;
            if (((((l_ix_25) < (8388608i32)) as i32)) != 0) {
                { let r_ = F32(0x4b800000); let n_ = rt::fmul(l_ax_4, r_); l_ax_4 = n_; };
                { let r_ = 24i32; let n_ = (l_n_22).wrapping_sub(r_); l_n_22 = n_; };
                'l9: loop {
                    'c10: {
                        { let v_ = l_ax_4; let a_ = fp.wrapping_add(16); c.m.w32(a_, (v_).0); };
                        l_ix_25 = ((c.m.r32(fp.wrapping_add(16))) as i32);
                    }
                    if !((0i32) != 0) { break 'l9; }
                }
            }
            { let r_ = ((l_ix_25).wrapping_shr((23i32) as u32)).wrapping_sub(127i32); let n_ = (l_n_22).wrapping_add(r_); l_n_22 = n_; };
            l_j_19 = ((l_ix_25) & (8388607i32));
            l_ix_25 = ((l_j_19) | (1065353216i32));
            if (((((l_j_19) <= (1885297i32)) as i32)) != 0) {
                l_k_20 = 0i32;
            } else {
                if (((((l_j_19) < (6140887i32)) as i32)) != 0) {
                    l_k_20 = 1i32;
                } else {
                    l_k_20 = 0i32;
                    { let r_ = 1i32; let n_ = (l_n_22).wrapping_add(r_); l_n_22 = n_; };
                    { let r_ = 8388608i32; let n_ = (l_ix_25).wrapping_sub(r_); l_ix_25 = n_; };
                }
            }
            'l11: loop {
                'c12: {
                    { let v_ = ((l_ix_25) as u32); let a_ = fp.wrapping_add(20); c.m.w32(a_, v_); };
                    l_ax_4 = F32(c.m.r32(fp.wrapping_add(20)));
                }
                if !((0i32) != 0) { break 'l11; }
            }
            l_u_15 = rt::fsub(l_ax_4, F32(c.m.r32((sym::ef_pow__bp).wrapping_add((((l_k_20) as u32)).wrapping_mul(4)))));
            l_v_16 = rt::fdiv(F32(0x3f800000), rt::fadd(l_ax_4, F32(c.m.r32((sym::ef_pow__bp).wrapping_add((((l_k_20) as u32)).wrapping_mul(4))))));
            l_s_13 = rt::fmul(l_u_15, l_v_16);
            l_s_h_29 = l_s_13;
            'l13: loop {
                'c14: {
                    { let v_ = l_s_h_29; let a_ = fp.wrapping_add(24); c.m.w32(a_, (v_).0); };
                    l_is_27 = ((c.m.r32(fp.wrapping_add(24))) as i32);
                }
                if !((0i32) != 0) { break 'l13; }
            }
            'l15: loop {
                'c16: {
                    { let v_ = ((((l_is_27) as u32)) & (4294963200u32)); let a_ = fp.wrapping_add(28); c.m.w32(a_, v_); };
                    l_s_h_29 = F32(c.m.r32(fp.wrapping_add(28)));
                }
                if !((0i32) != 0) { break 'l15; }
            }
            'l17: loop {
                'c18: {
                    { let v_ = (((((((l_ix_25).wrapping_shr((1i32) as u32)) | (536870912i32))).wrapping_add(262144i32)).wrapping_add((l_k_20).wrapping_shl((21i32) as u32))) as u32); let a_ = fp.wrapping_add(32); c.m.w32(a_, v_); };
                    l_t_h_31 = F32(c.m.r32(fp.wrapping_add(32)));
                }
                if !((0i32) != 0) { break 'l17; }
            }
            l_t_l_32 = rt::fsub(l_ax_4, rt::fsub(l_t_h_31, F32(c.m.r32((sym::ef_pow__bp).wrapping_add((((l_k_20) as u32)).wrapping_mul(4))))));
            l_s_l_30 = rt::fmul(l_v_16, rt::fsub(rt::fsub(l_u_15, rt::fmul(l_s_h_29, l_t_h_31)), rt::fmul(l_s_h_29, l_t_l_32)));
            l_s2_28 = rt::fmul(l_s_13, l_s_13);
            l_r_12 = rt::fmul(rt::fmul(l_s2_28, l_s2_28), rt::fadd(F32(0x3f19999a), rt::fmul(l_s2_28, rt::fadd(F32(0x3edb6db7), rt::fmul(l_s2_28, rt::fadd(F32(0x3eaaaaab), rt::fmul(l_s2_28, rt::fadd(F32(0x3e8ba305), rt::fmul(l_s2_28, rt::fadd(F32(0x3e6c3255), rt::fmul(l_s2_28, F32(0x3e53f142))))))))))));
            { let r_ = rt::fmul(l_s_l_30, rt::fadd(l_s_h_29, l_s_13)); let n_ = rt::fadd(l_r_12, r_); l_r_12 = n_; };
            l_s2_28 = rt::fmul(l_s_h_29, l_s_h_29);
            l_t_h_31 = rt::fadd(rt::fadd(F32(0x40400000), l_s2_28), l_r_12);
            'l19: loop {
                'c20: {
                    { let v_ = l_t_h_31; let a_ = fp.wrapping_add(36); c.m.w32(a_, (v_).0); };
                    l_is_27 = ((c.m.r32(fp.wrapping_add(36))) as i32);
                }
                if !((0i32) != 0) { break 'l19; }
            }
            'l21: loop {
                'c22: {
                    { let v_ = ((((l_is_27) as u32)) & (4294963200u32)); let a_ = fp.wrapping_add(40); c.m.w32(a_, v_); };
                    l_t_h_31 = F32(c.m.r32(fp.wrapping_add(40)));
                }
                if !((0i32) != 0) { break 'l21; }
            }
            l_t_l_32 = rt::fsub(l_r_12, rt::fsub(rt::fsub(l_t_h_31, F32(0x40400000)), l_s2_28));
            l_u_15 = rt::fmul(l_s_h_29, l_t_h_31);
            l_v_16 = rt::fadd(rt::fmul(l_s_l_30, l_t_h_31), rt::fmul(l_t_l_32, l_s_13));
            l_p_h_7 = rt::fadd(l_u_15, l_v_16);
            'l23: loop {
                'c24: {
                    { let v_ = l_p_h_7; let a_ = fp.wrapping_add(44); c.m.w32(a_, (v_).0); };
                    l_is_27 = ((c.m.r32(fp.wrapping_add(44))) as i32);
                }
                if !((0i32) != 0) { break 'l23; }
            }
            'l25: loop {
                'c26: {
                    { let v_ = ((((l_is_27) as u32)) & (4294963200u32)); let a_ = fp.wrapping_add(48); c.m.w32(a_, v_); };
                    l_p_h_7 = F32(c.m.r32(fp.wrapping_add(48)));
                }
                if !((0i32) != 0) { break 'l25; }
            }
            l_p_l_8 = rt::fsub(l_v_16, rt::fsub(l_p_h_7, l_u_15));
            l_z_h_5 = rt::fmul(F32(0x3f763800), l_p_h_7);
            l_z_l_6 = rt::fadd(rt::fadd(rt::fmul(F32(0x369dc3a0), l_p_h_7), rt::fmul(l_p_l_8, F32(0x3f76384f))), F32(c.m.r32((sym::ef_pow__dp_l).wrapping_add((((l_k_20) as u32)).wrapping_mul(4)))));
            l_t_14 = rt::i2f(l_n_22);
            l_t1_10 = rt::fadd(rt::fadd(rt::fadd(l_z_h_5, l_z_l_6), F32(c.m.r32((sym::ef_pow__dp_h).wrapping_add((((l_k_20) as u32)).wrapping_mul(4))))), l_t_14);
            'l27: loop {
                'c28: {
                    { let v_ = l_t1_10; let a_ = fp.wrapping_add(52); c.m.w32(a_, (v_).0); };
                    l_is_27 = ((c.m.r32(fp.wrapping_add(52))) as i32);
                }
                if !((0i32) != 0) { break 'l27; }
            }
            'l29: loop {
                'c30: {
                    { let v_ = ((((l_is_27) as u32)) & (4294963200u32)); let a_ = fp.wrapping_add(56); c.m.w32(a_, v_); };
                    l_t1_10 = F32(c.m.r32(fp.wrapping_add(56)));
                }
                if !((0i32) != 0) { break 'l29; }
            }
            l_t2_11 = rt::fsub(l_z_l_6, rt::fsub(rt::fsub(rt::fsub(l_t1_10, l_t_14), F32(c.m.r32((sym::ef_pow__dp_h).wrapping_add((((l_k_20) as u32)).wrapping_mul(4))))), l_z_h_5));
        }
        l_s_13 = F32(0x3f800000);
        if (((((((((((l_hx_23) as u32)).wrapping_shr((31i32) as u32)).wrapping_sub(((1i32) as u32))) | ((((l_yisint_21).wrapping_sub(1i32)) as u32)))) == (((0i32) as u32))) as i32)) != 0) {
            l_s_13 = F32(0xbf800000);
        }
        'l31: loop {
            'c32: {
                { let v_ = p_y_2; let a_ = fp.wrapping_add(60); c.m.w32(a_, (v_).0); };
                l_is_27 = ((c.m.r32(fp.wrapping_add(60))) as i32);
            }
            if !((0i32) != 0) { break 'l31; }
        }
        'l33: loop {
            'c34: {
                { let v_ = ((((l_is_27) as u32)) & (4294963200u32)); let a_ = fp.wrapping_add(64); c.m.w32(a_, v_); };
                l_y1_9 = F32(c.m.r32(fp.wrapping_add(64)));
            }
            if !((0i32) != 0) { break 'l33; }
        }
        l_p_l_8 = rt::fadd(rt::fmul(rt::fsub(p_y_2, l_y1_9), l_t1_10), rt::fmul(p_y_2, l_t2_11));
        l_p_h_7 = rt::fmul(l_y1_9, l_t1_10);
        l_z_3 = rt::fadd(l_p_l_8, l_p_h_7);
        'l35: loop {
            'c36: {
                { let v_ = l_z_3; let a_ = fp.wrapping_add(68); c.m.w32(a_, (v_).0); };
                l_j_19 = ((c.m.r32(fp.wrapping_add(68))) as i32);
            }
            if !((0i32) != 0) { break 'l35; }
        }
        l_i_18 = ((l_j_19) & (2147483647i32));
        if (((((l_j_19) > (0i32)) as i32)) != 0) {
            if (((((l_i_18) > (1124073472i32)) as i32)) != 0) {
                break 'f rt::fmul(rt::fmul(l_s_13, F32(0x7149f2ca)), F32(0x7149f2ca));
            } else {
                if (((((l_i_18) == (1124073472i32)) as i32)) != 0) {
                    if (((rt::fgt(rt::fadd(l_p_l_8, F32(0x3338aa3c)), rt::fsub(l_z_3, l_p_h_7)) as i32)) != 0) {
                        break 'f rt::fmul(rt::fmul(l_s_13, F32(0x7149f2ca)), F32(0x7149f2ca));
                    }
                }
            }
        } else {
            if (((((l_i_18) > (1125515264i32)) as i32)) != 0) {
                break 'f rt::fmul(rt::fmul(l_s_13, F32(0x0da24260)), F32(0x0da24260));
            } else {
                if (((((l_i_18) == (1125515264i32)) as i32)) != 0) {
                    if (((rt::fle(l_p_l_8, rt::fsub(l_z_3, l_p_h_7)) as i32)) != 0) {
                        break 'f rt::fmul(rt::fmul(l_s_13, F32(0x0da24260)), F32(0x0da24260));
                    }
                }
            }
        }
        l_k_20 = ((l_i_18).wrapping_shr((23i32) as u32)).wrapping_sub(127i32);
        l_n_22 = 0i32;
        if (((((l_i_18) > (1056964608i32)) as i32)) != 0) {
            l_n_22 = (l_j_19).wrapping_add((8388608i32).wrapping_shr(((l_k_20).wrapping_add(1i32)) as u32));
            l_k_20 = ((((l_n_22) & (2147483647i32))).wrapping_shr((23i32) as u32)).wrapping_sub(127i32);
            'l37: loop {
                'c38: {
                    { let v_ = ((((l_n_22) & ((!((8388607i32).wrapping_shr((l_k_20) as u32)))))) as u32); let a_ = fp.wrapping_add(72); c.m.w32(a_, v_); };
                    l_t_14 = F32(c.m.r32(fp.wrapping_add(72)));
                }
                if !((0i32) != 0) { break 'l37; }
            }
            l_n_22 = (((((l_n_22) & (8388607i32))) | (8388608i32))).wrapping_shr(((23i32).wrapping_sub(l_k_20)) as u32);
            if (((((l_j_19) < (0i32)) as i32)) != 0) {
                l_n_22 = (l_n_22).wrapping_neg();
            }
            { let r_ = l_t_14; let n_ = rt::fsub(l_p_h_7, r_); l_p_h_7 = n_; };
        }
        l_t_14 = rt::fadd(l_p_l_8, l_p_h_7);
        'l39: loop {
            'c40: {
                { let v_ = l_t_14; let a_ = fp.wrapping_add(76); c.m.w32(a_, (v_).0); };
                l_is_27 = ((c.m.r32(fp.wrapping_add(76))) as i32);
            }
            if !((0i32) != 0) { break 'l39; }
        }
        'l41: loop {
            'c42: {
                { let v_ = ((((l_is_27) as u32)) & (4294963200u32)); let a_ = fp.wrapping_add(80); c.m.w32(a_, v_); };
                l_t_14 = F32(c.m.r32(fp.wrapping_add(80)));
            }
            if !((0i32) != 0) { break 'l41; }
        }
        l_u_15 = rt::fmul(l_t_14, F32(0x3f317200));
        l_v_16 = rt::fadd(rt::fmul(rt::fsub(l_p_l_8, rt::fsub(l_t_14, l_p_h_7)), F32(0x3f317218)), rt::fmul(l_t_14, F32(0x35bfbe8c)));
        l_z_3 = rt::fadd(l_u_15, l_v_16);
        l_w_17 = rt::fsub(l_v_16, rt::fsub(l_z_3, l_u_15));
        l_t_14 = rt::fmul(l_z_3, l_z_3);
        l_t1_10 = rt::fsub(l_z_3, rt::fmul(l_t_14, rt::fadd(F32(0x3e2aaaab), rt::fmul(l_t_14, rt::fadd(F32(0xbb360b61), rt::fmul(l_t_14, rt::fadd(F32(0x388ab355), rt::fmul(l_t_14, rt::fadd(F32(0xb5ddea0e), rt::fmul(l_t_14, F32(0x3331bb4c)))))))))));
        l_r_12 = rt::fsub(rt::fdiv(rt::fmul(l_z_3, l_t1_10), rt::fsub(l_t1_10, F32(0x40000000))), rt::fadd(l_w_17, rt::fmul(l_z_3, l_w_17)));
        l_z_3 = rt::fsub(F32(0x3f800000), rt::fsub(l_r_12, l_z_3));
        'l43: loop {
            'c44: {
                { let v_ = l_z_3; let a_ = fp.wrapping_add(84); c.m.w32(a_, (v_).0); };
                l_j_19 = ((c.m.r32(fp.wrapping_add(84))) as i32);
            }
            if !((0i32) != 0) { break 'l43; }
        }
        { let r_ = (l_n_22).wrapping_shl((23i32) as u32); let n_ = (l_j_19).wrapping_add(r_); l_j_19 = n_; };
        if ((((((l_j_19).wrapping_shr((23i32) as u32)) <= (0i32)) as i32)) != 0) {
            l_z_3 = F::Port_scalbnf(c, l_z_3, l_n_22);
        } else {
            'l45: loop {
                'c46: {
                    { let v_ = ((l_j_19) as u32); let a_ = fp.wrapping_add(88); c.m.w32(a_, v_); };
                    l_z_3 = F32(c.m.r32(fp.wrapping_add(88)));
                }
                if !((0i32) != 0) { break 'l45; }
            }
        }
        break 'f rt::fmul(l_s_13, l_z_3);
       F32(0)
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "__ieee754_powf", &PROBE_N, &[], true); }
    r_
}
