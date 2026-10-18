#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn Port_AdxTick(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Port_AdxTick", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let mut l_v_2: u32 = 0;
    let r_ = 'f: {
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (16i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                l_v_2 = (sym::gPortAdxView).wrapping_add((((l_i_1) as u32)).wrapping_mul(280));
                if ((((((((((((((((((c.m.r32(l_v_2) as i32)) == (3i32)) as i32)) != 0) && ((((!(((c.m.r32((l_v_2).wrapping_add(4)) as i32)) != 0)) as i32)) != 0)) as i32)) != 0) && ((((((c.m.r32((l_v_2).wrapping_add(8)) as i32)) > (0i32)) as i32)) != 0)) as i32)) != 0) && ((((({ let a_ = (l_v_2).wrapping_add(8); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) == (0i32)) as i32)) != 0)) as i32)) != 0) {
                    { let v_ = 5i32; let a_ = l_v_2; c.m.w32(a_, (v_) as u32); };
                }
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Port_AdxTick", &PROBE_N, &[], true); }
    r_
}
