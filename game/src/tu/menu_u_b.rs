#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn menu_u_b__SimRock_Test(c: &mut Ctx, mut p_flags_1: i32, mut p_bit_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "menu_u_b__SimRock_Test", &PROBE_N, &[(p_flags_1) as u32, (p_bit_2) as u32], false); }
    let r_ = 'f: {
    break 'f ((p_flags_1) & (p_bit_2));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "menu_u_b__SimRock_Test", &PROBE_N, &[], true); }
    r_
}

pub fn SimEv32(c: &mut Ctx, mut p_day_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SimEv32", &PROBE_N, &[(p_day_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_done_2: i32 = 0;
    let mut l_ok_3: i32 = 0;
    let mut l_flash_4: u32 = 0;
    let r_ = 'f: {
        l_done_2 = 0i32;
        l_ok_3 = ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) as i32);
        's1: {
            let sv5: i32 = (c.m.r32((p_day_1).wrapping_add(2920)) as i32);
            match sv5 {
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
                    { let v_ = 2i32; let a_ = (p_day_1).wrapping_add(2936); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2940); c.m.w32(a_, (v_) as u32); };
                    F::SimDay_Cmd(c, 5i32);
                    { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                    break 's1;
                }
                4i32 => {
                    F::SimDay_Cmd(c, 6i32);
                    { let v_ = 184i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                    { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                    break 's1;
                }
                5i32 => {
                    { let v_ = 185i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                    { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                    break 's1;
                }
                6i32 => {
                    { let v_ = 186i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                    F::SimDay_Cmd(c, 17i32);
                    { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                    break 's1;
                }
                7i32 => {
                    { let v_ = (1i32).wrapping_neg(); let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                    if ((((((c.m.r32((p_day_1).wrapping_add(2956)) as i32)) == (0i32)) as i32)) != 0) {
                        { let v_ = 19i32; let a_ = (p_day_1).wrapping_add(2920); c.m.w32(a_, (v_) as u32); };
                        { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                    } else {
                        { let v_ = (c.m.r32(((p_day_1).wrapping_add(2944)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32); let a_ = sym::gSimRockRank; c.m.w32(a_, (v_) as u32); };
                        { let v_ = (({ let a_ = ((sym::gSimTrain1).wrapping_add(((((c.m.r32(sym::gSimRockRank) as i32)) as u32)).wrapping_mul(24))).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(3i32)).wrapping_add(3i32); let a_ = sym::gSimRockSpeed; c.m.w32(a_, (v_) as u32); };
                        { let v_ = { let a_ = (sym::gSimTrain1).wrapping_add(((((c.m.r32(sym::gSimRockRank) as i32)) as u32)).wrapping_mul(24)); (c.m.r32(a_) as i32) }; let a_ = sym::gSimRockLeft; c.m.w32(a_, (v_) as u32); };
                        { let v_ = 0i32; let a_ = sym::gSimRockHits; c.m.w32(a_, (v_) as u32); };
                        { let v_ = 0i32; let a_ = sym::gSimRockX; c.m.w32(a_, (v_) as u32); };
                        { let v_ = ((0i32) as u8); let a_ = sym::gSimRockFlags; c.m.w8(a_, v_); };
                        { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                    }
                    break 's1;
                }
                8i32 => {
                    { let v_ = 188i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                    { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                    break 's1;
                }
                9i32 => {
                    F::StreamSe_PlayDefault(c, 0i32, 68577i32);
                    F::Bgm_Stop(c);
                    { let v_ = 189i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                    F::SimDay_Cmd(c, 55i32);
                    { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                    break 's1;
                }
                10i32 => {
                    if ((((c.m.r32((((p_day_1).wrapping_add(32)).wrapping_add((((4i32) as u32)).wrapping_mul(44))).wrapping_add(12))) & (((1i32) as u32)))) != 0) {
                        { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                    }
                    break 's1;
                }
                11i32 => {
                    if ((((((((!(({ let a0_ = ((c.m.r8(sym::gSimRockFlags)) as i32); F::menu_u_b__SimRock_Test(c, a0_, 1i32) }) != 0)) as i32)) != 0) && ((l_ok_3) != 0)) as i32)) != 0) {
                        F::SimDay_Cmd(c, 56i32);
                        { let r_ = 1i32; let a_ = sym::gSimRockFlags; let n_ = ((((((c.m.r8(a_)) as i32)) | (r_))) as u8); c.m.w8(a_, n_); };
                        { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 33i32); };
                    }
                    if ((((c.m.r32((((p_day_1).wrapping_add(32)).wrapping_add((((4i32) as u32)).wrapping_mul(44))).wrapping_add(12))) & (((32i32) as u32)))) != 0) {
                        { let r_ = (!(1i32)); let a_ = sym::gSimRockFlags; let n_ = ((((((c.m.r8(a_)) as i32)) & (r_))) as u8); c.m.w8(a_, n_); };
                    }
                    if ((((c.m.r32((((p_day_1).wrapping_add(32)).wrapping_add((((4i32) as u32)).wrapping_mul(44))).wrapping_add(12))) & (((16i32) as u32)))) != 0) {
                        if ((((((((((c.m.r32(sym::gSimRockX) as i32)) >= ((412i32).wrapping_neg())) as i32)) != 0) && ((((((c.m.r32(sym::gSimRockX) as i32)) <= ((342i32).wrapping_neg())) as i32)) != 0)) as i32)) != 0) {
                            F::SimDay_Cmd(c, 57i32);
                            { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                            { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                            { let r_ = (!(1i32)); let a_ = sym::gSimRockFlags; let n_ = ((((((c.m.r8(a_)) as i32)) & (r_))) as u8); c.m.w8(a_, n_); };
                            { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 34i32); };
                            { let a_ = sym::gSimRockHits; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        }
                    }
                    { let r_ = (c.m.r32(sym::gSimRockSpeed) as i32); let a_ = sym::gSimRockX; let n_ = ((c.m.r32(a_) as i32)).wrapping_sub(r_); c.m.w32(a_, (n_) as u32); };
                    if ((((((c.m.r32(sym::gSimRockX) as i32)) < ((700i32).wrapping_neg())) as i32)) != 0) {
                        if ((((((((((c.m.r32(sym::gSimRockLeft) as i32)) == (0i32)) as i32)) != 0) || (({ { let v_ = 0i32; let a_ = sym::gSimRockX; c.m.w32(a_, (v_) as u32); }; ((({ let a_ = sym::gSimRockLeft; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) == (0i32)) as i32) }) != 0)) as i32)) != 0) {
                            { let v_ = 13i32; let a_ = (p_day_1).wrapping_add(2920); c.m.w32(a_, (v_) as u32); };
                            { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                        }
                    }
                    break 's1;
                }
                12i32 => {
                    if ((((c.m.r32((((p_day_1).wrapping_add(32)).wrapping_add((((4i32) as u32)).wrapping_mul(44))).wrapping_add(12))) & (((4i32) as u32)))) != 0) {
                        { let v_ = 0i32; let a_ = sym::gSimRockX; c.m.w32(a_, (v_) as u32); };
                        { let a_ = sym::gSimRockLeft; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                        if ((((((c.m.r32(sym::gSimRockLeft) as i32)) == (0i32)) as i32)) != 0) {
                            if ((((((c.m.r32(sym::gSimRockHits) as i32)) == ({ let a_ = (sym::gSimTrain1).wrapping_add(((((c.m.r32(sym::gSimRockRank) as i32)) as u32)).wrapping_mul(24)); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                                { let r_ = 2i32; let a_ = sym::gSimRockFlags; let n_ = ((((((c.m.r8(a_)) as i32)) | (r_))) as u8); c.m.w8(a_, n_); };
                                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                            }
                            { let v_ = 13i32; let a_ = (p_day_1).wrapping_add(2920); c.m.w32(a_, (v_) as u32); };
                            { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                        } else {
                            F::SimDay_Cmd(c, 58i32);
                            { let v_ = 11i32; let a_ = (p_day_1).wrapping_add(2920); c.m.w32(a_, (v_) as u32); };
                            { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                        }
                    }
                    break 's1;
                }
                13i32 => {
                    if (({ let a0_ = ((c.m.r8(sym::gSimRockFlags)) as i32); F::menu_u_b__SimRock_Test(c, a0_, 2i32) }) != 0) {
                        F::SimDay_Cmd(c, 59i32);
                        { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 31i32); };
                    } else {
                        F::SimDay_Cmd(c, 60i32);
                        { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 32i32); };
                    }
                    { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                    break 's1;
                }
                14i32 => {
                    if ((((c.m.r32((((p_day_1).wrapping_add(32)).wrapping_add((((4i32) as u32)).wrapping_mul(44))).wrapping_add(12))) & (((2i32) as u32)))) != 0) {
                        F::SimDay_Cmd(c, 61i32);
                        { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                    }
                    break 's1;
                }
                15i32 => {
                    if ((((c.m.r32((((p_day_1).wrapping_add(32)).wrapping_add((((4i32) as u32)).wrapping_mul(44))).wrapping_add(12))) & (((8i32) as u32)))) != 0) {
                        F::StreamSe_Stop(c, 0i32);
                        F::SimDay_PlayBgm(c);
                        F::SimDay_Cmd(c, 62i32);
                        if (({ let a0_ = ((c.m.r8(sym::gSimRockFlags)) as i32); F::menu_u_b__SimRock_Test(c, a0_, 2i32) }) != 0) {
                            { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                            { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                        } else {
                            { let v_ = 17i32; let a_ = (p_day_1).wrapping_add(2920); c.m.w32(a_, (v_) as u32); };
                            { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                        }
                    }
                    break 's1;
                }
                16i32 => {
                    { let v_ = 190i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                    { let a1_ = { let a_ = ((sym::gSimTrain1).wrapping_add(((((c.m.r32(sym::gSimRockRank) as i32)) as u32)).wrapping_mul(24))).wrapping_add(12); (c.m.r32(a_) as i32) }; F::SimDay_AddChange(c, 0i32, a1_) };
                    { let a1_ = { let a_ = ((sym::gSimTrain1).wrapping_add(((((c.m.r32(sym::gSimRockRank) as i32)) as u32)).wrapping_mul(24))).wrapping_add(8); (c.m.r32(a_) as i32) }; F::SimDay_AddChange(c, 3i32, a1_) };
                    F::SimDay_Cmd(c, 26i32);
                    { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 18i32; let a_ = (p_day_1).wrapping_add(2920); c.m.w32(a_, (v_) as u32); };
                    break 's1;
                }
                17i32 => {
                    { let v_ = 192i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                    { let a1_ = { let a_ = ((sym::gSimTrain1).wrapping_add(((((c.m.r32(sym::gSimRockRank) as i32)) as u32)).wrapping_mul(24))).wrapping_add(16); (c.m.r32(a_) as i32) }; F::SimDay_AddChange(c, 0i32, a1_) };
                    F::SimDay_Cmd(c, 26i32);
                    { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                    { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    break 's1;
                }
                18i32 => {
                    { let v_ = 191i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                    { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    { let v_ = 22i32; let a_ = (p_day_1).wrapping_add(2920); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                    break 's1;
                }
                19i32 => {
                    { let v_ = 187i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                    { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                    break 's1;
                }
                20i32 => {
                    { let v_ = (1i32).wrapping_neg(); let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                    F::SimDay_Cmd(c, 7i32);
                    { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                    break 's1;
                }
                21i32 => {
                    { let v_ = 99i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                    { let a1_ = (((((F::Rand_Range(c, ((((3i32) as i64 as u64)) as u32))) as u64)) as i32)).wrapping_neg(); F::SimDay_AddChange(c, 0i32, a1_) };
                    { let a1_ = (((((F::Rand_Range(c, ((((3i32) as i64 as u64)) as u32))) as u64)) as i32)).wrapping_neg(); F::SimDay_AddChange(c, 1i32, a1_) };
                    F::SimDay_Cmd(c, 26i32);
                    { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                    { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    break 's1;
                }
                22i32 => {
                    { let v_ = (1i32).wrapping_neg(); let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                    l_done_2 = 1i32;
                    break 's1;
                }
                _ => {
                }
            }
        }
        l_flash_4 = ((p_day_1).wrapping_add(32)).wrapping_add((((4i32) as u32)).wrapping_mul(44));
        F::Flash_FindLabel(c, l_flash_4, ((0i32) as u32), sym::menu_u_b__str0, fp);
        { let a2_ = (c.m.r32(sym::gSimRockX) as i32); F::Flash_ClipSetOffset(c, l_flash_4, fp, a2_, 0i32) };
        { let v_ = 512i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 128i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = 256i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        F::Flash_FindLabel(c, l_flash_4, ((0i32) as u32), sym::menu_u_b__str1, fp);
        F::Flash_ClipSetUv(c, l_flash_4, fp, fp.wrapping_add(8));
        { let v_ = 384i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 256i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = 128i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        F::Flash_FindLabel(c, l_flash_4, ((0i32) as u32), sym::menu_u_b__str2, fp);
        F::Flash_ClipSetUv(c, l_flash_4, fp, fp.wrapping_add(8));
        { let v_ = 256i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 128i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = 128i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        F::Flash_FindLabel(c, l_flash_4, ((0i32) as u32), sym::menu_u_b__str3, fp);
        F::Flash_ClipSetUv(c, l_flash_4, fp, fp.wrapping_add(8));
        if ((((c.m.r32((l_flash_4).wrapping_add(16))) & (((1i32) as u32)))) != 0) {
            { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 59i32); };
        }
        if ((((c.m.r32((l_flash_4).wrapping_add(16))) & (((2i32) as u32)))) != 0) {
            { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 60i32); };
        }
        break 'f l_done_2;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "SimEv32", &PROBE_N, &[], true); }
    r_
}
