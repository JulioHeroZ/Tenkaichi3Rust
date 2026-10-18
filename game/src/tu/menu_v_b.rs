#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn EvoZ_ClipGoto(c: &mut Ctx, mut p_ez_1: u32, mut p_flash_2: i32, mut p_kind_3: i32, mut p_sub_4: i32, mut p_label_5: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EvoZ_ClipGoto", &PROBE_N, &[(p_ez_1) as u32, (p_flash_2) as u32, (p_kind_3) as u32, (p_sub_4) as u32, (p_label_5) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(80) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 80);
    let mut l_f_6: u32 = 0;
    let r_ = 'f: {
        l_f_6 = ((p_ez_1).wrapping_add(44)).wrapping_add((((p_flash_2) as u32)).wrapping_mul(44));
        's1: {
            let sv7: i32 = p_kind_3;
            match sv7 {
                0i32 => {
                    { let _ = { let a2_ = { let a_ = c.m.r32((p_ez_1).wrapping_add(788)); (c.m.r32(a_) as i32) }; F::sprintf(c, fp.wrapping_add(8), sym::menu_v_b__str0, &[((a2_) as i64 as u64)]) }; };
                    break 's1;
                }
                1i32 => {
                    { let _ = { let a2_ = ((c.m.r32(((p_ez_1).wrapping_add(560)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)).wrapping_add(1i32); F::sprintf(c, fp.wrapping_add(8), sym::menu_v_b__str1, &[((a2_) as i64 as u64)]) }; };
                    break 's1;
                }
                2i32 => {
                    { let _ = ((((F::strcpy(c, fp.wrapping_add(8), sym::menu_v_b__str2)) as u64)) as i32); };
                    break 's1;
                }
                3i32 => {
                    { let _ = { let a2_ = ((c.m.r32(((p_ez_1).wrapping_add(560)).wrapping_add((((3i32) as u32)).wrapping_mul(4))) as i32)).wrapping_add(1i32); F::sprintf(c, fp.wrapping_add(8), sym::menu_v_b__str3, &[((a2_) as i64 as u64)]) }; };
                    break 's1;
                }
                5i32 => {
                    { let _ = { let a2_ = (c.m.r32(((p_ez_1).wrapping_add(560)).wrapping_add((((5i32) as u32)).wrapping_mul(4))) as i32); F::sprintf(c, fp.wrapping_add(8), sym::menu_v_b__str4, &[((a2_) as i64 as u64)]) }; };
                    break 's1;
                }
                10i32 => {
                    if (((((p_sub_4) == (0i32)) as i32)) != 0) {
                        { let _ = { let a2_ = (({ let a_ = ((p_ez_1).wrapping_add(6808)).wrapping_add(((((c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(16)) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_sub({ let a_ = (((p_ez_1).wrapping_add(6808)).wrapping_add(5648)).wrapping_add(((((c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(16)) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) })).wrapping_add(2i32); F::sprintf(c, fp.wrapping_add(8), sym::menu_v_b__str5, &[((a2_) as i64 as u64)]) }; };
                    } else {
                        { let _ = { let a2_ = (c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(16)) as i32); F::sprintf(c, fp.wrapping_add(8), sym::menu_v_b__str6, &[((a2_) as i64 as u64)]) }; };
                    }
                    break 's1;
                }
                _ => {
                    break 'f;
                }
            }
        }
        F::Flash_FindLabel(c, l_f_6, ((0i32) as u32), fp.wrapping_add(8), fp);
        F::Flash_ClipGotoLabel(c, l_f_6, fp, p_label_5);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EvoZ_ClipGoto", &PROBE_N, &[], true); }
    r_
}

pub fn EvoZ_Input(c: &mut Ctx, mut p_ez_1: u32, mut p_result_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EvoZ_Input", &PROBE_N, &[(p_ez_1) as u32, (p_result_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_id_3: i32 = 0;
    let mut l_f_4: u32 = 0;
    let mut l_id_5: i32 = 0;
    let r_ = 'f: {
        if ((((!((((c.m.r32((((p_ez_1).wrapping_add(44)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add(8))) & (((2i32) as u32)))) != 0)) as i32)) != 0) {
            break 'f;
        }
        if ((((!(((((c.m.r32((p_ez_1).wrapping_add(556)) as i32)) & (4i32))) != 0)) as i32)) != 0) {
            F::EvoZ_ClipGoto(c, p_ez_1, 1i32, 0i32, 0i32, sym::menu_v_b__str7);
            { let r_ = 4i32; let a_ = (p_ez_1).wrapping_add(556); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        }
        's1: {
            let sv6: i32 = (c.m.r32((p_ez_1).wrapping_add(608)) as i32);
            match sv6 {
                0i32 => {
                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((1i32) as u32)))) != 0) {
                        F::EvoZ_ClipGoto(c, p_ez_1, 1i32, 0i32, 0i32, sym::menu_v_b__str8);
                        { let a0_ = c.m.r32((p_ez_1).wrapping_add(796)); let a1_ = c.m.r32((p_ez_1).wrapping_add(788)); let a2_ = { let a_ = (c.m.r32((p_ez_1).wrapping_add(788))).wrapping_add(4); (c.m.r32(a_) as i32) }; { let _ = F::ChrGrid_MoveLeft(c, a0_, a1_, a2_); } };
                        F::EvoZ_ClipGoto(c, p_ez_1, 1i32, 0i32, 0i32, sym::menu_v_b__str9);
                        { let v_ = { let a_ = ((c.m.r32((p_ez_1).wrapping_add(788))).wrapping_add(48)).wrapping_add(((({ let a_ = c.m.r32((p_ez_1).wrapping_add(788)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32((p_ez_1).wrapping_add(788))).wrapping_add(108); c.m.w32(a_, (v_) as u32); };
                        F::EvoZ_RefreshStatus(c, p_ez_1);
                        F::EvoZ_RequestFace(c, p_ez_1);
                        { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 0i32); };
                    } else {
                        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((2i32) as u32)))) != 0) {
                            F::EvoZ_ClipGoto(c, p_ez_1, 1i32, 0i32, 0i32, sym::menu_v_b__str10);
                            { let a0_ = c.m.r32((p_ez_1).wrapping_add(796)); let a1_ = c.m.r32((p_ez_1).wrapping_add(788)); let a2_ = { let a_ = (c.m.r32((p_ez_1).wrapping_add(788))).wrapping_add(4); (c.m.r32(a_) as i32) }; { let _ = F::ChrGrid_MoveRight(c, a0_, a1_, a2_); } };
                            F::EvoZ_ClipGoto(c, p_ez_1, 1i32, 0i32, 0i32, sym::menu_v_b__str11);
                            { let v_ = { let a_ = ((c.m.r32((p_ez_1).wrapping_add(788))).wrapping_add(48)).wrapping_add(((({ let a_ = c.m.r32((p_ez_1).wrapping_add(788)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32((p_ez_1).wrapping_add(788))).wrapping_add(108); c.m.w32(a_, (v_) as u32); };
                            F::EvoZ_RefreshStatus(c, p_ez_1);
                            F::EvoZ_RequestFace(c, p_ez_1);
                            { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 0i32); };
                        } else {
                            if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((8i32) as u32)))) != 0) {
                                if ((((((c.m.r32((p_ez_1).wrapping_add(6744)) as i32)) >= (2i32)) as i32)) != 0) {
                                    F::Flash_GotoLabel(c, ((p_ez_1).wrapping_add(44)).wrapping_add((((1i32) as u32)).wrapping_mul(44)), sym::menu_v_b__str12, ((((1i32) as i64 as u64)) as u8));
                                    { let v_ = 1i32; let a_ = (c.m.r32((p_ez_1).wrapping_add(788))).wrapping_add(112); c.m.w32(a_, (v_) as u32); };
                                    F::EvoZ_ClipGoto(c, p_ez_1, 1i32, 0i32, 0i32, sym::menu_v_b__str13);
                                    { let a0_ = c.m.r32((p_ez_1).wrapping_add(796)); let a1_ = c.m.r32((p_ez_1).wrapping_add(788)); let a2_ = (c.m.r32((p_ez_1).wrapping_add(788))).wrapping_add(4); let a3_ = (c.m.r32((p_ez_1).wrapping_add(6744)) as i32); F::ChrGrid_MoveUp(c, a0_, a1_, a2_, a3_) };
                                    F::EvoZ_ClipGoto(c, p_ez_1, 1i32, 0i32, 0i32, sym::menu_v_b__str14);
                                    F::EvoZ_SetRowChips(c, p_ez_1);
                                    { let v_ = { let a_ = ((c.m.r32((p_ez_1).wrapping_add(788))).wrapping_add(48)).wrapping_add(((({ let a_ = c.m.r32((p_ez_1).wrapping_add(788)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32((p_ez_1).wrapping_add(788))).wrapping_add(108); c.m.w32(a_, (v_) as u32); };
                                    F::EvoZ_RefreshStatus(c, p_ez_1);
                                    F::EvoZ_RequestFace(c, p_ez_1);
                                    { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 2i32); };
                                }
                            } else {
                                if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((4i32) as u32)))) != 0) {
                                    if ((((((c.m.r32((p_ez_1).wrapping_add(6744)) as i32)) >= (2i32)) as i32)) != 0) {
                                        F::Flash_GotoLabel(c, ((p_ez_1).wrapping_add(44)).wrapping_add((((1i32) as u32)).wrapping_mul(44)), sym::menu_v_b__str15, ((((1i32) as i64 as u64)) as u8));
                                        { let v_ = 1i32; let a_ = (c.m.r32((p_ez_1).wrapping_add(788))).wrapping_add(112); c.m.w32(a_, (v_) as u32); };
                                        F::EvoZ_ClipGoto(c, p_ez_1, 1i32, 0i32, 0i32, sym::menu_v_b__str16);
                                        { let a0_ = c.m.r32((p_ez_1).wrapping_add(796)); let a1_ = c.m.r32((p_ez_1).wrapping_add(788)); let a2_ = (c.m.r32((p_ez_1).wrapping_add(788))).wrapping_add(4); let a3_ = (c.m.r32((p_ez_1).wrapping_add(6744)) as i32); F::ChrGrid_MoveDown(c, a0_, a1_, a2_, a3_) };
                                        F::EvoZ_ClipGoto(c, p_ez_1, 1i32, 0i32, 0i32, sym::menu_v_b__str17);
                                        F::EvoZ_SetRowChips(c, p_ez_1);
                                        { let v_ = { let a_ = ((c.m.r32((p_ez_1).wrapping_add(788))).wrapping_add(48)).wrapping_add(((({ let a_ = c.m.r32((p_ez_1).wrapping_add(788)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32((p_ez_1).wrapping_add(788))).wrapping_add(108); c.m.w32(a_, (v_) as u32); };
                                        F::EvoZ_RefreshStatus(c, p_ez_1);
                                        F::EvoZ_RequestFace(c, p_ez_1);
                                        { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 1i32); };
                                    }
                                } else {
                                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                                        F::Flash_GotoLabel(c, ((p_ez_1).wrapping_add(44)).wrapping_add((((0i32) as u32)).wrapping_mul(44)), sym::menu_v_b__str18, ((((1i32) as i64 as u64)) as u8));
                                        F::Flash_GotoLabel(c, ((p_ez_1).wrapping_add(44)).wrapping_add((((1i32) as u32)).wrapping_mul(44)), sym::menu_v_b__str19, ((((1i32) as i64 as u64)) as u8));
                                        F::EvoZ_ClipGoto(c, p_ez_1, 1i32, 0i32, 0i32, sym::menu_v_b__str20);
                                        F::EvoZ_ClipGoto(c, p_ez_1, 0i32, 1i32, 0i32, sym::menu_v_b__str21);
                                        F::EvoZ_ClipGoto(c, p_ez_1, 0i32, 2i32, 0i32, sym::menu_v_b__str22);
                                        { let v_ = 0i32; let a_ = ((p_ez_1).wrapping_add(560)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                        { let v_ = 1i32; let a_ = (p_ez_1).wrapping_add(608); c.m.w32(a_, (v_) as u32); };
                                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                                    } else {
                                        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1024i32) as u32)))) != 0) {
                                            F::EvoZ_ClipGoto(c, p_ez_1, 1i32, 0i32, 0i32, sym::menu_v_b__str23);
                                            F::Flash_GotoLabel(c, ((p_ez_1).wrapping_add(44)).wrapping_add((((0i32) as u32)).wrapping_mul(44)), sym::menu_v_b__str24, ((((1i32) as i64 as u64)) as u8));
                                            F::ColorFade_StartOut(c, 0i32, 0i32, 0i32, 20i32);
                                            { let v_ = 0i32; let a_ = p_result_2; c.m.w32(a_, (v_) as u32); };
                                            { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 2i32); };
                                        }
                                    }
                                }
                            }
                        }
                    }
                    break 's1;
                }
                1i32 => {
                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((8i32) as u32)))) != 0) {
                        F::EvoZ_ClipGoto(c, p_ez_1, 0i32, 1i32, 0i32, sym::menu_v_b__str25);
                        { let a_ = ((p_ez_1).wrapping_add(560)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                        if ((((((c.m.r32(((p_ez_1).wrapping_add(560)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) < (0i32)) as i32)) != 0) {
                            { let v_ = 2i32; let a_ = ((p_ez_1).wrapping_add(560)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                        }
                        F::EvoZ_ClipGoto(c, p_ez_1, 0i32, 1i32, 0i32, sym::menu_v_b__str26);
                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                    } else {
                        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((4i32) as u32)))) != 0) {
                            F::EvoZ_ClipGoto(c, p_ez_1, 0i32, 1i32, 0i32, sym::menu_v_b__str27);
                            { let a_ = ((p_ez_1).wrapping_add(560)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                            if ((((((c.m.r32(((p_ez_1).wrapping_add(560)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) >= (3i32)) as i32)) != 0) {
                                { let v_ = 0i32; let a_ = ((p_ez_1).wrapping_add(560)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                            }
                            F::EvoZ_ClipGoto(c, p_ez_1, 0i32, 1i32, 0i32, sym::menu_v_b__str28);
                            { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                        } else {
                            if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((1i32) as u32)))) != 0) {
                                { let a_ = ((p_ez_1).wrapping_add(560)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                                if ((((((c.m.r32(((p_ez_1).wrapping_add(560)).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32)) < (0i32)) as i32)) != 0) {
                                    { let v_ = 2i32; let a_ = ((p_ez_1).wrapping_add(560)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                }
                                { let v_ = 0i32; let a_ = ((p_ez_1).wrapping_add(560)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                { let v_ = (c.m.r32(((p_ez_1).wrapping_add(560)).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32); let a_ = (p_ez_1).wrapping_add(628); c.m.w32(a_, (v_) as u32); };
                                F::EvoZ_RefreshStatus(c, p_ez_1);
                                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                            } else {
                                if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((2i32) as u32)))) != 0) {
                                    { let a_ = ((p_ez_1).wrapping_add(560)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                                    if ((((((c.m.r32(((p_ez_1).wrapping_add(560)).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32)) >= (3i32)) as i32)) != 0) {
                                        { let v_ = 0i32; let a_ = ((p_ez_1).wrapping_add(560)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                    }
                                    { let v_ = 0i32; let a_ = ((p_ez_1).wrapping_add(560)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                    { let v_ = (c.m.r32(((p_ez_1).wrapping_add(560)).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32); let a_ = (p_ez_1).wrapping_add(628); c.m.w32(a_, (v_) as u32); };
                                    F::EvoZ_RefreshStatus(c, p_ez_1);
                                    { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                                } else {
                                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                                        's2: {
                                            let sv7: i32 = (c.m.r32(((p_ez_1).wrapping_add(560)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32);
                                            match sv7 {
                                                0i32 => {
                                                    F::EvoZ_ClipGoto(c, p_ez_1, 0i32, 1i32, 0i32, sym::menu_v_b__str29);
                                                    F::EvoZ_ClipGoto(c, p_ez_1, 0i32, 2i32, 0i32, sym::menu_v_b__str30);
                                                    F::Flash_GotoLabel(c, ((p_ez_1).wrapping_add(44)).wrapping_add((((0i32) as u32)).wrapping_mul(44)), sym::menu_v_b__str31, ((((1i32) as i64 as u64)) as u8));
                                                    F::Flash_GotoLabel(c, ((p_ez_1).wrapping_add(44)).wrapping_add((((2i32) as u32)).wrapping_mul(44)), sym::menu_v_b__str32, ((((1i32) as i64 as u64)) as u8));
                                                    F::EvoZ_ClipGoto(c, p_ez_1, 2i32, 3i32, 0i32, sym::menu_v_b__str33);
                                                    { let v_ = 3i32; let a_ = (p_ez_1).wrapping_add(608); c.m.w32(a_, (v_) as u32); };
                                                    { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                                                    break 's2;
                                                }
                                                1i32 => {
                                                    F::EvoZ_ClipGoto(c, p_ez_1, 0i32, 1i32, 0i32, sym::menu_v_b__str34);
                                                    F::EvoZ_ClipGoto(c, p_ez_1, 0i32, 2i32, 0i32, sym::menu_v_b__str35);
                                                    { let a0_ = ((((c.m.r32(sym::gSaveData)).wrapping_add(8)).wrapping_add(6144)).wrapping_add((((({ let a_ = c.m.r32((p_ez_1).wrapping_add(788)); (c.m.r32(a_) as i32) }).wrapping_add(({ let a_ = (c.m.r32((p_ez_1).wrapping_add(788))).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(7i32))) as u32)).wrapping_mul(56))).wrapping_add(((((c.m.r32((p_ez_1).wrapping_add(628)) as i32)) as u32)).wrapping_mul(16)); let a1_ = { let a_ = (c.m.r32((p_ez_1).wrapping_add(788))).wrapping_add(108); (c.m.r32(a_) as i32) }; let a2_ = (({ let a_ = ((((c.m.r32(sym::gSaveData)).wrapping_add(8)).wrapping_add(6144)).wrapping_add((((({ let a_ = c.m.r32((p_ez_1).wrapping_add(788)); (c.m.r32(a_) as i32) }).wrapping_add(({ let a_ = (c.m.r32((p_ez_1).wrapping_add(788))).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(7i32))) as u32)).wrapping_mul(56))).wrapping_add(52); c.m.r16(a_) }) as i32); F::PassWin_OpenEx(c, a0_, a1_, a2_) };
                                                    { let v_ = 4i32; let a_ = (p_ez_1).wrapping_add(608); c.m.w32(a_, (v_) as u32); };
                                                    { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                                                    break 's2;
                                                }
                                                2i32 => {
                                                    if ((((((c.m.r32((p_ez_1).wrapping_add(6768)) as i32)) != (0i32)) as i32)) != 0) {
                                                        F::EvoZ_ClipGoto(c, p_ez_1, 0i32, 1i32, 0i32, sym::menu_v_b__str36);
                                                        F::EvoZ_ClipGoto(c, p_ez_1, 0i32, 2i32, 0i32, sym::menu_v_b__str37);
                                                        F::Flash_GotoLabel(c, ((p_ez_1).wrapping_add(44)).wrapping_add((((0i32) as u32)).wrapping_mul(44)), sym::menu_v_b__str38, ((((1i32) as i64 as u64)) as u8));
                                                        F::EvoZ_ClipGoto(c, p_ez_1, 0i32, 5i32, 0i32, sym::menu_v_b__str39);
                                                        { let v_ = 0i32; let a_ = (p_ez_1).wrapping_add(624); c.m.w32(a_, (v_) as u32); };
                                                        { let v_ = 3i32; let a_ = ((p_ez_1).wrapping_add(560)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                                        { let v_ = 5i32; let a_ = (p_ez_1).wrapping_add(608); c.m.w32(a_, (v_) as u32); };
                                                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                                                    } else {
                                                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 7i32); };
                                                    }
                                                    break 's2;
                                                }
                                                _ => {
                                                }
                                            }
                                        }
                                    } else {
                                        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1024i32) as u32)))) != 0) {
                                            F::Flash_GotoLabel(c, ((p_ez_1).wrapping_add(44)).wrapping_add((((0i32) as u32)).wrapping_mul(44)), sym::menu_v_b__str40, ((((1i32) as i64 as u64)) as u8));
                                            { let r_ = (!(256i32)); let a_ = (p_ez_1).wrapping_add(556); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                                            F::Flash_GotoLabel(c, ((p_ez_1).wrapping_add(44)).wrapping_add((((1i32) as u32)).wrapping_mul(44)), sym::menu_v_b__str41, ((((1i32) as i64 as u64)) as u8));
                                            F::EvoZ_ClipGoto(c, p_ez_1, 1i32, 0i32, 0i32, sym::menu_v_b__str42);
                                            F::EvoZ_ClipGoto(c, p_ez_1, 0i32, 1i32, 0i32, sym::menu_v_b__str43);
                                            F::EvoZ_ClipGoto(c, p_ez_1, 0i32, 2i32, 0i32, sym::menu_v_b__str44);
                                            { let v_ = 0i32; let a_ = (p_ez_1).wrapping_add(608); c.m.w32(a_, (v_) as u32); };
                                            { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 2i32); };
                                        }
                                    }
                                }
                            }
                        }
                    }
                    break 's1;
                }
                3i32 => {
                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((8i32) as u32)))) != 0) {
                        F::EvoZ_ClipGoto(c, p_ez_1, 2i32, 3i32, 0i32, sym::menu_v_b__str45);
                        if ((((((c.m.r32(((p_ez_1).wrapping_add(560)).wrapping_add((((3i32) as u32)).wrapping_mul(4))) as i32)) == (7i32)) as i32)) != 0) {
                            'l3: while ((((((c.m.r32(((p_ez_1).wrapping_add(560)).wrapping_add((((3i32) as u32)).wrapping_mul(4))) as i32)) >= ((c.m.r32((p_ez_1).wrapping_add(640)) as i32))) as i32)) != 0) {
                                { let a_ = ((p_ez_1).wrapping_add(560)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                            }
                            if ((((((({ let a_ = (((((c.m.r32(sym::gSaveData)).wrapping_add(8)).wrapping_add(6144)).wrapping_add((((({ let a_ = c.m.r32((p_ez_1).wrapping_add(788)); (c.m.r32(a_) as i32) }).wrapping_add(({ let a_ = (c.m.r32((p_ez_1).wrapping_add(788))).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(7i32))) as u32)).wrapping_mul(56))).wrapping_add(((((c.m.r32((p_ez_1).wrapping_add(628)) as i32)) as u32)).wrapping_mul(16))).wrapping_add(((((c.m.r32(((p_ez_1).wrapping_add(560)).wrapping_add((((3i32) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(2)); c.m.r16(a_) }) as i32)) == (0i32)) as i32)) != 0) {
                                'l4: loop {
                                    'c5: {
                                        { let a_ = ((p_ez_1).wrapping_add(560)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                                        if ((((((({ let a_ = (((((c.m.r32(sym::gSaveData)).wrapping_add(8)).wrapping_add(6144)).wrapping_add((((({ let a_ = c.m.r32((p_ez_1).wrapping_add(788)); (c.m.r32(a_) as i32) }).wrapping_add(({ let a_ = (c.m.r32((p_ez_1).wrapping_add(788))).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(7i32))) as u32)).wrapping_mul(56))).wrapping_add(((((c.m.r32((p_ez_1).wrapping_add(628)) as i32)) as u32)).wrapping_mul(16))).wrapping_add(((((c.m.r32(((p_ez_1).wrapping_add(560)).wrapping_add((((3i32) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(2)); c.m.r16(a_) }) as i32)) != (0i32)) as i32)) != 0) {
                                            { let a_ = ((p_ez_1).wrapping_add(560)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                                            break 'l4;
                                        }
                                    }
                                    if !((((((c.m.r32(((p_ez_1).wrapping_add(560)).wrapping_add((((3i32) as u32)).wrapping_mul(4))) as i32)) != (0i32)) as i32)) != 0) { break 'l4; }
                                }
                            }
                        } else {
                            { let a_ = ((p_ez_1).wrapping_add(560)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                            if ((((((c.m.r32(((p_ez_1).wrapping_add(560)).wrapping_add((((3i32) as u32)).wrapping_mul(4))) as i32)) < (0i32)) as i32)) != 0) {
                                { let v_ = 7i32; let a_ = ((p_ez_1).wrapping_add(560)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                            }
                        }
                        F::EvoZ_ClipGoto(c, p_ez_1, 2i32, 3i32, 0i32, sym::menu_v_b__str46);
                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                    } else {
                        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((4i32) as u32)))) != 0) {
                            F::EvoZ_ClipGoto(c, p_ez_1, 2i32, 3i32, 0i32, sym::menu_v_b__str47);
                            if ((((((c.m.r32(((p_ez_1).wrapping_add(560)).wrapping_add((((3i32) as u32)).wrapping_mul(4))) as i32)) == (7i32)) as i32)) != 0) {
                                { let v_ = 0i32; let a_ = ((p_ez_1).wrapping_add(560)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                            } else {
                                if ((((((((((({ let a_ = (((((c.m.r32(sym::gSaveData)).wrapping_add(8)).wrapping_add(6144)).wrapping_add((((({ let a_ = c.m.r32((p_ez_1).wrapping_add(788)); (c.m.r32(a_) as i32) }).wrapping_add(({ let a_ = (c.m.r32((p_ez_1).wrapping_add(788))).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(7i32))) as u32)).wrapping_mul(56))).wrapping_add(((((c.m.r32((p_ez_1).wrapping_add(628)) as i32)) as u32)).wrapping_mul(16))).wrapping_add(((((c.m.r32(((p_ez_1).wrapping_add(560)).wrapping_add((((3i32) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(2)); c.m.r16(a_) }) as i32)) == (0i32)) as i32)) != 0) || ((((({ let a_ = ((p_ez_1).wrapping_add(560)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); n_ }) >= ((c.m.r32((p_ez_1).wrapping_add(640)) as i32))) as i32)) != 0)) as i32)) != 0) {
                                    { let v_ = 7i32; let a_ = ((p_ez_1).wrapping_add(560)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                }
                            }
                            F::EvoZ_ClipGoto(c, p_ez_1, 2i32, 3i32, 0i32, sym::menu_v_b__str48);
                            { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                        } else {
                            if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((1i32) as u32)))) != 0) {
                                { let a_ = ((p_ez_1).wrapping_add(560)).wrapping_add((((8i32) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                                if ((((((c.m.r32(((p_ez_1).wrapping_add(560)).wrapping_add((((8i32) as u32)).wrapping_mul(4))) as i32)) < (0i32)) as i32)) != 0) {
                                    { let v_ = 3i32; let a_ = ((p_ez_1).wrapping_add(560)).wrapping_add((((8i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                }
                                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                            } else {
                                if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((2i32) as u32)))) != 0) {
                                    { let a_ = ((p_ez_1).wrapping_add(560)).wrapping_add((((8i32) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                                    if ((((((c.m.r32(((p_ez_1).wrapping_add(560)).wrapping_add((((8i32) as u32)).wrapping_mul(4))) as i32)) >= (4i32)) as i32)) != 0) {
                                        { let v_ = 0i32; let a_ = ((p_ez_1).wrapping_add(560)).wrapping_add((((8i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                    }
                                    { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                                } else {
                                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                                        's6: {
                                            let sv8: i32 = (c.m.r32(((p_ez_1).wrapping_add(560)).wrapping_add((((8i32) as u32)).wrapping_mul(4))) as i32);
                                            match sv8 {
                                                0i32 => {
                                                    if ((((((c.m.r32(((p_ez_1).wrapping_add(560)).wrapping_add((((3i32) as u32)).wrapping_mul(4))) as i32)) == (7i32)) as i32)) != 0) {
                                                        { let v_ = 3i32; let a_ = ((p_ez_1).wrapping_add(6808)).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
                                                    } else {
                                                        { let v_ = (c.m.r32((p_ez_1).wrapping_add(632)) as i32); let a_ = ((p_ez_1).wrapping_add(6808)).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
                                                    }
                                                    F::Flash_GotoLabel(c, ((p_ez_1).wrapping_add(44)).wrapping_add((((2i32) as u32)).wrapping_mul(44)), sym::menu_v_b__str49, ((((1i32) as i64 as u64)) as u8));
                                                    F::EvoZ_ClipGoto(c, p_ez_1, 2i32, 3i32, 0i32, sym::menu_v_b__str50);
                                                    { let r_ = 128i32; let a_ = (p_ez_1).wrapping_add(556); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                                                    F::EvoZ_ClipGoto(c, p_ez_1, 2i32, 10i32, 1i32, sym::menu_v_b__str51);
                                                    { let v_ = (({ let a_ = (((((c.m.r32(sym::gSaveData)).wrapping_add(8)).wrapping_add(6144)).wrapping_add((((({ let a_ = c.m.r32((p_ez_1).wrapping_add(788)); (c.m.r32(a_) as i32) }).wrapping_add(({ let a_ = (c.m.r32((p_ez_1).wrapping_add(788))).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(7i32))) as u32)).wrapping_mul(56))).wrapping_add(((((c.m.r32((p_ez_1).wrapping_add(628)) as i32)) as u32)).wrapping_mul(16))).wrapping_add(((((c.m.r32(((p_ez_1).wrapping_add(560)).wrapping_add((((3i32) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(2)); c.m.r16(a_) }) as i32); let a_ = (p_ez_1).wrapping_add(636); c.m.w32(a_, (v_) as u32); };
                                                    { let v_ = 10i32; let a_ = (p_ez_1).wrapping_add(608); c.m.w32(a_, (v_) as u32); };
                                                    { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                                                    break 's6;
                                                }
                                                1i32 => {
                                                    F::EvoZ_ClipGoto(c, p_ez_1, 2i32, 3i32, 0i32, sym::menu_v_b__str52);
                                                    if ((((((({ let a_ = (((((c.m.r32(sym::gSaveData)).wrapping_add(8)).wrapping_add(6144)).wrapping_add((((({ let a_ = c.m.r32((p_ez_1).wrapping_add(788)); (c.m.r32(a_) as i32) }).wrapping_add(({ let a_ = (c.m.r32((p_ez_1).wrapping_add(788))).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(7i32))) as u32)).wrapping_mul(56))).wrapping_add(((((c.m.r32((p_ez_1).wrapping_add(628)) as i32)) as u32)).wrapping_mul(16))).wrapping_add(((((c.m.r32(((p_ez_1).wrapping_add(560)).wrapping_add((((3i32) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(2)); c.m.r16(a_) }) as i32)) != (0i32)) as i32)) != 0) {
                                                        F::ItemHelp_Open(c);
                                                        l_id_3 = (({ let a_ = (((((c.m.r32(sym::gSaveData)).wrapping_add(8)).wrapping_add(6144)).wrapping_add((((({ let a_ = c.m.r32((p_ez_1).wrapping_add(788)); (c.m.r32(a_) as i32) }).wrapping_add(({ let a_ = (c.m.r32((p_ez_1).wrapping_add(788))).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(7i32))) as u32)).wrapping_mul(56))).wrapping_add(((((c.m.r32((p_ez_1).wrapping_add(628)) as i32)) as u32)).wrapping_mul(16))).wrapping_add(((((c.m.r32(((p_ez_1).wrapping_add(560)).wrapping_add((((3i32) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(2)); c.m.r16(a_) }) as i32);
                                                        { let r_ = 8i32; let a_ = (p_ez_1).wrapping_add(556); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                                                        { let v_ = (l_id_3).wrapping_sub(1i32); let a_ = (p_ez_1).wrapping_add(620); c.m.w32(a_, (v_) as u32); };
                                                        { let r_ = (!(2i32)); let a_ = ((c.m.r32(sym::gSaveData)).wrapping_add(11976)).wrapping_add((((l_id_3).wrapping_sub(1i32)) as u32)); let n_ = ((((((c.m.r8(a_)) as i32)) & (r_))) as u8); c.m.w8(a_, n_); };
                                                        { let v_ = 9i32; let a_ = (p_ez_1).wrapping_add(608); c.m.w32(a_, (v_) as u32); };
                                                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                                                    } else {
                                                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 7i32); };
                                                    }
                                                    break 's6;
                                                }
                                                2i32 => {
                                                    F::EvoZ_ClipGoto(c, p_ez_1, 2i32, 3i32, 0i32, sym::menu_v_b__str53);
                                                    if ((((((({ let a_ = (((((c.m.r32(sym::gSaveData)).wrapping_add(8)).wrapping_add(6144)).wrapping_add((((({ let a_ = c.m.r32((p_ez_1).wrapping_add(788)); (c.m.r32(a_) as i32) }).wrapping_add(({ let a_ = (c.m.r32((p_ez_1).wrapping_add(788))).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(7i32))) as u32)).wrapping_mul(56))).wrapping_add(((((c.m.r32((p_ez_1).wrapping_add(628)) as i32)) as u32)).wrapping_mul(16))).wrapping_add(((((c.m.r32(((p_ez_1).wrapping_add(560)).wrapping_add((((3i32) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(2)); c.m.r16(a_) }) as i32)) != (0i32)) as i32)) != 0) {
                                                        { let r_ = 1i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(20); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                                                        F::EvoZ_RemoveItem(c, p_ez_1);
                                                        F::EvoZ_RefreshStatus(c, p_ez_1);
                                                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                                                    } else {
                                                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 7i32); };
                                                    }
                                                    break 's6;
                                                }
                                                3i32 => {
                                                    F::EvoZ_ClipGoto(c, p_ez_1, 2i32, 3i32, 0i32, sym::menu_v_b__str54);
                                                    if ((((((((((c.m.r32((p_ez_1).wrapping_add(6756)) as i32)) != (0i32)) as i32)) != 0) || ((((((({ let a_ = (((((c.m.r32(sym::gSaveData)).wrapping_add(8)).wrapping_add(6144)).wrapping_add((((({ let a_ = c.m.r32((p_ez_1).wrapping_add(788)); (c.m.r32(a_) as i32) }).wrapping_add(({ let a_ = (c.m.r32((p_ez_1).wrapping_add(788))).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(7i32))) as u32)).wrapping_mul(56))).wrapping_add(((((c.m.r32((p_ez_1).wrapping_add(628)) as i32)) as u32)).wrapping_mul(16))).wrapping_add((((7i32) as u32)).wrapping_mul(2)); c.m.r16(a_) }) as i32)) != (0i32)) as i32)) != 0)) as i32)) != 0) {
                                                        { let v_ = 1i32; let a_ = (p_ez_1).wrapping_add(644); c.m.w32(a_, (v_) as u32); };
                                                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                                                    } else {
                                                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 7i32); };
                                                    }
                                                    break 's6;
                                                }
                                                _ => {
                                                }
                                            }
                                        }
                                    } else {
                                        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1024i32) as u32)))) != 0) {
                                            F::Flash_GotoLabel(c, ((p_ez_1).wrapping_add(44)).wrapping_add((((0i32) as u32)).wrapping_mul(44)), sym::menu_v_b__str55, ((((1i32) as i64 as u64)) as u8));
                                            F::Flash_GotoLabel(c, ((p_ez_1).wrapping_add(44)).wrapping_add((((2i32) as u32)).wrapping_mul(44)), sym::menu_v_b__str56, ((((1i32) as i64 as u64)) as u8));
                                            F::EvoZ_ClipGoto(c, p_ez_1, 0i32, 1i32, 0i32, sym::menu_v_b__str57);
                                            F::EvoZ_ClipGoto(c, p_ez_1, 0i32, 2i32, 0i32, sym::menu_v_b__str58);
                                            F::EvoZ_ClipGoto(c, p_ez_1, 2i32, 3i32, 0i32, sym::menu_v_b__str59);
                                            { let v_ = 1i32; let a_ = (p_ez_1).wrapping_add(608); c.m.w32(a_, (v_) as u32); };
                                            { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 2i32); };
                                        }
                                    }
                                }
                            }
                        }
                    }
                    break 's1;
                }
                4i32 => {
                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1024i32) as u32)))) != 0) {
                        F::PassWin_Close(c);
                        F::EvoZ_ClipGoto(c, p_ez_1, 0i32, 1i32, 0i32, sym::menu_v_b__str60);
                        F::EvoZ_ClipGoto(c, p_ez_1, 0i32, 2i32, 0i32, sym::menu_v_b__str61);
                        { let v_ = 1i32; let a_ = (p_ez_1).wrapping_add(608); c.m.w32(a_, (v_) as u32); };
                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 2i32); };
                    }
                    break 's1;
                }
                5i32 => {
                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((8i32) as u32)))) != 0) {
                        if ((((((((((c.m.r32((p_ez_1).wrapping_add(624)) as i32)) < ({ let a_ = ((c.m.r32(sym::gSaveData)).wrapping_add(8)).wrapping_add(12320); (c.m.r32(a_) as i32) })) as i32)) != 0) && ((((((c.m.r32((p_ez_1).wrapping_add(624)) as i32)) < ((c.m.r32((p_ez_1).wrapping_add(6772)) as i32))) as i32)) != 0)) as i32)) != 0) {
                            if ((((({ let a_ = ((p_ez_1).wrapping_add(560)).wrapping_add(((((c.m.r32((p_ez_1).wrapping_add(608)) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
                                { let r_ = { let a1_ = { let a_ = ((p_ez_1).wrapping_add(560)).wrapping_add(((((c.m.r32((p_ez_1).wrapping_add(608)) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; F::EvoZ_Pow(c, 10i32, a1_) }; let a_ = (p_ez_1).wrapping_add(624); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                            } else {
                                { let a_ = (p_ez_1).wrapping_add(624); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                            }
                            if ((((({ let a_ = ((c.m.r32(sym::gSaveData)).wrapping_add(8)).wrapping_add(12320); (c.m.r32(a_) as i32) }) < ((c.m.r32((p_ez_1).wrapping_add(624)) as i32))) as i32)) != 0) {
                                { let v_ = { let a_ = ((c.m.r32(sym::gSaveData)).wrapping_add(8)).wrapping_add(12320); (c.m.r32(a_) as i32) }; let a_ = (p_ez_1).wrapping_add(624); c.m.w32(a_, (v_) as u32); };
                            }
                            if ((((((c.m.r32((p_ez_1).wrapping_add(6772)) as i32)) < ((c.m.r32((p_ez_1).wrapping_add(624)) as i32))) as i32)) != 0) {
                                { let v_ = (c.m.r32((p_ez_1).wrapping_add(6772)) as i32); let a_ = (p_ez_1).wrapping_add(624); c.m.w32(a_, (v_) as u32); };
                            }
                            { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 43i32); };
                        }
                    } else {
                        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((4i32) as u32)))) != 0) {
                            if ((((((c.m.r32((p_ez_1).wrapping_add(624)) as i32)) > (0i32)) as i32)) != 0) {
                                if ((((({ let a_ = ((p_ez_1).wrapping_add(560)).wrapping_add(((((c.m.r32((p_ez_1).wrapping_add(608)) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
                                    { let r_ = { let a1_ = { let a_ = ((p_ez_1).wrapping_add(560)).wrapping_add(((((c.m.r32((p_ez_1).wrapping_add(608)) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; F::EvoZ_Pow(c, 10i32, a1_) }; let a_ = (p_ez_1).wrapping_add(624); let n_ = ((c.m.r32(a_) as i32)).wrapping_sub(r_); c.m.w32(a_, (n_) as u32); };
                                } else {
                                    { let a_ = (p_ez_1).wrapping_add(624); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                                }
                                if ((((((c.m.r32((p_ez_1).wrapping_add(624)) as i32)) < (0i32)) as i32)) != 0) {
                                    { let v_ = 0i32; let a_ = (p_ez_1).wrapping_add(624); c.m.w32(a_, (v_) as u32); };
                                }
                                { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 43i32); };
                            }
                        } else {
                            if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((1i32) as u32)))) != 0) {
                                F::EvoZ_ClipGoto(c, p_ez_1, 0i32, 5i32, 0i32, sym::menu_v_b__str62);
                                { let a_ = ((p_ez_1).wrapping_add(560)).wrapping_add(((((c.m.r32((p_ez_1).wrapping_add(608)) as i32)) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                                if ((((({ let a_ = ((p_ez_1).wrapping_add(560)).wrapping_add(((((c.m.r32((p_ez_1).wrapping_add(608)) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) >= (5i32)) as i32)) != 0) {
                                    { let v_ = 0i32; let a_ = ((p_ez_1).wrapping_add(560)).wrapping_add(((((c.m.r32((p_ez_1).wrapping_add(608)) as i32)) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                }
                                F::EvoZ_ClipGoto(c, p_ez_1, 0i32, 5i32, 0i32, sym::menu_v_b__str63);
                                { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 44i32); };
                            } else {
                                if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((2i32) as u32)))) != 0) {
                                    F::EvoZ_ClipGoto(c, p_ez_1, 0i32, 5i32, 0i32, sym::menu_v_b__str64);
                                    { let a_ = ((p_ez_1).wrapping_add(560)).wrapping_add(((((c.m.r32((p_ez_1).wrapping_add(608)) as i32)) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                                    if ((((({ let a_ = ((p_ez_1).wrapping_add(560)).wrapping_add(((((c.m.r32((p_ez_1).wrapping_add(608)) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) < (0i32)) as i32)) != 0) {
                                        { let v_ = 4i32; let a_ = ((p_ez_1).wrapping_add(560)).wrapping_add(((((c.m.r32((p_ez_1).wrapping_add(608)) as i32)) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                    }
                                    F::EvoZ_ClipGoto(c, p_ez_1, 0i32, 5i32, 0i32, sym::menu_v_b__str65);
                                    { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 44i32); };
                                } else {
                                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                                        if ((((((c.m.r32((p_ez_1).wrapping_add(624)) as i32)) != (0i32)) as i32)) != 0) {
                                            { let v_ = 5i32; let a_ = (p_ez_1).wrapping_add(644); c.m.w32(a_, (v_) as u32); };
                                            { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 45i32); };
                                        } else {
                                            { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 7i32); };
                                        }
                                    } else {
                                        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1024i32) as u32)))) != 0) {
                                            F::Flash_GotoLabel(c, ((p_ez_1).wrapping_add(44)).wrapping_add((((0i32) as u32)).wrapping_mul(44)), sym::menu_v_b__str66, ((((1i32) as i64 as u64)) as u8));
                                            F::EvoZ_ClipGoto(c, p_ez_1, 0i32, 5i32, 0i32, sym::menu_v_b__str67);
                                            F::EvoZ_ClipGoto(c, p_ez_1, 0i32, 1i32, 0i32, sym::menu_v_b__str68);
                                            F::EvoZ_ClipGoto(c, p_ez_1, 0i32, 2i32, 0i32, sym::menu_v_b__str69);
                                            { let v_ = (c.m.r32((p_ez_1).wrapping_add(628)) as i32); let a_ = ((p_ez_1).wrapping_add(560)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                            { let v_ = 1i32; let a_ = (p_ez_1).wrapping_add(608); c.m.w32(a_, (v_) as u32); };
                                            { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 2i32); };
                                        }
                                    }
                                }
                            }
                        }
                    }
                    break 's1;
                }
                6i32 => {
                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                        { let r_ = (c.m.r32((p_ez_1).wrapping_add(624)) as i32); let a_ = ((c.m.r32(sym::gSaveData)).wrapping_add(8)).wrapping_add(12320); let n_ = ((c.m.r32(a_) as i32)).wrapping_sub(r_); c.m.w32(a_, (n_) as u32); };
                        { let r_ = (c.m.r32((p_ez_1).wrapping_add(624)) as i32); let a_ = ((((c.m.r32(sym::gSaveData)).wrapping_add(8)).wrapping_add(6144)).wrapping_add((((({ let a_ = c.m.r32((p_ez_1).wrapping_add(788)); (c.m.r32(a_) as i32) }).wrapping_add(({ let a_ = (c.m.r32((p_ez_1).wrapping_add(788))).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(7i32))) as u32)).wrapping_mul(56))).wrapping_add(48); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                        { let v_ = { let a_ = ((((c.m.r32(sym::gSaveData)).wrapping_add(8)).wrapping_add(6144)).wrapping_add((((({ let a_ = c.m.r32((p_ez_1).wrapping_add(788)); (c.m.r32(a_) as i32) }).wrapping_add(({ let a_ = (c.m.r32((p_ez_1).wrapping_add(788))).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(7i32))) as u32)).wrapping_mul(56))).wrapping_add(48); (c.m.r32(a_) as i32) }; let a_ = (p_ez_1).wrapping_add(6764); c.m.w32(a_, (v_) as u32); };
                        { let r_ = 32i32; let a_ = (p_ez_1).wrapping_add(556); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
                        { let v_ = 0i32; let a_ = (p_ez_1).wrapping_add(624); c.m.w32(a_, (v_) as u32); };
                        if ((((((c.m.r32((p_ez_1).wrapping_add(6764)) as i32)) >= ((c.m.r32((p_ez_1).wrapping_add(6768)) as i32))) as i32)) != 0) {
                            l_f_4 = ((p_ez_1).wrapping_add(44)).wrapping_add((((0i32) as u32)).wrapping_mul(44));
                            F::Flash_FindLabel(c, l_f_4, ((0i32) as u32), sym::menu_v_b__str70, fp);
                            F::Flash_ClipGotoLabel(c, l_f_4, fp, sym::menu_v_b__str71);
                            { let r_ = 64i32; let a_ = (p_ez_1).wrapping_add(556); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                            'l7: loop {
                                'c8: {
                                    { let a_ = (((c.m.r32(sym::gSaveData)).wrapping_add(6152)).wrapping_add((((({ let a_ = c.m.r32((p_ez_1).wrapping_add(788)); (c.m.r32(a_) as i32) }).wrapping_add(({ let a_ = (c.m.r32((p_ez_1).wrapping_add(788))).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(7i32))) as u32)).wrapping_mul(56))).wrapping_add(52); let o_ = c.m.r16(a_); let n_ = (o_).wrapping_add(1); c.m.w16(a_, n_); };
                                    { let v_ = { let a0_ = { let a_ = (c.m.r32((p_ez_1).wrapping_add(788))).wrapping_add(108); (c.m.r32(a_) as i32) }; let a1_ = ({ let a_ = c.m.r32((p_ez_1).wrapping_add(788)); (c.m.r32(a_) as i32) }).wrapping_add(({ let a_ = (c.m.r32((p_ez_1).wrapping_add(788))).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(7i32)); F::ChrTbl_GetExp(c, a0_, a1_) }; let a_ = (p_ez_1).wrapping_add(6768); c.m.w32(a_, (v_) as u32); };
                                }
                                if !((((((((((c.m.r32((p_ez_1).wrapping_add(6764)) as i32)) >= ((c.m.r32((p_ez_1).wrapping_add(6768)) as i32))) as i32)) != 0) && ((((((c.m.r32((p_ez_1).wrapping_add(6768)) as i32)) != (0i32)) as i32)) != 0)) as i32)) != 0) { break 'l7; }
                            }
                            { let v_ = 7i32; let a_ = (p_ez_1).wrapping_add(608); c.m.w32(a_, (v_) as u32); };
                            { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 47i32); };
                        } else {
                            { let v_ = 5i32; let a_ = (p_ez_1).wrapping_add(608); c.m.w32(a_, (v_) as u32); };
                            { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                        }
                        F::EvoZ_RefreshStatus(c, p_ez_1);
                    }
                    break 's1;
                }
                9i32 => {
                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                        if (((((c.m.r32((p_ez_1).wrapping_add(556)) as i32)) & (8i32))) != 0) {
                            { let r_ = 8i32; let a_ = (p_ez_1).wrapping_add(556); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
                            F::ItemHelp_Close(c);
                            F::EvoZ_ClipGoto(c, p_ez_1, 2i32, 3i32, 0i32, sym::menu_v_b__str72);
                            { let v_ = 3i32; let a_ = (p_ez_1).wrapping_add(608); c.m.w32(a_, (v_) as u32); };
                            { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 2i32); };
                        } else {
                            if (({ let a1_ = (c.m.r32((p_ez_1).wrapping_add(620)) as i32); F::EvoZ_CanEquip(c, p_ez_1, a1_) }) != 0) {
                                { let v_ = ((({ let a_ = ((((p_ez_1).wrapping_add(6808)).wrapping_add(48)).wrapping_add(((((c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(16)) as i32)) as u32)).wrapping_mul(1400))).wrapping_add(((({ let a_ = ((p_ez_1).wrapping_add(6808)).wrapping_add(((((c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(16)) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(1i32)) as u16); let a_ = (((((c.m.r32(sym::gSaveData)).wrapping_add(8)).wrapping_add(6144)).wrapping_add((((({ let a_ = c.m.r32((p_ez_1).wrapping_add(788)); (c.m.r32(a_) as i32) }).wrapping_add(({ let a_ = (c.m.r32((p_ez_1).wrapping_add(788))).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(7i32))) as u32)).wrapping_mul(56))).wrapping_add(((((c.m.r32((p_ez_1).wrapping_add(628)) as i32)) as u32)).wrapping_mul(16))).wrapping_add(((((c.m.r32(((p_ez_1).wrapping_add(560)).wrapping_add((((3i32) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(2)); c.m.w16(a_, v_); };
                                { let v_ = (({ let a_ = (((((c.m.r32(sym::gSaveData)).wrapping_add(8)).wrapping_add(6144)).wrapping_add((((({ let a_ = c.m.r32((p_ez_1).wrapping_add(788)); (c.m.r32(a_) as i32) }).wrapping_add(({ let a_ = (c.m.r32((p_ez_1).wrapping_add(788))).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(7i32))) as u32)).wrapping_mul(56))).wrapping_add(((((c.m.r32((p_ez_1).wrapping_add(628)) as i32)) as u32)).wrapping_mul(16))).wrapping_add(((((c.m.r32(((p_ez_1).wrapping_add(560)).wrapping_add((((3i32) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(2)); c.m.r16(a_) }) as i32); let a_ = (p_ez_1).wrapping_add(636); c.m.w32(a_, (v_) as u32); };
                                { let r_ = 1i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(20); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                                F::EvoZ_RefreshStatus(c, p_ez_1);
                                F::ItemHelp_Close(c);
                                F::Flash_GotoLabel(c, ((p_ez_1).wrapping_add(44)).wrapping_add((((2i32) as u32)).wrapping_mul(44)), sym::menu_v_b__str73, ((((1i32) as i64 as u64)) as u8));
                                { let r_ = 128i32; let a_ = (p_ez_1).wrapping_add(556); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                                F::EvoZ_ClipGoto(c, p_ez_1, 2i32, 10i32, 0i32, sym::menu_v_b__str74);
                                F::EvoZ_ClipGoto(c, p_ez_1, 2i32, 10i32, 1i32, sym::menu_v_b__str75);
                                { let v_ = 3i32; let a_ = (p_ez_1).wrapping_add(608); c.m.w32(a_, (v_) as u32); };
                                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                            } else {
                                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 7i32); };
                            }
                        }
                    } else {
                        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1024i32) as u32)))) != 0) {
                            F::ItemHelp_Close(c);
                            if (((((c.m.r32((p_ez_1).wrapping_add(556)) as i32)) & (8i32))) != 0) {
                                F::EvoZ_ClipGoto(c, p_ez_1, 2i32, 3i32, 0i32, sym::menu_v_b__str76);
                                { let r_ = 8i32; let a_ = (p_ez_1).wrapping_add(556); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
                                { let v_ = 3i32; let a_ = (p_ez_1).wrapping_add(608); c.m.w32(a_, (v_) as u32); };
                            } else {
                                F::EvoZ_ClipGoto(c, p_ez_1, 2i32, 10i32, 0i32, sym::menu_v_b__str77);
                                { let v_ = 10i32; let a_ = (p_ez_1).wrapping_add(608); c.m.w32(a_, (v_) as u32); };
                            }
                            { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 2i32); };
                        }
                    }
                    break 's1;
                }
                10i32 => {
                    if (((if ((((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(384)) as i32)) != (1i32)) as i32)) != 0) { ((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((16384i32) as u32))) } else { ((((((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((8i32) as u32)))) != 0) && ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(392))) & (((2048i32) as u32)))) != 0)) as i32)) as u32) })) != 0) {
                        if ((((({ let a_ = ((p_ez_1).wrapping_add(6808)).wrapping_add(((((c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(16)) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) > (0i32)) as i32)) != 0) {
                            F::EvoZ_ClipGoto(c, p_ez_1, 2i32, 10i32, 0i32, sym::menu_v_b__str78);
                            { let r_ = 6i32; let a_ = ((p_ez_1).wrapping_add(6808)).wrapping_add(((((c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(16)) as i32)) as u32)).wrapping_mul(4)); let n_ = ((c.m.r32(a_) as i32)).wrapping_sub(r_); c.m.w32(a_, (n_) as u32); };
                            if ((((({ let a_ = ((p_ez_1).wrapping_add(6808)).wrapping_add(((((c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(16)) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) < ({ let a_ = (((p_ez_1).wrapping_add(6808)).wrapping_add(5648)).wrapping_add(((((c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(16)) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                                if ((((({ let a_ = ((p_ez_1).wrapping_add(6808)).wrapping_add(((((c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(16)) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) < (6i32)) as i32)) != 0) {
                                    { let v_ = { let a_ = (((p_ez_1).wrapping_add(6808)).wrapping_add(5648)).wrapping_add(((((c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(16)) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ((p_ez_1).wrapping_add(6808)).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                                    { let v_ = 0i32; let a_ = ((p_ez_1).wrapping_add(6808)).wrapping_add(((((c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(16)) as i32)) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                    { let v_ = 0i32; let a_ = (((p_ez_1).wrapping_add(6808)).wrapping_add(5648)).wrapping_add(((((c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(16)) as i32)) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                    { let v_ = 5i32; let a_ = (((p_ez_1).wrapping_add(6808)).wrapping_add(5664)).wrapping_add(((((c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(16)) as i32)) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                } else {
                                    { let v_ = { let a_ = (((p_ez_1).wrapping_add(6808)).wrapping_add(5648)).wrapping_add(((((c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(16)) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ((p_ez_1).wrapping_add(6808)).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                                    { let r_ = 6i32; let a_ = (((p_ez_1).wrapping_add(6808)).wrapping_add(5648)).wrapping_add(((((c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(16)) as i32)) as u32)).wrapping_mul(4)); let n_ = ((c.m.r32(a_) as i32)).wrapping_sub(r_); c.m.w32(a_, (n_) as u32); };
                                    { let r_ = 6i32; let a_ = (((p_ez_1).wrapping_add(6808)).wrapping_add(5664)).wrapping_add(((((c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(16)) as i32)) as u32)).wrapping_mul(4)); let n_ = ((c.m.r32(a_) as i32)).wrapping_sub(r_); c.m.w32(a_, (n_) as u32); };
                                }
                            }
                            F::EvoZ_ClipGoto(c, p_ez_1, 2i32, 10i32, 0i32, sym::menu_v_b__str79);
                            { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                        }
                    } else {
                        if (((if ((((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(384)) as i32)) != (1i32)) as i32)) != 0) { ((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((65536i32) as u32))) } else { ((((((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((4i32) as u32)))) != 0) && ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(392))) & (((2048i32) as u32)))) != 0)) as i32)) as u32) })) != 0) {
                            if ((((({ let a_ = ((p_ez_1).wrapping_add(6808)).wrapping_add(((((c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(16)) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) < (({ let a_ = (((p_ez_1).wrapping_add(6808)).wrapping_add(32)).wrapping_add(((((c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(16)) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_sub(1i32))) as i32)) != 0) {
                                F::EvoZ_ClipGoto(c, p_ez_1, 2i32, 10i32, 0i32, sym::menu_v_b__str80);
                                { let r_ = 6i32; let a_ = ((p_ez_1).wrapping_add(6808)).wrapping_add(((((c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(16)) as i32)) as u32)).wrapping_mul(4)); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                                if ((((({ let a_ = (((p_ez_1).wrapping_add(6808)).wrapping_add(5664)).wrapping_add(((((c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(16)) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) < ({ let a_ = ((p_ez_1).wrapping_add(6808)).wrapping_add(((((c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(16)) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                                    if (((((({ let a_ = (((p_ez_1).wrapping_add(6808)).wrapping_add(32)).wrapping_add(((((c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(16)) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_sub(6i32)) < ({ let a_ = ((p_ez_1).wrapping_add(6808)).wrapping_add(((((c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(16)) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                                        { let v_ = ({ let a_ = (((p_ez_1).wrapping_add(6808)).wrapping_add(32)).wrapping_add(((((c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(16)) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_sub(1i32); let a_ = ((p_ez_1).wrapping_add(6808)).wrapping_add(((((c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(16)) as i32)) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                        { let v_ = { let a_ = (((p_ez_1).wrapping_add(6808)).wrapping_add(5664)).wrapping_add(((((c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(16)) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ((p_ez_1).wrapping_add(6808)).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                                        { let v_ = ({ let a_ = (((p_ez_1).wrapping_add(6808)).wrapping_add(32)).wrapping_add(((((c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(16)) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_sub(6i32); let a_ = (((p_ez_1).wrapping_add(6808)).wrapping_add(5648)).wrapping_add(((((c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(16)) as i32)) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                        { let v_ = ({ let a_ = (((p_ez_1).wrapping_add(6808)).wrapping_add(32)).wrapping_add(((((c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(16)) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_sub(1i32); let a_ = (((p_ez_1).wrapping_add(6808)).wrapping_add(5664)).wrapping_add(((((c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(16)) as i32)) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                    } else {
                                        { let v_ = { let a_ = (((p_ez_1).wrapping_add(6808)).wrapping_add(5664)).wrapping_add(((((c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(16)) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ((p_ez_1).wrapping_add(6808)).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                                        { let r_ = 6i32; let a_ = (((p_ez_1).wrapping_add(6808)).wrapping_add(5648)).wrapping_add(((((c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(16)) as i32)) as u32)).wrapping_mul(4)); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                                        { let r_ = 6i32; let a_ = (((p_ez_1).wrapping_add(6808)).wrapping_add(5664)).wrapping_add(((((c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(16)) as i32)) as u32)).wrapping_mul(4)); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                                    }
                                }
                                F::EvoZ_ClipGoto(c, p_ez_1, 2i32, 10i32, 0i32, sym::menu_v_b__str81);
                                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                            }
                        } else {
                            if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((8i32) as u32)))) != 0) {
                                if ((((({ let a_ = ((p_ez_1).wrapping_add(6808)).wrapping_add(((((c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(16)) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) > (0i32)) as i32)) != 0) {
                                    F::EvoZ_ClipGoto(c, p_ez_1, 2i32, 10i32, 0i32, sym::menu_v_b__str82);
                                    { let a_ = ((p_ez_1).wrapping_add(6808)).wrapping_add(((((c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(16)) as i32)) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                                    if ((((({ let a_ = ((p_ez_1).wrapping_add(6808)).wrapping_add(((((c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(16)) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) < ({ let a_ = (((p_ez_1).wrapping_add(6808)).wrapping_add(5648)).wrapping_add(((((c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(16)) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                                        F::Flash_GotoLabel(c, ((p_ez_1).wrapping_add(44)).wrapping_add((((2i32) as u32)).wrapping_mul(44)), sym::menu_v_b__str83, ((((1i32) as i64 as u64)) as u8));
                                        { let v_ = { let a_ = (((p_ez_1).wrapping_add(6808)).wrapping_add(5664)).wrapping_add(((((c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(16)) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ((p_ez_1).wrapping_add(6808)).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                                        { let a_ = (((p_ez_1).wrapping_add(6808)).wrapping_add(5648)).wrapping_add(((((c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(16)) as i32)) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                                        { let a_ = (((p_ez_1).wrapping_add(6808)).wrapping_add(5664)).wrapping_add(((((c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(16)) as i32)) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                                    }
                                    F::EvoZ_ClipGoto(c, p_ez_1, 2i32, 10i32, 0i32, sym::menu_v_b__str84);
                                    { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                                }
                            } else {
                                if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((4i32) as u32)))) != 0) {
                                    if ((((({ let a_ = ((p_ez_1).wrapping_add(6808)).wrapping_add(((((c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(16)) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) < (({ let a_ = (((p_ez_1).wrapping_add(6808)).wrapping_add(32)).wrapping_add(((((c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(16)) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_sub(1i32))) as i32)) != 0) {
                                        F::EvoZ_ClipGoto(c, p_ez_1, 2i32, 10i32, 0i32, sym::menu_v_b__str85);
                                        { let a_ = ((p_ez_1).wrapping_add(6808)).wrapping_add(((((c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(16)) as i32)) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                                        if ((((({ let a_ = ((p_ez_1).wrapping_add(6808)).wrapping_add(((((c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(16)) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) > ({ let a_ = (((p_ez_1).wrapping_add(6808)).wrapping_add(5664)).wrapping_add(((((c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(16)) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                                            F::Flash_GotoLabel(c, ((p_ez_1).wrapping_add(44)).wrapping_add((((2i32) as u32)).wrapping_mul(44)), sym::menu_v_b__str86, ((((1i32) as i64 as u64)) as u8));
                                            { let v_ = { let a_ = (((p_ez_1).wrapping_add(6808)).wrapping_add(5648)).wrapping_add(((((c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(16)) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ((p_ez_1).wrapping_add(6808)).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                                            { let a_ = (((p_ez_1).wrapping_add(6808)).wrapping_add(5648)).wrapping_add(((((c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(16)) as i32)) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                                            { let a_ = (((p_ez_1).wrapping_add(6808)).wrapping_add(5664)).wrapping_add(((((c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(16)) as i32)) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                                        }
                                        F::EvoZ_ClipGoto(c, p_ez_1, 2i32, 10i32, 0i32, sym::menu_v_b__str87);
                                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                                    }
                                } else {
                                    if ((((((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((1i32) as u32)))) != 0) && ((((((c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(16)) as i32)) != (3i32)) as i32)) != 0)) as i32)) != 0) {
                                        F::EvoZ_ClipGoto(c, p_ez_1, 2i32, 10i32, 0i32, sym::menu_v_b__str88);
                                        F::EvoZ_ClipGoto(c, p_ez_1, 2i32, 10i32, 1i32, sym::menu_v_b__str89);
                                        { let a_ = ((p_ez_1).wrapping_add(6808)).wrapping_add(16); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                                        if ((((((c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(16)) as i32)) < (0i32)) as i32)) != 0) {
                                            { let v_ = 2i32; let a_ = ((p_ez_1).wrapping_add(6808)).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
                                        }
                                        F::EvoZ_ClipGoto(c, p_ez_1, 2i32, 10i32, 0i32, sym::menu_v_b__str90);
                                        F::EvoZ_ClipGoto(c, p_ez_1, 2i32, 10i32, 1i32, sym::menu_v_b__str91);
                                        { let v_ = (c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(16)) as i32); let a_ = (p_ez_1).wrapping_add(632); c.m.w32(a_, (v_) as u32); };
                                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                                    } else {
                                        if ((((((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((2i32) as u32)))) != 0) && ((((((c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(16)) as i32)) != (3i32)) as i32)) != 0)) as i32)) != 0) {
                                            F::EvoZ_ClipGoto(c, p_ez_1, 2i32, 10i32, 0i32, sym::menu_v_b__str92);
                                            F::EvoZ_ClipGoto(c, p_ez_1, 2i32, 10i32, 1i32, sym::menu_v_b__str93);
                                            { let a_ = ((p_ez_1).wrapping_add(6808)).wrapping_add(16); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                                            if ((((((c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(16)) as i32)) >= (3i32)) as i32)) != 0) {
                                                { let v_ = 0i32; let a_ = ((p_ez_1).wrapping_add(6808)).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
                                            }
                                            F::EvoZ_ClipGoto(c, p_ez_1, 2i32, 10i32, 0i32, sym::menu_v_b__str94);
                                            F::EvoZ_ClipGoto(c, p_ez_1, 2i32, 10i32, 1i32, sym::menu_v_b__str95);
                                            { let v_ = (c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(16)) as i32); let a_ = (p_ez_1).wrapping_add(632); c.m.w32(a_, (v_) as u32); };
                                            { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                                        } else {
                                            if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                                                F::EvoZ_ClipGoto(c, p_ez_1, 2i32, 10i32, 0i32, sym::menu_v_b__str96);
                                                if (((((((({ let a_ = (((c.m.r32(sym::gSaveData)).wrapping_add(8)).wrapping_add(11968)).wrapping_add((({ let a_ = ((((p_ez_1).wrapping_add(6808)).wrapping_add(48)).wrapping_add(((((c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(16)) as i32)) as u32)).wrapping_mul(1400))).wrapping_add(((({ let a_ = ((p_ez_1).wrapping_add(6808)).wrapping_add(((((c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(16)) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)); c.m.r8(a_) }) as i32)) & (1i32))) as u8)) != 0) {
                                                    F::ItemHelp_Open(c);
                                                    l_id_5 = { let a_ = ((((p_ez_1).wrapping_add(6808)).wrapping_add(48)).wrapping_add(((((c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(16)) as i32)) as u32)).wrapping_mul(1400))).wrapping_add(((({ let a_ = ((p_ez_1).wrapping_add(6808)).wrapping_add(((((c.m.r32(((p_ez_1).wrapping_add(6808)).wrapping_add(16)) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
                                                    { let v_ = l_id_5; let a_ = (p_ez_1).wrapping_add(620); c.m.w32(a_, (v_) as u32); };
                                                    { let r_ = (!(2i32)); let a_ = ((c.m.r32(sym::gSaveData)).wrapping_add(11976)).wrapping_add(((l_id_5) as u32)); let n_ = ((((((c.m.r8(a_)) as i32)) & (r_))) as u8); c.m.w8(a_, n_); };
                                                    { let v_ = 9i32; let a_ = (p_ez_1).wrapping_add(608); c.m.w32(a_, (v_) as u32); };
                                                    { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                                                } else {
                                                    { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 7i32); };
                                                }
                                            } else {
                                                if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1024i32) as u32)))) != 0) {
                                                    F::Flash_GotoLabel(c, ((p_ez_1).wrapping_add(44)).wrapping_add((((2i32) as u32)).wrapping_mul(44)), sym::menu_v_b__str97, ((((1i32) as i64 as u64)) as u8));
                                                    { let r_ = 128i32; let a_ = (p_ez_1).wrapping_add(556); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                                                    F::EvoZ_ClipGoto(c, p_ez_1, 2i32, 10i32, 0i32, sym::menu_v_b__str98);
                                                    F::EvoZ_ClipGoto(c, p_ez_1, 2i32, 10i32, 1i32, sym::menu_v_b__str99);
                                                    { let v_ = 3i32; let a_ = (p_ez_1).wrapping_add(608); c.m.w32(a_, (v_) as u32); };
                                                    { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 2i32); };
                                                }
                                            }
                                        }
                                    }
                                }
                            }
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
    if rt::probing() { crate::host::harness::probe(c, "EvoZ_Input", &PROBE_N, &[], true); }
    r_
}

pub fn EvoZ_UpdateDialog(c: &mut Ctx, mut p_ez_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EvoZ_UpdateDialog", &PROBE_N, &[(p_ez_1) as u32], false); }
    let mut l_d_2: u32 = 0;
    let r_ = 'f: {
    l_d_2 = (p_ez_1).wrapping_add(644);
    if ((((((c.m.r32(l_d_2) as i32)) == (0i32)) as i32)) != 0) {
        break 'f;
    }
    's1: {
        let sv3: i32 = (c.m.r32(l_d_2) as i32);
        'k8: {
        'k7: {
        'k6: {
        'k5: {
        'k4: {
        'k3: {
        'k2: {
            match sv3 { 1i32 => break 'k2, 2i32 => break 'k3, 3i32 => break 'k4, 5i32 => break 'k5, 6i32 => break 'k6, 7i32 => break 'k7, 4i32 => break 'k8, 8i32 => break 'k8, _ => break 's1, }
        }
        { let v_ = 2i32; let a_ = l_d_2; c.m.w32(a_, (v_) as u32); };
        F::Dialog_SetLayout(c, 0i32);
        F::Dialog_Start(c, 0i32);
        F::Dialog_SetCursor(c, 1i32);
        { let v_ = 0i32; let a_ = (l_d_2).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        }
        F::Dialog_SetChoices(c, 1i32);
        F::Dialog_Start(c, 0i32);
        { let v_ = 1i32; let a_ = (l_d_2).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        if ((((((c.m.r32((l_d_2).wrapping_add(4)) as i32)) > (0i32)) as i32)) != 0) {
            { let a_ = (l_d_2).wrapping_add(8); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
            if ((((((c.m.r32((l_d_2).wrapping_add(8)) as i32)) < (0i32)) as i32)) != 0) {
                { let r_ = 1i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(20); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                F::EvoZ_RemoveAllItems(c, p_ez_1);
                F::EvoZ_RefreshStatus(c, p_ez_1);
                F::EvoZ_ClipGoto(c, p_ez_1, 2i32, 3i32, 0i32, sym::menu_v_b__str100);
                { let v_ = 0i32; let a_ = ((p_ez_1).wrapping_add(560)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                F::EvoZ_ClipGoto(c, p_ez_1, 2i32, 3i32, 0i32, sym::menu_v_b__str101);
                { let v_ = 120i32; let a_ = (l_d_2).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                { let v_ = 3i32; let a_ = l_d_2; c.m.w32(a_, (v_) as u32); };
            }
        } else {
            if ((((((c.m.r32((l_d_2).wrapping_add(4)) as i32)) < (0i32)) as i32)) != 0) {
                F::Dialog_SetCursor(c, 1i32);
                { let v_ = 120i32; let a_ = (l_d_2).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                { let v_ = 3i32; let a_ = l_d_2; c.m.w32(a_, (v_) as u32); };
            } else {
                { let v_ = F::Dialog_Input(c, 1i32); let a_ = (l_d_2).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                { let v_ = 12i32; let a_ = (l_d_2).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
            }
        }
        break 's1;
        }
        F::Dialog_SetChoices(c, 0i32);
        F::Dialog_Start(c, 1i32);
        { let _ = F::Dialog_Input(c, 0i32); };
        { let v_ = 4i32; let a_ = l_d_2; c.m.w32(a_, (v_) as u32); };
        break 's1;
        }
        { let v_ = 6i32; let a_ = l_d_2; c.m.w32(a_, (v_) as u32); };
        F::Dialog_SetLayout(c, 0i32);
        F::Dialog_Start(c, 0i32);
        F::Dialog_SetCursor(c, 1i32);
        { let v_ = 0i32; let a_ = (l_d_2).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        }
        F::Dialog_SetChoices(c, 1i32);
        F::Dialog_Start(c, 0i32);
        { let v_ = 0i32; let a_ = (l_d_2).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        if ((((((c.m.r32((l_d_2).wrapping_add(4)) as i32)) > (0i32)) as i32)) != 0) {
            { let a_ = (l_d_2).wrapping_add(8); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
            if ((((((c.m.r32((l_d_2).wrapping_add(8)) as i32)) < (0i32)) as i32)) != 0) {
                { let r_ = 1i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(20); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 6i32; let a_ = (p_ez_1).wrapping_add(608); c.m.w32(a_, (v_) as u32); };
                { let r_ = 32i32; let a_ = (p_ez_1).wrapping_add(556); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 120i32; let a_ = (l_d_2).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                { let v_ = 7i32; let a_ = l_d_2; c.m.w32(a_, (v_) as u32); };
            }
        } else {
            if ((((((c.m.r32((l_d_2).wrapping_add(4)) as i32)) < (0i32)) as i32)) != 0) {
                F::Dialog_SetCursor(c, 1i32);
                { let v_ = 120i32; let a_ = (l_d_2).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                { let v_ = 7i32; let a_ = l_d_2; c.m.w32(a_, (v_) as u32); };
            } else {
                { let v_ = F::Dialog_Input(c, 1i32); let a_ = (l_d_2).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                { let v_ = 12i32; let a_ = (l_d_2).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
            }
        }
        break 's1;
        }
        F::Dialog_SetChoices(c, 0i32);
        F::Dialog_Start(c, 1i32);
        { let _ = F::Dialog_Input(c, 0i32); };
        { let v_ = 8i32; let a_ = l_d_2; c.m.w32(a_, (v_) as u32); };
        break 's1;
        }
        { let _ = F::Dialog_Input(c, 0i32); };
        if ((F::Dialog_IsClosed(c)) != 0) {
            { let v_ = 0i32; let a_ = l_d_2; c.m.w32(a_, (v_) as u32); };
        }
        break 's1;
    }
    { let a0_ = (c.m.r32((l_d_2).wrapping_add(12)) as i32); F::Dialog_SetMsg(c, a0_) };
    F::Dialog_Draw(c, 1i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EvoZ_UpdateDialog", &PROBE_N, &[], true); }
    r_
}

pub fn EvoZ_ToggleList(c: &mut Ctx, mut p_ez_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EvoZ_ToggleList", &PROBE_N, &[(p_ez_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(80) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 80);
    let mut l_f_2: u32 = 0;
    let mut l_i_3: i32 = 0;
    let r_ = 'f: {
        l_f_2 = ((p_ez_1).wrapping_add(44)).wrapping_add((((2i32) as u32)).wrapping_mul(44));
        { let r_ = 1i32; let a_ = (p_ez_1).wrapping_add(616); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
        { let r_ = (!(128i32)); let a_ = (p_ez_1).wrapping_add(556); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
        if ((((((c.m.r32((p_ez_1).wrapping_add(616)) as i32)) == (0i32)) as i32)) != 0) {
            F::EvoZ_ClipGoto(c, p_ez_1, 2i32, 3i32, 0i32, sym::menu_v_b__str102);
            {
                l_i_3 = 0i32;
                'l1: loop {
                    if !(((((l_i_3) < (8i32)) as i32)) != 0) { break 'l1; }
                    'c2: {
                        { let _ = F::sprintf(c, fp.wrapping_add(8), sym::menu_v_b__str103, &[(((l_i_3).wrapping_add(1i32)) as i64 as u64)]); };
                        F::Flash_FindLabel(c, l_f_2, ((0i32) as u32), fp.wrapping_add(8), fp);
                        F::Flash_ClipSetCallbackA(c, l_f_2, fp, ((0i32) as u32), ((0i32) as u32));
                        F::Flash_ClipSetCallbackB(c, l_f_2, fp, ((0i32) as u32), ((0i32) as u32));
                        F::Flash_ClipSetColor(c, l_f_2, fp, F32(0x3f800000));
                    }
                    { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
                }
            }
        } else {
            F::EvoZ_ClipGoto(c, p_ez_1, 2i32, 3i32, 0i32, sym::menu_v_b__str104);
            F::EvoZ_ClipGoto(c, p_ez_1, 2i32, 10i32, 0i32, sym::menu_v_b__str105);
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EvoZ_ToggleList", &PROBE_N, &[], true); }
    r_
}

pub fn EvoZ_Pow(c: &mut Ctx, mut p_base_1: i32, mut p_exp_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EvoZ_Pow", &PROBE_N, &[(p_base_1) as u32, (p_exp_2) as u32], false); }
    let mut l_ret_3: i32 = 0;
    let mut l_i_4: i32 = 0;
    let r_ = 'f: {
    l_ret_3 = p_base_1;
    {
        l_i_4 = 1i32;
        'l1: loop {
            if !(((((l_i_4) < (p_exp_2)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let r_ = p_base_1; let n_ = (l_ret_3).wrapping_mul(r_); l_ret_3 = n_; };
            }
            { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
        }
    }
    break 'f l_ret_3;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EvoZ_Pow", &PROBE_N, &[], true); }
    r_
}

pub fn EvoZ_GetLevelExp(c: &mut Ctx, mut p_ez_1: u32, mut p_chara_2: i32, mut p_cell_3: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EvoZ_GetLevelExp", &PROBE_N, &[(p_ez_1) as u32, (p_chara_2) as u32, (p_cell_3) as u32], false); }
    let mut l_tbl_4: u32 = 0;
    let mut l_ret_5: i32 = 0;
    let r_ = 'f: {
    l_tbl_4 = ({ let a_ = ((c.m.r32(sym::gCommonRes)).wrapping_add(4)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = ((c.m.r32(sym::gCommonRes)).wrapping_add(4)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32));
    if ((((((({ let a_ = (((c.m.r32(sym::gSaveData)).wrapping_add(6152)).wrapping_add((((p_cell_3) as u32)).wrapping_mul(56))).wrapping_add(52); c.m.r16(a_) }) as i32)) != (0i32)) as i32)) != 0) {
        l_ret_5 = { let a_ = (((l_tbl_4).wrapping_add((((p_chara_2) as u32)).wrapping_mul(60))).wrapping_add(16)).wrapping_add((((((({ let a_ = (((c.m.r32(sym::gSaveData)).wrapping_add(6152)).wrapping_add((((p_cell_3) as u32)).wrapping_mul(56))).wrapping_add(52); c.m.r16(a_) }) as i32)).wrapping_sub(1i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
    } else {
        l_ret_5 = 0i32;
    }
    break 'f l_ret_5;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EvoZ_GetLevelExp", &PROBE_N, &[], true); }
    r_
}

pub fn EvoZ_RefreshStatus(c: &mut Ctx, mut p_ez_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EvoZ_RefreshStatus", &PROBE_N, &[(p_ez_1) as u32], false); }
    let mut l_base_2: i32 = 0;
    let r_ = 'f: {
    { let v_ = (((((({ let a_ = ((({ let a_ = ((c.m.r32(sym::gCommonRes)).wrapping_add(4)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = ((c.m.r32(sym::gCommonRes)).wrapping_add(4)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32))).wrapping_add(((({ let a_ = (c.m.r32((p_ez_1).wrapping_add(788))).wrapping_add(108); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(60))).wrapping_add(8); c.m.r16(a_) }) as i32)) ^ (1i32))) & (1i32)); let a_ = (p_ez_1).wrapping_add(6760); c.m.w32(a_, (v_) as u32); };
    { let v_ = { let a0_ = { let a_ = (c.m.r32((p_ez_1).wrapping_add(788))).wrapping_add(108); (c.m.r32(a_) as i32) }; let a1_ = ({ let a_ = c.m.r32((p_ez_1).wrapping_add(788)); (c.m.r32(a_) as i32) }).wrapping_add(({ let a_ = (c.m.r32((p_ez_1).wrapping_add(788))).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(7i32)); F::ChrTbl_GetLevel(c, a0_, a1_, 0i32) }; let a_ = (p_ez_1).wrapping_add(6752); c.m.w32(a_, (v_) as u32); };
    { let v_ = { let a0_ = ((((c.m.r32(sym::gSaveData)).wrapping_add(8)).wrapping_add(6144)).wrapping_add((((({ let a_ = c.m.r32((p_ez_1).wrapping_add(788)); (c.m.r32(a_) as i32) }).wrapping_add(({ let a_ = (c.m.r32((p_ez_1).wrapping_add(788))).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(7i32))) as u32)).wrapping_mul(56))).wrapping_add(((((c.m.r32((p_ez_1).wrapping_add(628)) as i32)) as u32)).wrapping_mul(16)); let a1_ = c.m.r32((p_ez_1).wrapping_add(12488)); let a2_ = (c.m.r32((p_ez_1).wrapping_add(6752)) as i32); F::ItemSet_Fit(c, a0_, a1_, a2_, (p_ez_1).wrapping_add(640)) }; let a_ = (p_ez_1).wrapping_add(6756); c.m.w32(a_, (v_) as u32); };
    { let v_ = { let a_ = ((((c.m.r32(sym::gSaveData)).wrapping_add(8)).wrapping_add(6144)).wrapping_add((((({ let a_ = c.m.r32((p_ez_1).wrapping_add(788)); (c.m.r32(a_) as i32) }).wrapping_add(({ let a_ = (c.m.r32((p_ez_1).wrapping_add(788))).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(7i32))) as u32)).wrapping_mul(56))).wrapping_add(48); (c.m.r32(a_) as i32) }; let a_ = (p_ez_1).wrapping_add(6764); c.m.w32(a_, (v_) as u32); };
    { let v_ = { let a0_ = { let a_ = (c.m.r32((p_ez_1).wrapping_add(788))).wrapping_add(108); (c.m.r32(a_) as i32) }; let a1_ = ({ let a_ = c.m.r32((p_ez_1).wrapping_add(788)); (c.m.r32(a_) as i32) }).wrapping_add(({ let a_ = (c.m.r32((p_ez_1).wrapping_add(788))).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(7i32)); F::ChrTbl_GetExp(c, a0_, a1_) }; let a_ = (p_ez_1).wrapping_add(6768); c.m.w32(a_, (v_) as u32); };
    { let v_ = ({ let a0_ = { let a_ = (c.m.r32((p_ez_1).wrapping_add(788))).wrapping_add(108); (c.m.r32(a_) as i32) }; F::ChrTbl_GetMaxExp(c, a0_) }).wrapping_sub((c.m.r32((p_ez_1).wrapping_add(6764)) as i32)); let a_ = (p_ez_1).wrapping_add(6772); c.m.w32(a_, (v_) as u32); };
    { let a0_ = ((((c.m.r32(sym::gSaveData)).wrapping_add(8)).wrapping_add(6144)).wrapping_add((((({ let a_ = c.m.r32((p_ez_1).wrapping_add(788)); (c.m.r32(a_) as i32) }).wrapping_add(({ let a_ = (c.m.r32((p_ez_1).wrapping_add(788))).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(7i32))) as u32)).wrapping_mul(56))).wrapping_add(((((c.m.r32((p_ez_1).wrapping_add(628)) as i32)) as u32)).wrapping_mul(16)); let a1_ = c.m.r32((p_ez_1).wrapping_add(12488)); F::ItemSet_GetBonus(c, a0_, a1_, (p_ez_1).wrapping_add(6776)) };
    l_base_2 = { let a1_ = { let a_ = (c.m.r32((p_ez_1).wrapping_add(788))).wrapping_add(108); (c.m.r32(a_) as i32) }; let a2_ = ({ let a_ = c.m.r32((p_ez_1).wrapping_add(788)); (c.m.r32(a_) as i32) }).wrapping_add(({ let a_ = (c.m.r32((p_ez_1).wrapping_add(788))).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(7i32)); F::EvoZ_GetLevelExp(c, p_ez_1, a1_, a2_) };
    { let v_ = rt::fmul(rt::fdiv(rt::i2f(((c.m.r32((p_ez_1).wrapping_add(6764)) as i32)).wrapping_sub(l_base_2)), rt::i2f(((c.m.r32((p_ez_1).wrapping_add(6768)) as i32)).wrapping_sub(l_base_2))), F32(0x40d51eb8)); let a_ = (p_ez_1).wrapping_add(6796); c.m.w32(a_, (v_).0); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EvoZ_RefreshStatus", &PROBE_N, &[], true); }
    r_
}

pub fn EvoZ_SetRowChips(c: &mut Ctx, mut p_ez_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EvoZ_SetRowChips", &PROBE_N, &[(p_ez_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_i_2: i32 = 0;
    let mut l_id_3: i32 = 0;
    let mut l_res_4: u32 = 0;
    let r_ = 'f: {
        rt::memset(c, fp, 0, 28);
        { let v_ = 8i32; c.m.w32(fp.wrapping_add(0), (v_) as u32); }
        { let v_ = 11i32; c.m.w32(fp.wrapping_add(4), (v_) as u32); }
        { let v_ = 12i32; c.m.w32(fp.wrapping_add(8), (v_) as u32); }
        { let v_ = 13i32; c.m.w32(fp.wrapping_add(12), (v_) as u32); }
        { let v_ = 14i32; c.m.w32(fp.wrapping_add(16), (v_) as u32); }
        { let v_ = 15i32; c.m.w32(fp.wrapping_add(20), (v_) as u32); }
        { let v_ = 16i32; c.m.w32(fp.wrapping_add(24), (v_) as u32); }
        {
            l_i_2 = 0i32;
            'l1: loop {
                if !(((((l_i_2) < (7i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_id_3 = { let a_ = (c.m.r32((p_ez_1).wrapping_add(796))).wrapping_add(((((l_i_2).wrapping_add(({ let a_ = (c.m.r32((p_ez_1).wrapping_add(788))).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(7i32))) as u32)).wrapping_mul(36)); (c.m.r32(a_) as i32) };
                    { let v_ = { let a_ = ((c.m.r32((p_ez_1).wrapping_add(788))).wrapping_add(48)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ((c.m.r32((p_ez_1).wrapping_add(788))).wrapping_add(76)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    { let v_ = l_id_3; let a_ = ((c.m.r32((p_ez_1).wrapping_add(788))).wrapping_add(48)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    l_res_4 = (c.m.r32((p_ez_1).wrapping_add(24))).wrapping_add((({ let a_ = (c.m.r32((p_ez_1).wrapping_add(24))).wrapping_add(((((l_id_3).wrapping_add(1i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32));
                    { let v_ = { let a_ = ((p_ez_1).wrapping_add(352)).wrapping_add(((((c.m.r32((fp).wrapping_add((((l_i_2) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) }; let a_ = ((p_ez_1).wrapping_add(352)).wrapping_add(((((18i32).wrapping_add(l_i_2)) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = c.m.r32((l_res_4).wrapping_add(16)); let a_ = ((p_ez_1).wrapping_add(352)).wrapping_add(((((c.m.r32((fp).wrapping_add((((l_i_2) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EvoZ_SetRowChips", &PROBE_N, &[], true); }
    r_
}

pub fn EvoZ_UpdateFace(c: &mut Ctx, mut p_ez_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EvoZ_UpdateFace", &PROBE_N, &[(p_ez_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        's1: {
            let sv2: i32 = (c.m.r32((p_ez_1).wrapping_add(612)) as i32);
            match sv2 {
                5i32 => {
                    { let v_ = 6i32; let a_ = (p_ez_1).wrapping_add(612); c.m.w32(a_, (v_) as u32); };
                    break 's1;
                }
                6i32 => {
                    if (((({ let a_ = (c.m.r32((p_ez_1).wrapping_add(788))).wrapping_add(104); (c.m.r32(a_) as i32) }) & (1i32))) != 0) {
                        { let r_ = 1i32; let a_ = (c.m.r32((p_ez_1).wrapping_add(788))).wrapping_add(104); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
                    }
                    { let v_ = 1i32; let a_ = (p_ez_1).wrapping_add(612); c.m.w32(a_, (v_) as u32); };
                    break 's1;
                }
                1i32 => {
                    F::File_CancelRequests(c);
                    { let _ = { let a0_ = ({ let a_ = (c.m.r32((p_ez_1).wrapping_add(788))).wrapping_add(108); (c.m.r32(a_) as i32) }).wrapping_add(761i32); let a1_ = c.m.r32((p_ez_1).wrapping_add(8)); let _ = 92160i32; F::File_Request(c, a0_, a1_) }; };
                    { let v_ = 2i32; let a_ = (p_ez_1).wrapping_add(612); c.m.w32(a_, (v_) as u32); };
                    break 's1;
                }
                2i32 => {
                    if ((F::File_UpdateRequests(c)) != 0) {
                        { let v_ = 3i32; let a_ = (p_ez_1).wrapping_add(612); c.m.w32(a_, (v_) as u32); };
                    }
                    break 's1;
                }
                3i32 => {
                    { let _ = { let a0_ = c.m.r32((p_ez_1).wrapping_add(8)); let a1_ = c.m.r32((p_ez_1).wrapping_add(12)); F::Sprite_Unpack(c, a0_, a1_, ((0i32) as u32)) }; };
                    { let v_ = c.m.r32((p_ez_1).wrapping_add(12)); let a_ = fp; c.m.w32(a_, v_); };
                    { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((p_ez_1).wrapping_add(180)).wrapping_add((((42i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let r_ = 2i32; let a_ = (c.m.r32((p_ez_1).wrapping_add(788))).wrapping_add(104); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    { let v_ = 4i32; let a_ = (p_ez_1).wrapping_add(612); c.m.w32(a_, (v_) as u32); };
                    break 's1;
                }
                4i32 => {
                    if (((({ let a_ = (c.m.r32((p_ez_1).wrapping_add(788))).wrapping_add(104); (c.m.r32(a_) as i32) }) & (1i32))) != 0) {
                        { let v_ = ((0i32) as u32); let a_ = ((p_ez_1).wrapping_add(180)).wrapping_add((((42i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                        { let v_ = 6i32; let a_ = (p_ez_1).wrapping_add(612); c.m.w32(a_, (v_) as u32); };
                    }
                    break 's1;
                }
                _ => {
                }
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EvoZ_UpdateFace", &PROBE_N, &[], true); }
    r_
}

pub fn EvoZ_RequestFace(c: &mut Ctx, mut p_ez_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EvoZ_RequestFace", &PROBE_N, &[(p_ez_1) as u32], false); }
    let r_ = 'f: {
    if (((({ let a_ = (c.m.r32((p_ez_1).wrapping_add(788))).wrapping_add(104); (c.m.r32(a_) as i32) }) & (2i32))) != 0) {
        { let r_ = 2i32; let a_ = (c.m.r32((p_ez_1).wrapping_add(788))).wrapping_add(104); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
    } else {
        { let v_ = 5i32; let a_ = (p_ez_1).wrapping_add(612); c.m.w32(a_, (v_) as u32); };
    }
    { let r_ = 1i32; let a_ = (c.m.r32((p_ez_1).wrapping_add(788))).wrapping_add(104); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EvoZ_RequestFace", &PROBE_N, &[], true); }
    r_
}

pub fn EvoZ_CanEquip(c: &mut Ctx, mut p_ez_1: u32, mut p_item_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EvoZ_CanEquip", &PROBE_N, &[(p_ez_1) as u32, (p_item_2) as u32], false); }
    let mut l_ret_3: i32 = 0;
    let mut l_ids_4: u32 = 0;
    let mut l_used_5: i32 = 0;
    let mut l_i_6: i32 = 0;
    let r_ = 'f: {
    l_ret_3 = 1i32;
    if ((((!(((((((({ let a_ = (((c.m.r32(sym::gSaveData)).wrapping_add(8)).wrapping_add(11968)).wrapping_add(((p_item_2) as u32)); c.m.r8(a_) }) as i32)) & (1i32))) as u8)) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    l_ids_4 = ((((c.m.r32(sym::gSaveData)).wrapping_add(8)).wrapping_add(6144)).wrapping_add((((({ let a_ = c.m.r32((p_ez_1).wrapping_add(788)); (c.m.r32(a_) as i32) }).wrapping_add(({ let a_ = (c.m.r32((p_ez_1).wrapping_add(788))).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(7i32))) as u32)).wrapping_mul(56))).wrapping_add(((((c.m.r32((p_ez_1).wrapping_add(628)) as i32)) as u32)).wrapping_mul(16));
    if ((((!(({ let a1_ = { let a_ = (c.m.r32((p_ez_1).wrapping_add(788))).wrapping_add(108); (c.m.r32(a_) as i32) }; let a2_ = c.m.r32((p_ez_1).wrapping_add(12488)); F::ItemTbl_CanEquip(c, p_item_2, a1_, a2_) }) != 0)) as i32)) != 0) {
        l_ret_3 = 0i32;
    } else {
        if ((((((c.m.r32((p_ez_1).wrapping_add(636)) as i32)) != (0i32)) as i32)) != 0) {
            l_used_5 = ((c.m.r32((p_ez_1).wrapping_add(6756)) as i32)).wrapping_sub((({ let a_ = ((c.m.r32((p_ez_1).wrapping_add(12488))).wrapping_add((((((c.m.r32((p_ez_1).wrapping_add(636)) as i32)).wrapping_sub(1i32)) as u32)).wrapping_mul(40))).wrapping_add(3); c.m.r8(a_) }) as i32));
        } else {
            l_used_5 = (c.m.r32((p_ez_1).wrapping_add(6756)) as i32);
        }
        if ((((((c.m.r32((p_ez_1).wrapping_add(6752)) as i32)) >= ((l_used_5).wrapping_add((({ let a_ = ((c.m.r32((p_ez_1).wrapping_add(12488))).wrapping_add((((p_item_2) as u32)).wrapping_mul(40))).wrapping_add(3); c.m.r8(a_) }) as i32)))) as i32)) != 0) {
            {
                l_i_6 = 0i32;
                'l1: loop {
                    if !(((((l_i_6) < (7i32)) as i32)) != 0) { break 'l1; }
                    'c2: {
                        if (((((((((l_i_6) != ((c.m.r32(((p_ez_1).wrapping_add(560)).wrapping_add((((3i32) as u32)).wrapping_mul(4))) as i32))) as i32)) != 0) && (((((((c.m.r16((l_ids_4).wrapping_add((((l_i_6) as u32)).wrapping_mul(2)))) as i32)) != (0i32)) as i32)) != 0)) as i32)) != 0) {
                            if ((((((({ let a_ = (c.m.r32((p_ez_1).wrapping_add(12488))).wrapping_add((((p_item_2) as u32)).wrapping_mul(40)); c.m.r8(a_) }) as i32)) == ((({ let a_ = (c.m.r32((p_ez_1).wrapping_add(12488))).wrapping_add(((((((c.m.r16((l_ids_4).wrapping_add((((l_i_6) as u32)).wrapping_mul(2)))) as i32)).wrapping_sub(1i32)) as u32)).wrapping_mul(40)); c.m.r8(a_) }) as i32))) as i32)) != 0) {
                                if ((((((({ let a_ = ((c.m.r32((p_ez_1).wrapping_add(12488))).wrapping_add((((p_item_2) as u32)).wrapping_mul(40))).wrapping_add(1); c.m.r8(a_) }) as i32)) == ((({ let a_ = ((c.m.r32((p_ez_1).wrapping_add(12488))).wrapping_add(((((((c.m.r16((l_ids_4).wrapping_add((((l_i_6) as u32)).wrapping_mul(2)))) as i32)).wrapping_sub(1i32)) as u32)).wrapping_mul(40))).wrapping_add(1); c.m.r8(a_) }) as i32))) as i32)) != 0) {
                                    l_ret_3 = 0i32;
                                    break 'l1;
                                }
                            }
                        }
                    }
                    { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; };
                }
            }
        } else {
            l_ret_3 = 0i32;
        }
    }
    break 'f l_ret_3;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EvoZ_CanEquip", &PROBE_N, &[], true); }
    r_
}

pub fn EvoZ_RemoveItem(c: &mut Ctx, mut p_ez_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EvoZ_RemoveItem", &PROBE_N, &[(p_ez_1) as u32], false); }
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
    { let v_ = ((0i32) as u16); let a_ = (((((c.m.r32(sym::gSaveData)).wrapping_add(8)).wrapping_add(6144)).wrapping_add((((({ let a_ = c.m.r32((p_ez_1).wrapping_add(788)); (c.m.r32(a_) as i32) }).wrapping_add(({ let a_ = (c.m.r32((p_ez_1).wrapping_add(788))).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(7i32))) as u32)).wrapping_mul(56))).wrapping_add(((((c.m.r32((p_ez_1).wrapping_add(628)) as i32)) as u32)).wrapping_mul(16))).wrapping_add(((((c.m.r32(((p_ez_1).wrapping_add(560)).wrapping_add((((3i32) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(2)); c.m.w16(a_, v_); };
    if ((((((c.m.r32(((p_ez_1).wrapping_add(560)).wrapping_add((((3i32) as u32)).wrapping_mul(4))) as i32)) != (7i32)) as i32)) != 0) {
        {
            l_i_2 = (c.m.r32(((p_ez_1).wrapping_add(560)).wrapping_add((((3i32) as u32)).wrapping_mul(4))) as i32);
            'l1: loop {
                if !(((((l_i_2) < (6i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let v_ = { let a_ = (((((c.m.r32(sym::gSaveData)).wrapping_add(8)).wrapping_add(6144)).wrapping_add((((({ let a_ = c.m.r32((p_ez_1).wrapping_add(788)); (c.m.r32(a_) as i32) }).wrapping_add(({ let a_ = (c.m.r32((p_ez_1).wrapping_add(788))).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(7i32))) as u32)).wrapping_mul(56))).wrapping_add(((((c.m.r32((p_ez_1).wrapping_add(628)) as i32)) as u32)).wrapping_mul(16))).wrapping_add(((((l_i_2).wrapping_add(1i32)) as u32)).wrapping_mul(2)); c.m.r16(a_) }; let a_ = (((((c.m.r32(sym::gSaveData)).wrapping_add(8)).wrapping_add(6144)).wrapping_add((((({ let a_ = c.m.r32((p_ez_1).wrapping_add(788)); (c.m.r32(a_) as i32) }).wrapping_add(({ let a_ = (c.m.r32((p_ez_1).wrapping_add(788))).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(7i32))) as u32)).wrapping_mul(56))).wrapping_add(((((c.m.r32((p_ez_1).wrapping_add(628)) as i32)) as u32)).wrapping_mul(16))).wrapping_add((((l_i_2) as u32)).wrapping_mul(2)); c.m.w16(a_, v_); };
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        { let v_ = ((0i32) as u16); let a_ = (((((c.m.r32(sym::gSaveData)).wrapping_add(8)).wrapping_add(6144)).wrapping_add((((({ let a_ = c.m.r32((p_ez_1).wrapping_add(788)); (c.m.r32(a_) as i32) }).wrapping_add(({ let a_ = (c.m.r32((p_ez_1).wrapping_add(788))).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(7i32))) as u32)).wrapping_mul(56))).wrapping_add(((((c.m.r32((p_ez_1).wrapping_add(628)) as i32)) as u32)).wrapping_mul(16))).wrapping_add((((6i32) as u32)).wrapping_mul(2)); c.m.w16(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EvoZ_RemoveItem", &PROBE_N, &[], true); }
    r_
}

pub fn EvoZ_RemoveAllItems(c: &mut Ctx, mut p_ez_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EvoZ_RemoveAllItems", &PROBE_N, &[(p_ez_1) as u32], false); }
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < (8i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let v_ = ((0i32) as u16); let a_ = (((((c.m.r32(sym::gSaveData)).wrapping_add(8)).wrapping_add(6144)).wrapping_add((((({ let a_ = c.m.r32((p_ez_1).wrapping_add(788)); (c.m.r32(a_) as i32) }).wrapping_add(({ let a_ = (c.m.r32((p_ez_1).wrapping_add(788))).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(7i32))) as u32)).wrapping_mul(56))).wrapping_add(((((c.m.r32((p_ez_1).wrapping_add(628)) as i32)) as u32)).wrapping_mul(16))).wrapping_add((((l_i_2) as u32)).wrapping_mul(2)); c.m.w16(a_, v_); };
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EvoZ_RemoveAllItems", &PROBE_N, &[], true); }
    r_
}
