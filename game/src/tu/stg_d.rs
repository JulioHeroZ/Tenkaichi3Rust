#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn StgRigidList_Last(c: &mut Ctx, p_head_1: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgRigidList_Last", &PROBE_N, &[(p_head_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_link_2: u32 = 0;
    let mut l_last_3: u32 = 0;
    c.m.w32(fp, p_head_1);
    let r_ = 'f: {
        l_link_2 = fp;
        l_last_3 = ((0i32) as u32);
        'l1: while (((((c.m.r32(l_link_2)) != (((0i32) as u32))) as i32)) != 0) {
            l_last_3 = c.m.r32(l_link_2);
            l_link_2 = (l_last_3).wrapping_add(4);
        }
        break 'f l_last_3;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "StgRigidList_Last", &PROBE_N, &[], true); }
    r_
}

pub fn StgRigid_Find(c: &mut Ctx, mut p_id_1: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgRigid_Find", &PROBE_N, &[(p_id_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_link_2: u32 = 0;
    let mut l_n_3: u32 = 0;
    let r_ = 'f: {
        { let v_ = { let a_ = (c.m.r32(sym::gStgRigid)).wrapping_add(73732); c.m.r32(a_) }; c.m.w32(fp, v_); }
        l_link_2 = fp;
        'l1: while (((((c.m.r32(l_link_2)) != (((0i32) as u32))) as i32)) != 0) {
            l_n_3 = c.m.r32(l_link_2);
            if ((((((c.m.r32((l_n_3).wrapping_add(12)) as i32)) == (p_id_1)) as i32)) != 0) {
                break 'f l_n_3;
            }
            l_link_2 = (l_n_3).wrapping_add(4);
        }
        break 'f ((0i32) as u32);
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "StgRigid_Find", &PROBE_N, &[], true); }
    r_
}

pub fn StgRigid_ClampToStage(c: &mut Ctx, mut p_body_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgRigid_ClampToStage", &PROBE_N, &[(p_body_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_dist_2: F32 = F32(0);
    let mut l_limit_3: F32 = F32(0);
    let mut l_k_4: F32 = F32(0);
    let r_ = 'f: {
        { let v_ = F32(0x00000000); let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((p_body_1).wrapping_add(64))); let a_ = fp; c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32(((p_body_1).wrapping_add(64)).wrapping_add(8))); let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_).0); };
        l_dist_2 = rt::fadd({ let a0_ = rt::fadd(rt::fmul(F32(c.m.r32(fp)), F32(c.m.r32(fp))), rt::fmul(F32(c.m.r32((fp).wrapping_add(8))), F32(c.m.r32((fp).wrapping_add(8))))); F::Port_sqrtf(c, a0_) }, F32(c.m.r32(((p_body_1).wrapping_add(464)).wrapping_add(16))));
        l_limit_3 = F::BtlStage_GetInnerRadius(c);
        if (((rt::flt(l_limit_3, l_dist_2) as i32)) != 0) {
            l_k_4 = rt::fdiv(l_limit_3, l_dist_2);
            { let r_ = l_k_4; let a_ = (p_body_1).wrapping_add(64); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            { let r_ = l_k_4; let a_ = ((p_body_1).wrapping_add(64)).wrapping_add(8); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            { let r_ = F32(0x3f000000); let a_ = (p_body_1).wrapping_add(160); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            { let r_ = F32(0x3f000000); let a_ = ((p_body_1).wrapping_add(160)).wrapping_add(4); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            { let r_ = F32(0x3f000000); let a_ = ((p_body_1).wrapping_add(160)).wrapping_add(8); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            { let r_ = F32(0x3f000000); let a_ = (p_body_1).wrapping_add(208); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            { let r_ = F32(0x3f000000); let a_ = ((p_body_1).wrapping_add(208)).wrapping_add(4); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            { let r_ = F32(0x3f000000); let a_ = ((p_body_1).wrapping_add(208)).wrapping_add(8); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            F::Rigid_SyncSpheres(c, p_body_1);
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "StgRigid_ClampToStage", &PROBE_N, &[], true); }
    r_
}

pub fn StgRigidList_BeginFrame(c: &mut Ctx, p_head_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgRigidList_BeginFrame", &PROBE_N, &[(p_head_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let mut l_n_2: u32 = 0;
    let mut l_link_3: u32 = 0;
    let mut l_hasWater_4: i32 = 0;
    c.m.w32(fp, p_head_1);
    let r_ = 'f: {
        l_hasWater_4 = F::BtlStage_GetWaterLevel(c, fp.wrapping_add(40));
        {
            l_link_3 = fp;
            'l1: loop {
                if !(((((c.m.r32(l_link_3)) != (((0i32) as u32))) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_n_2 = c.m.r32(l_link_3);
                    F::Rigid_BeginFrame(c, (l_n_2).wrapping_add(16));
                    F::Rigid_SyncSpheres(c, (l_n_2).wrapping_add(16));
                    if (((c.m.r32((l_n_2).wrapping_add(560)) as i32)) != 0) {
                        break 'c2;
                    }
                    if ((((((((((c.m.r32(((l_n_2).wrapping_add(16)).wrapping_add(228)) as i32)) != (0i32)) as i32)) != 0) && ((F::BtlStage_IsReady(c)) != 0)) as i32)) != 0) {
                        { let a2_ = rt::fadd(rt::fadd(F32(c.m.r32((((l_n_2).wrapping_add(16)).wrapping_add(464)).wrapping_add(16))), F32(c.m.r32(((l_n_2).wrapping_add(16)).wrapping_add(320)))), F32(0x3dcccccc)); F::ColSphere_Set(c, fp.wrapping_add(8), ((l_n_2).wrapping_add(16)).wrapping_add(464), a2_) };
                        { let a1_ = (c.m.r32(((l_n_2).wrapping_add(16)).wrapping_add(228)) as i32); { let _ = F::StgCol_CollectSphere(c, fp.wrapping_add(8), ((((a1_) as i64 as u64)) as u32)); } };
                    }
                    { let v_ = F32(0x3ac49ba5); let a_ = (l_n_2).wrapping_add(8); c.m.w32(a_, (v_).0); };
                    if ((((((F::BtlStage_IsReady(c)) != 0) && ((l_hasWater_4) != 0)) as i32)) != 0) {
                        if ((((((((((c.m.r32(((l_n_2).wrapping_add(16)).wrapping_add(228)) as i32)) != (0i32)) as i32)) != 0) && (((rt::flt(F32(c.m.r32(fp.wrapping_add(40))), F32(c.m.r32((((l_n_2).wrapping_add(16)).wrapping_add(464)).wrapping_add(4)))) as i32)) != 0)) as i32)) != 0) {
                            { let v_ = F32(0x3de45a1c); let a_ = (l_n_2).wrapping_add(8); c.m.w32(a_, (v_).0); };
                        }
                    }
                }
                l_link_3 = (l_n_2).wrapping_add(4);
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "StgRigidList_BeginFrame", &PROBE_N, &[], true); }
    r_
}

pub fn StgRigidList_UpdateCenters(c: &mut Ctx, p_head_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgRigidList_UpdateCenters", &PROBE_N, &[(p_head_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_n_2: u32 = 0;
    let mut l_link_3: u32 = 0;
    c.m.w32(fp, p_head_1);
    let r_ = 'f: {
        {
            l_link_3 = fp;
            'l1: loop {
                if !(((((c.m.r32(l_link_3)) != (((0i32) as u32))) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_n_2 = c.m.r32(l_link_3);
                    if (((c.m.r32((l_n_2).wrapping_add(560)) as i32)) != 0) {
                        break 'c2;
                    }
                    F::Rigid_UpdateCenter(c, (l_n_2).wrapping_add(16));
                    F::Rigid_SyncSpheres(c, (l_n_2).wrapping_add(16));
                }
                l_link_3 = (l_n_2).wrapping_add(4);
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "StgRigidList_UpdateCenters", &PROBE_N, &[], true); }
    r_
}

pub fn StgRigidList_Collide(c: &mut Ctx, p_head_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgRigidList_Collide", &PROBE_N, &[(p_head_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(64) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 64);
    let mut l_n_2: u32 = 0;
    let mut l_link_3: u32 = 0;
    let mut l_i_4: u32 = 0;
    let mut l_depth_5: F32 = F32(0);
    c.m.w32(fp, p_head_1);
    let r_ = 'f: {
        {
            l_link_3 = fp;
            'l1: loop {
                if !(((((c.m.r32(l_link_3)) != (((0i32) as u32))) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_n_2 = c.m.r32(l_link_3);
                    if ((((!((F::BtlStage_IsReady(c)) != 0)) as i32)) != 0) {
                        break 'c2;
                    }
                    if (((c.m.r32((l_n_2).wrapping_add(560)) as i32)) != 0) {
                        break 'c2;
                    }
                    if ((((((c.m.r32(((l_n_2).wrapping_add(16)).wrapping_add(228)) as i32)) == (0i32)) as i32)) != 0) {
                        break 'c2;
                    }
                    {
                        l_i_4 = ((0i32) as u32);
                        'l3: loop {
                            if !(((((l_i_4) < ({ let a_ = (((c.m.r32(((l_n_2).wrapping_add(16)).wrapping_add(228)) as i32)) as u32); c.m.r32(a_) })) as i32)) != 0) { break 'l3; }
                            'c4: {
                                if (({ let a2_ = (((((c.m.r32(((l_n_2).wrapping_add(16)).wrapping_add(228)) as i32)) as u32)).wrapping_add(16)).wrapping_add((l_i_4).wrapping_mul(64)); F::ColSphere_TestTri(c, fp.wrapping_add(4), ((l_n_2).wrapping_add(16)).wrapping_add(464), a2_) }) != 0) {
                                    F::Vec4_Sub(c, fp.wrapping_add(20), ((l_n_2).wrapping_add(16)).wrapping_add(464), fp.wrapping_add(4));
                                    l_depth_5 = rt::fsub(F32(c.m.r32((((l_n_2).wrapping_add(16)).wrapping_add(464)).wrapping_add(16))), { let a0_ = F::Vec3_Dot(c, fp.wrapping_add(20), fp.wrapping_add(20)); F::Port_sqrtf(c, a0_) });
                                    { let v_ = rt::fadd(F32(c.m.r32(((l_n_2).wrapping_add(16)).wrapping_add(464))), rt::fmul(rt::fneg({ let a_ = ((((((c.m.r32(((l_n_2).wrapping_add(16)).wrapping_add(228)) as i32)) as u32)).wrapping_add(16)).wrapping_add((l_i_4).wrapping_mul(64))).wrapping_add(48); F32(c.m.r32(a_)) }), F32(c.m.r32((((l_n_2).wrapping_add(16)).wrapping_add(464)).wrapping_add(16))))); let a_ = fp.wrapping_add(36); c.m.w32(a_, (v_).0); };
                                    { let v_ = rt::fadd(F32(c.m.r32((((l_n_2).wrapping_add(16)).wrapping_add(464)).wrapping_add(4))), rt::fmul(rt::fneg({ let a_ = (((((((c.m.r32(((l_n_2).wrapping_add(16)).wrapping_add(228)) as i32)) as u32)).wrapping_add(16)).wrapping_add((l_i_4).wrapping_mul(64))).wrapping_add(48)).wrapping_add(4); F32(c.m.r32(a_)) }), F32(c.m.r32((((l_n_2).wrapping_add(16)).wrapping_add(464)).wrapping_add(16))))); let a_ = (fp.wrapping_add(36)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                                    { let v_ = rt::fadd(F32(c.m.r32((((l_n_2).wrapping_add(16)).wrapping_add(464)).wrapping_add(8))), rt::fmul(rt::fneg({ let a_ = (((((((c.m.r32(((l_n_2).wrapping_add(16)).wrapping_add(228)) as i32)) as u32)).wrapping_add(16)).wrapping_add((l_i_4).wrapping_mul(64))).wrapping_add(48)).wrapping_add(8); F32(c.m.r32(a_)) }), F32(c.m.r32((((l_n_2).wrapping_add(16)).wrapping_add(464)).wrapping_add(16))))); let a_ = (fp.wrapping_add(36)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                                    { let v_ = F32(0x3f800000); let a_ = (fp.wrapping_add(36)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                                    { let a2_ = ((((((c.m.r32(((l_n_2).wrapping_add(16)).wrapping_add(228)) as i32)) as u32)).wrapping_add(16)).wrapping_add((l_i_4).wrapping_mul(64))).wrapping_add(48); let a3_ = F32(c.m.r32(sym::gStgRigidRestitution)); let a4_ = F32(c.m.r32(sym::gStgRigidFriction)); F::Rigid_AddContact(c, (l_n_2).wrapping_add(16), fp.wrapping_add(36), a2_, a3_, a4_, l_depth_5) };
                                }
                            }
                            { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
                        }
                    }
                }
                l_link_3 = (l_n_2).wrapping_add(4);
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "StgRigidList_Collide", &PROBE_N, &[], true); }
    r_
}

pub fn StgRigidList_AddForces(c: &mut Ctx, p_head_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgRigidList_AddForces", &PROBE_N, &[(p_head_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_n_2: u32 = 0;
    let mut l_link_3: u32 = 0;
    c.m.w32(fp, p_head_1);
    let r_ = 'f: {
        {
            l_link_3 = fp;
            'l1: loop {
                if !(((((c.m.r32(l_link_3)) != (((0i32) as u32))) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_n_2 = c.m.r32(l_link_3);
                    if (((c.m.r32((l_n_2).wrapping_add(560)) as i32)) != 0) {
                        break 'c2;
                    }
                    F::Rigid_AddGravity(c, (l_n_2).wrapping_add(16));
                    { let a1_ = F32(c.m.r32((l_n_2).wrapping_add(8))); F::Rigid_Damp(c, (l_n_2).wrapping_add(16), a1_) };
                }
                l_link_3 = (l_n_2).wrapping_add(4);
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "StgRigidList_AddForces", &PROBE_N, &[], true); }
    r_
}

pub fn StgRigidList_Step(c: &mut Ctx, p_head_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgRigidList_Step", &PROBE_N, &[(p_head_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_n_2: u32 = 0;
    let mut l_link_3: u32 = 0;
    c.m.w32(fp, p_head_1);
    let r_ = 'f: {
        {
            l_link_3 = fp;
            'l1: loop {
                if !(((((c.m.r32(l_link_3)) != (((0i32) as u32))) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_n_2 = c.m.r32(l_link_3);
                    if (((c.m.r32((l_n_2).wrapping_add(560)) as i32)) != 0) {
                        break 'c2;
                    }
                    F::Rigid_Step(c, (l_n_2).wrapping_add(16), F32(0x3c889d80));
                }
                l_link_3 = (l_n_2).wrapping_add(4);
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "StgRigidList_Step", &PROBE_N, &[], true); }
    r_
}

pub fn StgRigidList_EndFrame(c: &mut Ctx, p_head_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgRigidList_EndFrame", &PROBE_N, &[(p_head_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_n_2: u32 = 0;
    let mut l_link_3: u32 = 0;
    c.m.w32(fp, p_head_1);
    let r_ = 'f: {
        {
            l_link_3 = fp;
            'l1: loop {
                if !(((((c.m.r32(l_link_3)) != (((0i32) as u32))) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_n_2 = c.m.r32(l_link_3);
                    if (((c.m.r32((l_n_2).wrapping_add(560)) as i32)) != 0) {
                        break 'c2;
                    }
                    F::StgRigid_ClampToStage(c, (l_n_2).wrapping_add(16));
                    if ((((((c.m.r32(((l_n_2).wrapping_add(16)).wrapping_add(228)) as i32)) != (0i32)) as i32)) != 0) {
                        { let v_ = F::Rigid_CheckRest(c, (l_n_2).wrapping_add(16)); let a_ = (l_n_2).wrapping_add(560); c.m.w32(a_, (v_) as u32); };
                    } else {
                        { let v_ = 0i32; let a_ = (l_n_2).wrapping_add(560); c.m.w32(a_, (v_) as u32); };
                    }
                }
                l_link_3 = (l_n_2).wrapping_add(4);
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "StgRigidList_EndFrame", &PROBE_N, &[], true); }
    r_
}

pub fn StgRigid_Reset(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgRigid_Reset", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let v_ = 0i32; let a_ = (c.m.r32(sym::gStgRigid)).wrapping_add(73736); c.m.w32(a_, (v_) as u32); };
    { let v_ = ((0i32) as u32); let a_ = (c.m.r32(sym::gStgRigid)).wrapping_add(73728); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (c.m.r32(sym::gStgRigid)).wrapping_add(73732); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgRigid_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn StgRigid_Init(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgRigid_Init", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let r_ = 'f: {
    { let v_ = F::Heap_Alloc(c, ((73744u32) as i32), ((32i32) as u32), 0i32, 2i32); let a_ = sym::gStgRigid; c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32(sym::gStgRigid); F::memset(c, a0_, 0i32, 73744u32) }; };
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (128i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let v_ = (l_i_1).wrapping_add(1i32); let a_ = ((c.m.r32(sym::gStgRigid)).wrapping_add((((l_i_1) as u32)).wrapping_mul(576))).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
    F::StgRigid_Reset(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgRigid_Init", &PROBE_N, &[], true); }
    r_
}

pub fn StgRigid_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgRigid_Term", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if (((((c.m.r32(sym::gStgRigid)) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32(sym::gStgRigid); F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = sym::gStgRigid; c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgRigid_Term", &PROBE_N, &[], true); }
    r_
}

pub fn StgRigid_Create(c: &mut Ctx, mut p_pos_1: u32, mut p_radius_2: F32, mut p_user_3: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgRigid_Create", &PROBE_N, &[(p_pos_1) as u32, (p_radius_2).0, (p_user_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(64) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 64);
    let mut l_n_4: u32 = 0;
    let mut l_link_5: u32 = 0;
    let mut l_last_6: u32 = 0;
    let r_ = 'f: {
        { let _ = F::memset(c, fp, 0i32, 64u32); };
        { let v_ = F32(0x3f800000); let a_ = ((fp).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = ((fp).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = ((fp).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = ((fp).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        l_n_4 = F::StgRigid_AllocNode(c);
        if (((((l_n_4) == (((0i32) as u32))) as i32)) != 0) {
            break 'f (1i32).wrapping_neg();
        }
        l_last_6 = { let a0_ = { let a_ = (c.m.r32(sym::gStgRigid)).wrapping_add(73732); c.m.r32(a_) }; F::StgRigidList_Last(c, a0_) };
        if (((((l_last_6) == (((0i32) as u32))) as i32)) != 0) {
            { let v_ = l_n_4; let a_ = (c.m.r32(sym::gStgRigid)).wrapping_add(73732); c.m.w32(a_, v_); };
            { let v_ = ((0i32) as u32); let a_ = l_n_4; c.m.w32(a_, v_); };
        } else {
            { let v_ = l_n_4; let a_ = (l_last_6).wrapping_add(4); c.m.w32(a_, v_); };
            { let v_ = l_last_6; let a_ = l_n_4; c.m.w32(a_, v_); };
        }
        { let v_ = ((0i32) as u32); let a_ = (l_n_4).wrapping_add(4); c.m.w32(a_, v_); };
        { let a3_ = F32(c.m.r32(sym::gStgRigidDensity)); F::Rigid_Init(c, (l_n_4).wrapping_add(16), p_user_3, p_radius_2, a3_) };
        F::Rigid_SetTransform(c, (l_n_4).wrapping_add(16), p_pos_1, fp);
        F::Rigid_SyncSpheres(c, (l_n_4).wrapping_add(16));
        { let v_ = F32(0x3ac49ba5); let a_ = (l_n_4).wrapping_add(8); c.m.w32(a_, (v_).0); };
        break 'f (c.m.r32((l_n_4).wrapping_add(12)) as i32);
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "StgRigid_Create", &PROBE_N, &[], true); }
    r_
}

pub fn StgRigid_Launch(c: &mut Ctx, mut p_id_1: i32, mut p_push_2: u32, mut p_material_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgRigid_Launch", &PROBE_N, &[(p_id_1) as u32, (p_push_2) as u32, (p_material_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_n_4: u32 = 0;
    let mut l_link_5: u32 = 0;
    let mut l_up_6: F32 = F32(0);
    let mut l_d_7: F32 = F32(0);
    let r_ = 'f: {
        { let _ = F::memset(c, fp.wrapping_add(16), 0i32, 16u32); };
        { let v_ = F32(0x3f800000); let a_ = (fp.wrapping_add(16)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(F::Rand_FloatRange(c, F32(0x00000000), F32(0x3f800000)), F32(0xbe999999)); let a_ = fp; c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(F::Rand_FloatRange(c, F32(0x00000000), F32(0x3f800000)), F32(0xbeb33333)); let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fmul(F::Rand_FloatRange(c, F32(0x00000000), F32(0x3f800000)), F32(0xbe800000)); let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = (fp).wrapping_add(12); c.m.w32(a_, (v_).0); };
        l_n_4 = F::StgRigid_Find(c, p_id_1);
        if (((((l_n_4) == (((0i32) as u32))) as i32)) != 0) {
            { let _ = F::BtlStage_IsReady(c); };
            break 'f;
        }
        F::Vec4_Copy(c, fp.wrapping_add(16), p_push_2);
        { let v_ = F32(0x00000000); let a_ = (fp.wrapping_add(16)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        l_d_7 = F::Vec3_Dot(c, fp.wrapping_add(16), fp.wrapping_add(16));
        if (((rt::flt(F32(0x44960000), l_d_7) as i32)) != 0) {
            l_d_7 = rt::fdiv(F32(0x44960000), l_d_7);
            { let r_ = l_d_7; let a_ = fp.wrapping_add(16); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            { let r_ = l_d_7; let a_ = (fp.wrapping_add(16)).wrapping_add(8); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        }
        's1: {
            let sv8: i32 = p_material_3;
            'k6: {
            'k5: {
            'k4: {
            'k3: {
            'k2: {
                match sv8 { 0i32 => break 'k2, 1i32 => break 'k3, 2i32 => break 'k4, 3i32 => break 'k5, _ => break 'k6, }
            }
            l_up_6 = F32(0xc1200000);
            break 's1;
            }
            l_up_6 = F32(0xc1880000);
            break 's1;
            }
            l_up_6 = F32(0xc1a80000);
            break 's1;
            }
            { let v_ = F32(0x00000000); let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x00000000); let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x00000000); let a_ = fp; c.m.w32(a_, (v_).0); };
            }
            l_up_6 = F32(0xc1200000);
            break 's1;
        }
        { let v_ = rt::fadd(rt::fmul(rt::fmul(l_up_6, F32(0x3f000000)), F::Rand_FloatRange(c, F32(0x00000000), F32(0x3f800000))), rt::fmul(l_up_6, F32(0x3f000000))); let a_ = (fp.wrapping_add(16)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        if ((((F::Port_Rand(c)) & (1i32))) != 0) {
            { let v_ = rt::fneg(F32(c.m.r32(fp))); let a_ = fp; c.m.w32(a_, (v_).0); };
        }
        if ((((F::Port_Rand(c)) & (1i32))) != 0) {
            { let v_ = rt::fneg(F32(c.m.r32((fp).wrapping_add(4)))); let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
        }
        if ((((F::Port_Rand(c)) & (1i32))) != 0) {
            { let v_ = rt::fneg(F32(c.m.r32((fp).wrapping_add(8)))); let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_).0); };
        }
        if ((((F::Port_Rand(c)) & (1i32))) != 0) {
            { let r_ = rt::fmul(F32(c.m.r32(fp.wrapping_add(16))), F::Rand_FloatRange(c, F32(0x00000000), F32(0x3f800000))); let a_ = fp.wrapping_add(16); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        }
        if ((((F::Port_Rand(c)) & (1i32))) != 0) {
            { let r_ = rt::fmul(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))), F::Rand_FloatRange(c, F32(0x00000000), F32(0x3f800000))); let a_ = (fp.wrapping_add(16)).wrapping_add(4); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        }
        if ((((F::Port_Rand(c)) & (1i32))) != 0) {
            { let r_ = rt::fmul(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8))), F::Rand_FloatRange(c, F32(0x00000000), F32(0x3f800000))); let a_ = (fp.wrapping_add(16)).wrapping_add(8); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        }
        F::Rigid_SetVelocity(c, (l_n_4).wrapping_add(16), fp.wrapping_add(16), fp);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "StgRigid_Launch", &PROBE_N, &[], true); }
    r_
}

pub fn StgRigid_LaunchSpin(c: &mut Ctx, mut p_id_1: i32, mut p_push_2: u32, mut p_angVel_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgRigid_LaunchSpin", &PROBE_N, &[(p_id_1) as u32, (p_push_2) as u32, (p_angVel_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_n_4: u32 = 0;
    let mut l_link_5: u32 = 0;
    let mut l_d_6: F32 = F32(0);
    let r_ = 'f: {
        { let _ = F::memset(c, fp, 0i32, 16u32); };
        { let v_ = F32(0x3f800000); let a_ = (fp).wrapping_add(12); c.m.w32(a_, (v_).0); };
        l_n_4 = F::StgRigid_Find(c, p_id_1);
        if (((((l_n_4) == (((0i32) as u32))) as i32)) != 0) {
            { let _ = F::BtlStage_IsReady(c); };
            break 'f;
        }
        F::Vec4_Copy(c, fp, p_push_2);
        { let v_ = F32(0x00000000); let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
        l_d_6 = F::Vec3_Dot(c, fp, fp);
        if (((rt::flt(F32(0x44960000), l_d_6) as i32)) != 0) {
            l_d_6 = rt::fdiv(F32(0x44960000), l_d_6);
            { let r_ = l_d_6; let a_ = fp; let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            { let r_ = l_d_6; let a_ = (fp).wrapping_add(8); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        }
        { let v_ = rt::fadd(rt::fmul(F::Rand_FloatRange(c, F32(0x00000000), F32(0x3f800000)), F32(0xc0a00000)), F32(0xc0a00000)); let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
        if ((((F::Port_Rand(c)) & (1i32))) != 0) {
            { let r_ = rt::fmul(F32(c.m.r32(fp)), F::Rand_FloatRange(c, F32(0x00000000), F32(0x3f800000))); let a_ = fp; let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        }
        if ((((F::Port_Rand(c)) & (1i32))) != 0) {
            { let r_ = rt::fmul(F32(c.m.r32((fp).wrapping_add(4))), F::Rand_FloatRange(c, F32(0x00000000), F32(0x3f800000))); let a_ = (fp).wrapping_add(4); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        }
        if ((((F::Port_Rand(c)) & (1i32))) != 0) {
            { let r_ = rt::fmul(F32(c.m.r32((fp).wrapping_add(8))), F::Rand_FloatRange(c, F32(0x00000000), F32(0x3f800000))); let a_ = (fp).wrapping_add(8); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        }
        F::Rigid_SetVelocity(c, (l_n_4).wrapping_add(16), fp, p_angVel_3);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "StgRigid_LaunchSpin", &PROBE_N, &[], true); }
    r_
}

pub fn StgRigid_Release(c: &mut Ctx, mut p_id_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgRigid_Release", &PROBE_N, &[(p_id_1) as u32], false); }
    let mut l_n_2: u32 = 0;
    let r_ = 'f: {
    l_n_2 = F::StgRigid_Find(c, p_id_1);
    if (((((l_n_2) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    if ((((({ let a_ = (c.m.r32(sym::gStgRigid)).wrapping_add(73732); c.m.r32(a_) }) == (l_n_2)) as i32)) != 0) {
        { let v_ = c.m.r32((l_n_2).wrapping_add(4)); let a_ = (c.m.r32(sym::gStgRigid)).wrapping_add(73732); c.m.w32(a_, v_); };
    } else {
        if (((((c.m.r32(l_n_2)) != (((0i32) as u32))) as i32)) != 0) {
            { let v_ = c.m.r32((l_n_2).wrapping_add(4)); let a_ = (c.m.r32(l_n_2)).wrapping_add(4); c.m.w32(a_, v_); };
        }
        if (((((c.m.r32((l_n_2).wrapping_add(4))) != (((0i32) as u32))) as i32)) != 0) {
            { let v_ = c.m.r32(l_n_2); let a_ = c.m.r32((l_n_2).wrapping_add(4)); c.m.w32(a_, v_); };
        }
    }
    { let v_ = { let a_ = (c.m.r32(sym::gStgRigid)).wrapping_add(73728); c.m.r32(a_) }; let a_ = (l_n_2).wrapping_add(4); c.m.w32(a_, v_); };
    { let v_ = l_n_2; let a_ = (c.m.r32(sym::gStgRigid)).wrapping_add(73728); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgRigid_Release", &PROBE_N, &[], true); }
    r_
}

pub fn StgRigid_Update(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgRigid_Update", &PROBE_N, &[], false); }
    let mut l_t_1: F32 = F32(0);
    let r_ = 'f: {
    l_t_1 = F32(0x00000000);
    if (((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0) {
        break 'f;
    }
    { let a0_ = { let a_ = (c.m.r32(sym::gStgRigid)).wrapping_add(73732); c.m.r32(a_) }; F::StgRigidList_BeginFrame(c, a0_) };
    'l1: loop {
        'c2: {
            { let r_ = F32(0x3c889d80); let n_ = rt::fadd(l_t_1, r_); l_t_1 = n_; };
            { let a0_ = { let a_ = (c.m.r32(sym::gStgRigid)).wrapping_add(73732); c.m.r32(a_) }; F::StgRigidList_UpdateCenters(c, a0_) };
            { let a0_ = { let a_ = (c.m.r32(sym::gStgRigid)).wrapping_add(73732); c.m.r32(a_) }; F::StgRigidList_Collide(c, a0_) };
            { let a0_ = { let a_ = (c.m.r32(sym::gStgRigid)).wrapping_add(73732); c.m.r32(a_) }; F::StgRigidList_AddForces(c, a0_) };
            { let a0_ = { let a_ = (c.m.r32(sym::gStgRigid)).wrapping_add(73732); c.m.r32(a_) }; F::StgRigidList_Step(c, a0_) };
        }
        if !(((rt::fle(l_t_1, F32(0x3e2aaaaa)) as i32)) != 0) { break 'l1; }
    }
    { let a0_ = { let a_ = (c.m.r32(sym::gStgRigid)).wrapping_add(73732); c.m.r32(a_) }; F::StgRigidList_EndFrame(c, a0_) };
    { let v_ = { let a0_ = { let a_ = (c.m.r32(sym::gStgRigid)).wrapping_add(73732); c.m.r32(a_) }; F::StgRigidList_Count(c, a0_) }; let a_ = (c.m.r32(sym::gStgRigid)).wrapping_add(73740); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgRigid_Update", &PROBE_N, &[], true); }
    r_
}

pub fn StgRigid_GetMatrix(c: &mut Ctx, mut p_out_1: u32, mut p_id_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgRigid_GetMatrix", &PROBE_N, &[(p_out_1) as u32, (p_id_2) as u32], false); }
    let mut l_n_3: u32 = 0;
    let r_ = 'f: {
    l_n_3 = F::StgRigid_Find(c, p_id_2);
    if (((((l_n_3) != (((0i32) as u32))) as i32)) != 0) {
        F::Rigid_GetMatrix(c, p_out_1, (l_n_3).wrapping_add(16));
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "StgRigid_GetMatrix", &PROBE_N, &[], true); }
    r_
}

pub fn StgRigid_GetSphere(c: &mut Ctx, mut p_id_1: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgRigid_GetSphere", &PROBE_N, &[(p_id_1) as u32], false); }
    let mut l_n_2: u32 = 0;
    let r_ = 'f: {
    l_n_2 = F::StgRigid_Find(c, p_id_1);
    if (((((l_n_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f ((l_n_2).wrapping_add(16)).wrapping_add(464);
    }
    break 'f ((0i32) as u32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "StgRigid_GetSphere", &PROBE_N, &[], true); }
    r_
}

pub fn StgAabb_SetEmpty(c: &mut Ctx, mut p_box_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgAabb_SetEmpty", &PROBE_N, &[(p_box_1) as u32], false); }
    let mut l_big_2: F32 = F32(0);
    let r_ = 'f: {
    l_big_2 = F32(0x501502f9);
    { let v_ = l_big_2; let a_ = (p_box_1).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = l_big_2; let a_ = (p_box_1).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = l_big_2; let a_ = (p_box_1).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fneg(l_big_2); let a_ = ((p_box_1).wrapping_add(12)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fneg(l_big_2); let a_ = ((p_box_1).wrapping_add(12)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fneg(l_big_2); let a_ = ((p_box_1).wrapping_add(12)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgAabb_SetEmpty", &PROBE_N, &[], true); }
    r_
}
