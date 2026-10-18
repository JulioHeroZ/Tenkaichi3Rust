#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn BtlFx_HandleImpactEvents(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFx_HandleImpactEvents", &PROBE_N, &[(p_chr_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let mut l_time_2: F32 = F32(0);
    let mut l_work_3: i32 = 0;
    let mut l_on_4: i32 = 0;
    let mut l_kind_5: i32 = 0;
    let mut l_altKind_6: i32 = 0;
    let mut l_mask_7: i32 = 0;
    let mut l_sound_8: i32 = 0;
    let mut l_strength_9: F32 = F32(0);
    let mut l_obj_10: u32 = 0;
    let mut l_other_11: u32 = 0;
    let mut l_ok_12: i32 = 0;
    let r_ = 'f: {
        l_time_2 = F32(0x00000000);
        l_work_3 = 0i32;
        l_on_4 = 0i32;
        l_kind_5 = 0i32;
        l_altKind_6 = 0i32;
        l_mask_7 = 0i32;
        l_sound_8 = 0i32;
        l_strength_9 = l_time_2;
        l_obj_10 = F::BtlChar_GetObj(c, p_chr_1);
        if ((F::BtlAnim_TestAttr(c, p_chr_1, ((256i32) as u64))) != 0) {
            l_kind_5 = 15i32;
            if (((((F::BtlAct_GetCurrentClass(c, p_chr_1)) < (0i32)) as i32)) != 0) {
                l_kind_5 = 0i32;
            }
            l_strength_9 = F32(0x40a00000);
            l_sound_8 = (F::BtlChar_FrameMod(c, 2i32)).wrapping_add(2i32);
            l_time_2 = F32(0x3dcccccc);
            l_on_4 = 1i32;
            l_mask_7 = 256i32;
            l_altKind_6 = 9i32;
            l_work_3 = 77i32;
        }
        if ((F::BtlAnim_TestAttr(c, p_chr_1, ((128i32) as u64))) != 0) {
            l_kind_5 = 3i32;
            if ((((!(((((F::BtlAct_GetCurrentClass(c, p_chr_1)) < (0i32)) as i32)) != 0)) as i32)) != 0) {
                l_kind_5 = 16i32;
            }
            l_strength_9 = F32(0x41200000);
            l_time_2 = F32(0x3e4ccccc);
            l_on_4 = 1i32;
            l_mask_7 = 128i32;
            l_altKind_6 = 10i32;
            l_sound_8 = 73i32;
            l_work_3 = 54i32;
        }
        if ((l_on_4) != 0) {
            if ((((((((!(({ let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); F::BtlCharApi_IsInRushSequence(c, a0_) }) != 0)) as i32)) != 0) && (({ let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); F::BtlCharApi_HasWeaponOut(c, a0_) }) != 0)) as i32)) != 0) {
                l_sound_8 = l_work_3;
                l_kind_5 = l_altKind_6;
            }
            if ((((c.m.r32((l_obj_10).wrapping_add(2624))) & (((2i32) as u32)))) != 0) {
                if ((((!((F::BtlAnim_TestAttr(c, p_chr_1, ((134217728i32) as u64))) != 0)) as i32)) != 0) {
                    { let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); let a1_ = { let a0_ = ((((F::BtlObjAnim_GetEventArg(c, l_obj_10, ((l_mask_7) as u64))) as u64)) as i32); F::BtlObjAnim_MaskToNode(c, ((((a0_) as i64 as u64)) as u32)) }; F::BtlCharApi_GetNodePos(c, a0_, a1_, fp) };
                    { let v_ = F32(0x3f800000); let a_ = (fp).wrapping_add(12); c.m.w32(a_, (v_).0); };
                    { let v_ = { let a0_ = { let a_ = ((F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(16)).wrapping_add(4); F32(c.m.r32(a_)) }; F::Mathf_Sin(c, a0_) }; let a_ = (fp).wrapping_add(16); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(0x00000000); let a_ = ((fp).wrapping_add(16)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = { let a0_ = { let a_ = ((F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(16)).wrapping_add(4); F32(c.m.r32(a_)) }; F::Mathf_Cos(c, a0_) }; let a_ = ((fp).wrapping_add(16)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                    { let v_ = l_kind_5; let a_ = (fp).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
                    { let v_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); let a_ = (fp).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                    { let v_ = F32(0x00000000); let a_ = ((fp).wrapping_add(16)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                    { let v_ = 0i32; let a_ = (fp).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
                    { let _ = F::EftImpact_SpawnHit(c, fp); };
                }
                F::ChrCam_AddShake(c, p_chr_1, l_strength_9, l_time_2);
            }
            F::BtlCharSnd_PlayCommon(c, p_chr_1, l_sound_8);
            if (((({ let a0_ = F::BtlAnim_GetId(c, p_chr_1); ((((F::BtlAnim_GetFlags(c, ((((a0_) as i64 as u64)) as u32))) as u64)) as i32) }) & (16i32))) != 0) {
                l_work_3 = ((p_chr_1) as i32);
                l_other_11 = { let a0_ = F::BtlOpp_GetPlayer(c, ((l_work_3) as u32)); F::BtlChar_Get(c, a0_) };
            } else {
                l_other_11 = p_chr_1;
                l_work_3 = (({ let a0_ = F::BtlOpp_GetPlayer(c, l_other_11); F::BtlChar_Get(c, a0_) }) as i32);
            }
            l_ok_12 = ((!((F::BtlChar_TestFlag(c, l_other_11, ((((315i32) as i64 as u64)) as u32))) != 0)) as i32);
            if ((F::BtlChar_IsFrozen(c, ((l_work_3) as u32))) != 0) {
                l_ok_12 = 0i32;
            }
            if ((l_ok_12) != 0) {
                F::BtlChar_PlayVoice(c, ((l_work_3) as u32), 1i32);
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlFx_HandleImpactEvents", &PROBE_N, &[], true); }
    r_
}

pub fn BtlFx_HandleEvent1(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFx_HandleEvent1", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_obj_2: u32 = 0;
    let r_ = 'f: {
    l_obj_2 = F::BtlChar_GetObj(c, p_chr_1);
    if ((F::BtlAnim_TestAttr(c, p_chr_1, ((1i32) as u64))) != 0) {
        if (((((F::BtlAtk_GetId(c, p_chr_1)) != ((1i32).wrapping_neg())) as i32)) != 0) {
            if ((((((((F::BtlAtk_GetFlags(c, p_chr_1)) as u64)) as i32)) & (524288i32))) != 0) {
                if (((((F::BtlObjAnim_QueryEvent(c, l_obj_2, ((1i32) as u64), 0i32, ((((5i32) as i64 as u64)) as u32))) == (1i32)) as i32)) != 0) {
                    F::BtlChar_SetFxBit(c, p_chr_1, 27i32);
                }
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlFx_HandleEvent1", &PROBE_N, &[], true); }
    r_
}

pub fn BtlFx_HandleFlag30Events(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFx_HandleFlag30Events", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    if ((F::BtlAnim_TestAttr(c, p_chr_1, ((33554432i32) as u64))) != 0) {
        F::BtlChar_SetHeldFlag(c, p_chr_1, ((((48i32) as i64 as u64)) as u32));
    }
    if ((F::BtlAnim_TestAttr(c, p_chr_1, ((67108864i32) as u64))) != 0) {
        F::BtlChar_ClearFlag(c, p_chr_1, ((((48i32) as i64 as u64)) as u32));
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlFx_HandleFlag30Events", &PROBE_N, &[], true); }
    r_
}

pub fn BtlFx_PlaySwingSound(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFx_PlaySwingSound", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_sound_2: i32 = 0;
    let r_ = 'f: {
    if ((((!((F::BtlChar_TestFlag(c, p_chr_1, ((((145i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) {
        if ((F::BtlAnim_TestAttr(c, p_chr_1, ((2i32) as u64))) != 0) {
            if ((((((((!((F::BtlChar_TestFlag(c, p_chr_1, ((((96i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) || ((F::BtlChar_TestFlag(c, p_chr_1, ((((92i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) {
                l_sound_2 = { let a0_ = F::BtlAtk_GetHitSoundLevel(c, p_chr_1); F::BtlChar_GetHitSoundLine(c, a0_) };
                if (((((l_sound_2) >= (0i32)) as i32)) != 0) {
                    F::BtlCharSnd_PlayCommon(c, p_chr_1, l_sound_2);
                }
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlFx_PlaySwingSound", &PROBE_N, &[], true); }
    r_
}

pub fn BtlFx_SpawnPartFxEvent8(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFx_SpawnPartFxEvent8", &PROBE_N, &[(p_chr_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_obj_2: u32 = 0;
    let mut l_bits_3: i32 = 0;
    let mut l_i_4: i32 = 0;
    let mut l_part_5: i32 = 0;
    let mut l_bit_6: i32 = 0;
    let r_ = 'f: {
        l_obj_2 = F::BtlChar_GetObj(c, p_chr_1);
        if ((F::BtlAnim_TestAttr(c, p_chr_1, ((8i32) as u64))) != 0) {
            l_bits_3 = ((((F::BtlObjAnim_GetEventArg(c, l_obj_2, ((8i32) as u64))) as u64)) as i32);
            {
                l_i_4 = 0i32;
                'l1: loop {
                    if !(((((l_i_4) < (19i32)) as i32)) != 0) { break 'l1; }
                    'c2: {
                        l_bit_6 = ((((((l_bits_3) as u32)) & ((1u32).wrapping_shl((l_i_4) as u32)))) as i32);
                        if (((((l_bit_6) != (0i32)) as i32)) != 0) {
                            l_part_5 = F::BtlObjAnim_MaskToNode(c, ((((l_bit_6) as i64 as u64)) as u32));
                            F::BtlObj_GetNodeVelocity(c, l_obj_2, l_part_5, fp);
                            { let v_ = F32(0x00000000); let a_ = (fp).wrapping_add(12); c.m.w32(a_, (v_).0); };
                            F::Mtx_MulVec4(c, fp, (l_obj_2).wrapping_add(2464), fp);
                            { let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); { let _ = F::EftSpdLine_SpawnPartStreaks(c, a0_, fp, l_part_5); } };
                        }
                    }
                    { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
                }
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlFx_SpawnPartFxEvent8", &PROBE_N, &[], true); }
    r_
}

pub fn BtlFx_SpawnPartFlashes(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFx_SpawnPartFlashes", &PROBE_N, &[(p_chr_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let mut l_obj_2: u32 = 0;
    let mut l_bits_3: i32 = 0;
    let mut l_i_4: i32 = 0;
    let mut l_bit_5: i32 = 0;
    let r_ = 'f: {
        l_obj_2 = F::BtlChar_GetObj(c, p_chr_1);
        if ((F::BtlAnim_TestAttr(c, p_chr_1, ((16777216i32) as u64))) != 0) {
            l_bits_3 = ((((F::BtlObjAnim_GetEventArg(c, l_obj_2, ((16777216i32) as u64))) as u64)) as i32);
            if (((((l_bits_3) != (0i32)) as i32)) != 0) {
                { let v_ = { let a0_ = { let a_ = ((F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(16)).wrapping_add(4); F32(c.m.r32(a_)) }; F::Mathf_Sin(c, a0_) }; let a_ = (fp).wrapping_add(16); c.m.w32(a_, (v_).0); };
                { let v_ = F32(0x00000000); let a_ = ((fp).wrapping_add(16)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                { let v_ = { let a0_ = { let a_ = ((F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(16)).wrapping_add(4); F32(c.m.r32(a_)) }; F::Mathf_Cos(c, a0_) }; let a_ = ((fp).wrapping_add(16)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                { let v_ = 12i32; let a_ = (fp).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
                { let v_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); let a_ = (fp).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = F32(0x00000000); let a_ = ((fp).wrapping_add(16)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                { let v_ = 0i32; let a_ = (fp).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
                {
                    l_i_4 = 0i32;
                    'l1: loop {
                        if !(((((l_i_4) < (32i32)) as i32)) != 0) { break 'l1; }
                        'c2: {
                            l_bit_5 = ((((((l_bits_3) as u32)) & ((1u32).wrapping_shl((l_i_4) as u32)))) as i32);
                            if (((((l_bit_5) != (0i32)) as i32)) != 0) {
                                { let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); let a1_ = F::BtlObjAnim_MaskToNode(c, ((((l_bit_5) as i64 as u64)) as u32)); F::BtlCharApi_GetNodePos(c, a0_, a1_, fp) };
                                { let v_ = F32(0x3f800000); let a_ = (fp).wrapping_add(12); c.m.w32(a_, (v_).0); };
                                { let _ = F::EftImpact_SpawnHit(c, fp); };
                            }
                        }
                        { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
                    }
                }
                F::BtlCharSnd_PlayCommon(c, p_chr_1, 75i32);
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlFx_SpawnPartFlashes", &PROBE_N, &[], true); }
    r_
}

pub fn BtlFx_PushBackOnEvent400(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFx_PushBackOnEvent400", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    if (({ let a0_ = F::BtlAct_GetCurrent(c, p_chr_1); F::BtlAct_IsTechniqueId(c, a0_) }) != 0) {
        if (({ let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); F::BtlCharApi_IsInRushSequence(c, a0_) }) != 0) {
            if ((((!(((({ let a1_ = F::BtlAct_GetCurrentClass(c, p_chr_1); ((((F::BtlSuper_GetFlagsA(c, p_chr_1, a1_)) as u64)) as i32) }) & (16777216i32))) != 0)) as i32)) != 0) {
                if ((F::BtlAnim_TestAttr(c, p_chr_1, ((1024i32) as u64))) != 0) {
                    F::BtlChar_SetHeldFlag(c, p_chr_1, ((((159i32) as i64 as u64)) as u32));
                }
                if ((F::BtlChar_TestFlag(c, p_chr_1, ((((159i32) as i64 as u64)) as u32))) != 0) {
                    { let a1_ = { let a0_ = rt::fadd({ let a_ = ((F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(16)).wrapping_add(4); F32(c.m.r32(a_)) }, F32(0x40490fda)); F::BtlUtil_WrapAngle(c, a0_) }; F::BtlAct_PushAngle(c, p_chr_1, a1_, F32(0x3f800000), F32(0x40a00000)) };
                    if ((F::BtlPartner_IsActive(c, p_chr_1)) != 0) {
                        { let a1_ = { let a0_ = rt::fadd({ let a_ = ((F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(16)).wrapping_add(4); F32(c.m.r32(a_)) }, F32(0x40490fda)); F::BtlUtil_WrapAngle(c, a0_) }; F::BtlPartner_PushAngle(c, p_chr_1, a1_, F32(0x3f800000), F32(0x40a00000)) };
                    }
                }
            }
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlFx_PushBackOnEvent400", &PROBE_N, &[], true); }
    r_
}

pub fn BtlFx_SetObjMaskOnEvent400(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFx_SetObjMaskOnEvent400", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_obj_2: u32 = 0;
    let r_ = 'f: {
    l_obj_2 = F::BtlChar_GetObj(c, p_chr_1);
    if (({ let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); F::BtlCharApi_IsInSkill(c, a0_) }) != 0) {
        if ((((({ let a1_ = F::BtlAct_GetCurrentClass(c, p_chr_1); F::BtlSkill_GetId(c, p_chr_1, a1_) }) == (24i32)) as i32)) != 0) {
            if ((F::BtlAnim_TestAttr(c, p_chr_1, ((1024i32) as u64))) != 0) {
                F::BtlObj_SetColorMode(c, l_obj_2, 2i32, 1i32);
            }
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlFx_SetObjMaskOnEvent400", &PROBE_N, &[], true); }
    r_
}

pub fn btl_char_fx_c__Fx_ShakeCam(c: &mut Ctx, mut p_chr_1: u32, mut p_strength_2: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "btl_char_fx_c__Fx_ShakeCam", &PROBE_N, &[(p_chr_1) as u32, (p_strength_2).0], false); }
    let r_ = 'f: {
    F::ChrCam_AddShake(c, p_chr_1, p_strength_2, F32(0x3e999999));
       
    };
    if rt::probing() { crate::host::harness::probe(c, "btl_char_fx_c__Fx_ShakeCam", &PROBE_N, &[], true); }
    r_
}

pub fn BtlFx_ShakeCamOnEvent(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFx_ShakeCamOnEvent", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    if ((F::BtlAnim_TestAttr(c, p_chr_1, ((70368744177664i64) as u64))) != 0) {
        F::btl_char_fx_c__Fx_ShakeCam(c, p_chr_1, F32(0x40400000));
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlFx_ShakeCamOnEvent", &PROBE_N, &[], true); }
    r_
}

pub fn btl_char_fx_c__Fx_Vibrate(c: &mut Ctx, mut p_chr_1: u32, mut p_power_2: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "btl_char_fx_c__Fx_Vibrate", &PROBE_N, &[(p_chr_1) as u32, (p_power_2).0], false); }
    let r_ = 'f: {
    F::BtlChar_Vibrate(c, p_chr_1, p_power_2, F32(0x3e4ccccc));
       
    };
    if rt::probing() { crate::host::harness::probe(c, "btl_char_fx_c__Fx_Vibrate", &PROBE_N, &[], true); }
    r_
}

pub fn BtlFx_VibrateOnEvent(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFx_VibrateOnEvent", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    if ((F::BtlAnim_TestAttr(c, p_chr_1, ((35184372088832i64) as u64))) != 0) {
        F::btl_char_fx_c__Fx_Vibrate(c, p_chr_1, F32(0x3f800000));
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlFx_VibrateOnEvent", &PROBE_N, &[], true); }
    r_
}

pub fn BtlFxObj_HandleFlagEvents(c: &mut Ctx, mut p_obj_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFxObj_HandleFlagEvents", &PROBE_N, &[(p_obj_1) as u32], false); }
    let r_ = 'f: {
    if ((F::BtlObjAnim_TestEvent(c, p_obj_1, ((4398046511104i64) as u64))) != 0) {
        { let a0_ = (c.m.r32((p_obj_1).wrapping_add(16)) as i32); { let _ = F::EftShotFx_Start(c, a0_, ((((0i32) as i64 as u64)) as u8)); } };
    }
    if ((F::BtlObjAnim_TestEvent(c, p_obj_1, ((32i32) as u64))) != 0) {
        { let r_ = ((32i32) as u32); let a_ = (p_obj_1).wrapping_add(2624); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
    }
    if ((F::BtlObjAnim_TestEvent(c, p_obj_1, ((64i32) as u64))) != 0) {
        { let r_ = (((!(32i32))) as u32); let a_ = (p_obj_1).wrapping_add(2624); let n_ = ((c.m.r32(a_)) & (r_)); c.m.w32(a_, n_); };
    }
    if ((F::BtlObjAnim_TestEvent(c, p_obj_1, ((8796093022208i64) as u64))) != 0) {
        { let r_ = ((64i32) as u32); let a_ = (p_obj_1).wrapping_add(2624); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
    }
    if ((F::BtlObjAnim_TestEvent(c, p_obj_1, ((17592186044416i64) as u64))) != 0) {
        { let r_ = (((!(64i32))) as u32); let a_ = (p_obj_1).wrapping_add(2624); let n_ = ((c.m.r32(a_)) & (r_)); c.m.w32(a_, n_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlFxObj_HandleFlagEvents", &PROBE_N, &[], true); }
    r_
}

pub fn BtlFxObj_PlayEventSounds(c: &mut Ctx, mut p_obj_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFxObj_PlayEventSounds", &PROBE_N, &[(p_obj_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_id_2: i32 = 0;
    let r_ = 'f: {
        { let a0_ = (c.m.r32((p_obj_1).wrapping_add(16)) as i32); F::BtlCharApi_GetNodePos(c, a0_, 3i32, fp) };
        if ((F::BtlObjAnim_TestEvent(c, p_obj_1, ((16i32) as u64))) != 0) {
            { let a2_ = (F::BtlChar_FrameMod(c, 3i32)).wrapping_add(40i32); F::BtlCharSnd_RequestAt(c, fp, 0i32, a2_, F32(0x43480000), F32(0x44bb8000)) };
        }
        if ((F::BtlObjAnim_TestEvent(c, p_obj_1, ((268435456i32) as u64))) != 0) {
            F::BtlCharSnd_RequestAt(c, fp, 0i32, 30i32, F32(0x43480000), F32(0x44bb8000));
        }
        if ((F::BtlObjAnim_TestEvent(c, p_obj_1, ((536870912i32) as u64))) != 0) {
            { let a2_ = F::BtlChar_FrameMod(c, 2i32); F::BtlCharSnd_RequestAt(c, fp, 0i32, a2_, F32(0x43480000), F32(0x44bb8000)) };
        }
        if ((F::BtlObjAnim_TestEvent(c, p_obj_1, ((1073741824i32) as u64))) != 0) {
            { let a2_ = (F::BtlChar_FrameMod(c, 3i32)).wrapping_add(6i32); F::BtlCharSnd_RequestAt(c, fp, 0i32, a2_, F32(0x43480000), F32(0x44bb8000)) };
        }
        if ((F::BtlObjAnim_TestEvent(c, p_obj_1, ((34359738368i64) as u64))) != 0) {
            F::BtlCharSnd_RequestAt(c, fp, 0i32, 32i32, F32(0x43480000), F32(0x44bb8000));
        }
        if ((F::BtlObjAnim_TestEvent(c, p_obj_1, ((68719476736i64) as u64))) != 0) {
            F::BtlCharSnd_RequestAt(c, fp, 0i32, 77i32, F32(0x43480000), F32(0x44bb8000));
        }
        if ((F::BtlObjAnim_TestEvent(c, p_obj_1, ((137438953472i64) as u64))) != 0) {
            F::BtlCharSnd_RequestAt(c, fp, 0i32, 37i32, F32(0x43480000), F32(0x44bb8000));
        }
        if ((F::BtlObjAnim_TestEvent(c, p_obj_1, ((274877906944i64) as u64))) != 0) {
            l_id_2 = 51i32;
            if (((((c.m.r32((p_obj_1).wrapping_add(2332))) != (((0i32) as u32))) as i32)) != 0) {
                if (((({ let a_ = (c.m.r32((p_obj_1).wrapping_add(2332))).wrapping_add(16); c.m.r32(a_) }) & (((32768i32) as u32)))) != 0) {
                    l_id_2 = 11i32;
                }
            }
            F::BtlCharSnd_RequestAt(c, fp, 0i32, l_id_2, F32(0x43480000), F32(0x44bb8000));
        }
        if ((F::BtlObjAnim_TestEvent(c, p_obj_1, ((1099511627776i64) as u64))) != 0) {
            F::BtlCharSnd_RequestAt(c, fp, 0i32, 48i32, F32(0x43480000), F32(0x44bb8000));
        }
        if ((F::BtlObjAnim_TestEvent(c, p_obj_1, ((2199023255552i64) as u64))) != 0) {
            F::BtlCharSnd_RequestAt(c, fp, 4i32, 36155i32, F32(0x43480000), F32(0x44bb8000));
        }
        if ((F::BtlObjAnim_TestEvent(c, p_obj_1, ((36028797018963968i64) as u64))) != 0) {
            F::BtlCharSnd_RequestAt(c, fp, 4i32, 36157i32, F32(0x43480000), F32(0x44bb8000));
        }
        if ((F::BtlObjAnim_TestEvent(c, p_obj_1, ((140737488355328i64) as u64))) != 0) {
            F::BtlCharSnd_RequestAt(c, fp, 0i32, 72i32, F32(0x43480000), F32(0x44bb8000));
        }
        if ((F::BtlObjAnim_TestEvent(c, p_obj_1, ((281474976710656i64) as u64))) != 0) {
            F::BtlCharSnd_RequestAt(c, fp, 0i32, 18i32, F32(0x43480000), F32(0x44bb8000));
        }
        if ((F::BtlObjAnim_TestEvent(c, p_obj_1, ((562949953421312i64) as u64))) != 0) {
            F::BtlCharSnd_RequestAt(c, fp, 0i32, 24i32, F32(0x43480000), F32(0x44bb8000));
        }
        if ((F::BtlObjAnim_TestEvent(c, p_obj_1, ((1125899906842624i64) as u64))) != 0) {
            F::BtlCharSnd_RequestAt(c, fp, 0i32, 44i32, F32(0x43480000), F32(0x44bb8000));
        }
        if ((F::BtlObjAnim_TestEvent(c, p_obj_1, ((2251799813685248i64) as u64))) != 0) {
            F::BtlCharSnd_RequestAt(c, fp, 0i32, 31i32, F32(0x43480000), F32(0x44bb8000));
        }
        if ((F::BtlObjAnim_TestEvent(c, p_obj_1, ((4503599627370496i64) as u64))) != 0) {
            F::BtlCharSnd_RequestAt(c, fp, 0i32, 49i32, F32(0x43480000), F32(0x44bb8000));
        }
        if ((F::BtlObjAnim_TestEvent(c, p_obj_1, ((9007199254740992i64) as u64))) != 0) {
            { let a2_ = (F::BtlChar_FrameMod(c, 2i32)).wrapping_add(4i32); F::BtlCharSnd_RequestAt(c, fp, 0i32, a2_, F32(0x43480000), F32(0x44bb8000)) };
        }
        if ((F::BtlObjAnim_TestEvent(c, p_obj_1, ((2147483648u32) as u64))) != 0) {
            { let a2_ = F::BtlChar_FrameMod(c, 2i32); F::BtlCharSnd_RequestAt(c, fp, 1i32, a2_, F32(0x43480000), F32(0x44bb8000)) };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlFxObj_PlayEventSounds", &PROBE_N, &[], true); }
    r_
}

pub fn BtlFxObj_HandleSubStateEvents(c: &mut Ctx, mut p_obj_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFxObj_HandleSubStateEvents", &PROBE_N, &[(p_obj_1) as u32], false); }
    let r_ = 'f: {
    if ((F::BtlObjAnim_TestEvent(c, p_obj_1, ((72057594037927936i64) as u64))) != 0) {
        F::BtlObj_SetSubState(c, p_obj_1, 9i32, 0i32);
    }
    if ((F::BtlObjAnim_TestEvent(c, p_obj_1, ((144115188075855872i64) as u64))) != 0) {
        F::BtlObj_SetSubState(c, p_obj_1, 10i32, 0i32);
    }
    if ((F::BtlObjAnim_TestEvent(c, p_obj_1, ((288230376151711744i64) as u64))) != 0) {
        F::BtlObj_SetSubState(c, p_obj_1, 11i32, 0i32);
    }
    if ((F::BtlObjAnim_TestEvent(c, p_obj_1, ((576460752303423488i64) as u64))) != 0) {
        F::BtlObj_SetEyeFrame(c, p_obj_1, ((((2i32) as i64 as u64)) as u32));
    }
    if ((F::BtlObjAnim_TestEvent(c, p_obj_1, ((1152921504606846976i64) as u64))) != 0) {
        's1: {
            let sv2: i32 = F::BtlObj_GetMouthMode(c, p_obj_1);
            match sv2 {
                9i32 | 10i32 | 11i32 | 12i32 => {
                    F::BtlObj_SetSubState(c, p_obj_1, 0i32, 0i32);
                    break 's1;
                }
                _ => {
                }
            }
        }
        F::BtlObj_SetEyeFrame(c, p_obj_1, ((((9i32) as i64 as u64)) as u32));
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlFxObj_HandleSubStateEvents", &PROBE_N, &[], true); }
    r_
}

pub fn BtlFx_UpdateObjEvents(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFx_UpdateObjEvents", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    { let a0_ = F::BtlChar_GetObj(c, p_chr_1); F::BtlFxObj_HandleSubStateEvents(c, a0_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlFx_UpdateObjEvents", &PROBE_N, &[], true); }
    r_
}

pub fn BtlFx_UpdateAll(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFx_UpdateAll", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_obj_2: u32 = 0;
    let r_ = 'f: {
    l_obj_2 = F::BtlChar_GetObj(c, p_chr_1);
    { let _ = F::BtlFx_UpdateAura(c, p_chr_1); };
    F::BtlFx_UpdateChargeFx(c, p_chr_1);
    F::BtlChar_SpawnFxBits3C(c, p_chr_1);
    F::BtlFx_FireKiBlast(c, p_chr_1);
    F::BtlFx_SpawnHitSparkReq17(c, p_chr_1);
    F::BtlChar_SpawnFxBits0(c, p_chr_1);
    F::BtlFx_UpdateObjFlagReq3(c, p_chr_1);
    F::BtlFx_HandleImpactEvents(c, p_chr_1);
    F::BtlFx_HandleFlag30Events(c, p_chr_1);
    F::BtlFx_PlaySwingSound(c, p_chr_1);
    F::BtlFxObj_HandleFlagEvents(c, l_obj_2);
    F::BtlFxObj_PlayEventSounds(c, l_obj_2);
    { let _ = F::BtlFx_UpdatePowerUpLook(c, p_chr_1); };
    F::BtlFx_SpawnReq6(c, p_chr_1);
    F::BtlFx_UpdateScreenFilter(c, p_chr_1);
    F::BtlFx_SpawnBurstReqC(c, p_chr_1);
    F::BtlFx_SpawnPartFxEvent8(c, p_chr_1);
    F::BtlFx_SpawnFastMoveFx(c, p_chr_1);
    F::BtlFx_SpawnSpeedLines(c, p_chr_1);
    F::BtlFx_SpawnReq18(c, p_chr_1);
    F::BtlFx_UpdateGroundFx(c, p_chr_1);
    F::BtlFx_UpdateWaterFx(c, p_chr_1);
    F::BtlFx_UpdateAimedFxReq28(c, p_chr_1);
    F::BtlFx_UpdateReq1A(c, p_chr_1);
    F::BtlFx_UpdateReq38(c, p_chr_1);
    F::BtlFx_TriggerObjReq1E(c, p_chr_1);
    F::BtlFx_TriggerObjReq1F(c, p_chr_1);
    F::BtlFx_SpawnFlashReq2B(c, p_chr_1);
    F::BtlFx_UpdateObjCmdFlag137(c, p_chr_1);
    F::BtlFx_UpdateSubStateFlag96(c, p_chr_1);
    F::BtlFx_UpdateObjFlag80(c, p_chr_1);
    F::BtlFx_SpawnPartFlashes(c, p_chr_1);
    { let _ = F::BtlFx_PushBackOnEvent400(c, p_chr_1); };
    { let _ = F::BtlFx_SetObjMaskOnEvent400(c, p_chr_1); };
    F::BtlFx_ShakeCamOnEvent(c, p_chr_1);
    F::BtlFx_VibrateOnEvent(c, p_chr_1);
    F::BtlFx_UpdateCharaFx(c, p_chr_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlFx_UpdateAll", &PROBE_N, &[], true); }
    r_
}

pub fn BtlFx_UpdateAfterHits(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFx_UpdateAfterHits", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    F::BtlFx_TriggerObjReq1D(c, p_chr_1);
    F::BtlFx_HandleEvent1(c, p_chr_1);
    F::BtlChar_SpawnHitFx(c, p_chr_1);
    F::BtlFx_SpawnFlashReq20(c, p_chr_1);
    F::BtlFx_SpawnClashFlash(c, p_chr_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlFx_UpdateAfterHits", &PROBE_N, &[], true); }
    r_
}

pub fn BtlFx_UpdatePostScene(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFx_UpdatePostScene", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    F::BtlFx_SpawnReq37(c, p_chr_1);
    F::BtlFx_SpawnFlashReq2E(c, p_chr_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlFx_UpdatePostScene", &PROBE_N, &[], true); }
    r_
}

pub fn BtlFx_UpdateLate(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFx_UpdateLate", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    F::BtlFx_UpdateOccludedFlag(c, p_chr_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlFx_UpdateLate", &PROBE_N, &[], true); }
    r_
}

pub fn BtlFx_UpdateStageFxRequest(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFx_UpdateStageFxRequest", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_on_2: i32 = 0;
    let r_ = 'f: {
    if ((F::BtlChar_TestFxBit(c, p_chr_1, 16i32)) != 0) {
        l_on_2 = 0i32;
        if ((((((((!(({ let a0_ = F::BtlAct_GetCurrent(c, p_chr_1); F::BtlAct_IsTechniqueId(c, a0_) }) != 0)) as i32)) != 0) || ((((({ let a1_ = F::BtlAct_GetCurrentClass(c, p_chr_1); ((((F::BtlSuper_GetFlagsA(c, p_chr_1, a1_)) as u64)) as i32) }) >= (0i32)) as i32)) != 0)) as i32)) != 0) {
            l_on_2 = 1i32;
        }
        if ((l_on_2) != 0) {
            F::BtlChar_SetFxBit(c, p_chr_1, 18i32);
        }
    }
    if ((F::BtlChar_TestFxBit(c, p_chr_1, 17i32)) != 0) {
        if ((F::BtlAnim_TestAttr(c, p_chr_1, ((512i32) as u64))) != 0) {
            F::BtlChar_SetHeldFlag(c, p_chr_1, ((((302i32) as i64 as u64)) as u32));
        }
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((302i32) as i64 as u64)) as u32))) != 0) {
            F::BtlChar_SetFxBit(c, p_chr_1, 18i32);
        }
    } else {
        F::BtlChar_ClearFlag(c, p_chr_1, ((((302i32) as i64 as u64)) as u32));
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlFx_UpdateStageFxRequest", &PROBE_N, &[], true); }
    r_
}

pub fn BtlFx_UpdateStageFx(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFx_UpdateStageFx", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let mut l_prev_2: i32 = 0;
    let mut l_now_3: i32 = 0;
    let mut l_chr_4: u32 = 0;
    let r_ = 'f: {
    l_prev_2 = 0i32;
    l_now_3 = 0i32;
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (F::BtlChar_GetCount(c))) as i32)) != 0) { break 'l1; }
            'c2: {
                { let a0_ = F::BtlChar_Get(c, l_i_1); F::BtlFx_UpdateStageFxRequest(c, a0_) };
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
    {
        l_i_1 = 0i32;
        'l3: loop {
            if !(((((l_i_1) < (F::BtlChar_GetCount(c))) as i32)) != 0) { break 'l3; }
            'c4: {
                l_chr_4 = F::BtlChar_Get(c, l_i_1);
                { let r_ = F::BtlChar_TestFxBit(c, l_chr_4, 18i32); let n_ = ((l_now_3) | (r_)); l_now_3 = n_; };
                { let r_ = F::BtlChar_TestPrevFxBit(c, l_chr_4, 18i32); let n_ = ((l_prev_2) | (r_)); l_prev_2 = n_; };
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
    if ((((((l_now_3) != 0) && ((((!((l_prev_2) != 0)) as i32)) != 0)) as i32)) != 0) {
        F::StgTint_Start(c, ((((0i32) as i64 as u64)) as u32), 0i32, F32(0x3e4ccccc));
    }
    if ((((((((!((l_now_3) != 0)) as i32)) != 0) && ((l_prev_2) != 0)) as i32)) != 0) {
        F::StgTint_Start(c, ((((0i32) as i64 as u64)) as u32), 1i32, F32(0x3e4ccccc));
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlFx_UpdateStageFx", &PROBE_N, &[], true); }
    r_
}

pub fn BtlFx_UpdateRoster(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFx_UpdateRoster", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::BtlFx_UpdateStageFx(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlFx_UpdateRoster", &PROBE_N, &[], true); }
    r_
}

pub fn BtlFxObj_Place(c: &mut Ctx, mut p_obj_1: u32, mut p_pos_2: u32, mut p_rot_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFxObj_Place", &PROBE_N, &[(p_obj_1) as u32, (p_pos_2) as u32, (p_rot_3) as u32], false); }
    let r_ = 'f: {
    F::Vec4_Copy(c, (p_obj_1).wrapping_add(2384), p_pos_2);
    F::Vec4_Copy(c, (p_obj_1).wrapping_add(2400), p_rot_3);
    F::BtlObjXf_Update(c, p_obj_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlFxObj_Place", &PROBE_N, &[], true); }
    r_
}

pub fn BtlPartner_HandleEvents(c: &mut Ctx, mut p_obj_1: u32, mut p_chr_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlPartner_HandleEvents", &PROBE_N, &[(p_obj_1) as u32, (p_chr_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(64) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 64);
    let mut l_on_3: i32 = 0;
    let mut l_mask_4: i32 = 0;
    let mut l_kind_5: i32 = 0;
    let mut l_sound_6: i32 = 0;
    let r_ = 'f: {
        { let a0_ = (c.m.r32((p_obj_1).wrapping_add(16)) as i32); F::BtlCharApi_GetNodePos(c, a0_, 3i32, fp) };
        if ((F::BtlObjAnim_TestEvent(c, p_obj_1, ((33554432i32) as u64))) != 0) {
            { let r_ = (((!(2i32))) as u32); let a_ = (p_obj_1).wrapping_add(2624); let n_ = ((c.m.r32(a_)) & (r_)); c.m.w32(a_, n_); };
        }
        if ((F::BtlObjAnim_TestEvent(c, p_obj_1, ((67108864i32) as u64))) != 0) {
            { let r_ = ((2i32) as u32); let a_ = (p_obj_1).wrapping_add(2624); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
        }
        l_on_3 = 0i32;
        l_mask_4 = 0i32;
        l_kind_5 = 0i32;
        l_sound_6 = 0i32;
        if ((F::BtlObjAnim_TestEvent(c, p_obj_1, ((256i32) as u64))) != 0) {
            l_on_3 = 1i32;
            l_mask_4 = 256i32;
            l_sound_6 = (F::BtlChar_FrameMod(c, 2i32)).wrapping_add(2i32);
            l_kind_5 = 15i32;
        }
        if ((F::BtlObjAnim_TestEvent(c, p_obj_1, ((128i32) as u64))) != 0) {
            l_on_3 = 1i32;
            l_mask_4 = 128i32;
            l_kind_5 = 16i32;
            l_sound_6 = 73i32;
        }
        if ((l_on_3) != 0) {
            { let a0_ = (c.m.r32((p_obj_1).wrapping_add(16)) as i32); let a1_ = { let a0_ = ((((F::BtlObjAnim_GetEventArg(c, p_obj_1, ((l_mask_4) as u64))) as u64)) as i32); F::BtlObjAnim_MaskToNode(c, ((((a0_) as i64 as u64)) as u32)) }; F::BtlCharApi_GetNodePos(c, a0_, a1_, fp.wrapping_add(16)) };
            { let v_ = F32(0x3f800000); let a_ = (fp.wrapping_add(16)).wrapping_add(12); c.m.w32(a_, (v_).0); };
            { let v_ = { let a0_ = F32(c.m.r32(((p_obj_1).wrapping_add(2400)).wrapping_add(4))); F::Mathf_Sin(c, a0_) }; let a_ = (fp.wrapping_add(16)).wrapping_add(16); c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x00000000); let a_ = ((fp.wrapping_add(16)).wrapping_add(16)).wrapping_add(4); c.m.w32(a_, (v_).0); };
            { let v_ = { let a0_ = F32(c.m.r32(((p_obj_1).wrapping_add(2400)).wrapping_add(4))); F::Mathf_Cos(c, a0_) }; let a_ = ((fp.wrapping_add(16)).wrapping_add(16)).wrapping_add(8); c.m.w32(a_, (v_).0); };
            { let v_ = l_kind_5; let a_ = (fp.wrapping_add(16)).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
            { let v_ = (c.m.r32((p_obj_1).wrapping_add(16)) as i32); let a_ = (fp.wrapping_add(16)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
            { let v_ = F32(0x00000000); let a_ = ((fp.wrapping_add(16)).wrapping_add(16)).wrapping_add(12); c.m.w32(a_, (v_).0); };
            { let v_ = 0i32; let a_ = (fp.wrapping_add(16)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
            { let _ = F::EftImpact_SpawnHit(c, fp.wrapping_add(16)); };
            F::BtlCharSnd_RequestAt(c, fp, 0i32, l_sound_6, F32(0x43480000), F32(0x44bb8000));
            { let a0_ = { let a0_ = F::BtlOpp_GetPlayer(c, p_chr_2); F::BtlChar_Get(c, a0_) }; F::BtlChar_PlayVoice(c, a0_, 1i32) };
        }
        F::BtlFxObj_HandleFlagEvents(c, p_obj_1);
        F::BtlFxObj_PlayEventSounds(c, p_obj_1);
        l_sound_6 = 52274i32;
        if ((((!(((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5640); (c.m.r32(a_) as i32) }) & (1i32))) != 0)) as i32)) != 0) {
            l_sound_6 = 36174i32;
        }
        l_kind_5 = (1i32).wrapping_neg();
        l_mask_4 = (1i32).wrapping_neg();
        l_on_3 = (1i32).wrapping_neg();
        if ((F::BtlChar_TestFlag(c, p_chr_2, ((((304i32) as i64 as u64)) as u32))) != 0) {
            l_mask_4 = 95i32;
            if ((((!((F::BtlObjAnim_TestEvent(c, p_obj_1, ((32768i32) as u64))) != 0)) as i32)) != 0) {
                l_mask_4 = l_on_3;
            }
            if ((F::BtlObjAnim_TestEvent(c, p_obj_1, ((549755813888i64) as u64))) != 0) {
                l_mask_4 = 96i32;
            }
            if ((F::BtlObjAnim_TestEvent(c, p_obj_1, ((131072i32) as u64))) != 0) {
                l_mask_4 = 97i32;
            }
        }
        if ((F::BtlChar_TestFlag(c, p_chr_2, ((((305i32) as i64 as u64)) as u32))) != 0) {
            if ((F::BtlObjAnim_TestEvent(c, p_obj_1, ((549755846656i64) as u64))) != 0) {
                l_kind_5 = 98i32;
                l_mask_4 = 98i32;
            }
        }
        if (((((l_kind_5) >= (0i32)) as i32)) != 0) {
            { let a2_ = ((l_sound_6).wrapping_add(((c.m.r32((p_obj_1).wrapping_add(12)) as i32)).wrapping_mul(100i32))).wrapping_add(l_kind_5); F::BtlCharSnd_RequestAt(c, fp, 7i32, a2_, F32(0x43480000), F32(0x44bb8000)) };
        }
        if (((((l_mask_4) >= (0i32)) as i32)) != 0) {
            F::BtlObj_SetSubState(c, p_obj_1, 2i32, l_mask_4);
        }
        's1: {
            let sv7: i32 = F::BtlAct_GetCurrentClass(c, p_chr_2);
            match sv7 {
                2i32 => {
                    l_on_3 = 87i32;
                    break 's1;
                }
                3i32 => {
                    l_on_3 = 89i32;
                    break 's1;
                }
                4i32 => {
                    l_on_3 = 91i32;
                    break 's1;
                }
                _ => {
                }
            }
        }
        if (((((l_on_3) >= (0i32)) as i32)) != 0) {
            if ((F::BtlObjAnim_TestEvent(c, p_obj_1, ((32768i32) as u64))) != 0) {
                { let a2_ = ((l_sound_6).wrapping_add(({ let a_ = F::BtlMember_GetActive(c, p_chr_2); (c.m.r32(a_) as i32) }).wrapping_mul(100i32))).wrapping_add(l_on_3); F::BtlCharSnd_RequestAt(c, fp, 7i32, a2_, F32(0x43480000), F32(0x44bb8000)) };
                F::BtlObj_SetSubState(c, p_obj_1, 2i32, l_on_3);
            }
            if ((F::BtlObjAnim_TestEvent(c, p_obj_1, ((549755813888i64) as u64))) != 0) {
                { let a2_ = ((l_sound_6).wrapping_add((l_on_3).wrapping_add(({ let a_ = F::BtlMember_GetActive(c, p_chr_2); (c.m.r32(a_) as i32) }).wrapping_mul(100i32)))).wrapping_add(1i32); F::BtlCharSnd_RequestAt(c, fp, 7i32, a2_, F32(0x43480000), F32(0x44bb8000)) };
                F::BtlObj_SetSubState(c, p_obj_1, 2i32, (l_on_3).wrapping_add(1i32));
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlPartner_HandleEvents", &PROBE_N, &[], true); }
    r_
}

pub fn BtlPartner_IsActive(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlPartner_IsActive", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f (c.m.r32((p_chr_1).wrapping_add(4912)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlPartner_IsActive", &PROBE_N, &[], true); }
    r_
}

pub fn BtlPartner_Attach(c: &mut Ctx, mut p_chr_1: u32, mut p_res_2: i32, mut p_objId_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlPartner_Attach", &PROBE_N, &[(p_chr_1) as u32, (p_res_2) as u32, (p_objId_3) as u32], false); }
    let mut l_p_4: u32 = 0;
    let r_ = 'f: {
    l_p_4 = (p_chr_1).wrapping_add(4912);
    if ((((!(((c.m.r32(l_p_4) as i32)) != 0)) as i32)) != 0) {
        { let v_ = p_res_2; let a_ = (l_p_4).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = p_objId_3; let a_ = (l_p_4).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 1i32; let a_ = l_p_4; c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlPartner_Attach", &PROBE_N, &[], true); }
    r_
}

pub fn BtlPartner_Release(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlPartner_Release", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_p_2: u32 = 0;
    let r_ = 'f: {
    l_p_2 = (p_chr_1).wrapping_add(4912);
    if (((c.m.r32(l_p_2) as i32)) != 0) {
        { let _ = { let a0_ = (c.m.r32((l_p_2).wrapping_add(8)) as i32); F::BtlObj_Destroy(c, a0_) }; };
        { let a0_ = (c.m.r32((l_p_2).wrapping_add(4)) as i32); F::BtlRes_Free(c, a0_) };
        { let v_ = 0i32; let a_ = (l_p_2).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (l_p_2).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = l_p_2; c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlPartner_Release", &PROBE_N, &[], true); }
    r_
}

pub fn BtlPartner_Update(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlPartner_Update", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_p_2: u32 = 0;
    let mut l_obj_3: u32 = 0;
    let mut l_partner_4: u32 = 0;
    let mut l_pos_5: u32 = 0;
    let r_ = 'f: {
    l_p_2 = (p_chr_1).wrapping_add(4912);
    if (((c.m.r32(l_p_2) as i32)) != 0) {
        l_obj_3 = F::BtlChar_GetObj(c, p_chr_1);
        l_pos_5 = (l_obj_3).wrapping_add(2384);
        l_partner_4 = { let a0_ = (c.m.r32((l_p_2).wrapping_add(8)) as i32); F::BtlObj_Get(c, a0_) };
        F::BtlFxObj_Place(c, l_partner_4, l_pos_5, (l_obj_3).wrapping_add(2400));
        F::BtlObj_SaveNodePositions(c, l_partner_4, 0i32);
        F::BtlObjAnim_SamplePose(c, l_partner_4);
        F::BtlObjAnim_UpdateEvents(c, l_partner_4);
        F::BtlFxObj_HandleSubStateEvents(c, l_partner_4);
        F::BtlObjPose_CalcMatrices(c, l_partner_4);
        F::BtlObj_UpdateChains(c, l_partner_4);
        F::BtlObjPose_CalcMatrices(c, l_partner_4);
        { let r_ = (((!(33554432i32))) as u32); let a_ = (l_partner_4).wrapping_add(2624); let n_ = ((c.m.r32(a_)) & (r_)); c.m.w32(a_, n_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlPartner_Update", &PROBE_N, &[], true); }
    r_
}

pub fn BtlPartner_UpdateEvents(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlPartner_UpdateEvents", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_p_2: u32 = 0;
    let r_ = 'f: {
    l_p_2 = (p_chr_1).wrapping_add(4912);
    if (((c.m.r32(l_p_2) as i32)) != 0) {
        { let a0_ = { let a0_ = (c.m.r32((l_p_2).wrapping_add(8)) as i32); F::BtlObj_Get(c, a0_) }; F::BtlPartner_HandleEvents(c, a0_, p_chr_1) };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlPartner_UpdateEvents", &PROBE_N, &[], true); }
    r_
}

pub fn BtlPartner_PlayAnim(c: &mut Ctx, mut p_chr_1: u32, mut p_anim_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlPartner_PlayAnim", &PROBE_N, &[(p_chr_1) as u32, (p_anim_2) as u32], false); }
    let mut l_p_3: u32 = 0;
    let mut l_obj_4: u32 = 0;
    let r_ = 'f: {
    l_p_3 = (p_chr_1).wrapping_add(4912);
    if ((((!(((c.m.r32(l_p_3) as i32)) != 0)) as i32)) != 0) {
        break 'f;
    }
    if (((((p_anim_2) < (414i32)) as i32)) != 0) {
        break 'f;
    }
    l_obj_4 = { let a0_ = (c.m.r32((l_p_3).wrapping_add(8)) as i32); F::BtlObj_Get(c, a0_) };
    F::BtlObjAnim_PlayModel(c, l_obj_4, (p_anim_2).wrapping_sub(414i32), 0i32);
    if (((((c.m.r32((l_obj_4).wrapping_add(2880))) == (((0i32) as u32))) as i32)) != 0) {
        { let r_ = (((!(2i32))) as u32); let a_ = (l_obj_4).wrapping_add(2624); let n_ = ((c.m.r32(a_)) & (r_)); c.m.w32(a_, n_); };
    } else {
        if ((((((({ let a_ = (c.m.r32((l_obj_4).wrapping_add(2880))).wrapping_add(2); c.m.r16(a_) }) as i32)) == (0i32)) as i32)) != 0) {
            { let r_ = (((!(2i32))) as u32); let a_ = (l_obj_4).wrapping_add(2624); let n_ = ((c.m.r32(a_)) & (r_)); c.m.w32(a_, n_); };
        }
    }
    { let r_ = (((!(32i32))) as u32); let a_ = (l_obj_4).wrapping_add(2624); let n_ = ((c.m.r32(a_)) & (r_)); c.m.w32(a_, n_); };
    { let r_ = (((!(64i32))) as u32); let a_ = (l_obj_4).wrapping_add(2624); let n_ = ((c.m.r32(a_)) & (r_)); c.m.w32(a_, n_); };
    's1: {
        let sv5: i32 = F::BtlObj_GetMouthMode(c, l_obj_4);
        match sv5 {
            9i32 | 10i32 | 11i32 | 12i32 => {
                F::BtlObj_SetSubState(c, l_obj_4, 0i32, 0i32);
                break 's1;
            }
            _ => {
            }
        }
    }
    { let v_ = 1i32; let a_ = ((l_obj_4).wrapping_add(2880)).wrapping_add(304); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlPartner_PlayAnim", &PROBE_N, &[], true); }
    r_
}

pub fn BtlPartner_StepAnim(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlPartner_StepAnim", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_p_2: u32 = 0;
    let mut l_done_3: i32 = 0;
    let mut l_anim_4: u32 = 0;
    let r_ = 'f: {
    l_p_2 = (p_chr_1).wrapping_add(4912);
    l_done_3 = 0i32;
    if ((((!(((c.m.r32(l_p_2) as i32)) != 0)) as i32)) != 0) {
        break 'f 1i32;
    }
    l_anim_4 = ({ let a0_ = (c.m.r32((l_p_2).wrapping_add(8)) as i32); F::BtlObj_Get(c, a0_) }).wrapping_add(2880);
    { let v_ = F32(c.m.r32((l_anim_4).wrapping_add(312))); let a_ = (l_anim_4).wrapping_add(316); c.m.w32(a_, (v_).0); };
    { let r_ = F32(c.m.r32((l_anim_4).wrapping_add(320))); let a_ = (l_anim_4).wrapping_add(312); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
    if (((rt::fge(F32(c.m.r32((l_anim_4).wrapping_add(312))), F32(c.m.r32((l_anim_4).wrapping_add(4)))) as i32)) != 0) {
        { let v_ = F32(c.m.r32((l_anim_4).wrapping_add(4))); let a_ = (l_anim_4).wrapping_add(312); c.m.w32(a_, (v_).0); };
        l_done_3 = 1i32;
    }
    break 'f l_done_3;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlPartner_StepAnim", &PROBE_N, &[], true); }
    r_
}

pub fn BtlPartner_SetFlag10(c: &mut Ctx, mut p_chr_1: u32, mut p_on_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlPartner_SetFlag10", &PROBE_N, &[(p_chr_1) as u32, (p_on_2) as u32], false); }
    let mut l_p_3: u32 = 0;
    let mut l_obj_4: u32 = 0;
    let r_ = 'f: {
    l_p_3 = (p_chr_1).wrapping_add(4912);
    if (((c.m.r32(l_p_3) as i32)) != 0) {
        l_obj_4 = { let a0_ = (c.m.r32((l_p_3).wrapping_add(8)) as i32); F::BtlObj_Get(c, a0_) };
        if ((p_on_2) != 0) {
            { let r_ = ((16i32) as u32); let a_ = (l_obj_4).wrapping_add(2624); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
        } else {
            { let r_ = (((!(16i32))) as u32); let a_ = (l_obj_4).wrapping_add(2624); let n_ = ((c.m.r32(a_)) & (r_)); c.m.w32(a_, n_); };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlPartner_SetFlag10", &PROBE_N, &[], true); }
    r_
}

pub fn BtlPartner_LinkToOwner(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlPartner_LinkToOwner", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_p_2: u32 = 0;
    let r_ = 'f: {
    l_p_2 = (p_chr_1).wrapping_add(4912);
    if (((c.m.r32(l_p_2) as i32)) != 0) {
        { let a0_ = { let a0_ = (c.m.r32((l_p_2).wrapping_add(8)) as i32); F::BtlObj_Get(c, a0_) }; let a1_ = F::BtlChar_GetObj(c, p_chr_1); F::BtlObj_CopyLipTables(c, a0_, a1_) };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlPartner_LinkToOwner", &PROBE_N, &[], true); }
    r_
}

pub fn BtlPartner_GetObj(c: &mut Ctx, mut p_chr_1: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlPartner_GetObj", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_p_2: u32 = 0;
    let r_ = 'f: {
    l_p_2 = (p_chr_1).wrapping_add(4912);
    if (((c.m.r32(l_p_2) as i32)) != 0) {
        break 'f { let a0_ = (c.m.r32((l_p_2).wrapping_add(8)) as i32); F::BtlObj_Get(c, a0_) };
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlPartner_GetObj", &PROBE_N, &[], true); }
    r_
}

pub fn BtlPartner_PushAngle(c: &mut Ctx, mut p_chr_1: u32, mut p_yaw_2: F32, mut p_speed_3: F32, mut p_arg_4: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlPartner_PushAngle", &PROBE_N, &[(p_chr_1) as u32, (p_yaw_2).0, (p_speed_3).0, (p_arg_4).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_p_5: u32 = 0;
    let mut l_obj_6: u32 = 0;
    let r_ = 'f: {
        l_p_5 = (p_chr_1).wrapping_add(4912);
        if (((c.m.r32(l_p_5) as i32)) != 0) {
            l_obj_6 = { let a0_ = (c.m.r32((l_p_5).wrapping_add(8)) as i32); F::BtlObj_Get(c, a0_) };
            { let v_ = rt::fmul(F::Mathf_Sin(c, p_yaw_2), p_speed_3); let a_ = fp; c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x00000000); let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
            { let v_ = rt::fmul(F::Mathf_Cos(c, p_yaw_2), p_speed_3); let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x00000000); let a_ = (fp).wrapping_add(12); c.m.w32(a_, (v_).0); };
            F::BtlObj_AddPush(c, l_obj_6, fp, p_arg_4);
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlPartner_PushAngle", &PROBE_N, &[], true); }
    r_
}

pub fn BtlPartner_PushDir(c: &mut Ctx, mut p_chr_1: u32, mut p_dir_2: u32, mut p_speed_3: F32, mut p_arg_4: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlPartner_PushDir", &PROBE_N, &[(p_chr_1) as u32, (p_dir_2) as u32, (p_speed_3).0, (p_arg_4).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_p_5: u32 = 0;
    let mut l_obj_6: u32 = 0;
    let mut l_len_7: F32 = F32(0);
    let r_ = 'f: {
        l_p_5 = (p_chr_1).wrapping_add(4912);
        if (((c.m.r32(l_p_5) as i32)) != 0) {
            l_obj_6 = { let a0_ = (c.m.r32((l_p_5).wrapping_add(8)) as i32); F::BtlObj_Get(c, a0_) };
            l_len_7 = F::Vec3_Length(c, p_dir_2);
            if ((((!(((rt::flt(l_len_7, F32(0x38d1b717)) as i32)) != 0)) as i32)) != 0) {
                F::Vec4_Scale(c, fp, p_dir_2, rt::fdiv(F32(0x3f800000), l_len_7));
                F::Vec4_Scale(c, fp, fp, p_speed_3);
                F::BtlObj_AddPush(c, l_obj_6, fp, p_arg_4);
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlPartner_PushDir", &PROBE_N, &[], true); }
    r_
}

pub fn BtlPartner_SetObjFloats(c: &mut Ctx, mut p_chr_1: u32, mut p_a_2: F32, mut p_b_3: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlPartner_SetObjFloats", &PROBE_N, &[(p_chr_1) as u32, (p_a_2).0, (p_b_3).0], false); }
    let mut l_p_4: u32 = 0;
    let r_ = 'f: {
    l_p_4 = (p_chr_1).wrapping_add(4912);
    if (((c.m.r32(l_p_4) as i32)) != 0) {
        { let a0_ = { let a0_ = (c.m.r32((l_p_4).wrapping_add(8)) as i32); F::BtlObj_Get(c, a0_) }; F::BtlObj_AddSway(c, a0_, p_a_2, p_b_3) };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlPartner_SetObjFloats", &PROBE_N, &[], true); }
    r_
}

pub fn BtlPartner_SetFlag80(c: &mut Ctx, mut p_chr_1: u32, mut p_on_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlPartner_SetFlag80", &PROBE_N, &[(p_chr_1) as u32, (p_on_2) as u32], false); }
    let mut l_p_3: u32 = 0;
    let mut l_set_4: i32 = 0;
    let mut l_obj_5: u32 = 0;
    let r_ = 'f: {
    l_p_3 = (p_chr_1).wrapping_add(4912);
    l_set_4 = 0i32;
    if (((c.m.r32(l_p_3) as i32)) != 0) {
        l_obj_5 = { let a0_ = (c.m.r32((l_p_3).wrapping_add(8)) as i32); F::BtlObj_Get(c, a0_) };
        if (((((((((l_obj_5) != (((0i32) as u32))) as i32)) != 0) && (((((c.m.r32((l_obj_5).wrapping_add(2332))) != (((0i32) as u32))) as i32)) != 0)) as i32)) != 0) {
            if (((({ let a_ = (c.m.r32((l_obj_5).wrapping_add(2332))).wrapping_add(16); c.m.r32(a_) }) & (((2097152i32) as u32)))) != 0) {
                l_set_4 = (((p_on_2) != (0i32)) as i32);
            }
        }
        if ((l_set_4) != 0) {
            if (((((((c.m.r64((l_obj_5).wrapping_add(2624))) & (((130i32) as u64)))) != (((130i32) as u64))) as i32)) != 0) {
                { let r_ = ((128i32) as u32); let a_ = (l_obj_5).wrapping_add(2624); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
            }
        } else {
            if (((((((c.m.r64((l_obj_5).wrapping_add(2624))) & (((130i32) as u64)))) == (((130i32) as u64))) as i32)) != 0) {
                { let r_ = (((!(128i32))) as u32); let a_ = (l_obj_5).wrapping_add(2624); let n_ = ((c.m.r32(a_)) & (r_)); c.m.w32(a_, n_); };
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlPartner_SetFlag80", &PROBE_N, &[], true); }
    r_
}
