#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn BtlChar_ObjToPose(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_ObjToPose", &PROBE_N, &[(p_chr_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_obj_2: u32 = 0;
    let mut l_pose_3: u32 = 0;
    let mut l_node_4: u32 = 0;
    let r_ = 'f: {
        l_obj_2 = F::BtlChar_GetObj(c, p_chr_1);
        l_pose_3 = F::BtlChar_GetPos(c, p_chr_1);
        F::Vec4_Sub(c, l_pose_3, (l_obj_2).wrapping_add(2416), (l_pose_3).wrapping_add(32));
        F::Vec4_Copy(c, (l_pose_3).wrapping_add(16), (l_obj_2).wrapping_add(2400));
        l_node_4 = F::BtlObj_GetNode(c, l_obj_2, 0i32);
        if (((((l_node_4) != (((0i32) as u32))) as i32)) != 0) {
            F::Vec4_Copy(c, (l_pose_3).wrapping_add(80), (l_node_4).wrapping_add(144));
            F::Vec4_Copy(c, fp, (l_node_4).wrapping_add(160));
            F::Quat_ToEuler(c, (l_pose_3).wrapping_add(96), fp);
        } else {
            F::Vec4_SetZeroW1(c, (l_pose_3).wrapping_add(80));
            F::Vec4_SetZero(c, (l_pose_3).wrapping_add(96));
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_ObjToPose", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChar_PoseToObj(c: &mut Ctx, mut p_chr_1: u32, mut p_keepRoot_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_PoseToObj", &PROBE_N, &[(p_chr_1) as u32, (p_keepRoot_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(96) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 96);
    let mut l_obj_3: u32 = 0;
    let mut l_pose_4: u32 = 0;
    let mut l_apply_5: i32 = 0;
    let r_ = 'f: {
        l_obj_3 = F::BtlChar_GetObj(c, p_chr_1);
        l_pose_4 = F::BtlChar_GetPos(c, p_chr_1);
        F::Vec4_Copy(c, fp, (l_obj_3).wrapping_add(2384));
        F::Vec4_Add(c, (l_obj_3).wrapping_add(2384), l_pose_4, (l_pose_4).wrapping_add(32));
        F::Vec4_Copy(c, (l_obj_3).wrapping_add(2400), (l_pose_4).wrapping_add(16));
        { let v_ = F32(c.m.r32((p_chr_1).wrapping_add(2440))); let a_ = (l_obj_3).wrapping_add(2592); c.m.w32(a_, (v_).0); };
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((43i32) as i64 as u64)) as u32))) != 0) {
            l_apply_5 = 0i32;
        } else {
            l_apply_5 = 1i32;
        }
        if (((((p_keepRoot_2) == (0i32)) as i32)) != 0) {
            l_apply_5 = 1i32;
        }
        if ((l_apply_5) != 0) {
            F::Mtx_StoreIdentity(c, fp.wrapping_add(16));
            F::Mtx_RotateY(c, fp.wrapping_add(16), fp.wrapping_add(16), F32(0x40490fda));
            { let a2_ = F32(c.m.r32(((l_obj_3).wrapping_add(2400)).wrapping_add(4))); F::Mtx_RotateY(c, fp.wrapping_add(16), fp.wrapping_add(16), a2_) };
            { let a2_ = F32(c.m.r32((l_obj_3).wrapping_add(2400))); F::Mtx_RotateX(c, fp.wrapping_add(16), fp.wrapping_add(16), a2_) };
            { let a2_ = F32(c.m.r32(((l_obj_3).wrapping_add(2400)).wrapping_add(8))); F::Mtx_RotateZ(c, fp.wrapping_add(16), fp.wrapping_add(16), a2_) };
            F::Mtx_MulVec4(c, fp.wrapping_add(80), fp.wrapping_add(16), (l_pose_4).wrapping_add(80));
            { let a2_ = F32(c.m.r32((l_obj_3).wrapping_add(2592))); F::Vec4_Scale(c, fp.wrapping_add(80), fp.wrapping_add(80), a2_) };
            F::Vec4_Sub(c, (l_obj_3).wrapping_add(2384), (l_obj_3).wrapping_add(2384), fp.wrapping_add(80));
        }
        F::Vec4_Sub(c, fp.wrapping_add(16), (l_obj_3).wrapping_add(2384), fp);
        if (((rt::flt(F::Vec3_Length(c, fp.wrapping_add(16)), F32(0x3a83126e)) as i32)) != 0) {
            F::Vec4_Copy(c, (l_obj_3).wrapping_add(2384), fp);
        }
        F::BtlObjXf_Update(c, l_obj_3);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_PoseToObj", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChar_ApplyRootYaw(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_ApplyRootYaw", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_pose_2: u32 = 0;
    let mut l_yaw_3: F32 = F32(0);
    let r_ = 'f: {
    l_pose_2 = F::BtlChar_GetPos(c, p_chr_1);
    if ((((((F::BtlChar_TestFlag(c, p_chr_1, ((((51i32) as i64 as u64)) as u32))) != 0) && ((((!((F::BtlChar_TestFlag(c, p_chr_1, ((((148i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0)) as i32)) != 0) {
        l_yaw_3 = F32(c.m.r32(((l_pose_2).wrapping_add(96)).wrapping_add(4)));
        { let v_ = { let v_ = { let a0_ = rt::fadd(l_yaw_3, F32(c.m.r32(((l_pose_2).wrapping_add(16)).wrapping_add(4)))); F::BtlUtil_WrapAngle(c, a0_) }; let a_ = ((l_pose_2).wrapping_add(16)).wrapping_add(4); c.m.w32(a_, (v_).0); v_ }; let a_ = (l_pose_2).wrapping_add(148); c.m.w32(a_, (v_).0); };
        { let r_ = F::Mathf_Cos(c, l_yaw_3); let a_ = (l_pose_2).wrapping_add(144); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_ApplyRootYaw", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChar_UpdateLean(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_UpdateLean", &PROBE_N, &[(p_chr_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_pose_2: u32 = 0;
    let mut l_obj_3: u32 = 0;
    let r_ = 'f: {
        l_pose_2 = F::BtlChar_GetPos(c, p_chr_1);
        l_obj_3 = F::BtlChar_GetObj(c, p_chr_1);
        { let a1_ = rt::fneg(F32(c.m.r32((l_pose_2).wrapping_add(160)))); let a3_ = F32(c.m.r32((l_pose_2).wrapping_add(164))); F::Vec4_Set(c, fp, a1_, F32(0x00000000), a3_, F32(0x00000000)) };
        F::Quat_FromEuler(c, (l_obj_3).wrapping_add(3360), fp);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_UpdateLean", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChar_Place(c: &mut Ctx, mut p_chr_1: u32, mut p_pos_2: u32, mut p_rot_3: u32, mut p_area_4: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_Place", &PROBE_N, &[(p_chr_1) as u32, (p_pos_2) as u32, (p_rot_3) as u32, (p_area_4) as u32], false); }
    let mut l_pose_5: u32 = 0;
    let mut l_obj_6: u32 = 0;
    let r_ = 'f: {
    l_pose_5 = F::BtlChar_GetPos(c, p_chr_1);
    l_obj_6 = F::BtlChar_GetObj(c, p_chr_1);
    F::Vec4_Copy(c, l_pose_5, p_pos_2);
    F::Vec4_Copy(c, (l_pose_5).wrapping_add(16), p_rot_3);
    F::Vec4_Copy(c, (l_pose_5).wrapping_add(48), sym::D_002EC2A0);
    F::Vec4_Copy(c, (l_pose_5).wrapping_add(64), sym::D_002EC2A0);
    { let v_ = F32(c.m.r32((p_rot_3).wrapping_add(4))); let a_ = (l_pose_5).wrapping_add(148); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x00000000); let a_ = (l_pose_5).wrapping_add(144); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x00000000); let a_ = (l_pose_5).wrapping_add(152); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x00000000); let a_ = (l_pose_5).wrapping_add(156); c.m.w32(a_, (v_).0); };
    F::Vec4_Copy(c, (l_obj_6).wrapping_add(2384), p_pos_2);
    F::Vec4_Copy(c, (l_obj_6).wrapping_add(2416), p_pos_2);
    F::Vec4_Copy(c, (l_obj_6).wrapping_add(2400), p_rot_3);
    { let v_ = p_area_4; let a_ = (l_obj_6).wrapping_add(2596); c.m.w32(a_, (v_) as u32); };
    { let v_ = F32(c.m.r32(p_pos_2)); let a_ = ((p_chr_1).wrapping_add(1056)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fsub(F32(c.m.r32((p_pos_2).wrapping_add(4))), { let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); F::BtlCharApi_GetHeight(c, a0_) }); let a_ = ((p_chr_1).wrapping_add(1056)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((p_pos_2).wrapping_add(8))); let a_ = ((p_chr_1).wrapping_add(1056)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((p_rot_3).wrapping_add(4))); let a_ = (p_chr_1).wrapping_add(1184); c.m.w32(a_, (v_).0); };
    { let v_ = { let a0_ = F32(c.m.r32((p_rot_3).wrapping_add(4))); F::Mathf_Sin(c, a0_) }; let a_ = (l_pose_5).wrapping_add(128); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x00000000); let a_ = ((l_pose_5).wrapping_add(128)).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = { let a0_ = F32(c.m.r32((p_rot_3).wrapping_add(4))); F::Mathf_Cos(c, a0_) }; let a_ = ((l_pose_5).wrapping_add(128)).wrapping_add(8); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x00000000); let a_ = ((l_pose_5).wrapping_add(128)).wrapping_add(12); c.m.w32(a_, (v_).0); };
    F::BtlChar_SetFlag(c, p_chr_1, ((((205i32) as i64 as u64)) as u32));
    F::BtlChar_SetFlag(c, p_chr_1, ((((36i32) as i64 as u64)) as u32));
    F::BtlChar_SetFlag(c, p_chr_1, ((((63i32) as i64 as u64)) as u32));
    F::BtlObjBody_Reset(c, l_obj_6);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_Place", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChar_PlaceAtStart(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_PlaceAtStart", &PROBE_N, &[(p_chr_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let r_ = 'f: {
        { let a3_ = { let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlStage_GetStartPlace(c, a0_, fp, fp.wrapping_add(16), 0i32) }; F::BtlChar_Place(c, p_chr_1, fp, fp.wrapping_add(16), a3_) };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_PlaceAtStart", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChar_PlaceRestart(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_PlaceRestart", &PROBE_N, &[(p_chr_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let r_ = 'f: {
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((243i32) as i64 as u64)) as u32))) != 0) {
            { let a3_ = { let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlStage_GetStartPlace(c, a0_, fp, fp.wrapping_add(16), 1i32) }; F::BtlChar_Place(c, p_chr_1, fp, fp.wrapping_add(16), a3_) };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_PlaceRestart", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChar_PlaceCenter(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_PlaceCenter", &PROBE_N, &[(p_chr_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let r_ = 'f: {
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((244i32) as i64 as u64)) as u32))) != 0) {
            { let a3_ = F::BtlStage_GetPlace(c, fp, fp.wrapping_add(16)); F::BtlChar_Place(c, p_chr_1, fp, fp.wrapping_add(16), a3_) };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_PlaceCenter", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChar_PlaceRelative(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_PlaceRelative", &PROBE_N, &[(p_chr_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_area_2: i32 = 0;
    let r_ = 'f: {
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((246i32) as i64 as u64)) as u32))) != 0) {
            l_area_2 = F::BtlStage_GetPlace(c, fp, fp.wrapping_add(16));
            F::Vec4_Add(c, fp, fp, (p_chr_1).wrapping_add(3696));
            F::Vec4_Add(c, fp.wrapping_add(16), fp.wrapping_add(16), (p_chr_1).wrapping_add(3712));
            F::BtlUtil_WrapAngles(c, fp.wrapping_add(16), fp.wrapping_add(16));
            F::BtlChar_Place(c, p_chr_1, fp, fp.wrapping_add(16), l_area_2);
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_PlaceRelative", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChar_PlaceOnPath(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_PlaceOnPath", &PROBE_N, &[(p_chr_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(64) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 64);
    let mut l_work_2: u32 = 0;
    let mut l_path_3: u32 = 0;
    let mut l_index_4: i32 = 0;
    let mut l_first_5: u32 = 0;
    let mut l_point_6: u32 = 0;
    let mut l_len_7: F32 = F32(0);
    let mut l_margin_8: F32 = F32(0);
    let r_ = 'f: {
        l_work_2 = (c.m.r32(sym::gBtlChars)).wrapping_add(124);
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((248i32) as i64 as u64)) as u32))) != 0) {
            l_path_3 = { let a0_ = (c.m.r32((l_work_2).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32); F::BtlStage_GetPath(c, a0_) };
            if (((((l_path_3) != (((0i32) as u32))) as i32)) != 0) {
                l_index_4 = (rt::rem_i32((c.m.r32((l_work_2).wrapping_add((((3i32) as u32)).wrapping_mul(4))) as i32), ((c.m.r32(l_path_3) as i32)).wrapping_sub((c.m.r32((l_path_3).wrapping_add(8)) as i32)))).wrapping_add((c.m.r32((l_path_3).wrapping_add(8)) as i32));
                l_first_5 = c.m.r32((l_path_3).wrapping_add(4));
                l_point_6 = (l_first_5).wrapping_add((((l_index_4) as u32)).wrapping_mul(48));
                { let v_ = rt::fsub(F32(c.m.r32(l_point_6)), F32(c.m.r32(l_first_5))); let a_ = fp.wrapping_add(16); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fsub(F32(c.m.r32((l_point_6).wrapping_add(4))), F32(c.m.r32((l_first_5).wrapping_add(4)))); let a_ = (fp.wrapping_add(16)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fsub(F32(c.m.r32((l_point_6).wrapping_add(8))), F32(c.m.r32((l_first_5).wrapping_add(8)))); let a_ = (fp.wrapping_add(16)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                { let v_ = F32(0x00000000); let a_ = (fp.wrapping_add(16)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                { let v_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8))); let a_ = fp; c.m.w32(a_, (v_).0); };
                { let v_ = F32(0x00000000); let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fneg(F32(c.m.r32(fp.wrapping_add(16)))); let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_).0); };
                { let v_ = F32(0x00000000); let a_ = (fp).wrapping_add(12); c.m.w32(a_, (v_).0); };
                l_len_7 = F::Vec3_Length(c, fp);
                if (((rt::flt(l_len_7, F32(0x3a83126e)) as i32)) != 0) {
                    break 'f;
                }
                l_margin_8 = F32(0x41f00000);
                { let a2_ = rt::fdiv(rt::fadd({ let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); F::BtlCharApi_GetRadius(c, a0_) }, l_margin_8), l_len_7); F::Vec4_Scale(c, fp, fp, a2_) };
                { let v_ = F32(c.m.r32(l_point_6)); let a_ = fp.wrapping_add(32); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fadd(F32(c.m.r32((l_point_6).wrapping_add(4))), { let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); F::BtlCharApi_GetCenterHeight(c, a0_) }); let a_ = (fp.wrapping_add(32)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                { let v_ = F32(c.m.r32((l_point_6).wrapping_add(8))); let a_ = (fp.wrapping_add(32)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                { let v_ = F32(0x3f800000); let a_ = (fp.wrapping_add(32)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                { let v_ = F32(0x00000000); let a_ = fp.wrapping_add(48); c.m.w32(a_, (v_).0); };
                { let v_ = F32(0x00000000); let a_ = (fp.wrapping_add(48)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                { let v_ = F32(0x00000000); let a_ = (fp.wrapping_add(48)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                { let v_ = F32(0x00000000); let a_ = (fp.wrapping_add(48)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                if ((((((c.m.r32(p_chr_1) as i32)) == (0i32)) as i32)) != 0) {
                    F::Vec4_Sub(c, fp.wrapping_add(32), fp.wrapping_add(32), fp);
                    { let v_ = { let a0_ = F32(c.m.r32(fp)); let a1_ = F32(c.m.r32((fp).wrapping_add(8))); F::Port_atan2f(c, a0_, a1_) }; let a_ = (fp.wrapping_add(48)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                } else {
                    F::Vec4_Add(c, fp.wrapping_add(32), fp.wrapping_add(32), fp);
                    { let v_ = { let a0_ = rt::fneg(F32(c.m.r32(fp))); let a1_ = rt::fneg(F32(c.m.r32((fp).wrapping_add(8)))); F::Port_atan2f(c, a0_, a1_) }; let a_ = (fp.wrapping_add(48)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                }
                { let a3_ = F::BtlStage_FindZoneNear(c, (1i32).wrapping_neg(), fp.wrapping_add(32)); F::BtlChar_Place(c, p_chr_1, fp.wrapping_add(32), fp.wrapping_add(48), a3_) };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_PlaceOnPath", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChar_PlaceCenterHigh(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_PlaceCenterHigh", &PROBE_N, &[(p_chr_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_area_2: i32 = 0;
    let mut l_y_3: F32 = F32(0);
    let r_ = 'f: {
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((247i32) as i64 as u64)) as u32))) != 0) {
            l_area_2 = F::BtlStage_GetPlace(c, fp, fp.wrapping_add(16));
            l_y_3 = F::BtlStage_GetTop(c);
            { let v_ = rt::fadd(l_y_3, rt::fmul({ let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); F::BtlCharApi_GetHeight(c, a0_) }, F32(0x3f000000))); let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
            F::BtlChar_Place(c, p_chr_1, fp, fp.wrapping_add(16), l_area_2);
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_PlaceCenterHigh", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChar_PlaceSaved(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_PlaceSaved", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    if ((F::BtlChar_TestFlag(c, p_chr_1, ((((245i32) as i64 as u64)) as u32))) != 0) {
        { let a3_ = (c.m.r32((p_chr_1).wrapping_add(4800)) as i32); F::BtlChar_Place(c, p_chr_1, (p_chr_1).wrapping_add(4768), (p_chr_1).wrapping_add(4784), a3_) };
        if (((c.m.r32((p_chr_1).wrapping_add(4804)) as i32)) != 0) {
            F::BtlChar_SetHeldFlag(c, p_chr_1, ((((15i32) as i64 as u64)) as u32));
        } else {
            F::BtlChar_ClearFlag(c, p_chr_1, ((((15i32) as i64 as u64)) as u32));
        }
        if (((c.m.r32((p_chr_1).wrapping_add(4808)) as i32)) != 0) {
            F::BtlChar_SetHeldFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32));
        } else {
            F::BtlChar_ClearFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32));
        }
        if (((c.m.r32((p_chr_1).wrapping_add(4804)) as i32)) != 0) {
            F::BtlChar_SetFlag(c, p_chr_1, ((((16i32) as i64 as u64)) as u32));
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_PlaceSaved", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChar_PlaceWarp(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_PlaceWarp", &PROBE_N, &[(p_chr_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_changed_2: i32 = 0;
    let mut l_area_3: i32 = 0;
    let r_ = 'f: {
        l_changed_2 = 0i32;
        's1: {
            let sv4: i32 = F::BtlAct_GetCurrent(c, p_chr_1);
            match sv4 {
                4i32 | 7i32 | 8i32 | 9i32 | 10i32 => {
                    { let a1_ = F::BtlChar_GetPos(c, p_chr_1); F::Vec4_Copy(c, fp, a1_) };
                    { let a1_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(16); F::Vec4_Copy(c, fp.wrapping_add(16), a1_) };
                    l_area_3 = { let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2596); (c.m.r32(a_) as i32) };
                    if ((F::BtlChar_TestFlag(c, p_chr_1, ((((252i32) as i64 as u64)) as u32))) != 0) {
                        F::BtlChar_ClearFlag(c, p_chr_1, ((((252i32) as i64 as u64)) as u32));
                        l_changed_2 = 1i32;
                        F::Vec4_Copy(c, fp, (p_chr_1).wrapping_add(5472));
                        l_area_3 = F::BtlStage_FindZoneNear(c, (1i32).wrapping_neg(), fp);
                    }
                    if ((F::BtlChar_TestFlag(c, p_chr_1, ((((253i32) as i64 as u64)) as u32))) != 0) {
                        F::BtlChar_ClearFlag(c, p_chr_1, ((((253i32) as i64 as u64)) as u32));
                        l_changed_2 = 1i32;
                        F::Vec4_Copy(c, fp.wrapping_add(16), (p_chr_1).wrapping_add(5488));
                    }
                    if ((l_changed_2) != 0) {
                        F::BtlChar_Place(c, p_chr_1, fp, fp.wrapping_add(16), l_area_3);
                    }
                    break 's1;
                }
                _ => {
                }
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_PlaceWarp", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChar_GetSpacing(c: &mut Ctx, mut p_chr_1: u32, mut p_mode_2: i32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_GetSpacing", &PROBE_N, &[(p_chr_1) as u32, (p_mode_2) as u32], false); }
    let mut l_r_3: F32 = F32(0);
    let r_ = 'f: {
    l_r_3 = F32(0x00000000);
    { let r_ = { let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); F::BtlCharApi_GetHeight(c, a0_) }; let n_ = rt::fadd(l_r_3, r_); l_r_3 = n_; };
    { let r_ = F::BtlOpp_GetHeight(c, p_chr_1); let n_ = rt::fadd(l_r_3, r_); l_r_3 = n_; };
    's1: {
        let sv4: i32 = p_mode_2;
        match sv4 {
            0i32 => {
                if ((F::BtlChar_TestFlag(c, p_chr_1, ((((19i32) as i64 as u64)) as u32))) != 0) {
                    { let r_ = F32(0x42200000); let n_ = rt::fadd(l_r_3, r_); l_r_3 = n_; };
                } else {
                    { let r_ = F32(0x41f00000); let n_ = rt::fadd(l_r_3, r_); l_r_3 = n_; };
                }
                break 's1;
            }
            1i32 => {
                { let r_ = F32(0x41f00000); let n_ = rt::fadd(l_r_3, r_); l_r_3 = n_; };
                break 's1;
            }
            2i32 => {
                { let r_ = F32(0x41f00000); let n_ = rt::fadd(l_r_3, r_); l_r_3 = n_; };
                break 's1;
            }
            _ => {
            }
        }
    }
    break 'f l_r_3;
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_GetSpacing", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChars_UpdateHold(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChars_UpdateHold", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let mut l_held_1: u32 = 0;
    let mut l_holder_2: u32 = 0;
    let mut l_i_3: i32 = 0;
    let mut l_chr_4: u32 = 0;
    let mut l_holderObj_5: u32 = 0;
    let mut l_heldObj_6: u32 = 0;
    let mut l_node_7: i32 = 0;
    let r_ = 'f: {
        l_held_1 = ((0i32) as u32);
        l_holder_2 = ((0i32) as u32);
        {
            l_i_3 = 0i32;
            'l1: loop {
                if !(((((l_i_3) < (F::BtlChar_GetCount(c))) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_chr_4 = F::BtlChar_Get(c, l_i_3);
                    if ((((!((F::BtlChar_IsFrozen(c, l_chr_4)) != 0)) as i32)) != 0) {
                        if ((F::BtlChar_TestFlag(c, l_chr_4, ((((139i32) as i64 as u64)) as u32))) != 0) {
                            l_holder_2 = l_chr_4;
                        }
                        if ((F::BtlChar_TestFlag(c, l_chr_4, ((((151i32) as i64 as u64)) as u32))) != 0) {
                            l_held_1 = l_chr_4;
                        }
                    }
                }
                { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
            }
        }
        if (((((((((((((l_holder_2) != (((0i32) as u32))) as i32)) != 0) && (((((l_held_1) != (((0i32) as u32))) as i32)) != 0)) as i32)) != 0) && (((((l_holder_2) != (l_held_1)) as i32)) != 0)) as i32)) != 0) {
            l_holderObj_5 = F::BtlChar_GetObj(c, l_holder_2);
            l_heldObj_6 = F::BtlChar_GetObj(c, l_held_1);
            l_node_7 = (c.m.r32((l_heldObj_6).wrapping_add(3252)) as i32);
            { let a0_ = (c.m.r32((l_holder_2).wrapping_add(12)) as i32); F::BtlCharApi_GetNodePos(c, a0_, 52i32, fp) };
            { let a0_ = (c.m.r32((l_held_1).wrapping_add(12)) as i32); F::BtlCharApi_GetNodePos(c, a0_, l_node_7, fp.wrapping_add(16)) };
            if ((F::BtlChar_TestFlag(c, l_holder_2, ((((155i32) as i64 as u64)) as u32))) != 0) {
                F::Vec4_Sub(c, fp.wrapping_add(32), fp.wrapping_add(16), fp);
                if ((F::BtlChar_TestFlag(c, l_held_1, ((((52i32) as i64 as u64)) as u32))) != 0) {
                    { let a2_ = rt::fsub(F32(0x3f800000), F32(c.m.r32((l_heldObj_6).wrapping_add(3204)))); F::Vec4_Scale(c, fp.wrapping_add(32), fp.wrapping_add(32), a2_) };
                }
                { let a0_ = F::BtlChar_GetPos(c, l_holder_2); let a1_ = F::BtlChar_GetPos(c, l_holder_2); F::Vec4_Add(c, a0_, a1_, fp.wrapping_add(32)) };
                F::BtlChar_PoseToObj(c, l_holder_2, 0i32);
                F::BtlObjPose_CalcMatrices(c, l_holderObj_5);
                F::BtlChar_ObjToPose(c, l_holder_2);
            } else {
                F::Vec4_Sub(c, fp.wrapping_add(32), fp, fp.wrapping_add(16));
                if ((F::BtlChar_TestFlag(c, l_held_1, ((((52i32) as i64 as u64)) as u32))) != 0) {
                    { let a2_ = rt::fsub(F32(0x3f800000), F32(c.m.r32((l_heldObj_6).wrapping_add(3204)))); F::Vec4_Scale(c, fp.wrapping_add(32), fp.wrapping_add(32), a2_) };
                }
                { let a0_ = F::BtlChar_GetPos(c, l_held_1); let a1_ = F::BtlChar_GetPos(c, l_held_1); F::Vec4_Add(c, a0_, a1_, fp.wrapping_add(32)) };
                F::BtlChar_PoseToObj(c, l_held_1, 0i32);
                F::BtlObjPose_CalcMatrices(c, l_heldObj_6);
                F::BtlChar_ObjToPose(c, l_held_1);
                F::BtlChar_SetFlag(c, l_held_1, ((((35i32) as i64 as u64)) as u32));
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlChars_UpdateHold", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChar_SavePlacement(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_SavePlacement", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_obj_2: u32 = 0;
    let r_ = 'f: {
    l_obj_2 = F::BtlChar_GetObj(c, p_chr_1);
    { let a1_ = F::BtlChar_GetPos(c, p_chr_1); F::Vec4_Copy(c, (p_chr_1).wrapping_add(4768), a1_) };
    { let a1_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(16); F::Vec4_Copy(c, (p_chr_1).wrapping_add(4784), a1_) };
    { let a1_ = c.m.r32((l_obj_2).wrapping_add(4000)); F::Vec4_Copy(c, (p_chr_1).wrapping_add(4880), a1_) };
    { let v_ = { let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2596); (c.m.r32(a_) as i32) }; let a_ = (p_chr_1).wrapping_add(4800); c.m.w32(a_, (v_) as u32); };
    { let v_ = F::BtlChar_TestFlag(c, p_chr_1, ((((15i32) as i64 as u64)) as u32)); let a_ = (p_chr_1).wrapping_add(4804); c.m.w32(a_, (v_) as u32); };
    { let v_ = F::BtlChar_TestFlag(c, p_chr_1, ((((14i32) as i64 as u64)) as u32)); let a_ = (p_chr_1).wrapping_add(4808); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_SavePlacement", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChar_SetSavedPlacement(c: &mut Ctx, mut p_chr_1: u32, mut p_pos_2: u32, mut p_rot_3: u32, mut p_unk1310_4: u32, mut p_flagF_5: i32, mut p_flagE_6: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_SetSavedPlacement", &PROBE_N, &[(p_chr_1) as u32, (p_pos_2) as u32, (p_rot_3) as u32, (p_unk1310_4) as u32, (p_flagF_5) as u32, (p_flagE_6) as u32], false); }
    let r_ = 'f: {
    F::Vec4_Copy(c, (p_chr_1).wrapping_add(4768), p_pos_2);
    F::Vec4_Copy(c, (p_chr_1).wrapping_add(4784), p_rot_3);
    F::Vec4_Copy(c, (p_chr_1).wrapping_add(4880), p_unk1310_4);
    { let v_ = F::BtlStage_FindZoneNear(c, (1i32).wrapping_neg(), p_pos_2); let a_ = (p_chr_1).wrapping_add(4800); c.m.w32(a_, (v_) as u32); };
    { let v_ = p_flagF_5; let a_ = (p_chr_1).wrapping_add(4804); c.m.w32(a_, (v_) as u32); };
    { let v_ = p_flagE_6; let a_ = (p_chr_1).wrapping_add(4808); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_SetSavedPlacement", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChar_SetUnk1310(c: &mut Ctx, mut p_chr_1: u32, mut p_v_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_SetUnk1310", &PROBE_N, &[(p_chr_1) as u32, (p_v_2) as u32], false); }
    let r_ = 'f: {
    F::Vec4_Copy(c, (p_chr_1).wrapping_add(4880), p_v_2);
    F::BtlChar_SetFlag(c, p_chr_1, ((((85i32) as i64 as u64)) as u32));
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_SetUnk1310", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChar_ResetUnk1310(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_ResetUnk1310", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_obj_2: u32 = 0;
    let r_ = 'f: {
    l_obj_2 = F::BtlChar_GetObj(c, p_chr_1);
    { let a1_ = c.m.r32((l_obj_2).wrapping_add(4000)); F::Vec4_Copy(c, (p_chr_1).wrapping_add(4880), a1_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_ResetUnk1310", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChar_RequestPlaceRelative(c: &mut Ctx, mut p_chr_1: u32, mut p_arg1_2: i32, mut p_arg2_3: i32, mut p_self_4: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_RequestPlaceRelative", &PROBE_N, &[(p_chr_1) as u32, (p_arg1_2) as u32, (p_arg2_3) as u32, (p_self_4) as u32], false); }
    let mut l_target_5: u32 = 0;
    let r_ = 'f: {
    l_target_5 = p_chr_1;
    if ((((!((p_self_4) != 0)) as i32)) != 0) {
        l_target_5 = { let a0_ = F::BtlOpp_GetPlayer(c, p_chr_1); F::BtlChar_Get(c, a0_) };
    }
    F::BtlSuper_GetStagePlacement(c, l_target_5, p_arg1_2, p_arg2_3, (p_chr_1).wrapping_add(3696), (p_chr_1).wrapping_add(3712));
    F::BtlChar_SetFlag(c, p_chr_1, ((((246i32) as i64 as u64)) as u32));
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_RequestPlaceRelative", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChar_RequestPlaceOnPath(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_RequestPlaceOnPath", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    F::BtlChar_SetFlag(c, p_chr_1, ((((248i32) as i64 as u64)) as u32));
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_RequestPlaceOnPath", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChar_ClearSnapshots(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_ClearSnapshots", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = ((0i32) as u32); let a_ = (p_chr_1).wrapping_add(960); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_ClearSnapshots", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChar_Snapshot(c: &mut Ctx, mut p_chr_1: u32, mut p_pos_2: u32, mut p_rot_3: u32, mut p_n_4: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_Snapshot", &PROBE_N, &[(p_chr_1) as u32, (p_pos_2) as u32, (p_rot_3) as u32, (p_n_4) as u32], false); }
    let r_ = 'f: {
    F::Vec4_Copy(c, ((p_chr_1).wrapping_add(736)).wrapping_add((((p_n_4) as u32)).wrapping_mul(16)), p_pos_2);
    F::Vec4_Copy(c, ((p_chr_1).wrapping_add(848)).wrapping_add((((p_n_4) as u32)).wrapping_mul(16)), p_rot_3);
    { let r_ = (((1i32).wrapping_shl((p_n_4) as u32)) as u32); let a_ = (p_chr_1).wrapping_add(960); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
    F::Vec4_Copy(c, ((p_chr_1).wrapping_add(736)).wrapping_add((((6i32) as u32)).wrapping_mul(16)), p_pos_2);
    F::Vec4_Copy(c, ((p_chr_1).wrapping_add(848)).wrapping_add((((6i32) as u32)).wrapping_mul(16)), p_rot_3);
    { let r_ = (((1i32).wrapping_shl((6i32) as u32)) as u32); let a_ = (p_chr_1).wrapping_add(960); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_Snapshot", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChars_Snapshot(c: &mut Ctx, mut p_n_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChars_Snapshot", &PROBE_N, &[(p_n_1) as u32], false); }
    let mut l_i_2: i32 = 0;
    let mut l_chr_3: u32 = 0;
    let mut l_pose_4: u32 = 0;
    let r_ = 'f: {
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < (F::BtlChar_GetCount(c))) as i32)) != 0) { break 'l1; }
            'c2: {
                l_chr_3 = F::BtlChar_Get(c, l_i_2);
                l_pose_4 = F::BtlChar_GetPos(c, l_chr_3);
                F::BtlChar_Snapshot(c, l_chr_3, l_pose_4, (l_pose_4).wrapping_add(16), p_n_1);
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChars_Snapshot", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChar_GetSnapPos(c: &mut Ctx, mut p_chr_1: u32, mut p_out_2: u32, mut p_n_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_GetSnapPos", &PROBE_N, &[(p_chr_1) as u32, (p_out_2) as u32, (p_n_3) as u32], false); }
    let r_ = 'f: {
    if ((((c.m.r32((p_chr_1).wrapping_add(960))) & ((((1i32).wrapping_shl((p_n_3) as u32)) as u32)))) != 0) {
        F::Vec4_Copy(c, p_out_2, ((p_chr_1).wrapping_add(736)).wrapping_add((((p_n_3) as u32)).wrapping_mul(16)));
    } else {
        { let a1_ = F::BtlChar_GetPos(c, p_chr_1); F::Vec4_Copy(c, p_out_2, a1_) };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_GetSnapPos", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChar_GetSnapRot(c: &mut Ctx, mut p_chr_1: u32, mut p_out_2: u32, mut p_n_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_GetSnapRot", &PROBE_N, &[(p_chr_1) as u32, (p_out_2) as u32, (p_n_3) as u32], false); }
    let r_ = 'f: {
    if ((((c.m.r32((p_chr_1).wrapping_add(960))) & ((((1i32).wrapping_shl((p_n_3) as u32)) as u32)))) != 0) {
        F::Vec4_Copy(c, p_out_2, ((p_chr_1).wrapping_add(848)).wrapping_add((((p_n_3) as u32)).wrapping_mul(16)));
    } else {
        { let a1_ = (F::BtlChar_GetPos(c, p_chr_1)).wrapping_add(16); F::Vec4_Copy(c, p_out_2, a1_) };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_GetSnapRot", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChar_GetSnapDelta(c: &mut Ctx, mut p_chr_1: u32, mut p_out_2: u32, mut p_from_3: i32, mut p_to_4: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_GetSnapDelta", &PROBE_N, &[(p_chr_1) as u32, (p_out_2) as u32, (p_from_3) as u32, (p_to_4) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let r_ = 'f: {
        F::BtlChar_GetSnapPos(c, p_chr_1, fp, p_from_3);
        F::BtlChar_GetSnapPos(c, p_chr_1, fp.wrapping_add(16), p_to_4);
        F::Vec4_Sub(c, p_out_2, fp.wrapping_add(16), fp);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_GetSnapDelta", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChar_GetMoveSince(c: &mut Ctx, mut p_chr_1: u32, mut p_out_2: u32, mut p_n_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_GetMoveSince", &PROBE_N, &[(p_chr_1) as u32, (p_out_2) as u32, (p_n_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        F::BtlChar_GetSnapPos(c, p_chr_1, fp, p_n_3);
        { let a1_ = F::BtlChar_GetPos(c, p_chr_1); F::Vec4_Sub(c, p_out_2, a1_, fp) };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_GetMoveSince", &PROBE_N, &[], true); }
    r_
}
