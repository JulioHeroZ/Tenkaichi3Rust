#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn HudGauge_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudGauge_Term", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let r_ = 'f: {
    if ((((({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = { let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(4); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
    }
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (12i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((((({ let a_ = (({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((l_i_1) as u32)).wrapping_mul(56))).wrapping_add(36); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
                    { let a0_ = { let a_ = (({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((l_i_1) as u32)).wrapping_mul(56))).wrapping_add(36); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
                }
                if ((((({ let a_ = (({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((l_i_1) as u32)).wrapping_mul(56))).wrapping_add(44); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
                    { let a0_ = { let a_ = (({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((l_i_1) as u32)).wrapping_mul(56))).wrapping_add(44); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
                }
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
    if ((((({ let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(8); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = { let a_ = (c.m.r32(sym::gHudGauge)).wrapping_add(8); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
    }
    if (((((c.m.r32(sym::gHudGauge)) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32(sym::gHudGauge); F::Heap_Free(c, a0_) };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudGauge_Term", &PROBE_N, &[], true); }
    r_
}

pub fn HudGauge_Reset(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HudGauge_Reset", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let mut l_j_2: i32 = 0;
    let r_ = 'f: {
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (2i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let v_ = 0i32; let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(40)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(48)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(56)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(112)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(120)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(128)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(232)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(288)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(312)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                { let v_ = (1i32).wrapping_neg(); let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(432)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let _ = { let a0_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(320)).wrapping_add((((l_i_1) as u32)).wrapping_mul(16)); F::memset(c, a0_, 0i32, 16u32) }; };
                { let _ = { let a0_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(352)).wrapping_add((((l_i_1) as u32)).wrapping_mul(16)); F::memset(c, a0_, 0i32, 16u32) }; };
                { let _ = { let a0_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(384)).wrapping_add((((l_i_1) as u32)).wrapping_mul(24)); F::memset(c, a0_, 0i32, 24u32) }; };
                { let v_ = 0i32; let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(304)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let _ = { let a0_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(136)).wrapping_add((((l_i_1) as u32)).wrapping_mul(24)); F::memset(c, a0_, 0i32, 24u32) }; };
                { let _ = { let a0_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(184)).wrapping_add((((l_i_1) as u32)).wrapping_mul(24)); F::memset(c, a0_, 0i32, 24u32) }; };
                { let _ = { let a0_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(240)).wrapping_add((((l_i_1) as u32)).wrapping_mul(24)); F::memset(c, a0_, 0i32, 24u32) }; };
                { let _ = { let a0_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(64)).wrapping_add((((l_i_1) as u32)).wrapping_mul(24)); F::memset(c, a0_, 0i32, 24u32) }; };
                { let _ = { let a0_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(512)).wrapping_add((((l_i_1) as u32)).wrapping_mul(24)); F::memset(c, a0_, 0i32, 24u32) }; };
                { let _ = { let a0_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(440)).wrapping_add((((l_i_1) as u32)).wrapping_mul(24)); F::memset(c, a0_, 0i32, 24u32) }; };
                { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(488)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(496)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                { let v_ = 0i32; let a_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(504)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let _ = { let a0_ = ((c.m.r32(sym::gHudGauge)).wrapping_add(512)).wrapping_add((((l_i_1) as u32)).wrapping_mul(24)); F::memset(c, a0_, 0i32, 24u32) }; };
                {
                    l_j_2 = 0i32;
                    'l3: loop {
                        if !(((((l_j_2) < (3i32)) as i32)) != 0) { break 'l3; }
                        'c4: {
                            { let _ = { let a0_ = (((c.m.r32(sym::gHudGauge)).wrapping_add(12)).wrapping_add((((l_i_1) as u32)).wrapping_mul(12))).wrapping_add((((l_j_2) as u32)).wrapping_mul(4)); F::memset(c, a0_, 0i32, 4u32) }; };
                        }
                        { let o_ = l_j_2; let n_ = (o_).wrapping_add(1); l_j_2 = n_; };
                    }
                }
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
    { let _ = { let a0_ = (c.m.r32(sym::gHudGauge)).wrapping_add(560); F::memset(c, a0_, 0i32, 24u32) }; };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HudGauge_Reset", &PROBE_N, &[], true); }
    r_
}
