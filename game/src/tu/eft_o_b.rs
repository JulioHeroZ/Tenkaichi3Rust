#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn EftBlastObj_AddHit(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBlastObj_AddHit", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let mut l_rec_3: u32 = 0;
    let mut l_src_4: u32 = 0;
    let mut l_r_5: F32 = F32(0);
    let mut l_a_6: u32 = 0;
    let mut l_b_7: u32 = 0;
    let mut l_a_8: u32 = 0;
    let mut l_b_9: u32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    l_rec_3 = F::EftHit_GetNew(c);
    l_src_4 = c.m.r32(l_w_2);
    l_r_5 = rt::fmul(F32(c.m.r32((l_w_2).wrapping_add(1420))), F::EftEmit_GetTrailWidth(c, (l_w_2).wrapping_add(656)));
    { let s_ = (l_w_2).wrapping_add(1440); let d_ = (l_rec_3).wrapping_add(16); rt::memcpy(c, d_, s_, 64); };
    { let v_ = p_task_1; let a_ = (l_rec_3).wrapping_add(96); c.m.w32(a_, v_); };
    { let v_ = l_src_4; let a_ = (l_rec_3).wrapping_add(100); c.m.w32(a_, v_); };
    if (((((c.m.r32((l_w_2).wrapping_add(1408)) as i32)) & (256i32))) != 0) {
        { let r_ = 128i32; let a_ = (l_rec_3).wrapping_add(80); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
    if (((((c.m.r32((l_w_2).wrapping_add(1408)) as i32)) & (2048i32))) != 0) {
        { let r_ = 16i32; let a_ = (l_rec_3).wrapping_add(80); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
    's1: {
        let sv10: i32 = (({ let a_ = (c.m.r32((l_src_4).wrapping_add(36))).wrapping_add(8); (c.m.r8(a_) as i8) }) as i32);
        match sv10 {
            1i32 => {
                {
                    l_a_6 = F::EftHitArena_AllocBox(c);
                    l_b_7 = F::EftHitArena_AllocBox(c);
                    F::ColCapsule_Set(c, l_a_6, (l_w_2).wrapping_add(1440), ((l_w_2).wrapping_add(1440)).wrapping_add(16), l_r_5);
                    F::ColCapsule_Set(c, l_b_7, (l_w_2).wrapping_add(1440), ((l_w_2).wrapping_add(1440)).wrapping_add(32), l_r_5);
                    F::EftHit_SetShapeBoxes(c, l_rec_3, l_a_6, l_b_7);
                    break 's1;
                }
            }
            0i32 => {
                {
                    l_a_8 = F::EftHitArena_AllocSphere(c);
                    l_b_9 = F::EftHitArena_AllocSphere(c);
                    F::ColSphere_Set(c, l_a_8, ((l_w_2).wrapping_add(1440)).wrapping_add(16), l_r_5);
                    F::ColSphere_Set(c, l_b_9, ((l_w_2).wrapping_add(1440)).wrapping_add(32), l_r_5);
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
    if rt::probing() { crate::host::harness::probe(c, "EftBlastObj_AddHit", &PROBE_N, &[], true); }
    r_
}

pub fn EftBlastObj_InitModel(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBlastObj_InitModel", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_m_2: u32 = 0;
    let r_ = 'f: {
    l_m_2 = (c.m.r32((p_task_1).wrapping_add(56))).wrapping_add(1504);
    { let v_ = (({ let a1_ = c.m.r32(l_m_2); F::EftObj_Create(c, (l_m_2).wrapping_add(4), a1_) }) as i8); let a_ = (l_m_2).wrapping_add(53); c.m.w8(a_, (v_) as u8); };
    { let v_ = (((1i32).wrapping_neg()) as i8); let a_ = (l_m_2).wrapping_add(54); c.m.w8(a_, (v_) as u8); };
    { let a0_ = (((c.m.r8((l_m_2).wrapping_add(53)) as i8)) as i32); F::EftObj_SetVisible(c, a0_, 0i32) };
    { let a0_ = (((c.m.r8((l_m_2).wrapping_add(53)) as i8)) as i32); let _ = a0_; let _ = 0i32; F::EftObj_Nop(c) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBlastObj_InitModel", &PROBE_N, &[], true); }
    r_
}

pub fn EftBlastObj_UpdateModel(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBlastObj_UpdateModel", &PROBE_N, &[(p_task_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(112) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 112);
    let mut l_change_2: i32 = 0;
    let mut l_mode_3: i32 = 0;
    let mut l_w_4: u32 = 0;
    let mut l_m_5: u32 = 0;
    let mut l_pitch_6: F32 = F32(0);
    let mut l_yaw_7: F32 = F32(0);
    let r_ = 'f: {
        l_change_2 = 0i32;
        l_mode_3 = 1i32;
        l_w_4 = c.m.r32((p_task_1).wrapping_add(56));
        F::Mtx_StoreIdentity(c, fp);
        { let a2_ = F32(c.m.r32((l_w_4).wrapping_add(1420))); F::Mtx_ScaleDiagUniform(c, fp, fp, a2_) };
        l_m_5 = (l_w_4).wrapping_add(1504);
        if (((((c.m.r32((l_w_4).wrapping_add(1408)) as i32)) & (8192i32))) != 0) {
            if ((((((((c.m.r8((l_m_5).wrapping_add(54)) as i8)) as i32)) < (0i32)) as i32)) != 0) {
                if (((rt::fge(F32(c.m.r32((l_m_5).wrapping_add(56))), F32(0x00000000)) as i32)) != 0) {
                    l_change_2 = 1i32;
                    { let a_ = (l_m_5).wrapping_add(54); let o_ = (c.m.r8(a_) as i8); let n_ = (o_).wrapping_add(1); c.m.w8(a_, (n_) as u8); };
                }
            } else {
                if ((((((((c.m.r8((l_m_5).wrapping_add(54)) as i8)) as i32)) < (1i32)) as i32)) != 0) {
                    if ((((!(({ let a0_ = (((c.m.r8((l_m_5).wrapping_add(53)) as i8)) as i32); F::EftObj_IsAnimPlaying(c, a0_) }) != 0)) as i32)) != 0) {
                        l_change_2 = 1i32;
                        { let v_ = F32(0x00000000); let a_ = (l_m_5).wrapping_add(56); c.m.w32(a_, (v_).0); };
                        l_mode_3 = 2i32;
                        { let a_ = (l_m_5).wrapping_add(54); let o_ = (c.m.r8(a_) as i8); let n_ = (o_).wrapping_add(1); c.m.w8(a_, (n_) as u8); };
                    }
                } else {
                    if ((((((((c.m.r8((l_m_5).wrapping_add(54)) as i8)) as i32)) < (2i32)) as i32)) != 0) {
                        if (((((c.m.r32((l_w_4).wrapping_add(1408)) as i32)) & (16384i32))) != 0) {
                            { let v_ = F32(0x00000000); let a_ = (l_m_5).wrapping_add(56); c.m.w32(a_, (v_).0); };
                            { let a_ = (l_m_5).wrapping_add(54); let o_ = (c.m.r8(a_) as i8); let n_ = (o_).wrapping_add(1); c.m.w8(a_, (n_) as u8); };
                            l_change_2 = 1i32;
                            { let r_ = (!(128i32)); let a_ = (l_w_4).wrapping_add(1408); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                        }
                    } else {
                        if ((((((((c.m.r8((l_m_5).wrapping_add(54)) as i8)) as i32)) < (3i32)) as i32)) != 0) {
                            if ((((!(({ let a0_ = (((c.m.r8((l_m_5).wrapping_add(53)) as i8)) as i32); F::EftObj_IsAnimPlaying(c, a0_) }) != 0)) as i32)) != 0) {
                                { let a_ = (l_m_5).wrapping_add(54); let o_ = (c.m.r8(a_) as i8); let n_ = (o_).wrapping_add(1); c.m.w8(a_, (n_) as u8); };
                                { let v_ = F32(0x00000000); let a_ = (l_m_5).wrapping_add(56); c.m.w32(a_, (v_).0); };
                                l_change_2 = 1i32;
                                { let r_ = (!(64i32)); let a_ = (l_w_4).wrapping_add(1408); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                            }
                        } else {
                            if ((((((((c.m.r8((l_m_5).wrapping_add(54)) as i8)) as i32)) < (4i32)) as i32)) != 0) {
                                if ((((!(({ let a0_ = (((c.m.r8((l_m_5).wrapping_add(53)) as i8)) as i32); F::EftObj_IsAnimPlaying(c, a0_) }) != 0)) as i32)) != 0) {
                                    l_change_2 = 1i32;
                                    { let v_ = F32(0x00000000); let a_ = (l_m_5).wrapping_add(56); c.m.w32(a_, (v_).0); };
                                    l_mode_3 = 2i32;
                                    { let a_ = (l_m_5).wrapping_add(54); let o_ = (c.m.r8(a_) as i8); let n_ = (o_).wrapping_add(1); c.m.w8(a_, (n_) as u8); };
                                }
                            }
                        }
                    }
                }
            }
            if ((l_change_2) != 0) {
                { let a0_ = (((c.m.r8((l_m_5).wrapping_add(53)) as i8)) as i32); let a1_ = (((c.m.r8((l_m_5).wrapping_add(54)) as i8)) as i32); F::EftObj_PlayAnim(c, a0_, a1_, l_mode_3) };
            }
            F::Vec3_Clamp(c, fp.wrapping_add(64), (l_w_4).wrapping_add(1376), F32(0xbf800000), F32(0x3f800000));
            if ((((!(((((c.m.r32((l_w_4).wrapping_add(1408)) as i32)) & (64i32))) != 0)) as i32)) != 0) {
                { let a2_ = { let a0_ = { let a0_ = F32(c.m.r32((fp.wrapping_add(64)).wrapping_add(4))); F::Mathf_Asin(c, a0_) }; F::EftMath_WrapAngle(c, a0_) }; F::Mtx_RotateX(c, fp, fp, a2_) };
            }
            { let a2_ = { let a0_ = rt::fadd({ let a0_ = F32(c.m.r32(fp.wrapping_add(64))); let a1_ = F32(c.m.r32((fp.wrapping_add(64)).wrapping_add(8))); F::Port_atan2f(c, a0_, a1_) }, F32(0x40490fda)); F::EftMath_WrapAngle(c, a0_) }; F::Mtx_RotateY(c, fp, fp, a2_) };
            F::Vec4_Copy(c, fp.wrapping_add(80), ((l_w_4).wrapping_add(1440)).wrapping_add(16));
            if ((((!(((((c.m.r32((l_w_4).wrapping_add(1408)) as i32)) & (64i32))) != 0)) as i32)) != 0) {
                rt::memset(c, fp.wrapping_add(96), 0, 16);
                { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(96).wrapping_add(0), (v_).0); }
                { let v_ = F32(0x40b00000); c.m.w32(fp.wrapping_add(96).wrapping_add(4), (v_).0); }
                { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(96).wrapping_add(8), (v_).0); }
                { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(96).wrapping_add(12), (v_).0); }
                if ((((((({ let a_ = { let a_ = (c.m.r32(l_w_4)).wrapping_add(36); c.m.r32(a_) }; (c.m.r16(a_) as i16) }) as i32)) == (622i32)) as i32)) != 0) {
                    { let v_ = rt::fmul(F32(c.m.r32((l_w_4).wrapping_add(1420))), F32(0x40b00000)); let a_ = (fp.wrapping_add(96)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                } else {
                    if ((((((({ let a_ = { let a_ = (c.m.r32(l_w_4)).wrapping_add(36); c.m.r32(a_) }; (c.m.r16(a_) as i16) }) as i32)) == (623i32)) as i32)) != 0) {
                        { let v_ = rt::fmul(F32(c.m.r32((l_w_4).wrapping_add(1420))), F32(0x40e00000)); let a_ = (fp.wrapping_add(96)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    }
                }
                F::Mtx_MulVec4(c, fp.wrapping_add(96), fp, fp.wrapping_add(96));
                F::Vec3_Add(c, fp.wrapping_add(80), fp.wrapping_add(80), fp.wrapping_add(96));
            }
            F::Mtx_Translate(c, fp, fp, fp.wrapping_add(80));
        } else {
            F::Vec3_Clamp(c, fp.wrapping_add(64), (l_w_4).wrapping_add(1376), F32(0xbf800000), F32(0x3f800000));
            l_pitch_6 = { let a0_ = F32(c.m.r32((fp.wrapping_add(64)).wrapping_add(4))); F::Mathf_Asin(c, a0_) };
            l_yaw_7 = rt::fadd({ let a0_ = F32(c.m.r32(fp.wrapping_add(64))); let a1_ = F32(c.m.r32((fp.wrapping_add(64)).wrapping_add(8))); F::Port_atan2f(c, a0_, a1_) }, F32(0x40490fda));
            l_pitch_6 = F::EftMath_WrapAngle(c, l_pitch_6);
            { let a2_ = F::EftMath_WrapAngle(c, l_yaw_7); F::Mtx_RotateY(c, fp, fp, a2_) };
            F::Mtx_RotateX(c, fp, fp, l_pitch_6);
            F::Mtx_Translate(c, fp, fp, ((l_w_4).wrapping_add(1440)).wrapping_add(16));
        }
        { let a0_ = (((c.m.r8((l_m_5).wrapping_add(53)) as i8)) as i32); F::EftObj_SetMtx(c, a0_, fp) };
        { let a0_ = (((c.m.r8((l_m_5).wrapping_add(53)) as i8)) as i32); F::EftObj_SetVisible(c, a0_, 1i32) };
        { let r_ = F32(0x3f800000); let a_ = (l_m_5).wrapping_add(56); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftBlastObj_UpdateModel", &PROBE_N, &[], true); }
    r_
}

pub fn EftBlastObj_TermModel(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBlastObj_TermModel", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    { let a0_ = (({ let a_ = ((c.m.r32((p_task_1).wrapping_add(56))).wrapping_add(1504)).wrapping_add(53); (c.m.r8(a_) as i8) }) as i32); { let _ = F::EftObj_Destroy(c, a0_); } };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBlastObj_TermModel", &PROBE_N, &[], true); }
    r_
}

pub fn EftBlastObj_UpdateParts(c: &mut Ctx, mut p_objId_1: i32, mut p_task_2: u32, mut p_set_3: u32, mut p_reset_4: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBlastObj_UpdateParts", &PROBE_N, &[(p_objId_1) as u32, (p_task_2) as u32, (p_set_3) as u32, (p_reset_4) as u32], false); }
    let mut l_w_5: u32 = 0;
    let mut l_g_6: i32 = 0;
    let mut l_p_7: i32 = 0;
    let mut l_gd_8: u32 = 0;
    let mut l_idx_9: i32 = 0;
    let mut l_skip_10: i32 = 0;
    let mut l_i_11: i32 = 0;
    let mut l_f_12: i32 = 0;
    let r_ = 'f: {
    l_w_5 = c.m.r32((p_task_2).wrapping_add(56));
    {
        l_g_6 = 0i32;
        'l1: loop {
            if !(((((l_g_6) < (19i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if (((({ let a_ = c.m.r32(p_set_3); c.m.r32(a_) }) & ((((1i32).wrapping_shl((l_g_6) as u32)) as u32)))) != 0) {
                    l_gd_8 = c.m.r32(((p_set_3).wrapping_add(12)).wrapping_add((((l_g_6) as u32)).wrapping_mul(16)));
                    {
                        l_p_7 = 0i32;
                        'l3: loop {
                            if !(((((l_p_7) < (((c.m.r8((l_gd_8).wrapping_add(1))) as i32))) as i32)) != 0) { break 'l3; }
                            'c4: {
                                l_idx_9 = (((c.m.r8((((p_set_3).wrapping_add(12)).wrapping_add((((l_g_6) as u32)).wrapping_mul(16))).wrapping_add(8))) as i32)).wrapping_add(l_p_7);
                                l_skip_10 = 1i32;
                                {
                                    l_i_11 = 0i32;
                                    'l5: loop {
                                        if !(((((l_i_11) < ((c.m.r32((l_w_5).wrapping_add(1412)) as i32))) as i32)) != 0) { break 'l5; }
                                        'c6: {
                                            if ((((((({ let a_ = ((c.m.r32((p_set_3).wrapping_add(8))).wrapping_add((((l_idx_9) as u32)).wrapping_mul(64))).wrapping_add(8); c.m.r8(a_) }) as i32)) != ((c.m.r32(((l_w_5).wrapping_add(32)).wrapping_add((((l_i_11) as u32)).wrapping_mul(8))) as i32))) as i32)) != 0) {
                                                break 'c6;
                                            }
                                            if ((((((({ let a_ = ((c.m.r32((p_set_3).wrapping_add(8))).wrapping_add((((l_idx_9) as u32)).wrapping_mul(64))).wrapping_add(10); c.m.r8(a_) }) as i32)) == ((c.m.r32((((l_w_5).wrapping_add(32)).wrapping_add((((l_i_11) as u32)).wrapping_mul(8))).wrapping_add(4)) as i32))) as i32)) != 0) {
                                                l_skip_10 = 0i32;
                                                break 'l5;
                                            }
                                            if ((((((({ let a_ = ((c.m.r32((p_set_3).wrapping_add(8))).wrapping_add((((l_idx_9) as u32)).wrapping_mul(64))).wrapping_add(10); c.m.r8(a_) }) as i32)) == (5i32)) as i32)) != 0) {
                                                l_skip_10 = 0i32;
                                                break 'l5;
                                            }
                                        }
                                        { let o_ = l_i_11; let n_ = (o_).wrapping_add(1); l_i_11 = n_; };
                                    }
                                }
                                if ((l_skip_10) != 0) {
                                    break 'c4;
                                }
                                if (((((p_reset_4) == (0i32)) as i32)) != 0) {
                                    l_f_12 = { let a5_ = (((c.m.r32((l_w_5).wrapping_add(1408)) as i32)) & (2i32)); let a6_ = (((c.m.r32((l_w_5).wrapping_add(1408)) as i32)) & (8i32)); F::EftEmit_GetFlagsFromReq(c, p_set_3, (l_w_5).wrapping_add(656), p_objId_1, l_g_6, l_p_7, a5_, a6_) };
                                } else {
                                    l_f_12 = F::EftEmit_GetResetFlags(c, p_set_3, (l_w_5).wrapping_add(656), l_g_6, l_p_7);
                                }
                                if (((((l_f_12) != (0i32)) as i32)) != 0) {
                                    { let a8_ = F32(c.m.r32((l_w_5).wrapping_add(1420))); F::EftEmit_SpawnOwn(c, p_set_3, (l_w_5).wrapping_add(656), (l_w_5).wrapping_add(64), ((l_w_5).wrapping_add(1440)).wrapping_add(16), (l_w_5).wrapping_add(1376), l_g_6, l_p_7, l_f_12, a8_) };
                                }
                            }
                            { let o_ = l_p_7; let n_ = (o_).wrapping_add(1); l_p_7 = n_; };
                        }
                    }
                }
            }
            { let o_ = l_g_6; let n_ = (o_).wrapping_add(1); l_g_6 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBlastObj_UpdateParts", &PROBE_N, &[], true); }
    r_
}

pub fn EftBlastObj_Init(c: &mut Ctx, mut p_task_1: u32, p_arg_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBlastObj_Init", &PROBE_N, &[(p_task_1) as u32, (p_arg_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(64) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 64);
    let mut l_w_3: u32 = 0;
    let mut l_src_4: u32 = 0;
    let mut l_slot_5: i32 = 0;
    let mut l_node_6: i32 = 0;
    c.m.w32(fp, p_arg_2);
    let r_ = 'f: {
        l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
        { let _ = fp; };
        { let _ = F::memset(c, l_w_3, 0i32, 1568u32); };
        { let v_ = { let a_ = (c.m.r32(fp)).wrapping_add(4); c.m.r32(a_) }; let a_ = (l_w_3).wrapping_add(48); c.m.w32(a_, v_); };
        { let v_ = { l_src_4 = { let a_ = c.m.r32(fp); c.m.r32(a_) }; l_src_4 }; let a_ = l_w_3; c.m.w32(a_, v_); };
        { let v_ = { let a_ = (c.m.r32(fp)).wrapping_add(12); c.m.r32(a_) }; let a_ = (l_w_3).wrapping_add(4); c.m.w32(a_, v_); };
        { let v_ = { let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }; let a_ = (l_w_3).wrapping_add(8); c.m.w32(a_, v_); };
        { let v_ = { let a_ = (c.m.r32(fp)).wrapping_add(20); (c.m.r32(a_) as i32) }; let a_ = (l_w_3).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        { let v_ = { let a_ = (c.m.r32(fp)).wrapping_add(24); (c.m.r32(a_) as i32) }; let a_ = (l_w_3).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
        { let v_ = { let a_ = (c.m.r32(fp)).wrapping_add(36); F32(c.m.r32(a_)) }; let a_ = (l_w_3).wrapping_add(20); c.m.w32(a_, (v_).0); };
        { let v_ = { let a_ = (c.m.r32(fp)).wrapping_add(40); F32(c.m.r32(a_)) }; let a_ = (l_w_3).wrapping_add(24); c.m.w32(a_, (v_).0); };
        { let v_ = { let a_ = (c.m.r32(fp)).wrapping_add(44); F32(c.m.r32(a_)) }; let a_ = (l_w_3).wrapping_add(28); c.m.w32(a_, (v_).0); };
        { let v_ = { let a_ = (c.m.r32(fp)).wrapping_add(28); (c.m.r32(a_) as i32) }; let a_ = ((l_w_3).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w32(a_, (v_) as u32); };
        { let v_ = { let a_ = (c.m.r32(fp)).wrapping_add(32); (c.m.r32(a_) as i32) }; let a_ = (((l_w_3).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(8))).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = 1i32; let a_ = (l_w_3).wrapping_add(1412); c.m.w32(a_, (v_) as u32); };
        {
            rt::memset(c, fp.wrapping_add(4), 0, 28);
            { let v_ = 0i32; c.m.w32(fp.wrapping_add(4).wrapping_add(0), (v_) as u32); }
            { let v_ = 1i32; c.m.w32(fp.wrapping_add(4).wrapping_add(4), (v_) as u32); }
            { let v_ = 3i32; c.m.w32(fp.wrapping_add(4).wrapping_add(8), (v_) as u32); }
            { let v_ = 4i32; c.m.w32(fp.wrapping_add(4).wrapping_add(12), (v_) as u32); }
            { let v_ = 5i32; c.m.w32(fp.wrapping_add(4).wrapping_add(16), (v_) as u32); }
            { let v_ = (1i32).wrapping_neg(); c.m.w32(fp.wrapping_add(4).wrapping_add(20), (v_) as u32); }
            { let v_ = (1i32).wrapping_neg(); c.m.w32(fp.wrapping_add(4).wrapping_add(24), (v_) as u32); }
            rt::memset(c, fp.wrapping_add(32), 0, 24);
            { let v_ = 2i32; c.m.w32(fp.wrapping_add(32).wrapping_add(0), (v_) as u32); }
            { let v_ = 4i32; c.m.w32(fp.wrapping_add(32).wrapping_add(4), (v_) as u32); }
            { let v_ = 8i32; c.m.w32(fp.wrapping_add(32).wrapping_add(8), (v_) as u32); }
            { let v_ = 16i32; c.m.w32(fp.wrapping_add(32).wrapping_add(12), (v_) as u32); }
            { let v_ = 32i32; c.m.w32(fp.wrapping_add(32).wrapping_add(16), (v_) as u32); }
            { let v_ = 64i32; c.m.w32(fp.wrapping_add(32).wrapping_add(20), (v_) as u32); }
            l_slot_5 = { let a_ = (fp.wrapping_add(4)).wrapping_add(((((c.m.r32((((l_w_3).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(8))).wrapping_add(4)) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
            { let s_ = { let a_ = (c.m.r32(fp)).wrapping_add(8); c.m.r32(a_) }; let d_ = (l_w_3).wrapping_add(64); rt::memcpy(c, d_, s_, 592); };
            if (((((l_slot_5) >= (0i32)) as i32)) != 0) {
                l_node_6 = { let a0_ = (c.m.r32(l_src_4) as i32); let a1_ = (((c.m.r32((fp.wrapping_add(32)).wrapping_add((((l_slot_5) as u32)).wrapping_mul(4))) as i32)) as u64); F::EftShot_GetAttrKind(c, a0_, a1_) };
                { let v_ = l_node_6; let a_ = ((((l_w_3).wrapping_add(64)).wrapping_add(16)).wrapping_add((((l_slot_5) as u32)).wrapping_mul(64))).wrapping_add((((0i32) as u32)).wrapping_mul(32)); c.m.w32(a_, (v_) as u32); };
                { let a0_ = (c.m.r32(l_src_4) as i32); F::BtlCharApi_GetNodePos(c, a0_, l_node_6, (((((l_w_3).wrapping_add(64)).wrapping_add(16)).wrapping_add((((l_slot_5) as u32)).wrapping_mul(64))).wrapping_add((((0i32) as u32)).wrapping_mul(32))).wrapping_add(16)) };
            }
        }
        { let r_ = 1i32; let a_ = (l_w_3).wrapping_add(1408); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        if (((rt::fgt(F32(c.m.r32((l_w_3).wrapping_add(20))), F32(0x00000000)) as i32)) != 0) {
            { let r_ = 1024i32; let a_ = (l_w_3).wrapping_add(1408); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            { let v_ = rt::fmul(F32(c.m.r32((l_w_3).wrapping_add(20))), F32(0x41f00000)); let a_ = (l_w_3).wrapping_add(1416); c.m.w32(a_, (v_).0); };
        }
        { let v_ = F32(c.m.r32((l_w_3).wrapping_add(24))); let a_ = (l_w_3).wrapping_add(1420); c.m.w32(a_, (v_).0); };
        { let a1_ = c.m.r32((l_w_3).wrapping_add(4)); F::Vec4_Copy(c, (l_w_3).wrapping_add(1440), a1_) };
        F::Vec4_Copy(c, ((l_w_3).wrapping_add(1440)).wrapping_add(16), (l_w_3).wrapping_add(1440));
        { let a1_ = c.m.r32((l_w_3).wrapping_add(8)); F::Vec4_Copy(c, (l_w_3).wrapping_add(1376), a1_) };
        { let a0_ = (c.m.r32(l_src_4) as i32); F::BtlCharApi_GetNodePos(c, a0_, 17i32, ((l_w_3).wrapping_add(1440)).wrapping_add(32)) };
        { let a2_ = F32(c.m.r32((l_w_3).wrapping_add(28))); F::Vec3_Scale(c, ((l_w_3).wrapping_add(1440)).wrapping_add(48), (l_w_3).wrapping_add(1376), a2_) };
        { let v_ = F32(0x3f800000); let a_ = (l_w_3).wrapping_add(1424); c.m.w32(a_, (v_).0); };
        { let a0_ = c.m.r32((l_w_3).wrapping_add(48)); F::EftEmit_InitState(c, a0_, (l_w_3).wrapping_add(656)) };
        if ((((({ let a_ = (c.m.r32(fp)).wrapping_add(48); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
            { let r_ = 4096i32; let a_ = (l_w_3).wrapping_add(1408); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            { let v_ = { let a_ = (c.m.r32(fp)).wrapping_add(48); c.m.r32(a_) }; let a_ = (l_w_3).wrapping_add(1504); c.m.w32(a_, v_); };
            F::EftBlastObj_InitModel(c, p_task_1);
        }
        { let a1_ = (if ((((((c.m.r32(l_src_4) as i32)) == (0i32)) as i32)) != 0) { 2048i32 } else { 4096i32 }); F::BtlTask_SetOwnerTag(c, p_task_1, a1_) };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftBlastObj_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftBlastObj_Term(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBlastObj_Term", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    if (((((c.m.r32((l_w_2).wrapping_add(1408)) as i32)) & (4096i32))) != 0) {
        F::EftBlastObj_TermModel(c, p_task_1);
    }
    { let a0_ = c.m.r32((l_w_2).wrapping_add(48)); F::EftEmit_TermState(c, a0_, (l_w_2).wrapping_add(656)) };
    { let v_ = 0i32; let a_ = (l_w_2).wrapping_add(1408); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBlastObj_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftBlastObj_Update(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBlastObj_Update", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let mut l_src_3: u32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    l_src_3 = c.m.r32(l_w_2);
    if (({ let a0_ = (c.m.r32(l_src_3) as i32); F::BtlScene_IsCharStopped(c, a0_) }) != 0) {
        break 'f;
    }
    if (((((((((c.m.r32((l_w_2).wrapping_add(1408)) as i32)) & (8192i32))) != 0) && (({ let a0_ = (c.m.r32(l_src_3) as i32); F::EftShot_TestBits(c, a0_, 4i32) }) != 0)) as i32)) != 0) {
        { let r_ = 16384i32; let a_ = (l_w_2).wrapping_add(1408); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
    F::EftEmit_UpdateNodesReq(c, l_src_3, (l_w_2).wrapping_add(64));
    if (((((((rt::fle(F32(c.m.r32((l_w_2).wrapping_add(1424))), F32(0x00000000)) as i32)) != 0) && ((((!(((({ let a_ = (c.m.r32((l_src_3).wrapping_add(36))).wrapping_add(60); (c.m.r32(a_) as i32) }) & (2i32))) != 0)) as i32)) != 0)) as i32)) != 0) {
        if ((((!(((((c.m.r32((l_w_2).wrapping_add(1408)) as i32)) & (32i32))) != 0)) as i32)) != 0) {
            if ((((!(((((c.m.r32((l_w_2).wrapping_add(1408)) as i32)) & (64i32))) != 0)) as i32)) != 0) {
                F::Vec4_Copy(c, ((l_w_2).wrapping_add(1440)).wrapping_add(32), ((l_w_2).wrapping_add(1440)).wrapping_add(16));
                if (((((c.m.r32((l_w_2).wrapping_add(1408)) as i32)) & (512i32))) != 0) {
                    F::Vec4_Copy(c, ((l_w_2).wrapping_add(1440)).wrapping_add(16), (l_w_2).wrapping_add(1392));
                } else {
                    if (((rt::flt(F32(0x00000000), { let a_ = (c.m.r32((l_src_3).wrapping_add(36))).wrapping_add(56); F32(c.m.r32(a_)) }) as i32)) != 0) {
                        { let a3_ = (c.m.r32(l_src_3) as i32); let a4_ = F32(c.m.r32((l_w_2).wrapping_add(28))); let a5_ = { let a_ = (c.m.r32((l_src_3).wrapping_add(36))).wrapping_add(56); F32(c.m.r32(a_)) }; F::EftAim_Home(c, (l_w_2).wrapping_add(1376), ((l_w_2).wrapping_add(1440)).wrapping_add(16), (l_w_2).wrapping_add(1376), a3_, a4_, a5_) };
                        { let a2_ = F32(c.m.r32((l_w_2).wrapping_add(28))); F::Vec3_Scale(c, ((l_w_2).wrapping_add(1440)).wrapping_add(48), (l_w_2).wrapping_add(1376), a2_) };
                    }
                    F::Vec3_Add(c, ((l_w_2).wrapping_add(1440)).wrapping_add(16), ((l_w_2).wrapping_add(1440)).wrapping_add(16), ((l_w_2).wrapping_add(1440)).wrapping_add(48));
                }
            }
        }
    }
    if (((rt::flt(F32(0x00000000), F32(c.m.r32((l_w_2).wrapping_add(1424)))) as i32)) != 0) {
        { let r_ = F32(0x3f800000); let a_ = (l_w_2).wrapping_add(1424); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
    }
    if (((((c.m.r32((l_w_2).wrapping_add(1408)) as i32)) & (1024i32))) != 0) {
        { let r_ = F32(0x3f800000); let a_ = (l_w_2).wrapping_add(1416); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        if (((rt::fle(F32(c.m.r32((l_w_2).wrapping_add(1416))), F32(0x00000000)) as i32)) != 0) {
            { let r_ = 2i32; let a_ = (l_w_2).wrapping_add(1408); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        }
    }
    if (({ let a0_ = (c.m.r32(l_src_3) as i32); F::EftShot_TestBits(c, a0_, 1024i32) }) != 0) {
        { let r_ = 2i32; let a_ = (l_w_2).wrapping_add(1408); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
    if (((((c.m.r32((l_w_2).wrapping_add(1408)) as i32)) & (4096i32))) != 0) {
        F::EftBlastObj_UpdateModel(c, p_task_1);
    }
    { let a0_ = (c.m.r32(l_src_3) as i32); let a2_ = c.m.r32((l_w_2).wrapping_add(48)); F::EftBlastObj_UpdateParts(c, a0_, p_task_1, a2_, 0i32) };
    if (((((c.m.r32((l_w_2).wrapping_add(1408)) as i32)) & (4i32))) != 0) {
        F::BtlTask_SetDead(c, p_task_1);
    } else {
        if (((((c.m.r32((l_w_2).wrapping_add(1408)) as i32)) & (2i32))) != 0) {
            { let r_ = 4i32; let a_ = (l_w_2).wrapping_add(1408); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        } else {
            if ((((!(((((c.m.r32((l_w_2).wrapping_add(1408)) as i32)) & (128i32))) != 0)) as i32)) != 0) {
                { let a0_ = c.m.r32((l_w_2).wrapping_add(48)); F::EftEmit_UpdateTrailWidth(c, a0_, (l_w_2).wrapping_add(656)) };
                F::EftBlastObj_AddHit(c, p_task_1);
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBlastObj_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftBlastObj_PostUpdate(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBlastObj_PostUpdate", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    if (({ let a0_ = { let a_ = c.m.r32(l_w_2); (c.m.r32(a_) as i32) }; F::BtlScene_IsCharStopped(c, a0_) }) != 0) {
        break 'f;
    }
    { let _ = { let a0_ = c.m.r32((l_w_2).wrapping_add(48)); F::EftEmit_UpdateAlive(c, a0_, (l_w_2).wrapping_add(656)) }; };
    if ((((((((c.m.r16((p_task_1).wrapping_add(8))) as i32)) & (1i32))) as u16)) != 0) {
        F::Vec3_Copy(c, ((l_w_2).wrapping_add(1440)).wrapping_add(16), (p_task_1).wrapping_add(16));
        if ((((!(((((c.m.r32((l_w_2).wrapping_add(1408)) as i32)) & (32i32))) != 0)) as i32)) != 0) {
            { let r_ = 32i32; let a_ = (l_w_2).wrapping_add(1408); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        }
    }
    if ((((((c.m.r16((p_task_1).wrapping_add(8))) as i32)) & (4i32))) != 0) {
        { let r_ = 2i32; let a_ = (l_w_2).wrapping_add(1408); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBlastObj_PostUpdate", &PROBE_N, &[], true); }
    r_
}

pub fn EftBlastObj_Reset(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBlastObj_Reset", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    if ((((!(((((c.m.r32((l_w_2).wrapping_add(1408)) as i32)) & (16i32))) != 0)) as i32)) != 0) {
        { let r_ = 16i32; let a_ = (l_w_2).wrapping_add(1408); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        { let a0_ = c.m.r32((l_w_2).wrapping_add(48)); F::EftEmit_KillAll(c, a0_, (l_w_2).wrapping_add(656)) };
    }
    F::BtlTask_SetDead(c, p_task_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBlastObj_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn EftBlastObj_Draw(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBlastObj_Draw", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBlastObj_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn EftBlastObjMgr_Init(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBlastObjMgr_Init", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_mgr_2: u32 = 0;
    let r_ = 'f: {
    l_mgr_2 = { let a0_ = ((((F::BtlPool_GetCurrent(c)) as i64 as u64)) as u32); F::BtlPool_Alloc(c, ((((a0_) as u64)) as i32), 4i32) };
    { let v_ = l_mgr_2; let a_ = sym::gEftBlastObjMgr; c.m.w32(a_, v_); };
    { let v_ = 0i32; let a_ = l_mgr_2; c.m.w32(a_, (v_) as u32); };
    { let v_ = F::BtlTask_CreateChildList(c, p_task_1, ((((60i32) as i64 as u64)) as u32), ((((((1568u32) as i32)) as i64 as u64)) as u32)); let a_ = sym::gEftBlastObjList; c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBlastObjMgr_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftBlastObjMgr_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBlastObjMgr_Term", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let a0_ = ((((F::BtlPool_GetCurrent(c)) as i64 as u64)) as u32); let a1_ = c.m.r32(sym::gEftBlastObjMgr); F::BtlPool_Free(c, ((((a0_) as u64)) as i32), a1_) };
    { let v_ = ((0i32) as u32); let a_ = sym::gEftBlastObjMgr; c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBlastObjMgr_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftBlastObjMgr_Update(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBlastObjMgr_Update", &PROBE_N, &[], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBlastObjMgr_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftBlastObjMgr_Reset(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBlastObjMgr_Reset", &PROBE_N, &[], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBlastObjMgr_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn EftBlastObj_Create(c: &mut Ctx, mut p_arg_1: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBlastObj_Create", &PROBE_N, &[(p_arg_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(64) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 64);
    let r_ = 'f: {
        { let s_ = p_arg_1; rt::memcpy(c, fp, s_, 52); }
        { let v_ = ((0i32) as u32); let a_ = (fp).wrapping_add(48); c.m.w32(a_, v_); };
        break 'f { let a0_ = c.m.r32(sym::gEftBlastObjList); F::BtlTaskList_AddTail(c, a0_, sym::gEftBlastObjClass, fp) };
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftBlastObj_Create", &PROBE_N, &[], true); }
    r_
}

pub fn EftBlastObj_CreateWithModel(c: &mut Ctx, mut p_arg_1: u32, mut p_model_2: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBlastObj_CreateWithModel", &PROBE_N, &[(p_arg_1) as u32, (p_model_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(64) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 64);
    let r_ = 'f: {
        { let s_ = p_arg_1; rt::memcpy(c, fp, s_, 52); }
        { let v_ = p_model_2; let a_ = (fp).wrapping_add(48); c.m.w32(a_, v_); };
        break 'f { let a0_ = c.m.r32(sym::gEftBlastObjList); F::BtlTaskList_AddTail(c, a0_, sym::gEftBlastObjClass, fp) };
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftBlastObj_CreateWithModel", &PROBE_N, &[], true); }
    r_
}

pub fn EftBlastObj_Stop(c: &mut Ctx, mut p_task_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBlastObj_Stop", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) != (fid::EftBlastObj_Update)) as i32)) != 0) {
        break 'f 0i32;
    }
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    if ((((!(((((c.m.r32((l_w_2).wrapping_add(1408)) as i32)) & (1i32))) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    { let r_ = 2i32; let a_ = (l_w_2).wrapping_add(1408); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBlastObj_Stop", &PROBE_N, &[], true); }
    r_
}

pub fn EftBlastObj_Kill(c: &mut Ctx, mut p_task_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBlastObj_Kill", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) != (fid::EftBlastObj_Update)) as i32)) != 0) {
        break 'f 0i32;
    }
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    if ((((!(((((c.m.r32((l_w_2).wrapping_add(1408)) as i32)) & (1i32))) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    { let r_ = 4i32; let a_ = (l_w_2).wrapping_add(1408); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    if ((((!(((((c.m.r32((l_w_2).wrapping_add(1408)) as i32)) & (16i32))) != 0)) as i32)) != 0) {
        { let r_ = 16i32; let a_ = (l_w_2).wrapping_add(1408); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        { let a0_ = c.m.r32((l_w_2).wrapping_add(48)); F::EftEmit_KillAll(c, a0_, (l_w_2).wrapping_add(656)) };
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBlastObj_Kill", &PROBE_N, &[], true); }
    r_
}

pub fn EftBlastObj_IsAlive(c: &mut Ctx, mut p_task_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBlastObj_IsAlive", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) != (fid::EftBlastObj_Update)) as i32)) != 0) {
        break 'f 0i32;
    }
    if (((({ let a_ = (c.m.r32((p_task_1).wrapping_add(56))).wrapping_add(1408); (c.m.r32(a_) as i32) }) & (1i32))) != 0) {
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBlastObj_IsAlive", &PROBE_N, &[], true); }
    r_
}

pub fn EftBlastObj_IsDying(c: &mut Ctx, mut p_task_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBlastObj_IsDying", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) != (fid::EftBlastObj_Update)) as i32)) != 0) {
        break 'f 0i32;
    }
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    if ((((!(((((c.m.r32((l_w_2).wrapping_add(1408)) as i32)) & (1i32))) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    if (((((c.m.r32((l_w_2).wrapping_add(1408)) as i32)) & (4i32))) != 0) {
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBlastObj_IsDying", &PROBE_N, &[], true); }
    r_
}

pub fn EftBlastObj_SetSel(c: &mut Ctx, mut p_task_1: u32, mut p_n_2: i32, mut p_kind_3: i32, mut p_node_4: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBlastObj_SetSel", &PROBE_N, &[(p_task_1) as u32, (p_n_2) as u32, (p_kind_3) as u32, (p_node_4) as u32], false); }
    let mut l_w_5: u32 = 0;
    let r_ = 'f: {
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) != (fid::EftBlastObj_Update)) as i32)) != 0) {
        break 'f 0i32;
    }
    l_w_5 = c.m.r32((p_task_1).wrapping_add(56));
    if ((((!(((((c.m.r32((l_w_5).wrapping_add(1408)) as i32)) & (1i32))) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    { let v_ = p_kind_3; let a_ = ((l_w_5).wrapping_add(32)).wrapping_add((((p_n_2) as u32)).wrapping_mul(8)); c.m.w32(a_, (v_) as u32); };
    { let v_ = p_node_4; let a_ = (((l_w_5).wrapping_add(32)).wrapping_add((((p_n_2) as u32)).wrapping_mul(8))).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
    if ((((!(((((p_n_2) < ((c.m.r32((l_w_5).wrapping_add(1412)) as i32))) as i32)) != 0)) as i32)) != 0) {
        { let a_ = (l_w_5).wrapping_add(1412); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBlastObj_SetSel", &PROBE_N, &[], true); }
    r_
}

pub fn EftBlastObj_GetPose(c: &mut Ctx, mut p_task_1: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBlastObj_GetPose", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f ((0i32) as u32);
    }
    if ((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) != (fid::EftBlastObj_Update)) as i32)) != 0) {
        break 'f ((0i32) as u32);
    }
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    if ((((!(((((c.m.r32((l_w_2).wrapping_add(1408)) as i32)) & (1i32))) != 0)) as i32)) != 0) {
        break 'f ((0i32) as u32);
    }
    break 'f (l_w_2).wrapping_add(1440);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBlastObj_GetPose", &PROBE_N, &[], true); }
    r_
}

pub fn EftBlastObj_SetTarget(c: &mut Ctx, mut p_task_1: u32, mut p_on_2: i32, mut p_pos_3: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBlastObj_SetTarget", &PROBE_N, &[(p_task_1) as u32, (p_on_2) as u32, (p_pos_3) as u32], false); }
    let mut l_w_4: u32 = 0;
    let r_ = 'f: {
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) != (fid::EftBlastObj_Update)) as i32)) != 0) {
        break 'f 0i32;
    }
    l_w_4 = c.m.r32((p_task_1).wrapping_add(56));
    if ((((!(((((c.m.r32((l_w_4).wrapping_add(1408)) as i32)) & (1i32))) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((p_on_2) != 0) {
        { let r_ = 512i32; let a_ = (l_w_4).wrapping_add(1408); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        F::Vec4_Copy(c, (l_w_4).wrapping_add(1392), p_pos_3);
    } else {
        { let r_ = (!(512i32)); let a_ = (l_w_4).wrapping_add(1408); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBlastObj_SetTarget", &PROBE_N, &[], true); }
    r_
}

pub fn EftBlastObj_SetPrevPos(c: &mut Ctx, mut p_task_1: u32, mut p_pos_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBlastObj_SetPrevPos", &PROBE_N, &[(p_task_1) as u32, (p_pos_2) as u32], false); }
    let mut l_w_3: u32 = 0;
    let r_ = 'f: {
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) != (fid::EftBlastObj_Update)) as i32)) != 0) {
        break 'f 0i32;
    }
    l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
    if ((((!(((((c.m.r32((l_w_3).wrapping_add(1408)) as i32)) & (1i32))) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    F::Vec4_Copy(c, ((l_w_3).wrapping_add(1440)).wrapping_add(32), p_pos_2);
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBlastObj_SetPrevPos", &PROBE_N, &[], true); }
    r_
}

pub fn EftBlastObj_SetDir(c: &mut Ctx, mut p_task_1: u32, mut p_dir_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBlastObj_SetDir", &PROBE_N, &[(p_task_1) as u32, (p_dir_2) as u32], false); }
    let mut l_w_3: u32 = 0;
    let r_ = 'f: {
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) != (fid::EftBlastObj_Update)) as i32)) != 0) {
        break 'f 0i32;
    }
    l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
    if ((((!(((((c.m.r32((l_w_3).wrapping_add(1408)) as i32)) & (1i32))) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    F::Vec4_Copy(c, (l_w_3).wrapping_add(1376), p_dir_2);
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBlastObj_SetDir", &PROBE_N, &[], true); }
    r_
}

pub fn EftBlastObj_SetFrozen(c: &mut Ctx, mut p_task_1: u32, mut p_on_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBlastObj_SetFrozen", &PROBE_N, &[(p_task_1) as u32, (p_on_2) as u32], false); }
    let mut l_w_3: u32 = 0;
    let r_ = 'f: {
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) != (fid::EftBlastObj_Update)) as i32)) != 0) {
        break 'f 0i32;
    }
    l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
    if ((((!(((((c.m.r32((l_w_3).wrapping_add(1408)) as i32)) & (1i32))) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((p_on_2) != 0) {
        { let r_ = 64i32; let a_ = (l_w_3).wrapping_add(1408); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    } else {
        { let r_ = (!(64i32)); let a_ = (l_w_3).wrapping_add(1408); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBlastObj_SetFrozen", &PROBE_N, &[], true); }
    r_
}

pub fn EftBlastObj_SetNoHit(c: &mut Ctx, mut p_task_1: u32, mut p_on_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBlastObj_SetNoHit", &PROBE_N, &[(p_task_1) as u32, (p_on_2) as u32], false); }
    let mut l_w_3: u32 = 0;
    let r_ = 'f: {
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) != (fid::EftBlastObj_Update)) as i32)) != 0) {
        break 'f 0i32;
    }
    l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
    if ((((!(((((c.m.r32((l_w_3).wrapping_add(1408)) as i32)) & (1i32))) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((p_on_2) != 0) {
        { let r_ = 128i32; let a_ = (l_w_3).wrapping_add(1408); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    } else {
        { let r_ = (!(128i32)); let a_ = (l_w_3).wrapping_add(1408); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBlastObj_SetNoHit", &PROBE_N, &[], true); }
    r_
}

pub fn EftBlastObj_SetHeld(c: &mut Ctx, mut p_task_1: u32, mut p_on_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBlastObj_SetHeld", &PROBE_N, &[(p_task_1) as u32, (p_on_2) as u32], false); }
    let mut l_w_3: u32 = 0;
    let r_ = 'f: {
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) != (fid::EftBlastObj_Update)) as i32)) != 0) {
        break 'f 0i32;
    }
    l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
    if ((((!(((((c.m.r32((l_w_3).wrapping_add(1408)) as i32)) & (1i32))) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((p_on_2) != 0) {
        { let r_ = 256i32; let a_ = (l_w_3).wrapping_add(1408); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    } else {
        { let r_ = (!(256i32)); let a_ = (l_w_3).wrapping_add(1408); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBlastObj_SetHeld", &PROBE_N, &[], true); }
    r_
}

pub fn EftBlastObj_SetModelAnim(c: &mut Ctx, mut p_task_1: u32, mut p_on_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBlastObj_SetModelAnim", &PROBE_N, &[(p_task_1) as u32, (p_on_2) as u32], false); }
    let mut l_w_3: u32 = 0;
    let r_ = 'f: {
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) != (fid::EftBlastObj_Update)) as i32)) != 0) {
        break 'f 0i32;
    }
    l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
    if ((((!(((((c.m.r32((l_w_3).wrapping_add(1408)) as i32)) & (1i32))) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((p_on_2) != 0) {
        { let r_ = 8192i32; let a_ = (l_w_3).wrapping_add(1408); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    } else {
        { let r_ = (!(8192i32)); let a_ = (l_w_3).wrapping_add(1408); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBlastObj_SetModelAnim", &PROBE_N, &[], true); }
    r_
}

pub fn EftBlastObj_MarkLast(c: &mut Ctx, mut p_task_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBlastObj_MarkLast", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) != (fid::EftBlastObj_Update)) as i32)) != 0) {
        break 'f 0i32;
    }
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    if ((((!(((((c.m.r32((l_w_2).wrapping_add(1408)) as i32)) & (1i32))) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    { let r_ = 2048i32; let a_ = (l_w_2).wrapping_add(1408); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBlastObj_MarkLast", &PROBE_N, &[], true); }
    r_
}

pub fn EftBlastObj_SetDelay(c: &mut Ctx, mut p_task_1: u32, mut p_frames_2: F32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftBlastObj_SetDelay", &PROBE_N, &[(p_task_1) as u32, (p_frames_2).0], false); }
    let mut l_w_3: u32 = 0;
    let r_ = 'f: {
    if (((((p_task_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((({ let a_ = (c.m.r32((p_task_1).wrapping_add(40))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) != (fid::EftBlastObj_Update)) as i32)) != 0) {
        break 'f 0i32;
    }
    l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
    if ((((!(((((c.m.r32((l_w_3).wrapping_add(1408)) as i32)) & (1i32))) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    { let v_ = p_frames_2; let a_ = (l_w_3).wrapping_add(1424); c.m.w32(a_, (v_).0); };
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftBlastObj_SetDelay", &PROBE_N, &[], true); }
    r_
}
