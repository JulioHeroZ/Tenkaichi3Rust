#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn BtlPause_GetWork(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlPause_GetWork", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f sym::gBtlPause;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlPause_GetWork", &PROBE_N, &[], true); }
    r_
}

pub fn BtlPause_Reset(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlPause_Reset", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let v_ = 0i32; let a_ = (F::BtlPause_GetWork(c)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlPause_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn BtlPause_Init(c: &mut Ctx, mut p_padCount_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlPause_Init", &PROBE_N, &[(p_padCount_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_padCount_1; let a_ = F::BtlPause_GetWork(c); c.m.w32(a_, (v_) as u32); };
    F::BtlPause_Reset(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlPause_Init", &PROBE_N, &[], true); }
    r_
}

pub fn BtlPause_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlPause_Term", &PROBE_N, &[], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlPause_Term", &PROBE_N, &[], true); }
    r_
}

pub fn BtlPause_CheckOpen(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlPause_CheckOpen", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_work_1: u32 = 0;
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
        l_work_1 = F::BtlPause_GetWork(c);
        {
            l_i_2 = 0i32;
            'l1: loop {
                if !(((((l_i_2) < ((c.m.r32(l_work_1) as i32))) as i32)) != 0) { break 'l1; }
                'c2: {
                    if ((((c.m.r32(((sym::gPad).wrapping_add((((l_i_2) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((4096i32) as u32)))) != 0) {
                        { let v_ = l_i_2; let a_ = (l_work_1).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                        break 'f 1i32;
                    }
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        if (((((F::Battle_GetMode(c)) != (7i32)) as i32)) != 0) {
            if ((F::PadWatch_GetMissing(c, fp)) != 0) {
                { let v_ = (c.m.r32(fp) as i32); let a_ = (l_work_1).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                { let r_ = ((((256i32) | (16384i32))) as u64); let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
                F::Adx_PauseSeVoice(c);
                F::Snd_SetPause(c, 4i32, 1i32);
                break 'f 0i32;
            }
        } else {
            if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((4096i32) as u32)))) != 0) {
                break 'f 1i32;
            }
        }
        break 'f 0i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlPause_CheckOpen", &PROBE_N, &[], true); }
    r_
}

pub fn BtlPause_CheckStart(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlPause_CheckStart", &PROBE_N, &[], false); }
    let mut l_work_1: u32 = 0;
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
    l_work_1 = F::BtlPause_GetWork(c);
    if ((((((F::BattleReplay_IsActive(c)) != 0) && ((F::BattleReplay_IsLoaded(c)) != 0)) as i32)) != 0) {
        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((4096i32) as u32)))) != 0) {
            break 'f 1i32;
        }
        break 'f 0i32;
    }
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < ((c.m.r32(l_work_1) as i32))) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((((c.m.r32(((sym::gPad).wrapping_add((((l_i_2) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((4096i32) as u32)))) != 0) {
                    break 'f 1i32;
                }
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlPause_CheckStart", &PROBE_N, &[], true); }
    r_
}

pub fn BtlPause_GetPadCount(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlPause_GetPadCount", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f { let a_ = F::BtlPause_GetWork(c); (c.m.r32(a_) as i32) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlPause_GetPadCount", &PROBE_N, &[], true); }
    r_
}

pub fn BtlPause_SetPad(c: &mut Ctx, mut p_pad_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlPause_SetPad", &PROBE_N, &[(p_pad_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_pad_1; let a_ = (F::BtlPause_GetWork(c)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlPause_SetPad", &PROBE_N, &[], true); }
    r_
}

pub fn BtlPause_GetMenuPad(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlPause_GetMenuPad", &PROBE_N, &[], false); }
    let mut l_work_1: u32 = 0;
    let r_ = 'f: {
    l_work_1 = F::BtlPause_GetWork(c);
    if ((((((F::BattleReplay_IsActive(c)) != 0) && ((F::BattleReplay_IsLoaded(c)) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    break 'f (c.m.r32((l_work_1).wrapping_add(4)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlPause_GetMenuPad", &PROBE_N, &[], true); }
    r_
}

pub fn BtlPause_GetPad(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlPause_GetPad", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f { let a_ = (F::BtlPause_GetWork(c)).wrapping_add(4); (c.m.r32(a_) as i32) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlPause_GetPad", &PROBE_N, &[], true); }
    r_
}
