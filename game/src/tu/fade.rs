#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn Fade_DrawRect(c: &mut Ctx, mut p_fade_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Fade_DrawRect", &PROBE_N, &[(p_fade_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_rgba_2: u32 = 0;
    let mut l_i_3: i32 = 0;
    let r_ = 'f: {
        { let v_ = F::Dma_BeginDirect(c); let a_ = fp; c.m.w32(a_, v_); };
        F::Gfx_PutDefaultEnv(c, fp);
        l_rgba_2 = ((((((((rt::f2u(F32(c.m.r32((p_fade_1).wrapping_add((((0i32) as u32)).wrapping_mul(4)))))) & (((255i32) as u32)))) | ((((rt::f2u(F32(c.m.r32((p_fade_1).wrapping_add((((1i32) as u32)).wrapping_mul(4)))))) & (((255i32) as u32)))).wrapping_shl((8i32) as u32)))) | ((((rt::f2u(F32(c.m.r32((p_fade_1).wrapping_add((((2i32) as u32)).wrapping_mul(4)))))) & (((255i32) as u32)))).wrapping_shl((16i32) as u32)))) | ((rt::f2u(F32(c.m.r32((p_fade_1).wrapping_add((((3i32) as u32)).wrapping_mul(4)))))).wrapping_shl((24i32) as u32)));
        { let v_ = ((((((((4i32) as u64)) | ((((0i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (c.m.r32(fp)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((14i32) as u64); let a_ = (c.m.r32(fp)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let a_ = fp; let n_ = (c.m.r32(a_)).wrapping_add((((r_) as u32)).wrapping_mul(8)); c.m.w32(a_, n_); };
        { let v_ = ((196608i32) as u64); let a_ = (c.m.r32(fp)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((71i32) as u64); let a_ = (c.m.r32(fp)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let a_ = fp; let n_ = (c.m.r32(a_)).wrapping_add((((r_) as u32)).wrapping_mul(8)); c.m.w32(a_, n_); };
        { let v_ = ((((((224i32) as u64)) | ((((49i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((32i32) as u32))); let a_ = (c.m.r32(fp)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((78i32) as u64); let a_ = (c.m.r32(fp)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let a_ = fp; let n_ = (c.m.r32(a_)).wrapping_add((((r_) as u32)).wrapping_mul(8)); c.m.w32(a_, n_); };
        { let v_ = ((70i32) as u64); let a_ = (c.m.r32(fp)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((0i32) as u64); let a_ = (c.m.r32(fp)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let a_ = fp; let n_ = (c.m.r32(a_)).wrapping_add((((r_) as u32)).wrapping_mul(8)); c.m.w32(a_, n_); };
        { let v_ = ((((l_rgba_2) as u64)) | ((1065353216u64).wrapping_shl((32i32) as u32))); let a_ = (c.m.r32(fp)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((1i32) as u64); let a_ = (c.m.r32(fp)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let a_ = fp; let n_ = (c.m.r32(a_)).wrapping_add((((r_) as u32)).wrapping_mul(8)); c.m.w32(a_, n_); };
        { let v_ = ((((((((16i32) as u64)) | ((((1i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((2i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (c.m.r32(fp)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((((5i32) | ((5i32).wrapping_shl((4i32) as u32)))) as u64); let a_ = (c.m.r32(fp)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let a_ = fp; let n_ = (c.m.r32(a_)).wrapping_add((((r_) as u32)).wrapping_mul(8)); c.m.w32(a_, n_); };
        {
            l_i_3 = 0i32;
            'l1: loop {
                if !(((((l_i_3) < (16i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let v_ = (((((((l_i_3).wrapping_mul(32i32)).wrapping_add(1792i32)).wrapping_shl((4i32) as u32)) | (((1824i32).wrapping_shl((4i32) as u32)).wrapping_shl((16i32) as u32)))) as u64); let a_ = (c.m.r32(fp)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                    { let v_ = ((((((((((l_i_3).wrapping_mul(32i32)).wrapping_add(1792i32)).wrapping_add(32i32)).wrapping_shl((4i32) as u32)) as u64)) | ((((((1824i32).wrapping_add(448i32)).wrapping_shl((4i32) as u32)) as u64)).wrapping_shl((16i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((32i32) as u32))); let a_ = (c.m.r32(fp)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                    { let r_ = 2i32; let a_ = fp; let n_ = (c.m.r32(a_)).wrapping_add((((r_) as u32)).wrapping_mul(8)); c.m.w32(a_, n_); };
                }
                { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
            }
        }
        { let a0_ = c.m.r32(fp); F::Dma_EndDirect(c, a0_) };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Fade_DrawRect", &PROBE_N, &[], true); }
    r_
}

pub fn FadeSlot_Reset(c: &mut Ctx, mut p_fade_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "FadeSlot_Reset", &PROBE_N, &[(p_fade_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = 0i32; let a_ = (p_fade_1).wrapping_add(88); c.m.w32(a_, (v_) as u32); };
    F::Ramp_Stop(c, (p_fade_1).wrapping_add(64));
       
    };
    if rt::probing() { crate::host::harness::probe(c, "FadeSlot_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn FadeSlot_SetPause(c: &mut Ctx, mut p_fade_1: u32, mut p_pause_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "FadeSlot_SetPause", &PROBE_N, &[(p_fade_1) as u32, (p_pause_2) as u32], false); }
    let r_ = 'f: {
    if ((F::FadeSlot_IsActive(c, p_fade_1)) != 0) {
        if ((p_pause_2) != 0) {
            { let r_ = 8i32; let a_ = (p_fade_1).wrapping_add(88); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        } else {
            { let r_ = (!(8i32)); let a_ = (p_fade_1).wrapping_add(88); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "FadeSlot_SetPause", &PROBE_N, &[], true); }
    r_
}

pub fn FadeSlot_CalcColor(c: &mut Ctx, mut p_fade_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "FadeSlot_CalcColor", &PROBE_N, &[(p_fade_1) as u32], false); }
    let r_ = 'f: {
    { let a2_ = F32(c.m.r32(((p_fade_1).wrapping_add(64)).wrapping_add(16))); F::Vec4_Scale(c, p_fade_1, (p_fade_1).wrapping_add(48), a2_) };
    F::Vec4_Add(c, p_fade_1, p_fade_1, (p_fade_1).wrapping_add(16));
       
    };
    if rt::probing() { crate::host::harness::probe(c, "FadeSlot_CalcColor", &PROBE_N, &[], true); }
    r_
}

pub fn FadeSlot_IsPaused(c: &mut Ctx, mut p_fade_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "FadeSlot_IsPaused", &PROBE_N, &[(p_fade_1) as u32], false); }
    let r_ = 'f: {
    break 'f ((((c.m.r32((p_fade_1).wrapping_add(88)) as i32)).wrapping_shr((3i32) as u32)) & (1i32));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "FadeSlot_IsPaused", &PROBE_N, &[], true); }
    r_
}

pub fn FadeSlot_IsActive(c: &mut Ctx, mut p_fade_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "FadeSlot_IsActive", &PROBE_N, &[(p_fade_1) as u32], false); }
    let r_ = 'f: {
    break 'f ((((c.m.r32((p_fade_1).wrapping_add(88)) as i32)) != (0i32)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "FadeSlot_IsActive", &PROBE_N, &[], true); }
    r_
}

pub fn FadeSlot_IsDone(c: &mut Ctx, mut p_fade_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "FadeSlot_IsDone", &PROBE_N, &[(p_fade_1) as u32], false); }
    let mut l_done_2: i32 = 0;
    let r_ = 'f: {
    l_done_2 = 0i32;
    if (((((((((c.m.r32((p_fade_1).wrapping_add(88)) as i32)) & (4i32))) != 0) || ((((((c.m.r32((p_fade_1).wrapping_add(88)) as i32)) == (0i32)) as i32)) != 0)) as i32)) != 0) {
        l_done_2 = 1i32;
    }
    break 'f l_done_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "FadeSlot_IsDone", &PROBE_N, &[], true); }
    r_
}

pub fn FadeSlot_Update(c: &mut Ctx, mut p_fade_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "FadeSlot_Update", &PROBE_N, &[(p_fade_1) as u32], false); }
    let r_ = 'f: {
    if ((((((F::FadeSlot_IsActive(c, p_fade_1)) != 0) && ((((!((F::FadeSlot_IsPaused(c, p_fade_1)) != 0)) as i32)) != 0)) as i32)) != 0) {
        F::FadeSlot_CalcColor(c, p_fade_1);
        if (((((c.m.r32((p_fade_1).wrapping_add(88)) as i32)) & (4i32))) != 0) {
            if (((((rt::f2i(F32(c.m.r32((p_fade_1).wrapping_add((((3i32) as u32)).wrapping_mul(4)))))) == (0i32)) as i32)) != 0) {
                F::FadeSlot_Reset(c, p_fade_1);
            }
        } else {
            if ((F::Ramp_Step(c, (p_fade_1).wrapping_add(64))) != 0) {
                { let r_ = 4i32; let a_ = (p_fade_1).wrapping_add(88); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            }
        }
        'l1: loop {
            'c2: {
            }
            if !((0i32) != 0) { break 'l1; }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "FadeSlot_Update", &PROBE_N, &[], true); }
    r_
}

pub fn FadeSlot_Draw(c: &mut Ctx, mut p_fade_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "FadeSlot_Draw", &PROBE_N, &[(p_fade_1) as u32], false); }
    let r_ = 'f: {
    if ((F::FadeSlot_IsActive(c, p_fade_1)) != 0) {
        F::Fade_DrawRect(c, p_fade_1);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "FadeSlot_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn Fade_Init(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Fade_Init", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let _ = F::memset(c, sym::gFade, 0i32, 96u32); };
    F::Fade_ResetAll(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Fade_Init", &PROBE_N, &[], true); }
    r_
}

pub fn Fade_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Fade_Term", &PROBE_N, &[], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Fade_Term", &PROBE_N, &[], true); }
    r_
}

pub fn Fade_ResetAll(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Fade_ResetAll", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let r_ = 'f: {
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (3i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                F::FadeSlot_Reset(c, (sym::gFade).wrapping_add((((l_i_1) as u32)).wrapping_mul(96)));
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Fade_ResetAll", &PROBE_N, &[], true); }
    r_
}

pub fn Fade_Reset(c: &mut Ctx, mut p_idx_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Fade_Reset", &PROBE_N, &[(p_idx_1) as u32], false); }
    let r_ = 'f: {
    F::FadeSlot_Reset(c, (sym::gFade).wrapping_add((((p_idx_1) as u32)).wrapping_mul(96)));
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Fade_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn Fade_PauseAll(c: &mut Ctx, mut p_pause_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Fade_PauseAll", &PROBE_N, &[(p_pause_1) as u32], false); }
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < (3i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                F::FadeSlot_SetPause(c, (sym::gFade).wrapping_add((((l_i_2) as u32)).wrapping_mul(96)), p_pause_1);
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Fade_PauseAll", &PROBE_N, &[], true); }
    r_
}

pub fn Fade_Pause(c: &mut Ctx, mut p_idx_1: i32, mut p_pause_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Fade_Pause", &PROBE_N, &[(p_idx_1) as u32, (p_pause_2) as u32], false); }
    let r_ = 'f: {
    F::FadeSlot_SetPause(c, (sym::gFade).wrapping_add((((p_idx_1) as u32)).wrapping_mul(96)), p_pause_2);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Fade_Pause", &PROBE_N, &[], true); }
    r_
}

pub fn Fade_IsPaused(c: &mut Ctx, mut p_idx_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Fade_IsPaused", &PROBE_N, &[(p_idx_1) as u32], false); }
    let r_ = 'f: {
    break 'f F::FadeSlot_IsPaused(c, (sym::gFade).wrapping_add((((p_idx_1) as u32)).wrapping_mul(96)));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Fade_IsPaused", &PROBE_N, &[], true); }
    r_
}

pub fn Fade_IsActive(c: &mut Ctx, mut p_idx_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Fade_IsActive", &PROBE_N, &[(p_idx_1) as u32], false); }
    let r_ = 'f: {
    break 'f F::FadeSlot_IsActive(c, (sym::gFade).wrapping_add((((p_idx_1) as u32)).wrapping_mul(96)));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Fade_IsActive", &PROBE_N, &[], true); }
    r_
}

pub fn Fade_IsDone(c: &mut Ctx, mut p_idx_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Fade_IsDone", &PROBE_N, &[(p_idx_1) as u32], false); }
    let r_ = 'f: {
    break 'f F::FadeSlot_IsDone(c, (sym::gFade).wrapping_add((((p_idx_1) as u32)).wrapping_mul(96)));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Fade_IsDone", &PROBE_N, &[], true); }
    r_
}

pub fn Fade_Start(c: &mut Ctx, mut p_idx_1: i32, mut p_dir_2: i32, mut p_seconds_3: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Fade_Start", &PROBE_N, &[(p_idx_1) as u32, (p_dir_2) as u32, (p_seconds_3).0], false); }
    let mut l_fade_4: u32 = 0;
    let r_ = 'f: {
    l_fade_4 = (sym::gFade).wrapping_add((((p_idx_1) as u32)).wrapping_mul(96));
    's1: {
        let sv5: i32 = p_dir_2;
        match sv5 {
            0i32 => {
                { let v_ = 1i32; let a_ = (l_fade_4).wrapping_add(88); c.m.w32(a_, (v_) as u32); };
                F::Vec4_Copy(c, (l_fade_4).wrapping_add(16), (sym::gFadeColors).wrapping_add((((p_idx_1) as u32)).wrapping_mul(32)));
                F::Vec4_Copy(c, (l_fade_4).wrapping_add(32), ((sym::gFadeColors).wrapping_add((((p_idx_1) as u32)).wrapping_mul(32))).wrapping_add(16));
                break 's1;
            }
            1i32 => {
                { let v_ = 2i32; let a_ = (l_fade_4).wrapping_add(88); c.m.w32(a_, (v_) as u32); };
                F::Vec4_Copy(c, (l_fade_4).wrapping_add(16), ((sym::gFadeColors).wrapping_add((((p_idx_1) as u32)).wrapping_mul(32))).wrapping_add(16));
                F::Vec4_Copy(c, (l_fade_4).wrapping_add(32), (sym::gFadeColors).wrapping_add((((p_idx_1) as u32)).wrapping_mul(32)));
                break 's1;
            }
            _ => {
            }
        }
    }
    F::Vec4_Sub(c, (l_fade_4).wrapping_add(48), (l_fade_4).wrapping_add(32), (l_fade_4).wrapping_add(16));
    F::Ramp_Start(c, (l_fade_4).wrapping_add(64), p_seconds_3, F32(0x00000000), F32(0x3f800000));
    F::FadeSlot_Update(c, l_fade_4);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Fade_Start", &PROBE_N, &[], true); }
    r_
}

pub fn Fade_UpdateAll(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Fade_UpdateAll", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let r_ = 'f: {
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (3i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                F::FadeSlot_Update(c, (sym::gFade).wrapping_add((((l_i_1) as u32)).wrapping_mul(96)));
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Fade_UpdateAll", &PROBE_N, &[], true); }
    r_
}

pub fn Fade_DrawScreen(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Fade_DrawScreen", &PROBE_N, &[], false); }
    let mut l_fade_1: u32 = 0;
    let r_ = 'f: {
    l_fade_1 = sym::gFade;
    F::FadeSlot_Draw(c, l_fade_1);
    { let o_ = l_fade_1; let n_ = (o_).wrapping_add(96); l_fade_1 = n_; };
    F::FadeSlot_Draw(c, l_fade_1);
    { let o_ = l_fade_1; let n_ = (o_).wrapping_add(96); l_fade_1 = n_; };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Fade_DrawScreen", &PROBE_N, &[], true); }
    r_
}

pub fn Fade_DrawSlot2(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Fade_DrawSlot2", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::FadeSlot_Draw(c, (sym::gFade).wrapping_add((((2i32) as u32)).wrapping_mul(96)));
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Fade_DrawSlot2", &PROBE_N, &[], true); }
    r_
}
