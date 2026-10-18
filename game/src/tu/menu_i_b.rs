#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn BootCard_OnFlowDone(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BootCard_OnFlowDone", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let r_ = 1i32; let a_ = (c.m.r32(sym::gBootCard)).wrapping_add(8); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    { let r_ = 2i32; let a_ = (c.m.r32(sym::gBootCard)).wrapping_add(8); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    { let v_ = 15i32; let a_ = (c.m.r32(sym::gBootCard)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BootCard_OnFlowDone", &PROBE_N, &[], true); }
    r_
}

pub fn BootCard_Init(c: &mut Ctx, mut p_section_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BootCard_Init", &PROBE_N, &[(p_section_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = F::Heap_Alloc(c, 20i32, ((32i32) as u32), 0i32, 2i32); let a_ = sym::gBootCard; c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32(sym::gBootCard); F::memset(c, a0_, 0i32, ((20i32) as u32)) }; };
    { let v_ = (c.m.r32(sym::gMenuArc0)).wrapping_add((({ let a_ = (c.m.r32(sym::gMenuArc0)).wrapping_add((((p_section_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = c.m.r32(sym::gBootCard); c.m.w32(a_, v_); };
    { let v_ = { let a0_ = { let a_ = c.m.r32(sym::gBootCard); c.m.r32(a_) }; F::Sprite_Unpack(c, a0_, ((0i32) as u32), ((0i32) as u32)) }; let a_ = (c.m.r32(sym::gBootCard)).wrapping_add(4); c.m.w32(a_, v_); };
    { let a0_ = ({ let a_ = (c.m.r32(sym::gBootCard)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gBootCard)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); F::Dialog_Init(c, a0_, ((0i32) as u32), 0i32) };
    { let _ = 1i32; F::McFlow_Init(c) };
    F::McFlow_SetDoneCb(c, 0i32, fid::BootCard_OnFlowDone, ((((((0i32) as u32)) as u64)) as i32));
    F::McFlow_SetDoneCb(c, 1i32, fid::BootCard_OnFlowDone, ((((((0i32) as u32)) as u64)) as i32));
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BootCard_Init", &PROBE_N, &[], true); }
    r_
}

pub fn BootCard_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BootCard_Term", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::Dialog_Term(c);
    F::McFlow_Term(c);
    if ((((({ let a_ = (c.m.r32(sym::gBootCard)).wrapping_add(4); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = { let a_ = (c.m.r32(sym::gBootCard)).wrapping_add(4); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = (c.m.r32(sym::gBootCard)).wrapping_add(4); c.m.w32(a_, v_); };
    }
    if (((((c.m.r32(sym::gBootCard)) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32(sym::gBootCard); F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = sym::gBootCard; c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BootCard_Term", &PROBE_N, &[], true); }
    r_
}

pub fn BootCard_Run(c: &mut Ctx, mut p_section_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BootCard_Run", &PROBE_N, &[(p_section_1) as u32], false); }
    let r_ = 'f: {
    F::BootCard_Init(c, p_section_1);
    F::ColorFade_StartIn(c, 0i32, 0i32, 0i32, 20i32);
    'l1: while ((1i32) != 0) {
        F::Gfx_BeginFrame(c);
        F::Pad_Update(c);
        F::Snd_Update(c);
        F::ColorFade_Update(c);
        { let v_ = F::McFlow_Update(c); let a_ = (c.m.r32(sym::gBootCard)).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
        F::ColorFade_Draw(c);
        F::Gfx_EndFrame(c, 1i32);
        F::Dma_Flush(c);
        F::File_Stub264D90(c);
        if ((F::ColorFade_IsInDone(c)) != 0) {
            if ((((!(((({ let a_ = (c.m.r32(sym::gBootCard)).wrapping_add(8); (c.m.r32(a_) as i32) }) & (8i32))) != 0)) as i32)) != 0) {
                { let r_ = 8i32; let a_ = (c.m.r32(sym::gBootCard)).wrapping_add(8); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                F::McFlow_Start(c, 2i32);
            }
        }
        if ((F::ColorFade_IsFadingOut(c)) != 0) {
            continue 'l1;
        }
        if ((F::ColorFade_IsOutDone(c)) != 0) {
            break 'l1;
        }
        if (((({ let a_ = (c.m.r32(sym::gBootCard)).wrapping_add(8); (c.m.r32(a_) as i32) }) & (2i32))) != 0) {
            if ((((({ let a_ = (c.m.r32(sym::gBootCard)).wrapping_add(12); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) == ((1i32).wrapping_neg())) as i32)) != 0) {
                F::ColorFade_StartOut(c, 0i32, 0i32, 0i32, 20i32);
            }
        }
    }
    { let _ = F::sceGsSyncPath(c, 0i32, ((0i32) as u16)); };
    F::BootCard_Term(c);
    F::Dma_ResetBuffers(c);
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BootCard_Run", &PROBE_N, &[], true); }
    r_
}
