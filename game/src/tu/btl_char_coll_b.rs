#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn BtlColl_StartThrow(c: &mut Ctx, mut p_atk_1: u32, mut p_def_2: u32, mut p_slot_3: i32, mut p_techFlags_4: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlColl_StartThrow", &PROBE_N, &[(p_atk_1) as u32, (p_def_2) as u32, (p_slot_3) as u32, (p_techFlags_4) as u32], false); }
    let mut l_pick_5: i32 = 0;
    let mut l_back_6: i32 = 0;
    let mut l_yaw_7: F32 = F32(0);
    let r_ = 'f: {
    l_pick_5 = 0i32;
    if ((((((c.m.r32((p_atk_1).wrapping_add(4016)) as i32)) >= (3i32)) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((F::BtlChar_IsDead(c, p_atk_1)) != 0) {
        break 'f 0i32;
    }
    if ((F::BtlChar_IsDead(c, p_def_2)) != 0) {
        break 'f 0i32;
    }
    if ((((!((F::BtlChar_TestFlag(c, p_atk_1, ((((3i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    F::BtlChar_SetFlag(c, p_atk_1, ((((114i32) as i64 as u64)) as u32));
    { let v_ = F::BtlSuper_GetId(c, p_atk_1, p_slot_3); let a_ = (p_atk_1).wrapping_add(3728); c.m.w32(a_, (v_) as u32); };
    { let v_ = (c.m.r32(p_atk_1) as i32); let a_ = ((p_atk_1).wrapping_add(3728)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
    { let v_ = (c.m.r32(p_def_2) as i32); let a_ = ((p_atk_1).wrapping_add(3728)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
    { let v_ = p_slot_3; let a_ = ((p_atk_1).wrapping_add(3728)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
    { let v_ = F::BtlSuper_GetStepCount(c, p_atk_1, p_slot_3); let a_ = ((p_atk_1).wrapping_add(3728)).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
    { let v_ = F::BtlSuper_GetThrowChara(c, p_atk_1, p_slot_3); let a_ = ((p_atk_1).wrapping_add(3728)).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
    { let v_ = F::BtlSuper_GetThrowCostume(c, p_atk_1, p_slot_3); let a_ = ((p_atk_1).wrapping_add(3728)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
    { let v_ = F::BtlSuper_GetThrowGauge20(c, p_atk_1, p_slot_3); let a_ = ((p_atk_1).wrapping_add(3728)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
    { let v_ = F::BtlSuper_GetUnk226(c, p_atk_1, p_slot_3); let a_ = ((p_atk_1).wrapping_add(3728)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
    { let v_ = F::BtlSuper_GetUnk220(c, p_atk_1, p_slot_3); let a_ = ((p_atk_1).wrapping_add(3728)).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
    { let v_ = F::BtlSuper_GetLastStep(c, p_atk_1, p_slot_3); let a_ = ((p_atk_1).wrapping_add(3728)).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
    F::BtlChar_SetFlag(c, p_atk_1, ((((148i32) as i64 as u64)) as u32));
    { let v_ = { let a_ = (F::BtlChar_GetPos(c, p_atk_1)).wrapping_add(148); F32(c.m.r32(a_)) }; let a_ = ((p_atk_1).wrapping_add(4016)).wrapping_add(32); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x00000000); let a_ = ((p_atk_1).wrapping_add(4016)).wrapping_add(36); c.m.w32(a_, (v_).0); };
    l_back_6 = 0i32;
    if ((((p_techFlags_4) & (((268435456i32) as u32)))) != 0) {
        l_back_6 = 1i32;
    }
    if ((((p_techFlags_4) & (((1024i32) as u32)))) != 0) {
        l_pick_5 = (((F::BtlChar_FrameMod(c, 2i32)) != (0i32)) as i32);
    }
    { let v_ = l_pick_5; let a_ = ((p_atk_1).wrapping_add(3728)).wrapping_add(52); c.m.w32(a_, (v_) as u32); };
    { let v_ = F::BtlSuper_GetUnk14D(c, p_atk_1, p_slot_3); let a_ = ((p_atk_1).wrapping_add(3728)).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
    if ((((p_techFlags_4) & (((4194304i32) as u32)))) != 0) {
        { let v_ = 1i32; let a_ = ((p_atk_1).wrapping_add(3728)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
        { let v_ = F::BtlSuper_GetThrowAngleA(c, p_atk_1, p_slot_3); let a_ = ((p_atk_1).wrapping_add(3728)).wrapping_add(96); c.m.w32(a_, (v_).0); };
        { let v_ = F::BtlSuper_GetThrowAngleB(c, p_atk_1, p_slot_3); let a_ = ((p_atk_1).wrapping_add(3728)).wrapping_add(100); c.m.w32(a_, (v_).0); };
    } else {
        { let v_ = 0i32; let a_ = ((p_atk_1).wrapping_add(3728)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
        { let v_ = F32(0x00000000); let a_ = ((p_atk_1).wrapping_add(3728)).wrapping_add(96); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = ((p_atk_1).wrapping_add(3728)).wrapping_add(100); c.m.w32(a_, (v_).0); };
    }
    { let v_ = 0i32; let a_ = ((p_atk_1).wrapping_add(3728)).wrapping_add(64); c.m.w32(a_, (v_) as u32); };
    if ((((((((F::BtlSuper_GetFlagsA(c, p_atk_1, p_slot_3)) as u64)) as i32)) & (33554432i32))) != 0) {
        { let v_ = 1i32; let a_ = ((p_atk_1).wrapping_add(3728)).wrapping_add(64); c.m.w32(a_, (v_) as u32); };
    }
    { let v_ = 0i32; let a_ = ((p_atk_1).wrapping_add(3728)).wrapping_add(68); c.m.w32(a_, (v_) as u32); };
    { let v_ = 0i32; let a_ = ((p_atk_1).wrapping_add(3728)).wrapping_add(72); c.m.w32(a_, (v_) as u32); };
    if ((((p_techFlags_4) & (((8192i32) as u32)))) != 0) {
        if ((((({ let a_ = (F::BtlMember_GetActiveGauge(c, p_atk_1)).wrapping_add(32); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
            { let v_ = 1i32; let a_ = ((p_atk_1).wrapping_add(3728)).wrapping_add(72); c.m.w32(a_, (v_) as u32); };
        }
    }
    { let v_ = 0i32; let a_ = ((p_atk_1).wrapping_add(3728)).wrapping_add(76); c.m.w32(a_, (v_) as u32); };
    if ((((p_techFlags_4) & (((524288i32) as u32)))) != 0) {
        if ((((({ let a_ = (F::BtlMember_GetActiveGauge(c, p_def_2)).wrapping_add(32); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
            { let v_ = 1i32; let a_ = ((p_atk_1).wrapping_add(3728)).wrapping_add(76); c.m.w32(a_, (v_) as u32); };
        }
    }
    { let v_ = 0i32; let a_ = ((p_atk_1).wrapping_add(3728)).wrapping_add(80); c.m.w32(a_, (v_) as u32); };
    if ((((p_techFlags_4) & (((2048i32) as u32)))) != 0) {
        { let v_ = 1i32; let a_ = ((p_atk_1).wrapping_add(3728)).wrapping_add(80); c.m.w32(a_, (v_) as u32); };
    }
    { let v_ = 0i32; let a_ = ((p_atk_1).wrapping_add(3728)).wrapping_add(84); c.m.w32(a_, (v_) as u32); };
    if ((((p_techFlags_4) & (((65536i32) as u32)))) != 0) {
        { let v_ = 1i32; let a_ = ((p_atk_1).wrapping_add(3728)).wrapping_add(84); c.m.w32(a_, (v_) as u32); };
    }
    { let v_ = 0i32; let a_ = ((p_atk_1).wrapping_add(3728)).wrapping_add(88); c.m.w32(a_, (v_) as u32); };
    if (((({ let a0_ = F::BtlAnim_GetId(c, p_def_2); F::BtlAnim_GetFlags(c, ((((a0_) as i64 as u64)) as u32)) }) & (((2048i32) as u32)))) != 0) {
        { let v_ = 1i32; let a_ = ((p_atk_1).wrapping_add(3728)).wrapping_add(88); c.m.w32(a_, (v_) as u32); };
    }
    { let v_ = 0i32; let a_ = ((p_atk_1).wrapping_add(3728)).wrapping_add(92); c.m.w32(a_, (v_) as u32); };
    if ((((p_techFlags_4) & (((67108864i32) as u32)))) != 0) {
        { let v_ = 1i32; let a_ = ((p_atk_1).wrapping_add(3728)).wrapping_add(92); c.m.w32(a_, (v_) as u32); };
    }
    { let v_ = 34i32; let a_ = (p_def_2).wrapping_add(4016); c.m.w32(a_, (v_) as u32); };
    { let v_ = (c.m.r32((p_atk_1).wrapping_add(3728)) as i32); let a_ = (p_def_2).wrapping_add(3728); c.m.w32(a_, (v_) as u32); };
    { let v_ = (c.m.r32(((p_atk_1).wrapping_add(3728)).wrapping_add(4)) as i32); let a_ = ((p_def_2).wrapping_add(3728)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
    { let v_ = (c.m.r32(((p_atk_1).wrapping_add(3728)).wrapping_add(8)) as i32); let a_ = ((p_def_2).wrapping_add(3728)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
    { let v_ = (c.m.r32(((p_atk_1).wrapping_add(3728)).wrapping_add(12)) as i32); let a_ = ((p_def_2).wrapping_add(3728)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
    { let v_ = (c.m.r32(((p_atk_1).wrapping_add(3728)).wrapping_add(16)) as i32); let a_ = ((p_def_2).wrapping_add(3728)).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
    { let v_ = (1i32).wrapping_neg(); let a_ = ((p_def_2).wrapping_add(3728)).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
    { let v_ = 0i32; let a_ = ((p_def_2).wrapping_add(3728)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
    { let v_ = 0i32; let a_ = ((p_def_2).wrapping_add(3728)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
    { let v_ = 1i32; let a_ = ((p_def_2).wrapping_add(3728)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
    { let v_ = (c.m.r32(((p_atk_1).wrapping_add(3728)).wrapping_add(24)) as i32); let a_ = ((p_def_2).wrapping_add(3728)).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
    { let v_ = (c.m.r32(((p_atk_1).wrapping_add(3728)).wrapping_add(28)) as i32); let a_ = ((p_def_2).wrapping_add(3728)).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
    { let v_ = (c.m.r32(((p_atk_1).wrapping_add(3728)).wrapping_add(20)) as i32); let a_ = ((p_def_2).wrapping_add(3728)).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
    { let v_ = (c.m.r32(((p_atk_1).wrapping_add(3728)).wrapping_add(48)) as i32); let a_ = ((p_def_2).wrapping_add(3728)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
    { let v_ = F32(c.m.r32(((p_atk_1).wrapping_add(3728)).wrapping_add(96))); let a_ = ((p_def_2).wrapping_add(3728)).wrapping_add(96); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32(((p_atk_1).wrapping_add(3728)).wrapping_add(100))); let a_ = ((p_def_2).wrapping_add(3728)).wrapping_add(100); c.m.w32(a_, (v_).0); };
    { let v_ = (c.m.r32(((p_atk_1).wrapping_add(3728)).wrapping_add(52)) as i32); let a_ = ((p_def_2).wrapping_add(3728)).wrapping_add(52); c.m.w32(a_, (v_) as u32); };
    { let v_ = (c.m.r32(((p_atk_1).wrapping_add(3728)).wrapping_add(64)) as i32); let a_ = ((p_def_2).wrapping_add(3728)).wrapping_add(64); c.m.w32(a_, (v_) as u32); };
    { let v_ = (c.m.r32(((p_atk_1).wrapping_add(3728)).wrapping_add(68)) as i32); let a_ = ((p_def_2).wrapping_add(3728)).wrapping_add(68); c.m.w32(a_, (v_) as u32); };
    { let v_ = (c.m.r32(((p_atk_1).wrapping_add(3728)).wrapping_add(72)) as i32); let a_ = ((p_def_2).wrapping_add(3728)).wrapping_add(72); c.m.w32(a_, (v_) as u32); };
    { let v_ = (c.m.r32(((p_atk_1).wrapping_add(3728)).wrapping_add(76)) as i32); let a_ = ((p_def_2).wrapping_add(3728)).wrapping_add(76); c.m.w32(a_, (v_) as u32); };
    { let v_ = (c.m.r32(((p_atk_1).wrapping_add(3728)).wrapping_add(80)) as i32); let a_ = ((p_def_2).wrapping_add(3728)).wrapping_add(80); c.m.w32(a_, (v_) as u32); };
    { let v_ = (c.m.r32(((p_atk_1).wrapping_add(3728)).wrapping_add(84)) as i32); let a_ = ((p_def_2).wrapping_add(3728)).wrapping_add(84); c.m.w32(a_, (v_) as u32); };
    { let v_ = (c.m.r32(((p_atk_1).wrapping_add(3728)).wrapping_add(88)) as i32); let a_ = ((p_def_2).wrapping_add(3728)).wrapping_add(88); c.m.w32(a_, (v_) as u32); };
    { let v_ = (c.m.r32(((p_atk_1).wrapping_add(3728)).wrapping_add(92)) as i32); let a_ = ((p_def_2).wrapping_add(3728)).wrapping_add(92); c.m.w32(a_, (v_) as u32); };
    F::BtlChar_SetFlag(c, p_def_2, ((((148i32) as i64 as u64)) as u32));
    l_yaw_7 = F32(c.m.r32(((p_atk_1).wrapping_add(4016)).wrapping_add(32)));
    { let v_ = l_yaw_7; let a_ = ((p_def_2).wrapping_add(4016)).wrapping_add(32); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32(((p_atk_1).wrapping_add(4016)).wrapping_add(36))); let a_ = ((p_def_2).wrapping_add(4016)).wrapping_add(36); c.m.w32(a_, (v_).0); };
    { let v_ = l_yaw_7; let a_ = ((p_def_2).wrapping_add(4016)).wrapping_add(28); c.m.w32(a_, (v_).0); };
    { let v_ = l_back_6; let a_ = ((p_def_2).wrapping_add(4016)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
    F::BtlChar_SetFlag(c, p_atk_1, ((((68i32) as i64 as u64)) as u32));
    F::BtlChar_SetFlag(c, p_atk_1, ((((69i32) as i64 as u64)) as u32));
    F::BtlChar_SetFlag(c, p_atk_1, ((((70i32) as i64 as u64)) as u32));
    F::BtlChar_SetFlag(c, p_def_2, ((((68i32) as i64 as u64)) as u32));
    F::BtlChar_SetFlag(c, p_def_2, ((((69i32) as i64 as u64)) as u32));
    F::BtlChar_SetFlag(c, p_def_2, ((((70i32) as i64 as u64)) as u32));
    F::BtlChar_RaiseFirstClash(c, p_atk_1);
    F::BtlChar_SetFlag(c, p_atk_1, ((((296i32) as i64 as u64)) as u32));
    's1: {
        let sv8: i32 = p_slot_3;
        match sv8 {
            2i32 => {
                { let a0_ = (c.m.r32(p_atk_1) as i32); F::BtlEvent_Raise(c, a0_, 43i32) };
                break 's1;
            }
            3i32 => {
                { let a0_ = (c.m.r32(p_atk_1) as i32); F::BtlEvent_Raise(c, a0_, 44i32) };
                break 's1;
            }
            4i32 => {
                { let a0_ = (c.m.r32(p_atk_1) as i32); F::BtlEvent_Raise(c, a0_, 45i32) };
                break 's1;
            }
            _ => {
            }
        }
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlColl_StartThrow", &PROBE_N, &[], true); }
    r_
}

pub fn BtlColl_IsFacing(c: &mut Ctx, mut p_chr_1: u32, mut p_yaw_2: F32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlColl_IsFacing", &PROBE_N, &[(p_chr_1) as u32, (p_yaw_2).0], false); }
    let r_ = 'f: {
    if (((rt::flt({ let a0_ = { let a0_ = rt::fsub({ let a_ = ((F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(16)).wrapping_add(4); F32(c.m.r32(a_)) }, p_yaw_2); F::BtlUtil_WrapAngle(c, a0_) }; rt::ffabs(a0_) }, F32(0x3fc90fda)) as i32)) != 0) {
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlColl_IsFacing", &PROBE_N, &[], true); }
    r_
}

pub fn BtlColl_GetHitDir(c: &mut Ctx, mut p_chr_1: u32, mut p_hit_2: u32, mut p_outDir_3: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlColl_GetHitDir", &PROBE_N, &[(p_chr_1) as u32, (p_hit_2) as u32, (p_outDir_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_fromPos_4: i32 = 0;
    let mut l_fromAtk_5: i32 = 0;
    let mut l_atk_6: u32 = 0;
    let mut l_slot_7: i32 = 0;
    let mut l_yaw_8: F32 = F32(0);
    let r_ = 'f: {
        l_fromPos_4 = 0i32;
        l_fromAtk_5 = 0i32;
        if ((((((((((c.m.r32((p_hit_2).wrapping_add(84)) as i32)) != (0i32)) as i32)) != 0) || (((rt::flt(F::Vec3_Length(c, (p_hit_2).wrapping_add(64)), F32(0x3c23d70a)) as i32)) != 0)) as i32)) != 0) {
            l_fromPos_4 = 1i32;
        }
        if ((((((((((c.m.r32((p_hit_2).wrapping_add(12)) as i32)) == (1i32)) as i32)) != 0) && (((((c.m.r32((p_hit_2).wrapping_add(100))) != (((0i32) as u32))) as i32)) != 0)) as i32)) != 0) {
            l_slot_7 = { let a_ = (c.m.r32((p_hit_2).wrapping_add(100))).wrapping_add(4); (c.m.r32(a_) as i32) };
            l_atk_6 = { let a0_ = { let a_ = c.m.r32((p_hit_2).wrapping_add(100)); (c.m.r32(a_) as i32) }; F::BtlChar_Get(c, a0_) };
            's1: {
                let sv9: i32 = l_slot_7;
                match sv9 {
                    0i32..=1i32 => {
                        l_fromPos_4 = 0i32;
                        l_fromAtk_5 = 1i32;
                        break 's1;
                    }
                    2i32..=4i32 => {
                        if (((((F::BtlSuper_GetHitDirKind(c, l_atk_6, l_slot_7)) == (3i32)) as i32)) != 0) {
                            l_fromPos_4 = 0i32;
                            l_fromAtk_5 = 1i32;
                        }
                        break 's1;
                    }
                    _ => {
                    }
                }
            }
        }
        if ((l_fromPos_4) != 0) {
            { let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); F::BtlCharApi_GetNodePos(c, a0_, 3i32, fp.wrapping_add(16)) };
            F::Vec4_Sub(c, fp, fp.wrapping_add(16), (p_hit_2).wrapping_add(32));
        } else {
            if ((l_fromAtk_5) != 0) {
                F::BtlOpp_GetDelta(c, p_chr_1, fp);
                { let r_ = F::BtlOpp_GetHalfHeightDiff(c, p_chr_1); let a_ = (fp).wrapping_add(4); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                F::Vec4_Sub(c, fp, sym::D_002EC2A0, fp);
            } else {
                F::Vec4_Copy(c, fp, (p_hit_2).wrapping_add(64));
            }
        }
        if (((((((rt::fgt({ let a0_ = F32(c.m.r32(fp)); rt::ffabs(a0_) }, F32(0x3a83126e)) as i32)) != 0) || (((rt::fgt({ let a0_ = F32(c.m.r32((fp).wrapping_add(8))); rt::ffabs(a0_) }, F32(0x3a83126e)) as i32)) != 0)) as i32)) != 0) {
            F::Vec3_Normalize(c, fp, fp);
            l_yaw_8 = { let a0_ = F32(c.m.r32(fp)); let a1_ = F32(c.m.r32((fp).wrapping_add(8))); F::Port_atan2f(c, a0_, a1_) };
        } else {
            if (((rt::fgt({ let a0_ = F32(c.m.r32((fp).wrapping_add(4))); rt::ffabs(a0_) }, F32(0x3a83126e)) as i32)) != 0) {
                F::Vec3_Normalize(c, fp, fp);
            }
            l_yaw_8 = F32(0x00000000);
        }
        if (((((p_outDir_3) != (((0i32) as u32))) as i32)) != 0) {
            F::Vec4_Copy(c, p_outDir_3, fp);
        }
        break 'f l_yaw_8;
       F32(0)
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlColl_GetHitDir", &PROBE_N, &[], true); }
    r_
}

pub fn BtlColl_PlayHitSound(c: &mut Ctx, mut p_chr_1: u32, mut p_hit_2: u32, mut p_kind_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlColl_PlayHitSound", &PROBE_N, &[(p_chr_1) as u32, (p_hit_2) as u32, (p_kind_3) as u32], false); }
    let r_ = 'f: {
    's1: {
        let sv4: u32 = p_kind_3;
        match sv4 {
            0u32 => {
                if (((((p_chr_1) != (((0i32) as u32))) as i32)) != 0) {
                    F::BtlChar_PlayVoice(c, p_chr_1, 0i32);
                }
                break 's1;
            }
            1u32 => {
                if (((((p_hit_2) != (((0i32) as u32))) as i32)) != 0) {
                    { let a2_ = (if ((F::BtlChar_FrameMod(c, 2i32)) != 0) { 27i32 } else { 17i32 }); F::BtlCharSnd_RequestAt(c, (p_hit_2).wrapping_add(32), 0i32, a2_, F32(0x43480000), F32(0x44bb8000)) };
                }
                if (((((p_chr_1) != (((0i32) as u32))) as i32)) != 0) {
                    F::BtlChar_PlayVoice(c, p_chr_1, 0i32);
                }
                break 's1;
            }
            2u32 => {
                if (((((p_hit_2) != (((0i32) as u32))) as i32)) != 0) {
                    F::BtlCharSnd_RequestAt(c, (p_hit_2).wrapping_add(32), 0i32, 71i32, F32(0x43480000), F32(0x44bb8000));
                }
                if (((((p_chr_1) != (((0i32) as u32))) as i32)) != 0) {
                    F::BtlChar_PlayVoice(c, p_chr_1, 1i32);
                }
                break 's1;
            }
            3u32 => {
                if (((((p_hit_2) != (((0i32) as u32))) as i32)) != 0) {
                    F::BtlCharSnd_RequestAt(c, (p_hit_2).wrapping_add(32), 0i32, 74i32, F32(0x43480000), F32(0x44bb8000));
                }
                if (((((p_chr_1) != (((0i32) as u32))) as i32)) != 0) {
                    F::BtlChar_PlayVoice(c, p_chr_1, 3i32);
                }
                break 's1;
            }
            4u32 => {
                if (((((p_hit_2) != (((0i32) as u32))) as i32)) != 0) {
                    F::BtlCharSnd_RequestAt(c, (p_hit_2).wrapping_add(32), 0i32, 18i32, F32(0x43480000), F32(0x44bb8000));
                }
                if (((((p_chr_1) != (((0i32) as u32))) as i32)) != 0) {
                    F::BtlChar_PlayVoice(c, p_chr_1, 3i32);
                }
                break 's1;
            }
            _ => {
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlColl_PlayHitSound", &PROBE_N, &[], true); }
    r_
}

pub fn BtlColl_GuardBlast(c: &mut Ctx, mut p_chr_1: u32, mut p_hit_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlColl_GuardBlast", &PROBE_N, &[(p_chr_1) as u32, (p_hit_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_kind_3: i32 = 0;
    let mut l_yaw_4: F32 = F32(0);
    let mut l_dmg_5: i32 = 0;
    let mut l_time_6: F32 = F32(0);
    let r_ = 'f: {
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((68i32) as i64 as u64)) as u32))) != 0) {
            break 'f 0i32;
        }
        l_kind_3 = F::BtlColl_GetGuardKind(c, p_chr_1);
        l_yaw_4 = F::BtlColl_GetHitDir(c, p_chr_1, p_hit_2, fp);
        if (((((l_kind_3) != (6i32)) as i32)) != 0) {
            if ((((F::BtlKiBlast_GetFlagsOfHit(c, p_hit_2)) & (1i32))) != 0) {
                break 'f 0i32;
            }
            if ((F::BtlColl_IsFacing(c, p_chr_1, l_yaw_4)) != 0) {
                break 'f 0i32;
            }
        }
        l_dmg_5 = F::BtlKiBlast_GetGuardDamageOfHit(c, p_hit_2);
        if ((((((({ let a_ = (c.m.r32((p_hit_2).wrapping_add(104))).wrapping_add(65); c.m.r8(a_) }) as i32)) >= (2i32)) as i32)) != 0) {
            { let r_ = (({ let a_ = (c.m.r32((p_hit_2).wrapping_add(104))).wrapping_add(65); c.m.r8(a_) }) as i32); let n_ = rt::div_i32(l_dmg_5, r_); l_dmg_5 = n_; };
        }
        if (((((((((l_kind_3) == (0i32)) as i32)) != 0) || (((((l_kind_3) == (5i32)) as i32)) != 0)) as i32)) != 0) {
            break 'f 0i32;
        }
        F::BtlChar_SetFlag(c, p_chr_1, ((((102i32) as i64 as u64)) as u32));
        { let _ = F::BtlMember_Damage(c, p_chr_1, l_dmg_5, 12i32); };
        's1: {
            let sv7: i32 = F::BtlKiBlast_GetTypeOfHit(c, p_hit_2);
            match sv7 {
                2i32 => {
                    F::BtlCharSnd_RequestAt(c, (p_hit_2).wrapping_add(32), 0i32, 66i32, F32(0x43480000), F32(0x44bb8000));
                    break 's1;
                }
                3i32 => {
                    { let a2_ = (if ((F::BtlChar_FrameMod(c, 2i32)) != 0) { 12i32 } else { 13i32 }); F::BtlCharSnd_RequestAt(c, (p_hit_2).wrapping_add(32), 0i32, a2_, F32(0x43480000), F32(0x44bb8000)) };
                    break 's1;
                }
                _ => {
                    { let a2_ = (if ((F::BtlChar_FrameMod(c, 2i32)) != 0) { 27i32 } else { 17i32 }); F::BtlCharSnd_RequestAt(c, (p_hit_2).wrapping_add(32), 0i32, a2_, F32(0x43480000), F32(0x44bb8000)) };
                    break 's1;
                }
            }
        }
        l_time_6 = F32(0x3dcccccc);
        { let a2_ = F::BtlKiBlast_GetGuardPushOfHit(c, p_hit_2); F::BtlMove_SetImpulseDir(c, p_chr_1, fp, a2_) };
        F::ChrCam_AddShake(c, p_chr_1, F32(0x40400000), l_time_6);
        F::BtlChar_Vibrate(c, p_chr_1, F32(0x3f800000), l_time_6);
        break 'f 1i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlColl_GuardBlast", &PROBE_N, &[], true); }
    r_
}

pub fn BtlColl_GuardStrike(c: &mut Ctx, mut p_chr_1: u32, mut p_hit_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlColl_GuardStrike", &PROBE_N, &[(p_chr_1) as u32, (p_hit_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_kind_3: i32 = 0;
    let mut l_atk_4: u32 = 0;
    let mut l_slot_5: i32 = 0;
    let mut l_dmg_6: i32 = 0;
    let mut l_yaw_7: F32 = F32(0);
    let mut l_time_8: F32 = F32(0);
    let r_ = 'f: {
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((69i32) as i64 as u64)) as u32))) != 0) {
            break 'f 0i32;
        }
        if (((((c.m.r32((p_hit_2).wrapping_add(100))) == (((0i32) as u32))) as i32)) != 0) {
            break 'f 0i32;
        }
        l_kind_3 = F::BtlColl_GetGuardKind(c, p_chr_1);
        l_yaw_7 = F::BtlColl_GetHitDir(c, p_chr_1, p_hit_2, fp);
        if (((((l_kind_3) != (6i32)) as i32)) != 0) {
            if ((F::BtlColl_IsFacing(c, p_chr_1, l_yaw_7)) != 0) {
                break 'f 0i32;
            }
        }
        l_slot_5 = { let a_ = (c.m.r32((p_hit_2).wrapping_add(100))).wrapping_add(4); (c.m.r32(a_) as i32) };
        l_atk_4 = { let a0_ = { let a_ = c.m.r32((p_hit_2).wrapping_add(100)); (c.m.r32(a_) as i32) }; F::BtlChar_Get(c, a0_) };
        if ((((F::BtlSkill_GetFlags(c, l_atk_4, l_slot_5)) & (((2097152i32) as u32)))) != 0) {
            break 'f 0i32;
        }
        l_dmg_6 = F::BtlSkill_GetGuardDamage(c, l_atk_4, l_slot_5, 0i32);
        if (((((((((l_kind_3) == (0i32)) as i32)) != 0) || (((((l_kind_3) == (5i32)) as i32)) != 0)) as i32)) != 0) {
            break 'f 0i32;
        }
        F::BtlChar_SetFlag(c, p_chr_1, ((((102i32) as i64 as u64)) as u32));
        { let _ = F::BtlMember_Damage(c, p_chr_1, l_dmg_6, 12i32); };
        l_time_8 = F32(0x3e4ccccc);
        { let a2_ = ((F::BtlSkill_GetHitSound(c, l_atk_4, l_slot_5)) as u32); F::BtlColl_PlayHitSound(c, ((0i32) as u32), p_hit_2, a2_) };
        { let a2_ = F::BtlSkill_GetGuardPush(c, l_atk_4, l_slot_5); F::BtlMove_SetImpulseDir(c, p_chr_1, fp, a2_) };
        F::ChrCam_AddShake(c, p_chr_1, F32(0x40400000), l_time_8);
        F::BtlChar_Vibrate(c, p_chr_1, F32(0x3f800000), l_time_8);
        break 'f 1i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlColl_GuardStrike", &PROBE_N, &[], true); }
    r_
}

pub fn BtlColl_GuardRush(c: &mut Ctx, mut p_chr_1: u32, mut p_hit_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlColl_GuardRush", &PROBE_N, &[(p_chr_1) as u32, (p_hit_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_weak_3: i32 = 0;
    let mut l_atk_4: u32 = 0;
    let mut l_slot_5: i32 = 0;
    let mut l_kind_6: i32 = 0;
    let mut l_alt_7: i32 = 0;
    let mut l_n_8: i32 = 0;
    let mut l_yaw_9: F32 = F32(0);
    let mut l_time_10: F32 = F32(0);
    let r_ = 'f: {
        l_weak_3 = 0i32;
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((70i32) as i64 as u64)) as u32))) != 0) {
            break 'f 0i32;
        }
        if (((((c.m.r32((p_hit_2).wrapping_add(100))) == (((0i32) as u32))) as i32)) != 0) {
            break 'f 0i32;
        }
        l_slot_5 = { let a_ = (c.m.r32((p_hit_2).wrapping_add(100))).wrapping_add(4); (c.m.r32(a_) as i32) };
        l_atk_4 = { let a0_ = { let a_ = c.m.r32((p_hit_2).wrapping_add(100)); (c.m.r32(a_) as i32) }; F::BtlChar_Get(c, a0_) };
        l_kind_6 = F::BtlColl_GetGuardKind(c, p_chr_1);
        l_alt_7 = ((((c.m.r32((p_hit_2).wrapping_add(80)) as i32)).wrapping_shr((4i32) as u32)) & (1i32));
        's1: {
            let sv11: i32 = F::BtlSuper_GetReact(c, l_atk_4, l_slot_5);
            match sv11 {
                29i32 | 30i32 | 34i32 => {
                    if (((((((((F::BtlParam_GetUnk2(c, p_chr_1)) == (4i32)) as i32)) != 0) && ((((!((((F::BtlSuper_GetFlags(c, l_atk_4, l_slot_5)) & (((134217728i32) as u32)))) != 0)) as i32)) != 0)) as i32)) != 0) {
                        { let a2_ = ((F::BtlSuper_GetHitSound(c, l_atk_4, l_slot_5, l_alt_7)) as u32); F::BtlColl_PlayHitSound(c, ((0i32) as u32), p_hit_2, a2_) };
                        break 'f 1i32;
                    }
                    l_weak_3 = 1i32;
                    break 's1;
                }
                _ => {
                }
            }
        }
        if (((((F::BtlSuper_GetId(c, l_atk_4, l_slot_5)) == (725i32)) as i32)) != 0) {
            if (((rt::flt(F::BtlParam_GetUnkC7Scale(c, p_chr_1), F32(0x3a83126e)) as i32)) != 0) {
                { let a2_ = ((F::BtlSuper_GetHitSound(c, l_atk_4, l_slot_5, l_alt_7)) as u32); F::BtlColl_PlayHitSound(c, ((0i32) as u32), p_hit_2, a2_) };
                break 'f 1i32;
            }
        }
        l_yaw_9 = F::BtlColl_GetHitDir(c, p_chr_1, p_hit_2, fp);
        if (((((l_kind_6) != (6i32)) as i32)) != 0) {
            if ((F::BtlColl_IsFacing(c, p_chr_1, l_yaw_9)) != 0) {
                break 'f 0i32;
            }
        }
        if ((((F::BtlSuper_GetFlags(c, l_atk_4, l_slot_5)) & (((1i32) as u32)))) != 0) {
            if ((((((l_weak_3) != 0) || (((((l_kind_6) != (6i32)) as i32)) != 0)) as i32)) != 0) {
                break 'f 0i32;
            }
        }
        if ((((((((((((F::BtlSuper_GetType(c, l_atk_4, l_slot_5)) & (2i32))) != 0) && (((rt::fge(F32(c.m.r32((l_atk_4).wrapping_add(3652))), F32(0x3f7ff972)) as i32)) != 0)) as i32)) != 0) && ((((F::BtlSuper_GetFlags(c, l_atk_4, l_slot_5)) & (((2i32) as u32)))) != 0)) as i32)) != 0) {
            if (((((l_kind_6) != (6i32)) as i32)) != 0) {
                break 'f 0i32;
            }
        }
        if (((((((((l_kind_6) == (0i32)) as i32)) != 0) || (((((l_kind_6) == (5i32)) as i32)) != 0)) as i32)) != 0) {
            break 'f 0i32;
        }
        's2: {
            let sv12: i32 = F::BtlSuper_GetGuardKind(c, l_atk_4, l_slot_5);
            match sv12 {
                0i32 => {
                    F::BtlChar_SetFlag(c, p_chr_1, ((((102i32) as i64 as u64)) as u32));
                    break 's2;
                }
                1i32 => {
                    F::BtlChar_SetFlag(c, p_chr_1, ((((103i32) as i64 as u64)) as u32));
                    break 's2;
                }
                _ => {
                }
            }
        }
        { let _ = { let a1_ = F::BtlSuper_GetDamage(c, l_atk_4, l_slot_5, 1i32, 0i32); F::BtlMember_Damage(c, p_chr_1, a1_, 12i32) }; };
        { let a2_ = ((F::BtlSuper_GetHitSound(c, l_atk_4, l_slot_5, l_alt_7)) as u32); F::BtlColl_PlayHitSound(c, ((0i32) as u32), p_hit_2, a2_) };
        if ((l_alt_7) != 0) {
            { let a2_ = F::BtlSuper_GetGuardPush(c, l_atk_4, l_slot_5); F::BtlMove_SetImpulseDir(c, p_chr_1, fp, a2_) };
        }
        l_time_10 = F32(0x3e999999);
        F::ChrCam_AddShake(c, p_chr_1, F32(0x40a00000), l_time_10);
        F::BtlChar_Vibrate(c, p_chr_1, F32(0x3f800000), l_time_10);
        l_n_8 = F::BtlSuper_GetRecoilDamage(c, l_atk_4, l_slot_5);
        if (((((l_n_8) > (0i32)) as i32)) != 0) {
            if ((F::BtlMember_Damage(c, l_atk_4, l_n_8, 1067i32)) != 0) {
                F::BtlChar_SetFlag(c, l_atk_4, ((((149i32) as i64 as u64)) as u32));
            }
        }
        's3: {
            let sv13: i32 = l_slot_5;
            match sv13 {
                2i32 => {
                    { let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlEvent_Raise(c, a0_, 46i32) };
                    break 's3;
                }
                3i32 => {
                    { let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlEvent_Raise(c, a0_, 47i32) };
                    break 's3;
                }
                4i32 => {
                    { let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlEvent_Raise(c, a0_, 48i32) };
                    break 's3;
                }
                _ => {
                }
            }
        }
        break 'f 1i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlColl_GuardRush", &PROBE_N, &[], true); }
    r_
}

pub fn BtlColl_HitByBlast(c: &mut Ctx, mut p_chr_1: u32, mut p_hit_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlColl_HitByBlast", &PROBE_N, &[(p_chr_1) as u32, (p_hit_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_obj_3: u32 = 0;
    let mut l_stFlags_4: u32 = 0;
    let mut l_atk_5: u32 = 0;
    let mut l_yaw_6: F32 = F32(0);
    let mut l_front_7: i32 = 0;
    let mut l_react_8: i32 = 0;
    let mut l_n_9: i32 = 0;
    let mut l_dmg_10: i32 = 0;
    let mut l_time_11: F32 = F32(0);
    let mut l_g_12: u32 = 0;
    let mut l_r_13: u32 = 0;
    let r_ = 'f: {
        l_obj_3 = F::BtlChar_GetObj(c, p_chr_1);
        if (((((c.m.r32((p_hit_2).wrapping_add(104))) == (((0i32) as u32))) as i32)) != 0) {
            break 'f 0i32;
        }
        l_stFlags_4 = { let a0_ = F::BtlAnim_GetId(c, p_chr_1); F::BtlAnim_GetFlags(c, ((((a0_) as i64 as u64)) as u32)) };
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((68i32) as i64 as u64)) as u32))) != 0) {
            break 'f 0i32;
        }
        if (({ let a0_ = F::BtlAct_GetCurrent(c, p_chr_1); F::BtlAct_IsTechniqueId(c, a0_) }) != 0) {
            break 'f 1i32;
        }
        l_atk_5 = { let a0_ = (({ let a_ = (c.m.r32((p_hit_2).wrapping_add(104))).wrapping_add(16); (c.m.r16(a_) as i16) }) as i32); F::BtlChar_Get(c, a0_) };
        l_yaw_6 = F::BtlColl_GetHitDir(c, p_chr_1, p_hit_2, fp);
        l_front_7 = F::BtlColl_IsFacing(c, p_chr_1, l_yaw_6);
        if ((((((c.m.r32((p_chr_1).wrapping_add(3608)) as i32)) > (0i32)) as i32)) != 0) {
            l_react_8 = 2i32;
        } else {
            if ((((((((l_stFlags_4) & (((16384i32) as u32)))) != 0) || (((((F::BtlAct_GetCurrent(c, p_chr_1)) == (223i32)) as i32)) != 0)) as i32)) != 0) {
                l_react_8 = F::BtlKiBlast_GetReactS4000OfHit(c, p_hit_2);
            } else {
                if ((((l_stFlags_4) & (((2048i32) as u32)))) != 0) {
                    l_react_8 = F::BtlKiBlast_GetReactS800OfHit(c, p_hit_2);
                } else {
                    if ((((l_stFlags_4) & (((512i32) as u32)))) != 0) {
                        l_react_8 = F::BtlKiBlast_GetReactS200OfHit(c, p_hit_2);
                    } else {
                        l_n_9 = 0i32;
                        if ((((((((F::BtlParam_GetFlags(c, p_chr_1)) as u64)) as i32)) & (4i32))) != 0) {
                            l_n_9 = 1i32;
                        }
                        if ((((((((F::BtlParam_GetFlags(c, p_chr_1)) as u64)) as i32)) & (8i32))) != 0) {
                            { let o_ = l_n_9; let n_ = (o_).wrapping_sub(1); l_n_9 = n_; };
                        }
                        if ((((((((F::BtlParam_GetFlags(c, l_atk_5)) as u64)) as i32)) & (4i32))) != 0) {
                            { let o_ = l_n_9; let n_ = (o_).wrapping_sub(1); l_n_9 = n_; };
                        }
                        if ((((((((F::BtlParam_GetFlags(c, l_atk_5)) as u64)) as i32)) & (8i32))) != 0) {
                            { let o_ = l_n_9; let n_ = (o_).wrapping_add(1); l_n_9 = n_; };
                        }
                        if ((F::BtlAct_TestPoweredSkill(c, p_chr_1, ((((64i32) as i64 as u64)) as u32))) != 0) {
                            { let o_ = l_n_9; let n_ = (o_).wrapping_add(1); l_n_9 = n_; };
                        }
                        if ((((((c.m.r32((p_chr_1).wrapping_add(3604)) as i32)) > (0i32)) as i32)) != 0) {
                            { let o_ = l_n_9; let n_ = (o_).wrapping_add(1); l_n_9 = n_; };
                        }
                        if ((F::BtlMember_HasAbility(c, p_chr_1, 69i32)) != 0) {
                            { let o_ = l_n_9; let n_ = (o_).wrapping_add(1); l_n_9 = n_; };
                        }
                        if ((F::BtlAct_TestPoweredSkill(c, l_atk_5, ((((256i32) as i64 as u64)) as u32))) != 0) {
                            { let o_ = l_n_9; let n_ = (o_).wrapping_sub(1); l_n_9 = n_; };
                        }
                        if ((F::BtlMember_HasAbility(c, l_atk_5, 71i32)) != 0) {
                            { let o_ = l_n_9; let n_ = (o_).wrapping_sub(1); l_n_9 = n_; };
                        } else {
                            if ((F::BtlMember_HasAbility(c, l_atk_5, 112i32)) != 0) {
                                if ((F::BtlChar_TestFlag(c, l_atk_5, ((((6i32) as i64 as u64)) as u32))) != 0) {
                                    { let o_ = l_n_9; let n_ = (o_).wrapping_sub(1); l_n_9 = n_; };
                                }
                            }
                        }
                        if (((((((((l_n_9) > (0i32)) as i32)) != 0) && ((((F::BtlKiBlast_GetFlagsOfHit(c, p_hit_2)) & (16i32))) != 0)) as i32)) != 0) {
                            l_react_8 = 2i32;
                        } else {
                            if ((((((((l_stFlags_4) & (((256i32) as u32)))) != 0) && ((((F::BtlKiBlast_GetFlagsOfHit(c, p_hit_2)) & (16i32))) != 0)) as i32)) != 0) {
                                if (((((F::BtlObjAnim_QueryEvent(c, l_obj_3, ((1i32) as u64), 0i32, ((((3i32) as i64 as u64)) as u32))) == (0i32)) as i32)) != 0) {
                                    l_react_8 = 2i32;
                                } else {
                                    if ((((((((c.m.r8((l_obj_3).wrapping_add(3245)) as i8)) as i32)) != ((!((((c.m.r8((l_obj_3).wrapping_add(3244)) as i8)) as i32))))) as i32)) != 0) {
                                        l_react_8 = 2i32;
                                    } else {
                                        l_react_8 = F::BtlKiBlast_GetReactOfHit(c, p_hit_2);
                                    }
                                }
                            } else {
                                l_react_8 = F::BtlKiBlast_GetReactOfHit(c, p_hit_2);
                            }
                        }
                    }
                }
            }
        }
        if (((((l_react_8) == (2i32)) as i32)) != 0) {
            l_g_12 = (p_chr_1).wrapping_add(4016);
            { let v_ = l_react_8; let a_ = l_g_12; c.m.w32(a_, (v_) as u32); };
            { let v_ = 3i32; let a_ = (l_g_12).wrapping_add(56); c.m.w32(a_, (v_) as u32); };
        } else {
            l_r_13 = (p_chr_1).wrapping_add(4016);
            { let v_ = l_react_8; let a_ = l_r_13; c.m.w32(a_, (v_) as u32); };
            { let v_ = 0i32; let a_ = (l_r_13).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
            { let v_ = F32(0x00000000); let a_ = ((p_chr_1).wrapping_add(4016)).wrapping_add(32); c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x00000000); let a_ = ((p_chr_1).wrapping_add(4016)).wrapping_add(36); c.m.w32(a_, (v_).0); };
            { let v_ = l_yaw_6; let a_ = (l_r_13).wrapping_add(28); c.m.w32(a_, (v_).0); };
            { let v_ = l_front_7; let a_ = (l_r_13).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
            { let a2_ = F::BtlKiBlast_GetPushOfHit(c, p_hit_2); F::BtlMove_SetImpulseDir(c, p_chr_1, fp, a2_) };
            F::BtlChar_PlayVoice(c, p_chr_1, 0i32);
        }
        F::BtlChar_SetFxBit(c, p_chr_1, 29i32);
        if (((((F::BtlKiBlast_GetTypeOfHit(c, p_hit_2)) == (2i32)) as i32)) != 0) {
            F::BtlCharSnd_RequestAt(c, (p_hit_2).wrapping_add(32), 0i32, 66i32, F32(0x43480000), F32(0x44bb8000));
        } else {
            { let a2_ = (if ((F::BtlChar_FrameMod(c, 2i32)) != 0) { 27i32 } else { 17i32 }); F::BtlCharSnd_RequestAt(c, (p_hit_2).wrapping_add(32), 0i32, a2_, F32(0x43480000), F32(0x44bb8000)) };
        }
        l_dmg_10 = F::BtlKiBlast_GetDamageOfHit(c, p_hit_2);
        if ((((((({ let a_ = (c.m.r32((p_hit_2).wrapping_add(104))).wrapping_add(65); c.m.r8(a_) }) as i32)) >= (2i32)) as i32)) != 0) {
            { let r_ = (({ let a_ = (c.m.r32((p_hit_2).wrapping_add(104))).wrapping_add(65); c.m.r8(a_) }) as i32); let n_ = rt::div_i32(l_dmg_10, r_); l_dmg_10 = n_; };
        }
        l_time_11 = F32(0x3dcccccc);
        { let _ = F::BtlMember_Damage(c, p_chr_1, l_dmg_10, 0i32); };
        F::ChrCam_AddShake(c, p_chr_1, F32(0x40400000), l_time_11);
        F::BtlChar_Vibrate(c, p_chr_1, F32(0x3f800000), l_time_11);
        's1: {
            let sv14: i32 = (({ let a_ = (c.m.r32((p_hit_2).wrapping_add(104))).wrapping_add(24); (c.m.r16(a_) as i16) }) as i32);
            match sv14 {
                1i32..=3i32 => {
                    { let a0_ = (({ let a_ = (c.m.r32((p_hit_2).wrapping_add(104))).wrapping_add(16); (c.m.r16(a_) as i16) }) as i32); F::BtlEvent_Raise(c, a0_, 54i32) };
                    break 's1;
                }
                _ => {
                }
            }
        }
        if ((((!((F::BtlChar_TestFlag(c, p_chr_1, ((((147i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) {
            F::BtlChar_SetHeldFlag(c, p_chr_1, ((((5i32) as i64 as u64)) as u32));
        }
        break 'f 1i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlColl_HitByBlast", &PROBE_N, &[], true); }
    r_
}

pub fn BtlColl_HitByStrike(c: &mut Ctx, mut p_chr_1: u32, mut p_hit_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlColl_HitByStrike", &PROBE_N, &[(p_chr_1) as u32, (p_hit_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_yaw_3: F32 = F32(0);
    let mut l_front_4: i32 = 0;
    let mut l_atk_5: u32 = 0;
    let mut l_slot_6: i32 = 0;
    let mut l_react_7: i32 = 0;
    let mut l_r_8: u32 = 0;
    let mut l_fl_9: i32 = 0;
    let mut l_dmg_10: i32 = 0;
    let mut l_time_11: F32 = F32(0);
    let r_ = 'f: {
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((69i32) as i64 as u64)) as u32))) != 0) {
            break 'f 0i32;
        }
        if (((((c.m.r32((p_hit_2).wrapping_add(100))) == (((0i32) as u32))) as i32)) != 0) {
            break 'f 0i32;
        }
        l_yaw_3 = F::BtlColl_GetHitDir(c, p_chr_1, p_hit_2, fp);
        l_front_4 = F::BtlColl_IsFacing(c, p_chr_1, l_yaw_3);
        l_slot_6 = { let a_ = (c.m.r32((p_hit_2).wrapping_add(100))).wrapping_add(4); (c.m.r32(a_) as i32) };
        l_atk_5 = { let a0_ = { let a_ = c.m.r32((p_hit_2).wrapping_add(100)); (c.m.r32(a_) as i32) }; F::BtlChar_Get(c, a0_) };
        if (((((F::BtlSkill_GetHitDirKind(c, l_atk_5, l_slot_6)) != (4i32)) as i32)) != 0) {
            if (((rt::fgt({ let a0_ = F::BtlOpp_GetYawFromFacing(c, l_atk_5); rt::ffabs(a0_) }, F32(0x3fc90fda)) as i32)) != 0) {
                break 'f 0i32;
            }
        }
        if (((({ let a0_ = F::BtlAnim_GetId(c, p_chr_1); F::BtlAnim_GetFlags(c, ((((a0_) as i64 as u64)) as u32)) }) & (((2048i32) as u32)))) != 0) {
            l_react_7 = F::BtlSkill_GetReactS800(c, l_atk_5, l_slot_6);
        } else {
            l_react_7 = F::BtlSkill_GetReact(c, l_atk_5, l_slot_6);
        }
        if (((((l_react_7) == (0i32)) as i32)) != 0) {
            l_react_7 = 3i32;
        }
        if (((((l_react_7) == (1i32)) as i32)) != 0) {
            break 'f 0i32;
        }
        l_r_8 = (p_chr_1).wrapping_add(4016);
        if ((((((c.m.r32((p_chr_1).wrapping_add(3608)) as i32)) > (0i32)) as i32)) != 0) {
            l_react_7 = 2i32;
        }
        { let v_ = l_yaw_3; let a_ = (l_r_8).wrapping_add(28); c.m.w32(a_, (v_).0); };
        { let v_ = l_front_4; let a_ = (l_r_8).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        { let v_ = l_react_7; let a_ = l_r_8; c.m.w32(a_, (v_) as u32); };
        if (((((l_react_7) != (2i32)) as i32)) != 0) {
            { let v_ = 0i32; let a_ = (l_r_8).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
            { let v_ = F32(0x00000000); let a_ = ((p_chr_1).wrapping_add(4016)).wrapping_add(32); c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x00000000); let a_ = ((p_chr_1).wrapping_add(4016)).wrapping_add(36); c.m.w32(a_, (v_).0); };
            F::BtlChar_SetFlag(c, p_chr_1, ((((148i32) as i64 as u64)) as u32));
            if ((l_front_4) != 0) {
                { let v_ = l_yaw_3; let a_ = ((p_chr_1).wrapping_add(4016)).wrapping_add(32); c.m.w32(a_, (v_).0); };
                { let v_ = F32(0x00000000); let a_ = ((p_chr_1).wrapping_add(4016)).wrapping_add(36); c.m.w32(a_, (v_).0); };
            } else {
                { let v_ = F::BtlUtil_WrapAngle(c, rt::fadd(l_yaw_3, F32(0x40490fda))); let a_ = ((p_chr_1).wrapping_add(4016)).wrapping_add(32); c.m.w32(a_, (v_).0); };
                { let v_ = F32(0x00000000); let a_ = ((p_chr_1).wrapping_add(4016)).wrapping_add(36); c.m.w32(a_, (v_).0); };
            }
            { let a2_ = F::BtlSkill_GetPush(c, l_atk_5, l_slot_6); F::BtlMove_SetImpulseDir(c, p_chr_1, fp, a2_) };
            's1: {
                let sv12: i32 = l_react_7;
                match sv12 {
                    23i32..=28i32 => {
                        { let v_ = F::BtlSkill_GetFrames(c, l_atk_5, l_slot_6); let a_ = ((p_chr_1).wrapping_add(4016)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
                        if ((F::BtlMember_HasAbility(c, p_chr_1, 58i32)) != 0) {
                            { let r_ = 2i32; let a_ = ((p_chr_1).wrapping_add(4016)).wrapping_add(48); let n_ = rt::div_i32((c.m.r32(a_) as i32), r_); c.m.w32(a_, (n_) as u32); };
                        }
                        if ((F::BtlMember_HasAbility(c, l_atk_5, 59i32)) != 0) {
                            { let r_ = rt::div_i32((c.m.r32(((p_chr_1).wrapping_add(4016)).wrapping_add(48)) as i32), 2i32); let a_ = ((p_chr_1).wrapping_add(4016)).wrapping_add(48); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                        }
                        break 's1;
                    }
                    _ => {
                    }
                }
            }
        }
        F::BtlChar_SetFxBit(c, p_chr_1, 29i32);
        l_fl_9 = 0i32;
        { let a2_ = ((F::BtlSkill_GetHitSound(c, l_atk_5, l_slot_6)) as u32); F::BtlColl_PlayHitSound(c, p_chr_1, p_hit_2, a2_) };
        l_dmg_10 = F::BtlSkill_GetDamage(c, l_atk_5, l_slot_6, 0i32);
        's2: {
            let sv13: i32 = l_slot_6;
            match sv13 {
                0i32 => {
                    l_fl_9 = 2097152i32;
                    break 's2;
                }
                1i32 => {
                    l_fl_9 = 4194304i32;
                    break 's2;
                }
                _ => {
                }
            }
        }
        { let _ = F::BtlMember_Damage(c, p_chr_1, l_dmg_10, l_fl_9); };
        F::ChrCam_AddShake(c, p_chr_1, F32(0x40400000), F32(0x3e4ccccc));
        F::BtlChar_Vibrate(c, p_chr_1, F32(0x3f800000), F32(0x3e4ccccc));
        's3: {
            let sv14: i32 = l_slot_6;
            match sv14 {
                0i32 => {
                    { let a0_ = (c.m.r32(l_atk_5) as i32); F::BtlEvent_Raise(c, a0_, 41i32) };
                    break 's3;
                }
                1i32 => {
                    { let a0_ = (c.m.r32(l_atk_5) as i32); F::BtlEvent_Raise(c, a0_, 42i32) };
                    break 's3;
                }
                _ => {
                }
            }
        }
        break 'f 1i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlColl_HitByStrike", &PROBE_N, &[], true); }
    r_
}

pub fn BtlColl_ApplyRushHit(c: &mut Ctx, mut p_chr_1: u32, mut p_atk_2: u32, mut p_hit_3: u32, mut p_react_4: i32, mut p_yaw_5: F32, mut p_front_6: i32, mut p_dir_7: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlColl_ApplyRushHit", &PROBE_N, &[(p_chr_1) as u32, (p_atk_2) as u32, (p_hit_3) as u32, (p_react_4) as u32, (p_yaw_5).0, (p_front_6) as u32, (p_dir_7) as u32], false); }
    let mut l_r_8: u32 = 0;
    let mut l_slot_9: i32 = 0;
    let mut l_alt_10: i32 = 0;
    let mut l_fl_11: i32 = 0;
    let mut l_dmg_12: i32 = 0;
    let mut l_n_13: i32 = 0;
    let r_ = 'f: {
    l_r_8 = (p_chr_1).wrapping_add(4016);
    l_slot_9 = { let a_ = (c.m.r32((p_hit_3).wrapping_add(100))).wrapping_add(4); (c.m.r32(a_) as i32) };
    { let v_ = p_react_4; let a_ = l_r_8; c.m.w32(a_, (v_) as u32); };
    { let v_ = 0i32; let a_ = (l_r_8).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
    { let v_ = l_slot_9; let a_ = (l_r_8).wrapping_add(68); c.m.w32(a_, (v_) as u32); };
    l_alt_10 = ((((c.m.r32((p_hit_3).wrapping_add(80)) as i32)).wrapping_shr((4i32) as u32)) & (1i32));
    F::BtlChar_SetFlag(c, p_chr_1, ((((148i32) as i64 as u64)) as u32));
    if ((p_front_6) != 0) {
        { let v_ = p_yaw_5; let a_ = ((p_chr_1).wrapping_add(4016)).wrapping_add(32); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = ((p_chr_1).wrapping_add(4016)).wrapping_add(36); c.m.w32(a_, (v_).0); };
    } else {
        { let v_ = F::BtlUtil_WrapAngle(c, rt::fadd(p_yaw_5, F32(0x40490fda))); let a_ = ((p_chr_1).wrapping_add(4016)).wrapping_add(32); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = ((p_chr_1).wrapping_add(4016)).wrapping_add(36); c.m.w32(a_, (v_).0); };
    }
    { let v_ = p_yaw_5; let a_ = (l_r_8).wrapping_add(28); c.m.w32(a_, (v_).0); };
    { let v_ = p_front_6; let a_ = (l_r_8).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
    if ((l_alt_10) != 0) {
        { let a2_ = F::BtlSuper_GetPush(c, p_atk_2, l_slot_9); F::BtlMove_SetImpulseDir(c, p_chr_1, p_dir_7, a2_) };
    }
    F::BtlChar_SetFxBit(c, p_chr_1, 29i32);
    { let a2_ = ((F::BtlSuper_GetHitSound(c, p_atk_2, l_slot_9, l_alt_10)) as u32); F::BtlColl_PlayHitSound(c, p_chr_1, p_hit_3, a2_) };
    l_fl_11 = 256i32;
    l_dmg_12 = F::BtlSuper_GetDamage(c, p_atk_2, l_slot_9, 0i32, 0i32);
    's1: {
        let sv14: i32 = l_slot_9;
        match sv14 {
            2i32 => {
                l_fl_11 = 8388864i32;
                break 's1;
            }
            3i32 => {
                l_fl_11 = 16777472i32;
                break 's1;
            }
            4i32 => {
                l_fl_11 = 33554688i32;
                break 's1;
            }
            _ => {
            }
        }
    }
    { let _ = F::BtlMember_Damage(c, p_chr_1, l_dmg_12, l_fl_11); };
    F::BtlCharApi_ShakeCamsNear(c, (p_hit_3).wrapping_add(32), F32(0x42c80000), F32(0x44bb8000), F32(0x40a00000), F32(0x3e999999));
    F::BtlCharApi_RumbleNear(c, (p_hit_3).wrapping_add(32), F32(0x42c80000), F32(0x447a0000), F32(0x3f800000), F32(0x3e999999));
    F::BtlChar_SetHeldFlag(c, p_chr_1, ((((29i32) as i64 as u64)) as u32));
    F::BtlChar_SetHeldFlag(c, p_chr_1, ((((30i32) as i64 as u64)) as u32));
    if (((((F::BtlSuper_GetType(c, p_atk_2, l_slot_9)) == (7i32)) as i32)) != 0) {
        F::BtlChar_SetFlag(c, p_atk_2, ((((115i32) as i64 as u64)) as u32));
    }
    if (((((F::BtlSuper_GetType(c, p_atk_2, l_slot_9)) == (8i32)) as i32)) != 0) {
        l_n_13 = F::BtlSuper_GetRecoilDamage(c, p_atk_2, l_slot_9);
        if (((((l_n_13) > (0i32)) as i32)) != 0) {
            if ((F::BtlMember_Damage(c, p_atk_2, l_n_13, 1067i32)) != 0) {
                F::BtlChar_SetFlag(c, p_atk_2, ((((149i32) as i64 as u64)) as u32));
            }
        }
    }
    { let v_ = 0i32; let a_ = (p_chr_1).wrapping_add(4204); c.m.w32(a_, (v_) as u32); };
    's2: {
        let sv15: i32 = l_slot_9;
        match sv15 {
            2i32 | 3i32 => {
                F::BtlChar_AddStageTimer(c, F32(0x40000000));
                break 's2;
            }
            4i32 => {
                F::BtlChar_AddStageTimer(c, F32(0x40a00000));
                break 's2;
            }
            _ => {
            }
        }
    }
    if (((((l_slot_9) == (4i32)) as i32)) != 0) {
        if ((((((((!((F::BtlChar_TestFlag(c, p_chr_1, ((((19i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) && (((((p_react_4) != (34i32)) as i32)) != 0)) as i32)) != 0) {
            F::BtlChar_ClearFlag(c, p_chr_1, ((((5i32) as i64 as u64)) as u32));
        }
    } else {
        if ((((!((F::BtlChar_TestFlag(c, p_chr_1, ((((147i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) {
            F::BtlChar_SetHeldFlag(c, p_chr_1, ((((5i32) as i64 as u64)) as u32));
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlColl_ApplyRushHit", &PROBE_N, &[], true); }
    r_
}

pub fn BtlColl_ApplyRushNoFlinch(c: &mut Ctx, mut p_chr_1: u32, mut p_atk_2: u32, mut p_hit_3: u32, mut p_yaw_4: F32, mut p_front_5: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlColl_ApplyRushNoFlinch", &PROBE_N, &[(p_chr_1) as u32, (p_atk_2) as u32, (p_hit_3) as u32, (p_yaw_4).0, (p_front_5) as u32], false); }
    let mut l_fl_6: i32 = 0;
    let mut l_r_7: u32 = 0;
    let mut l_slot_8: i32 = 0;
    let mut l_alt_9: i32 = 0;
    let mut l_dmg_10: i32 = 0;
    let mut l_n_11: i32 = 0;
    let r_ = 'f: {
    l_fl_6 = 256i32;
    l_r_7 = (p_chr_1).wrapping_add(4016);
    l_slot_8 = { let a_ = (c.m.r32((p_hit_3).wrapping_add(100))).wrapping_add(4); (c.m.r32(a_) as i32) };
    { let v_ = p_front_5; let a_ = (l_r_7).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
    { let v_ = p_yaw_4; let a_ = (l_r_7).wrapping_add(28); c.m.w32(a_, (v_).0); };
    { let v_ = 2i32; let a_ = l_r_7; c.m.w32(a_, (v_) as u32); };
    l_alt_9 = ((((c.m.r32((p_hit_3).wrapping_add(80)) as i32)).wrapping_shr((4i32) as u32)) & (1i32));
    F::BtlChar_SetFxBit(c, p_chr_1, 29i32);
    { let a2_ = ((F::BtlSuper_GetHitSound(c, p_atk_2, l_slot_8, l_alt_9)) as u32); F::BtlColl_PlayHitSound(c, p_chr_1, p_hit_3, a2_) };
    l_dmg_10 = F::BtlSuper_GetDamage(c, p_atk_2, l_slot_8, 0i32, 0i32);
    's1: {
        let sv12: i32 = l_slot_8;
        match sv12 {
            2i32 => {
                l_fl_6 = 8388864i32;
                break 's1;
            }
            3i32 => {
                l_fl_6 = 16777472i32;
                break 's1;
            }
            4i32 => {
                l_fl_6 = 33554688i32;
                break 's1;
            }
            _ => {
            }
        }
    }
    { let _ = F::BtlMember_Damage(c, p_chr_1, l_dmg_10, l_fl_6); };
    l_n_11 = F::BtlSuper_GetRecoilDamage(c, p_atk_2, l_slot_8);
    if (((((l_n_11) > (0i32)) as i32)) != 0) {
        if ((F::BtlMember_Damage(c, p_atk_2, l_n_11, 1067i32)) != 0) {
            F::BtlChar_SetFlag(c, p_atk_2, ((((149i32) as i64 as u64)) as u32));
        }
    }
    F::BtlCharApi_ShakeCamsNear(c, (p_hit_3).wrapping_add(32), F32(0x42c80000), F32(0x44bb8000), F32(0x40a00000), F32(0x3e999999));
    F::BtlCharApi_RumbleNear(c, (p_hit_3).wrapping_add(32), F32(0x42c80000), F32(0x447a0000), F32(0x3f800000), F32(0x3e999999));
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlColl_ApplyRushNoFlinch", &PROBE_N, &[], true); }
    r_
}

pub fn BtlColl_StartRushCatch(c: &mut Ctx, mut p_chr_1: u32, mut p_atk_2: u32, mut p_hit_3: u32, mut p_react_4: i32, mut p_yaw_5: F32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlColl_StartRushCatch", &PROBE_N, &[(p_chr_1) as u32, (p_atk_2) as u32, (p_hit_3) as u32, (p_react_4) as u32, (p_yaw_5).0], false); }
    let mut l_r_6: u32 = 0;
    let mut l_slot_7: i32 = 0;
    let mut l_back_8: i32 = 0;
    let mut l_tf_9: u32 = 0;
    let mut l_alt_10: i32 = 0;
    let r_ = 'f: {
    l_r_6 = (p_chr_1).wrapping_add(4016);
    l_slot_7 = { let a_ = (c.m.r32((p_hit_3).wrapping_add(100))).wrapping_add(4); (c.m.r32(a_) as i32) };
    if ((((((c.m.r32((p_atk_2).wrapping_add(4016)) as i32)) >= (3i32)) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((F::BtlChar_IsDead(c, p_atk_2)) != 0) {
        break 'f 0i32;
    }
    if ((F::BtlChar_IsDead(c, p_chr_1)) != 0) {
        break 'f 0i32;
    }
    if ((((!((F::BtlChar_TestFlag(c, p_atk_2, ((((3i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    l_alt_10 = ((((c.m.r32((p_hit_3).wrapping_add(80)) as i32)).wrapping_shr((4i32) as u32)) & (1i32));
    { let v_ = p_react_4; let a_ = l_r_6; c.m.w32(a_, (v_) as u32); };
    { let v_ = 0i32; let a_ = (l_r_6).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
    { let v_ = F32(0x00000000); let a_ = ((p_chr_1).wrapping_add(4016)).wrapping_add(32); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x00000000); let a_ = ((p_chr_1).wrapping_add(4016)).wrapping_add(36); c.m.w32(a_, (v_).0); };
    { let v_ = F::BtlSuper_GetId(c, p_atk_2, l_slot_7); let a_ = (p_atk_2).wrapping_add(3728); c.m.w32(a_, (v_) as u32); };
    { let v_ = (c.m.r32(p_atk_2) as i32); let a_ = ((p_atk_2).wrapping_add(3728)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
    { let v_ = (c.m.r32(p_chr_1) as i32); let a_ = ((p_atk_2).wrapping_add(3728)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
    { let v_ = l_slot_7; let a_ = ((p_atk_2).wrapping_add(3728)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
    { let v_ = F::BtlSuper_GetUnk14D(c, p_atk_2, l_slot_7); let a_ = ((p_atk_2).wrapping_add(3728)).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
    l_tf_9 = F::BtlSuper_GetFlags(c, p_atk_2, l_slot_7);
    l_back_8 = 0i32;
    if ((((l_tf_9) & (((268435456i32) as u32)))) != 0) {
        l_back_8 = 1i32;
    }
    if ((((l_tf_9) & (((4194304i32) as u32)))) != 0) {
        { let v_ = 1i32; let a_ = ((p_atk_2).wrapping_add(3728)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
        { let v_ = F::BtlSuper_GetThrowAngleA(c, p_atk_2, l_slot_7); let a_ = ((p_atk_2).wrapping_add(3728)).wrapping_add(96); c.m.w32(a_, (v_).0); };
        { let v_ = F::BtlSuper_GetThrowAngleB(c, p_atk_2, l_slot_7); let a_ = ((p_atk_2).wrapping_add(3728)).wrapping_add(100); c.m.w32(a_, (v_).0); };
    } else {
        { let v_ = 0i32; let a_ = ((p_atk_2).wrapping_add(3728)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
        { let v_ = F32(0x00000000); let a_ = ((p_atk_2).wrapping_add(3728)).wrapping_add(96); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = ((p_atk_2).wrapping_add(3728)).wrapping_add(100); c.m.w32(a_, (v_).0); };
    }
    if ((((l_tf_9) & (((1048576i32) as u32)))) != 0) {
        { let v_ = 1i32; let a_ = ((p_atk_2).wrapping_add(3728)).wrapping_add(56); c.m.w32(a_, (v_) as u32); };
    } else {
        { let v_ = 0i32; let a_ = ((p_atk_2).wrapping_add(3728)).wrapping_add(56); c.m.w32(a_, (v_) as u32); };
    }
    if ((((l_tf_9) & (((8388608i32) as u32)))) != 0) {
        { let v_ = 1i32; let a_ = ((p_atk_2).wrapping_add(3728)).wrapping_add(60); c.m.w32(a_, (v_) as u32); };
    } else {
        { let v_ = 0i32; let a_ = ((p_atk_2).wrapping_add(3728)).wrapping_add(60); c.m.w32(a_, (v_) as u32); };
    }
    { let v_ = 0i32; let a_ = ((p_atk_2).wrapping_add(3728)).wrapping_add(68); c.m.w32(a_, (v_) as u32); };
    if (((((p_react_4) == (30i32)) as i32)) != 0) {
        { let v_ = 1i32; let a_ = ((p_atk_2).wrapping_add(3728)).wrapping_add(68); c.m.w32(a_, (v_) as u32); };
    }
    { let v_ = 0i32; let a_ = ((p_atk_2).wrapping_add(3728)).wrapping_add(84); c.m.w32(a_, (v_) as u32); };
    if ((((l_tf_9) & (((65536i32) as u32)))) != 0) {
        { let v_ = 1i32; let a_ = ((p_atk_2).wrapping_add(3728)).wrapping_add(84); c.m.w32(a_, (v_) as u32); };
    }
    { let v_ = 0i32; let a_ = ((p_atk_2).wrapping_add(3728)).wrapping_add(88); c.m.w32(a_, (v_) as u32); };
    if (((({ let a0_ = F::BtlAnim_GetId(c, p_chr_1); F::BtlAnim_GetFlags(c, ((((a0_) as i64 as u64)) as u32)) }) & (((2048i32) as u32)))) != 0) {
        { let v_ = 1i32; let a_ = ((p_atk_2).wrapping_add(3728)).wrapping_add(88); c.m.w32(a_, (v_) as u32); };
    }
    { let v_ = 0i32; let a_ = ((p_atk_2).wrapping_add(3728)).wrapping_add(72); c.m.w32(a_, (v_) as u32); };
    { let v_ = (1i32).wrapping_neg(); let a_ = ((p_atk_2).wrapping_add(3728)).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
    { let v_ = (1i32).wrapping_neg(); let a_ = ((p_atk_2).wrapping_add(3728)).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
    { let v_ = 0i32; let a_ = ((p_atk_2).wrapping_add(3728)).wrapping_add(76); c.m.w32(a_, (v_) as u32); };
    { let v_ = 0i32; let a_ = ((p_atk_2).wrapping_add(3728)).wrapping_add(80); c.m.w32(a_, (v_) as u32); };
    { let v_ = 0i32; let a_ = ((p_atk_2).wrapping_add(3728)).wrapping_add(92); c.m.w32(a_, (v_) as u32); };
    F::BtlChar_SetFlag(c, p_atk_2, ((((115i32) as i64 as u64)) as u32));
    { let v_ = (c.m.r32((p_atk_2).wrapping_add(3728)) as i32); let a_ = (p_chr_1).wrapping_add(3728); c.m.w32(a_, (v_) as u32); };
    { let v_ = (c.m.r32(((p_atk_2).wrapping_add(3728)).wrapping_add(4)) as i32); let a_ = ((p_chr_1).wrapping_add(3728)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
    { let v_ = (c.m.r32(((p_atk_2).wrapping_add(3728)).wrapping_add(8)) as i32); let a_ = ((p_chr_1).wrapping_add(3728)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
    { let v_ = (c.m.r32(((p_atk_2).wrapping_add(3728)).wrapping_add(12)) as i32); let a_ = ((p_chr_1).wrapping_add(3728)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
    { let v_ = (c.m.r32(((p_atk_2).wrapping_add(3728)).wrapping_add(20)) as i32); let a_ = ((p_chr_1).wrapping_add(3728)).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
    { let v_ = (c.m.r32(((p_atk_2).wrapping_add(3728)).wrapping_add(48)) as i32); let a_ = ((p_chr_1).wrapping_add(3728)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
    { let v_ = (c.m.r32(((p_atk_2).wrapping_add(3728)).wrapping_add(56)) as i32); let a_ = ((p_chr_1).wrapping_add(3728)).wrapping_add(56); c.m.w32(a_, (v_) as u32); };
    { let v_ = (c.m.r32(((p_atk_2).wrapping_add(3728)).wrapping_add(60)) as i32); let a_ = ((p_chr_1).wrapping_add(3728)).wrapping_add(60); c.m.w32(a_, (v_) as u32); };
    { let v_ = F32(c.m.r32(((p_atk_2).wrapping_add(3728)).wrapping_add(96))); let a_ = ((p_chr_1).wrapping_add(3728)).wrapping_add(96); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32(((p_atk_2).wrapping_add(3728)).wrapping_add(100))); let a_ = ((p_chr_1).wrapping_add(3728)).wrapping_add(100); c.m.w32(a_, (v_).0); };
    { let v_ = p_yaw_5; let a_ = ((p_chr_1).wrapping_add(4016)).wrapping_add(28); c.m.w32(a_, (v_).0); };
    { let v_ = l_back_8; let a_ = ((p_chr_1).wrapping_add(4016)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
    { let v_ = (c.m.r32(((p_atk_2).wrapping_add(3728)).wrapping_add(68)) as i32); let a_ = ((p_chr_1).wrapping_add(3728)).wrapping_add(68); c.m.w32(a_, (v_) as u32); };
    { let v_ = (c.m.r32(((p_atk_2).wrapping_add(3728)).wrapping_add(72)) as i32); let a_ = ((p_chr_1).wrapping_add(3728)).wrapping_add(72); c.m.w32(a_, (v_) as u32); };
    { let v_ = (c.m.r32(((p_atk_2).wrapping_add(3728)).wrapping_add(76)) as i32); let a_ = ((p_chr_1).wrapping_add(3728)).wrapping_add(76); c.m.w32(a_, (v_) as u32); };
    { let v_ = (c.m.r32(((p_atk_2).wrapping_add(3728)).wrapping_add(24)) as i32); let a_ = ((p_chr_1).wrapping_add(3728)).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
    { let v_ = (c.m.r32(((p_atk_2).wrapping_add(3728)).wrapping_add(28)) as i32); let a_ = ((p_chr_1).wrapping_add(3728)).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
    { let v_ = (c.m.r32(((p_atk_2).wrapping_add(3728)).wrapping_add(80)) as i32); let a_ = ((p_chr_1).wrapping_add(3728)).wrapping_add(80); c.m.w32(a_, (v_) as u32); };
    { let v_ = (c.m.r32(((p_atk_2).wrapping_add(3728)).wrapping_add(84)) as i32); let a_ = ((p_chr_1).wrapping_add(3728)).wrapping_add(84); c.m.w32(a_, (v_) as u32); };
    { let v_ = (c.m.r32(((p_atk_2).wrapping_add(3728)).wrapping_add(88)) as i32); let a_ = ((p_chr_1).wrapping_add(3728)).wrapping_add(88); c.m.w32(a_, (v_) as u32); };
    { let v_ = (c.m.r32(((p_atk_2).wrapping_add(3728)).wrapping_add(92)) as i32); let a_ = ((p_chr_1).wrapping_add(3728)).wrapping_add(92); c.m.w32(a_, (v_) as u32); };
    { let a2_ = ((F::BtlSuper_GetHitSound(c, p_atk_2, l_slot_7, l_alt_10)) as u32); F::BtlColl_PlayHitSound(c, ((0i32) as u32), p_hit_3, a2_) };
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlColl_StartRushCatch", &PROBE_N, &[], true); }
    r_
}

pub fn BtlColl_HitByRush(c: &mut Ctx, mut p_chr_1: u32, mut p_hit_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlColl_HitByRush", &PROBE_N, &[(p_chr_1) as u32, (p_hit_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_yaw_3: F32 = F32(0);
    let mut l_front_4: i32 = 0;
    let mut l_atk_5: u32 = 0;
    let mut l_slot_6: i32 = 0;
    let mut l_react_7: i32 = 0;
    let r_ = 'f: {
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((70i32) as i64 as u64)) as u32))) != 0) {
            break 'f 0i32;
        }
        if (((((c.m.r32((p_hit_2).wrapping_add(100))) == (((0i32) as u32))) as i32)) != 0) {
            break 'f 0i32;
        }
        l_yaw_3 = F::BtlColl_GetHitDir(c, p_chr_1, p_hit_2, fp);
        l_front_4 = F::BtlColl_IsFacing(c, p_chr_1, l_yaw_3);
        l_slot_6 = { let a_ = (c.m.r32((p_hit_2).wrapping_add(100))).wrapping_add(4); (c.m.r32(a_) as i32) };
        l_atk_5 = { let a0_ = { let a_ = c.m.r32((p_hit_2).wrapping_add(100)); (c.m.r32(a_) as i32) }; F::BtlChar_Get(c, a0_) };
        's1: {
            let sv8: i32 = F::BtlSuper_GetHitDirKind(c, l_atk_5, l_slot_6);
            match sv8 {
                0i32 | 2i32 => {
                    if (((rt::fgt({ let a0_ = F::BtlOpp_GetYawFromFacing(c, l_atk_5); rt::ffabs(a0_) }, F32(0x3fc90fda)) as i32)) != 0) {
                        break 'f 0i32;
                    }
                    break 's1;
                }
                _ => {
                }
            }
        }
        if (((({ let a0_ = F::BtlAnim_GetId(c, p_chr_1); F::BtlAnim_GetFlags(c, ((((a0_) as i64 as u64)) as u32)) }) & (((2048i32) as u32)))) != 0) {
            if (((((c.m.r32((p_hit_2).wrapping_add(80)) as i32)) & (16i32))) != 0) {
                l_react_7 = F::BtlSuper_GetReactAltS800(c, l_atk_5, l_slot_6);
            } else {
                l_react_7 = F::BtlSuper_GetReactS800(c, l_atk_5, l_slot_6);
            }
        } else {
            if (((((c.m.r32((p_hit_2).wrapping_add(80)) as i32)) & (16i32))) != 0) {
                l_react_7 = F::BtlSuper_GetReactAlt(c, l_atk_5, l_slot_6);
            } else {
                l_react_7 = F::BtlSuper_GetReact(c, l_atk_5, l_slot_6);
            }
        }
        if (((((l_react_7) == (0i32)) as i32)) != 0) {
            l_react_7 = 15i32;
        }
        if (((((l_react_7) == (1i32)) as i32)) != 0) {
            break 'f 0i32;
        }
        if (((((l_react_7) == (2i32)) as i32)) != 0) {
            F::BtlColl_ApplyRushNoFlinch(c, p_chr_1, l_atk_5, p_hit_2, l_yaw_3, l_front_4);
        } else {
            if (((((((((l_react_7) == (29i32)) as i32)) != 0) || (((((l_react_7) == (30i32)) as i32)) != 0)) as i32)) != 0) {
                if ((((!((F::BtlColl_StartRushCatch(c, p_chr_1, l_atk_5, p_hit_2, l_react_7, l_yaw_3)) != 0)) as i32)) != 0) {
                    break 'f 0i32;
                }
            } else {
                if (((((l_react_7) == (34i32)) as i32)) != 0) {
                    if ((((!(({ let a3_ = F::BtlSuper_GetFlags(c, l_atk_5, l_slot_6); F::BtlColl_StartThrow(c, l_atk_5, p_chr_1, l_slot_6, a3_) }) != 0)) as i32)) != 0) {
                        break 'f 0i32;
                    }
                } else {
                    if ((((((c.m.r32((p_chr_1).wrapping_add(3608)) as i32)) > (0i32)) as i32)) != 0) {
                        F::BtlColl_ApplyRushNoFlinch(c, p_chr_1, l_atk_5, p_hit_2, l_yaw_3, l_front_4);
                    } else {
                        F::BtlColl_ApplyRushHit(c, p_chr_1, l_atk_5, p_hit_2, l_react_7, l_yaw_3, l_front_4, fp);
                    }
                }
            }
        }
        's2: {
            let sv9: i32 = l_slot_6;
            match sv9 {
                2i32 => {
                    { let a0_ = (c.m.r32(l_atk_5) as i32); F::BtlEvent_Raise(c, a0_, 43i32) };
                    break 's2;
                }
                3i32 => {
                    { let a0_ = (c.m.r32(l_atk_5) as i32); F::BtlEvent_Raise(c, a0_, 44i32) };
                    break 's2;
                }
                4i32 => {
                    { let a0_ = (c.m.r32(l_atk_5) as i32); F::BtlEvent_Raise(c, a0_, 45i32) };
                    break 's2;
                }
                _ => {
                }
            }
        }
        break 'f 1i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlColl_HitByRush", &PROBE_N, &[], true); }
    r_
}

pub fn BtlColl_TryDodge(c: &mut Ctx, mut p_objId_1: i32, mut p_hit_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlColl_TryDodge", &PROBE_N, &[(p_objId_1) as u32, (p_hit_2) as u32], false); }
    let mut l_chr_3: u32 = 0;
    let mut l_atk_4: u32 = 0;
    let mut l_slot_5: i32 = 0;
    let mut l_canDodge_6: i32 = 0;
    let mut l_rush_7: i32 = 0;
    let mut l_ok_8: i32 = 0;
    let mut l_free_9: i32 = 0;
    let mut l_used_10: i32 = 0;
    let mut l_side_11: i32 = 0;
    let r_ = 'f: {
    l_chr_3 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_3) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((F::BtlChar_IsFrozen(c, l_chr_3)) != 0) {
        break 'f 1i32;
    }
    if ((F::BtlChar_TestFlag(c, l_chr_3, ((((170i32) as i64 as u64)) as u32))) != 0) {
        break 'f 1i32;
    }
    if ((F::BtlChar_TestFlag(c, l_chr_3, ((((90i32) as i64 as u64)) as u32))) != 0) {
        break 'f 0i32;
    }
    if (({ let a0_ = F::BtlAct_GetCurrent(c, l_chr_3); F::BtlAct_IsTechniqueId(c, a0_) }) != 0) {
        l_slot_5 = F::BtlAct_GetCurrentClass(c, l_chr_3);
        if (((((((((l_slot_5) >= (2i32)) as i32)) != 0) && (((((l_slot_5) < (5i32)) as i32)) != 0)) as i32)) != 0) {
            if (((((F::BtlSuper_GetType(c, l_chr_3, l_slot_5)) == (8i32)) as i32)) != 0) {
                if (((((F::BtlSuper_GetUnk1E(c, l_chr_3, l_slot_5)) >= ((c.m.r32((p_hit_2).wrapping_add(8)) as i32))) as i32)) != 0) {
                    break 'f 1i32;
                }
            }
        }
    }
    if ((((((c.m.r32((p_hit_2).wrapping_add(12)) as i32)) == (0i32)) as i32)) != 0) {
        break 'f (((F::BtlChar_TestFlag(c, l_chr_3, ((((68i32) as i64 as u64)) as u32))) != (0i32)) as i32);
    }
    l_canDodge_6 = 0i32;
    l_rush_7 = 0i32;
    if (((((c.m.r32((p_hit_2).wrapping_add(100))) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    l_slot_5 = { let a_ = (c.m.r32((p_hit_2).wrapping_add(100))).wrapping_add(4); (c.m.r32(a_) as i32) };
    l_atk_4 = { let a0_ = { let a_ = c.m.r32((p_hit_2).wrapping_add(100)); (c.m.r32(a_) as i32) }; F::BtlChar_Get(c, a0_) };
    if (((((c.m.r32((p_hit_2).wrapping_add(100))) != (((0i32) as u32))) as i32)) != 0) {
        's1: {
            let sv12: i32 = l_slot_5;
            match sv12 {
                0i32..=1i32 => {
                    if ((F::BtlChar_TestFlag(c, l_chr_3, ((((69i32) as i64 as u64)) as u32))) != 0) {
                        break 'f 1i32;
                    }
                    if ((((F::BtlSkill_GetFlags(c, l_atk_4, l_slot_5)) & (((4194304i32) as u32)))) != 0) {
                        l_canDodge_6 = 1i32;
                    }
                    break 's1;
                }
                2i32..=4i32 => {
                    if ((F::BtlChar_TestFlag(c, l_chr_3, ((((70i32) as i64 as u64)) as u32))) != 0) {
                        break 'f 1i32;
                    }
                    l_canDodge_6 = 1i32;
                    l_rush_7 = 1i32;
                    break 's1;
                }
                _ => {
                }
            }
        }
    }
    if ((((!((F::BtlChar_IsFree(c, l_chr_3)) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((((((!(((({ let a0_ = F::BtlAnim_GetId(c, l_chr_3); F::BtlAnim_GetFlags(c, ((((a0_) as i64 as u64)) as u32)) }) & (((177i32) as u32)))) != 0)) as i32)) != 0) || ((F::EftHit_GetHitCount(c, p_hit_2)) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((l_canDodge_6) != 0) {
        l_ok_8 = 0i32;
        l_free_9 = 0i32;
        l_used_10 = 0i32;
        if ((((((c.m.r32((l_chr_3).wrapping_add(4204)) as i32)) > (0i32)) as i32)) != 0) {
            l_ok_8 = 1i32;
            l_free_9 = 1i32;
        } else {
            if ((((((c.m.r32((l_chr_3).wrapping_add(3584)) as i32)) > (0i32)) as i32)) != 0) {
                { let a_ = (l_chr_3).wrapping_add(3584); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                l_ok_8 = 1i32;
                l_used_10 = 1i32;
            }
            if ((((((c.m.r32((l_chr_3).wrapping_add(3588)) as i32)) > (0i32)) as i32)) != 0) {
                { let a_ = (l_chr_3).wrapping_add(3588); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                l_ok_8 = 1i32;
                l_used_10 = 1i32;
            }
            if ((F::BtlChar_TestFlag(c, l_chr_3, ((((86i32) as i64 as u64)) as u32))) != 0) {
                l_ok_8 = 1i32;
                l_used_10 = 1i32;
            }
            if ((F::BtlChar_TestFlag(c, l_chr_3, ((((87i32) as i64 as u64)) as u32))) != 0) {
                l_ok_8 = 1i32;
            }
        }
        if ((l_ok_8) != 0) {
            l_side_11 = F::BtlChar_FrameMod(c, 2i32);
            if ((F::BtlInput_IsHeld(c, l_chr_3, ((64i32) as u32))) != 0) {
                l_side_11 = 0i32;
            }
            if ((F::BtlInput_IsHeld(c, l_chr_3, ((128i32) as u32))) != 0) {
                l_side_11 = 1i32;
            }
            's2: {
                let sv13: i32 = l_side_11;
                match sv13 {
                    0i32 => {
                        F::BtlChar_SetFlag(c, l_chr_3, ((((116i32) as i64 as u64)) as u32));
                        break 's2;
                    }
                    1i32 => {
                        F::BtlChar_SetFlag(c, l_chr_3, ((((117i32) as i64 as u64)) as u32));
                        break 's2;
                    }
                    _ => {
                    }
                }
            }
            F::BtlChar_PlayVoice(c, l_chr_3, 31i32);
            if ((l_rush_7) != 0) {
                if ((((!((F::BtlChar_TestFlag(c, l_chr_3, ((((128i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) {
                    if ((l_free_9) != 0) {
                        if ((F::BtlMember_HasAbility(c, l_chr_3, 34i32)) != 0) {
                            F::BtlMember_AddBlast(c, l_chr_3, 100000i32);
                        }
                    }
                }
                F::BtlChar_SetHeldFlag(c, l_chr_3, ((((128i32) as i64 as u64)) as u32));
            }
            if ((l_used_10) != 0) {
                's3: {
                    let sv14: i32 = (c.m.r32((l_chr_3).wrapping_add(3596)) as i32);
                    match sv14 {
                        0i32 => {
                            F::BtlChar_SetFlag(c, l_chr_3, ((((222i32) as i64 as u64)) as u32));
                            break 's3;
                        }
                        1i32 => {
                            F::BtlChar_SetFlag(c, l_chr_3, ((((223i32) as i64 as u64)) as u32));
                            break 's3;
                        }
                        _ => {
                        }
                    }
                }
            }
            break 'f 1i32;
        }
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlColl_TryDodge", &PROBE_N, &[], true); }
    r_
}

pub fn BtlColl_TryDeflect(c: &mut Ctx, mut p_objId_1: i32, mut p_hit_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlColl_TryDeflect", &PROBE_N, &[(p_objId_1) as u32, (p_hit_2) as u32], false); }
    let mut l_chr_3: u32 = 0;
    let mut l_perfect_4: i32 = 0;
    let mut l_vel_5: u32 = 0;
    let mut l_len_6: F32 = F32(0);
    let r_ = 'f: {
    l_perfect_4 = 0i32;
    l_chr_3 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_3) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((((c.m.r32((p_hit_2).wrapping_add(12)) as i32)) != (0i32)) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((((c.m.r32((p_hit_2).wrapping_add(84)) as i32)) == (1i32)) as i32)) != 0) {
        break 'f 0i32;
    }
    if (((((c.m.r32((p_hit_2).wrapping_add(104))) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((F::BtlKiBlast_GetFlagsOfHit(c, p_hit_2)) & (2i32))) != 0) {
        break 'f 0i32;
    }
    if ((((!((F::BtlChar_IsFree(c, l_chr_3)) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((((((((((!((F::BtlChar_TestFlag(c, l_chr_3, ((((82i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) && ((((!((F::BtlChar_TestFlag(c, l_chr_3, ((((84i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0)) as i32)) != 0) && ((F::BtlAct_TestPoweredSkill(c, l_chr_3, ((((128i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) {
        's1: {
            let sv7: i32 = F::BtlKiBlast_GetTypeOfHit(c, p_hit_2);
            match sv7 {
                2i32..=3i32 => {
                    { let a1_ = (if ((F::BtlChar_FrameMod(c, 2i32)) != 0) { 12i32 } else { 13i32 }); F::BtlCharSnd_PlayCommon(c, l_chr_3, a1_) };
                    break 's1;
                }
                _ => {
                    F::BtlCharSnd_PlayCommon(c, l_chr_3, 19i32);
                    break 's1;
                }
            }
        }
        break 'f 1i32;
    }
    if (({ let a1_ = F::BtlColl_GetHitDir(c, l_chr_3, p_hit_2, ((0i32) as u32)); F::BtlColl_IsFacing(c, l_chr_3, a1_) }) != 0) {
        break 'f 0i32;
    }
    if ((F::BtlChar_TestFlag(c, l_chr_3, ((((65i32) as i64 as u64)) as u32))) != 0) {
        if ((((((c.m.r32((l_chr_3).wrapping_add(4212)) as i32)) > (0i32)) as i32)) != 0) {
            if ((((!((((((((F::BtlParam_GetFlags(c, l_chr_3)) as u64)) as i32)) & (2i32))) != 0)) as i32)) != 0) {
                l_perfect_4 = 1i32;
            }
        }
    }
    if ((((!((l_perfect_4) != 0)) as i32)) != 0) {
        's2: {
            let sv8: i32 = (({ let a_ = (c.m.r32((p_hit_2).wrapping_add(104))).wrapping_add(24); (c.m.r16(a_) as i16) }) as i32);
            match sv8 {
                0i32 | 4i32 | 8i32 => {
                    if ((((!((F::BtlChar_TestFlag(c, l_chr_3, ((((80i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) {
                        break 'f 0i32;
                    }
                    break 's2;
                }
                _ => {
                    if ((((!((F::BtlChar_TestFlag(c, l_chr_3, ((((81i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) {
                        break 'f 0i32;
                    }
                    break 's2;
                }
            }
        }
    }
    's3: {
        let sv9: i32 = F::BtlKiBlast_GetTypeOfHit(c, p_hit_2);
        match sv9 {
            2i32..=3i32 => {
                { let a1_ = (if ((F::BtlChar_FrameMod(c, 2i32)) != 0) { 12i32 } else { 13i32 }); F::BtlCharSnd_PlayCommon(c, l_chr_3, a1_) };
                break 's3;
            }
            _ => {
                F::BtlCharSnd_PlayCommon(c, l_chr_3, 19i32);
                break 's3;
            }
        }
    }
    F::BtlChar_PlayVoice(c, l_chr_3, 11i32);
    F::BtlMember_AddBlast(c, l_chr_3, 5000i32);
    if ((l_perfect_4) != 0) {
        F::BtlChar_SetFlag(c, l_chr_3, ((((107i32) as i64 as u64)) as u32));
    }
    l_vel_5 = (p_hit_2).wrapping_add(64);
    l_len_6 = F::Vec3_Length(c, l_vel_5);
    if (((rt::flt(l_len_6, F32(0x3a83126e)) as i32)) != 0) {
        { let a1_ = (F::BtlChar_GetPos(c, l_chr_3)).wrapping_add(128); F::Vec4_Copy(c, (l_chr_3).wrapping_add(4960), a1_) };
    } else {
        F::Vec4_Scale(c, (l_chr_3).wrapping_add(4960), l_vel_5, rt::fdiv(F32(0xbf800000), l_len_6));
    }
    F::BtlChar_SetFxBit(c, l_chr_3, 46i32);
    { let a0_ = (c.m.r32(l_chr_3) as i32); F::BtlEvent_Raise(c, a0_, 88i32) };
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlColl_TryDeflect", &PROBE_N, &[], true); }
    r_
}

pub fn BtlColl_TryReflect(c: &mut Ctx, mut p_objId_1: i32, mut p_hit_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlColl_TryReflect", &PROBE_N, &[(p_objId_1) as u32, (p_hit_2) as u32], false); }
    let mut l_chr_3: u32 = 0;
    let mut l_vel_4: u32 = 0;
    let mut l_len_5: F32 = F32(0);
    let r_ = 'f: {
    l_chr_3 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_3) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((((c.m.r32((p_hit_2).wrapping_add(12)) as i32)) != (0i32)) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((((c.m.r32((p_hit_2).wrapping_add(84)) as i32)) == (1i32)) as i32)) != 0) {
        break 'f 0i32;
    }
    if (((((c.m.r32((p_hit_2).wrapping_add(104))) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((F::BtlKiBlast_GetFlagsOfHit(c, p_hit_2)) & (2i32))) != 0) {
        break 'f 0i32;
    }
    if ((((!((F::BtlChar_TestFlag(c, l_chr_3, ((((82i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((!((F::BtlChar_IsFree(c, l_chr_3)) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    if (({ let a1_ = F::BtlColl_GetHitDir(c, l_chr_3, p_hit_2, ((0i32) as u32)); F::BtlColl_IsFacing(c, l_chr_3, a1_) }) != 0) {
        break 'f 0i32;
    }
    's1: {
        let sv6: i32 = F::BtlKiBlast_GetTypeOfHit(c, p_hit_2);
        match sv6 {
            2i32..=3i32 => {
                { let a1_ = (if ((F::BtlChar_FrameMod(c, 2i32)) != 0) { 12i32 } else { 13i32 }); F::BtlCharSnd_PlayCommon(c, l_chr_3, a1_) };
                break 's1;
            }
            _ => {
                F::BtlCharSnd_PlayCommon(c, l_chr_3, 20i32);
                break 's1;
            }
        }
    }
    F::BtlChar_PlayVoice(c, l_chr_3, 11i32);
    l_vel_4 = (p_hit_2).wrapping_add(64);
    l_len_5 = F::Vec3_Length(c, l_vel_4);
    if (((rt::flt(l_len_5, F32(0x3a83126e)) as i32)) != 0) {
        { let a1_ = (F::BtlChar_GetPos(c, l_chr_3)).wrapping_add(128); F::Vec4_Copy(c, (l_chr_3).wrapping_add(4960), a1_) };
    } else {
        F::Vec4_Scale(c, (l_chr_3).wrapping_add(4960), l_vel_4, rt::fdiv(F32(0xbf800000), l_len_5));
    }
    F::BtlChar_SetFxBit(c, l_chr_3, 46i32);
    { let a0_ = (c.m.r32(l_chr_3) as i32); F::BtlEvent_Raise(c, a0_, 88i32) };
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlColl_TryReflect", &PROBE_N, &[], true); }
    r_
}

pub fn BtlColl_TryAbsorb(c: &mut Ctx, mut p_objId_1: i32, mut p_hit_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlColl_TryAbsorb", &PROBE_N, &[(p_objId_1) as u32, (p_hit_2) as u32], false); }
    let mut l_chr_3: u32 = 0;
    let mut l_atk_4: u32 = 0;
    let mut l_slot_5: i32 = 0;
    let mut l_amount_6: i32 = 0;
    let mut l_alt_7: i32 = 0;
    let mut l_n_8: i32 = 0;
    let r_ = 'f: {
    l_amount_6 = 0i32;
    l_alt_7 = 0i32;
    l_chr_3 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_3) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((((c.m.r32((p_hit_2).wrapping_add(84)) as i32)) == (1i32)) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((!((F::BtlChar_TestFlag(c, l_chr_3, ((((84i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) {
        if ((F::BtlChar_TestFlag(c, l_chr_3, ((((65i32) as i64 as u64)) as u32))) != 0) {
            if ((((((c.m.r32((l_chr_3).wrapping_add(4212)) as i32)) > (0i32)) as i32)) != 0) {
                if ((((((((F::BtlParam_GetFlags(c, l_chr_3)) as u64)) as i32)) & (2i32))) != 0) {
                    l_alt_7 = ((((c.m.r32((p_hit_2).wrapping_add(12)) as i32)) == (0i32)) as i32);
                }
            }
        }
        if ((((!((l_alt_7) != 0)) as i32)) != 0) {
            break 'f 0i32;
        }
    }
    if ((((!((F::BtlChar_IsFree(c, l_chr_3)) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    if (({ let a1_ = F::BtlColl_GetHitDir(c, l_chr_3, p_hit_2, ((0i32) as u32)); F::BtlColl_IsFacing(c, l_chr_3, a1_) }) != 0) {
        break 'f 0i32;
    }
    if ((((((c.m.r32((p_hit_2).wrapping_add(12)) as i32)) == (0i32)) as i32)) != 0) {
        if ((((F::BtlKiBlast_GetFlagsOfHit(c, p_hit_2)) & (2i32))) != 0) {
            break 'f 0i32;
        }
        's1: {
            let sv9: i32 = F::BtlKiBlast_GetTypeOfHit(c, p_hit_2);
            match sv9 {
                2i32..=3i32 => {
                    break 'f 0i32;
                }
                _ => {
                }
            }
        }
        l_amount_6 = (F::BtlKiBlast_GetKiCostOfHit(c, p_hit_2)).wrapping_mul(3i32);
    } else {
        if (((((c.m.r32((p_hit_2).wrapping_add(100))) == (((0i32) as u32))) as i32)) != 0) {
            break 'f 0i32;
        }
        l_slot_5 = { let a_ = (c.m.r32((p_hit_2).wrapping_add(100))).wrapping_add(4); (c.m.r32(a_) as i32) };
        l_atk_4 = { let a0_ = { let a_ = c.m.r32((p_hit_2).wrapping_add(100)); (c.m.r32(a_) as i32) }; F::BtlChar_Get(c, a0_) };
        's2: {
            let sv10: i32 = l_slot_5;
            match sv10 {
                0i32..=1i32 => {
                    break 'f 0i32;
                }
                2i32..=4i32 => {
                    if ((((!((((F::BtlSuper_GetFlags(c, l_atk_4, l_slot_5)) & (((4i32) as u32)))) != 0)) as i32)) != 0) {
                        break 'f 0i32;
                    }
                    l_amount_6 = F::BtlSuper_GetKiCost(c, l_atk_4, l_slot_5);
                    l_n_8 = F::BtlSuper_GetShots(c, l_atk_4, l_slot_5);
                    { let r_ = F::BtlSuper_GetHitsB(c, l_atk_4, l_slot_5); let n_ = (l_n_8).wrapping_mul(r_); l_n_8 = n_; };
                    if (((((l_n_8) >= (2i32)) as i32)) != 0) {
                        { let r_ = l_n_8; let n_ = rt::div_i32(l_amount_6, r_); l_amount_6 = n_; };
                    }
                    break 's2;
                }
                _ => {
                }
            }
        }
    }
    F::BtlCharSnd_PlayCommon(c, l_chr_3, 55i32);
    if (((rt::fgt(rt::i2f(F::BtlMember_AddKi(c, l_chr_3, l_amount_6)), F32(0x00000000)) as i32)) != 0) {
        if ((((!((F::BtlChar_TestFlag(c, l_chr_3, ((((6i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) {
            F::BtlMember_AddMaxPower(c, l_chr_3, rt::div_i32((l_amount_6).wrapping_mul(30000i32), 100000i32));
        }
    }
    F::BtlChar_SetFxBit(c, l_chr_3, 55i32);
    if ((l_alt_7) != 0) {
        F::BtlChar_SetFlag(c, l_chr_3, ((((107i32) as i64 as u64)) as u32));
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlColl_TryAbsorb", &PROBE_N, &[], true); }
    r_
}

pub fn BtlColl_TryGuard(c: &mut Ctx, mut p_objId_1: i32, mut p_hit_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlColl_TryGuard", &PROBE_N, &[(p_objId_1) as u32, (p_hit_2) as u32], false); }
    let mut l_chr_3: u32 = 0;
    let mut l_ret_4: i32 = 0;
    let r_ = 'f: {
    l_ret_4 = 0i32;
    l_chr_3 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_3) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((F::BtlChar_TestFlag(c, l_chr_3, ((((89i32) as i64 as u64)) as u32))) != 0) {
        break 'f 1i32;
    }
    if ((((((c.m.r32((p_hit_2).wrapping_add(12)) as i32)) == (0i32)) as i32)) != 0) {
        l_ret_4 = F::BtlColl_GuardBlast(c, l_chr_3, p_hit_2);
    } else {
        if (((((c.m.r32((p_hit_2).wrapping_add(100))) != (((0i32) as u32))) as i32)) != 0) {
            's1: {
                let sv5: i32 = { let a_ = (c.m.r32((p_hit_2).wrapping_add(100))).wrapping_add(4); (c.m.r32(a_) as i32) };
                match sv5 {
                    0i32..=1i32 => {
                        l_ret_4 = F::BtlColl_GuardStrike(c, l_chr_3, p_hit_2);
                        break 's1;
                    }
                    2i32..=4i32 => {
                        l_ret_4 = F::BtlColl_GuardRush(c, l_chr_3, p_hit_2);
                        break 's1;
                    }
                    _ => {
                    }
                }
            }
        }
    }
    break 'f l_ret_4;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlColl_TryGuard", &PROBE_N, &[], true); }
    r_
}

pub fn BtlColl_Hit(c: &mut Ctx, mut p_objId_1: i32, mut p_hit_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlColl_Hit", &PROBE_N, &[(p_objId_1) as u32, (p_hit_2) as u32], false); }
    let mut l_chr_3: u32 = 0;
    let mut l_ret_4: i32 = 0;
    let r_ = 'f: {
    l_ret_4 = 0i32;
    l_chr_3 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_3) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    's1: {
        let sv5: i32 = (c.m.r32((l_chr_3).wrapping_add(4016)) as i32);
        match sv5 {
            9i32 | 10i32 | 11i32 | 12i32 | 17i32 | 19i32 | 20i32 | 21i32 | 22i32 | 29i32 | 30i32 | 34i32 | 37i32 | 39i32 | 40i32 | 41i32 => {
                break 'f 1i32;
            }
            _ => {
            }
        }
    }
    if ((((((c.m.r32((p_hit_2).wrapping_add(12)) as i32)) == (0i32)) as i32)) != 0) {
        l_ret_4 = F::BtlColl_HitByBlast(c, l_chr_3, p_hit_2);
    } else {
        if (((((c.m.r32((p_hit_2).wrapping_add(100))) != (((0i32) as u32))) as i32)) != 0) {
            's2: {
                let sv6: i32 = { let a_ = (c.m.r32((p_hit_2).wrapping_add(100))).wrapping_add(4); (c.m.r32(a_) as i32) };
                match sv6 {
                    0i32 | 1i32 => {
                        l_ret_4 = F::BtlColl_HitByStrike(c, l_chr_3, p_hit_2);
                        break 's2;
                    }
                    2i32 | 3i32 => {
                        l_ret_4 = F::BtlColl_HitByRush(c, l_chr_3, p_hit_2);
                        break 's2;
                    }
                    4i32 => {
                        l_ret_4 = F::BtlColl_HitByRush(c, l_chr_3, p_hit_2);
                        break 's2;
                    }
                    _ => {
                    }
                }
            }
        }
    }
    if ((l_ret_4) != 0) {
        { let a0_ = { let a0_ = (c.m.r32(p_hit_2) as i32); F::BtlChar_Get(c, a0_) }; F::BtlChar_RaiseFirstClash(c, a0_) };
    }
    break 'f l_ret_4;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlColl_Hit", &PROBE_N, &[], true); }
    r_
}

pub fn BtlColl_PlayContactSound(c: &mut Ctx, mut p_hit_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlColl_PlayContactSound", &PROBE_N, &[(p_hit_1) as u32], false); }
    let mut l_atk_2: u32 = 0;
    let mut l_slot_3: i32 = 0;
    let mut l_sound_4: i32 = 0;
    let r_ = 'f: {
    's1: {
        let sv5: i32 = (c.m.r32((p_hit_1).wrapping_add(12)) as i32);
        match sv5 {
            0i32 => {
                's2: {
                    let sv6: i32 = F::BtlKiBlast_GetTypeOfHit(c, p_hit_1);
                    match sv6 {
                        2i32 => {
                            l_sound_4 = 66i32;
                            break 's2;
                        }
                        3i32 => {
                            l_sound_4 = (if ((F::BtlChar_FrameMod(c, 2i32)) != 0) { 12i32 } else { 13i32 });
                            break 's2;
                        }
                        _ => {
                            l_sound_4 = (if ((F::BtlChar_FrameMod(c, 2i32)) != 0) { 27i32 } else { 17i32 });
                            break 's2;
                        }
                    }
                }
                if (((((l_sound_4) >= (0i32)) as i32)) != 0) {
                    F::BtlCharSnd_RequestAt(c, (p_hit_1).wrapping_add(32), 0i32, l_sound_4, F32(0x43480000), F32(0x44bb8000));
                }
                break 's1;
            }
            1i32 => {
                if (((((c.m.r32((p_hit_1).wrapping_add(100))) != (((0i32) as u32))) as i32)) != 0) {
                    l_atk_2 = { let a0_ = { let a_ = c.m.r32((p_hit_1).wrapping_add(100)); (c.m.r32(a_) as i32) }; F::BtlChar_Get(c, a0_) };
                    l_slot_3 = { let a_ = (c.m.r32((p_hit_1).wrapping_add(100))).wrapping_add(4); (c.m.r32(a_) as i32) };
                    's3: {
                        let sv7: i32 = l_slot_3;
                        match sv7 {
                            0i32..=1i32 => {
                                { let a2_ = ((F::BtlSkill_GetHitSound(c, l_atk_2, l_slot_3)) as u32); F::BtlColl_PlayHitSound(c, ((0i32) as u32), p_hit_1, a2_) };
                                break 's3;
                            }
                            2i32..=4i32 => {
                                { let a2_ = ((F::BtlSuper_GetHitSound(c, l_atk_2, l_slot_3, 1i32)) as u32); F::BtlColl_PlayHitSound(c, ((0i32) as u32), p_hit_1, a2_) };
                                break 's3;
                            }
                            _ => {
                            }
                        }
                    }
                }
                break 's1;
            }
            _ => {
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlColl_PlayContactSound", &PROBE_N, &[], true); }
    r_
}

pub fn BtlColl_UpdateGround(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlColl_UpdateGround", &PROBE_N, &[(p_chr_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_landed_2: i32 = 0;
    let mut l_obj_3: u32 = 0;
    let mut l_pose_4: u32 = 0;
    let mut l_g_5: u32 = 0;
    let mut l_margin_6: F32 = F32(0);
    let mut l_floorY_7: F32 = F32(0);
    let mut l_size_8: F32 = F32(0);
    let mut l_lo_9: F32 = F32(0);
    let mut l_hi_10: F32 = F32(0);
    let r_ = 'f: {
        l_landed_2 = 0i32;
        l_obj_3 = F::BtlChar_GetObj(c, p_chr_1);
        l_pose_4 = F::BtlChar_GetPos(c, p_chr_1);
        l_g_5 = (c.m.r32((l_obj_3).wrapping_add(5728))).wrapping_add(98352);
        F::BtlChar_GetSnapDelta(c, p_chr_1, fp, 0i32, 1i32);
        if (((rt::fgt(F32(c.m.r32((l_g_5).wrapping_add(4))), F32(0x47c35000)) as i32)) != 0) {
        } else {
            { let s_ = l_g_5; let d_ = (l_pose_4).wrapping_add(176); rt::memcpy(c, d_, s_, 16); };
            { let s_ = (l_g_5).wrapping_add(16); let d_ = (l_pose_4).wrapping_add(192); rt::memcpy(c, d_, s_, 16); };
            { let v_ = (c.m.r32((l_g_5).wrapping_add(32)) as i32); let a_ = (l_pose_4).wrapping_add(208); c.m.w32(a_, (v_) as u32); };
        }
        l_margin_6 = rt::fadd(F::BtlUtil_LengthXZ(c, fp), F32(0x3f800000));
        if ((((!((F::BtlChar_TestFlag(c, p_chr_1, ((((15i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) {
            l_margin_6 = F32(0x00000000);
        }
        if ((F::BtlChar_IsStage4Or27(c)) != 0) {
            if ((((!(((((c.m.r32((l_g_5).wrapping_add(32)) as i32)) & (128i32))) != 0)) as i32)) != 0) {
                l_margin_6 = F32(0x00000000);
            }
        }
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32))) != 0) {
            l_margin_6 = F32(0x00000000);
        }
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((88i32) as i64 as u64)) as u32))) != 0) {
            l_margin_6 = F32(0x00000000);
        }
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((36i32) as i64 as u64)) as u32))) != 0) {
            l_margin_6 = F32(0x00000000);
        }
        l_floorY_7 = rt::fsub(F32(c.m.r32(((l_pose_4).wrapping_add(176)).wrapping_add(4))), F32(c.m.r32(((l_pose_4).wrapping_add(32)).wrapping_add(4))));
        if (((rt::flt(rt::fsub(l_floorY_7, l_margin_6), F32(c.m.r32((l_pose_4).wrapping_add(4)))) as i32)) != 0) {
            { let v_ = l_floorY_7; let a_ = (l_pose_4).wrapping_add(4); c.m.w32(a_, (v_).0); };
            if (((rt::fgt(F32(c.m.r32((fp).wrapping_add(4))), F32(0x00000000)) as i32)) != 0) {
                { let v_ = F32(0x00000000); let a_ = (l_pose_4).wrapping_add(156); c.m.w32(a_, (v_).0); };
                l_landed_2 = 1i32;
            }
        }
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((16i32) as i64 as u64)) as u32))) != 0) {
            l_landed_2 = 1i32;
            { let v_ = F32(0x00000000); let a_ = (l_pose_4).wrapping_add(156); c.m.w32(a_, (v_).0); };
            { let v_ = rt::fsub(F32(c.m.r32(((l_pose_4).wrapping_add(176)).wrapping_add(4))), F32(c.m.r32(((l_pose_4).wrapping_add(32)).wrapping_add(4)))); let a_ = (l_pose_4).wrapping_add(4); c.m.w32(a_, (v_).0); };
        }
        if (((({ let a_ = (c.m.r32((l_obj_3).wrapping_add(5728))).wrapping_add(98336); (c.m.r32(a_) as i32) }) & (64i32))) != 0) {
            F::BtlChar_SetFlag(c, p_chr_1, ((((32i32) as i64 as u64)) as u32));
            F::BtlChar_ClearFlag(c, p_chr_1, ((((15i32) as i64 as u64)) as u32));
            if ((F::BtlChar_TestFlag(c, p_chr_1, ((((10i32) as i64 as u64)) as u32))) != 0) {
                if (((({ let a_ = (c.m.r32((l_obj_3).wrapping_add(5728))).wrapping_add(98336); (c.m.r32(a_) as i32) }) & (128i32))) != 0) {
                    { let _ = F::BtlMember_Damage(c, p_chr_1, 200i32, 0i32); };
                }
                if (((({ let a_ = (c.m.r32((l_obj_3).wrapping_add(5728))).wrapping_add(98336); (c.m.r32(a_) as i32) }) & (256i32))) != 0) {
                    { let _ = F::BtlMember_Damage(c, p_chr_1, 600i32, 0i32); };
                }
                if (((({ let a_ = (c.m.r32((l_obj_3).wrapping_add(5728))).wrapping_add(98336); (c.m.r32(a_) as i32) }) & (512i32))) != 0) {
                    { let _ = F::BtlMember_Damage(c, p_chr_1, 1000i32, 0i32); };
                }
            }
        } else {
            if ((l_landed_2) != 0) {
                F::BtlChar_SetHeldFlag(c, p_chr_1, ((((15i32) as i64 as u64)) as u32));
            } else {
                F::BtlChar_ClearFlag(c, p_chr_1, ((((15i32) as i64 as u64)) as u32));
            }
        }
        F::BtlChar_ClearFlag(c, p_chr_1, ((((17i32) as i64 as u64)) as u32));
        F::BtlChar_ClearFlag(c, p_chr_1, ((((18i32) as i64 as u64)) as u32));
        if ((F::BtlStage_GetWaterLevel(c, fp.wrapping_add(16))) != 0) {
            if (((rt::flt(F32(c.m.r32(fp.wrapping_add(16))), F32(c.m.r32((l_g_5).wrapping_add(4)))) as i32)) != 0) {
                l_size_8 = { let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); F::BtlCharApi_GetHeight(c, a0_) };
                if (((rt::flt(F32(c.m.r32(fp.wrapping_add(16))), rt::fsub(F32(c.m.r32((l_pose_4).wrapping_add(4))), rt::fmul(l_size_8, F32(0x3f000000)))) as i32)) != 0) {
                    F::BtlChar_SetHeldFlag(c, p_chr_1, ((((17i32) as i64 as u64)) as u32));
                }
                l_lo_9 = rt::fsub(F32(c.m.r32((l_pose_4).wrapping_add(4))), l_size_8);
                l_hi_10 = rt::fadd(F32(c.m.r32((l_pose_4).wrapping_add(4))), rt::fmul(rt::fmul(F32(c.m.r32((l_pose_4).wrapping_add(152))), F32(0x41f00000)), F32(0x3ca3d70a)));
                if (((((((rt::flt(l_lo_9, F32(c.m.r32(fp.wrapping_add(16)))) as i32)) != 0) && (((rt::flt(F32(c.m.r32(fp.wrapping_add(16))), l_hi_10) as i32)) != 0)) as i32)) != 0) {
                    F::BtlChar_SetHeldFlag(c, p_chr_1, ((((18i32) as i64 as u64)) as u32));
                }
            }
        }
        if ((F::BtlChar_IsFlagRaised(c, p_chr_1, ((((17i32) as i64 as u64)) as u32))) != 0) {
            { let r_ = F32(0x3f000000); let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(156); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        }
        if ((F::BtlChar_IsStage4Or27(c)) != 0) {
            if (((rt::flt(F32(c.m.r32(((l_pose_4).wrapping_add(176)).wrapping_add(4))), F32(0xc1200000)) as i32)) != 0) {
                F::BtlChar_ClearFlag(c, p_chr_1, ((((23i32) as i64 as u64)) as u32));
            }
            if (((({ let a_ = (c.m.r32((l_obj_3).wrapping_add(5728))).wrapping_add(98336); (c.m.r32(a_) as i32) }) & (32i32))) != 0) {
                if ((((!(((({ let a_ = (c.m.r32((l_obj_3).wrapping_add(5728))).wrapping_add(98400); (c.m.r32(a_) as i32) }) & (192i32))) != 0)) as i32)) != 0) {
                    F::BtlChar_SetHeldFlag(c, p_chr_1, ((((7i32) as i64 as u64)) as u32));
                }
            }
            if (((({ let a_ = (c.m.r32((l_obj_3).wrapping_add(5728))).wrapping_add(98336); (c.m.r32(a_) as i32) }) & (64i32))) != 0) {
                F::BtlChar_SetHeldFlag(c, p_chr_1, ((((7i32) as i64 as u64)) as u32));
            }
            if ((l_landed_2) != 0) {
                if ((((!(((((c.m.r32((l_g_5).wrapping_add(32)) as i32)) & (192i32))) != 0)) as i32)) != 0) {
                    F::BtlChar_SetHeldFlag(c, p_chr_1, ((((7i32) as i64 as u64)) as u32));
                }
            }
        }
        if (((({ let a_ = (c.m.r32((l_obj_3).wrapping_add(5728))).wrapping_add(98336); (c.m.r32(a_) as i32) }) & (32i32))) != 0) {
            F::BtlChar_SetHeldFlag(c, p_chr_1, ((((31i32) as i64 as u64)) as u32));
        } else {
            F::BtlChar_ClearFlag(c, p_chr_1, ((((31i32) as i64 as u64)) as u32));
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlColl_UpdateGround", &PROBE_N, &[], true); }
    r_
}

pub fn BtlColl_ClearActionBits(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlColl_ClearActionBits", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = ((0i32) as u64); let a_ = (p_chr_1).wrapping_add(4744); c.m.w64(a_, v_); };
    F::BtlColl_SetUnk1294(c, p_chr_1, (1i32).wrapping_neg());
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlColl_ClearActionBits", &PROBE_N, &[], true); }
    r_
}

pub fn BtlColl_AddActionBit(c: &mut Ctx, mut p_chr_1: u32, mut p_action_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlColl_AddActionBit", &PROBE_N, &[(p_chr_1) as u32, (p_action_2) as u32], false); }
    let mut l_bit_3: u64 = 0;
    let r_ = 'f: {
    l_bit_3 = ((0i32) as u64);
    's1: {
        let sv4: i32 = p_action_2;
        match sv4 {
            112i32 => {
                l_bit_3 = ((1i32) as u64);
                break 's1;
            }
            115i32 => {
                l_bit_3 = ((2i32) as u64);
                break 's1;
            }
            116i32 => {
                l_bit_3 = ((4i32) as u64);
                break 's1;
            }
            119i32 => {
                l_bit_3 = ((8i32) as u64);
                break 's1;
            }
            126i32 => {
                l_bit_3 = ((16i32) as u64);
                break 's1;
            }
            128i32 => {
                l_bit_3 = ((32i32) as u64);
                break 's1;
            }
            130i32 => {
                l_bit_3 = ((64i32) as u64);
                break 's1;
            }
            123i32 | 124i32 => {
                l_bit_3 = ((134217728i32) as u64);
                break 's1;
            }
            25i32 => {
                l_bit_3 = ((128i32) as u64);
                break 's1;
            }
            90i32 | 91i32 | 92i32 | 93i32 => {
                l_bit_3 = ((256i32) as u64);
                break 's1;
            }
            68i32 => {
                l_bit_3 = ((512i32) as u64);
                break 's1;
            }
            71i32 | 72i32 | 73i32 | 74i32 | 75i32 | 76i32 => {
                l_bit_3 = ((1024i32) as u64);
                break 's1;
            }
            83i32 | 84i32 | 85i32 | 86i32 | 87i32 => {
                l_bit_3 = ((2048i32) as u64);
                break 's1;
            }
            107i32 | 108i32 | 109i32 | 110i32 | 111i32 => {
                l_bit_3 = 137438953472u64;
                break 's1;
            }
            174i32 => {
                l_bit_3 = ((4096i32) as u64);
                break 's1;
            }
            175i32 => {
                l_bit_3 = ((8192i32) as u64);
                break 's1;
            }
            176i32 => {
                l_bit_3 = ((16384i32) as u64);
                break 's1;
            }
            177i32 => {
                l_bit_3 = ((32768i32) as u64);
                break 's1;
            }
            178i32 => {
                l_bit_3 = ((65536i32) as u64);
                break 's1;
            }
            179i32 => {
                l_bit_3 = ((131072i32) as u64);
                break 's1;
            }
            94i32 => {
                l_bit_3 = ((262144i32) as u64);
                break 's1;
            }
            95i32 => {
                l_bit_3 = ((524288i32) as u64);
                break 's1;
            }
            96i32 => {
                l_bit_3 = ((1048576i32) as u64);
                break 's1;
            }
            97i32 => {
                l_bit_3 = ((2097152i32) as u64);
                break 's1;
            }
            98i32 => {
                l_bit_3 = ((4194304i32) as u64);
                break 's1;
            }
            99i32 => {
                l_bit_3 = ((8388608i32) as u64);
                break 's1;
            }
            102i32 => {
                l_bit_3 = ((16777216i32) as u64);
                break 's1;
            }
            100i32 | 101i32 => {
                l_bit_3 = ((33554432i32) as u64);
                break 's1;
            }
            132i32 | 133i32 => {
                l_bit_3 = 2147483648u64;
                break 's1;
            }
            180i32 => {
                l_bit_3 = ((67108864i32) as u64);
                break 's1;
            }
            182i32 => {
                l_bit_3 = ((1073741824i32) as u64);
                break 's1;
            }
            144i32 => {
                l_bit_3 = ((536870912i32) as u64);
                break 's1;
            }
            155i32 => {
                l_bit_3 = ((268435456i32) as u64);
                break 's1;
            }
            153i32 => {
                l_bit_3 = 4294967296u64;
                break 's1;
            }
            147i32 => {
                l_bit_3 = 8589934592u64;
                break 's1;
            }
            150i32 => {
                l_bit_3 = 17179869184u64;
                break 's1;
            }
            52i32 => {
                l_bit_3 = 34359738368u64;
                break 's1;
            }
            162i32 => {
                l_bit_3 = 68719476736u64;
                break 's1;
            }
            37i32 => {
                l_bit_3 = 2199023255552u64;
                break 's1;
            }
            38i32 => {
                l_bit_3 = 4398046511104u64;
                break 's1;
            }
            40i32 | 42i32 => {
                l_bit_3 = 8796093022208u64;
                break 's1;
            }
            215i32 => {
                l_bit_3 = 17592186044416u64;
                break 's1;
            }
            36i32 => {
                l_bit_3 = 35184372088832u64;
                break 's1;
            }
            66i32 => {
                l_bit_3 = 70368744177664u64;
                break 's1;
            }
            _ => {
                break 's1;
            }
        }
    }
    { let r_ = l_bit_3; let a_ = (p_chr_1).wrapping_add(4744); let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlColl_AddActionBit", &PROBE_N, &[], true); }
    r_
}

pub fn BtlColl_SetUnk1294(c: &mut Ctx, mut p_chr_1: u32, mut p_v_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlColl_SetUnk1294", &PROBE_N, &[(p_chr_1) as u32, (p_v_2) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_v_2; let a_ = (p_chr_1).wrapping_add(4756); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlColl_SetUnk1294", &PROBE_N, &[], true); }
    r_
}

pub fn BtlColl_SetFramesToReach(c: &mut Ctx, mut p_chr_1: u32, mut p_scale_2: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlColl_SetFramesToReach", &PROBE_N, &[(p_chr_1) as u32, (p_scale_2).0], false); }
    let mut l_len_3: F32 = F32(0);
    let mut l_have_4: F32 = F32(0);
    let mut l_rate_5: F32 = F32(0);
    let mut l_need_6: F32 = F32(0);
    let r_ = 'f: {
    l_len_3 = F::BtlAnim_GetLength(c, p_chr_1);
    l_have_4 = F::BtlAnim_GetFrame(c, p_chr_1);
    l_rate_5 = F::BtlAnim_GetStep(c, p_chr_1);
    l_need_6 = rt::fmul(l_len_3, p_scale_2);
    if (((rt::fle(l_need_6, l_have_4) as i32)) != 0) {
        break 'f;
    }
    if (((rt::flt(l_rate_5, F32(0x3c23d70a)) as i32)) != 0) {
        break 'f;
    }
    F::BtlColl_SetUnk1294(c, p_chr_1, rt::f2i(rt::fadd(rt::fdiv(rt::fsub(l_need_6, l_have_4), l_rate_5), F32(0x3f7d70a3))));
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlColl_SetFramesToReach", &PROBE_N, &[], true); }
    r_
}
