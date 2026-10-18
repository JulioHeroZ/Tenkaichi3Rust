#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn SimPopo_Swap(c: &mut Ctx, mut p_day_1: u32, mut p_a_2: i32, mut p_b_3: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SimPopo_Swap", &PROBE_N, &[(p_day_1) as u32, (p_a_2) as u32, (p_b_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(80) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 80);
    let mut l_done_4: i32 = 0;
    let mut l_d_5: i32 = 0;
    let mut l_flash_6: u32 = 0;
    let r_ = 'f: {
        l_done_4 = 0i32;
        's1: {
            let sv7: i32 = (c.m.r32(sym::gSimPopoState) as i32);
            match sv7 {
                0i32 => {
                    { let r_ = { let a_ = (sym::menu_u_d__sSimPopoSpeed).wrapping_add(((((c.m.r32(sym::gSimPopoSpeed) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = sym::gSimPopoDy; let n_ = ((c.m.r32(a_) as i32)).wrapping_sub(r_); c.m.w32(a_, (n_) as u32); };
                    if ((((((c.m.r32(sym::gSimPopoDy) as i32)) <= ((110i32).wrapping_neg())) as i32)) != 0) {
                        { let v_ = 1i32; let a_ = sym::gSimPopoState; c.m.w32(a_, (v_) as u32); };
                        { let v_ = (110i32).wrapping_neg(); let a_ = sym::gSimPopoDy; c.m.w32(a_, (v_) as u32); };
                    }
                    break 's1;
                }
                1i32 => {
                    l_d_5 = ((c.m.r32((sym::menu_u_d__sSimPopoX).wrapping_add((((p_b_3) as u32)).wrapping_mul(4))) as i32)).wrapping_sub((c.m.r32((sym::menu_u_d__sSimPopoX).wrapping_add((((p_a_2) as u32)).wrapping_mul(4))) as i32));
                    { let r_ = { let a_ = (sym::menu_u_d__sSimPopoSpeed).wrapping_add(((((c.m.r32(sym::gSimPopoSpeed) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = sym::gSimPopoDx; let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                    if ((((((c.m.r32(sym::gSimPopoDx) as i32)) >= (l_d_5)) as i32)) != 0) {
                        { let v_ = l_d_5; let a_ = sym::gSimPopoDx; c.m.w32(a_, (v_) as u32); };
                        { let v_ = 2i32; let a_ = sym::gSimPopoState; c.m.w32(a_, (v_) as u32); };
                    }
                    break 's1;
                }
                2i32 => {
                    { let r_ = { let a_ = (sym::menu_u_d__sSimPopoSpeed).wrapping_add(((((c.m.r32(sym::gSimPopoSpeed) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = sym::gSimPopoDy; let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                    if ((((((c.m.r32(sym::gSimPopoDy) as i32)) >= (0i32)) as i32)) != 0) {
                        { let v_ = 0i32; let a_ = sym::gSimPopoDy; c.m.w32(a_, (v_) as u32); };
                        { let v_ = 3i32; let a_ = sym::gSimPopoState; c.m.w32(a_, (v_) as u32); };
                    }
                    break 's1;
                }
                3i32 => {
                    l_done_4 = 1i32;
                    { let v_ = 0i32; let a_ = sym::gSimPopoState; c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = sym::gSimPopoDy; c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = sym::gSimPopoDx; c.m.w32(a_, (v_) as u32); };
                    if (((((p_a_2) == ((c.m.r32(sym::gSimPopoTarget) as i32))) as i32)) != 0) {
                        { let v_ = p_b_3; let a_ = sym::gSimPopoTarget; c.m.w32(a_, (v_) as u32); };
                    } else {
                        if (((((p_b_3) == ((c.m.r32(sym::gSimPopoTarget) as i32))) as i32)) != 0) {
                            { let v_ = p_a_2; let a_ = sym::gSimPopoTarget; c.m.w32(a_, (v_) as u32); };
                        }
                    }
                    break 's1;
                }
                _ => {
                }
            }
        }
        l_flash_6 = ((p_day_1).wrapping_add(32)).wrapping_add((((3i32) as u32)).wrapping_mul(44));
        { let _ = F::sprintf(c, fp.wrapping_add(8), sym::menu_u_d__sSimPopoClip, &[((p_a_2) as i64 as u64)]); };
        F::Flash_FindLabel(c, l_flash_6, ((0i32) as u32), fp.wrapping_add(8), fp);
        { let a2_ = (c.m.r32(sym::gSimPopoDx) as i32); let a3_ = (c.m.r32(sym::gSimPopoDy) as i32); F::Flash_ClipSetOffset(c, l_flash_6, fp, a2_, a3_) };
        { let _ = F::sprintf(c, fp.wrapping_add(8), sym::menu_u_d__sSimPopoClip, &[((p_b_3) as i64 as u64)]); };
        F::Flash_FindLabel(c, l_flash_6, ((0i32) as u32), fp.wrapping_add(8), fp);
        { let a2_ = ((c.m.r32(sym::gSimPopoDx) as i32)).wrapping_neg(); let a3_ = (c.m.r32(sym::gSimPopoDy) as i32); F::Flash_ClipSetOffset(c, l_flash_6, fp, a2_, a3_) };
        break 'f l_done_4;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "SimPopo_Swap", &PROBE_N, &[], true); }
    r_
}

pub fn SimPopo_CursorOn(c: &mut Ctx, mut p_day_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SimPopo_CursorOn", &PROBE_N, &[(p_day_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(80) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 80);
    let mut l_flash_2: u32 = 0;
    let r_ = 'f: {
        l_flash_2 = ((p_day_1).wrapping_add(32)).wrapping_add((((3i32) as u32)).wrapping_mul(44));
        { let _ = { let a2_ = (c.m.r32(((p_day_1).wrapping_add(1368)).wrapping_add((((3i32) as u32)).wrapping_mul(4))) as i32); F::sprintf(c, fp.wrapping_add(8), sym::menu_u_d__sSimPopoClip, &[((a2_) as i64 as u64)]) }; };
        F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), fp.wrapping_add(8), fp);
        F::Flash_ClipGotoLabel(c, l_flash_2, fp, sym::menu_u_d__str0);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "SimPopo_CursorOn", &PROBE_N, &[], true); }
    r_
}

pub fn SimPopo_CursorOff(c: &mut Ctx, mut p_day_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SimPopo_CursorOff", &PROBE_N, &[(p_day_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(80) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 80);
    let mut l_flash_2: u32 = 0;
    let r_ = 'f: {
        l_flash_2 = ((p_day_1).wrapping_add(32)).wrapping_add((((3i32) as u32)).wrapping_mul(44));
        { let _ = { let a2_ = (c.m.r32(((p_day_1).wrapping_add(1368)).wrapping_add((((3i32) as u32)).wrapping_mul(4))) as i32); F::sprintf(c, fp.wrapping_add(8), sym::menu_u_d__sSimPopoClip, &[((a2_) as i64 as u64)]) }; };
        F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), fp.wrapping_add(8), fp);
        F::Flash_ClipGotoLabel(c, l_flash_2, fp, sym::menu_u_d__str1);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "SimPopo_CursorOff", &PROBE_N, &[], true); }
    r_
}

pub fn SimEv34(c: &mut Ctx, mut p_day_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SimEv34", &PROBE_N, &[(p_day_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(96) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 96);
    let mut l_done_2: i32 = 0;
    let mut l_left_3: i32 = 0;
    let mut l_ok_4: i32 = 0;
    let mut l_right_5: i32 = 0;
    let mut l_flash_6: u32 = 0;
    let mut l_i_7: i32 = 0;
    let mut l_tmp_8: i32 = 0;
    let r_ = 'f: {
        l_done_2 = 0i32;
        l_left_3 = ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((1i32) as u32)))) as i32);
        l_ok_4 = ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) as i32);
        l_right_5 = ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((2i32) as u32)))) as i32);
        's1: {
            let sv9: i32 = (c.m.r32((p_day_1).wrapping_add(2920)) as i32);
            match sv9 {
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
                    { let v_ = 4i32; let a_ = (p_day_1).wrapping_add(2940); c.m.w32(a_, (v_) as u32); };
                    F::SimDay_Cmd(c, 5i32);
                    { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                    break 's1;
                }
                4i32 => {
                    F::SimDay_Cmd(c, 6i32);
                    { let v_ = 193i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                    { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                    break 's1;
                }
                5i32 => {
                    { let v_ = 199i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                    F::SimDay_Cmd(c, 17i32);
                    { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                    break 's1;
                }
                6i32 => {
                    if ((((((c.m.r32((p_day_1).wrapping_add(2956)) as i32)) == (0i32)) as i32)) != 0) {
                        { let v_ = 24i32; let a_ = (p_day_1).wrapping_add(2920); c.m.w32(a_, (v_) as u32); };
                        { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                    } else {
                        { let v_ = (c.m.r32(((p_day_1).wrapping_add(2944)).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32); let a_ = sym::gSimPopoRank; c.m.w32(a_, (v_) as u32); };
                        { let v_ = ((((F::Rand_Range(c, ((((5i32) as i64 as u64)) as u32))) as u64)) as i32); let a_ = sym::gSimPopoTarget; c.m.w32(a_, (v_) as u32); };
                        { let v_ = 0i32; let a_ = sym::gSimPopoCount; c.m.w32(a_, (v_) as u32); };
                        { let v_ = 0i32; let a_ = sym::gSimPopoWon; c.m.w32(a_, (v_) as u32); };
                        { let v_ = { let a_ = ((sym::gSimTrain2).wrapping_add(((((c.m.r32(sym::gSimPopoRank) as i32)) as u32)).wrapping_mul(24))).wrapping_add(4); (c.m.r32(a_) as i32) }; let a_ = sym::gSimPopoSpeed; c.m.w32(a_, (v_) as u32); };
                        { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                    }
                    break 's1;
                }
                7i32 => {
                    { let v_ = 201i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                    { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                    break 's1;
                }
                8i32 => {
                    F::StreamSe_PlayDefault(c, 0i32, 68577i32);
                    F::Bgm_Stop(c);
                    { let v_ = 202i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                    F::SimDay_Cmd(c, 45i32);
                    { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                    break 's1;
                }
                9i32 => {
                    if ((((c.m.r32((((p_day_1).wrapping_add(32)).wrapping_add((((3i32) as u32)).wrapping_mul(44))).wrapping_add(12))) & (((1i32) as u32)))) != 0) {
                        F::SimDay_Cmd(c, 46i32);
                        { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                    }
                    break 's1;
                }
                10i32 => {
                    if ((((({ let a_ = (p_day_1).wrapping_add(2924); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); n_ }) > (120i32)) as i32)) != 0) {
                        F::SimDay_Cmd(c, 47i32);
                        { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                    }
                    break 's1;
                }
                11i32 => {
                    if ((((({ let a_ = (p_day_1).wrapping_add(2924); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); n_ }) > (10i32)) as i32)) != 0) {
                        F::SimDay_Cmd(c, 48i32);
                        { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                    }
                    break 's1;
                }
                12i32 => {
                    if ((((c.m.r32((((p_day_1).wrapping_add(32)).wrapping_add((((3i32) as u32)).wrapping_mul(44))).wrapping_add(12))) & (((16i32) as u32)))) != 0) {
                        { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                    }
                    break 's1;
                }
                13i32 => {
                    if ((((({ let a_ = (p_day_1).wrapping_add(2924); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); n_ }) > (60i32)) as i32)) != 0) {
                        { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                    }
                    break 's1;
                }
                14i32 => {
                    if ((((((c.m.r32(sym::gSimPopoCount) as i32)) >= ({ let a_ = (sym::gSimTrain2).wrapping_add(((((c.m.r32(sym::gSimPopoRank) as i32)) as u32)).wrapping_mul(24)); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                        { let v_ = 16i32; let a_ = (p_day_1).wrapping_add(2920); c.m.w32(a_, (v_) as u32); };
                        { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                        F::SimPopo_CursorOn(c, p_day_1);
                    } else {
                        if (((((((((F::Rand_Range(c, ((((256i32) as i64 as u64)) as u32))) as u64)) as i32)) < (170i32)) as i32)) != 0) {
                            { let v_ = ((((F::Rand_Range(c, ((((5i32) as i64 as u64)) as u32))) as u64)) as i32); let a_ = sym::gSimPopoA; c.m.w32(a_, (v_) as u32); };
                        } else {
                            { let v_ = (c.m.r32(sym::gSimPopoTarget) as i32); let a_ = sym::gSimPopoA; c.m.w32(a_, (v_) as u32); };
                        }
                        'l2: loop {
                            'c3: {
                                { let v_ = ((((F::Rand_Range(c, ((((5i32) as i64 as u64)) as u32))) as u64)) as i32); let a_ = sym::gSimPopoB; c.m.w32(a_, (v_) as u32); };
                            }
                            if !((((((((((c.m.r32(sym::gSimPopoA) as i32)) == ((c.m.r32(sym::gSimPopoB) as i32))) as i32)) != 0) || ((((((c.m.r32(sym::gSimPopoB) as i32)) == ((c.m.r32(sym::gSimPopoTarget) as i32))) as i32)) != 0)) as i32)) != 0) { break 'l2; }
                        }
                        if ((((((c.m.r32(sym::gSimPopoB) as i32)) < ((c.m.r32(sym::gSimPopoA) as i32))) as i32)) != 0) {
                            l_tmp_8 = (c.m.r32(sym::gSimPopoA) as i32);
                            { let v_ = (c.m.r32(sym::gSimPopoB) as i32); let a_ = sym::gSimPopoA; c.m.w32(a_, (v_) as u32); };
                            { let v_ = l_tmp_8; let a_ = sym::gSimPopoB; c.m.w32(a_, (v_) as u32); };
                        }
                        { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                        { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 30i32); };
                        { let a_ = sym::gSimPopoCount; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    }
                    break 's1;
                }
                15i32 => {
                    if (({ let a1_ = (c.m.r32(sym::gSimPopoA) as i32); let a2_ = (c.m.r32(sym::gSimPopoB) as i32); F::SimPopo_Swap(c, p_day_1, a1_, a2_) }) != 0) {
                        { let v_ = 14i32; let a_ = (p_day_1).wrapping_add(2920); c.m.w32(a_, (v_) as u32); };
                        { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                    }
                    break 's1;
                }
                16i32 => {
                    if ((l_ok_4) != 0) {
                        if ((((({ let a_ = ((p_day_1).wrapping_add(1368)).wrapping_add(((((c.m.r32((p_day_1).wrapping_add(1424)) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) == ((c.m.r32(sym::gSimPopoTarget) as i32))) as i32)) != 0) {
                            { let v_ = 1i32; let a_ = sym::gSimPopoWon; c.m.w32(a_, (v_) as u32); };
                        }
                        F::SimDay_Cmd(c, 49i32);
                        { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                    } else {
                        if ((l_left_3) != 0) {
                            F::SimPopo_CursorOff(c, p_day_1);
                            { let a_ = ((p_day_1).wrapping_add(1368)).wrapping_add(((((c.m.r32((p_day_1).wrapping_add(1424)) as i32)) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                            if ((((({ let a_ = ((p_day_1).wrapping_add(1368)).wrapping_add(((((c.m.r32((p_day_1).wrapping_add(1424)) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) < (0i32)) as i32)) != 0) {
                                { let v_ = 4i32; let a_ = ((p_day_1).wrapping_add(1368)).wrapping_add(((((c.m.r32((p_day_1).wrapping_add(1424)) as i32)) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                            }
                            F::SimPopo_CursorOn(c, p_day_1);
                        } else {
                            if ((l_right_5) != 0) {
                                F::SimPopo_CursorOff(c, p_day_1);
                                { let a_ = ((p_day_1).wrapping_add(1368)).wrapping_add(((((c.m.r32((p_day_1).wrapping_add(1424)) as i32)) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                                if ((((({ let a_ = ((p_day_1).wrapping_add(1368)).wrapping_add(((((c.m.r32((p_day_1).wrapping_add(1424)) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) >= (5i32)) as i32)) != 0) {
                                    { let v_ = 0i32; let a_ = ((p_day_1).wrapping_add(1368)).wrapping_add(((((c.m.r32((p_day_1).wrapping_add(1424)) as i32)) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                }
                                F::SimPopo_CursorOn(c, p_day_1);
                            }
                        }
                    }
                    break 's1;
                }
                17i32 => {
                    if ((((c.m.r32((((p_day_1).wrapping_add(32)).wrapping_add((((3i32) as u32)).wrapping_mul(44))).wrapping_add(12))) & (((4i32) as u32)))) != 0) {
                        { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                    }
                    break 's1;
                }
                18i32 => {
                    if ((((({ let a_ = (p_day_1).wrapping_add(2924); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); n_ }) > (30i32)) as i32)) != 0) {
                        if (((c.m.r32(sym::gSimPopoWon) as i32)) != 0) {
                            F::SimDay_Cmd(c, 50i32);
                            { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 31i32); };
                        } else {
                            F::SimDay_Cmd(c, 51i32);
                            { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 32i32); };
                        }
                        { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                    }
                    break 's1;
                }
                19i32 => {
                    if ((((c.m.r32((((p_day_1).wrapping_add(32)).wrapping_add((((3i32) as u32)).wrapping_mul(44))).wrapping_add(12))) & (((2i32) as u32)))) != 0) {
                        F::StreamSe_Stop(c, 0i32);
                        F::SimDay_PlayBgm(c);
                        F::SimDay_Cmd(c, 52i32);
                        { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                    }
                    break 's1;
                }
                20i32 => {
                    if ((((c.m.r32((((p_day_1).wrapping_add(32)).wrapping_add((((3i32) as u32)).wrapping_mul(44))).wrapping_add(12))) & (((32i32) as u32)))) != 0) {
                        F::SimDay_Cmd(c, 53i32);
                        if (((c.m.r32(sym::gSimPopoWon) as i32)) != 0) {
                            { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                            { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                        } else {
                            { let v_ = 23i32; let a_ = (p_day_1).wrapping_add(2920); c.m.w32(a_, (v_) as u32); };
                            { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                        }
                    }
                    break 's1;
                }
                21i32 => {
                    { let v_ = 203i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                    { let a1_ = { let a_ = ((sym::gSimTrain2).wrapping_add(((((c.m.r32(sym::gSimPopoRank) as i32)) as u32)).wrapping_mul(24))).wrapping_add(12); (c.m.r32(a_) as i32) }; F::SimDay_AddChange(c, 1i32, a1_) };
                    { let a1_ = { let a_ = ((sym::gSimTrain2).wrapping_add(((((c.m.r32(sym::gSimPopoRank) as i32)) as u32)).wrapping_mul(24))).wrapping_add(8); (c.m.r32(a_) as i32) }; F::SimDay_AddChange(c, 3i32, a1_) };
                    F::SimDay_Cmd(c, 26i32);
                    { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                    { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    break 's1;
                }
                22i32 => {
                    { let v_ = 204i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                    { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    { let v_ = 27i32; let a_ = (p_day_1).wrapping_add(2920); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                    break 's1;
                }
                23i32 => {
                    { let v_ = 205i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                    { let a1_ = { let a_ = ((sym::gSimTrain2).wrapping_add(((((c.m.r32(sym::gSimPopoRank) as i32)) as u32)).wrapping_mul(24))).wrapping_add(16); (c.m.r32(a_) as i32) }; F::SimDay_AddChange(c, 1i32, a1_) };
                    F::SimDay_Cmd(c, 26i32);
                    { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 27i32; let a_ = (p_day_1).wrapping_add(2920); c.m.w32(a_, (v_) as u32); };
                    break 's1;
                }
                24i32 => {
                    { let v_ = 200i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                    { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                    break 's1;
                }
                25i32 => {
                    { let v_ = (1i32).wrapping_neg(); let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                    F::SimDay_Cmd(c, 7i32);
                    { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                    break 's1;
                }
                26i32 => {
                    { let v_ = 99i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                    { let a1_ = (((((F::Rand_Range(c, ((((3i32) as i64 as u64)) as u32))) as u64)) as i32)).wrapping_neg(); F::SimDay_AddChange(c, 0i32, a1_) };
                    { let a1_ = (((((F::Rand_Range(c, ((((3i32) as i64 as u64)) as u32))) as u64)) as i32)).wrapping_neg(); F::SimDay_AddChange(c, 1i32, a1_) };
                    F::SimDay_Cmd(c, 26i32);
                    { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                    { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    break 's1;
                }
                27i32 => {
                    l_done_2 = 1i32;
                    { let v_ = (1i32).wrapping_neg(); let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                    break 's1;
                }
                _ => {
                }
            }
        }
        l_flash_6 = ((p_day_1).wrapping_add(32)).wrapping_add((((3i32) as u32)).wrapping_mul(44));
        F::Flash_FindLabel(c, l_flash_6, ((0i32) as u32), sym::menu_u_d__str2, fp);
        { let a2_ = { let a_ = (sym::menu_u_d__sSimPopoX).wrapping_add(((((c.m.r32(sym::gSimPopoTarget) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; F::Flash_ClipSetOffset(c, l_flash_6, fp, a2_, 0i32) };
        { let v_ = 128i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = 512i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 256i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        F::Flash_FindLabel(c, l_flash_6, ((0i32) as u32), sym::menu_u_d__str3, fp);
        F::Flash_ClipSetUv(c, l_flash_6, fp, fp.wrapping_add(8));
        {
            l_i_7 = 0i32;
            'l4: loop {
                if !(((((l_i_7) < (5i32)) as i32)) != 0) { break 'l4; }
                'c5: {
                    if (((((l_i_7) != ((c.m.r32(sym::gSimPopoTarget) as i32))) as i32)) != 0) {
                        { let v_ = 64i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                        { let v_ = 0i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                        { let v_ = 128i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                        { let v_ = 64i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                    } else {
                        { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                        { let v_ = 0i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                        { let v_ = 64i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                        { let v_ = 64i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                    }
                    { let _ = F::sprintf(c, fp.wrapping_add(28), sym::menu_u_d__sSimPopoClip, &[((l_i_7) as i64 as u64)]); };
                    F::Flash_FindLabel(c, l_flash_6, fp.wrapping_add(28), sym::menu_u_d__str4, fp);
                    F::Flash_ClipSetUv(c, l_flash_6, fp, fp.wrapping_add(8));
                    if (((((l_i_7) != ((c.m.r32(sym::gSimPopoTarget) as i32))) as i32)) != 0) {
                        { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                        { let v_ = 0i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                        { let v_ = 64i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                        { let v_ = 128i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                    } else {
                        { let v_ = 64i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                        { let v_ = 0i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                        { let v_ = 128i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                        { let v_ = 128i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                    }
                    F::Flash_FindLabel(c, l_flash_6, fp.wrapping_add(28), sym::menu_u_d__str5, fp);
                    F::Flash_ClipSetUv(c, l_flash_6, fp, fp.wrapping_add(8));
                }
                { let o_ = l_i_7; let n_ = (o_).wrapping_add(1); l_i_7 = n_; };
            }
        }
        if ((((c.m.r32((l_flash_6).wrapping_add(16))) & (((1i32) as u32)))) != 0) {
            { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 60i32); };
        }
        break 'f l_done_2;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "SimEv34", &PROBE_N, &[], true); }
    r_
}
