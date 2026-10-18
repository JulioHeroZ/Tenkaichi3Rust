#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn BtlSuper_RequestCut(c: &mut Ctx, mut p_chr_1: u32, mut p_cls_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_RequestCut", &PROBE_N, &[(p_chr_1) as u32, (p_cls_2) as u32], false); }
    let mut l_cut_3: i32 = 0;
    let r_ = 'f: {
    l_cut_3 = 0i32;
    's1: {
        let sv4: i32 = p_cls_2;
        match sv4 {
            2i32 => {
                l_cut_3 = 0i32;
                break 's1;
            }
            3i32 => {
                l_cut_3 = 1i32;
                break 's1;
            }
            4i32 => {
                l_cut_3 = 2i32;
                break 's1;
            }
            _ => {
            }
        }
    }
    F::ChrCam_RequestCut(c, p_chr_1, 0i32, l_cut_3);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_RequestCut", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSuper_SetStartFx(c: &mut Ctx, mut p_chr_1: u32, mut p_cls_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_SetStartFx", &PROBE_N, &[(p_chr_1) as u32, (p_cls_2) as u32], false); }
    let mut l_bit_3: i32 = 0;
    let r_ = 'f: {
    l_bit_3 = 0i32;
    's1: {
        let sv4: i32 = p_cls_2;
        match sv4 {
            2i32 => {
                l_bit_3 = 62i32;
                break 's1;
            }
            3i32 => {
                l_bit_3 = 63i32;
                break 's1;
            }
            4i32 => {
                l_bit_3 = 64i32;
                break 's1;
            }
            _ => {
            }
        }
    }
    F::BtlChar_SetFxBit(c, p_chr_1, l_bit_3);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_SetStartFx", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSuper_SetClassFlag(c: &mut Ctx, mut p_chr_1: u32, mut p_cls_2: i32, mut p_force_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_SetClassFlag", &PROBE_N, &[(p_chr_1) as u32, (p_cls_2) as u32, (p_force_3) as u32], false); }
    let mut l_flag_4: i32 = 0;
    let r_ = 'f: {
    l_flag_4 = 0i32;
    if (((((((((p_force_3) == (0i32)) as i32)) != 0) && (((((p_cls_2) == (4i32)) as i32)) != 0)) as i32)) != 0) {
        break 'f;
    }
    's1: {
        let sv5: i32 = p_cls_2;
        match sv5 {
            2i32 => {
                l_flag_4 = 224i32;
                break 's1;
            }
            3i32 => {
                l_flag_4 = 225i32;
                break 's1;
            }
            4i32 => {
                l_flag_4 = 226i32;
                break 's1;
            }
            _ => {
            }
        }
    }
    F::BtlChar_SetFlag(c, p_chr_1, ((((l_flag_4) as i64 as u64)) as u32));
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_SetClassFlag", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSuper_RequestHitStop(c: &mut Ctx, mut p_chr_1: u32, mut p_light_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_RequestHitStop", &PROBE_N, &[(p_chr_1) as u32, (p_light_2) as u32], false); }
    let r_ = 'f: {
    if ((p_light_2) != 0) {
        F::BtlChar_SetFlag(c, p_chr_1, ((((68i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((69i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((294i32) as i64 as u64)) as u32));
    } else {
        F::BtlChar_SetFlag(c, p_chr_1, ((((66i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((67i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((68i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((69i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((70i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((293i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((298i32) as i64 as u64)) as u32));
    }
    F::BtlChar_SetFlag(c, p_chr_1, ((((296i32) as i64 as u64)) as u32));
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_RequestHitStop", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSuper_SetFiringFlags(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_SetFiringFlags", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    F::BtlChar_SetFlag(c, p_chr_1, ((((213i32) as i64 as u64)) as u32));
    F::BtlChar_SetFlag(c, p_chr_1, ((((187i32) as i64 as u64)) as u32));
    F::BtlChar_SetFlag(c, p_chr_1, ((((295i32) as i64 as u64)) as u32));
    if ((((((((!((F::BtlChar_TestFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) && ((((!((F::BtlChar_TestFlag(c, p_chr_1, ((((15i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0)) as i32)) != 0) {
        F::BtlChar_SetHeldFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32));
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_SetFiringFlags", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSuper_SetClass(c: &mut Ctx, mut p_chr_1: u32, mut p_cls_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_SetClass", &PROBE_N, &[(p_chr_1) as u32, (p_cls_2) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_cls_2; let a_ = (p_chr_1).wrapping_add(5524); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_SetClass", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSuper_SetRushFlags(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_SetRushFlags", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    F::BtlChar_SetFlag(c, p_chr_1, ((((213i32) as i64 as u64)) as u32));
    F::BtlChar_SetFlag(c, p_chr_1, ((((26i32) as i64 as u64)) as u32));
    F::BtlChar_SetFlag(c, p_chr_1, ((((63i32) as i64 as u64)) as u32));
    F::BtlChar_SetFlag(c, p_chr_1, ((((187i32) as i64 as u64)) as u32));
    F::BtlChar_SetFlag(c, p_chr_1, ((((296i32) as i64 as u64)) as u32));
    F::BtlChar_SetFlag(c, p_chr_1, ((((24i32) as i64 as u64)) as u32));
    F::BtlChar_SetFlag(c, p_chr_1, ((((301i32) as i64 as u64)) as u32));
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_SetRushFlags", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSuper_ShakeOnEvent(c: &mut Ctx, mut p_chr_1: u32, mut p_cls_2: i32, mut p_kind_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_ShakeOnEvent", &PROBE_N, &[(p_chr_1) as u32, (p_cls_2) as u32, (p_kind_3) as u32], false); }
    let mut l_obj_4: u32 = 0;
    let mut l_flags_5: u32 = 0;
    let mut l_n_6: i32 = 0;
    let mut l_frame_7: i32 = 0;
    let r_ = 'f: {
    l_obj_4 = F::BtlChar_GetObj(c, p_chr_1);
    l_flags_5 = ((((((F::BtlSuper_GetFlagsA(c, p_chr_1, p_cls_2)) as u64)) as i32)) as u32);
    's1: {
        let sv8: i32 = p_kind_3;
        match sv8 {
            0i32 => {
                if ((((l_flags_5) & (((67108864i32) as u32)))) != 0) {
                    l_n_6 = F::BtlObjAnim_QueryEvent(c, l_obj_4, ((1024i32) as u64), 0i32, ((((3i32) as i64 as u64)) as u32));
                    if (((((l_n_6) == (1i32)) as i32)) != 0) {
                        l_frame_7 = (F::BtlObjAnim_QueryEvent(c, l_obj_4, ((1024i32) as u64), 0i32, ((((0i32) as i64 as u64)) as u32))).wrapping_sub(4i32);
                        if (((((l_frame_7) < (0i32)) as i32)) != 0) {
                            l_frame_7 = 0i32;
                        }
                        if ((F::BtlAnim_PassedFrame(c, p_chr_1, rt::i2f(l_frame_7))) != 0) {
                            if ((((l_flags_5) & (((268435456i32) as u32)))) != 0) {
                                { let a0_ = F::BtlChar_GetPos(c, p_chr_1); F::BtlCharApi_ShakeCamsNear(c, a0_, F32(0x42c80000), F32(0x447a0000), F32(0x41200000), F32(0x3e4ccccc)) };
                                { let a0_ = F::BtlChar_GetPos(c, p_chr_1); F::BtlCharApi_RumbleNear(c, a0_, F32(0x42c80000), F32(0x447a0000), F32(0x3f800000), F32(0x3e999999)) };
                            } else {
                                F::ChrCam_AddShake(c, p_chr_1, F32(0x41200000), F32(0x3e4ccccc));
                                F::BtlChar_Vibrate(c, p_chr_1, F32(0x3f800000), F32(0x3e999999));
                            }
                        }
                    } else {
                        if (((((l_n_6) >= (2i32)) as i32)) != 0) {
                            if ((F::BtlAnim_TestAttr(c, p_chr_1, ((1024i32) as u64))) != 0) {
                                if ((((l_flags_5) & (((268435456i32) as u32)))) != 0) {
                                    { let a0_ = F::BtlChar_GetPos(c, p_chr_1); F::BtlCharApi_ShakeCamsNear(c, a0_, F32(0x42c80000), F32(0x447a0000), F32(0x41200000), F32(0x3dcccccc)) };
                                    { let a0_ = F::BtlChar_GetPos(c, p_chr_1); F::BtlCharApi_RumbleNear(c, a0_, F32(0x42c80000), F32(0x447a0000), F32(0x3f800000), F32(0x3e4ccccc)) };
                                } else {
                                    F::ChrCam_AddShake(c, p_chr_1, F32(0x41200000), F32(0x3dcccccc));
                                    F::BtlChar_Vibrate(c, p_chr_1, F32(0x3f800000), F32(0x3e4ccccc));
                                }
                            }
                        }
                    }
                }
                break 's1;
            }
            1i32 => {
                if ((((l_flags_5) & (((134217728i32) as u32)))) != 0) {
                    if ((((l_flags_5) & (((268435456i32) as u32)))) != 0) {
                        { let a0_ = F::BtlChar_GetPos(c, p_chr_1); F::BtlCharApi_ShakeCamsNear(c, a0_, F32(0x42c80000), F32(0x447a0000), F32(0x40400000), F32(0x3dcccccc)) };
                        { let a0_ = F::BtlChar_GetPos(c, p_chr_1); F::BtlCharApi_RumbleNear(c, a0_, F32(0x42c80000), F32(0x447a0000), F32(0x3f800000), F32(0x3dcccccc)) };
                    } else {
                        F::ChrCam_AddShake(c, p_chr_1, F32(0x40400000), F32(0x3dcccccc));
                        F::BtlChar_Vibrate(c, p_chr_1, F32(0x3f800000), F32(0x3dcccccc));
                    }
                }
                break 's1;
            }
            _ => {
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_ShakeOnEvent", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSuper_FitAnimToEvent(c: &mut Ctx, mut p_chr_1: u32, mut p_seconds_2: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_FitAnimToEvent", &PROBE_N, &[(p_chr_1) as u32, (p_seconds_2).0], false); }
    let mut l_event_3: F32 = F32(0);
    let mut l_frame_4: F32 = F32(0);
    let mut l_step_5: F32 = F32(0);
    let mut l_rest_6: F32 = F32(0);
    let r_ = 'f: {
    l_event_3 = rt::i2f({ let a0_ = F::BtlChar_GetObj(c, p_chr_1); F::BtlObjAnim_QueryEvent(c, a0_, ((1024i32) as u64), 0i32, ((((0i32) as i64 as u64)) as u32)) });
    if (((rt::flt(l_event_3, F32(0x00000000)) as i32)) != 0) {
        F::BtlAnim_SetDuration(c, p_chr_1, F32(0x3dcccccc));
        break 'f;
    }
    l_frame_4 = F::BtlAnim_GetFrame(c, p_chr_1);
    if (((rt::flt(rt::fsub(l_event_3, F32(0x40000000)), l_frame_4) as i32)) != 0) {
        break 'f;
    }
    if (((rt::flt(p_seconds_2, F32(0x3d088888)) as i32)) != 0) {
        l_step_5 = rt::fsub(l_event_3, l_frame_4);
    } else {
        l_rest_6 = rt::fsub(l_event_3, l_frame_4);
        l_step_5 = rt::fdiv(l_event_3, rt::fmul(p_seconds_2, F32(0x41f00000)));
        if (((rt::flt(l_rest_6, l_step_5) as i32)) != 0) {
            l_step_5 = l_rest_6;
        }
    }
    F::BtlAnim_SetStep(c, p_chr_1, l_step_5);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_FitAnimToEvent", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSuper_SetObjUnk(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_SetObjUnk", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    F::BtlAct_SetObjUnk(c, p_chr_1, F32(0x3e4ccccc), F32(0x40000000));
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_SetObjUnk", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSuper_VibrateCharge(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_VibrateCharge", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    F::BtlChar_SetVibration(c, p_chr_1, F32(0x3f4ccccc), F32(0x3dcccccc));
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_VibrateCharge", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSuper_Recoil(c: &mut Ctx, mut p_chr_1: u32, mut p_cls_2: i32, mut p_loop_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_Recoil", &PROBE_N, &[(p_chr_1) as u32, (p_cls_2) as u32, (p_loop_3) as u32], false); }
    let mut l_push_4: i32 = 0;
    let r_ = 'f: {
    l_push_4 = 0i32;
    's1: {
        let sv5: i32 = F::BtlSuper_GetHitDirKind(c, p_chr_1, p_cls_2);
        match sv5 {
            0i32 | 9i32 => {
                l_push_4 = 1i32;
                break 's1;
            }
            1i32 | 2i32 | 3i32 | 4i32 | 6i32 | 8i32 => {
                l_push_4 = (((p_loop_3) == (0i32)) as i32);
                break 's1;
            }
            5i32 | 7i32 => {
                break 's1;
            }
            _ => {
            }
        }
    }
    if ((l_push_4) != 0) {
        { let a1_ = { let a0_ = rt::fadd({ let a_ = ((F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(16)).wrapping_add(4); F32(c.m.r32(a_)) }, F32(0x40490fda)); F::BtlUtil_WrapAngle(c, a0_) }; F::BtlAct_PushAngle(c, p_chr_1, a1_, F32(0x3f800000), F32(0x40a00000)) };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_Recoil", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSuper_MeasureRushStep(c: &mut Ctx, mut p_chr_1: u32, mut p_cls_2: i32, mut p_step_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_MeasureRushStep", &PROBE_N, &[(p_chr_1) as u32, (p_cls_2) as u32, (p_step_3) as u32], false); }
    let mut l_count_4: i32 = 0;
    let mut l_obj_5: u32 = 0;
    let r_ = 'f: {
    l_count_4 = F::BtlSuper_GetStepCount(c, p_chr_1, p_cls_2);
    if (((((p_step_3) < (l_count_4)) as i32)) != 0) {
        l_obj_5 = F::BtlChar_GetObj(c, p_chr_1);
        if (((((p_step_3) == (0i32)) as i32)) != 0) {
            { let v_ = 0i32; let a_ = (p_chr_1).wrapping_add(3832); c.m.w32(a_, (v_) as u32); };
            { let v_ = 0i32; let a_ = (p_chr_1).wrapping_add(3860); c.m.w32(a_, (v_) as u32); };
            { let v_ = 0i32; let a_ = (p_chr_1).wrapping_add(3864); c.m.w32(a_, (v_) as u32); };
            { let v_ = (1i32).wrapping_neg(); let a_ = (p_chr_1).wrapping_add(3868); c.m.w32(a_, (v_) as u32); };
            { let v_ = (1i32).wrapping_neg(); let a_ = (p_chr_1).wrapping_add(3872); c.m.w32(a_, (v_) as u32); };
            { let v_ = 0i32; let a_ = (p_chr_1).wrapping_add(3856); c.m.w32(a_, (v_) as u32); };
        }
        F::BtlAnim_PlaySub(c, p_chr_1, ((p_step_3).wrapping_add((p_cls_2).wrapping_mul(34i32))).wrapping_add(216i32));
        if (((((c.m.r32((l_obj_5).wrapping_add(3032))) != (((0i32) as u32))) as i32)) != 0) {
            { let r_ = F::BtlObjAnim_QueryEvent(c, l_obj_5, ((8589934592i64) as u64), 1i32, ((((3i32) as i64 as u64)) as u32)); let a_ = (p_chr_1).wrapping_add(3832); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
            if ((((((c.m.r32((p_chr_1).wrapping_add(3868)) as i32)) < (0i32)) as i32)) != 0) {
                if (((((F::BtlObjAnim_QueryEvent(c, l_obj_5, ((131072i32) as u64), 1i32, ((((3i32) as i64 as u64)) as u32))) > (0i32)) as i32)) != 0) {
                    { let r_ = F::BtlObjAnim_QueryEvent(c, l_obj_5, ((131072i32) as u64), 1i32, ((((0i32) as i64 as u64)) as u32)); let a_ = (p_chr_1).wrapping_add(3860); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                    { let v_ = p_step_3; let a_ = (p_chr_1).wrapping_add(3868); c.m.w32(a_, (v_) as u32); };
                } else {
                    { let v_ = rt::f2i(rt::fadd(rt::i2f((c.m.r32((p_chr_1).wrapping_add(3860)) as i32)), rt::fadd(rt::i2f((({ let a_ = (c.m.r32((l_obj_5).wrapping_add(3032))).wrapping_add(2); c.m.r16(a_) }) as i32)), F32(0x3f800000)))); let a_ = (p_chr_1).wrapping_add(3860); c.m.w32(a_, (v_) as u32); };
                }
            }
            if ((((((c.m.r32((p_chr_1).wrapping_add(3872)) as i32)) < (0i32)) as i32)) != 0) {
                if (((((F::BtlObjAnim_QueryEvent(c, l_obj_5, ((262144i32) as u64), 1i32, ((((3i32) as i64 as u64)) as u32))) > (0i32)) as i32)) != 0) {
                    { let r_ = F::BtlObjAnim_QueryEvent(c, l_obj_5, ((262144i32) as u64), 1i32, ((((2i32) as i64 as u64)) as u32)); let a_ = (p_chr_1).wrapping_add(3864); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                    { let v_ = p_step_3; let a_ = (p_chr_1).wrapping_add(3872); c.m.w32(a_, (v_) as u32); };
                } else {
                    { let v_ = rt::f2i(rt::fadd(rt::i2f((c.m.r32((p_chr_1).wrapping_add(3864)) as i32)), rt::fadd(rt::i2f((({ let a_ = (c.m.r32((l_obj_5).wrapping_add(3032))).wrapping_add(2); c.m.r16(a_) }) as i32)), F32(0x3f800000)))); let a_ = (p_chr_1).wrapping_add(3864); c.m.w32(a_, (v_) as u32); };
                }
            }
            { let r_ = (({ let a_ = (c.m.r32((l_obj_5).wrapping_add(3032))).wrapping_add(2); c.m.r16(a_) }) as i32); let a_ = (p_chr_1).wrapping_add(3856); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
            if (((((p_step_3) < ((l_count_4).wrapping_sub(1i32))) as i32)) != 0) {
                { let v_ = rt::f2i(rt::fadd(rt::i2f((c.m.r32((p_chr_1).wrapping_add(3856)) as i32)), F32(0x3f800000))); let a_ = (p_chr_1).wrapping_add(3856); c.m.w32(a_, (v_) as u32); };
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_MeasureRushStep", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSuper_Begin(c: &mut Ctx, mut p_chr_1: u32, mut p_cls_2: i32, mut p_force_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_Begin", &PROBE_N, &[(p_chr_1) as u32, (p_cls_2) as u32, (p_force_3) as u32], false); }
    let r_ = 'f: {
    if (((((((((p_force_3) == (0i32)) as i32)) != 0) && (((((p_cls_2) == (4i32)) as i32)) != 0)) as i32)) != 0) {
        break 'f;
    }
    { let v_ = F32(0x00000000); let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(152); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x00000000); let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(156); c.m.w32(a_, (v_).0); };
    F::BtlChar_ClearFlagRange(c, p_chr_1, ((155i32) as u32), ((160i32) as u32));
    F::BtlChar_ClearFlagRange(c, p_chr_1, ((167i32) as u32), ((170i32) as u32));
    F::BtlChar_SetFlag(c, p_chr_1, ((((41i32) as i64 as u64)) as u32));
    { let v_ = F32(0x00000000); let a_ = (p_chr_1).wrapping_add(3652); c.m.w32(a_, (v_).0); };
    { let v_ = 0i32; let a_ = (p_chr_1).wrapping_add(3656); c.m.w32(a_, (v_) as u32); };
    { let a_ = ((F::BtlMember_GetActiveGauge(c, p_chr_1)).wrapping_add(56)).wrapping_add((((p_cls_2) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
    if ((((!((F::BtlChar_TestFlag(c, p_chr_1, ((((15i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) {
        F::BtlChar_SetHeldFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32));
    }
    if ((((((((((((F::BtlSuper_GetFlags(c, p_chr_1, p_cls_2)) as u64)) as i32)) & (32768i32))) != 0) && ((((!((F::BtlChar_TestFlag(c, p_chr_1, ((((152i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0)) as i32)) != 0) {
        { let a0_ = F::BtlChar_GetObj(c, p_chr_1); F::BtlObj_SetColorMode(c, a0_, 2i32, 1i32) };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_Begin", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSuper_PlayStartSound(c: &mut Ctx, mut p_chr_1: u32, mut p_cls_2: i32, mut p_force_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_PlayStartSound", &PROBE_N, &[(p_chr_1) as u32, (p_cls_2) as u32, (p_force_3) as u32], false); }
    let r_ = 'f: {
    if (((((((((p_force_3) == (0i32)) as i32)) != 0) && (((((p_cls_2) == (4i32)) as i32)) != 0)) as i32)) != 0) {
        break 'f;
    }
    F::BtlCharSnd_PlayCommon(c, p_chr_1, 43i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_PlayStartSound", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSuper_Leave(c: &mut Ctx, mut p_chr_1: u32, mut p_cls_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_Leave", &PROBE_N, &[(p_chr_1) as u32, (p_cls_2) as u32], false); }
    let mut l_obj_3: u32 = 0;
    let r_ = 'f: {
    if (({ let a0_ = F::BtlAct_GetRequested(c, p_chr_1); F::BtlAct_IsTechniqueId(c, a0_) }) != 0) {
        break 'f;
    }
    { let v_ = F::BtlSuper_GetCooldownFrames(c, p_chr_1, p_cls_2); let a_ = (p_chr_1).wrapping_add(3648); c.m.w32(a_, (v_) as u32); };
    F::BtlChar_ClearFlagRange(c, p_chr_1, ((167i32) as u32), ((170i32) as u32));
    F::BtlChar_ClearFlag(c, p_chr_1, ((((287i32) as i64 as u64)) as u32));
    if ((((((((((((F::BtlSuper_GetFlags(c, p_chr_1, p_cls_2)) as u64)) as i32)) & (32768i32))) != 0) && ((((!((F::BtlChar_TestFlag(c, p_chr_1, ((((152i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0)) as i32)) != 0) {
        l_obj_3 = F::BtlChar_GetObj(c, p_chr_1);
        if ((((c.m.r32((l_obj_3).wrapping_add(2624))) & (((262144i32) as u32)))) != 0) {
            F::BtlObj_SetColorMode(c, l_obj_3, 2i32, 0i32);
        }
    }
    's1: {
        let sv4: i32 = F::BtlSuper_GetId(c, p_chr_1, p_cls_2);
        match sv4 {
            616i32 => {
                { let v_ = 0i32; let a_ = (p_chr_1).wrapping_add(3676); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            717i32 => {
                { let v_ = 0i32; let a_ = (p_chr_1).wrapping_add(3680); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            _ => {
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_Leave", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSuper_SetupRushDamage(c: &mut Ctx, mut p_chr_1: u32, mut p_cls_2: i32, mut p_fromAnim_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_SetupRushDamage", &PROBE_N, &[(p_chr_1) as u32, (p_cls_2) as u32, (p_fromAnim_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_damage_4: i32 = 0;
    let mut l_drain_5: i32 = 0;
    let mut l_from_6: i32 = 0;
    let mut l_to_7: i32 = 0;
    let mut l_tmp_8: i32 = 0;
    let mut l_rem_9: i32 = 0;
    let mut l_immune_10: i32 = 0;
    let mut l_swap_11: i32 = 0;
    let r_ = 'f: {
        F::BtlChar_SetFlag(c, p_chr_1, ((((161i32) as i64 as u64)) as u32));
        { let v_ = 0i32; let a_ = (p_chr_1).wrapping_add(3488); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (p_chr_1).wrapping_add(3476); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (p_chr_1).wrapping_add(3480); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (p_chr_1).wrapping_add(3484); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (p_chr_1).wrapping_add(3492); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (p_chr_1).wrapping_add(3496); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (p_chr_1).wrapping_add(3500); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (p_chr_1).wrapping_add(3504); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (p_chr_1).wrapping_add(3508); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (p_chr_1).wrapping_add(3512); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (p_chr_1).wrapping_add(3516); c.m.w32(a_, (v_) as u32); };
        { let v_ = (1i32).wrapping_neg(); let a_ = (p_chr_1).wrapping_add(3520); c.m.w32(a_, (v_) as u32); };
        { let v_ = (1i32).wrapping_neg(); let a_ = (p_chr_1).wrapping_add(3524); c.m.w32(a_, (v_) as u32); };
        if ((((!((((((((F::BtlSuper_GetFlags(c, p_chr_1, p_cls_2)) as u64)) as i32)) & (16777216i32))) != 0)) as i32)) != 0) {
            l_damage_4 = F::BtlSuper_GetDamage(c, p_chr_1, p_cls_2, 0i32, 1i32);
            l_rem_9 = rt::rem_i32(l_damage_4, 10i32);
            if ((l_rem_9) != 0) {
                l_damage_4 = ((l_damage_4).wrapping_sub(l_rem_9)).wrapping_add(10i32);
            }
            if ((F::BtlChar_TestFlag(c, p_chr_1, ((((160i32) as i64 as u64)) as u32))) != 0) {
                { let r_ = rt::div_i32(l_damage_4, 2i32); let n_ = (l_damage_4).wrapping_add(r_); l_damage_4 = n_; };
            }
            if (((c.m.r32((p_chr_1).wrapping_add(3816)) as i32)) != 0) {
                { let r_ = 2i32; let n_ = rt::div_i32(l_damage_4, r_); l_damage_4 = n_; };
            }
            if ((p_fromAnim_3) != 0) {
                { let v_ = { let a0_ = F::BtlChar_GetObj(c, p_chr_1); F::BtlObjAnim_QueryEvent(c, a0_, ((8589934592i64) as u64), 0i32, ((((3i32) as i64 as u64)) as u32)) }; let a_ = (p_chr_1).wrapping_add(3488); c.m.w32(a_, (v_) as u32); };
            } else {
                { let v_ = (c.m.r32((p_chr_1).wrapping_add(3832)) as i32); let a_ = (p_chr_1).wrapping_add(3488); c.m.w32(a_, (v_) as u32); };
            }
            { let v_ = l_damage_4; let a_ = (p_chr_1).wrapping_add(3476); c.m.w32(a_, (v_) as u32); };
            { let v_ = l_damage_4; let a_ = (p_chr_1).wrapping_add(3480); c.m.w32(a_, (v_) as u32); };
            { let v_ = rt::div_i32(l_damage_4, ((c.m.r32((p_chr_1).wrapping_add(3488)) as i32)).wrapping_add(5i32)); let a_ = (p_chr_1).wrapping_add(3484); c.m.w32(a_, (v_) as u32); };
            { let v_ = 384i32; let a_ = (p_chr_1).wrapping_add(3492); c.m.w32(a_, (v_) as u32); };
            's1: {
                let sv12: i32 = p_cls_2;
                match sv12 {
                    2i32 => {
                        { let v_ = 8388992i32; let a_ = (p_chr_1).wrapping_add(3492); c.m.w32(a_, (v_) as u32); };
                        break 's1;
                    }
                    3i32 => {
                        { let v_ = 16777600i32; let a_ = (p_chr_1).wrapping_add(3492); c.m.w32(a_, (v_) as u32); };
                        break 's1;
                    }
                    4i32 => {
                        { let v_ = 33554816i32; let a_ = (p_chr_1).wrapping_add(3492); c.m.w32(a_, (v_) as u32); };
                        break 's1;
                    }
                    _ => {
                    }
                }
            }
            { let v_ = { let a0_ = (c.m.r32((p_chr_1).wrapping_add(3484)) as i32); F::BtlUtil_Min(c, a0_, 15000i32) }; let a_ = (p_chr_1).wrapping_add(3484); c.m.w32(a_, (v_) as u32); };
            l_rem_9 = rt::rem_i32((c.m.r32((p_chr_1).wrapping_add(3484)) as i32), 10i32);
            if ((l_rem_9) != 0) {
                { let v_ = (((c.m.r32((p_chr_1).wrapping_add(3484)) as i32)).wrapping_sub(l_rem_9)).wrapping_add(10i32); let a_ = (p_chr_1).wrapping_add(3484); c.m.w32(a_, (v_) as u32); };
            }
            l_immune_10 = ((F::BtlOpp_GetParamWord0(c, p_chr_1)) & (128i32));
            if ((((!((l_immune_10) != 0)) as i32)) != 0) {
                l_drain_5 = F::BtlSuper_GetDrain(c, p_chr_1, p_cls_2);
                if (((((l_drain_5) > (0i32)) as i32)) != 0) {
                    l_from_6 = (c.m.r32((p_chr_1).wrapping_add(3860)) as i32);
                    l_to_7 = (c.m.r32((p_chr_1).wrapping_add(3864)) as i32);
                    l_from_6 = { let a1_ = ((c.m.r32((p_chr_1).wrapping_add(3856)) as i32)).wrapping_sub(30i32); F::BtlUtil_Min(c, l_from_6, a1_) };
                    l_to_7 = { let a1_ = ((c.m.r32((p_chr_1).wrapping_add(3856)) as i32)).wrapping_sub(30i32); F::BtlUtil_Min(c, l_to_7, a1_) };
                    if (((((l_to_7) < (l_from_6)) as i32)) != 0) {
                        l_swap_11 = l_from_6;
                        l_from_6 = l_to_7;
                        l_to_7 = l_swap_11;
                    }
                    { let v_ = l_drain_5; let a_ = (p_chr_1).wrapping_add(3496); c.m.w32(a_, (v_) as u32); };
                    { let v_ = l_drain_5; let a_ = (p_chr_1).wrapping_add(3500); c.m.w32(a_, (v_) as u32); };
                    l_tmp_8 = rt::div_i32((l_to_7).wrapping_sub(l_from_6), 2i32);
                    if (((((l_tmp_8) <= (l_immune_10)) as i32)) != 0) {
                        l_tmp_8 = 1i32;
                    }
                    { let v_ = (rt::div_i32(l_drain_5, l_tmp_8)).wrapping_add(1i32); let a_ = (p_chr_1).wrapping_add(3504); c.m.w32(a_, (v_) as u32); };
                    { let v_ = l_from_6; let a_ = (p_chr_1).wrapping_add(3520); c.m.w32(a_, (v_) as u32); };
                    { let v_ = l_to_7; let a_ = (p_chr_1).wrapping_add(3524); c.m.w32(a_, (v_) as u32); };
                }
            }
            if ((((((((F::BtlSuper_GetFlags(c, p_chr_1, p_cls_2)) as u64)) as i32)) & (1024i32))) != 0) {
                { let v_ = 0i32; let a_ = (p_chr_1).wrapping_add(3484); c.m.w32(a_, (v_) as u32); };
                { let r_ = 512i32; let a_ = (p_chr_1).wrapping_add(3492); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                if (((c.m.r32((p_chr_1).wrapping_add(3780)) as i32)) != 0) {
                    { let v_ = 1i32; let a_ = (p_chr_1).wrapping_add(3476); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 1i32; let a_ = (p_chr_1).wrapping_add(3480); c.m.w32(a_, (v_) as u32); };
                    { let r_ = 3i32; let a_ = (p_chr_1).wrapping_add(3492); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                }
            }
        }
        if ((((((((F::BtlSuper_GetFlags(c, p_chr_1, p_cls_2)) as u64)) as i32)) & (2097152i32))) != 0) {
            F::BtlOpp_GetTargetRot(c, p_chr_1, fp.wrapping_add(16));
            { let a1_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))); F::BtlMove_SetHeading(c, p_chr_1, a1_, F32(0x00000000)) };
            F::BtlChar_SetFlag(c, p_chr_1, ((((155i32) as i64 as u64)) as u32));
            F::BtlOpp_GetObjVecFA0(c, p_chr_1, fp);
            F::BtlChar_SetUnk1310(c, p_chr_1, fp);
        }
        if (((((F::BtlSuper_GetId(c, p_chr_1, p_cls_2)) == (751i32)) as i32)) != 0) {
            if ((((!((((F::BtlOpp_GetParamWord0(c, p_chr_1)) & (128i32))) != 0)) as i32)) != 0) {
                { let v_ = { let a0_ = ((c.m.r32((p_chr_1).wrapping_add(3680)) as i32)).wrapping_add(5i32); F::BtlUtil_Clamp(c, a0_, 0i32, 10i32) }; let a_ = (p_chr_1).wrapping_add(3680); c.m.w32(a_, (v_) as u32); };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_SetupRushDamage", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSuper_Finish(c: &mut Ctx, mut p_chr_1: u32, mut p_cls_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_Finish", &PROBE_N, &[(p_chr_1) as u32, (p_cls_2) as u32], false); }
    let r_ = 'f: {
    if ((((((((F::BtlSuper_GetFlags(c, p_chr_1, p_cls_2)) as u64)) as i32)) & (16384i32))) != 0) {
        F::BtlChar_SetHeldFlag(c, p_chr_1, ((((29i32) as i64 as u64)) as u32));
        F::BtlChar_SetHeldFlag(c, p_chr_1, ((((30i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((53i32) as i64 as u64)) as u32));
        F::BtlAct_Request(c, p_chr_1, 217i32);
    } else {
        F::BtlAct_Request(c, p_chr_1, 11i32);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSuper_Finish", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_SuperBeamHandler(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_SuperBeamHandler", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let mut l_timer_3: u32 = 0;
    let mut l_cls_4: i32 = 0;
    let mut l_flagsB_5: u32 = 0;
    let mut l_obj_6: u32 = 0;
    let mut l_aim_7: i32 = 0;
    let r_ = 'f: {
    l_timer_3 = ((p_chr_1).wrapping_add(976)).wrapping_add((((2i32) as u32)).wrapping_mul(4));
    l_cls_4 = F::BtlAct_GetCurrentClass(c, p_chr_1);
    l_flagsB_5 = ((((((F::BtlSuper_GetFlags(c, p_chr_1, l_cls_4)) as u64)) as i32)) as u32);
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        F::BtlAnim_Play(c, p_chr_1, (((l_cls_4).wrapping_mul(34i32)).wrapping_add(193i32)).wrapping_add(0i32), F32(0x00000000));
        F::BtlSuper_Begin(c, p_chr_1, l_cls_4, 0i32);
        F::BtlSuper_PlayStartSound(c, p_chr_1, l_cls_4, 0i32);
        F::BtlMove_TurnYaw(c, p_chr_1, 2i32, F32(0x40490fda));
        F::BtlMove_TurnPitch(c, p_chr_1, 5i32, F32(0x40490fda));
        F::BtlMove_TurnModelYaw(c, p_chr_1, F32(0x40490fda), F32(0x3f800000));
        F::BtlSuper_SetStartFx(c, p_chr_1, l_cls_4);
        F::BtlSuper_SetClassFlag(c, p_chr_1, l_cls_4, 0i32);
        F::BtlSuper_RequestCut(c, p_chr_1, l_cls_4);
        { let _ = { let a1_ = F::BtlSuper_GetKiCost(c, p_chr_1, l_cls_4); F::BtlMember_SpendKi(c, p_chr_1, a1_, 1i32) }; };
        F::BtlChar_ResetLook(c, p_chr_1);
        if ((((((((l_flagsB_5) & (((8192i32) as u32)))) != 0) && ((((({ let a_ = (F::BtlMember_GetActiveGauge(c, p_chr_1)).wrapping_add(32); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0)) as i32)) != 0) {
            { let r_ = 1i32; let a_ = ((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        }
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        l_obj_6 = F::BtlChar_GetObj(c, p_chr_1);
        l_aim_7 = 0i32;
        if ((((((((F::BtlSuper_GetFlagsA(c, p_chr_1, l_cls_4)) as u64)) as i32)) & (2048i32))) != 0) {
            l_aim_7 = 1i32;
        }
        if ((((((((((c.m.r32((p_chr_1).wrapping_add(2404)) as i32)) == (1i32)) as i32)) != 0) && (((((c.m.r32(((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) & (1i32))) != 0)) as i32)) != 0) {
            F::BtlChar_SavePlacement(c, p_chr_1);
            F::BtlAct_SetFormCurrent(c, p_chr_1);
            { let a0_ = (c.m.r32(p_chr_1) as i32); let a1_ = (c.m.r32((p_chr_1).wrapping_add(4816)) as i32); let a2_ = (c.m.r32((p_chr_1).wrapping_add(4820)) as i32); let a3_ = (c.m.r32((p_chr_1).wrapping_add(4832)) as i32); let a4_ = (c.m.r32((p_chr_1).wrapping_add(4836)) as i32); let a5_ = (c.m.r32((p_chr_1).wrapping_add(4840)) as i32); let a6_ = (c.m.r32((p_chr_1).wrapping_add(4844)) as i32); F::BtlChange_RequestChara(c, a0_, a1_, a2_, a3_, a4_, a5_, a6_) };
        }
        's1: {
            let sv8: i32 = F::BtlAnim_GetId(c, p_chr_1);
            match sv8 {
                261i32 | 295i32 | 329i32 => {
                    l_aim_7 = 1i32;
                    { let _ = F::BtlAnim_AdvanceThen(c, p_chr_1, (((l_cls_4).wrapping_mul(34i32)).wrapping_add(193i32)).wrapping_add(1i32), F32(0x00000000), 0i32); };
                    F::BtlSuper_RequestHitStop(c, p_chr_1, 0i32);
                    F::BtlSuper_SetClass(c, p_chr_1, l_cls_4);
                    break 's1;
                }
                262i32 | 296i32 | 330i32 => {
                    l_aim_7 = 1i32;
                    F::BtlAnim_AdvanceLoop(c, p_chr_1, 0i32);
                    F::BtlSuper_RequestHitStop(c, p_chr_1, 0i32);
                    F::BtlSuper_SetClass(c, p_chr_1, l_cls_4);
                    F::BtlSuper_SetObjUnk(c, p_chr_1);
                    F::BtlSuper_VibrateCharge(c, p_chr_1);
                    if ((((({ let a_ = l_timer_3; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); n_ }) > (150i32)) as i32)) != 0) {
                        F::BtlChar_SetHeldFlag(c, p_chr_1, ((((167i32) as i64 as u64)) as u32));
                    }
                    if ((F::BtlChar_TestFlag(c, p_chr_1, ((((167i32) as i64 as u64)) as u32))) != 0) {
                        if (((((c.m.r32(((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) & (1i32))) != 0) {
                            if (({ let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlChange_IsLoadedFor(c, a0_) }) != 0) {
                                { let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlChange_SetReady(c, a0_) };
                                F::BtlAnim_Request(c, p_chr_1, (((l_cls_4).wrapping_mul(34i32)).wrapping_add(193i32)).wrapping_add(2i32), F32(0x00000000));
                                { let v_ = 0i32; let a_ = l_timer_3; c.m.w32(a_, (v_) as u32); };
                            }
                        } else {
                            F::BtlAnim_Request(c, p_chr_1, (((l_cls_4).wrapping_mul(34i32)).wrapping_add(193i32)).wrapping_add(2i32), F32(0x00000000));
                            { let v_ = 0i32; let a_ = l_timer_3; c.m.w32(a_, (v_) as u32); };
                        }
                    }
                    break 's1;
                }
                263i32 | 297i32 | 331i32 => {
                    { let _ = F::BtlAnim_AdvanceThen(c, p_chr_1, (((l_cls_4).wrapping_mul(34i32)).wrapping_add(193i32)).wrapping_add(3i32), F32(0x00000000), 0i32); };
                    F::BtlSuper_ShakeOnEvent(c, p_chr_1, l_cls_4, 0i32);
                    F::BtlSuper_Recoil(c, p_chr_1, l_cls_4, 0i32);
                    F::BtlSuper_SetClass(c, p_chr_1, l_cls_4);
                    if (((rt::flt(F::BtlAnim_GetFrame(c, p_chr_1), rt::i2f(F::BtlObjAnim_QueryEvent(c, l_obj_6, ((1024i32) as u64), 0i32, ((((0i32) as i64 as u64)) as u32)))) as i32)) != 0) {
                        l_aim_7 = 1i32;
                        F::BtlSuper_RequestHitStop(c, p_chr_1, 1i32);
                    }
                    if ((F::BtlAnim_IsNew(c, p_chr_1)) != 0) {
                        F::ChrCam_EndCut(c, p_chr_1);
                        if ((((l_flagsB_5) & (((8192i32) as u32)))) != 0) {
                            { let _ = { let a1_ = ({ let a_ = F::BtlMember_GetActiveGauge(c, p_chr_1); (c.m.r32(a_) as i32) }).wrapping_sub(1i32); F::BtlMember_Damage(c, p_chr_1, a1_, 91i32) }; };
                            { let a_ = ((F::BtlMember_GetActiveGauge(c, p_chr_1)).wrapping_add(76)).wrapping_add((((l_cls_4) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        }
                        if (((((c.m.r32(((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) & (1i32))) != 0) {
                            { let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlChange_SetDone(c, a0_) };
                            F::BtlChar_SetFlag(c, p_chr_1, ((((245i32) as i64 as u64)) as u32));
                            F::BtlChar_SetFlag(c, p_chr_1, ((((85i32) as i64 as u64)) as u32));
                        }
                    }
                    break 's1;
                }
                264i32 | 298i32 | 332i32 => {
                    F::BtlAnim_AdvanceLoop(c, p_chr_1, 0i32);
                    F::BtlSuper_ShakeOnEvent(c, p_chr_1, l_cls_4, 1i32);
                    F::BtlSuper_Recoil(c, p_chr_1, l_cls_4, 1i32);
                    if ((((({ let a_ = l_timer_3; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); n_ }) > (150i32)) as i32)) != 0) {
                        F::BtlChar_SetHeldFlag(c, p_chr_1, ((((168i32) as i64 as u64)) as u32));
                    }
                    if ((F::BtlChar_TestFlag(c, p_chr_1, ((((168i32) as i64 as u64)) as u32))) != 0) {
                        F::BtlAnim_Request(c, p_chr_1, (((l_cls_4).wrapping_mul(34i32)).wrapping_add(193i32)).wrapping_add(6i32), F32(0x00000000));
                        { let v_ = 0i32; let a_ = l_timer_3; c.m.w32(a_, (v_) as u32); };
                    }
                    break 's1;
                }
                267i32 | 301i32 | 335i32 => {
                    if ((F::BtlAnim_Advance(c, p_chr_1, 0i32)) != 0) {
                        { let v_ = 1i32; let a_ = ((p_chr_1).wrapping_add(976)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    }
                    break 's1;
                }
                _ => {
                }
            }
        }
        if ((F::BtlSuper_IsThrow(c, p_chr_1, l_cls_4)) != 0) {
            { let a2_ = (c.m.r32((p_chr_1).wrapping_add(2404)) as i32); F::BtlSuper_MeasureRushStep(c, p_chr_1, l_cls_4, a2_) };
        }
        if ((((!((((l_flagsB_5) & (((4096i32) as u32)))) != 0)) as i32)) != 0) {
            { let a1_ = (F::BtlAnim_GetId(c, p_chr_1)).wrapping_add(7i32); let a2_ = (F::BtlAnim_GetId(c, p_chr_1)).wrapping_add(14i32); F::BtlAct_SetPitchMotion(c, p_chr_1, a1_, a2_, l_aim_7) };
        }
        if ((l_aim_7) != 0) {
            F::BtlMove_Step(c, p_chr_1, 2i32, 5i32, 2i32, F32(0x00000000), F32(0x3f6d0979));
            F::BtlMove_ApplyGravity(c, p_chr_1);
        } else {
            F::BtlMove_Step(c, p_chr_1, 6i32, 5i32, 2i32, F32(0x00000000), F32(0x3f6d0979));
            F::BtlMove_ApplyGravity(c, p_chr_1);
        }
        F::BtlChar_SetFxBit(c, p_chr_1, 16i32);
        F::BtlSuper_SetFiringFlags(c, p_chr_1);
        if ((F::BtlInput_TestAction(c, p_chr_1, 111i32, 1i32)) != 0) {
            F::BtlChar_SetHeldFlag(c, p_chr_1, ((((158i32) as i64 as u64)) as u32));
            F::BtlChar_SetHeldFlag(c, p_chr_1, ((((157i32) as i64 as u64)) as u32));
            F::BtlChar_SetHeldFlag(c, p_chr_1, ((((156i32) as i64 as u64)) as u32));
            if ((F::BtlChar_IsFlagRaised(c, p_chr_1, ((156i32) as u32))) != 0) {
                F::BtlCharSnd_PlayCommon(c, p_chr_1, 20i32);
                F::BtlCharSnd_PlayCommon(c, p_chr_1, 21i32);
                F::BtlCharSnd_PlayCommon(c, p_chr_1, 28i32);
            }
        }
    }
    if (((((p_phase_2) == (2i32)) as i32)) != 0) {
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((115i32) as i64 as u64)) as u32))) != 0) {
            F::BtlAct_Request(c, p_chr_1, (l_cls_4).wrapping_add(296i32));
        }
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((114i32) as i64 as u64)) as u32))) != 0) {
            F::BtlAct_Request(c, p_chr_1, (l_cls_4).wrapping_add(299i32));
        }
        if (((c.m.r32(((p_chr_1).wrapping_add(976)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) != 0) {
            F::BtlSuper_Finish(c, p_chr_1, l_cls_4);
        }
    }
    if (((((p_phase_2) == (3i32)) as i32)) != 0) {
        F::BtlSuper_Leave(c, p_chr_1, l_cls_4);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_SuperBeamHandler", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_SuperWarpBeamHandler(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_SuperWarpBeamHandler", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let mut l_sub_3: u32 = 0;
    let mut l_stage_4: u32 = 0;
    let mut l_timer_5: u32 = 0;
    let mut l_cls_6: i32 = 0;
    let mut l_warped_7: i32 = 0;
    let mut l_obj_8: u32 = 0;
    let mut l_aim_9: i32 = 0;
    let mut l_event_10: F32 = F32(0);
    let mut l_frame_11: F32 = F32(0);
    let r_ = 'f: {
    l_sub_3 = ((p_chr_1).wrapping_add(976)).wrapping_add((((3i32) as u32)).wrapping_mul(4));
    l_stage_4 = ((p_chr_1).wrapping_add(976)).wrapping_add((((4i32) as u32)).wrapping_mul(4));
    l_timer_5 = ((p_chr_1).wrapping_add(976)).wrapping_add((((2i32) as u32)).wrapping_mul(4));
    l_cls_6 = F::BtlAct_GetCurrentClass(c, p_chr_1);
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        F::BtlAnim_Play(c, p_chr_1, (((l_cls_6).wrapping_mul(34i32)).wrapping_add(193i32)).wrapping_add(0i32), F32(0x00000000));
        F::BtlSuper_Begin(c, p_chr_1, l_cls_6, 0i32);
        F::BtlSuper_PlayStartSound(c, p_chr_1, l_cls_6, 0i32);
        F::BtlMove_TurnYaw(c, p_chr_1, 2i32, F32(0x40490fda));
        F::BtlMove_TurnPitch(c, p_chr_1, 5i32, F32(0x40490fda));
        F::BtlMove_TurnModelYaw(c, p_chr_1, F32(0x40490fda), F32(0x3f800000));
        F::BtlSuper_SetStartFx(c, p_chr_1, l_cls_6);
        F::BtlSuper_SetClassFlag(c, p_chr_1, l_cls_6, 0i32);
        F::BtlSuper_RequestCut(c, p_chr_1, l_cls_6);
        { let _ = { let a1_ = F::BtlSuper_GetKiCost(c, p_chr_1, l_cls_6); F::BtlMember_SpendKi(c, p_chr_1, a1_, 1i32) }; };
        F::BtlChar_ResetLook(c, p_chr_1);
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        l_warped_7 = 0i32;
        l_obj_8 = F::BtlChar_GetObj(c, p_chr_1);
        l_aim_9 = 0i32;
        if ((((((((F::BtlSuper_GetFlagsA(c, p_chr_1, l_cls_6)) as u64)) as i32)) & (2048i32))) != 0) {
            l_aim_9 = 1i32;
        }
        's1: {
            let sv12: i32 = F::BtlAnim_GetId(c, p_chr_1);
            match sv12 {
                261i32 | 295i32 | 329i32 => {
                    l_aim_9 = 1i32;
                    { let _ = F::BtlAnim_AdvanceThen(c, p_chr_1, (((l_cls_6).wrapping_mul(34i32)).wrapping_add(193i32)).wrapping_add(1i32), F32(0x00000000), 0i32); };
                    F::BtlSuper_RequestHitStop(c, p_chr_1, 0i32);
                    F::BtlSuper_SetClass(c, p_chr_1, l_cls_6);
                    break 's1;
                }
                262i32 | 296i32 | 330i32 => {
                    F::BtlAnim_AdvanceLoop(c, p_chr_1, 0i32);
                    F::BtlSuper_SetObjUnk(c, p_chr_1);
                    F::BtlSuper_VibrateCharge(c, p_chr_1);
                    l_aim_9 = 1i32;
                    's2: {
                        let sv13: i32 = (c.m.r32(l_stage_4) as i32);
                        match sv13 {
                            0i32 => {
                                F::BtlSuper_RequestHitStop(c, p_chr_1, 0i32);
                                F::BtlSuper_SetClass(c, p_chr_1, l_cls_6);
                                if ((((({ let a_ = l_timer_5; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); n_ }) > (150i32)) as i32)) != 0) {
                                    F::BtlChar_SetHeldFlag(c, p_chr_1, ((((167i32) as i64 as u64)) as u32));
                                }
                                if ((F::BtlChar_TestFlag(c, p_chr_1, ((((167i32) as i64 as u64)) as u32))) != 0) {
                                    { let a_ = l_stage_4; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                                    { let v_ = 0i32; let a_ = l_sub_3; c.m.w32(a_, (v_) as u32); };
                                    { let v_ = 0i32; let a_ = l_timer_5; c.m.w32(a_, (v_) as u32); };
                                    F::BtlChar_SetFxBit(c, p_chr_1, 12i32);
                                    F::BtlCharSnd_PlayCommon(c, p_chr_1, 32i32);
                                }
                                break 's2;
                            }
                            1i32 => {
                                F::BtlSuper_RequestHitStop(c, p_chr_1, 0i32);
                                if ((((({ let a_ = l_sub_3; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); n_ }) > (3i32)) as i32)) != 0) {
                                    F::BtlChar_SetFlag(c, p_chr_1, ((((11i32) as i64 as u64)) as u32));
                                }
                                if ((((((c.m.r32(l_sub_3) as i32)) >= (7i32)) as i32)) != 0) {
                                    { let a_ = l_stage_4; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                                    { let v_ = 0i32; let a_ = l_sub_3; c.m.w32(a_, (v_) as u32); };
                                }
                                break 's2;
                            }
                            2i32 => {
                                F::BtlSuper_RequestHitStop(c, p_chr_1, 0i32);
                                F::BtlMove_SnapToOpponent(c, p_chr_1);
                                F::BtlChar_SetFlag(c, p_chr_1, ((((11i32) as i64 as u64)) as u32));
                                l_warped_7 = 1i32;
                                F::BtlChar_SetFlag(c, p_chr_1, ((((204i32) as i64 as u64)) as u32));
                                { let a_ = l_stage_4; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                                { let v_ = 0i32; let a_ = l_sub_3; c.m.w32(a_, (v_) as u32); };
                                break 's2;
                            }
                            3i32 => {
                                F::BtlSuper_RequestHitStop(c, p_chr_1, 0i32);
                                F::BtlMove_WarpInFrontOfOpponent(c, p_chr_1, F32(0x42200000));
                                l_warped_7 = 1i32;
                                { let r_ = F32(0x41a00000); let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(4); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                                F::BtlMove_TurnYaw(c, p_chr_1, 2i32, F32(0x40490fda));
                                F::BtlMove_TurnPitch(c, p_chr_1, 1i32, F32(0x40490fda));
                                F::BtlMove_TurnModelYaw(c, p_chr_1, F32(0x40490fda), F32(0x3f800000));
                                F::BtlMove_SetDirection(c, p_chr_1, 3i32);
                                F::BtlChar_SetFlag(c, p_chr_1, ((((63i32) as i64 as u64)) as u32));
                                F::BtlChar_SetFlag(c, p_chr_1, ((((11i32) as i64 as u64)) as u32));
                                F::BtlChar_SetFxBit(c, p_chr_1, 13i32);
                                F::BtlChar_SetFlag(c, p_chr_1, ((((204i32) as i64 as u64)) as u32));
                                F::BtlChar_SetFlag(c, p_chr_1, ((((206i32) as i64 as u64)) as u32));
                                F::BtlChar_SetFlag(c, p_chr_1, ((((36i32) as i64 as u64)) as u32));
                                { let a_ = l_stage_4; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                                { let v_ = 0i32; let a_ = l_sub_3; c.m.w32(a_, (v_) as u32); };
                                break 's2;
                            }
                            4i32 => {
                                F::BtlSuper_RequestHitStop(c, p_chr_1, 1i32);
                                if ((((({ let a_ = l_sub_3; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); n_ }) == (1i32)) as i32)) != 0) {
                                    F::ChrCam_EndCut(c, p_chr_1);
                                }
                                if ((((((c.m.r32(l_sub_3) as i32)) < (3i32)) as i32)) != 0) {
                                    F::BtlChar_SetFlag(c, p_chr_1, ((((11i32) as i64 as u64)) as u32));
                                }
                                if ((((((c.m.r32(l_sub_3) as i32)) >= (7i32)) as i32)) != 0) {
                                    F::BtlAnim_Request(c, p_chr_1, (((l_cls_6).wrapping_mul(34i32)).wrapping_add(193i32)).wrapping_add(2i32), F32(0x00000000));
                                }
                                break 's2;
                            }
                            _ => {
                            }
                        }
                    }
                    break 's1;
                }
                263i32 | 297i32 | 331i32 => {
                    { let _ = F::BtlAnim_AdvanceThen(c, p_chr_1, (((l_cls_6).wrapping_mul(34i32)).wrapping_add(193i32)).wrapping_add(3i32), F32(0x00000000), 0i32); };
                    F::BtlSuper_ShakeOnEvent(c, p_chr_1, l_cls_6, 0i32);
                    F::BtlSuper_Recoil(c, p_chr_1, l_cls_6, 0i32);
                    l_event_10 = rt::i2f(F::BtlObjAnim_QueryEvent(c, l_obj_8, ((1024i32) as u64), 0i32, ((((0i32) as i64 as u64)) as u32)));
                    l_frame_11 = F::BtlAnim_GetFrame(c, p_chr_1);
                    if (((rt::flt(l_frame_11, rt::fsub(l_event_10, F32(0x40800000))) as i32)) != 0) {
                        F::BtlSuper_RequestHitStop(c, p_chr_1, 1i32);
                    }
                    if (((rt::flt(l_frame_11, l_event_10) as i32)) != 0) {
                        l_aim_9 = 1i32;
                    }
                    break 's1;
                }
                264i32 | 298i32 | 332i32 => {
                    F::BtlAnim_AdvanceLoop(c, p_chr_1, 0i32);
                    F::BtlSuper_ShakeOnEvent(c, p_chr_1, l_cls_6, 1i32);
                    F::BtlSuper_Recoil(c, p_chr_1, l_cls_6, 1i32);
                    if ((((({ let a_ = l_timer_5; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); n_ }) > (150i32)) as i32)) != 0) {
                        F::BtlChar_SetHeldFlag(c, p_chr_1, ((((168i32) as i64 as u64)) as u32));
                    }
                    if ((F::BtlChar_TestFlag(c, p_chr_1, ((((168i32) as i64 as u64)) as u32))) != 0) {
                        F::BtlAnim_Request(c, p_chr_1, (((l_cls_6).wrapping_mul(34i32)).wrapping_add(193i32)).wrapping_add(6i32), F32(0x00000000));
                        { let v_ = 0i32; let a_ = l_timer_5; c.m.w32(a_, (v_) as u32); };
                    }
                    break 's1;
                }
                267i32 | 301i32 | 335i32 => {
                    if ((F::BtlAnim_Advance(c, p_chr_1, 0i32)) != 0) {
                        { let v_ = 1i32; let a_ = ((p_chr_1).wrapping_add(976)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    }
                    break 's1;
                }
                _ => {
                }
            }
        }
        if ((((!((((((((F::BtlSuper_GetFlags(c, p_chr_1, l_cls_6)) as u64)) as i32)) & (4096i32))) != 0)) as i32)) != 0) {
            { let a1_ = (F::BtlAnim_GetId(c, p_chr_1)).wrapping_add(7i32); let a2_ = (F::BtlAnim_GetId(c, p_chr_1)).wrapping_add(14i32); F::BtlAct_SetPitchMotion(c, p_chr_1, a1_, a2_, l_aim_9) };
        }
        if ((((!((l_warped_7) != 0)) as i32)) != 0) {
            if ((l_aim_9) != 0) {
                F::BtlMove_Step(c, p_chr_1, 2i32, 5i32, 2i32, F32(0x00000000), F32(0x3f6d0979));
                F::BtlMove_ApplyGravity(c, p_chr_1);
            } else {
                F::BtlMove_Step(c, p_chr_1, 6i32, 5i32, 2i32, F32(0x00000000), F32(0x3f6d0979));
                F::BtlMove_ApplyGravity(c, p_chr_1);
            }
        }
        F::BtlChar_SetFxBit(c, p_chr_1, 16i32);
        F::BtlSuper_SetFiringFlags(c, p_chr_1);
        if ((F::BtlInput_TestAction(c, p_chr_1, 111i32, 1i32)) != 0) {
            F::BtlChar_SetHeldFlag(c, p_chr_1, ((((158i32) as i64 as u64)) as u32));
            F::BtlChar_SetHeldFlag(c, p_chr_1, ((((157i32) as i64 as u64)) as u32));
            F::BtlChar_SetHeldFlag(c, p_chr_1, ((((156i32) as i64 as u64)) as u32));
            if ((F::BtlChar_IsFlagRaised(c, p_chr_1, ((156i32) as u32))) != 0) {
                F::BtlCharSnd_PlayCommon(c, p_chr_1, 20i32);
                F::BtlCharSnd_PlayCommon(c, p_chr_1, 21i32);
                F::BtlCharSnd_PlayCommon(c, p_chr_1, 28i32);
            }
        }
    }
    if (((((p_phase_2) == (2i32)) as i32)) != 0) {
        if (((c.m.r32(((p_chr_1).wrapping_add(976)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) != 0) {
            F::BtlSuper_Finish(c, p_chr_1, l_cls_6);
        }
    }
    if (((((p_phase_2) == (3i32)) as i32)) != 0) {
        F::BtlSuper_Leave(c, p_chr_1, l_cls_6);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_SuperWarpBeamHandler", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_SuperLongBeamHandler(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_SuperLongBeamHandler", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let mut l_timer_3: u32 = 0;
    let mut l_cls_4: i32 = 0;
    let mut l_aim_5: i32 = 0;
    let r_ = 'f: {
    l_timer_3 = ((p_chr_1).wrapping_add(976)).wrapping_add((((2i32) as u32)).wrapping_mul(4));
    l_cls_4 = F::BtlAct_GetCurrentClass(c, p_chr_1);
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        F::BtlAnim_Play(c, p_chr_1, (((l_cls_4).wrapping_mul(34i32)).wrapping_add(193i32)).wrapping_add(0i32), F32(0x00000000));
        F::BtlSuper_Begin(c, p_chr_1, l_cls_4, 0i32);
        F::BtlSuper_PlayStartSound(c, p_chr_1, l_cls_4, 0i32);
        F::BtlMove_TurnYaw(c, p_chr_1, 2i32, F32(0x40490fda));
        F::BtlMove_TurnPitch(c, p_chr_1, 5i32, F32(0x40490fda));
        F::BtlMove_TurnModelYaw(c, p_chr_1, F32(0x40490fda), F32(0x3f800000));
        F::BtlSuper_SetStartFx(c, p_chr_1, l_cls_4);
        F::BtlSuper_SetClassFlag(c, p_chr_1, l_cls_4, 0i32);
        F::BtlSuper_RequestCut(c, p_chr_1, l_cls_4);
        { let _ = { let a1_ = F::BtlSuper_GetKiCost(c, p_chr_1, l_cls_4); F::BtlMember_SpendKi(c, p_chr_1, a1_, 1i32) }; };
        F::BtlChar_ResetLook(c, p_chr_1);
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        l_aim_5 = 0i32;
        if ((((((((F::BtlSuper_GetFlagsA(c, p_chr_1, l_cls_4)) as u64)) as i32)) & (2048i32))) != 0) {
            l_aim_5 = 1i32;
        }
        's1: {
            let sv6: i32 = F::BtlAnim_GetId(c, p_chr_1);
            match sv6 {
                261i32 | 295i32 | 329i32 => {
                    l_aim_5 = 1i32;
                    { let _ = F::BtlAnim_AdvanceThen(c, p_chr_1, (((l_cls_4).wrapping_mul(34i32)).wrapping_add(193i32)).wrapping_add(1i32), F32(0x00000000), 0i32); };
                    F::BtlSuper_RequestHitStop(c, p_chr_1, 0i32);
                    F::BtlSuper_SetClass(c, p_chr_1, l_cls_4);
                    break 's1;
                }
                262i32 | 296i32 | 330i32 => {
                    l_aim_5 = 1i32;
                    F::BtlAnim_AdvanceLoop(c, p_chr_1, 0i32);
                    F::BtlSuper_RequestHitStop(c, p_chr_1, 0i32);
                    F::BtlSuper_SetClass(c, p_chr_1, l_cls_4);
                    F::BtlSuper_SetObjUnk(c, p_chr_1);
                    F::BtlSuper_VibrateCharge(c, p_chr_1);
                    if ((((({ let a_ = l_timer_3; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); n_ }) > (150i32)) as i32)) != 0) {
                        F::BtlChar_SetHeldFlag(c, p_chr_1, ((((167i32) as i64 as u64)) as u32));
                    }
                    if ((F::BtlChar_TestFlag(c, p_chr_1, ((((167i32) as i64 as u64)) as u32))) != 0) {
                        F::BtlAnim_Request(c, p_chr_1, (((l_cls_4).wrapping_mul(34i32)).wrapping_add(193i32)).wrapping_add(2i32), F32(0x00000000));
                        { let v_ = 0i32; let a_ = l_timer_3; c.m.w32(a_, (v_) as u32); };
                    }
                    break 's1;
                }
                263i32 | 297i32 | 331i32 => {
                    { let _ = F::BtlAnim_AdvanceThen(c, p_chr_1, (((l_cls_4).wrapping_mul(34i32)).wrapping_add(193i32)).wrapping_add(3i32), F32(0x00000000), 0i32); };
                    F::BtlSuper_RequestHitStop(c, p_chr_1, 1i32);
                    F::BtlSuper_ShakeOnEvent(c, p_chr_1, l_cls_4, 0i32);
                    F::BtlSuper_Recoil(c, p_chr_1, l_cls_4, 0i32);
                    F::BtlSuper_SetClass(c, p_chr_1, l_cls_4);
                    if ((F::BtlAnim_IsNew(c, p_chr_1)) != 0) {
                        F::ChrCam_EndCut(c, p_chr_1);
                    }
                    break 's1;
                }
                264i32 | 298i32 | 332i32 => {
                    F::BtlAnim_AdvanceLoop(c, p_chr_1, 0i32);
                    F::BtlSuper_RequestHitStop(c, p_chr_1, 1i32);
                    F::BtlSuper_ShakeOnEvent(c, p_chr_1, l_cls_4, 1i32);
                    F::BtlSuper_Recoil(c, p_chr_1, l_cls_4, 1i32);
                    if ((((({ let a_ = l_timer_3; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); n_ }) > (150i32)) as i32)) != 0) {
                        F::BtlChar_SetHeldFlag(c, p_chr_1, ((((168i32) as i64 as u64)) as u32));
                    }
                    if ((F::BtlChar_TestFlag(c, p_chr_1, ((((168i32) as i64 as u64)) as u32))) != 0) {
                        F::BtlAnim_Request(c, p_chr_1, (((l_cls_4).wrapping_mul(34i32)).wrapping_add(193i32)).wrapping_add(4i32), F32(0x00000000));
                        { let v_ = 0i32; let a_ = l_timer_3; c.m.w32(a_, (v_) as u32); };
                    }
                    break 's1;
                }
                265i32 | 299i32 | 333i32 => {
                    { let _ = F::BtlAnim_AdvanceThen(c, p_chr_1, (((l_cls_4).wrapping_mul(34i32)).wrapping_add(193i32)).wrapping_add(5i32), F32(0x00000000), 0i32); };
                    break 's1;
                }
                266i32 | 300i32 | 334i32 => {
                    F::BtlAnim_AdvanceLoop(c, p_chr_1, 0i32);
                    if ((((({ let a_ = l_timer_3; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); n_ }) > (150i32)) as i32)) != 0) {
                        F::BtlChar_SetHeldFlag(c, p_chr_1, ((((169i32) as i64 as u64)) as u32));
                    }
                    if ((F::BtlChar_TestFlag(c, p_chr_1, ((((169i32) as i64 as u64)) as u32))) != 0) {
                        F::BtlAnim_Request(c, p_chr_1, (((l_cls_4).wrapping_mul(34i32)).wrapping_add(193i32)).wrapping_add(6i32), F32(0x00000000));
                        { let v_ = 0i32; let a_ = l_timer_3; c.m.w32(a_, (v_) as u32); };
                    }
                    break 's1;
                }
                267i32 | 301i32 | 335i32 => {
                    if ((F::BtlAnim_Advance(c, p_chr_1, 0i32)) != 0) {
                        { let v_ = 1i32; let a_ = ((p_chr_1).wrapping_add(976)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    }
                    break 's1;
                }
                _ => {
                }
            }
        }
        if ((((!((((((((F::BtlSuper_GetFlags(c, p_chr_1, l_cls_4)) as u64)) as i32)) & (4096i32))) != 0)) as i32)) != 0) {
            { let a1_ = (F::BtlAnim_GetId(c, p_chr_1)).wrapping_add(7i32); let a2_ = (F::BtlAnim_GetId(c, p_chr_1)).wrapping_add(14i32); F::BtlAct_SetPitchMotion(c, p_chr_1, a1_, a2_, l_aim_5) };
        }
        if ((l_aim_5) != 0) {
            F::BtlMove_Step(c, p_chr_1, 2i32, 5i32, 2i32, F32(0x00000000), F32(0x3f6d0979));
            F::BtlMove_ApplyGravity(c, p_chr_1);
        } else {
            F::BtlMove_Step(c, p_chr_1, 6i32, 5i32, 2i32, F32(0x00000000), F32(0x3f6d0979));
            F::BtlMove_ApplyGravity(c, p_chr_1);
        }
        F::BtlChar_SetFxBit(c, p_chr_1, 16i32);
        F::BtlSuper_SetFiringFlags(c, p_chr_1);
        if ((F::BtlInput_TestAction(c, p_chr_1, 111i32, 1i32)) != 0) {
            F::BtlChar_SetHeldFlag(c, p_chr_1, ((((158i32) as i64 as u64)) as u32));
            F::BtlChar_SetHeldFlag(c, p_chr_1, ((((157i32) as i64 as u64)) as u32));
            F::BtlChar_SetHeldFlag(c, p_chr_1, ((((156i32) as i64 as u64)) as u32));
            if ((F::BtlChar_IsFlagRaised(c, p_chr_1, ((156i32) as u32))) != 0) {
                F::BtlCharSnd_PlayCommon(c, p_chr_1, 20i32);
                F::BtlCharSnd_PlayCommon(c, p_chr_1, 21i32);
                F::BtlCharSnd_PlayCommon(c, p_chr_1, 28i32);
            }
        }
    }
    if (((((p_phase_2) == (2i32)) as i32)) != 0) {
        if (((c.m.r32(((p_chr_1).wrapping_add(976)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) != 0) {
            F::BtlSuper_Finish(c, p_chr_1, l_cls_4);
        }
    }
    if (((((p_phase_2) == (3i32)) as i32)) != 0) {
        F::BtlSuper_Leave(c, p_chr_1, l_cls_4);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_SuperLongBeamHandler", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_SuperChargeHandler(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_SuperChargeHandler", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let mut l_timer_3: u32 = 0;
    let mut l_cls_4: i32 = 0;
    let mut l_obj_5: u32 = 0;
    let mut l_limit_6: i32 = 0;
    let mut l_aim_7: i32 = 0;
    let r_ = 'f: {
    l_timer_3 = ((p_chr_1).wrapping_add(976)).wrapping_add((((2i32) as u32)).wrapping_mul(4));
    l_cls_4 = F::BtlAct_GetCurrentClass(c, p_chr_1);
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        F::BtlAnim_Play(c, p_chr_1, (((l_cls_4).wrapping_mul(34i32)).wrapping_add(193i32)).wrapping_add(0i32), F32(0x3e199999));
        F::BtlSuper_Begin(c, p_chr_1, l_cls_4, 0i32);
        F::BtlSuper_PlayStartSound(c, p_chr_1, l_cls_4, 0i32);
        F::BtlSuper_SetStartFx(c, p_chr_1, l_cls_4);
        F::BtlSuper_SetClassFlag(c, p_chr_1, l_cls_4, 0i32);
        { let _ = { let a1_ = F::BtlSuper_GetKiCost(c, p_chr_1, l_cls_4); F::BtlMember_SpendKi(c, p_chr_1, a1_, 1i32) }; };
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        l_obj_5 = F::BtlChar_GetObj(c, p_chr_1);
        l_limit_6 = F::BtlSuper_GetChargeLimitFrames(c, p_chr_1, l_cls_4);
        l_aim_7 = 0i32;
        if ((((((((F::BtlSuper_GetFlagsA(c, p_chr_1, l_cls_4)) as u64)) as i32)) & (2048i32))) != 0) {
            l_aim_7 = 1i32;
        }
        's1: {
            let sv8: i32 = F::BtlAnim_GetId(c, p_chr_1);
            match sv8 {
                261i32 | 295i32 | 329i32 => {
                    l_aim_7 = 1i32;
                    { let _ = F::BtlAnim_AdvanceThen(c, p_chr_1, (((l_cls_4).wrapping_mul(34i32)).wrapping_add(193i32)).wrapping_add(1i32), F32(0x00000000), 0i32); };
                    F::BtlSuper_SetClass(c, p_chr_1, l_cls_4);
                    F::BtlChar_SetFlag(c, p_chr_1, ((((83i32) as i64 as u64)) as u32));
                    break 's1;
                }
                262i32 | 296i32 | 330i32 => {
                    F::BtlAnim_AdvanceLoop(c, p_chr_1, 0i32);
                    F::BtlSuper_SetObjUnk(c, p_chr_1);
                    F::BtlSuper_VibrateCharge(c, p_chr_1);
                    l_aim_7 = 1i32;
                    F::BtlSuper_SetClass(c, p_chr_1, l_cls_4);
                    F::BtlChar_SetFlag(c, p_chr_1, ((((83i32) as i64 as u64)) as u32));
                    if ((((({ let a_ = l_timer_3; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); n_ }) >= (l_limit_6)) as i32)) != 0) {
                        { let r_ = 1i32; let a_ = ((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    }
                    { let v_ = rt::fdiv(rt::fmul(rt::i2f((c.m.r32(l_timer_3) as i32)), F32(0x40000000)), rt::i2f(l_limit_6)); let a_ = (p_chr_1).wrapping_add(3652); c.m.w32(a_, (v_).0); };
                    if ((F::BtlMember_HasAbility(c, p_chr_1, 74i32)) != 0) {
                        { let r_ = F32(0x3fc00000); let a_ = (p_chr_1).wrapping_add(3652); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    }
                    if (((rt::fge(F32(c.m.r32((p_chr_1).wrapping_add(3652))), F32(0x3f800000)) as i32)) != 0) {
                        { let v_ = F32(0x3f800000); let a_ = (p_chr_1).wrapping_add(3652); c.m.w32(a_, (v_).0); };
                        F::BtlChar_SetFxBit(c, p_chr_1, 30i32);
                        if ((((((c.m.r32((p_chr_1).wrapping_add(3656)) as i32)) == (0i32)) as i32)) != 0) {
                            F::BtlCharSnd_PlayCommon(c, p_chr_1, 20i32);
                            F::BtlCharSnd_PlayCommon(c, p_chr_1, 21i32);
                            F::BtlCharSnd_PlayCommon(c, p_chr_1, 28i32);
                        }
                        { let a_ = (p_chr_1).wrapping_add(3656); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    }
                    if (((((c.m.r32(((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) & (1i32))) != 0) {
                        F::BtlAnim_Request(c, p_chr_1, (((l_cls_4).wrapping_mul(34i32)).wrapping_add(193i32)).wrapping_add(2i32), F32(0x00000000));
                        { let v_ = 0i32; let a_ = l_timer_3; c.m.w32(a_, (v_) as u32); };
                    }
                    break 's1;
                }
                263i32 | 297i32 | 331i32 => {
                    { let _ = F::BtlAnim_AdvanceThen(c, p_chr_1, (((l_cls_4).wrapping_mul(34i32)).wrapping_add(193i32)).wrapping_add(3i32), F32(0x00000000), 0i32); };
                    F::BtlSuper_ShakeOnEvent(c, p_chr_1, l_cls_4, 0i32);
                    F::BtlSuper_Recoil(c, p_chr_1, l_cls_4, 0i32);
                    F::BtlChar_SetFlag(c, p_chr_1, ((((83i32) as i64 as u64)) as u32));
                    if (((rt::flt(F::BtlAnim_GetFrame(c, p_chr_1), rt::i2f(F::BtlObjAnim_QueryEvent(c, l_obj_5, ((1024i32) as u64), 0i32, ((((0i32) as i64 as u64)) as u32)))) as i32)) != 0) {
                        l_aim_7 = 1i32;
                    }
                    break 's1;
                }
                264i32 | 298i32 | 332i32 => {
                    F::BtlAnim_AdvanceLoop(c, p_chr_1, 0i32);
                    F::BtlSuper_ShakeOnEvent(c, p_chr_1, l_cls_4, 1i32);
                    F::BtlSuper_Recoil(c, p_chr_1, l_cls_4, 1i32);
                    if ((((({ let a_ = l_timer_3; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); n_ }) > (150i32)) as i32)) != 0) {
                        F::BtlChar_SetHeldFlag(c, p_chr_1, ((((168i32) as i64 as u64)) as u32));
                    }
                    if ((F::BtlChar_TestFlag(c, p_chr_1, ((((168i32) as i64 as u64)) as u32))) != 0) {
                        F::BtlAnim_Request(c, p_chr_1, (((l_cls_4).wrapping_mul(34i32)).wrapping_add(193i32)).wrapping_add(6i32), F32(0x00000000));
                        { let v_ = 0i32; let a_ = l_timer_3; c.m.w32(a_, (v_) as u32); };
                    }
                    break 's1;
                }
                267i32 | 301i32 | 335i32 => {
                    if ((F::BtlAnim_Advance(c, p_chr_1, 0i32)) != 0) {
                        { let v_ = 1i32; let a_ = ((p_chr_1).wrapping_add(976)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    }
                    break 's1;
                }
                _ => {
                }
            }
        }
        if ((((!((((((((F::BtlSuper_GetFlags(c, p_chr_1, l_cls_4)) as u64)) as i32)) & (4096i32))) != 0)) as i32)) != 0) {
            { let a1_ = (F::BtlAnim_GetId(c, p_chr_1)).wrapping_add(7i32); let a2_ = (F::BtlAnim_GetId(c, p_chr_1)).wrapping_add(14i32); F::BtlAct_SetPitchMotion(c, p_chr_1, a1_, a2_, l_aim_7) };
        }
        if ((l_aim_7) != 0) {
            F::BtlMove_Step(c, p_chr_1, 2i32, 5i32, 2i32, F32(0x00000000), F32(0x3f6d0979));
            F::BtlMove_ApplyGravity(c, p_chr_1);
        } else {
            F::BtlMove_Step(c, p_chr_1, 6i32, 5i32, 2i32, F32(0x00000000), F32(0x3f6d0979));
            F::BtlMove_ApplyGravity(c, p_chr_1);
        }
        F::BtlChar_SetFxBit(c, p_chr_1, 16i32);
        F::BtlSuper_SetFiringFlags(c, p_chr_1);
        F::BtlChar_SetFlag(c, p_chr_1, ((((162i32) as i64 as u64)) as u32));
    }
    if (((((p_phase_2) == (2i32)) as i32)) != 0) {
        's2: {
            let sv9: i32 = F::BtlAnim_GetId(c, p_chr_1);
            match sv9 {
                261i32 | 295i32 | 329i32 => {
                    break 's2;
                }
                262i32 | 296i32 | 330i32 => {
                    if ((F::BtlInput_TestAction(c, p_chr_1, 111i32, 1i32)) != 0) {
                        F::BtlChar_SetHeldFlag(c, p_chr_1, ((((158i32) as i64 as u64)) as u32));
                        F::BtlChar_SetHeldFlag(c, p_chr_1, ((((157i32) as i64 as u64)) as u32));
                        { let r_ = 1i32; let a_ = ((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                        if (((rt::fge(F32(c.m.r32((p_chr_1).wrapping_add(3652))), F32(0x3f800000)) as i32)) != 0) {
                            F::BtlChar_SetHeldFlag(c, p_chr_1, ((((156i32) as i64 as u64)) as u32));
                        }
                    }
                    break 's2;
                }
                263i32 | 297i32 | 331i32 => {
                    break 's2;
                }
                264i32 | 298i32 | 332i32 => {
                    break 's2;
                }
                267i32 | 301i32 | 335i32 => {
                    break 's2;
                }
                _ => {
                }
            }
        }
        if (((c.m.r32(((p_chr_1).wrapping_add(976)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) != 0) {
            F::BtlSuper_Finish(c, p_chr_1, l_cls_4);
        }
    }
    if (((((p_phase_2) == (3i32)) as i32)) != 0) {
        F::BtlSuper_Leave(c, p_chr_1, l_cls_4);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_SuperChargeHandler", &PROBE_N, &[], true); }
    r_
}

pub fn btl_act_f__BtlSuper_IsLastShot(c: &mut Ctx, mut p_chr_1: u32, mut p_cls_2: i32, mut p_shots_3: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "btl_act_f__BtlSuper_IsLastShot", &PROBE_N, &[(p_chr_1) as u32, (p_cls_2) as u32, (p_shots_3) as u32], false); }
    let r_ = 'f: {
    if ((((({ let a_ = F::BtlMember_GetActiveGauge(c, p_chr_1); (c.m.r32(a_) as i32) }) == (1i32)) as i32)) != 0) {
        break 'f 1i32;
    }
    if ((((((c.m.r32(p_shots_3) as i32)) < (F::BtlSuper_GetShots(c, p_chr_1, p_cls_2))) as i32)) != 0) {
        break 'f 0i32;
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "btl_act_f__BtlSuper_IsLastShot", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_SuperRepeatHandler(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_SuperRepeatHandler", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let mut l_timer_3: u32 = 0;
    let mut l_shots_4: u32 = 0;
    let mut l_cls_5: i32 = 0;
    let mut l_obj_6: u32 = 0;
    let mut l_aim_7: i32 = 0;
    let mut l_stop_8: i32 = 0;
    let r_ = 'f: {
    l_timer_3 = ((p_chr_1).wrapping_add(976)).wrapping_add((((2i32) as u32)).wrapping_mul(4));
    l_shots_4 = ((p_chr_1).wrapping_add(976)).wrapping_add((((3i32) as u32)).wrapping_mul(4));
    l_cls_5 = F::BtlAct_GetCurrentClass(c, p_chr_1);
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        F::BtlAnim_Play(c, p_chr_1, (((l_cls_5).wrapping_mul(34i32)).wrapping_add(193i32)).wrapping_add(0i32), F32(0x00000000));
        F::BtlSuper_Begin(c, p_chr_1, l_cls_5, 0i32);
        F::BtlSuper_PlayStartSound(c, p_chr_1, l_cls_5, 0i32);
        F::BtlMove_TurnYaw(c, p_chr_1, 2i32, F32(0x40490fda));
        F::BtlMove_TurnPitch(c, p_chr_1, 5i32, F32(0x40490fda));
        F::BtlMove_TurnModelYaw(c, p_chr_1, F32(0x40490fda), F32(0x3f800000));
        F::BtlSuper_SetStartFx(c, p_chr_1, l_cls_5);
        F::BtlSuper_SetClassFlag(c, p_chr_1, l_cls_5, 0i32);
        F::BtlSuper_RequestCut(c, p_chr_1, l_cls_5);
        { let _ = { let a1_ = F::BtlSuper_GetKiCost(c, p_chr_1, l_cls_5); F::BtlMember_SpendKi(c, p_chr_1, a1_, 1i32) }; };
        F::BtlChar_ResetLook(c, p_chr_1);
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        l_obj_6 = F::BtlChar_GetObj(c, p_chr_1);
        l_aim_7 = 0i32;
        if ((((((((F::BtlSuper_GetFlagsA(c, p_chr_1, l_cls_5)) as u64)) as i32)) & (2048i32))) != 0) {
            l_aim_7 = 1i32;
        }
        's1: {
            let sv9: i32 = F::BtlAnim_GetId(c, p_chr_1);
            match sv9 {
                261i32 | 295i32 | 329i32 => {
                    { let _ = F::BtlAnim_AdvanceThen(c, p_chr_1, (((l_cls_5).wrapping_mul(34i32)).wrapping_add(193i32)).wrapping_add(1i32), F32(0x00000000), 0i32); };
                    F::BtlSuper_RequestHitStop(c, p_chr_1, 0i32);
                    F::BtlSuper_SetClass(c, p_chr_1, l_cls_5);
                    if ((F::BtlInput_TestAction(c, p_chr_1, 111i32, 1i32)) != 0) {
                        F::BtlChar_SetHeldFlag(c, p_chr_1, ((((158i32) as i64 as u64)) as u32));
                        F::BtlChar_SetHeldFlag(c, p_chr_1, ((((157i32) as i64 as u64)) as u32));
                        F::BtlChar_SetHeldFlag(c, p_chr_1, ((((156i32) as i64 as u64)) as u32));
                        if ((F::BtlChar_IsFlagRaised(c, p_chr_1, ((156i32) as u32))) != 0) {
                            F::BtlCharSnd_PlayCommon(c, p_chr_1, 20i32);
                            F::BtlCharSnd_PlayCommon(c, p_chr_1, 21i32);
                            F::BtlCharSnd_PlayCommon(c, p_chr_1, 28i32);
                        }
                    }
                    l_aim_7 = 1i32;
                    break 's1;
                }
                262i32 | 296i32 | 330i32 => {
                    F::BtlAnim_AdvanceLoop(c, p_chr_1, 0i32);
                    F::BtlSuper_RequestHitStop(c, p_chr_1, 0i32);
                    F::BtlSuper_SetObjUnk(c, p_chr_1);
                    F::BtlSuper_VibrateCharge(c, p_chr_1);
                    F::BtlSuper_SetClass(c, p_chr_1, l_cls_5);
                    if ((F::BtlInput_TestAction(c, p_chr_1, 111i32, 1i32)) != 0) {
                        F::BtlChar_SetHeldFlag(c, p_chr_1, ((((158i32) as i64 as u64)) as u32));
                        F::BtlChar_SetHeldFlag(c, p_chr_1, ((((157i32) as i64 as u64)) as u32));
                        F::BtlChar_SetHeldFlag(c, p_chr_1, ((((156i32) as i64 as u64)) as u32));
                        if ((F::BtlChar_IsFlagRaised(c, p_chr_1, ((156i32) as u32))) != 0) {
                            F::BtlCharSnd_PlayCommon(c, p_chr_1, 20i32);
                            F::BtlCharSnd_PlayCommon(c, p_chr_1, 21i32);
                            F::BtlCharSnd_PlayCommon(c, p_chr_1, 28i32);
                        }
                    }
                    l_aim_7 = 1i32;
                    if ((((({ let a_ = l_timer_3; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); n_ }) > (150i32)) as i32)) != 0) {
                        F::BtlChar_SetHeldFlag(c, p_chr_1, ((((167i32) as i64 as u64)) as u32));
                    }
                    if ((F::BtlChar_TestFlag(c, p_chr_1, ((((167i32) as i64 as u64)) as u32))) != 0) {
                        F::BtlAnim_Request(c, p_chr_1, (((l_cls_5).wrapping_mul(34i32)).wrapping_add(193i32)).wrapping_add(2i32), F32(0x00000000));
                        { let v_ = 0i32; let a_ = l_timer_3; c.m.w32(a_, (v_) as u32); };
                    }
                    break 's1;
                }
                263i32 | 297i32 | 331i32 => {
                    { let _ = F::BtlAnim_AdvanceThen(c, p_chr_1, (((l_cls_5).wrapping_mul(34i32)).wrapping_add(193i32)).wrapping_add(3i32), F32(0x00000000), 0i32); };
                    F::BtlSuper_ShakeOnEvent(c, p_chr_1, l_cls_5, 0i32);
                    F::BtlSuper_Recoil(c, p_chr_1, l_cls_5, 0i32);
                    if (((rt::flt(F::BtlAnim_GetFrame(c, p_chr_1), rt::i2f(F::BtlObjAnim_QueryEvent(c, l_obj_6, ((1024i32) as u64), 0i32, ((((0i32) as i64 as u64)) as u32)))) as i32)) != 0) {
                        l_aim_7 = 1i32;
                        F::BtlSuper_RequestHitStop(c, p_chr_1, 1i32);
                    }
                    if ((F::BtlAnim_IsNew(c, p_chr_1)) != 0) {
                        { let _ = { let a1_ = F::BtlSuper_GetRecoilDamage(c, p_chr_1, l_cls_5); F::BtlMember_Damage(c, p_chr_1, a1_, 1067i32) }; };
                        { let a_ = l_shots_4; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        { let v_ = 0i32; let a_ = l_timer_3; c.m.w32(a_, (v_) as u32); };
                        F::ChrCam_EndCut(c, p_chr_1);
                        if ((((((c.m.r32(l_shots_4) as i32)) == (1i32)) as i32)) != 0) {
                            F::BtlChar_ClearFlag(c, p_chr_1, ((((158i32) as i64 as u64)) as u32));
                            F::BtlChar_ClearFlag(c, p_chr_1, ((((157i32) as i64 as u64)) as u32));
                        }
                    }
                    break 's1;
                }
                264i32 | 298i32 | 332i32 => {
                    F::BtlAnim_AdvanceLoop(c, p_chr_1, 0i32);
                    F::BtlSuper_ShakeOnEvent(c, p_chr_1, l_cls_5, 1i32);
                    F::BtlSuper_Recoil(c, p_chr_1, l_cls_5, 1i32);
                    break 's1;
                }
                267i32 | 301i32 | 335i32 => {
                    if ((F::BtlAnim_Advance(c, p_chr_1, 0i32)) != 0) {
                        { let v_ = 1i32; let a_ = ((p_chr_1).wrapping_add(976)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    }
                    break 's1;
                }
                _ => {
                }
            }
        }
        if ((((!((((((((F::BtlSuper_GetFlags(c, p_chr_1, l_cls_5)) as u64)) as i32)) & (4096i32))) != 0)) as i32)) != 0) {
            { let a1_ = (F::BtlAnim_GetId(c, p_chr_1)).wrapping_add(7i32); let a2_ = (F::BtlAnim_GetId(c, p_chr_1)).wrapping_add(14i32); F::BtlAct_SetPitchMotion(c, p_chr_1, a1_, a2_, l_aim_7) };
        }
        if ((l_aim_7) != 0) {
            F::BtlMove_Step(c, p_chr_1, 2i32, 5i32, 2i32, F32(0x00000000), F32(0x3f6d0979));
            F::BtlMove_ApplyGravity(c, p_chr_1);
        } else {
            F::BtlMove_Step(c, p_chr_1, 6i32, 5i32, 2i32, F32(0x00000000), F32(0x3f6d0979));
            F::BtlMove_ApplyGravity(c, p_chr_1);
        }
        F::BtlChar_SetFxBit(c, p_chr_1, 16i32);
        F::BtlSuper_SetFiringFlags(c, p_chr_1);
        F::BtlChar_SetFlag(c, p_chr_1, ((((163i32) as i64 as u64)) as u32));
    }
    if (((((p_phase_2) == (2i32)) as i32)) != 0) {
        's2: {
            let sv10: i32 = F::BtlAnim_GetId(c, p_chr_1);
            match sv10 {
                261i32 | 295i32 | 329i32 => {
                    break 's2;
                }
                262i32 | 296i32 | 330i32 => {
                    break 's2;
                }
                263i32 | 297i32 | 331i32 => {
                    if ((((!((F::btl_act_f__BtlSuper_IsLastShot(c, p_chr_1, l_cls_5, l_shots_4)) != 0)) as i32)) != 0) {
                        F::BtlSuper_SetClass(c, p_chr_1, l_cls_5);
                    }
                    break 's2;
                }
                264i32 | 298i32 | 332i32 => {
                    { let a_ = l_timer_3; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    if ((((((c.m.r32(l_timer_3) as i32)) >= ((rt::f2i(rt::fmul(F::BtlSuper_GetShotTime(c, p_chr_1, l_cls_5), F32(0x41f00000)))).wrapping_sub(1i32))) as i32)) != 0) {
                        l_stop_8 = 1i32;
                    } else {
                        if ((((({ let a_ = F::BtlMember_GetActiveGauge(c, p_chr_1); (c.m.r32(a_) as i32) }) == (1i32)) as i32)) != 0) {
                            l_stop_8 = 1i32;
                        } else {
                            if ((((((c.m.r32(l_shots_4) as i32)) < (F::BtlSuper_GetShots(c, p_chr_1, l_cls_5))) as i32)) != 0) {
                                l_stop_8 = 0i32;
                            } else {
                                l_stop_8 = 1i32;
                            }
                        }
                    }
                    if ((l_stop_8) != 0) {
                        if (((((F::BtlMove_CountOwnBeams(c, p_chr_1)) <= (0i32)) as i32)) != 0) {
                            F::BtlAnim_Request(c, p_chr_1, (((l_cls_5).wrapping_mul(34i32)).wrapping_add(193i32)).wrapping_add(6i32), F32(0x00000000));
                            { let v_ = 0i32; let a_ = l_timer_3; c.m.w32(a_, (v_) as u32); };
                        }
                    } else {
                        F::BtlSuper_SetClass(c, p_chr_1, l_cls_5);
                        if ((((((c.m.r32(l_timer_3) as i32)) >= (16i32)) as i32)) != 0) {
                            if ((F::BtlInput_TestAction(c, p_chr_1, 111i32, 1i32)) != 0) {
                                F::BtlAnim_Request(c, p_chr_1, (((l_cls_5).wrapping_mul(34i32)).wrapping_add(193i32)).wrapping_add(2i32), F32(0x00000000));
                            }
                        }
                    }
                    break 's2;
                }
                267i32 | 301i32 | 335i32 => {
                    break 's2;
                }
                _ => {
                }
            }
        }
        if (((c.m.r32(((p_chr_1).wrapping_add(976)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) != 0) {
            F::BtlSuper_Finish(c, p_chr_1, l_cls_5);
        }
    }
    if (((((p_phase_2) == (3i32)) as i32)) != 0) {
        F::BtlSuper_Leave(c, p_chr_1, l_cls_5);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_SuperRepeatHandler", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_SuperQuickBeamHandler(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_SuperQuickBeamHandler", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let mut l_timer_3: u32 = 0;
    let mut l_cls_4: i32 = 0;
    let mut l_obj_5: u32 = 0;
    let mut l_aim_6: i32 = 0;
    let r_ = 'f: {
    l_timer_3 = ((p_chr_1).wrapping_add(976)).wrapping_add((((2i32) as u32)).wrapping_mul(4));
    l_cls_4 = F::BtlAct_GetCurrentClass(c, p_chr_1);
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        F::BtlAnim_Play(c, p_chr_1, (((l_cls_4).wrapping_mul(34i32)).wrapping_add(193i32)).wrapping_add(0i32), F32(0x00000000));
        F::BtlSuper_Begin(c, p_chr_1, l_cls_4, 1i32);
        F::BtlSuper_SetStartFx(c, p_chr_1, l_cls_4);
        F::BtlChar_SetHeldFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32));
        if (((((F::BtlAct_GetPrev(c, p_chr_1)) == (46i32)) as i32)) != 0) {
            F::BtlChar_SetFlag(c, p_chr_1, ((((205i32) as i64 as u64)) as u32));
        }
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        l_obj_5 = F::BtlChar_GetObj(c, p_chr_1);
        l_aim_6 = 0i32;
        if ((((((((F::BtlSuper_GetFlagsA(c, p_chr_1, l_cls_4)) as u64)) as i32)) & (2048i32))) != 0) {
            l_aim_6 = 1i32;
        }
        's1: {
            let sv7: i32 = F::BtlAnim_GetId(c, p_chr_1);
            match sv7 {
                261i32 | 295i32 | 329i32 => {
                    F::BtlAnim_SetDuration(c, p_chr_1, F32(0x3dcccccc));
                    l_aim_6 = 1i32;
                    { let _ = F::BtlAnim_AdvanceThen(c, p_chr_1, (((l_cls_4).wrapping_mul(34i32)).wrapping_add(193i32)).wrapping_add(2i32), F32(0x00000000), 0i32); };
                    { let v_ = F32(0x00000000); let a_ = (p_chr_1).wrapping_add(3652); c.m.w32(a_, (v_).0); };
                    break 's1;
                }
                263i32 | 297i32 | 331i32 => {
                    F::BtlSuper_FitAnimToEvent(c, p_chr_1, F32(0x3dcccccc));
                    { let _ = F::BtlAnim_AdvanceThen(c, p_chr_1, (((l_cls_4).wrapping_mul(34i32)).wrapping_add(193i32)).wrapping_add(3i32), F32(0x00000000), 0i32); };
                    F::BtlSuper_ShakeOnEvent(c, p_chr_1, l_cls_4, 0i32);
                    F::BtlSuper_Recoil(c, p_chr_1, l_cls_4, 0i32);
                    if (((rt::flt(F::BtlAnim_GetFrame(c, p_chr_1), rt::i2f(F::BtlObjAnim_QueryEvent(c, l_obj_5, ((1024i32) as u64), 0i32, ((((0i32) as i64 as u64)) as u32)))) as i32)) != 0) {
                        l_aim_6 = 1i32;
                    }
                    break 's1;
                }
                264i32 | 298i32 | 332i32 => {
                    F::BtlAnim_AdvanceLoop(c, p_chr_1, 0i32);
                    F::BtlSuper_ShakeOnEvent(c, p_chr_1, l_cls_4, 1i32);
                    F::BtlSuper_Recoil(c, p_chr_1, l_cls_4, 1i32);
                    if ((((({ let a_ = l_timer_3; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); n_ }) > (150i32)) as i32)) != 0) {
                        F::BtlChar_SetHeldFlag(c, p_chr_1, ((((168i32) as i64 as u64)) as u32));
                    }
                    if ((F::BtlChar_TestFlag(c, p_chr_1, ((((168i32) as i64 as u64)) as u32))) != 0) {
                        F::BtlAnim_Request(c, p_chr_1, (((l_cls_4).wrapping_mul(34i32)).wrapping_add(193i32)).wrapping_add(6i32), F32(0x00000000));
                        { let v_ = 0i32; let a_ = l_timer_3; c.m.w32(a_, (v_) as u32); };
                    }
                    break 's1;
                }
                267i32 | 301i32 | 335i32 => {
                    if ((F::BtlAnim_Advance(c, p_chr_1, 0i32)) != 0) {
                        { let v_ = 1i32; let a_ = ((p_chr_1).wrapping_add(976)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    }
                    break 's1;
                }
                _ => {
                }
            }
        }
        if ((((!((((((((F::BtlSuper_GetFlags(c, p_chr_1, l_cls_4)) as u64)) as i32)) & (4096i32))) != 0)) as i32)) != 0) {
            { let a1_ = (F::BtlAnim_GetId(c, p_chr_1)).wrapping_add(7i32); let a2_ = (F::BtlAnim_GetId(c, p_chr_1)).wrapping_add(14i32); F::BtlAct_SetPitchMotion(c, p_chr_1, a1_, a2_, l_aim_6) };
        }
        if ((l_aim_6) != 0) {
            F::BtlMove_Step(c, p_chr_1, 2i32, 5i32, 2i32, F32(0x00000000), F32(0x3f6d0979));
            F::BtlMove_ApplyGravity(c, p_chr_1);
        } else {
            F::BtlMove_Step(c, p_chr_1, 6i32, 5i32, 2i32, F32(0x00000000), F32(0x3f6d0979));
            F::BtlMove_ApplyGravity(c, p_chr_1);
        }
        F::BtlChar_SetFxBit(c, p_chr_1, 16i32);
        F::BtlChar_SetFlag(c, p_chr_1, ((((213i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((164i32) as i64 as u64)) as u32));
    }
    if (((((p_phase_2) == (2i32)) as i32)) != 0) {
        if (((c.m.r32(((p_chr_1).wrapping_add(976)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) != 0) {
            F::BtlSuper_Finish(c, p_chr_1, l_cls_4);
        }
    }
    if (((((p_phase_2) == (3i32)) as i32)) != 0) {
        F::BtlSuper_Leave(c, p_chr_1, l_cls_4);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_SuperQuickBeamHandler", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_SuperQuickLongBeamHandler(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_SuperQuickLongBeamHandler", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let mut l_timer_3: u32 = 0;
    let mut l_cls_4: i32 = 0;
    let mut l_aim_5: i32 = 0;
    let r_ = 'f: {
    l_timer_3 = ((p_chr_1).wrapping_add(976)).wrapping_add((((2i32) as u32)).wrapping_mul(4));
    l_cls_4 = F::BtlAct_GetCurrentClass(c, p_chr_1);
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        F::BtlAnim_Play(c, p_chr_1, (((l_cls_4).wrapping_mul(34i32)).wrapping_add(193i32)).wrapping_add(0i32), F32(0x00000000));
        F::BtlSuper_Begin(c, p_chr_1, l_cls_4, 0i32);
        F::BtlSuper_SetStartFx(c, p_chr_1, l_cls_4);
        F::BtlChar_SetHeldFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32));
        if (((((F::BtlAct_GetPrev(c, p_chr_1)) == (46i32)) as i32)) != 0) {
            F::BtlChar_SetFlag(c, p_chr_1, ((((205i32) as i64 as u64)) as u32));
        }
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        l_aim_5 = 0i32;
        if ((((((((F::BtlSuper_GetFlagsA(c, p_chr_1, l_cls_4)) as u64)) as i32)) & (2048i32))) != 0) {
            l_aim_5 = 1i32;
        }
        's1: {
            let sv6: i32 = F::BtlAnim_GetId(c, p_chr_1);
            match sv6 {
                261i32 | 295i32 | 329i32 => {
                    F::BtlAnim_SetDuration(c, p_chr_1, F32(0x3dcccccc));
                    l_aim_5 = 1i32;
                    { let _ = F::BtlAnim_AdvanceThen(c, p_chr_1, (((l_cls_4).wrapping_mul(34i32)).wrapping_add(193i32)).wrapping_add(2i32), F32(0x00000000), 0i32); };
                    { let v_ = F32(0x00000000); let a_ = (p_chr_1).wrapping_add(3652); c.m.w32(a_, (v_).0); };
                    break 's1;
                }
                263i32 | 297i32 | 331i32 => {
                    F::BtlSuper_FitAnimToEvent(c, p_chr_1, F32(0x3dcccccc));
                    { let _ = F::BtlAnim_AdvanceThen(c, p_chr_1, (((l_cls_4).wrapping_mul(34i32)).wrapping_add(193i32)).wrapping_add(4i32), F32(0x00000000), 0i32); };
                    F::BtlSuper_ShakeOnEvent(c, p_chr_1, l_cls_4, 0i32);
                    F::BtlSuper_Recoil(c, p_chr_1, l_cls_4, 0i32);
                    if ((F::BtlAnim_IsNew(c, p_chr_1)) != 0) {
                        F::ChrCam_EndCut(c, p_chr_1);
                    }
                    break 's1;
                }
                265i32 | 299i32 | 333i32 => {
                    F::BtlSuper_FitAnimToEvent(c, p_chr_1, F32(0x3dcccccc));
                    { let _ = F::BtlAnim_AdvanceThen(c, p_chr_1, (((l_cls_4).wrapping_mul(34i32)).wrapping_add(193i32)).wrapping_add(5i32), F32(0x00000000), 0i32); };
                    break 's1;
                }
                266i32 | 300i32 | 334i32 => {
                    F::BtlAnim_AdvanceLoop(c, p_chr_1, 0i32);
                    if ((((({ let a_ = l_timer_3; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); n_ }) > (150i32)) as i32)) != 0) {
                        F::BtlChar_SetHeldFlag(c, p_chr_1, ((((169i32) as i64 as u64)) as u32));
                    }
                    if ((F::BtlChar_TestFlag(c, p_chr_1, ((((169i32) as i64 as u64)) as u32))) != 0) {
                        F::BtlAnim_Request(c, p_chr_1, (((l_cls_4).wrapping_mul(34i32)).wrapping_add(193i32)).wrapping_add(6i32), F32(0x00000000));
                        { let v_ = 0i32; let a_ = l_timer_3; c.m.w32(a_, (v_) as u32); };
                    }
                    break 's1;
                }
                267i32 | 301i32 | 335i32 => {
                    if ((F::BtlAnim_Advance(c, p_chr_1, 0i32)) != 0) {
                        { let v_ = 1i32; let a_ = ((p_chr_1).wrapping_add(976)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    }
                    break 's1;
                }
                _ => {
                }
            }
        }
        if ((((!((((((((F::BtlSuper_GetFlags(c, p_chr_1, l_cls_4)) as u64)) as i32)) & (4096i32))) != 0)) as i32)) != 0) {
            { let a1_ = (F::BtlAnim_GetId(c, p_chr_1)).wrapping_add(7i32); let a2_ = (F::BtlAnim_GetId(c, p_chr_1)).wrapping_add(14i32); F::BtlAct_SetPitchMotion(c, p_chr_1, a1_, a2_, l_aim_5) };
        }
        if ((l_aim_5) != 0) {
            F::BtlMove_Step(c, p_chr_1, 2i32, 5i32, 2i32, F32(0x00000000), F32(0x3f6d0979));
            F::BtlMove_ApplyGravity(c, p_chr_1);
        } else {
            F::BtlMove_Step(c, p_chr_1, 6i32, 5i32, 2i32, F32(0x00000000), F32(0x3f6d0979));
            F::BtlMove_ApplyGravity(c, p_chr_1);
        }
        F::BtlChar_SetFxBit(c, p_chr_1, 16i32);
        F::BtlChar_SetFlag(c, p_chr_1, ((((213i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((164i32) as i64 as u64)) as u32));
    }
    if (((((p_phase_2) == (2i32)) as i32)) != 0) {
        if (((c.m.r32(((p_chr_1).wrapping_add(976)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) != 0) {
            F::BtlSuper_Finish(c, p_chr_1, l_cls_4);
        }
    }
    if (((((p_phase_2) == (3i32)) as i32)) != 0) {
        F::BtlSuper_Leave(c, p_chr_1, l_cls_4);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_SuperQuickLongBeamHandler", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_SuperRushDashHandler(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_SuperRushDashHandler", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let mut l_still_3: u32 = 0;
    let mut l_frames_4: u32 = 0;
    let mut l_slot_5: i32 = 0;
    let mut l_pose_6: u32 = 0;
    let mut l_moving_7: i32 = 0;
    let mut l_lean_8: F32 = F32(0);
    let mut l_blend_9: F32 = F32(0);
    let mut l_speed_10: F32 = F32(0);
    let mut l_yawAccel_11: F32 = F32(0);
    let mut l_pitchAccel_12: F32 = F32(0);
    let mut l_yawMax_13: F32 = F32(0);
    let mut l_pitchMax_14: F32 = F32(0);
    let mut l_ratio_15: F32 = F32(0);
    let mut l_pose_16: u32 = 0;
    let mut l_stop_17: i32 = 0;
    let r_ = 'f: {
    l_still_3 = ((p_chr_1).wrapping_add(976)).wrapping_add((((2i32) as u32)).wrapping_mul(4));
    l_frames_4 = ((p_chr_1).wrapping_add(976)).wrapping_add((((3i32) as u32)).wrapping_mul(4));
    l_slot_5 = ((D::call(c, fid::BtlAct_GetCurrentClass, &[((p_chr_1) as u64)])) as i32);
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        { let _ = D::call(c, fid::BtlAnim_Play, &[((p_chr_1) as u64), (((((l_slot_5).wrapping_mul(34i32)).wrapping_add(193i32)).wrapping_add(0i32)) as i64 as u64), ((F32(0x3e199999)).0 as u64)]); };
        { let _ = D::call(c, fid::BtlMove_TurnYaw, &[((p_chr_1) as u64), ((2i32) as i64 as u64), ((F32(0x40490fda)).0 as u64)]); };
        { let _ = D::call(c, fid::BtlMove_TurnPitch, &[((p_chr_1) as u64), ((1i32) as i64 as u64), ((F32(0x40490fda)).0 as u64)]); };
        { let _ = D::call(c, fid::BtlMove_TurnModelYaw, &[((p_chr_1) as u64), ((F32(0x40490fda)).0 as u64), ((F32(0x3f800000)).0 as u64)]); };
        { let _ = D::call(c, fid::BtlSuper_Begin, &[((p_chr_1) as u64), ((l_slot_5) as i64 as u64), ((0i32) as i64 as u64)]); };
        { let _ = D::call(c, fid::BtlSuper_PlayStartSound, &[((p_chr_1) as u64), ((l_slot_5) as i64 as u64), ((0i32) as i64 as u64)]); };
        { let _ = D::call(c, fid::BtlSuper_SetStartFx, &[((p_chr_1) as u64), ((l_slot_5) as i64 as u64)]); };
        { let _ = D::call(c, fid::BtlSuper_SetClassFlag, &[((p_chr_1) as u64), ((l_slot_5) as i64 as u64), ((0i32) as i64 as u64)]); };
        { let _ = D::call(c, fid::BtlSuper_RequestCut, &[((p_chr_1) as u64), ((l_slot_5) as i64 as u64)]); };
        { let a1_ = rt::div_i32(((D::call(c, fid::BtlSuper_GetKiCost, &[((p_chr_1) as u64), ((l_slot_5) as i64 as u64)])) as i32), 2i32); { let _ = D::call(c, fid::BtlMember_SpendKi, &[((p_chr_1) as u64), ((a1_) as i64 as u64), ((1i32) as i64 as u64)]); } };
        { let _ = D::call(c, fid::BtlChar_SetHeldFlag, &[((p_chr_1) as u64), ((14i32) as i64 as u64)]); };
        { let _ = D::call(c, fid::BtlChar_ResetLook, &[((p_chr_1) as u64)]); };
        's1: {
            let sv18: i32 = F::BtlAct_GetCurrent(c, p_chr_1);
            match sv18 {
                286i32 | 287i32 | 288i32 => {
                    { let r_ = 1i32; let a_ = ((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    break 's1;
                }
                _ => {
                }
            }
        }
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        l_pose_6 = ((D::call(c, fid::BtlChar_GetPos, &[((p_chr_1) as u64)])) as u32);
        l_moving_7 = 0i32;
        l_lean_8 = F32(0x00000000);
        's2: {
            let sv19: i32 = ((D::call(c, fid::BtlAnim_GetId, &[((p_chr_1) as u64)])) as i32);
            match sv19 {
                261i32 | 295i32 | 329i32 => {
                    { let _ = ((D::call(c, fid::BtlAnim_AdvanceThen, &[((p_chr_1) as u64), (((((l_slot_5).wrapping_mul(34i32)).wrapping_add(193i32)).wrapping_add(1i32)) as i64 as u64), ((F32(0x00000000)).0 as u64), ((0i32) as i64 as u64)])) as i32); };
                    { let _ = D::call(c, fid::BtlSuper_RequestHitStop, &[((p_chr_1) as u64), ((0i32) as i64 as u64)]); };
                    l_lean_8 = F32(0x00000000);
                    { let _ = D::call(c, fid::BtlSuper_SetClass, &[((p_chr_1) as u64), ((l_slot_5) as i64 as u64)]); };
                    break 's2;
                }
                262i32 | 296i32 | 330i32 => {
                    { let _ = ((D::call(c, fid::BtlAnim_AdvanceThen, &[((p_chr_1) as u64), (((((l_slot_5).wrapping_mul(34i32)).wrapping_add(193i32)).wrapping_add(2i32)) as i64 as u64), ((F32(0x00000000)).0 as u64), ((0i32) as i64 as u64)])) as i32); };
                    { let _ = D::call(c, fid::BtlSuper_RequestHitStop, &[((p_chr_1) as u64), ((0i32) as i64 as u64)]); };
                    l_lean_8 = F32(0x00000000);
                    { let _ = D::call(c, fid::BtlSuper_SetClass, &[((p_chr_1) as u64), ((l_slot_5) as i64 as u64)]); };
                    break 's2;
                }
                263i32 | 297i32 | 331i32 => {
                    { let _ = ((D::call(c, fid::BtlAnim_AdvanceThen, &[((p_chr_1) as u64), (((((l_slot_5).wrapping_mul(34i32)).wrapping_add(193i32)).wrapping_add(3i32)) as i64 as u64), ((F32(0x00000000)).0 as u64), ((0i32) as i64 as u64)])) as i32); };
                    { let _ = D::call(c, fid::BtlSuper_SetClass, &[((p_chr_1) as u64), ((l_slot_5) as i64 as u64)]); };
                    if ((((D::call(c, fid::BtlAnim_IsNew, &[((p_chr_1) as u64)])) as i32)) != 0) {
                        { let _ = D::call(c, fid::BtlChar_Vibrate, &[((p_chr_1) as u64), ((F32(0x3f4ccccc)).0 as u64), ((F32(0x3e999999)).0 as u64)]); };
                        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((208i32) as i64 as u64)]); };
                        { let _ = D::call(c, fid::ChrCam_EndCut, &[((p_chr_1) as u64)]); };
                    }
                    { let _ = D::call(c, fid::BtlChar_SetFxBit, &[((p_chr_1) as u64), ((4i32) as i64 as u64)]); };
                    { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((71i32) as i64 as u64)]); };
                    l_moving_7 = 1i32;
                    if (((((((((((((c.m.r32(((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) & (1i32))) != 0) && ((((!((((D::call(c, fid::BtlAnim_IsNew, &[((p_chr_1) as u64)])) as i32)) != 0)) as i32)) != 0)) as i32)) != 0) && ((((!((F::BtlAnim_WasNew(c, p_chr_1)) != 0)) as i32)) != 0)) as i32)) != 0) {
                        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((73i32) as i64 as u64)]); };
                    }
                    l_lean_8 = F::BtlAnim_GetProgress(c, p_chr_1);
                    break 's2;
                }
                264i32 | 298i32 | 332i32 => {
                    l_moving_7 = 1i32;
                    { let _ = D::call(c, fid::BtlAnim_AdvanceLoop, &[((p_chr_1) as u64), ((0i32) as i64 as u64)]); };
                    l_lean_8 = F32(0x3f800000);
                    { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((73i32) as i64 as u64)]); };
                    { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((71i32) as i64 as u64)]); };
                    { let _ = D::call(c, fid::BtlChar_SetFxBit, &[((p_chr_1) as u64), ((4i32) as i64 as u64)]); };
                    F::BtlChar_SetSmallVibration(c, p_chr_1, F32(0x3dcccccc));
                    break 's2;
                }
                267i32 | 301i32 | 335i32 => {
                    if ((((D::call(c, fid::BtlAnim_Advance, &[((p_chr_1) as u64), ((0i32) as i64 as u64)])) as i32)) != 0) {
                        { let v_ = 1i32; let a_ = ((p_chr_1).wrapping_add(976)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    }
                    l_lean_8 = rt::fsub(F32(0x3f800000), F::BtlAnim_GetProgress(c, p_chr_1));
                    break 's2;
                }
                _ => {
                }
            }
        }
        if ((l_moving_7) != 0) {
            l_speed_10 = F::BtlSuper_GetRushSpeed(c, p_chr_1, l_slot_5);
            if (((((c.m.r32(((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) & (1i32))) != 0) {
                { let _ = D::call(c, fid::BtlMove_Step, &[((p_chr_1) as u64), ((6i32) as i64 as u64), ((6i32) as i64 as u64), ((3i32) as i64 as u64), ((l_speed_10).0 as u64), ((F32(0x461c4000)).0 as u64)]); };
            } else {
                l_yawAccel_11 = F::BtlMoveParam_GetTurnAccel(c, p_chr_1, 2i32);
                l_pitchAccel_12 = F::BtlMoveParam_GetTurnAccel(c, p_chr_1, 3i32);
                l_yawMax_13 = F::BtlMoveParam_GetTurnMax(c, p_chr_1, 2i32);
                l_pitchMax_14 = F::BtlMoveParam_GetTurnMax(c, p_chr_1, 3i32);
                l_ratio_15 = { let a0_ = rt::fmul(F::BtlOpp_GetGapXZ(c, p_chr_1), F32(0x3c23d70a)); F::BtlUtil_MinF(c, a0_, F32(0x3f800000)) };
                F::BtlMove_SteerAtOpponent(c, p_chr_1, l_speed_10, l_yawAccel_11, l_pitchAccel_12, rt::fmul(l_yawMax_13, l_ratio_15), rt::fmul(l_pitchMax_14, l_ratio_15), F32(0x40490fda));
                { let _ = D::call(c, fid::BtlMove_TurnModelYaw, &[((p_chr_1) as u64), ((F32(0x3f20d97a)).0 as u64), ((F32(0x3f000000)).0 as u64)]); };
                { let _ = D::call(c, fid::BtlMove_SetDirection, &[((p_chr_1) as u64), ((3i32) as i64 as u64)]); };
                F::BtlMove_Advance(c, p_chr_1, l_speed_10, F32(0x461c4000));
                { let _ = D::call(c, fid::BtlMove_ApplyGravity, &[((p_chr_1) as u64)]); };
            }
            { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((9i32) as i64 as u64)]); };
        } else {
            { let _ = D::call(c, fid::BtlMove_Step, &[((p_chr_1) as u64), ((6i32) as i64 as u64), ((6i32) as i64 as u64), ((7i32) as i64 as u64), ((F32(0x00000000)).0 as u64), ((F32(0x3fed0979)).0 as u64)]); };
        }
        { let _ = D::call(c, fid::BtlMove_ApplyGravity, &[((p_chr_1) as u64)]); };
        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((187i32) as i64 as u64)]); };
        { let _ = D::call(c, fid::BtlChar_SetFxBit, &[((p_chr_1) as u64), ((16i32) as i64 as u64)]); };
        { let _ = D::call(c, fid::BtlChar_SetFxBit, &[((p_chr_1) as u64), ((58i32) as i64 as u64)]); };
        if (((((c.m.r32(((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) & (1i32))) != 0) {
            { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((26i32) as i64 as u64)]); };
        }
        if ((((((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((5i32) as i64 as u64)])) as i32)) != 0) && (((((((D::call(c, fid::BtlSuper_GetFlags, &[((p_chr_1) as u64), ((l_slot_5) as i64 as u64)])) as i32)) >= (0i32)) as i32)) != 0)) as i32)) != 0) {
            { let a1_ = rt::fmul({ let a0_ = F32(c.m.r32((l_pose_6).wrapping_add(144))); F::BtlUtil_ClampF(c, a0_, F32(0xbf8cbe4b), F32(0x3f8cbe4b)) }, l_lean_8); F::BtlMove_SetLeanX(c, p_chr_1, a1_) };
        }
        { let a2_ = (c.m.r32((p_chr_1).wrapping_add(2404)) as i32); { let _ = D::call(c, fid::BtlSuper_MeasureRushStep, &[((p_chr_1) as u64), ((l_slot_5) as i64 as u64), ((a2_) as i64 as u64)]); } };
        if ((((D::call(c, fid::BtlInput_TestAction, &[((p_chr_1) as u64), ((111i32) as i64 as u64), ((1i32) as i64 as u64)])) as i32)) != 0) {
            { let _ = D::call(c, fid::BtlChar_SetHeldFlag, &[((p_chr_1) as u64), ((158i32) as i64 as u64)]); };
            { let _ = D::call(c, fid::BtlChar_SetHeldFlag, &[((p_chr_1) as u64), ((157i32) as i64 as u64)]); };
            { let _ = D::call(c, fid::BtlChar_SetHeldFlag, &[((p_chr_1) as u64), ((156i32) as i64 as u64)]); };
            if ((((D::call(c, fid::BtlChar_IsFlagRaised, &[((p_chr_1) as u64), ((156i32) as i64 as u64)])) as i32)) != 0) {
                { let _ = D::call(c, fid::BtlCharSnd_PlayCommon, &[((p_chr_1) as u64), ((20i32) as i64 as u64)]); };
                { let _ = D::call(c, fid::BtlCharSnd_PlayCommon, &[((p_chr_1) as u64), ((21i32) as i64 as u64)]); };
                { let _ = D::call(c, fid::BtlCharSnd_PlayCommon, &[((p_chr_1) as u64), ((28i32) as i64 as u64)]); };
            }
        }
    }
    if (((((p_phase_2) == (2i32)) as i32)) != 0) {
        l_pose_16 = ((D::call(c, fid::BtlChar_GetPos, &[((p_chr_1) as u64)])) as u32);
        's3: {
            let sv20: i32 = ((D::call(c, fid::BtlAnim_GetId, &[((p_chr_1) as u64)])) as i32);
            match sv20 {
                261i32 | 295i32 | 329i32 | 262i32 | 296i32 | 330i32 | 267i32 | 301i32 | 335i32 => {
                    break 's3;
                }
                263i32 | 297i32 | 331i32 => {
                    if (((((((((c.m.r32(((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) & (1i32))) != 0) && ((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((99i32) as i64 as u64)])) as i32)) != 0)) as i32)) != 0) {
                        { let _ = D::call(c, fid::BtlAnim_Request, &[((p_chr_1) as u64), (((((l_slot_5).wrapping_mul(34i32)).wrapping_add(193i32)).wrapping_add(6i32)) as i64 as u64), ((F32(0x3e199999)).0 as u64)]); };
                        { let v_ = F32(0x00000000); let a_ = (l_pose_16).wrapping_add(152); c.m.w32(a_, (v_).0); };
                    }
                    break 's3;
                }
                264i32 | 298i32 | 332i32 => {
                    {
                        l_stop_17 = 0i32;
                        if (((rt::flt(F::Vec3_Length(c, (l_pose_16).wrapping_add(64)), F32(0x3f6d0979)) as i32)) != 0) {
                            if ((((({ let a_ = l_still_3; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); n_ }) >= (16i32)) as i32)) != 0) {
                                { let v_ = F32(0x00000000); let a_ = (l_pose_16).wrapping_add(152); c.m.w32(a_, (v_).0); };
                                l_stop_17 = 1i32;
                            }
                        } else {
                            { let v_ = 0i32; let a_ = l_still_3; c.m.w32(a_, (v_) as u32); };
                        }
                        if (((rt::fgt(rt::i2f({ let a_ = l_frames_4; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); n_ }), rt::fmul(F32((D::call(c, fid::BtlSuper_GetShotTime, &[((p_chr_1) as u64), ((l_slot_5) as i64 as u64)])) as u32), F32(0x41f00000))) as i32)) != 0) {
                            { let v_ = F32(0x00000000); let a_ = (l_pose_16).wrapping_add(152); c.m.w32(a_, (v_).0); };
                            l_stop_17 = 1i32;
                        }
                        if ((((!((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((5i32) as i64 as u64)])) as i32)) != 0)) as i32)) != 0) {
                            l_stop_17 = 1i32;
                        }
                        if ((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((99i32) as i64 as u64)])) as i32)) != 0) {
                            { let v_ = F32(0x00000000); let a_ = (l_pose_16).wrapping_add(152); c.m.w32(a_, (v_).0); };
                            l_stop_17 = 1i32;
                        }
                        if ((l_stop_17) != 0) {
                            { let _ = D::call(c, fid::BtlAnim_Request, &[((p_chr_1) as u64), (((((l_slot_5).wrapping_mul(34i32)).wrapping_add(193i32)).wrapping_add(6i32)) as i64 as u64), ((F32(0x3e199999)).0 as u64)]); };
                        }
                        break 's3;
                    }
                }
                _ => {
                }
            }
        }
        if ((((((c.m.r32(((p_chr_1).wrapping_add(976)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) != (0i32)) as i32)) != 0) {
            { let _ = D::call(c, fid::BtlAct_Request, &[((p_chr_1) as u64), ((11i32) as i64 as u64)]); };
        }
        if ((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((100i32) as i64 as u64)])) as i32)) != 0) {
            { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((54i32) as i64 as u64)]); };
            { let _ = D::call(c, fid::BtlAct_Request, &[((p_chr_1) as u64), ((207i32) as i64 as u64)]); };
        }
        if ((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((114i32) as i64 as u64)])) as i32)) != 0) {
            { let _ = D::call(c, fid::BtlChar_SetFxBit, &[((p_chr_1) as u64), ((5i32) as i64 as u64)]); };
            { let _ = D::call(c, fid::BtlAct_Request, &[((p_chr_1) as u64), (((l_slot_5).wrapping_add(299i32)) as i64 as u64)]); };
        }
    }
    if (((((p_phase_2) == (3i32)) as i32)) != 0) {
        { let _ = D::call(c, fid::BtlSuper_Leave, &[((p_chr_1) as u64), ((l_slot_5) as i64 as u64)]); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_SuperRushDashHandler", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_SuperRushStrikeHandler(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_SuperRushStrikeHandler", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let mut l_slot_3: i32 = 0;
    let mut l_moving_4: i32 = 0;
    let r_ = 'f: {
    l_slot_3 = ((D::call(c, fid::BtlAct_GetCurrentClass, &[((p_chr_1) as u64)])) as i32);
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        { let _ = D::call(c, fid::BtlAnim_Play, &[((p_chr_1) as u64), (((((l_slot_3).wrapping_mul(34i32)).wrapping_add(193i32)).wrapping_add(0i32)) as i64 as u64), ((F32(0x3e199999)).0 as u64)]); };
        { let _ = D::call(c, fid::BtlMove_TurnYaw, &[((p_chr_1) as u64), ((2i32) as i64 as u64), ((F32(0x40490fda)).0 as u64)]); };
        { let _ = D::call(c, fid::BtlMove_TurnPitch, &[((p_chr_1) as u64), ((1i32) as i64 as u64), ((F32(0x40490fda)).0 as u64)]); };
        { let _ = D::call(c, fid::BtlMove_TurnModelYaw, &[((p_chr_1) as u64), ((F32(0x40490fda)).0 as u64), ((F32(0x3f800000)).0 as u64)]); };
        { let _ = D::call(c, fid::BtlSuper_Begin, &[((p_chr_1) as u64), ((l_slot_3) as i64 as u64), ((0i32) as i64 as u64)]); };
        { let _ = D::call(c, fid::BtlSuper_PlayStartSound, &[((p_chr_1) as u64), ((l_slot_3) as i64 as u64), ((0i32) as i64 as u64)]); };
        { let _ = D::call(c, fid::BtlSuper_SetStartFx, &[((p_chr_1) as u64), ((l_slot_3) as i64 as u64)]); };
        { let _ = D::call(c, fid::BtlSuper_SetClassFlag, &[((p_chr_1) as u64), ((l_slot_3) as i64 as u64), ((0i32) as i64 as u64)]); };
        { let _ = D::call(c, fid::BtlSuper_RequestCut, &[((p_chr_1) as u64), ((l_slot_3) as i64 as u64)]); };
        { let a1_ = rt::div_i32(((D::call(c, fid::BtlSuper_GetKiCost, &[((p_chr_1) as u64), ((l_slot_3) as i64 as u64)])) as i32), 2i32); { let _ = D::call(c, fid::BtlMember_SpendKi, &[((p_chr_1) as u64), ((a1_) as i64 as u64), ((1i32) as i64 as u64)]); } };
        { let _ = D::call(c, fid::BtlChar_SetHeldFlag, &[((p_chr_1) as u64), ((14i32) as i64 as u64)]); };
        { let _ = D::call(c, fid::BtlChar_ResetLook, &[((p_chr_1) as u64)]); };
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        l_moving_4 = 0i32;
        's1: {
            let sv5: i32 = ((D::call(c, fid::BtlAnim_GetId, &[((p_chr_1) as u64)])) as i32);
            match sv5 {
                261i32 | 295i32 | 329i32 => {
                    { let _ = ((D::call(c, fid::BtlAnim_AdvanceThen, &[((p_chr_1) as u64), (((((l_slot_3).wrapping_mul(34i32)).wrapping_add(193i32)).wrapping_add(1i32)) as i64 as u64), ((F32(0x00000000)).0 as u64), ((0i32) as i64 as u64)])) as i32); };
                    { let _ = D::call(c, fid::BtlSuper_RequestHitStop, &[((p_chr_1) as u64), ((0i32) as i64 as u64)]); };
                    { let _ = D::call(c, fid::BtlSuper_SetClass, &[((p_chr_1) as u64), ((l_slot_3) as i64 as u64)]); };
                    break 's1;
                }
                262i32 | 296i32 | 330i32 => {
                    { let _ = ((D::call(c, fid::BtlAnim_AdvanceThen, &[((p_chr_1) as u64), (((((l_slot_3).wrapping_mul(34i32)).wrapping_add(193i32)).wrapping_add(2i32)) as i64 as u64), ((F32(0x00000000)).0 as u64), ((0i32) as i64 as u64)])) as i32); };
                    { let _ = D::call(c, fid::BtlSuper_RequestHitStop, &[((p_chr_1) as u64), ((0i32) as i64 as u64)]); };
                    { let _ = D::call(c, fid::BtlSuper_SetClass, &[((p_chr_1) as u64), ((l_slot_3) as i64 as u64)]); };
                    break 's1;
                }
                263i32 | 297i32 | 331i32 => {
                    { let _ = ((D::call(c, fid::BtlAnim_AdvanceThen, &[((p_chr_1) as u64), (((((l_slot_3).wrapping_mul(34i32)).wrapping_add(193i32)).wrapping_add(6i32)) as i64 as u64), ((F32(0x00000000)).0 as u64), ((0i32) as i64 as u64)])) as i32); };
                    { let _ = D::call(c, fid::BtlSuper_SetClass, &[((p_chr_1) as u64), ((l_slot_3) as i64 as u64)]); };
                    if ((((D::call(c, fid::BtlAnim_IsNew, &[((p_chr_1) as u64)])) as i32)) != 0) {
                        { let _ = D::call(c, fid::ChrCam_EndCut, &[((p_chr_1) as u64)]); };
                    }
                    if (((rt::flt(F32(0x3f000000), F::BtlAnim_GetProgress(c, p_chr_1)) as i32)) != 0) {
                        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((73i32) as i64 as u64)]); };
                    }
                    F::BtlColl_SetFramesToReach(c, p_chr_1, F32(0x3f000000));
                    l_moving_4 = 1i32;
                    break 's1;
                }
                267i32 | 301i32 | 335i32 => {
                    if ((((D::call(c, fid::BtlAnim_Advance, &[((p_chr_1) as u64), ((0i32) as i64 as u64)])) as i32)) != 0) {
                        { let v_ = 1i32; let a_ = ((p_chr_1).wrapping_add(976)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    }
                    break 's1;
                }
                _ => {
                }
            }
        }
        if ((l_moving_4) != 0) {
            { let _ = D::call(c, fid::BtlMove_Step, &[((p_chr_1) as u64), ((6i32) as i64 as u64), ((5i32) as i64 as u64), ((2i32) as i64 as u64), ((F32(0x3f6d0979)).0 as u64), ((F32(0x3f6d0979)).0 as u64)]); };
        } else {
            { let _ = D::call(c, fid::BtlMove_Step, &[((p_chr_1) as u64), ((6i32) as i64 as u64), ((5i32) as i64 as u64), ((2i32) as i64 as u64), ((F32(0x00000000)).0 as u64), ((F32(0x3f6d0979)).0 as u64)]); };
        }
        { let _ = D::call(c, fid::BtlMove_ApplyGravity, &[((p_chr_1) as u64)]); };
        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((187i32) as i64 as u64)]); };
        { let _ = D::call(c, fid::BtlChar_SetFxBit, &[((p_chr_1) as u64), ((16i32) as i64 as u64)]); };
        { let _ = D::call(c, fid::BtlChar_SetFxBit, &[((p_chr_1) as u64), ((58i32) as i64 as u64)]); };
        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((165i32) as i64 as u64)]); };
        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((83i32) as i64 as u64)]); };
        { let a2_ = (c.m.r32((p_chr_1).wrapping_add(2404)) as i32); { let _ = D::call(c, fid::BtlSuper_MeasureRushStep, &[((p_chr_1) as u64), ((l_slot_3) as i64 as u64), ((a2_) as i64 as u64)]); } };
        if ((((D::call(c, fid::BtlInput_TestAction, &[((p_chr_1) as u64), ((111i32) as i64 as u64), ((1i32) as i64 as u64)])) as i32)) != 0) {
            { let _ = D::call(c, fid::BtlChar_SetHeldFlag, &[((p_chr_1) as u64), ((158i32) as i64 as u64)]); };
            { let _ = D::call(c, fid::BtlChar_SetHeldFlag, &[((p_chr_1) as u64), ((157i32) as i64 as u64)]); };
            { let _ = D::call(c, fid::BtlChar_SetHeldFlag, &[((p_chr_1) as u64), ((156i32) as i64 as u64)]); };
            if ((((D::call(c, fid::BtlChar_IsFlagRaised, &[((p_chr_1) as u64), ((156i32) as i64 as u64)])) as i32)) != 0) {
                { let _ = D::call(c, fid::BtlCharSnd_PlayCommon, &[((p_chr_1) as u64), ((20i32) as i64 as u64)]); };
                { let _ = D::call(c, fid::BtlCharSnd_PlayCommon, &[((p_chr_1) as u64), ((21i32) as i64 as u64)]); };
                { let _ = D::call(c, fid::BtlCharSnd_PlayCommon, &[((p_chr_1) as u64), ((28i32) as i64 as u64)]); };
            }
        }
    }
    if (((((p_phase_2) == (2i32)) as i32)) != 0) {
        { let _ = ((D::call(c, fid::BtlAnim_GetId, &[((p_chr_1) as u64)])) as i32); };
        if ((((((c.m.r32(((p_chr_1).wrapping_add(976)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) != (0i32)) as i32)) != 0) {
            { let _ = D::call(c, fid::BtlAct_Request, &[((p_chr_1) as u64), ((11i32) as i64 as u64)]); };
        }
        if ((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((122i32) as i64 as u64)])) as i32)) != 0) {
            { let _ = D::call(c, fid::BtlAct_Request, &[((p_chr_1) as u64), ((202i32) as i64 as u64)]); };
        }
        if ((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((100i32) as i64 as u64)])) as i32)) != 0) {
            { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((54i32) as i64 as u64)]); };
            { let _ = D::call(c, fid::BtlAct_Request, &[((p_chr_1) as u64), ((207i32) as i64 as u64)]); };
        }
        if ((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((114i32) as i64 as u64)])) as i32)) != 0) {
            { let _ = D::call(c, fid::BtlAct_Request, &[((p_chr_1) as u64), (((l_slot_3).wrapping_add(299i32)) as i64 as u64)]); };
        }
    }
    if (((((p_phase_2) == (3i32)) as i32)) != 0) {
        { let _ = D::call(c, fid::BtlSuper_Leave, &[((p_chr_1) as u64), ((l_slot_3) as i64 as u64)]); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_SuperRushStrikeHandler", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_SuperRushSequenceHandler(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_SuperRushSequenceHandler", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let mut l_stepA_3: u32 = 0;
    let mut l_stepB_4: u32 = 0;
    let mut l_slot_5: i32 = 0;
    let mut l_anim_6: i32 = 0;
    let mut l_level_7: i32 = 0;
    let mut l_okA_8: i32 = 0;
    let mut l_okB_9: i32 = 0;
    let mut l_next_10: i32 = 0;
    let mut l_yaw_11: F32 = F32(0);
    let mut l_addYaw_12: F32 = F32(0);
    let mut l_pitch_13: F32 = F32(0);
    let mut l_yaw_14: F32 = F32(0);
    let mut l_pose_15: u32 = 0;
    let r_ = 'f: {
    l_stepA_3 = ((p_chr_1).wrapping_add(976)).wrapping_add((((3i32) as u32)).wrapping_mul(4));
    l_stepB_4 = ((p_chr_1).wrapping_add(976)).wrapping_add((((2i32) as u32)).wrapping_mul(4));
    l_slot_5 = ((D::call(c, fid::BtlAct_GetCurrentClass, &[((p_chr_1) as u64)])) as i32);
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        l_anim_6 = 0i32;
        's1: {
            let sv16: i32 = F::BtlAct_GetCurrent(c, p_chr_1);
            match sv16 {
                301i32 | 302i32 | 303i32 => {
                    l_anim_6 = (((l_slot_5).wrapping_mul(34i32)).wrapping_add(193i32)).wrapping_add(23i32);
                    { let r_ = 1i32; let a_ = ((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    break 's1;
                }
                313i32 | 314i32 | 315i32 => {
                    l_anim_6 = (((l_slot_5).wrapping_mul(34i32)).wrapping_add(193i32)).wrapping_add(28i32);
                    break 's1;
                }
                _ => {
                }
            }
        }
        { let _ = D::call(c, fid::BtlAnim_Play, &[((p_chr_1) as u64), ((l_anim_6) as i64 as u64), ((F32(0x00000000)).0 as u64)]); };
        { let v_ = F32(0x00000000); let a_ = (((D::call(c, fid::BtlChar_GetPos, &[((p_chr_1) as u64)])) as u32)).wrapping_add(152); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (((D::call(c, fid::BtlChar_GetPos, &[((p_chr_1) as u64)])) as u32)).wrapping_add(156); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = ((p_chr_1).wrapping_add(3836)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = ((p_chr_1).wrapping_add(3836)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = ((p_chr_1).wrapping_add(3836)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = ((p_chr_1).wrapping_add(3836)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = ((p_chr_1).wrapping_add(3836)).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        if ((((((c.m.r32((p_chr_1).wrapping_add(3728)) as i32)) == (640i32)) as i32)) != 0) {
            { let r_ = 4i32; let a_ = ((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        }
        if (((((c.m.r32(((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) & (1i32))) != 0) {
            { let _ = D::call(c, fid::BtlSuper_SetupRushDamage, &[((p_chr_1) as u64), ((l_slot_5) as i64 as u64), ((0i32) as i64 as u64)]); };
            's2: {
                let sv17: i32 = ((D::call(c, fid::BtlAct_GetPrev, &[((p_chr_1) as u64)])) as i32);
                match sv17 {
                    283i32 | 284i32 | 285i32 | 286i32 | 287i32 | 288i32 | 289i32 | 290i32 | 291i32 => {
                        { let a1_ = rt::div_i32(((D::call(c, fid::BtlSuper_GetKiCost, &[((p_chr_1) as u64), ((l_slot_5) as i64 as u64)])) as i32), 2i32); { let _ = D::call(c, fid::BtlMember_SpendKi, &[((p_chr_1) as u64), ((a1_) as i64 as u64), ((1i32) as i64 as u64)]); } };
                        break 's2;
                    }
                    _ => {
                    }
                }
            }
            if ((((((c.m.r32(((p_chr_1).wrapping_add(3728)).wrapping_add(32)) as i32)) >= (0i32)) as i32)) != 0) {
                { let a0_ = (c.m.r32(p_chr_1) as i32); let a1_ = (c.m.r32(((p_chr_1).wrapping_add(3728)).wrapping_add(32)) as i32); let a2_ = (c.m.r32(((p_chr_1).wrapping_add(3728)).wrapping_add(36)) as i32); let a3_ = (c.m.r32(((p_chr_1).wrapping_add(3728)).wrapping_add(40)) as i32); let a4_ = (c.m.r32(((p_chr_1).wrapping_add(3728)).wrapping_add(44)) as i32); F::BtlChange_RequestObject(c, a0_, a1_, a2_, a3_, a4_) };
                { let r_ = 2i32; let a_ = ((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            }
        }
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        l_level_7 = { let a1_ = ((D::call(c, fid::BtlAnim_GetId, &[((p_chr_1) as u64)])) as i32); F::BtlAct_GetMotionLevel(c, p_chr_1, a1_) };
        if ((((D::call(c, fid::BtlAnim_IsNew, &[((p_chr_1) as u64)])) as i32)) != 0) {
            if (((((((((l_level_7) != (0i32)) as i32)) != 0) || ((((((c.m.r32(((p_chr_1).wrapping_add(3728)).wrapping_add(80)) as i32)) != (0i32)) as i32)) != 0)) as i32)) != 0) {
                { let a3_ = (((c.m.r32(((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) & (1i32)); F::BtlChar_RequestPlaceRelative(c, p_chr_1, l_slot_5, l_level_7, a3_) };
            }
        }
        if (((((l_level_7) == ((c.m.r32(((p_chr_1).wrapping_add(3728)).wrapping_add(24)) as i32))) as i32)) != 0) {
            if ((((D::call(c, fid::BtlAnim_Advance, &[((p_chr_1) as u64), ((0i32) as i64 as u64)])) as i32)) != 0) {
                if (({ let a0_ = (c.m.r32(((p_chr_1).wrapping_add(3728)).wrapping_add(4)) as i32); F::BtlChange_IsLoadedFor(c, a0_) }) != 0) {
                    if (((((c.m.r32(((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) & (2i32))) != 0) {
                        { let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlChange_SetReady(c, a0_) };
                    }
                    { let a1_ = (((D::call(c, fid::BtlAnim_GetId, &[((p_chr_1) as u64)])) as i32)).wrapping_add(1i32); { let _ = D::call(c, fid::BtlAnim_Request, &[((p_chr_1) as u64), ((a1_) as i64 as u64), ((F32(0x00000000)).0 as u64)]); } };
                }
            }
        } else {
            if (((((c.m.r32(((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) & (4i32))) != 0) {
                if (((((l_level_7) < (((c.m.r32(((p_chr_1).wrapping_add(3728)).wrapping_add(16)) as i32)).wrapping_sub(1i32))) as i32)) != 0) {
                    { let _ = { let a1_ = (((D::call(c, fid::BtlAnim_GetId, &[((p_chr_1) as u64)])) as i32)).wrapping_add(1i32); ((D::call(c, fid::BtlAnim_AdvanceThen, &[((p_chr_1) as u64), ((a1_) as i64 as u64), ((F32(0x00000000)).0 as u64), ((0i32) as i64 as u64)])) as i32) }; };
                } else {
                    's3: {
                        let sv18: i32 = (c.m.r32(l_stepA_3) as i32);
                        match sv18 {
                            0i32 => {
                                if (((((((((c.m.r32(((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) & (1i32))) != 0) && ((((D::call(c, fid::BtlAnim_IsNew, &[((p_chr_1) as u64)])) as i32)) != 0)) as i32)) != 0) {
                                    F::BtlAct_SetFormRandom(c, p_chr_1);
                                    { let a0_ = (c.m.r32(p_chr_1) as i32); let a1_ = (c.m.r32(((p_chr_1).wrapping_add(4812)).wrapping_add(4)) as i32); let a2_ = (c.m.r32(((p_chr_1).wrapping_add(4812)).wrapping_add(8)) as i32); let a3_ = (c.m.r32(((p_chr_1).wrapping_add(4812)).wrapping_add(20)) as i32); let a4_ = (c.m.r32(((p_chr_1).wrapping_add(4812)).wrapping_add(24)) as i32); let a5_ = (c.m.r32(((p_chr_1).wrapping_add(4812)).wrapping_add(28)) as i32); let a6_ = (c.m.r32(((p_chr_1).wrapping_add(4812)).wrapping_add(32)) as i32); F::BtlChange_RequestChara(c, a0_, a1_, a2_, a3_, a4_, a5_, a6_) };
                                }
                                if ((((D::call(c, fid::BtlAnim_Advance, &[((p_chr_1) as u64), ((0i32) as i64 as u64)])) as i32)) != 0) {
                                    { let a_ = l_stepA_3; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                                }
                                break 's3;
                            }
                            1i32 => {
                                if (({ let a0_ = (c.m.r32(((p_chr_1).wrapping_add(3728)).wrapping_add(4)) as i32); F::BtlChange_IsLoadedFor(c, a0_) }) != 0) {
                                    if (((((c.m.r32(((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) & (1i32))) != 0) {
                                        { let a0_ = (c.m.r32(((p_chr_1).wrapping_add(3728)).wrapping_add(4)) as i32); F::BtlChange_SetReady(c, a0_) };
                                        { let a1_ = ((D::call(c, fid::BtlAnim_GetId, &[((p_chr_1) as u64)])) as i32); { let _ = D::call(c, fid::BtlAnim_Request, &[((p_chr_1) as u64), ((a1_) as i64 as u64), ((F32(0x00000000)).0 as u64)]); } };
                                    }
                                    { let a_ = l_stepA_3; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                                }
                                break 's3;
                            }
                            2i32 => {
                                if (((((c.m.r32(((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) & (1i32))) != 0) {
                                    { let a0_ = (c.m.r32(((p_chr_1).wrapping_add(3728)).wrapping_add(4)) as i32); F::BtlChange_SetDone(c, a0_) };
                                    { let a3_ = (((c.m.r32(((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) & (1i32)); F::BtlChar_RequestPlaceRelative(c, p_chr_1, l_slot_5, l_level_7, a3_) };
                                    F::BtlAnim_JumpToEnd(c, p_chr_1);
                                }
                                { let v_ = 1i32; let a_ = ((p_chr_1).wrapping_add(976)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                break 's3;
                            }
                            _ => {
                            }
                        }
                    }
                }
            } else {
                l_okA_8 = 0i32;
                l_okB_9 = 0i32;
                if (((((c.m.r32(((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) & (2i32))) != 0) {
                    if ((((((c.m.r32(((p_chr_1).wrapping_add(3728)).wrapping_add(24)) as i32)) < (l_level_7)) as i32)) != 0) {
                        if ((((D::call(c, fid::BtlAnim_IsNew, &[((p_chr_1) as u64)])) as i32)) != 0) {
                            if (((((l_level_7) == (((c.m.r32(((p_chr_1).wrapping_add(3728)).wrapping_add(24)) as i32)).wrapping_add(1i32))) as i32)) != 0) {
                                { let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlChange_SetDone(c, a0_) };
                                if ((((((c.m.r32((p_chr_1).wrapping_add(3728)) as i32)) == (744i32)) as i32)) != 0) {
                                    F::BtlPartner_LinkToOwner(c, p_chr_1);
                                    if ((((D::call(c, fid::BtlMember_HasAbility, &[((p_chr_1) as u64), ((121i32) as i64 as u64)])) as i32)) != 0) {
                                        F::BtlPartner_SetFlag80(c, p_chr_1, 1i32);
                                    }
                                }
                            }
                            { let a1_ = ((l_level_7).wrapping_sub((c.m.r32(((p_chr_1).wrapping_add(3728)).wrapping_add(24)) as i32))).wrapping_add(413i32); F::BtlPartner_PlayAnim(c, p_chr_1, a1_) };
                        }
                        { let _ = F::BtlPartner_StepAnim(c, p_chr_1); };
                    }
                }
                { let _ = ((D::call(c, fid::BtlAnim_Advance, &[((p_chr_1) as u64), ((0i32) as i64 as u64)])) as i32); };
                if ((((((((((c.m.r32(((p_chr_1).wrapping_add(3728)).wrapping_add(76)) as i32)) != (0i32)) as i32)) != 0) && ((((((c.m.r32(((p_chr_1).wrapping_add(3728)).wrapping_add(28)) as i32)) == (l_level_7)) as i32)) != 0)) as i32)) != 0) {
                    's4: {
                        let sv19: i32 = (c.m.r32(l_stepB_4) as i32);
                        match sv19 {
                            0i32 => {
                                if ((((((((!(((((c.m.r32(((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) & (1i32))) != 0)) as i32)) != 0) && ((((D::call(c, fid::BtlAnim_IsNew, &[((p_chr_1) as u64)])) as i32)) != 0)) as i32)) != 0) {
                                    { let _ = D::call(c, fid::BtlAct_SetFormCurrent, &[((p_chr_1) as u64)]); };
                                    { let a0_ = (c.m.r32(p_chr_1) as i32); let a1_ = (c.m.r32(((p_chr_1).wrapping_add(4812)).wrapping_add(4)) as i32); let a2_ = (c.m.r32(((p_chr_1).wrapping_add(4812)).wrapping_add(8)) as i32); let a3_ = (c.m.r32(((p_chr_1).wrapping_add(4812)).wrapping_add(20)) as i32); let a4_ = (c.m.r32(((p_chr_1).wrapping_add(4812)).wrapping_add(24)) as i32); let a5_ = (c.m.r32(((p_chr_1).wrapping_add(4812)).wrapping_add(28)) as i32); let a6_ = (c.m.r32(((p_chr_1).wrapping_add(4812)).wrapping_add(32)) as i32); F::BtlChange_RequestChara(c, a0_, a1_, a2_, a3_, a4_, a5_, a6_) };
                                }
                                if ((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((49i32) as i64 as u64)])) as i32)) != 0) {
                                    { let a_ = l_stepB_4; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                                }
                                break 's4;
                            }
                            1i32 => {
                                if (({ let a0_ = (c.m.r32(((p_chr_1).wrapping_add(3728)).wrapping_add(8)) as i32); F::BtlChange_IsLoadedFor(c, a0_) }) != 0) {
                                    if ((((!(((((c.m.r32(((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) & (1i32))) != 0)) as i32)) != 0) {
                                        F::BtlChar_ResetUnk1310(c, p_chr_1);
                                        { let a0_ = (c.m.r32(((p_chr_1).wrapping_add(3728)).wrapping_add(8)) as i32); F::BtlChange_SetReady(c, a0_) };
                                        { let a1_ = ((D::call(c, fid::BtlAnim_GetId, &[((p_chr_1) as u64)])) as i32); { let _ = D::call(c, fid::BtlAnim_Request, &[((p_chr_1) as u64), ((a1_) as i64 as u64), ((F32(0x00000000)).0 as u64)]); } };
                                    }
                                    { let a_ = l_stepB_4; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                                }
                                break 's4;
                            }
                            2i32 => {
                                if ((((!(((((c.m.r32(((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) & (1i32))) != 0)) as i32)) != 0) {
                                    { let a0_ = (c.m.r32(((p_chr_1).wrapping_add(3728)).wrapping_add(8)) as i32); F::BtlChange_SetDone(c, a0_) };
                                    { let a3_ = (((c.m.r32(((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) & (1i32)); F::BtlChar_RequestPlaceRelative(c, p_chr_1, l_slot_5, l_level_7, a3_) };
                                    { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((85i32) as i64 as u64)]); };
                                    F::BtlAnim_JumpToEnd(c, p_chr_1);
                                }
                                { let a_ = l_stepB_4; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                                break 's4;
                            }
                            3i32 => {
                                l_okB_9 = 1i32;
                                break 's4;
                            }
                            _ => {
                            }
                        }
                    }
                } else {
                    l_okB_9 = 1i32;
                }
                if ((((((((((c.m.r32(((p_chr_1).wrapping_add(3728)).wrapping_add(72)) as i32)) != (0i32)) as i32)) != 0) && (((((((c.m.r32(((p_chr_1).wrapping_add(3728)).wrapping_add(16)) as i32)).wrapping_sub(1i32)) == (l_level_7)) as i32)) != 0)) as i32)) != 0) {
                    's5: {
                        let sv20: i32 = (c.m.r32(l_stepA_3) as i32);
                        match sv20 {
                            0i32 => {
                                if (((((((((c.m.r32(((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) & (1i32))) != 0) && ((((D::call(c, fid::BtlAnim_IsNew, &[((p_chr_1) as u64)])) as i32)) != 0)) as i32)) != 0) {
                                    { let _ = D::call(c, fid::BtlAct_SetFormCurrent, &[((p_chr_1) as u64)]); };
                                    { let a0_ = (c.m.r32(p_chr_1) as i32); let a1_ = (c.m.r32(((p_chr_1).wrapping_add(4812)).wrapping_add(4)) as i32); let a2_ = (c.m.r32(((p_chr_1).wrapping_add(4812)).wrapping_add(8)) as i32); let a3_ = (c.m.r32(((p_chr_1).wrapping_add(4812)).wrapping_add(20)) as i32); let a4_ = (c.m.r32(((p_chr_1).wrapping_add(4812)).wrapping_add(24)) as i32); let a5_ = (c.m.r32(((p_chr_1).wrapping_add(4812)).wrapping_add(28)) as i32); let a6_ = (c.m.r32(((p_chr_1).wrapping_add(4812)).wrapping_add(32)) as i32); F::BtlChange_RequestChara(c, a0_, a1_, a2_, a3_, a4_, a5_, a6_) };
                                }
                                if ((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((49i32) as i64 as u64)])) as i32)) != 0) {
                                    { let a_ = l_stepA_3; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                                }
                                break 's5;
                            }
                            1i32 => {
                                if (({ let a0_ = (c.m.r32(((p_chr_1).wrapping_add(3728)).wrapping_add(4)) as i32); F::BtlChange_IsLoadedFor(c, a0_) }) != 0) {
                                    if (((((c.m.r32(((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) & (1i32))) != 0) {
                                        F::BtlChar_ResetUnk1310(c, p_chr_1);
                                        { let a0_ = (c.m.r32(((p_chr_1).wrapping_add(3728)).wrapping_add(4)) as i32); F::BtlChange_SetReady(c, a0_) };
                                        { let a1_ = ((D::call(c, fid::BtlAnim_GetId, &[((p_chr_1) as u64)])) as i32); { let _ = D::call(c, fid::BtlAnim_Request, &[((p_chr_1) as u64), ((a1_) as i64 as u64), ((F32(0x00000000)).0 as u64)]); } };
                                    }
                                    { let a_ = l_stepA_3; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                                }
                                break 's5;
                            }
                            2i32 => {
                                if (((((c.m.r32(((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) & (1i32))) != 0) {
                                    { let a0_ = (c.m.r32(((p_chr_1).wrapping_add(3728)).wrapping_add(4)) as i32); F::BtlChange_SetDone(c, a0_) };
                                    { let a3_ = (((c.m.r32(((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) & (1i32)); F::BtlChar_RequestPlaceRelative(c, p_chr_1, l_slot_5, l_level_7, a3_) };
                                    { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((85i32) as i64 as u64)]); };
                                    F::BtlAnim_JumpToEnd(c, p_chr_1);
                                }
                                { let a_ = l_stepA_3; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                                break 's5;
                            }
                            3i32 => {
                                l_okA_8 = 1i32;
                                break 's5;
                            }
                            _ => {
                            }
                        }
                    }
                } else {
                    l_okA_8 = 1i32;
                }
                if ((((((((((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((49i32) as i64 as u64)])) as i32)) != 0) && ((l_okA_8) != 0)) as i32)) != 0) && ((l_okB_9) != 0)) as i32)) != 0) {
                    if (((((l_level_7) < (((c.m.r32(((p_chr_1).wrapping_add(3728)).wrapping_add(16)) as i32)).wrapping_sub(1i32))) as i32)) != 0) {
                        { let a1_ = (((D::call(c, fid::BtlAnim_GetId, &[((p_chr_1) as u64)])) as i32)).wrapping_add(1i32); { let _ = D::call(c, fid::BtlAnim_Request, &[((p_chr_1) as u64), ((a1_) as i64 as u64), ((F32(0x00000000)).0 as u64)]); } };
                    } else {
                        { let v_ = 1i32; let a_ = ((p_chr_1).wrapping_add(976)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    }
                }
            }
        }
        { let _ = D::call(c, fid::BtlMove_Step, &[((p_chr_1) as u64), ((6i32) as i64 as u64), ((6i32) as i64 as u64), ((7i32) as i64 as u64), ((F32(0x00000000)).0 as u64), ((F32(0x3f6d0979)).0 as u64)]); };
        F::BtlMove_BrakeVertical(c, p_chr_1);
        { let _ = D::call(c, fid::BtlSuper_SetRushFlags, &[((p_chr_1) as u64)]); };
        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((66i32) as i64 as u64)]); };
        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((67i32) as i64 as u64)]); };
        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((68i32) as i64 as u64)]); };
        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((69i32) as i64 as u64)]); };
        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((70i32) as i64 as u64)]); };
        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((9i32) as i64 as u64)]); };
        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((310i32) as i64 as u64)]); };
        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((88i32) as i64 as u64)]); };
        { let _ = D::call(c, fid::BtlChar_SetFxBit, &[((p_chr_1) as u64), ((59i32) as i64 as u64)]); };
        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((27i32) as i64 as u64)]); };
        if ((((((((((c.m.r32((p_chr_1).wrapping_add(3728)) as i32)) == (740i32)) as i32)) != 0) && (((((l_level_7) == (0i32)) as i32)) != 0)) as i32)) != 0) {
            { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((315i32) as i64 as u64)]); };
        }
        if (((((c.m.r32(((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) & (1i32))) != 0) {
            { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((139i32) as i64 as u64)]); };
            if (((((l_level_7) < (F::BtlSuper_GetUnk223(c, p_chr_1, l_slot_5))) as i32)) != 0) {
                { let _ = D::call(c, fid::BtlChar_SetFxBit, &[((p_chr_1) as u64), ((17i32) as i64 as u64)]); };
            }
            if ((((((D::call(c, fid::BtlSuper_GetFlagsA, &[((p_chr_1) as u64), ((l_slot_5) as i64 as u64)])) as i32)) & (536870912i32))) != 0) {
                { let _ = D::call(c, fid::BtlChar_SetFxBit, &[((p_chr_1) as u64), ((20i32) as i64 as u64)]); };
            }
            if ((((((D::call(c, fid::BtlSuper_GetFlags, &[((p_chr_1) as u64), ((l_slot_5) as i64 as u64)])) as i32)) & (536870912i32))) != 0) {
                { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((315i32) as i64 as u64)]); };
            }
            if ((((((D::call(c, fid::BtlSuper_GetFlags, &[((p_chr_1) as u64), ((l_slot_5) as i64 as u64)])) as i32)) & (8i32))) != 0) {
                { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((316i32) as i64 as u64)]); };
            }
            if (((((c.m.r32(((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) & (4i32))) != 0) {
                { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((166i32) as i64 as u64)]); };
            }
        } else {
            { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((151i32) as i64 as u64)]); };
            { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((149i32) as i64 as u64)]); };
            if (((({ let a0_ = { let a0_ = F::BtlOpp_GetPlayer(c, p_chr_1); F::BtlChar_Get(c, a0_) }; ((D::call(c, fid::BtlSuper_GetFlagsA, &[((a0_) as u64), ((l_slot_5) as i64 as u64)])) as i32) }) & (1073741824i32))) != 0) {
                { let _ = D::call(c, fid::BtlChar_SetFxBit, &[((p_chr_1) as u64), ((20i32) as i64 as u64)]); };
            }
        }
        { let v_ = F32((D::call(c, fid::BtlAnim_GetFrame, &[((p_chr_1) as u64)])) as u32); let a_ = ((p_chr_1).wrapping_add(3836)).wrapping_add((((l_level_7) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        if ((((((((((((((c.m.r32(((p_chr_1).wrapping_add(976)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) != (0i32)) as i32)) != 0) && (((((c.m.r32(((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) & (1i32))) != 0)) as i32)) != 0) && ((((((D::call(c, fid::BtlSuper_GetFlags, &[((p_chr_1) as u64), ((l_slot_5) as i64 as u64)])) as i32)) & (8192i32))) != 0)) as i32)) != 0) {
            { let a1_ = ({ let a_ = ((D::call(c, fid::BtlMember_GetActiveGauge, &[((p_chr_1) as u64)])) as u32); (c.m.r32(a_) as i32) }).wrapping_sub(1i32); { let _ = D::call(c, fid::BtlMember_Damage, &[((p_chr_1) as u64), ((a1_) as i64 as u64), ((91i32) as i64 as u64)]); } };
            { let a_ = ((((D::call(c, fid::BtlMember_GetActiveGauge, &[((p_chr_1) as u64)])) as u32)).wrapping_add(76)).wrapping_add((((l_slot_5) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        }
    }
    if (((((p_phase_2) == (2i32)) as i32)) != 0) {
        if ((((((c.m.r32(((p_chr_1).wrapping_add(976)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) != (0i32)) as i32)) != 0) {
            if (((((c.m.r32(((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) & (1i32))) != 0) {
                { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((140i32) as i64 as u64)]); };
                { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((51i32) as i64 as u64)]); };
                F::BtlChar_AddStageTimer(c, F32(0x40400000));
                { let _ = D::call(c, fid::BtlSuper_Finish, &[((p_chr_1) as u64), ((l_slot_5) as i64 as u64)]); };
            } else {
                { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((51i32) as i64 as u64)]); };
                if ((((((c.m.r32(((p_chr_1).wrapping_add(3728)).wrapping_add(52)) as i32)) != (0i32)) as i32)) != 0) {
                    { let _ = D::call(c, fid::BtlChar_SetHeldFlag, &[((p_chr_1) as u64), ((14i32) as i64 as u64)]); };
                    { let _ = D::call(c, fid::BtlAct_Request, &[((p_chr_1) as u64), ((11i32) as i64 as u64)]); };
                } else {
                    if ((((((((((c.m.r32(((p_chr_1).wrapping_add(3728)).wrapping_add(64)) as i32)) != (0i32)) as i32)) != 0) && ((F::BtlChar_IsDead(c, p_chr_1)) != 0)) as i32)) != 0) {
                        { let _ = D::call(c, fid::BtlAct_Request, &[((p_chr_1) as u64), ((235i32) as i64 as u64)]); };
                    } else {
                        { let _ = D::call(c, fid::BtlChar_SetHeldFlag, &[((p_chr_1) as u64), ((29i32) as i64 as u64)]); };
                        { let _ = D::call(c, fid::BtlChar_SetHeldFlag, &[((p_chr_1) as u64), ((30i32) as i64 as u64)]); };
                        l_next_10 = { let a2_ = (c.m.r32(((p_chr_1).wrapping_add(3728)).wrapping_add(20)) as i32); F::BtlAct_GetLandingAction(c, p_chr_1, l_slot_5, ((((a2_) as i64 as u64)) as u32)) };
                        if ((((((c.m.r32(((p_chr_1).wrapping_add(3728)).wrapping_add(48)) as i32)) != (0i32)) as i32)) != 0) {
                            { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((148i32) as i64 as u64)]); };
                            l_yaw_11 = { let a_ = ((((D::call(c, fid::BtlChar_GetPos, &[((p_chr_1) as u64)])) as u32)).wrapping_add(16)).wrapping_add(4); F32(c.m.r32(a_)) };
                            l_addYaw_12 = F32(c.m.r32(((p_chr_1).wrapping_add(3728)).wrapping_add(96)));
                            l_pitch_13 = F32(c.m.r32(((p_chr_1).wrapping_add(3728)).wrapping_add(100)));
                            if ((((((c.m.r32(((p_chr_1).wrapping_add(4016)).wrapping_add(12)) as i32)) != (0i32)) as i32)) != 0) {
                                { let v_ = F::BtlUtil_WrapAngle(c, rt::fadd(l_yaw_11, l_addYaw_12)); let a_ = ((p_chr_1).wrapping_add(4016)).wrapping_add(32); c.m.w32(a_, (v_).0); };
                                { let v_ = l_pitch_13; let a_ = ((p_chr_1).wrapping_add(4016)).wrapping_add(36); c.m.w32(a_, (v_).0); };
                            } else {
                                { let v_ = { let a0_ = rt::fadd(F::BtlUtil_WrapAngle(c, rt::fadd(l_yaw_11, l_addYaw_12)), F32(0x40490fda)); F::BtlUtil_WrapAngle(c, a0_) }; let a_ = ((p_chr_1).wrapping_add(4016)).wrapping_add(32); c.m.w32(a_, (v_).0); };
                                { let v_ = rt::fneg(l_pitch_13); let a_ = ((p_chr_1).wrapping_add(4016)).wrapping_add(36); c.m.w32(a_, (v_).0); };
                            }
                        } else {
                            if ((((((c.m.r32(((p_chr_1).wrapping_add(4016)).wrapping_add(12)) as i32)) != (0i32)) as i32)) != 0) {
                                l_yaw_14 = { let a0_ = rt::fadd({ let a_ = (((D::call(c, fid::BtlChar_GetPos, &[((p_chr_1) as u64)])) as u32)).wrapping_add(148); F32(c.m.r32(a_)) }, F32(0x40490fda)); F::BtlUtil_WrapAngle(c, a0_) };
                                { let a2_ = { let a_ = (((D::call(c, fid::BtlChar_GetPos, &[((p_chr_1) as u64)])) as u32)).wrapping_add(144); F32(c.m.r32(a_)) }; { let _ = D::call(c, fid::BtlMove_SetHeading, &[((p_chr_1) as u64), ((l_yaw_14).0 as u64), ((a2_).0 as u64)]); } };
                            }
                        }
                        if ((((((c.m.r32(((p_chr_1).wrapping_add(4016)).wrapping_add(12)) as i32)) != (0i32)) as i32)) != 0) {
                            { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((53i32) as i64 as u64)]); };
                        }
                        { let _ = D::call(c, fid::BtlAct_Request, &[((p_chr_1) as u64), ((l_next_10) as i64 as u64)]); };
                    }
                }
            }
        }
    }
    if (((((p_phase_2) == (3i32)) as i32)) != 0) {
        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((55i32) as i64 as u64)]); };
        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((205i32) as i64 as u64)]); };
        if (((((c.m.r32(((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) & (1i32))) != 0) {
            { let _ = D::call(c, fid::BtlSuper_Leave, &[((p_chr_1) as u64), ((l_slot_5) as i64 as u64)]); };
            if ((((((c.m.r32(((p_chr_1).wrapping_add(3728)).wrapping_add(92)) as i32)) != (0i32)) as i32)) != 0) {
                l_pose_15 = ((D::call(c, fid::BtlChar_GetPos, &[((p_chr_1) as u64)])) as u32);
                { let r_ = F::BtlAct_GetHeight(c, p_chr_1); let a_ = (l_pose_15).wrapping_add(4); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            } else {
                { let _ = D::call(c, fid::BtlChar_SetHeldFlag, &[((p_chr_1) as u64), ((14i32) as i64 as u64)]); };
            }
            if (((((c.m.r32(((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) & (2i32))) != 0) {
                F::BtlPartner_Release(c, p_chr_1);
            }
            F::BtlChar_AddStageTimer(c, F32(0x40400000));
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_SuperRushSequenceHandler", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_SuperRushFlyHandler(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_SuperRushFlyHandler", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let mut l_slot_3: i32 = 0;
    let mut l_lean_4: F32 = F32(0);
    let mut l_moving_5: i32 = 0;
    let mut l_speed_6: F32 = F32(0);
    let mut l_pose_7: u32 = 0;
    let mut l_stop_8: i32 = 0;
    let r_ = 'f: {
    l_slot_3 = ((D::call(c, fid::BtlAct_GetCurrentClass, &[((p_chr_1) as u64)])) as i32);
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        { let _ = D::call(c, fid::BtlAnim_Play, &[((p_chr_1) as u64), (((((l_slot_3).wrapping_mul(34i32)).wrapping_add(193i32)).wrapping_add(0i32)) as i64 as u64), ((F32(0x3e199999)).0 as u64)]); };
        { let _ = D::call(c, fid::BtlSuper_Begin, &[((p_chr_1) as u64), ((l_slot_3) as i64 as u64), ((0i32) as i64 as u64)]); };
        { let _ = D::call(c, fid::BtlSuper_PlayStartSound, &[((p_chr_1) as u64), ((l_slot_3) as i64 as u64), ((0i32) as i64 as u64)]); };
        { let _ = D::call(c, fid::BtlMove_TurnYaw, &[((p_chr_1) as u64), ((2i32) as i64 as u64), ((F32(0x40490fda)).0 as u64)]); };
        { let _ = D::call(c, fid::BtlMove_TurnPitch, &[((p_chr_1) as u64), ((1i32) as i64 as u64), ((F32(0x40490fda)).0 as u64)]); };
        { let _ = D::call(c, fid::BtlMove_TurnModelYaw, &[((p_chr_1) as u64), ((F32(0x40490fda)).0 as u64), ((F32(0x3f800000)).0 as u64)]); };
        { let _ = D::call(c, fid::BtlSuper_SetStartFx, &[((p_chr_1) as u64), ((l_slot_3) as i64 as u64)]); };
        { let _ = D::call(c, fid::BtlSuper_SetClassFlag, &[((p_chr_1) as u64), ((l_slot_3) as i64 as u64), ((0i32) as i64 as u64)]); };
        { let _ = D::call(c, fid::BtlSuper_RequestCut, &[((p_chr_1) as u64), ((l_slot_3) as i64 as u64)]); };
        { let a1_ = ((D::call(c, fid::BtlSuper_GetKiCost, &[((p_chr_1) as u64), ((l_slot_3) as i64 as u64)])) as i32); { let _ = D::call(c, fid::BtlMember_SpendKi, &[((p_chr_1) as u64), ((a1_) as i64 as u64), ((1i32) as i64 as u64)]); } };
        { let _ = D::call(c, fid::BtlChar_SetHeldFlag, &[((p_chr_1) as u64), ((14i32) as i64 as u64)]); };
        { let _ = D::call(c, fid::BtlChar_ResetLook, &[((p_chr_1) as u64)]); };
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        l_lean_4 = F32(0x00000000);
        l_moving_5 = 0i32;
        's1: {
            let sv9: i32 = ((D::call(c, fid::BtlAnim_GetId, &[((p_chr_1) as u64)])) as i32);
            match sv9 {
                261i32 | 295i32 | 329i32 => {
                    { let _ = ((D::call(c, fid::BtlAnim_AdvanceThen, &[((p_chr_1) as u64), (((((l_slot_3).wrapping_mul(34i32)).wrapping_add(193i32)).wrapping_add(1i32)) as i64 as u64), ((F32(0x00000000)).0 as u64), ((0i32) as i64 as u64)])) as i32); };
                    { let _ = D::call(c, fid::BtlSuper_RequestHitStop, &[((p_chr_1) as u64), ((0i32) as i64 as u64)]); };
                    l_lean_4 = F32(0x00000000);
                    { let _ = D::call(c, fid::BtlSuper_SetClass, &[((p_chr_1) as u64), ((l_slot_3) as i64 as u64)]); };
                    break 's1;
                }
                262i32 | 296i32 | 330i32 => {
                    { let _ = ((D::call(c, fid::BtlAnim_AdvanceThen, &[((p_chr_1) as u64), (((((l_slot_3).wrapping_mul(34i32)).wrapping_add(193i32)).wrapping_add(2i32)) as i64 as u64), ((F32(0x00000000)).0 as u64), ((0i32) as i64 as u64)])) as i32); };
                    { let _ = D::call(c, fid::BtlSuper_RequestHitStop, &[((p_chr_1) as u64), ((0i32) as i64 as u64)]); };
                    l_lean_4 = F32(0x00000000);
                    { let _ = D::call(c, fid::BtlSuper_SetClass, &[((p_chr_1) as u64), ((l_slot_3) as i64 as u64)]); };
                    break 's1;
                }
                263i32 | 297i32 | 331i32 => {
                    if ((((D::call(c, fid::BtlAnim_AdvanceThen, &[((p_chr_1) as u64), (((((l_slot_3).wrapping_mul(34i32)).wrapping_add(193i32)).wrapping_add(3i32)) as i64 as u64), ((F32(0x00000000)).0 as u64), ((0i32) as i64 as u64)])) as i32)) != 0) {
                        { let _ = D::call(c, fid::BtlChar_Vibrate, &[((p_chr_1) as u64), ((F32(0x3f4ccccc)).0 as u64), ((F32(0x3e999999)).0 as u64)]); };
                        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((208i32) as i64 as u64)]); };
                    }
                    { let _ = D::call(c, fid::BtlSuper_SetClass, &[((p_chr_1) as u64), ((l_slot_3) as i64 as u64)]); };
                    if ((((D::call(c, fid::BtlAnim_IsNew, &[((p_chr_1) as u64)])) as i32)) != 0) {
                        { let _ = D::call(c, fid::ChrCam_EndCut, &[((p_chr_1) as u64)]); };
                    }
                    l_lean_4 = F::BtlAnim_GetProgress(c, p_chr_1);
                    break 's1;
                }
                264i32 | 298i32 | 332i32 => {
                    { let _ = D::call(c, fid::BtlAnim_AdvanceLoop, &[((p_chr_1) as u64), ((0i32) as i64 as u64)]); };
                    if ((((D::call(c, fid::BtlAnim_IsNew, &[((p_chr_1) as u64)])) as i32)) != 0) {
                        { let _ = D::call(c, fid::BtlMove_TurnYaw, &[((p_chr_1) as u64), ((2i32) as i64 as u64), ((F32(0x40490fda)).0 as u64)]); };
                        { let _ = D::call(c, fid::BtlMove_TurnPitch, &[((p_chr_1) as u64), ((1i32) as i64 as u64), ((F32(0x40490fda)).0 as u64)]); };
                        { let _ = D::call(c, fid::BtlMove_TurnModelYaw, &[((p_chr_1) as u64), ((F32(0x40490fda)).0 as u64), ((F32(0x3f800000)).0 as u64)]); };
                    }
                    l_moving_5 = 1i32;
                    l_lean_4 = F32(0x3f800000);
                    F::BtlChar_SetSmallVibration(c, p_chr_1, F32(0x3dcccccc));
                    break 's1;
                }
                267i32 | 301i32 | 335i32 => {
                    if ((((D::call(c, fid::BtlAnim_Advance, &[((p_chr_1) as u64), ((0i32) as i64 as u64)])) as i32)) != 0) {
                        { let v_ = 1i32; let a_ = ((p_chr_1).wrapping_add(976)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    }
                    l_lean_4 = rt::fsub(F32(0x3f800000), F::BtlAnim_GetProgress(c, p_chr_1));
                    break 's1;
                }
                _ => {
                }
            }
        }
        if ((l_moving_5) != 0) {
            l_speed_6 = F::BtlSuper_GetRushSpeed(c, p_chr_1, l_slot_3);
            { let _ = D::call(c, fid::BtlMove_TurnYaw, &[((p_chr_1) as u64), ((1i32) as i64 as u64), ((F32(0x3e00adfb)).0 as u64)]); };
            { let _ = D::call(c, fid::BtlMove_TurnPitch, &[((p_chr_1) as u64), ((0i32) as i64 as u64), ((F32(0x3dc104f9)).0 as u64)]); };
            { let _ = D::call(c, fid::BtlMove_TurnModelYaw, &[((p_chr_1) as u64), ((F32(0x40490fda)).0 as u64), ((F32(0x3f000000)).0 as u64)]); };
            { let _ = D::call(c, fid::BtlMove_SetDirection, &[((p_chr_1) as u64), ((3i32) as i64 as u64)]); };
            F::BtlMove_Advance(c, p_chr_1, l_speed_6, F32(0x461c4000));
            { let _ = D::call(c, fid::BtlMove_ApplyGravity, &[((p_chr_1) as u64)]); };
            { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((9i32) as i64 as u64)]); };
        } else {
            { let _ = D::call(c, fid::BtlMove_Step, &[((p_chr_1) as u64), ((6i32) as i64 as u64), ((6i32) as i64 as u64), ((7i32) as i64 as u64), ((F32(0x00000000)).0 as u64), ((F32(0x3fed0979)).0 as u64)]); };
        }
        { let _ = D::call(c, fid::BtlMove_ApplyGravity, &[((p_chr_1) as u64)]); };
        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((187i32) as i64 as u64)]); };
        { let _ = D::call(c, fid::BtlChar_SetFxBit, &[((p_chr_1) as u64), ((16i32) as i64 as u64)]); };
        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((26i32) as i64 as u64)]); };
        if ((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((5i32) as i64 as u64)])) as i32)) != 0) {
            { let a1_ = rt::fmul({ let a_ = (((D::call(c, fid::BtlChar_GetPos, &[((p_chr_1) as u64)])) as u32)).wrapping_add(144); F32(c.m.r32(a_)) }, l_lean_4); F::BtlMove_SetLeanX(c, p_chr_1, a1_) };
        }
        if ((((D::call(c, fid::BtlInput_TestAction, &[((p_chr_1) as u64), ((111i32) as i64 as u64), ((1i32) as i64 as u64)])) as i32)) != 0) {
            { let _ = D::call(c, fid::BtlChar_SetHeldFlag, &[((p_chr_1) as u64), ((158i32) as i64 as u64)]); };
            { let _ = D::call(c, fid::BtlChar_SetHeldFlag, &[((p_chr_1) as u64), ((157i32) as i64 as u64)]); };
            { let _ = D::call(c, fid::BtlChar_SetHeldFlag, &[((p_chr_1) as u64), ((156i32) as i64 as u64)]); };
            if ((((D::call(c, fid::BtlChar_IsFlagRaised, &[((p_chr_1) as u64), ((156i32) as i64 as u64)])) as i32)) != 0) {
                { let _ = D::call(c, fid::BtlCharSnd_PlayCommon, &[((p_chr_1) as u64), ((20i32) as i64 as u64)]); };
                { let _ = D::call(c, fid::BtlCharSnd_PlayCommon, &[((p_chr_1) as u64), ((21i32) as i64 as u64)]); };
                { let _ = D::call(c, fid::BtlCharSnd_PlayCommon, &[((p_chr_1) as u64), ((28i32) as i64 as u64)]); };
            }
        }
    }
    if (((((p_phase_2) == (2i32)) as i32)) != 0) {
        l_pose_7 = ((D::call(c, fid::BtlChar_GetPos, &[((p_chr_1) as u64)])) as u32);
        's2: {
            let sv10: i32 = ((D::call(c, fid::BtlAnim_GetId, &[((p_chr_1) as u64)])) as i32);
            match sv10 {
                261i32 | 295i32 | 329i32 | 262i32 | 296i32 | 330i32 | 263i32 | 297i32 | 331i32 | 267i32 | 301i32 | 335i32 => {
                    break 's2;
                }
                264i32 | 298i32 | 332i32 => {
                    {
                        l_stop_8 = 0i32;
                        if ((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((168i32) as i64 as u64)])) as i32)) != 0) {
                            { let v_ = F32(0x00000000); let a_ = (l_pose_7).wrapping_add(152); c.m.w32(a_, (v_).0); };
                            l_stop_8 = 1i32;
                        }
                        if ((((!((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((5i32) as i64 as u64)])) as i32)) != 0)) as i32)) != 0) {
                            l_stop_8 = 1i32;
                        }
                        if ((l_stop_8) != 0) {
                            { let _ = D::call(c, fid::BtlAnim_Request, &[((p_chr_1) as u64), (((((l_slot_3).wrapping_mul(34i32)).wrapping_add(193i32)).wrapping_add(6i32)) as i64 as u64), ((F32(0x3e199999)).0 as u64)]); };
                        }
                        break 's2;
                    }
                }
                _ => {
                }
            }
        }
        if ((((((c.m.r32(((p_chr_1).wrapping_add(976)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) != (0i32)) as i32)) != 0) {
            { let _ = D::call(c, fid::BtlSuper_Finish, &[((p_chr_1) as u64), ((l_slot_3) as i64 as u64)]); };
        }
    }
    if (((((p_phase_2) == (3i32)) as i32)) != 0) {
        { let _ = D::call(c, fid::BtlSuper_Leave, &[((p_chr_1) as u64), ((l_slot_3) as i64 as u64)]); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_SuperRushFlyHandler", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_SuperRushFollowHandler(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_SuperRushFollowHandler", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let mut l_slot_3: i32 = 0;
    let mut l_lean_4: F32 = F32(0);
    let mut l_moving_5: i32 = 0;
    let mut l_speed_6: F32 = F32(0);
    let mut l_pose_7: u32 = 0;
    let mut l_stop_8: i32 = 0;
    let r_ = 'f: {
    l_slot_3 = ((D::call(c, fid::BtlAct_GetCurrentClass, &[((p_chr_1) as u64)])) as i32);
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        { let _ = D::call(c, fid::BtlAnim_Play, &[((p_chr_1) as u64), (((((l_slot_3).wrapping_mul(34i32)).wrapping_add(193i32)).wrapping_add(0i32)) as i64 as u64), ((F32(0x3e199999)).0 as u64)]); };
        { let _ = D::call(c, fid::BtlSuper_Begin, &[((p_chr_1) as u64), ((l_slot_3) as i64 as u64), ((0i32) as i64 as u64)]); };
        { let _ = D::call(c, fid::BtlSuper_PlayStartSound, &[((p_chr_1) as u64), ((l_slot_3) as i64 as u64), ((0i32) as i64 as u64)]); };
        { let _ = D::call(c, fid::BtlMove_TurnYaw, &[((p_chr_1) as u64), ((2i32) as i64 as u64), ((F32(0x40490fda)).0 as u64)]); };
        { let _ = D::call(c, fid::BtlMove_TurnPitch, &[((p_chr_1) as u64), ((1i32) as i64 as u64), ((F32(0x40490fda)).0 as u64)]); };
        { let _ = D::call(c, fid::BtlMove_TurnModelYaw, &[((p_chr_1) as u64), ((F32(0x40490fda)).0 as u64), ((F32(0x3f800000)).0 as u64)]); };
        { let _ = D::call(c, fid::BtlSuper_SetStartFx, &[((p_chr_1) as u64), ((l_slot_3) as i64 as u64)]); };
        { let _ = D::call(c, fid::BtlChar_SetHeldFlag, &[((p_chr_1) as u64), ((14i32) as i64 as u64)]); };
        if (((((((D::call(c, fid::BtlAct_GetPrev, &[((p_chr_1) as u64)])) as i32)) == (46i32)) as i32)) != 0) {
            { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((205i32) as i64 as u64)]); };
        }
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        l_lean_4 = F32(0x00000000);
        l_moving_5 = 0i32;
        's1: {
            let sv9: i32 = ((D::call(c, fid::BtlAnim_GetId, &[((p_chr_1) as u64)])) as i32);
            match sv9 {
                261i32 | 295i32 | 329i32 => {
                    { let _ = D::call(c, fid::BtlAnim_SetDuration, &[((p_chr_1) as u64), ((F32(0x3dcccccc)).0 as u64)]); };
                    { let _ = ((D::call(c, fid::BtlAnim_AdvanceThen, &[((p_chr_1) as u64), (((((l_slot_3).wrapping_mul(34i32)).wrapping_add(193i32)).wrapping_add(2i32)) as i64 as u64), ((F32(0x00000000)).0 as u64), ((0i32) as i64 as u64)])) as i32); };
                    l_lean_4 = F32(0x00000000);
                    break 's1;
                }
                263i32 | 297i32 | 331i32 => {
                    { let _ = D::call(c, fid::BtlAnim_SetDuration, &[((p_chr_1) as u64), ((F32(0x3dcccccc)).0 as u64)]); };
                    if ((((D::call(c, fid::BtlAnim_AdvanceThen, &[((p_chr_1) as u64), (((((l_slot_3).wrapping_mul(34i32)).wrapping_add(193i32)).wrapping_add(3i32)) as i64 as u64), ((F32(0x00000000)).0 as u64), ((0i32) as i64 as u64)])) as i32)) != 0) {
                        { let _ = D::call(c, fid::BtlChar_Vibrate, &[((p_chr_1) as u64), ((F32(0x3f4ccccc)).0 as u64), ((F32(0x3e999999)).0 as u64)]); };
                        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((208i32) as i64 as u64)]); };
                    }
                    l_lean_4 = F::BtlAnim_GetProgress(c, p_chr_1);
                    break 's1;
                }
                264i32 | 298i32 | 332i32 => {
                    { let _ = D::call(c, fid::BtlAnim_AdvanceLoop, &[((p_chr_1) as u64), ((0i32) as i64 as u64)]); };
                    if ((((D::call(c, fid::BtlAnim_IsNew, &[((p_chr_1) as u64)])) as i32)) != 0) {
                        { let _ = D::call(c, fid::BtlMove_TurnYaw, &[((p_chr_1) as u64), ((2i32) as i64 as u64), ((F32(0x40490fda)).0 as u64)]); };
                        { let _ = D::call(c, fid::BtlMove_TurnPitch, &[((p_chr_1) as u64), ((1i32) as i64 as u64), ((F32(0x40490fda)).0 as u64)]); };
                        { let _ = D::call(c, fid::BtlMove_TurnModelYaw, &[((p_chr_1) as u64), ((F32(0x40490fda)).0 as u64), ((F32(0x3f800000)).0 as u64)]); };
                    }
                    l_moving_5 = 1i32;
                    l_lean_4 = F32(0x3f800000);
                    F::BtlChar_SetSmallVibration(c, p_chr_1, F32(0x3dcccccc));
                    break 's1;
                }
                267i32 | 301i32 | 335i32 => {
                    if ((((D::call(c, fid::BtlAnim_Advance, &[((p_chr_1) as u64), ((0i32) as i64 as u64)])) as i32)) != 0) {
                        { let v_ = 1i32; let a_ = ((p_chr_1).wrapping_add(976)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    }
                    l_lean_4 = rt::fsub(F32(0x3f800000), F::BtlAnim_GetProgress(c, p_chr_1));
                    break 's1;
                }
                _ => {
                }
            }
        }
        if ((l_moving_5) != 0) {
            l_speed_6 = F::BtlSuper_GetRushSpeed(c, p_chr_1, l_slot_3);
            { let _ = D::call(c, fid::BtlMove_TurnYaw, &[((p_chr_1) as u64), ((1i32) as i64 as u64), ((F32(0x3e00adfb)).0 as u64)]); };
            { let _ = D::call(c, fid::BtlMove_TurnPitch, &[((p_chr_1) as u64), ((0i32) as i64 as u64), ((F32(0x3dc104f9)).0 as u64)]); };
            { let _ = D::call(c, fid::BtlMove_TurnModelYaw, &[((p_chr_1) as u64), ((F32(0x40490fda)).0 as u64), ((F32(0x3f000000)).0 as u64)]); };
            { let _ = D::call(c, fid::BtlMove_SetDirection, &[((p_chr_1) as u64), ((3i32) as i64 as u64)]); };
            F::BtlMove_Advance(c, p_chr_1, l_speed_6, F32(0x461c4000));
            { let _ = D::call(c, fid::BtlMove_ApplyGravity, &[((p_chr_1) as u64)]); };
            { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((9i32) as i64 as u64)]); };
        } else {
            { let _ = D::call(c, fid::BtlMove_Step, &[((p_chr_1) as u64), ((6i32) as i64 as u64), ((6i32) as i64 as u64), ((7i32) as i64 as u64), ((F32(0x00000000)).0 as u64), ((F32(0x3fed0979)).0 as u64)]); };
        }
        { let _ = D::call(c, fid::BtlMove_ApplyGravity, &[((p_chr_1) as u64)]); };
        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((187i32) as i64 as u64)]); };
        { let _ = D::call(c, fid::BtlChar_SetFxBit, &[((p_chr_1) as u64), ((16i32) as i64 as u64)]); };
        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((26i32) as i64 as u64)]); };
        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((164i32) as i64 as u64)]); };
        if ((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((5i32) as i64 as u64)])) as i32)) != 0) {
            { let a1_ = rt::fmul({ let a_ = (((D::call(c, fid::BtlChar_GetPos, &[((p_chr_1) as u64)])) as u32)).wrapping_add(144); F32(c.m.r32(a_)) }, l_lean_4); F::BtlMove_SetLeanX(c, p_chr_1, a1_) };
        }
    }
    if (((((p_phase_2) == (2i32)) as i32)) != 0) {
        l_pose_7 = ((D::call(c, fid::BtlChar_GetPos, &[((p_chr_1) as u64)])) as u32);
        's2: {
            let sv10: i32 = ((D::call(c, fid::BtlAnim_GetId, &[((p_chr_1) as u64)])) as i32);
            match sv10 {
                261i32 | 295i32 | 329i32 | 262i32 | 296i32 | 330i32 | 263i32 | 297i32 | 331i32 | 267i32 | 301i32 | 335i32 => {
                    break 's2;
                }
                264i32 | 298i32 | 332i32 => {
                    {
                        l_stop_8 = 0i32;
                        if ((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((168i32) as i64 as u64)])) as i32)) != 0) {
                            { let v_ = F32(0x00000000); let a_ = (l_pose_7).wrapping_add(152); c.m.w32(a_, (v_).0); };
                            l_stop_8 = 1i32;
                        }
                        if ((((!((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((5i32) as i64 as u64)])) as i32)) != 0)) as i32)) != 0) {
                            l_stop_8 = 1i32;
                        }
                        if ((l_stop_8) != 0) {
                            { let _ = D::call(c, fid::BtlAnim_Request, &[((p_chr_1) as u64), (((((l_slot_3).wrapping_mul(34i32)).wrapping_add(193i32)).wrapping_add(6i32)) as i64 as u64), ((F32(0x3e199999)).0 as u64)]); };
                        }
                        break 's2;
                    }
                }
                _ => {
                }
            }
        }
        if ((((((c.m.r32(((p_chr_1).wrapping_add(976)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) != (0i32)) as i32)) != 0) {
            { let _ = D::call(c, fid::BtlSuper_Finish, &[((p_chr_1) as u64), ((l_slot_3) as i64 as u64)]); };
        }
    }
    if (((((p_phase_2) == (3i32)) as i32)) != 0) {
        { let _ = D::call(c, fid::BtlSuper_Leave, &[((p_chr_1) as u64), ((l_slot_3) as i64 as u64)]); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_SuperRushFollowHandler", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_SuperRushFinishHandler(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_SuperRushFinishHandler", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let mut l_slot_3: i32 = 0;
    let mut l_cutOn_4: i32 = 0;
    let mut l_cutOff_5: i32 = 0;
    let mut l_pose_6: u32 = 0;
    let r_ = 'f: {
    l_slot_3 = ((D::call(c, fid::BtlAct_GetCurrentClass, &[((p_chr_1) as u64)])) as i32);
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        { let _ = D::call(c, fid::BtlAnim_Play, &[((p_chr_1) as u64), (((((l_slot_3).wrapping_mul(34i32)).wrapping_add(193i32)).wrapping_add(21i32)) as i64 as u64), ((F32(0x00000000)).0 as u64)]); };
        { let v_ = F32(0x00000000); let a_ = (((D::call(c, fid::BtlChar_GetPos, &[((p_chr_1) as u64)])) as u32)).wrapping_add(152); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (((D::call(c, fid::BtlChar_GetPos, &[((p_chr_1) as u64)])) as u32)).wrapping_add(156); c.m.w32(a_, (v_).0); };
        { let _ = D::call(c, fid::BtlSuper_SetupRushDamage, &[((p_chr_1) as u64), ((l_slot_3) as i64 as u64), ((1i32) as i64 as u64)]); };
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        if ((((D::call(c, fid::BtlAnim_Advance, &[((p_chr_1) as u64), ((0i32) as i64 as u64)])) as i32)) != 0) {
            { let v_ = 1i32; let a_ = ((p_chr_1).wrapping_add(976)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        }
        { let _ = D::call(c, fid::BtlMove_Step, &[((p_chr_1) as u64), ((6i32) as i64 as u64), ((6i32) as i64 as u64), ((7i32) as i64 as u64), ((F32(0x00000000)).0 as u64), ((F32(0x3f6d0979)).0 as u64)]); };
        F::BtlMove_BrakeVertical(c, p_chr_1);
        { let _ = D::call(c, fid::BtlSuper_SetRushFlags, &[((p_chr_1) as u64)]); };
        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((66i32) as i64 as u64)]); };
        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((67i32) as i64 as u64)]); };
        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((68i32) as i64 as u64)]); };
        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((69i32) as i64 as u64)]); };
        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((70i32) as i64 as u64)]); };
        { let _ = D::call(c, fid::BtlChar_SetFxBit, &[((p_chr_1) as u64), ((16i32) as i64 as u64)]); };
        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((310i32) as i64 as u64)]); };
        if ((((((c.m.r32(((p_chr_1).wrapping_add(3728)).wrapping_add(56)) as i32)) != (0i32)) as i32)) != 0) {
            { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((139i32) as i64 as u64)]); };
            { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((9i32) as i64 as u64)]); };
        }
        l_cutOn_4 = F::BtlSuper_GetFrames22C(c, p_chr_1, l_slot_3);
        l_cutOff_5 = F::BtlSuper_GetFrames238(c, p_chr_1, l_slot_3);
        if (((((((((l_cutOn_4) >= (0i32)) as i32)) != 0) && (((((l_cutOff_5) >= (0i32)) as i32)) != 0)) as i32)) != 0) {
            if ((((((c.m.r32((p_chr_1).wrapping_add(2404)) as i32)) == (l_cutOn_4)) as i32)) != 0) {
                { let _ = D::call(c, fid::ChrCam_RequestCut, &[((p_chr_1) as u64), ((0i32) as i64 as u64), ((9i32) as i64 as u64)]); };
            }
            if ((((((c.m.r32((p_chr_1).wrapping_add(2404)) as i32)) == (l_cutOff_5)) as i32)) != 0) {
                { let _ = D::call(c, fid::ChrCam_EndCut, &[((p_chr_1) as u64)]); };
            }
        }
    }
    if (((((p_phase_2) == (2i32)) as i32)) != 0) {
        if ((((((c.m.r32(((p_chr_1).wrapping_add(976)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) != (0i32)) as i32)) != 0) {
            { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((140i32) as i64 as u64)]); };
            { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((51i32) as i64 as u64)]); };
            { let _ = D::call(c, fid::BtlSuper_Finish, &[((p_chr_1) as u64), ((l_slot_3) as i64 as u64)]); };
        }
    }
    if (((((p_phase_2) == (3i32)) as i32)) != 0) {
        { let _ = D::call(c, fid::BtlSuper_Leave, &[((p_chr_1) as u64), ((l_slot_3) as i64 as u64)]); };
        F::BtlChar_AddStageTimer(c, F32(0x40400000));
        if (((rt::flt(F::BtlAct_GetHeight(c, p_chr_1), F32(0x40a00000)) as i32)) != 0) {
            { let _ = D::call(c, fid::BtlChar_SetHeldFlag, &[((p_chr_1) as u64), ((15i32) as i64 as u64)]); };
            l_pose_6 = ((D::call(c, fid::BtlChar_GetPos, &[((p_chr_1) as u64)])) as u32);
            { let v_ = { let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); F::BtlCharApi_GetGroundY(c, a0_) }; let a_ = (l_pose_6).wrapping_add(4); c.m.w32(a_, (v_).0); };
            { let _ = D::call(c, fid::BtlChar_ClearFlag, &[((p_chr_1) as u64), ((14i32) as i64 as u64)]); };
        } else {
            { let _ = D::call(c, fid::BtlChar_SetHeldFlag, &[((p_chr_1) as u64), ((14i32) as i64 as u64)]); };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_SuperRushFinishHandler", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_SuperRushCaughtHandler(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_SuperRushCaughtHandler", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let mut l_slot_3: i32 = 0;
    let mut l_next_4: i32 = 0;
    let mut l_yaw_5: F32 = F32(0);
    let mut l_addYaw_6: F32 = F32(0);
    let mut l_pitch_7: F32 = F32(0);
    let mut l_yaw_8: F32 = F32(0);
    let r_ = 'f: {
    l_slot_3 = ((D::call(c, fid::BtlAct_GetCurrentClass, &[((p_chr_1) as u64)])) as i32);
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        { let _ = D::call(c, fid::BtlAnim_Play, &[((p_chr_1) as u64), (((((l_slot_3).wrapping_mul(34i32)).wrapping_add(193i32)).wrapping_add(22i32)) as i64 as u64), ((F32(0x00000000)).0 as u64)]); };
        { let v_ = F32(0x00000000); let a_ = (((D::call(c, fid::BtlChar_GetPos, &[((p_chr_1) as u64)])) as u32)).wrapping_add(152); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (((D::call(c, fid::BtlChar_GetPos, &[((p_chr_1) as u64)])) as u32)).wrapping_add(156); c.m.w32(a_, (v_).0); };
        if ((((((c.m.r32(((p_chr_1).wrapping_add(3728)).wrapping_add(68)) as i32)) != (0i32)) as i32)) != 0) {
            { let _ = D::call(c, fid::BtlCharSnd_PlayCommon, &[((p_chr_1) as u64), ((18i32) as i64 as u64)]); };
        }
        if ((((((c.m.r32(((p_chr_1).wrapping_add(3728)).wrapping_add(60)) as i32)) != (0i32)) as i32)) != 0) {
            { let _ = D::call(c, fid::BtlMove_TurnYaw, &[((p_chr_1) as u64), ((2i32) as i64 as u64), ((F32(0x40490fda)).0 as u64)]); };
            { let _ = D::call(c, fid::BtlMove_TurnPitch, &[((p_chr_1) as u64), ((5i32) as i64 as u64), ((F32(0x40490fda)).0 as u64)]); };
        }
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        if ((((((c.m.r32(((p_chr_1).wrapping_add(3728)).wrapping_add(68)) as i32)) != (0i32)) as i32)) != 0) {
            { let _ = D::call(c, fid::BtlAnim_AdvanceLoop, &[((p_chr_1) as u64), ((0i32) as i64 as u64)]); };
        } else {
            if ((((D::call(c, fid::BtlAnim_Advance, &[((p_chr_1) as u64), ((0i32) as i64 as u64)])) as i32)) != 0) {
                { let v_ = 1i32; let a_ = ((p_chr_1).wrapping_add(976)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            }
        }
        { let _ = D::call(c, fid::BtlMove_Step, &[((p_chr_1) as u64), ((6i32) as i64 as u64), ((6i32) as i64 as u64), ((7i32) as i64 as u64), ((F32(0x00000000)).0 as u64), ((F32(0x3f6d0979)).0 as u64)]); };
        F::BtlMove_BrakeVertical(c, p_chr_1);
        { let _ = D::call(c, fid::BtlSuper_SetRushFlags, &[((p_chr_1) as u64)]); };
        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((149i32) as i64 as u64)]); };
        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((150i32) as i64 as u64)]); };
        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((310i32) as i64 as u64)]); };
        if ((((((c.m.r32(((p_chr_1).wrapping_add(3728)).wrapping_add(68)) as i32)) != (0i32)) as i32)) != 0) {
            { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((66i32) as i64 as u64)]); };
            { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((67i32) as i64 as u64)]); };
            { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((68i32) as i64 as u64)]); };
            { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((69i32) as i64 as u64)]); };
        } else {
            { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((66i32) as i64 as u64)]); };
            { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((67i32) as i64 as u64)]); };
            { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((68i32) as i64 as u64)]); };
            { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((69i32) as i64 as u64)]); };
            { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((70i32) as i64 as u64)]); };
        }
        if ((((((c.m.r32(((p_chr_1).wrapping_add(3728)).wrapping_add(56)) as i32)) != (0i32)) as i32)) != 0) {
            { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((151i32) as i64 as u64)]); };
            { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((10i32) as i64 as u64)]); };
        }
    }
    if (((((p_phase_2) == (2i32)) as i32)) != 0) {
        if ((((((c.m.r32(((p_chr_1).wrapping_add(3728)).wrapping_add(68)) as i32)) != (0i32)) as i32)) != 0) {
            if ((((!((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((175i32) as i64 as u64)])) as i32)) != 0)) as i32)) != 0) {
                { let v_ = 1i32; let a_ = ((p_chr_1).wrapping_add(976)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            }
        }
        if ((((((c.m.r32(((p_chr_1).wrapping_add(976)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) != (0i32)) as i32)) != 0) {
            { let _ = D::call(c, fid::BtlChar_SetHeldFlag, &[((p_chr_1) as u64), ((29i32) as i64 as u64)]); };
            { let _ = D::call(c, fid::BtlChar_SetHeldFlag, &[((p_chr_1) as u64), ((30i32) as i64 as u64)]); };
            l_next_4 = { let a2_ = (c.m.r32(((p_chr_1).wrapping_add(3728)).wrapping_add(20)) as i32); F::BtlAct_GetLandingAction(c, p_chr_1, l_slot_3, ((((a2_) as i64 as u64)) as u32)) };
            if ((((((c.m.r32(((p_chr_1).wrapping_add(3728)).wrapping_add(48)) as i32)) != (0i32)) as i32)) != 0) {
                { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((148i32) as i64 as u64)]); };
                l_yaw_5 = F::BtlOpp_GetTargetYaw(c, p_chr_1);
                l_addYaw_6 = F32(c.m.r32(((p_chr_1).wrapping_add(3728)).wrapping_add(96)));
                l_pitch_7 = F32(c.m.r32(((p_chr_1).wrapping_add(3728)).wrapping_add(100)));
                if ((((((c.m.r32(((p_chr_1).wrapping_add(4016)).wrapping_add(12)) as i32)) != (0i32)) as i32)) != 0) {
                    { let v_ = F::BtlUtil_WrapAngle(c, rt::fadd(l_yaw_5, l_addYaw_6)); let a_ = ((p_chr_1).wrapping_add(4016)).wrapping_add(32); c.m.w32(a_, (v_).0); };
                    { let v_ = l_pitch_7; let a_ = ((p_chr_1).wrapping_add(4016)).wrapping_add(36); c.m.w32(a_, (v_).0); };
                } else {
                    { let v_ = { let a0_ = rt::fadd(F::BtlUtil_WrapAngle(c, rt::fadd(l_yaw_5, l_addYaw_6)), F32(0x40490fda)); F::BtlUtil_WrapAngle(c, a0_) }; let a_ = ((p_chr_1).wrapping_add(4016)).wrapping_add(32); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fneg(l_pitch_7); let a_ = ((p_chr_1).wrapping_add(4016)).wrapping_add(36); c.m.w32(a_, (v_).0); };
                }
            } else {
                if ((((((c.m.r32(((p_chr_1).wrapping_add(4016)).wrapping_add(12)) as i32)) != (0i32)) as i32)) != 0) {
                    l_yaw_8 = { let a0_ = rt::fadd({ let a_ = (((D::call(c, fid::BtlChar_GetPos, &[((p_chr_1) as u64)])) as u32)).wrapping_add(148); F32(c.m.r32(a_)) }, F32(0x40490fda)); F::BtlUtil_WrapAngle(c, a0_) };
                    { let a2_ = { let a_ = (((D::call(c, fid::BtlChar_GetPos, &[((p_chr_1) as u64)])) as u32)).wrapping_add(144); F32(c.m.r32(a_)) }; { let _ = D::call(c, fid::BtlMove_SetHeading, &[((p_chr_1) as u64), ((l_yaw_8).0 as u64), ((a2_).0 as u64)]); } };
                }
            }
            if ((((((c.m.r32(((p_chr_1).wrapping_add(4016)).wrapping_add(12)) as i32)) != (0i32)) as i32)) != 0) {
                { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((53i32) as i64 as u64)]); };
            }
            { let _ = D::call(c, fid::BtlAct_Request, &[((p_chr_1) as u64), ((l_next_4) as i64 as u64)]); };
        }
    }
    if (((((p_phase_2) == (3i32)) as i32)) != 0) {
        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((51i32) as i64 as u64)]); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_SuperRushCaughtHandler", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_UltimateStartHandler(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_UltimateStartHandler", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let mut l_slot_3: i32 = 0;
    let r_ = 'f: {
    l_slot_3 = ((D::call(c, fid::BtlAct_GetCurrentClass, &[((p_chr_1) as u64)])) as i32);
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        { let _ = D::call(c, fid::BtlAnim_Play, &[((p_chr_1) as u64), ((363i32) as i64 as u64), ((F32(0x00000000)).0 as u64)]); };
        { let _ = D::call(c, fid::BtlSuper_Begin, &[((p_chr_1) as u64), ((l_slot_3) as i64 as u64), ((1i32) as i64 as u64)]); };
        { let _ = D::call(c, fid::BtlSuper_PlayStartSound, &[((p_chr_1) as u64), ((l_slot_3) as i64 as u64), ((1i32) as i64 as u64)]); };
        { let _ = D::call(c, fid::BtlMove_TurnYaw, &[((p_chr_1) as u64), ((2i32) as i64 as u64), ((F32(0x40490fda)).0 as u64)]); };
        { let _ = D::call(c, fid::BtlMove_TurnPitch, &[((p_chr_1) as u64), ((5i32) as i64 as u64), ((F32(0x40490fda)).0 as u64)]); };
        { let _ = D::call(c, fid::BtlMove_TurnModelYaw, &[((p_chr_1) as u64), ((F32(0x40490fda)).0 as u64), ((F32(0x3f800000)).0 as u64)]); };
        { let _ = D::call(c, fid::BtlSuper_SetClassFlag, &[((p_chr_1) as u64), ((l_slot_3) as i64 as u64), ((1i32) as i64 as u64)]); };
        { let _ = D::call(c, fid::ChrCam_RequestCut, &[((p_chr_1) as u64), ((0i32) as i64 as u64), ((3i32) as i64 as u64)]); };
        { let v_ = F32(0x00000000); let a_ = (((D::call(c, fid::BtlChar_GetPos, &[((p_chr_1) as u64)])) as u32)).wrapping_add(152); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (((D::call(c, fid::BtlChar_GetPos, &[((p_chr_1) as u64)])) as u32)).wrapping_add(156); c.m.w32(a_, (v_).0); };
        { let _ = D::call(c, fid::BtlChar_ResetLook, &[((p_chr_1) as u64)]); };
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        { let _ = ((D::call(c, fid::BtlAnim_Advance, &[((p_chr_1) as u64), ((0i32) as i64 as u64)])) as i32); };
        { let _ = D::call(c, fid::BtlSuper_SetClass, &[((p_chr_1) as u64), ((l_slot_3) as i64 as u64)]); };
        { let _ = D::call(c, fid::BtlMove_Step, &[((p_chr_1) as u64), ((6i32) as i64 as u64), ((5i32) as i64 as u64), ((2i32) as i64 as u64), ((F32(0x00000000)).0 as u64), ((F32(0x3f6d0979)).0 as u64)]); };
        { let _ = D::call(c, fid::BtlMove_ApplyGravity, &[((p_chr_1) as u64)]); };
        { let _ = D::call(c, fid::BtlChar_SetFxBit, &[((p_chr_1) as u64), ((16i32) as i64 as u64)]); };
        { let _ = D::call(c, fid::BtlSuper_RequestHitStop, &[((p_chr_1) as u64), ((0i32) as i64 as u64)]); };
        if (((((F::BtlAct_GetQueued(c, p_chr_1)) == (276i32)) as i32)) != 0) {
            { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((163i32) as i64 as u64)]); };
        }
        if ((((D::call(c, fid::BtlInput_TestAction, &[((p_chr_1) as u64), ((111i32) as i64 as u64), ((1i32) as i64 as u64)])) as i32)) != 0) {
            { let _ = D::call(c, fid::BtlChar_SetHeldFlag, &[((p_chr_1) as u64), ((158i32) as i64 as u64)]); };
            { let _ = D::call(c, fid::BtlChar_SetHeldFlag, &[((p_chr_1) as u64), ((157i32) as i64 as u64)]); };
            { let _ = D::call(c, fid::BtlChar_SetHeldFlag, &[((p_chr_1) as u64), ((156i32) as i64 as u64)]); };
            if ((((D::call(c, fid::BtlChar_IsFlagRaised, &[((p_chr_1) as u64), ((156i32) as i64 as u64)])) as i32)) != 0) {
                { let _ = D::call(c, fid::BtlCharSnd_PlayCommon, &[((p_chr_1) as u64), ((20i32) as i64 as u64)]); };
                { let _ = D::call(c, fid::BtlCharSnd_PlayCommon, &[((p_chr_1) as u64), ((21i32) as i64 as u64)]); };
                { let _ = D::call(c, fid::BtlCharSnd_PlayCommon, &[((p_chr_1) as u64), ((28i32) as i64 as u64)]); };
            }
        }
    }
    if (((((p_phase_2) == (2i32)) as i32)) != 0) {
        if ((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((49i32) as i64 as u64)])) as i32)) != 0) {
            { let a1_ = F::BtlAct_GetQueued(c, p_chr_1); { let _ = D::call(c, fid::BtlAct_Request, &[((p_chr_1) as u64), ((a1_) as i64 as u64)]); } };
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_UltimateStartHandler", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_ClashStruggleHandler(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_ClashStruggleHandler", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let mut l_slot_3: i32 = 0;
    let mut l_anim_4: i32 = 0;
    let mut l_up_5: i32 = 0;
    let mut l_down_6: i32 = 0;
    let r_ = 'f: {
    l_slot_3 = ((D::call(c, fid::BtlAct_GetCurrentClass, &[((p_chr_1) as u64)])) as i32);
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        l_anim_4 = (((l_slot_3).wrapping_mul(34i32)).wrapping_add(193i32)).wrapping_add(3i32);
        if (((((((D::call(c, fid::BtlSuper_GetHitDirKind, &[((p_chr_1) as u64), ((l_slot_3) as i64 as u64)])) as i32)) == (1i32)) as i32)) != 0) {
            l_anim_4 = 364i32;
        }
        { let _ = D::call(c, fid::BtlAnim_Play, &[((p_chr_1) as u64), ((l_anim_4) as i64 as u64), ((F32(0x3e199999)).0 as u64)]); };
        { let _ = D::call(c, fid::BtlChar_ClearFlag, &[((p_chr_1) as u64), ((170i32) as i64 as u64)]); };
        { let _ = D::call(c, fid::BtlChar_ClearFlagRange, &[((p_chr_1) as u64), ((((191i32) as u32)) as u64), ((((195i32) as u32)) as u64)]); };
        { let _ = D::call(c, fid::BtlChar_ClearFlag, &[((p_chr_1) as u64), ((287i32) as i64 as u64)]); };
        { let v_ = rt::div_i32(F::BtlSuper_GetUnk1E(c, p_chr_1, l_slot_3), 20i32); let a_ = (p_chr_1).wrapping_add(3660); c.m.w32(a_, (v_) as u32); };
        if ((((D::call(c, fid::BtlMember_HasAbility, &[((p_chr_1) as u64), ((20i32) as i64 as u64)])) as i32)) != 0) {
            { let r_ = 10i32; let a_ = (p_chr_1).wrapping_add(3660); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
        } else {
            if ((((D::call(c, fid::BtlMember_HasAbility, &[((p_chr_1) as u64), ((19i32) as i64 as u64)])) as i32)) != 0) {
                { let r_ = 6i32; let a_ = (p_chr_1).wrapping_add(3660); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
            } else {
                if ((((D::call(c, fid::BtlMember_HasAbility, &[((p_chr_1) as u64), ((18i32) as i64 as u64)])) as i32)) != 0) {
                    { let r_ = 3i32; let a_ = (p_chr_1).wrapping_add(3660); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                }
            }
        }
        if ((((D::call(c, fid::BtlMember_HasAbility, &[((p_chr_1) as u64), ((106i32) as i64 as u64)])) as i32)) != 0) {
            { let r_ = 10i32; let a_ = (p_chr_1).wrapping_add(3660); let n_ = ((c.m.r32(a_) as i32)).wrapping_sub(r_); c.m.w32(a_, (n_) as u32); };
        } else {
            if ((((D::call(c, fid::BtlMember_HasAbility, &[((p_chr_1) as u64), ((105i32) as i64 as u64)])) as i32)) != 0) {
                { let r_ = 6i32; let a_ = (p_chr_1).wrapping_add(3660); let n_ = ((c.m.r32(a_) as i32)).wrapping_sub(r_); c.m.w32(a_, (n_) as u32); };
            } else {
                if ((((D::call(c, fid::BtlMember_HasAbility, &[((p_chr_1) as u64), ((104i32) as i64 as u64)])) as i32)) != 0) {
                    { let r_ = 3i32; let a_ = (p_chr_1).wrapping_add(3660); let n_ = ((c.m.r32(a_) as i32)).wrapping_sub(r_); c.m.w32(a_, (n_) as u32); };
                }
            }
        }
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        l_up_5 = (1i32).wrapping_neg();
        l_down_6 = (1i32).wrapping_neg();
        { let _ = D::call(c, fid::BtlAnim_AdvanceLoop, &[((p_chr_1) as u64), ((0i32) as i64 as u64)]); };
        's1: {
            let sv7: i32 = ((D::call(c, fid::BtlAnim_GetId, &[((p_chr_1) as u64)])) as i32);
            match sv7 {
                264i32 | 298i32 | 332i32 => {
                    l_up_5 = (((l_slot_3).wrapping_mul(34i32)).wrapping_add(193i32)).wrapping_add(10i32);
                    l_down_6 = (((l_slot_3).wrapping_mul(34i32)).wrapping_add(193i32)).wrapping_add(17i32);
                    break 's1;
                }
                364i32 => {
                    l_up_5 = 366i32;
                    l_down_6 = 368i32;
                    break 's1;
                }
                _ => {
                }
            }
        }
        if (((((((((((((l_up_5) >= (0i32)) as i32)) != 0) && (((((l_down_6) >= (0i32)) as i32)) != 0)) as i32)) != 0) && ((((!((((((D::call(c, fid::BtlSuper_GetFlags, &[((p_chr_1) as u64), ((l_slot_3) as i64 as u64)])) as i32)) & (4096i32))) != 0)) as i32)) != 0)) as i32)) != 0) {
            { let _ = D::call(c, fid::BtlAct_SetPitchMotion, &[((p_chr_1) as u64), ((l_up_5) as i64 as u64), ((l_down_6) as i64 as u64), ((1i32) as i64 as u64)]); };
        }
        { let _ = D::call(c, fid::BtlMove_Step, &[((p_chr_1) as u64), ((2i32) as i64 as u64), ((5i32) as i64 as u64), ((2i32) as i64 as u64), ((F32(0x00000000)).0 as u64), ((F32(0x3f6d0979)).0 as u64)]); };
        { let _ = D::call(c, fid::BtlMove_ApplyGravity, &[((p_chr_1) as u64)]); };
        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((187i32) as i64 as u64)]); };
        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((296i32) as i64 as u64)]); };
        { let _ = D::call(c, fid::BtlChar_SetFxBit, &[((p_chr_1) as u64), ((16i32) as i64 as u64)]); };
        { let _ = D::call(c, fid::ChrCam_AddShake, &[((p_chr_1) as u64), ((F32(0x40a00000)).0 as u64), ((F32(0x3dcccccc)).0 as u64)]); };
        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((230i32) as i64 as u64)]); };
        { let _ = D::call(c, fid::BtlChar_Vibrate, &[((p_chr_1) as u64), ((F32(0x3f800000)).0 as u64), ((F32(0x3dcccccc)).0 as u64)]); };
        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((210i32) as i64 as u64)]); };
        { let a1_ = { let a0_ = rt::fadd({ let a_ = ((((D::call(c, fid::BtlChar_GetPos, &[((p_chr_1) as u64)])) as u32)).wrapping_add(16)).wrapping_add(4); F32(c.m.r32(a_)) }, F32(0x40490fda)); F::BtlUtil_WrapAngle(c, a0_) }; { let _ = D::call(c, fid::BtlAct_PushAngle, &[((p_chr_1) as u64), ((a1_).0 as u64), ((F32(0x3f800000)).0 as u64), ((F32(0x40a00000)).0 as u64)]); } };
        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((66i32) as i64 as u64)]); };
        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((67i32) as i64 as u64)]); };
        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((68i32) as i64 as u64)]); };
        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((69i32) as i64 as u64)]); };
        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((70i32) as i64 as u64)]); };
        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((308i32) as i64 as u64)]); };
        if ((((((((((c.m.r32((p_chr_1).wrapping_add(2404)) as i32)) >= (16i32)) as i32)) != 0) && ((((D::call(c, fid::BtlInput_TestAction, &[((p_chr_1) as u64), ((51i32) as i64 as u64), ((1i32) as i64 as u64)])) as i32)) != 0)) as i32)) != 0) {
            { let a_ = (p_chr_1).wrapping_add(3660); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        }
    }
    if (((((p_phase_2) == (2i32)) as i32)) != 0) {
        if ((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((195i32) as i64 as u64)])) as i32)) != 0) {
            { let _ = D::call(c, fid::BtlAct_Request, &[((p_chr_1) as u64), ((11i32) as i64 as u64)]); };
        }
        if ((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((194i32) as i64 as u64)])) as i32)) != 0) {
            { let _ = D::call(c, fid::BtlAct_Request, &[((p_chr_1) as u64), ((260i32) as i64 as u64)]); };
        }
        if ((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((114i32) as i64 as u64)])) as i32)) != 0) {
            { let _ = D::call(c, fid::BtlAct_Request, &[((p_chr_1) as u64), (((l_slot_3).wrapping_add(299i32)) as i64 as u64)]); };
        }
    }
    if (((((p_phase_2) == (3i32)) as i32)) != 0) {
        { let _ = D::call(c, fid::BtlSuper_Leave, &[((p_chr_1) as u64), ((l_slot_3) as i64 as u64)]); };
        { let _ = D::call(c, fid::BtlChar_ClearFlagRange, &[((p_chr_1) as u64), ((((191i32) as u32)) as u64), ((((195i32) as u32)) as u64)]); };
        { let v_ = 0i32; let a_ = (p_chr_1).wrapping_add(3660); c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_ClashStruggleHandler", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_ClashLostHandler(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_ClashLostHandler", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let mut l_step_3: u32 = 0;
    let r_ = 'f: {
    l_step_3 = ((p_chr_1).wrapping_add(976)).wrapping_add((((2i32) as u32)).wrapping_mul(4));
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        { let _ = D::call(c, fid::BtlAnim_Play, &[((p_chr_1) as u64), ((370i32) as i64 as u64), ((F32(0x00000000)).0 as u64)]); };
        { let a2_ = ((c.m.r32(p_chr_1) as i32)).wrapping_add(33i32); { let _ = D::call(c, fid::ChrCam_RequestCut, &[((p_chr_1) as u64), ((1i32) as i64 as u64), ((a2_) as i64 as u64)]); } };
        if ((((({ let a_ = (((D::call(c, fid::BtlMember_GetActiveGauge, &[((p_chr_1) as u64)])) as u32)).wrapping_add(32); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
            { let r_ = 1i32; let a_ = ((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            { let _ = D::call(c, fid::BtlAct_SetFormCurrent, &[((p_chr_1) as u64)]); };
        }
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        if (((((((((c.m.r32(((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) & (1i32))) != 0) && ((((((c.m.r32((p_chr_1).wrapping_add(2404)) as i32)) == (1i32)) as i32)) != 0)) as i32)) != 0) {
            { let _ = D::call(c, fid::BtlChar_SavePlacement, &[((p_chr_1) as u64)]); };
            { let a0_ = (c.m.r32(p_chr_1) as i32); let a1_ = (c.m.r32(((p_chr_1).wrapping_add(4812)).wrapping_add(4)) as i32); let a2_ = (c.m.r32(((p_chr_1).wrapping_add(4812)).wrapping_add(8)) as i32); let a3_ = (c.m.r32(((p_chr_1).wrapping_add(4812)).wrapping_add(20)) as i32); let a4_ = (c.m.r32(((p_chr_1).wrapping_add(4812)).wrapping_add(24)) as i32); let a5_ = (c.m.r32(((p_chr_1).wrapping_add(4812)).wrapping_add(28)) as i32); let a6_ = (c.m.r32(((p_chr_1).wrapping_add(4812)).wrapping_add(32)) as i32); F::BtlChange_RequestChara(c, a0_, a1_, a2_, a3_, a4_, a5_, a6_) };
        }
        if ((((D::call(c, fid::BtlAnim_Advance, &[((p_chr_1) as u64), ((0i32) as i64 as u64)])) as i32)) != 0) {
            { let r_ = 2i32; let a_ = ((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        }
        if (((((c.m.r32(((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) & (2i32))) != 0) {
            if (((((c.m.r32(((p_chr_1).wrapping_add(976)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) & (1i32))) != 0) {
                's1: {
                    let sv4: i32 = (c.m.r32(l_step_3) as i32);
                    match sv4 {
                        0i32 => {
                            if (({ let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlChange_IsLoadedFor(c, a0_) }) != 0) {
                                { let _ = D::call(c, fid::BtlAnim_Request, &[((p_chr_1) as u64), ((370i32) as i64 as u64), ((F32(0x00000000)).0 as u64)]); };
                                { let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlChange_SetReady(c, a0_) };
                                { let a_ = l_step_3; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                            }
                            break 's1;
                        }
                        1i32 => {
                            F::BtlAnim_JumpToEnd(c, p_chr_1);
                            F::BtlAnim_EnableHandle(c, p_chr_1);
                            { let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlChange_SetDone(c, a0_) };
                            { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((245i32) as i64 as u64)]); };
                            { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((85i32) as i64 as u64)]); };
                            { let v_ = 1i32; let a_ = ((p_chr_1).wrapping_add(976)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                            break 's1;
                        }
                        _ => {
                        }
                    }
                }
            } else {
                { let v_ = 1i32; let a_ = ((p_chr_1).wrapping_add(976)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            }
        }
        { let _ = D::call(c, fid::BtlMove_Step, &[((p_chr_1) as u64), ((6i32) as i64 as u64), ((5i32) as i64 as u64), ((2i32) as i64 as u64), ((F32(0x00000000)).0 as u64), ((F32(0x3f6d0979)).0 as u64)]); };
        { let _ = D::call(c, fid::BtlMove_ApplyGravity, &[((p_chr_1) as u64)]); };
        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((293i32) as i64 as u64)]); };
        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((187i32) as i64 as u64)]); };
        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((296i32) as i64 as u64)]); };
        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((210i32) as i64 as u64)]); };
        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((149i32) as i64 as u64)]); };
        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((150i32) as i64 as u64)]); };
        { let a1_ = { let a0_ = rt::fadd({ let a_ = ((((D::call(c, fid::BtlChar_GetPos, &[((p_chr_1) as u64)])) as u32)).wrapping_add(16)).wrapping_add(4); F32(c.m.r32(a_)) }, F32(0x40490fda)); F::BtlUtil_WrapAngle(c, a0_) }; { let _ = D::call(c, fid::BtlAct_PushAngle, &[((p_chr_1) as u64), ((a1_).0 as u64), ((F32(0x3f800000)).0 as u64), ((F32(0x40a00000)).0 as u64)]); } };
        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((66i32) as i64 as u64)]); };
        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((67i32) as i64 as u64)]); };
        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((68i32) as i64 as u64)]); };
        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((69i32) as i64 as u64)]); };
        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((70i32) as i64 as u64)]); };
        { let v_ = 1i32; let a_ = (p_chr_1).wrapping_add(2416); c.m.w32(a_, (v_) as u32); };
    }
    if (((((p_phase_2) == (2i32)) as i32)) != 0) {
        if ((((((c.m.r32(((p_chr_1).wrapping_add(976)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) != (0i32)) as i32)) != 0) {
            { let _ = D::call(c, fid::BtlChar_SetHeldFlag, &[((p_chr_1) as u64), ((29i32) as i64 as u64)]); };
            { let _ = D::call(c, fid::BtlChar_SetHeldFlag, &[((p_chr_1) as u64), ((30i32) as i64 as u64)]); };
            { let _ = D::call(c, fid::BtlAct_Request, &[((p_chr_1) as u64), ((217i32) as i64 as u64)]); };
        }
    }
    if (((((p_phase_2) == (3i32)) as i32)) != 0) {
        { let _ = D::call(c, fid::BtlChar_ClearFlagRange, &[((p_chr_1) as u64), ((((191i32) as u32)) as u64), ((((195i32) as u32)) as u64)]); };
        F::BtlChar_AddStageTimer(c, F32(0x40800000));
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_ClashLostHandler", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_SuperLaunchedUpHandler(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_SuperLaunchedUpHandler", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let mut l_slot_3: i32 = 0;
    let r_ = 'f: {
    l_slot_3 = ((D::call(c, fid::BtlAct_GetCurrentClass, &[((p_chr_1) as u64)])) as i32);
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        { let _ = D::call(c, fid::BtlAnim_Play, &[((p_chr_1) as u64), (((((l_slot_3).wrapping_mul(34i32)).wrapping_add(193i32)).wrapping_add(33i32)) as i64 as u64), ((F32(0x00000000)).0 as u64)]); };
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        if ((((D::call(c, fid::BtlAnim_Advance, &[((p_chr_1) as u64), ((0i32) as i64 as u64)])) as i32)) != 0) {
            { let _ = D::call(c, fid::BtlAct_Request, &[((p_chr_1) as u64), ((208i32) as i64 as u64)]); };
        }
        { let _ = D::call(c, fid::BtlMove_Step, &[((p_chr_1) as u64), ((6i32) as i64 as u64), ((5i32) as i64 as u64), ((2i32) as i64 as u64), ((F32(0x00000000)).0 as u64), ((F32(0x3f6d0979)).0 as u64)]); };
        F::BtlMove_MoveVertical(c, p_chr_1, F32(0xc15e38e2), F32(0x461c4000));
        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((213i32) as i64 as u64)]); };
        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((149i32) as i64 as u64)]); };
        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((150i32) as i64 as u64)]); };
        { let _ = D::call(c, fid::BtlChar_SetFxBit, &[((p_chr_1) as u64), ((59i32) as i64 as u64)]); };
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_SuperLaunchedUpHandler", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_ChangeRandomCharaHandler(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_ChangeRandomCharaHandler", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let mut l_step_3: u32 = 0;
    let r_ = 'f: {
    l_step_3 = ((p_chr_1).wrapping_add(976)).wrapping_add((((2i32) as u32)).wrapping_mul(4));
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        's1: {
            let sv4: i32 = (c.m.r32(l_step_3) as i32);
            match sv4 {
                0i32 => {
                    { let _ = D::call(c, fid::BtlChar_SavePlacement, &[((p_chr_1) as u64)]); };
                    F::BtlAct_SetFormRandom(c, p_chr_1);
                    { let a0_ = (c.m.r32(p_chr_1) as i32); let a1_ = (c.m.r32(((p_chr_1).wrapping_add(4812)).wrapping_add(4)) as i32); let a2_ = (c.m.r32(((p_chr_1).wrapping_add(4812)).wrapping_add(8)) as i32); let a3_ = (c.m.r32(((p_chr_1).wrapping_add(4812)).wrapping_add(20)) as i32); let a4_ = (c.m.r32(((p_chr_1).wrapping_add(4812)).wrapping_add(24)) as i32); let a5_ = (c.m.r32(((p_chr_1).wrapping_add(4812)).wrapping_add(28)) as i32); let a6_ = (c.m.r32(((p_chr_1).wrapping_add(4812)).wrapping_add(32)) as i32); F::BtlChange_RequestChara(c, a0_, a1_, a2_, a3_, a4_, a5_, a6_) };
                    { let a_ = l_step_3; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    break 's1;
                }
                1i32 => {
                    if (({ let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlChange_IsLoadedFor(c, a0_) }) != 0) {
                        { let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlChange_SetReady(c, a0_) };
                        { let _ = D::call(c, fid::BtlAnim_Request, &[((p_chr_1) as u64), ((0i32) as i64 as u64), ((F32(0x00000000)).0 as u64)]); };
                        { let a_ = l_step_3; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    }
                    break 's1;
                }
                2i32 => {
                    { let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlChange_SetDone(c, a0_) };
                    { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((245i32) as i64 as u64)]); };
                    { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((85i32) as i64 as u64)]); };
                    { let _ = D::call(c, fid::BtlAct_Request, &[((p_chr_1) as u64), ((11i32) as i64 as u64)]); };
                    break 's1;
                }
                _ => {
                }
            }
        }
        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((293i32) as i64 as u64)]); };
        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((310i32) as i64 as u64)]); };
        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((66i32) as i64 as u64)]); };
        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((67i32) as i64 as u64)]); };
        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((68i32) as i64 as u64)]); };
        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((69i32) as i64 as u64)]); };
        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((70i32) as i64 as u64)]); };
        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((187i32) as i64 as u64)]); };
        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((63i32) as i64 as u64)]); };
        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((296i32) as i64 as u64)]); };
        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((27i32) as i64 as u64)]); };
        { let _ = D::call(c, fid::BtlChar_SetFlag, &[((p_chr_1) as u64), ((26i32) as i64 as u64)]); };
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_ChangeRandomCharaHandler", &PROBE_N, &[], true); }
    r_
}

pub fn BtlActThrow_SetupDamage(c: &mut Ctx, mut p_chr_1: u32, mut p_extraHit_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlActThrow_SetupDamage", &PROBE_N, &[(p_chr_1) as u32, (p_extraHit_2) as u32], false); }
    let mut l_obj_3: u32 = 0;
    let mut l_damage_4: i32 = 0;
    let mut l_rem_5: i32 = 0;
    let mut l_immune_6: i32 = 0;
    let mut l_drainKi_7: i32 = 0;
    let mut l_drainHealth_8: i32 = 0;
    let mut l_from_9: i32 = 0;
    let mut l_to_10: i32 = 0;
    let mut l_span_11: i32 = 0;
    let mut l_up_12: i32 = 0;
    let r_ = 'f: {
    l_obj_3 = ((D::call(c, fid::BtlChar_GetObj, &[((p_chr_1) as u64)])) as u32);
    { let v_ = ((D::call(c, fid::BtlObjAnim_QueryEvent, &[((l_obj_3) as u64), ((((8589934592i64) as u64)) as u64), ((0i32) as i64 as u64), ((3i32) as i64 as u64)])) as i32); let a_ = ((p_chr_1).wrapping_add(3476)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
    if ((p_extraHit_2) != 0) {
        { let a_ = ((p_chr_1).wrapping_add(3476)).wrapping_add(12); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
    }
    l_damage_4 = F::BtlAtk_GetDamage(c, p_chr_1);
    { let v_ = l_damage_4; let a_ = (p_chr_1).wrapping_add(3476); c.m.w32(a_, (v_) as u32); };
    { let v_ = l_damage_4; let a_ = ((p_chr_1).wrapping_add(3476)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
    { let v_ = rt::div_i32(l_damage_4, ((c.m.r32(((p_chr_1).wrapping_add(3476)).wrapping_add(12)) as i32)).wrapping_add(5i32)); let a_ = ((p_chr_1).wrapping_add(3476)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
    { let v_ = 268435456i32; let a_ = ((p_chr_1).wrapping_add(3476)).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
    if (((((l_damage_4) <= (0i32)) as i32)) != 0) {
        { let v_ = 268435968i32; let a_ = ((p_chr_1).wrapping_add(3476)).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
    }
    l_rem_5 = rt::rem_i32((c.m.r32(((p_chr_1).wrapping_add(3476)).wrapping_add(8)) as i32), 10i32);
    if ((l_rem_5) != 0) {
        l_up_12 = (((c.m.r32(((p_chr_1).wrapping_add(3476)).wrapping_add(8)) as i32)).wrapping_sub(l_rem_5)).wrapping_add(10i32);
        if ((((((l_up_12).wrapping_mul((c.m.r32(((p_chr_1).wrapping_add(3476)).wrapping_add(12)) as i32))) < ((c.m.r32((p_chr_1).wrapping_add(3476)) as i32))) as i32)) != 0) {
            { let v_ = l_up_12; let a_ = ((p_chr_1).wrapping_add(3476)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        }
    }
    { let v_ = 0i32; let a_ = ((p_chr_1).wrapping_add(3476)).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
    { let v_ = 0i32; let a_ = ((p_chr_1).wrapping_add(3476)).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
    { let v_ = 0i32; let a_ = ((p_chr_1).wrapping_add(3476)).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
    { let v_ = 0i32; let a_ = ((p_chr_1).wrapping_add(3476)).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
    { let v_ = 0i32; let a_ = ((p_chr_1).wrapping_add(3476)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
    { let v_ = 0i32; let a_ = ((p_chr_1).wrapping_add(3476)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
    { let v_ = (1i32).wrapping_neg(); let a_ = ((p_chr_1).wrapping_add(3476)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
    { let v_ = (1i32).wrapping_neg(); let a_ = ((p_chr_1).wrapping_add(3476)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
    l_immune_6 = ((((D::call(c, fid::BtlOpp_GetParamWord0, &[((p_chr_1) as u64)])) as i32)) & (128i32));
    if ((((!((l_immune_6) != 0)) as i32)) != 0) {
        l_drainHealth_8 = F::BtlAtk_GetThrowParamC(c, p_chr_1);
        l_drainKi_7 = F::BtlAtk_GetThrowParamE(c, p_chr_1);
        if (((((((((l_drainHealth_8) > (0i32)) as i32)) != 0) || (((((l_drainKi_7) > (0i32)) as i32)) != 0)) as i32)) != 0) {
            l_from_9 = ((D::call(c, fid::BtlObjAnim_QueryEvent, &[((l_obj_3) as u64), ((((131072i32) as u64)) as u64), ((0i32) as i64 as u64), ((0i32) as i64 as u64)])) as i32);
            l_to_10 = ((D::call(c, fid::BtlObjAnim_QueryEvent, &[((l_obj_3) as u64), ((((262144i32) as u64)) as u64), ((0i32) as i64 as u64), ((2i32) as i64 as u64)])) as i32);
            l_span_11 = rt::div_i32((l_to_10).wrapping_sub(l_from_9), 2i32);
            if (((((l_span_11) <= (l_immune_6)) as i32)) != 0) {
                l_span_11 = 1i32;
            }
            { let v_ = l_drainHealth_8; let a_ = ((p_chr_1).wrapping_add(3476)).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
            { let v_ = l_drainHealth_8; let a_ = ((p_chr_1).wrapping_add(3476)).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
            { let v_ = (rt::div_i32(l_drainHealth_8, l_span_11)).wrapping_add(1i32); let a_ = ((p_chr_1).wrapping_add(3476)).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
            { let v_ = l_drainKi_7; let a_ = ((p_chr_1).wrapping_add(3476)).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
            { let v_ = l_drainKi_7; let a_ = ((p_chr_1).wrapping_add(3476)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
            { let v_ = (rt::div_i32(l_drainKi_7, l_span_11)).wrapping_add(1i32); let a_ = ((p_chr_1).wrapping_add(3476)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
            { let v_ = l_from_9; let a_ = ((p_chr_1).wrapping_add(3476)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            { let v_ = l_to_10; let a_ = ((p_chr_1).wrapping_add(3476)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlActThrow_SetupDamage", &PROBE_N, &[], true); }
    r_
}

pub fn BtlActThrow_TurnByRootYaw(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlActThrow_TurnByRootYaw", &PROBE_N, &[(p_chr_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_anim_2: u32 = 0;
    let mut l_track_3: u32 = 0;
    let mut l_yaw_4: F32 = F32(0);
    let mut l_count_5: i32 = 0;
    let mut l_packed_6: u64 = 0;
    let mut l_key_7: u32 = 0;
    let mut l_key_8: u32 = 0;
    let r_ = 'f: {
        l_anim_2 = { let a_ = (((D::call(c, fid::BtlChar_GetObj, &[((p_chr_1) as u64)])) as u32)).wrapping_add(2880); c.m.r32(a_) };
        if (((((((((l_anim_2) != (((0i32) as u32))) as i32)) != 0) && (((((((c.m.r16((l_anim_2).wrapping_add(6))) as i32)) != (0i32)) as i32)) != 0)) as i32)) != 0) {
            l_yaw_4 = F32(0x00000000);
            l_track_3 = (l_anim_2).wrapping_add((((((c.m.r16((l_anim_2).wrapping_add(6))) as i32)) as u32)).wrapping_mul(4));
            l_count_5 = ((c.m.r16((l_track_3).wrapping_add(2))) as i32);
            if (((((l_count_5) > (0i32)) as i32)) != 0) {
                if ((((((((c.m.r16(l_track_3)) as i32)) & (1i32))) as u16)) != 0) {
                    l_key_7 = (l_track_3).wrapping_add(4);
                    l_packed_6 = (((c.m.r32((l_key_7).wrapping_add((((0i32) as u32)).wrapping_mul(4)))) as u64)).wrapping_add((((c.m.r32((l_key_7).wrapping_add((((1i32) as u32)).wrapping_mul(4)))) as u64)).wrapping_shl((32i32) as u32));
                } else {
                    l_key_8 = (l_track_3).wrapping_add(4);
                    l_packed_6 = (((c.m.r32((l_key_8).wrapping_add((((4i32) as u32)).wrapping_mul(4)))) as u64)).wrapping_add((((c.m.r32((l_key_8).wrapping_add((((5i32) as u32)).wrapping_mul(4)))) as u64)).wrapping_shl((32i32) as u32));
                }
                F::Quat_Unpack(c, fp, l_packed_6);
                F::Quat_ToEuler(c, fp.wrapping_add(16), fp);
                l_yaw_4 = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4)));
            }
            { let a1_ = { let a0_ = rt::fsub({ let a_ = (((D::call(c, fid::BtlChar_GetPos, &[((p_chr_1) as u64)])) as u32)).wrapping_add(148); F32(c.m.r32(a_)) }, l_yaw_4); F::BtlUtil_WrapAngle(c, a0_) }; { let _ = D::call(c, fid::BtlMove_SetHeading, &[((p_chr_1) as u64), ((a1_).0 as u64), ((F32(0x00000000)).0 as u64)]); } };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlActThrow_TurnByRootYaw", &PROBE_N, &[], true); }
    r_
}
