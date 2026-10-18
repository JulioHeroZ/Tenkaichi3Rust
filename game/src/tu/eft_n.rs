#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn EftAura_GetNodeFade(c: &mut Ctx, mut p_objId_1: i32, mut p_pos_2: u32, mut p_nodes_3: u32, mut p_scale_4: F32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftAura_GetNodeFade", &PROBE_N, &[(p_objId_1) as u32, (p_pos_2) as u32, (p_nodes_3) as u32, (p_scale_4).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_dist_5: F32 = F32(0);
    let mut l_ratio_6: F32 = F32(0);
    let mut l_limit_7: F32 = F32(0);
    let mut l_i_8: i32 = 0;
    let mut l_t_9: F32 = F32(0);
    let r_ = 'f: {
        rt::memset(c, fp.wrapping_add(16), 0, 8);
        { let v_ = F32(0xbf800000); c.m.w32(fp.wrapping_add(16).wrapping_add(0), (v_).0); }
        { let v_ = F32(0xbf800000); c.m.w32(fp.wrapping_add(16).wrapping_add(4), (v_).0); }
        l_dist_5 = F32(0x00000000);
        l_ratio_6 = F32(0x3f800000);
        {
            l_i_8 = 0i32;
            'l1: loop {
                if !(((((l_i_8) < (7i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    F::Vec4_Sub(c, fp, p_pos_2, (p_nodes_3).wrapping_add((((l_i_8) as u32)).wrapping_mul(16)));
                    if (((((l_i_8) == (0i32)) as i32)) != 0) {
                        l_dist_5 = F::Vec3_LengthSq(c, fp);
                    } else {
                        l_t_9 = F::Vec3_LengthSq(c, fp);
                        if (((rt::flt(l_t_9, l_dist_5) as i32)) != 0) {
                            l_dist_5 = l_t_9;
                        }
                    }
                    if (((rt::flt(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(0x00000000)) as i32)) != 0) {
                        { let v_ = l_dist_5; let a_ = (fp.wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    } else {
                        if (((rt::flt(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(0x00000000)) as i32)) != 0) {
                            if (((rt::flt(l_dist_5, F32(c.m.r32((fp.wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4))))) as i32)) != 0) {
                                { let v_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = (fp.wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                                { let v_ = l_dist_5; let a_ = (fp.wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                            } else {
                                { let v_ = l_dist_5; let a_ = (fp.wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                            }
                        } else {
                            if (((rt::flt(l_dist_5, F32(c.m.r32((fp.wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4))))) as i32)) != 0) {
                                { let v_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = (fp.wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                                { let v_ = l_dist_5; let a_ = (fp.wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                            } else {
                                if (((rt::flt(l_dist_5, F32(c.m.r32((fp.wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(4))))) as i32)) != 0) {
                                    { let v_ = l_dist_5; let a_ = (fp.wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                                }
                            }
                        }
                    }
                }
                { let o_ = l_i_8; let n_ = (o_).wrapping_add(1); l_i_8 = n_; };
            }
        }
        { let v_ = { let a0_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); F::Port_sqrtf(c, a0_) }; let a_ = (fp.wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); F::Port_sqrtf(c, a0_) }; let a_ = (fp.wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        l_dist_5 = rt::fadd(rt::fmul(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), { let a_ = (c.m.r32(sym::gEftAuraPrm)).wrapping_add(24); F32(c.m.r32(a_)) }), rt::fmul(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), rt::fsub(F32(0x3f800000), { let a_ = (c.m.r32(sym::gEftAuraPrm)).wrapping_add(24); F32(c.m.r32(a_)) })));
        l_limit_7 = rt::fmul(p_scale_4, F32(0x40900000));
        if (((rt::flt(l_dist_5, l_limit_7) as i32)) != 0) {
            l_ratio_6 = rt::fdiv(l_dist_5, l_limit_7);
            l_limit_7 = rt::fmul(l_ratio_6, l_ratio_6);
            { let r_ = l_ratio_6; let n_ = rt::fmul(l_limit_7, r_); l_limit_7 = n_; };
            l_ratio_6 = rt::fmul(l_limit_7, l_limit_7);
        }
        break 'f rt::fadd(rt::fmul(l_ratio_6, F32(0x3f000000)), F32(0x3f000000));
       F32(0)
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftAura_GetNodeFade", &PROBE_N, &[], true); }
    r_
}

pub fn eft_n__EftAura_FlameFlagInl(c: &mut Ctx, mut p_f_1: u32, mut p_bit_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "eft_n__EftAura_FlameFlagInl", &PROBE_N, &[(p_f_1) as u32, (p_bit_2) as u32], false); }
    let r_ = 'f: {
    if (((((((c.m.r32(p_f_1) as i32)) as u32)) & (p_bit_2))) != 0) {
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "eft_n__EftAura_FlameFlagInl", &PROBE_N, &[], true); }
    r_
}

pub fn eft_n__EftAura_WorkFlagInl(c: &mut Ctx, mut p_aura_1: u32, mut p_bit_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "eft_n__EftAura_WorkFlagInl", &PROBE_N, &[(p_aura_1) as u32, (p_bit_2) as u32], false); }
    let r_ = 'f: {
    if (((((((c.m.r32(p_aura_1) as i32)) as u32)) & (p_bit_2))) != 0) {
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "eft_n__EftAura_WorkFlagInl", &PROBE_N, &[], true); }
    r_
}

pub fn eft_n__EftAura_OtAdd(c: &mut Ctx, mut p_p_1: u32, mut p_z_2: i32, mut p_layer_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "eft_n__EftAura_OtAdd", &PROBE_N, &[(p_p_1) as u32, (p_z_2) as u32, (p_layer_3) as u32], false); }
    let mut l_e_4: u32 = 0;
    let r_ = 'f: {
    if (((((p_layer_3) >= (2i32)) as i32)) != 0) {
        { let r_ = 2i32; let n_ = (p_layer_3).wrapping_sub(r_); p_layer_3 = n_; };
    }
    if (((((p_z_2) < (0i32)) as i32)) != 0) {
        l_e_4 = ((c.m.r32(sym::gOtZ)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((p_layer_3) as u32)).wrapping_mul(8));
    } else {
        if (((((p_z_2) >= (4096i32)) as i32)) != 0) {
            l_e_4 = ((c.m.r32(sym::gOtZ)).wrapping_add((((4095i32) as u32)).wrapping_mul(16))).wrapping_add((((p_layer_3) as u32)).wrapping_mul(8));
        } else {
            l_e_4 = ((c.m.r32(sym::gOtZ)).wrapping_add((((p_z_2) as u32)).wrapping_mul(16))).wrapping_add((((p_layer_3) as u32)).wrapping_mul(8));
        }
    }
    { let v_ = p_p_1; let a_ = (c.m.r32((l_e_4).wrapping_add(4))).wrapping_add(4); c.m.w32(a_, v_); };
    { let v_ = p_p_1; let a_ = (l_e_4).wrapping_add(4); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "eft_n__EftAura_OtAdd", &PROBE_N, &[], true); }
    r_
}

pub fn EftAura_DrawFlames(c: &mut Ctx, mut p_aura_1: u32, mut p_objId_2: i32, mut p_alpha_3: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftAura_DrawFlames", &PROBE_N, &[(p_aura_1) as u32, (p_objId_2) as u32, (p_alpha_3).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(448) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 448);
    let mut l_sparkTex_4: u32 = 0;
    let mut l_clipped_5: i32 = 0;
    let mut l_f_6: u32 = 0;
    let mut l_link_7: u32 = 0;
    let mut l_pkt_8: u32 = 0;
    let mut l_i_9: i32 = 0;
    let mut l_z_10: i32 = 0;
    let mut l_a_11: F32 = F32(0);
    let mut l_w_12: F32 = F32(0);
    let mut l_h_13: F32 = F32(0);
    let mut l_d_14: F32 = F32(0);
    let r_ = 'f: {
        { let v_ = { let a_ = (((c.m.r32(sym::gBtlCamView)).wrapping_add(64)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }; let a_ = fp.wrapping_add(216); c.m.w32(a_, (v_).0); };
        { let v_ = { let a_ = (((c.m.r32(sym::gBtlCamView)).wrapping_add(64)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }; let a_ = (fp.wrapping_add(216)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = { let a_ = (((c.m.r32(sym::gBtlCamView)).wrapping_add(64)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }; let a_ = (fp.wrapping_add(216)).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = (fp.wrapping_add(216)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        l_sparkTex_4 = (((c.m.r32(sym::gEftAuraPool)).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(560))).wrapping_add(40);
        if (((((c.m.r32(p_aura_1) as i32)) & (128i32))) != 0) {
            { let v_ = ((c.m.r32(sym::gEftAuraCfg)).wrapping_add(656)).wrapping_add((((0i32) as u32)).wrapping_mul(64)); let a_ = (fp.wrapping_add(64)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
            { let v_ = ((c.m.r32(sym::gEftAuraCfg)).wrapping_add(656)).wrapping_add((((1i32) as u32)).wrapping_mul(64)); let a_ = (fp.wrapping_add(64)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        } else {
            { let v_ = ((c.m.r32(sym::gEftAuraCfg)).wrapping_add(528)).wrapping_add((((0i32) as u32)).wrapping_mul(64)); let a_ = (fp.wrapping_add(64)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
            { let v_ = ((c.m.r32(sym::gEftAuraCfg)).wrapping_add(528)).wrapping_add((((1i32) as u32)).wrapping_mul(64)); let a_ = (fp.wrapping_add(64)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        }
        { let v_ = ((c.m.r32(sym::gEftAuraCfg)).wrapping_add(784)).wrapping_add((((0i32) as u32)).wrapping_mul(64)); let a_ = (fp.wrapping_add(72)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ((c.m.r32(sym::gEftAuraCfg)).wrapping_add(784)).wrapping_add((((1i32) as u32)).wrapping_mul(64)); let a_ = (fp.wrapping_add(72)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ((c.m.r32(sym::gEftAuraCfg)).wrapping_add(784)).wrapping_add((((2i32) as u32)).wrapping_mul(64)); let a_ = (fp.wrapping_add(72)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ((c.m.r32(sym::gEftAuraCfg)).wrapping_add(784)).wrapping_add((((3i32) as u32)).wrapping_mul(64)); let a_ = (fp.wrapping_add(72)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        {
            l_i_9 = 0i32;
            'l1: loop {
                if !(((((l_i_9) < (7i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let a1_ = { let a_ = ((c.m.r32(sym::gEftAuraCfg)).wrapping_add(1160)).wrapping_add((((l_i_9) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; F::BtlCharApi_GetNodePos(c, p_objId_2, a1_, (fp.wrapping_add(336)).wrapping_add((((l_i_9) as u32)).wrapping_mul(16))) };
                }
                { let o_ = l_i_9; let n_ = (o_).wrapping_add(1); l_i_9 = n_; };
            }
        }
        l_a_11 = rt::fmul(rt::fmul(F32(c.m.r32((p_aura_1).wrapping_add(56))), F32(c.m.r32((p_aura_1).wrapping_add(80)))), p_alpha_3);
        F::Dbg_ProfMark(c, sym::gBattleProf);
        {
            l_link_7 = (c.m.r32(sym::gEftAuraPool)).wrapping_add(20);
            'l3: loop {
                if !(((((c.m.r32(l_link_7)) != (((0i32) as u32))) as i32)) != 0) { break 'l3; }
                'c4: {
                    l_f_6 = c.m.r32(l_link_7);
                    if (((((((c.m.r8((l_f_6).wrapping_add(7))) as i32)) != (p_objId_2)) as i32)) != 0) {
                        break 'c4;
                    }
                    { let v_ = F32(c.m.r32((l_f_6).wrapping_add(128))); let a_ = fp.wrapping_add(152); c.m.w32(a_, (v_).0); };
                    l_clipped_5 = 0i32;
                    { let v_ = F32(c.m.r32(((l_f_6).wrapping_add(128)).wrapping_add(4))); let a_ = (fp.wrapping_add(152)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(c.m.r32(((l_f_6).wrapping_add(128)).wrapping_add(8))); let a_ = (fp.wrapping_add(152)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(0x3f800000); let a_ = (fp.wrapping_add(152)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                    if ((((({ let a_ = (((c.m.r32(sym::gEftAuraCfg)).wrapping_add(1040)).wrapping_add((((c.m.r8((l_f_6).wrapping_add(9))) as u32)).wrapping_mul(12))).wrapping_add(8); (c.m.r32(a_) as i32) }) >= (0i32)) as i32)) != 0) {
                        l_d_14 = { let a0_ = F::Vec3_Dot(c, fp.wrapping_add(216), fp.wrapping_add(152)); rt::ffabs(a0_) };
                        { let a1_ = ((p_aura_1).wrapping_add(624)).wrapping_add((((c.m.r8((l_f_6).wrapping_add(9))) as u32)).wrapping_mul(16)); F::Vec4_Sub(c, fp.wrapping_add(232), a1_, (p_aura_1).wrapping_add(112)) };
                        { let v_ = F32(0x00000000); let a_ = (fp.wrapping_add(232)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                        F::Vec3_Normalize(c, fp.wrapping_add(232), fp.wrapping_add(232));
                        { let r_ = rt::fmul(rt::fmul(F32(c.m.r32(fp.wrapping_add(232))), l_d_14), F32(0x3f000000)); let a_ = fp.wrapping_add(152); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                        { let r_ = rt::fmul(rt::fmul(F32(c.m.r32((fp.wrapping_add(232)).wrapping_add(8))), l_d_14), F32(0x3f000000)); let a_ = (fp.wrapping_add(152)).wrapping_add(8); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    }
                    F::Vec3_Normalize(c, fp.wrapping_add(168), fp.wrapping_add(152));
                    F::EftAura_BuildFlameMtx(c, fp, fp.wrapping_add(168), (l_f_6).wrapping_add(80));
                    l_w_12 = rt::fmul(rt::fmul(F32(c.m.r32((l_f_6).wrapping_add(32))), F32(c.m.r32((l_f_6).wrapping_add(24)))), F32(c.m.r32((p_aura_1).wrapping_add(88))));
                    l_h_13 = rt::fmul(rt::fmul(F32(c.m.r32((l_f_6).wrapping_add(28))), F32(c.m.r32((l_f_6).wrapping_add(24)))), F32(c.m.r32((p_aura_1).wrapping_add(88))));
                    {
                        l_i_9 = 0i32;
                        'l5: loop {
                            if !(((((l_i_9) < (4i32)) as i32)) != 0) { break 'l5; }
                            'c6: {
                                { let v_ = rt::fmul({ let a_ = ({ let a_ = (fp.wrapping_add(64)).wrapping_add((((c.m.r8((l_f_6).wrapping_add(4))) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add((((l_i_9) as u32)).wrapping_mul(16)); F32(c.m.r32(a_)) }, l_w_12); let a_ = fp.wrapping_add(200); c.m.w32(a_, (v_).0); };
                                { let v_ = rt::fmul({ let a_ = (({ let a_ = (fp.wrapping_add(64)).wrapping_add((((c.m.r8((l_f_6).wrapping_add(4))) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add((((l_i_9) as u32)).wrapping_mul(16))).wrapping_add(4); F32(c.m.r32(a_)) }, F32(c.m.r32((l_f_6).wrapping_add(24)))); let a_ = (fp.wrapping_add(200)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                                { let v_ = rt::fmul({ let a_ = (({ let a_ = (fp.wrapping_add(64)).wrapping_add((((c.m.r8((l_f_6).wrapping_add(4))) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add((((l_i_9) as u32)).wrapping_mul(16))).wrapping_add(8); F32(c.m.r32(a_)) }, l_h_13); let a_ = (fp.wrapping_add(200)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                                { let v_ = F32(0x3f800000); let a_ = (fp.wrapping_add(200)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                                if (((((l_i_9) >= (2i32)) as i32)) != 0) {
                                    { let r_ = F32(c.m.r32((l_f_6).wrapping_add(44))); let a_ = (fp.wrapping_add(200)).wrapping_add(8); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                                }
                                F::Mtx_MulVec4(c, fp.wrapping_add(184), fp, fp.wrapping_add(200));
                                { let _ = F::Vu0Cur_ProjectPoint(c, (fp.wrapping_add(252)).wrapping_add((((l_i_9) as u32)).wrapping_mul(16)), fp.wrapping_add(184)); };
                                if (((((c.m.r32((fp.wrapping_add(252)).wrapping_add((((l_i_9) as u32)).wrapping_mul(16)))) > (((65520i32) as u32))) as i32)) != 0) {
                                    l_clipped_5 = 1i32;
                                    break 'l5;
                                }
                                if (((((c.m.r32(((fp.wrapping_add(252)).wrapping_add((((l_i_9) as u32)).wrapping_mul(16))).wrapping_add(4))) > (((65520i32) as u32))) as i32)) != 0) {
                                    l_clipped_5 = 1i32;
                                    break 'l5;
                                }
                                if ((((((c.m.r32(((fp.wrapping_add(252)).wrapping_add((((l_i_9) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)) < (0i32)) as i32)) != 0) {
                                    l_clipped_5 = 1i32;
                                    break 'l5;
                                }
                                { let a1_ = ({ let a_ = (fp.wrapping_add(72)).wrapping_add((((((((c.m.r8((l_f_6).wrapping_add(4))) as i32)).wrapping_mul(2i32)).wrapping_add(((c.m.r8((l_f_6).wrapping_add(10))) as i32))) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add((((l_i_9) as u32)).wrapping_mul(16)); let a2_ = rt::fdiv(F32(0x3f800000), rt::i2f((c.m.r32(((fp.wrapping_add(252)).wrapping_add((((l_i_9) as u32)).wrapping_mul(16))).wrapping_add(12)) as i32))); F::Vec4_Scale(c, (fp.wrapping_add(88)).wrapping_add((((l_i_9) as u32)).wrapping_mul(16)), a1_, a2_) };
                                { let v_ = { let a0_ = ((c.m.r8((l_f_6).wrapping_add(7))) as i32); let a3_ = F32(c.m.r32((p_aura_1).wrapping_add(96))); F::EftAura_GetNodeFade(c, a0_, fp.wrapping_add(184), fp.wrapping_add(336), a3_) }; let a_ = (fp.wrapping_add(316)).wrapping_add((((l_i_9) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                                if (((((l_i_9) < (2i32)) as i32)) != 0) {
                                    { let r_ = F32(0x3dcccccc); let a_ = (fp.wrapping_add(316)).wrapping_add((((l_i_9) as u32)).wrapping_mul(4)); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                                }
                            }
                            { let o_ = l_i_9; let n_ = (o_).wrapping_add(1); l_i_9 = n_; };
                        }
                    }
                    if ((l_clipped_5) != 0) {
                        break 'c4;
                    }
                    l_z_10 = (((((c.m.r32(((fp.wrapping_add(252)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)).wrapping_add((c.m.r32(((fp.wrapping_add(252)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32))).wrapping_add((c.m.r32(((fp.wrapping_add(252)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32))).wrapping_add((c.m.r32(((fp.wrapping_add(252)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32))).wrapping_shr((10i32) as u32);
                    { let v_ = ((rt::f2u(rt::fmul(F32(c.m.r32((l_f_6).wrapping_add(96))), F32(0x437f0000)))) as u8); let a_ = (fp.wrapping_add(248)).wrapping_add(((0i32) as u32)); c.m.w8(a_, v_); };
                    { let v_ = ((rt::f2u(rt::fmul(F32(c.m.r32(((l_f_6).wrapping_add(96)).wrapping_add(4))), F32(0x437f0000)))) as u8); let a_ = (fp.wrapping_add(248)).wrapping_add(((1i32) as u32)); c.m.w8(a_, v_); };
                    { let v_ = ((rt::f2u(rt::fmul(F32(c.m.r32(((l_f_6).wrapping_add(96)).wrapping_add(8))), F32(0x437f0000)))) as u8); let a_ = (fp.wrapping_add(248)).wrapping_add(((2i32) as u32)); c.m.w8(a_, v_); };
                    {
                        l_i_9 = 0i32;
                        'l7: loop {
                            if !(((((l_i_9) < (4i32)) as i32)) != 0) { break 'l7; }
                            'c8: {
                                { let v_ = ((rt::f2u(rt::fmul(F32(c.m.r32((fp.wrapping_add(316)).wrapping_add((((l_i_9) as u32)).wrapping_mul(4)))), rt::fmul(rt::fmul(F32(c.m.r32(((l_f_6).wrapping_add(96)).wrapping_add(12))), F32(0x437f0000)), l_a_11)))) as u8); let a_ = (fp.wrapping_add(332)).wrapping_add(((l_i_9) as u32)); c.m.w8(a_, v_); };
                            }
                            { let o_ = l_i_9; let n_ = (o_).wrapping_add(1); l_i_9 = n_; };
                        }
                    }
                    if ((F::eft_n__EftAura_FlameFlagInl(c, l_f_6, ((2048i32) as u32))) != 0) {
                        break 'c4;
                    }
                    l_pkt_8 = c.m.r32(sym::gOtCur);
                    { let v_ = (l_pkt_8).wrapping_add((((1i32) as u32)).wrapping_mul(144)); let a_ = sym::gOtCur; c.m.w32(a_, v_); };
                    if (((((l_pkt_8) == (((0i32) as u32))) as i32)) != 0) {
                        break 'f;
                    }
                    { let v_ = ((92i32) as u64); let a_ = (l_pkt_8).wrapping_add(32); c.m.w64(a_, v_); };
                    { let v_ = ((536870920i32) as u32); let a_ = l_pkt_8; c.m.w32(a_, v_); };
                    { let v_ = ((268435456i32) as u32); let a_ = (l_pkt_8).wrapping_add(8); c.m.w32(a_, v_); };
                    { let v_ = ((1342177288i32) as u32); let a_ = (l_pkt_8).wrapping_add(12); c.m.w32(a_, v_); };
                    { let v_ = 16429131440647602177u64; let a_ = (l_pkt_8).wrapping_add(16); c.m.w64(a_, v_); };
                    { let v_ = ((18599481537995104i64) as u64); let a_ = (l_pkt_8).wrapping_add(24); c.m.w64(a_, v_); };
                    { let v_ = ((0i32) as u32); let a_ = (l_pkt_8).wrapping_add(4); c.m.w32(a_, v_); };
                    { let v_ = c.m.r8((fp.wrapping_add(248)).wrapping_add(((0i32) as u32))); let a_ = ((l_pkt_8).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24)); c.m.w8(a_, v_); };
                    { let v_ = c.m.r8((fp.wrapping_add(248)).wrapping_add(((1i32) as u32))); let a_ = (((l_pkt_8).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(1); c.m.w8(a_, v_); };
                    { let v_ = c.m.r8((fp.wrapping_add(248)).wrapping_add(((2i32) as u32))); let a_ = (((l_pkt_8).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(2); c.m.w8(a_, v_); };
                    { let v_ = c.m.r8((fp.wrapping_add(332)).wrapping_add(((0i32) as u32))); let a_ = (((l_pkt_8).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(3); c.m.w8(a_, v_); };
                    { let v_ = F32(c.m.r32(((fp.wrapping_add(88)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8))); let a_ = (((l_pkt_8).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = c.m.r8((fp.wrapping_add(248)).wrapping_add(((0i32) as u32))); let a_ = ((l_pkt_8).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24)); c.m.w8(a_, v_); };
                    { let v_ = c.m.r8((fp.wrapping_add(248)).wrapping_add(((1i32) as u32))); let a_ = (((l_pkt_8).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(1); c.m.w8(a_, v_); };
                    { let v_ = c.m.r8((fp.wrapping_add(248)).wrapping_add(((2i32) as u32))); let a_ = (((l_pkt_8).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(2); c.m.w8(a_, v_); };
                    { let v_ = c.m.r8((fp.wrapping_add(332)).wrapping_add(((1i32) as u32))); let a_ = (((l_pkt_8).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(3); c.m.w8(a_, v_); };
                    { let v_ = F32(c.m.r32(((fp.wrapping_add(88)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8))); let a_ = (((l_pkt_8).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = c.m.r8((fp.wrapping_add(248)).wrapping_add(((0i32) as u32))); let a_ = ((l_pkt_8).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24)); c.m.w8(a_, v_); };
                    { let v_ = c.m.r8((fp.wrapping_add(248)).wrapping_add(((1i32) as u32))); let a_ = (((l_pkt_8).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(1); c.m.w8(a_, v_); };
                    { let v_ = c.m.r8((fp.wrapping_add(248)).wrapping_add(((2i32) as u32))); let a_ = (((l_pkt_8).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(2); c.m.w8(a_, v_); };
                    { let v_ = c.m.r8((fp.wrapping_add(332)).wrapping_add(((2i32) as u32))); let a_ = (((l_pkt_8).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(3); c.m.w8(a_, v_); };
                    { let v_ = F32(c.m.r32(((fp.wrapping_add(88)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8))); let a_ = (((l_pkt_8).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = c.m.r8((fp.wrapping_add(248)).wrapping_add(((0i32) as u32))); let a_ = ((l_pkt_8).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24)); c.m.w8(a_, v_); };
                    { let v_ = c.m.r8((fp.wrapping_add(248)).wrapping_add(((1i32) as u32))); let a_ = (((l_pkt_8).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(1); c.m.w8(a_, v_); };
                    { let v_ = c.m.r8((fp.wrapping_add(248)).wrapping_add(((2i32) as u32))); let a_ = (((l_pkt_8).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(2); c.m.w8(a_, v_); };
                    { let v_ = c.m.r8((fp.wrapping_add(332)).wrapping_add(((3i32) as u32))); let a_ = (((l_pkt_8).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(3); c.m.w8(a_, v_); };
                    { let v_ = F32(c.m.r32(((fp.wrapping_add(88)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(8))); let a_ = (((l_pkt_8).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(c.m.r32((fp.wrapping_add(88)).wrapping_add((((0i32) as u32)).wrapping_mul(16)))); let a_ = (((l_pkt_8).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(8); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(c.m.r32(((fp.wrapping_add(88)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4))); let a_ = (((l_pkt_8).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(12); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(c.m.r32((fp.wrapping_add(88)).wrapping_add((((1i32) as u32)).wrapping_mul(16)))); let a_ = (((l_pkt_8).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(8); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(c.m.r32(((fp.wrapping_add(88)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(4))); let a_ = (((l_pkt_8).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(12); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(c.m.r32((fp.wrapping_add(88)).wrapping_add((((2i32) as u32)).wrapping_mul(16)))); let a_ = (((l_pkt_8).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(8); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(c.m.r32(((fp.wrapping_add(88)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(4))); let a_ = (((l_pkt_8).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(12); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(c.m.r32((fp.wrapping_add(88)).wrapping_add((((3i32) as u32)).wrapping_mul(16)))); let a_ = (((l_pkt_8).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(8); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(c.m.r32(((fp.wrapping_add(88)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(4))); let a_ = (((l_pkt_8).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(12); c.m.w32(a_, (v_).0); };
                    { let v_ = ((c.m.r32((fp.wrapping_add(252)).wrapping_add((((0i32) as u32)).wrapping_mul(16)))) as u64); let a_ = (((l_pkt_8).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(16); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                    { let v_ = ((c.m.r32(((fp.wrapping_add(252)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4))) as u64); let a_ = (((l_pkt_8).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(18); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                    { let v_ = (((c.m.r32(((fp.wrapping_add(252)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)) as u64); let a_ = (((l_pkt_8).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(20); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
                    { let v_ = ((255i32) as u64); let a_ = (((l_pkt_8).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(23); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
                    { let v_ = ((c.m.r32((fp.wrapping_add(252)).wrapping_add((((1i32) as u32)).wrapping_mul(16)))) as u64); let a_ = (((l_pkt_8).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(16); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                    { let v_ = ((c.m.r32(((fp.wrapping_add(252)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(4))) as u64); let a_ = (((l_pkt_8).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(18); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                    { let v_ = (((c.m.r32(((fp.wrapping_add(252)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)) as u64); let a_ = (((l_pkt_8).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(20); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
                    { let v_ = ((255i32) as u64); let a_ = (((l_pkt_8).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(23); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
                    { let v_ = ((c.m.r32((fp.wrapping_add(252)).wrapping_add((((2i32) as u32)).wrapping_mul(16)))) as u64); let a_ = (((l_pkt_8).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(16); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                    { let v_ = ((c.m.r32(((fp.wrapping_add(252)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(4))) as u64); let a_ = (((l_pkt_8).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(18); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                    { let v_ = (((c.m.r32(((fp.wrapping_add(252)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)) as u64); let a_ = (((l_pkt_8).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(20); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
                    { let v_ = ((255i32) as u64); let a_ = (((l_pkt_8).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(23); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
                    { let v_ = ((c.m.r32((fp.wrapping_add(252)).wrapping_add((((3i32) as u32)).wrapping_mul(16)))) as u64); let a_ = (((l_pkt_8).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(16); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                    { let v_ = ((c.m.r32(((fp.wrapping_add(252)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(4))) as u64); let a_ = (((l_pkt_8).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(18); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                    { let v_ = (((c.m.r32(((fp.wrapping_add(252)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)) as u64); let a_ = (((l_pkt_8).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(20); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
                    { let v_ = ((255i32) as u64); let a_ = (((l_pkt_8).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(23); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
                    if ((F::eft_n__EftAura_FlameFlagInl(c, l_f_6, ((2048i32) as u32))) != 0) {
                        { let v_ = c.m.r64((l_sparkTex_4).wrapping_add((((0i32) as u32)).wrapping_mul(16))); let a_ = (l_pkt_8).wrapping_add(40); c.m.w64(a_, v_); };
                    } else {
                        if ((F::eft_n__EftAura_WorkFlagInl(c, p_aura_1, ((128i32) as u32))) != 0) {
                            { let v_ = { let a_ = ((p_aura_1).wrapping_add(1232)).wrapping_add(((((((c.m.r8((l_f_6).wrapping_add(6))) as i32)).wrapping_add(1i32)) as u32)).wrapping_mul(8)); c.m.r64(a_) }; let a_ = (l_pkt_8).wrapping_add(40); c.m.w64(a_, v_); };
                        } else {
                            { let v_ = c.m.r64(((p_aura_1).wrapping_add(1232)).wrapping_add((((0i32) as u32)).wrapping_mul(8))); let a_ = (l_pkt_8).wrapping_add(40); c.m.w64(a_, v_); };
                        }
                    }
                    if ((F::BtlScene_IsStageFlagOn(c)) != 0) {
                        F::eft_n__EftAura_OtAdd(c, l_pkt_8, l_z_10, 1i32);
                    } else {
                        { let a2_ = ((c.m.r8((l_f_6).wrapping_add(11))) as i32); F::eft_n__EftAura_OtAdd(c, l_pkt_8, l_z_10, a2_) };
                    }
                }
                l_link_7 = (l_f_6).wrapping_add(192);
            }
        }
        { let _ = F::Dbg_ProfColor(c, sym::gBattleProf, 2147549183u32); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftAura_DrawFlames", &PROBE_N, &[], true); }
    r_
}

pub fn EftAura_UpdateTextures(c: &mut Ctx, mut p_aura_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftAura_UpdateTextures", &PROBE_N, &[(p_aura_1) as u32], false); }
    let mut l_set_2: u32 = 0;
    let mut l_ready_3: u32 = 0;
    let mut l_base_4: u64 = 0;
    let mut l_i_5: i32 = 0;
    let r_ = 'f: {
    l_set_2 = (((c.m.r32(sym::gEftAuraPool)).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(560))).wrapping_add(40);
    if (((((c.m.r32(p_aura_1) as i32)) & (128i32))) != 0) {
        l_base_4 = F::EftVram_AddImage(c, (l_set_2).wrapping_add((((8i32) as u32)).wrapping_mul(16)), 1i32, 0i32);
        {
            l_i_5 = (c.m.r32((p_aura_1).wrapping_add(28)) as i32);
            'l1: loop {
                if !(((((l_i_5) < (((c.m.r32((p_aura_1).wrapping_add(28)) as i32)).wrapping_add((c.m.r32((p_aura_1).wrapping_add(32)) as i32)))) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let v_ = ((l_base_4) | ((F::EftVram_AddClut(c, (l_set_2).wrapping_add((((l_i_5) as u32)).wrapping_mul(16)))).wrapping_shl((37i32) as u32))); let a_ = (((p_aura_1).wrapping_add(((8i32) as u32))).wrapping_add(1232)).wrapping_add(((((l_i_5).wrapping_sub((c.m.r32((p_aura_1).wrapping_add(28)) as i32))) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                }
                { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
            }
        }
    } else {
        { let v_ = F::EftVram_AddImage(c, (l_set_2).wrapping_add((((0i32) as u32)).wrapping_mul(16)), 1i32, 0i32); let a_ = ((p_aura_1).wrapping_add(1232)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = ({ let a0_ = (l_set_2).wrapping_add(((((c.m.r32((p_aura_1).wrapping_add(16)) as i32)) as u32)).wrapping_mul(16)); F::EftVram_AddClut(c, a0_) }).wrapping_shl((37i32) as u32); let a_ = ((p_aura_1).wrapping_add(1232)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
    }
    l_ready_3 = ((c.m.r32(sym::gEftAuraPool)).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(560));
    if ((((!(((c.m.r32(l_ready_3) as i32)) != 0)) as i32)) != 0) {
        l_set_2 = (((c.m.r32(sym::gEftAuraPool)).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(560))).wrapping_add(40);
        {
            l_i_5 = 0i32;
            'l3: loop {
                if !(((((l_i_5) < ((c.m.r32((l_set_2).wrapping_add(512)) as i32))) as i32)) != 0) { break 'l3; }
                'c4: {
                    { let v_ = F::EftVram_AddTex(c, (l_set_2).wrapping_add((((l_i_5) as u32)).wrapping_mul(16)), 1i32, 0i32); let a_ = (l_set_2).wrapping_add((((l_i_5) as u32)).wrapping_mul(16)); c.m.w64(a_, v_); };
                }
                { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
            }
        }
        { let v_ = 1i32; let a_ = l_ready_3; c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftAura_UpdateTextures", &PROBE_N, &[], true); }
    r_
}

pub fn EftAura_UpdateLightning(c: &mut Ctx, mut p_objId_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftAura_UpdateLightning", &PROBE_N, &[(p_objId_1) as u32], false); }
    let r_ = 'f: {
    if ((((F::BtlCharApi_ObjGetParamFlags0(c, p_objId_1)) & (12i32))) != 0) {
        { let _ = F::EftBolt_Enable(c, p_objId_1); };
    } else {
        { let _ = F::EftBolt_Disable(c, p_objId_1); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftAura_UpdateLightning", &PROBE_N, &[], true); }
    r_
}

pub fn eft_n__EftAura_LoadColors(c: &mut Ctx, mut p_aura_1: u32, mut p_alt_2: i32, mut p_multi_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "eft_n__EftAura_LoadColors", &PROBE_N, &[(p_aura_1) as u32, (p_alt_2) as u32, (p_multi_3) as u32], false); }
    let mut l_i_4: i32 = 0;
    let mut l_j_5: i32 = 0;
    let r_ = 'f: {
    if ((p_multi_3) != 0) {
        { let a1_ = (((c.m.r32(sym::gEftAuraCfg)).wrapping_add(3168)).wrapping_add((((p_alt_2) as u32)).wrapping_mul(208))).wrapping_add((((0i32) as u32)).wrapping_mul(16)); F::Vec4_Scale(c, (p_aura_1).wrapping_add(1008), a1_, F32(0x3b808080)) };
        { let a1_ = (((c.m.r32(sym::gEftAuraCfg)).wrapping_add(3168)).wrapping_add((((p_alt_2) as u32)).wrapping_mul(208))).wrapping_add((((0i32) as u32)).wrapping_mul(16)); F::Vec4_Scale(c, (p_aura_1).wrapping_add(1024), a1_, F32(0x3b808080)) };
        { let v_ = F32(0x00000000); let a_ = ((p_aura_1).wrapping_add(1024)).wrapping_add(12); c.m.w32(a_, (v_).0); };
    } else {
        { let a1_ = (((c.m.r32(sym::gEftAuraCfg)).wrapping_add(1296)).wrapping_add(((((c.m.r32((p_aura_1).wrapping_add(16)) as i32)) as u32)).wrapping_mul(208))).wrapping_add((((0i32) as u32)).wrapping_mul(16)); F::Vec4_Scale(c, (p_aura_1).wrapping_add(1008), a1_, F32(0x3b808080)) };
        { let a1_ = (((c.m.r32(sym::gEftAuraCfg)).wrapping_add(1296)).wrapping_add(((((c.m.r32((p_aura_1).wrapping_add(16)) as i32)) as u32)).wrapping_mul(208))).wrapping_add((((0i32) as u32)).wrapping_mul(16)); F::Vec4_Scale(c, (p_aura_1).wrapping_add(1024), a1_, F32(0x3b808080)) };
        { let v_ = F32(0x00000000); let a_ = ((p_aura_1).wrapping_add(1024)).wrapping_add(12); c.m.w32(a_, (v_).0); };
    }
    {
        l_i_4 = 0i32;
        'l1: loop {
            if !(((((l_i_4) < (4i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if (((((c.m.r32(p_aura_1) as i32)) & (128i32))) != 0) {
                    { let a1_ = (((c.m.r32(sym::gEftAuraCfg)).wrapping_add(3168)).wrapping_add((((p_alt_2) as u32)).wrapping_mul(208))).wrapping_add(((((1i32).wrapping_add(l_i_4)) as u32)).wrapping_mul(16)); F::Vec4_Scale(c, ((p_aura_1).wrapping_add(1040)).wrapping_add((((l_i_4) as u32)).wrapping_mul(16)), a1_, F32(0x3b808080)) };
                    { let a1_ = (((c.m.r32(sym::gEftAuraCfg)).wrapping_add(3168)).wrapping_add((((p_alt_2) as u32)).wrapping_mul(208))).wrapping_add(((((5i32).wrapping_add(l_i_4)) as u32)).wrapping_mul(16)); F::Vec4_Scale(c, ((p_aura_1).wrapping_add(1104)).wrapping_add((((l_i_4) as u32)).wrapping_mul(16)), a1_, F32(0x3b808080)) };
                    { let a1_ = (((c.m.r32(sym::gEftAuraCfg)).wrapping_add(3168)).wrapping_add((((p_alt_2) as u32)).wrapping_mul(208))).wrapping_add(((((9i32).wrapping_add(l_i_4)) as u32)).wrapping_mul(16)); F::Vec4_Scale(c, ((p_aura_1).wrapping_add(1168)).wrapping_add((((l_i_4) as u32)).wrapping_mul(16)), a1_, F32(0x3b808080)) };
                } else {
                    if ((((((c.m.r32((p_aura_1).wrapping_add(16)) as i32)) < (8i32)) as i32)) != 0) {
                        l_j_5 = 0i32;
                    } else {
                        l_j_5 = l_i_4;
                    }
                    { let a1_ = (((c.m.r32(sym::gEftAuraCfg)).wrapping_add(1296)).wrapping_add(((((c.m.r32((p_aura_1).wrapping_add(16)) as i32)) as u32)).wrapping_mul(208))).wrapping_add(((((1i32).wrapping_add(l_j_5)) as u32)).wrapping_mul(16)); F::Vec4_Scale(c, ((p_aura_1).wrapping_add(1040)).wrapping_add((((l_i_4) as u32)).wrapping_mul(16)), a1_, F32(0x3b808080)) };
                    { let a1_ = (((c.m.r32(sym::gEftAuraCfg)).wrapping_add(1296)).wrapping_add(((((c.m.r32((p_aura_1).wrapping_add(16)) as i32)) as u32)).wrapping_mul(208))).wrapping_add(((((5i32).wrapping_add(l_j_5)) as u32)).wrapping_mul(16)); F::Vec4_Scale(c, ((p_aura_1).wrapping_add(1104)).wrapping_add((((l_i_4) as u32)).wrapping_mul(16)), a1_, F32(0x3b808080)) };
                    { let a1_ = (((c.m.r32(sym::gEftAuraCfg)).wrapping_add(1296)).wrapping_add(((((c.m.r32((p_aura_1).wrapping_add(16)) as i32)) as u32)).wrapping_mul(208))).wrapping_add(((((9i32).wrapping_add(l_j_5)) as u32)).wrapping_mul(16)); F::Vec4_Scale(c, ((p_aura_1).wrapping_add(1168)).wrapping_add((((l_i_4) as u32)).wrapping_mul(16)), a1_, F32(0x3b808080)) };
                }
            }
            { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
        }
    }
    p_alt_2 = 0i32;
       
    };
    if rt::probing() { crate::host::harness::probe(c, "eft_n__EftAura_LoadColors", &PROBE_N, &[], true); }
    r_
}

pub fn EftAura_SetType(c: &mut Ctx, mut p_aura_1: u32, mut p_objId_2: i32, mut p_type_3: i32, mut p_paramFlags_4: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftAura_SetType", &PROBE_N, &[(p_aura_1) as u32, (p_objId_2) as u32, (p_type_3) as u32, (p_paramFlags_4) as u32], false); }
    let mut l_alt_5: i32 = 0;
    let mut l_multi_6: i32 = 0;
    let r_ = 'f: {
    l_alt_5 = 0i32;
    { let r_ = (!(128i32)); let a_ = p_aura_1; let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
    if ((((((((p_type_3).wrapping_sub(8i32)) as u32)) < (((3i32) as u32))) as i32)) != 0) {
        { let r_ = 128i32; let a_ = p_aura_1; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        { let v_ = 8i32; let a_ = (p_aura_1).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
        { let v_ = 2i32; let a_ = (p_aura_1).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
    }
    's1: {
        let sv7: i32 = p_type_3;
        match sv7 {
            8i32 => {
                { let v_ = 8i32; let a_ = (p_aura_1).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                { let v_ = 2i32; let a_ = (p_aura_1).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            9i32 => {
                { let v_ = 10i32; let a_ = (p_aura_1).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                { let v_ = 2i32; let a_ = (p_aura_1).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
                l_alt_5 = 2i32;
                break 's1;
            }
            10i32 => {
                { let v_ = 12i32; let a_ = (p_aura_1).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                { let v_ = 2i32; let a_ = (p_aura_1).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
                l_alt_5 = 3i32;
                break 's1;
            }
            _ => {
            }
        }
    }
    { let v_ = p_type_3; let a_ = (p_aura_1).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
    { let v_ = p_type_3; let a_ = (p_aura_1).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
    { let v_ = 1i32; let a_ = (p_aura_1).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
    if (((((c.m.r32(p_aura_1) as i32)) & (128i32))) != 0) {
        { let v_ = 2i32; let a_ = (p_aura_1).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
    }
    l_multi_6 = (((c.m.r32(p_aura_1) as i32)) & (128i32));
    F::eft_n__EftAura_LoadColors(c, p_aura_1, l_alt_5, l_multi_6);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftAura_SetType", &PROBE_N, &[], true); }
    r_
}

pub fn EftAura_Setup(c: &mut Ctx, mut p_aura_1: u32, mut p_objId_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftAura_Setup", &PROBE_N, &[(p_aura_1) as u32, (p_objId_2) as u32], false); }
    let mut l_flags_3: i32 = 0;
    let r_ = 'f: {
    l_flags_3 = F::BtlCharApi_ObjGetParamFlags0(c, p_objId_2);
    { let a2_ = F::BtlCharApi_GetAuraType(c, p_objId_2); F::EftAura_SetType(c, p_aura_1, p_objId_2, a2_, l_flags_3) };
    { let v_ = rt::fdiv(F::BtlCharApi_GetHeight(c, p_objId_2), F32(0x419acccc)); let a_ = (p_aura_1).wrapping_add(96); c.m.w32(a_, (v_).0); };
    if (((rt::flt(F32(c.m.r32((p_aura_1).wrapping_add(96))), F32(0x3f333333)) as i32)) != 0) {
        { let v_ = F32(0x3f333333); let a_ = (p_aura_1).wrapping_add(96); c.m.w32(a_, (v_).0); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftAura_Setup", &PROBE_N, &[], true); }
    r_
}

pub fn EftAuraTask_Init(c: &mut Ctx, mut p_task_1: u32, mut p_arg_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftAuraTask_Init", &PROBE_N, &[(p_task_1) as u32, (p_arg_2) as u32], false); }
    let mut l_aura_3: u32 = 0;
    let r_ = 'f: {
    l_aura_3 = c.m.r32((p_task_1).wrapping_add(56));
    { let _ = F::memset(c, l_aura_3, 0i32, 1296u32); };
    { let s_ = p_arg_2; let d_ = (l_aura_3).wrapping_add(100); rt::memcpy(c, d_, s_, 8); };
    { let v_ = F32(0x3f800000); let a_ = (l_aura_3).wrapping_add(72); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x00000000); let a_ = (l_aura_3).wrapping_add(56); c.m.w32(a_, (v_).0); };
    { let r_ = 8i32; let a_ = l_aura_3; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    { let v_ = 0i32; let a_ = (l_aura_3).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
    { let v_ = 0i32; let a_ = (l_aura_3).wrapping_add(52); c.m.w32(a_, (v_) as u32); };
    { let a1_ = (c.m.r32(p_arg_2) as i32); F::EftAura_Setup(c, l_aura_3, a1_) };
    F::EftAura_Start(c, l_aura_3, p_arg_2, 1i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftAuraTask_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftAuraTask_Term(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftAuraTask_Term", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_aura_2: u32 = 0;
    let mut l_objId_3: u32 = 0;
    let r_ = 'f: {
    l_aura_2 = c.m.r32((p_task_1).wrapping_add(56));
    { let v_ = 0i32; let a_ = l_aura_2; c.m.w32(a_, (v_) as u32); };
    l_objId_3 = (l_aura_2).wrapping_add(100);
    { let a0_ = (c.m.r32(l_objId_3) as i32); F::EftAura_FreeFlames(c, a0_) };
    { let a0_ = (c.m.r32(l_objId_3) as i32); F::EftAura_FreeSparks(c, a0_) };
    { let v_ = ((0i32) as u32); let a_ = ({ let a_ = (c.m.r32(sym::gEftAuraPool)).wrapping_add(1168); c.m.r32(a_) }).wrapping_add(((((c.m.r32(l_objId_3) as i32)) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftAuraTask_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftAuraTask_Update(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftAuraTask_Update", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_aura_2: u32 = 0;
    let mut l_objId_3: u32 = 0;
    let mut l_i_4: i32 = 0;
    let r_ = 'f: {
    l_aura_2 = c.m.r32((p_task_1).wrapping_add(56));
    l_objId_3 = (l_aura_2).wrapping_add(100);
    if (({ let a0_ = (c.m.r32(l_objId_3) as i32); F::BtlCharApi_IsModelNew(c, a0_) }) != 0) {
        { let r_ = 64i32; let a_ = l_aura_2; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
    { let a1_ = (c.m.r32(l_objId_3) as i32); F::EftAura_Setup(c, l_aura_2, a1_) };
    { let r_ = (!(64i32)); let a_ = l_aura_2; let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
    if ((((!(((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0)) as i32)) != 0) {
        {
            l_i_4 = 0i32;
            'l1: loop {
                if !(((((l_i_4) < (12i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let a0_ = (c.m.r32(l_objId_3) as i32); let a1_ = { let a_ = (((c.m.r32(sym::gEftAuraCfg)).wrapping_add(1184)).wrapping_add((((l_i_4) as u32)).wrapping_mul(8))).wrapping_add(4); (c.m.r32(a_) as i32) }; F::BtlCharApi_GetNodePos(c, a0_, a1_, ((l_aura_2).wrapping_add(816)).wrapping_add((((l_i_4) as u32)).wrapping_mul(16))) };
                }
                { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
            }
        }
        { let a0_ = (c.m.r32(l_objId_3) as i32); F::BtlCharApi_GetNodePos(c, a0_, 3i32, (l_aura_2).wrapping_add(112)) };
        { let a0_ = (c.m.r32(l_objId_3) as i32); F::BtlCharApi_GetNodePos(c, a0_, 48i32, (l_aura_2).wrapping_add(784)) };
        { let a0_ = (c.m.r32(l_objId_3) as i32); F::BtlCharApi_GetNodePos(c, a0_, 17i32, (l_aura_2).wrapping_add(800)) };
        {
            l_i_4 = 0i32;
            'l3: loop {
                if !(((((l_i_4) < (10i32)) as i32)) != 0) { break 'l3; }
                'c4: {
                    { let a0_ = (c.m.r32(l_objId_3) as i32); let a1_ = { let a_ = ((c.m.r32(sym::gEftAuraCfg)).wrapping_add(1040)).wrapping_add((((l_i_4) as u32)).wrapping_mul(12)); (c.m.r32(a_) as i32) }; F::BtlCharApi_GetNodePos(c, a0_, a1_, ((l_aura_2).wrapping_add(144)).wrapping_add((((l_i_4) as u32)).wrapping_mul(16))) };
                }
                { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
            }
        }
        {
            l_i_4 = 0i32;
            'l5: loop {
                if !(((((l_i_4) < (10i32)) as i32)) != 0) { break 'l5; }
                'c6: {
                    { let a0_ = (c.m.r32(l_objId_3) as i32); let a1_ = { let a_ = (((c.m.r32(sym::gEftAuraCfg)).wrapping_add(1040)).wrapping_add((((l_i_4) as u32)).wrapping_mul(12))).wrapping_add(4); (c.m.r32(a_) as i32) }; F::BtlCharApi_GetNodePos(c, a0_, a1_, ((l_aura_2).wrapping_add(464)).wrapping_add((((l_i_4) as u32)).wrapping_mul(16))) };
                }
                { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
            }
        }
        {
            l_i_4 = 0i32;
            'l7: loop {
                if !(((((l_i_4) < (10i32)) as i32)) != 0) { break 'l7; }
                'c8: {
                    { let a0_ = (c.m.r32(l_objId_3) as i32); let a1_ = { let a_ = (((c.m.r32(sym::gEftAuraCfg)).wrapping_add(1040)).wrapping_add((((l_i_4) as u32)).wrapping_mul(12))).wrapping_add(8); (c.m.r32(a_) as i32) }; F::BtlCharApi_GetNodePos(c, a0_, a1_, ((l_aura_2).wrapping_add(624)).wrapping_add((((l_i_4) as u32)).wrapping_mul(16))) };
                }
                { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
            }
        }
        if (((((c.m.r32(l_aura_2) as i32)) & (8i32))) != 0) {
            F::Vec4_Copy(c, (l_aura_2).wrapping_add(128), (l_aura_2).wrapping_add(112));
            {
                l_i_4 = 0i32;
                'l9: loop {
                    if !(((((l_i_4) < (10i32)) as i32)) != 0) { break 'l9; }
                    'c10: {
                        F::Vec4_Copy(c, ((l_aura_2).wrapping_add(304)).wrapping_add((((l_i_4) as u32)).wrapping_mul(16)), ((l_aura_2).wrapping_add(144)).wrapping_add((((l_i_4) as u32)).wrapping_mul(16)));
                    }
                    { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
                }
            }
            { let r_ = (!(8i32)); let a_ = l_aura_2; let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
        }
        { let a1_ = (c.m.r32(l_objId_3) as i32); F::EftAura_UpdateState(c, l_aura_2, a1_) };
        { let a1_ = (c.m.r32(l_objId_3) as i32); F::EftAura_UpdateSparks(c, l_aura_2, a1_, (l_aura_2).wrapping_add(1288)) };
        F::Vec4_Copy(c, (l_aura_2).wrapping_add(128), (l_aura_2).wrapping_add(112));
        {
            l_i_4 = 0i32;
            'l11: loop {
                if !(((((l_i_4) < (10i32)) as i32)) != 0) { break 'l11; }
                'c12: {
                    F::Vec4_Copy(c, ((l_aura_2).wrapping_add(304)).wrapping_add((((l_i_4) as u32)).wrapping_mul(16)), ((l_aura_2).wrapping_add(144)).wrapping_add((((l_i_4) as u32)).wrapping_mul(16)));
                }
                { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
            }
        }
        { let a_ = (l_aura_2).wrapping_add(48); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
    }
    if (((((c.m.r32(l_aura_2) as i32)) & (2i32))) != 0) {
        F::BtlTask_SetDead(c, p_task_1);
    } else {
        F::EftAura_UpdateTextures(c, l_aura_2);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftAuraTask_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftAuraTask_PostUpdate(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftAuraTask_PostUpdate", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    if (((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0) {
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftAuraTask_PostUpdate", &PROBE_N, &[], true); }
    r_
}

pub fn EftAuraTask_Reset(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftAuraTask_Reset", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    F::BtlTask_SetDead(c, p_task_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftAuraTask_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn EftAuraTask_Draw(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftAuraTask_Draw", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_alpha_2: F32 = F32(0);
    let mut l_aura_3: u32 = 0;
    let mut l_objId_4: u32 = 0;
    let r_ = 'f: {
    l_alpha_2 = F32(0x3f800000);
    l_aura_3 = c.m.r32((p_task_1).wrapping_add(56));
    l_objId_4 = (l_aura_3).wrapping_add(100);
    if (({ let a0_ = (c.m.r32(l_objId_4) as i32); F::BtlCharApi_IsHidden(c, a0_) }) != 0) {
        break 'f;
    }
    if (((((c.m.r32(l_aura_3) as i32)) & (4i32))) != 0) {
        break 'f;
    }
    if (((((({ let a0_ = (c.m.r32(l_objId_4) as i32); F::BtlCharApi_IsCamShown(c, a0_) }) != 0) && (({ let a0_ = (c.m.r32(l_objId_4) as i32); F::BtlCharApi_ObjTestFlagBit21(c, a0_) }) != 0)) as i32)) != 0) {
        l_alpha_2 = F32(0x3e999999);
    }
    if (({ let a0_ = (c.m.r32(l_objId_4) as i32); F::BtlCharApi_IsInTechnique(c, a0_) }) != 0) {
        break 'f;
    }
    if (({ let a0_ = (c.m.r32(l_objId_4) as i32); F::BtlCharApi_IsInRushSequence(c, a0_) }) != 0) {
        break 'f;
    }
    if (({ let a0_ = (c.m.r32(l_objId_4) as i32); F::EftGlow_IsActive(c, a0_) }) != 0) {
        break 'f;
    }
    if (((((c.m.r32(l_aura_3) as i32)) & (32i32))) != 0) {
        break 'f;
    }
    F::Vu0Cur_Push(c);
    { let a0_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(320); F::Vu0Cur_LoadMtx(c, a0_) };
    { let a1_ = (c.m.r32(l_objId_4) as i32); F::EftAura_DrawFlames(c, l_aura_3, a1_, l_alpha_2) };
    F::Vu0Cur_Pop(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftAuraTask_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn EftAuraMgr_Init(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftAuraMgr_Init", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
    { let v_ = { let a0_ = F::BtlPool_GetCurrent(c); F::BtlPool_Alloc(c, a0_, ((1184u32) as i32)) }; let a_ = sym::gEftAuraPool; c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32(sym::gEftAuraPool); F::memset(c, a0_, 0i32, 1184u32) }; };
    { let v_ = F::BtlScene_GetCharCount(c); let a_ = c.m.r32(sym::gEftAuraPool); c.m.w32(a_, (v_) as u32); };
    { let v_ = ({ let a_ = c.m.r32(sym::gEftAuraPool); (c.m.r32(a_) as i32) }).wrapping_mul(50i32); let a_ = (c.m.r32(sym::gEftAuraPool)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
    { let v_ = ({ let a_ = c.m.r32(sym::gEftAuraPool); (c.m.r32(a_) as i32) }).wrapping_mul(18i32); let a_ = (c.m.r32(sym::gEftAuraPool)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
    { let v_ = ((0i32) as u32); let a_ = (c.m.r32(sym::gEftAuraPool)).wrapping_add(16); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (c.m.r32(sym::gEftAuraPool)).wrapping_add(20); c.m.w32(a_, v_); };
    { let v_ = 0i32; let a_ = (c.m.r32(sym::gEftAuraPool)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
    { let v_ = { let a0_ = F::BtlPool_GetCurrent(c); let a1_ = ((((({ let a_ = (c.m.r32(sym::gEftAuraPool)).wrapping_add(4); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(208u32)) as i32); F::BtlPool_Alloc(c, a0_, a1_) }; let a_ = (c.m.r32(sym::gEftAuraPool)).wrapping_add(12); c.m.w32(a_, v_); };
    { let _ = { let a0_ = { let a_ = (c.m.r32(sym::gEftAuraPool)).wrapping_add(12); c.m.r32(a_) }; let a2_ = ((({ let a_ = (c.m.r32(sym::gEftAuraPool)).wrapping_add(4); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(208u32); F::memset(c, a0_, 0i32, a2_) }; };
    { let v_ = ((0i32) as u32); let a_ = (c.m.r32(sym::gEftAuraPool)).wrapping_add(28); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (c.m.r32(sym::gEftAuraPool)).wrapping_add(32); c.m.w32(a_, v_); };
    { let v_ = 0i32; let a_ = (c.m.r32(sym::gEftAuraPool)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
    { let v_ = { let a0_ = F::BtlPool_GetCurrent(c); let a1_ = ((((({ let a_ = (c.m.r32(sym::gEftAuraPool)).wrapping_add(8); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(176u32)) as i32); F::BtlPool_Alloc(c, a0_, a1_) }; let a_ = (c.m.r32(sym::gEftAuraPool)).wrapping_add(24); c.m.w32(a_, v_); };
    { let _ = { let a0_ = { let a_ = (c.m.r32(sym::gEftAuraPool)).wrapping_add(24); c.m.r32(a_) }; let a2_ = ((({ let a_ = (c.m.r32(sym::gEftAuraPool)).wrapping_add(8); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(176u32); F::memset(c, a0_, 0i32, a2_) }; };
    { let v_ = { let a0_ = F::BtlPool_GetCurrent(c); let a1_ = ({ let a_ = c.m.r32(sym::gEftAuraPool); (c.m.r32(a_) as i32) }).wrapping_mul(4i32); F::BtlPool_Alloc(c, a0_, a1_) }; let a_ = (c.m.r32(sym::gEftAuraPool)).wrapping_add(1168); c.m.w32(a_, v_); };
    { let _ = { let a0_ = { let a_ = (c.m.r32(sym::gEftAuraPool)).wrapping_add(1168); c.m.r32(a_) }; let a2_ = ((({ let a_ = c.m.r32(sym::gEftAuraPool); (c.m.r32(a_) as i32) }).wrapping_mul(4i32)) as u32); F::memset(c, a0_, 0i32, a2_) }; };
    { let v_ = F::BtlScene_GetCommonEntry(c, 3i32); let a_ = (((c.m.r32(sym::gEftAuraPool)).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(560))).wrapping_add(32); c.m.w32(a_, v_); };
    { let v_ = F::BtlScene_GetCommonEntry(c, 2i32); let a_ = (((c.m.r32(sym::gEftAuraPool)).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(560))).wrapping_add(32); c.m.w32(a_, v_); };
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < (2i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let a0_ = (((c.m.r32(sym::gEftAuraPool)).wrapping_add(48)).wrapping_add((((l_i_2) as u32)).wrapping_mul(560))).wrapping_add(40); let a1_ = { let a_ = (((c.m.r32(sym::gEftAuraPool)).wrapping_add(48)).wrapping_add((((l_i_2) as u32)).wrapping_mul(560))).wrapping_add(32); c.m.r32(a_) }; F::EftTexSet_Load32(c, a0_, a1_) };
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
    { let v_ = F::BtlScene_GetCommonEntry(c, 1i32); let a_ = (c.m.r32(sym::gEftAuraPool)).wrapping_add(1180); c.m.w32(a_, v_); };
    { let v_ = { let v_ = { let a_ = (c.m.r32(sym::gEftAuraPool)).wrapping_add(1180); c.m.r32(a_) }; let a_ = sym::gEftAuraCfg; c.m.w32(a_, v_); v_ }; let a_ = sym::gEftAuraPrm; c.m.w32(a_, v_); };
    {
        l_i_2 = 0i32;
        'l3: loop {
            if !(((((l_i_2) < ({ let a_ = c.m.r32(sym::gEftAuraPool); (c.m.r32(a_) as i32) })) as i32)) != 0) { break 'l3; }
            'c4: {
                { let v_ = ((0i32) as u32); let a_ = ({ let a_ = (c.m.r32(sym::gEftAuraPool)).wrapping_add(1168); c.m.r32(a_) }).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
    { let v_ = { let a1_ = { let a_ = c.m.r32(sym::gEftAuraPool); (c.m.r32(a_) as i32) }; F::BtlTask_CreateChildList(c, p_task_1, ((((a1_) as i64 as u64)) as u32), ((((((1296u32) as i32)) as i64 as u64)) as u32)) }; let a_ = sym::gEftAuraTaskList; c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftAuraMgr_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftAuraMgr_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftAuraMgr_Term", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let a0_ = F::BtlPool_GetCurrent(c); let a1_ = { let a_ = (c.m.r32(sym::gEftAuraPool)).wrapping_add(1168); c.m.r32(a_) }; F::BtlPool_Free(c, a0_, a1_) };
    { let a0_ = F::BtlPool_GetCurrent(c); let a1_ = { let a_ = (c.m.r32(sym::gEftAuraPool)).wrapping_add(12); c.m.r32(a_) }; F::BtlPool_Free(c, a0_, a1_) };
    { let a0_ = F::BtlPool_GetCurrent(c); let a1_ = { let a_ = (c.m.r32(sym::gEftAuraPool)).wrapping_add(24); c.m.r32(a_) }; F::BtlPool_Free(c, a0_, a1_) };
    { let a0_ = F::BtlPool_GetCurrent(c); let a1_ = c.m.r32(sym::gEftAuraPool); F::BtlPool_Free(c, a0_, a1_) };
    { let v_ = ((0i32) as u32); let a_ = sym::gEftAuraPool; c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftAuraMgr_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftAuraMgr_Update(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftAuraMgr_Update", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let r_ = 'f: {
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (2i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let v_ = 0i32; let a_ = ((c.m.r32(sym::gEftAuraPool)).wrapping_add(48)).wrapping_add((((l_i_1) as u32)).wrapping_mul(560)); c.m.w32(a_, (v_) as u32); };
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
    if ((((!((F::BtlStage_IsReady(c)) != 0)) as i32)) != 0) {
        { let v_ = 1i32; let a_ = (c.m.r32(sym::gEftAuraPool)).wrapping_add(1176); c.m.w32(a_, (v_) as u32); };
    }
    if (({ let a_ = (c.m.r32(sym::gEftAuraPool)).wrapping_add(1176); (c.m.r32(a_) as i32) }) != 0) {
        if ((F::BtlStage_IsReady(c)) != 0) {
            { let v_ = 0i32; let a_ = (c.m.r32(sym::gEftAuraPool)).wrapping_add(1172); c.m.w32(a_, (v_) as u32); };
            { let v_ = 0i32; let a_ = (c.m.r32(sym::gEftAuraPool)).wrapping_add(1176); c.m.w32(a_, (v_) as u32); };
        }
    }
    {
        l_i_1 = 0i32;
        'l3: loop {
            if !(((((l_i_1) < ({ let a_ = c.m.r32(sym::gEftAuraPool); (c.m.r32(a_) as i32) })) as i32)) != 0) { break 'l3; }
            'c4: {
                if ((((((((!(({ let a_ = (c.m.r32(sym::gEftAuraPool)).wrapping_add(1172); (c.m.r32(a_) as i32) }) != 0)) as i32)) != 0) || ((F::BtlCharApi_IsModelNew(c, l_i_1)) != 0)) as i32)) != 0) {
                    F::EftAura_UpdateLightning(c, l_i_1);
                }
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
    { let v_ = 1i32; let a_ = (c.m.r32(sym::gEftAuraPool)).wrapping_add(1172); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftAuraMgr_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftAuraMgr_Stub(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftAuraMgr_Stub", &PROBE_N, &[], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftAuraMgr_Stub", &PROBE_N, &[], true); }
    r_
}

pub fn EftAura_Request(c: &mut Ctx, mut p_objId_1: i32, mut p_variant_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftAura_Request", &PROBE_N, &[(p_objId_1) as u32, (p_variant_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_task_3: u32 = 0;
    let mut l_aura_4: u32 = 0;
    let r_ = 'f: {
        if ((((({ let a_ = ({ let a_ = (c.m.r32(sym::gEftAuraPool)).wrapping_add(1168); c.m.r32(a_) }).wrapping_add((((p_objId_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }) == (((0i32) as u32))) as i32)) != 0) {
            { let v_ = p_objId_1; let a_ = fp; c.m.w32(a_, (v_) as u32); };
            { let v_ = p_variant_2; let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
            l_task_3 = { let a0_ = c.m.r32(sym::gEftAuraTaskList); F::BtlTaskList_AddTail(c, a0_, sym::gEftAuraTaskClass, fp) };
            if (((((l_task_3) != (((0i32) as u32))) as i32)) != 0) {
                { let v_ = l_task_3; let a_ = ({ let a_ = (c.m.r32(sym::gEftAuraPool)).wrapping_add(1168); c.m.r32(a_) }).wrapping_add((((p_objId_1) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
            } else {
                break 'f 0i32;
            }
        } else {
            l_aura_4 = { let a_ = ({ let a_ = ({ let a_ = (c.m.r32(sym::gEftAuraPool)).wrapping_add(1168); c.m.r32(a_) }).wrapping_add((((p_objId_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(56); c.m.r32(a_) };
            { let v_ = p_variant_2; let a_ = ((l_aura_4).wrapping_add(100)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
            F::EftAura_Start(c, l_aura_4, (l_aura_4).wrapping_add(100), 0i32);
        }
        break 'f 1i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftAura_Request", &PROBE_N, &[], true); }
    r_
}

pub fn EftAura_Stop(c: &mut Ctx, mut p_objId_1: i32, mut p_variant_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftAura_Stop", &PROBE_N, &[(p_objId_1) as u32, (p_variant_2) as u32], false); }
    let mut l_task_3: u32 = 0;
    let mut l_aura_4: u32 = 0;
    let r_ = 'f: {
    l_task_3 = { let a_ = ({ let a_ = (c.m.r32(sym::gEftAuraPool)).wrapping_add(1168); c.m.r32(a_) }).wrapping_add((((p_objId_1) as u32)).wrapping_mul(4)); c.m.r32(a_) };
    if (((((l_task_3) != (((0i32) as u32))) as i32)) != 0) {
        l_aura_4 = c.m.r32((l_task_3).wrapping_add(56));
        if ((((!(((((c.m.r32(l_aura_4) as i32)) & (1i32))) != 0)) as i32)) != 0) {
            break 'f 0i32;
        }
        if ((((((c.m.r32((l_aura_4).wrapping_add(40)) as i32)) == (3i32)) as i32)) != 0) {
            break 'f 0i32;
        }
        if (((((p_variant_2) == (0i32)) as i32)) != 0) {
            if ((((!(((((c.m.r32(l_aura_4) as i32)) & (32i32))) != 0)) as i32)) != 0) {
                { let v_ = 3i32; let a_ = (l_aura_4).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (l_aura_4).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
                { let v_ = F32(0x00000000); let a_ = (l_aura_4).wrapping_add(64); c.m.w32(a_, (v_).0); };
            }
        } else {
            if (((((c.m.r32(l_aura_4) as i32)) & (32i32))) != 0) {
                { let r_ = (!(32i32)); let a_ = l_aura_4; let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
            }
        }
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftAura_Stop", &PROBE_N, &[], true); }
    r_
}

pub fn EftAura_Kill(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftAura_Kill", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_task_2: u32 = 0;
    let mut l_aura_3: u32 = 0;
    let r_ = 'f: {
    l_task_2 = { let a_ = ({ let a_ = (c.m.r32(sym::gEftAuraPool)).wrapping_add(1168); c.m.r32(a_) }).wrapping_add((((p_objId_1) as u32)).wrapping_mul(4)); c.m.r32(a_) };
    if (((((l_task_2) != (((0i32) as u32))) as i32)) != 0) {
        l_aura_3 = c.m.r32((l_task_2).wrapping_add(56));
        if ((((!(((((c.m.r32(l_aura_3) as i32)) & (1i32))) != 0)) as i32)) != 0) {
            break 'f 0i32;
        }
        { let v_ = 2i32; let a_ = l_aura_3; c.m.w32(a_, (v_) as u32); };
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftAura_Kill", &PROBE_N, &[], true); }
    r_
}

pub fn EftAura_SetLevel(c: &mut Ctx, mut p_objId_1: i32, mut p_level_2: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftAura_SetLevel", &PROBE_N, &[(p_objId_1) as u32, (p_level_2).0], false); }
    let mut l_task_3: u32 = 0;
    let r_ = 'f: {
    if (((((c.m.r32(sym::gEftAuraPool)) != (((0i32) as u32))) as i32)) != 0) {
        l_task_3 = { let a_ = ({ let a_ = (c.m.r32(sym::gEftAuraPool)).wrapping_add(1168); c.m.r32(a_) }).wrapping_add((((p_objId_1) as u32)).wrapping_mul(4)); c.m.r32(a_) };
        if (((((l_task_3) != (((0i32) as u32))) as i32)) != 0) {
            { let v_ = p_level_2; let a_ = (c.m.r32((l_task_3).wrapping_add(56))).wrapping_add(72); c.m.w32(a_, (v_).0); };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftAura_SetLevel", &PROBE_N, &[], true); }
    r_
}

pub fn EftAura_IsVisible(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftAura_IsVisible", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_ret_2: i32 = 0;
    let mut l_task_3: u32 = 0;
    let mut l_aura_4: u32 = 0;
    let r_ = 'f: {
    l_ret_2 = 0i32;
    if (((((c.m.r32(sym::gEftAuraPool)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    l_task_3 = { let a_ = ({ let a_ = (c.m.r32(sym::gEftAuraPool)).wrapping_add(1168); c.m.r32(a_) }).wrapping_add((((p_objId_1) as u32)).wrapping_mul(4)); c.m.r32(a_) };
    if (((((l_task_3) != (((0i32) as u32))) as i32)) != 0) {
        l_aura_4 = c.m.r32((l_task_3).wrapping_add(56));
        if (((((c.m.r32(l_aura_4) as i32)) & (1i32))) != 0) {
            if ((((!(((rt::fle(F32(c.m.r32((l_aura_4).wrapping_add(56))), F32(0x00000000)) as i32)) != 0)) as i32)) != 0) {
                l_ret_2 = 1i32;
            }
        }
    }
    break 'f l_ret_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftAura_IsVisible", &PROBE_N, &[], true); }
    r_
}

pub fn EftAura_IsBurstRising(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftAura_IsBurstRising", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_ret_2: i32 = 0;
    let mut l_task_3: u32 = 0;
    let mut l_aura_4: u32 = 0;
    let r_ = 'f: {
    l_ret_2 = 0i32;
    if (((((c.m.r32(sym::gEftAuraPool)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    l_task_3 = { let a_ = ({ let a_ = (c.m.r32(sym::gEftAuraPool)).wrapping_add(1168); c.m.r32(a_) }).wrapping_add((((p_objId_1) as u32)).wrapping_mul(4)); c.m.r32(a_) };
    if (((((l_task_3) != (((0i32) as u32))) as i32)) != 0) {
        l_aura_4 = c.m.r32((l_task_3).wrapping_add(56));
        if (((((c.m.r32(l_aura_4) as i32)) & (1i32))) != 0) {
            if ((((!(((rt::fle(F32(c.m.r32((l_aura_4).wrapping_add(56))), F32(0x00000000)) as i32)) != 0)) as i32)) != 0) {
                l_ret_2 = ((((c.m.r32((l_aura_4).wrapping_add(44)) as i32)) == (1i32)) as i32);
            }
        }
    }
    break 'f l_ret_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftAura_IsBurstRising", &PROBE_N, &[], true); }
    r_
}

pub fn EftAura_StartBurst(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftAura_StartBurst", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_task_2: u32 = 0;
    let mut l_aura_3: u32 = 0;
    let r_ = 'f: {
    l_task_2 = { let a_ = ({ let a_ = (c.m.r32(sym::gEftAuraPool)).wrapping_add(1168); c.m.r32(a_) }).wrapping_add((((p_objId_1) as u32)).wrapping_mul(4)); c.m.r32(a_) };
    if (((((l_task_2) != (((0i32) as u32))) as i32)) != 0) {
        l_aura_3 = c.m.r32((l_task_2).wrapping_add(56));
        if ((((!(((((c.m.r32(l_aura_3) as i32)) & (1i32))) != 0)) as i32)) != 0) {
            break 'f 0i32;
        }
        { let v_ = F32(0x00000000); let a_ = (l_aura_3).wrapping_add(64); c.m.w32(a_, (v_).0); };
        { let v_ = 1i32; let a_ = (l_aura_3).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftAura_StartBurst", &PROBE_N, &[], true); }
    r_
}

pub fn EftAura_EndBurst(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftAura_EndBurst", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_task_2: u32 = 0;
    let mut l_aura_3: u32 = 0;
    let r_ = 'f: {
    l_task_2 = { let a_ = ({ let a_ = (c.m.r32(sym::gEftAuraPool)).wrapping_add(1168); c.m.r32(a_) }).wrapping_add((((p_objId_1) as u32)).wrapping_mul(4)); c.m.r32(a_) };
    if (((((l_task_2) != (((0i32) as u32))) as i32)) != 0) {
        l_aura_3 = c.m.r32((l_task_2).wrapping_add(56));
        if ((((!(((((c.m.r32(l_aura_3) as i32)) & (1i32))) != 0)) as i32)) != 0) {
            break 'f 0i32;
        }
        { let v_ = F32(0x00000000); let a_ = (l_aura_3).wrapping_add(64); c.m.w32(a_, (v_).0); };
        { let v_ = 4i32; let a_ = (l_aura_3).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftAura_EndBurst", &PROBE_N, &[], true); }
    r_
}

pub fn EftAura_PeakBurst(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftAura_PeakBurst", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_task_2: u32 = 0;
    let mut l_aura_3: u32 = 0;
    let r_ = 'f: {
    l_task_2 = { let a_ = ({ let a_ = (c.m.r32(sym::gEftAuraPool)).wrapping_add(1168); c.m.r32(a_) }).wrapping_add((((p_objId_1) as u32)).wrapping_mul(4)); c.m.r32(a_) };
    if (((((l_task_2) != (((0i32) as u32))) as i32)) != 0) {
        l_aura_3 = c.m.r32((l_task_2).wrapping_add(56));
        if ((((!(((((c.m.r32(l_aura_3) as i32)) & (1i32))) != 0)) as i32)) != 0) {
            break 'f 0i32;
        }
        { let v_ = F32(0x3f999999); let a_ = (l_aura_3).wrapping_add(68); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = (l_aura_3).wrapping_add(76); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (l_aura_3).wrapping_add(64); c.m.w32(a_, (v_).0); };
        { let v_ = 2i32; let a_ = (l_aura_3).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftAura_PeakBurst", &PROBE_N, &[], true); }
    r_
}

pub fn EftAura_FadeIn(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftAura_FadeIn", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_task_2: u32 = 0;
    let mut l_aura_3: u32 = 0;
    let r_ = 'f: {
    l_task_2 = { let a_ = ({ let a_ = (c.m.r32(sym::gEftAuraPool)).wrapping_add(1168); c.m.r32(a_) }).wrapping_add((((p_objId_1) as u32)).wrapping_mul(4)); c.m.r32(a_) };
    if (((((l_task_2) != (((0i32) as u32))) as i32)) != 0) {
        l_aura_3 = c.m.r32((l_task_2).wrapping_add(56));
        if ((((!(((((c.m.r32(l_aura_3) as i32)) & (1i32))) != 0)) as i32)) != 0) {
            break 'f 0i32;
        }
        if ((((((c.m.r32((l_aura_3).wrapping_add(40)) as i32)) == (3i32)) as i32)) != 0) {
            break 'f 0i32;
        }
        { let v_ = 1i32; let a_ = (l_aura_3).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftAura_FadeIn", &PROBE_N, &[], true); }
    r_
}

pub fn EftAura_FadeOut(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftAura_FadeOut", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_task_2: u32 = 0;
    let mut l_aura_3: u32 = 0;
    let r_ = 'f: {
    l_task_2 = { let a_ = ({ let a_ = (c.m.r32(sym::gEftAuraPool)).wrapping_add(1168); c.m.r32(a_) }).wrapping_add((((p_objId_1) as u32)).wrapping_mul(4)); c.m.r32(a_) };
    if (((((l_task_2) != (((0i32) as u32))) as i32)) != 0) {
        l_aura_3 = c.m.r32((l_task_2).wrapping_add(56));
        if ((((!(((((c.m.r32(l_aura_3) as i32)) & (1i32))) != 0)) as i32)) != 0) {
            break 'f 0i32;
        }
        if ((((((c.m.r32((l_aura_3).wrapping_add(40)) as i32)) == (3i32)) as i32)) != 0) {
            break 'f 0i32;
        }
        { let v_ = 2i32; let a_ = (l_aura_3).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftAura_FadeOut", &PROBE_N, &[], true); }
    r_
}

pub fn EftAura_Command(c: &mut Ctx, mut p_objId_1: i32, mut p_cmd_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftAura_Command", &PROBE_N, &[(p_objId_1) as u32, (p_cmd_2) as u32], false); }
    let r_ = 'f: {
    if (((((c.m.r32(sym::gEftAuraPool)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    's1: {
        let sv3: i32 = p_cmd_2;
        match sv3 {
            0i32 => {
                { let _ = F::EftAura_Request(c, p_objId_1, 0i32); };
                break 's1;
            }
            1i32 => {
                { let _ = F::EftAura_Stop(c, p_objId_1, 0i32); };
                break 's1;
            }
            2i32 => {
                { let _ = F::EftAura_StartBurst(c, p_objId_1); };
                break 's1;
            }
            3i32 => {
                { let _ = F::EftAura_EndBurst(c, p_objId_1); };
                break 's1;
            }
            4i32 => {
                { let _ = F::EftAura_PeakBurst(c, p_objId_1); };
                break 's1;
            }
            5i32 => {
                { let _ = F::EftAura_FadeIn(c, p_objId_1); };
                break 's1;
            }
            6i32 => {
                { let _ = F::EftAura_FadeOut(c, p_objId_1); };
                break 's1;
            }
            7i32 => {
                { let _ = F::EftAura_Kill(c, p_objId_1); };
                break 's1;
            }
            _ => {
            }
        }
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftAura_Command", &PROBE_N, &[], true); }
    r_
}

pub fn EftAura_Show(c: &mut Ctx, mut p_objId_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftAura_Show", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_task_2: u32 = 0;
    let mut l_aura_3: u32 = 0;
    let r_ = 'f: {
    if (((((((((c.m.r32(sym::gEftAuraPool)) != (((0i32) as u32))) as i32)) != 0) && (((((p_objId_1) < (2i32)) as i32)) != 0)) as i32)) != 0) {
        l_task_2 = { let a_ = ({ let a_ = (c.m.r32(sym::gEftAuraPool)).wrapping_add(1168); c.m.r32(a_) }).wrapping_add((((p_objId_1) as u32)).wrapping_mul(4)); c.m.r32(a_) };
        if (((((l_task_2) != (((0i32) as u32))) as i32)) != 0) {
            l_aura_3 = c.m.r32((l_task_2).wrapping_add(56));
            if (((((c.m.r32(l_aura_3) as i32)) & (1i32))) != 0) {
                { let r_ = (!(4i32)); let a_ = l_aura_3; let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftAura_Show", &PROBE_N, &[], true); }
    r_
}

pub fn EftAura_Hide(c: &mut Ctx, mut p_objId_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftAura_Hide", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_task_2: u32 = 0;
    let mut l_aura_3: u32 = 0;
    let r_ = 'f: {
    if (((((((((c.m.r32(sym::gEftAuraPool)) != (((0i32) as u32))) as i32)) != 0) && (((((p_objId_1) < (2i32)) as i32)) != 0)) as i32)) != 0) {
        l_task_2 = { let a_ = ({ let a_ = (c.m.r32(sym::gEftAuraPool)).wrapping_add(1168); c.m.r32(a_) }).wrapping_add((((p_objId_1) as u32)).wrapping_mul(4)); c.m.r32(a_) };
        if (((((l_task_2) != (((0i32) as u32))) as i32)) != 0) {
            l_aura_3 = c.m.r32((l_task_2).wrapping_add(56));
            if (((((c.m.r32(l_aura_3) as i32)) & (1i32))) != 0) {
                { let r_ = 4i32; let a_ = l_aura_3; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftAura_Hide", &PROBE_N, &[], true); }
    r_
}

pub fn EftAura_MarkModelNew(c: &mut Ctx, mut p_objId_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftAura_MarkModelNew", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_task_2: u32 = 0;
    let mut l_aura_3: u32 = 0;
    let r_ = 'f: {
    if (((((((((c.m.r32(sym::gEftAuraPool)) != (((0i32) as u32))) as i32)) != 0) && (((((p_objId_1) < (2i32)) as i32)) != 0)) as i32)) != 0) {
        l_task_2 = { let a_ = ({ let a_ = (c.m.r32(sym::gEftAuraPool)).wrapping_add(1168); c.m.r32(a_) }).wrapping_add((((p_objId_1) as u32)).wrapping_mul(4)); c.m.r32(a_) };
        if (((((l_task_2) != (((0i32) as u32))) as i32)) != 0) {
            l_aura_3 = c.m.r32((l_task_2).wrapping_add(56));
            if (((((c.m.r32(l_aura_3) as i32)) & (1i32))) != 0) {
                { let r_ = 64i32; let a_ = l_aura_3; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftAura_MarkModelNew", &PROBE_N, &[], true); }
    r_
}

pub fn EftAura_ChangeType(c: &mut Ctx, mut p_objId_1: i32, mut p_type_2: i32, mut p_paramFlags_3: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftAura_ChangeType", &PROBE_N, &[(p_objId_1) as u32, (p_type_2) as u32, (p_paramFlags_3) as u32], false); }
    let mut l_task_4: u32 = 0;
    let r_ = 'f: {
    if (((((p_objId_1) < (2i32)) as i32)) != 0) {
        if (((((((((c.m.r32(sym::gEftAuraPool)) != (((0i32) as u32))) as i32)) != 0) && (((((p_type_2) >= (0i32)) as i32)) != 0)) as i32)) != 0) {
            l_task_4 = { let a_ = ({ let a_ = (c.m.r32(sym::gEftAuraPool)).wrapping_add(1168); c.m.r32(a_) }).wrapping_add((((p_objId_1) as u32)).wrapping_mul(4)); c.m.r32(a_) };
            if (((((l_task_4) != (((0i32) as u32))) as i32)) != 0) {
                { let a0_ = c.m.r32((l_task_4).wrapping_add(56)); F::EftAura_SetType(c, a0_, p_objId_1, p_type_2, p_paramFlags_3) };
            }
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftAura_ChangeType", &PROBE_N, &[], true); }
    r_
}

pub fn EftAura_GetColors(c: &mut Ctx, mut p_type_1: i32, mut p_paramFlags_2: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftAura_GetColors", &PROBE_N, &[(p_type_1) as u32, (p_paramFlags_2) as u32], false); }
    let r_ = 'f: {
    if (((((c.m.r32(sym::gEftAuraPool)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f ((0i32) as u32);
    }
    if (((((p_type_1) < (0i32)) as i32)) != 0) {
        break 'f ((0i32) as u32);
    }
    if ((((!((((p_paramFlags_2) & (94i32))) != 0)) as i32)) != 0) {
        if (((((p_type_1) >= (9i32)) as i32)) != 0) {
            break 'f ((0i32) as u32);
        }
        break 'f ((c.m.r32(sym::gEftAuraCfg)).wrapping_add(1296)).wrapping_add((((p_type_1) as u32)).wrapping_mul(208));
    }
    if (((((p_type_1) >= (5i32)) as i32)) != 0) {
        break 'f ((0i32) as u32);
    }
    break 'f ((c.m.r32(sym::gEftAuraCfg)).wrapping_add(3168)).wrapping_add((((p_type_1) as u32)).wrapping_mul(208));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftAura_GetColors", &PROBE_N, &[], true); }
    r_
}

pub fn EftAura_Finish(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftAura_Finish", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_task_2: u32 = 0;
    let r_ = 'f: {
    l_task_2 = { let a_ = ({ let a_ = (c.m.r32(sym::gEftAuraPool)).wrapping_add(1168); c.m.r32(a_) }).wrapping_add((((p_objId_1) as u32)).wrapping_mul(4)); c.m.r32(a_) };
    if (((((l_task_2) != (((0i32) as u32))) as i32)) != 0) {
        { let r_ = 2i32; let a_ = c.m.r32((l_task_2).wrapping_add(56)); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftAura_Finish", &PROBE_N, &[], true); }
    r_
}

pub fn EftAura_CommandAlt(c: &mut Ctx, mut p_objId_1: i32, mut p_cmd_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftAura_CommandAlt", &PROBE_N, &[(p_objId_1) as u32, (p_cmd_2) as u32], false); }
    let r_ = 'f: {
    's1: {
        let sv3: i32 = p_cmd_2;
        match sv3 {
            0i32 => {
                { let _ = F::EftAura_Request(c, p_objId_1, 1i32); };
                break 's1;
            }
            1i32 => {
                { let _ = F::EftAura_Stop(c, p_objId_1, p_cmd_2); };
                break 's1;
            }
            2i32 => {
                { let _ = F::EftAura_Finish(c, p_objId_1); };
                break 's1;
            }
            _ => {
            }
        }
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftAura_CommandAlt", &PROBE_N, &[], true); }
    r_
}

pub fn EftBolt_AllocSegs(c: &mut Ctx, mut p_bolt_1: u32, mut p_count_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBolt_AllocSegs", &PROBE_N, &[(p_bolt_1) as u32, (p_count_2) as u32], false); }
    let mut l_prev_3: u32 = 0;
    let mut l_first_4: i32 = 0;
    let mut l_tries_5: i32 = 0;
    let mut l_got_6: i32 = 0;
    let mut l_idx_7: i32 = 0;
    let mut l_seg_8: u32 = 0;
    let r_ = 'f: {
    l_prev_3 = ((0i32) as u32);
    l_first_4 = 1i32;
    l_got_6 = 0i32;
    l_idx_7 = { let a_ = (c.m.r32(sym::gEftBoltPool)).wrapping_add(12); (c.m.r32(a_) as i32) };
    {
        l_tries_5 = 0i32;
        'l1: loop {
            if !(((((l_tries_5) < ({ let a_ = (c.m.r32(sym::gEftBoltPool)).wrapping_add(4); (c.m.r32(a_) as i32) })) as i32)) != 0) { break 'l1; }
            'c2: {
                l_seg_8 = ((((l_idx_7) as u32)).wrapping_mul(64u32)).wrapping_add({ let a_ = (c.m.r32(sym::gEftBoltPool)).wrapping_add(8); c.m.r32(a_) });
                if ((((((c.m.r32(l_seg_8) as i32)) == (0i32)) as i32)) != 0) {
                    { let v_ = ((0i32) as u32); let a_ = (l_seg_8).wrapping_add(48); c.m.w32(a_, v_); };
                    if ((l_first_4) != 0) {
                        { let v_ = l_seg_8; let a_ = p_bolt_1; c.m.w32(a_, v_); };
                        l_first_4 = 0i32;
                    } else {
                        { let v_ = l_seg_8; let a_ = (l_prev_3).wrapping_add(48); c.m.w32(a_, v_); };
                    }
                    l_prev_3 = l_seg_8;
                    { let v_ = (l_idx_7).wrapping_add(1i32); let a_ = (c.m.r32(sym::gEftBoltPool)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                    if ((((!((((({ let a_ = (c.m.r32(sym::gEftBoltPool)).wrapping_add(12); (c.m.r32(a_) as i32) }) < ({ let a_ = (c.m.r32(sym::gEftBoltPool)).wrapping_add(4); (c.m.r32(a_) as i32) })) as i32)) != 0)) as i32)) != 0) {
                        { let v_ = 0i32; let a_ = (c.m.r32(sym::gEftBoltPool)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                    }
                    { let o_ = l_got_6; let n_ = (o_).wrapping_add(1); l_got_6 = n_; };
                    if (((((l_got_6) >= (p_count_2)) as i32)) != 0) {
                        break 'l1;
                    }
                }
                { let o_ = l_idx_7; let n_ = (o_).wrapping_add(1); l_idx_7 = n_; };
                if (((((l_idx_7) >= ({ let a_ = (c.m.r32(sym::gEftBoltPool)).wrapping_add(4); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                    l_idx_7 = 0i32;
                }
            }
            { let o_ = l_tries_5; let n_ = (o_).wrapping_add(1); l_tries_5 = n_; };
        }
    }
    { let v_ = l_got_6; let a_ = (p_bolt_1).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
    { let v_ = 0i32; let a_ = (p_bolt_1).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBolt_AllocSegs", &PROBE_N, &[], true); }
    r_
}

pub fn EftBolt_FreeSegs(c: &mut Ctx, mut p_bolt_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBolt_FreeSegs", &PROBE_N, &[(p_bolt_1) as u32], false); }
    let mut l_seg_2: u32 = 0;
    let mut l_next_3: u32 = 0;
    let r_ = 'f: {
    if (((((c.m.r32(p_bolt_1)) != (((0i32) as u32))) as i32)) != 0) {
        l_seg_2 = c.m.r32(p_bolt_1);
        'l1: loop {
            'c2: {
                l_next_3 = c.m.r32((l_seg_2).wrapping_add(48));
                { let v_ = 0i32; let a_ = l_seg_2; c.m.w32(a_, (v_) as u32); };
                l_seg_2 = l_next_3;
            }
            if !(((((l_next_3) != (((0i32) as u32))) as i32)) != 0) { break 'l1; }
        }
    }
    { let v_ = ((0i32) as u32); let a_ = p_bolt_1; c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBolt_FreeSegs", &PROBE_N, &[], true); }
    r_
}

pub fn EftBolt_Shape(c: &mut Ctx, mut p_bolt_1: u32, p_start_2: u32, p_end_3: u32, mut p_scale_4: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBolt_Shape", &PROBE_N, &[(p_bolt_1) as u32, (p_start_2) as u32, (p_end_3) as u32, (p_scale_4).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(144) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 144);
    let mut l_amp_5: F32 = F32(0);
    let mut l_bright_6: F32 = F32(0);
    let mut l_r1_7: F32 = F32(0);
    let mut l_r2_8: F32 = F32(0);
    let mut l_t_9: F32 = F32(0);
    let mut l_count_10: i32 = 0;
    let mut l_thick_11: i32 = 0;
    let mut l_n_12: i32 = 0;
    let mut l_seg_13: u32 = 0;
    let mut l_link_14: u32 = 0;
    rt::memcpy(c, fp, p_start_2, 16);
    rt::memcpy(c, fp.wrapping_add(16), p_end_3, 16);
    let r_ = 'f: {
        l_amp_5 = F32(0x3f800000);
        l_thick_11 = 1i32;
        l_n_12 = 0i32;
        l_r1_7 = rt::i2f(F::Port_Rand(c));
        l_r2_8 = rt::i2f(F::Port_Rand(c));
        l_count_10 = (c.m.r32((p_bolt_1).wrapping_add(8)) as i32);
        { let r_ = F32(0x4effffff); let n_ = rt::fdiv(l_r1_7, r_); l_r1_7 = n_; };
        F::Vec4_Sub(c, fp.wrapping_add(128), fp.wrapping_add(16), fp);
        F::Vec3_Normalize(c, fp.wrapping_add(128), fp.wrapping_add(128));
        F::Vec3_Cross(c, fp.wrapping_add(96), fp.wrapping_add(128), (p_bolt_1).wrapping_add(128));
        { let r_ = F32(0x4effffff); let n_ = rt::fdiv(l_r2_8, r_); l_r2_8 = n_; };
        F::Vec3_Cross(c, fp.wrapping_add(112), fp.wrapping_add(96), fp.wrapping_add(128));
        { let r_ = l_amp_5; let n_ = rt::fadd(l_r1_7, r_); l_r1_7 = n_; };
        F::Vec4_Copy(c, fp.wrapping_add(32), fp);
        { let r_ = p_scale_4; let n_ = rt::fmul(l_r1_7, r_); l_r1_7 = n_; };
        { let r_ = l_amp_5; let n_ = rt::fadd(l_r2_8, r_); l_r2_8 = n_; };
        F::Vec4_Copy(c, fp.wrapping_add(48), fp.wrapping_add(16));
        l_bright_6 = F32(0x3f800000);
        l_amp_5 = p_scale_4;
        { let r_ = p_scale_4; let n_ = rt::fmul(l_r2_8, r_); l_r2_8 = n_; };
        { let r_ = rt::fmul(F32(c.m.r32(fp.wrapping_add(112))), l_r1_7); let a_ = fp.wrapping_add(48); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        { let r_ = rt::fmul(F32(c.m.r32((fp.wrapping_add(112)).wrapping_add(4))), l_r1_7); let a_ = (fp.wrapping_add(48)).wrapping_add(4); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        { let r_ = rt::fmul(F32(c.m.r32((fp.wrapping_add(112)).wrapping_add(8))), l_r1_7); let a_ = (fp.wrapping_add(48)).wrapping_add(8); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        { let v_ = rt::fmul(F32(c.m.r32(fp.wrapping_add(112))), l_r2_8); let a_ = fp.wrapping_add(64); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(F32(c.m.r32((fp.wrapping_add(112)).wrapping_add(4))), l_r2_8); let a_ = (fp.wrapping_add(64)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(F32(c.m.r32((fp.wrapping_add(112)).wrapping_add(8))), l_r2_8); let a_ = (fp.wrapping_add(64)).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = l_bright_6; let a_ = (fp.wrapping_add(64)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        l_link_14 = p_bolt_1;
        'l1: while (((((c.m.r32(l_link_14)) != (((0i32) as u32))) as i32)) != 0) {
            l_seg_13 = c.m.r32(l_link_14);
            { let r_ = 1i32; let a_ = l_seg_13; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            if (((((l_n_12) == (0i32)) as i32)) != 0) {
                l_t_9 = rt::fmul(rt::fmul(rt::fsub(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff))), F32(0x3e4ccccc)), p_scale_4);
                { let r_ = rt::fmul(F32(c.m.r32(fp.wrapping_add(96))), l_t_9); let a_ = fp.wrapping_add(64); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                { let r_ = rt::fmul(F32(c.m.r32((fp.wrapping_add(96)).wrapping_add(4))), l_t_9); let a_ = (fp.wrapping_add(64)).wrapping_add(4); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                { let r_ = rt::fmul(F32(c.m.r32((fp.wrapping_add(96)).wrapping_add(8))), l_t_9); let a_ = (fp.wrapping_add(64)).wrapping_add(8); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            } else {
                { let o_ = l_count_10; let n_ = (o_).wrapping_sub(1); l_count_10 = n_; };
                l_t_9 = rt::fmul(rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x3f000000)), F32(0x3e999999)), l_amp_5);
                l_amp_5 = rt::fneg(l_amp_5);
                if (((((l_count_10) <= (0i32)) as i32)) != 0) {
                    l_count_10 = 1i32;
                }
                { let r_ = rt::fmul(F32(c.m.r32(fp.wrapping_add(96))), l_t_9); let a_ = fp.wrapping_add(64); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                { let r_ = rt::fmul(F32(c.m.r32((fp.wrapping_add(96)).wrapping_add(4))), l_t_9); let a_ = (fp.wrapping_add(64)).wrapping_add(4); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                { let r_ = rt::fmul(F32(c.m.r32((fp.wrapping_add(96)).wrapping_add(8))), l_t_9); let a_ = (fp.wrapping_add(64)).wrapping_add(8); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                F::Vec4_Add(c, fp.wrapping_add(80), fp.wrapping_add(32), fp.wrapping_add(64));
                F::Vec4_Sub(c, fp.wrapping_add(80), fp.wrapping_add(48), fp.wrapping_add(80));
                { let r_ = rt::i2f(l_count_10); let a_ = fp.wrapping_add(80); let n_ = rt::fdiv(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                { let r_ = rt::i2f(l_count_10); let a_ = (fp.wrapping_add(80)).wrapping_add(4); let n_ = rt::fdiv(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                { let r_ = rt::i2f(l_count_10); let a_ = (fp.wrapping_add(80)).wrapping_add(8); let n_ = rt::fdiv(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                l_t_9 = rt::fadd(rt::fmul(rt::fsub(F32(0x3f800000), rt::i2f(rt::div_i32((l_count_10).wrapping_sub(1i32), (c.m.r32((p_bolt_1).wrapping_add(8)) as i32)))), F32(0x3f199999)), F32(0x3ecccccc));
                { let r_ = rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x3e4ccccc)), F32(0x3f599999)); let n_ = rt::fmul(l_t_9, r_); l_t_9 = n_; };
                { let r_ = rt::fmul(F32(c.m.r32(fp.wrapping_add(80))), l_t_9); let a_ = fp.wrapping_add(64); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                { let r_ = rt::fmul(F32(c.m.r32((fp.wrapping_add(80)).wrapping_add(4))), l_t_9); let a_ = (fp.wrapping_add(64)).wrapping_add(4); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                { let r_ = rt::fmul(F32(c.m.r32((fp.wrapping_add(80)).wrapping_add(8))), l_t_9); let a_ = (fp.wrapping_add(64)).wrapping_add(8); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            }
            { let v_ = F32(c.m.r32(fp.wrapping_add(64))); let a_ = (l_seg_13).wrapping_add(16); c.m.w32(a_, (v_).0); };
            { let v_ = F32(c.m.r32((fp.wrapping_add(64)).wrapping_add(4))); let a_ = ((l_seg_13).wrapping_add(16)).wrapping_add(4); c.m.w32(a_, (v_).0); };
            { let v_ = F32(c.m.r32((fp.wrapping_add(64)).wrapping_add(8))); let a_ = ((l_seg_13).wrapping_add(16)).wrapping_add(8); c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x3f800000); let a_ = ((l_seg_13).wrapping_add(16)).wrapping_add(12); c.m.w32(a_, (v_).0); };
            l_t_9 = rt::i2f(rt::div_i32(l_count_10, (c.m.r32((p_bolt_1).wrapping_add(8)) as i32)));
            l_r1_7 = rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff));
            if (((rt::flt(l_amp_5, F32(0x00000000)) as i32)) != 0) {
                l_r1_7 = rt::fmul(rt::fmul(rt::fadd(rt::fmul(l_r1_7, F32(0x3f000000)), F32(0x3e999999)), l_t_9), l_amp_5);
                { let r_ = rt::fmul(F32(c.m.r32(fp.wrapping_add(96))), l_r1_7); let a_ = fp.wrapping_add(64); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                { let r_ = rt::fmul(F32(c.m.r32((fp.wrapping_add(96)).wrapping_add(4))), l_r1_7); let a_ = (fp.wrapping_add(64)).wrapping_add(4); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                { let r_ = rt::fmul(F32(c.m.r32((fp.wrapping_add(96)).wrapping_add(8))), l_r1_7); let a_ = (fp.wrapping_add(64)).wrapping_add(8); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                l_r1_7 = rt::fmul(rt::fmul(rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x3ecccccc)), F32(0x3dcccccc)), l_t_9), l_amp_5);
                { let r_ = rt::fmul(F32(c.m.r32(fp.wrapping_add(112))), l_r1_7); let a_ = fp.wrapping_add(64); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                { let r_ = rt::fmul(F32(c.m.r32((fp.wrapping_add(112)).wrapping_add(4))), l_r1_7); let a_ = (fp.wrapping_add(64)).wrapping_add(4); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                { let r_ = rt::fmul(F32(c.m.r32((fp.wrapping_add(112)).wrapping_add(8))), l_r1_7); let a_ = (fp.wrapping_add(64)).wrapping_add(8); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            } else {
                l_r1_7 = rt::fmul(rt::fmul(rt::fadd(rt::fmul(l_r1_7, F32(0x3f000000)), F32(0x3e999999)), l_t_9), l_amp_5);
                { let r_ = rt::fmul(F32(c.m.r32(fp.wrapping_add(96))), l_r1_7); let a_ = fp.wrapping_add(64); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                { let r_ = rt::fmul(F32(c.m.r32((fp.wrapping_add(96)).wrapping_add(4))), l_r1_7); let a_ = (fp.wrapping_add(64)).wrapping_add(4); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                { let r_ = rt::fmul(F32(c.m.r32((fp.wrapping_add(96)).wrapping_add(8))), l_r1_7); let a_ = (fp.wrapping_add(64)).wrapping_add(8); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                l_r1_7 = rt::fmul(rt::fmul(rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x3ecccccc)), F32(0x3dcccccc)), l_t_9), l_amp_5);
                { let r_ = rt::fmul(F32(c.m.r32(fp.wrapping_add(112))), l_r1_7); let a_ = fp.wrapping_add(64); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                { let r_ = rt::fmul(F32(c.m.r32((fp.wrapping_add(112)).wrapping_add(4))), l_r1_7); let a_ = (fp.wrapping_add(64)).wrapping_add(4); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                { let r_ = rt::fmul(F32(c.m.r32((fp.wrapping_add(112)).wrapping_add(8))), l_r1_7); let a_ = (fp.wrapping_add(64)).wrapping_add(8); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            }
            l_r1_7 = rt::fmul(rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x3ecccccc)), F32(0x3f199999)), p_scale_4);
            { let r_ = rt::fmul(F32(c.m.r32(fp.wrapping_add(112))), l_r1_7); let a_ = fp.wrapping_add(64); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            { let r_ = rt::fmul(F32(c.m.r32((fp.wrapping_add(112)).wrapping_add(4))), l_r1_7); let a_ = (fp.wrapping_add(64)).wrapping_add(4); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            { let r_ = rt::fmul(F32(c.m.r32((fp.wrapping_add(112)).wrapping_add(8))), l_r1_7); let a_ = (fp.wrapping_add(64)).wrapping_add(8); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            { let v_ = rt::fdiv(rt::fmul(l_bright_6, F32(0x42480000)), F32(0x437f0000)); let a_ = (l_seg_13).wrapping_add(32); c.m.w32(a_, (v_).0); };
            { let v_ = rt::fdiv(rt::fmul(l_bright_6, F32(0x41f00000)), F32(0x437f0000)); let a_ = (l_seg_13).wrapping_add(36); c.m.w32(a_, (v_).0); };
            { let v_ = rt::fdiv(rt::fmul(l_bright_6, F32(0x43160000)), F32(0x437f0000)); let a_ = (l_seg_13).wrapping_add(40); c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x3f169696); let a_ = (l_seg_13).wrapping_add(12); c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x00000000); let a_ = (l_seg_13).wrapping_add(44); c.m.w32(a_, (v_).0); };
            if (((((((((l_n_12) != (0i32)) as i32)) != 0) && (((((c.m.r32((l_seg_13).wrapping_add(48))) != (((0i32) as u32))) as i32)) != 0)) as i32)) != 0) {
                { let o_ = l_thick_11; let n_ = (o_).wrapping_add(1); l_thick_11 = n_; };
                { let r_ = 32i32; let a_ = l_seg_13; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                if (((((l_thick_11) == (1i32)) as i32)) != 0) {
                    { let v_ = rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x3cf5c28f)), F32(0x3d4ccccc)); let a_ = (l_seg_13).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x3cf5c28f)), F32(0x3d4ccccc)); let a_ = (l_seg_13).wrapping_add(8); c.m.w32(a_, (v_).0); };
                } else {
                    { let v_ = rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x3d4ccccc)), F32(0x3de147ae)); let a_ = (l_seg_13).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x3d4ccccc)), F32(0x3de147ae)); let a_ = (l_seg_13).wrapping_add(8); c.m.w32(a_, (v_).0); };
                }
                if (((((l_thick_11) >= (3i32)) as i32)) != 0) {
                    l_thick_11 = 0i32;
                }
            } else {
                { let v_ = { let v_ = F32(0x3ca3d70a); let a_ = (l_seg_13).wrapping_add(8); c.m.w32(a_, (v_).0); v_ }; let a_ = (l_seg_13).wrapping_add(4); c.m.w32(a_, (v_).0); };
            }
            { let o_ = l_n_12; let n_ = (o_).wrapping_add(1); l_n_12 = n_; };
            { let r_ = rt::fmul(p_scale_4, F32(0x40400000)); let a_ = (l_seg_13).wrapping_add(4); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            { let r_ = rt::fmul(p_scale_4, F32(0x40400000)); let a_ = (l_seg_13).wrapping_add(8); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            l_link_14 = (l_seg_13).wrapping_add(48);
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftBolt_Shape", &PROBE_N, &[], true); }
    r_
}

pub fn EftBolt_Step(c: &mut Ctx, mut p_bolt_1: u32, mut p_scale_2: F32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBolt_Step", &PROBE_N, &[(p_bolt_1) as u32, (p_scale_2).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_seg_3: u32 = 0;
    let mut l_next_4: u32 = 0;
    let mut l_link_5: u32 = 0;
    let mut l_n_6: i32 = 0;
    let mut l_done_7: i32 = 0;
    let mut l_flags_8: i32 = 0;
    let mut l_t_9: F32 = F32(0);
    let mut l_limit_10: F32 = F32(0);
    let r_ = 'f: {
        l_n_6 = 0i32;
        l_done_7 = 0i32;
        { let _ = F::memset(c, fp, 0i32, 16u32); };
        l_link_5 = p_bolt_1;
        'l1: while (((((c.m.r32(l_link_5)) != (((0i32) as u32))) as i32)) != 0) {
            l_seg_3 = c.m.r32(l_link_5);
            l_next_4 = c.m.r32((l_seg_3).wrapping_add(48));
            if (((((c.m.r32((p_bolt_1).wrapping_add(12)) as i32)) & (2i32))) != 0) {
                { let r_ = 2i32; let a_ = l_seg_3; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                if ((((!(((((c.m.r32(l_seg_3) as i32)) & (16i32))) != 0)) as i32)) != 0) {
                    if ((((!(((((((c.m.r32((p_bolt_1).wrapping_add(8)) as i32)).wrapping_sub((c.m.r32((p_bolt_1).wrapping_add(4)) as i32))) < (l_n_6)) as i32)) != 0)) as i32)) != 0) {
                        { let r_ = 16i32; let a_ = l_seg_3; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    }
                }
            } else {
                if ((((!((((((c.m.r32((p_bolt_1).wrapping_add(4)) as i32)) < (l_n_6)) as i32)) != 0)) as i32)) != 0) {
                    { let r_ = 2i32; let a_ = l_seg_3; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                }
            }
            l_flags_8 = (c.m.r32(l_seg_3) as i32);
            if ((((!((((l_flags_8) & (8i32))) != 0)) as i32)) != 0) {
                if ((((l_flags_8) & (2i32))) != 0) {
                    if ((((!((((l_flags_8) & (4i32))) != 0)) as i32)) != 0) {
                        { let v_ = rt::fmul(F32(c.m.r32((l_seg_3).wrapping_add(12))), F32(0x3eb33333)); let a_ = (l_seg_3).wrapping_add(44); c.m.w32(a_, (v_).0); };
                        if ((((l_flags_8) & (32i32))) != 0) {
                            if ((((((c.m.r32((p_bolt_1).wrapping_add(16)) as i32)) < (4i32)) as i32)) != 0) {
                                { let r_ = F32(0x40b00000); let a_ = (l_seg_3).wrapping_add(4); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                                { let r_ = F32(0x40b00000); let a_ = (l_seg_3).wrapping_add(8); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                            } else {
                                { let r_ = F32(0x40300000); let a_ = (l_seg_3).wrapping_add(4); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                                { let r_ = F32(0x40300000); let a_ = (l_seg_3).wrapping_add(8); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                            }
                        }
                        { let v_ = ((l_flags_8) | (4i32)); let a_ = l_seg_3; c.m.w32(a_, (v_) as u32); };
                    } else {
                        { let v_ = F32(c.m.r32((l_seg_3).wrapping_add(12))); let a_ = (l_seg_3).wrapping_add(44); c.m.w32(a_, (v_).0); };
                        if ((((l_flags_8) & (32i32))) != 0) {
                            if ((((((c.m.r32((p_bolt_1).wrapping_add(16)) as i32)) < (4i32)) as i32)) != 0) {
                                { let r_ = F32(0x3e999999); let a_ = (l_seg_3).wrapping_add(4); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                                { let r_ = F32(0x3e999999); let a_ = (l_seg_3).wrapping_add(8); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                            } else {
                                { let r_ = F32(0x3f1c28f5); let a_ = (l_seg_3).wrapping_add(4); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                                { let r_ = F32(0x3f1c28f5); let a_ = (l_seg_3).wrapping_add(8); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                            }
                        }
                        { let v_ = ((l_flags_8) | (8i32)); let a_ = l_seg_3; c.m.w32(a_, (v_) as u32); };
                    }
                }
            } else {
                if ((((l_flags_8) & (16i32))) != 0) {
                    { let r_ = F32(0x3f4ccccc); let a_ = (l_seg_3).wrapping_add(4); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    { let r_ = F32(0x3f4ccccc); let a_ = (l_seg_3).wrapping_add(8); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    if (((((l_next_4) != (((0i32) as u32))) as i32)) != 0) {
                        F::Vec4_Sub(c, fp, (l_next_4).wrapping_add(16), (l_seg_3).wrapping_add(16));
                    } else {
                        l_t_9 = rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x3ee66666));
                        { let v_ = rt::fmul(F32(c.m.r32((p_bolt_1).wrapping_add(112))), l_t_9); let a_ = fp; c.m.w32(a_, (v_).0); };
                        { let v_ = rt::fmul(F32(c.m.r32(((p_bolt_1).wrapping_add(112)).wrapping_add(4))), l_t_9); let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
                        { let v_ = rt::fmul(F32(c.m.r32(((p_bolt_1).wrapping_add(112)).wrapping_add(8))), l_t_9); let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_).0); };
                        l_t_9 = rt::fsub(rt::fsub(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x3eb33333)), rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x3eb33333))), F32(0x3e999999));
                        { let r_ = rt::fmul(F32(c.m.r32((p_bolt_1).wrapping_add(128))), l_t_9); let a_ = fp; let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                        { let r_ = rt::fmul(F32(c.m.r32(((p_bolt_1).wrapping_add(128)).wrapping_add(4))), l_t_9); let a_ = (fp).wrapping_add(4); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                        { let r_ = rt::fmul(F32(c.m.r32(((p_bolt_1).wrapping_add(128)).wrapping_add(8))), l_t_9); let a_ = (fp).wrapping_add(8); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                        F::Vec3_Normalize(c, fp, fp);
                        F::Vec4_Scale(c, fp, fp, rt::fmul(p_scale_2, F32(0x3f8ccccc)));
                    }
                    { let r_ = F32(c.m.r32(fp)); let a_ = (l_seg_3).wrapping_add(16); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    { let r_ = F32(c.m.r32((fp).wrapping_add(4))); let a_ = ((l_seg_3).wrapping_add(16)).wrapping_add(4); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    { let r_ = F32(c.m.r32((fp).wrapping_add(8))); let a_ = ((l_seg_3).wrapping_add(16)).wrapping_add(8); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    { let r_ = rt::fmul(F32(c.m.r32((l_seg_3).wrapping_add(12))), F32(0x3e800000)); let a_ = (l_seg_3).wrapping_add(44); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    l_limit_10 = rt::fmul(F32(c.m.r32((l_seg_3).wrapping_add(12))), F32(0x3dcccccc));
                    if (((rt::fle(F32(c.m.r32((l_seg_3).wrapping_add(44))), l_limit_10) as i32)) != 0) {
                        { let v_ = l_limit_10; let a_ = (l_seg_3).wrapping_add(44); c.m.w32(a_, (v_).0); };
                        if ((((!(((((l_n_6) < (((c.m.r32((p_bolt_1).wrapping_add(8)) as i32)).wrapping_sub(1i32))) as i32)) != 0)) as i32)) != 0) {
                            l_done_7 = 1i32;
                        }
                    }
                }
            }
            l_link_5 = (l_seg_3).wrapping_add(48);
            { let o_ = l_n_6; let n_ = (o_).wrapping_add(1); l_n_6 = n_; };
        }
        break 'f l_done_7;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftBolt_Step", &PROBE_N, &[], true); }
    r_
}

pub fn EftBolt_Draw(c: &mut Ctx, mut p_work_1: u32, mut p_bolt_2: u32, mut p_alpha_3: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBolt_Draw", &PROBE_N, &[(p_work_1) as u32, (p_bolt_2) as u32, (p_alpha_3).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(416) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 416);
    let mut l_clipped_4: i32 = 0;
    let mut l_first_5: i32 = 0;
    let mut l_n_6: i32 = 0;
    let mut l_n1_7: i32 = 0;
    let mut l_seg_8: u32 = 0;
    let mut l_next_9: u32 = 0;
    let mut l_link_10: u32 = 0;
    let mut l_pkt_11: u32 = 0;
    let mut l_i_12: i32 = 0;
    let mut l_z_13: i32 = 0;
    let r_ = 'f: {
        { let a1_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(544); F::Vec4_Copy(c, fp.wrapping_add(256), a1_) };
        F::Vec4_Add(c, fp.wrapping_add(224), (p_bolt_2).wrapping_add(48), (p_bolt_2).wrapping_add(80));
        l_first_5 = 1i32;
        l_n_6 = 0i32;
        {
            l_link_10 = p_bolt_2;
            'l1: loop {
                if !(((((c.m.r32(l_link_10)) != (((0i32) as u32))) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_seg_8 = c.m.r32(l_link_10);
                    l_next_9 = c.m.r32((l_seg_8).wrapping_add(48));
                    if (((((l_next_9) == (((0i32) as u32))) as i32)) != 0) {
                        break 'c2;
                    }
                    if ((((!(((((c.m.r32(l_seg_8) as i32)) & (2i32))) != 0)) as i32)) != 0) {
                        break 'c2;
                    }
                    F::Vec4_Sub(c, fp.wrapping_add(272), (l_next_9).wrapping_add(16), (l_seg_8).wrapping_add(16));
                    l_clipped_4 = 0i32;
                    F::Vec4_Add(c, fp.wrapping_add(240), fp.wrapping_add(224), (l_seg_8).wrapping_add(16));
                    { let v_ = F32(0x3f800000); let a_ = (fp.wrapping_add(240)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                    F::Vec4_Sub(c, fp.wrapping_add(288), fp.wrapping_add(240), fp.wrapping_add(256));
                    F::Vec3_Cross(c, fp.wrapping_add(272), fp.wrapping_add(272), fp.wrapping_add(288));
                    F::Vec3_Normalize(c, fp.wrapping_add(272), fp.wrapping_add(272));
                    if ((l_first_5) != 0) {
                        { let a2_ = F32(c.m.r32((l_seg_8).wrapping_add(8))); F::Vec3_Scale(c, fp.wrapping_add(304), fp.wrapping_add(272), a2_) };
                        F::Vec4_Add(c, (fp).wrapping_add((((0i32) as u32)).wrapping_mul(16)), fp.wrapping_add(240), fp.wrapping_add(304));
                        { let v_ = F32(0x3f800000); let a_ = ((fp).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
                        { let a2_ = rt::fneg(F32(c.m.r32((l_seg_8).wrapping_add(4)))); F::Vec3_Scale(c, fp.wrapping_add(304), fp.wrapping_add(272), a2_) };
                        F::Vec4_Add(c, (fp).wrapping_add((((1i32) as u32)).wrapping_mul(16)), fp.wrapping_add(240), fp.wrapping_add(304));
                        { let v_ = F32(0x3f800000); let a_ = ((fp).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
                    } else {
                        F::Vec4_Copy(c, (fp).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(64)).wrapping_add((((0i32) as u32)).wrapping_mul(16)));
                        F::Vec4_Copy(c, (fp).wrapping_add((((1i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(64)).wrapping_add((((1i32) as u32)).wrapping_mul(16)));
                    }
                    l_n1_7 = (l_n_6).wrapping_add(1i32);
                    { let a2_ = F32(c.m.r32((l_next_9).wrapping_add(8))); F::Vec3_Scale(c, fp.wrapping_add(304), fp.wrapping_add(272), a2_) };
                    F::Vec4_Add(c, (fp).wrapping_add((((2i32) as u32)).wrapping_mul(16)), fp.wrapping_add(240), fp.wrapping_add(304));
                    { let v_ = F32(0x3f800000); let a_ = ((fp).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
                    l_first_5 = 0i32;
                    { let a2_ = rt::fneg(F32(c.m.r32((l_next_9).wrapping_add(4)))); F::Vec3_Scale(c, fp.wrapping_add(304), fp.wrapping_add(272), a2_) };
                    F::Vec4_Add(c, (fp).wrapping_add((((3i32) as u32)).wrapping_mul(16)), fp.wrapping_add(240), fp.wrapping_add(304));
                    { let v_ = F32(0x3f800000); let a_ = ((fp).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
                    F::Vec4_Copy(c, (fp.wrapping_add(64)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (fp).wrapping_add((((2i32) as u32)).wrapping_mul(16)));
                    F::Vec4_Copy(c, (fp.wrapping_add(64)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), (fp).wrapping_add((((3i32) as u32)).wrapping_mul(16)));
                    { let v_ = rt::i2f(rt::rem_i32(l_n_6, 2i32)); let a_ = (fp.wrapping_add(160)).wrapping_add((((0i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::i2f(rt::rem_i32(l_n1_7, 2i32)); let a_ = ((fp.wrapping_add(160)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(0x3f800000); let a_ = ((fp.wrapping_add(160)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(0x00000000); let a_ = ((fp.wrapping_add(160)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::i2f(rt::rem_i32(l_n_6, 2i32)); let a_ = (fp.wrapping_add(160)).wrapping_add((((1i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::i2f(rt::rem_i32(l_n_6, 2i32)); let a_ = ((fp.wrapping_add(160)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(0x3f800000); let a_ = ((fp.wrapping_add(160)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(0x00000000); let a_ = ((fp.wrapping_add(160)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::i2f(rt::rem_i32(l_n1_7, 2i32)); let a_ = (fp.wrapping_add(160)).wrapping_add((((2i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::i2f(rt::rem_i32(l_n1_7, 2i32)); let a_ = ((fp.wrapping_add(160)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(0x3f800000); let a_ = ((fp.wrapping_add(160)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(0x00000000); let a_ = ((fp.wrapping_add(160)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::i2f(rt::rem_i32(l_n1_7, 2i32)); let a_ = (fp.wrapping_add(160)).wrapping_add((((3i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::i2f(rt::rem_i32(l_n_6, 2i32)); let a_ = ((fp.wrapping_add(160)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(0x3f800000); let a_ = ((fp.wrapping_add(160)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(0x00000000); let a_ = ((fp.wrapping_add(160)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
                    {
                        l_i_12 = 0i32;
                        'l3: loop {
                            if !(((((l_i_12) < (4i32)) as i32)) != 0) { break 'l3; }
                            'c4: {
                                { let _ = F::Vu0Cur_ProjectPoint(c, (fp.wrapping_add(320)).wrapping_add((((l_i_12) as u32)).wrapping_mul(16)), (fp).wrapping_add((((l_i_12) as u32)).wrapping_mul(16))); };
                                { let a2_ = rt::fdiv(F32(0x3f800000), rt::i2f((c.m.r32(((fp.wrapping_add(320)).wrapping_add((((l_i_12) as u32)).wrapping_mul(16))).wrapping_add(12)) as i32))); F::Vec4_Scale(c, (fp.wrapping_add(96)).wrapping_add((((l_i_12) as u32)).wrapping_mul(16)), (fp.wrapping_add(160)).wrapping_add((((l_i_12) as u32)).wrapping_mul(16)), a2_) };
                                if (((((c.m.r32((fp.wrapping_add(320)).wrapping_add((((l_i_12) as u32)).wrapping_mul(16)))) > (((65520i32) as u32))) as i32)) != 0) {
                                    l_clipped_4 = 1i32;
                                    break 'l3;
                                }
                                if (((((c.m.r32(((fp.wrapping_add(320)).wrapping_add((((l_i_12) as u32)).wrapping_mul(16))).wrapping_add(4))) > (((65520i32) as u32))) as i32)) != 0) {
                                    l_clipped_4 = 1i32;
                                    break 'l3;
                                }
                                if ((((((c.m.r32(((fp.wrapping_add(320)).wrapping_add((((l_i_12) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)) < (0i32)) as i32)) != 0) {
                                    l_clipped_4 = 1i32;
                                    break 'l3;
                                }
                            }
                            { let o_ = l_i_12; let n_ = (o_).wrapping_add(1); l_i_12 = n_; };
                        }
                    }
                    if ((((!((l_clipped_4) != 0)) as i32)) != 0) {
                        l_pkt_11 = c.m.r32(sym::gOtCur);
                        { let v_ = (l_pkt_11).wrapping_add((((1i32) as u32)).wrapping_mul(144)); let a_ = sym::gOtCur; c.m.w32(a_, v_); };
                        if (((((l_pkt_11) == (((0i32) as u32))) as i32)) != 0) {
                            break 'f;
                        }
                        { let v_ = ((536870920i32) as u32); let a_ = l_pkt_11; c.m.w32(a_, v_); };
                        { let v_ = ((1342177288i32) as u32); let a_ = (l_pkt_11).wrapping_add(12); c.m.w32(a_, v_); };
                        { let v_ = ((92i32) as u64); let a_ = (l_pkt_11).wrapping_add(32); c.m.w64(a_, v_); };
                        { let v_ = ((268435456i32) as u32); let a_ = (l_pkt_11).wrapping_add(8); c.m.w32(a_, v_); };
                        { let v_ = 16429131440647602177u64; let a_ = (l_pkt_11).wrapping_add(16); c.m.w64(a_, v_); };
                        { let v_ = ((18599481537995104i64) as u64); let a_ = (l_pkt_11).wrapping_add(24); c.m.w64(a_, v_); };
                        { let v_ = ((0i32) as u32); let a_ = (l_pkt_11).wrapping_add(4); c.m.w32(a_, v_); };
                        { let a1_ = rt::f2i(rt::fmul(F32(c.m.r32((l_seg_8).wrapping_add(32))), F32(0x437f0000))); let a2_ = rt::f2i(rt::fmul(F32(c.m.r32((l_seg_8).wrapping_add(36))), F32(0x437f0000))); let a3_ = rt::f2i(rt::fmul(F32(c.m.r32((l_seg_8).wrapping_add(40))), F32(0x437f0000))); let a4_ = rt::f2i(rt::fmul(rt::fmul(F32(c.m.r32((l_seg_8).wrapping_add(44))), F32(0x437f0000)), p_alpha_3)); F::IVec4_Set(c, fp.wrapping_add(384), a1_, a2_, a3_, a4_) };
                        { let a1_ = rt::f2i(rt::fmul(F32(c.m.r32((l_next_9).wrapping_add(32))), F32(0x437f0000))); let a2_ = rt::f2i(rt::fmul(F32(c.m.r32((l_next_9).wrapping_add(36))), F32(0x437f0000))); let a3_ = rt::f2i(rt::fmul(F32(c.m.r32((l_next_9).wrapping_add(40))), F32(0x437f0000))); let a4_ = rt::f2i(rt::fmul(rt::fmul(F32(c.m.r32((l_next_9).wrapping_add(44))), F32(0x437f0000)), p_alpha_3)); F::IVec4_Set(c, fp.wrapping_add(400), a1_, a2_, a3_, a4_) };
                        { let v_ = (((c.m.r32((fp.wrapping_add(384)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) as u8); let a_ = ((l_pkt_11).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24)); c.m.w8(a_, v_); };
                        { let v_ = (((c.m.r32((fp.wrapping_add(384)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) as u8); let a_ = (((l_pkt_11).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(1); c.m.w8(a_, v_); };
                        { let v_ = (((c.m.r32((fp.wrapping_add(384)).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32)) as u8); let a_ = (((l_pkt_11).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(2); c.m.w8(a_, v_); };
                        { let v_ = (((c.m.r32((fp.wrapping_add(384)).wrapping_add((((3i32) as u32)).wrapping_mul(4))) as i32)) as u8); let a_ = (((l_pkt_11).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(3); c.m.w8(a_, v_); };
                        { let v_ = F32(c.m.r32(((fp.wrapping_add(96)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8))); let a_ = (((l_pkt_11).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                        { let v_ = (((c.m.r32((fp.wrapping_add(384)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) as u8); let a_ = ((l_pkt_11).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24)); c.m.w8(a_, v_); };
                        { let v_ = (((c.m.r32((fp.wrapping_add(384)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) as u8); let a_ = (((l_pkt_11).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(1); c.m.w8(a_, v_); };
                        { let v_ = (((c.m.r32((fp.wrapping_add(384)).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32)) as u8); let a_ = (((l_pkt_11).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(2); c.m.w8(a_, v_); };
                        { let v_ = (((c.m.r32((fp.wrapping_add(384)).wrapping_add((((3i32) as u32)).wrapping_mul(4))) as i32)) as u8); let a_ = (((l_pkt_11).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(3); c.m.w8(a_, v_); };
                        { let v_ = F32(c.m.r32(((fp.wrapping_add(96)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8))); let a_ = (((l_pkt_11).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                        { let v_ = (((c.m.r32((fp.wrapping_add(400)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) as u8); let a_ = ((l_pkt_11).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24)); c.m.w8(a_, v_); };
                        { let v_ = (((c.m.r32((fp.wrapping_add(400)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) as u8); let a_ = (((l_pkt_11).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(1); c.m.w8(a_, v_); };
                        { let v_ = (((c.m.r32((fp.wrapping_add(400)).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32)) as u8); let a_ = (((l_pkt_11).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(2); c.m.w8(a_, v_); };
                        { let v_ = (((c.m.r32((fp.wrapping_add(400)).wrapping_add((((3i32) as u32)).wrapping_mul(4))) as i32)) as u8); let a_ = (((l_pkt_11).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(3); c.m.w8(a_, v_); };
                        { let v_ = F32(c.m.r32(((fp.wrapping_add(96)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8))); let a_ = (((l_pkt_11).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                        { let v_ = (((c.m.r32((fp.wrapping_add(400)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) as u8); let a_ = ((l_pkt_11).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24)); c.m.w8(a_, v_); };
                        { let v_ = (((c.m.r32((fp.wrapping_add(400)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) as u8); let a_ = (((l_pkt_11).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(1); c.m.w8(a_, v_); };
                        { let v_ = (((c.m.r32((fp.wrapping_add(400)).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32)) as u8); let a_ = (((l_pkt_11).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(2); c.m.w8(a_, v_); };
                        { let v_ = (((c.m.r32((fp.wrapping_add(400)).wrapping_add((((3i32) as u32)).wrapping_mul(4))) as i32)) as u8); let a_ = (((l_pkt_11).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(3); c.m.w8(a_, v_); };
                        { let v_ = F32(c.m.r32(((fp.wrapping_add(96)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(8))); let a_ = (((l_pkt_11).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                        { let v_ = F32(c.m.r32((fp.wrapping_add(96)).wrapping_add((((0i32) as u32)).wrapping_mul(16)))); let a_ = (((l_pkt_11).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(8); c.m.w32(a_, (v_).0); };
                        { let v_ = F32(c.m.r32(((fp.wrapping_add(96)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4))); let a_ = (((l_pkt_11).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(12); c.m.w32(a_, (v_).0); };
                        { let v_ = F32(c.m.r32((fp.wrapping_add(96)).wrapping_add((((1i32) as u32)).wrapping_mul(16)))); let a_ = (((l_pkt_11).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(8); c.m.w32(a_, (v_).0); };
                        { let v_ = F32(c.m.r32(((fp.wrapping_add(96)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(4))); let a_ = (((l_pkt_11).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(12); c.m.w32(a_, (v_).0); };
                        { let v_ = F32(c.m.r32((fp.wrapping_add(96)).wrapping_add((((2i32) as u32)).wrapping_mul(16)))); let a_ = (((l_pkt_11).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(8); c.m.w32(a_, (v_).0); };
                        { let v_ = F32(c.m.r32(((fp.wrapping_add(96)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(4))); let a_ = (((l_pkt_11).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(12); c.m.w32(a_, (v_).0); };
                        { let v_ = F32(c.m.r32((fp.wrapping_add(96)).wrapping_add((((3i32) as u32)).wrapping_mul(16)))); let a_ = (((l_pkt_11).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(8); c.m.w32(a_, (v_).0); };
                        { let v_ = F32(c.m.r32(((fp.wrapping_add(96)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(4))); let a_ = (((l_pkt_11).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(12); c.m.w32(a_, (v_).0); };
                        { let v_ = ((c.m.r32((fp.wrapping_add(320)).wrapping_add((((0i32) as u32)).wrapping_mul(16)))) as u64); let a_ = (((l_pkt_11).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(16); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                        { let v_ = ((c.m.r32(((fp.wrapping_add(320)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4))) as u64); let a_ = (((l_pkt_11).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(18); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                        { let v_ = (((c.m.r32(((fp.wrapping_add(320)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)) as u64); let a_ = (((l_pkt_11).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(20); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
                        { let v_ = ((255i32) as u64); let a_ = (((l_pkt_11).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add(23); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
                        { let v_ = ((c.m.r32((fp.wrapping_add(320)).wrapping_add((((1i32) as u32)).wrapping_mul(16)))) as u64); let a_ = (((l_pkt_11).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(16); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                        { let v_ = ((c.m.r32(((fp.wrapping_add(320)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(4))) as u64); let a_ = (((l_pkt_11).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(18); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                        { let v_ = (((c.m.r32(((fp.wrapping_add(320)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)) as u64); let a_ = (((l_pkt_11).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(20); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
                        { let v_ = ((255i32) as u64); let a_ = (((l_pkt_11).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add(23); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
                        { let v_ = ((c.m.r32((fp.wrapping_add(320)).wrapping_add((((2i32) as u32)).wrapping_mul(16)))) as u64); let a_ = (((l_pkt_11).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(16); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                        { let v_ = ((c.m.r32(((fp.wrapping_add(320)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(4))) as u64); let a_ = (((l_pkt_11).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(18); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                        { let v_ = (((c.m.r32(((fp.wrapping_add(320)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)) as u64); let a_ = (((l_pkt_11).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(20); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
                        { let v_ = ((255i32) as u64); let a_ = (((l_pkt_11).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(24))).wrapping_add(23); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
                        { let v_ = ((c.m.r32((fp.wrapping_add(320)).wrapping_add((((3i32) as u32)).wrapping_mul(16)))) as u64); let a_ = (((l_pkt_11).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(16); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                        { let v_ = ((c.m.r32(((fp.wrapping_add(320)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(4))) as u64); let a_ = (((l_pkt_11).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(18); rt::bf_set(c, a_, 0, 16, (v_) as u64);  };
                        { let v_ = (((c.m.r32(((fp.wrapping_add(320)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)) as u64); let a_ = (((l_pkt_11).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(20); rt::bf_set(c, a_, 0, 24, (v_) as u64);  };
                        { let v_ = ((255i32) as u64); let a_ = (((l_pkt_11).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(24))).wrapping_add(23); rt::bf_set(c, a_, 0, 8, (v_) as u64);  };
                        { let v_ = c.m.r64((p_work_1).wrapping_add(3736)); let a_ = (l_pkt_11).wrapping_add(40); c.m.w64(a_, v_); };
                        l_z_13 = (((((c.m.r32(((fp.wrapping_add(320)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)).wrapping_add((c.m.r32(((fp.wrapping_add(320)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32))).wrapping_add((c.m.r32(((fp.wrapping_add(320)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32))).wrapping_add((c.m.r32(((fp.wrapping_add(320)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(12)) as i32))).wrapping_shr((10i32) as u32);
                        F::eft_n__EftAura_OtAdd(c, l_pkt_11, l_z_13, 1i32);
                    }
                    l_n_6 = l_n1_7;
                }
                l_link_10 = (l_seg_8).wrapping_add(48);
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftBolt_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn EftBolt_Start(c: &mut Ctx, mut p_work_1: u32, mut p_bolt_2: u32, mut p_objId_3: i32, mut p_kind_4: i32, mut p_nodeA_5: i32, mut p_nodeB_6: i32, p_dirA_7: u32, p_dirB_8: u32, mut p_length_9: F32, mut p_follow_10: F32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBolt_Start", &PROBE_N, &[(p_work_1) as u32, (p_bolt_2) as u32, (p_objId_3) as u32, (p_kind_4) as u32, (p_nodeA_5) as u32, (p_nodeB_6) as u32, (p_dirA_7) as u32, (p_dirB_8) as u32, (p_length_9).0, (p_follow_10).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(96) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 96);
    let mut l_one_11: F32 = F32(0);
    let mut l_t_12: F32 = F32(0);
    let mut l_x_13: F32 = F32(0);
    let mut l_y_14: F32 = F32(0);
    let mut l_z_15: F32 = F32(0);
    let mut l_n_16: i32 = 0;
    rt::memcpy(c, fp, p_dirA_7, 16);
    rt::memcpy(c, fp.wrapping_add(16), p_dirB_8, 16);
    let r_ = 'f: {
        l_one_11 = F32(0x3f800000);
        { let v_ = p_kind_4; let a_ = (p_bolt_2).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
        { let v_ = p_nodeB_6; let a_ = (p_bolt_2).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
        { let r_ = 1i32; let a_ = (p_bolt_2).wrapping_add(12); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        { let v_ = p_nodeA_5; let a_ = (p_bolt_2).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
        { let v_ = F32(0x41f00000); let a_ = (p_bolt_2).wrapping_add(28); c.m.w32(a_, (v_).0); };
        F::BtlCharApi_GetNodePos(c, p_objId_3, p_nodeA_5, (p_bolt_2).wrapping_add(48));
        { let v_ = rt::fmul(F32(c.m.r32(fp)), p_length_9); let a_ = (p_bolt_2).wrapping_add(80); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(F32(c.m.r32((fp).wrapping_add(4))), p_length_9); let a_ = ((p_bolt_2).wrapping_add(80)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = l_one_11; let a_ = ((p_bolt_2).wrapping_add(80)).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(F32(c.m.r32(fp.wrapping_add(16))), p_length_9); let a_ = (p_bolt_2).wrapping_add(96); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))), p_length_9); let a_ = ((p_bolt_2).wrapping_add(96)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = l_one_11; let a_ = ((p_bolt_2).wrapping_add(96)).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = p_follow_10; let a_ = (p_bolt_2).wrapping_add(32); c.m.w32(a_, (v_).0); };
        { let a1_ = (c.m.r32((p_bolt_2).wrapping_add(24)) as i32); F::BtlCharApi_GetNodePos(c, p_objId_3, a1_, fp.wrapping_add(64)) };
        F::Vec4_Sub(c, fp.wrapping_add(64), fp.wrapping_add(64), (p_bolt_2).wrapping_add(48));
        l_x_13 = rt::fadd(F32(c.m.r32((p_bolt_2).wrapping_add(48))), rt::fmul(F32(c.m.r32(fp.wrapping_add(64))), F32(c.m.r32((p_bolt_2).wrapping_add(32)))));
        l_y_14 = rt::fadd(F32(c.m.r32(((p_bolt_2).wrapping_add(48)).wrapping_add(4))), rt::fmul(F32(c.m.r32((fp.wrapping_add(64)).wrapping_add(4))), F32(c.m.r32((p_bolt_2).wrapping_add(32)))));
        l_z_15 = rt::fadd(F32(c.m.r32(((p_bolt_2).wrapping_add(48)).wrapping_add(8))), rt::fmul(F32(c.m.r32((fp.wrapping_add(64)).wrapping_add(8))), F32(c.m.r32((p_bolt_2).wrapping_add(32)))));
        { let v_ = rt::fadd(l_x_13, F32(c.m.r32((p_bolt_2).wrapping_add(80)))); let a_ = fp.wrapping_add(32); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(l_y_14, F32(c.m.r32(((p_bolt_2).wrapping_add(80)).wrapping_add(4)))); let a_ = (fp.wrapping_add(32)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(l_z_15, F32(c.m.r32(((p_bolt_2).wrapping_add(80)).wrapping_add(8)))); let a_ = (fp.wrapping_add(32)).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = l_one_11; let a_ = (fp.wrapping_add(32)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(l_x_13, F32(c.m.r32((p_bolt_2).wrapping_add(96)))); let a_ = fp.wrapping_add(48); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(l_y_14, F32(c.m.r32(((p_bolt_2).wrapping_add(96)).wrapping_add(4)))); let a_ = (fp.wrapping_add(48)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(l_z_15, F32(c.m.r32(((p_bolt_2).wrapping_add(96)).wrapping_add(8)))); let a_ = (fp.wrapping_add(48)).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = l_one_11; let a_ = (fp.wrapping_add(48)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        F::Vec4_Sub(c, fp.wrapping_add(80), fp.wrapping_add(48), fp.wrapping_add(32));
        F::Vec3_Normalize(c, (p_bolt_2).wrapping_add(112), fp.wrapping_add(80));
        { let v_ = rt::fmul(rt::fadd(F32(c.m.r32(fp)), F32(c.m.r32(fp.wrapping_add(16)))), F32(0x3f000000)); let a_ = (p_bolt_2).wrapping_add(128); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(rt::fadd(F32(c.m.r32((fp).wrapping_add(4))), F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4)))), F32(0x3f000000)); let a_ = ((p_bolt_2).wrapping_add(128)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(rt::fadd(F32(c.m.r32((fp).wrapping_add(8))), F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8)))), F32(0x3f000000)); let a_ = ((p_bolt_2).wrapping_add(128)).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = l_one_11; let a_ = ((p_bolt_2).wrapping_add(128)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        F::Vec3_Normalize(c, (p_bolt_2).wrapping_add(128), (p_bolt_2).wrapping_add(128));
        l_t_12 = rt::fmul(F32(c.m.r32((p_work_1).wrapping_add(3744))), F32(0x3f199999));
        { let r_ = rt::fmul(F32(c.m.r32((p_bolt_2).wrapping_add(128))), l_t_12); let a_ = (p_bolt_2).wrapping_add(80); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        { let r_ = rt::fmul(F32(c.m.r32(((p_bolt_2).wrapping_add(128)).wrapping_add(4))), l_t_12); let a_ = ((p_bolt_2).wrapping_add(80)).wrapping_add(4); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        { let r_ = rt::fmul(F32(c.m.r32(((p_bolt_2).wrapping_add(128)).wrapping_add(8))), l_t_12); let a_ = ((p_bolt_2).wrapping_add(80)).wrapping_add(8); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        { let r_ = rt::fmul(F32(c.m.r32((p_bolt_2).wrapping_add(128))), l_t_12); let a_ = (p_bolt_2).wrapping_add(96); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        { let r_ = rt::fmul(F32(c.m.r32(((p_bolt_2).wrapping_add(128)).wrapping_add(4))), l_t_12); let a_ = ((p_bolt_2).wrapping_add(96)).wrapping_add(4); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        { let r_ = rt::fmul(F32(c.m.r32(((p_bolt_2).wrapping_add(128)).wrapping_add(8))), l_t_12); let a_ = ((p_bolt_2).wrapping_add(96)).wrapping_add(8); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        l_n_16 = rt::f2i(rt::fadd(rt::fdiv({ let a0_ = F::Vec3_Dot(c, fp.wrapping_add(80), fp.wrapping_add(80)); F::Port_sqrtf(c, a0_) }, F32(c.m.r32((p_work_1).wrapping_add(3744)))), F32(0x40400000)));
        if (((((l_n_16) > (10i32)) as i32)) != 0) {
            l_n_16 = 10i32;
        }
        { let _ = F::EftBolt_AllocSegs(c, p_bolt_2, l_n_16); };
        if ((((((c.m.r32((p_bolt_2).wrapping_add(8)) as i32)) >= (3i32)) as i32)) != 0) {
            { let a3_ = F32(c.m.r32((p_work_1).wrapping_add(3744))); F::EftBolt_Shape(c, p_bolt_2, fp.wrapping_add(32), fp.wrapping_add(48), a3_) };
        } else {
            F::EftBolt_FreeSegs(c, p_bolt_2);
            break 'f 0i32;
        }
        { let a_ = (p_work_1).wrapping_add(3704); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        break 'f 1i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftBolt_Start", &PROBE_N, &[], true); }
    r_
}

pub fn EftBolt_UpdateAll(c: &mut Ctx, mut p_work_1: u32, mut p_objId_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBolt_UpdateAll", &PROBE_N, &[(p_work_1) as u32, (p_objId_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_bolt_3: u32 = 0;
    let mut l_i_4: i32 = 0;
    let r_ = 'f: {
        {
            l_i_4 = 0i32;
            'l1: loop {
                if !(((((l_i_4) < (10i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_bolt_3 = ((p_work_1).wrapping_add(16)).wrapping_add((((l_i_4) as u32)).wrapping_mul(144));
                    if (((((c.m.r32((l_bolt_3).wrapping_add(12)) as i32)) & (1i32))) != 0) {
                        if ((((!(((((c.m.r32((l_bolt_3).wrapping_add(12)) as i32)) & (4i32))) != 0)) as i32)) != 0) {
                            { let a1_ = (c.m.r32((l_bolt_3).wrapping_add(20)) as i32); F::BtlCharApi_GetNodePos(c, p_objId_2, a1_, (l_bolt_3).wrapping_add(48)) };
                            { let a1_ = (c.m.r32((l_bolt_3).wrapping_add(24)) as i32); F::BtlCharApi_GetNodePos(c, p_objId_2, a1_, fp) };
                            F::Vec4_Sub(c, fp, fp, (l_bolt_3).wrapping_add(48));
                            { let r_ = rt::fmul(F32(c.m.r32(fp)), F32(c.m.r32((l_bolt_3).wrapping_add(32)))); let a_ = (l_bolt_3).wrapping_add(48); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                            { let r_ = rt::fmul(F32(c.m.r32((fp).wrapping_add(4))), F32(c.m.r32((l_bolt_3).wrapping_add(32)))); let a_ = ((l_bolt_3).wrapping_add(48)).wrapping_add(4); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                            { let r_ = rt::fmul(F32(c.m.r32((fp).wrapping_add(8))), F32(c.m.r32((l_bolt_3).wrapping_add(32)))); let a_ = ((l_bolt_3).wrapping_add(48)).wrapping_add(8); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                            if (({ let a1_ = F32(c.m.r32((p_work_1).wrapping_add(3744))); F::EftBolt_Step(c, l_bolt_3, a1_) }) != 0) {
                                { let r_ = 4i32; let a_ = (l_bolt_3).wrapping_add(12); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                            }
                            if ((((!(((((c.m.r32((l_bolt_3).wrapping_add(12)) as i32)) & (2i32))) != 0)) as i32)) != 0) {
                                if ((((((c.m.r32((l_bolt_3).wrapping_add(4)) as i32)) < ((c.m.r32((l_bolt_3).wrapping_add(8)) as i32))) as i32)) != 0) {
                                    { let r_ = (c.m.r32((l_bolt_3).wrapping_add(8)) as i32); let a_ = (l_bolt_3).wrapping_add(4); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                                    if ((((!((((((c.m.r32((l_bolt_3).wrapping_add(4)) as i32)) < ((c.m.r32((l_bolt_3).wrapping_add(8)) as i32))) as i32)) != 0)) as i32)) != 0) {
                                        { let v_ = (c.m.r32((l_bolt_3).wrapping_add(8)) as i32); let a_ = (l_bolt_3).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                                        { let r_ = 2i32; let a_ = (l_bolt_3).wrapping_add(12); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                                    }
                                }
                            }
                            if (((((c.m.r32((l_bolt_3).wrapping_add(12)) as i32)) & (2i32))) != 0) {
                                if ((((((c.m.r32((l_bolt_3).wrapping_add(4)) as i32)) > (0i32)) as i32)) != 0) {
                                    { let v_ = (((c.m.r32((l_bolt_3).wrapping_add(4)) as i32)).wrapping_sub(rt::rem_i32(F::Port_Rand(c), 2i32))).wrapping_sub(2i32); let a_ = (l_bolt_3).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                                    if ((((((c.m.r32((l_bolt_3).wrapping_add(4)) as i32)) < (0i32)) as i32)) != 0) {
                                        { let v_ = 0i32; let a_ = (l_bolt_3).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                                    }
                                }
                            }
                            { let r_ = F32(0x3f800000); let a_ = (l_bolt_3).wrapping_add(28); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                            if (((rt::fle(F32(c.m.r32((l_bolt_3).wrapping_add(28))), F32(0x00000000)) as i32)) != 0) {
                                { let r_ = 4i32; let a_ = (l_bolt_3).wrapping_add(12); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                            }
                        } else {
                            F::EftBolt_FreeSegs(c, l_bolt_3);
                            { let v_ = 0i32; let a_ = (l_bolt_3).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                            { let a_ = (p_work_1).wrapping_add(3704); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                        }
                    }
                }
                { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftBolt_UpdateAll", &PROBE_N, &[], true); }
    r_
}

pub fn EftBolt_DrawAll(c: &mut Ctx, mut p_work_1: u32, mut p_alpha_2: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBolt_DrawAll", &PROBE_N, &[(p_work_1) as u32, (p_alpha_2).0], false); }
    let mut l_bolt_3: u32 = 0;
    let mut l_i_4: i32 = 0;
    let r_ = 'f: {
    F::Vu0Cur_Push(c);
    { let a0_ = (c.m.r32(sym::gBtlCamView)).wrapping_add(320); F::Vu0Cur_LoadMtx(c, a0_) };
    l_bolt_3 = (p_work_1).wrapping_add(16);
    {
        l_i_4 = 0i32;
        'l1: loop {
            if !(((((l_i_4) < (10i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if (((((c.m.r32((l_bolt_3).wrapping_add(12)) as i32)) & (1i32))) != 0) {
                    F::EftBolt_Draw(c, p_work_1, l_bolt_3, p_alpha_2);
                }
            }
            { { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; }; { let o_ = l_bolt_3; let n_ = (o_).wrapping_add(144); l_bolt_3 = n_; }; };
        }
    }
    F::Vu0Cur_Pop(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBolt_DrawAll", &PROBE_N, &[], true); }
    r_
}

pub fn EftBolt_AddFlash(c: &mut Ctx, mut p_work_1: u32, mut p_kind_2: i32, mut p_nodeA_3: i32, mut p_nodeB_4: i32, p_vel_5: u32, mut p_size_6: F32, mut p_follow_7: F32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBolt_AddFlash", &PROBE_N, &[(p_work_1) as u32, (p_kind_2) as u32, (p_nodeA_3) as u32, (p_nodeB_4) as u32, (p_vel_5) as u32, (p_size_6).0, (p_follow_7).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_f_8: u32 = 0;
    let mut l_i_9: i32 = 0;
    rt::memcpy(c, fp, p_vel_5, 16);
    let r_ = 'f: {
        l_f_8 = ((0i32) as u32);
        {
            l_i_9 = 0i32;
            'l1: loop {
                if !(((((l_i_9) < (20i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    if ((((((c.m.r32(((p_work_1).wrapping_add(1456)).wrapping_add((((l_i_9) as u32)).wrapping_mul(112))) as i32)) == (0i32)) as i32)) != 0) {
                        l_f_8 = ((p_work_1).wrapping_add(1456)).wrapping_add((((l_i_9) as u32)).wrapping_mul(112));
                        break 'l1;
                    }
                }
                { let o_ = l_i_9; let n_ = (o_).wrapping_add(1); l_i_9 = n_; };
            }
        }
        if (((((l_f_8) == (((0i32) as u32))) as i32)) != 0) {
            break 'f 0i32;
        }
        { let r_ = 1i32; let a_ = l_f_8; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        { let v_ = p_kind_2; let a_ = (l_f_8).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
        if (((((p_kind_2) == (0i32)) as i32)) != 0) {
            { let v_ = F32(0x40c00000); let a_ = (l_f_8).wrapping_add(20); c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x3e48c8c8); let a_ = (l_f_8).wrapping_add(80); c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x3df0f0f0); let a_ = (l_f_8).wrapping_add(84); c.m.w32(a_, (v_).0); };
            { let v_ = { let v_ = F32(0x3f169696); let a_ = (l_f_8).wrapping_add(88); c.m.w32(a_, (v_).0); v_ }; let a_ = (l_f_8).wrapping_add(92); c.m.w32(a_, (v_).0); };
        } else {
            { let v_ = F32(0x40400000); let a_ = (l_f_8).wrapping_add(20); c.m.w32(a_, (v_).0); };
            { let v_ = { let v_ = { let v_ = F32(0x3f008080); let a_ = (l_f_8).wrapping_add(80); c.m.w32(a_, (v_).0); v_ }; let a_ = (l_f_8).wrapping_add(84); c.m.w32(a_, (v_).0); v_ }; let a_ = (l_f_8).wrapping_add(88); c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x3ec0c0c0); let a_ = (l_f_8).wrapping_add(92); c.m.w32(a_, (v_).0); };
        }
        { let v_ = p_nodeA_3; let a_ = (l_f_8).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = p_nodeB_4; let a_ = (l_f_8).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        F::Vec4_Copy(c, (l_f_8).wrapping_add(64), fp);
        { let v_ = p_follow_7; let a_ = (l_f_8).wrapping_add(36); c.m.w32(a_, (v_).0); };
        { let v_ = p_size_6; let a_ = (l_f_8).wrapping_add(24); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fdiv(rt::fneg(rt::fmul(p_size_6, F32(0x3e4ccccc))), F32(c.m.r32((l_f_8).wrapping_add(20)))); let a_ = (l_f_8).wrapping_add(28); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fneg(rt::fdiv(F32(c.m.r32((l_f_8).wrapping_add(92))), rt::fadd(F32(c.m.r32((l_f_8).wrapping_add(20))), F32(0x3f800000)))); let a_ = (l_f_8).wrapping_add(32); c.m.w32(a_, (v_).0); };
        { let v_ = rt::rem_i32(F::Port_Rand(c), 4i32); let a_ = (l_f_8).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        { let v_ = rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x40c90fda)); let a_ = (l_f_8).wrapping_add(40); c.m.w32(a_, (v_).0); };
        if (((rt::fge(F32(c.m.r32((l_f_8).wrapping_add(40))), F32(0x40490fda)) as i32)) != 0) {
            { let r_ = F32(0x40c90fda); let a_ = (l_f_8).wrapping_add(40); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        }
        break 'f 1i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftBolt_AddFlash", &PROBE_N, &[], true); }
    r_
}

pub fn EftBolt_UpdateFlashes(c: &mut Ctx, mut p_work_1: u32, mut p_objId_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBolt_UpdateFlashes", &PROBE_N, &[(p_work_1) as u32, (p_objId_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_f_3: u32 = 0;
    let mut l_i_4: i32 = 0;
    let r_ = 'f: {
        {
            l_i_4 = 0i32;
            'l1: loop {
                if !(((((l_i_4) < (20i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_f_3 = ((p_work_1).wrapping_add(1456)).wrapping_add((((l_i_4) as u32)).wrapping_mul(112));
                    if (((((c.m.r32(l_f_3) as i32)) & (1i32))) != 0) {
                        { let a1_ = (c.m.r32((l_f_3).wrapping_add(4)) as i32); F::BtlCharApi_GetNodePos(c, p_objId_2, a1_, (l_f_3).wrapping_add(48)) };
                        { let a1_ = (c.m.r32((l_f_3).wrapping_add(8)) as i32); F::BtlCharApi_GetNodePos(c, p_objId_2, a1_, fp) };
                        F::Vec4_Sub(c, fp, fp, (l_f_3).wrapping_add(48));
                        { let r_ = rt::fmul(F32(c.m.r32(fp)), F32(c.m.r32((l_f_3).wrapping_add(36)))); let a_ = (l_f_3).wrapping_add(48); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                        { let r_ = rt::fmul(F32(c.m.r32((fp).wrapping_add(4))), F32(c.m.r32((l_f_3).wrapping_add(36)))); let a_ = ((l_f_3).wrapping_add(48)).wrapping_add(4); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                        { let r_ = rt::fmul(F32(c.m.r32((fp).wrapping_add(8))), F32(c.m.r32((l_f_3).wrapping_add(36)))); let a_ = ((l_f_3).wrapping_add(48)).wrapping_add(8); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                        F::Vec4_Add(c, (l_f_3).wrapping_add(48), (l_f_3).wrapping_add(48), (l_f_3).wrapping_add(64));
                        { let r_ = F32(c.m.r32((l_f_3).wrapping_add(32))); let a_ = (l_f_3).wrapping_add(92); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                        { let r_ = F32(c.m.r32((l_f_3).wrapping_add(28))); let a_ = (l_f_3).wrapping_add(24); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                        if (((rt::flt(F32(c.m.r32((l_f_3).wrapping_add(92))), F32(0x00000000)) as i32)) != 0) {
                            { let v_ = F32(0x00000000); let a_ = (l_f_3).wrapping_add(92); c.m.w32(a_, (v_).0); };
                        }
                        { let v_ = rt::fmul(rt::i2f(rt::rem_i32((c.m.r32((l_f_3).wrapping_add(12)) as i32), 2i32)), F32(0x3f000000)); let a_ = (l_f_3).wrapping_add(96); c.m.w32(a_, (v_).0); };
                        { let v_ = rt::fmul(rt::i2f(rt::div_i32((c.m.r32((l_f_3).wrapping_add(12)) as i32), 2i32)), F32(0x3f000000)); let a_ = ((l_f_3).wrapping_add(96)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                        { let v_ = rt::fadd(F32(c.m.r32((l_f_3).wrapping_add(96))), F32(0x3f000000)); let a_ = ((l_f_3).wrapping_add(96)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                        { let v_ = rt::fadd(F32(c.m.r32(((l_f_3).wrapping_add(96)).wrapping_add(4))), F32(0x3f000000)); let a_ = ((l_f_3).wrapping_add(96)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                        { let a_ = (l_f_3).wrapping_add(12); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        if ((((((c.m.r32((l_f_3).wrapping_add(12)) as i32)) >= (4i32)) as i32)) != 0) {
                            { let v_ = 0i32; let a_ = (l_f_3).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                        }
                        { let r_ = F32(0x3f800000); let a_ = (l_f_3).wrapping_add(20); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                        if (((rt::flt(F32(c.m.r32((l_f_3).wrapping_add(20))), F32(0x00000000)) as i32)) != 0) {
                            { let v_ = 0i32; let a_ = l_f_3; c.m.w32(a_, (v_) as u32); };
                        }
                    }
                }
                { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftBolt_UpdateFlashes", &PROBE_N, &[], true); }
    r_
}

pub fn EftBolt_DrawFlashes(c: &mut Ctx, mut p_work_1: u32, mut p_alpha_2: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBolt_DrawFlashes", &PROBE_N, &[(p_work_1) as u32, (p_alpha_2).0], false); }
    let mut l_tex_3: u32 = 0;
    let mut l_f_4: u32 = 0;
    let mut l_i_5: i32 = 0;
    let r_ = 'f: {
    l_tex_3 = (c.m.r32(sym::gEftBoltPool)).wrapping_add(616);
    {
        l_i_5 = 0i32;
        'l1: loop {
            if !(((((l_i_5) < (20i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                l_f_4 = ((p_work_1).wrapping_add(1456)).wrapping_add((((l_i_5) as u32)).wrapping_mul(112));
                if (((((c.m.r32(l_f_4) as i32)) & (1i32))) != 0) {
                    { let a0_ = ((rt::f2u(rt::fmul(F32(c.m.r32((l_f_4).wrapping_add(80))), F32(0x437f0000)))) as u8); let a1_ = ((rt::f2u(rt::fmul(F32(c.m.r32((l_f_4).wrapping_add(84))), F32(0x437f0000)))) as u8); let a2_ = ((rt::f2u(rt::fmul(F32(c.m.r32((l_f_4).wrapping_add(88))), F32(0x437f0000)))) as u8); let a3_ = ((rt::f2u(rt::fmul(rt::fmul(F32(c.m.r32((l_f_4).wrapping_add(92))), F32(0x437f0000)), p_alpha_2))) as u8); let a4_ = F32(c.m.r32((l_f_4).wrapping_add(48))); let a5_ = F32(c.m.r32(((l_f_4).wrapping_add(48)).wrapping_add(4))); let a6_ = F32(c.m.r32(((l_f_4).wrapping_add(48)).wrapping_add(8))); let a7_ = F32(c.m.r32((l_f_4).wrapping_add(96))); let a8_ = F32(c.m.r32(((l_f_4).wrapping_add(96)).wrapping_add(4))); let a9_ = F32(c.m.r32(((l_f_4).wrapping_add(96)).wrapping_add(8))); let a10_ = F32(c.m.r32(((l_f_4).wrapping_add(96)).wrapping_add(12))); let a11_ = F32(c.m.r32((l_f_4).wrapping_add(40))); let a17_ = rt::f2u(rt::fmul(F32(c.m.r32((l_f_4).wrapping_add(24))), F32(0x45800000))); let a20_ = (l_tex_3).wrapping_add(((((c.m.r32((l_f_4).wrapping_add(16)) as i32)) as u32)).wrapping_mul(16)); F::EftSpr_DrawRot(c, a0_, a1_, a2_, a3_, a4_, a5_, a6_, a7_, a8_, a9_, a10_, a11_, 0i32, 0i32, ((((64i32) as i64 as u64)) as u32), ((((64i32) as i64 as u64)) as u32), 0i32, a17_, 0i32, 1i32, a20_) };
                }
            }
            { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBolt_DrawFlashes", &PROBE_N, &[], true); }
    r_
}

pub fn EftBolt_Spawn(c: &mut Ctx, mut p_work_1: u32, mut p_objId_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBolt_Spawn", &PROBE_N, &[(p_work_1) as u32, (p_objId_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(240) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 240);
    let mut l_n_3: i32 = 0;
    let mut l_count_4: i32 = 0;
    let mut l_side_5: i32 = 0;
    let mut l_spawned_6: i32 = 0;
    let mut l_bolt_7: u32 = 0;
    let mut l_pair_8: u32 = 0;
    let mut l_node_9: u32 = 0;
    let mut l_t_10: F32 = F32(0);
    let mut l_kind_11: i32 = 0;
    let mut l_nodeA_12: i32 = 0;
    let mut l_nodeB_13: i32 = 0;
    let mut l_i_14: i32 = 0;
    let mut l_left_15: i32 = 0;
    let mut l_out_16: u32 = 0;
    let mut l_ang_17: F32 = F32(0);
    let mut l_dAng_18: F32 = F32(0);
    let mut l_turn_19: F32 = F32(0);
    let mut l_follow_20: F32 = F32(0);
    let mut l_length_21: F32 = F32(0);
    let mut l_size_22: F32 = F32(0);
    let r_ = 'f: {
        l_n_3 = 0i32;
        l_count_4 = 0i32;
        l_side_5 = 0i32;
        l_spawned_6 = 0i32;
        if ((F::EftAura_IsVisible(c, p_objId_2)) != 0) {
            if ((((((((((F::EftAura_IsBurstRising(c, p_objId_2)) != 0) || ((((((c.m.r32((p_work_1).wrapping_add(3704)) as i32)) <= (0i32)) as i32)) != 0)) as i32)) != 0) || (((((rt::rem_i32(F::Port_Rand(c), 12i32)) == (0i32)) as i32)) != 0)) as i32)) != 0) {
                l_count_4 = 3i32;
            }
        } else {
            if ((F::EftGlow_IsActive(c, p_objId_2)) != 0) {
                if ((((((((((c.m.r32((p_work_1).wrapping_add(3704)) as i32)) <= (0i32)) as i32)) != 0) || (((((rt::rem_i32(F::Port_Rand(c), 6i32)) == (0i32)) as i32)) != 0)) as i32)) != 0) {
                    l_count_4 = 3i32;
                }
            } else {
                if ((((((((((c.m.r32((p_work_1).wrapping_add(3704)) as i32)) <= (0i32)) as i32)) != 0) || (((((rt::rem_i32(F::Port_Rand(c), 18i32)) == (0i32)) as i32)) != 0)) as i32)) != 0) {
                    l_count_4 = 3i32;
                }
            }
        }
        if (((((c.m.r32((p_work_1).wrapping_add(3696)) as i32)) & (8i32))) != 0) {
            if (((((rt::rem_i32(F::Port_Rand(c), 20i32)) == (0i32)) as i32)) != 0) {
                l_count_4 = 3i32;
            }
        }
        if (((((l_count_4) > (0i32)) as i32)) != 0) {
            l_spawned_6 = 1i32;
            l_side_5 = (c.m.r32((p_work_1).wrapping_add(3700)) as i32);
            { let v_ = ((l_side_5) ^ (1i32)); let a_ = (p_work_1).wrapping_add(3700); c.m.w32(a_, (v_) as u32); };
            if ((((((c.m.r32((p_work_1).wrapping_add(3708)) as i32)) == (0i32)) as i32)) != 0) {
                {
                    l_i_14 = 0i32;
                    'l1: loop {
                        if !(((((l_i_14) < (4i32)) as i32)) != 0) { break 'l1; }
                        'c2: {
                            { let v_ = l_i_14; let a_ = (fp.wrapping_add(12)).wrapping_add((((l_i_14) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                        }
                        { let o_ = l_i_14; let n_ = (o_).wrapping_add(1); l_i_14 = n_; };
                    }
                }
                {
                    { l_left_15 = 4i32; l_out_16 = (p_work_1).wrapping_add(3712); };
                    'l3: loop {
                        if !(((((l_left_15) != (0i32)) as i32)) != 0) { break 'l3; }
                        'c4: {
                            l_n_3 = rt::rem_i32(F::Port_Rand(c), l_left_15);
                            { let v_ = (c.m.r32((fp.wrapping_add(12)).wrapping_add((((l_n_3) as u32)).wrapping_mul(4))) as i32); let a_ = l_out_16; c.m.w32(a_, (v_) as u32); };
                            {
                                l_i_14 = l_n_3;
                                'l5: loop {
                                    if !(((((l_i_14) < ((l_left_15).wrapping_sub(1i32))) as i32)) != 0) { break 'l5; }
                                    'c6: {
                                        { let v_ = (c.m.r32((fp.wrapping_add(12)).wrapping_add(((((l_i_14).wrapping_add(1i32)) as u32)).wrapping_mul(4))) as i32); let a_ = (fp.wrapping_add(12)).wrapping_add((((l_i_14) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                    }
                                    { let o_ = l_i_14; let n_ = (o_).wrapping_add(1); l_i_14 = n_; };
                                }
                            }
                            { let o_ = l_out_16; let n_ = (o_).wrapping_add(4); l_out_16 = n_; };
                        }
                        { let o_ = l_left_15; let n_ = (o_).wrapping_sub(1); l_left_15 = n_; };
                    }
                }
            }
            { let v_ = { let a_ = ((p_work_1).wrapping_add(3712)).wrapping_add(((((c.m.r32((p_work_1).wrapping_add(3708)) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            if (((((l_side_5) != (0i32)) as i32)) != 0) {
                { let v_ = 5i32; let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = 6i32; let a_ = (fp).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            } else {
                { let v_ = 4i32; let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = 7i32; let a_ = (fp).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            }
            l_n_3 = 0i32;
        }
        'l7: while (((((l_count_4) != (0i32)) as i32)) != 0) {
            l_bolt_7 = ((0i32) as u32);
            {
                l_i_14 = 0i32;
                'l8: loop {
                    if !(((((l_i_14) < (10i32)) as i32)) != 0) { break 'l8; }
                    'c9: {
                        if ((((((c.m.r32((((p_work_1).wrapping_add(16)).wrapping_add((((l_i_14) as u32)).wrapping_mul(144))).wrapping_add(12)) as i32)) == (0i32)) as i32)) != 0) {
                            l_bolt_7 = ((p_work_1).wrapping_add(16)).wrapping_add((((l_i_14) as u32)).wrapping_mul(144));
                            break 'l8;
                        }
                    }
                    { let o_ = l_i_14; let n_ = (o_).wrapping_add(1); l_i_14 = n_; };
                }
            }
            if (((((l_bolt_7) == (((0i32) as u32))) as i32)) != 0) {
                break 'l7;
            }
            l_kind_11 = (c.m.r32((fp).wrapping_add((((rt::rem_i32(l_n_3, 3i32)) as u32)).wrapping_mul(4))) as i32);
            { let o_ = l_n_3; let n_ = (o_).wrapping_add(1); l_n_3 = n_; };
            if (((((l_side_5) == (0i32)) as i32)) != 0) {
                l_ang_17 = F32(c.m.r32(((p_work_1).wrapping_add(3728)).wrapping_add((((0i32) as u32)).wrapping_mul(4))));
                l_dAng_18 = rt::fmul(rt::fneg(rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x3eb33333)), F32(0x3d4ccccc))), F32(0x40490fda));
                l_turn_19 = rt::fneg(rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x3f20d97a)), F32(0x3fc90fda)));
            } else {
                l_ang_17 = F32(c.m.r32(((p_work_1).wrapping_add(3728)).wrapping_add((((1i32) as u32)).wrapping_mul(4))));
                l_dAng_18 = rt::fmul(rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x3eb33333)), F32(0x3d4ccccc)), F32(0x40490fda));
                l_turn_19 = rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x3f20d97a)), F32(0x3fc90fda));
            }
            l_nodeA_12 = { let a_ = ({ let a_ = (c.m.r32(sym::gEftBoltPool)).wrapping_add(1140); c.m.r32(a_) }).wrapping_add((((l_kind_11) as u32)).wrapping_mul(20)); (c.m.r32(a_) as i32) };
            l_nodeB_13 = { let a_ = (({ let a_ = (c.m.r32(sym::gEftBoltPool)).wrapping_add(1140); c.m.r32(a_) }).wrapping_add((((l_kind_11) as u32)).wrapping_mul(20))).wrapping_add(4); (c.m.r32(a_) as i32) };
            l_follow_20 = rt::fadd({ let a_ = (({ let a_ = (c.m.r32(sym::gEftBoltPool)).wrapping_add(1140); c.m.r32(a_) }).wrapping_add((((l_kind_11) as u32)).wrapping_mul(20))).wrapping_add(12); F32(c.m.r32(a_)) }, rt::fmul({ let a_ = (({ let a_ = (c.m.r32(sym::gEftBoltPool)).wrapping_add(1140); c.m.r32(a_) }).wrapping_add((((l_kind_11) as u32)).wrapping_mul(20))).wrapping_add(16); F32(c.m.r32(a_)) }, rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff))));
            if ((F::EftGlow_IsActive(c, p_objId_2)) != 0) {
                l_length_21 = rt::fmul(rt::fmul({ let a_ = (({ let a_ = (c.m.r32(sym::gEftBoltPool)).wrapping_add(1140); c.m.r32(a_) }).wrapping_add((((l_kind_11) as u32)).wrapping_mul(20))).wrapping_add(8); F32(c.m.r32(a_)) }, F32(0x3fb33333)), F32(c.m.r32((p_work_1).wrapping_add(3744))));
            } else {
                l_length_21 = rt::fmul({ let a_ = (({ let a_ = (c.m.r32(sym::gEftBoltPool)).wrapping_add(1140); c.m.r32(a_) }).wrapping_add((((l_kind_11) as u32)).wrapping_mul(20))).wrapping_add(8); F32(c.m.r32(a_)) }, F32(c.m.r32((p_work_1).wrapping_add(3744))));
            }
            { let v_ = F32(0x00000000); let a_ = fp.wrapping_add(92); c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x3f800000); let a_ = (fp.wrapping_add(92)).wrapping_add(4); c.m.w32(a_, (v_).0); };
            if (((((l_kind_11) < (4i32)) as i32)) != 0) {
                { let v_ = rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x3e999999)), F32(0xbf666666)); let a_ = (fp.wrapping_add(92)).wrapping_add(8); c.m.w32(a_, (v_).0); };
            } else {
                { let v_ = rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x3dcccccc)), F32(0xbf199999)); let a_ = (fp.wrapping_add(92)).wrapping_add(8); c.m.w32(a_, (v_).0); };
            }
            { let v_ = F32(0x3f800000); let a_ = (fp.wrapping_add(92)).wrapping_add(12); c.m.w32(a_, (v_).0); };
            { let r_ = l_dAng_18; let n_ = rt::fadd(l_ang_17, r_); l_ang_17 = n_; };
            F::Vec3_Normalize(c, fp.wrapping_add(92), fp.wrapping_add(92));
            F::Mtx_StoreIdentity(c, fp.wrapping_add(28));
            if (((rt::fge(l_ang_17, F32(0x40490fda)) as i32)) != 0) {
                { let r_ = F32(0x40c90fda); let n_ = rt::fsub(l_ang_17, r_); l_ang_17 = n_; };
            } else {
                if (((rt::fle(l_ang_17, F32(0xc0490fda)) as i32)) != 0) {
                    { let r_ = F32(0x40c90fda); let n_ = rt::fadd(l_ang_17, r_); l_ang_17 = n_; };
                }
            }
            F::Mtx_RotateZ(c, fp.wrapping_add(28), fp.wrapping_add(28), l_ang_17);
            F::Mtx_MulVec4(c, fp.wrapping_add(124), fp.wrapping_add(28), fp.wrapping_add(92));
            { let v_ = F32(0x00000000); let a_ = fp.wrapping_add(108); c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x3f800000); let a_ = (fp.wrapping_add(108)).wrapping_add(4); c.m.w32(a_, (v_).0); };
            if (((((l_kind_11) < (4i32)) as i32)) != 0) {
                { let v_ = rt::fsub(F32(0x3ecccccc), rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x3e800000))); let a_ = (fp.wrapping_add(108)).wrapping_add(8); c.m.w32(a_, (v_).0); };
            } else {
                { let v_ = rt::fsub(F32(0x3f000000), rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x3e4ccccc))); let a_ = (fp.wrapping_add(108)).wrapping_add(8); c.m.w32(a_, (v_).0); };
            }
            { let v_ = F32(0x3f800000); let a_ = (fp.wrapping_add(108)).wrapping_add(12); c.m.w32(a_, (v_).0); };
            { let r_ = l_turn_19; let n_ = rt::fadd(l_ang_17, r_); l_ang_17 = n_; };
            F::Vec3_Normalize(c, fp.wrapping_add(108), fp.wrapping_add(108));
            F::Mtx_StoreIdentity(c, fp.wrapping_add(28));
            if (((rt::fge(l_ang_17, F32(0x40490fda)) as i32)) != 0) {
                { let r_ = F32(0x40c90fda); let n_ = rt::fsub(l_ang_17, r_); l_ang_17 = n_; };
            } else {
                if (((rt::fle(l_ang_17, F32(0xc0490fda)) as i32)) != 0) {
                    { let r_ = F32(0x40c90fda); let n_ = rt::fadd(l_ang_17, r_); l_ang_17 = n_; };
                }
            }
            F::Mtx_RotateZ(c, fp.wrapping_add(28), fp.wrapping_add(28), l_ang_17);
            F::Mtx_MulVec4(c, fp.wrapping_add(140), fp.wrapping_add(28), fp.wrapping_add(108));
            F::BtlCharApi_GetNodePos(c, p_objId_2, l_nodeA_12, fp.wrapping_add(156));
            F::BtlCharApi_GetNodePos(c, p_objId_2, l_nodeB_13, fp.wrapping_add(172));
            F::Vec4_Sub(c, fp.wrapping_add(188), fp.wrapping_add(156), fp.wrapping_add(172));
            F::Vec3_Normalize(c, fp.wrapping_add(188), fp.wrapping_add(188));
            F::Mtx_StoreIdentity(c, fp.wrapping_add(28));
            { let v_ = { let a0_ = F32(c.m.r32((fp.wrapping_add(188)).wrapping_add(4))); F::Port_asinf(c, a0_) }; let a_ = fp.wrapping_add(204); c.m.w32(a_, (v_).0); };
            { let a2_ = F32(c.m.r32(fp.wrapping_add(204))); F::Mtx_RotateX(c, fp.wrapping_add(28), fp.wrapping_add(28), a2_) };
            { let v_ = rt::fadd({ let a0_ = F32(c.m.r32(fp.wrapping_add(188))); let a1_ = F32(c.m.r32((fp.wrapping_add(188)).wrapping_add(8))); F::Port_atan2f(c, a0_, a1_) }, F32(0x40490fda)); let a_ = (fp.wrapping_add(204)).wrapping_add(4); c.m.w32(a_, (v_).0); };
            if (((rt::fge(F32(c.m.r32((fp.wrapping_add(204)).wrapping_add(4))), F32(0x40490fda)) as i32)) != 0) {
                { let r_ = F32(0x40c90fda); let a_ = (fp.wrapping_add(204)).wrapping_add(4); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            }
            { let a2_ = F32(c.m.r32((fp.wrapping_add(204)).wrapping_add(4))); F::Mtx_RotateY(c, fp.wrapping_add(28), fp.wrapping_add(28), a2_) };
            F::Mtx_MulVec4(c, fp.wrapping_add(124), fp.wrapping_add(28), fp.wrapping_add(124));
            F::Vec3_Normalize(c, fp.wrapping_add(124), fp.wrapping_add(124));
            F::Mtx_MulVec4(c, fp.wrapping_add(140), fp.wrapping_add(28), fp.wrapping_add(140));
            F::Vec3_Normalize(c, fp.wrapping_add(140), fp.wrapping_add(140));
            if ((((!((F::EftBolt_Start(c, p_work_1, l_bolt_7, p_objId_2, l_kind_11, l_nodeA_12, l_nodeB_13, fp.wrapping_add(124), fp.wrapping_add(140), l_length_21, l_follow_20)) != 0)) as i32)) != 0) {
                break 'l7;
            }
            F::Vec4_Scale(c, fp.wrapping_add(220), fp.wrapping_add(124), rt::fmul(l_length_21, F32(0x3f4ccccc)));
            if ((F::EftGlow_IsActive(c, p_objId_2)) != 0) {
                l_size_22 = rt::fmul(rt::fmul(rt::fmul(rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x3e800000)), F32(0x3f333333)), F32(0x3ed70a3d)), F32(c.m.r32((p_work_1).wrapping_add(3744)))), F32(0x3fb33333));
            } else {
                l_size_22 = rt::fmul(rt::fmul(rt::fadd(rt::fmul(rt::fdiv(rt::i2f(F::Port_Rand(c)), F32(0x4effffff)), F32(0x3e800000)), F32(0x3f333333)), F32(0x3ed70a3d)), F32(c.m.r32((p_work_1).wrapping_add(3744))));
            }
            { let _ = F::EftBolt_AddFlash(c, p_work_1, 0i32, l_nodeA_12, l_nodeB_13, fp.wrapping_add(220), l_size_22, l_follow_20); };
            if (((((l_side_5) == (0i32)) as i32)) != 0) {
                { let v_ = l_ang_17; let a_ = ((p_work_1).wrapping_add(3728)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            } else {
                { let v_ = l_ang_17; let a_ = ((p_work_1).wrapping_add(3728)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            }
            { let o_ = l_count_4; let n_ = (o_).wrapping_sub(1); l_count_4 = n_; };
            { let r_ = 1i32; let n_ = ((l_side_5) ^ (r_)); l_side_5 = n_; };
        }
        if ((l_spawned_6) != 0) {
            { let a_ = (p_work_1).wrapping_add(3708); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
            if ((((((c.m.r32((p_work_1).wrapping_add(3708)) as i32)) >= (4i32)) as i32)) != 0) {
                { let v_ = 0i32; let a_ = (p_work_1).wrapping_add(3708); c.m.w32(a_, (v_) as u32); };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftBolt_Spawn", &PROBE_N, &[], true); }
    r_
}
