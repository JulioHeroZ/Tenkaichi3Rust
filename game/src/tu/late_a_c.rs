#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn ShenSave_OnFlowDone(c: &mut Ctx, mut p_arg_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ShenSave_OnFlowDone", &PROBE_N, &[(p_arg_1) as u32], false); }
    let r_ = 'f: {
    { let r_ = 1i32; let a_ = (c.m.r32(sym::gShenSave)).wrapping_add(8); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    { let v_ = 15i32; let a_ = (c.m.r32(sym::gShenSave)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ShenSave_OnFlowDone", &PROBE_N, &[], true); }
    r_
}

pub fn ShenSave_Init(c: &mut Ctx, mut p_section_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ShenSave_Init", &PROBE_N, &[(p_section_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = F::Heap_Alloc(c, ((24u32) as i32), ((32i32) as u32), 0i32, 2i32); let a_ = sym::gShenSave; c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32(sym::gShenSave); F::memset(c, a0_, 0i32, 24u32) }; };
    { let v_ = (c.m.r32(sym::gMenuArc12)).wrapping_add((({ let a_ = (c.m.r32(sym::gMenuArc12)).wrapping_add((((p_section_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = c.m.r32(sym::gShenSave); c.m.w32(a_, v_); };
    { let v_ = { let a0_ = { let a_ = c.m.r32(sym::gShenSave); c.m.r32(a_) }; F::Sprite_Unpack(c, a0_, ((0i32) as u32), ((0i32) as u32)) }; let a_ = (c.m.r32(sym::gShenSave)).wrapping_add(4); c.m.w32(a_, v_); };
    { let a0_ = ({ let a_ = (c.m.r32(sym::gShenSave)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gShenSave)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((10i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); F::Dialog_Init(c, a0_, ((0i32) as u32), 0i32) };
    { let _ = 1i32; F::McFlow_Init(c) };
    F::McFlow_SetDoneCb(c, 0i32, fid::ShenSave_OnFlowDone, 0i32);
    F::McFlow_SetDoneCb(c, 1i32, fid::ShenSave_OnFlowDone, 0i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ShenSave_Init", &PROBE_N, &[], true); }
    r_
}

pub fn ShenSave_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ShenSave_Term", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::Dialog_Term(c);
    F::McFlow_Term(c);
    if ((((({ let a_ = (c.m.r32(sym::gShenSave)).wrapping_add(4); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = { let a_ = (c.m.r32(sym::gShenSave)).wrapping_add(4); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = (c.m.r32(sym::gShenSave)).wrapping_add(4); c.m.w32(a_, v_); };
    }
    if (((((c.m.r32(sym::gShenSave)) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32(sym::gShenSave); F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = sym::gShenSave; c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ShenSave_Term", &PROBE_N, &[], true); }
    r_
}

pub fn ShenSave_Draw(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ShenSave_Draw", &PROBE_N, &[], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ShenSave_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn ShenSave_Advance(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ShenSave_Advance", &PROBE_N, &[], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ShenSave_Advance", &PROBE_N, &[], true); }
    r_
}

pub fn ShenSave_Update(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ShenSave_Update", &PROBE_N, &[], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ShenSave_Update", &PROBE_N, &[], true); }
    r_
}

pub fn ShenSave_UpdateExit(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ShenSave_UpdateExit", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if (((({ let a_ = (c.m.r32(sym::gShenSave)).wrapping_add(8); (c.m.r32(a_) as i32) }) & (1i32))) != 0) {
        if ((((!(((({ let a_ = (c.m.r32(sym::gShenSave)).wrapping_add(8); (c.m.r32(a_) as i32) }) & (2i32))) != 0)) as i32)) != 0) {
            { let r_ = 2i32; let a_ = (c.m.r32(sym::gShenSave)).wrapping_add(8); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            F::ColorFade_StartOut(c, 0i32, 0i32, 0i32, 20i32);
        }
        if ((((({ let a_ = (c.m.r32(sym::gShenSave)).wrapping_add(12); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) == (0i32)) as i32)) != 0) {
            break 'f 1i32;
        }
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "ShenSave_UpdateExit", &PROBE_N, &[], true); }
    r_
}

pub fn ShenSave_Run(c: &mut Ctx, mut p_section_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ShenSave_Run", &PROBE_N, &[(p_section_1) as u32], false); }
    let mut l_ret_2: i32 = 0;
    let r_ = 'f: {
    F::ShenSave_Init(c, p_section_1);
    F::ColorFade_StartIn(c, 0i32, 0i32, 0i32, 20i32);
    {
        'l1: loop {
            'c2: {
                F::Gfx_BeginFrame(c);
                F::Pad_Update(c);
                F::ColorFade_Update(c);
                F::Snd_Update(c);
                F::ShenSave_Advance(c);
                F::ShenSave_Draw(c);
                F::ColorFade_Draw(c);
                { let v_ = F::McFlow_Update(c); let a_ = (c.m.r32(sym::gShenSave)).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
                F::Gfx_EndFrame(c, 1i32);
                F::Dma_Flush(c);
                F::File_Stub264D90(c);
                if ((F::ShenSave_UpdateExit(c)) != 0) {
                    break 'l1;
                }
                if ((((((F::ColorFade_IsInDone(c)) != 0) && ((((!(((({ let a_ = (c.m.r32(sym::gShenSave)).wrapping_add(8); (c.m.r32(a_) as i32) }) & (4i32))) != 0)) as i32)) != 0)) as i32)) != 0) {
                    F::McFlow_Start(c, 0i32);
                    { let r_ = 4i32; let a_ = (c.m.r32(sym::gShenSave)).wrapping_add(8); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                }
                if ((((({ let a_ = (c.m.r32(sym::gShenSave)).wrapping_add(20); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
                    F::ShenSave_Update(c);
                }
            }
        }
    }
    l_ret_2 = { let a_ = (c.m.r32(sym::gShenSave)).wrapping_add(16); (c.m.r32(a_) as i32) };
    F::ShenSave_Term(c);
    F::Dma_ResetBuffers(c);
    break 'f l_ret_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "ShenSave_Run", &PROBE_N, &[], true); }
    r_
}
