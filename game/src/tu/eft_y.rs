#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn EftQuad_StepAll(c: &mut Ctx, mut p_w_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftQuad_StepAll", &PROBE_N, &[(p_w_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_def_2: u32 = 0;
    let mut l_pitch_3: F32 = F32(0);
    let mut l_yaw_4: F32 = F32(0);
    let mut l_q_5: u32 = 0;
    let mut l_fade_6: F32 = F32(0);
    let mut l_alpha_7: F32 = F32(0);
    let mut l_t_8: F32 = F32(0);
    let mut l_s_9: F32 = F32(0);
    let mut l_osc_10: F32 = F32(0);
    let mut l_speed_11: F32 = F32(0);
    let mut l_u_12: F32 = F32(0);
    let mut l_r_13: F32 = F32(0);
    let mut l_frame_14: u8 = 0;
    let r_ = 'f: {
        l_def_2 = c.m.r32(p_w_1);
        l_fade_6 = F32(0x00000000);
        l_alpha_7 = F32(0x3f800000);
        { let _ = F::memset(c, fp, 0i32, 16u32); };
        if (((rt::flt(F32(c.m.r32(((p_w_1).wrapping_add(64)).wrapping_add(4))), F32(0xbf800000)) as i32)) != 0) {
            { let v_ = F32(0xbf800000); let a_ = ((p_w_1).wrapping_add(64)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        }
        if (((rt::flt(F32(0x3f800000), F32(c.m.r32(((p_w_1).wrapping_add(64)).wrapping_add(4)))) as i32)) != 0) {
            { let v_ = F32(0x3f800000); let a_ = ((p_w_1).wrapping_add(64)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        }
        l_pitch_3 = { let a0_ = { let a0_ = rt::fneg(F32(c.m.r32(((p_w_1).wrapping_add(64)).wrapping_add(4)))); F::Mathf_Asin(c, a0_) }; F::EftMath_WrapAngle(c, a0_) };
        l_yaw_4 = { let a0_ = { let a0_ = F32(c.m.r32((p_w_1).wrapping_add(64))); let a1_ = F32(c.m.r32(((p_w_1).wrapping_add(64)).wrapping_add(8))); F::Port_atan2f(c, a0_, a1_) }; F::EftMath_WrapAngle(c, a0_) };
        {
            l_q_5 = c.m.r32((p_w_1).wrapping_add(968));
            'l1: loop {
                if !(((((l_q_5) != (((0i32) as u32))) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_t_8 = (if (((rt::fge(F32(c.m.r32((l_q_5).wrapping_add(340))), F32(c.m.r32((l_q_5).wrapping_add(336)))) as i32)) != 0) { F32(0x3f800000) } else { rt::fdiv(F32(c.m.r32((l_q_5).wrapping_add(340))), F32(c.m.r32((l_q_5).wrapping_add(336)))) });
                    if ((((!(((((c.m.r32((l_q_5).wrapping_add(356)) as i32)) & (16i32))) != 0)) as i32)) != 0) {
                        l_s_9 = F32(0x00000000);
                        l_osc_10 = F32(0x3f800000);
                        if (((((c.m.r32((l_q_5).wrapping_add(356)) as i32)) & (2048i32))) != 0) {
                            l_s_9 = (if (((rt::flt(rt::fdiv(F32(c.m.r32((l_q_5).wrapping_add(280))), F32(c.m.r32((l_q_5).wrapping_add(284)))), F32(0x00000000)) as i32)) != 0) { F32(0x00000000) } else { (if (((rt::flt(F32(0x3f800000), rt::fdiv(F32(c.m.r32((l_q_5).wrapping_add(280))), F32(c.m.r32((l_q_5).wrapping_add(284))))) as i32)) != 0) { F32(0x3f800000) } else { rt::fdiv(F32(c.m.r32((l_q_5).wrapping_add(280))), F32(c.m.r32((l_q_5).wrapping_add(284)))) }) });
                            l_osc_10 = rt::fadd(F32(c.m.r32((l_q_5).wrapping_add(276))), rt::fmul(F32(c.m.r32((l_q_5).wrapping_add(244))), l_s_9));
                            if ((((!(((((c.m.r32((l_q_5).wrapping_add(356)) as i32)) & (4096i32))) != 0)) as i32)) != 0) {
                                { let v_ = rt::fadd(F32(c.m.r32((l_q_5).wrapping_add(280))), F32(0x3f800000)); let a_ = (l_q_5).wrapping_add(280); c.m.w32(a_, (v_).0); };
                            } else {
                                { let v_ = rt::fsub(F32(c.m.r32((l_q_5).wrapping_add(280))), F32(0x3f800000)); let a_ = (l_q_5).wrapping_add(280); c.m.w32(a_, (v_).0); };
                            }
                            if (((((((rt::flt(F32(c.m.r32((l_q_5).wrapping_add(280))), F32(0x00000000)) as i32)) != 0) || (((rt::fle(F32(c.m.r32((l_q_5).wrapping_add(284))), F32(c.m.r32((l_q_5).wrapping_add(280)))) as i32)) != 0)) as i32)) != 0) {
                                { let v_ = (if (((rt::flt(F32(c.m.r32((l_q_5).wrapping_add(280))), F32(0x00000000)) as i32)) != 0) { F32(0x00000000) } else { (if (((rt::flt(F32(c.m.r32((l_q_5).wrapping_add(284))), F32(c.m.r32((l_q_5).wrapping_add(280)))) as i32)) != 0) { F32(c.m.r32((l_q_5).wrapping_add(284))) } else { F32(c.m.r32((l_q_5).wrapping_add(280))) }) }); let a_ = (l_q_5).wrapping_add(280); c.m.w32(a_, (v_).0); };
                                { let r_ = 4096i32; let a_ = (l_q_5).wrapping_add(356); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
                            }
                        }
                        if (((rt::flt(l_t_8, F32(c.m.r32((l_def_2).wrapping_add(544)))) as i32)) != 0) {
                            { let r_ = F32(c.m.r32(((l_q_5).wrapping_add(268)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = (l_q_5).wrapping_add(264); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                            { let r_ = F32(c.m.r32(((l_q_5).wrapping_add(320)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = (l_q_5).wrapping_add(316); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                        } else {
                            { let r_ = F32(c.m.r32(((l_q_5).wrapping_add(268)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a_ = (l_q_5).wrapping_add(264); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                            { let r_ = F32(c.m.r32(((l_q_5).wrapping_add(320)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a_ = (l_q_5).wrapping_add(316); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                        }
                        if (((rt::flt(F32(c.m.r32((l_q_5).wrapping_add(264))), F32(0x00000000)) as i32)) != 0) {
                            { let v_ = F32(0x00000000); let a_ = (l_q_5).wrapping_add(264); c.m.w32(a_, (v_).0); };
                        }
                        if (((rt::flt(F32(c.m.r32((l_q_5).wrapping_add(316))), F32(0x00000000)) as i32)) != 0) {
                            { let v_ = F32(0x00000000); let a_ = (l_q_5).wrapping_add(316); c.m.w32(a_, (v_).0); };
                        }
                        { let v_ = rt::fmul(rt::fmul(F32(c.m.r32((l_q_5).wrapping_add(312))), F32(c.m.r32((l_q_5).wrapping_add(316)))), l_osc_10); let a_ = (l_q_5).wrapping_add(308); c.m.w32(a_, (v_).0); };
                        l_speed_11 = rt::fmul(F32(c.m.r32((l_q_5).wrapping_add(260))), F32(c.m.r32((l_q_5).wrapping_add(264))));
                        { let r_ = F32(c.m.r32(((l_q_5).wrapping_add(224)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = ((l_q_5).wrapping_add(112)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                        { let r_ = F32(c.m.r32(((l_q_5).wrapping_add(224)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a_ = ((l_q_5).wrapping_add(112)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                        { let r_ = F32(c.m.r32(((l_q_5).wrapping_add(224)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); let a_ = ((l_q_5).wrapping_add(112)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                        { let v_ = { let a0_ = F32(c.m.r32(((l_q_5).wrapping_add(112)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); F::EftMath_WrapAngle(c, a0_) }; let a_ = ((l_q_5).wrapping_add(112)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                        { let v_ = { let a0_ = F32(c.m.r32(((l_q_5).wrapping_add(112)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); F::EftMath_WrapAngle(c, a0_) }; let a_ = ((l_q_5).wrapping_add(112)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                        { let v_ = { let a0_ = F32(c.m.r32(((l_q_5).wrapping_add(112)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); F::EftMath_WrapAngle(c, a0_) }; let a_ = ((l_q_5).wrapping_add(112)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                        F::Vec3_Add(c, (l_q_5).wrapping_add(32), (l_q_5).wrapping_add(32), (l_q_5).wrapping_add(48));
                        F::Vec3_Scale(c, fp, (l_q_5).wrapping_add(32), l_speed_11);
                        F::Vec3_Add(c, l_q_5, l_q_5, fp);
                        {
                            rt::memset(c, fp.wrapping_add(16), 0, 12);
                            { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(16).wrapping_add(0), (v_).0); }
                            { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(16).wrapping_add(4), (v_).0); }
                            { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(16).wrapping_add(8), (v_).0); }
                            l_u_12 = F32(0x00000000);
                            if (((((c.m.r32((l_def_2).wrapping_add(564)) as i32)) & (128i32))) != 0) {
                                if (((rt::flt(F32(0x00000000), F32(c.m.r32((l_q_5).wrapping_add(304)))) as i32)) != 0) {
                                    l_u_12 = (if (((rt::flt(rt::fdiv(F32(c.m.r32((l_q_5).wrapping_add(300))), F32(c.m.r32((l_q_5).wrapping_add(304)))), F32(0x00000000)) as i32)) != 0) { F32(0x00000000) } else { (if (((rt::flt(F32(0x3f800000), rt::fdiv(F32(c.m.r32((l_q_5).wrapping_add(300))), F32(c.m.r32((l_q_5).wrapping_add(304))))) as i32)) != 0) { F32(0x3f800000) } else { rt::fdiv(F32(c.m.r32((l_q_5).wrapping_add(300))), F32(c.m.r32((l_q_5).wrapping_add(304)))) }) });
                                } else {
                                    l_u_12 = F32(0x3f800000);
                                }
                                if (((((c.m.r32((l_q_5).wrapping_add(356)) as i32)) & (65536i32))) != 0) {
                                    { let v_ = rt::fadd(F32(c.m.r32(((l_q_5).wrapping_add(288)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(((l_q_5).wrapping_add(248)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), l_u_12)); let a_ = (fp.wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                                } else {
                                    { let v_ = F32(c.m.r32(((l_q_5).wrapping_add(288)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = (fp.wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                                }
                                if (((((c.m.r32((l_q_5).wrapping_add(356)) as i32)) & (131072i32))) != 0) {
                                    { let v_ = rt::fadd(F32(c.m.r32(((l_q_5).wrapping_add(288)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(((l_q_5).wrapping_add(248)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), l_u_12)); let a_ = (fp.wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                                } else {
                                    { let v_ = F32(c.m.r32(((l_q_5).wrapping_add(288)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a_ = (fp.wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                                }
                                if (((((c.m.r32((l_q_5).wrapping_add(356)) as i32)) & (262144i32))) != 0) {
                                    { let v_ = rt::fadd(F32(c.m.r32(((l_q_5).wrapping_add(288)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(((l_q_5).wrapping_add(248)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), l_u_12)); let a_ = (fp.wrapping_add(16)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                                } else {
                                    { let v_ = F32(c.m.r32(((l_q_5).wrapping_add(288)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); let a_ = (fp.wrapping_add(16)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                                }
                                { let v_ = (if (((rt::flt(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(0x00000000)) as i32)) != 0) { F32(0x00000000) } else { (if (((rt::flt(F32(0x3f800000), F32(c.m.r32((fp.wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4))))) as i32)) != 0) { F32(0x3f800000) } else { F32(c.m.r32((fp.wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))) }) }); let a_ = (fp.wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                                { let v_ = (if (((rt::flt(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(0x00000000)) as i32)) != 0) { F32(0x00000000) } else { (if (((rt::flt(F32(0x3f800000), F32(c.m.r32((fp.wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(4))))) as i32)) != 0) { F32(0x3f800000) } else { F32(c.m.r32((fp.wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))) }) }); let a_ = (fp.wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                                { let v_ = (if (((rt::flt(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), F32(0x00000000)) as i32)) != 0) { F32(0x00000000) } else { (if (((rt::flt(F32(0x3f800000), F32(c.m.r32((fp.wrapping_add(16)).wrapping_add((((2i32) as u32)).wrapping_mul(4))))) as i32)) != 0) { F32(0x3f800000) } else { F32(c.m.r32((fp.wrapping_add(16)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))) }) }); let a_ = (fp.wrapping_add(16)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                                if ((((!(((((c.m.r32((l_q_5).wrapping_add(356)) as i32)) & (524288i32))) != 0)) as i32)) != 0) {
                                    { let v_ = rt::fadd(F32(c.m.r32((l_q_5).wrapping_add(300))), F32(0x3f800000)); let a_ = (l_q_5).wrapping_add(300); c.m.w32(a_, (v_).0); };
                                    if (((rt::fle(F32(c.m.r32((l_q_5).wrapping_add(304))), F32(c.m.r32((l_q_5).wrapping_add(300)))) as i32)) != 0) {
                                        { let v_ = F32(c.m.r32((l_q_5).wrapping_add(304))); let a_ = (l_q_5).wrapping_add(300); c.m.w32(a_, (v_).0); };
                                        { let r_ = 524288i32; let a_ = (l_q_5).wrapping_add(356); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                                    }
                                } else {
                                    { let v_ = rt::fsub(F32(c.m.r32((l_q_5).wrapping_add(300))), F32(0x3f800000)); let a_ = (l_q_5).wrapping_add(300); c.m.w32(a_, (v_).0); };
                                    if (((rt::fle(F32(c.m.r32((l_q_5).wrapping_add(300))), F32(0x00000000)) as i32)) != 0) {
                                        { let v_ = F32(0x00000000); let a_ = (l_q_5).wrapping_add(300); c.m.w32(a_, (v_).0); };
                                        { let r_ = (!(524288i32)); let a_ = (l_q_5).wrapping_add(356); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                                    }
                                }
                            }
                            l_r_13 = F32(0x00000000);
                            if ((((!(((((c.m.r32((l_q_5).wrapping_add(356)) as i32)) & (64i32))) != 0)) as i32)) != 0) {
                                if (((((c.m.r32((l_q_5).wrapping_add(356)) as i32)) & (32768i32))) != 0) {
                                    if (((((((rt::fle(F32(c.m.r32((l_q_5).wrapping_add(348))), l_t_8) as i32)) != 0) && (((rt::flt(l_t_8, F32(c.m.r32((l_q_5).wrapping_add(352)))) as i32)) != 0)) as i32)) != 0) {
                                        l_r_13 = rt::fdiv(F32(c.m.r32((l_q_5).wrapping_add(332))), F32(c.m.r32((l_q_5).wrapping_add(328))));
                                        { let r_ = F32(0x3f800000); let a_ = (l_q_5).wrapping_add(332); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                                    } else {
                                        if (((rt::fle(F32(c.m.r32((l_q_5).wrapping_add(352))), l_t_8) as i32)) != 0) {
                                            l_r_13 = F32(0x3f800000);
                                        }
                                    }
                                } else {
                                    l_r_13 = l_t_8;
                                }
                                { let v_ = rt::fmul(rt::fadd(F32(c.m.r32(((l_q_5).wrapping_add(96)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(((l_q_5).wrapping_add(80)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), l_r_13)), F32(c.m.r32((fp.wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a_ = ((l_q_5).wrapping_add(64)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                                { let v_ = rt::fmul(rt::fadd(F32(c.m.r32(((l_q_5).wrapping_add(96)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(((l_q_5).wrapping_add(80)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), l_r_13)), F32(c.m.r32((fp.wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); let a_ = ((l_q_5).wrapping_add(64)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                                { let v_ = rt::fmul(rt::fadd(F32(c.m.r32(((l_q_5).wrapping_add(96)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(((l_q_5).wrapping_add(80)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), l_r_13)), F32(c.m.r32((fp.wrapping_add(16)).wrapping_add((((2i32) as u32)).wrapping_mul(4))))); let a_ = ((l_q_5).wrapping_add(64)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                            } else {
                                if (((((c.m.r32((l_def_2).wrapping_add(564)) as i32)) & (128i32))) != 0) {
                                    { let v_ = rt::fmul(F32(c.m.r32(((l_q_5).wrapping_add(96)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((fp.wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a_ = ((l_q_5).wrapping_add(64)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                                    { let v_ = rt::fmul(F32(c.m.r32(((l_q_5).wrapping_add(96)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((fp.wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); let a_ = ((l_q_5).wrapping_add(64)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                                    { let v_ = rt::fmul(F32(c.m.r32(((l_q_5).wrapping_add(96)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((fp.wrapping_add(16)).wrapping_add((((2i32) as u32)).wrapping_mul(4))))); let a_ = ((l_q_5).wrapping_add(64)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                                }
                            }
                        }
                        if (((((((rt::flt(l_t_8, F32(c.m.r32((l_def_2).wrapping_add(536)))) as i32)) != 0) && (((rt::flt(F32(0x00000000), F32(c.m.r32((l_q_5).wrapping_add(236)))) as i32)) != 0)) as i32)) != 0) {
                            l_fade_6 = rt::fdiv(F32(c.m.r32((l_q_5).wrapping_add(340))), F32(c.m.r32((l_q_5).wrapping_add(236))));
                            if (((rt::flt(F32(0x3f800000), l_fade_6) as i32)) != 0) {
                                l_fade_6 = F32(0x3f800000);
                            }
                        } else {
                            l_fade_6 = F32(0x3f800000);
                        }
                        if (((((((rt::fle(F32(c.m.r32((l_def_2).wrapping_add(540))), l_t_8) as i32)) != 0) && (((rt::flt(F32(0x00000000), F32(c.m.r32((l_q_5).wrapping_add(240)))) as i32)) != 0)) as i32)) != 0) {
                            l_fade_6 = rt::fsub(F32(0x3f800000), rt::fdiv(rt::fsub(F32(c.m.r32((l_q_5).wrapping_add(340))), rt::fsub(F32(c.m.r32((l_q_5).wrapping_add(336))), F32(c.m.r32((l_q_5).wrapping_add(240))))), F32(c.m.r32((l_q_5).wrapping_add(240)))));
                            if (((rt::flt(l_fade_6, F32(0x00000000)) as i32)) != 0) {
                                l_fade_6 = F32(0x00000000);
                            }
                        }
                        { let v_ = rt::fmul(rt::fmul(F32(c.m.r32((l_q_5).wrapping_add(108))), l_fade_6), l_alpha_7); let a_ = ((l_q_5).wrapping_add(64)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                        F::Vec4_Clamp(c, (l_q_5).wrapping_add(64), (l_q_5).wrapping_add(64), F32(0x00000000), F32(0x437f0000));
                        if (((((c.m.r32((p_w_1).wrapping_add(964)) as i32)) & (1024i32))) != 0) {
                            l_frame_14 = ((rt::f2u(F32(c.m.r32((l_q_5).wrapping_add(344))))) as u8);
                            if (((((((l_frame_14) as i32)) >= (((c.m.r8((p_w_1).wrapping_add(956))) as i32))) as i32)) != 0) {
                                l_frame_14 = ((0i32) as u8);
                            }
                            if (((((((((c.m.r32((l_def_2).wrapping_add(564)) as i32)) & (4i32))) != 0) && (((rt::flt(F32(c.m.r32(((l_q_5).wrapping_add(224)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), F32(0x00000000)) as i32)) != 0)) as i32)) != 0) {
                                { let v_ = F32(c.m.r32((((p_w_1).wrapping_add(120)).wrapping_add((((l_frame_14) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); let a_ = ((l_q_5).wrapping_add(192)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w32(a_, (v_).0); };
                                { let v_ = F32(c.m.r32((((p_w_1).wrapping_add(120)).wrapping_add((((l_frame_14) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a_ = (((l_q_5).wrapping_add(192)).wrapping_add((((0i32) as u32)).wrapping_mul(8))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                                { let v_ = F32(c.m.r32((((p_w_1).wrapping_add(120)).wrapping_add((((l_frame_14) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = ((l_q_5).wrapping_add(192)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w32(a_, (v_).0); };
                                { let v_ = F32(c.m.r32((((p_w_1).wrapping_add(120)).wrapping_add((((l_frame_14) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a_ = (((l_q_5).wrapping_add(192)).wrapping_add((((1i32) as u32)).wrapping_mul(8))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                                { let v_ = F32(c.m.r32((((p_w_1).wrapping_add(120)).wrapping_add((((l_frame_14) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); let a_ = ((l_q_5).wrapping_add(192)).wrapping_add((((2i32) as u32)).wrapping_mul(8)); c.m.w32(a_, (v_).0); };
                                { let v_ = F32(c.m.r32((((p_w_1).wrapping_add(120)).wrapping_add((((l_frame_14) as u32)).wrapping_mul(16))).wrapping_add((((3i32) as u32)).wrapping_mul(4)))); let a_ = (((l_q_5).wrapping_add(192)).wrapping_add((((2i32) as u32)).wrapping_mul(8))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                                { let v_ = F32(c.m.r32((((p_w_1).wrapping_add(120)).wrapping_add((((l_frame_14) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = ((l_q_5).wrapping_add(192)).wrapping_add((((3i32) as u32)).wrapping_mul(8)); c.m.w32(a_, (v_).0); };
                                { let v_ = F32(c.m.r32((((p_w_1).wrapping_add(120)).wrapping_add((((l_frame_14) as u32)).wrapping_mul(16))).wrapping_add((((3i32) as u32)).wrapping_mul(4)))); let a_ = (((l_q_5).wrapping_add(192)).wrapping_add((((3i32) as u32)).wrapping_mul(8))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                            } else {
                                { let v_ = F32(c.m.r32((((p_w_1).wrapping_add(120)).wrapping_add((((l_frame_14) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = ((l_q_5).wrapping_add(192)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w32(a_, (v_).0); };
                                { let v_ = F32(c.m.r32((((p_w_1).wrapping_add(120)).wrapping_add((((l_frame_14) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a_ = (((l_q_5).wrapping_add(192)).wrapping_add((((0i32) as u32)).wrapping_mul(8))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                                { let v_ = F32(c.m.r32((((p_w_1).wrapping_add(120)).wrapping_add((((l_frame_14) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); let a_ = ((l_q_5).wrapping_add(192)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w32(a_, (v_).0); };
                                { let v_ = F32(c.m.r32((((p_w_1).wrapping_add(120)).wrapping_add((((l_frame_14) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a_ = (((l_q_5).wrapping_add(192)).wrapping_add((((1i32) as u32)).wrapping_mul(8))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                                { let v_ = F32(c.m.r32((((p_w_1).wrapping_add(120)).wrapping_add((((l_frame_14) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = ((l_q_5).wrapping_add(192)).wrapping_add((((2i32) as u32)).wrapping_mul(8)); c.m.w32(a_, (v_).0); };
                                { let v_ = F32(c.m.r32((((p_w_1).wrapping_add(120)).wrapping_add((((l_frame_14) as u32)).wrapping_mul(16))).wrapping_add((((3i32) as u32)).wrapping_mul(4)))); let a_ = (((l_q_5).wrapping_add(192)).wrapping_add((((2i32) as u32)).wrapping_mul(8))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                                { let v_ = F32(c.m.r32((((p_w_1).wrapping_add(120)).wrapping_add((((l_frame_14) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); let a_ = ((l_q_5).wrapping_add(192)).wrapping_add((((3i32) as u32)).wrapping_mul(8)); c.m.w32(a_, (v_).0); };
                                { let v_ = F32(c.m.r32((((p_w_1).wrapping_add(120)).wrapping_add((((l_frame_14) as u32)).wrapping_mul(16))).wrapping_add((((3i32) as u32)).wrapping_mul(4)))); let a_ = (((l_q_5).wrapping_add(192)).wrapping_add((((3i32) as u32)).wrapping_mul(8))).wrapping_add(4); c.m.w32(a_, (v_).0); };
                            }
                            if (((((rt::rem_i32(rt::f2i(F32(c.m.r32((l_q_5).wrapping_add(340)))), ((c.m.r8((l_def_2).wrapping_add(577))) as i32))) == (0i32)) as i32)) != 0) {
                                { let r_ = F32(0x3f800000); let a_ = (l_q_5).wrapping_add(344); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                                if (((((((((rt::f2u(F32(c.m.r32((l_q_5).wrapping_add(344))))) as u8)) as i32)) >= (((c.m.r8((p_w_1).wrapping_add(956))) as i32))) as i32)) != 0) {
                                    { let v_ = F32(0x00000000); let a_ = (l_q_5).wrapping_add(344); c.m.w32(a_, (v_).0); };
                                }
                            }
                        }
                        { let r_ = F32(0x3f800000); let a_ = (l_q_5).wrapping_add(340); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                        if (((rt::fle(F32(c.m.r32((l_q_5).wrapping_add(336))), F32(c.m.r32((l_q_5).wrapping_add(340)))) as i32)) != 0) {
                            { let r_ = 16i32; let a_ = (l_q_5).wrapping_add(356); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                        }
                    }
                    if (((((((rt::fle(F32(c.m.r32(((l_q_5).wrapping_add(64)).wrapping_add(12))), F32(0x00000000)) as i32)) != 0) || (((rt::fle(F32(c.m.r32((l_q_5).wrapping_add(308))), F32(0x00000000)) as i32)) != 0)) as i32)) != 0) {
                        { let r_ = (!(512i32)); let a_ = (l_q_5).wrapping_add(356); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                    } else {
                        F::Vu0Cur_Push(c);
                        F::Vu0Cur_LoadIdentity(c);
                        { let a0_ = F32(c.m.r32((p_w_1).wrapping_add(92))); F::Vu0Cur_ScaleDiagUniform(c, a0_) };
                        if (((((c.m.r32((p_w_1).wrapping_add(964)) as i32)) & (256i32))) != 0) {
                            F::EftQuad_BuildCorners(c, l_q_5, p_w_1, (l_q_5).wrapping_add(16), l_pitch_3, l_yaw_4);
                        } else {
                            F::EftQuad_BuildCorners(c, l_q_5, p_w_1, (p_w_1).wrapping_add(48), l_pitch_3, l_yaw_4);
                        }
                        F::Vu0Cur_Pop(c);
                        { let r_ = 512i32; let a_ = (l_q_5).wrapping_add(356); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    }
                    if (((((((((c.m.r32((l_q_5).wrapping_add(356)) as i32)) & (16i32))) != 0) || (((((c.m.r32((p_w_1).wrapping_add(964)) as i32)) & (32i32))) != 0)) as i32)) != 0) {
                        { let v_ = 0i32; let a_ = (l_q_5).wrapping_add(356); c.m.w32(a_, (v_) as u32); };
                        F::EftQuad_ListRemove(c, (p_w_1).wrapping_add(968), (p_w_1).wrapping_add(972), l_q_5);
                    }
                }
                l_q_5 = c.m.r32((l_q_5).wrapping_add(364));
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftQuad_StepAll", &PROBE_N, &[], true); }
    r_
}

pub fn EftQuad_BuildCorners(c: &mut Ctx, mut p_q_1: u32, mut p_w_2: u32, p_pos_3: u32, mut p_pitch_4: F32, mut p_yaw_5: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftQuad_BuildCorners", &PROBE_N, &[(p_q_1) as u32, (p_w_2) as u32, (p_pos_3) as u32, (p_pitch_4).0, (p_yaw_5).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_def_6: u32 = 0;
    let mut l_h_7: F32 = F32(0);
    let mut l_i_8: i32 = 0;
    rt::memcpy(c, fp, p_pos_3, 16);
    let r_ = 'f: {
        l_def_6 = c.m.r32(p_w_2);
        { let _ = F::memset(c, fp.wrapping_add(16), 0i32, 16u32); };
        { let a0_ = F32(c.m.r32(((p_q_1).wrapping_add(112)).wrapping_add(8))); F::Vu0Cur_RotateZ(c, a0_) };
        { let a0_ = F32(c.m.r32((p_q_1).wrapping_add(112))); F::Vu0Cur_RotateX(c, a0_) };
        { let a0_ = F32(c.m.r32(((p_q_1).wrapping_add(112)).wrapping_add(4))); F::Vu0Cur_RotateY(c, a0_) };
        { let a2_ = F32(c.m.r32((p_w_2).wrapping_add(92))); F::Vec3_Scale(c, fp.wrapping_add(16), p_q_1, a2_) };
        F::Vu0Cur_Translate(c, fp.wrapping_add(16));
        F::Vu0Cur_RotateX(c, p_pitch_4);
        F::Vu0Cur_RotateY(c, p_yaw_5);
        F::Vu0Cur_Translate(c, fp);
        if ((((!(((((c.m.r32((l_def_6).wrapping_add(564)) as i32)) & (64i32))) != 0)) as i32)) != 0) {
            l_h_7 = F32(c.m.r32((p_q_1).wrapping_add(308)));
        } else {
            l_h_7 = rt::fmul(F32(c.m.r32((p_q_1).wrapping_add(308))), F32(0x3f000000));
        }
        F::Vec4_Set(c, ((p_q_1).wrapping_add(128)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), rt::fneg(l_h_7), rt::fneg(l_h_7), F32(0x00000000), F32(0x3f800000));
        F::Vec4_Set(c, ((p_q_1).wrapping_add(128)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), l_h_7, rt::fneg(l_h_7), F32(0x00000000), F32(0x3f800000));
        F::Vec4_Set(c, ((p_q_1).wrapping_add(128)).wrapping_add((((2i32) as u32)).wrapping_mul(16)), rt::fneg(l_h_7), l_h_7, F32(0x00000000), F32(0x3f800000));
        F::Vec4_Set(c, ((p_q_1).wrapping_add(128)).wrapping_add((((3i32) as u32)).wrapping_mul(16)), l_h_7, l_h_7, F32(0x00000000), F32(0x3f800000));
        {
            l_i_8 = 0i32;
            'l1: loop {
                if !(((((l_i_8) < (4i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    F::Vu0Cur_MulVec4(c, ((p_q_1).wrapping_add(128)).wrapping_add((((l_i_8) as u32)).wrapping_mul(16)), ((p_q_1).wrapping_add(128)).wrapping_add((((l_i_8) as u32)).wrapping_mul(16)));
                }
                { let o_ = l_i_8; let n_ = (o_).wrapping_add(1); l_i_8 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftQuad_BuildCorners", &PROBE_N, &[], true); }
    r_
}

pub fn EftQuad_DrawAllFacing(c: &mut Ctx, mut p_w_1: u32, mut p_layer_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftQuad_DrawAllFacing", &PROBE_N, &[(p_w_1) as u32, (p_layer_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(128) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 128);
    let mut l_q_3: u32 = 0;
    let mut l_i_4: i32 = 0;
    let mut l_j_5: i32 = 0;
    let r_ = 'f: {
        rt::memset(c, fp.wrapping_add(112), 0, 16);
        rt::memset(c, fp.wrapping_add(112).wrapping_add(0), 0, 4);
        { let v_ = ((0i32) as u8); c.m.w8(fp.wrapping_add(112).wrapping_add(0).wrapping_add(0), v_); }
        { let v_ = ((1i32) as u8); c.m.w8(fp.wrapping_add(112).wrapping_add(0).wrapping_add(1), v_); }
        { let v_ = ((2i32) as u8); c.m.w8(fp.wrapping_add(112).wrapping_add(0).wrapping_add(2), v_); }
        { let v_ = ((3i32) as u8); c.m.w8(fp.wrapping_add(112).wrapping_add(0).wrapping_add(3), v_); }
        rt::memset(c, fp.wrapping_add(112).wrapping_add(4), 0, 4);
        { let v_ = ((1i32) as u8); c.m.w8(fp.wrapping_add(112).wrapping_add(4).wrapping_add(0), v_); }
        { let v_ = ((0i32) as u8); c.m.w8(fp.wrapping_add(112).wrapping_add(4).wrapping_add(1), v_); }
        { let v_ = ((3i32) as u8); c.m.w8(fp.wrapping_add(112).wrapping_add(4).wrapping_add(2), v_); }
        { let v_ = ((2i32) as u8); c.m.w8(fp.wrapping_add(112).wrapping_add(4).wrapping_add(3), v_); }
        rt::memset(c, fp.wrapping_add(112).wrapping_add(8), 0, 4);
        { let v_ = ((2i32) as u8); c.m.w8(fp.wrapping_add(112).wrapping_add(8).wrapping_add(0), v_); }
        { let v_ = ((3i32) as u8); c.m.w8(fp.wrapping_add(112).wrapping_add(8).wrapping_add(1), v_); }
        { let v_ = ((0i32) as u8); c.m.w8(fp.wrapping_add(112).wrapping_add(8).wrapping_add(2), v_); }
        { let v_ = ((1i32) as u8); c.m.w8(fp.wrapping_add(112).wrapping_add(8).wrapping_add(3), v_); }
        rt::memset(c, fp.wrapping_add(112).wrapping_add(12), 0, 4);
        { let v_ = ((3i32) as u8); c.m.w8(fp.wrapping_add(112).wrapping_add(12).wrapping_add(0), v_); }
        { let v_ = ((2i32) as u8); c.m.w8(fp.wrapping_add(112).wrapping_add(12).wrapping_add(1), v_); }
        { let v_ = ((1i32) as u8); c.m.w8(fp.wrapping_add(112).wrapping_add(12).wrapping_add(2), v_); }
        { let v_ = ((0i32) as u8); c.m.w8(fp.wrapping_add(112).wrapping_add(12).wrapping_add(3), v_); }
        if ((((!(((({ let a_ = (c.m.r32(p_w_1)).wrapping_add(564); (c.m.r32(a_) as i32) }) & (64i32))) != 0)) as i32)) != 0) {
            {
                l_q_3 = c.m.r32((p_w_1).wrapping_add(968));
                'l1: loop {
                    if !(((((l_q_3) != (((0i32) as u32))) as i32)) != 0) { break 'l1; }
                    'c2: {
                        if (((((c.m.r32((l_q_3).wrapping_add(356)) as i32)) & (512i32))) != 0) {
                            { let a1_ = F32(c.m.r32(((l_q_3).wrapping_add(192)).wrapping_add((((0i32) as u32)).wrapping_mul(8)))); let a2_ = F32(c.m.r32((((l_q_3).wrapping_add(192)).wrapping_add((((0i32) as u32)).wrapping_mul(8))).wrapping_add(4))); let a3_ = F32(c.m.r32(((l_q_3).wrapping_add(192)).wrapping_add((((1i32) as u32)).wrapping_mul(8)))); let a4_ = F32(c.m.r32((((l_q_3).wrapping_add(192)).wrapping_add((((1i32) as u32)).wrapping_mul(8))).wrapping_add(4))); F::Vec4_Set(c, fp.wrapping_add(80), a1_, a2_, a3_, a4_) };
                            { let a1_ = F32(c.m.r32(((l_q_3).wrapping_add(192)).wrapping_add((((2i32) as u32)).wrapping_mul(8)))); let a2_ = F32(c.m.r32((((l_q_3).wrapping_add(192)).wrapping_add((((2i32) as u32)).wrapping_mul(8))).wrapping_add(4))); let a3_ = F32(c.m.r32(((l_q_3).wrapping_add(192)).wrapping_add((((3i32) as u32)).wrapping_mul(8)))); let a4_ = F32(c.m.r32((((l_q_3).wrapping_add(192)).wrapping_add((((3i32) as u32)).wrapping_mul(8))).wrapping_add(4))); F::Vec4_Set(c, fp.wrapping_add(96), a1_, a2_, a3_, a4_) };
                            { let a5_ = ((c.m.r8((l_q_3).wrapping_add(360))) as i32); let a7_ = ((((((c.m.r32((p_w_1).wrapping_add(964)) as i32)) & (128i32))) != (0i32)) as i32); let a8_ = c.m.r32((p_w_1).wrapping_add(8)); F::EftQuad_DrawFacing(c, (l_q_3).wrapping_add(128), fp.wrapping_add(80), fp.wrapping_add(96), (l_q_3).wrapping_add(64), p_layer_2, a5_, 0i32, a7_, a8_) };
                        }
                    }
                    l_q_3 = c.m.r32((l_q_3).wrapping_add(364));
                }
            }
        } else {
            {
                l_q_3 = c.m.r32((p_w_1).wrapping_add(968));
                'l3: loop {
                    if !(((((l_q_3) != (((0i32) as u32))) as i32)) != 0) { break 'l3; }
                    'c4: {
                        if (((((c.m.r32((l_q_3).wrapping_add(356)) as i32)) & (512i32))) != 0) {
                            {
                                l_i_4 = 0i32;
                                'l5: loop {
                                    if !(((((l_i_4) < (4i32)) as i32)) != 0) { break 'l5; }
                                    'c6: {
                                        F::Vec3_Sub(c, fp.wrapping_add(64), ((l_q_3).wrapping_add(128)).wrapping_add((((l_i_4) as u32)).wrapping_mul(16)), ((l_q_3).wrapping_add(128)).wrapping_add(((((3i32).wrapping_sub(l_i_4)) as u32)).wrapping_mul(16)));
                                        {
                                            l_j_5 = 0i32;
                                            'l7: loop {
                                                if !(((((l_j_5) < (4i32)) as i32)) != 0) { break 'l7; }
                                                'c8: {
                                                    F::Vec3_ScaleAdd(c, (fp).wrapping_add((((l_j_5) as u32)).wrapping_mul(16)), fp.wrapping_add(64), ((l_q_3).wrapping_add(128)).wrapping_add((((l_j_5) as u32)).wrapping_mul(16)), F32(0x3f000000));
                                                }
                                                { let o_ = l_j_5; let n_ = (o_).wrapping_add(1); l_j_5 = n_; };
                                            }
                                        }
                                        { let a1_ = { let a_ = (((l_q_3).wrapping_add(192)).wrapping_add((((c.m.r8(((fp.wrapping_add(112)).wrapping_add((((l_i_4) as u32)).wrapping_mul(4))).wrapping_add(((0i32) as u32)))) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }; let a2_ = { let a_ = (((l_q_3).wrapping_add(192)).wrapping_add((((c.m.r8(((fp.wrapping_add(112)).wrapping_add((((l_i_4) as u32)).wrapping_mul(4))).wrapping_add(((0i32) as u32)))) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }; let a3_ = { let a_ = (((l_q_3).wrapping_add(192)).wrapping_add((((c.m.r8(((fp.wrapping_add(112)).wrapping_add((((l_i_4) as u32)).wrapping_mul(4))).wrapping_add(((1i32) as u32)))) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }; let a4_ = { let a_ = (((l_q_3).wrapping_add(192)).wrapping_add((((c.m.r8(((fp.wrapping_add(112)).wrapping_add((((l_i_4) as u32)).wrapping_mul(4))).wrapping_add(((1i32) as u32)))) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }; F::Vec4_Set(c, fp.wrapping_add(80), a1_, a2_, a3_, a4_) };
                                        { let a1_ = { let a_ = (((l_q_3).wrapping_add(192)).wrapping_add((((c.m.r8(((fp.wrapping_add(112)).wrapping_add((((l_i_4) as u32)).wrapping_mul(4))).wrapping_add(((2i32) as u32)))) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }; let a2_ = { let a_ = (((l_q_3).wrapping_add(192)).wrapping_add((((c.m.r8(((fp.wrapping_add(112)).wrapping_add((((l_i_4) as u32)).wrapping_mul(4))).wrapping_add(((2i32) as u32)))) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }; let a3_ = { let a_ = (((l_q_3).wrapping_add(192)).wrapping_add((((c.m.r8(((fp.wrapping_add(112)).wrapping_add((((l_i_4) as u32)).wrapping_mul(4))).wrapping_add(((3i32) as u32)))) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }; let a4_ = { let a_ = (((l_q_3).wrapping_add(192)).wrapping_add((((c.m.r8(((fp.wrapping_add(112)).wrapping_add((((l_i_4) as u32)).wrapping_mul(4))).wrapping_add(((3i32) as u32)))) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }; F::Vec4_Set(c, fp.wrapping_add(96), a1_, a2_, a3_, a4_) };
                                        { let a5_ = ((c.m.r8((l_q_3).wrapping_add(360))) as i32); let a7_ = ((((((c.m.r32((p_w_1).wrapping_add(964)) as i32)) & (128i32))) != (0i32)) as i32); let a8_ = c.m.r32((p_w_1).wrapping_add(8)); F::EftQuad_DrawFacing(c, fp, fp.wrapping_add(80), fp.wrapping_add(96), (l_q_3).wrapping_add(64), p_layer_2, a5_, 0i32, a7_, a8_) };
                                    }
                                    { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
                                }
                            }
                        }
                    }
                    l_q_3 = c.m.r32((l_q_3).wrapping_add(364));
                }
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftQuad_DrawAllFacing", &PROBE_N, &[], true); }
    r_
}

pub fn EftQuad_DrawAllSprite(c: &mut Ctx, mut p_w_1: u32, mut p_layer_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftQuad_DrawAllSprite", &PROBE_N, &[(p_w_1) as u32, (p_layer_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(128) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 128);
    let mut l_q_3: u32 = 0;
    let mut l_i_4: i32 = 0;
    let mut l_j_5: i32 = 0;
    let r_ = 'f: {
        rt::memset(c, fp.wrapping_add(112), 0, 16);
        rt::memset(c, fp.wrapping_add(112).wrapping_add(0), 0, 4);
        { let v_ = ((0i32) as u8); c.m.w8(fp.wrapping_add(112).wrapping_add(0).wrapping_add(0), v_); }
        { let v_ = ((1i32) as u8); c.m.w8(fp.wrapping_add(112).wrapping_add(0).wrapping_add(1), v_); }
        { let v_ = ((2i32) as u8); c.m.w8(fp.wrapping_add(112).wrapping_add(0).wrapping_add(2), v_); }
        { let v_ = ((3i32) as u8); c.m.w8(fp.wrapping_add(112).wrapping_add(0).wrapping_add(3), v_); }
        rt::memset(c, fp.wrapping_add(112).wrapping_add(4), 0, 4);
        { let v_ = ((1i32) as u8); c.m.w8(fp.wrapping_add(112).wrapping_add(4).wrapping_add(0), v_); }
        { let v_ = ((0i32) as u8); c.m.w8(fp.wrapping_add(112).wrapping_add(4).wrapping_add(1), v_); }
        { let v_ = ((3i32) as u8); c.m.w8(fp.wrapping_add(112).wrapping_add(4).wrapping_add(2), v_); }
        { let v_ = ((2i32) as u8); c.m.w8(fp.wrapping_add(112).wrapping_add(4).wrapping_add(3), v_); }
        rt::memset(c, fp.wrapping_add(112).wrapping_add(8), 0, 4);
        { let v_ = ((2i32) as u8); c.m.w8(fp.wrapping_add(112).wrapping_add(8).wrapping_add(0), v_); }
        { let v_ = ((3i32) as u8); c.m.w8(fp.wrapping_add(112).wrapping_add(8).wrapping_add(1), v_); }
        { let v_ = ((0i32) as u8); c.m.w8(fp.wrapping_add(112).wrapping_add(8).wrapping_add(2), v_); }
        { let v_ = ((1i32) as u8); c.m.w8(fp.wrapping_add(112).wrapping_add(8).wrapping_add(3), v_); }
        rt::memset(c, fp.wrapping_add(112).wrapping_add(12), 0, 4);
        { let v_ = ((3i32) as u8); c.m.w8(fp.wrapping_add(112).wrapping_add(12).wrapping_add(0), v_); }
        { let v_ = ((2i32) as u8); c.m.w8(fp.wrapping_add(112).wrapping_add(12).wrapping_add(1), v_); }
        { let v_ = ((1i32) as u8); c.m.w8(fp.wrapping_add(112).wrapping_add(12).wrapping_add(2), v_); }
        { let v_ = ((0i32) as u8); c.m.w8(fp.wrapping_add(112).wrapping_add(12).wrapping_add(3), v_); }
        if ((((!(((({ let a_ = (c.m.r32(p_w_1)).wrapping_add(564); (c.m.r32(a_) as i32) }) & (64i32))) != 0)) as i32)) != 0) {
            {
                l_q_3 = c.m.r32((p_w_1).wrapping_add(968));
                'l1: loop {
                    if !(((((l_q_3) != (((0i32) as u32))) as i32)) != 0) { break 'l1; }
                    'c2: {
                        if (((((c.m.r32((l_q_3).wrapping_add(356)) as i32)) & (512i32))) != 0) {
                            { let a1_ = F32(c.m.r32(((l_q_3).wrapping_add(192)).wrapping_add((((0i32) as u32)).wrapping_mul(8)))); let a2_ = F32(c.m.r32((((l_q_3).wrapping_add(192)).wrapping_add((((0i32) as u32)).wrapping_mul(8))).wrapping_add(4))); let a3_ = F32(c.m.r32(((l_q_3).wrapping_add(192)).wrapping_add((((1i32) as u32)).wrapping_mul(8)))); let a4_ = F32(c.m.r32((((l_q_3).wrapping_add(192)).wrapping_add((((1i32) as u32)).wrapping_mul(8))).wrapping_add(4))); F::Vec4_Set(c, fp.wrapping_add(80), a1_, a2_, a3_, a4_) };
                            { let a1_ = F32(c.m.r32(((l_q_3).wrapping_add(192)).wrapping_add((((2i32) as u32)).wrapping_mul(8)))); let a2_ = F32(c.m.r32((((l_q_3).wrapping_add(192)).wrapping_add((((2i32) as u32)).wrapping_mul(8))).wrapping_add(4))); let a3_ = F32(c.m.r32(((l_q_3).wrapping_add(192)).wrapping_add((((3i32) as u32)).wrapping_mul(8)))); let a4_ = F32(c.m.r32((((l_q_3).wrapping_add(192)).wrapping_add((((3i32) as u32)).wrapping_mul(8))).wrapping_add(4))); F::Vec4_Set(c, fp.wrapping_add(96), a1_, a2_, a3_, a4_) };
                            { let a5_ = ((c.m.r8((l_q_3).wrapping_add(360))) as i32); let a6_ = ((((((c.m.r32((p_w_1).wrapping_add(964)) as i32)) & (128i32))) != (0i32)) as i32); let a7_ = c.m.r32((p_w_1).wrapping_add(8)); F::EftQuad_DrawSprite(c, (l_q_3).wrapping_add(128), fp.wrapping_add(80), fp.wrapping_add(96), (l_q_3).wrapping_add(64), p_layer_2, a5_, a6_, a7_) };
                        }
                    }
                    l_q_3 = c.m.r32((l_q_3).wrapping_add(364));
                }
            }
        } else {
            {
                l_q_3 = c.m.r32((p_w_1).wrapping_add(968));
                'l3: loop {
                    if !(((((l_q_3) != (((0i32) as u32))) as i32)) != 0) { break 'l3; }
                    'c4: {
                        if (((((c.m.r32((l_q_3).wrapping_add(356)) as i32)) & (512i32))) != 0) {
                            {
                                l_i_4 = 0i32;
                                'l5: loop {
                                    if !(((((l_i_4) < (4i32)) as i32)) != 0) { break 'l5; }
                                    'c6: {
                                        F::Vec3_Sub(c, fp.wrapping_add(64), ((l_q_3).wrapping_add(128)).wrapping_add((((l_i_4) as u32)).wrapping_mul(16)), ((l_q_3).wrapping_add(128)).wrapping_add(((((3i32).wrapping_sub(l_i_4)) as u32)).wrapping_mul(16)));
                                        {
                                            l_j_5 = 0i32;
                                            'l7: loop {
                                                if !(((((l_j_5) < (4i32)) as i32)) != 0) { break 'l7; }
                                                'c8: {
                                                    F::Vec3_ScaleAdd(c, (fp).wrapping_add((((l_j_5) as u32)).wrapping_mul(16)), fp.wrapping_add(64), ((l_q_3).wrapping_add(128)).wrapping_add((((l_j_5) as u32)).wrapping_mul(16)), F32(0x3f000000));
                                                }
                                                { let o_ = l_j_5; let n_ = (o_).wrapping_add(1); l_j_5 = n_; };
                                            }
                                        }
                                        { let a1_ = { let a_ = (((l_q_3).wrapping_add(192)).wrapping_add((((c.m.r8(((fp.wrapping_add(112)).wrapping_add((((l_i_4) as u32)).wrapping_mul(4))).wrapping_add(((0i32) as u32)))) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }; let a2_ = { let a_ = (((l_q_3).wrapping_add(192)).wrapping_add((((c.m.r8(((fp.wrapping_add(112)).wrapping_add((((l_i_4) as u32)).wrapping_mul(4))).wrapping_add(((0i32) as u32)))) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }; let a3_ = { let a_ = (((l_q_3).wrapping_add(192)).wrapping_add((((c.m.r8(((fp.wrapping_add(112)).wrapping_add((((l_i_4) as u32)).wrapping_mul(4))).wrapping_add(((1i32) as u32)))) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }; let a4_ = { let a_ = (((l_q_3).wrapping_add(192)).wrapping_add((((c.m.r8(((fp.wrapping_add(112)).wrapping_add((((l_i_4) as u32)).wrapping_mul(4))).wrapping_add(((1i32) as u32)))) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }; F::Vec4_Set(c, fp.wrapping_add(80), a1_, a2_, a3_, a4_) };
                                        { let a1_ = { let a_ = (((l_q_3).wrapping_add(192)).wrapping_add((((c.m.r8(((fp.wrapping_add(112)).wrapping_add((((l_i_4) as u32)).wrapping_mul(4))).wrapping_add(((2i32) as u32)))) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }; let a2_ = { let a_ = (((l_q_3).wrapping_add(192)).wrapping_add((((c.m.r8(((fp.wrapping_add(112)).wrapping_add((((l_i_4) as u32)).wrapping_mul(4))).wrapping_add(((2i32) as u32)))) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }; let a3_ = { let a_ = (((l_q_3).wrapping_add(192)).wrapping_add((((c.m.r8(((fp.wrapping_add(112)).wrapping_add((((l_i_4) as u32)).wrapping_mul(4))).wrapping_add(((3i32) as u32)))) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }; let a4_ = { let a_ = (((l_q_3).wrapping_add(192)).wrapping_add((((c.m.r8(((fp.wrapping_add(112)).wrapping_add((((l_i_4) as u32)).wrapping_mul(4))).wrapping_add(((3i32) as u32)))) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }; F::Vec4_Set(c, fp.wrapping_add(96), a1_, a2_, a3_, a4_) };
                                        { let a5_ = ((c.m.r8((l_q_3).wrapping_add(360))) as i32); let a6_ = ((((((c.m.r32((p_w_1).wrapping_add(964)) as i32)) & (128i32))) != (0i32)) as i32); let a7_ = c.m.r32((p_w_1).wrapping_add(8)); F::EftQuad_DrawSprite(c, fp, fp.wrapping_add(80), fp.wrapping_add(96), (l_q_3).wrapping_add(64), p_layer_2, a5_, a6_, a7_) };
                                    }
                                    { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
                                }
                            }
                        }
                    }
                    l_q_3 = c.m.r32((l_q_3).wrapping_add(364));
                }
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftQuad_DrawAllSprite", &PROBE_N, &[], true); }
    r_
}

pub fn EftQuad_Emit(c: &mut Ctx, mut p_w_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftQuad_Emit", &PROBE_N, &[(p_w_1) as u32], false); }
    let mut l_i_2: u8 = 0;
    let mut l_q_3: u32 = 0;
    let r_ = 'f: {
    if ((((({ let a_ = (c.m.r32(sym::gEftQuadMgr)).wrapping_add(76800); (c.m.r32(a_) as i32) }) >= (200i32)) as i32)) != 0) {
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gEftQuadMgr)).wrapping_add(76800); c.m.w32(a_, (v_) as u32); };
    }
    l_i_2 = (({ let a_ = (c.m.r32(sym::gEftQuadMgr)).wrapping_add(76800); (c.m.r32(a_) as i32) }) as u8);
    'l1: loop {
        'c2: {
            l_q_3 = (c.m.r32(sym::gEftQuadMgr)).wrapping_add((((l_i_2) as u32)).wrapping_mul(384));
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            if (((((((l_i_2) as i32)) >= (200i32)) as i32)) != 0) {
                l_i_2 = ((0i32) as u8);
            }
            if ((((((((((c.m.r32((l_q_3).wrapping_add(356)) as i32)) == (0i32)) as i32)) != 0) && ((F::EftQuad_InitQuad(c, l_q_3, p_w_1)) != 0)) as i32)) != 0) {
                { let r_ = 1i32; let a_ = (l_q_3).wrapping_add(356); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                F::EftQuad_ListAppend(c, (p_w_1).wrapping_add(968), (p_w_1).wrapping_add(972), l_q_3);
                { let v_ = ((l_i_2) as i32); let a_ = (c.m.r32(sym::gEftQuadMgr)).wrapping_add(76800); c.m.w32(a_, (v_) as u32); };
                break 'f 1i32;
            }
        }
        if !(((((((l_i_2) as i32)) != ({ let a_ = (c.m.r32(sym::gEftQuadMgr)).wrapping_add(76800); (c.m.r32(a_) as i32) })) as i32)) != 0) { break 'l1; }
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftQuad_Emit", &PROBE_N, &[], true); }
    r_
}

pub fn EftQuad_ListAppend(c: &mut Ctx, mut p_head_1: u32, mut p_tail_2: u32, mut p_q_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftQuad_ListAppend", &PROBE_N, &[(p_head_1) as u32, (p_tail_2) as u32, (p_q_3) as u32], false); }
    let r_ = 'f: {
    if (((((c.m.r32(p_head_1)) == (((0i32) as u32))) as i32)) != 0) {
        { let v_ = p_q_3; let a_ = p_head_1; c.m.w32(a_, v_); };
        { let v_ = p_q_3; let a_ = p_tail_2; c.m.w32(a_, v_); };
    } else {
        { let v_ = c.m.r32(p_tail_2); let a_ = (p_q_3).wrapping_add(368); c.m.w32(a_, v_); };
        { let v_ = p_q_3; let a_ = (c.m.r32(p_tail_2)).wrapping_add(364); c.m.w32(a_, v_); };
        { let v_ = p_q_3; let a_ = p_tail_2; c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftQuad_ListAppend", &PROBE_N, &[], true); }
    r_
}

pub fn EftQuad_ListRemove(c: &mut Ctx, mut p_head_1: u32, mut p_tail_2: u32, mut p_p_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftQuad_ListRemove", &PROBE_N, &[(p_head_1) as u32, (p_tail_2) as u32, (p_p_3) as u32], false); }
    let mut l_prev_4: u32 = 0;
    let mut l_a_5: u32 = 0;
    let mut l_b_6: u32 = 0;
    let r_ = 'f: {
    l_a_5 = c.m.r32(p_head_1);
    if (((((l_a_5) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    l_prev_4 = c.m.r32((p_p_3).wrapping_add(368));
    if (((((l_prev_4) == (((0i32) as u32))) as i32)) != 0) {
        p_p_3 = c.m.r32((p_p_3).wrapping_add(364));
        if (((((p_p_3) == (((0i32) as u32))) as i32)) != 0) {
            { let v_ = ((0i32) as u32); let a_ = p_head_1; c.m.w32(a_, v_); };
            { let v_ = ((0i32) as u32); let a_ = p_tail_2; c.m.w32(a_, v_); };
        } else {
            { let v_ = p_p_3; let a_ = p_head_1; c.m.w32(a_, v_); };
            { let v_ = ((0i32) as u32); let a_ = (p_p_3).wrapping_add(368); c.m.w32(a_, v_); };
        }
    } else {
        l_a_5 = c.m.r32((p_p_3).wrapping_add(364));
        if (((((l_a_5) == (((0i32) as u32))) as i32)) != 0) {
            { let v_ = l_prev_4; let a_ = p_tail_2; c.m.w32(a_, v_); };
            { let v_ = ((0i32) as u32); let a_ = (l_prev_4).wrapping_add(364); c.m.w32(a_, v_); };
        } else {
            l_b_6 = l_a_5;
            l_a_5 = l_prev_4;
            { let v_ = l_a_5; let a_ = (l_b_6).wrapping_add(368); c.m.w32(a_, v_); };
            { let v_ = l_b_6; let a_ = (l_a_5).wrapping_add(364); c.m.w32(a_, v_); };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftQuad_ListRemove", &PROBE_N, &[], true); }
    r_
}

pub fn EftQuad_CalcKeyDeltas(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftQuad_CalcKeyDeltas", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let mut l_def_3: u32 = 0;
    let mut l_def2_4: u32 = 0;
    let mut l_k_5: u8 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    l_def_3 = c.m.r32(l_w_2);
    l_def2_4 = c.m.r32((l_w_2).wrapping_add(4));
    l_k_5 = ((0i32) as u8);
    if (((rt::fle(F32(c.m.r32((l_w_2).wrapping_add(100))), F32(0x00000000)) as i32)) != 0) {
        l_k_5 = ((1i32) as u8);
        { let v_ = rt::fmul(F32(c.m.r32((l_def_3).wrapping_add(528))), F32(0x41f00000)); let a_ = (l_w_2).wrapping_add(660); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(F32(c.m.r32((l_w_2).wrapping_add(660))), F32(c.m.r32((l_def_3).wrapping_add(532)))); let a_ = (l_w_2).wrapping_add(664); c.m.w32(a_, (v_).0); };
    }
    if (((((((rt::fle(F32(c.m.r32((l_w_2).wrapping_add(664))), F32(c.m.r32((l_w_2).wrapping_add(100)))) as i32)) != 0) && ((((!(((((c.m.r32((l_w_2).wrapping_add(964)) as i32)) & (16384i32))) != 0)) as i32)) != 0)) as i32)) != 0) {
        { let r_ = 16384i32; let a_ = (l_w_2).wrapping_add(964); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        l_k_5 = ((2i32) as u8);
    }
    if ((((((((((((l_k_5) as i32)).wrapping_sub(1i32)) as u8)) as i32)) < (127i32)) as i32)) != 0) {
        { let v_ = rt::fsub(F32(c.m.r32(((l_def_3).wrapping_add((((l_k_5) as u32)).wrapping_mul(12))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_3).wrapping_add(((((((l_k_5) as i32)).wrapping_sub(1i32)) as u32)).wrapping_mul(12))).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a_ = ((l_w_2).wrapping_add(376)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_def_3).wrapping_add((((l_k_5) as u32)).wrapping_mul(12))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_3).wrapping_add(((((((l_k_5) as i32)).wrapping_sub(1i32)) as u32)).wrapping_mul(12))).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); let a_ = ((l_w_2).wrapping_add(376)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_def_3).wrapping_add((((l_k_5) as u32)).wrapping_mul(12))).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_3).wrapping_add(((((((l_k_5) as i32)).wrapping_sub(1i32)) as u32)).wrapping_mul(12))).wrapping_add((((2i32) as u32)).wrapping_mul(4))))); let a_ = ((l_w_2).wrapping_add(376)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32((((l_def_3).wrapping_add(36)).wrapping_add((((l_k_5) as u32)).wrapping_mul(12))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((((l_def_3).wrapping_add(36)).wrapping_add(((((((l_k_5) as i32)).wrapping_sub(1i32)) as u32)).wrapping_mul(12))).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a_ = (((l_w_2).wrapping_add(376)).wrapping_add(12)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32((((l_def_3).wrapping_add(36)).wrapping_add((((l_k_5) as u32)).wrapping_mul(12))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((((l_def_3).wrapping_add(36)).wrapping_add(((((((l_k_5) as i32)).wrapping_sub(1i32)) as u32)).wrapping_mul(12))).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); let a_ = (((l_w_2).wrapping_add(376)).wrapping_add(12)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32((((l_def_3).wrapping_add(36)).wrapping_add((((l_k_5) as u32)).wrapping_mul(12))).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((((l_def_3).wrapping_add(36)).wrapping_add(((((((l_k_5) as i32)).wrapping_sub(1i32)) as u32)).wrapping_mul(12))).wrapping_add((((2i32) as u32)).wrapping_mul(4))))); let a_ = (((l_w_2).wrapping_add(376)).wrapping_add(12)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32((((l_def_3).wrapping_add(72)).wrapping_add((((l_k_5) as u32)).wrapping_mul(12))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((((l_def_3).wrapping_add(72)).wrapping_add(((((((l_k_5) as i32)).wrapping_sub(1i32)) as u32)).wrapping_mul(12))).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a_ = (((l_w_2).wrapping_add(376)).wrapping_add(24)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32((((l_def_3).wrapping_add(72)).wrapping_add((((l_k_5) as u32)).wrapping_mul(12))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((((l_def_3).wrapping_add(72)).wrapping_add(((((((l_k_5) as i32)).wrapping_sub(1i32)) as u32)).wrapping_mul(12))).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); let a_ = (((l_w_2).wrapping_add(376)).wrapping_add(24)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32((((l_def_3).wrapping_add(72)).wrapping_add((((l_k_5) as u32)).wrapping_mul(12))).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((((l_def_3).wrapping_add(72)).wrapping_add(((((((l_k_5) as i32)).wrapping_sub(1i32)) as u32)).wrapping_mul(12))).wrapping_add((((2i32) as u32)).wrapping_mul(4))))); let a_ = (((l_w_2).wrapping_add(376)).wrapping_add(24)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32((((l_def_3).wrapping_add(108)).wrapping_add((((l_k_5) as u32)).wrapping_mul(12))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((((l_def_3).wrapping_add(108)).wrapping_add(((((((l_k_5) as i32)).wrapping_sub(1i32)) as u32)).wrapping_mul(12))).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a_ = (((l_w_2).wrapping_add(376)).wrapping_add(36)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32((((l_def_3).wrapping_add(108)).wrapping_add((((l_k_5) as u32)).wrapping_mul(12))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((((l_def_3).wrapping_add(108)).wrapping_add(((((((l_k_5) as i32)).wrapping_sub(1i32)) as u32)).wrapping_mul(12))).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); let a_ = (((l_w_2).wrapping_add(376)).wrapping_add(36)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32((((l_def_3).wrapping_add(108)).wrapping_add((((l_k_5) as u32)).wrapping_mul(12))).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((((l_def_3).wrapping_add(108)).wrapping_add(((((((l_k_5) as i32)).wrapping_sub(1i32)) as u32)).wrapping_mul(12))).wrapping_add((((2i32) as u32)).wrapping_mul(4))))); let a_ = (((l_w_2).wrapping_add(376)).wrapping_add(36)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_def_3).wrapping_add(396)).wrapping_add((((l_k_5) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_3).wrapping_add(396)).wrapping_add(((((((l_k_5) as i32)).wrapping_sub(1i32)) as u32)).wrapping_mul(4))))); let a_ = ((l_w_2).wrapping_add(376)).wrapping_add(48); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_def_3).wrapping_add(408)).wrapping_add((((l_k_5) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_3).wrapping_add(408)).wrapping_add(((((((l_k_5) as i32)).wrapping_sub(1i32)) as u32)).wrapping_mul(4))))); let a_ = ((l_w_2).wrapping_add(376)).wrapping_add(52); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_def_3).wrapping_add(420)).wrapping_add((((l_k_5) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_3).wrapping_add(420)).wrapping_add(((((((l_k_5) as i32)).wrapping_sub(1i32)) as u32)).wrapping_mul(4))))); let a_ = ((l_w_2).wrapping_add(376)).wrapping_add(56); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_def_3).wrapping_add(444)).wrapping_add((((l_k_5) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_3).wrapping_add(444)).wrapping_add(((((((l_k_5) as i32)).wrapping_sub(1i32)) as u32)).wrapping_mul(4))))); let a_ = ((l_w_2).wrapping_add(376)).wrapping_add(64); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_def_3).wrapping_add(468)).wrapping_add((((l_k_5) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_3).wrapping_add(468)).wrapping_add(((((((l_k_5) as i32)).wrapping_sub(1i32)) as u32)).wrapping_mul(4))))); let a_ = ((l_w_2).wrapping_add(376)).wrapping_add(72); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_def_3).wrapping_add(432)).wrapping_add((((l_k_5) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_3).wrapping_add(432)).wrapping_add(((((((l_k_5) as i32)).wrapping_sub(1i32)) as u32)).wrapping_mul(4))))); let a_ = ((l_w_2).wrapping_add(376)).wrapping_add(60); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_def_3).wrapping_add(456)).wrapping_add((((l_k_5) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_3).wrapping_add(456)).wrapping_add(((((((l_k_5) as i32)).wrapping_sub(1i32)) as u32)).wrapping_mul(4))))); let a_ = ((l_w_2).wrapping_add(376)).wrapping_add(68); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_def_3).wrapping_add(480)).wrapping_add((((l_k_5) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_3).wrapping_add(480)).wrapping_add(((((((l_k_5) as i32)).wrapping_sub(1i32)) as u32)).wrapping_mul(4))))); let a_ = ((l_w_2).wrapping_add(376)).wrapping_add(76); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_def_3).wrapping_add(492)).wrapping_add((((l_k_5) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_3).wrapping_add(492)).wrapping_add(((((((l_k_5) as i32)).wrapping_sub(1i32)) as u32)).wrapping_mul(4))))); let a_ = ((l_w_2).wrapping_add(376)).wrapping_add(80); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_def_3).wrapping_add(504)).wrapping_add((((l_k_5) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_3).wrapping_add(504)).wrapping_add(((((((l_k_5) as i32)).wrapping_sub(1i32)) as u32)).wrapping_mul(4))))); let a_ = ((l_w_2).wrapping_add(376)).wrapping_add(84); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_def_3).wrapping_add(516)).wrapping_add((((l_k_5) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_3).wrapping_add(516)).wrapping_add(((((((l_k_5) as i32)).wrapping_sub(1i32)) as u32)).wrapping_mul(4))))); let a_ = ((l_w_2).wrapping_add(376)).wrapping_add(88); c.m.w32(a_, (v_).0); };
        { let v_ = rt::i2f((((c.m.r8(((l_def_3).wrapping_add(568)).wrapping_add(((l_k_5) as u32)))) as i32)).wrapping_sub(((c.m.r8(((l_def_3).wrapping_add(568)).wrapping_add((((((l_k_5) as i32)).wrapping_sub(1i32)) as u32)))) as i32))); let a_ = ((l_w_2).wrapping_add(376)).wrapping_add(92); c.m.w32(a_, (v_).0); };
        { let v_ = rt::i2f((((c.m.r8(((l_def_3).wrapping_add(571)).wrapping_add(((l_k_5) as u32)))) as i32)).wrapping_sub(((c.m.r8(((l_def_3).wrapping_add(571)).wrapping_add((((((l_k_5) as i32)).wrapping_sub(1i32)) as u32)))) as i32))); let a_ = ((l_w_2).wrapping_add(376)).wrapping_add(96); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32((((l_def_3).wrapping_add(144)).wrapping_add((((l_k_5) as u32)).wrapping_mul(12))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((((l_def_3).wrapping_add(144)).wrapping_add(((((((l_k_5) as i32)).wrapping_sub(1i32)) as u32)).wrapping_mul(12))).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a_ = (((l_w_2).wrapping_add(376)).wrapping_add(100)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32((((l_def_3).wrapping_add(144)).wrapping_add((((l_k_5) as u32)).wrapping_mul(12))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((((l_def_3).wrapping_add(144)).wrapping_add(((((((l_k_5) as i32)).wrapping_sub(1i32)) as u32)).wrapping_mul(12))).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); let a_ = (((l_w_2).wrapping_add(376)).wrapping_add(100)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32((((l_def_3).wrapping_add(144)).wrapping_add((((l_k_5) as u32)).wrapping_mul(12))).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((((l_def_3).wrapping_add(144)).wrapping_add(((((((l_k_5) as i32)).wrapping_sub(1i32)) as u32)).wrapping_mul(12))).wrapping_add((((2i32) as u32)).wrapping_mul(4))))); let a_ = (((l_w_2).wrapping_add(376)).wrapping_add(100)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32((((l_def_3).wrapping_add(180)).wrapping_add((((l_k_5) as u32)).wrapping_mul(12))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((((l_def_3).wrapping_add(180)).wrapping_add(((((((l_k_5) as i32)).wrapping_sub(1i32)) as u32)).wrapping_mul(12))).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a_ = (((l_w_2).wrapping_add(376)).wrapping_add(112)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32((((l_def_3).wrapping_add(180)).wrapping_add((((l_k_5) as u32)).wrapping_mul(12))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((((l_def_3).wrapping_add(180)).wrapping_add(((((((l_k_5) as i32)).wrapping_sub(1i32)) as u32)).wrapping_mul(12))).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); let a_ = (((l_w_2).wrapping_add(376)).wrapping_add(112)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32((((l_def_3).wrapping_add(180)).wrapping_add((((l_k_5) as u32)).wrapping_mul(12))).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((((l_def_3).wrapping_add(180)).wrapping_add(((((((l_k_5) as i32)).wrapping_sub(1i32)) as u32)).wrapping_mul(12))).wrapping_add((((2i32) as u32)).wrapping_mul(4))))); let a_ = (((l_w_2).wrapping_add(376)).wrapping_add(112)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32((((l_def_3).wrapping_add(216)).wrapping_add((((l_k_5) as u32)).wrapping_mul(12))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((((l_def_3).wrapping_add(216)).wrapping_add(((((((l_k_5) as i32)).wrapping_sub(1i32)) as u32)).wrapping_mul(12))).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a_ = (((l_w_2).wrapping_add(376)).wrapping_add(124)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32((((l_def_3).wrapping_add(216)).wrapping_add((((l_k_5) as u32)).wrapping_mul(12))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((((l_def_3).wrapping_add(216)).wrapping_add(((((((l_k_5) as i32)).wrapping_sub(1i32)) as u32)).wrapping_mul(12))).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); let a_ = (((l_w_2).wrapping_add(376)).wrapping_add(124)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32((((l_def_3).wrapping_add(216)).wrapping_add((((l_k_5) as u32)).wrapping_mul(12))).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((((l_def_3).wrapping_add(216)).wrapping_add(((((((l_k_5) as i32)).wrapping_sub(1i32)) as u32)).wrapping_mul(12))).wrapping_add((((2i32) as u32)).wrapping_mul(4))))); let a_ = (((l_w_2).wrapping_add(376)).wrapping_add(124)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32((((l_def_3).wrapping_add(288)).wrapping_add((((l_k_5) as u32)).wrapping_mul(12))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((((l_def_3).wrapping_add(288)).wrapping_add(((((((l_k_5) as i32)).wrapping_sub(1i32)) as u32)).wrapping_mul(12))).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a_ = (((l_w_2).wrapping_add(376)).wrapping_add(136)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32((((l_def_3).wrapping_add(288)).wrapping_add((((l_k_5) as u32)).wrapping_mul(12))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((((l_def_3).wrapping_add(288)).wrapping_add(((((((l_k_5) as i32)).wrapping_sub(1i32)) as u32)).wrapping_mul(12))).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); let a_ = (((l_w_2).wrapping_add(376)).wrapping_add(136)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32((((l_def_3).wrapping_add(288)).wrapping_add((((l_k_5) as u32)).wrapping_mul(12))).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((((l_def_3).wrapping_add(288)).wrapping_add(((((((l_k_5) as i32)).wrapping_sub(1i32)) as u32)).wrapping_mul(12))).wrapping_add((((2i32) as u32)).wrapping_mul(4))))); let a_ = (((l_w_2).wrapping_add(376)).wrapping_add(136)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32((((l_def_3).wrapping_add(324)).wrapping_add((((l_k_5) as u32)).wrapping_mul(12))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((((l_def_3).wrapping_add(324)).wrapping_add(((((((l_k_5) as i32)).wrapping_sub(1i32)) as u32)).wrapping_mul(12))).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a_ = (((l_w_2).wrapping_add(376)).wrapping_add(148)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32((((l_def_3).wrapping_add(324)).wrapping_add((((l_k_5) as u32)).wrapping_mul(12))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((((l_def_3).wrapping_add(324)).wrapping_add(((((((l_k_5) as i32)).wrapping_sub(1i32)) as u32)).wrapping_mul(12))).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); let a_ = (((l_w_2).wrapping_add(376)).wrapping_add(148)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32((((l_def_3).wrapping_add(324)).wrapping_add((((l_k_5) as u32)).wrapping_mul(12))).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((((l_def_3).wrapping_add(324)).wrapping_add(((((((l_k_5) as i32)).wrapping_sub(1i32)) as u32)).wrapping_mul(12))).wrapping_add((((2i32) as u32)).wrapping_mul(4))))); let a_ = (((l_w_2).wrapping_add(376)).wrapping_add(148)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32((((l_def_3).wrapping_add(360)).wrapping_add((((l_k_5) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((((l_def_3).wrapping_add(360)).wrapping_add(((((((l_k_5) as i32)).wrapping_sub(1i32)) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a_ = (((l_w_2).wrapping_add(376)).wrapping_add(160)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32((((l_def_3).wrapping_add(360)).wrapping_add((((l_k_5) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((((l_def_3).wrapping_add(360)).wrapping_add(((((((l_k_5) as i32)).wrapping_sub(1i32)) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); let a_ = (((l_w_2).wrapping_add(376)).wrapping_add(160)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_def_3).wrapping_add(384)).wrapping_add((((l_k_5) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def_3).wrapping_add(384)).wrapping_add(((((((l_k_5) as i32)).wrapping_sub(1i32)) as u32)).wrapping_mul(4))))); let a_ = ((l_w_2).wrapping_add(376)).wrapping_add(168); c.m.w32(a_, (v_).0); };
        F::Vec4_Sub(c, ((l_w_2).wrapping_add(376)).wrapping_add(184), (l_def2_4).wrapping_add((((l_k_5) as u32)).wrapping_mul(16)), (l_def2_4).wrapping_add(((((((l_k_5) as i32)).wrapping_sub(1i32)) as u32)).wrapping_mul(16)));
        F::Vec4_Sub(c, ((l_w_2).wrapping_add(376)).wrapping_add(200), ((l_def2_4).wrapping_add(48)).wrapping_add((((l_k_5) as u32)).wrapping_mul(16)), ((l_def2_4).wrapping_add(48)).wrapping_add(((((((l_k_5) as i32)).wrapping_sub(1i32)) as u32)).wrapping_mul(16)));
        F::Vec4_Sub(c, ((l_w_2).wrapping_add(376)).wrapping_add(216), ((l_def2_4).wrapping_add(96)).wrapping_add((((l_k_5) as u32)).wrapping_mul(16)), ((l_def2_4).wrapping_add(96)).wrapping_add(((((((l_k_5) as i32)).wrapping_sub(1i32)) as u32)).wrapping_mul(16)));
        F::Vec4_Sub(c, ((l_w_2).wrapping_add(376)).wrapping_add(232), ((l_def2_4).wrapping_add(144)).wrapping_add((((l_k_5) as u32)).wrapping_mul(16)), ((l_def2_4).wrapping_add(144)).wrapping_add(((((((l_k_5) as i32)).wrapping_sub(1i32)) as u32)).wrapping_mul(16)));
        { let v_ = rt::fsub(F32(c.m.r32(((l_def2_4).wrapping_add(192)).wrapping_add((((l_k_5) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def2_4).wrapping_add(192)).wrapping_add(((((((l_k_5) as i32)).wrapping_sub(1i32)) as u32)).wrapping_mul(4))))); let a_ = ((l_w_2).wrapping_add(376)).wrapping_add(248); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_def2_4).wrapping_add(204)).wrapping_add((((l_k_5) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def2_4).wrapping_add(204)).wrapping_add(((((((l_k_5) as i32)).wrapping_sub(1i32)) as u32)).wrapping_mul(4))))); let a_ = ((l_w_2).wrapping_add(376)).wrapping_add(252); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32((((l_def2_4).wrapping_add(216)).wrapping_add((((l_k_5) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((((l_def2_4).wrapping_add(216)).wrapping_add(((((((l_k_5) as i32)).wrapping_sub(1i32)) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a_ = ((l_w_2).wrapping_add(376)).wrapping_add(256); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32((((l_def2_4).wrapping_add(240)).wrapping_add((((l_k_5) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((((l_def2_4).wrapping_add(240)).wrapping_add(((((((l_k_5) as i32)).wrapping_sub(1i32)) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a_ = ((l_w_2).wrapping_add(376)).wrapping_add(260); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32((((l_def2_4).wrapping_add(264)).wrapping_add((((l_k_5) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((((l_def2_4).wrapping_add(264)).wrapping_add(((((((l_k_5) as i32)).wrapping_sub(1i32)) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a_ = ((l_w_2).wrapping_add(376)).wrapping_add(264); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32((((l_def2_4).wrapping_add(216)).wrapping_add((((l_k_5) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((((l_def2_4).wrapping_add(216)).wrapping_add(((((((l_k_5) as i32)).wrapping_sub(1i32)) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); let a_ = ((l_w_2).wrapping_add(376)).wrapping_add(268); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32((((l_def2_4).wrapping_add(240)).wrapping_add((((l_k_5) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((((l_def2_4).wrapping_add(240)).wrapping_add(((((((l_k_5) as i32)).wrapping_sub(1i32)) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); let a_ = ((l_w_2).wrapping_add(376)).wrapping_add(272); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32((((l_def2_4).wrapping_add(264)).wrapping_add((((l_k_5) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((((l_def2_4).wrapping_add(264)).wrapping_add(((((((l_k_5) as i32)).wrapping_sub(1i32)) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); let a_ = ((l_w_2).wrapping_add(376)).wrapping_add(276); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_def2_4).wrapping_add(288)).wrapping_add((((l_k_5) as u32)).wrapping_mul(4)))), F32(c.m.r32(((l_def2_4).wrapping_add(288)).wrapping_add(((((((l_k_5) as i32)).wrapping_sub(1i32)) as u32)).wrapping_mul(4))))); let a_ = ((l_w_2).wrapping_add(376)).wrapping_add(280); c.m.w32(a_, (v_).0); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftQuad_CalcKeyDeltas", &PROBE_N, &[], true); }
    r_
}

pub fn EftQuad_Animate(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftQuad_Animate", &PROBE_N, &[(p_task_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_k_2: i32 = 0;
    let mut l_w_3: u32 = 0;
    let mut l_def_4: u32 = 0;
    let mut l_def2_5: u32 = 0;
    let mut l_t_6: F32 = F32(0);
    let r_ = 'f: {
        l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
        l_def_4 = c.m.r32(l_w_3);
        l_k_2 = 0i32;
        l_def2_5 = c.m.r32((l_w_3).wrapping_add(4));
        { let _ = F::memset(c, fp, 0i32, 16u32); };
        if ((((!(((((c.m.r32((l_w_3).wrapping_add(964)) as i32)) & (16384i32))) != 0)) as i32)) != 0) {
            l_t_6 = rt::fdiv(F32(c.m.r32((l_w_3).wrapping_add(100))), F32(c.m.r32((l_w_3).wrapping_add(664))));
        } else {
            l_k_2 = 1i32;
            l_t_6 = rt::fdiv(rt::fsub(F32(c.m.r32((l_w_3).wrapping_add(100))), F32(c.m.r32((l_w_3).wrapping_add(664)))), rt::fsub(F32(c.m.r32((l_w_3).wrapping_add(660))), F32(c.m.r32((l_w_3).wrapping_add(664)))));
        }
        { let v_ = rt::fadd(F32(c.m.r32(((l_def_4).wrapping_add((((l_k_2) as u32)).wrapping_mul(12))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(((l_w_3).wrapping_add(376)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), l_t_6)); let a_ = ((l_w_3).wrapping_add(668)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_def_4).wrapping_add((((l_k_2) as u32)).wrapping_mul(12))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(((l_w_3).wrapping_add(376)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), l_t_6)); let a_ = ((l_w_3).wrapping_add(668)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_def_4).wrapping_add((((l_k_2) as u32)).wrapping_mul(12))).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(((l_w_3).wrapping_add(376)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), l_t_6)); let a_ = ((l_w_3).wrapping_add(668)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32((((l_def_4).wrapping_add(36)).wrapping_add((((l_k_2) as u32)).wrapping_mul(12))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((((l_w_3).wrapping_add(376)).wrapping_add(12)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), l_t_6)); let a_ = (((l_w_3).wrapping_add(668)).wrapping_add(12)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32((((l_def_4).wrapping_add(36)).wrapping_add((((l_k_2) as u32)).wrapping_mul(12))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((((l_w_3).wrapping_add(376)).wrapping_add(12)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), l_t_6)); let a_ = (((l_w_3).wrapping_add(668)).wrapping_add(12)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32((((l_def_4).wrapping_add(36)).wrapping_add((((l_k_2) as u32)).wrapping_mul(12))).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((((l_w_3).wrapping_add(376)).wrapping_add(12)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), l_t_6)); let a_ = (((l_w_3).wrapping_add(668)).wrapping_add(12)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32((((l_def_4).wrapping_add(72)).wrapping_add((((l_k_2) as u32)).wrapping_mul(12))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((((l_w_3).wrapping_add(376)).wrapping_add(24)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), l_t_6)); let a_ = (((l_w_3).wrapping_add(668)).wrapping_add(24)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32((((l_def_4).wrapping_add(72)).wrapping_add((((l_k_2) as u32)).wrapping_mul(12))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((((l_w_3).wrapping_add(376)).wrapping_add(24)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), l_t_6)); let a_ = (((l_w_3).wrapping_add(668)).wrapping_add(24)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32((((l_def_4).wrapping_add(72)).wrapping_add((((l_k_2) as u32)).wrapping_mul(12))).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((((l_w_3).wrapping_add(376)).wrapping_add(24)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), l_t_6)); let a_ = (((l_w_3).wrapping_add(668)).wrapping_add(24)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32((((l_def_4).wrapping_add(108)).wrapping_add((((l_k_2) as u32)).wrapping_mul(12))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((((l_w_3).wrapping_add(376)).wrapping_add(36)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), l_t_6)); let a_ = (((l_w_3).wrapping_add(668)).wrapping_add(36)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32((((l_def_4).wrapping_add(108)).wrapping_add((((l_k_2) as u32)).wrapping_mul(12))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((((l_w_3).wrapping_add(376)).wrapping_add(36)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), l_t_6)); let a_ = (((l_w_3).wrapping_add(668)).wrapping_add(36)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32((((l_def_4).wrapping_add(108)).wrapping_add((((l_k_2) as u32)).wrapping_mul(12))).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((((l_w_3).wrapping_add(376)).wrapping_add(36)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), l_t_6)); let a_ = (((l_w_3).wrapping_add(668)).wrapping_add(36)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_def_4).wrapping_add(420)).wrapping_add((((l_k_2) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(((l_w_3).wrapping_add(376)).wrapping_add(56))), l_t_6)); let a_ = ((l_w_3).wrapping_add(668)).wrapping_add(56); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_def_4).wrapping_add(444)).wrapping_add((((l_k_2) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(((l_w_3).wrapping_add(376)).wrapping_add(64))), l_t_6)); let a_ = ((l_w_3).wrapping_add(668)).wrapping_add(64); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_def_4).wrapping_add(468)).wrapping_add((((l_k_2) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(((l_w_3).wrapping_add(376)).wrapping_add(72))), l_t_6)); let a_ = ((l_w_3).wrapping_add(668)).wrapping_add(72); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_def_4).wrapping_add(432)).wrapping_add((((l_k_2) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(((l_w_3).wrapping_add(376)).wrapping_add(60))), l_t_6)); let a_ = ((l_w_3).wrapping_add(668)).wrapping_add(60); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_def_4).wrapping_add(456)).wrapping_add((((l_k_2) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(((l_w_3).wrapping_add(376)).wrapping_add(68))), l_t_6)); let a_ = ((l_w_3).wrapping_add(668)).wrapping_add(68); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_def_4).wrapping_add(480)).wrapping_add((((l_k_2) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(((l_w_3).wrapping_add(376)).wrapping_add(76))), l_t_6)); let a_ = ((l_w_3).wrapping_add(668)).wrapping_add(76); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32((((l_def_4).wrapping_add(144)).wrapping_add((((l_k_2) as u32)).wrapping_mul(12))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((((l_w_3).wrapping_add(376)).wrapping_add(100)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), l_t_6)); let a_ = (((l_w_3).wrapping_add(668)).wrapping_add(92)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32((((l_def_4).wrapping_add(144)).wrapping_add((((l_k_2) as u32)).wrapping_mul(12))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((((l_w_3).wrapping_add(376)).wrapping_add(100)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), l_t_6)); let a_ = (((l_w_3).wrapping_add(668)).wrapping_add(92)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32((((l_def_4).wrapping_add(144)).wrapping_add((((l_k_2) as u32)).wrapping_mul(12))).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((((l_w_3).wrapping_add(376)).wrapping_add(100)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), l_t_6)); let a_ = (((l_w_3).wrapping_add(668)).wrapping_add(92)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32((((l_def_4).wrapping_add(180)).wrapping_add((((l_k_2) as u32)).wrapping_mul(12))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((((l_w_3).wrapping_add(376)).wrapping_add(112)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), l_t_6)); let a_ = (((l_w_3).wrapping_add(668)).wrapping_add(104)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32((((l_def_4).wrapping_add(180)).wrapping_add((((l_k_2) as u32)).wrapping_mul(12))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((((l_w_3).wrapping_add(376)).wrapping_add(112)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), l_t_6)); let a_ = (((l_w_3).wrapping_add(668)).wrapping_add(104)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32((((l_def_4).wrapping_add(180)).wrapping_add((((l_k_2) as u32)).wrapping_mul(12))).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((((l_w_3).wrapping_add(376)).wrapping_add(112)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), l_t_6)); let a_ = (((l_w_3).wrapping_add(668)).wrapping_add(104)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32((((l_def_4).wrapping_add(216)).wrapping_add((((l_k_2) as u32)).wrapping_mul(12))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((((l_w_3).wrapping_add(376)).wrapping_add(124)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), l_t_6)); let a_ = (((l_w_3).wrapping_add(668)).wrapping_add(116)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32((((l_def_4).wrapping_add(216)).wrapping_add((((l_k_2) as u32)).wrapping_mul(12))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((((l_w_3).wrapping_add(376)).wrapping_add(124)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), l_t_6)); let a_ = (((l_w_3).wrapping_add(668)).wrapping_add(116)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32((((l_def_4).wrapping_add(216)).wrapping_add((((l_k_2) as u32)).wrapping_mul(12))).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((((l_w_3).wrapping_add(376)).wrapping_add(124)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), l_t_6)); let a_ = (((l_w_3).wrapping_add(668)).wrapping_add(116)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32((((l_def_4).wrapping_add(288)).wrapping_add((((l_k_2) as u32)).wrapping_mul(12))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((((l_w_3).wrapping_add(376)).wrapping_add(136)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), l_t_6)); let a_ = (((l_w_3).wrapping_add(668)).wrapping_add(140)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32((((l_def_4).wrapping_add(288)).wrapping_add((((l_k_2) as u32)).wrapping_mul(12))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((((l_w_3).wrapping_add(376)).wrapping_add(136)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), l_t_6)); let a_ = (((l_w_3).wrapping_add(668)).wrapping_add(140)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32((((l_def_4).wrapping_add(288)).wrapping_add((((l_k_2) as u32)).wrapping_mul(12))).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((((l_w_3).wrapping_add(376)).wrapping_add(136)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), l_t_6)); let a_ = (((l_w_3).wrapping_add(668)).wrapping_add(140)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32((((l_def_4).wrapping_add(324)).wrapping_add((((l_k_2) as u32)).wrapping_mul(12))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((((l_w_3).wrapping_add(376)).wrapping_add(148)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), l_t_6)); let a_ = (((l_w_3).wrapping_add(668)).wrapping_add(152)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32((((l_def_4).wrapping_add(324)).wrapping_add((((l_k_2) as u32)).wrapping_mul(12))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((((l_w_3).wrapping_add(376)).wrapping_add(148)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), l_t_6)); let a_ = (((l_w_3).wrapping_add(668)).wrapping_add(152)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32((((l_def_4).wrapping_add(324)).wrapping_add((((l_k_2) as u32)).wrapping_mul(12))).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((((l_w_3).wrapping_add(376)).wrapping_add(148)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), l_t_6)); let a_ = (((l_w_3).wrapping_add(668)).wrapping_add(152)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32((((l_def_4).wrapping_add(360)).wrapping_add((((l_k_2) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((((l_w_3).wrapping_add(376)).wrapping_add(160)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), l_t_6)); let a_ = (((l_w_3).wrapping_add(668)).wrapping_add(164)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32((((l_def_4).wrapping_add(360)).wrapping_add((((l_k_2) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((((l_w_3).wrapping_add(376)).wrapping_add(160)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), l_t_6)); let a_ = (((l_w_3).wrapping_add(668)).wrapping_add(164)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_def_4).wrapping_add(384)).wrapping_add((((l_k_2) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(((l_w_3).wrapping_add(376)).wrapping_add(168))), l_t_6)); let a_ = ((l_w_3).wrapping_add(668)).wrapping_add(172); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_def_4).wrapping_add(396)).wrapping_add((((l_k_2) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(((l_w_3).wrapping_add(376)).wrapping_add(48))), l_t_6)); let a_ = ((l_w_3).wrapping_add(668)).wrapping_add(48); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_def_4).wrapping_add(408)).wrapping_add((((l_k_2) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(((l_w_3).wrapping_add(376)).wrapping_add(52))), l_t_6)); let a_ = ((l_w_3).wrapping_add(668)).wrapping_add(52); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_def_4).wrapping_add(492)).wrapping_add((((l_k_2) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(((l_w_3).wrapping_add(376)).wrapping_add(80))), l_t_6)); let a_ = ((l_w_3).wrapping_add(668)).wrapping_add(80); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_def_4).wrapping_add(504)).wrapping_add((((l_k_2) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(((l_w_3).wrapping_add(376)).wrapping_add(84))), l_t_6)); let a_ = ((l_w_3).wrapping_add(668)).wrapping_add(84); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(rt::i2f(((c.m.r8(((l_def_4).wrapping_add(571)).wrapping_add(((l_k_2) as u32)))) as i32)), rt::fmul(F32(c.m.r32(((l_w_3).wrapping_add(376)).wrapping_add(96))), l_t_6)); let a_ = ((l_w_3).wrapping_add(668)).wrapping_add(284); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(rt::i2f(((c.m.r8(((l_def_4).wrapping_add(568)).wrapping_add(((l_k_2) as u32)))) as i32)), rt::fmul(F32(c.m.r32(((l_w_3).wrapping_add(376)).wrapping_add(92))), l_t_6)); let a_ = ((l_w_3).wrapping_add(668)).wrapping_add(280); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_def_4).wrapping_add(516)).wrapping_add((((l_k_2) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(((l_w_3).wrapping_add(376)).wrapping_add(88))), l_t_6)); let a_ = ((l_w_3).wrapping_add(668)).wrapping_add(88); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_def2_5).wrapping_add(192)).wrapping_add((((l_k_2) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(((l_w_3).wrapping_add(376)).wrapping_add(248))), l_t_6)); let a_ = ((l_w_3).wrapping_add(668)).wrapping_add(244); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_def2_5).wrapping_add(204)).wrapping_add((((l_k_2) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(((l_w_3).wrapping_add(376)).wrapping_add(252))), l_t_6)); let a_ = ((l_w_3).wrapping_add(668)).wrapping_add(248); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32((((l_def2_5).wrapping_add(216)).wrapping_add((((l_k_2) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(((l_w_3).wrapping_add(376)).wrapping_add(256))), l_t_6)); let a_ = ((l_w_3).wrapping_add(668)).wrapping_add(252); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32((((l_def2_5).wrapping_add(240)).wrapping_add((((l_k_2) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(((l_w_3).wrapping_add(376)).wrapping_add(260))), l_t_6)); let a_ = ((l_w_3).wrapping_add(668)).wrapping_add(256); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32((((l_def2_5).wrapping_add(264)).wrapping_add((((l_k_2) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(((l_w_3).wrapping_add(376)).wrapping_add(264))), l_t_6)); let a_ = ((l_w_3).wrapping_add(668)).wrapping_add(260); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32((((l_def2_5).wrapping_add(216)).wrapping_add((((l_k_2) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(((l_w_3).wrapping_add(376)).wrapping_add(268))), l_t_6)); let a_ = ((l_w_3).wrapping_add(668)).wrapping_add(264); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32((((l_def2_5).wrapping_add(240)).wrapping_add((((l_k_2) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(((l_w_3).wrapping_add(376)).wrapping_add(272))), l_t_6)); let a_ = ((l_w_3).wrapping_add(668)).wrapping_add(268); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32((((l_def2_5).wrapping_add(264)).wrapping_add((((l_k_2) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(((l_w_3).wrapping_add(376)).wrapping_add(276))), l_t_6)); let a_ = ((l_w_3).wrapping_add(668)).wrapping_add(272); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_def2_5).wrapping_add(288)).wrapping_add((((l_k_2) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(((l_w_3).wrapping_add(376)).wrapping_add(280))), l_t_6)); let a_ = ((l_w_3).wrapping_add(668)).wrapping_add(276); c.m.w32(a_, (v_).0); };
        F::Vec4_Scale(c, fp, ((l_w_3).wrapping_add(376)).wrapping_add(184), l_t_6);
        F::Vec4_Add(c, ((l_w_3).wrapping_add(668)).wrapping_add(180), (l_def2_5).wrapping_add((((l_k_2) as u32)).wrapping_mul(16)), fp);
        F::Vec4_Scale(c, fp, ((l_w_3).wrapping_add(376)).wrapping_add(200), l_t_6);
        F::Vec4_Add(c, ((l_w_3).wrapping_add(668)).wrapping_add(196), ((l_def2_5).wrapping_add(48)).wrapping_add((((l_k_2) as u32)).wrapping_mul(16)), fp);
        F::Vec4_Scale(c, fp, ((l_w_3).wrapping_add(376)).wrapping_add(216), l_t_6);
        F::Vec4_Add(c, ((l_w_3).wrapping_add(668)).wrapping_add(212), ((l_def2_5).wrapping_add(96)).wrapping_add((((l_k_2) as u32)).wrapping_mul(16)), fp);
        F::Vec4_Scale(c, fp, ((l_w_3).wrapping_add(376)).wrapping_add(232), l_t_6);
        F::Vec4_Add(c, ((l_w_3).wrapping_add(668)).wrapping_add(228), ((l_def2_5).wrapping_add(144)).wrapping_add((((l_k_2) as u32)).wrapping_mul(16)), fp);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftQuad_Animate", &PROBE_N, &[], true); }
    r_
}

pub fn EftQuad_DrawSprite(c: &mut Ctx, mut p_corner_1: u32, p_uv0_2: u32, p_uv1_3: u32, p_color_4: u32, mut p_layer_5: i32, mut p_texIdx_6: i32, mut p_flip_7: i32, mut p_tex_8: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftQuad_DrawSprite", &PROBE_N, &[(p_corner_1) as u32, (p_uv0_2) as u32, (p_uv1_3) as u32, (p_color_4) as u32, (p_layer_5) as u32, (p_texIdx_6) as u32, (p_flip_7) as u32, (p_tex_8) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(256) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 256);
    let mut l_p_9: u32 = 0;
    let mut l_e_10: u32 = 0;
    let mut l_z_11: i32 = 0;
    let mut l_ctx_12: i32 = 0;
    let mut l_abe_13: i32 = 0;
    let mut l_l_14: i32 = 0;
    rt::memcpy(c, fp, p_uv0_2, 16);
    rt::memcpy(c, fp.wrapping_add(16), p_uv1_3, 16);
    rt::memcpy(c, fp.wrapping_add(32), p_color_4, 16);
    let r_ = 'f: {
        l_abe_13 = 1i32;
        { let a1_ = F32(c.m.r32(fp)); let a2_ = F32(c.m.r32((fp).wrapping_add(4))); F::Vec4_Set(c, (fp.wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), a1_, a2_, F32(0x3f800000), F32(0x3f800000)) };
        { let a1_ = F32(c.m.r32((fp).wrapping_add(8))); let a2_ = F32(c.m.r32((fp).wrapping_add(12))); F::Vec4_Set(c, (fp.wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), a1_, a2_, F32(0x3f800000), F32(0x3f800000)) };
        { let a1_ = F32(c.m.r32(fp.wrapping_add(16))); let a2_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))); F::Vec4_Set(c, (fp.wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(16)), a1_, a2_, F32(0x3f800000), F32(0x3f800000)) };
        { let a1_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8))); let a2_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(12))); F::Vec4_Set(c, (fp.wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(16)), a1_, a2_, F32(0x3f800000), F32(0x3f800000)) };
        if ((F::Vu0Cur_ProjectPointsStq(c, fp.wrapping_add(176), fp.wrapping_add(112), p_corner_1, fp.wrapping_add(48), 4i32)) != 0) {
            l_p_9 = c.m.r32(sym::gOtCur);
            { let v_ = (l_p_9).wrapping_add((((1i32) as u32)).wrapping_mul(144)); let a_ = sym::gOtCur; c.m.w32(a_, v_); };
            if (((((l_p_9) != (((0i32) as u32))) as i32)) != 0) {
                l_ctx_12 = (((p_layer_5) >= (2i32)) as i32);
                { let v_ = (((((((l_abe_13) as u64)).wrapping_shl((6i32) as u32)) | ((((l_ctx_12) as u64)).wrapping_shl((9i32) as u32)))) | (((28i32) as u64))); let a_ = (l_p_9).wrapping_add(32); c.m.w64(a_, v_); };
                { let v_ = ((536870920i32) as u32); let a_ = l_p_9; c.m.w32(a_, v_); };
                { let v_ = ((268435456i32) as u32); let a_ = (l_p_9).wrapping_add(8); c.m.w32(a_, v_); };
                { let v_ = ((1342177288i32) as u32); let a_ = (l_p_9).wrapping_add(12); c.m.w32(a_, v_); };
                { let v_ = 16429131440647602177u64; let a_ = (l_p_9).wrapping_add(16); c.m.w64(a_, v_); };
                { let v_ = (((18599481537995104i64).wrapping_add((((l_ctx_12).wrapping_shl((4i32) as u32)) as i64))) as u64); let a_ = (l_p_9).wrapping_add(24); c.m.w64(a_, v_); };
                { let v_ = ((0i32) as u32); let a_ = (l_p_9).wrapping_add(4); c.m.w32(a_, v_); };
                l_z_11 = (((((c.m.r32(((fp.wrapping_add(176)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32)).wrapping_add((c.m.r32(((fp.wrapping_add(176)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32))).wrapping_add((c.m.r32(((fp.wrapping_add(176)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32))).wrapping_add((c.m.r32(((fp.wrapping_add(176)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(8)) as i32))).wrapping_shr((10i32) as u32);
                if ((p_flip_7) != 0) {
                    { let v_ = { let v_ = { let v_ = { let v_ = 16777215i32; let a_ = ((fp.wrapping_add(176)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_) as u32); v_ }; let a_ = ((fp.wrapping_add(176)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_) as u32); v_ }; let a_ = ((fp.wrapping_add(176)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_) as u32); v_ }; let a_ = ((fp.wrapping_add(176)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
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
                { let v_ = c.m.r64((p_tex_8).wrapping_add((((p_texIdx_6) as u32)).wrapping_mul(16))); let a_ = (l_p_9).wrapping_add(40); c.m.w64(a_, v_); };
                l_l_14 = p_layer_5;
                if (((((l_l_14) >= (2i32)) as i32)) != 0) {
                    { let r_ = 2i32; let n_ = (l_l_14).wrapping_sub(r_); l_l_14 = n_; };
                }
                if (((((l_z_11) < (0i32)) as i32)) != 0) {
                    l_e_10 = ((c.m.r32(sym::gOtZ)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((l_l_14) as u32)).wrapping_mul(8));
                } else {
                    if (((((l_z_11) >= (4096i32)) as i32)) != 0) {
                        l_e_10 = ((c.m.r32(sym::gOtZ)).wrapping_add((((4095i32) as u32)).wrapping_mul(16))).wrapping_add((((l_l_14) as u32)).wrapping_mul(8));
                    } else {
                        l_e_10 = ((c.m.r32(sym::gOtZ)).wrapping_add((((l_z_11) as u32)).wrapping_mul(16))).wrapping_add((((l_l_14) as u32)).wrapping_mul(8));
                    }
                }
                { let v_ = l_p_9; let a_ = (c.m.r32((l_e_10).wrapping_add(4))).wrapping_add(4); c.m.w32(a_, v_); };
                { let v_ = l_p_9; let a_ = (l_e_10).wrapping_add(4); c.m.w32(a_, v_); };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftQuad_DrawSprite", &PROBE_N, &[], true); }
    r_
}

pub fn EftQuad_DrawFacing(c: &mut Ctx, mut p_corner_1: u32, p_uv0_2: u32, p_uv1_3: u32, p_color_4: u32, mut p_layer_5: i32, mut p_texIdx_6: i32, mut p_arg6_7: i32, mut p_flip_8: i32, mut p_tex_9: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftQuad_DrawFacing", &PROBE_N, &[(p_corner_1) as u32, (p_uv0_2) as u32, (p_uv1_3) as u32, (p_color_4) as u32, (p_layer_5) as u32, (p_texIdx_6) as u32, (p_arg6_7) as u32, (p_flip_8) as u32, (p_tex_9) as u32], false); }
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
        { let a6_ = c.m.r64((p_tex_9).wrapping_add((((p_texIdx_6) as u32)).wrapping_mul(16))); F::EftGfx_DrawPolyScaledZ(c, fp.wrapping_add(176), p_layer_5, 1i32, 0i32, p_flip_8, 0i32, a6_, F32(0x40000000)) };
        F::ClipVtx_Set(c, (fp.wrapping_add(176)).wrapping_add((((0i32) as u32)).wrapping_mul(48)), (fp.wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(112)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), fp.wrapping_add(32));
        F::ClipVtx_Set(c, (fp.wrapping_add(176)).wrapping_add((((1i32) as u32)).wrapping_mul(48)), (fp.wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(112)).wrapping_add((((2i32) as u32)).wrapping_mul(16)), fp.wrapping_add(32));
        F::ClipVtx_Set(c, (fp.wrapping_add(176)).wrapping_add((((2i32) as u32)).wrapping_mul(48)), (fp.wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(112)).wrapping_add((((3i32) as u32)).wrapping_mul(16)), fp.wrapping_add(32));
        { let a6_ = c.m.r64((p_tex_9).wrapping_add((((p_texIdx_6) as u32)).wrapping_mul(16))); F::EftGfx_DrawPolyScaledZ(c, fp.wrapping_add(176), p_layer_5, 1i32, 0i32, p_flip_8, 0i32, a6_, F32(0x40000000)) };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftQuad_DrawFacing", &PROBE_N, &[], true); }
    r_
}

pub fn EftQuad_SetSheet(c: &mut Ctx, mut p_w_1: u32, mut p_cols_2: u8, mut p_rows_3: u8) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftQuad_SetSheet", &PROBE_N, &[(p_w_1) as u32, (p_cols_2) as u32, (p_rows_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_i_4: i32 = 0;
    let mut l_j_5: i32 = 0;
    let mut l_n_6: i32 = 0;
    let r_ = 'f: {
        if (((((((p_cols_2) as i32)) == (0i32)) as i32)) != 0) {
            p_cols_2 = ((1i32) as u8);
        } else {
            if (((((((p_cols_2) as i32)) > (4i32)) as i32)) != 0) {
                p_cols_2 = ((4i32) as u8);
            }
        }
        if (((((((p_rows_3) as i32)) == (0i32)) as i32)) != 0) {
            p_rows_3 = ((1i32) as u8);
        } else {
            if (((((((p_rows_3) as i32)) > (4i32)) as i32)) != 0) {
                p_rows_3 = ((4i32) as u8);
            }
        }
        { let v_ = rt::fdiv(F32(0x3f800000), rt::i2f(((p_cols_2) as i32))); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fdiv(F32(0x3f800000), rt::i2f(((p_rows_3) as i32))); let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = (((((p_cols_2) as i32)).wrapping_mul(((p_rows_3) as i32))) as u8); let a_ = (p_w_1).wrapping_add(956); c.m.w8(a_, v_); };
        l_n_6 = 0i32;
        {
            l_j_5 = 0i32;
            'l1: loop {
                if !(((((l_j_5) < (((p_rows_3) as i32))) as i32)) != 0) { break 'l1; }
                'c2: {
                    {
                        l_i_4 = 0i32;
                        'l3: loop {
                            if !(((((l_i_4) < (((p_cols_2) as i32))) as i32)) != 0) { break 'l3; }
                            'c4: {
                                { let v_ = rt::fmul(F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), rt::i2f(l_i_4)); let a_ = (((p_w_1).wrapping_add(120)).wrapping_add((((l_n_6) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                                { let v_ = rt::fmul(F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), rt::i2f(l_j_5)); let a_ = (((p_w_1).wrapping_add(120)).wrapping_add((((l_n_6) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                                { let v_ = rt::fadd(rt::fmul(F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), rt::i2f(l_i_4)), F32(c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a_ = (((p_w_1).wrapping_add(120)).wrapping_add((((l_n_6) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                                { let v_ = rt::fadd(rt::fmul(F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), rt::i2f(l_j_5)), F32(c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); let a_ = (((p_w_1).wrapping_add(120)).wrapping_add((((l_n_6) as u32)).wrapping_mul(16))).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                                { let o_ = l_n_6; let n_ = (o_).wrapping_add(1); l_n_6 = n_; };
                            }
                            { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
                        }
                    }
                }
                { let o_ = l_j_5; let n_ = (o_).wrapping_add(1); l_j_5 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftQuad_SetSheet", &PROBE_N, &[], true); }
    r_
}

pub fn EftQuad_SetTex(c: &mut Ctx, mut p_w_1: u32, mut p_tex_2: u32, mut p_a_3: i32, mut p_b_4: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftQuad_SetTex", &PROBE_N, &[(p_w_1) as u32, (p_tex_2) as u32, (p_a_3) as u32, (p_b_4) as u32], false); }
    let r_ = 'f: {
    { let v_ = ((p_b_4) as u8); let a_ = (p_w_1).wrapping_add(959); c.m.w8(a_, v_); };
    { let v_ = p_tex_2; let a_ = (p_w_1).wrapping_add(8); c.m.w32(a_, v_); };
    { let v_ = ((p_a_3) as u8); let a_ = (p_w_1).wrapping_add(958); c.m.w8(a_, v_); };
    { let s_ = (p_tex_2).wrapping_add((((p_a_3) as u32)).wrapping_mul(16)); let d_ = (p_w_1).wrapping_add(16); rt::memcpy(c, d_, s_, 16); };
    { let s_ = (p_tex_2).wrapping_add((((p_b_4) as u32)).wrapping_mul(16)); let d_ = (p_w_1).wrapping_add(32); rt::memcpy(c, d_, s_, 16); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftQuad_SetTex", &PROBE_N, &[], true); }
    r_
}

pub fn EftQuad_LoadTex(c: &mut Ctx, mut p_w_1: u32, mut p_tex_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftQuad_LoadTex", &PROBE_N, &[(p_w_1) as u32, (p_tex_2) as u32], false); }
    let mut l_t_3: u64 = 0;
    let r_ = 'f: {
    if (((((((((p_tex_2) != (((0i32) as u32))) as i32)) != 0) && ((((!((((c.m.r32((p_tex_2).wrapping_add(132))) & ((((1i32).wrapping_shl((((c.m.r8((p_w_1).wrapping_add(958))) as i32)) as u32)) as u32)))) != 0)) as i32)) != 0)) as i32)) != 0) {
        l_t_3 = F::EftVram_AddImage(c, (p_w_1).wrapping_add(16), 1i32, 0i32);
        { let r_ = (F::EftVram_AddClut(c, (p_w_1).wrapping_add(32))).wrapping_shl((37i32) as u32); let n_ = ((l_t_3) | (r_)); l_t_3 = n_; };
        { let v_ = l_t_3; let a_ = (p_tex_2).wrapping_add((((c.m.r8((p_w_1).wrapping_add(958))) as u32)).wrapping_mul(16)); c.m.w64(a_, v_); };
        { let r_ = (((1i32).wrapping_shl((((c.m.r8((p_w_1).wrapping_add(958))) as i32)) as u32)) as u32); let a_ = (p_tex_2).wrapping_add(132); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftQuad_LoadTex", &PROBE_N, &[], true); }
    r_
}

pub fn EftQuad_SetLastKey(c: &mut Ctx, mut p_w_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftQuad_SetLastKey", &PROBE_N, &[(p_w_1) as u32], false); }
    let mut l_def_2: u32 = 0;
    let mut l_def2_3: u32 = 0;
    let r_ = 'f: {
    l_def_2 = c.m.r32(p_w_1);
    l_def2_3 = c.m.r32((p_w_1).wrapping_add(4));
    { let v_ = F32(c.m.r32((l_def_2).wrapping_add((((2i32) as u32)).wrapping_mul(12)))); let a_ = (p_w_1).wrapping_add(668); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32(((l_def_2).wrapping_add((((2i32) as u32)).wrapping_mul(12))).wrapping_add(4))); let a_ = ((p_w_1).wrapping_add(668)).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32(((l_def_2).wrapping_add((((2i32) as u32)).wrapping_mul(12))).wrapping_add(8))); let a_ = ((p_w_1).wrapping_add(668)).wrapping_add(8); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32(((l_def_2).wrapping_add(36)).wrapping_add((((2i32) as u32)).wrapping_mul(12)))); let a_ = ((p_w_1).wrapping_add(668)).wrapping_add(12); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((((l_def_2).wrapping_add(36)).wrapping_add((((2i32) as u32)).wrapping_mul(12))).wrapping_add(4))); let a_ = (((p_w_1).wrapping_add(668)).wrapping_add(12)).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((((l_def_2).wrapping_add(36)).wrapping_add((((2i32) as u32)).wrapping_mul(12))).wrapping_add(8))); let a_ = (((p_w_1).wrapping_add(668)).wrapping_add(12)).wrapping_add(8); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32(((l_def_2).wrapping_add(72)).wrapping_add((((2i32) as u32)).wrapping_mul(12)))); let a_ = ((p_w_1).wrapping_add(668)).wrapping_add(24); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((((l_def_2).wrapping_add(72)).wrapping_add((((2i32) as u32)).wrapping_mul(12))).wrapping_add(4))); let a_ = (((p_w_1).wrapping_add(668)).wrapping_add(24)).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((((l_def_2).wrapping_add(72)).wrapping_add((((2i32) as u32)).wrapping_mul(12))).wrapping_add(8))); let a_ = (((p_w_1).wrapping_add(668)).wrapping_add(24)).wrapping_add(8); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32(((l_def_2).wrapping_add(108)).wrapping_add((((2i32) as u32)).wrapping_mul(12)))); let a_ = ((p_w_1).wrapping_add(668)).wrapping_add(36); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((((l_def_2).wrapping_add(108)).wrapping_add((((2i32) as u32)).wrapping_mul(12))).wrapping_add(4))); let a_ = (((p_w_1).wrapping_add(668)).wrapping_add(36)).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((((l_def_2).wrapping_add(108)).wrapping_add((((2i32) as u32)).wrapping_mul(12))).wrapping_add(8))); let a_ = (((p_w_1).wrapping_add(668)).wrapping_add(36)).wrapping_add(8); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32(((l_def_2).wrapping_add(420)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); let a_ = ((p_w_1).wrapping_add(668)).wrapping_add(56); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32(((l_def_2).wrapping_add(444)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); let a_ = ((p_w_1).wrapping_add(668)).wrapping_add(64); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32(((l_def_2).wrapping_add(468)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); let a_ = ((p_w_1).wrapping_add(668)).wrapping_add(72); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32(((l_def_2).wrapping_add(432)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); let a_ = ((p_w_1).wrapping_add(668)).wrapping_add(60); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32(((l_def_2).wrapping_add(456)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); let a_ = ((p_w_1).wrapping_add(668)).wrapping_add(68); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32(((l_def_2).wrapping_add(480)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); let a_ = ((p_w_1).wrapping_add(668)).wrapping_add(76); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32(((l_def_2).wrapping_add(144)).wrapping_add((((2i32) as u32)).wrapping_mul(12)))); let a_ = ((p_w_1).wrapping_add(668)).wrapping_add(92); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((((l_def_2).wrapping_add(144)).wrapping_add((((2i32) as u32)).wrapping_mul(12))).wrapping_add(4))); let a_ = (((p_w_1).wrapping_add(668)).wrapping_add(92)).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((((l_def_2).wrapping_add(144)).wrapping_add((((2i32) as u32)).wrapping_mul(12))).wrapping_add(8))); let a_ = (((p_w_1).wrapping_add(668)).wrapping_add(92)).wrapping_add(8); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32(((l_def_2).wrapping_add(180)).wrapping_add((((2i32) as u32)).wrapping_mul(12)))); let a_ = ((p_w_1).wrapping_add(668)).wrapping_add(104); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((((l_def_2).wrapping_add(180)).wrapping_add((((2i32) as u32)).wrapping_mul(12))).wrapping_add(4))); let a_ = (((p_w_1).wrapping_add(668)).wrapping_add(104)).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((((l_def_2).wrapping_add(180)).wrapping_add((((2i32) as u32)).wrapping_mul(12))).wrapping_add(8))); let a_ = (((p_w_1).wrapping_add(668)).wrapping_add(104)).wrapping_add(8); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32(((l_def_2).wrapping_add(216)).wrapping_add((((2i32) as u32)).wrapping_mul(12)))); let a_ = ((p_w_1).wrapping_add(668)).wrapping_add(116); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((((l_def_2).wrapping_add(216)).wrapping_add((((2i32) as u32)).wrapping_mul(12))).wrapping_add(4))); let a_ = (((p_w_1).wrapping_add(668)).wrapping_add(116)).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((((l_def_2).wrapping_add(216)).wrapping_add((((2i32) as u32)).wrapping_mul(12))).wrapping_add(8))); let a_ = (((p_w_1).wrapping_add(668)).wrapping_add(116)).wrapping_add(8); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32(((l_def_2).wrapping_add(288)).wrapping_add((((2i32) as u32)).wrapping_mul(12)))); let a_ = ((p_w_1).wrapping_add(668)).wrapping_add(140); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((((l_def_2).wrapping_add(288)).wrapping_add((((2i32) as u32)).wrapping_mul(12))).wrapping_add(4))); let a_ = (((p_w_1).wrapping_add(668)).wrapping_add(140)).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((((l_def_2).wrapping_add(288)).wrapping_add((((2i32) as u32)).wrapping_mul(12))).wrapping_add(8))); let a_ = (((p_w_1).wrapping_add(668)).wrapping_add(140)).wrapping_add(8); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32(((l_def_2).wrapping_add(324)).wrapping_add((((2i32) as u32)).wrapping_mul(12)))); let a_ = ((p_w_1).wrapping_add(668)).wrapping_add(152); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((((l_def_2).wrapping_add(324)).wrapping_add((((2i32) as u32)).wrapping_mul(12))).wrapping_add(4))); let a_ = (((p_w_1).wrapping_add(668)).wrapping_add(152)).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((((l_def_2).wrapping_add(324)).wrapping_add((((2i32) as u32)).wrapping_mul(12))).wrapping_add(8))); let a_ = (((p_w_1).wrapping_add(668)).wrapping_add(152)).wrapping_add(8); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32(((l_def_2).wrapping_add(360)).wrapping_add((((2i32) as u32)).wrapping_mul(8)))); let a_ = ((p_w_1).wrapping_add(668)).wrapping_add(164); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((((l_def_2).wrapping_add(360)).wrapping_add((((2i32) as u32)).wrapping_mul(8))).wrapping_add(4))); let a_ = (((p_w_1).wrapping_add(668)).wrapping_add(164)).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32(((l_def_2).wrapping_add(384)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); let a_ = ((p_w_1).wrapping_add(668)).wrapping_add(172); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32(((l_def_2).wrapping_add(396)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); let a_ = ((p_w_1).wrapping_add(668)).wrapping_add(48); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32(((l_def_2).wrapping_add(408)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); let a_ = ((p_w_1).wrapping_add(668)).wrapping_add(52); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32(((l_def_2).wrapping_add(492)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); let a_ = ((p_w_1).wrapping_add(668)).wrapping_add(80); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32(((l_def_2).wrapping_add(504)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); let a_ = ((p_w_1).wrapping_add(668)).wrapping_add(84); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32(((l_def_2).wrapping_add(516)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); let a_ = ((p_w_1).wrapping_add(668)).wrapping_add(88); c.m.w32(a_, (v_).0); };
    F::Vec4_Copy(c, ((p_w_1).wrapping_add(668)).wrapping_add(180), (l_def2_3).wrapping_add((((2i32) as u32)).wrapping_mul(16)));
    F::Vec4_Copy(c, ((p_w_1).wrapping_add(668)).wrapping_add(196), ((l_def2_3).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(16)));
    F::Vec4_Copy(c, ((p_w_1).wrapping_add(668)).wrapping_add(212), ((l_def2_3).wrapping_add(96)).wrapping_add((((2i32) as u32)).wrapping_mul(16)));
    F::Vec4_Copy(c, ((p_w_1).wrapping_add(668)).wrapping_add(228), ((l_def2_3).wrapping_add(144)).wrapping_add((((2i32) as u32)).wrapping_mul(16)));
    { let v_ = F32(c.m.r32(((l_def2_3).wrapping_add(192)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); let a_ = ((p_w_1).wrapping_add(668)).wrapping_add(244); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32(((l_def2_3).wrapping_add(204)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); let a_ = ((p_w_1).wrapping_add(668)).wrapping_add(248); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32(((l_def2_3).wrapping_add(216)).wrapping_add((((2i32) as u32)).wrapping_mul(8)))); let a_ = ((p_w_1).wrapping_add(668)).wrapping_add(252); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32(((l_def2_3).wrapping_add(240)).wrapping_add((((2i32) as u32)).wrapping_mul(8)))); let a_ = ((p_w_1).wrapping_add(668)).wrapping_add(256); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32(((l_def2_3).wrapping_add(264)).wrapping_add((((2i32) as u32)).wrapping_mul(8)))); let a_ = ((p_w_1).wrapping_add(668)).wrapping_add(260); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((((l_def2_3).wrapping_add(216)).wrapping_add((((2i32) as u32)).wrapping_mul(8))).wrapping_add(4))); let a_ = ((p_w_1).wrapping_add(668)).wrapping_add(264); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((((l_def2_3).wrapping_add(240)).wrapping_add((((2i32) as u32)).wrapping_mul(8))).wrapping_add(4))); let a_ = ((p_w_1).wrapping_add(668)).wrapping_add(268); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((((l_def2_3).wrapping_add(264)).wrapping_add((((2i32) as u32)).wrapping_mul(8))).wrapping_add(4))); let a_ = ((p_w_1).wrapping_add(668)).wrapping_add(272); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32(((l_def2_3).wrapping_add(288)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); let a_ = ((p_w_1).wrapping_add(668)).wrapping_add(276); c.m.w32(a_, (v_).0); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftQuad_SetLastKey", &PROBE_N, &[], true); }
    r_
}

pub fn EftQuad_CreateEx(c: &mut Ctx, mut p_def_1: u32, mut p_def2_2: u32, mut p_tex_3: u32, p_pos_4: u32, p_dir_5: u32, mut p_size_6: F32, mut p_life_7: F32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftQuad_CreateEx", &PROBE_N, &[(p_def_1) as u32, (p_def2_2) as u32, (p_tex_3) as u32, (p_pos_4) as u32, (p_dir_5) as u32, (p_size_6).0, (p_life_7).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(96) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 96);
    rt::memcpy(c, fp, p_pos_4, 16);
    rt::memcpy(c, fp.wrapping_add(16), p_dir_5, 16);
    let r_ = 'f: {
        { let _ = F::memset(c, fp.wrapping_add(32), 0i32, 64u32); };
        if (((((c.m.r32(sym::gEftQuadMgr)) == (((0i32) as u32))) as i32)) != 0) {
            break 'f ((0i32) as u32);
        }
        if (((((p_def_1) == (((0i32) as u32))) as i32)) != 0) {
            break 'f ((0i32) as u32);
        }
        if (((((p_def2_2) == (((0i32) as u32))) as i32)) != 0) {
            break 'f ((0i32) as u32);
        }
        { let v_ = p_tex_3; let a_ = (fp.wrapping_add(32)).wrapping_add(8); c.m.w32(a_, v_); };
        { let v_ = p_def_1; let a_ = fp.wrapping_add(32); c.m.w32(a_, v_); };
        { let v_ = p_def2_2; let a_ = (fp.wrapping_add(32)).wrapping_add(4); c.m.w32(a_, v_); };
        { let s_ = fp; let d_ = (fp.wrapping_add(32)).wrapping_add(16); rt::memcpy(c, d_, s_, 16); };
        { let s_ = fp.wrapping_add(16); let d_ = (fp.wrapping_add(32)).wrapping_add(32); rt::memcpy(c, d_, s_, 16); };
        { let v_ = p_size_6; let a_ = (fp.wrapping_add(32)).wrapping_add(48); c.m.w32(a_, (v_).0); };
        { let v_ = p_life_7; let a_ = (fp.wrapping_add(32)).wrapping_add(52); c.m.w32(a_, (v_).0); };
        break 'f { let a0_ = { let a_ = (c.m.r32(sym::gEftQuadMgr)).wrapping_add(76804); c.m.r32(a_) }; F::BtlTaskList_AddTail(c, a0_, sym::gEftQuadClass, fp.wrapping_add(32)) };
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftQuad_CreateEx", &PROBE_N, &[], true); }
    r_
}

pub fn EftQuad_Create(c: &mut Ctx, mut p_arg_1: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftQuad_Create", &PROBE_N, &[(p_arg_1) as u32], false); }
    let r_ = 'f: {
    if (((((c.m.r32(sym::gEftQuadMgr)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f ((0i32) as u32);
    }
    if (((((p_arg_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f ((0i32) as u32);
    }
    break 'f { let a0_ = { let a_ = (c.m.r32(sym::gEftQuadMgr)).wrapping_add(76804); c.m.r32(a_) }; F::BtlTaskList_AddTail(c, a0_, sym::gEftQuadClass, p_arg_1) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftQuad_Create", &PROBE_N, &[], true); }
    r_
}

pub fn EftQuad_StopAlias(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftQuad_StopAlias", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    F::EftQuad_Stop(c, p_task_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftQuad_StopAlias", &PROBE_N, &[], true); }
    r_
}

pub fn EftQuad_Stop(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftQuad_Stop", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let mut l_now_3: i32 = 0;
    let r_ = 'f: {
    l_now_3 = 1i32;
    if (((((c.m.r32(sym::gEftQuadMgr)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    if (((((l_w_2) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    if ((((!(((((c.m.r32((l_w_2).wrapping_add(964)) as i32)) & (1i32))) != 0)) as i32)) != 0) {
        break 'f;
    }
    if (((rt::fgt(F32(c.m.r32((l_w_2).wrapping_add(108))), F32(0x00000000)) as i32)) != 0) {
        { let r_ = 4i32; let a_ = (l_w_2).wrapping_add(964); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        l_now_3 = 0i32;
    }
    if (((rt::fgt(F32(c.m.r32((l_w_2).wrapping_add(112))), F32(0x00000000)) as i32)) != 0) {
        { let r_ = 8i32; let a_ = (l_w_2).wrapping_add(964); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        l_now_3 = 0i32;
    }
    if ((l_now_3) != 0) {
        { let r_ = 2i32; let a_ = (l_w_2).wrapping_add(964); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftQuad_Stop", &PROBE_N, &[], true); }
    r_
}

pub fn EftQuad_SetFade(c: &mut Ctx, mut p_task_1: u32, mut p_frames_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftQuad_SetFade", &PROBE_N, &[(p_task_1) as u32, (p_frames_2) as u32], false); }
    let mut l_w_3: u32 = 0;
    let mut l_f_4: F32 = F32(0);
    let r_ = 'f: {
    if (((((c.m.r32(sym::gEftQuadMgr)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
    if (((((l_w_3) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    if (((((c.m.r32((l_w_3).wrapping_add(964)) as i32)) & (1i32))) != 0) {
        l_f_4 = rt::i2f(p_frames_2);
        { let v_ = l_f_4; let a_ = (l_w_3).wrapping_add(112); c.m.w32(a_, (v_).0); };
        { let v_ = l_f_4; let a_ = (l_w_3).wrapping_add(116); c.m.w32(a_, (v_).0); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftQuad_SetFade", &PROBE_N, &[], true); }
    r_
}

pub fn EftQuad_SetPosDir(c: &mut Ctx, mut p_task_1: u32, p_pos_2: u32, p_dir_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftQuad_SetPosDir", &PROBE_N, &[(p_task_1) as u32, (p_pos_2) as u32, (p_dir_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_w_4: u32 = 0;
    rt::memcpy(c, fp, p_pos_2, 16);
    rt::memcpy(c, fp.wrapping_add(16), p_dir_3, 16);
    let r_ = 'f: {
        if (((((c.m.r32(sym::gEftQuadMgr)) == (((0i32) as u32))) as i32)) != 0) {
            break 'f;
        }
        if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
            break 'f;
        }
        l_w_4 = c.m.r32((p_task_1).wrapping_add(56));
        if (((((l_w_4) == (((0i32) as u32))) as i32)) != 0) {
            break 'f;
        }
        if (((((c.m.r32((l_w_4).wrapping_add(964)) as i32)) & (1i32))) != 0) {
            F::Vec3_Copy(c, (l_w_4).wrapping_add(48), fp);
            F::Vec3_Copy(c, (l_w_4).wrapping_add(64), fp.wrapping_add(16));
            F::Vec3_Normalize(c, (l_w_4).wrapping_add(64), (l_w_4).wrapping_add(64));
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftQuad_SetPosDir", &PROBE_N, &[], true); }
    r_
}

pub fn EftQuad_Kill(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftQuad_Kill", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
    if (((((c.m.r32(sym::gEftQuadMgr)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    if (((((l_w_2) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    if (((((c.m.r32((l_w_2).wrapping_add(964)) as i32)) & (1i32))) != 0) {
        { let r_ = ((32i32) | (2i32)); let a_ = (l_w_2).wrapping_add(964); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftQuad_Kill", &PROBE_N, &[], true); }
    r_
}

pub fn EftQuad_SetPos(c: &mut Ctx, mut p_task_1: u32, p_pos_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftQuad_SetPos", &PROBE_N, &[(p_task_1) as u32, (p_pos_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_w_3: u32 = 0;
    rt::memcpy(c, fp, p_pos_2, 16);
    let r_ = 'f: {
        if (((((c.m.r32(sym::gEftQuadMgr)) == (((0i32) as u32))) as i32)) != 0) {
            break 'f;
        }
        if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
            break 'f;
        }
        l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
        if (((((l_w_3) == (((0i32) as u32))) as i32)) != 0) {
            break 'f;
        }
        if (((((c.m.r32((l_w_3).wrapping_add(964)) as i32)) & (1i32))) != 0) {
            F::Vec3_Copy(c, (l_w_3).wrapping_add(48), fp);
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftQuad_SetPos", &PROBE_N, &[], true); }
    r_
}

pub fn EftQuad_Warp(c: &mut Ctx, mut p_task_1: u32, p_pos_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftQuad_Warp", &PROBE_N, &[(p_task_1) as u32, (p_pos_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_w_3: u32 = 0;
    let mut l_q_4: u32 = 0;
    let mut l_pitch_5: F32 = F32(0);
    let mut l_yaw_6: F32 = F32(0);
    rt::memcpy(c, fp, p_pos_2, 16);
    let r_ = 'f: {
        if (((((c.m.r32(sym::gEftQuadMgr)) == (((0i32) as u32))) as i32)) != 0) {
            break 'f;
        }
        if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
            break 'f;
        }
        l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
        if (((((l_w_3) == (((0i32) as u32))) as i32)) != 0) {
            break 'f;
        }
        if (((((c.m.r32((l_w_3).wrapping_add(964)) as i32)) & (1i32))) != 0) {
            l_pitch_5 = { let a0_ = { let a0_ = rt::fneg(F32(c.m.r32(((l_w_3).wrapping_add(64)).wrapping_add(4)))); F::Mathf_Asin(c, a0_) }; F::EftMath_WrapAngle(c, a0_) };
            l_yaw_6 = { let a0_ = { let a0_ = F32(c.m.r32((l_w_3).wrapping_add(64))); let a1_ = F32(c.m.r32(((l_w_3).wrapping_add(64)).wrapping_add(8))); F::Port_atan2f(c, a0_, a1_) }; F::EftMath_WrapAngle(c, a0_) };
            F::Vec3_Copy(c, (l_w_3).wrapping_add(48), fp);
            F::Vu0Cur_Push(c);
            {
                l_q_4 = c.m.r32((l_w_3).wrapping_add(968));
                'l1: loop {
                    if !(((((l_q_4) != (((0i32) as u32))) as i32)) != 0) { break 'l1; }
                    'c2: {
                        F::Vu0Cur_LoadIdentity(c);
                        { let a0_ = F32(c.m.r32((l_w_3).wrapping_add(92))); F::Vu0Cur_ScaleDiagUniform(c, a0_) };
                        if (((((c.m.r32((l_w_3).wrapping_add(964)) as i32)) & (256i32))) != 0) {
                            F::EftQuad_BuildCorners(c, l_q_4, l_w_3, (l_q_4).wrapping_add(16), l_pitch_5, l_yaw_6);
                        } else {
                            F::EftQuad_BuildCorners(c, l_q_4, l_w_3, (l_w_3).wrapping_add(48), l_pitch_5, l_yaw_6);
                        }
                    }
                    l_q_4 = c.m.r32((l_q_4).wrapping_add(364));
                }
            }
            F::Vu0Cur_Pop(c);
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftQuad_Warp", &PROBE_N, &[], true); }
    r_
}

pub fn EftQuad_SetDir(c: &mut Ctx, mut p_task_1: u32, p_dir_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftQuad_SetDir", &PROBE_N, &[(p_task_1) as u32, (p_dir_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_w_3: u32 = 0;
    rt::memcpy(c, fp, p_dir_2, 16);
    let r_ = 'f: {
        if (((((c.m.r32(sym::gEftQuadMgr)) == (((0i32) as u32))) as i32)) != 0) {
            break 'f;
        }
        if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
            break 'f;
        }
        l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
        if (((((l_w_3) == (((0i32) as u32))) as i32)) != 0) {
            break 'f;
        }
        if (((((c.m.r32((l_w_3).wrapping_add(964)) as i32)) & (1i32))) != 0) {
            F::Vec3_Copy(c, (l_w_3).wrapping_add(64), fp);
            F::Vec3_Normalize(c, (l_w_3).wrapping_add(64), (l_w_3).wrapping_add(64));
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftQuad_SetDir", &PROBE_N, &[], true); }
    r_
}

pub fn EftQuad_SetSize(c: &mut Ctx, mut p_task_1: u32, mut p_size_2: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftQuad_SetSize", &PROBE_N, &[(p_task_1) as u32, (p_size_2).0], false); }
    let mut l_w_3: u32 = 0;
    let r_ = 'f: {
    if (((((c.m.r32(sym::gEftQuadMgr)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
    if (((((l_w_3) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    if (((((c.m.r32((l_w_3).wrapping_add(964)) as i32)) & (1i32))) != 0) {
        { let v_ = p_size_2; let a_ = (l_w_3).wrapping_add(92); c.m.w32(a_, (v_).0); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftQuad_SetSize", &PROBE_N, &[], true); }
    r_
}

pub fn EftQuad_SetLife(c: &mut Ctx, mut p_task_1: u32, mut p_seconds_2: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftQuad_SetLife", &PROBE_N, &[(p_task_1) as u32, (p_seconds_2).0], false); }
    let mut l_w_3: u32 = 0;
    let r_ = 'f: {
    if (((((c.m.r32(sym::gEftQuadMgr)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
    if (((((l_w_3) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    if (((((c.m.r32((l_w_3).wrapping_add(964)) as i32)) & (1i32))) != 0) {
        { let v_ = rt::fmul(p_seconds_2, F32(0x41f00000)); let a_ = (l_w_3).wrapping_add(96); c.m.w32(a_, (v_).0); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftQuad_SetLife", &PROBE_N, &[], true); }
    r_
}

pub fn EftQuad_SetTexTable(c: &mut Ctx, mut p_task_1: u32, mut p_tex_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftQuad_SetTexTable", &PROBE_N, &[(p_task_1) as u32, (p_tex_2) as u32], false); }
    let mut l_w_3: u32 = 0;
    let r_ = 'f: {
    if (((((c.m.r32(sym::gEftQuadMgr)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    if (((((p_tex_2) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
    if (((((l_w_3) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    if (((((c.m.r32((l_w_3).wrapping_add(964)) as i32)) & (1i32))) != 0) {
        { let v_ = p_tex_2; let a_ = (l_w_3).wrapping_add(8); c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftQuad_SetTexTable", &PROBE_N, &[], true); }
    r_
}

pub fn EftQuad_SetTexPair(c: &mut Ctx, mut p_task_1: u32, mut p_tex_2: u32, mut p_a_3: i32, mut p_b_4: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftQuad_SetTexPair", &PROBE_N, &[(p_task_1) as u32, (p_tex_2) as u32, (p_a_3) as u32, (p_b_4) as u32], false); }
    let mut l_w_5: u32 = 0;
    let r_ = 'f: {
    if (((((c.m.r32(sym::gEftQuadMgr)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    if (((((p_tex_2) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    l_w_5 = c.m.r32((p_task_1).wrapping_add(56));
    if (((((l_w_5) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    if ((((!(((((c.m.r32((l_w_5).wrapping_add(964)) as i32)) & (1i32))) != 0)) as i32)) != 0) {
        break 'f;
    }
    F::EftQuad_SetTex(c, l_w_5, p_tex_2, p_a_3, p_b_4);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftQuad_SetTexPair", &PROBE_N, &[], true); }
    r_
}

pub fn EftQuad_SetDelay(c: &mut Ctx, mut p_task_1: u32, mut p_frames_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftQuad_SetDelay", &PROBE_N, &[(p_task_1) as u32, (p_frames_2) as u32], false); }
    let mut l_w_3: u32 = 0;
    let r_ = 'f: {
    if (((((c.m.r32(sym::gEftQuadMgr)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
    if (((((l_w_3) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    if (((((c.m.r32((l_w_3).wrapping_add(964)) as i32)) & (1i32))) != 0) {
        { let v_ = rt::i2f(p_frames_2); let a_ = (l_w_3).wrapping_add(104); c.m.w32(a_, (v_).0); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftQuad_SetDelay", &PROBE_N, &[], true); }
    r_
}

pub fn EftQuad_SetStopDelay(c: &mut Ctx, mut p_task_1: u32, mut p_frames_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftQuad_SetStopDelay", &PROBE_N, &[(p_task_1) as u32, (p_frames_2) as u32], false); }
    let mut l_w_3: u32 = 0;
    let r_ = 'f: {
    if (((((c.m.r32(sym::gEftQuadMgr)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
    if (((((l_w_3) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    if (((((c.m.r32((l_w_3).wrapping_add(964)) as i32)) & (1i32))) != 0) {
        { let v_ = rt::i2f(p_frames_2); let a_ = (l_w_3).wrapping_add(108); c.m.w32(a_, (v_).0); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftQuad_SetStopDelay", &PROBE_N, &[], true); }
    r_
}

pub fn EftQuad_SetViewOnly(c: &mut Ctx, mut p_task_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftQuad_SetViewOnly", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
    if (((((c.m.r32(sym::gEftQuadMgr)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    if (((((l_w_2) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((!(((((c.m.r32((l_w_2).wrapping_add(964)) as i32)) & (1i32))) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    { let r_ = 128i32; let a_ = (l_w_2).wrapping_add(964); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftQuad_SetViewOnly", &PROBE_N, &[], true); }
    r_
}

pub fn EftQuad_IsAlive(c: &mut Ctx, mut p_task_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftQuad_IsAlive", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
    if (((((c.m.r32(sym::gEftQuadMgr)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    if (((((l_w_2) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if (((((c.m.r32((l_w_2).wrapping_add(964)) as i32)) & (1i32))) != 0) {
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftQuad_IsAlive", &PROBE_N, &[], true); }
    r_
}

pub fn EftQuad_SetOwnOrigin(c: &mut Ctx, mut p_task_1: u32, mut p_on_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftQuad_SetOwnOrigin", &PROBE_N, &[(p_task_1) as u32, (p_on_2) as u32], false); }
    let mut l_w_3: u32 = 0;
    let r_ = 'f: {
    if (((((c.m.r32(sym::gEftQuadMgr)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
    if (((((l_w_3) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    if (((((c.m.r32((l_w_3).wrapping_add(964)) as i32)) & (1i32))) != 0) {
        if ((p_on_2) != 0) {
            { let r_ = 256i32; let a_ = (l_w_3).wrapping_add(964); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        } else {
            { let r_ = (!(256i32)); let a_ = (l_w_3).wrapping_add(964); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftQuad_SetOwnOrigin", &PROBE_N, &[], true); }
    r_
}

pub fn EftQuad_SetCut(c: &mut Ctx, mut p_task_1: u32, mut p_cut_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftQuad_SetCut", &PROBE_N, &[(p_task_1) as u32, (p_cut_2) as u32], false); }
    let mut l_w_3: u32 = 0;
    let r_ = 'f: {
    if (((((c.m.r32(sym::gEftQuadMgr)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
    if (((((l_w_3) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    if (((((c.m.r32((l_w_3).wrapping_add(964)) as i32)) & (1i32))) != 0) {
        { let v_ = ((p_cut_2) as u8); let a_ = (l_w_3).wrapping_add(960); c.m.w8(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftQuad_SetCut", &PROBE_N, &[], true); }
    r_
}
