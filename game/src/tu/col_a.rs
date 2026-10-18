#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn ColBox_SetCenterHalf(c: &mut Ctx, mut p_box_1: u32, mut p_center_2: u32, mut p_half_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ColBox_SetCenterHalf", &PROBE_N, &[(p_box_1) as u32, (p_center_2) as u32, (p_half_3) as u32], false); }
    let r_ = 'f: {
    { let v_ = rt::fsub(F32(c.m.r32(p_center_2)), F32(c.m.r32(p_half_3))); let a_ = (p_box_1).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fsub(F32(c.m.r32((p_center_2).wrapping_add(4))), F32(c.m.r32((p_half_3).wrapping_add(4)))); let a_ = (p_box_1).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fsub(F32(c.m.r32((p_center_2).wrapping_add(8))), F32(c.m.r32((p_half_3).wrapping_add(8)))); let a_ = (p_box_1).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fadd(F32(c.m.r32(p_center_2)), F32(c.m.r32(p_half_3))); let a_ = ((p_box_1).wrapping_add(12)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fadd(F32(c.m.r32((p_center_2).wrapping_add(4))), F32(c.m.r32((p_half_3).wrapping_add(4)))); let a_ = ((p_box_1).wrapping_add(12)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fadd(F32(c.m.r32((p_center_2).wrapping_add(8))), F32(c.m.r32((p_half_3).wrapping_add(8)))); let a_ = ((p_box_1).wrapping_add(12)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ColBox_SetCenterHalf", &PROBE_N, &[], true); }
    r_
}

pub fn ColBox_AddPoint(c: &mut Ctx, mut p_box_1: u32, mut p_pt_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ColBox_AddPoint", &PROBE_N, &[(p_box_1) as u32, (p_pt_2) as u32], false); }
    let r_ = 'f: {
    if (((rt::flt(F32(c.m.r32(p_pt_2)), F32(c.m.r32((p_box_1).wrapping_add((((0i32) as u32)).wrapping_mul(4))))) as i32)) != 0) {
        { let v_ = F32(c.m.r32(p_pt_2)); let a_ = (p_box_1).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    }
    if (((rt::flt(F32(c.m.r32(((p_box_1).wrapping_add(12)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(p_pt_2))) as i32)) != 0) {
        { let v_ = F32(c.m.r32(p_pt_2)); let a_ = ((p_box_1).wrapping_add(12)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    }
    if (((rt::flt(F32(c.m.r32((p_pt_2).wrapping_add(4))), F32(c.m.r32((p_box_1).wrapping_add((((1i32) as u32)).wrapping_mul(4))))) as i32)) != 0) {
        { let v_ = F32(c.m.r32((p_pt_2).wrapping_add(4))); let a_ = (p_box_1).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    }
    if (((rt::flt(F32(c.m.r32(((p_box_1).wrapping_add(12)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((p_pt_2).wrapping_add(4)))) as i32)) != 0) {
        { let v_ = F32(c.m.r32((p_pt_2).wrapping_add(4))); let a_ = ((p_box_1).wrapping_add(12)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    }
    if (((rt::flt(F32(c.m.r32((p_pt_2).wrapping_add(8))), F32(c.m.r32((p_box_1).wrapping_add((((2i32) as u32)).wrapping_mul(4))))) as i32)) != 0) {
        { let v_ = F32(c.m.r32((p_pt_2).wrapping_add(8))); let a_ = (p_box_1).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    }
    if (((rt::flt(F32(c.m.r32(((p_box_1).wrapping_add(12)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((p_pt_2).wrapping_add(8)))) as i32)) != 0) {
        { let v_ = F32(c.m.r32((p_pt_2).wrapping_add(8))); let a_ = ((p_box_1).wrapping_add(12)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    }
    { let v_ = F32(0x3f800000); let a_ = (p_pt_2).wrapping_add(12); c.m.w32(a_, (v_).0); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ColBox_AddPoint", &PROBE_N, &[], true); }
    r_
}

pub fn ColBox_AddPointKeepW(c: &mut Ctx, mut p_box_1: u32, mut p_pt_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ColBox_AddPointKeepW", &PROBE_N, &[(p_box_1) as u32, (p_pt_2) as u32], false); }
    let r_ = 'f: {
    if (((rt::flt(F32(c.m.r32(p_pt_2)), F32(c.m.r32((p_box_1).wrapping_add((((0i32) as u32)).wrapping_mul(4))))) as i32)) != 0) {
        { let v_ = F32(c.m.r32(p_pt_2)); let a_ = (p_box_1).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    }
    if (((rt::flt(F32(c.m.r32(((p_box_1).wrapping_add(12)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(p_pt_2))) as i32)) != 0) {
        { let v_ = F32(c.m.r32(p_pt_2)); let a_ = ((p_box_1).wrapping_add(12)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    }
    if (((rt::flt(F32(c.m.r32((p_pt_2).wrapping_add(4))), F32(c.m.r32((p_box_1).wrapping_add((((1i32) as u32)).wrapping_mul(4))))) as i32)) != 0) {
        { let v_ = F32(c.m.r32((p_pt_2).wrapping_add(4))); let a_ = (p_box_1).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    }
    if (((rt::flt(F32(c.m.r32(((p_box_1).wrapping_add(12)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((p_pt_2).wrapping_add(4)))) as i32)) != 0) {
        { let v_ = F32(c.m.r32((p_pt_2).wrapping_add(4))); let a_ = ((p_box_1).wrapping_add(12)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    }
    if (((rt::flt(F32(c.m.r32((p_pt_2).wrapping_add(8))), F32(c.m.r32((p_box_1).wrapping_add((((2i32) as u32)).wrapping_mul(4))))) as i32)) != 0) {
        { let v_ = F32(c.m.r32((p_pt_2).wrapping_add(8))); let a_ = (p_box_1).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    }
    if (((rt::flt(F32(c.m.r32(((p_box_1).wrapping_add(12)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((p_pt_2).wrapping_add(8)))) as i32)) != 0) {
        { let v_ = F32(c.m.r32((p_pt_2).wrapping_add(8))); let a_ = ((p_box_1).wrapping_add(12)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ColBox_AddPointKeepW", &PROBE_N, &[], true); }
    r_
}

pub fn ColBox_GetCenter(c: &mut Ctx, mut p_box_1: u32, mut p_out_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ColBox_GetCenter", &PROBE_N, &[(p_box_1) as u32, (p_out_2) as u32], false); }
    let r_ = 'f: {
    { let v_ = rt::fmul(rt::fadd(F32(c.m.r32(((p_box_1).wrapping_add(12)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((p_box_1).wrapping_add((((0i32) as u32)).wrapping_mul(4))))), F32(0x3f000000)); let a_ = p_out_2; c.m.w32(a_, (v_).0); };
    { let v_ = rt::fmul(rt::fadd(F32(c.m.r32(((p_box_1).wrapping_add(12)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((p_box_1).wrapping_add((((1i32) as u32)).wrapping_mul(4))))), F32(0x3f000000)); let a_ = (p_out_2).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fmul(rt::fadd(F32(c.m.r32(((p_box_1).wrapping_add(12)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((p_box_1).wrapping_add((((2i32) as u32)).wrapping_mul(4))))), F32(0x3f000000)); let a_ = (p_out_2).wrapping_add(8); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x3f800000); let a_ = (p_out_2).wrapping_add(12); c.m.w32(a_, (v_).0); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ColBox_GetCenter", &PROBE_N, &[], true); }
    r_
}

pub fn ColBox_GetHalf(c: &mut Ctx, mut p_box_1: u32, mut p_out_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ColBox_GetHalf", &PROBE_N, &[(p_box_1) as u32, (p_out_2) as u32], false); }
    let r_ = 'f: {
    { let v_ = rt::fmul(rt::fsub(F32(c.m.r32(((p_box_1).wrapping_add(12)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((p_box_1).wrapping_add((((0i32) as u32)).wrapping_mul(4))))), F32(0x3f000000)); let a_ = p_out_2; c.m.w32(a_, (v_).0); };
    { let v_ = rt::fmul(rt::fsub(F32(c.m.r32(((p_box_1).wrapping_add(12)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((p_box_1).wrapping_add((((1i32) as u32)).wrapping_mul(4))))), F32(0x3f000000)); let a_ = (p_out_2).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fmul(rt::fsub(F32(c.m.r32(((p_box_1).wrapping_add(12)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((p_box_1).wrapping_add((((2i32) as u32)).wrapping_mul(4))))), F32(0x3f000000)); let a_ = (p_out_2).wrapping_add(8); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x3f800000); let a_ = (p_out_2).wrapping_add(12); c.m.w32(a_, (v_).0); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ColBox_GetHalf", &PROBE_N, &[], true); }
    r_
}

pub fn ColBox_GetSize(c: &mut Ctx, mut p_box_1: u32, mut p_out_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ColBox_GetSize", &PROBE_N, &[(p_box_1) as u32, (p_out_2) as u32], false); }
    let r_ = 'f: {
    { let v_ = rt::fsub(F32(c.m.r32(((p_box_1).wrapping_add(12)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((p_box_1).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a_ = p_out_2; c.m.w32(a_, (v_).0); };
    { let v_ = rt::fsub(F32(c.m.r32(((p_box_1).wrapping_add(12)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((p_box_1).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); let a_ = (p_out_2).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fsub(F32(c.m.r32(((p_box_1).wrapping_add(12)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((p_box_1).wrapping_add((((2i32) as u32)).wrapping_mul(4))))); let a_ = (p_out_2).wrapping_add(8); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x3f800000); let a_ = (p_out_2).wrapping_add(12); c.m.w32(a_, (v_).0); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ColBox_GetSize", &PROBE_N, &[], true); }
    r_
}

pub fn ColBox_HasVolume(c: &mut Ctx, mut p_box_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ColBox_HasVolume", &PROBE_N, &[(p_box_1) as u32], false); }
    let r_ = 'f: {
    if (((((((((((rt::flt(F32(c.m.r32(((p_box_1).wrapping_add(12)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), rt::fadd(F32(c.m.r32((p_box_1).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(0x358637bd))) as i32)) != 0) || (((rt::flt(F32(c.m.r32(((p_box_1).wrapping_add(12)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), rt::fadd(F32(c.m.r32((p_box_1).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(0x358637bd))) as i32)) != 0)) as i32)) != 0) || (((rt::flt(F32(c.m.r32(((p_box_1).wrapping_add(12)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), rt::fadd(F32(c.m.r32((p_box_1).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), F32(0x358637bd))) as i32)) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "ColBox_HasVolume", &PROBE_N, &[], true); }
    r_
}

pub fn ColBox_ContainsPoint(c: &mut Ctx, mut p_box_1: u32, mut p_pt_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ColBox_ContainsPoint", &PROBE_N, &[(p_box_1) as u32, (p_pt_2) as u32], false); }
    let mut l_ret_3: i32 = 0;
    let r_ = 'f: {
    l_ret_3 = 0i32;
    if ((((((((((((((((((((((((!(((rt::flt(F32(c.m.r32(((p_box_1).wrapping_add(12)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(p_pt_2))) as i32)) != 0)) as i32)) != 0) && ((((!(((rt::flt(F32(c.m.r32(p_pt_2)), F32(c.m.r32((p_box_1).wrapping_add((((0i32) as u32)).wrapping_mul(4))))) as i32)) != 0)) as i32)) != 0)) as i32)) != 0) && ((((!(((rt::flt(F32(c.m.r32(((p_box_1).wrapping_add(12)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((p_pt_2).wrapping_add(4)))) as i32)) != 0)) as i32)) != 0)) as i32)) != 0) && ((((!(((rt::flt(F32(c.m.r32((p_pt_2).wrapping_add(4))), F32(c.m.r32((p_box_1).wrapping_add((((1i32) as u32)).wrapping_mul(4))))) as i32)) != 0)) as i32)) != 0)) as i32)) != 0) && ((((!(((rt::flt(F32(c.m.r32(((p_box_1).wrapping_add(12)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((p_pt_2).wrapping_add(8)))) as i32)) != 0)) as i32)) != 0)) as i32)) != 0) && ((((!(((rt::flt(F32(c.m.r32((p_pt_2).wrapping_add(8))), F32(c.m.r32((p_box_1).wrapping_add((((2i32) as u32)).wrapping_mul(4))))) as i32)) != 0)) as i32)) != 0)) as i32)) != 0) {
        l_ret_3 = 1i32;
    }
    break 'f l_ret_3;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "ColBox_ContainsPoint", &PROBE_N, &[], true); }
    r_
}

pub fn ColNode_IsLeaf(c: &mut Ctx, mut p_node_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ColNode_IsLeaf", &PROBE_N, &[(p_node_1) as u32], false); }
    let r_ = 'f: {
    break 'f ((((c.m.r32((p_node_1).wrapping_add(28)) as i32)) == ((1i32).wrapping_neg())) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "ColNode_IsLeaf", &PROBE_N, &[], true); }
    r_
}

pub fn ColNode_IsEmptyLeaf(c: &mut Ctx, mut p_node_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ColNode_IsEmptyLeaf", &PROBE_N, &[(p_node_1) as u32], false); }
    let mut l_ret_2: i32 = 0;
    let r_ = 'f: {
    l_ret_2 = 0i32;
    if ((((((c.m.r32((p_node_1).wrapping_add(28)) as i32)) == ((1i32).wrapping_neg())) as i32)) != 0) {
        l_ret_2 = ((((c.m.r32((p_node_1).wrapping_add(24)) as i32)) == ((1i32).wrapping_neg())) as i32);
    }
    break 'f l_ret_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "ColNode_IsEmptyLeaf", &PROBE_N, &[], true); }
    r_
}

pub fn ColMesh_GetPolyVerts(c: &mut Ctx, mut p_mesh_1: u32, mut p_poly_2: u32, mut p_v0_3: u32, mut p_v1_4: u32, mut p_v2_5: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ColMesh_GetPolyVerts", &PROBE_N, &[(p_mesh_1) as u32, (p_poly_2) as u32, (p_v0_3) as u32, (p_v1_4) as u32, (p_v2_5) as u32], false); }
    let mut l_vtx_6: u32 = 0;
    let r_ = 'f: {
    l_vtx_6 = (c.m.r32(sym::gColMeshBase)).wrapping_add(((((c.m.r32((p_mesh_1).wrapping_add(20)) as i32)) as u32)).wrapping_mul(4));
    { let a1_ = (l_vtx_6).wrapping_add(((((c.m.r32(((p_poly_2).wrapping_add(4)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(16)); F::Vec4_Copy(c, p_v0_3, a1_) };
    { let a1_ = (l_vtx_6).wrapping_add(((((c.m.r32(((p_poly_2).wrapping_add(4)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(16)); F::Vec4_Copy(c, p_v1_4, a1_) };
    { let a1_ = (l_vtx_6).wrapping_add(((((c.m.r32(((p_poly_2).wrapping_add(4)).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(16)); F::Vec4_Copy(c, p_v2_5, a1_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ColMesh_GetPolyVerts", &PROBE_N, &[], true); }
    r_
}

pub fn ColMesh_GetPoly(c: &mut Ctx, mut p_mesh_1: u32, mut p_idx_2: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ColMesh_GetPoly", &PROBE_N, &[(p_mesh_1) as u32, (p_idx_2) as u32], false); }
    let mut l_polys_3: u32 = 0;
    let r_ = 'f: {
    l_polys_3 = (c.m.r32(sym::gColMeshBase)).wrapping_add(((((c.m.r32((p_mesh_1).wrapping_add(16)) as i32)) as u32)).wrapping_mul(4));
    break 'f (l_polys_3).wrapping_add((((p_idx_2) as u32)).wrapping_mul(32));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "ColMesh_GetPoly", &PROBE_N, &[], true); }
    r_
}

pub fn ColMesh_SetBase(c: &mut Ctx, mut p_base_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ColMesh_SetBase", &PROBE_N, &[(p_base_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_base_1; let a_ = sym::gColMeshBase; c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ColMesh_SetBase", &PROBE_N, &[], true); }
    r_
}

pub fn ColObb_Init(c: &mut Ctx, mut p_obb_1: u32, mut p_center_2: u32, mut p_hx_3: F32, mut p_hy_4: F32, mut p_hz_5: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ColObb_Init", &PROBE_N, &[(p_obb_1) as u32, (p_center_2) as u32, (p_hx_3).0, (p_hy_4).0, (p_hz_5).0], false); }
    let r_ = 'f: {
    { let v_ = F32(c.m.r32(p_center_2)); let a_ = p_obb_1; c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((p_center_2).wrapping_add(4))); let a_ = (p_obb_1).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((p_center_2).wrapping_add(8))); let a_ = (p_obb_1).wrapping_add(8); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x3f800000); let a_ = (p_obb_1).wrapping_add(12); c.m.w32(a_, (v_).0); };
    { let v_ = p_hx_3; let a_ = (p_obb_1).wrapping_add(16); c.m.w32(a_, (v_).0); };
    { let v_ = p_hy_4; let a_ = ((p_obb_1).wrapping_add(16)).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = p_hz_5; let a_ = ((p_obb_1).wrapping_add(16)).wrapping_add(8); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x3f800000); let a_ = ((p_obb_1).wrapping_add(16)).wrapping_add(12); c.m.w32(a_, (v_).0); };
    { let v_ = p_hx_3; let a_ = ((p_obb_1).wrapping_add(144)).wrapping_add((((0i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
    { let v_ = p_hy_4; let a_ = (((p_obb_1).wrapping_add(144)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = p_hz_5; let a_ = (((p_obb_1).wrapping_add(144)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x3f800000); let a_ = (((p_obb_1).wrapping_add(144)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fneg(p_hx_3); let a_ = ((p_obb_1).wrapping_add(144)).wrapping_add((((1i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
    { let v_ = p_hy_4; let a_ = (((p_obb_1).wrapping_add(144)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = p_hz_5; let a_ = (((p_obb_1).wrapping_add(144)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x3f800000); let a_ = (((p_obb_1).wrapping_add(144)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
    { let v_ = p_hx_3; let a_ = ((p_obb_1).wrapping_add(144)).wrapping_add((((2i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fneg(p_hy_4); let a_ = (((p_obb_1).wrapping_add(144)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = p_hz_5; let a_ = (((p_obb_1).wrapping_add(144)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x3f800000); let a_ = (((p_obb_1).wrapping_add(144)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fneg(p_hx_3); let a_ = ((p_obb_1).wrapping_add(144)).wrapping_add((((3i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fneg(p_hy_4); let a_ = (((p_obb_1).wrapping_add(144)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = p_hz_5; let a_ = (((p_obb_1).wrapping_add(144)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x3f800000); let a_ = (((p_obb_1).wrapping_add(144)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
    { let v_ = p_hx_3; let a_ = ((p_obb_1).wrapping_add(144)).wrapping_add((((4i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
    { let v_ = p_hy_4; let a_ = (((p_obb_1).wrapping_add(144)).wrapping_add((((4i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fneg(p_hz_5); let a_ = (((p_obb_1).wrapping_add(144)).wrapping_add((((4i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x3f800000); let a_ = (((p_obb_1).wrapping_add(144)).wrapping_add((((4i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fneg(p_hx_3); let a_ = ((p_obb_1).wrapping_add(144)).wrapping_add((((5i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
    { let v_ = p_hy_4; let a_ = (((p_obb_1).wrapping_add(144)).wrapping_add((((5i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fneg(p_hz_5); let a_ = (((p_obb_1).wrapping_add(144)).wrapping_add((((5i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x3f800000); let a_ = (((p_obb_1).wrapping_add(144)).wrapping_add((((5i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
    { let v_ = p_hx_3; let a_ = ((p_obb_1).wrapping_add(144)).wrapping_add((((6i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fneg(p_hy_4); let a_ = (((p_obb_1).wrapping_add(144)).wrapping_add((((6i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fneg(p_hz_5); let a_ = (((p_obb_1).wrapping_add(144)).wrapping_add((((6i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x3f800000); let a_ = (((p_obb_1).wrapping_add(144)).wrapping_add((((6i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fneg(p_hx_3); let a_ = ((p_obb_1).wrapping_add(144)).wrapping_add((((7i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fneg(p_hy_4); let a_ = (((p_obb_1).wrapping_add(144)).wrapping_add((((7i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fneg(p_hz_5); let a_ = (((p_obb_1).wrapping_add(144)).wrapping_add((((7i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x3f800000); let a_ = (((p_obb_1).wrapping_add(144)).wrapping_add((((7i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ColObb_Init", &PROBE_N, &[], true); }
    r_
}

pub fn ColObb_Update(c: &mut Ctx, mut p_obb_1: u32, mut p_mtx_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ColObb_Update", &PROBE_N, &[(p_obb_1) as u32, (p_mtx_2) as u32], false); }
    let r_ = 'f: {
    F::Mtx_Copy(c, (p_obb_1).wrapping_add(80), p_mtx_2);
    { let v_ = F32(c.m.r32((((p_obb_1).wrapping_add(80)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = ((p_obb_1).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((((p_obb_1).wrapping_add(80)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a_ = (((p_obb_1).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((((p_obb_1).wrapping_add(80)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); let a_ = (((p_obb_1).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((((p_obb_1).wrapping_add(80)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = ((p_obb_1).wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((((p_obb_1).wrapping_add(80)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a_ = (((p_obb_1).wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((((p_obb_1).wrapping_add(80)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); let a_ = (((p_obb_1).wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((((p_obb_1).wrapping_add(80)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = ((p_obb_1).wrapping_add(32)).wrapping_add((((2i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((((p_obb_1).wrapping_add(80)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a_ = (((p_obb_1).wrapping_add(32)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((((p_obb_1).wrapping_add(80)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); let a_ = (((p_obb_1).wrapping_add(32)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((((p_obb_1).wrapping_add(80)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = p_obb_1; c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((((p_obb_1).wrapping_add(80)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a_ = (p_obb_1).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((((p_obb_1).wrapping_add(80)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); let a_ = (p_obb_1).wrapping_add(8); c.m.w32(a_, (v_).0); };
    F::Mtx_MulVec4(c, ((p_obb_1).wrapping_add(272)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (p_obb_1).wrapping_add(80), ((p_obb_1).wrapping_add(144)).wrapping_add((((0i32) as u32)).wrapping_mul(16)));
    F::Mtx_MulVec4(c, ((p_obb_1).wrapping_add(272)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), (p_obb_1).wrapping_add(80), ((p_obb_1).wrapping_add(144)).wrapping_add((((1i32) as u32)).wrapping_mul(16)));
    F::Mtx_MulVec4(c, ((p_obb_1).wrapping_add(272)).wrapping_add((((2i32) as u32)).wrapping_mul(16)), (p_obb_1).wrapping_add(80), ((p_obb_1).wrapping_add(144)).wrapping_add((((2i32) as u32)).wrapping_mul(16)));
    F::Mtx_MulVec4(c, ((p_obb_1).wrapping_add(272)).wrapping_add((((3i32) as u32)).wrapping_mul(16)), (p_obb_1).wrapping_add(80), ((p_obb_1).wrapping_add(144)).wrapping_add((((3i32) as u32)).wrapping_mul(16)));
    F::Mtx_MulVec4(c, ((p_obb_1).wrapping_add(272)).wrapping_add((((4i32) as u32)).wrapping_mul(16)), (p_obb_1).wrapping_add(80), ((p_obb_1).wrapping_add(144)).wrapping_add((((4i32) as u32)).wrapping_mul(16)));
    F::Mtx_MulVec4(c, ((p_obb_1).wrapping_add(272)).wrapping_add((((5i32) as u32)).wrapping_mul(16)), (p_obb_1).wrapping_add(80), ((p_obb_1).wrapping_add(144)).wrapping_add((((5i32) as u32)).wrapping_mul(16)));
    F::Mtx_MulVec4(c, ((p_obb_1).wrapping_add(272)).wrapping_add((((6i32) as u32)).wrapping_mul(16)), (p_obb_1).wrapping_add(80), ((p_obb_1).wrapping_add(144)).wrapping_add((((6i32) as u32)).wrapping_mul(16)));
    F::Mtx_MulVec4(c, ((p_obb_1).wrapping_add(272)).wrapping_add((((7i32) as u32)).wrapping_mul(16)), (p_obb_1).wrapping_add(80), ((p_obb_1).wrapping_add(144)).wrapping_add((((7i32) as u32)).wrapping_mul(16)));
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ColObb_Update", &PROBE_N, &[], true); }
    r_
}

pub fn ColBox_Overlaps(c: &mut Ctx, mut p_a_1: u32, mut p_b_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ColBox_Overlaps", &PROBE_N, &[(p_a_1) as u32, (p_b_2) as u32], false); }
    let mut l_ret_3: i32 = 0;
    let r_ = 'f: {
    l_ret_3 = 0i32;
    if ((((((((((((((((((((((((!(((rt::fgt(F32(c.m.r32((p_a_1).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((p_b_2).wrapping_add(12)).wrapping_add((((0i32) as u32)).wrapping_mul(4))))) as i32)) != 0)) as i32)) != 0) && ((((!(((rt::flt(F32(c.m.r32(((p_a_1).wrapping_add(12)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((p_b_2).wrapping_add((((0i32) as u32)).wrapping_mul(4))))) as i32)) != 0)) as i32)) != 0)) as i32)) != 0) && ((((!(((rt::fgt(F32(c.m.r32((p_a_1).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((p_b_2).wrapping_add(12)).wrapping_add((((1i32) as u32)).wrapping_mul(4))))) as i32)) != 0)) as i32)) != 0)) as i32)) != 0) && ((((!(((rt::flt(F32(c.m.r32(((p_a_1).wrapping_add(12)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((p_b_2).wrapping_add((((1i32) as u32)).wrapping_mul(4))))) as i32)) != 0)) as i32)) != 0)) as i32)) != 0) && ((((!(((rt::fgt(F32(c.m.r32((p_a_1).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((p_b_2).wrapping_add(12)).wrapping_add((((2i32) as u32)).wrapping_mul(4))))) as i32)) != 0)) as i32)) != 0)) as i32)) != 0) && ((((!(((rt::flt(F32(c.m.r32(((p_a_1).wrapping_add(12)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((p_b_2).wrapping_add((((2i32) as u32)).wrapping_mul(4))))) as i32)) != 0)) as i32)) != 0)) as i32)) != 0) {
        l_ret_3 = 1i32;
    }
    break 'f l_ret_3;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "ColBox_Overlaps", &PROBE_N, &[], true); }
    r_
}

pub fn ColBox_ClipRay(c: &mut Ctx, mut p_box_1: u32, mut p_ray_2: u32, mut p_out_3: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ColBox_ClipRay", &PROBE_N, &[(p_box_1) as u32, (p_ray_2) as u32, (p_out_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_inside_4: i32 = 0;
    let mut l_i_5: i32 = 0;
    let mut l_best_6: i32 = 0;
    let r_ = 'f: {
        l_inside_4 = 1i32;
        {
            l_i_5 = 0i32;
            'l1: loop {
                if !(((((l_i_5) < (3i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    if (((rt::flt(F32(c.m.r32((p_ray_2).wrapping_add((((l_i_5) as u32)).wrapping_mul(4)))), F32(c.m.r32((p_box_1).wrapping_add((((l_i_5) as u32)).wrapping_mul(4))))) as i32)) != 0) {
                        { let v_ = ((1i32) as i8); let a_ = (fp).wrapping_add(((l_i_5) as u32)); c.m.w8(a_, (v_) as u8); };
                        { let v_ = F32(c.m.r32((p_box_1).wrapping_add((((l_i_5) as u32)).wrapping_mul(4)))); let a_ = (fp.wrapping_add(16)).wrapping_add((((l_i_5) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                        l_inside_4 = 0i32;
                    } else {
                        if (((rt::flt(F32(c.m.r32(((p_box_1).wrapping_add(12)).wrapping_add((((l_i_5) as u32)).wrapping_mul(4)))), F32(c.m.r32((p_ray_2).wrapping_add((((l_i_5) as u32)).wrapping_mul(4))))) as i32)) != 0) {
                            { let v_ = ((0i32) as i8); let a_ = (fp).wrapping_add(((l_i_5) as u32)); c.m.w8(a_, (v_) as u8); };
                            { let v_ = F32(c.m.r32(((p_box_1).wrapping_add(12)).wrapping_add((((l_i_5) as u32)).wrapping_mul(4)))); let a_ = (fp.wrapping_add(16)).wrapping_add((((l_i_5) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                            l_inside_4 = 0i32;
                        } else {
                            { let v_ = ((2i32) as i8); let a_ = (fp).wrapping_add(((l_i_5) as u32)); c.m.w8(a_, (v_) as u8); };
                        }
                    }
                }
                { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
            }
        }
        if ((l_inside_4) != 0) {
            { let v_ = F32(c.m.r32((p_ray_2).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = (p_out_3).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            { let v_ = F32(c.m.r32((p_ray_2).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a_ = (p_out_3).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            { let v_ = F32(c.m.r32((p_ray_2).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); let a_ = (p_out_3).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x3f800000); let a_ = (p_out_3).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            break 'f 1i32;
        }
        {
            l_i_5 = 0i32;
            'l3: loop {
                if !(((((l_i_5) < (3i32)) as i32)) != 0) { break 'l3; }
                'c4: {
                    if ((((((((((((c.m.r8((fp).wrapping_add(((l_i_5) as u32))) as i8)) as i32)) != (2i32)) as i32)) != 0) && (((rt::fne(F32(c.m.r32(((p_ray_2).wrapping_add(16)).wrapping_add((((l_i_5) as u32)).wrapping_mul(4)))), F32(0x00000000)) as i32)) != 0)) as i32)) != 0) {
                        { let v_ = rt::fdiv(rt::fsub(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add((((l_i_5) as u32)).wrapping_mul(4)))), F32(c.m.r32((p_ray_2).wrapping_add((((l_i_5) as u32)).wrapping_mul(4))))), F32(c.m.r32(((p_ray_2).wrapping_add(16)).wrapping_add((((l_i_5) as u32)).wrapping_mul(4))))); let a_ = (fp.wrapping_add(4)).wrapping_add((((l_i_5) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    } else {
                        { let v_ = F32(0xbf800000); let a_ = (fp.wrapping_add(4)).wrapping_add((((l_i_5) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    }
                }
                { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
            }
        }
        l_best_6 = 0i32;
        {
            l_i_5 = 1i32;
            'l5: loop {
                if !(((((l_i_5) < (3i32)) as i32)) != 0) { break 'l5; }
                'c6: {
                    if (((rt::flt(F32(c.m.r32((fp.wrapping_add(4)).wrapping_add((((l_best_6) as u32)).wrapping_mul(4)))), F32(c.m.r32((fp.wrapping_add(4)).wrapping_add((((l_i_5) as u32)).wrapping_mul(4))))) as i32)) != 0) {
                        l_best_6 = l_i_5;
                    }
                }
                { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
            }
        }
        if (((rt::flt(F32(c.m.r32((fp.wrapping_add(4)).wrapping_add((((l_best_6) as u32)).wrapping_mul(4)))), F32(0x00000000)) as i32)) != 0) {
            break 'f 0i32;
        }
        {
            l_i_5 = 0i32;
            'l7: loop {
                if !(((((l_i_5) < (3i32)) as i32)) != 0) { break 'l7; }
                'c8: {
                    if (((((l_best_6) != (l_i_5)) as i32)) != 0) {
                        { let v_ = rt::fadd(F32(c.m.r32((p_ray_2).wrapping_add((((l_i_5) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((fp.wrapping_add(4)).wrapping_add((((l_best_6) as u32)).wrapping_mul(4)))), F32(c.m.r32(((p_ray_2).wrapping_add(16)).wrapping_add((((l_i_5) as u32)).wrapping_mul(4)))))); let a_ = (p_out_3).wrapping_add((((l_i_5) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                        if (((((((rt::flt(F32(c.m.r32((p_out_3).wrapping_add((((l_i_5) as u32)).wrapping_mul(4)))), F32(c.m.r32((p_box_1).wrapping_add((((l_i_5) as u32)).wrapping_mul(4))))) as i32)) != 0) && (((rt::fgt({ let a0_ = rt::fsub(F32(c.m.r32((p_box_1).wrapping_add((((l_i_5) as u32)).wrapping_mul(4)))), F32(c.m.r32((p_out_3).wrapping_add((((l_i_5) as u32)).wrapping_mul(4))))); rt::ffabs(a0_) }, F32(0x3a83126e)) as i32)) != 0)) as i32)) != 0) {
                            break 'f 0i32;
                        }
                        if (((((((rt::fgt(F32(c.m.r32((p_out_3).wrapping_add((((l_i_5) as u32)).wrapping_mul(4)))), F32(c.m.r32(((p_box_1).wrapping_add(12)).wrapping_add((((l_i_5) as u32)).wrapping_mul(4))))) as i32)) != 0) && (((rt::fgt({ let a0_ = rt::fsub(F32(c.m.r32((p_out_3).wrapping_add((((l_i_5) as u32)).wrapping_mul(4)))), F32(c.m.r32(((p_box_1).wrapping_add(12)).wrapping_add((((l_i_5) as u32)).wrapping_mul(4))))); rt::ffabs(a0_) }, F32(0x3a83126e)) as i32)) != 0)) as i32)) != 0) {
                            break 'f 0i32;
                        }
                    } else {
                        { let v_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add((((l_i_5) as u32)).wrapping_mul(4)))); let a_ = (p_out_3).wrapping_add((((l_i_5) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    }
                }
                { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
            }
        }
        break 'f 1i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "ColBox_ClipRay", &PROBE_N, &[], true); }
    r_
}

pub fn ColBox_TestSegment(c: &mut Ctx, mut p_box_1: u32, mut p_seg_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ColBox_TestSegment", &PROBE_N, &[(p_box_1) as u32, (p_seg_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(64) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 64);
    let mut l_ax_3: F32 = F32(0);
    let mut l_ay_4: F32 = F32(0);
    let mut l_az_5: F32 = F32(0);
    let r_ = 'f: {
        F::ColBox_GetCenter(c, p_box_1, fp);
        F::ColBox_GetHalf(c, p_box_1, fp.wrapping_add(16));
        F::ColSeg_GetMidpoint(c, p_seg_2, fp.wrapping_add(32));
        F::Vec4_Sub(c, fp.wrapping_add(48), (p_seg_2).wrapping_add(16), fp.wrapping_add(32));
        F::Vec4_Sub(c, fp.wrapping_add(32), fp.wrapping_add(32), fp);
        l_ax_3 = { let a0_ = F32(c.m.r32(fp.wrapping_add(48))); rt::ffabs(a0_) };
        if (((rt::fgt({ let a0_ = F32(c.m.r32(fp.wrapping_add(32))); rt::ffabs(a0_) }, rt::fadd(F32(c.m.r32(fp.wrapping_add(16))), l_ax_3)) as i32)) != 0) {
            break 'f 0i32;
        }
        l_ay_4 = { let a0_ = F32(c.m.r32((fp.wrapping_add(48)).wrapping_add(4))); rt::ffabs(a0_) };
        if (((rt::fgt({ let a0_ = F32(c.m.r32((fp.wrapping_add(32)).wrapping_add(4))); rt::ffabs(a0_) }, rt::fadd(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))), l_ay_4)) as i32)) != 0) {
            break 'f 0i32;
        }
        l_az_5 = { let a0_ = F32(c.m.r32((fp.wrapping_add(48)).wrapping_add(8))); rt::ffabs(a0_) };
        if (((rt::fgt({ let a0_ = F32(c.m.r32((fp.wrapping_add(32)).wrapping_add(8))); rt::ffabs(a0_) }, rt::fadd(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8))), l_az_5)) as i32)) != 0) {
            break 'f 0i32;
        }
        { let r_ = F32(0x03c80000); let n_ = rt::fadd(l_ax_3, r_); l_ax_3 = n_; };
        { let r_ = F32(0x03c80000); let n_ = rt::fadd(l_ay_4, r_); l_ay_4 = n_; };
        { let r_ = F32(0x03c80000); let n_ = rt::fadd(l_az_5, r_); l_az_5 = n_; };
        if (((rt::fgt({ let a0_ = rt::fsub(rt::fmul(F32(c.m.r32((fp.wrapping_add(32)).wrapping_add(4))), F32(c.m.r32((fp.wrapping_add(48)).wrapping_add(8)))), rt::fmul(F32(c.m.r32((fp.wrapping_add(32)).wrapping_add(8))), F32(c.m.r32((fp.wrapping_add(48)).wrapping_add(4))))); rt::ffabs(a0_) }, rt::fadd(rt::fmul(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))), l_az_5), rt::fmul(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8))), l_ay_4))) as i32)) != 0) {
            break 'f 0i32;
        }
        if (((rt::fgt({ let a0_ = rt::fsub(rt::fmul(F32(c.m.r32((fp.wrapping_add(32)).wrapping_add(8))), F32(c.m.r32(fp.wrapping_add(48)))), rt::fmul(F32(c.m.r32(fp.wrapping_add(32))), F32(c.m.r32((fp.wrapping_add(48)).wrapping_add(8))))); rt::ffabs(a0_) }, rt::fadd(rt::fmul(F32(c.m.r32(fp.wrapping_add(16))), l_az_5), rt::fmul(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8))), l_ax_3))) as i32)) != 0) {
            break 'f 0i32;
        }
        if (((rt::fgt({ let a0_ = rt::fsub(rt::fmul(F32(c.m.r32(fp.wrapping_add(32))), F32(c.m.r32((fp.wrapping_add(48)).wrapping_add(4)))), rt::fmul(F32(c.m.r32((fp.wrapping_add(32)).wrapping_add(4))), F32(c.m.r32(fp.wrapping_add(48))))); rt::ffabs(a0_) }, rt::fadd(rt::fmul(F32(c.m.r32(fp.wrapping_add(16))), l_ay_4), rt::fmul(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))), l_ax_3))) as i32)) != 0) {
            break 'f 0i32;
        }
        break 'f 1i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "ColBox_TestSegment", &PROBE_N, &[], true); }
    r_
}

pub fn ColNode_TestBox(c: &mut Ctx, mut p_node_1: u32, mut p_box_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ColNode_TestBox", &PROBE_N, &[(p_node_1) as u32, (p_box_2) as u32], false); }
    let r_ = 'f: {
    break 'f (((F::ColBox_Overlaps(c, p_box_2, p_node_1)) != (0i32)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "ColNode_TestBox", &PROBE_N, &[], true); }
    r_
}

pub fn ColMesh_CollectNode(c: &mut Ctx, mut p_nodes_1: u32, mut p_box_2: u32, mut p_idx_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ColMesh_CollectNode", &PROBE_N, &[(p_nodes_1) as u32, (p_box_2) as u32, (p_idx_3) as u32], false); }
    let mut l_node_4: u32 = 0;
    let r_ = 'f: {
    l_node_4 = (p_nodes_1).wrapping_add((((p_idx_3) as u32)).wrapping_mul(32));
    if ((F::ColNode_TestBox(c, l_node_4, p_box_2)) != 0) {
        if ((F::ColNode_IsLeaf(c, l_node_4)) != 0) {
            if (((((c.m.r32(sym::gColWalkBuf)) != (((0i32) as u32))) as i32)) != 0) {
                if ((((((c.m.r32(sym::gColWalkCount) as i32)) >= ((c.m.r32(sym::gColWalkMax) as i32))) as i32)) != 0) {
                    break 'f;
                }
                { let v_ = (((c.m.r32((l_node_4).wrapping_add(24)) as i32)) as u16); let a_ = (c.m.r32(sym::gColWalkBuf)).wrapping_add(((((c.m.r32(sym::gColWalkCount) as i32)) as u32)).wrapping_mul(2)); c.m.w16(a_, v_); };
            }
            { let a_ = sym::gColWalkCount; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        } else {
            { let a2_ = (c.m.r32((l_node_4).wrapping_add(24)) as i32); F::ColMesh_CollectNode(c, p_nodes_1, p_box_2, a2_) };
            { let a2_ = (c.m.r32((l_node_4).wrapping_add(28)) as i32); F::ColMesh_CollectNode(c, p_nodes_1, p_box_2, a2_) };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ColMesh_CollectNode", &PROBE_N, &[], true); }
    r_
}

pub fn ColMesh_CollectBox(c: &mut Ctx, mut p_max_1: i32, mut p_buf_2: u32, mut p_mesh_3: u32, mut p_box_4: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ColMesh_CollectBox", &PROBE_N, &[(p_max_1) as u32, (p_buf_2) as u32, (p_mesh_3) as u32, (p_box_4) as u32], false); }
    let mut l_nodes_5: u32 = 0;
    let r_ = 'f: {
    if ((((((c.m.r32(p_mesh_3) as i32)) <= (0i32)) as i32)) != 0) {
        break 'f 0i32;
    }
    l_nodes_5 = (c.m.r32(sym::gColMeshBase)).wrapping_add(((((c.m.r32((p_mesh_3).wrapping_add(12)) as i32)) as u32)).wrapping_mul(4));
    { let v_ = p_max_1; let a_ = sym::gColWalkMax; c.m.w32(a_, (v_) as u32); };
    { let v_ = p_buf_2; let a_ = sym::gColWalkBuf; c.m.w32(a_, v_); };
    { let v_ = 0i32; let a_ = sym::gColWalkCount; c.m.w32(a_, (v_) as u32); };
    F::ColMesh_CollectNode(c, l_nodes_5, p_box_4, 0i32);
    { let v_ = ((0i32) as u32); let a_ = sym::gColWalkBuf; c.m.w32(a_, v_); };
    break 'f (c.m.r32(sym::gColWalkCount) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "ColMesh_CollectBox", &PROBE_N, &[], true); }
    r_
}

pub fn ColMesh_WalkNode(c: &mut Ctx, mut p_nodes_1: u32, mut p_box_2: u32, mut p_idx_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ColMesh_WalkNode", &PROBE_N, &[(p_nodes_1) as u32, (p_box_2) as u32, (p_idx_3) as u32], false); }
    let mut l_node_4: u32 = 0;
    let mut l_ret_5: i32 = 0;
    let r_ = 'f: {
    l_node_4 = (p_nodes_1).wrapping_add((((p_idx_3) as u32)).wrapping_mul(32));
    if ((F::ColNode_TestBox(c, l_node_4, p_box_2)) != 0) {
        if ((F::ColNode_IsLeaf(c, l_node_4)) != 0) {
            l_ret_5 = { let fp_ = c.m.r32(sym::gColWalkCb); let a1_ = c.m.r32(sym::gColWalkCtx); ((D::call(c, fp_, &[((l_node_4) as u64), ((a1_) as u64)])) as i32) };
            { let r_ = l_ret_5; let a_ = sym::gColWalkCount; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            break 'f;
        }
        { let a2_ = (c.m.r32((l_node_4).wrapping_add(24)) as i32); F::ColMesh_WalkNode(c, p_nodes_1, p_box_2, a2_) };
        { let a2_ = (c.m.r32((l_node_4).wrapping_add(28)) as i32); F::ColMesh_WalkNode(c, p_nodes_1, p_box_2, a2_) };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ColMesh_WalkNode", &PROBE_N, &[], true); }
    r_
}

pub fn ColMesh_WalkBox(c: &mut Ctx, mut p_mesh_1: u32, mut p_box_2: u32, mut p_ctx_3: u32, mut p_cb_4: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ColMesh_WalkBox", &PROBE_N, &[(p_mesh_1) as u32, (p_box_2) as u32, (p_ctx_3) as u32, (p_cb_4) as u32], false); }
    let mut l_nodes_5: u32 = 0;
    let r_ = 'f: {
    if ((((((c.m.r32(p_mesh_1) as i32)) <= (0i32)) as i32)) != 0) {
        break 'f 0i32;
    }
    if (((((p_cb_4) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    l_nodes_5 = (c.m.r32(sym::gColMeshBase)).wrapping_add(((((c.m.r32((p_mesh_1).wrapping_add(12)) as i32)) as u32)).wrapping_mul(4));
    { let v_ = p_box_2; let a_ = sym::gColWalkBox; c.m.w32(a_, v_); };
    { let v_ = p_cb_4; let a_ = sym::gColWalkCb; c.m.w32(a_, v_); };
    { let v_ = p_ctx_3; let a_ = sym::gColWalkCtx; c.m.w32(a_, v_); };
    { let v_ = 0i32; let a_ = sym::gColWalkCount; c.m.w32(a_, (v_) as u32); };
    F::ColMesh_WalkNode(c, l_nodes_5, p_box_2, 0i32);
    break 'f (c.m.r32(sym::gColWalkCount) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "ColMesh_WalkBox", &PROBE_N, &[], true); }
    r_
}

pub fn Col_NearEq(c: &mut Ctx, mut p_a_1: F32, mut p_b_2: F32, mut p_eps_3: F32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Col_NearEq", &PROBE_N, &[(p_a_1).0, (p_b_2).0, (p_eps_3).0], false); }
    let r_ = 'f: {
    if (((rt::fle(rt::ffabs(rt::fsub(p_a_1, p_b_2)), p_eps_3) as i32)) != 0) {
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Col_NearEq", &PROBE_N, &[], true); }
    r_
}

pub fn Col_LineLineParams(c: &mut Ctx, mut p_s_1: u32, mut p_t_2: u32, mut p_p1_3: u32, mut p_d1_4: u32, mut p_p2_5: u32, mut p_d2_6: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Col_LineLineParams", &PROBE_N, &[(p_s_1) as u32, (p_t_2) as u32, (p_p1_3) as u32, (p_d1_4) as u32, (p_p2_5) as u32, (p_d2_6) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_a_7: F32 = F32(0);
    let mut l_b_8: F32 = F32(0);
    let mut l_c_9: F32 = F32(0);
    let mut l_den_10: F32 = F32(0);
    let mut l_inv_11: F32 = F32(0);
    let r_ = 'f: {
        if (({ let a0_ = F::Vec3_LengthSq(c, p_d1_4); F::Col_NearEq(c, a0_, F32(0x3f800000), F32(0x358637bd)) }) != 0) {
            { let v_ = F32(0x00000000); let a_ = p_s_1; c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x00000000); let a_ = p_t_2; c.m.w32(a_, (v_).0); };
            break 'f;
        }
        if (({ let a0_ = F::Vec3_LengthSq(c, p_d2_6); F::Col_NearEq(c, a0_, F32(0x3f800000), F32(0x358637bd)) }) != 0) {
            { let v_ = F32(0x00000000); let a_ = p_s_1; c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x00000000); let a_ = p_t_2; c.m.w32(a_, (v_).0); };
            break 'f;
        }
        F::Vec4_Sub(c, fp, p_p2_5, p_p1_3);
        l_a_7 = F::Vec3_Dot(c, fp, p_d1_4);
        l_b_8 = F::Vec3_Dot(c, fp, p_d2_6);
        l_c_9 = F::Vec3_Dot(c, p_d1_4, p_d2_6);
        l_den_10 = rt::fsub(rt::fmul(l_c_9, l_c_9), F32(0x3f800000));
        if (((rt::flt(l_den_10, F32(0xb58637bd)) as i32)) != 0) {
            l_inv_11 = rt::fdiv(F32(0x3f800000), l_den_10);
            { let v_ = rt::fmul(rt::fsub(rt::fmul(l_c_9, l_b_8), l_a_7), l_inv_11); let a_ = p_s_1; c.m.w32(a_, (v_).0); };
            { let v_ = rt::fmul(rt::fsub(l_b_8, rt::fmul(l_c_9, l_a_7)), l_inv_11); let a_ = p_t_2; c.m.w32(a_, (v_).0); };
        } else {
            { let v_ = F32(0x00000000); let a_ = p_s_1; c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x00000000); let a_ = p_t_2; c.m.w32(a_, (v_).0); };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Col_LineLineParams", &PROBE_N, &[], true); }
    r_
}

pub fn ColObb_Overlaps(c: &mut Ctx, mut p_a_1: u32, mut p_b_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ColObb_Overlaps", &PROBE_N, &[(p_a_1) as u32, (p_b_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(160) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 160);
    let mut l_t_3: F32 = F32(0);
    let mut l_rb_4: F32 = F32(0);
    let mut l_sum_5: F32 = F32(0);
    let mut l_ra_6: F32 = F32(0);
    let r_ = 'f: {
        F::Vec3_Copy(c, fp, (p_a_1).wrapping_add(16));
        F::Vec3_Copy(c, fp.wrapping_add(16), (p_b_2).wrapping_add(16));
        F::Vec4_Sub(c, fp.wrapping_add(32), p_a_1, p_b_2);
        { let v_ = F::Vec3_Dot(c, ((p_a_1).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), ((p_b_2).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(16))); let a_ = ((fp.wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(12))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F::Vec3_Dot(c, ((p_a_1).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), ((p_b_2).wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(16))); let a_ = ((fp.wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(12))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F::Vec3_Dot(c, ((p_a_1).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), ((p_b_2).wrapping_add(32)).wrapping_add((((2i32) as u32)).wrapping_mul(16))); let a_ = ((fp.wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(12))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F::Vec3_Dot(c, ((p_a_1).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), fp.wrapping_add(32)); let a_ = fp.wrapping_add(144); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32(((fp.wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(12))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); rt::ffabs(a0_) }; let a_ = ((fp.wrapping_add(96)).wrapping_add((((0i32) as u32)).wrapping_mul(12))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32(((fp.wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(12))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); rt::ffabs(a0_) }; let a_ = ((fp.wrapping_add(96)).wrapping_add((((0i32) as u32)).wrapping_mul(12))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32(((fp.wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(12))).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); rt::ffabs(a0_) }; let a_ = ((fp.wrapping_add(96)).wrapping_add((((0i32) as u32)).wrapping_mul(12))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        l_t_3 = { let a0_ = F32(c.m.r32(fp.wrapping_add(144))); rt::ffabs(a0_) };
        l_rb_4 = rt::fadd(rt::fadd(rt::fmul(F32(c.m.r32(fp.wrapping_add(16))), F32(c.m.r32(((fp.wrapping_add(96)).wrapping_add((((0i32) as u32)).wrapping_mul(12))).wrapping_add((((0i32) as u32)).wrapping_mul(4))))), rt::fmul(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))), F32(c.m.r32(((fp.wrapping_add(96)).wrapping_add((((0i32) as u32)).wrapping_mul(12))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))))), rt::fmul(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8))), F32(c.m.r32(((fp.wrapping_add(96)).wrapping_add((((0i32) as u32)).wrapping_mul(12))).wrapping_add((((2i32) as u32)).wrapping_mul(4))))));
        l_sum_5 = rt::fadd(F32(c.m.r32(fp)), l_rb_4);
        if (((rt::fgt(l_t_3, l_sum_5) as i32)) != 0) {
            break 'f 0i32;
        }
        { let v_ = F::Vec3_Dot(c, ((p_a_1).wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), ((p_b_2).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(16))); let a_ = ((fp.wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(12))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F::Vec3_Dot(c, ((p_a_1).wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), ((p_b_2).wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(16))); let a_ = ((fp.wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(12))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F::Vec3_Dot(c, ((p_a_1).wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), ((p_b_2).wrapping_add(32)).wrapping_add((((2i32) as u32)).wrapping_mul(16))); let a_ = ((fp.wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(12))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F::Vec3_Dot(c, ((p_a_1).wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), fp.wrapping_add(32)); let a_ = (fp.wrapping_add(144)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32(((fp.wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(12))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); rt::ffabs(a0_) }; let a_ = ((fp.wrapping_add(96)).wrapping_add((((1i32) as u32)).wrapping_mul(12))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32(((fp.wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(12))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); rt::ffabs(a0_) }; let a_ = ((fp.wrapping_add(96)).wrapping_add((((1i32) as u32)).wrapping_mul(12))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32(((fp.wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(12))).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); rt::ffabs(a0_) }; let a_ = ((fp.wrapping_add(96)).wrapping_add((((1i32) as u32)).wrapping_mul(12))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        l_t_3 = { let a0_ = F32(c.m.r32((fp.wrapping_add(144)).wrapping_add(4))); rt::ffabs(a0_) };
        l_rb_4 = rt::fadd(rt::fadd(rt::fmul(F32(c.m.r32(fp.wrapping_add(16))), F32(c.m.r32(((fp.wrapping_add(96)).wrapping_add((((1i32) as u32)).wrapping_mul(12))).wrapping_add((((0i32) as u32)).wrapping_mul(4))))), rt::fmul(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))), F32(c.m.r32(((fp.wrapping_add(96)).wrapping_add((((1i32) as u32)).wrapping_mul(12))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))))), rt::fmul(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8))), F32(c.m.r32(((fp.wrapping_add(96)).wrapping_add((((1i32) as u32)).wrapping_mul(12))).wrapping_add((((2i32) as u32)).wrapping_mul(4))))));
        l_sum_5 = rt::fadd(F32(c.m.r32((fp).wrapping_add(4))), l_rb_4);
        if (((rt::fgt(l_t_3, l_sum_5) as i32)) != 0) {
            break 'f 0i32;
        }
        { let v_ = F::Vec3_Dot(c, ((p_a_1).wrapping_add(32)).wrapping_add((((2i32) as u32)).wrapping_mul(16)), ((p_b_2).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(16))); let a_ = ((fp.wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(12))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F::Vec3_Dot(c, ((p_a_1).wrapping_add(32)).wrapping_add((((2i32) as u32)).wrapping_mul(16)), ((p_b_2).wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(16))); let a_ = ((fp.wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(12))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F::Vec3_Dot(c, ((p_a_1).wrapping_add(32)).wrapping_add((((2i32) as u32)).wrapping_mul(16)), ((p_b_2).wrapping_add(32)).wrapping_add((((2i32) as u32)).wrapping_mul(16))); let a_ = ((fp.wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(12))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F::Vec3_Dot(c, ((p_a_1).wrapping_add(32)).wrapping_add((((2i32) as u32)).wrapping_mul(16)), fp.wrapping_add(32)); let a_ = (fp.wrapping_add(144)).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32(((fp.wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(12))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); rt::ffabs(a0_) }; let a_ = ((fp.wrapping_add(96)).wrapping_add((((2i32) as u32)).wrapping_mul(12))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32(((fp.wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(12))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); rt::ffabs(a0_) }; let a_ = ((fp.wrapping_add(96)).wrapping_add((((2i32) as u32)).wrapping_mul(12))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32(((fp.wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(12))).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); rt::ffabs(a0_) }; let a_ = ((fp.wrapping_add(96)).wrapping_add((((2i32) as u32)).wrapping_mul(12))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        l_t_3 = { let a0_ = F32(c.m.r32((fp.wrapping_add(144)).wrapping_add(8))); rt::ffabs(a0_) };
        l_rb_4 = rt::fadd(rt::fadd(rt::fmul(F32(c.m.r32(fp.wrapping_add(16))), F32(c.m.r32(((fp.wrapping_add(96)).wrapping_add((((2i32) as u32)).wrapping_mul(12))).wrapping_add((((0i32) as u32)).wrapping_mul(4))))), rt::fmul(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))), F32(c.m.r32(((fp.wrapping_add(96)).wrapping_add((((2i32) as u32)).wrapping_mul(12))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))))), rt::fmul(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8))), F32(c.m.r32(((fp.wrapping_add(96)).wrapping_add((((2i32) as u32)).wrapping_mul(12))).wrapping_add((((2i32) as u32)).wrapping_mul(4))))));
        l_sum_5 = rt::fadd(F32(c.m.r32((fp).wrapping_add(8))), l_rb_4);
        if (((rt::fgt(l_t_3, l_sum_5) as i32)) != 0) {
            break 'f 0i32;
        }
        l_t_3 = { let a0_ = F::Vec3_Dot(c, ((p_b_2).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(16)), fp.wrapping_add(32)); rt::ffabs(a0_) };
        l_ra_6 = rt::fadd(rt::fadd(rt::fmul(F32(c.m.r32(fp)), F32(c.m.r32(((fp.wrapping_add(96)).wrapping_add((((0i32) as u32)).wrapping_mul(12))).wrapping_add((((0i32) as u32)).wrapping_mul(4))))), rt::fmul(F32(c.m.r32((fp).wrapping_add(4))), F32(c.m.r32(((fp.wrapping_add(96)).wrapping_add((((1i32) as u32)).wrapping_mul(12))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))))), rt::fmul(F32(c.m.r32((fp).wrapping_add(8))), F32(c.m.r32(((fp.wrapping_add(96)).wrapping_add((((2i32) as u32)).wrapping_mul(12))).wrapping_add((((0i32) as u32)).wrapping_mul(4))))));
        l_sum_5 = rt::fadd(l_ra_6, F32(c.m.r32(fp.wrapping_add(16))));
        if (((rt::fgt(l_t_3, l_sum_5) as i32)) != 0) {
            break 'f 0i32;
        }
        l_t_3 = { let a0_ = F::Vec3_Dot(c, ((p_b_2).wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(16)), fp.wrapping_add(32)); rt::ffabs(a0_) };
        l_ra_6 = rt::fadd(rt::fadd(rt::fmul(F32(c.m.r32(fp)), F32(c.m.r32(((fp.wrapping_add(96)).wrapping_add((((0i32) as u32)).wrapping_mul(12))).wrapping_add((((1i32) as u32)).wrapping_mul(4))))), rt::fmul(F32(c.m.r32((fp).wrapping_add(4))), F32(c.m.r32(((fp.wrapping_add(96)).wrapping_add((((1i32) as u32)).wrapping_mul(12))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))))), rt::fmul(F32(c.m.r32((fp).wrapping_add(8))), F32(c.m.r32(((fp.wrapping_add(96)).wrapping_add((((2i32) as u32)).wrapping_mul(12))).wrapping_add((((1i32) as u32)).wrapping_mul(4))))));
        l_sum_5 = rt::fadd(l_ra_6, F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))));
        if (((rt::fgt(l_t_3, l_sum_5) as i32)) != 0) {
            break 'f 0i32;
        }
        l_t_3 = { let a0_ = F::Vec3_Dot(c, ((p_b_2).wrapping_add(32)).wrapping_add((((2i32) as u32)).wrapping_mul(16)), fp.wrapping_add(32)); rt::ffabs(a0_) };
        l_ra_6 = rt::fadd(rt::fadd(rt::fmul(F32(c.m.r32(fp)), F32(c.m.r32(((fp.wrapping_add(96)).wrapping_add((((0i32) as u32)).wrapping_mul(12))).wrapping_add((((2i32) as u32)).wrapping_mul(4))))), rt::fmul(F32(c.m.r32((fp).wrapping_add(4))), F32(c.m.r32(((fp.wrapping_add(96)).wrapping_add((((1i32) as u32)).wrapping_mul(12))).wrapping_add((((2i32) as u32)).wrapping_mul(4)))))), rt::fmul(F32(c.m.r32((fp).wrapping_add(8))), F32(c.m.r32(((fp.wrapping_add(96)).wrapping_add((((2i32) as u32)).wrapping_mul(12))).wrapping_add((((2i32) as u32)).wrapping_mul(4))))));
        l_sum_5 = rt::fadd(l_ra_6, F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8))));
        if (((rt::fgt(l_t_3, l_sum_5) as i32)) != 0) {
            break 'f 0i32;
        }
        l_t_3 = { let a0_ = rt::fsub(rt::fmul(F32(c.m.r32((fp.wrapping_add(144)).wrapping_add(8))), F32(c.m.r32(((fp.wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(12))).wrapping_add((((0i32) as u32)).wrapping_mul(4))))), rt::fmul(F32(c.m.r32((fp.wrapping_add(144)).wrapping_add(4))), F32(c.m.r32(((fp.wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(12))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))))); rt::ffabs(a0_) };
        l_ra_6 = rt::fadd(rt::fmul(F32(c.m.r32((fp).wrapping_add(4))), F32(c.m.r32(((fp.wrapping_add(96)).wrapping_add((((2i32) as u32)).wrapping_mul(12))).wrapping_add((((0i32) as u32)).wrapping_mul(4))))), rt::fmul(F32(c.m.r32((fp).wrapping_add(8))), F32(c.m.r32(((fp.wrapping_add(96)).wrapping_add((((1i32) as u32)).wrapping_mul(12))).wrapping_add((((0i32) as u32)).wrapping_mul(4))))));
        l_rb_4 = rt::fadd(rt::fmul(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))), F32(c.m.r32(((fp.wrapping_add(96)).wrapping_add((((0i32) as u32)).wrapping_mul(12))).wrapping_add((((2i32) as u32)).wrapping_mul(4))))), rt::fmul(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8))), F32(c.m.r32(((fp.wrapping_add(96)).wrapping_add((((0i32) as u32)).wrapping_mul(12))).wrapping_add((((1i32) as u32)).wrapping_mul(4))))));
        l_sum_5 = rt::fadd(l_ra_6, l_rb_4);
        if (((rt::fgt(l_t_3, l_sum_5) as i32)) != 0) {
            break 'f 0i32;
        }
        l_t_3 = { let a0_ = rt::fsub(rt::fmul(F32(c.m.r32((fp.wrapping_add(144)).wrapping_add(8))), F32(c.m.r32(((fp.wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(12))).wrapping_add((((1i32) as u32)).wrapping_mul(4))))), rt::fmul(F32(c.m.r32((fp.wrapping_add(144)).wrapping_add(4))), F32(c.m.r32(((fp.wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(12))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))))); rt::ffabs(a0_) };
        l_ra_6 = rt::fadd(rt::fmul(F32(c.m.r32((fp).wrapping_add(4))), F32(c.m.r32(((fp.wrapping_add(96)).wrapping_add((((2i32) as u32)).wrapping_mul(12))).wrapping_add((((1i32) as u32)).wrapping_mul(4))))), rt::fmul(F32(c.m.r32((fp).wrapping_add(8))), F32(c.m.r32(((fp.wrapping_add(96)).wrapping_add((((1i32) as u32)).wrapping_mul(12))).wrapping_add((((1i32) as u32)).wrapping_mul(4))))));
        l_rb_4 = rt::fadd(rt::fmul(F32(c.m.r32(fp.wrapping_add(16))), F32(c.m.r32(((fp.wrapping_add(96)).wrapping_add((((0i32) as u32)).wrapping_mul(12))).wrapping_add((((2i32) as u32)).wrapping_mul(4))))), rt::fmul(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8))), F32(c.m.r32(((fp.wrapping_add(96)).wrapping_add((((0i32) as u32)).wrapping_mul(12))).wrapping_add((((0i32) as u32)).wrapping_mul(4))))));
        l_sum_5 = rt::fadd(l_ra_6, l_rb_4);
        if (((rt::fgt(l_t_3, l_sum_5) as i32)) != 0) {
            break 'f 0i32;
        }
        l_t_3 = { let a0_ = rt::fsub(rt::fmul(F32(c.m.r32((fp.wrapping_add(144)).wrapping_add(8))), F32(c.m.r32(((fp.wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(12))).wrapping_add((((2i32) as u32)).wrapping_mul(4))))), rt::fmul(F32(c.m.r32((fp.wrapping_add(144)).wrapping_add(4))), F32(c.m.r32(((fp.wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(12))).wrapping_add((((2i32) as u32)).wrapping_mul(4)))))); rt::ffabs(a0_) };
        l_ra_6 = rt::fadd(rt::fmul(F32(c.m.r32((fp).wrapping_add(4))), F32(c.m.r32(((fp.wrapping_add(96)).wrapping_add((((2i32) as u32)).wrapping_mul(12))).wrapping_add((((2i32) as u32)).wrapping_mul(4))))), rt::fmul(F32(c.m.r32((fp).wrapping_add(8))), F32(c.m.r32(((fp.wrapping_add(96)).wrapping_add((((1i32) as u32)).wrapping_mul(12))).wrapping_add((((2i32) as u32)).wrapping_mul(4))))));
        l_rb_4 = rt::fadd(rt::fmul(F32(c.m.r32(fp.wrapping_add(16))), F32(c.m.r32(((fp.wrapping_add(96)).wrapping_add((((0i32) as u32)).wrapping_mul(12))).wrapping_add((((1i32) as u32)).wrapping_mul(4))))), rt::fmul(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))), F32(c.m.r32(((fp.wrapping_add(96)).wrapping_add((((0i32) as u32)).wrapping_mul(12))).wrapping_add((((0i32) as u32)).wrapping_mul(4))))));
        l_sum_5 = rt::fadd(l_ra_6, l_rb_4);
        if (((rt::fgt(l_t_3, l_sum_5) as i32)) != 0) {
            break 'f 0i32;
        }
        l_t_3 = { let a0_ = rt::fsub(rt::fmul(F32(c.m.r32(fp.wrapping_add(144))), F32(c.m.r32(((fp.wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(12))).wrapping_add((((0i32) as u32)).wrapping_mul(4))))), rt::fmul(F32(c.m.r32((fp.wrapping_add(144)).wrapping_add(8))), F32(c.m.r32(((fp.wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(12))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))))); rt::ffabs(a0_) };
        l_ra_6 = rt::fadd(rt::fmul(F32(c.m.r32(fp)), F32(c.m.r32(((fp.wrapping_add(96)).wrapping_add((((2i32) as u32)).wrapping_mul(12))).wrapping_add((((0i32) as u32)).wrapping_mul(4))))), rt::fmul(F32(c.m.r32((fp).wrapping_add(8))), F32(c.m.r32(((fp.wrapping_add(96)).wrapping_add((((0i32) as u32)).wrapping_mul(12))).wrapping_add((((0i32) as u32)).wrapping_mul(4))))));
        l_rb_4 = rt::fadd(rt::fmul(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))), F32(c.m.r32(((fp.wrapping_add(96)).wrapping_add((((1i32) as u32)).wrapping_mul(12))).wrapping_add((((2i32) as u32)).wrapping_mul(4))))), rt::fmul(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8))), F32(c.m.r32(((fp.wrapping_add(96)).wrapping_add((((1i32) as u32)).wrapping_mul(12))).wrapping_add((((1i32) as u32)).wrapping_mul(4))))));
        l_sum_5 = rt::fadd(l_ra_6, l_rb_4);
        if (((rt::fgt(l_t_3, l_sum_5) as i32)) != 0) {
            break 'f 0i32;
        }
        l_t_3 = { let a0_ = rt::fsub(rt::fmul(F32(c.m.r32(fp.wrapping_add(144))), F32(c.m.r32(((fp.wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(12))).wrapping_add((((1i32) as u32)).wrapping_mul(4))))), rt::fmul(F32(c.m.r32((fp.wrapping_add(144)).wrapping_add(8))), F32(c.m.r32(((fp.wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(12))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))))); rt::ffabs(a0_) };
        l_ra_6 = rt::fadd(rt::fmul(F32(c.m.r32(fp)), F32(c.m.r32(((fp.wrapping_add(96)).wrapping_add((((2i32) as u32)).wrapping_mul(12))).wrapping_add((((1i32) as u32)).wrapping_mul(4))))), rt::fmul(F32(c.m.r32((fp).wrapping_add(8))), F32(c.m.r32(((fp.wrapping_add(96)).wrapping_add((((0i32) as u32)).wrapping_mul(12))).wrapping_add((((1i32) as u32)).wrapping_mul(4))))));
        l_rb_4 = rt::fadd(rt::fmul(F32(c.m.r32(fp.wrapping_add(16))), F32(c.m.r32(((fp.wrapping_add(96)).wrapping_add((((1i32) as u32)).wrapping_mul(12))).wrapping_add((((2i32) as u32)).wrapping_mul(4))))), rt::fmul(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8))), F32(c.m.r32(((fp.wrapping_add(96)).wrapping_add((((1i32) as u32)).wrapping_mul(12))).wrapping_add((((0i32) as u32)).wrapping_mul(4))))));
        l_sum_5 = rt::fadd(l_ra_6, l_rb_4);
        if (((rt::fgt(l_t_3, l_sum_5) as i32)) != 0) {
            break 'f 0i32;
        }
        l_t_3 = { let a0_ = rt::fsub(rt::fmul(F32(c.m.r32(fp.wrapping_add(144))), F32(c.m.r32(((fp.wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(12))).wrapping_add((((2i32) as u32)).wrapping_mul(4))))), rt::fmul(F32(c.m.r32((fp.wrapping_add(144)).wrapping_add(8))), F32(c.m.r32(((fp.wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(12))).wrapping_add((((2i32) as u32)).wrapping_mul(4)))))); rt::ffabs(a0_) };
        l_ra_6 = rt::fadd(rt::fmul(F32(c.m.r32(fp)), F32(c.m.r32(((fp.wrapping_add(96)).wrapping_add((((2i32) as u32)).wrapping_mul(12))).wrapping_add((((2i32) as u32)).wrapping_mul(4))))), rt::fmul(F32(c.m.r32((fp).wrapping_add(8))), F32(c.m.r32(((fp.wrapping_add(96)).wrapping_add((((0i32) as u32)).wrapping_mul(12))).wrapping_add((((2i32) as u32)).wrapping_mul(4))))));
        l_rb_4 = rt::fadd(rt::fmul(F32(c.m.r32(fp.wrapping_add(16))), F32(c.m.r32(((fp.wrapping_add(96)).wrapping_add((((1i32) as u32)).wrapping_mul(12))).wrapping_add((((1i32) as u32)).wrapping_mul(4))))), rt::fmul(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))), F32(c.m.r32(((fp.wrapping_add(96)).wrapping_add((((1i32) as u32)).wrapping_mul(12))).wrapping_add((((0i32) as u32)).wrapping_mul(4))))));
        l_sum_5 = rt::fadd(l_ra_6, l_rb_4);
        if (((rt::fgt(l_t_3, l_sum_5) as i32)) != 0) {
            break 'f 0i32;
        }
        l_t_3 = { let a0_ = rt::fsub(rt::fmul(F32(c.m.r32((fp.wrapping_add(144)).wrapping_add(4))), F32(c.m.r32(((fp.wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(12))).wrapping_add((((0i32) as u32)).wrapping_mul(4))))), rt::fmul(F32(c.m.r32(fp.wrapping_add(144))), F32(c.m.r32(((fp.wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(12))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))))); rt::ffabs(a0_) };
        l_ra_6 = rt::fadd(rt::fmul(F32(c.m.r32(fp)), F32(c.m.r32(((fp.wrapping_add(96)).wrapping_add((((1i32) as u32)).wrapping_mul(12))).wrapping_add((((0i32) as u32)).wrapping_mul(4))))), rt::fmul(F32(c.m.r32((fp).wrapping_add(4))), F32(c.m.r32(((fp.wrapping_add(96)).wrapping_add((((0i32) as u32)).wrapping_mul(12))).wrapping_add((((0i32) as u32)).wrapping_mul(4))))));
        l_rb_4 = rt::fadd(rt::fmul(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))), F32(c.m.r32(((fp.wrapping_add(96)).wrapping_add((((2i32) as u32)).wrapping_mul(12))).wrapping_add((((2i32) as u32)).wrapping_mul(4))))), rt::fmul(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8))), F32(c.m.r32(((fp.wrapping_add(96)).wrapping_add((((2i32) as u32)).wrapping_mul(12))).wrapping_add((((1i32) as u32)).wrapping_mul(4))))));
        l_sum_5 = rt::fadd(l_ra_6, l_rb_4);
        if (((rt::fgt(l_t_3, l_sum_5) as i32)) != 0) {
            break 'f 0i32;
        }
        l_t_3 = { let a0_ = rt::fsub(rt::fmul(F32(c.m.r32((fp.wrapping_add(144)).wrapping_add(4))), F32(c.m.r32(((fp.wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(12))).wrapping_add((((1i32) as u32)).wrapping_mul(4))))), rt::fmul(F32(c.m.r32(fp.wrapping_add(144))), F32(c.m.r32(((fp.wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(12))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))))); rt::ffabs(a0_) };
        l_ra_6 = rt::fadd(rt::fmul(F32(c.m.r32(fp)), F32(c.m.r32(((fp.wrapping_add(96)).wrapping_add((((1i32) as u32)).wrapping_mul(12))).wrapping_add((((1i32) as u32)).wrapping_mul(4))))), rt::fmul(F32(c.m.r32((fp).wrapping_add(4))), F32(c.m.r32(((fp.wrapping_add(96)).wrapping_add((((0i32) as u32)).wrapping_mul(12))).wrapping_add((((1i32) as u32)).wrapping_mul(4))))));
        l_rb_4 = rt::fadd(rt::fmul(F32(c.m.r32(fp.wrapping_add(16))), F32(c.m.r32(((fp.wrapping_add(96)).wrapping_add((((2i32) as u32)).wrapping_mul(12))).wrapping_add((((2i32) as u32)).wrapping_mul(4))))), rt::fmul(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8))), F32(c.m.r32(((fp.wrapping_add(96)).wrapping_add((((2i32) as u32)).wrapping_mul(12))).wrapping_add((((0i32) as u32)).wrapping_mul(4))))));
        l_sum_5 = rt::fadd(l_ra_6, l_rb_4);
        if (((rt::fgt(l_t_3, l_sum_5) as i32)) != 0) {
            break 'f 0i32;
        }
        l_t_3 = { let a0_ = rt::fsub(rt::fmul(F32(c.m.r32((fp.wrapping_add(144)).wrapping_add(4))), F32(c.m.r32(((fp.wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(12))).wrapping_add((((2i32) as u32)).wrapping_mul(4))))), rt::fmul(F32(c.m.r32(fp.wrapping_add(144))), F32(c.m.r32(((fp.wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(12))).wrapping_add((((2i32) as u32)).wrapping_mul(4)))))); rt::ffabs(a0_) };
        l_ra_6 = rt::fadd(rt::fmul(F32(c.m.r32(fp)), F32(c.m.r32(((fp.wrapping_add(96)).wrapping_add((((1i32) as u32)).wrapping_mul(12))).wrapping_add((((2i32) as u32)).wrapping_mul(4))))), rt::fmul(F32(c.m.r32((fp).wrapping_add(4))), F32(c.m.r32(((fp.wrapping_add(96)).wrapping_add((((0i32) as u32)).wrapping_mul(12))).wrapping_add((((2i32) as u32)).wrapping_mul(4))))));
        l_rb_4 = rt::fadd(rt::fmul(F32(c.m.r32(fp.wrapping_add(16))), F32(c.m.r32(((fp.wrapping_add(96)).wrapping_add((((2i32) as u32)).wrapping_mul(12))).wrapping_add((((1i32) as u32)).wrapping_mul(4))))), rt::fmul(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))), F32(c.m.r32(((fp.wrapping_add(96)).wrapping_add((((2i32) as u32)).wrapping_mul(12))).wrapping_add((((0i32) as u32)).wrapping_mul(4))))));
        l_sum_5 = rt::fadd(l_ra_6, l_rb_4);
        if (((rt::fgt(l_t_3, l_sum_5) as i32)) != 0) {
            break 'f 0i32;
        }
        break 'f 1i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "ColObb_Overlaps", &PROBE_N, &[], true); }
    r_
}

pub fn ColObb_Contact(c: &mut Ctx, mut p_a_1: u32, mut p_b_2: u32, mut p_normal_3: u32, mut p_point_4: u32, mut p_depth_5: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ColObb_Contact", &PROBE_N, &[(p_a_1) as u32, (p_b_2) as u32, (p_normal_3) as u32, (p_point_4) as u32, (p_depth_5) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(224) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 224);
    let mut l_t_6: F32 = F32(0);
    let mut l_ra_7: F32 = F32(0);
    let mut l_rb_8: F32 = F32(0);
    let mut l_sum_9: F32 = F32(0);
    let mut l_d_10: F32 = F32(0);
    let mut l_len_11: F32 = F32(0);
    let mut l_w_12: F32 = F32(0);
    let mut l_ha_13: u32 = 0;
    let mut l_hb_14: u32 = 0;
    let mut l_code_15: i32 = 0;
    let mut l_i_16: i32 = 0;
    let mut l_j_17: i32 = 0;
    let mut l_i1_18: i32 = 0;
    let mut l_i2_19: i32 = 0;
    let mut l_j1_20: i32 = 0;
    let mut l_j2_21: i32 = 0;
    let mut l_k_22: i32 = 0;
    let mut l_m_23: i32 = 0;
    let mut l_eq_24: i32 = 0;
    let mut l_sa1_25: F32 = F32(0);
    let mut l_sa2_26: F32 = F32(0);
    let mut l_sb1_27: F32 = F32(0);
    let mut l_sb2_28: F32 = F32(0);
    let r_ = 'f: {
        l_ha_13 = (p_a_1).wrapping_add(16);
        l_hb_14 = (p_b_2).wrapping_add(16);
        l_code_15 = (1i32).wrapping_neg();
        { let v_ = F32(c.m.r32((sym::D_002FEBA8).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = p_depth_5; c.m.w32(a_, (v_).0); };
        F::Vec4_Sub(c, fp, p_b_2, p_a_1);
        {
            l_i_16 = 0i32;
            'l1: loop {
                if !(((((l_i_16) < (3i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let v_ = F::Vec3_Dot(c, ((p_a_1).wrapping_add(32)).wrapping_add((((l_i_16) as u32)).wrapping_mul(16)), ((p_b_2).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(16))); let a_ = ((fp.wrapping_add(16)).wrapping_add((((l_i_16) as u32)).wrapping_mul(12))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    { let v_ = F::Vec3_Dot(c, ((p_a_1).wrapping_add(32)).wrapping_add((((l_i_16) as u32)).wrapping_mul(16)), ((p_b_2).wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(16))); let a_ = ((fp.wrapping_add(16)).wrapping_add((((l_i_16) as u32)).wrapping_mul(12))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    { let v_ = F::Vec3_Dot(c, ((p_a_1).wrapping_add(32)).wrapping_add((((l_i_16) as u32)).wrapping_mul(16)), ((p_b_2).wrapping_add(32)).wrapping_add((((2i32) as u32)).wrapping_mul(16))); let a_ = ((fp.wrapping_add(16)).wrapping_add((((l_i_16) as u32)).wrapping_mul(12))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    { let v_ = F::Vec3_Dot(c, ((p_a_1).wrapping_add(32)).wrapping_add((((l_i_16) as u32)).wrapping_mul(16)), fp); let a_ = (fp.wrapping_add(112)).wrapping_add((((l_i_16) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    { let v_ = { let a0_ = F32(c.m.r32(((fp.wrapping_add(16)).wrapping_add((((l_i_16) as u32)).wrapping_mul(12))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); rt::ffabs(a0_) }; let a_ = ((fp.wrapping_add(64)).wrapping_add((((l_i_16) as u32)).wrapping_mul(12))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    { let v_ = { let a0_ = F32(c.m.r32(((fp.wrapping_add(16)).wrapping_add((((l_i_16) as u32)).wrapping_mul(12))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); rt::ffabs(a0_) }; let a_ = ((fp.wrapping_add(64)).wrapping_add((((l_i_16) as u32)).wrapping_mul(12))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    { let v_ = { let a0_ = F32(c.m.r32(((fp.wrapping_add(16)).wrapping_add((((l_i_16) as u32)).wrapping_mul(12))).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); rt::ffabs(a0_) }; let a_ = ((fp.wrapping_add(64)).wrapping_add((((l_i_16) as u32)).wrapping_mul(12))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    l_t_6 = { let a0_ = F32(c.m.r32((fp.wrapping_add(112)).wrapping_add((((l_i_16) as u32)).wrapping_mul(4)))); rt::ffabs(a0_) };
                    l_rb_8 = rt::fadd(rt::fadd(rt::fmul(F32(c.m.r32((l_hb_14).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((fp.wrapping_add(64)).wrapping_add((((l_i_16) as u32)).wrapping_mul(12))).wrapping_add((((0i32) as u32)).wrapping_mul(4))))), rt::fmul(F32(c.m.r32((l_hb_14).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((fp.wrapping_add(64)).wrapping_add((((l_i_16) as u32)).wrapping_mul(12))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))))), rt::fmul(F32(c.m.r32((l_hb_14).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((fp.wrapping_add(64)).wrapping_add((((l_i_16) as u32)).wrapping_mul(12))).wrapping_add((((2i32) as u32)).wrapping_mul(4))))));
                    l_sum_9 = rt::fadd(F32(c.m.r32((l_ha_13).wrapping_add((((l_i_16) as u32)).wrapping_mul(4)))), l_rb_8);
                    l_d_10 = rt::fsub(l_sum_9, l_t_6);
                    if (((rt::flt(l_d_10, F32(0x00000000)) as i32)) != 0) {
                        break 'f 0i32;
                    }
                    if (((rt::flt(l_d_10, F32(c.m.r32(p_depth_5))) as i32)) != 0) {
                        { let v_ = l_d_10; let a_ = p_depth_5; c.m.w32(a_, (v_).0); };
                        l_code_15 = l_i_16;
                    }
                }
                { let o_ = l_i_16; let n_ = (o_).wrapping_add(1); l_i_16 = n_; };
            }
        }
        {
            l_j_17 = 0i32;
            'l3: loop {
                if !(((((l_j_17) < (3i32)) as i32)) != 0) { break 'l3; }
                'c4: {
                    { let v_ = F::Vec3_Dot(c, ((p_b_2).wrapping_add(32)).wrapping_add((((l_j_17) as u32)).wrapping_mul(16)), fp); let a_ = (fp.wrapping_add(128)).wrapping_add((((l_j_17) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    l_t_6 = { let a0_ = F32(c.m.r32((fp.wrapping_add(128)).wrapping_add((((l_j_17) as u32)).wrapping_mul(4)))); rt::ffabs(a0_) };
                    l_ra_7 = rt::fadd(rt::fadd(rt::fmul(F32(c.m.r32((l_ha_13).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((fp.wrapping_add(64)).wrapping_add((((0i32) as u32)).wrapping_mul(12))).wrapping_add((((l_j_17) as u32)).wrapping_mul(4))))), rt::fmul(F32(c.m.r32((l_ha_13).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((fp.wrapping_add(64)).wrapping_add((((1i32) as u32)).wrapping_mul(12))).wrapping_add((((l_j_17) as u32)).wrapping_mul(4)))))), rt::fmul(F32(c.m.r32((l_ha_13).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((fp.wrapping_add(64)).wrapping_add((((2i32) as u32)).wrapping_mul(12))).wrapping_add((((l_j_17) as u32)).wrapping_mul(4))))));
                    l_sum_9 = rt::fadd(l_ra_7, F32(c.m.r32((l_hb_14).wrapping_add((((l_j_17) as u32)).wrapping_mul(4)))));
                    l_d_10 = rt::fsub(l_sum_9, l_t_6);
                    if (((rt::flt(l_d_10, F32(0x00000000)) as i32)) != 0) {
                        break 'f 0i32;
                    }
                    if (((rt::flt(l_d_10, F32(c.m.r32(p_depth_5))) as i32)) != 0) {
                        { let v_ = l_d_10; let a_ = p_depth_5; c.m.w32(a_, (v_).0); };
                        l_code_15 = (l_j_17).wrapping_add(3i32);
                    }
                }
                { let o_ = l_j_17; let n_ = (o_).wrapping_add(1); l_j_17 = n_; };
            }
        }
        {
            l_i_16 = 0i32;
            'l5: loop {
                if !(((((l_i_16) < (3i32)) as i32)) != 0) { break 'l5; }
                'c6: {
                    l_i1_18 = (if (((((l_i_16) == (0i32)) as i32)) != 0) { 1i32 } else { 0i32 });
                    l_i2_19 = (if (((((l_i_16) == (2i32)) as i32)) != 0) { 1i32 } else { 2i32 });
                    {
                        l_j_17 = 0i32;
                        'l7: loop {
                            if !(((((l_j_17) < (3i32)) as i32)) != 0) { break 'l7; }
                            'c8: {
                                l_j1_20 = (if (((((l_j_17) == (0i32)) as i32)) != 0) { 1i32 } else { 0i32 });
                                l_j2_21 = (if (((((l_j_17) == (2i32)) as i32)) != 0) { 1i32 } else { 2i32 });
                                if (((((((((l_i_16) == (0i32)) as i32)) != 0) && (((((l_j_17) == (0i32)) as i32)) != 0)) as i32)) != 0) {
                                    l_len_11 = { let a0_ = rt::fadd(rt::fmul(F32(c.m.r32(((fp.wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(12))).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((fp.wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(12))).wrapping_add((((2i32) as u32)).wrapping_mul(4))))), rt::fmul(F32(c.m.r32(((fp.wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(12))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((fp.wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(12))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))))); F::Port_sqrtf(c, a0_) };
                                } else {
                                    l_len_11 = { let a0_ = rt::fadd(rt::fmul(F32(c.m.r32(((fp.wrapping_add(16)).wrapping_add((((l_i2_19) as u32)).wrapping_mul(12))).wrapping_add((((l_j_17) as u32)).wrapping_mul(4)))), F32(c.m.r32(((fp.wrapping_add(16)).wrapping_add((((l_i2_19) as u32)).wrapping_mul(12))).wrapping_add((((l_j_17) as u32)).wrapping_mul(4))))), rt::fmul(F32(c.m.r32(((fp.wrapping_add(16)).wrapping_add((((l_i1_18) as u32)).wrapping_mul(12))).wrapping_add((((l_j_17) as u32)).wrapping_mul(4)))), F32(c.m.r32(((fp.wrapping_add(16)).wrapping_add((((l_i1_18) as u32)).wrapping_mul(12))).wrapping_add((((l_j_17) as u32)).wrapping_mul(4)))))); F::Port_sqrtf(c, a0_) };
                                }
                                l_t_6 = { let a0_ = rt::fsub(rt::fmul(F32(c.m.r32((fp.wrapping_add(112)).wrapping_add((((l_i2_19) as u32)).wrapping_mul(4)))), F32(c.m.r32(((fp.wrapping_add(16)).wrapping_add((((l_i1_18) as u32)).wrapping_mul(12))).wrapping_add((((l_j_17) as u32)).wrapping_mul(4))))), rt::fmul(F32(c.m.r32((fp.wrapping_add(112)).wrapping_add((((l_i1_18) as u32)).wrapping_mul(4)))), F32(c.m.r32(((fp.wrapping_add(16)).wrapping_add((((l_i2_19) as u32)).wrapping_mul(12))).wrapping_add((((l_j_17) as u32)).wrapping_mul(4)))))); rt::ffabs(a0_) };
                                l_eq_24 = { let a0_ = { let a0_ = rt::fsub(rt::fmul(F32(c.m.r32((fp.wrapping_add(128)).wrapping_add((((l_j1_20) as u32)).wrapping_mul(4)))), F32(c.m.r32(((fp.wrapping_add(16)).wrapping_add((((l_i_16) as u32)).wrapping_mul(12))).wrapping_add((((l_j2_21) as u32)).wrapping_mul(4))))), rt::fmul(F32(c.m.r32((fp.wrapping_add(128)).wrapping_add((((l_j2_21) as u32)).wrapping_mul(4)))), F32(c.m.r32(((fp.wrapping_add(16)).wrapping_add((((l_i_16) as u32)).wrapping_mul(12))).wrapping_add((((l_j1_20) as u32)).wrapping_mul(4)))))); rt::ffabs(a0_) }; F::Col_NearEq(c, a0_, l_t_6, F32(0x358637bd)) };
                                { let r_ = { let a1_ = { let a0_ = rt::fadd(rt::fmul(F32(c.m.r32(((fp.wrapping_add(16)).wrapping_add((((l_i_16) as u32)).wrapping_mul(12))).wrapping_add((((l_j2_21) as u32)).wrapping_mul(4)))), F32(c.m.r32(((fp.wrapping_add(16)).wrapping_add((((l_i_16) as u32)).wrapping_mul(12))).wrapping_add((((l_j2_21) as u32)).wrapping_mul(4))))), rt::fmul(F32(c.m.r32(((fp.wrapping_add(16)).wrapping_add((((l_i_16) as u32)).wrapping_mul(12))).wrapping_add((((l_j1_20) as u32)).wrapping_mul(4)))), F32(c.m.r32(((fp.wrapping_add(16)).wrapping_add((((l_i_16) as u32)).wrapping_mul(12))).wrapping_add((((l_j1_20) as u32)).wrapping_mul(4)))))); F::Port_sqrtf(c, a0_) }; F::Col_NearEq(c, l_len_11, a1_, F32(0x358637bd)) }; let n_ = ((l_eq_24) | (r_)); l_eq_24 = n_; };
                                if (((((l_eq_24) == (0i32)) as i32)) != 0) {
                                    l_ra_7 = rt::fadd(rt::fmul(F32(c.m.r32((l_ha_13).wrapping_add((((l_i1_18) as u32)).wrapping_mul(4)))), F32(c.m.r32(((fp.wrapping_add(64)).wrapping_add((((l_i2_19) as u32)).wrapping_mul(12))).wrapping_add((((l_j_17) as u32)).wrapping_mul(4))))), rt::fmul(F32(c.m.r32((l_ha_13).wrapping_add((((l_i2_19) as u32)).wrapping_mul(4)))), F32(c.m.r32(((fp.wrapping_add(64)).wrapping_add((((l_i1_18) as u32)).wrapping_mul(12))).wrapping_add((((l_j_17) as u32)).wrapping_mul(4))))));
                                    l_rb_8 = rt::fadd(rt::fmul(F32(c.m.r32((l_hb_14).wrapping_add((((l_j1_20) as u32)).wrapping_mul(4)))), F32(c.m.r32(((fp.wrapping_add(64)).wrapping_add((((l_i_16) as u32)).wrapping_mul(12))).wrapping_add((((l_j2_21) as u32)).wrapping_mul(4))))), rt::fmul(F32(c.m.r32((l_hb_14).wrapping_add((((l_j2_21) as u32)).wrapping_mul(4)))), F32(c.m.r32(((fp.wrapping_add(64)).wrapping_add((((l_i_16) as u32)).wrapping_mul(12))).wrapping_add((((l_j1_20) as u32)).wrapping_mul(4))))));
                                    l_sum_9 = rt::fadd(l_ra_7, l_rb_8);
                                    if (((rt::feq(l_len_11, F32(0x00000000)) as i32)) != 0) {
                                        l_len_11 = F32(0x358637bd);
                                    }
                                    l_d_10 = rt::fdiv(rt::fsub(l_sum_9, l_t_6), l_len_11);
                                    if (((rt::fne(l_d_10, F32(0x00000000)) as i32)) != 0) {
                                        if (((rt::flt(l_d_10, F32(0x00000000)) as i32)) != 0) {
                                            break 'f 0i32;
                                        }
                                        if (((rt::flt(l_d_10, F32(c.m.r32(p_depth_5))) as i32)) != 0) {
                                            { let v_ = l_d_10; let a_ = p_depth_5; c.m.w32(a_, (v_).0); };
                                            l_code_15 = ((6i32).wrapping_add((l_i_16).wrapping_mul(3i32))).wrapping_add(l_j_17);
                                        }
                                    }
                                }
                            }
                            { let o_ = l_j_17; let n_ = (o_).wrapping_add(1); l_j_17 = n_; };
                        }
                    }
                }
                { let o_ = l_i_16; let n_ = (o_).wrapping_add(1); l_i_16 = n_; };
            }
        }
        if (((((l_code_15) < (3i32)) as i32)) != 0) {
            { let v_ = (if (((rt::fgt(F32(c.m.r32(((fp.wrapping_add(16)).wrapping_add((((l_code_15) as u32)).wrapping_mul(12))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(0xb727c5ac)) as i32)) != 0) { 1i32 } else { (1i32).wrapping_neg() }); let a_ = (fp.wrapping_add(192)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            { let v_ = (if (((rt::fgt(F32(c.m.r32(((fp.wrapping_add(16)).wrapping_add((((l_code_15) as u32)).wrapping_mul(12))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(0xb727c5ac)) as i32)) != 0) { 1i32 } else { (1i32).wrapping_neg() }); let a_ = (fp.wrapping_add(192)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            { let v_ = (if (((rt::fgt(F32(c.m.r32(((fp.wrapping_add(16)).wrapping_add((((l_code_15) as u32)).wrapping_mul(12))).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), F32(0xb727c5ac)) as i32)) != 0) { 1i32 } else { (1i32).wrapping_neg() }); let a_ = (fp.wrapping_add(192)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            if ((((!(((rt::fgt(F32(c.m.r32((fp.wrapping_add(112)).wrapping_add((((l_code_15) as u32)).wrapping_mul(4)))), F32(0x00000000)) as i32)) != 0)) as i32)) != 0) {
                { let v_ = F32(c.m.r32(((p_a_1).wrapping_add(32)).wrapping_add((((l_code_15) as u32)).wrapping_mul(16)))); let a_ = p_normal_3; c.m.w32(a_, (v_).0); };
                { let v_ = F32(c.m.r32((((p_a_1).wrapping_add(32)).wrapping_add((((l_code_15) as u32)).wrapping_mul(16))).wrapping_add(4))); let a_ = (p_normal_3).wrapping_add(4); c.m.w32(a_, (v_).0); };
                { let v_ = F32(c.m.r32((((p_a_1).wrapping_add(32)).wrapping_add((((l_code_15) as u32)).wrapping_mul(16))).wrapping_add(8))); let a_ = (p_normal_3).wrapping_add(8); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fadd(rt::fadd(rt::fadd(F32(c.m.r32(p_b_2)), rt::fmul(rt::fmul(rt::i2f((c.m.r32((fp.wrapping_add(192)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)), F32(c.m.r32(((p_b_2).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(16))))), F32(c.m.r32((l_hb_14).wrapping_add((((0i32) as u32)).wrapping_mul(4)))))), rt::fmul(rt::fmul(rt::i2f((c.m.r32((fp.wrapping_add(192)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)), F32(c.m.r32(((p_b_2).wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(16))))), F32(c.m.r32((l_hb_14).wrapping_add((((1i32) as u32)).wrapping_mul(4)))))), rt::fmul(rt::fmul(rt::i2f((c.m.r32((fp.wrapping_add(192)).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32)), F32(c.m.r32(((p_b_2).wrapping_add(32)).wrapping_add((((2i32) as u32)).wrapping_mul(16))))), F32(c.m.r32((l_hb_14).wrapping_add((((2i32) as u32)).wrapping_mul(4)))))); let a_ = p_point_4; c.m.w32(a_, (v_).0); };
                { let v_ = rt::fadd(rt::fadd(rt::fadd(F32(c.m.r32((p_b_2).wrapping_add(4))), rt::fmul(rt::fmul(rt::i2f((c.m.r32((fp.wrapping_add(192)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)), F32(c.m.r32((((p_b_2).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4)))), F32(c.m.r32((l_hb_14).wrapping_add((((0i32) as u32)).wrapping_mul(4)))))), rt::fmul(rt::fmul(rt::i2f((c.m.r32((fp.wrapping_add(192)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)), F32(c.m.r32((((p_b_2).wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(4)))), F32(c.m.r32((l_hb_14).wrapping_add((((1i32) as u32)).wrapping_mul(4)))))), rt::fmul(rt::fmul(rt::i2f((c.m.r32((fp.wrapping_add(192)).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32)), F32(c.m.r32((((p_b_2).wrapping_add(32)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(4)))), F32(c.m.r32((l_hb_14).wrapping_add((((2i32) as u32)).wrapping_mul(4)))))); let a_ = (p_point_4).wrapping_add(4); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fadd(rt::fadd(rt::fadd(F32(c.m.r32((p_b_2).wrapping_add(8))), rt::fmul(rt::fmul(rt::i2f((c.m.r32((fp.wrapping_add(192)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)), F32(c.m.r32((((p_b_2).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8)))), F32(c.m.r32((l_hb_14).wrapping_add((((0i32) as u32)).wrapping_mul(4)))))), rt::fmul(rt::fmul(rt::i2f((c.m.r32((fp.wrapping_add(192)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)), F32(c.m.r32((((p_b_2).wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8)))), F32(c.m.r32((l_hb_14).wrapping_add((((1i32) as u32)).wrapping_mul(4)))))), rt::fmul(rt::fmul(rt::i2f((c.m.r32((fp.wrapping_add(192)).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32)), F32(c.m.r32((((p_b_2).wrapping_add(32)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8)))), F32(c.m.r32((l_hb_14).wrapping_add((((2i32) as u32)).wrapping_mul(4)))))); let a_ = (p_point_4).wrapping_add(8); c.m.w32(a_, (v_).0); };
            } else {
                { let v_ = rt::fneg(F32(c.m.r32(((p_a_1).wrapping_add(32)).wrapping_add((((l_code_15) as u32)).wrapping_mul(16))))); let a_ = p_normal_3; c.m.w32(a_, (v_).0); };
                { let v_ = rt::fneg(F32(c.m.r32((((p_a_1).wrapping_add(32)).wrapping_add((((l_code_15) as u32)).wrapping_mul(16))).wrapping_add(4)))); let a_ = (p_normal_3).wrapping_add(4); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fneg(F32(c.m.r32((((p_a_1).wrapping_add(32)).wrapping_add((((l_code_15) as u32)).wrapping_mul(16))).wrapping_add(8)))); let a_ = (p_normal_3).wrapping_add(8); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fsub(rt::fsub(rt::fsub(F32(c.m.r32(p_b_2)), rt::fmul(rt::fmul(rt::i2f((c.m.r32((fp.wrapping_add(192)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)), F32(c.m.r32(((p_b_2).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(16))))), F32(c.m.r32((l_hb_14).wrapping_add((((0i32) as u32)).wrapping_mul(4)))))), rt::fmul(rt::fmul(rt::i2f((c.m.r32((fp.wrapping_add(192)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)), F32(c.m.r32(((p_b_2).wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(16))))), F32(c.m.r32((l_hb_14).wrapping_add((((1i32) as u32)).wrapping_mul(4)))))), rt::fmul(rt::fmul(rt::i2f((c.m.r32((fp.wrapping_add(192)).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32)), F32(c.m.r32(((p_b_2).wrapping_add(32)).wrapping_add((((2i32) as u32)).wrapping_mul(16))))), F32(c.m.r32((l_hb_14).wrapping_add((((2i32) as u32)).wrapping_mul(4)))))); let a_ = p_point_4; c.m.w32(a_, (v_).0); };
                { let v_ = rt::fsub(rt::fsub(rt::fsub(F32(c.m.r32((p_b_2).wrapping_add(4))), rt::fmul(rt::fmul(rt::i2f((c.m.r32((fp.wrapping_add(192)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)), F32(c.m.r32((((p_b_2).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4)))), F32(c.m.r32((l_hb_14).wrapping_add((((0i32) as u32)).wrapping_mul(4)))))), rt::fmul(rt::fmul(rt::i2f((c.m.r32((fp.wrapping_add(192)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)), F32(c.m.r32((((p_b_2).wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(4)))), F32(c.m.r32((l_hb_14).wrapping_add((((1i32) as u32)).wrapping_mul(4)))))), rt::fmul(rt::fmul(rt::i2f((c.m.r32((fp.wrapping_add(192)).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32)), F32(c.m.r32((((p_b_2).wrapping_add(32)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(4)))), F32(c.m.r32((l_hb_14).wrapping_add((((2i32) as u32)).wrapping_mul(4)))))); let a_ = (p_point_4).wrapping_add(4); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fsub(rt::fsub(rt::fsub(F32(c.m.r32((p_b_2).wrapping_add(8))), rt::fmul(rt::fmul(rt::i2f((c.m.r32((fp.wrapping_add(192)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)), F32(c.m.r32((((p_b_2).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8)))), F32(c.m.r32((l_hb_14).wrapping_add((((0i32) as u32)).wrapping_mul(4)))))), rt::fmul(rt::fmul(rt::i2f((c.m.r32((fp.wrapping_add(192)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)), F32(c.m.r32((((p_b_2).wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8)))), F32(c.m.r32((l_hb_14).wrapping_add((((1i32) as u32)).wrapping_mul(4)))))), rt::fmul(rt::fmul(rt::i2f((c.m.r32((fp.wrapping_add(192)).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32)), F32(c.m.r32((((p_b_2).wrapping_add(32)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8)))), F32(c.m.r32((l_hb_14).wrapping_add((((2i32) as u32)).wrapping_mul(4)))))); let a_ = (p_point_4).wrapping_add(8); c.m.w32(a_, (v_).0); };
            }
        } else {
            if (((((l_code_15) < (6i32)) as i32)) != 0) {
                l_k_22 = (l_code_15).wrapping_sub(3i32);
                { let v_ = (if (((rt::fgt(F32(c.m.r32(((fp.wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(12))).wrapping_add((((l_k_22) as u32)).wrapping_mul(4)))), F32(0xb727c5ac)) as i32)) != 0) { 1i32 } else { (1i32).wrapping_neg() }); let a_ = (fp.wrapping_add(192)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = (if (((rt::fgt(F32(c.m.r32(((fp.wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(12))).wrapping_add((((l_k_22) as u32)).wrapping_mul(4)))), F32(0xb727c5ac)) as i32)) != 0) { 1i32 } else { (1i32).wrapping_neg() }); let a_ = (fp.wrapping_add(192)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = (if (((rt::fgt(F32(c.m.r32(((fp.wrapping_add(16)).wrapping_add((((2i32) as u32)).wrapping_mul(12))).wrapping_add((((l_k_22) as u32)).wrapping_mul(4)))), F32(0xb727c5ac)) as i32)) != 0) { 1i32 } else { (1i32).wrapping_neg() }); let a_ = (fp.wrapping_add(192)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                if ((((!(((rt::fgt(F::Vec3_Dot(c, ((p_b_2).wrapping_add(32)).wrapping_add((((l_k_22) as u32)).wrapping_mul(16)), fp), F32(0x00000000)) as i32)) != 0)) as i32)) != 0) {
                    { let v_ = F32(c.m.r32(((p_b_2).wrapping_add(32)).wrapping_add((((l_k_22) as u32)).wrapping_mul(16)))); let a_ = p_normal_3; c.m.w32(a_, (v_).0); };
                    { let v_ = F32(c.m.r32((((p_b_2).wrapping_add(32)).wrapping_add((((l_k_22) as u32)).wrapping_mul(16))).wrapping_add(4))); let a_ = (p_normal_3).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(c.m.r32((((p_b_2).wrapping_add(32)).wrapping_add((((l_k_22) as u32)).wrapping_mul(16))).wrapping_add(8))); let a_ = (p_normal_3).wrapping_add(8); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fsub(rt::fsub(rt::fsub(F32(c.m.r32(p_a_1)), rt::fmul(rt::fmul(rt::i2f((c.m.r32((fp.wrapping_add(192)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)), F32(c.m.r32(((p_a_1).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(16))))), F32(c.m.r32((l_ha_13).wrapping_add((((0i32) as u32)).wrapping_mul(4)))))), rt::fmul(rt::fmul(rt::i2f((c.m.r32((fp.wrapping_add(192)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)), F32(c.m.r32(((p_a_1).wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(16))))), F32(c.m.r32((l_ha_13).wrapping_add((((1i32) as u32)).wrapping_mul(4)))))), rt::fmul(rt::fmul(rt::i2f((c.m.r32((fp.wrapping_add(192)).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32)), F32(c.m.r32(((p_a_1).wrapping_add(32)).wrapping_add((((2i32) as u32)).wrapping_mul(16))))), F32(c.m.r32((l_ha_13).wrapping_add((((2i32) as u32)).wrapping_mul(4)))))); let a_ = p_point_4; c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fsub(rt::fsub(rt::fsub(F32(c.m.r32((p_a_1).wrapping_add(4))), rt::fmul(rt::fmul(rt::i2f((c.m.r32((fp.wrapping_add(192)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)), F32(c.m.r32((((p_a_1).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4)))), F32(c.m.r32((l_ha_13).wrapping_add((((0i32) as u32)).wrapping_mul(4)))))), rt::fmul(rt::fmul(rt::i2f((c.m.r32((fp.wrapping_add(192)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)), F32(c.m.r32((((p_a_1).wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(4)))), F32(c.m.r32((l_ha_13).wrapping_add((((1i32) as u32)).wrapping_mul(4)))))), rt::fmul(rt::fmul(rt::i2f((c.m.r32((fp.wrapping_add(192)).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32)), F32(c.m.r32((((p_a_1).wrapping_add(32)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(4)))), F32(c.m.r32((l_ha_13).wrapping_add((((2i32) as u32)).wrapping_mul(4)))))); let a_ = (p_point_4).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fsub(rt::fsub(rt::fsub(F32(c.m.r32((p_a_1).wrapping_add(8))), rt::fmul(rt::fmul(rt::i2f((c.m.r32((fp.wrapping_add(192)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)), F32(c.m.r32((((p_a_1).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8)))), F32(c.m.r32((l_ha_13).wrapping_add((((0i32) as u32)).wrapping_mul(4)))))), rt::fmul(rt::fmul(rt::i2f((c.m.r32((fp.wrapping_add(192)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)), F32(c.m.r32((((p_a_1).wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8)))), F32(c.m.r32((l_ha_13).wrapping_add((((1i32) as u32)).wrapping_mul(4)))))), rt::fmul(rt::fmul(rt::i2f((c.m.r32((fp.wrapping_add(192)).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32)), F32(c.m.r32((((p_a_1).wrapping_add(32)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8)))), F32(c.m.r32((l_ha_13).wrapping_add((((2i32) as u32)).wrapping_mul(4)))))); let a_ = (p_point_4).wrapping_add(8); c.m.w32(a_, (v_).0); };
                } else {
                    { let v_ = rt::fneg(F32(c.m.r32(((p_b_2).wrapping_add(32)).wrapping_add((((l_k_22) as u32)).wrapping_mul(16))))); let a_ = p_normal_3; c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fneg(F32(c.m.r32((((p_b_2).wrapping_add(32)).wrapping_add((((l_k_22) as u32)).wrapping_mul(16))).wrapping_add(4)))); let a_ = (p_normal_3).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fneg(F32(c.m.r32((((p_b_2).wrapping_add(32)).wrapping_add((((l_k_22) as u32)).wrapping_mul(16))).wrapping_add(8)))); let a_ = (p_normal_3).wrapping_add(8); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fadd(rt::fadd(rt::fadd(F32(c.m.r32(p_a_1)), rt::fmul(rt::fmul(rt::i2f((c.m.r32((fp.wrapping_add(192)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)), F32(c.m.r32(((p_a_1).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(16))))), F32(c.m.r32((l_ha_13).wrapping_add((((0i32) as u32)).wrapping_mul(4)))))), rt::fmul(rt::fmul(rt::i2f((c.m.r32((fp.wrapping_add(192)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)), F32(c.m.r32(((p_a_1).wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(16))))), F32(c.m.r32((l_ha_13).wrapping_add((((1i32) as u32)).wrapping_mul(4)))))), rt::fmul(rt::fmul(rt::i2f((c.m.r32((fp.wrapping_add(192)).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32)), F32(c.m.r32(((p_a_1).wrapping_add(32)).wrapping_add((((2i32) as u32)).wrapping_mul(16))))), F32(c.m.r32((l_ha_13).wrapping_add((((2i32) as u32)).wrapping_mul(4)))))); let a_ = p_point_4; c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fadd(rt::fadd(rt::fadd(F32(c.m.r32((p_a_1).wrapping_add(4))), rt::fmul(rt::fmul(rt::i2f((c.m.r32((fp.wrapping_add(192)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)), F32(c.m.r32((((p_a_1).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4)))), F32(c.m.r32((l_ha_13).wrapping_add((((0i32) as u32)).wrapping_mul(4)))))), rt::fmul(rt::fmul(rt::i2f((c.m.r32((fp.wrapping_add(192)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)), F32(c.m.r32((((p_a_1).wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(4)))), F32(c.m.r32((l_ha_13).wrapping_add((((1i32) as u32)).wrapping_mul(4)))))), rt::fmul(rt::fmul(rt::i2f((c.m.r32((fp.wrapping_add(192)).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32)), F32(c.m.r32((((p_a_1).wrapping_add(32)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(4)))), F32(c.m.r32((l_ha_13).wrapping_add((((2i32) as u32)).wrapping_mul(4)))))); let a_ = (p_point_4).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fadd(rt::fadd(rt::fadd(F32(c.m.r32((p_a_1).wrapping_add(8))), rt::fmul(rt::fmul(rt::i2f((c.m.r32((fp.wrapping_add(192)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)), F32(c.m.r32((((p_a_1).wrapping_add(32)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8)))), F32(c.m.r32((l_ha_13).wrapping_add((((0i32) as u32)).wrapping_mul(4)))))), rt::fmul(rt::fmul(rt::i2f((c.m.r32((fp.wrapping_add(192)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)), F32(c.m.r32((((p_a_1).wrapping_add(32)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8)))), F32(c.m.r32((l_ha_13).wrapping_add((((1i32) as u32)).wrapping_mul(4)))))), rt::fmul(rt::fmul(rt::i2f((c.m.r32((fp.wrapping_add(192)).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32)), F32(c.m.r32((((p_a_1).wrapping_add(32)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8)))), F32(c.m.r32((l_ha_13).wrapping_add((((2i32) as u32)).wrapping_mul(4)))))); let a_ = (p_point_4).wrapping_add(8); c.m.w32(a_, (v_).0); };
                }
            } else {
                if ((((((((l_code_15).wrapping_sub(6i32)) as u32)) < (((9i32) as u32))) as i32)) != 0) {
                    l_i_16 = rt::div_i32((l_code_15).wrapping_sub(6i32), 3i32);
                    l_j_17 = rt::rem_i32((l_code_15).wrapping_sub(6i32), 3i32);
                    l_i1_18 = rt::rem_i32((l_i_16).wrapping_add(1i32), 3i32);
                    l_i2_19 = rt::rem_i32((l_i_16).wrapping_add(2i32), 3i32);
                    l_j1_20 = rt::rem_i32((l_j_17).wrapping_add(1i32), 3i32);
                    l_j2_21 = rt::rem_i32((l_j_17).wrapping_add(2i32), 3i32);
                    l_sa1_25 = rt::i2f((if (((rt::flt(F32(c.m.r32(((fp.wrapping_add(16)).wrapping_add((((l_i2_19) as u32)).wrapping_mul(12))).wrapping_add((((l_j_17) as u32)).wrapping_mul(4)))), F32(0x00000000)) as i32)) != 0) { 1i32 } else { (1i32).wrapping_neg() }));
                    l_sa2_26 = rt::i2f((if (((rt::flt(F32(0x00000000), F32(c.m.r32(((fp.wrapping_add(16)).wrapping_add((((l_i1_18) as u32)).wrapping_mul(12))).wrapping_add((((l_j_17) as u32)).wrapping_mul(4))))) as i32)) != 0) { 1i32 } else { (1i32).wrapping_neg() }));
                    l_sb1_27 = rt::i2f((if (((rt::flt(F32(c.m.r32(((fp.wrapping_add(16)).wrapping_add((((l_i_16) as u32)).wrapping_mul(12))).wrapping_add((((l_j2_21) as u32)).wrapping_mul(4)))), F32(0x00000000)) as i32)) != 0) { 1i32 } else { (1i32).wrapping_neg() }));
                    l_sb2_28 = rt::i2f((if (((rt::flt(F32(0x00000000), F32(c.m.r32(((fp.wrapping_add(16)).wrapping_add((((l_i_16) as u32)).wrapping_mul(12))).wrapping_add((((l_j1_20) as u32)).wrapping_mul(4))))) as i32)) != 0) { 1i32 } else { (1i32).wrapping_neg() }));
                    l_w_12 = rt::fadd(rt::fmul(rt::fneg(F32(c.m.r32(((fp.wrapping_add(16)).wrapping_add((((l_i2_19) as u32)).wrapping_mul(12))).wrapping_add((((l_j_17) as u32)).wrapping_mul(4))))), F32(c.m.r32((fp.wrapping_add(112)).wrapping_add((((l_i1_18) as u32)).wrapping_mul(4))))), rt::fmul(F32(c.m.r32(((fp.wrapping_add(16)).wrapping_add((((l_i1_18) as u32)).wrapping_mul(12))).wrapping_add((((l_j_17) as u32)).wrapping_mul(4)))), F32(c.m.r32((fp.wrapping_add(112)).wrapping_add((((l_i2_19) as u32)).wrapping_mul(4))))));
                    if (((rt::fgt(rt::fadd(rt::fmul(rt::fmul(F32(c.m.r32((l_ha_13).wrapping_add((((l_i1_18) as u32)).wrapping_mul(4)))), l_sa1_25), rt::fmul(rt::fneg(F32(c.m.r32(((fp.wrapping_add(16)).wrapping_add((((l_i2_19) as u32)).wrapping_mul(12))).wrapping_add((((l_j_17) as u32)).wrapping_mul(4))))), l_w_12)), rt::fmul(rt::fmul(F32(c.m.r32((l_ha_13).wrapping_add((((l_i2_19) as u32)).wrapping_mul(4)))), l_sa2_26), rt::fmul(F32(c.m.r32(((fp.wrapping_add(16)).wrapping_add((((l_i1_18) as u32)).wrapping_mul(12))).wrapping_add((((l_j_17) as u32)).wrapping_mul(4)))), l_w_12))), F32(0xb727c5ac)) as i32)) != 0) {
                        {
                            l_m_23 = 0i32;
                            'l9: loop {
                                if !(((((l_m_23) < (3i32)) as i32)) != 0) { break 'l9; }
                                'c10: {
                                    { let v_ = rt::fadd(rt::fadd(rt::fadd(F32(c.m.r32((p_a_1).wrapping_add((((l_m_23) as u32)).wrapping_mul(4)))), rt::fmul(rt::fneg(F32(c.m.r32((((p_a_1).wrapping_add(32)).wrapping_add((((l_i_16) as u32)).wrapping_mul(16))).wrapping_add((((l_m_23) as u32)).wrapping_mul(4))))), F32(c.m.r32((l_ha_13).wrapping_add((((l_i_16) as u32)).wrapping_mul(4)))))), rt::fmul(rt::fmul(F32(c.m.r32((((p_a_1).wrapping_add(32)).wrapping_add((((l_i1_18) as u32)).wrapping_mul(16))).wrapping_add((((l_m_23) as u32)).wrapping_mul(4)))), F32(c.m.r32((l_ha_13).wrapping_add((((l_i1_18) as u32)).wrapping_mul(4))))), l_sa1_25)), rt::fmul(rt::fmul(F32(c.m.r32((((p_a_1).wrapping_add(32)).wrapping_add((((l_i2_19) as u32)).wrapping_mul(16))).wrapping_add((((l_m_23) as u32)).wrapping_mul(4)))), F32(c.m.r32((l_ha_13).wrapping_add((((l_i2_19) as u32)).wrapping_mul(4))))), l_sa2_26)); let a_ = (fp.wrapping_add(144)).wrapping_add((((l_m_23) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                                }
                                { let o_ = l_m_23; let n_ = (o_).wrapping_add(1); l_m_23 = n_; };
                            }
                        }
                    } else {
                        {
                            l_m_23 = 0i32;
                            'l11: loop {
                                if !(((((l_m_23) < (3i32)) as i32)) != 0) { break 'l11; }
                                'c12: {
                                    { let v_ = rt::fadd(rt::fadd(rt::fadd(F32(c.m.r32((p_a_1).wrapping_add((((l_m_23) as u32)).wrapping_mul(4)))), rt::fmul(rt::fneg(F32(c.m.r32((((p_a_1).wrapping_add(32)).wrapping_add((((l_i_16) as u32)).wrapping_mul(16))).wrapping_add((((l_m_23) as u32)).wrapping_mul(4))))), F32(c.m.r32((l_ha_13).wrapping_add((((l_i_16) as u32)).wrapping_mul(4)))))), rt::fmul(rt::fmul(rt::fneg(F32(c.m.r32((((p_a_1).wrapping_add(32)).wrapping_add((((l_i1_18) as u32)).wrapping_mul(16))).wrapping_add((((l_m_23) as u32)).wrapping_mul(4))))), F32(c.m.r32((l_ha_13).wrapping_add((((l_i1_18) as u32)).wrapping_mul(4))))), l_sa1_25)), rt::fmul(rt::fmul(rt::fneg(F32(c.m.r32((((p_a_1).wrapping_add(32)).wrapping_add((((l_i2_19) as u32)).wrapping_mul(16))).wrapping_add((((l_m_23) as u32)).wrapping_mul(4))))), F32(c.m.r32((l_ha_13).wrapping_add((((l_i2_19) as u32)).wrapping_mul(4))))), l_sa2_26)); let a_ = (fp.wrapping_add(144)).wrapping_add((((l_m_23) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                                }
                                { let o_ = l_m_23; let n_ = (o_).wrapping_add(1); l_m_23 = n_; };
                            }
                        }
                    }
                    l_w_12 = rt::fadd(rt::fmul(rt::fneg(F32(c.m.r32(((fp.wrapping_add(16)).wrapping_add((((l_i_16) as u32)).wrapping_mul(12))).wrapping_add((((l_j2_21) as u32)).wrapping_mul(4))))), F32(c.m.r32((fp.wrapping_add(128)).wrapping_add((((l_j1_20) as u32)).wrapping_mul(4))))), rt::fmul(F32(c.m.r32(((fp.wrapping_add(16)).wrapping_add((((l_i_16) as u32)).wrapping_mul(12))).wrapping_add((((l_j1_20) as u32)).wrapping_mul(4)))), F32(c.m.r32((fp.wrapping_add(128)).wrapping_add((((l_j2_21) as u32)).wrapping_mul(4))))));
                    if (((rt::fgt(rt::fadd(rt::fmul(rt::fmul(F32(c.m.r32((l_hb_14).wrapping_add((((l_j1_20) as u32)).wrapping_mul(4)))), l_sb1_27), rt::fmul(rt::fneg(F32(c.m.r32(((fp.wrapping_add(16)).wrapping_add((((l_i_16) as u32)).wrapping_mul(12))).wrapping_add((((l_j2_21) as u32)).wrapping_mul(4))))), l_w_12)), rt::fmul(rt::fmul(F32(c.m.r32((l_hb_14).wrapping_add((((l_j2_21) as u32)).wrapping_mul(4)))), l_sb2_28), rt::fmul(F32(c.m.r32(((fp.wrapping_add(16)).wrapping_add((((l_i_16) as u32)).wrapping_mul(12))).wrapping_add((((l_j1_20) as u32)).wrapping_mul(4)))), l_w_12))), F32(0xb727c5ac)) as i32)) != 0) {
                        {
                            l_m_23 = 0i32;
                            'l13: loop {
                                if !(((((l_m_23) < (3i32)) as i32)) != 0) { break 'l13; }
                                'c14: {
                                    { let v_ = rt::fadd(rt::fadd(rt::fadd(F32(c.m.r32((p_b_2).wrapping_add((((l_m_23) as u32)).wrapping_mul(4)))), rt::fmul(rt::fneg(F32(c.m.r32((((p_b_2).wrapping_add(32)).wrapping_add((((l_j_17) as u32)).wrapping_mul(16))).wrapping_add((((l_m_23) as u32)).wrapping_mul(4))))), F32(c.m.r32((l_hb_14).wrapping_add((((l_j_17) as u32)).wrapping_mul(4)))))), rt::fmul(rt::fmul(rt::fneg(F32(c.m.r32((((p_b_2).wrapping_add(32)).wrapping_add((((l_j1_20) as u32)).wrapping_mul(16))).wrapping_add((((l_m_23) as u32)).wrapping_mul(4))))), F32(c.m.r32((l_hb_14).wrapping_add((((l_j1_20) as u32)).wrapping_mul(4))))), l_sb1_27)), rt::fmul(rt::fmul(rt::fneg(F32(c.m.r32((((p_b_2).wrapping_add(32)).wrapping_add((((l_j2_21) as u32)).wrapping_mul(16))).wrapping_add((((l_m_23) as u32)).wrapping_mul(4))))), F32(c.m.r32((l_hb_14).wrapping_add((((l_j2_21) as u32)).wrapping_mul(4))))), l_sb2_28)); let a_ = (fp.wrapping_add(160)).wrapping_add((((l_m_23) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                                }
                                { let o_ = l_m_23; let n_ = (o_).wrapping_add(1); l_m_23 = n_; };
                            }
                        }
                    } else {
                        {
                            l_m_23 = 0i32;
                            'l15: loop {
                                if !(((((l_m_23) < (3i32)) as i32)) != 0) { break 'l15; }
                                'c16: {
                                    { let v_ = rt::fadd(rt::fadd(rt::fadd(F32(c.m.r32((p_b_2).wrapping_add((((l_m_23) as u32)).wrapping_mul(4)))), rt::fmul(rt::fneg(F32(c.m.r32((((p_b_2).wrapping_add(32)).wrapping_add((((l_j_17) as u32)).wrapping_mul(16))).wrapping_add((((l_m_23) as u32)).wrapping_mul(4))))), F32(c.m.r32((l_hb_14).wrapping_add((((l_j_17) as u32)).wrapping_mul(4)))))), rt::fmul(rt::fmul(F32(c.m.r32((((p_b_2).wrapping_add(32)).wrapping_add((((l_j1_20) as u32)).wrapping_mul(16))).wrapping_add((((l_m_23) as u32)).wrapping_mul(4)))), F32(c.m.r32((l_hb_14).wrapping_add((((l_j1_20) as u32)).wrapping_mul(4))))), l_sb1_27)), rt::fmul(rt::fmul(F32(c.m.r32((((p_b_2).wrapping_add(32)).wrapping_add((((l_j2_21) as u32)).wrapping_mul(16))).wrapping_add((((l_m_23) as u32)).wrapping_mul(4)))), F32(c.m.r32((l_hb_14).wrapping_add((((l_j2_21) as u32)).wrapping_mul(4))))), l_sb2_28)); let a_ = (fp.wrapping_add(160)).wrapping_add((((l_m_23) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                                }
                                { let o_ = l_m_23; let n_ = (o_).wrapping_add(1); l_m_23 = n_; };
                            }
                        }
                    }
                    F::Col_LineLineParams(c, fp.wrapping_add(204), fp.wrapping_add(208), fp.wrapping_add(144), ((p_a_1).wrapping_add(32)).wrapping_add((((l_i_16) as u32)).wrapping_mul(16)), fp.wrapping_add(160), ((p_b_2).wrapping_add(32)).wrapping_add((((l_j_17) as u32)).wrapping_mul(16)));
                    { let v_ = rt::fadd(F32(c.m.r32((fp.wrapping_add(144)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(((p_a_1).wrapping_add(32)).wrapping_add((((l_i_16) as u32)).wrapping_mul(16)))), F32(c.m.r32(fp.wrapping_add(204))))); let a_ = p_point_4; c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fadd(F32(c.m.r32((fp.wrapping_add(144)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((((p_a_1).wrapping_add(32)).wrapping_add((((l_i_16) as u32)).wrapping_mul(16))).wrapping_add(4))), F32(c.m.r32(fp.wrapping_add(204))))); let a_ = (p_point_4).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fadd(F32(c.m.r32((fp.wrapping_add(144)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((((p_a_1).wrapping_add(32)).wrapping_add((((l_i_16) as u32)).wrapping_mul(16))).wrapping_add(8))), F32(c.m.r32(fp.wrapping_add(204))))); let a_ = (p_point_4).wrapping_add(8); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fsub(rt::fadd(F32(c.m.r32((fp.wrapping_add(160)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32(((p_b_2).wrapping_add(32)).wrapping_add((((l_j_17) as u32)).wrapping_mul(16)))), F32(c.m.r32(fp.wrapping_add(208))))), F32(c.m.r32(p_point_4))); let a_ = fp.wrapping_add(176); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fsub(rt::fadd(F32(c.m.r32((fp.wrapping_add(160)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((((p_b_2).wrapping_add(32)).wrapping_add((((l_j_17) as u32)).wrapping_mul(16))).wrapping_add(4))), F32(c.m.r32(fp.wrapping_add(208))))), F32(c.m.r32((p_point_4).wrapping_add(4)))); let a_ = (fp.wrapping_add(176)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fsub(rt::fadd(F32(c.m.r32((fp.wrapping_add(160)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), rt::fmul(F32(c.m.r32((((p_b_2).wrapping_add(32)).wrapping_add((((l_j_17) as u32)).wrapping_mul(16))).wrapping_add(8))), F32(c.m.r32(fp.wrapping_add(208))))), F32(c.m.r32((p_point_4).wrapping_add(8)))); let a_ = (fp.wrapping_add(176)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                    F::Vec3_Normalize(c, p_normal_3, fp.wrapping_add(176));
                }
            }
        }
        break 'f 1i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "ColObb_Contact", &PROBE_N, &[], true); }
    r_
}
