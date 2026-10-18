#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn BtlMove_UpdateAnimVoice(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMove_UpdateAnimVoice", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_kind_2: i32 = 0;
    let r_ = 'f: {
    l_kind_2 = (1i32).wrapping_neg();
    if ((F::BtlChar_TestFlag(c, p_chr_1, ((((303i32) as i64 as u64)) as u32))) != 0) {
        if ((F::BtlAnim_TestAttr(c, p_chr_1, ((32768i32) as u64))) != 0) {
            l_kind_2 = 14i32;
        }
    }
    if ((F::BtlChar_TestFlag(c, p_chr_1, ((((304i32) as i64 as u64)) as u32))) != 0) {
        if ((F::BtlAnim_TestAttr(c, p_chr_1, ((32768i32) as u64))) != 0) {
            l_kind_2 = 52i32;
        }
        if ((F::BtlAnim_TestAttr(c, p_chr_1, ((549755813888i64) as u64))) != 0) {
            l_kind_2 = 53i32;
        }
        if ((F::BtlAnim_TestAttr(c, p_chr_1, ((131072i32) as u64))) != 0) {
            l_kind_2 = 54i32;
        }
        if ((F::BtlAnim_TestAttr(c, p_chr_1, ((262144i32) as u64))) != 0) {
            F::BtlCharSnd_PlayStream(c, p_chr_1, 36149i32);
        }
    }
    if ((F::BtlChar_TestFlag(c, p_chr_1, ((((305i32) as i64 as u64)) as u32))) != 0) {
        if ((F::BtlAnim_TestAttr(c, p_chr_1, ((549755846656i64) as u64))) != 0) {
            l_kind_2 = 55i32;
        }
        if ((F::BtlAnim_TestAttr(c, p_chr_1, ((262144i32) as u64))) != 0) {
            F::BtlCharSnd_PlayStream(c, p_chr_1, 36151i32);
        }
    }
    if ((F::BtlChar_TestFlag(c, p_chr_1, ((((306i32) as i64 as u64)) as u32))) != 0) {
        if ((F::BtlAnim_TestAttr(c, p_chr_1, ((65536i32) as u64))) != 0) {
            F::BtlCharSnd_PlayStream(c, p_chr_1, 36162i32);
        }
    }
    if ((F::BtlChar_TestFlag(c, p_chr_1, ((((307i32) as i64 as u64)) as u32))) != 0) {
        if ((F::BtlAnim_TestAttr(c, p_chr_1, ((65536i32) as u64))) != 0) {
            F::BtlCharSnd_PlayStream(c, p_chr_1, 36161i32);
        }
    }
    if (((((l_kind_2) >= (0i32)) as i32)) != 0) {
        F::BtlChar_PlayVoice(c, p_chr_1, l_kind_2);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMove_UpdateAnimVoice", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMove_ResetAnimStep(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMove_ResetAnimStep", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    F::BtlAnim_SetUnkC8C(c, p_chr_1, F32(0x00000000));
    F::BtlAnim_SetStep(c, p_chr_1, F32(0x40000000));
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMove_ResetAnimStep", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMove_UpdateAction(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMove_UpdateAction", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    F::BtlMove_ResetAnimStep(c, p_chr_1);
    F::BtlAct_Update(c, p_chr_1);
    F::BtlAnim_ApplyBlend(c, p_chr_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMove_UpdateAction", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMove_TestContact(c: &mut Ctx, mut p_chr_1: u32, mut p_floor_2: i32, mut p_angle_3: F32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMove_TestContact", &PROBE_N, &[(p_chr_1) as u32, (p_floor_2) as u32, (p_angle_3).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_obj_4: u32 = 0;
    let mut l_pose_5: u32 = 0;
    let mut l_hit_6: u32 = 0;
    let mut l_i_7: i32 = 0;
    let mut l_found_8: i32 = 0;
    let mut l_d_9: F32 = F32(0);
    let r_ = 'f: {
        l_obj_4 = F::BtlChar_GetObj(c, p_chr_1);
        l_pose_5 = F::BtlChar_GetPos(c, p_chr_1);
        l_hit_6 = (c.m.r32((l_obj_4).wrapping_add(5728))).wrapping_add(((98400i32) as u32));
        {
            l_i_7 = 0i32;
            'l1: loop {
                if !(((((l_i_7) < (2i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_found_8 = 0i32;
                    's3: {
                        let sv10: i32 = l_i_7;
                        match sv10 {
                            0i32 => {
                                if ((F::BtlChar_TestFlag(c, p_chr_1, ((((15i32) as i64 as u64)) as u32))) != 0) {
                                    l_found_8 = 1i32;
                                    { let a1_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(192); F::Vec4_Copy(c, fp, a1_) };
                                }
                                break 's3;
                            }
                            1i32 => {
                                if ((((((F::BtlChar_TestFlag(c, p_chr_1, ((((31i32) as i64 as u64)) as u32))) != 0) && ((((!((F::BtlChar_TestFlag(c, p_chr_1, ((((32i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0)) as i32)) != 0) {
                                    l_found_8 = 1i32;
                                    F::Vec4_Copy(c, fp, (l_hit_6).wrapping_add(16));
                                }
                                break 's3;
                            }
                            _ => {
                            }
                        }
                    }
                    if ((l_found_8) != 0) {
                        if (((rt::fgt(F32(c.m.r32((fp).wrapping_add(4))), F32(0xbf34fdf3)) as i32)) != 0) {
                            if (((((p_floor_2) == (1i32)) as i32)) != 0) {
                                break 'c2;
                            }
                        } else {
                            if (((((p_floor_2) == (0i32)) as i32)) != 0) {
                                break 'c2;
                            }
                        }
                        if (((rt::feq(p_angle_3, F32(0xc0c90fda)) as i32)) != 0) {
                            break 'f 1i32;
                        }
                        l_d_9 = rt::fneg(F::Vec3_Dot(c, fp, (l_pose_5).wrapping_add(128)));
                        if (((rt::flt(F::Mathf_Cos(c, p_angle_3), l_d_9) as i32)) != 0) {
                            break 'f 1i32;
                        }
                    }
                }
                { let o_ = l_i_7; let n_ = (o_).wrapping_add(1); l_i_7 = n_; };
            }
        }
        break 'f 0i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlMove_TestContact", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMove_IsHeadingIntoFloor(c: &mut Ctx, mut p_chr_1: u32, mut p_angle_2: F32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMove_IsHeadingIntoFloor", &PROBE_N, &[(p_chr_1) as u32, (p_angle_2).0], false); }
    let r_ = 'f: {
    break 'f F::BtlMove_TestContact(c, p_chr_1, 1i32, p_angle_2);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMove_IsHeadingIntoFloor", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMove_IsHeadingIntoWall(c: &mut Ctx, mut p_chr_1: u32, mut p_angle_2: F32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMove_IsHeadingIntoWall", &PROBE_N, &[(p_chr_1) as u32, (p_angle_2).0], false); }
    let r_ = 'f: {
    break 'f F::BtlMove_TestContact(c, p_chr_1, 0i32, p_angle_2);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMove_IsHeadingIntoWall", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMove_CanFireBlast(c: &mut Ctx, mut p_chr_1: u32, mut p_mode_2: i32, mut p_outCount_3: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMove_CanFireBlast", &PROBE_N, &[(p_chr_1) as u32, (p_mode_2) as u32, (p_outCount_3) as u32], false); }
    let mut l_n_4: i32 = 0;
    let mut l_list_5: u32 = 0;
    let mut l_i_6: i32 = 0;
    let mut l_rec_7: u32 = 0;
    let mut l_def_8: u32 = 0;
    let r_ = 'f: {
    l_n_4 = 0i32;
    l_list_5 = F::EftHit_GetList(c);
    {
        l_i_6 = 0i32;
        'l1: loop {
            if !(((((l_i_6) < ((c.m.r32((l_list_5).wrapping_add(25600)) as i32))) as i32)) != 0) { break 'l1; }
            'c2: {
                l_rec_7 = (l_list_5).wrapping_add((((l_i_6) as u32)).wrapping_mul(400));
                if ((((((c.m.r32((l_rec_7).wrapping_add(12)) as i32)) != (0i32)) as i32)) != 0) {
                    break 'c2;
                }
                l_def_8 = c.m.r32((l_rec_7).wrapping_add(104));
                if (((((l_def_8) == (((0i32) as u32))) as i32)) != 0) {
                    break 'c2;
                }
                if ((((((((c.m.r16((l_def_8).wrapping_add(16)) as i16)) as i32)) != ((c.m.r32((p_chr_1).wrapping_add(12)) as i32))) as i32)) != 0) {
                    break 'c2;
                }
                if ((((((c.m.r32((l_rec_7).wrapping_add(84)) as i32)) == (1i32)) as i32)) != 0) {
                    break 'c2;
                }
                's3: {
                    let sv9: i32 = (((c.m.r16((l_def_8).wrapping_add(24)) as i16)) as i32);
                    match sv9 {
                        0i32 | 4i32 | 8i32 => {
                            if (((((p_mode_2) == (0i32)) as i32)) != 0) {
                                { let o_ = l_n_4; let n_ = (o_).wrapping_add(1); l_n_4 = n_; };
                            }
                            break 's3;
                        }
                        _ => {
                            if (((((p_mode_2) == (1i32)) as i32)) != 0) {
                                { let o_ = l_n_4; let n_ = (o_).wrapping_add(1); l_n_4 = n_; };
                            }
                            break 's3;
                        }
                    }
                }
            }
            { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; };
        }
    }
    if (((((p_outCount_3) != (((0i32) as u32))) as i32)) != 0) {
        { let v_ = l_n_4; let a_ = p_outCount_3; c.m.w32(a_, (v_) as u32); };
    }
    's4: {
        let sv10: i32 = p_mode_2;
        match sv10 {
            0i32 => {
                if (((((l_n_4) < (F::BtlParam_GetCount80(c, p_chr_1))) as i32)) != 0) {
                    break 'f 1i32;
                }
                break 's4;
            }
            1i32 => {
                if (((((l_n_4) < (F::BtlParam_GetUnk82(c, p_chr_1))) as i32)) != 0) {
                    break 'f 1i32;
                }
                break 's4;
            }
            _ => {
            }
        }
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMove_CanFireBlast", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMove_IsBlastIncoming(c: &mut Ctx, mut p_chr_1: u32, mut p_a_2: i32, mut p_b_3: i32, mut p_c_4: i32, mut p_limit_5: F32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMove_IsBlastIncoming", &PROBE_N, &[(p_chr_1) as u32, (p_a_2) as u32, (p_b_3) as u32, (p_c_4) as u32, (p_limit_5).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_list_6: u32 = 0;
    let mut l_i_7: i32 = 0;
    let mut l_rec_8: u32 = 0;
    let mut l_dist_9: F32 = F32(0);
    let mut l_speed_10: F32 = F32(0);
    let r_ = 'f: {
        l_list_6 = F::EftHit_GetList(c);
        {
            l_i_7 = 0i32;
            'l1: loop {
                if !(((((l_i_7) < ((c.m.r32((l_list_6).wrapping_add(25600)) as i32))) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_rec_8 = (l_list_6).wrapping_add((((l_i_7) as u32)).wrapping_mul(400));
                    if ((((((c.m.r32((l_rec_8).wrapping_add(84)) as i32)) == (1i32)) as i32)) != 0) {
                        break 'c2;
                    }
                    if ((((((c.m.r32((l_rec_8).wrapping_add(12)) as i32)) == (0i32)) as i32)) != 0) {
                        if (((((c.m.r32((l_rec_8).wrapping_add(104))) == (((0i32) as u32))) as i32)) != 0) {
                            break 'c2;
                        }
                        's3: {
                            let sv11: i32 = (({ let a_ = (c.m.r32((l_rec_8).wrapping_add(104))).wrapping_add(24); (c.m.r16(a_) as i16) }) as i32);
                            match sv11 {
                                0i32 | 4i32 | 8i32 => {
                                    if ((((!((p_a_2) != 0)) as i32)) != 0) {
                                        break 'c2;
                                    }
                                    break 's3;
                                }
                                _ => {
                                    if ((((!((p_b_3) != 0)) as i32)) != 0) {
                                        break 'c2;
                                    }
                                    break 's3;
                                }
                            }
                        }
                    } else {
                        if (((((c.m.r32((l_rec_8).wrapping_add(100))) == (((0i32) as u32))) as i32)) != 0) {
                            break 'c2;
                        }
                        's4: {
                            let sv12: i32 = { let a_ = (c.m.r32((l_rec_8).wrapping_add(100))).wrapping_add(4); (c.m.r32(a_) as i32) };
                            match sv12 {
                                0i32 | 1i32 => {
                                    break 'c2;
                                }
                                _ => {
                                    if ((((!((p_c_4) != 0)) as i32)) != 0) {
                                        break 'c2;
                                    }
                                    break 's4;
                                }
                            }
                        }
                    }
                    F::Vec4_Sub(c, fp, (l_rec_8).wrapping_add(32), (l_rec_8).wrapping_add(48));
                    { let a1_ = F::BtlChar_GetPos(c, p_chr_1); F::Vec4_Sub(c, fp.wrapping_add(16), a1_, (l_rec_8).wrapping_add(32)) };
                    if (((rt::flt(F::Vec3_Dot(c, fp.wrapping_add(16), fp), F32(0x00000000)) as i32)) != 0) {
                        break 'c2;
                    }
                    l_dist_9 = F::Vec3_Length(c, fp.wrapping_add(16));
                    l_speed_10 = F::Vec3_Length(c, fp);
                    if (((rt::flt(l_speed_10, F32(0x3a83126e)) as i32)) != 0) {
                        break 'c2;
                    }
                    if (((rt::flt(p_limit_5, rt::fdiv(l_dist_9, l_speed_10)) as i32)) != 0) {
                        break 'c2;
                    }
                    break 'f 1i32;
                }
                { let o_ = l_i_7; let n_ = (o_).wrapping_add(1); l_i_7 = n_; };
            }
        }
        break 'f 0i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlMove_IsBlastIncoming", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMove_CountOwnBeams(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMove_CountOwnBeams", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_list_2: u32 = 0;
    let mut l_n_3: i32 = 0;
    let mut l_i_4: i32 = 0;
    let mut l_rec_5: u32 = 0;
    let r_ = 'f: {
    l_list_2 = F::EftHit_GetList(c);
    l_n_3 = 0i32;
    {
        l_i_4 = 0i32;
        'l1: loop {
            if !(((((l_i_4) < ((c.m.r32((l_list_2).wrapping_add(25600)) as i32))) as i32)) != 0) { break 'l1; }
            'c2: {
                l_rec_5 = (l_list_2).wrapping_add((((l_i_4) as u32)).wrapping_mul(400));
                if ((((((((((((((c.m.r32((l_rec_5).wrapping_add(12)) as i32)) == (1i32)) as i32)) != 0) && (((((c.m.r32((l_rec_5).wrapping_add(100))) != (((0i32) as u32))) as i32)) != 0)) as i32)) != 0) && ((((({ let a_ = c.m.r32((l_rec_5).wrapping_add(100)); (c.m.r32(a_) as i32) }) == ((c.m.r32((p_chr_1).wrapping_add(12)) as i32))) as i32)) != 0)) as i32)) != 0) {
                    { let o_ = l_n_3; let n_ = (o_).wrapping_add(1); l_n_3 = n_; };
                }
            }
            { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
        }
    }
    break 'f l_n_3;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMove_CountOwnBeams", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMove_ClipToRadius(c: &mut Ctx, mut p_out_1: u32, mut p_from_2: u32, mut p_to_3: u32, mut p_radius_4: F32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMove_ClipToRadius", &PROBE_N, &[(p_out_1) as u32, (p_from_2) as u32, (p_to_3) as u32, (p_radius_4).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let mut l_a_5: F32 = F32(0);
    let mut l_b_6: F32 = F32(0);
    let mut l_c_7: F32 = F32(0);
    let mut l_disc_8: F32 = F32(0);
    let mut l_t_9: F32 = F32(0);
    let r_ = 'f: {
        F::Vec4_Copy(c, fp, p_from_2);
        F::Vec4_Copy(c, fp.wrapping_add(16), p_to_3);
        { let v_ = F32(0x00000000); let a_ = (fp.wrapping_add(16)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
        F::Vec4_Sub(c, fp.wrapping_add(32), fp.wrapping_add(16), fp);
        l_a_5 = F::Vec3_Dot(c, fp.wrapping_add(32), fp.wrapping_add(32));
        l_b_6 = rt::fmul(F32(0x40000000), F::Vec3_Dot(c, fp, fp.wrapping_add(32)));
        l_c_7 = rt::fsub(F::Vec3_Dot(c, fp, fp), rt::fmul(p_radius_4, p_radius_4));
        l_disc_8 = rt::fsub(rt::fmul(l_b_6, l_b_6), rt::fmul(rt::fmul(l_a_5, F32(0x40800000)), l_c_7));
        if (((rt::flt(l_disc_8, F32(0x00000000)) as i32)) != 0) {
            break 'f 0i32;
        }
        if (((rt::flt(l_a_5, F32(0x3a83126e)) as i32)) != 0) {
            break 'f 0i32;
        }
        l_t_9 = rt::fdiv(rt::fsub(F::Port_sqrtf(c, l_disc_8), l_b_6), rt::fadd(l_a_5, l_a_5));
        if (((((((rt::flt(l_t_9, F32(0x00000000)) as i32)) != 0) || (((rt::fgt(l_t_9, F32(0x3f800000)) as i32)) != 0)) as i32)) != 0) {
            break 'f 0i32;
        }
        F::Vec4_Lerp(c, p_out_1, p_to_3, p_from_2, l_t_9);
        break 'f 1i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlMove_ClipToRadius", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMove_TurnYaw(c: &mut Ctx, mut p_chr_1: u32, mut p_mode_2: i32, mut p_maxStep_3: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMove_TurnYaw", &PROBE_N, &[(p_chr_1) as u32, (p_mode_2) as u32, (p_maxStep_3).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_pose_4: u32 = 0;
    let mut l_target_5: F32 = F32(0);
    let mut l_d_6: F32 = F32(0);
    let mut l_x_7: F32 = F32(0);
    let mut l_y_8: F32 = F32(0);
    let mut l_cam_9: F32 = F32(0);
    let r_ = 'f: {
        l_pose_4 = F::BtlChar_GetPos(c, p_chr_1);
        l_target_5 = F32(c.m.r32((l_pose_4).wrapping_add(148)));
        's1: {
            let sv10: i32 = p_mode_2;
            match sv10 {
                0i32 => {
                    if ((F::BtlInput_IsHeld(c, p_chr_1, ((240i32) as u32))) != 0) {
                        l_x_7 = F::BtlInput_GetStickX(c, p_chr_1);
                        l_y_8 = F::BtlInput_GetStickY(c, p_chr_1);
                        l_cam_9 = F32(c.m.r32((p_chr_1).wrapping_add(1184)));
                        l_target_5 = { let a0_ = rt::fadd(F::Port_atan2f(c, l_x_7, rt::fneg(l_y_8)), l_cam_9); F::BtlUtil_WrapAngle(c, a0_) };
                    }
                    break 's1;
                }
                1i32 => {
                    l_target_5 = { let a0_ = rt::fadd(F32(c.m.r32((l_pose_4).wrapping_add(148))), rt::fmul(p_maxStep_3, F::BtlInput_GetStickX(c, p_chr_1))); F::BtlUtil_WrapAngle(c, a0_) };
                    break 's1;
                }
                2i32 | 4i32 => {
                    if ((F::BtlChar_TestFlag(c, p_chr_1, ((((5i32) as i64 as u64)) as u32))) != 0) {
                        l_target_5 = F::BtlOpp_GetYaw(c, p_chr_1);
                    }
                    break 's1;
                }
                3i32 => {
                    if ((F::BtlChar_TestFlag(c, p_chr_1, ((((5i32) as i64 as u64)) as u32))) != 0) {
                        l_target_5 = { let a0_ = rt::fadd(F::BtlOpp_GetYaw(c, p_chr_1), F32(0x40490fda)); F::BtlUtil_WrapAngle(c, a0_) };
                    }
                    break 's1;
                }
                9i32 => {
                    F::BtlOpp_GetTargetRot(c, p_chr_1, fp);
                    l_target_5 = F32(c.m.r32((fp).wrapping_add(4)));
                    break 's1;
                }
                7i32 => {
                    l_target_5 = { let a0_ = rt::fneg(F32(c.m.r32(l_pose_4))); let a1_ = rt::fneg(F32(c.m.r32((l_pose_4).wrapping_add(8)))); F::Port_atan2f(c, a0_, a1_) };
                    break 's1;
                }
                8i32 => {
                    l_target_5 = { let a0_ = F32(c.m.r32(l_pose_4)); let a1_ = F32(c.m.r32((l_pose_4).wrapping_add(8))); F::Port_atan2f(c, a0_, a1_) };
                    break 's1;
                }
                _ => {
                }
            }
        }
        l_d_6 = { let a0_ = rt::fsub(l_target_5, F32(c.m.r32((l_pose_4).wrapping_add(148)))); F::BtlUtil_WrapAngle(c, a0_) };
        if (((rt::flt(l_d_6, rt::fneg(p_maxStep_3)) as i32)) != 0) {
            l_d_6 = rt::fneg(p_maxStep_3);
        }
        if (((rt::fgt(l_d_6, p_maxStep_3) as i32)) != 0) {
            l_d_6 = p_maxStep_3;
        }
        { let v_ = { let a0_ = rt::fadd(F32(c.m.r32((l_pose_4).wrapping_add(148))), l_d_6); F::BtlUtil_WrapAngle(c, a0_) }; let a_ = (l_pose_4).wrapping_add(148); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (l_pose_4).wrapping_add(212); c.m.w32(a_, (v_).0); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlMove_TurnYaw", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMove_TurnPitch(c: &mut Ctx, mut p_chr_1: u32, mut p_mode_2: i32, mut p_maxStep_3: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMove_TurnPitch", &PROBE_N, &[(p_chr_1) as u32, (p_mode_2) as u32, (p_maxStep_3).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(96) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 96);
    let mut l_pose_4: u32 = 0;
    let mut l_target_5: F32 = F32(0);
    let mut l_d_6: F32 = F32(0);
    let mut l_eps_7: F32 = F32(0);
    let mut l_speed_8: F32 = F32(0);
    let mut l_oppSpeed_9: F32 = F32(0);
    let mut l_len_10: F32 = F32(0);
    let mut l_dist_11: F32 = F32(0);
    let mut l_closing_12: F32 = F32(0);
    let mut l_t_13: F32 = F32(0);
    let mut l_k_14: F32 = F32(0);
    let r_ = 'f: {
        l_pose_4 = F::BtlChar_GetPos(c, p_chr_1);
        l_target_5 = F32(c.m.r32((l_pose_4).wrapping_add(144)));
        's1: {
            let sv15: i32 = p_mode_2;
            match sv15 {
                0i32 => {
                    l_target_5 = { let a0_ = rt::fsub(F32(c.m.r32((l_pose_4).wrapping_add(144))), rt::fmul(p_maxStep_3, F::BtlInput_GetStickY(c, p_chr_1))); F::BtlUtil_ClampF(c, a0_, F32(0xbfc7c82b), F32(0x3fc7c82b)) };
                    break 's1;
                }
                5i32 => {
                    l_target_5 = F32(0x00000000);
                    break 's1;
                }
                1i32 | 2i32 | 3i32 | 4i32 => {
                    if ((F::BtlChar_TestFlag(c, p_chr_1, ((((5i32) as i64 as u64)) as u32))) != 0) {
                        F::BtlOpp_GetDelta(c, p_chr_1, fp);
                        { let r_ = F::BtlOpp_GetHalfHeightDiff(c, p_chr_1); let a_ = (fp).wrapping_add(4); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                        if (((((p_mode_2) == (4i32)) as i32)) != 0) {
                            l_len_10 = F::Vec3_Length(c, fp);
                            if (((rt::flt(l_len_10, F32(0x42480000)) as i32)) != 0) {
                                { let r_ = rt::fdiv(l_len_10, F32(0x42480000)); let a_ = (fp).wrapping_add(4); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                            }
                        }
                        if (((((p_mode_2) == (3i32)) as i32)) != 0) {
                            { let r_ = F32(c.m.r32((fp).wrapping_add(4))); let a_ = (fp).wrapping_add(4); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                        }
                        l_eps_7 = F32(0x3c23d70a);
                        { let a1_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(48); F::Vec4_Copy(c, fp.wrapping_add(16), a1_) };
                        F::BtlOpp_GetPoseVec30(c, p_chr_1, fp.wrapping_add(32));
                        l_speed_8 = F::Vec3_Length(c, fp.wrapping_add(16));
                        l_oppSpeed_9 = F::Vec3_Length(c, fp.wrapping_add(32));
                        if (((((((rt::flt(l_eps_7, l_speed_8) as i32)) != 0) && (((rt::flt(l_eps_7, l_oppSpeed_9) as i32)) != 0)) as i32)) != 0) {
                            F::Vec3_Normalize(c, fp.wrapping_add(48), fp.wrapping_add(16));
                            F::Vec3_Normalize(c, fp.wrapping_add(64), fp.wrapping_add(32));
                            F::Vec3_Normalize(c, fp.wrapping_add(80), fp);
                            l_dist_11 = F::Vec3_Length(c, fp);
                            { let r_ = rt::fmul(rt::fsub(F32(0x3f800000), { let a0_ = F32(c.m.r32((fp.wrapping_add(80)).wrapping_add(4))); rt::ffabs(a0_) }), { let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); F::BtlCharApi_GetRadius(c, a0_) }); let n_ = rt::fsub(l_dist_11, r_); l_dist_11 = n_; };
                            { let r_ = rt::fmul(rt::fsub(F32(0x3f800000), { let a0_ = F32(c.m.r32((fp.wrapping_add(80)).wrapping_add(4))); rt::ffabs(a0_) }), F::BtlOpp_GetRadius(c, p_chr_1)); let n_ = rt::fsub(l_dist_11, r_); l_dist_11 = n_; };
                            if (((rt::flt(l_dist_11, F32(0x00000000)) as i32)) != 0) {
                                l_dist_11 = F32(0x00000000);
                            }
                            l_closing_12 = rt::fsub(l_speed_8, rt::fmul(l_oppSpeed_9, F::Vec3_Dot(c, fp.wrapping_add(48), fp.wrapping_add(64))));
                            if (((rt::flt(l_eps_7, l_closing_12) as i32)) != 0) {
                                l_t_13 = rt::fdiv(l_dist_11, l_closing_12);
                                if (((rt::flt(l_t_13, F32(0x3f800000)) as i32)) != 0) {
                                    { let r_ = l_t_13; let a_ = (fp).wrapping_add(4); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                                }
                            }
                        }
                        F::Vec3_Normalize(c, fp.wrapping_add(48), fp);
                        if (((rt::flt(F32(c.m.r32((fp.wrapping_add(48)).wrapping_add(4))), F32(0xbf800000)) as i32)) != 0) {
                            { let v_ = F32(0xbf800000); let a_ = (fp.wrapping_add(48)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                        }
                        if (((rt::fgt(F32(c.m.r32((fp.wrapping_add(48)).wrapping_add(4))), F32(0x3f800000)) as i32)) != 0) {
                            { let v_ = F32(0x3f800000); let a_ = (fp.wrapping_add(48)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                        }
                        l_target_5 = rt::fneg({ let a0_ = F32(c.m.r32((fp.wrapping_add(48)).wrapping_add(4))); F::Mathf_Asin(c, a0_) });
                        if (((((p_mode_2) == (2i32)) as i32)) != 0) {
                            l_k_14 = rt::fneg(F::BtlInput_GetStickY(c, p_chr_1));
                            if (((rt::flt(l_k_14, F32(0x00000000)) as i32)) != 0) {
                                l_k_14 = F32(0x00000000);
                            }
                            { let r_ = l_k_14; let n_ = rt::fmul(l_target_5, r_); l_target_5 = n_; };
                        }
                    }
                    break 's1;
                }
                6i32 => {
                    break 's1;
                }
                _ => {
                }
            }
        }
        l_d_6 = { let a0_ = rt::fsub(l_target_5, F32(c.m.r32((l_pose_4).wrapping_add(144)))); F::BtlUtil_WrapAngle(c, a0_) };
        if (((rt::flt(l_d_6, rt::fneg(p_maxStep_3)) as i32)) != 0) {
            l_d_6 = rt::fneg(p_maxStep_3);
        }
        if (((rt::fgt(l_d_6, p_maxStep_3) as i32)) != 0) {
            l_d_6 = p_maxStep_3;
        }
        { let v_ = { let a0_ = rt::fadd(F32(c.m.r32((l_pose_4).wrapping_add(144))), l_d_6); F::BtlUtil_WrapAngle(c, a0_) }; let a_ = (l_pose_4).wrapping_add(144); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (l_pose_4).wrapping_add(216); c.m.w32(a_, (v_).0); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlMove_TurnPitch", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMove_SteerAtOpponent(c: &mut Ctx, mut p_chr_1: u32, mut p_closeSpeed_2: F32, mut p_yawAccel_3: F32, mut p_pitchAccel_4: F32, mut p_yawMax_5: F32, mut p_pitchMax_6: F32, mut p_maxStep_7: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMove_SteerAtOpponent", &PROBE_N, &[(p_chr_1) as u32, (p_closeSpeed_2).0, (p_yawAccel_3).0, (p_pitchAccel_4).0, (p_yawMax_5).0, (p_pitchMax_6).0, (p_maxStep_7).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(64) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 64);
    let mut l_pose_8: u32 = 0;
    let mut l_dist_9: F32 = F32(0);
    let mut l_speed_10: F32 = F32(0);
    let mut l_oppSpeed_11: F32 = F32(0);
    let mut l_t_12: F32 = F32(0);
    let mut l_k_13: F32 = F32(0);
    let mut l_k2_14: F32 = F32(0);
    let mut l_goal2_15: F32 = F32(0);
    let mut l_cur_16: F32 = F32(0);
    let mut l_diff_17: F32 = F32(0);
    let mut l_goal_18: F32 = F32(0);
    let mut l_base_19: F32 = F32(0);
    let mut l_step_20: F32 = F32(0);
    let r_ = 'f: {
        l_pose_8 = F::BtlChar_GetPos(c, p_chr_1);
        if ((((!((F::BtlChar_TestFlag(c, p_chr_1, ((((5i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) {
            F::BtlMove_TurnYaw(c, p_chr_1, 6i32, p_maxStep_7);
            F::BtlMove_TurnPitch(c, p_chr_1, 6i32, p_maxStep_7);
            break 'f;
        }
        F::BtlOpp_GetDelta(c, p_chr_1, fp);
        { let r_ = F::BtlOpp_GetHalfHeightDiff(c, p_chr_1); let a_ = (fp).wrapping_add(4); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        l_dist_9 = F::Vec3_Length(c, fp);
        { let r_ = rt::fmul({ let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); F::BtlCharApi_GetRadius(c, a0_) }, F32(0x3f000000)); let n_ = rt::fsub(l_dist_9, r_); l_dist_9 = n_; };
        { let r_ = rt::fmul(F::BtlOpp_GetRadius(c, p_chr_1), F32(0x3f000000)); let n_ = rt::fsub(l_dist_9, r_); l_dist_9 = n_; };
        if (((rt::flt(l_dist_9, F32(0x00000000)) as i32)) != 0) {
            l_dist_9 = F32(0x00000000);
        }
        F::Vec4_Copy(c, fp.wrapping_add(16), (l_pose_8).wrapping_add(48));
        F::BtlOpp_GetPoseVec30(c, p_chr_1, fp.wrapping_add(32));
        l_speed_10 = F::Vec3_Length(c, fp.wrapping_add(16));
        l_oppSpeed_11 = F::Vec3_Length(c, fp.wrapping_add(32));
        if (((((((rt::flt(F32(0x3a83126e), l_speed_10) as i32)) != 0) && (((rt::flt(F32(0x3a83126e), l_oppSpeed_11) as i32)) != 0)) as i32)) != 0) {
            { let r_ = rt::fmul(l_oppSpeed_11, rt::fdiv(F::Vec3_Dot(c, fp.wrapping_add(16), fp.wrapping_add(32)), rt::fmul(l_speed_10, l_oppSpeed_11))); let n_ = rt::fsub(p_closeSpeed_2, r_); p_closeSpeed_2 = n_; };
        }
        l_t_12 = F32(0x49742400);
        if (((rt::flt(F32(0x3a83126e), p_closeSpeed_2) as i32)) != 0) {
            l_t_12 = rt::fdiv(l_dist_9, p_closeSpeed_2);
        }
        if (((rt::flt(F32(0x40000000), l_t_12) as i32)) != 0) {
            l_k_13 = F32(0x3f800000);
        } else {
            l_k_13 = rt::fmul(l_t_12, F32(0x3f000000));
        }
        l_goal_18 = { let a0_ = rt::fmul(rt::fmul(l_k_13, F32(0x3fc90fda)), F::BtlInput_GetStickX(c, p_chr_1)); F::BtlUtil_ClampF(c, a0_, rt::fneg(p_yawMax_5), p_yawMax_5) };
        l_diff_17 = rt::fsub(l_goal_18, F32(c.m.r32((l_pose_8).wrapping_add(212))));
        { let v_ = { let a0_ = F32(c.m.r32((l_pose_8).wrapping_add(212))); let a2_ = F::BtlUtil_MinF(c, rt::ffabs(l_diff_17), p_yawAccel_3); F::BtlUtil_ApproachF(c, a0_, l_goal_18, a2_) }; let a_ = (l_pose_8).wrapping_add(212); c.m.w32(a_, (v_).0); };
        l_goal_18 = { let a0_ = rt::fadd({ let a0_ = F32(c.m.r32(fp)); let a1_ = F32(c.m.r32((fp).wrapping_add(8))); F::Port_atan2f(c, a0_, a1_) }, F32(c.m.r32((l_pose_8).wrapping_add(212)))); F::BtlUtil_WrapAngle(c, a0_) };
        { let v_ = { let a0_ = rt::fadd(F32(c.m.r32((l_pose_8).wrapping_add(148))), { let a0_ = { let a0_ = rt::fsub(l_goal_18, F32(c.m.r32((l_pose_8).wrapping_add(148)))); F::BtlUtil_WrapAngle(c, a0_) }; F::BtlUtil_ClampF(c, a0_, rt::fneg(p_maxStep_7), p_maxStep_7) }); F::BtlUtil_WrapAngle(c, a0_) }; let a_ = (l_pose_8).wrapping_add(148); c.m.w32(a_, (v_).0); };
        if (((rt::flt(F32(0x40000000), l_t_12) as i32)) != 0) {
            l_k2_14 = F32(0x3f800000);
        } else {
            l_k2_14 = rt::fmul(l_t_12, F32(0x3f000000));
        }
        F::Vec4_Copy(c, fp.wrapping_add(48), fp);
        F::Vec3_Normalize(c, fp.wrapping_add(48), fp.wrapping_add(48));
        { let v_ = { let a0_ = F32(c.m.r32((fp.wrapping_add(48)).wrapping_add(4))); F::BtlUtil_ClampF(c, a0_, F32(0xbf800000), F32(0x3f800000)) }; let a_ = (fp.wrapping_add(48)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        l_base_19 = rt::fneg({ let a0_ = F32(c.m.r32((fp.wrapping_add(48)).wrapping_add(4))); F::Mathf_Asin(c, a0_) });
        l_goal2_15 = { let a0_ = rt::fmul(rt::fmul(rt::fneg(l_k2_14), F32(0x3fc90fda)), F::BtlInput_GetStickY(c, p_chr_1)); F::BtlUtil_ClampF(c, a0_, rt::fneg(p_pitchMax_6), p_pitchMax_6) };
        l_cur_16 = F32(c.m.r32((l_pose_8).wrapping_add(216)));
        l_diff_17 = rt::fsub(l_goal2_15, l_cur_16);
        l_step_20 = rt::ffabs(l_diff_17);
        if (((rt::flt(p_pitchAccel_4, l_step_20) as i32)) != 0) {
            l_step_20 = p_pitchAccel_4;
        }
        { let v_ = F::BtlUtil_ApproachF(c, l_cur_16, l_goal2_15, l_step_20); let a_ = (l_pose_8).wrapping_add(216); c.m.w32(a_, (v_).0); };
        l_goal2_15 = { let a0_ = rt::fadd(l_base_19, F32(c.m.r32((l_pose_8).wrapping_add(216)))); F::BtlUtil_WrapAngle(c, a0_) };
        { let v_ = { let a0_ = rt::fadd(F32(c.m.r32((l_pose_8).wrapping_add(144))), { let a0_ = { let a0_ = rt::fsub(l_goal2_15, F32(c.m.r32((l_pose_8).wrapping_add(144)))); F::BtlUtil_WrapAngle(c, a0_) }; F::BtlUtil_ClampF(c, a0_, rt::fneg(p_maxStep_7), p_maxStep_7) }); F::BtlUtil_WrapAngle(c, a0_) }; let a_ = (l_pose_8).wrapping_add(144); c.m.w32(a_, (v_).0); };
        if (((rt::flt(l_t_12, F32(0x3f800000)) as i32)) != 0) {
            { let v_ = rt::fmul(F32(c.m.r32((l_pose_8).wrapping_add(144))), l_t_12); let a_ = (l_pose_8).wrapping_add(144); c.m.w32(a_, (v_).0); };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlMove_SteerAtOpponent", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMove_TurnModelYaw(c: &mut Ctx, mut p_chr_1: u32, mut p_maxStep_2: F32, mut p_rate_3: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMove_TurnModelYaw", &PROBE_N, &[(p_chr_1) as u32, (p_maxStep_2).0, (p_rate_3).0], false); }
    let mut l_pose_4: u32 = 0;
    let mut l_d_5: F32 = F32(0);
    let mut l_step_6: F32 = F32(0);
    let r_ = 'f: {
    l_pose_4 = F::BtlChar_GetPos(c, p_chr_1);
    l_d_5 = { let a0_ = rt::fsub(F32(c.m.r32((l_pose_4).wrapping_add(148))), F32(c.m.r32(((l_pose_4).wrapping_add(16)).wrapping_add(4)))); F::BtlUtil_WrapAngle(c, a0_) };
    l_step_6 = rt::fmul(rt::ffabs(l_d_5), p_rate_3);
    if (((rt::flt(l_step_6, F32(0x3d00adfb)) as i32)) != 0) {
        l_step_6 = F32(0x3d00adfb);
    }
    if (((rt::flt(p_maxStep_2, l_step_6) as i32)) != 0) {
        l_step_6 = p_maxStep_2;
    }
    if (((rt::flt(l_d_5, rt::fneg(l_step_6)) as i32)) != 0) {
        { let v_ = { let a0_ = rt::fsub(F32(c.m.r32(((l_pose_4).wrapping_add(16)).wrapping_add(4))), l_step_6); F::BtlUtil_WrapAngle(c, a0_) }; let a_ = ((l_pose_4).wrapping_add(16)).wrapping_add(4); c.m.w32(a_, (v_).0); };
    } else {
        if (((rt::flt(l_step_6, l_d_5) as i32)) != 0) {
            { let v_ = { let a0_ = rt::fadd(F32(c.m.r32(((l_pose_4).wrapping_add(16)).wrapping_add(4))), l_step_6); F::BtlUtil_WrapAngle(c, a0_) }; let a_ = ((l_pose_4).wrapping_add(16)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        } else {
            { let v_ = F32(c.m.r32((l_pose_4).wrapping_add(148))); let a_ = ((l_pose_4).wrapping_add(16)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMove_TurnModelYaw", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMove_SetHeading(c: &mut Ctx, mut p_chr_1: u32, mut p_yaw_2: F32, mut p_pitch_3: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMove_SetHeading", &PROBE_N, &[(p_chr_1) as u32, (p_yaw_2).0, (p_pitch_3).0], false); }
    let mut l_pose_4: u32 = 0;
    let r_ = 'f: {
    l_pose_4 = F::BtlChar_GetPos(c, p_chr_1);
    { let v_ = p_yaw_2; let a_ = (l_pose_4).wrapping_add(148); c.m.w32(a_, (v_).0); };
    { let v_ = p_yaw_2; let a_ = ((l_pose_4).wrapping_add(16)).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = p_pitch_3; let a_ = (l_pose_4).wrapping_add(144); c.m.w32(a_, (v_).0); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMove_SetHeading", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMove_TurnToPoint(c: &mut Ctx, mut p_chr_1: u32, mut p_target_2: u32, mut p_yawStep_3: F32, mut p_pitchStep_4: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMove_TurnToPoint", &PROBE_N, &[(p_chr_1) as u32, (p_target_2) as u32, (p_yawStep_3).0, (p_pitchStep_4).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_pose_5: u32 = 0;
    let mut l_eps_6: F32 = F32(0);
    let mut l_ang_7: F32 = F32(0);
    let mut l_pitch_8: F32 = F32(0);
    let r_ = 'f: {
        l_pose_5 = F::BtlChar_GetPos(c, p_chr_1);
        F::Vec4_Sub(c, fp, p_target_2, l_pose_5);
        l_eps_6 = F32(0x3a83126e);
        if (((rt::flt(l_eps_6, F::Vec3_Length(c, fp)) as i32)) != 0) {
            F::Vec3_Normalize(c, fp, fp);
        }
        l_ang_7 = F32(0x00000000);
        if (((((((rt::flt(l_eps_6, { let a0_ = F32(c.m.r32(fp)); rt::ffabs(a0_) }) as i32)) != 0) || (((rt::flt(l_eps_6, { let a0_ = F32(c.m.r32((fp).wrapping_add(8))); rt::ffabs(a0_) }) as i32)) != 0)) as i32)) != 0) {
            l_ang_7 = { let a0_ = F32(c.m.r32(fp)); let a1_ = F32(c.m.r32((fp).wrapping_add(8))); F::Port_atan2f(c, a0_, a1_) };
        }
        { let v_ = { let a0_ = rt::fadd(F32(c.m.r32((l_pose_5).wrapping_add(148))), { let a0_ = { let a0_ = rt::fsub(l_ang_7, F32(c.m.r32((l_pose_5).wrapping_add(148)))); F::BtlUtil_WrapAngle(c, a0_) }; F::BtlUtil_ClampF(c, a0_, rt::fneg(p_yawStep_3), p_yawStep_3) }); F::BtlUtil_WrapAngle(c, a0_) }; let a_ = (l_pose_5).wrapping_add(148); c.m.w32(a_, (v_).0); };
        if (((rt::flt(F32(c.m.r32((fp).wrapping_add(4))), F32(0xbf800000)) as i32)) != 0) {
            { let v_ = F32(0xbf800000); let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
        }
        if (((rt::fgt(F32(c.m.r32((fp).wrapping_add(4))), F32(0x3f800000)) as i32)) != 0) {
            { let v_ = F32(0x3f800000); let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
        }
        l_pitch_8 = rt::fneg({ let a0_ = F32(c.m.r32((fp).wrapping_add(4))); F::Mathf_Asin(c, a0_) });
        { let v_ = { let a0_ = rt::fadd(F32(c.m.r32((l_pose_5).wrapping_add(144))), { let a0_ = { let a0_ = rt::fsub(l_pitch_8, F32(c.m.r32((l_pose_5).wrapping_add(144)))); F::BtlUtil_WrapAngle(c, a0_) }; F::BtlUtil_ClampF(c, a0_, rt::fneg(p_pitchStep_4), p_pitchStep_4) }); F::BtlUtil_WrapAngle(c, a0_) }; let a_ = (l_pose_5).wrapping_add(144); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (l_pose_5).wrapping_add(212); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (l_pose_5).wrapping_add(216); c.m.w32(a_, (v_).0); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlMove_TurnToPoint", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMove_SetDirection(c: &mut Ctx, mut p_chr_1: u32, mut p_mode_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMove_SetDirection", &PROBE_N, &[(p_chr_1) as u32, (p_mode_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_pose_3: u32 = 0;
    let mut l_dir_4: u32 = 0;
    let mut l_c_5: F32 = F32(0);
    let mut l_s_6: F32 = F32(0);
    let mut l_cp_7: F32 = F32(0);
    let mut l_sp_8: F32 = F32(0);
    let mut l_x_9: F32 = F32(0);
    let mut l_y_10: F32 = F32(0);
    let mut l_a_11: F32 = F32(0);
    let r_ = 'f: {
        l_pose_3 = F::BtlChar_GetPos(c, p_chr_1);
        l_dir_4 = (l_pose_3).wrapping_add(128);
        if (((rt::flt(F::Vec3_Length(c, l_dir_4), F32(0x3a83126e)) as i32)) != 0) {
            { let v_ = { let a0_ = F32(c.m.r32((l_pose_3).wrapping_add(148))); F::Mathf_Sin(c, a0_) }; let a_ = l_dir_4; c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x00000000); let a_ = ((l_pose_3).wrapping_add(128)).wrapping_add(4); c.m.w32(a_, (v_).0); };
            { let v_ = { let a0_ = F32(c.m.r32((l_pose_3).wrapping_add(148))); F::Mathf_Cos(c, a0_) }; let a_ = ((l_pose_3).wrapping_add(128)).wrapping_add(8); c.m.w32(a_, (v_).0); };
        }
        's1: {
            let sv12: i32 = p_mode_2;
            match sv12 {
                0i32 | 1i32 => {
                    if ((F::BtlInput_IsHeld(c, p_chr_1, ((240i32) as u32))) != 0) {
                        l_x_9 = F::BtlInput_GetStickX(c, p_chr_1);
                        l_y_10 = rt::fneg(F::BtlInput_GetStickY(c, p_chr_1));
                        l_a_11 = rt::fneg(F32(c.m.r32((p_chr_1).wrapping_add(1184))));
                        l_c_5 = F::Mathf_Cos(c, l_a_11);
                        l_s_6 = F::Mathf_Sin(c, l_a_11);
                        if (((((p_mode_2) == (1i32)) as i32)) != 0) {
                            l_cp_7 = { let a0_ = F32(c.m.r32((l_pose_3).wrapping_add(144))); F::Mathf_Cos(c, a0_) };
                            l_sp_8 = { let a0_ = F32(c.m.r32((l_pose_3).wrapping_add(144))); F::Mathf_Sin(c, a0_) };
                        } else {
                            l_cp_7 = F32(0x3f800000);
                            l_sp_8 = F32(0x00000000);
                        }
                        { let v_ = rt::fmul(l_cp_7, rt::fsub(rt::fmul(l_x_9, l_c_5), rt::fmul(l_y_10, l_s_6))); let a_ = fp; c.m.w32(a_, (v_).0); };
                        { let v_ = rt::fneg(l_sp_8); let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
                        { let v_ = rt::fmul(l_cp_7, rt::fadd(rt::fmul(l_x_9, l_s_6), rt::fmul(l_y_10, l_c_5))); let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_).0); };
                    } else {
                        F::Vec4_Copy(c, fp, l_dir_4);
                    }
                    break 's1;
                }
                2i32 => {
                    l_c_5 = { let a0_ = F32(c.m.r32((l_pose_3).wrapping_add(148))); F::Mathf_Cos(c, a0_) };
                    l_s_6 = { let a0_ = F32(c.m.r32((l_pose_3).wrapping_add(148))); F::Mathf_Sin(c, a0_) };
                    { let v_ = l_s_6; let a_ = fp; c.m.w32(a_, (v_).0); };
                    { let v_ = l_c_5; let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(0x00000000); let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    break 's1;
                }
                3i32 => {
                    l_cp_7 = { let a0_ = F32(c.m.r32((l_pose_3).wrapping_add(144))); F::Mathf_Cos(c, a0_) };
                    l_sp_8 = { let a0_ = F32(c.m.r32((l_pose_3).wrapping_add(144))); F::Mathf_Sin(c, a0_) };
                    l_c_5 = { let a0_ = F32(c.m.r32((l_pose_3).wrapping_add(148))); F::Mathf_Cos(c, a0_) };
                    l_s_6 = { let a0_ = F32(c.m.r32((l_pose_3).wrapping_add(148))); F::Mathf_Sin(c, a0_) };
                    { let v_ = rt::fmul(l_cp_7, l_s_6); let a_ = fp; c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fneg(l_sp_8); let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fmul(l_cp_7, l_c_5); let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_).0); };
                    break 's1;
                }
                4i32 => {
                    l_c_5 = { let a0_ = F32(c.m.r32((l_pose_3).wrapping_add(148))); F::Mathf_Cos(c, a0_) };
                    l_s_6 = { let a0_ = F32(c.m.r32((l_pose_3).wrapping_add(148))); F::Mathf_Sin(c, a0_) };
                    { let v_ = rt::fneg(l_s_6); let a_ = fp; c.m.w32(a_, (v_).0); };
                    { let v_ = F32(0x00000000); let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fneg(l_c_5); let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_).0); };
                    break 's1;
                }
                5i32 => {
                    l_cp_7 = { let a0_ = F32(c.m.r32((l_pose_3).wrapping_add(144))); F::Mathf_Cos(c, a0_) };
                    l_sp_8 = { let a0_ = F32(c.m.r32((l_pose_3).wrapping_add(144))); F::Mathf_Sin(c, a0_) };
                    l_c_5 = { let a0_ = F32(c.m.r32((l_pose_3).wrapping_add(148))); F::Mathf_Cos(c, a0_) };
                    l_s_6 = { let a0_ = F32(c.m.r32((l_pose_3).wrapping_add(148))); F::Mathf_Sin(c, a0_) };
                    { let v_ = rt::fmul(rt::fneg(l_cp_7), l_s_6); let a_ = fp; c.m.w32(a_, (v_).0); };
                    { let v_ = l_sp_8; let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fmul(rt::fneg(l_cp_7), l_c_5); let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_).0); };
                    break 's1;
                }
                6i32 => {
                    { let v_ = F32(c.m.r32((l_pose_3).wrapping_add(128))); let a_ = fp; c.m.w32(a_, (v_).0); };
                    { let v_ = F32(0x00000000); let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(c.m.r32(((l_pose_3).wrapping_add(128)).wrapping_add(8))); let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_).0); };
                    break 's1;
                }
                7i32 => {
                    { let v_ = F32(c.m.r32((l_pose_3).wrapping_add(128))); let a_ = fp; c.m.w32(a_, (v_).0); };
                    { let v_ = F32(c.m.r32(((l_pose_3).wrapping_add(128)).wrapping_add(4))); let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(c.m.r32(((l_pose_3).wrapping_add(128)).wrapping_add(8))); let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_).0); };
                    break 's1;
                }
                _ => {
                }
            }
        }
        F::Vec3_Normalize(c, l_dir_4, fp);
        { let v_ = F32(0x00000000); let a_ = ((l_pose_3).wrapping_add(128)).wrapping_add(12); c.m.w32(a_, (v_).0); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlMove_SetDirection", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMove_Advance(c: &mut Ctx, mut p_chr_1: u32, mut p_speed_2: F32, mut p_accel_3: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMove_Advance", &PROBE_N, &[(p_chr_1) as u32, (p_speed_2).0, (p_accel_3).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_pose_4: u32 = 0;
    let r_ = 'f: {
        l_pose_4 = F::BtlChar_GetPos(c, p_chr_1);
        { let v_ = { let a0_ = F32(c.m.r32((l_pose_4).wrapping_add(152))); F::BtlUtil_ApproachF(c, a0_, p_speed_2, p_accel_3) }; let a_ = (l_pose_4).wrapping_add(152); c.m.w32(a_, (v_).0); };
        { let a2_ = F32(c.m.r32((l_pose_4).wrapping_add(152))); F::Vec4_Scale(c, fp, (l_pose_4).wrapping_add(128), a2_) };
        F::Vec4_Add(c, l_pose_4, l_pose_4, fp);
        { let v_ = F32(0x3f800000); let a_ = (l_pose_4).wrapping_add(12); c.m.w32(a_, (v_).0); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlMove_Advance", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMove_Step(c: &mut Ctx, mut p_chr_1: u32, mut p_yawMode_2: i32, mut p_pitchMode_3: i32, mut p_dirMode_4: i32, mut p_speed_5: F32, mut p_accel_6: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMove_Step", &PROBE_N, &[(p_chr_1) as u32, (p_yawMode_2) as u32, (p_pitchMode_3) as u32, (p_dirMode_4) as u32, (p_speed_5).0, (p_accel_6).0], false); }
    let r_ = 'f: {
    F::BtlMove_TurnYaw(c, p_chr_1, p_yawMode_2, F32(0x40490fda));
    F::BtlMove_TurnPitch(c, p_chr_1, p_pitchMode_3, F32(0x40490fda));
    F::BtlMove_TurnModelYaw(c, p_chr_1, F32(0x3f20d97a), F32(0x3e999999));
    F::BtlMove_SetDirection(c, p_chr_1, p_dirMode_4);
    F::BtlMove_Advance(c, p_chr_1, p_speed_5, p_accel_6);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMove_Step", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMove_WarpAheadOfOpponent(c: &mut Ctx, mut p_chr_1: u32, mut p_lead_2: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMove_WarpAheadOfOpponent", &PROBE_N, &[(p_chr_1) as u32, (p_lead_2).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(80) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 80);
    let mut l_pose_3: u32 = 0;
    let mut l_min_4: F32 = F32(0);
    let mut l_rad_5: F32 = F32(0);
    let mut l_len_6: F32 = F32(0);
    let mut l_lim_7: F32 = F32(0);
    let r_ = 'f: {
        l_pose_3 = F::BtlChar_GetPos(c, p_chr_1);
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((5i32) as i64 as u64)) as u32))) != 0) {
            F::Vec4_Copy(c, fp.wrapping_add(32), l_pose_3);
            l_min_4 = F32(0x409425ec);
            F::BtlOpp_GetTargetPos(c, p_chr_1, fp);
            { let r_ = F::BtlOpp_GetHalfHeightDiff(c, p_chr_1); let a_ = (fp).wrapping_add(4); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            F::BtlOpp_GetPoseVec30(c, p_chr_1, fp.wrapping_add(16));
            if (((rt::flt(F::Vec3_Length(c, fp.wrapping_add(16)), l_min_4) as i32)) != 0) {
                F::BtlOpp_GetTargetRot(c, p_chr_1, fp.wrapping_add(64));
                { let v_ = rt::fmul(rt::fneg({ let a0_ = F32(c.m.r32((fp.wrapping_add(64)).wrapping_add(4))); F::Mathf_Sin(c, a0_) }), l_min_4); let a_ = fp.wrapping_add(16); c.m.w32(a_, (v_).0); };
                { let v_ = F32(0x00000000); let a_ = (fp.wrapping_add(16)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fmul(rt::fneg({ let a0_ = F32(c.m.r32((fp.wrapping_add(64)).wrapping_add(4))); F::Mathf_Cos(c, a0_) }), l_min_4); let a_ = (fp.wrapping_add(16)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                { let v_ = F32(0x00000000); let a_ = (fp.wrapping_add(16)).wrapping_add(12); c.m.w32(a_, (v_).0); };
            }
            F::Vec4_Scale(c, fp.wrapping_add(48), fp.wrapping_add(16), p_lead_2);
            F::Vec4_Add(c, fp, fp, fp.wrapping_add(48));
            F::Vec4_Copy(c, fp.wrapping_add(48), fp.wrapping_add(16));
            l_rad_5 = rt::fadd({ let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); F::BtlCharApi_GetRadius(c, a0_) }, F::BtlOpp_GetRadius(c, p_chr_1));
            l_len_6 = F::BtlUtil_LengthXZ(c, fp.wrapping_add(48));
            if (((rt::flt(F32(0x38d1b717), l_len_6) as i32)) != 0) {
                l_len_6 = rt::fdiv(l_rad_5, l_len_6);
                { let r_ = l_len_6; let a_ = fp.wrapping_add(48); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                { let r_ = l_len_6; let a_ = (fp.wrapping_add(48)).wrapping_add(8); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            }
            F::Vec4_Add(c, l_pose_3, fp, fp.wrapping_add(48));
            l_rad_5 = { let a_ = ({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(4000); c.m.r32(a_) }).wrapping_add(16); F32(c.m.r32(a_)) };
            l_rad_5 = rt::fsub(F::BtlStage_GetInnerRadius(c), l_rad_5);
            l_lim_7 = F::BtlUtil_LengthXZ(c, l_pose_3);
            if (((rt::flt(l_rad_5, l_lim_7) as i32)) != 0) {
                F::Vec4_Scale(c, l_pose_3, l_pose_3, rt::fdiv(l_rad_5, l_lim_7));
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlMove_WarpAheadOfOpponent", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMove_WarpBehindOpponent(c: &mut Ctx, mut p_chr_1: u32, mut p_gap_2: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMove_WarpBehindOpponent", &PROBE_N, &[(p_chr_1) as u32, (p_gap_2).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(64) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 64);
    let mut l_d_3: F32 = F32(0);
    let r_ = 'f: {
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((5i32) as i64 as u64)) as u32))) != 0) {
            F::BtlOpp_GetTargetPos(c, p_chr_1, fp);
            F::BtlOpp_GetTargetRot(c, p_chr_1, fp.wrapping_add(16));
            F::BtlOpp_GetPoseVec30(c, p_chr_1, fp.wrapping_add(32));
            F::Vec4_Add(c, fp, fp, fp.wrapping_add(32));
            l_d_3 = rt::fadd({ let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); F::BtlCharApi_GetRadius(c, a0_) }, F::BtlOpp_GetRadius(c, p_chr_1));
            { let r_ = p_gap_2; let n_ = rt::fadd(l_d_3, r_); l_d_3 = n_; };
            { let v_ = rt::fmul(rt::fneg({ let a0_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))); F::Mathf_Sin(c, a0_) }), l_d_3); let a_ = fp.wrapping_add(48); c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x00000000); let a_ = (fp.wrapping_add(48)).wrapping_add(4); c.m.w32(a_, (v_).0); };
            { let v_ = rt::fmul(rt::fneg({ let a0_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))); F::Mathf_Cos(c, a0_) }), l_d_3); let a_ = (fp.wrapping_add(48)).wrapping_add(8); c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x00000000); let a_ = (fp.wrapping_add(48)).wrapping_add(12); c.m.w32(a_, (v_).0); };
            { let a0_ = F::BtlChar_GetPos(c, p_chr_1); F::Vec4_Add(c, a0_, fp, fp.wrapping_add(48)) };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlMove_WarpBehindOpponent", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMove_WarpInFrontOfOpponent(c: &mut Ctx, mut p_chr_1: u32, mut p_gap_2: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMove_WarpInFrontOfOpponent", &PROBE_N, &[(p_chr_1) as u32, (p_gap_2).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(64) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 64);
    let mut l_d_3: F32 = F32(0);
    let r_ = 'f: {
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((5i32) as i64 as u64)) as u32))) != 0) {
            F::BtlOpp_GetTargetPos(c, p_chr_1, fp);
            F::BtlOpp_GetTargetRot(c, p_chr_1, fp.wrapping_add(16));
            F::BtlOpp_GetPoseVec30(c, p_chr_1, fp.wrapping_add(32));
            F::Vec4_Add(c, fp, fp, fp.wrapping_add(32));
            l_d_3 = rt::fadd({ let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); F::BtlCharApi_GetRadius(c, a0_) }, F::BtlOpp_GetRadius(c, p_chr_1));
            { let r_ = p_gap_2; let n_ = rt::fadd(l_d_3, r_); l_d_3 = n_; };
            { let v_ = rt::fmul({ let a0_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))); F::Mathf_Sin(c, a0_) }, l_d_3); let a_ = fp.wrapping_add(48); c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x00000000); let a_ = (fp.wrapping_add(48)).wrapping_add(4); c.m.w32(a_, (v_).0); };
            { let v_ = rt::fmul({ let a0_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))); F::Mathf_Cos(c, a0_) }, l_d_3); let a_ = (fp.wrapping_add(48)).wrapping_add(8); c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x00000000); let a_ = (fp.wrapping_add(48)).wrapping_add(12); c.m.w32(a_, (v_).0); };
            { let a0_ = F::BtlChar_GetPos(c, p_chr_1); F::Vec4_Add(c, a0_, fp, fp.wrapping_add(48)) };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlMove_WarpInFrontOfOpponent", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMove_SnapToOpponent(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMove_SnapToOpponent", &PROBE_N, &[(p_chr_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let mut l_pose_2: u32 = 0;
    let r_ = 'f: {
        l_pose_2 = F::BtlChar_GetPos(c, p_chr_1);
        { let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); F::BtlCharApi_GetBodyPos(c, a0_, fp) };
        F::BtlOpp_GetObjVecFA0(c, p_chr_1, fp.wrapping_add(16));
        F::Vec4_Sub(c, fp.wrapping_add(32), fp.wrapping_add(16), fp);
        F::Vec4_Add(c, l_pose_2, l_pose_2, fp.wrapping_add(32));
        { let v_ = F32(0x00000000); let a_ = (l_pose_2).wrapping_add(152); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (l_pose_2).wrapping_add(156); c.m.w32(a_, (v_).0); };
        F::Vec4_SetZero(c, (l_pose_2).wrapping_add(112));
        F::BtlChar_SetUnk1310(c, p_chr_1, fp.wrapping_add(16));
        F::BtlChar_SetFlag(c, p_chr_1, ((((63i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((36i32) as i64 as u64)) as u32));
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlMove_SnapToOpponent", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMove_CircleOpponent(c: &mut Ctx, mut p_chr_1: u32, mut p_side_2: i32, mut p_lead_3: i32, mut p_baseYaw_4: F32, mut p_speed_5: F32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMove_CircleOpponent", &PROBE_N, &[(p_chr_1) as u32, (p_side_2) as u32, (p_lead_3) as u32, (p_baseYaw_4).0, (p_speed_5).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(96) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 96);
    let mut l_ret_6: i32 = 0;
    let mut l_pose_7: u32 = 0;
    let mut l_radius_8: F32 = F32(0);
    let mut l_diff_9: F32 = F32(0);
    let mut l_scale_10: F32 = F32(0);
    let mut l_t_11: F32 = F32(0);
    let mut l_a_12: F32 = F32(0);
    let mut l_k_13: F32 = F32(0);
    let r_ = 'f: {
        l_ret_6 = 0i32;
        l_pose_7 = F::BtlChar_GetPos(c, p_chr_1);
        F::BtlOpp_GetTargetPos(c, p_chr_1, fp.wrapping_add(16));
        { let r_ = F::BtlOpp_GetHalfHeightDiff(c, p_chr_1); let a_ = (fp.wrapping_add(16)).wrapping_add(4); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        l_radius_8 = F32(0x43480000);
        l_diff_9 = F32(0x00000000);
        F::Vec4_Sub(c, fp.wrapping_add(32), fp.wrapping_add(16), l_pose_7);
        if (((((((rt::flt(F32(0x3c23d70a), { let a0_ = F32(c.m.r32(fp.wrapping_add(32))); rt::ffabs(a0_) }) as i32)) != 0) || (((rt::flt(F32(0x3c23d70a), { let a0_ = F32(c.m.r32((fp.wrapping_add(32)).wrapping_add(8))); rt::ffabs(a0_) }) as i32)) != 0)) as i32)) != 0) {
            l_diff_9 = { let a0_ = rt::fsub(p_baseYaw_4, { let a0_ = F32(c.m.r32(fp.wrapping_add(32))); let a1_ = F32(c.m.r32((fp.wrapping_add(32)).wrapping_add(8))); F::Port_atan2f(c, a0_, a1_) }); F::BtlUtil_WrapAngle(c, a0_) };
        }
        l_diff_9 = rt::ffabs(l_diff_9);
        l_scale_10 = F32(0x3f800000);
        if (((rt::flt(l_diff_9, F32(0x3fc90fda)) as i32)) != 0) {
            l_scale_10 = F32(0x00000000);
            if (((rt::flt(F32(0x3ea0d97a), l_diff_9) as i32)) != 0) {
                l_scale_10 = F::Mathf_Sin(c, rt::fsub(l_diff_9, F32(0x3ea0d97a)));
            }
        }
        l_t_11 = rt::fdiv(rt::fmul(l_radius_8, l_diff_9), p_speed_5);
        if ((p_lead_3) != 0) {
            F::BtlOpp_GetVelocity(c, p_chr_1, fp.wrapping_add(48));
            if (((rt::flt(F32(0x3dbda12e), F::BtlUtil_LengthXZ(c, fp.wrapping_add(48))) as i32)) != 0) {
                l_k_13 = rt::fdiv(rt::fsub({ let a0_ = { let a0_ = rt::fsub(p_baseYaw_4, { let a0_ = F32(c.m.r32(fp.wrapping_add(48))); let a1_ = F32(c.m.r32((fp.wrapping_add(48)).wrapping_add(8))); F::Port_atan2f(c, a0_, a1_) }); F::BtlUtil_WrapAngle(c, a0_) }; rt::ffabs(a0_) }, F32(0x3fc90fda)), F32(0x3fc90fda));
                if (((rt::flt(F32(0x00000000), l_k_13) as i32)) != 0) {
                    F::Vec4_Scale(c, fp.wrapping_add(48), fp.wrapping_add(48), rt::fmul(l_t_11, l_k_13));
                    F::Vec4_Add(c, fp.wrapping_add(16), fp.wrapping_add(16), fp.wrapping_add(48));
                }
            }
        }
        if (((((p_side_2) == (0i32)) as i32)) != 0) {
            l_diff_9 = rt::fneg(l_diff_9);
        }
        l_a_12 = { let a0_ = rt::fadd(rt::fmul(l_diff_9, F32(0x3f000000)), F::BtlUtil_WrapAngle(c, rt::fadd(p_baseYaw_4, F32(0x40490fda)))); F::BtlUtil_WrapAngle(c, a0_) };
        { let v_ = rt::fmul(rt::fmul(F::Mathf_Sin(c, l_a_12), l_radius_8), l_scale_10); let a_ = fp; c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(rt::fmul(F::Mathf_Cos(c, l_a_12), l_radius_8), l_scale_10); let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_).0); };
        F::Vec4_Add(c, fp, fp, fp.wrapping_add(16));
        if (({ let a3_ = F::BtlStage_GetInnerRadius(c); F::BtlMove_ClipToRadius(c, fp.wrapping_add(64), fp.wrapping_add(16), fp, a3_) }) != 0) {
            F::Vec4_Copy(c, fp, fp.wrapping_add(64));
        }
        F::BtlMove_TurnToPoint(c, p_chr_1, fp, F32(0x40490fda), F32(0x40490fda));
        F::BtlMove_TurnModelYaw(c, p_chr_1, F32(0x3fc90fda), F32(0x3f000000));
        F::BtlMove_SetDirection(c, p_chr_1, 3i32);
        { let v_ = p_speed_5; let a_ = (l_pose_7).wrapping_add(152); c.m.w32(a_, (v_).0); };
        F::Vec4_Sub(c, fp.wrapping_add(64), fp, l_pose_7);
        l_a_12 = { let a0_ = F::Vec3_Dot(c, fp.wrapping_add(64), (l_pose_7).wrapping_add(128)); F::BtlUtil_MaxF(c, a0_, F32(0x00000000)) };
        if (((rt::flt(l_a_12, F32(c.m.r32((l_pose_7).wrapping_add(152)))) as i32)) != 0) {
            { let v_ = l_a_12; let a_ = (l_pose_7).wrapping_add(152); c.m.w32(a_, (v_).0); };
            l_ret_6 = 1i32;
        }
        { let a2_ = F32(c.m.r32((l_pose_7).wrapping_add(152))); F::Vec4_Scale(c, fp.wrapping_add(80), (l_pose_7).wrapping_add(128), a2_) };
        F::Vec4_Add(c, l_pose_7, l_pose_7, fp.wrapping_add(80));
        { let v_ = F32(0x3f800000); let a_ = (l_pose_7).wrapping_add(12); c.m.w32(a_, (v_).0); };
        break 'f l_ret_6;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlMove_CircleOpponent", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMove_CalcApproachPoint(c: &mut Ctx, mut p_chr_1: u32, mut p_out_2: u32, mut p_outTarget_3: u32, mut p_dist_4: F32, mut p_angle_5: F32, mut p_distScale_6: F32, mut p_lead_7: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMove_CalcApproachPoint", &PROBE_N, &[(p_chr_1) as u32, (p_out_2) as u32, (p_outTarget_3) as u32, (p_dist_4).0, (p_angle_5).0, (p_distScale_6).0, (p_lead_7).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(128) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 128);
    let mut l_scale_8: F32 = F32(0);
    let mut l_oppScale_9: F32 = F32(0);
    let mut l_top_10: F32 = F32(0);
    let mut l_bottom_11: F32 = F32(0);
    let mut l_len_12: F32 = F32(0);
    let mut l_len3_13: F32 = F32(0);
    let mut l_lenXZ_14: F32 = F32(0);
    let mut l_len3_15: F32 = F32(0);
    let mut l_lenXZ_16: F32 = F32(0);
    let mut l_k_17: F32 = F32(0);
    let mut l_m_18: F32 = F32(0);
    let mut l_r_19: F32 = F32(0);
    let r_ = 'f: {
        F::BtlOpp_GetTargetPos(c, p_chr_1, fp);
        F::BtlOpp_GetVelocity(c, p_chr_1, fp.wrapping_add(32));
        F::Vec4_Scale(c, fp.wrapping_add(32), fp.wrapping_add(32), p_lead_7);
        F::Vec4_Add(c, fp.wrapping_add(16), fp, fp.wrapping_add(32));
        l_scale_8 = { let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); F::BtlCharApi_GetHeight(c, a0_) };
        l_oppScale_9 = F::BtlOpp_GetHeight(c, p_chr_1);
        { let r_ = F::BtlOpp_GetHalfHeightDiff(c, p_chr_1); let a_ = (fp.wrapping_add(16)).wrapping_add(4); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        F::ColSeg_Set(c, fp.wrapping_add(64), fp, fp.wrapping_add(16));
        if ((F::StgCol_TraceSegment(c, fp.wrapping_add(64))) != 0) {
            { let _ = F::StgCol_GetHitPos(c, fp.wrapping_add(96)); };
            F::Vec4_Sub(c, fp.wrapping_add(112), fp.wrapping_add(16), fp.wrapping_add(96));
            l_len3_13 = F::Vec3_Length(c, fp.wrapping_add(112));
            l_lenXZ_14 = F::BtlUtil_LengthXZ(c, fp.wrapping_add(112));
            if (((rt::flt(F32(0x3a83126e), l_lenXZ_14) as i32)) != 0) {
                l_lenXZ_14 = rt::fdiv(l_len3_13, l_lenXZ_14);
                { let v_ = rt::fadd(F32(c.m.r32(fp.wrapping_add(96))), rt::fmul(F32(c.m.r32(fp.wrapping_add(112))), l_lenXZ_14)); let a_ = fp.wrapping_add(16); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fadd(F32(c.m.r32((fp.wrapping_add(96)).wrapping_add(8))), rt::fmul(F32(c.m.r32((fp.wrapping_add(112)).wrapping_add(8))), l_lenXZ_14)); let a_ = (fp.wrapping_add(16)).wrapping_add(8); c.m.w32(a_, (v_).0); };
            }
            { let v_ = F32(c.m.r32((fp.wrapping_add(96)).wrapping_add(4))); let a_ = (fp.wrapping_add(16)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        }
        l_top_10 = rt::fadd(F::BtlStage_GetTop(c), rt::fmul(l_oppScale_9, F32(0x3f000000)));
        if (((rt::flt(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))), l_top_10) as i32)) != 0) {
            { let v_ = l_top_10; let a_ = (fp.wrapping_add(16)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        }
        l_bottom_11 = rt::fadd(F::BtlStage_GetBottom(c), rt::fmul(l_oppScale_9, F32(0x3f000000)));
        if (((rt::flt(l_bottom_11, F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4)))) as i32)) != 0) {
            F::Vec4_Sub(c, fp.wrapping_add(96), fp.wrapping_add(16), fp);
            l_len3_15 = F::Vec3_Length(c, fp.wrapping_add(96));
            l_lenXZ_16 = F::BtlUtil_LengthXZ(c, fp.wrapping_add(96));
            if (((rt::flt(F32(0x3a83126e), l_lenXZ_16) as i32)) != 0) {
                l_k_17 = F32(c.m.r32((fp.wrapping_add(96)).wrapping_add(4)));
                if (((rt::flt(F32(0x3a83126e), rt::ffabs(l_k_17)) as i32)) != 0) {
                    l_k_17 = rt::fdiv(rt::fsub(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))), l_bottom_11), l_k_17);
                    if (((rt::flt(F32(0x00000000), l_k_17) as i32)) != 0) {
                        l_m_18 = rt::fdiv(rt::fadd(rt::fmul(l_lenXZ_16, rt::fsub(F32(0x3f800000), l_k_17)), rt::fmul(l_len3_15, l_k_17)), l_lenXZ_16);
                        { let r_ = l_m_18; let a_ = fp.wrapping_add(96); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                        { let r_ = l_m_18; let a_ = (fp.wrapping_add(96)).wrapping_add(8); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                        { let v_ = rt::fadd(F32(c.m.r32(fp)), F32(c.m.r32(fp.wrapping_add(96)))); let a_ = fp.wrapping_add(16); c.m.w32(a_, (v_).0); };
                        { let v_ = rt::fadd(F32(c.m.r32((fp).wrapping_add(8))), F32(c.m.r32((fp.wrapping_add(96)).wrapping_add(8)))); let a_ = (fp.wrapping_add(16)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                    }
                }
            }
            { let v_ = l_bottom_11; let a_ = (fp.wrapping_add(16)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        }
        if (((((p_outTarget_3) != (((0i32) as u32))) as i32)) != 0) {
            F::Vec4_Copy(c, p_outTarget_3, fp.wrapping_add(16));
        }
        { let a2_ = F::BtlChar_GetPos(c, p_chr_1); F::Vec4_Sub(c, fp.wrapping_add(48), fp.wrapping_add(16), a2_) };
        { let v_ = F32(0x00000000); let a_ = (fp.wrapping_add(48)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        l_len_12 = F::Vec3_Length(c, fp.wrapping_add(48));
        if (((rt::flt(F32(0x3c23d70a), l_len_12) as i32)) != 0) {
            l_len_12 = rt::fdiv(F::Mathf_Cos(c, p_angle_5), l_len_12);
            { let a1_ = rt::fmul(F32(c.m.r32(fp.wrapping_add(48))), l_len_12); let a2_ = rt::fneg(F::Mathf_Sin(c, p_angle_5)); let a3_ = rt::fmul(F32(c.m.r32((fp.wrapping_add(48)).wrapping_add(8))), l_len_12); F::Vec4_Set(c, fp.wrapping_add(48), a1_, a2_, a3_, F32(0x00000000)) };
        } else {
            { let a2_ = rt::fneg(F::Mathf_Sin(c, p_angle_5)); let a3_ = F::Mathf_Cos(c, p_angle_5); F::Vec4_Set(c, fp.wrapping_add(48), F32(0x00000000), a2_, a3_, F32(0x00000000)) };
        }
        if (((((p_out_2) != (((0i32) as u32))) as i32)) != 0) {
            l_len_12 = rt::fmul(p_distScale_6, p_dist_4);
            { let r_ = { let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); F::BtlCharApi_GetRadius(c, a0_) }; let n_ = rt::fadd(l_len_12, r_); l_len_12 = n_; };
            { let a2_ = rt::fadd(l_len_12, F::BtlOpp_GetRadius(c, p_chr_1)); F::Vec4_Scale(c, fp.wrapping_add(48), fp.wrapping_add(48), a2_) };
            F::Vec4_Sub(c, p_out_2, fp.wrapping_add(16), fp.wrapping_add(48));
            l_top_10 = rt::fadd(F::BtlStage_GetTop(c), rt::fmul(l_scale_8, F32(0x3f000000)));
            if (((rt::flt(F32(c.m.r32((p_out_2).wrapping_add(4))), l_top_10) as i32)) != 0) {
                p_angle_5 = F::BtlUtil_LengthXZ(c, fp.wrapping_add(48));
                l_len_12 = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4)));
                l_r_19 = F::Vec3_Length(c, fp.wrapping_add(48));
                l_len_12 = rt::fsub(l_top_10, l_len_12);
                { let r_ = l_len_12; let n_ = rt::fmul(l_len_12, r_); l_len_12 = n_; };
                l_r_19 = F::Port_sqrtf(c, rt::fsub(rt::fmul(l_r_19, l_r_19), l_len_12));
                if (((rt::flt(F32(0x3c23d70a), p_angle_5) as i32)) != 0) {
                    l_len_12 = rt::fdiv(l_r_19, p_angle_5);
                    { let v_ = rt::fsub(F32(c.m.r32(fp.wrapping_add(16))), rt::fmul(F32(c.m.r32(fp.wrapping_add(48))), l_len_12)); let a_ = p_out_2; c.m.w32(a_, (v_).0); };
                    { let v_ = l_top_10; let a_ = (p_out_2).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fsub(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8))), rt::fmul(F32(c.m.r32((fp.wrapping_add(48)).wrapping_add(8))), l_len_12)); let a_ = (p_out_2).wrapping_add(8); c.m.w32(a_, (v_).0); };
                } else {
                    { let v_ = F32(c.m.r32(fp.wrapping_add(16))); let a_ = p_out_2; c.m.w32(a_, (v_).0); };
                    { let v_ = l_top_10; let a_ = (p_out_2).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fsub(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8))), l_r_19); let a_ = (p_out_2).wrapping_add(8); c.m.w32(a_, (v_).0); };
                }
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlMove_CalcApproachPoint", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMove_Fall(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMove_Fall", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_pose_2: u32 = 0;
    let r_ = 'f: {
    l_pose_2 = F::BtlChar_GetPos(c, p_chr_1);
    { let r_ = F32(0x3eed0979); let a_ = (l_pose_2).wrapping_add(156); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
    if (((rt::flt(F32(0x41de38e2), F32(c.m.r32((l_pose_2).wrapping_add(156)))) as i32)) != 0) {
        { let v_ = F32(0x41de38e2); let a_ = (l_pose_2).wrapping_add(156); c.m.w32(a_, (v_).0); };
    }
    { let r_ = F32(c.m.r32((l_pose_2).wrapping_add(156))); let a_ = (l_pose_2).wrapping_add(4); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMove_Fall", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMove_MoveVertical(c: &mut Ctx, mut p_chr_1: u32, mut p_speed_2: F32, mut p_accel_3: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMove_MoveVertical", &PROBE_N, &[(p_chr_1) as u32, (p_speed_2).0, (p_accel_3).0], false); }
    let mut l_pose_4: u32 = 0;
    let r_ = 'f: {
    l_pose_4 = F::BtlChar_GetPos(c, p_chr_1);
    { let v_ = { let a0_ = F32(c.m.r32((l_pose_4).wrapping_add(156))); F::BtlUtil_ApproachF(c, a0_, p_speed_2, p_accel_3) }; let a_ = (l_pose_4).wrapping_add(156); c.m.w32(a_, (v_).0); };
    { let r_ = F32(c.m.r32((l_pose_4).wrapping_add(156))); let a_ = (l_pose_4).wrapping_add(4); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMove_MoveVertical", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMove_BrakeVertical(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMove_BrakeVertical", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    F::BtlMove_MoveVertical(c, p_chr_1, F32(0x00000000), F32(0x3f6d0979));
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMove_BrakeVertical", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMove_ApplyGravity(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMove_ApplyGravity", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    if ((F::BtlChar_TestFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32))) != 0) {
        F::BtlMove_BrakeVertical(c, p_chr_1);
    } else {
        F::BtlMove_Fall(c, p_chr_1);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMove_ApplyGravity", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMove_SetLeanX(c: &mut Ctx, mut p_chr_1: u32, mut p_v_2: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMove_SetLeanX", &PROBE_N, &[(p_chr_1) as u32, (p_v_2).0], false); }
    let mut l_pose_3: u32 = 0;
    let r_ = 'f: {
    l_pose_3 = F::BtlChar_GetPos(c, p_chr_1);
    if (((rt::flt(p_v_2, F32(0xbfc7c82b)) as i32)) != 0) {
        p_v_2 = F32(0xbfc7c82b);
    }
    if (((rt::flt(F32(0x3fc7c82b), p_v_2) as i32)) != 0) {
        p_v_2 = F32(0x3fc7c82b);
    }
    { let v_ = p_v_2; let a_ = (l_pose_3).wrapping_add(160); c.m.w32(a_, (v_).0); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMove_SetLeanX", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMove_SetLeanZ(c: &mut Ctx, mut p_chr_1: u32, mut p_v_2: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMove_SetLeanZ", &PROBE_N, &[(p_chr_1) as u32, (p_v_2).0], false); }
    let r_ = 'f: {
    { let v_ = p_v_2; let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(164); c.m.w32(a_, (v_).0); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMove_SetLeanZ", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMove_IsBlockedByOpponent(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMove_IsBlockedByOpponent", &PROBE_N, &[(p_chr_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((94i32) as i64 as u64)) as u32))) != 0) {
            if ((F::BtlChar_TestFlag(c, p_chr_1, ((((95i32) as i64 as u64)) as u32))) != 0) {
                break 'f 1i32;
            }
            F::BtlOpp_GetDelta(c, p_chr_1, fp);
            if (((rt::flt(F32(0x00000000), F32(c.m.r32((fp).wrapping_add(4)))) as i32)) != 0) {
                if (((rt::flt(F32(c.m.r32((fp).wrapping_add(4))), rt::fmul(F::BtlOpp_GetHeight(c, p_chr_1), F32(0x3f000000))) as i32)) != 0) {
                    break 'f 1i32;
                }
            } else {
                if (((rt::flt(rt::fneg(F32(c.m.r32((fp).wrapping_add(4)))), rt::fmul({ let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); F::BtlCharApi_GetHeight(c, a0_) }, F32(0x3f000000))) as i32)) != 0) {
                    break 'f 1i32;
                }
            }
        }
        break 'f 0i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlMove_IsBlockedByOpponent", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMove_ClampToStage(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMove_ClampToStage", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_pose_2: u32 = 0;
    let mut l_r_3: F32 = F32(0);
    let mut l_rad_4: F32 = F32(0);
    let mut l_lim_5: F32 = F32(0);
    let mut l_k_6: F32 = F32(0);
    let mut l_top_7: F32 = F32(0);
    let mut l_bottom_8: F32 = F32(0);
    let mut l_half_9: F32 = F32(0);
    let r_ = 'f: {
    l_pose_2 = F::BtlChar_GetPos(c, p_chr_1);
    l_r_3 = F::BtlUtil_LengthXZ(c, l_pose_2);
    l_rad_4 = { let a_ = ({ let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(4000); c.m.r32(a_) }).wrapping_add(16); F32(c.m.r32(a_)) };
    l_lim_5 = rt::fsub(F::BtlStage_GetInnerRadius(c), l_rad_4);
    if (((rt::flt(l_lim_5, l_r_3) as i32)) != 0) {
        l_k_6 = rt::fdiv(l_lim_5, l_r_3);
        { let r_ = l_k_6; let a_ = l_pose_2; let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        { let r_ = l_k_6; let a_ = (l_pose_2).wrapping_add(8); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        F::BtlChar_SetHeldFlag(c, p_chr_1, ((((20i32) as i64 as u64)) as u32));
    } else {
        F::BtlChar_ClearFlag(c, p_chr_1, ((((20i32) as i64 as u64)) as u32));
    }
    if ((((!((F::BtlChar_TestFlag(c, p_chr_1, ((((24i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) {
        l_top_7 = F::BtlStage_GetTop(c);
        { let r_ = rt::fmul({ let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); F::BtlCharApi_GetHeight(c, a0_) }, F32(0x3f000000)); let n_ = rt::fadd(l_top_7, r_); l_top_7 = n_; };
        if (((rt::flt(F32(c.m.r32((l_pose_2).wrapping_add(4))), l_top_7) as i32)) != 0) {
            { let v_ = l_top_7; let a_ = (l_pose_2).wrapping_add(4); c.m.w32(a_, (v_).0); };
            F::BtlChar_SetHeldFlag(c, p_chr_1, ((((21i32) as i64 as u64)) as u32));
        } else {
            F::BtlChar_ClearFlag(c, p_chr_1, ((((21i32) as i64 as u64)) as u32));
        }
    }
    if ((((!((F::BtlChar_IsDead(c, p_chr_1)) != 0)) as i32)) != 0) {
        l_bottom_8 = F::BtlStage_GetBottom(c);
        { let r_ = rt::fmul({ let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); F::BtlCharApi_GetHeight(c, a0_) }, F32(0x3f000000)); let n_ = rt::fadd(l_bottom_8, r_); l_bottom_8 = n_; };
        if (((rt::flt(l_bottom_8, F32(c.m.r32((l_pose_2).wrapping_add(4)))) as i32)) != 0) {
            { let v_ = l_bottom_8; let a_ = (l_pose_2).wrapping_add(4); c.m.w32(a_, (v_).0); };
            F::BtlChar_SetHeldFlag(c, p_chr_1, ((((22i32) as i64 as u64)) as u32));
        } else {
            F::BtlChar_ClearFlag(c, p_chr_1, ((((22i32) as i64 as u64)) as u32));
        }
    }
    F::BtlChar_ClearFlag(c, p_chr_1, ((((23i32) as i64 as u64)) as u32));
    if ((F::BtlChar_IsStage4Or27(c)) != 0) {
        if ((((((((((((((F::BtlChar_IsFree(c, p_chr_1)) != 0) && ((((!((F::BtlChar_TestFlag(c, p_chr_1, ((((42i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0)) as i32)) != 0) && ((((!((F::BtlChar_TestFlag(c, p_chr_1, ((((7i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0)) as i32)) != 0) && (((rt::flt(F32(0xc1200000), F32(c.m.r32((l_pose_2).wrapping_add(4)))) as i32)) != 0)) as i32)) != 0) {
            { let v_ = F32(0xc1200000); let a_ = (l_pose_2).wrapping_add(4); c.m.w32(a_, (v_).0); };
            F::BtlChar_SetHeldFlag(c, p_chr_1, ((((23i32) as i64 as u64)) as u32));
        }
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((2i32) as i64 as u64)) as u32))) != 0) {
            l_half_9 = F32(0x43390000);
            { let v_ = { let a0_ = F32(c.m.r32(l_pose_2)); F::BtlUtil_ClampF(c, a0_, rt::fneg(l_half_9), l_half_9) }; let a_ = l_pose_2; c.m.w32(a_, (v_).0); };
            { let v_ = { let a0_ = F32(c.m.r32((l_pose_2).wrapping_add(8))); F::BtlUtil_ClampF(c, a0_, rt::fneg(l_half_9), l_half_9) }; let a_ = (l_pose_2).wrapping_add(8); c.m.w32(a_, (v_).0); };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMove_ClampToStage", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMove_RequestOrbit(c: &mut Ctx, mut p_chr_1: u32, mut p_near_2: F32, mut p_far_3: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMove_RequestOrbit", &PROBE_N, &[(p_chr_1) as u32, (p_near_2).0, (p_far_3).0], false); }
    let mut l_pose_4: u32 = 0;
    let r_ = 'f: {
    l_pose_4 = F::BtlChar_GetPos(c, p_chr_1);
    if ((F::BtlChar_TestFlag(c, p_chr_1, ((((5i32) as i64 as u64)) as u32))) != 0) {
        F::BtlChar_SetFlag(c, p_chr_1, ((((25i32) as i64 as u64)) as u32));
        { let v_ = p_near_2; let a_ = (l_pose_4).wrapping_add(220); c.m.w32(a_, (v_).0); };
        { let v_ = p_far_3; let a_ = (l_pose_4).wrapping_add(224); c.m.w32(a_, (v_).0); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMove_RequestOrbit", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMove_ApplyOrbit(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMove_ApplyOrbit", &PROBE_N, &[(p_chr_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(144) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 144);
    let mut l_pose_2: u32 = 0;
    let mut l_near_3: F32 = F32(0);
    let mut l_far_4: F32 = F32(0);
    let mut l_w_5: F32 = F32(0);
    let mut l_lenBase_6: F32 = F32(0);
    let mut l_len_7: F32 = F32(0);
    let mut l_lenE_8: F32 = F32(0);
    let mut l_c_9: F32 = F32(0);
    let mut l_a_10: F32 = F32(0);
    let r_ = 'f: {
        l_pose_2 = F::BtlChar_GetPos(c, p_chr_1);
        l_near_3 = F32(c.m.r32((l_pose_2).wrapping_add(220)));
        l_far_4 = F32(c.m.r32((l_pose_2).wrapping_add(224)));
        if ((((((F::BtlChar_TestFlag(c, p_chr_1, ((((5i32) as i64 as u64)) as u32))) != 0) && ((F::BtlChar_TestFlag(c, p_chr_1, ((((25i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) {
            F::BtlChar_GetSnapPos(c, p_chr_1, fp.wrapping_add(16), 0i32);
            l_w_5 = F32(0x3f800000);
            F::Vec4_Copy(c, fp, l_pose_2);
            F::BtlOpp_GetTargetPos(c, p_chr_1, fp.wrapping_add(32));
            F::Vec4_Sub(c, fp.wrapping_add(48), fp.wrapping_add(32), fp.wrapping_add(16));
            { let v_ = F32(0x00000000); let a_ = (fp.wrapping_add(48)).wrapping_add(4); c.m.w32(a_, (v_).0); };
            F::Vec4_Sub(c, fp.wrapping_add(64), fp.wrapping_add(32), fp);
            { let v_ = F32(0x00000000); let a_ = (fp.wrapping_add(64)).wrapping_add(4); c.m.w32(a_, (v_).0); };
            l_lenBase_6 = F::Vec3_Length(c, fp.wrapping_add(48));
            l_len_7 = F::Vec3_Length(c, fp.wrapping_add(64));
            if ((((!(((rt::flt(l_far_4, l_len_7) as i32)) != 0)) as i32)) != 0) {
                if ((((!(((rt::flt(l_near_3, l_len_7) as i32)) != 0)) as i32)) != 0) {
                    break 'f;
                }
                l_w_5 = rt::fdiv(rt::fsub(l_len_7, l_near_3), rt::fsub(l_far_4, l_near_3));
            }
            if (((rt::flt(l_lenBase_6, l_near_3) as i32)) != 0) {
                break 'f;
            }
            { let a2_ = rt::fsub(F32(0x3f800000), rt::fdiv(F::Vec3_Dot(c, fp.wrapping_add(48), fp.wrapping_add(64)), rt::fmul(l_lenBase_6, l_lenBase_6))); F::Vec4_Scale(c, fp.wrapping_add(96), fp.wrapping_add(48), a2_) };
            F::Vec4_Add(c, fp.wrapping_add(80), fp.wrapping_add(64), fp.wrapping_add(96));
            l_lenE_8 = F::Vec3_Length(c, fp.wrapping_add(80));
            if (((rt::flt(l_lenE_8, F32(0x3a83126e)) as i32)) != 0) {
                break 'f;
            }
            F::Vec4_Scale(c, fp.wrapping_add(112), fp.wrapping_add(80), rt::fmul(rt::fdiv(rt::fsub(l_lenE_8, l_lenBase_6), l_lenE_8), l_w_5));
            F::Vec4_Add(c, l_pose_2, l_pose_2, fp.wrapping_add(112));
            l_c_9 = rt::fdiv(F::Vec3_Dot(c, fp.wrapping_add(48), fp.wrapping_add(80)), rt::fmul(l_lenBase_6, l_lenE_8));
            if (((rt::flt(F32(0x3f800000), l_c_9) as i32)) != 0) {
                l_c_9 = F32(0x3f800000);
            }
            l_a_10 = F::Mathf_Acos(c, l_c_9);
            if (((rt::flt(l_w_5, F32(0x3f800000)) as i32)) != 0) {
                l_a_10 = rt::fmul(l_a_10, l_w_5);
                { let _ = F::Mathf_Cos(c, l_a_10); };
            }
            if (((rt::flt(rt::fsub(rt::fmul(F32(c.m.r32((fp.wrapping_add(48)).wrapping_add(8))), F32(c.m.r32(fp.wrapping_add(80)))), rt::fmul(F32(c.m.r32(fp.wrapping_add(48))), F32(c.m.r32((fp.wrapping_add(80)).wrapping_add(8))))), F32(0x00000000)) as i32)) != 0) {
                l_a_10 = rt::fneg(l_a_10);
            }
            F::Vec4_Set(c, fp.wrapping_add(128), F32(0x00000000), l_a_10, F32(0x00000000), F32(0x00000000));
            F::Vec4_RotateEuler(c, (l_pose_2).wrapping_add(128), fp.wrapping_add(128), (l_pose_2).wrapping_add(128));
            { let v_ = { let a0_ = rt::fadd(F32(c.m.r32(((l_pose_2).wrapping_add(16)).wrapping_add(4))), l_a_10); F::BtlUtil_WrapAngle(c, a0_) }; let a_ = ((l_pose_2).wrapping_add(16)).wrapping_add(4); c.m.w32(a_, (v_).0); };
            { let v_ = { let a0_ = rt::fadd(F32(c.m.r32((l_pose_2).wrapping_add(148))), l_a_10); F::BtlUtil_WrapAngle(c, a0_) }; let a_ = (l_pose_2).wrapping_add(148); c.m.w32(a_, (v_).0); };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlMove_ApplyOrbit", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMove_PushOut(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMove_PushOut", &PROBE_N, &[(p_chr_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(224) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 224);
    let mut l_pose_2: u32 = 0;
    let mut l_checkHeight_3: i32 = 0;
    let mut l_len_4: F32 = F32(0);
    let mut l_inv_5: F32 = F32(0);
    let mut l_sum_6: F32 = F32(0);
    let mut l_ratio_7: F32 = F32(0);
    let mut l_depth_8: F32 = F32(0);
    let r_ = 'f: {
        l_pose_2 = F::BtlChar_GetPos(c, p_chr_1);
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((63i32) as i64 as u64)) as u32))) != 0) {
            break 'f 0i32;
        }
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((174i32) as i64 as u64)) as u32))) != 0) {
            break 'f 0i32;
        }
        l_checkHeight_3 = ((!((F::BtlChar_TestFlag(c, p_chr_1, ((((5i32) as i64 as u64)) as u32))) != 0)) as i32);
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((26i32) as i64 as u64)) as u32))) != 0) {
            l_checkHeight_3 = 1i32;
        }
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((173i32) as i64 as u64)) as u32))) != 0) {
            l_checkHeight_3 = 1i32;
        }
        F::BtlChar_GetSnapPos(c, p_chr_1, fp.wrapping_add(64), 0i32);
        F::BtlOpp_GetSnapPos(c, p_chr_1, fp.wrapping_add(80), 0i32);
        F::Vec4_Sub(c, fp.wrapping_add(16), fp.wrapping_add(80), fp.wrapping_add(64));
        if ((l_checkHeight_3) != 0) {
            { let v_ = { let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); F::BtlCharApi_GetHeight(c, a0_) }; let a_ = (fp.wrapping_add(96)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            { let v_ = F::BtlOpp_GetHeight(c, p_chr_1); let a_ = (fp.wrapping_add(96)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            if (((((((rt::flt(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))), rt::fneg(F32(c.m.r32((fp.wrapping_add(96)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))))) as i32)) != 0) || (((rt::flt(F32(c.m.r32((fp.wrapping_add(96)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4)))) as i32)) != 0)) as i32)) != 0) {
                break 'f 0i32;
            }
        }
        F::Vec4_Copy(c, fp.wrapping_add(32), fp.wrapping_add(16));
        { let v_ = F32(0x00000000); let a_ = (fp.wrapping_add(32)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        l_len_4 = F::Vec3_Length(c, fp.wrapping_add(32));
        if (((rt::flt(l_len_4, F32(0x3a83126e)) as i32)) != 0) {
            break 'f 0i32;
        }
        l_inv_5 = rt::fdiv(F32(0x3f800000), l_len_4);
        { let r_ = l_inv_5; let a_ = fp.wrapping_add(32); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        { let r_ = l_inv_5; let a_ = (fp.wrapping_add(32)).wrapping_add(8); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        F::BtlOpp_GetSnapPos(c, p_chr_1, fp.wrapping_add(120), 0i32);
        F::BtlOpp_GetSnapPos(c, p_chr_1, fp.wrapping_add(136), 1i32);
        F::Vec4_Sub(c, fp.wrapping_add(104), fp.wrapping_add(136), fp.wrapping_add(120));
        { let v_ = F32(0x00000000); let a_ = (fp.wrapping_add(104)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        if (((rt::flt(F::Vec3_Dot(c, fp.wrapping_add(104), fp.wrapping_add(32)), F32(0x00000000)) as i32)) != 0) {
            F::Vec4_Copy(c, fp, fp.wrapping_add(120));
        } else {
            F::Vec4_Copy(c, fp, fp.wrapping_add(136));
        }
        { let v_ = rt::fneg(F32(c.m.r32(fp.wrapping_add(32)))); let a_ = fp.wrapping_add(48); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (fp.wrapping_add(48)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fneg(F32(c.m.r32((fp.wrapping_add(32)).wrapping_add(8)))); let a_ = (fp.wrapping_add(48)).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fneg(F::Vec3_Dot(c, fp.wrapping_add(48), fp)); let a_ = (fp.wrapping_add(48)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let r_ = { let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); F::BtlCharApi_GetRadius(c, a0_) }; let a_ = (fp.wrapping_add(48)).wrapping_add(12); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        { let r_ = F::BtlOpp_GetRadius(c, p_chr_1); let a_ = (fp.wrapping_add(48)).wrapping_add(12); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        F::BtlChar_GetSnapDelta(c, p_chr_1, fp.wrapping_add(152), 0i32, 1i32);
        F::BtlOpp_GetSnapPos(c, p_chr_1, fp.wrapping_add(184), 1i32);
        F::Vec4_Sub(c, fp.wrapping_add(168), fp.wrapping_add(184), fp);
        { let v_ = F32(0x00000000); let a_ = (fp.wrapping_add(152)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (fp.wrapping_add(168)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = F::Vec3_Dot(c, fp.wrapping_add(32), fp.wrapping_add(152)); let a_ = (fp.wrapping_add(96)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fneg(F::Vec3_Dot(c, fp.wrapping_add(32), fp.wrapping_add(168))); let a_ = (fp.wrapping_add(96)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        if (((rt::flt(F32(c.m.r32((fp.wrapping_add(96)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(0x00000000)) as i32)) != 0) {
            { let v_ = F32(0x00000000); let a_ = (fp.wrapping_add(96)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        }
        if (((rt::flt(F32(c.m.r32((fp.wrapping_add(96)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(0x00000000)) as i32)) != 0) {
            { let v_ = F32(0x00000000); let a_ = (fp.wrapping_add(96)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        }
        l_sum_6 = rt::fadd(F32(c.m.r32((fp.wrapping_add(96)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((fp.wrapping_add(96)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))));
        if (((rt::flt(l_sum_6, F32(0x3a83126e)) as i32)) != 0) {
            l_ratio_7 = F32(0x3f000000);
        } else {
            l_ratio_7 = rt::fdiv(F32(c.m.r32((fp.wrapping_add(96)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), l_sum_6);
        }
        l_depth_8 = rt::fsub(rt::fmul(rt::fadd(rt::fadd(F::Vec3_Dot(c, l_pose_2, fp.wrapping_add(48)), F32(c.m.r32((fp.wrapping_add(48)).wrapping_add(12)))), F32(c.m.r32((fp.wrapping_add(96)).wrapping_add((((0i32) as u32)).wrapping_mul(4))))), l_ratio_7), F32(c.m.r32((fp.wrapping_add(96)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))));
        if (((rt::flt(l_depth_8, F32(0x00000000)) as i32)) != 0) {
            F::Vec4_Scale(c, fp.wrapping_add(200), fp.wrapping_add(32), l_depth_8);
            F::Vec4_Add(c, l_pose_2, l_pose_2, fp.wrapping_add(200));
            F::BtlChar_SetFlag(c, p_chr_1, ((((94i32) as i64 as u64)) as u32));
            if ((l_checkHeight_3) != 0) {
                F::BtlChar_SetFlag(c, p_chr_1, ((((95i32) as i64 as u64)) as u32));
            }
            break 'f 1i32;
        }
        break 'f 0i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlMove_PushOut", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMove_CalcVerticalSpeed(c: &mut Ctx, mut p_height_1: F32, mut p_extra_2: F32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMove_CalcVerticalSpeed", &PROBE_N, &[(p_height_1).0, (p_extra_2).0], false); }
    let mut l_v_3: F32 = F32(0);
    let r_ = 'f: {
    l_v_3 = F32(0x00000000);
    {
        'l1: loop {
            'c2: {
                if (((rt::flt(F32(0x00000000), p_height_1) as i32)) != 0) {
                    { let r_ = l_v_3; let n_ = rt::fsub(p_height_1, r_); p_height_1 = n_; };
                    { let r_ = F32(0x3f6d0979); let n_ = rt::fadd(l_v_3, r_); l_v_3 = n_; };
                    if (((rt::flt(p_height_1, F32(0x00000000)) as i32)) != 0) {
                        { let r_ = rt::fsub(rt::fmul(rt::fsub(F32(0x3f800000), rt::fdiv(rt::fneg(p_height_1), l_v_3)), F32(0x3f6d0979)), F32(0x3f6d0979)); let n_ = rt::fadd(l_v_3, r_); l_v_3 = n_; };
                        break 'l1;
                    }
                } else {
                    { let r_ = l_v_3; let n_ = rt::fsub(p_height_1, r_); p_height_1 = n_; };
                    { let r_ = F32(0x3f6d0979); let n_ = rt::fsub(l_v_3, r_); l_v_3 = n_; };
                    if (((rt::flt(F32(0x00000000), p_height_1) as i32)) != 0) {
                        { let r_ = rt::fsub(rt::fmul(rt::fsub(F32(0x3f800000), rt::fdiv(rt::fneg(p_height_1), l_v_3)), F32(0x3f6d0979)), F32(0x3f6d0979)); let n_ = rt::fsub(l_v_3, r_); l_v_3 = n_; };
                        break 'l1;
                    }
                }
            }
        }
    }
    { let r_ = p_extra_2; let n_ = rt::fadd(l_v_3, r_); l_v_3 = n_; };
    if (((rt::flt(F32(0x40ed0979), l_v_3) as i32)) != 0) {
        l_v_3 = F32(0x40ed0979);
    }
    if (((rt::flt(l_v_3, F32(0xc0ed0979)) as i32)) != 0) {
        l_v_3 = F32(0xc0ed0979);
    }
    break 'f l_v_3;
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMove_CalcVerticalSpeed", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMove_AimVerticalAtOpponent(c: &mut Ctx, mut p_chr_1: u32, mut p_lead_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMove_AimVerticalAtOpponent", &PROBE_N, &[(p_chr_1) as u32, (p_lead_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_v_3: F32 = F32(0);
    let r_ = 'f: {
        F::BtlOpp_GetDelta(c, p_chr_1, fp);
        if ((p_lead_2) != 0) {
            F::BtlOpp_GetVelocity(c, p_chr_1, fp.wrapping_add(16));
            l_v_3 = { let a0_ = rt::fadd(F32(c.m.r32((fp).wrapping_add(4))), F::BtlOpp_GetHalfHeightDiff(c, p_chr_1)); let a1_ = rt::fmul(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))), F32(0x3f000000)); F::BtlMove_CalcVerticalSpeed(c, a0_, a1_) };
        } else {
            l_v_3 = { let a0_ = rt::fadd(F32(c.m.r32((fp).wrapping_add(4))), F::BtlOpp_GetHalfHeightDiff(c, p_chr_1)); F::BtlMove_CalcVerticalSpeed(c, a0_, F32(0x00000000)) };
        }
        { let v_ = l_v_3; let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(156); c.m.w32(a_, (v_).0); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlMove_AimVerticalAtOpponent", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMove_BeginRiseToOpponent(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMove_BeginRiseToOpponent", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    if ((((!((F::BtlChar_TestFlag(c, p_chr_1, ((((138i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) {
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((5i32) as i64 as u64)) as u32))) != 0) {
            F::BtlMove_TurnYaw(c, p_chr_1, 2i32, F32(0x40490fda));
            F::BtlMove_TurnPitch(c, p_chr_1, 5i32, F32(0x40490fda));
            F::BtlMove_SetDirection(c, p_chr_1, 2i32);
            F::BtlChar_SetHeldFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32));
            F::BtlMove_AimVerticalAtOpponent(c, p_chr_1, 1i32);
            F::BtlChar_SetHeldFlag(c, p_chr_1, ((((129i32) as i64 as u64)) as u32));
        }
    }
    F::BtlChar_SetHeldFlag(c, p_chr_1, ((((130i32) as i64 as u64)) as u32));
    F::BtlChar_SetHeldFlag(c, p_chr_1, ((((138i32) as i64 as u64)) as u32));
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMove_BeginRiseToOpponent", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMove_CalcJumpSpeed(c: &mut Ctx, mut p_height_1: F32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMove_CalcJumpSpeed", &PROBE_N, &[(p_height_1).0], false); }
    let mut l_v_2: F32 = F32(0);
    let r_ = 'f: {
    l_v_2 = F32(0x00000000);
    {
        'l1: loop {
            'c2: {
                { let r_ = l_v_2; let n_ = rt::fsub(p_height_1, r_); p_height_1 = n_; };
                { let r_ = F32(0x3eed0979); let n_ = rt::fadd(l_v_2, r_); l_v_2 = n_; };
                if (((rt::flt(p_height_1, F32(0x00000000)) as i32)) != 0) {
                    { let r_ = rt::fsub(rt::fmul(rt::fsub(F32(0x3f800000), rt::fdiv(rt::fneg(p_height_1), l_v_2)), F32(0x3eed0979)), F32(0x3eed0979)); let n_ = rt::fadd(l_v_2, r_); l_v_2 = n_; };
                    break 'l1;
                }
            }
        }
    }
    break 'f rt::fneg(l_v_2);
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMove_CalcJumpSpeed", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMove_SetImpulse(c: &mut Ctx, mut p_chr_1: u32, mut p_vel_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMove_SetImpulse", &PROBE_N, &[(p_chr_1) as u32, (p_vel_2) as u32], false); }
    let mut l_cur_3: u32 = 0;
    let r_ = 'f: {
    l_cur_3 = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(112);
    if (((rt::flt(F::Vec3_Length(c, l_cur_3), F::Vec3_Length(c, p_vel_2)) as i32)) != 0) {
        F::Vec4_Copy(c, l_cur_3, p_vel_2);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMove_SetImpulse", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMove_SetImpulseDir(c: &mut Ctx, mut p_chr_1: u32, mut p_dir_2: u32, mut p_speed_3: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMove_SetImpulseDir", &PROBE_N, &[(p_chr_1) as u32, (p_dir_2) as u32, (p_speed_3).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        F::Vec4_Scale(c, fp, p_dir_2, p_speed_3);
        F::BtlMove_SetImpulse(c, p_chr_1, fp);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlMove_SetImpulseDir", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMove_SetImpulseYaw(c: &mut Ctx, mut p_chr_1: u32, mut p_yaw_2: F32, mut p_speed_3: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMove_SetImpulseYaw", &PROBE_N, &[(p_chr_1) as u32, (p_yaw_2).0, (p_speed_3).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        { let v_ = F::Mathf_Sin(c, p_yaw_2); let a_ = fp; c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = F::Mathf_Cos(c, p_yaw_2); let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (fp).wrapping_add(12); c.m.w32(a_, (v_).0); };
        F::BtlMove_SetImpulseDir(c, p_chr_1, fp, p_speed_3);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlMove_SetImpulseYaw", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMove_DeflectAtStageLimit(c: &mut Ctx, mut p_chr_1: u32, mut p_back_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMove_DeflectAtStageLimit", &PROBE_N, &[(p_chr_1) as u32, (p_back_2) as u32], false); }
    let mut l_pose_3: u32 = 0;
    let mut l_a_4: F32 = F32(0);
    let r_ = 'f: {
    l_pose_3 = F::BtlChar_GetPos(c, p_chr_1);
    if ((((((F::BtlChar_TestFlag(c, p_chr_1, ((((20i32) as i64 as u64)) as u32))) != 0) && (((rt::flt(F32(0x00000000), F::BtlUtil_LengthXZ(c, l_pose_3)) as i32)) != 0)) as i32)) != 0) {
        if (((rt::flt(F32(0x00000000), rt::fsub(rt::fmul(F32(c.m.r32(l_pose_3)), F32(c.m.r32(((l_pose_3).wrapping_add(48)).wrapping_add(8)))), rt::fmul(F32(c.m.r32((l_pose_3).wrapping_add(8))), F32(c.m.r32((l_pose_3).wrapping_add(48)))))) as i32)) != 0) {
            l_a_4 = { let a0_ = F32(c.m.r32((l_pose_3).wrapping_add(8))); let a1_ = rt::fneg(F32(c.m.r32(l_pose_3))); F::Port_atan2f(c, a0_, a1_) };
        } else {
            l_a_4 = { let a0_ = rt::fneg(F32(c.m.r32((l_pose_3).wrapping_add(8)))); let a1_ = F32(c.m.r32(l_pose_3)); F::Port_atan2f(c, a0_, a1_) };
        }
        if ((p_back_2) != 0) {
            l_a_4 = F::BtlUtil_WrapAngle(c, rt::fadd(l_a_4, F32(0x40490fda)));
        }
        { let v_ = l_a_4; let a_ = (l_pose_3).wrapping_add(148); c.m.w32(a_, (v_).0); };
    }
    if ((F::BtlChar_TestFlag(c, p_chr_1, ((((21i32) as i64 as u64)) as u32))) != 0) {
        if ((p_back_2) != 0) {
            if (((rt::flt(F32(0x00000000), F32(c.m.r32((l_pose_3).wrapping_add(144)))) as i32)) != 0) {
                { let v_ = F32(0x00000000); let a_ = (l_pose_3).wrapping_add(144); c.m.w32(a_, (v_).0); };
            }
        } else {
            if (((rt::flt(F32(c.m.r32((l_pose_3).wrapping_add(144))), F32(0x00000000)) as i32)) != 0) {
                { let v_ = F32(0x00000000); let a_ = (l_pose_3).wrapping_add(144); c.m.w32(a_, (v_).0); };
            }
        }
    }
    if ((F::BtlChar_TestFlag(c, p_chr_1, ((((22i32) as i64 as u64)) as u32))) != 0) {
        if ((p_back_2) != 0) {
            if (((rt::flt(F32(c.m.r32((l_pose_3).wrapping_add(144))), F32(0x00000000)) as i32)) != 0) {
                { let v_ = F32(0x00000000); let a_ = (l_pose_3).wrapping_add(144); c.m.w32(a_, (v_).0); };
            }
        } else {
            if (((rt::flt(F32(0x00000000), F32(c.m.r32((l_pose_3).wrapping_add(144)))) as i32)) != 0) {
                { let v_ = F32(0x00000000); let a_ = (l_pose_3).wrapping_add(144); c.m.w32(a_, (v_).0); };
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMove_DeflectAtStageLimit", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMove_ApplyImpulse(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMove_ApplyImpulse", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_pose_2: u32 = 0;
    let mut l_vel_3: u32 = 0;
    let mut l_len_4: F32 = F32(0);
    let r_ = 'f: {
    l_pose_2 = F::BtlChar_GetPos(c, p_chr_1);
    if ((((!((F::BtlChars_IsTimeStopped(c)) != 0)) as i32)) != 0) {
        l_vel_3 = (l_pose_2).wrapping_add(112);
        F::Vec4_Add(c, l_pose_2, l_pose_2, l_vel_3);
        { let v_ = F32(0x3f800000); let a_ = (l_pose_2).wrapping_add(12); c.m.w32(a_, (v_).0); };
        l_len_4 = F::Vec3_Length(c, l_vel_3);
        if (((rt::flt(l_len_4, F32(0x3f1e0650)) as i32)) != 0) {
            F::Vec4_SetZero(c, l_vel_3);
        } else {
            F::Vec4_Scale(c, l_vel_3, l_vel_3, rt::fdiv(rt::fsub(l_len_4, F32(0x3f1e0650)), l_len_4));
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMove_ApplyImpulse", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMove_UpdateHoverOffset(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMove_UpdateHoverOffset", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_pose_2: u32 = 0;
    let mut l_a_3: F32 = F32(0);
    let mut l_k_4: F32 = F32(0);
    let r_ = 'f: {
    l_pose_2 = F::BtlChar_GetPos(c, p_chr_1);
    F::Vec4_SetZero(c, (l_pose_2).wrapping_add(32));
    if ((F::BtlChar_TestFlag(c, p_chr_1, ((((41i32) as i64 as u64)) as u32))) != 0) {
        { let v_ = F32(0x00000000); let a_ = (l_pose_2).wrapping_add(168); c.m.w32(a_, (v_).0); };
    }
    if ((F::BtlChars_IsTimeStopped(c)) != 0) {
        break 'f;
    }
    if ((((((F::BtlChar_TestFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32))) != 0) && ((F::BtlChar_TestFlag(c, p_chr_1, ((((28i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) {
        { let v_ = { let a0_ = rt::fadd(F32(c.m.r32((l_pose_2).wrapping_add(168))), F32(0x3dd6774e)); F::BtlUtil_WrapAngle(c, a0_) }; let a_ = (l_pose_2).wrapping_add(168); c.m.w32(a_, (v_).0); };
    } else {
        if (((rt::flt(F32(0x3fc90fda), F32(c.m.r32((l_pose_2).wrapping_add(168)))) as i32)) != 0) {
            { let v_ = rt::fsub(F32(0x40490fda), F32(c.m.r32((l_pose_2).wrapping_add(168)))); let a_ = (l_pose_2).wrapping_add(168); c.m.w32(a_, (v_).0); };
        }
        if (((rt::flt(F32(c.m.r32((l_pose_2).wrapping_add(168))), F32(0xbfc90fda)) as i32)) != 0) {
            { let v_ = rt::fsub(F32(0xc0490fda), F32(c.m.r32((l_pose_2).wrapping_add(168)))); let a_ = (l_pose_2).wrapping_add(168); c.m.w32(a_, (v_).0); };
        }
        if (((rt::flt(F32(0x00000000), F32(c.m.r32((l_pose_2).wrapping_add(168)))) as i32)) != 0) {
            { let r_ = F32(0x3dd6774e); let a_ = (l_pose_2).wrapping_add(168); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            if (((rt::flt(F32(c.m.r32((l_pose_2).wrapping_add(168))), F32(0x00000000)) as i32)) != 0) {
                { let v_ = F32(0x00000000); let a_ = (l_pose_2).wrapping_add(168); c.m.w32(a_, (v_).0); };
            }
        }
        if (((rt::flt(F32(c.m.r32((l_pose_2).wrapping_add(168))), F32(0x00000000)) as i32)) != 0) {
            { let r_ = F32(0x3dd6774e); let a_ = (l_pose_2).wrapping_add(168); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            if (((rt::flt(F32(0x00000000), F32(c.m.r32((l_pose_2).wrapping_add(168)))) as i32)) != 0) {
                { let v_ = F32(0x00000000); let a_ = (l_pose_2).wrapping_add(168); c.m.w32(a_, (v_).0); };
            }
        }
    }
    { let v_ = rt::fmul({ let a0_ = F32(c.m.r32((l_pose_2).wrapping_add(168))); F::Mathf_Sin(c, a0_) }, F32(0x40000000)); let a_ = ((l_pose_2).wrapping_add(32)).wrapping_add(4); c.m.w32(a_, (v_).0); };
    if ((((((c.m.r32((p_chr_1).wrapping_add(4072)) as i32)) > (0i32)) as i32)) != 0) {
        l_k_4 = rt::fmul(rt::fsub(rt::i2f((((c.m.r32((p_chr_1).wrapping_add(4072)) as i32)) & (1i32))), F32(0x3f000000)), F32(0x3f000000));
        { let v_ = rt::fmul(rt::fneg({ let a0_ = F32(c.m.r32((l_pose_2).wrapping_add(148))); F::Mathf_Cos(c, a0_) }), l_k_4); let a_ = (l_pose_2).wrapping_add(32); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul({ let a0_ = F32(c.m.r32((l_pose_2).wrapping_add(148))); F::Mathf_Sin(c, a0_) }, l_k_4); let a_ = ((l_pose_2).wrapping_add(32)).wrapping_add(8); c.m.w32(a_, (v_).0); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMove_UpdateHoverOffset", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMove_UpdateDefenseTimers(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMove_UpdateDefenseTimers", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_on_2: i32 = 0;
    let mut l_hp_3: F32 = F32(0);
    let r_ = 'f: {
    l_on_2 = F::BtlInput_TestAction(c, p_chr_1, 39i32, 1i32);
    l_hp_3 = F::BtlMember_GetHealthRatio(c, p_chr_1);
    if ((((!((F::BtlChar_TestFlag(c, p_chr_1, ((((5i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) {
        l_on_2 = 0i32;
    }
    if ((F::BtlChar_TestFlag(c, p_chr_1, ((((300i32) as i64 as u64)) as u32))) != 0) {
        { let v_ = (30i32).wrapping_neg(); let a_ = (p_chr_1).wrapping_add(4200); c.m.w32(a_, (v_) as u32); };
        { let v_ = (30i32).wrapping_neg(); let a_ = (p_chr_1).wrapping_add(4204); c.m.w32(a_, (v_) as u32); };
        { let v_ = (30i32).wrapping_neg(); let a_ = (p_chr_1).wrapping_add(4220); c.m.w32(a_, (v_) as u32); };
    }
    { let a_ = (p_chr_1).wrapping_add(4200); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
    if ((((((c.m.r32((p_chr_1).wrapping_add(4200)) as i32)) < ((30i32).wrapping_neg())) as i32)) != 0) {
        { let v_ = (30i32).wrapping_neg(); let a_ = (p_chr_1).wrapping_add(4200); c.m.w32(a_, (v_) as u32); };
        if ((l_on_2) != 0) {
            if (((rt::flt(l_hp_3, F32(0x3e999999)) as i32)) != 0) {
                { let v_ = 2i32; let a_ = (p_chr_1).wrapping_add(4200); c.m.w32(a_, (v_) as u32); };
            } else {
                if (((rt::flt(l_hp_3, F32(0x3f333333)) as i32)) != 0) {
                    { let v_ = 3i32; let a_ = (p_chr_1).wrapping_add(4200); c.m.w32(a_, (v_) as u32); };
                } else {
                    { let v_ = 4i32; let a_ = (p_chr_1).wrapping_add(4200); c.m.w32(a_, (v_) as u32); };
                }
            }
        }
    } else {
        if ((l_on_2) != 0) {
            { let v_ = (1i32).wrapping_neg(); let a_ = (p_chr_1).wrapping_add(4200); c.m.w32(a_, (v_) as u32); };
        }
        if ((((((c.m.r32((p_chr_1).wrapping_add(4016)) as i32)) != (1i32)) as i32)) != 0) {
            { let v_ = (30i32).wrapping_neg(); let a_ = (p_chr_1).wrapping_add(4200); c.m.w32(a_, (v_) as u32); };
        }
    }
    { let a_ = (p_chr_1).wrapping_add(4204); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
    if ((((((c.m.r32((p_chr_1).wrapping_add(4204)) as i32)) < ((30i32).wrapping_neg())) as i32)) != 0) {
        { let v_ = (30i32).wrapping_neg(); let a_ = (p_chr_1).wrapping_add(4204); c.m.w32(a_, (v_) as u32); };
        if ((l_on_2) != 0) {
            if ((F::BtlChar_TestFlag(c, p_chr_1, ((((312i32) as i64 as u64)) as u32))) != 0) {
                { let v_ = 4i32; let a_ = (p_chr_1).wrapping_add(4204); c.m.w32(a_, (v_) as u32); };
            } else {
                { let v_ = 2i32; let a_ = (p_chr_1).wrapping_add(4204); c.m.w32(a_, (v_) as u32); };
            }
        }
    } else {
        if ((l_on_2) != 0) {
            { let v_ = (1i32).wrapping_neg(); let a_ = (p_chr_1).wrapping_add(4204); c.m.w32(a_, (v_) as u32); };
        }
    }
    { let a_ = (p_chr_1).wrapping_add(4208); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
    if ((((((c.m.r32((p_chr_1).wrapping_add(4208)) as i32)) < ((30i32).wrapping_neg())) as i32)) != 0) {
        { let v_ = (30i32).wrapping_neg(); let a_ = (p_chr_1).wrapping_add(4208); c.m.w32(a_, (v_) as u32); };
        if ((F::BtlInput_TestAction(c, p_chr_1, 38i32, 1i32)) != 0) {
            { let v_ = 1i32; let a_ = (p_chr_1).wrapping_add(4208); c.m.w32(a_, (v_) as u32); };
        }
    } else {
        if ((F::BtlInput_TestAction(c, p_chr_1, 38i32, 1i32)) != 0) {
            { let v_ = (1i32).wrapping_neg(); let a_ = (p_chr_1).wrapping_add(4208); c.m.w32(a_, (v_) as u32); };
        }
    }
    { let a_ = (p_chr_1).wrapping_add(4212); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
    if ((((((c.m.r32((p_chr_1).wrapping_add(4212)) as i32)) < ((15i32).wrapping_neg())) as i32)) != 0) {
        { let v_ = (15i32).wrapping_neg(); let a_ = (p_chr_1).wrapping_add(4212); c.m.w32(a_, (v_) as u32); };
        if ((((((F::BtlInput_TestAction(c, p_chr_1, 43i32, 1i32)) != 0) && ((((((((F::BtlParam_GetFlags2(c, p_chr_1)) as u64)) as i32)) & (32i32))) != 0)) as i32)) != 0) {
            { let v_ = 5i32; let a_ = (p_chr_1).wrapping_add(4212); c.m.w32(a_, (v_) as u32); };
        }
    }
    { let a_ = (p_chr_1).wrapping_add(4216); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
    if ((((((c.m.r32((p_chr_1).wrapping_add(4216)) as i32)) < ((30i32).wrapping_neg())) as i32)) != 0) {
        { let v_ = (30i32).wrapping_neg(); let a_ = (p_chr_1).wrapping_add(4216); c.m.w32(a_, (v_) as u32); };
        if ((l_on_2) != 0) {
            { let v_ = 5i32; let a_ = (p_chr_1).wrapping_add(4216); c.m.w32(a_, (v_) as u32); };
        }
    }
    { let a_ = (p_chr_1).wrapping_add(4220); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
    if ((((((c.m.r32((p_chr_1).wrapping_add(4220)) as i32)) < ((30i32).wrapping_neg())) as i32)) != 0) {
        { let v_ = (30i32).wrapping_neg(); let a_ = (p_chr_1).wrapping_add(4220); c.m.w32(a_, (v_) as u32); };
        if ((l_on_2) != 0) {
            if ((F::BtlChar_TestFlag(c, p_chr_1, ((((312i32) as i64 as u64)) as u32))) != 0) {
                { let v_ = 4i32; let a_ = (p_chr_1).wrapping_add(4220); c.m.w32(a_, (v_) as u32); };
            } else {
                { let v_ = 2i32; let a_ = (p_chr_1).wrapping_add(4220); c.m.w32(a_, (v_) as u32); };
            }
        }
    } else {
        if ((l_on_2) != 0) {
            { let v_ = (1i32).wrapping_neg(); let a_ = (p_chr_1).wrapping_add(4220); c.m.w32(a_, (v_) as u32); };
        }
    }
    if ((((((((F::BtlParam_GetFlags2(c, p_chr_1)) as u64)) as i32)) & (8i32))) != 0) {
        { let a_ = (p_chr_1).wrapping_add(4224); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        if ((((((c.m.r32((p_chr_1).wrapping_add(4224)) as i32)) >= (2i32)) as i32)) != 0) {
            { let v_ = 1i32; let a_ = (p_chr_1).wrapping_add(4224); c.m.w32(a_, (v_) as u32); };
        }
        if ((l_on_2) != 0) {
            { let v_ = (15i32).wrapping_neg(); let a_ = (p_chr_1).wrapping_add(4224); c.m.w32(a_, (v_) as u32); };
        }
    } else {
        { let v_ = (1i32).wrapping_neg(); let a_ = (p_chr_1).wrapping_add(4224); c.m.w32(a_, (v_) as u32); };
    }
    if ((((((F::BtlChar_TestFlag(c, p_chr_1, ((((128i32) as i64 as u64)) as u32))) != 0) && ((F::BtlMember_HasAbility(c, p_chr_1, 56i32)) != 0)) as i32)) != 0) {
        { let v_ = 1i32; let a_ = (p_chr_1).wrapping_add(4204); c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMove_UpdateDefenseTimers", &PROBE_N, &[], true); }
    r_
}
