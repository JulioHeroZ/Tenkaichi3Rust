#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn Sprite_Unpack(c: &mut Ctx, mut p_src_1: u32, mut p_dst_2: u32, mut p_rawSize_3: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Sprite_Unpack", &PROBE_N, &[(p_src_1) as u32, (p_dst_2) as u32, (p_rawSize_3) as u32], false); }
    let r_ = 'f: {
    break 'f F::Bpe_Decode(c, p_src_1, p_dst_2, p_rawSize_3);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Sprite_Unpack", &PROBE_N, &[], true); }
    r_
}

pub fn Sprite_ConvY(c: &mut Ctx, mut p_y_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Sprite_ConvY", &PROBE_N, &[(p_y_1) as u32], false); }
    let r_ = 'f: {
    break 'f p_y_1;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Sprite_ConvY", &PROBE_N, &[], true); }
    r_
}

pub fn Sprite_DrawPicture(c: &mut Ctx, mut p_res_1: u32, mut p_x_2: i32, mut p_y_3: i32, mut p_alpha_4: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Sprite_DrawPicture", &PROBE_N, &[(p_res_1) as u32, (p_x_2) as u32, (p_y_3) as u32, (p_alpha_4) as u32], false); }
    let mut l_p_5: u32 = 0;
    let mut l_row_6: i32 = 0;
    let mut l_col_7: i32 = 0;
    let mut l_idx_8: i32 = 0;
    let mut l_yy_9: i32 = 0;
    let mut l_y0_10: i32 = 0;
    let mut l_y1_11: i32 = 0;
    let r_ = 'f: {
    if (((((p_alpha_4) == (0i32)) as i32)) != 0) {
        break 'f;
    }
    F::Gfx_AddDefaultEnv(c);
    l_yy_9 = p_y_3;
    l_p_5 = F::Dma_BeginDirect(c);
    { let v_ = ((((((((5i32) as u64)) | ((((1i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (l_p_5).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((14i32) as u64); let a_ = (l_p_5).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_5).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_5 = n_; };
    { let v_ = ((68i32) as u64); let a_ = (l_p_5).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((66i32) as u64); let a_ = (l_p_5).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_5).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_5 = n_; };
    { let v_ = ((0i32) as u64); let a_ = (l_p_5).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((74i32) as u64); let a_ = (l_p_5).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_5).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_5 = n_; };
    { let v_ = ((196608i32) as u64); let a_ = (l_p_5).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((71i32) as u64); let a_ = (l_p_5).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_5).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_5 = n_; };
    { let v_ = (((((p_alpha_4) as u64)).wrapping_shl((24i32) as u32)) | (((8421504i32) as u64))); let a_ = (l_p_5).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((1i32) as u64); let a_ = (l_p_5).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_5).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_5 = n_; };
    { let v_ = ((5i32) as u64); let a_ = (l_p_5).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((8i32) as u64); let a_ = (l_p_5).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_5).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_5 = n_; };
    F::Dma_EndDirect(c, l_p_5);
    {
        l_row_6 = 0i32;
        'l1: loop {
            if !(((((l_row_6) < (2i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                {
                    l_col_7 = 0i32;
                    'l3: loop {
                        if !(((((l_col_7) < (4i32)) as i32)) != 0) { break 'l3; }
                        'c4: {
                            l_idx_8 = (l_col_7).wrapping_add((l_row_6).wrapping_mul(4i32));
                            { let a3_ = ({ let a_ = ((c.m.r32((p_res_1).wrapping_add(16))).wrapping_add((((l_idx_8) as u32)).wrapping_mul(64))).wrapping_add(16); (c.m.r32(a_) as i32) }).wrapping_add(12288i32); F::TexFile_UploadOne(c, p_res_1, l_idx_8, 12288i32, a3_) };
                            F::Dma_AddTexFlush(c);
                            l_p_5 = F::Dma_BeginDirect(c);
                            l_y1_11 = (if ((l_row_6) != 0) { (l_yy_9).wrapping_add(192i32) } else { (l_yy_9).wrapping_add(224i32) });
                            l_y0_10 = (if ((l_row_6) != 0) { (l_yy_9).wrapping_sub(32i32) } else { l_yy_9 });
                            l_y0_10 = ((l_y0_10).wrapping_shl((4i32) as u32)).wrapping_add(29184i32);
                            { let v_ = ((((((((1i32) as u64)) | ((((1i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((6i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (l_p_5).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                            { let r_ = (((1i32) as u64)).wrapping_shl((58i32) as u32); let a_ = (l_p_5).wrapping_add((((0i32) as u32)).wrapping_mul(8)); let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
                            { let v_ = ((5460742i32) as u64); let a_ = (l_p_5).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                            { let r_ = 2i32; let n_ = (l_p_5).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_5 = n_; };
                            { let v_ = (((({ let a_ = ((c.m.r32((p_res_1).wrapping_add(16))).wrapping_add((((l_idx_8) as u32)).wrapping_mul(64))).wrapping_add(48); c.m.r64(a_) }) | ((((({ let a_ = ((c.m.r32((p_res_1).wrapping_add(16))).wrapping_add((((l_idx_8) as u32)).wrapping_mul(64))).wrapping_add(16); (c.m.r32(a_) as i32) }).wrapping_add(12288i32)) as u64)).wrapping_shl((37i32) as u32)))) | ((((((4i32) as u64)).wrapping_shl((32i32) as u32)) | (((12288i32) as u64))))); let a_ = (l_p_5).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                            { let v_ = ((342i32) as u64); let a_ = (l_p_5).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                            { let r_ = 2i32; let n_ = (l_p_5).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_5 = n_; };
                            { let v_ = ((0i32) as u64); let a_ = (l_p_5).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                            { let v_ = (((((((p_x_2).wrapping_add((l_col_7).wrapping_mul(128i32))).wrapping_add(1792i32)).wrapping_shl((4i32) as u32)) as u64)) | ((((l_y0_10) as u64)).wrapping_shl((16i32) as u32))); let a_ = (l_p_5).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                            { let r_ = 2i32; let n_ = (l_p_5).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_5 = n_; };
                            { let v_ = (((((128i32).wrapping_shl((4i32) as u32)) as u64)) | (((((256i32).wrapping_shl((4i32) as u32)) as u64)).wrapping_shl((16i32) as u32))); let a_ = (l_p_5).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                            { let v_ = (((((((p_x_2).wrapping_add((l_col_7).wrapping_mul(128i32))).wrapping_add(1920i32)).wrapping_shl((4i32) as u32)) as u64)) | ((((((l_y1_11).wrapping_add(1824i32)).wrapping_shl((4i32) as u32)) as u64)).wrapping_shl((16i32) as u32))); let a_ = (l_p_5).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                            { let r_ = 2i32; let n_ = (l_p_5).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_5 = n_; };
                            F::Dma_EndDirect(c, l_p_5);
                        }
                        { let o_ = l_col_7; let n_ = (o_).wrapping_add(1); l_col_7 = n_; };
                    }
                }
                { let r_ = 256i32; let n_ = (l_yy_9).wrapping_add(r_); l_yy_9 = n_; };
            }
            { let o_ = l_row_6; let n_ = (o_).wrapping_add(1); l_row_6 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Sprite_DrawPicture", &PROBE_N, &[], true); }
    r_
}

pub fn Sprite_DrawList(c: &mut Ctx, mut p_res_1: u32, mut p_x_2: i32, mut p_y_3: i32, mut p_list_4: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Sprite_DrawList", &PROBE_N, &[(p_res_1) as u32, (p_x_2) as u32, (p_y_3) as u32, (p_list_4) as u32], false); }
    let mut l_p_5: u32 = 0;
    let mut l_i_6: i32 = 0;
    let mut l_spr_7: u32 = 0;
    let r_ = 'f: {
    if ((((((c.m.r32((p_list_4).wrapping_add(52)) as i32)) == (0i32)) as i32)) != 0) {
        break 'f;
    }
    F::Gfx_AddDefaultEnv(c);
    l_p_5 = F::Dma_BeginDirect(c);
    { let v_ = ((((((((3i32) as u64)) | ((((1i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (l_p_5).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((14i32) as u64); let a_ = (l_p_5).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_5).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_5 = n_; };
    { let v_ = ((68i32) as u64); let a_ = (l_p_5).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((66i32) as u64); let a_ = (l_p_5).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_5).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_5 = n_; };
    { let v_ = ((96i32) as u64); let a_ = (l_p_5).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((20i32) as u64); let a_ = (l_p_5).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_5).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_5 = n_; };
    { let v_ = ((196608i32) as u64); let a_ = (l_p_5).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((71i32) as u64); let a_ = (l_p_5).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_5).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_5 = n_; };
    F::Dma_EndDirect(c, l_p_5);
    {
        l_i_6 = 0i32;
        'l1: loop {
            if !((((!(((((c.m.r32((p_list_4).wrapping_add((((l_i_6) as u32)).wrapping_mul(56))) as i32)) & (2i32))) != 0)) as i32)) != 0) { break 'l1; }
            'c2: {
                l_spr_7 = (p_list_4).wrapping_add((((l_i_6) as u32)).wrapping_mul(56));
                if (((((c.m.r32(l_spr_7) as i32)) & (1i32))) != 0) {
                    if (((((p_res_1) != (((0i32) as u32))) as i32)) != 0) {
                        { let a1_ = (c.m.r32((l_spr_7).wrapping_add(36)) as i32); let a3_ = ({ let a_ = ((c.m.r32((p_res_1).wrapping_add(16))).wrapping_add(((((c.m.r32((l_spr_7).wrapping_add(36)) as i32)) as u32)).wrapping_mul(64))).wrapping_add(16); (c.m.r32(a_) as i32) }).wrapping_add(12352i32); F::TexFile_UploadOne(c, p_res_1, a1_, 12288i32, a3_) };
                        F::Dma_AddTexFlush(c);
                    }
                    l_p_5 = F::Dma_BeginDirect(c);
                    { let v_ = ((((((((1i32) as u64)) | ((((1i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((8i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (l_p_5).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                    { let v_ = ((1397950177i32) as u64); let a_ = (l_p_5).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                    { let r_ = 2i32; let n_ = (l_p_5).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_5 = n_; };
                    { let v_ = (((((((((c.m.r32((l_spr_7).wrapping_add(40)) as i32)) as u64)) | (((((c.m.r32((l_spr_7).wrapping_add(44)) as i32)) as u64)).wrapping_shl((8i32) as u32)))) | (((((c.m.r32((l_spr_7).wrapping_add(48)) as i32)) as u64)).wrapping_shl((16i32) as u32)))) | (((((c.m.r32((l_spr_7).wrapping_add(52)) as i32)) as u64)).wrapping_shl((24i32) as u32))); let a_ = (l_p_5).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                    { let v_ = ((0i32) as u64); let a_ = (l_p_5).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                    { let r_ = 2i32; let n_ = (l_p_5).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_5 = n_; };
                    if (((((((((c.m.r32(l_spr_7) as i32)) & (4i32))) != 0) || (((((p_res_1) == (((0i32) as u32))) as i32)) != 0)) as i32)) != 0) {
                        { let v_ = ((0i32) as u64); let a_ = (l_p_5).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                        { let v_ = ((70i32) as u64); let a_ = (l_p_5).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                    } else {
                        { let v_ = (((({ let a_ = ((c.m.r32((p_res_1).wrapping_add(16))).wrapping_add(((((c.m.r32((l_spr_7).wrapping_add(36)) as i32)) as u32)).wrapping_mul(64))).wrapping_add(48); c.m.r64(a_) }) | ((((({ let a_ = ((c.m.r32((p_res_1).wrapping_add(16))).wrapping_add(((((c.m.r32((l_spr_7).wrapping_add(36)) as i32)) as u32)).wrapping_mul(64))).wrapping_add(16); (c.m.r32(a_) as i32) }).wrapping_add(12352i32)) as u64)).wrapping_shl((37i32) as u32)))) | ((((((4i32) as u64)).wrapping_shl((32i32) as u32)) | (((12288i32) as u64))))); let a_ = (l_p_5).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                        { let v_ = ((342i32) as u64); let a_ = (l_p_5).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                    }
                    { let r_ = 2i32; let n_ = (l_p_5).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_5 = n_; };
                    { let v_ = ((((((c.m.r32((l_spr_7).wrapping_add(20)) as i32)).wrapping_shl((4i32) as u32)) as u64)) | ((((((c.m.r32((l_spr_7).wrapping_add(24)) as i32)).wrapping_shl((4i32) as u32)) as u64)).wrapping_shl((16i32) as u32))); let a_ = (l_p_5).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                    { let v_ = ((((((((c.m.r32((l_spr_7).wrapping_add(4)) as i32)).wrapping_add(p_x_2)).wrapping_shl((4i32) as u32)).wrapping_add(28672i32)) as u64)) | (((((({ let a0_ = ((c.m.r32((l_spr_7).wrapping_add(8)) as i32)).wrapping_add(p_y_3); F::Sprite_ConvY(c, a0_) }).wrapping_shl((4i32) as u32)).wrapping_add(29184i32)) as u64)).wrapping_shl((16i32) as u32))); let a_ = (l_p_5).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                    { let r_ = 2i32; let n_ = (l_p_5).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_5 = n_; };
                    { let v_ = ((((((c.m.r32((l_spr_7).wrapping_add(28)) as i32)).wrapping_shl((4i32) as u32)) as u64)) | ((((((c.m.r32((l_spr_7).wrapping_add(32)) as i32)).wrapping_shl((4i32) as u32)) as u64)).wrapping_shl((16i32) as u32))); let a_ = (l_p_5).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                    { let v_ = ((((((((c.m.r32((l_spr_7).wrapping_add(12)) as i32)).wrapping_add(p_x_2)).wrapping_shl((4i32) as u32)).wrapping_add(28672i32)) as u64)) | (((((({ let a0_ = ((c.m.r32((l_spr_7).wrapping_add(16)) as i32)).wrapping_add(p_y_3); F::Sprite_ConvY(c, a0_) }).wrapping_shl((4i32) as u32)).wrapping_add(29184i32)) as u64)).wrapping_shl((16i32) as u32))); let a_ = (l_p_5).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                    { let r_ = 2i32; let n_ = (l_p_5).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_5 = n_; };
                    F::Dma_EndDirect(c, l_p_5);
                }
            }
            { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Sprite_DrawList", &PROBE_N, &[], true); }
    r_
}

pub fn Sprite_Stub126B08(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Sprite_Stub126B08", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Sprite_Stub126B08", &PROBE_N, &[], true); }
    r_
}

pub fn Sprite_SetScissor(c: &mut Ctx, mut p_x0_1: i32, mut p_x1_2: i32, mut p_y0_3: i32, mut p_y1_4: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Sprite_SetScissor", &PROBE_N, &[(p_x0_1) as u32, (p_x1_2) as u32, (p_y0_3) as u32, (p_y1_4) as u32], false); }
    let mut l_bottom_5: i32 = 0;
    let mut l_right_6: i32 = 0;
    let r_ = 'f: {
    l_bottom_5 = (if (((((p_y1_4) == (448i32)) as i32)) != 0) { 447i32 } else { p_y1_4 });
    l_right_6 = (if (((((p_x1_2) == (512i32)) as i32)) != 0) { 511i32 } else { p_x1_2 });
    { let a2_ = F::Sprite_ConvY(c, p_y0_3); let a3_ = F::Sprite_ConvY(c, l_bottom_5); F::Dma_AddScissor(c, p_x0_1, l_right_6, a2_, a3_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Sprite_SetScissor", &PROBE_N, &[], true); }
    r_
}
