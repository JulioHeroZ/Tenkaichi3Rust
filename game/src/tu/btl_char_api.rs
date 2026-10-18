#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn BtlCharApi_ObjHasFlags22(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_ObjHasFlags22", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_obj_2: u32 = 0;
    let r_ = 'f: {
    l_obj_2 = F::BtlObj_Get(c, p_objId_1);
    if (((((l_obj_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f (((((c.m.r64((l_obj_2).wrapping_add(2624))) & (((34i32) as u64)))) == (((34i32) as u64))) as i32);
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_ObjHasFlags22", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_ObjHasFlags42(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_ObjHasFlags42", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_obj_2: u32 = 0;
    let r_ = 'f: {
    l_obj_2 = F::BtlObj_Get(c, p_objId_1);
    if (((((l_obj_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f (((((c.m.r64((l_obj_2).wrapping_add(2624))) & (((66i32) as u64)))) == (((66i32) as u64))) as i32);
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_ObjHasFlags42", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_AnyHasFlag128(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_AnyHasFlag128", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let r_ = 'f: {
    if (((((c.m.r32(sym::gBtlChars)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (F::BtlChar_GetCount(c))) as i32)) != 0) { break 'l1; }
            'c2: {
                if (({ let a0_ = F::BtlChar_Get(c, l_i_1); F::BtlChar_TestFlag(c, a0_, ((((296i32) as i64 as u64)) as u32)) }) != 0) {
                    break 'f 1i32;
                }
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_AnyHasFlag128", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_IsHpEmpty(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_IsHpEmpty", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f F::BtlChar_IsDead(c, l_chr_2);
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_IsHpEmpty", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_TestFlagA4(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_TestFlagA4", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f F::BtlChar_TestFlag(c, l_chr_2, ((((164i32) as i64 as u64)) as u32));
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_TestFlagA4", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_GetMemberUnk60(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetMemberUnk60", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f { let a_ = (F::BtlMember_GetActiveGauge(c, l_chr_2)).wrapping_add(32); (c.m.r32(a_) as i32) };
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetMemberUnk60", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_IsFlag8Action104(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_IsFlag8Action104", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        if ((((((F::BtlChar_TestFlag(c, l_chr_2, ((((8i32) as i64 as u64)) as u32))) != 0) && (((((F::BtlAct_GetCurrent(c, l_chr_2)) == (260i32)) as i32)) != 0)) as i32)) != 0) {
            break 'f 1i32;
        }
        break 'f 0i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_IsFlag8Action104", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_IsAction103OrFlagA6(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_IsAction103OrFlagA6", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        if (((((F::BtlAct_GetCurrent(c, l_chr_2)) == (259i32)) as i32)) != 0) {
            break 'f 1i32;
        }
        break 'f (((F::BtlChar_TestFlag(c, l_chr_2, ((((166i32) as i64 as u64)) as u32))) != (0i32)) as i32);
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_IsAction103OrFlagA6", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_HasMemberUnk70(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_HasMemberUnk70", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f F::BtlChar_TestMemberUnk70(c, l_chr_2);
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_HasMemberUnk70", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_SetHeldFlagA7(c: &mut Ctx, mut p_objId_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_SetHeldFlagA7", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        F::BtlChar_SetHeldFlag(c, l_chr_2, ((((167i32) as i64 as u64)) as u32));
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_SetHeldFlagA7", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_SetHeldFlagA8(c: &mut Ctx, mut p_objId_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_SetHeldFlagA8", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        F::BtlChar_SetHeldFlag(c, l_chr_2, ((((168i32) as i64 as u64)) as u32));
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_SetHeldFlagA8", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_SetHeldFlagA9(c: &mut Ctx, mut p_objId_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_SetHeldFlagA9", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        F::BtlChar_SetHeldFlag(c, l_chr_2, ((((169i32) as i64 as u64)) as u32));
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_SetHeldFlagA9", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_SetHeldFlagAA(c: &mut Ctx, mut p_objId_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_SetHeldFlagAA", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        F::BtlChar_SetHeldFlag(c, l_chr_2, ((((170i32) as i64 as u64)) as u32));
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_SetHeldFlagAA", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_SetHeldFlagAB(c: &mut Ctx, mut p_objId_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_SetHeldFlagAB", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        F::BtlChar_SetHeldFlag(c, l_chr_2, ((((171i32) as i64 as u64)) as u32));
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_SetHeldFlagAB", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_PlaySoundAt(c: &mut Ctx, mut p_pos_1: u32, mut p_kind_2: i32, mut p_id_3: i32, mut p_near_4: F32, mut p_far_5: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_PlaySoundAt", &PROBE_N, &[(p_pos_1) as u32, (p_kind_2) as u32, (p_id_3) as u32, (p_near_4).0, (p_far_5).0], false); }
    let r_ = 'f: {
    F::BtlCharSnd_RequestAt(c, p_pos_1, p_kind_2, p_id_3, p_near_4, p_far_5);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_PlaySoundAt", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_PlayTechniqueSound(c: &mut Ctx, mut p_objId_1: i32, mut p_n_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_PlayTechniqueSound", &PROBE_N, &[(p_objId_1) as u32, (p_n_2) as u32], false); }
    let mut l_base_3: i32 = 0;
    let mut l_chr_4: u32 = 0;
    let r_ = 'f: {
    l_base_3 = (1i32).wrapping_neg();
    l_chr_4 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((((((l_chr_4) != (((0i32) as u32))) as i32)) != 0) && (((((p_n_2) < (((4i32) as u32))) as i32)) != 0)) as i32)) != 0) {
        's1: {
            let sv5: i32 = F::BtlAct_GetCurrentClass(c, l_chr_4);
            match sv5 {
                0i32 => {
                    l_base_3 = 37i32;
                    break 's1;
                }
                1i32 => {
                    l_base_3 = 41i32;
                    break 's1;
                }
                _ => {
                }
            }
        }
        if (((((l_base_3) >= (0i32)) as i32)) != 0) {
            F::BtlCharSnd_PlayOwn(c, l_chr_4, (((((l_base_3) as u32)).wrapping_add(p_n_2)) as i32));
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_PlayTechniqueSound", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_GetSoundCount(c: &mut Ctx, mut p_side_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetSoundCount", &PROBE_N, &[(p_side_1) as u32], false); }
    let mut l_count_2: i32 = 0;
    let mut l_i_3: i32 = 0;
    let mut l_e_4: u32 = 0;
    let r_ = 'f: {
    l_count_2 = 0i32;
    if (((((c.m.r32(sym::gBtlChars)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    l_e_4 = ({ let a_ = (c.m.r32(sym::gBtlChars)).wrapping_add(12); c.m.r32(a_) }).wrapping_add((((p_side_1) as u32)).wrapping_mul(52));
    {
        l_i_3 = 0i32;
        'l1: loop {
            if !(((((l_i_3) < (4i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((((((c.m.r32(l_e_4) as i32)) >= (0i32)) as i32)) != 0) {
                    { let o_ = l_count_2; let n_ = (o_).wrapping_add(1); l_count_2 = n_; };
                }
            }
            { { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; }; { let o_ = l_e_4; let n_ = (o_).wrapping_add(12); l_e_4 = n_; }; };
        }
    }
    break 'f l_count_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetSoundCount", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_GetSound(c: &mut Ctx, mut p_side_1: i32, mut p_n_2: i32, mut p_handle_3: u32, mut p_out3_4: u32, mut p_out4_5: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetSound", &PROBE_N, &[(p_side_1) as u32, (p_n_2) as u32, (p_handle_3) as u32, (p_out3_4) as u32, (p_out4_5) as u32], false); }
    let mut l_count_6: i32 = 0;
    let mut l_i_7: i32 = 0;
    let mut l_e_8: u32 = 0;
    let r_ = 'f: {
    l_count_6 = 0i32;
    if (((((c.m.r32(sym::gBtlChars)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    l_e_8 = ({ let a_ = (c.m.r32(sym::gBtlChars)).wrapping_add(12); c.m.r32(a_) }).wrapping_add((((p_side_1) as u32)).wrapping_mul(52));
    {
        l_i_7 = 0i32;
        'l1: loop {
            if !(((((l_i_7) < (4i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((((((c.m.r32(l_e_8) as i32)) >= (0i32)) as i32)) != 0) {
                    if (((((l_count_6) == (p_n_2)) as i32)) != 0) {
                        if (((((p_handle_3) != (((0i32) as u32))) as i32)) != 0) {
                            { let v_ = (c.m.r32(l_e_8) as i32); let a_ = p_handle_3; c.m.w32(a_, (v_) as u32); };
                        }
                        if (((((p_out3_4) != (((0i32) as u32))) as i32)) != 0) {
                            { let v_ = { let a0_ = ((c.m.r8((l_e_8).wrapping_add(8))) as i32); F::BtlCharSnd_GetBankMask(c, a0_) }; let a_ = p_out3_4; c.m.w32(a_, (v_) as u32); };
                        }
                        if (((((p_out4_5) != (((0i32) as u32))) as i32)) != 0) {
                            { let v_ = (c.m.r32((l_e_8).wrapping_add(4)) as i32); let a_ = p_out4_5; c.m.w32(a_, (v_) as u32); };
                        }
                        break 'f;
                    }
                    { let o_ = l_count_6; let n_ = (o_).wrapping_add(1); l_count_6 = n_; };
                }
            }
            { { let o_ = l_i_7; let n_ = (o_).wrapping_add(1); l_i_7 = n_; }; { let o_ = l_e_8; let n_ = (o_).wrapping_add(12); l_e_8 = n_; }; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetSound", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_ObjClearMaskBit3(c: &mut Ctx, mut p_objId_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_ObjClearMaskBit3", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_obj_2: u32 = 0;
    let r_ = 'f: {
    l_obj_2 = F::BtlObj_Get(c, p_objId_1);
    if (((((l_obj_2) != (((0i32) as u32))) as i32)) != 0) {
        F::BtlObj_SetColorMode(c, l_obj_2, 3i32, 0i32);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_ObjClearMaskBit3", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_ObjSetMaskBit3(c: &mut Ctx, mut p_objId_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_ObjSetMaskBit3", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_obj_2: u32 = 0;
    let r_ = 'f: {
    l_obj_2 = F::BtlObj_Get(c, p_objId_1);
    if (((((l_obj_2) != (((0i32) as u32))) as i32)) != 0) {
        F::BtlObj_SetColorMode(c, l_obj_2, 3i32, 1i32);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_ObjSetMaskBit3", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_ObjTestFlagBit21(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_ObjTestFlagBit21", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_obj_2: u32 = 0;
    let r_ = 'f: {
    l_obj_2 = F::BtlObj_Get(c, p_objId_1);
    if (((((l_obj_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f ((((c.m.r32((l_obj_2).wrapping_add(2624)) as i32)).wrapping_shr((21i32) as u32)) & (1i32));
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_ObjTestFlagBit21", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_GetUnkE44(c: &mut Ctx, mut p_objId_1: i32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetUnkE44", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f F32(c.m.r32((l_chr_2).wrapping_add(3652)));
    }
    break 'f F32(0x00000000);
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetUnkE44", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_GetUnkE5CRatio(c: &mut Ctx, mut p_objId_1: i32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetUnkE5CRatio", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) == (((0i32) as u32))) as i32)) != 0) {
        break 'f F32(0x00000000);
    }
    break 'f { let a0_ = rt::fdiv(rt::i2f((c.m.r32((l_chr_2).wrapping_add(3676)) as i32)), F32(0x40400000)); F::BtlUtil_ClampF(c, a0_, F32(0x00000000), F32(0x3f800000)) };
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetUnkE5CRatio", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_GetUnkE60Ratio(c: &mut Ctx, mut p_objId_1: i32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetUnkE60Ratio", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) == (((0i32) as u32))) as i32)) != 0) {
        break 'f F32(0x00000000);
    }
    break 'f { let a0_ = rt::fdiv(rt::i2f((c.m.r32((l_chr_2).wrapping_add(3680)) as i32)), F32(0x40a00000)); F::BtlUtil_ClampF(c, a0_, F32(0x00000000), F32(0x3f800000)) };
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetUnkE60Ratio", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_ObjSetFlag100(c: &mut Ctx, mut p_objId_1: i32, mut p_on_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_ObjSetFlag100", &PROBE_N, &[(p_objId_1) as u32, (p_on_2) as u32], false); }
    let mut l_obj_3: u32 = 0;
    let r_ = 'f: {
    l_obj_3 = F::BtlObj_Get(c, p_objId_1);
    if (((((l_obj_3) != (((0i32) as u32))) as i32)) != 0) {
        if ((p_on_2) != 0) {
            { let r_ = 256i32; let a_ = (l_obj_3).wrapping_add(2624); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        } else {
            { let r_ = (!(256i32)); let a_ = (l_obj_3).wrapping_add(2624); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_ObjSetFlag100", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_ObjHasFlags102(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_ObjHasFlags102", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_obj_2: u32 = 0;
    let r_ = 'f: {
    l_obj_2 = F::BtlObj_Get(c, p_objId_1);
    if (((((l_obj_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f (((((c.m.r64((l_obj_2).wrapping_add(2624))) & (((258i32) as u64)))) == (((258i32) as u64))) as i32);
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_ObjHasFlags102", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_IsTargetBelowHalfHp(c: &mut Ctx, mut p_objId_1: i32, mut p_targetId_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_IsTargetBelowHalfHp", &PROBE_N, &[(p_objId_1) as u32, (p_targetId_2) as u32], false); }
    let mut l_chr_3: u32 = 0;
    let mut l_target_4: u32 = 0;
    let mut l_result_5: i32 = 0;
    let r_ = 'f: {
    l_chr_3 = F::BtlChar_FindByObjId(c, p_objId_1);
    l_target_4 = F::BtlChar_FindByObjId(c, p_targetId_2);
    l_result_5 = 0i32;
    if (((((((((l_chr_3) != (((0i32) as u32))) as i32)) != 0) && (((((l_target_4) != (((0i32) as u32))) as i32)) != 0)) as i32)) != 0) {
        if ((F::BtlChar_IsDead(c, l_chr_3)) != 0) {
            break 'f 0i32;
        }
        if ((F::BtlChar_IsDead(c, l_target_4)) != 0) {
            break 'f 0i32;
        }
        break 'f ((({ let a_ = F::BtlMember_GetActiveGauge(c, l_target_4); (c.m.r32(a_) as i32) }) < (rt::div_i32({ let a_ = (F::BtlMember_GetActiveGauge(c, l_target_4)).wrapping_add(4); (c.m.r32(a_) as i32) }, 2i32))) as i32);
    }
    break 'f l_result_5;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_IsTargetBelowHalfHp", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_CanTechniqueFinish(c: &mut Ctx, mut p_objId_1: i32, mut p_targetId_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_CanTechniqueFinish", &PROBE_N, &[(p_objId_1) as u32, (p_targetId_2) as u32], false); }
    let mut l_chr_3: u32 = 0;
    let mut l_target_4: u32 = 0;
    let mut l_slot_5: i32 = 0;
    let mut l_result_6: i32 = 0;
    let r_ = 'f: {
    l_chr_3 = F::BtlChar_FindByObjId(c, p_objId_1);
    l_target_4 = F::BtlChar_FindByObjId(c, p_targetId_2);
    l_result_6 = 0i32;
    if (((((((((l_chr_3) != (((0i32) as u32))) as i32)) != 0) && (((((l_target_4) != (((0i32) as u32))) as i32)) != 0)) as i32)) != 0) {
        if ((((!(({ let a0_ = F::BtlAct_GetCurrent(c, l_chr_3); F::BtlAct_IsTechniqueId(c, a0_) }) != 0)) as i32)) != 0) {
            break 'f 0i32;
        }
        l_slot_5 = F::BtlAct_GetCurrentClass(c, l_chr_3);
        if ((((!((((((((F::BtlSuper_GetFlags(c, l_chr_3, l_slot_5)) as u64)) as i32)) & (524288i32))) != 0)) as i32)) != 0) {
            break 'f 0i32;
        }
        if ((F::BtlSuper_IsThrow(c, l_chr_3, l_slot_5)) != 0) {
            break 'f 0i32;
        }
        break 'f ((({ let a_ = (F::BtlMember_GetActiveGauge(c, l_target_4)).wrapping_add(32); (c.m.r32(a_) as i32) }) == (0i32)) as i32);
    }
    break 'f l_result_6;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_CanTechniqueFinish", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_GetMgrTimerRatio(c: &mut Ctx) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetMgrTimerRatio", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if (((((c.m.r32(sym::gBtlChars)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f F32(0x00000000);
    }
    break 'f { let a0_ = rt::fdiv(rt::i2f({ let a_ = (c.m.r32(sym::gBtlChars)).wrapping_add(28); (c.m.r32(a_) as i32) }), F32(0x42b40000)); F::BtlUtil_ClampF(c, a0_, F32(0x00000000), F32(0x3f800000)) };
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetMgrTimerRatio", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_RumbleNear(c: &mut Ctx, mut p_pos_1: u32, mut p_near_2: F32, mut p_far_3: F32, mut p_power_4: F32, mut p_time_5: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_RumbleNear", &PROBE_N, &[(p_pos_1) as u32, (p_near_2).0, (p_far_3).0, (p_power_4).0, (p_time_5).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_i_6: i32 = 0;
    let mut l_chr_7: u32 = 0;
    let mut l_rate_8: F32 = F32(0);
    let mut l_t_9: F32 = F32(0);
    let r_ = 'f: {
        if (((((c.m.r32(sym::gBtlChars)) == (((0i32) as u32))) as i32)) != 0) {
            break 'f;
        }
        {
            l_i_6 = 0i32;
            'l1: loop {
                if !(((((l_i_6) < (F::BtlChar_GetCount(c))) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_chr_7 = F::BtlChar_Get(c, l_i_6);
                    { let a1_ = F::BtlChar_GetPos(c, l_chr_7); F::Vec4_Sub(c, fp, a1_, p_pos_1) };
                    l_rate_8 = rt::fsub(F32(0x3f800000), rt::fdiv(rt::fsub(F::Vec3_Length(c, fp), p_near_2), rt::fsub(p_far_3, p_near_2)));
                    if (((rt::flt(l_rate_8, F32(0x00000000)) as i32)) != 0) {
                        break 'c2;
                    }
                    if (((rt::fgt(l_rate_8, F32(0x3f800000)) as i32)) != 0) {
                        l_rate_8 = F32(0x3f800000);
                    }
                    l_t_9 = rt::fmul(p_time_5, l_rate_8);
                    F::BtlChar_SetVibration(c, l_chr_7, rt::fmul(p_power_4, l_rate_8), l_t_9);
                    if (((rt::fgt(l_rate_8, F32(0x3f000000)) as i32)) != 0) {
                        F::BtlChar_SetSmallVibration(c, l_chr_7, l_t_9);
                    }
                }
                { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_RumbleNear", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_ObjTestAttr(c: &mut Ctx, mut p_objId_1: i32, mut p_mask_2: u64) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_ObjTestAttr", &PROBE_N, &[(p_objId_1) as u32, (p_mask_2) as u32], false); }
    let mut l_chr_3: u32 = 0;
    let mut l_obj_4: u32 = 0;
    let r_ = 'f: {
    l_chr_3 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_3) != (((0i32) as u32))) as i32)) != 0) {
        break 'f F::BtlAnim_TestAttr(c, l_chr_3, p_mask_2);
    }
    l_obj_4 = F::BtlObj_Get(c, p_objId_1);
    if (((((l_obj_4) != (((0i32) as u32))) as i32)) != 0) {
        break 'f F::BtlObjAnim_TestEvent(c, l_obj_4, p_mask_2);
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_ObjTestAttr", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_ObjGetAttrValue(c: &mut Ctx, mut p_objId_1: i32, mut p_mask_2: u64) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_ObjGetAttrValue", &PROBE_N, &[(p_objId_1) as u32, (p_mask_2) as u32], false); }
    let mut l_obj_3: u32 = 0;
    let r_ = 'f: {
    l_obj_3 = F::BtlObj_Get(c, p_objId_1);
    if (((((l_obj_3) != (((0i32) as u32))) as i32)) != 0) {
        break 'f ((((F::BtlObjAnim_GetEventArg(c, l_obj_3, p_mask_2)) as u64)) as i32);
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_ObjGetAttrValue", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_ObjGetAttrKind(c: &mut Ctx, mut p_objId_1: i32, mut p_mask_2: u64) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_ObjGetAttrKind", &PROBE_N, &[(p_objId_1) as u32, (p_mask_2) as u32], false); }
    let mut l_obj_3: u32 = 0;
    let mut l_bits_4: i32 = 0;
    let mut l_kind_5: i32 = 0;
    let r_ = 'f: {
    l_obj_3 = F::BtlObj_Get(c, p_objId_1);
    if (((((l_obj_3) != (((0i32) as u32))) as i32)) != 0) {
        l_bits_4 = ((((F::BtlObjAnim_GetEventArg(c, l_obj_3, p_mask_2)) as u64)) as i32);
        l_kind_5 = 0i32;
        if (((((l_bits_4) != (0i32)) as i32)) != 0) {
            l_kind_5 = F::BtlObjAnim_MaskToNode(c, ((((l_bits_4) as i64 as u64)) as u32));
        }
        break 'f l_kind_5;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_ObjGetAttrKind", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_GetChargedUnkC78(c: &mut Ctx, mut p_objId_1: i32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetChargedUnkC78", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let mut l_value_3: F32 = F32(0);
    let mut l_n_4: i32 = 0;
    let mut l_i_5: i32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) == (((0i32) as u32))) as i32)) != 0) {
        break 'f F32(0x00000000);
    }
    l_value_3 = F::BtlAnim_GetFrame(c, l_chr_2);
    if ((F::BtlCharApi_IsInRushSequence(c, p_objId_1)) != 0) {
        l_n_4 = { let a1_ = F::BtlAnim_GetId(c, l_chr_2); F::BtlAct_GetMotionLevel(c, l_chr_2, a1_) };
        {
            l_i_5 = 0i32;
            'l1: loop {
                if !(((((l_i_5) < (l_n_4)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let r_ = rt::fadd(F32(c.m.r32(((l_chr_2).wrapping_add(3836)).wrapping_add((((l_i_5) as u32)).wrapping_mul(4)))), F32(0x3f800000)); let n_ = rt::fadd(l_value_3, r_); l_value_3 = n_; };
                }
                { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
            }
        }
    }
    break 'f l_value_3;
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetChargedUnkC78", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_ObjGetUnkC78(c: &mut Ctx, mut p_objId_1: i32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_ObjGetUnkC78", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_obj_2: u32 = 0;
    let r_ = 'f: {
    l_obj_2 = F::BtlObj_Get(c, p_objId_1);
    if (((((l_obj_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f F32(c.m.r32((l_obj_2).wrapping_add(3192)));
    }
    break 'f F32(0x00000000);
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_ObjGetUnkC78", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_ObjGetUnkC80(c: &mut Ctx, mut p_objId_1: i32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_ObjGetUnkC80", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_obj_2: u32 = 0;
    let r_ = 'f: {
    l_obj_2 = F::BtlObj_Get(c, p_objId_1);
    if (((((l_obj_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f F32(c.m.r32((l_obj_2).wrapping_add(3200)));
    }
    break 'f F32(0x00000000);
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_ObjGetUnkC80", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_ObjQuery24D610(c: &mut Ctx, mut p_objId_1: i32, mut p_arg1_2: i32, mut p_arg2_3: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_ObjQuery24D610", &PROBE_N, &[(p_objId_1) as u32, (p_arg1_2) as u32, (p_arg2_3) as u32], false); }
    let mut l_obj_4: u32 = 0;
    let r_ = 'f: {
    l_obj_4 = F::BtlObj_Get(c, p_objId_1);
    if (((((l_obj_4) != (((0i32) as u32))) as i32)) != 0) {
        break 'f F::BtlObjAnim_QueryEvent(c, l_obj_4, ((p_arg1_2) as u64), 0i32, ((((p_arg2_3) as i64 as u64)) as u32));
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_ObjQuery24D610", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_GetUnk974(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetUnk974", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f F::BtlAnim_GetId(c, l_chr_2);
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetUnk974", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_GetUnk974Data(c: &mut Ctx, mut p_objId_1: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetUnk974Data", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f { let a0_ = F::BtlAnim_GetId(c, l_chr_2); F::BtlAnim_GetFlags(c, ((((a0_) as i64 as u64)) as u32)) };
    }
    break 'f ((0i32) as u32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetUnk974Data", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_TestFlag2B(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_TestFlag2B", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f F::BtlChar_TestFlag(c, l_chr_2, ((((43i32) as i64 as u64)) as u32));
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_TestFlag2B", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_GetCamPose(c: &mut Ctx, mut p_objId_1: i32, mut p_pos_2: u32, mut p_rot_3: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetCamPose", &PROBE_N, &[(p_objId_1) as u32, (p_pos_2) as u32, (p_rot_3) as u32], false); }
    let mut l_chr_4: u32 = 0;
    let r_ = 'f: {
    l_chr_4 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_4) != (((0i32) as u32))) as i32)) != 0) {
        F::Vec4_Copy(c, p_pos_2, (l_chr_4).wrapping_add(1072));
        F::Vec4_Copy(c, p_rot_3, (l_chr_4).wrapping_add(1088));
        break 'f (c.m.r32((l_chr_4).wrapping_add(1172)) as i32);
    }
    F::Vec4_SetZeroW1(c, p_pos_2);
    F::Vec4_SetZero(c, p_rot_3);
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetCamPose", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_GetCamUnk4A0(c: &mut Ctx, mut p_objId_1: i32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetCamUnk4A0", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f F32(c.m.r32((l_chr_2).wrapping_add(1184)));
    }
    break 'f F32(0x00000000);
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetCamUnk4A0", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_HasCamPriority(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_HasCamPriority", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f (((F::BtlChar_TestFlag(c, l_chr_2, ((((211i32) as i64 as u64)) as u32))) != (0i32)) as i32);
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_HasCamPriority", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_ShakeCamsNear(c: &mut Ctx, mut p_pos_1: u32, mut p_near_2: F32, mut p_far_3: F32, mut p_arg3_4: F32, mut p_arg4_5: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_ShakeCamsNear", &PROBE_N, &[(p_pos_1) as u32, (p_near_2).0, (p_far_3).0, (p_arg3_4).0, (p_arg4_5).0], false); }
    let mut l_i_6: i32 = 0;
    let mut l_chr_7: u32 = 0;
    let mut l_dist_8: F32 = F32(0);
    let mut l_rate_9: F32 = F32(0);
    let r_ = 'f: {
    if (((((c.m.r32(sym::gBtlChars)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    {
        l_i_6 = 0i32;
        'l1: loop {
            if !(((((l_i_6) < (F::BtlChar_GetCount(c))) as i32)) != 0) { break 'l1; }
            'c2: {
                l_chr_7 = F::BtlChar_Get(c, l_i_6);
                if ((F::BtlChar_IsFrozen(c, l_chr_7)) != 0) {
                    break 'c2;
                }
                l_dist_8 = F::Vec3_Dist(c, (l_chr_7).wrapping_add(1056), p_pos_1);
                if (((rt::flt(l_dist_8, p_far_3) as i32)) != 0) {
                    l_rate_9 = F::BtlUtil_ClampF(c, rt::fsub(F32(0x3f800000), rt::fdiv(rt::fsub(l_dist_8, p_near_2), rt::fsub(p_far_3, p_near_2))), F32(0x00000000), F32(0x3f800000));
                    F::ChrCam_AddShake(c, l_chr_7, rt::fmul(p_arg3_4, l_rate_9), rt::fmul(p_arg4_5, l_rate_9));
                }
            }
            { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_ShakeCamsNear", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_IsCamShown(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_IsCamShown", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let mut l_mine_3: i32 = 0;
    let mut l_other_4: i32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((F::BtlCharApi_IsInClashA(c, p_objId_1)) != 0) {
        break 'f 0i32;
    }
    if ((F::BtlCharApi_IsInClashBC(c, p_objId_1)) != 0) {
        break 'f 0i32;
    }
    if ((F::DemoCam_IsActive(c)) != 0) {
        break 'f 1i32;
    }
    l_mine_3 = F::BtlCharApi_HasCamPriority(c, p_objId_1);
    l_other_4 = { let a0_ = F::BtlOpp_GetObjId(c, l_chr_2); F::BtlCharApi_HasCamPriority(c, a0_) };
    if (((((l_other_4) < (l_mine_3)) as i32)) != 0) {
        break 'f 1i32;
    }
    if (((((l_mine_3) < (l_other_4)) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((F::Battle_IsSplitScreen(c)) != 0) {
        if ((((!((F::BattleReplay_IsActive(c)) != 0)) as i32)) != 0) {
            break 'f 1i32;
        }
    }
    break 'f ((((c.m.r32(l_chr_2) as i32)) == (F::BtlCam_GetDefaultView(c))) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_IsCamShown", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_GetCamUnk460(c: &mut Ctx, mut p_objId_1: i32, mut p_out_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetCamUnk460", &PROBE_N, &[(p_objId_1) as u32, (p_out_2) as u32], false); }
    let mut l_chr_3: u32 = 0;
    let r_ = 'f: {
    l_chr_3 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_3) != (((0i32) as u32))) as i32)) != 0) {
        F::Vec4_Copy(c, p_out_2, (l_chr_3).wrapping_add(1120));
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetCamUnk460", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_GetMgrUnk134(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetMgrUnk134", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f F::BtlReplay_GetViewSide(c);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetMgrUnk134", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_AnyCamPriority(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_AnyCamPriority", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    if (((((c.m.r32(sym::gBtlChars)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < ({ let a_ = c.m.r32(sym::gBtlChars); (c.m.r32(a_) as i32) })) as i32)) != 0) { break 'l1; }
            'c2: {
                l_chr_2 = F::BtlChar_Get(c, l_i_1);
                if ((((((F::ChrCam_IsCutActive(c, l_chr_2)) != 0) && ((F::BtlChar_TestFlag(c, l_chr_2, ((((211i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) {
                    break 'f 1i32;
                }
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_AnyCamPriority", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_IsInputInjected(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_IsInputInjected", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f (c.m.r32((l_chr_2).wrapping_add(4728)) as i32);
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_IsInputInjected", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_SetInjectedInput(c: &mut Ctx, mut p_objId_1: i32, mut p_buttons_2: u32, mut p_stickX_3: F32, mut p_stickY_4: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_SetInjectedInput", &PROBE_N, &[(p_objId_1) as u32, (p_buttons_2) as u32, (p_stickX_3).0, (p_stickY_4).0], false); }
    let mut l_chr_5: u32 = 0;
    let r_ = 'f: {
    l_chr_5 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_5) != (((0i32) as u32))) as i32)) != 0) {
        { let v_ = p_buttons_2; let a_ = (l_chr_5).wrapping_add(4732); c.m.w32(a_, v_); };
        { let v_ = p_stickX_3; let a_ = (l_chr_5).wrapping_add(4736); c.m.w32(a_, (v_).0); };
        { let v_ = p_stickY_4; let a_ = (l_chr_5).wrapping_add(4740); c.m.w32(a_, (v_).0); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_SetInjectedInput", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_TestFlag0F(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_TestFlag0F", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f F::BtlChar_TestFlag(c, l_chr_2, ((((15i32) as i64 as u64)) as u32));
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_TestFlag0F", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_TestFlag05(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_TestFlag05", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f F::BtlChar_TestFlag(c, l_chr_2, ((((5i32) as i64 as u64)) as u32));
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_TestFlag05", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_GetParamByte84(c: &mut Ctx, mut p_objId_1: i32, mut p_n_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetParamByte84", &PROBE_N, &[(p_objId_1) as u32, (p_n_2) as u32], false); }
    let mut l_chr_3: u32 = 0;
    let r_ = 'f: {
    l_chr_3 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_3) != (((0i32) as u32))) as i32)) != 0) {
        break 'f F::BtlParam_GetUnk84(c, l_chr_3, p_n_2);
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetParamByte84", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_GetParamByte8A(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetParamByte8A", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f F::BtlParam_GetUnk8A(c, l_chr_2, ((0i32) as u32));
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetParamByte8A", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_GetParamByte8D(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetParamByte8D", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f F::BtlParam_GetUnk8A(c, l_chr_2, ((3i32) as u32));
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetParamByte8D", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_GetUnk1288(c: &mut Ctx, mut p_objId_1: i32) -> u64 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetUnk1288", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f c.m.r64((l_chr_2).wrapping_add(4744));
    }
    break 'f ((0i32) as u64);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_GetUnk1288", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_CheckUnkCAC(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_CheckUnkCAC", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let mut l_obj_3: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    l_obj_3 = F::BtlChar_GetObj(c, l_chr_2);
    if (((((l_obj_3) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((((((c.m.r8((l_obj_3).wrapping_add(3245)) as i8)) as i32)) != ((!((((c.m.r8((l_obj_3).wrapping_add(3244)) as i8)) as i32))))) as i32)) != 0) {
        break 'f 1i32;
    }
    if (((((F::BtlChar_TestFlag(c, l_chr_2, ((((96i32) as i64 as u64)) as u32))) == (1i32)) as i32)) != 0) {
        break 'f 0i32;
    }
    break 'f ((((((c.m.r8((l_obj_3).wrapping_add(3245)) as i8)) as i32)) >= (0i32)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_CheckUnkCAC", &PROBE_N, &[], true); }
    r_
}

pub fn BtlCharApi_TestFlag60(c: &mut Ctx, mut p_objId_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_TestFlag60", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_chr_2: u32 = 0;
    let r_ = 'f: {
    l_chr_2 = F::BtlChar_FindByObjId(c, p_objId_1);
    if (((((l_chr_2) != (((0i32) as u32))) as i32)) != 0) {
        break 'f F::BtlChar_TestFlag(c, l_chr_2, ((((96i32) as i64 as u64)) as u32));
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlCharApi_TestFlag60", &PROBE_N, &[], true); }
    r_
}
