#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn FontIcon_Stub23C310(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "FontIcon_Stub23C310", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "FontIcon_Stub23C310", &PROBE_N, &[], true); }
    r_
}

pub fn FontIcon_Stub23C318(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "FontIcon_Stub23C318", &PROBE_N, &[], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "FontIcon_Stub23C318", &PROBE_N, &[], true); }
    r_
}

pub fn FontIcon_Stub23C320(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "FontIcon_Stub23C320", &PROBE_N, &[], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "FontIcon_Stub23C320", &PROBE_N, &[], true); }
    r_
}

pub fn FontIcon_Upload(c: &mut Ctx, mut p_vramBase_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "FontIcon_Upload", &PROBE_N, &[(p_vramBase_1) as u32], false); }
    let r_ = 'f: {
    { let a0_ = c.m.r32(sym::gFontIconRes); F::TexFile_UploadAll(c, a0_, (p_vramBase_1).wrapping_add(64i32), p_vramBase_1) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "FontIcon_Upload", &PROBE_N, &[], true); }
    r_
}

pub fn FontIcon_IsOnScreen(c: &mut Ctx, mut p_x0_1: i32, mut p_y0_2: i32, mut p_x1_3: i32, mut p_y1_4: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "FontIcon_IsOnScreen", &PROBE_N, &[(p_x0_1) as u32, (p_y0_2) as u32, (p_x1_3) as u32, (p_y1_4) as u32], false); }
    let mut l_result_5: i32 = 0;
    let r_ = 'f: {
    l_result_5 = 0i32;
    if (((((((((((((((((p_x0_1) > (0i32)) as i32)) != 0) && (((((p_x1_3) < (4096i32)) as i32)) != 0)) as i32)) != 0) && (((((p_y0_2) > (0i32)) as i32)) != 0)) as i32)) != 0) && (((((p_y1_4) < (4096i32)) as i32)) != 0)) as i32)) != 0) {
        l_result_5 = 1i32;
    }
    break 'f l_result_5;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "FontIcon_IsOnScreen", &PROBE_N, &[], true); }
    r_
}

pub fn FontIcon_PutSprite(c: &mut Ctx, mut p_pkt_1: u32, mut p_cmd_2: u32, mut p_icon_3: u32, mut p_frame_4: i32, mut p_x_5: i32, mut p_y_6: i32, mut p_scale_7: F32, mut p_color_8: u32, mut p_vramBase_9: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "FontIcon_PutSprite", &PROBE_N, &[(p_pkt_1) as u32, (p_cmd_2) as u32, (p_icon_3) as u32, (p_frame_4) as u32, (p_x_5) as u32, (p_y_6) as u32, (p_scale_7).0, (p_color_8) as u32, (p_vramBase_9) as u32], false); }
    let mut l_tex_10: u32 = 0;
    let mut l_size_11: i32 = 0;
    let mut l_tex0_12: u64 = 0;
    let mut l_reg_13: u64 = 0;
    let mut l_tbp_14: i32 = 0;
    let mut l_cbp_15: i32 = 0;
    let mut l_drawn_16: i32 = 0;
    let mut l_perRow_17: i32 = 0;
    let mut l_x0_18: i32 = 0;
    let mut l_y0_19: i32 = 0;
    let mut l_x1_20: i32 = 0;
    let mut l_y1_21: i32 = 0;
    let mut l_tu0_22: i32 = 0;
    let mut l_tv0_23: i32 = 0;
    let mut l_tu1_24: i32 = 0;
    let mut l_tv1_25: i32 = 0;
    let r_ = 'f: {
    l_tex_10 = ({ let a_ = (c.m.r32(sym::gFontIconRes)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((c.m.r8(p_icon_3)) as u32)).wrapping_mul(64));
    l_size_11 = ((c.m.r8((p_icon_3).wrapping_add(1))) as i32);
    l_tex0_12 = c.m.r64((l_tex_10).wrapping_add(48));
    l_drawn_16 = rt::f2i(rt::fmul(rt::i2f(l_size_11), p_scale_7));
    l_perRow_17 = rt::div_i32((1i32).wrapping_shl(((((l_tex0_12).wrapping_shr((26i32) as u32)) & (((15i32) as u64)))) as u32), l_size_11);
    l_tbp_14 = (((c.m.r32((l_tex_10).wrapping_add(32)) as i32)).wrapping_add(p_vramBase_9)).wrapping_add(64i32);
    l_cbp_15 = ((c.m.r32((l_tex_10).wrapping_add(36)) as i32)).wrapping_add(p_vramBase_9);
    { let r_ = (((l_cbp_15) as u64)).wrapping_shl((37i32) as u32); let n_ = ((l_tex0_12) | (r_)); l_tex0_12 = n_; };
    l_reg_13 = ((((l_tex0_12) | (((l_tbp_14) as u64)))) | (((17179869184i64) as u64)));
    l_x0_18 = (p_x_5).wrapping_add(1792i32);
    l_y0_19 = (p_y_6).wrapping_add(1824i32);
    l_x1_20 = ((p_x_5).wrapping_add(l_drawn_16)).wrapping_add(1792i32);
    l_y1_21 = ((p_y_6).wrapping_add(l_drawn_16)).wrapping_add(1824i32);
    if ((F::FontIcon_IsOnScreen(c, l_x0_18, l_y0_19, l_x1_20, l_y1_21)) != 0) {
        l_tu0_22 = (l_size_11).wrapping_mul(rt::rem_i32(p_frame_4, l_perRow_17));
        l_tv0_23 = (l_size_11).wrapping_mul(rt::div_i32(p_frame_4, l_perRow_17));
        l_tu1_24 = ((rt::rem_i32(p_frame_4, l_perRow_17)).wrapping_add(1i32)).wrapping_mul(l_size_11);
        l_tv1_25 = ((rt::div_i32(p_frame_4, l_perRow_17)).wrapping_add(1i32)).wrapping_mul(l_size_11);
        { let v_ = l_reg_13; let a_ = (c.m.r32(p_pkt_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((((p_color_8) as u64)) | ((((1065353216i32) as u64)).wrapping_shl((32i32) as u32))); let a_ = (c.m.r32(p_pkt_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let a_ = p_pkt_1; let n_ = (c.m.r32(a_)).wrapping_add((((r_) as u32)).wrapping_mul(8)); c.m.w32(a_, n_); };
        { let v_ = (((((l_tu0_22) as u64)).wrapping_shl((4i32) as u32)) | ((((l_tv0_23) as u64)).wrapping_shl((20i32) as u32))); let a_ = (c.m.r32(p_pkt_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = (((((((l_x0_18) as u64)).wrapping_shl((4i32) as u32)) | ((((l_y0_19) as u64)).wrapping_shl((20i32) as u32)))) | ((((4294967280u32) as u64)).wrapping_shl((32i32) as u32))); let a_ = (c.m.r32(p_pkt_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let a_ = p_pkt_1; let n_ = (c.m.r32(a_)).wrapping_add((((r_) as u32)).wrapping_mul(8)); c.m.w32(a_, n_); };
        { let v_ = (((((l_tu1_24) as u64)).wrapping_shl((4i32) as u32)) | ((((l_tv1_25) as u64)).wrapping_shl((20i32) as u32))); let a_ = (c.m.r32(p_pkt_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = (((((((l_x1_20) as u64)).wrapping_shl((4i32) as u32)) | ((((l_y1_21) as u64)).wrapping_shl((20i32) as u32)))) | ((((4294967280u32) as u64)).wrapping_shl((32i32) as u32))); let a_ = (c.m.r32(p_pkt_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let a_ = p_pkt_1; let n_ = (c.m.r32(a_)).wrapping_add((((r_) as u32)).wrapping_mul(8)); c.m.w32(a_, n_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "FontIcon_PutSprite", &PROBE_N, &[], true); }
    r_
}

pub fn FontIcon_GetAnimLength(c: &mut Ctx, mut p_entry_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "FontIcon_GetAnimLength", &PROBE_N, &[(p_entry_1) as u32], false); }
    let mut l_total_2: i32 = 0;
    let mut l_i_3: i32 = 0;
    let r_ = 'f: {
    l_total_2 = 0i32;
    {
        l_i_3 = 0i32;
        'l1: loop {
            if !(((((l_i_3) < ((({ let a_ = (c.m.r32((p_entry_1).wrapping_add(4))).wrapping_add(4); c.m.r16(a_) }) as i32))) as i32)) != 0) { break 'l1; }
            'c2: {
                { let r_ = (({ let a_ = ((c.m.r32((p_entry_1).wrapping_add(4))).wrapping_add(6)).wrapping_add(((l_i_3) as u32)); (c.m.r8(a_) as i8) }) as i32); let n_ = (l_total_2).wrapping_add(r_); l_total_2 = n_; };
            }
            { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
        }
    }
    break 'f l_total_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "FontIcon_GetAnimLength", &PROBE_N, &[], true); }
    r_
}

pub fn FontIcon_GetTotalLength(c: &mut Ctx, mut p_str_1: u32, mut p_padType_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "FontIcon_GetTotalLength", &PROBE_N, &[(p_str_1) as u32, (p_padType_2) as u32], false); }
    let mut l_total_3: i32 = 0;
    let mut l_entry_4: u32 = 0;
    let r_ = 'f: {
    l_total_3 = 0i32;
    'l1: loop {
        'c2: {
            if (((((((c.m.r16(p_str_1)) as i32)) == (0i32)) as i32)) != 0) {
                break 'l1;
            }
            l_entry_4 = F::FontIcon_Find(c, p_str_1, p_padType_2);
            if (((((l_entry_4) == (((0i32) as u32))) as i32)) != 0) {
                break 'l1;
            }
            { let r_ = F::FontIcon_GetAnimLength(c, l_entry_4); let n_ = (l_total_3).wrapping_add(r_); l_total_3 = n_; };
            { let r_ = { let a_ = (c.m.r32(l_entry_4)).wrapping_add(8); (c.m.r32(a_) as i32) }; let n_ = (p_str_1).wrapping_add((((r_) as u32)).wrapping_mul(2)); p_str_1 = n_; };
        }
        if !((1i32) != 0) { break 'l1; }
    }
    break 'f l_total_3;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "FontIcon_GetTotalLength", &PROBE_N, &[], true); }
    r_
}

pub fn FontIcon_GetMaxSize(c: &mut Ctx, mut p_str_1: u32, mut p_padType_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "FontIcon_GetMaxSize", &PROBE_N, &[(p_str_1) as u32, (p_padType_2) as u32], false); }
    let mut l_max_3: i32 = 0;
    let mut l_entry_4: u32 = 0;
    let r_ = 'f: {
    l_max_3 = 0i32;
    'l1: while (((((((c.m.r16(p_str_1)) as i32)) != (0i32)) as i32)) != 0) {
        l_entry_4 = F::FontIcon_Find(c, p_str_1, p_padType_2);
        if (((((l_entry_4) != (((0i32) as u32))) as i32)) != 0) {
            if (((((l_max_3) < ((({ let a_ = (c.m.r32((l_entry_4).wrapping_add(4))).wrapping_add(1); c.m.r8(a_) }) as i32))) as i32)) != 0) {
                l_max_3 = (({ let a_ = (c.m.r32((l_entry_4).wrapping_add(4))).wrapping_add(1); c.m.r8(a_) }) as i32);
            }
            { let r_ = { let a_ = (c.m.r32(l_entry_4)).wrapping_add(8); (c.m.r32(a_) as i32) }; let n_ = (p_str_1).wrapping_add((((r_) as u32)).wrapping_mul(2)); p_str_1 = n_; };
        } else {
            break 'l1;
        }
    }
    break 'f l_max_3;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "FontIcon_GetMaxSize", &PROBE_N, &[], true); }
    r_
}

pub fn FontIcon_GetFrame(c: &mut Ctx, mut p_entry_1: u32, mut p_icon_2: u32, mut p_length_3: i32, mut p_start_4: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "FontIcon_GetFrame", &PROBE_N, &[(p_entry_1) as u32, (p_icon_2) as u32, (p_length_3) as u32, (p_start_4) as u32], false); }
    let mut l_frame_5: i32 = 0;
    let mut l_t_6: i32 = 0;
    let mut l_sum_7: i32 = 0;
    let mut l_i_8: i32 = 0;
    let mut l_n_9: i32 = 0;
    let r_ = 'f: {
    l_frame_5 = 0i32;
    if (((((((c.m.r16((p_icon_2).wrapping_add(4))) as i32)) >= (2i32)) as i32)) != 0) {
        l_t_6 = (rt::rem_i32((c.m.r32(sym::gFontIconFrame) as i32), p_length_3)).wrapping_sub(p_start_4);
        if (((((l_t_6) > (0i32)) as i32)) != 0) {
            l_sum_7 = 0i32;
            l_n_9 = ((c.m.r16((p_icon_2).wrapping_add(4))) as i32);
            l_frame_5 = (l_n_9).wrapping_sub(1i32);
            {
                l_i_8 = 0i32;
                'l1: loop {
                    if !(((((l_i_8) < (l_n_9)) as i32)) != 0) { break 'l1; }
                    'c2: {
                        if (((((l_t_6) < ((l_sum_7).wrapping_add((((c.m.r8(((p_icon_2).wrapping_add(6)).wrapping_add(((l_i_8) as u32))) as i8)) as i32)))) as i32)) != 0) {
                            l_frame_5 = l_i_8;
                            break 'l1;
                        }
                        { let r_ = (((c.m.r8(((p_icon_2).wrapping_add(6)).wrapping_add(((l_i_8) as u32))) as i8)) as i32); let n_ = (l_sum_7).wrapping_add(r_); l_sum_7 = n_; };
                    }
                    { let o_ = l_i_8; let n_ = (o_).wrapping_add(1); l_i_8 = n_; };
                }
            }
        }
    }
    break 'f l_frame_5;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "FontIcon_GetFrame", &PROBE_N, &[], true); }
    r_
}

pub fn FontIcon_Find(c: &mut Ctx, mut p_str_1: u32, mut p_padType_2: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "FontIcon_Find", &PROBE_N, &[(p_str_1) as u32, (p_padType_2) as u32], false); }
    let mut l_entry_3: u32 = 0;
    let mut l_best_4: u32 = 0;
    let mut l_name_5: u32 = 0;
    let r_ = 'f: {
    l_entry_3 = c.m.r32((sym::gFontIconTables).wrapping_add((((p_padType_2) as u32)).wrapping_mul(4)));
    l_best_4 = ((0i32) as u32);
    'l1: while (((((c.m.r32(l_entry_3)) != (((0i32) as u32))) as i32)) != 0) {
        l_name_5 = c.m.r32(l_entry_3);
        if (({ let a2_ = (c.m.r32((l_name_5).wrapping_add(8)) as i32); F::FontTag_Match(c, p_str_1, l_name_5, a2_) }) != 0) {
            if (((((((((l_best_4) == (((0i32) as u32))) as i32)) != 0) || ((((({ let a_ = (c.m.r32(l_best_4)).wrapping_add(8); (c.m.r32(a_) as i32) }) < ((c.m.r32((l_name_5).wrapping_add(8)) as i32))) as i32)) != 0)) as i32)) != 0) {
                l_best_4 = l_entry_3;
            }
        }
        { let o_ = l_entry_3; let n_ = (o_).wrapping_add(12); l_entry_3 = n_; };
    }
    break 'f l_best_4;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "FontIcon_Find", &PROBE_N, &[], true); }
    r_
}

pub fn FontTag_Match(c: &mut Ctx, mut p_str_1: u32, mut p_name_2: u32, mut p_len_3: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "FontTag_Match", &PROBE_N, &[(p_str_1) as u32, (p_name_2) as u32, (p_len_3) as u32], false); }
    let mut l_result_4: i32 = 0;
    let mut l_i_5: i32 = 0;
    let r_ = 'f: {
    l_result_4 = 1i32;
    {
        l_i_5 = 0i32;
        'l1: loop {
            if !(((((l_i_5) < (p_len_3)) as i32)) != 0) { break 'l1; }
            'c2: {
                if (((((((((((c.m.r16(p_str_1)) as i32)) == (0i32)) as i32)) != 0) || (((((((c.m.r16({ let o_ = p_str_1; let n_ = (o_).wrapping_add(2); p_str_1 = n_; o_ })) as i32)) != (((c.m.r16({ let o_ = p_name_2; let n_ = (o_).wrapping_add(2); p_name_2 = n_; o_ })) as i32))) as i32)) != 0)) as i32)) != 0) {
                    l_result_4 = 0i32;
                    break 'l1;
                }
            }
            { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
        }
    }
    break 'f l_result_4;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "FontTag_Match", &PROBE_N, &[], true); }
    r_
}

pub fn FontTag_CopyArg(c: &mut Ctx, mut p_src_1: u32, mut p_dst_2: u32, mut p_max_3: i32, mut p_stop_4: u16) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "FontTag_CopyArg", &PROBE_N, &[(p_src_1) as u32, (p_dst_2) as u32, (p_max_3) as u32, (p_stop_4) as u32], false); }
    let mut l_p_5: u32 = 0;
    let mut l_i_6: i32 = 0;
    let r_ = 'f: {
    l_p_5 = c.m.r32(p_src_1);
    {
        l_i_6 = 0i32;
        'l1: loop {
            if !(((((l_i_6) < (p_max_3)) as i32)) != 0) { break 'l1; }
            'c2: {
                if (((((((c.m.r16(l_p_5)) as i32)) == (((p_stop_4) as i32))) as i32)) != 0) {
                    break 'l1;
                }
                { let v_ = c.m.r16({ let o_ = l_p_5; let n_ = (o_).wrapping_add(2); l_p_5 = n_; o_ }); let a_ = { let o_ = p_dst_2; let n_ = (o_).wrapping_add(2); p_dst_2 = n_; o_ }; c.m.w16(a_, v_); };
            }
            { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; };
        }
    }
    { let v_ = ((0i32) as u16); let a_ = p_dst_2; c.m.w16(a_, v_); };
    { let v_ = l_p_5; let a_ = p_src_1; c.m.w32(a_, v_); };
    break 'f (((l_i_6) == (p_max_3)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "FontTag_CopyArg", &PROBE_N, &[], true); }
    r_
}

pub fn FontTag_ToAscii(c: &mut Ctx, mut p_src_1: u32, mut p_dst_2: u32, mut p_max_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "FontTag_ToAscii", &PROBE_N, &[(p_src_1) as u32, (p_dst_2) as u32, (p_max_3) as u32], false); }
    let mut l_i_4: i32 = 0;
    let r_ = 'f: {
    {
        l_i_4 = 0i32;
        'l1: loop {
            if !(((((l_i_4) < (p_max_3)) as i32)) != 0) { break 'l1; }
            'c2: {
                if (((((((c.m.r16(p_src_1)) as i32)) == (0i32)) as i32)) != 0) {
                    break 'l1;
                }
                { let v_ = ((c.m.r8(p_src_1)) as i8); let a_ = { let o_ = p_dst_2; let n_ = (o_).wrapping_add(1); p_dst_2 = n_; o_ }; c.m.w8(a_, (v_) as u8); };
                { let o_ = p_src_1; let n_ = (o_).wrapping_add(2); p_src_1 = n_; };
            }
            { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "FontTag_ToAscii", &PROBE_N, &[], true); }
    r_
}

pub fn FontIcon_Begin(c: &mut Ctx, mut p_vramBase_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "FontIcon_Begin", &PROBE_N, &[(p_vramBase_1) as u32], false); }
    let r_ = 'f: {
    if ((((!(((c.m.r32(sym::gFontIconUploaded) as i32)) != 0)) as i32)) != 0) {
        F::FontIcon_Upload(c, p_vramBase_1);
        { let v_ = 1i32; let a_ = sym::gFontIconUploaded; c.m.w32(a_, (v_) as u32); };
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "FontIcon_Begin", &PROBE_N, &[], true); }
    r_
}

pub fn FontTag_Handle(c: &mut Ctx, mut p_pkt_1: u32, mut p_cmd_2: u32, mut p_saved_3: u32, mut p_x_4: u32, mut p_y_5: u32, mut p_str_6: u32, mut p_vramBase_7: i32, mut p_draw_8: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "FontTag_Handle", &PROBE_N, &[(p_pkt_1) as u32, (p_cmd_2) as u32, (p_saved_3) as u32, (p_x_4) as u32, (p_y_5) as u32, (p_str_6) as u32, (p_vramBase_7) as u32, (p_draw_8) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_p_9: u32 = 0;
    let mut l_def_10: u32 = 0;
    let mut l_i_11: u32 = 0;
    let mut l_ch_12: u16 = 0;
    let r_ = 'f: {
        l_p_9 = c.m.r32(p_str_6);
        { let v_ = 0i32; let a_ = sym::gFontIconUploaded; c.m.w32(a_, (v_) as u32); };
        if (((((((c.m.r16(l_p_9)) as i32)) == (60i32)) as i32)) != 0) {
            l_def_10 = sym::gFontTags;
            {
                l_i_11 = ((0i32) as u32);
                'l1: loop {
                    if !(((((l_i_11) < (((4i32) as u32))) as i32)) != 0) { break 'l1; }
                    'c2: {
                        if ((((!(({ let a1_ = c.m.r32(l_def_10); let a2_ = ((c.m.r16((l_def_10).wrapping_add(4))) as i32); F::FontTag_Match(c, (l_p_9).wrapping_add((((1i32) as u32)).wrapping_mul(2)), a1_, a2_) }) != 0)) as i32)) != 0) {
                            break 'c2;
                        }
                        l_ch_12 = { let a_ = (l_p_9).wrapping_add(((((((c.m.r16((l_def_10).wrapping_add(4))) as i32)).wrapping_add(1i32)) as u32)).wrapping_mul(2)); c.m.r16(a_) };
                        if ((((((c.m.r32((l_def_10).wrapping_add(12)) as i32)) == (1i32)) as i32)) != 0) {
                            if (((((((l_ch_12) as i32)) != (61i32)) as i32)) != 0) {
                                { let a2_ = ((c.m.r16((l_def_10).wrapping_add(4))) as i32); F::FontTag_ToAscii(c, l_p_9, fp, a2_) };
                                break 'l1;
                            }
                        } else {
                            if (((((((l_ch_12) as i32)) != (62i32)) as i32)) != 0) {
                                { let a2_ = ((c.m.r16((l_def_10).wrapping_add(4))) as i32); F::FontTag_ToAscii(c, l_p_9, fp, a2_) };
                                break 'l1;
                            }
                        }
                        { let fp_ = c.m.r32((l_def_10).wrapping_add(8)); { let _ = D::call(c, fp_, &[((l_def_10) as u64), ((p_pkt_1) as u64), ((p_cmd_2) as u64), ((p_saved_3) as u64), ((p_x_4) as u64), ((p_y_5) as u64), ((p_str_6) as u64), ((p_vramBase_7) as i64 as u64), ((p_draw_8) as i64 as u64)]); } };
                        l_ch_12 = { let a_ = c.m.r32(p_str_6); c.m.r16(a_) };
                        if (((((((l_ch_12) as i32)) != (62i32)) as i32)) != 0) {
                            { let a2_ = ((c.m.r16((l_def_10).wrapping_add(4))) as i32); F::FontTag_ToAscii(c, l_p_9, fp, a2_) };
                            break 'l1;
                        }
                        { let r_ = 1i32; let a_ = p_str_6; let n_ = (c.m.r32(a_)).wrapping_add((((r_) as u32)).wrapping_mul(2)); c.m.w32(a_, n_); };
                        F::FontTag_Handle(c, p_pkt_1, p_cmd_2, p_saved_3, p_x_4, p_y_5, p_str_6, p_vramBase_7, p_draw_8);
                        break 'l1;
                    }
                    { { let o_ = l_i_11; let n_ = (o_).wrapping_add(1); l_i_11 = n_; }; { let o_ = l_def_10; let n_ = (o_).wrapping_add(16); l_def_10 = n_; }; };
                }
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "FontTag_Handle", &PROBE_N, &[], true); }
    r_
}

pub fn FontTag_Pad(c: &mut Ctx, mut p_def_1: u32, mut p_pkt_2: u32, mut p_cmd_3: u32, mut p_saved_4: u32, mut p_x_5: u32, mut p_y_6: u32, mut p_str_7: u32, mut p_vramBase_8: i32, mut p_draw_9: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "FontTag_Pad", &PROBE_N, &[(p_def_1) as u32, (p_pkt_2) as u32, (p_cmd_3) as u32, (p_saved_4) as u32, (p_x_5) as u32, (p_y_6) as u32, (p_str_7) as u32, (p_vramBase_8) as u32, (p_draw_9) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(80) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 80);
    let mut l_padTypePtr_10: u32 = 0;
    let mut l_padType_11: i32 = 0;
    let mut l_max_12: i32 = 0;
    let mut l_s_13: u32 = 0;
    let mut l_entry_14: u32 = 0;
    let mut l_icon_15: u32 = 0;
    let mut l_name_16: u32 = 0;
    let mut l_frame_17: i32 = 0;
    let mut l_dy_18: i32 = 0;
    let r_ = 'f: {
        { let v_ = ((c.m.r32(p_str_7)).wrapping_add((((((c.m.r16((p_def_1).wrapping_add(4))) as i32)) as u32)).wrapping_mul(2))).wrapping_add((((2i32) as u32)).wrapping_mul(2)); let a_ = fp.wrapping_add(64); c.m.w32(a_, v_); };
        l_padTypePtr_10 = c.m.r32((p_cmd_3).wrapping_add(80));
        { let _ = F::FontTag_CopyArg(c, fp.wrapping_add(64), fp, 31i32, ((62i32) as u16)); };
        l_padType_11 = (c.m.r32(l_padTypePtr_10) as i32);
        { let _ = F::FontIcon_GetTotalLength(c, fp, l_padType_11); };
        l_max_12 = F::FontIcon_GetMaxSize(c, fp, l_padType_11);
        {
            l_s_13 = fp;
            'l1: loop {
                if !(((((((c.m.r16(l_s_13)) as i32)) != (0i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_entry_14 = F::FontIcon_Find(c, l_s_13, l_padType_11);
                    if (((((l_entry_14) != (((0i32) as u32))) as i32)) != 0) {
                        l_icon_15 = c.m.r32((l_entry_14).wrapping_add(4));
                        l_name_16 = c.m.r32(l_entry_14);
                        l_frame_17 = { let a2_ = F::FontIcon_GetAnimLength(c, l_entry_14); F::FontIcon_GetFrame(c, l_entry_14, l_icon_15, a2_, 0i32) };
                        's3: {
                            let sv19: i32 = ((c.m.r8((l_icon_15).wrapping_add(1))) as i32);
                            match sv19 {
                                32i32 => {
                                    l_dy_18 = (rt::div_i32((l_max_12).wrapping_sub(32i32), 2i32)).wrapping_sub(6i32);
                                    break 's3;
                                }
                                64i32 => {
                                    l_dy_18 = rt::div_i32((l_max_12).wrapping_sub(64i32), 2i32);
                                    break 's3;
                                }
                                128i32 => {
                                    l_dy_18 = (rt::div_i32((l_max_12).wrapping_sub(128i32), 2i32)).wrapping_sub(6i32);
                                    break 's3;
                                }
                                _ => {
                                    l_dy_18 = (rt::div_i32((l_max_12).wrapping_sub(((c.m.r8((l_icon_15).wrapping_add(1))) as i32)), 2i32)).wrapping_sub(6i32);
                                    break 's3;
                                }
                            }
                        }
                        if ((p_draw_9) != 0) {
                            { let a4_ = (c.m.r32(p_x_5) as i32); let a5_ = ((c.m.r32(p_y_6) as i32)).wrapping_add(l_dy_18); let a6_ = F32(c.m.r32((l_entry_14).wrapping_add(8))); let a7_ = c.m.r32((p_cmd_3).wrapping_add(32)); F::FontIcon_PutSprite(c, p_pkt_2, p_cmd_3, l_icon_15, l_frame_17, a4_, a5_, a6_, a7_, p_vramBase_8) };
                        }
                        { let r_ = (rt::f2i(rt::fmul(rt::i2f((c.m.r8((l_icon_15).wrapping_add(2))) as i32), F32(c.m.r32((l_entry_14).wrapping_add(8)))))).wrapping_add(((c.m.r16((p_cmd_3).wrapping_add(24))) as i32)); let a_ = p_x_5; let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                    } else {
                        break 'l1;
                    }
                }
                { let r_ = (c.m.r32((l_name_16).wrapping_add(8)) as i32); let n_ = (l_s_13).wrapping_add((((r_) as u32)).wrapping_mul(2)); l_s_13 = n_; };
            }
        }
        { let v_ = c.m.r32(fp.wrapping_add(64)); let a_ = p_str_7; c.m.w32(a_, v_); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "FontTag_Pad", &PROBE_N, &[], true); }
    r_
}

pub fn FontTag_PadSequence(c: &mut Ctx, mut p_def_1: u32, mut p_pkt_2: u32, mut p_cmd_3: u32, mut p_saved_4: u32, mut p_x_5: u32, mut p_y_6: u32, mut p_str_7: u32, mut p_vramBase_8: i32, mut p_draw_9: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "FontTag_PadSequence", &PROBE_N, &[(p_def_1) as u32, (p_pkt_2) as u32, (p_cmd_3) as u32, (p_saved_4) as u32, (p_x_5) as u32, (p_y_6) as u32, (p_str_7) as u32, (p_vramBase_8) as u32, (p_draw_9) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(272) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 272);
    let mut l_padTypePtr_10: u32 = 0;
    let mut l_padType_11: i32 = 0;
    let mut l_total_12: i32 = 0;
    let mut l_start_13: i32 = 0;
    let mut l_max_14: i32 = 0;
    let mut l_s_15: u32 = 0;
    let mut l_entry_16: u32 = 0;
    let mut l_icon_17: u32 = 0;
    let mut l_name_18: u32 = 0;
    let mut l_frame_19: i32 = 0;
    let mut l_dy_20: i32 = 0;
    let r_ = 'f: {
        { let v_ = ((c.m.r32(p_str_7)).wrapping_add((((((c.m.r16((p_def_1).wrapping_add(4))) as i32)) as u32)).wrapping_mul(2))).wrapping_add((((2i32) as u32)).wrapping_mul(2)); let a_ = fp.wrapping_add(256); c.m.w32(a_, v_); };
        l_start_13 = 0i32;
        l_padTypePtr_10 = c.m.r32((p_cmd_3).wrapping_add(80));
        { let _ = F::FontTag_CopyArg(c, fp.wrapping_add(256), (fp).wrapping_add((((0i32) as u32)).wrapping_mul(64)), 31i32, ((44i32) as u16)); };
        { let a_ = fp.wrapping_add(256); let o_ = c.m.r32(a_); let n_ = (o_).wrapping_add(2); c.m.w32(a_, n_); };
        { let _ = F::FontTag_CopyArg(c, fp.wrapping_add(256), (fp).wrapping_add((((1i32) as u32)).wrapping_mul(64)), 31i32, ((44i32) as u16)); };
        { let a_ = fp.wrapping_add(256); let o_ = c.m.r32(a_); let n_ = (o_).wrapping_add(2); c.m.w32(a_, n_); };
        { let _ = F::FontTag_CopyArg(c, fp.wrapping_add(256), (fp).wrapping_add((((2i32) as u32)).wrapping_mul(64)), 31i32, ((44i32) as u16)); };
        { let a_ = fp.wrapping_add(256); let o_ = c.m.r32(a_); let n_ = (o_).wrapping_add(2); c.m.w32(a_, n_); };
        { let _ = F::FontTag_CopyArg(c, fp.wrapping_add(256), (fp).wrapping_add((((3i32) as u32)).wrapping_mul(64)), 31i32, ((62i32) as u16)); };
        l_padType_11 = (c.m.r32(l_padTypePtr_10) as i32);
        l_total_12 = F::FontIcon_GetTotalLength(c, (fp).wrapping_add((((l_padType_11) as u32)).wrapping_mul(64)), l_padType_11);
        l_max_14 = F::FontIcon_GetMaxSize(c, (fp).wrapping_add((((l_padType_11) as u32)).wrapping_mul(64)), l_padType_11);
        {
            l_s_15 = (fp).wrapping_add((((l_padType_11) as u32)).wrapping_mul(64));
            'l1: loop {
                if !(((((((c.m.r16(l_s_15)) as i32)) != (0i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_entry_16 = F::FontIcon_Find(c, l_s_15, l_padType_11);
                    if (((((l_entry_16) != (((0i32) as u32))) as i32)) != 0) {
                        l_icon_17 = c.m.r32((l_entry_16).wrapping_add(4));
                        l_name_18 = c.m.r32(l_entry_16);
                        l_frame_19 = F::FontIcon_GetFrame(c, l_entry_16, l_icon_17, l_total_12, l_start_13);
                        { let r_ = F::FontIcon_GetAnimLength(c, l_entry_16); let n_ = (l_start_13).wrapping_add(r_); l_start_13 = n_; };
                        's3: {
                            let sv21: i32 = ((c.m.r8((l_icon_17).wrapping_add(1))) as i32);
                            match sv21 {
                                32i32 => {
                                    l_dy_20 = (rt::div_i32((l_max_14).wrapping_sub(32i32), 2i32)).wrapping_sub(6i32);
                                    break 's3;
                                }
                                64i32 => {
                                    l_dy_20 = rt::div_i32((l_max_14).wrapping_sub(64i32), 2i32);
                                    break 's3;
                                }
                                128i32 => {
                                    l_dy_20 = (rt::div_i32((l_max_14).wrapping_sub(128i32), 2i32)).wrapping_sub(6i32);
                                    break 's3;
                                }
                                _ => {
                                    l_dy_20 = (rt::div_i32((l_max_14).wrapping_sub(((c.m.r8((l_icon_17).wrapping_add(1))) as i32)), 2i32)).wrapping_sub(6i32);
                                    break 's3;
                                }
                            }
                        }
                        if ((p_draw_9) != 0) {
                            { let a4_ = (c.m.r32(p_x_5) as i32); let a5_ = ((c.m.r32(p_y_6) as i32)).wrapping_add(l_dy_20); let a6_ = F32(c.m.r32((l_entry_16).wrapping_add(8))); let a7_ = c.m.r32((p_cmd_3).wrapping_add(32)); F::FontIcon_PutSprite(c, p_pkt_2, p_cmd_3, l_icon_17, l_frame_19, a4_, a5_, a6_, a7_, p_vramBase_8) };
                        }
                        { let r_ = (rt::f2i(rt::fmul(rt::i2f((c.m.r8((l_icon_17).wrapping_add(2))) as i32), F32(c.m.r32((l_entry_16).wrapping_add(8)))))).wrapping_add(((c.m.r16((p_cmd_3).wrapping_add(24))) as i32)); let a_ = p_x_5; let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                    } else {
                        break 'l1;
                    }
                }
                { let r_ = (c.m.r32((l_name_18).wrapping_add(8)) as i32); let n_ = (l_s_15).wrapping_add((((r_) as u32)).wrapping_mul(2)); l_s_15 = n_; };
            }
        }
        { let v_ = c.m.r32(fp.wrapping_add(256)); let a_ = p_str_7; c.m.w32(a_, v_); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "FontTag_PadSequence", &PROBE_N, &[], true); }
    r_
}

pub fn FontTag_Color(c: &mut Ctx, mut p_def_1: u32, mut p_pkt_2: u32, mut p_cmd_3: u32, mut p_saved_4: u32, mut p_x_5: u32, mut p_y_6: u32, mut p_str_7: u32, mut p_vramBase_8: i32, mut p_draw_9: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "FontTag_Color", &PROBE_N, &[(p_def_1) as u32, (p_pkt_2) as u32, (p_cmd_3) as u32, (p_saved_4) as u32, (p_x_5) as u32, (p_y_6) as u32, (p_str_7) as u32, (p_vramBase_8) as u32, (p_draw_9) as u32], false); }
    let mut l_p_10: u32 = 0;
    let mut l_s_11: u32 = 0;
    let mut l_color_12: u32 = 0;
    let mut l_hex_13: u32 = 0;
    let mut l_i_14: i32 = 0;
    let mut l_ch_15: u16 = 0;
    let r_ = 'f: {
    l_p_10 = (c.m.r32(p_str_7)).wrapping_add((((((c.m.r16((p_def_1).wrapping_add(4))) as i32)) as u32)).wrapping_mul(2));
    l_s_11 = (l_p_10).wrapping_add((((2i32) as u32)).wrapping_mul(2));
    if ((F::FontTag_Match(c, l_s_11, sym::D_002C6428, 3i32)) != 0) {
        l_color_12 = c.m.r32((p_saved_4).wrapping_add(28));
        l_s_11 = (l_p_10).wrapping_add((((5i32) as u32)).wrapping_mul(2));
    } else {
        l_hex_13 = ((0i32) as u32);
        {
            l_i_14 = 7i32;
            'l1: loop {
                if !(((((l_i_14) >= (0i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_ch_15 = c.m.r16({ let o_ = l_s_11; let n_ = (o_).wrapping_add(2); l_s_11 = n_; o_ });
                    if ((((((((((((l_ch_15) as i32)).wrapping_sub(48i32)) as u16)) as i32)) < (10i32)) as i32)) != 0) {
                        { let r_ = ((((((l_ch_15) as i32)).wrapping_sub(48i32)).wrapping_shl(((l_i_14).wrapping_mul(4i32)) as u32)) as u32); let n_ = ((l_hex_13) | (r_)); l_hex_13 = n_; };
                    } else {
                        if ((((((((((((l_ch_15) as i32)).wrapping_sub(65i32)) as u16)) as i32)) < (6i32)) as i32)) != 0) {
                            { let r_ = (((((((l_ch_15) as i32)).wrapping_sub(65i32)).wrapping_add(10i32)).wrapping_shl(((l_i_14).wrapping_mul(4i32)) as u32)) as u32); let n_ = ((l_hex_13) | (r_)); l_hex_13 = n_; };
                        } else {
                            break 'f;
                        }
                    }
                }
                { let o_ = l_i_14; let n_ = (o_).wrapping_sub(1); l_i_14 = n_; };
            }
        }
        l_color_12 = (((((((l_hex_13).wrapping_shr((16i32) as u32)) & (((255i32) as u32)))) | ((((l_hex_13) & (((255i32) as u32)))).wrapping_shl((16i32) as u32)))) | (((((l_hex_13).wrapping_shr((24i32) as u32)).wrapping_shl((24i32) as u32)) | (((l_hex_13) & (((65280i32) as u32)))))));
    }
    if ((p_draw_9) != 0) {
        { let v_ = l_color_12; let a_ = (p_cmd_3).wrapping_add(28); c.m.w32(a_, v_); };
    }
    { let v_ = l_s_11; let a_ = p_str_7; c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "FontTag_Color", &PROBE_N, &[], true); }
    r_
}

pub fn FontTag_Ub0(c: &mut Ctx, mut p_def_1: u32, mut p_pkt_2: u32, mut p_cmd_3: u32, mut p_saved_4: u32, mut p_x_5: u32, mut p_y_6: u32, mut p_str_7: u32, mut p_vramBase_8: i32, mut p_draw_9: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "FontTag_Ub0", &PROBE_N, &[(p_def_1) as u32, (p_pkt_2) as u32, (p_cmd_3) as u32, (p_saved_4) as u32, (p_x_5) as u32, (p_y_6) as u32, (p_str_7) as u32, (p_vramBase_8) as u32, (p_draw_9) as u32], false); }
    let mut l_p_10: u32 = 0;
    let r_ = 'f: {
    l_p_10 = ((c.m.r32(p_str_7)).wrapping_add((((((c.m.r16((p_def_1).wrapping_add(4))) as i32)) as u32)).wrapping_mul(2))).wrapping_add((((1i32) as u32)).wrapping_mul(2));
    if ((p_draw_9) != 0) {
        { let _ = F::Font_DrawCharNow(c, p_pkt_2, p_cmd_3, p_x_5, p_y_6, ((65296i32) as u16)); };
    }
    { let v_ = l_p_10; let a_ = p_str_7; c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "FontTag_Ub0", &PROBE_N, &[], true); }
    r_
}

pub fn FontIcon_Init(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "FontIcon_Init", &PROBE_N, &[], false); }
    let mut l_boot_1: u32 = 0;
    let r_ = 'f: {
    l_boot_1 = { let a_ = c.m.r32(sym::gCommonRes); c.m.r32(a_) };
    { let v_ = (l_boot_1).wrapping_add(((c.m.r32((l_boot_1).wrapping_add((((8i32) as u32)).wrapping_mul(4)))).wrapping_shr((2i32) as u32)).wrapping_mul(4)); let a_ = sym::gFontIconRes; c.m.w32(a_, v_); };
    { let a1_ = c.m.r32(sym::gFontIconRes); let a2_ = c.m.r32(sym::gFontIconRes); F::Res_RelocateOffsets(c, sym::gFontIconRes, a1_, a2_) };
    F::Font_SetBeginCallback(c, fid::FontIcon_Begin);
    F::Font_SetTagCallback(c, fid::FontTag_Handle, sym::gFontIconPadType);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "FontIcon_Init", &PROBE_N, &[], true); }
    r_
}

pub fn FontIcon_SetPadType(c: &mut Ctx, mut p_padType_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "FontIcon_SetPadType", &PROBE_N, &[(p_padType_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_padType_1; let a_ = sym::gFontIconPadType; c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "FontIcon_SetPadType", &PROBE_N, &[], true); }
    r_
}

pub fn FontIcon_GetRes(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "FontIcon_GetRes", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f c.m.r32(sym::gFontIconRes);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "FontIcon_GetRes", &PROBE_N, &[], true); }
    r_
}

pub fn FontIcon_GetDefs(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "FontIcon_GetDefs", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f sym::D_002C6838;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "FontIcon_GetDefs", &PROBE_N, &[], true); }
    r_
}

pub fn FontIcon_ResetAnim(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "FontIcon_ResetAnim", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let v_ = 0i32; let a_ = sym::gFontIconFrame; c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "FontIcon_ResetAnim", &PROBE_N, &[], true); }
    r_
}

pub fn FontIcon_Tick(c: &mut Ctx, mut p_vsyncs_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "FontIcon_Tick", &PROBE_N, &[(p_vsyncs_1) as u32], false); }
    let r_ = 'f: {
    if (((((p_vsyncs_1) == (1i32)) as i32)) != 0) {
        if ((((!(((((c.m.r32(sym::gFontIconTick) as i32)) & (1i32))) != 0)) as i32)) != 0) {
            { let a_ = sym::gFontIconFrame; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        }
        { let a_ = sym::gFontIconTick; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
    } else {
        { let a_ = sym::gFontIconFrame; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "FontIcon_Tick", &PROBE_N, &[], true); }
    r_
}

pub fn FontIcon_GetPadTagSize(c: &mut Ctx, mut p_str_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "FontIcon_GetPadTagSize", &PROBE_N, &[(p_str_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(80) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 80);
    let r_ = 'f: {
        { let v_ = (p_str_1).wrapping_add((((5i32) as u32)).wrapping_mul(2)); let a_ = fp.wrapping_add(64); c.m.w32(a_, v_); };
        { let _ = F::FontTag_CopyArg(c, fp.wrapping_add(64), fp, 31i32, ((62i32) as u16)); };
        break 'f { let a1_ = (c.m.r32(sym::gFontIconPadType) as i32); F::FontIcon_GetMaxSize(c, fp, a1_) };
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "FontIcon_GetPadTagSize", &PROBE_N, &[], true); }
    r_
}

pub fn FontIcon_Stub23D1E0(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "FontIcon_Stub23D1E0", &PROBE_N, &[], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "FontIcon_Stub23D1E0", &PROBE_N, &[], true); }
    r_
}
