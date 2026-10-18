#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn eft_d__EftSurf_QueueTri(c: &mut Ctx, mut p_scr0_1: u32, mut p_scr1_2: u32, mut p_scr2_3: u32, mut p_col0_4: u32, mut p_col1_5: u32, mut p_col2_6: u32, mut p_st0_7: u32, mut p_st1_8: u32, mut p_st2_9: u32, mut p_unused_10: i32, mut p_z_11: i32, mut p_tex_12: u32, mut p_fog_13: u32, mut p_count_14: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "eft_d__EftSurf_QueueTri", &PROBE_N, &[(p_scr0_1) as u32, (p_scr1_2) as u32, (p_scr2_3) as u32, (p_col0_4) as u32, (p_col1_5) as u32, (p_col2_6) as u32, (p_st0_7) as u32, (p_st1_8) as u32, (p_st2_9) as u32, (p_unused_10) as u32, (p_z_11) as u32, (p_tex_12) as u32, (p_fog_13) as u32, (p_count_14) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(112) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 112);
    let mut l_pkt_15: u32 = 0;
    let mut l_layer_16: i32 = 0;
    let mut l_l_17: i32 = 0;
    let mut l_i_18: i32 = 0;
    let mut l_entry_19: u32 = 0;
    let mut l_abe_20: u64 = 0;
    let mut l_c_21: u32 = 0;
    let r_ = 'f: {
        l_abe_20 = ((1i32) as u64);
        if (((((((((((rt::fle(F32(c.m.r32((p_col0_4).wrapping_add(12))), F32(0x00000000)) as i32)) != 0) && (((rt::fle(F32(c.m.r32((p_col1_5).wrapping_add(12))), F32(0x00000000)) as i32)) != 0)) as i32)) != 0) && (((rt::fle(F32(c.m.r32((p_col2_6).wrapping_add(12))), F32(0x00000000)) as i32)) != 0)) as i32)) != 0) {
            break 'f;
        }
        if (((((((((((rt::fle(F32(c.m.r32(((p_fog_13).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(12))), F32(0x3c23d70a)) as i32)) != 0) && (((rt::fle(F32(c.m.r32(((p_fog_13).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(12))), F32(0x3c23d70a)) as i32)) != 0)) as i32)) != 0) && (((rt::fle(F32(c.m.r32(((p_fog_13).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(12))), F32(0x3c23d70a)) as i32)) != 0)) as i32)) != 0) {
            p_count_14 = 1i32;
        }
        F::Vec4_Copy(c, (fp).wrapping_add((((0i32) as u32)).wrapping_mul(16)), p_col0_4);
        F::Vec4_Copy(c, (fp).wrapping_add((((1i32) as u32)).wrapping_mul(16)), p_col1_5);
        F::Vec4_Copy(c, (fp).wrapping_add((((2i32) as u32)).wrapping_mul(16)), p_col2_6);
        F::Vec4_Copy(c, (fp).wrapping_add((((3i32) as u32)).wrapping_mul(16)), p_col0_4);
        F::Vec4_Copy(c, (fp).wrapping_add((((4i32) as u32)).wrapping_mul(16)), p_col1_5);
        F::Vec4_Copy(c, (fp).wrapping_add((((5i32) as u32)).wrapping_mul(16)), p_col2_6);
        { let r_ = F32(c.m.r32(((p_fog_13).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(12))); let a_ = ((fp).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(12); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        { let r_ = F32(c.m.r32(((p_fog_13).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(12))); let a_ = ((fp).wrapping_add((((4i32) as u32)).wrapping_mul(16))).wrapping_add(12); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        { let r_ = F32(c.m.r32(((p_fog_13).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(12))); let a_ = ((fp).wrapping_add((((5i32) as u32)).wrapping_mul(16))).wrapping_add(12); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        l_layer_16 = 0i32;
        {
            l_i_18 = 0i32;
            'l1: loop {
                if !(((((l_i_18) < (p_count_14)) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_c_21 = (fp).wrapping_add(((((l_i_18).wrapping_mul(3i32)) as u32)).wrapping_mul(16));
                    if (((((((((p_count_14) == (1i32)) as i32)) != 0) && ((F::StgTint_IsOn(c, ((((0i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) {
                        break 'c2;
                    }
                    l_pkt_15 = c.m.r32(sym::gOtCur);
                    { let v_ = (l_pkt_15).wrapping_add((((1i32) as u32)).wrapping_mul(128)); let a_ = sym::gOtCur; c.m.w32(a_, v_); };
                    { let v_ = ((27i32) as u64); let a_ = fp.wrapping_add(96); c.m.w64(a_, v_); };
                    { let v_ = l_abe_20; let a_ = fp.wrapping_add(96); rt::bf_set(c, a_, 6, 1, (v_) as u64);  };
                    { let v_ = c.m.r64(fp.wrapping_add(96)); let a_ = (l_pkt_15).wrapping_add(32); c.m.w64(a_, v_); };
                    { let v_ = ((536870919i32) as u32); let a_ = l_pkt_15; c.m.w32(a_, v_); };
                    { let v_ = ((268435456i32) as u32); let a_ = (l_pkt_15).wrapping_add(8); c.m.w32(a_, v_); };
                    { let v_ = ((1342177287i32) as u32); let a_ = (l_pkt_15).wrapping_add(12); c.m.w32(a_, v_); };
                    { let v_ = 14123288431433908225u64; let a_ = ((l_pkt_15).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                    { let v_ = ((268423679713632i64) as u64); let a_ = ((l_pkt_15).wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                    { let v_ = ((0i32) as u32); let a_ = (l_pkt_15).wrapping_add(4); c.m.w32(a_, v_); };
                    { let v_ = ((0i32) as u64); let a_ = (l_pkt_15).wrapping_add(120); c.m.w64(a_, v_); };
                    { let v_ = ((rt::f2i(F32(c.m.r32((l_c_21).wrapping_add((((0i32) as u32)).wrapping_mul(16)))))) as u8); let a_ = ((l_pkt_15).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24)); c.m.w8(a_, v_); };
                    { let v_ = ((rt::f2i(F32(c.m.r32(((l_c_21).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4))))) as u8); let a_ = (((l_pkt_15).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(1); c.m.w8(a_, v_); };
                    { let v_ = ((rt::f2i(F32(c.m.r32(((l_c_21).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8))))) as u8); let a_ = (((l_pkt_15).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(2); c.m.w8(a_, v_); };
                    { let v_ = ((rt::f2i(F32(c.m.r32(((l_c_21).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(12))))) as u8); let a_ = (((l_pkt_15).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(3); c.m.w8(a_, v_); };
                    { let v_ = F32(c.m.r32((p_st0_7).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); let a_ = (((l_pkt_15).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = ((rt::f2i(F32(c.m.r32((l_c_21).wrapping_add((((1i32) as u32)).wrapping_mul(16)))))) as u8); let a_ = ((l_pkt_15).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24)); c.m.w8(a_, v_); };
                    { let v_ = ((rt::f2i(F32(c.m.r32(((l_c_21).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(4))))) as u8); let a_ = (((l_pkt_15).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(1); c.m.w8(a_, v_); };
                    { let v_ = ((rt::f2i(F32(c.m.r32(((l_c_21).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8))))) as u8); let a_ = (((l_pkt_15).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(2); c.m.w8(a_, v_); };
                    { let v_ = ((rt::f2i(F32(c.m.r32(((l_c_21).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(12))))) as u8); let a_ = (((l_pkt_15).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(3); c.m.w8(a_, v_); };
                    { let v_ = F32(c.m.r32((p_st1_8).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); let a_ = (((l_pkt_15).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = ((rt::f2i(F32(c.m.r32((l_c_21).wrapping_add((((2i32) as u32)).wrapping_mul(16)))))) as u8); let a_ = ((l_pkt_15).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24)); c.m.w8(a_, v_); };
                    { let v_ = ((rt::f2i(F32(c.m.r32(((l_c_21).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(4))))) as u8); let a_ = (((l_pkt_15).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(1); c.m.w8(a_, v_); };
                    { let v_ = ((rt::f2i(F32(c.m.r32(((l_c_21).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8))))) as u8); let a_ = (((l_pkt_15).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(2); c.m.w8(a_, v_); };
                    { let v_ = ((rt::f2i(F32(c.m.r32(((l_c_21).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(12))))) as u8); let a_ = (((l_pkt_15).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(3); c.m.w8(a_, v_); };
                    l_l_17 = l_layer_16;
                    { let v_ = F32(c.m.r32((p_st2_9).wrapping_add(8))); let a_ = (((l_pkt_15).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    if ((((!(((((l_layer_16) < (2i32)) as i32)) != 0)) as i32)) != 0) {
                        l_l_17 = (l_layer_16).wrapping_sub(2i32);
                    }
                    { let v_ = c.m.r64((p_tex_12).wrapping_add((((l_i_18) as u32)).wrapping_mul(8))); let a_ = (l_pkt_15).wrapping_add(40); c.m.w64(a_, v_); };
                    { let v_ = F32(c.m.r32((p_st0_7).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = (((l_pkt_15).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(8); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(c.m.r32((p_st0_7).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a_ = (((l_pkt_15).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(12); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(c.m.r32((p_st1_8).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = (((l_pkt_15).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(8); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(c.m.r32((p_st1_8).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a_ = (((l_pkt_15).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(12); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(c.m.r32((p_st2_9).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = (((l_pkt_15).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(8); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(c.m.r32((p_st2_9).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a_ = (((l_pkt_15).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(12); c.m.w32(a_, (v_).0); };
                    { let v_ = (((c.m.r32((p_scr0_1).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) as u64); let a_ = (((l_pkt_15).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(16); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                    { let v_ = (((c.m.r32((p_scr0_1).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) as u64); let a_ = ((((l_pkt_15).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                    { let v_ = (((c.m.r32((p_scr0_1).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32)) as u64); let a_ = ((((l_pkt_15).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
                    { let v_ = ((255i32) as u64); let a_ = ((((l_pkt_15).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
                    { let v_ = (((c.m.r32((p_scr1_2).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) as u64); let a_ = (((l_pkt_15).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(16); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                    { let v_ = (((c.m.r32((p_scr1_2).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) as u64); let a_ = ((((l_pkt_15).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                    { let v_ = (((c.m.r32((p_scr1_2).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32)) as u64); let a_ = ((((l_pkt_15).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
                    { let v_ = ((255i32) as u64); let a_ = ((((l_pkt_15).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
                    { let v_ = (((c.m.r32((p_scr2_3).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) as u64); let a_ = (((l_pkt_15).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(16); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                    { let v_ = (((c.m.r32((p_scr2_3).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) as u64); let a_ = ((((l_pkt_15).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                    { let v_ = (((c.m.r32((p_scr2_3).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32)) as u64); let a_ = ((((l_pkt_15).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
                    { let v_ = ((255i32) as u64); let a_ = ((((l_pkt_15).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
                    if (((((p_z_11) < (0i32)) as i32)) != 0) {
                        l_entry_19 = ((c.m.r32(sym::gOtZ)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((l_l_17) as u32)).wrapping_mul(8));
                    } else {
                        if (((((p_z_11) >= (4096i32)) as i32)) != 0) {
                            l_entry_19 = ((c.m.r32(sym::gOtZ)).wrapping_add((((4095i32) as u32)).wrapping_mul(16))).wrapping_add((((l_l_17) as u32)).wrapping_mul(8));
                        } else {
                            l_entry_19 = ((c.m.r32(sym::gOtZ)).wrapping_add((((p_z_11) as u32)).wrapping_mul(16))).wrapping_add((((l_l_17) as u32)).wrapping_mul(8));
                        }
                    }
                    { let v_ = l_pkt_15; let a_ = (c.m.r32((l_entry_19).wrapping_add(4))).wrapping_add(4); c.m.w32(a_, v_); };
                    { let v_ = l_pkt_15; let a_ = (l_entry_19).wrapping_add(4); c.m.w32(a_, v_); };
                    { let o_ = l_layer_16; let n_ = (o_).wrapping_add(1); l_layer_16 = n_; };
                }
                { let o_ = l_i_18; let n_ = (o_).wrapping_add(1); l_i_18 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "eft_d__EftSurf_QueueTri", &PROBE_N, &[], true); }
    r_
}

pub fn EftSurf_DrawPolyOtClipped(c: &mut Ctx, mut p_poly_1: u32, mut p_unused_2: i32, mut p_tex_3: u32, mut p_fog_4: u32, mut p_zBias_5: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSurf_DrawPolyOtClipped", &PROBE_N, &[(p_poly_1) as u32, (p_unused_2) as u32, (p_tex_3) as u32, (p_fog_4) as u32, (p_zBias_5) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(1056) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 1056);
    let mut l_plane_6: u32 = 0;
    let mut l_count_7: i32 = 0;
    let mut l_k_8: i32 = 0;
    let r_ = 'f: {
        { let _ = { rt::memcpy(c, fp, p_poly_1, ((((3i32) as u32)).wrapping_mul(48u32)) as u32); fp }; };
        p_poly_1 = fp;
        l_plane_6 = { let a_ = (c.m.r32(sym::gEftSurf)).wrapping_add(16); c.m.r32(a_) };
        l_count_7 = 3i32;
        {
            l_k_8 = 4i32;
            'l1: loop {
                if !(((((l_k_8) >= (0i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_count_7 = F::ClipPoly_ClipPlane(c, p_poly_1, l_plane_6, l_count_7);
                    { let o_ = l_plane_6; let n_ = (o_).wrapping_add(16); l_plane_6 = n_; };
                }
                { let o_ = l_k_8; let n_ = (o_).wrapping_sub(1); l_k_8 = n_; };
            }
        }
        if (((((l_count_7) != (0i32)) as i32)) != 0) {
            F::ClipPoly_ProjectCur(c, fp.wrapping_add(768), fp.wrapping_add(912), p_poly_1, l_count_7);
            {
                l_k_8 = 2i32;
                'l3: loop {
                    if !(((((l_k_8) < (l_count_7)) as i32)) != 0) { break 'l3; }
                    'c4: {
                        { let a10_ = (((c.m.r32(((fp.wrapping_add(768)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32)).wrapping_shr((8i32) as u32)).wrapping_sub(p_zBias_5); F::eft_d__EftSurf_QueueTri(c, (fp.wrapping_add(768)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(768)).wrapping_add(((((l_k_8).wrapping_sub(1i32)) as u32)).wrapping_mul(16)), (fp.wrapping_add(768)).wrapping_add((((l_k_8) as u32)).wrapping_mul(16)), ((p_poly_1).wrapping_add((((0i32) as u32)).wrapping_mul(48))).wrapping_add(32), ((p_poly_1).wrapping_add(((((l_k_8).wrapping_sub(1i32)) as u32)).wrapping_mul(48))).wrapping_add(32), ((p_poly_1).wrapping_add((((l_k_8) as u32)).wrapping_mul(48))).wrapping_add(32), (fp.wrapping_add(912)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(912)).wrapping_add(((((l_k_8).wrapping_sub(1i32)) as u32)).wrapping_mul(16)), (fp.wrapping_add(912)).wrapping_add((((l_k_8) as u32)).wrapping_mul(16)), p_unused_2, a10_, p_tex_3, p_fog_4, 2i32) };
                    }
                    { let o_ = l_k_8; let n_ = (o_).wrapping_add(1); l_k_8 = n_; };
                }
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftSurf_DrawPolyOtClipped", &PROBE_N, &[], true); }
    r_
}

pub fn EftSurf_SetColorA(c: &mut Ctx, p_v_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSurf_SetColorA", &PROBE_N, &[(p_v_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    rt::memcpy(c, fp, p_v_1, 16);
    let r_ = 'f: {
        { let a0_ = (c.m.r32(sym::gEftSurf)).wrapping_add(4800); F::Vec4_Copy(c, a0_, fp) };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftSurf_SetColorA", &PROBE_N, &[], true); }
    r_
}

pub fn EftSurf_SetColorB(c: &mut Ctx, p_v_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSurf_SetColorB", &PROBE_N, &[(p_v_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    rt::memcpy(c, fp, p_v_1, 16);
    let r_ = 'f: {
        { let a0_ = (c.m.r32(sym::gEftSurf)).wrapping_add(4816); F::Vec4_Copy(c, a0_, fp) };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftSurf_SetColorB", &PROBE_N, &[], true); }
    r_
}

pub fn EftSurf_SetLightDir(c: &mut Ctx, p_v_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSurf_SetLightDir", &PROBE_N, &[(p_v_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    rt::memcpy(c, fp, p_v_1, 16);
    let r_ = 'f: {
        { let a0_ = (c.m.r32(sym::gEftSurf)).wrapping_add(4832); F::Vec4_Copy(c, a0_, fp) };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftSurf_SetLightDir", &PROBE_N, &[], true); }
    r_
}

pub fn EftSurf_SetAnimFrames(c: &mut Ctx, mut p_v_1: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSurf_SetAnimFrames", &PROBE_N, &[(p_v_1).0], false); }
    let r_ = 'f: {
    { let v_ = rt::f2i(p_v_1); let a_ = (c.m.r32(sym::gEftSurf)).wrapping_add(4852); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftSurf_SetAnimFrames", &PROBE_N, &[], true); }
    r_
}

pub fn EftSurf_SetParam30(c: &mut Ctx, mut p_v_1: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSurf_SetParam30", &PROBE_N, &[(p_v_1).0], false); }
    let r_ = 'f: {
    { let v_ = p_v_1; let a_ = (c.m.r32(sym::gEftSurf)).wrapping_add(4848); c.m.w32(a_, (v_).0); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftSurf_SetParam30", &PROBE_N, &[], true); }
    r_
}

pub fn EftSurf_GetParam(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSurf_GetParam", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if (((((c.m.r32(sym::gEftSurf)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f ((0i32) as u32);
    }
    break 'f (c.m.r32(sym::gEftSurf)).wrapping_add(4800);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftSurf_GetParam", &PROBE_N, &[], true); }
    r_
}

pub fn eft_d__EftSurf_IsOffScreen(c: &mut Ctx, mut p_xyz_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "eft_d__EftSurf_IsOffScreen", &PROBE_N, &[(p_xyz_1) as u32], false); }
    let mut l_xMin_2: i32 = 0;
    let mut l_xMax_3: i32 = 0;
    let mut l_yMax_4: i32 = 0;
    let mut l_yMin_5: i32 = 0;
    let mut l_ret_6: i32 = 0;
    let r_ = 'f: {
    l_xMin_2 = 1776i32;
    l_xMax_3 = 2320i32;
    l_yMax_4 = 2288i32;
    l_yMin_5 = 1808i32;
    if ((((!((F::BtlScene_IsSingleView(c)) != 0)) as i32)) != 0) {
        if ((((!((F::BtlScene_IsSecondView(c)) != 0)) as i32)) != 0) {
            l_xMax_3 = 2064i32;
        } else {
            l_xMin_2 = 2032i32;
        }
    }
    l_ret_6 = 1i32;
    if ((((((((((((((((((((((c.m.r32((p_xyz_1).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32)) > (0i32)) as i32)) != 0) && ((((((c.m.r32((p_xyz_1).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) < ((l_xMax_3).wrapping_shl((4i32) as u32))) as i32)) != 0)) as i32)) != 0) && ((((((c.m.r32((p_xyz_1).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) > ((l_xMin_2).wrapping_shl((4i32) as u32))) as i32)) != 0)) as i32)) != 0) && ((((((c.m.r32((p_xyz_1).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) < ((l_yMax_4).wrapping_shl((4i32) as u32))) as i32)) != 0)) as i32)) != 0) && ((((((c.m.r32((p_xyz_1).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) > ((l_yMin_5).wrapping_shl((4i32) as u32))) as i32)) != 0)) as i32)) != 0) {
        l_ret_6 = 0i32;
    }
    break 'f l_ret_6;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "eft_d__EftSurf_IsOffScreen", &PROBE_N, &[], true); }
    r_
}

pub fn EftSurf_DrawTriOt(c: &mut Ctx, mut p_tri_1: u32, mut p_unused_2: i32, mut p_tex_3: u32, mut p_fog_4: u32, mut p_zBias_5: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSurf_DrawTriOt", &PROBE_N, &[(p_tri_1) as u32, (p_unused_2) as u32, (p_tex_3) as u32, (p_fog_4) as u32, (p_zBias_5) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(96) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 96);
    let r_ = 'f: {
        F::ClipPoly_ProjectCur(c, fp, fp.wrapping_add(48), p_tri_1, 3i32);
        if ((((((((((F::eft_d__EftSurf_IsOffScreen(c, (fp).wrapping_add((((0i32) as u32)).wrapping_mul(16)))) != 0) && ((F::eft_d__EftSurf_IsOffScreen(c, (fp).wrapping_add((((1i32) as u32)).wrapping_mul(16)))) != 0)) as i32)) != 0) && ((F::eft_d__EftSurf_IsOffScreen(c, (fp).wrapping_add((((2i32) as u32)).wrapping_mul(16)))) != 0)) as i32)) != 0) {
            break 'f;
        }
        { let a10_ = (((c.m.r32(((fp).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32)).wrapping_shr((8i32) as u32)).wrapping_sub(p_zBias_5); F::eft_d__EftSurf_QueueTri(c, (fp).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (fp).wrapping_add((((1i32) as u32)).wrapping_mul(16)), (fp).wrapping_add((((2i32) as u32)).wrapping_mul(16)), ((p_tri_1).wrapping_add((((0i32) as u32)).wrapping_mul(48))).wrapping_add(32), ((p_tri_1).wrapping_add((((1i32) as u32)).wrapping_mul(48))).wrapping_add(32), ((p_tri_1).wrapping_add((((2i32) as u32)).wrapping_mul(48))).wrapping_add(32), (fp.wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(16)), p_unused_2, a10_, p_tex_3, p_fog_4, 2i32) };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftSurf_DrawTriOt", &PROBE_N, &[], true); }
    r_
}

pub fn EftSurf_DrawTriOtEx(c: &mut Ctx, mut p_scr0_1: u32, mut p_scr1_2: u32, mut p_scr2_3: u32, mut p_col0_4: u32, mut p_col1_5: u32, mut p_col2_6: u32, mut p_st0_7: u32, mut p_st1_8: u32, mut p_st2_9: u32, mut p_unused_10: i32, mut p_z_11: i32, mut p_tex_12: u32, mut p_fog_13: u32, mut p_count_14: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSurf_DrawTriOtEx", &PROBE_N, &[(p_scr0_1) as u32, (p_scr1_2) as u32, (p_scr2_3) as u32, (p_col0_4) as u32, (p_col1_5) as u32, (p_col2_6) as u32, (p_st0_7) as u32, (p_st1_8) as u32, (p_st2_9) as u32, (p_unused_10) as u32, (p_z_11) as u32, (p_tex_12) as u32, (p_fog_13) as u32, (p_count_14) as u32], false); }
    let r_ = 'f: {
    F::eft_d__EftSurf_QueueTri(c, p_scr0_1, p_scr1_2, p_scr2_3, p_col0_4, p_col1_5, p_col2_6, p_st0_7, p_st1_8, p_st2_9, p_unused_10, p_z_11, p_tex_12, p_fog_13, p_count_14);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftSurf_DrawTriOtEx", &PROBE_N, &[], true); }
    r_
}

pub fn EftSurf_DrawTriDirect(c: &mut Ctx, mut p_scr0_1: u32, mut p_scr1_2: u32, mut p_scr2_3: u32, mut p_col0_4: u32, mut p_col1_5: u32, mut p_col2_6: u32, mut p_st0_7: u32, mut p_st1_8: u32, mut p_st2_9: u32, mut p_blend_10: i32, mut p_tex_11: u32, mut p_fog_12: u32, mut p_count_13: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftSurf_DrawTriDirect", &PROBE_N, &[(p_scr0_1) as u32, (p_scr1_2) as u32, (p_scr2_3) as u32, (p_col0_4) as u32, (p_col1_5) as u32, (p_col2_6) as u32, (p_st0_7) as u32, (p_st1_8) as u32, (p_st2_9) as u32, (p_blend_10) as u32, (p_tex_11) as u32, (p_fog_12) as u32, (p_count_13) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(112) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 112);
    let mut l_p_14: u32 = 0;
    let mut l_i_15: i32 = 0;
    let mut l_abe_16: i32 = 0;
    let mut l_c_17: u32 = 0;
    let mut l_ctxt_18: i8 = 0;
    let r_ = 'f: {
        l_abe_16 = 1i32;
        if (((((((((((rt::fle(F32(c.m.r32((p_col0_4).wrapping_add(12))), F32(0x00000000)) as i32)) != 0) && (((rt::fle(F32(c.m.r32((p_col1_5).wrapping_add(12))), F32(0x00000000)) as i32)) != 0)) as i32)) != 0) && (((rt::fle(F32(c.m.r32((p_col2_6).wrapping_add(12))), F32(0x00000000)) as i32)) != 0)) as i32)) != 0) {
            break 'f;
        }
        if (((((((((((rt::fle(F32(c.m.r32(((p_fog_12).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(12))), F32(0x3c23d70a)) as i32)) != 0) && (((rt::fle(F32(c.m.r32(((p_fog_12).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(12))), F32(0x3c23d70a)) as i32)) != 0)) as i32)) != 0) && (((rt::fle(F32(c.m.r32(((p_fog_12).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(12))), F32(0x3c23d70a)) as i32)) != 0)) as i32)) != 0) {
            p_count_13 = 1i32;
        }
        { let v_ = c.m.r64((p_tex_11).wrapping_add((((0i32) as u32)).wrapping_mul(8))); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = c.m.r64((p_tex_11).wrapping_add((((1i32) as u32)).wrapping_mul(8))); let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        if (((((p_blend_10) < (0i32)) as i32)) != 0) {
            l_abe_16 = 0i32;
            p_blend_10 = 0i32;
        }
        F::Vec4_ToInt(c, (fp.wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), p_col0_4);
        F::Vec4_ToInt(c, (fp.wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), p_col1_5);
        F::Vec4_ToInt(c, (fp.wrapping_add(16)).wrapping_add((((2i32) as u32)).wrapping_mul(16)), p_col2_6);
        F::Vec4_ToInt(c, (fp.wrapping_add(16)).wrapping_add((((3i32) as u32)).wrapping_mul(16)), p_col0_4);
        F::Vec4_ToInt(c, (fp.wrapping_add(16)).wrapping_add((((4i32) as u32)).wrapping_mul(16)), p_col1_5);
        F::Vec4_ToInt(c, (fp.wrapping_add(16)).wrapping_add((((5i32) as u32)).wrapping_mul(16)), p_col2_6);
        { let v_ = rt::f2i(rt::fmul(rt::i2f((c.m.r32(((fp.wrapping_add(16)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((3i32) as u32)).wrapping_mul(4))) as i32)), F32(c.m.r32(((p_fog_12).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(12))))); let a_ = ((fp.wrapping_add(16)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = rt::f2i(rt::fmul(rt::i2f((c.m.r32(((fp.wrapping_add(16)).wrapping_add((((4i32) as u32)).wrapping_mul(16))).wrapping_add((((3i32) as u32)).wrapping_mul(4))) as i32)), F32(c.m.r32(((p_fog_12).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(12))))); let a_ = ((fp.wrapping_add(16)).wrapping_add((((4i32) as u32)).wrapping_mul(16))).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = rt::f2i(rt::fmul(rt::i2f((c.m.r32(((fp.wrapping_add(16)).wrapping_add((((5i32) as u32)).wrapping_mul(16))).wrapping_add((((3i32) as u32)).wrapping_mul(4))) as i32)), F32(c.m.r32(((p_fog_12).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(12))))); let a_ = ((fp.wrapping_add(16)).wrapping_add((((5i32) as u32)).wrapping_mul(16))).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        {
            l_i_15 = 0i32;
            'l1: loop {
                if !(((((l_i_15) < (p_count_13)) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_c_17 = (fp.wrapping_add(16)).wrapping_add(((((l_i_15).wrapping_mul(3i32)) as u32)).wrapping_mul(16));
                    l_p_14 = F::Dma_BeginDirect(c);
                    l_ctxt_18 = ((l_i_15) as i8);
                    { let v_ = ((((((27i32) as u64)) | ((((l_abe_16) as u64)).wrapping_shl((6i32) as u32)))) | ((((l_ctxt_18) as u64)).wrapping_shl((9i32) as u32))); let a_ = (l_p_14).wrapping_add(16); c.m.w64(a_, v_); };
                    { let v_ = 14123288431433908225u64; let a_ = (l_p_14).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                    { let v_ = (((268423679713632i64).wrapping_add((((((l_ctxt_18) as i32)).wrapping_shl((4i32) as u32)) as i64))) as u64); let a_ = (l_p_14).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                    { let v_ = ((0i32) as u64); let a_ = (l_p_14).wrapping_add(104); c.m.w64(a_, v_); };
                    { let v_ = (((c.m.r32((l_c_17).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) as u8); let a_ = ((l_p_14).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(24)); c.m.w8(a_, v_); };
                    { let v_ = (((c.m.r32((l_c_17).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) as u8); let a_ = (((l_p_14).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(1); c.m.w8(a_, v_); };
                    { let v_ = (((c.m.r32((l_c_17).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32)) as u8); let a_ = (((l_p_14).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(2); c.m.w8(a_, v_); };
                    { let v_ = (((c.m.r32((l_c_17).wrapping_add((((3i32) as u32)).wrapping_mul(4))) as i32)) as u8); let a_ = (((l_p_14).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(3); c.m.w8(a_, v_); };
                    { let v_ = F32(c.m.r32((p_st0_7).wrapping_add(8))); let a_ = (((l_p_14).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = (((c.m.r32((l_c_17).wrapping_add((((4i32) as u32)).wrapping_mul(4))) as i32)) as u8); let a_ = ((l_p_14).wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(24)); c.m.w8(a_, v_); };
                    { let v_ = (((c.m.r32((l_c_17).wrapping_add((((5i32) as u32)).wrapping_mul(4))) as i32)) as u8); let a_ = (((l_p_14).wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(1); c.m.w8(a_, v_); };
                    { let v_ = (((c.m.r32((l_c_17).wrapping_add((((6i32) as u32)).wrapping_mul(4))) as i32)) as u8); let a_ = (((l_p_14).wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(2); c.m.w8(a_, v_); };
                    { let v_ = (((c.m.r32((l_c_17).wrapping_add((((7i32) as u32)).wrapping_mul(4))) as i32)) as u8); let a_ = (((l_p_14).wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(3); c.m.w8(a_, v_); };
                    { let v_ = F32(c.m.r32((p_st1_8).wrapping_add(8))); let a_ = (((l_p_14).wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = (((c.m.r32((l_c_17).wrapping_add((((8i32) as u32)).wrapping_mul(4))) as i32)) as u8); let a_ = ((l_p_14).wrapping_add(32)).wrapping_add((((2i32) as u32)).wrapping_mul(24)); c.m.w8(a_, v_); };
                    { let v_ = (((c.m.r32((l_c_17).wrapping_add((((9i32) as u32)).wrapping_mul(4))) as i32)) as u8); let a_ = (((l_p_14).wrapping_add(32)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(1); c.m.w8(a_, v_); };
                    { let v_ = (((c.m.r32((l_c_17).wrapping_add((((10i32) as u32)).wrapping_mul(4))) as i32)) as u8); let a_ = (((l_p_14).wrapping_add(32)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(2); c.m.w8(a_, v_); };
                    { let v_ = (((c.m.r32((l_c_17).wrapping_add((((11i32) as u32)).wrapping_mul(4))) as i32)) as u8); let a_ = (((l_p_14).wrapping_add(32)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(3); c.m.w8(a_, v_); };
                    { let v_ = F32(c.m.r32((p_st2_9).wrapping_add(8))); let a_ = (((l_p_14).wrapping_add(32)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = c.m.r64((fp).wrapping_add((((l_i_15) as u32)).wrapping_mul(8))); let a_ = (l_p_14).wrapping_add(24); c.m.w64(a_, v_); };
                    { let v_ = F32(c.m.r32(p_st0_7)); let a_ = (((l_p_14).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(8); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(c.m.r32((p_st0_7).wrapping_add(4))); let a_ = (((l_p_14).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(12); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(c.m.r32(p_st1_8)); let a_ = (((l_p_14).wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(8); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(c.m.r32((p_st1_8).wrapping_add(4))); let a_ = (((l_p_14).wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(12); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(c.m.r32(p_st2_9)); let a_ = (((l_p_14).wrapping_add(32)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(8); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(c.m.r32((p_st2_9).wrapping_add(4))); let a_ = (((l_p_14).wrapping_add(32)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(12); c.m.w32(a_, (v_).0); };
                    { let v_ = (((c.m.r32(p_scr0_1) as i32)) as u64); let a_ = (((l_p_14).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(16); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                    { let v_ = (((c.m.r32((p_scr0_1).wrapping_add(4)) as i32)) as u64); let a_ = ((((l_p_14).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                    { let v_ = (((c.m.r32((p_scr0_1).wrapping_add(8)) as i32)) as u64); let a_ = ((((l_p_14).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
                    { let v_ = ((255i32) as u64); let a_ = ((((l_p_14).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
                    { let v_ = (((c.m.r32(p_scr1_2) as i32)) as u64); let a_ = (((l_p_14).wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(16); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                    { let v_ = (((c.m.r32((p_scr1_2).wrapping_add(4)) as i32)) as u64); let a_ = ((((l_p_14).wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                    { let v_ = (((c.m.r32((p_scr1_2).wrapping_add(8)) as i32)) as u64); let a_ = ((((l_p_14).wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
                    { let v_ = ((255i32) as u64); let a_ = ((((l_p_14).wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
                    { let v_ = (((c.m.r32(p_scr2_3) as i32)) as u64); let a_ = (((l_p_14).wrapping_add(32)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(16); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                    { let v_ = (((c.m.r32((p_scr2_3).wrapping_add(4)) as i32)) as u64); let a_ = ((((l_p_14).wrapping_add(32)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(2); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                    { let v_ = (((c.m.r32((p_scr2_3).wrapping_add(8)) as i32)) as u64); let a_ = ((((l_p_14).wrapping_add(32)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(4); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
                    { let v_ = ((255i32) as u64); let a_ = ((((l_p_14).wrapping_add(32)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add(7); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
                    F::Dma_EndDirect(c, (l_p_14).wrapping_add((((1i32) as u32)).wrapping_mul(112)));
                }
                { let o_ = l_i_15; let n_ = (o_).wrapping_add(1); l_i_15 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftSurf_DrawTriDirect", &PROBE_N, &[], true); }
    r_
}
