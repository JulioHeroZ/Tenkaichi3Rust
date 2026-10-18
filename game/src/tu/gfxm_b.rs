#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn GfxPost_DrawDepthClut(c: &mut Ctx, mut p_mode_1: i32, mut p_tbp_2: i32, mut p_cbp_3: i32, mut p_alpha_4: u64) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "GfxPost_DrawDepthClut", &PROBE_N, &[(p_mode_1) as u32, (p_tbp_2) as u32, (p_cbp_3) as u32, (p_alpha_4) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(64) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 64);
    let mut l_p_5: u32 = 0;
    let mut l_tag_6: u32 = 0;
    let mut l_z_7: u32 = 0;
    let mut l_i_8: i32 = 0;
    let mut l_u1_9: i32 = 0;
    let mut l_w_10: i32 = 0;
    let mut l_tw_11: i32 = 0;
    let mut l_u_12: i32 = 0;
    let mut l_x_13: i32 = 0;
    let mut l_h_14: i32 = 0;
    let mut l_n_15: i32 = 0;
    let r_ = 'f: {
        l_w_10 = 32i32;
        l_tw_11 = 32i32;
        l_u_12 = 0i32;
        l_x_13 = 0i32;
        l_h_14 = 448i32;
        l_n_15 = rt::div_i32(512i32, l_w_10);
        l_p_5 = F::Dma_BeginDirect(c);
        { let v_ = ((((((((8i32) as u64)) | ((((1i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (l_p_5).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((14i32) as u64); let a_ = (l_p_5).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_5).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_5 = n_; };
        l_z_7 = ((65448i32) as u32);
        if (((((p_mode_1) != (0i32)) as i32)) != 0) {
            { let v_ = ((458752i32) as u64); let a_ = (l_p_5).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        } else {
            { let v_ = ((196608i32) as u64); let a_ = (l_p_5).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            l_z_7 = ((0i32) as u32);
        }
        { let v_ = ((71i32) as u64); let a_ = (l_p_5).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_5).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_5 = n_; };
        { let v_ = p_alpha_4; let a_ = (l_p_5).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((66i32) as u64); let a_ = (l_p_5).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_5).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_5 = n_; };
        { let v_ = ((0i32) as u64); let a_ = (l_p_5).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((63i32) as u64); let a_ = (l_p_5).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_5).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_5 = n_; };
        { let v_ = ((549755813888i64) as u64); let a_ = (l_p_5).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((59i32) as u64); let a_ = (l_p_5).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_5).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_5 = n_; };
        { let v_ = ((0i32) as u64); let a_ = (l_p_5).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((20i32) as u64); let a_ = (l_p_5).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_5).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_5 = n_; };
        { let v_ = ((((((((((p_tbp_2) | (632422400i32))) as u32)) as u64)) | ((((p_cbp_3) as u64)).wrapping_shl((37i32) as u32)))) | (((2305843070416977920i64) as u64))); let a_ = (l_p_5).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((6i32) as u64); let a_ = (l_p_5).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_5).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_5 = n_; };
        { let v_ = ((342i32) as u64); let a_ = (l_p_5).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((0i32) as u64); let a_ = (l_p_5).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_5).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_5 = n_; };
        { let v_ = ((4575657223564329088i64) as u64); let a_ = (l_p_5).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((1i32) as u64); let a_ = (l_p_5).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_5).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_5 = n_; };
        l_tag_6 = l_p_5;
        { let r_ = 2i32; let n_ = (l_p_5).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_5 = n_; };
        {
            l_i_8 = 0i32;
            'l1: loop {
                if !(((((l_i_8) < (l_n_15)) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_u1_9 = (l_u_12).wrapping_add(32i32);
                    F::GfxPostQuad_Set(c, fp, 1792i32, 1824i32, ((l_u_12) as i16), ((0i32) as i16), ((l_u1_9) as i16), ((l_h_14) as i16), l_x_13, 0i32, l_tw_11, l_h_14, ((1i32) as i16));
                    { let v_ = (((((((c.m.r32((fp).wrapping_add(32)) as i32)) as i64)) | (((((c.m.r32((fp).wrapping_add(36)) as i32)) as i64)).wrapping_shl((16i32) as u32)))) as u64); let a_ = (l_p_5).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                    { let v_ = (((((((((c.m.r32((fp).wrapping_add(16)) as i32)) as i64)) | (((((c.m.r32((fp).wrapping_add(20)) as i32)) as i64)).wrapping_shl((16i32) as u32)))) as u64)) | ((((l_z_7) as u64)).wrapping_shl((32i32) as u32))); let a_ = (l_p_5).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                    { let r_ = 2i32; let n_ = (l_p_5).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_5 = n_; };
                    { let v_ = (((((((c.m.r32((fp).wrapping_add(40)) as i32)) as i64)) | (((((c.m.r32((fp).wrapping_add(44)) as i32)) as i64)).wrapping_shl((16i32) as u32)))) as u64); let a_ = (l_p_5).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                    { let v_ = (((((((((c.m.r32((fp).wrapping_add(24)) as i32)) as i64)) | (((((c.m.r32((fp).wrapping_add(28)) as i32)) as i64)).wrapping_shl((16i32) as u32)))) as u64)) | ((((l_z_7) as u64)).wrapping_shl((32i32) as u32))); let a_ = (l_p_5).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                    { let r_ = 2i32; let n_ = (l_p_5).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_5 = n_; };
                    l_u_12 = l_u1_9;
                    { let r_ = 32i32; let n_ = (l_x_13).wrapping_add(r_); l_x_13 = n_; };
                }
                { let o_ = l_i_8; let n_ = (o_).wrapping_add(1); l_i_8 = n_; };
            }
        }
        { let v_ = ((((((((16i32) as u64)) | ((((1i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((4i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (l_tag_6).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((21331i32) as u64); let a_ = (l_tag_6).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        F::Dma_EndDirect(c, l_p_5);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "GfxPost_DrawDepthClut", &PROBE_N, &[], true); }
    r_
}

pub fn GfxPost_CopyAlphaToDepth(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "GfxPost_CopyAlphaToDepth", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(64) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 64);
    let mut l_p_1: u32 = 0;
    let mut l_tag_2: u32 = 0;
    let mut l_i_3: i32 = 0;
    let mut l_u1_4: i32 = 0;
    let mut l_w_5: i32 = 0;
    let mut l_tw_6: i32 = 0;
    let mut l_u_7: i32 = 0;
    let mut l_x_8: i32 = 0;
    let mut l_h_9: i32 = 0;
    let mut l_n_10: i32 = 0;
    let r_ = 'f: {
        l_w_5 = 32i32;
        l_tw_6 = 32i32;
        l_u_7 = 0i32;
        l_x_8 = 0i32;
        l_h_9 = 448i32;
        l_n_10 = rt::div_i32(512i32, l_w_5);
        l_p_1 = F::Dma_BeginDirect(c);
        { let v_ = ((((((((9i32) as u64)) | ((((1i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (l_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((14i32) as u64); let a_ = (l_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_1 = n_; };
        { let v_ = ((((((((224i32) as u64)) | ((((8i32) as u64)).wrapping_shl((16i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((16777215i32) as u64)).wrapping_shl((32i32) as u32))); let a_ = (l_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((76i32) as u64); let a_ = (l_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_1 = n_; };
        { let v_ = ((((((224i32) as u64)) | ((((49i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((32i32) as u32))); let a_ = (l_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((78i32) as u64); let a_ = (l_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_1 = n_; };
        { let v_ = ((196608i32) as u64); let a_ = (l_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((71i32) as u64); let a_ = (l_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_1 = n_; };
        { let v_ = ((549755813956i64) as u64); let a_ = (l_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((66i32) as u64); let a_ = (l_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_1 = n_; };
        { let v_ = ((0i32) as u64); let a_ = (l_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((20i32) as u64); let a_ = (l_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_1 = n_; };
        { let v_ = ((0i32) as u64); let a_ = (l_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((63i32) as u64); let a_ = (l_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_1 = n_; };
        { let v_ = (((if ((((!(((((c.m.r32((sym::gGfx).wrapping_add(564)) as i32)) & (1i32))) != 0)) as i32)) != 0) { 61807398400i64 } else { 61807394816i64 })) as u64); let a_ = (l_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((6i32) as u64); let a_ = (l_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_1 = n_; };
        { let v_ = ((342i32) as u64); let a_ = (l_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((0i32) as u64); let a_ = (l_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_1 = n_; };
        { let v_ = ((4575657223564329088i64) as u64); let a_ = (l_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((1i32) as u64); let a_ = (l_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_1 = n_; };
        l_tag_2 = l_p_1;
        { let r_ = 2i32; let n_ = (l_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_1 = n_; };
        {
            l_i_3 = 0i32;
            'l1: loop {
                if !(((((l_i_3) < (l_n_10)) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_u1_4 = (l_u_7).wrapping_add(32i32);
                    F::GfxPostQuad_Set(c, fp, 1792i32, 1824i32, ((l_u_7) as i16), ((0i32) as i16), ((l_u1_4) as i16), ((l_h_9) as i16), l_x_8, 0i32, l_tw_6, l_h_9, ((1i32) as i16));
                    { let v_ = (((((((c.m.r32((fp).wrapping_add(32)) as i32)) as i64)) | (((((c.m.r32((fp).wrapping_add(36)) as i32)) as i64)).wrapping_shl((16i32) as u32)))) as u64); let a_ = (l_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                    { let v_ = (((((((c.m.r32((fp).wrapping_add(16)) as i32)) as i64)) | (((((c.m.r32((fp).wrapping_add(20)) as i32)) as i64)).wrapping_shl((16i32) as u32)))) as u64); let a_ = (l_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                    { let r_ = 2i32; let n_ = (l_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_1 = n_; };
                    { let v_ = (((((((c.m.r32((fp).wrapping_add(40)) as i32)) as i64)) | (((((c.m.r32((fp).wrapping_add(44)) as i32)) as i64)).wrapping_shl((16i32) as u32)))) as u64); let a_ = (l_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                    { let v_ = (((((((c.m.r32((fp).wrapping_add(24)) as i32)) as i64)) | (((((c.m.r32((fp).wrapping_add(28)) as i32)) as i64)).wrapping_shl((16i32) as u32)))) as u64); let a_ = (l_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                    { let r_ = 2i32; let n_ = (l_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_1 = n_; };
                    l_u_7 = l_u1_4;
                    { let r_ = 32i32; let n_ = (l_x_8).wrapping_add(r_); l_x_8 = n_; };
                }
                { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
            }
        }
        { let v_ = ((((((((16i32) as u64)) | ((((1i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((4i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (l_tag_2).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((21331i32) as u64); let a_ = (l_tag_2).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((((((((1i32) as u64)) | ((((1i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (l_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((14i32) as u64); let a_ = (l_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_1 = n_; };
        { let v_ = (((if ((((!(((((c.m.r32((sym::gGfx).wrapping_add(564)) as i32)) & (1i32))) != 0)) as i32)) != 0) { 524400i32 } else { 524288i32 })) as u64); let a_ = (l_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((76i32) as u64); let a_ = (l_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_1 = n_; };
        F::Dma_EndDirect(c, l_p_1);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "GfxPost_CopyAlphaToDepth", &PROBE_N, &[], true); }
    r_
}

pub fn GfxPost_DrawTexRect(c: &mut Ctx, mut p_tex0_1: u64, mut p_x_2: i32, mut p_y_3: i32, mut p_w_4: i32, mut p_h_5: i32, mut p_fullHeight_6: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "GfxPost_DrawTexRect", &PROBE_N, &[(p_tex0_1) as u32, (p_x_2) as u32, (p_y_3) as u32, (p_w_4) as u32, (p_h_5) as u32, (p_fullHeight_6) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(64) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 64);
    let mut l_p_7: u32 = 0;
    let mut l_tw_8: i32 = 0;
    let mut l_th_9: i32 = 0;
    let mut l_f_10: u32 = 0;
    let r_ = 'f: {
        l_p_7 = F::Dma_BeginDirect(c);
        { let v_ = ((((((((8i32) as u64)) | ((((0i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (l_p_7).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((14i32) as u64); let a_ = (l_p_7).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_7).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_7 = n_; };
        { let v_ = ((549755813988i64) as u64); let a_ = (l_p_7).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((66i32) as u64); let a_ = (l_p_7).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_7).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_7 = n_; };
        { let v_ = ((196608i32) as u64); let a_ = (l_p_7).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((71i32) as u64); let a_ = (l_p_7).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_7).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_7 = n_; };
        { let v_ = ((0i32) as u64); let a_ = (l_p_7).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((74i32) as u64); let a_ = (l_p_7).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_7).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_7 = n_; };
        { let v_ = ((342i32) as u64); let a_ = (l_p_7).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((0i32) as u64); let a_ = (l_p_7).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_7).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_7 = n_; };
        { let v_ = ((0i32) as u64); let a_ = (l_p_7).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((20i32) as u64); let a_ = (l_p_7).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_7).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_7 = n_; };
        { let v_ = ((4575657223564329088i64) as u64); let a_ = (l_p_7).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((1i32) as u64); let a_ = (l_p_7).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_7).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_7 = n_; };
        { let v_ = ((0i32) as u64); let a_ = (l_p_7).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((63i32) as u64); let a_ = (l_p_7).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_7).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_7 = n_; };
        { let v_ = p_tex0_1; let a_ = (l_p_7).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((6i32) as u64); let a_ = (l_p_7).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_7).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_7 = n_; };
        l_tw_8 = (1i32).wrapping_shl(((((((p_tex0_1).wrapping_shr((26i32) as u32)) as i32)) & (15i32))) as u32);
        if (((((p_fullHeight_6) != (0i32)) as i32)) != 0) {
            l_th_9 = 448i32;
        } else {
            l_th_9 = (1i32).wrapping_shl(((((((p_tex0_1).wrapping_shr((30i32) as u32)) as i32)) & (15i32))) as u32);
        }
        F::GfxPostQuad_Set(c, fp, 1792i32, 1824i32, ((0i32) as i16), ((0i32) as i16), ((l_tw_8) as i16), ((l_th_9) as i16), p_x_2, p_y_3, p_w_4, p_h_5, ((0i32) as i16));
        { let v_ = ((((((((2i32) as u64)) | ((((1i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((2i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (l_p_7).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((83i32) as u64); let a_ = (l_p_7).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_7).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_7 = n_; };
        { let v_ = (((((((c.m.r32((fp).wrapping_add(32)) as i32)) as i64)) | (((((c.m.r32((fp).wrapping_add(36)) as i32)) as i64)).wrapping_shl((16i32) as u32)))) as u64); let a_ = (l_p_7).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = (((((((c.m.r32((fp).wrapping_add(16)) as i32)) as i64)) | (((((c.m.r32((fp).wrapping_add(20)) as i32)) as i64)).wrapping_shl((16i32) as u32)))) as u64); let a_ = (l_p_7).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_7).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_7 = n_; };
        { let v_ = (((((((c.m.r32((fp).wrapping_add(40)) as i32)) as i64)) | (((((c.m.r32((fp).wrapping_add(44)) as i32)) as i64)).wrapping_shl((16i32) as u32)))) as u64); let a_ = (l_p_7).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = (((((((c.m.r32((fp).wrapping_add(24)) as i32)) as i64)) | (((((c.m.r32((fp).wrapping_add(28)) as i32)) as i64)).wrapping_shl((16i32) as u32)))) as u64); let a_ = (l_p_7).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_7).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_7 = n_; };
        { let v_ = ((((((((1i32) as u64)) | ((((1i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (l_p_7).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((14i32) as u64); let a_ = (l_p_7).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_7).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_7 = n_; };
        { let v_ = ((327680i32) as u64); let a_ = (l_p_7).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((71i32) as u64); let a_ = (l_p_7).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_7).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_7 = n_; };
        F::Dma_EndDirect(c, l_p_7);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "GfxPost_DrawTexRect", &PROBE_N, &[], true); }
    r_
}

pub fn GfxPost_DrawTintRect(c: &mut Ctx, mut p_alpha_1: u64, mut p_color_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "GfxPost_DrawTintRect", &PROBE_N, &[(p_alpha_1) as u32, (p_color_2) as u32], false); }
    let mut l_p_3: u32 = 0;
    let mut l_tag_4: u32 = 0;
    let mut l_i_5: i32 = 0;
    let mut l_x_6: i32 = 0;
    let r_ = 'f: {
    l_x_6 = 0i32;
    F::Gfx_AddDefaultEnv(c);
    F::Dma_AddZbuf(c, 224i32, 1i32);
    l_p_3 = F::Dma_BeginDirect(c);
    { let v_ = ((((((((5i32) as u64)) | ((((0i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (l_p_3).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((14i32) as u64); let a_ = (l_p_3).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_3).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_3 = n_; };
    { let v_ = p_alpha_1; let a_ = (l_p_3).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((66i32) as u64); let a_ = (l_p_3).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_3).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_3 = n_; };
    { let v_ = ((196608i32) as u64); let a_ = (l_p_3).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((71i32) as u64); let a_ = (l_p_3).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_3).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_3 = n_; };
    { let v_ = ((0i32) as u64); let a_ = (l_p_3).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((74i32) as u64); let a_ = (l_p_3).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_3).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_3 = n_; };
    { let v_ = ((70i32) as u64); let a_ = (l_p_3).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((0i32) as u64); let a_ = (l_p_3).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_3).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_3 = n_; };
    { let v_ = ((((((((((((rt::f2u(F32(c.m.r32(p_color_2)))) as u8)) as u64)) | ((((((rt::f2u(F32(c.m.r32((p_color_2).wrapping_add(4))))) as u8)) as u64)).wrapping_shl((8i32) as u32)))) | ((((((rt::f2u(F32(c.m.r32((p_color_2).wrapping_add(8))))) as u8)) as u64)).wrapping_shl((16i32) as u32)))) | ((((((rt::f2u(F32(c.m.r32((p_color_2).wrapping_add(12))))) as u8)) as u64)).wrapping_shl((24i32) as u32)))) | (((4575657221408423936i64) as u64))); let a_ = (l_p_3).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((1i32) as u64); let a_ = (l_p_3).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_3).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_3 = n_; };
    l_tag_4 = l_p_3;
    { let r_ = 2i32; let n_ = (l_p_3).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_3 = n_; };
    {
        l_i_5 = 0i32;
        'l1: loop {
            if !(((((l_i_5) < (16i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let v_ = ((((((((l_x_6).wrapping_shl((4i32) as u32)).wrapping_add(28672i32)) as i64)) | (((1912602624i32) as i64)))) as u64); let a_ = (l_p_3).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                { let v_ = (((((((((l_x_6).wrapping_add(32i32)).wrapping_shl((4i32) as u32)).wrapping_add(28672i32)) as i64)) | (((2382364672u32) as i64)))) as u64); let a_ = (l_p_3).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                { let r_ = 2i32; let n_ = (l_p_3).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_3 = n_; };
                { let r_ = 32i32; let n_ = (l_x_6).wrapping_add(r_); l_x_6 = n_; };
            }
            { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
        }
    }
    { let v_ = ((((((((16i32) as u64)) | ((((1i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((2i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (l_tag_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((85i32) as u64); let a_ = (l_tag_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    F::Dma_EndDirect(c, l_p_3);
    F::Dma_AddZbuf(c, 224i32, 0i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "GfxPost_DrawTintRect", &PROBE_N, &[], true); }
    r_
}

pub fn GfxLens_Init(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "GfxLens_Init", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let _ = F::memset(c, sym::gGfxLens, 0i32, 400u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "GfxLens_Init", &PROBE_N, &[], true); }
    r_
}

pub fn GfxLens_Stub107758(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "GfxLens_Stub107758", &PROBE_N, &[], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "GfxLens_Stub107758", &PROBE_N, &[], true); }
    r_
}

pub fn GfxLens_Clear(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "GfxLens_Clear", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let _ = F::memset(c, sym::gGfxLens, 0i32, 384u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "GfxLens_Clear", &PROBE_N, &[], true); }
    r_
}

pub fn GfxLens_Start(c: &mut Ctx, mut p_pos_1: u32, mut p_life_2: i32, mut p_rgba_3: u32, mut p_blend_4: i32, mut p_size_5: F32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "GfxLens_Start", &PROBE_N, &[(p_pos_1) as u32, (p_life_2) as u32, (p_rgba_3) as u32, (p_blend_4) as u32, (p_size_5).0], false); }
    let mut l_i_6: i32 = 0;
    let mut l_f_7: u32 = 0;
    let r_ = 'f: {
    {
        { l_i_6 = 0i32; l_f_7 = sym::gGfxLens; };
        'l1: loop {
            if !(((((l_i_6) < (8i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((((((c.m.r32((l_f_7).wrapping_add(16)) as i32)) == (0i32)) as i32)) != 0) {
                    { let v_ = p_life_2; let a_ = (l_f_7).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
                    { let v_ = p_life_2; let a_ = (l_f_7).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
                    { let v_ = p_rgba_3; let a_ = (l_f_7).wrapping_add(24); c.m.w32(a_, v_); };
                    { let v_ = p_blend_4; let a_ = (l_f_7).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
                    { let v_ = p_size_5; let a_ = (l_f_7).wrapping_add(28); c.m.w32(a_, (v_).0); };
                    F::Vec4_Copy(c, l_f_7, p_pos_1);
                    break 'f l_i_6;
                }
            }
            { { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; }; { let o_ = l_f_7; let n_ = (o_).wrapping_add(48); l_f_7 = n_; }; };
        }
    }
    break 'f (1i32).wrapping_neg();
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "GfxLens_Start", &PROBE_N, &[], true); }
    r_
}

pub fn GfxLens_Stop(c: &mut Ctx, mut p_slot_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "GfxLens_Stop", &PROBE_N, &[(p_slot_1) as u32], false); }
    let r_ = 'f: {
    if (((((p_slot_1) < (((8i32) as u32))) as i32)) != 0) {
        { let v_ = 0i32; let a_ = ((sym::gGfxLens).wrapping_add((p_slot_1).wrapping_mul(48))).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "GfxLens_Stop", &PROBE_N, &[], true); }
    r_
}

pub fn GfxLens_SetHold(c: &mut Ctx, mut p_slot_1: u32, mut p_hold_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "GfxLens_SetHold", &PROBE_N, &[(p_slot_1) as u32, (p_hold_2) as u32], false); }
    let r_ = 'f: {
    if (((((p_slot_1) < (((8i32) as u32))) as i32)) != 0) {
        { let v_ = p_hold_2; let a_ = ((sym::gGfxLens).wrapping_add((p_slot_1).wrapping_mul(48))).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "GfxLens_SetHold", &PROBE_N, &[], true); }
    r_
}

pub fn GfxLens_SetPos(c: &mut Ctx, mut p_slot_1: u32, mut p_pos_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "GfxLens_SetPos", &PROBE_N, &[(p_slot_1) as u32, (p_pos_2) as u32], false); }
    let r_ = 'f: {
    if (((((p_slot_1) < (((8i32) as u32))) as i32)) != 0) {
        F::Vec4_Copy(c, (sym::gGfxLens).wrapping_add((p_slot_1).wrapping_mul(48)), p_pos_2);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "GfxLens_SetPos", &PROBE_N, &[], true); }
    r_
}

pub fn GfxLens_DrawOne(c: &mut Ctx, mut p_pt_1: u32, mut p_split_2: i32, mut p_side_3: i32, mut p_rgba_4: u32, mut p_blend_5: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "GfxLens_DrawOne", &PROBE_N, &[(p_pt_1) as u32, (p_split_2) as u32, (p_side_3) as u32, (p_rgba_4) as u32, (p_blend_5) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_p_6: u32 = 0;
    let mut l_i_7: i32 = 0;
    let r_ = 'f: {
        F::Gfx_AddDefaultEnv(c);
        l_p_6 = F::Dma_BeginDirect(c);
        { let v_ = ((((((((1i32) as u64)) | ((((1i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (l_p_6).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((14i32) as u64); let a_ = (l_p_6).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_6).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_6 = n_; };
        's1: {
            let sv8: i32 = p_blend_5;
            match sv8 {
                1i32 => {
                    { let v_ = ((88i32) as u64); let a_ = (l_p_6).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                    { let v_ = ((66i32) as u64); let a_ = (l_p_6).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                    { let r_ = 2i32; let n_ = (l_p_6).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_6 = n_; };
                    break 's1;
                }
                2i32 => {
                    { let v_ = ((82i32) as u64); let a_ = (l_p_6).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                    { let v_ = ((66i32) as u64); let a_ = (l_p_6).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                    { let r_ = 2i32; let n_ = (l_p_6).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_6 = n_; };
                    break 's1;
                }
                _ => {
                    { let v_ = ((84i32) as u64); let a_ = (l_p_6).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                    { let v_ = ((66i32) as u64); let a_ = (l_p_6).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                    { let r_ = 2i32; let n_ = (l_p_6).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_6 = n_; };
                    break 's1;
                }
            }
        }
        { let v_ = ((((((((1i32) as u64)) | ((((1i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((4i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (l_p_6).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((21776i32) as u64); let a_ = (l_p_6).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_6).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_6 = n_; };
        { let v_ = ((582i32) as u64); let a_ = (l_p_6).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((4575657221408423936i64) as u64); let a_ = (l_p_6).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_6).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_6 = n_; };
        { let v_ = ((1912631296i32) as u64); let a_ = (l_p_6).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((2382401536u32) as u64); let a_ = (l_p_6).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_6).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_6 = n_; };
        { let v_ = ((((((((4i32) as u64)) | ((((1i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((6i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (l_p_6).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((5460752i32) as u64); let a_ = (l_p_6).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_6).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_6 = n_; };
        {
            l_i_7 = 0i32;
            'l2: loop {
                if !(((((l_i_7) < (4i32)) as i32)) != 0) { break 'l2; }
                'c3: {
                    { let v_ = ((854i32) as u64); let a_ = (l_p_6).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                    { let v_ = ((4575657221953716352i64) as u64); let a_ = (l_p_6).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                    { let r_ = 2i32; let n_ = (l_p_6).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_6 = n_; };
                    { let v_ = ((0i32) as u64); let a_ = (l_p_6).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                    { let v_ = (((((((c.m.r32((p_pt_1).wrapping_add((((l_i_7) as u32)).wrapping_mul(16))) as i32)) as i64)) | (((((c.m.r32(((p_pt_1).wrapping_add((((l_i_7) as u32)).wrapping_mul(16))).wrapping_add(4)) as i32)) as i64)).wrapping_shl((16i32) as u32)))) as u64); let a_ = (l_p_6).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                    { let r_ = 2i32; let n_ = (l_p_6).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_6 = n_; };
                    { let v_ = ((267390960i32) as u64); let a_ = (l_p_6).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                    { let v_ = (((((((c.m.r32((p_pt_1).wrapping_add((((4i32) as u32)).wrapping_mul(16))) as i32)) as i64)) | (((((c.m.r32(((p_pt_1).wrapping_add((((4i32) as u32)).wrapping_mul(16))).wrapping_add(4)) as i32)) as i64)).wrapping_shl((16i32) as u32)))) as u64); let a_ = (l_p_6).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                    { let r_ = 2i32; let n_ = (l_p_6).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_6 = n_; };
                }
                { let o_ = l_i_7; let n_ = (o_).wrapping_add(1); l_i_7 = n_; };
            }
        }
        { let v_ = ((((((((1i32) as u64)) | ((((1i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((6i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (l_p_6).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((5394960i32) as u64); let a_ = (l_p_6).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_6).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_6 = n_; };
        { let v_ = ((86i32) as u64); let a_ = (l_p_6).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((((p_rgba_4) as u64)) | (((4575657221408423936i64) as u64))); let a_ = (l_p_6).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_6).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_6 = n_; };
        { let v_ = rt::fmul(rt::fmul(rt::i2f(((c.m.r32((p_pt_1).wrapping_add((((5i32) as u32)).wrapping_mul(16))) as i32)).wrapping_sub(28672i32)), F32(0x3b000000)), F32(0x3d800000)); let a_ = fp; c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(rt::fmul(rt::i2f(((c.m.r32(((p_pt_1).wrapping_add((((5i32) as u32)).wrapping_mul(16))).wrapping_add(4)) as i32)).wrapping_sub(29184i32)), F32(0x3b124924)), F32(0x3d800000)); let a_ = fp.wrapping_add(4); c.m.w32(a_, (v_).0); };
        if (((((p_split_2) != (0i32)) as i32)) != 0) {
            { let r_ = F32(c.m.r32(fp)); let a_ = fp; let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            if (((((p_side_3) != (0i32)) as i32)) != 0) {
                { let r_ = F32(0x3f800000); let a_ = fp; let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            }
        }
        { let v_ = ((((c.m.r32(fp)) as u64)) | ((((c.m.r32(fp.wrapping_add(4))) as u64)).wrapping_shl((32i32) as u32))); let a_ = (l_p_6).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = (((((((c.m.r32((p_pt_1).wrapping_add((((0i32) as u32)).wrapping_mul(16))) as i32)) as i64)) | (((((c.m.r32(((p_pt_1).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4)) as i32)) as i64)).wrapping_shl((16i32) as u32)))) as u64); let a_ = (l_p_6).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_6).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_6 = n_; };
        { let v_ = rt::fmul(rt::fmul(rt::i2f(((c.m.r32((p_pt_1).wrapping_add((((6i32) as u32)).wrapping_mul(16))) as i32)).wrapping_sub(28672i32)), F32(0x3b000000)), F32(0x3d800000)); let a_ = fp; c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(rt::fmul(rt::i2f(((c.m.r32(((p_pt_1).wrapping_add((((6i32) as u32)).wrapping_mul(16))).wrapping_add(4)) as i32)).wrapping_sub(29184i32)), F32(0x3b124924)), F32(0x3d800000)); let a_ = fp.wrapping_add(4); c.m.w32(a_, (v_).0); };
        if (((((p_split_2) != (0i32)) as i32)) != 0) {
            { let r_ = F32(c.m.r32(fp)); let a_ = fp; let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            if (((((p_side_3) != (0i32)) as i32)) != 0) {
                { let r_ = F32(0x3f800000); let a_ = fp; let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            }
        }
        { let v_ = ((((c.m.r32(fp)) as u64)) | ((((c.m.r32(fp.wrapping_add(4))) as u64)).wrapping_shl((32i32) as u32))); let a_ = (l_p_6).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = (((((((c.m.r32((p_pt_1).wrapping_add((((3i32) as u32)).wrapping_mul(16))) as i32)) as i64)) | (((((c.m.r32(((p_pt_1).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(4)) as i32)) as i64)).wrapping_shl((16i32) as u32)))) as u64); let a_ = (l_p_6).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_6).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_6 = n_; };
        F::Dma_EndDirect(c, l_p_6);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "GfxLens_DrawOne", &PROBE_N, &[], true); }
    r_
}

pub fn gfxm_b__GfxLens_IsOnScreen(c: &mut Ctx, mut p_scr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "gfxm_b__GfxLens_IsOnScreen", &PROBE_N, &[(p_scr_1) as u32], false); }
    let mut l_flags_2: i32 = 0;
    let r_ = 'f: {
    if ((((((c.m.r32(((p_scr_1).wrapping_add((((4i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32)) <= (0i32)) as i32)) != 0) {
        break 'f 0i32;
    }
    l_flags_2 = ((((((((((c.m.r32(((p_scr_1).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) >= (28672i32)) as i32)) != 0) && ((((((c.m.r32(((p_scr_1).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) <= (36864i32)) as i32)) != 0)) as i32)) | (((((((((c.m.r32(((p_scr_1).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) >= (28672i32)) as i32)) != 0) && ((((((c.m.r32(((p_scr_1).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) <= (36864i32)) as i32)) != 0)) as i32)));
    if ((((((((((c.m.r32(((p_scr_1).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) <= (28672i32)) as i32)) != 0) && ((((((c.m.r32(((p_scr_1).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) >= (36864i32)) as i32)) != 0)) as i32)) != 0) {
        { let r_ = 1i32; let n_ = ((l_flags_2) | (r_)); l_flags_2 = n_; };
    }
    if ((((((c.m.r32(((p_scr_1).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) <= (28672i32)) as i32)) != 0) {
        { let r_ = ((((c.m.r32(((p_scr_1).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) >= (36864i32)) as i32); let n_ = ((l_flags_2) | (r_)); l_flags_2 = n_; };
    }
    if ((((((((((c.m.r32(((p_scr_1).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) >= (29184i32)) as i32)) != 0) && ((((((c.m.r32(((p_scr_1).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) <= (36352i32)) as i32)) != 0)) as i32)) != 0) {
        { let r_ = 2i32; let n_ = ((l_flags_2) | (r_)); l_flags_2 = n_; };
    }
    if ((((((((((c.m.r32(((p_scr_1).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) >= (29184i32)) as i32)) != 0) && ((((((c.m.r32(((p_scr_1).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) <= (36352i32)) as i32)) != 0)) as i32)) != 0) {
        { let r_ = 2i32; let n_ = ((l_flags_2) | (r_)); l_flags_2 = n_; };
    }
    if ((((((((((c.m.r32(((p_scr_1).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) <= (29184i32)) as i32)) != 0) && ((((((c.m.r32(((p_scr_1).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) >= (36352i32)) as i32)) != 0)) as i32)) != 0) {
        { let r_ = 2i32; let n_ = ((l_flags_2) | (r_)); l_flags_2 = n_; };
    }
    if ((((((((((c.m.r32(((p_scr_1).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) <= (29184i32)) as i32)) != 0) && ((((((c.m.r32(((p_scr_1).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) >= (36352i32)) as i32)) != 0)) as i32)) != 0) {
        { let r_ = 2i32; let n_ = ((l_flags_2) | (r_)); l_flags_2 = n_; };
    }
    break 'f (((l_flags_2) == (3i32)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "gfxm_b__GfxLens_IsOnScreen", &PROBE_N, &[], true); }
    r_
}

pub fn GfxLens_DrawAll(c: &mut Ctx, mut p_view_1: u32, mut p_split_2: i32, mut p_side_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "GfxLens_DrawAll", &PROBE_N, &[(p_view_1) as u32, (p_split_2) as u32, (p_side_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(480) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 480);
    let mut l_i_4: i32 = 0;
    let mut l_j_5: i32 = 0;
    let mut l_f_6: u32 = 0;
    let mut l_t_7: F32 = F32(0);
    let mut l_half_8: F32 = F32(0);
    let mut l_v_9: u32 = 0;
    let mut l_o_10: i32 = 0;
    let mut l_w_11: u32 = 0;
    let r_ = 'f: {
        rt::memset(c, fp.wrapping_add(64), 0, 112);
        rt::memset(c, fp.wrapping_add(64).wrapping_add(0), 0, 16);
        { let v_ = F32(0xbf800000); c.m.w32(fp.wrapping_add(64).wrapping_add(0).wrapping_add(0), (v_).0); }
        { let v_ = F32(0xbf800000); c.m.w32(fp.wrapping_add(64).wrapping_add(0).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(64).wrapping_add(0).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(64).wrapping_add(0).wrapping_add(12), (v_).0); }
        rt::memset(c, fp.wrapping_add(64).wrapping_add(16), 0, 16);
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(64).wrapping_add(16).wrapping_add(0), (v_).0); }
        { let v_ = F32(0xbf800000); c.m.w32(fp.wrapping_add(64).wrapping_add(16).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(64).wrapping_add(16).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(64).wrapping_add(16).wrapping_add(12), (v_).0); }
        rt::memset(c, fp.wrapping_add(64).wrapping_add(32), 0, 16);
        { let v_ = F32(0xbf800000); c.m.w32(fp.wrapping_add(64).wrapping_add(32).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(64).wrapping_add(32).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(64).wrapping_add(32).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(64).wrapping_add(32).wrapping_add(12), (v_).0); }
        rt::memset(c, fp.wrapping_add(64).wrapping_add(48), 0, 16);
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(64).wrapping_add(48).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(64).wrapping_add(48).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(64).wrapping_add(48).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(64).wrapping_add(48).wrapping_add(12), (v_).0); }
        rt::memset(c, fp.wrapping_add(64).wrapping_add(64), 0, 16);
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(64).wrapping_add(64).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(64).wrapping_add(64).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(64).wrapping_add(64).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(64).wrapping_add(64).wrapping_add(12), (v_).0); }
        rt::memset(c, fp.wrapping_add(64).wrapping_add(80), 0, 16);
        { let v_ = F32(0xbf800000); c.m.w32(fp.wrapping_add(64).wrapping_add(80).wrapping_add(0), (v_).0); }
        { let v_ = F32(0xbf800000); c.m.w32(fp.wrapping_add(64).wrapping_add(80).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(64).wrapping_add(80).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(64).wrapping_add(80).wrapping_add(12), (v_).0); }
        rt::memset(c, fp.wrapping_add(64).wrapping_add(96), 0, 16);
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(64).wrapping_add(96).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(64).wrapping_add(96).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(64).wrapping_add(96).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(64).wrapping_add(96).wrapping_add(12), (v_).0); }
        F::Vu0Screen_StoreMtx(c, fp.wrapping_add(416));
        F::Mtx_Transpose(c, fp, (p_view_1).wrapping_add(64));
        {
            l_j_5 = 0i32;
            'l1: loop {
                if !(((((l_j_5) < (8i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_f_6 = (sym::gGfxLens).wrapping_add((((l_j_5) as u32)).wrapping_mul(48));
                    if ((((((c.m.r32((l_f_6).wrapping_add(16)) as i32)) == (0i32)) as i32)) != 0) {
                        break 'c2;
                    }
                    if ((((((c.m.r32((l_f_6).wrapping_add(36)) as i32)) == (0i32)) as i32)) != 0) {
                        { let a_ = (l_f_6).wrapping_add(16); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                    }
                    l_t_7 = rt::fmul(rt::fdiv(rt::i2f((c.m.r32((l_f_6).wrapping_add(16)) as i32)), rt::i2f((c.m.r32((l_f_6).wrapping_add(20)) as i32))), F32(c.m.r32((l_f_6).wrapping_add(28))));
                    l_half_8 = rt::fmul(F32(c.m.r32((l_f_6).wrapping_add(28))), F32(0x3f000000));
                    {
                        { { { l_i_4 = 0i32; l_w_11 = ((fp.wrapping_add(176)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((3i32) as u32)).wrapping_mul(4)); }; l_v_9 = fp.wrapping_add(176); }; l_o_10 = 0i32; };
                        'l3: loop {
                            if !(((((l_i_4) < (7i32)) as i32)) != 0) { break 'l3; }
                            'c4: {
                                if (((((l_i_4) < (5i32)) as i32)) != 0) {
                                    F::Vec4_Scale(c, l_v_9, (fp.wrapping_add(64)).wrapping_add(((l_o_10) as u32)), l_t_7);
                                } else {
                                    F::Vec4_Scale(c, l_v_9, (fp.wrapping_add(64)).wrapping_add(((l_o_10) as u32)), rt::fadd(l_half_8, rt::fmul(l_t_7, F32(0x3f000000))));
                                }
                                F::Mtx_MulVec4(c, l_v_9, fp, l_v_9);
                                F::Vec4_Add(c, l_v_9, l_f_6, l_v_9);
                                { let v_ = F32(0x3f800000); let a_ = (l_w_11).wrapping_add(((l_o_10) as u32)); c.m.w32(a_, (v_).0); };
                                { let o_ = l_v_9; let n_ = (o_).wrapping_add(16); l_v_9 = n_; };
                            }
                            { { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; }; { let r_ = 16i32; let n_ = (l_o_10).wrapping_add(r_); l_o_10 = n_; }; };
                        }
                    }
                    {
                        l_i_4 = 0i32;
                        'l5: loop {
                            if !(((((l_i_4) < (7i32)) as i32)) != 0) { break 'l5; }
                            'c6: {
                                F::Mtx_MulVec4(c, fp.wrapping_add(400), fp.wrapping_add(416), (fp.wrapping_add(176)).wrapping_add((((l_i_4) as u32)).wrapping_mul(16)));
                                { let a2_ = F32(c.m.r32((fp.wrapping_add(400)).wrapping_add((((3i32) as u32)).wrapping_mul(4)))); F::Vec4_Div(c, fp.wrapping_add(400), fp.wrapping_add(400), a2_) };
                                F::Vec4_ToFixed4(c, (fp.wrapping_add(288)).wrapping_add((((l_i_4) as u32)).wrapping_mul(16)), fp.wrapping_add(400));
                            }
                            { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
                        }
                    }
                    if ((F::gfxm_b__GfxLens_IsOnScreen(c, fp.wrapping_add(288))) != 0) {
                        { let a3_ = c.m.r32((l_f_6).wrapping_add(24)); let a4_ = (c.m.r32((l_f_6).wrapping_add(32)) as i32); F::GfxLens_DrawOne(c, fp.wrapping_add(288), p_split_2, p_side_3, a3_, a4_) };
                    }
                }
                { let o_ = l_j_5; let n_ = (o_).wrapping_add(1); l_j_5 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "GfxLens_DrawAll", &PROBE_N, &[], true); }
    r_
}

pub fn GfxLens_PutCapture(c: &mut Ctx, mut p_p_1: u32, mut p_split_2: i32, mut p_side_3: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "GfxLens_PutCapture", &PROBE_N, &[(p_p_1) as u32, (p_split_2) as u32, (p_side_3) as u32], false); }
    let mut l_i_4: i32 = 0;
    let mut l_u_5: i32 = 0;
    let mut l_sh_6: i32 = 0;
    let mut l_x_7: i32 = 0;
    let mut l_col_8: i32 = 0;
    let r_ = 'f: {
    { let v_ = ((((((((13i32) as u64)) | ((((1i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (p_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((14i32) as u64); let a_ = (p_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (p_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); p_p_1 = n_; };
    { let v_ = ((1i32) as u64); let a_ = (p_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((70i32) as u64); let a_ = (p_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (p_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); p_p_1 = n_; };
    { let v_ = ((0i32) as u64); let a_ = (p_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((74i32) as u64); let a_ = (p_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (p_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); p_p_1 = n_; };
    { let v_ = ((0i32) as u64); let a_ = (p_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((75i32) as u64); let a_ = (p_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (p_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); p_p_1 = n_; };
    { let v_ = ((((((((336i32) as u64)) | ((((4i32) as u64)).wrapping_shl((16i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((32i32) as u32))); let a_ = (p_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((76i32) as u64); let a_ = (p_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (p_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); p_p_1 = n_; };
    { let v_ = ((((((224i32) as u64)) | ((((49i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((32i32) as u32))); let a_ = (p_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((78i32) as u64); let a_ = (p_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (p_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); p_p_1 = n_; };
    { let v_ = ((((30720i32) as u64)) | ((((30720i32) as u64)).wrapping_shl((32i32) as u32))); let a_ = (p_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((24i32) as u64); let a_ = (p_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (p_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); p_p_1 = n_; };
    { let v_ = ((((((((0i32) as u64)) | ((((255i32) as u64)).wrapping_shl((16i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((32i32) as u32)))) | ((((255i32) as u64)).wrapping_shl((48i32) as u32))); let a_ = (p_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((64i32) as u64); let a_ = (p_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (p_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); p_p_1 = n_; };
    { let v_ = ((7679409897482i64) as u64); let a_ = (p_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((8i32) as u64); let a_ = (p_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (p_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); p_p_1 = n_; };
    { let v_ = ((549755813988i64) as u64); let a_ = (p_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((66i32) as u64); let a_ = (p_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (p_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); p_p_1 = n_; };
    { let v_ = ((96i32) as u64); let a_ = (p_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((20i32) as u64); let a_ = (p_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (p_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); p_p_1 = n_; };
    { let v_ = ((131072i32) as u64); let a_ = (p_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((71i32) as u64); let a_ = (p_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (p_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); p_p_1 = n_; };
    { let v_ = ((70i32) as u64); let a_ = (p_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((0i32) as u64); let a_ = (p_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (p_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); p_p_1 = n_; };
    { let v_ = ((4575657223555907584i64) as u64); let a_ = (p_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((1i32) as u64); let a_ = (p_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (p_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); p_p_1 = n_; };
    { let v_ = ((((((((6i32) as u64)) | ((((1i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (p_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((14i32) as u64); let a_ = (p_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (p_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); p_p_1 = n_; };
    { let v_ = ((0i32) as u64); let a_ = (p_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((63i32) as u64); let a_ = (p_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (p_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); p_p_1 = n_; };
    { let v_ = (((if ((((!(((((c.m.r32((sym::gGfx).wrapping_add(564)) as i32)) & (1i32))) != 0)) as i32)) != 0) { 2305843036661353984i64 } else { 2305843036661350400i64 })) as u64); let a_ = (p_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((6i32) as u64); let a_ = (p_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (p_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); p_p_1 = n_; };
    { let v_ = ((68i32) as u64); let a_ = (p_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((66i32) as u64); let a_ = (p_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (p_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); p_p_1 = n_; };
    { let v_ = ((0i32) as u64); let a_ = (p_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((71i32) as u64); let a_ = (p_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (p_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); p_p_1 = n_; };
    { let v_ = ((278i32) as u64); let a_ = (p_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((0i32) as u64); let a_ = (p_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (p_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); p_p_1 = n_; };
    { let v_ = ((4575657223564329088i64) as u64); let a_ = (p_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((1i32) as u64); let a_ = (p_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (p_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); p_p_1 = n_; };
    l_col_8 = 0i32;
    l_sh_6 = 4i32;
    if (((((p_split_2) != (0i32)) as i32)) != 0) {
        l_col_8 = (if (((((p_side_3) == (1i32)) as i32)) != 0) { 16i32 } else { 0i32 });
        l_sh_6 = 3i32;
    }
    l_x_7 = (l_col_8).wrapping_shl((5i32) as u32);
    { let v_ = ((((((((16i32) as u64)) | ((((1i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((4i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (p_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((21331i32) as u64); let a_ = (p_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (p_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); p_p_1 = n_; };
    {
        l_i_4 = 0i32;
        'l1: loop {
            if !(((((l_i_4) < (16i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if (((((l_i_4) == (0i32)) as i32)) != 0) {
                    if (((((p_side_3) == (1i32)) as i32)) != 0) {
                        { let v_ = ((((l_x_7).wrapping_add(3i32)).wrapping_shl((l_sh_6) as u32)) as u64); let a_ = (p_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                        { let v_ = ((((((120i32).wrapping_add(l_i_4)).wrapping_shl((8i32) as u32)) | (2013265920i32))) as u64); let a_ = (p_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                        { let r_ = 2i32; let n_ = (p_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); p_p_1 = n_; };
                    } else {
                        { let v_ = (((l_x_7).wrapping_shl((l_sh_6) as u32)) as u64); let a_ = (p_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                        { let v_ = ((((((120i32).wrapping_add(l_i_4)).wrapping_shl((8i32) as u32)) | (2013265920i32))) as u64); let a_ = (p_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                        { let r_ = 2i32; let n_ = (p_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); p_p_1 = n_; };
                    }
                    { let v_ = (((((((l_x_7).wrapping_add((l_i_4).wrapping_mul(32i32))).wrapping_add(32i32)).wrapping_shl((l_sh_6) as u32)) | (469762048i32))) as u64); let a_ = (p_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                    { let v_ = ((((((121i32).wrapping_add(l_i_4)).wrapping_shl((8i32) as u32)) as u64)) | (((2281701376u32) as u64))); let a_ = (p_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                    { let r_ = 2i32; let n_ = (p_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); p_p_1 = n_; };
                } else {
                    l_u_5 = (l_x_7).wrapping_add((l_i_4).wrapping_mul(32i32));
                    if (((((l_i_4) == (15i32)) as i32)) != 0) {
                        { let v_ = (((l_u_5).wrapping_shl((l_sh_6) as u32)) as u64); let a_ = (p_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                        { let v_ = ((((((120i32).wrapping_add(l_i_4)).wrapping_shl((8i32) as u32)) | (2013265920i32))) as u64); let a_ = (p_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                        { let r_ = 2i32; let n_ = (p_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); p_p_1 = n_; };
                        { let v_ = ((((((l_u_5).wrapping_add(31i32)).wrapping_shl((l_sh_6) as u32)) | (469762048i32))) as u64); let a_ = (p_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                        { let v_ = ((((((121i32).wrapping_add(l_i_4)).wrapping_shl((8i32) as u32)) as u64)) | (((2281701376u32) as u64))); let a_ = (p_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                    } else {
                        { let v_ = (((l_u_5).wrapping_shl((l_sh_6) as u32)) as u64); let a_ = (p_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                        { let v_ = ((((((120i32).wrapping_add(l_i_4)).wrapping_shl((8i32) as u32)) | (2013265920i32))) as u64); let a_ = (p_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                        { let r_ = 2i32; let n_ = (p_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); p_p_1 = n_; };
                        { let v_ = ((((((l_u_5).wrapping_add(32i32)).wrapping_shl((l_sh_6) as u32)) | (469762048i32))) as u64); let a_ = (p_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                        { let v_ = ((((((121i32).wrapping_add(l_i_4)).wrapping_shl((8i32) as u32)) as u64)) | (((2281701376u32) as u64))); let a_ = (p_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                    }
                    { let r_ = 2i32; let n_ = (p_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); p_p_1 = n_; };
                }
            }
            { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
        }
    }
    { let v_ = ((((((((6i32) as u64)) | ((((1i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (p_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((14i32) as u64); let a_ = (p_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (p_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); p_p_1 = n_; };
    { let v_ = (((if ((((!(((((c.m.r32((sym::gGfx).wrapping_add(564)) as i32)) & (1i32))) != 0)) as i32)) != 0) { 524400i32 } else { 524288i32 })) as u64); let a_ = (p_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((76i32) as u64); let a_ = (p_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (p_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); p_p_1 = n_; };
    { let v_ = ((((28672i32) as u64)) | ((((29184i32) as u64)).wrapping_shl((32i32) as u32))); let a_ = (p_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((24i32) as u64); let a_ = (p_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (p_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); p_p_1 = n_; };
    if (((((p_split_2) != (0i32)) as i32)) != 0) {
        if (((((p_side_3) == (0i32)) as i32)) != 0) {
            { let v_ = ((((((((0i32) as u64)) | ((((256i32) as u64)).wrapping_shl((16i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((32i32) as u32)))) | ((((447i32) as u64)).wrapping_shl((48i32) as u32))); let a_ = (p_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let v_ = ((64i32) as u64); let a_ = (p_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let r_ = 2i32; let n_ = (p_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); p_p_1 = n_; };
        } else {
            { let v_ = ((((((((256i32) as u64)) | ((((511i32) as u64)).wrapping_shl((16i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((32i32) as u32)))) | ((((447i32) as u64)).wrapping_shl((48i32) as u32))); let a_ = (p_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let v_ = ((64i32) as u64); let a_ = (p_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let r_ = 2i32; let n_ = (p_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); p_p_1 = n_; };
        }
    } else {
        { let v_ = ((((((((0i32) as u64)) | ((((511i32) as u64)).wrapping_shl((16i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((32i32) as u32)))) | ((((447i32) as u64)).wrapping_shl((48i32) as u32))); let a_ = (p_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((64i32) as u64); let a_ = (p_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (p_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); p_p_1 = n_; };
    }
    { let v_ = ((458752i32) as u64); let a_ = (p_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((71i32) as u64); let a_ = (p_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (p_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); p_p_1 = n_; };
    { let v_ = ((((((224i32) as u64)) | ((((49i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((32i32) as u32))); let a_ = (p_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((78i32) as u64); let a_ = (p_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (p_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); p_p_1 = n_; };
    { let v_ = ((0i32) as u64); let a_ = (p_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((8i32) as u64); let a_ = (p_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (p_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); p_p_1 = n_; };
    break 'f p_p_1;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "GfxLens_PutCapture", &PROBE_N, &[], true); }
    r_
}

pub fn GfxLens_Draw(c: &mut Ctx, mut p_split_1: i32, mut p_side_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "GfxLens_Draw", &PROBE_N, &[(p_split_1) as u32, (p_side_2) as u32], false); }
    let mut l_p_3: u32 = 0;
    let mut l_i_4: i32 = 0;
    let mut l_ent_5: u32 = 0;
    let mut l_tw_6: i32 = 0;
    let mut l_th_7: i32 = 0;
    let mut l_f_8: u32 = 0;
    let r_ = 'f: {
    l_f_8 = sym::gGfxLens;
    {
        l_i_4 = 0i32;
        'l1: loop {
            if !(((((l_i_4) < (8i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((((((c.m.r32(((l_f_8).wrapping_add((((l_i_4) as u32)).wrapping_mul(48))).wrapping_add(16)) as i32)) != (0i32)) as i32)) != 0) {
                    break 'l1;
                }
            }
            { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
        }
    }
    if (((((l_i_4) == (8i32)) as i32)) != 0) {
        break 'f;
    }
    { let a0_ = c.m.r32((sym::gGfxLens).wrapping_add(384)); F::TexFile_UploadPacked(c, a0_, 12032i32, 11776i32) };
    F::Dma_AddTexFlush(c);
    l_ent_5 = { let a_ = (c.m.r32((sym::gGfxLens).wrapping_add(384))).wrapping_add(16); c.m.r32(a_) };
    F::Gfx_AddDefaultEnv(c);
    l_p_3 = F::Dma_BeginDirect(c);
    l_p_3 = F::GfxLens_PutCapture(c, l_p_3, p_split_1, p_side_2);
    { let v_ = ((((((((10i32) as u64)) | ((((1i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (l_p_3).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((14i32) as u64); let a_ = (l_p_3).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_3).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_3 = n_; };
    { let v_ = ((84i32) as u64); let a_ = (l_p_3).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((66i32) as u64); let a_ = (l_p_3).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_3).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_3 = n_; };
    { let v_ = ((74i32) as u64); let a_ = (l_p_3).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((67i32) as u64); let a_ = (l_p_3).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_3).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_3 = n_; };
    { let v_ = ((0i32) as u64); let a_ = (l_p_3).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((74i32) as u64); let a_ = (l_p_3).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_3).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_3 = n_; };
    { let v_ = ((0i32) as u64); let a_ = (l_p_3).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((75i32) as u64); let a_ = (l_p_3).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_3).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_3 = n_; };
    { let v_ = ((131084i32) as u64); let a_ = (l_p_3).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((71i32) as u64); let a_ = (l_p_3).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_3).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_3 = n_; };
    { let v_ = ((131084i32) as u64); let a_ = (l_p_3).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((72i32) as u64); let a_ = (l_p_3).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_3).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_3 = n_; };
    { let v_ = ((5i32) as u64); let a_ = (l_p_3).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((8i32) as u64); let a_ = (l_p_3).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_3).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_3 = n_; };
    { let v_ = ((5i32) as u64); let a_ = (l_p_3).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((9i32) as u64); let a_ = (l_p_3).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_3).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_3 = n_; };
    l_tw_6 = F::Tex_Log2Size(c, 256i32);
    l_th_7 = F::Tex_Log2Size(c, 256i32);
    { let v_ = (((((((((l_tw_6) as i64)).wrapping_shl((26i32) as u32)) | ((((l_th_7) as i64)).wrapping_shl((30i32) as u32)))) | (((76288i32) as i64)))) as u64); let a_ = (l_p_3).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((6i32) as u64); let a_ = (l_p_3).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_3).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_3 = n_; };
    { let v_ = ((c.m.r64((l_ent_5).wrapping_add(48))) | (((1618498295967488i64) as u64))); let a_ = (l_p_3).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = (((6i32).wrapping_add(1i32)) as u64); let a_ = (l_p_3).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_3).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_3 = n_; };
    F::Dma_EndDirect(c, l_p_3);
    { let a0_ = { let a_ = (c.m.r32(sym::gBtlCam)).wrapping_add(3136); c.m.r32(a_) }; F::GfxLens_DrawAll(c, a0_, p_split_1, p_side_2) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "GfxLens_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn GfxLens_DrawFull(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "GfxLens_DrawFull", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::GfxLens_Draw(c, 0i32, 0i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "GfxLens_DrawFull", &PROBE_N, &[], true); }
    r_
}

pub fn GfxAlphaKey_BuildClut(c: &mut Ctx, mut p_clut_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "GfxAlphaKey_BuildClut", &PROBE_N, &[(p_clut_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let mut l_i_2: i32 = 0;
    let mut l_n_3: i32 = 0;
    let r_ = 'f: {
        rt::memset(c, fp, 0, 33);
        { let v_ = ((63i32) as u8); c.m.w8(fp.wrapping_add(0), v_); }
        { let v_ = ((63i32) as u8); c.m.w8(fp.wrapping_add(1), v_); }
        { let v_ = ((255i32) as u8); c.m.w8(fp.wrapping_add(2), v_); }
        { let v_ = ((128i32) as u8); c.m.w8(fp.wrapping_add(3), v_); }
        { let v_ = ((0i32) as u8); c.m.w8(fp.wrapping_add(4), v_); }
        { let v_ = ((255i32) as u8); c.m.w8(fp.wrapping_add(5), v_); }
        { let v_ = ((255i32) as u8); c.m.w8(fp.wrapping_add(6), v_); }
        { let v_ = ((255i32) as u8); c.m.w8(fp.wrapping_add(7), v_); }
        { let v_ = ((32i32) as u8); c.m.w8(fp.wrapping_add(8), v_); }
        { let v_ = ((255i32) as u8); c.m.w8(fp.wrapping_add(9), v_); }
        { let v_ = ((0i32) as u8); c.m.w8(fp.wrapping_add(10), v_); }
        { let v_ = ((0i32) as u8); c.m.w8(fp.wrapping_add(11), v_); }
        { let v_ = ((0i32) as u8); c.m.w8(fp.wrapping_add(12), v_); }
        { let v_ = ((255i32) as u8); c.m.w8(fp.wrapping_add(13), v_); }
        { let v_ = ((0i32) as u8); c.m.w8(fp.wrapping_add(14), v_); }
        { let v_ = ((84i32) as u8); c.m.w8(fp.wrapping_add(15), v_); }
        { let v_ = ((253i32) as u8); c.m.w8(fp.wrapping_add(16), v_); }
        { let v_ = ((255i32) as u8); c.m.w8(fp.wrapping_add(17), v_); }
        { let v_ = ((255i32) as u8); c.m.w8(fp.wrapping_add(18), v_); }
        { let v_ = ((0i32) as u8); c.m.w8(fp.wrapping_add(19), v_); }
        { let v_ = ((255i32) as u8); c.m.w8(fp.wrapping_add(20), v_); }
        { let v_ = ((64i32) as u8); c.m.w8(fp.wrapping_add(21), v_); }
        { let v_ = ((0i32) as u8); c.m.w8(fp.wrapping_add(22), v_); }
        { let v_ = ((255i32) as u8); c.m.w8(fp.wrapping_add(23), v_); }
        { let v_ = ((255i32) as u8); c.m.w8(fp.wrapping_add(24), v_); }
        { let v_ = ((255i32) as u8); c.m.w8(fp.wrapping_add(25), v_); }
        { let v_ = ((32i32) as u8); c.m.w8(fp.wrapping_add(26), v_); }
        { let v_ = ((255i32) as u8); c.m.w8(fp.wrapping_add(27), v_); }
        { let v_ = ((255i32) as u8); c.m.w8(fp.wrapping_add(28), v_); }
        { let v_ = ((42i32) as u8); c.m.w8(fp.wrapping_add(29), v_); }
        { let v_ = ((255i32) as u8); c.m.w8(fp.wrapping_add(30), v_); }
        { let v_ = ((255i32) as u8); c.m.w8(fp.wrapping_add(31), v_); }
        { let v_ = ((52i32) as u8); c.m.w8(fp.wrapping_add(32), v_); }
        { let v_ = ((0i32) as u8); let a_ = (p_clut_1).wrapping_add(((0i32) as u32)); c.m.w8(a_, v_); };
        { let v_ = ((0i32) as u8); let a_ = (p_clut_1).wrapping_add(((1i32) as u32)); c.m.w8(a_, v_); };
        { let v_ = ((0i32) as u8); let a_ = (p_clut_1).wrapping_add(((2i32) as u32)); c.m.w8(a_, v_); };
        { let v_ = ((0i32) as u8); let a_ = (p_clut_1).wrapping_add(((3i32) as u32)); c.m.w8(a_, v_); };
        {
            l_i_2 = 0i32;
            'l1: loop {
                if !(((((l_i_2) < (256i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_n_3 = (((((((l_i_2) & (231i32))) | ((((l_i_2) & (8i32))).wrapping_shl((1i32) as u32)))) | ((((l_i_2) & (16i32))).wrapping_shr((1i32) as u32)))).wrapping_mul(4i32);
                    if (((((((((l_i_2) >= (244i32)) as i32)) != 0) && (((((l_i_2) != (255i32)) as i32)) != 0)) as i32)) != 0) {
                        { let v_ = c.m.r8((fp).wrapping_add((((((l_i_2).wrapping_sub(244i32)).wrapping_mul(3i32)).wrapping_add(0i32)) as u32))); let a_ = (p_clut_1).wrapping_add((((l_n_3).wrapping_add(0i32)) as u32)); c.m.w8(a_, v_); };
                        { let v_ = c.m.r8((fp).wrapping_add((((((l_i_2).wrapping_sub(244i32)).wrapping_mul(3i32)).wrapping_add(1i32)) as u32))); let a_ = (p_clut_1).wrapping_add((((l_n_3).wrapping_add(1i32)) as u32)); c.m.w8(a_, v_); };
                        { let v_ = c.m.r8((fp).wrapping_add((((((l_i_2).wrapping_sub(244i32)).wrapping_mul(3i32)).wrapping_add(2i32)) as u32))); let a_ = (p_clut_1).wrapping_add((((l_n_3).wrapping_add(2i32)) as u32)); c.m.w8(a_, v_); };
                        { let v_ = ((48i32) as u8); let a_ = (p_clut_1).wrapping_add((((l_n_3).wrapping_add(3i32)) as u32)); c.m.w8(a_, v_); };
                    } else {
                        { let v_ = ((0i32) as u8); let a_ = (p_clut_1).wrapping_add((((l_n_3).wrapping_add(0i32)) as u32)); c.m.w8(a_, v_); };
                        { let v_ = ((0i32) as u8); let a_ = (p_clut_1).wrapping_add((((l_n_3).wrapping_add(1i32)) as u32)); c.m.w8(a_, v_); };
                        { let v_ = ((0i32) as u8); let a_ = (p_clut_1).wrapping_add((((l_n_3).wrapping_add(2i32)) as u32)); c.m.w8(a_, v_); };
                        { let v_ = ((0i32) as u8); let a_ = (p_clut_1).wrapping_add((((l_n_3).wrapping_add(3i32)) as u32)); c.m.w8(a_, v_); };
                    }
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "GfxAlphaKey_BuildClut", &PROBE_N, &[], true); }
    r_
}

pub fn GfxAlphaKey_Init(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "GfxAlphaKey_Init", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let v_ = F::Heap_Alloc(c, ((1216u32) as i32), ((32i32) as u32), 0i32, 2i32); let a_ = sym::gGfxAlphaKey; c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32(sym::gGfxAlphaKey); F::memset(c, a0_, 0i32, 1216u32) }; };
    { let a0_ = c.m.r32(sym::gGfxAlphaKey); F::GfxClut_InitPacket(c, a0_, ((16020i32) as u16)) };
    { let a0_ = { let a_ = (c.m.r32(sym::gGfxAlphaKey)).wrapping_add(1168); c.m.r32(a_) }; F::GfxAlphaKey_BuildClut(c, a0_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "GfxAlphaKey_Init", &PROBE_N, &[], true); }
    r_
}

pub fn GfxAlphaKey_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "GfxAlphaKey_Term", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let a0_ = c.m.r32(sym::gGfxAlphaKey); F::Heap_Free(c, a0_) };
    { let v_ = ((0i32) as u32); let a_ = sym::gGfxAlphaKey; c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "GfxAlphaKey_Term", &PROBE_N, &[], true); }
    r_
}

pub fn GfxAlphaKey_Draw(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "GfxAlphaKey_Draw", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::Port_GsMarker(c, 2i32);
    { let a0_ = (c.m.r32(sym::gGfxAlphaKey)).wrapping_add(1172); F::Dma_AddData(c, a0_, 16i32) };
    F::Dma_AddTexFlush(c);
    F::Dma_AddZbuf(c, 224i32, 1i32);
    { let a2_ = (({ let a_ = (c.m.r32(sym::gGfxAlphaKey)).wrapping_add(1204); c.m.r16(a_) }) as i32); F::GfxPost_DrawDepthClut(c, 0i32, (224i32).wrapping_mul(32i32), a2_, ((68i32) as u64)) };
    F::Dma_AddZbuf(c, 224i32, 0i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "GfxAlphaKey_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn GfxWater_Reset(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "GfxWater_Reset", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let r_ = 'f: {
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (2i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let v_ = F32(0x3d75c28f); let a_ = ((c.m.r32(sym::gGfxWater)).wrapping_add((((l_i_1) as u32)).wrapping_mul(20))).wrapping_add(8); c.m.w32(a_, (v_).0); };
                { let v_ = F32(0x3d23d70a); let a_ = ((c.m.r32(sym::gGfxWater)).wrapping_add((((l_i_1) as u32)).wrapping_mul(20))).wrapping_add(12); c.m.w32(a_, (v_).0); };
                F::GfxWater_SetColor(c, l_i_1, ((92i32) as u8), ((92i32) as u8), ((128i32) as u8), 128i32);
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "GfxWater_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn GfxWater_Init(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "GfxWater_Init", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let v_ = F::Heap_Alloc(c, (((20u32).wrapping_mul(((2i32) as u32))) as i32), ((32i32) as u32), 0i32, 2i32); let a_ = sym::gGfxWater; c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32(sym::gGfxWater); F::memset(c, a0_, 0i32, (20u32).wrapping_mul(((2i32) as u32))) }; };
    F::GfxWater_Reset(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "GfxWater_Init", &PROBE_N, &[], true); }
    r_
}

pub fn GfxWater_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "GfxWater_Term", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if (((((c.m.r32(sym::gGfxWater)) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32(sym::gGfxWater); F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = sym::gGfxWater; c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "GfxWater_Term", &PROBE_N, &[], true); }
    r_
}

pub fn GfxWater_LoadStageColor(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "GfxWater_LoadStageColor", &PROBE_N, &[], false); }
    let mut l_st_1: u32 = 0;
    let r_ = 'f: {
    l_st_1 = F::BtlStage_GetList90(c);
    if (((((l_st_1) != (((0i32) as u32))) as i32)) != 0) {
        if ((((c.m.r32(l_st_1)) & (((1i32) as u32)))) != 0) {
            { let a1_ = c.m.r8((l_st_1).wrapping_add(4)); let a2_ = c.m.r8((l_st_1).wrapping_add(5)); let a3_ = c.m.r8((l_st_1).wrapping_add(6)); let a4_ = ((c.m.r8((l_st_1).wrapping_add(7))) as i32); F::GfxWater_SetColor(c, 0i32, a1_, a2_, a3_, a4_) };
            { let a1_ = c.m.r8((l_st_1).wrapping_add(4)); let a2_ = c.m.r8((l_st_1).wrapping_add(5)); let a3_ = c.m.r8((l_st_1).wrapping_add(6)); let a4_ = ((c.m.r8((l_st_1).wrapping_add(7))) as i32); F::GfxWater_SetColor(c, 1i32, a1_, a2_, a3_, a4_) };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "GfxWater_LoadStageColor", &PROBE_N, &[], true); }
    r_
}

pub fn GfxWater_DrawView(c: &mut Ctx, mut p_split_1: i32, mut p_view_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "GfxWater_DrawView", &PROBE_N, &[(p_split_1) as u32, (p_view_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(480) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 480);
    let mut l_srcW_3: i32 = 0;
    let mut l_w_4: i32 = 0;
    let mut l_x0_5: i32 = 0;
    let mut l_x1_6: i32 = 0;
    let mut l_cols_7: i32 = 0;
    let mut l_row_8: i32 = 0;
    let mut l_col_9: i32 = 0;
    let mut l_cx_10: i32 = 0;
    let mut l_cy_11: i32 = 0;
    let mut l_i_12: i32 = 0;
    let mut l_n_13: i32 = 0;
    let mut l_step_14: i32 = 0;
    let mut l_sx_15: i32 = 0;
    let mut l_rgba_16: u32 = 0;
    let mut l_p_17: u32 = 0;
    let mut l_fw_18: F32 = F32(0);
    let mut l_fx_19: F32 = F32(0);
    let mut l_fy_20: F32 = F32(0);
    let mut l_fh_21: F32 = F32(0);
    let mut l_dx_22: F32 = F32(0);
    let mut l_dy_23: F32 = F32(0);
    let mut l_d_24: F32 = F32(0);
    let mut l_ph_25: F32 = F32(0);
    let mut l_h_26: i32 = 0;
    let mut l_srcH_27: i32 = 0;
    let r_ = 'f: {
        l_h_26 = 224i32;
        l_srcH_27 = 448i32;
        if (((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((8192i32) as u64)))) != 0) {
            break 'f;
        }
        if ((((!(((({ let a_ = F::BtlStage_GetList90(c); c.m.r32(a_) }) & (((1i32) as u32)))) != 0)) as i32)) != 0) {
            break 'f;
        }
        if ((((!(((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0)) as i32)) != 0) {
            { let r_ = { let a_ = ((c.m.r32(sym::gGfxWater)).wrapping_add((((p_view_2) as u32)).wrapping_mul(20))).wrapping_add(8); F32(c.m.r32(a_)) }; let a_ = (c.m.r32(sym::gGfxWater)).wrapping_add((((p_view_2) as u32)).wrapping_mul(20)); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            if (((rt::fgt({ let a_ = (c.m.r32(sym::gGfxWater)).wrapping_add((((p_view_2) as u32)).wrapping_mul(20)); F32(c.m.r32(a_)) }, F32(0x40490fda)) as i32)) != 0) {
                { let r_ = F32(0x40c90fda); let a_ = (c.m.r32(sym::gGfxWater)).wrapping_add((((p_view_2) as u32)).wrapping_mul(20)); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            }
            { let r_ = { let a_ = ((c.m.r32(sym::gGfxWater)).wrapping_add((((p_view_2) as u32)).wrapping_mul(20))).wrapping_add(12); F32(c.m.r32(a_)) }; let a_ = ((c.m.r32(sym::gGfxWater)).wrapping_add((((p_view_2) as u32)).wrapping_mul(20))).wrapping_add(4); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            if (((rt::fgt({ let a_ = ((c.m.r32(sym::gGfxWater)).wrapping_add((((p_view_2) as u32)).wrapping_mul(20))).wrapping_add(4); F32(c.m.r32(a_)) }, F32(0x40490fda)) as i32)) != 0) {
                { let r_ = F32(0x40c90fda); let a_ = ((c.m.r32(sym::gGfxWater)).wrapping_add((((p_view_2) as u32)).wrapping_mul(20))).wrapping_add(4); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            }
        }
        if (((((p_split_1) != (0i32)) as i32)) != 0) {
            l_srcW_3 = 256i32;
            l_w_4 = 256i32;
            l_cols_7 = 5i32;
            if (((((p_view_2) == (0i32)) as i32)) != 0) {
                l_x0_5 = 0i32;
                l_x1_6 = 256i32;
            } else {
                l_x0_5 = 256i32;
                l_x1_6 = 512i32;
            }
        } else {
            l_srcW_3 = 512i32;
            l_w_4 = 512i32;
            l_x0_5 = 0i32;
            l_x1_6 = 512i32;
            l_cols_7 = 10i32;
        }
        {
            l_row_8 = 0i32;
            'l1: loop {
                if !(((((l_row_8) < (6i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    {
                        l_col_9 = 0i32;
                        'l3: loop {
                            if !(((((l_col_9) < (l_cols_7)) as i32)) != 0) { break 'l3; }
                            'c4: {
                                l_cx_10 = rt::div_i32((l_col_9).wrapping_mul(l_w_4), (l_cols_7).wrapping_sub(1i32));
                                l_cy_11 = rt::div_i32((l_h_26).wrapping_mul(l_row_8), 5i32);
                                { let v_ = (l_cx_10).wrapping_shl((4i32) as u32); let a_ = ((fp).wrapping_add((((l_row_8) as u32)).wrapping_mul(40))).wrapping_add((((l_col_9) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                { let v_ = (l_cy_11).wrapping_shl((4i32) as u32); let a_ = ((fp.wrapping_add(240)).wrapping_add((((l_row_8) as u32)).wrapping_mul(40))).wrapping_add((((l_col_9) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                if (((((((((l_row_8) != (0i32)) as i32)) != 0) && (((((l_row_8) != (9i32)) as i32)) != 0)) as i32)) != 0) {
                                    l_fw_18 = rt::i2f(l_w_4);
                                    l_fx_19 = rt::i2f(l_cx_10);
                                    l_fy_20 = rt::i2f(l_cy_11);
                                    l_fh_21 = rt::i2f(l_h_26);
                                    l_dx_22 = rt::fsub(l_fx_19, rt::fmul(l_fw_18, F32(0x3e4ccccc)));
                                    l_dy_23 = rt::fsub(l_fy_20, F32(0x42333332));
                                    l_d_24 = F::Port_sqrtf(c, rt::fadd(rt::fmul(l_dx_22, l_dx_22), rt::fmul(l_dy_23, l_dy_23)));
                                    l_ph_25 = rt::fmul(l_d_24, F32(0x3e4ccccc));
                                    { let r_ = rt::f2i(rt::fmul(rt::fmul(rt::fmul({ let a0_ = rt::fadd({ let a_ = (c.m.r32(sym::gGfxWater)).wrapping_add((((p_view_2) as u32)).wrapping_mul(20)); F32(c.m.r32(a_)) }, l_ph_25); F::Port_cosf(c, a0_) }, rt::fdiv(l_dx_22, l_d_24)), F32(0x40400000)), F32(0x41800000))); let a_ = ((fp).wrapping_add((((l_row_8) as u32)).wrapping_mul(40))).wrapping_add((((l_col_9) as u32)).wrapping_mul(4)); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                                    { let r_ = rt::f2i(rt::fmul(rt::fmul(rt::fmul({ let a0_ = rt::fadd({ let a_ = (c.m.r32(sym::gGfxWater)).wrapping_add((((p_view_2) as u32)).wrapping_mul(20)); F32(c.m.r32(a_)) }, l_ph_25); F::Port_cosf(c, a0_) }, rt::fdiv(l_dy_23, l_d_24)), F32(0x40400000)), F32(0x41800000))); let a_ = ((fp.wrapping_add(240)).wrapping_add((((l_row_8) as u32)).wrapping_mul(40))).wrapping_add((((l_col_9) as u32)).wrapping_mul(4)); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                                    l_dx_22 = rt::fsub(l_fx_19, rt::fmul(l_fw_18, F32(0x3f4ccccc)));
                                    l_dy_23 = rt::fsub(l_fy_20, rt::fmul(F32(0x3f4ccccc), l_fh_21));
                                    l_d_24 = F::Port_sqrtf(c, rt::fadd(rt::fmul(l_dx_22, l_dx_22), rt::fmul(l_dy_23, l_dy_23)));
                                    l_ph_25 = rt::fmul(l_d_24, F32(0x3e4ccccc));
                                    { let r_ = rt::f2i(rt::fmul(rt::fmul(rt::fmul({ let a0_ = rt::fadd({ let a_ = ((c.m.r32(sym::gGfxWater)).wrapping_add((((p_view_2) as u32)).wrapping_mul(20))).wrapping_add(4); F32(c.m.r32(a_)) }, l_ph_25); F::Port_cosf(c, a0_) }, rt::fdiv(l_dx_22, l_d_24)), F32(0x40400000)), F32(0x41800000))); let a_ = ((fp).wrapping_add((((l_row_8) as u32)).wrapping_mul(40))).wrapping_add((((l_col_9) as u32)).wrapping_mul(4)); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                                    { let r_ = rt::f2i(rt::fmul(rt::fmul(rt::fmul({ let a0_ = rt::fadd({ let a_ = ((c.m.r32(sym::gGfxWater)).wrapping_add((((p_view_2) as u32)).wrapping_mul(20))).wrapping_add(4); F32(c.m.r32(a_)) }, l_ph_25); F::Port_cosf(c, a0_) }, rt::fdiv(l_dy_23, l_d_24)), F32(0x40400000)), F32(0x41800000))); let a_ = ((fp.wrapping_add(240)).wrapping_add((((l_row_8) as u32)).wrapping_mul(40))).wrapping_add((((l_col_9) as u32)).wrapping_mul(4)); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                                }
                                if ((((((c.m.r32(((fp).wrapping_add((((l_row_8) as u32)).wrapping_mul(40))).wrapping_add((((l_col_9) as u32)).wrapping_mul(4))) as i32)) < (0i32)) as i32)) != 0) {
                                    { let v_ = 0i32; let a_ = ((fp).wrapping_add((((l_row_8) as u32)).wrapping_mul(40))).wrapping_add((((l_col_9) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                }
                                if ((((((c.m.r32(((fp).wrapping_add((((l_row_8) as u32)).wrapping_mul(40))).wrapping_add((((l_col_9) as u32)).wrapping_mul(4))) as i32)) > ((l_w_4).wrapping_shl((4i32) as u32))) as i32)) != 0) {
                                    { let v_ = (l_w_4).wrapping_shl((4i32) as u32); let a_ = ((fp).wrapping_add((((l_row_8) as u32)).wrapping_mul(40))).wrapping_add((((l_col_9) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                }
                                if ((((((c.m.r32(((fp.wrapping_add(240)).wrapping_add((((l_row_8) as u32)).wrapping_mul(40))).wrapping_add((((l_col_9) as u32)).wrapping_mul(4))) as i32)) < (0i32)) as i32)) != 0) {
                                    { let v_ = 0i32; let a_ = ((fp.wrapping_add(240)).wrapping_add((((l_row_8) as u32)).wrapping_mul(40))).wrapping_add((((l_col_9) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                }
                                if ((((((c.m.r32(((fp.wrapping_add(240)).wrapping_add((((l_row_8) as u32)).wrapping_mul(40))).wrapping_add((((l_col_9) as u32)).wrapping_mul(4))) as i32)) > ((l_h_26).wrapping_shl((4i32) as u32))) as i32)) != 0) {
                                    { let v_ = (l_h_26).wrapping_shl((4i32) as u32); let a_ = ((fp.wrapping_add(240)).wrapping_add((((l_row_8) as u32)).wrapping_mul(40))).wrapping_add((((l_col_9) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                }
                            }
                            { let o_ = l_col_9; let n_ = (o_).wrapping_add(1); l_col_9 = n_; };
                        }
                    }
                }
                { let o_ = l_row_8; let n_ = (o_).wrapping_add(1); l_row_8 = n_; };
            }
        }
        F::Gfx_AddDefaultEnv(c);
        l_p_17 = F::Dma_BeginDirect(c);
        { let v_ = ((((((((9i32) as u64)) | ((((0i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (l_p_17).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((14i32) as u64); let a_ = (l_p_17).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_17).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_17 = n_; };
        { let v_ = ((196608i32) as u64); let a_ = (l_p_17).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((71i32) as u64); let a_ = (l_p_17).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_17).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_17 = n_; };
        { let v_ = ((0i32) as u64); let a_ = (l_p_17).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((74i32) as u64); let a_ = (l_p_17).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_17).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_17 = n_; };
        { let v_ = ((96i32) as u64); let a_ = (l_p_17).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((20i32) as u64); let a_ = (l_p_17).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_17).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_17 = n_; };
        { let v_ = (((if ((((!(((((c.m.r32((sym::gGfx).wrapping_add(564)) as i32)) & (1i32))) != 0)) as i32)) != 0) { 10267790848i64 } else { 10267787264i64 })) as u64); let a_ = (l_p_17).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((6i32) as u64); let a_ = (l_p_17).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_17).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_17 = n_; };
        { let v_ = ((((((((336i32) as u64)) | ((((8i32) as u64)).wrapping_shl((16i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((32i32) as u32))); let a_ = (l_p_17).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((76i32) as u64); let a_ = (l_p_17).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_17).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_17 = n_; };
        { let v_ = ((((((224i32) as u64)) | ((((49i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((32i32) as u32))); let a_ = (l_p_17).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((78i32) as u64); let a_ = (l_p_17).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_17).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_17 = n_; };
        { let v_ = ((((((((l_w_4).wrapping_sub(1i32)) as i64)).wrapping_shl((16i32) as u32)) as u64)) | ((((223i32) as u64)).wrapping_shl((48i32) as u32))); let a_ = (l_p_17).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((64i32) as u64); let a_ = (l_p_17).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_17).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_17 = n_; };
        { let v_ = (((((((((((l_x0_5) as i64)).wrapping_shl((4i32) as u32)) as u64)) | ((((448i32) as u64)).wrapping_shl((34i32) as u32)))) | ((((((l_x1_6) as i64)).wrapping_shl((14i32) as u32)) as u64)))) | (((10i32) as u64))); let a_ = (l_p_17).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((8i32) as u64); let a_ = (l_p_17).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_17).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_17 = n_; };
        { let v_ = ((68i32) as u64); let a_ = (l_p_17).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((66i32) as u64); let a_ = (l_p_17).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_17).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_17 = n_; };
        { let v_ = ((((((((2i32) as u64)) | ((((0i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (l_p_17).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((14i32) as u64); let a_ = (l_p_17).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_17).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_17 = n_; };
        { let v_ = ((278i32) as u64); let a_ = (l_p_17).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((0i32) as u64); let a_ = (l_p_17).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_17).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_17 = n_; };
        { let v_ = ((4575657223564329088i64) as u64); let a_ = (l_p_17).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((1i32) as u64); let a_ = (l_p_17).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_17).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_17 = n_; };
        l_n_13 = (((((l_srcW_3) as u32)).wrapping_shr((5i32) as u32)) as i32);
        l_step_14 = rt::div_i32(l_w_4, l_n_13);
        { let v_ = ((((((((l_n_13) as u64)) | ((((0i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((4i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (l_p_17).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((21331i32) as u64); let a_ = (l_p_17).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_17).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_17 = n_; };
        {
            l_row_8 = 0i32;
            'l5: loop {
                if !(((((l_row_8) < (l_n_13)) as i32)) != 0) { break 'l5; }
                'c6: {
                    { let v_ = ((((l_x0_5).wrapping_add((l_row_8).wrapping_mul(32i32))).wrapping_shl((4i32) as u32)) as u64); let a_ = (l_p_17).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                    { let v_ = (((((28672i32).wrapping_add(((l_row_8).wrapping_mul(l_step_14)).wrapping_shl((4i32) as u32))) | ((29184i32).wrapping_shl((16i32) as u32)))) as u64); let a_ = (l_p_17).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                    { let r_ = 2i32; let n_ = (l_p_17).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_17 = n_; };
                    { let v_ = ((((((l_x0_5).wrapping_add(((l_row_8).wrapping_add(1i32)).wrapping_mul(32i32))).wrapping_shl((4i32) as u32)) | ((7168i32).wrapping_shl((16i32) as u32)))) as u64); let a_ = (l_p_17).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                    { let v_ = (((((((28672i32).wrapping_add((((l_row_8).wrapping_add(1i32)).wrapping_mul(l_step_14)).wrapping_shl((4i32) as u32))) as i64)) as u64)) | ((((32768i32) as u64)).wrapping_shl((16i32) as u32))); let a_ = (l_p_17).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                    { let r_ = 2i32; let n_ = (l_p_17).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_17 = n_; };
                }
                { let o_ = l_row_8; let n_ = (o_).wrapping_add(1); l_row_8 = n_; };
            }
        }
        { let v_ = ((((((((4i32) as u64)) | ((((0i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (l_p_17).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((14i32) as u64); let a_ = (l_p_17).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_17).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_17 = n_; };
        { let v_ = ((10267798016i64) as u64); let a_ = (l_p_17).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((6i32) as u64); let a_ = (l_p_17).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_17).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_17 = n_; };
        { let v_ = (((if ((((!(((((c.m.r32((sym::gGfx).wrapping_add(564)) as i32)) & (1i32))) != 0)) as i32)) != 0) { 524400i32 } else { 524288i32 })) as u64); let a_ = (l_p_17).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((76i32) as u64); let a_ = (l_p_17).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_17).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_17 = n_; };
        { let v_ = ((((((((l_x0_5) as i64)) | (((((l_x1_6).wrapping_sub(1i32)) as i64)).wrapping_shl((16i32) as u32)))) as u64)) | ((((447i32) as u64)).wrapping_shl((48i32) as u32))); let a_ = (l_p_17).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((64i32) as u64); let a_ = (l_p_17).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_17).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_17 = n_; };
        { let v_ = (((((((((l_w_4) as i64)).wrapping_shl((14i32) as u32)) | ((((l_h_26) as i64)).wrapping_shl((34i32) as u32)))) | (((10i32) as i64)))) as u64); let a_ = (l_p_17).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((8i32) as u64); let a_ = (l_p_17).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_17).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_17 = n_; };
        {
            l_row_8 = 0i32;
            'l7: loop {
                if !(((((l_row_8) < (5i32)) as i32)) != 0) { break 'l7; }
                'c8: {
                    { let v_ = ((((((((1i32) as u64)) | ((((0i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((2i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (l_p_17).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                    { let v_ = ((16i32) as u64); let a_ = (l_p_17).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                    { let r_ = 2i32; let n_ = (l_p_17).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_17 = n_; };
                    { let v_ = ((276i32) as u64); let a_ = (l_p_17).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                    l_rgba_16 = { let a_ = ((c.m.r32(sym::gGfxWater)).wrapping_add((((p_view_2) as u32)).wrapping_mul(20))).wrapping_add(16); c.m.r32(a_) };
                    { let v_ = ((((((((((l_rgba_16) & (((255i32) as u32)))) as u64)) | (((((((l_rgba_16).wrapping_shr((8i32) as u32)) & (((255i32) as u32)))) as u64)).wrapping_shl((8i32) as u32)))) | (((((((l_rgba_16).wrapping_shr((16i32) as u32)) & (((255i32) as u32)))) as u64)).wrapping_shl((16i32) as u32)))) | (((((((l_rgba_16).wrapping_shr((24i32) as u32)) & (((255i32) as u32)))) as u64)).wrapping_shl((24i32) as u32))); let a_ = (l_p_17).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                    { let r_ = 2i32; let n_ = (l_p_17).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_17 = n_; };
                    { let v_ = ((((((((l_cols_7) as u64)) | ((((0i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((4i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (l_p_17).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                    { let v_ = ((21331i32) as u64); let a_ = (l_p_17).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                    { let r_ = 2i32; let n_ = (l_p_17).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_17 = n_; };
                    {
                        l_col_9 = 0i32;
                        'l9: loop {
                            if !(((((l_col_9) < (l_cols_7)) as i32)) != 0) { break 'l9; }
                            'c10: {
                                l_sx_15 = (((l_x0_5).wrapping_add(rt::div_i32((l_col_9).wrapping_mul(l_srcW_3), (l_cols_7).wrapping_sub(1i32)))).wrapping_shl((4i32) as u32)).wrapping_add(28672i32);
                                { let v_ = (((((((c.m.r32(((fp).wrapping_add((((l_row_8) as u32)).wrapping_mul(40))).wrapping_add((((l_col_9) as u32)).wrapping_mul(4))) as i32)) as i64)) | (((((c.m.r32(((fp.wrapping_add(240)).wrapping_add((((l_row_8) as u32)).wrapping_mul(40))).wrapping_add((((l_col_9) as u32)).wrapping_mul(4))) as i32)) as i64)).wrapping_shl((16i32) as u32)))) as u64); let a_ = (l_p_17).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                                { let v_ = ((((((l_sx_15) as i64)) | ((((((rt::div_i32((l_srcH_27).wrapping_mul(l_row_8), 5i32)).wrapping_shl((4i32) as u32)).wrapping_add(29184i32)) as i64)).wrapping_shl((16i32) as u32)))) as u64); let a_ = (l_p_17).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                                { let r_ = 2i32; let n_ = (l_p_17).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_17 = n_; };
                                { let v_ = (((((((c.m.r32(((fp).wrapping_add(((((l_row_8).wrapping_add(1i32)) as u32)).wrapping_mul(40))).wrapping_add((((l_col_9) as u32)).wrapping_mul(4))) as i32)) as i64)) | (((((c.m.r32(((fp.wrapping_add(240)).wrapping_add(((((l_row_8).wrapping_add(1i32)) as u32)).wrapping_mul(40))).wrapping_add((((l_col_9) as u32)).wrapping_mul(4))) as i32)) as i64)).wrapping_shl((16i32) as u32)))) as u64); let a_ = (l_p_17).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                                { let v_ = ((((((l_sx_15) as i64)) | ((((((rt::div_i32((l_srcH_27).wrapping_mul((l_row_8).wrapping_add(1i32)), 5i32)).wrapping_shl((4i32) as u32)).wrapping_add(29184i32)) as i64)).wrapping_shl((16i32) as u32)))) as u64); let a_ = (l_p_17).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                                { let r_ = 2i32; let n_ = (l_p_17).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_17 = n_; };
                            }
                            { let o_ = l_col_9; let n_ = (o_).wrapping_add(1); l_col_9 = n_; };
                        }
                    }
                }
                { let o_ = l_row_8; let n_ = (o_).wrapping_add(1); l_row_8 = n_; };
            }
        }
        { let v_ = ((((((((2i32) as u64)) | ((((1i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (l_p_17).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((14i32) as u64); let a_ = (l_p_17).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_17).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_17 = n_; };
        { let v_ = ((((((224i32) as u64)) | ((((49i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((32i32) as u32))); let a_ = (l_p_17).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((78i32) as u64); let a_ = (l_p_17).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_17).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_17 = n_; };
        { let v_ = ((((((((0i32) as u64)) | ((((511i32) as u64)).wrapping_shl((16i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((32i32) as u32)))) | ((((447i32) as u64)).wrapping_shl((48i32) as u32))); let a_ = (l_p_17).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((64i32) as u64); let a_ = (l_p_17).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_17).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_17 = n_; };
        F::Dma_EndDirect(c, l_p_17);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "GfxWater_DrawView", &PROBE_N, &[], true); }
    r_
}

pub fn GfxWater_Draw(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "GfxWater_Draw", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(80) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 80);
    let mut l_views_1: i32 = 0;
    let mut l_split_2: i32 = 0;
    let mut l_i_3: i32 = 0;
    let mut l_v_4: u32 = 0;
    let mut l_c_5: u32 = 0;
    let r_ = 'f: {
        l_views_1 = 1i32;
        l_split_2 = 0i32;
        if (((((c.m.r32(sym::gBtlCam)) == (((0i32) as u32))) as i32)) != 0) {
            break 'f;
        }
        if ((((((((((F::Battle_IsSplitScreen(c)) != 0) && ((((!((F::DemoCam_IsActive(c)) != 0)) as i32)) != 0)) as i32)) != 0) && ((((({ let a_ = ({ let a_ = (c.m.r32(sym::gBtlCam)).wrapping_add(3136); c.m.r32(a_) }).wrapping_add(640); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0)) as i32)) != 0) {
            l_split_2 = 1i32;
            l_views_1 = 2i32;
        }
        if (((((l_views_1) >= (2i32)) as i32)) != 0) {
            {
                l_i_3 = 0i32;
                'l1: loop {
                    if !(((((l_i_3) < (l_views_1)) as i32)) != 0) { break 'l1; }
                    'c2: {
                        l_c_5 = c.m.r32(sym::gBtlCam);
                        if ((F::BtlStage_GetWaterLevel(c, fp.wrapping_add(64))) != 0) {
                            F::Mtx_InverseRT(c, fp, (((l_c_5).wrapping_add(1824)).wrapping_add((((l_i_3) as u32)).wrapping_mul(656))).wrapping_add(64));
                            if (((rt::flt(F32(c.m.r32(fp.wrapping_add(64))), F32(c.m.r32(((fp).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4))))) as i32)) != 0) {
                                F::GfxWater_DrawView(c, l_split_2, l_i_3);
                            }
                        }
                    }
                    { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
                }
            }
        } else {
            l_v_4 = c.m.r32(sym::gBtlCamView);
            if ((F::BtlStage_GetWaterLevel(c, fp.wrapping_add(64))) != 0) {
                F::Mtx_InverseRT(c, fp, (l_v_4).wrapping_add(64));
                if (((rt::flt(F32(c.m.r32(fp.wrapping_add(64))), F32(c.m.r32(((fp).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4))))) as i32)) != 0) {
                    F::GfxWater_DrawView(c, l_split_2, 0i32);
                }
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "GfxWater_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn GfxWater_SetColor(c: &mut Ctx, mut p_view_1: i32, mut p_r_2: u8, mut p_g_3: u8, mut p_b_4: u8, mut p_a_5: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "GfxWater_SetColor", &PROBE_N, &[(p_view_1) as u32, (p_r_2) as u32, (p_g_3) as u32, (p_b_4) as u32, (p_a_5) as u32], false); }
    let r_ = 'f: {
    { let v_ = ((((((((p_r_2) as i32)) | ((p_a_5).wrapping_shl((24i32) as u32)))) | ((((((p_b_4) as i32)).wrapping_shl((16i32) as u32)) | ((((p_g_3) as i32)).wrapping_shl((8i32) as u32)))))) as u32); let a_ = ((c.m.r32(sym::gGfxWater)).wrapping_add((((p_view_1) as u32)).wrapping_mul(20))).wrapping_add(16); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "GfxWater_SetColor", &PROBE_N, &[], true); }
    r_
}

pub fn GfxDepthFog_BuildClut(c: &mut Ctx, mut p_clut_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "GfxDepthFog_BuildClut", &PROBE_N, &[(p_clut_1) as u32], false); }
    let mut l_i_2: i32 = 0;
    let mut l_n_3: i32 = 0;
    let r_ = 'f: {
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < (256i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                l_n_3 = (((((((l_i_2) & (231i32))) | ((((l_i_2) & (8i32))).wrapping_shl((1i32) as u32)))) | ((((l_i_2) & (16i32))).wrapping_shr((1i32) as u32)))).wrapping_mul(4i32);
                { let v_ = ((0i32) as u8); let a_ = (p_clut_1).wrapping_add((((l_n_3).wrapping_add(0i32)) as u32)); c.m.w8(a_, v_); };
                { let v_ = ((0i32) as u8); let a_ = (p_clut_1).wrapping_add((((l_n_3).wrapping_add(1i32)) as u32)); c.m.w8(a_, v_); };
                { let v_ = ((0i32) as u8); let a_ = (p_clut_1).wrapping_add((((l_n_3).wrapping_add(2i32)) as u32)); c.m.w8(a_, v_); };
                { let v_ = (((255i32).wrapping_sub(l_i_2)) as u8); let a_ = (p_clut_1).wrapping_add((((l_n_3).wrapping_add(3i32)) as u32)); c.m.w8(a_, v_); };
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "GfxDepthFog_BuildClut", &PROBE_N, &[], true); }
    r_
}

pub fn GfxPost_ShiftHighWord(c: &mut Ctx, mut p_fbp_1: i32, mut p_tbp_2: i32, mut p_h_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "GfxPost_ShiftHighWord", &PROBE_N, &[(p_fbp_1) as u32, (p_tbp_2) as u32, (p_h_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(64) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 64);
    let mut l_p_4: u32 = 0;
    let mut l_tag_5: u32 = 0;
    let mut l_i_6: i32 = 0;
    let mut l_w_7: i32 = 0;
    let mut l_tw_8: i32 = 0;
    let mut l_n_9: i32 = 0;
    let mut l_u_10: i32 = 0;
    let r_ = 'f: {
        l_w_7 = 8i32;
        l_tw_8 = 8i32;
        l_p_4 = F::Dma_BeginDirect(c);
        { let v_ = ((((((((11i32) as u64)) | ((((1i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((14i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
        { let v_ = ((((((((p_fbp_1) | (34078720i32))) as i64)) | (70364449210368i64))) as u64); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((76i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
        { let v_ = ((196608i32) as u64); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((71i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
        { let v_ = ((((((p_h_3).wrapping_sub(1i32)) as u64)).wrapping_shl((48i32) as u32)) | (((33488896i32) as u64))); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((64i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
        { let v_ = ((0i32) as u64); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((63i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
        { let v_ = ((549755813888i64) as u64); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((59i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
        { let v_ = ((0i32) as u64); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((20i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
        { let v_ = ((((((((p_tbp_2) | (723648512i32))) as i64)) | (62277025792i64))) as u64); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((6i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
        { let v_ = ((278i32) as u64); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((0i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
        { let v_ = ((4575657223564329088i64) as u64); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((1i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
        { let v_ = ((0i32) as u64); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((74i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
        { let v_ = ((((28672i32) as u64)) | (((((29184i32).wrapping_add(8i32)) as u64)).wrapping_shl((32i32) as u32))); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((24i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
        l_tag_5 = l_p_4;
        l_n_9 = rt::div_i32(512i32, l_w_7);
        { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
        {
            l_i_6 = 0i32;
            'l1: loop {
                if !(((((l_i_6) < (l_n_9)) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_u_10 = ((l_i_6).wrapping_add(1i32)).wrapping_mul(l_tw_8);
                    F::GfxPostQuad_Set(c, fp, 1792i32, 1824i32, (((l_i_6).wrapping_mul(l_w_7)) as i16), ((0i32) as i16), ((((l_i_6).wrapping_mul(l_w_7)).wrapping_add(8i32)) as i16), ((p_h_3) as i16), l_u_10, 0i32, l_tw_8, p_h_3, ((0i32) as i16));
                    { let v_ = (((((((c.m.r32((fp).wrapping_add(32)) as i32)) as i64)) | (((((c.m.r32((fp).wrapping_add(36)) as i32)) as i64)).wrapping_shl((16i32) as u32)))) as u64); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                    { let v_ = (((((((c.m.r32((fp).wrapping_add(16)) as i32)) as i64)) | (((((c.m.r32((fp).wrapping_add(20)) as i32)) as i64)).wrapping_shl((16i32) as u32)))) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
                    { let v_ = (((((((c.m.r32((fp).wrapping_add(40)) as i32)) as i64)) | (((((c.m.r32((fp).wrapping_add(44)) as i32)) as i64)).wrapping_shl((16i32) as u32)))) as u64); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                    { let v_ = (((((((c.m.r32((fp).wrapping_add(24)) as i32)) as i64)) | (((((c.m.r32((fp).wrapping_add(28)) as i32)) as i64)).wrapping_shl((16i32) as u32)))) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
                }
                { let r_ = 2i32; let n_ = (l_i_6).wrapping_add(r_); l_i_6 = n_; };
            }
        }
        { let v_ = ((((((((64i32) as u64)) | ((((1i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((2i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (l_tag_5).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((83i32) as u64); let a_ = (l_tag_5).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((((((((2i32) as u64)) | ((((1i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((14i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
        { let v_ = ((((((((0i32) as u64)) | ((((511i32) as u64)).wrapping_shl((16i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((32i32) as u32)))) | ((((447i32) as u64)).wrapping_shl((48i32) as u32))); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((64i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
        { let v_ = ((((28672i32) as u64)) | ((((29184i32) as u64)).wrapping_shl((32i32) as u32))); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((24i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
        F::Dma_EndDirect(c, l_p_4);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "GfxPost_ShiftHighWord", &PROBE_N, &[], true); }
    r_
}

pub fn GfxDepthFog_Init(c: &mut Ctx, mut p_cbp_1: u16) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "GfxDepthFog_Init", &PROBE_N, &[(p_cbp_1) as u32], false); }
    let r_ = 'f: {
    { let _ = F::memset(c, sym::gGfxDepthFog, 0i32, 1216u32); };
    F::GfxClut_InitPacket(c, sym::gGfxDepthFog, p_cbp_1);
    { let a0_ = c.m.r32((sym::gGfxDepthFog).wrapping_add(1168)); F::GfxDepthFog_BuildClut(c, a0_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "GfxDepthFog_Init", &PROBE_N, &[], true); }
    r_
}

pub fn GfxDepthFog_Draw(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "GfxDepthFog_Draw", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::Dma_AddData(c, (sym::gGfxDepthFog).wrapping_add(1172), 16i32);
    F::Dma_AddTexFlush(c);
    F::Dma_AddZbuf(c, 224i32, 1i32);
    { let a0_ = (if (((((c.m.r32((sym::gGfx).wrapping_add(564)) as i32)) & (1i32))) != 0) { 0i32 } else { 112i32 }); F::GfxPost_ShiftHighWord(c, a0_, (224i32).wrapping_mul(32i32), 896i32) };
    F::Dma_AddFrame(c, 224i32, 8i32, 16777215i32);
    F::Dma_AddFillRect(c, 1792i32, 1824i32, 512i32, 448i32, ((0i32) as u32));
    { let a1_ = (if (((((c.m.r32((sym::gGfx).wrapping_add(564)) as i32)) & (1i32))) != 0) { (0i32).wrapping_mul(32i32) } else { (112i32).wrapping_mul(32i32) }); let a2_ = ((c.m.r16((sym::gGfxDepthFog).wrapping_add(1204))) as i32); F::GfxPost_DrawDepthClut(c, 1i32, a1_, a2_, ((68i32) as u64)) };
    { let a0_ = (if (((((c.m.r32((sym::gGfx).wrapping_add(564)) as i32)) & (1i32))) != 0) { 0i32 } else { 112i32 }); F::Dma_AddFrame(c, a0_, 8i32, 0i32) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "GfxDepthFog_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn GfxDepthFog_Stub109928(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "GfxDepthFog_Stub109928", &PROBE_N, &[], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "GfxDepthFog_Stub109928", &PROBE_N, &[], true); }
    r_
}

pub fn GfxDepthFog_Stub109930(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "GfxDepthFog_Stub109930", &PROBE_N, &[], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "GfxDepthFog_Stub109930", &PROBE_N, &[], true); }
    r_
}
