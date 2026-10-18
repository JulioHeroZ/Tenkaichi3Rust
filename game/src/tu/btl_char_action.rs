#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn BtlAct_Request(c: &mut Ctx, mut p_chr_1: u32, mut p_id_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_Request", &PROBE_N, &[(p_chr_1) as u32, (p_id_2) as u32], false); }
    let r_ = 'f: {
    if (((((p_id_2) != ((1i32).wrapping_neg())) as i32)) != 0) {
        { let v_ = p_id_2; let a_ = ((p_chr_1).wrapping_add(2376)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_Request", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_SetQueue(c: &mut Ctx, mut p_chr_1: u32, mut p_slot_2: u32, mut p_id_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_SetQueue", &PROBE_N, &[(p_chr_1) as u32, (p_slot_2) as u32, (p_id_3) as u32], false); }
    let mut l_i_4: i32 = 0;
    let r_ = 'f: {
    if (((((((((p_slot_2) < (((4i32) as u32))) as i32)) != 0) && (((((p_id_3) != ((1i32).wrapping_neg())) as i32)) != 0)) as i32)) != 0) {
        { let v_ = p_id_3; let a_ = (((p_chr_1).wrapping_add(2376)).wrapping_add(12)).wrapping_add((p_slot_2).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        {
            l_i_4 = (((p_slot_2).wrapping_add(((1i32) as u32))) as i32);
            'l1: loop {
                if !(((((l_i_4) < (4i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let v_ = (1i32).wrapping_neg(); let a_ = (((p_chr_1).wrapping_add(2376)).wrapping_add(12)).wrapping_add((((l_i_4) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                }
                { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_SetQueue", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_ClearQueue(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_ClearQueue", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < (4i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let v_ = (1i32).wrapping_neg(); let a_ = (((p_chr_1).wrapping_add(2376)).wrapping_add(12)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_ClearQueue", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_HasQueued(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_HasQueued", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f ((((c.m.r32((((p_chr_1).wrapping_add(2376)).wrapping_add(12)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) != ((1i32).wrapping_neg())) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_HasQueued", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_GetCurrent(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_GetCurrent", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f (c.m.r32((p_chr_1).wrapping_add(2376)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_GetCurrent", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_GetRequested(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_GetRequested", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f (c.m.r32(((p_chr_1).wrapping_add(2376)).wrapping_add(4)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_GetRequested", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_GetPrev(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_GetPrev", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f (c.m.r32(((p_chr_1).wrapping_add(2376)).wrapping_add(8)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_GetPrev", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_GetQueued(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_GetQueued", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f (c.m.r32((((p_chr_1).wrapping_add(2376)).wrapping_add(12)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_GetQueued", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_PopQueue(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_PopQueue", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < (3i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let v_ = (c.m.r32((((p_chr_1).wrapping_add(2376)).wrapping_add(12)).wrapping_add(((((l_i_2).wrapping_add(1i32)) as u32)).wrapping_mul(4))) as i32); let a_ = (((p_chr_1).wrapping_add(2376)).wrapping_add(12)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
    { let v_ = (1i32).wrapping_neg(); let a_ = (((p_chr_1).wrapping_add(2376)).wrapping_add(12)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_PopQueue", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_IsTechniqueId(c: &mut Ctx, mut p_id_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_IsTechniqueId", &PROBE_N, &[(p_id_1) as u32], false); }
    let r_ = 'f: {
    if ((((((((p_id_1).wrapping_sub(262i32)) as u32)) < (((45i32) as u32))) as i32)) != 0) {
        break 'f 1i32;
    }
    break 'f (((p_id_1) == (261i32)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_IsTechniqueId", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_GetIdClass(c: &mut Ctx, mut p_chr_1: u32, mut p_id_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_GetIdClass", &PROBE_N, &[(p_chr_1) as u32, (p_id_2) as u32], false); }
    let r_ = 'f: {
    if (((((p_id_2) == (261i32)) as i32)) != 0) {
        break 'f 4i32;
    }
    if ((((((((p_id_2).wrapping_sub(253i32)) as u32)) < (((6i32) as u32))) as i32)) != 0) {
        break 'f rt::rem_i32((p_id_2).wrapping_sub(253i32), 2i32);
    }
    if ((((((((p_id_2).wrapping_sub(262i32)) as u32)) < (((54i32) as u32))) as i32)) != 0) {
        break 'f (rt::rem_i32((p_id_2).wrapping_sub(262i32), 3i32)).wrapping_add(2i32);
    }
    break 'f (1i32).wrapping_neg();
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_GetIdClass", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_GetCurrentClass(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_GetCurrentClass", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f { let a1_ = F::BtlAct_GetCurrent(c, p_chr_1); F::BtlAct_GetIdClass(c, p_chr_1, a1_) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_GetCurrentClass", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_GetPrevClass(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_GetPrevClass", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f { let a1_ = F::BtlAct_GetPrev(c, p_chr_1); F::BtlAct_GetIdClass(c, p_chr_1, a1_) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_GetPrevClass", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_GetMotionLevel(c: &mut Ctx, mut p_chr_1: u32, mut p_motion_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_GetMotionLevel", &PROBE_N, &[(p_chr_1) as u32, (p_motion_2) as u32], false); }
    let r_ = 'f: {
    if ((((((((p_motion_2).wrapping_sub(284i32)) as u32)) < (((5i32) as u32))) as i32)) != 0) {
        break 'f (p_motion_2).wrapping_sub(284i32);
    }
    if ((((((((p_motion_2).wrapping_sub(318i32)) as u32)) < (((5i32) as u32))) as i32)) != 0) {
        break 'f (p_motion_2).wrapping_sub(318i32);
    }
    if ((((((((p_motion_2).wrapping_sub(352i32)) as u32)) < (((5i32) as u32))) as i32)) != 0) {
        break 'f (p_motion_2).wrapping_sub(352i32);
    }
    if ((((((((p_motion_2).wrapping_sub(289i32)) as u32)) < (((5i32) as u32))) as i32)) != 0) {
        break 'f (p_motion_2).wrapping_sub(289i32);
    }
    if ((((((((p_motion_2).wrapping_sub(323i32)) as u32)) < (((5i32) as u32))) as i32)) != 0) {
        break 'f (p_motion_2).wrapping_sub(323i32);
    }
    if ((((((((p_motion_2).wrapping_sub(357i32)) as u32)) < (((5i32) as u32))) as i32)) != 0) {
        break 'f (p_motion_2).wrapping_sub(357i32);
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_GetMotionLevel", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_GetGroundY(c: &mut Ctx, mut p_chr_1: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_GetGroundY", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f { let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(180); F32(c.m.r32(a_)) };
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_GetGroundY", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_GetHeight(c: &mut Ctx, mut p_chr_1: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_GetHeight", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_ground_2: F32 = F32(0);
    let r_ = 'f: {
    l_ground_2 = F::BtlAct_GetGroundY(c, p_chr_1);
    break 'f rt::fsub(l_ground_2, { let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(4); F32(c.m.r32(a_)) });
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_GetHeight", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_GetHeightRatio(c: &mut Ctx, mut p_chr_1: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_GetHeightRatio", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_half_2: F32 = F32(0);
    let mut l_top_3: F32 = F32(0);
    let mut l_bottom_4: F32 = F32(0);
    let mut l_ground_5: F32 = F32(0);
    let mut l_pose_6: u32 = 0;
    let mut l_y_7: F32 = F32(0);
    let r_ = 'f: {
    l_half_2 = { let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); F::BtlCharApi_GetHeight(c, a0_) };
    l_top_3 = rt::fadd(F::BtlStage_GetTop(c), rt::fmul(l_half_2, F32(0x3f000000)));
    l_bottom_4 = rt::fadd(F::BtlStage_GetBottom(c), rt::fmul(l_half_2, F32(0x3f000000)));
    l_ground_5 = F::BtlAct_GetGroundY(c, p_chr_1);
    l_pose_6 = F::BtlChar_GetPos(c, p_chr_1);
    l_y_7 = F32(c.m.r32((l_pose_6).wrapping_add(4)));
    if (((rt::flt(l_bottom_4, l_ground_5) as i32)) != 0) {
        l_ground_5 = l_bottom_4;
    }
    break 'f rt::fdiv(rt::fsub(l_ground_5, l_y_7), rt::fsub(l_ground_5, l_top_3));
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_GetHeightRatio", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_GetFramesToGround(c: &mut Ctx, mut p_chr_1: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_GetFramesToGround", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_fall_2: F32 = F32(0);
    let mut l_height_3: F32 = F32(0);
    let r_ = 'f: {
    if ((F::BtlChar_TestFlag(c, p_chr_1, ((((15i32) as i64 as u64)) as u32))) != 0) {
        break 'f F32(0x00000000);
    }
    l_fall_2 = { let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(68); F32(c.m.r32(a_)) };
    if (((rt::flt(l_fall_2, F32(0x3a83126e)) as i32)) != 0) {
        break 'f F32(0x49742400);
    }
    l_height_3 = F::BtlAct_GetHeight(c, p_chr_1);
    if (((rt::flt(l_height_3, F32(0x00000000)) as i32)) != 0) {
        break 'f F32(0x00000000);
    }
    break 'f rt::fdiv(l_height_3, l_fall_2);
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_GetFramesToGround", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_GetRotYRelCam(c: &mut Ctx, mut p_chr_1: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_GetRotYRelCam", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f { let a0_ = rt::fsub({ let a_ = ((F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(16)).wrapping_add(4); F32(c.m.r32(a_)) }, F32(c.m.r32((p_chr_1).wrapping_add(1184)))); F::BtlUtil_WrapAngle(c, a0_) };
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_GetRotYRelCam", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_GetFacingRelCam(c: &mut Ctx, mut p_chr_1: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_GetFacingRelCam", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f { let a0_ = rt::fsub({ let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(148); F32(c.m.r32(a_)) }, F32(c.m.r32((p_chr_1).wrapping_add(1184)))); F::BtlUtil_WrapAngle(c, a0_) };
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_GetFacingRelCam", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_GetVelDirRelCam(c: &mut Ctx, mut p_chr_1: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_GetVelDirRelCam", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_pose_2: u32 = 0;
    let r_ = 'f: {
    l_pose_2 = F::BtlChar_GetPos(c, p_chr_1);
    break 'f { let a0_ = rt::fsub({ let a0_ = F32(c.m.r32((l_pose_2).wrapping_add(128))); let a1_ = F32(c.m.r32(((l_pose_2).wrapping_add(128)).wrapping_add(8))); F::Port_atan2f(c, a0_, a1_) }, F32(c.m.r32((p_chr_1).wrapping_add(1184)))); F::BtlUtil_WrapAngle(c, a0_) };
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_GetVelDirRelCam", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_ScaleSpeedByApproach(c: &mut Ctx, mut p_chr_1: u32, mut p_speed_2: F32, mut p_range_3: F32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_ScaleSpeedByApproach", &PROBE_N, &[(p_chr_1) as u32, (p_speed_2).0, (p_range_3).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_dist_4: F32 = F32(0);
    let mut l_along_5: F32 = F32(0);
    let r_ = 'f: {
        if ((((!((F::BtlChar_TestFlag(c, p_chr_1, ((((5i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) {
            break 'f p_speed_2;
        }
        F::BtlOpp_GetDelta(c, p_chr_1, fp);
        { let v_ = F32(0x00000000); let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
        l_dist_4 = F::Vec3_Length(c, fp);
        if (((rt::flt(l_dist_4, F32(0x3c23d70a)) as i32)) != 0) {
            l_dist_4 = F32(0x3c23d70a);
        }
        if (((rt::flt(l_dist_4, p_range_3) as i32)) != 0) {
            { let a1_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(128); F::Vec3_Normalize(c, fp.wrapping_add(16), a1_) };
            { let v_ = F32(0x00000000); let a_ = (fp.wrapping_add(16)).wrapping_add(4); c.m.w32(a_, (v_).0); };
            l_along_5 = { let a0_ = rt::fdiv(F::Vec3_Dot(c, fp.wrapping_add(16), fp), l_dist_4); rt::ffabs(a0_) };
            p_speed_2 = rt::fadd(rt::fmul(p_speed_2, l_along_5), rt::fmul(rt::fmul(p_speed_2, rt::fsub(F32(0x3f800000), l_along_5)), rt::fdiv(l_dist_4, p_range_3)));
        }
        break 'f p_speed_2;
       F32(0)
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_ScaleSpeedByApproach", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_ScaleSpeedByDist(c: &mut Ctx, mut p_chr_1: u32, mut p_speed_2: F32, mut p_range_3: F32, mut p_frames_4: F32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_ScaleSpeedByDist", &PROBE_N, &[(p_chr_1) as u32, (p_speed_2).0, (p_range_3).0, (p_frames_4).0], false); }
    let mut l_limit_5: F32 = F32(0);
    let mut l_travel_6: F32 = F32(0);
    let mut l_scale_7: F32 = F32(0);
    let r_ = 'f: {
    if ((((!((F::BtlChar_TestFlag(c, p_chr_1, ((((5i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) {
        break 'f p_speed_2;
    }
    l_travel_6 = rt::fmul(p_speed_2, p_frames_4);
    l_limit_5 = rt::fmul(p_range_3, F::BtlOpp_GetDistanceXZ(c, p_chr_1));
    l_scale_7 = F32(0x3f800000);
    if (((((((rt::flt(l_limit_5, l_travel_6) as i32)) != 0) && (((rt::flt(F32(0x3c23d70a), l_travel_6) as i32)) != 0)) as i32)) != 0) {
        l_scale_7 = rt::fdiv(l_limit_5, l_travel_6);
    }
    break 'f rt::fmul(p_speed_2, l_scale_7);
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_ScaleSpeedByDist", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_SetPitchMotion(c: &mut Ctx, mut p_chr_1: u32, mut p_motionUp_2: i32, mut p_motionDown_3: i32, mut p_recalc_4: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_SetPitchMotion", &PROBE_N, &[(p_chr_1) as u32, (p_motionUp_2) as u32, (p_motionDown_3) as u32, (p_recalc_4) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let mut l_len_5: F32 = F32(0);
    let mut l_t_6: F32 = F32(0);
    let r_ = 'f: {
        if ((((!((F::BtlChar_TestFlag(c, p_chr_1, ((((5i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) {
            { let v_ = F32(0x00000000); let a_ = (p_chr_1).wrapping_add(2412); c.m.w32(a_, (v_).0); };
            break 'f;
        }
        if ((p_recalc_4) != 0) {
            { let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); F::BtlCharApi_GetNodePos(c, a0_, 17i32, fp) };
            F::BtlOpp_GetNodePos(c, p_chr_1, 17i32, fp.wrapping_add(16));
            F::Vec4_Sub(c, fp.wrapping_add(32), fp.wrapping_add(16), fp);
            l_len_5 = F::Vec3_Length(c, fp.wrapping_add(32));
            if (((rt::flt(F32(0x38d1b717), l_len_5) as i32)) != 0) {
                F::Vec4_Scale(c, fp.wrapping_add(32), fp.wrapping_add(32), rt::fdiv(F32(0x3f800000), l_len_5));
            }
            { let v_ = { let a0_ = F32(c.m.r32((fp.wrapping_add(32)).wrapping_add(4))); F::BtlUtil_ClampF(c, a0_, F32(0xbf800000), F32(0x3f800000)) }; let a_ = (fp.wrapping_add(32)).wrapping_add(4); c.m.w32(a_, (v_).0); };
            { let v_ = rt::fneg({ let a0_ = F32(c.m.r32((fp.wrapping_add(32)).wrapping_add(4))); F::Mathf_Asin(c, a0_) }); let a_ = (p_chr_1).wrapping_add(2412); c.m.w32(a_, (v_).0); };
        }
        l_t_6 = rt::fdiv(F32(c.m.r32((p_chr_1).wrapping_add(2412))), F32(0x3f490fda));
        if (((rt::flt(l_t_6, F32(0xbf800000)) as i32)) != 0) {
            l_t_6 = F32(0xbf800000);
        }
        if (((rt::flt(F32(0x3f800000), l_t_6) as i32)) != 0) {
            l_t_6 = F32(0x3f800000);
        }
        if (((rt::flt(l_t_6, F32(0x00000000)) as i32)) != 0) {
            F::BtlAnim_SetUnkC8C(c, p_chr_1, rt::fneg(l_t_6));
            F::BtlAnim_PlaySub(c, p_chr_1, p_motionDown_3);
        } else {
            F::BtlAnim_SetUnkC8C(c, p_chr_1, l_t_6);
            F::BtlAnim_PlaySub(c, p_chr_1, p_motionUp_2);
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_SetPitchMotion", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_IsAirMotion(c: &mut Ctx, mut p_chr_1: u32, mut p_useSaved_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_IsAirMotion", &PROBE_N, &[(p_chr_1) as u32, (p_useSaved_2) as u32], false); }
    let r_ = 'f: {
    if ((F::BtlChar_TestFlag(c, p_chr_1, ((((53i32) as i64 as u64)) as u32))) != 0) {
        break 'f 1i32;
    }
    if ((F::BtlChar_TestFlag(c, p_chr_1, ((((54i32) as i64 as u64)) as u32))) != 0) {
        break 'f 0i32;
    }
    if ((((((p_useSaved_2) != 0) && ((((((c.m.r32((p_chr_1).wrapping_add(4016)) as i32)) != (1i32)) as i32)) != 0)) as i32)) != 0) {
        break 'f (c.m.r32((p_chr_1).wrapping_add(4028)) as i32);
    }
    if (((({ let a0_ = F::BtlAnim_GetId(c, p_chr_1); F::BtlAnim_GetFlags(c, ((((a0_) as i64 as u64)) as u32)) }) & (((32768i32) as u32)))) != 0) {
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_IsAirMotion", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_PrepareSwitch(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_PrepareSwitch", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = F::BtlMember_GetSwitchTarget(c, p_chr_1); let a_ = (p_chr_1).wrapping_add(3288); c.m.w32(a_, (v_) as u32); };
    { let v_ = { let a_ = { let a1_ = (c.m.r32((p_chr_1).wrapping_add(3288)) as i32); F::BtlMember_Get(c, p_chr_1, a1_) }; (c.m.r32(a_) as i32) }; let a_ = (p_chr_1).wrapping_add(3292); c.m.w32(a_, (v_) as u32); };
    { let v_ = { let a_ = ({ let a1_ = (c.m.r32((p_chr_1).wrapping_add(3288)) as i32); F::BtlMember_Get(c, p_chr_1, a1_) }).wrapping_add(4); (c.m.r32(a_) as i32) }; let a_ = (p_chr_1).wrapping_add(3296); c.m.w32(a_, (v_) as u32); };
    { let v_ = { let a_ = ({ let a1_ = (c.m.r32((p_chr_1).wrapping_add(3288)) as i32); F::BtlMember_GetGauge(c, p_chr_1, a1_) }).wrapping_add(32); (c.m.r32(a_) as i32) }; let a_ = (p_chr_1).wrapping_add(3300); c.m.w32(a_, (v_) as u32); };
    if ((((({ let a_ = ({ let a1_ = (c.m.r32((p_chr_1).wrapping_add(3288)) as i32); F::BtlMember_GetGauge(c, p_chr_1, a1_) }).wrapping_add(48); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
        { let v_ = 86i32; let a_ = (p_chr_1).wrapping_add(3304); c.m.w32(a_, (v_) as u32); };
        { let v_ = 86i32; let a_ = (p_chr_1).wrapping_add(3308); c.m.w32(a_, (v_) as u32); };
        { let v_ = 86i32; let a_ = (p_chr_1).wrapping_add(3312); c.m.w32(a_, (v_) as u32); };
    } else {
        { let v_ = (c.m.r32((p_chr_1).wrapping_add(3292)) as i32); let a_ = (p_chr_1).wrapping_add(3304); c.m.w32(a_, (v_) as u32); };
        { let v_ = (c.m.r32((p_chr_1).wrapping_add(3292)) as i32); let a_ = (p_chr_1).wrapping_add(3308); c.m.w32(a_, (v_) as u32); };
        { let v_ = (c.m.r32((p_chr_1).wrapping_add(3292)) as i32); let a_ = (p_chr_1).wrapping_add(3312); c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_PrepareSwitch", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_PushAngle(c: &mut Ctx, mut p_chr_1: u32, mut p_angle_2: F32, mut p_speed_3: F32, mut p_arg_4: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_PushAngle", &PROBE_N, &[(p_chr_1) as u32, (p_angle_2).0, (p_speed_3).0, (p_arg_4).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        { let v_ = rt::fmul(F::Mathf_Sin(c, p_angle_2), p_speed_3); let a_ = fp; c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(F::Mathf_Cos(c, p_angle_2), p_speed_3); let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (fp).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let a0_ = F::BtlChar_GetObj(c, p_chr_1); F::BtlObj_AddPush(c, a0_, fp, p_arg_4) };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_PushAngle", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_PushDir(c: &mut Ctx, mut p_chr_1: u32, mut p_dir_2: u32, mut p_speed_3: F32, mut p_arg_4: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_PushDir", &PROBE_N, &[(p_chr_1) as u32, (p_dir_2) as u32, (p_speed_3).0, (p_arg_4).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_len_5: F32 = F32(0);
    let r_ = 'f: {
        l_len_5 = F::Vec3_Length(c, p_dir_2);
        if ((((!(((rt::flt(l_len_5, F32(0x38d1b717)) as i32)) != 0)) as i32)) != 0) {
            F::Vec4_Scale(c, fp, p_dir_2, rt::fdiv(F32(0x3f800000), l_len_5));
            F::Vec4_Scale(c, fp, fp, p_speed_3);
            { let a0_ = F::BtlChar_GetObj(c, p_chr_1); F::BtlObj_AddPush(c, a0_, fp, p_arg_4) };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_PushDir", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_SetObjUnk(c: &mut Ctx, mut p_chr_1: u32, mut p_a_2: F32, mut p_b_3: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_SetObjUnk", &PROBE_N, &[(p_chr_1) as u32, (p_a_2).0, (p_b_3).0], false); }
    let r_ = 'f: {
    { let a0_ = F::BtlChar_GetObj(c, p_chr_1); F::BtlObj_AddSway(c, a0_, p_a_2, p_b_3) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_SetObjUnk", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_IsAttackId(c: &mut Ctx, mut p_id_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_IsAttackId", &PROBE_N, &[(p_id_1) as u32], false); }
    let r_ = 'f: {
    break 'f ((((((p_id_1).wrapping_sub(112i32)) as u32)) < (((62i32) as u32))) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_IsAttackId", &PROBE_N, &[], true); }
    r_
}

pub fn btl_char_action__BtlAct_GetAttackRec(c: &mut Ctx, mut p_attack_1: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "btl_char_action__BtlAct_GetAttackRec", &PROBE_N, &[(p_attack_1) as u32], false); }
    let r_ = 'f: {
    break 'f ({ let a_ = (c.m.r32(sym::gBtlChars)).wrapping_add(44); c.m.r32(a_) }).wrapping_add((((p_attack_1) as u32)).wrapping_mul(24));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "btl_char_action__BtlAct_GetAttackRec", &PROBE_N, &[], true); }
    r_
}

pub fn btl_char_action__BtlAct_TestSkillLo(c: &mut Ctx, mut p_chr_1: u32, mut p_bit_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "btl_char_action__BtlAct_TestSkillLo", &PROBE_N, &[(p_chr_1) as u32, (p_bit_2) as u32], false); }
    let r_ = 'f: {
    break 'f ((((F::BtlParam_GetFlags2(c, p_chr_1)) & ((((1i32).wrapping_shl((p_bit_2) as u32)) as u32)))) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "btl_char_action__BtlAct_TestSkillLo", &PROBE_N, &[], true); }
    r_
}

pub fn btl_char_action__BtlAct_TestSkillHi(c: &mut Ctx, mut p_chr_1: u32, mut p_bit_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "btl_char_action__BtlAct_TestSkillHi", &PROBE_N, &[(p_chr_1) as u32, (p_bit_2) as u32], false); }
    let r_ = 'f: {
    break 'f ((((F::BtlParam_GetFlags3(c, p_chr_1)) & ((((1i32).wrapping_shl((p_bit_2) as u32)) as u32)))) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "btl_char_action__BtlAct_TestSkillHi", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_TestAttackSkill(c: &mut Ctx, mut p_chr_1: u32, mut p_attack_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_TestAttackSkill", &PROBE_N, &[(p_chr_1) as u32, (p_attack_2) as u32], false); }
    let mut l_bit_3: i32 = 0;
    let r_ = 'f: {
    l_bit_3 = (({ let a_ = (F::btl_char_action__BtlAct_GetAttackRec(c, p_attack_2)).wrapping_add(19); (c.m.r8(a_) as i8) }) as i32);
    if (((((l_bit_3) >= (0i32)) as i32)) != 0) {
        if (((((((l_bit_3) as u32)) >= (((32i32) as u32))) as i32)) != 0) {
            if ((F::btl_char_action__BtlAct_TestSkillHi(c, p_chr_1, (l_bit_3).wrapping_sub(32i32))) != 0) {
                break 'f 1i32;
            }
        } else {
            if ((F::btl_char_action__BtlAct_TestSkillLo(c, p_chr_1, l_bit_3)) != 0) {
                break 'f 1i32;
            }
        }
        break 'f 0i32;
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_TestAttackSkill", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_PrepareAttack(c: &mut Ctx, mut p_chr_1: u32, mut p_id_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_PrepareAttack", &PROBE_N, &[(p_chr_1) as u32, (p_id_2) as u32], false); }
    let mut l_rec_3: u32 = 0;
    let mut l_i_4: i32 = 0;
    let mut l_v_5: F32 = F32(0);
    let r_ = 'f: {
    if ((F::BtlAct_IsAttackId(c, p_id_2)) != 0) {
        l_rec_3 = F::btl_char_action__BtlAct_GetAttackRec(c, (p_id_2).wrapping_sub(112i32));
        if ((((((((c.m.r8((l_rec_3).wrapping_add(4)) as i8)) as i32)) >= (2i32)) as i32)) != 0) {
            { let v_ = ((c.m.r16((l_rec_3).wrapping_add(6))) as i16); let a_ = ((p_chr_1).wrapping_add(3356)).wrapping_add(4); c.m.w16(a_, (v_) as u16); };
            { let v_ = (((((c.m.r16((l_rec_3).wrapping_add(6))) as i32)).wrapping_add(2i32)) as i16); let a_ = ((p_chr_1).wrapping_add(3356)).wrapping_add(6); c.m.w16(a_, (v_) as u16); };
        } else {
            { let v_ = ((c.m.r16((l_rec_3).wrapping_add(6))) as i16); let a_ = ((p_chr_1).wrapping_add(3356)).wrapping_add(4); c.m.w16(a_, (v_) as u16); };
            { let v_ = (((1i32).wrapping_neg()) as i16); let a_ = ((p_chr_1).wrapping_add(3356)).wrapping_add(6); c.m.w16(a_, (v_) as u16); };
        }
        { let v_ = (({ let a0_ = (((c.m.r8((l_rec_3).wrapping_add(4)) as i8)) as i32); F::BtlUtil_Max(c, a0_, 1i32) }) as i8); let a_ = ((p_chr_1).wrapping_add(3356)).wrapping_add(8); c.m.w8(a_, (v_) as u8); };
        { let v_ = ((c.m.r16((l_rec_3).wrapping_add(20))) as i32); let a_ = (p_chr_1).wrapping_add(3356); c.m.w32(a_, (v_) as u32); };
        { let v_ = rt::fmul(rt::fmul(rt::i2f(((c.m.r8(l_rec_3)) as i32)), F32(0x41200000)), F32(0x3c17b425)); let a_ = ((p_chr_1).wrapping_add(3356)).wrapping_add(28); c.m.w32(a_, (v_).0); };
        { let v_ = c.m.r8((l_rec_3).wrapping_add(1)); let a_ = ((p_chr_1).wrapping_add(3356)).wrapping_add(9); c.m.w8(a_, v_); };
        if ((((((((c.m.r8((l_rec_3).wrapping_add(12)) as i8)) as i32)) > (0i32)) as i32)) != 0) {
            l_v_5 = rt::fmul(rt::i2f((((c.m.r8((l_rec_3).wrapping_add(12)) as i8)) as i32)), F32(0x3dcccccc));
        } else {
            l_v_5 = F32(0x3f800000);
        }
        { let v_ = l_v_5; let a_ = ((p_chr_1).wrapping_add(3356)).wrapping_add(16); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((l_rec_3).wrapping_add(8))); let a_ = ((p_chr_1).wrapping_add(3356)).wrapping_add(20); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = ((p_chr_1).wrapping_add(3356)).wrapping_add(24); c.m.w32(a_, (v_).0); };
        { let v_ = 0i32; let a_ = ((p_chr_1).wrapping_add(3356)).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
        if ((((((((c.m.r8((l_rec_3).wrapping_add(18)) as i8)) as i32)) < (0i32)) as i32)) != 0) {
            { let v_ = (c.m.r8((l_rec_3).wrapping_add(17)) as i8); let a_ = ((p_chr_1).wrapping_add(3356)).wrapping_add(10); c.m.w8(a_, (v_) as u8); };
        } else {
            if ((((((c.m.r16((l_rec_3).wrapping_add(20))) as i32)) & (32i32))) != 0) {
                if (((rt::flt(F32(c.m.r32((p_chr_1).wrapping_add(1192))), F32(c.m.r32(((p_chr_1).wrapping_add(3356)).wrapping_add(24)))) as i32)) != 0) {
                    { let v_ = (c.m.r8((l_rec_3).wrapping_add(17)) as i8); let a_ = ((p_chr_1).wrapping_add(3356)).wrapping_add(10); c.m.w8(a_, (v_) as u8); };
                } else {
                    { let v_ = (c.m.r8((l_rec_3).wrapping_add(18)) as i8); let a_ = ((p_chr_1).wrapping_add(3356)).wrapping_add(10); c.m.w8(a_, (v_) as u8); };
                }
            } else {
                { let v_ = (((if ((F::BtlChar_FrameMod(c, 2i32)) != 0) { (((c.m.r8((l_rec_3).wrapping_add(17)) as i8)) as i32) } else { (((c.m.r8((l_rec_3).wrapping_add(18)) as i8)) as i32) })) as i8); let a_ = ((p_chr_1).wrapping_add(3356)).wrapping_add(10); c.m.w8(a_, (v_) as u8); };
            }
        }
        if ((((((F::BtlChar_TestFlag(c, p_chr_1, ((((6i32) as i64 as u64)) as u32))) != 0) && ((((!((F::BtlChar_TestMemberUnk70(c, p_chr_1)) != 0)) as i32)) != 0)) as i32)) != 0) {
            {
                l_i_4 = 0i32;
                'l1: loop {
                    if !(((((l_i_4) < (2i32)) as i32)) != 0) { break 'l1; }
                    'c2: {
                        if (({ let a1_ = (((c.m.r8(((l_rec_3).wrapping_add(15)).wrapping_add(((l_i_4) as u32))) as i8)) as i32); F::BtlAct_TestAttackSkill(c, p_chr_1, a1_) }) != 0) {
                            { let v_ = (c.m.r8(((l_rec_3).wrapping_add(15)).wrapping_add(((l_i_4) as u32))) as i8); let a_ = (((p_chr_1).wrapping_add(3356)).wrapping_add(11)).wrapping_add(((l_i_4) as u32)); c.m.w8(a_, (v_) as u8); };
                        } else {
                            { let v_ = (c.m.r8(((l_rec_3).wrapping_add(13)).wrapping_add(((l_i_4) as u32))) as i8); let a_ = (((p_chr_1).wrapping_add(3356)).wrapping_add(11)).wrapping_add(((l_i_4) as u32)); c.m.w8(a_, (v_) as u8); };
                        }
                    }
                    { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
                }
            }
            { let v_ = p_id_2; let a_ = (p_chr_1).wrapping_add(3472); c.m.w32(a_, (v_) as u32); };
        } else {
            {
                l_i_4 = 0i32;
                'l3: loop {
                    if !(((((l_i_4) < (2i32)) as i32)) != 0) { break 'l3; }
                    'c4: {
                        { let v_ = (c.m.r8(((l_rec_3).wrapping_add(13)).wrapping_add(((l_i_4) as u32))) as i8); let a_ = (((p_chr_1).wrapping_add(3356)).wrapping_add(11)).wrapping_add(((l_i_4) as u32)); c.m.w8(a_, (v_) as u8); };
                    }
                    { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
                }
            }
            { let v_ = p_id_2; let a_ = (p_chr_1).wrapping_add(3472); c.m.w32(a_, (v_) as u32); };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_PrepareAttack", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_ResetAttack(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_ResetAttack", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
    { let v_ = (((1i32).wrapping_neg()) as i16); let a_ = ((p_chr_1).wrapping_add(3356)).wrapping_add(4); c.m.w16(a_, (v_) as u16); };
    { let v_ = (((1i32).wrapping_neg()) as i16); let a_ = ((p_chr_1).wrapping_add(3356)).wrapping_add(6); c.m.w16(a_, (v_) as u16); };
    { let v_ = ((0i32) as i8); let a_ = ((p_chr_1).wrapping_add(3356)).wrapping_add(8); c.m.w8(a_, (v_) as u8); };
    { let v_ = 0i32; let a_ = (p_chr_1).wrapping_add(3356); c.m.w32(a_, (v_) as u32); };
    { let v_ = F32(0x00000000); let a_ = ((p_chr_1).wrapping_add(3356)).wrapping_add(28); c.m.w32(a_, (v_).0); };
    { let v_ = ((0i32) as u8); let a_ = ((p_chr_1).wrapping_add(3356)).wrapping_add(9); c.m.w8(a_, v_); };
    { let v_ = F32(0x3f800000); let a_ = ((p_chr_1).wrapping_add(3356)).wrapping_add(16); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x3f800000); let a_ = ((p_chr_1).wrapping_add(3356)).wrapping_add(20); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x41700000); let a_ = ((p_chr_1).wrapping_add(3356)).wrapping_add(24); c.m.w32(a_, (v_).0); };
    { let v_ = 0i32; let a_ = ((p_chr_1).wrapping_add(3356)).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
    { let v_ = (((1i32).wrapping_neg()) as i8); let a_ = ((p_chr_1).wrapping_add(3356)).wrapping_add(10); c.m.w8(a_, (v_) as u8); };
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < (2i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let v_ = (((1i32).wrapping_neg()) as i8); let a_ = (((p_chr_1).wrapping_add(3356)).wrapping_add(11)).wrapping_add(((l_i_2) as u32)); c.m.w8(a_, (v_) as u8); };
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_ResetAttack", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_LatchAttack(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_LatchAttack", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    { let s_ = (p_chr_1).wrapping_add(3356); let d_ = (p_chr_1).wrapping_add(3320); rt::memcpy(c, d_, s_, 36); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_LatchAttack", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_TestPoweredSkill(c: &mut Ctx, mut p_chr_1: u32, mut p_mask_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_TestPoweredSkill", &PROBE_N, &[(p_chr_1) as u32, (p_mask_2) as u32], false); }
    let r_ = 'f: {
    if ((F::BtlChar_TestFlag(c, p_chr_1, ((((6i32) as i64 as u64)) as u32))) != 0) {
        break 'f (((((F::BtlParam_GetFlags3(c, p_chr_1)) & (p_mask_2))) != (((0i32) as u32))) as i32);
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_TestPoweredSkill", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_SetFormCurrent(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_SetFormCurrent", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = (1i32).wrapping_neg(); let a_ = (p_chr_1).wrapping_add(4812); c.m.w32(a_, (v_) as u32); };
    { let v_ = { let a_ = F::BtlMember_GetActive(c, p_chr_1); (c.m.r32(a_) as i32) }; let a_ = ((p_chr_1).wrapping_add(4812)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
    { let v_ = { let a_ = (F::BtlMember_GetActive(c, p_chr_1)).wrapping_add(4); (c.m.r32(a_) as i32) }; let a_ = ((p_chr_1).wrapping_add(4812)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
    { let v_ = 1i32; let a_ = ((p_chr_1).wrapping_add(4812)).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
    { let v_ = 0i32; let a_ = ((p_chr_1).wrapping_add(4812)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
    { let v_ = 0i32; let a_ = ((p_chr_1).wrapping_add(4812)).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
    { let v_ = (1i32).wrapping_neg(); let a_ = ((p_chr_1).wrapping_add(4812)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
    { let v_ = (1i32).wrapping_neg(); let a_ = ((p_chr_1).wrapping_add(4812)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
    { let v_ = 0i32; let a_ = ((p_chr_1).wrapping_add(4812)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
    { let v_ = (1i32).wrapping_neg(); let a_ = ((p_chr_1).wrapping_add(4812)).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
    { let v_ = (1i32).wrapping_neg(); let a_ = ((p_chr_1).wrapping_add(4812)).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
    { let v_ = (1i32).wrapping_neg(); let a_ = ((p_chr_1).wrapping_add(4812)).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_SetFormCurrent", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_SetFormRandom(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_SetFormRandom", &PROBE_N, &[(p_chr_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
        rt::memset(c, fp, 0, 48);
        rt::memset(c, fp.wrapping_add(0), 0, 8);
        { let v_ = 1i32; c.m.w32(fp.wrapping_add(0).wrapping_add(0), (v_) as u32); }
        { let v_ = 0i32; c.m.w32(fp.wrapping_add(0).wrapping_add(4), (v_) as u32); }
        rt::memset(c, fp.wrapping_add(8), 0, 8);
        { let v_ = 13i32; c.m.w32(fp.wrapping_add(8).wrapping_add(0), (v_) as u32); }
        { let v_ = 2i32; c.m.w32(fp.wrapping_add(8).wrapping_add(4), (v_) as u32); }
        rt::memset(c, fp.wrapping_add(16), 0, 8);
        { let v_ = 25i32; c.m.w32(fp.wrapping_add(16).wrapping_add(0), (v_) as u32); }
        { let v_ = 1i32; c.m.w32(fp.wrapping_add(16).wrapping_add(4), (v_) as u32); }
        rt::memset(c, fp.wrapping_add(24), 0, 8);
        { let v_ = 29i32; c.m.w32(fp.wrapping_add(24).wrapping_add(0), (v_) as u32); }
        { let v_ = 1i32; c.m.w32(fp.wrapping_add(24).wrapping_add(4), (v_) as u32); }
        rt::memset(c, fp.wrapping_add(32), 0, 8);
        { let v_ = 24i32; c.m.w32(fp.wrapping_add(32).wrapping_add(0), (v_) as u32); }
        { let v_ = 0i32; c.m.w32(fp.wrapping_add(32).wrapping_add(4), (v_) as u32); }
        rt::memset(c, fp.wrapping_add(40), 0, 8);
        { let v_ = 89i32; c.m.w32(fp.wrapping_add(40).wrapping_add(0), (v_) as u32); }
        { let v_ = 0i32; c.m.w32(fp.wrapping_add(40).wrapping_add(4), (v_) as u32); }
        l_i_2 = F::BtlChar_FrameMod(c, 6i32);
        { let v_ = (c.m.r32(((fp).wrapping_add((((l_i_2) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32); let a_ = ((p_chr_1).wrapping_add(4812)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = (c.m.r32(((fp).wrapping_add((((l_i_2) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32); let a_ = ((p_chr_1).wrapping_add(4812)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = (1i32).wrapping_neg(); let a_ = (p_chr_1).wrapping_add(4812); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = ((p_chr_1).wrapping_add(4812)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = ((p_chr_1).wrapping_add(4812)).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
        { let v_ = (1i32).wrapping_neg(); let a_ = ((p_chr_1).wrapping_add(4812)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
        { let v_ = (1i32).wrapping_neg(); let a_ = ((p_chr_1).wrapping_add(4812)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = ((p_chr_1).wrapping_add(4812)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = ((p_chr_1).wrapping_add(4812)).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
        { let v_ = (1i32).wrapping_neg(); let a_ = ((p_chr_1).wrapping_add(4812)).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
        { let v_ = (1i32).wrapping_neg(); let a_ = ((p_chr_1).wrapping_add(4812)).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
        { let v_ = (1i32).wrapping_neg(); let a_ = ((p_chr_1).wrapping_add(4812)).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_SetFormRandom", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_CountAndMarkOpponent(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_CountAndMarkOpponent", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    { let a_ = (p_chr_1).wrapping_add(3664); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
    { let v_ = 1i32; let a_ = ({ let a0_ = F::BtlOpp_GetPlayer(c, p_chr_1); F::BtlChar_Get(c, a0_) }).wrapping_add(3404); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_CountAndMarkOpponent", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_EndFlag98(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_EndFlag98", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_obj_2: u32 = 0;
    let r_ = 'f: {
    if ((F::BtlChar_TestFlag(c, p_chr_1, ((((152i32) as i64 as u64)) as u32))) != 0) {
        l_obj_2 = F::BtlChar_GetObj(c, p_chr_1);
        F::BtlChar_ClearFlag(c, p_chr_1, ((((152i32) as i64 as u64)) as u32));
        F::BtlObj_SetColorMode(c, l_obj_2, 2i32, 0i32);
        F::BtlStat_EndKind4(c, p_chr_1);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_EndFlag98", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_ApplyFlag94(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_ApplyFlag94", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    if ((F::BtlChar_TestFlag(c, p_chr_1, ((((148i32) as i64 as u64)) as u32))) != 0) {
        { let a1_ = F32(c.m.r32((p_chr_1).wrapping_add(4048))); let a2_ = F32(c.m.r32((p_chr_1).wrapping_add(4052))); F::BtlMove_SetHeading(c, p_chr_1, a1_, a2_) };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_ApplyFlag94", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_CheckForced(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_CheckForced", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_act_2: u32 = 0;
    let r_ = 'f: {
    l_act_2 = (p_chr_1).wrapping_add(2376);
    if ((((((c.m.r32((p_chr_1).wrapping_add(2416)) as i32)) != (0i32)) as i32)) != 0) {
        break 'f;
    }
    if ((((((c.m.r32(l_act_2) as i32)) == (0i32)) as i32)) != 0) {
        F::BtlAct_Request(c, p_chr_1, 11i32);
    }
    if ((F::BtlChar_IsFree(c, p_chr_1)) != 0) {
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((116i32) as i64 as u64)) as u32))) != 0) {
            F::BtlAct_Request(c, p_chr_1, 33i32);
        }
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((117i32) as i64 as u64)) as u32))) != 0) {
            F::BtlAct_Request(c, p_chr_1, 34i32);
        }
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((118i32) as i64 as u64)) as u32))) != 0) {
            F::BtlAct_Request(c, p_chr_1, 32i32);
        }
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((119i32) as i64 as u64)) as u32))) != 0) {
            { let a1_ = (if ((F::BtlChar_IsStage4Or27(c)) != 0) { 122i32 } else { 121i32 }); { let _ = F::BtlDecide_QueueAttack(c, p_chr_1, a1_); } };
            { let a1_ = F::BtlAct_GetQueued(c, p_chr_1); F::BtlAct_Request(c, p_chr_1, a1_) };
        }
    }
    if ((F::BtlChar_TestFlag(c, p_chr_1, ((((124i32) as i64 as u64)) as u32))) != 0) {
        { let a1_ = F::BtlAct_GetEvasionAttack(c, p_chr_1); { let _ = F::BtlDecide_QueueAttack(c, p_chr_1, a1_); } };
        { let a1_ = F::BtlAct_GetQueued(c, p_chr_1); F::BtlAct_Request(c, p_chr_1, a1_) };
    }
    if ((F::BtlChar_TestFlag(c, p_chr_1, ((((109i32) as i64 as u64)) as u32))) != 0) {
        F::BtlAct_Request(c, p_chr_1, 201i32);
    }
    if ((F::BtlChar_TestFlag(c, p_chr_1, ((((111i32) as i64 as u64)) as u32))) != 0) {
        F::BtlAct_Request(c, p_chr_1, 202i32);
    }
    if ((((((((((F::BtlChar_TestFlag(c, p_chr_1, ((((104i32) as i64 as u64)) as u32))) != 0) || ((F::BtlChar_TestFlag(c, p_chr_1, ((((105i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) || ((F::BtlChar_TestFlag(c, p_chr_1, ((((106i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) {
        F::BtlAct_Request(c, p_chr_1, 56i32);
    }
    if ((F::BtlChar_TestFlag(c, p_chr_1, ((((123i32) as i64 as u64)) as u32))) != 0) {
        F::BtlAct_Request(c, p_chr_1, 57i32);
    }
    if ((F::BtlChar_TestFlag(c, p_chr_1, ((((107i32) as i64 as u64)) as u32))) != 0) {
        if ((((F::BtlParam_GetFlags(c, p_chr_1)) & (((2i32) as u32)))) != 0) {
            F::BtlAct_Request(c, p_chr_1, 61i32);
        } else {
            F::BtlAct_Request(c, p_chr_1, 59i32);
        }
    }
    if ((F::BtlChar_TestFlag(c, p_chr_1, ((((121i32) as i64 as u64)) as u32))) != 0) {
        F::BtlAct_Request(c, p_chr_1, 189i32);
    }
    if (({ let a1_ = (c.m.r32((p_chr_1).wrapping_add(4016)) as i32); F::BtlAct_QueueReaction(c, p_chr_1, ((((a1_) as i64 as u64)) as u32)) }) != 0) {
        { let a1_ = F::BtlAct_GetQueued(c, p_chr_1); F::BtlAct_Request(c, p_chr_1, a1_) };
    }
    if ((F::BtlChar_TestFlag(c, p_chr_1, ((((127i32) as i64 as u64)) as u32))) != 0) {
        F::BtlAct_Request(c, p_chr_1, 190i32);
    }
    if ((F::BtlChar_TestFlag(c, p_chr_1, ((((97i32) as i64 as u64)) as u32))) != 0) {
        F::BtlAct_Request(c, p_chr_1, 250i32);
    }
    if ((F::BtlChar_TestFlag(c, p_chr_1, ((((98i32) as i64 as u64)) as u32))) != 0) {
        F::BtlAct_Request(c, p_chr_1, 251i32);
    }
    if ((F::BtlChar_TestFlag(c, p_chr_1, ((((170i32) as i64 as u64)) as u32))) != 0) {
        { let a1_ = (F::BtlAct_GetCurrentClass(c, p_chr_1)).wrapping_add(302i32); F::BtlAct_Request(c, p_chr_1, a1_) };
    }
    if ((F::BtlChar_TestFlag(c, p_chr_1, ((((171i32) as i64 as u64)) as u32))) != 0) {
        F::BtlChar_ClearFlag(c, p_chr_1, ((((171i32) as i64 as u64)) as u32));
        F::BtlAct_Request(c, p_chr_1, 260i32);
    }
    if ((F::BtlChar_TestFlag(c, p_chr_1, ((((125i32) as i64 as u64)) as u32))) != 0) {
        F::BtlAct_Request(c, p_chr_1, 65i32);
    }
    if ((F::BtlChar_TestFlag(c, p_chr_1, ((((126i32) as i64 as u64)) as u32))) != 0) {
        F::BtlAct_Request(c, p_chr_1, 69i32);
    }
    if ((F::BtlChar_TestFlag(c, p_chr_1, ((((313i32) as i64 as u64)) as u32))) != 0) {
        F::BtlChar_ClearFlag(c, p_chr_1, ((((313i32) as i64 as u64)) as u32));
        F::BtlAct_Request(c, p_chr_1, 11i32);
    }
    if ((F::BtlChar_TestFlag(c, p_chr_1, ((((314i32) as i64 as u64)) as u32))) != 0) {
        F::BtlChar_ClearFlag(c, p_chr_1, ((((314i32) as i64 as u64)) as u32));
        F::BtlAct_Request(c, p_chr_1, 216i32);
    }
    if ((F::BtlChar_TestFlag(c, p_chr_1, ((((239i32) as i64 as u64)) as u32))) != 0) {
        F::BtlAct_Request(c, p_chr_1, 1i32);
    }
    if ((F::BtlChar_TestFlag(c, p_chr_1, ((((241i32) as i64 as u64)) as u32))) != 0) {
        F::BtlAct_Request(c, p_chr_1, 2i32);
    }
    if ((F::BtlChar_TestFlag(c, p_chr_1, ((((242i32) as i64 as u64)) as u32))) != 0) {
        F::BtlAct_Request(c, p_chr_1, 3i32);
    }
    if ((F::BtlAct_CheckStoryForced(c, p_chr_1)) != 0) {
        { let a1_ = F::BtlAct_GetQueued(c, p_chr_1); F::BtlAct_Request(c, p_chr_1, a1_) };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_CheckForced", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_UpdateGauges(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_UpdateGauges", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_tick_2: i32 = 0;
    let mut l_mflags_3: u32 = 0;
    let mut l_amount_4: i32 = 0;
    let mut l_floor_5: i32 = 0;
    let mut l_i_6: i32 = 0;
    let mut l_member_7: u32 = 0;
    let mut l_v_8: u32 = 0;
    let mut l_cap_9: i32 = 0;
    let r_ = 'f: {
    l_tick_2 = 0i32;
    l_mflags_3 = { let a0_ = F::BtlAnim_GetId(c, p_chr_1); F::BtlAnim_GetFlags(c, ((((a0_) as i64 as u64)) as u32)) };
    if ((F::BtlChars_IsTimeStopped(c)) != 0) {
        break 'f;
    }
    if ((((!((F::BtlChar_TestFlag(c, p_chr_1, ((((3i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) {
        break 'f;
    }
    if ((F::BtlChar_TestFlag(c, p_chr_1, ((((293i32) as i64 as u64)) as u32))) != 0) {
        break 'f;
    }
    if ((F::BtlChar_TestFlag(c, p_chr_1, ((((294i32) as i64 as u64)) as u32))) != 0) {
        break 'f;
    }
    if ((F::BtlChar_TestFlag(c, p_chr_1, ((((296i32) as i64 as u64)) as u32))) != 0) {
        break 'f;
    }
    if ((((!((F::BtlChar_TestFlag(c, p_chr_1, ((((187i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) {
        l_amount_4 = F::BtlParam_GetMaxPowerDrain(c, p_chr_1);
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((188i32) as i64 as u64)) as u32))) != 0) {
            { let r_ = 3i32; let n_ = (l_amount_4).wrapping_mul(r_); l_amount_4 = n_; };
        } else {
            if ((F::BtlChar_TestFlag(c, p_chr_1, ((((189i32) as i64 as u64)) as u32))) != 0) {
                { let r_ = 2i32; let n_ = (l_amount_4).wrapping_mul(r_); l_amount_4 = n_; };
            } else {
                if ((F::BtlMember_HasAbility(c, p_chr_1, 66i32)) != 0) {
                    { let r_ = 2i32; let n_ = (l_amount_4).wrapping_mul(r_); l_amount_4 = n_; };
                } else {
                    if ((F::BtlMember_HasAbility(c, p_chr_1, 65i32)) != 0) {
                        l_amount_4 = rt::div_i32((l_amount_4).wrapping_mul(2i32), 3i32);
                    }
                }
            }
        }
        F::BtlMember_SubMaxPower(c, p_chr_1, l_amount_4);
        if ((F::BtlMember_IsMaxPowerEmpty(c, p_chr_1)) != 0) {
            F::BtlChar_ClearFlag(c, p_chr_1, ((((6i32) as i64 as u64)) as u32));
            F::BtlStat_EndKind3(c, p_chr_1);
        }
    }
    if ((((((((((F::BtlChar_TestFlag(c, p_chr_1, ((((152i32) as i64 as u64)) as u32))) != 0) && ((((!((F::BtlChar_TestFlag(c, p_chr_1, ((((153i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0)) as i32)) != 0) && ((((!(({ let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); F::BtlCharApi_IsInTechnique(c, a0_) }) != 0)) as i32)) != 0)) as i32)) != 0) {
        { let a1_ = (c.m.r32((p_chr_1).wrapping_add(3600)) as i32); { let _ = F::BtlMember_SpendKi(c, p_chr_1, a1_, 0i32); } };
        if ((F::BtlMember_IsKiEmpty(c, p_chr_1)) != 0) {
            F::BtlAct_EndFlag98(c, p_chr_1);
        }
    }
    if ((F::BtlChar_IsDead(c, p_chr_1)) != 0) {
        break 'f;
    }
    if ((((({ let a_ = (p_chr_1).wrapping_add(4908); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); n_ }) >= (30i32)) as i32)) != 0) {
        { let v_ = 0i32; let a_ = (p_chr_1).wrapping_add(4908); c.m.w32(a_, (v_) as u32); };
        l_tick_2 = 1i32;
    }
    if ((((l_mflags_3) & (((65536i32) as u32)))) != 0) {
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((190i32) as i64 as u64)) as u32))) != 0) {
            l_amount_4 = F::BtlParam_GetKiRecoverRate(c, p_chr_1);
            F::BtlChar_SetFlag(c, p_chr_1, ((((231i32) as i64 as u64)) as u32));
            if ((F::BtlInput_IsPressed(c, p_chr_1, ((1048576i32) as u32))) != 0) {
                { let r_ = 4i32; let n_ = (l_amount_4).wrapping_mul(r_); l_amount_4 = n_; };
            }
            { let _ = F::BtlMember_AddKi(c, p_chr_1, l_amount_4); };
            if (({ let a1_ = F::BtlParam_GetGaugeB(c, p_chr_1); F::BtlMember_HasKi(c, p_chr_1, a1_) }) != 0) {
                F::BtlChar_ClearFlag(c, p_chr_1, ((((190i32) as i64 as u64)) as u32));
            }
        } else {
            if ((((!((F::BtlChar_TestFlag(c, p_chr_1, ((((152i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) {
                if ((((!(({ let a1_ = F::BtlParam_GetKiRegenLimit(c, p_chr_1); F::BtlMember_HasKi(c, p_chr_1, a1_) }) != 0)) as i32)) != 0) {
                    { let a1_ = F::BtlParam_GetKiRegenRate(c, p_chr_1); { let _ = F::BtlMember_AddKi(c, p_chr_1, a1_); } };
                }
                if (((((((((F::Battle_GetMode(c)) == (5i32)) as i32)) != 0) || (((((F::Battle_GetMode(c)) == (6i32)) as i32)) != 0)) as i32)) != 0) {
                    { let _ = F::BtlMember_AddKi(c, p_chr_1, 1000i32); };
                }
                if ((l_tick_2) != 0) {
                    if ((F::BtlMember_HasAbility(c, p_chr_1, 120i32)) != 0) {
                        { let _ = F::BtlMember_AddKi(c, p_chr_1, 20000i32); };
                    } else {
                        if ((F::BtlMember_HasAbility(c, p_chr_1, 31i32)) != 0) {
                            { let _ = F::BtlMember_AddKi(c, p_chr_1, 900i32); };
                        } else {
                            if ((F::BtlMember_HasAbility(c, p_chr_1, 30i32)) != 0) {
                                { let _ = F::BtlMember_AddKi(c, p_chr_1, 600i32); };
                            }
                        }
                    }
                    if ((F::BtlMember_HasAbility(c, p_chr_1, 87i32)) != 0) {
                        { let _ = F::BtlMember_AddKi(c, p_chr_1, 300i32); };
                    }
                    if ((((((F::BtlChar_TestFlag(c, p_chr_1, ((((17i32) as i64 as u64)) as u32))) != 0) && ((F::BtlMember_HasAbility(c, p_chr_1, 92i32)) != 0)) as i32)) != 0) {
                        { let _ = F::BtlMember_AddKi(c, p_chr_1, 900i32); };
                    }
                }
            }
        }
    }
    if ((((l_mflags_3) & (((131072i32) as u32)))) != 0) {
        { let a1_ = F::BtlParam_GetBlastGainRate(c, p_chr_1); F::BtlMember_AddBlast(c, p_chr_1, a1_) };
        if (((((((((F::Battle_GetMode(c)) == (5i32)) as i32)) != 0) || (((((F::Battle_GetMode(c)) == (6i32)) as i32)) != 0)) as i32)) != 0) {
            F::BtlMember_AddBlast(c, p_chr_1, 3333i32);
        }
    }
    F::BtlStat_Update(c, p_chr_1);
    F::BtlSkill_UpdateTimers(c, p_chr_1);
    { let r_ = 400i32; let a_ = (p_chr_1).wrapping_add(3456); let n_ = ((c.m.r32(a_) as i32)).wrapping_sub(r_); c.m.w32(a_, (n_) as u32); };
    l_floor_5 = (if ((F::BtlAct_TestPoweredSkill(c, p_chr_1, ((1i32) as u32))) != 0) { 50000i32 } else { 0i32 });
    if ((((((c.m.r32((p_chr_1).wrapping_add(3616)) as i32)) > (0i32)) as i32)) != 0) {
        if (((((l_floor_5) < ((c.m.r32((p_chr_1).wrapping_add(3616)) as i32))) as i32)) != 0) {
            l_floor_5 = (c.m.r32((p_chr_1).wrapping_add(3616)) as i32);
        }
    }
    if ((((((c.m.r32((p_chr_1).wrapping_add(3456)) as i32)) < (l_floor_5)) as i32)) != 0) {
        { let v_ = l_floor_5; let a_ = (p_chr_1).wrapping_add(3456); c.m.w32(a_, (v_) as u32); };
    }
    if (({ let a0_ = (c.m.r32(p_chr_1) as i32); F::BattleSide_GetUnk200(c, a0_) }) != 0) {
        { let r_ = rt::f2i(rt::fdiv(F32(0x47c35000), rt::fmul(F::BtlParam_GetGauge99CTime(c, p_chr_1), F32(0x41f00000)))); let a_ = (p_chr_1).wrapping_add(2460); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
        if (((((((((F::Battle_GetMode(c)) == (5i32)) as i32)) != 0) || (((((F::Battle_GetMode(c)) == (6i32)) as i32)) != 0)) as i32)) != 0) {
            { let r_ = 1111i32; let a_ = (p_chr_1).wrapping_add(2460); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
        }
        if ((((((c.m.r32((p_chr_1).wrapping_add(2460)) as i32)) > (100000i32)) as i32)) != 0) {
            { let v_ = 100000i32; let a_ = (p_chr_1).wrapping_add(2460); c.m.w32(a_, (v_) as u32); };
        }
    }
    if (((((((((F::Battle_GetMode(c)) == (5i32)) as i32)) != 0) || (((((F::Battle_GetMode(c)) == (6i32)) as i32)) != 0)) as i32)) != 0) {
        if ((((!((F::BtlChar_TestFlag(c, p_chr_1, ((((149i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) {
            F::BtlMember_AddHealth(c, p_chr_1, 200i32);
        }
    }
    if ((l_tick_2) != 0) {
        if ((F::BtlMember_HasAbility(c, p_chr_1, 29i32)) != 0) {
            F::BtlMember_AddHealth(c, p_chr_1, 100i32);
        } else {
            if ((F::BtlMember_HasAbility(c, p_chr_1, 28i32)) != 0) {
                F::BtlMember_AddHealth(c, p_chr_1, 50i32);
            }
        }
        if ((((((F::BtlChar_TestFlag(c, p_chr_1, ((((17i32) as i64 as u64)) as u32))) != 0) && ((F::BtlMember_HasAbility(c, p_chr_1, 92i32)) != 0)) as i32)) != 0) {
            F::BtlMember_AddHealth(c, p_chr_1, 100i32);
        }
        {
            l_i_6 = 0i32;
            'l1: loop {
                if !(((((l_i_6) < ((c.m.r32((p_chr_1).wrapping_add(2456)) as i32))) as i32)) != 0) { break 'l1; }
                'c2: {
                    if (((((l_i_6) != ((c.m.r32((p_chr_1).wrapping_add(2452)) as i32))) as i32)) != 0) {
                        l_member_7 = F::BtlMember_Get(c, p_chr_1, l_i_6);
                        l_v_8 = (l_member_7).wrapping_add(64);
                        if (((c.m.r32((l_member_7).wrapping_add(8)) as i32)) != 0) {
                            if ((((((c.m.r32(l_v_8) as i32)) > (0i32)) as i32)) != 0) {
                                l_cap_9 = (rt::div_i32(((c.m.r32(l_v_8) as i32)).wrapping_add(9999i32), 10000i32)).wrapping_mul(10000i32);
                                { let v_ = { let a0_ = ((c.m.r32(l_v_8) as i32)).wrapping_add(100i32); F::BtlUtil_Min(c, a0_, l_cap_9) }; let a_ = l_v_8; c.m.w32(a_, (v_) as u32); };
                                if ((F::BtlMember_HasAbilityOf(c, p_chr_1, l_i_6, 95i32)) != 0) {
                                    { let v_ = { let a0_ = ((c.m.r32(l_v_8) as i32)).wrapping_add(200i32); F::BtlUtil_Min(c, a0_, l_cap_9) }; let a_ = l_v_8; c.m.w32(a_, (v_) as u32); };
                                }
                                { let v_ = { let a0_ = ((c.m.r32((l_v_8).wrapping_add(12)) as i32)).wrapping_add(1000i32); let a1_ = (c.m.r32((l_v_8).wrapping_add(16)) as i32); F::BtlUtil_Min(c, a0_, a1_) }; let a_ = (l_v_8).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                                if ((F::BtlMember_HasAbilityOf(c, p_chr_1, l_i_6, 96i32)) != 0) {
                                    { let v_ = { let a0_ = ((c.m.r32((l_v_8).wrapping_add(12)) as i32)).wrapping_add(2000i32); let a1_ = (c.m.r32((l_v_8).wrapping_add(16)) as i32); F::BtlUtil_Min(c, a0_, a1_) }; let a_ = (l_v_8).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                                }
                            }
                        }
                    }
                }
                { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; };
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_UpdateGauges", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_UpdateTimers(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_UpdateTimers", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_obj_2: u32 = 0;
    let mut l_mflags_3: u32 = 0;
    let mut l_pose_4: u32 = 0;
    let mut l_i_5: i32 = 0;
    let mut l_dir_6: i32 = 0;
    let mut l_kind_7: i32 = 0;
    let mut l_j_8: i32 = 0;
    let r_ = 'f: {
    l_obj_2 = F::BtlChar_GetObj(c, p_chr_1);
    l_mflags_3 = { let a0_ = F::BtlAnim_GetId(c, p_chr_1); F::BtlAnim_GetFlags(c, ((((a0_) as i64 as u64)) as u32)) };
    { let _ = F::BtlMove_ApplyImpulse(c, p_chr_1); };
    F::BtlMove_UpdateHoverOffset(c, p_chr_1);
    F::BtlMove_UpdateDefenseTimers(c, p_chr_1);
    if ((F::BtlChar_TestFlag(c, p_chr_1, ((((139i32) as i64 as u64)) as u32))) != 0) {
        { let a0_ = F::BtlChar_GetPos(c, p_chr_1); let a1_ = F::BtlChar_GetPos(c, p_chr_1); F::Vec4_Add(c, a0_, a1_, (p_chr_1).wrapping_add(3888)) };
    }
    if ((F::BtlChars_IsTimeStopped(c)) != 0) {
        break 'f;
    }
    if ((((!((F::BtlChar_TestFlag(c, p_chr_1, ((((130i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) {
        F::BtlChar_ClearFlag(c, p_chr_1, ((((138i32) as i64 as u64)) as u32));
    }
    if ((((!((F::BtlChar_TestFlag(c, p_chr_1, ((((136i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) {
        { let v_ = 0i32; let a_ = (p_chr_1).wrapping_add(3424); c.m.w32(a_, (v_) as u32); };
        if ((((!((((l_mflags_3) & (((2i32) as u32)))) != 0)) as i32)) != 0) {
            F::BtlChar_ClearFlag(c, p_chr_1, ((((134i32) as i64 as u64)) as u32));
        }
    }
    if ((((!((F::BtlChar_TestFlag(c, p_chr_1, ((((137i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) {
        { let v_ = 0i32; let a_ = (p_chr_1).wrapping_add(3432); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (p_chr_1).wrapping_add(3440); c.m.w32(a_, (v_) as u32); };
    }
    if ((((!((F::BtlChar_TestFlag(c, p_chr_1, ((((149i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) {
        { let v_ = 0i32; let a_ = (p_chr_1).wrapping_add(3392); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (p_chr_1).wrapping_add(3396); c.m.w32(a_, (v_) as u32); };
        if ((((((c.m.r32((p_chr_1).wrapping_add(3400)) as i32)) > (0i32)) as i32)) != 0) {
            { let a_ = (p_chr_1).wrapping_add(3400); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
        }
        if ((F::BtlChar_TestPrevFlag(c, p_chr_1, ((((149i32) as i64 as u64)) as u32))) != 0) {
            {
                l_j_8 = 0i32;
                'l1: loop {
                    if !(((((l_j_8) < (50i32)) as i32)) != 0) { break 'l1; }
                    'c2: {
                        { let v_ = ((0i32) as u8); let a_ = ((p_chr_1).wrapping_add(4100)).wrapping_add(((l_j_8) as u32)); c.m.w8(a_, v_); };
                    }
                    { let o_ = l_j_8; let n_ = (o_).wrapping_add(1); l_j_8 = n_; };
                }
            }
        }
    }
    {
        l_i_5 = 0i32;
        'l3: loop {
            if !(((((l_i_5) < (50i32)) as i32)) != 0) { break 'l3; }
            'c4: {
                if ((((((((c.m.r8(((p_chr_1).wrapping_add(4150)).wrapping_add(((l_i_5) as u32))) as i8)) as i32)) < (100i32)) as i32)) != 0) {
                    { let a_ = ((p_chr_1).wrapping_add(4150)).wrapping_add(((l_i_5) as u32)); let o_ = (c.m.r8(a_) as i8); let n_ = (o_).wrapping_add(1); c.m.w8(a_, (n_) as u8); };
                }
            }
            { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
        }
    }
    if ((((((c.m.r32((p_chr_1).wrapping_add(4064)) as i32)) > (0i32)) as i32)) != 0) {
        F::BtlChar_SetFxBit(c, p_chr_1, 26i32);
        if ((((l_mflags_3) & (((65536i32) as u32)))) != 0) {
            { let a_ = (p_chr_1).wrapping_add(4064); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
            F::BtlChar_SetSmallVibration(c, p_chr_1, F32(0x3dcccccc));
            F::BtlChar_SetFlag(c, p_chr_1, ((((231i32) as i64 as u64)) as u32));
            if ((F::BtlInput_IsPressed(c, p_chr_1, ((1048576i32) as u32))) != 0) {
                { let r_ = 3i32; let a_ = (p_chr_1).wrapping_add(4064); let n_ = ((c.m.r32(a_) as i32)).wrapping_sub(r_); c.m.w32(a_, (n_) as u32); };
                if ((((((c.m.r32((p_chr_1).wrapping_add(4064)) as i32)) < (0i32)) as i32)) != 0) {
                    { let v_ = 0i32; let a_ = (p_chr_1).wrapping_add(4064); c.m.w32(a_, (v_) as u32); };
                }
            }
        }
        if ((F::BtlChar_IsDead(c, p_chr_1)) != 0) {
            { let v_ = 0i32; let a_ = (p_chr_1).wrapping_add(4064); c.m.w32(a_, (v_) as u32); };
        }
    }
    if ((((((c.m.r32((p_chr_1).wrapping_add(4088)) as i32)) > (0i32)) as i32)) != 0) {
        { let a_ = (p_chr_1).wrapping_add(4088); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
        F::BtlChar_SetFlag(c, p_chr_1, ((((147i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((311i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((150i32) as i64 as u64)) as u32));
    }
    { let v_ = { let a0_ = (c.m.r32((p_chr_1).wrapping_add(4092)) as i32); let a1_ = ((c.m.r32((p_chr_1).wrapping_add(4088)) as i32)).wrapping_mul(2i32); F::BtlUtil_Approach(c, a0_, a1_, 15i32) }; let a_ = (p_chr_1).wrapping_add(4092); c.m.w32(a_, (v_) as u32); };
    if ((((((c.m.r32((p_chr_1).wrapping_add(4072)) as i32)) > (0i32)) as i32)) != 0) {
        { let a_ = (p_chr_1).wrapping_add(4072); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
        if ((F::BtlChar_IsDead(c, p_chr_1)) != 0) {
            { let v_ = 0i32; let a_ = (p_chr_1).wrapping_add(4072); c.m.w32(a_, (v_) as u32); };
        }
    }
    if ((((((c.m.r32((p_chr_1).wrapping_add(4068)) as i32)) > (0i32)) as i32)) != 0) {
        { let a_ = (p_chr_1).wrapping_add(4068); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
        F::BtlChar_SetFlag(c, p_chr_1, ((((150i32) as i64 as u64)) as u32));
    }
    if ((((((c.m.r32((p_chr_1).wrapping_add(3556)) as i32)) > (0i32)) as i32)) != 0) {
        if ((((!((((l_mflags_3) & (((4i32) as u32)))) != 0)) as i32)) != 0) {
            { let a_ = (p_chr_1).wrapping_add(3556); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
        }
    } else {
        { let v_ = 0i32; let a_ = (p_chr_1).wrapping_add(3552); c.m.w32(a_, (v_) as u32); };
    }
    if ((((!(({ let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); F::BtlCharApi_IsInTechnique(c, a0_) }) != 0)) as i32)) != 0) {
        if ((((((c.m.r32((p_chr_1).wrapping_add(3648)) as i32)) > (0i32)) as i32)) != 0) {
            { let a_ = (p_chr_1).wrapping_add(3648); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
        }
    }
    if ((((((c.m.r32((p_chr_1).wrapping_add(4080)) as i32)) > (0i32)) as i32)) != 0) {
        { let a_ = (p_chr_1).wrapping_add(4080); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
        if ((((!((F::BtlChar_TestFlag(c, p_chr_1, ((((19i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) {
            { let v_ = 0i32; let a_ = (p_chr_1).wrapping_add(4080); c.m.w32(a_, (v_) as u32); };
        }
    }
    l_pose_4 = F::BtlChar_GetPos(c, p_chr_1);
    's5: {
        let sv9: i32 = F::BtlAct_GetCurrent(c, p_chr_1);
        match sv9 {
            15i32 | 62i32 | 88i32 | 176i32 | 177i32 => {
                { let v_ = { let a0_ = F32(c.m.r32((l_pose_4).wrapping_add(172))); F::BtlUtil_MaxF(c, a0_, F32(0x3f800000)) }; let a_ = (l_pose_4).wrapping_add(172); c.m.w32(a_, (v_).0); };
                break 's5;
            }
            _ => {
                { let v_ = F32(0x3f800000); let a_ = (l_pose_4).wrapping_add(172); c.m.w32(a_, (v_).0); };
                break 's5;
            }
        }
    }
    if ((((!((((l_mflags_3) & (((2064i32) as u32)))) != 0)) as i32)) != 0) {
        F::BtlChar_ClearFlag(c, p_chr_1, ((((29i32) as i64 as u64)) as u32));
        F::BtlChar_ClearFlag(c, p_chr_1, ((((30i32) as i64 as u64)) as u32));
    }
    if (((((((((((((({ let a0_ = (c.m.r32(p_chr_1) as i32); F::BattleSide_GetUnk200(c, a0_) }) != 0) && (((((F::Battle_GetMode(c)) != (1i32)) as i32)) != 0)) as i32)) != 0) && ((((!((F::BtlChar_TestFlag(c, p_chr_1, ((((308i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0)) as i32)) != 0) && (((((F::BtlMember_CountAlive(c, p_chr_1)) >= (3i32)) as i32)) != 0)) as i32)) != 0) {
        if ((F::BtlInput_IsPressed(c, p_chr_1, ((33554432i32) as u32))) != 0) {
            F::BtlMember_PrevSwitchTarget(c, p_chr_1);
            F::BtlChar_SetFlag(c, p_chr_1, ((((228i32) as i64 as u64)) as u32));
        }
        if ((F::BtlInput_IsPressed(c, p_chr_1, ((67108864i32) as u32))) != 0) {
            F::BtlMember_NextSwitchTarget(c, p_chr_1);
            F::BtlChar_SetFlag(c, p_chr_1, ((((229i32) as i64 as u64)) as u32));
        }
    }
    's6: {
        let sv10: i32 = F::BtlAct_GetCurrent(c, p_chr_1);
        match sv10 {
            137i32 | 138i32 | 139i32 | 140i32 => {
                if ((F::BtlChar_TestFlag(c, p_chr_1, ((((8i32) as i64 as u64)) as u32))) != 0) {
                    { let a_ = (p_chr_1).wrapping_add(3464); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                }
                break 's6;
            }
            43i32 | 45i32 | 46i32 => {
                break 's6;
            }
            _ => {
                { let v_ = 0i32; let a_ = (p_chr_1).wrapping_add(3464); c.m.w32(a_, (v_) as u32); };
                break 's6;
            }
        }
    }
    l_kind_7 = F::BtlOpp_GetParamByte2(c, p_chr_1);
    if (((((l_kind_7) == (4i32)) as i32)) != 0) {
        if (((((F::BtlParam_GetUnk2(c, p_chr_1)) == (l_kind_7)) as i32)) != 0) {
            { let a1_ = rt::fmul(F::BtlAnim_GetObjRate(c, p_chr_1), F32(0x40000000)); F::BtlAnim_SetObjRate(c, p_chr_1, a1_) };
        } else {
            { let a1_ = rt::fmul(F::BtlAnim_GetObjRate(c, p_chr_1), F32(0x40400000)); F::BtlAnim_SetObjRate(c, p_chr_1, a1_) };
        }
    }
    if ((((!(({ let a0_ = F::BtlAct_GetCurrent(c, p_chr_1); F::BtlAct_IsTechniqueId(c, a0_) }) != 0)) as i32)) != 0) {
        l_dir_6 = (1i32).wrapping_neg();
        if ((((!((F::BtlInput_IsHeld(c, p_chr_1, ((65536i32) as u32))) != 0)) as i32)) != 0) {
            if ((F::BtlInput_IsHeld(c, p_chr_1, ((262144i32) as u32))) != 0) {
                l_dir_6 = 2i32;
            } else {
                if ((F::BtlInput_IsHeld(c, p_chr_1, ((524288i32) as u32))) != 0) {
                    l_dir_6 = 3i32;
                } else {
                    if ((F::BtlInput_IsHeld(c, p_chr_1, ((131072i32) as u32))) != 0) {
                        l_dir_6 = 4i32;
                    }
                }
            }
        }
        { let v_ = l_dir_6; let a_ = (p_chr_1).wrapping_add(5520); c.m.w32(a_, (v_) as u32); };
    }
    if ((((((F::BtlChar_TestFlag(c, p_chr_1, ((((6i32) as i64 as u64)) as u32))) != 0) && ((F::BtlMember_HasAbility(c, p_chr_1, 66i32)) != 0)) as i32)) != 0) {
        F::BtlChar_SetFlag(c, p_chr_1, ((((87i32) as i64 as u64)) as u32));
    }
    if (((({ let a0_ = F::BtlAnim_GetId(c, p_chr_1); F::BtlAnim_GetFlags(c, ((((a0_) as i64 as u64)) as u32)) }) & (((2048i32) as u32)))) != 0) {
        if ((F::BtlMember_HasAbility(c, p_chr_1, 57i32)) != 0) {
            F::BtlChar_SetFlag(c, p_chr_1, ((((87i32) as i64 as u64)) as u32));
        }
        if ((((({ let a_ = (F::BtlMember_GetActiveGauge(c, p_chr_1)).wrapping_add(40); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
            { let v_ = 0i32; let a_ = (p_chr_1).wrapping_add(4096); c.m.w32(a_, (v_) as u32); };
        } else {
            if ((((!(((({ let a0_ = F::BtlAnim_GetPrevId(c, p_chr_1); F::BtlAnim_GetFlags(c, ((((a0_) as i64 as u64)) as u32)) }) & (((2048i32) as u32)))) != 0)) as i32)) != 0) {
                { let v_ = 4i32; let a_ = (p_chr_1).wrapping_add(4096); c.m.w32(a_, (v_) as u32); };
            }
        }
    } else {
        { let v_ = 0i32; let a_ = (p_chr_1).wrapping_add(4096); c.m.w32(a_, (v_) as u32); };
    }
    if ((((((F::BtlChar_IsDead(c, p_chr_1)) != 0) && ((((c.m.r32((l_obj_2).wrapping_add(2624))) & (((262144i32) as u32)))) != 0)) as i32)) != 0) {
        F::BtlObj_SetColorMode(c, l_obj_2, 2i32, 0i32);
    }
    if (((((((((F::Battle_GetMode(c)) == (1i32)) as i32)) != 0) && ((((((c.m.r32((p_chr_1).wrapping_add(5504)) as i32)) > (0i32)) as i32)) != 0)) as i32)) != 0) {
        { let a_ = (p_chr_1).wrapping_add(5504); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
        F::BtlChar_SetFlag(c, p_chr_1, ((((286i32) as i64 as u64)) as u32));
    }
    { let a_ = (p_chr_1).wrapping_add(2404); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_UpdateTimers", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_Update(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_Update", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_act_2: u32 = 0;
    let mut l_tbl_3: u32 = 0;
    let r_ = 'f: {
    l_act_2 = (p_chr_1).wrapping_add(2376);
    l_tbl_3 = sym::gBtlActHandlers;
    F::BtlChar_SetStage(c, p_chr_1, ((((4i32) as i64 as u64)) as u8));
    if ((((({ let a_ = (l_tbl_3).wrapping_add(((((c.m.r32(l_act_2) as i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let _ = { let fp_ = { let a_ = (l_tbl_3).wrapping_add(((((c.m.r32(l_act_2) as i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) }; ((D::call(c, fp_, &[((p_chr_1) as u64), ((2i32) as i64 as u64)])) as i32) }; };
    }
    F::BtlAct_CheckForced(c, p_chr_1);
    if ((((((c.m.r32((l_act_2).wrapping_add(4)) as i32)) != ((1i32).wrapping_neg())) as i32)) != 0) {
        if ((((({ let a_ = (l_tbl_3).wrapping_add(((((c.m.r32(l_act_2) as i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
            { let _ = { let fp_ = { let a_ = (l_tbl_3).wrapping_add(((((c.m.r32(l_act_2) as i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) }; ((D::call(c, fp_, &[((p_chr_1) as u64), ((3i32) as i64 as u64)])) as i32) }; };
        }
        F::BtlChar_ApplyRootYaw(c, p_chr_1);
        { let v_ = (c.m.r32((l_act_2).wrapping_add(4)) as i32); let a_ = l_act_2; c.m.w32(a_, (v_) as u32); };
        { let v_ = (1i32).wrapping_neg(); let a_ = (l_act_2).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let _ = F::memset(c, (p_chr_1).wrapping_add(976), 0i32, 80u32); };
        { let v_ = 0i32; let a_ = (p_chr_1).wrapping_add(2404); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (p_chr_1).wrapping_add(3448); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (p_chr_1).wrapping_add(3452); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (p_chr_1).wrapping_add(3460); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (p_chr_1).wrapping_add(3564); c.m.w32(a_, (v_) as u32); };
        if ((((!((F::BtlChar_TestFlag(c, p_chr_1, ((((221i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) {
            F::ChrCam_EndCut(c, p_chr_1);
        }
        F::BtlAct_PopQueue(c, p_chr_1);
        F::BtlChar_ClearFlagRange(c, p_chr_1, ((((129i32) as i64 as u64)) as u32), ((((132i32) as i64 as u64)) as u32));
        F::BtlChar_ClearFlagRange(c, p_chr_1, ((((91i32) as i64 as u64)) as u32), ((((93i32) as i64 as u64)) as u32));
        F::BtlChar_ClearFlagRange(c, p_chr_1, ((((221i32) as i64 as u64)) as u32), ((((221i32) as i64 as u64)) as u32));
        { let v_ = 0i32; let a_ = (p_chr_1).wrapping_add(3912); c.m.w32(a_, (v_) as u32); };
        F::BtlChar_SetFlag(c, p_chr_1, ((((8i32) as i64 as u64)) as u32));
        F::BtlAct_ApplyFlag94(c, p_chr_1);
        if ((((({ let a_ = (l_tbl_3).wrapping_add(((((c.m.r32(l_act_2) as i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
            { let _ = { let fp_ = { let a_ = (l_tbl_3).wrapping_add(((((c.m.r32(l_act_2) as i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) }; ((D::call(c, fp_, &[((p_chr_1) as u64), ((0i32) as i64 as u64)])) as i32) }; };
        }
    } else {
        F::BtlAnim_FlushRequest(c, p_chr_1);
        F::BtlAnim_FlushRequestKeep(c, p_chr_1);
        F::BtlAnim_FlushSubToMain(c, p_chr_1);
        F::BtlChar_ApplyRootYaw(c, p_chr_1);
    }
    F::BtlChar_SetStage(c, p_chr_1, ((((5i32) as i64 as u64)) as u8));
    { let v_ = 0i32; let a_ = (p_chr_1).wrapping_add(2416); c.m.w32(a_, (v_) as u32); };
    if ((((({ let a_ = (sym::gBtlActHandlers).wrapping_add(((((c.m.r32(l_act_2) as i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let _ = { let fp_ = { let a_ = (sym::gBtlActHandlers).wrapping_add(((((c.m.r32(l_act_2) as i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) }; ((D::call(c, fp_, &[((p_chr_1) as u64), ((1i32) as i64 as u64)])) as i32) }; };
    }
    F::BtlAct_UpdateGauges(c, p_chr_1);
    F::BtlAct_UpdateTimers(c, p_chr_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_Update", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_Action01(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_Action01", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let mut l_obj_3: u32 = 0;
    let mut l_frameA_4: F32 = F32(0);
    let mut l_frameB_5: F32 = F32(0);
    let mut l_frame_6: F32 = F32(0);
    let r_ = 'f: {
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        F::BtlAnim_Play(c, p_chr_1, 384i32, F32(0x00000000));
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        's1: {
            let sv7: i32 = F::BtlAnim_GetId(c, p_chr_1);
            match sv7 {
                384i32 => {
                    { let _ = F::BtlAnim_AdvanceThen(c, p_chr_1, 385i32, F32(0x00000000), 0i32); };
                    break 's1;
                }
                385i32 => {
                    F::BtlAnim_AdvanceLoop(c, p_chr_1, 0i32);
                    break 's1;
                }
                _ => {
                }
            }
        }
        F::BtlMove_Step(c, p_chr_1, 6i32, 6i32, 7i32, F32(0x00000000), F32(0x3f6d0979));
        F::BtlMove_ApplyGravity(c, p_chr_1);
        l_obj_3 = F::BtlChar_GetObj(c, p_chr_1);
        l_frameA_4 = rt::i2f(F::BtlObjAnim_QueryEvent(c, l_obj_3, ((512i32) as u64), 0i32, ((((0i32) as i64 as u64)) as u32)));
        l_frameB_5 = rt::i2f(F::BtlObjAnim_QueryEvent(c, l_obj_3, ((1024i32) as u64), 0i32, ((((0i32) as i64 as u64)) as u32)));
        l_frame_6 = F::BtlAnim_GetFrame(c, p_chr_1);
        if (((((((((((rt::fle(F32(0x00000000), l_frameA_4) as i32)) != 0) && (((rt::flt(l_frameA_4, l_frame_6) as i32)) != 0)) as i32)) != 0) && (((((((rt::flt(l_frameB_5, F32(0x00000000)) as i32)) != 0) || (((rt::flt(l_frame_6, l_frameB_5) as i32)) != 0)) as i32)) != 0)) as i32)) != 0) {
            F::BtlChar_SetFxBit(c, p_chr_1, 7i32);
            { let a1_ = F::BtlParam_GetChargeLoopSound(c, p_chr_1); F::BtlCharSnd_PlayCommon(c, p_chr_1, a1_) };
        }
        if (((((((rt::fle(F32(0x00000000), l_frameB_5) as i32)) != 0) && ((F::BtlAnim_PassedFrame(c, p_chr_1, l_frameB_5)) != 0)) as i32)) != 0) {
            F::BtlChar_SetFxBit(c, p_chr_1, 7i32);
            F::BtlChar_SetFxBit(c, p_chr_1, 65i32);
            { let a1_ = F::BtlParam_GetMaxPowerSound(c, p_chr_1); F::BtlCharSnd_PlayCommon(c, p_chr_1, a1_) };
        }
    }
    if (((((p_phase_2) == (2i32)) as i32)) != 0) {
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((240i32) as i64 as u64)) as u32))) != 0) {
            F::BtlAct_Request(c, p_chr_1, 11i32);
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_Action01", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_Action02(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_Action02", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let mut l_obj_3: u32 = 0;
    let r_ = 'f: {
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        l_obj_3 = F::BtlChar_GetObj(c, p_chr_1);
        F::BtlAnim_Play(c, p_chr_1, 386i32, F32(0x3e199999));
        F::BtlChar_SetFlag(c, p_chr_1, ((((243i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((36i32) as i64 as u64)) as u32));
        { let v_ = F32(0x00000000); let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(152); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(156); c.m.w32(a_, (v_).0); };
        { let v_ = 0i32; let a_ = (p_chr_1).wrapping_add(4064); c.m.w32(a_, (v_) as u32); };
        if ((((c.m.r32((l_obj_3).wrapping_add(2624))) & (((262144i32) as u32)))) != 0) {
            F::BtlObj_SetColorMode(c, l_obj_3, 2i32, 0i32);
        }
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        's1: {
            let sv4: i32 = F::BtlAnim_GetId(c, p_chr_1);
            match sv4 {
                386i32 => {
                    { let _ = F::BtlAnim_AdvanceThen(c, p_chr_1, 387i32, F32(0x00000000), 0i32); };
                    break 's1;
                }
                387i32 => {
                    F::BtlAnim_AdvanceLoop(c, p_chr_1, 0i32);
                    break 's1;
                }
                _ => {
                }
            }
        }
        F::BtlMove_Step(c, p_chr_1, 6i32, 6i32, 7i32, F32(0x00000000), F32(0x3f6d0979));
        F::BtlMove_ApplyGravity(c, p_chr_1);
        F::BtlChar_SetFlag(c, p_chr_1, ((((66i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((67i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((68i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((69i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((70i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((293i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((187i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((63i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((298i32) as i64 as u64)) as u32));
        F::BtlChar_SetFxBit(c, p_chr_1, 59i32);
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_Action02", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_Action03(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_Action03", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let r_ = 'f: {
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        F::BtlAnim_Play(c, p_chr_1, 388i32, F32(0x3e199999));
        F::BtlChar_SetFlag(c, p_chr_1, ((((243i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((36i32) as i64 as u64)) as u32));
        { let v_ = F32(0x00000000); let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(152); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(156); c.m.w32(a_, (v_).0); };
        { let v_ = 0i32; let a_ = (p_chr_1).wrapping_add(4088); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (p_chr_1).wrapping_add(4092); c.m.w32(a_, (v_) as u32); };
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        { let _ = F::BtlAnim_Advance(c, p_chr_1, 0i32); };
        F::BtlMove_Step(c, p_chr_1, 6i32, 6i32, 7i32, F32(0x00000000), F32(0x3f6d0979));
        F::BtlMove_ApplyGravity(c, p_chr_1);
        F::BtlChar_SetFlag(c, p_chr_1, ((((66i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((67i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((68i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((69i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((70i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((293i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((187i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((63i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((298i32) as i64 as u64)) as u32));
        F::BtlChar_SetFxBit(c, p_chr_1, 20i32);
        F::BtlChar_SetFxBit(c, p_chr_1, 59i32);
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_Action03", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_Action04(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_Action04", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let mut l_motion_3: i32 = 0;
    let mut l_blend_4: F32 = F32(0);
    let mut l_pose_5: u32 = 0;
    let r_ = 'f: {
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        l_motion_3 = (c.m.r32((p_chr_1).wrapping_add(5456)) as i32);
        l_blend_4 = F32(c.m.r32((p_chr_1).wrapping_add(5460)));
        if (((((F::BtlAct_GetPrev(c, p_chr_1)) != (4i32)) as i32)) != 0) {
            l_blend_4 = F32(0x00000000);
        }
        F::BtlAnim_Play(c, p_chr_1, l_motion_3, l_blend_4);
        { let v_ = F32(0x00000000); let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(152); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(156); c.m.w32(a_, (v_).0); };
        F::BtlChar_SetFlag(c, p_chr_1, ((((41i32) as i64 as u64)) as u32));
        F::BtlAct_EndFlag98(c, p_chr_1);
        F::BtlChar_ClearFlag(c, p_chr_1, ((((190i32) as i64 as u64)) as u32));
        { let v_ = 0i32; let a_ = (p_chr_1).wrapping_add(4064); c.m.w32(a_, (v_) as u32); };
        F::BtlChar_ClearFlag(c, p_chr_1, ((((250i32) as i64 as u64)) as u32));
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        's1: {
            let sv6: i32 = (c.m.r32((p_chr_1).wrapping_add(5464)) as i32);
            match sv6 {
                0i32 => {
                    { let _ = F::BtlAnim_Advance(c, p_chr_1, 0i32); };
                    break 's1;
                }
                1i32 => {
                    F::BtlAnim_AdvanceLoop(c, p_chr_1, 0i32);
                    break 's1;
                }
                _ => {
                }
            }
        }
        { let a0_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(112); F::Vec4_SetZero(c, a0_) };
        { let v_ = 1i32; let a_ = (p_chr_1).wrapping_add(2416); c.m.w32(a_, (v_) as u32); };
        F::BtlChar_SetFlag(c, p_chr_1, ((((63i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((296i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((66i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((67i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((68i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((69i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((70i32) as i64 as u64)) as u32));
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((254i32) as i64 as u64)) as u32))) != 0) {
            F::BtlChar_SetFxBit(c, p_chr_1, 19i32);
        }
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((255i32) as i64 as u64)) as u32))) != 0) {
            F::BtlChar_SetFxBit(c, p_chr_1, 20i32);
        }
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((256i32) as i64 as u64)) as u32))) != 0) {
            F::BtlChar_SetFlag(c, p_chr_1, ((((48i32) as i64 as u64)) as u32));
        }
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((257i32) as i64 as u64)) as u32))) != 0) {
            F::BtlChar_SetFxBit(c, p_chr_1, 7i32);
            { let a1_ = F::BtlParam_GetChargeLoopSound(c, p_chr_1); F::BtlCharSnd_PlayCommon(c, p_chr_1, a1_) };
        }
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((258i32) as i64 as u64)) as u32))) != 0) {
            F::BtlChar_ClearFlag(c, p_chr_1, ((((258i32) as i64 as u64)) as u32));
            F::BtlChar_ClearFlag(c, p_chr_1, ((((257i32) as i64 as u64)) as u32));
            F::BtlChar_SetFxBit(c, p_chr_1, 7i32);
            F::BtlChar_SetFxBit(c, p_chr_1, 8i32);
            { let a1_ = F::BtlParam_GetMaxPowerSound(c, p_chr_1); F::BtlCharSnd_PlayCommon(c, p_chr_1, a1_) };
        }
        if (((rt::flt(F32(0x40a00000), F::BtlAct_GetHeight(c, p_chr_1)) as i32)) != 0) {
            F::BtlChar_SetFxBit(c, p_chr_1, 58i32);
        } else {
            F::BtlChar_SetFxBit(c, p_chr_1, 59i32);
        }
        { let v_ = 3i32; let a_ = (p_chr_1).wrapping_add(5504); c.m.w32(a_, (v_) as u32); };
    }
    if (((((p_phase_2) == (2i32)) as i32)) != 0) {
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((250i32) as i64 as u64)) as u32))) != 0) {
            F::BtlAct_Request(c, p_chr_1, 4i32);
        }
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((251i32) as i64 as u64)) as u32))) != 0) {
            if (((({ let a0_ = (c.m.r32((p_chr_1).wrapping_add(5456)) as i32); F::BtlAnim_GetFlags(c, ((((a0_) as i64 as u64)) as u32)) }) & (((2048i32) as u32)))) != 0) {
                F::BtlAct_Request(c, p_chr_1, 216i32);
            } else {
                F::BtlAct_Request(c, p_chr_1, 11i32);
            }
            if ((F::BtlAct_CheckStoryForced(c, p_chr_1)) != 0) {
                { let a1_ = F::BtlAct_GetQueued(c, p_chr_1); F::BtlAct_Request(c, p_chr_1, a1_) };
            }
        }
    }
    if (((((p_phase_2) == (3i32)) as i32)) != 0) {
        if (((((F::BtlAct_GetRequested(c, p_chr_1)) != (4i32)) as i32)) != 0) {
            if (((rt::flt(F::BtlAct_GetHeight(c, p_chr_1), F32(0x40a00000)) as i32)) != 0) {
                F::BtlChar_SetHeldFlag(c, p_chr_1, ((((15i32) as i64 as u64)) as u32));
                l_pose_5 = F::BtlChar_GetPos(c, p_chr_1);
                { let v_ = { let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); F::BtlCharApi_GetGroundY(c, a0_) }; let a_ = (l_pose_5).wrapping_add(4); c.m.w32(a_, (v_).0); };
                F::BtlChar_ClearFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32));
            } else {
                F::BtlChar_SetHeldFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32));
            }
            F::BtlChar_ClearFlag(c, p_chr_1, ((((254i32) as i64 as u64)) as u32));
            F::BtlChar_ClearFlag(c, p_chr_1, ((((255i32) as i64 as u64)) as u32));
            F::BtlChar_ClearFlag(c, p_chr_1, ((((257i32) as i64 as u64)) as u32));
            F::BtlChar_ClearFlag(c, p_chr_1, ((((258i32) as i64 as u64)) as u32));
            F::BtlChar_ClearFlag(c, p_chr_1, ((((256i32) as i64 as u64)) as u32));
            break 'f { F::BtlChar_SetFlag(c, p_chr_1, ((((55i32) as i64 as u64)) as u32)); 0 };
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_Action04", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAct_Action05(c: &mut Ctx, mut p_chr_1: u32, mut p_phase_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_Action05", &PROBE_N, &[(p_chr_1) as u32, (p_phase_2) as u32], false); }
    let mut l_rate_3: F32 = F32(0);
    let mut l_moving_4: i32 = 0;
    let mut l_flag_5: i32 = 0;
    let r_ = 'f: {
    if (((((p_phase_2) == (0i32)) as i32)) != 0) {
        F::BtlAnim_Play(c, p_chr_1, 24i32, F32(0x3e199999));
        F::BtlChar_PlayVoice(c, p_chr_1, 36i32);
        F::BtlChar_SetHeldFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32));
    }
    if (((((p_phase_2) == (1i32)) as i32)) != 0) {
        l_rate_3 = F32(0x00000000);
        l_moving_4 = 1i32;
        l_flag_5 = 1i32;
        's1: {
            let sv6: i32 = F::BtlAnim_GetId(c, p_chr_1);
            match sv6 {
                24i32 => {
                    { let _ = F::BtlAnim_AdvanceThen(c, p_chr_1, 25i32, F32(0x3e4ccccc), 0i32); };
                    l_rate_3 = F::BtlAnim_GetProgress(c, p_chr_1);
                    l_moving_4 = 0i32;
                    break 's1;
                }
                25i32 => {
                    F::BtlAnim_AdvanceLoop(c, p_chr_1, 0i32);
                    if ((F::BtlAnim_IsNew(c, p_chr_1)) != 0) {
                        { let a1_ = F::BtlParam_GetDashSound(c, p_chr_1); F::BtlCharSnd_PlayCommon(c, p_chr_1, a1_) };
                        F::BtlChar_SetFlag(c, p_chr_1, ((((209i32) as i64 as u64)) as u32));
                        if (((rt::flt(F::BtlAct_GetHeight(c, p_chr_1), F32(0x41a00000)) as i32)) != 0) {
                            F::BtlChar_SetFxBit(c, p_chr_1, 53i32);
                        }
                    }
                    l_rate_3 = F32(0x3f800000);
                    l_moving_4 = 1i32;
                    break 's1;
                }
                26i32 => {
                    if ((F::BtlAnim_Advance(c, p_chr_1, 0i32)) != 0) {
                        F::BtlAct_Request(c, p_chr_1, 11i32);
                        { let v_ = l_rate_3; let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(152); c.m.w32(a_, (v_).0); };
                    }
                    l_moving_4 = 0i32;
                    l_rate_3 = rt::fsub(F32(0x3f800000), F::BtlAnim_GetProgress(c, p_chr_1));
                    l_flag_5 = 0i32;
                    break 's1;
                }
                _ => {
                }
            }
        }
        if ((l_moving_4) != 0) {
            { let a4_ = F::BtlMoveParam_GetSpeed(c, p_chr_1, 6i32); F::BtlMove_Step(c, p_chr_1, 2i32, 1i32, 3i32, a4_, F32(0x461c4000)) };
            F::BtlMove_ApplyGravity(c, p_chr_1);
            F::BtlChar_SetFlag(c, p_chr_1, ((((9i32) as i64 as u64)) as u32));
            F::BtlChar_SetFlag(c, p_chr_1, ((((80i32) as i64 as u64)) as u32));
            F::BtlChar_SetFlag(c, p_chr_1, ((((81i32) as i64 as u64)) as u32));
            F::BtlChar_SetFxBit(c, p_chr_1, 4i32);
            F::BtlChar_SetFxBit(c, p_chr_1, 9i32);
            F::BtlChar_SetFxBit(c, p_chr_1, 58i32);
        } else {
            F::BtlMove_Step(c, p_chr_1, (if ((l_flag_5) != 0) { 2i32 } else { 6i32 }), 6i32, 7i32, F32(0x00000000), F32(0x3f6d0979));
            F::BtlMove_ApplyGravity(c, p_chr_1);
        }
        { let a1_ = rt::fmul({ let a_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(144); F32(c.m.r32(a_)) }, l_rate_3); F::BtlMove_SetLeanX(c, p_chr_1, a1_) };
        F::BtlChar_SetFlag(c, p_chr_1, ((((66i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((67i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((68i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((69i32) as i64 as u64)) as u32));
        F::BtlChar_SetFlag(c, p_chr_1, ((((70i32) as i64 as u64)) as u32));
        's2: {
            let sv7: i32 = F::BtlAct_GetCurrent(c, p_chr_1);
            match sv7 {
                5i32 => {
                    F::BtlChar_SetFlag(c, p_chr_1, ((((71i32) as i64 as u64)) as u32));
                    break 's2;
                }
                6i32 => {
                    F::BtlChar_SetFlag(c, p_chr_1, ((((72i32) as i64 as u64)) as u32));
                    break 's2;
                }
                _ => {
                }
            }
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAct_Action05", &PROBE_N, &[], true); }
    r_
}
