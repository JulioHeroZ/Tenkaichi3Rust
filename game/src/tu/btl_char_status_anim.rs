#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn BtlAnim_ApplyTableFlags(c: &mut Ctx, mut p_handle_1: u32, mut p_anim_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAnim_ApplyTableFlags", &PROBE_N, &[(p_handle_1) as u32, (p_anim_2) as u32], false); }
    let r_ = 'f: {
    if (((((p_handle_1) != (((0i32) as u32))) as i32)) != 0) {
        'l1: loop {
            'c2: {
                if ((((F::BtlAnim_GetFlags(c, ((p_anim_2) as u32))) & (((1073741824i32) as u32)))) != 0) {
                    F::BtlObjAnim_ZeroRootAxes(c, p_handle_1, 1i32, 1i32, 1i32);
                } else {
                    if ((((F::BtlAnim_GetFlags(c, ((p_anim_2) as u32))) & (2147483648u32))) != 0) {
                        F::BtlObjAnim_RebaseRoot(c, p_handle_1);
                        if ((((F::BtlAnim_GetFlags(c, ((p_anim_2) as u32))) & (((2i32) as u32)))) != 0) {
                            F::BtlObjAnim_ZeroRootAxes(c, p_handle_1, 1i32, 0i32, 0i32);
                        }
                    }
                }
            }
            if !((0i32) != 0) { break 'l1; }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAnim_ApplyTableFlags", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAnim_HideModelNodes(c: &mut Ctx, mut p_handle_1: u32, mut p_model_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAnim_HideModelNodes", &PROBE_N, &[(p_handle_1) as u32, (p_model_2) as u32], false); }
    let r_ = 'f: {
    if (((((p_handle_1) != (((0i32) as u32))) as i32)) != 0) {
        'l1: loop {
            'c2: {
                's3: {
                    let sv3: i32 = p_model_2;
                    match sv3 {
                        152i32 => {
                            F::BtlObjAnim_ClearNodeRot(c, p_handle_1, 16i32);
                            F::BtlObjAnim_ClearNodeRot(c, p_handle_1, 17i32);
                            F::BtlObjAnim_ClearNodeRot(c, p_handle_1, 46i32);
                            F::BtlObjAnim_ClearNodeRot(c, p_handle_1, 47i32);
                            break 's3;
                        }
                        153i32 => {
                            F::BtlObjAnim_ClearNodeRot(c, p_handle_1, 17i32);
                            break 's3;
                        }
                        120i32 => {
                            F::BtlObjAnim_ClearNodeRot(c, p_handle_1, 17i32);
                            F::BtlObjAnim_ClearNodeRot(c, p_handle_1, 46i32);
                            F::BtlObjAnim_ClearNodeRot(c, p_handle_1, 35i32);
                            F::BtlObjAnim_ClearNodeRot(c, p_handle_1, 21i32);
                            break 's3;
                        }
                        _ => {
                        }
                    }
                }
            }
            if !((0i32) != 0) { break 'l1; }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAnim_HideModelNodes", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAnim_HideModelNode10(c: &mut Ctx, mut p_handle_1: u32, mut p_model_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAnim_HideModelNode10", &PROBE_N, &[(p_handle_1) as u32, (p_model_2) as u32], false); }
    let r_ = 'f: {
    if (((((p_handle_1) != (((0i32) as u32))) as i32)) != 0) {
        'l1: loop {
            'c2: {
                if (((((p_model_2) == (120i32)) as i32)) != 0) {
                    F::BtlObjAnim_ClearNodeRot(c, p_handle_1, 16i32);
                }
            }
            if !((0i32) != 0) { break 'l1; }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAnim_HideModelNode10", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAnim_Play(c: &mut Ctx, mut p_chr_1: u32, mut p_anim_2: i32, mut p_blend_3: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAnim_Play", &PROBE_N, &[(p_chr_1) as u32, (p_anim_2) as u32, (p_blend_3).0], false); }
    let mut l_ids_4: u32 = 0;
    let mut l_obj_5: u32 = 0;
    let mut l_n_6: i32 = 0;
    let mut l_arg_7: i32 = 0;
    let mut l_obj2_8: u32 = 0;
    let r_ = 'f: {
    l_ids_4 = (p_chr_1).wrapping_add(2420);
    l_obj_5 = F::BtlChar_GetObj(c, p_chr_1);
    if ((((F::BtlAnim_GetFlags(c, ((p_anim_2) as u32))) & (((536870912i32) as u32)))) != 0) {
        l_arg_7 = 0i32;
        if (({ let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); F::BtlCharApi_IsInRushSequence(c, a0_) }) != 0) {
            l_arg_7 = (c.m.r32((p_chr_1).wrapping_add(3812)) as i32);
        }
        l_obj2_8 = F::BtlChar_GetObj(c, p_chr_1);
        { let a1_ = ((((F::BtlOpp_GetObj(c, p_chr_1)) as u64)) as i32); F::BtlObjAnim_PlayFrom(c, l_obj2_8, ((((a1_) as i64 as u64)) as u32), p_anim_2, l_arg_7) };
        { let a0_ = c.m.r32((l_obj_5).wrapping_add(2880)); let a1_ = (c.m.r32((l_obj_5).wrapping_add(12)) as i32); F::BtlAnim_HideModelNode10(c, a0_, a1_) };
    } else {
        F::BtlObjAnim_Play(c, l_obj_5, 0i32, p_anim_2, 1i32);
    }
    { let r_ = (((!(32i32))) as u32); let a_ = (l_obj_5).wrapping_add(2624); let n_ = ((c.m.r32(a_)) & (r_)); c.m.w32(a_, n_); };
    { let r_ = (((!(64i32))) as u32); let a_ = (l_obj_5).wrapping_add(2624); let n_ = ((c.m.r32(a_)) & (r_)); c.m.w32(a_, n_); };
    l_n_6 = F::BtlObj_GetMouthMode(c, l_obj_5);
    if (((((l_n_6) < (13i32)) as i32)) != 0) {
        if (((((l_n_6) >= (9i32)) as i32)) != 0) {
            F::BtlObj_SetSubState(c, l_obj_5, 0i32, 0i32);
        }
    }
    F::BtlObj_SetEyeFrame(c, l_obj_5, ((((9i32) as i64 as u64)) as u32));
    F::BtlChar_ClearFlag(c, p_chr_1, ((((48i32) as i64 as u64)) as u32));
    { let a0_ = c.m.r32((l_obj_5).wrapping_add(2880)); F::BtlAnim_ApplyTableFlags(c, a0_, p_anim_2) };
    { let a0_ = c.m.r32((l_obj_5).wrapping_add(2880)); let a1_ = (c.m.r32((l_obj_5).wrapping_add(12)) as i32); F::BtlAnim_HideModelNodes(c, a0_, a1_) };
    F::BtlChar_ClearFlagRange(c, p_chr_1, ((((45i32) as i64 as u64)) as u32), ((((49i32) as i64 as u64)) as u32));
    F::BtlChar_SetFlag(c, p_chr_1, ((((43i32) as i64 as u64)) as u32));
    { let v_ = (1i32).wrapping_neg(); let a_ = (l_ids_4).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
    { let v_ = p_anim_2; let a_ = l_ids_4; c.m.w32(a_, (v_) as u32); };
    { let v_ = 0i32; let a_ = (p_chr_1).wrapping_add(2444); c.m.w32(a_, (v_) as u32); };
    if (((rt::fgt(p_blend_3, F32(0x00000000)) as i32)) != 0) {
        F::BtlAnim_SetBlend(c, p_chr_1, p_blend_3);
        F::BtlAnim_ApplyBlend(c, p_chr_1);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAnim_Play", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAnim_PlayKeep(c: &mut Ctx, mut p_chr_1: u32, mut p_anim_2: i32, mut p_blend_3: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAnim_PlayKeep", &PROBE_N, &[(p_chr_1) as u32, (p_anim_2) as u32, (p_blend_3).0], false); }
    let mut l_obj_4: u32 = 0;
    let mut l_ids_5: u32 = 0;
    let r_ = 'f: {
    l_obj_4 = F::BtlChar_GetObj(c, p_chr_1);
    l_ids_5 = (p_chr_1).wrapping_add(2420);
    F::BtlObjAnim_Play(c, l_obj_4, 0i32, p_anim_2, 0i32);
    { let a0_ = c.m.r32((l_obj_4).wrapping_add(2880)); F::BtlAnim_ApplyTableFlags(c, a0_, p_anim_2) };
    { let a0_ = c.m.r32((l_obj_4).wrapping_add(2880)); let a1_ = (c.m.r32((l_obj_4).wrapping_add(12)) as i32); F::BtlAnim_HideModelNodes(c, a0_, a1_) };
    F::BtlChar_ClearFlagRange(c, p_chr_1, ((((45i32) as i64 as u64)) as u32), ((((49i32) as i64 as u64)) as u32));
    F::BtlChar_SetFlag(c, p_chr_1, ((((44i32) as i64 as u64)) as u32));
    { let v_ = p_anim_2; let a_ = l_ids_5; c.m.w32(a_, (v_) as u32); };
    { let v_ = (1i32).wrapping_neg(); let a_ = (l_ids_5).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
    if (((rt::fgt(p_blend_3, F32(0x00000000)) as i32)) != 0) {
        F::BtlAnim_SetBlend(c, p_chr_1, p_blend_3);
        F::BtlAnim_ApplyBlend(c, p_chr_1);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAnim_PlayKeep", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAnim_SubToMain(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAnim_SubToMain", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_obj_2: u32 = 0;
    let mut l_ids_3: u32 = 0;
    let r_ = 'f: {
    l_obj_2 = F::BtlChar_GetObj(c, p_chr_1);
    l_ids_3 = (p_chr_1).wrapping_add(2420);
    F::BtlObjAnim_PromoteLayer(c, l_obj_2);
    { let v_ = (c.m.r32((l_ids_3).wrapping_add(12)) as i32); let a_ = l_ids_3; c.m.w32(a_, (v_) as u32); };
    { let r_ = (((!(32i32))) as u32); let a_ = (l_obj_2).wrapping_add(2624); let n_ = ((c.m.r32(a_)) & (r_)); c.m.w32(a_, n_); };
    { let r_ = (((!(64i32))) as u32); let a_ = (l_obj_2).wrapping_add(2624); let n_ = ((c.m.r32(a_)) & (r_)); c.m.w32(a_, n_); };
    F::BtlChar_ClearFlagRange(c, p_chr_1, ((((45i32) as i64 as u64)) as u32), ((((49i32) as i64 as u64)) as u32));
    F::BtlChar_SetFlag(c, p_chr_1, ((((43i32) as i64 as u64)) as u32));
    { let v_ = 0i32; let a_ = (p_chr_1).wrapping_add(2444); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAnim_SubToMain", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAnim_Request(c: &mut Ctx, mut p_chr_1: u32, mut p_anim_2: i32, mut p_blend_3: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAnim_Request", &PROBE_N, &[(p_chr_1) as u32, (p_anim_2) as u32, (p_blend_3).0], false); }
    let r_ = 'f: {
    F::BtlChar_SetHeldFlag(c, p_chr_1, ((((45i32) as i64 as u64)) as u32));
    { let v_ = p_anim_2; let a_ = ((p_chr_1).wrapping_add(2420)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
    F::BtlAnim_SetBlend(c, p_chr_1, p_blend_3);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAnim_Request", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAnim_RequestKeep(c: &mut Ctx, mut p_chr_1: u32, mut p_anim_2: i32, mut p_blend_3: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAnim_RequestKeep", &PROBE_N, &[(p_chr_1) as u32, (p_anim_2) as u32, (p_blend_3).0], false); }
    let r_ = 'f: {
    F::BtlChar_SetHeldFlag(c, p_chr_1, ((((46i32) as i64 as u64)) as u32));
    { let v_ = p_anim_2; let a_ = ((p_chr_1).wrapping_add(2420)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
    F::BtlAnim_SetBlend(c, p_chr_1, p_blend_3);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAnim_RequestKeep", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAnim_RequestSubToMain(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAnim_RequestSubToMain", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    F::BtlChar_SetHeldFlag(c, p_chr_1, ((((47i32) as i64 as u64)) as u32));
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAnim_RequestSubToMain", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAnim_SetBlend(c: &mut Ctx, mut p_chr_1: u32, mut p_blend_2: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAnim_SetBlend", &PROBE_N, &[(p_chr_1) as u32, (p_blend_2).0], false); }
    let r_ = 'f: {
    F::BtlChar_SetHeldFlag(c, p_chr_1, ((((50i32) as i64 as u64)) as u32));
    { let v_ = p_blend_2; let a_ = (p_chr_1).wrapping_add(2436); c.m.w32(a_, (v_).0); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAnim_SetBlend", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAnim_SetNoBlend(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAnim_SetNoBlend", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    F::BtlAnim_SetBlend(c, p_chr_1, F32(0x00000000));
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAnim_SetNoBlend", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAnim_FlushRequest(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAnim_FlushRequest", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    if ((F::BtlChar_TestFlag(c, p_chr_1, ((((45i32) as i64 as u64)) as u32))) != 0) {
        { let a1_ = (c.m.r32(((p_chr_1).wrapping_add(2420)).wrapping_add(4)) as i32); let a2_ = F32(c.m.r32((p_chr_1).wrapping_add(2436))); F::BtlAnim_Play(c, p_chr_1, a1_, a2_) };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAnim_FlushRequest", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAnim_FlushRequestKeep(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAnim_FlushRequestKeep", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    if ((F::BtlChar_TestFlag(c, p_chr_1, ((((46i32) as i64 as u64)) as u32))) != 0) {
        { let a1_ = (c.m.r32(((p_chr_1).wrapping_add(2420)).wrapping_add(4)) as i32); let a2_ = F32(c.m.r32((p_chr_1).wrapping_add(2436))); F::BtlAnim_PlayKeep(c, p_chr_1, a1_, a2_) };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAnim_FlushRequestKeep", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAnim_FlushSubToMain(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAnim_FlushSubToMain", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    if ((F::BtlChar_TestFlag(c, p_chr_1, ((((47i32) as i64 as u64)) as u32))) != 0) {
        if ((((((c.m.r32(((p_chr_1).wrapping_add(2420)).wrapping_add(12)) as i32)) >= (0i32)) as i32)) != 0) {
            F::BtlAnim_SubToMain(c, p_chr_1);
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAnim_FlushSubToMain", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAnim_ApplyBlend(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAnim_ApplyBlend", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_obj_2: u32 = 0;
    let mut l_a_3: u32 = 0;
    let r_ = 'f: {
    l_obj_2 = F::BtlChar_GetObj(c, p_chr_1);
    if ((F::BtlChar_TestFlag(c, p_chr_1, ((((55i32) as i64 as u64)) as u32))) != 0) {
        F::BtlObjAnim_StartBlend(c, l_obj_2, F32(0x00000000));
        F::BtlChar_ClearFlag(c, p_chr_1, ((((50i32) as i64 as u64)) as u32));
        { let v_ = F32(0x00000000); let a_ = (p_chr_1).wrapping_add(2436); c.m.w32(a_, (v_).0); };
    } else {
        if ((F::BtlChar_TestFlag(c, p_chr_1, ((((50i32) as i64 as u64)) as u32))) != 0) {
            { let a1_ = F32(c.m.r32((p_chr_1).wrapping_add(2436))); F::BtlObjAnim_StartBlend(c, l_obj_2, a1_) };
            l_a_3 = (l_obj_2).wrapping_add(2880);
            F::BtlChar_ClearFlag(c, p_chr_1, ((((50i32) as i64 as u64)) as u32));
            { let v_ = F32(0x00000000); let a_ = (p_chr_1).wrapping_add(2436); c.m.w32(a_, (v_).0); };
            { let r_ = F32(c.m.r32((l_a_3).wrapping_add(328))); let a_ = (l_a_3).wrapping_add(324); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            if (((rt::flt(F32(c.m.r32((l_a_3).wrapping_add(324))), F32(0x00000000)) as i32)) != 0) {
                { let v_ = F32(0x00000000); let a_ = (l_a_3).wrapping_add(324); c.m.w32(a_, (v_).0); };
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAnim_ApplyBlend", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAnim_PlaySub(c: &mut Ctx, mut p_chr_1: u32, mut p_anim_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAnim_PlaySub", &PROBE_N, &[(p_chr_1) as u32, (p_anim_2) as u32], false); }
    let mut l_ids_3: u32 = 0;
    let mut l_obj_4: u32 = 0;
    let r_ = 'f: {
    if ((((((c.m.r32(((p_chr_1).wrapping_add(2420)).wrapping_add(12)) as i32)) != (p_anim_2)) as i32)) != 0) {
        l_ids_3 = (p_chr_1).wrapping_add(2420);
        l_obj_4 = F::BtlChar_GetObj(c, p_chr_1);
        F::BtlObjAnim_Play(c, l_obj_4, 1i32, p_anim_2, 0i32);
        { let a0_ = c.m.r32(((l_obj_4).wrapping_add(2880)).wrapping_add(152)); F::BtlAnim_ApplyTableFlags(c, a0_, p_anim_2) };
        { let a0_ = c.m.r32(((l_obj_4).wrapping_add(2880)).wrapping_add(152)); let a1_ = (c.m.r32((l_obj_4).wrapping_add(12)) as i32); F::BtlAnim_HideModelNodes(c, a0_, a1_) };
        { let v_ = p_anim_2; let a_ = (l_ids_3).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAnim_PlaySub", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAnim_SetUnkC8C(c: &mut Ctx, mut p_chr_1: u32, mut p_v_2: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAnim_SetUnkC8C", &PROBE_N, &[(p_chr_1) as u32, (p_v_2).0], false); }
    let r_ = 'f: {
    { let v_ = p_v_2; let a_ = ((F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2880)).wrapping_add(332); c.m.w32(a_, (v_).0); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAnim_SetUnkC8C", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAnim_SetStep(c: &mut Ctx, mut p_chr_1: u32, mut p_step_2: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAnim_SetStep", &PROBE_N, &[(p_chr_1) as u32, (p_step_2).0], false); }
    let r_ = 'f: {
    { let v_ = p_step_2; let a_ = ((F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2880)).wrapping_add(320); c.m.w32(a_, (v_).0); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAnim_SetStep", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAnim_SetDuration(c: &mut Ctx, mut p_chr_1: u32, mut p_seconds_2: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAnim_SetDuration", &PROBE_N, &[(p_chr_1) as u32, (p_seconds_2).0], false); }
    let mut l_a_3: u32 = 0;
    let r_ = 'f: {
    l_a_3 = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2880);
    if (((rt::flt(p_seconds_2, F32(0x3d088888)) as i32)) != 0) {
        { let v_ = F32(c.m.r32((l_a_3).wrapping_add(4))); let a_ = (l_a_3).wrapping_add(320); c.m.w32(a_, (v_).0); };
    } else {
        { let v_ = rt::fdiv(F32(c.m.r32((l_a_3).wrapping_add(4))), rt::fmul(p_seconds_2, F32(0x41f00000))); let a_ = (l_a_3).wrapping_add(320); c.m.w32(a_, (v_).0); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAnim_SetDuration", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAnim_SetObjRate(c: &mut Ctx, mut p_chr_1: u32, mut p_rate_2: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAnim_SetObjRate", &PROBE_N, &[(p_chr_1) as u32, (p_rate_2).0], false); }
    let r_ = 'f: {
    { let v_ = p_rate_2; let a_ = ((F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2880)).wrapping_add(376); c.m.w32(a_, (v_).0); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAnim_SetObjRate", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAnim_SetRate(c: &mut Ctx, mut p_chr_1: u32, mut p_rate_2: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAnim_SetRate", &PROBE_N, &[(p_chr_1) as u32, (p_rate_2).0], false); }
    let r_ = 'f: {
    { let v_ = p_rate_2; let a_ = (p_chr_1).wrapping_add(2440); c.m.w32(a_, (v_).0); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAnim_SetRate", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAnim_JumpToEnd(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAnim_JumpToEnd", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_a_2: u32 = 0;
    let r_ = 'f: {
    l_a_2 = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2880);
    { let v_ = F32(0x00000000); let a_ = (l_a_2).wrapping_add(324); c.m.w32(a_, (v_).0); };
    F::BtlChar_ClearFlag(c, p_chr_1, ((((52i32) as i64 as u64)) as u32));
    F::BtlChar_SetHeldFlag(c, p_chr_1, ((((49i32) as i64 as u64)) as u32));
    { let v_ = { let v_ = F32(c.m.r32((l_a_2).wrapping_add(4))); let a_ = (l_a_2).wrapping_add(312); c.m.w32(a_, (v_).0); v_ }; let a_ = (l_a_2).wrapping_add(316); c.m.w32(a_, (v_).0); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAnim_JumpToEnd", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAnim_EnableHandle(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAnim_EnableHandle", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    { let a0_ = { let a_ = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2880); c.m.r32(a_) }; F::BtlObjAnim_ZeroRootAxes(c, a0_, 1i32, 1i32, 1i32) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAnim_EnableHandle", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAnim_GetId(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAnim_GetId", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f (c.m.r32((p_chr_1).wrapping_add(2420)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAnim_GetId", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAnim_GetNextId(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAnim_GetNextId", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f (c.m.r32(((p_chr_1).wrapping_add(2420)).wrapping_add(4)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAnim_GetNextId", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAnim_GetPrevId(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAnim_GetPrevId", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f (c.m.r32(((p_chr_1).wrapping_add(2420)).wrapping_add(8)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAnim_GetPrevId", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAnim_GetFlags(c: &mut Ctx, mut p_anim_1: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAnim_GetFlags", &PROBE_N, &[(p_anim_1) as u32], false); }
    let mut l_roster_2: u32 = 0;
    let mut l_tbl_3: u32 = 0;
    let r_ = 'f: {
    l_roster_2 = c.m.r32(sym::gBtlChars);
    if (((((l_roster_2) == (((0i32) as u32))) as i32)) != 0) {
        break 'f ((0i32) as u32);
    }
    l_tbl_3 = c.m.r32((l_roster_2).wrapping_add(32));
    if (((((l_tbl_3) == (((0i32) as u32))) as i32)) != 0) {
        break 'f ((0i32) as u32);
    }
    if (((((p_anim_1) >= (((414i32) as u32))) as i32)) != 0) {
        break 'f ((0i32) as u32);
    }
    break 'f c.m.r32((l_tbl_3).wrapping_add((p_anim_1).wrapping_mul(4)));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAnim_GetFlags", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAnim_GetFrame(c: &mut Ctx, mut p_chr_1: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAnim_GetFrame", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f { let a_ = ((F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2880)).wrapping_add(312); F32(c.m.r32(a_)) };
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAnim_GetFrame", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAnim_GetProgress(c: &mut Ctx, mut p_chr_1: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAnim_GetProgress", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_a_2: u32 = 0;
    let r_ = 'f: {
    l_a_2 = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2880);
    break 'f rt::fdiv(F32(c.m.r32((l_a_2).wrapping_add(312))), F32(c.m.r32((l_a_2).wrapping_add(4))));
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAnim_GetProgress", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAnim_GetLength(c: &mut Ctx, mut p_chr_1: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAnim_GetLength", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f { let a_ = ((F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2880)).wrapping_add(4); F32(c.m.r32(a_)) };
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAnim_GetLength", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAnim_GetStep(c: &mut Ctx, mut p_chr_1: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAnim_GetStep", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f { let a_ = ((F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2880)).wrapping_add(320); F32(c.m.r32(a_)) };
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAnim_GetStep", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAnim_GetObjRate(c: &mut Ctx, mut p_chr_1: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAnim_GetObjRate", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f { let a_ = ((F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2880)).wrapping_add(376); F32(c.m.r32(a_)) };
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAnim_GetObjRate", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAnim_TestAttr(c: &mut Ctx, mut p_chr_1: u32, mut p_mask_2: u64) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAnim_TestAttr", &PROBE_N, &[(p_chr_1) as u32, (p_mask_2) as u32], false); }
    let r_ = 'f: {
    { let _ = F::BtlChar_GetObj(c, p_chr_1); };
    if ((((((c.m.r32((p_chr_1).wrapping_add(2448)) as i32)) > (0i32)) as i32)) != 0) {
        break 'f 0i32;
    }
    break 'f { let a0_ = F::BtlChar_GetObj(c, p_chr_1); F::BtlObjAnim_TestEvent(c, a0_, p_mask_2) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAnim_TestAttr", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAnim_Advance(c: &mut Ctx, mut p_chr_1: u32, mut p_flags_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAnim_Advance", &PROBE_N, &[(p_chr_1) as u32, (p_flags_2) as u32], false); }
    let mut l_done_3: i32 = 0;
    let mut l_a_4: u32 = 0;
    let r_ = 'f: {
    l_done_3 = 0i32;
    l_a_4 = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2880);
    { let r_ = F32(c.m.r32((l_a_4).wrapping_add(328))); let a_ = (l_a_4).wrapping_add(324); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
    if (((rt::flt(F32(c.m.r32((l_a_4).wrapping_add(324))), F32(0x00000000)) as i32)) != 0) {
        { let v_ = F32(0x00000000); let a_ = (l_a_4).wrapping_add(324); c.m.w32(a_, (v_).0); };
        F::BtlChar_ClearFlag(c, p_chr_1, ((((52i32) as i64 as u64)) as u32));
    } else {
        F::BtlChar_SetHeldFlag(c, p_chr_1, ((((52i32) as i64 as u64)) as u32));
    }
    { let v_ = F32(c.m.r32((l_a_4).wrapping_add(312))); let a_ = (l_a_4).wrapping_add(316); c.m.w32(a_, (v_).0); };
    if ((((((((!((((p_flags_2) & (1i32))) != 0)) as i32)) != 0) || (((rt::feq(F32(c.m.r32((l_a_4).wrapping_add(324))), F32(0x00000000)) as i32)) != 0)) as i32)) != 0) {
        { let r_ = F32(c.m.r32((l_a_4).wrapping_add(320))); let a_ = (l_a_4).wrapping_add(312); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        if (((rt::fge(F32(c.m.r32((l_a_4).wrapping_add(312))), rt::fsub(F32(c.m.r32((l_a_4).wrapping_add(4))), F32(0x3dcccccc))) as i32)) != 0) {
            F::BtlChar_SetHeldFlag(c, p_chr_1, ((((49i32) as i64 as u64)) as u32));
            l_done_3 = 1i32;
            { let v_ = F32(c.m.r32((l_a_4).wrapping_add(4))); let a_ = (l_a_4).wrapping_add(312); c.m.w32(a_, (v_).0); };
        }
    }
    { let v_ = (c.m.r32((p_chr_1).wrapping_add(2444)) as i32); let a_ = (p_chr_1).wrapping_add(2448); c.m.w32(a_, (v_) as u32); };
    if (((rt::feq(F32(c.m.r32((l_a_4).wrapping_add(316))), F32(c.m.r32((l_a_4).wrapping_add(312)))) as i32)) != 0) {
        { let a_ = (p_chr_1).wrapping_add(2444); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
    } else {
        { let v_ = 0i32; let a_ = (p_chr_1).wrapping_add(2444); c.m.w32(a_, (v_) as u32); };
    }
    break 'f l_done_3;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAnim_Advance", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAnim_AdvanceThen(c: &mut Ctx, mut p_chr_1: u32, mut p_next_2: i32, mut p_blend_3: F32, mut p_flags_4: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAnim_AdvanceThen", &PROBE_N, &[(p_chr_1) as u32, (p_next_2) as u32, (p_blend_3).0, (p_flags_4) as u32], false); }
    let mut l_done_5: i32 = 0;
    let r_ = 'f: {
    l_done_5 = F::BtlAnim_Advance(c, p_chr_1, p_flags_4);
    if ((l_done_5) != 0) {
        F::BtlAnim_Request(c, p_chr_1, p_next_2, p_blend_3);
    }
    break 'f l_done_5;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAnim_AdvanceThen", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAnim_AdvanceLoop(c: &mut Ctx, mut p_chr_1: u32, mut p_flags_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAnim_AdvanceLoop", &PROBE_N, &[(p_chr_1) as u32, (p_flags_2) as u32], false); }
    let mut l_a_3: u32 = 0;
    let r_ = 'f: {
    l_a_3 = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2880);
    { let r_ = F32(c.m.r32((l_a_3).wrapping_add(328))); let a_ = (l_a_3).wrapping_add(324); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
    if (((rt::flt(F32(c.m.r32((l_a_3).wrapping_add(324))), F32(0x00000000)) as i32)) != 0) {
        { let v_ = F32(0x00000000); let a_ = (l_a_3).wrapping_add(324); c.m.w32(a_, (v_).0); };
        F::BtlChar_ClearFlag(c, p_chr_1, ((((52i32) as i64 as u64)) as u32));
    } else {
        F::BtlChar_SetHeldFlag(c, p_chr_1, ((((52i32) as i64 as u64)) as u32));
    }
    { let v_ = F32(c.m.r32((l_a_3).wrapping_add(312))); let a_ = (l_a_3).wrapping_add(316); c.m.w32(a_, (v_).0); };
    if ((((((((!((((p_flags_2) & (1i32))) != 0)) as i32)) != 0) || (((rt::feq(F32(c.m.r32((l_a_3).wrapping_add(324))), F32(0x00000000)) as i32)) != 0)) as i32)) != 0) {
        { let r_ = F32(c.m.r32((l_a_3).wrapping_add(320))); let a_ = (l_a_3).wrapping_add(312); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        if (((rt::fge(F32(c.m.r32((l_a_3).wrapping_add(312))), rt::fsub(F32(c.m.r32((l_a_3).wrapping_add(4))), F32(0x3dcccccc))) as i32)) != 0) {
            { let v_ = F32(0x00000000); let a_ = (l_a_3).wrapping_add(312); c.m.w32(a_, (v_).0); };
        }
    }
    { let v_ = (c.m.r32((p_chr_1).wrapping_add(2444)) as i32); let a_ = (p_chr_1).wrapping_add(2448); c.m.w32(a_, (v_) as u32); };
    if (((rt::feq(F32(c.m.r32((l_a_3).wrapping_add(316))), F32(c.m.r32((l_a_3).wrapping_add(312)))) as i32)) != 0) {
        { let a_ = (p_chr_1).wrapping_add(2444); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
    } else {
        { let v_ = 0i32; let a_ = (p_chr_1).wrapping_add(2444); c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAnim_AdvanceLoop", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAnim_PassedRatio(c: &mut Ctx, mut p_chr_1: u32, mut p_ratio_2: F32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAnim_PassedRatio", &PROBE_N, &[(p_chr_1) as u32, (p_ratio_2).0], false); }
    let mut l_a_3: u32 = 0;
    let mut l_t_4: F32 = F32(0);
    let r_ = 'f: {
    l_a_3 = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2880);
    l_t_4 = rt::fmul(F32(c.m.r32((l_a_3).wrapping_add(4))), p_ratio_2);
    if (((rt::fge(F32(c.m.r32((l_a_3).wrapping_add(312))), F32(c.m.r32((l_a_3).wrapping_add(316)))) as i32)) != 0) {
        if (((((((rt::fle(F32(c.m.r32((l_a_3).wrapping_add(316))), l_t_4) as i32)) != 0) && (((rt::flt(l_t_4, F32(c.m.r32((l_a_3).wrapping_add(312)))) as i32)) != 0)) as i32)) != 0) {
            break 'f 1i32;
        }
    } else {
        if (((((((rt::fle(F32(c.m.r32((l_a_3).wrapping_add(316))), l_t_4) as i32)) != 0) || (((rt::flt(l_t_4, F32(c.m.r32((l_a_3).wrapping_add(312)))) as i32)) != 0)) as i32)) != 0) {
            break 'f 1i32;
        }
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAnim_PassedRatio", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAnim_PassedFrame(c: &mut Ctx, mut p_chr_1: u32, mut p_frame_2: F32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAnim_PassedFrame", &PROBE_N, &[(p_chr_1) as u32, (p_frame_2).0], false); }
    let mut l_a_3: u32 = 0;
    let r_ = 'f: {
    l_a_3 = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2880);
    if (((rt::fge(F32(c.m.r32((l_a_3).wrapping_add(312))), F32(c.m.r32((l_a_3).wrapping_add(316)))) as i32)) != 0) {
        if (((((((rt::fle(F32(c.m.r32((l_a_3).wrapping_add(316))), p_frame_2) as i32)) != 0) && (((rt::flt(p_frame_2, F32(c.m.r32((l_a_3).wrapping_add(312)))) as i32)) != 0)) as i32)) != 0) {
            break 'f 1i32;
        }
    } else {
        if (((((((rt::fle(F32(c.m.r32((l_a_3).wrapping_add(316))), p_frame_2) as i32)) != 0) || (((rt::flt(p_frame_2, F32(c.m.r32((l_a_3).wrapping_add(312)))) as i32)) != 0)) as i32)) != 0) {
            break 'f 1i32;
        }
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAnim_PassedFrame", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAnim_InRatioRange(c: &mut Ctx, mut p_chr_1: u32, mut p_lo_2: F32, mut p_hi_3: F32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAnim_InRatioRange", &PROBE_N, &[(p_chr_1) as u32, (p_lo_2).0, (p_hi_3).0], false); }
    let mut l_len_4: F32 = F32(0);
    let r_ = 'f: {
    l_len_4 = { let a_ = ((F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2880)).wrapping_add(4); F32(c.m.r32(a_)) };
    break 'f F::BtlAnim_InFrameRange(c, p_chr_1, rt::fmul(l_len_4, p_lo_2), rt::fmul(l_len_4, p_hi_3));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAnim_InRatioRange", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAnim_InFrameRange(c: &mut Ctx, mut p_chr_1: u32, mut p_lo_2: F32, mut p_hi_3: F32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAnim_InFrameRange", &PROBE_N, &[(p_chr_1) as u32, (p_lo_2).0, (p_hi_3).0], false); }
    let mut l_a_4: u32 = 0;
    let r_ = 'f: {
    l_a_4 = (F::BtlChar_GetObj(c, p_chr_1)).wrapping_add(2880);
    if (((rt::fge(F32(c.m.r32((l_a_4).wrapping_add(312))), F32(c.m.r32((l_a_4).wrapping_add(316)))) as i32)) != 0) {
        if (((((((rt::fle(F32(c.m.r32((l_a_4).wrapping_add(316))), p_hi_3) as i32)) != 0) && (((rt::fle(p_lo_2, F32(c.m.r32((l_a_4).wrapping_add(312)))) as i32)) != 0)) as i32)) != 0) {
            break 'f 1i32;
        }
    } else {
        if (((((((rt::fle(F32(c.m.r32((l_a_4).wrapping_add(316))), p_hi_3) as i32)) != 0) || (((rt::fle(p_lo_2, F32(c.m.r32((l_a_4).wrapping_add(312)))) as i32)) != 0)) as i32)) != 0) {
            break 'f 1i32;
        }
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAnim_InFrameRange", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAnim_IsNew(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAnim_IsNew", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f (((F::BtlChar_TestFlag(c, p_chr_1, ((((43i32) as i64 as u64)) as u32))) != (0i32)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAnim_IsNew", &PROBE_N, &[], true); }
    r_
}

pub fn BtlAnim_WasNew(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlAnim_WasNew", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f (((F::BtlChar_TestPrevFlag(c, p_chr_1, ((((43i32) as i64 as u64)) as u32))) != (0i32)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlAnim_WasNew", &PROBE_N, &[], true); }
    r_
}
