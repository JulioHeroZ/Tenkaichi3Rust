#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn EftLink_DrawQuad(c: &mut Ctx, mut p_corner_1: u32, p_uv0_2: u32, p_uv1_3: u32, p_color_4: u32, mut p_layer_5: i32, mut p_texIdx_6: i32, mut p_noDepth_7: i32, mut p_tex_8: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftLink_DrawQuad", &PROBE_N, &[(p_corner_1) as u32, (p_uv0_2) as u32, (p_uv1_3) as u32, (p_color_4) as u32, (p_layer_5) as u32, (p_texIdx_6) as u32, (p_noDepth_7) as u32, (p_tex_8) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(256) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 256);
    let mut l_p_9: u32 = 0;
    let mut l_e_10: u32 = 0;
    let mut l_abe_11: i32 = 0;
    let mut l_ctx_12: i32 = 0;
    let mut l_z_13: i32 = 0;
    let mut l_l_14: i32 = 0;
    rt::memcpy(c, fp, p_uv0_2, 16);
    rt::memcpy(c, fp.wrapping_add(16), p_uv1_3, 16);
    rt::memcpy(c, fp.wrapping_add(32), p_color_4, 16);
    let r_ = 'f: {
        l_abe_11 = 1i32;
        { let a1_ = F32(c.m.r32(fp)); let a2_ = F32(c.m.r32((fp).wrapping_add(4))); F::Vec4_Set(c, (fp.wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), a1_, a2_, F32(0x3f800000), F32(0x3f800000)) };
        { let a1_ = F32(c.m.r32((fp).wrapping_add(8))); let a2_ = F32(c.m.r32((fp).wrapping_add(12))); F::Vec4_Set(c, (fp.wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), a1_, a2_, F32(0x3f800000), F32(0x3f800000)) };
        { let a1_ = F32(c.m.r32(fp.wrapping_add(16))); let a2_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))); F::Vec4_Set(c, (fp.wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(16)), a1_, a2_, F32(0x3f800000), F32(0x3f800000)) };
        { let a1_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8))); let a2_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(12))); F::Vec4_Set(c, (fp.wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(16)), a1_, a2_, F32(0x3f800000), F32(0x3f800000)) };
        if ((((!((F::Vu0Cur_ProjectPointsStq(c, fp.wrapping_add(176), fp.wrapping_add(112), p_corner_1, fp.wrapping_add(48), 4i32)) != 0)) as i32)) != 0) {
            break 'f;
        }
        l_p_9 = c.m.r32(sym::gOtCur);
        { let v_ = (l_p_9).wrapping_add((((1i32) as u32)).wrapping_mul(144)); let a_ = sym::gOtCur; c.m.w32(a_, v_); };
        if (((((l_p_9) == (((0i32) as u32))) as i32)) != 0) {
            break 'f;
        }
        l_ctx_12 = (((p_layer_5) >= (2i32)) as i32);
        { let v_ = (((((((l_abe_11) as u64)).wrapping_shl((6i32) as u32)) | ((((l_ctx_12) as u64)).wrapping_shl((9i32) as u32)))) | (((28i32) as u64))); let a_ = (l_p_9).wrapping_add(32); c.m.w64(a_, v_); };
        { let v_ = ((536870920i32) as u32); let a_ = l_p_9; c.m.w32(a_, v_); };
        { let v_ = ((268435456i32) as u32); let a_ = (l_p_9).wrapping_add(8); c.m.w32(a_, v_); };
        { let v_ = ((1342177288i32) as u32); let a_ = (l_p_9).wrapping_add(12); c.m.w32(a_, v_); };
        { let v_ = 16429131440647602177u64; let a_ = (l_p_9).wrapping_add(16); c.m.w64(a_, v_); };
        { let v_ = (((18599481537995104i64).wrapping_add((((l_ctx_12).wrapping_shl((4i32) as u32)) as i64))) as u64); let a_ = (l_p_9).wrapping_add(24); c.m.w64(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = (l_p_9).wrapping_add(4); c.m.w32(a_, v_); };
        l_z_13 = (((((c.m.r32(((fp.wrapping_add(176)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)).wrapping_add((c.m.r32(((fp.wrapping_add(176)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32))).wrapping_add((c.m.r32(((fp.wrapping_add(176)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32))).wrapping_add((c.m.r32(((fp.wrapping_add(176)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32))).wrapping_shr((10i32) as u32);
        if ((p_noDepth_7) != 0) {
            { let v_ = 16777215i32; let a_ = ((fp.wrapping_add(176)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
            { let v_ = 16777215i32; let a_ = ((fp.wrapping_add(176)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
            { let v_ = 16777215i32; let a_ = ((fp.wrapping_add(176)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
            { let v_ = 16777215i32; let a_ = ((fp.wrapping_add(176)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        }
        F::Vec4_ToInt(c, fp.wrapping_add(240), fp.wrapping_add(32));
        { let v_ = (((c.m.r32(fp.wrapping_add(240)) as i32)) as u8); let a_ = ((l_p_9).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24)); c.m.w8(a_, v_); };
        { let v_ = (((c.m.r32((fp.wrapping_add(240)).wrapping_add(4)) as i32)) as u8); let a_ = (((l_p_9).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(1); c.m.w8(a_, v_); };
        { let v_ = (((c.m.r32((fp.wrapping_add(240)).wrapping_add(8)) as i32)) as u8); let a_ = (((l_p_9).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(2); c.m.w8(a_, v_); };
        { let v_ = (((c.m.r32((fp.wrapping_add(240)).wrapping_add(12)) as i32)) as u8); let a_ = (((l_p_9).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(3); c.m.w8(a_, v_); };
        { let v_ = F32(c.m.r32(((fp.wrapping_add(112)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8))); let a_ = (((l_p_9).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = (((c.m.r32(fp.wrapping_add(240)) as i32)) as u8); let a_ = ((l_p_9).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24)); c.m.w8(a_, v_); };
        { let v_ = (((c.m.r32((fp.wrapping_add(240)).wrapping_add(4)) as i32)) as u8); let a_ = (((l_p_9).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(1); c.m.w8(a_, v_); };
        { let v_ = (((c.m.r32((fp.wrapping_add(240)).wrapping_add(8)) as i32)) as u8); let a_ = (((l_p_9).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(2); c.m.w8(a_, v_); };
        { let v_ = (((c.m.r32((fp.wrapping_add(240)).wrapping_add(12)) as i32)) as u8); let a_ = (((l_p_9).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(3); c.m.w8(a_, v_); };
        { let v_ = F32(c.m.r32(((fp.wrapping_add(112)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8))); let a_ = (((l_p_9).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = (((c.m.r32(fp.wrapping_add(240)) as i32)) as u8); let a_ = ((l_p_9).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24)); c.m.w8(a_, v_); };
        { let v_ = (((c.m.r32((fp.wrapping_add(240)).wrapping_add(4)) as i32)) as u8); let a_ = (((l_p_9).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(1); c.m.w8(a_, v_); };
        { let v_ = (((c.m.r32((fp.wrapping_add(240)).wrapping_add(8)) as i32)) as u8); let a_ = (((l_p_9).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(2); c.m.w8(a_, v_); };
        { let v_ = (((c.m.r32((fp.wrapping_add(240)).wrapping_add(12)) as i32)) as u8); let a_ = (((l_p_9).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(3); c.m.w8(a_, v_); };
        { let v_ = F32(c.m.r32(((fp.wrapping_add(112)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8))); let a_ = (((l_p_9).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = (((c.m.r32(fp.wrapping_add(240)) as i32)) as u8); let a_ = ((l_p_9).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24)); c.m.w8(a_, v_); };
        { let v_ = (((c.m.r32((fp.wrapping_add(240)).wrapping_add(4)) as i32)) as u8); let a_ = (((l_p_9).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(1); c.m.w8(a_, v_); };
        { let v_ = (((c.m.r32((fp.wrapping_add(240)).wrapping_add(8)) as i32)) as u8); let a_ = (((l_p_9).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(2); c.m.w8(a_, v_); };
        { let v_ = (((c.m.r32((fp.wrapping_add(240)).wrapping_add(12)) as i32)) as u8); let a_ = (((l_p_9).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(3); c.m.w8(a_, v_); };
        { let v_ = F32(c.m.r32(((fp.wrapping_add(112)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(8))); let a_ = (((l_p_9).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((fp.wrapping_add(112)).wrapping_add((((0i32) as u32)).wrapping_mul(16)))); let a_ = (((l_p_9).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32(((fp.wrapping_add(112)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4))); let a_ = ((((l_p_9).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((fp.wrapping_add(112)).wrapping_add((((1i32) as u32)).wrapping_mul(16)))); let a_ = (((l_p_9).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32(((fp.wrapping_add(112)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(4))); let a_ = ((((l_p_9).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((fp.wrapping_add(112)).wrapping_add((((2i32) as u32)).wrapping_mul(16)))); let a_ = (((l_p_9).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32(((fp.wrapping_add(112)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(4))); let a_ = ((((l_p_9).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((fp.wrapping_add(112)).wrapping_add((((3i32) as u32)).wrapping_mul(16)))); let a_ = (((l_p_9).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32(((fp.wrapping_add(112)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(4))); let a_ = ((((l_p_9).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = (((c.m.r32((fp.wrapping_add(176)).wrapping_add((((0i32) as u32)).wrapping_mul(16))) as i32)) as u64); let a_ = (((l_p_9).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(16); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = (((c.m.r32(((fp.wrapping_add(176)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4)) as i32)) as u64); let a_ = ((((l_p_9).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = (((c.m.r32(((fp.wrapping_add(176)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)) as u64); let a_ = ((((l_p_9).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
        { let v_ = ((255i32) as u64); let a_ = ((((l_p_9).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
        { let v_ = (((c.m.r32((fp.wrapping_add(176)).wrapping_add((((1i32) as u32)).wrapping_mul(16))) as i32)) as u64); let a_ = (((l_p_9).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(16); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = (((c.m.r32(((fp.wrapping_add(176)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(4)) as i32)) as u64); let a_ = ((((l_p_9).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = (((c.m.r32(((fp.wrapping_add(176)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)) as u64); let a_ = ((((l_p_9).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
        { let v_ = ((255i32) as u64); let a_ = ((((l_p_9).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
        { let v_ = (((c.m.r32((fp.wrapping_add(176)).wrapping_add((((2i32) as u32)).wrapping_mul(16))) as i32)) as u64); let a_ = (((l_p_9).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(16); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = (((c.m.r32(((fp.wrapping_add(176)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(4)) as i32)) as u64); let a_ = ((((l_p_9).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = (((c.m.r32(((fp.wrapping_add(176)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)) as u64); let a_ = ((((l_p_9).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
        { let v_ = ((255i32) as u64); let a_ = ((((l_p_9).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
        { let v_ = (((c.m.r32((fp.wrapping_add(176)).wrapping_add((((3i32) as u32)).wrapping_mul(16))) as i32)) as u64); let a_ = (((l_p_9).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(16); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = (((c.m.r32(((fp.wrapping_add(176)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(4)) as i32)) as u64); let a_ = ((((l_p_9).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = (((c.m.r32(((fp.wrapping_add(176)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)) as u64); let a_ = ((((l_p_9).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
        { let v_ = ((255i32) as u64); let a_ = ((((l_p_9).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
        { let v_ = c.m.r64((p_tex_8).wrapping_add((((p_texIdx_6).wrapping_shl((4i32) as u32)) as u32))); let a_ = (l_p_9).wrapping_add(40); c.m.w64(a_, v_); };
        l_l_14 = p_layer_5;
        if (((((l_l_14) >= (2i32)) as i32)) != 0) {
            { let r_ = 2i32; let n_ = (l_l_14).wrapping_sub(r_); l_l_14 = n_; };
        }
        if (((((l_z_13) < (0i32)) as i32)) != 0) {
            l_e_10 = ((c.m.r32(sym::gOtZ)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((l_l_14) as u32)).wrapping_mul(8));
        } else {
            if (((((l_z_13) >= (4096i32)) as i32)) != 0) {
                l_e_10 = ((c.m.r32(sym::gOtZ)).wrapping_add((((4095i32) as u32)).wrapping_mul(16))).wrapping_add((((l_l_14) as u32)).wrapping_mul(8));
            } else {
                l_e_10 = ((c.m.r32(sym::gOtZ)).wrapping_add((((l_z_13) as u32)).wrapping_mul(16))).wrapping_add((((l_l_14) as u32)).wrapping_mul(8));
            }
        }
        { let v_ = l_p_9; let a_ = (c.m.r32((l_e_10).wrapping_add(4))).wrapping_add(4); c.m.w32(a_, v_); };
        { let v_ = l_p_9; let a_ = (l_e_10).wrapping_add(4); c.m.w32(a_, v_); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftLink_DrawQuad", &PROBE_N, &[], true); }
    r_
}

pub fn EftLink_DrawQuadClipped(c: &mut Ctx, mut p_corner_1: u32, p_uv0_2: u32, p_uv1_3: u32, p_color_4: u32, mut p_layer_5: i32, mut p_texIdx_6: i32, mut p_noDepth_7: i32, mut p_tex_8: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftLink_DrawQuadClipped", &PROBE_N, &[(p_corner_1) as u32, (p_uv0_2) as u32, (p_uv1_3) as u32, (p_color_4) as u32, (p_layer_5) as u32, (p_texIdx_6) as u32, (p_noDepth_7) as u32, (p_tex_8) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(608) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 608);
    rt::memcpy(c, fp, p_uv0_2, 16);
    rt::memcpy(c, fp.wrapping_add(16), p_uv1_3, 16);
    rt::memcpy(c, fp.wrapping_add(32), p_color_4, 16);
    let r_ = 'f: {
        { let a1_ = F32(c.m.r32(fp)); let a2_ = F32(c.m.r32((fp).wrapping_add(4))); F::Vec4_Set(c, (fp.wrapping_add(112)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), a1_, a2_, F32(0x3f800000), F32(0x3f800000)) };
        { let a1_ = F32(c.m.r32((fp).wrapping_add(8))); let a2_ = F32(c.m.r32((fp).wrapping_add(12))); F::Vec4_Set(c, (fp.wrapping_add(112)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), a1_, a2_, F32(0x3f800000), F32(0x3f800000)) };
        { let a1_ = F32(c.m.r32(fp.wrapping_add(16))); let a2_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))); F::Vec4_Set(c, (fp.wrapping_add(112)).wrapping_add((((2i32) as u32)).wrapping_mul(16)), a1_, a2_, F32(0x3f800000), F32(0x3f800000)) };
        { let a1_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8))); let a2_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(12))); F::Vec4_Set(c, (fp.wrapping_add(112)).wrapping_add((((3i32) as u32)).wrapping_mul(16)), a1_, a2_, F32(0x3f800000), F32(0x3f800000)) };
        F::Vec4_Copy(c, (fp.wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (p_corner_1).wrapping_add((((0i32) as u32)).wrapping_mul(16)));
        F::Vec4_Copy(c, (fp.wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), (p_corner_1).wrapping_add((((1i32) as u32)).wrapping_mul(16)));
        F::Vec4_Copy(c, (fp.wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(16)), (p_corner_1).wrapping_add((((2i32) as u32)).wrapping_mul(16)));
        F::Vec4_Copy(c, (fp.wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(16)), (p_corner_1).wrapping_add((((3i32) as u32)).wrapping_mul(16)));
        F::ClipVtx_Set(c, (fp.wrapping_add(176)).wrapping_add((((0i32) as u32)).wrapping_mul(48)), (fp.wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(112)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), fp.wrapping_add(32));
        F::ClipVtx_Set(c, (fp.wrapping_add(176)).wrapping_add((((1i32) as u32)).wrapping_mul(48)), (fp.wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(112)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), fp.wrapping_add(32));
        F::ClipVtx_Set(c, (fp.wrapping_add(176)).wrapping_add((((2i32) as u32)).wrapping_mul(48)), (fp.wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(112)).wrapping_add((((2i32) as u32)).wrapping_mul(16)), fp.wrapping_add(32));
        { let a6_ = c.m.r64((p_tex_8).wrapping_add((((p_texIdx_6).wrapping_shl((4i32) as u32)) as u32))); F::EftGfx_DrawPolyScaledZ(c, fp.wrapping_add(176), p_layer_5, 1i32, 0i32, p_noDepth_7, 0i32, a6_, F32(0x40000000)) };
        F::ClipVtx_Set(c, (fp.wrapping_add(176)).wrapping_add((((0i32) as u32)).wrapping_mul(48)), (fp.wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(112)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), fp.wrapping_add(32));
        F::ClipVtx_Set(c, (fp.wrapping_add(176)).wrapping_add((((1i32) as u32)).wrapping_mul(48)), (fp.wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(112)).wrapping_add((((2i32) as u32)).wrapping_mul(16)), fp.wrapping_add(32));
        F::ClipVtx_Set(c, (fp.wrapping_add(176)).wrapping_add((((2i32) as u32)).wrapping_mul(48)), (fp.wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(112)).wrapping_add((((3i32) as u32)).wrapping_mul(16)), fp.wrapping_add(32));
        { let a6_ = c.m.r64((p_tex_8).wrapping_add((((p_texIdx_6).wrapping_shl((4i32) as u32)) as u32))); F::EftGfx_DrawPolyScaledZ(c, fp.wrapping_add(176), p_layer_5, 1i32, 0i32, p_noDepth_7, 0i32, a6_, F32(0x40000000)) };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftLink_DrawQuadClipped", &PROBE_N, &[], true); }
    r_
}

pub fn EftLink_SetKey(c: &mut Ctx, mut p_w_1: u32, mut p_i_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftLink_SetKey", &PROBE_N, &[(p_w_1) as u32, (p_i_2) as u32], false); }
    let mut l_def_3: u32 = 0;
    let mut l_key_4: u32 = 0;
    let r_ = 'f: {
    l_def_3 = c.m.r32((p_w_1).wrapping_add(48));
    { let v_ = F32(c.m.r32((l_def_3).wrapping_add((((p_i_2) as u32)).wrapping_mul(4)))); let a_ = (p_w_1).wrapping_add(328); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32(((l_def_3).wrapping_add(12)).wrapping_add((((p_i_2) as u32)).wrapping_mul(4)))); let a_ = (p_w_1).wrapping_add(336); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32(((l_def_3).wrapping_add(24)).wrapping_add((((p_i_2) as u32)).wrapping_mul(4)))); let a_ = (p_w_1).wrapping_add(344); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32(((l_def_3).wrapping_add(36)).wrapping_add((((p_i_2) as u32)).wrapping_mul(4)))); let a_ = (p_w_1).wrapping_add(352); c.m.w32(a_, (v_).0); };
    l_key_4 = c.m.r32(((p_w_1).wrapping_add(48)).wrapping_add(4));
    { let v_ = F32(c.m.r32(((l_def_3).wrapping_add(48)).wrapping_add((((p_i_2) as u32)).wrapping_mul(4)))); let a_ = (p_w_1).wrapping_add(360); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32(((l_def_3).wrapping_add(60)).wrapping_add((((p_i_2) as u32)).wrapping_mul(4)))); let a_ = (p_w_1).wrapping_add(368); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32(((l_def_3).wrapping_add(72)).wrapping_add((((p_i_2) as u32)).wrapping_mul(4)))); let a_ = (p_w_1).wrapping_add(376); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32(((l_def_3).wrapping_add(84)).wrapping_add((((p_i_2) as u32)).wrapping_mul(4)))); let a_ = (p_w_1).wrapping_add(384); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32(((l_def_3).wrapping_add(96)).wrapping_add((((p_i_2) as u32)).wrapping_mul(4)))); let a_ = (p_w_1).wrapping_add(392); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32(((l_def_3).wrapping_add(108)).wrapping_add((((p_i_2) as u32)).wrapping_mul(4)))); let a_ = (p_w_1).wrapping_add(400); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32(((l_def_3).wrapping_add(120)).wrapping_add((((p_i_2) as u32)).wrapping_mul(4)))); let a_ = (p_w_1).wrapping_add(408); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32(((l_def_3).wrapping_add(132)).wrapping_add((((p_i_2) as u32)).wrapping_mul(4)))); let a_ = (p_w_1).wrapping_add(416); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32(((l_def_3).wrapping_add(144)).wrapping_add((((p_i_2) as u32)).wrapping_mul(4)))); let a_ = (p_w_1).wrapping_add(424); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32(((l_def_3).wrapping_add(156)).wrapping_add((((p_i_2) as u32)).wrapping_mul(4)))); let a_ = (p_w_1).wrapping_add(432); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32(((l_def_3).wrapping_add(168)).wrapping_add((((p_i_2) as u32)).wrapping_mul(4)))); let a_ = (p_w_1).wrapping_add(440); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32(((l_def_3).wrapping_add(180)).wrapping_add((((p_i_2) as u32)).wrapping_mul(4)))); let a_ = (p_w_1).wrapping_add(448); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((((l_def_3).wrapping_add(192)).wrapping_add((((p_i_2) as u32)).wrapping_mul(12))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = ((p_w_1).wrapping_add(456)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((((l_def_3).wrapping_add(192)).wrapping_add((((p_i_2) as u32)).wrapping_mul(12))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a_ = ((p_w_1).wrapping_add(456)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((((l_def_3).wrapping_add(192)).wrapping_add((((p_i_2) as u32)).wrapping_mul(12))).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); let a_ = ((p_w_1).wrapping_add(456)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((((l_def_3).wrapping_add(228)).wrapping_add((((p_i_2) as u32)).wrapping_mul(12))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = ((p_w_1).wrapping_add(480)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((((l_def_3).wrapping_add(228)).wrapping_add((((p_i_2) as u32)).wrapping_mul(12))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a_ = ((p_w_1).wrapping_add(480)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((((l_def_3).wrapping_add(228)).wrapping_add((((p_i_2) as u32)).wrapping_mul(12))).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); let a_ = ((p_w_1).wrapping_add(480)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32(((l_def_3).wrapping_add(264)).wrapping_add((((p_i_2) as u32)).wrapping_mul(4)))); let a_ = (p_w_1).wrapping_add(504); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32(((l_def_3).wrapping_add(276)).wrapping_add((((p_i_2) as u32)).wrapping_mul(4)))); let a_ = (p_w_1).wrapping_add(512); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32(((l_def_3).wrapping_add(288)).wrapping_add((((p_i_2) as u32)).wrapping_mul(4)))); let a_ = (p_w_1).wrapping_add(520); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32(((l_def_3).wrapping_add(300)).wrapping_add((((p_i_2) as u32)).wrapping_mul(4)))); let a_ = (p_w_1).wrapping_add(528); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((((l_def_3).wrapping_add(312)).wrapping_add((((p_i_2) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = ((p_w_1).wrapping_add(536)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((((l_def_3).wrapping_add(312)).wrapping_add((((p_i_2) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a_ = ((p_w_1).wrapping_add(536)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32(((l_def_3).wrapping_add(336)).wrapping_add((((p_i_2) as u32)).wrapping_mul(4)))); let a_ = (p_w_1).wrapping_add(552); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((((l_key_4).wrapping_add(192)).wrapping_add((((p_i_2) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = ((p_w_1).wrapping_add(256)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((((l_key_4).wrapping_add(192)).wrapping_add((((p_i_2) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a_ = ((p_w_1).wrapping_add(256)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((((l_key_4).wrapping_add(216)).wrapping_add((((p_i_2) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = ((p_w_1).wrapping_add(272)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((((l_key_4).wrapping_add(216)).wrapping_add((((p_i_2) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a_ = ((p_w_1).wrapping_add(272)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((((l_key_4).wrapping_add(240)).wrapping_add((((p_i_2) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = ((p_w_1).wrapping_add(288)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((((l_key_4).wrapping_add(240)).wrapping_add((((p_i_2) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a_ = ((p_w_1).wrapping_add(288)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32(((l_key_4).wrapping_add(264)).wrapping_add((((p_i_2) as u32)).wrapping_mul(4)))); let a_ = (p_w_1).wrapping_add(304); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32(((l_key_4).wrapping_add(276)).wrapping_add((((p_i_2) as u32)).wrapping_mul(4)))); let a_ = (p_w_1).wrapping_add(312); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32(((l_key_4).wrapping_add(288)).wrapping_add((((p_i_2) as u32)).wrapping_mul(4)))); let a_ = (p_w_1).wrapping_add(320); c.m.w32(a_, (v_).0); };
    F::Vec4_Copy(c, (p_w_1).wrapping_add(128), (l_key_4).wrapping_add((((p_i_2) as u32)).wrapping_mul(16)));
    F::Vec4_Copy(c, (p_w_1).wrapping_add(192), ((l_key_4).wrapping_add(96)).wrapping_add((((p_i_2) as u32)).wrapping_mul(16)));
    F::Vec4_Copy(c, (p_w_1).wrapping_add(160), ((l_key_4).wrapping_add(48)).wrapping_add((((p_i_2) as u32)).wrapping_mul(16)));
    F::Vec4_Copy(c, (p_w_1).wrapping_add(224), ((l_key_4).wrapping_add(144)).wrapping_add((((p_i_2) as u32)).wrapping_mul(16)));
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftLink_SetKey", &PROBE_N, &[], true); }
    r_
}

pub fn EftLink_SelectTex(c: &mut Ctx, mut p_w_1: u32, mut p_tbl_2: u32, mut p_a_3: i32, mut p_b_4: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftLink_SelectTex", &PROBE_N, &[(p_w_1) as u32, (p_tbl_2) as u32, (p_a_3) as u32, (p_b_4) as u32], false); }
    let r_ = 'f: {
    if (((((p_a_3) == (p_b_4)) as i32)) != 0) {
        { let v_ = ((p_b_4) as u8); let a_ = (p_w_1).wrapping_add(916); c.m.w8(a_, v_); };
    } else {
        { let v_ = (((p_a_3).wrapping_add(p_b_4)) as u8); let a_ = (p_w_1).wrapping_add(916); c.m.w8(a_, v_); };
    }
    { let s_ = (p_tbl_2).wrapping_add((((p_a_3) as u32)).wrapping_mul(16)); let d_ = (p_w_1).wrapping_add(8); rt::memcpy(c, d_, s_, 16); };
    { let s_ = (p_tbl_2).wrapping_add((((p_b_4) as u32)).wrapping_mul(16)); let d_ = (p_w_1).wrapping_add(24); rt::memcpy(c, d_, s_, 16); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftLink_SelectTex", &PROBE_N, &[], true); }
    r_
}

pub fn EftLink_BuildTex(c: &mut Ctx, mut p_w_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftLink_BuildTex", &PROBE_N, &[(p_w_1) as u32], false); }
    let mut l_arg_2: u32 = 0;
    let mut l_t_3: u64 = 0;
    let r_ = 'f: {
    l_arg_2 = (p_w_1).wrapping_add(48);
    if (((((c.m.r32((l_arg_2).wrapping_add(8))) != (((0i32) as u32))) as i32)) != 0) {
        if ((((!(((({ let a_ = (c.m.r32((l_arg_2).wrapping_add(8))).wrapping_add(260); c.m.r32(a_) }) & ((((1i32).wrapping_shl((((c.m.r8((p_w_1).wrapping_add(916))) as i32)) as u32)) as u32)))) != 0)) as i32)) != 0) {
            l_t_3 = F::EftVram_AddImage(c, (p_w_1).wrapping_add(8), 1i32, 0i32);
            { let r_ = (F::EftVram_AddClut(c, (p_w_1).wrapping_add(24))).wrapping_shl((37i32) as u32); let n_ = ((l_t_3) | (r_)); l_t_3 = n_; };
            { let v_ = l_t_3; let a_ = (c.m.r32((l_arg_2).wrapping_add(8))).wrapping_add((((c.m.r8((p_w_1).wrapping_add(916))) as u32)).wrapping_mul(16)); c.m.w64(a_, v_); };
            { let r_ = (((1i32).wrapping_shl((((c.m.r8((p_w_1).wrapping_add(916))) as i32)) as u32)) as u32); let a_ = (c.m.r32((l_arg_2).wrapping_add(8))).wrapping_add(260); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftLink_BuildTex", &PROBE_N, &[], true); }
    r_
}

pub fn EftLink_NewNode(c: &mut Ctx, mut p_w_1: u32, p_pos_2: u32, mut p_t_3: F32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftLink_NewNode", &PROBE_N, &[(p_w_1) as u32, (p_pos_2) as u32, (p_t_3).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_p_4: u32 = 0;
    let mut l_i_5: u8 = 0;
    rt::memcpy(c, fp, p_pos_2, 16);
    let r_ = 'f: {
        if ((((({ let a_ = (c.m.r32(sym::gEftLink)).wrapping_add(67216); (c.m.r32(a_) as i32) }) >= (200i32)) as i32)) != 0) {
            { let v_ = 0i32; let a_ = (c.m.r32(sym::gEftLink)).wrapping_add(67216); c.m.w32(a_, (v_) as u32); };
        }
        l_i_5 = (({ let a_ = (c.m.r32(sym::gEftLink)).wrapping_add(67216); (c.m.r32(a_) as i32) }) as u8);
        'l1: loop {
            'c2: {
                l_p_4 = ((c.m.r32(sym::gEftLink)).wrapping_add(16)).wrapping_add((((l_i_5) as u32)).wrapping_mul(336));
                { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
                if (((((((l_i_5) as i32)) >= (200i32)) as i32)) != 0) {
                    l_i_5 = ((0i32) as u8);
                }
                if ((((((c.m.r32((l_p_4).wrapping_add(320)) as i32)) == (0i32)) as i32)) != 0) {
                    F::EftLink_InitNode(c, l_p_4, p_w_1);
                    F::Vec4_Copy(c, (l_p_4).wrapping_add(96), fp);
                    { let v_ = { let a0_ = rt::fmul(rt::fmul(F32(c.m.r32((p_w_1).wrapping_add(392))), F32(0x40c90fda)), p_t_3); F::EftMath_WrapAngle(c, a0_) }; let a_ = (l_p_4).wrapping_add(216); c.m.w32(a_, (v_).0); };
                    { let v_ = ((0i32) as u32); let a_ = (l_p_4).wrapping_add(324); c.m.w32(a_, v_); };
                    { let v_ = ((0i32) as u32); let a_ = (l_p_4).wrapping_add(328); c.m.w32(a_, v_); };
                    { let r_ = 1i32; let a_ = (l_p_4).wrapping_add(320); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    F::EftLink_LinkNode(c, (p_w_1).wrapping_add(924), (p_w_1).wrapping_add(928), l_p_4);
                    { let v_ = ((l_i_5) as i32); let a_ = (c.m.r32(sym::gEftLink)).wrapping_add(67216); c.m.w32(a_, (v_) as u32); };
                    break 'f l_p_4;
                }
            }
            if !(((((((l_i_5) as i32)) != ({ let a_ = (c.m.r32(sym::gEftLink)).wrapping_add(67216); (c.m.r32(a_) as i32) })) as i32)) != 0) { break 'l1; }
        }
        break 'f ((0i32) as u32);
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftLink_NewNode", &PROBE_N, &[], true); }
    r_
}

pub fn EftLink_InitNode(c: &mut Ctx, mut p_p_1: u32, mut p_w_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftLink_InitNode", &PROBE_N, &[(p_p_1) as u32, (p_w_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_def_3: u32 = 0;
    let mut l_flip_4: i32 = 0;
    let mut l_a_5: F32 = F32(0);
    let mut l_r_6: F32 = F32(0);
    let r_ = 'f: {
        l_def_3 = c.m.r32((p_w_2).wrapping_add(48));
        l_flip_4 = 1i32;
        l_r_6 = F32(0x00000000);
        F::Vec4_Set(c, (p_p_1).wrapping_add(112), F32(0x00000000), F32(0x00000000), F32(0x3f800000), F32(0x3f800000));
        { let a1_ = F32(c.m.r32((p_w_2).wrapping_add(400))); let a2_ = F32(c.m.r32((p_w_2).wrapping_add(408))); F::Vec4_Set(c, (p_p_1).wrapping_add(64), a1_, a2_, F32(0x00000000), F32(0x3f800000)) };
        l_a_5 = { let a0_ = F32(c.m.r32((p_w_2).wrapping_add(328))); let a1_ = rt::fadd(F32(c.m.r32((p_w_2).wrapping_add(328))), F32(c.m.r32((p_w_2).wrapping_add(336)))); F::Rand_FloatRange(c, a0_, a1_) };
        { let v_ = F::EftMath_WrapAngle(c, rt::fmul(l_a_5, F32(0x40c90fda))); let a_ = (p_p_1).wrapping_add(208); c.m.w32(a_, (v_).0); };
        l_a_5 = { let a0_ = F32(c.m.r32((p_w_2).wrapping_add(344))); let a1_ = rt::fadd(F32(c.m.r32((p_w_2).wrapping_add(344))), F32(c.m.r32((p_w_2).wrapping_add(352)))); F::Rand_FloatRange(c, a0_, a1_) };
        { let v_ = F::EftMath_WrapAngle(c, rt::fmul(l_a_5, F32(0x40c90fda))); let a_ = (p_p_1).wrapping_add(212); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (p_p_1).wrapping_add(216); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = rt::fmul({ let a0_ = F32(c.m.r32((p_w_2).wrapping_add(376))); let a1_ = rt::fadd(F32(c.m.r32((p_w_2).wrapping_add(376))), F32(c.m.r32((p_w_2).wrapping_add(384)))); F::Rand_FloatRange(c, a0_, a1_) }, F32(0x40490fda)); F::EftMath_WrapAngle(c, a0_) }; let a_ = (p_p_1).wrapping_add(220); c.m.w32(a_, (v_).0); };
        if (((((c.m.r32((l_def_3).wrapping_add(416)) as i32)) & (256i32))) != 0) {
            if (((((((rt::flt(F32(c.m.r32((p_w_2).wrapping_add(376))), F32(0x00000000)) as i32)) != 0) && (((rt::flt(F32(0x00000000), rt::fadd(F32(c.m.r32((p_w_2).wrapping_add(376))), F32(c.m.r32((p_w_2).wrapping_add(384))))) as i32)) != 0)) as i32)) != 0) {
                l_flip_4 = 0i32;
            }
            if (((((((rt::flt(F32(0x00000000), F32(c.m.r32((p_w_2).wrapping_add(376)))) as i32)) != 0) && (((rt::flt(rt::fadd(F32(c.m.r32((p_w_2).wrapping_add(376))), F32(c.m.r32((p_w_2).wrapping_add(384)))), F32(0x00000000)) as i32)) != 0)) as i32)) != 0) {
                l_flip_4 = 0i32;
            }
            if ((((((((!((((F::Rand_IntRange(c, 0i32, 1024i32)) & (1i32))) != 0)) as i32)) != 0) && ((l_flip_4) != 0)) as i32)) != 0) {
                { let v_ = rt::fneg(F32(c.m.r32((p_p_1).wrapping_add(220)))); let a_ = (p_p_1).wrapping_add(220); c.m.w32(a_, (v_).0); };
            }
        }
        l_a_5 = { let a0_ = rt::fmul({ let a0_ = F32(c.m.r32((p_w_2).wrapping_add(360))); let a1_ = rt::fadd(F32(c.m.r32((p_w_2).wrapping_add(360))), F32(c.m.r32((p_w_2).wrapping_add(368)))); F::Rand_FloatRange(c, a0_, a1_) }, F32(0x40c90fda)); F::EftMath_WrapAngle(c, a0_) };
        l_r_6 = rt::fadd(F32(c.m.r32((p_w_2).wrapping_add(448))), { let a0_ = F32(c.m.r32((p_w_2).wrapping_add(432))); let a1_ = rt::fadd(F32(c.m.r32((p_w_2).wrapping_add(432))), F32(c.m.r32((p_w_2).wrapping_add(440)))); F::Rand_FloatRange(c, a0_, a1_) });
        { let v_ = rt::fmul(F::Port_cosf(c, l_a_5), l_r_6); let a_ = (p_p_1).wrapping_add(80); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(F::Port_sinf(c, l_a_5), l_r_6); let a_ = ((p_p_1).wrapping_add(80)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = ((p_p_1).wrapping_add(80)).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = ((p_p_1).wrapping_add(80)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        if (((((((((c.m.r32((l_def_3).wrapping_add(416)) as i32)) & (8i32))) != 0) && (((rt::flt(F32(c.m.r32((p_p_1).wrapping_add(220))), F32(c.m.r32(((p_p_1).wrapping_add(80)).wrapping_add(8)))) as i32)) != 0)) as i32)) != 0) {
            F::Vec4_Set(c, (p_p_1).wrapping_add(176), F32(0x3f7c0000), F32(0x3c800000), F32(0x3c800000), F32(0x3c800000));
            F::Vec4_Set(c, (p_p_1).wrapping_add(192), F32(0x3f7c0000), F32(0x3f7c0000), F32(0x3c800000), F32(0x3f7c0000));
            { let r_ = 128i32; let a_ = (p_p_1).wrapping_add(320); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        } else {
            F::Vec4_Set(c, (p_p_1).wrapping_add(176), F32(0x3c800000), F32(0x3c800000), F32(0x3c800000), F32(0x3f7c0000));
            F::Vec4_Set(c, (p_p_1).wrapping_add(192), F32(0x3f7c0000), F32(0x3c800000), F32(0x3f7c0000), F32(0x3f7c0000));
        }
        if (((((c.m.r32((l_def_3).wrapping_add(416)) as i32)) & (128i32))) != 0) {
            { let v_ = rt::i2f({ let a1_ = rt::f2i(rt::fsub(F32(c.m.r32((p_w_2).wrapping_add(596))), F32(0x3f800000))); F::Rand_IntRange(c, 0i32, a1_) }); let a_ = (p_p_1).wrapping_add(312); c.m.w32(a_, (v_).0); };
        }
        { let v_ = rt::fmul({ let a0_ = F32(c.m.r32((p_w_2).wrapping_add(504))); let a1_ = rt::fadd(F32(c.m.r32((p_w_2).wrapping_add(504))), F32(c.m.r32((p_w_2).wrapping_add(512)))); F::Rand_FloatRange(c, a0_, a1_) }, F32(0x41f00000)); let a_ = (p_p_1).wrapping_add(248); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (p_p_1).wrapping_add(244); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul({ let a0_ = F32(c.m.r32((p_w_2).wrapping_add(520))); let a1_ = rt::fadd(F32(c.m.r32((p_w_2).wrapping_add(520))), F32(c.m.r32((p_w_2).wrapping_add(528)))); F::Rand_FloatRange(c, a0_, a1_) }, F32(0x41f00000)); let a_ = (p_p_1).wrapping_add(316); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(F32(c.m.r32((p_p_1).wrapping_add(248))), F32(c.m.r32((l_def_3).wrapping_add(356)))); let a_ = (p_p_1).wrapping_add(236); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(F32(c.m.r32((p_p_1).wrapping_add(248))), rt::fsub(F32(0x3f800000), F32(c.m.r32((l_def_3).wrapping_add(360))))); let a_ = (p_p_1).wrapping_add(240); c.m.w32(a_, (v_).0); };
        { let v_ = 0i32; let a_ = (p_p_1).wrapping_add(320); c.m.w32(a_, (v_) as u32); };
        if ((((!(((((c.m.r32((l_def_3).wrapping_add(416)) as i32)) & (32i32))) != 0)) as i32)) != 0) {
            { let v_ = 2048i32; let a_ = (p_p_1).wrapping_add(320); c.m.w32(a_, (v_) as u32); };
        } else {
            if (((((c.m.r32((l_def_3).wrapping_add(416)) as i32)) & (64i32))) != 0) {
                if ((((!((((F::Port_Rand(c)) & (1i32))) != 0)) as i32)) != 0) {
                    { let r_ = 2048i32; let a_ = (p_p_1).wrapping_add(320); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                }
            }
        }
        { let v_ = rt::fmul(F32(c.m.r32((p_p_1).wrapping_add(248))), rt::fsub(F32(c.m.r32((p_w_2).wrapping_add(320))), F32(c.m.r32((p_w_2).wrapping_add(312))))); let a_ = (p_p_1).wrapping_add(308); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (p_p_1).wrapping_add(304); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32((p_w_2).wrapping_add(128))); let a1_ = rt::fadd(F32(c.m.r32((p_w_2).wrapping_add(128))), F32(c.m.r32((p_w_2).wrapping_add(160)))); F::Rand_FloatRange(c, a0_, a1_) }; let a_ = (p_p_1).wrapping_add(144); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32(((p_w_2).wrapping_add(128)).wrapping_add(4))); let a1_ = rt::fadd(F32(c.m.r32(((p_w_2).wrapping_add(128)).wrapping_add(4))), F32(c.m.r32(((p_w_2).wrapping_add(160)).wrapping_add(4)))); F::Rand_FloatRange(c, a0_, a1_) }; let a_ = ((p_p_1).wrapping_add(144)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32(((p_w_2).wrapping_add(128)).wrapping_add(8))); let a1_ = rt::fadd(F32(c.m.r32(((p_w_2).wrapping_add(128)).wrapping_add(8))), F32(c.m.r32(((p_w_2).wrapping_add(160)).wrapping_add(8)))); F::Rand_FloatRange(c, a0_, a1_) }; let a_ = ((p_p_1).wrapping_add(144)).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32(((p_w_2).wrapping_add(128)).wrapping_add(12))); let a1_ = rt::fadd(F32(c.m.r32(((p_w_2).wrapping_add(128)).wrapping_add(12))), F32(c.m.r32(((p_w_2).wrapping_add(160)).wrapping_add(12)))); F::Rand_FloatRange(c, a0_, a1_) }; let a_ = ((p_p_1).wrapping_add(144)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32((p_w_2).wrapping_add(192))); let a1_ = rt::fadd(F32(c.m.r32((p_w_2).wrapping_add(192))), F32(c.m.r32((p_w_2).wrapping_add(224)))); F::Rand_FloatRange(c, a0_, a1_) }; let a_ = fp; c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32(((p_w_2).wrapping_add(192)).wrapping_add(4))); let a1_ = rt::fadd(F32(c.m.r32(((p_w_2).wrapping_add(192)).wrapping_add(4))), F32(c.m.r32(((p_w_2).wrapping_add(224)).wrapping_add(4)))); F::Rand_FloatRange(c, a0_, a1_) }; let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32(((p_w_2).wrapping_add(192)).wrapping_add(8))); let a1_ = rt::fadd(F32(c.m.r32(((p_w_2).wrapping_add(192)).wrapping_add(8))), F32(c.m.r32(((p_w_2).wrapping_add(224)).wrapping_add(8)))); F::Rand_FloatRange(c, a0_, a1_) }; let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32(((p_w_2).wrapping_add(192)).wrapping_add(12))); let a1_ = rt::fadd(F32(c.m.r32(((p_w_2).wrapping_add(192)).wrapping_add(12))), F32(c.m.r32(((p_w_2).wrapping_add(224)).wrapping_add(12)))); F::Rand_FloatRange(c, a0_, a1_) }; let a_ = (fp).wrapping_add(12); c.m.w32(a_, (v_).0); };
        F::Vec4_Clamp(c, (p_p_1).wrapping_add(144), (p_p_1).wrapping_add(144), F32(0x00000000), F32(0x437f0000));
        F::Vec4_Clamp(c, fp, fp, F32(0x00000000), F32(0x437f0000));
        F::Vec3_Sub(c, (p_p_1).wrapping_add(160), fp, (p_p_1).wrapping_add(144));
        F::Vec4_Copy(c, (p_p_1).wrapping_add(128), (p_p_1).wrapping_add(144));
        { let v_ = { let a0_ = F32(c.m.r32(((p_w_2).wrapping_add(456)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a1_ = rt::fadd(F32(c.m.r32(((p_w_2).wrapping_add(456)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((p_w_2).wrapping_add(480)).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); F::Rand_FloatRange(c, a0_, a1_) }; let a_ = (fp.wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32(((p_w_2).wrapping_add(456)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a1_ = rt::fadd(F32(c.m.r32(((p_w_2).wrapping_add(456)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((p_w_2).wrapping_add(480)).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); F::Rand_FloatRange(c, a0_, a1_) }; let a_ = (fp.wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32(((p_w_2).wrapping_add(456)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); let a1_ = rt::fadd(F32(c.m.r32(((p_w_2).wrapping_add(456)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((p_w_2).wrapping_add(480)).wrapping_add((((2i32) as u32)).wrapping_mul(4))))); F::Rand_FloatRange(c, a0_, a1_) }; let a_ = (fp.wrapping_add(16)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fdiv(rt::fsub(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((fp.wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4))))), rt::fmul(F32(c.m.r32((p_p_1).wrapping_add(248))), F32(c.m.r32((l_def_3).wrapping_add(364))))); let a_ = (p_p_1).wrapping_add(228); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fdiv(rt::fsub(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((fp.wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(4))))), rt::fmul(F32(c.m.r32((p_p_1).wrapping_add(248))), rt::fsub(F32(0x3f800000), F32(c.m.r32((l_def_3).wrapping_add(364)))))); let a_ = (p_p_1).wrapping_add(232); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = (p_p_1).wrapping_add(224); c.m.w32(a_, (v_).0); };
        if (((((c.m.r32((l_def_3).wrapping_add(416)) as i32)) & (2048i32))) != 0) {
            { let v_ = rt::fmul(F32(c.m.r32((p_w_2).wrapping_add(552))), F32(0x41f00000)); let a_ = (p_p_1).wrapping_add(260); c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x00000000); let a_ = (p_p_1).wrapping_add(256); c.m.w32(a_, (v_).0); };
            { let v_ = rt::fsub(F32(c.m.r32(((p_w_2).wrapping_add(536)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((p_w_2).wrapping_add(536)).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a_ = (p_p_1).wrapping_add(268); c.m.w32(a_, (v_).0); };
            { let v_ = F32(c.m.r32(((p_w_2).wrapping_add(536)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = (p_p_1).wrapping_add(264); c.m.w32(a_, (v_).0); };
            { let v_ = F32(c.m.r32(((p_w_2).wrapping_add(536)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = (p_p_1).wrapping_add(252); c.m.w32(a_, (v_).0); };
        } else {
            { let v_ = F32(0x3f800000); let a_ = (p_p_1).wrapping_add(252); c.m.w32(a_, (v_).0); };
        }
        if (((((c.m.r32((l_def_3).wrapping_add(416)) as i32)) & (4096i32))) != 0) {
            { let v_ = rt::fmul(F32(c.m.r32((p_w_2).wrapping_add(304))), F32(0x41f00000)); let a_ = (p_p_1).wrapping_add(272); c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x00000000); let a_ = (p_p_1).wrapping_add(276); c.m.w32(a_, (v_).0); };
            { let v_ = rt::fsub(F32(c.m.r32(((p_w_2).wrapping_add(256)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((p_w_2).wrapping_add(256)).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a_ = ((p_p_1).wrapping_add(280)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            { let v_ = rt::fsub(F32(c.m.r32(((p_w_2).wrapping_add(272)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((p_w_2).wrapping_add(272)).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a_ = ((p_p_1).wrapping_add(280)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            { let v_ = rt::fsub(F32(c.m.r32(((p_w_2).wrapping_add(288)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((p_w_2).wrapping_add(288)).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a_ = ((p_p_1).wrapping_add(280)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            { let v_ = F32(c.m.r32(((p_w_2).wrapping_add(256)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = ((p_p_1).wrapping_add(292)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            { let v_ = F32(c.m.r32(((p_w_2).wrapping_add(272)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = ((p_p_1).wrapping_add(292)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            { let v_ = F32(c.m.r32(((p_w_2).wrapping_add(288)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = ((p_p_1).wrapping_add(292)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftLink_InitNode", &PROBE_N, &[], true); }
    r_
}

pub fn EftLink_StepNodes(c: &mut Ctx, mut p_w_1: u32, mut p_alpha_2: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftLink_StepNodes", &PROBE_N, &[(p_w_1) as u32, (p_alpha_2).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_p_3: u32 = 0;
    let mut l_def_4: u32 = 0;
    let mut l_t_5: F32 = F32(0);
    let mut l_a_6: F32 = F32(0);
    let mut l_u2_7: F32 = F32(0);
    let mut l_u_8: F32 = F32(0);
    let mut l_u_9: F32 = F32(0);
    let mut l_f_10: u8 = 0;
    let mut l_old_11: i32 = 0;
    let r_ = 'f: {
        l_p_3 = c.m.r32((p_w_1).wrapping_add(924));
        rt::memset(c, fp, 0, 12);
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(0), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(4), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(8), (v_).0); }
        l_def_4 = c.m.r32((p_w_1).wrapping_add(48));
        l_t_5 = F32(0x00000000);
        {
            'l1: loop {
                if !(((((l_p_3) != (((0i32) as u32))) as i32)) != 0) { break 'l1; }
                'c2: {
                    if (((rt::flt(F32(0x00000000), F32(c.m.r32((l_p_3).wrapping_add(316)))) as i32)) != 0) {
                        { let r_ = F32(0x3f800000); let a_ = (l_p_3).wrapping_add(316); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                        break 'c2;
                    }
                    l_t_5 = (if (((rt::flt(rt::fdiv(F32(c.m.r32((l_p_3).wrapping_add(244))), F32(c.m.r32((l_p_3).wrapping_add(248)))), F32(0x00000000)) as i32)) != 0) { F32(0x00000000) } else { (if (((rt::flt(F32(0x3f800000), rt::fdiv(F32(c.m.r32((l_p_3).wrapping_add(244))), F32(c.m.r32((l_p_3).wrapping_add(248))))) as i32)) != 0) { F32(0x3f800000) } else { rt::fdiv(F32(c.m.r32((l_p_3).wrapping_add(244))), F32(c.m.r32((l_p_3).wrapping_add(248)))) }) });
                    l_u_8 = F32(0x00000000);
                    if (((((c.m.r32((l_def_4).wrapping_add(416)) as i32)) & (2048i32))) != 0) {
                        l_u_8 = (if (((rt::flt(rt::fdiv(F32(c.m.r32((l_p_3).wrapping_add(256))), F32(c.m.r32((l_p_3).wrapping_add(260)))), F32(0x00000000)) as i32)) != 0) { F32(0x00000000) } else { (if (((rt::flt(F32(0x3f800000), rt::fdiv(F32(c.m.r32((l_p_3).wrapping_add(256))), F32(c.m.r32((l_p_3).wrapping_add(260))))) as i32)) != 0) { F32(0x3f800000) } else { rt::fdiv(F32(c.m.r32((l_p_3).wrapping_add(256))), F32(c.m.r32((l_p_3).wrapping_add(260)))) }) });
                        { let v_ = rt::fadd(F32(c.m.r32((l_p_3).wrapping_add(264))), rt::fmul(F32(c.m.r32((l_p_3).wrapping_add(268))), l_u_8)); let a_ = (l_p_3).wrapping_add(252); c.m.w32(a_, (v_).0); };
                        if ((((!(((((c.m.r32((l_p_3).wrapping_add(320)) as i32)) & (65536i32))) != 0)) as i32)) != 0) {
                            { let v_ = rt::fadd(F32(c.m.r32((l_p_3).wrapping_add(256))), F32(0x3f800000)); let a_ = (l_p_3).wrapping_add(256); c.m.w32(a_, (v_).0); };
                        } else {
                            { let v_ = rt::fsub(F32(c.m.r32((l_p_3).wrapping_add(256))), F32(0x3f800000)); let a_ = (l_p_3).wrapping_add(256); c.m.w32(a_, (v_).0); };
                        }
                        if (((((((rt::flt(F32(c.m.r32((l_p_3).wrapping_add(256))), F32(0x00000000)) as i32)) != 0) || (((rt::fle(F32(c.m.r32((l_p_3).wrapping_add(260))), F32(c.m.r32((l_p_3).wrapping_add(256)))) as i32)) != 0)) as i32)) != 0) {
                            { let r_ = 65536i32; let a_ = (l_p_3).wrapping_add(320); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
                            { let v_ = (if (((rt::flt(F32(c.m.r32((l_p_3).wrapping_add(256))), F32(0x00000000)) as i32)) != 0) { F32(0x00000000) } else { (if (((rt::flt(F32(c.m.r32((l_p_3).wrapping_add(260))), F32(c.m.r32((l_p_3).wrapping_add(256)))) as i32)) != 0) { F32(c.m.r32((l_p_3).wrapping_add(260))) } else { F32(c.m.r32((l_p_3).wrapping_add(256))) }) }); let a_ = (l_p_3).wrapping_add(256); c.m.w32(a_, (v_).0); };
                        }
                    }
                    l_u2_7 = F32(0x00000000);
                    if (((((c.m.r32((l_def_4).wrapping_add(416)) as i32)) & (4096i32))) != 0) {
                        l_u2_7 = (if (((rt::flt(rt::fdiv(F32(c.m.r32((l_p_3).wrapping_add(276))), F32(c.m.r32((l_p_3).wrapping_add(272)))), F32(0x00000000)) as i32)) != 0) { F32(0x00000000) } else { (if (((rt::flt(F32(0x3f800000), rt::fdiv(F32(c.m.r32((l_p_3).wrapping_add(276))), F32(c.m.r32((l_p_3).wrapping_add(272))))) as i32)) != 0) { F32(0x3f800000) } else { rt::fdiv(F32(c.m.r32((l_p_3).wrapping_add(276))), F32(c.m.r32((l_p_3).wrapping_add(272)))) }) });
                        { let v_ = rt::fadd(F32(c.m.r32(((l_p_3).wrapping_add(292)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(((l_p_3).wrapping_add(280)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), l_u2_7)); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                        { let v_ = rt::fadd(F32(c.m.r32(((l_p_3).wrapping_add(292)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(((l_p_3).wrapping_add(280)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), l_u2_7)); let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                        { let v_ = rt::fadd(F32(c.m.r32(((l_p_3).wrapping_add(292)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(((l_p_3).wrapping_add(280)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), l_u2_7)); let a_ = (fp).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                        if ((((!(((((c.m.r32((l_p_3).wrapping_add(320)) as i32)) & (131072i32))) != 0)) as i32)) != 0) {
                            { let v_ = rt::fadd(F32(c.m.r32((l_p_3).wrapping_add(276))), F32(0x3f800000)); let a_ = (l_p_3).wrapping_add(276); c.m.w32(a_, (v_).0); };
                        } else {
                            { let v_ = rt::fsub(F32(c.m.r32((l_p_3).wrapping_add(276))), F32(0x3f800000)); let a_ = (l_p_3).wrapping_add(276); c.m.w32(a_, (v_).0); };
                        }
                        if (((((((rt::flt(F32(c.m.r32((l_p_3).wrapping_add(276))), F32(0x00000000)) as i32)) != 0) || (((rt::fle(F32(c.m.r32((l_p_3).wrapping_add(272))), F32(c.m.r32((l_p_3).wrapping_add(276)))) as i32)) != 0)) as i32)) != 0) {
                            { let r_ = 131072i32; let a_ = (l_p_3).wrapping_add(320); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
                            { let v_ = (if (((rt::flt(F32(c.m.r32((l_p_3).wrapping_add(276))), F32(0x00000000)) as i32)) != 0) { F32(0x00000000) } else { (if (((rt::flt(F32(c.m.r32((l_p_3).wrapping_add(272))), F32(c.m.r32((l_p_3).wrapping_add(276)))) as i32)) != 0) { F32(c.m.r32((l_p_3).wrapping_add(272))) } else { F32(c.m.r32((l_p_3).wrapping_add(276))) }) }); let a_ = (l_p_3).wrapping_add(276); c.m.w32(a_, (v_).0); };
                        }
                    } else {
                        { let v_ = F32(0x3f800000); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                        { let v_ = F32(0x3f800000); let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                        { let v_ = F32(0x3f800000); let a_ = (fp).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    }
                    if (((rt::flt(l_t_5, F32(c.m.r32((l_def_4).wrapping_add(364)))) as i32)) != 0) {
                        { let r_ = F32(c.m.r32((l_p_3).wrapping_add(228))); let a_ = (l_p_3).wrapping_add(224); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    } else {
                        { let r_ = F32(c.m.r32((l_p_3).wrapping_add(232))); let a_ = (l_p_3).wrapping_add(224); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    }
                    { let r_ = F32(c.m.r32((l_p_3).wrapping_add(220))); let a_ = (l_p_3).wrapping_add(212); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    { let v_ = { let a0_ = F32(c.m.r32((l_p_3).wrapping_add(212))); F::EftMath_WrapAngle(c, a0_) }; let a_ = (l_p_3).wrapping_add(212); c.m.w32(a_, (v_).0); };
                    if (((rt::flt(l_t_5, F32(c.m.r32((l_def_4).wrapping_add(356)))) as i32)) != 0) {
                        l_a_6 = rt::fdiv(F32(c.m.r32((l_p_3).wrapping_add(244))), F32(c.m.r32((l_p_3).wrapping_add(236))));
                    } else {
                        l_a_6 = F32(0x3f800000);
                    }
                    if (((rt::fle(F32(c.m.r32((l_def_4).wrapping_add(360))), l_t_5) as i32)) != 0) {
                        l_a_6 = rt::fsub(F32(0x3f800000), rt::fdiv(rt::fsub(F32(c.m.r32((l_p_3).wrapping_add(244))), rt::fsub(F32(c.m.r32((l_p_3).wrapping_add(248))), F32(c.m.r32((l_p_3).wrapping_add(240))))), F32(c.m.r32((l_p_3).wrapping_add(240)))));
                    }
                    if (((((c.m.r32((l_p_3).wrapping_add(320)) as i32)) & (2048i32))) != 0) {
                        l_u_9 = (if (((rt::flt(rt::fdiv(F32(c.m.r32((l_p_3).wrapping_add(304))), F32(c.m.r32((l_p_3).wrapping_add(308)))), F32(0x00000000)) as i32)) != 0) { F32(0x00000000) } else { (if (((rt::flt(F32(0x3f800000), rt::fdiv(F32(c.m.r32((l_p_3).wrapping_add(304))), F32(c.m.r32((l_p_3).wrapping_add(308))))) as i32)) != 0) { F32(0x3f800000) } else { rt::fdiv(F32(c.m.r32((l_p_3).wrapping_add(304))), F32(c.m.r32((l_p_3).wrapping_add(308)))) }) });
                        { let v_ = rt::fmul(rt::fadd(F32(c.m.r32((l_p_3).wrapping_add(144))), rt::fmul(F32(c.m.r32((l_p_3).wrapping_add(160))), l_u_9)), F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a_ = (l_p_3).wrapping_add(128); c.m.w32(a_, (v_).0); };
                        { let v_ = rt::fmul(rt::fadd(F32(c.m.r32(((l_p_3).wrapping_add(144)).wrapping_add(4))), rt::fmul(F32(c.m.r32(((l_p_3).wrapping_add(160)).wrapping_add(4))), l_u_9)), F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); let a_ = ((l_p_3).wrapping_add(128)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                        { let v_ = rt::fmul(rt::fadd(F32(c.m.r32(((l_p_3).wrapping_add(144)).wrapping_add(8))), rt::fmul(F32(c.m.r32(((l_p_3).wrapping_add(160)).wrapping_add(8))), l_u_9)), F32(c.m.r32((fp).wrapping_add((((2i32) as u32)).wrapping_mul(4))))); let a_ = ((l_p_3).wrapping_add(128)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                        if (((((((rt::fle(F32(c.m.r32((p_w_1).wrapping_add(312))), l_t_5) as i32)) != 0) && (((rt::flt(l_t_5, F32(c.m.r32((p_w_1).wrapping_add(320)))) as i32)) != 0)) as i32)) != 0) {
                            { let r_ = F32(0x3f800000); let a_ = (l_p_3).wrapping_add(304); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                        }
                    } else {
                        { let v_ = rt::fmul(F32(c.m.r32((l_p_3).wrapping_add(144))), F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a_ = (l_p_3).wrapping_add(128); c.m.w32(a_, (v_).0); };
                        { let v_ = rt::fmul(F32(c.m.r32(((l_p_3).wrapping_add(144)).wrapping_add(4))), F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); let a_ = ((l_p_3).wrapping_add(128)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                        { let v_ = rt::fmul(F32(c.m.r32(((l_p_3).wrapping_add(144)).wrapping_add(8))), F32(c.m.r32((fp).wrapping_add((((2i32) as u32)).wrapping_mul(4))))); let a_ = ((l_p_3).wrapping_add(128)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                    }
                    { let v_ = rt::fmul(rt::fmul(F32(c.m.r32(((l_p_3).wrapping_add(144)).wrapping_add(12))), l_a_6), p_alpha_2); let a_ = ((l_p_3).wrapping_add(128)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                    F::Vec4_Clamp(c, (l_p_3).wrapping_add(128), (l_p_3).wrapping_add(128), F32(0x00000000), F32(0x437f0000));
                    if (((((c.m.r32((p_w_1).wrapping_add(920)) as i32)) & (256i32))) != 0) {
                        l_f_10 = ((0i32) as u8);
                        if ((((!(((rt::flt(F32(c.m.r32((l_p_3).wrapping_add(312))), F32(0x00000000)) as i32)) != 0)) as i32)) != 0) {
                            if (((rt::flt(rt::fsub(F32(c.m.r32((p_w_1).wrapping_add(596))), F32(0x3f800000)), F32(c.m.r32((l_p_3).wrapping_add(312)))) as i32)) != 0) {
                                l_f_10 = ((rt::f2i(rt::fsub(F32(c.m.r32((p_w_1).wrapping_add(596))), F32(0x3f800000)))) as u8);
                            } else {
                                l_f_10 = ((rt::f2i(F32(c.m.r32((l_p_3).wrapping_add(312))))) as u8);
                            }
                        }
                        if (((((c.m.r32((l_p_3).wrapping_add(320)) as i32)) & (128i32))) != 0) {
                            { let a1_ = F32(c.m.r32((((p_w_1).wrapping_add(616)).wrapping_add((((l_f_10) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); let a2_ = F32(c.m.r32((((p_w_1).wrapping_add(616)).wrapping_add((((l_f_10) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a3_ = F32(c.m.r32((((p_w_1).wrapping_add(616)).wrapping_add((((l_f_10) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a4_ = F32(c.m.r32((((p_w_1).wrapping_add(616)).wrapping_add((((l_f_10) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); F::Vec4_Set(c, (l_p_3).wrapping_add(176), a1_, a2_, a3_, a4_) };
                            { let a1_ = F32(c.m.r32((((p_w_1).wrapping_add(616)).wrapping_add((((l_f_10) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); let a2_ = F32(c.m.r32((((p_w_1).wrapping_add(616)).wrapping_add((((l_f_10) as u32)).wrapping_mul(16))).wrapping_add((((3i32) as u32)).wrapping_mul(4)))); let a3_ = F32(c.m.r32((((p_w_1).wrapping_add(616)).wrapping_add((((l_f_10) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a4_ = F32(c.m.r32((((p_w_1).wrapping_add(616)).wrapping_add((((l_f_10) as u32)).wrapping_mul(16))).wrapping_add((((3i32) as u32)).wrapping_mul(4)))); F::Vec4_Set(c, (l_p_3).wrapping_add(192), a1_, a2_, a3_, a4_) };
                        } else {
                            { let a1_ = F32(c.m.r32((((p_w_1).wrapping_add(616)).wrapping_add((((l_f_10) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a2_ = F32(c.m.r32((((p_w_1).wrapping_add(616)).wrapping_add((((l_f_10) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a3_ = F32(c.m.r32((((p_w_1).wrapping_add(616)).wrapping_add((((l_f_10) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); let a4_ = F32(c.m.r32((((p_w_1).wrapping_add(616)).wrapping_add((((l_f_10) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); F::Vec4_Set(c, (l_p_3).wrapping_add(176), a1_, a2_, a3_, a4_) };
                            { let a1_ = F32(c.m.r32((((p_w_1).wrapping_add(616)).wrapping_add((((l_f_10) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a2_ = F32(c.m.r32((((p_w_1).wrapping_add(616)).wrapping_add((((l_f_10) as u32)).wrapping_mul(16))).wrapping_add((((3i32) as u32)).wrapping_mul(4)))); let a3_ = F32(c.m.r32((((p_w_1).wrapping_add(616)).wrapping_add((((l_f_10) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); let a4_ = F32(c.m.r32((((p_w_1).wrapping_add(616)).wrapping_add((((l_f_10) as u32)).wrapping_mul(16))).wrapping_add((((3i32) as u32)).wrapping_mul(4)))); F::Vec4_Set(c, (l_p_3).wrapping_add(192), a1_, a2_, a3_, a4_) };
                        }
                        if (((((rt::rem_i32(rt::f2i(F32(c.m.r32((p_w_1).wrapping_add(592)))), ((c.m.r8((l_def_4).wrapping_add(414))) as i32))) == (0i32)) as i32)) != 0) {
                            { let r_ = F32(0x3f800000); let a_ = (l_p_3).wrapping_add(312); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                            if (((rt::fle(F32(c.m.r32((p_w_1).wrapping_add(596))), F32(c.m.r32((l_p_3).wrapping_add(312)))) as i32)) != 0) {
                                { let v_ = F32(0x00000000); let a_ = (l_p_3).wrapping_add(312); c.m.w32(a_, (v_).0); };
                            }
                        }
                    }
                    if (((((((rt::flt(F32(0x00000000), F32(c.m.r32(((l_p_3).wrapping_add(128)).wrapping_add(12)))) as i32)) != 0) && (((rt::flt(F32(0x00000000), F32(c.m.r32((l_p_3).wrapping_add(224)))) as i32)) != 0)) as i32)) != 0) {
                        if ((((!(((((c.m.r32((l_def_4).wrapping_add(416)) as i32)) & (16i32))) != 0)) as i32)) != 0) {
                            F::Vu0Cur_Push(c);
                            F::Vu0Cur_LoadIdentity(c);
                            F::EftLink_BuildCorners(c, l_p_3, p_w_1);
                            F::Vu0Cur_Pop(c);
                        }
                        { let r_ = 64i32; let a_ = (l_p_3).wrapping_add(320); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    } else {
                        { let r_ = (!(64i32)); let a_ = (l_p_3).wrapping_add(320); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                    }
                    { let r_ = F32(0x3f800000); let a_ = (l_p_3).wrapping_add(244); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    if (((rt::fle(F32(c.m.r32((l_p_3).wrapping_add(248))), F32(c.m.r32((l_p_3).wrapping_add(244)))) as i32)) != 0) {
                        l_old_11 = (c.m.r32((l_p_3).wrapping_add(320)) as i32);
                        { let r_ = (!(1i32)); let a_ = (l_p_3).wrapping_add(320); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                        if (((((((((c.m.r32((p_w_1).wrapping_add(920)) as i32)) & (4i32))) != 0) || ((((!((((l_old_11) & (1024i32))) != 0)) as i32)) != 0)) as i32)) != 0) {
                            { let v_ = 0i32; let a_ = (l_p_3).wrapping_add(320); c.m.w32(a_, (v_) as u32); };
                            F::EftLink_UnlinkNode(c, (p_w_1).wrapping_add(924), (p_w_1).wrapping_add(928), l_p_3);
                        }
                    }
                    { let r_ = (!(1024i32)); let a_ = (l_p_3).wrapping_add(320); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                }
                l_p_3 = c.m.r32((l_p_3).wrapping_add(324));
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftLink_StepNodes", &PROBE_N, &[], true); }
    r_
}

pub fn EftLink_BuildCorners(c: &mut Ctx, mut p_p_1: u32, mut p_w_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftLink_BuildCorners", &PROBE_N, &[(p_p_1) as u32, (p_w_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_def_3: u32 = 0;
    let r_ = 'f: {
        l_def_3 = c.m.r32((p_w_2).wrapping_add(48));
        { let a1_ = rt::fmul(F32(c.m.r32((p_p_1).wrapping_add(224))), F32(0xbf000000)); let a2_ = rt::fmul(F32(c.m.r32((p_p_1).wrapping_add(224))), F32(0xbf000000)); F::Vec4_Set(c, (p_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(16)), a1_, a2_, F32(0x00000000), F32(0x3f800000)) };
        { let a1_ = rt::fmul(F32(c.m.r32((p_p_1).wrapping_add(224))), F32(0xbf000000)); let a2_ = rt::fmul(F32(c.m.r32((p_p_1).wrapping_add(224))), F32(0x3f000000)); F::Vec4_Set(c, (p_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(16)), a1_, a2_, F32(0x00000000), F32(0x3f800000)) };
        { let a1_ = rt::fmul(F32(c.m.r32((p_p_1).wrapping_add(224))), F32(0x3f000000)); let a2_ = rt::fmul(F32(c.m.r32((p_p_1).wrapping_add(224))), F32(0xbf000000)); F::Vec4_Set(c, (p_p_1).wrapping_add((((2i32) as u32)).wrapping_mul(16)), a1_, a2_, F32(0x00000000), F32(0x3f800000)) };
        { let a1_ = rt::fmul(F32(c.m.r32((p_p_1).wrapping_add(224))), F32(0x3f000000)); let a2_ = rt::fmul(F32(c.m.r32((p_p_1).wrapping_add(224))), F32(0x3f000000)); F::Vec4_Set(c, (p_p_1).wrapping_add((((3i32) as u32)).wrapping_mul(16)), a1_, a2_, F32(0x00000000), F32(0x3f800000)) };
        F::Vu0Cur_Translate(c, (p_p_1).wrapping_add(64));
        { let a0_ = rt::fmul(rt::fmul(F32(c.m.r32(((p_w_2).wrapping_add(48)).wrapping_add(48))), F32(c.m.r32((p_w_2).wrapping_add(880)))), F32(c.m.r32((p_p_1).wrapping_add(252)))); F::Vu0Cur_ScaleDiagUniform(c, a0_) };
        { let a0_ = F32(c.m.r32((p_p_1).wrapping_add(212))); F::Vu0Cur_RotateZ(c, a0_) };
        { let a0_ = F32(c.m.r32((p_p_1).wrapping_add(208))); F::Vu0Cur_RotateX(c, a0_) };
        { let a0_ = F32(c.m.r32((p_p_1).wrapping_add(216))); F::Vu0Cur_RotateZ(c, a0_) };
        if ((((!(((((c.m.r32((l_def_3).wrapping_add(416)) as i32)) & (8192i32))) != 0)) as i32)) != 0) {
            { let a2_ = F32(c.m.r32((p_w_2).wrapping_add(880))); F::Vec3_Scale(c, fp, (p_p_1).wrapping_add(80), a2_) };
            F::Vu0Cur_Translate(c, fp);
        } else {
            F::Vu0Cur_Translate(c, (p_p_1).wrapping_add(80));
        }
        { let a0_ = F32(c.m.r32((p_w_2).wrapping_add(904))); F::Vu0Cur_RotateX(c, a0_) };
        { let a0_ = F32(c.m.r32((p_w_2).wrapping_add(908))); F::Vu0Cur_RotateY(c, a0_) };
        F::Vu0Cur_Translate(c, (p_p_1).wrapping_add(96));
        F::Vu0Cur_MulVec4(c, (p_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (p_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(16)));
        F::Vu0Cur_MulVec4(c, (p_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(16)), (p_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(16)));
        F::Vu0Cur_MulVec4(c, (p_p_1).wrapping_add((((2i32) as u32)).wrapping_mul(16)), (p_p_1).wrapping_add((((2i32) as u32)).wrapping_mul(16)));
        F::Vu0Cur_MulVec4(c, (p_p_1).wrapping_add((((3i32) as u32)).wrapping_mul(16)), (p_p_1).wrapping_add((((3i32) as u32)).wrapping_mul(16)));
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftLink_BuildCorners", &PROBE_N, &[], true); }
    r_
}

pub fn EftLink_LinkNode(c: &mut Ctx, mut p_head_1: u32, mut p_tail_2: u32, mut p_p_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftLink_LinkNode", &PROBE_N, &[(p_head_1) as u32, (p_tail_2) as u32, (p_p_3) as u32], false); }
    let r_ = 'f: {
    if (((((c.m.r32(p_head_1)) == (((0i32) as u32))) as i32)) != 0) {
        { let v_ = p_p_3; let a_ = p_head_1; c.m.w32(a_, v_); };
        { let v_ = p_p_3; let a_ = p_tail_2; c.m.w32(a_, v_); };
    } else {
        { let v_ = c.m.r32(p_tail_2); let a_ = (p_p_3).wrapping_add(328); c.m.w32(a_, v_); };
        { let v_ = p_p_3; let a_ = (c.m.r32(p_tail_2)).wrapping_add(324); c.m.w32(a_, v_); };
        { let v_ = p_p_3; let a_ = p_tail_2; c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftLink_LinkNode", &PROBE_N, &[], true); }
    r_
}

pub fn EftLink_UnlinkNode(c: &mut Ctx, mut p_head_1: u32, mut p_tail_2: u32, mut p_p_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftLink_UnlinkNode", &PROBE_N, &[(p_head_1) as u32, (p_tail_2) as u32, (p_p_3) as u32], false); }
    let mut l_prev_4: u32 = 0;
    let r_ = 'f: {
    if (((((c.m.r32(p_head_1)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    if (((((c.m.r32((p_p_3).wrapping_add(328))) == (((0i32) as u32))) as i32)) != 0) {
        p_p_3 = c.m.r32((p_p_3).wrapping_add(324));
        if (((((p_p_3) == (((0i32) as u32))) as i32)) != 0) {
            { let v_ = ((0i32) as u32); let a_ = p_head_1; c.m.w32(a_, v_); };
            { let v_ = ((0i32) as u32); let a_ = p_tail_2; c.m.w32(a_, v_); };
        } else {
            { let v_ = p_p_3; let a_ = p_head_1; c.m.w32(a_, v_); };
            { let v_ = ((0i32) as u32); let a_ = (p_p_3).wrapping_add(328); c.m.w32(a_, v_); };
        }
    } else {
        if (((((c.m.r32((p_p_3).wrapping_add(324))) == (((0i32) as u32))) as i32)) != 0) {
            l_prev_4 = c.m.r32((p_p_3).wrapping_add(328));
            { let v_ = l_prev_4; let a_ = p_tail_2; c.m.w32(a_, v_); };
            { let v_ = ((0i32) as u32); let a_ = (l_prev_4).wrapping_add(324); c.m.w32(a_, v_); };
        } else {
            { let v_ = c.m.r32((p_p_3).wrapping_add(328)); let a_ = (c.m.r32((p_p_3).wrapping_add(324))).wrapping_add(328); c.m.w32(a_, v_); };
            { let v_ = c.m.r32((p_p_3).wrapping_add(324)); let a_ = (c.m.r32((p_p_3).wrapping_add(328))).wrapping_add(324); c.m.w32(a_, v_); };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftLink_UnlinkNode", &PROBE_N, &[], true); }
    r_
}

pub fn EftLink_DrawBillboardClipped(c: &mut Ctx, mut p_pos_1: u32, mut p_w_2: F32, mut p_h_3: F32, mut p_color_4: u32, mut p_scale_5: u32, mut p_u0_6: F32, mut p_v0_7: F32, mut p_u1_8: F32, mut p_v1_9: F32, mut p_rot_10: F32, mut p_layer_11: i32, mut p_noDepth_12: i32, mut p_tex0_13: u64, mut p_zScale_14: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftLink_DrawBillboardClipped", &PROBE_N, &[(p_pos_1) as u32, (p_w_2).0, (p_h_3).0, (p_color_4) as u32, (p_scale_5) as u32, (p_u0_6).0, (p_v0_7).0, (p_u1_8).0, (p_v1_9).0, (p_rot_10).0, (p_layer_11) as u32, (p_noDepth_12) as u32, (p_tex0_13) as u32, (p_zScale_14).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(704) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 704);
    let mut l_i_15: i32 = 0;
    let mut l_z_16: i32 = 0;
    let r_ = 'f: {
        F::Vec4_Set(c, (fp.wrapping_add(560)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), rt::fneg(p_w_2), rt::fneg(p_h_3), F32(0x00000000), F32(0x3f800000));
        F::Vec4_Set(c, (fp.wrapping_add(560)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), p_w_2, rt::fneg(p_h_3), F32(0x00000000), F32(0x3f800000));
        F::Vec4_Set(c, (fp.wrapping_add(560)).wrapping_add((((2i32) as u32)).wrapping_mul(16)), rt::fneg(p_w_2), p_h_3, F32(0x00000000), F32(0x3f800000));
        F::Vec4_Set(c, (fp.wrapping_add(560)).wrapping_add((((3i32) as u32)).wrapping_mul(16)), p_w_2, p_h_3, F32(0x00000000), F32(0x3f800000));
        { let a1_ = (c.m.r32(sym::gEftLink)).wrapping_add(67232); F::Mtx_Translate(c, fp.wrapping_add(432), a1_, p_scale_5) };
        F::Mtx_RotateZ(c, fp.wrapping_add(432), fp.wrapping_add(432), p_rot_10);
        { let a1_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(64); F::Mtx_InverseRT(c, fp.wrapping_add(496), a1_) };
        { let v_ = F32(0x00000000); let a_ = ((fp.wrapping_add(496)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = ((fp.wrapping_add(496)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = ((fp.wrapping_add(496)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        F::Mtx_Mul(c, fp.wrapping_add(432), fp.wrapping_add(496), fp.wrapping_add(432));
        {
            l_i_15 = 0i32;
            'l1: loop {
                if !(((((l_i_15) < (4i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    F::Mtx_MulVec4(c, (fp.wrapping_add(560)).wrapping_add((((l_i_15) as u32)).wrapping_mul(16)), fp.wrapping_add(432), (fp.wrapping_add(560)).wrapping_add((((l_i_15) as u32)).wrapping_mul(16)));
                    F::Vec3_Add(c, (fp.wrapping_add(560)).wrapping_add((((l_i_15) as u32)).wrapping_mul(16)), (fp.wrapping_add(560)).wrapping_add((((l_i_15) as u32)).wrapping_mul(16)), p_pos_1);
                    { let v_ = F32(0x3f800000); let a_ = ((fp.wrapping_add(560)).wrapping_add((((l_i_15) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
                }
                { let o_ = l_i_15; let n_ = (o_).wrapping_add(1); l_i_15 = n_; };
            }
        }
        { let _ = F::Vu0Cur_ProjectPoint(c, fp.wrapping_add(688), p_pos_1); };
        l_z_16 = ((c.m.r32((fp.wrapping_add(688)).wrapping_add(8)) as i32)).wrapping_shr((8i32) as u32);
        if ((((((c.m.r32((fp.wrapping_add(688)).wrapping_add(8)) as i32)) >= (0i32)) as i32)) != 0) {
            F::Vec4_Set(c, (fp.wrapping_add(624)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), p_u0_6, p_v0_7, F32(0x3f800000), F32(0x00000000));
            F::Vec4_Set(c, (fp.wrapping_add(624)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), p_u1_8, p_v0_7, F32(0x3f800000), F32(0x00000000));
            F::Vec4_Set(c, (fp.wrapping_add(624)).wrapping_add((((2i32) as u32)).wrapping_mul(16)), p_u0_6, p_v1_9, F32(0x3f800000), F32(0x00000000));
            F::Vec4_Set(c, (fp.wrapping_add(624)).wrapping_add((((3i32) as u32)).wrapping_mul(16)), p_u1_8, p_v1_9, F32(0x3f800000), F32(0x00000000));
            {
                l_i_15 = 0i32;
                'l3: loop {
                    if !(((((l_i_15) < (2i32)) as i32)) != 0) { break 'l3; }
                    'c4: {
                        F::ClipVtx_Set(c, (fp).wrapping_add((((0i32) as u32)).wrapping_mul(48)), (fp.wrapping_add(560)).wrapping_add((((l_i_15) as u32)).wrapping_mul(16)), (fp.wrapping_add(624)).wrapping_add((((l_i_15) as u32)).wrapping_mul(16)), p_color_4);
                        F::ClipVtx_Set(c, (fp).wrapping_add((((1i32) as u32)).wrapping_mul(48)), (fp.wrapping_add(560)).wrapping_add(((((l_i_15).wrapping_add(1i32)) as u32)).wrapping_mul(16)), (fp.wrapping_add(624)).wrapping_add(((((l_i_15).wrapping_add(1i32)) as u32)).wrapping_mul(16)), p_color_4);
                        F::ClipVtx_Set(c, (fp).wrapping_add((((2i32) as u32)).wrapping_mul(48)), (fp.wrapping_add(560)).wrapping_add(((((l_i_15).wrapping_add(2i32)) as u32)).wrapping_mul(16)), (fp.wrapping_add(624)).wrapping_add(((((l_i_15).wrapping_add(2i32)) as u32)).wrapping_mul(16)), p_color_4);
                        F::EftGfx_DrawPolyFixedZ(c, fp, p_layer_11, 0i32, 0i32, p_noDepth_12, 0i32, p_tex0_13, rt::f2i(rt::fmul(rt::i2f(l_z_16), p_zScale_14)));
                    }
                    { let o_ = l_i_15; let n_ = (o_).wrapping_add(1); l_i_15 = n_; };
                }
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftLink_DrawBillboardClipped", &PROBE_N, &[], true); }
    r_
}

pub fn EftLink_DrawBillboard(c: &mut Ctx, mut p_pos_1: u32, mut p_w_2: F32, mut p_h_3: F32, mut p_color_4: u32, mut p_offX_5: i32, mut p_offY_6: i32, mut p_u0_7: F32, mut p_v0_8: F32, mut p_u1_9: F32, mut p_v1_10: F32, mut p_rot_11: F32, mut p_layer_12: i32, mut p_noDepth_13: i32, mut p_tex0_14: u64) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftLink_DrawBillboard", &PROBE_N, &[(p_pos_1) as u32, (p_w_2).0, (p_h_3).0, (p_color_4) as u32, (p_offX_5) as u32, (p_offY_6) as u32, (p_u0_7).0, (p_v0_8).0, (p_u1_9).0, (p_v1_10).0, (p_rot_11).0, (p_layer_12) as u32, (p_noDepth_13) as u32, (p_tex0_14) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(224) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 224);
    let mut l_scale_15: F32 = F32(0);
    let mut l_sw_16: i32 = 0;
    let mut l_sh_17: i32 = 0;
    let mut l_k_18: i32 = 0;
    let mut l_oy_19: i32 = 0;
    let mut l_z_20: i32 = 0;
    let mut l_zz_21: i32 = 0;
    let mut l_x0_22: i32 = 0;
    let mut l_y0_23: i32 = 0;
    let mut l_x1_24: i32 = 0;
    let mut l_y1_25: i32 = 0;
    let mut l_x2_26: i32 = 0;
    let mut l_y2_27: i32 = 0;
    let mut l_x3_28: i32 = 0;
    let mut l_y3_29: i32 = 0;
    let mut l_abe_30: i32 = 0;
    let mut l_ctx_31: i32 = 0;
    let mut l_l_32: i32 = 0;
    let mut l_alpha_33: u32 = 0;
    let mut l_p_34: u32 = 0;
    let mut l_e_35: u32 = 0;
    let r_ = 'f: {
        l_scale_15 = { let a_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(564); F32(c.m.r32(a_)) };
        l_abe_30 = 1i32;
        { let a2_ = rt::fmul(F32(0x3f955555), F::Port_WideFactor(c)); F::Vec4_Set(c, fp.wrapping_add(64), F32(0x3f800000), a2_, F32(0x3f800000), F32(0x3f800000)) };
        { let v_ = F32(0x3f800000); let a_ = (p_pos_1).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let _ = F::Vu0Cur_ProjectPoint(c, fp.wrapping_add(144), p_pos_1); };
        l_sw_16 = rt::f2i(p_w_2);
        l_sh_17 = rt::f2i(p_h_3);
        { let r_ = F32(0x45800000); let n_ = rt::fmul(l_scale_15, r_); l_scale_15 = n_; };
        l_z_20 = ((c.m.r32((fp.wrapping_add(144)).wrapping_add(8)) as i32)).wrapping_shr((8i32) as u32);
        { let r_ = rt::i2f((c.m.r32((fp.wrapping_add(144)).wrapping_add(12)) as i32)); let n_ = rt::fdiv(l_scale_15, r_); l_scale_15 = n_; };
        l_k_18 = rt::f2i(l_scale_15);
        l_sw_16 = ((l_sw_16).wrapping_mul(l_k_18)).wrapping_shr((12i32) as u32);
        l_sh_17 = ((l_sh_17).wrapping_mul(l_k_18)).wrapping_shr((12i32) as u32);
        if (((((p_offX_5) >= (0i32)) as i32)) != 0) {
            p_offX_5 = ((p_offX_5).wrapping_mul(l_k_18)).wrapping_shr((12i32) as u32);
        } else {
            p_offX_5 = ((((p_offX_5).wrapping_neg()).wrapping_mul(l_k_18)).wrapping_shr((12i32) as u32)).wrapping_neg();
        }
        if (((((p_offY_6) >= (0i32)) as i32)) != 0) {
            l_oy_19 = ((p_offY_6).wrapping_mul(l_k_18)).wrapping_shr((12i32) as u32);
        } else {
            l_oy_19 = ((((p_offY_6).wrapping_neg()).wrapping_mul(l_k_18)).wrapping_shr((12i32) as u32)).wrapping_neg();
        }
        /* asm: __asm__("") */
        if (((((l_sw_16) < (2i32)) as i32)) != 0) {
            break 'f;
        }
        if (((((l_sh_17) < (2i32)) as i32)) != 0) {
            break 'f;
        }
        F::Vec4_Set(c, fp.wrapping_add(80), rt::i2f((p_offX_5).wrapping_sub(l_sw_16)), rt::i2f((l_oy_19).wrapping_sub(l_sh_17)), F32(0x00000000), F32(0x3f800000));
        F::Vec4_Set(c, fp.wrapping_add(96), rt::i2f((p_offX_5).wrapping_sub(l_sw_16)), rt::i2f((l_sh_17).wrapping_add(l_oy_19)), F32(0x00000000), F32(0x3f800000));
        F::Vec4_Set(c, fp.wrapping_add(112), rt::i2f((l_sw_16).wrapping_add(p_offX_5)), rt::i2f((l_oy_19).wrapping_sub(l_sh_17)), F32(0x00000000), F32(0x3f800000));
        F::Vec4_Set(c, fp.wrapping_add(128), rt::i2f((l_sw_16).wrapping_add(p_offX_5)), rt::i2f((l_sh_17).wrapping_add(l_oy_19)), F32(0x00000000), F32(0x3f800000));
        { let a1_ = (c.m.r32(sym::gEftLink)).wrapping_add(67232); F::Mtx_RotateZ(c, fp, a1_, p_rot_11) };
        F::Mtx_MulVec4(c, fp.wrapping_add(80), fp, fp.wrapping_add(80));
        F::Mtx_MulVec4(c, fp.wrapping_add(96), fp, fp.wrapping_add(96));
        F::Mtx_MulVec4(c, fp.wrapping_add(112), fp, fp.wrapping_add(112));
        F::Mtx_MulVec4(c, fp.wrapping_add(128), fp, fp.wrapping_add(128));
        F::Vec4_Mul(c, fp.wrapping_add(80), fp.wrapping_add(80), fp.wrapping_add(64));
        F::Vec4_Mul(c, fp.wrapping_add(96), fp.wrapping_add(96), fp.wrapping_add(64));
        F::Vec4_Mul(c, fp.wrapping_add(112), fp.wrapping_add(112), fp.wrapping_add(64));
        F::Vec4_Mul(c, fp.wrapping_add(128), fp.wrapping_add(128), fp.wrapping_add(64));
        F::Vec4_ToInt(c, fp.wrapping_add(160), fp.wrapping_add(80));
        F::Vec4_ToInt(c, fp.wrapping_add(176), fp.wrapping_add(96));
        F::Vec4_ToInt(c, fp.wrapping_add(192), fp.wrapping_add(112));
        F::Vec4_ToInt(c, fp.wrapping_add(208), fp.wrapping_add(128));
        if (({ let a0_ = ((c.m.r32(fp.wrapping_add(144)) as i32)).wrapping_add((c.m.r32(fp.wrapping_add(160)) as i32)); let a1_ = ((c.m.r32((fp.wrapping_add(144)).wrapping_add(4)) as i32)).wrapping_add((c.m.r32((fp.wrapping_add(160)).wrapping_add(4)) as i32)); let a2_ = (c.m.r32((fp.wrapping_add(144)).wrapping_add(8)) as i32); F::EftLink_IsCornerOffScreen(c, a0_, a1_, a2_) }) != 0) {
            break 'f;
        }
        if (({ let a0_ = ((c.m.r32(fp.wrapping_add(144)) as i32)).wrapping_add((c.m.r32(fp.wrapping_add(176)) as i32)); let a1_ = ((c.m.r32((fp.wrapping_add(144)).wrapping_add(4)) as i32)).wrapping_add((c.m.r32((fp.wrapping_add(176)).wrapping_add(4)) as i32)); let a2_ = (c.m.r32((fp.wrapping_add(144)).wrapping_add(8)) as i32); F::EftLink_IsCornerOffScreen(c, a0_, a1_, a2_) }) != 0) {
            break 'f;
        }
        if (({ let a0_ = ((c.m.r32(fp.wrapping_add(144)) as i32)).wrapping_add((c.m.r32(fp.wrapping_add(192)) as i32)); let a1_ = ((c.m.r32((fp.wrapping_add(144)).wrapping_add(4)) as i32)).wrapping_add((c.m.r32((fp.wrapping_add(192)).wrapping_add(4)) as i32)); let a2_ = (c.m.r32((fp.wrapping_add(144)).wrapping_add(8)) as i32); F::EftLink_IsCornerOffScreen(c, a0_, a1_, a2_) }) != 0) {
            break 'f;
        }
        if (({ let a0_ = ((c.m.r32(fp.wrapping_add(144)) as i32)).wrapping_add((c.m.r32(fp.wrapping_add(208)) as i32)); let a1_ = ((c.m.r32((fp.wrapping_add(144)).wrapping_add(4)) as i32)).wrapping_add((c.m.r32((fp.wrapping_add(208)).wrapping_add(4)) as i32)); let a2_ = (c.m.r32((fp.wrapping_add(144)).wrapping_add(8)) as i32); F::EftLink_IsCornerOffScreen(c, a0_, a1_, a2_) }) != 0) {
            break 'f;
        }
        if ((p_noDepth_13) != 0) {
            { let v_ = 16777215i32; let a_ = (fp.wrapping_add(144)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        }
        l_zz_21 = (((c.m.r32((fp.wrapping_add(144)).wrapping_add(8)) as i32)).wrapping_shr((8i32) as u32)).wrapping_shl((8i32) as u32);
        l_p_34 = c.m.r32(sym::gOtCur);
        l_x0_22 = ((c.m.r32(fp.wrapping_add(144)) as i32)).wrapping_add((c.m.r32(fp.wrapping_add(160)) as i32));
        l_y0_23 = ((c.m.r32((fp.wrapping_add(144)).wrapping_add(4)) as i32)).wrapping_add((c.m.r32((fp.wrapping_add(160)).wrapping_add(4)) as i32));
        l_x1_24 = ((c.m.r32(fp.wrapping_add(144)) as i32)).wrapping_add((c.m.r32(fp.wrapping_add(176)) as i32));
        l_y1_25 = ((c.m.r32((fp.wrapping_add(144)).wrapping_add(4)) as i32)).wrapping_add((c.m.r32((fp.wrapping_add(176)).wrapping_add(4)) as i32));
        l_x2_26 = ((c.m.r32(fp.wrapping_add(144)) as i32)).wrapping_add((c.m.r32(fp.wrapping_add(192)) as i32));
        l_y2_27 = ((c.m.r32((fp.wrapping_add(144)).wrapping_add(4)) as i32)).wrapping_add((c.m.r32((fp.wrapping_add(192)).wrapping_add(4)) as i32));
        l_x3_28 = ((c.m.r32(fp.wrapping_add(144)) as i32)).wrapping_add((c.m.r32(fp.wrapping_add(208)) as i32));
        { let v_ = (l_p_34).wrapping_add((((1i32) as u32)).wrapping_mul(128)); let a_ = sym::gOtCur; c.m.w32(a_, v_); };
        l_y3_29 = ((c.m.r32((fp.wrapping_add(144)).wrapping_add(4)) as i32)).wrapping_add((c.m.r32((fp.wrapping_add(208)).wrapping_add(4)) as i32));
        if (((((l_p_34) == (((0i32) as u32))) as i32)) != 0) {
            break 'f;
        }
        l_ctx_31 = (((p_layer_12) >= (2i32)) as i32);
        { let v_ = (((((((l_abe_30) as u64)).wrapping_shl((6i32) as u32)) | ((((l_ctx_31) as u64)).wrapping_shl((9i32) as u32)))) | (((20i32) as u64))); let a_ = (l_p_34).wrapping_add(32); c.m.w64(a_, v_); };
        { let v_ = ((536870919i32) as u32); let a_ = l_p_34; c.m.w32(a_, v_); };
        { let v_ = ((268435456i32) as u32); let a_ = (l_p_34).wrapping_add(8); c.m.w32(a_, v_); };
        { let v_ = ((1342177287i32) as u32); let a_ = (l_p_34).wrapping_add(12); c.m.w32(a_, v_); };
        { let v_ = 14123288431433908225u64; let a_ = (l_p_34).wrapping_add(16); c.m.w64(a_, v_); };
        { let v_ = (((268436062347616i64).wrapping_add((((l_ctx_31).wrapping_shl((4i32) as u32)) as i64))) as u64); let a_ = (l_p_34).wrapping_add(24); c.m.w64(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = (l_p_34).wrapping_add(4); c.m.w32(a_, v_); };
        { let v_ = ((rt::f2i(F32(c.m.r32(p_color_4)))) as u8); let a_ = (l_p_34).wrapping_add(48); c.m.w8(a_, v_); };
        { let v_ = ((rt::f2i(F32(c.m.r32((p_color_4).wrapping_add(4))))) as u8); let a_ = ((l_p_34).wrapping_add(48)).wrapping_add(1); c.m.w8(a_, v_); };
        { let v_ = ((rt::f2i(F32(c.m.r32((p_color_4).wrapping_add(8))))) as u8); let a_ = ((l_p_34).wrapping_add(48)).wrapping_add(2); c.m.w8(a_, v_); };
        l_alpha_33 = rt::f2u(F32(c.m.r32((p_color_4).wrapping_add(12))));
        { let v_ = ((l_x0_22) as u64); let a_ = (l_p_34).wrapping_add(64); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = ((l_y0_23) as u64); let a_ = ((l_p_34).wrapping_add(64)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        l_l_32 = p_layer_12;
        { let v_ = ((l_x1_24) as u64); let a_ = (l_p_34).wrapping_add(80); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = ((l_y1_25) as u64); let a_ = ((l_p_34).wrapping_add(80)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = ((l_x2_26) as u64); let a_ = (l_p_34).wrapping_add(96); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = ((l_y2_27) as u64); let a_ = ((l_p_34).wrapping_add(96)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = ((l_x3_28) as u64); let a_ = (l_p_34).wrapping_add(112); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = ((l_y3_29) as u64); let a_ = ((l_p_34).wrapping_add(112)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
        { let v_ = ((l_zz_21) as u64); let a_ = ((l_p_34).wrapping_add(64)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
        { let v_ = ((l_zz_21) as u64); let a_ = ((l_p_34).wrapping_add(80)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
        { let v_ = ((l_zz_21) as u64); let a_ = ((l_p_34).wrapping_add(96)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
        { let v_ = ((l_zz_21) as u64); let a_ = ((l_p_34).wrapping_add(112)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
        if (((((l_l_32) >= (2i32)) as i32)) != 0) {
            { let r_ = 2i32; let n_ = (l_l_32).wrapping_sub(r_); l_l_32 = n_; };
        }
        { let v_ = ((l_alpha_33) as u8); let a_ = ((l_p_34).wrapping_add(48)).wrapping_add(3); c.m.w8(a_, v_); };
        { let v_ = F32(0x3f800000); let a_ = ((l_p_34).wrapping_add(48)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = p_u0_7; let a_ = (l_p_34).wrapping_add(56); c.m.w32(a_, (v_).0); };
        { let v_ = p_v0_8; let a_ = ((l_p_34).wrapping_add(56)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = p_u0_7; let a_ = (l_p_34).wrapping_add(72); c.m.w32(a_, (v_).0); };
        { let v_ = p_v1_10; let a_ = ((l_p_34).wrapping_add(72)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = p_u1_9; let a_ = (l_p_34).wrapping_add(88); c.m.w32(a_, (v_).0); };
        { let v_ = p_v0_8; let a_ = ((l_p_34).wrapping_add(88)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = p_u1_9; let a_ = (l_p_34).wrapping_add(104); c.m.w32(a_, (v_).0); };
        { let v_ = p_v1_10; let a_ = ((l_p_34).wrapping_add(104)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = ((255i32) as u64); let a_ = ((l_p_34).wrapping_add(64)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
        { let v_ = ((255i32) as u64); let a_ = ((l_p_34).wrapping_add(80)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
        { let v_ = ((255i32) as u64); let a_ = ((l_p_34).wrapping_add(96)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
        { let v_ = ((255i32) as u64); let a_ = ((l_p_34).wrapping_add(112)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
        { let v_ = p_tex0_14; let a_ = (l_p_34).wrapping_add(40); c.m.w64(a_, v_); };
        if (((((l_z_20) < (0i32)) as i32)) != 0) {
            l_e_35 = ((c.m.r32(sym::gOtZ)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((l_l_32) as u32)).wrapping_mul(8));
        } else {
            if (((((l_z_20) >= (4096i32)) as i32)) != 0) {
                l_e_35 = ((c.m.r32(sym::gOtZ)).wrapping_add((((4095i32) as u32)).wrapping_mul(16))).wrapping_add((((l_l_32) as u32)).wrapping_mul(8));
            } else {
                l_e_35 = ((c.m.r32(sym::gOtZ)).wrapping_add((((l_z_20) as u32)).wrapping_mul(16))).wrapping_add((((l_l_32) as u32)).wrapping_mul(8));
            }
        }
        { let v_ = l_p_34; let a_ = (c.m.r32((l_e_35).wrapping_add(4))).wrapping_add(4); c.m.w32(a_, v_); };
        { let v_ = l_p_34; let a_ = (l_e_35).wrapping_add(4); c.m.w32(a_, v_); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftLink_DrawBillboard", &PROBE_N, &[], true); }
    r_
}

pub fn EftLink_Create(c: &mut Ctx, mut p_arg_1: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftLink_Create", &PROBE_N, &[(p_arg_1) as u32], false); }
    let r_ = 'f: {
    if (((((c.m.r32(sym::gEftLink)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f ((0i32) as u32);
    }
    if (((((p_arg_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f ((0i32) as u32);
    }
    break 'f { let a0_ = { let a_ = c.m.r32(sym::gEftLink); c.m.r32(a_) }; F::BtlTaskList_AddTail(c, a0_, sym::gEftLinkClass, p_arg_1) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftLink_Create", &PROBE_N, &[], true); }
    r_
}

pub fn EftLink_Stop(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftLink_Stop", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let mut l_now_3: i32 = 0;
    let r_ = 'f: {
    l_now_3 = 1i32;
    if (((((c.m.r32(sym::gEftLink)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    if ((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) != (fid::EftLink_Update)) as i32)) != 0) {
        break 'f;
    }
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    if (((((l_w_2) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    if (((((c.m.r32((l_w_2).wrapping_add(920)) as i32)) & (4096i32))) != 0) {
        break 'f;
    }
    if ((((!(((((c.m.r32((l_w_2).wrapping_add(920)) as i32)) & (1i32))) != 0)) as i32)) != 0) {
        break 'f;
    }
    if (((rt::flt(F32(0x00000000), F32(c.m.r32((l_w_2).wrapping_add(604)))) as i32)) != 0) {
        { let r_ = 8i32; let a_ = (l_w_2).wrapping_add(920); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        l_now_3 = 0i32;
    }
    if (((rt::flt(F32(0x00000000), F32(c.m.r32((l_w_2).wrapping_add(612)))) as i32)) != 0) {
        { let r_ = 32i32; let a_ = (l_w_2).wrapping_add(920); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        l_now_3 = 0i32;
    }
    if ((l_now_3) != 0) {
        { let r_ = 4i32; let a_ = (l_w_2).wrapping_add(920); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
    { let r_ = 4096i32; let a_ = (l_w_2).wrapping_add(920); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftLink_Stop", &PROBE_N, &[], true); }
    r_
}

pub fn EftLink_Kill(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftLink_Kill", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
    if (((((c.m.r32(sym::gEftLink)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    if ((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) != (fid::EftLink_Update)) as i32)) != 0) {
        break 'f;
    }
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    if (((((l_w_2) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    if (((((c.m.r32((l_w_2).wrapping_add(920)) as i32)) & (1i32))) != 0) {
        { let r_ = 20i32; let a_ = (l_w_2).wrapping_add(920); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftLink_Kill", &PROBE_N, &[], true); }
    r_
}

pub fn EftLink_SetFade(c: &mut Ctx, mut p_task_1: u32, mut p_frames_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftLink_SetFade", &PROBE_N, &[(p_task_1) as u32, (p_frames_2) as u32], false); }
    let mut l_w_3: u32 = 0;
    let r_ = 'f: {
    if (((((c.m.r32(sym::gEftLink)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    if ((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) != (fid::EftLink_Update)) as i32)) != 0) {
        break 'f;
    }
    l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
    if (((((l_w_3) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    if (((((c.m.r32((l_w_3).wrapping_add(920)) as i32)) & (1i32))) != 0) {
        { let v_ = rt::i2f(p_frames_2); let a_ = (l_w_3).wrapping_add(608); c.m.w32(a_, (v_).0); };
        { let v_ = rt::i2f(p_frames_2); let a_ = (l_w_3).wrapping_add(612); c.m.w32(a_, (v_).0); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftLink_SetFade", &PROBE_N, &[], true); }
    r_
}

pub fn EftLink_SetEnds(c: &mut Ctx, mut p_task_1: u32, p_pos_2: u32, p_pos2_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftLink_SetEnds", &PROBE_N, &[(p_task_1) as u32, (p_pos_2) as u32, (p_pos2_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_w_4: u32 = 0;
    rt::memcpy(c, fp, p_pos_2, 16);
    rt::memcpy(c, fp.wrapping_add(16), p_pos2_3, 16);
    let r_ = 'f: {
        if (((((c.m.r32(sym::gEftLink)) == (((0i32) as u32))) as i32)) != 0) {
            break 'f;
        }
        if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
            break 'f;
        }
        if ((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) != (fid::EftLink_Update)) as i32)) != 0) {
            break 'f;
        }
        l_w_4 = c.m.r32((p_task_1).wrapping_add(56));
        if (((((l_w_4) == (((0i32) as u32))) as i32)) != 0) {
            break 'f;
        }
        if (((((c.m.r32((l_w_4).wrapping_add(920)) as i32)) & (1i32))) != 0) {
            F::Vec4_Copy(c, ((l_w_4).wrapping_add(48)).wrapping_add(16), fp);
            F::Vec4_Copy(c, ((l_w_4).wrapping_add(48)).wrapping_add(32), fp.wrapping_add(16));
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftLink_SetEnds", &PROBE_N, &[], true); }
    r_
}

pub fn EftLink_SetPos(c: &mut Ctx, mut p_task_1: u32, p_pos_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftLink_SetPos", &PROBE_N, &[(p_task_1) as u32, (p_pos_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_w_3: u32 = 0;
    rt::memcpy(c, fp, p_pos_2, 16);
    let r_ = 'f: {
        if (((((c.m.r32(sym::gEftLink)) == (((0i32) as u32))) as i32)) != 0) {
            break 'f;
        }
        if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
            break 'f;
        }
        if ((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) != (fid::EftLink_Update)) as i32)) != 0) {
            break 'f;
        }
        l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
        if (((((l_w_3) == (((0i32) as u32))) as i32)) != 0) {
            break 'f;
        }
        if (((((c.m.r32((l_w_3).wrapping_add(920)) as i32)) & (1i32))) != 0) {
            F::Vec4_Copy(c, ((l_w_3).wrapping_add(48)).wrapping_add(16), fp);
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftLink_SetPos", &PROBE_N, &[], true); }
    r_
}

pub fn EftLink_SetPos2(c: &mut Ctx, mut p_task_1: u32, p_pos_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftLink_SetPos2", &PROBE_N, &[(p_task_1) as u32, (p_pos_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_w_3: u32 = 0;
    rt::memcpy(c, fp, p_pos_2, 16);
    let r_ = 'f: {
        if (((((c.m.r32(sym::gEftLink)) == (((0i32) as u32))) as i32)) != 0) {
            break 'f;
        }
        if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
            break 'f;
        }
        if ((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) != (fid::EftLink_Update)) as i32)) != 0) {
            break 'f;
        }
        l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
        if (((((l_w_3) == (((0i32) as u32))) as i32)) != 0) {
            break 'f;
        }
        if (((((c.m.r32((l_w_3).wrapping_add(920)) as i32)) & (1i32))) != 0) {
            F::Vec4_Copy(c, ((l_w_3).wrapping_add(48)).wrapping_add(32), fp);
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftLink_SetPos2", &PROBE_N, &[], true); }
    r_
}

pub fn EftLink_Warp(c: &mut Ctx, mut p_task_1: u32, p_pos_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftLink_Warp", &PROBE_N, &[(p_task_1) as u32, (p_pos_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_x_3: F32 = F32(0);
    rt::memcpy(c, fp, p_pos_2, 16);
    let r_ = 'f: {
        l_x_3 = F32(c.m.r32(fp));
        if (((rt::flt(l_x_3, F32(0x00000000)) as i32)) != 0) {
            l_x_3 = F32(0x00000000);
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftLink_Warp", &PROBE_N, &[], true); }
    r_
}

pub fn EftLink_SetDir(c: &mut Ctx, mut p_task_1: u32, p_dir_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftLink_SetDir", &PROBE_N, &[(p_task_1) as u32, (p_dir_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_x_3: F32 = F32(0);
    rt::memcpy(c, fp, p_dir_2, 16);
    let r_ = 'f: {
        l_x_3 = F32(c.m.r32(fp));
        if (((rt::flt(l_x_3, F32(0x00000000)) as i32)) != 0) {
            l_x_3 = F32(0x00000000);
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftLink_SetDir", &PROBE_N, &[], true); }
    r_
}

pub fn EftLink_SetSize(c: &mut Ctx, mut p_task_1: u32, mut p_size_2: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftLink_SetSize", &PROBE_N, &[(p_task_1) as u32, (p_size_2).0], false); }
    let mut l_w_3: u32 = 0;
    let r_ = 'f: {
    if (((((c.m.r32(sym::gEftLink)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    if ((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) != (fid::EftLink_Update)) as i32)) != 0) {
        break 'f;
    }
    l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
    if (((((l_w_3) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    if (((((c.m.r32((l_w_3).wrapping_add(920)) as i32)) & (1i32))) != 0) {
        { let v_ = p_size_2; let a_ = ((l_w_3).wrapping_add(48)).wrapping_add(48); c.m.w32(a_, (v_).0); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftLink_SetSize", &PROBE_N, &[], true); }
    r_
}

pub fn EftLink_SetRate(c: &mut Ctx, mut p_task_1: u32, mut p_rate_2: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftLink_SetRate", &PROBE_N, &[(p_task_1) as u32, (p_rate_2).0], false); }
    let mut l_w_3: u32 = 0;
    let r_ = 'f: {
    if (((((c.m.r32(sym::gEftLink)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    if ((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) != (fid::EftLink_Update)) as i32)) != 0) {
        break 'f;
    }
    l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
    if (((((l_w_3) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    if (((((c.m.r32((l_w_3).wrapping_add(920)) as i32)) & (1i32))) != 0) {
        { let v_ = p_rate_2; let a_ = ((l_w_3).wrapping_add(48)).wrapping_add(52); c.m.w32(a_, (v_).0); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftLink_SetRate", &PROBE_N, &[], true); }
    r_
}

pub fn EftLink_SetTex(c: &mut Ctx, mut p_task_1: u32, mut p_tex_2: u32, mut p_a_3: i32, mut p_b_4: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftLink_SetTex", &PROBE_N, &[(p_task_1) as u32, (p_tex_2) as u32, (p_a_3) as u32, (p_b_4) as u32], false); }
    let mut l_w_5: u32 = 0;
    let r_ = 'f: {
    if (((((c.m.r32(sym::gEftLink)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    if ((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) == (fid::EftLink_Update)) as i32)) != 0) {
        l_w_5 = c.m.r32((p_task_1).wrapping_add(56));
        if (((((((((l_w_5) != (((0i32) as u32))) as i32)) != 0) && (((((c.m.r32((l_w_5).wrapping_add(920)) as i32)) & (1i32))) != 0)) as i32)) != 0) {
            { let v_ = p_tex_2; let a_ = ((l_w_5).wrapping_add(48)).wrapping_add(8); c.m.w32(a_, v_); };
            F::EftLink_SelectTex(c, l_w_5, p_tex_2, p_a_3, p_b_4);
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftLink_SetTex", &PROBE_N, &[], true); }
    r_
}

pub fn EftLink_SetDelay(c: &mut Ctx, mut p_task_1: u32, mut p_v_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftLink_SetDelay", &PROBE_N, &[(p_task_1) as u32, (p_v_2) as u32], false); }
    let mut l_w_3: u32 = 0;
    let r_ = 'f: {
    if (((((c.m.r32(sym::gEftLink)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    if ((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) != (fid::EftLink_Update)) as i32)) != 0) {
        break 'f;
    }
    l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
    if (((((l_w_3) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    if (((((c.m.r32((l_w_3).wrapping_add(920)) as i32)) & (1i32))) != 0) {
        { let v_ = rt::fdiv(rt::i2f(p_v_2), F32(0x41f00000)); let a_ = (l_w_3).wrapping_add(600); c.m.w32(a_, (v_).0); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftLink_SetDelay", &PROBE_N, &[], true); }
    r_
}

pub fn EftLink_SetStopDelay(c: &mut Ctx, mut p_task_1: u32, mut p_v_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftLink_SetStopDelay", &PROBE_N, &[(p_task_1) as u32, (p_v_2) as u32], false); }
    let mut l_w_3: u32 = 0;
    let r_ = 'f: {
    if (((((c.m.r32(sym::gEftLink)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    if ((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) != (fid::EftLink_Update)) as i32)) != 0) {
        break 'f;
    }
    l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
    if (((((l_w_3) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    if (((((c.m.r32((l_w_3).wrapping_add(920)) as i32)) & (1i32))) != 0) {
        { let v_ = rt::i2f(p_v_2); let a_ = (l_w_3).wrapping_add(604); c.m.w32(a_, (v_).0); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftLink_SetStopDelay", &PROBE_N, &[], true); }
    r_
}

pub fn EftLink_SetFlag200(c: &mut Ctx, mut p_task_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftLink_SetFlag200", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
    if (((((c.m.r32(sym::gEftLink)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) != (fid::EftLink_Update)) as i32)) != 0) {
        break 'f 0i32;
    }
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    if (((((l_w_2) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if (((((c.m.r32((l_w_2).wrapping_add(920)) as i32)) & (1i32))) != 0) {
        { let r_ = 512i32; let a_ = (l_w_2).wrapping_add(920); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftLink_SetFlag200", &PROBE_N, &[], true); }
    r_
}

pub fn EftLink_SetType(c: &mut Ctx, mut p_task_1: u32, mut p_v_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftLink_SetType", &PROBE_N, &[(p_task_1) as u32, (p_v_2) as u32], false); }
    let mut l_w_3: u32 = 0;
    let r_ = 'f: {
    if (((((c.m.r32(sym::gEftLink)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) != (fid::EftLink_Update)) as i32)) != 0) {
        break 'f 0i32;
    }
    l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
    if (((((l_w_3) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if (((((c.m.r32((l_w_3).wrapping_add(920)) as i32)) & (1i32))) != 0) {
        { let v_ = p_v_2; let a_ = (l_w_3).wrapping_add(912); c.m.w32(a_, (v_) as u32); };
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftLink_SetType", &PROBE_N, &[], true); }
    r_
}

pub fn EftLink_IsAlive(c: &mut Ctx, mut p_task_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftLink_IsAlive", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
    if (((((c.m.r32(sym::gEftLink)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) != (fid::EftLink_Update)) as i32)) != 0) {
        break 'f 0i32;
    }
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    if (((((l_w_2) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if (((((c.m.r32((l_w_2).wrapping_add(920)) as i32)) & (1i32))) != 0) {
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftLink_IsAlive", &PROBE_N, &[], true); }
    r_
}

pub fn EftLink_IsCornerOffScreen(c: &mut Ctx, mut p_x_1: i32, mut p_y_2: i32, mut p_z_3: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftLink_IsCornerOffScreen", &PROBE_N, &[(p_x_1) as u32, (p_y_2) as u32, (p_z_3) as u32], false); }
    let r_ = 'f: {
    if (((((p_z_3) <= (0i32)) as i32)) != 0) {
        break 'f 1i32;
    }
    if (((((p_x_1) > (65519i32)) as i32)) != 0) {
        break 'f 1i32;
    }
    if (((((p_x_1) <= (0i32)) as i32)) != 0) {
        break 'f 1i32;
    }
    if (((((p_y_2) > (65519i32)) as i32)) != 0) {
        break 'f 1i32;
    }
    if (((((p_y_2) <= (0i32)) as i32)) != 0) {
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftLink_IsCornerOffScreen", &PROBE_N, &[], true); }
    r_
}

pub fn EftPart10Mgr_Init(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftPart10Mgr_Init", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = { let a0_ = F::BtlPool_GetCurrent(c); F::BtlPool_Alloc(c, a0_, ((53488u32) as i32)) }; let a_ = sym::gEftPart10Mgr; c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32(sym::gEftPart10Mgr); F::memset(c, a0_, 0i32, 53488u32) }; };
    { let v_ = F::BtlTask_CreateChildList(c, p_task_1, ((((12i32) as i64 as u64)) as u32), ((((((976u32) as i32)) as i64 as u64)) as u32)); let a_ = c.m.r32(sym::gEftPart10Mgr); c.m.w32(a_, v_); };
    { let a0_ = (c.m.r32(sym::gEftPart10Mgr)).wrapping_add(53424); F::Mtx_StoreIdentity(c, a0_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftPart10Mgr_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftPart10Mgr_Update(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftPart10Mgr_Update", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftPart10Mgr_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftPart10Mgr_Reset(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftPart10Mgr_Reset", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftPart10Mgr_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn EftPart10Mgr_Term(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftPart10Mgr_Term", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    { let a0_ = F::BtlPool_GetCurrent(c); let a1_ = c.m.r32(sym::gEftPart10Mgr); F::BtlPool_Free(c, a0_, a1_) };
    { let v_ = ((0i32) as u32); let a_ = sym::gEftPart10Mgr; c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftPart10Mgr_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftPart10_Init(c: &mut Ctx, mut p_task_1: u32, mut p_arg_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftPart10_Init", &PROBE_N, &[(p_task_1) as u32, (p_arg_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_w_3: u32 = 0;
    let mut l_def_4: u32 = 0;
    let mut l_i_5: i32 = 0;
    let mut l_j_6: i32 = 0;
    let mut l_n_7: i32 = 0;
    let r_ = 'f: {
        l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
        l_def_4 = c.m.r32(p_arg_2);
        { let _ = F::memset(c, l_w_3, 0i32, 976u32); };
        { let s_ = p_arg_2; let d_ = (l_w_3).wrapping_add(48); rt::memcpy(c, d_, s_, 80); };
        { let v_ = c.m.r32(p_arg_2); let a_ = (l_w_3).wrapping_add(48); c.m.w32(a_, v_); };
        { let v_ = c.m.r32((p_arg_2).wrapping_add(4)); let a_ = ((l_w_3).wrapping_add(48)).wrapping_add(4); c.m.w32(a_, v_); };
        { let v_ = c.m.r32((p_arg_2).wrapping_add(8)); let a_ = ((l_w_3).wrapping_add(48)).wrapping_add(8); c.m.w32(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = (l_w_3).wrapping_add(964); c.m.w32(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = (l_w_3).wrapping_add(968); c.m.w32(a_, v_); };
        if (((((c.m.r32((l_def_4).wrapping_add(500)) as i32)) & (64i32))) != 0) {
            F::EftPart10_InitSpin(c, l_w_3);
        } else {
            { let v_ = F32(0x00000000); let a_ = (l_w_3).wrapping_add(600); c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x3f800000); let a_ = (l_w_3).wrapping_add(616); c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x00000000); let a_ = (l_w_3).wrapping_add(604); c.m.w32(a_, (v_).0); };
            { let v_ = F32(c.m.r32((l_def_4).wrapping_add(460))); let a_ = (l_w_3).wrapping_add(620); c.m.w32(a_, (v_).0); };
        }
        if (((((c.m.r32((l_def_4).wrapping_add(500)) as i32)) & (512i32))) != 0) {
            F::EftPart10_StartKeys(c, l_w_3);
            { let r_ = 262144i32; let a_ = (l_w_3).wrapping_add(960); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        } else {
            F::EftPart10_SetKey(c, l_w_3, 2i32);
        }
        { let _ = F::memset(c, fp, 0i32, 8u32); };
        if (((((((((((c.m.r8((l_def_4).wrapping_add(504))) as i32)) >= (2i32)) as i32)) != 0) || (((((((c.m.r8((l_def_4).wrapping_add(505))) as i32)) >= (2i32)) as i32)) != 0)) as i32)) != 0) {
            { let v_ = rt::i2f((((c.m.r8((l_def_4).wrapping_add(504))) as i32)).wrapping_mul(((c.m.r8((l_def_4).wrapping_add(505))) as i32))); let a_ = (l_w_3).wrapping_add(924); c.m.w32(a_, (v_).0); };
            { let v_ = rt::fdiv(F32(0x3f800000), rt::i2f(((c.m.r8((l_def_4).wrapping_add(504))) as i32))); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            { let v_ = rt::fdiv(F32(0x3f800000), rt::i2f(((c.m.r8((l_def_4).wrapping_add(505))) as i32))); let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            l_n_7 = 0i32;
            {
                l_j_6 = 0i32;
                'l1: loop {
                    if !(((((l_j_6) < (((c.m.r8((l_def_4).wrapping_add(505))) as i32))) as i32)) != 0) { break 'l1; }
                    'c2: {
                        {
                            l_i_5 = 0i32;
                            'l3: loop {
                                if !(((((l_i_5) < (((c.m.r8((l_def_4).wrapping_add(504))) as i32))) as i32)) != 0) { break 'l3; }
                                'c4: {
                                    { let v_ = rt::fmul(F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), rt::i2f(l_i_5)); let a_ = (((l_w_3).wrapping_add(668)).wrapping_add((((l_n_7) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                                    { let v_ = rt::fmul(F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), rt::i2f(l_j_6)); let a_ = (((l_w_3).wrapping_add(668)).wrapping_add((((l_n_7) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                                    { let v_ = rt::fadd(rt::fmul(F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), rt::i2f(l_i_5)), F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a_ = (((l_w_3).wrapping_add(668)).wrapping_add((((l_n_7) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                                    { let v_ = rt::fadd(rt::fmul(F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), rt::i2f(l_j_6)), F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); let a_ = (((l_w_3).wrapping_add(668)).wrapping_add((((l_n_7) as u32)).wrapping_mul(16))).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                                    { let o_ = l_n_7; let n_ = (o_).wrapping_add(1); l_n_7 = n_; };
                                }
                                { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
                            }
                        }
                    }
                    { let o_ = l_j_6; let n_ = (o_).wrapping_add(1); l_j_6 = n_; };
                }
            }
            { let r_ = 256i32; let a_ = (l_w_3).wrapping_add(960); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        }
        { let v_ = { let a0_ = rt::fmul(F32(c.m.r32((l_def_4).wrapping_add(520))), F32(0x40490fda)); F::EftMath_WrapAngle(c, a0_) }; let a_ = (l_w_3).wrapping_add(664); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = rt::fmul(F32(c.m.r32((l_def_4).wrapping_add(516))), F32(0x40490fda)); F::EftMath_WrapAngle(c, a0_) }; let a_ = (l_w_3).wrapping_add(660); c.m.w32(a_, (v_).0); };
        { let a1_ = c.m.r32(((l_w_3).wrapping_add(48)).wrapping_add(8)); let a2_ = (c.m.r32(((l_w_3).wrapping_add(48)).wrapping_add(56)) as i32); let a3_ = (c.m.r32(((l_w_3).wrapping_add(48)).wrapping_add(56)) as i32); F::EftPart10_SelectTex(c, l_w_3, a1_, a2_, a3_) };
        { let r_ = 65i32; let a_ = (l_w_3).wrapping_add(960); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftPart10_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftPart10_Update(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftPart10_Update", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let mut l_arg_3: u32 = 0;
    let mut l_def_4: u32 = 0;
    let mut l_alpha_5: F32 = F32(0);
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    l_arg_3 = (l_w_2).wrapping_add(48);
    l_def_4 = c.m.r32(l_arg_3);
    l_alpha_5 = F32(0x3f800000);
    if ((((!(({ let a0_ = (c.m.r32((l_arg_3).wrapping_add(64)) as i32); let a1_ = ((c.m.r8((l_w_2).wrapping_add(957))) as i32); F::BtlScene_IsEffectStopped(c, a0_, a1_) }) != 0)) as i32)) != 0) {
        if (((rt::flt(F32(0x00000000), F32(c.m.r32((l_w_2).wrapping_add(932)))) as i32)) != 0) {
            { let r_ = F32(0x3f800000); let a_ = (l_w_2).wrapping_add(932); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        } else {
            if (((((((((c.m.r32((l_w_2).wrapping_add(960)) as i32)) & (16i32))) != 0) && ((((!(((((c.m.r32((l_w_2).wrapping_add(960)) as i32)) & (8i32))) != 0)) as i32)) != 0)) as i32)) != 0) {
                l_alpha_5 = rt::fdiv(F32(c.m.r32((l_w_2).wrapping_add(940))), F32(c.m.r32((l_w_2).wrapping_add(944))));
                if ((((!(((rt::flt(F32(c.m.r32((l_w_2).wrapping_add(940))), F32(0x00000000)) as i32)) != 0)) as i32)) != 0) {
                    { let r_ = F32(0x3f800000); let a_ = (l_w_2).wrapping_add(940); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                } else {
                    { let v_ = (((((c.m.r32((l_w_2).wrapping_add(960)) as i32)) & ((!(1i32))))) | (2i32)); let a_ = (l_w_2).wrapping_add(960); c.m.w32(a_, (v_) as u32); };
                }
            }
            if (((((c.m.r32((l_w_2).wrapping_add(960)) as i32)) & (262144i32))) != 0) {
                F::EftPart10_UpdateKeys(c, l_w_2);
            }
            if (((((c.m.r32((l_def_4).wrapping_add(500)) as i32)) & (64i32))) != 0) {
                F::EftPart10_UpdateSpin(c, l_w_2);
            }
            if ((((!(((((c.m.r32((l_w_2).wrapping_add(960)) as i32)) & (4i32))) != 0)) as i32)) != 0) {
                F::EftPart10_EmitRings(c, l_w_2);
            }
            F::EftPart10_Step(c, l_alpha_5, l_w_2);
            { let r_ = F32(0x3f800000); let a_ = (l_w_2).wrapping_add(928); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            if (((((c.m.r32((l_w_2).wrapping_add(960)) as i32)) & (262144i32))) != 0) {
                F::EftPart10_StartKeys(c, l_w_2);
                if (((rt::fge(F32(c.m.r32((l_w_2).wrapping_add(928))), F32(c.m.r32((l_w_2).wrapping_add(384)))) as i32)) != 0) {
                    F::EftPart10_SetKey(c, l_w_2, 2i32);
                    { let r_ = (!(262144i32)); let a_ = (l_w_2).wrapping_add(960); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                }
            }
            if (((((((rt::fle(rt::fmul(F32(c.m.r32((l_arg_3).wrapping_add(52))), F32(0x41f00000)), F32(c.m.r32((l_w_2).wrapping_add(928)))) as i32)) != 0) && (((rt::flt(F32(0x00000000), F32(c.m.r32((l_arg_3).wrapping_add(52)))) as i32)) != 0)) as i32)) != 0) {
                if (((rt::flt(F32(0x00000000), F32(c.m.r32((l_w_2).wrapping_add(944)))) as i32)) != 0) {
                    { let r_ = 524304i32; let a_ = (l_w_2).wrapping_add(960); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                } else {
                    { let r_ = 4i32; let a_ = (l_w_2).wrapping_add(960); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                }
            }
            if (((((c.m.r32((l_w_2).wrapping_add(960)) as i32)) & (8i32))) != 0) {
                if (((rt::flt(F32(0x00000000), F32(c.m.r32((l_w_2).wrapping_add(936)))) as i32)) != 0) {
                    { let r_ = F32(0x3f800000); let a_ = (l_w_2).wrapping_add(936); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                } else {
                    { let r_ = (!(8i32)); let a_ = (l_w_2).wrapping_add(960); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                    if ((((!(((((c.m.r32((l_w_2).wrapping_add(960)) as i32)) & (16i32))) != 0)) as i32)) != 0) {
                        { let r_ = 4i32; let a_ = (l_w_2).wrapping_add(960); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    }
                }
            }
            if (((((c.m.r32((l_w_2).wrapping_add(964))) == (((0i32) as u32))) as i32)) != 0) {
                if (((((c.m.r32((l_w_2).wrapping_add(960)) as i32)) & (4i32))) != 0) {
                    { let r_ = 2i32; let a_ = (l_w_2).wrapping_add(960); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                }
            }
            if (((((((((c.m.r32((l_w_2).wrapping_add(960)) as i32)) & (32i32))) != 0) || (((((c.m.r32((l_w_2).wrapping_add(960)) as i32)) & (2i32))) != 0)) as i32)) != 0) {
                { let r_ = (!(1i32)); let a_ = (l_w_2).wrapping_add(960); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                F::BtlTask_SetDead(c, p_task_1);
            }
        }
    }
    if ((((!(((((c.m.r32((l_w_2).wrapping_add(960)) as i32)) & (32i32))) != 0)) as i32)) != 0) {
        if ((((!(((((c.m.r32((l_w_2).wrapping_add(960)) as i32)) & (2i32))) != 0)) as i32)) != 0) {
            { let a1_ = c.m.r32(((l_w_2).wrapping_add(48)).wrapping_add(8)); F::EftPart10_BuildTex(c, l_w_2, a1_) };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftPart10_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftPart10_PostUpdate(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftPart10_PostUpdate", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftPart10_PostUpdate", &PROBE_N, &[], true); }
    r_
}

pub fn EftPart10_Draw(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftPart10_Draw", &PROBE_N, &[(p_task_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_w_2: u32 = 0;
    let mut l_def_3: u32 = 0;
    let mut l_grp_4: u32 = 0;
    let mut l_p_5: u32 = 0;
    let mut l_half_6: F32 = F32(0);
    let r_ = 'f: {
        l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
        l_def_3 = c.m.r32((l_w_2).wrapping_add(48));
        { let _ = F::memset(c, fp, 0i32, 16u32); };
        { let v_ = F32(0x3f800000); let a_ = (fp).wrapping_add(12); c.m.w32(a_, (v_).0); };
        F::Vu0Cur_Push(c);
        { let a0_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(320); F::Vu0Cur_LoadMtx(c, a0_) };
        {
            l_grp_4 = c.m.r32((l_w_2).wrapping_add(964));
            'l1: loop {
                if !(((((l_grp_4) != (((0i32) as u32))) as i32)) != 0) { break 'l1; }
                'c2: {
                    {
                        l_p_5 = c.m.r32((l_grp_4).wrapping_add(4));
                        'l3: loop {
                            if !(((((l_p_5) != (((0i32) as u32))) as i32)) != 0) { break 'l3; }
                            'c4: {
                                if (((((c.m.r32((l_p_5).wrapping_add(320)) as i32)) & (64i32))) != 0) {
                                    if (((((((c.m.r8((l_def_3).wrapping_add(524))) as i32)) == (1i32)) as i32)) != 0) {
                                        l_half_6 = rt::fmul(rt::fmul(rt::fmul(rt::fmul(F32(c.m.r32((l_p_5).wrapping_add(224))), F32(c.m.r32(((l_w_2).wrapping_add(48)).wrapping_add(48)))), F32(c.m.r32((l_w_2).wrapping_add(616)))), F32(c.m.r32((l_p_5).wrapping_add(244)))), F32(0x3f000000));
                                        F::Vec3_Add(c, fp, l_p_5, ((l_w_2).wrapping_add(48)).wrapping_add(16));
                                        if (((((c.m.r32((l_def_3).wrapping_add(500)) as i32)) & (8i32))) != 0) {
                                            { let a5_ = F32(c.m.r32((l_p_5).wrapping_add(160))); let a6_ = F32(c.m.r32(((l_p_5).wrapping_add(160)).wrapping_add(4))); let a7_ = F32(c.m.r32(((l_p_5).wrapping_add(176)).wrapping_add(8))); let a8_ = F32(c.m.r32(((l_p_5).wrapping_add(176)).wrapping_add(12))); let a9_ = F32(c.m.r32(((l_p_5).wrapping_add(200)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); let a10_ = ((c.m.r8((l_def_3).wrapping_add(507))) as i32); let a11_ = ((((c.m.r32((l_w_2).wrapping_add(960)) as i32)).wrapping_shr((10i32) as u32)) & (1i32)); let a12_ = { let a_ = (c.m.r32(((l_w_2).wrapping_add(48)).wrapping_add(8))).wrapping_add((((((c.m.r8((l_w_2).wrapping_add(956))) as i32)).wrapping_shl((4i32) as u32)) as u32)); c.m.r64(a_) }; F::EftPart10_DrawBillboardClipped(c, fp, l_half_6, l_half_6, (l_p_5).wrapping_add(80), (l_p_5).wrapping_add(32), a5_, a6_, a7_, a8_, a9_, a10_, a11_, a12_, F32(0x40000000)) };
                                        } else {
                                            { let a3_ = (rt::f2i(F32(c.m.r32((l_p_5).wrapping_add(32))))).wrapping_shl((4i32) as u32); let a4_ = (rt::f2i(F32(c.m.r32(((l_p_5).wrapping_add(32)).wrapping_add(4))))).wrapping_shl((4i32) as u32); let a6_ = F32(c.m.r32((l_p_5).wrapping_add(160))); let a7_ = F32(c.m.r32(((l_p_5).wrapping_add(160)).wrapping_add(4))); let a8_ = ((c.m.r8((l_def_3).wrapping_add(507))) as i32); let a9_ = F32(c.m.r32(((l_p_5).wrapping_add(176)).wrapping_add(8))); let a10_ = F32(c.m.r32(((l_p_5).wrapping_add(176)).wrapping_add(12))); let a11_ = ((((c.m.r32((l_w_2).wrapping_add(960)) as i32)).wrapping_shr((10i32) as u32)) & (1i32)); let a12_ = { let a_ = (c.m.r32(((l_w_2).wrapping_add(48)).wrapping_add(8))).wrapping_add((((((c.m.r8((l_w_2).wrapping_add(956))) as i32)).wrapping_shl((4i32) as u32)) as u32)); c.m.r64(a_) }; let a13_ = F32(c.m.r32(((l_p_5).wrapping_add(200)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); F::EftPart10_DrawBillboard(c, fp, (l_p_5).wrapping_add(80), rt::fmul(l_half_6, F32(0x41800000)), a3_, a4_, rt::fmul(l_half_6, F32(0x41800000)), a6_, a7_, a8_, a9_, a10_, a11_, a12_, a13_) };
                                        }
                                    } else {
                                        if (((((c.m.r32((l_def_3).wrapping_add(500)) as i32)) & (8i32))) != 0) {
                                            { let a4_ = ((c.m.r8((l_def_3).wrapping_add(507))) as i32); let a5_ = ((c.m.r8((l_w_2).wrapping_add(956))) as i32); let a6_ = ((((c.m.r32((l_w_2).wrapping_add(960)) as i32)).wrapping_shr((10i32) as u32)) & (1i32)); let a7_ = c.m.r32(((l_w_2).wrapping_add(48)).wrapping_add(8)); F::EftPart10_DrawQuadClipped(c, ((l_p_5).wrapping_add(96)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (l_p_5).wrapping_add(160), (l_p_5).wrapping_add(176), (l_p_5).wrapping_add(80), a4_, a5_, a6_, a7_) };
                                        } else {
                                            { let a4_ = ((c.m.r8((l_def_3).wrapping_add(507))) as i32); let a5_ = ((c.m.r8((l_w_2).wrapping_add(956))) as i32); let a6_ = ((((c.m.r32((l_w_2).wrapping_add(960)) as i32)).wrapping_shr((10i32) as u32)) & (1i32)); let a7_ = c.m.r32(((l_w_2).wrapping_add(48)).wrapping_add(8)); F::EftPart10_DrawQuad(c, ((l_p_5).wrapping_add(96)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (l_p_5).wrapping_add(160), (l_p_5).wrapping_add(176), (l_p_5).wrapping_add(80), a4_, a5_, a6_, a7_) };
                                        }
                                    }
                                }
                            }
                            l_p_5 = c.m.r32((l_p_5).wrapping_add(324));
                        }
                    }
                }
                l_grp_4 = c.m.r32((l_grp_4).wrapping_add(12));
            }
        }
        F::Vu0Cur_Pop(c);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftPart10_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn EftPart10_Reset(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftPart10_Reset", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    F::BtlTask_SetDead(c, p_task_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftPart10_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn EftPart10_Term(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftPart10_Term", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let mut l_grp_3: u32 = 0;
    let mut l_p_4: u32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    {
        l_grp_3 = c.m.r32((l_w_2).wrapping_add(964));
        'l1: loop {
            if !(((((l_grp_3) != (((0i32) as u32))) as i32)) != 0) { break 'l1; }
            'c2: {
                {
                    l_p_4 = c.m.r32((l_grp_3).wrapping_add(4));
                    'l3: loop {
                        if !(((((l_p_4) != (((0i32) as u32))) as i32)) != 0) { break 'l3; }
                        'c4: {
                            { let v_ = 0i32; let a_ = (l_p_4).wrapping_add(320); c.m.w32(a_, (v_) as u32); };
                            F::EftPart10_UnlinkPtcl(c, (l_grp_3).wrapping_add(4), (l_grp_3).wrapping_add(8), l_p_4);
                        }
                        l_p_4 = c.m.r32((l_p_4).wrapping_add(324));
                    }
                }
                if (((((c.m.r32((l_grp_3).wrapping_add(4))) == (((0i32) as u32))) as i32)) != 0) {
                    { let v_ = 0i32; let a_ = l_grp_3; c.m.w32(a_, (v_) as u32); };
                    F::EftPart10_UnlinkGroup(c, (l_w_2).wrapping_add(964), (l_w_2).wrapping_add(968), l_grp_3);
                }
            }
            l_grp_3 = c.m.r32((l_grp_3).wrapping_add(12));
        }
    }
    { let v_ = 0i32; let a_ = (l_w_2).wrapping_add(960); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftPart10_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftPart10_EmitRings(c: &mut Ctx, mut p_w_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftPart10_EmitRings", &PROBE_N, &[(p_w_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_def_2: u32 = 0;
    let mut l_grp_3: u32 = 0;
    let mut l_step_4: F32 = F32(0);
    let mut l_i_5: i32 = 0;
    let mut l_j_6: i32 = 0;
    let mut l_a_7: F32 = F32(0);
    let mut l_b_8: F32 = F32(0);
    let r_ = 'f: {
        l_def_2 = c.m.r32((p_w_1).wrapping_add(48));
        l_grp_3 = c.m.r32((p_w_1).wrapping_add(964));
        l_step_4 = rt::fmul(rt::fdiv(F32(0x3f800000), F32(c.m.r32((p_w_1).wrapping_add(620)))), F32(0x40c90fda));
        { let v_ = rt::f2i(rt::fmul(F32(c.m.r32((p_w_1).wrapping_add(620))), F32(c.m.r32((l_def_2).wrapping_add(508))))); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = rt::f2i(rt::fmul(F32(c.m.r32((p_w_1).wrapping_add(620))), F32(c.m.r32((l_def_2).wrapping_add(512))))); let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = F32(0x00000000); let a_ = (p_w_1).wrapping_add(656); c.m.w32(a_, (v_).0); };
        {
            l_i_5 = 0i32;
            'l1: loop {
                if !(((((l_i_5) < ((rt::f2i(F32(c.m.r32((p_w_1).wrapping_add(620))))).wrapping_add(1i32))) as i32)) != 0) { break 'l1; }
                'c2: {
                    if (((((((((l_i_5) >= ((c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32))) as i32)) != 0) && (((((l_i_5) <= ((c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32))) as i32)) != 0)) as i32)) != 0) {
                        {
                            l_j_6 = 0i32;
                            'l3: loop {
                                if !(((((l_j_6) < (rt::f2i(F32(c.m.r32((p_w_1).wrapping_add(620)))))) as i32)) != 0) { break 'l3; }
                                'c4: {
                                    if (((((l_grp_3) == (((0i32) as u32))) as i32)) != 0) {
                                        l_grp_3 = F::EftPart10_NewGroup(c, p_w_1);
                                        if (((((l_grp_3) == (((0i32) as u32))) as i32)) != 0) {
                                            break 'c4;
                                        }
                                    }
                                    if (((((c.m.r32((l_grp_3).wrapping_add(4))) == (((0i32) as u32))) as i32)) != 0) {
                                        { let r_ = rt::fmul(F32(c.m.r32((l_def_2).wrapping_add(384))), F32(0x40490fda)); let a_ = (p_w_1).wrapping_add(656); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                                        l_a_7 = { let a0_ = rt::fadd(rt::fmul(l_step_4, rt::i2f(l_i_5)), rt::fmul({ let a0_ = F32(c.m.r32((p_w_1).wrapping_add(240))); let a1_ = rt::fadd(F32(c.m.r32((p_w_1).wrapping_add(240))), F32(c.m.r32((p_w_1).wrapping_add(248)))); F::Rand_FloatRange(c, a0_, a1_) }, F32(0x40490fda))); F::EftMath_WrapAngle(c, a0_) };
                                        l_b_8 = { let a0_ = rt::fadd(rt::fmul(l_step_4, rt::i2f(l_j_6)), rt::fmul({ let a0_ = F32(c.m.r32((p_w_1).wrapping_add(256))); let a1_ = rt::fadd(F32(c.m.r32((p_w_1).wrapping_add(256))), F32(c.m.r32((p_w_1).wrapping_add(264)))); F::Rand_FloatRange(c, a0_, a1_) }, F32(0x40490fda))); F::EftMath_WrapAngle(c, a0_) };
                                        F::EftPart10_Emit(c, p_w_1, l_grp_3, l_a_7, l_b_8);
                                    }
                                    { let r_ = 2048i32; let a_ = l_grp_3; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                                    l_grp_3 = c.m.r32((l_grp_3).wrapping_add(12));
                                }
                                { let o_ = l_j_6; let n_ = (o_).wrapping_add(1); l_j_6 = n_; };
                            }
                        }
                    }
                }
                { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftPart10_EmitRings", &PROBE_N, &[], true); }
    r_
}

pub fn EftPart10_Step(c: &mut Ctx, mut p_alpha_1: F32, mut p_w_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftPart10_Step", &PROBE_N, &[(p_alpha_1).0, (p_w_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_def_3: u32 = 0;
    let mut l_yaw_4: F32 = F32(0);
    let mut l_pitch_5: F32 = F32(0);
    let mut l_grp_6: u32 = 0;
    let mut l_p_7: u32 = 0;
    let mut l_t_8: F32 = F32(0);
    let mut l_a_9: F32 = F32(0);
    let mut l_u_10: F32 = F32(0);
    let mut l_u2_11: F32 = F32(0);
    let mut l_v_12: F32 = F32(0);
    let r_ = 'f: {
        rt::memset(c, fp, 0, 12);
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(0), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(4), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(8), (v_).0); }
        l_def_3 = c.m.r32((p_w_2).wrapping_add(48));
        l_yaw_4 = F32(0x00000000);
        l_pitch_5 = F32(0x00000000);
        if (((((c.m.r32((l_def_3).wrapping_add(500)) as i32)) & (1i32))) != 0) {
            l_pitch_5 = { let a0_ = { let a0_ = rt::fneg(F32(c.m.r32((((p_w_2).wrapping_add(48)).wrapping_add(32)).wrapping_add(4)))); F::Mathf_Asin(c, a0_) }; F::EftMath_WrapAngle(c, a0_) };
            l_yaw_4 = { let a0_ = { let a0_ = F32(c.m.r32(((p_w_2).wrapping_add(48)).wrapping_add(32))); let a1_ = F32(c.m.r32((((p_w_2).wrapping_add(48)).wrapping_add(32)).wrapping_add(8))); F::Port_atan2f(c, a0_, a1_) }; F::EftMath_WrapAngle(c, a0_) };
        }
        {
            l_grp_6 = c.m.r32((p_w_2).wrapping_add(964));
            'l1: loop {
                if !(((((l_grp_6) != (((0i32) as u32))) as i32)) != 0) { break 'l1; }
                'c2: {
                    {
                        l_p_7 = c.m.r32((l_grp_6).wrapping_add(4));
                        'l3: loop {
                            if !(((((l_p_7) != (((0i32) as u32))) as i32)) != 0) { break 'l3; }
                            'c4: {
                                if (((rt::flt(F32(0x00000000), F32(c.m.r32((l_p_7).wrapping_add(308)))) as i32)) != 0) {
                                    { let r_ = F32(0x3f800000); let a_ = (l_p_7).wrapping_add(308); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                                    break 'c4;
                                }
                                l_t_8 = rt::fdiv(F32(c.m.r32((l_p_7).wrapping_add(316))), F32(c.m.r32((l_p_7).wrapping_add(312))));
                                l_u_10 = F32(0x00000000);
                                if (((((c.m.r32((l_def_3).wrapping_add(500)) as i32)) & (256i32))) != 0) {
                                    if (((rt::flt(F32(0x00000000), F32(c.m.r32((l_p_7).wrapping_add(288)))) as i32)) != 0) {
                                        l_u_10 = (if (((rt::flt(rt::fdiv(F32(c.m.r32((l_p_7).wrapping_add(284))), F32(c.m.r32((l_p_7).wrapping_add(288)))), F32(0x00000000)) as i32)) != 0) { F32(0x00000000) } else { (if (((rt::flt(F32(0x3f800000), rt::fdiv(F32(c.m.r32((l_p_7).wrapping_add(284))), F32(c.m.r32((l_p_7).wrapping_add(288))))) as i32)) != 0) { F32(0x3f800000) } else { rt::fdiv(F32(c.m.r32((l_p_7).wrapping_add(284))), F32(c.m.r32((l_p_7).wrapping_add(288)))) }) });
                                    } else {
                                        l_u_10 = F32(0x3f800000);
                                    }
                                    if (((((c.m.r32((l_p_7).wrapping_add(320)) as i32)) & (8192i32))) != 0) {
                                        { let v_ = rt::fadd(F32(c.m.r32(((l_p_7).wrapping_add(272)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(((l_p_7).wrapping_add(260)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), l_u_10)); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                                    }
                                    if (((((c.m.r32((l_p_7).wrapping_add(320)) as i32)) & (16384i32))) != 0) {
                                        { let v_ = rt::fadd(F32(c.m.r32(((l_p_7).wrapping_add(272)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(((l_p_7).wrapping_add(260)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), l_u_10)); let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                                    }
                                    if (((((c.m.r32((l_p_7).wrapping_add(320)) as i32)) & (32768i32))) != 0) {
                                        { let v_ = rt::fadd(F32(c.m.r32(((l_p_7).wrapping_add(272)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(((l_p_7).wrapping_add(260)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), l_u_10)); let a_ = (fp).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                                    }
                                    if (((((c.m.r32((l_p_7).wrapping_add(320)) as i32)) & (65536i32))) != 0) {
                                        { let v_ = rt::fsub(F32(c.m.r32((l_p_7).wrapping_add(284))), F32(0x3f800000)); let a_ = (l_p_7).wrapping_add(284); c.m.w32(a_, (v_).0); };
                                    } else {
                                        { let v_ = rt::fadd(F32(c.m.r32((l_p_7).wrapping_add(284))), F32(0x3f800000)); let a_ = (l_p_7).wrapping_add(284); c.m.w32(a_, (v_).0); };
                                    }
                                    if (((((((rt::fge(F32(c.m.r32((l_p_7).wrapping_add(284))), F32(c.m.r32((l_p_7).wrapping_add(288)))) as i32)) != 0) || (((rt::fle(F32(c.m.r32((l_p_7).wrapping_add(284))), F32(0x00000000)) as i32)) != 0)) as i32)) != 0) {
                                        { let r_ = 65536i32; let a_ = (l_p_7).wrapping_add(320); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
                                        { let v_ = (if (((rt::flt(F32(c.m.r32((l_p_7).wrapping_add(284))), F32(0x00000000)) as i32)) != 0) { F32(0x00000000) } else { (if (((rt::flt(F32(c.m.r32((l_p_7).wrapping_add(288))), F32(c.m.r32((l_p_7).wrapping_add(284)))) as i32)) != 0) { F32(c.m.r32((l_p_7).wrapping_add(288))) } else { F32(c.m.r32((l_p_7).wrapping_add(284))) }) }); let a_ = (l_p_7).wrapping_add(284); c.m.w32(a_, (v_).0); };
                                    }
                                } else {
                                    { let v_ = F32(0x3f800000); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                                    { let v_ = F32(0x3f800000); let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                                    { let v_ = F32(0x3f800000); let a_ = (fp).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                                }
                                l_u2_11 = F32(0x00000000);
                                if (((((c.m.r32((l_def_3).wrapping_add(500)) as i32)) & (128i32))) != 0) {
                                    if (((rt::flt(F32(0x00000000), F32(c.m.r32((l_p_7).wrapping_add(256)))) as i32)) != 0) {
                                        l_u2_11 = (if (((rt::flt(rt::fdiv(F32(c.m.r32((l_p_7).wrapping_add(252))), F32(c.m.r32((l_p_7).wrapping_add(256)))), F32(0x00000000)) as i32)) != 0) { F32(0x00000000) } else { (if (((rt::flt(F32(0x3f800000), rt::fdiv(F32(c.m.r32((l_p_7).wrapping_add(252))), F32(c.m.r32((l_p_7).wrapping_add(256))))) as i32)) != 0) { F32(0x3f800000) } else { rt::fdiv(F32(c.m.r32((l_p_7).wrapping_add(252))), F32(c.m.r32((l_p_7).wrapping_add(256)))) }) });
                                    } else {
                                        l_u2_11 = F32(0x3f800000);
                                    }
                                    { let v_ = rt::fadd(F32(c.m.r32((l_p_7).wrapping_add(248))), rt::fmul(F32(c.m.r32((l_p_7).wrapping_add(300))), l_u2_11)); let a_ = (l_p_7).wrapping_add(244); c.m.w32(a_, (v_).0); };
                                    if (((((c.m.r32((l_p_7).wrapping_add(320)) as i32)) & (4096i32))) != 0) {
                                        { let v_ = rt::fsub(F32(c.m.r32((l_p_7).wrapping_add(252))), F32(0x3f800000)); let a_ = (l_p_7).wrapping_add(252); c.m.w32(a_, (v_).0); };
                                    } else {
                                        { let v_ = rt::fadd(F32(c.m.r32((l_p_7).wrapping_add(252))), F32(0x3f800000)); let a_ = (l_p_7).wrapping_add(252); c.m.w32(a_, (v_).0); };
                                    }
                                    if (((((((rt::fle(F32(c.m.r32((l_p_7).wrapping_add(256))), F32(c.m.r32((l_p_7).wrapping_add(252)))) as i32)) != 0) || (((rt::fle(F32(c.m.r32((l_p_7).wrapping_add(252))), F32(0x00000000)) as i32)) != 0)) as i32)) != 0) {
                                        { let r_ = 4096i32; let a_ = (l_p_7).wrapping_add(320); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
                                        { let v_ = (if (((rt::flt(F32(c.m.r32((l_p_7).wrapping_add(284))), F32(0x00000000)) as i32)) != 0) { F32(0x00000000) } else { (if (((rt::flt(F32(c.m.r32((l_p_7).wrapping_add(288))), F32(c.m.r32((l_p_7).wrapping_add(284)))) as i32)) != 0) { F32(c.m.r32((l_p_7).wrapping_add(288))) } else { F32(c.m.r32((l_p_7).wrapping_add(284))) }) }); let a_ = (l_p_7).wrapping_add(284); c.m.w32(a_, (v_).0); };
                                    }
                                }
                                { let r_ = F32(c.m.r32((l_p_7).wrapping_add(196))); let a_ = (l_p_7).wrapping_add(192); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                                { let r_ = F32(c.m.r32(((l_p_7).wrapping_add(212)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = ((l_p_7).wrapping_add(200)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                                { let r_ = F32(c.m.r32(((l_p_7).wrapping_add(212)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a_ = ((l_p_7).wrapping_add(200)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                                { let r_ = F32(c.m.r32(((l_p_7).wrapping_add(212)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); let a_ = ((l_p_7).wrapping_add(200)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                                { let v_ = { let a0_ = F32(c.m.r32(((l_p_7).wrapping_add(200)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); F::EftMath_WrapAngle(c, a0_) }; let a_ = ((l_p_7).wrapping_add(200)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                                { let v_ = { let a0_ = F32(c.m.r32(((l_p_7).wrapping_add(200)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); F::EftMath_WrapAngle(c, a0_) }; let a_ = ((l_p_7).wrapping_add(200)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                                { let v_ = { let a0_ = F32(c.m.r32(((l_p_7).wrapping_add(200)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); F::EftMath_WrapAngle(c, a0_) }; let a_ = ((l_p_7).wrapping_add(200)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                                { let a2_ = rt::fmul(rt::fmul(F32(c.m.r32((l_p_7).wrapping_add(192))), F32(c.m.r32(((p_w_2).wrapping_add(48)).wrapping_add(48)))), F32(c.m.r32((p_w_2).wrapping_add(616)))); F::Vec3_Scale(c, l_p_7, (l_p_7).wrapping_add(16), a2_) };
                                F::Vu0Cur_Push(c);
                                F::Vu0Cur_LoadIdentity(c);
                                { let a0_ = F32(c.m.r32((p_w_2).wrapping_add(660))); F::Vu0Cur_RotateX(c, a0_) };
                                { let a0_ = F32(c.m.r32((p_w_2).wrapping_add(664))); F::Vu0Cur_RotateY(c, a0_) };
                                { let a0_ = { let a0_ = rt::fadd(rt::fadd(F32(c.m.r32(((l_p_7).wrapping_add(200)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((p_w_2).wrapping_add(608)))), l_pitch_5); F::EftMath_WrapAngle(c, a0_) }; F::Vu0Cur_RotateX(c, a0_) };
                                { let a0_ = { let a0_ = rt::fadd(rt::fadd(F32(c.m.r32(((l_p_7).wrapping_add(200)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((p_w_2).wrapping_add(612)))), l_yaw_4); F::EftMath_WrapAngle(c, a0_) }; F::Vu0Cur_RotateY(c, a0_) };
                                F::Vu0Cur_MulVec4(c, l_p_7, l_p_7);
                                F::Vu0Cur_Pop(c);
                                if (((rt::flt(l_t_8, F32(c.m.r32((l_def_3).wrapping_add(496)))) as i32)) != 0) {
                                    { let r_ = F32(c.m.r32((l_p_7).wrapping_add(228))); let a_ = (l_p_7).wrapping_add(224); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                                } else {
                                    { let r_ = F32(c.m.r32((l_p_7).wrapping_add(232))); let a_ = (l_p_7).wrapping_add(224); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                                }
                                l_v_12 = { l_a_9 = F32(0x00000000); l_a_9 };
                                if (((((c.m.r32((l_p_7).wrapping_add(320)) as i32)) & (128i32))) != 0) {
                                    l_v_12 = (if (((rt::flt(rt::fdiv(F32(c.m.r32((l_p_7).wrapping_add(292))), F32(c.m.r32((l_p_7).wrapping_add(296)))), F32(0x00000000)) as i32)) != 0) { F32(0x00000000) } else { (if (((rt::flt(F32(0x3f800000), rt::fdiv(F32(c.m.r32((l_p_7).wrapping_add(292))), F32(c.m.r32((l_p_7).wrapping_add(296))))) as i32)) != 0) { F32(0x3f800000) } else { rt::fdiv(F32(c.m.r32((l_p_7).wrapping_add(292))), F32(c.m.r32((l_p_7).wrapping_add(296)))) }) });
                                    { let v_ = rt::fmul(rt::fadd(F32(c.m.r32((l_p_7).wrapping_add(48))), rt::fmul(F32(c.m.r32((l_p_7).wrapping_add(64))), l_v_12)), F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a_ = (l_p_7).wrapping_add(80); c.m.w32(a_, (v_).0); };
                                    { let v_ = rt::fmul(rt::fadd(F32(c.m.r32(((l_p_7).wrapping_add(48)).wrapping_add(4))), rt::fmul(F32(c.m.r32(((l_p_7).wrapping_add(64)).wrapping_add(4))), l_v_12)), F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); let a_ = ((l_p_7).wrapping_add(80)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                                    { let v_ = rt::fmul(rt::fadd(F32(c.m.r32(((l_p_7).wrapping_add(48)).wrapping_add(8))), rt::fmul(F32(c.m.r32(((l_p_7).wrapping_add(64)).wrapping_add(8))), l_v_12)), F32(c.m.r32((fp).wrapping_add((((2i32) as u32)).wrapping_mul(4))))); let a_ = ((l_p_7).wrapping_add(80)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                                    if (((((((rt::fle(F32(c.m.r32((p_w_2).wrapping_add(584))), l_t_8) as i32)) != 0) && (((rt::flt(l_t_8, F32(c.m.r32((p_w_2).wrapping_add(592)))) as i32)) != 0)) as i32)) != 0) {
                                        { let r_ = F32(0x3f800000); let a_ = (l_p_7).wrapping_add(292); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                                    }
                                } else {
                                    { let v_ = rt::fmul(F32(c.m.r32((l_p_7).wrapping_add(48))), F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a_ = (l_p_7).wrapping_add(80); c.m.w32(a_, (v_).0); };
                                    { let v_ = rt::fmul(F32(c.m.r32(((l_p_7).wrapping_add(48)).wrapping_add(4))), F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); let a_ = ((l_p_7).wrapping_add(80)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                                    { let v_ = rt::fmul(F32(c.m.r32(((l_p_7).wrapping_add(48)).wrapping_add(8))), F32(c.m.r32((fp).wrapping_add((((2i32) as u32)).wrapping_mul(4))))); let a_ = ((l_p_7).wrapping_add(80)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                                }
                                if (((((((rt::fle(l_t_8, F32(c.m.r32((l_def_3).wrapping_add(488)))) as i32)) != 0) && (((rt::flt(F32(0x00000000), F32(c.m.r32((l_p_7).wrapping_add(236)))) as i32)) != 0)) as i32)) != 0) {
                                    l_a_9 = rt::fdiv(F32(c.m.r32((l_p_7).wrapping_add(316))), F32(c.m.r32((l_p_7).wrapping_add(236))));
                                } else {
                                    l_a_9 = F32(0x3f800000);
                                }
                                if (((((((rt::flt(F32(c.m.r32((l_def_3).wrapping_add(492))), l_t_8) as i32)) != 0) && (((rt::flt(F32(0x00000000), F32(c.m.r32((l_p_7).wrapping_add(240)))) as i32)) != 0)) as i32)) != 0) {
                                    l_a_9 = rt::fsub(F32(0x3f800000), rt::fdiv(rt::fsub(F32(c.m.r32((l_p_7).wrapping_add(316))), rt::fsub(F32(c.m.r32((l_p_7).wrapping_add(312))), F32(c.m.r32((l_p_7).wrapping_add(240))))), F32(c.m.r32((l_p_7).wrapping_add(240)))));
                                }
                                { let v_ = rt::fmul(rt::fmul(F32(c.m.r32(((l_p_7).wrapping_add(48)).wrapping_add(12))), (if (((rt::flt(l_a_9, F32(0x00000000)) as i32)) != 0) { F32(0x00000000) } else { (if (((rt::flt(F32(0x3f800000), l_a_9) as i32)) != 0) { F32(0x3f800000) } else { l_a_9 }) })), p_alpha_1); let a_ = ((l_p_7).wrapping_add(80)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                                F::Vec4_Clamp(c, (l_p_7).wrapping_add(80), (l_p_7).wrapping_add(80), F32(0x00000000), F32(0x437f0000));
                                if (((((c.m.r32((p_w_2).wrapping_add(960)) as i32)) & (256i32))) != 0) {
                                    if (((((c.m.r32((l_p_7).wrapping_add(320)) as i32)) & (512i32))) != 0) {
                                        { let a1_ = { let a_ = (((p_w_2).wrapping_add(668)).wrapping_add((((rt::f2i(F32(c.m.r32((l_p_7).wrapping_add(304))))) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }; let a2_ = { let a_ = (((p_w_2).wrapping_add(668)).wrapping_add((((rt::f2i(F32(c.m.r32((l_p_7).wrapping_add(304))))) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }; let a3_ = { let a_ = (((p_w_2).wrapping_add(668)).wrapping_add((((rt::f2i(F32(c.m.r32((l_p_7).wrapping_add(304))))) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }; let a4_ = { let a_ = (((p_w_2).wrapping_add(668)).wrapping_add((((rt::f2i(F32(c.m.r32((l_p_7).wrapping_add(304))))) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }; F::Vec4_Set(c, (l_p_7).wrapping_add(160), a1_, a2_, a3_, a4_) };
                                        { let a1_ = { let a_ = (((p_w_2).wrapping_add(668)).wrapping_add((((rt::f2i(F32(c.m.r32((l_p_7).wrapping_add(304))))) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }; let a2_ = { let a_ = (((p_w_2).wrapping_add(668)).wrapping_add((((rt::f2i(F32(c.m.r32((l_p_7).wrapping_add(304))))) as u32)).wrapping_mul(16))).wrapping_add((((3i32) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }; let a3_ = { let a_ = (((p_w_2).wrapping_add(668)).wrapping_add((((rt::f2i(F32(c.m.r32((l_p_7).wrapping_add(304))))) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }; let a4_ = { let a_ = (((p_w_2).wrapping_add(668)).wrapping_add((((rt::f2i(F32(c.m.r32((l_p_7).wrapping_add(304))))) as u32)).wrapping_mul(16))).wrapping_add((((3i32) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }; F::Vec4_Set(c, (l_p_7).wrapping_add(176), a1_, a2_, a3_, a4_) };
                                    } else {
                                        { let a1_ = { let a_ = (((p_w_2).wrapping_add(668)).wrapping_add((((rt::f2i(F32(c.m.r32((l_p_7).wrapping_add(304))))) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }; let a2_ = { let a_ = (((p_w_2).wrapping_add(668)).wrapping_add((((rt::f2i(F32(c.m.r32((l_p_7).wrapping_add(304))))) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }; let a3_ = { let a_ = (((p_w_2).wrapping_add(668)).wrapping_add((((rt::f2i(F32(c.m.r32((l_p_7).wrapping_add(304))))) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }; let a4_ = { let a_ = (((p_w_2).wrapping_add(668)).wrapping_add((((rt::f2i(F32(c.m.r32((l_p_7).wrapping_add(304))))) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }; F::Vec4_Set(c, (l_p_7).wrapping_add(160), a1_, a2_, a3_, a4_) };
                                        { let a1_ = { let a_ = (((p_w_2).wrapping_add(668)).wrapping_add((((rt::f2i(F32(c.m.r32((l_p_7).wrapping_add(304))))) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }; let a2_ = { let a_ = (((p_w_2).wrapping_add(668)).wrapping_add((((rt::f2i(F32(c.m.r32((l_p_7).wrapping_add(304))))) as u32)).wrapping_mul(16))).wrapping_add((((3i32) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }; let a3_ = { let a_ = (((p_w_2).wrapping_add(668)).wrapping_add((((rt::f2i(F32(c.m.r32((l_p_7).wrapping_add(304))))) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }; let a4_ = { let a_ = (((p_w_2).wrapping_add(668)).wrapping_add((((rt::f2i(F32(c.m.r32((l_p_7).wrapping_add(304))))) as u32)).wrapping_mul(16))).wrapping_add((((3i32) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }; F::Vec4_Set(c, (l_p_7).wrapping_add(176), a1_, a2_, a3_, a4_) };
                                    }
                                    if (((((rt::rem_i32(rt::f2i(F32(c.m.r32((l_p_7).wrapping_add(316)))), ((c.m.r8((l_def_3).wrapping_add(506))) as i32))) == (0i32)) as i32)) != 0) {
                                        { let r_ = F32(0x3f800000); let a_ = (l_p_7).wrapping_add(304); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                                        if (((rt::fle(F32(c.m.r32((p_w_2).wrapping_add(924))), F32(c.m.r32((l_p_7).wrapping_add(304)))) as i32)) != 0) {
                                            { let v_ = F32(0x00000000); let a_ = (l_p_7).wrapping_add(304); c.m.w32(a_, (v_).0); };
                                        }
                                    }
                                }
                                if (((((((rt::flt(F32(0x00000000), F32(c.m.r32(((l_p_7).wrapping_add(80)).wrapping_add(12)))) as i32)) != 0) && (((rt::flt(F32(0x00000000), F32(c.m.r32((l_p_7).wrapping_add(224)))) as i32)) != 0)) as i32)) != 0) {
                                    if ((((((({ let a_ = (c.m.r32((p_w_2).wrapping_add(48))).wrapping_add(524); c.m.r8(a_) }) as i32)) != (1i32)) as i32)) != 0) {
                                        F::Vu0Cur_Push(c);
                                        F::Vu0Cur_LoadIdentity(c);
                                        F::EftPart10_BuildCorners(c, l_p_7, p_w_2);
                                        F::Vu0Cur_Pop(c);
                                    }
                                    { let r_ = 64i32; let a_ = (l_p_7).wrapping_add(320); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                                } else {
                                    { let r_ = (!(64i32)); let a_ = (l_p_7).wrapping_add(320); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                                }
                                { let r_ = F32(0x3f800000); let a_ = (l_p_7).wrapping_add(316); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                                if (((((((rt::fle(F32(c.m.r32((l_p_7).wrapping_add(312))), F32(c.m.r32((l_p_7).wrapping_add(316)))) as i32)) != 0) || (((((c.m.r32((p_w_2).wrapping_add(960)) as i32)) & (32i32))) != 0)) as i32)) != 0) {
                                    F::EftPart10_UnlinkPtcl(c, (l_grp_6).wrapping_add(4), (l_grp_6).wrapping_add(8), l_p_7);
                                    { let v_ = 0i32; let a_ = (l_p_7).wrapping_add(320); c.m.w32(a_, (v_) as u32); };
                                }
                            }
                            l_p_7 = c.m.r32((l_p_7).wrapping_add(324));
                        }
                    }
                    if (((((c.m.r32((l_grp_6).wrapping_add(4))) == (((0i32) as u32))) as i32)) != 0) {
                        if ((((!(((((c.m.r32(l_grp_6) as i32)) & (2048i32))) != 0)) as i32)) != 0) {
                            F::EftPart10_UnlinkGroup(c, (p_w_2).wrapping_add(964), (p_w_2).wrapping_add(968), l_grp_6);
                            { let v_ = 0i32; let a_ = l_grp_6; c.m.w32(a_, (v_) as u32); };
                        }
                    }
                    { let r_ = (!(2048i32)); let a_ = l_grp_6; let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                }
                l_grp_6 = c.m.r32((l_grp_6).wrapping_add(12));
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftPart10_Step", &PROBE_N, &[], true); }
    r_
}

pub fn EftPart10_NewGroup(c: &mut Ctx, mut p_w_1: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftPart10_NewGroup", &PROBE_N, &[(p_w_1) as u32], false); }
    let mut l_g_2: u32 = 0;
    let mut l_i_3: u8 = 0;
    let r_ = 'f: {
    if ((((({ let a_ = (c.m.r32(sym::gEftPart10Mgr)).wrapping_add(53408); (c.m.r32(a_) as i32) }) >= (150i32)) as i32)) != 0) {
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gEftPart10Mgr)).wrapping_add(53408); c.m.w32(a_, (v_) as u32); };
    }
    l_i_3 = (({ let a_ = (c.m.r32(sym::gEftPart10Mgr)).wrapping_add(53408); (c.m.r32(a_) as i32) }) as u8);
    'l1: loop {
        'c2: {
            l_g_2 = ((c.m.r32(sym::gEftPart10Mgr)).wrapping_add(4)).wrapping_add((((l_i_3) as u32)).wrapping_mul(20));
            { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
            if (((((((l_i_3) as i32)) >= (150i32)) as i32)) != 0) {
                l_i_3 = ((0i32) as u8);
            }
            if ((((((c.m.r32(l_g_2) as i32)) == (0i32)) as i32)) != 0) {
                { let v_ = 1i32; let a_ = l_g_2; c.m.w32(a_, (v_) as u32); };
                { let v_ = ((0i32) as u32); let a_ = (l_g_2).wrapping_add(4); c.m.w32(a_, v_); };
                { let v_ = ((0i32) as u32); let a_ = (l_g_2).wrapping_add(8); c.m.w32(a_, v_); };
                { let v_ = ((0i32) as u32); let a_ = (l_g_2).wrapping_add(12); c.m.w32(a_, v_); };
                { let v_ = ((0i32) as u32); let a_ = (l_g_2).wrapping_add(16); c.m.w32(a_, v_); };
                F::EftPart10_LinkGroup(c, (p_w_1).wrapping_add(964), (p_w_1).wrapping_add(968), l_g_2);
                { let v_ = ((l_i_3) as i32); let a_ = (c.m.r32(sym::gEftPart10Mgr)).wrapping_add(53408); c.m.w32(a_, (v_) as u32); };
                break 'f l_g_2;
            }
        }
        if !(((((((l_i_3) as i32)) != ({ let a_ = (c.m.r32(sym::gEftPart10Mgr)).wrapping_add(53408); (c.m.r32(a_) as i32) })) as i32)) != 0) { break 'l1; }
    }
    break 'f ((0i32) as u32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftPart10_NewGroup", &PROBE_N, &[], true); }
    r_
}
