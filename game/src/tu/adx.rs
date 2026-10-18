#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn Adx_CalcOutVol(c: &mut Ctx, mut p_ch_1: i32, mut p_vol_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Adx_CalcOutVol", &PROBE_N, &[(p_ch_1) as u32, (p_vol_2) as u32], false); }
    let mut l_kind_3: i32 = 0;
    let mut l_v_4: i32 = 0;
    let mut l_s_5: F32 = F32(0);
    let r_ = 'f: {
    's1: {
        let sv6: i32 = p_ch_1;
        match sv6 {
            0i32 | 1i32 => {
                l_kind_3 = 1i32;
                break 's1;
            }
            _ => {
                l_kind_3 = 2i32;
                break 's1;
            }
        }
    }
    p_vol_2 = ((p_vol_2).wrapping_mul((c.m.r32((sym::gAdxChannelGainTbl).wrapping_add((((p_ch_1) as u32)).wrapping_mul(4))) as i32))).wrapping_mul({ let a_ = ((c.m.r32(sym::gSaveData)).wrapping_add(5796)).wrapping_add((((l_kind_3) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) });
    p_vol_2 = rt::div_i32(p_vol_2, 1152i32);
    l_v_4 = (if (((((p_vol_2) < (0i32)) as i32)) != 0) { 0i32 } else { (if (((((p_vol_2) > (128i32)) as i32)) != 0) { 128i32 } else { p_vol_2 }) });
    p_vol_2 = l_v_4;
    if (((((p_vol_2) == (0i32)) as i32)) != 0) {
        p_vol_2 = (960i32).wrapping_neg();
    } else {
        l_s_5 = F::Mathf_Sin(c, rt::fmul(rt::i2f((128i32).wrapping_sub(p_vol_2)), F32(0x3c000000)));
        p_vol_2 = rt::f2i(rt::fmul(rt::fmul(rt::fmul(l_s_5, l_s_5), F32(0x3e8f5c28)), F32(0xc4700000)));
    }
    break 'f p_vol_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Adx_CalcOutVol", &PROBE_N, &[], true); }
    r_
}

pub fn Adx_ConvPan(c: &mut Ctx, mut p_pan_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Adx_ConvPan", &PROBE_N, &[(p_pan_1) as u32], false); }
    let mut l_p_2: i32 = 0;
    let r_ = 'f: {
    l_p_2 = (if (((((p_pan_1) < ((64i32).wrapping_neg())) as i32)) != 0) { (64i32).wrapping_neg() } else { (if (((((p_pan_1) > (64i32)) as i32)) != 0) { 64i32 } else { p_pan_1 }) });
    p_pan_1 = l_p_2;
    break 'f rt::div_i32((p_pan_1).wrapping_sub(1i32), 4i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Adx_ConvPan", &PROBE_N, &[], true); }
    r_
}

pub fn Adx_CalcVolume(c: &mut Ctx, mut p_ch_1: i32, mut p_vol_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Adx_CalcVolume", &PROBE_N, &[(p_ch_1) as u32, (p_vol_2) as u32], false); }
    let mut l_kind_3: i32 = 0;
    let mut l_v_4: i32 = 0;
    let r_ = 'f: {
    's1: {
        let sv5: i32 = p_ch_1;
        match sv5 {
            0i32 | 1i32 => {
                l_kind_3 = 1i32;
                break 's1;
            }
            _ => {
                l_kind_3 = 2i32;
                break 's1;
            }
        }
    }
    p_vol_2 = ((p_vol_2).wrapping_mul((c.m.r32((sym::gAdxChannelGainTbl).wrapping_add((((p_ch_1) as u32)).wrapping_mul(4))) as i32))).wrapping_mul({ let a_ = ((c.m.r32(sym::gSaveData)).wrapping_add(5796)).wrapping_add((((l_kind_3) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) });
    p_vol_2 = rt::div_i32(p_vol_2, 1152i32);
    l_v_4 = (if (((((p_vol_2) < (0i32)) as i32)) != 0) { 0i32 } else { (if (((((p_vol_2) > (128i32)) as i32)) != 0) { 128i32 } else { p_vol_2 }) });
    p_vol_2 = l_v_4;
    break 'f p_vol_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Adx_CalcVolume", &PROBE_N, &[], true); }
    r_
}

pub fn Adx_Stub265720(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Adx_Stub265720", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Adx_Stub265720", &PROBE_N, &[], true); }
    r_
}

pub fn Adx_StopAll(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Adx_StopAll", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let mut l_done_2: i32 = 0;
    let r_ = 'f: {
    F::Adx_ResumeSeVoice(c);
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (6i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                F::Adx_Stop(c, l_i_1);
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
    'l3: while ((1i32) != 0) {
        l_done_2 = 1i32;
        {
            l_i_1 = 0i32;
            'l4: loop {
                if !(((((l_i_1) < (6i32)) as i32)) != 0) { break 'l4; }
                'c5: {
                    if (((((F::Adx_GetStat(c, l_i_1)) != (0i32)) as i32)) != 0) {
                        l_done_2 = 0i32;
                        break 'l4;
                    }
                }
                { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
            }
        }
        if ((l_done_2) != 0) {
            break 'l3;
        }
        F::ADXM_ExecMain(c);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Adx_StopAll", &PROBE_N, &[], true); }
    r_
}

pub fn Adx_PauseSeVoice(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Adx_PauseSeVoice", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let a0_ = c.m.r32(((sym::gAdxPlayerTbl).wrapping_add((((2i32) as u32)).wrapping_mul(20))).wrapping_add(16)); F::ADXT_Pause(c, a0_, 1i32) };
    { let a0_ = c.m.r32(((sym::gAdxPlayerTbl).wrapping_add((((3i32) as u32)).wrapping_mul(20))).wrapping_add(16)); F::ADXT_Pause(c, a0_, 1i32) };
    { let a0_ = c.m.r32(((sym::gAdxPlayerTbl).wrapping_add((((4i32) as u32)).wrapping_mul(20))).wrapping_add(16)); F::ADXT_Pause(c, a0_, 1i32) };
    { let a0_ = c.m.r32(((sym::gAdxPlayerTbl).wrapping_add((((5i32) as u32)).wrapping_mul(20))).wrapping_add(16)); F::ADXT_Pause(c, a0_, 1i32) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Adx_PauseSeVoice", &PROBE_N, &[], true); }
    r_
}

pub fn Adx_ResumeSeVoice(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Adx_ResumeSeVoice", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let a0_ = c.m.r32(((sym::gAdxPlayerTbl).wrapping_add((((2i32) as u32)).wrapping_mul(20))).wrapping_add(16)); F::ADXT_Pause(c, a0_, 0i32) };
    { let a0_ = c.m.r32(((sym::gAdxPlayerTbl).wrapping_add((((3i32) as u32)).wrapping_mul(20))).wrapping_add(16)); F::ADXT_Pause(c, a0_, 0i32) };
    { let a0_ = c.m.r32(((sym::gAdxPlayerTbl).wrapping_add((((4i32) as u32)).wrapping_mul(20))).wrapping_add(16)); F::ADXT_Pause(c, a0_, 0i32) };
    { let a0_ = c.m.r32(((sym::gAdxPlayerTbl).wrapping_add((((5i32) as u32)).wrapping_mul(20))).wrapping_add(16)); F::ADXT_Pause(c, a0_, 0i32) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Adx_ResumeSeVoice", &PROBE_N, &[], true); }
    r_
}

pub fn Adx_Play(c: &mut Ctx, mut p_ch_1: i32, mut p_id_2: i32, mut p_vol_3: i32, mut p_pan_4: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Adx_Play", &PROBE_N, &[(p_ch_1) as u32, (p_id_2) as u32, (p_vol_3) as u32, (p_pan_4) as u32], false); }
    let r_ = 'f: {
    's1: {
        let sv5: i32 = p_ch_1;
        match sv5 {
            4i32 => {
                if ((F::BtlScript_IsSlotBusy(c, 0i32, 0i32)) != 0) {
                    break 'f;
                }
                if ((F::BtlScript_IsSlotBusy(c, 0i32, 1i32)) != 0) {
                    break 'f;
                }
                break 's1;
            }
            5i32 => {
                if ((F::BtlScript_IsSlotBusy(c, 1i32, 0i32)) != 0) {
                    break 'f;
                }
                if ((F::BtlScript_IsSlotBusy(c, 1i32, 1i32)) != 0) {
                    break 'f;
                }
                break 's1;
            }
            _ => {
            }
        }
    }
    F::Adx_SetPan(c, p_ch_1, p_pan_4, p_pan_4);
    F::Adx_SetVolume(c, p_ch_1, p_vol_3);
    { let a0_ = c.m.r32(((sym::gAdxPlayerTbl).wrapping_add((((p_ch_1) as u32)).wrapping_mul(20))).wrapping_add(16)); F::ADXT_Pause(c, a0_, 0i32) };
    { let a0_ = c.m.r32(((sym::gAdxPlayerTbl).wrapping_add((((p_ch_1) as u32)).wrapping_mul(20))).wrapping_add(16)); F::File_StartAdxById(c, a0_, p_id_2) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Adx_Play", &PROBE_N, &[], true); }
    r_
}

pub fn Adx_Stop(c: &mut Ctx, mut p_ch_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Adx_Stop", &PROBE_N, &[(p_ch_1) as u32], false); }
    let r_ = 'f: {
    { let a0_ = c.m.r32(((sym::gAdxPlayerTbl).wrapping_add((((p_ch_1) as u32)).wrapping_mul(20))).wrapping_add(16)); F::ADXT_Stop(c, a0_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Adx_Stop", &PROBE_N, &[], true); }
    r_
}

pub fn Adx_PlayPaused(c: &mut Ctx, mut p_ch_1: i32, mut p_id_2: i32, mut p_vol_3: i32, mut p_pan_4: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Adx_PlayPaused", &PROBE_N, &[(p_ch_1) as u32, (p_id_2) as u32, (p_vol_3) as u32, (p_pan_4) as u32], false); }
    let r_ = 'f: {
    F::Adx_SetPan(c, p_ch_1, p_pan_4, p_pan_4);
    F::Adx_SetVolume(c, p_ch_1, p_vol_3);
    { let a0_ = c.m.r32(((sym::gAdxPlayerTbl).wrapping_add((((p_ch_1) as u32)).wrapping_mul(20))).wrapping_add(16)); F::ADXT_Pause(c, a0_, 1i32) };
    { let a0_ = c.m.r32(((sym::gAdxPlayerTbl).wrapping_add((((p_ch_1) as u32)).wrapping_mul(20))).wrapping_add(16)); F::File_StartAdxById(c, a0_, p_id_2) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Adx_PlayPaused", &PROBE_N, &[], true); }
    r_
}

pub fn Adx_PlayFilePaused(c: &mut Ctx, mut p_ch_1: i32, mut p_fname_2: u32, mut p_vol_3: i32, mut p_pan_4: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Adx_PlayFilePaused", &PROBE_N, &[(p_ch_1) as u32, (p_fname_2) as u32, (p_vol_3) as u32, (p_pan_4) as u32], false); }
    let r_ = 'f: {
    F::Adx_SetPan(c, p_ch_1, p_pan_4, p_pan_4);
    F::Adx_SetVolume(c, p_ch_1, p_vol_3);
    { let a0_ = c.m.r32(((sym::gAdxPlayerTbl).wrapping_add((((p_ch_1) as u32)).wrapping_mul(20))).wrapping_add(16)); F::ADXT_Pause(c, a0_, 1i32) };
    { let a0_ = c.m.r32(((sym::gAdxPlayerTbl).wrapping_add((((p_ch_1) as u32)).wrapping_mul(20))).wrapping_add(16)); F::ADXT_StartFname(c, a0_, p_fname_2) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Adx_PlayFilePaused", &PROBE_N, &[], true); }
    r_
}

pub fn Adx_Resume(c: &mut Ctx, mut p_ch_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Adx_Resume", &PROBE_N, &[(p_ch_1) as u32], false); }
    let r_ = 'f: {
    { let a0_ = c.m.r32(((sym::gAdxPlayerTbl).wrapping_add((((p_ch_1) as u32)).wrapping_mul(20))).wrapping_add(16)); F::ADXT_Pause(c, a0_, 0i32) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Adx_Resume", &PROBE_N, &[], true); }
    r_
}

pub fn Adx_SetVolume(c: &mut Ctx, mut p_ch_1: i32, mut p_vol_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Adx_SetVolume", &PROBE_N, &[(p_ch_1) as u32, (p_vol_2) as u32], false); }
    let mut l_adxt_3: u32 = 0;
    let r_ = 'f: {
    l_adxt_3 = c.m.r32(((sym::gAdxPlayerTbl).wrapping_add((((p_ch_1) as u32)).wrapping_mul(20))).wrapping_add(16));
    { let a1_ = F::Adx_CalcOutVol(c, p_ch_1, p_vol_2); F::ADXT_SetOutVol(c, l_adxt_3, a1_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Adx_SetVolume", &PROBE_N, &[], true); }
    r_
}

pub fn Adx_GetStat(c: &mut Ctx, mut p_ch_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Adx_GetStat", &PROBE_N, &[(p_ch_1) as u32], false); }
    let r_ = 'f: {
    break 'f { let a0_ = c.m.r32(((sym::gAdxPlayerTbl).wrapping_add((((p_ch_1) as u32)).wrapping_mul(20))).wrapping_add(16)); F::ADXT_GetStat(c, a0_) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Adx_GetStat", &PROBE_N, &[], true); }
    r_
}

pub fn Adx_IsStopped(c: &mut Ctx, mut p_ch_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Adx_IsStopped", &PROBE_N, &[(p_ch_1) as u32], false); }
    let mut l_stopped_2: i32 = 0;
    let r_ = 'f: {
    's1: {
        let sv3: i32 = F::Adx_GetStat(c, p_ch_1);
        match sv3 {
            0i32 | 5i32 => {
                l_stopped_2 = 1i32;
                break 's1;
            }
            _ => {
                l_stopped_2 = 0i32;
                break 's1;
            }
        }
    }
    break 'f l_stopped_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Adx_IsStopped", &PROBE_N, &[], true); }
    r_
}

pub fn Adx_SetPan(c: &mut Ctx, mut p_ch_1: i32, mut p_pan0_2: i32, mut p_pan1_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Adx_SetPan", &PROBE_N, &[(p_ch_1) as u32, (p_pan0_2) as u32, (p_pan1_3) as u32], false); }
    let r_ = 'f: {
    { let a0_ = c.m.r32(((sym::gAdxPlayerTbl).wrapping_add((((p_ch_1) as u32)).wrapping_mul(20))).wrapping_add(16)); let a2_ = F::Adx_ConvPan(c, p_pan0_2); F::ADXT_SetOutPan(c, a0_, 0i32, a2_) };
    { let a0_ = c.m.r32(((sym::gAdxPlayerTbl).wrapping_add((((p_ch_1) as u32)).wrapping_mul(20))).wrapping_add(16)); let a2_ = F::Adx_ConvPan(c, p_pan1_3); F::ADXT_SetOutPan(c, a0_, 1i32, a2_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Adx_SetPan", &PROBE_N, &[], true); }
    r_
}

pub fn Adx_FadeOutStep(c: &mut Ctx, mut p_ch_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Adx_FadeOutStep", &PROBE_N, &[(p_ch_1) as u32], false); }
    let mut l_min_2: i32 = 0;
    let mut l_vol_3: i32 = 0;
    let r_ = 'f: {
    l_min_2 = (960i32).wrapping_neg();
    l_vol_3 = { let a0_ = c.m.r32(((sym::gAdxPlayerTbl).wrapping_add((((p_ch_1) as u32)).wrapping_mul(20))).wrapping_add(16)); F::ADXT_GetOutVol(c, a0_) };
    if (((((l_vol_3) > (l_min_2)) as i32)) != 0) {
        { let r_ = 120i32; let n_ = (l_vol_3).wrapping_sub(r_); l_vol_3 = n_; };
        if (((((l_vol_3) < ((960i32).wrapping_neg())) as i32)) != 0) {
            l_vol_3 = (960i32).wrapping_neg();
        }
        { let a0_ = c.m.r32(((sym::gAdxPlayerTbl).wrapping_add((((p_ch_1) as u32)).wrapping_mul(20))).wrapping_add(16)); F::ADXT_SetOutVol(c, a0_, l_vol_3) };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Adx_FadeOutStep", &PROBE_N, &[], true); }
    r_
}

pub fn Adx_SetMono(c: &mut Ctx, mut p_mono_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Adx_SetMono", &PROBE_N, &[(p_mono_1) as u32], false); }
    let r_ = 'f: {
    F::ADXT_SetOutputMono(c, (((p_mono_1) != (0i32)) as i32));
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Adx_SetMono", &PROBE_N, &[], true); }
    r_
}

pub fn Bgm_PlayEx(c: &mut Ctx, mut p_id_1: i32, mut p_vol_2: i32, mut p_pan_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Bgm_PlayEx", &PROBE_N, &[(p_id_1) as u32, (p_vol_2) as u32, (p_pan_3) as u32], false); }
    let r_ = 'f: {
    F::Adx_Play(c, 0i32, p_id_1, p_vol_2, p_pan_3);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Bgm_PlayEx", &PROBE_N, &[], true); }
    r_
}

pub fn Bgm_Play(c: &mut Ctx, mut p_id_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Bgm_Play", &PROBE_N, &[(p_id_1) as u32], false); }
    let r_ = 'f: {
    F::Bgm_PlayEx(c, p_id_1, 64i32, 0i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Bgm_Play", &PROBE_N, &[], true); }
    r_
}

pub fn Bgm_Stop(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Bgm_Stop", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::Adx_Stop(c, 0i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Bgm_Stop", &PROBE_N, &[], true); }
    r_
}

pub fn Bgm_SetVolume(c: &mut Ctx, mut p_vol_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Bgm_SetVolume", &PROBE_N, &[(p_vol_1) as u32], false); }
    let r_ = 'f: {
    F::Adx_SetVolume(c, 0i32, p_vol_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Bgm_SetVolume", &PROBE_N, &[], true); }
    r_
}

pub fn Bgm_GetStat(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Bgm_GetStat", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f F::Adx_GetStat(c, 0i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Bgm_GetStat", &PROBE_N, &[], true); }
    r_
}

pub fn Bgm_IsStopped(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Bgm_IsStopped", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f F::Adx_IsStopped(c, 0i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Bgm_IsStopped", &PROBE_N, &[], true); }
    r_
}

pub fn Bgm_FadeOutStep(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Bgm_FadeOutStep", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::Adx_FadeOutStep(c, 0i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Bgm_FadeOutStep", &PROBE_N, &[], true); }
    r_
}

pub fn MapBgm_PlayEx(c: &mut Ctx, mut p_id_1: i32, mut p_vol_2: i32, mut p_pan_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "MapBgm_PlayEx", &PROBE_N, &[(p_id_1) as u32, (p_vol_2) as u32, (p_pan_3) as u32], false); }
    let r_ = 'f: {
    F::Adx_Play(c, 1i32, p_id_1, p_vol_2, p_pan_3);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "MapBgm_PlayEx", &PROBE_N, &[], true); }
    r_
}

pub fn MapBgm_Play(c: &mut Ctx, mut p_id_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "MapBgm_Play", &PROBE_N, &[(p_id_1) as u32], false); }
    let r_ = 'f: {
    F::MapBgm_PlayEx(c, p_id_1, 64i32, 0i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "MapBgm_Play", &PROBE_N, &[], true); }
    r_
}

pub fn MapBgm_Stop(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "MapBgm_Stop", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::Adx_Stop(c, 1i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "MapBgm_Stop", &PROBE_N, &[], true); }
    r_
}

pub fn MapBgm_SetVolume(c: &mut Ctx, mut p_vol_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "MapBgm_SetVolume", &PROBE_N, &[(p_vol_1) as u32], false); }
    let r_ = 'f: {
    F::Adx_SetVolume(c, 1i32, p_vol_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "MapBgm_SetVolume", &PROBE_N, &[], true); }
    r_
}

pub fn MapBgm_GetStat(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "MapBgm_GetStat", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f F::Adx_GetStat(c, 1i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "MapBgm_GetStat", &PROBE_N, &[], true); }
    r_
}

pub fn MapBgm_IsStopped(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "MapBgm_IsStopped", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f F::Adx_IsStopped(c, 1i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "MapBgm_IsStopped", &PROBE_N, &[], true); }
    r_
}

pub fn MapBgm_FadeOutStep(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "MapBgm_FadeOutStep", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::Adx_FadeOutStep(c, 1i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "MapBgm_FadeOutStep", &PROBE_N, &[], true); }
    r_
}

pub fn Voice_PlayCharaEx(c: &mut Ctx, mut p_voice_1: i32, mut p_chara_2: i32, mut p_line_3: i32, mut p_vol_4: i32, mut p_pan_5: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Voice_PlayCharaEx", &PROBE_N, &[(p_voice_1) as u32, (p_chara_2) as u32, (p_line_3) as u32, (p_vol_4) as u32, (p_pan_5) as u32], false); }
    let mut l_id_6: i32 = 0;
    let r_ = 'f: {
    if (((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5640); (c.m.r32(a_) as i32) }) & (1i32))) != 0) {
        l_id_6 = (52274i32).wrapping_add(((p_chara_2).wrapping_mul(100i32)).wrapping_add(p_line_3));
    } else {
        l_id_6 = (36174i32).wrapping_add(((p_chara_2).wrapping_mul(100i32)).wrapping_add(p_line_3));
    }
    F::Adx_Play(c, (p_voice_1).wrapping_add(4i32), l_id_6, p_vol_4, p_pan_5);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Voice_PlayCharaEx", &PROBE_N, &[], true); }
    r_
}

pub fn Voice_PlayChara(c: &mut Ctx, mut p_voice_1: i32, mut p_chara_2: i32, mut p_line_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Voice_PlayChara", &PROBE_N, &[(p_voice_1) as u32, (p_chara_2) as u32, (p_line_3) as u32], false); }
    let r_ = 'f: {
    F::Voice_PlayCharaEx(c, p_voice_1, p_chara_2, p_line_3, 128i32, 0i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Voice_PlayChara", &PROBE_N, &[], true); }
    r_
}

pub fn Voice_Play(c: &mut Ctx, mut p_voice_1: i32, mut p_id_2: i32, mut p_vol_3: i32, mut p_pan_4: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Voice_Play", &PROBE_N, &[(p_voice_1) as u32, (p_id_2) as u32, (p_vol_3) as u32, (p_pan_4) as u32], false); }
    let r_ = 'f: {
    F::Adx_Play(c, (p_voice_1).wrapping_add(4i32), p_id_2, p_vol_3, p_pan_4);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Voice_Play", &PROBE_N, &[], true); }
    r_
}

pub fn Voice_PlayDefault(c: &mut Ctx, mut p_voice_1: i32, mut p_id_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Voice_PlayDefault", &PROBE_N, &[(p_voice_1) as u32, (p_id_2) as u32], false); }
    let r_ = 'f: {
    F::Voice_Play(c, p_voice_1, p_id_2, 128i32, 0i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Voice_PlayDefault", &PROBE_N, &[], true); }
    r_
}

pub fn Voice_PlayPaused(c: &mut Ctx, mut p_voice_1: i32, mut p_id_2: i32, mut p_vol_3: i32, mut p_pan_4: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Voice_PlayPaused", &PROBE_N, &[(p_voice_1) as u32, (p_id_2) as u32, (p_vol_3) as u32, (p_pan_4) as u32], false); }
    let r_ = 'f: {
    F::Adx_PlayPaused(c, (p_voice_1).wrapping_add(4i32), p_id_2, p_vol_3, p_pan_4);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Voice_PlayPaused", &PROBE_N, &[], true); }
    r_
}

pub fn Voice_PlayPausedDefault(c: &mut Ctx, mut p_voice_1: i32, mut p_id_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Voice_PlayPausedDefault", &PROBE_N, &[(p_voice_1) as u32, (p_id_2) as u32], false); }
    let r_ = 'f: {
    F::Voice_PlayPaused(c, p_voice_1, p_id_2, 128i32, 0i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Voice_PlayPausedDefault", &PROBE_N, &[], true); }
    r_
}

pub fn Voice_Resume(c: &mut Ctx, mut p_voice_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Voice_Resume", &PROBE_N, &[(p_voice_1) as u32], false); }
    let r_ = 'f: {
    F::Adx_Resume(c, (p_voice_1).wrapping_add(4i32));
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Voice_Resume", &PROBE_N, &[], true); }
    r_
}

pub fn Voice_Stop(c: &mut Ctx, mut p_voice_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Voice_Stop", &PROBE_N, &[(p_voice_1) as u32], false); }
    let r_ = 'f: {
    F::Adx_Stop(c, (p_voice_1).wrapping_add(4i32));
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Voice_Stop", &PROBE_N, &[], true); }
    r_
}

pub fn Voice_GetStat(c: &mut Ctx, mut p_voice_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Voice_GetStat", &PROBE_N, &[(p_voice_1) as u32], false); }
    let r_ = 'f: {
    break 'f F::Adx_GetStat(c, (p_voice_1).wrapping_add(4i32));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Voice_GetStat", &PROBE_N, &[], true); }
    r_
}

pub fn Voice_IsStopped(c: &mut Ctx, mut p_voice_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Voice_IsStopped", &PROBE_N, &[(p_voice_1) as u32], false); }
    let r_ = 'f: {
    break 'f F::Adx_IsStopped(c, (p_voice_1).wrapping_add(4i32));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Voice_IsStopped", &PROBE_N, &[], true); }
    r_
}

pub fn Voice_FadeOutStep(c: &mut Ctx, mut p_voice_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Voice_FadeOutStep", &PROBE_N, &[(p_voice_1) as u32], false); }
    let r_ = 'f: {
    F::Adx_FadeOutStep(c, (p_voice_1).wrapping_add(4i32));
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Voice_FadeOutStep", &PROBE_N, &[], true); }
    r_
}

pub fn StreamSe_Play(c: &mut Ctx, mut p_se_1: i32, mut p_id_2: i32, mut p_vol_3: i32, mut p_pan_4: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StreamSe_Play", &PROBE_N, &[(p_se_1) as u32, (p_id_2) as u32, (p_vol_3) as u32, (p_pan_4) as u32], false); }
    let r_ = 'f: {
    F::Adx_Play(c, (p_se_1).wrapping_add(2i32), p_id_2, p_vol_3, p_pan_4);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StreamSe_Play", &PROBE_N, &[], true); }
    r_
}

pub fn StreamSe_PlayDefault(c: &mut Ctx, mut p_se_1: i32, mut p_id_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StreamSe_PlayDefault", &PROBE_N, &[(p_se_1) as u32, (p_id_2) as u32], false); }
    let r_ = 'f: {
    F::StreamSe_Play(c, p_se_1, p_id_2, 64i32, 0i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StreamSe_PlayDefault", &PROBE_N, &[], true); }
    r_
}

pub fn StreamSe_PlayPaused(c: &mut Ctx, mut p_se_1: i32, mut p_id_2: i32, mut p_vol_3: i32, mut p_pan_4: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StreamSe_PlayPaused", &PROBE_N, &[(p_se_1) as u32, (p_id_2) as u32, (p_vol_3) as u32, (p_pan_4) as u32], false); }
    let r_ = 'f: {
    F::Adx_PlayPaused(c, (p_se_1).wrapping_add(2i32), p_id_2, p_vol_3, p_pan_4);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StreamSe_PlayPaused", &PROBE_N, &[], true); }
    r_
}

pub fn StreamSe_PlayPausedDefault(c: &mut Ctx, mut p_se_1: i32, mut p_id_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StreamSe_PlayPausedDefault", &PROBE_N, &[(p_se_1) as u32, (p_id_2) as u32], false); }
    let r_ = 'f: {
    F::StreamSe_PlayPaused(c, p_se_1, p_id_2, 64i32, 0i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StreamSe_PlayPausedDefault", &PROBE_N, &[], true); }
    r_
}

pub fn StreamSe_Resume(c: &mut Ctx, mut p_se_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StreamSe_Resume", &PROBE_N, &[(p_se_1) as u32], false); }
    let r_ = 'f: {
    F::Adx_Resume(c, (p_se_1).wrapping_add(2i32));
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StreamSe_Resume", &PROBE_N, &[], true); }
    r_
}

pub fn StreamSe_Stop(c: &mut Ctx, mut p_se_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StreamSe_Stop", &PROBE_N, &[(p_se_1) as u32], false); }
    let r_ = 'f: {
    F::Adx_Stop(c, (p_se_1).wrapping_add(2i32));
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StreamSe_Stop", &PROBE_N, &[], true); }
    r_
}

pub fn StreamSe_GetStat(c: &mut Ctx, mut p_se_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StreamSe_GetStat", &PROBE_N, &[(p_se_1) as u32], false); }
    let r_ = 'f: {
    break 'f F::Adx_GetStat(c, (p_se_1).wrapping_add(2i32));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "StreamSe_GetStat", &PROBE_N, &[], true); }
    r_
}

pub fn StreamSe_IsStopped(c: &mut Ctx, mut p_se_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StreamSe_IsStopped", &PROBE_N, &[(p_se_1) as u32], false); }
    let r_ = 'f: {
    break 'f F::Adx_IsStopped(c, (p_se_1).wrapping_add(2i32));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "StreamSe_IsStopped", &PROBE_N, &[], true); }
    r_
}

pub fn StreamSe_FadeOutStep(c: &mut Ctx, mut p_se_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StreamSe_FadeOutStep", &PROBE_N, &[(p_se_1) as u32], false); }
    let r_ = 'f: {
    F::Adx_FadeOutStep(c, (p_se_1).wrapping_add(2i32));
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StreamSe_FadeOutStep", &PROBE_N, &[], true); }
    r_
}
