#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn StgOfsTable_Relocate(c: &mut Ctx, mut p_t_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgOfsTable_Relocate", &PROBE_N, &[(p_t_1) as u32], false); }
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
    if ((((((c.m.r32((p_t_1).wrapping_add(12)) as i32)) == (0i32)) as i32)) != 0) {
        {
            l_i_2 = 0i32;
            'l1: loop {
                if !(((((l_i_2) < ((c.m.r32((p_t_1).wrapping_add(8)) as i32))) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let r_ = ((p_t_1) as i32); let a_ = ((p_t_1).wrapping_add(16)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        { let v_ = 1i32; let a_ = (p_t_1).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgOfsTable_Relocate", &PROBE_N, &[], true); }
    r_
}

pub fn StgOfsTable_Get(c: &mut Ctx, mut p_t_1: u32, mut p_i_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgOfsTable_Get", &PROBE_N, &[(p_t_1) as u32, (p_i_2) as u32], false); }
    let r_ = 'f: {
    if (((((p_i_2) < ((c.m.r32((p_t_1).wrapping_add(8)) as i32))) as i32)) != 0) {
        break 'f (c.m.r32(((p_t_1).wrapping_add(16)).wrapping_add((((p_i_2) as u32)).wrapping_mul(4))) as i32);
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "StgOfsTable_Get", &PROBE_N, &[], true); }
    r_
}

pub fn StgOfsTable_GetCount(c: &mut Ctx, mut p_t_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgOfsTable_GetCount", &PROBE_N, &[(p_t_1) as u32], false); }
    let r_ = 'f: {
    break 'f (c.m.r32((p_t_1).wrapping_add(8)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "StgOfsTable_GetCount", &PROBE_N, &[], true); }
    r_
}

pub fn StgPathTable_Relocate(c: &mut Ctx, mut p_t_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgPathTable_Relocate", &PROBE_N, &[(p_t_1) as u32], false); }
    let mut l_i_2: i32 = 0;
    let mut l_p_3: u32 = 0;
    let r_ = 'f: {
    if ((((((c.m.r32((p_t_1).wrapping_add(16)) as i32)) == (0i32)) as i32)) != 0) {
        { let v_ = (((((c.m.r32((p_t_1).wrapping_add(12))) as i32)).wrapping_add(((p_t_1) as i32))) as u32); let a_ = (p_t_1).wrapping_add(12); c.m.w32(a_, v_); };
        l_p_3 = c.m.r32((p_t_1).wrapping_add(12));
        {
            l_i_2 = 0i32;
            'l1: loop {
                if !(((((l_i_2) < ((c.m.r32((p_t_1).wrapping_add(8)) as i32))) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let r_ = ((p_t_1) as i32); let a_ = (l_p_3).wrapping_add(4); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                }
                { { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; }; { let o_ = l_p_3; let n_ = (o_).wrapping_add(16); l_p_3 = n_; }; };
            }
        }
        { let v_ = 1i32; let a_ = (p_t_1).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgPathTable_Relocate", &PROBE_N, &[], true); }
    r_
}

pub fn BtlStage_IsReady(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_IsReady", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if (((((c.m.r32(sym::gBtlStage)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if (((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((8192i32) as u64)))) != 0) {
        break 'f 0i32;
    }
    if (((({ let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(8); (c.m.r32(a_) as i32) }) & (1i32))) != 0) {
        break 'f 0i32;
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_IsReady", &PROBE_N, &[], true); }
    r_
}

pub fn BtlStage_Reset(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_Reset", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let v_ = 0i32; let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
    { let a0_ = c.m.r32(sym::gBtlStage); F::BtlStage_BindFile(c, a0_) };
    F::BtlStage_ResetObjs(c, 0i32);
    F::StgModel_ResetAnims(c);
    F::StgRigid_Reset(c);
    F::StgAmb_Start(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn BtlStage_Init(c: &mut Ctx, mut p_stage_1: u32, mut p_arg_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_Init", &PROBE_N, &[(p_stage_1) as u32, (p_arg_2) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_stage_1; let a_ = sym::gBtlStage; c.m.w32(a_, v_); };
    F::BtlStage_Relocate(c, p_arg_2);
    F::ColMesh_SetBase(c, p_arg_2);
    { let v_ = 0i32; let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
    { let a0_ = c.m.r32(sym::gBtlStage); F::BtlStage_BindFile(c, a0_) };
    F::StgModel_BindAnims(c);
    F::StgRigid_Init(c);
    F::BtlStage_Reset(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_Init", &PROBE_N, &[], true); }
    r_
}

pub fn BtlStage_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_Term", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::StgRigid_Term(c);
    { let v_ = ((0i32) as u32); let a_ = sym::gBtlStage; c.m.w32(a_, v_); };
    F::StgAmb_Stop(c);
    F::StgModel_ClearAnims(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_Term", &PROBE_N, &[], true); }
    r_
}

pub fn StgDbg_SetState(c: &mut Ctx, mut p_state_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgDbg_SetState", &PROBE_N, &[(p_state_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_state_1; let a_ = sym::gStgDbgState; c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgDbg_SetState", &PROBE_N, &[], true); }
    r_
}

pub fn StgDbg_Bind(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgDbg_Bind", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::StgDbg_BindFile(c);
    F::StgFx_Reset(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgDbg_Bind", &PROBE_N, &[], true); }
    r_
}

pub fn StgDbg_PickRandomPlace(c: &mut Ctx, mut p_pos_1: u32, mut p_target_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgDbg_PickRandomPlace", &PROBE_N, &[(p_pos_1) as u32, (p_target_2) as u32], false); }
    let mut l_n_3: i32 = 0;
    let r_ = 'f: {
    l_n_3 = rt::rem_i32(F::Port_Rand(c), 6i32);
    F::Vec4_Copy(c, p_pos_1, (sym::gStgDbgPlaces).wrapping_add((((l_n_3) as u32)).wrapping_mul(32)));
    F::Vec4_Copy(c, p_target_2, ((sym::gStgDbgPlaces).wrapping_add((((l_n_3) as u32)).wrapping_mul(32))).wrapping_add(16));
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgDbg_PickRandomPlace", &PROBE_N, &[], true); }
    r_
}

pub fn StgDbg_GetState(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgDbg_GetState", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f (c.m.r32(sym::gStgDbgState) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "StgDbg_GetState", &PROBE_N, &[], true); }
    r_
}

pub fn StgDbg_ClearState(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgDbg_ClearState", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let v_ = 0i32; let a_ = sym::gStgDbgState; c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgDbg_ClearState", &PROBE_N, &[], true); }
    r_
}

pub fn BtlStage_FindZone(c: &mut Ctx, mut p_pos_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_FindZone", &PROBE_N, &[(p_pos_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_i_2: u32 = 0;
    let mut l_zones_3: u32 = 0;
    let r_ = 'f: {
        l_zones_3 = { let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(60); c.m.r32(a_) };
        {
            l_i_2 = ((0i32) as u32);
            'l1: loop {
                if !(((((l_i_2) < ({ let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(56); c.m.r32(a_) })) as i32)) != 0) { break 'l1; }
                'c2: {
                    if ((((!(({ { { { { let v_ = rt::fsub(F32(c.m.r32(((l_zones_3).wrapping_add((l_i_2).wrapping_mul(64))).wrapping_add(32))), F32(c.m.r32(((l_zones_3).wrapping_add((l_i_2).wrapping_mul(64))).wrapping_add(48)))); let a_ = fp; c.m.w32(a_, (v_).0); }; { let v_ = rt::fsub(F32(c.m.r32((((l_zones_3).wrapping_add((l_i_2).wrapping_mul(64))).wrapping_add(32)).wrapping_add(8))), F32(c.m.r32((((l_zones_3).wrapping_add((l_i_2).wrapping_mul(64))).wrapping_add(48)).wrapping_add(8)))); let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_).0); }; }; { let v_ = rt::fadd(F32(c.m.r32(((l_zones_3).wrapping_add((l_i_2).wrapping_mul(64))).wrapping_add(32))), F32(c.m.r32(((l_zones_3).wrapping_add((l_i_2).wrapping_mul(64))).wrapping_add(48)))); let a_ = fp.wrapping_add(16); c.m.w32(a_, (v_).0); }; }; { let v_ = rt::fadd(F32(c.m.r32((((l_zones_3).wrapping_add((l_i_2).wrapping_mul(64))).wrapping_add(32)).wrapping_add(8))), F32(c.m.r32((((l_zones_3).wrapping_add((l_i_2).wrapping_mul(64))).wrapping_add(48)).wrapping_add(8)))); let a_ = (fp.wrapping_add(16)).wrapping_add(8); c.m.w32(a_, (v_).0); }; }; (((((((((((((rt::flt(F32(c.m.r32(fp.wrapping_add(16))), F32(c.m.r32(p_pos_1))) as i32)) != 0) || (((rt::flt(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8))), F32(c.m.r32((p_pos_1).wrapping_add(8)))) as i32)) != 0)) as i32)) != 0) || (((rt::flt(F32(c.m.r32(p_pos_1)), F32(c.m.r32(fp))) as i32)) != 0)) as i32)) != 0) || (((rt::flt(F32(c.m.r32((p_pos_1).wrapping_add(8))), F32(c.m.r32((fp).wrapping_add(8)))) as i32)) != 0)) as i32) }) != 0)) as i32)) != 0) {
                        break 'f ((l_i_2) as i32);
                    }
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        break 'f (1i32).wrapping_neg();
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_FindZone", &PROBE_N, &[], true); }
    r_
}

pub fn BtlStage_GetRadius(c: &mut Ctx) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_GetRadius", &PROBE_N, &[], false); }
    let mut l_b_1: u32 = 0;
    let r_ = 'f: {
    if ((((!((F::BtlStage_IsReady(c)) != 0)) as i32)) != 0) {
        break 'f F32(c.m.r32(sym::gStgLastRadius));
    }
    l_b_1 = { let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(64); c.m.r32(a_) };
    { let v_ = F32(c.m.r32(l_b_1)); let a_ = sym::gStgLastRadius; c.m.w32(a_, (v_).0); };
    break 'f F32(c.m.r32(l_b_1));
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_GetRadius", &PROBE_N, &[], true); }
    r_
}

pub fn BtlStage_GetInnerRadius(c: &mut Ctx) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_GetInnerRadius", &PROBE_N, &[], false); }
    let mut l_b_1: u32 = 0;
    let r_ = 'f: {
    if ((((!((F::BtlStage_IsReady(c)) != 0)) as i32)) != 0) {
        break 'f F32(c.m.r32(sym::gStgLastRadius));
    }
    l_b_1 = { let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(64); c.m.r32(a_) };
    { let v_ = F32(c.m.r32(l_b_1)); let a_ = sym::gStgLastRadius; c.m.w32(a_, (v_).0); };
    break 'f rt::fsub(F32(c.m.r32(l_b_1)), F32(0x42c80000));
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_GetInnerRadius", &PROBE_N, &[], true); }
    r_
}

pub fn BtlStage_GetTop(c: &mut Ctx) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_GetTop", &PROBE_N, &[], false); }
    let mut l_b_1: u32 = 0;
    let r_ = 'f: {
    if ((((!((F::BtlStage_IsReady(c)) != 0)) as i32)) != 0) {
        break 'f F32(c.m.r32(sym::gStgLastTop));
    }
    l_b_1 = { let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(64); c.m.r32(a_) };
    { let v_ = F32(c.m.r32((l_b_1).wrapping_add(4))); let a_ = sym::gStgLastTop; c.m.w32(a_, (v_).0); };
    break 'f F32(c.m.r32((l_b_1).wrapping_add(4)));
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_GetTop", &PROBE_N, &[], true); }
    r_
}

pub fn BtlStage_GetBottom(c: &mut Ctx) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_GetBottom", &PROBE_N, &[], false); }
    let mut l_b_1: u32 = 0;
    let r_ = 'f: {
    if ((((!((F::BtlStage_IsReady(c)) != 0)) as i32)) != 0) {
        break 'f F32(c.m.r32(sym::gStgLastBottom));
    }
    l_b_1 = { let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(64); c.m.r32(a_) };
    { let v_ = F32(c.m.r32((l_b_1).wrapping_add(8))); let a_ = sym::gStgLastBottom; c.m.w32(a_, (v_).0); };
    break 'f F32(c.m.r32((l_b_1).wrapping_add(8)));
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_GetBottom", &PROBE_N, &[], true); }
    r_
}

pub fn BtlStage_FindZoneNear(c: &mut Ctx, mut p_zone_1: i32, mut p_pos_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_FindZoneNear", &PROBE_N, &[(p_zone_1) as u32, (p_pos_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_zones_3: u32 = 0;
    let mut l_near_4: u32 = 0;
    let mut l_i_5: i32 = 0;
    let mut l_r_6: i32 = 0;
    let r_ = 'f: {
        l_zones_3 = { let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(60); c.m.r32(a_) };
        if (((((((((((p_zone_1) as u32)) < ({ let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(56); c.m.r32(a_) })) as i32)) != 0) && (((((p_zone_1) >= (0i32)) as i32)) != 0)) as i32)) != 0) {
            if ((((!(({ { { { { let v_ = rt::fsub(F32(c.m.r32(((l_zones_3).wrapping_add((((p_zone_1) as u32)).wrapping_mul(64))).wrapping_add(32))), F32(c.m.r32(((l_zones_3).wrapping_add((((p_zone_1) as u32)).wrapping_mul(64))).wrapping_add(48)))); let a_ = fp; c.m.w32(a_, (v_).0); }; { let v_ = rt::fsub(F32(c.m.r32((((l_zones_3).wrapping_add((((p_zone_1) as u32)).wrapping_mul(64))).wrapping_add(32)).wrapping_add(8))), F32(c.m.r32((((l_zones_3).wrapping_add((((p_zone_1) as u32)).wrapping_mul(64))).wrapping_add(48)).wrapping_add(8)))); let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_).0); }; }; { let v_ = rt::fadd(F32(c.m.r32(((l_zones_3).wrapping_add((((p_zone_1) as u32)).wrapping_mul(64))).wrapping_add(32))), F32(c.m.r32(((l_zones_3).wrapping_add((((p_zone_1) as u32)).wrapping_mul(64))).wrapping_add(48)))); let a_ = fp.wrapping_add(16); c.m.w32(a_, (v_).0); }; }; { let v_ = rt::fadd(F32(c.m.r32((((l_zones_3).wrapping_add((((p_zone_1) as u32)).wrapping_mul(64))).wrapping_add(32)).wrapping_add(8))), F32(c.m.r32((((l_zones_3).wrapping_add((((p_zone_1) as u32)).wrapping_mul(64))).wrapping_add(48)).wrapping_add(8)))); let a_ = (fp.wrapping_add(16)).wrapping_add(8); c.m.w32(a_, (v_).0); }; }; (((((((((((((rt::flt(F32(c.m.r32(fp.wrapping_add(16))), F32(c.m.r32(p_pos_2))) as i32)) != 0) || (((rt::flt(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8))), F32(c.m.r32((p_pos_2).wrapping_add(8)))) as i32)) != 0)) as i32)) != 0) || (((rt::flt(F32(c.m.r32(p_pos_2)), F32(c.m.r32(fp))) as i32)) != 0)) as i32)) != 0) || (((rt::flt(F32(c.m.r32((p_pos_2).wrapping_add(8))), F32(c.m.r32((fp).wrapping_add(8)))) as i32)) != 0)) as i32) }) != 0)) as i32)) != 0) {
                break 'f p_zone_1;
            }
            l_near_4 = c.m.r32(((l_zones_3).wrapping_add((((p_zone_1) as u32)).wrapping_mul(64))).wrapping_add(12));
            {
                l_i_5 = 0i32;
                'l1: loop {
                    if !(((((l_i_5) < ((c.m.r32(((l_zones_3).wrapping_add((((p_zone_1) as u32)).wrapping_mul(64))).wrapping_add(8)) as i32))) as i32)) != 0) { break 'l1; }
                    'c2: {
                        if ((((!(({ { { { { let v_ = rt::fsub({ let a_ = ((l_zones_3).wrapping_add(((((c.m.r32((l_near_4).wrapping_add((((l_i_5) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(64))).wrapping_add(32); F32(c.m.r32(a_)) }, { let a_ = ((l_zones_3).wrapping_add(((((c.m.r32((l_near_4).wrapping_add((((l_i_5) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(64))).wrapping_add(48); F32(c.m.r32(a_)) }); let a_ = fp; c.m.w32(a_, (v_).0); }; { let v_ = rt::fsub({ let a_ = (((l_zones_3).wrapping_add(((((c.m.r32((l_near_4).wrapping_add((((l_i_5) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(64))).wrapping_add(32)).wrapping_add(8); F32(c.m.r32(a_)) }, { let a_ = (((l_zones_3).wrapping_add(((((c.m.r32((l_near_4).wrapping_add((((l_i_5) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(64))).wrapping_add(48)).wrapping_add(8); F32(c.m.r32(a_)) }); let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_).0); }; }; { let v_ = rt::fadd({ let a_ = ((l_zones_3).wrapping_add(((((c.m.r32((l_near_4).wrapping_add((((l_i_5) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(64))).wrapping_add(32); F32(c.m.r32(a_)) }, { let a_ = ((l_zones_3).wrapping_add(((((c.m.r32((l_near_4).wrapping_add((((l_i_5) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(64))).wrapping_add(48); F32(c.m.r32(a_)) }); let a_ = fp.wrapping_add(16); c.m.w32(a_, (v_).0); }; }; { let v_ = rt::fadd({ let a_ = (((l_zones_3).wrapping_add(((((c.m.r32((l_near_4).wrapping_add((((l_i_5) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(64))).wrapping_add(32)).wrapping_add(8); F32(c.m.r32(a_)) }, { let a_ = (((l_zones_3).wrapping_add(((((c.m.r32((l_near_4).wrapping_add((((l_i_5) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(64))).wrapping_add(48)).wrapping_add(8); F32(c.m.r32(a_)) }); let a_ = (fp.wrapping_add(16)).wrapping_add(8); c.m.w32(a_, (v_).0); }; }; (((((((((((((rt::flt(F32(c.m.r32(fp.wrapping_add(16))), F32(c.m.r32(p_pos_2))) as i32)) != 0) || (((rt::flt(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8))), F32(c.m.r32((p_pos_2).wrapping_add(8)))) as i32)) != 0)) as i32)) != 0) || (((rt::flt(F32(c.m.r32(p_pos_2)), F32(c.m.r32(fp))) as i32)) != 0)) as i32)) != 0) || (((rt::flt(F32(c.m.r32((p_pos_2).wrapping_add(8))), F32(c.m.r32((fp).wrapping_add(8)))) as i32)) != 0)) as i32) }) != 0)) as i32)) != 0) {
                            break 'f (c.m.r32((l_near_4).wrapping_add((((l_i_5) as u32)).wrapping_mul(4))) as i32);
                        }
                    }
                    { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
                }
            }
        }
        l_r_6 = F::BtlStage_FindZone(c, p_pos_2);
        if (((((l_r_6) == ((1i32).wrapping_neg())) as i32)) != 0) {
            l_r_6 = 0i32;
        }
        break 'f l_r_6;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_FindZoneNear", &PROBE_N, &[], true); }
    r_
}

pub fn BtlStage_GetZone(c: &mut Ctx, mut p_zone_1: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_GetZone", &PROBE_N, &[(p_zone_1) as u32], false); }
    let r_ = 'f: {
    break 'f ({ let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(60); c.m.r32(a_) }).wrapping_add((((p_zone_1) as u32)).wrapping_mul(64));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_GetZone", &PROBE_N, &[], true); }
    r_
}

pub fn BtlStage_ClearZoneMarks(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_ClearZoneMarks", &PROBE_N, &[], false); }
    let mut l_i_1: u32 = 0;
    let mut l_zones_2: u32 = 0;
    let mut l_z_3: u32 = 0;
    let r_ = 'f: {
    l_zones_2 = { let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(60); c.m.r32(a_) };
    {
        l_i_1 = ((0i32) as u32);
        'l1: loop {
            if !(((((l_i_1) < ({ let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(56); c.m.r32(a_) })) as i32)) != 0) { break 'l1; }
            'c2: {
                l_z_3 = (l_zones_2).wrapping_add((l_i_1).wrapping_mul(64));
                { let v_ = 0i32; let a_ = l_z_3; c.m.w32(a_, (v_) as u32); };
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_ClearZoneMarks", &PROBE_N, &[], true); }
    r_
}

pub fn StgFrustum_Build(c: &mut Ctx, mut p_view_1: u32, mut p_fr_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgFrustum_Build", &PROBE_N, &[(p_view_1) as u32, (p_fr_2) as u32], false); }
    let mut l_k_3: F32 = F32(0);
    let mut l_left_4: F32 = F32(0);
    let mut l_right_5: F32 = F32(0);
    let mut l_top_6: F32 = F32(0);
    let mut l_bottom_7: F32 = F32(0);
    let mut l_i_8: i32 = 0;
    let r_ = 'f: {
    { let _ = F::memset(c, p_fr_2, 0i32, 208u32); };
    if (((rt::fgt(rt::i2f(((c.m.r32((p_view_1).wrapping_add(520)) as i32)).wrapping_sub((c.m.r32((p_view_1).wrapping_add(512)) as i32))), F32(0x43800000)) as i32)) != 0) {
        { let v_ = F32(0x43d88000); let a_ = (p_fr_2).wrapping_add((((0i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = ((p_fr_2).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x43155555); let a_ = ((p_fr_2).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = ((p_fr_2).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0xc3d88000); let a_ = (p_fr_2).wrapping_add((((1i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = ((p_fr_2).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x43155555); let a_ = ((p_fr_2).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = ((p_fr_2).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (p_fr_2).wrapping_add((((2i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x43d88000); let a_ = ((p_fr_2).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x43600000); let a_ = ((p_fr_2).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = ((p_fr_2).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (p_fr_2).wrapping_add((((3i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0xc3d88000); let a_ = ((p_fr_2).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x43600000); let a_ = ((p_fr_2).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = ((p_fr_2).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (p_fr_2).wrapping_add((((4i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = ((p_fr_2).wrapping_add((((4i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = ((p_fr_2).wrapping_add((((4i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = ((p_fr_2).wrapping_add((((4i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
    } else {
        { let v_ = F32(0x43d88000); let a_ = (p_fr_2).wrapping_add((((0i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = ((p_fr_2).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x43955555); let a_ = ((p_fr_2).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = ((p_fr_2).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0xc3d88000); let a_ = (p_fr_2).wrapping_add((((1i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = ((p_fr_2).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x43955555); let a_ = ((p_fr_2).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = ((p_fr_2).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (p_fr_2).wrapping_add((((2i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x43d88000); let a_ = ((p_fr_2).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x43600000); let a_ = ((p_fr_2).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = ((p_fr_2).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (p_fr_2).wrapping_add((((3i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0xc3d88000); let a_ = ((p_fr_2).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x43600000); let a_ = ((p_fr_2).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = ((p_fr_2).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (p_fr_2).wrapping_add((((4i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = ((p_fr_2).wrapping_add((((4i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = ((p_fr_2).wrapping_add((((4i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = ((p_fr_2).wrapping_add((((4i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
    }
    { let r_ = F::Port_WideFactor(c); let a_ = ((p_fr_2).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
    { let r_ = F::Port_WideFactor(c); let a_ = ((p_fr_2).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
    {
        l_i_8 = 0i32;
        'l1: loop {
            if !(((((l_i_8) < (4i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                F::Vec3_Normalize(c, (p_fr_2).wrapping_add((((l_i_8) as u32)).wrapping_mul(16)), (p_fr_2).wrapping_add((((l_i_8) as u32)).wrapping_mul(16)));
            }
            { let o_ = l_i_8; let n_ = (o_).wrapping_add(1); l_i_8 = n_; };
        }
    }
    l_k_3 = rt::fdiv(F32(c.m.r32((p_view_1).wrapping_add(592))), F32(c.m.r32((p_view_1).wrapping_add(564))));
    l_top_6 = rt::fmul(rt::fneg(l_k_3), F32(0xc3600000));
    l_bottom_7 = rt::fmul(rt::fneg(l_k_3), F32(0x43600000));
    if (((rt::fgt(rt::i2f(((c.m.r32((p_view_1).wrapping_add(520)) as i32)).wrapping_sub((c.m.r32((p_view_1).wrapping_add(512)) as i32))), F32(0x43800000)) as i32)) != 0) {
        l_right_5 = rt::fmul(rt::fmul(rt::fmul(l_k_3, F32(0x43800000)), F32(0x3f000000)), F32(0x3f955555));
        l_left_4 = rt::fmul(l_k_3, F32(0xc3155555));
    } else {
        l_right_5 = rt::fmul(rt::fmul(l_k_3, F32(0x43800000)), F32(0x3f955555));
        l_left_4 = rt::fmul(l_k_3, F32(0xc3955555));
    }
    { let r_ = F::Port_WideFactor(c); let n_ = rt::fmul(l_right_5, r_); l_right_5 = n_; };
    { let r_ = F::Port_WideFactor(c); let n_ = rt::fmul(l_left_4, r_); l_left_4 = n_; };
    l_k_3 = rt::fdiv(F32(0x3f800000), rt::fsub(l_right_5, l_left_4));
    { let v_ = rt::fmul(rt::fadd(F32(c.m.r32((p_view_1).wrapping_add(592))), F32(c.m.r32((p_view_1).wrapping_add(592)))), l_k_3); let a_ = (((p_fr_2).wrapping_add(144)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x00000000); let a_ = (((p_fr_2).wrapping_add(144)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fmul(rt::fadd(l_right_5, l_left_4), l_k_3); let a_ = (((p_fr_2).wrapping_add(144)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x00000000); let a_ = (((p_fr_2).wrapping_add(144)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    l_k_3 = rt::fdiv(F32(0x3f800000), rt::fsub(l_top_6, l_bottom_7));
    { let v_ = F32(0x00000000); let a_ = (((p_fr_2).wrapping_add(144)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fmul(rt::fadd(F32(c.m.r32((p_view_1).wrapping_add(592))), F32(c.m.r32((p_view_1).wrapping_add(592)))), l_k_3); let a_ = (((p_fr_2).wrapping_add(144)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fmul(rt::fadd(l_top_6, l_bottom_7), l_k_3); let a_ = (((p_fr_2).wrapping_add(144)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x00000000); let a_ = (((p_fr_2).wrapping_add(144)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x00000000); let a_ = (((p_fr_2).wrapping_add(144)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x00000000); let a_ = (((p_fr_2).wrapping_add(144)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    l_k_3 = rt::fdiv(F32(0x3f800000), rt::fsub(F32(c.m.r32((p_view_1).wrapping_add(596))), F32(c.m.r32((p_view_1).wrapping_add(592)))));
    { let v_ = rt::fmul(rt::fneg(F32(c.m.r32((p_view_1).wrapping_add(592)))), l_k_3); let a_ = (((p_fr_2).wrapping_add(144)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fmul(rt::fneg(rt::fmul(F32(c.m.r32((p_view_1).wrapping_add(596))), F32(c.m.r32((p_view_1).wrapping_add(592))))), l_k_3); let a_ = (((p_fr_2).wrapping_add(144)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x00000000); let a_ = (((p_fr_2).wrapping_add(144)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x00000000); let a_ = (((p_fr_2).wrapping_add(144)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0xbf800000); let a_ = (((p_fr_2).wrapping_add(144)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x00000000); let a_ = (((p_fr_2).wrapping_add(144)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    F::Mtx_Copy(c, (p_fr_2).wrapping_add(80), (p_view_1).wrapping_add(64));
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgFrustum_Build", &PROBE_N, &[], true); }
    r_
}

pub fn StgFrustum_TestBox(c: &mut Ctx, mut p_fr_1: u32, mut p_x_2: F32, mut p_y_3: F32, mut p_z_4: F32, mut p_r_5: F32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgFrustum_TestBox", &PROBE_N, &[(p_fr_1) as u32, (p_x_2).0, (p_y_3).0, (p_z_4).0, (p_r_5).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_i_6: i32 = 0;
    let mut l_j_7: i32 = 0;
    let mut l_k_8: i32 = 0;
    let mut l_any_9: u64 = 0;
    let mut l_all_10: u64 = 0;
    let mut l_code_11: u64 = 0;
    let mut l_bit_12: u64 = 0;
    let r_ = 'f: {
        l_any_9 = ((0i32) as u64);
        l_all_10 = (((1i32).wrapping_neg()) as u64);
        {
            l_i_6 = 0i32;
            'l1: loop {
                if !(((((l_i_6) < (2i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    {
                        l_j_7 = 0i32;
                        'l3: loop {
                            if !(((((l_j_7) < (2i32)) as i32)) != 0) { break 'l3; }
                            'c4: {
                                {
                                    l_k_8 = 0i32;
                                    'l5: loop {
                                        if !(((((l_k_8) < (2i32)) as i32)) != 0) { break 'l5; }
                                        'c6: {
                                            l_code_11 = ((0i32) as u64);
                                            if (((((l_i_6) == (0i32)) as i32)) != 0) {
                                                { let v_ = rt::fsub(p_x_2, p_r_5); let a_ = fp.wrapping_add(16); c.m.w32(a_, (v_).0); };
                                            } else {
                                                { let v_ = rt::fadd(p_x_2, p_r_5); let a_ = fp.wrapping_add(16); c.m.w32(a_, (v_).0); };
                                            }
                                            if (((((l_j_7) == (0i32)) as i32)) != 0) {
                                                { let v_ = rt::fsub(p_y_3, p_r_5); let a_ = (fp.wrapping_add(16)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                                            } else {
                                                { let v_ = rt::fadd(p_y_3, p_r_5); let a_ = (fp.wrapping_add(16)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                                            }
                                            if (((((l_k_8) == (0i32)) as i32)) != 0) {
                                                { let v_ = rt::fsub(p_z_4, p_r_5); let a_ = (fp.wrapping_add(16)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                                            } else {
                                                { let v_ = rt::fadd(p_z_4, p_r_5); let a_ = (fp.wrapping_add(16)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                                            }
                                            { let v_ = F32(0x3f800000); let a_ = (fp.wrapping_add(16)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                                            F::Mtx_MulVec4(c, fp, (p_fr_1).wrapping_add(80), fp.wrapping_add(16));
                                            { let v_ = F32(0x3f800000); let a_ = (fp.wrapping_add(16)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                                            F::Mtx_MulVec4(c, fp.wrapping_add(16), (p_fr_1).wrapping_add(144), fp);
                                            { let a2_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(12))); F::Vec4_Div(c, fp.wrapping_add(16), fp.wrapping_add(16), a2_) };
                                            if (((rt::flt(F32(c.m.r32(fp.wrapping_add(16))), F32(0xbf800000)) as i32)) != 0) {
                                                l_code_11 = ((1i32) as u64);
                                            }
                                            l_bit_12 = ((2i32) as u64);
                                            if (((rt::fgt(F32(c.m.r32(fp.wrapping_add(16))), F32(0x3f800000)) as i32)) != 0) {
                                                { let r_ = l_bit_12; let n_ = ((l_code_11) | (r_)); l_code_11 = n_; };
                                            }
                                            l_bit_12 = ((4i32) as u64);
                                            if (((rt::flt(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))), F32(0xbf800000)) as i32)) != 0) {
                                                { let r_ = l_bit_12; let n_ = ((l_code_11) | (r_)); l_code_11 = n_; };
                                            }
                                            l_bit_12 = ((8i32) as u64);
                                            if (((rt::fgt(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))), F32(0x3f800000)) as i32)) != 0) {
                                                { let r_ = l_bit_12; let n_ = ((l_code_11) | (r_)); l_code_11 = n_; };
                                            }
                                            l_bit_12 = ((16i32) as u64);
                                            if (((rt::flt(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8))), F32(0x00000000)) as i32)) != 0) {
                                                { let r_ = l_bit_12; let n_ = ((l_code_11) | (r_)); l_code_11 = n_; };
                                            }
                                            l_bit_12 = ((32i32) as u64);
                                            if (((rt::fgt(F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8))), F32(0x3f800000)) as i32)) != 0) {
                                                { let r_ = l_bit_12; let n_ = ((l_code_11) | (r_)); l_code_11 = n_; };
                                            }
                                            { let r_ = l_code_11; let n_ = ((l_all_10) & (r_)); l_all_10 = n_; };
                                            { let r_ = l_code_11; let n_ = ((l_any_9) | (r_)); l_any_9 = n_; };
                                        }
                                        { let o_ = l_k_8; let n_ = (o_).wrapping_add(1); l_k_8 = n_; };
                                    }
                                }
                            }
                            { let o_ = l_j_7; let n_ = (o_).wrapping_add(1); l_j_7 = n_; };
                        }
                    }
                }
                { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; };
            }
        }
        if (((((l_any_9) == (((0i32) as u64))) as i32)) != 0) {
            break 'f 0i32;
        }
        break 'f (if (((((l_all_10) != (((0i32) as u64))) as i32)) != 0) { 2i32 } else { 1i32 });
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "StgFrustum_TestBox", &PROBE_N, &[], true); }
    r_
}

pub fn StgFrustum_TestPart(c: &mut Ctx, mut p_part_1: u32, mut p_fr_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgFrustum_TestPart", &PROBE_N, &[(p_part_1) as u32, (p_fr_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(64) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 64);
    let mut l_pl_3: u32 = 0;
    let mut l_node_4: u32 = 0;
    let mut l_dot_5: F32 = F32(0);
    let mut l_in_6: i32 = 0;
    let mut l_cross_7: i32 = 0;
    let mut l_count_8: i32 = 0;
    let mut l_key_9: u32 = 0;
    let mut bb_: u32 = 0;
    let r_ = 'f: {
        'fl1: loop {
            match bb_ {
                0 => {
                    l_in_6 = 1i32;
                    l_cross_7 = 0i32;
                    { let v_ = F32(c.m.r32((p_part_1).wrapping_add(16))); let a_ = fp.wrapping_add(16); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(c.m.r32((p_part_1).wrapping_add(20))); let a_ = (fp.wrapping_add(16)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(c.m.r32((p_part_1).wrapping_add(24))); let a_ = (fp.wrapping_add(16)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                    { let v_ = F32(0x3f800000); let a_ = (fp.wrapping_add(16)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                    l_node_4 = c.m.r32((p_part_1).wrapping_add(8));
                    if (((((l_node_4) != (((0i32) as u32))) as i32)) != 0) { bb_ = 1; continue 'fl1; } else { bb_ = 2; continue 'fl1; }
                }
                1 => {
                    l_count_8 = (c.m.r32((l_node_4).wrapping_add(64)) as i32);
                    l_key_9 = c.m.r32((l_node_4).wrapping_add(68));
                    if (((((l_count_8) != (0i32)) as i32)) != 0) { bb_ = 3; continue 'fl1; } else { bb_ = 4; continue 'fl1; }
                }
                2 => {
                    F::Mtx_MulVec4(c, fp, (p_fr_2).wrapping_add(80), fp.wrapping_add(16));
                    l_pl_3 = (p_fr_2).wrapping_add((((4i32) as u32)).wrapping_mul(16));
                    { let v_ = F32(0x3f800000); let a_ = (fp).wrapping_add(12); c.m.w32(a_, (v_).0); };
                    { let a2_ = F32(c.m.r32((p_part_1).wrapping_add(12))); F::Vec4_Scale(c, fp.wrapping_add(32), l_pl_3, a2_) };
                    F::Vec4_Add(c, fp.wrapping_add(32), fp, fp.wrapping_add(32));
                    if (((rt::flt(F::Vec3_Dot(c, fp.wrapping_add(32), l_pl_3), F32(0x00000000)) as i32)) != 0) { bb_ = 5; continue 'fl1; } else { bb_ = 7; continue 'fl1; }
                }
                3 => {
                    { let r_ = F32(c.m.r32(l_key_9)); let a_ = fp.wrapping_add(16); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    { let r_ = F32(c.m.r32((l_key_9).wrapping_add(4))); let a_ = (fp.wrapping_add(16)).wrapping_add(4); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    { let r_ = F32(c.m.r32((l_key_9).wrapping_add(8))); let a_ = (fp.wrapping_add(16)).wrapping_add(8); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    bb_ = 4; continue 'fl1;
                }
                4 => {
                    bb_ = 2; continue 'fl1;
                }
                5 => {
                    l_in_6 = 0i32;
                    bb_ = 6; continue 'fl1;
                }
                6 => {
                    bb_ = 10; continue 'fl1;
                }
                7 => {
                    F::Mtx_MulVec4(c, fp.wrapping_add(48), (p_fr_2).wrapping_add(144), fp);
                    { let a2_ = F32(c.m.r32((fp.wrapping_add(48)).wrapping_add(12))); F::Vec4_Div(c, fp.wrapping_add(48), fp.wrapping_add(48), a2_) };
                    if (((((((((((((((rt::fle(F32(0xbf800000), F32(c.m.r32(fp.wrapping_add(48)))) as i32)) != 0) && (((rt::fle(F32(c.m.r32(fp.wrapping_add(48))), F32(0x3f800000)) as i32)) != 0)) as i32)) != 0) && (((rt::fle(F32(0xbf800000), F32(c.m.r32((fp.wrapping_add(48)).wrapping_add(4)))) as i32)) != 0)) as i32)) != 0) && (((rt::fle(F32(c.m.r32((fp.wrapping_add(48)).wrapping_add(4))), F32(0x3f800000)) as i32)) != 0)) as i32)) != 0) { bb_ = 8; continue 'fl1; } else { bb_ = 9; continue 'fl1; }
                }
                8 => {
                    bb_ = 10; continue 'fl1;
                }
                9 => {
                    if (((((((rt::fle(F32(0xbf800000), F32(c.m.r32(fp.wrapping_add(48)))) as i32)) != 0) && (((rt::fle(F32(c.m.r32(fp.wrapping_add(48))), F32(0x3f800000)) as i32)) != 0)) as i32)) != 0) { bb_ = 12; continue 'fl1; } else { bb_ = 14; continue 'fl1; }
                }
                10 => {
                    break 'f (if ((l_in_6) != 0) { l_cross_7 } else { 2i32 });
                }
                11 => {
                    bb_ = 9; continue 'fl1;
                }
                12 => {
                    if (((rt::flt(F32(c.m.r32((fp.wrapping_add(48)).wrapping_add(4))), F32(0xbf800000)) as i32)) != 0) { bb_ = 15; continue 'fl1; } else { bb_ = 17; continue 'fl1; }
                }
                13 => {
                    bb_ = 6; continue 'fl1;
                }
                14 => {
                    if (((rt::flt(F32(c.m.r32(fp.wrapping_add(48))), F32(0xbf800000)) as i32)) != 0) { bb_ = 26; continue 'fl1; } else { bb_ = 28; continue 'fl1; }
                }
                15 => {
                    l_pl_3 = (p_fr_2).wrapping_add((((3i32) as u32)).wrapping_mul(16));
                    { let a2_ = F32(c.m.r32((p_part_1).wrapping_add(12))); F::Vec4_Scale(c, fp.wrapping_add(32), l_pl_3, a2_) };
                    F::Vec4_Add(c, fp.wrapping_add(32), fp, fp.wrapping_add(32));
                    l_dot_5 = F::Vec3_Dot(c, fp.wrapping_add(32), l_pl_3);
                    if (((rt::flt(l_dot_5, F32(0x00000000)) as i32)) != 0) { bb_ = 18; continue 'fl1; } else { bb_ = 20; continue 'fl1; }
                }
                16 => {
                    bb_ = 13; continue 'fl1;
                }
                17 => {
                    if (((rt::fgt(F32(c.m.r32((fp.wrapping_add(48)).wrapping_add(4))), F32(0x3f800000)) as i32)) != 0) { bb_ = 21; continue 'fl1; } else { bb_ = 22; continue 'fl1; }
                }
                18 => {
                    l_in_6 = 0i32;
                    bb_ = 19; continue 'fl1;
                }
                19 => {
                    bb_ = 16; continue 'fl1;
                }
                20 => {
                    l_cross_7 = 1i32;
                    bb_ = 19; continue 'fl1;
                }
                21 => {
                    l_pl_3 = (p_fr_2).wrapping_add((((2i32) as u32)).wrapping_mul(16));
                    { let a2_ = F32(c.m.r32((p_part_1).wrapping_add(12))); F::Vec4_Scale(c, fp.wrapping_add(32), l_pl_3, a2_) };
                    F::Vec4_Add(c, fp.wrapping_add(32), fp, fp.wrapping_add(32));
                    l_dot_5 = F::Vec3_Dot(c, fp.wrapping_add(32), l_pl_3);
                    if (((rt::flt(l_dot_5, F32(0x00000000)) as i32)) != 0) { bb_ = 23; continue 'fl1; } else { bb_ = 25; continue 'fl1; }
                }
                22 => {
                    bb_ = 16; continue 'fl1;
                }
                23 => {
                    l_in_6 = 0i32;
                    bb_ = 24; continue 'fl1;
                }
                24 => {
                    bb_ = 22; continue 'fl1;
                }
                25 => {
                    l_cross_7 = 1i32;
                    bb_ = 24; continue 'fl1;
                }
                26 => {
                    l_pl_3 = (p_fr_2).wrapping_add((((1i32) as u32)).wrapping_mul(16));
                    { let a2_ = F32(c.m.r32((p_part_1).wrapping_add(12))); F::Vec4_Scale(c, fp.wrapping_add(32), l_pl_3, a2_) };
                    F::Vec4_Add(c, fp.wrapping_add(32), fp, fp.wrapping_add(32));
                    l_dot_5 = F::Vec3_Dot(c, fp.wrapping_add(32), l_pl_3);
                    if (((rt::flt(l_dot_5, F32(0x00000000)) as i32)) != 0) { bb_ = 29; continue 'fl1; } else { bb_ = 31; continue 'fl1; }
                }
                27 => {
                    if (((((l_cross_7) == (1i32)) as i32)) != 0) { bb_ = 37; continue 'fl1; } else { bb_ = 38; continue 'fl1; }
                }
                28 => {
                    if (((rt::fgt(F32(c.m.r32(fp.wrapping_add(48))), F32(0x3f800000)) as i32)) != 0) { bb_ = 32; continue 'fl1; } else { bb_ = 33; continue 'fl1; }
                }
                29 => {
                    l_in_6 = 0i32;
                    bb_ = 30; continue 'fl1;
                }
                30 => {
                    bb_ = 27; continue 'fl1;
                }
                31 => {
                    l_cross_7 = 1i32;
                    bb_ = 30; continue 'fl1;
                }
                32 => {
                    { let a2_ = F32(c.m.r32((p_part_1).wrapping_add(12))); F::Vec4_Scale(c, fp.wrapping_add(32), (p_fr_2).wrapping_add((((0i32) as u32)).wrapping_mul(16)), a2_) };
                    F::Vec4_Add(c, fp.wrapping_add(32), fp, fp.wrapping_add(32));
                    l_dot_5 = F::Vec3_Dot(c, fp.wrapping_add(32), (p_fr_2).wrapping_add((((0i32) as u32)).wrapping_mul(16)));
                    if (((rt::flt(l_dot_5, F32(0x00000000)) as i32)) != 0) { bb_ = 34; continue 'fl1; } else { bb_ = 36; continue 'fl1; }
                }
                33 => {
                    bb_ = 27; continue 'fl1;
                }
                34 => {
                    l_in_6 = 0i32;
                    bb_ = 35; continue 'fl1;
                }
                35 => {
                    bb_ = 33; continue 'fl1;
                }
                36 => {
                    l_cross_7 = 1i32;
                    bb_ = 35; continue 'fl1;
                }
                37 => {
                    if (((rt::flt(F32(c.m.r32((fp.wrapping_add(48)).wrapping_add(4))), F32(0xbf800000)) as i32)) != 0) { bb_ = 39; continue 'fl1; } else { bb_ = 41; continue 'fl1; }
                }
                38 => {
                    bb_ = 13; continue 'fl1;
                }
                39 => {
                    l_pl_3 = (p_fr_2).wrapping_add((((3i32) as u32)).wrapping_mul(16));
                    { let a2_ = F32(c.m.r32((p_part_1).wrapping_add(12))); F::Vec4_Scale(c, fp.wrapping_add(32), l_pl_3, a2_) };
                    F::Vec4_Add(c, fp.wrapping_add(32), fp, fp.wrapping_add(32));
                    l_dot_5 = F::Vec3_Dot(c, fp.wrapping_add(32), l_pl_3);
                    if (((rt::flt(l_dot_5, F32(0x00000000)) as i32)) != 0) { bb_ = 42; continue 'fl1; } else { bb_ = 43; continue 'fl1; }
                }
                40 => {
                    bb_ = 38; continue 'fl1;
                }
                41 => {
                    if (((rt::fgt(F32(c.m.r32((fp.wrapping_add(48)).wrapping_add(4))), F32(0x3f800000)) as i32)) != 0) { bb_ = 44; continue 'fl1; } else { bb_ = 45; continue 'fl1; }
                }
                42 => {
                    l_in_6 = 0i32;
                    bb_ = 43; continue 'fl1;
                }
                43 => {
                    bb_ = 40; continue 'fl1;
                }
                44 => {
                    l_pl_3 = (p_fr_2).wrapping_add((((2i32) as u32)).wrapping_mul(16));
                    { let a2_ = F32(c.m.r32((p_part_1).wrapping_add(12))); F::Vec4_Scale(c, fp.wrapping_add(32), l_pl_3, a2_) };
                    F::Vec4_Add(c, fp.wrapping_add(32), fp, fp.wrapping_add(32));
                    l_dot_5 = F::Vec3_Dot(c, fp.wrapping_add(32), l_pl_3);
                    if (((rt::flt(l_dot_5, F32(0x00000000)) as i32)) != 0) { bb_ = 46; continue 'fl1; } else { bb_ = 47; continue 'fl1; }
                }
                45 => {
                    bb_ = 40; continue 'fl1;
                }
                46 => {
                    l_in_6 = 0i32;
                    bb_ = 47; continue 'fl1;
                }
                47 => {
                    bb_ = 45; continue 'fl1;
                }
                48 => {
                    break 'fl1;
                }
                _ => unreachable!(),
            }
        }
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "StgFrustum_TestPart", &PROBE_N, &[], true); }
    r_
}

pub fn StgDbg_FreeFile(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgDbg_FreeFile", &PROBE_N, &[], false); }
    let mut l_res_1: u32 = 0;
    let r_ = 'f: {
    if ((((((c.m.r32(sym::gStgDbgExternFile) as i32)) != (1i32)) as i32)) != 0) {
        l_res_1 = (c.m.r32(sym::gCommonRes)).wrapping_add(32);
        if (((((c.m.r32((l_res_1).wrapping_add(4))) != (((0i32) as u32))) as i32)) != 0) {
            F::BtlStage_Term(c);
            { let a0_ = c.m.r32((l_res_1).wrapping_add(4)); F::Heap_Free(c, a0_) };
            { let v_ = ((0i32) as u32); let a_ = (l_res_1).wrapping_add(4); c.m.w32(a_, v_); };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgDbg_FreeFile", &PROBE_N, &[], true); }
    r_
}

pub fn stg_a__StgDbg_ResetOnce(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "stg_a__StgDbg_ResetOnce", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let v_ = 0i32; let a_ = sym::gStgDbgUnk; c.m.w32(a_, (v_) as u32); };
    F::StgDbg_ClearState(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "stg_a__StgDbg_ResetOnce", &PROBE_N, &[], true); }
    r_
}

pub fn StgDbg_Reset(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgDbg_Reset", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::stg_a__StgDbg_ResetOnce(c);
    F::stg_a__StgDbg_ResetOnce(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgDbg_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn StgDbg_Unload(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgDbg_Unload", &PROBE_N, &[], false); }
    let mut l_res_1: u32 = 0;
    let r_ = 'f: {
    l_res_1 = (c.m.r32(sym::gCommonRes)).wrapping_add(32);
    F::BtlStage_Term(c);
    if ((((((c.m.r32(sym::gStgDbgExternFile) as i32)) == (1i32)) as i32)) != 0) {
        { let v_ = ((0i32) as u32); let a_ = (l_res_1).wrapping_add(4); c.m.w32(a_, v_); };
        break 'f;
    }
    if (((((c.m.r32((l_res_1).wrapping_add(4))) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32((l_res_1).wrapping_add(4)); F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = (l_res_1).wrapping_add(4); c.m.w32(a_, v_); };
    }
    F::StgDbg_ClearState(c);
    F::BtlStage_Term(c);
    F::StgDbg_ClearState(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgDbg_Unload", &PROBE_N, &[], true); }
    r_
}

pub fn StgDbg_RequestFile(c: &mut Ctx, mut p_arg_1: i32, mut p_id_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgDbg_RequestFile", &PROBE_N, &[(p_arg_1) as u32, (p_id_2) as u32], false); }
    let mut l_res_3: u32 = 0;
    let r_ = 'f: {
    l_res_3 = (c.m.r32(sym::gCommonRes)).wrapping_add(32);
    if (((((c.m.r32((l_res_3).wrapping_add(4))) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32((l_res_3).wrapping_add(4)); F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = (l_res_3).wrapping_add(4); c.m.w32(a_, v_); };
    }
    { let v_ = { let _ = (((p_arg_1) != (0i32)) as i32); F::File_Request(c, p_id_2, ((0i32) as u32)) }; let a_ = (l_res_3).wrapping_add(4); c.m.w32(a_, v_); };
    { let v_ = 0i32; let a_ = sym::gStgDbgExternFile; c.m.w32(a_, (v_) as u32); };
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "StgDbg_RequestFile", &PROBE_N, &[], true); }
    r_
}

pub fn StgDbg_BindFile(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgDbg_BindFile", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::StgModel_InitStage(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgDbg_BindFile", &PROBE_N, &[], true); }
    r_
}

pub fn StgNode_IsRigid(c: &mut Ctx, mut p_node_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgNode_IsRigid", &PROBE_N, &[(p_node_1) as u32], false); }
    let r_ = 'f: {
    break 'f ((((c.m.r32((p_node_1).wrapping_add(64)) as i32)) == (1i32)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "StgNode_IsRigid", &PROBE_N, &[], true); }
    r_
}

pub fn Stg_FadeRatio(c: &mut Ctx, mut p_a_1: F32, mut p_b_2: F32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Stg_FadeRatio", &PROBE_N, &[(p_a_1).0, (p_b_2).0], false); }
    let mut l_d_3: F32 = F32(0);
    let r_ = 'f: {
    l_d_3 = rt::fsub(p_b_2, p_a_1);
    if (((rt::flt(l_d_3, F32(0x41f00000)) as i32)) != 0) {
        break 'f rt::fdiv(l_d_3, F32(0x41f00000));
    }
    break 'f F32(0x3f800000);
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "Stg_FadeRatio", &PROBE_N, &[], true); }
    r_
}

pub fn Stg_FadeByCamDist(c: &mut Ctx, mut p_pos_1: u32, mut p_value_2: F32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Stg_FadeByCamDist", &PROBE_N, &[(p_pos_1) as u32, (p_value_2).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_view_3: u32 = 0;
    let mut l_dist_4: F32 = F32(0);
    let mut l_r_5: F32 = F32(0);
    let r_ = 'f: {
        l_view_3 = c.m.r32(sym::gBtlCamView);
        { let v_ = rt::fsub(F32(c.m.r32((l_view_3).wrapping_add(544))), F32(c.m.r32(p_pos_1))); let a_ = fp; c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_view_3).wrapping_add(544)).wrapping_add(4))), F32(c.m.r32((p_pos_1).wrapping_add(4)))); let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(F32(c.m.r32(((l_view_3).wrapping_add(544)).wrapping_add(8))), F32(c.m.r32((p_pos_1).wrapping_add(8)))); let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = (fp).wrapping_add(12); c.m.w32(a_, (v_).0); };
        l_dist_4 = rt::fsub(F::Vec3_Length(c, fp), F32(c.m.r32((l_view_3).wrapping_add(592))));
        l_r_5 = p_value_2;
        if (((rt::fle(l_dist_4, F32(0x00000000)) as i32)) != 0) {
            break 'f F32(0x00000000);
        }
        if (((rt::fgt(l_dist_4, F32(0x42c80000)) as i32)) != 0) {
            break 'f l_r_5;
        }
        if (((rt::fgt(l_dist_4, F32(0x42960000)) as i32)) != 0) {
            l_r_5 = F32(0x00000000);
            l_r_5 = rt::fmul(rt::fdiv(rt::fsub(l_dist_4, F32(0x42960000)), F32(0x41c80000)), p_value_2);
        } else {
            l_r_5 = F32(0x00000000);
        }
        break 'f l_r_5;
       F32(0)
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Stg_FadeByCamDist", &PROBE_N, &[], true); }
    r_
}

pub fn StgPart_GetPos(c: &mut Ctx, mut p_part_1: u32, mut p_out_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgPart_GetPos", &PROBE_N, &[(p_part_1) as u32, (p_out_2) as u32], false); }
    let r_ = 'f: {
    { let a1_ = (c.m.r32((p_part_1).wrapping_add(8))).wrapping_add((((3i32) as u32)).wrapping_mul(16)); F::Vec4_Copy(c, p_out_2, a1_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgPart_GetPos", &PROBE_N, &[], true); }
    r_
}

pub fn Stg_WrapRange(c: &mut Ctx, mut p_lo_1: F32, mut p_hi_2: F32, mut p_v_3: F32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Stg_WrapRange", &PROBE_N, &[(p_lo_1).0, (p_hi_2).0, (p_v_3).0], false); }
    let r_ = 'f: {
    if (((rt::flt(p_v_3, p_lo_1) as i32)) != 0) {
        'l1: while (((rt::fgt(p_lo_1, p_v_3) as i32)) != 0) {
            { let r_ = rt::fsub(p_hi_2, p_lo_1); let n_ = rt::fadd(p_v_3, r_); p_v_3 = n_; };
        }
    } else {
        'l2: while (((rt::flt(p_hi_2, p_v_3) as i32)) != 0) {
            { let r_ = rt::fsub(p_hi_2, p_lo_1); let n_ = rt::fsub(p_v_3, r_); p_v_3 = n_; };
        }
    }
    break 'f p_v_3;
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "Stg_WrapRange", &PROBE_N, &[], true); }
    r_
}
