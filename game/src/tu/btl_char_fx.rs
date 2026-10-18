#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn BtlChar_SetFxBit(c: &mut Ctx, mut p_chr_1: u32, mut p_n_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_SetFxBit", &PROBE_N, &[(p_chr_1) as u32, (p_n_2) as u32], false); }
    let r_ = 'f: {
    { let r_ = (1i32).wrapping_shl((((p_n_2) & (7i32))) as u32); let a_ = ((p_chr_1).wrapping_add(4706)).wrapping_add((((p_n_2).wrapping_shr((3i32) as u32)) as u32)); let n_ = ((((((c.m.r8(a_)) as i32)) | (r_))) as u8); c.m.w8(a_, n_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_SetFxBit", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChar_ClearFxBit(c: &mut Ctx, mut p_chr_1: u32, mut p_n_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_ClearFxBit", &PROBE_N, &[(p_chr_1) as u32, (p_n_2) as u32], false); }
    let mut l_mask_3: u8 = 0;
    let r_ = 'f: {
    l_mask_3 = (((1i32).wrapping_shl((((p_n_2) & (7i32))) as u32)) as u8);
    { let r_ = (!(((l_mask_3) as i32))); let a_ = ((p_chr_1).wrapping_add(4706)).wrapping_add((((p_n_2).wrapping_shr((3i32) as u32)) as u32)); let n_ = ((((((c.m.r8(a_)) as i32)) & (r_))) as u8); c.m.w8(a_, n_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_ClearFxBit", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChar_TestFxBit(c: &mut Ctx, mut p_chr_1: u32, mut p_n_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_TestFxBit", &PROBE_N, &[(p_chr_1) as u32, (p_n_2) as u32], false); }
    let mut l_mask_3: u8 = 0;
    let r_ = 'f: {
    l_mask_3 = (((1i32).wrapping_shl((((p_n_2) & (7i32))) as u32)) as u8);
    break 'f (((((((c.m.r8(((p_chr_1).wrapping_add(4706)).wrapping_add((((p_n_2).wrapping_shr((3i32) as u32)) as u32)))) as i32)) & (((l_mask_3) as i32)))) != (0i32)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_TestFxBit", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChar_TestPrevFxBit(c: &mut Ctx, mut p_chr_1: u32, mut p_n_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_TestPrevFxBit", &PROBE_N, &[(p_chr_1) as u32, (p_n_2) as u32], false); }
    let mut l_mask_3: u8 = 0;
    let r_ = 'f: {
    l_mask_3 = (((1i32).wrapping_shl((((p_n_2) & (7i32))) as u32)) as u8);
    break 'f (((((((c.m.r8(((p_chr_1).wrapping_add(4715)).wrapping_add((((p_n_2).wrapping_shr((3i32) as u32)) as u32)))) as i32)) & (((l_mask_3) as i32)))) != (0i32)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_TestPrevFxBit", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChar_IsFxBitNew(c: &mut Ctx, mut p_chr_1: u32, mut p_n_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_IsFxBitNew", &PROBE_N, &[(p_chr_1) as u32, (p_n_2) as u32], false); }
    let mut l_r_3: i32 = 0;
    let r_ = 'f: {
    l_r_3 = 0i32;
    if ((F::BtlChar_TestFxBit(c, p_chr_1, p_n_2)) != 0) {
        l_r_3 = (((F::BtlChar_TestPrevFxBit(c, p_chr_1, p_n_2)) == (0i32)) as i32);
    }
    break 'f l_r_3;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_IsFxBitNew", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChar_IsFxBitEnded(c: &mut Ctx, mut p_chr_1: u32, mut p_n_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_IsFxBitEnded", &PROBE_N, &[(p_chr_1) as u32, (p_n_2) as u32], false); }
    let mut l_r_3: i32 = 0;
    let r_ = 'f: {
    l_r_3 = 0i32;
    if ((((!((F::BtlChar_TestFxBit(c, p_chr_1, p_n_2)) != 0)) as i32)) != 0) {
        l_r_3 = (((F::BtlChar_TestPrevFxBit(c, p_chr_1, p_n_2)) != (0i32)) as i32);
    }
    break 'f l_r_3;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_IsFxBitEnded", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChar_GetHitSoundLine(c: &mut Ctx, mut p_kind_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_GetHitSoundLine", &PROBE_N, &[(p_kind_1) as u32], false); }
    let r_ = 'f: {
    's1: {
        let sv2: i32 = p_kind_1;
        match sv2 {
            0i32 => {
                break 's1;
            }
            1i32 | 2i32 | 3i32 | 4i32 => {
                break 'f F::BtlChar_FrameMod(c, 2i32);
            }
            5i32 | 9i32 | 10i32 | 11i32 | 12i32 => {
                break 'f (F::BtlChar_FrameMod(c, 3i32)).wrapping_add(6i32);
            }
            6i32 => {
                break 'f 40i32;
            }
            7i32 => {
                break 'f 41i32;
            }
            8i32 => {
                break 'f 42i32;
            }
            _ => {
            }
        }
    }
    break 'f (1i32).wrapping_neg();
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_GetHitSoundLine", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChar_SpawnFxBits3C(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_SpawnFxBits3C", &PROBE_N, &[(p_chr_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_kind_2: i32 = 0;
    let mut l_unkA24_3: i32 = 0;
    let r_ = 'f: {
        l_kind_2 = (1i32).wrapping_neg();
        if ((F::BtlChar_TestFxBit(c, p_chr_1, 60i32)) != 0) {
            l_kind_2 = 0i32;
        }
        if ((F::BtlChar_TestFxBit(c, p_chr_1, 61i32)) != 0) {
            l_kind_2 = 1i32;
        }
        if ((F::BtlChar_TestFxBit(c, p_chr_1, 62i32)) != 0) {
            l_kind_2 = 2i32;
        }
        if ((F::BtlChar_TestFxBit(c, p_chr_1, 63i32)) != 0) {
            l_kind_2 = 3i32;
        }
        if ((F::BtlChar_TestFxBit(c, p_chr_1, 64i32)) != 0) {
            l_kind_2 = 4i32;
        }
        if (((((l_kind_2) >= (0i32)) as i32)) != 0) {
            { let v_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); let a_ = fp; c.m.w32(a_, (v_) as u32); };
            l_unkA24_3 = { let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2596); (c.m.r32(a_) as i32) };
            { let v_ = l_kind_2; let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
            { let v_ = l_unkA24_3; let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
            's1: {
                let sv4: i32 = l_kind_2;
                match sv4 {
                    0i32 | 1i32 => {
                        { let v_ = F::BtlSkill_GetShotUnk30(c, p_chr_1, l_kind_2); let a_ = (fp).wrapping_add(12); c.m.w32(a_, (v_).0); };
                        { let v_ = F::BtlSkill_GetShotTime(c, p_chr_1, l_kind_2); let a_ = (fp).wrapping_add(16); c.m.w32(a_, (v_).0); };
                        { let v_ = F::BtlSkill_GetShotSpeed(c, p_chr_1, l_kind_2); let a_ = (fp).wrapping_add(20); c.m.w32(a_, (v_).0); };
                        { let v_ = F::BtlSkill_GetShotTurnRate(c, p_chr_1, l_kind_2); let a_ = (fp).wrapping_add(24); c.m.w32(a_, (v_).0); };
                        break 's1;
                    }
                    2i32 | 3i32 | 4i32 => {
                        { let v_ = F::BtlSuper_GetShotUnk6C(c, p_chr_1, l_kind_2); let a_ = (fp).wrapping_add(12); c.m.w32(a_, (v_).0); };
                        { let v_ = F::BtlSuper_GetShotTime(c, p_chr_1, l_kind_2); let a_ = (fp).wrapping_add(16); c.m.w32(a_, (v_).0); };
                        { let v_ = F::BtlSuper_GetShotSpeed(c, p_chr_1, l_kind_2); let a_ = (fp).wrapping_add(20); c.m.w32(a_, (v_).0); };
                        { let v_ = F::BtlSuper_GetShotTurnRate(c, p_chr_1, l_kind_2); let a_ = (fp).wrapping_add(24); c.m.w32(a_, (v_).0); };
                        break 's1;
                    }
                    _ => {
                    }
                }
            }
            F::EftShot_Request(c, fp);
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_SpawnFxBits3C", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChar_SpawnFxBits0(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_SpawnFxBits0", &PROBE_N, &[(p_chr_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        if ((F::BtlChar_TestFxBit(c, p_chr_1, 0i32)) != 0) {
            { let v_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); let a_ = fp; c.m.w32(a_, (v_) as u32); };
            { let v_ = (1i32).wrapping_neg(); let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
            's1: {
                let sv2: i32 = (c.m.r32((p_chr_1).wrapping_add(4828)) as i32);
                match sv2 {
                    0i32 => {
                        { let v_ = (1i32).wrapping_neg(); let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                        break 's1;
                    }
                    1i32 => {
                        { let v_ = 1i32; let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                        break 's1;
                    }
                    2i32 => {
                        { let v_ = 0i32; let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                        break 's1;
                    }
                    3i32 => {
                        { let v_ = 5i32; let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                        break 's1;
                    }
                    4i32 => {
                        { let v_ = 6i32; let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                        break 's1;
                    }
                    5i32 => {
                        { let v_ = 2i32; let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                        break 's1;
                    }
                    6i32 => {
                        { let v_ = 3i32; let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                        break 's1;
                    }
                    7i32 => {
                        { let v_ = 4i32; let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                        break 's1;
                    }
                    8i32 => {
                        { let v_ = 7i32; let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                        break 's1;
                    }
                    9i32 => {
                        { let v_ = 8i32; let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                        break 's1;
                    }
                    10i32 => {
                        { let v_ = 9i32; let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                        break 's1;
                    }
                    11i32 => {
                        { let v_ = 10i32; let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                        break 's1;
                    }
                    _ => {
                    }
                }
            }
            if ((((((c.m.r32((fp).wrapping_add(4)) as i32)) >= (0i32)) as i32)) != 0) {
                { let _ = F::EftTransform_Start(c, fp); };
            }
        }
        if ((((((F::BtlChar_TestFxBit(c, p_chr_1, 1i32)) != 0) && ((((((c.m.r32((p_chr_1).wrapping_add(4828)) as i32)) != (0i32)) as i32)) != 0)) as i32)) != 0) {
            { let _ = F::EftTransform_Flash(c); };
        }
        if ((((((F::BtlChar_TestFxBit(c, p_chr_1, 2i32)) != 0) && ((((((c.m.r32((p_chr_1).wrapping_add(4828)) as i32)) != (0i32)) as i32)) != 0)) as i32)) != 0) {
            { let _ = F::EftTransform_End(c); };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_SpawnFxBits0", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChar_SpawnHitFx(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_SpawnHitFx", &PROBE_N, &[(p_chr_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(176) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 176);
    let mut l_sound_2: i32 = 0;
    let mut l_pose_3: u32 = 0;
    let mut l_obj_4: u32 = 0;
    let mut l_guarded_5: i32 = 0;
    let mut l_level_6: i32 = 0;
    let mut l_yaw_7: F32 = F32(0);
    let mut l_pitch_8: F32 = F32(0);
    let mut l_sinYaw_9: F32 = F32(0);
    let mut l_cosYaw_10: F32 = F32(0);
    let mut l_sinPitch_11: F32 = F32(0);
    let mut l_cosPitch_12: F32 = F32(0);
    let mut l_bits_13: u32 = 0;
    let mut l_hit_14: u32 = 0;
    let mut l_i_15: i32 = 0;
    let mut l_d_16: F32 = F32(0);
    let r_ = 'f: {
        l_sound_2 = (1i32).wrapping_neg();
        l_pose_3 = F::BtlChar_GetPos(c, p_chr_1);
        l_obj_4 = F::BtlChar_GetObj(c, p_chr_1);
        { let v_ = (1i32).wrapping_neg(); let a_ = (fp).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
        { let v_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); let a_ = (fp).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (fp).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
        if ((F::BtlChar_TestFxBit(c, p_chr_1, 27i32)) != 0) {
            l_guarded_5 = 0i32;
            's1: {
                let sv17: i32 = F::BtlAtk_GetHitFxKind(c, p_chr_1);
                match sv17 {
                    1i32 => {
                        { let v_ = 0i32; let a_ = (fp).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
                        break 's1;
                    }
                    2i32 => {
                        { let v_ = 2i32; let a_ = (fp).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
                        break 's1;
                    }
                    3i32 => {
                        { let v_ = 2i32; let a_ = (fp).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
                        break 's1;
                    }
                    4i32 => {
                        { let v_ = 1i32; let a_ = (fp).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
                        break 's1;
                    }
                    5i32 => {
                        { let v_ = 2i32; let a_ = (fp).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
                        break 's1;
                    }
                    6i32 => {
                        { let v_ = 3i32; let a_ = (fp).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
                        break 's1;
                    }
                    7i32 => {
                        { let v_ = 12i32; let a_ = (fp).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
                        break 's1;
                    }
                    8i32 => {
                        { let v_ = 7i32; let a_ = (fp).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
                        break 's1;
                    }
                    9i32 => {
                        { let v_ = 8i32; let a_ = (fp).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
                        break 's1;
                    }
                    10i32 => {
                        { let v_ = 13i32; let a_ = (fp).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
                        break 's1;
                    }
                    _ => {
                    }
                }
            }
            l_level_6 = F::BtlAtk_GetHitSoundLevel(c, p_chr_1);
            if (((((({ let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); F::BtlCharApi_HasWeaponOut(c, a0_) }) != 0) && ((((!((((((((F::BtlAtk_GetFlags(c, p_chr_1)) as u64)) as i32)) & (8192i32))) != 0)) as i32)) != 0)) as i32)) != 0) {
                l_guarded_5 = 1i32;
            }
            if ((l_guarded_5) != 0) {
                if ((((((c.m.r32((fp).wrapping_add(32)) as i32)) < (3i32)) as i32)) != 0) {
                    { let v_ = 9i32; let a_ = (fp).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
                } else {
                    { let v_ = 10i32; let a_ = (fp).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
                }
                l_sound_2 = (if (((((l_level_6) < (5i32)) as i32)) != 0) { 77i32 } else { 54i32 });
            } else {
                's2: {
                    let sv18: i32 = l_level_6;
                    match sv18 {
                        1i32 => {
                            l_sound_2 = (F::BtlChar_FrameMod(c, 2i32)).wrapping_add(2i32);
                            break 's2;
                        }
                        2i32 => {
                            l_sound_2 = 25i32;
                            break 's2;
                        }
                        3i32 => {
                            l_sound_2 = 34i32;
                            break 's2;
                        }
                        4i32 => {
                            l_sound_2 = 69i32;
                            break 's2;
                        }
                        5i32 => {
                            l_sound_2 = (F::BtlChar_FrameMod(c, 2i32)).wrapping_add(9i32);
                            break 's2;
                        }
                        6i32 => {
                            l_sound_2 = 37i32;
                            break 's2;
                        }
                        7i32 => {
                            l_sound_2 = 38i32;
                            break 's2;
                        }
                        8i32 => {
                            l_sound_2 = 39i32;
                            break 's2;
                        }
                        9i32 => {
                            l_sound_2 = 73i32;
                            break 's2;
                        }
                        10i32 => {
                            l_sound_2 = 78i32;
                            break 's2;
                        }
                        11i32 => {
                            l_sound_2 = 80i32;
                            break 's2;
                        }
                        12i32 => {
                            l_sound_2 = 79i32;
                            break 's2;
                        }
                        13i32 => {
                            l_sound_2 = 75i32;
                            break 's2;
                        }
                        14i32 => {
                            l_sound_2 = 14i32;
                            break 's2;
                        }
                        _ => {
                        }
                    }
                }
            }
        }
        if ((F::BtlChar_TestFxBit(c, p_chr_1, 28i32)) != 0) {
            { let v_ = 6i32; let a_ = (fp).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
            if (((((F::BtlAtk_GetHitSoundLevel(c, p_chr_1)) < (5i32)) as i32)) != 0) {
                l_sound_2 = (F::BtlChar_FrameMod(c, 2i32)).wrapping_add(4i32);
            } else {
                l_sound_2 = (F::BtlChar_FrameMod(c, 2i32)).wrapping_add(12i32);
            }
        }
        if ((F::BtlChar_TestFxBit(c, p_chr_1, 33i32)) != 0) {
            l_sound_2 = 36i32;
            { let v_ = 6i32; let a_ = (fp).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
        }
        if ((F::BtlChar_TestFxBit(c, p_chr_1, 36i32)) != 0) {
            l_sound_2 = 66i32;
            { let v_ = 6i32; let a_ = (fp).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
        }
        if ((F::BtlChar_TestFxBit(c, p_chr_1, 34i32)) != 0) {
            if (((((F::BtlAtk_GetHitFxKind(c, p_chr_1)) < (4i32)) as i32)) != 0) {
                { let v_ = 4i32; let a_ = (fp).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
                l_sound_2 = (F::BtlChar_FrameMod(c, 2i32)).wrapping_add(12i32);
            } else {
                l_sound_2 = 36i32;
                { let v_ = 5i32; let a_ = (fp).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
            }
        }
        if ((((((c.m.r32((fp).wrapping_add(32)) as i32)) >= (0i32)) as i32)) != 0) {
            l_yaw_7 = { let a0_ = rt::fadd(F32(c.m.r32((l_pose_3).wrapping_add(148))), F::BtlAtk_GetLaunchAngleA(c, p_chr_1)); F::BtlUtil_WrapAngle(c, a0_) };
            l_pitch_8 = F::BtlAtk_GetLaunchAngleB(c, p_chr_1);
            l_sinYaw_9 = F::Mathf_Sin(c, l_yaw_7);
            l_cosYaw_10 = F::Mathf_Cos(c, l_yaw_7);
            l_sinPitch_11 = F::Mathf_Sin(c, l_pitch_8);
            l_cosPitch_12 = F::Mathf_Cos(c, l_pitch_8);
            { let v_ = rt::fmul(l_cosPitch_12, l_sinYaw_9); let a_ = (fp).wrapping_add(16); c.m.w32(a_, (v_).0); };
            { let v_ = rt::fneg(l_sinPitch_11); let a_ = ((fp).wrapping_add(16)).wrapping_add(4); c.m.w32(a_, (v_).0); };
            { let v_ = rt::fmul(l_cosPitch_12, l_cosYaw_10); let a_ = ((fp).wrapping_add(16)).wrapping_add(8); c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x00000000); let a_ = ((fp).wrapping_add(16)).wrapping_add(12); c.m.w32(a_, (v_).0); };
            if ((F::BtlChar_TestFxBit(c, p_chr_1, 37i32)) != 0) {
                F::BtlOpp_GetTargetPos(c, p_chr_1, fp.wrapping_add(44));
                { let v_ = rt::fsub(F32(c.m.r32(fp.wrapping_add(44))), rt::fmul({ let a0_ = F32(c.m.r32((l_pose_3).wrapping_add(148))); F::Mathf_Sin(c, a0_) }, F::BtlOpp_GetRadius(c, p_chr_1))); let a_ = fp; c.m.w32(a_, (v_).0); };
                { let v_ = { let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(4); F32(c.m.r32(a_)) }; let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fsub(F32(c.m.r32((fp.wrapping_add(44)).wrapping_add(8))), rt::fmul({ let a0_ = F32(c.m.r32((l_pose_3).wrapping_add(148))); F::Mathf_Cos(c, a0_) }, F::BtlOpp_GetRadius(c, p_chr_1))); let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_).0); };
                { let v_ = F32(0x3f800000); let a_ = (fp).wrapping_add(12); c.m.w32(a_, (v_).0); };
                { let _ = F::EftImpact_SpawnHit(c, fp); };
            } else {
                if ((F::BtlChar_TestFxBit(c, p_chr_1, 38i32)) != 0) {
                    F::BtlOpp_GetTargetPos(c, p_chr_1, fp.wrapping_add(60));
                    { let v_ = rt::fsub(F32(c.m.r32(fp.wrapping_add(60))), rt::fmul({ let a0_ = F32(c.m.r32((l_pose_3).wrapping_add(148))); F::Mathf_Sin(c, a0_) }, F::BtlOpp_GetRadius(c, p_chr_1))); let a_ = fp; c.m.w32(a_, (v_).0); };
                    { let v_ = { let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(4); F32(c.m.r32(a_)) }; let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fsub(F32(c.m.r32((fp.wrapping_add(60)).wrapping_add(8))), rt::fmul({ let a0_ = F32(c.m.r32((l_pose_3).wrapping_add(148))); F::Mathf_Cos(c, a0_) }, F::BtlOpp_GetRadius(c, p_chr_1))); let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(0x3f800000); let a_ = (fp).wrapping_add(12); c.m.w32(a_, (v_).0); };
                    { let r_ = rt::fmul({ let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); F::BtlCharApi_GetHeight(c, a0_) }, F32(0x3f000000)); let a_ = (fp).wrapping_add(4); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    { let _ = F::EftImpact_SpawnHit(c, fp); };
                } else {
                    if ((F::BtlChar_TestFxBit(c, p_chr_1, 39i32)) != 0) {
                        F::BtlOpp_GetTargetPos(c, p_chr_1, fp.wrapping_add(76));
                        { let v_ = rt::fsub(F32(c.m.r32(fp.wrapping_add(76))), rt::fmul({ let a0_ = F32(c.m.r32((l_pose_3).wrapping_add(148))); F::Mathf_Sin(c, a0_) }, F::BtlOpp_GetRadius(c, p_chr_1))); let a_ = fp; c.m.w32(a_, (v_).0); };
                        { let v_ = { let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(4); F32(c.m.r32(a_)) }; let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
                        { let v_ = rt::fsub(F32(c.m.r32((fp.wrapping_add(76)).wrapping_add(8))), rt::fmul({ let a0_ = F32(c.m.r32((l_pose_3).wrapping_add(148))); F::Mathf_Cos(c, a0_) }, F::BtlOpp_GetRadius(c, p_chr_1))); let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_).0); };
                        { let v_ = F32(0x3f800000); let a_ = (fp).wrapping_add(12); c.m.w32(a_, (v_).0); };
                        { let r_ = { let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); F::BtlCharApi_GetHeight(c, a0_) }; let a_ = (fp).wrapping_add(4); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                        { let _ = F::EftImpact_SpawnHit(c, fp); };
                    } else {
                        rt::memset(c, fp.wrapping_add(92), 0, 36);
                        { let v_ = 262144i32; c.m.w32(fp.wrapping_add(92).wrapping_add(0), (v_) as u32); }
                        { let v_ = 2048i32; c.m.w32(fp.wrapping_add(92).wrapping_add(4), (v_) as u32); }
                        { let v_ = 256i32; c.m.w32(fp.wrapping_add(92).wrapping_add(8), (v_) as u32); }
                        { let v_ = 1024i32; c.m.w32(fp.wrapping_add(92).wrapping_add(12), (v_) as u32); }
                        { let v_ = 128i32; c.m.w32(fp.wrapping_add(92).wrapping_add(16), (v_) as u32); }
                        { let v_ = 32i32; c.m.w32(fp.wrapping_add(92).wrapping_add(20), (v_) as u32); }
                        { let v_ = 8i32; c.m.w32(fp.wrapping_add(92).wrapping_add(24), (v_) as u32); }
                        { let v_ = 16i32; c.m.w32(fp.wrapping_add(92).wrapping_add(28), (v_) as u32); }
                        { let v_ = 4i32; c.m.w32(fp.wrapping_add(92).wrapping_add(32), (v_) as u32); }
                        l_bits_13 = c.m.r32((l_obj_4).wrapping_add(3228));
                        if (((((l_bits_13) == (((0i32) as u32))) as i32)) != 0) {
                            l_bits_13 = ((((F::BtlObjAnim_QueryEvent(c, l_obj_4, ((1i32) as u64), 0i32, ((((6i32) as i64 as u64)) as u32))) as i64 as u64)) as u32);
                        }
                        {
                            l_i_15 = 0i32;
                            'l3: loop {
                                if !(((((l_i_15) < (9i32)) as i32)) != 0) { break 'l3; }
                                'c4: {
                                    l_hit_14 = ((l_bits_13) & ((((c.m.r32((fp.wrapping_add(92)).wrapping_add((((l_i_15) as u32)).wrapping_mul(4))) as i32)) as u32)));
                                    if (((((l_hit_14) != (((0i32) as u32))) as i32)) != 0) {
                                        break 'l3;
                                    }
                                }
                                { let o_ = l_i_15; let n_ = (o_).wrapping_add(1); l_i_15 = n_; };
                            }
                        }
                        if (((((l_hit_14) == (((0i32) as u32))) as i32)) != 0) {
                            {
                                l_i_15 = 0i32;
                                'l5: loop {
                                    if !(((((l_i_15) < (19i32)) as i32)) != 0) { break 'l5; }
                                    'c6: {
                                        l_hit_14 = ((l_bits_13) & ((((1i32).wrapping_shl((l_i_15) as u32)) as u32)));
                                        if (((((l_hit_14) != (((0i32) as u32))) as i32)) != 0) {
                                            break 'l5;
                                        }
                                    }
                                    { let o_ = l_i_15; let n_ = (o_).wrapping_add(1); l_i_15 = n_; };
                                }
                            }
                        }
                        if (((((l_hit_14) != (((0i32) as u32))) as i32)) != 0) {
                            { let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); let a1_ = F::BtlObjAnim_MaskToNode(c, l_hit_14); F::BtlCharApi_GetNodePos(c, a0_, a1_, fp) };
                            { let v_ = { let a0_ = F32(c.m.r32((l_pose_3).wrapping_add(20))); F::Mathf_Sin(c, a0_) }; let a_ = fp.wrapping_add(128); c.m.w32(a_, (v_).0); };
                            { let v_ = F32(0x00000000); let a_ = (fp.wrapping_add(128)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                            { let v_ = { let a0_ = F32(c.m.r32((l_pose_3).wrapping_add(20))); F::Mathf_Cos(c, a0_) }; let a_ = (fp.wrapping_add(128)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                            F::Vec4_Scale(c, fp.wrapping_add(160), fp.wrapping_add(128), F32(0x40a00000));
                            F::Vec4_Add(c, fp.wrapping_add(144), l_pose_3, fp.wrapping_add(160));
                            { let v_ = rt::fneg(F::Vec3_Dot(c, fp.wrapping_add(128), fp.wrapping_add(144))); let a_ = (fp.wrapping_add(128)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                            l_d_16 = rt::fadd(F::Vec3_Dot(c, fp.wrapping_add(128), fp), F32(c.m.r32((fp.wrapping_add(128)).wrapping_add(12))));
                            if (((rt::flt(l_d_16, F32(0x00000000)) as i32)) != 0) {
                                F::Vec4_Scale(c, fp.wrapping_add(160), fp.wrapping_add(128), rt::fneg(l_d_16));
                                F::Vec4_Add(c, fp, fp, fp.wrapping_add(160));
                            }
                            { let v_ = F32(0x3f800000); let a_ = (fp).wrapping_add(12); c.m.w32(a_, (v_).0); };
                            { let _ = F::EftImpact_SpawnHit(c, fp); };
                        }
                    }
                }
            }
        }
        if (((((l_sound_2) >= (0i32)) as i32)) != 0) {
            F::BtlCharSnd_PlayCommon(c, p_chr_1, l_sound_2);
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_SpawnHitFx", &PROBE_N, &[], true); }
    r_
}

pub fn BtlFx_SpawnFlashReq20(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFx_SpawnFlashReq20", &PROBE_N, &[(p_chr_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let r_ = 'f: {
        if ((((D::call(c, fid::BtlChar_TestFxBit, &[((p_chr_1) as u64), ((32i32) as i64 as u64)])) as i32)) != 0) {
            { let v_ = 14i32; let a_ = (fp).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
            { let v_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); let a_ = (fp).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
            { let v_ = 0i32; let a_ = (fp).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
            { let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); F::BtlCharApi_GetNodePos(c, a0_, 46i32, fp) };
            { let a1_ = (((D::call(c, fid::BtlChar_GetPos, &[((p_chr_1) as u64)])) as u32)).wrapping_add(128); F::Vec4_Copy(c, (fp).wrapping_add(16), a1_) };
            { let _ = D::call(c, fid::EftImpact_SpawnHit, &[((fp) as u64)]); };
            { let _ = D::call(c, fid::BtlCharSnd_PlayCommon, &[((p_chr_1) as u64), ((70i32) as i64 as u64)]); };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlFx_SpawnFlashReq20", &PROBE_N, &[], true); }
    r_
}

pub fn BtlFx_SpawnFlashReq2B(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFx_SpawnFlashReq2B", &PROBE_N, &[(p_chr_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let r_ = 'f: {
        if ((((D::call(c, fid::BtlChar_TestFxBit, &[((p_chr_1) as u64), ((43i32) as i64 as u64)])) as i32)) != 0) {
            { let v_ = 13i32; let a_ = (fp).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
            { let v_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); let a_ = (fp).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
            { let v_ = 0i32; let a_ = (fp).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
            { let a1_ = (((D::call(c, fid::BtlChar_GetPos, &[((p_chr_1) as u64)])) as u32)).wrapping_add(128); F::Vec4_Copy(c, (fp).wrapping_add(16), a1_) };
            { let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); F::BtlCharApi_GetNodePos(c, a0_, 16i32, fp) };
            { let _ = D::call(c, fid::EftImpact_SpawnHit, &[((fp) as u64)]); };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlFx_SpawnFlashReq2B", &PROBE_N, &[], true); }
    r_
}

pub fn BtlFx_SpawnHitSparkReq17(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFx_SpawnHitSparkReq17", &PROBE_N, &[(p_chr_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_type_2: i32 = 0;
    let r_ = 'f: {
        if ((((D::call(c, fid::BtlChar_TestFxBit, &[((p_chr_1) as u64), ((23i32) as i64 as u64)])) as i32)) != 0) {
            l_type_2 = F::BtlKiBlast_GetType(c, p_chr_1);
            F::Vec4_Copy(c, fp, (p_chr_1).wrapping_add(3536));
            { let v_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); let a_ = (fp).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
            { let v_ = (c.m.r32((p_chr_1).wrapping_add(3568)) as i32); let a_ = (fp).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
            { let v_ = F::BtlKiBlast_GetUnk3(c, p_chr_1); let a_ = (fp).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
            { let v_ = F32(0x3f800000); let a_ = (fp).wrapping_add(28); c.m.w32(a_, (v_).0); };
            if ((F::BtlChar_TestMemberUnk70(c, p_chr_1)) != 0) {
                l_type_2 = 1i32;
            }
            if ((((((c.m.r32((fp).wrapping_add(20)) as i32)) == (21i32)) as i32)) != 0) {
                { let v_ = 31i32; let a_ = (fp).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
            }
            if ((((((c.m.r32((fp).wrapping_add(20)) as i32)) == (35i32)) as i32)) != 0) {
                { let v_ = 45i32; let a_ = (fp).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
            }
            if (((((l_type_2) == (5i32)) as i32)) != 0) {
            } else {
                if (((((l_type_2) == (4i32)) as i32)) != 0) {
                    { let a1_ = { let a0_ = (c.m.r32((fp).wrapping_add(20)) as i32); F::BtlObj_GetNodeSide(c, a0_) }; { let _ = F::EftDisc_SpawnHeld(c, fp, a1_); } };
                } else {
                    if (((((l_type_2) == (2i32)) as i32)) != 0) {
                    } else {
                        if (((((l_type_2) == (3i32)) as i32)) != 0) {
                        } else {
                            { let _ = F::EftBlastCharge_Start(c, fp); };
                        }
                    }
                }
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlFx_SpawnHitSparkReq17", &PROBE_N, &[], true); }
    r_
}

pub fn BtlFx_UpdateAura(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFx_UpdateAura", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_obj_2: u32 = 0;
    let mut l_g_3: u32 = 0;
    let mut l_level_4: F32 = F32(0);
    let r_ = 'f: {
    l_obj_2 = ((D::call(c, fid::BtlChar_GetObj, &[((p_chr_1) as u64)])) as u32);
    l_g_3 = ((D::call(c, fid::BtlMember_GetActiveGauge, &[((p_chr_1) as u64)])) as u32);
    if ((((((c.m.r32((l_g_3).wrapping_add(16)) as i32)) > (0i32)) as i32)) != 0) {
        l_level_4 = rt::fdiv(rt::i2f((c.m.r32((l_g_3).wrapping_add(12)) as i32)), rt::i2f((c.m.r32((l_g_3).wrapping_add(16)) as i32)));
        if (((rt::flt(l_level_4, F32(0x3f000000)) as i32)) != 0) {
            l_level_4 = F32(0x00000000);
        }
    } else {
        l_level_4 = F32(0x00000000);
    }
    if ((((((((F::BtlParam_GetFlags(c, p_chr_1)) as u64)) as i32)) & (16i32))) != 0) {
        if ((((!((((D::call(c, fid::BtlMember_HasAnyListedAbility, &[((p_chr_1) as u64)])) as i32)) != 0)) as i32)) != 0) {
            l_level_4 = F32(0x00000000);
        }
    }
    if ((((c.m.r32((l_obj_2).wrapping_add(2624))) & (((262144i32) as u32)))) != 0) {
        l_level_4 = F32(0x3f800000);
    }
    if ((((D::call(c, fid::BtlChar_TestFxBit, &[((p_chr_1) as u64), ((19i32) as i64 as u64)])) as i32)) != 0) {
        l_level_4 = F32(0x3f800000);
    }
    if ((((D::call(c, fid::BtlChar_TestFxBit, &[((p_chr_1) as u64), ((20i32) as i64 as u64)])) as i32)) != 0) {
        l_level_4 = F32(0x00000000);
        { let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); { let _ = F::EftAura_Command(c, a0_, 7i32); } };
    }
    if (((rt::fgt(l_level_4, F32(0x00000000)) as i32)) != 0) {
        { let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); { let _ = F::EftAura_Command(c, a0_, 0i32); } };
        { let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); F::EftAura_SetLevel(c, a0_, l_level_4) };
    } else {
        { let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); { let _ = F::EftAura_Command(c, a0_, 1i32); } };
    }
    if ((((!((((((((F::BtlParam_GetFlags(c, p_chr_1)) as u64)) as i32)) & (16i32))) != 0)) as i32)) != 0) {
        if ((((D::call(c, fid::BtlChar_IsFxBitNew, &[((p_chr_1) as u64), ((7i32) as i64 as u64)])) as i32)) != 0) {
            { let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); { let _ = F::EftAura_Command(c, a0_, 2i32); } };
            { let _ = F::BtlParam_GetUnk0(c, p_chr_1); };
        }
        if ((((D::call(c, fid::BtlChar_IsFxBitEnded, &[((p_chr_1) as u64), ((7i32) as i64 as u64)])) as i32)) != 0) {
            { let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); { let _ = F::EftAura_Command(c, a0_, 3i32); } };
            { let _ = F::BtlParam_GetUnk0(c, p_chr_1); };
        }
        if ((((D::call(c, fid::BtlChar_TestFxBit, &[((p_chr_1) as u64), ((8i32) as i64 as u64)])) as i32)) != 0) {
            { let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); { let _ = F::EftAura_Command(c, a0_, 4i32); } };
            { let _ = F::BtlParam_GetUnk0(c, p_chr_1); };
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlFx_UpdateAura", &PROBE_N, &[], true); }
    r_
}

pub fn BtlFx_UpdateChargeFx(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFx_UpdateChargeFx", &PROBE_N, &[(p_chr_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        if ((((D::call(c, fid::BtlChar_TestFxBit, &[((p_chr_1) as u64), ((7i32) as i64 as u64)])) as i32)) != 0) {
            if ((((!(({ let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); F::EftCharge_IsActive(c, a0_) }) != 0)) as i32)) != 0) {
                { let v_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let _ = F::EftCharge_Start(c, fp); };
            }
        } else {
            if (({ let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); F::EftCharge_IsActive(c, a0_) }) != 0) {
                { let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); { let _ = F::EftCharge_Stop(c, a0_); } };
            }
        }
        if ((((D::call(c, fid::BtlChar_TestFxBit, &[((p_chr_1) as u64), ((8i32) as i64 as u64)])) as i32)) != 0) {
            { let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); { let _ = F::EftCharge_Burst(c, a0_); } };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlFx_UpdateChargeFx", &PROBE_N, &[], true); }
    r_
}

pub fn BtlFx_UpdatePowerUpLook(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFx_UpdatePowerUpLook", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_obj_2: u32 = 0;
    let r_ = 'f: {
    l_obj_2 = ((D::call(c, fid::BtlChar_GetObj, &[((p_chr_1) as u64)])) as u32);
    if ((((((((!((((((((F::BtlParam_GetFlags(c, p_chr_1)) as u64)) as i32)) & (16i32))) != 0)) as i32)) != 0) || ((((D::call(c, fid::BtlMember_HasAnyListedAbility, &[((p_chr_1) as u64)])) as i32)) != 0)) as i32)) != 0) {
        if ((((D::call(c, fid::BtlChar_TestFxBit, &[((p_chr_1) as u64), ((5i32) as i64 as u64)])) as i32)) != 0) {
            { let a0_ = (c.m.r32(p_chr_1) as i32); { let _ = F::EftGlow_Request(c, a0_, 2i32); } };
            { let a0_ = (c.m.r32(p_chr_1) as i32); { let _ = F::EftAura_Command(c, a0_, 5i32); } };
            F::BtlObj_SetColorMode(c, l_obj_2, 6i32, 0i32);
        } else {
            if ((((D::call(c, fid::BtlChar_IsFxBitNew, &[((p_chr_1) as u64), ((4i32) as i64 as u64)])) as i32)) != 0) {
                { let a0_ = (c.m.r32(p_chr_1) as i32); { let _ = F::EftGlow_Request(c, a0_, 0i32); } };
                { let a0_ = (c.m.r32(p_chr_1) as i32); { let _ = F::EftAura_Command(c, a0_, 6i32); } };
                F::BtlObj_SetColorMode(c, l_obj_2, 6i32, 1i32);
            }
            if ((((D::call(c, fid::BtlChar_IsFxBitEnded, &[((p_chr_1) as u64), ((4i32) as i64 as u64)])) as i32)) != 0) {
                { let a0_ = (c.m.r32(p_chr_1) as i32); { let _ = F::EftGlow_Request(c, a0_, 1i32); } };
                { let a0_ = (c.m.r32(p_chr_1) as i32); { let _ = F::EftAura_Command(c, a0_, 5i32); } };
                F::BtlObj_SetColorMode(c, l_obj_2, 6i32, 0i32);
            }
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlFx_UpdatePowerUpLook", &PROBE_N, &[], true); }
    r_
}

pub fn BtlFx_SpawnReq6(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFx_SpawnReq6", &PROBE_N, &[(p_chr_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        if ((((D::call(c, fid::BtlChar_TestFxBit, &[((p_chr_1) as u64), ((6i32) as i64 as u64)])) as i32)) != 0) {
            { let v_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            { let _ = F::EftShock_Start(c, fp); };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlFx_SpawnReq6", &PROBE_N, &[], true); }
    r_
}

pub fn BtlFx_UpdateScreenFilter(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFx_UpdateScreenFilter", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_a_2: u8 = 0;
    let mut l_b_3: u8 = 0;
    let mut l_c_4: u8 = 0;
    let r_ = 'f: {
    if ((((D::call(c, fid::BtlChar_TestFxBit, &[((p_chr_1) as u64), ((9i32) as i64 as u64)])) as i32)) != 0) {
        { let r_ = F32(0x3e2aaaab); let a_ = (p_chr_1).wrapping_add(4928); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        if (((rt::fgt(F32(c.m.r32((p_chr_1).wrapping_add(4928))), F32(0x3f800000)) as i32)) != 0) {
            { let v_ = F32(0x3f800000); let a_ = (p_chr_1).wrapping_add(4928); c.m.w32(a_, (v_).0); };
        }
    } else {
        if ((((D::call(c, fid::BtlChar_TestFxBit, &[((p_chr_1) as u64), ((10i32) as i64 as u64)])) as i32)) != 0) {
            { let v_ = F32(0x3f800000); let a_ = (p_chr_1).wrapping_add(4928); c.m.w32(a_, (v_).0); };
        } else {
            { let r_ = F32(0x3eaaaaab); let a_ = (p_chr_1).wrapping_add(4928); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            if (((rt::flt(F32(c.m.r32((p_chr_1).wrapping_add(4928))), F32(0x00000000)) as i32)) != 0) {
                { let v_ = F32(0x00000000); let a_ = (p_chr_1).wrapping_add(4928); c.m.w32(a_, (v_).0); };
            }
        }
    }
    l_a_2 = ((rt::f2i(rt::fmul(F32(c.m.r32((p_chr_1).wrapping_add(4928))), F32(0x41000000)))) as u8);
    l_b_3 = ((rt::f2i(rt::fmul(F32(c.m.r32((p_chr_1).wrapping_add(4928))), F32(0x42000000)))) as u8);
    l_c_4 = ((rt::f2i(rt::fmul(F32(c.m.r32((p_chr_1).wrapping_add(4928))), F32(0x43000000)))) as u8);
    { let a0_ = (c.m.r32((p_chr_1).wrapping_add(8)) as i32); F::StgBlur_SetPasses(c, a0_, 3i32) };
    { let a0_ = (c.m.r32((p_chr_1).wrapping_add(8)) as i32); F::StgBlur_SetColor3Rgba(c, a0_, ((((128i32) as i64 as u64)) as u8), ((((128i32) as i64 as u64)) as u8), ((((128i32) as i64 as u64)) as u8), ((((64i32) as i64 as u64)) as u8)) };
    { let a0_ = (c.m.r32((p_chr_1).wrapping_add(8)) as i32); F::StgBlur_SetColor0Rgba(c, a0_, ((((128i32) as i64 as u64)) as u8), ((((128i32) as i64 as u64)) as u8), ((((128i32) as i64 as u64)) as u8), ((((((l_a_2) as i32)) as i64 as u64)) as u8)) };
    { let a0_ = (c.m.r32((p_chr_1).wrapping_add(8)) as i32); F::StgBlur_SetColor1Rgba(c, a0_, ((((128i32) as i64 as u64)) as u8), ((((128i32) as i64 as u64)) as u8), ((((128i32) as i64 as u64)) as u8), ((((((l_b_3) as i32)) as i64 as u64)) as u8)) };
    { let a0_ = (c.m.r32((p_chr_1).wrapping_add(8)) as i32); F::StgBlur_SetColor2Rgba(c, a0_, ((((128i32) as i64 as u64)) as u8), ((((128i32) as i64 as u64)) as u8), ((((128i32) as i64 as u64)) as u8), ((((((l_c_4) as i32)) as i64 as u64)) as u8)) };
    if ((((D::call(c, fid::BtlChar_TestFxBit, &[((p_chr_1) as u64), ((9i32) as i64 as u64)])) as i32)) != 0) {
        { let a0_ = (c.m.r32((p_chr_1).wrapping_add(8)) as i32); let a1_ = (((D::call(c, fid::BtlChar_GetPos, &[((p_chr_1) as u64)])) as u32)).wrapping_add(128); F::StgBlur_SetCenter(c, a0_, a1_, 0i32) };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlFx_UpdateScreenFilter", &PROBE_N, &[], true); }
    r_
}

pub fn BtlFx_SpawnBurstReqC(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFx_SpawnBurstReqC", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    if ((((D::call(c, fid::BtlChar_TestFxBit, &[((p_chr_1) as u64), ((12i32) as i64 as u64)])) as i32)) != 0) {
        { let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); { let _ = F::EftShotFx_Start(c, a0_, ((((0i32) as i64 as u64)) as u8)); } };
    }
    if ((((D::call(c, fid::BtlChar_TestFxBit, &[((p_chr_1) as u64), ((13i32) as i64 as u64)])) as i32)) != 0) {
        { let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); { let _ = F::EftShotFx_Start(c, a0_, ((((1i32) as i64 as u64)) as u8)); } };
    }
    if ((((D::call(c, fid::BtlChar_TestFxBit, &[((p_chr_1) as u64), ((14i32) as i64 as u64)])) as i32)) != 0) {
        { let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); { let _ = F::EftShotFx_Start(c, a0_, ((((2i32) as i64 as u64)) as u8)); } };
    }
    if ((((D::call(c, fid::BtlChar_TestFxBit, &[((p_chr_1) as u64), ((15i32) as i64 as u64)])) as i32)) != 0) {
        { let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); { let _ = F::EftShotFx_Start(c, a0_, ((((3i32) as i64 as u64)) as u8)); } };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlFx_SpawnBurstReqC", &PROBE_N, &[], true); }
    r_
}

pub fn BtlFx_SpawnFastMoveFx(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFx_SpawnFastMoveFx", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_pose_2: u32 = 0;
    let r_ = 'f: {
    if ((((D::call(c, fid::BtlChar_TestFxBit, &[((p_chr_1) as u64), ((11i32) as i64 as u64)])) as i32)) != 0) {
        l_pose_2 = ((D::call(c, fid::BtlChar_GetPos, &[((p_chr_1) as u64)])) as u32);
        if (((rt::fgt(F32(c.m.r32((l_pose_2).wrapping_add(152))), F32(0x3c23d70a)) as i32)) != 0) {
            if (((rt::fgt(rt::fdiv(F::Vec3_Length(c, (l_pose_2).wrapping_add(64)), F32(c.m.r32((l_pose_2).wrapping_add(152)))), F32(0x3f000000)) as i32)) != 0) {
                { let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); { let _ = F::EftSpdLine_SpawnBodyTrails(c, a0_); } };
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlFx_SpawnFastMoveFx", &PROBE_N, &[], true); }
    r_
}

pub fn BtlFx_SpawnSpeedLines(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFx_SpawnSpeedLines", &PROBE_N, &[(p_chr_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(112) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 112);
    let mut l_angle_2: F32 = F32(0);
    let mut l_eps_3: F32 = F32(0);
    let r_ = 'f: {
        if ((((D::call(c, fid::BtlChar_TestFxBit, &[((p_chr_1) as u64), ((21i32) as i64 as u64)])) as i32)) != 0) {
            rt::memset(c, fp, 0, 96);
            rt::memset(c, fp.wrapping_add(0), 0, 16);
            { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(0).wrapping_add(0), (v_).0); }
            { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(0).wrapping_add(4), (v_).0); }
            { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(0).wrapping_add(8), (v_).0); }
            { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(0).wrapping_add(12), (v_).0); }
            { let v_ = 128i32; c.m.w32(fp.wrapping_add(16), (v_) as u32); }
            rt::memset(c, fp.wrapping_add(20), 0, 12);
            { let v_ = 128i32; c.m.w32(fp.wrapping_add(20).wrapping_add(0), (v_) as u32); }
            { let v_ = 128i32; c.m.w32(fp.wrapping_add(20).wrapping_add(4), (v_) as u32); }
            { let v_ = 64i32; c.m.w32(fp.wrapping_add(20).wrapping_add(8), (v_) as u32); }
            { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(32), (v_).0); }
            { let v_ = F32(0x44480000); c.m.w32(fp.wrapping_add(36), (v_).0); }
            { let v_ = F32(0x3fc00000); c.m.w32(fp.wrapping_add(40), (v_).0); }
            { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(44), (v_).0); }
            { let v_ = F32(0x42480000); c.m.w32(fp.wrapping_add(48), (v_).0); }
            { let v_ = 2i32; c.m.w32(fp.wrapping_add(52), (v_) as u32); }
            { let v_ = 48i32; c.m.w32(fp.wrapping_add(56), (v_) as u32); }
            { let v_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); c.m.w32(fp.wrapping_add(60), (v_) as u32); }
            { let v_ = 1i32; c.m.w32(fp.wrapping_add(64), (v_) as u32); }
            { let v_ = 1i32; c.m.w32(fp.wrapping_add(68), (v_) as u32); }
            { let v_ = 0i32; c.m.w32(fp.wrapping_add(72), (v_) as u32); }
            { let v_ = 0i32; c.m.w32(fp.wrapping_add(76), (v_) as u32); }
            { let v_ = 1i32; c.m.w32(fp.wrapping_add(80), (v_) as u32); }
            F::BtlOpp_GetDelta(c, p_chr_1, fp.wrapping_add(96));
            l_eps_3 = F32(0x3a83126e);
            l_angle_2 = F32(0x00000000);
            if (((((((rt::fgt({ let a0_ = F32(c.m.r32(fp.wrapping_add(96))); rt::ffabs(a0_) }, l_eps_3) as i32)) != 0) || (((rt::fgt({ let a0_ = F32(c.m.r32((fp.wrapping_add(96)).wrapping_add(8))); rt::ffabs(a0_) }, l_eps_3) as i32)) != 0)) as i32)) != 0) {
                l_angle_2 = { let a0_ = rt::fsub({ let a0_ = F32(c.m.r32(fp.wrapping_add(96))); let a1_ = F32(c.m.r32((fp.wrapping_add(96)).wrapping_add(8))); F::Port_atan2f(c, a0_, a1_) }, F32(c.m.r32((p_chr_1).wrapping_add(1184)))); F::BtlUtil_WrapAngle(c, a0_) };
            }
            { let v_ = rt::fadd(rt::fmul(rt::fdiv(rt::ffabs(l_angle_2), F32(0x40490fda)), F32(0x3f000000)), F32(0x3e999999)); let a_ = (fp).wrapping_add(32); c.m.w32(a_, (v_).0); };
            { let v_ = rt::fsub(F32(0xc1a00000), rt::fmul(F::Vec3_Length(c, fp.wrapping_add(96)), F32(0x3dcccccc))); let a_ = (fp).wrapping_add(44); c.m.w32(a_, (v_).0); };
            if (((rt::flt(F32(c.m.r32((fp).wrapping_add(44))), F32(0xc2480000)) as i32)) != 0) {
                { let v_ = F32(0xc2480000); let a_ = (fp).wrapping_add(44); c.m.w32(a_, (v_).0); };
            }
            { let _ = F::EftRay_CreateByValue(c, fp); };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlFx_SpawnSpeedLines", &PROBE_N, &[], true); }
    r_
}

pub fn BtlFx_SpawnReq18(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFx_SpawnReq18", &PROBE_N, &[(p_chr_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        if ((((D::call(c, fid::BtlChar_TestFxBit, &[((p_chr_1) as u64), ((24i32) as i64 as u64)])) as i32)) != 0) {
            { let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); F::BtlCharApi_GetNodePos(c, a0_, 17i32, fp) };
            { let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); { let _ = F::EftRay_StartHit(c, a0_, fp, 0i32, 0i32, 1i32, F32(0x3f000000)); } };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlFx_SpawnReq18", &PROBE_N, &[], true); }
    r_
}

pub fn btl_char_fx__Fx_FootDust(c: &mut Ctx, mut p_objId_1: i32, mut p_pos_2: u32, mut p_scale_3: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "btl_char_fx__Fx_FootDust", &PROBE_N, &[(p_objId_1) as u32, (p_pos_2) as u32, (p_scale_3).0], false); }
    let r_ = 'f: {
    F::EftGndDust_SpawnImpact(c, p_objId_1, p_pos_2, p_scale_3);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "btl_char_fx__Fx_FootDust", &PROBE_N, &[], true); }
    r_
}

pub fn BtlFx_UpdateGroundFx(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFx_UpdateGroundFx", &PROBE_N, &[(p_chr_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let mut l_under_2: i32 = 0;
    let r_ = 'f: {
        if (((({ let a_ = (((D::call(c, fid::BtlChar_GetPos, &[((p_chr_1) as u64)])) as u32)).wrapping_add(208); c.m.r32(a_) }) & (((805306469i32) as u32)))) != 0) {
            { let _ = D::call(c, fid::BtlChar_ClearFxBit, &[((p_chr_1) as u64), ((47i32) as i64 as u64)]); };
            { let _ = D::call(c, fid::BtlChar_ClearFxBit, &[((p_chr_1) as u64), ((48i32) as i64 as u64)]); };
            { let _ = D::call(c, fid::BtlChar_ClearFxBit, &[((p_chr_1) as u64), ((49i32) as i64 as u64)]); };
            { let _ = D::call(c, fid::BtlChar_ClearFxBit, &[((p_chr_1) as u64), ((50i32) as i64 as u64)]); };
            { let _ = D::call(c, fid::BtlChar_ClearFxBit, &[((p_chr_1) as u64), ((51i32) as i64 as u64)]); };
            { let _ = D::call(c, fid::BtlChar_ClearFxBit, &[((p_chr_1) as u64), ((52i32) as i64 as u64)]); };
            { let _ = D::call(c, fid::BtlChar_ClearFxBit, &[((p_chr_1) as u64), ((53i32) as i64 as u64)]); };
            { let _ = D::call(c, fid::BtlChar_ClearFxBit, &[((p_chr_1) as u64), ((54i32) as i64 as u64)]); };
        }
        if ((((D::call(c, fid::BtlChar_IsFxBitNew, &[((p_chr_1) as u64), ((48i32) as i64 as u64)])) as i32)) != 0) {
            { let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); F::EftGndDust_SetSlide(c, a0_, 0i32, F32(0x3f800000)) };
        }
        if ((((D::call(c, fid::BtlChar_IsFxBitEnded, &[((p_chr_1) as u64), ((48i32) as i64 as u64)])) as i32)) != 0) {
            { let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); F::EftGndDust_SetSlide(c, a0_, 1i32, F32(0x3f800000)) };
        }
        if ((((D::call(c, fid::BtlChar_IsFxBitNew, &[((p_chr_1) as u64), ((51i32) as i64 as u64)])) as i32)) != 0) {
            { let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); F::EftGndDust_SetDash(c, a0_, 0i32, F32(0x3f800000)) };
        }
        if ((((D::call(c, fid::BtlChar_IsFxBitEnded, &[((p_chr_1) as u64), ((51i32) as i64 as u64)])) as i32)) != 0) {
            { let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); F::EftGndDust_SetDash(c, a0_, 1i32, F32(0x3f800000)) };
        }
        if ((((D::call(c, fid::BtlChar_TestFxBit, &[((p_chr_1) as u64), ((52i32) as i64 as u64)])) as i32)) != 0) {
            { let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); F::EftGndDust_SpawnBurst(c, a0_, F32(0x3f800000), F32(0x3f800000)) };
        }
        if ((((D::call(c, fid::BtlChar_TestFxBit, &[((p_chr_1) as u64), ((53i32) as i64 as u64)])) as i32)) != 0) {
            { let v_ = F32(0x00000000); let a_ = fp.wrapping_add(32); c.m.w32(a_, (v_).0); };
            l_under_2 = 0i32;
            if ((F::BtlStage_GetWaterLevel(c, fp.wrapping_add(32))) != 0) {
                if (((rt::fgt({ let a_ = ((((D::call(c, fid::BtlChar_GetPos, &[((p_chr_1) as u64)])) as u32)).wrapping_add(176)).wrapping_add(4); F32(c.m.r32(a_)) }, F32(c.m.r32(fp.wrapping_add(32)))) as i32)) != 0) {
                    l_under_2 = 1i32;
                }
            }
            if ((l_under_2) != 0) {
                { let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); F::EftGndDust_SpawnBurst(c, a0_, F32(0x3f800000), F32(0x3f800000)) };
            } else {
                { let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); let a1_ = (((D::call(c, fid::BtlChar_GetPos, &[((p_chr_1) as u64)])) as u32)).wrapping_add(176); let a2_ = (((D::call(c, fid::BtlChar_GetPos, &[((p_chr_1) as u64)])) as u32)).wrapping_add(192); F::EftGndDust_SpawnLanding(c, a0_, a1_, a2_, F32(0x3f800000)) };
            }
        }
        if ((((D::call(c, fid::BtlChar_TestFxBit, &[((p_chr_1) as u64), ((49i32) as i64 as u64)])) as i32)) != 0) {
            { let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); F::EftGndDust_Stub(c, a0_, F32(0x3f800000)) };
        }
        if ((((D::call(c, fid::BtlChar_TestFxBit, &[((p_chr_1) as u64), ((50i32) as i64 as u64)])) as i32)) != 0) {
            { let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); F::BtlCharApi_GetNodePos(c, a0_, 15i32, fp) };
            { let v_ = { let a_ = ((((D::call(c, fid::BtlChar_GetPos, &[((p_chr_1) as u64)])) as u32)).wrapping_add(176)).wrapping_add(4); F32(c.m.r32(a_)) }; let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
            { let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); F::BtlCharApi_GetNodePos(c, a0_, 11i32, fp.wrapping_add(16)) };
            { let v_ = { let a_ = ((((D::call(c, fid::BtlChar_GetPos, &[((p_chr_1) as u64)])) as u32)).wrapping_add(176)).wrapping_add(4); F32(c.m.r32(a_)) }; let a_ = (fp.wrapping_add(16)).wrapping_add(4); c.m.w32(a_, (v_).0); };
            { let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); F::btl_char_fx__Fx_FootDust(c, a0_, fp, F32(0x3f800000)) };
            { let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); F::btl_char_fx__Fx_FootDust(c, a0_, fp.wrapping_add(16), F32(0x3f800000)) };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlFx_UpdateGroundFx", &PROBE_N, &[], true); }
    r_
}

pub fn BtlFx_UpdateWaterFx(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFx_UpdateWaterFx", &PROBE_N, &[(p_chr_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_pose_2: u32 = 0;
    let r_ = 'f: {
        l_pose_2 = ((D::call(c, fid::BtlChar_GetPos, &[((p_chr_1) as u64)])) as u32);
        if (((((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((17i32) as i64 as u64)])) as i32)) != (F::BtlChar_TestPrevFlag(c, p_chr_1, ((((17i32) as i64 as u64)) as u32)))) as i32)) != 0) {
            if ((((!((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((36i32) as i64 as u64)])) as i32)) != 0)) as i32)) != 0) {
                if ((F::BtlStage_GetWaterLevel(c, fp.wrapping_add(16))) != 0) {
                    { let v_ = F32(c.m.r32(l_pose_2)); let a_ = fp; c.m.w32(a_, (v_).0); };
                    { let v_ = F32(c.m.r32(fp.wrapping_add(16))); let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(c.m.r32((l_pose_2).wrapping_add(8))); let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(0x3f800000); let a_ = (fp).wrapping_add(12); c.m.w32(a_, (v_).0); };
                    { let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); let a3_ = { let a_ = ((((D::call(c, fid::BtlChar_GetPos, &[((p_chr_1) as u64)])) as u32)).wrapping_add(64)).wrapping_add(4); F32(c.m.r32(a_)) }; { let _ = F::EftWater_AddSplashFor(c, a0_, fp, ((((0i32) as i64 as u64)) as u8), a3_); } };
                    if ((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((17i32) as i64 as u64)])) as i32)) != 0) {
                        { let _ = D::call(c, fid::BtlCharSnd_PlayCommon, &[((p_chr_1) as u64), ((58i32) as i64 as u64)]); };
                    } else {
                        { let _ = D::call(c, fid::BtlCharSnd_PlayCommon, &[((p_chr_1) as u64), ((59i32) as i64 as u64)]); };
                    }
                    if ((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((17i32) as i64 as u64)])) as i32)) != 0) {
                        { let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); { let _ = F::EftBubble_StartBurst(c, a0_); } };
                    }
                }
            }
        }
        if ((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((18i32) as i64 as u64)])) as i32)) != 0) {
            if (((rt::fgt(F::Vec3_Length(c, (l_pose_2).wrapping_add(64)), F32(0x40cf684a)) as i32)) != 0) {
                { let _ = D::call(c, fid::BtlChar_SetFxBit, &[((p_chr_1) as u64), ((25i32) as i64 as u64)]); };
            }
        }
        if ((((D::call(c, fid::BtlChar_IsFxBitNew, &[((p_chr_1) as u64), ((25i32) as i64 as u64)])) as i32)) != 0) {
            { let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); { let _ = F::EftWater_SetWake(c, a0_, ((((0i32) as i64 as u64)) as u8)); } };
        }
        if ((((D::call(c, fid::BtlChar_IsFxBitEnded, &[((p_chr_1) as u64), ((25i32) as i64 as u64)])) as i32)) != 0) {
            { let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); { let _ = F::EftWater_SetWake(c, a0_, ((((1i32) as i64 as u64)) as u8)); } };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlFx_UpdateWaterFx", &PROBE_N, &[], true); }
    r_
}

pub fn BtlFx_UpdateAimedFxReq28(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFx_UpdateAimedFxReq28", &PROBE_N, &[(p_chr_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_pitch_2: F32 = F32(0);
    let mut l_yaw_3: F32 = F32(0);
    let r_ = 'f: {
        if ((((D::call(c, fid::BtlChar_IsFxBitNew, &[((p_chr_1) as u64), ((40i32) as i64 as u64)])) as i32)) != 0) {
            l_pitch_2 = F::BtlAtk_GetLaunchAngleBOf(c, p_chr_1, ((((134i32) as i64 as u64)) as u32));
            l_yaw_3 = { let a0_ = rt::fadd({ let a_ = (((D::call(c, fid::BtlChar_GetPos, &[((p_chr_1) as u64)])) as u32)).wrapping_add(148); F32(c.m.r32(a_)) }, F::BtlAtk_GetLaunchAngleAOf(c, p_chr_1, ((((134i32) as i64 as u64)) as u32))); F::BtlUtil_WrapAngle(c, a0_) };
            { let _ = F::memset(c, fp, 0i32, 32u32); };
            { let v_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); let a_ = (fp).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
            { let v_ = (c.m.r32((p_chr_1).wrapping_add(4948)) as i32); let a_ = (fp).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
            { let v_ = F32(0x3f800000); let a_ = (fp).wrapping_add(24); c.m.w32(a_, (v_).0); };
            { let v_ = rt::fmul(F::Mathf_Cos(c, l_pitch_2), F::Mathf_Sin(c, l_yaw_3)); let a_ = fp; c.m.w32(a_, (v_).0); };
            { let v_ = rt::fneg(F::Mathf_Sin(c, l_pitch_2)); let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
            { let v_ = rt::fmul(F::Mathf_Cos(c, l_pitch_2), F::Mathf_Cos(c, l_yaw_3)); let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_).0); };
            F::EftRushBurst_Start(c, fp);
        }
        if ((((D::call(c, fid::BtlChar_IsFxBitNew, &[((p_chr_1) as u64), ((41i32) as i64 as u64)])) as i32)) != 0) {
            { let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); { let _ = F::EftRushBurst_Stage2(c, a0_); } };
        }
        if ((((D::call(c, fid::BtlChar_IsFxBitNew, &[((p_chr_1) as u64), ((42i32) as i64 as u64)])) as i32)) != 0) {
            { let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); { let _ = F::EftRushBurst_Stop(c, a0_); } };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlFx_UpdateAimedFxReq28", &PROBE_N, &[], true); }
    r_
}

pub fn BtlFx_UpdateReq1A(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFx_UpdateReq1A", &PROBE_N, &[(p_chr_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        if ((((D::call(c, fid::BtlChar_IsFxBitNew, &[((p_chr_1) as u64), ((26i32) as i64 as u64)])) as i32)) != 0) {
            { let v_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); let a_ = fp; c.m.w32(a_, (v_) as u32); };
            { let v_ = F32(0x3f800000); let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_).0); };
            { let v_ = 0i32; let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
            { let _ = F::EftBodyFx_Start(c, fp); };
        }
        if ((((D::call(c, fid::BtlChar_IsFxBitEnded, &[((p_chr_1) as u64), ((26i32) as i64 as u64)])) as i32)) != 0) {
            { let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); { let _ = F::EftBodyFx_Stop(c, a0_); } };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlFx_UpdateReq1A", &PROBE_N, &[], true); }
    r_
}

pub fn BtlFx_UpdateReq38(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFx_UpdateReq38", &PROBE_N, &[(p_chr_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        if ((((D::call(c, fid::BtlChar_IsFxBitNew, &[((p_chr_1) as u64), ((56i32) as i64 as u64)])) as i32)) != 0) {
            { let v_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); let a_ = fp; c.m.w32(a_, (v_) as u32); };
            { let v_ = F32(0x3f800000); let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
            { let _ = F::EftAbsorb_Start(c, fp); };
        }
        if ((((D::call(c, fid::BtlChar_IsFxBitEnded, &[((p_chr_1) as u64), ((56i32) as i64 as u64)])) as i32)) != 0) {
            { let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); { let _ = F::EftAbsorb_Stop(c, a0_); } };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlFx_UpdateReq38", &PROBE_N, &[], true); }
    r_
}

pub fn BtlFx_SpawnReq37(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFx_SpawnReq37", &PROBE_N, &[(p_chr_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        if ((((D::call(c, fid::BtlChar_TestFxBit, &[((p_chr_1) as u64), ((55i32) as i64 as u64)])) as i32)) != 0) {
            { let v_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); let a_ = fp; c.m.w32(a_, (v_) as u32); };
            { let v_ = F32(0x3f800000); let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
            F::EftAbsorb_StartHands(c, fp);
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlFx_SpawnReq37", &PROBE_N, &[], true); }
    r_
}

pub fn BtlFx_SpawnFlashReq2E(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFx_SpawnFlashReq2E", &PROBE_N, &[(p_chr_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let r_ = 'f: {
        if ((((D::call(c, fid::BtlChar_TestFxBit, &[((p_chr_1) as u64), ((46i32) as i64 as u64)])) as i32)) != 0) {
            { let v_ = 11i32; let a_ = (fp).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
            { let v_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); let a_ = (fp).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
            { let v_ = 0i32; let a_ = (fp).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
            F::Vec4_Copy(c, (fp).wrapping_add(16), (p_chr_1).wrapping_add(4960));
            { let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); F::BtlCharApi_GetNodePos(c, a0_, 17i32, fp) };
            { let _ = D::call(c, fid::EftImpact_SpawnHit, &[((fp) as u64)]); };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlFx_SpawnFlashReq2E", &PROBE_N, &[], true); }
    r_
}

pub fn BtlFx_UpdateCharaFx(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFx_UpdateCharaFx", &PROBE_N, &[(p_chr_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_on_2: i32 = 0;
    let r_ = 'f: {
        l_on_2 = 0i32;
        if ((((!(({ let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); F::BtlCharApi_IsModelNew(c, a0_) }) != 0)) as i32)) != 0) {
            's1: {
                let sv3: i32 = { let a_ = ((D::call(c, fid::BtlMember_GetActive, &[((p_chr_1) as u64)])) as u32); (c.m.r32(a_) as i32) };
                match sv3 {
                    55i32 | 152i32 | 153i32 => {
                        break 's1;
                    }
                    _ => {
                        break 'f;
                    }
                }
            }
            if ((((D::call(c, fid::BtlChar_TestFxBit, &[((p_chr_1) as u64), ((58i32) as i64 as u64)])) as i32)) != 0) {
                l_on_2 = 1i32;
            } else {
                if ((((!((((D::call(c, fid::BtlChar_TestFxBit, &[((p_chr_1) as u64), ((59i32) as i64 as u64)])) as i32)) != 0)) as i32)) != 0) {
                    if ((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((14i32) as i64 as u64)])) as i32)) != 0) {
                        l_on_2 = ((!((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((17i32) as i64 as u64)])) as i32)) != 0)) as i32);
                    }
                }
            }
            if ((l_on_2) != 0) {
                if ((((((c.m.r32((p_chr_1).wrapping_add(4936)) as i32)) == (0i32)) as i32)) != 0) {
                    { let v_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); let a_ = fp; c.m.w32(a_, (v_) as u32); };
                    { let v_ = F32(0x3f800000); let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let _ = F::EftCharaFx_Start(c, fp); };
                }
                { let v_ = 1i32; let a_ = (p_chr_1).wrapping_add(4936); c.m.w32(a_, (v_) as u32); };
                break 'f;
            }
        }
        if ((((((c.m.r32((p_chr_1).wrapping_add(4936)) as i32)) == (1i32)) as i32)) != 0) {
            { let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); { let _ = F::EftCharaFx_Stop(c, a0_); } };
        }
        { let v_ = 0i32; let a_ = (p_chr_1).wrapping_add(4936); c.m.w32(a_, (v_) as u32); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlFx_UpdateCharaFx", &PROBE_N, &[], true); }
    r_
}

pub fn BtlFx_SpawnClashFlash(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFx_SpawnClashFlash", &PROBE_N, &[(p_chr_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(80) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 80);
    let mut l_shake_2: i32 = 0;
    let mut l_size_3: F32 = F32(0);
    let mut l_near_4: F32 = F32(0);
    let mut l_far_5: F32 = F32(0);
    let r_ = 'f: {
        l_shake_2 = 0i32;
        l_size_3 = F32(0xbf800000);
        if ((((D::call(c, fid::BtlChar_TestFxBit, &[((p_chr_1) as u64), ((44i32) as i64 as u64)])) as i32)) != 0) {
            l_size_3 = F32(0x40400000);
        }
        if ((((D::call(c, fid::BtlChar_TestFxBit, &[((p_chr_1) as u64), ((45i32) as i64 as u64)])) as i32)) != 0) {
            l_size_3 = F32(0x40c00000);
            l_shake_2 = 1i32;
        }
        if (((rt::fgt(l_size_3, F32(0x00000000)) as i32)) != 0) {
            { let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); F::BtlCharApi_GetNodePos(c, a0_, 3i32, fp.wrapping_add(48)) };
            { let a0_ = F::BtlOpp_GetObjId(c, p_chr_1); F::BtlCharApi_GetNodePos(c, a0_, 3i32, fp.wrapping_add(64)) };
            F::Vec4_Lerp(c, fp, fp.wrapping_add(48), fp.wrapping_add(64), F32(0x3f000000));
            { let v_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); let a_ = (fp).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
            { let v_ = 0i32; let a_ = (fp).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
            { let a1_ = (((D::call(c, fid::BtlChar_GetPos, &[((p_chr_1) as u64)])) as u32)).wrapping_add(128); F::Vec4_Copy(c, (fp).wrapping_add(16), a1_) };
            { let v_ = 6i32; let a_ = (fp).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
            { let _ = F::EftImpact_SpawnHitScaled(c, fp, l_size_3); };
            { let v_ = 13i32; let a_ = (fp).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
            { let _ = F::EftImpact_SpawnHitScaled(c, fp, l_size_3); };
            {
                l_near_4 = F32(0x43480000);
                l_far_5 = F32(0x44bb8000);
                if ((l_shake_2) != 0) {
                    F::BtlCharApi_ShakeCamsNear(c, fp, l_near_4, l_far_5, F32(0x40c00000), F32(0x3e4ccccc));
                }
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlFx_SpawnClashFlash", &PROBE_N, &[], true); }
    r_
}

pub fn BtlFx_TriggerObjReq1D(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFx_TriggerObjReq1D", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_obj_2: u32 = 0;
    let r_ = 'f: {
    l_obj_2 = ((D::call(c, fid::BtlChar_GetObj, &[((p_chr_1) as u64)])) as u32);
    if ((((D::call(c, fid::BtlChar_TestFxBit, &[((p_chr_1) as u64), ((29i32) as i64 as u64)])) as i32)) != 0) {
        { let _ = F::BtlObjFlash_Start(c, l_obj_2, 0i32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlFx_TriggerObjReq1D", &PROBE_N, &[], true); }
    r_
}

pub fn BtlFx_TriggerObjReq1E(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFx_TriggerObjReq1E", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_obj_2: u32 = 0;
    let r_ = 'f: {
    l_obj_2 = ((D::call(c, fid::BtlChar_GetObj, &[((p_chr_1) as u64)])) as u32);
    if ((((D::call(c, fid::BtlChar_TestFxBit, &[((p_chr_1) as u64), ((30i32) as i64 as u64)])) as i32)) != 0) {
        if ((((((c.m.r32((p_chr_1).wrapping_add(4940)) as i32)) == (0i32)) as i32)) != 0) {
            { let _ = F::BtlObjFlash_Start(c, l_obj_2, 1i32); };
        }
        { let v_ = rt::rem_i32(((c.m.r32((p_chr_1).wrapping_add(4940)) as i32)).wrapping_add(1i32), 4i32); let a_ = (p_chr_1).wrapping_add(4940); c.m.w32(a_, (v_) as u32); };
    } else {
        { let v_ = 0i32; let a_ = (p_chr_1).wrapping_add(4940); c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlFx_TriggerObjReq1E", &PROBE_N, &[], true); }
    r_
}

pub fn BtlFx_TriggerObjReq1F(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFx_TriggerObjReq1F", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_obj_2: u32 = 0;
    let r_ = 'f: {
    l_obj_2 = ((D::call(c, fid::BtlChar_GetObj, &[((p_chr_1) as u64)])) as u32);
    if ((((D::call(c, fid::BtlChar_TestFxBit, &[((p_chr_1) as u64), ((31i32) as i64 as u64)])) as i32)) != 0) {
        if ((((((c.m.r32((p_chr_1).wrapping_add(4944)) as i32)) == (0i32)) as i32)) != 0) {
            { let _ = F::BtlObjFlash_Start(c, l_obj_2, 2i32); };
        }
        { let v_ = rt::rem_i32(((c.m.r32((p_chr_1).wrapping_add(4944)) as i32)).wrapping_add(1i32), 4i32); let a_ = (p_chr_1).wrapping_add(4944); c.m.w32(a_, (v_) as u32); };
    } else {
        { let v_ = 0i32; let a_ = (p_chr_1).wrapping_add(4944); c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlFx_TriggerObjReq1F", &PROBE_N, &[], true); }
    r_
}

pub fn BtlFx_UpdateObjFlagReq3(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFx_UpdateObjFlagReq3", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_obj_2: u32 = 0;
    let r_ = 'f: {
    l_obj_2 = ((D::call(c, fid::BtlChar_GetObj, &[((p_chr_1) as u64)])) as u32);
    if ((((D::call(c, fid::BtlChar_IsFxBitNew, &[((p_chr_1) as u64), ((3i32) as i64 as u64)])) as i32)) != 0) {
        { let v_ = ((((c.m.r32((l_obj_2).wrapping_add(2624))) & (4278255615u32))) | (((524288i32) as u32))); let a_ = (l_obj_2).wrapping_add(2624); c.m.w32(a_, v_); };
    }
    if ((((D::call(c, fid::BtlChar_IsFxBitEnded, &[((p_chr_1) as u64), ((3i32) as i64 as u64)])) as i32)) != 0) {
        { let v_ = ((((c.m.r32((l_obj_2).wrapping_add(2624))) & (4278255615u32))) | (((65536i32) as u32))); let a_ = (l_obj_2).wrapping_add(2624); c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlFx_UpdateObjFlagReq3", &PROBE_N, &[], true); }
    r_
}

pub fn BtlFx_UpdateOccludedFlag(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFx_UpdateOccludedFlag", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_obj_2: u32 = 0;
    let mut l_on_3: i32 = 0;
    let r_ = 'f: {
    l_obj_2 = ((D::call(c, fid::BtlChar_GetObj, &[((p_chr_1) as u64)])) as u32);
    l_on_3 = 0i32;
    if ((((((c.m.r32((p_chr_1).wrapping_add(1172)) as i32)) != (0i32)) as i32)) != 0) {
        l_on_3 = ((!((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((301i32) as i64 as u64)])) as i32)) != 0)) as i32);
    }
    if ((l_on_3) != 0) {
        { let r_ = ((33554432i32) as u32); let a_ = (l_obj_2).wrapping_add(2624); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
    } else {
        { let r_ = (((!(33554432i32))) as u32); let a_ = (l_obj_2).wrapping_add(2624); let n_ = ((c.m.r32(a_)) & (r_)); c.m.w32(a_, n_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlFx_UpdateOccludedFlag", &PROBE_N, &[], true); }
    r_
}

pub fn BtlFx_UpdateObjCmdFlag137(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFx_UpdateObjCmdFlag137", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_obj_2: u32 = 0;
    let r_ = 'f: {
    l_obj_2 = ((D::call(c, fid::BtlChar_GetObj, &[((p_chr_1) as u64)])) as u32);
    if ((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((311i32) as i64 as u64)])) as i32)) != 0) {
        F::BtlObj_SetEyeFrame(c, l_obj_2, ((((1i32) as i64 as u64)) as u32));
    } else {
        if (((((F::BtlObj_GetEyeFrame(c, l_obj_2)) == (1i32)) as i32)) != 0) {
            F::BtlObj_SetEyeFrame(c, l_obj_2, ((((9i32) as i64 as u64)) as u32));
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlFx_UpdateObjCmdFlag137", &PROBE_N, &[], true); }
    r_
}

pub fn BtlFx_UpdateSubStateFlag96(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFx_UpdateSubStateFlag96", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_obj_2: u32 = 0;
    let mut l_state_3: i32 = 0;
    let r_ = 'f: {
    if ((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((150i32) as i64 as u64)])) as i32)) != 0) {
        l_obj_2 = ((D::call(c, fid::BtlChar_GetObj, &[((p_chr_1) as u64)])) as u32);
        l_state_3 = F::BtlObj_GetMouthMode(c, l_obj_2);
        if (((((((((l_state_3) == (0i32)) as i32)) != 0) || (((((l_state_3) == (4i32)) as i32)) != 0)) as i32)) != 0) {
            F::BtlObj_SetSubState(c, l_obj_2, 5i32, 0i32);
        }
    } else {
        l_obj_2 = ((D::call(c, fid::BtlChar_GetObj, &[((p_chr_1) as u64)])) as u32);
        if (((((F::BtlObj_GetMouthMode(c, l_obj_2)) == (5i32)) as i32)) != 0) {
            F::BtlObj_SetSubState(c, l_obj_2, 0i32, 0i32);
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlFx_UpdateSubStateFlag96", &PROBE_N, &[], true); }
    r_
}

pub fn BtlFx_UpdateObjFlag80(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFx_UpdateObjFlag80", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_obj_2: u32 = 0;
    let mut l_on_3: i32 = 0;
    let r_ = 'f: {
    l_obj_2 = ((D::call(c, fid::BtlChar_GetObj, &[((p_chr_1) as u64)])) as u32);
    l_on_3 = 0i32;
    if ((((((((F::BtlParam_GetFlags(c, p_chr_1)) as u64)) as i32)) & (2097152i32))) != 0) {
        l_on_3 = (((((D::call(c, fid::BtlMember_HasAbility, &[((p_chr_1) as u64), ((121i32) as i64 as u64)])) as i32)) != (0i32)) as i32);
    }
    if ((l_on_3) != 0) {
        if (((((((c.m.r64((l_obj_2).wrapping_add(2624))) & (((130i32) as u64)))) != (((130i32) as u64))) as i32)) != 0) {
            { let r_ = ((128i32) as u32); let a_ = (l_obj_2).wrapping_add(2624); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
        }
    } else {
        if (((((((c.m.r64((l_obj_2).wrapping_add(2624))) & (((130i32) as u64)))) == (((130i32) as u64))) as i32)) != 0) {
            { let r_ = (((!(128i32))) as u32); let a_ = (l_obj_2).wrapping_add(2624); let n_ = ((c.m.r32(a_)) & (r_)); c.m.w32(a_, n_); };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlFx_UpdateObjFlag80", &PROBE_N, &[], true); }
    r_
}

pub fn BtlFx_FireKiBlast(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlFx_FireKiBlast", &PROBE_N, &[(p_chr_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(112) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 112);
    let mut l_code_2: i32 = 0;
    let mut l_obj_3: u32 = 0;
    let mut l_react_4: i32 = 0;
    let mut l_count_5: i32 = 0;
    let mut l_i_6: i32 = 0;
    let mut l_speed_7: F32 = F32(0);
    let mut l_d_8: F32 = F32(0);
    let mut l_back_9: i32 = 0;
    let r_ = 'f: {
        l_obj_3 = ((D::call(c, fid::BtlChar_GetObj, &[((p_chr_1) as u64)])) as u32);
        if ((((D::call(c, fid::BtlAnim_TestAttr, &[((p_chr_1) as u64), ((((4i32) as u64)) as u64)])) as i32)) != 0) {
            l_react_4 = F::BtlKiBlast_GetUnk31(c, p_chr_1);
            l_code_2 = { let a0_ = ((((F::BtlObjAnim_GetEventArg(c, l_obj_3, ((4i32) as u64))) as u64)) as i32); ((D::call(c, fid::BtlObjAnim_MaskToNode, &[((a0_) as i64 as u64)])) as i32) };
            F::Vec4_Copy(c, fp.wrapping_add(80), (p_chr_1).wrapping_add(3536));
            l_count_5 = F::BtlKiBlast_GetHits(c, p_chr_1);
            if (((((l_count_5) <= (0i32)) as i32)) != 0) {
                l_count_5 = 1i32;
            }
            { let v_ = ((l_code_2) as u16); let a_ = (fp).wrapping_add(20); c.m.w16(a_, v_); };
            { let v_ = (((c.m.r32((p_chr_1).wrapping_add(12)) as i32)) as u16); let a_ = (fp).wrapping_add(18); c.m.w16(a_, v_); };
            { let v_ = (((c.m.r32((p_chr_1).wrapping_add(12)) as i32)) as u16); let a_ = (fp).wrapping_add(16); c.m.w16(a_, v_); };
            { let v_ = (((c.m.r32((l_obj_3).wrapping_add(2596)) as i32)) as u16); let a_ = (fp).wrapping_add(22); c.m.w16(a_, v_); };
            { let v_ = ((F::BtlKiBlast_GetType(c, p_chr_1)) as u8); let a_ = (fp).wrapping_add(26); c.m.w8(a_, v_); };
            { let v_ = ((F::BtlKiBlast_GetUnk3(c, p_chr_1)) as u8); let a_ = (fp).wrapping_add(27); c.m.w8(a_, v_); };
            { let v_ = ((F::BtlKiBlast_GetCurrentKind(c, p_chr_1)) as i16); let a_ = (fp).wrapping_add(24); c.m.w16(a_, (v_) as u16); };
            { let v_ = ((F::BtlKiBlast_GetFrames(c, p_chr_1)) as i16); let a_ = (fp).wrapping_add(28); c.m.w16(a_, (v_) as u16); };
            { let v_ = F::BtlKiBlast_GetSpeed(c, p_chr_1); let a_ = (fp).wrapping_add(32); c.m.w32(a_, (v_).0); };
            { let v_ = F::BtlKiBlast_GetTurnRate(c, p_chr_1); let a_ = (fp).wrapping_add(36); c.m.w32(a_, (v_).0); };
            { let v_ = F::BtlKiBlast_GetUnk28(c, p_chr_1); let a_ = (fp).wrapping_add(40); c.m.w32(a_, (v_).0); };
            { let v_ = F::BtlKiBlast_GetUnk2C(c, p_chr_1); let a_ = (fp).wrapping_add(44); c.m.w32(a_, (v_).0); };
            { let v_ = 1i32; let a_ = (fp).wrapping_add(52); c.m.w32(a_, (v_) as u32); };
            { let v_ = 1i32; let a_ = (fp).wrapping_add(56); c.m.w32(a_, (v_) as u32); };
            { let v_ = (((F::BtlKiBlast_GetFlags(c, p_chr_1)).wrapping_shr((5i32) as u32)) & (1i32)); let a_ = (fp).wrapping_add(60); c.m.w32(a_, (v_) as u32); };
            { let v_ = ((((D::call(c, fid::BtlObjAnim_QueryEvent, &[((l_obj_3) as u64), ((((4i32) as u64)) as u64), ((0i32) as i64 as u64), ((3i32) as i64 as u64)])) as i32)) as u8); let a_ = (fp).wrapping_add(65); c.m.w8(a_, v_); };
            's1: {
                let sv10: i32 = (((c.m.r16((fp).wrapping_add(24)) as i16)) as i32);
                match sv10 {
                    0i32 => {
                        { let v_ = 0i32; let a_ = (fp).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
                        break 's1;
                    }
                    1i32 => {
                        { let v_ = 1i32; let a_ = (fp).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
                        break 's1;
                    }
                    2i32 => {
                        { let v_ = 2i32; let a_ = (fp).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
                        break 's1;
                    }
                    3i32 => {
                        { let v_ = 3i32; let a_ = (fp).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
                        break 's1;
                    }
                    4i32 => {
                        { let v_ = 0i32; let a_ = (fp).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
                        break 's1;
                    }
                    5i32 => {
                        { let v_ = 1i32; let a_ = (fp).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
                        break 's1;
                    }
                    6i32 => {
                        { let v_ = 2i32; let a_ = (fp).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
                        break 's1;
                    }
                    7i32 => {
                        { let v_ = 3i32; let a_ = (fp).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
                        break 's1;
                    }
                    8i32 => {
                        { let v_ = 0i32; let a_ = (fp).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
                        break 's1;
                    }
                    9i32 => {
                        { let v_ = 1i32; let a_ = (fp).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
                        break 's1;
                    }
                    10i32 => {
                        { let v_ = 2i32; let a_ = (fp).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
                        break 's1;
                    }
                    11i32 => {
                        { let v_ = 3i32; let a_ = (fp).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
                        break 's1;
                    }
                    _ => {
                        { let v_ = 0i32; let a_ = (fp).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
                        break 's1;
                    }
                }
            }
            if ((F::BtlChar_TestMemberUnk70(c, p_chr_1)) != 0) {
                's2: {
                    let sv11: i32 = (((c.m.r16((fp).wrapping_add(24)) as i16)) as i32);
                    match sv11 {
                        0i32 | 4i32 | 8i32 => {
                            { let v_ = ((0i32) as u8); let a_ = (fp).wrapping_add(26); c.m.w8(a_, v_); };
                            break 's2;
                        }
                        _ => {
                            { let v_ = ((1i32) as u8); let a_ = (fp).wrapping_add(26); c.m.w8(a_, v_); };
                            break 's2;
                        }
                    }
                }
            }
            {
                l_i_6 = 0i32;
                'l3: loop {
                    if !(((((l_i_6) < (l_count_5)) as i32)) != 0) { break 'l3; }
                    'c4: {
                        F::Vec4_Copy(c, fp, fp.wrapping_add(80));
                        's5: {
                            let sv12: i32 = l_react_4;
                            match sv12 {
                                1i32 => {
                                    break 's5;
                                }
                                0i32 => {
                                    l_speed_7 = F::BtlAct_GetHeight(c, p_chr_1);
                                    if ((((!(((rt::flt(rt::fmul({ let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); F::BtlCharApi_GetHeight(c, a0_) }, F32(0x40400000)), l_speed_7) as i32)) != 0)) as i32)) != 0) {
                                        { let r_ = rt::fmul(rt::fsub(F::BtlChar_RandF(c), F32(0x3f000000)), F32(0x3ecccccc)); let a_ = fp; let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                                        { let r_ = rt::fmul(rt::fsub(F::BtlChar_RandF(c), F32(0x3f800000)), F32(0x3e4ccccc)); let a_ = (fp).wrapping_add(4); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                                    } else {
                                        { let r_ = rt::fmul(rt::fsub(F::BtlChar_RandF(c), F32(0x3f000000)), F32(0x3ecccccc)); let a_ = fp; let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                                        { let r_ = rt::fmul(rt::fsub(F::BtlChar_RandF(c), F32(0x3f000000)), F32(0x3ecccccc)); let a_ = (fp).wrapping_add(4); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                                    }
                                    { let r_ = rt::fmul(rt::fsub(F::BtlChar_RandF(c), F32(0x3f000000)), F32(0x3ecccccc)); let a_ = (fp).wrapping_add(8); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                                    break 's5;
                                }
                                2i32 | 3i32 | 4i32 | 5i32 | 6i32 => {
                                    if ((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((5i32) as i64 as u64)])) as i32)) != 0) {
                                        l_d_8 = rt::fmul(rt::fdiv(rt::fmul(F::BtlOpp_GetDistance(c, p_chr_1), F32(0x3f4ccccc)), F32(c.m.r32((fp).wrapping_add(32)))), F32(c.m.r32((fp).wrapping_add(36))));
                                        { let r_ = F32(0x3f000000); let n_ = rt::fmul(l_d_8, r_); l_d_8 = n_; };
                                        if (((rt::fgt(l_d_8, F32(0x3f400000)) as i32)) != 0) {
                                            l_d_8 = F32(0x3f400000);
                                        }
                                        if (((rt::flt(l_d_8, F32(0x3e4ccccc)) as i32)) != 0) {
                                            l_d_8 = F32(0x3e4ccccc);
                                        }
                                        if (((((l_react_4) == (2i32)) as i32)) != 0) {
                                            { let v_ = F32(c.m.r32((fp.wrapping_add(80)).wrapping_add(8))); let a_ = fp.wrapping_add(96); c.m.w32(a_, (v_).0); };
                                            { let v_ = F32(0x00000000); let a_ = (fp.wrapping_add(96)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                                            { let v_ = rt::fneg(F32(c.m.r32(fp.wrapping_add(80)))); let a_ = (fp.wrapping_add(96)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                                            F::Vec3_Normalize(c, fp.wrapping_add(96), fp.wrapping_add(96));
                                            F::Vec3_RotateAxis(c, fp, fp, fp.wrapping_add(96), l_d_8);
                                        } else {
                                            l_back_9 = 0i32;
                                            's6: {
                                                let sv13: i32 = l_react_4;
                                                match sv13 {
                                                    3i32 => {
                                                        l_back_9 = (((((D::call(c, fid::BtlChar_TestFlag, &[((p_chr_1) as u64), ((141i32) as i64 as u64)])) as i32)) != (0i32)) as i32);
                                                        F::BtlChar_ToggleHeldFlag(c, p_chr_1, ((((141i32) as i64 as u64)) as u32));
                                                        break 's6;
                                                    }
                                                    4i32 => {
                                                        l_back_9 = 1i32;
                                                        break 's6;
                                                    }
                                                    5i32 => {
                                                        break 's6;
                                                    }
                                                    6i32 => {
                                                        l_back_9 = (((F::BtlObj_GetNodeSide(c, l_code_2)) == (0i32)) as i32);
                                                        break 's6;
                                                    }
                                                    _ => {
                                                    }
                                                }
                                            }
                                            if ((l_back_9) != 0) {
                                                F::Vec3_RotateY(c, fp, fp, rt::fneg(l_d_8));
                                            } else {
                                                F::Vec3_RotateY(c, fp, fp, l_d_8);
                                            }
                                        }
                                    }
                                    break 's5;
                                }
                                _ => {
                                }
                            }
                        }
                        if (((((((c.m.r8((fp).wrapping_add(26))) as i32)) == (5i32)) as i32)) != 0) {
                            { let _ = F::EftDisc_SpawnFromNode(c, fp); };
                        } else {
                            if (((((((c.m.r8((fp).wrapping_add(26))) as i32)) == (4i32)) as i32)) != 0) {
                                { let _ = F::EftDisc_Throw(c, fp); };
                            } else {
                                if (((((((c.m.r8((fp).wrapping_add(26))) as i32)) == (2i32)) as i32)) != 0) {
                                    F::EftKiObj_Create(c, fp);
                                } else {
                                    if (((((((c.m.r8((fp).wrapping_add(26))) as i32)) == (3i32)) as i32)) != 0) {
                                        F::EftKiBomb_Fire(c, fp);
                                    } else {
                                        F::EftKiBlast_Fire(c, fp);
                                    }
                                }
                            }
                        }
                    }
                    { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; };
                }
            }
            's7: {
                let sv14: i32 = ((c.m.r8((fp).wrapping_add(26))) as i32);
                match sv14 {
                    2i32 | 3i32 => {
                        { let _ = D::call(c, fid::BtlCharSnd_PlayCommon, &[((p_chr_1) as u64), ((1i32) as i64 as u64)]); };
                        break 's7;
                    }
                    _ => {
                        { let a1_ = (F::BtlChar_FrameMod(c, 2i32)).wrapping_add(15i32); { let _ = D::call(c, fid::BtlCharSnd_PlayCommon, &[((p_chr_1) as u64), ((a1_) as i64 as u64)]); } };
                        break 's7;
                    }
                }
            }
            F::BtlChar_SetSmallVibration(c, p_chr_1, F32(0x3dcccccc));
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlFx_FireKiBlast", &PROBE_N, &[], true); }
    r_
}
