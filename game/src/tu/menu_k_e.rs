#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn Bracket_SetupDraw(c: &mut Ctx, mut p_b_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Bracket_SetupDraw", &PROBE_N, &[(p_b_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(96) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 96);
    let mut l_flash_2: u32 = 0;
    let mut l_i_3: i32 = 0;
    let mut l_clip_4: u32 = 0;
    let mut l_clip_5: u32 = 0;
    let mut l_x_6: i32 = 0;
    let mut l_y_7: i32 = 0;
    let mut l_x_8: i32 = 0;
    let mut l_y_9: i32 = 0;
    let mut l_x_10: i32 = 0;
    let mut l_y_11: i32 = 0;
    let mut l_chara_12: i32 = 0;
    let mut l_cols_13: i32 = 0;
    let mut l_plate_14: i32 = 0;
    let r_ = 'f: {
        l_flash_2 = ((p_b_1).wrapping_add(48)).wrapping_add((((5i32) as u32)).wrapping_mul(44));
        { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = ((c.m.r32((p_b_1).wrapping_add(6724)) as i32)).wrapping_mul(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = 256i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = ((c.m.r32((fp.wrapping_add(8)).wrapping_add(4)) as i32)).wrapping_add(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        F::Flash_FindLabel(c, l_flash_2, sym::menu_k_e__str0, sym::menu_k_e__str1, fp);
        F::Flash_ClipSetUv(c, l_flash_2, fp, fp.wrapping_add(8));
        { let v_ = 128i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = 512i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 256i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), sym::menu_k_e__str2, fp);
        F::Flash_ClipSetUv(c, l_flash_2, fp, fp.wrapping_add(8));
        { let v_ = (if (((((c.m.r32((p_b_1).wrapping_add(732)) as i32)) & (4096i32))) != 0) { 64i32 } else { 0i32 }); let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = ((c.m.r32((fp.wrapping_add(8)).wrapping_add(4)) as i32)).wrapping_add(64i32); let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        { let v_ = 256i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        F::Flash_FindLabel(c, l_flash_2, sym::menu_k_e__str3, sym::menu_k_e__str4, fp);
        F::Flash_ClipSetUv(c, l_flash_2, fp, fp.wrapping_add(8));
        l_flash_2 = ((p_b_1).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(44));
        's1: {
            let sv15: i32 = { let a_ = (c.m.r32(sym::gProgress)).wrapping_add(132); (c.m.r32(a_) as i32) };
            match sv15 {
                0i32 => {
                    F::Flash_FindLabel(c, l_flash_2, sym::menu_k_e__str5, sym::menu_k_e__str6, fp);
                    F::FlashAnim_Talk(c, l_flash_2, fp, ((p_b_1).wrapping_add(6752)).wrapping_add((((0i32) as u32)).wrapping_mul(4)), 0i32);
                    break 's1;
                }
                1i32 => {
                    {
                        l_i_3 = 0i32;
                        'l2: loop {
                            if !(((((l_i_3) < (2i32)) as i32)) != 0) { break 'l2; }
                            'c3: {
                                if ((((((c.m.r32((p_b_1).wrapping_add(748)) as i32)) == (l_i_3)) as i32)) != 0) {
                                    l_clip_4 = (if ((l_i_3) != 0) { sym::menu_k_e__str7 } else { sym::menu_k_e__str8 });
                                    F::Flash_FindLabel(c, l_flash_2, l_clip_4, (if ((l_i_3) != 0) { sym::menu_k_e__str9 } else { sym::menu_k_e__str10 }), fp);
                                    F::FlashAnim_Blink(c, l_flash_2, fp, ((p_b_1).wrapping_add(6744)).wrapping_add((((0i32) as u32)).wrapping_mul(4)), 0i32);
                                    F::Flash_FindLabel(c, l_flash_2, l_clip_4, (if ((l_i_3) != 0) { sym::menu_k_e__str11 } else { sym::menu_k_e__str12 }), fp);
                                    F::FlashAnim_Talk(c, l_flash_2, fp, ((p_b_1).wrapping_add(6752)).wrapping_add((((0i32) as u32)).wrapping_mul(4)), 0i32);
                                }
                            }
                            { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
                        }
                    }
                    break 's1;
                }
                2i32 => {
                    {
                        l_i_3 = 0i32;
                        'l4: loop {
                            if !(((((l_i_3) < (2i32)) as i32)) != 0) { break 'l4; }
                            'c5: {
                                if ((((((c.m.r32((p_b_1).wrapping_add(748)) as i32)) == (l_i_3)) as i32)) != 0) {
                                    l_clip_5 = (if ((l_i_3) != 0) { sym::menu_k_e__str13 } else { sym::menu_k_e__str14 });
                                    F::Flash_FindLabel(c, l_flash_2, l_clip_5, (if ((l_i_3) != 0) { sym::menu_k_e__str15 } else { sym::menu_k_e__str16 }), fp);
                                    F::FlashAnim_Blink(c, l_flash_2, fp, ((p_b_1).wrapping_add(6744)).wrapping_add((((0i32) as u32)).wrapping_mul(4)), 0i32);
                                    F::Flash_FindLabel(c, l_flash_2, l_clip_5, (if ((l_i_3) != 0) { sym::menu_k_e__str17 } else { sym::menu_k_e__str18 }), fp);
                                    F::FlashAnim_Talk(c, l_flash_2, fp, ((p_b_1).wrapping_add(6752)).wrapping_add((((0i32) as u32)).wrapping_mul(4)), 0i32);
                                }
                            }
                            { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
                        }
                    }
                    break 's1;
                }
                3i32 => {
                    F::Flash_FindLabel(c, l_flash_2, sym::menu_k_e__str19, sym::menu_k_e__str20, fp);
                    F::FlashAnim_Talk(c, l_flash_2, fp, ((p_b_1).wrapping_add(6752)).wrapping_add((((0i32) as u32)).wrapping_mul(4)), 0i32);
                    break 's1;
                }
                4i32 => {
                    {
                        l_i_3 = 0i32;
                        'l6: loop {
                            if !(((((l_i_3) < (2i32)) as i32)) != 0) { break 'l6; }
                            'c7: {
                                { let a2_ = (if ((l_i_3) != 0) { (if (((c.m.r32((p_b_1).wrapping_add(748)) as i32)) != 0) { sym::menu_k_e__str23 } else { sym::menu_k_e__str24 }) } else { sym::menu_k_e__str25 }); F::Flash_FindLabel(c, l_flash_2, (if ((l_i_3) != 0) { sym::menu_k_e__str21 } else { sym::menu_k_e__str22 }), a2_, fp) };
                                F::FlashAnim_Blink(c, l_flash_2, fp, ((p_b_1).wrapping_add(6744)).wrapping_add((((l_i_3) as u32)).wrapping_mul(4)), 0i32);
                                { let a2_ = (if ((l_i_3) != 0) { (if (((c.m.r32((p_b_1).wrapping_add(748)) as i32)) != 0) { sym::menu_k_e__str28 } else { sym::menu_k_e__str29 }) } else { sym::menu_k_e__str30 }); F::Flash_FindLabel(c, l_flash_2, (if ((l_i_3) != 0) { sym::menu_k_e__str26 } else { sym::menu_k_e__str27 }), a2_, fp) };
                                if ((((((c.m.r32((p_b_1).wrapping_add(752)) as i32)) == (l_i_3)) as i32)) != 0) {
                                    F::FlashAnim_Talk(c, l_flash_2, fp, ((p_b_1).wrapping_add(6752)).wrapping_add((((l_i_3) as u32)).wrapping_mul(4)), 0i32);
                                } else {
                                    F::FlashAnim_ShowNext2(c, l_flash_2, fp, 0i32);
                                }
                            }
                            { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
                        }
                    }
                    break 's1;
                }
                _ => {
                }
            }
        }
        l_flash_2 = ((p_b_1).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(44));
        { let a1_ = rt::f2i(F32(c.m.r32((p_b_1).wrapping_add(6740)))); F::Flash_SetOffset(c, l_flash_2, a1_, 0i32) };
        F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), sym::menu_k_e__str31, fp);
        { let a2_ = (rt::f2i(F32(c.m.r32((p_b_1).wrapping_add(6740))))).wrapping_neg(); F::Flash_ClipSetOffset(c, l_flash_2, fp, a2_, 0i32) };
        {
            l_i_3 = 0i32;
            'l8: loop {
                if !(((((l_i_3) < (2i32)) as i32)) != 0) { break 'l8; }
                'c9: {
                    { let v_ = (l_i_3).wrapping_mul(64i32); let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    { let v_ = ((c.m.r32(fp.wrapping_add(8)) as i32)).wrapping_add(64i32); let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 64i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                    F::Flash_FindLabel(c, l_flash_2, sym::menu_k_e__str32, (if ((l_i_3) != 0) { sym::menu_k_e__str33 } else { sym::menu_k_e__str34 }), fp);
                    F::Flash_ClipSetUv(c, l_flash_2, fp, fp.wrapping_add(8));
                    if (((((c.m.r32((p_b_1).wrapping_add(732)) as i32)) & (4i32))) != 0) {
                        if (((((((((((((rt::f2i(F32(c.m.r32((p_b_1).wrapping_add(6740))))) <= ((512i32).wrapping_neg())) as i32)) != 0) && (((((l_i_3) == (0i32)) as i32)) != 0)) as i32)) != 0) || (((((((((rt::f2i(F32(c.m.r32((p_b_1).wrapping_add(6740))))) >= (0i32)) as i32)) != 0) && (((((l_i_3) != (0i32)) as i32)) != 0)) as i32)) != 0)) as i32)) != 0) {
                            { let _ = F::Flash_ClipSetFlags(c, l_flash_2, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
                        } else {
                            { let _ = F::Flash_ClipSetFlags(c, l_flash_2, fp, 2i32, ((((1i32) as i64 as u64)) as u8)); };
                        }
                    } else {
                        { let _ = F::Flash_ClipSetFlags(c, l_flash_2, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
                    }
                }
                { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
            }
        }
        {
            l_i_3 = 0i32;
            'l10: loop {
                if !(((((l_i_3) < (17i32)) as i32)) != 0) { break 'l10; }
                'c11: {
                    { let _ = F::sprintf(c, fp.wrapping_add(28), sym::menu_k_e__str35, &[((l_i_3) as i64 as u64)]); };
                    F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), fp.wrapping_add(28), fp);
                    if ((((((c.m.r16(((p_b_1).wrapping_add(6760)).wrapping_add((((l_i_3) as u32)).wrapping_mul(40)))) as i32)) & (1i32))) != 0) {
                        { let v_ = (rt::rem_i32((c.m.r32((((p_b_1).wrapping_add(6760)).wrapping_add((((l_i_3) as u32)).wrapping_mul(40))).wrapping_add(12)) as i32), 4i32)).wrapping_mul(64i32); let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                        { let v_ = (rt::div_i32((c.m.r32((((p_b_1).wrapping_add(6760)).wrapping_add((((l_i_3) as u32)).wrapping_mul(40))).wrapping_add(12)) as i32), 4i32)).wrapping_mul(64i32); let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                        { let v_ = ((c.m.r32((fp.wrapping_add(8)).wrapping_add(4)) as i32)).wrapping_add(64i32); let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                        { let v_ = ((c.m.r32(fp.wrapping_add(8)) as i32)).wrapping_add(64i32); let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                        F::Flash_ClipSetTex(c, l_flash_2, fp, 1i32);
                        F::Flash_ClipSetUv(c, l_flash_2, fp, fp.wrapping_add(8));
                    }
                    { let _ = F::sprintf(c, fp.wrapping_add(28), sym::menu_k_e__str36, &[((l_i_3) as i64 as u64)]); };
                    F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), fp.wrapping_add(28), fp);
                    l_x_6 = { let a0_ = (c.m.r32((((p_b_1).wrapping_add(6760)).wrapping_add((((l_i_3) as u32)).wrapping_mul(40))).wrapping_add(16)) as i32); F::Bracket_GetPosX(c, a0_) };
                    l_y_7 = { let a0_ = (c.m.r32((((p_b_1).wrapping_add(6760)).wrapping_add((((l_i_3) as u32)).wrapping_mul(40))).wrapping_add(16)) as i32); F::Bracket_GetPosY(c, a0_) };
                    F::Flash_ClipSetOffset(c, l_flash_2, fp, l_x_6, l_y_7);
                    if ((((((c.m.r16(((p_b_1).wrapping_add(6760)).wrapping_add((((l_i_3) as u32)).wrapping_mul(40)))) as i32)) & (8i32))) != 0) {
                        { let _ = F::Flash_ClipSetFlags(c, l_flash_2, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
                    } else {
                        { let _ = F::Flash_ClipSetFlags(c, l_flash_2, fp, 2i32, ((((1i32) as i64 as u64)) as u8)); };
                        if ((((((c.m.r16(((p_b_1).wrapping_add(6760)).wrapping_add((((l_i_3) as u32)).wrapping_mul(40)))) as i32)) & (2i32))) != 0) {
                            F::Flash_ClipSetColor(c, l_flash_2, fp, F32(0x3f199999));
                        }
                    }
                }
                { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
            }
        }
        l_flash_2 = ((p_b_1).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(44));
        if ((((c.m.r32((((p_b_1).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(44))).wrapping_add(8))) & (((1i32) as u32)))) != 0) {
            {
                l_i_3 = 0i32;
                'l12: loop {
                    if !(((((l_i_3) < (2i32)) as i32)) != 0) { break 'l12; }
                    'c13: {
                        if (((((({ let a_ = ((p_b_1).wrapping_add(6760)).wrapping_add(((({ let a_ = ((((p_b_1).wrapping_add(7440)).wrapping_add(((((c.m.r32((p_b_1).wrapping_add(6728)) as i32)) as u32)).wrapping_mul(16))).wrapping_add(2)).wrapping_add((((l_i_3) as u32)).wrapping_mul(2)); c.m.r16(a_) }) as u32)).wrapping_mul(40)); c.m.r16(a_) }) as i32)) & (1i32))) != 0) {
                            { let v_ = (rt::rem_i32({ let a_ = (((p_b_1).wrapping_add(6760)).wrapping_add(((({ let a_ = ((((p_b_1).wrapping_add(7440)).wrapping_add(((((c.m.r32((p_b_1).wrapping_add(6728)) as i32)) as u32)).wrapping_mul(16))).wrapping_add(2)).wrapping_add((((l_i_3) as u32)).wrapping_mul(2)); c.m.r16(a_) }) as u32)).wrapping_mul(40))).wrapping_add(12); (c.m.r32(a_) as i32) }, 4i32)).wrapping_mul(64i32); let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                            { let v_ = (rt::div_i32({ let a_ = (((p_b_1).wrapping_add(6760)).wrapping_add(((({ let a_ = ((((p_b_1).wrapping_add(7440)).wrapping_add(((((c.m.r32((p_b_1).wrapping_add(6728)) as i32)) as u32)).wrapping_mul(16))).wrapping_add(2)).wrapping_add((((l_i_3) as u32)).wrapping_mul(2)); c.m.r16(a_) }) as u32)).wrapping_mul(40))).wrapping_add(12); (c.m.r32(a_) as i32) }, 4i32)).wrapping_mul(64i32); let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                            { let v_ = ((c.m.r32((fp.wrapping_add(8)).wrapping_add(4)) as i32)).wrapping_add(64i32); let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                            { let v_ = ((c.m.r32(fp.wrapping_add(8)) as i32)).wrapping_add(64i32); let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                            F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), (if ((l_i_3) != 0) { sym::menu_k_e__str37 } else { sym::menu_k_e__str38 }), fp);
                            F::Flash_ClipSetTex(c, l_flash_2, fp, 1i32);
                            F::Flash_ClipSetUv(c, l_flash_2, fp, fp.wrapping_add(8));
                        }
                    }
                    { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
                }
            }
            l_x_8 = ({ let a0_ = { let a_ = (((p_b_1).wrapping_add(6760)).wrapping_add(((({ let a_ = ((((p_b_1).wrapping_add(7440)).wrapping_add(((((c.m.r32((p_b_1).wrapping_add(6728)) as i32)) as u32)).wrapping_mul(16))).wrapping_add(2)).wrapping_add((((0i32) as u32)).wrapping_mul(2)); c.m.r16(a_) }) as u32)).wrapping_mul(40))).wrapping_add(16); (c.m.r32(a_) as i32) }; F::Bracket_GetPosX(c, a0_) }).wrapping_add(rt::f2i(F32(c.m.r32((p_b_1).wrapping_add(6740)))));
            l_y_9 = { let a0_ = { let a_ = (((p_b_1).wrapping_add(6760)).wrapping_add(((({ let a_ = ((((p_b_1).wrapping_add(7440)).wrapping_add(((((c.m.r32((p_b_1).wrapping_add(6728)) as i32)) as u32)).wrapping_mul(16))).wrapping_add(2)).wrapping_add((((0i32) as u32)).wrapping_mul(2)); c.m.r16(a_) }) as u32)).wrapping_mul(40))).wrapping_add(16); (c.m.r32(a_) as i32) }; F::Bracket_GetPosY(c, a0_) };
            F::Flash_SetOffset(c, l_flash_2, l_x_8, l_y_9);
        }
        l_flash_2 = ((p_b_1).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(44));
        if ((((c.m.r32((((p_b_1).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(44))).wrapping_add(8))) & (((1i32) as u32)))) != 0) {
            {
                l_i_3 = 0i32;
                'l14: loop {
                    if !(((((l_i_3) < (2i32)) as i32)) != 0) { break 'l14; }
                    'c15: {
                        if (((((({ let a_ = ((p_b_1).wrapping_add(6760)).wrapping_add(((({ let a_ = ((((p_b_1).wrapping_add(7440)).wrapping_add(((((c.m.r32((p_b_1).wrapping_add(6728)) as i32)) as u32)).wrapping_mul(16))).wrapping_add(2)).wrapping_add((((l_i_3) as u32)).wrapping_mul(2)); c.m.r16(a_) }) as u32)).wrapping_mul(40)); c.m.r16(a_) }) as i32)) & (1i32))) != 0) {
                            { let v_ = (rt::rem_i32({ let a_ = (((p_b_1).wrapping_add(6760)).wrapping_add(((({ let a_ = ((((p_b_1).wrapping_add(7440)).wrapping_add(((((c.m.r32((p_b_1).wrapping_add(6728)) as i32)) as u32)).wrapping_mul(16))).wrapping_add(2)).wrapping_add((((l_i_3) as u32)).wrapping_mul(2)); c.m.r16(a_) }) as u32)).wrapping_mul(40))).wrapping_add(12); (c.m.r32(a_) as i32) }, 4i32)).wrapping_mul(64i32); let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                            { let v_ = (rt::div_i32({ let a_ = (((p_b_1).wrapping_add(6760)).wrapping_add(((({ let a_ = ((((p_b_1).wrapping_add(7440)).wrapping_add(((((c.m.r32((p_b_1).wrapping_add(6728)) as i32)) as u32)).wrapping_mul(16))).wrapping_add(2)).wrapping_add((((l_i_3) as u32)).wrapping_mul(2)); c.m.r16(a_) }) as u32)).wrapping_mul(40))).wrapping_add(12); (c.m.r32(a_) as i32) }, 4i32)).wrapping_mul(64i32); let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                            { let v_ = ((c.m.r32((fp.wrapping_add(8)).wrapping_add(4)) as i32)).wrapping_add(64i32); let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                            { let v_ = ((c.m.r32(fp.wrapping_add(8)) as i32)).wrapping_add(64i32); let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                            F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), (if ((l_i_3) != 0) { sym::menu_k_e__str39 } else { sym::menu_k_e__str40 }), fp);
                            F::Flash_ClipSetTex(c, l_flash_2, fp, 1i32);
                            F::Flash_ClipSetUv(c, l_flash_2, fp, fp.wrapping_add(8));
                        }
                    }
                    { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
                }
            }
            l_x_10 = ({ let a0_ = { let a_ = (((p_b_1).wrapping_add(6760)).wrapping_add(((({ let a_ = ((((p_b_1).wrapping_add(7440)).wrapping_add(((((c.m.r32((p_b_1).wrapping_add(6728)) as i32)) as u32)).wrapping_mul(16))).wrapping_add(2)).wrapping_add((((0i32) as u32)).wrapping_mul(2)); c.m.r16(a_) }) as u32)).wrapping_mul(40))).wrapping_add(16); (c.m.r32(a_) as i32) }; F::Bracket_GetPosX(c, a0_) }).wrapping_add(rt::f2i(F32(c.m.r32((p_b_1).wrapping_add(6740)))));
            l_y_11 = { let a0_ = { let a_ = (((p_b_1).wrapping_add(6760)).wrapping_add(((({ let a_ = ((((p_b_1).wrapping_add(7440)).wrapping_add(((((c.m.r32((p_b_1).wrapping_add(6728)) as i32)) as u32)).wrapping_mul(16))).wrapping_add(2)).wrapping_add((((0i32) as u32)).wrapping_mul(2)); c.m.r16(a_) }) as u32)).wrapping_mul(40))).wrapping_add(16); (c.m.r32(a_) as i32) }; F::Bracket_GetPosY(c, a0_) };
            F::Flash_SetOffset(c, l_flash_2, l_x_10, l_y_11);
        }
        l_flash_2 = ((p_b_1).wrapping_add(48)).wrapping_add((((4i32) as u32)).wrapping_mul(44));
        if ((((c.m.r32((((p_b_1).wrapping_add(48)).wrapping_add((((4i32) as u32)).wrapping_mul(44))).wrapping_add(8))) & (((1i32) as u32)))) != 0) {
            {
                l_i_3 = 0i32;
                'l16: loop {
                    if !(((((l_i_3) < (2i32)) as i32)) != 0) { break 'l16; }
                    'c17: {
                        l_chara_12 = { let a_ = (((p_b_1).wrapping_add(6760)).wrapping_add(((({ let a_ = ((((p_b_1).wrapping_add(7440)).wrapping_add(((((c.m.r32((p_b_1).wrapping_add(6728)) as i32)) as u32)).wrapping_mul(16))).wrapping_add(2)).wrapping_add((((l_i_3) as u32)).wrapping_mul(2)); c.m.r16(a_) }) as u32)).wrapping_mul(40))).wrapping_add(4); (c.m.r32(a_) as i32) };
                        l_cols_13 = 5i32;
                        if (((((({ let a_ = ((p_b_1).wrapping_add(6760)).wrapping_add(((({ let a_ = ((((p_b_1).wrapping_add(7440)).wrapping_add(((((c.m.r32((p_b_1).wrapping_add(6728)) as i32)) as u32)).wrapping_mul(16))).wrapping_add(2)).wrapping_add((((l_i_3) as u32)).wrapping_mul(2)); c.m.r16(a_) }) as u32)).wrapping_mul(40)); c.m.r16(a_) }) as i32)) & (16i32))) != 0) {
                            l_chara_12 = 163i32;
                        } else {
                            's18: {
                                let sv16: i32 = l_chara_12;
                                match sv16 {
                                    22i32 | 23i32 => {
                                        l_chara_12 = 167i32;
                                        break 's18;
                                    }
                                    58i32 | 59i32 => {
                                        l_chara_12 = 165i32;
                                        break 's18;
                                    }
                                    _ => {
                                    }
                                }
                            }
                        }
                        F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), (if ((l_i_3) != 0) { sym::menu_k_e__str41 } else { sym::menu_k_e__str42 }), fp);
                        F::TextBox_AttachLine(c, l_flash_2, fp, 0i32, 0i32, l_chara_12, ((p_b_1).wrapping_add(7696)).wrapping_add((((l_i_3) as u32)).wrapping_mul(140)));
                        F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), (if ((l_i_3) != 0) { sym::menu_k_e__str43 } else { sym::menu_k_e__str44 }), fp);
                        F::TextBox_AttachLine(c, l_flash_2, fp, 0i32, 0i32, l_chara_12, ((p_b_1).wrapping_add(7976)).wrapping_add((((l_i_3) as u32)).wrapping_mul(140)));
                        if (((((({ let a_ = ((p_b_1).wrapping_add(6760)).wrapping_add(((({ let a_ = ((((p_b_1).wrapping_add(7440)).wrapping_add(((((c.m.r32((p_b_1).wrapping_add(6728)) as i32)) as u32)).wrapping_mul(16))).wrapping_add(2)).wrapping_add((((l_i_3) as u32)).wrapping_mul(2)); c.m.r16(a_) }) as u32)).wrapping_mul(40)); c.m.r16(a_) }) as i32)) & (1i32))) != 0) {
                            { let v_ = (rt::rem_i32({ let a_ = (((p_b_1).wrapping_add(6760)).wrapping_add(((({ let a_ = ((((p_b_1).wrapping_add(7440)).wrapping_add(((((c.m.r32((p_b_1).wrapping_add(6728)) as i32)) as u32)).wrapping_mul(16))).wrapping_add(2)).wrapping_add((((l_i_3) as u32)).wrapping_mul(2)); c.m.r16(a_) }) as u32)).wrapping_mul(40))).wrapping_add(12); (c.m.r32(a_) as i32) }, l_cols_13)).wrapping_mul(96i32); let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                            { let v_ = (rt::div_i32({ let a_ = (((p_b_1).wrapping_add(6760)).wrapping_add(((({ let a_ = ((((p_b_1).wrapping_add(7440)).wrapping_add(((((c.m.r32((p_b_1).wrapping_add(6728)) as i32)) as u32)).wrapping_mul(16))).wrapping_add(2)).wrapping_add((((l_i_3) as u32)).wrapping_mul(2)); c.m.r16(a_) }) as u32)).wrapping_mul(40))).wrapping_add(12); (c.m.r32(a_) as i32) }, l_cols_13)).wrapping_mul(64i32); let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                            { let v_ = ((c.m.r32(fp.wrapping_add(8)) as i32)).wrapping_add(96i32); let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                            { let v_ = ((c.m.r32((fp.wrapping_add(8)).wrapping_add(4)) as i32)).wrapping_add(64i32); let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                        } else {
                            { let v_ = 288i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                            { let v_ = 64i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                            { let v_ = 384i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                            { let v_ = 128i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                        }
                        F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), (if ((l_i_3) != 0) { sym::menu_k_e__str45 } else { sym::menu_k_e__str46 }), fp);
                        F::Flash_ClipSetUv(c, l_flash_2, fp, fp.wrapping_add(8));
                        if ((((((c.m.r32((p_b_1).wrapping_add(6724)) as i32)) == (4i32)) as i32)) != 0) {
                            l_plate_14 = (if ((((((c.m.r32((p_b_1).wrapping_add(740)) as i32)) == (l_i_3)) as i32)) != 0) { 3i32 } else { 2i32 });
                        } else {
                            l_plate_14 = ((((c.m.r32((p_b_1).wrapping_add(740)) as i32)) == (l_i_3)) as i32);
                        }
                        { let v_ = 256i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                        { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                        { let v_ = (l_plate_14).wrapping_mul(64i32); let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                        { let v_ = ((c.m.r32((fp.wrapping_add(8)).wrapping_add(4)) as i32)).wrapping_add(64i32); let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                        F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), (if ((l_i_3) != 0) { sym::menu_k_e__str47 } else { sym::menu_k_e__str48 }), fp);
                        F::Flash_ClipSetUv(c, l_flash_2, fp, fp.wrapping_add(8));
                    }
                    { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
                }
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Bracket_SetupDraw", &PROBE_N, &[], true); }
    r_
}
