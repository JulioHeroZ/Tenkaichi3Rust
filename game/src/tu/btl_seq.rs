#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn BtlText_DrawPart(c: &mut Ctx, mut p_pkt_1: u32, mut p_x_2: i32, mut p_y_3: i32, mut p_w_4: i32, mut p_h_5: i32, mut p_part_6: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlText_DrawPart", &PROBE_N, &[(p_pkt_1) as u32, (p_x_2) as u32, (p_y_3) as u32, (p_w_4) as u32, (p_h_5) as u32, (p_part_6) as u32], false); }
    let r_ = 'f: {
    F::BtlText_PutSprite(c, p_pkt_1, p_x_2, p_y_3, (p_x_2).wrapping_add(p_w_4), (p_y_3).wrapping_add(p_h_5), 0i32, 0i32, p_w_4, (if (((((p_h_5) < (0i32)) as i32)) != 0) { (p_h_5).wrapping_neg() } else { p_h_5 }), 2155905152u32, p_part_6);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlText_DrawPart", &PROBE_N, &[], true); }
    r_
}

pub fn BtlText_DrawPageIcon(c: &mut Ctx, mut p_pkt_1: u32, mut p_x_2: i32, mut p_y_3: i32, mut p_digit_4: u16) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlText_DrawPageIcon", &PROBE_N, &[(p_pkt_1) as u32, (p_x_2) as u32, (p_y_3) as u32, (p_digit_4) as u32], false); }
    let mut l_w_5: i32 = 0;
    let mut l_idx_6: i32 = 0;
    let mut l_part_7: i32 = 0;
    let r_ = 'f: {
    l_w_5 = F::Font_GetGlyphHeight(c);
    l_idx_6 = (((p_digit_4) as i32)).wrapping_sub(48i32);
    l_part_7 = 0i32;
    { let r_ = 20i32; let n_ = (l_w_5).wrapping_mul(r_); l_w_5 = n_; };
    's1: {
        let sv8: i32 = l_idx_6;
        match sv8 {
            0i32 => {
                l_part_7 = 0i32;
                break 's1;
            }
            1i32 => {
                l_part_7 = 1i32;
                break 's1;
            }
            2i32 => {
                l_part_7 = 2i32;
                break 's1;
            }
            3i32 => {
                l_part_7 = 3i32;
                break 's1;
            }
            4i32 => {
                l_part_7 = 4i32;
                break 's1;
            }
            5i32 => {
                l_part_7 = 5i32;
                break 's1;
            }
            6i32 => {
                l_part_7 = 6i32;
                break 's1;
            }
            7i32 | 8i32 => {
                l_part_7 = 7i32;
                break 's1;
            }
            _ => {
            }
        }
    }
    F::BtlText_DrawPart(c, p_pkt_1, p_x_2, p_y_3, l_w_5, 32i32, (l_part_7).wrapping_add(7i32));
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlText_DrawPageIcon", &PROBE_N, &[], true); }
    r_
}

pub fn BtlText_CheckLineMask(c: &mut Ctx, mut p_cursor_1: u32, mut p_side_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlText_CheckLineMask", &PROBE_N, &[(p_cursor_1) as u32, (p_side_2) as u32], false); }
    let mut l_work_3: u32 = 0;
    let mut l_p_4: u32 = 0;
    let mut l_mask_5: u32 = 0;
    let mut l_shift_6: i32 = 0;
    let r_ = 'f: {
    l_work_3 = F::BtlMenu_GetWork(c);
    l_p_4 = c.m.r32(p_cursor_1);
    l_mask_5 = ((0i32) as u32);
    's1: {
        let sv7: i32 = ((c.m.r16((l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(2)))) as i32);
        match sv7 {
            70i32 => {
                l_mask_5 = ((15i32) as u32);
                break 's1;
            }
            49i32 => {
                l_mask_5 = ((1i32) as u32);
                break 's1;
            }
            50i32 => {
                l_mask_5 = ((2i32) as u32);
                break 's1;
            }
            52i32 => {
                l_mask_5 = ((4i32) as u32);
                break 's1;
            }
            56i32 => {
                l_mask_5 = ((8i32) as u32);
                break 's1;
            }
            69i32 => {
                l_mask_5 = ((14i32) as u32);
                break 's1;
            }
            68i32 => {
                l_mask_5 = ((13i32) as u32);
                break 's1;
            }
            66i32 => {
                l_mask_5 = ((11i32) as u32);
                break 's1;
            }
            55i32 => {
                l_mask_5 = ((7i32) as u32);
                break 's1;
            }
            _ => {
            }
        }
    }
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(2)); l_p_4 = n_; };
    l_shift_6 = (c.m.r32((l_work_3).wrapping_add(16)) as i32);
    { let v_ = l_p_4; let a_ = p_cursor_1; c.m.w32(a_, v_); };
    break 'f (((((l_mask_5).wrapping_shr((l_shift_6) as u32)) & (((1i32) as u32)))) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlText_CheckLineMask", &PROBE_N, &[], true); }
    r_
}

pub fn BtlText_CheckUnlock(c: &mut Ctx, mut p_cursor_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlText_CheckUnlock", &PROBE_N, &[(p_cursor_1) as u32], false); }
    let mut l_work_2: u32 = 0;
    let mut l_ret_3: i32 = 0;
    let mut l_p_4: u32 = 0;
    let mut l_id_5: i32 = 0;
    let r_ = 'f: {
    l_work_2 = F::BtlMenu_GetWork(c);
    l_ret_3 = 1i32;
    l_p_4 = c.m.r32(p_cursor_1);
    if (((((((c.m.r16((l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(2)))) as i32)) == (38i32)) as i32)) != 0) {
        l_id_5 = ((((c.m.r16((l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(2)))) as i32)).wrapping_sub(48i32)).wrapping_mul(100i32);
        { let r_ = ((((c.m.r16((l_p_4).wrapping_add((((2i32) as u32)).wrapping_mul(2)))) as i32)).wrapping_sub(48i32)).wrapping_mul(10i32); let n_ = (l_id_5).wrapping_add(r_); l_id_5 = n_; };
        { let r_ = (((c.m.r16((l_p_4).wrapping_add((((3i32) as u32)).wrapping_mul(2)))) as i32)).wrapping_sub(48i32); let n_ = (l_id_5).wrapping_add(r_); l_id_5 = n_; };
        { let r_ = 4i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(2)); l_p_4 = n_; };
        if (((((l_id_5) == (86i32)) as i32)) != 0) {
            if ((((({ let a0_ = (c.m.r32((l_work_2).wrapping_add(12)) as i32); F::BtlCtrl_TestMemberUnk70(c, a0_) }) != (0i32)) as i32)) != 0) {
                l_ret_3 = 0i32;
            }
        } else {
            l_ret_3 = ((((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(((((rt::div_i32(3088i32, 8i32)).wrapping_add(rt::div_i32(l_id_5, 64i32))) as u32)).wrapping_mul(8)); c.m.r64(a_) }).wrapping_shr((rt::rem_i32(l_id_5, 64i32)) as u32)) & (((1i32) as u64)))) as i32);
        }
    }
    { let v_ = l_p_4; let a_ = p_cursor_1; c.m.w32(a_, v_); };
    break 'f l_ret_3;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlText_CheckUnlock", &PROBE_N, &[], true); }
    r_
}

pub fn btl_seq__BtlText_IsLineDimmed(c: &mut Ctx, mut p_page_1: i32, mut p_entry_2: i32, mut p_side_3: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "btl_seq__BtlText_IsLineDimmed", &PROBE_N, &[(p_page_1) as u32, (p_entry_2) as u32, (p_side_3) as u32], false); }
    let r_ = 'f: {
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "btl_seq__BtlText_IsLineDimmed", &PROBE_N, &[], true); }
    r_
}

pub fn BtlText_DrawList(c: &mut Ctx, mut p_pkt_1: u32, mut p_x0_2: i32, mut p_x1_3: i32, mut p_y0_4: i32, mut p_y1_5: i32, mut p_mode_6: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlText_DrawList", &PROBE_N, &[(p_pkt_1) as u32, (p_x0_2) as u32, (p_x1_3) as u32, (p_y0_4) as u32, (p_y1_5) as u32, (p_mode_6) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_work_7: u32 = 0;
    let mut l_side_8: i32 = 0;
    let mut l_lineH_9: i32 = 0;
    let mut l_go_10: i32 = 0;
    let mut l_page_11: i32 = 0;
    let mut l_entry_12: i32 = 0;
    let mut l_y_13: i32 = 0;
    let mut l_list_14: u32 = 0;
    let mut l_fit_15: F32 = F32(0);
    let mut l_tag_16: i32 = 0;
    let mut l_line_17: u32 = 0;
    let mut l_h_18: i32 = 0;
    let r_ = 'f: {
        l_work_7 = F::BtlMenu_GetWork(c);
        l_go_10 = 1i32;
        l_page_11 = (1i32).wrapping_neg();
        l_side_8 = (c.m.r32((l_work_7).wrapping_add(12)) as i32);
        l_y_13 = p_y0_4;
        l_entry_12 = 0i32;
        l_list_14 = ((l_work_7).wrapping_add(32)).wrapping_add((((l_side_8) as u32)).wrapping_mul(128));
        { let v_ = F::BtlMenu_GetScript(c); let a_ = fp.wrapping_add(12); c.m.w32(a_, v_); };
        l_lineH_9 = (F::Font_GetGlyphHeight(c)).wrapping_mul(20i32);
        if (((((c.m.r32(fp.wrapping_add(12))) == (((0i32) as u32))) as i32)) != 0) {
            break 'f;
        }
        if (((((p_mode_6) == (0i32)) as i32)) != 0) {
            { let r_ = ({ let a_ = ((l_list_14).wrapping_add(48)).wrapping_add(((((c.m.r32((l_list_14).wrapping_add(44)) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_mul(28i32); let n_ = (l_y_13).wrapping_sub(r_); l_y_13 = n_; };
        }
        F::Font_PushStyle(c);
        F::Font_SetFlags(c, 1i32);
        F::Font_SetScaleXY(c, F32(0x3f800000), F32(0x3f800000));
        { let _ = F::Battle_IsSplitScreen(c); };
        F::Font_SetShadowMode(c, 2i32);
        F::Font_SetColorRGBA(c, ((((255i32) as i64 as u64)) as u8), ((((255i32) as i64 as u64)) as u8), ((((255i32) as i64 as u64)) as u8), 128i32);
        F::Font_SetShadowColorRGBA(c, ((((0i32) as i64 as u64)) as u8), ((((16i32) as i64 as u64)) as u8), ((((16i32) as i64 as u64)) as u8), 64i32);
        if (((((p_mode_6) == (0i32)) as i32)) != 0) {
            F::Font_SetClip(c, p_x0_2, p_y0_4, p_x1_3, p_y1_5);
            F::BtlText_PutScissor(c, p_pkt_1, p_x0_2, p_y0_4, p_x1_3, p_y1_5);
        } else {
            F::Font_SetClip(c, p_x0_2, (p_y0_4).wrapping_sub(7i32), p_x1_3, (p_y1_5).wrapping_add(7i32));
            F::BtlText_PutScissor(c, p_pkt_1, p_x0_2, (p_y0_4).wrapping_sub(7i32), p_x1_3, (p_y1_5).wrapping_add(7i32));
        }
        { let r_ = 1i32; let a_ = fp.wrapping_add(12); let n_ = (c.m.r32(a_)).wrapping_add((((r_) as u32)).wrapping_mul(2)); c.m.w32(a_, n_); };
        'l1: while ((((((l_go_10) != 0) && ((((((({ let a_ = c.m.r32(fp.wrapping_add(12)); c.m.r16(a_) }) as i32)) != (0i32)) as i32)) != 0)) as i32)) != 0) {
            if (((((F::BtlText_CheckLineMask(c, fp.wrapping_add(12), l_side_8)) == (0i32)) as i32)) != 0) {
                { let v_ = { let a0_ = c.m.r32(fp.wrapping_add(12)); F::BtlText_NextLine(c, a0_) }; let a_ = fp.wrapping_add(12); c.m.w32(a_, v_); };
                continue 'l1;
            }
            if (((((F::BtlText_CheckUnlock(c, fp.wrapping_add(12))) == (0i32)) as i32)) != 0) {
                { let v_ = F32(0x3f000000); let a_ = sym::gBtlTextAlpha; c.m.w32(a_, (v_).0); };
            }
            l_line_17 = c.m.r32(fp.wrapping_add(12));
            l_tag_16 = ((c.m.r16(l_line_17)) as i32);
            's2: {
                let sv19: i32 = l_tag_16;
                match sv19 {
                    36i32 => {
                        l_page_11 = (((c.m.r16((l_line_17).wrapping_add((((1i32) as u32)).wrapping_mul(2)))) as i32)).wrapping_sub(48i32);
                        l_entry_12 = (1i32).wrapping_neg();
                        break 's2;
                    }
                    42i32 => {
                        { let o_ = l_entry_12; let n_ = (o_).wrapping_add(1); l_entry_12 = n_; };
                        break 's2;
                    }
                    _ => {
                    }
                }
            }
            if (((((((((l_tag_16) != (64i32)) as i32)) != 0) && (((((l_page_11) != ((c.m.r32((l_list_14).wrapping_add(44)) as i32))) as i32)) != 0)) as i32)) != 0) {
                { let v_ = F::BtlText_NextLine(c, l_line_17); let a_ = fp.wrapping_add(12); c.m.w32(a_, v_); };
                continue 'l1;
            }
            if (((((((((l_tag_16) != (36i32)) as i32)) != 0) && (((((F::btl_seq__BtlText_IsLineDimmed(c, l_page_11, l_entry_12, l_side_8)) != (0i32)) as i32)) != 0)) as i32)) != 0) {
                { let v_ = F32(0x3f000000); let a_ = sym::gBtlTextAlpha; c.m.w32(a_, (v_).0); };
            }
            if (((((p_mode_6) == (0i32)) as i32)) != 0) {
                's3: {
                    let sv20: i32 = l_tag_16;
                    match sv20 {
                        36i32 => {
                            F::Font_PushStyle(c);
                            F::BtlText_PutScissor(c, p_pkt_1, 0i32, 0i32, 511i32, 447i32);
                            F::Font_SetClip(c, 0i32, 0i32, 511i32, 447i32);
                            F::Font_SetColorRGBA(c, ((((255i32) as i64 as u64)) as u8), ((((255i32) as i64 as u64)) as u8), ((((255i32) as i64 as u64)) as u8), 128i32);
                            F::Font_SetShadowColorRGBA(c, ((((86i32) as i64 as u64)) as u8), ((((61i32) as i64 as u64)) as u8), ((((57i32) as i64 as u64)) as u8), 128i32);
                            { let a2_ = (c.m.r32(fp.wrapping_add(12))).wrapping_add((((2i32) as u32)).wrapping_mul(2)); F::Font_PrintAt(c, (p_x0_2).wrapping_add(30i32), (p_y0_4).wrapping_sub(34i32), a2_) };
                            { let a3_ = { let a_ = (c.m.r32(fp.wrapping_add(12))).wrapping_add((((1i32) as u32)).wrapping_mul(2)); c.m.r16(a_) }; F::BtlText_DrawPageIcon(c, p_pkt_1, p_x0_2, (p_y0_4).wrapping_sub(40i32), a3_) };
                            F::BtlText_PutScissor(c, p_pkt_1, p_x0_2, p_y0_4, p_x1_3, p_y1_5);
                            F::Font_PopStyle(c);
                            break 's3;
                        }
                        42i32 => {
                            if (((((((((p_y0_4) < ((l_y_13).wrapping_add(32i32))) as i32)) != 0) && (((((l_y_13) < (p_y1_5)) as i32)) != 0)) as i32)) != 0) {
                                { let v_ = ((({ let a_ = (c.m.r32(fp.wrapping_add(12))).wrapping_add((((1i32) as u32)).wrapping_mul(2)); c.m.r16(a_) }) as i32)).wrapping_sub(48i32); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                { let v_ = ((({ let a_ = (c.m.r32(fp.wrapping_add(12))).wrapping_add((((2i32) as u32)).wrapping_mul(2)); c.m.r16(a_) }) as i32)).wrapping_sub(48i32); let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                { let v_ = ((({ let a_ = (c.m.r32(fp.wrapping_add(12))).wrapping_add((((3i32) as u32)).wrapping_mul(2)); c.m.r16(a_) }) as i32)).wrapping_sub(48i32); let a_ = (fp).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                if (((((l_entry_12) == ({ let a_ = ((l_list_14).wrapping_add(88)).wrapping_add(((((c.m.r32((l_list_14).wrapping_add(44)) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                                    { let a0_ = ((((rt::f2u(rt::fmul(F32(c.m.r32(sym::gBtlTextAlpha)), F32(0x437f0000)))) as u8)) as i32); let a1_ = ((((rt::f2u(rt::fmul(F32(c.m.r32(sym::gBtlTextAlpha)), F32(0x43770000)))) as u8)) as i32); let a2_ = ((((rt::f2u(rt::fmul(F32(c.m.r32(sym::gBtlTextAlpha)), F32(0x41700000)))) as u8)) as i32); F::Font_SetColorRGBA(c, ((((a0_) as i64 as u64)) as u8), ((((a1_) as i64 as u64)) as u8), ((((a2_) as i64 as u64)) as u8), 128i32) };
                                    { let a0_ = ((((rt::f2u(rt::fmul(F32(c.m.r32(sym::gBtlTextAlpha)), F32(0x41e00000)))) as u8)) as i32); let a1_ = ((((rt::f2u(rt::fmul(F32(c.m.r32(sym::gBtlTextAlpha)), F32(0x42000000)))) as u8)) as i32); let a2_ = ((((rt::f2u(rt::fmul(F32(c.m.r32(sym::gBtlTextAlpha)), F32(0x41a80000)))) as u8)) as i32); F::Font_SetShadowColorRGBA(c, ((((a0_) as i64 as u64)) as u8), ((((a1_) as i64 as u64)) as u8), ((((a2_) as i64 as u64)) as u8), 64i32) };
                                } else {
                                    { let a0_ = ((((rt::f2u(rt::fmul(F32(c.m.r32(sym::gBtlTextAlpha)), F32(0x437f0000)))) as u8)) as i32); let a1_ = ((((rt::f2u(rt::fmul(F32(c.m.r32(sym::gBtlTextAlpha)), F32(0x437f0000)))) as u8)) as i32); let a2_ = ((((rt::f2u(rt::fmul(F32(c.m.r32(sym::gBtlTextAlpha)), F32(0x437f0000)))) as u8)) as i32); F::Font_SetColorRGBA(c, ((((a0_) as i64 as u64)) as u8), ((((a1_) as i64 as u64)) as u8), ((((a2_) as i64 as u64)) as u8), 128i32) };
                                    { let a0_ = ((((rt::f2u(rt::fmul(F32(c.m.r32(sym::gBtlTextAlpha)), F32(0x42000000)))) as u8)) as i32); let a1_ = ((((rt::f2u(rt::fmul(F32(c.m.r32(sym::gBtlTextAlpha)), F32(0x422c0000)))) as u8)) as i32); let a2_ = ((((rt::f2u(rt::fmul(F32(c.m.r32(sym::gBtlTextAlpha)), F32(0x42bc0000)))) as u8)) as i32); F::Font_SetShadowColorRGBA(c, ((((a0_) as i64 as u64)) as u8), ((((a1_) as i64 as u64)) as u8), ((((a2_) as i64 as u64)) as u8), 64i32) };
                                }
                                l_fit_15 = { let a0_ = (c.m.r32(fp.wrapping_add(12))).wrapping_add((((2i32) as u32)).wrapping_mul(2)); F::Font_FitScale(c, a0_, ((p_x1_3).wrapping_sub(p_x0_2)).wrapping_sub(68i32)) };
                                if (((rt::flt(l_fit_15, F32(0x3f800000)) as i32)) != 0) {
                                    F::Font_PushStyle(c);
                                    F::Font_GetScaleXY(c, fp.wrapping_add(16), fp.wrapping_add(20));
                                    { let a0_ = rt::fmul(F32(c.m.r32(fp.wrapping_add(16))), l_fit_15); let a1_ = F32(c.m.r32(fp.wrapping_add(20))); F::Font_SetScaleXY(c, a0_, a1_) };
                                    { let a2_ = (c.m.r32(fp.wrapping_add(12))).wrapping_add((((4i32) as u32)).wrapping_mul(2)); F::Font_PrintAt(c, (p_x0_2).wrapping_add(15i32), (l_y_13).wrapping_add(3i32), a2_) };
                                    F::Font_PopStyle(c);
                                } else {
                                    { let a2_ = (c.m.r32(fp.wrapping_add(12))).wrapping_add((((4i32) as u32)).wrapping_mul(2)); F::Font_PrintAt(c, (p_x0_2).wrapping_add(15i32), (l_y_13).wrapping_add(3i32), a2_) };
                                }
                                l_h_18 = (l_lineH_9).wrapping_sub(2i32);
                                if (((((l_entry_12) == ({ let a_ = ((l_list_14).wrapping_add(88)).wrapping_add(((((c.m.r32((l_list_14).wrapping_add(44)) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                                    F::BtlText_DrawPart(c, p_pkt_1, (p_x0_2).wrapping_sub(1i32), l_y_13, l_h_18, 31i32, 17i32);
                                } else {
                                    F::BtlText_DrawPart(c, p_pkt_1, (p_x0_2).wrapping_sub(1i32), l_y_13, l_h_18, 31i32, 16i32);
                                }
                                if ((((((c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) > (0i32)) as i32)) != 0) {
                                    F::BtlText_DrawPart(c, p_pkt_1, (p_x1_3).wrapping_sub(53i32), (l_y_13).wrapping_sub(5i32), l_h_18, 31i32, 19i32);
                                }
                                if ((((((c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) > (0i32)) as i32)) != 0) {
                                    { let a5_ = ((c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)).wrapping_add(28i32); F::BtlText_DrawPart(c, p_pkt_1, (p_x1_3).wrapping_sub(32i32), (l_y_13).wrapping_sub(5i32), l_h_18, 31i32, a5_) };
                                }
                                if ((((((c.m.r32((fp).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32)) > (0i32)) as i32)) != 0) {
                                    { let a5_ = ((c.m.r32((fp).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32)).wrapping_add(19i32); F::BtlText_DrawPart(c, p_pkt_1, (p_x1_3).wrapping_sub(32i32), (l_y_13).wrapping_sub(5i32), l_h_18, 31i32, a5_) };
                                }
                            }
                            { let r_ = 28i32; let n_ = (l_y_13).wrapping_add(r_); l_y_13 = n_; };
                            break 's3;
                        }
                        64i32 => {
                            l_go_10 = 0i32;
                            break 's3;
                        }
                        _ => {
                        }
                    }
                }
            } else {
                's4: {
                    let sv21: i32 = l_tag_16;
                    match sv21 {
                        37i32 => {
                            's5: {
                                let sv22: i32 = ((c.m.r16((l_line_17).wrapping_add((((1i32) as u32)).wrapping_mul(2)))) as i32);
                                match sv22 {
                                    48i32 => {
                                        if (((((l_entry_12) != ({ let a_ = ((l_list_14).wrapping_add(88)).wrapping_add(((((c.m.r32((l_list_14).wrapping_add(44)) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                                            break 's5;
                                        }
                                        { let a0_ = ((((rt::f2u(rt::fmul(F32(c.m.r32(sym::gBtlTextAlpha)), F32(0x432a0000)))) as u8)) as i32); let a1_ = ((((rt::f2u(rt::fmul(F32(c.m.r32(sym::gBtlTextAlpha)), F32(0x43570000)))) as u8)) as i32); let a2_ = ((((rt::f2u(rt::fmul(F32(c.m.r32(sym::gBtlTextAlpha)), F32(0x437f0000)))) as u8)) as i32); F::Font_SetColorRGBA(c, ((((a0_) as i64 as u64)) as u8), ((((a1_) as i64 as u64)) as u8), ((((a2_) as i64 as u64)) as u8), 128i32) };
                                        { let a0_ = ((((rt::f2u(rt::fmul(F32(c.m.r32(sym::gBtlTextAlpha)), F32(0x422c0000)))) as u8)) as i32); let a1_ = ((((rt::f2u(rt::fmul(F32(c.m.r32(sym::gBtlTextAlpha)), F32(0x42580000)))) as u8)) as i32); let a2_ = ((((rt::f2u(rt::fmul(F32(c.m.r32(sym::gBtlTextAlpha)), F32(0x42800000)))) as u8)) as i32); F::Font_SetShadowColorRGBA(c, ((((a0_) as i64 as u64)) as u8), ((((a1_) as i64 as u64)) as u8), ((((a2_) as i64 as u64)) as u8), 64i32) };
                                        l_fit_15 = { let a0_ = (c.m.r32(fp.wrapping_add(12))).wrapping_add((((2i32) as u32)).wrapping_mul(2)); F::Font_FitScale(c, a0_, ((p_x1_3).wrapping_sub(p_x0_2)).wrapping_sub(30i32)) };
                                        if (((rt::flt(l_fit_15, F32(0x3f800000)) as i32)) != 0) {
                                            F::Font_PushStyle(c);
                                            F::Font_GetScaleXY(c, fp.wrapping_add(16), fp.wrapping_add(20));
                                            { let a0_ = rt::fmul(F32(c.m.r32(fp.wrapping_add(16))), l_fit_15); let a1_ = F32(c.m.r32(fp.wrapping_add(20))); F::Font_SetScaleXY(c, a0_, a1_) };
                                            { let a2_ = (c.m.r32(fp.wrapping_add(12))).wrapping_add((((2i32) as u32)).wrapping_mul(2)); F::Font_PrintAt(c, (p_x0_2).wrapping_add(15i32), (l_y_13).wrapping_add(3i32), a2_) };
                                            F::Font_PopStyle(c);
                                        } else {
                                            { let a2_ = (c.m.r32(fp.wrapping_add(12))).wrapping_add((((2i32) as u32)).wrapping_mul(2)); F::Font_PrintAt(c, (p_x0_2).wrapping_add(15i32), (l_y_13).wrapping_add(3i32), a2_) };
                                        }
                                        { let r_ = 28i32; let n_ = (l_y_13).wrapping_add(r_); l_y_13 = n_; };
                                        break 's5;
                                    }
                                    49i32 => {
                                        if (((((l_entry_12) != ({ let a_ = ((l_list_14).wrapping_add(88)).wrapping_add(((((c.m.r32((l_list_14).wrapping_add(44)) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                                            break 's5;
                                        }
                                        { let a0_ = ((((rt::f2u(rt::fmul(F32(c.m.r32(sym::gBtlTextAlpha)), F32(0x436f0000)))) as u8)) as i32); let a1_ = ((((rt::f2u(rt::fmul(F32(c.m.r32(sym::gBtlTextAlpha)), F32(0x42b20000)))) as u8)) as i32); let a2_ = ((((rt::f2u(rt::fmul(F32(c.m.r32(sym::gBtlTextAlpha)), F32(0x42b20000)))) as u8)) as i32); F::Font_SetColorRGBA(c, ((((a0_) as i64 as u64)) as u8), ((((a1_) as i64 as u64)) as u8), ((((a2_) as i64 as u64)) as u8), 128i32) };
                                        { let a0_ = ((((rt::f2u(rt::fmul(F32(c.m.r32(sym::gBtlTextAlpha)), F32(0x42800000)))) as u8)) as i32); let a1_ = ((((rt::f2u(rt::fmul(F32(c.m.r32(sym::gBtlTextAlpha)), F32(0x41880000)))) as u8)) as i32); let a2_ = ((((rt::f2u(rt::fmul(F32(c.m.r32(sym::gBtlTextAlpha)), F32(0x00000000)))) as u8)) as i32); F::Font_SetShadowColorRGBA(c, ((((a0_) as i64 as u64)) as u8), ((((a1_) as i64 as u64)) as u8), ((((a2_) as i64 as u64)) as u8), 64i32) };
                                        l_fit_15 = { let a0_ = (c.m.r32(fp.wrapping_add(12))).wrapping_add((((2i32) as u32)).wrapping_mul(2)); F::Font_FitScale(c, a0_, ((p_x1_3).wrapping_sub(p_x0_2)).wrapping_sub(30i32)) };
                                        if (((rt::flt(l_fit_15, F32(0x3f800000)) as i32)) != 0) {
                                            F::Font_PushStyle(c);
                                            F::Font_GetScaleXY(c, fp.wrapping_add(16), fp.wrapping_add(20));
                                            { let a0_ = rt::fmul(F32(c.m.r32(fp.wrapping_add(16))), l_fit_15); let a1_ = F32(c.m.r32(fp.wrapping_add(20))); F::Font_SetScaleXY(c, a0_, a1_) };
                                            { let a2_ = (c.m.r32(fp.wrapping_add(12))).wrapping_add((((2i32) as u32)).wrapping_mul(2)); F::Font_PrintAt(c, (p_x0_2).wrapping_add(15i32), (l_y_13).wrapping_add(3i32), a2_) };
                                            F::Font_PopStyle(c);
                                        } else {
                                            { let a2_ = (c.m.r32(fp.wrapping_add(12))).wrapping_add((((2i32) as u32)).wrapping_mul(2)); F::Font_PrintAt(c, (p_x0_2).wrapping_add(15i32), (l_y_13).wrapping_add(3i32), a2_) };
                                        }
                                        { let r_ = 28i32; let n_ = (l_y_13).wrapping_add(r_); l_y_13 = n_; };
                                        break 's5;
                                    }
                                    _ => {
                                    }
                                }
                            }
                            break 's4;
                        }
                        35i32 => {
                            if (((((l_entry_12) == ({ let a_ = ((l_list_14).wrapping_add(88)).wrapping_add(((((c.m.r32((l_list_14).wrapping_add(44)) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                                { let a0_ = ((((rt::f2u(rt::fmul(F32(c.m.r32(sym::gBtlTextAlpha)), F32(0x43000000)))) as u8)) as i32); let a1_ = ((((rt::f2u(rt::fmul(F32(c.m.r32(sym::gBtlTextAlpha)), F32(0x43000000)))) as u8)) as i32); let a2_ = ((((rt::f2u(rt::fmul(F32(c.m.r32(sym::gBtlTextAlpha)), F32(0x43000000)))) as u8)) as i32); F::Font_SetColor2RGBA(c, ((((a0_) as i64 as u64)) as u8), ((((a1_) as i64 as u64)) as u8), ((((a2_) as i64 as u64)) as u8), 128i32) };
                                { let a2_ = (c.m.r32(fp.wrapping_add(12))).wrapping_add((((1i32) as u32)).wrapping_mul(2)); F::Font_PrintAt(c, (p_x0_2).wrapping_add(15i32), l_y_13, a2_) };
                                { let r_ = 28i32; let n_ = (l_y_13).wrapping_add(r_); l_y_13 = n_; };
                                F::Font_SetColor2RGBA(c, ((((128i32) as i64 as u64)) as u8), ((((128i32) as i64 as u64)) as u8), ((((128i32) as i64 as u64)) as u8), 128i32);
                                if ((((((c.m.r32((l_work_7).wrapping_add(16)) as i32)) == (1i32)) as i32)) != 0) {
                                    if ((((({ let a0_ = (c.m.r32(fp.wrapping_add(12))).wrapping_add((((1i32) as u32)).wrapping_mul(2)); F::FontIcon_GetPadTagSize(c, a0_) }) > (32i32)) as i32)) != 0) {
                                        { let r_ = 20i32; let n_ = (l_y_13).wrapping_add(r_); l_y_13 = n_; };
                                    }
                                }
                            }
                            break 's4;
                        }
                        64i32 => {
                            l_go_10 = 0i32;
                            break 's4;
                        }
                        _ => {
                        }
                    }
                }
            }
            { let v_ = F32(0x3f800000); let a_ = sym::gBtlTextAlpha; c.m.w32(a_, (v_).0); };
            { let v_ = { let a0_ = c.m.r32(fp.wrapping_add(12)); F::BtlText_NextLine(c, a0_) }; let a_ = fp.wrapping_add(12); c.m.w32(a_, v_); };
        }
        F::Font_PopStyle(c);
        F::BtlText_PutScissor(c, p_pkt_1, 0i32, 0i32, 511i32, 447i32);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlText_DrawList", &PROBE_N, &[], true); }
    r_
}

pub fn BtlText_DrawScrollBar(c: &mut Ctx, mut p_pkt_1: u32, mut p_unused_2: i32, mut p_x_3: i32, mut p_y0_4: i32, mut p_y1_5: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlText_DrawScrollBar", &PROBE_N, &[(p_pkt_1) as u32, (p_unused_2) as u32, (p_x_3) as u32, (p_y0_4) as u32, (p_y1_5) as u32], false); }
    let mut l_visible_6: i32 = 0;
    let mut l_work_7: u32 = 0;
    let mut l_list_8: u32 = 0;
    let mut l_h_9: i32 = 0;
    let mut l_half_10: i32 = 0;
    let mut l_count_11: i32 = 0;
    let mut l_scroll_12: i32 = 0;
    let mut l_over_13: i32 = 0;
    let mut l_len_14: i32 = 0;
    let mut l_unit_15: F32 = F32(0);
    let r_ = 'f: {
    l_visible_6 = 7i32;
    l_work_7 = F::BtlMenu_GetWork(c);
    l_list_8 = ((l_work_7).wrapping_add(32)).wrapping_add(((((c.m.r32((l_work_7).wrapping_add(12)) as i32)) as u32)).wrapping_mul(128));
    l_h_9 = (p_y1_5).wrapping_sub(p_y0_4);
    l_half_10 = rt::div_i32(l_h_9, 2i32);
    l_count_11 = { let a_ = ((l_list_8).wrapping_add(4)).wrapping_add(((((c.m.r32((l_list_8).wrapping_add(44)) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
    l_scroll_12 = { let a_ = ((l_list_8).wrapping_add(88)).wrapping_add(((((c.m.r32((l_list_8).wrapping_add(44)) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
    F::BtlText_DrawPart(c, p_pkt_1, p_x_3, p_y0_4, 9i32, (l_h_9).wrapping_sub(l_half_10), 5i32);
    F::BtlText_DrawPart(c, p_pkt_1, p_x_3, p_y1_5, 9i32, (l_half_10).wrapping_neg(), 5i32);
    { let r_ = 2i32; let n_ = (p_y0_4).wrapping_add(r_); p_y0_4 = n_; };
    { let r_ = 2i32; let n_ = (p_x_3).wrapping_add(r_); p_x_3 = n_; };
    { let r_ = 2i32; let n_ = (p_y1_5).wrapping_sub(r_); p_y1_5 = n_; };
    l_over_13 = (((l_visible_6) < (l_count_11)) as i32);
    { let o_ = l_count_11; let n_ = (o_).wrapping_sub(1); l_count_11 = n_; };
    if (((((l_count_11) < (0i32)) as i32)) != 0) {
        l_count_11 = 0i32;
    }
    l_len_14 = (p_y1_5).wrapping_sub(p_y0_4);
    if ((l_over_13) != 0) {
        l_unit_15 = rt::fdiv(rt::i2f(l_len_14), rt::i2f((l_count_11).wrapping_add(7i32)));
        { let r_ = rt::f2i(rt::fmul(rt::i2f(l_scroll_12), l_unit_15)); let n_ = (p_y0_4).wrapping_add(r_); p_y0_4 = n_; };
        p_y1_5 = (p_y0_4).wrapping_add(rt::f2i(rt::fmul(l_unit_15, rt::i2f(l_visible_6))));
    }
    l_h_9 = p_y0_4;
    l_h_9 = (p_y1_5).wrapping_sub(l_h_9);
    l_half_10 = rt::div_i32(l_h_9, 2i32);
    F::BtlText_DrawPart(c, p_pkt_1, p_x_3, (p_y0_4).wrapping_sub(27i32), 5i32, ((l_h_9).wrapping_sub(l_half_10)).wrapping_add(27i32), 6i32);
    F::BtlText_DrawPart(c, p_pkt_1, p_x_3, (p_y1_5).wrapping_add(27i32), 5i32, ((l_half_10).wrapping_add(27i32)).wrapping_neg(), 6i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlText_DrawScrollBar", &PROBE_N, &[], true); }
    r_
}

pub fn BtlText_CountEntries(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlText_CountEntries", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_go_1: i32 = 0;
    let mut l_page_2: i32 = 0;
    let mut l_work_3: u32 = 0;
    let mut l_side_4: i32 = 0;
    let mut l_list_5: u32 = 0;
    let mut l_i_6: i32 = 0;
    let mut l_tag_7: i32 = 0;
    let r_ = 'f: {
        l_go_1 = 1i32;
        l_page_2 = (1i32).wrapping_neg();
        l_work_3 = F::BtlMenu_GetWork(c);
        l_side_4 = (c.m.r32((l_work_3).wrapping_add(12)) as i32);
        l_list_5 = ((l_work_3).wrapping_add(32)).wrapping_add((((l_side_4) as u32)).wrapping_mul(128));
        { let v_ = F::BtlMenu_GetScript(c); let a_ = fp; c.m.w32(a_, v_); };
        if (((((c.m.r32(fp)) == (((0i32) as u32))) as i32)) != 0) {
            break 'f;
        }
        { let v_ = 0i32; let a_ = l_list_5; c.m.w32(a_, (v_) as u32); };
        {
            l_i_6 = 9i32;
            'l1: loop {
                if !(((((l_i_6) >= (0i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let v_ = 0i32; let a_ = ((l_list_5).wrapping_add(4)).wrapping_add((((l_i_6) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                }
                { let o_ = l_i_6; let n_ = (o_).wrapping_sub(1); l_i_6 = n_; };
            }
        }
        { let r_ = 1i32; let a_ = fp; let n_ = (c.m.r32(a_)).wrapping_add((((r_) as u32)).wrapping_mul(2)); c.m.w32(a_, n_); };
        'l3: while ((((((l_go_1) != 0) && ((((((({ let a_ = c.m.r32(fp); c.m.r16(a_) }) as i32)) != (0i32)) as i32)) != 0)) as i32)) != 0) {
            if (((((F::BtlText_CheckLineMask(c, fp, l_side_4)) == (0i32)) as i32)) != 0) {
                { let v_ = { let a0_ = c.m.r32(fp); F::BtlText_NextLine(c, a0_) }; let a_ = fp; c.m.w32(a_, v_); };
                continue 'l3;
            }
            { let _ = F::BtlText_CheckUnlock(c, fp); };
            l_tag_7 = (({ let a_ = c.m.r32(fp); c.m.r16(a_) }) as i32);
            if (((((l_tag_7) == (36i32)) as i32)) != 0) {
                l_page_2 = ((({ let a_ = (c.m.r32(fp)).wrapping_add((((1i32) as u32)).wrapping_mul(2)); c.m.r16(a_) }) as i32)).wrapping_sub(48i32);
            }
            's4: {
                let sv8: i32 = l_tag_7;
                match sv8 {
                    36i32 => {
                        { let a_ = l_list_5; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        break 's4;
                    }
                    42i32 => {
                        { let a_ = ((l_list_5).wrapping_add(4)).wrapping_add((((l_page_2) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        break 's4;
                    }
                    64i32 => {
                        l_go_1 = 0i32;
                        break 's4;
                    }
                    _ => {
                    }
                }
            }
            { let v_ = { let a0_ = c.m.r32(fp); F::BtlText_NextLine(c, a0_) }; let a_ = fp; c.m.w32(a_, v_); };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlText_CountEntries", &PROBE_N, &[], true); }
    r_
}

pub fn BtlText_FindEntry(c: &mut Ctx, mut p_n_1: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlText_FindEntry", &PROBE_N, &[(p_n_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_go_2: i32 = 0;
    let mut l_ret_3: u32 = 0;
    let r_ = 'f: {
        l_go_2 = 1i32;
        l_ret_3 = ((0i32) as u32);
        { let v_ = F::BtlMenu_GetScript2(c); let a_ = fp; c.m.w32(a_, v_); };
        if (((((c.m.r32(fp)) == (((0i32) as u32))) as i32)) != 0) {
            break 'f 0;
        }
        { let r_ = 1i32; let a_ = fp; let n_ = (c.m.r32(a_)).wrapping_add((((r_) as u32)).wrapping_mul(2)); c.m.w32(a_, n_); };
        'l1: while ((((((l_go_2) != 0) && ((((((({ let a_ = c.m.r32(fp); c.m.r16(a_) }) as i32)) != (0i32)) as i32)) != 0)) as i32)) != 0) {
            { let r_ = 2i32; let a_ = fp; let n_ = (c.m.r32(a_)).wrapping_add((((r_) as u32)).wrapping_mul(2)); c.m.w32(a_, n_); };
            { let _ = F::BtlText_CheckUnlock(c, fp); };
            's2: {
                let sv4: i32 = (({ let a_ = c.m.r32(fp); c.m.r16(a_) }) as i32);
                match sv4 {
                    42i32 => {
                        if (((((p_n_1) <= (0i32)) as i32)) != 0) {
                            l_ret_3 = (c.m.r32(fp)).wrapping_add((((4i32) as u32)).wrapping_mul(2));
                            l_go_2 = 0i32;
                        }
                        { let o_ = p_n_1; let n_ = (o_).wrapping_sub(1); p_n_1 = n_; };
                        break 's2;
                    }
                    64i32 => {
                        l_go_2 = 0i32;
                        break 's2;
                    }
                    _ => {
                    }
                }
            }
            { let v_ = { let a0_ = c.m.r32(fp); F::BtlText_NextLine(c, a0_) }; let a_ = fp; c.m.w32(a_, v_); };
        }
        break 'f l_ret_3;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlText_FindEntry", &PROBE_N, &[], true); }
    r_
}

pub fn BtlText_DrawEntryName(c: &mut Ctx, mut p_x_1: i32, mut p_y_2: i32, mut p_n_3: i32, mut p_align_4: i32, mut p_alpha_5: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlText_DrawEntryName", &PROBE_N, &[(p_x_1) as u32, (p_y_2) as u32, (p_n_3) as u32, (p_align_4) as u32, (p_alpha_5).0], false); }
    let mut l_str_6: u32 = 0;
    let mut l_font_7: i32 = 0;
    let r_ = 'f: {
    l_str_6 = F::BtlText_FindEntry(c, p_n_3);
    if (((((l_str_6) != (((0i32) as u32))) as i32)) != 0) {
        l_font_7 = F::Font_GetCmdCount(c);
        F::Font_PushStyle(c);
        F::Font_SetFlags(c, 1i32);
        F::Font_SetScale(c, F32(0x3f666666));
        F::Font_SetScaleXY(c, F32(0x3f733333), F32(0x3f800000));
        F::Font_SetColorRGBA(c, ((((255i32) as i64 as u64)) as u8), ((((255i32) as i64 as u64)) as u8), ((((255i32) as i64 as u64)) as u8), ((((rt::f2u(rt::fmul(p_alpha_5, F32(0x43000000)))) as u8)) as i32));
        F::Font_SetShadowMode(c, 2i32);
        F::Font_SetShadowColorRGBA(c, ((((32i32) as i64 as u64)) as u8), ((((32i32) as i64 as u64)) as u8), ((((255i32) as i64 as u64)) as u8), ((((rt::f2u(rt::fmul(p_alpha_5, F32(0x42800000)))) as u8)) as i32));
        's1: {
            let sv8: i32 = p_align_4;
            match sv8 {
                0i32 => {
                    F::Font_SetAlign(c, 0i32);
                    break 's1;
                }
                1i32 => {
                    F::Font_SetAlign(c, 2i32);
                    break 's1;
                }
                _ => {
                    F::Font_SetAlign(c, 1i32);
                    break 's1;
                }
            }
        }
        F::Font_PrintAt(c, p_x_1, p_y_2, l_str_6);
        F::Font_PopStyle(c);
        F::Font_Flush(c, l_font_7);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlText_DrawEntryName", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSeq_FirstState(c: &mut Ctx, mut p_table_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSeq_FirstState", &PROBE_N, &[(p_table_1) as u32], false); }
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < (7i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if (((((c.m.r32((p_table_1).wrapping_add((((l_i_2) as u32)).wrapping_mul(24)))) != (((0i32) as u32))) as i32)) != 0) {
                    break 'f l_i_2;
                }
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSeq_FirstState", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSeq_Call(c: &mut Ctx, mut p_func_1: u32, mut p_state_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSeq_Call", &PROBE_N, &[(p_func_1) as u32, (p_state_2) as u32], false); }
    let r_ = 'f: {
    if (((((p_func_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f (1i32).wrapping_neg();
    }
    { let v_ = { let a_ = (({ let a_ = (c.m.r32(sym::gBtlSeq)).wrapping_add(260); c.m.r32(a_) }).wrapping_add((((p_state_2) as u32)).wrapping_mul(24))).wrapping_add(20); c.m.r32(a_) }; let a_ = (c.m.r32(sym::gBtlSeq)).wrapping_add(4); c.m.w32(a_, v_); };
    break 'f { let a0_ = (c.m.r32(sym::gBtlSeq)).wrapping_add(4); ((D::call(c, p_func_1, &[((a0_) as u64)])) as i32) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSeq_Call", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSeq_Reset(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSeq_Reset", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let _ = { let a0_ = c.m.r32(sym::gBtlSeq); F::memset(c, a0_, 0i32, 300u32) }; };
    F::BtlSeq_ResetClocks(c);
    's1: {
        let sv1: i32 = F::Battle_GetMode(c);
        match sv1 {
            1i32 => {
                { let v_ = sym::gBtlSeqTblMode1; let a_ = (c.m.r32(sym::gBtlSeq)).wrapping_add(260); c.m.w32(a_, v_); };
                break 's1;
            }
            5i32 | 6i32 | 7i32 => {
                { let v_ = sym::gBtlSeqTblMode5to7; let a_ = (c.m.r32(sym::gBtlSeq)).wrapping_add(260); c.m.w32(a_, v_); };
                break 's1;
            }
            _ => {
                { let v_ = sym::gBtlSeqTblDefault; let a_ = (c.m.r32(sym::gBtlSeq)).wrapping_add(260); c.m.w32(a_, v_); };
                break 's1;
            }
        }
    }
    { let v_ = { let a0_ = { let a_ = (c.m.r32(sym::gBtlSeq)).wrapping_add(260); c.m.r32(a_) }; F::BtlSeq_FirstState(c, a0_) }; let a_ = c.m.r32(sym::gBtlSeq); c.m.w32(a_, (v_) as u32); };
    F::Voice_Stop(c, 0i32);
    F::Voice_Stop(c, 1i32);
    break 'f { let a0_ = { let a_ = ({ let a_ = (c.m.r32(sym::gBtlSeq)).wrapping_add(260); c.m.r32(a_) }).wrapping_add(((({ let a_ = c.m.r32(sym::gBtlSeq); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(24)); c.m.r32(a_) }; let a1_ = { let a_ = c.m.r32(sym::gBtlSeq); (c.m.r32(a_) as i32) }; F::BtlSeq_Call(c, a0_, a1_) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSeq_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSeq_Init(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSeq_Init", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let v_ = F::Heap_Alloc(c, ((300u32) as i32), ((32i32) as u32), 0i32, 2i32); let a_ = sym::gBtlSeq; c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32(sym::gBtlSeq); F::memset(c, a0_, 0i32, 300u32) }; };
    break 'f F::BtlSeq_Reset(c);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSeq_Init", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSeq_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSeq_Term", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let a0_ = c.m.r32(sym::gBtlSeq); F::Heap_Free(c, a0_) };
    { let v_ = ((0i32) as u32); let a_ = sym::gBtlSeq; c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSeq_Term", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSeq_PreUpdate(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSeq_PreUpdate", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f { let a0_ = { let a_ = (({ let a_ = (c.m.r32(sym::gBtlSeq)).wrapping_add(260); c.m.r32(a_) }).wrapping_add(((({ let a_ = c.m.r32(sym::gBtlSeq); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(24))).wrapping_add(4); c.m.r32(a_) }; let a1_ = { let a_ = c.m.r32(sym::gBtlSeq); (c.m.r32(a_) as i32) }; F::BtlSeq_Call(c, a0_, a1_) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSeq_PreUpdate", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSeq_Update(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSeq_Update", &PROBE_N, &[], false); }
    let mut l_next_1: i32 = 0;
    let r_ = 'f: {
    l_next_1 = { let a0_ = { let a_ = (({ let a_ = (c.m.r32(sym::gBtlSeq)).wrapping_add(260); c.m.r32(a_) }).wrapping_add(((({ let a_ = c.m.r32(sym::gBtlSeq); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(24))).wrapping_add(8); c.m.r32(a_) }; let a1_ = { let a_ = c.m.r32(sym::gBtlSeq); (c.m.r32(a_) as i32) }; F::BtlSeq_Call(c, a0_, a1_) };
    if (((((l_next_1) != ({ let a_ = c.m.r32(sym::gBtlSeq); (c.m.r32(a_) as i32) })) as i32)) != 0) {
        { let _ = { let a0_ = { let a_ = (({ let a_ = (c.m.r32(sym::gBtlSeq)).wrapping_add(260); c.m.r32(a_) }).wrapping_add(((({ let a_ = c.m.r32(sym::gBtlSeq); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(24))).wrapping_add(12); c.m.r32(a_) }; let a1_ = { let a_ = c.m.r32(sym::gBtlSeq); (c.m.r32(a_) as i32) }; F::BtlSeq_Call(c, a0_, a1_) }; };
        { let v_ = l_next_1; let a_ = c.m.r32(sym::gBtlSeq); c.m.w32(a_, (v_) as u32); };
        if ((((({ let a_ = c.m.r32(sym::gBtlSeq); (c.m.r32(a_) as i32) }) == (99i32)) as i32)) != 0) {
            if ((((((F::Battle_IsRematchRequested(c)) != 0) || (((((((((F::Battle_GetMode(c)) == (6i32)) as i32)) != 0) && ((F::BattleResult_IsReasonBit2(c)) != 0)) as i32)) != 0)) as i32)) != 0) {
                { let r_ = ((32768i32) as u64); let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
                break 'f 0i32;
            }
            break 'f 1i32;
        }
        if ((((({ let a_ = ({ let a_ = (c.m.r32(sym::gBtlSeq)).wrapping_add(260); c.m.r32(a_) }).wrapping_add(((({ let a_ = c.m.r32(sym::gBtlSeq); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(24)); c.m.r32(a_) }) == (((0i32) as u32))) as i32)) != 0) {
            { let v_ = 6i32; let a_ = c.m.r32(sym::gBtlSeq); c.m.w32(a_, (v_) as u32); };
        }
        { let _ = { let a0_ = { let a_ = ({ let a_ = (c.m.r32(sym::gBtlSeq)).wrapping_add(260); c.m.r32(a_) }).wrapping_add(((({ let a_ = c.m.r32(sym::gBtlSeq); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(24)); c.m.r32(a_) }; let a1_ = { let a_ = c.m.r32(sym::gBtlSeq); (c.m.r32(a_) as i32) }; F::BtlSeq_Call(c, a0_, a1_) }; };
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSeq_Update", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSeq_GetClock(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSeq_GetClock", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f (c.m.r32(sym::gBtlSeq)).wrapping_add(264);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSeq_GetClock", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSeq_GetSubClock(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSeq_GetSubClock", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f (c.m.r32(sym::gBtlSeq)).wrapping_add(280);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSeq_GetSubClock", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSeq_GetState(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSeq_GetState", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f { let a_ = c.m.r32(sym::gBtlSeq); (c.m.r32(a_) as i32) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSeq_GetState", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSeq_IsFighting(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSeq_IsFighting", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f (((F::BtlSeq_GetState(c)) == (3i32)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSeq_IsFighting", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSeq_IsFinish(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSeq_IsFinish", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f (((F::BtlSeq_GetState(c)) == (4i32)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSeq_IsFinish", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSeq_CallExtra(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSeq_CallExtra", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f { let a0_ = { let a_ = (({ let a_ = (c.m.r32(sym::gBtlSeq)).wrapping_add(260); c.m.r32(a_) }).wrapping_add(((({ let a_ = c.m.r32(sym::gBtlSeq); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(24))).wrapping_add(16); c.m.r32(a_) }; let a1_ = { let a_ = c.m.r32(sym::gBtlSeq); (c.m.r32(a_) as i32) }; F::BtlSeq_Call(c, a0_, a1_) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSeq_CallExtra", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSeq_SetEndCheckOff(c: &mut Ctx, mut p_off_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSeq_SetEndCheckOff", &PROBE_N, &[(p_off_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_off_1; let a_ = (c.m.r32(sym::gBtlSeq)).wrapping_add(296); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSeq_SetEndCheckOff", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSeq_GetEndCheckOff(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSeq_GetEndCheckOff", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f { let a_ = (c.m.r32(sym::gBtlSeq)).wrapping_add(296); (c.m.r32(a_) as i32) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSeq_GetEndCheckOff", &PROBE_N, &[], true); }
    r_
}

pub fn BtlClock_Clear(c: &mut Ctx, mut p_clock_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlClock_Clear", &PROBE_N, &[(p_clock_1) as u32], false); }
    let r_ = 'f: {
    { let _ = F::memset(c, p_clock_1, 0i32, 16u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlClock_Clear", &PROBE_N, &[], true); }
    r_
}

pub fn BtlClock_Tick(c: &mut Ctx, mut p_clock_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlClock_Tick", &PROBE_N, &[(p_clock_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_add_2: i32 = 0;
    let r_ = 'f: {
        rt::memset(c, fp, 0, 12);
        { let v_ = 34i32; c.m.w32(fp.wrapping_add(0), (v_) as u32); }
        { let v_ = 32i32; c.m.w32(fp.wrapping_add(4), (v_) as u32); }
        { let v_ = 34i32; c.m.w32(fp.wrapping_add(8), (v_) as u32); }
        l_add_2 = { let a_ = (fp).wrapping_add((rt::rem_u32({ let a_ = p_clock_1; let o_ = c.m.r32(a_); let n_ = (o_).wrapping_add(1); c.m.w32(a_, n_); o_ }, ((3i32) as u32))).wrapping_mul(4)); (c.m.r32(a_) as i32) };
        { let r_ = l_add_2; let a_ = (p_clock_1).wrapping_add(10); let n_ = ((((((c.m.r16(a_) as i16)) as i32)).wrapping_add(r_)) as i16); c.m.w16(a_, (n_) as u16); };
        if ((((((((((((((((((((c.m.r16((p_clock_1).wrapping_add(4)) as i16)) as i32)) >= (9i32)) as i32)) != 0) && ((((((((c.m.r16((p_clock_1).wrapping_add(6)) as i16)) as i32)) >= (59i32)) as i32)) != 0)) as i32)) != 0) && ((((((((c.m.r16((p_clock_1).wrapping_add(8)) as i16)) as i32)) >= (59i32)) as i32)) != 0)) as i32)) != 0) && ((((((((c.m.r16((p_clock_1).wrapping_add(10)) as i16)) as i32)) >= (999i32)) as i32)) != 0)) as i32)) != 0) {
            { let v_ = ((9i32) as i16); let a_ = (p_clock_1).wrapping_add(4); c.m.w16(a_, (v_) as u16); };
            { let v_ = ((59i32) as i16); let a_ = (p_clock_1).wrapping_add(6); c.m.w16(a_, (v_) as u16); };
            { let v_ = ((59i32) as i16); let a_ = (p_clock_1).wrapping_add(8); c.m.w16(a_, (v_) as u16); };
            { let v_ = ((999i32) as i16); let a_ = (p_clock_1).wrapping_add(10); c.m.w16(a_, (v_) as u16); };
        } else {
            if ((((((((c.m.r16((p_clock_1).wrapping_add(10)) as i16)) as i32)) >= (1000i32)) as i32)) != 0) {
                { let r_ = 1000i32; let a_ = (p_clock_1).wrapping_add(10); let n_ = ((((((c.m.r16(a_) as i16)) as i32)).wrapping_sub(r_)) as i16); c.m.w16(a_, (n_) as u16); };
                { let a_ = (p_clock_1).wrapping_add(8); let o_ = (c.m.r16(a_) as i16); let n_ = (o_).wrapping_add(1); c.m.w16(a_, (n_) as u16); };
            }
            if ((((((((c.m.r16((p_clock_1).wrapping_add(8)) as i16)) as i32)) >= (60i32)) as i32)) != 0) {
                { let r_ = 60i32; let a_ = (p_clock_1).wrapping_add(8); let n_ = ((((((c.m.r16(a_) as i16)) as i32)).wrapping_sub(r_)) as i16); c.m.w16(a_, (n_) as u16); };
                { let a_ = (p_clock_1).wrapping_add(6); let o_ = (c.m.r16(a_) as i16); let n_ = (o_).wrapping_add(1); c.m.w16(a_, (n_) as u16); };
            }
            if ((((((((c.m.r16((p_clock_1).wrapping_add(6)) as i16)) as i32)) >= (60i32)) as i32)) != 0) {
                { let r_ = 60i32; let a_ = (p_clock_1).wrapping_add(6); let n_ = ((((((c.m.r16(a_) as i16)) as i32)).wrapping_sub(r_)) as i16); c.m.w16(a_, (n_) as u16); };
                { let a_ = (p_clock_1).wrapping_add(4); let o_ = (c.m.r16(a_) as i16); let n_ = (o_).wrapping_add(1); c.m.w16(a_, (n_) as u16); };
            }
        }
        break 'f 1i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlClock_Tick", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSeq_ResetClocks(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSeq_ResetClocks", &PROBE_N, &[], false); }
    let mut l_clock_1: u32 = 0;
    let r_ = 'f: {
    l_clock_1 = F::BtlSeq_GetClock(c);
    F::BtlClock_Clear(c, l_clock_1);
    { let v_ = ((F::Battle_GetTimeLimit(c)) as i16); let a_ = (l_clock_1).wrapping_add(12); c.m.w16(a_, (v_) as u16); };
    { let a0_ = F::BtlSeq_GetSubClock(c); F::BtlClock_Clear(c, a0_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSeq_ResetClocks", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSeq_TickClocks(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSeq_TickClocks", &PROBE_N, &[], false); }
    let mut l_clock_1: u32 = 0;
    let r_ = 'f: {
    { let _ = F::BtlSeq_GetClock(c); };
    { let _ = { let a0_ = F::BtlSeq_GetSubClock(c); F::BtlClock_Tick(c, a0_) }; };
    l_clock_1 = F::BtlSeq_GetClock(c);
    { let _ = F::BtlClock_Tick(c, l_clock_1); };
    if ((F::Battle_IsTimeLimitOff(c)) != 0) {
        break 'f 0i32;
    }
    if ((((((((((c.m.r16((l_clock_1).wrapping_add(6)) as i16)) as i32)).wrapping_mul(60i32)).wrapping_add((((c.m.r16((l_clock_1).wrapping_add(8)) as i16)) as i32))) >= (F::Battle_GetTimeLimit(c))) as i32)) != 0) {
        { let v_ = ((0i32) as i16); let a_ = (l_clock_1).wrapping_add(12); c.m.w16(a_, (v_) as u16); };
        break 'f 1i32;
    }
    { let v_ = (((F::Battle_GetTimeLimit(c)).wrapping_sub((((((c.m.r16((l_clock_1).wrapping_add(6)) as i16)) as i32)).wrapping_mul(60i32)).wrapping_add((((c.m.r16((l_clock_1).wrapping_add(8)) as i16)) as i32)))) as i16); let a_ = (l_clock_1).wrapping_add(12); c.m.w16(a_, (v_) as u16); };
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSeq_TickClocks", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSeq_ResetSubClock(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSeq_ResetSubClock", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let a0_ = F::BtlSeq_GetSubClock(c); F::BtlClock_Clear(c, a0_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSeq_ResetSubClock", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSeq_GetTimeLeft(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSeq_GetTimeLeft", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if ((F::Battle_IsTimeLimitOff(c)) != 0) {
        break 'f (1i32).wrapping_neg();
    }
    break 'f (({ let a_ = (F::BtlSeq_GetClock(c)).wrapping_add(12); (c.m.r16(a_) as i16) }) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSeq_GetTimeLeft", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSeq_StopTalk(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSeq_StopTalk", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let a0_ = { let a0_ = F::BattleSide_GetObjId(c, 0i32); F::BtlObj_Get(c, a0_) }; F::BtlObj_SetSubState(c, a0_, 0i32, (1i32).wrapping_neg()) };
    { let a0_ = { let a0_ = F::BattleSide_GetObjId(c, 1i32); F::BtlObj_Get(c, a0_) }; F::BtlObj_SetSubState(c, a0_, 0i32, (1i32).wrapping_neg()) };
    F::Voice_Stop(c, 0i32);
    F::Voice_Stop(c, 1i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSeq_StopTalk", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSeqIntroTalk_Setup(c: &mut Ctx, mut p_ctx_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSeqIntroTalk_Setup", &PROBE_N, &[(p_ctx_1) as u32], false); }
    let mut l_obj0_2: u32 = 0;
    let mut l_tbl0_3: u32 = 0;
    let mut l_obj1_4: u32 = 0;
    let mut l_tbl1_5: u32 = 0;
    let r_ = 'f: {
    l_obj0_2 = { let a0_ = F::BattleSide_GetObjId(c, 0i32); F::BtlObj_Get(c, a0_) };
    l_tbl0_3 = c.m.r32((l_obj0_2).wrapping_add(2364));
    l_obj1_4 = { let a0_ = F::BattleSide_GetObjId(c, 1i32); F::BtlObj_Get(c, a0_) };
    l_tbl1_5 = c.m.r32((l_obj1_4).wrapping_add(2364));
    if (((((((((l_tbl0_3) != (((0i32) as u32))) as i32)) != 0) && (((((l_tbl1_5) != (((0i32) as u32))) as i32)) != 0)) as i32)) != 0) {
        if ((((((({ let a_ = (l_tbl0_3).wrapping_add(((((c.m.r32((l_obj1_4).wrapping_add(12)) as i32)).wrapping_mul(4i32)) as u32)); c.m.r8(a_) }) as i32)) != (255i32)) as i32)) != 0) {
            if ((((((({ let a_ = (l_tbl0_3).wrapping_add((((((c.m.r32((l_obj1_4).wrapping_add(12)) as i32)).wrapping_mul(4i32)).wrapping_add(1i32)) as u32)); c.m.r8(a_) }) as i32)) == (0i32)) as i32)) != 0) {
                { let v_ = 0i32; let a_ = ((p_ctx_1).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = 1i32; let a_ = ((p_ctx_1).wrapping_add(8)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = (c.m.r32((l_obj0_2).wrapping_add(12)) as i32); let a_ = ((p_ctx_1).wrapping_add(24)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = (c.m.r32((l_obj1_4).wrapping_add(12)) as i32); let a_ = ((p_ctx_1).wrapping_add(24)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = ((({ let a_ = (l_tbl0_3).wrapping_add(((((c.m.r32((l_obj1_4).wrapping_add(12)) as i32)).wrapping_mul(4i32)) as u32)); c.m.r8(a_) }) as i32)).wrapping_add(6i32); let a_ = ((p_ctx_1).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = ((({ let a_ = (l_tbl1_5).wrapping_add(((((c.m.r32((l_obj0_2).wrapping_add(12)) as i32)).wrapping_mul(4i32)) as u32)); c.m.r8(a_) }) as i32)).wrapping_add(6i32); let a_ = ((p_ctx_1).wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            } else {
                { let v_ = 0i32; let a_ = ((p_ctx_1).wrapping_add(8)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = 1i32; let a_ = ((p_ctx_1).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = (c.m.r32((l_obj1_4).wrapping_add(12)) as i32); let a_ = ((p_ctx_1).wrapping_add(24)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = (c.m.r32((l_obj0_2).wrapping_add(12)) as i32); let a_ = ((p_ctx_1).wrapping_add(24)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = ((({ let a_ = (l_tbl1_5).wrapping_add(((((c.m.r32((l_obj0_2).wrapping_add(12)) as i32)).wrapping_mul(4i32)) as u32)); c.m.r8(a_) }) as i32)).wrapping_add(6i32); let a_ = ((p_ctx_1).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = ((({ let a_ = (l_tbl0_3).wrapping_add(((((c.m.r32((l_obj1_4).wrapping_add(12)) as i32)).wrapping_mul(4i32)) as u32)); c.m.r8(a_) }) as i32)).wrapping_add(6i32); let a_ = ((p_ctx_1).wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            }
        } else {
            { let v_ = 0i32; let a_ = ((p_ctx_1).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            { let v_ = 1i32; let a_ = ((p_ctx_1).wrapping_add(8)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            { let v_ = (c.m.r32((l_obj0_2).wrapping_add(12)) as i32); let a_ = ((p_ctx_1).wrapping_add(24)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            { let v_ = (c.m.r32((l_obj1_4).wrapping_add(12)) as i32); let a_ = ((p_ctx_1).wrapping_add(24)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            { let v_ = rt::rem_i32(F::Port_Rand(c), 2i32); let a_ = ((p_ctx_1).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            { let v_ = rt::rem_i32(F::Port_Rand(c), 2i32); let a_ = ((p_ctx_1).wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        }
    } else {
        { let v_ = 0i32; let a_ = ((p_ctx_1).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = 1i32; let a_ = ((p_ctx_1).wrapping_add(8)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = (c.m.r32((l_obj0_2).wrapping_add(12)) as i32); let a_ = ((p_ctx_1).wrapping_add(24)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = (c.m.r32((l_obj1_4).wrapping_add(12)) as i32); let a_ = ((p_ctx_1).wrapping_add(24)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = rt::rem_i32(F::Port_Rand(c), 2i32); let a_ = ((p_ctx_1).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = rt::rem_i32(F::Port_Rand(c), 2i32); let a_ = ((p_ctx_1).wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    }
    { let v_ = 0i32; let a_ = (p_ctx_1).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
    F::Ramp_Start(c, (p_ctx_1).wrapping_add(32), F32(0x41200000), F32(0x00000000), F32(0x3f800000));
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSeqIntroTalk_Setup", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSeqIntroTalk_Enter(c: &mut Ctx, mut p_ctx_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSeqIntroTalk_Enter", &PROBE_N, &[(p_ctx_1) as u32], false); }
    let r_ = 'f: {
    if (((((F::Battle_GetMode(c)) == (1i32)) as i32)) != 0) {
        F::BtlFacade_SetCtrl10D(c, 0i32);
        F::BtlFacade_SetCtrl10D(c, 1i32);
        { let v_ = 0i32; let a_ = (p_ctx_1).wrapping_add(56); c.m.w32(a_, (v_) as u32); };
        { let v_ = 99i32; let a_ = (p_ctx_1).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
    } else {
        { let _ = F::BtlSeqIntroTalk_Setup(c, p_ctx_1); };
    }
    { let r_ = ((512i32) as u64); let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSeqIntroTalk_Enter", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSeqIntroTalk_PreUpdate(c: &mut Ctx, mut p_ctx_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSeqIntroTalk_PreUpdate", &PROBE_N, &[(p_ctx_1) as u32], false); }
    let r_ = 'f: {
    's1: {
        let sv2: i32 = (c.m.r32((p_ctx_1).wrapping_add(4)) as i32);
        match sv2 {
            0i32 => {
                { let a0_ = (c.m.r32(((p_ctx_1).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32); F::BtlCtrl_StartEntrance(c, a0_) };
                F::Ramp_Start(c, (p_ctx_1).wrapping_add(32), F32(0x41200000), F32(0x00000000), F32(0x3f800000));
                { let a0_ = (c.m.r32(((p_ctx_1).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32); F::DemoCam_PlayObjAnim(c, a0_, 0i32) };
                { let a0_ = (c.m.r32(((p_ctx_1).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32); let a1_ = (c.m.r32(((p_ctx_1).wrapping_add(24)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32); let a2_ = (c.m.r32(((p_ctx_1).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32); F::Voice_PlayChara(c, a0_, a1_, a2_) };
                { let a0_ = { let a0_ = { let a0_ = (c.m.r32(((p_ctx_1).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32); F::BattleSide_GetObjId(c, a0_) }; F::BtlObj_Get(c, a0_) }; let a2_ = (c.m.r32(((p_ctx_1).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32); F::BtlObj_SetSubState(c, a0_, 2i32, a2_) };
                { let a_ = (p_ctx_1).wrapping_add(4); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            2i32 => {
                { let a0_ = (c.m.r32(((p_ctx_1).wrapping_add(8)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32); F::BtlCtrl_StartEntrance(c, a0_) };
                F::Ramp_Start(c, (p_ctx_1).wrapping_add(32), F32(0x41200000), F32(0x00000000), F32(0x3f800000));
                { let a0_ = (c.m.r32(((p_ctx_1).wrapping_add(8)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32); F::DemoCam_PlayObjAnim(c, a0_, 0i32) };
                { let a0_ = (c.m.r32(((p_ctx_1).wrapping_add(8)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32); let a1_ = (c.m.r32(((p_ctx_1).wrapping_add(24)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32); let a2_ = (c.m.r32(((p_ctx_1).wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32); F::Voice_PlayChara(c, a0_, a1_, a2_) };
                { let a0_ = { let a0_ = { let a0_ = (c.m.r32(((p_ctx_1).wrapping_add(8)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32); F::BattleSide_GetObjId(c, a0_) }; F::BtlObj_Get(c, a0_) }; let a2_ = (c.m.r32(((p_ctx_1).wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32); F::BtlObj_SetSubState(c, a0_, 2i32, a2_) };
                { let a_ = (p_ctx_1).wrapping_add(4); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            3i32 => {
                break 's1;
            }
            _ => {
            }
        }
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSeqIntroTalk_PreUpdate", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSeqIntroTalk_Update(c: &mut Ctx, mut p_ctx_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSeqIntroTalk_Update", &PROBE_N, &[(p_ctx_1) as u32], false); }
    let mut sv_2: i32 = 0;
    let mut bb_: u32 = 0;
    let r_ = 'f: {
    'fl1: loop {
        match bb_ {
            0 => {
                sv_2 = (c.m.r32((p_ctx_1).wrapping_add(4)) as i32);
                bb_ = match sv_2 { 0i32 => 3, 1i32 => 4, 2i32 => 13, 3i32 => 14, 4i32 => 20, 99i32 => 22, _ => 1, }; continue 'fl1;
            }
            1 => {
                if (((((c.m.r32(p_ctx_1)) != (((0i32) as u32))) as i32)) != 0) { bb_ = 28; continue 'fl1; } else { bb_ = 29; continue 'fl1; }
            }
            2 => {
                bb_ = 3; continue 'fl1;
            }
            3 => {
                bb_ = 4; continue 'fl1;
            }
            4 => {
                if (({ let a0_ = (c.m.r32(((p_ctx_1).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32); F::BtlCtrl_IsPoseReached(c, a0_) }) != 0) { bb_ = 5; continue 'fl1; } else { bb_ = 6; continue 'fl1; }
            }
            5 => {
                if (({ let a0_ = (c.m.r32(((p_ctx_1).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32); F::Voice_IsStopped(c, a0_) }) != 0) { bb_ = 7; continue 'fl1; } else { bb_ = 9; continue 'fl1; }
            }
            6 => {
                bb_ = 1; continue 'fl1;
            }
            7 => {
                F::ScrXfade_RequestCapture(c);
                F::ScrXfade_Start(c, 1i32, F32(0x3f800000));
                { let a_ = (p_ctx_1).wrapping_add(4); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                bb_ = 8; continue 'fl1;
            }
            8 => {
                bb_ = 6; continue 'fl1;
            }
            9 => {
                if ((F::Ramp_Step(c, (p_ctx_1).wrapping_add(32))) != 0) { bb_ = 10; continue 'fl1; } else { bb_ = 11; continue 'fl1; }
            }
            10 => {
                F::ScrXfade_RequestCapture(c);
                F::ScrXfade_Start(c, 1i32, F32(0x3f800000));
                { let a_ = (p_ctx_1).wrapping_add(4); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                bb_ = 11; continue 'fl1;
            }
            11 => {
                bb_ = 8; continue 'fl1;
            }
            12 => {
                bb_ = 13; continue 'fl1;
            }
            13 => {
                bb_ = 14; continue 'fl1;
            }
            14 => {
                if (({ let a0_ = (c.m.r32(((p_ctx_1).wrapping_add(8)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32); F::BtlCtrl_IsPoseReached(c, a0_) }) != 0) { bb_ = 15; continue 'fl1; } else { bb_ = 16; continue 'fl1; }
            }
            15 => {
                if (((((({ let a0_ = (c.m.r32(((p_ctx_1).wrapping_add(8)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32); F::Voice_IsStopped(c, a0_) }) != 0) || ((F::Ramp_Step(c, (p_ctx_1).wrapping_add(32))) != 0)) as i32)) != 0) { bb_ = 17; continue 'fl1; } else { bb_ = 18; continue 'fl1; }
            }
            16 => {
                bb_ = 1; continue 'fl1;
            }
            17 => {
                F::ScrXfade_RequestCapture(c);
                F::ScrXfade_Start(c, 1i32, F32(0x3f800000));
                { let a_ = (p_ctx_1).wrapping_add(4); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                bb_ = 18; continue 'fl1;
            }
            18 => {
                bb_ = 16; continue 'fl1;
            }
            19 => {
                bb_ = 20; continue 'fl1;
            }
            20 => {
                break 'f 2i32;
            }
            21 => {
                bb_ = 22; continue 'fl1;
            }
            22 => {
                if (((c.m.r32((p_ctx_1).wrapping_add(56)) as i32)) != 0) { bb_ = 23; continue 'fl1; } else { bb_ = 24; continue 'fl1; }
            }
            23 => {
                F::ScrXfade_RequestCapture(c);
                F::ScrXfade_Start(c, 1i32, F32(0x3f800000));
                bb_ = 25; continue 'fl1;
            }
            24 => {
                bb_ = 1; continue 'fl1;
            }
            25 => {
                F::BtlSeq_StopTalk(c);
                break 'f 2i32;
            }
            26 => {
                bb_ = 24; continue 'fl1;
            }
            27 => {
                bb_ = 1; continue 'fl1;
            }
            28 => {
                if (({ let fp_ = c.m.r32(p_ctx_1); ((D::call(c, fp_, &[])) as i32) }) != 0) { bb_ = 30; continue 'fl1; } else { bb_ = 31; continue 'fl1; }
            }
            29 => {
                break 'f 1i32;
            }
            30 => {
                if ((((((c.m.r32((p_ctx_1).wrapping_add(4)) as i32)) != (4i32)) as i32)) != 0) { bb_ = 32; continue 'fl1; } else { bb_ = 33; continue 'fl1; }
            }
            31 => {
                bb_ = 29; continue 'fl1;
            }
            32 => {
                F::ScrXfade_RequestCapture(c);
                F::ScrXfade_Start(c, 1i32, F32(0x3f800000));
                if ((F::BtlScript_IsEventRunning(c)) != 0) { bb_ = 34; continue 'fl1; } else { bb_ = 35; continue 'fl1; }
            }
            33 => {
                bb_ = 25; continue 'fl1;
            }
            34 => {
                F::BtlScript_AbortEvents(c);
                bb_ = 35; continue 'fl1;
            }
            35 => {
                bb_ = 33; continue 'fl1;
            }
            36 => {
                bb_ = 31; continue 'fl1;
            }
            37 => {
                break 'fl1;
            }
            _ => unreachable!(),
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSeqIntroTalk_Update", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSeqIntroTalk_Exit(c: &mut Ctx, mut p_ctx_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSeqIntroTalk_Exit", &PROBE_N, &[(p_ctx_1) as u32], false); }
    let r_ = 'f: {
    if (((((F::Battle_GetMode(c)) != (1i32)) as i32)) != 0) {
        F::DemoCam_Stop(c);
        { let a0_ = (c.m.r32(((p_ctx_1).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32); F::BtlCtrl_EndEntrance(c, a0_) };
        { let a0_ = (c.m.r32(((p_ctx_1).wrapping_add(8)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32); F::BtlCtrl_EndEntrance(c, a0_) };
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSeqIntroTalk_Exit", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSeqIntroTalk_Skip(c: &mut Ctx, mut p_ctx_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSeqIntroTalk_Skip", &PROBE_N, &[(p_ctx_1) as u32], false); }
    let mut l_mode_2: i32 = 0;
    let r_ = 'f: {
    l_mode_2 = F::Battle_GetMode(c);
    if (((((l_mode_2) == (1i32)) as i32)) != 0) {
        { let v_ = l_mode_2; let a_ = (p_ctx_1).wrapping_add(56); c.m.w32(a_, (v_) as u32); };
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSeqIntroTalk_Skip", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSeqWinTalk_Setup(c: &mut Ctx, mut p_ctx_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSeqWinTalk_Setup", &PROBE_N, &[(p_ctx_1) as u32], false); }
    let mut l_winner_2: i32 = 0;
    let mut l_loser_3: i32 = 0;
    let mut l_winObj_4: u32 = 0;
    let mut l_winTbl_5: u32 = 0;
    let mut l_loseObj_6: u32 = 0;
    let mut l_loseTbl_7: u32 = 0;
    let r_ = 'f: {
    l_winner_2 = F::BattleResult_GetWinnerSide(c);
    l_loser_3 = (((l_winner_2) == (0i32)) as i32);
    l_winObj_4 = { let a0_ = F::BattleSide_GetObjId(c, l_winner_2); F::BtlObj_Get(c, a0_) };
    l_winTbl_5 = c.m.r32((l_winObj_4).wrapping_add(2364));
    l_loseObj_6 = { let a0_ = F::BattleSide_GetObjId(c, l_loser_3); F::BtlObj_Get(c, a0_) };
    l_loseTbl_7 = c.m.r32((l_loseObj_6).wrapping_add(2364));
    if ((F::BattleResult_IsPlayerWin(c)) != 0) {
        if (((((((((((((((((l_winTbl_5) == (((0i32) as u32))) as i32)) != 0) || (((((l_loseTbl_7) == (((0i32) as u32))) as i32)) != 0)) as i32)) != 0) || ((F::BtlCharApi_HasMemberUnk70(c, l_winner_2)) != 0)) as i32)) != 0) || ((F::BtlCharApi_HasMemberUnk70(c, l_loser_3)) != 0)) as i32)) != 0) {
            { let v_ = l_winner_2; let a_ = ((p_ctx_1).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            if ((F::BtlCharApi_HasMemberUnk70(c, l_winner_2)) != 0) {
                { let v_ = 86i32; let a_ = ((p_ctx_1).wrapping_add(24)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            } else {
                { let v_ = (c.m.r32((l_winObj_4).wrapping_add(12)) as i32); let a_ = ((p_ctx_1).wrapping_add(24)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            }
            { let v_ = (rt::rem_i32(F::Port_Rand(c), 2i32)).wrapping_add(3i32); let a_ = ((p_ctx_1).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            { let v_ = 0i32; let a_ = (p_ctx_1).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        } else {
            if ((((((({ let a_ = (l_winTbl_5).wrapping_add((((((c.m.r32((l_loseObj_6).wrapping_add(12)) as i32)).wrapping_mul(4i32)).wrapping_add(2i32)) as u32)); c.m.r8(a_) }) as i32)) != (255i32)) as i32)) != 0) {
                { let v_ = l_winner_2; let a_ = ((p_ctx_1).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = (c.m.r32((l_winObj_4).wrapping_add(12)) as i32); let a_ = ((p_ctx_1).wrapping_add(24)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = ((({ let a_ = (l_winTbl_5).wrapping_add((((((c.m.r32((l_loseObj_6).wrapping_add(12)) as i32)).wrapping_mul(4i32)).wrapping_add(2i32)) as u32)); c.m.r8(a_) }) as i32)).wrapping_add(30i32); let a_ = ((p_ctx_1).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (p_ctx_1).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
            } else {
                { let v_ = l_winner_2; let a_ = ((p_ctx_1).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = (c.m.r32((l_winObj_4).wrapping_add(12)) as i32); let a_ = ((p_ctx_1).wrapping_add(24)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = (rt::rem_i32(F::Port_Rand(c), 2i32)).wrapping_add(3i32); let a_ = ((p_ctx_1).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (p_ctx_1).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
            }
        }
        F::HudNotice_Show(c, 6i32);
    } else {
        { let v_ = l_loser_3; let a_ = ((p_ctx_1).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        if ((F::BtlCharApi_HasMemberUnk70(c, l_loser_3)) != 0) {
            { let v_ = 86i32; let a_ = ((p_ctx_1).wrapping_add(24)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        } else {
            { let v_ = (c.m.r32((l_loseObj_6).wrapping_add(12)) as i32); let a_ = ((p_ctx_1).wrapping_add(24)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        }
        { let v_ = 0i32; let a_ = (p_ctx_1).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = 42i32; let a_ = ((p_ctx_1).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        F::HudNotice_Show(c, 7i32);
    }
    F::Ramp_Start(c, (p_ctx_1).wrapping_add(32), F32(0x41200000), F32(0x00000000), F32(0x3f800000));
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSeqWinTalk_Setup", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSeqWinTalk_Enter(c: &mut Ctx, mut p_ctx_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSeqWinTalk_Enter", &PROBE_N, &[(p_ctx_1) as u32], false); }
    let r_ = 'f: {
    { let r_ = ((512i32) as u64); let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
    F::ScrXfade_RequestCapture(c);
    F::ScrXfade_Start(c, 1i32, F32(0x3f800000));
    if (((((F::Battle_GetMode(c)) == (1i32)) as i32)) != 0) {
        if (((((({ let a_ = F::BtlScript_GetCurrentEvent(c); c.m.r8(a_) }) as i32)) & (16i32))) != 0) {
            { let v_ = 0i32; let a_ = (p_ctx_1).wrapping_add(56); c.m.w32(a_, (v_) as u32); };
            { let v_ = 99i32; let a_ = (p_ctx_1).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        } else {
            { let _ = F::BtlSeqWinTalk_Setup(c, p_ctx_1); };
        }
        F::Hud_ShowGauges(c, 0i32);
        F::Hud_ShowTimer(c, 0i32);
        F::Hud_ShowCombo(c, 0i32);
        F::Hud_ShowPrompt(c, 0i32);
        F::Hud_ShowNotice(c, 0i32);
    } else {
        { let _ = F::BtlSeqWinTalk_Setup(c, p_ctx_1); };
        F::Hud_ShowGauges(c, 0i32);
        F::Hud_ShowTimer(c, 0i32);
        F::Hud_ShowCombo(c, 0i32);
        F::Hud_ShowPrompt(c, 0i32);
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSeqWinTalk_Enter", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSeqWinTalk_PreUpdate(c: &mut Ctx, mut p_ctx_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSeqWinTalk_PreUpdate", &PROBE_N, &[(p_ctx_1) as u32], false); }
    let r_ = 'f: {
    's1: {
        let sv2: i32 = (c.m.r32((p_ctx_1).wrapping_add(4)) as i32);
        match sv2 {
            0i32 => {
                if ((F::BattleResult_IsPlayerWin(c)) != 0) {
                    { let a0_ = (c.m.r32(((p_ctx_1).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32); F::BtlCtrl_StartWinPose(c, a0_) };
                    { let a0_ = (c.m.r32(((p_ctx_1).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32); F::DemoCam_PlayObjAnim(c, a0_, 1i32) };
                    F::DemoCam_SetScaleHeight(c, 1i32);
                } else {
                    { let a0_ = (c.m.r32(((p_ctx_1).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32); F::BtlCtrl_StartLosePose(c, a0_) };
                    { let a0_ = (c.m.r32(((p_ctx_1).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32); F::DemoCam_PlayObjAnim(c, a0_, 2i32) };
                }
                F::Ramp_Start(c, (p_ctx_1).wrapping_add(32), F32(0x41200000), F32(0x00000000), F32(0x3f800000));
                { let a0_ = (c.m.r32(((p_ctx_1).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32); let a1_ = (c.m.r32(((p_ctx_1).wrapping_add(24)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32); let a2_ = (c.m.r32(((p_ctx_1).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32); F::Voice_PlayChara(c, a0_, a1_, a2_) };
                { let a0_ = { let a0_ = { let a0_ = (c.m.r32(((p_ctx_1).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32); F::BattleSide_GetObjId(c, a0_) }; F::BtlObj_Get(c, a0_) }; let a2_ = (c.m.r32(((p_ctx_1).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32); F::BtlObj_SetSubState(c, a0_, 2i32, a2_) };
                { let a_ = (p_ctx_1).wrapping_add(4); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            2i32 => {
                F::Ramp_Start(c, (p_ctx_1).wrapping_add(32), F32(0x3fc00000), F32(0x00000000), F32(0x3f800000));
                { let a_ = (p_ctx_1).wrapping_add(4); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            3i32 => {
                break 's1;
            }
            _ => {
            }
        }
    }
    break 'f 5i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSeqWinTalk_PreUpdate", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSeqWinTalk_Update(c: &mut Ctx, mut p_ctx_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSeqWinTalk_Update", &PROBE_N, &[(p_ctx_1) as u32], false); }
    let mut sv_2: i32 = 0;
    let mut bb_: u32 = 0;
    let r_ = 'f: {
    'fl1: loop {
        match bb_ {
            0 => {
                sv_2 = (c.m.r32((p_ctx_1).wrapping_add(4)) as i32);
                bb_ = match sv_2 { 0i32 => 3, 1i32 => 4, 2i32 => 10, 3i32 => 11, 4i32 => 15, 99i32 => 17, _ => 1, }; continue 'fl1;
            }
            1 => {
                if (((((c.m.r32(p_ctx_1)) != (((0i32) as u32))) as i32)) != 0) { bb_ = 23; continue 'fl1; } else { bb_ = 24; continue 'fl1; }
            }
            2 => {
                bb_ = 3; continue 'fl1;
            }
            3 => {
                bb_ = 4; continue 'fl1;
            }
            4 => {
                if (({ let a0_ = (c.m.r32(((p_ctx_1).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32); F::BtlCtrl_IsPoseReached(c, a0_) }) != 0) { bb_ = 5; continue 'fl1; } else { bb_ = 6; continue 'fl1; }
            }
            5 => {
                if (((((({ let a0_ = (c.m.r32(((p_ctx_1).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32); F::Voice_IsStopped(c, a0_) }) != 0) || ((F::Ramp_Step(c, (p_ctx_1).wrapping_add(32))) != 0)) as i32)) != 0) { bb_ = 7; continue 'fl1; } else { bb_ = 8; continue 'fl1; }
            }
            6 => {
                bb_ = 1; continue 'fl1;
            }
            7 => {
                { let v_ = 2i32; let a_ = (p_ctx_1).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                bb_ = 8; continue 'fl1;
            }
            8 => {
                bb_ = 6; continue 'fl1;
            }
            9 => {
                bb_ = 10; continue 'fl1;
            }
            10 => {
                bb_ = 11; continue 'fl1;
            }
            11 => {
                if ((F::Ramp_Step(c, (p_ctx_1).wrapping_add(32))) != 0) { bb_ = 12; continue 'fl1; } else { bb_ = 13; continue 'fl1; }
            }
            12 => {
                { let v_ = 4i32; let a_ = (p_ctx_1).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                bb_ = 13; continue 'fl1;
            }
            13 => {
                bb_ = 1; continue 'fl1;
            }
            14 => {
                bb_ = 15; continue 'fl1;
            }
            15 => {
                break 'f 6i32;
            }
            16 => {
                bb_ = 17; continue 'fl1;
            }
            17 => {
                if (((c.m.r32((p_ctx_1).wrapping_add(56)) as i32)) != 0) { bb_ = 18; continue 'fl1; } else { bb_ = 19; continue 'fl1; }
            }
            18 => {
                bb_ = 20; continue 'fl1;
            }
            19 => {
                bb_ = 1; continue 'fl1;
            }
            20 => {
                F::BtlSeq_StopTalk(c);
                break 'f 6i32;
            }
            21 => {
                bb_ = 19; continue 'fl1;
            }
            22 => {
                bb_ = 1; continue 'fl1;
            }
            23 => {
                if (({ let fp_ = c.m.r32(p_ctx_1); ((D::call(c, fp_, &[])) as i32) }) != 0) { bb_ = 25; continue 'fl1; } else { bb_ = 26; continue 'fl1; }
            }
            24 => {
                break 'f 5i32;
            }
            25 => {
                bb_ = 20; continue 'fl1;
            }
            26 => {
                bb_ = 24; continue 'fl1;
            }
            27 => {
                bb_ = 26; continue 'fl1;
            }
            28 => {
                break 'fl1;
            }
            _ => unreachable!(),
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSeqWinTalk_Update", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSeqWinTalk_Exit(c: &mut Ctx, mut p_ctx_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSeqWinTalk_Exit", &PROBE_N, &[(p_ctx_1) as u32], false); }
    let r_ = 'f: {
    if (((((F::Battle_GetMode(c)) != (1i32)) as i32)) != 0) {
        F::BtlSeq_StopTalk(c);
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSeqWinTalk_Exit", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSeqWinTalk_Skip(c: &mut Ctx, mut p_ctx_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSeqWinTalk_Skip", &PROBE_N, &[(p_ctx_1) as u32], false); }
    let mut l_mode_2: i32 = 0;
    let r_ = 'f: {
    l_mode_2 = F::Battle_GetMode(c);
    if (((((l_mode_2) == (1i32)) as i32)) != 0) {
        { let v_ = l_mode_2; let a_ = (p_ctx_1).wrapping_add(56); c.m.w32(a_, (v_) as u32); };
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSeqWinTalk_Skip", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSeqStageIntro_Enter(c: &mut Ctx, mut p_ctx_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSeqStageIntro_Enter", &PROBE_N, &[(p_ctx_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = 0i32; let a_ = (p_ctx_1).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
    { let r_ = ((512i32) as u64); let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSeqStageIntro_Enter", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSeqStageIntro_PreUpdate(c: &mut Ctx, mut p_ctx_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSeqStageIntro_PreUpdate", &PROBE_N, &[(p_ctx_1) as u32], false); }
    let r_ = 'f: {
    's1: {
        let sv2: i32 = (c.m.r32((p_ctx_1).wrapping_add(4)) as i32);
        match sv2 {
            0i32 => {
                { let _ = F::DemoCam_PlayStageAnim(c, 0i32); };
                { let a_ = (p_ctx_1).wrapping_add(4); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            2i32 => {
                { let _ = F::DemoCam_PlayStageAnim(c, 1i32); };
                { let a_ = (p_ctx_1).wrapping_add(4); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            4i32 => {
                { let _ = F::DemoCam_PlayStageAnim(c, 2i32); };
                { let a_ = (p_ctx_1).wrapping_add(4); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            1i32 | 3i32 | 5i32 | 6i32 => {
                break 's1;
            }
            _ => {
            }
        }
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSeqStageIntro_PreUpdate", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSeqStageIntro_Update(c: &mut Ctx, mut p_ctx_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSeqStageIntro_Update", &PROBE_N, &[(p_ctx_1) as u32], false); }
    let r_ = 'f: {
    's1: {
        let sv2: i32 = (c.m.r32((p_ctx_1).wrapping_add(4)) as i32);
        match sv2 {
            0i32 | 1i32 | 2i32 | 3i32 => {
                if ((((!((F::DemoCam_IsActive(c)) != 0)) as i32)) != 0) {
                    F::ScrXfade_RequestCapture(c);
                    F::ScrXfade_Start(c, 1i32, F32(0x3f800000));
                    { let a_ = (p_ctx_1).wrapping_add(4); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                }
                break 's1;
            }
            4i32 | 5i32 => {
                if ((((!((F::DemoCam_IsActive(c)) != 0)) as i32)) != 0) {
                    { let a_ = (p_ctx_1).wrapping_add(4); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    break 'f 1i32;
                }
                break 's1;
            }
            6i32 => {
                break 'f 1i32;
            }
            _ => {
            }
        }
    }
    if (((((c.m.r32(p_ctx_1)) != (((0i32) as u32))) as i32)) != 0) {
        if (({ let fp_ = c.m.r32(p_ctx_1); ((D::call(c, fp_, &[])) as i32) }) != 0) {
            break 'f 1i32;
        }
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSeqStageIntro_Update", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSeqStageIntro_Exit(c: &mut Ctx, mut p_ctx_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSeqStageIntro_Exit", &PROBE_N, &[(p_ctx_1) as u32], false); }
    let r_ = 'f: {
    if ((F::DemoCam_IsActive(c)) != 0) {
        F::DemoCam_Stop(c);
    }
    F::ScrXfade_RequestCapture(c);
    F::ScrXfade_Start(c, 1i32, F32(0x3f800000));
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSeqStageIntro_Exit", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSeq_CanDraw(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSeq_CanDraw", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if ((((((F::BattleResult_IsTimeUp(c)) != 0) && (((((F::Battle_GetMode(c)) == (0i32)) as i32)) != 0)) as i32)) != 0) {
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSeq_CanDraw", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSeq_JudgeByHealth(c: &mut Ctx, mut p_result_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSeq_JudgeByHealth", &PROBE_N, &[(p_result_1) as u32], false); }
    let r_ = 'f: {
    if (((rt::feq(F32(c.m.r32(((p_result_1).wrapping_add(44)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((p_result_1).wrapping_add(44)).wrapping_add((((1i32) as u32)).wrapping_mul(4))))) as i32)) != 0) {
        if ((F::BattleResult_IsEvent3CSet(c, 0i32)) != 0) {
            { let v_ = 1i32; let a_ = p_result_1; c.m.w32(a_, (v_) as u32); };
        } else {
            if ((F::BattleResult_IsEvent3CSet(c, 1i32)) != 0) {
                { let v_ = 2i32; let a_ = p_result_1; c.m.w32(a_, (v_) as u32); };
            } else {
                if ((F::BtlSeq_CanDraw(c)) != 0) {
                    { let v_ = 4i32; let a_ = p_result_1; c.m.w32(a_, (v_) as u32); };
                } else {
                    if (((((F::Battle_GetMode(c)) == (8i32)) as i32)) != 0) {
                        { let v_ = 2i32; let a_ = p_result_1; c.m.w32(a_, (v_) as u32); };
                    } else {
                        if ((((F::Port_Rand(c)) & (1i32))) != 0) {
                            { let v_ = 1i32; let a_ = p_result_1; c.m.w32(a_, (v_) as u32); };
                        } else {
                            { let v_ = 2i32; let a_ = p_result_1; c.m.w32(a_, (v_) as u32); };
                        }
                    }
                }
            }
        }
    } else {
        if (((rt::flt(F32(c.m.r32(((p_result_1).wrapping_add(44)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((p_result_1).wrapping_add(44)).wrapping_add((((0i32) as u32)).wrapping_mul(4))))) as i32)) != 0) {
            { let v_ = 1i32; let a_ = p_result_1; c.m.w32(a_, (v_) as u32); };
        } else {
            { let v_ = 2i32; let a_ = p_result_1; c.m.w32(a_, (v_) as u32); };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSeq_JudgeByHealth", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSeq_CheckBattleEnd(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSeq_CheckBattleEnd", &PROBE_N, &[], false); }
    let mut l_result_1: u32 = 0;
    let mut l_timeUp_2: i32 = 0;
    let r_ = 'f: {
    l_result_1 = F::Battle_GetResult(c);
    if (((((c.m.r32(l_result_1) as i32)) & (31i32))) != 0) {
        break 'f 1i32;
    }
    if ((F::BtlSeq_GetEndCheckOff(c)) != 0) {
        break 'f 0i32;
    }
    if (((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0) {
        break 'f 0i32;
    }
    if (((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((8192i32) as u64)))) != 0) {
        break 'f 0i32;
    }
    if ((F::BtlCharApi_AnyHasFlag128(c)) != 0) {
        break 'f 0i32;
    }
    l_timeUp_2 = F::BtlSeq_TickClocks(c);
    if (((((F::Battle_GetMode(c)) == (1i32)) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((l_timeUp_2) != 0) {
        { let v_ = 2i32; let a_ = (l_result_1).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        F::BtlSeq_JudgeByHealth(c, l_result_1);
        break 'f 1i32;
    }
    if ((((((F::BtlCtrl_TestFlag7(c, 0i32)) != 0) && ((F::BtlCtrl_TestFlag7(c, 1i32)) != 0)) as i32)) != 0) {
        { let v_ = 4i32; let a_ = (l_result_1).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        F::BtlSeq_JudgeByHealth(c, l_result_1);
        break 'f 1i32;
    }
    if ((F::BtlCtrl_TestFlag7(c, 1i32)) != 0) {
        { let v_ = 1i32; let a_ = l_result_1; c.m.w32(a_, (v_) as u32); };
        { let v_ = 4i32; let a_ = (l_result_1).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        break 'f 1i32;
    }
    if ((F::BtlCtrl_TestFlag7(c, 0i32)) != 0) {
        { let v_ = 2i32; let a_ = l_result_1; c.m.w32(a_, (v_) as u32); };
        { let v_ = 4i32; let a_ = (l_result_1).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        break 'f 1i32;
    }
    if ((((((F::BtlCtrl_IsTeamDead(c, 0i32)) != 0) && ((F::BtlCtrl_IsTeamDead(c, 1i32)) != 0)) as i32)) != 0) {
        { let v_ = 1i32; let a_ = (l_result_1).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        F::BtlSeq_JudgeByHealth(c, l_result_1);
        break 'f 1i32;
    }
    if ((F::BtlCtrl_IsTeamDead(c, 1i32)) != 0) {
        { let v_ = 1i32; let a_ = l_result_1; c.m.w32(a_, (v_) as u32); };
        { let v_ = 1i32; let a_ = (l_result_1).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        break 'f 1i32;
    }
    if ((F::BtlCtrl_IsTeamDead(c, 0i32)) != 0) {
        { let v_ = 2i32; let a_ = l_result_1; c.m.w32(a_, (v_) as u32); };
        { let v_ = 1i32; let a_ = (l_result_1).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        break 'f 1i32;
    }
    if ((((!((F::BattleReplay_IsActive(c)) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((!((F::BattleReplay_TestDataFlag(c)) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    { let v_ = 16i32; let a_ = l_result_1; c.m.w32(a_, (v_) as u32); };
    { let v_ = 262144i32; let a_ = (l_result_1).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSeq_CheckBattleEnd", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSeqFight_Enter(c: &mut Ctx, mut p_ctx_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSeqFight_Enter", &PROBE_N, &[(p_ctx_1) as u32], false); }
    let r_ = 'f: {
    { let r_ = (((!(512i32))) as u64); let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); let n_ = ((c.m.r64(a_)) & (r_)); c.m.w64(a_, n_); };
    { let r_ = (((!(1024i32))) as u64); let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); let n_ = ((c.m.r64(a_)) & (r_)); c.m.w64(a_, n_); };
    if (((((F::Battle_GetMode(c)) == (8i32)) as i32)) != 0) {
        F::Hud_ShowAll(c, 1i32);
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSeqFight_Enter", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSeqFight_PreUpdate(c: &mut Ctx, mut p_ctx_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSeqFight_PreUpdate", &PROBE_N, &[(p_ctx_1) as u32], false); }
    let r_ = 'f: {
    break 'f 3i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSeqFight_PreUpdate", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSeqFight_Update(c: &mut Ctx, mut p_ctx_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSeqFight_Update", &PROBE_N, &[(p_ctx_1) as u32], false); }
    let r_ = 'f: {
    if (((((F::Battle_GetMode(c)) != (8i32)) as i32)) != 0) {
        if (((((((((((((c.m.r32(p_ctx_1)) != (((0i32) as u32))) as i32)) != 0) && ((((!(((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((16384i32) as u64)))) != 0)) as i32)) != 0)) as i32)) != 0) && (({ let fp_ = c.m.r32(p_ctx_1); ((D::call(c, fp_, &[])) as i32) }) != 0)) as i32)) != 0) {
            if (((((F::Battle_GetMode(c)) == (7i32)) as i32)) != 0) {
                F::BattleResult_Set(c, 8i32, 0i32);
            } else {
                if ((((((((((F::BattleReplay_IsActive(c)) != 0) && ((F::BattleReplay_IsLoaded(c)) != 0)) as i32)) != 0) && (((((F::BtlPause_GetPad(c)) == (1i32)) as i32)) != 0)) as i32)) != 0) {
                } else {
                    if (((((((((F::Battle_GetMode(c)) == (1i32)) as i32)) != 0) && ((F::BtlScript_IsEventRunning(c)) != 0)) as i32)) != 0) {
                        F::BtlScript_AbortEvents(c);
                    } else {
                        { let r_ = ((((16384i32) | (256i32))) as u64); let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
                        F::Adx_PauseSeVoice(c);
                        F::Snd_SetPause(c, 4i32, 1i32);
                    }
                }
            }
        }
        if (((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((16384i32) as u64)))) != 0) {
            if ((((({ let a0_ = F::BtlPause_GetMenuPad(c); F::PauseMenu_Update(c, a0_, 0i32) }) == (0i32)) as i32)) != 0) {
                { let r_ = (((!(((16384i32) | (256i32))))) as u64); let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); let n_ = ((c.m.r64(a_)) & (r_)); c.m.w64(a_, n_); };
                F::Snd_SetPause(c, 4i32, 0i32);
            }
        }
    }
    if ((F::BtlSeq_CheckBattleEnd(c)) != 0) {
        if (((((F::Battle_GetMode(c)) == (7i32)) as i32)) != 0) {
            break 'f 6i32;
        }
        break 'f (if ((F::BattleResult_IsAborted(c)) != 0) { 6i32 } else { 4i32 });
    }
    break 'f 3i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSeqFight_Update", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSeqFight_Exit(c: &mut Ctx, mut p_ctx_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSeqFight_Exit", &PROBE_N, &[(p_ctx_1) as u32], false); }
    let r_ = 'f: {
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSeqFight_Exit", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSeqReady_Enter(c: &mut Ctx, mut p_ctx_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSeqReady_Enter", &PROBE_N, &[(p_ctx_1) as u32], false); }
    let r_ = 'f: {
    { let r_ = ((512i32) as u64); let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
    { let v_ = 0i32; let a_ = (p_ctx_1).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
    F::Ramp_Start(c, (p_ctx_1).wrapping_add(8), F32(0x3f4ccccc), F32(0x00000000), F32(0x3f800000));
    F::Hud_ShowAll(c, 1i32);
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSeqReady_Enter", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSeqReady_PreUpdate(c: &mut Ctx, mut p_ctx_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSeqReady_PreUpdate", &PROBE_N, &[(p_ctx_1) as u32], false); }
    let r_ = 'f: {
    break 'f 2i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSeqReady_PreUpdate", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSeqReady_Update(c: &mut Ctx, mut p_ctx_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSeqReady_Update", &PROBE_N, &[(p_ctx_1) as u32], false); }
    let r_ = 'f: {
    's1: {
        let sv2: i32 = (c.m.r32((p_ctx_1).wrapping_add(4)) as i32);
        match sv2 {
            0i32 => {
                if ((F::Ramp_Step(c, (p_ctx_1).wrapping_add(8))) != 0) {
                    F::HudNotice_Show(c, 0i32);
                    { let r_ = (((!(512i32))) as u64); let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); let n_ = ((c.m.r64(a_)) & (r_)); c.m.w64(a_, n_); };
                    { let r_ = ((1024i32) as u64); let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
                    F::Ramp_Start(c, (p_ctx_1).wrapping_add(8), F32(0x40000000), F32(0x00000000), F32(0x3f800000));
                    { let a_ = (p_ctx_1).wrapping_add(4); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                }
                break 's1;
            }
            1i32 => {
                if ((F::Ramp_Step(c, (p_ctx_1).wrapping_add(8))) != 0) {
                    break 'f 3i32;
                }
                break 's1;
            }
            _ => {
            }
        }
    }
    break 'f 2i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSeqReady_Update", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSeqReady_Exit(c: &mut Ctx, mut p_ctx_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSeqReady_Exit", &PROBE_N, &[(p_ctx_1) as u32], false); }
    let r_ = 'f: {
    F::HudNotice_Show(c, 1i32);
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSeqReady_Exit", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSeq_SetResultPad(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSeq_SetResultPad", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if (((((F::BtlPause_GetPadCount(c)) == (1i32)) as i32)) != 0) {
        F::BtlPause_SetPad(c, 0i32);
    } else {
        { let a0_ = F::BattleResult_GetWinnerSide(c); F::BtlPause_SetPad(c, a0_) };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSeq_SetResultPad", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSeq_IsModeZero(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSeq_IsModeZero", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f (((F::Battle_GetMode(c)) == (0i32)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSeq_IsModeZero", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSeqEnd_Enter(c: &mut Ctx, mut p_ctx_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSeqEnd_Enter", &PROBE_N, &[(p_ctx_1) as u32], false); }
    let r_ = 'f: {
    { let r_ = ((512i32) as u64); let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
    if ((F::BattleResult_IsAborted(c)) != 0) {
        F::Fade_Start(c, 0i32, 0i32, F32(0x3f800000));
        F::Ramp_Start(c, (p_ctx_1).wrapping_add(8), F32(0x3f999999), F32(0x00000000), F32(0x3f800000));
        { let v_ = 1i32; let a_ = (p_ctx_1).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
    } else {
        if ((F::BtlSeq_IsModeZero(c)) != 0) {
            { let v_ = 0i32; let a_ = (p_ctx_1).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
            F::BtlSeq_SetResultPad(c);
        } else {
            F::Fade_Start(c, 0i32, 0i32, F32(0x3f800000));
            F::Ramp_Start(c, (p_ctx_1).wrapping_add(8), F32(0x3f999999), F32(0x00000000), F32(0x3f800000));
            { let v_ = 1i32; let a_ = (p_ctx_1).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        }
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSeqEnd_Enter", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSeqEnd_PreUpdate(c: &mut Ctx, mut p_ctx_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSeqEnd_PreUpdate", &PROBE_N, &[(p_ctx_1) as u32], false); }
    let r_ = 'f: {
    break 'f 6i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSeqEnd_PreUpdate", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSeqEnd_Update(c: &mut Ctx, mut p_ctx_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSeqEnd_Update", &PROBE_N, &[(p_ctx_1) as u32], false); }
    let r_ = 'f: {
    's1: {
        let sv2: i32 = (c.m.r32((p_ctx_1).wrapping_add(4)) as i32);
        match sv2 {
            0i32 => {
                { let _ = { let a0_ = F::BtlPause_GetMenuPad(c); F::PauseMenu_Update(c, a0_, 1i32) }; };
                F::PauseMenu_Draw(c);
                if ((F::BattleResult_IsAborted(c)) != 0) {
                    F::Fade_Start(c, 0i32, 0i32, F32(0x3f800000));
                    F::Ramp_Start(c, (p_ctx_1).wrapping_add(8), F32(0x3f999999), F32(0x00000000), F32(0x3f800000));
                    { let v_ = 1i32; let a_ = (p_ctx_1).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                }
                break 's1;
            }
            1i32 => {
                if ((F::Ramp_Step(c, (p_ctx_1).wrapping_add(8))) != 0) {
                    break 'f 99i32;
                }
                break 's1;
            }
            _ => {
            }
        }
    }
    break 'f 6i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSeqEnd_Update", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSeqEnd_Exit(c: &mut Ctx, mut p_ctx_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSeqEnd_Exit", &PROBE_N, &[(p_ctx_1) as u32], false); }
    let r_ = 'f: {
    if ((F::DemoCam_IsInUse(c)) != 0) {
        F::DemoCam_Stop(c);
    }
    F::DemoCam_SetScaleHeight(c, 0i32);
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSeqEnd_Exit", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSeqFinish_Enter(c: &mut Ctx, mut p_ctx_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSeqFinish_Enter", &PROBE_N, &[(p_ctx_1) as u32], false); }
    let mut l_timer_2: u32 = 0;
    let r_ = 'f: {
    { let r_ = ((512i32) as u64); let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
    l_timer_2 = (p_ctx_1).wrapping_add(8);
    F::Ramp_Start(c, l_timer_2, F32(0x40600000), F32(0x00000000), F32(0x3f800000));
    if ((F::BattleResult_IsKo(c)) != 0) {
        if ((F::BattleResult_IsWinnerEvent59Clear(c)) != 0) {
            F::HudNotice_Show(c, 3i32);
        } else {
            F::HudNotice_Show(c, 2i32);
        }
    } else {
        if ((F::BattleResult_IsTimeUp(c)) != 0) {
            F::HudNotice_Show(c, 5i32);
        } else {
            if ((F::BattleResult_IsReasonBit2(c)) != 0) {
                F::HudNotice_Show(c, 4i32);
            } else {
                if ((F::BattleResult_IsReasonBit18(c)) != 0) {
                    F::Ramp_Start(c, l_timer_2, F32(0x3f8ccccc), F32(0x00000000), F32(0x3f800000));
                }
            }
        }
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSeqFinish_Enter", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSeqFinish_PreUpdate(c: &mut Ctx, mut p_ctx_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSeqFinish_PreUpdate", &PROBE_N, &[(p_ctx_1) as u32], false); }
    let r_ = 'f: {
    break 'f 4i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSeqFinish_PreUpdate", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSeqFinish_Update(c: &mut Ctx, mut p_ctx_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSeqFinish_Update", &PROBE_N, &[(p_ctx_1) as u32], false); }
    let r_ = 'f: {
    if ((F::Ramp_Step(c, (p_ctx_1).wrapping_add(8))) != 0) {
        if (((((((((F::Battle_GetMode(c)) != (8i32)) as i32)) != 0) && ((F::BattleResult_HasWinner(c)) != 0)) as i32)) != 0) {
            break 'f 5i32;
        }
        break 'f 6i32;
    }
    break 'f 4i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSeqFinish_Update", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSeqFinish_Exit(c: &mut Ctx, mut p_ctx_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSeqFinish_Exit", &PROBE_N, &[(p_ctx_1) as u32], false); }
    let r_ = 'f: {
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSeqFinish_Exit", &PROBE_N, &[], true); }
    r_
}
