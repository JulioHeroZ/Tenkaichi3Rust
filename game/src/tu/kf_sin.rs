#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn __kernel_sinf(c: &mut Ctx, mut p_x_1: F32, mut p_y_2: F32, mut p_iy_3: i32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "__kernel_sinf", &PROBE_N, &[(p_x_1).0, (p_y_2).0, (p_iy_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_z_4: F32 = F32(0);
    let mut l_r_5: F32 = F32(0);
    let mut l_v_6: F32 = F32(0);
    let mut l_ix_7: i32 = 0;
    let r_ = 'f: {
        'l1: loop {
            'c2: {
                { let v_ = p_x_1; let a_ = fp; c.m.w32(a_, (v_).0); };
                l_ix_7 = ((c.m.r32(fp)) as i32);
            }
            if !((0i32) != 0) { break 'l1; }
        }
        { let r_ = 2147483647i32; let n_ = ((l_ix_7) & (r_)); l_ix_7 = n_; };
        if (((((l_ix_7) < (838860800i32)) as i32)) != 0) {
            if (((((rt::f2i(p_x_1)) == (0i32)) as i32)) != 0) {
                break 'f p_x_1;
            }
        }
        l_z_4 = rt::fmul(p_x_1, p_x_1);
        l_v_6 = rt::fmul(l_z_4, p_x_1);
        l_r_5 = rt::fadd(F32(0x3c088889), rt::fmul(l_z_4, rt::fadd(F32(0xb9500d01), rt::fmul(l_z_4, rt::fadd(F32(0x3638ef1a), rt::fmul(l_z_4, rt::fadd(F32(0xb2d72f34), rt::fmul(l_z_4, F32(0x2f2ec9d2)))))))));
        if (((((p_iy_3) == (0i32)) as i32)) != 0) {
            break 'f rt::fadd(p_x_1, rt::fmul(l_v_6, rt::fadd(F32(0xbe2aaaaa), rt::fmul(l_z_4, l_r_5))));
        } else {
            break 'f rt::fsub(p_x_1, rt::fsub(rt::fsub(rt::fmul(l_z_4, rt::fsub(rt::fmul(F32(0x3f000000), p_y_2), rt::fmul(l_v_6, l_r_5))), p_y_2), rt::fmul(l_v_6, F32(0xbe2aaaaa))));
        }
       F32(0)
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "__kernel_sinf", &PROBE_N, &[], true); }
    r_
}
