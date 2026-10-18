#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn Logo_Init(c: &mut Ctx, mut p_pack_1: u32, mut p_section_2: i32, mut p_frames_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Logo_Init", &PROBE_N, &[(p_pack_1) as u32, (p_section_2) as u32, (p_frames_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        { let v_ = ((0i32) as u32); c.m.w32(fp, v_); }
        { let v_ = F::Heap_Alloc(c, 36i32, ((32i32) as u32), 0i32, 2i32); let a_ = sym::gLogo; c.m.w32(a_, v_); };
        { let _ = { let a0_ = c.m.r32(sym::gLogo); F::memset(c, a0_, 0i32, ((36i32) as u32)) }; };
        { let v_ = (p_pack_1).wrapping_add(((c.m.r32((p_pack_1).wrapping_add((((p_section_2) as u32)).wrapping_mul(4)))).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = c.m.r32(fp); let a_ = c.m.r32(sym::gLogo); c.m.w32(a_, v_); };
        { let v_ = p_frames_3; let a_ = (c.m.r32(sym::gLogo)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Logo_Init", &PROBE_N, &[], true); }
    r_
}

pub fn Logo_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Logo_Term", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if (((((c.m.r32(sym::gLogo)) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32(sym::gLogo); F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = sym::gLogo; c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Logo_Term", &PROBE_N, &[], true); }
    r_
}

pub fn Logo_Show(c: &mut Ctx, mut p_pack_1: u32, mut p_section_2: i32, mut p_frames_3: i32, mut p_fade_4: i32, mut p_skip_5: i32, mut p_inBlack_6: i32, mut p_outBlack_7: i32, mut p_fadeFirst_8: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Logo_Show", &PROBE_N, &[(p_pack_1) as u32, (p_section_2) as u32, (p_frames_3) as u32, (p_fade_4) as u32, (p_skip_5) as u32, (p_inBlack_6) as u32, (p_outBlack_7) as u32, (p_fadeFirst_8) as u32], false); }
    let r_ = 'f: {
    F::Logo_Init(c, p_pack_1, p_section_2, p_frames_3);
    if ((p_fadeFirst_8) != 0) {
        if ((p_inBlack_6) != 0) {
            F::ColorFade_StartOut(c, 0i32, 0i32, 0i32, p_fade_4);
        } else {
            F::ColorFade_StartOut(c, 255i32, 255i32, 255i32, p_fade_4);
        }
        'l1: loop {
            'c2: {
                'l3: loop {
                    'c4: {
                        F::Gfx_BeginFrame(c);
                        F::Pad_Update(c);
                        F::ColorFade_Update(c);
                        F::File_WaitPartitionTimeout(c, 2i32);
                        F::ColorFade_Draw(c);
                        F::Gfx_EndFrame(c, 1i32);
                        F::Dma_Flush(c);
                        F::File_Stub264D90(c);
                    }
                    if !((F::ColorFade_IsFadingOut(c)) != 0) { break 'l3; }
                }
            }
            if !((((!((F::ColorFade_IsOutDone(c)) != 0)) as i32)) != 0) { break 'l1; }
        }
    }
    if ((p_inBlack_6) != 0) {
        F::ColorFade_StartIn(c, 0i32, 0i32, 0i32, p_fade_4);
    } else {
        F::ColorFade_StartIn(c, 255i32, 255i32, 255i32, p_fade_4);
    }
    'l5: while ((1i32) != 0) {
        F::Gfx_BeginFrame(c);
        F::Pad_Update(c);
        F::ColorFade_Update(c);
        { let a0_ = { let a_ = c.m.r32(sym::gLogo); c.m.r32(a_) }; F::Sprite_DrawPicture(c, a0_, 0i32, 0i32, 128i32) };
        F::File_WaitPartitionTimeout(c, 2i32);
        F::ColorFade_Draw(c);
        F::Gfx_EndFrame(c, 1i32);
        F::Dma_Flush(c);
        F::File_Stub264D90(c);
        if ((F::ColorFade_IsInDone(c)) != 0) {
            { let r_ = 1i32; let a_ = (c.m.r32(sym::gLogo)).wrapping_add(8); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        }
        if ((F::ColorFade_IsFadingOut(c)) != 0) {
            continue 'l5;
        }
        if ((F::ColorFade_IsOutDone(c)) != 0) {
            break 'l5;
        }
        if (((({ let a_ = (c.m.r32(sym::gLogo)).wrapping_add(8); (c.m.r32(a_) as i32) }) & (1i32))) != 0) {
            { let a_ = (c.m.r32(sym::gLogo)).wrapping_add(16); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
            if ((((({ let a_ = (c.m.r32(sym::gLogo)).wrapping_add(16); (c.m.r32(a_) as i32) }) >= (({ let a_ = (c.m.r32(sym::gLogo)).wrapping_add(12); (c.m.r32(a_) as i32) }).wrapping_add(2i32))) as i32)) != 0) {
                if ((p_outBlack_7) != 0) {
                    F::ColorFade_StartOut(c, 0i32, 0i32, 0i32, p_fade_4);
                } else {
                    F::ColorFade_StartOut(c, 255i32, 255i32, 255i32, p_fade_4);
                }
            }
        }
        's6: {
            let sv9: i32 = p_skip_5;
            match sv9 {
                1i32 => {
                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((((4096i32) | (512i32))) as u32)))) != 0) {
                        if ((p_outBlack_7) != 0) {
                            F::ColorFade_StartOut(c, 0i32, 0i32, 0i32, rt::div_i32(p_fade_4, 2i32));
                        } else {
                            F::ColorFade_StartOut(c, 255i32, 255i32, 255i32, rt::div_i32(p_fade_4, 2i32));
                        }
                    }
                    break 's6;
                }
                2i32 => {
                    if ((((({ let a_ = (c.m.r32(sym::gLogo)).wrapping_add(16); (c.m.r32(a_) as i32) }) >= (60i32)) as i32)) != 0) {
                        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((((4096i32) | (512i32))) as u32)))) != 0) {
                            if ((p_outBlack_7) != 0) {
                                F::ColorFade_StartOut(c, 0i32, 0i32, 0i32, p_fade_4);
                            } else {
                                F::ColorFade_StartOut(c, 255i32, 255i32, 255i32, p_fade_4);
                            }
                        }
                    }
                    break 's6;
                }
                _ => {
                }
            }
        }
    }
    { let _ = F::sceGsSyncPath(c, 0i32, ((0i32) as u16)); };
    F::Logo_Term(c);
    F::Dma_ResetBuffers(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Logo_Show", &PROBE_N, &[], true); }
    r_
}

pub fn Logo_ShowAll(c: &mut Ctx, mut p_pack_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Logo_ShowAll", &PROBE_N, &[(p_pack_1) as u32], false); }
    let r_ = 'f: {
    F::Logo_Show(c, p_pack_1, 6i32, 120i32, 60i32, 1i32, 1i32, 0i32, 0i32);
    F::Logo_Show(c, p_pack_1, 7i32, 120i32, 60i32, 1i32, 0i32, 0i32, 0i32);
    F::Logo_Show(c, p_pack_1, 8i32, 120i32, 60i32, 1i32, 0i32, 0i32, 0i32);
    F::Logo_Show(c, p_pack_1, 3i32, 120i32, 60i32, 1i32, 0i32, 0i32, 0i32);
    F::Logo_Show(c, p_pack_1, 4i32, 120i32, 60i32, 1i32, 0i32, 0i32, 0i32);
    F::Logo_Show(c, p_pack_1, 5i32, 120i32, 60i32, 0i32, 0i32, 1i32, 0i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Logo_ShowAll", &PROBE_N, &[], true); }
    r_
}

pub fn FirstRun_WaitPartition(c: &mut Ctx, mut p_pt_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "FirstRun_WaitPartition", &PROBE_N, &[(p_pt_1) as u32], false); }
    let mut l_flags_2: i32 = 0;
    let r_ = 'f: {
    l_flags_2 = 0i32;
    if ((F::File_IsPartitionLoaded(c, p_pt_1)) != 0) {
        break 'f;
    }
    F::Dma_ResetBuffers(c);
    F::Load_InitScreen(c);
    F::ColorFade_StartIn(c, 0i32, 0i32, 0i32, 15i32);
    'l1: while ((1i32) != 0) {
        F::Snd_Update(c);
        F::Gfx_BeginFrame(c);
        F::Pad_Update(c);
        F::ColorFade_Update(c);
        F::File_WaitPartitionTimeout(c, p_pt_1);
        if ((F::File_IsPartitionLoaded(c, p_pt_1)) != 0) {
            if ((((l_flags_2) & (1i32))) != 0) {
                if ((((!((((l_flags_2) & (2i32))) != 0)) as i32)) != 0) {
                    F::ColorFade_StartOut(c, 0i32, 0i32, 0i32, 15i32);
                    { let r_ = 2i32; let n_ = ((l_flags_2) | (r_)); l_flags_2 = n_; };
                }
            }
        }
        F::Load_UpdateScreen(c);
        F::Load_DrawScreen(c);
        F::ColorFade_Draw(c);
        F::Gfx_EndFrame(c, 2i32);
        F::Dma_Flush(c);
        if ((F::ColorFade_IsInDone(c)) != 0) {
            { let r_ = 1i32; let n_ = ((l_flags_2) | (r_)); l_flags_2 = n_; };
        }
        if ((F::ColorFade_IsOutDone(c)) != 0) {
            break 'l1;
        }
        F::Load_ReadInput(c);
    }
    F::Dma_ResetBuffers(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "FirstRun_WaitPartition", &PROBE_N, &[], true); }
    r_
}

pub fn FirstRun_Main(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "FirstRun_Main", &PROBE_N, &[], false); }
    let mut l_res_1: u32 = 0;
    let r_ = 'f: {
    { let _ = F::File_LoadPartitionNw(c, 2i32); };
    { let _ = { let _ = 0i32; let _ = 0i32; F::FirstRun_WaitPad(c) }; };
    F::PadWatch_SetEnabled(c, 0i32);
    if (((((c.m.r32(sym::gMenuArc0)) == (((0i32) as u32))) as i32)) != 0) {
        { let v_ = { let a0_ = ({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_add(25i32); F::File_LoadSync(c, a0_, ((0i32) as u32), 0i32) }; let a_ = sym::gMenuArc0; c.m.w32(a_, v_); };
    }
    { let _ = F::BootCard_Run(c, 2i32); };
    l_res_1 = { let a0_ = (c.m.r32(sym::gMenuArc0)).wrapping_add((({ let a_ = (c.m.r32(sym::gMenuArc0)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); F::Sprite_Unpack(c, a0_, ((0i32) as u32), ((0i32) as u32)) };
    F::Logo_ShowAll(c, l_res_1);
    if (((((l_res_1) != (((0i32) as u32))) as i32)) != 0) {
        F::Heap_Free(c, l_res_1);
    }
    if (((((c.m.r32(sym::gMenuArc0)) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32(sym::gMenuArc0); F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = sym::gMenuArc0; c.m.w32(a_, v_); };
    }
    F::FirstRun_WaitPartition(c, 2i32);
    F::Movie_PlayOpening(c);
    F::PadWatch_SetEnabled(c, 1i32);
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "FirstRun_Main", &PROBE_N, &[], true); }
    r_
}

pub fn FirstRun_WaitPad(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "FirstRun_WaitPad", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::Pad_Update(c);
    if ((((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(384)) as i32)) == (255i32)) as i32)) != 0) {
        'l1: loop {
            'c2: {
                F::Gfx_BeginFrame(c);
                F::Pad_Update(c);
                F::File_WaitPartitionTimeout(c, 2i32);
                F::Gfx_EndFrame(c, 1i32);
                F::Dma_Flush(c);
                F::File_Stub264D90(c);
            }
            if !((((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(384)) as i32)) == (255i32)) as i32)) != 0) { break 'l1; }
        }
        F::Dma_ResetBuffers(c);
    }
    { let _ = F::sceGsSyncPath(c, 0i32, ((0i32) as u16)); };
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "FirstRun_WaitPad", &PROBE_N, &[], true); }
    r_
}
