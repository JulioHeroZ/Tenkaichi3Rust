#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn __ieee754_asinf(c: &mut Ctx, mut p_x_1: F32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "__ieee754_asinf", &PROBE_N, &[(p_x_1).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_t_2: F32 = F32(0);
    let mut l_w_3: F32 = F32(0);
    let mut l_p_4: F32 = F32(0);
    let mut l_q_5: F32 = F32(0);
    let mut l_c_6: F32 = F32(0);
    let mut l_r_7: F32 = F32(0);
    let mut l_s_8: F32 = F32(0);
    let mut l_hx_9: i32 = 0;
    let mut l_ix_10: i32 = 0;
    let mut l_iw_11: i32 = 0;
    let r_ = 'f: {
        'l1: loop {
            'c2: {
                { let v_ = p_x_1; let a_ = fp; c.m.w32(a_, (v_).0); };
                l_hx_9 = ((c.m.r32(fp)) as i32);
            }
            if !((0i32) != 0) { break 'l1; }
        }
        l_ix_10 = ((l_hx_9) & (2147483647i32));
        if (((((l_ix_10) == (1065353216i32)) as i32)) != 0) {
            break 'f rt::fadd(rt::fmul(p_x_1, F32(0x3fc90fdb)), rt::fmul(p_x_1, F32(0xb33bbd2e)));
        } else {
            if (((((l_ix_10) > (1065353216i32)) as i32)) != 0) {
                break 'f rt::fdiv(rt::fsub(p_x_1, p_x_1), rt::fsub(p_x_1, p_x_1));
            } else {
                if (((((l_ix_10) < (1056964608i32)) as i32)) != 0) {
                    if (((((l_ix_10) < (838860800i32)) as i32)) != 0) {
                        if (((rt::fgt(rt::fadd(F32(0x7149f2c9), p_x_1), F32(0x3f800000)) as i32)) != 0) {
                            break 'f p_x_1;
                        }
                    } else {
                        l_t_2 = rt::fmul(p_x_1, p_x_1);
                        l_p_4 = rt::fmul(l_t_2, rt::fadd(F32(0x3e2aaaaa), rt::fmul(l_t_2, rt::fadd(F32(0xbea6b08f), rt::fmul(l_t_2, rt::fadd(F32(0x3e4e0aa8), rt::fmul(l_t_2, rt::fadd(F32(0xbd241146), rt::fmul(l_t_2, rt::fadd(F32(0x3a4f7f04), rt::fmul(l_t_2, F32(0x3811ef08))))))))))));
                        l_q_5 = rt::fadd(F32(0x3f800000), rt::fmul(l_t_2, rt::fadd(F32(0xc019d138), rt::fmul(l_t_2, rt::fadd(F32(0x4001572c), rt::fmul(l_t_2, rt::fadd(F32(0xbf303360), rt::fmul(l_t_2, F32(0x3d9dc62d)))))))));
                        l_w_3 = rt::fdiv(l_p_4, l_q_5);
                        break 'f rt::fadd(p_x_1, rt::fmul(p_x_1, l_w_3));
                    }
                }
            }
        }
        l_w_3 = rt::fsub(F32(0x3f800000), F::Port_fabsf(c, p_x_1));
        l_t_2 = rt::fmul(l_w_3, F32(0x3f000000));
        l_p_4 = rt::fmul(l_t_2, rt::fadd(F32(0x3e2aaaaa), rt::fmul(l_t_2, rt::fadd(F32(0xbea6b08f), rt::fmul(l_t_2, rt::fadd(F32(0x3e4e0aa8), rt::fmul(l_t_2, rt::fadd(F32(0xbd241146), rt::fmul(l_t_2, rt::fadd(F32(0x3a4f7f04), rt::fmul(l_t_2, F32(0x3811ef08))))))))))));
        l_q_5 = rt::fadd(F32(0x3f800000), rt::fmul(l_t_2, rt::fadd(F32(0xc019d138), rt::fmul(l_t_2, rt::fadd(F32(0x4001572c), rt::fmul(l_t_2, rt::fadd(F32(0xbf303360), rt::fmul(l_t_2, F32(0x3d9dc62d)))))))));
        l_s_8 = F::__ieee754_sqrtf(c, l_t_2);
        if (((((l_ix_10) >= (1064933786i32)) as i32)) != 0) {
            l_w_3 = rt::fdiv(l_p_4, l_q_5);
            l_t_2 = rt::fsub(F32(0x3fc90fdb), rt::fsub(rt::fmul(F32(0x40000000), rt::fadd(l_s_8, rt::fmul(l_s_8, l_w_3))), F32(0xb33bbd2e)));
        } else {
            l_w_3 = l_s_8;
            'l3: loop {
                'c4: {
                    { let v_ = l_w_3; let a_ = fp.wrapping_add(4); c.m.w32(a_, (v_).0); };
                    l_iw_11 = ((c.m.r32(fp.wrapping_add(4))) as i32);
                }
                if !((0i32) != 0) { break 'l3; }
            }
            'l5: loop {
                'c6: {
                    { let v_ = ((((l_iw_11) as u32)) & (4294963200u32)); let a_ = fp.wrapping_add(8); c.m.w32(a_, v_); };
                    l_w_3 = F32(c.m.r32(fp.wrapping_add(8)));
                }
                if !((0i32) != 0) { break 'l5; }
            }
            l_c_6 = rt::fdiv(rt::fsub(l_t_2, rt::fmul(l_w_3, l_w_3)), rt::fadd(l_s_8, l_w_3));
            l_r_7 = rt::fdiv(l_p_4, l_q_5);
            l_p_4 = rt::fsub(rt::fmul(rt::fmul(F32(0x40000000), l_s_8), l_r_7), rt::fsub(F32(0xb33bbd2e), rt::fmul(F32(0x40000000), l_c_6)));
            l_q_5 = rt::fsub(F32(0x3f490fdb), rt::fmul(F32(0x40000000), l_w_3));
            l_t_2 = rt::fsub(F32(0x3f490fdb), rt::fsub(l_p_4, l_q_5));
        }
        if (((((l_hx_9) > (0i32)) as i32)) != 0) {
            break 'f l_t_2;
        } else {
            break 'f rt::fneg(l_t_2);
        }
       F32(0)
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "__ieee754_asinf", &PROBE_N, &[], true); }
    r_
}
