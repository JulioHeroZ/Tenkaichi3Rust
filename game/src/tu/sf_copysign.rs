#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn Port_copysignf(c: &mut Ctx, mut p_x_1: F32, mut p_y_2: F32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Port_copysignf", &PROBE_N, &[(p_x_1).0, (p_y_2).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_ix_3: u32 = 0;
    let mut l_iy_4: u32 = 0;
    let r_ = 'f: {
        'l1: loop {
            'c2: {
                { let v_ = p_x_1; let a_ = fp; c.m.w32(a_, (v_).0); };
                l_ix_3 = c.m.r32(fp);
            }
            if !((0i32) != 0) { break 'l1; }
        }
        'l3: loop {
            'c4: {
                { let v_ = p_y_2; let a_ = fp.wrapping_add(4); c.m.w32(a_, (v_).0); };
                l_iy_4 = c.m.r32(fp.wrapping_add(4));
            }
            if !((0i32) != 0) { break 'l3; }
        }
        'l5: loop {
            'c6: {
                { let v_ = ((((l_ix_3) & (((2147483647i32) as u32)))) | (((l_iy_4) & (2147483648u32)))); let a_ = fp.wrapping_add(8); c.m.w32(a_, v_); };
                p_x_1 = F32(c.m.r32(fp.wrapping_add(8)));
            }
            if !((0i32) != 0) { break 'l5; }
        }
        break 'f p_x_1;
       F32(0)
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Port_copysignf", &PROBE_N, &[], true); }
    r_
}
