#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn BtlObj_DecayPush(c: &mut Ctx, mut p_obj_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_DecayPush", &PROBE_N, &[(p_obj_1) as u32], false); }
    let mut l_rate_2: F32 = F32(0);
    let r_ = 'f: {
    if ((((((c.m.r32((p_obj_1).wrapping_add(5720)) as i32)) != (0i32)) as i32)) != 0) {
        { let v_ = 0i32; let a_ = (p_obj_1).wrapping_add(5720); c.m.w32(a_, (v_) as u32); };
    } else {
        l_rate_2 = F32(0x3f599999);
        F::Vec4_Scale(c, (p_obj_1).wrapping_add(5696), (p_obj_1).wrapping_add(5696), l_rate_2);
        { let r_ = l_rate_2; let a_ = (p_obj_1).wrapping_add(5712); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_DecayPush", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObj_AddPush(c: &mut Ctx, mut p_obj_1: u32, mut p_v_2: u32, mut p_max_3: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_AddPush", &PROBE_N, &[(p_obj_1) as u32, (p_v_2) as u32, (p_max_3).0], false); }
    let mut l_push_4: u32 = 0;
    let mut l_len_5: F32 = F32(0);
    let r_ = 'f: {
    l_push_4 = (p_obj_1).wrapping_add(5696);
    if (((rt::fle(p_max_3, F32(0x00000000)) as i32)) != 0) {
        break 'f;
    }
    if (((rt::flt(F::Vec3_Length(c, l_push_4), p_max_3) as i32)) != 0) {
        F::Vec4_Add(c, l_push_4, l_push_4, p_v_2);
        l_len_5 = F::Vec3_Length(c, l_push_4);
        if (((rt::flt(p_max_3, l_len_5) as i32)) != 0) {
            F::Vec4_Scale(c, l_push_4, l_push_4, rt::fdiv(p_max_3, l_len_5));
        }
    }
    { let v_ = 1i32; let a_ = (p_obj_1).wrapping_add(5720); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_AddPush", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObj_AddSway(c: &mut Ctx, mut p_obj_1: u32, mut p_add_2: F32, mut p_max_3: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_AddSway", &PROBE_N, &[(p_obj_1) as u32, (p_add_2).0, (p_max_3).0], false); }
    let r_ = 'f: {
    if (((rt::fle(p_max_3, F32(0x00000000)) as i32)) != 0) {
        break 'f;
    }
    { let _ = F::Vec3_Length(c, (p_obj_1).wrapping_add(5696)); };
    if (((rt::flt(F32(c.m.r32((p_obj_1).wrapping_add(5712))), p_max_3) as i32)) != 0) {
        { let r_ = p_add_2; let a_ = (p_obj_1).wrapping_add(5712); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        if (((rt::flt(p_max_3, F32(c.m.r32((p_obj_1).wrapping_add(5712)))) as i32)) != 0) {
            { let v_ = p_max_3; let a_ = (p_obj_1).wrapping_add(5712); c.m.w32(a_, (v_).0); };
        }
    }
    { let v_ = 1i32; let a_ = (p_obj_1).wrapping_add(5720); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_AddSway", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObj_SetMoveVec(c: &mut Ctx, mut p_obj_1: u32, mut p_v_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_SetMoveVec", &PROBE_N, &[(p_obj_1) as u32, (p_v_2) as u32], false); }
    let r_ = 'f: {
    F::Vec4_Copy(c, (p_obj_1).wrapping_add(5680), p_v_2);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_SetMoveVec", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObj_InitChains(c: &mut Ctx, mut p_obj_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_InitChains", &PROBE_N, &[(p_obj_1) as u32], false); }
    let r_ = 'f: {
    { let _ = F::memset(c, (p_obj_1).wrapping_add(4112), 0i32, ((1616i32) as u32)); };
    { let v_ = F32(0x3f000000); let a_ = (p_obj_1).wrapping_add(5716); c.m.w32(a_, (v_).0); };
    F::Vec4_Set(c, sym::D_003337D0, F32(0x3dcccccc), F32(0x00000000), F32(0x00000000), F32(0x00000000));
    F::BObjChainA_Build(c, p_obj_1);
    F::BObjChainB_Build(c, p_obj_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_InitChains", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObj_UpdateChains(c: &mut Ctx, mut p_obj_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_UpdateChains", &PROBE_N, &[(p_obj_1) as u32], false); }
    let r_ = 'f: {
    F::BObjChainA_StepAll(c, p_obj_1);
    F::BObjChainB_StepAll(c, p_obj_1);
    F::BtlObj_DecayPush(c, p_obj_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_UpdateChains", &PROBE_N, &[], true); }
    r_
}

pub fn BtlObj_ChaosRand(c: &mut Ctx, mut p_obj_1: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_ChaosRand", &PROBE_N, &[(p_obj_1) as u32], false); }
    let r_ = 'f: {
    if (((rt::feq(F32(c.m.r32((p_obj_1).wrapping_add(5716))), F32(0x00000000)) as i32)) != 0) {
        { let v_ = F32(0x3f000000); let a_ = (p_obj_1).wrapping_add(5716); c.m.w32(a_, (v_).0); };
    }
    { let v_ = rt::fmul(rt::fmul(F32(c.m.r32((p_obj_1).wrapping_add(5716))), F32(0x407ffe5c)), rt::fsub(F32(0x3f800000), F32(c.m.r32((p_obj_1).wrapping_add(5716))))); let a_ = (p_obj_1).wrapping_add(5716); c.m.w32(a_, (v_).0); };
    break 'f F32(c.m.r32((p_obj_1).wrapping_add(5716)));
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlObj_ChaosRand", &PROBE_N, &[], true); }
    r_
}

pub fn BObjChainB_WrapAngle(c: &mut Ctx, mut p_a_1: F32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BObjChainB_WrapAngle", &PROBE_N, &[(p_a_1).0], false); }
    let r_ = 'f: {
    if (((rt::flt(p_a_1, F32(0xc0490fda)) as i32)) != 0) {
        { let r_ = F32(0x40c90fda); let n_ = rt::fadd(p_a_1, r_); p_a_1 = n_; };
    }
    if (((rt::fgt(p_a_1, F32(0x40490fda)) as i32)) != 0) {
        { let r_ = F32(0x40c90fda); let n_ = rt::fsub(p_a_1, r_); p_a_1 = n_; };
    }
    break 'f p_a_1;
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BObjChainB_WrapAngle", &PROBE_N, &[], true); }
    r_
}

pub fn BObjChainB_Step(c: &mut Ctx, mut p_obj_1: u32, mut p_link_2: u32, mut p_parent_3: u32, mut p_out_4: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BObjChainB_Step", &PROBE_N, &[(p_obj_1) as u32, (p_link_2) as u32, (p_parent_3) as u32, (p_out_4) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(368) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 368);
    let mut l_param_5: u32 = 0;
    let mut l_node_6: u32 = 0;
    let mut l_chest_7: u32 = 0;
    let mut l_up_8: u32 = 0;
    let mut l_scale_9: F32 = F32(0);
    let mut l_mult_10: F32 = F32(0);
    let mut l_fps_11: F32 = F32(0);
    let mut l_yawMax_12: F32 = F32(0);
    let mut l_yawMin_13: F32 = F32(0);
    let mut l_gravity_14: F32 = F32(0);
    let mut l_swing_15: F32 = F32(0);
    let mut l_amount_16: F32 = F32(0);
    let mut l_len_17: F32 = F32(0);
    let mut l_sum_18: F32 = F32(0);
    let mut l_k_19: F32 = F32(0);
    let mut l_t_20: F32 = F32(0);
    let mut l_ang_21: F32 = F32(0);
    let mut l_lo_22: F32 = F32(0);
    let mut l_hi_23: F32 = F32(0);
    let mut l_slot_24: i32 = 0;
    let mut l_id_25: i32 = 0;
    let mut l_limA_26: F32 = F32(0);
    let mut l_limB_27: F32 = F32(0);
    let mut l_rate_28: F32 = F32(0);
    let mut l_half_29: F32 = F32(0);
    let r_ = 'f: {
        l_scale_9 = F32(0x3f800000);
        l_mult_10 = F32(0x3f800000);
        l_fps_11 = F32(0x41f00000);
        l_id_25 = (((c.m.r8((p_link_2).wrapping_add(33)) as i8)) as i32);
        if ((((c.m.r32((p_obj_1).wrapping_add(2624))) & (((16777216i32) as u32)))) != 0) {
            l_fps_11 = F32(0x42700000);
            l_mult_10 = F32(0x40000000);
            l_scale_9 = F32(0x3f000000);
        }
        l_param_5 = c.m.r32(((p_obj_1).wrapping_add(24)).wrapping_add(2344));
        l_limA_26 = F32(0x3f060a91);
        l_limB_27 = F32(0x3db2b8c1);
        l_rate_28 = F32(0x3e999999);
        l_half_29 = F32(0x3f000000);
        l_slot_24 = (((c.m.r8((p_link_2).wrapping_add(32)) as i8)) as i32);
        { let v_ = F32(0x00000000); let a_ = fp.wrapping_add(176); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = rt::fdiv(rt::fmul(rt::i2f((((c.m.r8(((l_param_5).wrapping_add(760)).wrapping_add(((l_slot_24) as u32))) as i8)) as i32)), F32(0x40490fda)), F32(0x43340000)); F::Mathf_Cos(c, a0_) }; let a_ = (fp.wrapping_add(176)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = rt::fdiv(rt::fmul(rt::i2f((((c.m.r8(((l_param_5).wrapping_add(760)).wrapping_add(((l_slot_24) as u32))) as i8)) as i32)), F32(0x40490fda)), F32(0x43340000)); F::Mathf_Sin(c, a0_) }; let a_ = (fp.wrapping_add(176)).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (fp.wrapping_add(176)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        l_gravity_14 = F32(c.m.r32(((l_param_5).wrapping_add(768)).wrapping_add((((l_slot_24) as u32)).wrapping_mul(4))));
        l_swing_15 = F32(c.m.r32(((l_param_5).wrapping_add(800)).wrapping_add((((l_slot_24) as u32)).wrapping_mul(4))));
        l_yawMax_12 = rt::fdiv(rt::fmul(rt::i2f((((c.m.r8(((l_param_5).wrapping_add(832)).wrapping_add(((l_slot_24) as u32))) as i8)) as i32)), F32(0x40490fda)), F32(0x43340000));
        l_yawMin_13 = rt::fdiv(rt::fmul(rt::i2f((((c.m.r8(((l_param_5).wrapping_add(840)).wrapping_add(((l_slot_24) as u32))) as i8)) as i32)), F32(0x40490fda)), F32(0x43340000));
        F::Vec4_Copy(c, fp.wrapping_add(144), p_link_2);
        l_node_6 = F::BtlObj_GetNode(c, p_obj_1, l_id_25);
        if (((((l_node_6) == (((0i32) as u32))) as i32)) != 0) {
            break 'f;
        }
        F::Mtx_InverseRT(c, fp, p_parent_3);
        F::Vec4_SetZero(c, fp.wrapping_add(64));
        F::Vec4_SetZero(c, fp.wrapping_add(96));
        F::Vec4_SetZero(c, fp.wrapping_add(112));
        if (((((c.m.r32((p_link_2).wrapping_add(36))) == (((0i32) as u32))) as i32)) != 0) {
            F::BtlObj_GetNodeVelocity(c, p_obj_1, l_id_25, fp.wrapping_add(128));
            { let v_ = F32(0x00000000); let a_ = (fp.wrapping_add(128)).wrapping_add(12); c.m.w32(a_, (v_).0); };
            F::Mtx_MulVec4(c, fp.wrapping_add(128), ((p_obj_1).wrapping_add(2384)).wrapping_add(80), fp.wrapping_add(128));
            F::Vec4_Scale(c, fp.wrapping_add(128), fp.wrapping_add(128), rt::fmul(l_mult_10, l_rate_28));
            F::Vec4_Add(c, fp.wrapping_add(96), fp.wrapping_add(96), fp.wrapping_add(128));
        }
        F::Vec4_Scale(c, fp.wrapping_add(128), (p_obj_1).wrapping_add(5680), rt::fmul(l_mult_10, l_rate_28));
        F::Vec4_Add(c, fp.wrapping_add(96), fp.wrapping_add(96), fp.wrapping_add(128));
        l_len_17 = F::Vec3_Length(c, fp.wrapping_add(96));
        if (((rt::flt(rt::fdiv(F32(0x44ad9c71), l_fps_11), l_len_17) as i32)) != 0) {
            F::Vec4_SetZero(c, fp.wrapping_add(96));
        }
        F::Vec4_Sub(c, fp.wrapping_add(64), fp.wrapping_add(64), fp.wrapping_add(96));
        { let _ = F::BtlStage_GetLightDir(c, fp.wrapping_add(192)); };
        l_len_17 = F::Vec3_Length(c, fp.wrapping_add(192));
        if (((rt::flt(F32(0x3a83126e), l_len_17) as i32)) != 0) {
            { let v_ = rt::fmul(rt::fmul(rt::fdiv(F32(c.m.r32(fp.wrapping_add(192))), l_len_17), F32(c.m.r32((fp.wrapping_add(192)).wrapping_add(12)))), F32(0x3e4ccccc)); let a_ = fp.wrapping_add(112); c.m.w32(a_, (v_).0); };
            { let v_ = rt::fmul(rt::fmul(rt::fdiv(F32(c.m.r32((fp.wrapping_add(192)).wrapping_add(4))), l_len_17), F32(c.m.r32((fp.wrapping_add(192)).wrapping_add(12)))), F32(0x3e4ccccc)); let a_ = (fp.wrapping_add(112)).wrapping_add(4); c.m.w32(a_, (v_).0); };
            { let v_ = rt::fmul(rt::fmul(rt::fdiv(F32(c.m.r32((fp.wrapping_add(192)).wrapping_add(8))), l_len_17), F32(c.m.r32((fp.wrapping_add(192)).wrapping_add(12)))), F32(0x3e4ccccc)); let a_ = (fp.wrapping_add(112)).wrapping_add(8); c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x00000000); let a_ = (fp.wrapping_add(112)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        }
        l_up_8 = c.m.r32((p_link_2).wrapping_add(36));
        if (((((l_up_8) == (((0i32) as u32))) as i32)) != 0) {
            { let v_ = { let a0_ = rt::fadd(F32(c.m.r32(((p_link_2).wrapping_add(40)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), rt::fmul(l_scale_9, F32(0x3dcccccc))); F::BObjChainB_WrapAngle(c, a0_) }; let a_ = ((p_link_2).wrapping_add(40)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            { let v_ = { let a0_ = rt::fadd(F32(c.m.r32(((p_link_2).wrapping_add(40)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), rt::fmul(l_scale_9, F32(0x3e6b851e))); F::BObjChainB_WrapAngle(c, a0_) }; let a_ = ((p_link_2).wrapping_add(40)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            { let v_ = { let a0_ = rt::fadd(F32(c.m.r32(((p_link_2).wrapping_add(40)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), rt::fmul(l_scale_9, F32(0x3e8a3d70))); F::BObjChainB_WrapAngle(c, a0_) }; let a_ = ((p_link_2).wrapping_add(40)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        } else {
            { let v_ = { let a0_ = rt::fsub(F32(c.m.r32(((l_up_8).wrapping_add(40)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(0x3f666666)); F::BObjChainB_WrapAngle(c, a0_) }; let a_ = ((p_link_2).wrapping_add(40)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            { let v_ = { let a0_ = rt::fsub(F32(c.m.r32(((l_up_8).wrapping_add(40)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(0x3f666666)); F::BObjChainB_WrapAngle(c, a0_) }; let a_ = ((p_link_2).wrapping_add(40)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            { let v_ = { let a0_ = rt::fsub(F32(c.m.r32(((l_up_8).wrapping_add(40)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), F32(0x3f666666)); F::BObjChainB_WrapAngle(c, a0_) }; let a_ = ((p_link_2).wrapping_add(40)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        }
        l_sum_18 = F32(0x00000000);
        { let r_ = rt::fadd(rt::fmul({ let a0_ = F32(c.m.r32(((p_link_2).wrapping_add(40)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); F::Mathf_Sin(c, a0_) }, F32(0x3e800000)), F32(0x3e800000)); let n_ = rt::fadd(l_sum_18, r_); l_sum_18 = n_; };
        { let r_ = rt::fadd(rt::fmul({ let a0_ = F32(c.m.r32(((p_link_2).wrapping_add(40)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); F::Mathf_Sin(c, a0_) }, F32(0x3e000000)), F32(0x3e000000)); let n_ = rt::fadd(l_sum_18, r_); l_sum_18 = n_; };
        { let r_ = rt::fadd(rt::fmul({ let a0_ = F32(c.m.r32(((p_link_2).wrapping_add(40)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); F::Mathf_Sin(c, a0_) }, F32(0x3e000000)), F32(0x3e000000)); let n_ = rt::fadd(l_sum_18, r_); l_sum_18 = n_; };
        F::Vec4_Scale(c, fp.wrapping_add(112), fp.wrapping_add(112), l_sum_18);
        F::Vec4_Scale(c, fp.wrapping_add(128), fp.wrapping_add(96), rt::fmul(l_half_29, l_mult_10));
        F::Vec4_Sub(c, fp.wrapping_add(112), fp.wrapping_add(112), fp.wrapping_add(128));
        F::Vec4_Add(c, fp.wrapping_add(112), fp.wrapping_add(112), (p_obj_1).wrapping_add(5696));
        { let a2_ = F32(c.m.r32((p_obj_1).wrapping_add(5712))); F::Vec4_Scale(c, fp.wrapping_add(128), fp.wrapping_add(176), a2_) };
        { let v_ = F32(0x00000000); let a_ = (fp.wrapping_add(128)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        F::Mtx_MulVec4(c, fp.wrapping_add(128), fp, fp.wrapping_add(128));
        F::Vec4_Add(c, fp.wrapping_add(112), fp.wrapping_add(112), fp.wrapping_add(128));
        F::Vec4_Set(c, fp.wrapping_add(208), F32(0x3f800000), F32(0x00000000), F32(0x00000000), F32(0x00000000));
        l_chest_7 = F::BtlObj_GetNode(c, p_obj_1, 3i32);
        if (((((l_chest_7) != (((0i32) as u32))) as i32)) != 0) {
            F::Vec4_Copy(c, fp.wrapping_add(208), ((l_chest_7).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(16)));
        }
        l_amount_16 = rt::fmul(rt::fmul(F::Vec3_Length(c, (p_obj_1).wrapping_add(5680)), F32(0x40000000)), l_mult_10);
        { let r_ = rt::fmul(F::Vec3_Length(c, (p_obj_1).wrapping_add(5696)), F32(0x40800000)); let n_ = rt::fadd(l_amount_16, r_); l_amount_16 = n_; };
        { let r_ = rt::fmul(F32(c.m.r32((p_obj_1).wrapping_add(5712))), F32(0x40800000)); let n_ = rt::fadd(l_amount_16, r_); l_amount_16 = n_; };
        if (((((((c.m.r8((p_link_2).wrapping_add(35)) as i8)) as i32)) & (1i32))) != 0) {
            { let v_ = { let a0_ = rt::fadd(F32(c.m.r32(((p_link_2).wrapping_add(52)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), rt::fmul(rt::fmul(rt::fmul(l_amount_16, F32(0x3dcccccc)), rt::fadd(rt::fmul(F::BtlObj_ChaosRand(c, p_obj_1), F32(0x3e999999)), F32(0x3f666666))), l_scale_9)); F::BObjChainB_WrapAngle(c, a0_) }; let a_ = ((p_link_2).wrapping_add(52)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            { let v_ = { let a0_ = rt::fadd(F32(c.m.r32(((p_link_2).wrapping_add(52)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), rt::fmul(rt::fmul(rt::fmul(l_amount_16, F32(0x3dcccccc)), rt::fadd(rt::fmul(F::BtlObj_ChaosRand(c, p_obj_1), F32(0x3e999999)), F32(0x3f333333))), l_scale_9)); F::BObjChainB_WrapAngle(c, a0_) }; let a_ = ((p_link_2).wrapping_add(52)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            { let v_ = { let a0_ = rt::fadd(F32(c.m.r32(((p_link_2).wrapping_add(52)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), rt::fmul(rt::fmul(rt::fmul(l_amount_16, F32(0x3dcccccc)), rt::fadd(rt::fmul(F::BtlObj_ChaosRand(c, p_obj_1), F32(0x3e999999)), F32(0x3fa66666))), l_scale_9)); F::BObjChainB_WrapAngle(c, a0_) }; let a_ = ((p_link_2).wrapping_add(52)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        } else {
            { let v_ = { let a0_ = rt::fsub(F32(c.m.r32(((p_link_2).wrapping_add(52)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), rt::fmul(rt::fmul(l_amount_16, F32(0x3dcccccc)), rt::fadd(rt::fmul(F::BtlObj_ChaosRand(c, p_obj_1), F32(0x3e999999)), F32(0x3f666666)))); F::BObjChainB_WrapAngle(c, a0_) }; let a_ = ((p_link_2).wrapping_add(52)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            { let v_ = { let a0_ = rt::fsub(F32(c.m.r32(((p_link_2).wrapping_add(52)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), rt::fmul(rt::fmul(l_amount_16, F32(0x3dcccccc)), rt::fadd(rt::fmul(F::BtlObj_ChaosRand(c, p_obj_1), F32(0x3e999999)), F32(0x3f333333)))); F::BObjChainB_WrapAngle(c, a0_) }; let a_ = ((p_link_2).wrapping_add(52)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            { let v_ = { let a0_ = rt::fsub(F32(c.m.r32(((p_link_2).wrapping_add(52)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), rt::fmul(rt::fmul(l_amount_16, F32(0x3dcccccc)), rt::fadd(rt::fmul(F::BtlObj_ChaosRand(c, p_obj_1), F32(0x3e999999)), F32(0x3fa66666)))); F::BObjChainB_WrapAngle(c, a0_) }; let a_ = ((p_link_2).wrapping_add(52)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        }
        l_scale_9 = F32(0x3f000000);
        l_sum_18 = F32(0x00000000);
        { let r_ = rt::fmul({ let a0_ = F32(c.m.r32(((p_link_2).wrapping_add(52)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); F::Mathf_Sin(c, a0_) }, l_scale_9); let n_ = rt::fadd(l_sum_18, r_); l_sum_18 = n_; };
        { let r_ = rt::fmul({ let a0_ = F32(c.m.r32(((p_link_2).wrapping_add(52)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); F::Mathf_Sin(c, a0_) }, l_scale_9); let n_ = rt::fadd(l_sum_18, r_); l_sum_18 = n_; };
        l_k_19 = rt::fmul(rt::fmul(l_amount_16, l_swing_15), l_scale_9);
        l_ang_21 = rt::fmul(l_sum_18, l_k_19);
        if (((rt::flt(l_ang_21, rt::fneg(l_limA_26)) as i32)) != 0) {
            l_ang_21 = rt::fneg(l_limA_26);
        }
        if (((rt::flt(l_limA_26, l_ang_21) as i32)) != 0) {
            l_ang_21 = l_limA_26;
        }
        F::Vec3_RotateAxis(c, fp.wrapping_add(112), fp.wrapping_add(112), fp.wrapping_add(208), l_ang_21);
        l_sum_18 = F32(0x00000000);
        { let r_ = { let a0_ = F32(c.m.r32(((p_link_2).wrapping_add(52)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); F::Mathf_Sin(c, a0_) }; let n_ = rt::fadd(l_sum_18, r_); l_sum_18 = n_; };
        l_ang_21 = rt::fmul(l_sum_18, l_k_19);
        if (((rt::flt(l_ang_21, rt::fneg(l_limB_27)) as i32)) != 0) {
            l_ang_21 = rt::fneg(l_limB_27);
        }
        if (((rt::flt(l_limB_27, l_ang_21) as i32)) != 0) {
            l_ang_21 = l_limB_27;
        }
        F::Vec3_RotateY(c, fp.wrapping_add(112), fp.wrapping_add(112), l_ang_21);
        F::Vec4_Add(c, fp.wrapping_add(64), fp.wrapping_add(64), fp.wrapping_add(112));
        { let v_ = F32(0x00000000); let a_ = (fp.wrapping_add(64)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let r_ = l_gravity_14; let a_ = (fp.wrapping_add(64)).wrapping_add(4); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        F::Mtx_MulVec4(c, fp.wrapping_add(64), fp, fp.wrapping_add(64));
        l_t_20 = rt::fmul(F::Vec3_Length(c, fp.wrapping_add(64)), F32(0x3e4ccccc));
        if (((rt::flt(l_t_20, F32(0x3dcccccc)) as i32)) != 0) {
            l_t_20 = F32(0x3dcccccc);
        }
        if (((rt::flt(l_scale_9, l_t_20) as i32)) != 0) {
            l_t_20 = l_scale_9;
        }
        F::Quat_ConjugateBy(c, fp.wrapping_add(208), fp.wrapping_add(176), fp.wrapping_add(144));
        l_len_17 = F::Vec3_Length(c, fp.wrapping_add(64));
        if (((rt::flt(F32(0x38d1b717), l_len_17) as i32)) != 0) {
            F::Vec4_Scale(c, fp.wrapping_add(80), fp.wrapping_add(64), rt::fdiv(F32(0x3f800000), l_len_17));
        } else {
            F::Vec4_Copy(c, fp.wrapping_add(80), fp.wrapping_add(176));
        }
        F::Quat_FromVectors(c, fp.wrapping_add(224), fp.wrapping_add(208), fp.wrapping_add(80), l_t_20);
        F::Quat_ConjugateBy(c, fp.wrapping_add(80), fp.wrapping_add(208), fp.wrapping_add(224));
        F::Quat_FromMtx(c, fp.wrapping_add(240), p_parent_3);
        F::Quat_Inverse(c, fp.wrapping_add(256), fp.wrapping_add(240));
        F::Quat_Mul(c, fp.wrapping_add(272), fp.wrapping_add(256), (p_link_2).wrapping_add(16));
        F::Quat_ConjugateBy(c, fp.wrapping_add(80), fp.wrapping_add(80), fp.wrapping_add(272));
        F::Vec4_Copy(c, (p_link_2).wrapping_add(16), fp.wrapping_add(240));
        { let v_ = { let a0_ = F32(c.m.r32((fp.wrapping_add(176)).wrapping_add(8))); let a1_ = F32(c.m.r32((fp.wrapping_add(176)).wrapping_add(4))); F::Port_atan2f(c, a0_, a1_) }; let a_ = (fp.wrapping_add(288)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32((fp.wrapping_add(80)).wrapping_add(8))); let a1_ = F32(c.m.r32((fp.wrapping_add(80)).wrapping_add(4))); F::Port_atan2f(c, a0_, a1_) }; let a_ = (fp.wrapping_add(288)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        l_t_20 = { let a0_ = rt::fsub(F32(c.m.r32((fp.wrapping_add(288)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((fp.wrapping_add(288)).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); F::BObjChainB_WrapAngle(c, a0_) };
        if (((rt::flt(l_t_20, l_yawMax_12) as i32)) != 0) {
            F::Vec3_RotateX(c, fp.wrapping_add(80), fp.wrapping_add(80), rt::fsub(l_yawMax_12, l_t_20));
        }
        if (((rt::flt(l_yawMin_13, l_t_20) as i32)) != 0) {
            F::Vec3_RotateX(c, fp.wrapping_add(80), fp.wrapping_add(80), rt::fsub(l_yawMin_13, l_t_20));
        }
        l_lo_22 = F::Mathf_Sin(c, F32(0xbdcccccc));
        l_hi_23 = F::Mathf_Sin(c, F32(0x3dcccccc));
        if (((rt::flt(F32(c.m.r32(fp.wrapping_add(80))), l_lo_22) as i32)) != 0) {
            { let v_ = l_lo_22; let a_ = fp.wrapping_add(80); c.m.w32(a_, (v_).0); };
        }
        if (((rt::flt(l_hi_23, F32(c.m.r32(fp.wrapping_add(80)))) as i32)) != 0) {
            { let v_ = l_hi_23; let a_ = fp.wrapping_add(80); c.m.w32(a_, (v_).0); };
        }
        F::Vec3_Normalize(c, fp.wrapping_add(80), fp.wrapping_add(80));
        F::Vec3_Normalize(c, fp.wrapping_add(128), fp.wrapping_add(80));
        F::Quat_FromVectors(c, fp.wrapping_add(160), fp.wrapping_add(176), fp.wrapping_add(128), F32(0x3f800000));
        F::Vec4_Copy(c, (l_node_6).wrapping_add(160), fp.wrapping_add(160));
        F::Vec4_Copy(c, p_link_2, fp.wrapping_add(160));
        { let r_ = (((!(1i32))) as u32); let a_ = (l_node_6).wrapping_add(4); let n_ = ((c.m.r32(a_)) & (r_)); c.m.w32(a_, n_); };
        F::Quat_ToMtx(c, fp.wrapping_add(296), fp.wrapping_add(160));
        F::Mtx_Mul(c, p_out_4, p_parent_3, fp.wrapping_add(296));
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BObjChainB_Step", &PROBE_N, &[], true); }
    r_
}

pub fn BObjChainB_Build(c: &mut Ctx, mut p_obj_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BObjChainB_Build", &PROBE_N, &[(p_obj_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(160) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 160);
    let mut l_p_2: u32 = 0;
    let mut l_work_3: u32 = 0;
    let mut l_link_4: u32 = 0;
    let mut l_bound_5: u32 = 0;
    let mut l_param_6: u32 = 0;
    let mut l_depth_7: i32 = 0;
    let mut l_level_8: i32 = 0;
    let mut l_slot_9: i32 = 0;
    let mut l_i_10: i32 = 0;
    let mut l_q_11: u32 = 0;
    let r_ = 'f: {
        l_depth_7 = 0i32;
        l_level_8 = 0i32;
        l_work_3 = (p_obj_1).wrapping_add(5152);
        l_bound_5 = c.m.r32(((p_obj_1).wrapping_add(24)).wrapping_add(44));
        { let v_ = 0i32; let a_ = (l_work_3).wrapping_add(512); c.m.w32(a_, (v_) as u32); };
        {
            l_q_11 = (((p_obj_1).wrapping_add(5152)).wrapping_add((((0i32) as u32)).wrapping_mul(64))).wrapping_add(16);
            {
                l_i_10 = 0i32;
                'l1: loop {
                    if !(((((l_i_10) < (8i32)) as i32)) != 0) { break 'l1; }
                    'c2: {
                        { let v_ = (((1i32).wrapping_neg()) as i8); let a_ = (l_q_11).wrapping_add(((16i32) as u32)); c.m.w8(a_, (v_) as u8); };
                        { { { { let v_ = F32(0x00000000); let a_ = (l_q_11).wrapping_add(((((1i32).wrapping_neg()) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); }; { let v_ = F32(0x00000000); let a_ = ((l_q_11).wrapping_add(((((1i32).wrapping_neg()) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); }; }; { let v_ = F32(0x00000000); let a_ = ((l_q_11).wrapping_add(((((1i32).wrapping_neg()) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_).0); }; }; { let v_ = F32(0x3f800000); let a_ = ((l_q_11).wrapping_add(((((1i32).wrapping_neg()) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); }; };
                        { { { { let v_ = F32(0x00000000); let a_ = (l_q_11).wrapping_add((((0i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); }; { let v_ = F32(0x00000000); let a_ = ((l_q_11).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); }; }; { let v_ = F32(0x00000000); let a_ = ((l_q_11).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_).0); }; }; { let v_ = F32(0x3f800000); let a_ = ((l_q_11).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); }; };
                    }
                    { { let o_ = l_i_10; let n_ = (o_).wrapping_add(1); l_i_10 = n_; }; { let r_ = 4i32; let n_ = (l_q_11).wrapping_add((((r_) as u32)).wrapping_mul(16)); l_q_11 = n_; }; };
                }
            }
        }
        if (((((c.m.r32(((p_obj_1).wrapping_add(24)).wrapping_add(2344))) != (((0i32) as u32))) as i32)) != 0) {
            l_p_2 = fp.wrapping_add(32);
            { let v_ = (1i32).wrapping_neg(); let a_ = (l_p_2).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            'l3: while ((1i32) != 0) {
                { let _ = { let a1_ = ((c.m.r16((l_bound_5).wrapping_add(10))) as i32); F::BtlObj_GetNode(c, p_obj_1, a1_) }; };
                { let o_ = l_level_8; let n_ = (o_).wrapping_add(1); l_level_8 = n_; };
                { let v_ = ((c.m.r16((l_bound_5).wrapping_add(10))) as i32); let a_ = (fp.wrapping_add(32)).wrapping_add((((l_level_8) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                l_slot_9 = (1i32).wrapping_neg();
                if ((((((c.m.r32((fp.wrapping_add(32)).wrapping_add((((l_level_8) as u32)).wrapping_mul(4))) as i32)) >= (71i32)) as i32)) != 0) {
                    l_param_6 = c.m.r32(((p_obj_1).wrapping_add(24)).wrapping_add(2344));
                    {
                        l_i_10 = 0i32;
                        'l4: loop {
                            if !(((((l_i_10) < (8i32)) as i32)) != 0) { break 'l4; }
                            'c5: {
                                if (((((((c.m.r8(((l_param_6).wrapping_add(752)).wrapping_add(((l_i_10) as u32)))) as i32)) == ((c.m.r32((fp.wrapping_add(32)).wrapping_add((((l_level_8) as u32)).wrapping_mul(4))) as i32))) as i32)) != 0) {
                                    l_slot_9 = l_i_10;
                                    break 'l4;
                                }
                            }
                            { let o_ = l_i_10; let n_ = (o_).wrapping_add(1); l_i_10 = n_; };
                        }
                    }
                }
                if (((((l_slot_9) >= (0i32)) as i32)) != 0) {
                    l_link_4 = (l_work_3).wrapping_add(((((c.m.r32((l_work_3).wrapping_add(512)) as i32)) as u32)).wrapping_mul(64));
                    { let v_ = ((l_slot_9) as i8); let a_ = (l_link_4).wrapping_add(32); c.m.w8(a_, (v_) as u8); };
                    { let v_ = (((c.m.r32((fp.wrapping_add(32)).wrapping_add((((l_level_8) as u32)).wrapping_mul(4))) as i32)) as i8); let a_ = (l_link_4).wrapping_add(33); c.m.w8(a_, (v_) as u8); };
                    { let v_ = (((c.m.r32((fp.wrapping_add(32)).wrapping_add(((((l_level_8).wrapping_sub(1i32)) as u32)).wrapping_mul(4))) as i32)) as i8); let a_ = (l_link_4).wrapping_add(34); c.m.w8(a_, (v_) as u8); };
                    { let v_ = l_link_4; let a_ = (fp).wrapping_add((((l_depth_7) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = ((l_depth_7) as i8); let a_ = (l_link_4).wrapping_add(35); c.m.w8(a_, (v_) as u8); };
                    if (((((l_depth_7) > (0i32)) as i32)) != 0) {
                        { let v_ = c.m.r32((fp).wrapping_add(((((l_depth_7).wrapping_sub(1i32)) as u32)).wrapping_mul(4))); let a_ = (l_link_4).wrapping_add(36); c.m.w32(a_, v_); };
                    } else {
                        { let v_ = ((0i32) as u32); let a_ = (l_link_4).wrapping_add(36); c.m.w32(a_, v_); };
                    }
                    { let a_ = (l_work_3).wrapping_add(512); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    { let o_ = l_depth_7; let n_ = (o_).wrapping_add(1); l_depth_7 = n_; };
                    if (((((((c.m.r16((l_bound_5).wrapping_add(4))) as i32)) != (0i32)) as i32)) != 0) {
                        { let r_ = ((c.m.r16((l_bound_5).wrapping_add(4))) as i32); let n_ = (l_depth_7).wrapping_sub(r_); l_depth_7 = n_; };
                        if (((((l_depth_7) < (0i32)) as i32)) != 0) {
                            l_depth_7 = 0i32;
                        }
                    }
                }
                if (((((((c.m.r16((l_bound_5).wrapping_add(4))) as i32)) != (0i32)) as i32)) != 0) {
                    { let r_ = ((c.m.r16((l_bound_5).wrapping_add(4))) as i32); let n_ = (l_level_8).wrapping_sub(r_); l_level_8 = n_; };
                }
                if (((((((c.m.r16((l_bound_5).wrapping_add(6))) as i32)) != (0i32)) as i32)) != 0) {
                    break 'l3;
                }
                l_bound_5 = (l_bound_5).wrapping_add((((c.m.r32(l_bound_5) as i32)) as u32));
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BObjChainB_Build", &PROBE_N, &[], true); }
    r_
}

pub fn BObjChainB_StepAll(c: &mut Ctx, mut p_obj_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BObjChainB_StepAll", &PROBE_N, &[(p_obj_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(512) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 512);
    let mut l_node_2: u32 = 0;
    let mut l_i_3: i32 = 0;
    let mut l_slot_4: i8 = 0;
    let mut l_depth_5: i8 = 0;
    let mut l_id_6: i8 = 0;
    let r_ = 'f: {
        {
            l_i_3 = 0i32;
            'l1: loop {
                if !(((((l_i_3) < ((c.m.r32(((p_obj_1).wrapping_add(5152)).wrapping_add(512)) as i32))) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_slot_4 = (c.m.r8((((p_obj_1).wrapping_add(5152)).wrapping_add((((l_i_3) as u32)).wrapping_mul(64))).wrapping_add(32)) as i8);
                    l_depth_5 = (c.m.r8((((p_obj_1).wrapping_add(5152)).wrapping_add((((l_i_3) as u32)).wrapping_mul(64))).wrapping_add(35)) as i8);
                    l_id_6 = (c.m.r8((((p_obj_1).wrapping_add(5152)).wrapping_add((((l_i_3) as u32)).wrapping_mul(64))).wrapping_add(33)) as i8);
                    if (((((((l_slot_4) as i32)) >= (0i32)) as i32)) != 0) {
                        if (((((((l_depth_5) as i32)) <= (0i32)) as i32)) != 0) {
                            l_node_2 = F::BtlObj_GetNode(c, p_obj_1, ((l_id_6) as i32));
                            if (((((l_node_2) != (((0i32) as u32))) as i32)) != 0) {
                                F::BObjChainB_Step(c, p_obj_1, ((p_obj_1).wrapping_add(5152)).wrapping_add((((l_i_3) as u32)).wrapping_mul(64)), (l_node_2).wrapping_add(80), (fp).wrapping_add((((0i32) as u32)).wrapping_mul(64)));
                            }
                        } else {
                            F::BObjChainB_Step(c, p_obj_1, ((p_obj_1).wrapping_add(5152)).wrapping_add((((l_i_3) as u32)).wrapping_mul(64)), (fp).wrapping_add(((((((l_depth_5) as i32)).wrapping_sub(1i32)) as u32)).wrapping_mul(64)), (fp).wrapping_add((((l_depth_5) as u32)).wrapping_mul(64)));
                        }
                    }
                }
                { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BObjChainB_StepAll", &PROBE_N, &[], true); }
    r_
}

pub fn BObjChainA_WrapAngle(c: &mut Ctx, mut p_a_1: F32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BObjChainA_WrapAngle", &PROBE_N, &[(p_a_1).0], false); }
    let r_ = 'f: {
    if (((rt::flt(p_a_1, F32(0xc0490fda)) as i32)) != 0) {
        { let r_ = F32(0x40c90fda); let n_ = rt::fadd(p_a_1, r_); p_a_1 = n_; };
    }
    if (((rt::fgt(p_a_1, F32(0x40490fda)) as i32)) != 0) {
        { let r_ = F32(0x40c90fda); let n_ = rt::fsub(p_a_1, r_); p_a_1 = n_; };
    }
    break 'f p_a_1;
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BObjChainA_WrapAngle", &PROBE_N, &[], true); }
    r_
}

pub fn BObjChainA_Step(c: &mut Ctx, mut p_obj_1: u32, mut p_link_2: u32, mut p_parent_3: u32, mut p_out_4: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BObjChainA_Step", &PROBE_N, &[(p_obj_1) as u32, (p_link_2) as u32, (p_parent_3) as u32, (p_out_4) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(320) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 320);
    let mut l_param_5: u32 = 0;
    let mut l_node_6: u32 = 0;
    let mut l_chest_7: u32 = 0;
    let mut l_up_8: u32 = 0;
    let mut l_hinge_9: i32 = 0;
    let mut l_follow_10: F32 = F32(0);
    let mut l_rest_11: F32 = F32(0);
    let mut l_gravity_12: F32 = F32(0);
    let mut l_limit_13: F32 = F32(0);
    let mut l_hingeOfs_14: F32 = F32(0);
    let mut l_inertia_15: F32 = F32(0);
    let mut l_damping_16: F32 = F32(0);
    let mut l_stiff_17: F32 = F32(0);
    let mut l_scale_18: F32 = F32(0);
    let mut l_mult_19: F32 = F32(0);
    let mut l_fps_20: F32 = F32(0);
    let mut l_hingeLimit_21: F32 = F32(0);
    let mut l_speed_22: F32 = F32(0);
    let mut l_phase_23: F32 = F32(0);
    let mut l_swingMax_24: F32 = F32(0);
    let mut l_swing_25: F32 = F32(0);
    let mut l_amount_26: F32 = F32(0);
    let mut l_angle_27: F32 = F32(0);
    let mut l_len_28: F32 = F32(0);
    let mut l_sum_29: F32 = F32(0);
    let mut l_idle_30: F32 = F32(0);
    let mut l_k_31: F32 = F32(0);
    let mut l_t_32: F32 = F32(0);
    let mut l_ang_33: F32 = F32(0);
    let mut l_eps_34: F32 = F32(0);
    let mut l_rate_35: F32 = F32(0);
    let mut l_half_36: F32 = F32(0);
    let mut l_slot_37: i32 = 0;
    let mut l_id_38: i32 = 0;
    let r_ = 'f: {
        l_scale_18 = F32(0x3f800000);
        l_mult_19 = F32(0x3f800000);
        l_fps_20 = F32(0x41f00000);
        l_slot_37 = (((c.m.r8((p_link_2).wrapping_add(32)) as i8)) as i32);
        l_id_38 = (((c.m.r8((p_link_2).wrapping_add(33)) as i8)) as i32);
        if ((((c.m.r32((p_obj_1).wrapping_add(2624))) & (((16777216i32) as u32)))) != 0) {
            l_fps_20 = F32(0x42700000);
            l_mult_19 = F32(0x40000000);
            l_scale_18 = F32(0x3f000000);
        }
        l_param_5 = c.m.r32(((p_obj_1).wrapping_add(24)).wrapping_add(2344));
        l_eps_34 = F32(0x3a83126e);
        l_rate_35 = F32(0x3e4ccccc);
        l_half_36 = F32(0x3f000000);
        { let v_ = F32(c.m.r32(((l_param_5).wrapping_add(16)).wrapping_add((((l_slot_37) as u32)).wrapping_mul(4)))); let a_ = fp.wrapping_add(176); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fneg(F32(c.m.r32(((l_param_5).wrapping_add(80)).wrapping_add((((l_slot_37) as u32)).wrapping_mul(4))))); let a_ = (fp.wrapping_add(176)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fneg(F32(c.m.r32(((l_param_5).wrapping_add(144)).wrapping_add((((l_slot_37) as u32)).wrapping_mul(4))))); let a_ = (fp.wrapping_add(176)).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (fp.wrapping_add(176)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        l_limit_13 = rt::fdiv(rt::fmul(rt::i2f(((c.m.r8(((l_param_5).wrapping_add(592)).wrapping_add(((l_slot_37) as u32)))) as i32)), F32(0x40490fda)), F32(0x43340000));
        l_hingeOfs_14 = rt::fdiv(rt::fmul(rt::i2f((((c.m.r16(((l_param_5).wrapping_add(224)).wrapping_add((((l_slot_37) as u32)).wrapping_mul(2))) as i16)) as i32)), F32(0x40490fda)), F32(0x43340000));
        l_hingeLimit_21 = rt::fdiv(rt::fmul(rt::i2f(((c.m.r8(((l_param_5).wrapping_add(256)).wrapping_add(((l_slot_37) as u32)))) as i32)), F32(0x40490fda)), F32(0x43340000));
        l_hinge_9 = ((c.m.r8(((l_param_5).wrapping_add(208)).wrapping_add(((l_slot_37) as u32)))) as i32);
        l_speed_22 = rt::fdiv(rt::fmul(rt::i2f((((c.m.r8(((l_param_5).wrapping_add(400)).wrapping_add(((l_slot_37) as u32))) as i8)) as i32)), F32(0x40490fda)), F32(0x43340000));
        l_phase_23 = rt::fdiv(rt::fmul(rt::i2f((((c.m.r16(((l_param_5).wrapping_add(416)).wrapping_add((((l_slot_37) as u32)).wrapping_mul(2))) as i16)) as i32)), F32(0x40490fda)), F32(0x43340000));
        l_swingMax_24 = rt::fdiv(rt::fmul(rt::i2f(((c.m.r8(((l_param_5).wrapping_add(448)).wrapping_add(((l_slot_37) as u32)))) as i32)), F32(0x40490fda)), F32(0x43340000));
        l_stiff_17 = rt::fmul(rt::i2f(((c.m.r8(((l_param_5).wrapping_add(736)).wrapping_add(((l_slot_37) as u32)))) as i32)), F32(0x3c23d70a));
        l_follow_10 = F32(c.m.r32(((l_param_5).wrapping_add(528)).wrapping_add((((l_slot_37) as u32)).wrapping_mul(4))));
        l_rest_11 = F32(c.m.r32(((l_param_5).wrapping_add(608)).wrapping_add((((l_slot_37) as u32)).wrapping_mul(4))));
        l_gravity_12 = F32(c.m.r32(((l_param_5).wrapping_add(672)).wrapping_add((((l_slot_37) as u32)).wrapping_mul(4))));
        l_inertia_15 = F32(c.m.r32(((l_param_5).wrapping_add(272)).wrapping_add((((l_slot_37) as u32)).wrapping_mul(4))));
        l_damping_16 = F32(c.m.r32(((l_param_5).wrapping_add(336)).wrapping_add((((l_slot_37) as u32)).wrapping_mul(4))));
        l_swing_25 = F32(c.m.r32(((l_param_5).wrapping_add(464)).wrapping_add((((l_slot_37) as u32)).wrapping_mul(4))));
        l_len_28 = F::Vec3_Length(c, fp.wrapping_add(176));
        if (((rt::flt(l_eps_34, l_len_28) as i32)) != 0) {
            F::Vec4_Scale(c, fp.wrapping_add(176), fp.wrapping_add(176), rt::fdiv(F32(0x3f800000), l_len_28));
        }
        F::Vec4_Copy(c, fp.wrapping_add(128), p_link_2);
        l_node_6 = F::BtlObj_GetNode(c, p_obj_1, l_id_38);
        if (((((l_node_6) == (((0i32) as u32))) as i32)) != 0) {
            break 'f;
        }
        F::Mtx_InverseRT(c, fp, p_parent_3);
        F::Vec4_SetZero(c, fp.wrapping_add(64));
        F::Vec4_SetZero(c, fp.wrapping_add(80));
        F::Vec4_SetZero(c, fp.wrapping_add(96));
        if (((((c.m.r32((p_link_2).wrapping_add(36))) == (((0i32) as u32))) as i32)) != 0) {
            F::BtlObj_GetNodeVelocity(c, p_obj_1, l_id_38, fp.wrapping_add(112));
            { let v_ = F32(0x00000000); let a_ = (fp.wrapping_add(112)).wrapping_add(12); c.m.w32(a_, (v_).0); };
            F::Mtx_MulVec4(c, fp.wrapping_add(112), ((p_obj_1).wrapping_add(2384)).wrapping_add(80), fp.wrapping_add(112));
            F::Vec4_Scale(c, fp.wrapping_add(112), fp.wrapping_add(112), rt::fmul(l_mult_19, l_rate_35));
            F::Vec4_Add(c, fp.wrapping_add(80), fp.wrapping_add(80), fp.wrapping_add(112));
        }
        F::Vec4_Scale(c, fp.wrapping_add(112), (p_obj_1).wrapping_add(5680), rt::fmul(l_mult_19, l_rate_35));
        F::Vec4_Add(c, fp.wrapping_add(80), fp.wrapping_add(80), fp.wrapping_add(112));
        l_len_28 = F::Vec3_Length(c, fp.wrapping_add(80));
        if (((rt::flt(rt::fdiv(F32(0x44ad9c71), l_fps_20), l_len_28) as i32)) != 0) {
            F::Vec4_SetZero(c, fp.wrapping_add(80));
        }
        F::Vec4_Sub(c, fp.wrapping_add(64), fp.wrapping_add(64), fp.wrapping_add(80));
        { let _ = F::BtlStage_GetLightDir(c, fp.wrapping_add(192)); };
        l_len_28 = F::Vec3_Length(c, fp.wrapping_add(192));
        if (((rt::flt(l_eps_34, l_len_28) as i32)) != 0) {
            { let v_ = rt::fmul(rt::fmul(rt::fdiv(F32(c.m.r32(fp.wrapping_add(192))), l_len_28), F32(c.m.r32((fp.wrapping_add(192)).wrapping_add(12)))), l_rate_35); let a_ = fp.wrapping_add(96); c.m.w32(a_, (v_).0); };
            { let v_ = rt::fmul(rt::fmul(rt::fdiv(F32(c.m.r32((fp.wrapping_add(192)).wrapping_add(4))), l_len_28), F32(c.m.r32((fp.wrapping_add(192)).wrapping_add(12)))), l_rate_35); let a_ = (fp.wrapping_add(96)).wrapping_add(4); c.m.w32(a_, (v_).0); };
            { let v_ = rt::fmul(rt::fmul(rt::fdiv(F32(c.m.r32((fp.wrapping_add(192)).wrapping_add(8))), l_len_28), F32(c.m.r32((fp.wrapping_add(192)).wrapping_add(12)))), l_rate_35); let a_ = (fp.wrapping_add(96)).wrapping_add(8); c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x00000000); let a_ = (fp.wrapping_add(96)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        }
        l_up_8 = c.m.r32((p_link_2).wrapping_add(36));
        if (((((l_up_8) == (((0i32) as u32))) as i32)) != 0) {
            { let v_ = { let a0_ = rt::fadd(F32(c.m.r32(((p_link_2).wrapping_add(40)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), rt::fmul(rt::fadd(l_speed_22, F32(0x3dcccccc)), l_scale_18)); F::BObjChainA_WrapAngle(c, a0_) }; let a_ = ((p_link_2).wrapping_add(40)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            { let v_ = { let a0_ = rt::fadd(F32(c.m.r32(((p_link_2).wrapping_add(40)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), rt::fmul(rt::fadd(l_speed_22, F32(0x3e6b851e)), l_scale_18)); F::BObjChainA_WrapAngle(c, a0_) }; let a_ = ((p_link_2).wrapping_add(40)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            { let v_ = { let a0_ = rt::fadd(F32(c.m.r32(((p_link_2).wrapping_add(40)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), rt::fmul(rt::fadd(l_speed_22, F32(0x3e8a3d70)), l_scale_18)); F::BObjChainA_WrapAngle(c, a0_) }; let a_ = ((p_link_2).wrapping_add(40)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        } else {
            { let v_ = { let a0_ = rt::fsub(F32(c.m.r32(((l_up_8).wrapping_add(40)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(0x3f666666)); F::BObjChainA_WrapAngle(c, a0_) }; let a_ = ((p_link_2).wrapping_add(40)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            { let v_ = { let a0_ = rt::fsub(F32(c.m.r32(((l_up_8).wrapping_add(40)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(0x3f666666)); F::BObjChainA_WrapAngle(c, a0_) }; let a_ = ((p_link_2).wrapping_add(40)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            { let v_ = { let a0_ = rt::fsub(F32(c.m.r32(((l_up_8).wrapping_add(40)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), F32(0x3f666666)); F::BObjChainA_WrapAngle(c, a0_) }; let a_ = ((p_link_2).wrapping_add(40)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        }
        l_idle_30 = F32(0x00000000);
        { let r_ = rt::fadd(rt::fmul({ let a0_ = rt::fadd(F32(c.m.r32(((p_link_2).wrapping_add(40)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), l_phase_23); F::Mathf_Sin(c, a0_) }, F32(0x3e800000)), F32(0x3e800000)); let n_ = rt::fadd(l_idle_30, r_); l_idle_30 = n_; };
        { let r_ = rt::fadd(rt::fmul({ let a0_ = rt::fadd(F32(c.m.r32(((p_link_2).wrapping_add(40)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), l_phase_23); F::Mathf_Sin(c, a0_) }, F32(0x3e000000)), F32(0x3e000000)); let n_ = rt::fadd(l_idle_30, r_); l_idle_30 = n_; };
        { let r_ = rt::fadd(rt::fmul({ let a0_ = rt::fadd(F32(c.m.r32(((p_link_2).wrapping_add(40)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), l_phase_23); F::Mathf_Sin(c, a0_) }, F32(0x3e000000)), F32(0x3e000000)); let n_ = rt::fadd(l_idle_30, r_); l_idle_30 = n_; };
        F::Vec4_Scale(c, fp.wrapping_add(96), fp.wrapping_add(96), l_idle_30);
        F::Vec4_Scale(c, fp.wrapping_add(112), fp.wrapping_add(80), rt::fmul(l_half_36, l_mult_19));
        F::Vec4_Sub(c, fp.wrapping_add(96), fp.wrapping_add(96), fp.wrapping_add(112));
        F::Vec4_Add(c, fp.wrapping_add(96), fp.wrapping_add(96), (p_obj_1).wrapping_add(5696));
        { let a2_ = F32(c.m.r32((p_obj_1).wrapping_add(5712))); F::Vec4_Scale(c, fp.wrapping_add(112), fp.wrapping_add(176), a2_) };
        { let v_ = F32(0x00000000); let a_ = (fp.wrapping_add(112)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        F::Mtx_MulVec4(c, fp.wrapping_add(112), fp, fp.wrapping_add(112));
        F::Vec4_Add(c, fp.wrapping_add(96), fp.wrapping_add(96), fp.wrapping_add(112));
        F::Vec4_Set(c, fp.wrapping_add(208), F32(0x3f800000), F32(0x00000000), F32(0x00000000), F32(0x00000000));
        l_chest_7 = F::BtlObj_GetNode(c, p_obj_1, 3i32);
        if (((((l_chest_7) != (((0i32) as u32))) as i32)) != 0) {
            F::Vec4_Copy(c, fp.wrapping_add(208), ((l_chest_7).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(16)));
        }
        l_amount_26 = rt::fmul(F::Vec3_Length(c, (p_obj_1).wrapping_add(5680)), l_mult_19);
        { let r_ = rt::fmul(F::Vec3_Length(c, (p_obj_1).wrapping_add(5696)), F32(0x40400000)); let n_ = rt::fadd(l_amount_26, r_); l_amount_26 = n_; };
        { let r_ = rt::fmul(F32(c.m.r32((p_obj_1).wrapping_add(5712))), F32(0x40400000)); let n_ = rt::fadd(l_amount_26, r_); l_amount_26 = n_; };
        l_up_8 = c.m.r32((p_link_2).wrapping_add(36));
        if (((((l_up_8) == (((0i32) as u32))) as i32)) != 0) {
            { let v_ = { let a0_ = rt::fadd(F32(c.m.r32(((p_link_2).wrapping_add(52)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), rt::fmul(rt::fmul(rt::fmul(l_amount_26, F32(0x3dcccccc)), rt::fadd(rt::fmul(F::BtlObj_ChaosRand(c, p_obj_1), F32(0x3e999999)), F32(0x3f666666))), l_scale_18)); F::BObjChainA_WrapAngle(c, a0_) }; let a_ = ((p_link_2).wrapping_add(52)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            { let v_ = { let a0_ = rt::fadd(F32(c.m.r32(((p_link_2).wrapping_add(52)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), rt::fmul(rt::fmul(rt::fmul(l_amount_26, F32(0x3dcccccc)), rt::fadd(rt::fmul(F::BtlObj_ChaosRand(c, p_obj_1), F32(0x3e999999)), F32(0x3f333333))), l_scale_18)); F::BObjChainA_WrapAngle(c, a0_) }; let a_ = ((p_link_2).wrapping_add(52)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            { let v_ = { let a0_ = rt::fadd(F32(c.m.r32(((p_link_2).wrapping_add(52)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), rt::fmul(rt::fmul(rt::fmul(l_amount_26, F32(0x3dcccccc)), rt::fadd(rt::fmul(F::BtlObj_ChaosRand(c, p_obj_1), F32(0x3e999999)), F32(0x3fa66666))), l_scale_18)); F::BObjChainA_WrapAngle(c, a0_) }; let a_ = ((p_link_2).wrapping_add(52)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        } else {
            { let v_ = { let a0_ = rt::fsub(F32(c.m.r32(((l_up_8).wrapping_add(52)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(0x3fc90fda)); F::BObjChainA_WrapAngle(c, a0_) }; let a_ = ((p_link_2).wrapping_add(52)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            { let v_ = { let a0_ = rt::fsub(F32(c.m.r32(((l_up_8).wrapping_add(52)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(0x3fc90fda)); F::BObjChainA_WrapAngle(c, a0_) }; let a_ = ((p_link_2).wrapping_add(52)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            { let v_ = { let a0_ = rt::fsub(F32(c.m.r32(((l_up_8).wrapping_add(52)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), F32(0x3fc90fda)); F::BObjChainA_WrapAngle(c, a0_) }; let a_ = ((p_link_2).wrapping_add(52)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        }
        l_sum_29 = F32(0x00000000);
        { let r_ = rt::fmul({ let a0_ = F32(c.m.r32(((p_link_2).wrapping_add(52)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); F::Mathf_Sin(c, a0_) }, F32(0x3f000000)); let n_ = rt::fadd(l_sum_29, r_); l_sum_29 = n_; };
        { let r_ = rt::fmul({ let a0_ = F32(c.m.r32(((p_link_2).wrapping_add(52)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); F::Mathf_Sin(c, a0_) }, F32(0x3f000000)); let n_ = rt::fadd(l_sum_29, r_); l_sum_29 = n_; };
        l_k_31 = rt::fmul(rt::fmul(l_amount_26, l_swing_25), F32(0x3dcccccc));
        l_ang_33 = rt::fmul(l_sum_29, l_k_31);
        if (((rt::flt(l_ang_33, rt::fneg(l_swingMax_24)) as i32)) != 0) {
            l_ang_33 = rt::fneg(l_swingMax_24);
        }
        if (((rt::flt(l_swingMax_24, l_ang_33) as i32)) != 0) {
            l_ang_33 = l_swingMax_24;
        }
        F::Vec3_RotateAxis(c, fp.wrapping_add(96), fp.wrapping_add(96), fp.wrapping_add(208), l_ang_33);
        l_sum_29 = F32(0x00000000);
        { let r_ = { let a0_ = F32(c.m.r32(((p_link_2).wrapping_add(52)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); F::Mathf_Sin(c, a0_) }; let n_ = rt::fadd(l_sum_29, r_); l_sum_29 = n_; };
        l_ang_33 = rt::fmul(l_sum_29, l_k_31);
        if (((rt::flt(l_ang_33, rt::fneg(l_swingMax_24)) as i32)) != 0) {
            l_ang_33 = rt::fneg(l_swingMax_24);
        }
        if (((rt::flt(l_swingMax_24, l_ang_33) as i32)) != 0) {
            l_ang_33 = l_swingMax_24;
        }
        F::Vec3_RotateY(c, fp.wrapping_add(96), fp.wrapping_add(96), l_ang_33);
        F::Vec4_Add(c, fp.wrapping_add(64), fp.wrapping_add(64), fp.wrapping_add(96));
        F::Vec4_Lerp(c, (p_link_2).wrapping_add(16), fp.wrapping_add(80), (p_link_2).wrapping_add(16), l_damping_16);
        F::Vec4_Scale(c, fp.wrapping_add(112), (p_link_2).wrapping_add(16), l_inertia_15);
        F::Vec4_Add(c, fp.wrapping_add(64), fp.wrapping_add(64), fp.wrapping_add(112));
        { let v_ = F32(0x00000000); let a_ = (fp.wrapping_add(64)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let r_ = l_gravity_12; let a_ = (fp.wrapping_add(64)).wrapping_add(4); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        F::Mtx_MulVec4(c, fp.wrapping_add(64), fp, fp.wrapping_add(64));
        F::Vec4_Scale(c, fp.wrapping_add(112), fp.wrapping_add(176), l_rest_11);
        F::Vec4_Add(c, fp.wrapping_add(64), fp.wrapping_add(64), fp.wrapping_add(112));
        l_len_28 = F::Vec3_Length(c, fp.wrapping_add(64));
        if (((rt::flt(F32(0x38d1b717), l_len_28) as i32)) != 0) {
            F::Vec4_Scale(c, fp.wrapping_add(112), fp.wrapping_add(64), rt::fdiv(F32(0x3f800000), l_len_28));
            F::Quat_FromVectors(c, fp.wrapping_add(144), fp.wrapping_add(176), fp.wrapping_add(112), l_follow_10);
        } else {
            F::Quat_SetIdentity(c, fp.wrapping_add(144));
        }
        if (((rt::flt(F32(0x00000000), l_stiff_17) as i32)) != 0) {
            l_k_31 = F32(0x3f000000);
            l_t_32 = rt::fmul(rt::fmul(F::Vec3_Length(c, fp.wrapping_add(64)), F32(0x3d4ccccc)), l_stiff_17);
            l_ang_33 = rt::fadd(rt::fmul(rt::fsub(F32(0x3f800000), l_stiff_17), l_k_31), l_t_32);
            if (((rt::flt(l_ang_33, F32(0x3d4ccccc)) as i32)) != 0) {
                l_ang_33 = F32(0x3d4ccccc);
            }
            if (((rt::flt(l_k_31, l_ang_33) as i32)) != 0) {
                l_ang_33 = l_k_31;
            }
            F::Quat_Slerp(c, fp.wrapping_add(160), fp.wrapping_add(128), fp.wrapping_add(144), l_ang_33);
        } else {
            F::Quat_Slerp(c, fp.wrapping_add(160), fp.wrapping_add(128), fp.wrapping_add(144), F32(0x3f000000));
        }
        F::Quat_LimitAngle(c, fp.wrapping_add(160), fp.wrapping_add(160), l_limit_13);
        if (((rt::flt(F32(0x00000000), l_hingeLimit_21) as i32)) != 0) {
            's1: {
                let sv39: i32 = l_hinge_9;
                match sv39 {
                    0i32 => {
                        { let v_ = F32(0x00000000); let a_ = fp.wrapping_add(208); c.m.w32(a_, (v_).0); };
                        { let v_ = F32(c.m.r32((fp.wrapping_add(176)).wrapping_add(8))); let a_ = (fp.wrapping_add(208)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                        { let v_ = rt::fneg(F32(c.m.r32((fp.wrapping_add(176)).wrapping_add(4)))); let a_ = (fp.wrapping_add(208)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                        { let v_ = F32(0x00000000); let a_ = (fp.wrapping_add(208)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                        break 's1;
                    }
                    1i32 => {
                        { let v_ = rt::fneg(F32(c.m.r32((fp.wrapping_add(176)).wrapping_add(8)))); let a_ = fp.wrapping_add(208); c.m.w32(a_, (v_).0); };
                        { let v_ = F32(0x00000000); let a_ = (fp.wrapping_add(208)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                        { let v_ = F32(c.m.r32(fp.wrapping_add(176))); let a_ = (fp.wrapping_add(208)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                        { let v_ = F32(0x00000000); let a_ = (fp.wrapping_add(208)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                        break 's1;
                    }
                    2i32 => {
                        { let v_ = F32(c.m.r32((fp.wrapping_add(176)).wrapping_add(4))); let a_ = fp.wrapping_add(208); c.m.w32(a_, (v_).0); };
                        { let v_ = rt::fneg(F32(c.m.r32(fp.wrapping_add(176)))); let a_ = (fp.wrapping_add(208)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                        { let v_ = F32(0x00000000); let a_ = (fp.wrapping_add(208)).wrapping_add(8); c.m.w32(a_, (v_).0); };
                        { let v_ = F32(0x00000000); let a_ = (fp.wrapping_add(208)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                        break 's1;
                    }
                    _ => {
                    }
                }
            }
            l_sum_29 = F32(0x00000000);
            F::Quat_FromVectors(c, fp.wrapping_add(224), fp.wrapping_add(208), fp.wrapping_add(160), F32(0x3f800000));
            l_angle_27 = F::Quat_GetAngle(c, fp.wrapping_add(224));
            if (((rt::flt(F::Vec3_Dot(c, fp.wrapping_add(176), fp.wrapping_add(224)), l_sum_29) as i32)) != 0) {
                l_angle_27 = rt::fneg(l_angle_27);
            }
            l_angle_27 = F::BObjChainA_WrapAngle(c, rt::fadd(l_angle_27, l_hingeOfs_14));
            l_t_32 = rt::fsub(F32(0x40490fda), l_hingeLimit_21);
            if (((rt::flt(l_t_32, rt::ffabs(l_angle_27)) as i32)) != 0) {
                if ((((!(((rt::flt(l_sum_29, l_angle_27) as i32)) != 0)) as i32)) != 0) {
                    l_t_32 = rt::fneg(l_t_32);
                }
                l_angle_27 = rt::fsub(l_t_32, l_angle_27);
                l_sum_29 = F::Mathf_Cos(c, l_angle_27);
                if (((rt::flt(F32(0x00000000), l_sum_29) as i32)) != 0) {
                    { let a1_ = F32(c.m.r32(fp.wrapping_add(176))); let a2_ = F32(c.m.r32((fp.wrapping_add(176)).wrapping_add(4))); let a3_ = F32(c.m.r32((fp.wrapping_add(176)).wrapping_add(8))); F::Quat_FromAxisAngle(c, fp.wrapping_add(240), a1_, a2_, a3_, l_angle_27) };
                    F::Quat_ConjugateBy(c, fp.wrapping_add(160), fp.wrapping_add(160), fp.wrapping_add(240));
                    F::Quat_SlerpIdentity(c, fp.wrapping_add(160), fp.wrapping_add(160), l_sum_29);
                } else {
                    F::Quat_SetIdentity(c, fp.wrapping_add(160));
                }
            }
        }
        F::Vec4_Copy(c, (l_node_6).wrapping_add(160), fp.wrapping_add(160));
        F::Vec4_Copy(c, p_link_2, fp.wrapping_add(160));
        { let r_ = (((!(1i32))) as u32); let a_ = (l_node_6).wrapping_add(4); let n_ = ((c.m.r32(a_)) & (r_)); c.m.w32(a_, n_); };
        F::Quat_ToMtx(c, fp.wrapping_add(256), fp.wrapping_add(160));
        F::Mtx_Mul(c, p_out_4, p_parent_3, fp.wrapping_add(256));
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BObjChainA_Step", &PROBE_N, &[], true); }
    r_
}

pub fn BObjChainA_Build(c: &mut Ctx, mut p_obj_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BObjChainA_Build", &PROBE_N, &[(p_obj_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(64) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 64);
    let mut l_work_2: u32 = 0;
    let mut l_link_3: u32 = 0;
    let mut l_bound_4: u32 = 0;
    let mut l_param_5: u32 = 0;
    let mut l_depth_6: i32 = 0;
    let mut l_slot_7: i32 = 0;
    let mut l_id_8: u8 = 0;
    let mut l_i_9: i32 = 0;
    let mut l_q_10: u32 = 0;
    let r_ = 'f: {
        l_depth_6 = 0i32;
        l_work_2 = (p_obj_1).wrapping_add(4112);
        l_bound_4 = c.m.r32(((p_obj_1).wrapping_add(24)).wrapping_add(44));
        { let v_ = 0i32; let a_ = (l_work_2).wrapping_add(1024); c.m.w32(a_, (v_) as u32); };
        {
            l_q_10 = (l_work_2).wrapping_add((((0i32) as u32)).wrapping_mul(64));
            {
                l_i_9 = 0i32;
                'l1: loop {
                    if !(((((l_i_9) < (16i32)) as i32)) != 0) { break 'l1; }
                    'c2: {
                        { let v_ = (((1i32).wrapping_neg()) as i8); let a_ = (l_q_10).wrapping_add(((32i32) as u32)); c.m.w8(a_, (v_) as u8); };
                        { { { { let v_ = F32(0x00000000); let a_ = l_q_10; c.m.w32(a_, (v_).0); }; { let v_ = F32(0x00000000); let a_ = (l_q_10).wrapping_add(4); c.m.w32(a_, (v_).0); }; }; { let v_ = F32(0x00000000); let a_ = (l_q_10).wrapping_add(8); c.m.w32(a_, (v_).0); }; }; { let v_ = F32(0x3f800000); let a_ = (l_q_10).wrapping_add(12); c.m.w32(a_, (v_).0); }; };
                    }
                    { { let o_ = l_i_9; let n_ = (o_).wrapping_add(1); l_i_9 = n_; }; { let r_ = 4i32; let n_ = (l_q_10).wrapping_add((((r_) as u32)).wrapping_mul(16)); l_q_10 = n_; }; };
                }
            }
        }
        if (((((c.m.r32(((p_obj_1).wrapping_add(24)).wrapping_add(2344))) != (((0i32) as u32))) as i32)) != 0) {
            'l3: while ((1i32) != 0) {
                { let _ = { let a1_ = ((c.m.r16((l_bound_4).wrapping_add(10))) as i32); F::BtlObj_GetNode(c, p_obj_1, a1_) }; };
                l_slot_7 = (1i32).wrapping_neg();
                if (((((((c.m.r16((l_bound_4).wrapping_add(10))) as i32)) >= (71i32)) as i32)) != 0) {
                    l_param_5 = c.m.r32(((p_obj_1).wrapping_add(24)).wrapping_add(2344));
                    {
                        l_i_9 = 0i32;
                        'l4: loop {
                            if !(((((l_i_9) < (16i32)) as i32)) != 0) { break 'l4; }
                            'c5: {
                                if (((((((c.m.r8((l_param_5).wrapping_add(((l_i_9) as u32)))) as i32)) == (((c.m.r16((l_bound_4).wrapping_add(10))) as i32))) as i32)) != 0) {
                                    l_slot_7 = l_i_9;
                                    break 'l4;
                                }
                            }
                            { let o_ = l_i_9; let n_ = (o_).wrapping_add(1); l_i_9 = n_; };
                        }
                    }
                }
                if (((((l_slot_7) >= (0i32)) as i32)) != 0) {
                    l_link_3 = (l_work_2).wrapping_add(((((c.m.r32((l_work_2).wrapping_add(1024)) as i32)) as u32)).wrapping_mul(64));
                    { let v_ = ((l_slot_7) as i8); let a_ = (l_link_3).wrapping_add(32); c.m.w8(a_, (v_) as u8); };
                    l_id_8 = ((c.m.r16((l_bound_4).wrapping_add(10))) as u8);
                    { let v_ = l_link_3; let a_ = (fp).wrapping_add((((l_depth_6) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = ((l_id_8) as i8); let a_ = (l_link_3).wrapping_add(33); c.m.w8(a_, (v_) as u8); };
                    { let v_ = ((l_depth_6) as i8); let a_ = (l_link_3).wrapping_add(34); c.m.w8(a_, (v_) as u8); };
                    if (((((l_depth_6) > (0i32)) as i32)) != 0) {
                        { let v_ = c.m.r32((fp).wrapping_add(((((l_depth_6).wrapping_sub(1i32)) as u32)).wrapping_mul(4))); let a_ = (l_link_3).wrapping_add(36); c.m.w32(a_, v_); };
                    } else {
                        { let v_ = ((0i32) as u32); let a_ = (l_link_3).wrapping_add(36); c.m.w32(a_, v_); };
                    }
                    { let a_ = (l_work_2).wrapping_add(1024); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    { let o_ = l_depth_6; let n_ = (o_).wrapping_add(1); l_depth_6 = n_; };
                    if (((((((c.m.r16((l_bound_4).wrapping_add(4))) as i32)) != (0i32)) as i32)) != 0) {
                        { let r_ = ((c.m.r16((l_bound_4).wrapping_add(4))) as i32); let n_ = (l_depth_6).wrapping_sub(r_); l_depth_6 = n_; };
                        if (((((l_depth_6) < (0i32)) as i32)) != 0) {
                            l_depth_6 = 0i32;
                        }
                    }
                }
                if (((((((c.m.r16((l_bound_4).wrapping_add(6))) as i32)) != (0i32)) as i32)) != 0) {
                    break 'l3;
                }
                l_bound_4 = (l_bound_4).wrapping_add((((c.m.r32(l_bound_4) as i32)) as u32));
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BObjChainA_Build", &PROBE_N, &[], true); }
    r_
}

pub fn BObjChainA_StepAll(c: &mut Ctx, mut p_obj_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BObjChainA_StepAll", &PROBE_N, &[(p_obj_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(1024) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 1024);
    let mut l_node_2: u32 = 0;
    let mut l_i_3: i32 = 0;
    let mut l_slot_4: i8 = 0;
    let mut l_depth_5: i8 = 0;
    let mut l_id_6: i8 = 0;
    let r_ = 'f: {
        {
            l_i_3 = 0i32;
            'l1: loop {
                if !(((((l_i_3) < ((c.m.r32(((p_obj_1).wrapping_add(4112)).wrapping_add(1024)) as i32))) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_slot_4 = (c.m.r8((((p_obj_1).wrapping_add(4112)).wrapping_add((((l_i_3) as u32)).wrapping_mul(64))).wrapping_add(32)) as i8);
                    l_depth_5 = (c.m.r8((((p_obj_1).wrapping_add(4112)).wrapping_add((((l_i_3) as u32)).wrapping_mul(64))).wrapping_add(34)) as i8);
                    l_id_6 = (c.m.r8((((p_obj_1).wrapping_add(4112)).wrapping_add((((l_i_3) as u32)).wrapping_mul(64))).wrapping_add(33)) as i8);
                    if (((((((l_slot_4) as i32)) >= (0i32)) as i32)) != 0) {
                        if (((((((l_depth_5) as i32)) <= (0i32)) as i32)) != 0) {
                            l_node_2 = F::BtlObj_GetNode(c, p_obj_1, ((l_id_6) as i32));
                            if (((((l_node_2) != (((0i32) as u32))) as i32)) != 0) {
                                F::BObjChainA_Step(c, p_obj_1, ((p_obj_1).wrapping_add(4112)).wrapping_add((((l_i_3) as u32)).wrapping_mul(64)), (l_node_2).wrapping_add(80), (fp).wrapping_add((((0i32) as u32)).wrapping_mul(64)));
                            }
                        } else {
                            F::BObjChainA_Step(c, p_obj_1, ((p_obj_1).wrapping_add(4112)).wrapping_add((((l_i_3) as u32)).wrapping_mul(64)), (fp).wrapping_add(((((((l_depth_5) as i32)).wrapping_sub(1i32)) as u32)).wrapping_mul(64)), (fp).wrapping_add((((l_depth_5) as u32)).wrapping_mul(64)));
                        }
                    }
                }
                { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BObjChainA_StepAll", &PROBE_N, &[], true); }
    r_
}
