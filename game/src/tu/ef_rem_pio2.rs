#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn __ieee754_rem_pio2f(c: &mut Ctx, mut p_x_1: F32, mut p_y_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "__ieee754_rem_pio2f", &PROBE_N, &[(p_x_1).0, (p_y_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_z_3: F32 = F32(0);
    let mut l_w_4: F32 = F32(0);
    let mut l_t_5: F32 = F32(0);
    let mut l_r_6: F32 = F32(0);
    let mut l_fn_7: F32 = F32(0);
    let mut l_i_8: i32 = 0;
    let mut l_j_9: i32 = 0;
    let mut l_n_10: i32 = 0;
    let mut l_ix_11: i32 = 0;
    let mut l_hx_12: i32 = 0;
    let mut l_e0_13: i32 = 0;
    let mut l_nx_14: i32 = 0;
    let mut l_high_15: u32 = 0;
    let r_ = 'f: {
        'l1: loop {
            'c2: {
                { let v_ = p_x_1; let a_ = fp.wrapping_add(12); c.m.w32(a_, (v_).0); };
                l_hx_12 = ((c.m.r32(fp.wrapping_add(12))) as i32);
            }
            if !((0i32) != 0) { break 'l1; }
        }
        l_ix_11 = ((l_hx_12) & (2147483647i32));
        if (((((l_ix_11) <= (1061752792i32)) as i32)) != 0) {
            { let v_ = p_x_1; let a_ = (p_y_2).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x00000000); let a_ = (p_y_2).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            break 'f 0i32;
        }
        if (((((l_ix_11) < (1075235812i32)) as i32)) != 0) {
            if (((((l_hx_12) > (0i32)) as i32)) != 0) {
                l_z_3 = rt::fsub(p_x_1, F32(0x3fc90f80));
                if (((((((((l_ix_11) as u32)) & (4294967280u32))) != (((1070141392i32) as u32))) as i32)) != 0) {
                    { let v_ = rt::fsub(l_z_3, F32(0x37354442)); let a_ = (p_y_2).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fsub(rt::fsub(l_z_3, F32(c.m.r32((p_y_2).wrapping_add((((0i32) as u32)).wrapping_mul(4))))), F32(0x37354442)); let a_ = (p_y_2).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                } else {
                    { let r_ = F32(0x373543ff); let n_ = rt::fsub(l_z_3, r_); l_z_3 = n_; };
                    { let v_ = rt::fsub(l_z_3, F32(0x2e85a308)); let a_ = (p_y_2).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fsub(rt::fsub(l_z_3, F32(c.m.r32((p_y_2).wrapping_add((((0i32) as u32)).wrapping_mul(4))))), F32(0x2e85a308)); let a_ = (p_y_2).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                }
                break 'f 1i32;
            } else {
                l_z_3 = rt::fadd(p_x_1, F32(0x3fc90f80));
                if (((((((((l_ix_11) as u32)) & (4294967280u32))) != (((1070141392i32) as u32))) as i32)) != 0) {
                    { let v_ = rt::fadd(l_z_3, F32(0x37354442)); let a_ = (p_y_2).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fadd(rt::fsub(l_z_3, F32(c.m.r32((p_y_2).wrapping_add((((0i32) as u32)).wrapping_mul(4))))), F32(0x37354442)); let a_ = (p_y_2).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                } else {
                    { let r_ = F32(0x373543ff); let n_ = rt::fadd(l_z_3, r_); l_z_3 = n_; };
                    { let v_ = rt::fadd(l_z_3, F32(0x2e85a308)); let a_ = (p_y_2).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fadd(rt::fsub(l_z_3, F32(c.m.r32((p_y_2).wrapping_add((((0i32) as u32)).wrapping_mul(4))))), F32(0x2e85a308)); let a_ = (p_y_2).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                }
                break 'f (1i32).wrapping_neg();
            }
        }
        if (((((l_ix_11) <= (1128861568i32)) as i32)) != 0) {
            l_t_5 = F::Port_fabsf(c, p_x_1);
            l_n_10 = rt::f2i(rt::fadd(rt::fmul(l_t_5, F32(0x3f22f984)), F32(0x3f000000)));
            l_fn_7 = rt::i2f(l_n_10);
            l_r_6 = rt::fsub(l_t_5, rt::fmul(l_fn_7, F32(0x3fc90f80)));
            l_w_4 = rt::fmul(l_fn_7, F32(0x37354442));
            if (((((((((l_n_10) < (32i32)) as i32)) != 0) && (((((((((l_ix_11) as u32)) & (4294967040u32))) != ((((c.m.r32((sym::ef_rem_pio2__npio2_hw).wrapping_add(((((l_n_10).wrapping_sub(1i32)) as u32)).wrapping_mul(4))) as i32)) as u32))) as i32)) != 0)) as i32)) != 0) {
                { let v_ = rt::fsub(l_r_6, l_w_4); let a_ = (p_y_2).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            } else {
                l_j_9 = (l_ix_11).wrapping_shr((23i32) as u32);
                { let v_ = rt::fsub(l_r_6, l_w_4); let a_ = (p_y_2).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                'l3: loop {
                    'c4: {
                        { let v_ = F32(c.m.r32((p_y_2).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = fp.wrapping_add(16); c.m.w32(a_, (v_).0); };
                        l_high_15 = c.m.r32(fp.wrapping_add(16));
                    }
                    if !((0i32) != 0) { break 'l3; }
                }
                l_i_8 = (((((l_j_9) as u32)).wrapping_sub((((l_high_15).wrapping_shr((23i32) as u32)) & (((255i32) as u32))))) as i32);
                if (((((l_i_8) > (8i32)) as i32)) != 0) {
                    l_t_5 = l_r_6;
                    l_w_4 = rt::fmul(l_fn_7, F32(0x373543ff));
                    l_r_6 = rt::fsub(l_t_5, l_w_4);
                    l_w_4 = rt::fsub(rt::fmul(l_fn_7, F32(0x2e85a308)), rt::fsub(rt::fsub(l_t_5, l_r_6), l_w_4));
                    { let v_ = rt::fsub(l_r_6, l_w_4); let a_ = (p_y_2).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    'l5: loop {
                        'c6: {
                            { let v_ = F32(c.m.r32((p_y_2).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = fp.wrapping_add(20); c.m.w32(a_, (v_).0); };
                            l_high_15 = c.m.r32(fp.wrapping_add(20));
                        }
                        if !((0i32) != 0) { break 'l5; }
                    }
                    l_i_8 = (((((l_j_9) as u32)).wrapping_sub((((l_high_15).wrapping_shr((23i32) as u32)) & (((255i32) as u32))))) as i32);
                    if (((((l_i_8) > (25i32)) as i32)) != 0) {
                        l_t_5 = l_r_6;
                        l_w_4 = rt::fmul(l_fn_7, F32(0x2e85a300));
                        l_r_6 = rt::fsub(l_t_5, l_w_4);
                        l_w_4 = rt::fsub(rt::fmul(l_fn_7, F32(0x248d3131)), rt::fsub(rt::fsub(l_t_5, l_r_6), l_w_4));
                        { let v_ = rt::fsub(l_r_6, l_w_4); let a_ = (p_y_2).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    }
                }
            }
            { let v_ = rt::fsub(rt::fsub(l_r_6, F32(c.m.r32((p_y_2).wrapping_add((((0i32) as u32)).wrapping_mul(4))))), l_w_4); let a_ = (p_y_2).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            if (((((l_hx_12) < (0i32)) as i32)) != 0) {
                { let v_ = rt::fneg(F32(c.m.r32((p_y_2).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a_ = (p_y_2).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fneg(F32(c.m.r32((p_y_2).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); let a_ = (p_y_2).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                break 'f (l_n_10).wrapping_neg();
            } else {
                break 'f l_n_10;
            }
        }
        if ((((!(((((l_ix_11) < (2139095040i32)) as i32)) != 0)) as i32)) != 0) {
            { let v_ = { let v_ = rt::fsub(p_x_1, p_x_1); let a_ = (p_y_2).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); v_ }; let a_ = (p_y_2).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            break 'f 0i32;
        }
        l_e0_13 = ((l_ix_11).wrapping_shr((23i32) as u32)).wrapping_sub(134i32);
        'l7: loop {
            'c8: {
                { let v_ = (((l_ix_11).wrapping_sub((l_e0_13).wrapping_shl((23i32) as u32))) as u32); let a_ = fp.wrapping_add(24); c.m.w32(a_, v_); };
                l_z_3 = F32(c.m.r32(fp.wrapping_add(24)));
            }
            if !((0i32) != 0) { break 'l7; }
        }
        {
            l_i_8 = 0i32;
            'l9: loop {
                if !(((((l_i_8) < (2i32)) as i32)) != 0) { break 'l9; }
                'c10: {
                    { let v_ = rt::i2f(rt::f2i(l_z_3)); let a_ = (fp).wrapping_add((((l_i_8) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    l_z_3 = rt::fmul(rt::fsub(l_z_3, F32(c.m.r32((fp).wrapping_add((((l_i_8) as u32)).wrapping_mul(4))))), F32(0x43800000));
                }
                { let o_ = l_i_8; let n_ = (o_).wrapping_add(1); l_i_8 = n_; };
            }
        }
        { let v_ = l_z_3; let a_ = (fp).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        l_nx_14 = 3i32;
        'l11: while (((rt::feq(F32(c.m.r32((fp).wrapping_add(((((l_nx_14).wrapping_sub(1i32)) as u32)).wrapping_mul(4)))), F32(0x00000000)) as i32)) != 0) {
            { let o_ = l_nx_14; let n_ = (o_).wrapping_sub(1); l_nx_14 = n_; };
        }
        l_n_10 = F::__kernel_rem_pio2f(c, fp, p_y_2, l_e0_13, l_nx_14, 2i32, sym::ef_rem_pio2__two_over_pi);
        if (((((l_hx_12) < (0i32)) as i32)) != 0) {
            { let v_ = rt::fneg(F32(c.m.r32((p_y_2).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a_ = (p_y_2).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            { let v_ = rt::fneg(F32(c.m.r32((p_y_2).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); let a_ = (p_y_2).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            break 'f (l_n_10).wrapping_neg();
        }
        break 'f l_n_10;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "__ieee754_rem_pio2f", &PROBE_N, &[], true); }
    r_
}
