#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn EftRingShot_AllocShot(c: &mut Ctx, mut p_w_1: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRingShot_AllocShot", &PROBE_N, &[(p_w_1) as u32], false); }
    let mut l_shot_2: u32 = 0;
    let mut l_found_3: u32 = 0;
    let mut l_i_4: i32 = 0;
    let r_ = 'f: {
    l_shot_2 = (p_w_1).wrapping_add(1520);
    l_found_3 = ((0i32) as u32);
    l_i_4 = 0i32;
    'l1: while (((((l_i_4) < (20i32)) as i32)) != 0) {
        { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
        if (((((c.m.r32((l_shot_2).wrapping_add(100))) == (((0i32) as u32))) as i32)) != 0) {
            l_found_3 = l_shot_2;
            { let _ = F::memset(c, l_found_3, 0i32, 112u32); };
            break 'l1;
        }
        { let o_ = l_shot_2; let n_ = (o_).wrapping_add(112); l_shot_2 = n_; };
    }
    break 'f l_found_3;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftRingShot_AllocShot", &PROBE_N, &[], true); }
    r_
}

pub fn EftRingShot_Fire(c: &mut Ctx, mut p_objId_1: i32, mut p_task_2: u32, mut p_node_3: i32, mut p_arg3_4: i32, mut p_volley_5: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRingShot_Fire", &PROBE_N, &[(p_objId_1) as u32, (p_task_2) as u32, (p_node_3) as u32, (p_arg3_4) as u32, (p_volley_5) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(400) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 400);
    let mut l_w_6: u32 = 0;
    let mut l_arg_7: u32 = 0;
    let mut l_def_8: u32 = 0;
    let mut l_count_9: i32 = 0;
    let mut l_i_10: i32 = 0;
    let mut l_t_11: F32 = F32(0);
    let mut l_h_12: F32 = F32(0);
    let mut l_scale_13: F32 = F32(0);
    let mut l_r_14: F32 = F32(0);
    let mut l_r0_15: F32 = F32(0);
    let mut l_shot_16: u32 = 0;
    let mut l_opp_17: i32 = 0;
    let mut l_half_18: i32 = 0;
    let mut l_inner_19: i32 = 0;
    let mut l_outer_20: i32 = 0;
    let mut l_home_21: u32 = 0;
    let mut l_from_22: u32 = 0;
    let mut l_half_23: i32 = 0;
    let mut l_inner_24: i32 = 0;
    let mut l_outer_25: i32 = 0;
    let mut l_three_26: i32 = 0;
    let r_ = 'f: {
        l_w_6 = c.m.r32((p_task_2).wrapping_add(56));
        l_arg_7 = c.m.r32((l_w_6).wrapping_add(1316));
        l_def_8 = c.m.r32((l_arg_7).wrapping_add(36));
        l_count_9 = 1i32;
        if ((p_volley_5) != 0) {
            l_count_9 = (((c.m.r16((l_def_8).wrapping_add(68)) as i16)) as i32);
            if (((((l_count_9) <= (0i32)) as i32)) != 0) {
                l_count_9 = 1i32;
            }
        }
        {
            l_i_10 = 0i32;
            'l1: loop {
                if !(((((l_i_10) < (l_count_9)) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_shot_16 = F::EftRingShot_AllocShot(c, l_w_6);
                    if (((((l_shot_16) != (((0i32) as u32))) as i32)) != 0) {
                        l_t_11 = rt::fdiv(rt::i2f({ let a_ = (c.m.r32((l_arg_7).wrapping_add(36))).wrapping_add(40); (c.m.r32(a_) as i32) }), F32(0x41f00000));
                        { let s_ = { rt::memset(c, fp.wrapping_add(352), 0, 48);; { let v_ = l_arg_7; c.m.w32(fp.wrapping_add(352).wrapping_add(0), v_); }; { let v_ = c.m.r32((l_w_6).wrapping_add(1312)); c.m.w32(fp.wrapping_add(352).wrapping_add(4), v_); }; { let v_ = (l_w_6).wrapping_add(720); c.m.w32(fp.wrapping_add(352).wrapping_add(8), v_); }; { let v_ = l_shot_16; c.m.w32(fp.wrapping_add(352).wrapping_add(12), v_); }; { let v_ = (l_shot_16).wrapping_add(16); c.m.w32(fp.wrapping_add(352).wrapping_add(16), v_); }; { let v_ = (c.m.r32((l_w_6).wrapping_add(3760)) as i32); c.m.w32(fp.wrapping_add(352).wrapping_add(20), (v_) as u32); }; { let v_ = (c.m.r32((l_w_6).wrapping_add(3764)) as i32); c.m.w32(fp.wrapping_add(352).wrapping_add(24), (v_) as u32); }; { let v_ = p_node_3; c.m.w32(fp.wrapping_add(352).wrapping_add(28), (v_) as u32); }; { let v_ = p_arg3_4; c.m.w32(fp.wrapping_add(352).wrapping_add(32), (v_) as u32); }; { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(352).wrapping_add(36), (v_).0); }; { let v_ = F32(c.m.r32((l_w_6).wrapping_add(1492))); c.m.w32(fp.wrapping_add(352).wrapping_add(40), (v_).0); }; { let v_ = F32(c.m.r32((l_w_6).wrapping_add(1480))); c.m.w32(fp.wrapping_add(352).wrapping_add(44), (v_).0); }; fp.wrapping_add(352) }; let d_ = fp; rt::memcpy(c, d_, s_, 48); };
                        { let v_ = (c.m.r32((l_w_6).wrapping_add(3760)) as i32); let a_ = (l_shot_16).wrapping_add(84); c.m.w32(a_, (v_) as u32); };
                        F::Vec4_Copy(c, (l_shot_16).wrapping_add(16), (l_w_6).wrapping_add(1392));
                        's3: {
                            let sv27: i32 = (c.m.r32((l_w_6).wrapping_add(1476)) as i32);
                            match sv27 {
                                0i32 | 1i32 => {
                                    { let a1_ = (c.m.r32((l_w_6).wrapping_add(800)) as i32); F::BtlCharApi_GetNodePos(c, p_objId_1, a1_, l_shot_16) };
                                    if ((((((c.m.r32((l_w_6).wrapping_add(3760)) as i32)) == (0i32)) as i32)) != 0) {
                                        { let r_ = ((4i32) as u32); let a_ = (l_shot_16).wrapping_add(80); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
                                    }
                                    break 's3;
                                }
                                2i32 => {
                                    { let v_ = l_t_11; let a_ = (fp).wrapping_add(36); c.m.w32(a_, (v_).0); };
                                    F::Vec4_Copy(c, l_shot_16, (l_w_6).wrapping_add(1344));
                                    if ((((!((((((l_i_10).wrapping_add(1i32)) < (l_count_9)) as i32)) != 0)) as i32)) != 0) {
                                        { let r_ = ((4i32) as u32); let a_ = (l_shot_16).wrapping_add(80); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
                                    }
                                    break 's3;
                                }
                                _ => {
                                }
                            }
                        }
                        's4: {
                            let sv28: i32 = (c.m.r32((l_w_6).wrapping_add(1476)) as i32);
                            match sv28 {
                                0i32 => {
                                    {
                                        l_t_11 = F32(0x3f333333);
                                        l_h_12 = F32(0x3f800000);
                                        l_opp_17 = { let a0_ = (c.m.r32(l_arg_7) as i32); F::BtlCharApi_GetOpponentObjId(c, a0_) };
                                        l_half_18 = rt::div_i32((c.m.r32((l_w_6).wrapping_add(3764)) as i32), 2i32);
                                        l_r0_15 = l_h_12;
                                        l_inner_19 = rt::f2i(rt::fmul(rt::i2f(l_half_18), F32(0x3ecccccc)));
                                        if (((((l_inner_19) <= (0i32)) as i32)) != 0) {
                                            l_inner_19 = 1i32;
                                        }
                                        F::BtlCharApi_GetNodePos(c, l_opp_17, 17i32, fp.wrapping_add(48));
                                        l_scale_13 = F::BtlScene_GetCharScale(c, l_opp_17);
                                        l_outer_20 = (l_half_18).wrapping_sub(l_inner_19);
                                        if (((rt::flt(l_scale_13, l_t_11) as i32)) != 0) {
                                            l_scale_13 = l_t_11;
                                        }
                                        F::Vec4_Set(c, fp.wrapping_add(64), F32(0x00000000), F32(0x00000000), F32(0xbf800000), l_h_12);
                                        F::Mtx_StoreIdentity(c, fp.wrapping_add(96));
                                        if ((((((c.m.r32((l_w_6).wrapping_add(3760)) as i32)) < (l_half_18)) as i32)) != 0) {
                                            if ((((((c.m.r32((l_w_6).wrapping_add(3760)) as i32)) < (l_inner_19)) as i32)) != 0) {
                                                l_h_12 = F32(0x40000000);
                                                l_r0_15 = F32(0x3f000000);
                                                l_t_11 = rt::fmul(rt::fdiv(rt::i2f((c.m.r32((l_w_6).wrapping_add(3760)) as i32)), rt::i2f(l_inner_19)), F32(0x40c90fda));
                                            } else {
                                                l_h_12 = l_t_11;
                                                l_t_11 = rt::fmul(rt::fdiv(rt::i2f(((c.m.r32((l_w_6).wrapping_add(3760)) as i32)).wrapping_sub(l_inner_19)), rt::i2f(l_outer_20)), F32(0x40c90fda));
                                            }
                                        } else {
                                            if (((((((c.m.r32((l_w_6).wrapping_add(3760)) as i32)).wrapping_sub(l_half_18)) < (l_inner_19)) as i32)) != 0) {
                                                l_h_12 = F32(0xc0000000);
                                                l_r0_15 = F32(0x3f000000);
                                                l_t_11 = rt::fmul(rt::fdiv(rt::i2f((((c.m.r32((l_w_6).wrapping_add(3760)) as i32)).wrapping_sub(l_half_18)).wrapping_add(1i32)), rt::i2f(l_inner_19)), F32(0x40c90fda));
                                            } else {
                                                l_h_12 = F32(0xbf333333);
                                                l_t_11 = rt::fmul(rt::fdiv(rt::i2f((((c.m.r32((l_w_6).wrapping_add(3760)) as i32)).wrapping_sub((l_half_18).wrapping_add(l_inner_19))).wrapping_add(1i32)), rt::i2f(l_outer_20)), F32(0x40c90fda));
                                            }
                                        }
                                        l_home_21 = (l_shot_16).wrapping_add(48);
                                        { let a2_ = F::EftMath_WrapAngle(c, l_t_11); F::Mtx_RotateY(c, fp.wrapping_add(96), fp.wrapping_add(96), a2_) };
                                        F::Mtx_MulVec4(c, fp.wrapping_add(64), fp.wrapping_add(96), fp.wrapping_add(64));
                                        l_r_14 = F32(0x41200000);
                                        l_from_22 = (l_shot_16).wrapping_add(64);
                                        F::Vec3_Sub(c, fp.wrapping_add(80), fp.wrapping_add(48), (l_w_6).wrapping_add(1328));
                                        l_t_11 = { let a0_ = F32(c.m.r32(fp.wrapping_add(80))); let a1_ = F32(c.m.r32((fp.wrapping_add(80)).wrapping_add(8))); F::Port_atan2f(c, a0_, a1_) };
                                        F::Mtx_StoreIdentity(c, fp.wrapping_add(96));
                                        F::Mtx_RotateY(c, fp.wrapping_add(96), fp.wrapping_add(96), l_t_11);
                                        F::Mtx_MulVec4(c, fp.wrapping_add(64), fp.wrapping_add(96), fp.wrapping_add(64));
                                        { let r_ = rt::fmul(rt::fmul(l_scale_13, F32(0x428c0000)), l_r0_15); let a_ = fp.wrapping_add(64); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                                        { let r_ = rt::fmul(rt::fmul(l_scale_13, F32(0x420c0000)), l_h_12); let a_ = (fp.wrapping_add(64)).wrapping_add(4); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                                        { let r_ = rt::fmul(rt::fmul(l_scale_13, F32(0x428c0000)), l_r0_15); let a_ = (fp.wrapping_add(64)).wrapping_add(8); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                                        F::Vec3_Add(c, l_home_21, fp.wrapping_add(48), fp.wrapping_add(64));
                                        F::Vec4_Copy(c, fp.wrapping_add(64), l_home_21);
                                        { let v_ = rt::fmul(F::BtlScene_RandF(c), F32(0x40490fda)); let a_ = (l_shot_16).wrapping_add(88); c.m.w32(a_, (v_).0); };
                                        { let r_ = rt::fmul({ let a0_ = F32(c.m.r32((l_shot_16).wrapping_add(88))); F::Port_sinf(c, a0_) }, l_r_14); let a_ = (fp.wrapping_add(64)).wrapping_add(4); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                                        F::Vec3_Sub(c, l_from_22, fp.wrapping_add(64), (l_w_6).wrapping_add(1328));
                                        { let v_ = rt::fdiv({ let a0_ = F::Vec3_Dot(c, l_from_22, l_from_22); F::Port_sqrtf(c, a0_) }, F32(c.m.r32((l_def_8).wrapping_add(52)))); let a_ = (l_shot_16).wrapping_add(96); c.m.w32(a_, (v_).0); };
                                        if (((rt::flt(F32(c.m.r32((l_shot_16).wrapping_add(96))), l_r_14) as i32)) != 0) {
                                            { let v_ = l_r_14; let a_ = (l_shot_16).wrapping_add(96); c.m.w32(a_, (v_).0); };
                                        }
                                        { let v_ = F32(0x00000000); let a_ = (l_shot_16).wrapping_add(92); c.m.w32(a_, (v_).0); };
                                        break 's4;
                                    }
                                }
                                1i32 => {
                                    {
                                        l_half_23 = rt::div_i32((c.m.r32((l_w_6).wrapping_add(3764)) as i32), 2i32);
                                        l_h_12 = F32(0x3f800000);
                                        l_r_14 = l_h_12;
                                        l_inner_24 = rt::f2i(rt::fmul(rt::i2f(l_half_23), F32(0x3ecccccc)));
                                        if (((((l_inner_24) <= (0i32)) as i32)) != 0) {
                                            l_inner_24 = 1i32;
                                        }
                                        { let a0_ = (c.m.r32(l_arg_7) as i32); F::BtlCharApi_GetNodePos(c, a0_, 17i32, fp.wrapping_add(224)) };
                                        l_outer_25 = (l_half_23).wrapping_sub(l_inner_24);
                                        l_scale_13 = { let a0_ = (c.m.r32(l_arg_7) as i32); F::BtlScene_GetCharScale(c, a0_) };
                                        if (((rt::flt(l_scale_13, F32(0x3f333333)) as i32)) != 0) {
                                            l_scale_13 = F32(0x3f333333);
                                        }
                                        F::Vec4_Set(c, fp.wrapping_add(240), F32(0x00000000), F32(0x00000000), F32(0xbf800000), l_h_12);
                                        F::Mtx_StoreIdentity(c, fp.wrapping_add(160));
                                        if ((((((c.m.r32((l_w_6).wrapping_add(3760)) as i32)) < (l_half_23)) as i32)) != 0) {
                                            if ((((((c.m.r32((l_w_6).wrapping_add(3760)) as i32)) < (l_inner_24)) as i32)) != 0) {
                                                l_h_12 = F32(0x40000000);
                                                l_r_14 = F32(0x00000000);
                                                l_t_11 = rt::fmul(rt::fdiv(rt::i2f((c.m.r32((l_w_6).wrapping_add(3760)) as i32)), rt::i2f(l_inner_24)), F32(0x40c90fda));
                                                if (((((l_inner_24) >= (2i32)) as i32)) != 0) {
                                                    l_r_14 = F32(0x3f000000);
                                                }
                                            } else {
                                                l_t_11 = rt::fmul(rt::fdiv(rt::i2f(((c.m.r32((l_w_6).wrapping_add(3760)) as i32)).wrapping_sub(l_inner_24)), rt::i2f(l_outer_25)), F32(0x40c90fda));
                                            }
                                        } else {
                                            if (((((((c.m.r32((l_w_6).wrapping_add(3760)) as i32)).wrapping_sub(l_half_23)) < (l_inner_24)) as i32)) != 0) {
                                                l_h_12 = F32(0xc0000000);
                                                l_r_14 = F32(0x00000000);
                                                l_t_11 = rt::fmul(rt::fdiv(rt::i2f((((c.m.r32((l_w_6).wrapping_add(3760)) as i32)).wrapping_sub(l_half_23)).wrapping_add(1i32)), rt::i2f(l_inner_24)), F32(0x40c90fda));
                                                if (((((l_inner_24) >= (2i32)) as i32)) != 0) {
                                                    l_r_14 = F32(0x3f000000);
                                                }
                                            } else {
                                                l_h_12 = F32(0xbf800000);
                                                l_t_11 = rt::fmul(rt::fdiv(rt::i2f((((c.m.r32((l_w_6).wrapping_add(3760)) as i32)).wrapping_sub((l_half_23).wrapping_add(l_inner_24))).wrapping_add(1i32)), rt::i2f(l_outer_25)), F32(0x40c90fda));
                                            }
                                        }
                                        { let a2_ = F::EftMath_WrapAngle(c, l_t_11); F::Mtx_RotateY(c, fp.wrapping_add(160), fp.wrapping_add(160), a2_) };
                                        F::Mtx_MulVec4(c, fp.wrapping_add(240), fp.wrapping_add(160), fp.wrapping_add(240));
                                        F::Vec3_Sub(c, fp.wrapping_add(256), fp.wrapping_add(224), (l_w_6).wrapping_add(1328));
                                        l_t_11 = { let a0_ = F32(c.m.r32(fp.wrapping_add(256))); let a1_ = F32(c.m.r32((fp.wrapping_add(256)).wrapping_add(8))); F::Port_atan2f(c, a0_, a1_) };
                                        F::Mtx_StoreIdentity(c, fp.wrapping_add(160));
                                        F::Mtx_RotateY(c, fp.wrapping_add(160), fp.wrapping_add(160), l_t_11);
                                        F::Mtx_MulVec4(c, fp.wrapping_add(240), fp.wrapping_add(160), fp.wrapping_add(240));
                                        { let r_ = rt::fmul(rt::fmul(l_scale_13, F32(0x41f00000)), l_r_14); let a_ = fp.wrapping_add(240); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                                        { let r_ = rt::fmul(rt::fmul(l_scale_13, F32(0x41700000)), l_h_12); let a_ = (fp.wrapping_add(240)).wrapping_add(4); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                                        { let r_ = rt::fmul(rt::fmul(l_scale_13, F32(0x41f00000)), l_r_14); let a_ = (fp.wrapping_add(240)).wrapping_add(8); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                                        F::Vec3_Add(c, (l_shot_16).wrapping_add(48), fp.wrapping_add(224), fp.wrapping_add(240));
                                        F::Vec4_Copy(c, fp.wrapping_add(240), (l_shot_16).wrapping_add(48));
                                        { let v_ = rt::fmul(F::BtlScene_RandF(c), F32(0x40490fda)); let a_ = (l_shot_16).wrapping_add(88); c.m.w32(a_, (v_).0); };
                                        { let r_ = rt::fmul({ let a0_ = F32(c.m.r32((l_shot_16).wrapping_add(88))); F::Port_sinf(c, a0_) }, F32(0x41200000)); let a_ = (fp.wrapping_add(240)).wrapping_add(4); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                                        F::Vec3_Sub(c, (l_shot_16).wrapping_add(64), fp.wrapping_add(240), (l_w_6).wrapping_add(1328));
                                        { let v_ = rt::fdiv({ let a0_ = F::Vec3_Dot(c, (l_shot_16).wrapping_add(64), (l_shot_16).wrapping_add(64)); F::Port_sqrtf(c, a0_) }, F32(c.m.r32((l_def_8).wrapping_add(52)))); let a_ = (l_shot_16).wrapping_add(96); c.m.w32(a_, (v_).0); };
                                        if (((rt::flt(F32(c.m.r32((l_shot_16).wrapping_add(96))), F32(0x3f800000)) as i32)) != 0) {
                                            { let v_ = F32(0x3f800000); let a_ = (l_shot_16).wrapping_add(96); c.m.w32(a_, (v_).0); };
                                        }
                                        { let v_ = F32(0x00000000); let a_ = (l_shot_16).wrapping_add(92); c.m.w32(a_, (v_).0); };
                                        break 's4;
                                    }
                                }
                                2i32 => {
                                    {
                                        l_h_12 = rt::fadd(rt::fmul(F::BtlScene_RandF(c), F32(0x3eb33333)), F32(0x3e199999));
                                        { let a0_ = { let a0_ = (c.m.r32(l_arg_7) as i32); F::BtlCharApi_GetOpponentObjId(c, a0_) }; F::BtlCharApi_GetNodePos(c, a0_, 17i32, (l_shot_16).wrapping_add(16)) };
                                        F::Vec3_Sub(c, (l_shot_16).wrapping_add(16), (l_shot_16).wrapping_add(16), l_shot_16);
                                        F::Vec3_Normalize(c, (l_shot_16).wrapping_add(16), (l_shot_16).wrapping_add(16));
                                        if ((((((l_i_10).wrapping_add(1i32)) < (l_count_9)) as i32)) != 0) {
                                            l_three_26 = 3i32;
                                            if (((((rt::rem_i32(l_i_10, l_three_26)) != (0i32)) as i32)) != 0) {
                                                l_t_11 = F::EftMath_WrapAngle(c, rt::fmul(rt::fdiv(rt::i2f(l_i_10), rt::i2f(l_count_9)), F32(0x40c90fda)));
                                                F::Vec4_Set(c, fp.wrapping_add(336), F32(0x00000000), l_h_12, rt::fsub(F32(0x3f800000), l_h_12), F32(0x3f800000));
                                                F::Mtx_StoreIdentity(c, fp.wrapping_add(272));
                                                F::Mtx_RotateZ(c, fp.wrapping_add(272), fp.wrapping_add(272), l_t_11);
                                                F::Mtx_MulVec4(c, fp.wrapping_add(336), fp.wrapping_add(272), fp.wrapping_add(336));
                                                F::Vec3_Clamp(c, (l_shot_16).wrapping_add(16), (l_shot_16).wrapping_add(16), F32(0xbf800000), F32(0x3f800000));
                                                l_t_11 = { let a0_ = rt::fneg(F32(c.m.r32(((l_shot_16).wrapping_add(16)).wrapping_add(4)))); F::Port_asinf(c, a0_) };
                                                l_h_12 = { let a0_ = F32(c.m.r32((l_shot_16).wrapping_add(16))); let a1_ = F32(c.m.r32(((l_shot_16).wrapping_add(16)).wrapping_add(8))); F::Port_atan2f(c, a0_, a1_) };
                                                F::Mtx_StoreIdentity(c, fp.wrapping_add(272));
                                                F::Mtx_RotateX(c, fp.wrapping_add(272), fp.wrapping_add(272), l_t_11);
                                                F::Mtx_RotateY(c, fp.wrapping_add(272), fp.wrapping_add(272), l_h_12);
                                                F::Mtx_MulVec4(c, (l_shot_16).wrapping_add(16), fp.wrapping_add(272), fp.wrapping_add(336));
                                                F::Vec3_Normalize(c, (l_shot_16).wrapping_add(16), (l_shot_16).wrapping_add(16));
                                            }
                                        }
                                        break 's4;
                                    }
                                }
                                _ => {
                                }
                            }
                        }
                        { let v_ = F::EftBlastObj_Create(c, fp); let a_ = (l_shot_16).wrapping_add(100); c.m.w32(a_, v_); };
                        { let r_ = ((1i32) as u32); let a_ = (l_shot_16).wrapping_add(80); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
                        's5: {
                            let sv29: i32 = (c.m.r32((l_w_6).wrapping_add(1476)) as i32);
                            match sv29 {
                                0i32 | 1i32 => {
                                    { let a0_ = c.m.r32((l_shot_16).wrapping_add(100)); { let _ = F::EftBlastObj_SetHeld(c, a0_, 1i32); } };
                                    break 's5;
                                }
                                2i32 => {
                                    { let a0_ = c.m.r32((l_shot_16).wrapping_add(100)); { let _ = F::EftBlastObj_SetDelay(c, a0_, rt::fmul(rt::i2f(l_i_10), F32(0x40400000))); } };
                                    { let a0_ = c.m.r32((l_shot_16).wrapping_add(100)); { let _ = F::EftBlastObj_SetPrevPos(c, a0_, l_shot_16); } };
                                    break 's5;
                                }
                                _ => {
                                }
                            }
                        }
                        if ((((c.m.r32((l_shot_16).wrapping_add(80))) & (((4i32) as u32)))) != 0) {
                            { let a0_ = c.m.r32((l_shot_16).wrapping_add(100)); { let _ = F::EftBlastObj_MarkLast(c, a0_); } };
                        }
                        { let r_ = ((256i32) as u32); let a_ = (l_w_6).wrapping_add(1472); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
                        { let a_ = (l_w_6).wrapping_add(3760); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    }
                }
                { let o_ = l_i_10; let n_ = (o_).wrapping_add(1); l_i_10 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftRingShot_Fire", &PROBE_N, &[], true); }
    r_
}

pub fn EftRingShot_BobShot(c: &mut Ctx, mut p_shot_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRingShot_BobShot", &PROBE_N, &[(p_shot_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        F::Vec4_Copy(c, fp, (p_shot_1).wrapping_add(48));
        { let r_ = rt::fmul({ let a0_ = F32(c.m.r32((p_shot_1).wrapping_add(88))); F::Port_sinf(c, a0_) }, F32(0x41200000)); let a_ = (fp).wrapping_add(4); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        { let r_ = F32(0x3e4104f9); let a_ = (p_shot_1).wrapping_add(88); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        if (((rt::fge(F32(c.m.r32((p_shot_1).wrapping_add(88))), F32(0x40490fda)) as i32)) != 0) {
            { let r_ = F32(0x40c90fda); let a_ = (p_shot_1).wrapping_add(88); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        }
        { let a0_ = c.m.r32((p_shot_1).wrapping_add(100)); { let _ = F::EftBlastObj_SetTarget(c, a0_, 1i32, fp); } };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftRingShot_BobShot", &PROBE_N, &[], true); }
    r_
}

pub fn EftRingShot_UpdateRing(c: &mut Ctx, mut p_objId_1: i32, mut p_task_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRingShot_UpdateRing", &PROBE_N, &[(p_objId_1) as u32, (p_task_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let mut l_node_3: i32 = 0;
    let mut l_arg3_4: i32 = 0;
    let mut l_w_5: u32 = 0;
    let mut l_arg_6: u32 = 0;
    let mut l_allArrived_7: i32 = 0;
    let mut l_volley_8: i32 = 0;
    let mut l_shot_9: u32 = 0;
    let mut l_i_10: i32 = 0;
    let mut l_tmp_11: i32 = 0;
    let mut l_t_12: F32 = F32(0);
    let mut bb_: u32 = 0;
    let r_ = 'f: {
        'fl1: loop {
            match bb_ {
                0 => {
                    l_node_3 = (1i32).wrapping_neg();
                    l_arg3_4 = 0i32;
                    l_w_5 = c.m.r32((p_task_2).wrapping_add(56));
                    l_arg_6 = c.m.r32((l_w_5).wrapping_add(1316));
                    l_tmp_11 = ((c.m.r32((l_w_5).wrapping_add(1472))) as i32);
                    if ((((!((((l_tmp_11) & (1024i32))) != 0)) as i32)) != 0) { bb_ = 1; continue 'fl1; } else { bb_ = 3; continue 'fl1; }
                }
                1 => {
                    l_allArrived_7 = 1i32;
                    l_volley_8 = 0i32;
                    if ((F::EftShot_TestBits(c, p_objId_1, 4i32)) != 0) { bb_ = 4; continue 'fl1; } else { bb_ = 5; continue 'fl1; }
                }
                2 => {
                    break 'fl1;
                }
                3 => {
                    if ((((!((((l_tmp_11) & (512i32))) != 0)) as i32)) != 0) { bb_ = 43; continue 'fl1; } else { bb_ = 44; continue 'fl1; }
                }
                4 => {
                    l_node_3 = 1i32;
                    l_arg3_4 = 1i32;
                    l_volley_8 = 1i32;
                    bb_ = 5; continue 'fl1;
                }
                5 => {
                    if (((((l_node_3) >= (0i32)) as i32)) != 0) { bb_ = 6; continue 'fl1; } else { bb_ = 7; continue 'fl1; }
                }
                6 => {
                    if ((((({ let a0_ = c.m.r32((l_w_5).wrapping_add(1312)); F::EftEmit_GetPhaseMask(c, a0_) }).wrapping_shr((l_node_3) as u32)) & (1i32))) != 0) { bb_ = 8; continue 'fl1; } else { bb_ = 9; continue 'fl1; }
                }
                7 => {
                    l_i_10 = 0i32;
                    bb_ = 10; continue 'fl1;
                }
                8 => {
                    F::EftRingShot_Fire(c, p_objId_1, p_task_2, l_node_3, l_arg3_4, l_volley_8);
                    { let a0_ = (c.m.r32(l_arg_6) as i32); F::EftTechEvt_RequestRestart(c, a0_) };
                    bb_ = 9; continue 'fl1;
                }
                9 => {
                    bb_ = 7; continue 'fl1;
                }
                10 => {
                    if (((((l_i_10) < (20i32)) as i32)) != 0) { bb_ = 11; continue 'fl1; } else { bb_ = 13; continue 'fl1; }
                }
                11 => {
                    l_shot_9 = ((l_w_5).wrapping_add(1520)).wrapping_add((((l_i_10) as u32)).wrapping_mul(112));
                    if (({ let a0_ = c.m.r32((l_shot_9).wrapping_add(100)); F::EftBlastObj_IsAlive(c, a0_) }) != 0) { bb_ = 14; continue 'fl1; } else { bb_ = 16; continue 'fl1; }
                }
                12 => {
                    { let o_ = l_i_10; let n_ = (o_).wrapping_add(1); l_i_10 = n_; };
                    bb_ = 10; continue 'fl1;
                }
                13 => {
                    if ((((((c.m.r32((l_w_5).wrapping_add(3760)) as i32)) >= ((c.m.r32((l_w_5).wrapping_add(3764)) as i32))) as i32)) != 0) { bb_ = 26; continue 'fl1; } else { bb_ = 27; continue 'fl1; }
                }
                14 => {
                    if (((rt::fgt(F32(c.m.r32((l_shot_9).wrapping_add(92))), F32(c.m.r32((l_shot_9).wrapping_add(96)))) as i32)) != 0) { bb_ = 17; continue 'fl1; } else { bb_ = 18; continue 'fl1; }
                }
                15 => {
                    bb_ = 12; continue 'fl1;
                }
                16 => {
                    if ((((c.m.r32((l_shot_9).wrapping_add(80))) & (((1i32) as u32)))) != 0) { bb_ = 24; continue 'fl1; } else { bb_ = 25; continue 'fl1; }
                }
                17 => {
                    { let r_ = ((2i32) as u32); let a_ = (l_shot_9).wrapping_add(80); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
                    bb_ = 18; continue 'fl1;
                }
                18 => {
                    if ((((!((((c.m.r32((l_shot_9).wrapping_add(80))) & (((2i32) as u32)))) != 0)) as i32)) != 0) { bb_ = 19; continue 'fl1; } else { bb_ = 21; continue 'fl1; }
                }
                19 => {
                    l_t_12 = rt::fdiv(F32(c.m.r32((l_shot_9).wrapping_add(92))), F32(c.m.r32((l_shot_9).wrapping_add(96))));
                    if (((rt::fgt(l_t_12, F32(0x3f800000)) as i32)) != 0) { bb_ = 22; continue 'fl1; } else { bb_ = 23; continue 'fl1; }
                }
                20 => {
                    { let r_ = F32(0x3f800000); let a_ = (l_shot_9).wrapping_add(92); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    bb_ = 15; continue 'fl1;
                }
                21 => {
                    F::EftRingShot_BobShot(c, l_shot_9);
                    bb_ = 20; continue 'fl1;
                }
                22 => {
                    l_t_12 = F32(0x3f800000);
                    bb_ = 23; continue 'fl1;
                }
                23 => {
                    F::Vec3_ScaleAdd(c, fp, (l_shot_9).wrapping_add(64), (l_w_5).wrapping_add(1328), l_t_12);
                    { let a0_ = c.m.r32((l_shot_9).wrapping_add(100)); { let _ = F::EftBlastObj_SetTarget(c, a0_, 1i32, fp); } };
                    bb_ = 20; continue 'fl1;
                }
                24 => {
                    { let r_ = ((2i32) as u32); let a_ = (l_shot_9).wrapping_add(80); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
                    bb_ = 25; continue 'fl1;
                }
                25 => {
                    bb_ = 15; continue 'fl1;
                }
                26 => {
                    l_i_10 = 0i32;
                    l_shot_9 = ((l_w_5).wrapping_add(1520)).wrapping_add((((0i32) as u32)).wrapping_mul(112));
                    bb_ = 28; continue 'fl1;
                }
                27 => {
                    bb_ = 2; continue 'fl1;
                }
                28 => {
                    if (((((((((c.m.r32((l_shot_9).wrapping_add(100))) == (((0i32) as u32))) as i32)) != 0) || ((((c.m.r32((l_shot_9).wrapping_add(80))) & (((2i32) as u32)))) != 0)) as i32)) != 0) { bb_ = 29; continue 'fl1; } else { bb_ = 30; continue 'fl1; }
                }
                29 => {
                    { let o_ = l_i_10; let n_ = (o_).wrapping_add(1); l_i_10 = n_; };
                    if (((((l_i_10) >= (20i32)) as i32)) != 0) { bb_ = 31; continue 'fl1; } else { bb_ = 32; continue 'fl1; }
                }
                30 => {
                    l_allArrived_7 = 0i32;
                    bb_ = 33; continue 'fl1;
                }
                31 => {
                    bb_ = 33; continue 'fl1;
                }
                32 => {
                    l_shot_9 = ((l_w_5).wrapping_add(1520)).wrapping_add((((l_i_10) as u32)).wrapping_mul(112));
                    bb_ = 28; continue 'fl1;
                }
                33 => {
                    if ((l_allArrived_7) != 0) { bb_ = 35; continue 'fl1; } else { bb_ = 36; continue 'fl1; }
                }
                34 => {
                    bb_ = 32; continue 'fl1;
                }
                35 => {
                    l_i_10 = 0i32;
                    bb_ = 37; continue 'fl1;
                }
                36 => {
                    bb_ = 27; continue 'fl1;
                }
                37 => {
                    if (((((l_i_10) < (20i32)) as i32)) != 0) { bb_ = 38; continue 'fl1; } else { bb_ = 40; continue 'fl1; }
                }
                38 => {
                    l_shot_9 = ((l_w_5).wrapping_add(1520)).wrapping_add((((l_i_10) as u32)).wrapping_mul(112));
                    if (((((c.m.r32((l_shot_9).wrapping_add(100))) != (((0i32) as u32))) as i32)) != 0) { bb_ = 41; continue 'fl1; } else { bb_ = 42; continue 'fl1; }
                }
                39 => {
                    { let o_ = l_i_10; let n_ = (o_).wrapping_add(1); l_i_10 = n_; };
                    bb_ = 37; continue 'fl1;
                }
                40 => {
                    { let a0_ = (c.m.r32(l_arg_6) as i32); F::EftShot_SetHeldFlagA8(c, a0_) };
                    { let r_ = ((1024i32) as u32); let a_ = (l_w_5).wrapping_add(1472); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
                    bb_ = 36; continue 'fl1;
                }
                41 => {
                    { let a0_ = c.m.r32((l_shot_9).wrapping_add(100)); { let _ = F::EftBlastObj_SetHeld(c, a0_, 0i32); } };
                    bb_ = 42; continue 'fl1;
                }
                42 => {
                    bb_ = 39; continue 'fl1;
                }
                43 => {
                    l_i_10 = 0i32;
                    bb_ = 45; continue 'fl1;
                }
                44 => {
                    if (({ let a0_ = (c.m.r32(l_arg_6) as i32); F::EftShot_TestBits(c, a0_, 64i32) }) != 0) { bb_ = 51; continue 'fl1; } else { bb_ = 52; continue 'fl1; }
                }
                45 => {
                    if (((((l_i_10) < (20i32)) as i32)) != 0) { bb_ = 46; continue 'fl1; } else { bb_ = 48; continue 'fl1; }
                }
                46 => {
                    l_shot_9 = ((l_w_5).wrapping_add(1520)).wrapping_add((((l_i_10) as u32)).wrapping_mul(112));
                    if (((((c.m.r32((l_shot_9).wrapping_add(100))) != (((0i32) as u32))) as i32)) != 0) { bb_ = 49; continue 'fl1; } else { bb_ = 50; continue 'fl1; }
                }
                47 => {
                    { let o_ = l_i_10; let n_ = (o_).wrapping_add(1); l_i_10 = n_; };
                    bb_ = 45; continue 'fl1;
                }
                48 => {
                    bb_ = 44; continue 'fl1;
                }
                49 => {
                    F::EftRingShot_BobShot(c, l_shot_9);
                    bb_ = 50; continue 'fl1;
                }
                50 => {
                    bb_ = 47; continue 'fl1;
                }
                51 => {
                    { let a0_ = { let a0_ = (c.m.r32(l_arg_6) as i32); F::BtlCharApi_GetOpponentObjId(c, a0_) }; F::BtlCharApi_GetNodePos(c, a0_, 17i32, fp.wrapping_add(16)) };
                    l_i_10 = 0i32;
                    bb_ = 53; continue 'fl1;
                }
                52 => {
                    bb_ = 2; continue 'fl1;
                }
                53 => {
                    if (((((l_i_10) < (20i32)) as i32)) != 0) { bb_ = 54; continue 'fl1; } else { bb_ = 56; continue 'fl1; }
                }
                54 => {
                    l_shot_9 = ((l_w_5).wrapping_add(1520)).wrapping_add((((l_i_10) as u32)).wrapping_mul(112));
                    if (((((c.m.r32((l_shot_9).wrapping_add(100))) != (((0i32) as u32))) as i32)) != 0) { bb_ = 57; continue 'fl1; } else { bb_ = 58; continue 'fl1; }
                }
                55 => {
                    { let o_ = l_i_10; let n_ = (o_).wrapping_add(1); l_i_10 = n_; };
                    bb_ = 53; continue 'fl1;
                }
                56 => {
                    { let r_ = ((512i32) as u32); let a_ = (l_w_5).wrapping_add(1472); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
                    bb_ = 52; continue 'fl1;
                }
                57 => {
                    { let a0_ = c.m.r32((l_shot_9).wrapping_add(100)); { let _ = F::EftBlastObj_SetTarget(c, a0_, 0i32, ((0i32) as u32)); } };
                    F::Vec3_Sub(c, fp.wrapping_add(32), fp.wrapping_add(16), (l_shot_9).wrapping_add(48));
                    F::Vec3_Normalize(c, fp.wrapping_add(32), fp.wrapping_add(32));
                    { let a0_ = c.m.r32((l_shot_9).wrapping_add(100)); { let _ = F::EftBlastObj_SetDir(c, a0_, fp.wrapping_add(32)); } };
                    l_tmp_11 = (c.m.r32((l_shot_9).wrapping_add(84)) as i32);
                    l_tmp_11 = (20i32).wrapping_sub(l_tmp_11);
                    { let a0_ = c.m.r32((l_shot_9).wrapping_add(100)); { let _ = F::EftBlastObj_SetDelay(c, a0_, rt::fmul(rt::i2f(l_tmp_11), F32(0x3f400000))); } };
                    bb_ = 58; continue 'fl1;
                }
                58 => {
                    bb_ = 55; continue 'fl1;
                }
                _ => unreachable!(),
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftRingShot_UpdateRing", &PROBE_N, &[], true); }
    r_
}

pub fn EftRingShot_UpdatePath(c: &mut Ctx, mut p_objId_1: i32, mut p_task_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRingShot_UpdatePath", &PROBE_N, &[(p_objId_1) as u32, (p_task_2) as u32], false); }
    let mut l_node_3: i32 = 0;
    let mut l_arg3_4: i32 = 0;
    let mut l_w_5: u32 = 0;
    let mut l_arg_6: u32 = 0;
    let mut l_opp_7: i32 = 0;
    let mut l_scale_8: F32 = F32(0);
    let mut l_t_9: F32 = F32(0);
    let mut l_u_10: F32 = F32(0);
    let mut l_volley_11: i32 = 0;
    let r_ = 'f: {
    l_node_3 = (1i32).wrapping_neg();
    l_arg3_4 = 0i32;
    l_w_5 = c.m.r32((p_task_2).wrapping_add(56));
    l_arg_6 = c.m.r32((l_w_5).wrapping_add(1316));
    if ((((!((((c.m.r32((l_w_5).wrapping_add(1472))) & (((1024i32) as u32)))) != 0)) as i32)) != 0) {
        if ((F::EftShot_TestBits(c, p_objId_1, 4i32)) != 0) {
            l_opp_7 = { let a0_ = (c.m.r32(l_arg_6) as i32); F::BtlCharApi_GetOpponentObjId(c, a0_) };
            l_scale_8 = F::BtlScene_GetCharScale(c, l_opp_7);
            if (((rt::flt(l_scale_8, F32(0x3f199999)) as i32)) != 0) {
                l_scale_8 = F32(0x3f199999);
            }
            F::Vec4_Copy(c, (l_w_5).wrapping_add(1408), (l_w_5).wrapping_add(1328));
            F::BtlCharApi_GetNodePos(c, l_opp_7, 17i32, (l_w_5).wrapping_add(1440));
            F::Vec3_Sub(c, (l_w_5).wrapping_add(1424), (l_w_5).wrapping_add(1440), (l_w_5).wrapping_add(1408));
            F::Vec3_Scale(c, (l_w_5).wrapping_add(1424), (l_w_5).wrapping_add(1424), F32(0x3f000000));
            F::Vec3_Add(c, (l_w_5).wrapping_add(1424), (l_w_5).wrapping_add(1424), (l_w_5).wrapping_add(1408));
            { let r_ = rt::fadd(rt::fmul(l_scale_8, F32(0x41200000)), F32(0x42200000)); let a_ = ((l_w_5).wrapping_add(1424)).wrapping_add(4); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            { let r_ = rt::fadd(rt::fmul(l_scale_8, F32(0x41c80000)), F32(0x43110000)); let a_ = ((l_w_5).wrapping_add(1440)).wrapping_add(4); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            { let v_ = F32(0x3f800000); let a_ = ((l_w_5).wrapping_add(1424)).wrapping_add(12); c.m.w32(a_, (v_).0); };
            F::Vec4_Copy(c, (l_w_5).wrapping_add(1456), (l_w_5).wrapping_add(1440));
            { let r_ = ((2048i32) as u32); let a_ = (l_w_5).wrapping_add(1472); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
        }
        if ((((c.m.r32((l_w_5).wrapping_add(1472))) & (((2048i32) as u32)))) != 0) {
            l_t_9 = F32(c.m.r32((l_w_5).wrapping_add(1512)));
            if (((rt::fgt(l_t_9, F32(0x41900000)) as i32)) != 0) {
                l_t_9 = F32(0x41900000);
            }
            if (((rt::flt(l_t_9, F32(0x41100000)) as i32)) != 0) {
                F::EftMath_Spline3(c, (l_w_5).wrapping_add(1344), (l_w_5).wrapping_add(1408), rt::fdiv(l_t_9, F32(0x41100000)));
            } else {
                l_u_10 = rt::fdiv(rt::fsub(l_t_9, F32(0x41100000)), F32(0x41100000));
                if (((rt::fgt(l_u_10, F32(0x3f800000)) as i32)) != 0) {
                    l_u_10 = F32(0x3f800000);
                }
                F::EftMath_Spline3(c, (l_w_5).wrapping_add(1344), (l_w_5).wrapping_add(1424), l_u_10);
            }
            { let r_ = F32(0x3f800000); let a_ = (l_w_5).wrapping_add(1512); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            if (((rt::fge(F32(c.m.r32((l_w_5).wrapping_add(1512))), F32(0x41a00000)) as i32)) != 0) {
                { let a0_ = (c.m.r32(l_arg_6) as i32); F::EftShot_SetHeldFlagA8(c, a0_) };
                { let r_ = ((1024i32) as u32); let a_ = (l_w_5).wrapping_add(1472); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
            }
        }
    } else {
        if ((((!((((c.m.r32((l_w_5).wrapping_add(1472))) & (((256i32) as u32)))) != 0)) as i32)) != 0) {
            l_volley_11 = 0i32;
            if ((F::EftShot_TestBits(c, p_objId_1, 16i32)) != 0) {
                l_node_3 = 3i32;
                l_arg3_4 = 3i32;
                l_volley_11 = 1i32;
            }
            if (((((l_node_3) >= (0i32)) as i32)) != 0) {
                F::EftRingShot_Fire(c, p_objId_1, p_task_2, l_node_3, l_arg3_4, l_volley_11);
                { let a0_ = (c.m.r32(l_arg_6) as i32); F::EftTechEvt_RequestRestart(c, a0_) };
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftRingShot_UpdatePath", &PROBE_N, &[], true); }
    r_
}

pub fn EftRingShot_UpdateFixed(c: &mut Ctx, mut p_objId_1: i32, mut p_task_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRingShot_UpdateFixed", &PROBE_N, &[(p_objId_1) as u32, (p_task_2) as u32], false); }
    let mut l_w_3: u32 = 0;
    let mut l_arg_4: u32 = 0;
    let mut l_opp_5: i32 = 0;
    let r_ = 'f: {
    l_w_3 = c.m.r32((p_task_2).wrapping_add(56));
    l_arg_4 = c.m.r32((l_w_3).wrapping_add(1316));
    if ((((!((((c.m.r32((l_w_3).wrapping_add(1472))) & (((1024i32) as u32)))) != 0)) as i32)) != 0) {
        if ((F::EftShot_TestBits(c, p_objId_1, 16i32)) != 0) {
            l_opp_5 = { let a0_ = (c.m.r32(l_arg_4) as i32); F::BtlCharApi_GetOpponentObjId(c, a0_) };
            F::BtlCharApi_GetNodePos(c, l_opp_5, 3i32, (l_w_3).wrapping_add(1344));
            F::BtlCharApi_GetNodePos(c, l_opp_5, 3i32, (l_w_3).wrapping_add(1360));
            { let a0_ = (c.m.r32(l_arg_4) as i32); F::EftShot_SetHeldFlagA8(c, a0_) };
            { let r_ = ((1024i32) as u32); let a_ = (l_w_3).wrapping_add(1472); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
        }
    } else {
        { let _ = { let a0_ = (c.m.r32(l_arg_4) as i32); F::EftShot_TestBits(c, a0_, 4i32) }; };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftRingShot_UpdateFixed", &PROBE_N, &[], true); }
    r_
}

pub fn EftRingShot_ReapShots(c: &mut Ctx, mut p_task_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRingShot_ReapShots", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_alive_2: i32 = 0;
    let mut l_w_3: u32 = 0;
    let mut l_shot_4: u32 = 0;
    let mut l_i_5: i32 = 0;
    let r_ = 'f: {
    l_alive_2 = 0i32;
    l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
    {
        { l_i_5 = 19i32; l_shot_4 = (l_w_3).wrapping_add(1520); };
        'l1: loop {
            if !(((((l_i_5) >= (0i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if (({ let a0_ = c.m.r32((l_shot_4).wrapping_add(100)); F::EftBlastObj_IsAlive(c, a0_) }) != 0) {
                    l_alive_2 = 1i32;
                } else {
                    { let v_ = ((0i32) as u32); let a_ = (l_shot_4).wrapping_add(100); c.m.w32(a_, v_); };
                }
            }
            { { let o_ = l_i_5; let n_ = (o_).wrapping_sub(1); l_i_5 = n_; }; { let o_ = l_shot_4; let n_ = (o_).wrapping_add(112); l_shot_4 = n_; }; };
        }
    }
    break 'f l_alive_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftRingShot_ReapShots", &PROBE_N, &[], true); }
    r_
}

pub fn EftRingShot_UpdateShots(c: &mut Ctx, mut p_objId_1: i32, mut p_task_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRingShot_UpdateShots", &PROBE_N, &[(p_objId_1) as u32, (p_task_2) as u32], false); }
    let mut l_w_3: u32 = 0;
    let r_ = 'f: {
    l_w_3 = c.m.r32((p_task_2).wrapping_add(56));
    's1: {
        let sv4: i32 = (c.m.r32((l_w_3).wrapping_add(1476)) as i32);
        match sv4 {
            0i32 => {
                F::EftRingShot_UpdateRing(c, p_objId_1, p_task_2);
                break 's1;
            }
            1i32 => {
                F::EftRingShot_UpdateRing(c, p_objId_1, p_task_2);
                break 's1;
            }
            2i32 => {
                F::EftRingShot_UpdatePath(c, p_objId_1, p_task_2);
                break 's1;
            }
            3i32 => {
                F::EftRingShot_UpdateFixed(c, p_objId_1, p_task_2);
                break 's1;
            }
            _ => {
            }
        }
    }
    break 'f F::EftRingShot_ReapShots(c, p_task_2);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftRingShot_UpdateShots", &PROBE_N, &[], true); }
    r_
}

pub fn EftRingShot_AddHit(c: &mut Ctx, mut p_task_1: u32, mut p_last_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRingShot_AddHit", &PROBE_N, &[(p_task_1) as u32, (p_last_2) as u32], false); }
    let mut l_rec_3: u32 = 0;
    let mut l_w_4: u32 = 0;
    let mut l_radius_5: F32 = F32(0);
    let mut l_a_6: u32 = 0;
    let mut l_b_7: u32 = 0;
    let mut l_a_8: u32 = 0;
    let mut l_b_9: u32 = 0;
    let r_ = 'f: {
    l_w_4 = c.m.r32((p_task_1).wrapping_add(56));
    l_rec_3 = F::EftHit_GetNew(c);
    l_radius_5 = rt::fmul(F32(c.m.r32((l_w_4).wrapping_add(1496))), F::EftEmit_GetTrailWidth(c, l_w_4));
    { let s_ = (l_w_4).wrapping_add(1328); let d_ = (l_rec_3).wrapping_add(16); rt::memcpy(c, d_, s_, 64); };
    { let v_ = p_task_1; let a_ = (l_rec_3).wrapping_add(96); c.m.w32(a_, v_); };
    { let v_ = c.m.r32((l_w_4).wrapping_add(1316)); let a_ = (l_rec_3).wrapping_add(100); c.m.w32(a_, v_); };
    if ((p_last_2) != 0) {
        { let r_ = ((128i32) as u32); let a_ = (l_rec_3).wrapping_add(80); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
    }
    's1: {
        let sv10: i32 = (({ let a_ = ({ let a_ = (c.m.r32((l_w_4).wrapping_add(1316))).wrapping_add(36); c.m.r32(a_) }).wrapping_add(8); (c.m.r8(a_) as i8) }) as i32);
        match sv10 {
            1i32 => {
                {
                    l_a_6 = F::EftHitArena_AllocBox(c);
                    l_b_7 = F::EftHitArena_AllocBox(c);
                    F::ColCapsule_Set(c, l_a_6, (l_w_4).wrapping_add(816), (l_w_4).wrapping_add(1344), l_radius_5);
                    F::ColCapsule_Set(c, l_b_7, (l_w_4).wrapping_add(816), (l_w_4).wrapping_add(1360), l_radius_5);
                    F::EftHit_SetShapeBoxes(c, l_rec_3, l_a_6, l_b_7);
                    break 's1;
                }
            }
            0i32 => {
                {
                    l_a_8 = F::EftHitArena_AllocSphere(c);
                    l_b_9 = F::EftHitArena_AllocSphere(c);
                    F::ColSphere_Set(c, l_a_8, (l_w_4).wrapping_add(1344), l_radius_5);
                    F::ColSphere_Set(c, l_b_9, (l_w_4).wrapping_add(1360), l_radius_5);
                    F::EftHit_SetShapeSpheres(c, l_rec_3, l_a_8, l_b_9);
                    break 's1;
                }
            }
            _ => {
                break 'f;
            }
        }
    }
    F::EftHit_Add(c, l_rec_3);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftRingShot_AddHit", &PROBE_N, &[], true); }
    r_
}

pub fn EftRingShot_Draw(c: &mut Ctx, mut p_objId_1: i32, mut p_task_2: u32, mut p_model_3: u32, mut p_mode_4: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRingShot_Draw", &PROBE_N, &[(p_objId_1) as u32, (p_task_2) as u32, (p_model_3) as u32, (p_mode_4) as u32], false); }
    let mut l_w_5: u32 = 0;
    let mut l_part_6: i32 = 0;
    let mut l_sub_7: i32 = 0;
    let mut l_def_8: u32 = 0;
    let mut l_spawn_9: i32 = 0;
    let r_ = 'f: {
    l_w_5 = c.m.r32((p_task_2).wrapping_add(56));
    {
        l_part_6 = 0i32;
        'l1: loop {
            if !(((((l_part_6) < (19i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if (((({ let a_ = c.m.r32(p_model_3); c.m.r32(a_) }) & ((((1i32).wrapping_shl((l_part_6) as u32)) as u32)))) != 0) {
                    l_def_8 = c.m.r32(((p_model_3).wrapping_add(12)).wrapping_add((((l_part_6) as u32)).wrapping_mul(16)));
                    {
                        l_sub_7 = 0i32;
                        'l3: loop {
                            if !(((((l_sub_7) < (((c.m.r8((l_def_8).wrapping_add(1))) as i32))) as i32)) != 0) { break 'l3; }
                            'c4: {
                                if (({ let a1_ = ((c.m.r8(l_def_8)) as i32); F::EftEmit_IsPartDeferred(c, p_model_3, a1_, l_sub_7) }) != 0) {
                                    break 'c4;
                                }
                                if (((((p_mode_4) == (0i32)) as i32)) != 0) {
                                    l_spawn_9 = { let a5_ = ((((c.m.r32((l_w_5).wrapping_add(1472))) & (((1i32) as u32)))) as i32); let a6_ = ((((c.m.r32((l_w_5).wrapping_add(1472))) & (((4i32) as u32)))) as i32); F::EftEmit_GetFlagsFromReq(c, p_model_3, l_w_5, p_objId_1, l_part_6, l_sub_7, a5_, a6_) };
                                } else {
                                    l_spawn_9 = F::EftEmit_GetResetFlags(c, p_model_3, l_w_5, l_part_6, l_sub_7);
                                }
                                if ((((c.m.r32((l_w_5).wrapping_add(1472))) & (((128i32) as u32)))) != 0) {
                                    l_spawn_9 = 2i32;
                                }
                                if (((((l_spawn_9) != (0i32)) as i32)) != 0) {
                                    { let a8_ = F32(c.m.r32((l_w_5).wrapping_add(1492))); F::EftEmit_SpawnOwn(c, p_model_3, l_w_5, (l_w_5).wrapping_add(720), (l_w_5).wrapping_add(1344), (l_w_5).wrapping_add(1392), l_part_6, l_sub_7, l_spawn_9, a8_) };
                                }
                            }
                            { let o_ = l_sub_7; let n_ = (o_).wrapping_add(1); l_sub_7 = n_; };
                        }
                    }
                }
            }
            { let o_ = l_part_6; let n_ = (o_).wrapping_add(1); l_part_6 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftRingShot_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn EftRingShot_Init(c: &mut Ctx, mut p_task_1: u32, mut p_arg_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRingShot_Init", &PROBE_N, &[(p_task_1) as u32, (p_arg_2) as u32], false); }
    let mut l_mgr_3: u32 = 0;
    let mut l_w_4: u32 = 0;
    let mut l_model_5: u32 = 0;
    let mut l_def_6: u32 = 0;
    let mut l_id_7: i32 = 0;
    let r_ = 'f: {
    l_mgr_3 = { let a_ = (F::BtlTask_GetParent(c, p_task_1)).wrapping_add(56); c.m.r32(a_) };
    l_w_4 = c.m.r32((p_task_1).wrapping_add(56));
    { let _ = F::memset(c, l_w_4, 0i32, 3776u32); };
    { let v_ = p_arg_2; let a_ = (l_w_4).wrapping_add(1316); c.m.w32(a_, v_); };
    l_model_5 = (l_mgr_3).wrapping_add(4);
    l_def_6 = c.m.r32((p_arg_2).wrapping_add(36));
    { let v_ = F32(c.m.r32((l_def_6).wrapping_add(52))); let a_ = (l_w_4).wrapping_add(1480); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((l_def_6).wrapping_add(56))); let a_ = (l_w_4).wrapping_add(1484); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((l_def_6).wrapping_add(48))); let a_ = (l_w_4).wrapping_add(1488); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((l_def_6).wrapping_add(48))); let a_ = (l_w_4).wrapping_add(1492); c.m.w32(a_, (v_).0); };
    { let v_ = ((((c.m.r8((l_def_6).wrapping_add(9)) as i8)) as i32)).wrapping_mul((((c.m.r16((l_def_6).wrapping_add(68)) as i16)) as i32)); let a_ = (l_w_4).wrapping_add(3764); c.m.w32(a_, (v_) as u32); };
    { let v_ = F32(c.m.r32((l_w_4).wrapping_add(1488))); let a_ = (l_w_4).wrapping_add(1496); c.m.w32(a_, (v_).0); };
    { let v_ = l_model_5; let a_ = (l_w_4).wrapping_add(1312); c.m.w32(a_, v_); };
    F::EftEmit_InitState(c, l_model_5, l_w_4);
    { let v_ = rt::i2f({ let a0_ = c.m.r32((l_w_4).wrapping_add(1312)); F::EftEmit_GetEndFrames(c, a0_) }); let a_ = (l_w_4).wrapping_add(1508); c.m.w32(a_, (v_).0); };
    l_id_7 = (((c.m.r16(l_def_6) as i16)) as i32);
    if (((((l_id_7) == (621i32)) as i32)) != 0) {
        { let v_ = 0i32; let a_ = (l_w_4).wrapping_add(1476); c.m.w32(a_, (v_) as u32); };
    } else {
        if (((((l_id_7) == (397i32)) as i32)) != 0) {
            { let v_ = 1i32; let a_ = (l_w_4).wrapping_add(1476); c.m.w32(a_, (v_) as u32); };
        } else {
            if (((((l_id_7) == (342i32)) as i32)) != 0) {
                { let v_ = 2i32; let a_ = (l_w_4).wrapping_add(1476); c.m.w32(a_, (v_) as u32); };
            } else {
                if (((((l_id_7) == (476i32)) as i32)) != 0) {
                    { let v_ = 2i32; let a_ = (l_w_4).wrapping_add(1476); c.m.w32(a_, (v_) as u32); };
                } else {
                    if (((((l_id_7) == (748i32)) as i32)) != 0) {
                        { let v_ = 2i32; let a_ = (l_w_4).wrapping_add(1476); c.m.w32(a_, (v_) as u32); };
                    } else {
                        if (((((l_id_7) == (407i32)) as i32)) != 0) {
                            { let v_ = 3i32; let a_ = (l_w_4).wrapping_add(1476); c.m.w32(a_, (v_) as u32); };
                        } else {
                            if (((((l_id_7) == (408i32)) as i32)) != 0) {
                                { let v_ = 3i32; let a_ = (l_w_4).wrapping_add(1476); c.m.w32(a_, (v_) as u32); };
                            } else {
                                if (((((l_id_7) == (512i32)) as i32)) != 0) {
                                    { let v_ = 3i32; let a_ = (l_w_4).wrapping_add(1476); c.m.w32(a_, (v_) as u32); };
                                } else {
                                    if (((((l_id_7) == (676i32)) as i32)) != 0) {
                                        { let v_ = 3i32; let a_ = (l_w_4).wrapping_add(1476); c.m.w32(a_, (v_) as u32); };
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    { let a1_ = (if ((((((c.m.r32(p_arg_2) as i32)) == (0i32)) as i32)) != 0) { 2048i32 } else { 4096i32 }); F::BtlTask_SetOwnerTag(c, p_task_1, a1_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftRingShot_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftRingShot_Term(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRingShot_Term", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let mut l_arg_3: u32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    l_arg_3 = c.m.r32((l_w_2).wrapping_add(1316));
    { let a0_ = c.m.r32((l_w_2).wrapping_add(1312)); F::EftEmit_TermState(c, a0_, l_w_2) };
    if ((((!((((c.m.r32((l_w_2).wrapping_add(1472))) & (((1024i32) as u32)))) != 0)) as i32)) != 0) {
        { let a0_ = (c.m.r32(l_arg_3) as i32); F::EftShot_SetHeldFlagA8(c, a0_) };
    }
    if ((((((({ let a_ = (c.m.r32((l_arg_3).wrapping_add(36))).wrapping_add(4); (c.m.r8(a_) as i8) }) as i32)) != (0i32)) as i32)) != 0) {
        { let a0_ = (c.m.r32(l_arg_3) as i32); F::EftShot_SetHeldFlagA9(c, a0_) };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftRingShot_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftRingShot_Update(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRingShot_Update", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_advance_2: i32 = 0;
    let mut l_w_3: u32 = 0;
    let mut l_arg_4: u32 = 0;
    let mut l_alive_5: i32 = 0;
    let r_ = 'f: {
    l_advance_2 = 0i32;
    l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
    l_arg_4 = c.m.r32((l_w_3).wrapping_add(1316));
    if (({ let a0_ = (c.m.r32(l_arg_4) as i32); F::BtlScene_IsCharStopped(c, a0_) }) != 0) {
        break 'f;
    }
    F::EftEmit_UpdateNodesReq(c, l_arg_4, (l_w_3).wrapping_add(720));
    if ((((!((((c.m.r32((l_w_3).wrapping_add(1472))) & (((64i32) as u32)))) != 0)) as i32)) != 0) {
        if (({ let a0_ = (c.m.r32(l_arg_4) as i32); F::EftShot_TestBits(c, a0_, 2i32) }) != 0) {
            { let a2_ = (c.m.r32(l_arg_4) as i32); { let _ = F::EftAim_GetDir(c, (l_w_3).wrapping_add(1392), (((((l_w_3).wrapping_add(752)) as u64)) as i32), a2_); } };
            { let v_ = ((0i32) as u8); let a_ = (p_task_1).wrapping_add(1); c.m.w8(a_, v_); };
            { let r_ = ((64i32) as u32); let a_ = (l_w_3).wrapping_add(1472); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
        } else {
            if (({ let a0_ = (c.m.r32(l_arg_4) as i32); F::EftShot_TestBits(c, a0_, 4i32) }) != 0) {
                { let v_ = ((0i32) as u8); let a_ = (p_task_1).wrapping_add(1); c.m.w8(a_, v_); };
                { let r_ = ((64i32) as u32); let a_ = (l_w_3).wrapping_add(1472); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
            }
        }
    }
    if ((((c.m.r32((l_w_3).wrapping_add(1472))) & (((64i32) as u32)))) != 0) {
        's1: {
            let sv6: i32 = ((c.m.r8((p_task_1).wrapping_add(1))) as i32);
            match sv6 {
                0i32 => {
                    if (({ let a0_ = (c.m.r32(l_arg_4) as i32); F::EftShot_TestBits(c, a0_, 4i32) }) != 0) {
                        { let r_ = ((16i32) as u32); let a_ = (l_w_3).wrapping_add(1472); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
                        's2: {
                            let sv7: i32 = (c.m.r32((l_w_3).wrapping_add(1476)) as i32);
                            match sv7 {
                                0i32 | 1i32 => {
                                    { let a0_ = (c.m.r32(l_arg_4) as i32); F::BtlCharApi_GetNodePos(c, a0_, 17i32, (l_w_3).wrapping_add(1328)) };
                                    F::Vec4_Copy(c, (l_w_3).wrapping_add(1344), (l_w_3).wrapping_add(1328));
                                    { let a0_ = (c.m.r32(l_arg_4) as i32); F::BtlCharApi_GetNodePos(c, a0_, 17i32, (l_w_3).wrapping_add(1360)) };
                                    { let a3_ = (c.m.r32(l_arg_4) as i32); { let _ = F::EftAim_GetDirKeep(c, l_arg_4, (l_w_3).wrapping_add(1392), (((((l_w_3).wrapping_add(1344)) as u64)) as i32), a3_); } };
                                    { let a2_ = F32(c.m.r32((l_w_3).wrapping_add(1480))); F::Vec3_Scale(c, (l_w_3).wrapping_add(1376), (l_w_3).wrapping_add(1392), a2_) };
                                    break 's2;
                                }
                                2i32 => {
                                    F::Vec4_Copy(c, (l_w_3).wrapping_add(1328), (l_w_3).wrapping_add(816));
                                    F::Vec4_Copy(c, (l_w_3).wrapping_add(1344), (l_w_3).wrapping_add(1328));
                                    { let a0_ = (c.m.r32(l_arg_4) as i32); F::BtlCharApi_GetNodePos(c, a0_, 17i32, (l_w_3).wrapping_add(1360)) };
                                    { let a3_ = (c.m.r32(l_arg_4) as i32); { let _ = F::EftAim_GetDirKeep(c, l_arg_4, (l_w_3).wrapping_add(1392), (((((l_w_3).wrapping_add(1344)) as u64)) as i32), a3_); } };
                                    { let a2_ = F32(c.m.r32((l_w_3).wrapping_add(1480))); F::Vec3_Scale(c, (l_w_3).wrapping_add(1376), (l_w_3).wrapping_add(1392), a2_) };
                                    break 's2;
                                }
                                3i32 => {
                                    break 's2;
                                }
                                _ => {
                                }
                            }
                        }
                        { let v_ = ((1i32) as u8); let a_ = (p_task_1).wrapping_add(1); c.m.w8(a_, v_); };
                    }
                    break 's1;
                }
                2i32 => {
                    if ((((!((((c.m.r32((l_w_3).wrapping_add(1472))) & (((32i32) as u32)))) != 0)) as i32)) != 0) {
                        F::Vec4_Copy(c, (l_w_3).wrapping_add(1360), (l_w_3).wrapping_add(1344));
                    }
                    if (({ let a0_ = (c.m.r32(l_arg_4) as i32); F::EftShot_TestBits(c, a0_, 8i32) }) != 0) {
                        { let r_ = ((((8i32) | (1i32))) as u32); let a_ = (l_w_3).wrapping_add(1472); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
                        { let v_ = ((3i32) as u8); let a_ = (p_task_1).wrapping_add(1); c.m.w8(a_, v_); };
                    }
                    break 's1;
                }
                1i32 => {
                    l_advance_2 = 1i32;
                    break 's1;
                }
                3i32 => {
                    l_advance_2 = 1i32;
                    break 's1;
                }
                _ => {
                }
            }
        }
    }
    l_alive_5 = { let a0_ = (c.m.r32(l_arg_4) as i32); F::EftRingShot_UpdateShots(c, a0_, p_task_1) };
    if ((((!((l_alive_5) != 0)) as i32)) != 0) {
        if ((((c.m.r32((l_w_3).wrapping_add(1472))) & (((256i32) as u32)))) != 0) {
            { let r_ = ((1i32) as u32); let a_ = (l_w_3).wrapping_add(1472); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
        }
    }
    if (({ let a0_ = (c.m.r32(l_arg_4) as i32); F::EftShot_TestBits(c, a0_, 1024i32) }) != 0) {
        if ((((!((((c.m.r32((l_w_3).wrapping_add(1472))) & (((8i32) as u32)))) != 0)) as i32)) != 0) {
            { let r_ = ((4i32) as u32); let a_ = (l_w_3).wrapping_add(1472); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
        }
        { let r_ = ((1i32) as u32); let a_ = (l_w_3).wrapping_add(1472); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
    }
    { let a0_ = (c.m.r32(l_arg_4) as i32); let a2_ = c.m.r32((l_w_3).wrapping_add(1312)); F::EftRingShot_Draw(c, a0_, p_task_1, a2_, 0i32) };
    if ((l_advance_2) != 0) {
        { let a_ = (p_task_1).wrapping_add(1); let o_ = c.m.r8(a_); let n_ = (o_).wrapping_add(1); c.m.w8(a_, n_); };
    }
    if ((((c.m.r32((l_w_3).wrapping_add(1472))) & (((1i32) as u32)))) != 0) {
        { let r_ = F32(0x3f800000); let a_ = (l_w_3).wrapping_add(1504); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
    }
    if ((((((((!((l_alive_5) != 0)) as i32)) != 0) && ((((c.m.r32((l_w_3).wrapping_add(1472))) & (((2i32) as u32)))) != 0)) as i32)) != 0) {
        F::BtlTask_SetDead(c, p_task_1);
    } else {
        if ((((c.m.r32((l_w_3).wrapping_add(1472))) & (((1i32) as u32)))) != 0) {
            if ((((((((c.m.r32((l_w_3).wrapping_add(1472))) & (((4i32) as u32)))) != 0) || (((rt::fge(F32(c.m.r32((l_w_3).wrapping_add(1504))), F32(c.m.r32((l_w_3).wrapping_add(1508)))) as i32)) != 0)) as i32)) != 0) {
                { let r_ = ((2i32) as u32); let a_ = (l_w_3).wrapping_add(1472); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
            }
        } else {
            if (((({ let a_ = (c.m.r32((l_arg_4).wrapping_add(36))).wrapping_add(60); c.m.r32(a_) }) & (((1i32) as u32)))) != 0) {
                if ((((c.m.r32((l_w_3).wrapping_add(1472))) & (((16i32) as u32)))) != 0) {
                    { let a0_ = c.m.r32((l_w_3).wrapping_add(1312)); F::EftEmit_UpdateTrailWidth(c, a0_, l_w_3) };
                    's3: {
                        let sv8: i32 = (c.m.r32((l_w_3).wrapping_add(1476)) as i32);
                        match sv8 {
                            0i32 | 1i32 => {
                                break 's3;
                            }
                            2i32 => {
                                if ((((!((((c.m.r32((l_w_3).wrapping_add(1472))) & (((1024i32) as u32)))) != 0)) as i32)) != 0) {
                                    F::EftRingShot_AddHit(c, p_task_1, 1i32);
                                }
                                break 's3;
                            }
                            3i32 => {
                                F::EftRingShot_AddHit(c, p_task_1, 0i32);
                                break 's3;
                            }
                            _ => {
                            }
                        }
                    }
                }
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftRingShot_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftRingShot_PostUpdate(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRingShot_PostUpdate", &PROBE_N, &[(p_task_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
        l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
        if (({ let a0_ = { let a_ = c.m.r32((l_w_2).wrapping_add(1316)); (c.m.r32(a_) as i32) }; F::BtlScene_IsCharStopped(c, a0_) }) != 0) {
            break 'f;
        }
        { let _ = { let a0_ = c.m.r32((l_w_2).wrapping_add(1312)); F::EftEmit_UpdateAlive(c, a0_, l_w_2) }; };
        { let _ = F::EftRingShot_ReapShots(c, p_task_1); };
        if ((((((c.m.r16((p_task_1).wrapping_add(8))) as i32)) & (1i32))) != 0) {
            F::Vec3_Sub(c, fp, (p_task_1).wrapping_add(16), (l_w_2).wrapping_add(1344));
            F::Vec3_Copy(c, (l_w_2).wrapping_add(1344), (p_task_1).wrapping_add(16));
            F::Vec3_Add(c, (l_w_2).wrapping_add(1360), (l_w_2).wrapping_add(1360), fp);
            if ((((!((((c.m.r32((l_w_2).wrapping_add(1472))) & (((32i32) as u32)))) != 0)) as i32)) != 0) {
                { let r_ = ((32i32) as u32); let a_ = (l_w_2).wrapping_add(1472); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
            }
        } else {
            { let r_ = (((!(32i32))) as u32); let a_ = (l_w_2).wrapping_add(1472); let n_ = ((c.m.r32(a_)) & (r_)); c.m.w32(a_, n_); };
        }
        if ((((((c.m.r16((p_task_1).wrapping_add(8))) as i32)) & (4i32))) != 0) {
            { let r_ = ((1i32) as u32); let a_ = (l_w_2).wrapping_add(1472); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftRingShot_PostUpdate", &PROBE_N, &[], true); }
    r_
}

pub fn EftRingShot_Reset(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRingShot_Reset", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    if ((((!((((c.m.r32((l_w_2).wrapping_add(1472))) & (((128i32) as u32)))) != 0)) as i32)) != 0) {
        { let r_ = ((128i32) as u32); let a_ = (l_w_2).wrapping_add(1472); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
        { let a0_ = c.m.r32((l_w_2).wrapping_add(1312)); F::EftEmit_KillAll(c, a0_, l_w_2) };
    }
    F::BtlTask_SetDead(c, p_task_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftRingShot_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn EftRingShot_Nop(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRingShot_Nop", &PROBE_N, &[], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftRingShot_Nop", &PROBE_N, &[], true); }
    r_
}

pub fn EftRingShotMgr_Init(c: &mut Ctx, mut p_task_1: u32, mut p_arg_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRingShotMgr_Init", &PROBE_N, &[(p_task_1) as u32, (p_arg_2) as u32], false); }
    let mut l_mgr_3: u32 = 0;
    let r_ = 'f: {
    l_mgr_3 = c.m.r32((p_task_1).wrapping_add(56));
    { let _ = ((804u32) as i32); F::EftShot_Nop(c) };
    { let _ = F::memset(c, l_mgr_3, 0i32, 804u32); };
    { let a3_ = c.m.r32((p_arg_2).wrapping_add(28)); F::EftEmit_LoadSet(c, p_arg_2, (l_mgr_3).wrapping_add(4), ((((0i32) as i64 as u64)) as u32), a3_, 0i32, 4i32) };
    { let _ = F::BtlTask_CreateChildList(c, p_task_1, ((((2i32) as i64 as u64)) as u32), ((((((3776u32) as i32)) as i64 as u64)) as u32)); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftRingShotMgr_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftRingShotMgr_Term(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRingShotMgr_Term", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_mgr_2: u32 = 0;
    let r_ = 'f: {
    l_mgr_2 = c.m.r32((p_task_1).wrapping_add(56));
    F::EftEmit_FreeSet(c, (l_mgr_2).wrapping_add(4));
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftRingShotMgr_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftRingShotMgr_Update(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRingShotMgr_Update", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_mgr_2: u32 = 0;
    let r_ = 'f: {
    l_mgr_2 = c.m.r32((p_task_1).wrapping_add(56));
    F::EftEmit_BeginFrame(c, (l_mgr_2).wrapping_add(4));
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftRingShotMgr_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftRingShotMgr_Reset(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftRingShotMgr_Reset", &PROBE_N, &[], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftRingShotMgr_Reset", &PROBE_N, &[], true); }
    r_
}
