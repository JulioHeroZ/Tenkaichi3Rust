#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn BtlStat_ClearSlotOn(c: &mut Ctx, mut p_chr_1: u32, mut p_slot_2: i32, mut p_kind_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlStat_ClearSlotOn", &PROBE_N, &[(p_chr_1) as u32, (p_slot_2) as u32, (p_kind_3) as u32], false); }
    let mut l_i_4: i32 = 0;
    let r_ = 'f: {
    l_i_4 = (((p_slot_2) != (0i32)) as i32);
    if (((((F::BtlSkill_GetStatKind(c, p_chr_1, p_slot_2)) == (p_kind_3)) as i32)) != 0) {
        { let v_ = 0i32; let a_ = ((p_chr_1).wrapping_add(3640)).wrapping_add((((l_i_4) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlStat_ClearSlotOn", &PROBE_N, &[], true); }
    r_
}

pub fn BtlStat_EndKind2(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlStat_EndKind2", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < (4i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let v_ = ((0i32) as i8); let a_ = ((((p_chr_1).wrapping_add(3920)).wrapping_add((((l_i_2) as u32)).wrapping_mul(24))).wrapping_add(4)).wrapping_add(((0i32) as u32)); c.m.w8(a_, (v_) as u8); };
                { let v_ = ((0i32) as i8); let a_ = ((((p_chr_1).wrapping_add(3920)).wrapping_add((((l_i_2) as u32)).wrapping_mul(24))).wrapping_add(4)).wrapping_add(((1i32) as u32)); c.m.w8(a_, (v_) as u8); };
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
    { let v_ = 0i32; let a_ = (p_chr_1).wrapping_add(3632); c.m.w32(a_, (v_) as u32); };
    F::BtlStat_ClearSlotOn(c, p_chr_1, 0i32, 2i32);
    F::BtlStat_ClearSlotOn(c, p_chr_1, 1i32, 2i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlStat_EndKind2", &PROBE_N, &[], true); }
    r_
}

pub fn BtlStat_EndKind3(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlStat_EndKind3", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < (4i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let v_ = ((0i32) as i8); let a_ = ((((p_chr_1).wrapping_add(3920)).wrapping_add((((l_i_2) as u32)).wrapping_mul(24))).wrapping_add(6)).wrapping_add(((0i32) as u32)); c.m.w8(a_, (v_) as u8); };
                { let v_ = ((0i32) as i8); let a_ = ((((p_chr_1).wrapping_add(3920)).wrapping_add((((l_i_2) as u32)).wrapping_mul(24))).wrapping_add(6)).wrapping_add(((1i32) as u32)); c.m.w8(a_, (v_) as u8); };
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
    { let v_ = 0i32; let a_ = (p_chr_1).wrapping_add(3636); c.m.w32(a_, (v_) as u32); };
    F::BtlStat_ClearSlotOn(c, p_chr_1, 0i32, 3i32);
    F::BtlStat_ClearSlotOn(c, p_chr_1, 1i32, 3i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlStat_EndKind3", &PROBE_N, &[], true); }
    r_
}

pub fn BtlStat_EndKind4(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlStat_EndKind4", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < (4i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let v_ = ((0i32) as i8); let a_ = ((((p_chr_1).wrapping_add(3920)).wrapping_add((((l_i_2) as u32)).wrapping_mul(24))).wrapping_add(8)).wrapping_add(((0i32) as u32)); c.m.w8(a_, (v_) as u8); };
                { let v_ = ((0i32) as i8); let a_ = ((((p_chr_1).wrapping_add(3920)).wrapping_add((((l_i_2) as u32)).wrapping_mul(24))).wrapping_add(8)).wrapping_add(((1i32) as u32)); c.m.w8(a_, (v_) as u8); };
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
    F::BtlStat_ClearSlotOn(c, p_chr_1, 0i32, 4i32);
    F::BtlStat_ClearSlotOn(c, p_chr_1, 1i32, 4i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlStat_EndKind4", &PROBE_N, &[], true); }
    r_
}

pub fn BtlStat_ClearPenalty(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlStat_ClearPenalty", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < (4i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let v_ = ((0i32) as i8); let a_ = ((((p_chr_1).wrapping_add(3920)).wrapping_add((((l_i_2) as u32)).wrapping_mul(24))).wrapping_add(10)).wrapping_add(((0i32) as u32)); c.m.w8(a_, (v_) as u8); };
                { let v_ = ((0i32) as i8); let a_ = ((((p_chr_1).wrapping_add(3920)).wrapping_add((((l_i_2) as u32)).wrapping_mul(24))).wrapping_add(10)).wrapping_add(((1i32) as u32)); c.m.w8(a_, (v_) as u8); };
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlStat_ClearPenalty", &PROBE_N, &[], true); }
    r_
}

pub fn BtlStat_ClearFrameMods(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlStat_ClearFrameMods", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
    {
        l_i_2 = (4i32).wrapping_sub(1i32);
        'l1: loop {
            if !(((((l_i_2) >= (0i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let v_ = ((0i32) as i8); let a_ = (((p_chr_1).wrapping_add(3920)).wrapping_add((((l_i_2) as u32)).wrapping_mul(24))).wrapping_add(12); c.m.w8(a_, (v_) as u8); };
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_sub(1); l_i_2 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlStat_ClearFrameMods", &PROBE_N, &[], true); }
    r_
}

pub fn BtlStat_Reset(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlStat_Reset", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_i_2: i32 = 0;
    let mut l_j_3: i32 = 0;
    let r_ = 'f: {
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < (4i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                {
                    l_j_3 = 0i32;
                    'l3: loop {
                        if !(((((l_j_3) < (2i32)) as i32)) != 0) { break 'l3; }
                        'c4: {
                            { let v_ = ((0i32) as i8); let a_ = (((p_chr_1).wrapping_add(3920)).wrapping_add((((l_i_2) as u32)).wrapping_mul(24))).wrapping_add(((l_j_3) as u32)); c.m.w8(a_, (v_) as u8); };
                            { let v_ = ((0i32) as i8); let a_ = ((((p_chr_1).wrapping_add(3920)).wrapping_add((((l_i_2) as u32)).wrapping_mul(24))).wrapping_add(2)).wrapping_add(((l_j_3) as u32)); c.m.w8(a_, (v_) as u8); };
                            { let v_ = ((0i32) as i8); let a_ = ((((p_chr_1).wrapping_add(3920)).wrapping_add((((l_i_2) as u32)).wrapping_mul(24))).wrapping_add(4)).wrapping_add(((l_j_3) as u32)); c.m.w8(a_, (v_) as u8); };
                            { let v_ = ((0i32) as i8); let a_ = ((((p_chr_1).wrapping_add(3920)).wrapping_add((((l_i_2) as u32)).wrapping_mul(24))).wrapping_add(6)).wrapping_add(((l_j_3) as u32)); c.m.w8(a_, (v_) as u8); };
                            { let v_ = ((0i32) as i8); let a_ = ((((p_chr_1).wrapping_add(3920)).wrapping_add((((l_i_2) as u32)).wrapping_mul(24))).wrapping_add(8)).wrapping_add(((l_j_3) as u32)); c.m.w8(a_, (v_) as u8); };
                            { let v_ = ((0i32) as i8); let a_ = ((((p_chr_1).wrapping_add(3920)).wrapping_add((((l_i_2) as u32)).wrapping_mul(24))).wrapping_add(10)).wrapping_add(((l_j_3) as u32)); c.m.w8(a_, (v_) as u8); };
                            { let v_ = 0i32; let a_ = ((((p_chr_1).wrapping_add(3920)).wrapping_add((((l_i_2) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add((((l_j_3) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                        }
                        { let o_ = l_j_3; let n_ = (o_).wrapping_add(1); l_j_3 = n_; };
                    }
                }
                { let v_ = ((0i32) as i8); let a_ = (((p_chr_1).wrapping_add(3920)).wrapping_add((((l_i_2) as u32)).wrapping_mul(24))).wrapping_add(12); c.m.w8(a_, (v_) as u8); };
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlStat_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn BtlStat_Update(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlStat_Update", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_i_2: i32 = 0;
    let mut l_j_3: i32 = 0;
    let r_ = 'f: {
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < (4i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                {
                    l_j_3 = 0i32;
                    'l3: loop {
                        if !(((((l_j_3) < (2i32)) as i32)) != 0) { break 'l3; }
                        'c4: {
                            { let a_ = ((((p_chr_1).wrapping_add(3920)).wrapping_add((((l_i_2) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add((((l_j_3) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                            if ((((((c.m.r32(((((p_chr_1).wrapping_add(3920)).wrapping_add((((l_i_2) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add((((l_j_3) as u32)).wrapping_mul(4))) as i32)) <= (0i32)) as i32)) != 0) {
                                { let v_ = 0i32; let a_ = ((((p_chr_1).wrapping_add(3920)).wrapping_add((((l_i_2) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add((((l_j_3) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                { let v_ = ((0i32) as i8); let a_ = ((((p_chr_1).wrapping_add(3920)).wrapping_add((((l_i_2) as u32)).wrapping_mul(24))).wrapping_add(2)).wrapping_add(((l_j_3) as u32)); c.m.w8(a_, (v_) as u8); };
                                F::BtlStat_ClearSlotOn(c, p_chr_1, l_j_3, 1i32);
                            }
                        }
                        { let o_ = l_j_3; let n_ = (o_).wrapping_add(1); l_j_3 = n_; };
                    }
                }
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
    if ((((((((((((!(({ let a0_ = F::BtlAct_GetCurrent(c, p_chr_1); F::BtlAct_IsTechniqueId(c, a0_) }) != 0)) as i32)) != 0) && (({ let a0_ = F::BtlAct_GetPrev(c, p_chr_1); F::BtlAct_IsTechniqueId(c, a0_) }) != 0)) as i32)) != 0) && ((((!((F::BtlChar_TestPrevFlag(c, p_chr_1, ((((164i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0)) as i32)) != 0) {
        F::BtlStat_EndKind2(c, p_chr_1);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlStat_Update", &PROBE_N, &[], true); }
    r_
}

pub fn BtlStat_SetBase(c: &mut Ctx, mut p_chr_1: u32, mut p_stat_2: i32, mut p_slot_3: i32, mut p_val_4: i32, mut p_add_5: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlStat_SetBase", &PROBE_N, &[(p_chr_1) as u32, (p_stat_2) as u32, (p_slot_3) as u32, (p_val_4) as u32, (p_add_5) as u32], false); }
    let r_ = 'f: {
    if ((p_add_5) != 0) {
        { let r_ = p_val_4; let a_ = (((p_chr_1).wrapping_add(3920)).wrapping_add((((p_stat_2) as u32)).wrapping_mul(24))).wrapping_add(((p_slot_3) as u32)); let n_ = ((((((c.m.r8(a_) as i8)) as i32)).wrapping_add(r_)) as i8); c.m.w8(a_, (n_) as u8); };
    } else {
        { let v_ = ((p_val_4) as i8); let a_ = (((p_chr_1).wrapping_add(3920)).wrapping_add((((p_stat_2) as u32)).wrapping_mul(24))).wrapping_add(((p_slot_3) as u32)); c.m.w8(a_, (v_) as u8); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlStat_SetBase", &PROBE_N, &[], true); }
    r_
}

pub fn BtlStat_SetFrameMod(c: &mut Ctx, mut p_chr_1: u32, mut p_stat_2: i32, mut p_val_3: i32, mut p_add_4: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlStat_SetFrameMod", &PROBE_N, &[(p_chr_1) as u32, (p_stat_2) as u32, (p_val_3) as u32, (p_add_4) as u32], false); }
    let r_ = 'f: {
    if ((p_add_4) != 0) {
        { let r_ = p_val_3; let a_ = (((p_chr_1).wrapping_add(3920)).wrapping_add((((p_stat_2) as u32)).wrapping_mul(24))).wrapping_add(12); let n_ = ((((((c.m.r8(a_) as i8)) as i32)).wrapping_add(r_)) as i8); c.m.w8(a_, (n_) as u8); };
    } else {
        { let v_ = ((p_val_3) as i8); let a_ = (((p_chr_1).wrapping_add(3920)).wrapping_add((((p_stat_2) as u32)).wrapping_mul(24))).wrapping_add(12); c.m.w8(a_, (v_) as u8); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlStat_SetFrameMod", &PROBE_N, &[], true); }
    r_
}

pub fn BtlStat_SetTimed(c: &mut Ctx, mut p_chr_1: u32, mut p_stat_2: i32, mut p_slot_3: i32, mut p_val_4: i32, mut p_add_5: i32, mut p_time_6: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlStat_SetTimed", &PROBE_N, &[(p_chr_1) as u32, (p_stat_2) as u32, (p_slot_3) as u32, (p_val_4) as u32, (p_add_5) as u32, (p_time_6) as u32], false); }
    let r_ = 'f: {
    if ((p_add_5) != 0) {
        { let r_ = p_val_4; let a_ = ((((p_chr_1).wrapping_add(3920)).wrapping_add((((p_stat_2) as u32)).wrapping_mul(24))).wrapping_add(2)).wrapping_add(((p_slot_3) as u32)); let n_ = ((((((c.m.r8(a_) as i8)) as i32)).wrapping_add(r_)) as i8); c.m.w8(a_, (n_) as u8); };
    } else {
        { let v_ = ((p_val_4) as i8); let a_ = ((((p_chr_1).wrapping_add(3920)).wrapping_add((((p_stat_2) as u32)).wrapping_mul(24))).wrapping_add(2)).wrapping_add(((p_slot_3) as u32)); c.m.w8(a_, (v_) as u8); };
    }
    { let v_ = p_time_6; let a_ = ((((p_chr_1).wrapping_add(3920)).wrapping_add((((p_stat_2) as u32)).wrapping_mul(24))).wrapping_add(16)).wrapping_add((((p_slot_3) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlStat_SetTimed", &PROBE_N, &[], true); }
    r_
}

pub fn BtlStat_SetKind2(c: &mut Ctx, mut p_chr_1: u32, mut p_stat_2: i32, mut p_slot_3: i32, mut p_val_4: i32, mut p_add_5: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlStat_SetKind2", &PROBE_N, &[(p_chr_1) as u32, (p_stat_2) as u32, (p_slot_3) as u32, (p_val_4) as u32, (p_add_5) as u32], false); }
    let r_ = 'f: {
    if ((p_add_5) != 0) {
        { let r_ = p_val_4; let a_ = ((((p_chr_1).wrapping_add(3920)).wrapping_add((((p_stat_2) as u32)).wrapping_mul(24))).wrapping_add(4)).wrapping_add(((p_slot_3) as u32)); let n_ = ((((((c.m.r8(a_) as i8)) as i32)).wrapping_add(r_)) as i8); c.m.w8(a_, (n_) as u8); };
    } else {
        { let v_ = ((p_val_4) as i8); let a_ = ((((p_chr_1).wrapping_add(3920)).wrapping_add((((p_stat_2) as u32)).wrapping_mul(24))).wrapping_add(4)).wrapping_add(((p_slot_3) as u32)); c.m.w8(a_, (v_) as u8); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlStat_SetKind2", &PROBE_N, &[], true); }
    r_
}

pub fn BtlStat_SetKind3(c: &mut Ctx, mut p_chr_1: u32, mut p_stat_2: i32, mut p_slot_3: i32, mut p_val_4: i32, mut p_add_5: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlStat_SetKind3", &PROBE_N, &[(p_chr_1) as u32, (p_stat_2) as u32, (p_slot_3) as u32, (p_val_4) as u32, (p_add_5) as u32], false); }
    let r_ = 'f: {
    if ((p_add_5) != 0) {
        { let r_ = p_val_4; let a_ = ((((p_chr_1).wrapping_add(3920)).wrapping_add((((p_stat_2) as u32)).wrapping_mul(24))).wrapping_add(6)).wrapping_add(((p_slot_3) as u32)); let n_ = ((((((c.m.r8(a_) as i8)) as i32)).wrapping_add(r_)) as i8); c.m.w8(a_, (n_) as u8); };
    } else {
        { let v_ = ((p_val_4) as i8); let a_ = ((((p_chr_1).wrapping_add(3920)).wrapping_add((((p_stat_2) as u32)).wrapping_mul(24))).wrapping_add(6)).wrapping_add(((p_slot_3) as u32)); c.m.w8(a_, (v_) as u8); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlStat_SetKind3", &PROBE_N, &[], true); }
    r_
}

pub fn BtlStat_SetKind4(c: &mut Ctx, mut p_chr_1: u32, mut p_stat_2: i32, mut p_slot_3: i32, mut p_val_4: i32, mut p_add_5: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlStat_SetKind4", &PROBE_N, &[(p_chr_1) as u32, (p_stat_2) as u32, (p_slot_3) as u32, (p_val_4) as u32, (p_add_5) as u32], false); }
    let r_ = 'f: {
    if ((p_add_5) != 0) {
        { let r_ = p_val_4; let a_ = ((((p_chr_1).wrapping_add(3920)).wrapping_add((((p_stat_2) as u32)).wrapping_mul(24))).wrapping_add(8)).wrapping_add(((p_slot_3) as u32)); let n_ = ((((((c.m.r8(a_) as i8)) as i32)).wrapping_add(r_)) as i8); c.m.w8(a_, (n_) as u8); };
    } else {
        { let v_ = ((p_val_4) as i8); let a_ = ((((p_chr_1).wrapping_add(3920)).wrapping_add((((p_stat_2) as u32)).wrapping_mul(24))).wrapping_add(8)).wrapping_add(((p_slot_3) as u32)); c.m.w8(a_, (v_) as u8); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlStat_SetKind4", &PROBE_N, &[], true); }
    r_
}

pub fn BtlStat_SetPenalty(c: &mut Ctx, mut p_chr_1: u32, mut p_stat_2: i32, mut p_slot_3: i32, mut p_val_4: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlStat_SetPenalty", &PROBE_N, &[(p_chr_1) as u32, (p_stat_2) as u32, (p_slot_3) as u32, (p_val_4) as u32], false); }
    let r_ = 'f: {
    { let v_ = ((p_val_4) as i8); let a_ = ((((p_chr_1).wrapping_add(3920)).wrapping_add((((p_stat_2) as u32)).wrapping_mul(24))).wrapping_add(10)).wrapping_add(((p_slot_3) as u32)); c.m.w8(a_, (v_) as u8); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlStat_SetPenalty", &PROBE_N, &[], true); }
    r_
}

pub fn BtlStat_GetMod(c: &mut Ctx, mut p_chr_1: u32, mut p_stat_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlStat_GetMod", &PROBE_N, &[(p_chr_1) as u32, (p_stat_2) as u32], false); }
    let mut l_m_3: u32 = 0;
    let mut l_i_4: i32 = 0;
    let mut l_sum_5: i32 = 0;
    let mut l_lost_6: i32 = 0;
    let r_ = 'f: {
    l_m_3 = ((p_chr_1).wrapping_add(3920)).wrapping_add((((p_stat_2) as u32)).wrapping_mul(24));
    l_sum_5 = 0i32;
    {
        l_i_4 = 0i32;
        'l1: loop {
            if !(((((l_i_4) < (2i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let r_ = (((c.m.r8((l_m_3).wrapping_add(((l_i_4) as u32))) as i8)) as i32); let n_ = (l_sum_5).wrapping_add(r_); l_sum_5 = n_; };
                { let r_ = (((c.m.r8(((l_m_3).wrapping_add(2)).wrapping_add(((l_i_4) as u32))) as i8)) as i32); let n_ = (l_sum_5).wrapping_add(r_); l_sum_5 = n_; };
                { let r_ = (((c.m.r8(((l_m_3).wrapping_add(4)).wrapping_add(((l_i_4) as u32))) as i8)) as i32); let n_ = (l_sum_5).wrapping_add(r_); l_sum_5 = n_; };
                { let r_ = (((c.m.r8(((l_m_3).wrapping_add(6)).wrapping_add(((l_i_4) as u32))) as i8)) as i32); let n_ = (l_sum_5).wrapping_add(r_); l_sum_5 = n_; };
                { let r_ = (((c.m.r8(((l_m_3).wrapping_add(8)).wrapping_add(((l_i_4) as u32))) as i8)) as i32); let n_ = (l_sum_5).wrapping_add(r_); l_sum_5 = n_; };
            }
            { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
        }
    }
    if ((((!((F::BtlChar_TestFlag(c, p_chr_1, ((((6i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) {
        {
            l_i_4 = 0i32;
            'l3: loop {
                if !(((((l_i_4) < (2i32)) as i32)) != 0) { break 'l3; }
                'c4: {
                    { let r_ = (((c.m.r8(((l_m_3).wrapping_add(10)).wrapping_add(((l_i_4) as u32))) as i8)) as i32); let n_ = (l_sum_5).wrapping_add(r_); l_sum_5 = n_; };
                }
                { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
            }
        }
    }
    { let r_ = (((c.m.r8((l_m_3).wrapping_add(12)) as i8)) as i32); let n_ = (l_sum_5).wrapping_add(r_); l_sum_5 = n_; };
    l_lost_6 = (rt::div_i32(({ let a_ = (F::BtlMember_GetActiveGauge(c, p_chr_1)).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_sub({ let a_ = F::BtlMember_GetActiveGauge(c, p_chr_1); (c.m.r32(a_) as i32) }), 10000i32)).wrapping_mul(3i32);
    's5: {
        let sv7: i32 = p_stat_2;
        match sv7 {
            0i32 => {
                if ((F::BtlMember_HasAbility(c, p_chr_1, 88i32)) != 0) {
                    { let r_ = l_lost_6; let n_ = (l_sum_5).wrapping_add(r_); l_sum_5 = n_; };
                }
                break 's5;
            }
            1i32 => {
                if ((F::BtlMember_HasAbility(c, p_chr_1, 113i32)) != 0) {
                    { let r_ = l_lost_6; let n_ = (l_sum_5).wrapping_add(r_); l_sum_5 = n_; };
                }
                break 's5;
            }
            2i32 => {
                if ((F::BtlMember_HasAbility(c, p_chr_1, 114i32)) != 0) {
                    { let r_ = l_lost_6; let n_ = (l_sum_5).wrapping_add(r_); l_sum_5 = n_; };
                }
                break 's5;
            }
            3i32 => {
                if ((F::BtlMember_HasAbility(c, p_chr_1, 115i32)) != 0) {
                    { let r_ = l_lost_6; let n_ = (l_sum_5).wrapping_add(r_); l_sum_5 = n_; };
                }
                break 's5;
            }
            _ => {
            }
        }
    }
    's6: {
        let sv8: i32 = F::Battle_GetStage(c);
        match sv8 {
            2i32 | 3i32 => {
                if ((F::BtlMember_HasAbility(c, p_chr_1, 89i32)) != 0) {
                    { let r_ = 15i32; let n_ = (l_sum_5).wrapping_add(r_); l_sum_5 = n_; };
                }
                break 's6;
            }
            7i32 | 8i32 | 13i32 | 16i32 | 18i32 | 34i32 => {
                if ((F::BtlMember_HasAbility(c, p_chr_1, 91i32)) != 0) {
                    { let r_ = 10i32; let n_ = (l_sum_5).wrapping_add(r_); l_sum_5 = n_; };
                }
                break 's6;
            }
            _ => {
                if ((F::BtlMember_HasAbility(c, p_chr_1, 90i32)) != 0) {
                    { let r_ = 5i32; let n_ = (l_sum_5).wrapping_add(r_); l_sum_5 = n_; };
                }
                break 's6;
            }
        }
    }
    break 'f F::BtlUtil_Clamp(c, l_sum_5, (20i32).wrapping_neg(), 20i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlStat_GetMod", &PROBE_N, &[], true); }
    r_
}

pub fn BtlStat_GetLevel0(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlStat_GetLevel0", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_bonus_2: i32 = 0;
    let r_ = 'f: {
    l_bonus_2 = { let a_ = ((F::BtlMember_GetActive(c, p_chr_1)).wrapping_add(12)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
    break 'f { let a0_ = (l_bonus_2).wrapping_add(F::BtlStat_GetMod(c, p_chr_1, 2i32)); F::BtlUtil_Clamp(c, a0_, (20i32).wrapping_neg(), 80i32) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlStat_GetLevel0", &PROBE_N, &[], true); }
    r_
}

pub fn BtlStat_GetLevel1(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlStat_GetLevel1", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_bonus_2: i32 = 0;
    let r_ = 'f: {
    l_bonus_2 = { let a_ = ((F::BtlMember_GetActive(c, p_chr_1)).wrapping_add(12)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
    break 'f { let a0_ = (l_bonus_2).wrapping_add(F::BtlStat_GetMod(c, p_chr_1, 0i32)); F::BtlUtil_Clamp(c, a0_, (20i32).wrapping_neg(), 80i32) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlStat_GetLevel1", &PROBE_N, &[], true); }
    r_
}

pub fn BtlStat_GetLevel2(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlStat_GetLevel2", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_bonus_2: i32 = 0;
    let r_ = 'f: {
    l_bonus_2 = { let a_ = ((F::BtlMember_GetActive(c, p_chr_1)).wrapping_add(12)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
    break 'f { let a0_ = (l_bonus_2).wrapping_add(F::BtlStat_GetMod(c, p_chr_1, 1i32)); F::BtlUtil_Clamp(c, a0_, (20i32).wrapping_neg(), 80i32) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlStat_GetLevel2", &PROBE_N, &[], true); }
    r_
}

pub fn BtlStat_GetLevel3(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlStat_GetLevel3", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_bonus_2: i32 = 0;
    let r_ = 'f: {
    l_bonus_2 = { let a_ = ((F::BtlMember_GetActive(c, p_chr_1)).wrapping_add(12)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
    break 'f { let a0_ = (l_bonus_2).wrapping_add(F::BtlStat_GetMod(c, p_chr_1, 2i32)); F::BtlUtil_Clamp(c, a0_, (20i32).wrapping_neg(), 80i32) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlStat_GetLevel3", &PROBE_N, &[], true); }
    r_
}

pub fn BtlStat_GetLevel4(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlStat_GetLevel4", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_bonus_2: i32 = 0;
    let r_ = 'f: {
    l_bonus_2 = { let a_ = ((F::BtlMember_GetActive(c, p_chr_1)).wrapping_add(12)).wrapping_add((((4i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
    break 'f { let a0_ = (l_bonus_2).wrapping_add(F::BtlStat_GetMod(c, p_chr_1, 3i32)); F::BtlUtil_Clamp(c, a0_, (20i32).wrapping_neg(), 80i32) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlStat_GetLevel4", &PROBE_N, &[], true); }
    r_
}

pub fn BtlStat_GetLevel5(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlStat_GetLevel5", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_bonus_2: i32 = 0;
    let r_ = 'f: {
    l_bonus_2 = { let a_ = ((F::BtlMember_GetActive(c, p_chr_1)).wrapping_add(12)).wrapping_add((((5i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
    break 'f { let a0_ = (l_bonus_2).wrapping_add(F::BtlStat_GetMod(c, p_chr_1, 3i32)); F::BtlUtil_Clamp(c, a0_, (20i32).wrapping_neg(), 80i32) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlStat_GetLevel5", &PROBE_N, &[], true); }
    r_
}

pub fn BtlStat_GetLevel6(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlStat_GetLevel6", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_bonus_2: i32 = 0;
    let r_ = 'f: {
    l_bonus_2 = { let a_ = ((F::BtlMember_GetActive(c, p_chr_1)).wrapping_add(12)).wrapping_add((((6i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
    break 'f { let a0_ = (l_bonus_2).wrapping_add(F::BtlStat_GetMod(c, p_chr_1, 3i32)); F::BtlUtil_Clamp(c, a0_, (20i32).wrapping_neg(), 80i32) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlStat_GetLevel6", &PROBE_N, &[], true); }
    r_
}

pub fn BtlStat_EvalCurve(c: &mut Ctx, mut p_level_1: i32, mut p_row_2: i32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlStat_EvalCurve", &PROBE_N, &[(p_level_1) as u32, (p_row_2) as u32], false); }
    let r_ = 'f: {
    if (((((p_level_1) >= (0i32)) as i32)) != 0) {
        break 'f rt::fadd(rt::fmul(rt::fsub(F32(c.m.r32(((sym::gBtlStatCurve).wrapping_add((((p_row_2) as u32)).wrapping_mul(12))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((sym::gBtlStatCurve).wrapping_add((((p_row_2) as u32)).wrapping_mul(12))).wrapping_add((((1i32) as u32)).wrapping_mul(4))))), rt::fdiv(rt::i2f(p_level_1), F32(0x42a00000))), F32(c.m.r32(((sym::gBtlStatCurve).wrapping_add((((p_row_2) as u32)).wrapping_mul(12))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))));
    }
    break 'f rt::fadd(rt::fmul(rt::fsub(F32(c.m.r32(((sym::gBtlStatCurve).wrapping_add((((p_row_2) as u32)).wrapping_mul(12))).wrapping_add((((2i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((sym::gBtlStatCurve).wrapping_add((((p_row_2) as u32)).wrapping_mul(12))).wrapping_add((((1i32) as u32)).wrapping_mul(4))))), rt::fdiv(rt::i2f(p_level_1), F32(0xc1a00000))), F32(c.m.r32(((sym::gBtlStatCurve).wrapping_add((((p_row_2) as u32)).wrapping_mul(12))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))));
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlStat_EvalCurve", &PROBE_N, &[], true); }
    r_
}

pub fn BtlStat_GetKiChargeBonus(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlStat_GetKiChargeBonus", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f rt::f2i(rt::fdiv({ let a0_ = F::BtlStat_GetLevel0(c, p_chr_1); F::BtlStat_EvalCurve(c, a0_, 0i32) }, F32(0x41f00000)));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlStat_GetKiChargeBonus", &PROBE_N, &[], true); }
    r_
}

pub fn BtlStat_GetKiRegenBonus(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlStat_GetKiRegenBonus", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f rt::f2i(rt::fdiv({ let a0_ = F::BtlStat_GetLevel0(c, p_chr_1); F::BtlStat_EvalCurve(c, a0_, 1i32) }, F32(0x41f00000)));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlStat_GetKiRegenBonus", &PROBE_N, &[], true); }
    r_
}

pub fn BtlStat_GetKiRecoverBonus(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlStat_GetKiRecoverBonus", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f rt::f2i(rt::fdiv({ let a0_ = F::BtlStat_GetLevel0(c, p_chr_1); F::BtlStat_EvalCurve(c, a0_, 2i32) }, F32(0x41f00000)));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlStat_GetKiRecoverBonus", &PROBE_N, &[], true); }
    r_
}

pub fn BtlStat_GetBlastGainBonus(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlStat_GetBlastGainBonus", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f rt::f2i(rt::fdiv({ let a0_ = F::BtlStat_GetLevel4(c, p_chr_1); F::BtlStat_EvalCurve(c, a0_, 3i32) }, F32(0x41f00000)));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlStat_GetBlastGainBonus", &PROBE_N, &[], true); }
    r_
}

pub fn BtlStat_GetMeleeDamageScale(c: &mut Ctx, mut p_chr_1: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlStat_GetMeleeDamageScale", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f { let a0_ = F::BtlStat_GetLevel1(c, p_chr_1); F::BtlStat_EvalCurve(c, a0_, 4i32) };
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlStat_GetMeleeDamageScale", &PROBE_N, &[], true); }
    r_
}

pub fn BtlStat_GetGuardKiCostScale(c: &mut Ctx, mut p_chr_1: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlStat_GetGuardKiCostScale", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f { let a0_ = F::BtlStat_GetLevel1(c, p_chr_1); F::BtlStat_EvalCurve(c, a0_, 5i32) };
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlStat_GetGuardKiCostScale", &PROBE_N, &[], true); }
    r_
}

pub fn BtlStat_GetKiBlastDamageScale(c: &mut Ctx, mut p_chr_1: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlStat_GetKiBlastDamageScale", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f { let a0_ = F::BtlStat_GetLevel1(c, p_chr_1); F::BtlStat_EvalCurve(c, a0_, 6i32) };
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlStat_GetKiBlastDamageScale", &PROBE_N, &[], true); }
    r_
}

pub fn BtlStat_GetScale7(c: &mut Ctx, mut p_chr_1: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlStat_GetScale7", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f { let a0_ = F::BtlStat_GetLevel2(c, p_chr_1); F::BtlStat_EvalCurve(c, a0_, 7i32) };
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlStat_GetScale7", &PROBE_N, &[], true); }
    r_
}

pub fn BtlStat_GetSpeedScale(c: &mut Ctx, mut p_chr_1: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlStat_GetSpeedScale", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f { let a0_ = F::BtlStat_GetLevel3(c, p_chr_1); F::BtlStat_EvalCurve(c, a0_, 8i32) };
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlStat_GetSpeedScale", &PROBE_N, &[], true); }
    r_
}

pub fn BtlStat_GetBlast2DamageScale(c: &mut Ctx, mut p_chr_1: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlStat_GetBlast2DamageScale", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f { let a0_ = F::BtlStat_GetLevel5(c, p_chr_1); F::BtlStat_EvalCurve(c, a0_, 9i32) };
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlStat_GetBlast2DamageScale", &PROBE_N, &[], true); }
    r_
}

pub fn BtlStat_GetUltimateDamageScale(c: &mut Ctx, mut p_chr_1: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlStat_GetUltimateDamageScale", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f { let a0_ = F::BtlStat_GetLevel6(c, p_chr_1); F::BtlStat_EvalCurve(c, a0_, 10i32) };
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlStat_GetUltimateDamageScale", &PROBE_N, &[], true); }
    r_
}

pub fn BtlStat_GetMaxPowerChargeScale(c: &mut Ctx, mut p_chr_1: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlStat_GetMaxPowerChargeScale", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f { let a0_ = F::BtlStat_GetLevel0(c, p_chr_1); F::BtlStat_EvalCurve(c, a0_, 11i32) };
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlStat_GetMaxPowerChargeScale", &PROBE_N, &[], true); }
    r_
}

pub fn BtlStat_GetMaxPowerExtraTime(c: &mut Ctx, mut p_chr_1: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlStat_GetMaxPowerExtraTime", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f { let a0_ = F::BtlStat_GetLevel0(c, p_chr_1); F::BtlStat_EvalCurve(c, a0_, 12i32) };
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlStat_GetMaxPowerExtraTime", &PROBE_N, &[], true); }
    r_
}
