#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn SimEv28_SelectCard(c: &mut Ctx, mut p_day_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SimEv28_SelectCard", &PROBE_N, &[(p_day_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(80) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 80);
    let mut l_flash_2: u32 = 0;
    let r_ = 'f: {
        l_flash_2 = ((p_day_1).wrapping_add(32)).wrapping_add((((2i32) as u32)).wrapping_mul(44));
        { let _ = { let a2_ = (c.m.r32(((p_day_1).wrapping_add(1368)).wrapping_add((((3i32) as u32)).wrapping_mul(4))) as i32); F::sprintf(c, fp.wrapping_add(8), sym::menu_t_b__str0, &[((a2_) as i64 as u64)]) }; };
        F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), fp.wrapping_add(8), fp);
        F::Flash_ClipGotoLabel(c, l_flash_2, fp, sym::menu_t_b__str1);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "SimEv28_SelectCard", &PROBE_N, &[], true); }
    r_
}

pub fn SimEv28_UnselectCard(c: &mut Ctx, mut p_day_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SimEv28_UnselectCard", &PROBE_N, &[(p_day_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(80) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 80);
    let mut l_flash_2: u32 = 0;
    let r_ = 'f: {
        l_flash_2 = ((p_day_1).wrapping_add(32)).wrapping_add((((2i32) as u32)).wrapping_mul(44));
        { let _ = { let a2_ = (c.m.r32(((p_day_1).wrapping_add(1368)).wrapping_add((((3i32) as u32)).wrapping_mul(4))) as i32); F::sprintf(c, fp.wrapping_add(8), sym::menu_t_b__str2, &[((a2_) as i64 as u64)]) }; };
        F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), fp.wrapping_add(8), fp);
        F::Flash_ClipGotoLabel(c, l_flash_2, fp, sym::menu_t_b__str3);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "SimEv28_UnselectCard", &PROBE_N, &[], true); }
    r_
}

pub fn SimEv28_MoveCursor(c: &mut Ctx, mut p_day_1: u32, mut p_dir_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SimEv28_MoveCursor", &PROBE_N, &[(p_day_1) as u32, (p_dir_2) as u32], false); }
    let mut l_n_3: i32 = 0;
    let mut l_cur_4: i32 = 0;
    let mut l_i_5: i32 = 0;
    let r_ = 'f: {
    l_n_3 = ({ let a_ = (sym::gSimTrain0).wrapping_add(((((c.m.r32(sym::gSimCardRank) as i32)) as u32)).wrapping_mul(24)); (c.m.r32(a_) as i32) }).wrapping_sub(1i32);
    l_cur_4 = { let a_ = ((p_day_1).wrapping_add(1368)).wrapping_add(((((c.m.r32((p_day_1).wrapping_add(1424)) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
    {
        l_i_5 = 0i32;
        'l1: loop {
            if !(((((l_i_5) < (l_n_3)) as i32)) != 0) { break 'l1; }
            'c2: {
                if (((((p_dir_2) != (0i32)) as i32)) != 0) {
                    { let o_ = l_cur_4; let n_ = (o_).wrapping_add(1); l_cur_4 = n_; };
                    if (((((l_cur_4) > (l_n_3)) as i32)) != 0) {
                        l_cur_4 = 0i32;
                    }
                } else {
                    { let o_ = l_cur_4; let n_ = (o_).wrapping_sub(1); l_cur_4 = n_; };
                    if (((((l_cur_4) < (0i32)) as i32)) != 0) {
                        l_cur_4 = l_n_3;
                    }
                }
                if ((((!(((((((c.m.r8(sym::gSimCardPicked)) as i32)).wrapping_shr((l_cur_4) as u32)) & (1i32))) != 0)) as i32)) != 0) {
                    break 'f l_cur_4;
                }
            }
            { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
        }
    }
    break 'f (1i32).wrapping_neg();
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "SimEv28_MoveCursor", &PROBE_N, &[], true); }
    r_
}

pub fn SimEv28_Deal(c: &mut Ctx, mut p_n_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SimEv28_Deal", &PROBE_N, &[(p_n_1) as u32], false); }
    let mut l_i_2: i32 = 0;
    let mut l_j_3: i32 = 0;
    let mut l_r_4: i32 = 0;
    let mut l_t_5: i32 = 0;
    let r_ = 'f: {
    {
        l_i_2 = 9i32;
        'l1: loop {
            if !(((((l_i_2) >= (0i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let v_ = l_i_2; let a_ = (sym::gSimCardOrder).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_sub(1); l_i_2 = n_; };
        }
    }
    {
        l_i_2 = 0i32;
        'l3: loop {
            if !(((((l_i_2) < (10i32)) as i32)) != 0) { break 'l3; }
            'c4: {
                l_r_4 = rt::rem_i32(F::Port_Rand(c), 10i32);
                l_t_5 = (c.m.r32((sym::gSimCardOrder).wrapping_add((((l_i_2) as u32)).wrapping_mul(4))) as i32);
                { let v_ = (c.m.r32((sym::gSimCardOrder).wrapping_add((((l_r_4) as u32)).wrapping_mul(4))) as i32); let a_ = (sym::gSimCardOrder).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = l_t_5; let a_ = (sym::gSimCardOrder).wrapping_add((((l_r_4) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
    { let _ = F::memcpy(c, sym::gSimCardShown, sym::gSimCardOrder, (((p_n_1).wrapping_mul(4i32)) as u32)); };
    {
        l_i_2 = 0i32;
        'l5: loop {
            if !(((((l_i_2) < (p_n_1)) as i32)) != 0) { break 'l5; }
            'c6: {
                {
                    l_j_3 = (p_n_1).wrapping_sub(1i32);
                    'l7: loop {
                        if !(((((l_j_3) > (l_i_2)) as i32)) != 0) { break 'l7; }
                        'c8: {
                            if ((((((c.m.r32((sym::gSimCardOrder).wrapping_add((((l_j_3) as u32)).wrapping_mul(4))) as i32)) < ((c.m.r32((sym::gSimCardOrder).wrapping_add(((((l_j_3).wrapping_sub(1i32)) as u32)).wrapping_mul(4))) as i32))) as i32)) != 0) {
                                l_t_5 = (c.m.r32((sym::gSimCardOrder).wrapping_add((((l_j_3) as u32)).wrapping_mul(4))) as i32);
                                { let v_ = (c.m.r32((sym::gSimCardOrder).wrapping_add(((((l_j_3).wrapping_sub(1i32)) as u32)).wrapping_mul(4))) as i32); let a_ = (sym::gSimCardOrder).wrapping_add((((l_j_3) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                { let v_ = l_t_5; let a_ = (sym::gSimCardOrder).wrapping_add(((((l_j_3).wrapping_sub(1i32)) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                            }
                        }
                        { let o_ = l_j_3; let n_ = (o_).wrapping_sub(1); l_j_3 = n_; };
                    }
                }
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
    {
        l_i_2 = 0i32;
        'l9: loop {
            if !(((((l_i_2) < (p_n_1)) as i32)) != 0) { break 'l9; }
            'c10: {
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
    { let v_ = 0i32; let a_ = sym::gSimCardNext; c.m.w32(a_, (v_) as u32); };
    { let v_ = ((0i32) as u8); let a_ = sym::gSimCardPicked; c.m.w8(a_, v_); };
    { let v_ = ((0i32) as u8); let a_ = sym::gSimCardWrong; c.m.w8(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "SimEv28_Deal", &PROBE_N, &[], true); }
    r_
}

pub fn SimEv28(c: &mut Ctx, mut p_day_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SimEv28", &PROBE_N, &[(p_day_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(96) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 96);
    let mut l_flash_2: u32 = 0;
    let mut l_i_3: i32 = 0;
    let mut l_cur_4: i32 = 0;
    let mut l_ok_5: i32 = 0;
    let mut l_left_6: i32 = 0;
    let mut l_right_7: i32 = 0;
    let mut l_place_8: i32 = 0;
    let mut l_x_9: i32 = 0;
    let mut l_y_10: i32 = 0;
    let r_ = 'f: {
        l_ok_5 = ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) as i32);
        l_left_6 = ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((1i32) as u32)))) as i32);
        l_right_7 = ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((2i32) as u32)))) as i32);
        's1: {
            let sv11: i32 = (c.m.r32((p_day_1).wrapping_add(2920)) as i32);
            match sv11 {
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
                    { let v_ = 158i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                    F::SimDay_Cmd(c, 17i32);
                    { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                    break 's1;
                }
                6i32 => {
                    { let v_ = (1i32).wrapping_neg(); let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                    if ((((((c.m.r32((p_day_1).wrapping_add(2956)) as i32)) == (0i32)) as i32)) != 0) {
                        { let v_ = 23i32; let a_ = (p_day_1).wrapping_add(2920); c.m.w32(a_, (v_) as u32); };
                        { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                    } else {
                        { let v_ = (c.m.r32(((p_day_1).wrapping_add(2944)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32); let a_ = sym::gSimCardRank; c.m.w32(a_, (v_) as u32); };
                        { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                        { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    }
                    break 's1;
                }
                7i32 => {
                    { let v_ = 160i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                    { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                    break 's1;
                }
                8i32 => {
                    F::StreamSe_PlayDefault(c, 0i32, 68577i32);
                    F::Bgm_Stop(c);
                    { let a0_ = { let a_ = (sym::gSimTrain0).wrapping_add(((((c.m.r32(sym::gSimCardRank) as i32)) as u32)).wrapping_mul(24)); (c.m.r32(a_) as i32) }; F::SimEv28_Deal(c, a0_) };
                    { let v_ = 161i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                    F::SimDay_Cmd(c, 36i32);
                    { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                    break 's1;
                }
                9i32 => {
                    if ((((c.m.r32((((p_day_1).wrapping_add(32)).wrapping_add((((2i32) as u32)).wrapping_mul(44))).wrapping_add(12))) & (((1i32) as u32)))) != 0) {
                        { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                    }
                    break 's1;
                }
                10i32 => {
                    F::SimDay_Cmd(c, 37i32);
                    { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 4i32); };
                    { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                    break 's1;
                }
                11i32 => {
                    if ((((c.m.r32((((p_day_1).wrapping_add(32)).wrapping_add((((2i32) as u32)).wrapping_mul(44))).wrapping_add(12))) & (((8i32) as u32)))) != 0) {
                        { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                    }
                    break 's1;
                }
                12i32 => {
                    if ((((({ let a_ = (p_day_1).wrapping_add(2924); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); n_ }) > (({ let a_ = ((sym::gSimTrain0).wrapping_add(((((c.m.r32(sym::gSimCardRank) as i32)) as u32)).wrapping_mul(24))).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(60i32))) as i32)) != 0) {
                        { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                    }
                    break 's1;
                }
                13i32 => {
                    F::SimDay_Cmd(c, 38i32);
                    { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 4i32); };
                    { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                    break 's1;
                }
                14i32 => {
                    if ((((c.m.r32((((p_day_1).wrapping_add(32)).wrapping_add((((2i32) as u32)).wrapping_mul(44))).wrapping_add(12))) & (((16i32) as u32)))) != 0) {
                        F::SimEv28_SelectCard(c, p_day_1);
                        { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                    }
                    break 's1;
                }
                15i32 => {
                    if ((((((((((c.m.r32(sym::gSimCardNext) as i32)) == ({ let a_ = (sym::gSimTrain0).wrapping_add(((((c.m.r32(sym::gSimCardRank) as i32)) as u32)).wrapping_mul(24)); (c.m.r32(a_) as i32) })) as i32)) != 0) || ((((((c.m.r32(sym::gSimCardNext) as i32)) == ((1i32).wrapping_neg())) as i32)) != 0)) as i32)) != 0) {
                        if ((((({ let a_ = (p_day_1).wrapping_add(2924); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); n_ }) > (60i32)) as i32)) != 0) {
                            { let v_ = 17i32; let a_ = (p_day_1).wrapping_add(2920); c.m.w32(a_, (v_) as u32); };
                            { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                        }
                    } else {
                        if ((l_ok_5) != 0) {
                            { let r_ = (1i32).wrapping_shl(({ let a_ = ((p_day_1).wrapping_add(1368)).wrapping_add(((((c.m.r32((p_day_1).wrapping_add(1424)) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32); let a_ = sym::gSimCardPicked; let n_ = ((((((c.m.r8(a_)) as i32)) | (r_))) as u8); c.m.w8(a_, n_); };
                            F::SimDay_Cmd(c, 39i32);
                            { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 4i32); };
                            if ((((({ let a_ = (sym::gSimCardShown).wrapping_add(((({ let a_ = ((p_day_1).wrapping_add(1368)).wrapping_add(((((c.m.r32((p_day_1).wrapping_add(1424)) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) == ({ let a_ = (sym::gSimCardOrder).wrapping_add(((((c.m.r32(sym::gSimCardNext) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                                { let a_ = sym::gSimCardNext; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                            } else {
                                { let v_ = (1i32).wrapping_neg(); let a_ = sym::gSimCardNext; c.m.w32(a_, (v_) as u32); };
                                { let r_ = (1i32).wrapping_shl(({ let a_ = ((p_day_1).wrapping_add(1368)).wrapping_add(((((c.m.r32((p_day_1).wrapping_add(1424)) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32); let a_ = sym::gSimCardWrong; let n_ = ((((((c.m.r8(a_)) as i32)) | (r_))) as u8); c.m.w8(a_, n_); };
                            }
                            { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                            { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                        } else {
                            if ((l_left_6) != 0) {
                                l_cur_4 = F::SimEv28_MoveCursor(c, p_day_1, 0i32);
                                if (((((l_cur_4) >= (0i32)) as i32)) != 0) {
                                    F::SimEv28_UnselectCard(c, p_day_1);
                                    { let v_ = l_cur_4; let a_ = ((p_day_1).wrapping_add(1368)).wrapping_add(((((c.m.r32((p_day_1).wrapping_add(1424)) as i32)) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                    F::SimEv28_SelectCard(c, p_day_1);
                                }
                            } else {
                                if ((l_right_7) != 0) {
                                    l_cur_4 = F::SimEv28_MoveCursor(c, p_day_1, 1i32);
                                    if (((((l_cur_4) >= (0i32)) as i32)) != 0) {
                                        F::SimEv28_UnselectCard(c, p_day_1);
                                        { let v_ = l_cur_4; let a_ = ((p_day_1).wrapping_add(1368)).wrapping_add(((((c.m.r32((p_day_1).wrapping_add(1424)) as i32)) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                        F::SimEv28_SelectCard(c, p_day_1);
                                    }
                                }
                            }
                        }
                    }
                    break 's1;
                }
                16i32 => {
                    if ((((c.m.r32((((p_day_1).wrapping_add(32)).wrapping_add((((2i32) as u32)).wrapping_mul(44))).wrapping_add(12))) & (((4i32) as u32)))) != 0) {
                        if ((((((c.m.r32(sym::gSimCardNext) as i32)) != ((1i32).wrapping_neg())) as i32)) != 0) {
                            l_place_8 = F::SimEv28_MoveCursor(c, p_day_1, 1i32);
                            if (((((l_place_8) >= (0i32)) as i32)) != 0) {
                                { let v_ = l_place_8; let a_ = ((p_day_1).wrapping_add(1368)).wrapping_add(((((c.m.r32((p_day_1).wrapping_add(1424)) as i32)) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                F::SimEv28_SelectCard(c, p_day_1);
                            }
                        }
                        { let v_ = 15i32; let a_ = (p_day_1).wrapping_add(2920); c.m.w32(a_, (v_) as u32); };
                        { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                    }
                    break 's1;
                }
                17i32 => {
                    if (((((((c.m.r8(sym::gSimCardWrong)) as i32)) != (0i32)) as i32)) != 0) {
                        F::SimDay_Cmd(c, 41i32);
                        { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 32i32); };
                    } else {
                        F::SimDay_Cmd(c, 40i32);
                        { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 31i32); };
                    }
                    { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                    break 's1;
                }
                18i32 => {
                    if ((((c.m.r32((((p_day_1).wrapping_add(32)).wrapping_add((((2i32) as u32)).wrapping_mul(44))).wrapping_add(12))) & (((2i32) as u32)))) != 0) {
                        F::StreamSe_Stop(c, 0i32);
                        F::SimDay_PlayBgm(c);
                        F::SimDay_Cmd(c, 42i32);
                        { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                    }
                    break 's1;
                }
                19i32 => {
                    if ((((c.m.r32((((p_day_1).wrapping_add(32)).wrapping_add((((2i32) as u32)).wrapping_mul(44))).wrapping_add(12))) & (((32i32) as u32)))) != 0) {
                        F::SimDay_Cmd(c, 43i32);
                        if (((((((c.m.r8(sym::gSimCardWrong)) as i32)) != (0i32)) as i32)) != 0) {
                            { let v_ = 21i32; let a_ = (p_day_1).wrapping_add(2920); c.m.w32(a_, (v_) as u32); };
                            { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                        } else {
                            { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                            { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                        }
                    }
                    break 's1;
                }
                20i32 => {
                    { let v_ = 162i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                    { let a1_ = { let a_ = ((sym::gSimTrain0).wrapping_add(((((c.m.r32(sym::gSimCardRank) as i32)) as u32)).wrapping_mul(24))).wrapping_add(8); (c.m.r32(a_) as i32) }; F::SimDay_AddChange(c, 3i32, a1_) };
                    { let a1_ = { let a_ = ((sym::gSimTrain0).wrapping_add(((((c.m.r32(sym::gSimCardRank) as i32)) as u32)).wrapping_mul(24))).wrapping_add(12); (c.m.r32(a_) as i32) }; F::SimDay_AddChange(c, 0i32, a1_) };
                    F::SimDay_Cmd(c, 26i32);
                    { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 22i32; let a_ = (p_day_1).wrapping_add(2920); c.m.w32(a_, (v_) as u32); };
                    break 's1;
                }
                21i32 => {
                    { let v_ = 163i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                    { let a1_ = { let a_ = ((sym::gSimTrain0).wrapping_add(((((c.m.r32(sym::gSimCardRank) as i32)) as u32)).wrapping_mul(24))).wrapping_add(16); (c.m.r32(a_) as i32) }; F::SimDay_AddChange(c, 0i32, a1_) };
                    F::SimDay_Cmd(c, 26i32);
                    { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                    { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    break 's1;
                }
                22i32 => {
                    { let v_ = 157i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                    { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    { let v_ = 26i32; let a_ = (p_day_1).wrapping_add(2920); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                    break 's1;
                }
                23i32 => {
                    { let v_ = 159i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                    { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                    break 's1;
                }
                24i32 => {
                    { let v_ = (1i32).wrapping_neg(); let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                    F::SimDay_Cmd(c, 7i32);
                    { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                    break 's1;
                }
                25i32 => {
                    { let v_ = 99i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                    { let a1_ = (((F::Rand_Range(c, ((3i32) as u32))).wrapping_neg()) as i32); F::SimDay_AddChange(c, 0i32, a1_) };
                    { let a1_ = (((F::Rand_Range(c, ((3i32) as u32))).wrapping_neg()) as i32); F::SimDay_AddChange(c, 1i32, a1_) };
                    F::SimDay_Cmd(c, 26i32);
                    { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                    { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    break 's1;
                }
                26i32 => {
                    { let v_ = (1i32).wrapping_neg(); let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                    break 'f 1i32;
                }
                _ => {
                }
            }
        }
        l_flash_2 = ((p_day_1).wrapping_add(32)).wrapping_add((((2i32) as u32)).wrapping_mul(44));
        {
            l_i_3 = 0i32;
            'l2: loop {
                if !(((((l_i_3) < ({ let a_ = (sym::gSimTrain0).wrapping_add(((((c.m.r32(sym::gSimCardRank) as i32)) as u32)).wrapping_mul(24)); (c.m.r32(a_) as i32) })) as i32)) != 0) { break 'l2; }
                'c3: {
                    { let _ = F::sprintf(c, fp, sym::menu_t_b__str4, &[((l_i_3) as i64 as u64)]); };
                    l_x_9 = rt::rem_i32((c.m.r32((sym::gSimCardShown).wrapping_add((((l_i_3) as u32)).wrapping_mul(4))) as i32), 4i32);
                    l_y_10 = rt::div_i32((c.m.r32((sym::gSimCardShown).wrapping_add((((l_i_3) as u32)).wrapping_mul(4))) as i32), 4i32);
                    { let r_ = 64i32; let n_ = (l_x_9).wrapping_mul(r_); l_x_9 = n_; };
                    { let r_ = 64i32; let n_ = (l_y_10).wrapping_mul(r_); l_y_10 = n_; };
                    { let v_ = l_x_9; let a_ = fp.wrapping_add(72); c.m.w32(a_, (v_) as u32); };
                    { let v_ = l_y_10; let a_ = (fp.wrapping_add(72)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    { let v_ = (l_x_9).wrapping_add(64i32); let a_ = (fp.wrapping_add(72)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = (l_y_10).wrapping_add(64i32); let a_ = (fp.wrapping_add(72)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                    F::Flash_FindLabel(c, l_flash_2, fp, sym::menu_t_b__str5, fp.wrapping_add(64));
                    F::Flash_ClipSetUv(c, l_flash_2, fp.wrapping_add(64), fp.wrapping_add(72));
                    if (((((((c.m.r8(sym::gSimCardWrong)) as i32)).wrapping_shr((l_i_3) as u32)) & (1i32))) != 0) {
                        { let v_ = 64i32; let a_ = fp.wrapping_add(72); c.m.w32(a_, (v_) as u32); };
                        { let v_ = 0i32; let a_ = (fp.wrapping_add(72)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                        { let v_ = 128i32; let a_ = (fp.wrapping_add(72)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                        { let v_ = 64i32; let a_ = (fp.wrapping_add(72)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                        F::Flash_FindLabel(c, l_flash_2, fp, sym::menu_t_b__str6, fp.wrapping_add(64));
                        F::Flash_ClipSetUv(c, l_flash_2, fp.wrapping_add(64), fp.wrapping_add(72));
                    }
                }
                { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
            }
        }
        { let v_ = 0i32; let a_ = fp.wrapping_add(72); c.m.w32(a_, (v_) as u32); };
        { let v_ = 128i32; let a_ = (fp.wrapping_add(72)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = 512i32; let a_ = (fp.wrapping_add(72)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 256i32; let a_ = (fp.wrapping_add(72)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), sym::menu_t_b__str7, fp.wrapping_add(64));
        F::Flash_ClipSetUv(c, l_flash_2, fp.wrapping_add(64), fp.wrapping_add(72));
        if ((((c.m.r32((l_flash_2).wrapping_add(16))) & (((1i32) as u32)))) != 0) {
            { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 60i32); };
        }
        break 'f 0i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "SimEv28", &PROBE_N, &[], true); }
    r_
}

pub fn SimEv29(c: &mut Ctx, mut p_day_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SimEv29", &PROBE_N, &[(p_day_1) as u32], false); }
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
                { let v_ = 8i32; let a_ = (p_day_1).wrapping_add(2936); c.m.w32(a_, (v_) as u32); };
                { let v_ = 9i32; let a_ = (p_day_1).wrapping_add(2940); c.m.w32(a_, (v_) as u32); };
                F::SimDay_Cmd(c, 5i32);
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            4i32 => {
                F::SimDay_Cmd(c, 6i32);
                { let v_ = 164i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            5i32 => {
                { let v_ = 165i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            6i32 => {
                { let v_ = 166i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            7i32 => {
                { let v_ = 167i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                F::SimDay_Cmd(c, 17i32);
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            8i32 => {
                { let v_ = (1i32).wrapping_neg(); let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                if ((((((c.m.r32((p_day_1).wrapping_add(2956)) as i32)) != (0i32)) as i32)) != 0) {
                    { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                } else {
                    { let v_ = 18i32; let a_ = (p_day_1).wrapping_add(2920); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                }
                break 's1;
            }
            9i32 => {
                { let v_ = 169i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            10i32 => {
                { let v_ = 170i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            11i32 => {
                { let v_ = 171i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            12i32 => {
                { let v_ = 152i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            13i32 => {
                { let v_ = 152i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            14i32 => {
                { let v_ = 172i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            15i32 => {
                { let v_ = 173i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = ((512i32) | (1024i32)); let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                F::SimDay_AddChange(c, 0i32, 15i32);
                F::SimDay_AddChange(c, 1i32, 15i32);
                F::SimDay_Cmd(c, 26i32);
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            16i32 => {
                F::SimDay_Cmd(c, 33i32);
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            17i32 => {
                { let v_ = 174i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 21i32; let a_ = (p_day_1).wrapping_add(2920); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            18i32 => {
                { let v_ = 168i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
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
    if rt::probing() { crate::host::harness::probe(c, "SimEv29", &PROBE_N, &[], true); }
    r_
}

pub fn SimEv30(c: &mut Ctx, mut p_day_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SimEv30", &PROBE_N, &[(p_day_1) as u32], false); }
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
                { let v_ = 3i32; let a_ = (p_day_1).wrapping_add(2940); c.m.w32(a_, (v_) as u32); };
                { let v_ = 6i32; let a_ = (p_day_1).wrapping_add(2936); c.m.w32(a_, (v_) as u32); };
                F::SimDay_Cmd(c, 5i32);
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            4i32 => {
                F::SimDay_Cmd(c, 6i32);
                { let v_ = 175i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            5i32 => {
                { let v_ = 176i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                F::SimDay_Cmd(c, 17i32);
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            6i32 => {
                { let v_ = (1i32).wrapping_neg(); let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                if ((((((c.m.r32((p_day_1).wrapping_add(2956)) as i32)) != (0i32)) as i32)) != 0) {
                    { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                } else {
                    { let v_ = 13i32; let a_ = (p_day_1).wrapping_add(2920); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                }
                break 's1;
            }
            7i32 => {
                { let v_ = 177i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            8i32 => {
                F::SimDay_Cmd(c, 7i32);
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            9i32 => {
                { let v_ = 2i32; let a_ = (p_day_1).wrapping_add(2940); c.m.w32(a_, (v_) as u32); };
                F::SimDay_Cmd(c, 6i32);
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            10i32 => {
                { let v_ = 178i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            11i32 => {
                { let v_ = 179i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                F::SimDay_AddChange(c, 1i32, (5i32).wrapping_neg());
                F::SimDay_Cmd(c, 26i32);
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            12i32 => {
                { let v_ = 180i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 17i32; let a_ = (p_day_1).wrapping_add(2920); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            13i32 => {
                { let v_ = 181i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            14i32 => {
                { let v_ = 182i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                F::SimDay_AddChange(c, 3i32, 1000i32);
                F::SimDay_Cmd(c, 26i32);
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
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
    if rt::probing() { crate::host::harness::probe(c, "SimEv30", &PROBE_N, &[], true); }
    r_
}
