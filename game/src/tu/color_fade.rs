#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn ColorFade_Draw(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ColorFade_Draw", &PROBE_N, &[], false); }
    let mut l_fade_1: u32 = 0;
    let mut l_p_2: u32 = 0;
    let r_ = 'f: {
    l_fade_1 = sym::gColorFade;
    { let v_ = (((if ((((((c.m.r32((l_fade_1).wrapping_add(8)) as i32)) > (128i32)) as i32)) != 0) { 128i32 } else { (if ((((((c.m.r32((l_fade_1).wrapping_add(8)) as i32)) < (0i32)) as i32)) != 0) { 0i32 } else { (c.m.r32((l_fade_1).wrapping_add(8)) as i32) }) })) as u8); let a_ = (l_fade_1).wrapping_add(7); c.m.w8(a_, v_); };
    if (((((((c.m.r8((sym::gColorFade).wrapping_add(7))) as i32)) != (0i32)) as i32)) != 0) {
        F::Gfx_AddDefaultEnv(c);
        l_p_2 = F::Dma_BeginDirect(c);
        { let v_ = ((((((((2i32) as u64)) | ((((1i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (l_p_2).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((14i32) as u64); let a_ = (l_p_2).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_2).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_2 = n_; };
        { let v_ = ((68i32) as u64); let a_ = (l_p_2).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((66i32) as u64); let a_ = (l_p_2).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_2).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_2 = n_; };
        { let v_ = ((196608i32) as u64); let a_ = (l_p_2).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((71i32) as u64); let a_ = (l_p_2).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_2).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_2 = n_; };
        { let v_ = ((((((((1i32) as u64)) | ((((1i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((4i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (l_p_2).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((21776i32) as u64); let a_ = (l_p_2).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_2).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_2 = n_; };
        { let v_ = ((70i32) as u64); let a_ = (l_p_2).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((((((((c.m.r8((sym::gColorFade).wrapping_add(4))) as u64)) | ((((c.m.r8((sym::gColorFade).wrapping_add(5))) as u64)).wrapping_shl((8i32) as u32)))) | ((((c.m.r8((sym::gColorFade).wrapping_add(6))) as u64)).wrapping_shl((16i32) as u32)))) | ((((c.m.r8((sym::gColorFade).wrapping_add(7))) as u64)).wrapping_shl((24i32) as u32))); let a_ = (l_p_2).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_2).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_2 = n_; };
        { let v_ = ((1912631296i32) as u64); let a_ = (l_p_2).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((2382401536u32) as u64); let a_ = (l_p_2).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_2).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_2 = n_; };
        F::Dma_EndDirect(c, l_p_2);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ColorFade_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn ColorFade_Update(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ColorFade_Update", &PROBE_N, &[], false); }
    let mut l_fade_1: u32 = 0;
    let r_ = 'f: {
    l_fade_1 = sym::gColorFade;
    if (((((c.m.r32(l_fade_1) as i32)) & (1i32))) != 0) {
        if ((((((c.m.r32((l_fade_1).wrapping_add(8)) as i32)) > (0i32)) as i32)) != 0) {
            { let r_ = (c.m.r32((l_fade_1).wrapping_add(12)) as i32); let a_ = (l_fade_1).wrapping_add(8); let n_ = ((c.m.r32(a_) as i32)).wrapping_sub(r_); c.m.w32(a_, (n_) as u32); };
        } else {
            { let v_ = 4i32; let a_ = l_fade_1; c.m.w32(a_, (v_) as u32); };
        }
    }
    if (((((c.m.r32(sym::gColorFade) as i32)) & (2i32))) != 0) {
        if ((((((c.m.r32((sym::gColorFade).wrapping_add(8)) as i32)) < (((c.m.r32((sym::gColorFade).wrapping_add(12)) as i32)).wrapping_add(128i32))) as i32)) != 0) {
            { let r_ = (c.m.r32((sym::gColorFade).wrapping_add(12)) as i32); let a_ = (sym::gColorFade).wrapping_add(8); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
        } else {
            { let v_ = 8i32; let a_ = sym::gColorFade; c.m.w32(a_, (v_) as u32); };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ColorFade_Update", &PROBE_N, &[], true); }
    r_
}

pub fn ColorFade_StartIn(c: &mut Ctx, mut p_r_1: i32, mut p_g_2: i32, mut p_b_3: i32, mut p_frames_4: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ColorFade_StartIn", &PROBE_N, &[(p_r_1) as u32, (p_g_2) as u32, (p_b_3) as u32, (p_frames_4) as u32], false); }
    let r_ = 'f: {
    { let _ = F::memset(c, sym::gColorFade, 0i32, 16u32); };
    { let v_ = 1i32; let a_ = sym::gColorFade; c.m.w32(a_, (v_) as u32); };
    { let v_ = ((p_r_1) as u8); let a_ = (sym::gColorFade).wrapping_add(4); c.m.w8(a_, v_); };
    { let v_ = ((p_g_2) as u8); let a_ = (sym::gColorFade).wrapping_add(5); c.m.w8(a_, v_); };
    { let v_ = ((p_b_3) as u8); let a_ = (sym::gColorFade).wrapping_add(6); c.m.w8(a_, v_); };
    { let v_ = 128i32; let a_ = (sym::gColorFade).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
    { let v_ = rt::div_i32(128i32, p_frames_4); let a_ = (sym::gColorFade).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
    F::LipSync_Clear(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ColorFade_StartIn", &PROBE_N, &[], true); }
    r_
}

pub fn ColorFade_StartOut(c: &mut Ctx, mut p_r_1: i32, mut p_g_2: i32, mut p_b_3: i32, mut p_frames_4: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ColorFade_StartOut", &PROBE_N, &[(p_r_1) as u32, (p_g_2) as u32, (p_b_3) as u32, (p_frames_4) as u32], false); }
    let r_ = 'f: {
    { let _ = F::memset(c, sym::gColorFade, 0i32, 16u32); };
    { let v_ = 2i32; let a_ = sym::gColorFade; c.m.w32(a_, (v_) as u32); };
    { let v_ = ((p_r_1) as u8); let a_ = (sym::gColorFade).wrapping_add(4); c.m.w8(a_, v_); };
    { let v_ = ((p_g_2) as u8); let a_ = (sym::gColorFade).wrapping_add(5); c.m.w8(a_, v_); };
    { let v_ = ((p_b_3) as u8); let a_ = (sym::gColorFade).wrapping_add(6); c.m.w8(a_, v_); };
    { let v_ = 0i32; let a_ = (sym::gColorFade).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
    { let v_ = rt::div_i32(128i32, p_frames_4); let a_ = (sym::gColorFade).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ColorFade_StartOut", &PROBE_N, &[], true); }
    r_
}

pub fn ColorFade_IsInDone(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ColorFade_IsInDone", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f ((((c.m.r32(sym::gColorFade) as i32)).wrapping_shr((2i32) as u32)) & (1i32));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "ColorFade_IsInDone", &PROBE_N, &[], true); }
    r_
}

pub fn ColorFade_IsOutDone(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ColorFade_IsOutDone", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f ((((c.m.r32(sym::gColorFade) as i32)).wrapping_shr((3i32) as u32)) & (1i32));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "ColorFade_IsOutDone", &PROBE_N, &[], true); }
    r_
}

pub fn ColorFade_IsFadingIn(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ColorFade_IsFadingIn", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f (((c.m.r32(sym::gColorFade) as i32)) & (1i32));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "ColorFade_IsFadingIn", &PROBE_N, &[], true); }
    r_
}

pub fn ColorFade_IsFadingOut(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ColorFade_IsFadingOut", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f ((((c.m.r32(sym::gColorFade) as i32)).wrapping_shr((1i32) as u32)) & (1i32));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "ColorFade_IsFadingOut", &PROBE_N, &[], true); }
    r_
}
