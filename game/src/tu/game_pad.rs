#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn Pad_UpdateGameButtons(c: &mut Ctx, mut p_pad_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Pad_UpdateGameButtons", &PROBE_N, &[(p_pad_1) as u32], false); }
    let mut l_p_2: u32 = 0;
    let mut l_btn_3: u32 = 0;
    let r_ = 'f: {
    l_p_2 = (sym::gPad).wrapping_add((((p_pad_1) as u32)).wrapping_mul(448));
    l_btn_3 = ((0i32) as u32);
    if ((F::Pad_IsHeld(c, p_pad_1, ((128i32) as u32))) != 0) {
        { let r_ = ((1i32) as u32); let n_ = ((l_btn_3) | (r_)); l_btn_3 = n_; };
    }
    if ((F::Pad_IsHeld(c, p_pad_1, ((32i32) as u32))) != 0) {
        { let r_ = ((2i32) as u32); let n_ = ((l_btn_3) | (r_)); l_btn_3 = n_; };
    }
    if ((F::Pad_IsHeld(c, p_pad_1, ((64i32) as u32))) != 0) {
        { let r_ = ((4i32) as u32); let n_ = ((l_btn_3) | (r_)); l_btn_3 = n_; };
    }
    if ((F::Pad_IsHeld(c, p_pad_1, ((16i32) as u32))) != 0) {
        { let r_ = ((8i32) as u32); let n_ = ((l_btn_3) | (r_)); l_btn_3 = n_; };
    }
    if ((F::Pad_IsHeld(c, p_pad_1, ((1048576i32) as u32))) != 0) {
        { let r_ = ((16i32) as u32); let n_ = ((l_btn_3) | (r_)); l_btn_3 = n_; };
    }
    if ((F::Pad_IsHeld(c, p_pad_1, ((2097152i32) as u32))) != 0) {
        { let r_ = ((32i32) as u32); let n_ = ((l_btn_3) | (r_)); l_btn_3 = n_; };
    }
    if ((F::Pad_IsHeld(c, p_pad_1, ((4194304i32) as u32))) != 0) {
        { let r_ = ((64i32) as u32); let n_ = ((l_btn_3) | (r_)); l_btn_3 = n_; };
    }
    if ((F::Pad_IsHeld(c, p_pad_1, ((8388608i32) as u32))) != 0) {
        { let r_ = ((128i32) as u32); let n_ = ((l_btn_3) | (r_)); l_btn_3 = n_; };
    }
    if ((F::Pad_IsHeld(c, p_pad_1, ((8192i32) as u32))) != 0) {
        { let r_ = ((256i32) as u32); let n_ = ((l_btn_3) | (r_)); l_btn_3 = n_; };
    }
    if ((F::Pad_IsHeld(c, p_pad_1, ((16384i32) as u32))) != 0) {
        { let r_ = ((512i32) as u32); let n_ = ((l_btn_3) | (r_)); l_btn_3 = n_; };
    }
    if ((F::Pad_IsHeld(c, p_pad_1, ((4096i32) as u32))) != 0) {
        { let r_ = ((1024i32) as u32); let n_ = ((l_btn_3) | (r_)); l_btn_3 = n_; };
    }
    if ((F::Pad_IsHeld(c, p_pad_1, ((32768i32) as u32))) != 0) {
        { let r_ = ((2048i32) as u32); let n_ = ((l_btn_3) | (r_)); l_btn_3 = n_; };
    }
    if ((F::Pad_IsHeld(c, p_pad_1, ((8i32) as u32))) != 0) {
        { let r_ = ((4096i32) as u32); let n_ = ((l_btn_3) | (r_)); l_btn_3 = n_; };
    }
    if ((F::Pad_IsHeld(c, p_pad_1, ((1i32) as u32))) != 0) {
        { let r_ = ((8192i32) as u32); let n_ = ((l_btn_3) | (r_)); l_btn_3 = n_; };
    }
    if ((F::Pad_IsHeld(c, p_pad_1, ((1024i32) as u32))) != 0) {
        { let r_ = ((16384i32) as u32); let n_ = ((l_btn_3) | (r_)); l_btn_3 = n_; };
    }
    if ((F::Pad_IsHeld(c, p_pad_1, ((256i32) as u32))) != 0) {
        { let r_ = ((32768i32) as u32); let n_ = ((l_btn_3) | (r_)); l_btn_3 = n_; };
    }
    if ((F::Pad_IsHeld(c, p_pad_1, ((2048i32) as u32))) != 0) {
        { let r_ = ((65536i32) as u32); let n_ = ((l_btn_3) | (r_)); l_btn_3 = n_; };
    }
    if ((F::Pad_IsHeld(c, p_pad_1, ((512i32) as u32))) != 0) {
        { let r_ = ((131072i32) as u32); let n_ = ((l_btn_3) | (r_)); l_btn_3 = n_; };
    }
    if ((F::Pad_IsHeld(c, p_pad_1, ((2i32) as u32))) != 0) {
        { let r_ = ((262144i32) as u32); let n_ = ((l_btn_3) | (r_)); l_btn_3 = n_; };
    }
    if ((F::Pad_IsHeld(c, p_pad_1, ((4i32) as u32))) != 0) {
        { let r_ = ((524288i32) as u32); let n_ = ((l_btn_3) | (r_)); l_btn_3 = n_; };
    }
    if ((F::Pad_IsHeld(c, p_pad_1, ((0i32) as u32))) != 0) {
        { let r_ = ((1048576i32) as u32); let n_ = ((l_btn_3) | (r_)); l_btn_3 = n_; };
    }
    if ((F::Pad_IsHeld(c, p_pad_1, ((128i32) as u32))) != 0) {
        { let r_ = ((2097152i32) as u32); let n_ = ((l_btn_3) | (r_)); l_btn_3 = n_; };
    }
    if ((F::Pad_IsHeld(c, p_pad_1, ((32i32) as u32))) != 0) {
        { let r_ = ((4194304i32) as u32); let n_ = ((l_btn_3) | (r_)); l_btn_3 = n_; };
    }
    if ((F::Pad_IsHeld(c, p_pad_1, ((64i32) as u32))) != 0) {
        { let r_ = ((8388608i32) as u32); let n_ = ((l_btn_3) | (r_)); l_btn_3 = n_; };
    }
    if ((F::Pad_IsHeld(c, p_pad_1, ((16i32) as u32))) != 0) {
        { let r_ = ((16777216i32) as u32); let n_ = ((l_btn_3) | (r_)); l_btn_3 = n_; };
    }
    { let v_ = ((l_btn_3) & ((!(c.m.r32((l_p_2).wrapping_add(392)))))); let a_ = (l_p_2).wrapping_add(396); c.m.w32(a_, v_); };
    { let v_ = l_btn_3; let a_ = (l_p_2).wrapping_add(392); c.m.w32(a_, v_); };
    { let v_ = { let a1_ = c.m.r32((l_p_2).wrapping_add(396)); let a4_ = (c.m.r32((l_p_2).wrapping_add(412)) as i32); let a5_ = (c.m.r32((l_p_2).wrapping_add(416)) as i32); F::Pad_CalcRepeat(c, l_btn_3, a1_, (l_p_2).wrapping_add(404), (l_p_2).wrapping_add(408), a4_, a5_) }; let a_ = ((sym::gPad).wrapping_add((((p_pad_1) as u32)).wrapping_mul(448))).wrapping_add(400); c.m.w32(a_, v_); };
    F::Pad_GetSticks(c, p_pad_1, (l_p_2).wrapping_add(420), (l_p_2).wrapping_add(428));
    if ((((((c.m.r32((l_p_2).wrapping_add(384)) as i32)) < (4i32)) as i32)) != 0) {
        { let v_ = (c.m.r32((l_p_2).wrapping_add(384)) as i32); let a_ = (l_p_2).wrapping_add(388); c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Pad_UpdateGameButtons", &PROBE_N, &[], true); }
    r_
}

pub fn Pad_CalcRepeat(c: &mut Ctx, mut p_held_1: u32, mut p_pressed_2: u32, mut p_timer_3: u32, mut p_last_4: u32, mut p_delay_5: i32, mut p_interval_6: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Pad_CalcRepeat", &PROBE_N, &[(p_held_1) as u32, (p_pressed_2) as u32, (p_timer_3) as u32, (p_last_4) as u32, (p_delay_5) as u32, (p_interval_6) as u32], false); }
    let mut l_result_7: u32 = 0;
    let r_ = 'f: {
    l_result_7 = ((0i32) as u32);
    if (((((((((p_held_1) == (c.m.r32(p_last_4))) as i32)) != 0) && (((((p_held_1) != (((0i32) as u32))) as i32)) != 0)) as i32)) != 0) {
        if ((((({ let a_ = p_timer_3; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) < (0i32)) as i32)) != 0) {
            { let v_ = p_interval_6; let a_ = p_timer_3; c.m.w32(a_, (v_) as u32); };
            l_result_7 = p_held_1;
        }
    } else {
        { let v_ = p_held_1; let a_ = p_last_4; c.m.w32(a_, v_); };
        { let v_ = p_delay_5; let a_ = p_timer_3; c.m.w32(a_, (v_) as u32); };
    }
    if (((((p_pressed_2) != (((0i32) as u32))) as i32)) != 0) {
        l_result_7 = p_pressed_2;
    }
    break 'f l_result_7;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Pad_CalcRepeat", &PROBE_N, &[], true); }
    r_
}

pub fn Pad_SetRepeat(c: &mut Ctx, mut p_delay_1: i32, mut p_interval_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Pad_SetRepeat", &PROBE_N, &[(p_delay_1) as u32, (p_interval_2) as u32], false); }
    let mut l_i_3: i32 = 0;
    let r_ = 'f: {
    {
        l_i_3 = 0i32;
        'l1: loop {
            if !(((((l_i_3) < (2i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let v_ = p_delay_1; let a_ = ((sym::gPad).wrapping_add((((l_i_3) as u32)).wrapping_mul(448))).wrapping_add(412); c.m.w32(a_, (v_) as u32); };
                { let v_ = p_interval_2; let a_ = ((sym::gPad).wrapping_add((((l_i_3) as u32)).wrapping_mul(448))).wrapping_add(416); c.m.w32(a_, (v_) as u32); };
            }
            { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Pad_SetRepeat", &PROBE_N, &[], true); }
    r_
}

pub fn Pad_GetStatus(c: &mut Ctx, mut p_pad_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Pad_GetStatus", &PROBE_N, &[(p_pad_1) as u32], false); }
    let r_ = 'f: {
    break 'f (c.m.r32(((sym::gPad).wrapping_add((((p_pad_1) as u32)).wrapping_mul(448))).wrapping_add(384)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Pad_GetStatus", &PROBE_N, &[], true); }
    r_
}

pub fn Pad_GetLastStatus(c: &mut Ctx, mut p_pad_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Pad_GetLastStatus", &PROBE_N, &[(p_pad_1) as u32], false); }
    let r_ = 'f: {
    break 'f (c.m.r32(((sym::gPad).wrapping_add((((p_pad_1) as u32)).wrapping_mul(448))).wrapping_add(388)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Pad_GetLastStatus", &PROBE_N, &[], true); }
    r_
}
