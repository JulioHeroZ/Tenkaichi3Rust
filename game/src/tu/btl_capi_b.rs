#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn BtlCharApi_IsUnkCACPaired(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_IsUnkCACPaired", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let mut l_obj_3: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        l_obj_3 = F::BtlChar_GetObj(c, l_chr_2);
        if (((((l_obj_3) != (((0i32) as u32))) as i32)) != 0) {
            break 'f ((((((c.m.r8((l_obj_3).wrapping_add(3245)) as i8)) as i32)) == ((!((((c.m.r8((l_obj_3).wrapping_add(3244)) as i8)) as i32))))) as i32);
        }
        break 'f 0i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_IsUnkCACPaired", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_GetAction(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetAction", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f F::BtlAct_GetCurrent(c, l_chr_2);
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetAction", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_GetBlastShots(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetBlastShots", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f (c.m.r32((l_chr_2).wrapping_add(3552)) as i32);
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetBlastShots", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_GetBlastRoom(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetBlastRoom", &PROBE_N, &[(p_objId_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_chr_2: u32 = 0;
    let mut l_left_3: i32 = 0;
    let mut l_room_4: i32 = 0;
    let r_ = 'f: {
        l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
        if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
            l_left_3 = (F::BtlParam_GetCount80(c, l_chr_2)).wrapping_sub((c.m.r32((l_chr_2).wrapping_add(3552)) as i32));
            { let _ = F::BtlMove_CanFireBlast(c, l_chr_2, 0i32, fp); };
            l_room_4 = (F::BtlParam_GetCount80(c, l_chr_2)).wrapping_sub((c.m.r32(fp) as i32));
            if (((((l_left_3) < (l_room_4)) as i32)) != 0) {
                l_room_4 = l_left_3;
            }
            break 'f l_room_4;
        }
        break 'f 0i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetBlastRoom", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_HasBlastLimit(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_HasBlastLimit", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f (((F::BtlParam_GetCount80(c, l_chr_2)) > (0i32)) as i32);
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_HasBlastLimit", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_GetBlastRoomB(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetBlastRoomB", &PROBE_N, &[(p_objId_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_chr_2: u32 = 0;
    let mut l_left_3: i32 = 0;
    let mut l_room_4: i32 = 0;
    let r_ = 'f: {
        l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
        if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
            l_left_3 = (F::BtlParam_GetCount80(c, l_chr_2)).wrapping_sub((c.m.r32((l_chr_2).wrapping_add(3552)) as i32));
            { let _ = F::BtlMove_CanFireBlast(c, l_chr_2, 1i32, fp); };
            l_room_4 = (F::BtlParam_GetUnk82(c, l_chr_2)).wrapping_sub((c.m.r32(fp) as i32));
            if (((((l_left_3) < (l_room_4)) as i32)) != 0) {
                l_room_4 = l_left_3;
            }
            break 'f l_room_4;
        }
        break 'f 0i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetBlastRoomB", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_TestFlag66(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_TestFlag66", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f F::BtlChar_TestFlag(c, l_chr_2, ((((102i32) as i64 as u64)) as u32));
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_TestFlag66", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_SkipBlastRec(c: &mut Ctx, mut p_objId_1: i32, mut p_rec_2: u32, mut p_mode_3: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_SkipBlastRec", &PROBE_N, &[(p_objId_1) as u32, (p_rec_2) as u32, (p_mode_3) as u32], false); }
    let r_ = 'f: {
    if (((((p_mode_3) < (2i32)) as i32)) != 0) {
        if ((((((c.m.r32((p_rec_2).wrapping_add(12)) as i32)) != (0i32)) as i32)) != 0) {
            break 'f 1i32;
        }
        if (((((c.m.r32((p_rec_2).wrapping_add(104))) == (((0i32) as u32))) as i32)) != 0) {
            break 'f 1i32;
        }
        if (((((p_objId_1) == ((c.m.r32(p_rec_2) as i32))) as i32)) != 0) {
            break 'f 1i32;
        }
    } else {
        if ((((((c.m.r32((p_rec_2).wrapping_add(12)) as i32)) != (1i32)) as i32)) != 0) {
            break 'f 1i32;
        }
        if (((((c.m.r32((p_rec_2).wrapping_add(100))) == (((0i32) as u32))) as i32)) != 0) {
            break 'f 1i32;
        }
        if (((((p_objId_1) == ({ let a_ = c.m.r32((p_rec_2).wrapping_add(100)); (c.m.r32(a_) as i32) })) as i32)) != 0) {
            break 'f 1i32;
        }
    }
    if ((((((c.m.r32((p_rec_2).wrapping_add(84)) as i32)) == (1i32)) as i32)) != 0) {
        break 'f 1i32;
    }
    break 'f ((((c.m.r32((p_rec_2).wrapping_add(384)) as i32)) == (1i32)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_SkipBlastRec", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_FindIncomingBlast(c: &mut Ctx, mut p_objId_1: i32, mut p_mode_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_FindIncomingBlast", &PROBE_N, &[(p_objId_1) as u32, (p_mode_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(96) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 96);
    let mut l_list_3: u32 = 0;
    let mut l_rec_4: u32 = 0;
    let mut l_i_5: i32 = 0;
    let mut l_dist_6: F32 = F32(0);
    let mut l_reach_7: F32 = F32(0);
    let r_ = 'f: {
        if (((((p_mode_2) < (2i32)) as i32)) != 0) {
            l_reach_7 = F32(0x40a00000);
        } else {
            l_reach_7 = F32(0x3ff33333);
        }
        F::BtlCharApi_GetPos(c, p_objId_1, fp);
        l_list_3 = F::EftHit_GetList(c);
        {
            l_i_5 = 0i32;
            'l1: loop {
                if !(((((l_i_5) < ((c.m.r32((l_list_3).wrapping_add(25600)) as i32))) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_rec_4 = (l_list_3).wrapping_add((((l_i_5) as u32)).wrapping_mul(400));
                    if (((((F::BtlCharApi_SkipBlastRec(c, p_objId_1, l_rec_4, p_mode_2)) == (1i32)) as i32)) != 0) {
                        break 'c2;
                    }
                    { let v_ = c.m.r64(((l_rec_4).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(8))); let a_ = (fp.wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                    { let v_ = c.m.r64(((l_rec_4).wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(8))); let a_ = (fp.wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                    F::Vec3_Sub(c, fp.wrapping_add(16), (l_rec_4).wrapping_add(32), fp);
                    F::Vec3_Normalize(c, fp.wrapping_add(16), fp.wrapping_add(16));
                    F::Vec3_Sub(c, fp.wrapping_add(80), (l_rec_4).wrapping_add(32), (l_rec_4).wrapping_add(48));
                    F::Vec3_Normalize(c, fp.wrapping_add(48), fp.wrapping_add(80));
                    if (((rt::flt(F32(0x00000000), F::Vec3_Dot(c, fp.wrapping_add(16), fp.wrapping_add(48))) as i32)) != 0) {
                        break 'c2;
                    }
                    F::Vec3_Sub(c, fp.wrapping_add(64), fp.wrapping_add(32), fp);
                    l_dist_6 = rt::fsub(rt::fsub(F::Vec3_Length(c, fp.wrapping_add(64)), F::BtlCharApi_GetRadius(c, p_objId_1)), F32(0x41200000));
                    if (((rt::flt(rt::fmul(F::Vec3_Length(c, fp.wrapping_add(80)), l_reach_7), l_dist_6) as i32)) != 0) {
                        break 'c2;
                    }
                    if (((((p_mode_2) != (1i32)) as i32)) != 0) {
                        break 'f l_i_5;
                    }
                    's3: {
                        let sv8: i32 = (({ let a_ = (c.m.r32((l_rec_4).wrapping_add(104))).wrapping_add(24); (c.m.r16(a_) as i16) }) as i32);
                        match sv8 {
                            0i32 | 4i32 | 8i32 => {
                                break 'f 0i32;
                            }
                            _ => {
                            }
                        }
                    }
                    break 'f (if ((((F::BtlKiBlast_GetFlagsOfHit(c, l_rec_4)) & (1i32))) != 0) { 2i32 } else { 1i32 });
                }
                { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
            }
        }
        break 'f (1i32).wrapping_neg();
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_FindIncomingBlast", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_IsBlastPassing(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_IsBlastPassing", &PROBE_N, &[(p_objId_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(64) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 64);
    let mut l_list_2: u32 = 0;
    let mut l_rec_3: u32 = 0;
    let mut l_i_4: i32 = 0;
    let r_ = 'f: {
        l_list_2 = F::EftHit_GetList(c);
        F::BtlCharApi_GetPos(c, p_objId_1, fp);
        {
            l_i_4 = 0i32;
            'l1: loop {
                if !(((((l_i_4) < ((c.m.r32((l_list_2).wrapping_add(25600)) as i32))) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_rec_3 = (l_list_2).wrapping_add((((l_i_4) as u32)).wrapping_mul(400));
                    if ((((((c.m.r32((l_rec_3).wrapping_add(12)) as i32)) == (0i32)) as i32)) != 0) {
                        if (((((p_objId_1) == ((({ let a_ = (c.m.r32((l_rec_3).wrapping_add(104))).wrapping_add(16); (c.m.r16(a_) as i16) }) as i32))) as i32)) != 0) {
                            break 'c2;
                        }
                    } else {
                        if (((((p_objId_1) == ({ let a_ = c.m.r32((l_rec_3).wrapping_add(100)); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                            break 'c2;
                        }
                    }
                    if ((((((c.m.r32((l_rec_3).wrapping_add(84)) as i32)) == (1i32)) as i32)) != 0) {
                        break 'c2;
                    }
                    F::Vec3_Sub(c, fp.wrapping_add(16), (l_rec_3).wrapping_add(32), fp);
                    F::Vec3_Normalize(c, fp.wrapping_add(16), fp.wrapping_add(16));
                    F::Vec3_Sub(c, fp.wrapping_add(48), (l_rec_3).wrapping_add(32), (l_rec_3).wrapping_add(48));
                    F::Vec3_Normalize(c, fp.wrapping_add(32), fp.wrapping_add(48));
                    if ((((!(((rt::flt(F32(0x00000000), F::Vec3_Dot(c, fp.wrapping_add(16), fp.wrapping_add(32))) as i32)) != 0)) as i32)) != 0) {
                        break 'f 1i32;
                    }
                }
                { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
            }
        }
        break 'f 0i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_IsBlastPassing", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_MarkIncomingBlast(c: &mut Ctx, mut p_objId_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_MarkIncomingBlast", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_list_2: u32 = 0;
    let mut l_i_3: i32 = 0;
    let mut l_rec_4: u32 = 0;
    let r_ = 'f: {
    l_list_2 = F::EftHit_GetList(c);
    l_i_3 = F::BtlCharApi_FindIncomingBlast(c, p_objId_1, 0i32);
    if (((((l_i_3) >= (0i32)) as i32)) != 0) {
        l_rec_4 = l_list_2;
        { let r_ = l_i_3; let n_ = (l_rec_4).wrapping_add((((r_) as u32)).wrapping_mul(400)); l_rec_4 = n_; };
        { let v_ = 1i32; let a_ = (l_rec_4).wrapping_add(384); c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_MarkIncomingBlast", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_GetTechniqueProgress(c: &mut Ctx, mut p_objId_1: i32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetTechniqueProgress", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let mut l_frame_3: i32 = 0;
    let mut l_start_4: F32 = F32(0);
    let mut l_len_5: F32 = F32(0);
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) == (((0i32) as u32))) as i32)) != 0) {
        break 'f F32(0x00000000);
    }
    if ((((((c.m.r32((l_chr_2).wrapping_add(4756)) as i32)) >= (0i32)) as i32)) != 0) {
        break 'f rt::i2f((c.m.r32((l_chr_2).wrapping_add(4756)) as i32));
    }
    l_frame_3 = F::BtlCharApi_ObjQuery24D610(c, p_objId_1, 1i32, 1i32);
    if (((((l_frame_3) < (0i32)) as i32)) != 0) {
        break 'f F32(0xbf800000);
    }
    l_start_4 = F::BtlCharApi_ObjGetUnkC78(c, p_objId_1);
    l_len_5 = F::BtlCharApi_ObjGetUnkC80(c, p_objId_1);
    if (((rt::flt(F32(0x3c23d70a), l_len_5) as i32)) != 0) {
        break 'f rt::fdiv(rt::fsub(rt::i2f(l_frame_3), l_start_4), l_len_5);
    }
    break 'f F32(0xbf800000);
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetTechniqueProgress", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_GetSkillTable(c: &mut Ctx, mut p_objId_1: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetSkillTable", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let mut l_obj_3: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        l_obj_3 = F::BtlChar_GetObj(c, l_chr_2);
        if (((((l_obj_3) != (((0i32) as u32))) as i32)) != 0) {
            break 'f c.m.r32((l_obj_3).wrapping_add(2348));
        }
        break 'f ((0i32) as u32);
    }
    break 'f ((0i32) as u32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetSkillTable", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_GetMoveTable(c: &mut Ctx, mut p_objId_1: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetMoveTable", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let mut l_obj_3: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        l_obj_3 = F::BtlChar_GetObj(c, l_chr_2);
        if (((((l_obj_3) != (((0i32) as u32))) as i32)) != 0) {
            break 'f c.m.r32((l_obj_3).wrapping_add(2352));
        }
        break 'f ((0i32) as u32);
    }
    break 'f ((0i32) as u32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetMoveTable", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_GetAttackAttr(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetAttackAttr", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        if (((((((((F::BtlAtk_GetId(c, l_chr_2)) as i64 as u64)) as u32)) < (((163i32) as u32))) as i32)) != 0) {
            break 'f ((((F::BtlAtk_GetFlags(c, l_chr_2)) as u64)) as i32);
        }
        break 'f 0i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetAttackAttr", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_GetHp(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetHp", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f { let a_ = F::BtlMember_GetActiveGauge(c, l_chr_2); (c.m.r32(a_) as i32) };
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetHp", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_GetHpMax(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetHpMax", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f { let a_ = (F::BtlMember_GetActiveGauge(c, l_chr_2)).wrapping_add(4); (c.m.r32(a_) as i32) };
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetHpMax", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_GetKi(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetKi", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f { let a_ = (F::BtlMember_GetActiveGauge(c, l_chr_2)).wrapping_add(12); (c.m.r32(a_) as i32) };
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetKi", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_GetBlast(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetBlast", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f { let a_ = (F::BtlMember_GetActiveGauge(c, l_chr_2)).wrapping_add(20); (c.m.r32(a_) as i32) };
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetBlast", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_GetBlastMax(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetBlastMax", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f { let a_ = (F::BtlMember_GetActiveGauge(c, l_chr_2)).wrapping_add(24); (c.m.r32(a_) as i32) };
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetBlastMax", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_GetMaxPower(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetMaxPower", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f { let a_ = (F::BtlMember_GetActiveGauge(c, l_chr_2)).wrapping_add(28); (c.m.r32(a_) as i32) };
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetMaxPower", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_GetSwitchGauge(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetSwitchGauge", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f (c.m.r32((l_chr_2).wrapping_add(2460)) as i32);
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetSwitchGauge", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_IsBlockedByOpponent(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_IsBlockedByOpponent", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f F::BtlMove_IsBlockedByOpponent(c, l_chr_2);
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_IsBlockedByOpponent", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_HasParamBit80(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_HasParamBit80", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f (((F::BtlParam_GetUnk0(c, l_chr_2)).wrapping_shr((7i32) as u32)) & (1i32));
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_HasParamBit80", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_CanTransform(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_CanTransform", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let mut l_slot_3: i32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        l_slot_3 = F::BtlParam_GetUnkAC(c, l_chr_2);
        if (((((l_slot_3) < (0i32)) as i32)) != 0) {
            break 'f 0i32;
        }
        break 'f (((F::BtlAct_CanTransform(c, l_chr_2, ((l_slot_3) as u32), 1i32, 1i32)) != (0i32)) as i32);
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_CanTransform", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_CanFuse(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_CanFuse", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let mut l_i_3: i32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    {
        l_i_3 = 0i32;
        'l1: loop {
            if !(((((l_i_3) < (3i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((F::BtlAct_CanFuse(c, l_chr_2, l_i_3, 1i32, 1i32, ((0i32) as u32))) != 0) {
                    break 'f 1i32;
                }
            }
            { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
        }
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_CanFuse", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_CanSwitch(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_CanSwitch", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f (((F::BtlAct_CanSwitch(c, l_chr_2, 1i32, 1i32)) != (0i32)) as i32);
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_CanSwitch", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_GetCpuLevel(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetCpuLevel", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f { let a_ = (F::BtlMember_GetActive(c, l_chr_2)).wrapping_add(56); (c.m.r32(a_) as i32) };
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetCpuLevel", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_GetAiType(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetAiType", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f { let a_ = (F::BtlMember_GetActive(c, l_chr_2)).wrapping_add(60); (c.m.r32(a_) as i32) };
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetAiType", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_IsUnk1070Set(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_IsUnk1070Set", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f ((((c.m.r32((l_chr_2).wrapping_add(4208)) as i32)) != ((30i32).wrapping_neg())) as i32);
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_IsUnk1070Set", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_GetTransformCost(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetTransformCost", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let mut l_slot_3: i32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        if (((((F::BtlParam_CountSlots(c, l_chr_2)) != (0i32)) as i32)) != 0) {
            l_slot_3 = F::BtlParam_GetUnkAC(c, l_chr_2);
            if (((((l_slot_3) == ((1i32).wrapping_neg())) as i32)) != 0) {
                break 'f 0i32;
            }
            break 'f F::BtlParam_GetSlotCost(c, l_chr_2, l_slot_3);
        }
        break 'f 0i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetTransformCost", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_PickFusionSlot(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_PickFusionSlot", &PROBE_N, &[(p_objId_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_chr_2: u32 = 0;
    let mut l_n_3: i32 = 0;
    let mut l_blast_4: i32 = 0;
    let mut l_r_5: i32 = 0;
    let mut l_i_6: i32 = 0;
    let mut l_cost_7: i32 = 0;
    let r_ = 'f: {
        l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
        l_n_3 = 0i32;
        l_blast_4 = F::BtlSide_GetBlast(c, p_objId_1);
        l_r_5 = ((((F::Rand_Range(c, ((((100i32) as i64 as u64)) as u32))) as u64)) as i32);
        if (((((l_chr_2) == (((0i32) as u32))) as i32)) != 0) {
            break 'f 0i32;
        }
        {
            l_i_6 = 0i32;
            'l1: loop {
                if !(((((l_i_6) < (3i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_cost_7 = F::BtlParam_GetCostAE(c, l_chr_2, l_i_6);
                    if (((((((((l_cost_7) != (0i32)) as i32)) != 0) && ((((!(((((l_blast_4) < (l_cost_7)) as i32)) != 0)) as i32)) != 0)) as i32)) != 0) {
                        { let v_ = l_i_6; let a_ = (fp).wrapping_add((((l_n_3) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                        { let o_ = l_n_3; let n_ = (o_).wrapping_add(1); l_n_3 = n_; };
                    }
                }
                { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; };
            }
        }
        's3: {
            let sv8: i32 = l_n_3;
            match sv8 {
                1i32 => {
                    break 'f (c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32);
                }
                0i32 => {
                    break 'f (1i32).wrapping_neg();
                }
                2i32 => {
                    break 'f { let a_ = (fp).wrapping_add((((((((F::Rand_Range(c, ((((2i32) as i64 as u64)) as u32))) as u64)) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
                }
                3i32 => {
                    if (((((l_r_5) < (35i32)) as i32)) != 0) {
                        break 'f 0i32;
                    }
                    break 'f (if (((((l_r_5) < (55i32)) as i32)) != 0) { 1i32 } else { 2i32 });
                }
                _ => {
                }
            }
        }
        break 'f (1i32).wrapping_neg();
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_PickFusionSlot", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_PickTransformSlot(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_PickTransformSlot", &PROBE_N, &[(p_objId_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_n_2: i32 = 0;
    let mut l_chr_3: u32 = 0;
    let mut l_has3_4: i32 = 0;
    let mut l_blast_5: i32 = 0;
    let mut l_r_6: i32 = 0;
    let mut l_i_7: i32 = 0;
    let mut l_cost_8: i32 = 0;
    let r_ = 'f: {
        l_n_2 = 0i32;
        l_chr_3 = F::BtlChar_FindByObjId(c, p_objId_1);
        l_has3_4 = 0i32;
        l_blast_5 = F::BtlSide_GetBlast(c, p_objId_1);
        l_r_6 = ((((F::Rand_Range(c, ((((100i32) as i64 as u64)) as u32))) as u64)) as i32);
        if (((((l_chr_3) == (((0i32) as u32))) as i32)) != 0) {
            break 'f 0i32;
        }
        {
            l_i_7 = 0i32;
            'l1: loop {
                if !(((((l_i_7) < (4i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_cost_8 = F::BtlParam_GetSlotCost(c, l_chr_3, l_i_7);
                    if (((((((((l_cost_8) == (0i32)) as i32)) != 0) && (((((l_i_7) != (3i32)) as i32)) != 0)) as i32)) != 0) {
                        break 'c2;
                    }
                    if (((((l_blast_5) < (l_cost_8)) as i32)) != 0) {
                        break 'c2;
                    }
                    { let v_ = l_i_7; let a_ = (fp).wrapping_add((((l_n_2) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    if (((((l_i_7) == (3i32)) as i32)) != 0) {
                        l_has3_4 = 1i32;
                    }
                    { let o_ = l_n_2; let n_ = (o_).wrapping_add(1); l_n_2 = n_; };
                }
                { let o_ = l_i_7; let n_ = (o_).wrapping_add(1); l_i_7 = n_; };
            }
        }
        's3: {
            let sv9: i32 = l_n_2;
            match sv9 {
                0i32 => {
                    break 's3;
                }
                1i32 => {
                    break 'f (c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32);
                }
                2i32 => {
                    if ((l_has3_4) != 0) {
                        if (((((l_r_6) < (2i32)) as i32)) != 0) {
                            break 'f 3i32;
                        }
                        break 'f (c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32);
                    }
                    break 'f { let a_ = (fp).wrapping_add((((((((F::Rand_Range(c, ((((2i32) as i64 as u64)) as u32))) as u64)) as i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
                }
                3i32 => {
                    if ((l_has3_4) != 0) {
                        if (((((l_r_6) < (2i32)) as i32)) != 0) {
                            break 'f 3i32;
                        }
                        if (((((l_r_6) < (60i32)) as i32)) != 0) {
                            break 'f (c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32);
                        }
                        break 'f (c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32);
                    }
                    if (((((l_r_6) < (60i32)) as i32)) != 0) {
                        break 'f (c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32);
                    }
                    if (((((l_r_6) < (90i32)) as i32)) != 0) {
                        break 'f (c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32);
                    }
                    break 'f (c.m.r32((fp).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32);
                }
                4i32 => {
                    if ((l_has3_4) != 0) {
                        if (((((l_r_6) < (2i32)) as i32)) != 0) {
                            break 'f 3i32;
                        }
                        if (((((l_r_6) < (60i32)) as i32)) != 0) {
                            break 'f (c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32);
                        }
                        if (((((l_r_6) < (90i32)) as i32)) != 0) {
                            break 'f (c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32);
                        }
                        break 'f (c.m.r32((fp).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32);
                    }
                    break 's3;
                }
                _ => {
                }
            }
        }
        break 'f (1i32).wrapping_neg();
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_PickTransformSlot", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_IsChangingForm(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_IsChangingForm", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let mut l_action_3: i32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        l_action_3 = F::BtlAct_GetCurrent(c, l_chr_2);
        's1: {
            let sv4: i32 = l_action_3;
            match sv4 {
                236i32 | 237i32 | 238i32 | 239i32 | 240i32 | 241i32 | 242i32 => {
                    break 'f 1i32;
                }
                _ => {
                }
            }
        }
        break 'f 0i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_IsChangingForm", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_GetParamUnk14(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetParamUnk14", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f { let a_ = ({ let a_ = (F::BtlChar_GetObj(c, l_chr_2)).wrapping_add(2332); c.m.r32(a_) }).wrapping_add(20); (c.m.r32(a_) as i32) };
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetParamUnk14", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_GetParamFlags(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetParamFlags", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f { let a_ = ({ let a_ = (F::BtlChar_GetObj(c, l_chr_2)).wrapping_add(2332); c.m.r32(a_) }).wrapping_add(16); (c.m.r32(a_) as i32) };
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetParamFlags", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_GetParamUnk18(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetParamUnk18", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f { let a_ = ({ let a_ = (F::BtlChar_GetObj(c, l_chr_2)).wrapping_add(2332); c.m.r32(a_) }).wrapping_add(24); (c.m.r32(a_) as i32) };
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetParamUnk18", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_GetMemberCount(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetMemberCount", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f (c.m.r32((l_chr_2).wrapping_add(2456)) as i32);
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetMemberCount", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_GetActiveMember(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetActiveMember", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f F::BtlMember_GetActiveIndex(c, l_chr_2);
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetActiveMember", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_GetSwitchTarget(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetSwitchTarget", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f F::BtlMember_GetSwitchTarget(c, l_chr_2);
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetSwitchTarget", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_GetMemberHpPercent(c: &mut Ctx, mut p_objId_1: i32, mut p_member_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetMemberHpPercent", &PROBE_N, &[(p_objId_1) as u32, (p_member_2) as u32], false); }
    let mut l_chr_3: u32 = 0;
    let mut l_g_4: u32 = 0;
    let mut l_pct_5: i32 = 0;
    let r_ = 'f: {
    l_chr_3 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_3) != (((0i32) as u32))) as i32)) != 0) {
        l_g_4 = F::BtlMember_GetGauge(c, l_chr_3, p_member_2);
        if ((((((c.m.r32((l_g_4).wrapping_add(4)) as i32)) > (0i32)) as i32)) != 0) {
            l_pct_5 = rt::f2i(rt::fmul(rt::fdiv(rt::i2f((c.m.r32(l_g_4) as i32)), rt::i2f((c.m.r32((l_g_4).wrapping_add(4)) as i32))), F32(0x42c80000)));
            if (((((((((l_pct_5) <= (0i32)) as i32)) != 0) && ((((((c.m.r32(l_g_4) as i32)) > (0i32)) as i32)) != 0)) as i32)) != 0) {
                l_pct_5 = 1i32;
            }
            break 'f l_pct_5;
        }
        break 'f 0i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetMemberHpPercent", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_GetMemberKiPercent(c: &mut Ctx, mut p_objId_1: i32, mut p_member_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetMemberKiPercent", &PROBE_N, &[(p_objId_1) as u32, (p_member_2) as u32], false); }
    let mut l_chr_3: u32 = 0;
    let mut l_g_4: u32 = 0;
    let mut l_pct_5: i32 = 0;
    let r_ = 'f: {
    l_chr_3 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_3) != (((0i32) as u32))) as i32)) != 0) {
        l_g_4 = F::BtlMember_GetGauge(c, l_chr_3, p_member_2);
        if ((((((c.m.r32((l_g_4).wrapping_add(16)) as i32)) > (0i32)) as i32)) != 0) {
            l_pct_5 = rt::f2i(rt::fmul(rt::fdiv(rt::i2f((c.m.r32((l_g_4).wrapping_add(12)) as i32)), rt::i2f((c.m.r32((l_g_4).wrapping_add(16)) as i32))), F32(0x42c80000)));
            if (((((((((l_pct_5) <= (0i32)) as i32)) != 0) && ((((((c.m.r32((l_g_4).wrapping_add(12)) as i32)) > (0i32)) as i32)) != 0)) as i32)) != 0) {
                l_pct_5 = 1i32;
            }
            break 'f l_pct_5;
        }
        break 'f 0i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetMemberKiPercent", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_GetOppSkillKind(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetOppSkillKind", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let mut l_opp_3: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) == (((0i32) as u32))) as i32)) != 0) {
        break 'f (1i32).wrapping_neg();
    }
    l_opp_3 = { let a0_ = F::BtlOpp_GetPlayer(c, l_chr_2); F::BtlChar_Get(c, a0_) };
    if ((((!(({ let a0_ = F::BtlAct_GetCurrent(c, l_opp_3); F::BtlAct_IsTechniqueId(c, a0_) }) != 0)) as i32)) != 0) {
        break 'f (1i32).wrapping_neg();
    }
    break 'f { let a1_ = F::BtlAct_GetCurrentClass(c, l_opp_3); F::BtlSuper_GetUnk165(c, l_opp_3, a1_) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetOppSkillKind", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_IsOppSkillFlag4(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_IsOppSkillFlag4", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let mut l_opp_3: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        l_opp_3 = { let a0_ = F::BtlOpp_GetPlayer(c, l_chr_2); F::BtlChar_Get(c, a0_) };
        if (({ let a0_ = F::BtlAct_GetCurrent(c, l_opp_3); F::BtlAct_IsTechniqueId(c, a0_) }) != 0) {
            break 'f ((({ let a1_ = F::BtlAct_GetCurrentClass(c, l_opp_3); ((((F::BtlSuper_GetFlags(c, l_opp_3, a1_)) as u64)) as i32) }).wrapping_shr((2i32) as u32)) & (1i32));
        }
        break 'f 0i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_IsOppSkillFlag4", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_TestOppSkillFlags(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_TestOppSkillFlags", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let mut l_opp_3: u32 = 0;
    let mut l_cls_4: i32 = 0;
    let mut l_flags_5: i32 = 0;
    let mut l_unk_6: i32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        l_opp_3 = { let a0_ = F::BtlOpp_GetPlayer(c, l_chr_2); F::BtlChar_Get(c, a0_) };
        if (({ let a0_ = F::BtlAct_GetCurrent(c, l_opp_3); F::BtlAct_IsTechniqueId(c, a0_) }) != 0) {
            l_cls_4 = F::BtlAct_GetCurrentClass(c, l_opp_3);
            l_flags_5 = ((((F::BtlSuper_GetFlags(c, l_opp_3, l_cls_4)) as u64)) as i32);
            l_unk_6 = F::BtlSuper_GetType(c, l_opp_3, l_cls_4);
            if ((((l_flags_5) & (1i32))) != 0) {
                break 'f 1i32;
            }
            if ((((((((l_flags_5) & (2i32))) != 0) && (((rt::flt(F32(0x3f666666), F32(c.m.r32((l_chr_2).wrapping_add(3652)))) as i32)) != 0)) as i32)) != 0) {
                break 'f 1i32;
            }
            break 'f (((l_unk_6) == (1i32)) as i32);
        }
        break 'f 0i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_TestOppSkillFlags", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_GetOppSkillClass(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetOppSkillClass", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let mut l_opp_3: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) == (((0i32) as u32))) as i32)) != 0) {
        break 'f (1i32).wrapping_neg();
    }
    l_opp_3 = { let a0_ = F::BtlOpp_GetPlayer(c, l_chr_2); F::BtlChar_Get(c, a0_) };
    if ((((!(({ let a0_ = F::BtlAct_GetCurrent(c, l_opp_3); F::BtlAct_IsTechniqueId(c, a0_) }) != 0)) as i32)) != 0) {
        break 'f (1i32).wrapping_neg();
    }
    break 'f F::BtlAct_GetCurrentClass(c, l_opp_3);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetOppSkillClass", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_GetOppMoveKind(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetOppMoveKind", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let mut l_opp_3: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) == (((0i32) as u32))) as i32)) != 0) {
        break 'f (1i32).wrapping_neg();
    }
    l_opp_3 = { let a0_ = F::BtlOpp_GetPlayer(c, l_chr_2); F::BtlChar_Get(c, a0_) };
    if ((((((((F::BtlAct_GetCurrent(c, l_opp_3)).wrapping_sub(253i32)) as u32)) >= (((6i32) as u32))) as i32)) != 0) {
        break 'f (1i32).wrapping_neg();
    }
    break 'f { let a1_ = F::BtlAct_GetCurrentClass(c, l_opp_3); F::BtlSkill_GetUnk9E(c, l_opp_3, a1_) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetOppMoveKind", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_GetClashCountB(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetClashCountB", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f (c.m.r32((l_chr_2).wrapping_add(3664)) as i32);
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetClashCountB", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_GetClashCountA(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetClashCountA", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f (c.m.r32((l_chr_2).wrapping_add(3660)) as i32);
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetClashCountA", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_IsMoveSlotActive(c: &mut Ctx, mut p_objId_1: i32, mut p_slot_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_IsMoveSlotActive", &PROBE_N, &[(p_objId_1) as u32, (p_slot_2) as u32], false); }
    let mut l_chr_3: u32 = 0;
    let mut l_n_4: i32 = 0;
    let r_ = 'f: {
    l_chr_3 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_3) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if (((((p_slot_2) >= (((2i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    l_n_4 = (((p_slot_2) != (((0i32) as u32))) as i32);
    if ((((((((c.m.r32((l_chr_3).wrapping_add(3596)) as i32)) as u32)) == (p_slot_2)) as i32)) != 0) {
        if ((((((c.m.r32((l_chr_3).wrapping_add(3584)) as i32)) > (0i32)) as i32)) != 0) {
            break 'f 1i32;
        }
        if ((((((c.m.r32((l_chr_3).wrapping_add(3588)) as i32)) > (0i32)) as i32)) != 0) {
            break 'f 1i32;
        }
    }
    break 'f (c.m.r32(((l_chr_3).wrapping_add(3640)).wrapping_add((((l_n_4) as u32)).wrapping_mul(4))) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_IsMoveSlotActive", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_IsMoveFlag100(c: &mut Ctx, mut p_objId_1: i32, mut p_slot_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_IsMoveFlag100", &PROBE_N, &[(p_objId_1) as u32, (p_slot_2) as u32], false); }
    let mut l_chr_3: u32 = 0;
    let r_ = 'f: {
    l_chr_3 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_3) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if (((((p_slot_2) >= (((2i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    break 'f (((((((F::BtlSkill_GetFlags(c, l_chr_3, ((p_slot_2) as i32))) as u64)) as i32)).wrapping_shr((8i32) as u32)) & (1i32));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_IsMoveFlag100", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_GetStunTimer(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetStunTimer", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f (c.m.r32((l_chr_2).wrapping_add(4064)) as i32);
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetStunTimer", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_GetPromptButtons(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetPromptButtons", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let mut l_p_3: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((((c.m.r32((l_chr_2).wrapping_add(5524)) as i32)) < (2i32)) as i32)) != 0) {
        break 'f 0i32;
    }
    l_p_3 = ({ let a_ = (c.m.r32(sym::gBtlChars)).wrapping_add(52); c.m.r32(a_) }).wrapping_add(((({ let a1_ = (c.m.r32((l_chr_2).wrapping_add(5524)) as i32); F::BtlSuper_GetPromptRowIndex(c, l_chr_2, a1_) }) as u32)).wrapping_mul(20));
    if ((F::BtlChar_TestFlag(c, l_chr_2, ((((162i32) as i64 as u64)) as u32))) != 0) {
        break 'f 0i32;
    }
    if ((F::BtlChar_TestFlag(c, l_chr_2, ((((163i32) as i64 as u64)) as u32))) != 0) {
        break 'f 4i32;
    }
    's1: {
        let sv4: i32 = (((c.m.r8((l_p_3).wrapping_add(2)) as i8)) as i32);
        match sv4 {
            0i32 => {
                break 'f 24i32;
            }
            1i32 => {
                break 'f 40i32;
            }
            2i32 => {
                break 'f 72i32;
            }
            3i32 => {
                break 'f 136i32;
            }
            _ => {
            }
        }
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetPromptButtons", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_GetUnk1290(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetUnk1290", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        if (((((F::Battle_GetMode(c)) != (1i32)) as i32)) != 0) {
            break 'f 0i32;
        }
        break 'f (c.m.r32((l_chr_2).wrapping_add(4752)) as i32);
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetUnk1290", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_IsUnk106CLow(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_IsUnk106CLow", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f ((((c.m.r32((l_chr_2).wrapping_add(4204)) as i32)) < ((29i32).wrapping_neg())) as i32);
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_IsUnk106CLow", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_GetArmorBreakLevel(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetArmorBreakLevel", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let mut l_opp_3: u32 = 0;
    let mut l_level_4: i32 = 0;
    let mut l_i_5: i32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 4i32;
    }
    l_opp_3 = { let a0_ = F::BtlOpp_GetPlayer(c, l_chr_2); F::BtlChar_Get(c, a0_) };
    l_level_4 = 0i32;
    if ((((((((F::BtlParam_GetFlags(c, l_opp_3)) as u64)) as i32)) & (4i32))) != 0) {
        l_level_4 = 1i32;
    }
    if ((((((((F::BtlParam_GetFlags(c, l_opp_3)) as u64)) as i32)) & (8i32))) != 0) {
        { let o_ = l_level_4; let n_ = (o_).wrapping_sub(1); l_level_4 = n_; };
    }
    if ((((((((F::BtlParam_GetFlags(c, l_chr_2)) as u64)) as i32)) & (4i32))) != 0) {
        { let o_ = l_level_4; let n_ = (o_).wrapping_sub(1); l_level_4 = n_; };
    }
    if ((((((((F::BtlParam_GetFlags(c, l_chr_2)) as u64)) as i32)) & (8i32))) != 0) {
        { let o_ = l_level_4; let n_ = (o_).wrapping_add(1); l_level_4 = n_; };
    }
    if ((F::BtlAct_TestPoweredSkill(c, l_opp_3, ((64i32) as u32))) != 0) {
        { let o_ = l_level_4; let n_ = (o_).wrapping_add(1); l_level_4 = n_; };
    }
    { let r_ = ((((c.m.r32((l_opp_3).wrapping_add(3604)) as i32)) > (0i32)) as i32); let n_ = (l_level_4).wrapping_add(r_); l_level_4 = n_; };
    if ((F::BtlMember_HasAbility(c, l_opp_3, 70i32)) != 0) {
        { let o_ = l_level_4; let n_ = (o_).wrapping_add(1); l_level_4 = n_; };
    }
    if ((F::BtlAct_TestPoweredSkill(c, l_chr_2, ((256i32) as u32))) != 0) {
        { let o_ = l_level_4; let n_ = (o_).wrapping_sub(1); l_level_4 = n_; };
    }
    if ((F::BtlMember_HasAbility(c, l_chr_2, 71i32)) != 0) {
        { let o_ = l_level_4; let n_ = (o_).wrapping_sub(1); l_level_4 = n_; };
    } else {
        if ((F::BtlMember_HasAbility(c, l_chr_2, 112i32)) != 0) {
            if ((F::BtlChar_TestFlag(c, l_chr_2, ((((6i32) as i64 as u64)) as u32))) != 0) {
                { let o_ = l_level_4; let n_ = (o_).wrapping_sub(1); l_level_4 = n_; };
            }
        }
    }
    {
        l_i_5 = 0i32;
        'l1: loop {
            if !(((((l_i_5) < (5i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((((((l_level_4).wrapping_sub(F::BtlAtk_GetUnk2COf(c, l_chr_2, ((((l_i_5) as i64 as u64)) as u32)))) <= (0i32)) as i32)) != 0) {
                    if (((((l_i_5) < (2i32)) as i32)) != 0) {
                        break 'f 0i32;
                    }
                    break 'f (l_i_5).wrapping_sub(1i32);
                }
            }
            { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
        }
    }
    break 'f 4i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetArmorBreakLevel", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_GetParamByte2(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetParamByte2", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f (({ let a_ = ({ let a_ = (F::BtlChar_GetObj(c, l_chr_2)).wrapping_add(2332); c.m.r32(a_) }).wrapping_add(2); (c.m.r8(a_) as i8) }) as i32);
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetParamByte2", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_GetUnkE44B(c: &mut Ctx, mut p_objId_1: i32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetUnkE44B", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f F32(c.m.r32((l_chr_2).wrapping_add(3652)));
    }
    break 'f F32(0x00000000);
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetUnkE44B", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_GetUnkD74Diff(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetUnkD74Diff", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        if ((((((c.m.r32((l_chr_2).wrapping_add(3444)) as i32)) == (0i32)) as i32)) != 0) {
            break 'f (1i32).wrapping_neg();
        }
        break 'f ((c.m.r32((l_chr_2).wrapping_add(3444)) as i32)).wrapping_sub((c.m.r32((l_chr_2).wrapping_add(3440)) as i32));
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetUnkD74Diff", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_IsAnimFlag2800(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_IsAnimFlag2800", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let mut l_flags_3: i32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        l_flags_3 = { let a0_ = F::BtlAnim_GetId(c, l_chr_2); ((((F::BtlAnim_GetFlags(c, ((((a0_) as i64 as u64)) as u32))) as u64)) as i32) };
        if ((((!((((l_flags_3) & (10240i32))) != 0)) as i32)) != 0) {
            break 'f 0i32;
        }
        if ((((l_flags_3) & (32768i32))) != 0) {
            break 'f 0i32;
        }
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_IsAnimFlag2800", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_TestPoseBit80(c: &mut Ctx, mut p_objId_1: i32, mut p_always_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_TestPoseBit80", &PROBE_N, &[(p_objId_1) as u32, (p_always_2) as u32], false); }
    let mut l_chr_3: u32 = 0;
    let r_ = 'f: {
    l_chr_3 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_3) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if (((((((((p_always_2) == (0i32)) as i32)) != 0) && ((((!((F::BtlChar_IsStage4Or27(c)) != 0)) as i32)) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    break 'f (({ let a_ = (F::BtlChar_GetPos(c, l_chr_3)).wrapping_add(208); (rt::bf_get_u(c, a_, 7, 1) as u8) }) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_TestPoseBit80", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_TestFlag98(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_TestFlag98", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f F::BtlChar_TestFlag(c, l_chr_2, ((((152i32) as i64 as u64)) as u32));
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_TestFlag98", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_TestFlagBE(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_TestFlagBE", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f F::BtlChar_TestFlag(c, l_chr_2, ((((190i32) as i64 as u64)) as u32));
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_TestFlagBE", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_GetUnkE40(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetUnkE40", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f (c.m.r32((l_chr_2).wrapping_add(3648)) as i32);
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetUnkE40", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_GetParamByte8F(c: &mut Ctx, mut p_objId_1: i32, mut p_n_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetParamByte8F", &PROBE_N, &[(p_objId_1) as u32, (p_n_2) as u32], false); }
    let mut l_chr_3: u32 = 0;
    let r_ = 'f: {
    l_chr_3 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_3) != (((0i32) as u32))) as i32)) != 0) {
        break 'f F::BtlParam_GetUnk8F(c, l_chr_3, p_n_2);
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetParamByte8F", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCtrl_StartEntrance(c: &mut Ctx, mut p_player_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_StartEntrance", &PROBE_N, &[(p_player_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_Get(c, p_player_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        F::BtlChar_SetFlag(c, l_chr_2, ((((239i32) as i64 as u64)) as u32));
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_StartEntrance", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCtrl_EndEntrance(c: &mut Ctx, mut p_player_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_EndEntrance", &PROBE_N, &[(p_player_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_Get(c, p_player_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        F::BtlChar_SetFlag(c, l_chr_2, ((((240i32) as i64 as u64)) as u32));
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_EndEntrance", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCtrl_StartWinPose(c: &mut Ctx, mut p_player_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_StartWinPose", &PROBE_N, &[(p_player_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_Get(c, p_player_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        F::BtlChar_SetFlag(c, l_chr_2, ((((241i32) as i64 as u64)) as u32));
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_StartWinPose", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCtrl_StartLosePose(c: &mut Ctx, mut p_player_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_StartLosePose", &PROBE_N, &[(p_player_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_Get(c, p_player_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        F::BtlChar_SetFlag(c, l_chr_2, ((((242i32) as i64 as u64)) as u32));
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_StartLosePose", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCtrl_IsPoseReached(c: &mut Ctx, mut p_player_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_IsPoseReached", &PROBE_N, &[(p_player_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_Get(c, p_player_1);
    if (((((l_chr_2) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    's1: {
        let sv3: i32 = F::BtlAnim_GetId(c, l_chr_2);
        match sv3 {
            385i32 | 387i32 => {
                break 'f 1i32;
            }
            388i32 => {
                if ((F::BtlChar_TestFlag(c, l_chr_2, ((((49i32) as i64 as u64)) as u32))) != 0) {
                    break 'f 1i32;
                }
                break 's1;
            }
            _ => {
            }
        }
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_IsPoseReached", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCtrl_PlayMotion(c: &mut Ctx, mut p_player_1: i32, mut p_motion_2: i32, mut p_loop_3: i32, mut p_blend_4: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_PlayMotion", &PROBE_N, &[(p_player_1) as u32, (p_motion_2) as u32, (p_loop_3) as u32, (p_blend_4).0], false); }
    let mut l_chr_5: u32 = 0;
    let r_ = 'f: {
    l_chr_5 = F::BtlChar_Get(c, p_player_1);
    if (((((l_chr_5) != (((0i32) as u32))) as i32)) != 0) {
        { let v_ = p_motion_2; let a_ = (l_chr_5).wrapping_add(5456); c.m.w32(a_, (v_) as u32); };
        { let v_ = p_blend_4; let a_ = (l_chr_5).wrapping_add(5460); c.m.w32(a_, (v_).0); };
        { let v_ = p_loop_3; let a_ = (l_chr_5).wrapping_add(5464); c.m.w32(a_, (v_) as u32); };
        F::BtlChar_SetHeldFlag(c, l_chr_5, ((((250i32) as i64 as u64)) as u32));
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_PlayMotion", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCtrl_SetPos(c: &mut Ctx, mut p_player_1: i32, mut p_pos_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_SetPos", &PROBE_N, &[(p_player_1) as u32, (p_pos_2) as u32], false); }
    let mut l_chr_3: u32 = 0;
    let r_ = 'f: {
    l_chr_3 = F::BtlChar_Get(c, p_player_1);
    if (((((l_chr_3) != (((0i32) as u32))) as i32)) != 0) {
        F::Vec4_Copy(c, (l_chr_3).wrapping_add(5472), p_pos_2);
        F::BtlChar_SetHeldFlag(c, l_chr_3, ((((252i32) as i64 as u64)) as u32));
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_SetPos", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCtrl_GetPos(c: &mut Ctx, mut p_player_1: i32, mut p_out_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_GetPos", &PROBE_N, &[(p_player_1) as u32, (p_out_2) as u32], false); }
    let mut l_chr_3: u32 = 0;
    let r_ = 'f: {
    l_chr_3 = F::BtlChar_Get(c, p_player_1);
    if (((((l_chr_3) != (((0i32) as u32))) as i32)) != 0) {
        { let a1_ = F::BtlChar_GetPos(c, l_chr_3); F::Vec4_Copy(c, p_out_2, a1_) };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_GetPos", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCtrl_SetRot(c: &mut Ctx, mut p_player_1: i32, mut p_rot_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_SetRot", &PROBE_N, &[(p_player_1) as u32, (p_rot_2) as u32], false); }
    let mut l_chr_3: u32 = 0;
    let r_ = 'f: {
    l_chr_3 = F::BtlChar_Get(c, p_player_1);
    if (((((l_chr_3) != (((0i32) as u32))) as i32)) != 0) {
        F::Vec4_Copy(c, (l_chr_3).wrapping_add(5488), p_rot_2);
        F::BtlChar_SetHeldFlag(c, l_chr_3, ((((253i32) as i64 as u64)) as u32));
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_SetRot", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCtrl_GetRot(c: &mut Ctx, mut p_player_1: i32, mut p_out_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_GetRot", &PROBE_N, &[(p_player_1) as u32, (p_out_2) as u32], false); }
    let mut l_chr_3: u32 = 0;
    let r_ = 'f: {
    l_chr_3 = F::BtlChar_Get(c, p_player_1);
    if (((((l_chr_3) != (((0i32) as u32))) as i32)) != 0) {
        { let a1_ = (F::BtlChar_GetPos(c, l_chr_3)).wrapping_add(16); F::Vec4_Copy(c, p_out_2, a1_) };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_GetRot", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCtrl_GetActiveMember(c: &mut Ctx, mut p_player_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_GetActiveMember", &PROBE_N, &[(p_player_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_Get(c, p_player_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f F::BtlMember_GetActiveIndex(c, l_chr_2);
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_GetActiveMember", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCtrl_IsAction4(c: &mut Ctx, mut p_player_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_IsAction4", &PROBE_N, &[(p_player_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_Get(c, p_player_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f (((F::BtlAct_GetCurrent(c, l_chr_2)) == (4i32)) as i32);
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_IsAction4", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCtrl_IsMotionPlaying(c: &mut Ctx, mut p_player_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_IsMotionPlaying", &PROBE_N, &[(p_player_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_Get(c, p_player_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        if ((((((c.m.r32((l_chr_2).wrapping_add(5464)) as i32)) == (1i32)) as i32)) != 0) {
            break 'f 1i32;
        }
        break 'f (((F::BtlChar_TestFlag(c, l_chr_2, ((((49i32) as i64 as u64)) as u32))) == (0i32)) as i32);
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_IsMotionPlaying", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCtrl_StopMotion(c: &mut Ctx, mut p_player_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_StopMotion", &PROBE_N, &[(p_player_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_Get(c, p_player_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        F::BtlChar_ClearFlag(c, l_chr_2, ((((250i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, l_chr_2, ((((251i32) as i64 as u64)) as u32));
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_StopMotion", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCtrl_SetAuraOn(c: &mut Ctx, mut p_player_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_SetAuraOn", &PROBE_N, &[(p_player_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_Get(c, p_player_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        F::BtlChar_SetHeldFlag(c, l_chr_2, ((((254i32) as i64 as u64)) as u32));
        F::BtlChar_ClearFlag(c, l_chr_2, ((((255i32) as i64 as u64)) as u32));
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_SetAuraOn", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCtrl_SetAuraOff(c: &mut Ctx, mut p_player_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_SetAuraOff", &PROBE_N, &[(p_player_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_Get(c, p_player_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        F::BtlChar_ClearFlag(c, l_chr_2, ((((254i32) as i64 as u64)) as u32));
        F::BtlChar_SetHeldFlag(c, l_chr_2, ((((255i32) as i64 as u64)) as u32));
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_SetAuraOff", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCtrl_SetChargeFx(c: &mut Ctx, mut p_player_1: i32, mut p_on_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_SetChargeFx", &PROBE_N, &[(p_player_1) as u32, (p_on_2) as u32], false); }
    let mut l_chr_3: u32 = 0;
    let r_ = 'f: {
    l_chr_3 = F::BtlChar_Get(c, p_player_1);
    if (((((l_chr_3) != (((0i32) as u32))) as i32)) != 0) {
        if ((p_on_2) != 0) {
            F::BtlChar_SetHeldFlag(c, l_chr_3, ((((257i32) as i64 as u64)) as u32));
        } else {
            F::BtlChar_ClearFlag(c, l_chr_3, ((((257i32) as i64 as u64)) as u32));
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_SetChargeFx", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCtrl_BurstChargeFx(c: &mut Ctx, mut p_player_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_BurstChargeFx", &PROBE_N, &[(p_player_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_Get(c, p_player_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        F::BtlChar_SetHeldFlag(c, l_chr_2, ((((258i32) as i64 as u64)) as u32));
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_BurstChargeFx", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCtrl_ClearHidden(c: &mut Ctx, mut p_player_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_ClearHidden", &PROBE_N, &[(p_player_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_Get(c, p_player_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        F::BtlChar_ClearFlag(c, l_chr_2, ((((256i32) as i64 as u64)) as u32));
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_ClearHidden", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCtrl_SetHidden(c: &mut Ctx, mut p_player_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_SetHidden", &PROBE_N, &[(p_player_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_Get(c, p_player_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        F::BtlChar_SetHeldFlag(c, l_chr_2, ((((256i32) as i64 as u64)) as u32));
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_SetHidden", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCtrl_ReloadMemberBonus(c: &mut Ctx, mut p_player_1: i32, mut p_member_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_ReloadMemberBonus", &PROBE_N, &[(p_player_1) as u32, (p_member_2) as u32], false); }
    let mut l_chr_3: u32 = 0;
    let r_ = 'f: {
    l_chr_3 = F::BtlChar_Get(c, p_player_1);
    if (((((((((l_chr_3) != (((0i32) as u32))) as i32)) != 0) && (((((p_member_2) < (((5i32) as u32))) as i32)) != 0)) as i32)) != 0) {
        F::BtlChar_SetHeldFlag(c, l_chr_3, (((((((p_member_2).wrapping_add(((259i32) as u32))) as i32)) as i64 as u64)) as u32));
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_ReloadMemberBonus", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCtrl_ReloadMemberAbilities(c: &mut Ctx, mut p_player_1: i32, mut p_member_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_ReloadMemberAbilities", &PROBE_N, &[(p_player_1) as u32, (p_member_2) as u32], false); }
    let mut l_chr_3: u32 = 0;
    let r_ = 'f: {
    l_chr_3 = F::BtlChar_Get(c, p_player_1);
    if (((((((((l_chr_3) != (((0i32) as u32))) as i32)) != 0) && (((((p_member_2) < (((5i32) as u32))) as i32)) != 0)) as i32)) != 0) {
        F::BtlChar_SetHeldFlag(c, l_chr_3, (((((((p_member_2).wrapping_add(((264i32) as u32))) as i32)) as i64 as u64)) as u32));
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_ReloadMemberAbilities", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCtrl_SetFlag10D(c: &mut Ctx, mut p_player_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_SetFlag10D", &PROBE_N, &[(p_player_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_Get(c, p_player_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        F::BtlChar_SetHeldFlag(c, l_chr_2, ((((269i32) as i64 as u64)) as u32));
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_SetFlag10D", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCtrl_ClearFlag10D(c: &mut Ctx, mut p_player_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_ClearFlag10D", &PROBE_N, &[(p_player_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_Get(c, p_player_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        F::BtlChar_ClearFlag(c, l_chr_2, ((((269i32) as i64 as u64)) as u32));
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_ClearFlag10D", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCtrl_AddHp(c: &mut Ctx, mut p_player_1: i32, mut p_member_2: i32, mut p_percent_3: F32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_AddHp", &PROBE_N, &[(p_player_1) as u32, (p_member_2) as u32, (p_percent_3).0], false); }
    let mut l_chr_4: u32 = 0;
    let mut l_g_5: u32 = 0;
    let mut l_amount_6: i32 = 0;
    let r_ = 'f: {
    l_chr_4 = F::BtlChar_Get(c, p_player_1);
    if (((((l_chr_4) != (((0i32) as u32))) as i32)) != 0) {
        l_g_5 = F::BtlMember_GetGauge(c, l_chr_4, p_member_2);
        l_amount_6 = rt::f2i(rt::fmul(rt::fmul(rt::i2f((c.m.r32((l_g_5).wrapping_add(4)) as i32)), p_percent_3), F32(0x3c23d70a)));
        if (((((p_member_2) == (F::BtlMember_GetActiveIndex(c, l_chr_4))) as i32)) != 0) {
            if (((((l_amount_6) >= (0i32)) as i32)) != 0) {
                F::BtlMember_AddHealth(c, l_chr_4, l_amount_6);
            } else {
                { let _ = F::BtlMember_Damage(c, l_chr_4, (l_amount_6).wrapping_neg(), 1067i32); };
            }
        } else {
            { let v_ = { let a0_ = ((c.m.r32(l_g_5) as i32)).wrapping_add(l_amount_6); let a2_ = (c.m.r32((l_g_5).wrapping_add(4)) as i32); F::BtlUtil_Clamp(c, a0_, 0i32, a2_) }; let a_ = l_g_5; c.m.w32(a_, (v_) as u32); };
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_AddHp", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCtrl_RaiseHp(c: &mut Ctx, mut p_player_1: i32, mut p_member_2: i32, mut p_percent_3: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_RaiseHp", &PROBE_N, &[(p_player_1) as u32, (p_member_2) as u32, (p_percent_3).0], false); }
    let mut l_chr_4: u32 = 0;
    let mut l_g_5: u32 = 0;
    let mut l_target_6: i32 = 0;
    let mut l_max_7: i32 = 0;
    let r_ = 'f: {
    l_chr_4 = F::BtlChar_Get(c, p_player_1);
    if (((((l_chr_4) != (((0i32) as u32))) as i32)) != 0) {
        l_g_5 = F::BtlMember_GetGauge(c, l_chr_4, p_member_2);
        l_max_7 = (c.m.r32((l_g_5).wrapping_add(4)) as i32);
        l_target_6 = rt::f2i(rt::fmul(rt::fmul(rt::i2f(l_max_7), p_percent_3), F32(0x3c23d70a)));
        if (((((((((l_target_6) <= (0i32)) as i32)) != 0) && (((rt::flt(F32(0x00000000), p_percent_3) as i32)) != 0)) as i32)) != 0) {
            l_target_6 = 1i32;
        }
        { let v_ = { let a1_ = (c.m.r32(l_g_5) as i32); F::BtlUtil_Clamp(c, l_target_6, a1_, l_max_7) }; let a_ = l_g_5; c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_RaiseHp", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCtrl_LowerHp(c: &mut Ctx, mut p_player_1: i32, mut p_member_2: i32, mut p_percent_3: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_LowerHp", &PROBE_N, &[(p_player_1) as u32, (p_member_2) as u32, (p_percent_3).0], false); }
    let mut l_chr_4: u32 = 0;
    let mut l_g_5: u32 = 0;
    let mut l_target_6: i32 = 0;
    let r_ = 'f: {
    l_chr_4 = F::BtlChar_Get(c, p_player_1);
    if (((((l_chr_4) != (((0i32) as u32))) as i32)) != 0) {
        l_g_5 = F::BtlMember_GetGauge(c, l_chr_4, p_member_2);
        l_target_6 = rt::f2i(rt::fmul(rt::fmul(rt::i2f((c.m.r32((l_g_5).wrapping_add(4)) as i32)), p_percent_3), F32(0x3c23d70a)));
        if (((((((((l_target_6) <= (0i32)) as i32)) != 0) && (((rt::flt(F32(0x00000000), p_percent_3) as i32)) != 0)) as i32)) != 0) {
            l_target_6 = 1i32;
        }
        { let v_ = { let a2_ = (c.m.r32(l_g_5) as i32); F::BtlUtil_Clamp(c, l_target_6, 0i32, a2_) }; let a_ = l_g_5; c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_LowerHp", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCtrl_AddKi(c: &mut Ctx, mut p_player_1: i32, mut p_member_2: i32, mut p_bars_3: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_AddKi", &PROBE_N, &[(p_player_1) as u32, (p_member_2) as u32, (p_bars_3) as u32], false); }
    let mut l_chr_4: u32 = 0;
    let mut l_g_5: u32 = 0;
    let mut l_amount_6: i32 = 0;
    let r_ = 'f: {
    l_chr_4 = F::BtlChar_Get(c, p_player_1);
    if (((((l_chr_4) != (((0i32) as u32))) as i32)) != 0) {
        l_g_5 = F::BtlMember_GetGauge(c, l_chr_4, p_member_2);
        l_amount_6 = (p_bars_3).wrapping_mul(20000i32);
        if (((((p_member_2) == (F::BtlMember_GetActiveIndex(c, l_chr_4))) as i32)) != 0) {
            if (((((l_amount_6) >= (0i32)) as i32)) != 0) {
                { let _ = F::BtlMember_AddKi(c, l_chr_4, l_amount_6); };
            } else {
                { let _ = F::BtlMember_DrainKi(c, l_chr_4, (l_amount_6).wrapping_neg()); };
            }
        } else {
            { let v_ = { let a0_ = ((c.m.r32((l_g_5).wrapping_add(12)) as i32)).wrapping_add(l_amount_6); let a2_ = (c.m.r32((l_g_5).wrapping_add(16)) as i32); F::BtlUtil_Clamp(c, a0_, 0i32, a2_) }; let a_ = (l_g_5).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_AddKi", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCtrl_RaiseKi(c: &mut Ctx, mut p_player_1: i32, mut p_member_2: i32, mut p_bars_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_RaiseKi", &PROBE_N, &[(p_player_1) as u32, (p_member_2) as u32, (p_bars_3) as u32], false); }
    let mut l_chr_4: u32 = 0;
    let mut l_g_5: u32 = 0;
    let r_ = 'f: {
    l_chr_4 = F::BtlChar_Get(c, p_player_1);
    if (((((l_chr_4) != (((0i32) as u32))) as i32)) != 0) {
        l_g_5 = F::BtlMember_GetGauge(c, l_chr_4, p_member_2);
        { let v_ = { let a1_ = (c.m.r32((l_g_5).wrapping_add(12)) as i32); let a2_ = (c.m.r32((l_g_5).wrapping_add(16)) as i32); F::BtlUtil_Clamp(c, (p_bars_3).wrapping_mul(20000i32), a1_, a2_) }; let a_ = (l_g_5).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_RaiseKi", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCtrl_LowerKi(c: &mut Ctx, mut p_player_1: i32, mut p_member_2: i32, mut p_bars_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_LowerKi", &PROBE_N, &[(p_player_1) as u32, (p_member_2) as u32, (p_bars_3) as u32], false); }
    let mut l_chr_4: u32 = 0;
    let mut l_g_5: u32 = 0;
    let r_ = 'f: {
    l_chr_4 = F::BtlChar_Get(c, p_player_1);
    if (((((l_chr_4) != (((0i32) as u32))) as i32)) != 0) {
        l_g_5 = F::BtlMember_GetGauge(c, l_chr_4, p_member_2);
        { let v_ = { let a2_ = (c.m.r32((l_g_5).wrapping_add(12)) as i32); F::BtlUtil_Clamp(c, (p_bars_3).wrapping_mul(20000i32), 0i32, a2_) }; let a_ = (l_g_5).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_LowerKi", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCtrl_AddBlast(c: &mut Ctx, mut p_player_1: i32, mut p_member_2: i32, mut p_stocks_3: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_AddBlast", &PROBE_N, &[(p_player_1) as u32, (p_member_2) as u32, (p_stocks_3) as u32], false); }
    let mut l_chr_4: u32 = 0;
    let mut l_g_5: u32 = 0;
    let mut l_amount_6: i32 = 0;
    let r_ = 'f: {
    l_chr_4 = F::BtlChar_Get(c, p_player_1);
    if (((((l_chr_4) != (((0i32) as u32))) as i32)) != 0) {
        l_g_5 = F::BtlMember_GetGauge(c, l_chr_4, p_member_2);
        l_amount_6 = (p_stocks_3).wrapping_mul(100000i32);
        if (((((p_member_2) == (F::BtlMember_GetActiveIndex(c, l_chr_4))) as i32)) != 0) {
            if (((((l_amount_6) >= (0i32)) as i32)) != 0) {
                F::BtlMember_AddBlast(c, l_chr_4, l_amount_6);
            } else {
                F::BtlMember_SubBlast(c, l_chr_4, (l_amount_6).wrapping_neg());
            }
        } else {
            { let v_ = { let a0_ = ((c.m.r32((l_g_5).wrapping_add(20)) as i32)).wrapping_add(l_amount_6); let a2_ = (c.m.r32((l_g_5).wrapping_add(24)) as i32); F::BtlUtil_Clamp(c, a0_, 0i32, a2_) }; let a_ = (l_g_5).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_AddBlast", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCtrl_RaiseBlast(c: &mut Ctx, mut p_player_1: i32, mut p_member_2: i32, mut p_stocks_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_RaiseBlast", &PROBE_N, &[(p_player_1) as u32, (p_member_2) as u32, (p_stocks_3) as u32], false); }
    let mut l_chr_4: u32 = 0;
    let mut l_g_5: u32 = 0;
    let r_ = 'f: {
    l_chr_4 = F::BtlChar_Get(c, p_player_1);
    if (((((l_chr_4) != (((0i32) as u32))) as i32)) != 0) {
        l_g_5 = F::BtlMember_GetGauge(c, l_chr_4, p_member_2);
        { let v_ = { let a1_ = (c.m.r32((l_g_5).wrapping_add(20)) as i32); let a2_ = (c.m.r32((l_g_5).wrapping_add(24)) as i32); F::BtlUtil_Clamp(c, (p_stocks_3).wrapping_mul(100000i32), a1_, a2_) }; let a_ = (l_g_5).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_RaiseBlast", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCtrl_LowerBlast(c: &mut Ctx, mut p_player_1: i32, mut p_member_2: i32, mut p_stocks_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_LowerBlast", &PROBE_N, &[(p_player_1) as u32, (p_member_2) as u32, (p_stocks_3) as u32], false); }
    let mut l_chr_4: u32 = 0;
    let mut l_g_5: u32 = 0;
    let r_ = 'f: {
    l_chr_4 = F::BtlChar_Get(c, p_player_1);
    if (((((l_chr_4) != (((0i32) as u32))) as i32)) != 0) {
        l_g_5 = F::BtlMember_GetGauge(c, l_chr_4, p_member_2);
        { let v_ = { let a2_ = (c.m.r32((l_g_5).wrapping_add(20)) as i32); F::BtlUtil_Clamp(c, (p_stocks_3).wrapping_mul(100000i32), 0i32, a2_) }; let a_ = (l_g_5).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_LowerBlast", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCtrl_SetMaxPower(c: &mut Ctx, mut p_player_1: i32, mut p_on_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_SetMaxPower", &PROBE_N, &[(p_player_1) as u32, (p_on_2) as u32], false); }
    let mut l_chr_3: u32 = 0;
    let r_ = 'f: {
    l_chr_3 = F::BtlChar_Get(c, p_player_1);
    if (((((l_chr_3) != (((0i32) as u32))) as i32)) != 0) {
        if ((p_on_2) != 0) {
            { let _ = F::BtlMember_AddKi(c, l_chr_3, 100000i32); };
            F::BtlMember_AddMaxPower(c, l_chr_3, 30000i32);
            F::BtlChar_SetHeldFlag(c, l_chr_3, ((((6i32) as i64 as u64)) as u32));
        } else {
            F::BtlChar_ClearFlag(c, l_chr_3, ((((6i32) as i64 as u64)) as u32));
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_SetMaxPower", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCtrl_Transform(c: &mut Ctx, mut p_player_1: i32, mut p_id_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_Transform", &PROBE_N, &[(p_player_1) as u32, (p_id_2) as u32], false); }
    let mut l_chr_3: u32 = 0;
    let mut l_slot_4: i32 = 0;
    let mut l_i_5: i32 = 0;
    let mut l_g_6: u32 = 0;
    let r_ = 'f: {
    l_chr_3 = F::BtlChar_Get(c, p_player_1);
    if (((((l_chr_3) != (((0i32) as u32))) as i32)) != 0) {
        l_slot_4 = (1i32).wrapping_neg();
        {
            l_i_5 = 0i32;
            'l1: loop {
                if !(((((l_i_5) < (4i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    if (((((F::BtlParam_GetSlotId(c, l_chr_3, l_i_5)) == (p_id_2)) as i32)) != 0) {
                        l_slot_4 = l_i_5;
                        break 'l1;
                    }
                }
                { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
            }
        }
        if (((((l_slot_4) < (0i32)) as i32)) != 0) {
            break 'f 0i32;
        }
        if ((((!((F::BtlAct_CanTransform(c, l_chr_3, ((l_slot_4) as u32), 0i32, 0i32)) != 0)) as i32)) != 0) {
            break 'f 0i32;
        }
        l_g_6 = F::BtlMember_GetActiveGauge(c, l_chr_3);
        { let v_ = { let a0_ = F::BtlParam_GetSlotCost(c, l_chr_3, l_slot_4); let a1_ = (c.m.r32((l_g_6).wrapping_add(20)) as i32); let a2_ = (c.m.r32((l_g_6).wrapping_add(24)) as i32); F::BtlUtil_Clamp(c, a0_, a1_, a2_) }; let a_ = (l_g_6).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
        F::BtlChar_SetHeldFlag(c, l_chr_3, (((((l_slot_4).wrapping_add(278i32)) as i64 as u64)) as u32));
        F::BtlChar_ClearFlag(c, l_chr_3, ((((190i32) as i64 as u64)) as u32));
        { let v_ = 0i32; let a_ = (l_chr_3).wrapping_add(4064); c.m.w32(a_, (v_) as u32); };
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_Transform", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCtrl_Fuse(c: &mut Ctx, mut p_player_1: i32, mut p_id_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_Fuse", &PROBE_N, &[(p_player_1) as u32, (p_id_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_chr_3: u32 = 0;
    let mut l_slot_4: i32 = 0;
    let mut l_i_5: i32 = 0;
    let mut l_g_6: u32 = 0;
    let r_ = 'f: {
        l_chr_3 = F::BtlChar_Get(c, p_player_1);
        if (((((l_chr_3) != (((0i32) as u32))) as i32)) != 0) {
            l_slot_4 = (1i32).wrapping_neg();
            {
                l_i_5 = 0i32;
                'l1: loop {
                    if !(((((l_i_5) < (3i32)) as i32)) != 0) { break 'l1; }
                    'c2: {
                        if (((((F::BtlParam_GetUnkB4(c, l_chr_3, l_i_5)) == (p_id_2)) as i32)) != 0) {
                            l_slot_4 = l_i_5;
                            break 'l1;
                        }
                    }
                    { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
                }
            }
            if (((((l_slot_4) < (0i32)) as i32)) != 0) {
                break 'f 0i32;
            }
            if ((((!((F::BtlAct_CanFuse(c, l_chr_3, l_slot_4, 0i32, 0i32, fp)) != 0)) as i32)) != 0) {
                break 'f 0i32;
            }
            l_g_6 = F::BtlMember_GetActiveGauge(c, l_chr_3);
            { let v_ = { let a0_ = F::BtlParam_GetCostAE(c, l_chr_3, l_slot_4); let a1_ = (c.m.r32((l_g_6).wrapping_add(20)) as i32); let a2_ = (c.m.r32((l_g_6).wrapping_add(24)) as i32); F::BtlUtil_Clamp(c, a0_, a1_, a2_) }; let a_ = (l_g_6).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
            F::BtlChar_SetHeldFlag(c, l_chr_3, (((((l_slot_4).wrapping_add(282i32)) as i64 as u64)) as u32));
            F::BtlChar_ClearFlag(c, l_chr_3, ((((190i32) as i64 as u64)) as u32));
            { let v_ = 0i32; let a_ = (l_chr_3).wrapping_add(4064); c.m.w32(a_, (v_) as u32); };
            { let v_ = (c.m.r32((l_g_6).wrapping_add(4)) as i32); let a_ = l_g_6; c.m.w32(a_, (v_) as u32); };
            { let v_ = { let a_ = ({ let a1_ = (c.m.r32(fp) as i32); F::BtlMember_GetGauge(c, l_chr_3, a1_) }).wrapping_add(4); (c.m.r32(a_) as i32) }; let a_ = { let a1_ = (c.m.r32(fp) as i32); F::BtlMember_GetGauge(c, l_chr_3, a1_) }; c.m.w32(a_, (v_) as u32); };
            break 'f 1i32;
        }
        break 'f 0i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_Fuse", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCtrl_ChangeMember(c: &mut Ctx, mut p_player_1: i32, mut p_member_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_ChangeMember", &PROBE_N, &[(p_player_1) as u32, (p_member_2) as u32], false); }
    let mut l_chr_3: u32 = 0;
    let r_ = 'f: {
    l_chr_3 = F::BtlChar_Get(c, p_player_1);
    if (((((l_chr_3) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if (((((p_member_2) == (F::BtlMember_GetActiveIndex(c, l_chr_3))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((!((F::BtlChar_IsDead(c, l_chr_3)) != 0)) as i32)) != 0) {
        if ((((!((F::BtlAct_CanSwitch(c, l_chr_3, 0i32, 0i32)) != 0)) as i32)) != 0) {
            break 'f 0i32;
        }
    }
    if ((((!((F::BtlMember_SetSwitchTarget(c, l_chr_3, p_member_2)) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((!((F::BtlChar_IsDead(c, l_chr_3)) != 0)) as i32)) != 0) {
        { let v_ = 100000i32; let a_ = (l_chr_3).wrapping_add(2460); c.m.w32(a_, (v_) as u32); };
        F::BtlChar_SetHeldFlag(c, l_chr_3, ((((285i32) as i64 as u64)) as u32));
        F::BtlChar_ClearFlag(c, l_chr_3, ((((190i32) as i64 as u64)) as u32));
        { let v_ = 0i32; let a_ = (l_chr_3).wrapping_add(4064); c.m.w32(a_, (v_) as u32); };
    } else {
        F::BtlChar_SetHeldFlag(c, l_chr_3, ((((270i32) as i64 as u64)) as u32));
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_ChangeMember", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCtrl_ForceFlag11x(c: &mut Ctx, mut p_player_1: i32, mut p_kind_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_ForceFlag11x", &PROBE_N, &[(p_player_1) as u32, (p_kind_2) as u32], false); }
    let mut l_chr_3: u32 = 0;
    let mut l_flag_4: i32 = 0;
    let r_ = 'f: {
    l_chr_3 = F::BtlChar_Get(c, p_player_1);
    if (((((l_chr_3) != (((0i32) as u32))) as i32)) != 0) {
        's1: {
            let sv5: i32 = p_kind_2;
            match sv5 {
                0i32 => {
                    l_flag_4 = 272i32;
                    break 's1;
                }
                1i32 => {
                    l_flag_4 = 273i32;
                    break 's1;
                }
                _ => {
                    l_flag_4 = 272i32;
                    break 's1;
                }
            }
        }
        F::BtlChar_SetHeldFlag(c, l_chr_3, ((((l_flag_4) as i64 as u64)) as u32));
        F::BtlChar_ClearFlag(c, l_chr_3, ((((190i32) as i64 as u64)) as u32));
        { let v_ = 0i32; let a_ = (l_chr_3).wrapping_add(4064); c.m.w32(a_, (v_) as u32); };
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_ForceFlag11x", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCtrl_UseTechnique(c: &mut Ctx, mut p_player_1: i32, mut p_kind_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_UseTechnique", &PROBE_N, &[(p_player_1) as u32, (p_kind_2) as u32], false); }
    let mut l_chr_3: u32 = 0;
    let mut l_g_4: u32 = 0;
    let r_ = 'f: {
    l_chr_3 = F::BtlChar_Get(c, p_player_1);
    if (((((l_chr_3) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    l_g_4 = F::BtlMember_GetActiveGauge(c, l_chr_3);
    's1: {
        let sv5: i32 = p_kind_2;
        match sv5 {
            0i32 | 1i32 => {
                { let v_ = { let a0_ = F::BtlSkill_GetBlastCost(c, l_chr_3, p_kind_2); let a1_ = (c.m.r32((l_g_4).wrapping_add(20)) as i32); let a2_ = (c.m.r32((l_g_4).wrapping_add(24)) as i32); F::BtlUtil_Clamp(c, a0_, a1_, a2_) }; let a_ = (l_g_4).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
                if ((((((((F::BtlSkill_GetFlags(c, l_chr_3, p_kind_2)) as u64)) as i32)) & (1i32))) != 0) {
                    F::BtlChar_SetHeldFlag(c, l_chr_3, ((((5i32) as i64 as u64)) as u32));
                }
                break 's1;
            }
            2i32 | 3i32 | 4i32 => {
                { let v_ = { let a0_ = F::BtlSuper_GetKiCost(c, l_chr_3, p_kind_2); let a1_ = (c.m.r32((l_g_4).wrapping_add(12)) as i32); let a2_ = (c.m.r32((l_g_4).wrapping_add(16)) as i32); F::BtlUtil_Clamp(c, a0_, a1_, a2_) }; let a_ = (l_g_4).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                if (((((p_kind_2) == (4i32)) as i32)) != 0) {
                    F::BtlChar_SetHeldFlag(c, l_chr_3, ((((6i32) as i64 as u64)) as u32));
                    { let v_ = 30000i32; let a_ = (l_g_4).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                    { let v_ = (c.m.r32((l_g_4).wrapping_add(16)) as i32); let a_ = (l_g_4).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                }
                F::BtlChar_SetHeldFlag(c, l_chr_3, ((((287i32) as i64 as u64)) as u32));
                F::BtlChar_SetHeldFlag(c, l_chr_3, ((((5i32) as i64 as u64)) as u32));
                break 's1;
            }
            _ => {
                break 'f 0i32;
            }
        }
    }
    F::BtlChar_SetHeldFlag(c, l_chr_3, (((((p_kind_2).wrapping_add(288i32)) as i64 as u64)) as u32));
    F::BtlChar_SetHeldFlag(c, l_chr_3, ((((286i32) as i64 as u64)) as u32));
    F::BtlChar_ClearFlag(c, l_chr_3, ((((190i32) as i64 as u64)) as u32));
    { let v_ = 0i32; let a_ = (l_chr_3).wrapping_add(4064); c.m.w32(a_, (v_) as u32); };
    { let v_ = 0i32; let a_ = (l_chr_3).wrapping_add(3648); c.m.w32(a_, (v_) as u32); };
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_UseTechnique", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCtrl_ForceReaction(c: &mut Ctx, mut p_player_1: i32, mut p_kind_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_ForceReaction", &PROBE_N, &[(p_player_1) as u32, (p_kind_2) as u32], false); }
    let mut l_chr_3: u32 = 0;
    let r_ = 'f: {
    l_chr_3 = F::BtlChar_Get(c, p_player_1);
    if (((((l_chr_3) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if (((((p_kind_2) >= (((4i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    F::BtlChar_SetHeldFlag(c, l_chr_3, (((((((p_kind_2).wrapping_add(((274i32) as u32))) as i32)) as i64 as u64)) as u32));
    F::BtlChar_ClearFlag(c, l_chr_3, ((((190i32) as i64 as u64)) as u32));
    { let v_ = 0i32; let a_ = (l_chr_3).wrapping_add(4064); c.m.w32(a_, (v_) as u32); };
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_ForceReaction", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCtrl_IsInterruptible(c: &mut Ctx, mut p_player_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_IsInterruptible", &PROBE_N, &[(p_player_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let mut l_action_3: i32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_Get(c, p_player_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        l_action_3 = F::BtlAct_GetCurrent(c, l_chr_2);
        's1: {
            let sv4: i32 = l_action_3;
            match sv4 {
                183i32 | 184i32 | 185i32 | 186i32 | 187i32 | 188i32 | 190i32 | 191i32 | 250i32 | 251i32 | 252i32 | 260i32 | 261i32 | 304i32 | 305i32 | 306i32 => {
                    break 'f 0i32;
                }
                _ => {
                }
            }
        }
        if ((((((((l_action_3).wrapping_sub(253i32)) as u32)) < (((6i32) as u32))) as i32)) != 0) {
            break 'f 0i32;
        }
        if ((((((((l_action_3).wrapping_sub(262i32)) as u32)) < (((54i32) as u32))) as i32)) != 0) {
            break 'f 0i32;
        }
        if ((((((((l_action_3).wrapping_sub(236i32)) as u32)) < (((14i32) as u32))) as i32)) != 0) {
            break 'f 0i32;
        }
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_IsInterruptible", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCtrl_CanAct(c: &mut Ctx, mut p_player_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_CanAct", &PROBE_N, &[(p_player_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let mut l_action_3: i32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_Get(c, p_player_1);
    if (((((l_chr_2) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((F::BtlChar_IsFrozen(c, l_chr_2)) != 0) {
        break 'f 0i32;
    }
    if ((((((c.m.r32((l_chr_2).wrapping_add(4064)) as i32)) > (0i32)) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((((c.m.r32((l_chr_2).wrapping_add(4016)) as i32)) >= (3i32)) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((F::BtlChar_TestFlag(c, l_chr_2, ((((175i32) as i64 as u64)) as u32))) != 0) {
        break 'f 0i32;
    }
    if ((F::BtlChar_TestFlag(c, l_chr_2, ((((185i32) as i64 as u64)) as u32))) != 0) {
        break 'f 0i32;
    }
    l_action_3 = F::BtlAct_GetCurrent(c, l_chr_2);
    's1: {
        let sv4: i32 = l_action_3;
        match sv4 {
            11i32 | 13i32 | 14i32 | 54i32 => {
                break 'f 1i32;
            }
            _ => {
                if ((F::BtlChar_IsDead(c, l_chr_2)) != 0) {
                    's2: {
                        let sv5: i32 = l_action_3;
                        match sv5 {
                            216i32 | 235i32 => {
                                break 'f 1i32;
                            }
                            _ => {
                            }
                        }
                    }
                }
                break 's1;
            }
        }
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_CanAct", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChange_IsPendingType0(c: &mut Ctx, mut p_player_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChange_IsPendingType0", &PROBE_N, &[(p_player_1) as u32], false); }
    let mut l_q_2: u32 = 0;
    let r_ = 'f: {
    if (((((c.m.r32(sym::gBtlChars)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    l_q_2 = (c.m.r32(sym::gBtlChars)).wrapping_add(312);
    if (((((c.m.r32((l_q_2).wrapping_add(288))) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((({ let a_ = (c.m.r32((l_q_2).wrapping_add(288))).wrapping_add(4); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((({ let a_ = c.m.r32((l_q_2).wrapping_add(288)); (c.m.r32(a_) as i32) }) != (p_player_1)) as i32)) != 0) {
        break 'f 0i32;
    }
    break 'f ((((c.m.r32((l_q_2).wrapping_add(300)) as i32)) == (1i32)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChange_IsPendingType0", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChange_IsPendingType1(c: &mut Ctx, mut p_player_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChange_IsPendingType1", &PROBE_N, &[(p_player_1) as u32], false); }
    let mut l_q_2: u32 = 0;
    let mut l_type_3: i32 = 0;
    let r_ = 'f: {
    if (((((c.m.r32(sym::gBtlChars)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    l_q_2 = (c.m.r32(sym::gBtlChars)).wrapping_add(312);
    if (((((c.m.r32((l_q_2).wrapping_add(288))) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    l_type_3 = { let a_ = (c.m.r32((l_q_2).wrapping_add(288))).wrapping_add(4); (c.m.r32(a_) as i32) };
    if (((((l_type_3) != (1i32)) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((({ let a_ = c.m.r32((l_q_2).wrapping_add(288)); (c.m.r32(a_) as i32) }) != (p_player_1)) as i32)) != 0) {
        break 'f 0i32;
    }
    break 'f ((((c.m.r32((l_q_2).wrapping_add(300)) as i32)) == (l_type_3)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChange_IsPendingType1", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChange_GetArgs(c: &mut Ctx, mut p_a_1: u32, mut p_b_2: u32, mut p_c_3: u32, mut p_d_4: u32, mut p_e_5: u32, mut p_f_6: u32, mut p_g_7: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChange_GetArgs", &PROBE_N, &[(p_a_1) as u32, (p_b_2) as u32, (p_c_3) as u32, (p_d_4) as u32, (p_e_5) as u32, (p_f_6) as u32, (p_g_7) as u32], false); }
    let mut l_q_8: u32 = 0;
    let r_ = 'f: {
    l_q_8 = (c.m.r32(sym::gBtlChars)).wrapping_add(312);
    if (((((c.m.r32((l_q_8).wrapping_add(288))) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    if (((((p_a_1) != (((0i32) as u32))) as i32)) != 0) {
        { let v_ = { let a_ = ((c.m.r32((l_q_8).wrapping_add(288))).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = p_a_1; c.m.w32(a_, (v_) as u32); };
    }
    if (((((p_b_2) != (((0i32) as u32))) as i32)) != 0) {
        { let v_ = { let a_ = ((c.m.r32((l_q_8).wrapping_add(288))).wrapping_add(8)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = p_b_2; c.m.w32(a_, (v_) as u32); };
    }
    if (((((p_c_3) != (((0i32) as u32))) as i32)) != 0) {
        { let v_ = { let a_ = ((c.m.r32((l_q_8).wrapping_add(288))).wrapping_add(8)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = p_c_3; c.m.w32(a_, (v_) as u32); };
    }
    if (((((p_d_4) != (((0i32) as u32))) as i32)) != 0) {
        { let v_ = { let a_ = ((c.m.r32((l_q_8).wrapping_add(288))).wrapping_add(8)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = p_d_4; c.m.w32(a_, (v_) as u32); };
    }
    if (((((p_e_5) != (((0i32) as u32))) as i32)) != 0) {
        { let v_ = { let a_ = ((c.m.r32((l_q_8).wrapping_add(288))).wrapping_add(8)).wrapping_add((((4i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = p_e_5; c.m.w32(a_, (v_) as u32); };
    }
    if (((((p_f_6) != (((0i32) as u32))) as i32)) != 0) {
        { let v_ = { let a_ = ((c.m.r32((l_q_8).wrapping_add(288))).wrapping_add(8)).wrapping_add((((5i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = p_f_6; c.m.w32(a_, (v_) as u32); };
    }
    if (((((p_g_7) != (((0i32) as u32))) as i32)) != 0) {
        { let v_ = { let a_ = ((c.m.r32((l_q_8).wrapping_add(288))).wrapping_add(8)).wrapping_add((((6i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = p_g_7; c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChange_GetArgs", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChange_NotifyTaken(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChange_NotifyTaken", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::BtlChange_SetTaken(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChange_NotifyTaken", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChange_NotifyLoaded(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChange_NotifyLoaded", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::BtlChange_SetLoaded(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChange_NotifyLoaded", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChange_IsReady(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChange_IsReady", &PROBE_N, &[], false); }
    let mut l_q_1: u32 = 0;
    let r_ = 'f: {
    l_q_1 = (c.m.r32(sym::gBtlChars)).wrapping_add(312);
    if (((((c.m.r32((l_q_1).wrapping_add(288))) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if (((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0) {
        break 'f 0i32;
    }
    break 'f ((((c.m.r32((l_q_1).wrapping_add(300)) as i32)) == (4i32)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChange_IsReady", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCtrl_AttachPartner(c: &mut Ctx, mut p_player_1: i32, mut p_slot_2: i32, mut p_objId_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_AttachPartner", &PROBE_N, &[(p_player_1) as u32, (p_slot_2) as u32, (p_objId_3) as u32], false); }
    let mut l_chr_4: u32 = 0;
    let r_ = 'f: {
    l_chr_4 = F::BtlChar_Get(c, p_player_1);
    if (((((l_chr_4) != (((0i32) as u32))) as i32)) != 0) {
        F::BtlPartner_Attach(c, l_chr_4, p_slot_2, p_objId_3);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_AttachPartner", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_GetButtonIcon(c: &mut Ctx, mut p_chr_1: u32, mut p_bit_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetButtonIcon", &PROBE_N, &[(p_chr_1) as u32, (p_bit_2) as u32], false); }
    let mut l_mask_3: u32 = 0;
    let r_ = 'f: {
    's1: {
        let sv4: u32 = p_bit_2;
        match sv4 {
            4u32 => {
                break 'f 16i32;
            }
            5u32 => {
                break 'f 17i32;
            }
            6u32 => {
                break 'f 18i32;
            }
            7u32 => {
                break 'f 19i32;
            }
            _ => {
            }
        }
    }
    l_mask_3 = (p_chr_1).wrapping_add(1400);
    { let r_ = p_bit_2; let n_ = (l_mask_3).wrapping_add((r_).wrapping_mul(4)); l_mask_3 = n_; };
    's2: {
        let sv5: u32 = c.m.r32(l_mask_3);
        match sv5 {
            4096u32 => {
                break 'f 0i32;
            }
            8192u32 => {
                break 'f 2i32;
            }
            16384u32 => {
                break 'f 3i32;
            }
            32768u32 => {
                break 'f 1i32;
            }
            _ => {
            }
        }
    }
    break 'f (1i32).wrapping_neg();
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetButtonIcon", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSide_GetHp(c: &mut Ctx, mut p_side_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSide_GetHp", &PROBE_N, &[(p_side_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindBySide(c, p_side_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f { let a_ = F::BtlMember_GetActiveGauge(c, l_chr_2); (c.m.r32(a_) as i32) };
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSide_GetHp", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSide_GetKi(c: &mut Ctx, mut p_side_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSide_GetKi", &PROBE_N, &[(p_side_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindBySide(c, p_side_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f { let a_ = (F::BtlMember_GetActiveGauge(c, l_chr_2)).wrapping_add(12); (c.m.r32(a_) as i32) };
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSide_GetKi", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSide_GetBlast(c: &mut Ctx, mut p_side_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSide_GetBlast", &PROBE_N, &[(p_side_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindBySide(c, p_side_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f { let a_ = (F::BtlMember_GetActiveGauge(c, l_chr_2)).wrapping_add(20); (c.m.r32(a_) as i32) };
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSide_GetBlast", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSide_GetBlastMax(c: &mut Ctx, mut p_side_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSide_GetBlastMax", &PROBE_N, &[(p_side_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindBySide(c, p_side_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f { let a_ = (F::BtlMember_GetActiveGauge(c, l_chr_2)).wrapping_add(24); (c.m.r32(a_) as i32) };
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSide_GetBlastMax", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSide_GetMaxPower(c: &mut Ctx, mut p_side_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSide_GetMaxPower", &PROBE_N, &[(p_side_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindBySide(c, p_side_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f { let a_ = (F::BtlMember_GetActiveGauge(c, l_chr_2)).wrapping_add(28); (c.m.r32(a_) as i32) };
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSide_GetMaxPower", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSide_GetSwitchGauge(c: &mut Ctx, mut p_side_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSide_GetSwitchGauge", &PROBE_N, &[(p_side_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindBySide(c, p_side_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f (c.m.r32((l_chr_2).wrapping_add(2460)) as i32);
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSide_GetSwitchGauge", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSide_GetActiveMember(c: &mut Ctx, mut p_side_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSide_GetActiveMember", &PROBE_N, &[(p_side_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindBySide(c, p_side_1);
    if (((((l_chr_2) == (((0i32) as u32))) as i32)) != 0) {
        break 'f (1i32).wrapping_neg();
    }
    break 'f F::BtlMember_GetActiveIndex(c, l_chr_2);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSide_GetActiveMember", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSide_GetSwitchTarget(c: &mut Ctx, mut p_side_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSide_GetSwitchTarget", &PROBE_N, &[(p_side_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindBySide(c, p_side_1);
    if (((((l_chr_2) == (((0i32) as u32))) as i32)) != 0) {
        break 'f (1i32).wrapping_neg();
    }
    if (((((F::BtlMember_CountAlive(c, l_chr_2)) < (2i32)) as i32)) != 0) {
        break 'f (1i32).wrapping_neg();
    }
    break 'f F::BtlMember_GetSwitchTarget(c, l_chr_2);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSide_GetSwitchTarget", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSide_CountAlive(c: &mut Ctx, mut p_side_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSide_CountAlive", &PROBE_N, &[(p_side_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindBySide(c, p_side_1);
    if (((((l_chr_2) == (((0i32) as u32))) as i32)) != 0) {
        break 'f (1i32).wrapping_neg();
    }
    break 'f F::BtlMember_CountAlive(c, l_chr_2);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSide_CountAlive", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSide_GetSwitchTargetHp(c: &mut Ctx, mut p_side_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSide_GetSwitchTargetHp", &PROBE_N, &[(p_side_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindBySide(c, p_side_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        if (((((F::BtlMember_CountAlive(c, l_chr_2)) < (2i32)) as i32)) != 0) {
            break 'f 0i32;
        }
        break 'f { let a_ = { let a1_ = F::BtlMember_GetSwitchTarget(c, l_chr_2); F::BtlMember_GetGauge(c, l_chr_2, a1_) }; (c.m.r32(a_) as i32) };
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSide_GetSwitchTargetHp", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSide_GetSwitchTargetHpMax(c: &mut Ctx, mut p_side_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSide_GetSwitchTargetHpMax", &PROBE_N, &[(p_side_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindBySide(c, p_side_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        if (((((F::BtlMember_CountAlive(c, l_chr_2)) < (2i32)) as i32)) != 0) {
            break 'f 0i32;
        }
        break 'f { let a_ = ({ let a1_ = F::BtlMember_GetSwitchTarget(c, l_chr_2); F::BtlMember_GetGauge(c, l_chr_2, a1_) }).wrapping_add(4); (c.m.r32(a_) as i32) };
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSide_GetSwitchTargetHpMax", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSide_GetParamUnk2C(c: &mut Ctx, mut p_side_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSide_GetParamUnk2C", &PROBE_N, &[(p_side_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindBySide(c, p_side_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f F::BtlParam_GetGaugeB(c, l_chr_2);
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSide_GetParamUnk2C", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSide_IsPoweredUp(c: &mut Ctx, mut p_side_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSide_IsPoweredUp", &PROBE_N, &[(p_side_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindBySide(c, p_side_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f F::BtlChar_TestFlag(c, l_chr_2, ((((6i32) as i64 as u64)) as u32));
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSide_IsPoweredUp", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSide_TestFlagBE(c: &mut Ctx, mut p_side_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSide_TestFlagBE", &PROBE_N, &[(p_side_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindBySide(c, p_side_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f F::BtlChar_TestFlag(c, l_chr_2, ((((190i32) as i64 as u64)) as u32));
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSide_TestFlagBE", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCtrl_IsActiveDead(c: &mut Ctx, mut p_player_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_IsActiveDead", &PROBE_N, &[(p_player_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_Get(c, p_player_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f F::BtlChar_IsDead(c, l_chr_2);
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_IsActiveDead", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCtrl_IsTeamDead(c: &mut Ctx, mut p_player_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_IsTeamDead", &PROBE_N, &[(p_player_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let mut l_dead_3: i32 = 0;
    let mut l_i_4: i32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_Get(c, p_player_1);
    l_dead_3 = 1i32;
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        {
            l_i_4 = 0i32;
            'l1: loop {
                if !(((((l_i_4) < ((c.m.r32((l_chr_2).wrapping_add(2456)) as i32))) as i32)) != 0) { break 'l1; }
                'c2: {
                    if ((((({ let a_ = F::BtlMember_GetGauge(c, l_chr_2, l_i_4); (c.m.r32(a_) as i32) }) > (0i32)) as i32)) != 0) {
                        l_dead_3 = 0i32;
                        break 'l1;
                    }
                }
                { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
            }
        }
        break 'f l_dead_3;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_IsTeamDead", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCtrl_TestFlag7(c: &mut Ctx, mut p_player_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_TestFlag7", &PROBE_N, &[(p_player_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_Get(c, p_player_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f F::BtlChar_TestFlag(c, l_chr_2, ((((7i32) as i64 as u64)) as u32));
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCtrl_TestFlag7", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSide_GetComboHits(c: &mut Ctx, mut p_side_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSide_GetComboHits", &PROBE_N, &[(p_side_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindBySide(c, p_side_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f { let a_ = ({ let a0_ = F::BtlOpp_GetPlayer(c, l_chr_2); F::BtlChar_Get(c, a0_) }).wrapping_add(3396); (c.m.r32(a_) as i32) };
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSide_GetComboHits", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSide_GetComboDamage(c: &mut Ctx, mut p_side_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSide_GetComboDamage", &PROBE_N, &[(p_side_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let mut l_opp_3: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindBySide(c, p_side_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        l_opp_3 = { let a0_ = F::BtlOpp_GetPlayer(c, l_chr_2); F::BtlChar_Get(c, a0_) };
        if ((F::BtlChar_TestFlag(c, l_opp_3, ((((237i32) as i64 as u64)) as u32))) != 0) {
            break 'f (1i32).wrapping_neg();
        }
        break 'f (c.m.r32((l_opp_3).wrapping_add(3392)) as i32);
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSide_GetComboDamage", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSide_IsComboShown(c: &mut Ctx, mut p_side_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSide_IsComboShown", &PROBE_N, &[(p_side_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindBySide(c, p_side_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f ((({ let a_ = ({ let a0_ = F::BtlOpp_GetPlayer(c, l_chr_2); F::BtlChar_Get(c, a0_) }).wrapping_add(3400); (c.m.r32(a_) as i32) }) > (0i32)) as i32);
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSide_IsComboShown", &PROBE_N, &[], true); }
    r_
}

pub fn BtlSide_IsComboHitNew(c: &mut Ctx, mut p_side_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlSide_IsComboHitNew", &PROBE_N, &[(p_side_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let mut l_opp_3: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindBySide(c, p_side_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        l_opp_3 = { let a0_ = F::BtlOpp_GetPlayer(c, l_chr_2); F::BtlChar_Get(c, a0_) };
        if ((F::BtlChar_IsFrozen(c, l_opp_3)) != 0) {
            break 'f 0i32;
        }
        if (((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((8192i32) as u64)))) != 0) {
            break 'f 0i32;
        }
        break 'f (c.m.r32((l_opp_3).wrapping_add(3404)) as i32);
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlSide_IsComboHitNew", &PROBE_N, &[], true); }
    r_
}
