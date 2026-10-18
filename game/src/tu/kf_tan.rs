#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn __kernel_tanf(c: &mut Ctx, mut p_x_1: F32, mut p_y_2: F32, mut p_iy_3: i32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "__kernel_tanf", &PROBE_N, &[(p_x_1).0, (p_y_2).0, (p_iy_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_z_4: F32 = F32(0);
    let mut l_r_5: F32 = F32(0);
    let mut l_v_6: F32 = F32(0);
    let mut l_w_7: F32 = F32(0);
    let mut l_s_8: F32 = F32(0);
    let mut l_ix_9: i32 = 0;
    let mut l_hx_10: i32 = 0;
    let mut l_a_11: F32 = F32(0);
    let mut l_t_12: F32 = F32(0);
    let mut l_i_13: i32 = 0;
    let r_ = 'f: {
        'l1: loop {
            'c2: {
                { let v_ = p_x_1; let a_ = fp; c.m.w32(a_, (v_).0); };
                l_hx_10 = ((c.m.r32(fp)) as i32);
            }
            if !((0i32) != 0) { break 'l1; }
        }
        l_ix_9 = ((l_hx_10) & (2147483647i32));
        if (((((l_ix_9) < (830472192i32)) as i32)) != 0) {
            if (((((rt::f2i(p_x_1)) == (0i32)) as i32)) != 0) {
                if (((((((l_ix_9) | ((p_iy_3).wrapping_add(1i32)))) == (0i32)) as i32)) != 0) {
                    break 'f rt::fdiv(F32(0x3f800000), F::Port_fabsf(c, p_x_1));
                } else {
                    break 'f (if (((((p_iy_3) == (1i32)) as i32)) != 0) { p_x_1 } else { rt::fdiv(F32(0xbf800000), p_x_1) });
                }
            }
        }
        if (((((l_ix_9) >= (1059889472i32)) as i32)) != 0) {
            if (((((l_hx_10) < (0i32)) as i32)) != 0) {
                p_x_1 = rt::fneg(p_x_1);
                p_y_2 = rt::fneg(p_y_2);
            }
            l_z_4 = rt::fsub(F32(0x3f490fda), p_x_1);
            l_w_7 = rt::fsub(F32(0x33222167), p_y_2);
            p_x_1 = rt::fadd(l_z_4, l_w_7);
            p_y_2 = F32(0x00000000);
        }
        l_z_4 = rt::fmul(p_x_1, p_x_1);
        l_w_7 = rt::fmul(l_z_4, l_z_4);
        l_r_5 = rt::fadd(F32(0x3e088889), rt::fmul(l_w_7, rt::fadd(F32(0x3cb327a3), rt::fmul(l_w_7, rt::fadd(F32(0x3b6b6916), rt::fmul(l_w_7, rt::fadd(F32(0x3a1a26c7), rt::fmul(l_w_7, rt::fadd(F32(0x38a3f445), rt::fmul(l_w_7, F32(0xb79bae5e)))))))))));
        l_v_6 = rt::fmul(l_z_4, rt::fadd(F32(0x3d5d0dd1), rt::fmul(l_w_7, rt::fadd(F32(0x3c11371e), rt::fmul(l_w_7, rt::fadd(F32(0x3abede47), rt::fmul(l_w_7, rt::fadd(F32(0x398137b8), rt::fmul(l_w_7, rt::fadd(F32(0x3895c07a), rt::fmul(l_w_7, F32(0x37d95384))))))))))));
        l_s_8 = rt::fmul(l_z_4, p_x_1);
        l_r_5 = rt::fadd(p_y_2, rt::fmul(l_z_4, rt::fadd(rt::fmul(l_s_8, rt::fadd(l_r_5, l_v_6)), p_y_2)));
        { let r_ = rt::fmul(F32(0x3eaaaaab), l_s_8); let n_ = rt::fadd(l_r_5, r_); l_r_5 = n_; };
        l_w_7 = rt::fadd(p_x_1, l_r_5);
        if (((((l_ix_9) >= (1059889472i32)) as i32)) != 0) {
            l_v_6 = rt::i2f(p_iy_3);
            break 'f rt::fmul(rt::i2f((1i32).wrapping_sub((((l_hx_10).wrapping_shr((30i32) as u32)) & (2i32)))), rt::fsub(l_v_6, rt::fmul(F32(0x40000000), rt::fsub(p_x_1, rt::fsub(rt::fdiv(rt::fmul(l_w_7, l_w_7), rt::fadd(l_w_7, l_v_6)), l_r_5)))));
        }
        if (((((p_iy_3) == (1i32)) as i32)) != 0) {
            break 'f l_w_7;
        } else {
            l_z_4 = l_w_7;
            'l3: loop {
                'c4: {
                    { let v_ = l_z_4; let a_ = fp.wrapping_add(4); c.m.w32(a_, (v_).0); };
                    l_i_13 = ((c.m.r32(fp.wrapping_add(4))) as i32);
                }
                if !((0i32) != 0) { break 'l3; }
            }
            'l5: loop {
                'c6: {
                    { let v_ = ((((l_i_13) as u32)) & (4294963200u32)); let a_ = fp.wrapping_add(8); c.m.w32(a_, v_); };
                    l_z_4 = F32(c.m.r32(fp.wrapping_add(8)));
                }
                if !((0i32) != 0) { break 'l5; }
            }
            l_v_6 = rt::fsub(l_r_5, rt::fsub(l_z_4, p_x_1));
            l_t_12 = { l_a_11 = rt::fdiv(F32(0xbf800000), l_w_7); l_a_11 };
            'l7: loop {
                'c8: {
                    { let v_ = l_t_12; let a_ = fp.wrapping_add(12); c.m.w32(a_, (v_).0); };
                    l_i_13 = ((c.m.r32(fp.wrapping_add(12))) as i32);
                }
                if !((0i32) != 0) { break 'l7; }
            }
            'l9: loop {
                'c10: {
                    { let v_ = ((((l_i_13) as u32)) & (4294963200u32)); let a_ = fp.wrapping_add(16); c.m.w32(a_, v_); };
                    l_t_12 = F32(c.m.r32(fp.wrapping_add(16)));
                }
                if !((0i32) != 0) { break 'l9; }
            }
            l_s_8 = rt::fadd(F32(0x3f800000), rt::fmul(l_t_12, l_z_4));
            break 'f rt::fadd(l_t_12, rt::fmul(l_a_11, rt::fadd(l_s_8, rt::fmul(l_t_12, l_v_6))));
        }
       F32(0)
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "__kernel_tanf", &PROBE_N, &[], true); }
    r_
}
