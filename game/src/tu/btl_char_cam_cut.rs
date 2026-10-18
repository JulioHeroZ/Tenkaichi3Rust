#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn ChrCam_CalcCut(c: &mut Ctx, mut p_chr_1: u32, mut p_eye_2: u32, mut p_rot_3: u32, mut p_target_4: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ChrCam_CalcCut", &PROBE_N, &[(p_chr_1) as u32, (p_eye_2) as u32, (p_rot_3) as u32, (p_target_4) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(96) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 96);
    let mut l_cut_5: u32 = 0;
    let mut l_frozen_6: i32 = 0;
    let mut l_cam_7: u32 = 0;
    let mut l_refreshed_8: i32 = 0;
    let mut l_usedOpp_9: i32 = 0;
    let mut l_usedMid_10: i32 = 0;
    let mut l_t_11: F32 = F32(0);
    let mut l_node_12: i32 = 0;
    let mut l_other_13: i32 = 0;
    let mut l_id_14: i32 = 0;
    let mut l_eps_15: F32 = F32(0);
    let mut l_yaw_16: F32 = F32(0);
    let mut l_pitch_17: F32 = F32(0);
    let mut l_dist_18: F32 = F32(0);
    let mut l_len_19: F32 = F32(0);
    let mut l_limit_20: F32 = F32(0);
    let mut l_a_21: F32 = F32(0);
    let r_ = 'f: {
        l_cut_5 = (p_chr_1).wrapping_add(1216);
        l_frozen_6 = F::BtlChar_TestFlag(c, p_chr_1, ((((180i32) as i64 as u64)) as u32));
        l_cam_7 = (p_chr_1).wrapping_add(1056);
        l_t_11 = F32(0x00000000);
        l_refreshed_8 = 0i32;
        l_usedOpp_9 = 0i32;
        l_usedMid_10 = 0i32;
        if ((((((c.m.r32((l_cut_5).wrapping_add(156)) as i32)) > (0i32)) as i32)) != 0) {
            l_t_11 = rt::fsub(F32(0x3f800000), rt::fdiv(rt::i2f((c.m.r32((l_cut_5).wrapping_add(152)) as i32)), rt::i2f((c.m.r32((l_cut_5).wrapping_add(156)) as i32))));
        }
        l_node_12 = (c.m.r32((l_cut_5).wrapping_add(136)) as i32);
        if (((((l_node_12) >= (0i32)) as i32)) != 0) {
            l_other_13 = 0i32;
            if ((((l_node_12) & (1073741824i32))) != 0) {
                l_usedOpp_9 = 1i32;
                l_id_14 = F::BtlOpp_GetObjId(c, p_chr_1);
                l_other_13 = 1i32;
            } else {
                if ((((l_node_12) & (536870912i32))) != 0) {
                    l_id_14 = (1i32).wrapping_neg();
                    l_usedMid_10 = 1i32;
                } else {
                    l_id_14 = (c.m.r32((p_chr_1).wrapping_add(12)) as i32);
                }
            }
            if (((((l_id_14) < (0i32)) as i32)) != 0) {
                { let a0_ = { let a_ = (F::BtlChar_Get(c, 0i32)).wrapping_add(12); (c.m.r32(a_) as i32) }; F::BtlCharApi_GetNodePos(c, a0_, ((((((l_node_12) as u32)) & (2684354559u32))) as i32), fp.wrapping_add(32)) };
                { let a0_ = { let a_ = (F::BtlChar_Get(c, 1i32)).wrapping_add(12); (c.m.r32(a_) as i32) }; F::BtlCharApi_GetNodePos(c, a0_, ((((((l_node_12) as u32)) & (2684354559u32))) as i32), fp.wrapping_add(48)) };
                F::Vec4_Lerp(c, (l_cut_5).wrapping_add(16), fp.wrapping_add(32), fp.wrapping_add(48), F32(0x3f000000));
            } else {
                if (((((((((l_frozen_6) == (0i32)) as i32)) != 0) || (((((l_other_13) == (0i32)) as i32)) != 0)) as i32)) != 0) {
                    { let a1_ = (((((((c.m.r32((l_cut_5).wrapping_add(136)) as i32)) as u32)) & (2684354559u32))) as i32); F::BtlCharApi_GetNodePos(c, l_id_14, a1_, (l_cut_5).wrapping_add(16)) };
                }
            }
            l_refreshed_8 = 1i32;
        }
        l_node_12 = (c.m.r32((l_cut_5).wrapping_add(140)) as i32);
        if (((((l_node_12) >= (0i32)) as i32)) != 0) {
            if ((((((c.m.r32((l_cut_5).wrapping_add(136)) as i32)) == (l_node_12)) as i32)) != 0) {
                F::Vec4_SetZeroW1(c, (l_cut_5).wrapping_add(32));
            } else {
                l_other_13 = 0i32;
                if ((((l_node_12) & (1073741824i32))) != 0) {
                    l_usedOpp_9 = 1i32;
                    l_id_14 = F::BtlOpp_GetObjId(c, p_chr_1);
                    l_other_13 = 1i32;
                } else {
                    if ((((l_node_12) & (536870912i32))) != 0) {
                        l_id_14 = (1i32).wrapping_neg();
                        l_usedMid_10 = 1i32;
                    } else {
                        l_id_14 = (c.m.r32((p_chr_1).wrapping_add(12)) as i32);
                    }
                }
                if (((((l_id_14) < (0i32)) as i32)) != 0) {
                    { let a0_ = { let a_ = (F::BtlChar_Get(c, 0i32)).wrapping_add(12); (c.m.r32(a_) as i32) }; F::BtlCharApi_GetNodePos(c, a0_, ((((((l_node_12) as u32)) & (2684354559u32))) as i32), fp.wrapping_add(64)) };
                    { let a0_ = { let a_ = (F::BtlChar_Get(c, 1i32)).wrapping_add(12); (c.m.r32(a_) as i32) }; F::BtlCharApi_GetNodePos(c, a0_, ((((((l_node_12) as u32)) & (2684354559u32))) as i32), fp.wrapping_add(80)) };
                    F::Vec4_Lerp(c, fp.wrapping_add(32), fp.wrapping_add(64), fp.wrapping_add(80), F32(0x3f000000));
                } else {
                    if (((((((((l_frozen_6) == (0i32)) as i32)) != 0) || (((((l_other_13) == (0i32)) as i32)) != 0)) as i32)) != 0) {
                        { let a1_ = (((((((c.m.r32((l_cut_5).wrapping_add(140)) as i32)) as u32)) & (2684354559u32))) as i32); F::BtlCharApi_GetNodePos(c, l_id_14, a1_, fp.wrapping_add(32)) };
                    }
                }
                F::Vec4_Sub(c, (l_cut_5).wrapping_add(32), fp.wrapping_add(32), (l_cut_5).wrapping_add(16));
            }
            l_refreshed_8 = 1i32;
        }
        l_node_12 = (c.m.r32((l_cut_5).wrapping_add(144)) as i32);
        if (((((l_node_12) >= (0i32)) as i32)) != 0) {
            l_other_13 = 0i32;
            if ((((l_node_12) & (1073741824i32))) != 0) {
                l_id_14 = F::BtlOpp_GetObjId(c, p_chr_1);
                l_other_13 = 1i32;
            } else {
                if ((((l_node_12) & (536870912i32))) != 0) {
                    l_id_14 = (1i32).wrapping_neg();
                } else {
                    l_id_14 = (c.m.r32((p_chr_1).wrapping_add(12)) as i32);
                }
            }
            if (((((l_id_14) < (0i32)) as i32)) != 0) {
                { let a0_ = { let a_ = (F::BtlChar_Get(c, 0i32)).wrapping_add(12); (c.m.r32(a_) as i32) }; F::BtlCharApi_GetNodePos(c, a0_, ((((((l_node_12) as u32)) & (2684354559u32))) as i32), fp.wrapping_add(32)) };
                { let a0_ = { let a_ = (F::BtlChar_Get(c, 1i32)).wrapping_add(12); (c.m.r32(a_) as i32) }; F::BtlCharApi_GetNodePos(c, a0_, ((((((l_node_12) as u32)) & (2684354559u32))) as i32), fp.wrapping_add(48)) };
                F::Vec4_Lerp(c, (l_cut_5).wrapping_add(80), fp.wrapping_add(32), fp.wrapping_add(48), F32(0x3f000000));
            } else {
                if (((((((((l_frozen_6) == (0i32)) as i32)) != 0) || (((((l_other_13) == (0i32)) as i32)) != 0)) as i32)) != 0) {
                    F::BtlCharApi_GetNodePos(c, l_id_14, ((((((l_node_12) as u32)) & (2684354559u32))) as i32), (l_cut_5).wrapping_add(80));
                }
            }
        }
        l_node_12 = (c.m.r32((l_cut_5).wrapping_add(148)) as i32);
        if (((((l_node_12) >= (0i32)) as i32)) != 0) {
            if ((((((c.m.r32((l_cut_5).wrapping_add(144)) as i32)) == (l_node_12)) as i32)) != 0) {
                F::Vec4_SetZeroW1(c, (l_cut_5).wrapping_add(96));
            } else {
                l_other_13 = 0i32;
                if ((((l_node_12) & (1073741824i32))) != 0) {
                    l_id_14 = F::BtlOpp_GetObjId(c, p_chr_1);
                    l_other_13 = 1i32;
                } else {
                    if ((((l_node_12) & (536870912i32))) != 0) {
                        l_id_14 = (1i32).wrapping_neg();
                    } else {
                        l_id_14 = (c.m.r32((p_chr_1).wrapping_add(12)) as i32);
                    }
                }
                if (((((l_id_14) < (0i32)) as i32)) != 0) {
                    { let a0_ = { let a_ = (F::BtlChar_Get(c, 0i32)).wrapping_add(12); (c.m.r32(a_) as i32) }; F::BtlCharApi_GetNodePos(c, a0_, ((((((l_node_12) as u32)) & (2684354559u32))) as i32), fp.wrapping_add(48)) };
                    { let a0_ = { let a_ = (F::BtlChar_Get(c, 1i32)).wrapping_add(12); (c.m.r32(a_) as i32) }; F::BtlCharApi_GetNodePos(c, a0_, ((((((l_node_12) as u32)) & (2684354559u32))) as i32), fp.wrapping_add(64)) };
                    F::Vec4_Lerp(c, fp.wrapping_add(32), fp.wrapping_add(48), fp.wrapping_add(64), F32(0x3f000000));
                } else {
                    if (((((((((l_frozen_6) == (0i32)) as i32)) != 0) || (((((l_other_13) == (0i32)) as i32)) != 0)) as i32)) != 0) {
                        F::BtlCharApi_GetNodePos(c, l_id_14, ((((((l_node_12) as u32)) & (2684354559u32))) as i32), fp.wrapping_add(32));
                    }
                }
                F::Vec4_Sub(c, (l_cut_5).wrapping_add(96), fp.wrapping_add(32), (l_cut_5).wrapping_add(80));
            }
        }
        if ((l_refreshed_8) != 0) {
            if ((l_usedOpp_9) != 0) {
                { let a1_ = { let a_ = (F::BtlOpp_GetObj(c, p_chr_1)).wrapping_add(((4000i32) as u32)); c.m.r32(a_) }; F::Vec4_Copy(c, (l_cut_5).wrapping_add(48), a1_) };
            } else {
                if ((l_usedMid_10) != 0) {
                    { let a1_ = { let a_ = (F::BtlOpp_GetObj(c, p_chr_1)).wrapping_add(((4000i32) as u32)); c.m.r32(a_) }; F::Vec4_Copy(c, fp.wrapping_add(32), a1_) };
                    { let a1_ = { let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(((4000i32) as u32)); c.m.r32(a_) }; F::Vec4_Copy(c, fp.wrapping_add(48), a1_) };
                    F::Vec4_Lerp(c, (l_cut_5).wrapping_add(48), fp.wrapping_add(32), fp.wrapping_add(48), F32(0x3f000000));
                } else {
                    { let a1_ = { let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(((4000i32) as u32)); c.m.r32(a_) }; F::Vec4_Copy(c, (l_cut_5).wrapping_add(48), a1_) };
                }
            }
            F::Vec4_SetZeroW1(c, (l_cut_5).wrapping_add(64));
        }
        l_eps_15 = F32(0x3a83126e);
        F::Vec4_Scale(c, p_eye_2, (l_cut_5).wrapping_add(32), l_t_11);
        F::Vec4_Add(c, p_eye_2, (l_cut_5).wrapping_add(16), p_eye_2);
        F::Vec4_Scale(c, p_target_4, (l_cut_5).wrapping_add(64), l_t_11);
        F::Vec4_Add(c, p_target_4, (l_cut_5).wrapping_add(48), p_target_4);
        F::Vec4_Scale(c, fp, (l_cut_5).wrapping_add(96), l_t_11);
        F::Vec4_Add(c, fp, (l_cut_5).wrapping_add(80), fp);
        l_yaw_16 = rt::fadd(F32(c.m.r32((l_cut_5).wrapping_add(112))), rt::fmul(F32(c.m.r32((l_cut_5).wrapping_add(116))), l_t_11));
        l_pitch_17 = rt::fneg(rt::fadd(F32(c.m.r32((l_cut_5).wrapping_add(120))), rt::fmul(F32(c.m.r32((l_cut_5).wrapping_add(124))), l_t_11)));
        l_dist_18 = rt::fadd(F32(c.m.r32((l_cut_5).wrapping_add(128))), rt::fmul(F32(c.m.r32((l_cut_5).wrapping_add(132))), l_t_11));
        { let v_ = rt::fmul(F::Mathf_Cos(c, l_pitch_17), F::Mathf_Sin(c, l_yaw_16)); let a_ = fp.wrapping_add(16); c.m.w32(a_, (v_).0); };
        { let v_ = F::Mathf_Sin(c, l_pitch_17); let a_ = (fp.wrapping_add(16)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(F::Mathf_Cos(c, l_pitch_17), F::Mathf_Cos(c, l_yaw_16)); let a_ = (fp.wrapping_add(16)).wrapping_add(8); c.m.w32(a_, (v_).0); };
        F::Vec4_Scale(c, fp.wrapping_add(16), fp.wrapping_add(16), l_dist_18);
        F::Vec4_Sub(c, p_eye_2, p_eye_2, fp.wrapping_add(16));
        F::Vec4_SetZero(c, p_rot_3);
        F::Vec4_Sub(c, fp.wrapping_add(16), fp, p_eye_2);
        l_len_19 = F::Vec3_Length(c, fp.wrapping_add(16));
        if (((rt::flt(l_eps_15, l_len_19) as i32)) != 0) {
            F::Vec4_Scale(c, fp.wrapping_add(16), fp.wrapping_add(16), rt::fdiv(F32(0x3f800000), l_len_19));
            F::Vec3_Clamp(c, fp.wrapping_add(16), fp.wrapping_add(16), F32(0xbf800000), F32(0x3f800000));
            { let v_ = { let a0_ = rt::fneg(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4)))); F::Mathf_Asin(c, a0_) }; let a_ = p_rot_3; c.m.w32(a_, (v_).0); };
            if (((((((rt::flt({ let a0_ = F32(c.m.r32(fp.wrapping_add(16))); rt::ffabs(a0_) }, l_eps_15) as i32)) != 0) && (((rt::flt({ let a0_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8))); rt::ffabs(a0_) }, l_eps_15) as i32)) != 0)) as i32)) != 0) {
                { let v_ = F32(0x00000000); let a_ = (p_rot_3).wrapping_add(4); c.m.w32(a_, (v_).0); };
            } else {
                { let v_ = { let a0_ = F32(c.m.r32(fp.wrapping_add(16))); let a1_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8))); F::Port_atan2f(c, a0_, a1_) }; let a_ = (p_rot_3).wrapping_add(4); c.m.w32(a_, (v_).0); };
            }
        }
        if (((((c.m.r32((l_cut_5).wrapping_add(160)) as i32)) & (2i32))) != 0) {
            F::BtlChar_SetFlag(c, p_chr_1, ((((205i32) as i64 as u64)) as u32));
        }
        if (((((c.m.r32((l_cut_5).wrapping_add(160)) as i32)) & (16i32))) != 0) {
            F::BtlChar_SetFlag(c, p_chr_1, ((((209i32) as i64 as u64)) as u32));
        }
        if (((((c.m.r32((l_cut_5).wrapping_add(160)) as i32)) & (64i32))) != 0) {
            F::BtlChar_SetFlag(c, p_chr_1, ((((211i32) as i64 as u64)) as u32));
        }
        if ((((((c.m.r32((l_cut_5).wrapping_add(152)) as i32)) > (0i32)) as i32)) != 0) {
            { let a_ = (l_cut_5).wrapping_add(152); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
            if ((((((((((c.m.r32((l_cut_5).wrapping_add(152)) as i32)) == (0i32)) as i32)) != 0) && ((((!(((((c.m.r32((l_cut_5).wrapping_add(160)) as i32)) & (128i32))) != 0)) as i32)) != 0)) as i32)) != 0) {
                if (((((c.m.r32((l_cut_5).wrapping_add(160)) as i32)) & (4i32))) != 0) {
                    F::BtlChar_SetFlag(c, p_chr_1, ((((206i32) as i64 as u64)) as u32));
                }
                F::ChrCam_EndCut(c, p_chr_1);
            }
        }
        if (((((c.m.r32((l_cut_5).wrapping_add(160)) as i32)) & (256i32))) != 0) {
            { let a1_ = F::BtlChar_GetPos(c, p_chr_1); F::Vec4_Sub(c, fp.wrapping_add(64), a1_, l_cam_7) };
            if (((((((rt::flt(F32(0x3a83126e), { let a0_ = F32(c.m.r32(fp.wrapping_add(64))); rt::ffabs(a0_) }) as i32)) != 0) || (((rt::flt(F32(0x3a83126e), { let a0_ = F32(c.m.r32((fp.wrapping_add(64)).wrapping_add(8))); rt::ffabs(a0_) }) as i32)) != 0)) as i32)) != 0) {
                l_limit_20 = F::ChrCam_GetSideLimit(c, p_chr_1);
                l_a_21 = { let a0_ = F32(c.m.r32(fp.wrapping_add(64))); let a1_ = F32(c.m.r32((fp.wrapping_add(64)).wrapping_add(8))); F::Port_atan2f(c, a0_, a1_) };
                { let v_ = { let a0_ = rt::fneg({ let a0_ = rt::fsub(l_a_21, { let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(148); F32(c.m.r32(a_)) }); F::BtlUtil_WrapAngle(c, a0_) }); F::BtlUtil_ClampF(c, a0_, rt::fneg(l_limit_20), l_limit_20) }; let a_ = ((p_chr_1).wrapping_add(1056)).wrapping_add(136); c.m.w32(a_, (v_).0); };
            }
        }
        F::BtlChar_SetFlag(c, p_chr_1, ((((212i32) as i64 as u64)) as u32));
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "ChrCam_CalcCut", &PROBE_N, &[], true); }
    r_
}
