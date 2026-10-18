#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn BtlChar_PromoteFlags(c: &mut Ctx, mut p_chr_1: u32, mut p_stage_2: u8) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_PromoteFlags", &PROBE_N, &[(p_chr_1) as u32, (p_stage_2) as u32], false); }
    let mut l_n_3: i32 = 0;
    let mut l_i_4: i32 = 0;
    let mut l_m_5: u8 = 0;
    let r_ = 'f: {
    {
        l_n_3 = 0i32;
        'l1: loop {
            if !(((((l_n_3) < (317i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if (((((((c.m.r8(((p_chr_1).wrapping_add(4389)).wrapping_add(((l_n_3) as u32)))) as i32)) == (((p_stage_2) as i32))) as i32)) != 0) {
                    l_i_4 = (l_n_3).wrapping_shr((3i32) as u32);
                    l_m_5 = (((1i32).wrapping_shl((((l_n_3) & (7i32))) as u32)) as u8);
                    { let r_ = (!(((l_m_5) as i32))); let a_ = ((p_chr_1).wrapping_add(4309)).wrapping_add(((l_i_4) as u32)); let n_ = ((((((c.m.r8(a_)) as i32)) & (r_))) as u8); c.m.w8(a_, n_); };
                    { let r_ = ((((c.m.r8(((p_chr_1).wrapping_add(4229)).wrapping_add(((l_i_4) as u32)))) as i32)) & (((l_m_5) as i32))); let a_ = ((p_chr_1).wrapping_add(4309)).wrapping_add(((l_i_4) as u32)); let n_ = ((((((c.m.r8(a_)) as i32)) | (r_))) as u8); c.m.w8(a_, n_); };
                    { let r_ = (!(((l_m_5) as i32))); let a_ = ((p_chr_1).wrapping_add(4349)).wrapping_add(((l_i_4) as u32)); let n_ = ((((((c.m.r8(a_)) as i32)) & (r_))) as u8); c.m.w8(a_, n_); };
                    { let r_ = ((((c.m.r8(((p_chr_1).wrapping_add(4269)).wrapping_add(((l_i_4) as u32)))) as i32)) & (((l_m_5) as i32))); let a_ = ((p_chr_1).wrapping_add(4349)).wrapping_add(((l_i_4) as u32)); let n_ = ((((((c.m.r8(a_)) as i32)) | (r_))) as u8); c.m.w8(a_, n_); };
                    { let r_ = (!(((l_m_5) as i32))); let a_ = ((p_chr_1).wrapping_add(4269)).wrapping_add(((l_i_4) as u32)); let n_ = ((((((c.m.r8(a_)) as i32)) & (r_))) as u8); c.m.w8(a_, n_); };
                }
            }
            { let o_ = l_n_3; let n_ = (o_).wrapping_add(1); l_n_3 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_PromoteFlags", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChar_SetStage(c: &mut Ctx, mut p_chr_1: u32, mut p_stage_2: u8) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_SetStage", &PROBE_N, &[(p_chr_1) as u32, (p_stage_2) as u32], false); }
    let r_ = 'f: {
    if (((((((p_stage_2) as i32)) < (((c.m.r8((p_chr_1).wrapping_add(4228))) as i32))) as i32)) != 0) {
        F::BtlChar_PromoteFlags(c, p_chr_1, ((1i32) as u8));
    }
    F::BtlChar_PromoteFlags(c, p_chr_1, p_stage_2);
    { let v_ = p_stage_2; let a_ = (p_chr_1).wrapping_add(4228); c.m.w8(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_SetStage", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChar_GetStage(c: &mut Ctx, mut p_chr_1: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_GetStage", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f ((c.m.r8((p_chr_1).wrapping_add(4228))) as u32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_GetStage", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChar_SetHeldFlag(c: &mut Ctx, mut p_chr_1: u32, mut p_n_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_SetHeldFlag", &PROBE_N, &[(p_chr_1) as u32, (p_n_2) as u32], false); }
    let mut l_i_3: u32 = 0;
    let mut l_m_4: u8 = 0;
    let r_ = 'f: {
    l_i_3 = (p_n_2).wrapping_shr((3i32) as u32);
    l_m_4 = (((1i32).wrapping_shl((((p_n_2) & (((7i32) as u32)))) as u32)) as u8);
    if (((((((c.m.r8((p_chr_1).wrapping_add(4228))) as i32)) < (((c.m.r8(((p_chr_1).wrapping_add(4389)).wrapping_add(p_n_2))) as i32))) as i32)) != 0) {
        { let r_ = (!(((l_m_4) as i32))); let a_ = ((p_chr_1).wrapping_add(4309)).wrapping_add(l_i_3); let n_ = ((((((c.m.r8(a_)) as i32)) & (r_))) as u8); c.m.w8(a_, n_); };
        { let r_ = ((((c.m.r8(((p_chr_1).wrapping_add(4229)).wrapping_add(l_i_3))) as i32)) & (((l_m_4) as i32))); let a_ = ((p_chr_1).wrapping_add(4309)).wrapping_add(l_i_3); let n_ = ((((((c.m.r8(a_)) as i32)) | (r_))) as u8); c.m.w8(a_, n_); };
    }
    { let r_ = ((l_m_4) as i32); let a_ = ((p_chr_1).wrapping_add(4229)).wrapping_add(l_i_3); let n_ = ((((((c.m.r8(a_)) as i32)) | (r_))) as u8); c.m.w8(a_, n_); };
    { let v_ = c.m.r8((p_chr_1).wrapping_add(4228)); let a_ = ((p_chr_1).wrapping_add(4389)).wrapping_add(p_n_2); c.m.w8(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_SetHeldFlag", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChar_ClearFlag(c: &mut Ctx, mut p_chr_1: u32, mut p_n_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_ClearFlag", &PROBE_N, &[(p_chr_1) as u32, (p_n_2) as u32], false); }
    let mut l_i_3: u32 = 0;
    let mut l_m_4: u8 = 0;
    let r_ = 'f: {
    l_i_3 = (p_n_2).wrapping_shr((3i32) as u32);
    l_m_4 = (((1i32).wrapping_shl((((p_n_2) & (((7i32) as u32)))) as u32)) as u8);
    if (((((((c.m.r8((p_chr_1).wrapping_add(4228))) as i32)) < (((c.m.r8(((p_chr_1).wrapping_add(4389)).wrapping_add(p_n_2))) as i32))) as i32)) != 0) {
        { let r_ = (!(((l_m_4) as i32))); let a_ = ((p_chr_1).wrapping_add(4309)).wrapping_add(l_i_3); let n_ = ((((((c.m.r8(a_)) as i32)) & (r_))) as u8); c.m.w8(a_, n_); };
        { let r_ = ((((c.m.r8(((p_chr_1).wrapping_add(4229)).wrapping_add(l_i_3))) as i32)) & (((l_m_4) as i32))); let a_ = ((p_chr_1).wrapping_add(4309)).wrapping_add(l_i_3); let n_ = ((((((c.m.r8(a_)) as i32)) | (r_))) as u8); c.m.w8(a_, n_); };
    }
    { let r_ = (!(((l_m_4) as i32))); let a_ = ((p_chr_1).wrapping_add(4229)).wrapping_add(l_i_3); let n_ = ((((((c.m.r8(a_)) as i32)) & (r_))) as u8); c.m.w8(a_, n_); };
    { let r_ = (!(((l_m_4) as i32))); let a_ = ((p_chr_1).wrapping_add(4269)).wrapping_add(l_i_3); let n_ = ((((((c.m.r8(a_)) as i32)) & (r_))) as u8); c.m.w8(a_, n_); };
    { let v_ = c.m.r8((p_chr_1).wrapping_add(4228)); let a_ = ((p_chr_1).wrapping_add(4389)).wrapping_add(p_n_2); c.m.w8(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_ClearFlag", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChar_ClearFlagRange(c: &mut Ctx, mut p_chr_1: u32, mut p_a_2: u32, mut p_b_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_ClearFlagRange", &PROBE_N, &[(p_chr_1) as u32, (p_a_2) as u32, (p_b_3) as u32], false); }
    let mut l_n_4: u32 = 0;
    let mut l_t_5: u32 = 0;
    let r_ = 'f: {
    if (((((p_b_3) < (p_a_2)) as i32)) != 0) {
        l_t_5 = p_a_2;
        p_a_2 = p_b_3;
        p_b_3 = l_t_5;
    }
    {
        l_n_4 = p_a_2;
        'l1: loop {
            if !(((((l_n_4) <= (p_b_3)) as i32)) != 0) { break 'l1; }
            'c2: {
                F::BtlChar_ClearFlag(c, p_chr_1, l_n_4);
            }
            { let o_ = l_n_4; let n_ = (o_).wrapping_add(1); l_n_4 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_ClearFlagRange", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChar_ToggleHeldFlag(c: &mut Ctx, mut p_chr_1: u32, mut p_n_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_ToggleHeldFlag", &PROBE_N, &[(p_chr_1) as u32, (p_n_2) as u32], false); }
    let mut l_i_3: u32 = 0;
    let mut l_m_4: u8 = 0;
    let r_ = 'f: {
    l_i_3 = (p_n_2).wrapping_shr((3i32) as u32);
    l_m_4 = (((1i32).wrapping_shl((((p_n_2) & (((7i32) as u32)))) as u32)) as u8);
    if (((((((c.m.r8((p_chr_1).wrapping_add(4228))) as i32)) < (((c.m.r8(((p_chr_1).wrapping_add(4389)).wrapping_add(p_n_2))) as i32))) as i32)) != 0) {
        { let r_ = (!(((l_m_4) as i32))); let a_ = ((p_chr_1).wrapping_add(4309)).wrapping_add(l_i_3); let n_ = ((((((c.m.r8(a_)) as i32)) & (r_))) as u8); c.m.w8(a_, n_); };
        { let r_ = ((((c.m.r8(((p_chr_1).wrapping_add(4229)).wrapping_add(l_i_3))) as i32)) & (((l_m_4) as i32))); let a_ = ((p_chr_1).wrapping_add(4309)).wrapping_add(l_i_3); let n_ = ((((((c.m.r8(a_)) as i32)) | (r_))) as u8); c.m.w8(a_, n_); };
    }
    { let r_ = ((l_m_4) as i32); let a_ = ((p_chr_1).wrapping_add(4229)).wrapping_add(l_i_3); let n_ = ((((((c.m.r8(a_)) as i32)) ^ (r_))) as u8); c.m.w8(a_, n_); };
    { let v_ = c.m.r8((p_chr_1).wrapping_add(4228)); let a_ = ((p_chr_1).wrapping_add(4389)).wrapping_add(p_n_2); c.m.w8(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_ToggleHeldFlag", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChar_SetFlag(c: &mut Ctx, mut p_chr_1: u32, mut p_n_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_SetFlag", &PROBE_N, &[(p_chr_1) as u32, (p_n_2) as u32], false); }
    let mut l_i_3: u32 = 0;
    let mut l_m_4: u8 = 0;
    let r_ = 'f: {
    l_i_3 = (p_n_2).wrapping_shr((3i32) as u32);
    l_m_4 = (((1i32).wrapping_shl((((p_n_2) & (((7i32) as u32)))) as u32)) as u8);
    if (((((((c.m.r8((p_chr_1).wrapping_add(4228))) as i32)) < (((c.m.r8(((p_chr_1).wrapping_add(4389)).wrapping_add(p_n_2))) as i32))) as i32)) != 0) {
        { let r_ = (!(((l_m_4) as i32))); let a_ = ((p_chr_1).wrapping_add(4349)).wrapping_add(l_i_3); let n_ = ((((((c.m.r8(a_)) as i32)) & (r_))) as u8); c.m.w8(a_, n_); };
        { let r_ = ((((c.m.r8(((p_chr_1).wrapping_add(4269)).wrapping_add(l_i_3))) as i32)) & (((l_m_4) as i32))); let a_ = ((p_chr_1).wrapping_add(4349)).wrapping_add(l_i_3); let n_ = ((((((c.m.r8(a_)) as i32)) | (r_))) as u8); c.m.w8(a_, n_); };
        { let r_ = (!(((l_m_4) as i32))); let a_ = ((p_chr_1).wrapping_add(4269)).wrapping_add(l_i_3); let n_ = ((((((c.m.r8(a_)) as i32)) & (r_))) as u8); c.m.w8(a_, n_); };
    }
    { let r_ = ((l_m_4) as i32); let a_ = ((p_chr_1).wrapping_add(4269)).wrapping_add(l_i_3); let n_ = ((((((c.m.r8(a_)) as i32)) | (r_))) as u8); c.m.w8(a_, n_); };
    { let v_ = c.m.r8((p_chr_1).wrapping_add(4228)); let a_ = ((p_chr_1).wrapping_add(4389)).wrapping_add(p_n_2); c.m.w8(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_SetFlag", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChar_TestFlag(c: &mut Ctx, mut p_chr_1: u32, mut p_n_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_TestFlag", &PROBE_N, &[(p_chr_1) as u32, (p_n_2) as u32], false); }
    let mut l_i_3: u32 = 0;
    let mut l_m_4: u8 = 0;
    let mut l_cur_5: u8 = 0;
    let r_ = 'f: {
    l_i_3 = (p_n_2).wrapping_shr((3i32) as u32);
    l_m_4 = (((1i32).wrapping_shl((((p_n_2) & (((7i32) as u32)))) as u32)) as u8);
    l_cur_5 = ((((((c.m.r8(((p_chr_1).wrapping_add(4229)).wrapping_add(l_i_3))) as i32)) | (((c.m.r8(((p_chr_1).wrapping_add(4269)).wrapping_add(l_i_3))) as i32)))) as u8);
    break 'f (((((((l_cur_5) as i32)) & (((l_m_4) as i32)))) != (0i32)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_TestFlag", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChar_TestPrevFlag(c: &mut Ctx, mut p_chr_1: u32, mut p_n_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_TestPrevFlag", &PROBE_N, &[(p_chr_1) as u32, (p_n_2) as u32], false); }
    let mut l_i_3: u32 = 0;
    let mut l_m_4: u8 = 0;
    let mut l_prev_5: u8 = 0;
    let r_ = 'f: {
    l_i_3 = (p_n_2).wrapping_shr((3i32) as u32);
    l_m_4 = (((1i32).wrapping_shl((((p_n_2) & (((7i32) as u32)))) as u32)) as u8);
    l_prev_5 = ((((((c.m.r8(((p_chr_1).wrapping_add(4309)).wrapping_add(l_i_3))) as i32)) | (((c.m.r8(((p_chr_1).wrapping_add(4349)).wrapping_add(l_i_3))) as i32)))) as u8);
    break 'f (((((((l_prev_5) as i32)) & (((l_m_4) as i32)))) != (0i32)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_TestPrevFlag", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChar_IsFlagRaised(c: &mut Ctx, mut p_chr_1: u32, mut p_n_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_IsFlagRaised", &PROBE_N, &[(p_chr_1) as u32, (p_n_2) as u32], false); }
    let mut l_i_3: u32 = 0;
    let mut l_m_4: u8 = 0;
    let r_ = 'f: {
    l_i_3 = (p_n_2).wrapping_shr((3i32) as u32);
    l_m_4 = (((1i32).wrapping_shl((((p_n_2) & (((7i32) as u32)))) as u32)) as u8);
    break 'f (((((((((((c.m.r8(((p_chr_1).wrapping_add(4229)).wrapping_add(l_i_3))) as i32)) | (((c.m.r8(((p_chr_1).wrapping_add(4269)).wrapping_add(l_i_3))) as i32)))) & ((!(((((c.m.r8(((p_chr_1).wrapping_add(4309)).wrapping_add(l_i_3))) as i32)) | (((c.m.r8(((p_chr_1).wrapping_add(4349)).wrapping_add(l_i_3))) as i32)))))))) & (((l_m_4) as i32)))) != (0i32)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_IsFlagRaised", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChar_IsFlagDropped(c: &mut Ctx, mut p_chr_1: u32, mut p_n_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_IsFlagDropped", &PROBE_N, &[(p_chr_1) as u32, (p_n_2) as u32], false); }
    let mut l_i_3: u32 = 0;
    let mut l_m_4: u8 = 0;
    let mut l_cur_5: u8 = 0;
    let mut l_prev_6: u8 = 0;
    let r_ = 'f: {
    l_i_3 = (p_n_2).wrapping_shr((3i32) as u32);
    l_m_4 = (((1i32).wrapping_shl((((p_n_2) & (((7i32) as u32)))) as u32)) as u8);
    l_cur_5 = ((((((c.m.r8(((p_chr_1).wrapping_add(4229)).wrapping_add(l_i_3))) as i32)) | (((c.m.r8(((p_chr_1).wrapping_add(4269)).wrapping_add(l_i_3))) as i32)))) as u8);
    l_prev_6 = ((((((c.m.r8(((p_chr_1).wrapping_add(4309)).wrapping_add(l_i_3))) as i32)) | (((c.m.r8(((p_chr_1).wrapping_add(4349)).wrapping_add(l_i_3))) as i32)))) as u8);
    break 'f (((((((((l_prev_6) as i32)) & ((!(((l_cur_5) as i32)))))) & (((l_m_4) as i32)))) != (0i32)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_IsFlagDropped", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChar_UpdateLockOn(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_UpdateLockOn", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_range_2: F32 = F32(0);
    let r_ = 'f: {
    if ((F::BtlChar_TestFlag(c, p_chr_1, ((147i32) as u32))) != 0) {
        F::BtlChar_ClearFlag(c, p_chr_1, ((5i32) as u32));
    }
    if ((F::BtlChar_TestFlag(c, p_chr_1, ((178i32) as u32))) != 0) {
        F::BtlChar_ClearFlag(c, p_chr_1, ((5i32) as u32));
    }
    F::BtlChar_ClearFlag(c, p_chr_1, ((19i32) as u32));
    if ((((((((((((((((!((F::BtlChar_TestFlag(c, p_chr_1, ((27i32) as u32))) != 0)) as i32)) != 0) && ((((!((F::BtlChar_TestFlag(c, p_chr_1, ((147i32) as u32))) != 0)) as i32)) != 0)) as i32)) != 0) && ((((((c.m.r32((p_chr_1).wrapping_add(4016)) as i32)) < (3i32)) as i32)) != 0)) as i32)) != 0) && ((((!((F::BtlChar_TestFlag(c, p_chr_1, ((178i32) as u32))) != 0)) as i32)) != 0)) as i32)) != 0) {
        l_range_2 = F::BtlChar_GetSpacing(c, p_chr_1, 0i32);
        if (((rt::flt(F::BtlOpp_GetDistance(c, p_chr_1), l_range_2) as i32)) != 0) {
            F::BtlChar_SetHeldFlag(c, p_chr_1, ((19i32) as u32));
            F::BtlChar_SetHeldFlag(c, p_chr_1, ((5i32) as u32));
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_UpdateLockOn", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChars_UpdateSightFlag(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChars_UpdateSightFlag", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(64) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 64);
    let mut l_chr0_1: u32 = 0;
    let mut l_chr1_2: u32 = 0;
    let r_ = 'f: {
        l_chr0_1 = F::BtlChar_Get(c, 0i32);
        l_chr1_2 = F::BtlChar_Get(c, 1i32);
        F::BtlChar_ClearFlag(c, l_chr0_1, ((186i32) as u32));
        F::BtlChar_ClearFlag(c, l_chr1_2, ((186i32) as u32));
        { let a0_ = (c.m.r32((l_chr0_1).wrapping_add(12)) as i32); F::BtlCharApi_GetNodePos(c, a0_, 47i32, fp.wrapping_add(32)) };
        { let a0_ = (c.m.r32((l_chr1_2).wrapping_add(12)) as i32); F::BtlCharApi_GetNodePos(c, a0_, 47i32, fp.wrapping_add(48)) };
        F::ColSeg_Set(c, fp, fp.wrapping_add(32), fp.wrapping_add(48));
        if ((F::StgCol_TraceSegment(c, fp)) != 0) {
            F::BtlChar_SetHeldFlag(c, l_chr0_1, ((186i32) as u32));
            F::BtlChar_SetHeldFlag(c, l_chr1_2, ((186i32) as u32));
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlChars_UpdateSightFlag", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChar_SetOnceBit(c: &mut Ctx, mut p_chr_1: u32, mut p_n_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_SetOnceBit", &PROBE_N, &[(p_chr_1) as u32, (p_n_2) as u32], false); }
    let r_ = 'f: {
    { let r_ = (1i32).wrapping_shl((((p_n_2) & (7i32))) as u32); let a_ = (((p_chr_1).wrapping_add(5432)).wrapping_add(8)).wrapping_add((((p_n_2).wrapping_shr((3i32) as u32)) as u32)); let n_ = ((((((c.m.r8(a_)) as i32)) | (r_))) as u8); c.m.w8(a_, n_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_SetOnceBit", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChar_TestOnceBit(c: &mut Ctx, mut p_chr_1: u32, mut p_n_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_TestOnceBit", &PROBE_N, &[(p_chr_1) as u32, (p_n_2) as u32], false); }
    let mut l_m_3: u8 = 0;
    let r_ = 'f: {
    l_m_3 = (((1i32).wrapping_shl((((p_n_2) & (7i32))) as u32)) as u8);
    break 'f (((((((c.m.r8((((p_chr_1).wrapping_add(5432)).wrapping_add(8)).wrapping_add((((p_n_2).wrapping_shr((3i32) as u32)) as u32)))) as i32)) & (((l_m_3) as i32)))) != (0i32)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_TestOnceBit", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChar_SetFrameBits(c: &mut Ctx, mut p_chr_1: u32, mut p_bits_2: u64) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_SetFrameBits", &PROBE_N, &[(p_chr_1) as u32, (p_bits_2) as u32], false); }
    let r_ = 'f: {
    { let r_ = p_bits_2; let a_ = (p_chr_1).wrapping_add(5432); let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_SetFrameBits", &PROBE_N, &[], true); }
    r_
}

pub fn BtlChar_RaiseFirstClash(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_RaiseFirstClash", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    if ((((((((!(({ let a0_ = F::BtlChar_Get(c, 0i32); F::BtlChar_TestOnceBit(c, a0_, 0i32) }) != 0)) as i32)) != 0) && ((((!(({ let a0_ = F::BtlChar_Get(c, 1i32); F::BtlChar_TestOnceBit(c, a0_, 0i32) }) != 0)) as i32)) != 0)) as i32)) != 0) {
        F::BtlChar_SetOnceBit(c, p_chr_1, 0i32);
        F::BtlChar_SetFlag(c, p_chr_1, ((227i32) as u32));
        { let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlEvent_Raise(c, a0_, 60i32) };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlChar_RaiseFirstClash", &PROBE_N, &[], true); }
    r_
}
