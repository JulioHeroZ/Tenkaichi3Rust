#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn ColLine2_Intersect(c: &mut Ctx, mut p_p_1: u32, mut p_q_2: u32, mut p_r_3: u32, mut p_s_4: u32, mut p_t_5: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ColLine2_Intersect", &PROBE_N, &[(p_p_1) as u32, (p_q_2) as u32, (p_r_3) as u32, (p_s_4) as u32, (p_t_5) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_det_6: F32 = F32(0);
    let r_ = 'f: {
        { let v_ = rt::fsub(F32(c.m.r32((p_q_2).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((p_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a_ = fp; c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32((p_q_2).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((p_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32((p_r_3).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((p_s_4).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a_ = fp.wrapping_add(16); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32((p_r_3).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((p_s_4).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); let a_ = (fp.wrapping_add(16)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        l_det_6 = rt::fsub(rt::fmul(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))), F32(c.m.r32(fp))), rt::fmul(F32(c.m.r32(fp.wrapping_add(16))), F32(c.m.r32((fp).wrapping_add(4)))));
        if (((rt::feq(l_det_6, F32(0x00000000)) as i32)) != 0) {
            break 'f 0i32;
        }
        if (((((p_t_5) != (((0i32) as u32))) as i32)) != 0) {
            { let v_ = rt::fdiv(rt::fsub(rt::fmul(F32(c.m.r32(fp.wrapping_add(16))), rt::fsub(F32(c.m.r32((p_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((p_r_3).wrapping_add((((1i32) as u32)).wrapping_mul(4)))))), rt::fmul(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))), rt::fsub(F32(c.m.r32((p_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((p_r_3).wrapping_add((((0i32) as u32)).wrapping_mul(4))))))), l_det_6); let a_ = p_t_5; c.m.w32(a_, (v_).0); };
        }
        break 'f 1i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "ColLine2_Intersect", &PROBE_N, &[], true); }
    r_
}

pub fn ColSweep_TestTri(c: &mut Ctx, mut p_sw_1: u32, mut p_tri_2: u32, mut p_pos_3: u32, mut p_dist_4: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ColSweep_TestTri", &PROBE_N, &[(p_sw_1) as u32, (p_tri_2) as u32, (p_pos_3) as u32, (p_dist_4) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(368) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 368);
    let mut l_kind_5: i32 = 0;
    let mut l_best_6: F32 = F32(0);
    let mut l_h_7: F32 = F32(0);
    let mut l_i_8: i32 = 0;
    let mut l_dn_9: F32 = F32(0);
    let mut l_m_10: F32 = F32(0);
    let mut l_rad_11: F32 = F32(0);
    let mut l_d_12: F32 = F32(0);
    let mut l_ax_13: F32 = F32(0);
    let mut l_ay_14: F32 = F32(0);
    let mut l_az_15: F32 = F32(0);
    let mut l_i0_16: i32 = 0;
    let mut l_i1_17: i32 = 0;
    let mut l_j_18: i32 = 0;
    let r_ = 'f: {
        F::Vec4_Copy(c, fp.wrapping_add(16), (p_sw_1).wrapping_add(96));
        if (((rt::fgt(F::Vec3_Dot(c, fp.wrapping_add(16), (p_tri_2).wrapping_add(48)), F32(0xba83126e)) as i32)) != 0) {
            break 'f 0i32;
        }
        l_best_6 = F32(0x7f7ffffe);
        F::Vec4_Copy(c, fp.wrapping_add(32), (p_tri_2).wrapping_add(48));
        { let v_ = rt::fneg(F32(c.m.r32((fp.wrapping_add(32)).wrapping_add(12)))); let a_ = (fp.wrapping_add(32)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        l_kind_5 = (1i32).wrapping_neg();
        l_h_7 = rt::fadd(F::Vec3_Dot(c, fp.wrapping_add(32), (p_sw_1).wrapping_add(48)), F32(c.m.r32((fp.wrapping_add(32)).wrapping_add(12))));
        if (((rt::flt(l_h_7, rt::fneg(F32(c.m.r32(((p_sw_1).wrapping_add(48)).wrapping_add(16))))) as i32)) != 0) {
            break 'f 0i32;
        }
        if (((rt::fgt(l_h_7, F32(c.m.r32(((p_sw_1).wrapping_add(48)).wrapping_add(16)))) as i32)) != 0) {
            { let r_ = F32(c.m.r32(((p_sw_1).wrapping_add(48)).wrapping_add(16))); let n_ = rt::fsub(l_h_7, r_); l_h_7 = n_; };
            l_dn_9 = F::Vec3_Dot(c, (p_tri_2).wrapping_add(48), fp.wrapping_add(16));
            if (((rt::fne(l_dn_9, F32(0x00000000)) as i32)) != 0) {
                l_h_7 = rt::fdiv(rt::fneg(l_h_7), l_dn_9);
                { let v_ = rt::fadd(F32(c.m.r32((p_sw_1).wrapping_add(48))), rt::fmul(F32(c.m.r32(fp.wrapping_add(16))), l_h_7)); let a_ = fp.wrapping_add(48); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fadd(F32(c.m.r32(((p_sw_1).wrapping_add(48)).wrapping_add(4))), rt::fmul(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))), l_h_7)); let a_ = (fp.wrapping_add(48)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fadd(F32(c.m.r32(((p_sw_1).wrapping_add(48)).wrapping_add(8))), rt::fmul(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8))), l_h_7)); let a_ = (fp.wrapping_add(48)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                { let v_ = F32(0x3f800000); let a_ = (fp.wrapping_add(48)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                if ((((((F::ColTri_ContainsPoint(c, p_tri_2, fp.wrapping_add(48))) != 0) && (((rt::flt(l_h_7, l_best_6) as i32)) != 0)) as i32)) != 0) {
                    F::Vec4_Copy(c, fp, (p_tri_2).wrapping_add(48));
                    l_best_6 = l_h_7;
                    l_kind_5 = 0i32;
                }
            }
        }
        {
            l_i_8 = 0i32;
            'l1: loop {
                if !(((((l_i_8) < (3i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let v_ = F32(0x7f7ffffe); let a_ = fp.wrapping_add(356); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(0x7f7ffffe); let a_ = fp.wrapping_add(360); c.m.w32(a_, (v_).0); };
                    F::Vec4_Copy(c, fp.wrapping_add(48), (p_tri_2).wrapping_add((((l_i_8) as u32)).wrapping_mul(16)));
                    F::Vec4_Sub(c, fp.wrapping_add(64), fp.wrapping_add(48), fp.wrapping_add(16));
                    F::Vec4_Sub(c, fp.wrapping_add(80), fp.wrapping_add(64), fp.wrapping_add(48));
                    if ((F::ColSphere_TestLine(c, (p_sw_1).wrapping_add(48), fp.wrapping_add(48), fp.wrapping_add(64), fp.wrapping_add(352), fp.wrapping_add(356), fp.wrapping_add(360))) != 0) {
                        l_m_10 = F32(c.m.r32(fp.wrapping_add(356)));
                        if (((rt::flt(F32(c.m.r32(fp.wrapping_add(360))), l_m_10) as i32)) != 0) {
                            l_m_10 = F32(c.m.r32(fp.wrapping_add(360)));
                        }
                        if (((rt::flt(l_m_10, F32(0x00000000)) as i32)) != 0) {
                            break 'c2;
                        }
                        if (((rt::flt(l_m_10, l_best_6) as i32)) != 0) {
                            l_best_6 = l_m_10;
                            { let v_ = rt::fadd(F32(c.m.r32(fp.wrapping_add(48))), rt::fmul(F32(c.m.r32(fp.wrapping_add(80))), l_best_6)); let a_ = (fp.wrapping_add(96)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                            { let v_ = rt::fadd(F32(c.m.r32((fp.wrapping_add(48)).wrapping_add(4))), rt::fmul(F32(c.m.r32((fp.wrapping_add(80)).wrapping_add(4))), l_best_6)); let a_ = (fp.wrapping_add(96)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                            { let v_ = rt::fadd(F32(c.m.r32((fp.wrapping_add(48)).wrapping_add(8))), rt::fmul(F32(c.m.r32((fp.wrapping_add(80)).wrapping_add(8))), l_best_6)); let a_ = (fp.wrapping_add(96)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                            l_kind_5 = 1i32;
                            F::Vec4_Sub(c, fp, (p_sw_1).wrapping_add(48), fp.wrapping_add(96));
                        }
                    }
                }
                { let o_ = l_i_8; let n_ = (o_).wrapping_add(1); l_i_8 = n_; };
            }
        }
        {
            l_i_8 = 0i32;
            'l3: loop {
                if !(((((l_i_8) < (3i32)) as i32)) != 0) { break 'l3; }
                'c4: {
                    F::Vec4_Copy(c, fp.wrapping_add(96), (p_tri_2).wrapping_add((((l_i_8) as u32)).wrapping_mul(16)));
                    l_j_18 = (l_i_8).wrapping_add(1i32);
                    if (((((l_j_18) == (3i32)) as i32)) != 0) {
                        l_j_18 = 0i32;
                    }
                    F::Vec4_Copy(c, fp.wrapping_add(112), (p_tri_2).wrapping_add((((l_j_18) as u32)).wrapping_mul(16)));
                    l_i0_16 = 0i32;
                    l_i1_17 = 1i32;
                    F::Vec4_Sub(c, fp.wrapping_add(144), fp.wrapping_add(112), fp.wrapping_add(16));
                    F::ColPlane_FromPoints(c, fp.wrapping_add(128), fp.wrapping_add(96), fp.wrapping_add(112), fp.wrapping_add(144));
                    l_d_12 = rt::fadd(F::Vec3_Dot(c, fp.wrapping_add(128), (p_sw_1).wrapping_add(48)), F32(c.m.r32((fp.wrapping_add(128)).wrapping_add(12))));
                    if (((rt::fgt(l_d_12, F32(c.m.r32(((p_sw_1).wrapping_add(48)).wrapping_add(16)))) as i32)) != 0) {
                        break 'c4;
                    }
                    if (((rt::flt(l_d_12, rt::fneg(F32(c.m.r32(((p_sw_1).wrapping_add(48)).wrapping_add(16))))) as i32)) != 0) {
                        break 'c4;
                    }
                    l_rad_11 = { let a0_ = rt::fsub(rt::fmul(F32(c.m.r32(((p_sw_1).wrapping_add(48)).wrapping_add(16))), F32(c.m.r32(((p_sw_1).wrapping_add(48)).wrapping_add(16)))), rt::fmul(l_d_12, l_d_12)); F::Port_sqrtf(c, a0_) };
                    F::ColPlane_ProjectPoint(c, fp.wrapping_add(160), fp.wrapping_add(128), (p_sw_1).wrapping_add(48));
                    { let _ = F::ColLine_DistPoint(c, fp.wrapping_add(160), fp.wrapping_add(96), fp.wrapping_add(112), fp.wrapping_add(176)); };
                    F::Vec4_Sub(c, fp.wrapping_add(192), fp.wrapping_add(176), fp.wrapping_add(160));
                    F::Vec3_Normalize(c, fp.wrapping_add(192), fp.wrapping_add(192));
                    { let v_ = rt::fadd(rt::fmul(F32(c.m.r32(fp.wrapping_add(192))), l_rad_11), F32(c.m.r32(fp.wrapping_add(160)))); let a_ = (fp.wrapping_add(208)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fadd(rt::fmul(F32(c.m.r32((fp.wrapping_add(192)).wrapping_add(4))), l_rad_11), F32(c.m.r32((fp.wrapping_add(160)).wrapping_add(4)))); let a_ = (fp.wrapping_add(208)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fadd(rt::fmul(F32(c.m.r32((fp.wrapping_add(192)).wrapping_add(8))), l_rad_11), F32(c.m.r32((fp.wrapping_add(160)).wrapping_add(8)))); let a_ = (fp.wrapping_add(208)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    l_ax_13 = { let a0_ = F32(c.m.r32(fp.wrapping_add(128))); rt::ffabs(a0_) };
                    l_ay_14 = { let a0_ = F32(c.m.r32((fp.wrapping_add(128)).wrapping_add(4))); rt::ffabs(a0_) };
                    l_az_15 = { let a0_ = F32(c.m.r32((fp.wrapping_add(128)).wrapping_add(8))); rt::ffabs(a0_) };
                    if (((((((rt::fgt(l_ax_13, l_ay_14) as i32)) != 0) && (((rt::fgt(l_ax_13, l_az_15) as i32)) != 0)) as i32)) != 0) {
                        l_i0_16 = 1i32;
                        l_i1_17 = 2i32;
                    } else {
                        if (((rt::fgt(l_ay_14, l_az_15) as i32)) != 0) {
                            l_i0_16 = 0i32;
                            l_i1_17 = 2i32;
                        }
                    }
                    F::Vec4_Add(c, fp.wrapping_add(224), fp.wrapping_add(208), fp.wrapping_add(16));
                    { let v_ = F32(c.m.r32((fp.wrapping_add(208)).wrapping_add((((l_i0_16) as u32)).wrapping_mul(4)))); let a_ = (fp.wrapping_add(240)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(c.m.r32((fp.wrapping_add(208)).wrapping_add((((l_i1_17) as u32)).wrapping_mul(4)))); let a_ = (fp.wrapping_add(240)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(c.m.r32((fp.wrapping_add(224)).wrapping_add((((l_i0_16) as u32)).wrapping_mul(4)))); let a_ = (fp.wrapping_add(256)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(c.m.r32((fp.wrapping_add(224)).wrapping_add((((l_i1_17) as u32)).wrapping_mul(4)))); let a_ = (fp.wrapping_add(256)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(c.m.r32((fp.wrapping_add(96)).wrapping_add((((l_i0_16) as u32)).wrapping_mul(4)))); let a_ = (fp.wrapping_add(272)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(c.m.r32((fp.wrapping_add(96)).wrapping_add((((l_i1_17) as u32)).wrapping_mul(4)))); let a_ = (fp.wrapping_add(272)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(c.m.r32((fp.wrapping_add(112)).wrapping_add((((l_i0_16) as u32)).wrapping_mul(4)))); let a_ = (fp.wrapping_add(288)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(c.m.r32((fp.wrapping_add(112)).wrapping_add((((l_i1_17) as u32)).wrapping_mul(4)))); let a_ = (fp.wrapping_add(288)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    if ((F::ColLine2_Intersect(c, fp.wrapping_add(240), fp.wrapping_add(256), fp.wrapping_add(272), fp.wrapping_add(288), fp.wrapping_add(364))) != 0) {
                        if (((rt::flt(F32(c.m.r32(fp.wrapping_add(364))), F32(0x00000000)) as i32)) != 0) {
                            break 'c4;
                        }
                        { let v_ = rt::fadd(F32(c.m.r32((fp.wrapping_add(208)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(fp.wrapping_add(16))), F32(c.m.r32(fp.wrapping_add(364))))); let a_ = fp.wrapping_add(304); c.m.w32(a_, (v_).0); };
                        { let v_ = rt::fadd(F32(c.m.r32((fp.wrapping_add(208)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))), F32(c.m.r32(fp.wrapping_add(364))))); let a_ = (fp.wrapping_add(304)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                        { let v_ = rt::fadd(F32(c.m.r32((fp.wrapping_add(208)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8))), F32(c.m.r32(fp.wrapping_add(364))))); let a_ = (fp.wrapping_add(304)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                        F::Vec4_Sub(c, fp.wrapping_add(320), fp.wrapping_add(96), fp.wrapping_add(304));
                        F::Vec4_Sub(c, fp.wrapping_add(336), fp.wrapping_add(112), fp.wrapping_add(304));
                        if (((rt::fgt(F::Vec3_Dot(c, fp.wrapping_add(320), fp.wrapping_add(336)), F32(0x00000000)) as i32)) != 0) {
                            break 'c4;
                        }
                        if (((rt::fgt(F32(c.m.r32(fp.wrapping_add(364))), l_best_6) as i32)) != 0) {
                            break 'c4;
                        }
                        l_best_6 = F32(c.m.r32(fp.wrapping_add(364)));
                        F::Vec4_Sub(c, fp, (p_sw_1).wrapping_add(48), fp.wrapping_add(208));
                        l_kind_5 = 2i32;
                    }
                }
                { let o_ = l_i_8; let n_ = (o_).wrapping_add(1); l_i_8 = n_; };
            }
        }
        if (((((l_kind_5) != ((1i32).wrapping_neg())) as i32)) != 0) {
            { let v_ = rt::fadd(F32(c.m.r32(p_sw_1)), rt::fmul(l_best_6, F32(c.m.r32(fp.wrapping_add(16))))); let a_ = p_pos_3; c.m.w32(a_, (v_).0); };
            { let v_ = rt::fadd(F32(c.m.r32((p_sw_1).wrapping_add(4))), rt::fmul(l_best_6, F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))))); let a_ = (p_pos_3).wrapping_add(4); c.m.w32(a_, (v_).0); };
            { let v_ = rt::fadd(F32(c.m.r32((p_sw_1).wrapping_add(8))), rt::fmul(l_best_6, F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8))))); let a_ = (p_pos_3).wrapping_add(8); c.m.w32(a_, (v_).0); };
            { let v_ = l_best_6; let a_ = p_dist_4; c.m.w32(a_, (v_).0); };
        }
        break 'f (((l_kind_5) != ((1i32).wrapping_neg())) as i32);
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "ColSweep_TestTri", &PROBE_N, &[], true); }
    r_
}

pub fn ColSeg_TestTri(c: &mut Ctx, mut p_seg_1: u32, mut p_tri_2: u32, mut p_pos_3: u32, mut p_dist_4: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ColSeg_TestTri", &PROBE_N, &[(p_seg_1) as u32, (p_tri_2) as u32, (p_pos_3) as u32, (p_dist_4) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(96) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 96);
    let mut l_det_5: F32 = F32(0);
    let mut l_u_6: F32 = F32(0);
    let mut l_v_7: F32 = F32(0);
    let mut l_inv_8: F32 = F32(0);
    let mut l_k_9: F32 = F32(0);
    let mut l_ret_10: i32 = 0;
    let r_ = 'f: {
        F::Vec4_Sub(c, fp, (p_tri_2).wrapping_add((((1i32) as u32)).wrapping_mul(16)), (p_tri_2).wrapping_add((((0i32) as u32)).wrapping_mul(16)));
        F::Vec4_Sub(c, fp.wrapping_add(16), (p_tri_2).wrapping_add((((2i32) as u32)).wrapping_mul(16)), (p_tri_2).wrapping_add((((0i32) as u32)).wrapping_mul(16)));
        F::Vec4_Sub(c, fp.wrapping_add(48), p_seg_1, (p_seg_1).wrapping_add(16));
        F::Vec3_Cross(c, fp.wrapping_add(32), fp, fp.wrapping_add(16));
        l_det_5 = F::Vec3_Dot(c, fp.wrapping_add(48), fp.wrapping_add(32));
        if (((rt::fle(l_det_5, F32(0x00000000)) as i32)) != 0) {
            break 'f 0i32;
        }
        F::Vec4_Sub(c, fp.wrapping_add(80), p_seg_1, (p_tri_2).wrapping_add((((0i32) as u32)).wrapping_mul(16)));
        { let v_ = F::Vec3_Dot(c, fp.wrapping_add(80), fp.wrapping_add(32)); let a_ = p_dist_4; c.m.w32(a_, (v_).0); };
        if (((rt::flt(F32(c.m.r32(p_dist_4)), F32(0x00000000)) as i32)) != 0) {
            break 'f 0i32;
        }
        if (((rt::fgt(F32(c.m.r32(p_dist_4)), l_det_5) as i32)) != 0) {
            break 'f 0i32;
        }
        F::Vec3_Cross(c, fp.wrapping_add(64), fp.wrapping_add(48), fp.wrapping_add(80));
        l_u_6 = F::Vec3_Dot(c, fp.wrapping_add(16), fp.wrapping_add(64));
        if (((((((rt::flt(l_u_6, F32(0x00000000)) as i32)) != 0) || (((rt::fgt(l_u_6, l_det_5) as i32)) != 0)) as i32)) != 0) {
            break 'f 0i32;
        }
        l_v_7 = rt::fneg(F::Vec3_Dot(c, fp, fp.wrapping_add(64)));
        if (((((((rt::flt(l_v_7, F32(0x00000000)) as i32)) != 0) || (((rt::fgt(rt::fadd(l_u_6, l_v_7), l_det_5) as i32)) != 0)) as i32)) != 0) {
            break 'f 0i32;
        }
        l_ret_10 = 1i32;
        l_inv_8 = rt::fdiv(F32(0x3f800000), l_det_5);
        { let r_ = l_inv_8; let n_ = rt::fmul(l_u_6, r_); l_u_6 = n_; };
        { let r_ = l_inv_8; let n_ = rt::fmul(l_v_7, r_); l_v_7 = n_; };
        { let r_ = l_inv_8; let a_ = p_dist_4; let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        l_k_9 = rt::fsub(rt::fsub(F32(0x3f800000), l_u_6), l_v_7);
        { let v_ = rt::fadd(rt::fadd(rt::fmul(F32(c.m.r32((p_tri_2).wrapping_add((((0i32) as u32)).wrapping_mul(16)))), l_k_9), rt::fmul(F32(c.m.r32((p_tri_2).wrapping_add((((1i32) as u32)).wrapping_mul(16)))), l_u_6)), rt::fmul(F32(c.m.r32((p_tri_2).wrapping_add((((2i32) as u32)).wrapping_mul(16)))), l_v_7)); let a_ = p_pos_3; c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(rt::fadd(rt::fmul(F32(c.m.r32(((p_tri_2).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4))), l_k_9), rt::fmul(F32(c.m.r32(((p_tri_2).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(4))), l_u_6)), rt::fmul(F32(c.m.r32(((p_tri_2).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(4))), l_v_7)); let a_ = (p_pos_3).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(rt::fadd(rt::fmul(F32(c.m.r32(((p_tri_2).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8))), l_k_9), rt::fmul(F32(c.m.r32(((p_tri_2).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8))), l_u_6)), rt::fmul(F32(c.m.r32(((p_tri_2).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8))), l_v_7)); let a_ = (p_pos_3).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = (p_pos_3).wrapping_add(12); c.m.w32(a_, (v_).0); };
        break 'f l_ret_10;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "ColSeg_TestTri", &PROBE_N, &[], true); }
    r_
}

pub fn ColSphere_ContactObb(c: &mut Ctx, mut p_sph_1: u32, mut p_obb_2: u32, mut p_nrm_3: u32, mut p_pos_4: u32, mut p_depth_5: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ColSphere_ContactObb", &PROBE_N, &[(p_sph_1) as u32, (p_obb_2) as u32, (p_nrm_3) as u32, (p_pos_4) as u32, (p_depth_5) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(64) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 64);
    let mut l_sx_6: F32 = F32(0);
    let mut l_sy_7: F32 = F32(0);
    let mut l_sz_8: F32 = F32(0);
    let mut l_dd_9: F32 = F32(0);
    let mut l_dist_10: F32 = F32(0);
    let mut l_dd_11: F32 = F32(0);
    let mut l_dist_12: F32 = F32(0);
    let mut l_dd_13: F32 = F32(0);
    let mut l_dist_14: F32 = F32(0);
    let mut l_dd_15: F32 = F32(0);
    let mut l_dist_16: F32 = F32(0);
    let r_ = 'f: {
        F::Vec4_Sub(c, fp, p_sph_1, p_obb_2);
        { let v_ = rt::fadd(rt::fadd(rt::fmul(F32(c.m.r32(fp)), F32(c.m.r32(((p_obb_2).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(16))))), rt::fmul(F32(c.m.r32((fp).wrapping_add(4))), F32(c.m.r32((((p_obb_2).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4))))), rt::fmul(F32(c.m.r32((fp).wrapping_add(8))), F32(c.m.r32((((p_obb_2).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8))))); let a_ = fp.wrapping_add(32); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(rt::fadd(rt::fmul(F32(c.m.r32(fp)), F32(c.m.r32(((p_obb_2).wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(16))))), rt::fmul(F32(c.m.r32((fp).wrapping_add(4))), F32(c.m.r32((((p_obb_2).wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(4))))), rt::fmul(F32(c.m.r32((fp).wrapping_add(8))), F32(c.m.r32((((p_obb_2).wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8))))); let a_ = (fp.wrapping_add(32)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(rt::fadd(rt::fmul(F32(c.m.r32(fp)), F32(c.m.r32(((p_obb_2).wrapping_add(32)).wrapping_add((((2i32) as u32)).wrapping_mul(16))))), rt::fmul(F32(c.m.r32((fp).wrapping_add(4))), F32(c.m.r32((((p_obb_2).wrapping_add(32)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(4))))), rt::fmul(F32(c.m.r32((fp).wrapping_add(8))), F32(c.m.r32((((p_obb_2).wrapping_add(32)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8))))); let a_ = (fp.wrapping_add(32)).wrapping_add(8); c.m.w32(a_, (v_).0); };
        l_sz_8 = F32(0x3f800000);
        l_sy_7 = F32(0x3f800000);
        l_sx_6 = F32(0x3f800000);
        F::Vec4_Copy(c, fp.wrapping_add(16), fp.wrapping_add(32));
        if (((rt::flt(F32(c.m.r32(fp.wrapping_add(16))), F32(0x00000000)) as i32)) != 0) {
            { let v_ = rt::fneg(F32(c.m.r32(fp.wrapping_add(16)))); let a_ = fp.wrapping_add(16); c.m.w32(a_, (v_).0); };
            l_sx_6 = F32(0xbf800000);
        }
        if (((rt::flt(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))), F32(0x00000000)) as i32)) != 0) {
            { let v_ = rt::fneg(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4)))); let a_ = (fp.wrapping_add(16)).wrapping_add(4); c.m.w32(a_, (v_).0); };
            l_sy_7 = F32(0xbf800000);
        }
        if (((rt::flt(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8))), F32(0x00000000)) as i32)) != 0) {
            { let v_ = rt::fneg(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8)))); let a_ = (fp.wrapping_add(16)).wrapping_add(8); c.m.w32(a_, (v_).0); };
            l_sz_8 = F32(0xbf800000);
        }
        { let r_ = F32(c.m.r32((p_obb_2).wrapping_add(16))); let a_ = fp.wrapping_add(16); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        { let r_ = F32(c.m.r32(((p_obb_2).wrapping_add(16)).wrapping_add(4))); let a_ = (fp.wrapping_add(16)).wrapping_add(4); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        { let r_ = F32(c.m.r32(((p_obb_2).wrapping_add(16)).wrapping_add(8))); let a_ = (fp.wrapping_add(16)).wrapping_add(8); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        if (((((((((((rt::fgt(F32(c.m.r32(fp.wrapping_add(16))), F32(c.m.r32((p_sph_1).wrapping_add(16)))) as i32)) != 0) || (((rt::fgt(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))), F32(c.m.r32((p_sph_1).wrapping_add(16)))) as i32)) != 0)) as i32)) != 0) || (((rt::fgt(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8))), F32(c.m.r32((p_sph_1).wrapping_add(16)))) as i32)) != 0)) as i32)) != 0) {
            break 'f 0i32;
        }
        if (((rt::flt(F32(c.m.r32(fp.wrapping_add(16))), F32(0x00000000)) as i32)) != 0) {
            if (((rt::flt(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))), F32(0x00000000)) as i32)) != 0) {
                { let v_ = rt::fmul(F32(c.m.r32(((p_obb_2).wrapping_add(32)).wrapping_add((((2i32) as u32)).wrapping_mul(16)))), l_sz_8); let a_ = p_nrm_3; c.m.w32(a_, (v_).0); };
                { let v_ = rt::fmul(F32(c.m.r32(((p_obb_2).wrapping_add(32)).wrapping_add((((2i32) as u32)).wrapping_mul(16)))), l_sz_8); let a_ = (p_nrm_3).wrapping_add(4); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fmul(F32(c.m.r32(((p_obb_2).wrapping_add(32)).wrapping_add((((2i32) as u32)).wrapping_mul(16)))), l_sz_8); let a_ = (p_nrm_3).wrapping_add(8); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fsub(F32(c.m.r32(p_sph_1)), rt::fmul(F32(c.m.r32(p_nrm_3)), F32(c.m.r32((p_sph_1).wrapping_add(16))))); let a_ = p_pos_4; c.m.w32(a_, (v_).0); };
                { let v_ = rt::fsub(F32(c.m.r32((p_sph_1).wrapping_add(4))), rt::fmul(F32(c.m.r32((p_nrm_3).wrapping_add(4))), F32(c.m.r32((p_sph_1).wrapping_add(16))))); let a_ = (p_pos_4).wrapping_add(4); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fsub(F32(c.m.r32((p_sph_1).wrapping_add(8))), rt::fmul(F32(c.m.r32((p_nrm_3).wrapping_add(8))), F32(c.m.r32((p_sph_1).wrapping_add(16))))); let a_ = (p_pos_4).wrapping_add(8); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fsub(F32(c.m.r32((p_sph_1).wrapping_add(16))), F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8)))); let a_ = p_depth_5; c.m.w32(a_, (v_).0); };
                break 'f 1i32;
            } else {
                if (((rt::flt(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8))), F32(0x00000000)) as i32)) != 0) {
                    { let v_ = rt::fmul(F32(c.m.r32(((p_obb_2).wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(16)))), l_sz_8); let a_ = p_nrm_3; c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fmul(F32(c.m.r32(((p_obb_2).wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(16)))), l_sz_8); let a_ = (p_nrm_3).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fmul(F32(c.m.r32(((p_obb_2).wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(16)))), l_sz_8); let a_ = (p_nrm_3).wrapping_add(8); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fsub(F32(c.m.r32(p_sph_1)), rt::fmul(F32(c.m.r32(p_nrm_3)), F32(c.m.r32((p_sph_1).wrapping_add(16))))); let a_ = p_pos_4; c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fsub(F32(c.m.r32((p_sph_1).wrapping_add(4))), rt::fmul(F32(c.m.r32((p_nrm_3).wrapping_add(4))), F32(c.m.r32((p_sph_1).wrapping_add(16))))); let a_ = (p_pos_4).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fsub(F32(c.m.r32((p_sph_1).wrapping_add(8))), rt::fmul(F32(c.m.r32((p_nrm_3).wrapping_add(8))), F32(c.m.r32((p_sph_1).wrapping_add(16))))); let a_ = (p_pos_4).wrapping_add(8); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fsub(F32(c.m.r32((p_sph_1).wrapping_add(16))), F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4)))); let a_ = p_depth_5; c.m.w32(a_, (v_).0); };
                    break 'f 1i32;
                } else {
                    l_dd_9 = rt::fadd(rt::fmul(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))), F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4)))), rt::fmul(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8))), F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8)))));
                    if (((rt::fle(l_dd_9, rt::fmul(F32(c.m.r32((p_sph_1).wrapping_add(16))), F32(c.m.r32((p_sph_1).wrapping_add(16))))) as i32)) != 0) {
                        l_dist_10 = F::Port_sqrtf(c, l_dd_9);
                        { let v_ = F32(0x00000000); let a_ = fp.wrapping_add(48); c.m.w32(a_, (v_).0); };
                        { let v_ = rt::fdiv(rt::fmul(l_sy_7, F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4)))), l_dist_10); let a_ = (fp.wrapping_add(48)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                        { let v_ = rt::fdiv(rt::fmul(l_sz_8, F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8)))), l_dist_10); let a_ = (fp.wrapping_add(48)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                        { let v_ = rt::fadd(rt::fadd(rt::fmul(F32(c.m.r32(((p_obb_2).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(16)))), F32(c.m.r32(fp.wrapping_add(48)))), rt::fmul(F32(c.m.r32((fp.wrapping_add(48)).wrapping_add(4))), F32(c.m.r32(((p_obb_2).wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(16)))))), rt::fmul(F32(c.m.r32((fp.wrapping_add(48)).wrapping_add(8))), F32(c.m.r32(((p_obb_2).wrapping_add(32)).wrapping_add((((2i32) as u32)).wrapping_mul(16)))))); let a_ = p_nrm_3; c.m.w32(a_, (v_).0); };
                        { let v_ = rt::fadd(rt::fadd(rt::fmul(F32(c.m.r32((((p_obb_2).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4))), F32(c.m.r32(fp.wrapping_add(48)))), rt::fmul(F32(c.m.r32((fp.wrapping_add(48)).wrapping_add(4))), F32(c.m.r32((((p_obb_2).wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(4))))), rt::fmul(F32(c.m.r32((fp.wrapping_add(48)).wrapping_add(8))), F32(c.m.r32((((p_obb_2).wrapping_add(32)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(4))))); let a_ = (p_nrm_3).wrapping_add(4); c.m.w32(a_, (v_).0); };
                        { let v_ = rt::fadd(rt::fadd(rt::fmul(F32(c.m.r32((((p_obb_2).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8))), F32(c.m.r32(fp.wrapping_add(48)))), rt::fmul(F32(c.m.r32((fp.wrapping_add(48)).wrapping_add(4))), F32(c.m.r32((((p_obb_2).wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8))))), rt::fmul(F32(c.m.r32((fp.wrapping_add(48)).wrapping_add(8))), F32(c.m.r32((((p_obb_2).wrapping_add(32)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8))))); let a_ = (p_nrm_3).wrapping_add(8); c.m.w32(a_, (v_).0); };
                        { let v_ = rt::fsub(F32(c.m.r32(p_sph_1)), rt::fmul(F32(c.m.r32(p_nrm_3)), F32(c.m.r32((p_sph_1).wrapping_add(16))))); let a_ = p_pos_4; c.m.w32(a_, (v_).0); };
                        { let v_ = rt::fsub(F32(c.m.r32((p_sph_1).wrapping_add(4))), rt::fmul(F32(c.m.r32((p_nrm_3).wrapping_add(4))), F32(c.m.r32((p_sph_1).wrapping_add(16))))); let a_ = (p_pos_4).wrapping_add(4); c.m.w32(a_, (v_).0); };
                        { let v_ = rt::fsub(F32(c.m.r32((p_sph_1).wrapping_add(8))), rt::fmul(F32(c.m.r32((p_nrm_3).wrapping_add(8))), F32(c.m.r32((p_sph_1).wrapping_add(16))))); let a_ = (p_pos_4).wrapping_add(8); c.m.w32(a_, (v_).0); };
                        { let v_ = rt::fsub(F32(c.m.r32((p_sph_1).wrapping_add(16))), l_dist_10); let a_ = p_depth_5; c.m.w32(a_, (v_).0); };
                        break 'f 1i32;
                    }
                }
            }
        } else {
            if (((rt::flt(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))), F32(0x00000000)) as i32)) != 0) {
                if (((rt::flt(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8))), F32(0x00000000)) as i32)) != 0) {
                    { let v_ = rt::fmul(F32(c.m.r32(((p_obb_2).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(16)))), l_sx_6); let a_ = p_nrm_3; c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fmul(F32(c.m.r32((((p_obb_2).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4))), l_sx_6); let a_ = (p_nrm_3).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fmul(F32(c.m.r32((((p_obb_2).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8))), l_sx_6); let a_ = (p_nrm_3).wrapping_add(8); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fsub(F32(c.m.r32(p_sph_1)), rt::fmul(F32(c.m.r32(p_nrm_3)), F32(c.m.r32((p_sph_1).wrapping_add(16))))); let a_ = p_pos_4; c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fsub(F32(c.m.r32((p_sph_1).wrapping_add(4))), rt::fmul(F32(c.m.r32((p_nrm_3).wrapping_add(4))), F32(c.m.r32((p_sph_1).wrapping_add(16))))); let a_ = (p_pos_4).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fsub(F32(c.m.r32((p_sph_1).wrapping_add(8))), rt::fmul(F32(c.m.r32((p_nrm_3).wrapping_add(8))), F32(c.m.r32((p_sph_1).wrapping_add(16))))); let a_ = (p_pos_4).wrapping_add(8); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fsub(F32(c.m.r32((p_sph_1).wrapping_add(16))), F32(c.m.r32(fp.wrapping_add(16)))); let a_ = p_depth_5; c.m.w32(a_, (v_).0); };
                    break 'f 1i32;
                } else {
                    l_dd_11 = rt::fadd(rt::fmul(F32(c.m.r32(fp.wrapping_add(16))), F32(c.m.r32(fp.wrapping_add(16)))), rt::fmul(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8))), F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8)))));
                    if (((rt::fle(l_dd_11, rt::fmul(F32(c.m.r32((p_sph_1).wrapping_add(16))), F32(c.m.r32((p_sph_1).wrapping_add(16))))) as i32)) != 0) {
                        l_dist_12 = F::Port_sqrtf(c, l_dd_11);
                        { let v_ = F32(0x00000000); let a_ = (fp.wrapping_add(48)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                        { let v_ = rt::fdiv(rt::fmul(l_sx_6, F32(c.m.r32(fp.wrapping_add(16)))), l_dist_12); let a_ = fp.wrapping_add(48); c.m.w32(a_, (v_).0); };
                        { let v_ = rt::fdiv(rt::fmul(l_sz_8, F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8)))), l_dist_12); let a_ = (fp.wrapping_add(48)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                        { let v_ = rt::fadd(rt::fadd(rt::fmul(F32(c.m.r32(fp.wrapping_add(48))), F32(c.m.r32(((p_obb_2).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(16))))), rt::fmul(F32(c.m.r32(((p_obb_2).wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(16)))), F32(c.m.r32((fp.wrapping_add(48)).wrapping_add(4))))), rt::fmul(F32(c.m.r32((fp.wrapping_add(48)).wrapping_add(8))), F32(c.m.r32(((p_obb_2).wrapping_add(32)).wrapping_add((((2i32) as u32)).wrapping_mul(16)))))); let a_ = p_nrm_3; c.m.w32(a_, (v_).0); };
                        { let v_ = rt::fadd(rt::fadd(rt::fmul(F32(c.m.r32(fp.wrapping_add(48))), F32(c.m.r32((((p_obb_2).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4)))), rt::fmul(F32(c.m.r32((((p_obb_2).wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(4))), F32(c.m.r32((fp.wrapping_add(48)).wrapping_add(4))))), rt::fmul(F32(c.m.r32((fp.wrapping_add(48)).wrapping_add(8))), F32(c.m.r32((((p_obb_2).wrapping_add(32)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(4))))); let a_ = (p_nrm_3).wrapping_add(4); c.m.w32(a_, (v_).0); };
                        { let v_ = rt::fadd(rt::fadd(rt::fmul(F32(c.m.r32(fp.wrapping_add(48))), F32(c.m.r32((((p_obb_2).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8)))), rt::fmul(F32(c.m.r32((((p_obb_2).wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8))), F32(c.m.r32((fp.wrapping_add(48)).wrapping_add(4))))), rt::fmul(F32(c.m.r32((fp.wrapping_add(48)).wrapping_add(8))), F32(c.m.r32((((p_obb_2).wrapping_add(32)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8))))); let a_ = (p_nrm_3).wrapping_add(8); c.m.w32(a_, (v_).0); };
                        { let v_ = rt::fsub(F32(c.m.r32(p_sph_1)), rt::fmul(F32(c.m.r32(p_nrm_3)), F32(c.m.r32((p_sph_1).wrapping_add(16))))); let a_ = p_pos_4; c.m.w32(a_, (v_).0); };
                        { let v_ = rt::fsub(F32(c.m.r32((p_sph_1).wrapping_add(4))), rt::fmul(F32(c.m.r32((p_nrm_3).wrapping_add(4))), F32(c.m.r32((p_sph_1).wrapping_add(16))))); let a_ = (p_pos_4).wrapping_add(4); c.m.w32(a_, (v_).0); };
                        { let v_ = rt::fsub(F32(c.m.r32((p_sph_1).wrapping_add(8))), rt::fmul(F32(c.m.r32((p_nrm_3).wrapping_add(8))), F32(c.m.r32((p_sph_1).wrapping_add(16))))); let a_ = (p_pos_4).wrapping_add(8); c.m.w32(a_, (v_).0); };
                        { let v_ = rt::fsub(F32(c.m.r32((p_sph_1).wrapping_add(16))), l_dist_12); let a_ = p_depth_5; c.m.w32(a_, (v_).0); };
                        break 'f 1i32;
                    }
                }
            } else {
                if (((rt::flt(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8))), F32(0x00000000)) as i32)) != 0) {
                    l_dd_13 = rt::fadd(rt::fmul(F32(c.m.r32(fp.wrapping_add(16))), F32(c.m.r32(fp.wrapping_add(16)))), rt::fmul(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))), F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4)))));
                    if (((rt::fle(l_dd_13, rt::fmul(F32(c.m.r32((p_sph_1).wrapping_add(16))), F32(c.m.r32((p_sph_1).wrapping_add(16))))) as i32)) != 0) {
                        l_dist_14 = F::Port_sqrtf(c, l_dd_13);
                        { let v_ = F32(0x00000000); let a_ = (fp.wrapping_add(48)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                        { let v_ = rt::fdiv(rt::fmul(l_sx_6, F32(c.m.r32(fp.wrapping_add(16)))), l_dist_14); let a_ = fp.wrapping_add(48); c.m.w32(a_, (v_).0); };
                        { let v_ = rt::fdiv(rt::fmul(l_sy_7, F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4)))), l_dist_14); let a_ = (fp.wrapping_add(48)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                        { let v_ = rt::fadd(rt::fadd(rt::fmul(F32(c.m.r32(fp.wrapping_add(48))), F32(c.m.r32(((p_obb_2).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(16))))), rt::fmul(F32(c.m.r32((fp.wrapping_add(48)).wrapping_add(4))), F32(c.m.r32(((p_obb_2).wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(16)))))), rt::fmul(F32(c.m.r32(((p_obb_2).wrapping_add(32)).wrapping_add((((2i32) as u32)).wrapping_mul(16)))), F32(c.m.r32((fp.wrapping_add(48)).wrapping_add(8))))); let a_ = p_nrm_3; c.m.w32(a_, (v_).0); };
                        { let v_ = rt::fadd(rt::fadd(rt::fmul(F32(c.m.r32(fp.wrapping_add(48))), F32(c.m.r32((((p_obb_2).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4)))), rt::fmul(F32(c.m.r32((fp.wrapping_add(48)).wrapping_add(4))), F32(c.m.r32((((p_obb_2).wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(4))))), rt::fmul(F32(c.m.r32((((p_obb_2).wrapping_add(32)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(4))), F32(c.m.r32((fp.wrapping_add(48)).wrapping_add(8))))); let a_ = (p_nrm_3).wrapping_add(4); c.m.w32(a_, (v_).0); };
                        { let v_ = rt::fadd(rt::fadd(rt::fmul(F32(c.m.r32(fp.wrapping_add(48))), F32(c.m.r32((((p_obb_2).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8)))), rt::fmul(F32(c.m.r32((fp.wrapping_add(48)).wrapping_add(4))), F32(c.m.r32((((p_obb_2).wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8))))), rt::fmul(F32(c.m.r32((((p_obb_2).wrapping_add(32)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8))), F32(c.m.r32((fp.wrapping_add(48)).wrapping_add(8))))); let a_ = (p_nrm_3).wrapping_add(8); c.m.w32(a_, (v_).0); };
                        { let v_ = rt::fsub(F32(c.m.r32(p_sph_1)), rt::fmul(F32(c.m.r32(p_nrm_3)), F32(c.m.r32((p_sph_1).wrapping_add(16))))); let a_ = p_pos_4; c.m.w32(a_, (v_).0); };
                        { let v_ = rt::fsub(F32(c.m.r32((p_sph_1).wrapping_add(4))), rt::fmul(F32(c.m.r32((p_nrm_3).wrapping_add(4))), F32(c.m.r32((p_sph_1).wrapping_add(16))))); let a_ = (p_pos_4).wrapping_add(4); c.m.w32(a_, (v_).0); };
                        { let v_ = rt::fsub(F32(c.m.r32((p_sph_1).wrapping_add(8))), rt::fmul(F32(c.m.r32((p_nrm_3).wrapping_add(8))), F32(c.m.r32((p_sph_1).wrapping_add(16))))); let a_ = (p_pos_4).wrapping_add(8); c.m.w32(a_, (v_).0); };
                        { let v_ = rt::fsub(F32(c.m.r32((p_sph_1).wrapping_add(16))), l_dist_14); let a_ = p_depth_5; c.m.w32(a_, (v_).0); };
                        break 'f 1i32;
                    }
                } else {
                    l_dd_15 = rt::fadd(rt::fadd(rt::fmul(F32(c.m.r32(fp.wrapping_add(16))), F32(c.m.r32(fp.wrapping_add(16)))), rt::fmul(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))), F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))))), rt::fmul(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8))), F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8)))));
                    if (((rt::fle(l_dd_15, rt::fmul(F32(c.m.r32((p_sph_1).wrapping_add(16))), F32(c.m.r32((p_sph_1).wrapping_add(16))))) as i32)) != 0) {
                        l_dist_16 = F::Port_sqrtf(c, l_dd_15);
                        { let v_ = rt::fdiv(rt::fmul(l_sx_6, F32(c.m.r32(fp.wrapping_add(16)))), l_dist_16); let a_ = fp.wrapping_add(48); c.m.w32(a_, (v_).0); };
                        { let v_ = rt::fdiv(rt::fmul(l_sy_7, F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4)))), l_dist_16); let a_ = (fp.wrapping_add(48)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                        { let v_ = rt::fdiv(rt::fmul(l_sz_8, F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8)))), l_dist_16); let a_ = (fp.wrapping_add(48)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                        { let v_ = rt::fadd(rt::fadd(rt::fmul(F32(c.m.r32(fp.wrapping_add(48))), F32(c.m.r32(((p_obb_2).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(16))))), rt::fmul(F32(c.m.r32((fp.wrapping_add(48)).wrapping_add(4))), F32(c.m.r32(((p_obb_2).wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(16)))))), rt::fmul(F32(c.m.r32((fp.wrapping_add(48)).wrapping_add(8))), F32(c.m.r32(((p_obb_2).wrapping_add(32)).wrapping_add((((2i32) as u32)).wrapping_mul(16)))))); let a_ = p_nrm_3; c.m.w32(a_, (v_).0); };
                        { let v_ = rt::fadd(rt::fadd(rt::fmul(F32(c.m.r32(fp.wrapping_add(48))), F32(c.m.r32((((p_obb_2).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4)))), rt::fmul(F32(c.m.r32((fp.wrapping_add(48)).wrapping_add(4))), F32(c.m.r32((((p_obb_2).wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(4))))), rt::fmul(F32(c.m.r32((fp.wrapping_add(48)).wrapping_add(8))), F32(c.m.r32((((p_obb_2).wrapping_add(32)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(4))))); let a_ = (p_nrm_3).wrapping_add(4); c.m.w32(a_, (v_).0); };
                        { let v_ = rt::fadd(rt::fadd(rt::fmul(F32(c.m.r32(fp.wrapping_add(48))), F32(c.m.r32((((p_obb_2).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8)))), rt::fmul(F32(c.m.r32((fp.wrapping_add(48)).wrapping_add(4))), F32(c.m.r32((((p_obb_2).wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8))))), rt::fmul(F32(c.m.r32((fp.wrapping_add(48)).wrapping_add(8))), F32(c.m.r32((((p_obb_2).wrapping_add(32)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8))))); let a_ = (p_nrm_3).wrapping_add(8); c.m.w32(a_, (v_).0); };
                        { let v_ = rt::fsub(F32(c.m.r32(p_sph_1)), rt::fmul(F32(c.m.r32(p_nrm_3)), F32(c.m.r32((p_sph_1).wrapping_add(16))))); let a_ = p_pos_4; c.m.w32(a_, (v_).0); };
                        { let v_ = rt::fsub(F32(c.m.r32((p_sph_1).wrapping_add(4))), rt::fmul(F32(c.m.r32((p_nrm_3).wrapping_add(4))), F32(c.m.r32((p_sph_1).wrapping_add(16))))); let a_ = (p_pos_4).wrapping_add(4); c.m.w32(a_, (v_).0); };
                        { let v_ = rt::fsub(F32(c.m.r32((p_sph_1).wrapping_add(8))), rt::fmul(F32(c.m.r32((p_nrm_3).wrapping_add(8))), F32(c.m.r32((p_sph_1).wrapping_add(16))))); let a_ = (p_pos_4).wrapping_add(8); c.m.w32(a_, (v_).0); };
                        { let v_ = rt::fsub(F32(c.m.r32((p_sph_1).wrapping_add(16))), l_dist_16); let a_ = p_depth_5; c.m.w32(a_, (v_).0); };
                        break 'f 1i32;
                    }
                }
            }
        }
        break 'f 0i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "ColSphere_ContactObb", &PROBE_N, &[], true); }
    r_
}

pub fn ColSphere_TestObb(c: &mut Ctx, mut p_sph_1: u32, mut p_obb_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ColSphere_TestObb", &PROBE_N, &[(p_sph_1) as u32, (p_obb_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let r_ = 'f: {
        F::Vec4_Sub(c, fp, p_sph_1, p_obb_2);
        { let v_ = rt::fadd(rt::fadd(rt::fmul(F32(c.m.r32(fp)), F32(c.m.r32(((p_obb_2).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(16))))), rt::fmul(F32(c.m.r32((fp).wrapping_add(4))), F32(c.m.r32((((p_obb_2).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4))))), rt::fmul(F32(c.m.r32((fp).wrapping_add(8))), F32(c.m.r32((((p_obb_2).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8))))); let a_ = fp.wrapping_add(32); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(rt::fadd(rt::fmul(F32(c.m.r32(fp)), F32(c.m.r32(((p_obb_2).wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(16))))), rt::fmul(F32(c.m.r32((fp).wrapping_add(4))), F32(c.m.r32((((p_obb_2).wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(4))))), rt::fmul(F32(c.m.r32((fp).wrapping_add(8))), F32(c.m.r32((((p_obb_2).wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8))))); let a_ = (fp.wrapping_add(32)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(rt::fadd(rt::fmul(F32(c.m.r32(fp)), F32(c.m.r32(((p_obb_2).wrapping_add(32)).wrapping_add((((2i32) as u32)).wrapping_mul(16))))), rt::fmul(F32(c.m.r32((fp).wrapping_add(4))), F32(c.m.r32((((p_obb_2).wrapping_add(32)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(4))))), rt::fmul(F32(c.m.r32((fp).wrapping_add(8))), F32(c.m.r32((((p_obb_2).wrapping_add(32)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8))))); let a_ = (fp.wrapping_add(32)).wrapping_add(8); c.m.w32(a_, (v_).0); };
        F::Vec4_Copy(c, fp.wrapping_add(16), fp.wrapping_add(32));
        if (((rt::flt(F32(c.m.r32(fp.wrapping_add(16))), F32(0x00000000)) as i32)) != 0) {
            { let v_ = rt::fneg(F32(c.m.r32(fp.wrapping_add(16)))); let a_ = fp.wrapping_add(16); c.m.w32(a_, (v_).0); };
        }
        if (((rt::flt(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))), F32(0x00000000)) as i32)) != 0) {
            { let v_ = rt::fneg(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4)))); let a_ = (fp.wrapping_add(16)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        }
        if (((rt::flt(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8))), F32(0x00000000)) as i32)) != 0) {
            { let v_ = rt::fneg(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8)))); let a_ = (fp.wrapping_add(16)).wrapping_add(8); c.m.w32(a_, (v_).0); };
        }
        { let r_ = F32(c.m.r32((p_obb_2).wrapping_add(16))); let a_ = fp.wrapping_add(16); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        { let r_ = F32(c.m.r32(((p_obb_2).wrapping_add(16)).wrapping_add(4))); let a_ = (fp.wrapping_add(16)).wrapping_add(4); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        { let r_ = F32(c.m.r32(((p_obb_2).wrapping_add(16)).wrapping_add(8))); let a_ = (fp.wrapping_add(16)).wrapping_add(8); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        if (((((((((((rt::fgt(F32(c.m.r32(fp.wrapping_add(16))), F32(c.m.r32((p_sph_1).wrapping_add(16)))) as i32)) != 0) || (((rt::fgt(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))), F32(c.m.r32((p_sph_1).wrapping_add(16)))) as i32)) != 0)) as i32)) != 0) || (((rt::fgt(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8))), F32(c.m.r32((p_sph_1).wrapping_add(16)))) as i32)) != 0)) as i32)) != 0) {
            break 'f 0i32;
        }
        if (((rt::flt(F32(c.m.r32(fp.wrapping_add(16))), F32(0x00000000)) as i32)) != 0) {
            if (((rt::flt(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))), F32(0x00000000)) as i32)) != 0) {
                break 'f 1i32;
            }
            if (((rt::flt(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8))), F32(0x00000000)) as i32)) != 0) {
                break 'f 1i32;
            }
            break 'f (rt::fle(rt::fadd(rt::fmul(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))), F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4)))), rt::fmul(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8))), F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8))))), rt::fmul(F32(c.m.r32((p_sph_1).wrapping_add(16))), F32(c.m.r32((p_sph_1).wrapping_add(16))))) as i32);
        }
        if (((rt::flt(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))), F32(0x00000000)) as i32)) != 0) {
            if (((rt::flt(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8))), F32(0x00000000)) as i32)) != 0) {
                break 'f 1i32;
            }
            break 'f (rt::fle(rt::fadd(rt::fmul(F32(c.m.r32(fp.wrapping_add(16))), F32(c.m.r32(fp.wrapping_add(16)))), rt::fmul(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8))), F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8))))), rt::fmul(F32(c.m.r32((p_sph_1).wrapping_add(16))), F32(c.m.r32((p_sph_1).wrapping_add(16))))) as i32);
        }
        if (((rt::flt(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8))), F32(0x00000000)) as i32)) != 0) {
            if (((rt::fle(rt::fadd(rt::fmul(F32(c.m.r32(fp.wrapping_add(16))), F32(c.m.r32(fp.wrapping_add(16)))), rt::fmul(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))), F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))))), rt::fmul(F32(c.m.r32((p_sph_1).wrapping_add(16))), F32(c.m.r32((p_sph_1).wrapping_add(16))))) as i32)) != 0) {
                break 'f 1i32;
            }
        } else {
            if (((rt::fle(rt::fadd(rt::fadd(rt::fmul(F32(c.m.r32(fp.wrapping_add(16))), F32(c.m.r32(fp.wrapping_add(16)))), rt::fmul(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))), F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))))), rt::fmul(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8))), F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8))))), rt::fmul(F32(c.m.r32((p_sph_1).wrapping_add(16))), F32(c.m.r32((p_sph_1).wrapping_add(16))))) as i32)) != 0) {
                break 'f 1i32;
            }
        }
        break 'f 0i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "ColSphere_TestObb", &PROBE_N, &[], true); }
    r_
}

pub fn ColSphere_TestRay(c: &mut Ctx, mut p_sph_1: u32, mut p_ray_2: u32, mut p_t_3: u32, mut p_pos_4: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ColSphere_TestRay", &PROBE_N, &[(p_sph_1) as u32, (p_ray_2) as u32, (p_t_3) as u32, (p_pos_4) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_b_5: F32 = F32(0);
    let mut l_c_6: F32 = F32(0);
    let mut l_disc_7: F32 = F32(0);
    let r_ = 'f: {
        F::Vec4_Sub(c, fp, p_ray_2, p_sph_1);
        l_b_5 = F::Vec3_Dot(c, fp, (p_ray_2).wrapping_add(16));
        l_c_6 = rt::fsub(F::Vec3_Dot(c, fp, fp), rt::fmul(F32(c.m.r32((p_sph_1).wrapping_add(16))), F32(c.m.r32((p_sph_1).wrapping_add(16)))));
        if (((((((rt::fgt(l_c_6, F32(0x00000000)) as i32)) != 0) && (((rt::fgt(l_b_5, F32(0x00000000)) as i32)) != 0)) as i32)) != 0) {
            break 'f 0i32;
        }
        l_disc_7 = rt::fsub(rt::fmul(l_b_5, l_b_5), l_c_6);
        if (((rt::flt(l_disc_7, F32(0x00000000)) as i32)) != 0) {
            break 'f 0i32;
        }
        { let v_ = rt::fsub(rt::fneg(l_b_5), F::Port_sqrtf(c, l_disc_7)); let a_ = p_t_3; c.m.w32(a_, (v_).0); };
        if (((rt::flt(F32(c.m.r32(p_t_3)), F32(0x00000000)) as i32)) != 0) {
            { let v_ = F32(0x00000000); let a_ = p_t_3; c.m.w32(a_, (v_).0); };
        }
        { let v_ = rt::fadd(F32(c.m.r32(p_ray_2)), rt::fmul(F32(c.m.r32(p_t_3)), F32(c.m.r32((p_ray_2).wrapping_add(16))))); let a_ = p_pos_4; c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32((p_ray_2).wrapping_add(4))), rt::fmul(F32(c.m.r32(p_t_3)), F32(c.m.r32(((p_ray_2).wrapping_add(16)).wrapping_add(4))))); let a_ = (p_pos_4).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32((p_ray_2).wrapping_add(8))), rt::fmul(F32(c.m.r32(p_t_3)), F32(c.m.r32(((p_ray_2).wrapping_add(16)).wrapping_add(8))))); let a_ = (p_pos_4).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = (p_pos_4).wrapping_add(12); c.m.w32(a_, (v_).0); };
        break 'f 1i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "ColSphere_TestRay", &PROBE_N, &[], true); }
    r_
}

pub fn ColSphere_TestLine(c: &mut Ctx, mut p_sph_1: u32, mut p_a_2: u32, mut p_b_3: u32, mut p_count_4: u32, mut p_t0_5: u32, mut p_t1_6: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ColSphere_TestLine", &PROBE_N, &[(p_sph_1) as u32, (p_a_2) as u32, (p_b_3) as u32, (p_count_4) as u32, (p_t0_5) as u32, (p_t1_6) as u32], false); }
    let mut l_qa_7: F32 = F32(0);
    let mut l_qb_8: F32 = F32(0);
    let mut l_qc_9: F32 = F32(0);
    let mut l_disc_10: F32 = F32(0);
    let r_ = 'f: {
    l_qa_7 = rt::fadd(rt::fadd(rt::fmul(rt::fsub(F32(c.m.r32(p_b_3)), F32(c.m.r32(p_a_2))), rt::fsub(F32(c.m.r32(p_b_3)), F32(c.m.r32(p_a_2)))), rt::fmul(rt::fsub(F32(c.m.r32((p_b_3).wrapping_add(4))), F32(c.m.r32((p_a_2).wrapping_add(4)))), rt::fsub(F32(c.m.r32((p_b_3).wrapping_add(4))), F32(c.m.r32((p_a_2).wrapping_add(4)))))), rt::fmul(rt::fsub(F32(c.m.r32((p_b_3).wrapping_add(8))), F32(c.m.r32((p_a_2).wrapping_add(8)))), rt::fsub(F32(c.m.r32((p_b_3).wrapping_add(8))), F32(c.m.r32((p_a_2).wrapping_add(8))))));
    l_qb_8 = rt::fmul(F32(0x40000000), rt::fadd(rt::fadd(rt::fmul(rt::fsub(F32(c.m.r32(p_b_3)), F32(c.m.r32(p_a_2))), rt::fsub(F32(c.m.r32(p_a_2)), F32(c.m.r32(p_sph_1)))), rt::fmul(rt::fsub(F32(c.m.r32((p_b_3).wrapping_add(4))), F32(c.m.r32((p_a_2).wrapping_add(4)))), rt::fsub(F32(c.m.r32((p_a_2).wrapping_add(4))), F32(c.m.r32((p_sph_1).wrapping_add(4)))))), rt::fmul(rt::fsub(F32(c.m.r32((p_b_3).wrapping_add(8))), F32(c.m.r32((p_a_2).wrapping_add(8)))), rt::fsub(F32(c.m.r32((p_a_2).wrapping_add(8))), F32(c.m.r32((p_sph_1).wrapping_add(8)))))));
    l_qc_9 = rt::fsub(rt::fsub(rt::fadd(rt::fadd(rt::fadd(rt::fadd(rt::fadd(rt::fmul(F32(c.m.r32(p_sph_1)), F32(c.m.r32(p_sph_1))), rt::fmul(F32(c.m.r32((p_sph_1).wrapping_add(4))), F32(c.m.r32((p_sph_1).wrapping_add(4))))), rt::fmul(F32(c.m.r32((p_sph_1).wrapping_add(8))), F32(c.m.r32((p_sph_1).wrapping_add(8))))), rt::fmul(F32(c.m.r32(p_a_2)), F32(c.m.r32(p_a_2)))), rt::fmul(F32(c.m.r32((p_a_2).wrapping_add(4))), F32(c.m.r32((p_a_2).wrapping_add(4))))), rt::fmul(F32(c.m.r32((p_a_2).wrapping_add(8))), F32(c.m.r32((p_a_2).wrapping_add(8))))), rt::fmul(F32(0x40000000), rt::fadd(rt::fadd(rt::fmul(F32(c.m.r32(p_sph_1)), F32(c.m.r32(p_a_2))), rt::fmul(F32(c.m.r32((p_sph_1).wrapping_add(4))), F32(c.m.r32((p_a_2).wrapping_add(4))))), rt::fmul(F32(c.m.r32((p_sph_1).wrapping_add(8))), F32(c.m.r32((p_a_2).wrapping_add(8))))))), rt::fmul(F32(c.m.r32((p_sph_1).wrapping_add(16))), F32(c.m.r32((p_sph_1).wrapping_add(16)))));
    l_disc_10 = rt::fsub(rt::fmul(l_qb_8, l_qb_8), rt::fmul(rt::fmul(F32(0x40800000), l_qa_7), l_qc_9));
    if (((rt::flt(l_disc_10, F32(0x00000000)) as i32)) != 0) {
        break 'f 0i32;
    }
    if (((rt::feq(l_qa_7, F32(0x00000000)) as i32)) != 0) {
        l_qa_7 = F32(0x38d1b717);
    }
    if (((rt::feq(l_disc_10, F32(0x00000000)) as i32)) != 0) {
        if (((((p_count_4) != (((0i32) as u32))) as i32)) != 0) {
            { let v_ = 1i32; let a_ = p_count_4; c.m.w32(a_, (v_) as u32); };
        }
        if (((((p_t0_5) != (((0i32) as u32))) as i32)) != 0) {
            { let v_ = rt::fdiv(rt::fneg(l_qb_8), rt::fmul(F32(0x40000000), l_qa_7)); let a_ = p_t0_5; c.m.w32(a_, (v_).0); };
        }
        break 'f 1i32;
    }
    if (((((p_count_4) != (((0i32) as u32))) as i32)) != 0) {
        { let v_ = 2i32; let a_ = p_count_4; c.m.w32(a_, (v_) as u32); };
    }
    if (((((p_t0_5) != (((0i32) as u32))) as i32)) != 0) {
        { let v_ = rt::fdiv(rt::fadd(rt::fneg(l_qb_8), F::Port_sqrtf(c, rt::fsub(rt::fmul(l_qb_8, l_qb_8), rt::fmul(rt::fmul(F32(0x40800000), l_qa_7), l_qc_9)))), rt::fmul(F32(0x40000000), l_qa_7)); let a_ = p_t0_5; c.m.w32(a_, (v_).0); };
    }
    if (((((p_t1_6) != (((0i32) as u32))) as i32)) != 0) {
        { let v_ = rt::fdiv(rt::fsub(rt::fneg(l_qb_8), F::Port_sqrtf(c, rt::fsub(rt::fmul(l_qb_8, l_qb_8), rt::fmul(rt::fmul(F32(0x40800000), l_qa_7), l_qc_9)))), rt::fmul(F32(0x40000000), l_qa_7)); let a_ = p_t1_6; c.m.w32(a_, (v_).0); };
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "ColSphere_TestLine", &PROBE_N, &[], true); }
    r_
}

pub fn ColSphere_TestSphere(c: &mut Ctx, mut p_a_1: u32, mut p_b_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ColSphere_TestSphere", &PROBE_N, &[(p_a_1) as u32, (p_b_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_r_3: F32 = F32(0);
    let r_ = 'f: {
        l_r_3 = rt::fadd(F32(c.m.r32((p_a_1).wrapping_add(16))), F32(c.m.r32((p_b_2).wrapping_add(16))));
        F::Vec4_Sub(c, fp, p_a_1, p_b_2);
        if (((rt::fle(F::Vec3_LengthSq(c, fp), rt::fmul(l_r_3, l_r_3)) as i32)) != 0) {
            break 'f 1i32;
        }
        break 'f 0i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "ColSphere_TestSphere", &PROBE_N, &[], true); }
    r_
}

pub fn ColSphere_ContactSphere(c: &mut Ctx, mut p_a_1: u32, mut p_b_2: u32, mut p_nrm_3: u32, mut p_pos_4: u32, mut p_depth_5: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ColSphere_ContactSphere", &PROBE_N, &[(p_a_1) as u32, (p_b_2) as u32, (p_nrm_3) as u32, (p_pos_4) as u32, (p_depth_5) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_r_6: F32 = F32(0);
    let mut l_dist_7: F32 = F32(0);
    let r_ = 'f: {
        l_r_6 = rt::fadd(F32(c.m.r32((p_a_1).wrapping_add(16))), F32(c.m.r32((p_b_2).wrapping_add(16))));
        F::Vec4_Sub(c, fp, p_a_1, p_b_2);
        l_dist_7 = F::Vec3_LengthSq(c, fp);
        if ((((!(((rt::fle(l_dist_7, rt::fmul(l_r_6, l_r_6)) as i32)) != 0)) as i32)) != 0) {
            break 'f 0i32;
        }
        l_dist_7 = F::Port_sqrtf(c, l_dist_7);
        { let v_ = rt::fsub(l_r_6, l_dist_7); let a_ = p_depth_5; c.m.w32(a_, (v_).0); };
        { let v_ = rt::fdiv(F32(c.m.r32(fp)), l_dist_7); let a_ = p_nrm_3; c.m.w32(a_, (v_).0); };
        { let v_ = rt::fdiv(F32(c.m.r32((fp).wrapping_add(4))), l_dist_7); let a_ = (p_nrm_3).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fdiv(F32(c.m.r32((fp).wrapping_add(8))), l_dist_7); let a_ = (p_nrm_3).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(p_a_1)), rt::fmul(F32(c.m.r32(p_nrm_3)), rt::fsub(F32(c.m.r32((p_b_2).wrapping_add(16))), rt::fmul(F32(c.m.r32(p_depth_5)), F32(0x3f000000))))); let a_ = p_pos_4; c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32((p_a_1).wrapping_add(4))), rt::fmul(F32(c.m.r32((p_nrm_3).wrapping_add(4))), rt::fsub(F32(c.m.r32((p_b_2).wrapping_add(16))), rt::fmul(F32(c.m.r32(p_depth_5)), F32(0x3f000000))))); let a_ = (p_pos_4).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32((p_a_1).wrapping_add(8))), rt::fmul(F32(c.m.r32((p_nrm_3).wrapping_add(8))), rt::fsub(F32(c.m.r32((p_b_2).wrapping_add(16))), rt::fmul(F32(c.m.r32(p_depth_5)), F32(0x3f000000))))); let a_ = (p_pos_4).wrapping_add(8); c.m.w32(a_, (v_).0); };
        F::Vec3_Normalize(c, p_nrm_3, p_nrm_3);
        break 'f 1i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "ColSphere_ContactSphere", &PROBE_N, &[], true); }
    r_
}

pub fn ColSphere_SweepSphere(c: &mut Ctx, mut p_a_1: u32, mut p_b_2: u32, mut p_moveA_3: u32, mut p_moveB_4: u32, mut p_t_5: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ColSphere_SweepSphere", &PROBE_N, &[(p_a_1) as u32, (p_b_2) as u32, (p_moveA_3) as u32, (p_moveB_4) as u32, (p_t_5) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_r_6: F32 = F32(0);
    let mut l_qa_7: F32 = F32(0);
    let mut l_qb_8: F32 = F32(0);
    let mut l_qc_9: F32 = F32(0);
    let mut l_disc_10: F32 = F32(0);
    let r_ = 'f: {
        F::Vec4_Sub(c, fp, p_b_2, p_a_1);
        F::Vec4_Sub(c, fp.wrapping_add(16), p_moveB_4, p_moveA_3);
        l_r_6 = rt::fadd(F32(c.m.r32((p_b_2).wrapping_add(16))), F32(c.m.r32((p_a_1).wrapping_add(16))));
        l_qc_9 = rt::fsub(F::Vec3_Dot(c, fp, fp), rt::fmul(l_r_6, l_r_6));
        if (((rt::flt(l_qc_9, F32(0x00000000)) as i32)) != 0) {
            { let v_ = F32(0x00000000); let a_ = p_t_5; c.m.w32(a_, (v_).0); };
            break 'f 1i32;
        }
        l_qa_7 = F::Vec3_Dot(c, fp.wrapping_add(16), fp.wrapping_add(16));
        if (((rt::flt(l_qa_7, F32(0x03c80000)) as i32)) != 0) {
            break 'f 0i32;
        }
        l_qb_8 = F::Vec3_Dot(c, fp.wrapping_add(16), fp);
        if (((rt::fge(l_qb_8, F32(0x00000000)) as i32)) != 0) {
            break 'f 0i32;
        }
        l_disc_10 = rt::fsub(rt::fmul(l_qb_8, l_qb_8), rt::fmul(l_qa_7, l_qc_9));
        if (((rt::flt(l_disc_10, F32(0x00000000)) as i32)) != 0) {
            break 'f 0i32;
        }
        { let v_ = rt::fdiv(rt::fsub(rt::fneg(l_qb_8), F::Port_sqrtf(c, l_disc_10)), l_qa_7); let a_ = p_t_5; c.m.w32(a_, (v_).0); };
        break 'f 1i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "ColSphere_SweepSphere", &PROBE_N, &[], true); }
    r_
}

pub fn ColSphere_SeparateSphere(c: &mut Ctx, mut p_a_1: u32, mut p_b_2: u32, mut p_posA_3: u32, mut p_posB_4: u32, mut p_nrm_5: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ColSphere_SeparateSphere", &PROBE_N, &[(p_a_1) as u32, (p_b_2) as u32, (p_posA_3) as u32, (p_posB_4) as u32, (p_nrm_5) as u32], false); }
    let mut l_dist_6: F32 = F32(0);
    let mut l_r_7: F32 = F32(0);
    let mut l_push_8: F32 = F32(0);
    let mut l_inv_9: F32 = F32(0);
    let r_ = 'f: {
    F::Vec4_Sub(c, p_nrm_5, p_b_2, p_a_1);
    l_dist_6 = { let a0_ = F::Vec3_Dot(c, p_nrm_5, p_nrm_5); F::Port_sqrtf(c, a0_) };
    l_push_8 = l_dist_6;
    if (((rt::fne(l_push_8, F32(0x00000000)) as i32)) != 0) {
        l_inv_9 = rt::fdiv(F32(0x3f800000), l_push_8);
        { let r_ = l_inv_9; let a_ = p_nrm_5; let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        { let r_ = l_inv_9; let a_ = (p_nrm_5).wrapping_add(4); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        { let r_ = l_inv_9; let a_ = (p_nrm_5).wrapping_add(8); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        { let v_ = F32(0x3f800000); let a_ = (p_nrm_5).wrapping_add(12); c.m.w32(a_, (v_).0); };
    } else {
        { let v_ = F32(0x3f800000); let a_ = p_nrm_5; c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (p_nrm_5).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (p_nrm_5).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = (p_nrm_5).wrapping_add(12); c.m.w32(a_, (v_).0); };
    }
    l_r_7 = rt::fadd(F32(c.m.r32((p_a_1).wrapping_add(16))), F32(c.m.r32((p_b_2).wrapping_add(16))));
    if (((rt::fgt(l_dist_6, l_r_7) as i32)) != 0) {
        if (((rt::fgt(rt::ffabs(rt::fsub(l_dist_6, l_r_7)), F32(0x3a83126e)) as i32)) != 0) {
            break 'f 0i32;
        }
    }
    l_push_8 = rt::fadd(rt::fmul(rt::fsub(l_r_7, l_dist_6), F32(0x3f000000)), F32(0x3a83126e));
    { let v_ = rt::fsub(F32(c.m.r32(p_a_1)), rt::fmul(F32(c.m.r32(p_nrm_5)), l_push_8)); let a_ = p_posA_3; c.m.w32(a_, (v_).0); };
    { let v_ = rt::fsub(F32(c.m.r32((p_a_1).wrapping_add(4))), rt::fmul(F32(c.m.r32((p_nrm_5).wrapping_add(4))), l_push_8)); let a_ = (p_posA_3).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fsub(F32(c.m.r32((p_a_1).wrapping_add(8))), rt::fmul(F32(c.m.r32((p_nrm_5).wrapping_add(8))), l_push_8)); let a_ = (p_posA_3).wrapping_add(8); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fadd(F32(c.m.r32(p_b_2)), rt::fmul(F32(c.m.r32(p_nrm_5)), l_push_8)); let a_ = p_posB_4; c.m.w32(a_, (v_).0); };
    { let v_ = rt::fadd(F32(c.m.r32((p_b_2).wrapping_add(4))), rt::fmul(F32(c.m.r32((p_nrm_5).wrapping_add(4))), l_push_8)); let a_ = (p_posB_4).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fadd(F32(c.m.r32((p_b_2).wrapping_add(8))), rt::fmul(F32(c.m.r32((p_nrm_5).wrapping_add(8))), l_push_8)); let a_ = (p_posB_4).wrapping_add(8); c.m.w32(a_, (v_).0); };
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "ColSphere_SeparateSphere", &PROBE_N, &[], true); }
    r_
}

pub fn ColTri_DistSqPoint(c: &mut Ctx, mut p_pt_1: u32, mut p_tri_2: u32, mut p_s_3: u32, mut p_t_4: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ColTri_DistSqPoint", &PROBE_N, &[(p_pt_1) as u32, (p_tri_2) as u32, (p_s_3) as u32, (p_t_4) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let mut l_fA00_5: F32 = F32(0);
    let mut l_fA01_6: F32 = F32(0);
    let mut l_fA11_7: F32 = F32(0);
    let mut l_fB0_8: F32 = F32(0);
    let mut l_fB1_9: F32 = F32(0);
    let mut l_fC_10: F32 = F32(0);
    let mut l_fDet_11: F32 = F32(0);
    let mut l_fS_12: F32 = F32(0);
    let mut l_fT_13: F32 = F32(0);
    let mut l_fSqrDist_14: F32 = F32(0);
    let mut l_fInvDet_15: F32 = F32(0);
    let mut l_fTmp0_16: F32 = F32(0);
    let mut l_fTmp1_17: F32 = F32(0);
    let mut l_fNumer_18: F32 = F32(0);
    let mut l_fDenom_19: F32 = F32(0);
    let r_ = 'f: {
        F::Vec4_Sub(c, fp, (p_tri_2).wrapping_add((((1i32) as u32)).wrapping_mul(16)), (p_tri_2).wrapping_add((((0i32) as u32)).wrapping_mul(16)));
        F::Vec4_Sub(c, fp.wrapping_add(16), (p_tri_2).wrapping_add((((2i32) as u32)).wrapping_mul(16)), (p_tri_2).wrapping_add((((0i32) as u32)).wrapping_mul(16)));
        F::Vec4_Sub(c, fp.wrapping_add(32), (p_tri_2).wrapping_add((((0i32) as u32)).wrapping_mul(16)), p_pt_1);
        l_fA00_5 = F::Vec3_Dot(c, fp, fp);
        l_fA01_6 = F::Vec3_Dot(c, fp, fp.wrapping_add(16));
        l_fA11_7 = F::Vec3_Dot(c, fp.wrapping_add(16), fp.wrapping_add(16));
        l_fB0_8 = F::Vec3_Dot(c, fp.wrapping_add(32), fp);
        l_fB1_9 = F::Vec3_Dot(c, fp.wrapping_add(32), fp.wrapping_add(16));
        l_fC_10 = F::Vec3_Dot(c, fp.wrapping_add(32), fp.wrapping_add(32));
        l_fDet_11 = rt::ffabs(rt::fsub(rt::fmul(l_fA00_5, l_fA11_7), rt::fmul(l_fA01_6, l_fA01_6)));
        l_fS_12 = rt::fsub(rt::fmul(l_fA01_6, l_fB1_9), rt::fmul(l_fA11_7, l_fB0_8));
        l_fT_13 = rt::fsub(rt::fmul(l_fA01_6, l_fB0_8), rt::fmul(l_fA00_5, l_fB1_9));
        if (((rt::flt(rt::ffabs(l_fDet_11), F32(0x3089705f)) as i32)) != 0) {
            break 'f F32(0x4cbebc20);
        }
        if (((rt::fle(rt::fadd(l_fS_12, l_fT_13), l_fDet_11) as i32)) != 0) {
            if (((rt::flt(l_fS_12, F32(0x00000000)) as i32)) != 0) {
                if (((rt::flt(l_fT_13, F32(0x00000000)) as i32)) != 0) {
                    if (((rt::flt(l_fB0_8, F32(0x00000000)) as i32)) != 0) {
                        l_fT_13 = F32(0x00000000);
                        if (((rt::fge(rt::fneg(l_fB0_8), l_fA00_5) as i32)) != 0) {
                            l_fS_12 = F32(0x3f800000);
                            l_fSqrDist_14 = rt::fadd(rt::fadd(l_fA00_5, rt::fmul(F32(0x40000000), l_fB0_8)), l_fC_10);
                        } else {
                            l_fS_12 = rt::fdiv(rt::fneg(l_fB0_8), l_fA00_5);
                            l_fSqrDist_14 = rt::fadd(rt::fmul(l_fB0_8, l_fS_12), l_fC_10);
                        }
                    } else {
                        l_fS_12 = F32(0x00000000);
                        if (((rt::fge(l_fB1_9, F32(0x00000000)) as i32)) != 0) {
                            l_fT_13 = F32(0x00000000);
                            l_fSqrDist_14 = l_fC_10;
                        } else {
                            if (((rt::fge(rt::fneg(l_fB1_9), l_fA11_7) as i32)) != 0) {
                                l_fT_13 = F32(0x3f800000);
                                l_fSqrDist_14 = rt::fadd(rt::fadd(l_fA11_7, rt::fmul(F32(0x40000000), l_fB1_9)), l_fC_10);
                            } else {
                                l_fT_13 = rt::fdiv(rt::fneg(l_fB1_9), l_fA11_7);
                                l_fSqrDist_14 = rt::fadd(rt::fmul(l_fB1_9, l_fT_13), l_fC_10);
                            }
                        }
                    }
                } else {
                    l_fS_12 = F32(0x00000000);
                    if (((rt::fge(l_fB1_9, F32(0x00000000)) as i32)) != 0) {
                        l_fT_13 = F32(0x00000000);
                        l_fSqrDist_14 = l_fC_10;
                    } else {
                        if (((rt::fge(rt::fneg(l_fB1_9), l_fA11_7) as i32)) != 0) {
                            l_fT_13 = F32(0x3f800000);
                            l_fSqrDist_14 = rt::fadd(rt::fadd(l_fA11_7, rt::fmul(F32(0x40000000), l_fB1_9)), l_fC_10);
                        } else {
                            l_fT_13 = rt::fdiv(rt::fneg(l_fB1_9), l_fA11_7);
                            l_fSqrDist_14 = rt::fadd(rt::fmul(l_fB1_9, l_fT_13), l_fC_10);
                        }
                    }
                }
            } else {
                if (((rt::flt(l_fT_13, F32(0x00000000)) as i32)) != 0) {
                    l_fT_13 = F32(0x00000000);
                    if (((rt::fge(l_fB0_8, F32(0x00000000)) as i32)) != 0) {
                        l_fS_12 = F32(0x00000000);
                        l_fSqrDist_14 = l_fC_10;
                    } else {
                        if (((rt::fge(rt::fneg(l_fB0_8), l_fA00_5) as i32)) != 0) {
                            l_fS_12 = F32(0x3f800000);
                            l_fSqrDist_14 = rt::fadd(rt::fadd(l_fA00_5, rt::fmul(F32(0x40000000), l_fB0_8)), l_fC_10);
                        } else {
                            l_fS_12 = rt::fdiv(rt::fneg(l_fB0_8), l_fA00_5);
                            l_fSqrDist_14 = rt::fadd(rt::fmul(l_fB0_8, l_fS_12), l_fC_10);
                        }
                    }
                } else {
                    l_fInvDet_15 = rt::fdiv(F32(0x3f800000), l_fDet_11);
                    { let r_ = l_fInvDet_15; let n_ = rt::fmul(l_fS_12, r_); l_fS_12 = n_; };
                    { let r_ = l_fInvDet_15; let n_ = rt::fmul(l_fT_13, r_); l_fT_13 = n_; };
                    l_fSqrDist_14 = rt::fadd(rt::fadd(rt::fmul(l_fS_12, rt::fadd(rt::fadd(rt::fmul(l_fA00_5, l_fS_12), rt::fmul(l_fA01_6, l_fT_13)), rt::fmul(F32(0x40000000), l_fB0_8))), rt::fmul(l_fT_13, rt::fadd(rt::fadd(rt::fmul(l_fA01_6, l_fS_12), rt::fmul(l_fA11_7, l_fT_13)), rt::fmul(F32(0x40000000), l_fB1_9)))), l_fC_10);
                }
            }
        } else {
            if (((rt::flt(l_fS_12, F32(0x00000000)) as i32)) != 0) {
                l_fTmp0_16 = rt::fadd(l_fA01_6, l_fB0_8);
                l_fTmp1_17 = rt::fadd(l_fA11_7, l_fB1_9);
                if (((rt::fgt(l_fTmp1_17, l_fTmp0_16) as i32)) != 0) {
                    l_fNumer_18 = rt::fsub(l_fTmp1_17, l_fTmp0_16);
                    l_fDenom_19 = rt::fadd(rt::fsub(l_fA00_5, rt::fmul(F32(0x40000000), l_fA01_6)), l_fA11_7);
                    if (((rt::fge(l_fNumer_18, l_fDenom_19) as i32)) != 0) {
                        l_fS_12 = F32(0x3f800000);
                        l_fT_13 = F32(0x00000000);
                        l_fSqrDist_14 = rt::fadd(rt::fadd(l_fA00_5, rt::fmul(F32(0x40000000), l_fB0_8)), l_fC_10);
                    } else {
                        l_fS_12 = rt::fdiv(l_fNumer_18, l_fDenom_19);
                        l_fT_13 = rt::fsub(F32(0x3f800000), l_fS_12);
                        l_fSqrDist_14 = rt::fadd(rt::fadd(rt::fmul(l_fS_12, rt::fadd(rt::fadd(rt::fmul(l_fA00_5, l_fS_12), rt::fmul(l_fA01_6, l_fT_13)), rt::fmul(F32(0x40000000), l_fB0_8))), rt::fmul(l_fT_13, rt::fadd(rt::fadd(rt::fmul(l_fA01_6, l_fS_12), rt::fmul(l_fA11_7, l_fT_13)), rt::fmul(F32(0x40000000), l_fB1_9)))), l_fC_10);
                    }
                } else {
                    l_fS_12 = F32(0x00000000);
                    if (((rt::fle(l_fTmp1_17, F32(0x00000000)) as i32)) != 0) {
                        l_fT_13 = F32(0x3f800000);
                        l_fSqrDist_14 = rt::fadd(rt::fadd(l_fA11_7, rt::fmul(F32(0x40000000), l_fB1_9)), l_fC_10);
                    } else {
                        if (((rt::fge(l_fB1_9, F32(0x00000000)) as i32)) != 0) {
                            l_fT_13 = F32(0x00000000);
                            l_fSqrDist_14 = l_fC_10;
                        } else {
                            l_fT_13 = rt::fdiv(rt::fneg(l_fB1_9), l_fA11_7);
                            l_fSqrDist_14 = rt::fadd(rt::fmul(l_fB1_9, l_fT_13), l_fC_10);
                        }
                    }
                }
            } else {
                if (((rt::flt(l_fT_13, F32(0x00000000)) as i32)) != 0) {
                    l_fTmp0_16 = rt::fadd(l_fA01_6, l_fB1_9);
                    l_fTmp1_17 = rt::fadd(l_fA00_5, l_fB0_8);
                    if (((rt::fgt(l_fTmp1_17, l_fTmp0_16) as i32)) != 0) {
                        l_fNumer_18 = rt::fsub(l_fTmp1_17, l_fTmp0_16);
                        l_fDenom_19 = rt::fadd(rt::fsub(l_fA00_5, rt::fmul(F32(0x40000000), l_fA01_6)), l_fA11_7);
                        if (((rt::fge(l_fNumer_18, l_fDenom_19) as i32)) != 0) {
                            l_fT_13 = F32(0x3f800000);
                            l_fS_12 = F32(0x00000000);
                            l_fSqrDist_14 = rt::fadd(rt::fadd(l_fA11_7, rt::fmul(F32(0x40000000), l_fB1_9)), l_fC_10);
                        } else {
                            l_fT_13 = rt::fdiv(l_fNumer_18, l_fDenom_19);
                            l_fS_12 = rt::fsub(F32(0x3f800000), l_fT_13);
                            l_fSqrDist_14 = rt::fadd(rt::fadd(rt::fmul(l_fS_12, rt::fadd(rt::fadd(rt::fmul(l_fA00_5, l_fS_12), rt::fmul(l_fA01_6, l_fT_13)), rt::fmul(F32(0x40000000), l_fB0_8))), rt::fmul(l_fT_13, rt::fadd(rt::fadd(rt::fmul(l_fA01_6, l_fS_12), rt::fmul(l_fA11_7, l_fT_13)), rt::fmul(F32(0x40000000), l_fB1_9)))), l_fC_10);
                        }
                    } else {
                        l_fT_13 = F32(0x00000000);
                        if (((rt::fle(l_fTmp1_17, F32(0x00000000)) as i32)) != 0) {
                            l_fS_12 = F32(0x3f800000);
                            l_fSqrDist_14 = rt::fadd(rt::fadd(l_fA00_5, rt::fmul(F32(0x40000000), l_fB0_8)), l_fC_10);
                        } else {
                            if (((rt::fge(l_fB0_8, F32(0x00000000)) as i32)) != 0) {
                                l_fS_12 = F32(0x00000000);
                                l_fSqrDist_14 = l_fC_10;
                            } else {
                                l_fS_12 = rt::fdiv(rt::fneg(l_fB0_8), l_fA00_5);
                                l_fSqrDist_14 = rt::fadd(rt::fmul(l_fB0_8, l_fS_12), l_fC_10);
                            }
                        }
                    }
                } else {
                    l_fNumer_18 = rt::fsub(rt::fsub(rt::fadd(l_fA11_7, l_fB1_9), l_fA01_6), l_fB0_8);
                    if (((rt::fle(l_fNumer_18, F32(0x00000000)) as i32)) != 0) {
                        l_fS_12 = F32(0x00000000);
                        l_fT_13 = F32(0x3f800000);
                        l_fSqrDist_14 = rt::fadd(rt::fadd(l_fA11_7, rt::fmul(F32(0x40000000), l_fB1_9)), l_fC_10);
                    } else {
                        l_fDenom_19 = rt::fadd(rt::fsub(l_fA00_5, rt::fmul(F32(0x40000000), l_fA01_6)), l_fA11_7);
                        if (((rt::fge(l_fNumer_18, l_fDenom_19) as i32)) != 0) {
                            l_fS_12 = F32(0x3f800000);
                            l_fT_13 = F32(0x00000000);
                            l_fSqrDist_14 = rt::fadd(rt::fadd(l_fA00_5, rt::fmul(F32(0x40000000), l_fB0_8)), l_fC_10);
                        } else {
                            l_fS_12 = rt::fdiv(l_fNumer_18, l_fDenom_19);
                            l_fT_13 = rt::fsub(F32(0x3f800000), l_fS_12);
                            l_fSqrDist_14 = rt::fadd(rt::fadd(rt::fmul(l_fS_12, rt::fadd(rt::fadd(rt::fmul(l_fA00_5, l_fS_12), rt::fmul(l_fA01_6, l_fT_13)), rt::fmul(F32(0x40000000), l_fB0_8))), rt::fmul(l_fT_13, rt::fadd(rt::fadd(rt::fmul(l_fA01_6, l_fS_12), rt::fmul(l_fA11_7, l_fT_13)), rt::fmul(F32(0x40000000), l_fB1_9)))), l_fC_10);
                        }
                    }
                }
            }
        }
        if (((((p_s_3) != (((0i32) as u32))) as i32)) != 0) {
            { let v_ = l_fS_12; let a_ = p_s_3; c.m.w32(a_, (v_).0); };
        }
        if (((((p_t_4) != (((0i32) as u32))) as i32)) != 0) {
            { let v_ = l_fT_13; let a_ = p_t_4; c.m.w32(a_, (v_).0); };
        }
        break 'f rt::ffabs(l_fSqrDist_14);
       F32(0)
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "ColTri_DistSqPoint", &PROBE_N, &[], true); }
    r_
}

pub fn ColTri_GetPoint(c: &mut Ctx, mut p_out_1: u32, mut p_tri_2: u32, mut p_s_3: F32, mut p_t_4: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ColTri_GetPoint", &PROBE_N, &[(p_out_1) as u32, (p_tri_2) as u32, (p_s_3).0, (p_t_4).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let r_ = 'f: {
        F::Vec4_Sub(c, fp, (p_tri_2).wrapping_add((((1i32) as u32)).wrapping_mul(16)), (p_tri_2).wrapping_add((((0i32) as u32)).wrapping_mul(16)));
        F::Vec4_Scale(c, fp, fp, p_s_3);
        F::Vec4_Add(c, fp, fp, (p_tri_2).wrapping_add((((0i32) as u32)).wrapping_mul(16)));
        F::Vec4_Sub(c, fp.wrapping_add(16), (p_tri_2).wrapping_add((((2i32) as u32)).wrapping_mul(16)), (p_tri_2).wrapping_add((((0i32) as u32)).wrapping_mul(16)));
        F::Vec4_Scale(c, fp.wrapping_add(16), fp.wrapping_add(16), p_t_4);
        F::Vec4_Add(c, p_out_1, fp, fp.wrapping_add(16));
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "ColTri_GetPoint", &PROBE_N, &[], true); }
    r_
}

pub fn ColSphere_TestTri(c: &mut Ctx, mut p_pos_1: u32, mut p_sph_2: u32, mut p_tri_3: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ColSphere_TestTri", &PROBE_N, &[(p_pos_1) as u32, (p_sph_2) as u32, (p_tri_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        if (((rt::fgt(F::ColTri_DistSqPoint(c, p_sph_2, p_tri_3, (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)), (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4))), F32(c.m.r32((p_sph_2).wrapping_add(20)))) as i32)) != 0) {
            break 'f 0i32;
        }
        { let a2_ = F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a3_ = F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); F::ColTri_GetPoint(c, p_pos_1, p_tri_3, a2_, a3_) };
        break 'f 1i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "ColSphere_TestTri", &PROBE_N, &[], true); }
    r_
}

pub fn ColBounds_OfCapsule(c: &mut Ctx, mut p_box_1: u32, mut p_cap_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ColBounds_OfCapsule", &PROBE_N, &[(p_box_1) as u32, (p_cap_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_i_3: i32 = 0;
    let r_ = 'f: {
        F::StgAabb_SetEmpty(c, p_box_1);
        {
            l_i_3 = 0i32;
            'l1: loop {
                if !(((((l_i_3) < (3i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    F::Vec4_Copy(c, fp, p_cap_2);
                    { let r_ = F32(c.m.r32((p_cap_2).wrapping_add(32))); let a_ = (fp).wrapping_add((((l_i_3) as u32)).wrapping_mul(4)); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    F::ColBox_AddPoint(c, p_box_1, fp);
                    F::Vec4_Copy(c, fp, p_cap_2);
                    { let r_ = F32(c.m.r32((p_cap_2).wrapping_add(32))); let a_ = (fp).wrapping_add((((l_i_3) as u32)).wrapping_mul(4)); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    F::ColBox_AddPoint(c, p_box_1, fp);
                    F::Vec4_Copy(c, fp, (p_cap_2).wrapping_add(16));
                    { let r_ = F32(c.m.r32((p_cap_2).wrapping_add(32))); let a_ = (fp).wrapping_add((((l_i_3) as u32)).wrapping_mul(4)); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    F::ColBox_AddPoint(c, p_box_1, fp);
                    F::Vec4_Copy(c, fp, (p_cap_2).wrapping_add(16));
                    { let r_ = F32(c.m.r32((p_cap_2).wrapping_add(32))); let a_ = (fp).wrapping_add((((l_i_3) as u32)).wrapping_mul(4)); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    F::ColBox_AddPoint(c, p_box_1, fp);
                }
                { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "ColBounds_OfCapsule", &PROBE_N, &[], true); }
    r_
}

pub fn ColBounds_OfSphere(c: &mut Ctx, mut p_box_1: u32, mut p_sph_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ColBounds_OfSphere", &PROBE_N, &[(p_box_1) as u32, (p_sph_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        F::StgAabb_SetEmpty(c, p_box_1);
        F::Vec4_Copy(c, fp, p_sph_2);
        { let r_ = F32(c.m.r32((p_sph_2).wrapping_add(16))); let a_ = fp; let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        { let r_ = F32(c.m.r32((p_sph_2).wrapping_add(16))); let a_ = (fp).wrapping_add(4); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        { let r_ = F32(c.m.r32((p_sph_2).wrapping_add(16))); let a_ = (fp).wrapping_add(8); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        F::ColBox_AddPoint(c, p_box_1, fp);
        F::Vec4_Copy(c, fp, p_sph_2);
        { let r_ = F32(c.m.r32((p_sph_2).wrapping_add(16))); let a_ = fp; let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        { let r_ = F32(c.m.r32((p_sph_2).wrapping_add(16))); let a_ = (fp).wrapping_add(4); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        { let r_ = F32(c.m.r32((p_sph_2).wrapping_add(16))); let a_ = (fp).wrapping_add(8); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        F::ColBox_AddPoint(c, p_box_1, fp);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "ColBounds_OfSphere", &PROBE_N, &[], true); }
    r_
}

pub fn ColRay_FromSeg(c: &mut Ctx, mut p_ray_1: u32, mut p_seg_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ColRay_FromSeg", &PROBE_N, &[(p_ray_1) as u32, (p_seg_2) as u32], false); }
    let r_ = 'f: {
    F::Vec4_Copy(c, p_ray_1, p_seg_2);
    F::Vec4_Sub(c, (p_ray_1).wrapping_add(16), (p_seg_2).wrapping_add(16), p_seg_2);
    F::Vec3_Normalize(c, (p_ray_1).wrapping_add(16), (p_ray_1).wrapping_add(16));
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ColRay_FromSeg", &PROBE_N, &[], true); }
    r_
}

pub fn ColBounds_AddSphere(c: &mut Ctx, mut p_box_1: u32, mut p_sph_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ColBounds_AddSphere", &PROBE_N, &[(p_box_1) as u32, (p_sph_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        F::Vec4_Copy(c, fp, p_sph_2);
        { let r_ = F32(c.m.r32((p_sph_2).wrapping_add(16))); let a_ = fp; let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        { let r_ = F32(c.m.r32((p_sph_2).wrapping_add(16))); let a_ = (fp).wrapping_add(4); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        { let r_ = F32(c.m.r32((p_sph_2).wrapping_add(16))); let a_ = (fp).wrapping_add(8); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        F::ColBox_AddPoint(c, p_box_1, fp);
        F::Vec4_Copy(c, fp, p_sph_2);
        { let r_ = F32(c.m.r32((p_sph_2).wrapping_add(16))); let a_ = fp; let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        { let r_ = F32(c.m.r32((p_sph_2).wrapping_add(16))); let a_ = (fp).wrapping_add(4); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        { let r_ = F32(c.m.r32((p_sph_2).wrapping_add(16))); let a_ = (fp).wrapping_add(8); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        F::ColBox_AddPoint(c, p_box_1, fp);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "ColBounds_AddSphere", &PROBE_N, &[], true); }
    r_
}

pub fn ColBounds_OfObb(c: &mut Ctx, mut p_box_1: u32, mut p_obb_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ColBounds_OfObb", &PROBE_N, &[(p_box_1) as u32, (p_obb_2) as u32], false); }
    let mut l_i_3: i32 = 0;
    let r_ = 'f: {
    F::StgAabb_SetEmpty(c, p_box_1);
    {
        l_i_3 = 0i32;
        'l1: loop {
            if !(((((l_i_3) < (8i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                F::ColBox_AddPoint(c, p_box_1, ((p_obb_2).wrapping_add(272)).wrapping_add((((l_i_3) as u32)).wrapping_mul(16)));
            }
            { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ColBounds_OfObb", &PROBE_N, &[], true); }
    r_
}

pub fn ColBounds_OfSeg(c: &mut Ctx, mut p_box_1: u32, mut p_seg_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ColBounds_OfSeg", &PROBE_N, &[(p_box_1) as u32, (p_seg_2) as u32], false); }
    let r_ = 'f: {
    F::StgAabb_SetEmpty(c, p_box_1);
    F::ColBox_AddPoint(c, p_box_1, p_seg_2);
    F::ColBox_AddPoint(c, p_box_1, (p_seg_2).wrapping_add(16));
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ColBounds_OfSeg", &PROBE_N, &[], true); }
    r_
}

pub fn ColCapsule_GetLongSeg(c: &mut Ctx, mut p_out_1: u32, mut p_cap_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ColCapsule_GetLongSeg", &PROBE_N, &[(p_out_1) as u32, (p_cap_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        F::Vec4_Sub(c, fp, (p_cap_2).wrapping_add(16), p_cap_2);
        F::Vec3_Normalize(c, fp, fp);
        F::Vec4_Copy(c, p_out_1, p_cap_2);
        F::Vec4_Copy(c, (p_out_1).wrapping_add(16), (p_cap_2).wrapping_add(16));
        { let r_ = rt::fmul(F32(c.m.r32(fp)), F32(c.m.r32((p_cap_2).wrapping_add(32)))); let a_ = (p_out_1).wrapping_add(16); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        { let r_ = rt::fmul(F32(c.m.r32((fp).wrapping_add(4))), F32(c.m.r32((p_cap_2).wrapping_add(32)))); let a_ = ((p_out_1).wrapping_add(16)).wrapping_add(4); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        { let r_ = rt::fmul(F32(c.m.r32((fp).wrapping_add(8))), F32(c.m.r32((p_cap_2).wrapping_add(32)))); let a_ = ((p_out_1).wrapping_add(16)).wrapping_add(8); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        { let r_ = rt::fmul(F32(c.m.r32(fp)), F32(c.m.r32((p_cap_2).wrapping_add(32)))); let a_ = p_out_1; let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        { let r_ = rt::fmul(F32(c.m.r32((fp).wrapping_add(4))), F32(c.m.r32((p_cap_2).wrapping_add(32)))); let a_ = (p_out_1).wrapping_add(4); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        { let r_ = rt::fmul(F32(c.m.r32((fp).wrapping_add(8))), F32(c.m.r32((p_cap_2).wrapping_add(32)))); let a_ = (p_out_1).wrapping_add(8); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "ColCapsule_GetLongSeg", &PROBE_N, &[], true); }
    r_
}

pub fn ColCapsule_GetLongSegDir(c: &mut Ctx, mut p_out_1: u32, mut p_dir_2: u32, mut p_cap_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ColCapsule_GetLongSegDir", &PROBE_N, &[(p_out_1) as u32, (p_dir_2) as u32, (p_cap_3) as u32], false); }
    let r_ = 'f: {
    F::Vec4_Sub(c, p_dir_2, (p_cap_3).wrapping_add(16), p_cap_3);
    F::Vec3_Normalize(c, p_dir_2, p_dir_2);
    F::Vec4_Copy(c, p_out_1, p_cap_3);
    F::Vec4_Copy(c, (p_out_1).wrapping_add(16), (p_cap_3).wrapping_add(16));
    { let r_ = rt::fmul(F32(c.m.r32(p_dir_2)), F32(c.m.r32((p_cap_3).wrapping_add(32)))); let a_ = (p_out_1).wrapping_add(16); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
    { let r_ = rt::fmul(F32(c.m.r32((p_dir_2).wrapping_add(4))), F32(c.m.r32((p_cap_3).wrapping_add(32)))); let a_ = ((p_out_1).wrapping_add(16)).wrapping_add(4); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
    { let r_ = rt::fmul(F32(c.m.r32((p_dir_2).wrapping_add(8))), F32(c.m.r32((p_cap_3).wrapping_add(32)))); let a_ = ((p_out_1).wrapping_add(16)).wrapping_add(8); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
    { let r_ = rt::fmul(F32(c.m.r32(p_dir_2)), F32(c.m.r32((p_cap_3).wrapping_add(32)))); let a_ = p_out_1; let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
    { let r_ = rt::fmul(F32(c.m.r32((p_dir_2).wrapping_add(4))), F32(c.m.r32((p_cap_3).wrapping_add(32)))); let a_ = (p_out_1).wrapping_add(4); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
    { let r_ = rt::fmul(F32(c.m.r32((p_dir_2).wrapping_add(8))), F32(c.m.r32((p_cap_3).wrapping_add(32)))); let a_ = (p_out_1).wrapping_add(8); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
    { let v_ = F32(0x3f800000); let a_ = (p_dir_2).wrapping_add(12); c.m.w32(a_, (v_).0); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ColCapsule_GetLongSegDir", &PROBE_N, &[], true); }
    r_
}

pub fn ColCapsule_GetBoundSphere(c: &mut Ctx, mut p_out_1: u32, mut p_cap_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ColCapsule_GetBoundSphere", &PROBE_N, &[(p_out_1) as u32, (p_cap_2) as u32], false); }
    let mut l_r_3: F32 = F32(0);
    let r_ = 'f: {
    F::ColSeg_GetMidpoint3(c, p_out_1, p_cap_2);
    l_r_3 = rt::fadd(rt::fmul(F::Vec3_Dist(c, p_cap_2, (p_cap_2).wrapping_add(16)), F32(0x3f000000)), F32(c.m.r32((p_cap_2).wrapping_add(32))));
    { let v_ = l_r_3; let a_ = (p_out_1).wrapping_add(16); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fmul(l_r_3, l_r_3); let a_ = (p_out_1).wrapping_add(20); c.m.w32(a_, (v_).0); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ColCapsule_GetBoundSphere", &PROBE_N, &[], true); }
    r_
}

pub fn ColSeg_ClosestToSeg(c: &mut Ctx, mut p_a_1: u32, mut p_b_2: u32, mut p_onA_3: u32, mut p_onB_4: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ColSeg_ClosestToSeg", &PROBE_N, &[(p_a_1) as u32, (p_b_2) as u32, (p_onA_3) as u32, (p_onB_4) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(64) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 64);
    let r_ = 'f: {
        F::Vec4_Copy(c, fp, p_a_1);
        F::Vec4_Copy(c, (fp).wrapping_add(16), (p_a_1).wrapping_add(16));
        F::Vec4_Copy(c, fp.wrapping_add(32), p_b_2);
        F::Vec4_Copy(c, (fp.wrapping_add(32)).wrapping_add(16), (p_b_2).wrapping_add(16));
        if ((F::ColSeg_Nearest(c, fp, fp.wrapping_add(32), p_onA_3, p_onB_4)) != 0) {
            { let v_ = F32(0x3f800000); let a_ = (p_onA_3).wrapping_add(12); c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x3f800000); let a_ = (p_onB_4).wrapping_add(12); c.m.w32(a_, (v_).0); };
            break 'f 1i32;
        }
        break 'f 0i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "ColSeg_ClosestToSeg", &PROBE_N, &[], true); }
    r_
}

pub fn ColLine_DistSqPoint(c: &mut Ctx, mut p_pt_1: u32, mut p_a_2: u32, mut p_b_3: u32, mut p_out_4: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ColLine_DistSqPoint", &PROBE_N, &[(p_pt_1) as u32, (p_a_2) as u32, (p_b_3) as u32, (p_out_4) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let mut l_len_5: F32 = F32(0);
    let r_ = 'f: {
        F::Vec4_Sub(c, fp, p_pt_1, p_a_2);
        F::Vec4_Sub(c, fp.wrapping_add(16), p_b_3, p_a_2);
        l_len_5 = F::Vec3_Dot(c, fp.wrapping_add(16), fp.wrapping_add(16));
        if (((rt::feq(l_len_5, F32(0x00000000)) as i32)) != 0) {
            l_len_5 = F32(0x38d1b717);
        }
        { let a2_ = rt::fdiv(F::Vec3_Dot(c, fp, fp.wrapping_add(16)), l_len_5); F::Vec3_Scale(c, fp.wrapping_add(32), fp.wrapping_add(16), a2_) };
        F::Vec4_Add(c, p_out_4, p_a_2, fp.wrapping_add(32));
        F::Vec4_Sub(c, fp, fp, fp.wrapping_add(32));
        break 'f F::Vec3_Dot(c, fp, fp);
       F32(0)
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "ColLine_DistSqPoint", &PROBE_N, &[], true); }
    r_
}

pub fn ColLine_DistPoint(c: &mut Ctx, mut p_pt_1: u32, mut p_a_2: u32, mut p_b_3: u32, mut p_out_4: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ColLine_DistPoint", &PROBE_N, &[(p_pt_1) as u32, (p_a_2) as u32, (p_b_3) as u32, (p_out_4) as u32], false); }
    let r_ = 'f: {
    break 'f { let a0_ = F::ColLine_DistSqPoint(c, p_pt_1, p_a_2, p_b_3, p_out_4); F::Port_sqrtf(c, a0_) };
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "ColLine_DistPoint", &PROBE_N, &[], true); }
    r_
}

pub fn ColSeg_NearestOnLine(c: &mut Ctx, mut p_a_1: u32, mut p_l_2: u32, mut p_b_3: u32, mut p_infinite_4: i32, mut p_epsSq_5: F32, mut p_out_6: u32, mut p_param_7: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ColSeg_NearestOnLine", &PROBE_N, &[(p_a_1) as u32, (p_l_2) as u32, (p_b_3) as u32, (p_infinite_4) as u32, (p_epsSq_5).0, (p_out_6) as u32, (p_param_7) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_d_8: F32 = F32(0);
    let r_ = 'f: {
        l_d_8 = F::Vec3_Dot(c, p_l_2, p_l_2);
        if (((rt::flt(l_d_8, p_epsSq_5) as i32)) != 0) {
            F::Vec4_Copy(c, p_out_6, p_a_1);
            break 'f;
        }
        F::Vec4_Sub(c, fp, p_b_3, p_a_1);
        { let v_ = rt::fdiv(F::Vec3_Dot(c, p_l_2, fp), l_d_8); let a_ = p_param_7; c.m.w32(a_, (v_).0); };
        if ((((!((p_infinite_4) != 0)) as i32)) != 0) {
            { let v_ = (if (((rt::fgt(F32(0x00000000), (if (((rt::fgt(F32(0x3f800000), F32(c.m.r32(p_param_7))) as i32)) != 0) { F32(c.m.r32(p_param_7)) } else { F32(0x3f800000) })) as i32)) != 0) { F32(0x00000000) } else { (if (((rt::fgt(F32(0x3f800000), F32(c.m.r32(p_param_7))) as i32)) != 0) { F32(c.m.r32(p_param_7)) } else { F32(0x3f800000) }) }); let a_ = p_param_7; c.m.w32(a_, (v_).0); };
        }
        { let a2_ = F32(c.m.r32(p_param_7)); F::Vec4_Scale(c, p_out_6, p_l_2, a2_) };
        F::Vec4_Add(c, p_out_6, p_out_6, p_a_1);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "ColSeg_NearestOnLine", &PROBE_N, &[], true); }
    r_
}

pub fn ColSeg_NearestParallel(c: &mut Ctx, mut p_a_1: u32, mut p_la_2: u32, mut p_b_3: u32, mut p_lb_4: u32, mut p_infinite_5: i32, mut p_epsSq_6: F32, mut p_onA_7: u32, mut p_onB_8: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ColSeg_NearestParallel", &PROBE_N, &[(p_a_1) as u32, (p_la_2) as u32, (p_b_3) as u32, (p_lb_4) as u32, (p_infinite_5) as u32, (p_epsSq_6).0, (p_onA_7) as u32, (p_onB_8) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let r_ = 'f: {
        F::ColSeg_NearestOnLine(c, p_a_1, p_la_2, p_b_3, 1i32, p_epsSq_6, p_onA_7, (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)));
        if (((((p_infinite_5) == (1i32)) as i32)) != 0) {
            F::Vec4_Copy(c, p_onB_8, p_b_3);
        } else {
            F::ColSeg_NearestOnLine(c, p_a_1, p_la_2, (p_b_3).wrapping_add(16), 1i32, p_epsSq_6, fp.wrapping_add(16), (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)));
            if (((((((rt::flt(F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(0x00000000)) as i32)) != 0) && (((rt::flt(F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(0x00000000)) as i32)) != 0)) as i32)) != 0) {
                F::Vec4_Copy(c, p_onA_7, p_a_1);
                if (((rt::flt(F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4))))) as i32)) != 0) {
                    F::Vec4_Copy(c, p_onB_8, (p_b_3).wrapping_add(16));
                } else {
                    F::Vec4_Copy(c, p_onB_8, p_b_3);
                }
            } else {
                if (((((((rt::fgt(F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(0x3f800000)) as i32)) != 0) && (((rt::fgt(F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(0x3f800000)) as i32)) != 0)) as i32)) != 0) {
                    F::Vec4_Copy(c, p_onA_7, (p_a_1).wrapping_add(16));
                    if (((rt::flt(F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4))))) as i32)) != 0) {
                        F::Vec4_Copy(c, p_onB_8, p_b_3);
                    } else {
                        F::Vec4_Copy(c, p_onB_8, (p_b_3).wrapping_add(16));
                    }
                } else {
                    { let v_ = rt::fmul(F32(0x3f000000), rt::fadd((if (((rt::fgt(F32(0x00000000), (if (((rt::fgt(F32(0x3f800000), F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4))))) as i32)) != 0) { F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)))) } else { F32(0x3f800000) })) as i32)) != 0) { F32(0x00000000) } else { (if (((rt::fgt(F32(0x3f800000), F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4))))) as i32)) != 0) { F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)))) } else { F32(0x3f800000) }) }), (if (((rt::fgt(F32(0x00000000), (if (((rt::fgt(F32(0x3f800000), F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4))))) as i32)) != 0) { F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)))) } else { F32(0x3f800000) })) as i32)) != 0) { F32(0x00000000) } else { (if (((rt::fgt(F32(0x3f800000), F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4))))) as i32)) != 0) { F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)))) } else { F32(0x3f800000) }) }))); let a_ = fp.wrapping_add(32); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fadd(F32(c.m.r32(p_a_1)), rt::fmul(F32(c.m.r32(fp.wrapping_add(32))), F32(c.m.r32(p_la_2)))); let a_ = p_onA_7; c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fadd(F32(c.m.r32((p_a_1).wrapping_add(4))), rt::fmul(F32(c.m.r32(fp.wrapping_add(32))), F32(c.m.r32((p_la_2).wrapping_add(4))))); let a_ = (p_onA_7).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fadd(F32(c.m.r32((p_a_1).wrapping_add(8))), rt::fmul(F32(c.m.r32(fp.wrapping_add(32))), F32(c.m.r32((p_la_2).wrapping_add(8))))); let a_ = (p_onA_7).wrapping_add(8); c.m.w32(a_, (v_).0); };
                    F::ColSeg_NearestOnLine(c, p_b_3, p_lb_4, p_onA_7, 1i32, p_epsSq_6, p_onB_8, fp.wrapping_add(32));
                }
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "ColSeg_NearestParallel", &PROBE_N, &[], true); }
    r_
}

pub fn ColSeg_AdjustNearest(c: &mut Ctx, mut p_a_1: u32, mut p_la_2: u32, mut p_b_3: u32, mut p_lb_4: u32, mut p_epsSq_5: F32, p_s_6: F32, p_t_7: F32, mut p_onA_8: u32, mut p_onB_9: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ColSeg_AdjustNearest", &PROBE_N, &[(p_a_1) as u32, (p_la_2) as u32, (p_b_3) as u32, (p_lb_4) as u32, (p_epsSq_5).0, (p_s_6).0, (p_t_7).0, (p_onA_8) as u32, (p_onB_9) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    c.m.w32(fp, (p_s_6).0);
    c.m.w32(fp.wrapping_add(4), (p_t_7).0);
    let r_ = 'f: {
        if (((((((((((rt::flt(F32(c.m.r32(fp)), F32(0x00000000)) as i32)) != 0) || (((rt::fgt(F32(c.m.r32(fp)), F32(0x3f800000)) as i32)) != 0)) as i32)) != 0) && (((((((rt::flt(F32(c.m.r32(fp.wrapping_add(4))), F32(0x00000000)) as i32)) != 0) || (((rt::fgt(F32(c.m.r32(fp.wrapping_add(4))), F32(0x3f800000)) as i32)) != 0)) as i32)) != 0)) as i32)) != 0) {
            { let v_ = (if (((rt::fgt(F32(0x00000000), (if (((rt::fgt(F32(0x3f800000), F32(c.m.r32(fp))) as i32)) != 0) { F32(c.m.r32(fp)) } else { F32(0x3f800000) })) as i32)) != 0) { F32(0x00000000) } else { (if (((rt::fgt(F32(0x3f800000), F32(c.m.r32(fp))) as i32)) != 0) { F32(c.m.r32(fp)) } else { F32(0x3f800000) }) }); let a_ = fp; c.m.w32(a_, (v_).0); };
            { let v_ = rt::fadd(F32(c.m.r32(p_a_1)), rt::fmul(F32(c.m.r32(fp)), F32(c.m.r32(p_la_2)))); let a_ = p_onA_8; c.m.w32(a_, (v_).0); };
            { let v_ = rt::fadd(F32(c.m.r32((p_a_1).wrapping_add(4))), rt::fmul(F32(c.m.r32(fp)), F32(c.m.r32((p_la_2).wrapping_add(4))))); let a_ = (p_onA_8).wrapping_add(4); c.m.w32(a_, (v_).0); };
            { let v_ = rt::fadd(F32(c.m.r32((p_a_1).wrapping_add(8))), rt::fmul(F32(c.m.r32(fp)), F32(c.m.r32((p_la_2).wrapping_add(8))))); let a_ = (p_onA_8).wrapping_add(8); c.m.w32(a_, (v_).0); };
            F::ColSeg_NearestOnLine(c, p_b_3, p_lb_4, p_onA_8, 1i32, p_epsSq_5, p_onB_9, fp.wrapping_add(4));
            if (((((((rt::flt(F32(c.m.r32(fp.wrapping_add(4))), F32(0x00000000)) as i32)) != 0) || (((rt::fgt(F32(c.m.r32(fp.wrapping_add(4))), F32(0x3f800000)) as i32)) != 0)) as i32)) != 0) {
                { let v_ = (if (((rt::fgt(F32(0x00000000), (if (((rt::fgt(F32(0x3f800000), F32(c.m.r32(fp.wrapping_add(4)))) as i32)) != 0) { F32(c.m.r32(fp.wrapping_add(4))) } else { F32(0x3f800000) })) as i32)) != 0) { F32(0x00000000) } else { (if (((rt::fgt(F32(0x3f800000), F32(c.m.r32(fp.wrapping_add(4)))) as i32)) != 0) { F32(c.m.r32(fp.wrapping_add(4))) } else { F32(0x3f800000) }) }); let a_ = fp.wrapping_add(4); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fadd(F32(c.m.r32(p_b_3)), rt::fmul(F32(c.m.r32(fp.wrapping_add(4))), F32(c.m.r32(p_lb_4)))); let a_ = p_onB_9; c.m.w32(a_, (v_).0); };
                { let v_ = rt::fadd(F32(c.m.r32((p_b_3).wrapping_add(4))), rt::fmul(F32(c.m.r32(fp.wrapping_add(4))), F32(c.m.r32((p_lb_4).wrapping_add(4))))); let a_ = (p_onB_9).wrapping_add(4); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fadd(F32(c.m.r32((p_b_3).wrapping_add(8))), rt::fmul(F32(c.m.r32(fp.wrapping_add(4))), F32(c.m.r32((p_lb_4).wrapping_add(8))))); let a_ = (p_onB_9).wrapping_add(8); c.m.w32(a_, (v_).0); };
                F::ColSeg_NearestOnLine(c, p_a_1, p_la_2, p_onB_9, 0i32, p_epsSq_5, p_onA_8, fp);
                F::ColSeg_NearestOnLine(c, p_b_3, p_lb_4, p_onA_8, 0i32, p_epsSq_5, p_onB_9, fp.wrapping_add(4));
            }
        } else {
            if (((((((rt::flt(F32(c.m.r32(fp)), F32(0x00000000)) as i32)) != 0) || (((rt::fgt(F32(c.m.r32(fp)), F32(0x3f800000)) as i32)) != 0)) as i32)) != 0) {
                { let v_ = (if (((rt::fgt(F32(0x00000000), (if (((rt::fgt(F32(0x3f800000), F32(c.m.r32(fp))) as i32)) != 0) { F32(c.m.r32(fp)) } else { F32(0x3f800000) })) as i32)) != 0) { F32(0x00000000) } else { (if (((rt::fgt(F32(0x3f800000), F32(c.m.r32(fp))) as i32)) != 0) { F32(c.m.r32(fp)) } else { F32(0x3f800000) }) }); let a_ = fp; c.m.w32(a_, (v_).0); };
                { let v_ = rt::fadd(F32(c.m.r32(p_a_1)), rt::fmul(F32(c.m.r32(fp)), F32(c.m.r32(p_la_2)))); let a_ = p_onA_8; c.m.w32(a_, (v_).0); };
                { let v_ = rt::fadd(F32(c.m.r32((p_a_1).wrapping_add(4))), rt::fmul(F32(c.m.r32(fp)), F32(c.m.r32((p_la_2).wrapping_add(4))))); let a_ = (p_onA_8).wrapping_add(4); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fadd(F32(c.m.r32((p_a_1).wrapping_add(8))), rt::fmul(F32(c.m.r32(fp)), F32(c.m.r32((p_la_2).wrapping_add(8))))); let a_ = (p_onA_8).wrapping_add(8); c.m.w32(a_, (v_).0); };
                F::ColSeg_NearestOnLine(c, p_b_3, p_lb_4, p_onA_8, 0i32, p_epsSq_5, p_onB_9, fp.wrapping_add(4));
            } else {
                if (((((((rt::flt(F32(c.m.r32(fp.wrapping_add(4))), F32(0x00000000)) as i32)) != 0) || (((rt::fgt(F32(c.m.r32(fp.wrapping_add(4))), F32(0x3f800000)) as i32)) != 0)) as i32)) != 0) {
                    { let v_ = (if (((rt::fgt(F32(0x00000000), (if (((rt::fgt(F32(0x3f800000), F32(c.m.r32(fp.wrapping_add(4)))) as i32)) != 0) { F32(c.m.r32(fp.wrapping_add(4))) } else { F32(0x3f800000) })) as i32)) != 0) { F32(0x00000000) } else { (if (((rt::fgt(F32(0x3f800000), F32(c.m.r32(fp.wrapping_add(4)))) as i32)) != 0) { F32(c.m.r32(fp.wrapping_add(4))) } else { F32(0x3f800000) }) }); let a_ = fp.wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fadd(F32(c.m.r32(p_b_3)), rt::fmul(F32(c.m.r32(fp.wrapping_add(4))), F32(c.m.r32(p_lb_4)))); let a_ = p_onB_9; c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fadd(F32(c.m.r32((p_b_3).wrapping_add(4))), rt::fmul(F32(c.m.r32(fp.wrapping_add(4))), F32(c.m.r32((p_lb_4).wrapping_add(4))))); let a_ = (p_onB_9).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fadd(F32(c.m.r32((p_b_3).wrapping_add(8))), rt::fmul(F32(c.m.r32(fp.wrapping_add(4))), F32(c.m.r32((p_lb_4).wrapping_add(8))))); let a_ = (p_onB_9).wrapping_add(8); c.m.w32(a_, (v_).0); };
                    F::ColSeg_NearestOnLine(c, p_a_1, p_la_2, p_onB_9, 0i32, p_epsSq_5, p_onA_8, fp);
                }
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "ColSeg_AdjustNearest", &PROBE_N, &[], true); }
    r_
}

pub fn ColSeg_Nearest(c: &mut Ctx, mut p_a_1: u32, mut p_b_2: u32, mut p_onA_3: u32, mut p_onB_4: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ColSeg_Nearest", &PROBE_N, &[(p_a_1) as u32, (p_b_2) as u32, (p_onA_3) as u32, (p_onB_4) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(96) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 96);
    let mut l_eps_5: F32 = F32(0);
    let mut l_epsSq_6: F32 = F32(0);
    let mut l_l11_7: F32 = F32(0);
    let mut l_l22_8: F32 = F32(0);
    let mut l_l12_9: F32 = F32(0);
    let mut l_detL_10: F32 = F32(0);
    let mut l_ra_11: F32 = F32(0);
    let mut l_rb_12: F32 = F32(0);
    let mut l_t_13: F32 = F32(0);
    let mut l_s_14: F32 = F32(0);
    let r_ = 'f: {
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(80), (v_).0); }
        l_eps_5 = F32(0x358637bd);
        l_epsSq_6 = rt::fmul(l_eps_5, l_eps_5);
        F::Vec4_Sub(c, fp, (p_a_1).wrapping_add(16), p_a_1);
        F::Vec4_Sub(c, fp.wrapping_add(16), (p_b_2).wrapping_add(16), p_b_2);
        l_l11_7 = F::Vec3_Dot(c, fp, fp);
        l_l22_8 = F::Vec3_Dot(c, fp.wrapping_add(16), fp.wrapping_add(16));
        if (((rt::flt(l_l11_7, l_epsSq_6) as i32)) != 0) {
            F::Vec4_Copy(c, p_onA_3, p_a_1);
            F::ColSeg_NearestOnLine(c, p_b_2, fp.wrapping_add(16), p_a_1, 0i32, l_eps_5, p_onB_4, fp.wrapping_add(80));
        } else {
            if (((rt::flt(l_l22_8, l_epsSq_6) as i32)) != 0) {
                F::Vec4_Copy(c, p_onB_4, p_b_2);
                F::ColSeg_NearestOnLine(c, p_a_1, fp, p_b_2, 0i32, l_eps_5, p_onA_3, fp.wrapping_add(80));
            } else {
                F::Vec4_Sub(c, fp.wrapping_add(32), p_b_2, p_a_1);
                l_l12_9 = rt::fsub(rt::fsub(rt::fneg(rt::fmul(F32(c.m.r32(fp)), F32(c.m.r32(fp.wrapping_add(16))))), rt::fmul(F32(c.m.r32((fp).wrapping_add(4))), F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))))), rt::fmul(F32(c.m.r32((fp).wrapping_add(8))), F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8)))));
                l_detL_10 = rt::fsub(rt::fmul(l_l11_7, l_l22_8), rt::fmul(l_l12_9, l_l12_9));
                if (((rt::flt(rt::ffabs(l_detL_10), l_eps_5) as i32)) != 0) {
                    F::ColSeg_NearestParallel(c, p_a_1, fp, p_b_2, fp.wrapping_add(16), 0i32, l_eps_5, p_onA_3, p_onB_4);
                } else {
                    l_ra_11 = rt::fadd(rt::fadd(rt::fmul(F32(c.m.r32(fp)), F32(c.m.r32(fp.wrapping_add(32)))), rt::fmul(F32(c.m.r32((fp).wrapping_add(4))), F32(c.m.r32((fp.wrapping_add(32)).wrapping_add(4))))), rt::fmul(F32(c.m.r32((fp).wrapping_add(8))), F32(c.m.r32((fp.wrapping_add(32)).wrapping_add(8)))));
                    l_rb_12 = rt::fsub(rt::fsub(rt::fmul(rt::fneg(F32(c.m.r32(fp.wrapping_add(16)))), F32(c.m.r32(fp.wrapping_add(32)))), rt::fmul(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))), F32(c.m.r32((fp.wrapping_add(32)).wrapping_add(4))))), rt::fmul(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8))), F32(c.m.r32((fp.wrapping_add(32)).wrapping_add(8)))));
                    l_t_13 = rt::fdiv(rt::fsub(rt::fmul(l_l11_7, l_rb_12), rt::fmul(l_ra_11, l_l12_9)), l_detL_10);
                    l_s_14 = rt::fdiv(rt::fsub(l_ra_11, rt::fmul(l_l12_9, l_t_13)), l_l11_7);
                    { let v_ = rt::fadd(F32(c.m.r32(p_a_1)), rt::fmul(l_s_14, F32(c.m.r32(fp)))); let a_ = p_onA_3; c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fadd(F32(c.m.r32((p_a_1).wrapping_add(4))), rt::fmul(l_s_14, F32(c.m.r32((fp).wrapping_add(4))))); let a_ = (p_onA_3).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fadd(F32(c.m.r32((p_a_1).wrapping_add(8))), rt::fmul(l_s_14, F32(c.m.r32((fp).wrapping_add(8))))); let a_ = (p_onA_3).wrapping_add(8); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fadd(F32(c.m.r32(p_b_2)), rt::fmul(l_t_13, F32(c.m.r32(fp.wrapping_add(16))))); let a_ = p_onB_4; c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fadd(F32(c.m.r32((p_b_2).wrapping_add(4))), rt::fmul(l_t_13, F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))))); let a_ = (p_onB_4).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fadd(F32(c.m.r32((p_b_2).wrapping_add(8))), rt::fmul(l_t_13, F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8))))); let a_ = (p_onB_4).wrapping_add(8); c.m.w32(a_, (v_).0); };
                    if (((((((((((rt::flt(l_s_14, F32(0x00000000)) as i32)) != 0) || (((rt::fgt(l_s_14, F32(0x3f800000)) as i32)) != 0)) as i32)) != 0) || (((((((rt::flt(l_t_13, F32(0x00000000)) as i32)) != 0) || (((rt::fgt(l_t_13, F32(0x3f800000)) as i32)) != 0)) as i32)) != 0)) as i32)) != 0) {
                        F::ColSeg_AdjustNearest(c, p_a_1, fp, p_b_2, fp.wrapping_add(16), l_eps_5, l_s_14, l_t_13, p_onA_3, p_onB_4);
                    }
                }
            }
        }
        { let v_ = rt::fmul(F32(0x3f000000), rt::fadd(F32(c.m.r32(p_onA_3)), F32(c.m.r32(p_onB_4)))); let a_ = fp.wrapping_add(48); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(F32(0x3f000000), rt::fadd(F32(c.m.r32((p_onA_3).wrapping_add(4))), F32(c.m.r32((p_onB_4).wrapping_add(4))))); let a_ = (fp.wrapping_add(48)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(F32(0x3f000000), rt::fadd(F32(c.m.r32((p_onA_3).wrapping_add(8))), F32(c.m.r32((p_onB_4).wrapping_add(8))))); let a_ = (fp.wrapping_add(48)).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(p_onB_4)), F32(c.m.r32(p_onA_3))); let a_ = fp.wrapping_add(64); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32((p_onB_4).wrapping_add(4))), F32(c.m.r32((p_onA_3).wrapping_add(4)))); let a_ = (fp.wrapping_add(64)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32((p_onB_4).wrapping_add(8))), F32(c.m.r32((p_onA_3).wrapping_add(8)))); let a_ = (fp.wrapping_add(64)).wrapping_add(8); c.m.w32(a_, (v_).0); };
        break 'f (if (((rt::flt(rt::fadd(rt::fadd({ let a0_ = F32(c.m.r32(fp.wrapping_add(64))); rt::ffabs(a0_) }, { let a0_ = F32(c.m.r32((fp.wrapping_add(64)).wrapping_add(4))); rt::ffabs(a0_) }), { let a0_ = F32(c.m.r32((fp.wrapping_add(64)).wrapping_add(8))); rt::ffabs(a0_) }), l_eps_5) as i32)) != 0) { 1i32 } else { 0i32 });
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "ColSeg_Nearest", &PROBE_N, &[], true); }
    r_
}

pub fn ColCapsule_Set(c: &mut Ctx, mut p_cap_1: u32, mut p_a_2: u32, mut p_b_3: u32, mut p_radius_4: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ColCapsule_Set", &PROBE_N, &[(p_cap_1) as u32, (p_a_2) as u32, (p_b_3) as u32, (p_radius_4).0], false); }
    let r_ = 'f: {
    F::Vec4_Copy(c, p_cap_1, p_a_2);
    F::Vec4_Copy(c, (p_cap_1).wrapping_add(16), p_b_3);
    { let v_ = p_radius_4; let a_ = (p_cap_1).wrapping_add(32); c.m.w32(a_, (v_).0); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ColCapsule_Set", &PROBE_N, &[], true); }
    r_
}

pub fn ColSeg_GetMidpoint3(c: &mut Ctx, mut p_out_1: u32, mut p_seg_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ColSeg_GetMidpoint3", &PROBE_N, &[(p_out_1) as u32, (p_seg_2) as u32], false); }
    let r_ = 'f: {
    F::Vec4_Add(c, p_out_1, p_seg_2, (p_seg_2).wrapping_add(16));
    F::Vec3_Scale(c, p_out_1, p_out_1, F32(0x3f000000));
    { let v_ = F32(0x3f800000); let a_ = (p_out_1).wrapping_add(12); c.m.w32(a_, (v_).0); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ColSeg_GetMidpoint3", &PROBE_N, &[], true); }
    r_
}

pub fn ColSweep_FromCapsule(c: &mut Ctx, mut p_sw_1: u32, mut p_cap_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ColSweep_FromCapsule", &PROBE_N, &[(p_sw_1) as u32, (p_cap_2) as u32], false); }
    let mut l_len_3: F32 = F32(0);
    let r_ = 'f: {
    { let s_ = p_cap_2; let d_ = p_sw_1; rt::memcpy(c, d_, s_, 48); };
    F::Vec4_Copy(c, (p_sw_1).wrapping_add(48), p_sw_1);
    { let v_ = F32(c.m.r32((p_sw_1).wrapping_add(32))); let a_ = ((p_sw_1).wrapping_add(48)).wrapping_add(16); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fmul(F32(c.m.r32((p_sw_1).wrapping_add(32))), F32(c.m.r32((p_sw_1).wrapping_add(32)))); let a_ = ((p_sw_1).wrapping_add(48)).wrapping_add(20); c.m.w32(a_, (v_).0); };
    F::Vec4_Sub(c, (p_sw_1).wrapping_add(80), (p_sw_1).wrapping_add(16), p_sw_1);
    l_len_3 = { let a0_ = F::Vec3_Dot(c, (p_sw_1).wrapping_add(80), (p_sw_1).wrapping_add(80)); F::Port_sqrtf(c, a0_) };
    { let v_ = l_len_3; let a_ = (p_sw_1).wrapping_add(112); c.m.w32(a_, (v_).0); };
    if (((rt::fgt(l_len_3, F32(0x00000000)) as i32)) != 0) {
        F::Vec4_Scale(c, (p_sw_1).wrapping_add(96), (p_sw_1).wrapping_add(80), rt::fdiv(F32(0x3f800000), l_len_3));
    } else {
        { let v_ = F32(0x00000000); let a_ = (p_sw_1).wrapping_add(96); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = ((p_sw_1).wrapping_add(96)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = ((p_sw_1).wrapping_add(96)).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = ((p_sw_1).wrapping_add(96)).wrapping_add(12); c.m.w32(a_, (v_).0); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ColSweep_FromCapsule", &PROBE_N, &[], true); }
    r_
}

pub fn ColPlane_FromPoints(c: &mut Ctx, mut p_plane_1: u32, mut p_a_2: u32, mut p_b_3: u32, mut p_c_4: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ColPlane_FromPoints", &PROBE_N, &[(p_plane_1) as u32, (p_a_2) as u32, (p_b_3) as u32, (p_c_4) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let r_ = 'f: {
        F::Vec4_Sub(c, fp, p_a_2, p_b_3);
        F::Vec4_Sub(c, fp.wrapping_add(16), p_c_4, p_b_3);
        F::Vec3_Cross(c, fp.wrapping_add(32), fp.wrapping_add(16), fp);
        F::Vec3_Normalize(c, fp.wrapping_add(32), fp.wrapping_add(32));
        F::Vec4_Copy(c, p_plane_1, fp.wrapping_add(32));
        { let v_ = rt::fneg(F::Vec3_Dot(c, p_a_2, p_plane_1)); let a_ = (p_plane_1).wrapping_add(12); c.m.w32(a_, (v_).0); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "ColPlane_FromPoints", &PROBE_N, &[], true); }
    r_
}

pub fn ColPlane_ProjectPoint(c: &mut Ctx, mut p_out_1: u32, mut p_plane_2: u32, mut p_pt_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ColPlane_ProjectPoint", &PROBE_N, &[(p_out_1) as u32, (p_plane_2) as u32, (p_pt_3) as u32], false); }
    let mut l_d_4: F32 = F32(0);
    let r_ = 'f: {
    l_d_4 = rt::fadd(F::Vec3_Dot(c, p_plane_2, p_pt_3), F32(c.m.r32((p_plane_2).wrapping_add(12))));
    { let v_ = rt::fsub(F32(c.m.r32(p_pt_3)), rt::fmul(F32(c.m.r32(p_plane_2)), l_d_4)); let a_ = p_out_1; c.m.w32(a_, (v_).0); };
    { let v_ = rt::fsub(F32(c.m.r32((p_pt_3).wrapping_add(4))), rt::fmul(F32(c.m.r32((p_plane_2).wrapping_add(4))), l_d_4)); let a_ = (p_out_1).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fsub(F32(c.m.r32((p_pt_3).wrapping_add(8))), rt::fmul(F32(c.m.r32((p_plane_2).wrapping_add(8))), l_d_4)); let a_ = (p_out_1).wrapping_add(8); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x3f800000); let a_ = (p_out_1).wrapping_add(12); c.m.w32(a_, (v_).0); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ColPlane_ProjectPoint", &PROBE_N, &[], true); }
    r_
}

pub fn ColPlane_DistPoint(c: &mut Ctx, mut p_plane_1: u32, mut p_pt_2: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ColPlane_DistPoint", &PROBE_N, &[(p_plane_1) as u32, (p_pt_2) as u32], false); }
    let r_ = 'f: {
    break 'f rt::fsub(F::Vec3_Dot(c, p_plane_1, p_pt_2), F32(c.m.r32((p_plane_1).wrapping_add(12))));
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "ColPlane_DistPoint", &PROBE_N, &[], true); }
    r_
}

pub fn ColSeg_Set2(c: &mut Ctx, mut p_seg_1: u32, mut p_a_2: u32, mut p_b_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ColSeg_Set2", &PROBE_N, &[(p_seg_1) as u32, (p_a_2) as u32, (p_b_3) as u32], false); }
    let r_ = 'f: {
    { let v_ = F32(c.m.r32(p_a_2)); let a_ = p_seg_1; c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((p_a_2).wrapping_add(4))); let a_ = (p_seg_1).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((p_a_2).wrapping_add(8))); let a_ = (p_seg_1).wrapping_add(8); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x3f800000); let a_ = (p_seg_1).wrapping_add(12); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32(p_b_3)); let a_ = (p_seg_1).wrapping_add(16); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((p_b_3).wrapping_add(4))); let a_ = ((p_seg_1).wrapping_add(16)).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((p_b_3).wrapping_add(8))); let a_ = ((p_seg_1).wrapping_add(16)).wrapping_add(8); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x3f800000); let a_ = ((p_seg_1).wrapping_add(16)).wrapping_add(12); c.m.w32(a_, (v_).0); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ColSeg_Set2", &PROBE_N, &[], true); }
    r_
}

pub fn ColSeg_Set(c: &mut Ctx, mut p_seg_1: u32, mut p_a_2: u32, mut p_b_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ColSeg_Set", &PROBE_N, &[(p_seg_1) as u32, (p_a_2) as u32, (p_b_3) as u32], false); }
    let r_ = 'f: {
    { let v_ = F32(c.m.r32(p_a_2)); let a_ = p_seg_1; c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((p_a_2).wrapping_add(4))); let a_ = (p_seg_1).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((p_a_2).wrapping_add(8))); let a_ = (p_seg_1).wrapping_add(8); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x3f800000); let a_ = (p_seg_1).wrapping_add(12); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32(p_b_3)); let a_ = (p_seg_1).wrapping_add(16); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((p_b_3).wrapping_add(4))); let a_ = ((p_seg_1).wrapping_add(16)).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((p_b_3).wrapping_add(8))); let a_ = ((p_seg_1).wrapping_add(16)).wrapping_add(8); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x3f800000); let a_ = ((p_seg_1).wrapping_add(16)).wrapping_add(12); c.m.w32(a_, (v_).0); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ColSeg_Set", &PROBE_N, &[], true); }
    r_
}

pub fn ColSeg_GetMidpoint(c: &mut Ctx, mut p_seg_1: u32, mut p_out_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ColSeg_GetMidpoint", &PROBE_N, &[(p_seg_1) as u32, (p_out_2) as u32], false); }
    let r_ = 'f: {
    F::Vec4_Copy(c, p_out_2, p_seg_1);
    F::Vec4_Add(c, p_out_2, p_out_2, (p_seg_1).wrapping_add(16));
    F::Vec4_Scale(c, p_out_2, p_out_2, F32(0x3f000000));
    { let v_ = F32(0x3f800000); let a_ = (p_out_2).wrapping_add(12); c.m.w32(a_, (v_).0); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ColSeg_GetMidpoint", &PROBE_N, &[], true); }
    r_
}

pub fn ColSphere_Set(c: &mut Ctx, mut p_sph_1: u32, mut p_pos_2: u32, mut p_radius_3: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ColSphere_Set", &PROBE_N, &[(p_sph_1) as u32, (p_pos_2) as u32, (p_radius_3).0], false); }
    let r_ = 'f: {
    { let v_ = F32(c.m.r32(p_pos_2)); let a_ = p_sph_1; c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((p_pos_2).wrapping_add(4))); let a_ = (p_sph_1).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((p_pos_2).wrapping_add(8))); let a_ = (p_sph_1).wrapping_add(8); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x3f800000); let a_ = (p_sph_1).wrapping_add(12); c.m.w32(a_, (v_).0); };
    { let v_ = p_radius_3; let a_ = (p_sph_1).wrapping_add(16); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fmul(p_radius_3, p_radius_3); let a_ = (p_sph_1).wrapping_add(20); c.m.w32(a_, (v_).0); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ColSphere_Set", &PROBE_N, &[], true); }
    r_
}

pub fn ColTri_Set(c: &mut Ctx, mut p_tri_1: u32, mut p_a_2: u32, mut p_b_3: u32, mut p_c_4: u32, mut p_nrm_5: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ColTri_Set", &PROBE_N, &[(p_tri_1) as u32, (p_a_2) as u32, (p_b_3) as u32, (p_c_4) as u32, (p_nrm_5) as u32], false); }
    let r_ = 'f: {
    F::Vec4_Copy(c, (p_tri_1).wrapping_add((((0i32) as u32)).wrapping_mul(16)), p_a_2);
    F::Vec4_Copy(c, (p_tri_1).wrapping_add((((1i32) as u32)).wrapping_mul(16)), p_b_3);
    F::Vec4_Copy(c, (p_tri_1).wrapping_add((((2i32) as u32)).wrapping_mul(16)), p_c_4);
    F::Vec4_Copy(c, (p_tri_1).wrapping_add(48), p_nrm_5);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ColTri_Set", &PROBE_N, &[], true); }
    r_
}

pub fn ColTri_ContainsPoint(c: &mut Ctx, mut p_tri_1: u32, mut p_pt_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ColTri_ContainsPoint", &PROBE_N, &[(p_tri_1) as u32, (p_pt_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let mut l_a_3: F32 = F32(0);
    let mut l_b_4: F32 = F32(0);
    let mut l_c_5: F32 = F32(0);
    let mut l_d0_6: F32 = F32(0);
    let mut l_d1_7: F32 = F32(0);
    let mut l_det_8: F32 = F32(0);
    let mut l_inv_9: F32 = F32(0);
    let mut l_u_10: F32 = F32(0);
    let mut l_v_11: F32 = F32(0);
    let mut l_ret_12: i32 = 0;
    let r_ = 'f: {
        F::Vec4_Sub(c, fp, (p_tri_1).wrapping_add((((1i32) as u32)).wrapping_mul(16)), (p_tri_1).wrapping_add((((0i32) as u32)).wrapping_mul(16)));
        F::Vec4_Sub(c, fp.wrapping_add(16), (p_tri_1).wrapping_add((((2i32) as u32)).wrapping_mul(16)), (p_tri_1).wrapping_add((((0i32) as u32)).wrapping_mul(16)));
        F::Vec4_Sub(c, fp.wrapping_add(32), p_pt_2, (p_tri_1).wrapping_add((((0i32) as u32)).wrapping_mul(16)));
        l_a_3 = F::Vec3_Dot(c, fp, fp);
        l_b_4 = F::Vec3_Dot(c, fp, fp.wrapping_add(16));
        l_c_5 = F::Vec3_Dot(c, fp.wrapping_add(16), fp.wrapping_add(16));
        l_d0_6 = F::Vec3_Dot(c, fp.wrapping_add(32), fp);
        l_d1_7 = F::Vec3_Dot(c, fp.wrapping_add(32), fp.wrapping_add(16));
        l_det_8 = rt::fsub(rt::fmul(l_b_4, l_b_4), rt::fmul(l_a_3, l_c_5));
        if (((rt::feq(l_det_8, F32(0x00000000)) as i32)) != 0) {
            l_det_8 = F32(0x3a83126e);
        }
        l_inv_9 = rt::fdiv(F32(0x3f800000), l_det_8);
        l_u_10 = rt::fmul(rt::fsub(rt::fmul(l_b_4, l_d1_7), rt::fmul(l_c_5, l_d0_6)), l_inv_9);
        if (((((((rt::flt(l_u_10, F32(0x00000000)) as i32)) != 0) || (((rt::fgt(l_u_10, F32(0x3f800000)) as i32)) != 0)) as i32)) != 0) {
            break 'f 0i32;
        }
        l_v_11 = rt::fmul(rt::fsub(rt::fmul(l_b_4, l_d0_6), rt::fmul(l_a_3, l_d1_7)), l_inv_9);
        if (((rt::flt(l_v_11, F32(0x00000000)) as i32)) != 0) {
            break 'f 0i32;
        }
        l_ret_12 = 1i32;
        if (((rt::fgt(rt::fadd(l_u_10, l_v_11), F32(0x3f800000)) as i32)) != 0) {
            l_ret_12 = 0i32;
        }
        break 'f l_ret_12;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "ColTri_ContainsPoint", &PROBE_N, &[], true); }
    r_
}
