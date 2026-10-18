#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn StgRigidList_Count(c: &mut Ctx, p_head_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgRigidList_Count", &PROBE_N, &[(p_head_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_link_2: u32 = 0;
    let mut l_n_3: i32 = 0;
    c.m.w32(fp, p_head_1);
    let r_ = 'f: {
        l_link_2 = fp;
        l_n_3 = 0i32;
        'l1: while (((((c.m.r32(l_link_2)) != (((0i32) as u32))) as i32)) != 0) {
            { let o_ = l_n_3; let n_ = (o_).wrapping_add(1); l_n_3 = n_; };
            l_link_2 = (c.m.r32(l_link_2)).wrapping_add(4);
        }
        break 'f l_n_3;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "StgRigidList_Count", &PROBE_N, &[], true); }
    r_
}

pub fn StgRigid_AllocNode(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgRigid_AllocNode", &PROBE_N, &[], false); }
    let mut l_n_1: u32 = 0;
    let r_ = 'f: {
    if ((((({ let a_ = (c.m.r32(sym::gStgRigid)).wrapping_add(73728); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        l_n_1 = { let a_ = (c.m.r32(sym::gStgRigid)).wrapping_add(73728); c.m.r32(a_) };
        { let v_ = c.m.r32((l_n_1).wrapping_add(4)); let a_ = (c.m.r32(sym::gStgRigid)).wrapping_add(73728); c.m.w32(a_, v_); };
    } else {
        if ((((({ let a_ = (c.m.r32(sym::gStgRigid)).wrapping_add(73736); (c.m.r32(a_) as i32) }) < ((128i32).wrapping_sub(1i32))) as i32)) != 0) {
            l_n_1 = (c.m.r32(sym::gStgRigid)).wrapping_add(((({ let a_ = (c.m.r32(sym::gStgRigid)).wrapping_add(73736); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); o_ }) as u32)).wrapping_mul(576));
        } else {
            break 'f ((0i32) as u32);
        }
    }
    { let v_ = 0i32; let a_ = (l_n_1).wrapping_add(560); c.m.w32(a_, (v_) as u32); };
    break 'f l_n_1;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "StgRigid_AllocNode", &PROBE_N, &[], true); }
    r_
}
