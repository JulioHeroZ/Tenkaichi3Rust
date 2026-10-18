#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn BtlAct_Throw(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_Throw", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let mut l_alt_3: i32 = 0;
    let mut l_radius_4: F32 = F32(0);
    let mut l_dist_5: F32 = F32(0);
    let r_ = 'f: {
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        l_alt_3 = (((F::BtlAct_GetCurrent(c, p_chr_1)) == (185i32)) as i32);
        { let a2_ = (if ((((((c.m.r32(((p_chr_1).wrapping_add(4016)).wrapping_add(16)) as i32)) != (0i32)) as i32)) != 0) { F32(0x00000000) } else { F32(0x3e999999) }); F::BtlAnim_Play(c, p_chr_1, (if ((l_alt_3) != 0) { 392i32 } else { 150i32 }), a2_) };
        F::BtlChar_PlayVoice(c, p_chr_1, 7i32);
        F::BtlChar_SetHeldFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32));
        F::BtlActThrow_SetupDamage(c, p_chr_1, 0i32);
        if ((((((((!((l_alt_3) != 0)) as i32)) != 0) && ((((!((((F::BtlAtk_GetFlags(c, p_chr_1)) & (((32768i32) as u32)))) != 0)) as i32)) != 0)) as i32)) != 0) {
            { let a2_ = (if ((F::BtlChar_FrameMod(c, 2i32)) != 0) { 7i32 } else { 8i32 }); F::ChrCam_RequestCut(c, p_chr_1, 0i32, a2_) };
        }
        if (((rt::flt({ let a0_ = F::BtlAtk_GetLaunchAngleA(c, p_chr_1); rt::ffabs(a0_) }, F32(0x3fc90fda)) as i32)) != 0) {
            l_radius_4 = F::BtlStage_GetInnerRadius(c);
            l_dist_5 = { let a0_ = F::BtlChar_GetPos(c, p_chr_1); F::BtlUtil_LengthXZ(c, a0_) };
            if (((rt::flt(rt::fsub(l_radius_4, F32(0x43480000)), l_dist_5) as i32)) != 0) {
                F::BtlMove_TurnYaw(c, p_chr_1, (if ((l_alt_3) != 0) { 8i32 } else { 7i32 }), F32(0x40490fda));
            }
        }
        if ((((((((({ let a_ = F::BtlMember_GetActive(c, p_chr_1); (c.m.r32(a_) as i32) }) == (110i32)) as i32)) != 0) && ((((!((((F::BtlOpp_GetParamWord0(c, p_chr_1)) & (128i32))) != 0)) as i32)) != 0)) as i32)) != 0) {
            { let v_ = { let a0_ = ((c.m.r32((p_chr_1).wrapping_add(3680)) as i32)).wrapping_add(1i32); F::BtlUtil_Clamp(c, a0_, 0i32, 10i32) }; let a_ = (p_chr_1).wrapping_add(3680); c.m.w32(a_, (v_) as u32); };
        }
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        's1: {
            let sv6: i32 = F::BtlAnim_GetId(c, p_chr_1);
            match sv6 {
                150i32 => {
                    { let _ = F::BtlAnim_AdvanceThen(c, p_chr_1, 152i32, F32(0x00000000), 1i32); };
                    F::BtlChar_SetFlag(c, p_chr_1, ((((139i32) as i64 as u64)) as u32));
                    break 's1;
                }
                392i32 => {
                    { let _ = F::BtlAnim_AdvanceThen(c, p_chr_1, 394i32, F32(0x00000000), 1i32); };
                    F::BtlChar_SetFlag(c, p_chr_1, ((((139i32) as i64 as u64)) as u32));
                    break 's1;
                }
                152i32 | 394i32 => {
                    if ((F::BtlAnim_Advance(c, p_chr_1, 0i32)) != 0) {
                        F::BtlAct_Request(c, p_chr_1, 11i32);
                    }
                    break 's1;
                }
                _ => {
                }
            }
        }
        F::BtlMove_Step(c, p_chr_1, 6i32, 5i32, 2i32, F32(0x00000000), F32(0x3f6d0979));
        F::BtlMove_ApplyGravity(c, p_chr_1);
        F::BtlChar_SetFlag(c, p_chr_1, ((((187i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((26i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((63i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((68i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((69i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((70i32) as i64 as u64)) as u32));
        if ((((F::BtlAtk_GetFlags(c, p_chr_1)) & (((1024i32) as u32)))) != 0) {
            F::BtlChar_SetFlag(c, p_chr_1, ((((315i32) as i64 as u64)) as u32));
        }
        if ((((((((((c.m.r32((p_chr_1).wrapping_add(3520)) as i32)) >= (0i32)) as i32)) != 0) && ((((((c.m.r32((p_chr_1).wrapping_add(3524)) as i32)) >= (0i32)) as i32)) != 0)) as i32)) != 0) {
            if (({ let a1_ = rt::i2f((c.m.r32((p_chr_1).wrapping_add(3520)) as i32)); let a2_ = rt::i2f((c.m.r32((p_chr_1).wrapping_add(3524)) as i32)); F::BtlAnim_InFrameRange(c, p_chr_1, a1_, a2_) }) != 0) {
                F::BtlCharSnd_PlayCommon(c, p_chr_1, 81i32);
                F::BtlChar_SetFxBit(c, p_chr_1, 56i32);
            }
            if (({ let a1_ = rt::i2f((c.m.r32((p_chr_1).wrapping_add(3524)) as i32)); F::BtlAnim_PassedFrame(c, p_chr_1, a1_) }) != 0) {
                F::BtlCharSnd_PlayCommon(c, p_chr_1, 82i32);
            }
            if ((((F::BtlAtk_GetFlags(c, p_chr_1)) & (((536870912i32) as u32)))) != 0) {
                F::BtlChar_SetFlag(c, p_chr_1, ((((316i32) as i64 as u64)) as u32));
            }
        }
    }
    if (((((p_phase_2) == (3i32)) as i32)) != 0) {
        F::BtlChar_SetFlag(c, p_chr_1, ((((140i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((51i32) as i64 as u64)) as u32));
        break 'f 0;
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_Throw", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_Thrown(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_Thrown", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let r_ = 'f: {
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        { let a1_ = (if (((((F::BtlAct_GetCurrent(c, p_chr_1)) == (186i32)) as i32)) != 0) { 393i32 } else { 151i32 }); let a2_ = (if ((((((c.m.r32(((p_chr_1).wrapping_add(4016)).wrapping_add(16)) as i32)) != (0i32)) as i32)) != 0) { F32(0x00000000) } else { F32(0x3e999999) }); F::BtlAnim_Play(c, p_chr_1, a1_, a2_) };
        F::BtlChar_SetHeldFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32));
        F::BtlActThrow_TurnByRootYaw(c, p_chr_1);
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        if ((F::BtlAnim_Advance(c, p_chr_1, 1i32)) != 0) {
            if ((((((c.m.r32(((p_chr_1).wrapping_add(4016)).wrapping_add(20)) as i32)) == (0i32)) as i32)) != 0) {
                F::BtlChar_PlayVoice(c, p_chr_1, 2i32);
            }
            if ((((((c.m.r32(((p_chr_1).wrapping_add(4016)).wrapping_add(12)) as i32)) != (0i32)) as i32)) != 0) {
                F::BtlChar_SetFlag(c, p_chr_1, ((((53i32) as i64 as u64)) as u32));
            }
            { let a1_ = (c.m.r32(((p_chr_1).wrapping_add(4016)).wrapping_add(4)) as i32); { let _ = F::BtlAct_QueueReaction(c, p_chr_1, ((((a1_) as i64 as u64)) as u32)); } };
            { let a1_ = F::BtlAct_GetQueued(c, p_chr_1); F::BtlAct_Request(c, p_chr_1, a1_) };
        }
        F::BtlMove_Step(c, p_chr_1, 9i32, 6i32, 3i32, F32(0x411425ec), F32(0x3f6d0979));
        F::BtlMove_ApplyGravity(c, p_chr_1);
        F::BtlChar_SetFlag(c, p_chr_1, ((((151i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((149i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((187i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((10i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((26i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((63i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((68i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((69i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((70i32) as i64 as u64)) as u32));
        if ((((((c.m.r32(((p_chr_1).wrapping_add(4016)).wrapping_add(20)) as i32)) == (0i32)) as i32)) != 0) {
            F::BtlChar_SetFlag(c, p_chr_1, ((((150i32) as i64 as u64)) as u32));
        }
        F::BtlChar_SetFlag(c, p_chr_1, ((((27i32) as i64 as u64)) as u32));
        F::BtlChar_SetFxBit(c, p_chr_1, 59i32);
        if ((((((((!((F::BtlChar_TestFlag(c, p_chr_1, ((((35i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) && ((F::BtlChar_TestPrevFlag(c, p_chr_1, ((((35i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) {
            F::BtlActThrow_SetReleaseHeading(c, p_chr_1);
            { let r_ = 1i32; let a_ = ((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        }
        if (((((c.m.r32(((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) & (1i32))) != 0) {
            { let a1_ = rt::fneg({ let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(144); F32(c.m.r32(a_)) }); F::BtlMove_SetLeanX(c, p_chr_1, a1_) };
        }
    }
    if (((((p_phase_2) == (3i32)) as i32)) != 0) {
        F::BtlChar_SetFlag(c, p_chr_1, ((((51i32) as i64 as u64)) as u32));
        if ((((!(((((c.m.r32(((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) & (1i32))) != 0)) as i32)) != 0) {
            F::BtlActThrow_SetReleaseHeading(c, p_chr_1);
            break 'f 0;
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_Thrown", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_SlamThrow(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_SlamThrow", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_timer_3: u32 = 0;
    let mut l_rise_4: i32 = 0;
    let mut l_end_5: i32 = 0;
    let mut l_room_6: F32 = F32(0);
    let r_ = 'f: {
        l_timer_3 = ((p_chr_1).wrapping_add(976)).wrapping_add((((2i32) as u32)).wrapping_mul(4));
        if (((((p_phase_2) == (0i32)) as i32)) != 0) {
            { let a2_ = (if ((((((c.m.r32(((p_chr_1).wrapping_add(4016)).wrapping_add(16)) as i32)) != (0i32)) as i32)) != 0) { F32(0x00000000) } else { F32(0x3e999999) }); F::BtlAnim_Play(c, p_chr_1, 150i32, a2_) };
            F::BtlActThrow_SetupDamage(c, p_chr_1, 1i32);
            F::BtlChar_PlayVoice(c, p_chr_1, 7i32);
            F::BtlChar_SetHeldFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32));
            F::ChrCam_RequestCut(c, p_chr_1, 0i32, 7i32);
        }
        if (((((p_phase_2) == (1i32)) as i32)) != 0) {
            l_rise_4 = 0i32;
            's1: {
                let sv7: i32 = F::BtlAnim_GetId(c, p_chr_1);
                match sv7 {
                    150i32 => {
                        { let _ = F::BtlAnim_AdvanceThen(c, p_chr_1, 153i32, F32(0x00000000), 1i32); };
                        if (((rt::fgt(F::BtlAnim_GetProgress(c, p_chr_1), F32(0x3f666666)) as i32)) != 0) {
                            l_rise_4 = 1i32;
                        }
                        F::BtlChar_SetFlag(c, p_chr_1, ((((139i32) as i64 as u64)) as u32));
                        break 's1;
                    }
                    153i32 => {
                        F::BtlAnim_AdvanceLoop(c, p_chr_1, 0i32);
                        if ((F::BtlAnim_IsNew(c, p_chr_1)) != 0) {
                            { let a1_ = F::BtlParam_GetDashSound(c, p_chr_1); F::BtlCharSnd_PlayCommon(c, p_chr_1, a1_) };
                            F::BtlChar_PlayVoice(c, p_chr_1, 7i32);
                            F::ChrCam_RequestCut(c, p_chr_1, 0i32, 8i32);
                            F::BtlChar_Vibrate(c, p_chr_1, F32(0x3f4ccccc), F32(0x3e999999));
                        } else {
                            F::BtlChar_SetVibration(c, p_chr_1, F32(0x3f333333), F32(0x3dcccccc));
                        }
                        F::BtlChar_SetFlag(c, p_chr_1, ((((139i32) as i64 as u64)) as u32));
                        F::BtlChar_SetFxBit(c, p_chr_1, 4i32);
                        l_rise_4 = 1i32;
                        break 's1;
                    }
                    152i32 => {
                        if ((F::BtlAnim_Advance(c, p_chr_1, 0i32)) != 0) {
                            F::BtlAct_Request(c, p_chr_1, 11i32);
                        }
                        break 's1;
                    }
                    _ => {
                    }
                }
            }
            F::BtlMove_Step(c, p_chr_1, 6i32, 5i32, 2i32, F32(0x00000000), F32(0x3f6d0979));
            if ((l_rise_4) != 0) {
                F::BtlMove_MoveVertical(c, p_chr_1, F32(0x415e38e2), F32(0x3f6d0979));
            } else {
                F::BtlMove_BrakeVertical(c, p_chr_1);
            }
            F::BtlChar_SetFlag(c, p_chr_1, ((((187i32) as i64 as u64)) as u32));
            F::BtlChar_SetFlag(c, p_chr_1, ((((26i32) as i64 as u64)) as u32));
            F::BtlChar_SetFlag(c, p_chr_1, ((((63i32) as i64 as u64)) as u32));
            F::BtlChar_SetFlag(c, p_chr_1, ((((68i32) as i64 as u64)) as u32));
            F::BtlChar_SetFlag(c, p_chr_1, ((((69i32) as i64 as u64)) as u32));
            F::BtlChar_SetFlag(c, p_chr_1, ((((70i32) as i64 as u64)) as u32));
        }
        if (((((p_phase_2) == (2i32)) as i32)) != 0) {
            's2: {
                let sv8: i32 = F::BtlAnim_GetId(c, p_chr_1);
                match sv8 {
                    150i32 | 152i32 => {
                        break 's2;
                    }
                    153i32 => {
                        l_end_5 = (((F::BtlChar_TestFlag(c, p_chr_1, ((((15i32) as i64 as u64)) as u32))) != (0i32)) as i32);
                        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((22i32) as i64 as u64)) as u32))) != 0) {
                            l_end_5 = 1i32;
                        }
                        if (((((F::BtlOpp_GetSeenAction(c, p_chr_1)) != (188i32)) as i32)) != 0) {
                            l_end_5 = 1i32;
                        }
                        if ((((((F::BtlChar_IsStage4Or27(c)) != 0) && ((F::BtlChar_TestFlag(c, p_chr_1, ((((23i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) {
                            l_end_5 = 1i32;
                        }
                        if ((((({ let a_ = l_timer_3; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); n_ }) > (10i32)) as i32)) != 0) {
                            if ((((((c.m.r32(l_timer_3) as i32)) > (90i32)) as i32)) != 0) {
                                l_end_5 = 1i32;
                            }
                            if ((((F::BtlParam_GetFlags(c, p_chr_1)) & (((1024i32) as u32)))) != 0) {
                                if ((F::BtlStage_GetWaterLevel(c, fp)) != 0) {
                                    l_room_6 = rt::fsub(F32(c.m.r32(fp)), { let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(4); F32(c.m.r32(a_)) });
                                    if (((rt::flt(l_room_6, rt::fmul({ let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); F::BtlCharApi_GetHeight(c, a0_) }, F32(0x40400000))) as i32)) != 0) {
                                        l_end_5 = 1i32;
                                    }
                                }
                            }
                        }
                        if ((l_end_5) != 0) {
                            F::BtlAnim_Request(c, p_chr_1, 152i32, F32(0x00000000));
                            { let v_ = F32(0x00000000); let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(156); c.m.w32(a_, (v_).0); };
                        }
                        break 's2;
                    }
                    _ => {
                    }
                }
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_SlamThrow", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_SlamThrown(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_SlamThrown", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_anim_3: u32 = 0;
    let mut l_track_4: u32 = 0;
    let mut l_rise_5: i32 = 0;
    let mut l_yaw_6: F32 = F32(0);
    let mut l_count_7: i32 = 0;
    let mut l_packed_8: u64 = 0;
    let mut l_key_9: u32 = 0;
    let mut l_key_10: u32 = 0;
    let r_ = 'f: {
        if (((((p_phase_2) == (0i32)) as i32)) != 0) {
            { let a2_ = (if ((((((c.m.r32(((p_chr_1).wrapping_add(4016)).wrapping_add(16)) as i32)) != (0i32)) as i32)) != 0) { F32(0x00000000) } else { F32(0x3e999999) }); F::BtlAnim_Play(c, p_chr_1, 151i32, a2_) };
            F::BtlChar_SetHeldFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32));
            l_anim_3 = { let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2880); c.m.r32(a_) };
            if (((((((((l_anim_3) != (((0i32) as u32))) as i32)) != 0) && (((((((c.m.r16((l_anim_3).wrapping_add(6))) as i32)) != (0i32)) as i32)) != 0)) as i32)) != 0) {
                l_yaw_6 = F32(0x00000000);
                l_track_4 = (l_anim_3).wrapping_add((((((c.m.r16((l_anim_3).wrapping_add(6))) as i32)) as u32)).wrapping_mul(4));
                l_count_7 = ((c.m.r16((l_track_4).wrapping_add(2))) as i32);
                if (((((l_count_7) > (0i32)) as i32)) != 0) {
                    if ((((((((c.m.r16(l_track_4)) as i32)) & (1i32))) as u16)) != 0) {
                        l_key_9 = (l_track_4).wrapping_add(4);
                        l_packed_8 = (((c.m.r32((l_key_9).wrapping_add((((0i32) as u32)).wrapping_mul(4)))) as u64)).wrapping_add((((c.m.r32((l_key_9).wrapping_add((((1i32) as u32)).wrapping_mul(4)))) as u64)).wrapping_shl((32i32) as u32));
                    } else {
                        l_key_10 = (l_track_4).wrapping_add(4);
                        l_packed_8 = (((c.m.r32((l_key_10).wrapping_add((((4i32) as u32)).wrapping_mul(4)))) as u64)).wrapping_add((((c.m.r32((l_key_10).wrapping_add((((5i32) as u32)).wrapping_mul(4)))) as u64)).wrapping_shl((32i32) as u32));
                    }
                    F::Quat_Unpack(c, fp, l_packed_8);
                    F::Quat_ToEuler(c, fp.wrapping_add(16), fp);
                    l_yaw_6 = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4)));
                }
                { let a1_ = { let a0_ = rt::fsub({ let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(148); F32(c.m.r32(a_)) }, l_yaw_6); F::BtlUtil_WrapAngle(c, a0_) }; F::BtlMove_SetHeading(c, p_chr_1, a1_, F32(0x00000000)) };
            }
        }
        if (((((p_phase_2) == (1i32)) as i32)) != 0) {
            l_rise_5 = 0i32;
            's1: {
                let sv11: i32 = F::BtlAnim_GetId(c, p_chr_1);
                match sv11 {
                    151i32 => {
                        { let _ = F::BtlAnim_AdvanceThen(c, p_chr_1, 154i32, F32(0x00000000), 1i32); };
                        F::BtlChar_SetFlag(c, p_chr_1, ((((151i32) as i64 as u64)) as u32));
                        break 's1;
                    }
                    154i32 => {
                        F::BtlAnim_AdvanceLoop(c, p_chr_1, 0i32);
                        F::BtlChar_SetFlag(c, p_chr_1, ((((151i32) as i64 as u64)) as u32));
                        F::BtlChar_SetFlag(c, p_chr_1, ((((10i32) as i64 as u64)) as u32));
                        l_rise_5 = 1i32;
                        break 's1;
                    }
                    155i32 => {
                        if ((F::BtlAnim_Advance(c, p_chr_1, 0i32)) != 0) {
                            { let v_ = 1i32; let a_ = ((p_chr_1).wrapping_add(976)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                        }
                        break 's1;
                    }
                    _ => {
                    }
                }
            }
            F::BtlMove_Step(c, p_chr_1, 9i32, 6i32, 3i32, F32(0x00000000), F32(0x3f6d0979));
            if ((l_rise_5) != 0) {
                F::BtlMove_MoveVertical(c, p_chr_1, F32(0x415e38e2), F32(0x461c4000));
            } else {
                F::BtlMove_ApplyGravity(c, p_chr_1);
            }
            F::BtlChar_SetFlag(c, p_chr_1, ((((149i32) as i64 as u64)) as u32));
            F::BtlChar_SetFlag(c, p_chr_1, ((((187i32) as i64 as u64)) as u32));
            F::BtlChar_SetFlag(c, p_chr_1, ((((26i32) as i64 as u64)) as u32));
            F::BtlChar_SetFlag(c, p_chr_1, ((((63i32) as i64 as u64)) as u32));
            F::BtlChar_SetFlag(c, p_chr_1, ((((68i32) as i64 as u64)) as u32));
            F::BtlChar_SetFlag(c, p_chr_1, ((((69i32) as i64 as u64)) as u32));
            F::BtlChar_SetFlag(c, p_chr_1, ((((70i32) as i64 as u64)) as u32));
            F::BtlChar_SetFlag(c, p_chr_1, ((((150i32) as i64 as u64)) as u32));
            F::BtlChar_SetFlag(c, p_chr_1, ((((27i32) as i64 as u64)) as u32));
            F::BtlChar_SetFxBit(c, p_chr_1, 59i32);
        }
        if (((((p_phase_2) == (2i32)) as i32)) != 0) {
            if (((((F::BtlAnim_GetId(c, p_chr_1)) == (154i32)) as i32)) != 0) {
                if ((F::BtlChar_TestFlag(c, p_chr_1, ((((22i32) as i64 as u64)) as u32))) != 0) {
                    F::BtlAct_Request(c, p_chr_1, 230i32);
                }
                if ((((((F::BtlChar_IsStage4Or27(c)) != 0) && ((F::BtlChar_TestFlag(c, p_chr_1, ((((23i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) {
                    F::BtlAct_Request(c, p_chr_1, 230i32);
                }
                if (((((F::BtlOpp_GetSeenAction(c, p_chr_1)) != (187i32)) as i32)) != 0) {
                    F::BtlAct_Request(c, p_chr_1, 230i32);
                }
                if ((((((((!((F::BtlChar_TestFlag(c, p_chr_1, ((((35i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) && ((F::BtlChar_TestFlag(c, p_chr_1, ((((15i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) {
                    F::BtlAnim_Request(c, p_chr_1, 155i32, F32(0x00000000));
                    { let a0_ = { let a0_ = F::BtlOpp_GetPlayer(c, p_chr_1); F::BtlChar_Get(c, a0_) }; F::BtlChar_SetFlag(c, a0_, ((((140i32) as i64 as u64)) as u32)) };
                }
            }
            if ((((((c.m.r32(((p_chr_1).wrapping_add(976)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) != (0i32)) as i32)) != 0) {
                if ((((((c.m.r32(((p_chr_1).wrapping_add(4016)).wrapping_add(12)) as i32)) != (0i32)) as i32)) != 0) {
                    F::BtlChar_SetFlag(c, p_chr_1, ((((53i32) as i64 as u64)) as u32));
                }
                { let a0_ = F::BtlChar_GetPos(c, p_chr_1); F::BtlCharApi_ShakeCamsNear(c, a0_, F32(0x42c80000), F32(0x447a0000), F32(0x40400000), F32(0x3ecccccc)) };
                { let a0_ = F::BtlChar_GetPos(c, p_chr_1); F::BtlCharApi_RumbleNear(c, a0_, F32(0x42c80000), F32(0x447a0000), F32(0x3f4ccccc), F32(0x3e4ccccc)) };
                F::BtlAct_Request(c, p_chr_1, 222i32);
            }
        }
        if (((((p_phase_2) == (3i32)) as i32)) != 0) {
            F::BtlChar_SetFlag(c, p_chr_1, ((((51i32) as i64 as u64)) as u32));
        }
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_SlamThrown", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_DragDown(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_DragDown", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let mut l_lean_3: F32 = F32(0);
    let mut l_speed_4: F32 = F32(0);
    let mut l_dropped_5: i32 = 0;
    let r_ = 'f: {
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        F::BtlAnim_Play(c, p_chr_1, 407i32, F32(0x00000000));
        { let a2_ = (if (((rt::flt(F32(c.m.r32((p_chr_1).wrapping_add(1192))), F32(0x00000000)) as i32)) != 0) { 24i32 } else { 25i32 }); F::ChrCam_RequestCut(c, p_chr_1, 1i32, a2_) };
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        l_lean_3 = F32(0x00000000);
        l_speed_4 = F32(0x00000000);
        's1: {
            let sv6: i32 = F::BtlAnim_GetId(c, p_chr_1);
            match sv6 {
                407i32 => {
                    F::BtlAnim_AdvanceLoop(c, p_chr_1, 0i32);
                    l_speed_4 = F32(0x00000000);
                    l_lean_3 = F32(0x3f800000);
                    F::BtlMove_MoveVertical(c, p_chr_1, F32(0x419425ec), F32(0x461c4000));
                    break 's1;
                }
                408i32 => {
                    { let _ = F::BtlAnim_AdvanceThen(c, p_chr_1, 40i32, F32(0x3e199999), 0i32); };
                    l_speed_4 = F32(0x00000000);
                    l_lean_3 = rt::fsub(F32(0x3f800000), F::BtlAnim_GetProgress(c, p_chr_1));
                    F::BtlMove_MoveVertical(c, p_chr_1, F32(0x00000000), F32(0x461c4000));
                    break 's1;
                }
                40i32 => {
                    if ((F::BtlAnim_Advance(c, p_chr_1, 0i32)) != 0) {
                        F::BtlAct_Request(c, p_chr_1, 11i32);
                    }
                    l_speed_4 = F32(0x00000000);
                    if (((rt::flt(F::BtlAnim_GetProgress(c, p_chr_1), F32(0x3f199999)) as i32)) != 0) {
                        l_speed_4 = F::BtlMoveParam_GetSpeed(c, p_chr_1, 11i32);
                    }
                    F::BtlMove_MoveVertical(c, p_chr_1, F32(0x00000000), F32(0x461c4000));
                    break 's1;
                }
                _ => {
                }
            }
        }
        F::BtlMove_Step(c, p_chr_1, 6i32, 5i32, 4i32, l_speed_4, F32(0x3f6d0979));
        F::BtlChar_SetFlag(c, p_chr_1, ((((187i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((26i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((63i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((68i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((69i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((70i32) as i64 as u64)) as u32));
        F::BtlMove_SetLeanX(c, p_chr_1, rt::fmul(l_lean_3, F32(0xbfc90fda)));
        if ((F::ChrCam_IsCutActive(c, p_chr_1)) != 0) {
            F::BtlChar_SetFlag(c, p_chr_1, ((((211i32) as i64 as u64)) as u32));
        }
    }
    if (((((p_phase_2) == (2i32)) as i32)) != 0) {
        if (((((((((F::BtlAnim_GetId(c, p_chr_1)) == (407i32)) as i32)) != 0) && ((((((c.m.r32((p_chr_1).wrapping_add(2404)) as i32)) > (0i32)) as i32)) != 0)) as i32)) != 0) {
            l_dropped_5 = (((F::BtlOpp_GetSeenAction(c, p_chr_1)) != (191i32)) as i32);
            if ((l_dropped_5) != 0) {
                F::BtlAnim_Request(c, p_chr_1, 408i32, F32(0x00000000));
                F::ChrCam_EndCut(c, p_chr_1);
            }
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_DragDown", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_DraggedDown(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_DraggedDown", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(96) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 96);
    let mut l_pose_3: u32 = 0;
    let mut l_air_4: i32 = 0;
    let mut l_startAir_5: i32 = 0;
    let r_ = 'f: {
        l_pose_3 = F::BtlChar_GetPos(c, p_chr_1);
        if (((((p_phase_2) == (0i32)) as i32)) != 0) {
            l_startAir_5 = F::BtlAct_IsAirMotion(c, p_chr_1, 0i32);
            if (((((F::BtlAct_GetPrev(c, p_chr_1)) == (213i32)) as i32)) != 0) {
                l_startAir_5 = 1i32;
                if ((((!(((rt::flt({ let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(144); F32(c.m.r32(a_)) }, F32(0x3e32b8c1)) as i32)) != 0)) as i32)) != 0) {
                    l_startAir_5 = 0i32;
                }
            }
            F::BtlAnim_Play(c, p_chr_1, (if ((l_startAir_5) != 0) { 196i32 } else { 195i32 }), F32(0x00000000));
            { let v_ = F32(0x00000000); let a_ = (l_pose_3).wrapping_add(152); c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x00000000); let a_ = (l_pose_3).wrapping_add(156); c.m.w32(a_, (v_).0); };
        }
        if (((((p_phase_2) == (1i32)) as i32)) != 0) {
            l_air_4 = F::BtlAct_IsAirMotion(c, p_chr_1, 0i32);
            F::BtlAnim_AdvanceLoop(c, p_chr_1, 0i32);
            F::BtlMove_Step(c, p_chr_1, 6i32, 6i32, 7i32, F32(0x00000000), F32(0x3f6d0979));
            F::BtlMove_MoveVertical(c, p_chr_1, F32(0x419425ec), F32(0x461c4000));
            if (((((((((F::BtlOpp_GetSeenAction(c, p_chr_1)) == (190i32)) as i32)) != 0) || ((((((c.m.r32((p_chr_1).wrapping_add(2404)) as i32)) == (0i32)) as i32)) != 0)) as i32)) != 0) {
                F::BtlOpp_GetTargetPos(c, p_chr_1, fp.wrapping_add(64));
                F::BtlOpp_GetPoseVec30(c, p_chr_1, fp.wrapping_add(80));
                F::Vec4_Add(c, fp, fp.wrapping_add(64), fp.wrapping_add(80));
                { let r_ = rt::fmul(F::BtlOpp_GetHeight(c, p_chr_1), F32(0x3e999999)); let a_ = (fp).wrapping_add(4); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                { let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); F::BtlCharApi_GetNodePos(c, a0_, 0i32, fp.wrapping_add(16)) };
                { let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); F::BtlCharApi_GetNodePos(c, a0_, 16i32, fp.wrapping_add(32)) };
                F::Vec4_Sub(c, fp.wrapping_add(48), fp.wrapping_add(16), fp.wrapping_add(32));
                F::Vec4_Add(c, fp, fp, fp.wrapping_add(48));
                F::Vec4_Copy(c, l_pose_3, fp);
            }
            F::BtlChar_SetFlag(c, p_chr_1, ((((187i32) as i64 as u64)) as u32));
            F::BtlChar_SetFlag(c, p_chr_1, ((((10i32) as i64 as u64)) as u32));
            F::BtlChar_SetFlag(c, p_chr_1, ((((26i32) as i64 as u64)) as u32));
            F::BtlChar_SetFlag(c, p_chr_1, ((((63i32) as i64 as u64)) as u32));
            F::BtlChar_SetFlag(c, p_chr_1, ((((68i32) as i64 as u64)) as u32));
            F::BtlChar_SetFlag(c, p_chr_1, ((((69i32) as i64 as u64)) as u32));
            F::BtlChar_SetFlag(c, p_chr_1, ((((70i32) as i64 as u64)) as u32));
            F::BtlChar_SetFlag(c, p_chr_1, ((((149i32) as i64 as u64)) as u32));
            F::BtlChar_SetFlag(c, p_chr_1, ((((150i32) as i64 as u64)) as u32));
            F::BtlChar_SetFlag(c, p_chr_1, ((((27i32) as i64 as u64)) as u32));
            F::BtlChar_SetFxBit(c, p_chr_1, 59i32);
            F::BtlMove_SetLeanX(c, p_chr_1, (if ((l_air_4) != 0) { F32(0xbfc90fda) } else { F32(0x3fc90fda) }));
        }
        if (((((p_phase_2) == (2i32)) as i32)) != 0) {
            if ((((((F::BtlChar_TestFlag(c, p_chr_1, ((((31i32) as i64 as u64)) as u32))) != 0) || ((F::BtlChar_TestFlag(c, p_chr_1, ((((15i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) {
                F::BtlAct_Request(c, p_chr_1, 222i32);
                { let _ = F::BtlMember_Damage(c, p_chr_1, 500i32, 0i32); };
                { let a0_ = F::BtlChar_GetPos(c, p_chr_1); F::BtlCharApi_ShakeCamsNear(c, a0_, F32(0x42c80000), F32(0x447a0000), F32(0x40a00000), F32(0x3f000000)) };
                { let a0_ = F::BtlChar_GetPos(c, p_chr_1); F::BtlCharApi_RumbleNear(c, a0_, F32(0x42c80000), F32(0x447a0000), F32(0x3f4ccccc), F32(0x3e4ccccc)) };
            }
            if ((F::BtlChar_TestFlag(c, p_chr_1, ((((22i32) as i64 as u64)) as u32))) != 0) {
                F::BtlAct_Request(c, p_chr_1, 230i32);
            }
            if ((F::BtlChar_TestFlag(c, p_chr_1, ((((182i32) as i64 as u64)) as u32))) != 0) {
                if ((F::BtlChar_IsDead(c, p_chr_1)) != 0) {
                    F::BtlAct_Request(c, p_chr_1, 217i32);
                } else {
                    F::BtlAct_Request(c, p_chr_1, 230i32);
                }
            }
            if ((F::BtlChar_IsStage4Or27(c)) != 0) {
                if ((F::BtlChar_TestFlag(c, p_chr_1, ((((23i32) as i64 as u64)) as u32))) != 0) {
                    F::BtlAct_Request(c, p_chr_1, 230i32);
                }
                if ((F::BtlChar_TestFlag(c, p_chr_1, ((((183i32) as i64 as u64)) as u32))) != 0) {
                    if ((F::BtlChar_IsDead(c, p_chr_1)) != 0) {
                        F::BtlAct_Request(c, p_chr_1, 217i32);
                    } else {
                        F::BtlAct_Request(c, p_chr_1, 230i32);
                    }
                }
            }
        }
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_DraggedDown", &PROBE_N, &[], true); }
    r_
}

pub fn BtlActChange_SetFlags(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlActChange_SetFlags", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    F::BtlChar_SetFlag(c, p_chr_1, ((((293i32) as i64 as u64)) as u32));
    F::BtlChar_SetFlag(c, p_chr_1, ((((298i32) as i64 as u64)) as u32));
    F::BtlChar_SetFlag(c, p_chr_1, ((((66i32) as i64 as u64)) as u32));
    F::BtlChar_SetFlag(c, p_chr_1, ((((67i32) as i64 as u64)) as u32));
    F::BtlChar_SetFlag(c, p_chr_1, ((((68i32) as i64 as u64)) as u32));
    F::BtlChar_SetFlag(c, p_chr_1, ((((69i32) as i64 as u64)) as u32));
    F::BtlChar_SetFlag(c, p_chr_1, ((((70i32) as i64 as u64)) as u32));
    F::BtlChar_SetFlag(c, p_chr_1, ((((187i32) as i64 as u64)) as u32));
    F::BtlChar_SetFlag(c, p_chr_1, ((((63i32) as i64 as u64)) as u32));
    F::BtlChar_SetFlag(c, p_chr_1, ((((296i32) as i64 as u64)) as u32));
    F::BtlChar_SetFlag(c, p_chr_1, ((((27i32) as i64 as u64)) as u32));
    F::BtlChar_SetFlag(c, p_chr_1, ((((26i32) as i64 as u64)) as u32));
    F::BtlChar_SetFlag(c, p_chr_1, ((((310i32) as i64 as u64)) as u32));
    { let v_ = 1i32; let a_ = (p_chr_1).wrapping_add(2416); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlActChange_SetFlags", &PROBE_N, &[], true); }
    r_
}

pub fn BtlActChange_PushBack(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlActChange_PushBack", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    { let a1_ = { let a0_ = rt::fadd({ let a_ = ((F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(16)).wrapping_add(4); F32(c.m.r32(a_)) }, F32(0x40490fda)); F::BtlUtil_WrapAngle(c, a0_) }; F::BtlAct_PushAngle(c, p_chr_1, a1_, F32(0x3ecccccc), F32(0x40400000)) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlActChange_PushBack", &PROBE_N, &[], true); }
    r_
}

pub fn BtlActSwitch_SaveEntryPlacement(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlActSwitch_SaveEntryPlacement", &PROBE_N, &[(p_chr_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(80) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 80);
    let mut l_limit_2: F32 = F32(0);
    let mut l_t_3: F32 = F32(0);
    let mut l_len_4: F32 = F32(0);
    let r_ = 'f: {
        { let a0_ = (c.m.r32(p_chr_1) as i32); { let _ = F::BtlStage_GetStartPlace(c, a0_, fp, fp.wrapping_add(64), 0i32); } };
        { let a0_ = F::BtlOpp_GetPlayer(c, p_chr_1); { let _ = F::BtlStage_GetStartPlace(c, a0_, fp.wrapping_add(16), fp.wrapping_add(64), 0i32); } };
        l_limit_2 = F::BtlStage_GetTop(c);
        { let v_ = rt::fsub(F32(c.m.r32(fp)), F32(c.m.r32(fp.wrapping_add(16)))); let a_ = fp.wrapping_add(64); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32((fp).wrapping_add(8))), F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8)))); let a_ = (fp.wrapping_add(64)).wrapping_add(8); c.m.w32(a_, (v_).0); };
        l_t_3 = rt::fneg(F::Mathf_Tan(c, F32(0x3f32b8c1)));
        { let v_ = rt::fmul(l_t_3, { let a0_ = rt::fadd(rt::fmul(F32(c.m.r32(fp.wrapping_add(64))), F32(c.m.r32(fp.wrapping_add(64)))), rt::fmul(F32(c.m.r32((fp.wrapping_add(64)).wrapping_add(8))), F32(c.m.r32((fp.wrapping_add(64)).wrapping_add(8))))); F::Port_sqrtf(c, a0_) }); let a_ = (fp.wrapping_add(64)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        if (((rt::flt(F::Vec3_Length(c, fp.wrapping_add(64)), F32(0x3a83126e)) as i32)) != 0) {
            { let v_ = F32(0x00000000); let a_ = fp.wrapping_add(64); c.m.w32(a_, (v_).0); };
            { let v_ = rt::fneg(F::Mathf_Sin(c, F32(0x3eb2b8c1))); let a_ = (fp.wrapping_add(64)).wrapping_add(4); c.m.w32(a_, (v_).0); };
            { let v_ = F::Mathf_Cos(c, F32(0x3eb2b8c1)); let a_ = (fp.wrapping_add(64)).wrapping_add(8); c.m.w32(a_, (v_).0); };
        } else {
            F::Vec3_Normalize(c, fp.wrapping_add(64), fp.wrapping_add(64));
        }
        l_t_3 = rt::fdiv(rt::fsub(l_limit_2, F32(c.m.r32((fp).wrapping_add(4)))), F32(c.m.r32((fp.wrapping_add(64)).wrapping_add(4))));
        if (((rt::fgt(l_t_3, F32(0x43960000)) as i32)) != 0) {
            l_t_3 = F32(0x43960000);
        }
        { let v_ = rt::fadd(F32(c.m.r32(fp)), rt::fmul(F32(c.m.r32(fp.wrapping_add(64))), l_t_3)); let a_ = fp.wrapping_add(32); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32((fp).wrapping_add(4))), rt::fmul(F32(c.m.r32((fp.wrapping_add(64)).wrapping_add(4))), l_t_3)); let a_ = (fp.wrapping_add(32)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32((fp).wrapping_add(8))), rt::fmul(F32(c.m.r32((fp.wrapping_add(64)).wrapping_add(8))), l_t_3)); let a_ = (fp.wrapping_add(32)).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = fp.wrapping_add(48); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32(fp.wrapping_add(64))); let a1_ = F32(c.m.r32((fp.wrapping_add(64)).wrapping_add(8))); F::Port_atan2f(c, a0_, a1_) }; let a_ = (fp.wrapping_add(48)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (fp.wrapping_add(48)).wrapping_add(8); c.m.w32(a_, (v_).0); };
        l_t_3 = rt::fsub(F::BtlStage_GetInnerRadius(c), { let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); F::BtlCharApi_GetRadius(c, a0_) });
        l_len_4 = { let a0_ = rt::fadd(rt::fmul(F32(c.m.r32(fp.wrapping_add(32))), F32(c.m.r32(fp.wrapping_add(32)))), rt::fmul(F32(c.m.r32((fp.wrapping_add(32)).wrapping_add(8))), F32(c.m.r32((fp.wrapping_add(32)).wrapping_add(8))))); F::Port_sqrtf(c, a0_) };
        if (((rt::flt(l_t_3, l_len_4) as i32)) != 0) {
            { let r_ = l_len_4; let n_ = rt::fdiv(l_t_3, r_); l_t_3 = n_; };
            { let r_ = l_t_3; let a_ = fp.wrapping_add(32); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            { let r_ = l_t_3; let a_ = (fp.wrapping_add(32)).wrapping_add(8); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        }
        F::BtlChar_SetSavedPlacement(c, p_chr_1, fp.wrapping_add(32), fp.wrapping_add(48), fp.wrapping_add(32), 0i32, 0i32);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlActSwitch_SaveEntryPlacement", &PROBE_N, &[], true); }
    r_
}

pub fn BtlActSwitch_SetLeaveCut(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlActSwitch_SetLeaveCut", &PROBE_N, &[(p_chr_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_hidden_2: i32 = 0;
    let mut l_pose_3: u32 = 0;
    let mut l_onFighter_4: i32 = 0;
    let r_ = 'f: {
        l_hidden_2 = 0i32;
        l_pose_3 = F::BtlChar_GetPos(c, p_chr_1);
        l_onFighter_4 = 1i32;
        if (((rt::flt(F::BtlStage_GetBottom(c), F32(c.m.r32((l_pose_3).wrapping_add(4)))) as i32)) != 0) {
            l_onFighter_4 = 0i32;
            l_hidden_2 = 1i32;
        }
        if (((((F::BtlAct_GetPrev(c, p_chr_1)) == (235i32)) as i32)) != 0) {
            l_onFighter_4 = 0i32;
            l_hidden_2 = 1i32;
        }
        if ((l_onFighter_4) != 0) {
            { let a12_ = rt::fmul({ let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); F::BtlCharApi_GetHeight(c, a0_) }, F32(0x3fc00000)); let a13_ = rt::fmul({ let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); F::BtlCharApi_GetHeight(c, a0_) }, F32(0x3f000000)); F::ChrCam_SetCut(c, p_chr_1, sym::D_002EC2A0, sym::D_002EC2A0, sym::D_002EC2A0, sym::D_002EC2A0, sym::D_002EC2A0, sym::D_002EC2A0, 3i32, F32(0x00000000), F32(0x3fc90fda), F32(0xbf666666), F32(0xbecccccc), a12_, a13_, 3i32, 3i32, 3i32, 60i32, 197i32) };
        } else {
            { let _ = F::BtlStage_GetPlace(c, fp, fp.wrapping_add(16)); };
            { let r_ = F32(0x40a00000); let a_ = (fp).wrapping_add(4); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            F::ChrCam_SetCut(c, p_chr_1, fp, sym::D_002EC2A0, fp, sym::D_002EC2A0, fp, sym::D_002EC2A0, (1i32).wrapping_neg(), F32(0x00000000), F32(0xbf490fda), F32(0xbe4ccccc), F32(0x00000000), F32(0x42c80000), F32(0x00000000), (1i32).wrapping_neg(), (1i32).wrapping_neg(), (1i32).wrapping_neg(), 60i32, 197i32);
        }
        break 'f l_hidden_2;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlActSwitch_SetLeaveCut", &PROBE_N, &[], true); }
    r_
}

pub fn BtlActSwitch_SetEnterCut(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlActSwitch_SetEnterCut", &PROBE_N, &[(p_chr_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(64) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 64);
    let r_ = 'f: {
        { let a0_ = (c.m.r32(p_chr_1) as i32); { let _ = F::BtlStage_GetStartPlace(c, a0_, fp, fp.wrapping_add(16), 0i32); } };
        { let v_ = F32(c.m.r32(fp)); let a_ = fp.wrapping_add(32); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32((fp).wrapping_add(4))), rt::fmul({ let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); F::BtlCharApi_GetHeight(c, a0_) }, F32(0x3e999999))); let a_ = (fp.wrapping_add(32)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((fp).wrapping_add(8))); let a_ = (fp.wrapping_add(32)).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32(fp.wrapping_add(32))); let a_ = fp.wrapping_add(48); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32((fp.wrapping_add(32)).wrapping_add(4))), rt::fmul({ let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); F::BtlCharApi_GetHeight(c, a0_) }, F32(0x3f000000))); let a_ = (fp.wrapping_add(48)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((fp.wrapping_add(32)).wrapping_add(8))); let a_ = (fp.wrapping_add(48)).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let a8_ = { let a0_ = rt::fadd({ let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(148); F32(c.m.r32(a_)) }, F32(0x40490fda)); F::BtlUtil_WrapAngle(c, a0_) }; let a12_ = rt::fadd({ let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); F::BtlCharApi_GetHeight(c, a0_) }, F32(0x40a00000)); F::ChrCam_SetCut(c, p_chr_1, fp.wrapping_add(32), sym::D_002EC2A0, fp.wrapping_add(32), sym::D_002EC2A0, fp.wrapping_add(48), sym::D_002EC2A0, (1i32).wrapping_neg(), a8_, F32(0x00000000), F32(0x00000000), F32(0x00000000), a12_, F32(0x00000000), (1i32).wrapping_neg(), (1i32).wrapping_neg(), 17i32, 15i32, 197i32) };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlActSwitch_SetEnterCut", &PROBE_N, &[], true); }
    r_
}

pub fn BtlActChange_Finish(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlActChange_Finish", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    F::BtlAct_Request(c, p_chr_1, 11i32);
    if ((F::BtlAct_CheckStoryForced(c, p_chr_1)) != 0) {
        { let a1_ = F::BtlAct_GetQueued(c, p_chr_1); F::BtlAct_Request(c, p_chr_1, a1_) };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlActChange_Finish", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_TransformA(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_TransformA", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let mut l_pose_3: u32 = 0;
    let mut l_waiting_4: i32 = 0;
    let r_ = 'f: {
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        l_pose_3 = F::BtlChar_GetPos(c, p_chr_1);
        F::BtlAnim_Play(c, p_chr_1, 375i32, F32(0x00000000));
        F::BtlAnim_PlaySub(c, p_chr_1, 376i32);
        { let v_ = F32(0x00000000); let a_ = (l_pose_3).wrapping_add(152); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (l_pose_3).wrapping_add(156); c.m.w32(a_, (v_).0); };
        { let a1_ = F32(c.m.r32(((l_pose_3).wrapping_add(16)).wrapping_add(4))); F::BtlMove_SetHeading(c, p_chr_1, a1_, F32(0x00000000)) };
        F::BtlChar_SetHeldFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((41i32) as i64 as u64)) as u32));
        F::ChrCam_RequestCut(c, p_chr_1, 0i32, 4i32);
        F::BtlChar_SavePlacement(c, p_chr_1);
        F::BtlChar_ResetLook(c, p_chr_1);
        { let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlEvent_Raise(c, a0_, 32i32) };
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        l_waiting_4 = 0i32;
        if ((((((c.m.r32((p_chr_1).wrapping_add(2404)) as i32)) == (1i32)) as i32)) != 0) {
            F::BtlCharSnd_PlayCommon(c, p_chr_1, 84i32);
            F::BtlChar_PlayVoice(c, p_chr_1, 14i32);
            F::BtlChar_SetFxBit(c, p_chr_1, 0i32);
            { let a0_ = (c.m.r32(p_chr_1) as i32); let a1_ = (c.m.r32(((p_chr_1).wrapping_add(4812)).wrapping_add(4)) as i32); let a2_ = (c.m.r32(((p_chr_1).wrapping_add(4812)).wrapping_add(8)) as i32); let a3_ = (c.m.r32(((p_chr_1).wrapping_add(4812)).wrapping_add(20)) as i32); let a4_ = (c.m.r32(((p_chr_1).wrapping_add(4812)).wrapping_add(24)) as i32); let a5_ = (c.m.r32(((p_chr_1).wrapping_add(4812)).wrapping_add(28)) as i32); let a6_ = (c.m.r32(((p_chr_1).wrapping_add(4812)).wrapping_add(32)) as i32); F::BtlChange_RequestChara(c, a0_, a1_, a2_, a3_, a4_, a5_, a6_) };
        }
        's1: {
            let sv5: i32 = F::BtlAnim_GetId(c, p_chr_1);
            match sv5 {
                375i32 => {
                    if ((F::BtlAnim_Advance(c, p_chr_1, 0i32)) != 0) {
                        F::BtlAnim_RequestSubToMain(c, p_chr_1);
                    }
                    F::BtlActChange_PushBack(c, p_chr_1);
                    l_waiting_4 = 1i32;
                    break 's1;
                }
                376i32 => {
                    F::BtlAnim_AdvanceLoop(c, p_chr_1, 0i32);
                    l_waiting_4 = 1i32;
                    F::BtlActChange_PushBack(c, p_chr_1);
                    if (({ let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlChange_IsLoadedFor(c, a0_) }) != 0) {
                        F::BtlAnim_Request(c, p_chr_1, 377i32, F32(0x00000000));
                        { let a1_ = (c.m.r32(((p_chr_1).wrapping_add(4812)).wrapping_add(12)) as i32); F::BtlMember_SubBlast(c, p_chr_1, a1_) };
                        { let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlChange_SetReady(c, a0_) };
                    }
                    break 's1;
                }
                377i32 => {
                    if ((F::BtlAnim_IsNew(c, p_chr_1)) != 0) {
                        { let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlChange_SetDone(c, a0_) };
                        F::BtlChar_SetFlag(c, p_chr_1, ((((245i32) as i64 as u64)) as u32));
                        F::BtlChar_SetFlag(c, p_chr_1, ((((85i32) as i64 as u64)) as u32));
                        F::BtlChar_SetFxBit(c, p_chr_1, 1i32);
                        F::BtlChar_PlayVoice(c, p_chr_1, 34i32);
                        F::ChrCam_RequestCut(c, p_chr_1, 0i32, 6i32);
                        if ((((F::BtlParam_GetFlags(c, p_chr_1)) & (((262144i32) as u32)))) != 0) {
                            F::BtlCharSnd_PlayCommon(c, p_chr_1, 76i32);
                        } else {
                            F::BtlCharSnd_PlayCommon(c, p_chr_1, 83i32);
                        }
                    }
                    if ((F::BtlAnim_WasNew(c, p_chr_1)) != 0) {
                        F::ChrCam_AddShake(c, p_chr_1, F32(0x40a00000), F32(0x3ecccccc));
                        F::BtlChar_Vibrate(c, p_chr_1, F32(0x3f800000), F32(0x3f000000));
                    }
                    if ((F::BtlAnim_Advance(c, p_chr_1, 0i32)) != 0) {
                        { let v_ = 1i32; let a_ = ((p_chr_1).wrapping_add(976)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                        { let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlEvent_Raise(c, a0_, 90i32) };
                        F::BtlChar_SetFxBit(c, p_chr_1, 2i32);
                    }
                    break 's1;
                }
                _ => {
                }
            }
        }
        F::BtlMove_Step(c, p_chr_1, 6i32, 6i32, 7i32, F32(0x00000000), F32(0x3f6d0979));
        F::BtlMove_BrakeVertical(c, p_chr_1);
        F::BtlActChange_SetFlags(c, p_chr_1);
        F::BtlChar_SetFxBit(c, p_chr_1, 16i32);
        F::BtlChar_SetFxBit(c, p_chr_1, 20i32);
        if ((((((l_waiting_4) != 0) && ((((!((F::ChrCam_IsCutActive(c, p_chr_1)) != 0)) as i32)) != 0)) as i32)) != 0) {
            F::ChrCam_RequestCut(c, p_chr_1, 0i32, 5i32);
        }
    }
    if (((((((((p_phase_2) == (2i32)) as i32)) != 0) && ((((((c.m.r32(((p_chr_1).wrapping_add(976)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) != (0i32)) as i32)) != 0)) as i32)) != 0) {
        F::BtlActChange_Finish(c, p_chr_1);
    }
    if (((((p_phase_2) == (3i32)) as i32)) != 0) {
        F::BtlChar_SetFlag(c, p_chr_1, ((((55i32) as i64 as u64)) as u32));
        { let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlEvent_Raise(c, a0_, 77i32) };
        F::BtlChar_AddStageTimer(c, F32(0x40000000));
        break 'f 0;
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_TransformA", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_TransformB(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_TransformB", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let mut l_pose_3: u32 = 0;
    let r_ = 'f: {
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        l_pose_3 = F::BtlChar_GetPos(c, p_chr_1);
        F::BtlAnim_Play(c, p_chr_1, 399i32, F32(0x00000000));
        F::BtlAnim_PlaySub(c, p_chr_1, 400i32);
        { let v_ = F32(0x00000000); let a_ = (l_pose_3).wrapping_add(152); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (l_pose_3).wrapping_add(156); c.m.w32(a_, (v_).0); };
        { let a1_ = F32(c.m.r32(((l_pose_3).wrapping_add(16)).wrapping_add(4))); F::BtlMove_SetHeading(c, p_chr_1, a1_, F32(0x00000000)) };
        F::BtlChar_SetHeldFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((41i32) as i64 as u64)) as u32));
        F::ChrCam_RequestCut(c, p_chr_1, 1i32, 0i32);
        F::BtlChar_SavePlacement(c, p_chr_1);
        F::BtlCharSnd_PlayCommon(c, p_chr_1, 84i32);
        { let a0_ = (c.m.r32(p_chr_1) as i32); let a1_ = (c.m.r32(((p_chr_1).wrapping_add(4812)).wrapping_add(4)) as i32); let a2_ = (c.m.r32(((p_chr_1).wrapping_add(4812)).wrapping_add(8)) as i32); let a3_ = (c.m.r32(((p_chr_1).wrapping_add(4812)).wrapping_add(20)) as i32); let a4_ = (c.m.r32(((p_chr_1).wrapping_add(4812)).wrapping_add(24)) as i32); let a5_ = (c.m.r32(((p_chr_1).wrapping_add(4812)).wrapping_add(28)) as i32); let a6_ = (c.m.r32(((p_chr_1).wrapping_add(4812)).wrapping_add(32)) as i32); F::BtlChange_RequestChara(c, a0_, a1_, a2_, a3_, a4_, a5_, a6_) };
        F::BtlChar_ResetLook(c, p_chr_1);
        { let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlEvent_Raise(c, a0_, 32i32) };
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        's1: {
            let sv4: i32 = F::BtlAnim_GetId(c, p_chr_1);
            match sv4 {
                399i32 => {
                    if ((F::BtlAnim_Advance(c, p_chr_1, 0i32)) != 0) {
                        F::BtlAnim_RequestSubToMain(c, p_chr_1);
                    }
                    break 's1;
                }
                400i32 => {
                    F::BtlAnim_AdvanceLoop(c, p_chr_1, 0i32);
                    if (({ let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlChange_IsLoadedFor(c, a0_) }) != 0) {
                        F::BtlAnim_Request(c, p_chr_1, 401i32, F32(0x00000000));
                        { let a1_ = (c.m.r32(((p_chr_1).wrapping_add(4812)).wrapping_add(12)) as i32); F::BtlMember_SubBlast(c, p_chr_1, a1_) };
                        { let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlChange_SetReady(c, a0_) };
                    }
                    break 's1;
                }
                401i32 => {
                    if ((F::BtlAnim_IsNew(c, p_chr_1)) != 0) {
                        { let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlChange_SetDone(c, a0_) };
                        F::BtlCharSnd_PlayStream(c, p_chr_1, 36148i32);
                        F::BtlChar_PlayVoice(c, p_chr_1, 34i32);
                        F::BtlChar_SetFlag(c, p_chr_1, ((((245i32) as i64 as u64)) as u32));
                        F::BtlChar_SetFlag(c, p_chr_1, ((((85i32) as i64 as u64)) as u32));
                    }
                    if (((rt::flt(F::BtlAnim_GetFrame(c, p_chr_1), F32(0x40400000)) as i32)) != 0) {
                        F::BtlChar_SetFxBit(c, p_chr_1, 3i32);
                    }
                    if ((F::BtlAnim_Advance(c, p_chr_1, 0i32)) != 0) {
                        { let v_ = 1i32; let a_ = ((p_chr_1).wrapping_add(976)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                        { let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlEvent_Raise(c, a0_, 90i32) };
                    }
                    break 's1;
                }
                _ => {
                }
            }
        }
        F::BtlMove_Step(c, p_chr_1, 6i32, 6i32, 7i32, F32(0x00000000), F32(0x3f6d0979));
        F::BtlMove_BrakeVertical(c, p_chr_1);
        F::BtlActChange_SetFlags(c, p_chr_1);
        F::BtlChar_SetFxBit(c, p_chr_1, 20i32);
    }
    if (((((((((p_phase_2) == (2i32)) as i32)) != 0) && ((((((c.m.r32(((p_chr_1).wrapping_add(976)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) != (0i32)) as i32)) != 0)) as i32)) != 0) {
        F::BtlActChange_Finish(c, p_chr_1);
    }
    if (((((p_phase_2) == (3i32)) as i32)) != 0) {
        F::BtlChar_SetFlag(c, p_chr_1, ((((55i32) as i64 as u64)) as u32));
        { let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlEvent_Raise(c, a0_, 77i32) };
        break 'f 0;
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_TransformB", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_TransformC(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_TransformC", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let r_ = 'f: {
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        F::BtlAnim_Play(c, p_chr_1, 375i32, F32(0x00000000));
        { let v_ = F32(0x00000000); let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(152); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(156); c.m.w32(a_, (v_).0); };
        { let a1_ = { let a_ = ((F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(16)).wrapping_add(4); F32(c.m.r32(a_)) }; F::BtlMove_SetHeading(c, p_chr_1, a1_, F32(0x00000000)) };
        F::BtlChar_SavePlacement(c, p_chr_1);
        F::BtlChar_SetFlag(c, p_chr_1, ((((244i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((215i32) as i64 as u64)) as u32));
        F::BtlChar_ResetLook(c, p_chr_1);
        { let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlEvent_Raise(c, a0_, 32i32) };
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        if ((((((c.m.r32((p_chr_1).wrapping_add(2404)) as i32)) == (1i32)) as i32)) != 0) {
            F::BtlCharSnd_PlayCommon(c, p_chr_1, 84i32);
            F::BtlChar_SetFxBit(c, p_chr_1, 0i32);
            { let a0_ = (c.m.r32(p_chr_1) as i32); let a1_ = (c.m.r32(((p_chr_1).wrapping_add(4812)).wrapping_add(4)) as i32); let a2_ = (c.m.r32(((p_chr_1).wrapping_add(4812)).wrapping_add(8)) as i32); let a3_ = (c.m.r32(((p_chr_1).wrapping_add(4812)).wrapping_add(20)) as i32); let a4_ = (c.m.r32(((p_chr_1).wrapping_add(4812)).wrapping_add(24)) as i32); let a5_ = (c.m.r32(((p_chr_1).wrapping_add(4812)).wrapping_add(28)) as i32); let a6_ = (c.m.r32(((p_chr_1).wrapping_add(4812)).wrapping_add(32)) as i32); F::BtlChange_RequestChara(c, a0_, a1_, a2_, a3_, a4_, a5_, a6_) };
        }
        's1: {
            let sv3: i32 = F::BtlAnim_GetId(c, p_chr_1);
            match sv3 {
                375i32 => {
                    if ((F::BtlAnim_Advance(c, p_chr_1, 0i32)) != 0) {
                        if (({ let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlChange_IsLoadedFor(c, a0_) }) != 0) {
                            F::BtlAnim_Request(c, p_chr_1, 377i32, F32(0x00000000));
                            { let a1_ = (c.m.r32(((p_chr_1).wrapping_add(4812)).wrapping_add(12)) as i32); F::BtlMember_SubBlast(c, p_chr_1, a1_) };
                            { let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlChange_SetReady(c, a0_) };
                        }
                    }
                    F::BtlChar_SetFxBit(c, p_chr_1, 17i32);
                    F::BtlChar_SetFlag(c, p_chr_1, ((((303i32) as i64 as u64)) as u32));
                    break 's1;
                }
                377i32 => {
                    if ((F::BtlAnim_IsNew(c, p_chr_1)) != 0) {
                        { let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlChange_SetDone(c, a0_) };
                        F::BtlChar_SetFxBit(c, p_chr_1, 1i32);
                        F::BtlChar_PlayVoice(c, p_chr_1, 34i32);
                        F::BtlChar_SetFlag(c, p_chr_1, ((((217i32) as i64 as u64)) as u32));
                        F::BtlChar_Vibrate(c, p_chr_1, F32(0x3f800000), F32(0x3f000000));
                        F::BtlCharSnd_PlayStream(c, p_chr_1, 36150i32);
                    }
                    if ((F::BtlAnim_Advance(c, p_chr_1, 0i32)) != 0) {
                        { let v_ = 1i32; let a_ = ((p_chr_1).wrapping_add(976)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                        { let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlEvent_Raise(c, a0_, 90i32) };
                        F::BtlChar_SetFxBit(c, p_chr_1, 2i32);
                    }
                    break 's1;
                }
                _ => {
                }
            }
        }
        F::BtlMove_Step(c, p_chr_1, 6i32, 5i32, 7i32, F32(0x00000000), F32(0x3f6d0979));
        F::BtlMove_ApplyGravity(c, p_chr_1);
        F::BtlActChange_SetFlags(c, p_chr_1);
        F::BtlChar_SetFlag(c, p_chr_1, ((((301i32) as i64 as u64)) as u32));
        F::BtlChar_SetFxBit(c, p_chr_1, 20i32);
        if ((((((c.m.r32(((p_chr_1).wrapping_add(4812)).wrapping_add(16)) as i32)) == (4i32)) as i32)) != 0) {
            F::BtlChar_SetFlag(c, p_chr_1, ((((306i32) as i64 as u64)) as u32));
        }
        if ((((((c.m.r32(((p_chr_1).wrapping_add(4812)).wrapping_add(16)) as i32)) == (3i32)) as i32)) != 0) {
            F::BtlChar_SetFlag(c, p_chr_1, ((((307i32) as i64 as u64)) as u32));
        }
    }
    if (((((((((p_phase_2) == (2i32)) as i32)) != 0) && ((((((c.m.r32(((p_chr_1).wrapping_add(976)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) != (0i32)) as i32)) != 0)) as i32)) != 0) {
        F::BtlActChange_Finish(c, p_chr_1);
    }
    if (((((p_phase_2) == (3i32)) as i32)) != 0) {
        F::BtlChar_SetFlag(c, p_chr_1, ((((55i32) as i64 as u64)) as u32));
        if ((((((((!((F::BtlChar_TestFlag(c, p_chr_1, ((((252i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) && ((((!((F::BtlChar_TestFlag(c, p_chr_1, ((((253i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0)) as i32)) != 0) {
            F::BtlChar_SetFlag(c, p_chr_1, ((((245i32) as i64 as u64)) as u32));
            F::BtlChar_SetFlag(c, p_chr_1, ((((85i32) as i64 as u64)) as u32));
        }
        F::BtlChar_SetFlag(c, p_chr_1, ((((218i32) as i64 as u64)) as u32));
        { let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlEvent_Raise(c, a0_, 77i32) };
        F::BtlChar_AddStageTimer(c, F32(0x40000000));
        break 'f 0;
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_TransformC", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_TransformD(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_TransformD", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let mut l_timer_3: u32 = 0;
    let mut l_pose_4: u32 = 0;
    let r_ = 'f: {
    l_timer_3 = ((p_chr_1).wrapping_add(976)).wrapping_add((((2i32) as u32)).wrapping_mul(4));
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        l_pose_4 = F::BtlChar_GetPos(c, p_chr_1);
        F::BtlAnim_Play(c, p_chr_1, 43i32, F32(0x00000000));
        F::BtlAnim_PlaySub(c, p_chr_1, 382i32);
        { let v_ = F32(0x00000000); let a_ = (l_pose_4).wrapping_add(152); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (l_pose_4).wrapping_add(156); c.m.w32(a_, (v_).0); };
        { let a1_ = F32(c.m.r32(((l_pose_4).wrapping_add(16)).wrapping_add(4))); F::BtlMove_SetHeading(c, p_chr_1, a1_, F32(0x00000000)) };
        F::ChrCam_RequestCut(c, p_chr_1, 0i32, 4i32);
        F::BtlChar_SavePlacement(c, p_chr_1);
        F::BtlChar_ResetLook(c, p_chr_1);
        { let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlEvent_Raise(c, a0_, 32i32) };
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        if ((((((c.m.r32((p_chr_1).wrapping_add(2404)) as i32)) == (1i32)) as i32)) != 0) {
            F::BtlCharSnd_PlayCommon(c, p_chr_1, 24i32);
            { let a0_ = (c.m.r32(p_chr_1) as i32); let a1_ = (c.m.r32(((p_chr_1).wrapping_add(4812)).wrapping_add(36)) as i32); let a2_ = (c.m.r32(((p_chr_1).wrapping_add(4812)).wrapping_add(40)) as i32); F::BtlChange_RequestObject(c, a0_, a1_, a2_, 0i32, 2i32) };
            { let a0_ = (c.m.r32(p_chr_1) as i32); let a1_ = (c.m.r32(((p_chr_1).wrapping_add(4812)).wrapping_add(4)) as i32); let a2_ = (c.m.r32(((p_chr_1).wrapping_add(4812)).wrapping_add(8)) as i32); let a3_ = (c.m.r32(((p_chr_1).wrapping_add(4812)).wrapping_add(20)) as i32); let a4_ = (c.m.r32(((p_chr_1).wrapping_add(4812)).wrapping_add(24)) as i32); let a5_ = (c.m.r32(((p_chr_1).wrapping_add(4812)).wrapping_add(28)) as i32); let a6_ = (c.m.r32(((p_chr_1).wrapping_add(4812)).wrapping_add(32)) as i32); F::BtlChange_RequestChara(c, a0_, a1_, a2_, a3_, a4_, a5_, a6_) };
        }
        's1: {
            let sv5: i32 = F::BtlAnim_GetId(c, p_chr_1);
            match sv5 {
                43i32 => {
                    if ((F::BtlAnim_Advance(c, p_chr_1, 0i32)) != 0) {
                        F::BtlAnim_RequestSubToMain(c, p_chr_1);
                    }
                    F::BtlMove_MoveVertical(c, p_chr_1, F32(0xc0ed0979), F32(0x3f6d0979));
                    F::BtlChar_SetFlag(c, p_chr_1, ((((24i32) as i64 as u64)) as u32));
                    break 's1;
                }
                382i32 => {
                    if ((F::BtlAnim_IsNew(c, p_chr_1)) != 0) {
                        F::ChrCam_RequestCut(c, p_chr_1, 0i32, 5i32);
                        F::BtlChar_PlayVoice(c, p_chr_1, 38i32);
                        F::BtlChar_SetFlag(c, p_chr_1, ((((247i32) as i64 as u64)) as u32));
                    }
                    F::BtlAnim_AdvanceLoop(c, p_chr_1, 0i32);
                    F::BtlMove_MoveVertical(c, p_chr_1, F32(0x00000000), F32(0x461c4000));
                    if ((((((((({ let a_ = l_timer_3; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); n_ }) > (60i32)) as i32)) != 0) && (({ let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlChange_IsLoadedFor(c, a0_) }) != 0)) as i32)) != 0) {
                        F::BtlAnim_Request(c, p_chr_1, 375i32, F32(0x00000000));
                        { let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlChange_SetReady(c, a0_) };
                    }
                    F::BtlChar_SetFxBit(c, p_chr_1, 20i32);
                    break 's1;
                }
                375i32 => {
                    if ((F::BtlAnim_IsNew(c, p_chr_1)) != 0) {
                        { let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlChange_SetDone(c, a0_) };
                        F::BtlPartner_PlayAnim(c, p_chr_1, 418i32);
                        F::BtlChar_SetFlag(c, p_chr_1, ((((244i32) as i64 as u64)) as u32));
                        F::BtlChar_SetFlag(c, p_chr_1, ((((215i32) as i64 as u64)) as u32));
                        F::BtlChar_SetFxBit(c, p_chr_1, 0i32);
                    }
                    if ((((((F::BtlAnim_Advance(c, p_chr_1, 0i32)) != 0) && (({ let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlChange_IsLoadedFor(c, a0_) }) != 0)) as i32)) != 0) {
                        F::BtlAnim_Request(c, p_chr_1, 377i32, F32(0x00000000));
                        { let a1_ = (c.m.r32(((p_chr_1).wrapping_add(4812)).wrapping_add(12)) as i32); F::BtlMember_SubBlast(c, p_chr_1, a1_) };
                        { let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlChange_SetReady(c, a0_) };
                    }
                    { let _ = F::BtlPartner_StepAnim(c, p_chr_1); };
                    F::BtlMove_BrakeVertical(c, p_chr_1);
                    F::BtlChar_SetFxBit(c, p_chr_1, 20i32);
                    F::BtlChar_SetFlag(c, p_chr_1, ((((303i32) as i64 as u64)) as u32));
                    break 's1;
                }
                377i32 => {
                    if ((F::BtlAnim_IsNew(c, p_chr_1)) != 0) {
                        F::BtlChar_SetFlag(c, p_chr_1, ((((244i32) as i64 as u64)) as u32));
                        F::BtlPartner_Release(c, p_chr_1);
                        F::BtlChar_SetFlag(c, p_chr_1, ((((217i32) as i64 as u64)) as u32));
                        F::BtlCharSnd_PlayStream(c, p_chr_1, 36150i32);
                        F::BtlChar_SetFxBit(c, p_chr_1, 1i32);
                    }
                    if ((F::BtlAnim_PassedRatio(c, p_chr_1, F32(0x3e999999))) != 0) {
                        F::BtlChar_PlayVoice(c, p_chr_1, 34i32);
                    }
                    if ((F::BtlAnim_Advance(c, p_chr_1, 0i32)) != 0) {
                        { let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlChange_SetDone(c, a0_) };
                        F::BtlChar_SetFxBit(c, p_chr_1, 2i32);
                        { let v_ = 1i32; let a_ = ((p_chr_1).wrapping_add(976)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                        { let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlEvent_Raise(c, a0_, 90i32) };
                    }
                    F::BtlMove_BrakeVertical(c, p_chr_1);
                    break 's1;
                }
                _ => {
                }
            }
        }
        F::BtlMove_Step(c, p_chr_1, 6i32, 5i32, 7i32, F32(0x00000000), F32(0x3f6d0979));
        F::BtlActChange_SetFlags(c, p_chr_1);
        F::BtlChar_SetFlag(c, p_chr_1, ((((301i32) as i64 as u64)) as u32));
    }
    if (((((((((p_phase_2) == (2i32)) as i32)) != 0) && ((((((c.m.r32(((p_chr_1).wrapping_add(976)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) != (0i32)) as i32)) != 0)) as i32)) != 0) {
        F::BtlActChange_Finish(c, p_chr_1);
    }
    if (((((p_phase_2) == (3i32)) as i32)) != 0) {
        F::BtlChar_SetFlag(c, p_chr_1, ((((55i32) as i64 as u64)) as u32));
        if ((((((((!((F::BtlChar_TestFlag(c, p_chr_1, ((((252i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) && ((((!((F::BtlChar_TestFlag(c, p_chr_1, ((((253i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0)) as i32)) != 0) {
            F::BtlChar_SetFlag(c, p_chr_1, ((((245i32) as i64 as u64)) as u32));
            F::BtlChar_SetFlag(c, p_chr_1, ((((85i32) as i64 as u64)) as u32));
        }
        F::BtlChar_SetFlag(c, p_chr_1, ((((218i32) as i64 as u64)) as u32));
        F::ChrCam_EndCut(c, p_chr_1);
        F::BtlChar_SetFlag(c, p_chr_1, ((((205i32) as i64 as u64)) as u32));
        { let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlEvent_Raise(c, a0_, 77i32) };
        F::BtlChar_AddStageTimer(c, F32(0x40000000));
        break 'f 0;
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_TransformD", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_TransformE(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_TransformE", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let mut l_pose_3: u32 = 0;
    let mut l_waiting_4: i32 = 0;
    let r_ = 'f: {
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        l_pose_3 = F::BtlChar_GetPos(c, p_chr_1);
        F::BtlAnim_Play(c, p_chr_1, 375i32, F32(0x00000000));
        F::BtlAnim_PlaySub(c, p_chr_1, 376i32);
        { let v_ = F32(0x00000000); let a_ = (l_pose_3).wrapping_add(152); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (l_pose_3).wrapping_add(156); c.m.w32(a_, (v_).0); };
        { let a1_ = F32(c.m.r32(((l_pose_3).wrapping_add(16)).wrapping_add(4))); F::BtlMove_SetHeading(c, p_chr_1, a1_, F32(0x00000000)) };
        F::BtlChar_SetHeldFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((41i32) as i64 as u64)) as u32));
        F::ChrCam_RequestCut(c, p_chr_1, 0i32, 4i32);
        F::BtlChar_SavePlacement(c, p_chr_1);
        F::BtlChar_ResetLook(c, p_chr_1);
        { let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlEvent_Raise(c, a0_, 32i32) };
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        l_waiting_4 = 0i32;
        if ((((((c.m.r32((p_chr_1).wrapping_add(2404)) as i32)) == (1i32)) as i32)) != 0) {
            F::BtlChar_PlayVoice(c, p_chr_1, 14i32);
            F::BtlChar_SetFxBit(c, p_chr_1, 0i32);
            { let a0_ = (c.m.r32(p_chr_1) as i32); let a1_ = (c.m.r32(((p_chr_1).wrapping_add(4812)).wrapping_add(4)) as i32); let a2_ = (c.m.r32(((p_chr_1).wrapping_add(4812)).wrapping_add(8)) as i32); let a3_ = (c.m.r32(((p_chr_1).wrapping_add(4812)).wrapping_add(20)) as i32); let a4_ = (c.m.r32(((p_chr_1).wrapping_add(4812)).wrapping_add(24)) as i32); let a5_ = (c.m.r32(((p_chr_1).wrapping_add(4812)).wrapping_add(28)) as i32); let a6_ = (c.m.r32(((p_chr_1).wrapping_add(4812)).wrapping_add(32)) as i32); F::BtlChange_RequestChara(c, a0_, a1_, a2_, a3_, a4_, a5_, a6_) };
        }
        's1: {
            let sv5: i32 = F::BtlAnim_GetId(c, p_chr_1);
            match sv5 {
                375i32 => {
                    if ((F::BtlAnim_Advance(c, p_chr_1, 0i32)) != 0) {
                        F::BtlAnim_RequestSubToMain(c, p_chr_1);
                    }
                    l_waiting_4 = 1i32;
                    if ((F::BtlAnim_PassedRatio(c, p_chr_1, F32(0x3ecccccc))) != 0) {
                        F::BtlCharSnd_PlayCommon(c, p_chr_1, 68i32);
                    }
                    if ((F::BtlAnim_PassedRatio(c, p_chr_1, F32(0x3f000000))) != 0) {
                        F::BtlCharSnd_PlayCommon(c, p_chr_1, 34i32);
                    }
                    if ((F::BtlAnim_PassedRatio(c, p_chr_1, F32(0x3f4ccccc))) != 0) {
                        F::BtlCharSnd_PlayCommon(c, p_chr_1, 24i32);
                    }
                    break 's1;
                }
                376i32 => {
                    F::BtlAnim_AdvanceLoop(c, p_chr_1, 0i32);
                    l_waiting_4 = 1i32;
                    if (({ let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlChange_IsLoadedFor(c, a0_) }) != 0) {
                        F::BtlAnim_Request(c, p_chr_1, 377i32, F32(0x00000000));
                        { let a1_ = (c.m.r32(((p_chr_1).wrapping_add(4812)).wrapping_add(12)) as i32); F::BtlMember_SubBlast(c, p_chr_1, a1_) };
                        { let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlChange_SetReady(c, a0_) };
                    }
                    break 's1;
                }
                377i32 => {
                    if ((F::BtlAnim_IsNew(c, p_chr_1)) != 0) {
                        { let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlChange_SetDone(c, a0_) };
                        F::BtlChar_SetFlag(c, p_chr_1, ((((245i32) as i64 as u64)) as u32));
                        F::BtlChar_SetFlag(c, p_chr_1, ((((85i32) as i64 as u64)) as u32));
                        F::BtlChar_SetFxBit(c, p_chr_1, 1i32);
                        F::BtlChar_PlayVoice(c, p_chr_1, 34i32);
                        F::ChrCam_RequestCut(c, p_chr_1, 0i32, 6i32);
                    }
                    if ((F::BtlAnim_Advance(c, p_chr_1, 0i32)) != 0) {
                        { let v_ = 1i32; let a_ = ((p_chr_1).wrapping_add(976)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                        { let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlEvent_Raise(c, a0_, 90i32) };
                        F::BtlChar_SetFxBit(c, p_chr_1, 2i32);
                    }
                    if ((F::BtlAnim_PassedRatio(c, p_chr_1, F32(0x3df5c28f))) != 0) {
                        if (((rt::flt(F::BtlAct_GetHeight(c, p_chr_1), F32(0x40a00000)) as i32)) != 0) {
                            F::BtlCharSnd_PlayCommon(c, p_chr_1, 37i32);
                            F::ChrCam_AddShake(c, p_chr_1, F32(0x40a00000), F32(0x3e4ccccc));
                            F::BtlChar_Vibrate(c, p_chr_1, F32(0x3f800000), F32(0x3e199999));
                        }
                    }
                    if ((F::BtlAnim_PassedRatio(c, p_chr_1, F32(0x3eb33333))) != 0) {
                        F::BtlCharSnd_PlayCommon(c, p_chr_1, 20i32);
                        F::ChrCam_AddShake(c, p_chr_1, F32(0x40a00000), F32(0x3e4ccccc));
                        F::BtlChar_Vibrate(c, p_chr_1, F32(0x3f800000), F32(0x3e199999));
                    }
                    if ((F::BtlAnim_PassedRatio(c, p_chr_1, F32(0x3f2147ae))) != 0) {
                        F::BtlCharSnd_PlayCommon(c, p_chr_1, 69i32);
                        F::ChrCam_AddShake(c, p_chr_1, F32(0x40a00000), F32(0x3dcccccc));
                        F::BtlChar_Vibrate(c, p_chr_1, F32(0x3f800000), F32(0x3dcccccc));
                    }
                    if ((F::BtlAnim_PassedRatio(c, p_chr_1, F32(0x3f400000))) != 0) {
                        F::BtlCharSnd_PlayCommon(c, p_chr_1, 44i32);
                        F::BtlChar_SetFxBit(c, p_chr_1, 24i32);
                    }
                    break 's1;
                }
                _ => {
                }
            }
        }
        F::BtlMove_Step(c, p_chr_1, 6i32, 6i32, 7i32, F32(0x00000000), F32(0x3f6d0979));
        F::BtlMove_BrakeVertical(c, p_chr_1);
        F::BtlActChange_SetFlags(c, p_chr_1);
        F::BtlChar_SetFxBit(c, p_chr_1, 20i32);
        F::BtlChar_SetFxBit(c, p_chr_1, 58i32);
        if ((((((l_waiting_4) != 0) && ((((!((F::ChrCam_IsCutActive(c, p_chr_1)) != 0)) as i32)) != 0)) as i32)) != 0) {
            F::ChrCam_RequestCut(c, p_chr_1, 0i32, 5i32);
        }
    }
    if (((((((((p_phase_2) == (2i32)) as i32)) != 0) && ((((((c.m.r32(((p_chr_1).wrapping_add(976)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) != (0i32)) as i32)) != 0)) as i32)) != 0) {
        F::BtlActChange_Finish(c, p_chr_1);
    }
    if (((((p_phase_2) == (3i32)) as i32)) != 0) {
        F::BtlChar_SetFlag(c, p_chr_1, ((((55i32) as i64 as u64)) as u32));
        { let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlEvent_Raise(c, a0_, 77i32) };
        F::BtlChar_AddStageTimer(c, F32(0x40000000));
        break 'f 0;
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_TransformE", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_Fusion(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_Fusion", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let r_ = 'f: {
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        F::BtlAnim_Play(c, p_chr_1, 31i32, F32(0x00000000));
        { let v_ = F32(0x00000000); let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(152); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(156); c.m.w32(a_, (v_).0); };
        { let a1_ = { let a_ = ((F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(16)).wrapping_add(4); F32(c.m.r32(a_)) }; F::BtlMove_SetHeading(c, p_chr_1, a1_, F32(0x00000000)) };
        F::BtlChar_SavePlacement(c, p_chr_1);
        F::ChrCam_RequestCut(c, p_chr_1, 1i32, 1i32);
        F::BtlChar_ResetLook(c, p_chr_1);
        { let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlEvent_Raise(c, a0_, 32i32) };
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        if ((((((c.m.r32((p_chr_1).wrapping_add(2404)) as i32)) == (1i32)) as i32)) != 0) {
            F::BtlCharSnd_PlayCommon(c, p_chr_1, 49i32);
            { let a0_ = (c.m.r32(p_chr_1) as i32); let a1_ = (c.m.r32(((p_chr_1).wrapping_add(4812)).wrapping_add(36)) as i32); let a2_ = (c.m.r32(((p_chr_1).wrapping_add(4812)).wrapping_add(40)) as i32); let a3_ = (c.m.r32(((p_chr_1).wrapping_add(4812)).wrapping_add(48)) as i32); F::BtlChange_RequestObject(c, a0_, a1_, a2_, a3_, 2i32) };
            { let a0_ = (c.m.r32(p_chr_1) as i32); let a1_ = (c.m.r32(((p_chr_1).wrapping_add(4812)).wrapping_add(4)) as i32); let a2_ = (c.m.r32(((p_chr_1).wrapping_add(4812)).wrapping_add(8)) as i32); let a3_ = (c.m.r32(((p_chr_1).wrapping_add(4812)).wrapping_add(20)) as i32); let a4_ = (c.m.r32(((p_chr_1).wrapping_add(4812)).wrapping_add(24)) as i32); let a5_ = (c.m.r32(((p_chr_1).wrapping_add(4812)).wrapping_add(28)) as i32); let a6_ = (c.m.r32(((p_chr_1).wrapping_add(4812)).wrapping_add(32)) as i32); F::BtlChange_RequestChara(c, a0_, a1_, a2_, a3_, a4_, a5_, a6_) };
        }
        's1: {
            let sv3: i32 = F::BtlAnim_GetId(c, p_chr_1);
            match sv3 {
                31i32 => {
                    { let _ = F::BtlAnim_AdvanceThen(c, p_chr_1, 32i32, F32(0x00000000), 0i32); };
                    if (((rt::flt(F::BtlAnim_GetProgress(c, p_chr_1), F32(0x3f000000)) as i32)) != 0) {
                        F::BtlMove_MoveVertical(c, p_chr_1, F32(0x00000000), F32(0x461c4000));
                    } else {
                        F::BtlMove_MoveVertical(c, p_chr_1, F32(0xc11425ec), F32(0x461c4000));
                    }
                    break 's1;
                }
                32i32 => {
                    if ((F::BtlAnim_Advance(c, p_chr_1, 0i32)) != 0) {
                        F::BtlMove_MoveVertical(c, p_chr_1, F32(0x00000000), F32(0x461c4000));
                        if (({ let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlChange_IsLoadedFor(c, a0_) }) != 0) {
                            if (((((F::BtlAct_GetCurrent(c, p_chr_1)) == (241i32)) as i32)) != 0) {
                                F::BtlAnim_Request(c, p_chr_1, 418i32, F32(0x00000000));
                            } else {
                                F::BtlAnim_Request(c, p_chr_1, 419i32, F32(0x00000000));
                            }
                            F::BtlChar_SetFlag(c, p_chr_1, ((((11i32) as i64 as u64)) as u32));
                            { let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlChange_SetReady(c, a0_) };
                        }
                    } else {
                        F::BtlMove_MoveVertical(c, p_chr_1, F32(0xc11425ec), F32(0x461c4000));
                    }
                    F::BtlChar_SetFlag(c, p_chr_1, ((((24i32) as i64 as u64)) as u32));
                    break 's1;
                }
                418i32 | 419i32 => {
                    if ((F::BtlAnim_IsNew(c, p_chr_1)) != 0) {
                        { let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlChange_SetDone(c, a0_) };
                        { let a1_ = F::BtlAnim_GetId(c, p_chr_1); F::BtlPartner_PlayAnim(c, p_chr_1, a1_) };
                        if (({ let a1_ = (c.m.r32(((p_chr_1).wrapping_add(4812)).wrapping_add(44)) as i32); F::BtlMember_HasAbilityOf(c, p_chr_1, a1_, 121i32) }) != 0) {
                            F::BtlPartner_SetFlag80(c, p_chr_1, 1i32);
                        }
                        F::BtlChar_SetFxBit(c, p_chr_1, 0i32);
                        F::BtlChar_SetFlag(c, p_chr_1, ((((244i32) as i64 as u64)) as u32));
                        if (((((F::BtlAct_GetCurrent(c, p_chr_1)) == (241i32)) as i32)) != 0) {
                            F::BtlChar_SetFlag(c, p_chr_1, ((((215i32) as i64 as u64)) as u32));
                        } else {
                            F::BtlChar_SetFlag(c, p_chr_1, ((((216i32) as i64 as u64)) as u32));
                        }
                    }
                    if (((((F::BtlAct_GetCurrent(c, p_chr_1)) == (241i32)) as i32)) != 0) {
                        F::BtlChar_SetFlag(c, p_chr_1, ((((304i32) as i64 as u64)) as u32));
                    } else {
                        F::BtlChar_SetFlag(c, p_chr_1, ((((305i32) as i64 as u64)) as u32));
                    }
                    if ((((((F::BtlAnim_Advance(c, p_chr_1, 0i32)) != 0) && (({ let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlChange_IsLoadedFor(c, a0_) }) != 0)) as i32)) != 0) {
                        F::BtlAnim_Request(c, p_chr_1, 420i32, F32(0x00000000));
                        { let a1_ = (c.m.r32(((p_chr_1).wrapping_add(4812)).wrapping_add(12)) as i32); F::BtlMember_SubBlast(c, p_chr_1, a1_) };
                        { let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlChange_SetReady(c, a0_) };
                    }
                    { let _ = F::BtlPartner_StepAnim(c, p_chr_1); };
                    F::BtlMove_BrakeVertical(c, p_chr_1);
                    F::BtlChar_SetFxBit(c, p_chr_1, 20i32);
                    break 's1;
                }
                420i32 => {
                    if ((F::BtlAnim_IsNew(c, p_chr_1)) != 0) {
                        F::BtlChar_SetFlag(c, p_chr_1, ((((244i32) as i64 as u64)) as u32));
                        F::BtlPartner_Release(c, p_chr_1);
                        F::BtlChar_SetFlag(c, p_chr_1, ((((217i32) as i64 as u64)) as u32));
                        F::BtlChar_SetFxBit(c, p_chr_1, 1i32);
                        F::BtlChar_Vibrate(c, p_chr_1, F32(0x3f800000), F32(0x3f000000));
                        if (((((F::BtlAct_GetCurrent(c, p_chr_1)) == (241i32)) as i32)) != 0) {
                            F::BtlCharSnd_PlayStream(c, p_chr_1, 36150i32);
                        } else {
                            F::BtlCharSnd_PlayStream(c, p_chr_1, 36152i32);
                        }
                    }
                    if ((F::BtlAnim_PassedRatio(c, p_chr_1, F32(0x3e999999))) != 0) {
                        F::BtlChar_PlayVoice(c, p_chr_1, 34i32);
                    }
                    if ((F::BtlAnim_Advance(c, p_chr_1, 0i32)) != 0) {
                        { let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlChange_SetDone(c, a0_) };
                        F::BtlChar_SetFxBit(c, p_chr_1, 2i32);
                        { let v_ = 1i32; let a_ = ((p_chr_1).wrapping_add(976)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                        { let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlEvent_Raise(c, a0_, 90i32) };
                    }
                    F::BtlMove_BrakeVertical(c, p_chr_1);
                    break 's1;
                }
                _ => {
                }
            }
        }
        F::BtlMove_Step(c, p_chr_1, 6i32, 5i32, 7i32, F32(0x00000000), F32(0x3f6d0979));
        F::BtlActChange_SetFlags(c, p_chr_1);
        F::BtlChar_SetFlag(c, p_chr_1, ((((301i32) as i64 as u64)) as u32));
    }
    if (((((((((p_phase_2) == (2i32)) as i32)) != 0) && ((((((c.m.r32(((p_chr_1).wrapping_add(976)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) != (0i32)) as i32)) != 0)) as i32)) != 0) {
        F::BtlActChange_Finish(c, p_chr_1);
    }
    if (((((p_phase_2) == (3i32)) as i32)) != 0) {
        F::BtlChar_SetFlag(c, p_chr_1, ((((55i32) as i64 as u64)) as u32));
        if ((((((((!((F::BtlChar_TestFlag(c, p_chr_1, ((((252i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) && ((((!((F::BtlChar_TestFlag(c, p_chr_1, ((((253i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0)) as i32)) != 0) {
            F::BtlChar_SetFlag(c, p_chr_1, ((((245i32) as i64 as u64)) as u32));
            F::BtlChar_SetFlag(c, p_chr_1, ((((85i32) as i64 as u64)) as u32));
        }
        F::BtlChar_SetFlag(c, p_chr_1, ((((218i32) as i64 as u64)) as u32));
        F::ChrCam_EndCut(c, p_chr_1);
        F::BtlChar_SetFlag(c, p_chr_1, ((((205i32) as i64 as u64)) as u32));
        { let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlEvent_Raise(c, a0_, 77i32) };
        F::BtlChar_AddStageTimer(c, F32(0x40800000));
        break 'f 0;
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_Fusion", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_SwitchLeave(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_SwitchLeave", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let mut l_timer_3: u32 = 0;
    let r_ = 'f: {
    l_timer_3 = ((p_chr_1).wrapping_add(976)).wrapping_add((((2i32) as u32)).wrapping_mul(4));
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        F::BtlAnim_Play(c, p_chr_1, 238i32, F32(0x00000000));
        F::BtlAnim_PlaySub(c, p_chr_1, 47i32);
        { let v_ = F32(0x00000000); let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(152); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(156); c.m.w32(a_, (v_).0); };
        { let a1_ = { let a_ = ((F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(16)).wrapping_add(4); F32(c.m.r32(a_)) }; F::BtlMove_SetHeading(c, p_chr_1, a1_, F32(0x00000000)) };
        F::BtlChar_SavePlacement(c, p_chr_1);
        F::BtlChar_SetFlag(c, p_chr_1, ((((244i32) as i64 as u64)) as u32));
        F::BtlChar_ResetLook(c, p_chr_1);
        F::ChrCam_RequestCut(c, p_chr_1, 1i32, 2i32);
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        if ((((((c.m.r32((p_chr_1).wrapping_add(2404)) as i32)) == (1i32)) as i32)) != 0) {
            F::BtlChar_PlayVoice(c, p_chr_1, 16i32);
            { let a0_ = (c.m.r32(p_chr_1) as i32); let a1_ = (c.m.r32((p_chr_1).wrapping_add(3292)) as i32); let a2_ = (c.m.r32((p_chr_1).wrapping_add(3296)) as i32); let a3_ = (c.m.r32((p_chr_1).wrapping_add(3300)) as i32); let a4_ = (c.m.r32((p_chr_1).wrapping_add(3304)) as i32); let a5_ = (c.m.r32((p_chr_1).wrapping_add(3308)) as i32); let a6_ = (c.m.r32((p_chr_1).wrapping_add(3312)) as i32); F::BtlChange_RequestChara(c, a0_, a1_, a2_, a3_, a4_, a5_, a6_) };
        }
        's1: {
            let sv4: i32 = F::BtlAnim_GetId(c, p_chr_1);
            match sv4 {
                238i32 => {
                    if ((F::BtlAnim_Advance(c, p_chr_1, 0i32)) != 0) {
                        F::BtlChar_SetFlag(c, p_chr_1, ((((51i32) as i64 as u64)) as u32));
                        F::BtlAnim_RequestSubToMain(c, p_chr_1);
                        F::BtlAnim_SetBlend(c, p_chr_1, F32(0x3e199999));
                        { let a1_ = F::BtlParam_GetDashSound(c, p_chr_1); F::BtlCharSnd_PlayCommon(c, p_chr_1, a1_) };
                    }
                    F::BtlMove_Fall(c, p_chr_1);
                    break 's1;
                }
                47i32 => {
                    F::BtlAnim_AdvanceLoop(c, p_chr_1, 0i32);
                    F::BtlChar_SetFlag(c, p_chr_1, ((((24i32) as i64 as u64)) as u32));
                    F::BtlChar_SetFxBit(c, p_chr_1, 4i32);
                    F::BtlChar_SetFxBit(c, p_chr_1, 58i32);
                    if ((((({ let a_ = l_timer_3; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); n_ }) < (15i32)) as i32)) != 0) {
                        F::BtlMove_MoveVertical(c, p_chr_1, F32(0xc11425ec), F32(0x461c4000));
                    } else {
                        F::BtlMove_BrakeVertical(c, p_chr_1);
                    }
                    if ((((((c.m.r32(l_timer_3) as i32)) == (15i32)) as i32)) != 0) {
                        F::BtlChar_SetFxBit(c, p_chr_1, 12i32);
                        F::BtlCharSnd_PlayCommon(c, p_chr_1, 32i32);
                    }
                    if ((((((c.m.r32(l_timer_3) as i32)) >= (16i32)) as i32)) != 0) {
                        F::BtlChar_SetFlag(c, p_chr_1, ((((11i32) as i64 as u64)) as u32));
                    }
                    if ((((((((((c.m.r32(l_timer_3) as i32)) >= (46i32)) as i32)) != 0) && (({ let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlChange_IsLoadedFor(c, a0_) }) != 0)) as i32)) != 0) {
                        F::BtlAct_Request(c, p_chr_1, 244i32);
                        { let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlChange_SetReady(c, a0_) };
                        F::BtlChar_SetFxBit(c, p_chr_1, 5i32);
                        { let v_ = 0i32; let a_ = l_timer_3; c.m.w32(a_, (v_) as u32); };
                    }
                    break 's1;
                }
                _ => {
                }
            }
        }
        F::BtlMove_Step(c, p_chr_1, 6i32, 5i32, 7i32, F32(0x00000000), F32(0x3f6d0979));
        F::BtlActChange_SetFlags(c, p_chr_1);
        F::BtlChar_SetFlag(c, p_chr_1, ((((301i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((308i32) as i64 as u64)) as u32));
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_SwitchLeave", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_SwitchArriveWait(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_SwitchArriveWait", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let r_ = 'f: {
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        { let _ = D::call(c, fid::BtlAnim_Play, &[((p_chr_1) as u64), ((0i32) as i64 as u64), ((F32(0x00000000)).0 as u64)]); };
        { let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlChange_SetDone(c, a0_) };
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        { let _ = D::call(c, fid::BtlMove_Step, &[((p_chr_1) as u64), ((6i32) as i64 as u64), ((5i32) as i64 as u64), ((7i32) as i64 as u64), ((F32(0x00000000)).0 as u64), ((F32(0x3f6d0979)).0 as u64)]); };
        { let _ = D::call(c, fid::BtlActChange_SetFlags, &[((p_chr_1) as u64)]); };
        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((301i32) as i64 as u64)]); };
        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((308i32) as i64 as u64)]); };
        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((11i32) as i64 as u64)]); };
        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((204i32) as i64 as u64)]); };
    }
    if (((((p_phase_2) == (2i32)) as i32)) != 0) {
        { let _ = D::call(c, fid::BtlAct_Request, &[((p_chr_1) as u64), ((245i32) as i64 as u64)]); };
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_SwitchArriveWait", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_SwitchArriveLand(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_SwitchArriveLand", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let mut l_timer_3: u32 = 0;
    let r_ = 'f: {
    l_timer_3 = ((p_chr_1).wrapping_add(976)).wrapping_add((((2i32) as u32)).wrapping_mul(4));
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        { let _ = D::call(c, fid::BtlAnim_Play, &[((p_chr_1) as u64), ((45i32) as i64 as u64), ((F32(0x00000000)).0 as u64)]); };
        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((244i32) as i64 as u64)]); };
        { let _ = D::call(c, fid::ChrCam_RequestCut, &[((p_chr_1) as u64), ((1i32) as i64 as u64), ((3i32) as i64 as u64)]); };
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        's1: {
            let sv4: i32 = ((D::call(c, fid::BtlAnim_GetId, &[((p_chr_1) as u64)])) as i32);
            match sv4 {
                45i32 => {
                    if ((((({ let a_ = l_timer_3; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); n_ }) >= (8i32)) as i32)) != 0) {
                        if ((((D::call(c, fid::BtlAnim_AdvanceThen, &[((p_chr_1) as u64), ((0i32) as i64 as u64), ((F32(0x3e199999)).0 as u64), ((0i32) as i64 as u64)])) as i32)) != 0) {
                            { let _ = D::call(c, fid::BtlChar_SetFxBit, &[((p_chr_1) as u64), ((13i32) as i64 as u64)]); };
                            { let _ = D::call(c, fid::BtlCharSnd_PlayCommon, &[((p_chr_1) as u64), ((32i32) as i64 as u64)]); };
                            { let _ = D::call(c, fid::BtlChar_PlayVoice, &[((p_chr_1) as u64), ((15i32) as i64 as u64)]); };
                        }
                    }
                    if (((rt::flt(F32((D::call(c, fid::BtlAnim_GetProgress, &[((p_chr_1) as u64)])) as u32), F32(0x3f000000)) as i32)) != 0) {
                        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((11i32) as i64 as u64)]); };
                    }
                    break 's1;
                }
                38i32 | 0i32 => {
                    { let _ = ((D::call(c, fid::BtlAnim_Advance, &[((p_chr_1) as u64), ((0i32) as i64 as u64)])) as i32); };
                    if (((rt::fgt(F32((D::call(c, fid::BtlAnim_GetProgress, &[((p_chr_1) as u64)])) as u32), F32(0x3e4ccccc)) as i32)) != 0) {
                        { let v_ = 1i32; let a_ = ((p_chr_1).wrapping_add(976)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                        { let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlEvent_Raise(c, a0_, 91i32) };
                        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((205i32) as i64 as u64)]); };
                    }
                    break 's1;
                }
                _ => {
                }
            }
        }
        { let _ = D::call(c, fid::BtlMove_Step, &[((p_chr_1) as u64), ((6i32) as i64 as u64), ((5i32) as i64 as u64), ((7i32) as i64 as u64), ((F32(0x00000000)).0 as u64), ((F32(0x3f6d0979)).0 as u64)]); };
        { let _ = D::call(c, fid::BtlMove_BrakeVertical, &[((p_chr_1) as u64)]); };
        { let _ = D::call(c, fid::BtlActChange_SetFlags, &[((p_chr_1) as u64)]); };
        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((301i32) as i64 as u64)]); };
        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((308i32) as i64 as u64)]); };
    }
    if (((((((((p_phase_2) == (2i32)) as i32)) != 0) && ((((((c.m.r32(((p_chr_1).wrapping_add(976)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) != (0i32)) as i32)) != 0)) as i32)) != 0) {
        { let _ = D::call(c, fid::BtlActChange_Finish, &[((p_chr_1) as u64)]); };
    }
    if (((((p_phase_2) == (3i32)) as i32)) != 0) {
        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((55i32) as i64 as u64)]); };
        if ((((((((!((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((252i32) as i64 as u64)])) as i32)) != 0)) as i32)) != 0) && ((((!((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((253i32) as i64 as u64)])) as i32)) != 0)) as i32)) != 0)) as i32)) != 0) {
            { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((245i32) as i64 as u64)]); };
            { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((85i32) as i64 as u64)]); };
        }
        { let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlEvent_Raise(c, a0_, 78i32) };
        F::BtlChar_AddStageTimer(c, F32(0x40000000));
        break 'f 0;
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_SwitchArriveLand", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_KoSwitchLoad(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_KoSwitchLoad", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let r_ = 'f: {
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        { let a1_ = (if ((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((17i32) as i64 as u64)])) as i32)) != 0) { 216i32 } else { 214i32 }); { let _ = D::call(c, fid::BtlAnim_Play, &[((p_chr_1) as u64), ((a1_) as i64 as u64), ((F32(0x00000000)).0 as u64)]); } };
        { let v_ = F32(0x00000000); let a_ = (((D::call(c, fid::BtlChar_GetPos, &[((p_chr_1) as u64)])) as u32)).wrapping_add(152); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (((D::call(c, fid::BtlChar_GetPos, &[((p_chr_1) as u64)])) as u32)).wrapping_add(156); c.m.w32(a_, (v_).0); };
        F::BtlAct_PrepareSwitch(c, p_chr_1);
        if ((((D::call(c, fid::BtlActSwitch_SetLeaveCut, &[((p_chr_1) as u64)])) as i32)) != 0) {
            { let r_ = 1i32; let a_ = ((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        }
        F::BtlChar_ClearFlag(c, p_chr_1, ((((270i32) as i64 as u64)) as u32));
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        if ((((((c.m.r32((p_chr_1).wrapping_add(2404)) as i32)) == (1i32)) as i32)) != 0) {
            { let a0_ = (c.m.r32(p_chr_1) as i32); let a1_ = (c.m.r32((p_chr_1).wrapping_add(3292)) as i32); let a2_ = (c.m.r32((p_chr_1).wrapping_add(3296)) as i32); let a3_ = (c.m.r32((p_chr_1).wrapping_add(3300)) as i32); let a4_ = (c.m.r32((p_chr_1).wrapping_add(3304)) as i32); let a5_ = (c.m.r32((p_chr_1).wrapping_add(3308)) as i32); let a6_ = (c.m.r32((p_chr_1).wrapping_add(3312)) as i32); F::BtlChange_RequestChara(c, a0_, a1_, a2_, a3_, a4_, a5_, a6_) };
        } else {
            if ((((((((((c.m.r32((p_chr_1).wrapping_add(2404)) as i32)) >= (2i32)) as i32)) != 0) && (({ let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlChange_IsLoadedFor(c, a0_) }) != 0)) as i32)) != 0) {
                { let _ = D::call(c, fid::BtlAct_Request, &[((p_chr_1) as u64), ((247i32) as i64 as u64)]); };
                { let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlChange_SetReady(c, a0_) };
            }
        }
        { let _ = D::call(c, fid::BtlAnim_AdvanceLoop, &[((p_chr_1) as u64), ((0i32) as i64 as u64)]); };
        { let _ = D::call(c, fid::BtlMove_Step, &[((p_chr_1) as u64), ((6i32) as i64 as u64), ((5i32) as i64 as u64), ((7i32) as i64 as u64), ((F32(0x00000000)).0 as u64), ((F32(0x3f6d0979)).0 as u64)]); };
        { let _ = D::call(c, fid::BtlMove_BrakeVertical, &[((p_chr_1) as u64)]); };
        { let _ = D::call(c, fid::BtlActChange_SetFlags, &[((p_chr_1) as u64)]); };
        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((301i32) as i64 as u64)]); };
        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((308i32) as i64 as u64)]); };
        if (((((c.m.r32(((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) & (1i32))) != 0) {
            { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((11i32) as i64 as u64)]); };
        }
    }
    if (((((p_phase_2) == (3i32)) as i32)) != 0) {
        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((55i32) as i64 as u64)]); };
        break 'f 0;
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_KoSwitchLoad", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_KoSwitchEnterWait(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_KoSwitchEnterWait", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let r_ = 'f: {
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        { let _ = D::call(c, fid::BtlAnim_Play, &[((p_chr_1) as u64), ((0i32) as i64 as u64), ((F32(0x00000000)).0 as u64)]); };
        { let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlChange_SetDone(c, a0_) };
        { let _ = D::call(c, fid::BtlActSwitch_SaveEntryPlacement, &[((p_chr_1) as u64)]); };
        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((245i32) as i64 as u64)]); };
        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((85i32) as i64 as u64)]); };
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        { let _ = D::call(c, fid::BtlMove_Step, &[((p_chr_1) as u64), ((6i32) as i64 as u64), ((5i32) as i64 as u64), ((7i32) as i64 as u64), ((F32(0x00000000)).0 as u64), ((F32(0x3f6d0979)).0 as u64)]); };
        { let _ = D::call(c, fid::BtlActChange_SetFlags, &[((p_chr_1) as u64)]); };
        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((301i32) as i64 as u64)]); };
        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((308i32) as i64 as u64)]); };
        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((11i32) as i64 as u64)]); };
        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((204i32) as i64 as u64)]); };
    }
    if (((((p_phase_2) == (2i32)) as i32)) != 0) {
        { let _ = D::call(c, fid::BtlAct_Request, &[((p_chr_1) as u64), ((248i32) as i64 as u64)]); };
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_KoSwitchEnterWait", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_KoSwitchFlyIn(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_KoSwitchFlyIn", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_timer_3: u32 = 0;
    let mut l_landed_4: i32 = 0;
    let r_ = 'f: {
        l_timer_3 = ((p_chr_1).wrapping_add(976)).wrapping_add((((2i32) as u32)).wrapping_mul(4));
        if (((((p_phase_2) == (0i32)) as i32)) != 0) {
            { let _ = D::call(c, fid::BtlAnim_Play, &[((p_chr_1) as u64), ((25i32) as i64 as u64), ((F32(0x00000000)).0 as u64)]); };
            { let a0_ = (c.m.r32(p_chr_1) as i32); { let _ = F::BtlStage_GetStartPlace(c, a0_, fp, fp.wrapping_add(16), 0i32); } };
            F::BtlMove_TurnToPoint(c, p_chr_1, fp, F32(0x40490fda), F32(0x40490fda));
            F::BtlMove_TurnModelYaw(c, p_chr_1, F32(0x40490fda), F32(0x3f800000));
            { let a1_ = ((D::call(c, fid::BtlParam_GetDashSound, &[((p_chr_1) as u64)])) as i32); { let _ = D::call(c, fid::BtlCharSnd_PlayCommon, &[((p_chr_1) as u64), ((a1_) as i64 as u64)]); } };
            { let _ = D::call(c, fid::BtlActSwitch_SetEnterCut, &[((p_chr_1) as u64)]); };
        }
        if (((((p_phase_2) == (1i32)) as i32)) != 0) {
            's1: {
                let sv5: i32 = ((D::call(c, fid::BtlAnim_GetId, &[((p_chr_1) as u64)])) as i32);
                match sv5 {
                    25i32 => {
                        { let _ = D::call(c, fid::BtlAnim_AdvanceLoop, &[((p_chr_1) as u64), ((0i32) as i64 as u64)]); };
                        { let _ = D::call(c, fid::BtlMove_Step, &[((p_chr_1) as u64), ((6i32) as i64 as u64), ((6i32) as i64 as u64), ((3i32) as i64 as u64), ((F32(0x41b92f68)).0 as u64), ((F32(0x3f6d0979)).0 as u64)]); };
                        { let _ = D::call(c, fid::BtlMove_BrakeVertical, &[((p_chr_1) as u64)]); };
                        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((9i32) as i64 as u64)]); };
                        { let _ = D::call(c, fid::BtlChar_SetFxBit, &[((p_chr_1) as u64), ((4i32) as i64 as u64)]); };
                        { let _ = D::call(c, fid::BtlChar_SetFxBit, &[((p_chr_1) as u64), ((10i32) as i64 as u64)]); };
                        { let _ = D::call(c, fid::BtlChar_SetFxBit, &[((p_chr_1) as u64), ((58i32) as i64 as u64)]); };
                        break 's1;
                    }
                    45i32 => {
                        { let _ = ((D::call(c, fid::BtlAnim_AdvanceThen, &[((p_chr_1) as u64), ((0i32) as i64 as u64), ((F32(0x3e199999)).0 as u64), ((1i32) as i64 as u64)])) as i32); };
                        { let _ = D::call(c, fid::BtlMove_Step, &[((p_chr_1) as u64), ((6i32) as i64 as u64), ((5i32) as i64 as u64), ((3i32) as i64 as u64), ((F32(0x00000000)).0 as u64), ((F32(0x461c4000)).0 as u64)]); };
                        { let _ = D::call(c, fid::BtlMove_ApplyGravity, &[((p_chr_1) as u64)]); };
                        break 's1;
                    }
                    0i32 => {
                        { let _ = D::call(c, fid::BtlAnim_AdvanceLoop, &[((p_chr_1) as u64), ((0i32) as i64 as u64)]); };
                        { let _ = D::call(c, fid::BtlMove_Step, &[((p_chr_1) as u64), ((6i32) as i64 as u64), ((5i32) as i64 as u64), ((3i32) as i64 as u64), ((F32(0x00000000)).0 as u64), ((F32(0x3f6d0979)).0 as u64)]); };
                        { let _ = D::call(c, fid::BtlMove_ApplyGravity, &[((p_chr_1) as u64)]); };
                        break 's1;
                    }
                    _ => {
                    }
                }
            }
            { let _ = D::call(c, fid::BtlActChange_SetFlags, &[((p_chr_1) as u64)]); };
            { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((301i32) as i64 as u64)]); };
            { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((308i32) as i64 as u64)]); };
        }
        if (((((p_phase_2) == (2i32)) as i32)) != 0) {
            's2: {
                let sv6: i32 = ((D::call(c, fid::BtlAnim_GetId, &[((p_chr_1) as u64)])) as i32);
                match sv6 {
                    25i32 => {
                        {
                            l_landed_4 = (((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((15i32) as i64 as u64)])) as i32)) != (0i32)) as i32);
                            if ((((((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((31i32) as i64 as u64)])) as i32)) != 0) && ((((!((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((32i32) as i64 as u64)])) as i32)) != 0)) as i32)) != 0)) as i32)) != 0) {
                                l_landed_4 = 1i32;
                            }
                            if ((l_landed_4) != 0) {
                                { let _ = D::call(c, fid::BtlAnim_Request, &[((p_chr_1) as u64), ((45i32) as i64 as u64), ((F32(0x3e199999)).0 as u64)]); };
                                { let _ = D::call(c, fid::BtlChar_SetFxBit, &[((p_chr_1) as u64), ((53i32) as i64 as u64)]); };
                                { let _ = D::call(c, fid::BtlCharSnd_PlayCommon, &[((p_chr_1) as u64), ((37i32) as i64 as u64)]); };
                                { let _ = D::call(c, fid::BtlChar_PlayVoice, &[((p_chr_1) as u64), ((15i32) as i64 as u64)]); };
                                { let v_ = F32(0x00000000); let a_ = (((D::call(c, fid::BtlChar_GetPos, &[((p_chr_1) as u64)])) as u32)).wrapping_add(152); c.m.w32(a_, (v_).0); };
                                { let _ = D::call(c, fid::ChrCam_AddShake, &[((p_chr_1) as u64), ((F32(0x40400000)).0 as u64), ((F32(0x3f333333)).0 as u64)]); };
                                { let _ = D::call(c, fid::BtlChar_Vibrate, &[((p_chr_1) as u64), ((F32(0x3f800000)).0 as u64), ((F32(0x3f000000)).0 as u64)]); };
                            }
                            break 's2;
                        }
                    }
                    45i32 => {
                        break 's2;
                    }
                    0i32 => {
                        if ((((({ let a_ = l_timer_3; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); n_ }) == (30i32)) as i32)) != 0) {
                            { let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlEvent_Raise(c, a0_, 91i32) };
                        }
                        if ((((((c.m.r32(l_timer_3) as i32)) > (30i32)) as i32)) != 0) {
                            { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((249i32) as i64 as u64)]); };
                            { let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlEvent_Raise(c, a0_, 78i32) };
                        }
                        break 's2;
                    }
                    _ => {
                    }
                }
            }
        }
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_KoSwitchFlyIn", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_RoundReset(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_RoundReset", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let r_ = 'f: {
    if (((((p_phase_2) == (2i32)) as i32)) != 0) {
        { let _ = D::call(c, fid::BtlActChange_Finish, &[((p_chr_1) as u64)]); };
    }
    if (((((p_phase_2) == (3i32)) as i32)) != 0) {
        { let _ = D::call(c, fid::BtlChar_SetFxBit, &[((p_chr_1) as u64), ((5i32) as i64 as u64)]); };
        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((213i32) as i64 as u64)]); };
        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((205i32) as i64 as u64)]); };
        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((55i32) as i64 as u64)]); };
        F::BtlChar_AddStageTimer(c, F32(0x40000000));
        break 'f 0;
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_RoundReset", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSkill_PlayAnim(c: &mut Ctx, mut p_chr_1: u32, mut p_cls_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSkill_PlayAnim", &PROBE_N, &[(p_chr_1) as u32, (p_cls_2) as u32], false); }
    let mut l_anim_3: i32 = 0;
    let r_ = 'f: {
    l_anim_3 = 0i32;
    's1: {
        let sv4: i32 = p_cls_2;
        match sv4 {
            0i32 => {
                l_anim_3 = 259i32;
                break 's1;
            }
            1i32 => {
                l_anim_3 = 260i32;
                break 's1;
            }
            _ => {
            }
        }
    }
    { let _ = D::call(c, fid::BtlAnim_Play, &[((p_chr_1) as u64), ((l_anim_3) as i64 as u64), ((F32(0x3e199999)).0 as u64)]); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSkill_PlayAnim", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSkill_SetFxBit(c: &mut Ctx, mut p_chr_1: u32, mut p_cls_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSkill_SetFxBit", &PROBE_N, &[(p_chr_1) as u32, (p_cls_2) as u32], false); }
    let mut l_bit_3: i32 = 0;
    let r_ = 'f: {
    l_bit_3 = 0i32;
    's1: {
        let sv4: i32 = p_cls_2;
        match sv4 {
            0i32 => {
                l_bit_3 = 60i32;
                break 's1;
            }
            1i32 => {
                l_bit_3 = 61i32;
                break 's1;
            }
            _ => {
            }
        }
    }
    { let _ = D::call(c, fid::BtlChar_SetFxBit, &[((p_chr_1) as u64), ((l_bit_3) as i64 as u64)]); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSkill_SetFxBit", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSkill_SetFlag(c: &mut Ctx, mut p_chr_1: u32, mut p_cls_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSkill_SetFlag", &PROBE_N, &[(p_chr_1) as u32, (p_cls_2) as u32], false); }
    let mut l_flag_3: i32 = 0;
    let r_ = 'f: {
    l_flag_3 = 0i32;
    's1: {
        let sv4: i32 = p_cls_2;
        match sv4 {
            0i32 => {
                l_flag_3 = 222i32;
                break 's1;
            }
            1i32 => {
                l_flag_3 = 223i32;
                break 's1;
            }
            _ => {
            }
        }
    }
    { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((l_flag_3) as i64 as u64)]); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSkill_SetFlag", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSkill_Apply(c: &mut Ctx, mut p_chr_1: u32, mut p_slot_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSkill_Apply", &PROBE_N, &[(p_chr_1) as u32, (p_slot_2) as u32], false); }
    let mut l_idx_3: i32 = 0;
    let mut l_id_4: i32 = 0;
    let mut l_attr_5: i32 = 0;
    let mut l_health_6: i32 = 0;
    let mut l_ki_7: i32 = 0;
    let mut l_time_8: i32 = 0;
    let mut l_kind_9: i32 = 0;
    let mut l_lv0_10: i32 = 0;
    let mut l_lv1_11: i32 = 0;
    let mut l_lv2_12: i32 = 0;
    let mut l_lv3_13: i32 = 0;
    let mut l_valE_14: i32 = 0;
    let mut l_valF_15: i32 = 0;
    let mut l_add_16: i32 = 0;
    let r_ = 'f: {
    l_idx_3 = (((p_slot_2) != (0i32)) as i32);
    l_id_4 = F::BtlSkill_GetId(c, p_chr_1, p_slot_2);
    l_attr_5 = ((((F::BtlSkill_GetFlags(c, p_chr_1, p_slot_2)) as u64)) as i32);
    { let _ = F::BtlSkill_GetBlastCost(c, p_chr_1, p_slot_2); };
    l_health_6 = F::BtlSkill_GetHealthChange(c, p_chr_1, p_slot_2);
    l_ki_7 = F::BtlSkill_GetKiChange(c, p_chr_1, p_slot_2);
    l_time_8 = F::BtlSkill_GetFrames(c, p_chr_1, p_slot_2);
    l_kind_9 = F::BtlSkill_GetStatKind(c, p_chr_1, p_slot_2);
    l_lv0_10 = F::BtlSkill_GetStatLevel0(c, p_chr_1, p_slot_2);
    l_lv1_11 = F::BtlSkill_GetStatLevel1(c, p_chr_1, p_slot_2);
    l_lv2_12 = F::BtlSkill_GetStatLevel2(c, p_chr_1, p_slot_2);
    l_lv3_13 = F::BtlSkill_GetStatLevel3(c, p_chr_1, p_slot_2);
    l_valE_14 = F::BtlSkill_GetValE(c, p_chr_1, p_slot_2);
    l_valF_15 = F::BtlSkill_GetValF(c, p_chr_1, p_slot_2);
    l_add_16 = (((l_attr_5).wrapping_shr((8i32) as u32)) & (1i32));
    's1: {
        let sv17: i32 = l_id_4;
        match sv17 {
            12i32 | 50i32 => {
                if ((l_add_16) != 0) {
                    { let v_ = { let a0_ = ((c.m.r32((p_chr_1).wrapping_add(3584)) as i32)).wrapping_add(1i32); F::BtlUtil_Min(c, a0_, 3i32) }; let a_ = (p_chr_1).wrapping_add(3584); c.m.w32(a_, (v_) as u32); };
                } else {
                    { let v_ = 1i32; let a_ = (p_chr_1).wrapping_add(3584); c.m.w32(a_, (v_) as u32); };
                }
                { let v_ = p_slot_2; let a_ = (p_chr_1).wrapping_add(3596); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            51i32 => {
                { let v_ = l_time_8; let a_ = (p_chr_1).wrapping_add(3592); c.m.w32(a_, (v_) as u32); };
                { let v_ = p_slot_2; let a_ = (p_chr_1).wrapping_add(3596); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            55i32 => {
                if ((l_add_16) != 0) {
                    { let v_ = { let a0_ = ((c.m.r32((p_chr_1).wrapping_add(3588)) as i32)).wrapping_add(1i32); F::BtlUtil_Min(c, a0_, 3i32) }; let a_ = (p_chr_1).wrapping_add(3588); c.m.w32(a_, (v_) as u32); };
                } else {
                    { let v_ = 1i32; let a_ = (p_chr_1).wrapping_add(3588); c.m.w32(a_, (v_) as u32); };
                }
                { let v_ = p_slot_2; let a_ = (p_chr_1).wrapping_add(3596); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            23i32 => {
                l_health_6 = (3000i32).wrapping_neg();
                break 's1;
            }
            24i32 => {
                { let _ = D::call(c, fid::BtlChar_SetHeldFlag, &[((p_chr_1) as u64), ((152i32) as i64 as u64)]); };
                { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((153i32) as i64 as u64)]); };
                l_ki_7 = { let a_ = (F::BtlMember_GetActiveGauge(c, p_chr_1)).wrapping_add(16); (c.m.r32(a_) as i32) };
                if (((((l_time_8) > (0i32)) as i32)) != 0) {
                    { let v_ = rt::div_i32(l_ki_7, l_time_8); let a_ = (p_chr_1).wrapping_add(3600); c.m.w32(a_, (v_) as u32); };
                }
                break 's1;
            }
            65i32 => {
                { let v_ = { let a0_ = ((c.m.r32((p_chr_1).wrapping_add(3676)) as i32)).wrapping_add(1i32); F::BtlUtil_Clamp(c, a0_, 0i32, 3i32) }; let a_ = (p_chr_1).wrapping_add(3676); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            _ => {
            }
        }
    }
    if ((((l_attr_5) & (512i32))) != 0) {
        l_ki_7 = { let a_ = (F::BtlMember_GetActiveGauge(c, p_chr_1)).wrapping_add(16); (c.m.r32(a_) as i32) };
        F::BtlMember_AddMaxPower(c, p_chr_1, 30000i32);
        { let _ = D::call(c, fid::BtlChar_SetHeldFlag, &[((p_chr_1) as u64), ((6i32) as i64 as u64)]); };
        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((187i32) as i64 as u64)]); };
    }
    if ((((l_attr_5) & (8388608i32))) != 0) {
        F::BtlStat_SetPenalty(c, p_chr_1, 2i32, l_idx_3, (10i32).wrapping_neg());
    }
    if ((((l_attr_5) & (2048i32))) != 0) {
        's2: {
            let sv18: i32 = l_kind_9;
            match sv18 {
                0i32 => {
                    break 's2;
                }
                1i32 => {
                    { let v_ = l_time_8; let a_ = (p_chr_1).wrapping_add(3604); c.m.w32(a_, (v_) as u32); };
                    break 's2;
                }
                2i32 => {
                    { let v_ = 1i32; let a_ = (p_chr_1).wrapping_add(3632); c.m.w32(a_, (v_) as u32); };
                    break 's2;
                }
                3i32 => {
                    { let v_ = 1i32; let a_ = (p_chr_1).wrapping_add(3636); c.m.w32(a_, (v_) as u32); };
                    break 's2;
                }
                4i32 => {
                    break 's2;
                }
                _ => {
                }
            }
        }
    }
    if ((((l_attr_5) & (1024i32))) != 0) {
        { let v_ = l_time_8; let a_ = (p_chr_1).wrapping_add(3608); c.m.w32(a_, (v_) as u32); };
    }
    if (((((l_health_6) > (0i32)) as i32)) != 0) {
        F::BtlMember_AddHealth(c, p_chr_1, l_health_6);
    }
    if (((((l_health_6) < (0i32)) as i32)) != 0) {
        { let _ = D::call(c, fid::BtlMember_Damage, &[((p_chr_1) as u64), (((l_health_6).wrapping_neg()) as i64 as u64), ((1051i32) as i64 as u64)]); };
    }
    if (((((l_ki_7) > (0i32)) as i32)) != 0) {
        { let _ = F::BtlMember_AddKi(c, p_chr_1, l_ki_7); };
    }
    if (((((l_ki_7) < (0i32)) as i32)) != 0) {
        { let _ = F::BtlMember_SpendKi(c, p_chr_1, (l_ki_7).wrapping_neg(), 1i32); };
    }
    's3: {
        let sv19: i32 = l_kind_9;
        match sv19 {
            0i32 => {
                F::BtlStat_SetBase(c, p_chr_1, 0i32, l_idx_3, l_lv0_10, l_add_16);
                F::BtlStat_SetBase(c, p_chr_1, 1i32, l_idx_3, l_lv1_11, l_add_16);
                F::BtlStat_SetBase(c, p_chr_1, 2i32, l_idx_3, l_lv2_12, l_add_16);
                F::BtlStat_SetBase(c, p_chr_1, 3i32, l_idx_3, l_lv3_13, l_add_16);
                break 's3;
            }
            1i32 => {
                F::BtlStat_SetTimed(c, p_chr_1, 0i32, l_idx_3, l_lv0_10, l_add_16, l_time_8);
                F::BtlStat_SetTimed(c, p_chr_1, 1i32, l_idx_3, l_lv1_11, l_add_16, l_time_8);
                F::BtlStat_SetTimed(c, p_chr_1, 2i32, l_idx_3, l_lv2_12, l_add_16, l_time_8);
                F::BtlStat_SetTimed(c, p_chr_1, 3i32, l_idx_3, l_lv3_13, l_add_16, l_time_8);
                break 's3;
            }
            2i32 => {
                F::BtlStat_SetKind2(c, p_chr_1, 0i32, l_idx_3, l_lv0_10, l_add_16);
                F::BtlStat_SetKind2(c, p_chr_1, 1i32, l_idx_3, l_lv1_11, l_add_16);
                F::BtlStat_SetKind2(c, p_chr_1, 2i32, l_idx_3, l_lv2_12, l_add_16);
                F::BtlStat_SetKind2(c, p_chr_1, 3i32, l_idx_3, l_lv3_13, l_add_16);
                break 's3;
            }
            3i32 => {
                F::BtlStat_SetKind3(c, p_chr_1, 0i32, l_idx_3, l_lv0_10, l_add_16);
                F::BtlStat_SetKind3(c, p_chr_1, 1i32, l_idx_3, l_lv1_11, l_add_16);
                F::BtlStat_SetKind3(c, p_chr_1, 2i32, l_idx_3, l_lv2_12, l_add_16);
                F::BtlStat_SetKind3(c, p_chr_1, 3i32, l_idx_3, l_lv3_13, l_add_16);
                break 's3;
            }
            4i32 => {
                F::BtlStat_SetKind4(c, p_chr_1, 0i32, l_idx_3, l_lv0_10, l_add_16);
                F::BtlStat_SetKind4(c, p_chr_1, 1i32, l_idx_3, l_lv1_11, l_add_16);
                F::BtlStat_SetKind4(c, p_chr_1, 2i32, l_idx_3, l_lv2_12, l_add_16);
                F::BtlStat_SetKind4(c, p_chr_1, 3i32, l_idx_3, l_lv3_13, l_add_16);
                break 's3;
            }
            _ => {
            }
        }
    }
    if (((((l_valE_14) > (0i32)) as i32)) != 0) {
        { let v_ = l_valE_14; let a_ = (p_chr_1).wrapping_add(3616); c.m.w32(a_, (v_) as u32); };
        { let v_ = l_time_8; let a_ = (p_chr_1).wrapping_add(3612); c.m.w32(a_, (v_) as u32); };
    }
    if (((((l_valF_15) > (0i32)) as i32)) != 0) {
        { let v_ = l_valF_15; let a_ = (p_chr_1).wrapping_add(3624); c.m.w32(a_, (v_) as u32); };
        { let v_ = l_time_8; let a_ = (p_chr_1).wrapping_add(3620); c.m.w32(a_, (v_) as u32); };
    }
    { let a_ = ((F::BtlMember_GetActiveGauge(c, p_chr_1)).wrapping_add(56)).wrapping_add((((p_slot_2) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
    if (((((l_kind_9) != (0i32)) as i32)) != 0) {
        { let v_ = 1i32; let a_ = ((p_chr_1).wrapping_add(3640)).wrapping_add((((l_idx_3) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    }
    { let a1_ = F::BtlSkill_GetBlastCost(c, p_chr_1, p_slot_2); { let _ = D::call(c, fid::BtlMember_SubBlast, &[((p_chr_1) as u64), ((a1_) as i64 as u64)]); } };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSkill_Apply", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSkill_UpdateTimers(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSkill_UpdateTimers", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    if ((F::BtlChars_IsTimeStopped(c)) != 0) {
        break 'f;
    }
    if ((((({ let a_ = (p_chr_1).wrapping_add(3612); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) < (0i32)) as i32)) != 0) {
        { let v_ = 0i32; let a_ = (p_chr_1).wrapping_add(3612); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (p_chr_1).wrapping_add(3616); c.m.w32(a_, (v_) as u32); };
    }
    if ((((({ let a_ = (p_chr_1).wrapping_add(3620); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) < (0i32)) as i32)) != 0) {
        { let v_ = 0i32; let a_ = (p_chr_1).wrapping_add(3620); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (p_chr_1).wrapping_add(3624); c.m.w32(a_, (v_) as u32); };
    }
    { let a_ = (p_chr_1).wrapping_add(3604); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
    if ((((((c.m.r32((p_chr_1).wrapping_add(3604)) as i32)) < (0i32)) as i32)) != 0) {
        { let v_ = 0i32; let a_ = (p_chr_1).wrapping_add(3604); c.m.w32(a_, (v_) as u32); };
    }
    if ((((((c.m.r32((p_chr_1).wrapping_add(3632)) as i32)) != (0i32)) as i32)) != 0) {
        { let v_ = 1i32; let a_ = (p_chr_1).wrapping_add(3604); c.m.w32(a_, (v_) as u32); };
    }
    if ((((((c.m.r32((p_chr_1).wrapping_add(3636)) as i32)) != (0i32)) as i32)) != 0) {
        { let v_ = 1i32; let a_ = (p_chr_1).wrapping_add(3604); c.m.w32(a_, (v_) as u32); };
    }
    { let a_ = (p_chr_1).wrapping_add(3608); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
    if ((((((c.m.r32((p_chr_1).wrapping_add(3608)) as i32)) < (0i32)) as i32)) != 0) {
        { let v_ = 0i32; let a_ = (p_chr_1).wrapping_add(3608); c.m.w32(a_, (v_) as u32); };
    }
    if ((((((c.m.r32((p_chr_1).wrapping_add(3592)) as i32)) > (0i32)) as i32)) != 0) {
        { let a_ = (p_chr_1).wrapping_add(3592); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((86i32) as i64 as u64)]); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSkill_UpdateTimers", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_SkillBasic(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_SkillBasic", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let mut l_cls_3: i32 = 0;
    let mut l_attr_4: i32 = 0;
    let mut l_obj_5: u32 = 0;
    let r_ = 'f: {
    l_cls_3 = F::BtlAct_GetCurrentClass(c, p_chr_1);
    l_attr_4 = ((((F::BtlSkill_GetFlags(c, p_chr_1, l_cls_3)) as u64)) as i32);
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        F::BtlSkill_PlayAnim(c, p_chr_1, l_cls_3);
        F::BtlSkill_SetFxBit(c, p_chr_1, l_cls_3);
        F::BtlSkill_SetFlag(c, p_chr_1, l_cls_3);
        if ((((!((((l_attr_4) & (128i32))) != 0)) as i32)) != 0) {
            F::BtlSkill_Apply(c, p_chr_1, l_cls_3);
        }
        if ((((l_attr_4) & (2i32))) != 0) {
            { let _ = D::call(c, fid::BtlMove_TurnYaw, &[((p_chr_1) as u64), ((2i32) as i64 as u64), ((F32(0x40490fda)).0 as u64)]); };
            F::BtlMove_TurnPitch(c, p_chr_1, 5i32, F32(0x40490fda));
        }
        if ((((l_attr_4) & (4i32))) != 0) {
            { let _ = D::call(c, fid::BtlMove_TurnYaw, &[((p_chr_1) as u64), ((2i32) as i64 as u64), ((F32(0x40490fda)).0 as u64)]); };
            F::BtlMove_TurnPitch(c, p_chr_1, 5i32, F32(0x40490fda));
            F::BtlMove_TurnModelYaw(c, p_chr_1, F32(0x40490fda), F32(0x3f800000));
        }
        if ((((l_attr_4) & (262144i32))) != 0) {
            { let _ = D::call(c, fid::ChrCam_RequestCut, &[((p_chr_1) as u64), ((1i32) as i64 as u64), ((5i32) as i64 as u64)]); };
        }
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        if ((((((((!((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((14i32) as i64 as u64)])) as i32)) != 0)) as i32)) != 0) && ((((!((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((15i32) as i64 as u64)])) as i32)) != 0)) as i32)) != 0)) as i32)) != 0) {
            { let _ = D::call(c, fid::BtlChar_SetHeldFlag, &[((p_chr_1) as u64), ((14i32) as i64 as u64)]); };
        }
        if ((((D::call(c, fid::BtlAnim_Advance, &[((p_chr_1) as u64), ((0i32) as i64 as u64)])) as i32)) != 0) {
            { let _ = D::call(c, fid::BtlAct_Request, &[((p_chr_1) as u64), ((11i32) as i64 as u64)]); };
            if ((((l_attr_4) & (128i32))) != 0) {
                F::BtlSkill_Apply(c, p_chr_1, l_cls_3);
            }
            { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((154i32) as i64 as u64)]); };
        }
        { let _ = D::call(c, fid::BtlMove_Step, &[((p_chr_1) as u64), ((6i32) as i64 as u64), ((6i32) as i64 as u64), ((7i32) as i64 as u64), ((F32(0x00000000)).0 as u64), ((F32(0x3f6d0979)).0 as u64)]); };
        { let _ = D::call(c, fid::BtlMove_ApplyGravity, &[((p_chr_1) as u64)]); };
        if ((((((((F::BtlSkill_GetFlags(c, p_chr_1, l_cls_3)) as u64)) as i32)) & (16777216i32))) != 0) {
            { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((66i32) as i64 as u64)]); };
            { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((67i32) as i64 as u64)]); };
        }
        if ((((((((F::BtlSkill_GetFlags(c, p_chr_1, l_cls_3)) as u64)) as i32)) & (33554432i32))) != 0) {
            { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((68i32) as i64 as u64)]); };
        }
        if ((((((((F::BtlSkill_GetFlags(c, p_chr_1, l_cls_3)) as u64)) as i32)) & (67108864i32))) != 0) {
            { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((70i32) as i64 as u64)]); };
        }
        if ((((((((F::BtlSkill_GetFlags(c, p_chr_1, l_cls_3)) as u64)) as i32)) & (134217728i32))) != 0) {
            { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((69i32) as i64 as u64)]); };
        }
        { let _ = ((((F::BtlSkill_GetFlags(c, p_chr_1, l_cls_3)) as u64)) as i32); };
        if ((((l_attr_4) & (262144i32))) != 0) {
            { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((293i32) as i64 as u64)]); };
            { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((298i32) as i64 as u64)]); };
            { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((66i32) as i64 as u64)]); };
            { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((67i32) as i64 as u64)]); };
            { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((68i32) as i64 as u64)]); };
            { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((69i32) as i64 as u64)]); };
            { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((70i32) as i64 as u64)]); };
            { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((296i32) as i64 as u64)]); };
            { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((187i32) as i64 as u64)]); };
        }
        if (((((F::BtlSkill_GetId(c, p_chr_1, l_cls_3)) == (1i32)) as i32)) != 0) {
            { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((311i32) as i64 as u64)]); };
        }
    }
    if (((((p_phase_2) == (3i32)) as i32)) != 0) {
        's1: {
            let sv6: i32 = F::BtlSkill_GetId(c, p_chr_1, l_cls_3);
            match sv6 {
                24i32 => {
                    if ((((!((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((152i32) as i64 as u64)])) as i32)) != 0)) as i32)) != 0) {
                        l_obj_5 = ((D::call(c, fid::BtlChar_GetObj, &[((p_chr_1) as u64)])) as u32);
                        if ((((c.m.r32((l_obj_5).wrapping_add(2624))) & (((262144i32) as u32)))) != 0) {
                            F::BtlObj_SetColorMode(c, l_obj_5, 2i32, 0i32);
                            break 'f 0;
                        }
                    }
                    break 's1;
                }
                1i32 => {
                    { let _ = D::call(c, fid::BtlChar_SetHeldFlag, &[((p_chr_1) as u64), (((l_cls_3).wrapping_add(76i32)) as i64 as u64)]); };
                    break 'f 0;
                }
                _ => {
                }
            }
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_SkillBasic", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_SkillTeleport(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_SkillTeleport", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let mut l_timer_3: u32 = 0;
    let mut l_cls_4: i32 = 0;
    let r_ = 'f: {
    l_timer_3 = ((p_chr_1).wrapping_add(976)).wrapping_add((((2i32) as u32)).wrapping_mul(4));
    l_cls_4 = F::BtlAct_GetCurrentClass(c, p_chr_1);
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        F::BtlSkill_PlayAnim(c, p_chr_1, l_cls_4);
        F::BtlSkill_SetFlag(c, p_chr_1, l_cls_4);
        { let _ = D::call(c, fid::BtlChar_SetHeldFlag, &[((p_chr_1) as u64), ((14i32) as i64 as u64)]); };
        F::BtlSkill_Apply(c, p_chr_1, l_cls_4);
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        if ((((!((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((49i32) as i64 as u64)])) as i32)) != 0)) as i32)) != 0) {
            { let _ = ((D::call(c, fid::BtlAnim_Advance, &[((p_chr_1) as u64), ((0i32) as i64 as u64)])) as i32); };
            if ((((D::call(c, fid::BtlAnim_PassedRatio, &[((p_chr_1) as u64), ((F32(0x3f4ccccc)).0 as u64)])) as i32)) != 0) {
                { let _ = D::call(c, fid::BtlChar_SetFxBit, &[((p_chr_1) as u64), ((12i32) as i64 as u64)]); };
                { let _ = D::call(c, fid::BtlCharSnd_PlayCommon, &[((p_chr_1) as u64), ((32i32) as i64 as u64)]); };
                F::BtlChar_SetSmallVibration(c, p_chr_1, F32(0x3e4ccccc));
            }
            { let _ = D::call(c, fid::BtlMove_Step, &[((p_chr_1) as u64), ((6i32) as i64 as u64), ((6i32) as i64 as u64), ((7i32) as i64 as u64), ((F32(0x00000000)).0 as u64), ((F32(0x3f6d0979)).0 as u64)]); };
            { let _ = D::call(c, fid::BtlMove_ApplyGravity, &[((p_chr_1) as u64)]); };
        } else {
            { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((66i32) as i64 as u64)]); };
            { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((67i32) as i64 as u64)]); };
            { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((68i32) as i64 as u64)]); };
            { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((69i32) as i64 as u64)]); };
            { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((70i32) as i64 as u64)]); };
            { let v_ = F32(0x00000000); let a_ = (((D::call(c, fid::BtlChar_GetPos, &[((p_chr_1) as u64)])) as u32)).wrapping_add(152); c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x00000000); let a_ = (((D::call(c, fid::BtlChar_GetPos, &[((p_chr_1) as u64)])) as u32)).wrapping_add(156); c.m.w32(a_, (v_).0); };
            's1: {
                let sv5: i32 = { let a_ = l_timer_3; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); o_ };
                match sv5 {
                    5i32 => {
                        if ((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((5i32) as i64 as u64)])) as i32)) != 0) {
                            F::BtlMove_SnapToOpponent(c, p_chr_1);
                            { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((204i32) as i64 as u64)]); };
                        }
                        break 's1;
                    }
                    6i32 => {
                        if ((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((5i32) as i64 as u64)])) as i32)) != 0) {
                            F::BtlMove_WarpBehindOpponent(c, p_chr_1, F32(0x00000000));
                            { let _ = D::call(c, fid::BtlMove_TurnYaw, &[((p_chr_1) as u64), ((2i32) as i64 as u64), ((F32(0x40490fda)).0 as u64)]); };
                            F::BtlMove_TurnPitch(c, p_chr_1, 1i32, F32(0x40490fda));
                            F::BtlMove_TurnModelYaw(c, p_chr_1, F32(0x40490fda), F32(0x3f800000));
                            F::BtlMove_SetDirection(c, p_chr_1, 3i32);
                            { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((63i32) as i64 as u64)]); };
                            { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((204i32) as i64 as u64)]); };
                            { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((36i32) as i64 as u64)]); };
                            { let _ = D::call(c, fid::BtlChar_SetFxBit, &[((p_chr_1) as u64), ((13i32) as i64 as u64)]); };
                        }
                        break 's1;
                    }
                    7i32 => {
                        if ((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((5i32) as i64 as u64)])) as i32)) != 0) {
                            { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((205i32) as i64 as u64)]); };
                            { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((213i32) as i64 as u64)]); };
                        }
                        break 's1;
                    }
                    9i32 => {
                        { let _ = D::call(c, fid::BtlAct_Request, &[((p_chr_1) as u64), ((11i32) as i64 as u64)]); };
                        break 's1;
                    }
                    _ => {
                    }
                }
            }
            if ((((((c.m.r32(l_timer_3) as i32)) >= (4i32)) as i32)) != 0) {
                { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((38i32) as i64 as u64)]); };
            }
            if ((((((c.m.r32(l_timer_3) as i32)) < (9i32)) as i32)) != 0) {
                { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((11i32) as i64 as u64)]); };
            }
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_SkillTeleport", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_SkillInstant(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_SkillInstant", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let mut l_cls_3: i32 = 0;
    let r_ = 'f: {
    l_cls_3 = F::BtlAct_GetCurrentClass(c, p_chr_1);
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        F::BtlSkill_Apply(c, p_chr_1, l_cls_3);
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        { let _ = ((D::call(c, fid::BtlAnim_Advance, &[((p_chr_1) as u64), ((0i32) as i64 as u64)])) as i32); };
        { let _ = D::call(c, fid::BtlMove_Step, &[((p_chr_1) as u64), ((6i32) as i64 as u64), ((6i32) as i64 as u64), ((7i32) as i64 as u64), ((F32(0x00000000)).0 as u64), ((F32(0x3f6d0979)).0 as u64)]); };
        { let _ = D::call(c, fid::BtlMove_ApplyGravity, &[((p_chr_1) as u64)]); };
    }
    if (((((p_phase_2) == (2i32)) as i32)) != 0) {
        { let _ = D::call(c, fid::BtlAct_Request, &[((p_chr_1) as u64), ((11i32) as i64 as u64)]); };
        if ((F::BtlDecide_Main(c, p_chr_1, 524288i32)) != 0) {
            { let a1_ = ((D::call(c, fid::BtlAct_GetQueued, &[((p_chr_1) as u64)])) as i32); { let _ = D::call(c, fid::BtlAct_Request, &[((p_chr_1) as u64), ((a1_) as i64 as u64)]); } };
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_SkillInstant", &PROBE_N, &[], true); }
    r_
}

pub fn BtlDecide_Common(c: &mut Ctx, mut p_chr_1: u32, mut p_mask_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlDecide_Common", &PROBE_N, &[(p_chr_1) as u32, (p_mask_2) as u32], false); }
    let r_ = 'f: {
    if ((((((((p_mask_2) & (8i32))) != 0) && ((((((c.m.r32((p_chr_1).wrapping_add(4064)) as i32)) > (0i32)) as i32)) != 0)) as i32)) != 0) {
        F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), 211i32);
        break 'f 1i32;
    }
    if ((((((((p_mask_2) & (4i32))) != 0) && ((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((190i32) as i64 as u64)])) as i32)) != 0)) as i32)) != 0) {
        F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), 234i32);
        break 'f 1i32;
    }
    if ((((((((((((((((p_mask_2) & (1i32))) != 0) && ((((!((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((14i32) as i64 as u64)])) as i32)) != 0)) as i32)) != 0)) as i32)) != 0) && ((((!((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((15i32) as i64 as u64)])) as i32)) != 0)) as i32)) != 0)) as i32)) != 0) && ((((!((((D::call(c, fid::BtlChar_TestPrevFlag, &[((p_chr_1) as u64), ((15i32) as i64 as u64)])) as i32)) != 0)) as i32)) != 0)) as i32)) != 0) {
        F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), 17i32);
        break 'f 1i32;
    }
    if ((((((((p_mask_2) & (2i32))) != 0) && ((((D::call(c, fid::BtlChar_IsDead, &[((p_chr_1) as u64)])) as i32)) != 0)) as i32)) != 0) {
        F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), 210i32);
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlDecide_Common", &PROBE_N, &[], true); }
    r_
}

pub fn BtlDecide_Main(c: &mut Ctx, mut p_chr_1: u32, mut p_mask_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlDecide_Main", &PROBE_N, &[(p_chr_1) as u32, (p_mask_2) as u32], false); }
    let mut l_ok_3: i32 = 0;
    let r_ = 'f: {
    if ((((!((F::BtlChar_IsFree(c, p_chr_1)) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((((((p_mask_2) & (268435456i32))) != 0) && ((F::BtlMember_HasHealth(c, p_chr_1, 5000i32)) != 0)) as i32)) != 0) {
        F::BtlColl_AddActionBit(c, p_chr_1, 66i32);
        if ((F::BtlInput_TestAction(c, p_chr_1, 46i32, 1i32)) != 0) {
            F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), 66i32);
            break 'f 1i32;
        }
    }
    if ((((((((p_mask_2) & (1024i32))) != 0) && ((F::BtlMember_HasKi(c, p_chr_1, 1000i32)) != 0)) as i32)) != 0) {
        if ((F::BtlInput_TestAction(c, p_chr_1, 21i32, 1i32)) != 0) {
            F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), 26i32);
            break 'f 1i32;
        }
        if ((((((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((5i32) as i64 as u64)])) as i32)) != 0) && ((F::BtlInput_TestAction(c, p_chr_1, 20i32, 1i32)) != 0)) as i32)) != 0) {
            F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), 25i32);
            break 'f 1i32;
        }
    }
    if ((((((((((((p_mask_2) & (134217728i32))) != 0) && ((F::BtlMember_HasKi(c, p_chr_1, 10000i32)) != 0)) as i32)) != 0) && ((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((5i32) as i64 as u64)])) as i32)) != 0)) as i32)) != 0) {
        l_ok_3 = 0i32;
        if ((((!((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((19i32) as i64 as u64)])) as i32)) != 0)) as i32)) != 0) {
            l_ok_3 = 1i32;
        }
        if (((((((D::call(c, fid::BtlOpp_GetSeenAction, &[((p_chr_1) as u64)])) as i32)) == (213i32)) as i32)) != 0) {
            l_ok_3 = 1i32;
        }
        if ((((((l_ok_3) != 0) && ((F::BtlInput_TestAction(c, p_chr_1, 20i32, 1i32)) != 0)) as i32)) != 0) {
            F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), 51i32);
            break 'f 1i32;
        }
    }
    if ((((((((p_mask_2) & (2048i32))) != 0) && ((F::BtlInput_TestAction(c, p_chr_1, 15i32, 1i32)) != 0)) as i32)) != 0) {
        F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), 23i32);
        break 'f 1i32;
    }
    if ((((((((((((p_mask_2) & (4096i32))) != 0) && ((((!((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((15i32) as i64 as u64)])) as i32)) != 0)) as i32)) != 0)) as i32)) != 0) && ((F::BtlInput_TestAction(c, p_chr_1, 17i32, 1i32)) != 0)) as i32)) != 0) {
        F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), 24i32);
        break 'f 1i32;
    }
    if ((((((((((((((((((((p_mask_2) & (6i32))) != 0) && ((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((15i32) as i64 as u64)])) as i32)) != 0)) as i32)) != 0) && ((((!((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((17i32) as i64 as u64)])) as i32)) != 0)) as i32)) != 0)) as i32)) != 0) && ((((!((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((14i32) as i64 as u64)])) as i32)) != 0)) as i32)) != 0)) as i32)) != 0) && ((F::BtlInput_TestAction(c, p_chr_1, 8i32, 1i32)) != 0)) as i32)) != 0) {
        if ((((p_mask_2) & (2i32))) != 0) {
            F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), 16i32);
        } else {
            F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), 18i32);
        }
        break 'f 1i32;
    }
    if ((((((((((((p_mask_2) & (24i32))) != 0) && ((((((((!((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((19i32) as i64 as u64)])) as i32)) != 0)) as i32)) != 0) || ((((p_mask_2) & (16i32))) != 0)) as i32)) != 0)) as i32)) != 0) && ((F::BtlInput_TestAction(c, p_chr_1, 3i32, 1i32)) != 0)) as i32)) != 0) {
        F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), 15i32);
        break 'f 1i32;
    }
    if ((((((((((((p_mask_2) & (24576i32))) != 0) && ((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((5i32) as i64 as u64)])) as i32)) != 0)) as i32)) != 0) && ((((((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((19i32) as i64 as u64)])) as i32)) != 0) || ((((p_mask_2) & (16384i32))) != 0)) as i32)) != 0)) as i32)) != 0) {
        if ((F::BtlInput_TestAction(c, p_chr_1, 25i32, 1i32)) != 0) {
            F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), 27i32);
            break 'f 1i32;
        }
        if ((F::BtlInput_TestAction(c, p_chr_1, 26i32, 1i32)) != 0) {
            F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), 28i32);
            break 'f 1i32;
        }
        if ((F::BtlInput_TestAction(c, p_chr_1, 27i32, 1i32)) != 0) {
            F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), 29i32);
            break 'f 1i32;
        }
    }
    if ((((((((((((((((((((p_mask_2) & (98304i32))) != 0) && ((((((((F::BtlParam_GetFlags2(c, p_chr_1)) as u64)) as i32)) & (1i32))) != 0)) as i32)) != 0) && ((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((5i32) as i64 as u64)])) as i32)) != 0)) as i32)) != 0) && ((((((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((19i32) as i64 as u64)])) as i32)) != 0) || ((((p_mask_2) & (65536i32))) != 0)) as i32)) != 0)) as i32)) != 0) && ((F::BtlInput_TestAction(c, p_chr_1, 28i32, 1i32)) != 0)) as i32)) != 0) {
        F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), 30i32);
        break 'f 1i32;
    }
    if ((((((((((((((((p_mask_2) & (131072i32))) != 0) && ((((((((F::BtlParam_GetFlags2(c, p_chr_1)) as u64)) as i32)) & (8i32))) != 0)) as i32)) != 0) && ((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((5i32) as i64 as u64)])) as i32)) != 0)) as i32)) != 0) && (({ let a1_ = F::BtlParam_GetAmountA(c, p_chr_1); F::BtlMember_HasKi(c, p_chr_1, a1_) }) != 0)) as i32)) != 0) {
        if ((F::BtlInput_TestAction(c, p_chr_1, 32i32, 1i32)) != 0) {
            F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), 35i32);
            break 'f 1i32;
        }
        if ((F::BtlInput_TestAction(c, p_chr_1, 29i32, 1i32)) != 0) {
            F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), 32i32);
            break 'f 1i32;
        }
        if ((F::BtlInput_TestAction(c, p_chr_1, 30i32, 1i32)) != 0) {
            F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), 33i32);
            break 'f 1i32;
        }
        if ((F::BtlInput_TestAction(c, p_chr_1, 31i32, 1i32)) != 0) {
            F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), 34i32);
            break 'f 1i32;
        }
    }
    if ((((((((((((((((((((((((p_mask_2) & (262144i32))) != 0) && ((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((5i32) as i64 as u64)])) as i32)) != 0)) as i32)) != 0) && ((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((19i32) as i64 as u64)])) as i32)) != 0)) as i32)) != 0) && (({ let a1_ = F::BtlParam_GetAmountB(c, p_chr_1); F::BtlMember_HasKi(c, p_chr_1, a1_) }) != 0)) as i32)) != 0) && ((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((179i32) as i64 as u64)])) as i32)) != 0)) as i32)) != 0) && ((((((c.m.r32((p_chr_1).wrapping_add(4224)) as i32)) > (0i32)) as i32)) != 0)) as i32)) != 0) {
        F::BtlColl_AddActionBit(c, p_chr_1, 36i32);
        if ((F::BtlInput_TestAction(c, p_chr_1, 33i32, 1i32)) != 0) {
            F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), 36i32);
            break 'f 1i32;
        }
    }
    if ((((((((((((p_mask_2) & (536870912i32))) != 0) && ((F::BtlAct_TestPoweredSkill(c, p_chr_1, ((8i32) as u32))) != 0)) as i32)) != 0) && ((F::BtlInput_TestAction(c, p_chr_1, 49i32, 1i32)) != 0)) as i32)) != 0) {
        F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), 53i32);
        break 'f 1i32;
    }
    if ((((((((p_mask_2) & (1048576i32))) != 0) && ((F::BtlAct_CheckTechniqueInput(c, p_chr_1)) != 0)) as i32)) != 0) {
        break 'f 1i32;
    }
    if ((((((((p_mask_2) & (8388608i32))) != 0) && ((F::BtlAct_CheckSkillInput(c, p_chr_1)) != 0)) as i32)) != 0) {
        break 'f 1i32;
    }
    if ((((((((((((p_mask_2) & (128i32))) != 0) && ((((!((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((6i32) as i64 as u64)])) as i32)) != 0)) as i32)) != 0)) as i32)) != 0) && ((((!((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((152i32) as i64 as u64)])) as i32)) != 0)) as i32)) != 0)) as i32)) != 0) {
        if ((((F::BtlParam_GetUnk0(c, p_chr_1)) & (128i32))) != 0) {
            if ((((((((((F::BtlMember_IsKiFull(c, p_chr_1)) != 0) && ((F::BtlMember_HasBlast(c, p_chr_1, 100000i32)) != 0)) as i32)) != 0) && ((F::BtlInput_TestAction(c, p_chr_1, 19i32, 1i32)) != 0)) as i32)) != 0) {
                F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), 55i32);
                break 'f 1i32;
            }
        } else {
            if ((F::BtlInput_TestAction(c, p_chr_1, 19i32, 1i32)) != 0) {
                F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), 55i32);
                break 'f 1i32;
            }
        }
    }
    if ((((((((p_mask_2) & (16777216i32))) != 0) && ((((((((F::BtlParam_GetFlags2(c, p_chr_1)) as u64)) as i32)) & (32i32))) != 0)) as i32)) != 0) {
        if ((((((D::call(c, fid::BtlParam_GetFlags, &[((p_chr_1) as u64)])) as i32)) & (2i32))) != 0) {
            if ((((((F::BtlMove_IsBlastIncoming(c, p_chr_1, 1i32, 1i32, 1i32, F32(0x42b40000))) != 0) && ((F::BtlInput_TestAction(c, p_chr_1, 43i32, 1i32)) != 0)) as i32)) != 0) {
                F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), 61i32);
                break 'f 1i32;
            }
        } else {
            if ((((((F::BtlMove_IsBlastIncoming(c, p_chr_1, 1i32, 1i32, 0i32, F32(0x42b40000))) != 0) && ((F::BtlInput_TestAction(c, p_chr_1, 43i32, 1i32)) != 0)) as i32)) != 0) {
                F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), 59i32);
                break 'f 1i32;
            }
        }
    }
    if ((((((((((((((((p_mask_2) & (33554432i32))) != 0) && ((((((((F::BtlParam_GetFlags2(c, p_chr_1)) as u64)) as i32)) & (32i32))) != 0)) as i32)) != 0) && ((F::BtlMove_IsBlastIncoming(c, p_chr_1, 0i32, 1i32, 0i32, F32(0x40a00000))) != 0)) as i32)) != 0) && ((F::BtlInput_TestAction(c, p_chr_1, 39i32, 1i32)) != 0)) as i32)) != 0) {
        F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), 60i32);
        break 'f 1i32;
    }
    if ((((((((p_mask_2) & (524288i32))) != 0) && ((F::BtlInput_TestAction(c, p_chr_1, 34i32, 1i32)) != 0)) as i32)) != 0) {
        if ((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((5i32) as i64 as u64)])) as i32)) != 0) {
            if (((rt::fgt({ let a0_ = F::BtlOpp_GetYawFromFacing(c, p_chr_1); rt::ffabs(a0_) }, F32(0x3fd99999)) as i32)) != 0) {
                F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), 58i32);
                F::BtlAct_SetQueue(c, p_chr_1, ((1i32) as u32), 56i32);
                break 'f 2i32;
            }
            F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), 56i32);
            break 'f 1i32;
        }
        F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), 56i32);
        break 'f 1i32;
    }
    if ((((((((((((p_mask_2) & (32i32))) != 0) && ((((((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((14i32) as i64 as u64)])) as i32)) != 0) || ((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((17i32) as i64 as u64)])) as i32)) != 0)) as i32)) != 0)) as i32)) != 0) && ((F::BtlInput_TestAction(c, p_chr_1, 10i32, 1i32)) != 0)) as i32)) != 0) {
        { let a2_ = F::BtlAct_GetAction14or16(c, p_chr_1); F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), a2_) };
        break 'f 1i32;
    }
    if ((((((((((((p_mask_2) & (64i32))) != 0) && ((((((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((14i32) as i64 as u64)])) as i32)) != 0) || ((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((17i32) as i64 as u64)])) as i32)) != 0)) as i32)) != 0)) as i32)) != 0) && ((((!((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((15i32) as i64 as u64)])) as i32)) != 0)) as i32)) != 0)) as i32)) != 0) {
        if ((((((((!((F::BtlParam_CanFly(c, p_chr_1)) != 0)) as i32)) != 0) && ((((!((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((17i32) as i64 as u64)])) as i32)) != 0)) as i32)) != 0)) as i32)) != 0) {
            if ((F::BtlInput_TestAction(c, p_chr_1, 14i32, 1i32)) != 0) {
                F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), 17i32);
                break 'f 1i32;
            }
        } else {
            if ((F::BtlInput_TestAction(c, p_chr_1, 12i32, 1i32)) != 0) {
                F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), 21i32);
                break 'f 1i32;
            }
        }
    }
    if ((((((((p_mask_2) & (4194304i32))) != 0) && ((F::BtlAct_CheckSwitchInput(c, p_chr_1)) != 0)) as i32)) != 0) {
        break 'f 1i32;
    }
    if ((((((((p_mask_2) & (256i32))) != 0) && ((F::BtlAct_CheckTransformInput(c, p_chr_1)) != 0)) as i32)) != 0) {
        break 'f 1i32;
    }
    if ((((((((p_mask_2) & (512i32))) != 0) && ((F::BtlAct_CheckFusionInput(c, p_chr_1)) != 0)) as i32)) != 0) {
        break 'f 1i32;
    }
    if ((((((((p_mask_2) & (1i32))) != 0) && ((F::BtlInput_TestAction(c, p_chr_1, 1i32, 1i32)) != 0)) as i32)) != 0) {
        if ((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((19i32) as i64 as u64)])) as i32)) != 0) {
            if (((((((D::call(c, fid::BtlAct_GetCurrent, &[((p_chr_1) as u64)])) as i32)) != (14i32)) as i32)) != 0) {
                F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), 14i32);
                break 'f 1i32;
            }
        } else {
            if (((((((D::call(c, fid::BtlAct_GetCurrent, &[((p_chr_1) as u64)])) as i32)) != (13i32)) as i32)) != 0) {
                F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), 13i32);
                break 'f 1i32;
            }
        }
    }
    if ((((p_mask_2) & (2097152i32))) != 0) {
        if ((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((5i32) as i64 as u64)])) as i32)) != 0) {
            break 'f 0i32;
        }
        if ((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((147i32) as i64 as u64)])) as i32)) != 0) {
            break 'f 0i32;
        }
        if ((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((3i32) as i64 as u64)])) as i32)) != 0) {
            F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), 54i32);
            break 'f 1i32;
        }
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlDecide_Main", &PROBE_N, &[], true); }
    r_
}

pub fn BtlDecide_Attack(c: &mut Ctx, mut p_chr_1: u32, mut p_mask_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlDecide_Attack", &PROBE_N, &[(p_chr_1) as u32, (p_mask_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_n_3: i32 = 0;
    let mut l_i_4: i32 = 0;
    let mut l_n_5: i32 = 0;
    let mut l_i_6: i32 = 0;
    let mut l_act_7: i32 = 0;
    let mut l_n_8: i32 = 0;
    let mut l_kind_9: i32 = 0;
    let mut l_input_10: i32 = 0;
    let mut l_i_11: i32 = 0;
    let mut l_ret_12: i32 = 0;
    let r_ = 'f: {
        if ((((!((F::BtlChar_IsFree(c, p_chr_1)) != 0)) as i32)) != 0) {
            break 'f 0i32;
        }
        if ((((p_mask_2) & (1i32))) != 0) {
            F::BtlColl_AddActionBit(c, p_chr_1, 68i32);
            if ((F::BtlInput_TestAction(c, p_chr_1, 52i32, 1i32)) != 0) {
                if ((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((5i32) as i64 as u64)])) as i32)) != 0) {
                    if (((rt::fgt({ let a0_ = F::BtlOpp_GetYawFromFacing(c, p_chr_1); rt::ffabs(a0_) }, F32(0x3fd99999)) as i32)) != 0) {
                        F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), 58i32);
                        F::BtlAct_SetQueue(c, p_chr_1, ((1i32) as u32), 68i32);
                        break 'f 2i32;
                    }
                    F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), 68i32);
                    break 'f 1i32;
                }
                F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), 68i32);
                break 'f 1i32;
            }
        }
        if ((((p_mask_2) & (2i32))) != 0) {
            F::BtlColl_AddActionBit(c, p_chr_1, 71i32);
            if ((F::BtlInput_TestAction(c, p_chr_1, 53i32, 1i32)) != 0) {
                F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), 71i32);
                break 'f 1i32;
            }
            if ((F::BtlInput_TestAction(c, p_chr_1, 54i32, 1i32)) != 0) {
                F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), 72i32);
                break 'f 1i32;
            }
            if ((F::BtlInput_TestAction(c, p_chr_1, 55i32, 1i32)) != 0) {
                F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), 73i32);
                break 'f 1i32;
            }
            if ((F::BtlInput_TestAction(c, p_chr_1, 56i32, 1i32)) != 0) {
                F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), 74i32);
                break 'f 1i32;
            }
            if ((F::BtlInput_TestAction(c, p_chr_1, 57i32, 1i32)) != 0) {
                if ((((((c.m.r32((p_chr_1).wrapping_add(3424)) as i32)) < (5i32)) as i32)) != 0) {
                    F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), 75i32);
                } else {
                    F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), 76i32);
                }
                break 'f 1i32;
            }
        }
        if ((((p_mask_2) & (8192i32))) != 0) {
            F::BtlColl_AddActionBit(c, p_chr_1, 85i32);
            if ((F::BtlInput_TestAction(c, p_chr_1, 58i32, 1i32)) != 0) {
                F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), 85i32);
                break 'f 1i32;
            }
            if ((F::BtlInput_TestAction(c, p_chr_1, 59i32, 1i32)) != 0) {
                F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), 86i32);
                break 'f 1i32;
            }
            if ((F::BtlInput_TestAction(c, p_chr_1, 60i32, 1i32)) != 0) {
                F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), 83i32);
                break 'f 1i32;
            }
            if ((F::BtlInput_TestAction(c, p_chr_1, 61i32, 1i32)) != 0) {
                F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), 84i32);
                break 'f 1i32;
            }
            if ((F::BtlInput_TestAction(c, p_chr_1, 62i32, 1i32)) != 0) {
                F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), 87i32);
                break 'f 1i32;
            }
        }
        if ((((((((p_mask_2) & (67108864i32))) != 0) && ((((((((F::BtlParam_GetFlags2(c, p_chr_1)) as u64)) as i32)) & (65536i32))) != 0)) as i32)) != 0) {
            F::BtlColl_AddActionBit(c, p_chr_1, 109i32);
            if ((F::BtlInput_TestAction(c, p_chr_1, 74i32, 1i32)) != 0) {
                F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), 109i32);
                break 'f 1i32;
            }
            if ((F::BtlInput_TestAction(c, p_chr_1, 75i32, 1i32)) != 0) {
                F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), 110i32);
                break 'f 1i32;
            }
            if ((F::BtlInput_TestAction(c, p_chr_1, 72i32, 1i32)) != 0) {
                F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), 107i32);
                break 'f 1i32;
            }
            if ((F::BtlInput_TestAction(c, p_chr_1, 73i32, 1i32)) != 0) {
                F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), 108i32);
                break 'f 1i32;
            }
            if ((F::BtlInput_TestAction(c, p_chr_1, 76i32, 1i32)) != 0) {
                F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), 110i32);
                break 'f 1i32;
            }
        }
        if ((((((((((((((((p_mask_2) & (4i32))) != 0) && (({ let a1_ = F::BtlKiBlast_GetKiCostOf(c, p_chr_1, ((((0i32) as i64 as u64)) as u32)); F::BtlMember_HasKi(c, p_chr_1, a1_) }) != 0)) as i32)) != 0) && ((((((c.m.r32((p_chr_1).wrapping_add(3552)) as i32)) < (F::BtlParam_GetCount80(c, p_chr_1))) as i32)) != 0)) as i32)) != 0) && ((F::BtlMove_CanFireBlast(c, p_chr_1, 0i32, ((0i32) as u32))) != 0)) as i32)) != 0) {
            F::BtlColl_AddActionBit(c, p_chr_1, 174i32);
            if ((F::BtlInput_TestAction(c, p_chr_1, 89i32, 1i32)) != 0) {
                if ((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((5i32) as i64 as u64)])) as i32)) != 0) {
                    if (((rt::fgt({ let a0_ = F::BtlOpp_GetYawFromFacing(c, p_chr_1); rt::ffabs(a0_) }, F32(0x3fd99999)) as i32)) != 0) {
                        F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), 58i32);
                        F::BtlAct_SetQueue(c, p_chr_1, ((1i32) as u32), 174i32);
                        break 'f 2i32;
                    }
                    F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), 174i32);
                    break 'f 1i32;
                }
                F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), 174i32);
                break 'f 1i32;
            }
        }
        if ((((((((((((((((p_mask_2) & (8i32))) != 0) && (({ let a1_ = F::BtlKiBlast_GetKiCostOf(c, p_chr_1, ((((3i32) as i64 as u64)) as u32)); F::BtlMember_HasKi(c, p_chr_1, a1_) }) != 0)) as i32)) != 0) && ((((((c.m.r32((p_chr_1).wrapping_add(3552)) as i32)) < (F::BtlParam_GetCount80(c, p_chr_1))) as i32)) != 0)) as i32)) != 0) && ((F::BtlMove_CanFireBlast(c, p_chr_1, 1i32, ((0i32) as u32))) != 0)) as i32)) != 0) {
            F::BtlColl_AddActionBit(c, p_chr_1, 175i32);
            if ((F::BtlInput_TestAction(c, p_chr_1, 90i32, 1i32)) != 0) {
                F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), 175i32);
                break 'f 1i32;
            }
        }
        if ((((((((((((((((((((p_mask_2) & (16384i32))) != 0) && ((((((((F::BtlParam_GetFlags2(c, p_chr_1)) as u64)) as i32)) & (16i32))) != 0)) as i32)) != 0) && (({ let a1_ = F::BtlKiBlast_GetKiCostOf(c, p_chr_1, ((((4i32) as i64 as u64)) as u32)); F::BtlMember_HasKi(c, p_chr_1, a1_) }) != 0)) as i32)) != 0) && ((((((c.m.r32((p_chr_1).wrapping_add(3552)) as i32)) < (F::BtlParam_GetCount80(c, p_chr_1))) as i32)) != 0)) as i32)) != 0) && ((F::BtlMove_CanFireBlast(c, p_chr_1, 0i32, ((0i32) as u32))) != 0)) as i32)) != 0) {
            F::BtlColl_AddActionBit(c, p_chr_1, 176i32);
            if ((F::BtlInput_TestAction(c, p_chr_1, 89i32, 1i32)) != 0) {
                F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), 176i32);
                break 'f 1i32;
            }
        }
        if ((((((((((((((((((((p_mask_2) & (32768i32))) != 0) && ((((((((F::BtlParam_GetFlags2(c, p_chr_1)) as u64)) as i32)) & (16i32))) != 0)) as i32)) != 0) && (({ let a1_ = F::BtlKiBlast_GetKiCostOf(c, p_chr_1, ((((7i32) as i64 as u64)) as u32)); F::BtlMember_HasKi(c, p_chr_1, a1_) }) != 0)) as i32)) != 0) && ((((((c.m.r32((p_chr_1).wrapping_add(3552)) as i32)) < (F::BtlParam_GetCount80(c, p_chr_1))) as i32)) != 0)) as i32)) != 0) && ((F::BtlMove_CanFireBlast(c, p_chr_1, 1i32, ((0i32) as u32))) != 0)) as i32)) != 0) {
            F::BtlColl_AddActionBit(c, p_chr_1, 177i32);
            if ((F::BtlInput_TestAction(c, p_chr_1, 90i32, 1i32)) != 0) {
                F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), 177i32);
                break 'f 1i32;
            }
        }
        if ((((((((p_mask_2) & (65536i32))) != 0) && ((F::BtlInput_TestAction(c, p_chr_1, 64i32, 1i32)) != 0)) as i32)) != 0) {
            F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), 89i32);
            break 'f 1i32;
        }
        if ((((((((((((((((((((p_mask_2) & (131072i32))) != 0) && ((((((((F::BtlParam_GetFlags2(c, p_chr_1)) as u64)) as i32)) & (16i32))) != 0)) as i32)) != 0) && (({ let a1_ = F::BtlKiBlast_GetKiCostOf(c, p_chr_1, ((((8i32) as i64 as u64)) as u32)); F::BtlMember_HasKi(c, p_chr_1, a1_) }) != 0)) as i32)) != 0) && ((((((c.m.r32((p_chr_1).wrapping_add(3552)) as i32)) < (F::BtlParam_GetCount80(c, p_chr_1))) as i32)) != 0)) as i32)) != 0) && ((F::BtlMove_CanFireBlast(c, p_chr_1, 0i32, ((0i32) as u32))) != 0)) as i32)) != 0) {
            F::BtlColl_AddActionBit(c, p_chr_1, 178i32);
            if ((F::BtlInput_TestAction(c, p_chr_1, 89i32, 1i32)) != 0) {
                F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), 178i32);
                break 'f 1i32;
            }
        }
        if ((((((((((((((((((((p_mask_2) & (262144i32))) != 0) && ((((((((F::BtlParam_GetFlags2(c, p_chr_1)) as u64)) as i32)) & (16i32))) != 0)) as i32)) != 0) && (({ let a1_ = F::BtlKiBlast_GetKiCostOf(c, p_chr_1, ((((11i32) as i64 as u64)) as u32)); F::BtlMember_HasKi(c, p_chr_1, a1_) }) != 0)) as i32)) != 0) && ((((((c.m.r32((p_chr_1).wrapping_add(3552)) as i32)) < (F::BtlParam_GetCount80(c, p_chr_1))) as i32)) != 0)) as i32)) != 0) && ((F::BtlMove_CanFireBlast(c, p_chr_1, 1i32, ((0i32) as u32))) != 0)) as i32)) != 0) {
            F::BtlColl_AddActionBit(c, p_chr_1, 179i32);
            if ((F::BtlInput_TestAction(c, p_chr_1, 90i32, 1i32)) != 0) {
                F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), 179i32);
                break 'f 1i32;
            }
        }
        if ((((((((p_mask_2) & (16i32))) != 0) && ((((((((F::BtlParam_GetFlags2(c, p_chr_1)) as u64)) as i32)) & (2048i32))) != 0)) as i32)) != 0) {
            F::BtlColl_AddActionBit(c, p_chr_1, 94i32);
            if ((F::BtlInput_TestAction(c, p_chr_1, 77i32, 1i32)) != 0) {
                F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), 94i32);
                break 'f 1i32;
            }
        }
        if ((((((((p_mask_2) & (8388608i32))) != 0) && ((((((((F::BtlParam_GetFlags2(c, p_chr_1)) as u64)) as i32)) & (2048i32))) != 0)) as i32)) != 0) {
            F::BtlColl_AddActionBit(c, p_chr_1, 94i32);
            if ((F::BtlInput_TestAction(c, p_chr_1, 77i32, 1i32)) != 0) {
                { let _ = F::memset(c, fp, 0i32, ((((((8u32) as i32)) as i64 as u64)) as u32)); };
                l_n_3 = 0i32;
                's1: {
                    let sv13: i32 = F::BtlParam_GetUnk8A(c, p_chr_1, ((((1i32) as i64 as u64)) as u32));
                    match sv13 {
                        0i32 => {
                            { let v_ = 94i32; let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                            l_n_3 = 1i32;
                            break 's1;
                        }
                        1i32 => {
                            { let v_ = 64i32; let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                            { let v_ = 94i32; let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                            l_n_3 = 2i32;
                            break 's1;
                        }
                        _ => {
                        }
                    }
                }
                if (((((l_n_3) > (0i32)) as i32)) != 0) {
                    {
                        l_i_4 = 0i32;
                        'l2: loop {
                            if !(((((l_i_4) < (l_n_3)) as i32)) != 0) { break 'l2; }
                            'c3: {
                                { let a2_ = (c.m.r32((fp).wrapping_add((((l_i_4) as u32)).wrapping_mul(4))) as i32); F::BtlAct_SetQueue(c, p_chr_1, ((l_i_4) as u32), a2_) };
                            }
                            { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
                        }
                    }
                    break 'f l_n_3;
                }
            }
        }
        if ((((((((p_mask_2) & (32i32))) != 0) && ((((((((F::BtlParam_GetFlags2(c, p_chr_1)) as u64)) as i32)) & (4096i32))) != 0)) as i32)) != 0) {
            F::BtlColl_AddActionBit(c, p_chr_1, 95i32);
            if ((F::BtlInput_TestAction(c, p_chr_1, 78i32, 1i32)) != 0) {
                F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), 95i32);
                break 'f 1i32;
            }
        }
        if ((((((((p_mask_2) & (16777216i32))) != 0) && ((((((((F::BtlParam_GetFlags2(c, p_chr_1)) as u64)) as i32)) & (4096i32))) != 0)) as i32)) != 0) {
            F::BtlColl_AddActionBit(c, p_chr_1, 95i32);
            if ((F::BtlInput_TestAction(c, p_chr_1, 78i32, 1i32)) != 0) {
                { let _ = F::memset(c, fp.wrapping_add(8), 0i32, ((((((8u32) as i32)) as i64 as u64)) as u32)); };
                l_n_5 = 0i32;
                's4: {
                    let sv14: i32 = F::BtlParam_GetUnk8A(c, p_chr_1, ((((2i32) as i64 as u64)) as u32));
                    match sv14 {
                        0i32 => {
                            { let v_ = 95i32; let a_ = (fp.wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                            l_n_5 = 1i32;
                            break 's4;
                        }
                        1i32 => {
                            { let v_ = 64i32; let a_ = (fp.wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                            { let v_ = 95i32; let a_ = (fp.wrapping_add(8)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                            l_n_5 = 2i32;
                            break 's4;
                        }
                        _ => {
                        }
                    }
                }
                if (((((l_n_5) > (0i32)) as i32)) != 0) {
                    {
                        l_i_6 = 0i32;
                        'l5: loop {
                            if !(((((l_i_6) < (l_n_5)) as i32)) != 0) { break 'l5; }
                            'c6: {
                                { let a2_ = (c.m.r32((fp.wrapping_add(8)).wrapping_add((((l_i_6) as u32)).wrapping_mul(4))) as i32); F::BtlAct_SetQueue(c, p_chr_1, ((l_i_6) as u32), a2_) };
                            }
                            { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; };
                        }
                    }
                    break 'f l_n_5;
                }
            }
        }
        if ((((p_mask_2) & (2097152i32))) != 0) {
            l_act_7 = 0i32;
            's7: {
                let sv15: i32 = { let a1_ = rt::rem_i32(((c.m.r32((p_chr_1).wrapping_add(3424)) as i32)).wrapping_add(4i32), 5i32); F::BtlParam_GetUnk84(c, p_chr_1, ((((a1_) as i64 as u64)) as u32)) };
                match sv15 {
                    0i32 => {
                        l_act_7 = 96i32;
                        break 's7;
                    }
                    1i32 => {
                        l_act_7 = 97i32;
                        break 's7;
                    }
                    2i32 => {
                        l_act_7 = 98i32;
                        break 's7;
                    }
                    3i32 => {
                        l_act_7 = 99i32;
                        break 's7;
                    }
                    4i32 => {
                        l_act_7 = 102i32;
                        break 's7;
                    }
                    5i32 => {
                        l_act_7 = (if ((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((143i32) as i64 as u64)])) as i32)) != 0) { 100i32 } else { 101i32 });
                        break 's7;
                    }
                    _ => {
                    }
                }
            }
            F::BtlColl_AddActionBit(c, p_chr_1, l_act_7);
            if ((F::BtlInput_TestAction(c, p_chr_1, 66i32, 1i32)) != 0) {
                F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), l_act_7);
                break 'f 1i32;
            }
        }
        if ((((p_mask_2) & (171966464i32))) != 0) {
            { let _ = F::memset(c, fp.wrapping_add(16), 0i32, ((((((8u32) as i32)) as i64 as u64)) as u32)); };
            l_n_8 = 0i32;
            l_kind_9 = (1i32).wrapping_neg();
            l_input_10 = (1i32).wrapping_neg();
            if ((((p_mask_2) & (4194304i32))) != 0) {
                l_kind_9 = 0i32;
                l_input_10 = 66i32;
            } else {
                if ((((p_mask_2) & (33554432i32))) != 0) {
                    l_kind_9 = 3i32;
                    l_input_10 = 66i32;
                } else {
                    if (((((F::BtlParam_GetCount80(c, p_chr_1)) <= (0i32)) as i32)) != 0) {
                        l_kind_9 = 4i32;
                        l_input_10 = 88i32;
                    }
                }
            }
            if (((((l_kind_9) >= (0i32)) as i32)) != 0) {
                's8: {
                    let sv16: i32 = F::BtlParam_GetUnk8A(c, p_chr_1, ((((l_kind_9) as i64 as u64)) as u32));
                    'k20: {
                    'k19: {
                    'k18: {
                    'k17: {
                    'k16: {
                    'k15: {
                    'k14: {
                    'k13: {
                    'k12: {
                    'k11: {
                    'k10: {
                    'k9: {
                        match sv16 { 1i32 => break 'k9, 0i32 => break 'k10, 3i32 => break 'k11, 2i32 => break 'k12, 5i32 => break 'k13, 4i32 => break 'k14, 7i32 => break 'k15, 6i32 => break 'k16, 9i32 => break 'k17, 8i32 => break 'k18, 11i32 => break 'k19, 10i32 => break 'k20, _ => break 's8, }
                    }
                    { let v_ = 64i32; let a_ = (fp.wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    l_n_8 = 1i32;
                    }
                    { let v_ = 96i32; let a_ = (fp.wrapping_add(16)).wrapping_add(((({ let o_ = l_n_8; let n_ = (o_).wrapping_add(1); l_n_8 = n_; o_ }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    break 's8;
                    }
                    { let v_ = 64i32; let a_ = (fp.wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    l_n_8 = 1i32;
                    }
                    { let v_ = 97i32; let a_ = (fp.wrapping_add(16)).wrapping_add(((({ let o_ = l_n_8; let n_ = (o_).wrapping_add(1); l_n_8 = n_; o_ }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    break 's8;
                    }
                    { let v_ = 64i32; let a_ = (fp.wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    l_n_8 = 1i32;
                    }
                    { let v_ = 98i32; let a_ = (fp.wrapping_add(16)).wrapping_add(((({ let o_ = l_n_8; let n_ = (o_).wrapping_add(1); l_n_8 = n_; o_ }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    break 's8;
                    }
                    { let v_ = 64i32; let a_ = (fp.wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    l_n_8 = 1i32;
                    }
                    { let v_ = 99i32; let a_ = (fp.wrapping_add(16)).wrapping_add(((({ let o_ = l_n_8; let n_ = (o_).wrapping_add(1); l_n_8 = n_; o_ }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    break 's8;
                    }
                    { let v_ = 64i32; let a_ = (fp.wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    l_n_8 = 1i32;
                    }
                    { let v_ = 102i32; let a_ = (fp.wrapping_add(16)).wrapping_add(((({ let o_ = l_n_8; let n_ = (o_).wrapping_add(1); l_n_8 = n_; o_ }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    break 's8;
                    }
                    { let v_ = 64i32; let a_ = (fp.wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    l_n_8 = 1i32;
                    }
                    { let v_ = (if ((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((143i32) as i64 as u64)])) as i32)) != 0) { 100i32 } else { 101i32 }); let a_ = (fp.wrapping_add(16)).wrapping_add(((({ let o_ = l_n_8; let n_ = (o_).wrapping_add(1); l_n_8 = n_; o_ }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    break 's8;
                }
                if (((((l_n_8) > (0i32)) as i32)) != 0) {
                    { let a1_ = (c.m.r32((fp.wrapping_add(16)).wrapping_add(((((l_n_8).wrapping_sub(1i32)) as u32)).wrapping_mul(4))) as i32); F::BtlColl_AddActionBit(c, p_chr_1, a1_) };
                    if ((F::BtlInput_TestAction(c, p_chr_1, l_input_10, 1i32)) != 0) {
                        {
                            l_i_11 = 0i32;
                            'l21: loop {
                                if !(((((l_i_11) < (l_n_8)) as i32)) != 0) { break 'l21; }
                                'c22: {
                                    { let a2_ = (c.m.r32((fp.wrapping_add(16)).wrapping_add((((l_i_11) as u32)).wrapping_mul(4))) as i32); F::BtlAct_SetQueue(c, p_chr_1, ((l_i_11) as u32), a2_) };
                                }
                                { let o_ = l_i_11; let n_ = (o_).wrapping_add(1); l_i_11 = n_; };
                            }
                        }
                        break 'f l_n_8;
                    }
                }
            }
        }
        if ((((((((p_mask_2) & (192i32))) != 0) && ((((((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((19i32) as i64 as u64)])) as i32)) != 0) || ((((p_mask_2) & (128i32))) != 0)) as i32)) != 0)) as i32)) != 0) {
            F::BtlColl_AddActionBit(c, p_chr_1, 180i32);
            if ((F::BtlInput_TestAction(c, p_chr_1, 92i32, 1i32)) != 0) {
                F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), 180i32);
                break 'f 1i32;
            }
        }
        if ((((((((((((p_mask_2) & (768i32))) != 0) && ((((((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((19i32) as i64 as u64)])) as i32)) != 0) || ((((p_mask_2) & (512i32))) != 0)) as i32)) != 0)) as i32)) != 0) && ((((((((F::BtlParam_GetFlags2(c, p_chr_1)) as u64)) as i32)) & (524288i32))) != 0)) as i32)) != 0) {
            F::BtlColl_AddActionBit(c, p_chr_1, 182i32);
            if ((F::BtlInput_TestAction(c, p_chr_1, 94i32, 1i32)) != 0) {
                F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), 182i32);
                break 'f 1i32;
            }
        }
        if ((((((((((((p_mask_2) & (524288i32))) != 0) && ((((((c.m.r32((p_chr_1).wrapping_add(3432)) as i32)) < ((c.m.r32((p_chr_1).wrapping_add(3436)) as i32))) as i32)) != 0)) as i32)) != 0) && ((F::BtlMember_HasKi(c, p_chr_1, 5000i32)) != 0)) as i32)) != 0) {
            F::BtlColl_AddActionBit(c, p_chr_1, 25i32);
            if ((F::BtlInput_TestAction(c, p_chr_1, 42i32, 1i32)) != 0) {
                F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), 25i32);
                break 'f 1i32;
            }
        }
        if ((((p_mask_2) & (1048576i32))) != 0) {
            l_ret_12 = F::BtlDecide_QueueAttack(c, p_chr_1, 155i32);
            if (((((l_ret_12) > (0i32)) as i32)) != 0) {
                break 'f l_ret_12;
            }
            if ((((((c.m.r32((p_chr_1).wrapping_add(3440)) as i32)) < ((c.m.r32((p_chr_1).wrapping_add(3444)) as i32))) as i32)) != 0) {
                if ((((((((F::BtlParam_GetFlags2(c, p_chr_1)) as u64)) as i32)) & (8192i32))) != 0) {
                    F::BtlColl_AddActionBit(c, p_chr_1, 92i32);
                    if ((F::BtlInput_TestAction(c, p_chr_1, 68i32, 1i32)) != 0) {
                        F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), 92i32);
                        break 'f 1i32;
                    }
                    if ((F::BtlInput_TestAction(c, p_chr_1, 69i32, 1i32)) != 0) {
                        F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), 93i32);
                        break 'f 1i32;
                    }
                    if ((F::BtlInput_TestAction(c, p_chr_1, 70i32, 1i32)) != 0) {
                        F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), 90i32);
                        break 'f 1i32;
                    }
                    if ((F::BtlInput_TestAction(c, p_chr_1, 71i32, 1i32)) != 0) {
                        F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), 91i32);
                        break 'f 1i32;
                    }
                }
            } else {
                if ((((((((((((F::BtlParam_GetFlags3(c, p_chr_1)) as u64)) as i32)) & (4096i32))) != 0) && ((((!((F::BtlChar_TestMemberUnk70(c, p_chr_1)) != 0)) as i32)) != 0)) as i32)) != 0) {
                    l_ret_12 = F::BtlAct_QueueComboFinish(c, p_chr_1);
                    if (((((l_ret_12) > (0i32)) as i32)) != 0) {
                        break 'f l_ret_12;
                    }
                }
            }
        }
        if ((((((((p_mask_2) & (1024i32))) != 0) && ((F::BtlInput_TestAction(c, p_chr_1, 95i32, 1i32)) != 0)) as i32)) != 0) {
            F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), 103i32);
            break 'f 1i32;
        }
        if ((((((((p_mask_2) & (4096i32))) != 0) && ((F::BtlInput_TestAction(c, p_chr_1, 96i32, 1i32)) != 0)) as i32)) != 0) {
            F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), 104i32);
            break 'f 1i32;
        }
        if ((((((((p_mask_2) & (2048i32))) != 0) && ((F::BtlInput_TestAction(c, p_chr_1, 96i32, 1i32)) != 0)) as i32)) != 0) {
            if (((((F::BtlParam_GetUnk88(c, p_chr_1, ((((1i32) as i64 as u64)) as u32))) == (5i32)) as i32)) != 0) {
                F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), 64i32);
                break 'f 1i32;
            }
            F::BtlAct_SetQueue(c, p_chr_1, ((0i32) as u32), 105i32);
            break 'f 1i32;
        }
        break 'f 0i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlDecide_Attack", &PROBE_N, &[], true); }
    r_
}

pub fn BtlDecide_QueueAttack(c: &mut Ctx, mut p_chr_1: u32, mut p_id_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlDecide_QueueAttack", &PROBE_N, &[(p_chr_1) as u32, (p_id_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_queued_3: i32 = 0;
    let mut l_rec_4: u32 = 0;
    let mut l_idx_5: i32 = 0;
    let mut l_n_6: i32 = 0;
    let mut l_ok_7: i32 = 0;
    let mut l_i_8: i32 = 0;
    let mut l_input_9: i32 = 0;
    let r_ = 'f: {
        l_queued_3 = 0i32;
        if ((((!((F::BtlAct_IsAttackId(c, p_id_2)) != 0)) as i32)) != 0) {
            break 'f 0i32;
        }
        l_idx_5 = (p_id_2).wrapping_sub(112i32);
        l_rec_4 = ({ let a_ = (c.m.r32(sym::gBtlChars)).wrapping_add(44); c.m.r32(a_) }).wrapping_add((((l_idx_5) as u32)).wrapping_mul(24));
        if ((((!((F::BtlAct_TestAttackSkill(c, p_chr_1, l_idx_5)) != 0)) as i32)) != 0) {
            break 'f 0i32;
        }
        l_n_6 = 0i32;
        l_ok_7 = 0i32;
        's1: {
            let sv10: i32 = (((c.m.r8((l_rec_4).wrapping_add(5)) as i8)) as i32);
            match sv10 {
                0i32 => {
                    l_ok_7 = 1i32;
                    break 's1;
                }
                1i32 => {
                    { let v_ = 212i32; let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    l_n_6 = 1i32;
                    break 's1;
                }
                2i32 => {
                    { let v_ = 213i32; let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    l_n_6 = 1i32;
                    break 's1;
                }
                3i32 => {
                    { let v_ = 208i32; let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    l_n_6 = 1i32;
                    break 's1;
                }
                4i32 => {
                    { let v_ = 205i32; let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    l_n_6 = 1i32;
                    break 's1;
                }
                5i32 => {
                    { let v_ = 203i32; let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    l_n_6 = 1i32;
                    break 's1;
                }
                6i32 => {
                    { let v_ = 198i32; let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    l_n_6 = 1i32;
                    break 's1;
                }
                7i32 => {
                    { let v_ = 199i32; let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 200i32; let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    l_n_6 = 2i32;
                    break 's1;
                }
                8i32 => {
                    { let v_ = 206i32; let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    l_n_6 = 1i32;
                    break 's1;
                }
                _ => {
                }
            }
        }
        if (((((((((l_n_6) > (0i32)) as i32)) != 0) && ((F::BtlChar_IsFlagRaised(c, p_chr_1, ((((91i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) {
            break 'f 0i32;
        }
        {
            l_i_8 = 0i32;
            'l2: loop {
                if !(((((l_i_8) < (l_n_6)) as i32)) != 0) { break 'l2; }
                'c3: {
                    if (((((((D::call(c, fid::BtlOpp_GetSeenAction, &[((p_chr_1) as u64)])) as i32)) == ((c.m.r32((fp).wrapping_add((((l_i_8) as u32)).wrapping_mul(4))) as i32))) as i32)) != 0) {
                        l_ok_7 = 1i32;
                        break 'l2;
                    }
                }
                { let o_ = l_i_8; let n_ = (o_).wrapping_add(1); l_i_8 = n_; };
            }
        }
        if ((((!((l_ok_7) != 0)) as i32)) != 0) {
            break 'f 0i32;
        }
        F::BtlColl_AddActionBit(c, p_chr_1, p_id_2);
        l_input_9 = (1i32).wrapping_neg();
        's4: {
            let sv11: i32 = (((c.m.r8((l_rec_4).wrapping_add(2)) as i8)) as i32);
            match sv11 {
                1i32 => {
                    l_input_9 = 79i32;
                    break 's4;
                }
                2i32 => {
                    l_input_9 = 80i32;
                    break 's4;
                }
                3i32 => {
                    l_input_9 = 81i32;
                    break 's4;
                }
                4i32 => {
                    l_input_9 = 82i32;
                    break 's4;
                }
                5i32 => {
                    l_input_9 = 83i32;
                    break 's4;
                }
                6i32 => {
                    l_input_9 = 84i32;
                    break 's4;
                }
                7i32 => {
                    l_input_9 = 85i32;
                    break 's4;
                }
                8i32 => {
                    l_input_9 = 86i32;
                    break 's4;
                }
                9i32 => {
                    l_input_9 = 87i32;
                    break 's4;
                }
                _ => {
                }
            }
        }
        if (((((((((l_input_9) >= (0i32)) as i32)) != 0) && ((((!((F::BtlInput_TestAction(c, p_chr_1, l_input_9, 1i32)) != 0)) as i32)) != 0)) as i32)) != 0) {
            break 'f 0i32;
        }
        's5: {
            let sv12: i32 = (((c.m.r8((l_rec_4).wrapping_add(1)) as i8)) as i32);
            match sv12 {
                0i32 => {
                    break 's5;
                }
                1i32 => {
                    F::BtlAct_SetQueue(c, p_chr_1, (({ let o_ = l_queued_3; let n_ = (o_).wrapping_add(1); l_queued_3 = n_; o_ }) as u32), 43i32);
                    break 's5;
                }
                2i32 => {
                    F::BtlAct_SetQueue(c, p_chr_1, (({ let o_ = l_queued_3; let n_ = (o_).wrapping_add(1); l_queued_3 = n_; o_ }) as u32), 43i32);
                    F::BtlAct_SetQueue(c, p_chr_1, (({ let o_ = l_queued_3; let n_ = (o_).wrapping_add(1); l_queued_3 = n_; o_ }) as u32), 45i32);
                    break 's5;
                }
                3i32 => {
                    F::BtlAct_SetQueue(c, p_chr_1, (({ let o_ = l_queued_3; let n_ = (o_).wrapping_add(1); l_queued_3 = n_; o_ }) as u32), 43i32);
                    F::BtlAct_SetQueue(c, p_chr_1, (({ let o_ = l_queued_3; let n_ = (o_).wrapping_add(1); l_queued_3 = n_; o_ }) as u32), 46i32);
                    break 's5;
                }
                4i32 => {
                    { let a2_ = (if ((F::BtlChar_FrameMod(c, 2i32)) != 0) { 33i32 } else { 34i32 }); F::BtlAct_SetQueue(c, p_chr_1, (({ let o_ = l_queued_3; let n_ = (o_).wrapping_add(1); l_queued_3 = n_; o_ }) as u32), a2_) };
                    break 's5;
                }
                5i32 => {
                    F::BtlAct_SetQueue(c, p_chr_1, (({ let o_ = l_queued_3; let n_ = (o_).wrapping_add(1); l_queued_3 = n_; o_ }) as u32), 43i32);
                    F::BtlAct_SetQueue(c, p_chr_1, (({ let o_ = l_queued_3; let n_ = (o_).wrapping_add(1); l_queued_3 = n_; o_ }) as u32), 47i32);
                    break 's5;
                }
                6i32 => {
                    F::BtlAct_SetQueue(c, p_chr_1, (({ let o_ = l_queued_3; let n_ = (o_).wrapping_add(1); l_queued_3 = n_; o_ }) as u32), 49i32);
                    break 's5;
                }
                7i32 => {
                    F::BtlAct_SetQueue(c, p_chr_1, (({ let o_ = l_queued_3; let n_ = (o_).wrapping_add(1); l_queued_3 = n_; o_ }) as u32), 50i32);
                    break 's5;
                }
                8i32 => {
                    F::BtlAct_SetQueue(c, p_chr_1, (({ let o_ = l_queued_3; let n_ = (o_).wrapping_add(1); l_queued_3 = n_; o_ }) as u32), 43i32);
                    F::BtlAct_SetQueue(c, p_chr_1, (({ let o_ = l_queued_3; let n_ = (o_).wrapping_add(1); l_queued_3 = n_; o_ }) as u32), 48i32);
                    break 's5;
                }
                9i32 => {
                    F::BtlAct_SetQueue(c, p_chr_1, (({ let o_ = l_queued_3; let n_ = (o_).wrapping_add(1); l_queued_3 = n_; o_ }) as u32), 44i32);
                    F::BtlAct_SetQueue(c, p_chr_1, (({ let o_ = l_queued_3; let n_ = (o_).wrapping_add(1); l_queued_3 = n_; o_ }) as u32), 45i32);
                    break 's5;
                }
                _ => {
                }
            }
        }
        F::BtlAct_SetQueue(c, p_chr_1, (({ let o_ = l_queued_3; let n_ = (o_).wrapping_add(1); l_queued_3 = n_; o_ }) as u32), p_id_2);
        F::BtlAct_PrepareAttack(c, p_chr_1, p_id_2);
        break 'f l_queued_3;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlDecide_QueueAttack", &PROBE_N, &[], true); }
    r_
}
