#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn Port_sinf(c: &mut Ctx, mut p_x_1: F32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Port_sinf", &PROBE_N, &[(p_x_1).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_z_2: F32 = F32(0);
    let mut l_n_3: i32 = 0;
    let mut l_ix_4: i32 = 0;
    let r_ = 'f: {
        l_z_2 = F32(0x00000000);
        'l1: loop {
            'c2: {
                { let v_ = p_x_1; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_).0); };
                l_ix_4 = ((c.m.r32(fp.wrapping_add(8))) as i32);
            }
            if !((0i32) != 0) { break 'l1; }
        }
        { let r_ = 2147483647i32; let n_ = ((l_ix_4) & (r_)); l_ix_4 = n_; };
        if (((((l_ix_4) <= (1061752792i32)) as i32)) != 0) {
            break 'f F::__kernel_sinf(c, p_x_1, l_z_2, 0i32);
        } else {
            if ((((!(((((l_ix_4) < (2139095040i32)) as i32)) != 0)) as i32)) != 0) {
                break 'f rt::fsub(p_x_1, p_x_1);
            } else {
                l_n_3 = F::__ieee754_rem_pio2f(c, p_x_1, fp);
                's3: {
                    let sv5: i32 = ((l_n_3) & (3i32));
                    match sv5 {
                        0i32 => {
                            break 'f { let a0_ = F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a1_ = F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); F::__kernel_sinf(c, a0_, a1_, 1i32) };
                        }
                        1i32 => {
                            break 'f { let a0_ = F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a1_ = F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); F::__kernel_cosf(c, a0_, a1_) };
                        }
                        2i32 => {
                            break 'f rt::fneg({ let a0_ = F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a1_ = F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); F::__kernel_sinf(c, a0_, a1_, 1i32) });
                        }
                        _ => {
                            break 'f rt::fneg({ let a0_ = F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a1_ = F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); F::__kernel_cosf(c, a0_, a1_) });
                        }
                    }
                }
            }
        }
       F32(0)
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Port_sinf", &PROBE_N, &[], true); }
    r_
}
