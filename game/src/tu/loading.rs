#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn Load_RunBlocking(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Load_RunBlocking", &PROBE_N, &[], false); }
    let mut l_busy_1: i32 = 0;
    let mut l_done_2: i32 = 0;
    let mut l_prev_3: i32 = 0;
    let r_ = 'f: {
    l_busy_1 = 1i32;
    l_done_2 = 0i32;
    F::PadWatch_SetEnabled(c, 0i32);
    F::Dma_ResetBuffers(c);
    F::Load_InitScreen(c);
    F::Fade_ResetAll(c);
    F::Fade_Start(c, 0i32, 1i32, F32(0x3f800000));
    { let r_ = 16384i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(20); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    {
        'l1: loop {
            'c2: {
                F::Snd_Update(c);
                l_prev_3 = l_busy_1;
                F::Gfx_BeginFrame(c);
                F::Pad_Update(c);
                l_busy_1 = F::Job_Run(c);
                if (((((l_prev_3) != (l_busy_1)) as i32)) != 0) {
                    F::Fade_Start(c, 0i32, 0i32, F32(0x3f800000));
                }
                if ((((!((l_busy_1) != 0)) as i32)) != 0) {
                    if ((F::Fade_IsDone(c, 0i32)) != 0) {
                        l_done_2 = 1i32;
                    }
                }
                F::Load_UpdateScreen(c);
                F::Load_DrawScreen(c);
                F::Gfx_EndFrame(c, 2i32);
                F::Dma_Flush(c);
                if ((l_done_2) != 0) {
                    break 'l1;
                }
                F::Load_ReadInput(c);
            }
        }
    }
    F::Dma_ResetBuffers(c);
    { let r_ = (!(16384i32)); let a_ = (c.m.r32(sym::gProgress)).wrapping_add(20); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
    F::PadWatch_SetEnabled(c, 1i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Load_RunBlocking", &PROBE_N, &[], true); }
    r_
}

pub fn Load_BuildSprites(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Load_BuildSprites", &PROBE_N, &[], false); }
    let mut l_spr_1: u32 = 0;
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
    { let v_ = { let a_ = (c.m.r32(sym::gProgress)).wrapping_add(16); c.m.r32(a_) }; let a_ = (sym::gLoadScreen).wrapping_add(28); c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32((sym::gLoadScreen).wrapping_add(28)); F::memset(c, a0_, 0i32, (((16i32) as u32)).wrapping_mul(56u32)) }; };
    's1: {
        let sv3: i32 = ((c.m.r16((sym::gLoadScreen).wrapping_add(18))) as i32);
        match sv3 {
            0i32 => {
                l_spr_1 = c.m.r32((sym::gLoadScreen).wrapping_add(28));
                { let v_ = 5i32; let a_ = l_spr_1; c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                { let v_ = 512i32; let a_ = (l_spr_1).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                { let v_ = 448i32; let a_ = (l_spr_1).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 34i32; let a_ = (l_spr_1).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
                { let v_ = 73i32; let a_ = (l_spr_1).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
                { let v_ = 104i32; let a_ = (l_spr_1).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
                { let v_ = 128i32; let a_ = (l_spr_1).wrapping_add(52); c.m.w32(a_, (v_) as u32); };
                { let o_ = l_spr_1; let n_ = (o_).wrapping_add(56); l_spr_1 = n_; };
                { let v_ = 2i32; let a_ = l_spr_1; c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(52); c.m.w32(a_, (v_) as u32); };
                { let o_ = l_spr_1; let n_ = (o_).wrapping_add(56); l_spr_1 = n_; };
                { let v_ = 1i32; let a_ = l_spr_1; c.m.w32(a_, (v_) as u32); };
                { let v_ = 45i32; let a_ = (l_spr_1).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                { let v_ = 265i32; let a_ = (l_spr_1).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                { let v_ = 109i32; let a_ = (l_spr_1).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                { let v_ = 393i32; let a_ = (l_spr_1).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                { let v_ = 64i32; let a_ = (l_spr_1).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                { let v_ = 128i32; let a_ = (l_spr_1).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
                { let v_ = 1i32; let a_ = (l_spr_1).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 128i32; let a_ = (l_spr_1).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
                { let v_ = 128i32; let a_ = (l_spr_1).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
                { let v_ = 128i32; let a_ = (l_spr_1).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
                { let v_ = 128i32; let a_ = (l_spr_1).wrapping_add(52); c.m.w32(a_, (v_) as u32); };
                { let o_ = l_spr_1; let n_ = (o_).wrapping_add(56); l_spr_1 = n_; };
                { let v_ = 1i32; let a_ = l_spr_1; c.m.w32(a_, (v_) as u32); };
                { let v_ = 19i32; let a_ = (l_spr_1).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                { let v_ = 318i32; let a_ = (l_spr_1).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                { let v_ = 147i32; let a_ = (l_spr_1).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                { let v_ = 425i32; let a_ = (l_spr_1).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
                { let v_ = 21i32; let a_ = (l_spr_1).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                { let v_ = 128i32; let a_ = (l_spr_1).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                { let v_ = 128i32; let a_ = (l_spr_1).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 128i32; let a_ = (l_spr_1).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
                { let v_ = 128i32; let a_ = (l_spr_1).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
                { let v_ = 128i32; let a_ = (l_spr_1).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
                { let v_ = 128i32; let a_ = (l_spr_1).wrapping_add(52); c.m.w32(a_, (v_) as u32); };
                { let o_ = l_spr_1; let n_ = (o_).wrapping_add(56); l_spr_1 = n_; };
                { let v_ = 2i32; let a_ = l_spr_1; c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(52); c.m.w32(a_, (v_) as u32); };
                { let o_ = l_spr_1; let n_ = (o_).wrapping_add(56); l_spr_1 = n_; };
                { let v_ = 1i32; let a_ = l_spr_1; c.m.w32(a_, (v_) as u32); };
                { let v_ = 113i32; let a_ = (l_spr_1).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                { let v_ = 325i32; let a_ = (l_spr_1).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                { let v_ = 136i32; let a_ = (l_spr_1).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                { let v_ = 342i32; let a_ = (l_spr_1).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                { let v_ = 23i32; let a_ = (l_spr_1).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                { let v_ = 17i32; let a_ = (l_spr_1).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 128i32; let a_ = (l_spr_1).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
                { let v_ = 128i32; let a_ = (l_spr_1).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
                { let v_ = 128i32; let a_ = (l_spr_1).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
                { let v_ = 128i32; let a_ = (l_spr_1).wrapping_add(52); c.m.w32(a_, (v_) as u32); };
                { let o_ = l_spr_1; let n_ = (o_).wrapping_add(56); l_spr_1 = n_; };
                {
                    l_i_2 = 0i32;
                    'l2: loop {
                        if !(((((l_i_2) < (9i32)) as i32)) != 0) { break 'l2; }
                        'c3: {
                            { let v_ = 1i32; let a_ = l_spr_1; c.m.w32(a_, (v_) as u32); };
                            { let v_ = 113i32; let a_ = (l_spr_1).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                            { let v_ = (319i32).wrapping_sub((l_i_2).wrapping_mul(6i32)); let a_ = (l_spr_1).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                            { let v_ = 136i32; let a_ = (l_spr_1).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                            { let v_ = (328i32).wrapping_sub((l_i_2).wrapping_mul(6i32)); let a_ = (l_spr_1).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
                            { let v_ = 32i32; let a_ = (l_spr_1).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
                            { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                            { let v_ = 55i32; let a_ = (l_spr_1).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                            { let v_ = 9i32; let a_ = (l_spr_1).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
                            { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                            { let v_ = 128i32; let a_ = (l_spr_1).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
                            { let v_ = 128i32; let a_ = (l_spr_1).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
                            { let v_ = 128i32; let a_ = (l_spr_1).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
                            { let v_ = 128i32; let a_ = (l_spr_1).wrapping_add(52); c.m.w32(a_, (v_) as u32); };
                            { let o_ = l_spr_1; let n_ = (o_).wrapping_add(56); l_spr_1 = n_; };
                        }
                        { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
                    }
                }
                { let v_ = 2i32; let a_ = l_spr_1; c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(52); c.m.w32(a_, (v_) as u32); };
                { let o_ = l_spr_1; let n_ = (o_).wrapping_add(56); l_spr_1 = n_; };
                break 's1;
            }
            1i32 => {
                l_spr_1 = c.m.r32((sym::gLoadScreen).wrapping_add(28));
                { let v_ = 5i32; let a_ = l_spr_1; c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                { let v_ = 512i32; let a_ = (l_spr_1).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                { let v_ = 448i32; let a_ = (l_spr_1).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 70i32; let a_ = (l_spr_1).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
                { let v_ = 16i32; let a_ = (l_spr_1).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
                { let v_ = 18i32; let a_ = (l_spr_1).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
                { let v_ = 128i32; let a_ = (l_spr_1).wrapping_add(52); c.m.w32(a_, (v_) as u32); };
                { let o_ = l_spr_1; let n_ = (o_).wrapping_add(56); l_spr_1 = n_; };
                { let v_ = 2i32; let a_ = l_spr_1; c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(52); c.m.w32(a_, (v_) as u32); };
                { let o_ = l_spr_1; let n_ = (o_).wrapping_add(56); l_spr_1 = n_; };
                { let v_ = 1i32; let a_ = l_spr_1; c.m.w32(a_, (v_) as u32); };
                { let v_ = 26i32; let a_ = (l_spr_1).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                { let v_ = 297i32; let a_ = (l_spr_1).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                { let v_ = 154i32; let a_ = (l_spr_1).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                { let v_ = 425i32; let a_ = (l_spr_1).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                { let v_ = 128i32; let a_ = (l_spr_1).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                { let v_ = 128i32; let a_ = (l_spr_1).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 128i32; let a_ = (l_spr_1).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
                { let v_ = 128i32; let a_ = (l_spr_1).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
                { let v_ = 128i32; let a_ = (l_spr_1).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
                { let v_ = 128i32; let a_ = (l_spr_1).wrapping_add(52); c.m.w32(a_, (v_) as u32); };
                { let o_ = l_spr_1; let n_ = (o_).wrapping_add(56); l_spr_1 = n_; };
                { let v_ = 2i32; let a_ = l_spr_1; c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(52); c.m.w32(a_, (v_) as u32); };
                { let o_ = l_spr_1; let n_ = (o_).wrapping_add(56); l_spr_1 = n_; };
                { let v_ = 1i32; let a_ = l_spr_1; c.m.w32(a_, (v_) as u32); };
                { let v_ = 121i32; let a_ = (l_spr_1).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                { let v_ = 321i32; let a_ = (l_spr_1).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                { let v_ = 188i32; let a_ = (l_spr_1).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                { let v_ = 384i32; let a_ = (l_spr_1).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
                { let v_ = 64i32; let a_ = (l_spr_1).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                { let v_ = 67i32; let a_ = (l_spr_1).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                { let v_ = 127i32; let a_ = (l_spr_1).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
                { let v_ = 2i32; let a_ = (l_spr_1).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 128i32; let a_ = (l_spr_1).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
                { let v_ = 128i32; let a_ = (l_spr_1).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
                { let v_ = 128i32; let a_ = (l_spr_1).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
                { let v_ = 128i32; let a_ = (l_spr_1).wrapping_add(52); c.m.w32(a_, (v_) as u32); };
                { let o_ = l_spr_1; let n_ = (o_).wrapping_add(56); l_spr_1 = n_; };
                { let v_ = 2i32; let a_ = l_spr_1; c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(52); c.m.w32(a_, (v_) as u32); };
                { let o_ = l_spr_1; let n_ = (o_).wrapping_add(56); l_spr_1 = n_; };
                {
                    l_i_2 = 0i32;
                    'l4: loop {
                        if !(((((l_i_2) < (3i32)) as i32)) != 0) { break 'l4; }
                        'c5: {
                            { let v_ = 1i32; let a_ = l_spr_1; c.m.w32(a_, (v_) as u32); };
                            { let v_ = (139i32).wrapping_add((l_i_2).wrapping_mul(12i32)); let a_ = (l_spr_1).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                            { let v_ = 337i32; let a_ = (l_spr_1).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                            { let v_ = (153i32).wrapping_add((l_i_2).wrapping_mul(12i32)); let a_ = (l_spr_1).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                            { let v_ = 365i32; let a_ = (l_spr_1).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
                            { let v_ = 32i32; let a_ = (l_spr_1).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
                            { let v_ = 28i32; let a_ = (l_spr_1).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                            { let v_ = 48i32; let a_ = (l_spr_1).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                            { let v_ = 56i32; let a_ = (l_spr_1).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
                            { let v_ = 2i32; let a_ = (l_spr_1).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                            { let v_ = 128i32; let a_ = (l_spr_1).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
                            { let v_ = 128i32; let a_ = (l_spr_1).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
                            { let v_ = 128i32; let a_ = (l_spr_1).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
                            { let v_ = 128i32; let a_ = (l_spr_1).wrapping_add(52); c.m.w32(a_, (v_) as u32); };
                            { let o_ = l_spr_1; let n_ = (o_).wrapping_add(56); l_spr_1 = n_; };
                        }
                        { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
                    }
                }
                { let v_ = 2i32; let a_ = l_spr_1; c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(52); c.m.w32(a_, (v_) as u32); };
                { let o_ = l_spr_1; let n_ = (o_).wrapping_add(56); l_spr_1 = n_; };
                { let _ = F::memset(c, (sym::gLoadScreen).wrapping_add(396), 0i32, 32u32); };
                { let s_ = ((c.m.r32((sym::gLoadScreen).wrapping_add(28))).wrapping_add((((4i32) as u32)).wrapping_mul(56))).wrapping_add(4); let d_ = (sym::gLoadScreen).wrapping_add(396); rt::memcpy(c, d_, s_, 16); };
                { let v_ = F32(0x3f000000); let a_ = ((sym::gLoadScreen).wrapping_add(396)).wrapping_add(24); c.m.w32(a_, (v_).0); };
                { let v_ = F32(0x3e4ccccc); let a_ = ((sym::gLoadScreen).wrapping_add(396)).wrapping_add(28); c.m.w32(a_, (v_).0); };
                break 's1;
            }
            2i32 => {
                l_spr_1 = c.m.r32((sym::gLoadScreen).wrapping_add(28));
                { let v_ = 5i32; let a_ = l_spr_1; c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                { let v_ = 512i32; let a_ = (l_spr_1).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                { let v_ = 448i32; let a_ = (l_spr_1).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 31i32; let a_ = (l_spr_1).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
                { let v_ = 82i32; let a_ = (l_spr_1).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
                { let v_ = 24i32; let a_ = (l_spr_1).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
                { let v_ = 128i32; let a_ = (l_spr_1).wrapping_add(52); c.m.w32(a_, (v_) as u32); };
                { let o_ = l_spr_1; let n_ = (o_).wrapping_add(56); l_spr_1 = n_; };
                { let v_ = 2i32; let a_ = l_spr_1; c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(52); c.m.w32(a_, (v_) as u32); };
                { let o_ = l_spr_1; let n_ = (o_).wrapping_add(56); l_spr_1 = n_; };
                { let v_ = 1i32; let a_ = l_spr_1; c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                { let v_ = 64i32; let a_ = (l_spr_1).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                { let v_ = 64i32; let a_ = (l_spr_1).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
                { let v_ = 64i32; let a_ = (l_spr_1).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
                { let v_ = 64i32; let a_ = (l_spr_1).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                { let v_ = 128i32; let a_ = (l_spr_1).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                { let v_ = 128i32; let a_ = (l_spr_1).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
                { let v_ = 1i32; let a_ = (l_spr_1).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 128i32; let a_ = (l_spr_1).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
                { let v_ = 128i32; let a_ = (l_spr_1).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
                { let v_ = 128i32; let a_ = (l_spr_1).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
                { let v_ = 128i32; let a_ = (l_spr_1).wrapping_add(52); c.m.w32(a_, (v_) as u32); };
                { let o_ = l_spr_1; let n_ = (o_).wrapping_add(56); l_spr_1 = n_; };
                { let v_ = 2i32; let a_ = l_spr_1; c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(52); c.m.w32(a_, (v_) as u32); };
                { let o_ = l_spr_1; let n_ = (o_).wrapping_add(56); l_spr_1 = n_; };
                { let v_ = 1i32; let a_ = l_spr_1; c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                { let v_ = 64i32; let a_ = (l_spr_1).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                { let v_ = 64i32; let a_ = (l_spr_1).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
                { let v_ = 64i32; let a_ = (l_spr_1).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                { let v_ = 128i32; let a_ = (l_spr_1).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                { let v_ = 64i32; let a_ = (l_spr_1).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
                { let v_ = 1i32; let a_ = (l_spr_1).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 128i32; let a_ = (l_spr_1).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
                { let v_ = 128i32; let a_ = (l_spr_1).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
                { let v_ = 128i32; let a_ = (l_spr_1).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
                { let v_ = 128i32; let a_ = (l_spr_1).wrapping_add(52); c.m.w32(a_, (v_) as u32); };
                { let o_ = l_spr_1; let n_ = (o_).wrapping_add(56); l_spr_1 = n_; };
                { let v_ = 2i32; let a_ = l_spr_1; c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(52); c.m.w32(a_, (v_) as u32); };
                { let o_ = l_spr_1; let n_ = (o_).wrapping_add(56); l_spr_1 = n_; };
                { let v_ = 1i32; let a_ = l_spr_1; c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                { let v_ = 64i32; let a_ = (l_spr_1).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                { let v_ = 128i32; let a_ = (l_spr_1).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                { let v_ = 64i32; let a_ = (l_spr_1).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                { let v_ = 128i32; let a_ = (l_spr_1).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 128i32; let a_ = (l_spr_1).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
                { let v_ = 128i32; let a_ = (l_spr_1).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
                { let v_ = 128i32; let a_ = (l_spr_1).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
                { let v_ = 128i32; let a_ = (l_spr_1).wrapping_add(52); c.m.w32(a_, (v_) as u32); };
                { let o_ = l_spr_1; let n_ = (o_).wrapping_add(56); l_spr_1 = n_; };
                { let v_ = 2i32; let a_ = l_spr_1; c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_spr_1).wrapping_add(52); c.m.w32(a_, (v_) as u32); };
                { let o_ = l_spr_1; let n_ = (o_).wrapping_add(56); l_spr_1 = n_; };
                { let _ = F::memset(c, (sym::gLoadScreen).wrapping_add(32), 0i32, 360u32); };
                {
                    l_i_2 = 0i32;
                    'l6: loop {
                        if !(((((l_i_2) < (30i32)) as i32)) != 0) { break 'l6; }
                        'c7: {
                            { let v_ = F::Rand_IntRange(c, 16i32, 412i32); let a_ = ((sym::gLoadScreen).wrapping_add(32)).wrapping_add((((((l_i_2).wrapping_mul(3i32)).wrapping_add(1i32)) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                            { let v_ = F::Rand_IntRange(c, 48i32, 352i32); let a_ = ((sym::gLoadScreen).wrapping_add(32)).wrapping_add((((((l_i_2).wrapping_mul(3i32)).wrapping_add(2i32)) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                        }
                        { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
                    }
                }
                { let v_ = ((F::Rand_Range(c, ((30i32) as u32))) as i32); let a_ = (sym::gLoadScreen).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            _ => {
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Load_BuildSprites", &PROBE_N, &[], true); }
    r_
}

pub fn Load_InitScreen(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Load_InitScreen", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let _ = F::memset(c, sym::gLoadScreen, 0i32, 428u32); };
    'l1: loop {
        'c2: {
            { let v_ = ((F::Rand_Range(c, ((3i32) as u32))) as u16); let a_ = (sym::gLoadScreen).wrapping_add(18); c.m.w16(a_, v_); };
        }
        if !((((((((c.m.r16((sym::gLoadScreen).wrapping_add(18))) as i32)).wrapping_add(1i32)) == ({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(32); (c.m.r32(a_) as i32) })) as i32)) != 0) { break 'l1; }
    }
    { let v_ = (((c.m.r16((sym::gLoadScreen).wrapping_add(18))) as i32)).wrapping_add(1i32); let a_ = (c.m.r32(sym::gProgress)).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
    { let v_ = { let a0_ = (((c.m.r16((sym::gLoadScreen).wrapping_add(18))) as i32)).wrapping_add(961i32); let a1_ = { let a_ = (c.m.r32(sym::gProgress)).wrapping_add(8); c.m.r32(a_) }; F::File_LoadSync(c, a0_, a1_, 12288i32) }; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(8); c.m.w32(a_, v_); };
    { let _ = { let a0_ = { let a_ = (c.m.r32(sym::gProgress)).wrapping_add(8); c.m.r32(a_) }; let a1_ = { let a_ = (c.m.r32(sym::gProgress)).wrapping_add(12); c.m.r32(a_) }; F::Sprite_Unpack(c, a0_, a1_, ((0i32) as u32)) }; };
    { let v_ = { let a_ = (c.m.r32(sym::gProgress)).wrapping_add(12); c.m.r32(a_) }; let a_ = (sym::gLoadScreen).wrapping_add(24); c.m.w32(a_, v_); };
    { let a1_ = c.m.r32((sym::gLoadScreen).wrapping_add(24)); let a2_ = c.m.r32((sym::gLoadScreen).wrapping_add(24)); F::Res_RelocateOffsets(c, (sym::gLoadScreen).wrapping_add(24), a1_, a2_) };
    's3: {
        let sv1: i32 = ((c.m.r16((sym::gLoadScreen).wrapping_add(18))) as i32);
        match sv1 {
            0i32 => {
                { let v_ = ((12i32) as u16); let a_ = (sym::gLoadScreen).wrapping_add(8); c.m.w16(a_, v_); };
                { let v_ = ((1i32) as i16); let a_ = (sym::gLoadScreen).wrapping_add(12); c.m.w16(a_, (v_) as u16); };
                { let v_ = ((3i32) as u16); let a_ = (sym::gLoadScreen).wrapping_add(16); c.m.w16(a_, v_); };
                break 's3;
            }
            1i32 => {
                { let v_ = ((0i32) as u16); let a_ = (sym::gLoadScreen).wrapping_add(8); c.m.w16(a_, v_); };
                { let v_ = ((2i32) as i16); let a_ = (sym::gLoadScreen).wrapping_add(12); c.m.w16(a_, (v_) as u16); };
                { let v_ = ((1i32) as u16); let a_ = (sym::gLoadScreen).wrapping_add(16); c.m.w16(a_, v_); };
                break 's3;
            }
            2i32 => {
                { let v_ = ((12i32) as u16); let a_ = (sym::gLoadScreen).wrapping_add(8); c.m.w16(a_, v_); };
                { let v_ = ((5i32) as i16); let a_ = (sym::gLoadScreen).wrapping_add(12); c.m.w16(a_, (v_) as u16); };
                { let v_ = ((5i32) as u16); let a_ = (sym::gLoadScreen).wrapping_add(16); c.m.w16(a_, v_); };
                break 's3;
            }
            _ => {
            }
        }
    }
    F::Load_BuildSprites(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Load_InitScreen", &PROBE_N, &[], true); }
    r_
}

pub fn Load_UpdateScreen(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Load_UpdateScreen", &PROBE_N, &[], false); }
    let r_ = 'f: {
    's1: {
        let sv1: i32 = ((c.m.r16((sym::gLoadScreen).wrapping_add(18))) as i32);
        match sv1 {
            0i32 => {
                if (((((((((((c.m.r16((sym::gLoadScreen).wrapping_add(8))) as u32)) < (c.m.r32((sym::gLoadScreen).wrapping_add(4)))) as i32)) != 0) || ((((((((c.m.r16((sym::gLoadScreen).wrapping_add(12)) as i16)) as i32)) == (0i32)) as i32)) != 0)) as i32)) != 0) {
                    { let v_ = ((1i32) as i16); let a_ = (sym::gLoadScreen).wrapping_add(12); c.m.w16(a_, (v_) as u16); };
                    { let r_ = 1i32; let a_ = (sym::gLoadScreen).wrapping_add(14); let n_ = ((((((c.m.r16(a_)) as i32)) ^ (r_))) as u16); c.m.w16(a_, n_); };
                    { let v_ = ((0i32) as u32); let a_ = (sym::gLoadScreen).wrapping_add(4); c.m.w32(a_, v_); };
                    if (((((((c.m.r16((sym::gLoadScreen).wrapping_add(14))) as i32)) == (0i32)) as i32)) != 0) {
                        { let a_ = (sym::gLoadScreen).wrapping_add(16); let o_ = c.m.r16(a_); let n_ = (o_).wrapping_sub(1); c.m.w16(a_, n_); };
                        if (((((((c.m.r16((sym::gLoadScreen).wrapping_add(16))) as i32)) == (0i32)) as i32)) != 0) {
                            if (((((((c.m.r16((sym::gLoadScreen).wrapping_add(10))) as i32)) < (100i32)) as i32)) != 0) {
                                { let a_ = (sym::gLoadScreen).wrapping_add(10); let o_ = c.m.r16(a_); let n_ = (o_).wrapping_add(1); c.m.w16(a_, n_); };
                            }
                            { let v_ = ((3i32) as u16); let a_ = (sym::gLoadScreen).wrapping_add(16); c.m.w16(a_, v_); };
                        }
                    }
                }
                break 's1;
            }
            1i32 => {
                if ((((((((c.m.r16((sym::gLoadScreen).wrapping_add(12)) as i16)) as i32)) == (0i32)) as i32)) != 0) {
                    { let r_ = 1i32; let a_ = (sym::gLoadScreen).wrapping_add(14); let n_ = ((((((c.m.r16(a_)) as i32)) ^ (r_))) as u16); c.m.w16(a_, n_); };
                    { let v_ = (((((c.m.r16((sym::gLoadScreen).wrapping_add(14))) as i32)).wrapping_add(2i32)) as i16); let a_ = (sym::gLoadScreen).wrapping_add(12); c.m.w16(a_, (v_) as u16); };
                    if (((((((c.m.r16((sym::gLoadScreen).wrapping_add(14))) as i32)) == (0i32)) as i32)) != 0) {
                        { let a_ = (sym::gLoadScreen).wrapping_add(16); let o_ = c.m.r16(a_); let n_ = (o_).wrapping_sub(1); c.m.w16(a_, n_); };
                        if (((((((c.m.r16((sym::gLoadScreen).wrapping_add(16))) as i32)) == (0i32)) as i32)) != 0) {
                            if (((((((c.m.r16((sym::gLoadScreen).wrapping_add(10))) as i32)) < (999i32)) as i32)) != 0) {
                                { let a_ = (sym::gLoadScreen).wrapping_add(10); let o_ = c.m.r16(a_); let n_ = (o_).wrapping_add(1); c.m.w16(a_, n_); };
                            }
                            { let v_ = ((1i32) as u16); let a_ = (sym::gLoadScreen).wrapping_add(16); c.m.w16(a_, v_); };
                        }
                    }
                }
                break 's1;
            }
            2i32 => {
                if (((((((((((c.m.r16((sym::gLoadScreen).wrapping_add(8))) as u32)) < (c.m.r32((sym::gLoadScreen).wrapping_add(4)))) as i32)) != 0) || (((((c.m.r32((sym::gLoadScreen).wrapping_add(392)) as i32)) & (2i32))) != 0)) as i32)) != 0) {
                    { let v_ = ((0i32) as u32); let a_ = (sym::gLoadScreen).wrapping_add(4); c.m.w32(a_, v_); };
                    { let r_ = 1i32; let a_ = (sym::gLoadScreen).wrapping_add(14); let n_ = ((((((c.m.r16(a_)) as i32)) ^ (r_))) as u16); c.m.w16(a_, n_); };
                    if (((((c.m.r32((sym::gLoadScreen).wrapping_add(392)) as i32)) & (1i32))) != 0) {
                        if (((((((c.m.r16((sym::gLoadScreen).wrapping_add(10))) as i32)) < (30i32)) as i32)) != 0) {
                            'l2: loop {
                                'c3: {
                                    { let v_ = ((F::Rand_Range(c, ((30i32) as u32))) as i32); let a_ = (sym::gLoadScreen).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
                                }
                                if !(((({ let a_ = ((sym::gLoadScreen).wrapping_add(32)).wrapping_add(((((((c.m.r32((sym::gLoadScreen).wrapping_add(20)) as i32)).wrapping_mul(3i32)).wrapping_add(0i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) & (1i32))) != 0) { break 'l2; }
                            }
                        } else {
                            { let v_ = ((0i32) as u16); let a_ = (sym::gLoadScreen).wrapping_add(10); c.m.w16(a_, v_); };
                            F::Load_BuildSprites(c);
                        }
                        { let r_ = (!(1i32)); let a_ = (sym::gLoadScreen).wrapping_add(392); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                    }
                }
                if ((((((((c.m.r16((sym::gLoadScreen).wrapping_add(12)) as i16)) as i32)) == (0i32)) as i32)) != 0) {
                    if (((((((c.m.r16((sym::gLoadScreen).wrapping_add(14))) as i32)) == (1i32)) as i32)) != 0) {
                        { let r_ = 1i32; let a_ = ((sym::gLoadScreen).wrapping_add(32)).wrapping_add(((((((c.m.r32((sym::gLoadScreen).wrapping_add(20)) as i32)).wrapping_mul(3i32)).wrapping_add(0i32)) as u32)).wrapping_mul(4)); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                        { let v_ = ((5i32) as i16); let a_ = (sym::gLoadScreen).wrapping_add(12); c.m.w16(a_, (v_) as u16); };
                        { let r_ = 1i32; let a_ = (sym::gLoadScreen).wrapping_add(392); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                        { let a_ = (sym::gLoadScreen).wrapping_add(10); let o_ = c.m.r16(a_); let n_ = (o_).wrapping_add(1); c.m.w16(a_, n_); };
                    }
                }
                break 's1;
            }
            _ => {
            }
        }
    }
    { let a_ = (sym::gLoadScreen).wrapping_add(4); let o_ = c.m.r32(a_); let n_ = (o_).wrapping_add(1); c.m.w32(a_, n_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Load_UpdateScreen", &PROBE_N, &[], true); }
    r_
}

pub fn Load_ReadInput(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Load_ReadInput", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if (((((c.m.r32((sym::gLoadScreen).wrapping_add(392)) as i32)) & (2i32))) != 0) {
        { let r_ = (!(2i32)); let a_ = (sym::gLoadScreen).wrapping_add(392); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
    }
    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
        { let a_ = (sym::gLoadScreen).wrapping_add(12); let o_ = (c.m.r16(a_) as i16); let n_ = (o_).wrapping_sub(1); c.m.w16(a_, (n_) as u16); };
        { let r_ = 2i32; let a_ = (sym::gLoadScreen).wrapping_add(392); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        if ((((((((c.m.r16((sym::gLoadScreen).wrapping_add(12)) as i16)) as i32)) < (0i32)) as i32)) != 0) {
            { let v_ = ((0i32) as i16); let a_ = (sym::gLoadScreen).wrapping_add(12); c.m.w16(a_, (v_) as u16); };
        }
    }
    if ((((c.m.r32(((sym::gPad).wrapping_add((((1i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
        { let a_ = (sym::gLoadScreen).wrapping_add(12); let o_ = (c.m.r16(a_) as i16); let n_ = (o_).wrapping_sub(1); c.m.w16(a_, (n_) as u16); };
        { let r_ = 2i32; let a_ = (sym::gLoadScreen).wrapping_add(392); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        if ((((((((c.m.r16((sym::gLoadScreen).wrapping_add(12)) as i16)) as i32)) < (0i32)) as i32)) != 0) {
            { let v_ = ((0i32) as i16); let a_ = (sym::gLoadScreen).wrapping_add(12); c.m.w16(a_, (v_) as u16); };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Load_ReadInput", &PROBE_N, &[], true); }
    r_
}

pub fn Load_DrawScreen(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Load_DrawScreen", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_i_1: i32 = 0;
    let mut l_spr_2: u32 = 0;
    let r_ = 'f: {
        's1: {
            let sv3: i32 = ((c.m.r16((sym::gLoadScreen).wrapping_add(18))) as i32);
            match sv3 {
                0i32 => {
                    { let a3_ = c.m.r32((sym::gLoadScreen).wrapping_add(28)); F::Sprite_DrawList(c, ((0i32) as u32), 0i32, 0i32, a3_) };
                    { let v_ = (((c.m.r16((sym::gLoadScreen).wrapping_add(14))) as i32)).wrapping_shl((6i32) as u32); let a_ = ((c.m.r32((sym::gLoadScreen).wrapping_add(28))).wrapping_add((((2i32) as u32)).wrapping_mul(56))).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
                    { let v_ = ({ let a_ = ((c.m.r32((sym::gLoadScreen).wrapping_add(28))).wrapping_add((((2i32) as u32)).wrapping_mul(56))).wrapping_add(20); (c.m.r32(a_) as i32) }).wrapping_add(64i32); let a_ = ((c.m.r32((sym::gLoadScreen).wrapping_add(28))).wrapping_add((((2i32) as u32)).wrapping_mul(56))).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                    { let a0_ = c.m.r32((sym::gLoadScreen).wrapping_add(24)); let a3_ = (c.m.r32((sym::gLoadScreen).wrapping_add(28))).wrapping_add((((2i32) as u32)).wrapping_mul(56)); F::Sprite_DrawList(c, a0_, 0i32, 0i32, a3_) };
                    {
                        l_i_1 = 0i32;
                        'l2: loop {
                            if !(((((l_i_1) < (10i32)) as i32)) != 0) { break 'l2; }
                            'c3: {
                                { let v_ = 0i32; let a_ = (c.m.r32((sym::gLoadScreen).wrapping_add(28))).wrapping_add(((((5i32).wrapping_add(l_i_1)) as u32)).wrapping_mul(56)); c.m.w32(a_, (v_) as u32); };
                            }
                            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
                        }
                    }
                    {
                        l_i_1 = 0i32;
                        'l4: loop {
                            if !(((((l_i_1) < (rt::rem_i32(((c.m.r16((sym::gLoadScreen).wrapping_add(10))) as i32), 10i32))) as i32)) != 0) { break 'l4; }
                            'c5: {
                                { let v_ = 1i32; let a_ = (c.m.r32((sym::gLoadScreen).wrapping_add(28))).wrapping_add(((((5i32).wrapping_add(l_i_1)) as u32)).wrapping_mul(56)); c.m.w32(a_, (v_) as u32); };
                            }
                            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
                        }
                    }
                    { let a0_ = c.m.r32((sym::gLoadScreen).wrapping_add(24)); let a3_ = (c.m.r32((sym::gLoadScreen).wrapping_add(28))).wrapping_add((((5i32) as u32)).wrapping_mul(56)); F::Sprite_DrawList(c, a0_, 0i32, 0i32, a3_) };
                    {
                        l_i_1 = 0i32;
                        'l6: loop {
                            if !(((((l_i_1) < (10i32)) as i32)) != 0) { break 'l6; }
                            'c7: {
                                { let v_ = 1i32; let a_ = (c.m.r32((sym::gLoadScreen).wrapping_add(28))).wrapping_add(((((5i32).wrapping_add(l_i_1)) as u32)).wrapping_mul(56)); c.m.w32(a_, (v_) as u32); };
                            }
                            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
                        }
                    }
                    {
                        l_i_1 = 0i32;
                        'l8: loop {
                            if !(((((l_i_1) < (rt::div_i32(((c.m.r16((sym::gLoadScreen).wrapping_add(10))) as i32), 10i32))) as i32)) != 0) { break 'l8; }
                            'c9: {
                                { let a0_ = c.m.r32((sym::gLoadScreen).wrapping_add(24)); let a3_ = (c.m.r32((sym::gLoadScreen).wrapping_add(28))).wrapping_add((((5i32) as u32)).wrapping_mul(56)); F::Sprite_DrawList(c, a0_, (30i32).wrapping_add((l_i_1).wrapping_mul(25i32)), 60i32, a3_) };
                            }
                            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
                        }
                    }
                    break 's1;
                }
                1i32 => {
                    { let a3_ = c.m.r32((sym::gLoadScreen).wrapping_add(28)); F::Sprite_DrawList(c, ((0i32) as u32), 0i32, 0i32, a3_) };
                    { let v_ = ((c.m.r16((sym::gLoadScreen).wrapping_add(14))) as i32); let a_ = ((c.m.r32((sym::gLoadScreen).wrapping_add(28))).wrapping_add((((2i32) as u32)).wrapping_mul(56))).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                    { let a0_ = c.m.r32((sym::gLoadScreen).wrapping_add(24)); let a3_ = (c.m.r32((sym::gLoadScreen).wrapping_add(28))).wrapping_add((((2i32) as u32)).wrapping_mul(56)); F::Sprite_DrawList(c, a0_, 0i32, 0i32, a3_) };
                    { let r_ = rt::fmul({ let a0_ = F32(c.m.r32(((sym::gLoadScreen).wrapping_add(396)).wrapping_add(20))); F::Mathf_Sin(c, a0_) }, F32(c.m.r32(((sym::gLoadScreen).wrapping_add(396)).wrapping_add(24)))); let a_ = ((sym::gLoadScreen).wrapping_add(396)).wrapping_add(16); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    { let r_ = F32(c.m.r32(((sym::gLoadScreen).wrapping_add(396)).wrapping_add(28))); let a_ = ((sym::gLoadScreen).wrapping_add(396)).wrapping_add(20); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    if (((rt::fge(F32(c.m.r32(((sym::gLoadScreen).wrapping_add(396)).wrapping_add(20))), F32(0x40490fda)) as i32)) != 0) {
                        { let r_ = F32(0x40c90fda); let a_ = ((sym::gLoadScreen).wrapping_add(396)).wrapping_add(20); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    }
                    { let v_ = ((c.m.r32(((sym::gLoadScreen).wrapping_add(396)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)).wrapping_sub(rt::f2i(F32(c.m.r32(((sym::gLoadScreen).wrapping_add(396)).wrapping_add(16))))); let a_ = ((c.m.r32((sym::gLoadScreen).wrapping_add(28))).wrapping_add((((4i32) as u32)).wrapping_mul(56))).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = ((c.m.r32(((sym::gLoadScreen).wrapping_add(396)).wrapping_add((((3i32) as u32)).wrapping_mul(4))) as i32)).wrapping_add(rt::f2i(F32(c.m.r32(((sym::gLoadScreen).wrapping_add(396)).wrapping_add(16))))); let a_ = ((c.m.r32((sym::gLoadScreen).wrapping_add(28))).wrapping_add((((4i32) as u32)).wrapping_mul(56))).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
                    { let v_ = ((c.m.r32(((sym::gLoadScreen).wrapping_add(396)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)).wrapping_sub(rt::f2i(F32(c.m.r32(((sym::gLoadScreen).wrapping_add(396)).wrapping_add(16))))); let a_ = ((c.m.r32((sym::gLoadScreen).wrapping_add(28))).wrapping_add((((4i32) as u32)).wrapping_mul(56))).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    { let v_ = ((c.m.r32(((sym::gLoadScreen).wrapping_add(396)).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32)).wrapping_add(rt::f2i(F32(c.m.r32(((sym::gLoadScreen).wrapping_add(396)).wrapping_add(16))))); let a_ = ((c.m.r32((sym::gLoadScreen).wrapping_add(28))).wrapping_add((((4i32) as u32)).wrapping_mul(56))).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                    { let a0_ = c.m.r32((sym::gLoadScreen).wrapping_add(24)); let a3_ = (c.m.r32((sym::gLoadScreen).wrapping_add(28))).wrapping_add((((4i32) as u32)).wrapping_mul(56)); F::Sprite_DrawList(c, a0_, 0i32, 0i32, a3_) };
                    if (((((((c.m.r16((sym::gLoadScreen).wrapping_add(10))) as i32)) != (0i32)) as i32)) != 0) {
                        { let a1_ = ((c.m.r16((sym::gLoadScreen).wrapping_add(10))) as i32); F::Num_ToDigits(c, fp, a1_, 3i32, 0i32) };
                        {
                            l_i_1 = 0i32;
                            'l10: loop {
                                if !(((((l_i_1) < (3i32)) as i32)) != 0) { break 'l10; }
                                'c11: {
                                    l_spr_2 = (c.m.r32((sym::gLoadScreen).wrapping_add(28))).wrapping_add(((((6i32).wrapping_add(l_i_1)) as u32)).wrapping_mul(56));
                                    { let v_ = (((((c.m.r8((fp).wrapping_add(((l_i_1) as u32)))) as i32)) != (10i32)) as i32); let a_ = l_spr_2; c.m.w32(a_, (v_) as u32); };
                                    { let v_ = ((((c.m.r8((fp).wrapping_add(((l_i_1) as u32)))) as i32)).wrapping_shr((3i32) as u32)).wrapping_mul(28i32); let a_ = (l_spr_2).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                                    { let v_ = ((c.m.r32((l_spr_2).wrapping_add(24)) as i32)).wrapping_add(28i32); let a_ = (l_spr_2).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
                                    { let v_ = (((((c.m.r8((fp).wrapping_add(((l_i_1) as u32)))) as i32)) & (7i32))).wrapping_shl((4i32) as u32); let a_ = (l_spr_2).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
                                    { let v_ = ((c.m.r32((l_spr_2).wrapping_add(20)) as i32)).wrapping_add(16i32); let a_ = (l_spr_2).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                                }
                                { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
                            }
                        }
                        's12: {
                            let sv4: i32 = { let a0_ = ((c.m.r16((sym::gLoadScreen).wrapping_add(10))) as i32); F::Num_CountDigits(c, a0_) };
                            match sv4 {
                                1i32 => {
                                    { let a0_ = c.m.r32((sym::gLoadScreen).wrapping_add(24)); let a3_ = (c.m.r32((sym::gLoadScreen).wrapping_add(28))).wrapping_add((((6i32) as u32)).wrapping_mul(56)); F::Sprite_DrawList(c, a0_, (14i32).wrapping_neg(), 0i32, a3_) };
                                    break 's12;
                                }
                                2i32 => {
                                    { let a0_ = c.m.r32((sym::gLoadScreen).wrapping_add(24)); let a3_ = (c.m.r32((sym::gLoadScreen).wrapping_add(28))).wrapping_add((((6i32) as u32)).wrapping_mul(56)); F::Sprite_DrawList(c, a0_, (7i32).wrapping_neg(), 0i32, a3_) };
                                    break 's12;
                                }
                                _ => {
                                    { let a0_ = c.m.r32((sym::gLoadScreen).wrapping_add(24)); let a3_ = (c.m.r32((sym::gLoadScreen).wrapping_add(28))).wrapping_add((((6i32) as u32)).wrapping_mul(56)); F::Sprite_DrawList(c, a0_, 0i32, 0i32, a3_) };
                                    break 's12;
                                }
                            }
                        }
                    } else {
                        { let a0_ = c.m.r32((sym::gLoadScreen).wrapping_add(24)); let a3_ = (c.m.r32((sym::gLoadScreen).wrapping_add(28))).wrapping_add((((6i32) as u32)).wrapping_mul(56)); F::Sprite_DrawList(c, a0_, 0i32, 0i32, a3_) };
                    }
                    break 's1;
                }
                2i32 => {
                    { let a3_ = c.m.r32((sym::gLoadScreen).wrapping_add(28)); F::Sprite_DrawList(c, ((0i32) as u32), 0i32, 0i32, a3_) };
                    {
                        l_i_1 = 0i32;
                        'l13: loop {
                            if !(((((l_i_1) < (30i32)) as i32)) != 0) { break 'l13; }
                            'c14: {
                                if ((((((((((((((c.m.r32((sym::gLoadScreen).wrapping_add(20)) as i32)) != (l_i_1)) as i32)) != 0) && ((((!(((((c.m.r32(((sym::gLoadScreen).wrapping_add(32)).wrapping_add(((((l_i_1).wrapping_mul(3i32)) as u32)).wrapping_mul(4))) as i32)) & (1i32))) != 0)) as i32)) != 0)) as i32)) != 0) && (((((((c.m.r32((((sym::gLoadScreen).wrapping_add(4)).wrapping_add(32)).wrapping_add((((((l_i_1).wrapping_mul(3i32)).wrapping_add(1i32)) as u32)).wrapping_mul(4))) as i32)).wrapping_add(32i32)) <= (({ let a_ = (((sym::gLoadScreen).wrapping_add(4)).wrapping_add(32)).wrapping_add(((((((c.m.r32((sym::gLoadScreen).wrapping_add(20)) as i32)).wrapping_mul(3i32)).wrapping_add(1i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(64i32))) as i32)) != 0)) as i32)) != 0) {
                                    { let a0_ = c.m.r32((sym::gLoadScreen).wrapping_add(24)); let a1_ = (c.m.r32((((sym::gLoadScreen).wrapping_add(4)).wrapping_add(32)).wrapping_add(((((l_i_1).wrapping_mul(3i32)) as u32)).wrapping_mul(4))) as i32); let a2_ = (c.m.r32((((sym::gLoadScreen).wrapping_add(4)).wrapping_add(32)).wrapping_add((((((l_i_1).wrapping_mul(3i32)).wrapping_add(1i32)) as u32)).wrapping_mul(4))) as i32); let a3_ = (c.m.r32((sym::gLoadScreen).wrapping_add(28))).wrapping_add((((4i32) as u32)).wrapping_mul(56)); F::Sprite_DrawList(c, a0_, a1_, a2_, a3_) };
                                }
                            }
                            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
                        }
                    }
                    if (((((c.m.r32((sym::gLoadScreen).wrapping_add(392)) as i32)) & (1i32))) != 0) {
                        { let v_ = 0i32; let a_ = ((c.m.r32((sym::gLoadScreen).wrapping_add(28))).wrapping_add((((6i32) as u32)).wrapping_mul(56))).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
                        { let v_ = ({ let a_ = ((c.m.r32((sym::gLoadScreen).wrapping_add(28))).wrapping_add((((6i32) as u32)).wrapping_mul(56))).wrapping_add(20); (c.m.r32(a_) as i32) }).wrapping_add(64i32); let a_ = ((c.m.r32((sym::gLoadScreen).wrapping_add(28))).wrapping_add((((6i32) as u32)).wrapping_mul(56))).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                        { let v_ = 1i32; let a_ = ((c.m.r32((sym::gLoadScreen).wrapping_add(28))).wrapping_add((((6i32) as u32)).wrapping_mul(56))).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                        { let a0_ = c.m.r32((sym::gLoadScreen).wrapping_add(24)); let a1_ = { let a_ = (((sym::gLoadScreen).wrapping_add(4)).wrapping_add(32)).wrapping_add((((((c.m.r32((sym::gLoadScreen).wrapping_add(20)) as i32)).wrapping_mul(3i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a2_ = { let a_ = (((sym::gLoadScreen).wrapping_add(4)).wrapping_add(32)).wrapping_add(((((((c.m.r32((sym::gLoadScreen).wrapping_add(20)) as i32)).wrapping_mul(3i32)).wrapping_add(1i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a3_ = (c.m.r32((sym::gLoadScreen).wrapping_add(28))).wrapping_add((((2i32) as u32)).wrapping_mul(56)); F::Sprite_DrawList(c, a0_, a1_, a2_, a3_) };
                    } else {
                        { let v_ = (((c.m.r16((sym::gLoadScreen).wrapping_add(14))) as i32)).wrapping_shl((6i32) as u32); let a_ = ((c.m.r32((sym::gLoadScreen).wrapping_add(28))).wrapping_add((((6i32) as u32)).wrapping_mul(56))).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
                        { let v_ = ({ let a_ = ((c.m.r32((sym::gLoadScreen).wrapping_add(28))).wrapping_add((((6i32) as u32)).wrapping_mul(56))).wrapping_add(20); (c.m.r32(a_) as i32) }).wrapping_add(64i32); let a_ = ((c.m.r32((sym::gLoadScreen).wrapping_add(28))).wrapping_add((((6i32) as u32)).wrapping_mul(56))).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                        { let v_ = 0i32; let a_ = ((c.m.r32((sym::gLoadScreen).wrapping_add(28))).wrapping_add((((6i32) as u32)).wrapping_mul(56))).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                    }
                    { let a0_ = c.m.r32((sym::gLoadScreen).wrapping_add(24)); let a1_ = { let a_ = (((sym::gLoadScreen).wrapping_add(4)).wrapping_add(32)).wrapping_add((((((c.m.r32((sym::gLoadScreen).wrapping_add(20)) as i32)).wrapping_mul(3i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a2_ = ({ let a_ = (((sym::gLoadScreen).wrapping_add(4)).wrapping_add(32)).wrapping_add(((((((c.m.r32((sym::gLoadScreen).wrapping_add(20)) as i32)).wrapping_mul(3i32)).wrapping_add(1i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_sub(64i32); let a3_ = (c.m.r32((sym::gLoadScreen).wrapping_add(28))).wrapping_add((((6i32) as u32)).wrapping_mul(56)); F::Sprite_DrawList(c, a0_, a1_, a2_, a3_) };
                    {
                        l_i_1 = 0i32;
                        'l15: loop {
                            if !(((((l_i_1) < (30i32)) as i32)) != 0) { break 'l15; }
                            'c16: {
                                if ((((((((((((((c.m.r32((sym::gLoadScreen).wrapping_add(20)) as i32)) != (l_i_1)) as i32)) != 0) && ((((!(((((c.m.r32(((sym::gLoadScreen).wrapping_add(32)).wrapping_add(((((l_i_1).wrapping_mul(3i32)) as u32)).wrapping_mul(4))) as i32)) & (1i32))) != 0)) as i32)) != 0)) as i32)) != 0) && (((((((c.m.r32((((sym::gLoadScreen).wrapping_add(4)).wrapping_add(32)).wrapping_add((((((l_i_1).wrapping_mul(3i32)).wrapping_add(1i32)) as u32)).wrapping_mul(4))) as i32)).wrapping_add(32i32)) > (({ let a_ = (((sym::gLoadScreen).wrapping_add(4)).wrapping_add(32)).wrapping_add(((((((c.m.r32((sym::gLoadScreen).wrapping_add(20)) as i32)).wrapping_mul(3i32)).wrapping_add(1i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(64i32))) as i32)) != 0)) as i32)) != 0) {
                                    { let a0_ = c.m.r32((sym::gLoadScreen).wrapping_add(24)); let a1_ = (c.m.r32((((sym::gLoadScreen).wrapping_add(4)).wrapping_add(32)).wrapping_add(((((l_i_1).wrapping_mul(3i32)) as u32)).wrapping_mul(4))) as i32); let a2_ = (c.m.r32((((sym::gLoadScreen).wrapping_add(4)).wrapping_add(32)).wrapping_add((((((l_i_1).wrapping_mul(3i32)).wrapping_add(1i32)) as u32)).wrapping_mul(4))) as i32); let a3_ = (c.m.r32((sym::gLoadScreen).wrapping_add(28))).wrapping_add((((4i32) as u32)).wrapping_mul(56)); F::Sprite_DrawList(c, a0_, a1_, a2_, a3_) };
                                }
                            }
                            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
                        }
                    }
                    break 's1;
                }
                _ => {
                }
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Load_DrawScreen", &PROBE_N, &[], true); }
    r_
}

pub fn Load_RunFileQueue(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Load_RunFileQueue", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::PadWatch_SetEnabled(c, 0i32);
    F::Dma_ResetBuffers(c);
    F::Load_InitScreen(c);
    {
        'l1: loop {
            'c2: {
                F::Gfx_BeginFrame(c);
                F::Pad_Update(c);
                F::Load_UpdateScreen(c);
                F::Load_DrawScreen(c);
                F::Gfx_EndFrame(c, 2i32);
                F::Dma_Flush(c);
                F::File_Stub264D90(c);
                { let a_ = sym::gLoadScreen; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                if ((F::File_UpdateRequests(c)) != 0) {
                    break 'l1;
                }
                F::Load_ReadInput(c);
            }
        }
    }
    F::Dma_ResetBuffers(c);
    F::PadWatch_SetEnabled(c, 1i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Load_RunFileQueue", &PROBE_N, &[], true); }
    r_
}
