#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn StgCol_SetModeAll(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgCol_SetModeAll", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let v_ = 0i32; let a_ = sym::gStgColMode; c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgCol_SetModeAll", &PROBE_N, &[], true); }
    r_
}

pub fn StgCol_SetModeObjects(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgCol_SetModeObjects", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let v_ = 1i32; let a_ = sym::gStgColMode; c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgCol_SetModeObjects", &PROBE_N, &[], true); }
    r_
}

pub fn StgCol_QueryZone(c: &mut Ctx, mut p_res_1: u32, mut p_zone_2: u32, mut p_box_3: u32, mut p_ctx_4: u32, mut p_cb_5: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgCol_QueryZone", &PROBE_N, &[(p_res_1) as u32, (p_zone_2) as u32, (p_box_3) as u32, (p_ctx_4) as u32, (p_cb_5) as u32], false); }
    let mut l_objs_6: u32 = 0;
    let mut l_recs_7: u32 = 0;
    let mut l_i_8: i32 = 0;
    let mut l_rec_9: u32 = 0;
    let r_ = 'f: {
    l_objs_6 = { let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(72); c.m.r32(a_) };
    l_recs_7 = c.m.r32((p_zone_2).wrapping_add(24));
    { let _ = F::memset(c, p_res_1, 0i32, 32u32); };
    if ((((((c.m.r32(sym::gStgColMode) as i32)) == (0i32)) as i32)) != 0) {
        { let v_ = c.m.r32((p_zone_2).wrapping_add(4)); let a_ = sym::gStgColMesh; c.m.w32(a_, v_); };
        if (({ let a0_ = c.m.r32(sym::gStgColMesh); F::ColMesh_WalkBox(c, a0_, p_box_3, p_ctx_4, p_cb_5) }) != 0) {
            { let v_ = 1i32; let a_ = p_res_1; c.m.w32(a_, (v_) as u32); };
            { let v_ = 1i32; let a_ = (p_res_1).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        }
    }
    {
        l_i_8 = 0i32;
        'l1: loop {
            if !(((((l_i_8) < ((c.m.r32((p_zone_2).wrapping_add(20)) as i32))) as i32)) != 0) { break 'l1; }
            'c2: {
                l_rec_9 = (l_recs_7).wrapping_add((((l_i_8) as u32)).wrapping_mul(16));
                if (((c.m.r32(l_rec_9) as i32)) != 0) {
                    if ((((!(((({ let a_ = (l_objs_6).wrapping_add(((((c.m.r32((l_rec_9).wrapping_add(4)) as i32)) as u32)).wrapping_mul(80)); (c.m.r32(a_) as i32) }) & (1i32))) != 0)) as i32)) != 0) {
                        break 'c2;
                    }
                    if ((((((c.m.r32(sym::gStgColMode) as i32)) == (1i32)) as i32)) != 0) {
                        break 'c2;
                    }
                } else {
                    if (((({ let a_ = (l_objs_6).wrapping_add(((((c.m.r32((l_rec_9).wrapping_add(4)) as i32)) as u32)).wrapping_mul(80)); (c.m.r32(a_) as i32) }) & (1i32))) != 0) {
                        break 'c2;
                    }
                }
                { let v_ = c.m.r32((l_rec_9).wrapping_add(8)); let a_ = sym::gStgColMesh; c.m.w32(a_, v_); };
                if (({ let a0_ = c.m.r32(sym::gStgColMesh); F::ColMesh_WalkBox(c, a0_, p_box_3, p_ctx_4, p_cb_5) }) != 0) {
                    { let r_ = (((1i64).wrapping_shl((l_i_8) as u32)) as u64); let a_ = (p_res_1).wrapping_add(16); let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
                    { let v_ = 1i32; let a_ = p_res_1; c.m.w32(a_, (v_) as u32); };
                    { let v_ = 1i32; let a_ = (p_res_1).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    if ((((!(((c.m.r32(l_rec_9) as i32)) != 0)) as i32)) != 0) {
                        { let v_ = 1i32; let a_ = (p_res_1).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                        { let r_ = (((1i64).wrapping_shl(((c.m.r32((l_rec_9).wrapping_add(4)) as i32)) as u32)) as u64); let a_ = (p_res_1).wrapping_add(24); let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
                    }
                }
            }
            { let o_ = l_i_8; let n_ = (o_).wrapping_add(1); l_i_8 = n_; };
        }
    }
    break 'f (c.m.r32(p_res_1) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "StgCol_QueryZone", &PROBE_N, &[], true); }
    r_
}

pub fn StgCol_FirstBit(c: &mut Ctx, mut p_mask_1: u64) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgCol_FirstBit", &PROBE_N, &[(p_mask_1) as u32], false); }
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < (64i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if (((((p_mask_1).wrapping_shr((l_i_2) as u32)) & (((1i32) as u64)))) != 0) {
                    break 'f l_i_2;
                }
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
    break 'f (1i32).wrapping_neg();
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "StgCol_FirstBit", &PROBE_N, &[], true); }
    r_
}

pub fn StgCol_SweepCb(c: &mut Ctx, mut p_node_1: u32, mut p_ctx_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgCol_SweepCb", &PROBE_N, &[(p_node_1) as u32, (p_ctx_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(96) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 96);
    let mut l_poly_3: u32 = 0;
    let mut l_mesh_4: u32 = 0;
    let mut l_idx_5: i32 = 0;
    let r_ = 'f: {
        { let _ = F::memset(c, fp.wrapping_add(64), 0i32, 16u32); };
        l_idx_5 = (c.m.r32((p_node_1).wrapping_add(24)) as i32);
        l_mesh_4 = c.m.r32(sym::gStgColMesh);
        { let v_ = F32(0x3f800000); let a_ = (fp.wrapping_add(64)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        l_poly_3 = F::ColMesh_GetPoly(c, l_mesh_4, l_idx_5);
        if (((((c.m.r32(l_poly_3) as i32)) & (((((((134217728i32) | (67108864i32))) | (64i32))) | (32i32))))) != 0) {
            break 'f 0i32;
        }
        { let a0_ = c.m.r32(sym::gStgColMesh); F::ColMesh_GetPolyVerts(c, a0_, l_poly_3, (fp).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (fp).wrapping_add((((1i32) as u32)).wrapping_mul(16)), (fp).wrapping_add((((2i32) as u32)).wrapping_mul(16))) };
        F::Vec4_Copy(c, (fp).wrapping_add(48), (l_poly_3).wrapping_add(16));
        if ((F::ColSweep_TestTri(c, p_ctx_2, fp, fp.wrapping_add(64), fp.wrapping_add(80))) != 0) {
            if (((rt::flt(F32(c.m.r32(fp.wrapping_add(80))), F32(c.m.r32((p_ctx_2).wrapping_add(128)))) as i32)) != 0) {
                { let v_ = F32(c.m.r32(fp.wrapping_add(80))); let a_ = (p_ctx_2).wrapping_add(128); c.m.w32(a_, (v_).0); };
                { let v_ = 1i32; let a_ = (p_ctx_2).wrapping_add(224); c.m.w32(a_, (v_) as u32); };
                F::Vec4_Copy(c, (p_ctx_2).wrapping_add(144), fp.wrapping_add(64));
                { let s_ = fp; let d_ = (p_ctx_2).wrapping_add(160); rt::memcpy(c, d_, s_, 64); };
                break 'f 1i32;
            }
        }
        break 'f 0i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "StgCol_SweepCb", &PROBE_N, &[], true); }
    r_
}

pub fn StgCol_TraceSphere(c: &mut Ctx, mut p_zoneIdx_1: i32, mut p_seg_2: u32, mut p_hitPos_3: u32, mut p_frac_4: u32, mut p_tri_5: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgCol_TraceSphere", &PROBE_N, &[(p_zoneIdx_1) as u32, (p_seg_2) as u32, (p_hitPos_3) as u32, (p_frac_4) as u32, (p_tri_5) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(352) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 352);
    let mut l_zone_6: u32 = 0;
    let r_ = 'f: {
        l_zone_6 = F::BtlStage_GetZone(c, p_zoneIdx_1);
        F::ColBounds_OfCapsule(c, fp, p_seg_2);
        { let _ = F::memset(c, fp.wrapping_add(32), 0i32, 240u32); };
        { let s_ = p_seg_2; let d_ = fp.wrapping_add(32); rt::memcpy(c, d_, s_, 48); };
        F::ColCapsule_GetLongSegDir(c, fp.wrapping_add(320), fp.wrapping_add(304), fp.wrapping_add(32));
        { let r_ = rt::fmul(rt::fmul(F32(c.m.r32(fp.wrapping_add(304))), F32(c.m.r32((p_seg_2).wrapping_add(32)))), F32(0x40000000)); let a_ = fp.wrapping_add(32); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        { let r_ = rt::fmul(rt::fmul(F32(c.m.r32((fp.wrapping_add(304)).wrapping_add(4))), F32(c.m.r32((p_seg_2).wrapping_add(32)))), F32(0x40000000)); let a_ = (fp.wrapping_add(32)).wrapping_add(4); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        { let r_ = rt::fmul(rt::fmul(F32(c.m.r32((fp.wrapping_add(304)).wrapping_add(8))), F32(c.m.r32((p_seg_2).wrapping_add(32)))), F32(0x40000000)); let a_ = (fp.wrapping_add(32)).wrapping_add(8); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        F::Vec4_Copy(c, (fp.wrapping_add(32)).wrapping_add(48), fp.wrapping_add(32));
        { let v_ = F32(c.m.r32((fp.wrapping_add(32)).wrapping_add(32))); let a_ = ((fp.wrapping_add(32)).wrapping_add(48)).wrapping_add(16); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(F32(c.m.r32((fp.wrapping_add(32)).wrapping_add(32))), F32(c.m.r32((fp.wrapping_add(32)).wrapping_add(32)))); let a_ = ((fp.wrapping_add(32)).wrapping_add(48)).wrapping_add(20); c.m.w32(a_, (v_).0); };
        F::Vec3_Sub(c, (fp.wrapping_add(32)).wrapping_add(80), (fp.wrapping_add(32)).wrapping_add(16), fp.wrapping_add(32));
        { let v_ = { let a0_ = F::Vec3_Dot(c, (fp.wrapping_add(32)).wrapping_add(80), (fp.wrapping_add(32)).wrapping_add(80)); F::Port_sqrtf(c, a0_) }; let a_ = (fp.wrapping_add(32)).wrapping_add(112); c.m.w32(a_, (v_).0); };
        if (((rt::fgt(F32(c.m.r32((fp.wrapping_add(32)).wrapping_add(112))), F32(0x00000000)) as i32)) != 0) {
            { let a2_ = rt::fdiv(F32(0x3f800000), F32(c.m.r32((fp.wrapping_add(32)).wrapping_add(112)))); F::Vec3_Scale(c, (fp.wrapping_add(32)).wrapping_add(96), (fp.wrapping_add(32)).wrapping_add(80), a2_) };
        } else {
            F::Vec4_Copy(c, (fp.wrapping_add(32)).wrapping_add(96), sym::D_002EC2C0);
        }
        { let v_ = F32(c.m.r32((fp.wrapping_add(32)).wrapping_add(112))); let a_ = (fp.wrapping_add(32)).wrapping_add(128); c.m.w32(a_, (v_).0); };
        F::StgCol_SetModeAll(c);
        if ((F::StgCol_QueryZone(c, fp.wrapping_add(272), l_zone_6, fp, fp.wrapping_add(32), fid::StgCol_SweepCb)) != 0) {
            if (((((p_frac_4) != (((0i32) as u32))) as i32)) != 0) {
                if (((rt::fgt(F32(c.m.r32((fp.wrapping_add(32)).wrapping_add(112))), F32(0x00000000)) as i32)) != 0) {
                    { let v_ = rt::fdiv(F32(c.m.r32((fp.wrapping_add(32)).wrapping_add(128))), F32(c.m.r32((fp.wrapping_add(32)).wrapping_add(112)))); let a_ = p_frac_4; c.m.w32(a_, (v_).0); };
                } else {
                    { let v_ = F32(0x00000000); let a_ = p_frac_4; c.m.w32(a_, (v_).0); };
                }
            }
            if (((((p_tri_5) != (((0i32) as u32))) as i32)) != 0) {
                { let s_ = (fp.wrapping_add(32)).wrapping_add(160); let d_ = p_tri_5; rt::memcpy(c, d_, s_, 64); };
            }
            F::Vec4_Copy(c, p_hitPos_3, (fp.wrapping_add(32)).wrapping_add(144));
            break 'f 1i32;
        }
        break 'f 0i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "StgCol_TraceSphere", &PROBE_N, &[], true); }
    r_
}

pub fn StgCol_SplitStep(c: &mut Ctx, mut p_out_1: u32, mut p_delta_2: u32, mut p_radius_3: F32, mut p_dist_4: F32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgCol_SplitStep", &PROBE_N, &[(p_out_1) as u32, (p_delta_2) as u32, (p_radius_3).0, (p_dist_4).0], false); }
    let mut l_n_5: i32 = 0;
    let mut l_scale_6: F32 = F32(0);
    let r_ = 'f: {
    l_n_5 = rt::f2i(rt::fdiv(p_dist_4, rt::fmul(p_radius_3, F32(0x3f000000))));
    l_scale_6 = F32(0x3f800000);
    'l1: loop {
        'c2: {
            if (((((l_n_5) == (0i32)) as i32)) != 0) {
                F::Vec4_Copy(c, p_out_1, p_delta_2);
                break 'f 1i32;
            }
            { let o_ = l_n_5; let n_ = (o_).wrapping_add(1); l_n_5 = n_; };
            { let r_ = rt::i2f(l_n_5); let n_ = rt::fdiv(l_scale_6, r_); l_scale_6 = n_; };
            F::Vec3_Scale(c, p_out_1, p_delta_2, l_scale_6);
        }
        if !((0i32) != 0) { break 'l1; }
    }
    break 'f l_n_5;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "StgCol_SplitStep", &PROBE_N, &[], true); }
    r_
}

pub fn StgCol_StepSphere(c: &mut Ctx, mut p_sphere_1: u32, mut p_box_2: u32, mut p_step_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgCol_StepSphere", &PROBE_N, &[(p_sphere_1) as u32, (p_box_2) as u32, (p_step_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        F::Vec4_Add(c, p_sphere_1, p_sphere_1, p_step_3);
        { let v_ = { let v_ = { let v_ = rt::fadd(F32(c.m.r32((p_sphere_1).wrapping_add(16))), F32(0x3dcccccc)); let a_ = fp; c.m.w32(a_, (v_).0); v_ }; let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); v_ }; let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_).0); };
        F::ColBox_SetCenterHalf(c, p_box_2, p_sphere_1, fp);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "StgCol_StepSphere", &PROBE_N, &[], true); }
    r_
}

pub fn StgCol_PushSphere(c: &mut Ctx, mut p_sphere_1: u32, mut p_box_2: u32, mut p_point_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgCol_PushSphere", &PROBE_N, &[(p_sphere_1) as u32, (p_box_2) as u32, (p_point_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let mut l_len_4: F32 = F32(0);
    let r_ = 'f: {
        F::Vec3_Sub(c, fp, p_sphere_1, p_point_3);
        l_len_4 = { let a0_ = F::Vec3_Dot(c, fp, fp); F::Port_sqrtf(c, a0_) };
        { let a2_ = rt::fdiv(rt::fsub(F32(c.m.r32((p_sphere_1).wrapping_add(16))), l_len_4), l_len_4); F::Vec3_Scale(c, fp.wrapping_add(16), fp, a2_) };
        F::Vec4_Add(c, p_sphere_1, p_sphere_1, fp.wrapping_add(16));
        { let v_ = { let v_ = { let v_ = rt::fadd(F32(c.m.r32((p_sphere_1).wrapping_add(16))), F32(0x3f8ccccc)); let a_ = fp.wrapping_add(32); c.m.w32(a_, (v_).0); v_ }; let a_ = (fp.wrapping_add(32)).wrapping_add(4); c.m.w32(a_, (v_).0); v_ }; let a_ = (fp.wrapping_add(32)).wrapping_add(8); c.m.w32(a_, (v_).0); };
        F::ColBox_SetCenterHalf(c, p_box_2, p_sphere_1, fp.wrapping_add(32));
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "StgCol_PushSphere", &PROBE_N, &[], true); }
    r_
}

pub fn StgCol_PushCb(c: &mut Ctx, mut p_node_1: u32, mut p_ctx_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgCol_PushCb", &PROBE_N, &[(p_node_1) as u32, (p_ctx_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(80) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 80);
    let mut l_poly_3: u32 = 0;
    let mut l_mesh_4: u32 = 0;
    let mut l_idx_5: i32 = 0;
    let r_ = 'f: {
        { let _ = F::memset(c, fp.wrapping_add(64), 0i32, 16u32); };
        l_idx_5 = (c.m.r32((p_node_1).wrapping_add(24)) as i32);
        l_mesh_4 = c.m.r32(sym::gStgColMesh);
        { let v_ = F32(0x3f800000); let a_ = (fp.wrapping_add(64)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        l_poly_3 = F::ColMesh_GetPoly(c, l_mesh_4, l_idx_5);
        if (((((c.m.r32(l_poly_3) as i32)) & (134217728i32))) != 0) {
            break 'f 0i32;
        }
        { let a0_ = c.m.r32(sym::gStgColMesh); F::ColMesh_GetPolyVerts(c, a0_, l_poly_3, (fp).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (fp).wrapping_add((((1i32) as u32)).wrapping_mul(16)), (fp).wrapping_add((((2i32) as u32)).wrapping_mul(16))) };
        F::Vec4_Copy(c, (fp).wrapping_add(48), (l_poly_3).wrapping_add(16));
        if ((F::ColSphere_TestTri(c, fp.wrapping_add(64), (p_ctx_2).wrapping_add(16), fp)) != 0) {
            F::StgCol_PushSphere(c, (p_ctx_2).wrapping_add(16), (p_ctx_2).wrapping_add(48), fp.wrapping_add(64));
            { let s_ = l_poly_3; let d_ = (p_ctx_2).wrapping_add(80); rt::memcpy(c, d_, s_, 32); };
            { let v_ = 1i32; let a_ = p_ctx_2; c.m.w32(a_, (v_) as u32); };
            break 'f 1i32;
        }
        break 'f 0i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "StgCol_PushCb", &PROBE_N, &[], true); }
    r_
}

pub fn StgCol_MoveFighter(c: &mut Ctx, mut p_obj_1: u32, mut p_ctx_2: u32, mut p_zoneIdx_3: i32, mut p_steps_4: i32, mut p_step_5: u32, mut p_delta_6: u32, mut p_crossZones_7: i32, mut p_keepSphere_8: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgCol_MoveFighter", &PROBE_N, &[(p_obj_1) as u32, (p_ctx_2) as u32, (p_zoneIdx_3) as u32, (p_steps_4) as u32, (p_step_5) as u32, (p_delta_6) as u32, (p_crossZones_7) as u32, (p_keepSphere_8) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_pose_9: u32 = 0;
    let mut l_body_10: u32 = 0;
    let mut l_work_11: u32 = 0;
    let mut l_sphere_12: u32 = 0;
    let mut l_zone_13: u32 = 0;
    let mut l_cur_14: i32 = 0;
    let r_ = 'f: {
        l_pose_9 = (p_obj_1).wrapping_add(2384);
        l_body_10 = (p_obj_1).wrapping_add(3920);
        l_work_11 = c.m.r32((p_obj_1).wrapping_add(5728));
        l_sphere_12 = c.m.r32((l_body_10).wrapping_add(80));
        if ((((!((p_crossZones_7) != 0)) as i32)) != 0) {
            l_zone_13 = F::BtlStage_GetZone(c, p_zoneIdx_3);
            l_cur_14 = p_zoneIdx_3;
        } else {
            l_cur_14 = p_zoneIdx_3;
            l_zone_13 = ((0i32) as u32);
        }
        'l1: loop {
            'c2: {
                F::StgCol_StepSphere(c, (p_ctx_2).wrapping_add(16), (p_ctx_2).wrapping_add(48), p_step_5);
                if ((p_crossZones_7) != 0) {
                    l_cur_14 = F::BtlStage_FindZoneNear(c, l_cur_14, (p_ctx_2).wrapping_add(16));
                    l_zone_13 = F::BtlStage_GetZone(c, l_cur_14);
                }
                F::StgCol_SetModeAll(c);
                if ((F::StgCol_QueryZone(c, fp, l_zone_13, (p_ctx_2).wrapping_add(48), p_ctx_2, fid::StgCol_PushCb)) != 0) {
                    if (((c.m.r32((fp).wrapping_add(12)) as i32)) != 0) {
                        { let a1_ = { let a0_ = c.m.r64((fp).wrapping_add(24)); F::StgCol_FirstBit(c, a0_) }; F::StgCol_FighterBreakObj(c, p_obj_1, a1_, p_delta_6) };
                    }
                }
            }
            if !((((({ let o_ = p_steps_4; let n_ = (o_).wrapping_sub(1); p_steps_4 = n_; n_ }) > (0i32)) as i32)) != 0) { break 'l1; }
        }
        if (((c.m.r32(p_ctx_2) as i32)) != 0) {
            { let r_ = 32i32; let a_ = (l_work_11).wrapping_add(98336); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            { let s_ = (p_ctx_2).wrapping_add(80); let d_ = (l_work_11).wrapping_add(98400); rt::memcpy(c, d_, s_, 32); };
            F::Vec3_Sub(c, (l_body_10).wrapping_add(96), l_sphere_12, (p_ctx_2).wrapping_add(16));
            F::Vec3_Sub(c, l_pose_9, l_pose_9, (l_body_10).wrapping_add(96));
            { let v_ = F32(0x3f800000); let a_ = (l_pose_9).wrapping_add(12); c.m.w32(a_, (v_).0); };
            F::BtlObjXf_Update(c, p_obj_1);
            F::BtlObjPose_CalcMatrices(c, p_obj_1);
            if ((p_keepSphere_8) != 0) {
                { let _ = F::BtlObjBody_Update(c, p_obj_1, 0i32); };
                break 'f 1i32;
            }
            F::Vec4_Copy(c, l_sphere_12, (p_ctx_2).wrapping_add(16));
            break 'f 1i32;
        }
        F::Vec4_Copy(c, (l_body_10).wrapping_add(96), sym::D_002EC2C0);
        break 'f 0i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "StgCol_MoveFighter", &PROBE_N, &[], true); }
    r_
}

pub fn StgCol_UpdateFighter(c: &mut Ctx, mut p_obj_1: u32, mut p_keepSphere_2: i32, mut p_grow_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgCol_UpdateFighter", &PROBE_N, &[(p_obj_1) as u32, (p_keepSphere_2) as u32, (p_grow_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(256) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 256);
    let mut l_body_4: u32 = 0;
    let mut l_pose_5: u32 = 0;
    let mut l_work_6: u32 = 0;
    let mut l_cur_7: u32 = 0;
    let mut l_prev_8: u32 = 0;
    let mut l_zone_9: i32 = 0;
    let mut l_steps_10: i32 = 0;
    let mut l_dist_11: F32 = F32(0);
    let mut l_r_12: F32 = F32(0);
    let r_ = 'f: {
        l_body_4 = (p_obj_1).wrapping_add(3920);
        l_pose_5 = (p_obj_1).wrapping_add(2384);
        l_work_6 = c.m.r32((p_obj_1).wrapping_add(5728));
        if (((((l_work_6) == (((0i32) as u32))) as i32)) != 0) {
            break 'f;
        }
        F::BtlObjBody_BeginFrame(c, p_obj_1);
        l_cur_7 = c.m.r32((l_body_4).wrapping_add(80));
        l_prev_8 = c.m.r32((l_body_4).wrapping_add(84));
        { let r_ = (!(32i32)); let a_ = (l_work_6).wrapping_add(98336); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
        { let r_ = (!(64i32)); let a_ = (l_work_6).wrapping_add(98336); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
        if ((p_keepSphere_2) != 0) {
            { let _ = F::BtlObjBody_Update(c, p_obj_1, 0i32); };
        } else {
            l_r_12 = F32(c.m.r32((l_body_4).wrapping_add(180)));
            { let v_ = l_r_12; let a_ = (l_cur_7).wrapping_add(16); c.m.w32(a_, (v_).0); };
            { let v_ = rt::fmul(l_r_12, l_r_12); let a_ = (l_cur_7).wrapping_add(20); c.m.w32(a_, (v_).0); };
        }
        if ((p_grow_3) != 0) {
            { let r_ = F32(0x3f800000); let a_ = (l_body_4).wrapping_add(184); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            if (((rt::flt(F32(c.m.r32((l_body_4).wrapping_add(180))), F32(c.m.r32((l_body_4).wrapping_add(184)))) as i32)) != 0) {
                { let v_ = F32(c.m.r32((l_body_4).wrapping_add(180))); let a_ = (l_body_4).wrapping_add(184); c.m.w32(a_, (v_).0); };
            }
        }
        if (((rt::flt(F32(c.m.r32((l_body_4).wrapping_add(184))), F32(0x40800000)) as i32)) != 0) {
            { let v_ = F32(0x40800000); let a_ = (l_body_4).wrapping_add(184); c.m.w32(a_, (v_).0); };
        }
        if (((((l_prev_8) != (((0i32) as u32))) as i32)) != 0) {
            { let v_ = F32(c.m.r32((l_body_4).wrapping_add(184))); let a_ = (l_prev_8).wrapping_add(16); c.m.w32(a_, (v_).0); };
            { let v_ = rt::fmul(F32(c.m.r32((l_body_4).wrapping_add(184))), F32(c.m.r32((l_body_4).wrapping_add(184)))); let a_ = (l_prev_8).wrapping_add(20); c.m.w32(a_, (v_).0); };
        }
        if (((((l_cur_7) != (((0i32) as u32))) as i32)) != 0) {
            { let v_ = F32(c.m.r32((l_body_4).wrapping_add(184))); let a_ = (l_cur_7).wrapping_add(16); c.m.w32(a_, (v_).0); };
            { let v_ = rt::fmul(F32(c.m.r32((l_body_4).wrapping_add(184))), F32(c.m.r32((l_body_4).wrapping_add(184)))); let a_ = (l_cur_7).wrapping_add(20); c.m.w32(a_, (v_).0); };
        }
        l_zone_9 = { let a0_ = (c.m.r32((l_pose_5).wrapping_add(212)) as i32); F::BtlStage_FindZoneNear(c, a0_, l_cur_7) };
        { let _ = F::memset(c, fp.wrapping_add(32), 0i32, 112u32); };
        if ((((!(((c.m.r32((l_body_4).wrapping_add(188)) as i32)) != 0)) as i32)) != 0) {
            { let s_ = l_prev_8; let d_ = (fp.wrapping_add(32)).wrapping_add(16); rt::memcpy(c, d_, s_, 32); };
            F::Vec3_Sub(c, fp.wrapping_add(16), l_cur_7, l_prev_8);
            l_dist_11 = { let a0_ = F::Vec3_Dot(c, fp.wrapping_add(16), fp.wrapping_add(16)); F::Port_sqrtf(c, a0_) };
        } else {
            { let s_ = l_cur_7; let d_ = (fp.wrapping_add(32)).wrapping_add(16); rt::memcpy(c, d_, s_, 32); };
            F::Vec4_Copy(c, fp.wrapping_add(16), sym::D_002EC2C0);
            { let v_ = 0i32; let a_ = (l_body_4).wrapping_add(188); c.m.w32(a_, (v_) as u32); };
            l_dist_11 = F32(0x00000000);
        }
        l_steps_10 = { let a2_ = F32(c.m.r32(((fp.wrapping_add(32)).wrapping_add(16)).wrapping_add(16))); F::StgCol_SplitStep(c, fp, fp.wrapping_add(16), a2_, l_dist_11) };
        { let s_ = fp.wrapping_add(32); let d_ = fp.wrapping_add(144); rt::memcpy(c, d_, s_, 112); };
        if (((((l_prev_8) != (((0i32) as u32))) as i32)) != 0) {
            if (((((l_zone_9) != ((c.m.r32((l_pose_5).wrapping_add(212)) as i32))) as i32)) != 0) {
                { let _ = { let a2_ = (c.m.r32((l_pose_5).wrapping_add(212)) as i32); F::StgCol_MoveFighter(c, p_obj_1, fp.wrapping_add(32), a2_, l_steps_10, fp, fp.wrapping_add(16), 1i32, p_keepSphere_2) }; };
            } else {
                { let _ = { let a2_ = (c.m.r32((l_pose_5).wrapping_add(212)) as i32); F::StgCol_MoveFighter(c, p_obj_1, fp.wrapping_add(32), a2_, l_steps_10, fp, fp.wrapping_add(16), 0i32, p_keepSphere_2) }; };
            }
        } else {
            { let _ = F::StgCol_MoveFighter(c, p_obj_1, fp.wrapping_add(32), l_zone_9, l_steps_10, fp, fp.wrapping_add(16), 0i32, p_keepSphere_2); };
        }
        { let v_ = l_zone_9; let a_ = (l_pose_5).wrapping_add(212); c.m.w32(a_, (v_) as u32); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "StgCol_UpdateFighter", &PROBE_N, &[], true); }
    r_
}

pub fn StgCol_FighterBreakObj(c: &mut Ctx, mut p_obj_1: u32, mut p_idx_2: i32, mut p_hitPos_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgCol_FighterBreakObj", &PROBE_N, &[(p_obj_1) as u32, (p_idx_2) as u32, (p_hitPos_3) as u32], false); }
    let mut l_work_4: u32 = 0;
    let mut l_type_5: i32 = 0;
    let r_ = 'f: {
    l_work_4 = c.m.r32((p_obj_1).wrapping_add(5728));
    if (((((p_idx_2) < (0i32)) as i32)) != 0) {
        break 'f;
    }
    if (((((l_work_4) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    l_type_5 = F::BtlStage_GetObjType(c, p_idx_2);
    if (({ let a0_ = (c.m.r32((p_obj_1).wrapping_add(16)) as i32); F::BtlCharApi_TestStageBreakA(c, a0_) }) != 0) {
        if ((((l_type_5) & (256i32))) != 0) {
            { let _ = { let a0_ = (c.m.r32((p_obj_1).wrapping_add(16)) as i32); F::BtlStage_DestroyObj(c, a0_, p_idx_2, p_hitPos_3) }; };
            { let r_ = 64i32; let a_ = (l_work_4).wrapping_add(98336); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        }
    } else {
        if (({ let a0_ = (c.m.r32((p_obj_1).wrapping_add(16)) as i32); F::BtlCharApi_TestStageBreakB(c, a0_) }) != 0) {
            { let _ = { let a0_ = (c.m.r32((p_obj_1).wrapping_add(16)) as i32); F::BtlStage_DestroyObj(c, a0_, p_idx_2, p_hitPos_3) }; };
            { let r_ = 64i32; let a_ = (l_work_4).wrapping_add(98336); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        }
    }
    if (((((c.m.r32((l_work_4).wrapping_add(98336)) as i32)) & (64i32))) != 0) {
        if ((((l_type_5) & (2048i32))) != 0) {
            { let r_ = 128i32; let a_ = (l_work_4).wrapping_add(98336); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        }
        if ((((l_type_5) & (8192i32))) != 0) {
            { let r_ = 256i32; let a_ = (l_work_4).wrapping_add(98336); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        }
        if ((((l_type_5) & (4096i32))) != 0) {
            { let r_ = 512i32; let a_ = (l_work_4).wrapping_add(98336); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgCol_FighterBreakObj", &PROBE_N, &[], true); }
    r_
}

pub fn StgCol_SegCb(c: &mut Ctx, mut p_node_1: u32, mut p_ctx_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgCol_SegCb", &PROBE_N, &[(p_node_1) as u32, (p_ctx_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(96) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 96);
    let mut l_poly_3: u32 = 0;
    let r_ = 'f: {
        l_poly_3 = { let a0_ = c.m.r32(sym::gStgColMesh); let a1_ = (c.m.r32((p_node_1).wrapping_add(24)) as i32); F::ColMesh_GetPoly(c, a0_, a1_) };
        if (((((c.m.r32(l_poly_3) as i32)) & (134217728i32))) != 0) {
            break 'f 0i32;
        }
        { let a0_ = c.m.r32(sym::gStgColMesh); F::ColMesh_GetPolyVerts(c, a0_, l_poly_3, (fp).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (fp).wrapping_add((((1i32) as u32)).wrapping_mul(16)), (fp).wrapping_add((((2i32) as u32)).wrapping_mul(16))) };
        F::Vec4_Copy(c, (fp).wrapping_add(48), (l_poly_3).wrapping_add(16));
        if ((F::ColSeg_TestTri(c, (p_ctx_2).wrapping_add(32), fp, fp.wrapping_add(64), fp.wrapping_add(80))) != 0) {
            if (((rt::fgt(F32(c.m.r32((p_ctx_2).wrapping_add(128))), F32(c.m.r32(fp.wrapping_add(80)))) as i32)) != 0) {
                { let s_ = l_poly_3; let d_ = (p_ctx_2).wrapping_add(160); rt::memcpy(c, d_, s_, 32); };
                F::Vec4_Copy(c, (p_ctx_2).wrapping_add(144), fp.wrapping_add(64));
                { let v_ = 1i32; let a_ = (p_ctx_2).wrapping_add(200); c.m.w32(a_, (v_) as u32); };
                { let v_ = F32(c.m.r32(fp.wrapping_add(80))); let a_ = (p_ctx_2).wrapping_add(128); c.m.w32(a_, (v_).0); };
                break 'f 1i32;
            }
        }
        break 'f 0i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "StgCol_SegCb", &PROBE_N, &[], true); }
    r_
}

pub fn StgCol_CollectCb(c: &mut Ctx, mut p_node_1: u32, mut p_ctx_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgCol_CollectCb", &PROBE_N, &[(p_node_1) as u32, (p_ctx_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(80) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 80);
    let mut l_poly_3: u32 = 0;
    let r_ = 'f: {
        l_poly_3 = { let a0_ = c.m.r32(sym::gStgColMesh); let a1_ = (c.m.r32((p_node_1).wrapping_add(24)) as i32); F::ColMesh_GetPoly(c, a0_, a1_) };
        if (((((c.m.r32(l_poly_3) as i32)) & (134217728i32))) != 0) {
            break 'f 0i32;
        }
        { let a0_ = c.m.r32(sym::gStgColMesh); F::ColMesh_GetPolyVerts(c, a0_, l_poly_3, (fp).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (fp).wrapping_add((((1i32) as u32)).wrapping_mul(16)), (fp).wrapping_add((((2i32) as u32)).wrapping_mul(16))) };
        F::Vec4_Copy(c, (fp).wrapping_add(48), (l_poly_3).wrapping_add(16));
        if ((F::ColSphere_TestTri(c, fp.wrapping_add(64), p_ctx_2, fp)) != 0) {
            if (((((c.m.r32((p_ctx_2).wrapping_add(192))) != (((0i32) as u32))) as i32)) != 0) {
                if ((((({ let a_ = c.m.r32((p_ctx_2).wrapping_add(192)); c.m.r32(a_) }) < (((64i32) as u32))) as i32)) != 0) {
                    { let s_ = fp; let d_ = ((c.m.r32((p_ctx_2).wrapping_add(192))).wrapping_add(({ let a_ = c.m.r32((p_ctx_2).wrapping_add(192)); c.m.r32(a_) }).wrapping_mul(64))).wrapping_add(((16i32) as u32)); rt::memcpy(c, d_, s_, 64); };
                    { let a_ = c.m.r32((p_ctx_2).wrapping_add(192)); let o_ = c.m.r32(a_); let n_ = (o_).wrapping_add(1); c.m.w32(a_, n_); };
                }
            }
            { let v_ = 1i32; let a_ = (p_ctx_2).wrapping_add(200); c.m.w32(a_, (v_) as u32); };
            break 'f 1i32;
        }
        break 'f 0i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "StgCol_CollectCb", &PROBE_N, &[], true); }
    r_
}

pub fn StgCol_InitSegCtx(c: &mut Ctx, mut p_ctx_1: u32, mut p_seg_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgCol_InitSegCtx", &PROBE_N, &[(p_ctx_1) as u32, (p_seg_2) as u32], false); }
    let mut l_far_3: F32 = F32(0);
    let r_ = 'f: {
    { let _ = F::memset(c, p_ctx_1, 0i32, 208u32); };
    { let s_ = p_seg_2; let d_ = (p_ctx_1).wrapping_add(32); rt::memcpy(c, d_, s_, 32); };
    F::ColBounds_OfSeg(c, (p_ctx_1).wrapping_add(64), p_seg_2);
    { let v_ = F::Vec3_Dist(c, p_seg_2, (p_seg_2).wrapping_add(16)); let a_ = (p_ctx_1).wrapping_add(128); c.m.w32(a_, (v_).0); };
    F::ColRay_FromSeg(c, (p_ctx_1).wrapping_add(96), p_seg_2);
    l_far_3 = F32(c.m.r32((sym::gStgColFar).wrapping_add((((0i32) as u32)).wrapping_mul(4))));
    { let v_ = (1i32).wrapping_neg(); let a_ = (p_ctx_1).wrapping_add(196); c.m.w32(a_, (v_) as u32); };
    { let v_ = l_far_3; let a_ = (p_ctx_1).wrapping_add(128); c.m.w32(a_, (v_).0); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgCol_InitSegCtx", &PROBE_N, &[], true); }
    r_
}

pub fn StgCol_InitSphereCtx(c: &mut Ctx, mut p_ctx_1: u32, mut p_sphere_2: u32, mut p_list_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgCol_InitSphereCtx", &PROBE_N, &[(p_ctx_1) as u32, (p_sphere_2) as u32, (p_list_3) as u32], false); }
    let r_ = 'f: {
    { let _ = F::memset(c, p_ctx_1, 0i32, 208u32); };
    { let s_ = p_sphere_2; let d_ = p_ctx_1; rt::memcpy(c, d_, s_, 32); };
    F::ColBounds_OfSphere(c, (p_ctx_1).wrapping_add(64), p_sphere_2);
    { let v_ = p_list_3; let a_ = (p_ctx_1).wrapping_add(192); c.m.w32(a_, v_); };
    if (((((p_list_3) != (((0i32) as u32))) as i32)) != 0) {
        { let v_ = ((0i32) as u32); let a_ = p_list_3; c.m.w32(a_, v_); };
    }
    { let v_ = (1i32).wrapping_neg(); let a_ = (p_ctx_1).wrapping_add(196); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgCol_InitSphereCtx", &PROBE_N, &[], true); }
    r_
}

pub fn StgCol_GetZoneBox(c: &mut Ctx, mut p_box_1: u32, mut p_zoneIdx_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgCol_GetZoneBox", &PROBE_N, &[(p_box_1) as u32, (p_zoneIdx_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_zones_3: u32 = 0;
    let r_ = 'f: {
        l_zones_3 = { let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(60); c.m.r32(a_) };
        rt::memset(c, fp, 0, 16);
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(0), (v_).0); }
        { let v_ = F32(0xc61c4000); c.m.w32(fp.wrapping_add(4), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(8), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(12), (v_).0); }
        rt::memset(c, fp.wrapping_add(16), 0, 16);
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x461c4000); c.m.w32(fp.wrapping_add(16).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(16).wrapping_add(12), (v_).0); }
        F::StgAabb_SetEmpty(c, p_box_1);
        { let v_ = rt::fsub(F32(c.m.r32(((l_zones_3).wrapping_add((((p_zoneIdx_2) as u32)).wrapping_mul(64))).wrapping_add(32))), F32(c.m.r32(((l_zones_3).wrapping_add((((p_zoneIdx_2) as u32)).wrapping_mul(64))).wrapping_add(48)))); let a_ = fp; c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32((((l_zones_3).wrapping_add((((p_zoneIdx_2) as u32)).wrapping_mul(64))).wrapping_add(32)).wrapping_add(8))), F32(c.m.r32((((l_zones_3).wrapping_add((((p_zoneIdx_2) as u32)).wrapping_mul(64))).wrapping_add(48)).wrapping_add(8)))); let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32(((l_zones_3).wrapping_add((((p_zoneIdx_2) as u32)).wrapping_mul(64))).wrapping_add(32))), F32(c.m.r32(((l_zones_3).wrapping_add((((p_zoneIdx_2) as u32)).wrapping_mul(64))).wrapping_add(48)))); let a_ = fp.wrapping_add(16); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32((((l_zones_3).wrapping_add((((p_zoneIdx_2) as u32)).wrapping_mul(64))).wrapping_add(32)).wrapping_add(8))), F32(c.m.r32((((l_zones_3).wrapping_add((((p_zoneIdx_2) as u32)).wrapping_mul(64))).wrapping_add(48)).wrapping_add(8)))); let a_ = (fp.wrapping_add(16)).wrapping_add(8); c.m.w32(a_, (v_).0); };
        F::ColBox_AddPointKeepW(c, p_box_1, fp);
        F::ColBox_AddPointKeepW(c, p_box_1, fp.wrapping_add(16));
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "StgCol_GetZoneBox", &PROBE_N, &[], true); }
    r_
}

pub fn StgCol_ClipSegToZone(c: &mut Ctx, mut p_out_1: u32, mut p_cur_2: i32, mut p_from_3: i32, mut p_to_4: i32, mut p_ctx_5: u32, mut p_zoneBox_6: u32, mut p_entry_7: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgCol_ClipSegToZone", &PROBE_N, &[(p_out_1) as u32, (p_cur_2) as u32, (p_from_3) as u32, (p_to_4) as u32, (p_ctx_5) as u32, (p_zoneBox_6) as u32, (p_entry_7) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let r_ = 'f: {
        F::StgAabb_SetEmpty(c, p_out_1);
        if (((((p_from_3) == (p_to_4)) as i32)) != 0) {
            { let s_ = (p_ctx_5).wrapping_add(64); let d_ = p_out_1; rt::memcpy(c, d_, s_, 24); };
            break 'f;
        }
        if (((((p_cur_2) == (p_from_3)) as i32)) != 0) {
            F::ColBox_AddPoint(c, p_out_1, (p_ctx_5).wrapping_add(32));
            { let s_ = (p_ctx_5).wrapping_add(96); let d_ = fp; rt::memcpy(c, d_, s_, 32); };
            F::Vec4_Copy(c, fp, ((p_ctx_5).wrapping_add(32)).wrapping_add(16));
            F::Vec3_Scale(c, (fp).wrapping_add(16), (fp).wrapping_add(16), F32(0xbf800000));
            if ((F::ColBox_ClipRay(c, p_zoneBox_6, fp, p_entry_7)) != 0) {
                F::ColBox_AddPoint(c, p_out_1, p_entry_7);
            }
        } else {
            if (((((p_cur_2) == (p_to_4)) as i32)) != 0) {
                F::ColBox_AddPoint(c, p_out_1, ((p_ctx_5).wrapping_add(32)).wrapping_add(16));
                F::ColBox_AddPoint(c, p_out_1, p_entry_7);
            } else {
                { let s_ = (p_ctx_5).wrapping_add(96); let d_ = fp; rt::memcpy(c, d_, s_, 32); };
                F::Vec4_Copy(c, fp, ((p_ctx_5).wrapping_add(32)).wrapping_add(16));
                F::Vec3_Scale(c, (fp).wrapping_add(16), (fp).wrapping_add(16), F32(0xbf800000));
                F::ColBox_AddPoint(c, p_out_1, p_entry_7);
                if ((F::ColBox_ClipRay(c, p_zoneBox_6, fp, p_entry_7)) != 0) {
                    F::ColBox_AddPoint(c, p_out_1, p_entry_7);
                }
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "StgCol_ClipSegToZone", &PROBE_N, &[], true); }
    r_
}

pub fn StgCol_TraceZone(c: &mut Ctx, mut p_cur_1: i32, mut p_from_2: i32, mut p_to_3: i32, mut p_ctx_4: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgCol_TraceZone", &PROBE_N, &[(p_cur_1) as u32, (p_from_2) as u32, (p_to_3) as u32, (p_ctx_4) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(96) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 96);
    let mut l_zones_5: u32 = 0;
    let mut l_last_6: i32 = 0;
    let mut l_obj_7: i32 = 0;
    let mut l_near_8: u32 = 0;
    let mut l_i_9: i32 = 0;
    let r_ = 'f: {
        l_zones_5 = { let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(60); c.m.r32(a_) };
        if (((((c.m.r32((l_zones_5).wrapping_add((((p_cur_1) as u32)).wrapping_mul(64))) as i32)) & (1i32))) != 0) {
            break 'f;
        }
        { let r_ = 1i32; let a_ = (l_zones_5).wrapping_add((((p_cur_1) as u32)).wrapping_mul(64)); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        l_last_6 = (((p_to_3) == (p_cur_1)) as i32);
        { let a_ = sym::gStgColZoneVisits; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        F::StgCol_GetZoneBox(c, fp.wrapping_add(32), p_cur_1);
        if ((((!((F::ColBox_Overlaps(c, fp.wrapping_add(32), (p_ctx_4).wrapping_add(64))) != 0)) as i32)) != 0) {
            break 'f;
        }
        if ((((!((F::ColBox_ClipRay(c, fp.wrapping_add(32), (p_ctx_4).wrapping_add(96), fp.wrapping_add(80))) != 0)) as i32)) != 0) {
            break 'f;
        }
        F::StgCol_ClipSegToZone(c, fp.wrapping_add(56), p_cur_1, p_from_2, p_to_3, p_ctx_4, fp.wrapping_add(32), fp.wrapping_add(80));
        F::StgCol_SetModeAll(c);
        if ((F::StgCol_QueryZone(c, fp, (l_zones_5).wrapping_add((((p_cur_1) as u32)).wrapping_mul(64)), fp.wrapping_add(56), p_ctx_4, fid::StgCol_SegCb)) != 0) {
            l_obj_7 = { let a0_ = c.m.r64((fp).wrapping_add(24)); F::StgCol_FirstBit(c, a0_) };
            if (((((l_obj_7) >= (0i32)) as i32)) != 0) {
                if ((((!((F::BtlStage_IsObjBroken(c, l_obj_7)) != 0)) as i32)) != 0) {
                    { let v_ = l_obj_7; let a_ = (p_ctx_4).wrapping_add(196); c.m.w32(a_, (v_) as u32); };
                }
            }
        } else {
            { let a_ = sym::gStgColZoneMisses; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
            if ((((!((l_last_6) != 0)) as i32)) != 0) {
                l_near_8 = c.m.r32(((l_zones_5).wrapping_add((((p_cur_1) as u32)).wrapping_mul(64))).wrapping_add(12));
                {
                    l_i_9 = 0i32;
                    'l1: loop {
                        if !(((((l_i_9) < ((c.m.r32(((l_zones_5).wrapping_add((((p_cur_1) as u32)).wrapping_mul(64))).wrapping_add(8)) as i32))) as i32)) != 0) { break 'l1; }
                        'c2: {
                            { let a1_ = (c.m.r32((l_near_8).wrapping_add((((l_i_9) as u32)).wrapping_mul(4))) as i32); F::StgCol_GetZoneBox(c, fp.wrapping_add(32), a1_) };
                            if ((F::ColBox_ContainsPoint(c, fp.wrapping_add(32), fp.wrapping_add(80))) != 0) {
                                { let a0_ = (c.m.r32((l_near_8).wrapping_add((((l_i_9) as u32)).wrapping_mul(4))) as i32); F::StgCol_TraceZone(c, a0_, p_from_2, p_to_3, p_ctx_4) };
                            }
                        }
                        { let o_ = l_i_9; let n_ = (o_).wrapping_add(1); l_i_9 = n_; };
                    }
                }
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "StgCol_TraceZone", &PROBE_N, &[], true); }
    r_
}

pub fn StgCol_CollectZone(c: &mut Ctx, mut p_zoneIdx_1: i32, mut p_ctx_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgCol_CollectZone", &PROBE_N, &[(p_zoneIdx_1) as u32, (p_ctx_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_zones_3: u32 = 0;
    let mut l_obj_4: i32 = 0;
    let r_ = 'f: {
        l_zones_3 = { let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(60); c.m.r32(a_) };
        F::StgCol_SetModeAll(c);
        if ((((!((F::StgCol_QueryZone(c, fp, (l_zones_3).wrapping_add((((p_zoneIdx_1) as u32)).wrapping_mul(64)), (p_ctx_2).wrapping_add(64), p_ctx_2, fid::StgCol_CollectCb)) != 0)) as i32)) != 0) {
            break 'f;
        }
        l_obj_4 = { let a0_ = c.m.r64((fp).wrapping_add(24)); F::StgCol_FirstBit(c, a0_) };
        if (((((l_obj_4) >= (0i32)) as i32)) != 0) {
            if ((((!((F::BtlStage_IsObjBroken(c, l_obj_4)) != 0)) as i32)) != 0) {
                { let v_ = l_obj_4; let a_ = (p_ctx_2).wrapping_add(196); c.m.w32(a_, (v_) as u32); };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "StgCol_CollectZone", &PROBE_N, &[], true); }
    r_
}

pub fn StgCol_TraceSegment(c: &mut Ctx, mut p_seg_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgCol_TraceSegment", &PROBE_N, &[(p_seg_1) as u32], false); }
    let mut l_from_2: i32 = 0;
    let mut l_to_3: i32 = 0;
    let r_ = 'f: {
    if ((((!((F::BtlStage_IsReady(c)) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    { let _ = F::memset(c, sym::gStgColHit, 0i32, 80u32); };
    F::BtlStage_ClearZoneMarks(c);
    l_from_2 = F::BtlStage_FindZoneNear(c, (1i32).wrapping_neg(), p_seg_1);
    l_to_3 = F::BtlStage_FindZoneNear(c, (1i32).wrapping_neg(), (p_seg_1).wrapping_add(16));
    F::StgCol_InitSegCtx(c, sym::gStgColCtx, p_seg_1);
    { let v_ = 0i32; let a_ = sym::gStgColZoneMisses; c.m.w32(a_, (v_) as u32); };
    { let v_ = 0i32; let a_ = sym::gStgColZoneVisits; c.m.w32(a_, (v_) as u32); };
    F::StgCol_TraceZone(c, l_from_2, l_from_2, l_to_3, sym::gStgColCtx);
    if (((c.m.r32((sym::gStgColCtx).wrapping_add(200)) as i32)) != 0) {
        { let v_ = 1i32; let a_ = sym::gStgColHit; c.m.w32(a_, (v_) as u32); };
        F::Vec4_Copy(c, (sym::gStgColHit).wrapping_add(16), (sym::gStgColCtx).wrapping_add(144));
        { let _ = F::memcpy(c, (sym::gStgColHit).wrapping_add(32), (sym::gStgColCtx).wrapping_add(160), 32u32); };
        { let v_ = (c.m.r32((sym::gStgColCtx).wrapping_add(196)) as i32); let a_ = (sym::gStgColHit).wrapping_add(64); c.m.w32(a_, (v_) as u32); };
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "StgCol_TraceSegment", &PROBE_N, &[], true); }
    r_
}

pub fn StgCol_GetHitPos(c: &mut Ctx, mut p_out_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgCol_GetHitPos", &PROBE_N, &[(p_out_1) as u32], false); }
    let r_ = 'f: {
    if (((c.m.r32((sym::gStgColCtx).wrapping_add(200)) as i32)) != 0) {
        F::Vec4_Copy(c, p_out_1, (sym::gStgColCtx).wrapping_add(144));
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "StgCol_GetHitPos", &PROBE_N, &[], true); }
    r_
}

pub fn StgCol_GetHit(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgCol_GetHit", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f sym::gStgColHit;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "StgCol_GetHit", &PROBE_N, &[], true); }
    r_
}

pub fn StgCol_CollectSphere(c: &mut Ctx, mut p_sphere_1: u32, mut p_list_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgCol_CollectSphere", &PROBE_N, &[(p_sphere_1) as u32, (p_list_2) as u32], false); }
    let mut l_zone_3: i32 = 0;
    let mut l_ctx_4: u32 = 0;
    let r_ = 'f: {
    { let _ = F::memset(c, sym::gStgColHit, 0i32, 80u32); };
    F::BtlStage_ClearZoneMarks(c);
    l_zone_3 = F::BtlStage_FindZoneNear(c, (1i32).wrapping_neg(), p_sphere_1);
    l_ctx_4 = sym::gStgColCtx;
    F::StgCol_InitSphereCtx(c, l_ctx_4, p_sphere_1, p_list_2);
    { let v_ = 0i32; let a_ = sym::gStgColZoneMisses; c.m.w32(a_, (v_) as u32); };
    { let v_ = 0i32; let a_ = sym::gStgColZoneVisits; c.m.w32(a_, (v_) as u32); };
    F::StgCol_CollectZone(c, l_zone_3, l_ctx_4);
    break 'f ((((c.m.r32((l_ctx_4).wrapping_add(200)) as i32)) != (0i32)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "StgCol_CollectSphere", &PROBE_N, &[], true); }
    r_
}

pub fn StgShadow_InitCtx(c: &mut Ctx, mut p_ctx_1: u32, mut p_box_2: u32, mut p_max_3: i32, mut p_buf_4: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgShadow_InitCtx", &PROBE_N, &[(p_ctx_1) as u32, (p_box_2) as u32, (p_max_3) as u32, (p_buf_4) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let r_ = 'f: {
        { let _ = F::memset(c, p_ctx_1, 0i32, 96u32); };
        { let v_ = p_max_3; let a_ = p_ctx_1; c.m.w32(a_, (v_) as u32); };
        { let v_ = p_buf_4; let a_ = (p_ctx_1).wrapping_add(8); c.m.w32(a_, v_); };
        F::ColBox_GetCenter(c, p_box_2, fp);
        F::ColBox_GetHalf(c, p_box_2, fp.wrapping_add(16));
        { let v_ = rt::fmul(rt::fadd(rt::fmul(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))), F32(0x40000000)), F32(0x42480000)), F32(0x3f000000)); let a_ = (fp.wrapping_add(16)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fadd(F32(c.m.r32((p_box_2).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4)))); let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
        F::ColBox_SetCenterHalf(c, (p_ctx_1).wrapping_add(64), fp, fp.wrapping_add(16));
        { let v_ = { let v_ = F32(0x3dcccccc); let a_ = fp.wrapping_add(16); c.m.w32(a_, (v_).0); v_ }; let a_ = (fp.wrapping_add(16)).wrapping_add(8); c.m.w32(a_, (v_).0); };
        F::ColBox_SetCenterHalf(c, (p_ctx_1).wrapping_add(40), fp, fp.wrapping_add(16));
        { let v_ = F32(c.m.r32(fp)); let a_ = (p_ctx_1).wrapping_add(16); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32(((p_ctx_1).wrapping_add(40)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a_ = ((p_ctx_1).wrapping_add(16)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((fp).wrapping_add(8))); let a_ = ((p_ctx_1).wrapping_add(16)).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = ((p_ctx_1).wrapping_add(16)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((((p_ctx_1).wrapping_add(40)).wrapping_add(12)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a_ = (p_ctx_1).wrapping_add(32); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32(((p_box_2).wrapping_add(12)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a_ = (p_ctx_1).wrapping_add(36); c.m.w32(a_, (v_).0); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "StgShadow_InitCtx", &PROBE_N, &[], true); }
    r_
}

pub fn StgShadow_ProbeTri(c: &mut Ctx, mut p_pt_1: u32, mut p_tri_2: u32, mut p_y_3: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgShadow_ProbeTri", &PROBE_N, &[(p_pt_1) as u32, (p_tri_2) as u32, (p_y_3) as u32], false); }
    let mut l_ret_4: i32 = 0;
    let mut l_x0_5: F32 = F32(0);
    let mut l_z0_6: F32 = F32(0);
    let mut l_pz_7: F32 = F32(0);
    let mut l_px_8: F32 = F32(0);
    let mut l_x2_9: F32 = F32(0);
    let mut l_z2_10: F32 = F32(0);
    let mut l_x1_11: F32 = F32(0);
    let mut l_z1_12: F32 = F32(0);
    let mut l_h_13: F32 = F32(0);
    let r_ = 'f: {
    l_ret_4 = 0i32;
    l_x0_5 = F32(c.m.r32((p_tri_2).wrapping_add((((0i32) as u32)).wrapping_mul(16))));
    l_z0_6 = F32(c.m.r32(((p_tri_2).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8)));
    l_pz_7 = F32(c.m.r32((p_pt_1).wrapping_add(8)));
    l_px_8 = F32(c.m.r32(p_pt_1));
    l_x2_9 = F32(c.m.r32((p_tri_2).wrapping_add((((2i32) as u32)).wrapping_mul(16))));
    l_z2_10 = F32(c.m.r32(((p_tri_2).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8)));
    l_x1_11 = F32(c.m.r32((p_tri_2).wrapping_add((((1i32) as u32)).wrapping_mul(16))));
    l_z1_12 = F32(c.m.r32(((p_tri_2).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8)));
    l_h_13 = rt::fsub(rt::fmul(rt::fsub(l_x2_9, l_x0_5), rt::fsub(l_pz_7, l_z0_6)), rt::fmul(rt::fsub(l_z2_10, l_z0_6), rt::fsub(l_px_8, l_x0_5)));
    if (((rt::fle(l_h_13, F32(0x3a83126e)) as i32)) != 0) {
        if (((rt::fle(rt::fsub(rt::fmul(rt::fsub(l_x1_11, l_x2_9), rt::fsub(l_pz_7, l_z2_10)), rt::fmul(rt::fsub(l_z1_12, l_z2_10), rt::fsub(l_px_8, l_x2_9))), F32(0x3a83126e)) as i32)) != 0) {
            if (((rt::fle(rt::fsub(rt::fmul(rt::fsub(l_x0_5, l_x1_11), rt::fsub(l_pz_7, l_z1_12)), rt::fmul(rt::fsub(l_z0_6, l_z1_12), rt::fsub(l_px_8, l_x1_11))), F32(0x3a83126e)) as i32)) != 0) {
                l_h_13 = rt::fdiv(rt::fsub(rt::fsub(F32(c.m.r32(((p_tri_2).wrapping_add(48)).wrapping_add(12))), rt::fmul(F32(c.m.r32((p_tri_2).wrapping_add(48))), l_px_8)), rt::fmul(F32(c.m.r32(((p_tri_2).wrapping_add(48)).wrapping_add(8))), l_pz_7)), F32(c.m.r32(((p_tri_2).wrapping_add(48)).wrapping_add(4))));
                if (((rt::flt(l_h_13, F32(c.m.r32(p_y_3))) as i32)) != 0) {
                    { let v_ = l_h_13; let a_ = p_y_3; c.m.w32(a_, (v_).0); };
                }
                l_ret_4 = 1i32;
            }
        }
    }
    break 'f l_ret_4;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "StgShadow_ProbeTri", &PROBE_N, &[], true); }
    r_
}

pub fn StgShadow_AddTri(c: &mut Ctx, mut p_ctx_1: u32, mut p_tri_2: u32, mut p_flags_3: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgShadow_AddTri", &PROBE_N, &[(p_ctx_1) as u32, (p_tri_2) as u32, (p_flags_3) as u32], false); }
    let mut l_count_4: u32 = 0;
    let r_ = 'f: {
    if ((((((c.m.r32((p_ctx_1).wrapping_add(4)) as i32)) >= ((c.m.r32(p_ctx_1) as i32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((p_flags_3) & (16777216i32))) != 0) {
        if (((rt::fge(F32(c.m.r32(((p_tri_2).wrapping_add(48)).wrapping_add(4))), F32(0xbf7fbe76)) as i32)) != 0) {
            break 'f 0i32;
        }
    }
    l_count_4 = (p_ctx_1).wrapping_add(4);
    { let s_ = p_tri_2; let d_ = (c.m.r32((p_ctx_1).wrapping_add(8))).wrapping_add(((({ let a_ = l_count_4; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); o_ }) as u32)).wrapping_mul(64)); rt::memcpy(c, d_, s_, 64); };
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "StgShadow_AddTri", &PROBE_N, &[], true); }
    r_
}

pub fn StgShadow_Cb(c: &mut Ctx, mut p_node_1: u32, mut p_ctx_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgShadow_Cb", &PROBE_N, &[(p_node_1) as u32, (p_ctx_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(64) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 64);
    let mut l_poly_3: u32 = 0;
    let r_ = 'f: {
        l_poly_3 = { let a0_ = c.m.r32(sym::gStgColMesh); let a1_ = (c.m.r32((p_node_1).wrapping_add(24)) as i32); F::ColMesh_GetPoly(c, a0_, a1_) };
        if (((((c.m.r32(l_poly_3) as i32)) & (((((((134217728i32) | (33554432i32))) | (64i32))) | (32i32))))) != 0) {
            break 'f 0i32;
        }
        if ((((!(((rt::flt(F::Vec3_Dot(c, sym::D_002EC2E0, (l_poly_3).wrapping_add(16)), F32(0x00000000)) as i32)) != 0)) as i32)) != 0) {
            break 'f 0i32;
        }
        { let a0_ = c.m.r32(sym::gStgColMesh); F::ColMesh_GetPolyVerts(c, a0_, l_poly_3, (fp).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (fp).wrapping_add((((1i32) as u32)).wrapping_mul(16)), (fp).wrapping_add((((2i32) as u32)).wrapping_mul(16))) };
        F::Vec4_Copy(c, (fp).wrapping_add(48), (l_poly_3).wrapping_add(16));
        if ((F::ColNode_TestBox(c, p_node_1, (p_ctx_2).wrapping_add(40))) != 0) {
            { let _ = F::StgShadow_ProbeTri(c, (p_ctx_2).wrapping_add(16), fp, (p_ctx_2).wrapping_add(32)); };
        }
        break 'f ((({ let a2_ = (c.m.r32(l_poly_3) as i32); F::StgShadow_AddTri(c, p_ctx_2, fp, a2_) }) != (0i32)) as i32);
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "StgShadow_Cb", &PROBE_N, &[], true); }
    r_
}

pub fn StgShadow_CalcAlpha(c: &mut Ctx, mut p_height_1: F32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgShadow_CalcAlpha", &PROBE_N, &[(p_height_1).0], false); }
    let mut l_alpha_2: F32 = F32(0);
    let r_ = 'f: {
    l_alpha_2 = F32(0x3f800000);
    if (((rt::fle(p_height_1, F32(0x00000000)) as i32)) != 0) {
        break 'f l_alpha_2;
    }
    if (((rt::fge(p_height_1, F32(0x42480000)) as i32)) != 0) {
        break 'f F32(0x00000000);
    }
    if (((rt::fgt(p_height_1, F32(0x41a00000)) as i32)) != 0) {
        break 'f rt::fsub(l_alpha_2, rt::fdiv(rt::fsub(p_height_1, F32(0x41a00000)), F32(0x41f00000)));
    }
    break 'f l_alpha_2;
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "StgShadow_CalcAlpha", &PROBE_N, &[], true); }
    r_
}

pub fn StgShadow_Finish(c: &mut Ctx, mut p_ctx_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgShadow_Finish", &PROBE_N, &[(p_ctx_1) as u32], false); }
    let mut l_alpha_2: F32 = F32(0);
    let r_ = 'f: {
    { let _ = F::memset(c, sym::gStgShadow, 0i32, 8u32); };
    l_alpha_2 = { let a0_ = rt::fsub(F32(c.m.r32((p_ctx_1).wrapping_add(32))), F32(c.m.r32((p_ctx_1).wrapping_add(36)))); F::StgShadow_CalcAlpha(c, a0_) };
    { let v_ = l_alpha_2; let a_ = sym::gStgShadow; c.m.w32(a_, (v_).0); };
    if (((rt::fle(l_alpha_2, F32(0x00000000)) as i32)) != 0) {
        { let v_ = 0i32; let a_ = (sym::gStgShadow).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
    } else {
        { let v_ = (c.m.r32((p_ctx_1).wrapping_add(4)) as i32); let a_ = (sym::gStgShadow).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgShadow_Finish", &PROBE_N, &[], true); }
    r_
}

pub fn StgShadow_Collect(c: &mut Ctx, mut p_zoneIdx_1: i32, mut p_box_2: u32, mut p_max_3: i32, mut p_buf_4: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgShadow_Collect", &PROBE_N, &[(p_zoneIdx_1) as u32, (p_box_2) as u32, (p_max_3) as u32, (p_buf_4) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(128) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 128);
    let mut l_zone_5: u32 = 0;
    let r_ = 'f: {
        if (((((p_buf_4) == (((0i32) as u32))) as i32)) != 0) {
            break 'f 0i32;
        }
        if (((((p_box_2) == (((0i32) as u32))) as i32)) != 0) {
            break 'f 0i32;
        }
        if ((((!((F::BtlStage_IsReady(c)) != 0)) as i32)) != 0) {
            break 'f 0i32;
        }
        F::StgShadow_InitCtx(c, fp, p_box_2, p_max_3, p_buf_4);
        l_zone_5 = { let a0_ = F::BtlStage_FindZoneNear(c, p_zoneIdx_1, (fp).wrapping_add(16)); F::BtlStage_GetZone(c, a0_) };
        F::StgCol_SetModeAll(c);
        { let _ = F::StgCol_QueryZone(c, fp.wrapping_add(96), l_zone_5, (fp).wrapping_add(64), fp, fid::StgShadow_Cb); };
        F::StgShadow_Finish(c, fp);
        break 'f (c.m.r32((fp).wrapping_add(4)) as i32);
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "StgShadow_Collect", &PROBE_N, &[], true); }
    r_
}

pub fn StgShadow_GetOut(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgShadow_GetOut", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f sym::gStgShadow;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "StgShadow_GetOut", &PROBE_N, &[], true); }
    r_
}
