#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn SimEv00_Roll(c: &mut Ctx, mut p_day_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SimEv00_Roll", &PROBE_N, &[(p_day_1) as u32], false); }
    let mut l_r_2: i32 = 0;
    let mut l_sum_3: i32 = 0;
    let mut l_i_4: i32 = 0;
    let r_ = 'f: {
    l_r_2 = ((F::Rand_Range(c, ((101i32) as u32))) as i32);
    if (((((c.m.r32((p_day_1).wrapping_add(1360)) as i32)) & (4096i32))) != 0) {
        break 'f 5i32;
    }
    if (((((c.m.r32((p_day_1).wrapping_add(1360)) as i32)) & (2048i32))) != 0) {
        break 'f 0i32;
    }
    l_sum_3 = 0i32;
    {
        l_i_4 = 0i32;
        'l1: loop {
            if !(((((l_i_4) < (6i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let r_ = { let a_ = (((c.m.r32((p_day_1).wrapping_add(2908))).wrapping_add((((0i32) as u32)).wrapping_mul(192))).wrapping_add(((((c.m.r32((p_day_1).wrapping_add(2932)) as i32)) as u32)).wrapping_mul(24))).wrapping_add((((l_i_4) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let n_ = (l_sum_3).wrapping_add(r_); l_sum_3 = n_; };
                if (((((l_r_2) < (l_sum_3)) as i32)) != 0) {
                    break 'f l_i_4;
                }
            }
            { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
        }
    }
    break 'f 5i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "SimEv00_Roll", &PROBE_N, &[], true); }
    r_
}

pub fn SimEv00(c: &mut Ctx, mut p_day_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SimEv00", &PROBE_N, &[(p_day_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_hp_2: i32 = 0;
    let r_ = 'f: {
        's1: {
            let sv3: i32 = (c.m.r32((p_day_1).wrapping_add(2920)) as i32);
            match sv3 {
                0i32 => {
                    { let v_ = F::SimEv00_Roll(c, p_day_1); let a_ = sym::gSimTrainOutcome0; c.m.w32(a_, (v_) as u32); };
                    { let v_ = 1i32; let a_ = (p_day_1).wrapping_add(2936); c.m.w32(a_, (v_) as u32); };
                    F::SimDay_Cmd(c, 5i32);
                    { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                    break 's1;
                }
                1i32 => {
                    { let v_ = 89i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                    if ((F::Rand_Range(c, ((2i32) as u32))) != 0) {
                        { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 21i32); };
                    } else {
                        { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 22i32); };
                    }
                    { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                    break 's1;
                }
                2i32 => {
                    { let v_ = ((c.m.r32(sym::gSimTrainOutcome0) as i32)).wrapping_add(90i32); let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                    { let v_ = { let a_ = (((c.m.r32((p_day_1).wrapping_add(2908))).wrapping_add((((0i32) as u32)).wrapping_mul(192))).wrapping_add(96)).wrapping_add(((((c.m.r32(sym::gSimTrainOutcome0) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    { let v_ = { let a_ = (((c.m.r32((p_day_1).wrapping_add(2908))).wrapping_add((((0i32) as u32)).wrapping_mul(192))).wrapping_add(120)).wrapping_add(((((c.m.r32(sym::gSimTrainOutcome0) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    { let v_ = { let a_ = (((c.m.r32((p_day_1).wrapping_add(2908))).wrapping_add((((0i32) as u32)).wrapping_mul(192))).wrapping_add(144)).wrapping_add(((((c.m.r32(sym::gSimTrainOutcome0) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (fp.wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    { let v_ = { let a_ = (((c.m.r32((p_day_1).wrapping_add(2908))).wrapping_add((((0i32) as u32)).wrapping_mul(192))).wrapping_add(168)).wrapping_add(((((c.m.r32(sym::gSimTrainOutcome0) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (fp.wrapping_add(8)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    l_hp_2 = { let a_ = (((c.m.r32((p_day_1).wrapping_add(2908))).wrapping_add((((0i32) as u32)).wrapping_mul(192))).wrapping_add(72)).wrapping_add(((((c.m.r32(sym::gSimTrainOutcome0) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
                    if ((((((c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) == ((c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32))) as i32)) != 0) {
                        { let a1_ = (c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32); F::SimDay_AddChange(c, 0i32, a1_) };
                    } else {
                        { let a1_ = ((({ let a0_ = (((((c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)).wrapping_sub((c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32))).wrapping_add(1i32)) as u32); F::Rand_Range(c, a0_) }).wrapping_add((((c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) as u32))) as i32); F::SimDay_AddChange(c, 0i32, a1_) };
                    }
                    if ((((((c.m.r32((fp.wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) == ((c.m.r32((fp.wrapping_add(8)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32))) as i32)) != 0) {
                        { let a1_ = (c.m.r32((fp.wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32); F::SimDay_AddChange(c, 1i32, a1_) };
                    } else {
                        { let a1_ = ((({ let a0_ = (((((c.m.r32((fp.wrapping_add(8)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)).wrapping_sub((c.m.r32((fp.wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32))).wrapping_add(1i32)) as u32); F::Rand_Range(c, a0_) }).wrapping_add((((c.m.r32((fp.wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) as u32))) as i32); F::SimDay_AddChange(c, 1i32, a1_) };
                    }
                    F::SimDay_AddChange(c, 2i32, l_hp_2);
                    F::SimDay_Cmd(c, 26i32);
                    { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                    break 's1;
                }
                3i32 => {
                    { let v_ = (1i32).wrapping_neg(); let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                    break 'f 1i32;
                }
                _ => {
                }
            }
        }
        break 'f 0i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "SimEv00", &PROBE_N, &[], true); }
    r_
}

pub fn SimEv01_Roll(c: &mut Ctx, mut p_day_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SimEv01_Roll", &PROBE_N, &[(p_day_1) as u32], false); }
    let mut l_r_2: i32 = 0;
    let mut l_sum_3: i32 = 0;
    let mut l_i_4: i32 = 0;
    let r_ = 'f: {
    l_r_2 = ((F::Rand_Range(c, ((101i32) as u32))) as i32);
    if (((((c.m.r32((p_day_1).wrapping_add(1360)) as i32)) & (4096i32))) != 0) {
        break 'f 5i32;
    }
    if (((((c.m.r32((p_day_1).wrapping_add(1360)) as i32)) & (2048i32))) != 0) {
        break 'f 0i32;
    }
    l_sum_3 = 0i32;
    {
        l_i_4 = 0i32;
        'l1: loop {
            if !(((((l_i_4) < (6i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let r_ = { let a_ = (((c.m.r32((p_day_1).wrapping_add(2908))).wrapping_add((((1i32) as u32)).wrapping_mul(192))).wrapping_add(((((c.m.r32((p_day_1).wrapping_add(2932)) as i32)) as u32)).wrapping_mul(24))).wrapping_add((((l_i_4) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let n_ = (l_sum_3).wrapping_add(r_); l_sum_3 = n_; };
                if (((((l_r_2) < (l_sum_3)) as i32)) != 0) {
                    break 'f l_i_4;
                }
            }
            { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
        }
    }
    break 'f 5i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "SimEv01_Roll", &PROBE_N, &[], true); }
    r_
}

pub fn SimEv01(c: &mut Ctx, mut p_day_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SimEv01", &PROBE_N, &[(p_day_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_hp_2: i32 = 0;
    let r_ = 'f: {
        's1: {
            let sv3: i32 = (c.m.r32((p_day_1).wrapping_add(2920)) as i32);
            match sv3 {
                0i32 => {
                    { let v_ = F::SimEv01_Roll(c, p_day_1); let a_ = sym::gSimTrainOutcome1; c.m.w32(a_, (v_) as u32); };
                    { let v_ = 2i32; let a_ = (p_day_1).wrapping_add(2936); c.m.w32(a_, (v_) as u32); };
                    F::SimDay_Cmd(c, 5i32);
                    { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                    break 's1;
                }
                1i32 => {
                    { let v_ = 89i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                    if ((F::Rand_Range(c, ((2i32) as u32))) != 0) {
                        { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 21i32); };
                    } else {
                        { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 22i32); };
                    }
                    { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                    break 's1;
                }
                2i32 => {
                    { let v_ = ((c.m.r32(sym::gSimTrainOutcome1) as i32)).wrapping_add(90i32); let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                    { let v_ = { let a_ = (((c.m.r32((p_day_1).wrapping_add(2908))).wrapping_add((((1i32) as u32)).wrapping_mul(192))).wrapping_add(96)).wrapping_add(((((c.m.r32(sym::gSimTrainOutcome1) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    { let v_ = { let a_ = (((c.m.r32((p_day_1).wrapping_add(2908))).wrapping_add((((1i32) as u32)).wrapping_mul(192))).wrapping_add(120)).wrapping_add(((((c.m.r32(sym::gSimTrainOutcome1) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    { let v_ = { let a_ = (((c.m.r32((p_day_1).wrapping_add(2908))).wrapping_add((((1i32) as u32)).wrapping_mul(192))).wrapping_add(144)).wrapping_add(((((c.m.r32(sym::gSimTrainOutcome1) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (fp.wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    { let v_ = { let a_ = (((c.m.r32((p_day_1).wrapping_add(2908))).wrapping_add((((1i32) as u32)).wrapping_mul(192))).wrapping_add(168)).wrapping_add(((((c.m.r32(sym::gSimTrainOutcome1) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (fp.wrapping_add(8)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    l_hp_2 = { let a_ = (((c.m.r32((p_day_1).wrapping_add(2908))).wrapping_add((((1i32) as u32)).wrapping_mul(192))).wrapping_add(72)).wrapping_add(((((c.m.r32(sym::gSimTrainOutcome1) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
                    if ((((((c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) == ((c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32))) as i32)) != 0) {
                        { let a1_ = (c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32); F::SimDay_AddChange(c, 0i32, a1_) };
                    } else {
                        { let a1_ = ((({ let a0_ = (((((c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)).wrapping_sub((c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32))).wrapping_add(1i32)) as u32); F::Rand_Range(c, a0_) }).wrapping_add((((c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) as u32))) as i32); F::SimDay_AddChange(c, 0i32, a1_) };
                    }
                    if ((((((c.m.r32((fp.wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) == ((c.m.r32((fp.wrapping_add(8)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32))) as i32)) != 0) {
                        { let a1_ = (c.m.r32((fp.wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32); F::SimDay_AddChange(c, 1i32, a1_) };
                    } else {
                        { let a1_ = ((({ let a0_ = (((((c.m.r32((fp.wrapping_add(8)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)).wrapping_sub((c.m.r32((fp.wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32))).wrapping_add(1i32)) as u32); F::Rand_Range(c, a0_) }).wrapping_add((((c.m.r32((fp.wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) as u32))) as i32); F::SimDay_AddChange(c, 1i32, a1_) };
                    }
                    F::SimDay_AddChange(c, 2i32, l_hp_2);
                    F::SimDay_Cmd(c, 26i32);
                    { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                    break 's1;
                }
                3i32 => {
                    { let v_ = (1i32).wrapping_neg(); let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                    break 'f 1i32;
                }
                _ => {
                }
            }
        }
        break 'f 0i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "SimEv01", &PROBE_N, &[], true); }
    r_
}

pub fn SimEv02_Roll(c: &mut Ctx, mut p_day_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SimEv02_Roll", &PROBE_N, &[(p_day_1) as u32], false); }
    let mut l_r_2: i32 = 0;
    let mut l_sum_3: i32 = 0;
    let mut l_i_4: i32 = 0;
    let r_ = 'f: {
    l_r_2 = ((F::Rand_Range(c, ((101i32) as u32))) as i32);
    if (((((c.m.r32((p_day_1).wrapping_add(1360)) as i32)) & (4096i32))) != 0) {
        break 'f 5i32;
    }
    if (((((c.m.r32((p_day_1).wrapping_add(1360)) as i32)) & (2048i32))) != 0) {
        break 'f 0i32;
    }
    l_sum_3 = 0i32;
    {
        l_i_4 = 0i32;
        'l1: loop {
            if !(((((l_i_4) < (6i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let r_ = { let a_ = (((c.m.r32((p_day_1).wrapping_add(2908))).wrapping_add((((2i32) as u32)).wrapping_mul(192))).wrapping_add(((((c.m.r32((p_day_1).wrapping_add(2932)) as i32)) as u32)).wrapping_mul(24))).wrapping_add((((l_i_4) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let n_ = (l_sum_3).wrapping_add(r_); l_sum_3 = n_; };
                if (((((l_r_2) < (l_sum_3)) as i32)) != 0) {
                    break 'f l_i_4;
                }
            }
            { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
        }
    }
    break 'f 5i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "SimEv02_Roll", &PROBE_N, &[], true); }
    r_
}

pub fn SimEv02(c: &mut Ctx, mut p_day_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SimEv02", &PROBE_N, &[(p_day_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_hp_2: i32 = 0;
    let r_ = 'f: {
        's1: {
            let sv3: i32 = (c.m.r32((p_day_1).wrapping_add(2920)) as i32);
            match sv3 {
                0i32 => {
                    { let v_ = F::SimEv02_Roll(c, p_day_1); let a_ = sym::gSimTrainOutcome2; c.m.w32(a_, (v_) as u32); };
                    { let v_ = 3i32; let a_ = (p_day_1).wrapping_add(2936); c.m.w32(a_, (v_) as u32); };
                    F::SimDay_Cmd(c, 5i32);
                    { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                    break 's1;
                }
                1i32 => {
                    { let v_ = 89i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                    if ((F::Rand_Range(c, ((2i32) as u32))) != 0) {
                        { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 21i32); };
                    } else {
                        { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 22i32); };
                    }
                    { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                    break 's1;
                }
                2i32 => {
                    { let v_ = ((c.m.r32(sym::gSimTrainOutcome2) as i32)).wrapping_add(90i32); let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                    { let v_ = { let a_ = (((c.m.r32((p_day_1).wrapping_add(2908))).wrapping_add((((2i32) as u32)).wrapping_mul(192))).wrapping_add(96)).wrapping_add(((((c.m.r32(sym::gSimTrainOutcome2) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    { let v_ = { let a_ = (((c.m.r32((p_day_1).wrapping_add(2908))).wrapping_add((((2i32) as u32)).wrapping_mul(192))).wrapping_add(120)).wrapping_add(((((c.m.r32(sym::gSimTrainOutcome2) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    { let v_ = { let a_ = (((c.m.r32((p_day_1).wrapping_add(2908))).wrapping_add((((2i32) as u32)).wrapping_mul(192))).wrapping_add(144)).wrapping_add(((((c.m.r32(sym::gSimTrainOutcome2) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (fp.wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    { let v_ = { let a_ = (((c.m.r32((p_day_1).wrapping_add(2908))).wrapping_add((((2i32) as u32)).wrapping_mul(192))).wrapping_add(168)).wrapping_add(((((c.m.r32(sym::gSimTrainOutcome2) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (fp.wrapping_add(8)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    l_hp_2 = { let a_ = (((c.m.r32((p_day_1).wrapping_add(2908))).wrapping_add((((2i32) as u32)).wrapping_mul(192))).wrapping_add(72)).wrapping_add(((((c.m.r32(sym::gSimTrainOutcome2) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
                    if ((((((c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) == ((c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32))) as i32)) != 0) {
                        { let a1_ = (c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32); F::SimDay_AddChange(c, 0i32, a1_) };
                    } else {
                        { let a1_ = ((({ let a0_ = (((((c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)).wrapping_sub((c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32))).wrapping_add(1i32)) as u32); F::Rand_Range(c, a0_) }).wrapping_add((((c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) as u32))) as i32); F::SimDay_AddChange(c, 0i32, a1_) };
                    }
                    if ((((((c.m.r32((fp.wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) == ((c.m.r32((fp.wrapping_add(8)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32))) as i32)) != 0) {
                        { let a1_ = (c.m.r32((fp.wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32); F::SimDay_AddChange(c, 1i32, a1_) };
                    } else {
                        { let a1_ = ((({ let a0_ = (((((c.m.r32((fp.wrapping_add(8)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)).wrapping_sub((c.m.r32((fp.wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32))).wrapping_add(1i32)) as u32); F::Rand_Range(c, a0_) }).wrapping_add((((c.m.r32((fp.wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) as u32))) as i32); F::SimDay_AddChange(c, 1i32, a1_) };
                    }
                    F::SimDay_AddChange(c, 2i32, l_hp_2);
                    F::SimDay_Cmd(c, 26i32);
                    { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                    break 's1;
                }
                3i32 => {
                    { let v_ = (1i32).wrapping_neg(); let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                    break 'f 1i32;
                }
                _ => {
                }
            }
        }
        break 'f 0i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "SimEv02", &PROBE_N, &[], true); }
    r_
}

pub fn SimEv03(c: &mut Ctx, mut p_day_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SimEv03", &PROBE_N, &[(p_day_1) as u32], false); }
    let r_ = 'f: {
    's1: {
        let sv2: i32 = (c.m.r32((p_day_1).wrapping_add(2920)) as i32);
        match sv2 {
            0i32 => {
                { let v_ = 10i32; let a_ = (p_day_1).wrapping_add(2936); c.m.w32(a_, (v_) as u32); };
                F::SimDay_Cmd(c, 5i32);
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            1i32 => {
                { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 24i32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            2i32 => {
                if ((((({ let a_ = (p_day_1).wrapping_add(2924); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); n_ }) > (60i32)) as i32)) != 0) {
                    { let v_ = 96i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                    { let a1_ = (rt::div_i32(F::SimDay_GetStat(c, 2i32, 0i32), 2i32)).wrapping_add(5i32); F::SimDay_AddChange(c, 2i32, a1_) };
                    F::SimDay_Cmd(c, 26i32);
                    { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                    { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                }
                break 's1;
            }
            3i32 => {
                { let v_ = 99i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let a1_ = (((!(F::Rand_Range(c, ((1i32) as u32))))) as i32); F::SimDay_AddChange(c, 0i32, a1_) };
                { let a1_ = (((!(F::Rand_Range(c, ((1i32) as u32))))) as i32); F::SimDay_AddChange(c, 1i32, a1_) };
                F::SimDay_Cmd(c, 26i32);
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            4i32 => {
                { let v_ = (1i32).wrapping_neg(); let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                break 'f 1i32;
            }
            _ => {
            }
        }
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "SimEv03", &PROBE_N, &[], true); }
    r_
}

pub fn SimEv04(c: &mut Ctx, mut p_day_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SimEv04", &PROBE_N, &[(p_day_1) as u32], false); }
    let r_ = 'f: {
    's1: {
        let sv2: i32 = (c.m.r32((p_day_1).wrapping_add(2920)) as i32);
        match sv2 {
            0i32 => {
                { let v_ = 10i32; let a_ = (p_day_1).wrapping_add(2936); c.m.w32(a_, (v_) as u32); };
                F::SimDay_Cmd(c, 5i32);
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            1i32 => {
                { let v_ = 97i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let a1_ = (100i32).wrapping_sub(F::SimDay_GetStat(c, 2i32, 0i32)); F::SimDay_AddChange(c, 2i32, a1_) };
                F::SimDay_Cmd(c, 26i32);
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            2i32 => {
                { let v_ = (1i32).wrapping_neg(); let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                break 'f 1i32;
            }
            _ => {
            }
        }
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "SimEv04", &PROBE_N, &[], true); }
    r_
}

pub fn SimEv05(c: &mut Ctx, mut p_day_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SimEv05", &PROBE_N, &[(p_day_1) as u32], false); }
    let r_ = 'f: {
    's1: {
        let sv2: i32 = (c.m.r32((p_day_1).wrapping_add(2920)) as i32);
        match sv2 {
            0i32 => {
                { let v_ = 11i32; let a_ = (p_day_1).wrapping_add(2936); c.m.w32(a_, (v_) as u32); };
                F::SimDay_Cmd(c, 5i32);
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            1i32 => {
                { let v_ = 98i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 23i32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            2i32 => {
                { let v_ = 100i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            3i32 => {
                { let v_ = 99i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let a1_ = (((F::Rand_Range(c, ((3i32) as u32))).wrapping_neg()) as i32); F::SimDay_AddChange(c, 0i32, a1_) };
                { let a1_ = (((F::Rand_Range(c, ((3i32) as u32))).wrapping_neg()) as i32); F::SimDay_AddChange(c, 1i32, a1_) };
                F::SimDay_Cmd(c, 26i32);
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            4i32 => {
                { let v_ = (1i32).wrapping_neg(); let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                break 'f 1i32;
            }
            _ => {
            }
        }
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "SimEv05", &PROBE_N, &[], true); }
    r_
}

pub fn SimEv06(c: &mut Ctx, mut p_day_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SimEv06", &PROBE_N, &[(p_day_1) as u32], false); }
    let r_ = 'f: {
    's1: {
        let sv2: i32 = (c.m.r32((p_day_1).wrapping_add(2920)) as i32);
        match sv2 {
            0i32 => {
                { let v_ = 11i32; let a_ = (p_day_1).wrapping_add(2936); c.m.w32(a_, (v_) as u32); };
                F::SimDay_Cmd(c, 5i32);
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            1i32 => {
                { let v_ = 98i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 23i32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            2i32 => {
                { let v_ = (1i32).wrapping_neg(); let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                F::SimDay_Cmd(c, 25i32);
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            3i32 => {
                { let v_ = ((F::Rand_Range(c, ((12i32) as u32))) as i32); let a_ = (p_day_1).wrapping_add(2936); c.m.w32(a_, (v_) as u32); };
                { let v_ = 11i32; let a_ = (p_day_1).wrapping_add(2940); c.m.w32(a_, (v_) as u32); };
                F::SimDay_Cmd(c, 5i32);
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            4i32 => {
                F::SimDay_Cmd(c, 6i32);
                { let v_ = 101i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            5i32 => {
                { let v_ = 102i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let a1_ = (((({ let a0_ = (({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1652); (c.m.r32(a_) as i32) }) as u32); F::Rand_Range(c, a0_) }).wrapping_mul(((100i32) as u32))).wrapping_add(((100i32) as u32))) as i32); F::SimDay_AddChange(c, 3i32, a1_) };
                F::SimDay_Cmd(c, 26i32);
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            6i32 => {
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            7i32 => {
                { let v_ = (1i32).wrapping_neg(); let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                F::SimDay_Cmd(c, 7i32);
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            8i32 => {
                { let v_ = 99i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let a1_ = (((F::Rand_Range(c, ((3i32) as u32))).wrapping_neg()) as i32); F::SimDay_AddChange(c, 0i32, a1_) };
                { let a1_ = (((F::Rand_Range(c, ((3i32) as u32))).wrapping_neg()) as i32); F::SimDay_AddChange(c, 1i32, a1_) };
                F::SimDay_Cmd(c, 26i32);
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            9i32 => {
                { let v_ = (1i32).wrapping_neg(); let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                break 'f 1i32;
            }
            _ => {
            }
        }
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "SimEv06", &PROBE_N, &[], true); }
    r_
}

pub fn SimEv07(c: &mut Ctx, mut p_day_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SimEv07", &PROBE_N, &[(p_day_1) as u32], false); }
    let r_ = 'f: {
    's1: {
        let sv2: i32 = (c.m.r32((p_day_1).wrapping_add(2920)) as i32);
        match sv2 {
            0i32 => {
                { let v_ = 11i32; let a_ = (p_day_1).wrapping_add(2936); c.m.w32(a_, (v_) as u32); };
                F::SimDay_Cmd(c, 5i32);
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            1i32 => {
                { let v_ = 98i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 23i32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            2i32 => {
                { let v_ = (1i32).wrapping_neg(); let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                F::SimDay_Cmd(c, 25i32);
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            3i32 => {
                { let v_ = ((F::Rand_Range(c, ((12i32) as u32))) as i32); let a_ = (p_day_1).wrapping_add(2936); c.m.w32(a_, (v_) as u32); };
                { let v_ = 10i32; let a_ = (p_day_1).wrapping_add(2940); c.m.w32(a_, (v_) as u32); };
                F::SimDay_Cmd(c, 5i32);
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            4i32 => {
                F::SimDay_Cmd(c, 6i32);
                { let v_ = 103i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            5i32 => {
                F::SimDay_Cmd(c, 17i32);
                { let v_ = 104i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            6i32 => {
                if ((((((c.m.r32((p_day_1).wrapping_add(2956)) as i32)) != (0i32)) as i32)) != 0) {
                    { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                } else {
                    F::SimDay_Cmd(c, 7i32);
                    { let v_ = 8i32; let a_ = (p_day_1).wrapping_add(2920); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                }
                break 's1;
            }
            7i32 => {
                F::SimDay_Cmd(c, 7i32);
                F::SimDay_GiveItem(c);
                { let v_ = 105i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            8i32 => {
                { let v_ = 99i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let a1_ = (((F::Rand_Range(c, ((3i32) as u32))).wrapping_neg()) as i32); F::SimDay_AddChange(c, 0i32, a1_) };
                { let a1_ = (((F::Rand_Range(c, ((3i32) as u32))).wrapping_neg()) as i32); F::SimDay_AddChange(c, 1i32, a1_) };
                F::SimDay_Cmd(c, 26i32);
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            9i32 => {
                { let v_ = (1i32).wrapping_neg(); let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                break 'f 1i32;
            }
            _ => {
            }
        }
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "SimEv07", &PROBE_N, &[], true); }
    r_
}

pub fn SimEv08(c: &mut Ctx, mut p_day_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SimEv08", &PROBE_N, &[(p_day_1) as u32], false); }
    let r_ = 'f: {
    's1: {
        let sv2: i32 = (c.m.r32((p_day_1).wrapping_add(2920)) as i32);
        match sv2 {
            0i32 => {
                { let v_ = 11i32; let a_ = (p_day_1).wrapping_add(2936); c.m.w32(a_, (v_) as u32); };
                F::SimDay_Cmd(c, 5i32);
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            1i32 => {
                { let v_ = 98i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 23i32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            2i32 => {
                { let v_ = (1i32).wrapping_neg(); let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let v_ = 7i32; let a_ = (p_day_1).wrapping_add(2940); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2936); c.m.w32(a_, (v_) as u32); };
                F::SimDay_Cmd(c, 5i32);
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            3i32 => {
                F::SimDay_Cmd(c, 6i32);
                { let v_ = 106i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            4i32 => {
                { let v_ = 107i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            5i32 => {
                { let a1_ = rt::div_i32(F::SimDay_GetStat(c, 2i32, 0i32), 5i32); F::SimDay_AddChange(c, 2i32, a1_) };
                F::SimDay_Cmd(c, 26i32);
                { let v_ = 108i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            6i32 => {
                { let v_ = 110i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            7i32 => {
                { let v_ = (1i32).wrapping_neg(); let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                F::SimDay_Cmd(c, 7i32);
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            8i32 => {
                { let v_ = 99i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let a1_ = (((F::Rand_Range(c, ((3i32) as u32))).wrapping_neg()) as i32); F::SimDay_AddChange(c, 0i32, a1_) };
                { let a1_ = (((F::Rand_Range(c, ((3i32) as u32))).wrapping_neg()) as i32); F::SimDay_AddChange(c, 1i32, a1_) };
                F::SimDay_Cmd(c, 26i32);
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            9i32 => {
                { let v_ = (1i32).wrapping_neg(); let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                break 'f 1i32;
            }
            _ => {
            }
        }
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "SimEv08", &PROBE_N, &[], true); }
    r_
}

pub fn SimEv09(c: &mut Ctx, mut p_day_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SimEv09", &PROBE_N, &[(p_day_1) as u32], false); }
    let r_ = 'f: {
    's1: {
        let sv2: i32 = (c.m.r32((p_day_1).wrapping_add(2920)) as i32);
        match sv2 {
            0i32 => {
                { let v_ = 11i32; let a_ = (p_day_1).wrapping_add(2936); c.m.w32(a_, (v_) as u32); };
                F::SimDay_Cmd(c, 5i32);
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            1i32 => {
                { let v_ = 98i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 23i32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            2i32 => {
                { let v_ = (1i32).wrapping_neg(); let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let v_ = 7i32; let a_ = (p_day_1).wrapping_add(2940); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2936); c.m.w32(a_, (v_) as u32); };
                F::SimDay_Cmd(c, 5i32);
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            3i32 => {
                F::SimDay_Cmd(c, 6i32);
                { let v_ = 106i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            4i32 => {
                { let v_ = 107i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            5i32 => {
                { let a1_ = rt::div_i32(F::SimDay_GetStat(c, 2i32, 0i32), 5i32); F::SimDay_AddChange(c, 2i32, a1_) };
                F::SimDay_Cmd(c, 26i32);
                { let v_ = 108i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            6i32 => {
                { let a1_ = (((F::Rand_Range(c, ((5i32) as u32))).wrapping_add(((1i32) as u32))) as i32); F::SimDay_AddChange(c, 0i32, a1_) };
                F::SimDay_Cmd(c, 26i32);
                { let v_ = 109i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            7i32 => {
                { let v_ = 110i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            8i32 => {
                { let v_ = (1i32).wrapping_neg(); let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                break 'f 1i32;
            }
            _ => {
            }
        }
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "SimEv09", &PROBE_N, &[], true); }
    r_
}

pub fn SimEv10(c: &mut Ctx, mut p_day_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SimEv10", &PROBE_N, &[(p_day_1) as u32], false); }
    let r_ = 'f: {
    's1: {
        let sv2: i32 = (c.m.r32((p_day_1).wrapping_add(2920)) as i32);
        match sv2 {
            0i32 => {
                { let v_ = 11i32; let a_ = (p_day_1).wrapping_add(2936); c.m.w32(a_, (v_) as u32); };
                F::SimDay_Cmd(c, 5i32);
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            1i32 => {
                { let v_ = 98i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 23i32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            2i32 => {
                { let v_ = (1i32).wrapping_neg(); let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let v_ = 7i32; let a_ = (p_day_1).wrapping_add(2940); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2936); c.m.w32(a_, (v_) as u32); };
                F::SimDay_Cmd(c, 5i32);
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            3i32 => {
                F::SimDay_Cmd(c, 6i32);
                { let v_ = 106i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            4i32 => {
                { let v_ = 107i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            5i32 => {
                { let a1_ = rt::div_i32(F::SimDay_GetStat(c, 2i32, 0i32), 5i32); F::SimDay_AddChange(c, 2i32, a1_) };
                F::SimDay_Cmd(c, 26i32);
                { let v_ = 108i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            6i32 => {
                { let a1_ = (((F::Rand_Range(c, ((5i32) as u32))).wrapping_add(((1i32) as u32))) as i32); F::SimDay_AddChange(c, 1i32, a1_) };
                F::SimDay_Cmd(c, 26i32);
                { let v_ = 109i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            7i32 => {
                { let v_ = 110i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            8i32 => {
                { let v_ = (1i32).wrapping_neg(); let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                break 'f 1i32;
            }
            _ => {
            }
        }
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "SimEv10", &PROBE_N, &[], true); }
    r_
}

pub fn SimEv11(c: &mut Ctx, mut p_day_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SimEv11", &PROBE_N, &[(p_day_1) as u32], false); }
    let r_ = 'f: {
    's1: {
        let sv2: i32 = (c.m.r32((p_day_1).wrapping_add(2920)) as i32);
        match sv2 {
            0i32 => {
                { let v_ = 11i32; let a_ = (p_day_1).wrapping_add(2936); c.m.w32(a_, (v_) as u32); };
                F::SimDay_Cmd(c, 5i32);
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            1i32 => {
                { let v_ = 98i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 23i32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            2i32 => {
                { let v_ = (1i32).wrapping_neg(); let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let v_ = 7i32; let a_ = (p_day_1).wrapping_add(2940); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2936); c.m.w32(a_, (v_) as u32); };
                F::SimDay_Cmd(c, 5i32);
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            3i32 => {
                F::SimDay_Cmd(c, 6i32);
                { let v_ = 106i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            4i32 => {
                { let v_ = 107i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            5i32 => {
                { let a1_ = rt::div_i32(F::SimDay_GetStat(c, 2i32, 0i32), 5i32); F::SimDay_AddChange(c, 2i32, a1_) };
                F::SimDay_Cmd(c, 26i32);
                { let v_ = 108i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            6i32 => {
                { let a1_ = (((F::Rand_Range(c, ((5i32) as u32))).wrapping_add(((1i32) as u32))) as i32); F::SimDay_AddChange(c, 0i32, a1_) };
                { let a1_ = (((F::Rand_Range(c, ((5i32) as u32))).wrapping_add(((1i32) as u32))) as i32); F::SimDay_AddChange(c, 1i32, a1_) };
                F::SimDay_Cmd(c, 26i32);
                { let v_ = 109i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            7i32 => {
                { let v_ = 110i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            8i32 => {
                { let v_ = (1i32).wrapping_neg(); let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                break 'f 1i32;
            }
            _ => {
            }
        }
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "SimEv11", &PROBE_N, &[], true); }
    r_
}

pub fn SimEv12(c: &mut Ctx, mut p_day_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SimEv12", &PROBE_N, &[(p_day_1) as u32], false); }
    let r_ = 'f: {
    's1: {
        let sv2: i32 = (c.m.r32((p_day_1).wrapping_add(2920)) as i32);
        match sv2 {
            0i32 => {
                { let v_ = 11i32; let a_ = (p_day_1).wrapping_add(2936); c.m.w32(a_, (v_) as u32); };
                F::SimDay_Cmd(c, 5i32);
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            1i32 => {
                { let v_ = 98i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 23i32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            2i32 => {
                { let v_ = (1i32).wrapping_neg(); let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                F::SimDay_Cmd(c, 25i32);
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            3i32 => {
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2936); c.m.w32(a_, (v_) as u32); };
                { let v_ = 7i32; let a_ = (p_day_1).wrapping_add(2940); c.m.w32(a_, (v_) as u32); };
                F::SimDay_Cmd(c, 5i32);
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            4i32 => {
                F::SimDay_Cmd(c, 6i32);
                { let v_ = 106i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            5i32 => {
                { let v_ = 111i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            6i32 => {
                { let v_ = 112i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            7i32 => {
                { let v_ = 113i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let a1_ = (rt::div_i32((F::SimDay_GetStat(c, 2i32, 0i32)).wrapping_mul((((F::Rand_Range(c, ((16i32) as u32))).wrapping_add(((5i32) as u32))) as i32)), 100i32)).wrapping_neg(); F::SimDay_AddChange(c, 2i32, a1_) };
                F::SimDay_Cmd(c, 26i32);
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            8i32 => {
                { let v_ = 102i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let a1_ = (((F::Rand_Range(c, ((4001i32) as u32))).wrapping_add(((1000i32) as u32))) as i32); F::SimDay_AddChange(c, 3i32, a1_) };
                F::SimDay_Cmd(c, 26i32);
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            9i32 => {
                { let v_ = 114i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            10i32 => {
                { let v_ = (1i32).wrapping_neg(); let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                F::SimDay_Cmd(c, 7i32);
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            11i32 => {
                { let v_ = 99i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let a1_ = (((F::Rand_Range(c, ((3i32) as u32))).wrapping_neg()) as i32); F::SimDay_AddChange(c, 0i32, a1_) };
                { let a1_ = (((F::Rand_Range(c, ((3i32) as u32))).wrapping_neg()) as i32); F::SimDay_AddChange(c, 1i32, a1_) };
                F::SimDay_Cmd(c, 26i32);
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            12i32 => {
                { let v_ = (1i32).wrapping_neg(); let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                break 'f 1i32;
            }
            _ => {
            }
        }
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "SimEv12", &PROBE_N, &[], true); }
    r_
}

pub fn SimEv13(c: &mut Ctx, mut p_day_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SimEv13", &PROBE_N, &[(p_day_1) as u32], false); }
    let r_ = 'f: {
    's1: {
        let sv2: i32 = (c.m.r32((p_day_1).wrapping_add(2920)) as i32);
        match sv2 {
            0i32 => {
                { let v_ = 11i32; let a_ = (p_day_1).wrapping_add(2936); c.m.w32(a_, (v_) as u32); };
                F::SimDay_Cmd(c, 5i32);
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            1i32 => {
                { let v_ = 98i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 23i32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            2i32 => {
                { let v_ = (1i32).wrapping_neg(); let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                F::SimDay_Cmd(c, 25i32);
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            3i32 => {
                { let v_ = (1i32).wrapping_neg(); let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let v_ = 4i32; let a_ = (p_day_1).wrapping_add(2936); c.m.w32(a_, (v_) as u32); };
                { let v_ = 6i32; let a_ = (p_day_1).wrapping_add(2940); c.m.w32(a_, (v_) as u32); };
                F::SimDay_Cmd(c, 5i32);
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            4i32 => {
                F::SimDay_Cmd(c, 6i32);
                { let v_ = 115i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            5i32 => {
                { let v_ = 116i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            6i32 => {
                { let v_ = 117i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                F::SimDay_Cmd(c, 17i32);
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            7i32 => {
                { let v_ = (1i32).wrapping_neg(); let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                if ((((((c.m.r32((p_day_1).wrapping_add(2956)) as i32)) == (0i32)) as i32)) != 0) {
                    { let v_ = 14i32; let a_ = (p_day_1).wrapping_add(2920); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                } else {
                    { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                }
                break 's1;
            }
            8i32 => {
                if (((((F::SimDay_GetStat(c, 3i32, 0i32)) < (10000i32)) as i32)) != 0) {
                    { let v_ = 13i32; let a_ = (p_day_1).wrapping_add(2920); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                } else {
                    { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                }
                break 's1;
            }
            9i32 => {
                { let v_ = 118i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                F::SimDay_AddChange(c, 3i32, (10000i32).wrapping_neg());
                F::SimDay_Cmd(c, 26i32);
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            10i32 => {
                { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 29i32); };
                { let v_ = 120i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            11i32 => {
                { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 34i32); };
                { let v_ = 121i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            12i32 => {
                { let v_ = 122i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 15i32; let a_ = (p_day_1).wrapping_add(2920); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            13i32 => {
                { let v_ = 215i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            14i32 => {
                { let v_ = 119i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            15i32 => {
                { let v_ = (1i32).wrapping_neg(); let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                F::SimDay_Cmd(c, 7i32);
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            16i32 => {
                { let v_ = 99i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let a1_ = (((F::Rand_Range(c, ((3i32) as u32))).wrapping_neg()) as i32); F::SimDay_AddChange(c, 0i32, a1_) };
                { let a1_ = (((F::Rand_Range(c, ((3i32) as u32))).wrapping_neg()) as i32); F::SimDay_AddChange(c, 1i32, a1_) };
                F::SimDay_Cmd(c, 26i32);
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            17i32 => {
                { let v_ = (1i32).wrapping_neg(); let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                break 'f 1i32;
            }
            _ => {
            }
        }
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "SimEv13", &PROBE_N, &[], true); }
    r_
}

pub fn SimEv14(c: &mut Ctx, mut p_day_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SimEv14", &PROBE_N, &[(p_day_1) as u32], false); }
    let r_ = 'f: {
    's1: {
        let sv2: i32 = (c.m.r32((p_day_1).wrapping_add(2920)) as i32);
        match sv2 {
            0i32 => {
                { let v_ = 11i32; let a_ = (p_day_1).wrapping_add(2936); c.m.w32(a_, (v_) as u32); };
                F::SimDay_Cmd(c, 5i32);
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            1i32 => {
                { let v_ = 98i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 23i32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            2i32 => {
                { let v_ = (1i32).wrapping_neg(); let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                F::SimDay_Cmd(c, 25i32);
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            3i32 => {
                { let v_ = (1i32).wrapping_neg(); let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let v_ = 4i32; let a_ = (p_day_1).wrapping_add(2936); c.m.w32(a_, (v_) as u32); };
                { let v_ = 6i32; let a_ = (p_day_1).wrapping_add(2940); c.m.w32(a_, (v_) as u32); };
                F::SimDay_Cmd(c, 5i32);
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            4i32 => {
                F::SimDay_Cmd(c, 6i32);
                { let v_ = 115i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            5i32 => {
                { let v_ = 116i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            6i32 => {
                { let v_ = 117i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                F::SimDay_Cmd(c, 17i32);
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            7i32 => {
                { let v_ = (1i32).wrapping_neg(); let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                if ((((((c.m.r32((p_day_1).wrapping_add(2956)) as i32)) == (0i32)) as i32)) != 0) {
                    { let v_ = 15i32; let a_ = (p_day_1).wrapping_add(2920); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                } else {
                    { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                }
                break 's1;
            }
            8i32 => {
                if (((((F::SimDay_GetStat(c, 3i32, 0i32)) < (10000i32)) as i32)) != 0) {
                    { let v_ = 14i32; let a_ = (p_day_1).wrapping_add(2920); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                } else {
                    { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                }
                break 's1;
            }
            9i32 => {
                { let v_ = 118i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                F::SimDay_AddChange(c, 3i32, (10000i32).wrapping_neg());
                F::SimDay_Cmd(c, 26i32);
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            10i32 => {
                { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 29i32); };
                { let v_ = 120i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            11i32 => {
                { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 31i32); };
                { let v_ = 129i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            12i32 => {
                { let v_ = 102i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                F::SimDay_AddChange(c, 3i32, 30000i32);
                F::SimDay_Cmd(c, 26i32);
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            13i32 => {
                { let v_ = 125i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 16i32; let a_ = (p_day_1).wrapping_add(2920); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            14i32 => {
                { let v_ = 215i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            15i32 => {
                { let v_ = 119i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            16i32 => {
                { let v_ = (1i32).wrapping_neg(); let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                F::SimDay_Cmd(c, 7i32);
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            17i32 => {
                { let v_ = 99i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let a1_ = (((F::Rand_Range(c, ((3i32) as u32))).wrapping_neg()) as i32); F::SimDay_AddChange(c, 0i32, a1_) };
                { let a1_ = (((F::Rand_Range(c, ((3i32) as u32))).wrapping_neg()) as i32); F::SimDay_AddChange(c, 1i32, a1_) };
                F::SimDay_Cmd(c, 26i32);
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            18i32 => {
                { let v_ = (1i32).wrapping_neg(); let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                break 'f 1i32;
            }
            _ => {
            }
        }
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "SimEv14", &PROBE_N, &[], true); }
    r_
}

pub fn SimEv15(c: &mut Ctx, mut p_day_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SimEv15", &PROBE_N, &[(p_day_1) as u32], false); }
    let r_ = 'f: {
    's1: {
        let sv2: i32 = (c.m.r32((p_day_1).wrapping_add(2920)) as i32);
        match sv2 {
            0i32 => {
                { let v_ = 11i32; let a_ = (p_day_1).wrapping_add(2936); c.m.w32(a_, (v_) as u32); };
                F::SimDay_Cmd(c, 5i32);
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            1i32 => {
                { let v_ = 98i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 23i32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            2i32 => {
                { let v_ = (1i32).wrapping_neg(); let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                F::SimDay_Cmd(c, 25i32);
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            3i32 => {
                { let v_ = (1i32).wrapping_neg(); let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let v_ = 4i32; let a_ = (p_day_1).wrapping_add(2936); c.m.w32(a_, (v_) as u32); };
                { let v_ = 6i32; let a_ = (p_day_1).wrapping_add(2940); c.m.w32(a_, (v_) as u32); };
                F::SimDay_Cmd(c, 5i32);
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            4i32 => {
                F::SimDay_Cmd(c, 6i32);
                { let v_ = 115i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            5i32 => {
                { let v_ = 116i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            6i32 => {
                { let v_ = 117i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                F::SimDay_Cmd(c, 17i32);
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            7i32 => {
                { let v_ = (1i32).wrapping_neg(); let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                if ((((((c.m.r32((p_day_1).wrapping_add(2956)) as i32)) == (0i32)) as i32)) != 0) {
                    { let v_ = 15i32; let a_ = (p_day_1).wrapping_add(2920); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                } else {
                    { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                }
                break 's1;
            }
            8i32 => {
                if (((((F::SimDay_GetStat(c, 3i32, 0i32)) < (10000i32)) as i32)) != 0) {
                    { let v_ = 14i32; let a_ = (p_day_1).wrapping_add(2920); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                } else {
                    { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                }
                break 's1;
            }
            9i32 => {
                { let v_ = 118i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                F::SimDay_AddChange(c, 3i32, (10000i32).wrapping_neg());
                F::SimDay_Cmd(c, 26i32);
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            10i32 => {
                { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 29i32); };
                { let v_ = 120i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            11i32 => {
                { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 31i32); };
                { let v_ = 126i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            12i32 => {
                { let v_ = 127i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                F::SimDay_AddChange(c, 0i32, 5i32);
                F::SimDay_AddChange(c, 1i32, 5i32);
                F::SimDay_Cmd(c, 26i32);
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            13i32 => {
                { let v_ = 125i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 18i32; let a_ = (p_day_1).wrapping_add(2920); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            14i32 => {
                { let v_ = 215i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            15i32 => {
                { let v_ = 119i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            16i32 => {
                { let v_ = (1i32).wrapping_neg(); let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                F::SimDay_Cmd(c, 7i32);
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            17i32 => {
                { let v_ = 99i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let a1_ = (((F::Rand_Range(c, ((3i32) as u32))).wrapping_neg()) as i32); F::SimDay_AddChange(c, 0i32, a1_) };
                { let a1_ = (((F::Rand_Range(c, ((3i32) as u32))).wrapping_neg()) as i32); F::SimDay_AddChange(c, 1i32, a1_) };
                F::SimDay_Cmd(c, 26i32);
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            18i32 => {
                { let v_ = (1i32).wrapping_neg(); let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                break 'f 1i32;
            }
            _ => {
            }
        }
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "SimEv15", &PROBE_N, &[], true); }
    r_
}

pub fn SimEv16(c: &mut Ctx, mut p_day_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SimEv16", &PROBE_N, &[(p_day_1) as u32], false); }
    let r_ = 'f: {
    's1: {
        let sv2: i32 = (c.m.r32((p_day_1).wrapping_add(2920)) as i32);
        match sv2 {
            0i32 => {
                { let v_ = 11i32; let a_ = (p_day_1).wrapping_add(2936); c.m.w32(a_, (v_) as u32); };
                F::SimDay_Cmd(c, 5i32);
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            1i32 => {
                { let v_ = 98i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 23i32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            2i32 => {
                { let v_ = (1i32).wrapping_neg(); let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                F::SimDay_Cmd(c, 25i32);
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            3i32 => {
                { let v_ = (1i32).wrapping_neg(); let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let v_ = 4i32; let a_ = (p_day_1).wrapping_add(2936); c.m.w32(a_, (v_) as u32); };
                { let v_ = 6i32; let a_ = (p_day_1).wrapping_add(2940); c.m.w32(a_, (v_) as u32); };
                F::SimDay_Cmd(c, 5i32);
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            4i32 => {
                F::SimDay_Cmd(c, 6i32);
                { let v_ = 115i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            5i32 => {
                { let v_ = 116i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            6i32 => {
                { let v_ = 117i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                F::SimDay_Cmd(c, 17i32);
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            7i32 => {
                { let v_ = (1i32).wrapping_neg(); let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                if ((((((c.m.r32((p_day_1).wrapping_add(2956)) as i32)) == (0i32)) as i32)) != 0) {
                    { let v_ = 18i32; let a_ = (p_day_1).wrapping_add(2920); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                } else {
                    { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                }
                break 's1;
            }
            8i32 => {
                if (((((F::SimDay_GetStat(c, 3i32, 0i32)) < (10000i32)) as i32)) != 0) {
                    { let v_ = 17i32; let a_ = (p_day_1).wrapping_add(2920); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                } else {
                    { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                }
                break 's1;
            }
            9i32 => {
                { let v_ = 118i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                F::SimDay_AddChange(c, 3i32, (10000i32).wrapping_neg());
                F::SimDay_Cmd(c, 26i32);
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            10i32 => {
                { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 29i32); };
                { let v_ = 120i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            11i32 => {
                { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 31i32); };
                { let v_ = 128i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            12i32 => {
                F::SimDay_Cmd(c, 17i32);
                { let v_ = 104i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            13i32 => {
                if ((((((c.m.r32((p_day_1).wrapping_add(2956)) as i32)) == (0i32)) as i32)) != 0) {
                    { let v_ = 16i32; let a_ = (p_day_1).wrapping_add(2920); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                } else {
                    { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                }
                break 's1;
            }
            14i32 => {
                F::SimDay_GiveItem(c);
                { let v_ = 105i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            15i32 | 16i32 => {
                { let v_ = 125i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 19i32; let a_ = (p_day_1).wrapping_add(2920); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            17i32 => {
                { let v_ = 215i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            18i32 => {
                { let v_ = 119i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            19i32 => {
                { let v_ = (1i32).wrapping_neg(); let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                F::SimDay_Cmd(c, 7i32);
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            20i32 => {
                { let v_ = 99i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let a1_ = (((F::Rand_Range(c, ((3i32) as u32))).wrapping_neg()) as i32); F::SimDay_AddChange(c, 0i32, a1_) };
                { let a1_ = (((F::Rand_Range(c, ((3i32) as u32))).wrapping_neg()) as i32); F::SimDay_AddChange(c, 1i32, a1_) };
                F::SimDay_Cmd(c, 26i32);
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            21i32 => {
                { let v_ = (1i32).wrapping_neg(); let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                break 'f 1i32;
            }
            _ => {
            }
        }
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "SimEv16", &PROBE_N, &[], true); }
    r_
}

pub fn SimEv17(c: &mut Ctx, mut p_day_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SimEv17", &PROBE_N, &[(p_day_1) as u32], false); }
    let r_ = 'f: {
    's1: {
        let sv2: i32 = (c.m.r32((p_day_1).wrapping_add(2920)) as i32);
        match sv2 {
            0i32 => {
                { let v_ = 11i32; let a_ = (p_day_1).wrapping_add(2936); c.m.w32(a_, (v_) as u32); };
                F::SimDay_Cmd(c, 5i32);
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            1i32 => {
                { let v_ = 98i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 23i32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            2i32 => {
                { let v_ = (1i32).wrapping_neg(); let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                F::SimDay_Cmd(c, 25i32);
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            3i32 => {
                { let v_ = (1i32).wrapping_neg(); let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let v_ = 4i32; let a_ = (p_day_1).wrapping_add(2936); c.m.w32(a_, (v_) as u32); };
                { let v_ = 6i32; let a_ = (p_day_1).wrapping_add(2940); c.m.w32(a_, (v_) as u32); };
                F::SimDay_Cmd(c, 5i32);
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            4i32 => {
                F::SimDay_Cmd(c, 6i32);
                { let v_ = 115i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            5i32 => {
                { let v_ = 116i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            6i32 => {
                { let v_ = 117i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                F::SimDay_Cmd(c, 17i32);
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            7i32 => {
                { let v_ = (1i32).wrapping_neg(); let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                if ((((((c.m.r32((p_day_1).wrapping_add(2956)) as i32)) == (0i32)) as i32)) != 0) {
                    { let v_ = 15i32; let a_ = (p_day_1).wrapping_add(2920); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                } else {
                    { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                }
                break 's1;
            }
            8i32 => {
                if (((((F::SimDay_GetStat(c, 3i32, 0i32)) < (10000i32)) as i32)) != 0) {
                    { let v_ = 14i32; let a_ = (p_day_1).wrapping_add(2920); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                } else {
                    { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                }
                break 's1;
            }
            9i32 => {
                { let v_ = 118i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                F::SimDay_AddChange(c, 3i32, (10000i32).wrapping_neg());
                F::SimDay_Cmd(c, 26i32);
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            10i32 => {
                { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 29i32); };
                { let v_ = 120i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            11i32 => {
                { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 31i32); };
                { let v_ = 123i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            12i32 => {
                { let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(524); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 124i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            13i32 => {
                { let v_ = 125i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 16i32; let a_ = (p_day_1).wrapping_add(2920); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            14i32 => {
                { let v_ = 215i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            15i32 => {
                { let v_ = 119i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            16i32 => {
                { let v_ = (1i32).wrapping_neg(); let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                F::SimDay_Cmd(c, 7i32);
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            17i32 => {
                { let v_ = 99i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let a1_ = (((F::Rand_Range(c, ((3i32) as u32))).wrapping_neg()) as i32); F::SimDay_AddChange(c, 0i32, a1_) };
                { let a1_ = (((F::Rand_Range(c, ((3i32) as u32))).wrapping_neg()) as i32); F::SimDay_AddChange(c, 1i32, a1_) };
                F::SimDay_Cmd(c, 26i32);
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            18i32 => {
                { let v_ = (1i32).wrapping_neg(); let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                break 'f 1i32;
            }
            _ => {
            }
        }
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "SimEv17", &PROBE_N, &[], true); }
    r_
}

pub fn SimEv18(c: &mut Ctx, mut p_day_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SimEv18", &PROBE_N, &[(p_day_1) as u32], false); }
    let r_ = 'f: {
    's1: {
        let sv2: i32 = (c.m.r32((p_day_1).wrapping_add(2920)) as i32);
        match sv2 {
            0i32 => {
                { let v_ = 11i32; let a_ = (p_day_1).wrapping_add(2936); c.m.w32(a_, (v_) as u32); };
                F::SimDay_Cmd(c, 5i32);
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            1i32 => {
                { let v_ = 98i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 23i32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            2i32 => {
                { let v_ = (1i32).wrapping_neg(); let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                F::SimDay_Cmd(c, 25i32);
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            3i32 => {
                { let v_ = (1i32).wrapping_neg(); let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let v_ = 5i32; let a_ = (p_day_1).wrapping_add(2936); c.m.w32(a_, (v_) as u32); };
                { let v_ = 5i32; let a_ = (p_day_1).wrapping_add(2940); c.m.w32(a_, (v_) as u32); };
                F::SimDay_Cmd(c, 5i32);
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            4i32 => {
                F::SimDay_Cmd(c, 6i32);
                { let v_ = 130i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            5i32 => {
                { let v_ = 131i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            6i32 => {
                { let a1_ = (100i32).wrapping_sub(F::SimDay_GetStat(c, 2i32, 0i32)); F::SimDay_AddChange(c, 2i32, a1_) };
                F::SimDay_Cmd(c, 26i32);
                { let v_ = 132i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            7i32 => {
                { let v_ = 133i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            8i32 => {
                { let v_ = (1i32).wrapping_neg(); let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                break 'f 1i32;
            }
            _ => {
            }
        }
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "SimEv18", &PROBE_N, &[], true); }
    r_
}

pub fn SimEv19(c: &mut Ctx, mut p_day_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SimEv19", &PROBE_N, &[(p_day_1) as u32], false); }
    let r_ = 'f: {
    's1: {
        let sv2: i32 = (c.m.r32((p_day_1).wrapping_add(2920)) as i32);
        match sv2 {
            0i32 => {
                { let v_ = 11i32; let a_ = (p_day_1).wrapping_add(2936); c.m.w32(a_, (v_) as u32); };
                F::SimDay_Cmd(c, 5i32);
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            1i32 => {
                { let v_ = 98i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 23i32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            2i32 => {
                { let v_ = (1i32).wrapping_neg(); let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                F::SimDay_Cmd(c, 25i32);
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            3i32 => {
                { let v_ = (1i32).wrapping_neg(); let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let v_ = 5i32; let a_ = (p_day_1).wrapping_add(2936); c.m.w32(a_, (v_) as u32); };
                { let v_ = 5i32; let a_ = (p_day_1).wrapping_add(2940); c.m.w32(a_, (v_) as u32); };
                F::SimDay_Cmd(c, 5i32);
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            4i32 => {
                F::SimDay_Cmd(c, 6i32);
                { let v_ = 130i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            5i32 => {
                { let v_ = 134i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            6i32 => {
                { let v_ = 102i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                F::SimDay_AddChange(c, 3i32, 300i32);
                F::SimDay_Cmd(c, 26i32);
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            7i32 => {
                { let v_ = 133i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            8i32 => {
                { let v_ = (1i32).wrapping_neg(); let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                F::SimDay_Cmd(c, 7i32);
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            9i32 => {
                { let v_ = 99i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let a1_ = (((F::Rand_Range(c, ((3i32) as u32))).wrapping_neg()) as i32); F::SimDay_AddChange(c, 0i32, a1_) };
                { let a1_ = (((F::Rand_Range(c, ((3i32) as u32))).wrapping_neg()) as i32); F::SimDay_AddChange(c, 1i32, a1_) };
                F::SimDay_Cmd(c, 26i32);
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            10i32 => {
                { let v_ = (1i32).wrapping_neg(); let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                break 'f 1i32;
            }
            _ => {
            }
        }
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "SimEv19", &PROBE_N, &[], true); }
    r_
}

pub fn SimEv20(c: &mut Ctx, mut p_day_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SimEv20", &PROBE_N, &[(p_day_1) as u32], false); }
    let r_ = 'f: {
    's1: {
        let sv2: i32 = (c.m.r32((p_day_1).wrapping_add(2920)) as i32);
        match sv2 {
            0i32 => {
                { let v_ = 11i32; let a_ = (p_day_1).wrapping_add(2936); c.m.w32(a_, (v_) as u32); };
                F::SimDay_Cmd(c, 5i32);
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            1i32 => {
                { let v_ = 98i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 23i32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            2i32 => {
                { let v_ = (1i32).wrapping_neg(); let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                F::SimDay_Cmd(c, 25i32);
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            3i32 => {
                { let v_ = (1i32).wrapping_neg(); let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let v_ = 5i32; let a_ = (p_day_1).wrapping_add(2936); c.m.w32(a_, (v_) as u32); };
                { let v_ = 5i32; let a_ = (p_day_1).wrapping_add(2940); c.m.w32(a_, (v_) as u32); };
                F::SimDay_Cmd(c, 5i32);
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            4i32 => {
                F::SimDay_Cmd(c, 6i32);
                { let v_ = 130i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            5i32 => {
                { let v_ = 135i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            6i32 => {
                { let v_ = 102i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                F::SimDay_AddChange(c, 3i32, 300i32);
                F::SimDay_Cmd(c, 26i32);
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            7i32 => {
                { let v_ = 133i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            8i32 => {
                { let v_ = (1i32).wrapping_neg(); let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                F::SimDay_Cmd(c, 7i32);
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            9i32 => {
                { let v_ = 99i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let a1_ = (((F::Rand_Range(c, ((3i32) as u32))).wrapping_neg()) as i32); F::SimDay_AddChange(c, 0i32, a1_) };
                { let a1_ = (((F::Rand_Range(c, ((3i32) as u32))).wrapping_neg()) as i32); F::SimDay_AddChange(c, 1i32, a1_) };
                F::SimDay_Cmd(c, 26i32);
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            10i32 => {
                { let v_ = (1i32).wrapping_neg(); let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                break 'f 1i32;
            }
            _ => {
            }
        }
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "SimEv20", &PROBE_N, &[], true); }
    r_
}

pub fn SimEv21(c: &mut Ctx, mut p_day_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SimEv21", &PROBE_N, &[(p_day_1) as u32], false); }
    let r_ = 'f: {
    's1: {
        let sv2: i32 = (c.m.r32((p_day_1).wrapping_add(2920)) as i32);
        match sv2 {
            0i32 => {
                { let v_ = 11i32; let a_ = (p_day_1).wrapping_add(2936); c.m.w32(a_, (v_) as u32); };
                F::SimDay_Cmd(c, 5i32);
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            1i32 => {
                { let v_ = 98i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 23i32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            2i32 => {
                { let v_ = (1i32).wrapping_neg(); let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                F::SimDay_Cmd(c, 25i32);
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            3i32 => {
                { let v_ = (1i32).wrapping_neg(); let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let v_ = 5i32; let a_ = (p_day_1).wrapping_add(2936); c.m.w32(a_, (v_) as u32); };
                { let v_ = 5i32; let a_ = (p_day_1).wrapping_add(2940); c.m.w32(a_, (v_) as u32); };
                F::SimDay_Cmd(c, 5i32);
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            4i32 => {
                F::SimDay_Cmd(c, 6i32);
                { let v_ = 130i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            5i32 => {
                { let v_ = 136i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            6i32 => {
                { let v_ = 102i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                F::SimDay_AddChange(c, 3i32, 300i32);
                F::SimDay_Cmd(c, 26i32);
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            7i32 => {
                { let v_ = 133i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            8i32 => {
                { let v_ = (1i32).wrapping_neg(); let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                F::SimDay_Cmd(c, 7i32);
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            9i32 => {
                { let v_ = 99i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let a1_ = (((F::Rand_Range(c, ((3i32) as u32))).wrapping_neg()) as i32); F::SimDay_AddChange(c, 0i32, a1_) };
                { let a1_ = (((F::Rand_Range(c, ((3i32) as u32))).wrapping_neg()) as i32); F::SimDay_AddChange(c, 1i32, a1_) };
                F::SimDay_Cmd(c, 26i32);
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            10i32 => {
                { let v_ = (1i32).wrapping_neg(); let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                break 'f 1i32;
            }
            _ => {
            }
        }
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "SimEv21", &PROBE_N, &[], true); }
    r_
}

pub fn SimEv22(c: &mut Ctx, mut p_day_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SimEv22", &PROBE_N, &[(p_day_1) as u32], false); }
    let r_ = 'f: {
    's1: {
        let sv2: i32 = (c.m.r32((p_day_1).wrapping_add(2920)) as i32);
        match sv2 {
            0i32 => {
                { let v_ = 11i32; let a_ = (p_day_1).wrapping_add(2936); c.m.w32(a_, (v_) as u32); };
                F::SimDay_Cmd(c, 5i32);
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            1i32 => {
                { let v_ = 98i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 23i32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            2i32 => {
                { let v_ = (1i32).wrapping_neg(); let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                F::SimDay_Cmd(c, 25i32);
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            3i32 => {
                { let v_ = (1i32).wrapping_neg(); let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let v_ = 5i32; let a_ = (p_day_1).wrapping_add(2936); c.m.w32(a_, (v_) as u32); };
                { let v_ = 5i32; let a_ = (p_day_1).wrapping_add(2940); c.m.w32(a_, (v_) as u32); };
                F::SimDay_Cmd(c, 5i32);
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            4i32 => {
                F::SimDay_Cmd(c, 6i32);
                { let v_ = 130i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            5i32 => {
                { let v_ = 137i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            6i32 => {
                { let v_ = 102i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                F::SimDay_AddChange(c, 3i32, 300i32);
                F::SimDay_Cmd(c, 26i32);
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            7i32 => {
                { let v_ = 133i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            8i32 => {
                { let v_ = (1i32).wrapping_neg(); let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                F::SimDay_Cmd(c, 7i32);
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            9i32 => {
                { let v_ = 99i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let a1_ = (((F::Rand_Range(c, ((3i32) as u32))).wrapping_neg()) as i32); F::SimDay_AddChange(c, 0i32, a1_) };
                { let a1_ = (((F::Rand_Range(c, ((3i32) as u32))).wrapping_neg()) as i32); F::SimDay_AddChange(c, 1i32, a1_) };
                F::SimDay_Cmd(c, 26i32);
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            10i32 => {
                { let v_ = (1i32).wrapping_neg(); let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                break 'f 1i32;
            }
            _ => {
            }
        }
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "SimEv22", &PROBE_N, &[], true); }
    r_
}

pub fn SimEv23(c: &mut Ctx, mut p_day_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SimEv23", &PROBE_N, &[(p_day_1) as u32], false); }
    let r_ = 'f: {
    's1: {
        let sv2: i32 = (c.m.r32((p_day_1).wrapping_add(2920)) as i32);
        match sv2 {
            0i32 => {
                { let v_ = 11i32; let a_ = (p_day_1).wrapping_add(2936); c.m.w32(a_, (v_) as u32); };
                F::SimDay_Cmd(c, 5i32);
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            1i32 => {
                { let v_ = 98i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 23i32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            2i32 => {
                { let v_ = (1i32).wrapping_neg(); let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                F::SimDay_Cmd(c, 25i32);
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            3i32 => {
                { let v_ = (1i32).wrapping_neg(); let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let v_ = 6i32; let a_ = (p_day_1).wrapping_add(2936); c.m.w32(a_, (v_) as u32); };
                { let v_ = 8i32; let a_ = (p_day_1).wrapping_add(2940); c.m.w32(a_, (v_) as u32); };
                F::SimDay_Cmd(c, 5i32);
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            4i32 => {
                F::SimDay_Cmd(c, 6i32);
                { let v_ = 138i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            5i32 => {
                { let v_ = 139i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            6i32 => {
                F::SimDay_Cmd(c, 17i32);
                { let v_ = 140i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            7i32 => {
                if ((((((c.m.r32((p_day_1).wrapping_add(2956)) as i32)) != (0i32)) as i32)) != 0) {
                    { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                } else {
                    { let v_ = 13i32; let a_ = (p_day_1).wrapping_add(2920); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                }
                break 's1;
            }
            8i32 => {
                if (((((F::SimDay_GetStat(c, 3i32, 0i32)) < (20000i32)) as i32)) != 0) {
                    { let v_ = 12i32; let a_ = (p_day_1).wrapping_add(2920); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                } else {
                    { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                }
                break 's1;
            }
            9i32 => {
                { let v_ = 142i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                F::SimDay_AddChange(c, 3i32, (20000i32).wrapping_neg());
                F::SimDay_Cmd(c, 26i32);
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            10i32 => {
                { let v_ = (((((c.m.r32((p_day_1).wrapping_add(1360)) as i32)) & ((!(2048i32))))) | (4096i32)); let a_ = (p_day_1).wrapping_add(1360); c.m.w32(a_, (v_) as u32); };
                { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 31i32); };
                { let v_ = 143i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            11i32 => {
                { let v_ = 144i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 14i32; let a_ = (p_day_1).wrapping_add(2920); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            12i32 => {
                { let v_ = 215i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            13i32 => {
                { let v_ = 141i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            14i32 => {
                { let v_ = (1i32).wrapping_neg(); let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                F::SimDay_Cmd(c, 7i32);
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            15i32 => {
                { let v_ = 99i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let a1_ = (((F::Rand_Range(c, ((3i32) as u32))).wrapping_neg()) as i32); F::SimDay_AddChange(c, 0i32, a1_) };
                { let a1_ = (((F::Rand_Range(c, ((3i32) as u32))).wrapping_neg()) as i32); F::SimDay_AddChange(c, 1i32, a1_) };
                F::SimDay_Cmd(c, 26i32);
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            16i32 => {
                { let v_ = (1i32).wrapping_neg(); let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                break 'f 1i32;
            }
            _ => {
            }
        }
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "SimEv23", &PROBE_N, &[], true); }
    r_
}

pub fn SimEv24(c: &mut Ctx, mut p_day_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SimEv24", &PROBE_N, &[(p_day_1) as u32], false); }
    let r_ = 'f: {
    's1: {
        let sv2: i32 = (c.m.r32((p_day_1).wrapping_add(2920)) as i32);
        match sv2 {
            0i32 => {
                { let v_ = 11i32; let a_ = (p_day_1).wrapping_add(2936); c.m.w32(a_, (v_) as u32); };
                F::SimDay_Cmd(c, 5i32);
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            1i32 => {
                { let v_ = 98i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 23i32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            2i32 => {
                { let v_ = (1i32).wrapping_neg(); let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                F::SimDay_Cmd(c, 25i32);
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            3i32 => {
                { let v_ = (1i32).wrapping_neg(); let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let v_ = 6i32; let a_ = (p_day_1).wrapping_add(2936); c.m.w32(a_, (v_) as u32); };
                { let v_ = 8i32; let a_ = (p_day_1).wrapping_add(2940); c.m.w32(a_, (v_) as u32); };
                F::SimDay_Cmd(c, 5i32);
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            4i32 => {
                F::SimDay_Cmd(c, 6i32);
                { let v_ = 138i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            5i32 => {
                { let v_ = 139i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            6i32 => {
                F::SimDay_Cmd(c, 17i32);
                { let v_ = 140i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            7i32 => {
                if ((((((c.m.r32((p_day_1).wrapping_add(2956)) as i32)) != (0i32)) as i32)) != 0) {
                    { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                } else {
                    { let v_ = 13i32; let a_ = (p_day_1).wrapping_add(2920); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                }
                break 's1;
            }
            8i32 => {
                if (((((F::SimDay_GetStat(c, 3i32, 0i32)) < (20000i32)) as i32)) != 0) {
                    { let v_ = 12i32; let a_ = (p_day_1).wrapping_add(2920); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                } else {
                    { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                }
                break 's1;
            }
            9i32 => {
                { let v_ = 142i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                F::SimDay_AddChange(c, 3i32, (20000i32).wrapping_neg());
                F::SimDay_Cmd(c, 26i32);
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            10i32 => {
                { let v_ = (((((c.m.r32((p_day_1).wrapping_add(1360)) as i32)) & ((!(4096i32))))) | (2048i32)); let a_ = (p_day_1).wrapping_add(1360); c.m.w32(a_, (v_) as u32); };
                { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 31i32); };
                { let v_ = 145i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            11i32 => {
                { let v_ = 144i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 14i32; let a_ = (p_day_1).wrapping_add(2920); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            12i32 => {
                { let v_ = 215i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            13i32 => {
                { let v_ = 141i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            14i32 => {
                { let v_ = (1i32).wrapping_neg(); let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                F::SimDay_Cmd(c, 7i32);
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            15i32 => {
                { let v_ = 99i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let a1_ = (((F::Rand_Range(c, ((3i32) as u32))).wrapping_neg()) as i32); F::SimDay_AddChange(c, 0i32, a1_) };
                { let a1_ = (((F::Rand_Range(c, ((3i32) as u32))).wrapping_neg()) as i32); F::SimDay_AddChange(c, 1i32, a1_) };
                F::SimDay_Cmd(c, 26i32);
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            16i32 => {
                { let v_ = (1i32).wrapping_neg(); let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                break 'f 1i32;
            }
            _ => {
            }
        }
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "SimEv24", &PROBE_N, &[], true); }
    r_
}

pub fn SimEv25(c: &mut Ctx, mut p_day_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SimEv25", &PROBE_N, &[(p_day_1) as u32], false); }
    let r_ = 'f: {
    's1: {
        let sv2: i32 = (c.m.r32((p_day_1).wrapping_add(2920)) as i32);
        match sv2 {
            0i32 => {
                { let v_ = 11i32; let a_ = (p_day_1).wrapping_add(2936); c.m.w32(a_, (v_) as u32); };
                F::SimDay_Cmd(c, 5i32);
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            1i32 => {
                { let v_ = 98i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 23i32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            2i32 => {
                { let v_ = (1i32).wrapping_neg(); let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                F::SimDay_Cmd(c, 25i32);
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            3i32 => {
                { let v_ = (1i32).wrapping_neg(); let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let v_ = 1i32; let a_ = (p_day_1).wrapping_add(2936); c.m.w32(a_, (v_) as u32); };
                { let v_ = 1i32; let a_ = (p_day_1).wrapping_add(2940); c.m.w32(a_, (v_) as u32); };
                F::SimDay_Cmd(c, 5i32);
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            4i32 => {
                F::SimDay_Cmd(c, 6i32);
                { let v_ = 146i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            5i32 => {
                { let v_ = 147i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            6i32 => {
                F::SimDay_Cmd(c, 17i32);
                { let v_ = 148i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            7i32 => {
                if ((((((c.m.r32((p_day_1).wrapping_add(2956)) as i32)) != (0i32)) as i32)) != 0) {
                    { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                } else {
                    { let v_ = 13i32; let a_ = (p_day_1).wrapping_add(2920); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                }
                break 's1;
            }
            8i32 => {
                { let v_ = 150i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            9i32 => {
                { let v_ = 151i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            10i32 => {
                { let v_ = 152i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            11i32 => {
                { let v_ = 153i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            12i32 => {
                { let v_ = 157i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                F::SimDay_AddChange(c, 3i32, 500i32);
                F::SimDay_Cmd(c, 26i32);
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                { let v_ = 14i32; let a_ = (p_day_1).wrapping_add(2920); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            13i32 => {
                { let v_ = 149i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            14i32 => {
                { let v_ = (1i32).wrapping_neg(); let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                F::SimDay_Cmd(c, 7i32);
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            15i32 => {
                { let v_ = 99i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let a1_ = (((F::Rand_Range(c, ((3i32) as u32))).wrapping_neg()) as i32); F::SimDay_AddChange(c, 0i32, a1_) };
                { let a1_ = (((F::Rand_Range(c, ((3i32) as u32))).wrapping_neg()) as i32); F::SimDay_AddChange(c, 1i32, a1_) };
                F::SimDay_Cmd(c, 26i32);
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            16i32 => {
                { let v_ = (1i32).wrapping_neg(); let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                break 'f 1i32;
            }
            _ => {
            }
        }
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "SimEv25", &PROBE_N, &[], true); }
    r_
}

pub fn SimEv26(c: &mut Ctx, mut p_day_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SimEv26", &PROBE_N, &[(p_day_1) as u32], false); }
    let r_ = 'f: {
    's1: {
        let sv2: i32 = (c.m.r32((p_day_1).wrapping_add(2920)) as i32);
        match sv2 {
            0i32 => {
                { let v_ = 11i32; let a_ = (p_day_1).wrapping_add(2936); c.m.w32(a_, (v_) as u32); };
                F::SimDay_Cmd(c, 5i32);
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            1i32 => {
                { let v_ = 98i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 23i32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            2i32 => {
                { let v_ = (1i32).wrapping_neg(); let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                F::SimDay_Cmd(c, 25i32);
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            3i32 => {
                { let v_ = (1i32).wrapping_neg(); let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let v_ = 1i32; let a_ = (p_day_1).wrapping_add(2936); c.m.w32(a_, (v_) as u32); };
                { let v_ = 1i32; let a_ = (p_day_1).wrapping_add(2940); c.m.w32(a_, (v_) as u32); };
                F::SimDay_Cmd(c, 5i32);
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            4i32 => {
                F::SimDay_Cmd(c, 6i32);
                { let v_ = 146i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            5i32 => {
                { let v_ = 147i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            6i32 => {
                F::SimDay_Cmd(c, 17i32);
                { let v_ = 148i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            7i32 => {
                if ((((((c.m.r32((p_day_1).wrapping_add(2956)) as i32)) != (0i32)) as i32)) != 0) {
                    { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                } else {
                    { let v_ = 12i32; let a_ = (p_day_1).wrapping_add(2920); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                }
                break 's1;
            }
            8i32 => {
                { let v_ = 150i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            9i32 => {
                { let v_ = 151i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            10i32 => {
                { let v_ = 154i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            11i32 => {
                { let v_ = 157i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let a1_ = (rt::div_i32(F::SimDay_GetStat(c, 2i32, 0i32), 2i32)).wrapping_neg(); F::SimDay_AddChange(c, 2i32, a1_) };
                F::SimDay_AddChange(c, 3i32, 500i32);
                F::SimDay_Cmd(c, 26i32);
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                { let v_ = 13i32; let a_ = (p_day_1).wrapping_add(2920); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            12i32 => {
                { let v_ = 149i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            13i32 => {
                { let v_ = (1i32).wrapping_neg(); let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                F::SimDay_Cmd(c, 7i32);
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            14i32 => {
                { let v_ = 99i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let a1_ = (((F::Rand_Range(c, ((3i32) as u32))).wrapping_neg()) as i32); F::SimDay_AddChange(c, 0i32, a1_) };
                { let a1_ = (((F::Rand_Range(c, ((3i32) as u32))).wrapping_neg()) as i32); F::SimDay_AddChange(c, 1i32, a1_) };
                F::SimDay_Cmd(c, 26i32);
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            15i32 => {
                { let v_ = (1i32).wrapping_neg(); let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                break 'f 1i32;
            }
            _ => {
            }
        }
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "SimEv26", &PROBE_N, &[], true); }
    r_
}

pub fn SimEv27(c: &mut Ctx, mut p_day_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SimEv27", &PROBE_N, &[(p_day_1) as u32], false); }
    let r_ = 'f: {
    's1: {
        let sv2: i32 = (c.m.r32((p_day_1).wrapping_add(2920)) as i32);
        match sv2 {
            0i32 => {
                { let v_ = 11i32; let a_ = (p_day_1).wrapping_add(2936); c.m.w32(a_, (v_) as u32); };
                F::SimDay_Cmd(c, 5i32);
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            1i32 => {
                { let v_ = 98i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 23i32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            2i32 => {
                { let v_ = (1i32).wrapping_neg(); let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                F::SimDay_Cmd(c, 25i32);
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            3i32 => {
                { let v_ = (1i32).wrapping_neg(); let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let v_ = 1i32; let a_ = (p_day_1).wrapping_add(2936); c.m.w32(a_, (v_) as u32); };
                { let v_ = 1i32; let a_ = (p_day_1).wrapping_add(2940); c.m.w32(a_, (v_) as u32); };
                F::SimDay_Cmd(c, 5i32);
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            4i32 => {
                F::SimDay_Cmd(c, 6i32);
                { let v_ = 146i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            5i32 => {
                { let v_ = 147i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            6i32 => {
                F::SimDay_Cmd(c, 17i32);
                { let v_ = 148i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            7i32 => {
                if ((((((c.m.r32((p_day_1).wrapping_add(2956)) as i32)) != (0i32)) as i32)) != 0) {
                    { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                } else {
                    { let v_ = 13i32; let a_ = (p_day_1).wrapping_add(2920); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                }
                break 's1;
            }
            8i32 => {
                { let v_ = 150i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            9i32 => {
                { let v_ = 151i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            10i32 => {
                { let v_ = 155i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            11i32 => {
                { let v_ = 156i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                F::SimDay_AddChange(c, 0i32, 15i32);
                F::SimDay_AddChange(c, 1i32, 15i32);
                { let a1_ = (rt::div_i32(F::SimDay_GetStat(c, 2i32, 0i32), 2i32)).wrapping_neg(); F::SimDay_AddChange(c, 2i32, a1_) };
                F::SimDay_AddChange(c, 3i32, 700i32);
                F::SimDay_Cmd(c, 26i32);
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            12i32 => {
                { let v_ = 157i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 16i32; let a_ = (p_day_1).wrapping_add(2920); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            13i32 => {
                { let v_ = 149i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            14i32 => {
                { let v_ = (1i32).wrapping_neg(); let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                F::SimDay_Cmd(c, 7i32);
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            15i32 => {
                { let v_ = 99i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let a1_ = (((F::Rand_Range(c, ((3i32) as u32))).wrapping_neg()) as i32); F::SimDay_AddChange(c, 0i32, a1_) };
                { let a1_ = (((F::Rand_Range(c, ((3i32) as u32))).wrapping_neg()) as i32); F::SimDay_AddChange(c, 1i32, a1_) };
                F::SimDay_Cmd(c, 26i32);
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            16i32 => {
                { let v_ = (1i32).wrapping_neg(); let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                break 'f 1i32;
            }
            _ => {
            }
        }
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "SimEv27", &PROBE_N, &[], true); }
    r_
}
