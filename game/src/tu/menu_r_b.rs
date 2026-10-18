#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn SimEvent_Run(c: &mut Ctx, mut p_day_1: u32, mut p_event_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SimEvent_Run", &PROBE_N, &[(p_day_1) as u32, (p_event_2) as u32], false); }
    let mut l_done_3: i32 = 0;
    let r_ = 'f: {
    l_done_3 = 1i32;
    if (((((p_event_2) < (((37i32) as u32))) as i32)) != 0) {
        l_done_3 = { let fp_ = c.m.r32((sym::gSimEvent).wrapping_add((p_event_2).wrapping_mul(4))); ((D::call(c, fp_, &[((p_day_1) as u64)])) as i32) };
    }
    break 'f l_done_3;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "SimEvent_Run", &PROBE_N, &[], true); }
    r_
}
