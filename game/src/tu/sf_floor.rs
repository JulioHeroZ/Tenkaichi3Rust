#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn Port_floorf(c: &mut Ctx, mut p_x_1: F32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Port_floorf", &PROBE_N, &[(p_x_1).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_i0_2: i32 = 0;
    let mut l_j0_3: i32 = 0;
    let mut l_i_4: u32 = 0;
    let mut l_ix_5: u32 = 0;
    let r_ = 'f: {
        'l1: loop {
            'c2: {
                { let v_ = p_x_1; let a_ = fp; c.m.w32(a_, (v_).0); };
                l_i0_2 = ((c.m.r32(fp)) as i32);
            }
            if !((0i32) != 0) { break 'l1; }
        }
        l_ix_5 = ((((l_i0_2) & (2147483647i32))) as u32);
        l_j0_3 = ((((l_ix_5).wrapping_shr((23i32) as u32)).wrapping_sub(((127i32) as u32))) as i32);
        if (((((l_j0_3) < (23i32)) as i32)) != 0) {
            if (((((l_j0_3) < (0i32)) as i32)) != 0) {
                if (((rt::fgt(rt::fadd(F32(0x7149f2c9), p_x_1), F32(0x00000000)) as i32)) != 0) {
                    if (((((l_i0_2) >= (0i32)) as i32)) != 0) {
                        l_i0_2 = 0i32;
                    } else {
                        if ((((!(((((l_ix_5) == (((0i32) as u32))) as i32)) != 0)) as i32)) != 0) {
                            l_i0_2 = ((3212836864u32) as i32);
                        }
                    }
                }
            } else {
                l_i_4 = (((8388607i32).wrapping_shr((l_j0_3) as u32)) as u32);
                if (((((((((l_i0_2) as u32)) & (l_i_4))) == (((0i32) as u32))) as i32)) != 0) {
                    break 'f p_x_1;
                }
                if (((rt::fgt(rt::fadd(F32(0x7149f2c9), p_x_1), F32(0x00000000)) as i32)) != 0) {
                    if (((((l_i0_2) < (0i32)) as i32)) != 0) {
                        { let r_ = (8388608i32).wrapping_shr((l_j0_3) as u32); let n_ = (l_i0_2).wrapping_add(r_); l_i0_2 = n_; };
                    }
                    { let r_ = (!(l_i_4)); let n_ = ((((((l_i0_2) as u32)) & (r_))) as i32); l_i0_2 = n_; };
                }
            }
        } else {
            if ((((!(((((l_ix_5) < (((2139095040i32) as u32))) as i32)) != 0)) as i32)) != 0) {
                break 'f rt::fadd(p_x_1, p_x_1);
            } else {
                break 'f p_x_1;
            }
        }
        'l3: loop {
            'c4: {
                { let v_ = ((l_i0_2) as u32); let a_ = fp.wrapping_add(4); c.m.w32(a_, v_); };
                p_x_1 = F32(c.m.r32(fp.wrapping_add(4)));
            }
            if !((0i32) != 0) { break 'l3; }
        }
        break 'f p_x_1;
       F32(0)
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Port_floorf", &PROBE_N, &[], true); }
    r_
}
