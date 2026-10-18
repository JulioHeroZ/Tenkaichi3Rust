#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn HistSave_OnCardDone(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HistSave_OnCardDone", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let r_ = 1i32; let a_ = (c.m.r32(sym::gHistSave)).wrapping_add(8); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    { let r_ = 2i32; let a_ = (c.m.r32(sym::gHistSave)).wrapping_add(8); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    { let v_ = 15i32; let a_ = (c.m.r32(sym::gHistSave)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HistSave_OnCardDone", &PROBE_N, &[], true); }
    r_
}

pub fn HistSave_Init(c: &mut Ctx, mut p_section_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HistSave_Init", &PROBE_N, &[(p_section_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = F::Heap_Alloc(c, 20i32, ((32i32) as u32), 0i32, 2i32); let a_ = sym::gHistSave; c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32(sym::gHistSave); F::memset(c, a0_, 0i32, ((20i32) as u32)) }; };
    { let v_ = (c.m.r32(sym::gMenuArc2)).wrapping_add((({ let a_ = (c.m.r32(sym::gMenuArc2)).wrapping_add((((p_section_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = c.m.r32(sym::gHistSave); c.m.w32(a_, v_); };
    { let v_ = { let a0_ = { let a_ = c.m.r32(sym::gHistSave); c.m.r32(a_) }; F::Sprite_Unpack(c, a0_, ((0i32) as u32), ((0i32) as u32)) }; let a_ = (c.m.r32(sym::gHistSave)).wrapping_add(4); c.m.w32(a_, v_); };
    { let a0_ = ({ let a_ = (c.m.r32(sym::gHistSave)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gHistSave)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); F::Dialog_Init(c, a0_, ((0i32) as u32), 0i32) };
    { let _ = 1i32; F::McFlow_Init(c) };
    F::McFlow_SetDoneCb(c, 0i32, fid::HistSave_OnCardDone, ((((((0i32) as u32)) as u64)) as i32));
    F::McFlow_SetDoneCb(c, 1i32, fid::HistSave_OnCardDone, ((((((0i32) as u32)) as u64)) as i32));
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HistSave_Init", &PROBE_N, &[], true); }
    r_
}

pub fn HistSave_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HistSave_Term", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::Dialog_Term(c);
    F::McFlow_Term(c);
    if ((((({ let a_ = (c.m.r32(sym::gHistSave)).wrapping_add(4); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = { let a_ = (c.m.r32(sym::gHistSave)).wrapping_add(4); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = (c.m.r32(sym::gHistSave)).wrapping_add(4); c.m.w32(a_, v_); };
    }
    if (((((c.m.r32(sym::gHistSave)) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32(sym::gHistSave); F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = sym::gHistSave; c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HistSave_Term", &PROBE_N, &[], true); }
    r_
}

pub fn HistSave_Draw(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HistSave_Draw", &PROBE_N, &[], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HistSave_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn HistSave_Update(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HistSave_Update", &PROBE_N, &[], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HistSave_Update", &PROBE_N, &[], true); }
    r_
}

pub fn HistSave_Input(c: &mut Ctx, mut p_result_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HistSave_Input", &PROBE_N, &[(p_result_1) as u32], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HistSave_Input", &PROBE_N, &[], true); }
    r_
}

pub fn HistSave_Run(c: &mut Ctx, mut p_section_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HistSave_Run", &PROBE_N, &[(p_section_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        { let v_ = 1i32; c.m.w32(fp, (v_) as u32); }
        F::HistSave_Init(c, p_section_1);
        F::ColorFade_StartIn(c, 0i32, 0i32, 0i32, 20i32);
        'l1: while ((1i32) != 0) {
            F::Gfx_BeginFrame(c);
            F::Pad_Update(c);
            F::Snd_Update(c);
            F::ColorFade_Update(c);
            F::HistSave_Update(c);
            F::HistSave_Draw(c);
            { let v_ = F::McFlow_Update(c); let a_ = (c.m.r32(sym::gHistSave)).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
            F::ColorFade_Draw(c);
            F::Gfx_EndFrame(c, 1i32);
            F::Dma_Flush(c);
            F::File_Stub264D90(c);
            if (((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(20); (c.m.r32(a_) as i32) }) & (256i32))) != 0) {
                continue 'l1;
            }
            if ((F::ColorFade_IsInDone(c)) != 0) {
                if ((((!(((({ let a_ = (c.m.r32(sym::gHistSave)).wrapping_add(8); (c.m.r32(a_) as i32) }) & (8i32))) != 0)) as i32)) != 0) {
                    { let r_ = 8i32; let a_ = (c.m.r32(sym::gHistSave)).wrapping_add(8); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    F::McFlow_Start(c, 0i32);
                }
            }
            if ((F::ColorFade_IsFadingOut(c)) != 0) {
                continue 'l1;
            }
            if ((F::ColorFade_IsOutDone(c)) != 0) {
                break 'l1;
            }
            if (((({ let a_ = (c.m.r32(sym::gHistSave)).wrapping_add(8); (c.m.r32(a_) as i32) }) & (2i32))) != 0) {
                if ((((({ let a_ = (c.m.r32(sym::gHistSave)).wrapping_add(12); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) == ((1i32).wrapping_neg())) as i32)) != 0) {
                    F::ColorFade_StartOut(c, 0i32, 0i32, 0i32, 20i32);
                }
            } else {
                if ((((({ let a_ = (c.m.r32(sym::gHistSave)).wrapping_add(16); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
                    F::HistSave_Input(c, fp);
                }
            }
        }
        's2: {
            let sv2: i32 = { let a_ = (c.m.r32(sym::gProgress)).wrapping_add(28); (c.m.r32(a_) as i32) };
            match sv2 {
                3i32 => {
                    { let v_ = 0i32; let a_ = fp; c.m.w32(a_, (v_) as u32); };
                    break 's2;
                }
                8i32 => {
                    { let v_ = 1i32; let a_ = fp; c.m.w32(a_, (v_) as u32); };
                    break 's2;
                }
                9i32 => {
                    { let v_ = 0i32; let a_ = fp; c.m.w32(a_, (v_) as u32); };
                    break 's2;
                }
                _ => {
                }
            }
        }
        F::HistSave_Term(c);
        F::Dma_ResetBuffers(c);
        break 'f (c.m.r32(fp) as i32);
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "HistSave_Run", &PROBE_N, &[], true); }
    r_
}
