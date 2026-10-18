#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn BtlUtil_WrapAngle(c: &mut Ctx, mut p_a_1: F32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlUtil_WrapAngle", &PROBE_N, &[(p_a_1).0], false); }
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
    if rt::probing() { crate::host::harness::probe(c, "BtlUtil_WrapAngle", &PROBE_N, &[], true); }
    r_
}

pub fn BtlUtil_WrapAngles(c: &mut Ctx, mut p_dst_1: u32, mut p_src_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlUtil_WrapAngles", &PROBE_N, &[(p_dst_1) as u32, (p_src_2) as u32], false); }
    let r_ = 'f: {
    { let v_ = { let a0_ = F32(c.m.r32(p_src_2)); F::BtlUtil_WrapAngle(c, a0_) }; let a_ = p_dst_1; c.m.w32(a_, (v_).0); };
    { let v_ = { let a0_ = F32(c.m.r32((p_src_2).wrapping_add(4))); F::BtlUtil_WrapAngle(c, a0_) }; let a_ = (p_dst_1).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = { let a0_ = F32(c.m.r32((p_src_2).wrapping_add(8))); F::BtlUtil_WrapAngle(c, a0_) }; let a_ = (p_dst_1).wrapping_add(8); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((p_src_2).wrapping_add(12))); let a_ = (p_dst_1).wrapping_add(12); c.m.w32(a_, (v_).0); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlUtil_WrapAngles", &PROBE_N, &[], true); }
    r_
}

pub fn BtlUtil_LengthXZ(c: &mut Ctx, mut p_v_1: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlUtil_LengthXZ", &PROBE_N, &[(p_v_1) as u32], false); }
    let r_ = 'f: {
    break 'f { let a0_ = rt::fadd(rt::fmul(F32(c.m.r32(p_v_1)), F32(c.m.r32(p_v_1))), rt::fmul(F32(c.m.r32((p_v_1).wrapping_add(8))), F32(c.m.r32((p_v_1).wrapping_add(8))))); F::Port_sqrtf(c, a0_) };
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlUtil_LengthXZ", &PROBE_N, &[], true); }
    r_
}

pub fn BtlUtil_ApproachF(c: &mut Ctx, mut p_cur_1: F32, mut p_target_2: F32, mut p_step_3: F32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlUtil_ApproachF", &PROBE_N, &[(p_cur_1).0, (p_target_2).0, (p_step_3).0], false); }
    let r_ = 'f: {
    p_step_3 = rt::ffabs(p_step_3);
    if (((rt::flt(rt::fadd(p_cur_1, p_step_3), p_target_2) as i32)) != 0) {
        { let r_ = p_step_3; let n_ = rt::fadd(p_cur_1, r_); p_cur_1 = n_; };
    } else {
        { let r_ = p_step_3; let n_ = rt::fsub(p_cur_1, r_); p_cur_1 = n_; };
        if ((((!(((rt::flt(p_target_2, p_cur_1) as i32)) != 0)) as i32)) != 0) {
            p_cur_1 = p_target_2;
        }
    }
    break 'f p_cur_1;
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlUtil_ApproachF", &PROBE_N, &[], true); }
    r_
}

pub fn BtlUtil_Approach(c: &mut Ctx, mut p_cur_1: i32, mut p_target_2: i32, mut p_step_3: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlUtil_Approach", &PROBE_N, &[(p_cur_1) as u32, (p_target_2) as u32, (p_step_3) as u32], false); }
    let r_ = 'f: {
    p_step_3 = (p_step_3).wrapping_abs();
    if ((((((p_cur_1).wrapping_add(p_step_3)) < (p_target_2)) as i32)) != 0) {
        { let r_ = p_step_3; let n_ = (p_cur_1).wrapping_add(r_); p_cur_1 = n_; };
    } else {
        if (((((p_target_2) < ((p_cur_1).wrapping_sub(p_step_3))) as i32)) != 0) {
            { let r_ = p_step_3; let n_ = (p_cur_1).wrapping_sub(r_); p_cur_1 = n_; };
        } else {
            p_cur_1 = p_target_2;
        }
    }
    break 'f p_cur_1;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlUtil_Approach", &PROBE_N, &[], true); }
    r_
}

pub fn BtlUtil_ClampF(c: &mut Ctx, mut p_v_1: F32, mut p_lo_2: F32, mut p_hi_3: F32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlUtil_ClampF", &PROBE_N, &[(p_v_1).0, (p_lo_2).0, (p_hi_3).0], false); }
    let r_ = 'f: {
    if (((rt::flt(p_v_1, p_lo_2) as i32)) != 0) {
        p_v_1 = p_lo_2;
    }
    if (((rt::flt(p_hi_3, p_v_1) as i32)) != 0) {
        p_v_1 = p_hi_3;
    }
    break 'f p_v_1;
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlUtil_ClampF", &PROBE_N, &[], true); }
    r_
}

pub fn BtlUtil_MinF(c: &mut Ctx, mut p_a_1: F32, mut p_b_2: F32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlUtil_MinF", &PROBE_N, &[(p_a_1).0, (p_b_2).0], false); }
    let r_ = 'f: {
    if (((rt::flt(p_b_2, p_a_1) as i32)) != 0) {
        p_a_1 = p_b_2;
    }
    break 'f p_a_1;
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlUtil_MinF", &PROBE_N, &[], true); }
    r_
}

pub fn BtlUtil_MaxF(c: &mut Ctx, mut p_a_1: F32, mut p_b_2: F32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlUtil_MaxF", &PROBE_N, &[(p_a_1).0, (p_b_2).0], false); }
    let r_ = 'f: {
    if (((rt::flt(p_a_1, p_b_2) as i32)) != 0) {
        p_a_1 = p_b_2;
    }
    break 'f p_a_1;
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlUtil_MaxF", &PROBE_N, &[], true); }
    r_
}

pub fn BtlUtil_Clamp(c: &mut Ctx, mut p_v_1: i32, mut p_lo_2: i32, mut p_hi_3: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlUtil_Clamp", &PROBE_N, &[(p_v_1) as u32, (p_lo_2) as u32, (p_hi_3) as u32], false); }
    let r_ = 'f: {
    if (((((p_v_1) < (p_lo_2)) as i32)) != 0) {
        p_v_1 = p_lo_2;
    }
    if (((((p_hi_3) < (p_v_1)) as i32)) != 0) {
        p_v_1 = p_hi_3;
    }
    break 'f p_v_1;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlUtil_Clamp", &PROBE_N, &[], true); }
    r_
}

pub fn BtlUtil_Min(c: &mut Ctx, mut p_a_1: i32, mut p_b_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlUtil_Min", &PROBE_N, &[(p_a_1) as u32, (p_b_2) as u32], false); }
    let r_ = 'f: {
    break 'f (if (((((p_b_2) < (p_a_1)) as i32)) != 0) { p_b_2 } else { p_a_1 });
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlUtil_Min", &PROBE_N, &[], true); }
    r_
}

pub fn BtlUtil_Max(c: &mut Ctx, mut p_a_1: i32, mut p_b_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlUtil_Max", &PROBE_N, &[(p_a_1) as u32, (p_b_2) as u32], false); }
    let r_ = 'f: {
    break 'f (if (((((p_a_1) < (p_b_2)) as i32)) != 0) { p_b_2 } else { p_a_1 });
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlUtil_Max", &PROBE_N, &[], true); }
    r_
}

pub fn BtlUtil_RoundUp(c: &mut Ctx, mut p_n_1: i32, mut p_unit_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlUtil_RoundUp", &PROBE_N, &[(p_n_1) as u32, (p_unit_2) as u32], false); }
    let mut l_r_3: i32 = 0;
    let r_ = 'f: {
    l_r_3 = rt::rem_i32(p_n_1, p_unit_2);
    if (((((l_r_3) != (0i32)) as i32)) != 0) {
        { let r_ = (p_unit_2).wrapping_sub(l_r_3); let n_ = (p_n_1).wrapping_add(r_); p_n_1 = n_; };
    }
    break 'f p_n_1;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlUtil_RoundUp", &PROBE_N, &[], true); }
    r_
}

pub fn BtlUtil_AngleToSector(c: &mut Ctx, mut p_a_1: F32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlUtil_AngleToSector", &PROBE_N, &[(p_a_1).0], false); }
    let r_ = 'f: {
    if (((rt::flt(p_a_1, F32(0xc016cbe3)) as i32)) != 0) {
        break 'f 1i32;
    }
    if (((rt::flt(p_a_1, F32(0xbf490fda)) as i32)) != 0) {
        break 'f 2i32;
    }
    if (((rt::flt(p_a_1, F32(0x3f490fda)) as i32)) != 0) {
        break 'f 0i32;
    }
    if (((rt::flt(p_a_1, F32(0x4016cbe3)) as i32)) != 0) {
        break 'f 3i32;
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlUtil_AngleToSector", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChar_GetCount(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_GetCount", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f { let a_ = c.m.r32(sym::gBtlChars); (c.m.r32(a_) as i32) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_GetCount", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChar_Get(c: &mut Ctx, mut p_i_1: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_Get", &PROBE_N, &[(p_i_1) as u32], false); }
    let r_ = 'f: {
    break 'f ({ let a_ = (c.m.r32(sym::gBtlChars)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((p_i_1) as u32)).wrapping_mul(5632));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_Get", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChar_FindByObjId(c: &mut Ctx, mut p_objId_1: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_FindByObjId", &PROBE_N, &[(p_objId_1) as u32], false); }
    let mut l_i_2: i32 = 0;
    let mut l_chr_3: u32 = 0;
    let r_ = 'f: {
    if (((((c.m.r32(sym::gBtlChars)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f ((0i32) as u32);
    }
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < (F::BtlChar_GetCount(c))) as i32)) != 0) { break 'l1; }
            'c2: {
                l_chr_3 = F::BtlChar_Get(c, l_i_2);
                if ((((((c.m.r32((l_chr_3).wrapping_add(12)) as i32)) == (p_objId_1)) as i32)) != 0) {
                    break 'f l_chr_3;
                }
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
    break 'f ((0i32) as u32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_FindByObjId", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChar_FindBySide(c: &mut Ctx, mut p_side_1: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_FindBySide", &PROBE_N, &[(p_side_1) as u32], false); }
    let mut l_i_2: i32 = 0;
    let mut l_chr_3: u32 = 0;
    let r_ = 'f: {
    if (((((c.m.r32(sym::gBtlChars)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f ((0i32) as u32);
    }
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < (F::BtlChar_GetCount(c))) as i32)) != 0) { break 'l1; }
            'c2: {
                l_chr_3 = F::BtlChar_Get(c, l_i_2);
                if ((((((c.m.r32((l_chr_3).wrapping_add(8)) as i32)) == (p_side_1)) as i32)) != 0) {
                    break 'f l_chr_3;
                }
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
    break 'f ((0i32) as u32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_FindBySide", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChar_GetObj(c: &mut Ctx, mut p_chr_1: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_GetObj", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f { let a0_ = (c.m.r32((p_chr_1).wrapping_add(12)) as i32); F::BtlObj_Get(c, a0_) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_GetObj", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChar_GetPos(c: &mut Ctx, mut p_chr_1: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_GetPos", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f (p_chr_1).wrapping_add(16);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_GetPos", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChar_GetPad(c: &mut Ctx, mut p_chr_1: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_GetPad", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f (sym::gPad).wrapping_add(((((c.m.r32((p_chr_1).wrapping_add(4)) as i32)) as u32)).wrapping_mul(448));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_GetPad", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChar_IsFrozen(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_IsFrozen", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f ((((c.m.r32((p_chr_1).wrapping_add(4896)) as i32)) > (0i32)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_IsFrozen", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChar_IsFree(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_IsFree", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    if ((((((F::BtlChar_IsDead(c, p_chr_1)) != 0) || ((F::BtlChar_TestFlag(c, p_chr_1, ((((190i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    break 'f ((((c.m.r32((p_chr_1).wrapping_add(4064)) as i32)) <= (0i32)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_IsFree", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChar_IsDead(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_IsDead", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f ((({ let a_ = F::BtlMember_GetActiveGauge(c, p_chr_1); (c.m.r32(a_) as i32) }) <= (0i32)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_IsDead", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChar_TestMemberUnk70(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_TestMemberUnk70", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f ((({ let a_ = (F::BtlMember_GetActiveGauge(c, p_chr_1)).wrapping_add(48); (c.m.r32(a_) as i32) }) != (0i32)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_TestMemberUnk70", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChar_GetFrame(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_GetFrame", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f { let a_ = (c.m.r32(sym::gBtlChars)).wrapping_add(20); (c.m.r32(a_) as i32) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_GetFrame", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChar_FrameMod(c: &mut Ctx, mut p_n_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_FrameMod", &PROBE_N, &[(p_n_1) as u32], false); }
    let r_ = 'f: {
    if (((((p_n_1) <= (0i32)) as i32)) != 0) {
        p_n_1 = 1i32;
    }
    break 'f rt::rem_i32({ let a_ = (c.m.r32(sym::gBtlChars)).wrapping_add(20); (c.m.r32(a_) as i32) }, p_n_1);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_FrameMod", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChar_Rand(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_Rand", &PROBE_N, &[], false); }
    let mut l_state_1: u64 = 0;
    let mut l_next_2: i32 = 0;
    let r_ = 'f: {
    l_state_1 = (({ let a_ = (c.m.r32(sym::gBtlChars)).wrapping_add(24); c.m.r32(a_) }) as u64);
    l_next_2 = ((rt::rem_u64(((l_state_1).wrapping_mul(((714025i32) as u64))).wrapping_add(((4096i32) as u64)), ((150889i32) as u64))) as i32);
    if ((((!((F::BtlChars_IsTimeStopped(c)) != 0)) as i32)) != 0) {
        { let v_ = ((l_next_2) as u32); let a_ = (c.m.r32(sym::gBtlChars)).wrapping_add(24); c.m.w32(a_, v_); };
    }
    break 'f l_next_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_Rand", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChar_RandF(c: &mut Ctx) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_RandF", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f rt::fdiv(rt::u2f(((F::BtlChar_Rand(c)) as u32)), F32(0x48135a40));
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_RandF", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChar_IsStage4Or27(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_IsStage4Or27", &PROBE_N, &[], false); }
    let mut l_stage_1: i32 = 0;
    let r_ = 'f: {
    l_stage_1 = F::Battle_GetStage(c);
    if (((((((((l_stage_1) == (4i32)) as i32)) != 0) || (((((l_stage_1) == (27i32)) as i32)) != 0)) as i32)) != 0) {
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_IsStage4Or27", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChar_AddStageTimer(c: &mut Ctx, mut p_seconds_1: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_AddStageTimer", &PROBE_N, &[(p_seconds_1).0], false); }
    let mut l_frames_2: i32 = 0;
    let r_ = 'f: {
    l_frames_2 = rt::f2i(rt::fmul(p_seconds_1, F32(0x41f00000)));
    if ((((((F::BtlChar_IsStage4Or27(c)) != 0) && (((((c.m.r32(sym::gBtlChars)) != (((0i32) as u32))) as i32)) != 0)) as i32)) != 0) {
        { let v_ = { let a0_ = ({ let a_ = (c.m.r32(sym::gBtlChars)).wrapping_add(28); (c.m.r32(a_) as i32) }).wrapping_add(l_frames_2); F::BtlUtil_Min(c, a0_, 150i32) }; let a_ = (c.m.r32(sym::gBtlChars)).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_AddStageTimer", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChar_SetVibration(c: &mut Ctx, mut p_chr_1: u32, mut p_power_2: F32, mut p_seconds_3: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_SetVibration", &PROBE_N, &[(p_chr_1) as u32, (p_power_2).0, (p_seconds_3).0], false); }
    let mut l_vib_4: u32 = 0;
    let r_ = 'f: {
    l_vib_4 = (p_chr_1).wrapping_add(5584);
    if (((rt::fgt(p_power_2, F32(0x3f800000)) as i32)) != 0) {
        p_power_2 = F32(0x3f800000);
    }
    if ((((((((((c.m.r32((l_vib_4).wrapping_add(8)) as i32)) <= (0i32)) as i32)) != 0) || (((rt::flt(F32(c.m.r32((l_vib_4).wrapping_add(4))), p_power_2) as i32)) != 0)) as i32)) != 0) {
        { let v_ = p_power_2; let a_ = (l_vib_4).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::f2i(rt::fmul(p_seconds_3, F32(0x41f00000))); let a_ = (l_vib_4).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_SetVibration", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChar_SetSmallVibration(c: &mut Ctx, mut p_chr_1: u32, mut p_seconds_2: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_SetSmallVibration", &PROBE_N, &[(p_chr_1) as u32, (p_seconds_2).0], false); }
    let mut l_vib_3: u32 = 0;
    let mut l_frames_4: i32 = 0;
    let r_ = 'f: {
    l_vib_3 = (p_chr_1).wrapping_add(5584);
    l_frames_4 = rt::f2i(rt::fmul(p_seconds_2, F32(0x41f00000)));
    if ((((((c.m.r32((l_vib_3).wrapping_add(12)) as i32)) < (l_frames_4)) as i32)) != 0) {
        { let v_ = l_frames_4; let a_ = (l_vib_3).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_SetSmallVibration", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChar_Vibrate(c: &mut Ctx, mut p_chr_1: u32, mut p_power_2: F32, mut p_seconds_3: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_Vibrate", &PROBE_N, &[(p_chr_1) as u32, (p_power_2).0, (p_seconds_3).0], false); }
    let r_ = 'f: {
    F::BtlChar_SetVibration(c, p_chr_1, p_power_2, p_seconds_3);
    F::BtlChar_SetSmallVibration(c, p_chr_1, p_seconds_3);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_Vibrate", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChar_UpdateVibration(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_UpdateVibration", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_vib_2: u32 = 0;
    let mut l_pad_3: i32 = 0;
    let mut l_small_4: i32 = 0;
    let mut l_power_5: F32 = F32(0);
    let mut l_t_6: i32 = 0;
    let mut l_st_7: i32 = 0;
    let mut l_half_8: F32 = F32(0);
    let r_ = 'f: {
    l_vib_2 = (p_chr_1).wrapping_add(5584);
    if ((((((((((((!(((c.m.r32(l_vib_2) as i32)) != 0)) as i32)) != 0) || ((F::BattleReplay_IsActive(c)) != 0)) as i32)) != 0) || (((c.m.r32((p_chr_1).wrapping_add(4728)) as i32)) != 0)) as i32)) != 0) {
        { let v_ = 0i32; let a_ = (l_vib_2).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (l_vib_2).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
    } else {
        l_pad_3 = (c.m.r32((p_chr_1).wrapping_add(4)) as i32);
        l_small_4 = 0i32;
        l_power_5 = F32(0x00000000);
        if ((((((c.m.r32((l_vib_2).wrapping_add(8)) as i32)) > (0i32)) as i32)) != 0) {
            l_half_8 = F32(0x3f000000);
            l_power_5 = rt::fmul(F32(c.m.r32((l_vib_2).wrapping_add(4))), rt::fadd(rt::fmul(rt::fadd(rt::fmul({ let a0_ = rt::fdiv(rt::fmul(rt::i2f((c.m.r32((l_vib_2).wrapping_add(16)) as i32)), F32(0x4216cbe3)), F32(0x41f00000)); F::Port_sinf(c, a0_) }, l_half_8), l_half_8), l_half_8), l_half_8));
            { let a_ = (l_vib_2).wrapping_add(16); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        } else {
            { let v_ = 0i32; let a_ = (l_vib_2).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
        }
        if ((((((c.m.r32((l_vib_2).wrapping_add(12)) as i32)) > (0i32)) as i32)) != 0) {
            l_small_4 = (((rt::rem_u32((((c.m.r32((l_vib_2).wrapping_add(20)) as i32)) as u32), ((2i32) as u32))) == (((0i32) as u32))) as i32);
            { let a_ = (l_vib_2).wrapping_add(20); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        } else {
            { let v_ = 0i32; let a_ = (l_vib_2).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
        }
        F::Pad_AddVibration(c, l_pad_3, l_small_4, l_power_5);
        l_t_6 = ((c.m.r32((l_vib_2).wrapping_add(8)) as i32)).wrapping_sub(1i32);
        l_st_7 = ((c.m.r32((l_vib_2).wrapping_add(12)) as i32)).wrapping_sub(1i32);
        if (((((l_t_6) < (0i32)) as i32)) != 0) {
            l_t_6 = 0i32;
        }
        if (((((l_st_7) < (0i32)) as i32)) != 0) {
            l_st_7 = 0i32;
        }
        { let v_ = l_t_6; let a_ = (l_vib_2).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = l_st_7; let a_ = (l_vib_2).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_UpdateVibration", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChar_GetVoiceEntry(c: &mut Ctx, mut p_kind_1: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_GetVoiceEntry", &PROBE_N, &[(p_kind_1) as u32], false); }
    let r_ = 'f: {
    break 'f ({ let a_ = (c.m.r32(sym::gBtlChars)).wrapping_add(40); c.m.r32(a_) }).wrapping_add((((p_kind_1) as u32)).wrapping_mul(8));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_GetVoiceEntry", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChar_PlayVoice(c: &mut Ctx, mut p_chr_1: u32, mut p_kind_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_PlayVoice", &PROBE_N, &[(p_chr_1) as u32, (p_kind_2) as u32], false); }
    let mut l_obj_3: u32 = 0;
    let mut l_ent_4: u32 = 0;
    let mut l_line_5: i32 = 0;
    let r_ = 'f: {
    l_obj_3 = F::BtlChar_GetObj(c, p_chr_1);
    if ((F::BtlChar_IsDead(c, p_chr_1)) != 0) {
        break 'f;
    }
    if ((((((c.m.r32((((p_chr_1).wrapping_add(4976)).wrapping_add(228)).wrapping_add((((p_kind_2) as u32)).wrapping_mul(4))) as i32)) > (0i32)) as i32)) != 0) {
        break 'f;
    }
    if (((((((((F::Battle_GetMode(c)) == (1i32)) as i32)) != 0) && (((((p_kind_2) >= (14i32)) as i32)) != 0)) as i32)) != 0) {
        if (((((p_kind_2) < (17i32)) as i32)) != 0) {
            break 'f;
        }
        if (((((p_kind_2) == (34i32)) as i32)) != 0) {
            break 'f;
        }
    }
    l_ent_4 = F::BtlChar_GetVoiceEntry(c, p_kind_2);
    if ((((((((c.m.r16((l_ent_4).wrapping_add(2)) as i16)) as i32)) >= (2i32)) as i32)) != 0) {
        l_line_5 = ((((c.m.r16(l_ent_4) as i16)) as i32)).wrapping_add({ let a0_ = ((((c.m.r16((l_ent_4).wrapping_add(2)) as i16)) as i32)).wrapping_sub(1i32); F::BtlChar_FrameMod(c, a0_) });
        if (((((l_line_5) >= ((c.m.r32(((p_chr_1).wrapping_add(4976)).wrapping_add((((p_kind_2) as u32)).wrapping_mul(4))) as i32))) as i32)) != 0) {
            { let o_ = l_line_5; let n_ = (o_).wrapping_add(1); l_line_5 = n_; };
        }
    } else {
        l_line_5 = (((c.m.r16(l_ent_4) as i16)) as i32);
    }
    if (((((p_kind_2) < (15i32)) as i32)) != 0) {
        F::BtlCharSnd_PlayOwn(c, p_chr_1, l_line_5);
        F::BtlObj_SetSubState(c, l_obj_3, 4i32, 0i32);
    } else {
        F::BtlCharSnd_PlayVoice(c, p_chr_1, l_line_5);
        F::BtlObj_SetSubState(c, l_obj_3, 2i32, l_line_5);
    }
    { let v_ = l_line_5; let a_ = ((p_chr_1).wrapping_add(4976)).wrapping_add((((p_kind_2) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    { let v_ = rt::f2i(rt::fmul(F32(c.m.r32((l_ent_4).wrapping_add(4))), F32(0x41f00000))); let a_ = (((p_chr_1).wrapping_add(4976)).wrapping_add(228)).wrapping_add((((p_kind_2) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_PlayVoice", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChar_StopVoiceOnFlag(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_StopVoiceOnFlag", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    if (((((F::Battle_GetMode(c)) == (1i32)) as i32)) != 0) {
        if ((F::BtlScript_IsTextShown(c)) != 0) {
            break 'f;
        }
    }
    if ((((((F::BtlChar_TestFlag(c, p_chr_1, ((((297i32) as i64 as u64)) as u32))) != 0) && ((((!(({ let a0_ = (c.m.r32(p_chr_1) as i32); F::Voice_IsStopped(c, a0_) }) != 0)) as i32)) != 0)) as i32)) != 0) {
        { let a0_ = (c.m.r32(p_chr_1) as i32); F::Voice_Stop(c, a0_) };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_StopVoiceOnFlag", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChar_TickVoiceTimers(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_TickVoiceTimers", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < (57i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((((((c.m.r32((((p_chr_1).wrapping_add(4976)).wrapping_add(228)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4))) as i32)) > (0i32)) as i32)) != 0) {
                    { let a_ = (((p_chr_1).wrapping_add(4976)).wrapping_add(228)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                }
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_TickVoiceTimers", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChar_PlayVoiceSingle(c: &mut Ctx, mut p_chr_1: u32, mut p_kind_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_PlayVoiceSingle", &PROBE_N, &[(p_chr_1) as u32, (p_kind_2) as u32], false); }
    let mut l_ent_3: u32 = 0;
    let r_ = 'f: {
    l_ent_3 = F::BtlChar_GetVoiceEntry(c, p_kind_2);
    if ((((((((c.m.r16((l_ent_3).wrapping_add(2)) as i16)) as i32)) < (2i32)) as i32)) != 0) {
        { let a1_ = (((c.m.r16(l_ent_3) as i16)) as i32); F::BtlCharSnd_StopOwn(c, p_chr_1, a1_) };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_PlayVoiceSingle", &PROBE_N, &[], true); }
    r_
}
