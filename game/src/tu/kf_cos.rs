#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn __kernel_cosf(c: &mut Ctx, mut p_x_1: F32, mut p_y_2: F32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "__kernel_cosf", &PROBE_N, &[(p_x_1).0, (p_y_2).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_a_3: F32 = F32(0);
    let mut l_hz_4: F32 = F32(0);
    let mut l_z_5: F32 = F32(0);
    let mut l_r_6: F32 = F32(0);
    let mut l_qx_7: F32 = F32(0);
    let mut l_ix_8: i32 = 0;
    let r_ = 'f: {
        'l1: loop {
            'c2: {
                { let v_ = p_x_1; let a_ = fp; c.m.w32(a_, (v_).0); };
                l_ix_8 = ((c.m.r32(fp)) as i32);
            }
            if !((0i32) != 0) { break 'l1; }
        }
        { let r_ = 2147483647i32; let n_ = ((l_ix_8) & (r_)); l_ix_8 = n_; };
        if (((((l_ix_8) < (838860800i32)) as i32)) != 0) {
            if (((((rt::f2i(p_x_1)) == (0i32)) as i32)) != 0) {
                break 'f F32(0x3f800000);
            }
        }
        l_z_5 = rt::fmul(p_x_1, p_x_1);
        l_r_6 = rt::fmul(l_z_5, rt::fadd(F32(0x3d2aaaaa), rt::fmul(l_z_5, rt::fadd(F32(0xbab60b60), rt::fmul(l_z_5, rt::fadd(F32(0x37d00d00), rt::fmul(l_z_5, rt::fadd(F32(0xb493f27b), rt::fmul(l_z_5, rt::fadd(F32(0x310f74f5), rt::fmul(l_z_5, F32(0xad47d74e))))))))))));
        if (((((l_ix_8) < (1050253722i32)) as i32)) != 0) {
            break 'f rt::fsub(F32(0x3f800000), rt::fsub(rt::fmul(F32(0x3f000000), l_z_5), rt::fsub(rt::fmul(l_z_5, l_r_6), rt::fmul(p_x_1, p_y_2))));
        } else {
            if (((((l_ix_8) > (1061683200i32)) as i32)) != 0) {
                l_qx_7 = F32(0x3e900000);
            } else {
                'l3: loop {
                    'c4: {
                        { let v_ = (((l_ix_8).wrapping_sub(16777216i32)) as u32); let a_ = fp.wrapping_add(4); c.m.w32(a_, v_); };
                        l_qx_7 = F32(c.m.r32(fp.wrapping_add(4)));
                    }
                    if !((0i32) != 0) { break 'l3; }
                }
            }
            l_hz_4 = rt::fsub(rt::fmul(F32(0x3f000000), l_z_5), l_qx_7);
            l_a_3 = rt::fsub(F32(0x3f800000), l_qx_7);
            break 'f rt::fsub(l_a_3, rt::fsub(l_hz_4, rt::fsub(rt::fmul(l_z_5, l_r_6), rt::fmul(p_x_1, p_y_2))));
        }
       F32(0)
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "__kernel_cosf", &PROBE_N, &[], true); }
    r_
}
