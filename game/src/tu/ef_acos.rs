#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn __ieee754_acosf(c: &mut Ctx, mut p_x_1: F32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "__ieee754_acosf", &PROBE_N, &[(p_x_1).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_z_2: F32 = F32(0);
    let mut l_p_3: F32 = F32(0);
    let mut l_q_4: F32 = F32(0);
    let mut l_r_5: F32 = F32(0);
    let mut l_w_6: F32 = F32(0);
    let mut l_s_7: F32 = F32(0);
    let mut l_c_8: F32 = F32(0);
    let mut l_df_9: F32 = F32(0);
    let mut l_hx_10: i32 = 0;
    let mut l_ix_11: i32 = 0;
    let mut l_idf_12: i32 = 0;
    let r_ = 'f: {
        'l1: loop {
            'c2: {
                { let v_ = p_x_1; let a_ = fp; c.m.w32(a_, (v_).0); };
                l_hx_10 = ((c.m.r32(fp)) as i32);
            }
            if !((0i32) != 0) { break 'l1; }
        }
        l_ix_11 = ((l_hx_10) & (2147483647i32));
        if (((((l_ix_11) == (1065353216i32)) as i32)) != 0) {
            if (((((l_hx_10) > (0i32)) as i32)) != 0) {
                break 'f F32(0x00000000);
            } else {
                break 'f F32(0x40490fdb);
            }
        } else {
            if (((((l_ix_11) > (1065353216i32)) as i32)) != 0) {
                break 'f rt::fdiv(rt::fsub(p_x_1, p_x_1), rt::fsub(p_x_1, p_x_1));
            }
        }
        if (((((l_ix_11) < (1056964608i32)) as i32)) != 0) {
            if (((((l_ix_11) <= (587202560i32)) as i32)) != 0) {
                break 'f F32(0x3fc90fdb);
            }
            l_z_2 = rt::fmul(p_x_1, p_x_1);
            l_p_3 = rt::fmul(l_z_2, rt::fadd(F32(0x3e2aaaaa), rt::fmul(l_z_2, rt::fadd(F32(0xbea6b08f), rt::fmul(l_z_2, rt::fadd(F32(0x3e4e0aa8), rt::fmul(l_z_2, rt::fadd(F32(0xbd241146), rt::fmul(l_z_2, rt::fadd(F32(0x3a4f7f04), rt::fmul(l_z_2, F32(0x3811ef08))))))))))));
            l_q_4 = rt::fadd(F32(0x3f800000), rt::fmul(l_z_2, rt::fadd(F32(0xc019d138), rt::fmul(l_z_2, rt::fadd(F32(0x4001572c), rt::fmul(l_z_2, rt::fadd(F32(0xbf303360), rt::fmul(l_z_2, F32(0x3d9dc62d)))))))));
            l_r_5 = rt::fdiv(l_p_3, l_q_4);
            break 'f rt::fsub(F32(0x3fc90fda), rt::fsub(p_x_1, rt::fsub(F32(0x33a22168), rt::fmul(p_x_1, l_r_5))));
        } else {
            if (((((l_hx_10) < (0i32)) as i32)) != 0) {
                l_z_2 = rt::fmul(rt::fadd(F32(0x3f800000), p_x_1), F32(0x3f000000));
                l_p_3 = rt::fmul(l_z_2, rt::fadd(F32(0x3e2aaaaa), rt::fmul(l_z_2, rt::fadd(F32(0xbea6b08f), rt::fmul(l_z_2, rt::fadd(F32(0x3e4e0aa8), rt::fmul(l_z_2, rt::fadd(F32(0xbd241146), rt::fmul(l_z_2, rt::fadd(F32(0x3a4f7f04), rt::fmul(l_z_2, F32(0x3811ef08))))))))))));
                l_q_4 = rt::fadd(F32(0x3f800000), rt::fmul(l_z_2, rt::fadd(F32(0xc019d138), rt::fmul(l_z_2, rt::fadd(F32(0x4001572c), rt::fmul(l_z_2, rt::fadd(F32(0xbf303360), rt::fmul(l_z_2, F32(0x3d9dc62d)))))))));
                l_s_7 = F::__ieee754_sqrtf(c, l_z_2);
                l_r_5 = rt::fdiv(l_p_3, l_q_4);
                l_w_6 = rt::fsub(rt::fmul(l_r_5, l_s_7), F32(0x33a22168));
                break 'f rt::fsub(F32(0x40490fda), rt::fmul(F32(0x40000000), rt::fadd(l_s_7, l_w_6)));
            } else {
                l_z_2 = rt::fmul(rt::fsub(F32(0x3f800000), p_x_1), F32(0x3f000000));
                l_s_7 = F::__ieee754_sqrtf(c, l_z_2);
                l_df_9 = l_s_7;
                'l3: loop {
                    'c4: {
                        { let v_ = l_df_9; let a_ = fp.wrapping_add(4); c.m.w32(a_, (v_).0); };
                        l_idf_12 = ((c.m.r32(fp.wrapping_add(4))) as i32);
                    }
                    if !((0i32) != 0) { break 'l3; }
                }
                'l5: loop {
                    'c6: {
                        { let v_ = ((((l_idf_12) as u32)) & (4294963200u32)); let a_ = fp.wrapping_add(8); c.m.w32(a_, v_); };
                        l_df_9 = F32(c.m.r32(fp.wrapping_add(8)));
                    }
                    if !((0i32) != 0) { break 'l5; }
                }
                l_c_8 = rt::fdiv(rt::fsub(l_z_2, rt::fmul(l_df_9, l_df_9)), rt::fadd(l_s_7, l_df_9));
                l_p_3 = rt::fmul(l_z_2, rt::fadd(F32(0x3e2aaaaa), rt::fmul(l_z_2, rt::fadd(F32(0xbea6b08f), rt::fmul(l_z_2, rt::fadd(F32(0x3e4e0aa8), rt::fmul(l_z_2, rt::fadd(F32(0xbd241146), rt::fmul(l_z_2, rt::fadd(F32(0x3a4f7f04), rt::fmul(l_z_2, F32(0x3811ef08))))))))))));
                l_q_4 = rt::fadd(F32(0x3f800000), rt::fmul(l_z_2, rt::fadd(F32(0xc019d138), rt::fmul(l_z_2, rt::fadd(F32(0x4001572c), rt::fmul(l_z_2, rt::fadd(F32(0xbf303360), rt::fmul(l_z_2, F32(0x3d9dc62d)))))))));
                l_r_5 = rt::fdiv(l_p_3, l_q_4);
                l_w_6 = rt::fadd(rt::fmul(l_r_5, l_s_7), l_c_8);
                break 'f rt::fmul(F32(0x40000000), rt::fadd(l_df_9, l_w_6));
            }
        }
       F32(0)
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "__ieee754_acosf", &PROBE_N, &[], true); }
    r_
}
