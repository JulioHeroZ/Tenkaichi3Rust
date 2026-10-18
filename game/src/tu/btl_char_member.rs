#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn BtlColl_NextPoolMember(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlColl_NextPoolMember", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_pool_2: u32 = 0;
    let mut l_next_3: i32 = 0;
    let r_ = 'f: {
    if ((((((c.m.r32(p_chr_1) as i32)) != (1i32)) as i32)) != 0) {
        break 'f;
    }
    l_pool_2 = F::Battle_GetWork5A8(c);
    if (((((l_pool_2) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    l_next_3 = ((c.m.r32(l_pool_2) as i32)).wrapping_add(1i32);
    if (((((l_next_3) < ((c.m.r32((l_pool_2).wrapping_add(4)) as i32))) as i32)) != 0) {
        if (((((F::BtlMember_Replace(c, p_chr_1, ((l_pool_2).wrapping_add(8)).wrapping_add((((l_next_3) as u32)).wrapping_mul(100)))) >= (0i32)) as i32)) != 0) {
            { let v_ = l_next_3; let a_ = l_pool_2; c.m.w32(a_, (v_) as u32); };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlColl_NextPoolMember", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMember_LoadParams(c: &mut Ctx, mut p_chr_1: u32, mut p_member_2: i32, mut p_init_3: i32, mut p_healthPct_4: F32, mut p_variant_5: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMember_LoadParams", &PROBE_N, &[(p_chr_1) as u32, (p_member_2) as u32, (p_init_3) as u32, (p_healthPct_4).0, (p_variant_5) as u32], false); }
    let mut l_m_6: u32 = 0;
    let mut l_g_7: u32 = 0;
    let r_ = 'f: {
    l_m_6 = F::BtlMember_Get(c, p_chr_1, p_member_2);
    l_g_7 = (l_m_6).wrapping_add(64);
    if ((p_init_3) != 0) {
        { let v_ = 0i32; let a_ = (l_g_7).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (l_g_7).wrapping_add(52); c.m.w32(a_, (v_) as u32); };
    }
    if ((((((c.m.r32((l_g_7).wrapping_add(52)) as i32)) == (0i32)) as i32)) != 0) {
        { let v_ = { let a_ = ({ let a_ = (c.m.r32(sym::gBtlChars)).wrapping_add(48); c.m.r32(a_) }).wrapping_add(((((c.m.r32(l_m_6) as i32)) as u32)).wrapping_mul(16)); (c.m.r32(a_) as i32) }; let a_ = (l_g_7).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        if ((F::BtlMember_HasAbilityOf(c, p_chr_1, p_member_2, 100i32)) != 0) {
            { let r_ = 30000i32; let a_ = (l_g_7).wrapping_add(4); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
        } else {
            if ((F::BtlMember_HasAbilityOf(c, p_chr_1, p_member_2, 99i32)) != 0) {
                { let r_ = 20000i32; let a_ = (l_g_7).wrapping_add(4); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
            } else {
                if ((F::BtlMember_HasAbilityOf(c, p_chr_1, p_member_2, 98i32)) != 0) {
                    { let r_ = 10000i32; let a_ = (l_g_7).wrapping_add(4); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                }
            }
        }
        if ((F::BtlMember_HasAbilityOf(c, p_chr_1, p_member_2, 103i32)) != 0) {
            { let r_ = 30000i32; let a_ = (l_g_7).wrapping_add(4); let n_ = ((c.m.r32(a_) as i32)).wrapping_sub(r_); c.m.w32(a_, (n_) as u32); };
        } else {
            if ((F::BtlMember_HasAbilityOf(c, p_chr_1, p_member_2, 102i32)) != 0) {
                { let r_ = 20000i32; let a_ = (l_g_7).wrapping_add(4); let n_ = ((c.m.r32(a_) as i32)).wrapping_sub(r_); c.m.w32(a_, (n_) as u32); };
            } else {
                if ((F::BtlMember_HasAbilityOf(c, p_chr_1, p_member_2, 101i32)) != 0) {
                    { let r_ = 10000i32; let a_ = (l_g_7).wrapping_add(4); let n_ = ((c.m.r32(a_) as i32)).wrapping_sub(r_); c.m.w32(a_, (n_) as u32); };
                }
            }
        }
        { let v_ = { let a0_ = (c.m.r32((l_g_7).wrapping_add(4)) as i32); F::BtlUtil_Max(c, a0_, 10000i32) }; let a_ = (l_g_7).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
    }
    { let v_ = 100000i32; let a_ = (l_g_7).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
    { let v_ = ({ let a_ = (({ let a_ = (c.m.r32(sym::gBtlChars)).wrapping_add(48); c.m.r32(a_) }).wrapping_add(((((c.m.r32(l_m_6) as i32)) as u32)).wrapping_mul(16))).wrapping_add(12); (c.m.r32(a_) as i32) }).wrapping_mul(100000i32); let a_ = (l_g_7).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
    if ((p_init_3) != 0) {
        { let v_ = rt::f2i(rt::fdiv(rt::fmul(rt::i2f((c.m.r32((l_g_7).wrapping_add(4)) as i32)), p_healthPct_4), F32(0x42c80000))); let a_ = l_g_7; c.m.w32(a_, (v_) as u32); };
        { let v_ = { let a0_ = (c.m.r32(l_g_7) as i32); F::BtlUtil_Max(c, a0_, 1i32) }; let a_ = l_g_7; c.m.w32(a_, (v_) as u32); };
        { let v_ = { let a0_ = (c.m.r32((l_g_7).wrapping_add(4)) as i32); let a1_ = (c.m.r32(l_g_7) as i32); F::BtlUtil_Max(c, a0_, a1_) }; let a_ = (l_g_7).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = { let a_ = (({ let a_ = (c.m.r32(sym::gBtlChars)).wrapping_add(48); c.m.r32(a_) }).wrapping_add(((((c.m.r32(l_m_6) as i32)) as u32)).wrapping_mul(16))).wrapping_add(4); (c.m.r32(a_) as i32) }; let a_ = (l_g_7).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        if ((F::BtlMember_HasAbilityOf(c, p_chr_1, p_member_2, 24i32)) != 0) {
            { let v_ = 0i32; let a_ = (l_g_7).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        } else {
            if ((F::BtlMember_HasAbilityOf(c, p_chr_1, p_member_2, 25i32)) != 0) {
                { let v_ = (c.m.r32((l_g_7).wrapping_add(16)) as i32); let a_ = (l_g_7).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
            } else {
                if ((F::BtlMember_HasAbilityOf(c, p_chr_1, p_member_2, 23i32)) != 0) {
                    { let r_ = 40000i32; let a_ = (l_g_7).wrapping_add(12); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                } else {
                    if ((F::BtlMember_HasAbilityOf(c, p_chr_1, p_member_2, 22i32)) != 0) {
                        { let r_ = 20000i32; let a_ = (l_g_7).wrapping_add(12); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                    }
                }
            }
        }
        { let v_ = { let a0_ = (c.m.r32((l_g_7).wrapping_add(12)) as i32); let a1_ = (c.m.r32((l_g_7).wrapping_add(16)) as i32); F::BtlUtil_Min(c, a0_, a1_) }; let a_ = (l_g_7).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (l_g_7).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
        if ((F::BtlMember_HasAbilityOf(c, p_chr_1, p_member_2, 26i32)) != 0) {
            { let v_ = (c.m.r32((l_g_7).wrapping_add(24)) as i32); let a_ = (l_g_7).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
        }
        { let v_ = p_variant_5; let a_ = (l_g_7).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (l_g_7).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMember_LoadParams", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMember_Get(c: &mut Ctx, mut p_chr_1: u32, mut p_n_2: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMember_Get", &PROBE_N, &[(p_chr_1) as u32, (p_n_2) as u32], false); }
    let r_ = 'f: {
    break 'f ((p_chr_1).wrapping_add(2468)).wrapping_add((((p_n_2) as u32)).wrapping_mul(164));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMember_Get", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMember_GetActive(c: &mut Ctx, mut p_chr_1: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMember_GetActive", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f { let a1_ = (c.m.r32((p_chr_1).wrapping_add(2452)) as i32); F::BtlMember_Get(c, p_chr_1, a1_) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMember_GetActive", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMember_FindPresent(c: &mut Ctx, mut p_chr_1: u32, mut p_chara_2: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMember_FindPresent", &PROBE_N, &[(p_chr_1) as u32, (p_chara_2) as u32], false); }
    let mut l_i_3: i32 = 0;
    let r_ = 'f: {
    {
        l_i_3 = 0i32;
        'l1: loop {
            if !(((((l_i_3) < ((c.m.r32((p_chr_1).wrapping_add(2456)) as i32))) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((((((((({ let a_ = (F::BtlMember_Get(c, p_chr_1, l_i_3)).wrapping_add(8); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) && ((((({ let a_ = F::BtlMember_Get(c, p_chr_1, l_i_3); (c.m.r32(a_) as i32) }) == (p_chara_2)) as i32)) != 0)) as i32)) != 0) {
                    break 'f F::BtlMember_Get(c, p_chr_1, l_i_3);
                }
            }
            { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
        }
    }
    break 'f ((0i32) as u32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMember_FindPresent", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMember_FindPresentIndex(c: &mut Ctx, mut p_chr_1: u32, mut p_chara_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMember_FindPresentIndex", &PROBE_N, &[(p_chr_1) as u32, (p_chara_2) as u32], false); }
    let mut l_i_3: i32 = 0;
    let r_ = 'f: {
    {
        l_i_3 = 0i32;
        'l1: loop {
            if !(((((l_i_3) < ((c.m.r32((p_chr_1).wrapping_add(2456)) as i32))) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((((((((({ let a_ = (F::BtlMember_Get(c, p_chr_1, l_i_3)).wrapping_add(8); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) && ((((({ let a_ = F::BtlMember_Get(c, p_chr_1, l_i_3); (c.m.r32(a_) as i32) }) == (p_chara_2)) as i32)) != 0)) as i32)) != 0) {
                    break 'f l_i_3;
                }
            }
            { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
        }
    }
    break 'f (1i32).wrapping_neg();
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMember_FindPresentIndex", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMember_GetGauge(c: &mut Ctx, mut p_chr_1: u32, mut p_n_2: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMember_GetGauge", &PROBE_N, &[(p_chr_1) as u32, (p_n_2) as u32], false); }
    let r_ = 'f: {
    break 'f (F::BtlMember_Get(c, p_chr_1, p_n_2)).wrapping_add(64);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMember_GetGauge", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMember_GetActiveGauge(c: &mut Ctx, mut p_chr_1: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMember_GetActiveGauge", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f (F::BtlMember_GetActive(c, p_chr_1)).wrapping_add(64);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMember_GetActiveGauge", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMember_Replace(c: &mut Ctx, mut p_chr_1: u32, mut p_src_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMember_Replace", &PROBE_N, &[(p_chr_1) as u32, (p_src_2) as u32], false); }
    let mut l_slot_3: i32 = 0;
    let mut l_i_4: i32 = 0;
    let mut l_m_5: u32 = 0;
    let mut l_health_6: F32 = F32(0);
    let r_ = 'f: {
    l_slot_3 = (1i32).wrapping_neg();
    {
        l_i_4 = 0i32;
        'l1: loop {
            if !(((((l_i_4) < (5i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((((((((((c.m.r32((p_chr_1).wrapping_add(2452)) as i32)) != (l_i_4)) as i32)) != 0) && ((((({ let a_ = F::BtlMember_GetGauge(c, p_chr_1, l_i_4); (c.m.r32(a_) as i32) }) <= (0i32)) as i32)) != 0)) as i32)) != 0) {
                    l_slot_3 = l_i_4;
                    break 'l1;
                }
            }
            { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
        }
    }
    if (((((l_slot_3) < (0i32)) as i32)) != 0) {
        break 'f (1i32).wrapping_neg();
    }
    { let v_ = { let a1_ = (c.m.r32((p_chr_1).wrapping_add(2456)) as i32); F::BtlUtil_Max(c, (l_slot_3).wrapping_add(1i32), a1_) }; let a_ = (p_chr_1).wrapping_add(2456); c.m.w32(a_, (v_) as u32); };
    l_m_5 = ((p_chr_1).wrapping_add(2468)).wrapping_add((((l_slot_3) as u32)).wrapping_mul(164));
    { let _ = F::memset(c, l_m_5, 0i32, 164u32); };
    { let v_ = 1i32; let a_ = (l_m_5).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
    { let v_ = (c.m.r32(p_src_2) as i32); let a_ = l_m_5; c.m.w32(a_, (v_) as u32); };
    { let v_ = (c.m.r32((p_src_2).wrapping_add(4)) as i32); let a_ = (l_m_5).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
    { let v_ = (c.m.r32((p_src_2).wrapping_add(12)) as i32); let a_ = (l_m_5).wrapping_add(56); c.m.w32(a_, (v_) as u32); };
    { let v_ = (c.m.r32((p_src_2).wrapping_add(68)) as i32); let a_ = (l_m_5).wrapping_add(60); c.m.w32(a_, (v_) as u32); };
    {
        l_i_4 = 0i32;
        'l3: loop {
            if !(((((l_i_4) < (4i32)) as i32)) != 0) { break 'l3; }
            'c4: {
                { let v_ = (c.m.r32(((p_src_2).wrapping_add(72)).wrapping_add((((l_i_4) as u32)).wrapping_mul(4))) as i32); let a_ = ((l_m_5).wrapping_add(40)).wrapping_add((((l_i_4) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            }
            { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
        }
    }
    { let a3_ = F32(c.m.r32((p_src_2).wrapping_add(16))); let a4_ = (c.m.r32((p_src_2).wrapping_add(8)) as i32); F::BtlMember_LoadParams(c, p_chr_1, l_slot_3, 1i32, a3_, a4_) };
    { let v_ = (c.m.r32(((p_src_2).wrapping_add(36)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32); let a_ = ((l_m_5).wrapping_add(12)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    { let v_ = (c.m.r32(((p_src_2).wrapping_add(36)).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32); let a_ = ((l_m_5).wrapping_add(12)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    { let v_ = (c.m.r32(((p_src_2).wrapping_add(36)).wrapping_add((((3i32) as u32)).wrapping_mul(4))) as i32); let a_ = ((l_m_5).wrapping_add(12)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    { let v_ = (c.m.r32(((p_src_2).wrapping_add(36)).wrapping_add((((4i32) as u32)).wrapping_mul(4))) as i32); let a_ = ((l_m_5).wrapping_add(12)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    { let v_ = (c.m.r32(((p_src_2).wrapping_add(36)).wrapping_add((((5i32) as u32)).wrapping_mul(4))) as i32); let a_ = ((l_m_5).wrapping_add(12)).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    { let v_ = (c.m.r32(((p_src_2).wrapping_add(36)).wrapping_add((((6i32) as u32)).wrapping_mul(4))) as i32); let a_ = ((l_m_5).wrapping_add(12)).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    { let v_ = (c.m.r32(((p_src_2).wrapping_add(36)).wrapping_add((((7i32) as u32)).wrapping_mul(4))) as i32); let a_ = ((l_m_5).wrapping_add(12)).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    break 'f l_slot_3;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMember_Replace", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMember_FindIndex(c: &mut Ctx, mut p_chr_1: u32, mut p_chara_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMember_FindIndex", &PROBE_N, &[(p_chr_1) as u32, (p_chara_2) as u32], false); }
    let mut l_i_3: i32 = 0;
    let r_ = 'f: {
    {
        l_i_3 = 0i32;
        'l1: loop {
            if !(((((l_i_3) < ((c.m.r32((p_chr_1).wrapping_add(2456)) as i32))) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((((({ let a_ = F::BtlMember_Get(c, p_chr_1, l_i_3); (c.m.r32(a_) as i32) }) == (p_chara_2)) as i32)) != 0) {
                    break 'f l_i_3;
                }
            }
            { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
        }
    }
    break 'f (1i32).wrapping_neg();
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMember_FindIndex", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMember_SetActiveIndex(c: &mut Ctx, mut p_chr_1: u32, mut p_n_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMember_SetActiveIndex", &PROBE_N, &[(p_chr_1) as u32, (p_n_2) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_n_2; let a_ = (p_chr_1).wrapping_add(2452); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMember_SetActiveIndex", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMember_CountAlive(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMember_CountAlive", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_n_2: i32 = 0;
    let mut l_i_3: i32 = 0;
    let r_ = 'f: {
    l_n_2 = 0i32;
    {
        l_i_3 = 0i32;
        'l1: loop {
            if !(((((l_i_3) < ((c.m.r32((p_chr_1).wrapping_add(2456)) as i32))) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((((({ let a_ = (F::BtlMember_Get(c, p_chr_1, l_i_3)).wrapping_add(8); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
                    if ((((({ let a_ = F::BtlMember_GetGauge(c, p_chr_1, l_i_3); (c.m.r32(a_) as i32) }) > (0i32)) as i32)) != 0) {
                        { let o_ = l_n_2; let n_ = (o_).wrapping_add(1); l_n_2 = n_; };
                    }
                }
            }
            { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
        }
    }
    break 'f l_n_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMember_CountAlive", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMember_GetActiveIndex(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMember_GetActiveIndex", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f (c.m.r32((p_chr_1).wrapping_add(2452)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMember_GetActiveIndex", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMember_GetSwitchTarget(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMember_GetSwitchTarget", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    if ((((({ let a_ = { let a1_ = (c.m.r32((p_chr_1).wrapping_add(2464)) as i32); F::BtlMember_GetGauge(c, p_chr_1, a1_) }; (c.m.r32(a_) as i32) }) <= (0i32)) as i32)) != 0) {
        F::BtlMember_NextSwitchTarget(c, p_chr_1);
    }
    if ((((((c.m.r32((p_chr_1).wrapping_add(2464)) as i32)) == ((c.m.r32((p_chr_1).wrapping_add(2452)) as i32))) as i32)) != 0) {
        F::BtlMember_NextSwitchTarget(c, p_chr_1);
    }
    break 'f (c.m.r32((p_chr_1).wrapping_add(2464)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMember_GetSwitchTarget", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMember_SetSwitchTarget(c: &mut Ctx, mut p_chr_1: u32, mut p_n_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMember_SetSwitchTarget", &PROBE_N, &[(p_chr_1) as u32, (p_n_2) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_n_2; let a_ = (p_chr_1).wrapping_add(2464); c.m.w32(a_, (v_) as u32); };
    break 'f (((p_n_2) == (F::BtlMember_GetSwitchTarget(c, p_chr_1))) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMember_SetSwitchTarget", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMember_NextSwitchTarget(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMember_NextSwitchTarget", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_i_2: i32 = 0;
    let mut l_n_3: i32 = 0;
    let r_ = 'f: {
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < ((c.m.r32((p_chr_1).wrapping_add(2456)) as i32))) as i32)) != 0) { break 'l1; }
            'c2: {
                l_n_3 = rt::rem_i32((((c.m.r32((p_chr_1).wrapping_add(2464)) as i32)).wrapping_add(l_i_2)).wrapping_add(1i32), (c.m.r32((p_chr_1).wrapping_add(2456)) as i32));
                if (((((((((l_n_3) != ((c.m.r32((p_chr_1).wrapping_add(2452)) as i32))) as i32)) != 0) && ((((({ let a_ = F::BtlMember_GetGauge(c, p_chr_1, l_n_3); (c.m.r32(a_) as i32) }) > (0i32)) as i32)) != 0)) as i32)) != 0) {
                    { let v_ = l_n_3; let a_ = (p_chr_1).wrapping_add(2464); c.m.w32(a_, (v_) as u32); };
                    break 'f;
                }
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMember_NextSwitchTarget", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMember_PrevSwitchTarget(c: &mut Ctx, mut p_chr_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMember_PrevSwitchTarget", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_i_2: i32 = 0;
    let mut l_n_3: i32 = 0;
    let r_ = 'f: {
    {
        l_i_2 = ((c.m.r32((p_chr_1).wrapping_add(2456)) as i32)).wrapping_sub(1i32);
        'l1: loop {
            if !(((((l_i_2) >= (0i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                l_n_3 = rt::rem_i32(((c.m.r32((p_chr_1).wrapping_add(2464)) as i32)).wrapping_add(l_i_2), (c.m.r32((p_chr_1).wrapping_add(2456)) as i32));
                if (((((((((l_n_3) != ((c.m.r32((p_chr_1).wrapping_add(2452)) as i32))) as i32)) != 0) && ((((({ let a_ = F::BtlMember_GetGauge(c, p_chr_1, l_n_3); (c.m.r32(a_) as i32) }) > (0i32)) as i32)) != 0)) as i32)) != 0) {
                    { let v_ = l_n_3; let a_ = (p_chr_1).wrapping_add(2464); c.m.w32(a_, (v_) as u32); };
                    break 'f;
                }
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_sub(1); l_i_2 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMember_PrevSwitchTarget", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMember_Damage(c: &mut Ctx, mut p_chr_1: u32, mut p_amount_2: i32, mut p_flags_3: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMember_Damage", &PROBE_N, &[(p_chr_1) as u32, (p_amount_2) as u32, (p_flags_3) as u32], false); }
    let mut l_combo_4: u32 = 0;
    let mut l_g_5: u32 = 0;
    let mut l_before_6: i32 = 0;
    let mut l_health_7: i32 = 0;
    let mut l_n_8: i32 = 0;
    let mut l_pct_9: i32 = 0;
    let mut l_second_10: i32 = 0;
    let mut l_v_11: i32 = 0;
    let mut l_r_12: i32 = 0;
    let mut l_other_13: i32 = 0;
    let r_ = 'f: {
    l_combo_4 = (p_chr_1).wrapping_add(3392);
    l_g_5 = F::BtlMember_GetActiveGauge(c, p_chr_1);
    l_before_6 = (c.m.r32(l_g_5) as i32);
    if ((((!((((p_flags_3) & (128i32))) != 0)) as i32)) != 0) {
        if (((({ let a0_ = F::BtlAnim_GetId(c, p_chr_1); ((((F::BtlAnim_GetFlags(c, ((((a0_) as i64 as u64)) as u32))) as u64)) as i32) }) & (2048i32))) != 0) {
            { let r_ = 2i32; let n_ = rt::div_i32(p_amount_2, r_); p_amount_2 = n_; };
        }
    }
    if ((((!((((p_flags_3) & (256i32))) != 0)) as i32)) != 0) {
        l_n_8 = ((c.m.r32((l_combo_4).wrapping_add(4)) as i32)).wrapping_sub(4i32);
        if (((((l_n_8) > (0i32)) as i32)) != 0) {
            l_pct_9 = F::BtlUtil_Max(c, (100i32).wrapping_sub((l_n_8).wrapping_mul(3i32)), 50i32);
            if ((F::BtlOpp_HasAbility(c, p_chr_1, 93i32)) != 0) {
                l_pct_9 = F::BtlUtil_Max(c, (100i32).wrapping_sub((l_n_8).wrapping_mul(2i32)), 70i32);
            }
            p_amount_2 = rt::div_i32((p_amount_2).wrapping_mul(l_pct_9), 100i32);
        }
    }
    if ((((!((((p_flags_3) & (1i32))) != 0)) as i32)) != 0) {
        l_second_10 = 1i32;
        l_v_11 = rt::f2i(rt::fmul(rt::i2f(p_amount_2), F::BtlParam_GetDamageTakenScale(c, p_chr_1)));
        if (((((((((l_v_11) <= (0i32)) as i32)) != 0) && (((((p_amount_2) > (0i32)) as i32)) != 0)) as i32)) != 0) {
            p_amount_2 = 1i32;
        } else {
            p_amount_2 = l_v_11;
        }
        if ((((p_flags_3) & (58720256i32))) != 0) {
            l_second_10 = (((F::BtlOpp_HasAbility(c, p_chr_1, 76i32)) == (0i32)) as i32);
        }
        if ((l_second_10) != 0) {
            l_v_11 = rt::f2i(rt::fmul(rt::i2f(p_amount_2), F::BtlStat_GetScale7(c, p_chr_1)));
            if (((((((((l_v_11) <= (0i32)) as i32)) != 0) && (((((p_amount_2) > (0i32)) as i32)) != 0)) as i32)) != 0) {
                p_amount_2 = 1i32;
            } else {
                p_amount_2 = l_v_11;
            }
        }
    }
    if ((((p_flags_3) & (4i32))) != 0) {
        if ((F::BtlMember_HasAbility(c, p_chr_1, 63i32)) != 0) {
            p_amount_2 = 0i32;
        } else {
            if ((F::BtlMember_HasAbility(c, p_chr_1, 62i32)) != 0) {
                { let r_ = 2i32; let n_ = rt::div_i32(p_amount_2, r_); p_amount_2 = n_; };
            }
        }
    }
    if ((F::BtlMember_HasAbility(c, p_chr_1, 118i32)) != 0) {
        p_amount_2 = 0i32;
        { let r_ = 512i32; let n_ = ((p_flags_3) | (r_)); p_flags_3 = n_; };
    }
    if (((((p_amount_2) <= (0i32)) as i32)) != 0) {
        if ((((!((((p_flags_3) & (512i32))) != 0)) as i32)) != 0) {
            break 'f 0i32;
        }
        p_amount_2 = 0i32;
    }
    if ((((((((!((F::BtlChar_TestFlag(c, p_chr_1, ((((3i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) && ((((!((((p_flags_3) & (64i32))) != 0)) as i32)) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((((((!((((p_flags_3) & (2i32))) != 0)) as i32)) != 0) && (((((p_amount_2) >= (10i32)) as i32)) != 0)) as i32)) != 0) {
        l_r_12 = rt::rem_i32(p_amount_2, 10i32);
        if (((((l_r_12) != (0i32)) as i32)) != 0) {
            p_amount_2 = ((p_amount_2).wrapping_sub(l_r_12)).wrapping_add(10i32);
        }
    }
    if (((((((((F::Battle_GetMode(c)) == (5i32)) as i32)) != 0) || (((((F::Battle_GetMode(c)) == (6i32)) as i32)) != 0)) as i32)) != 0) {
        l_health_7 = (c.m.r32(l_g_5) as i32);
        if (((((p_amount_2) > ((l_health_7).wrapping_sub(1i32))) as i32)) != 0) {
            p_amount_2 = (l_health_7).wrapping_sub(1i32);
        }
    } else {
        l_health_7 = (c.m.r32(l_g_5) as i32);
    }
    if ((((p_flags_3) & (4i32))) != 0) {
        if (((((p_amount_2) > ((l_health_7).wrapping_sub(1i32))) as i32)) != 0) {
            p_amount_2 = (l_health_7).wrapping_sub(1i32);
        }
    }
    if ((((p_flags_3) & (1024i32))) != 0) {
        if (((((p_amount_2) > ((l_health_7).wrapping_sub(1i32))) as i32)) != 0) {
            p_amount_2 = (l_health_7).wrapping_sub(1i32);
        }
    }
    { let v_ = (l_health_7).wrapping_sub(p_amount_2); let a_ = l_g_5; c.m.w32(a_, (v_) as u32); };
    if (((((((((p_amount_2) > (0i32)) as i32)) != 0) || ((((p_flags_3) & (512i32))) != 0)) as i32)) != 0) {
        if ((((p_flags_3) & (8i32))) != 0) {
            if ((((!((((p_flags_3) & (16i32))) != 0)) as i32)) != 0) {
                { let r_ = p_amount_2; let a_ = l_combo_4; let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                if ((((!((((p_flags_3) & (32i32))) != 0)) as i32)) != 0) {
                    { let v_ = (rt::f2i(rt::fmul(rt::fmul(rt::i2f((c.m.r32(l_combo_4) as i32)), F32(0x41f00000)), F32(0x38d1b717)))).wrapping_add(30i32); let a_ = (l_combo_4).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    if ((((((c.m.r32((l_combo_4).wrapping_add(8)) as i32)) > (90i32)) as i32)) != 0) {
                        { let v_ = 90i32; let a_ = (l_combo_4).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    }
                    { let v_ = 1i32; let a_ = (l_combo_4).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
                }
            }
        } else {
            { let a_ = (l_combo_4).wrapping_add(4); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
            if ((((((c.m.r32((l_combo_4).wrapping_add(4)) as i32)) == (1i32)) as i32)) != 0) {
                { let v_ = p_amount_2; let a_ = l_combo_4; c.m.w32(a_, (v_) as u32); };
            } else {
                { let r_ = p_amount_2; let a_ = l_combo_4; let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
            }
            { let v_ = (rt::f2i(rt::fmul(rt::fmul(rt::i2f((c.m.r32(l_combo_4) as i32)), F32(0x41f00000)), F32(0x38d1b717)))).wrapping_add(30i32); let a_ = (l_combo_4).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
            if ((((((c.m.r32((l_combo_4).wrapping_add(8)) as i32)) > (90i32)) as i32)) != 0) {
                { let v_ = 90i32; let a_ = (l_combo_4).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
            }
            { let v_ = 1i32; let a_ = (l_combo_4).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
            { let v_ = 1i32; let a_ = (l_combo_4).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
        }
    }
    { let v_ = (((p_flags_3).wrapping_shr((2i32) as u32)) & (1i32)); let a_ = (l_combo_4).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
    l_health_7 = (c.m.r32(l_g_5) as i32);
    if (((((l_health_7) < (0i32)) as i32)) != 0) {
        { let v_ = 0i32; let a_ = l_g_5; c.m.w32(a_, (v_) as u32); };
        l_health_7 = 0i32;
    }
    if (((((((((l_before_6) > (0i32)) as i32)) != 0) && (((((l_health_7) == (0i32)) as i32)) != 0)) as i32)) != 0) {
        l_other_13 = F::BtlOpp_GetPlayer(c, p_chr_1);
        { let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlEvent_Raise(c, a0_, 74i32) };
        F::BtlEvent_Raise(c, l_other_13, 72i32);
        if ((((p_flags_3) & (2097152i32))) != 0) {
            F::BtlEvent_Raise(c, l_other_13, 83i32);
        }
        if ((((p_flags_3) & (4194304i32))) != 0) {
            F::BtlEvent_Raise(c, l_other_13, 83i32);
        }
        if ((((p_flags_3) & (8388608i32))) != 0) {
            F::BtlEvent_Raise(c, l_other_13, 82i32);
        }
        if ((((p_flags_3) & (16777216i32))) != 0) {
            F::BtlEvent_Raise(c, l_other_13, 82i32);
        }
        if ((((p_flags_3) & (33554432i32))) != 0) {
            F::BtlEvent_Raise(c, l_other_13, 81i32);
        }
    }
    if (((((p_amount_2) > (0i32)) as i32)) != 0) {
        { let a0_ = (c.m.r32(p_chr_1) as i32); F::BtlEvent_Raise(c, a0_, 89i32) };
    }
    if ((((((c.m.r32(l_g_5) as i32)) == (0i32)) as i32)) != 0) {
        { let v_ = 0i32; let a_ = (l_g_5).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (l_g_5).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (l_g_5).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (p_chr_1).wrapping_add(4064); c.m.w32(a_, (v_) as u32); };
        F::BtlChar_ClearFlag(c, p_chr_1, ((((190i32) as i64 as u64)) as u32));
        if (((((F::Battle_GetMode(c)) == (3i32)) as i32)) != 0) {
            if (((((F::BtlMember_CountAlive(c, p_chr_1)) <= (0i32)) as i32)) != 0) {
                F::BtlColl_NextPoolMember(c, p_chr_1);
            }
        }
    }
    break 'f (l_before_6).wrapping_sub(l_health_7);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMember_Damage", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMember_AddHealth(c: &mut Ctx, mut p_chr_1: u32, mut p_amount_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMember_AddHealth", &PROBE_N, &[(p_chr_1) as u32, (p_amount_2) as u32], false); }
    let mut l_g_3: u32 = 0;
    let r_ = 'f: {
    l_g_3 = F::BtlMember_GetActiveGauge(c, p_chr_1);
    if (((((p_amount_2) > (0i32)) as i32)) != 0) {
        { let r_ = p_amount_2; let a_ = l_g_3; let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
        if ((((((c.m.r32(l_g_3) as i32)) > ((c.m.r32((l_g_3).wrapping_add(4)) as i32))) as i32)) != 0) {
            { let v_ = (c.m.r32((l_g_3).wrapping_add(4)) as i32); let a_ = l_g_3; c.m.w32(a_, (v_) as u32); };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMember_AddHealth", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMember_HasHealth(c: &mut Ctx, mut p_chr_1: u32, mut p_amount_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMember_HasHealth", &PROBE_N, &[(p_chr_1) as u32, (p_amount_2) as u32], false); }
    let r_ = 'f: {
    if ((((({ let a_ = F::BtlMember_GetActiveGauge(c, p_chr_1); (c.m.r32(a_) as i32) }) < (p_amount_2)) as i32)) != 0) {
        break 'f 0i32;
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMember_HasHealth", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMember_AddKi(c: &mut Ctx, mut p_chr_1: u32, mut p_amount_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMember_AddKi", &PROBE_N, &[(p_chr_1) as u32, (p_amount_2) as u32], false); }
    let mut l_g_3: u32 = 0;
    let mut l_over_4: i32 = 0;
    let r_ = 'f: {
    l_g_3 = F::BtlMember_GetActiveGauge(c, p_chr_1);
    l_over_4 = 0i32;
    if (((((p_amount_2) <= (0i32)) as i32)) != 0) {
        break 'f 0i32;
    }
    { let r_ = p_amount_2; let a_ = (l_g_3).wrapping_add(12); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
    if ((((((c.m.r32((l_g_3).wrapping_add(12)) as i32)) > ((c.m.r32((l_g_3).wrapping_add(16)) as i32))) as i32)) != 0) {
        l_over_4 = ((c.m.r32((l_g_3).wrapping_add(12)) as i32)).wrapping_sub((c.m.r32((l_g_3).wrapping_add(16)) as i32));
        { let v_ = (c.m.r32((l_g_3).wrapping_add(16)) as i32); let a_ = (l_g_3).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
    }
    break 'f l_over_4;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMember_AddKi", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMember_HasKi(c: &mut Ctx, mut p_chr_1: u32, mut p_amount_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMember_HasKi", &PROBE_N, &[(p_chr_1) as u32, (p_amount_2) as u32], false); }
    let r_ = 'f: {
    if ((((({ let a_ = (F::BtlMember_GetActiveGauge(c, p_chr_1)).wrapping_add(12); (c.m.r32(a_) as i32) }) < (p_amount_2)) as i32)) != 0) {
        break 'f 0i32;
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMember_HasKi", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMember_SpendKi(c: &mut Ctx, mut p_chr_1: u32, mut p_amount_2: i32, mut p_force_3: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMember_SpendKi", &PROBE_N, &[(p_chr_1) as u32, (p_amount_2) as u32, (p_force_3) as u32], false); }
    let mut l_g_4: u32 = 0;
    let mut l_ranOut_5: i32 = 0;
    let r_ = 'f: {
    l_g_4 = F::BtlMember_GetActiveGauge(c, p_chr_1);
    l_ranOut_5 = 0i32;
    if (((((((((p_force_3) == (0i32)) as i32)) != 0) && ((F::BtlChar_TestFlag(c, p_chr_1, ((((6i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    { let r_ = p_amount_2; let a_ = (l_g_4).wrapping_add(12); let n_ = ((c.m.r32(a_) as i32)).wrapping_sub(r_); c.m.w32(a_, (n_) as u32); };
    if ((((((c.m.r32((l_g_4).wrapping_add(12)) as i32)) < (0i32)) as i32)) != 0) {
        { let v_ = 0i32; let a_ = (l_g_4).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        l_ranOut_5 = 1i32;
    }
    { let v_ = 0i32; let a_ = (l_g_4).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
    break 'f l_ranOut_5;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMember_SpendKi", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMember_DrainKi(c: &mut Ctx, mut p_chr_1: u32, mut p_amount_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMember_DrainKi", &PROBE_N, &[(p_chr_1) as u32, (p_amount_2) as u32], false); }
    let mut l_g_3: u32 = 0;
    let mut l_before_4: i32 = 0;
    let r_ = 'f: {
    l_g_3 = F::BtlMember_GetActiveGauge(c, p_chr_1);
    if (((((p_amount_2) <= (0i32)) as i32)) != 0) {
        break 'f 0i32;
    }
    l_before_4 = (c.m.r32((l_g_3).wrapping_add(12)) as i32);
    { let v_ = 0i32; let a_ = (l_g_3).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
    { let v_ = (l_before_4).wrapping_sub(p_amount_2); let a_ = (l_g_3).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
    if ((((((c.m.r32((l_g_3).wrapping_add(12)) as i32)) < (0i32)) as i32)) != 0) {
        { let v_ = 0i32; let a_ = (l_g_3).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
    }
    break 'f (l_before_4).wrapping_sub((c.m.r32((l_g_3).wrapping_add(12)) as i32));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMember_DrainKi", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMember_AddBlast(c: &mut Ctx, mut p_chr_1: u32, mut p_amount_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMember_AddBlast", &PROBE_N, &[(p_chr_1) as u32, (p_amount_2) as u32], false); }
    let mut l_g_3: u32 = 0;
    let r_ = 'f: {
    l_g_3 = F::BtlMember_GetActiveGauge(c, p_chr_1);
    { let r_ = p_amount_2; let a_ = (l_g_3).wrapping_add(20); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
    if ((((((c.m.r32((l_g_3).wrapping_add(20)) as i32)) > ((c.m.r32((l_g_3).wrapping_add(24)) as i32))) as i32)) != 0) {
        { let v_ = (c.m.r32((l_g_3).wrapping_add(24)) as i32); let a_ = (l_g_3).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMember_AddBlast", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMember_HasBlast(c: &mut Ctx, mut p_chr_1: u32, mut p_amount_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMember_HasBlast", &PROBE_N, &[(p_chr_1) as u32, (p_amount_2) as u32], false); }
    let r_ = 'f: {
    if ((((({ let a_ = (F::BtlMember_GetActiveGauge(c, p_chr_1)).wrapping_add(20); (c.m.r32(a_) as i32) }) < (p_amount_2)) as i32)) != 0) {
        break 'f 0i32;
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMember_HasBlast", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMember_SubBlast(c: &mut Ctx, mut p_chr_1: u32, mut p_amount_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMember_SubBlast", &PROBE_N, &[(p_chr_1) as u32, (p_amount_2) as u32], false); }
    let mut l_g_3: u32 = 0;
    let r_ = 'f: {
    l_g_3 = F::BtlMember_GetActiveGauge(c, p_chr_1);
    { let r_ = p_amount_2; let a_ = (l_g_3).wrapping_add(20); let n_ = ((c.m.r32(a_) as i32)).wrapping_sub(r_); c.m.w32(a_, (n_) as u32); };
    if ((((((c.m.r32((l_g_3).wrapping_add(20)) as i32)) < (0i32)) as i32)) != 0) {
        { let v_ = 0i32; let a_ = (l_g_3).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMember_SubBlast", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMember_AddMaxPower(c: &mut Ctx, mut p_chr_1: u32, mut p_amount_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMember_AddMaxPower", &PROBE_N, &[(p_chr_1) as u32, (p_amount_2) as u32], false); }
    let mut l_g_3: u32 = 0;
    let r_ = 'f: {
    l_g_3 = F::BtlMember_GetActiveGauge(c, p_chr_1);
    { let r_ = p_amount_2; let a_ = (l_g_3).wrapping_add(28); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
    if ((((((c.m.r32((l_g_3).wrapping_add(28)) as i32)) > (30000i32)) as i32)) != 0) {
        { let v_ = 30000i32; let a_ = (l_g_3).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMember_AddMaxPower", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMember_SubMaxPower(c: &mut Ctx, mut p_chr_1: u32, mut p_amount_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMember_SubMaxPower", &PROBE_N, &[(p_chr_1) as u32, (p_amount_2) as u32], false); }
    let mut l_g_3: u32 = 0;
    let r_ = 'f: {
    l_g_3 = F::BtlMember_GetActiveGauge(c, p_chr_1);
    { let r_ = p_amount_2; let a_ = (l_g_3).wrapping_add(28); let n_ = ((c.m.r32(a_) as i32)).wrapping_sub(r_); c.m.w32(a_, (n_) as u32); };
    if ((((((c.m.r32((l_g_3).wrapping_add(28)) as i32)) < (0i32)) as i32)) != 0) {
        { let v_ = 0i32; let a_ = (l_g_3).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMember_SubMaxPower", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMember_SetMaxPower(c: &mut Ctx, mut p_chr_1: u32, mut p_value_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMember_SetMaxPower", &PROBE_N, &[(p_chr_1) as u32, (p_value_2) as u32], false); }
    let mut l_g_3: u32 = 0;
    let r_ = 'f: {
    l_g_3 = F::BtlMember_GetActiveGauge(c, p_chr_1);
    { let v_ = F::BtlUtil_Clamp(c, p_value_2, 0i32, 30000i32); let a_ = (l_g_3).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMember_SetMaxPower", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMember_GetHealthRatio(c: &mut Ctx, mut p_chr_1: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMember_GetHealthRatio", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_g_2: u32 = 0;
    let r_ = 'f: {
    l_g_2 = F::BtlMember_GetActiveGauge(c, p_chr_1);
    if ((((((c.m.r32((l_g_2).wrapping_add(4)) as i32)) > (0i32)) as i32)) != 0) {
        break 'f rt::fdiv(rt::i2f((c.m.r32(l_g_2) as i32)), rt::i2f((c.m.r32((l_g_2).wrapping_add(4)) as i32)));
    }
    break 'f F32(0x00000000);
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMember_GetHealthRatio", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMember_GetTeamHealthRatio(c: &mut Ctx, mut p_chr_1: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMember_GetTeamHealthRatio", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_max_2: i32 = 0;
    let mut l_cur_3: i32 = 0;
    let mut l_i_4: i32 = 0;
    let mut l_g_5: u32 = 0;
    let r_ = 'f: {
    l_max_2 = 0i32;
    l_cur_3 = 0i32;
    {
        l_i_4 = 0i32;
        'l1: loop {
            if !(((((l_i_4) < ((c.m.r32((p_chr_1).wrapping_add(2456)) as i32))) as i32)) != 0) { break 'l1; }
            'c2: {
                l_g_5 = F::BtlMember_GetGauge(c, p_chr_1, l_i_4);
                { let r_ = (c.m.r32(l_g_5) as i32); let n_ = (l_cur_3).wrapping_add(r_); l_cur_3 = n_; };
                { let r_ = (c.m.r32((l_g_5).wrapping_add(4)) as i32); let n_ = (l_max_2).wrapping_add(r_); l_max_2 = n_; };
            }
            { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
        }
    }
    if (((((l_max_2) > (0i32)) as i32)) != 0) {
        break 'f rt::fdiv(rt::i2f(l_cur_3), rt::i2f(l_max_2));
    }
    break 'f F32(0x00000000);
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMember_GetTeamHealthRatio", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMember_GetKiRatio(c: &mut Ctx, mut p_chr_1: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMember_GetKiRatio", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_g_2: u32 = 0;
    let r_ = 'f: {
    l_g_2 = F::BtlMember_GetActiveGauge(c, p_chr_1);
    if ((((((c.m.r32((l_g_2).wrapping_add(16)) as i32)) > (0i32)) as i32)) != 0) {
        break 'f rt::fdiv(rt::i2f((c.m.r32((l_g_2).wrapping_add(12)) as i32)), rt::i2f((c.m.r32((l_g_2).wrapping_add(16)) as i32)));
    }
    break 'f F32(0x00000000);
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMember_GetKiRatio", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMember_IsHealthFull(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMember_IsHealthFull", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_g_2: u32 = 0;
    let r_ = 'f: {
    l_g_2 = F::BtlMember_GetActiveGauge(c, p_chr_1);
    if ((((((c.m.r32(l_g_2) as i32)) < ((c.m.r32((l_g_2).wrapping_add(4)) as i32))) as i32)) != 0) {
        break 'f 0i32;
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMember_IsHealthFull", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMember_IsKiFull(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMember_IsKiFull", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_g_2: u32 = 0;
    let r_ = 'f: {
    l_g_2 = F::BtlMember_GetActiveGauge(c, p_chr_1);
    if ((((((c.m.r32((l_g_2).wrapping_add(12)) as i32)) < ((c.m.r32((l_g_2).wrapping_add(16)) as i32))) as i32)) != 0) {
        break 'f 0i32;
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMember_IsKiFull", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMember_IsMaxPowerFull(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMember_IsMaxPowerFull", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    if ((((({ let a_ = (F::BtlMember_GetActiveGauge(c, p_chr_1)).wrapping_add(28); (c.m.r32(a_) as i32) }) < (30000i32)) as i32)) != 0) {
        break 'f 0i32;
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMember_IsMaxPowerFull", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMember_IsHealthEmpty(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMember_IsHealthEmpty", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f ((({ let a_ = F::BtlMember_GetActiveGauge(c, p_chr_1); (c.m.r32(a_) as i32) }) <= (0i32)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMember_IsHealthEmpty", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMember_IsKiEmpty(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMember_IsKiEmpty", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f ((({ let a_ = (F::BtlMember_GetActiveGauge(c, p_chr_1)).wrapping_add(12); (c.m.r32(a_) as i32) }) <= (0i32)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMember_IsKiEmpty", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMember_IsMaxPowerEmpty(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMember_IsMaxPowerEmpty", &PROBE_N, &[(p_chr_1) as u32], false); }
    let r_ = 'f: {
    break 'f ((({ let a_ = (F::BtlMember_GetActiveGauge(c, p_chr_1)).wrapping_add(28); (c.m.r32(a_) as i32) }) <= (0i32)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMember_IsMaxPowerEmpty", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMember_HasAbility(c: &mut Ctx, mut p_chr_1: u32, mut p_n_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMember_HasAbility", &PROBE_N, &[(p_chr_1) as u32, (p_n_2) as u32], false); }
    let r_ = 'f: {
    break 'f { let a1_ = F::BtlMember_GetActiveIndex(c, p_chr_1); F::BtlMember_HasAbilityOf(c, p_chr_1, a1_, p_n_2) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMember_HasAbility", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMember_HasAbilityOf(c: &mut Ctx, mut p_chr_1: u32, mut p_member_2: i32, mut p_n_3: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMember_HasAbilityOf", &PROBE_N, &[(p_chr_1) as u32, (p_member_2) as u32, (p_n_3) as u32], false); }
    let mut l_m_4: u32 = 0;
    let mut l_mask_5: u32 = 0;
    let r_ = 'f: {
    l_m_4 = F::BtlMember_Get(c, p_chr_1, p_member_2);
    l_mask_5 = (((1i32).wrapping_shl((((p_n_3) & (31i32))) as u32)) as u32);
    break 'f ((((((((c.m.r32(((l_m_4).wrapping_add(40)).wrapping_add(((((p_n_3).wrapping_shr((5i32) as u32)) as u32)).wrapping_mul(4))) as i32)) as u32)) & (l_mask_5))) != (((0i32) as u32))) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMember_HasAbilityOf", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMember_HasAnyListedAbility(c: &mut Ctx, mut p_chr_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMember_HasAnyListedAbility", &PROBE_N, &[(p_chr_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
        rt::memset(c, fp, 0, 44);
        { let v_ = 77i32; c.m.w32(fp.wrapping_add(0), (v_) as u32); }
        { let v_ = 78i32; c.m.w32(fp.wrapping_add(4), (v_) as u32); }
        { let v_ = 79i32; c.m.w32(fp.wrapping_add(8), (v_) as u32); }
        { let v_ = 80i32; c.m.w32(fp.wrapping_add(12), (v_) as u32); }
        { let v_ = 81i32; c.m.w32(fp.wrapping_add(16), (v_) as u32); }
        { let v_ = 82i32; c.m.w32(fp.wrapping_add(20), (v_) as u32); }
        { let v_ = 83i32; c.m.w32(fp.wrapping_add(24), (v_) as u32); }
        { let v_ = 84i32; c.m.w32(fp.wrapping_add(28), (v_) as u32); }
        { let v_ = 85i32; c.m.w32(fp.wrapping_add(32), (v_) as u32); }
        { let v_ = 86i32; c.m.w32(fp.wrapping_add(36), (v_) as u32); }
        { let v_ = 116i32; c.m.w32(fp.wrapping_add(40), (v_) as u32); }
        {
            l_i_2 = 0i32;
            'l1: loop {
                if !(((((l_i_2) < (11i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    if (({ let a1_ = (c.m.r32((fp).wrapping_add((((l_i_2) as u32)).wrapping_mul(4))) as i32); F::BtlMember_HasAbility(c, p_chr_1, a1_) }) != 0) {
                        break 'f 1i32;
                    }
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        break 'f 0i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlMember_HasAnyListedAbility", &PROBE_N, &[], true); }
    r_
}

pub fn BtlMembers_UpdateQueuedDamage(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlMembers_UpdateQueuedDamage", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let mut l_chr_2: u32 = 0;
    let mut l_other_3: u32 = 0;
    let mut l_active_4: i32 = 0;
    let mut l_frame_5: i32 = 0;
    let mut l_amount_6: i32 = 0;
    let mut l_flags_7: i32 = 0;
    let mut l_lost_8: i32 = 0;
    let mut l_amount_9: i32 = 0;
    let mut l_lost_10: i32 = 0;
    let r_ = 'f: {
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (F::BtlChar_GetCount(c))) as i32)) != 0) { break 'l1; }
            'c2: {
                l_chr_2 = F::BtlChar_Get(c, l_i_1);
                if ((F::BtlChar_IsFrozen(c, l_chr_2)) != 0) {
                    break 'c2;
                }
                l_other_3 = { let a0_ = F::BtlOpp_GetPlayer(c, l_chr_2); F::BtlChar_Get(c, a0_) };
                if ((F::BtlAnim_TestAttr(c, l_chr_2, ((8589934592i64) as u64))) != 0) {
                    if ((((((c.m.r32(((l_chr_2).wrapping_add(3476)).wrapping_add(12)) as i32)) >= (2i32)) as i32)) != 0) {
                        { let _ = { let a1_ = (c.m.r32(((l_chr_2).wrapping_add(3476)).wrapping_add(8)) as i32); let a2_ = (c.m.r32(((l_chr_2).wrapping_add(3476)).wrapping_add(16)) as i32); F::BtlMember_Damage(c, l_other_3, a1_, a2_) }; };
                        { let r_ = (c.m.r32(((l_chr_2).wrapping_add(3476)).wrapping_add(8)) as i32); let a_ = ((l_chr_2).wrapping_add(3476)).wrapping_add(4); let n_ = ((c.m.r32(a_) as i32)).wrapping_sub(r_); c.m.w32(a_, (n_) as u32); };
                        F::BtlChar_Vibrate(c, l_chr_2, F32(0x3f800000), F32(0x3dcccccc));
                        F::BtlChar_Vibrate(c, l_other_3, F32(0x3f800000), F32(0x3dcccccc));
                    } else {
                        { let _ = { let a1_ = (c.m.r32(((l_chr_2).wrapping_add(3476)).wrapping_add(4)) as i32); let a2_ = (c.m.r32(((l_chr_2).wrapping_add(3476)).wrapping_add(16)) as i32); F::BtlMember_Damage(c, l_other_3, a1_, a2_) }; };
                        { let v_ = 0i32; let a_ = ((l_chr_2).wrapping_add(3476)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                        F::BtlChar_Vibrate(c, l_chr_2, F32(0x3f800000), F32(0x3e999999));
                        F::BtlChar_Vibrate(c, l_other_3, F32(0x3f800000), F32(0x3e999999));
                    }
                    { let a_ = ((l_chr_2).wrapping_add(3476)).wrapping_add(12); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                    { let v_ = 30i32; let a_ = (l_other_3).wrapping_add(4068); c.m.w32(a_, (v_) as u32); };
                    if ((F::BtlChar_TestFlag(c, l_chr_2, ((((315i32) as i64 as u64)) as u32))) != 0) {
                        { let v_ = 0i32; let a_ = (l_other_3).wrapping_add(4068); c.m.w32(a_, (v_) as u32); };
                    }
                }
                l_active_4 = 0i32;
                l_frame_5 = 0i32;
                if (((((((({ let a0_ = F::BtlAnim_GetId(c, l_chr_2); ((((F::BtlAnim_GetFlags(c, ((((a0_) as i64 as u64)) as u32))) as u64)) as i32) }) & (8i32))) != 0) && ((((!(((({ let a0_ = F::BtlAnim_GetId(c, l_chr_2); ((((F::BtlAnim_GetFlags(c, ((((a0_) as i64 as u64)) as u32))) as u64)) as i32) }) & (16i32))) != 0)) as i32)) != 0)) as i32)) != 0) {
                    l_active_4 = 1i32;
                    l_frame_5 = rt::f2i(F::BtlAnim_GetFrame(c, l_chr_2));
                }
                if (((((({ let a0_ = (c.m.r32((l_chr_2).wrapping_add(12)) as i32); F::BtlCharApi_IsInTechnique(c, a0_) }) != 0) && (({ let a0_ = (c.m.r32((l_chr_2).wrapping_add(12)) as i32); F::BtlCharApi_IsInRushSequence(c, a0_) }) != 0)) as i32)) != 0) {
                    l_active_4 = 1i32;
                    l_frame_5 = rt::f2i({ let a0_ = (c.m.r32((l_chr_2).wrapping_add(12)) as i32); F::BtlCharApi_GetChargedUnkC78(c, a0_) });
                }
                if ((((((l_active_4) != 0) && ((((((c.m.r32(((l_chr_2).wrapping_add(3476)).wrapping_add(44)) as i32)) < (l_frame_5)) as i32)) != 0)) as i32)) != 0) {
                    if ((((((c.m.r32(((l_chr_2).wrapping_add(3476)).wrapping_add(24)) as i32)) > (0i32)) as i32)) != 0) {
                        if ((((((c.m.r32(((l_chr_2).wrapping_add(3476)).wrapping_add(28)) as i32)) < ((c.m.r32(((l_chr_2).wrapping_add(3476)).wrapping_add(24)) as i32))) as i32)) != 0) {
                            l_amount_6 = (c.m.r32(((l_chr_2).wrapping_add(3476)).wrapping_add(28)) as i32);
                            { let r_ = l_amount_6; let a_ = ((l_chr_2).wrapping_add(3476)).wrapping_add(24); let n_ = ((c.m.r32(a_) as i32)).wrapping_sub(r_); c.m.w32(a_, (n_) as u32); };
                        } else {
                            l_amount_6 = (c.m.r32(((l_chr_2).wrapping_add(3476)).wrapping_add(24)) as i32);
                            { let v_ = 0i32; let a_ = ((l_chr_2).wrapping_add(3476)).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                        }
                        l_flags_7 = (c.m.r32(((l_chr_2).wrapping_add(3476)).wrapping_add(16)) as i32);
                        if ((((((c.m.r32(((l_other_3).wrapping_add(3392)).wrapping_add(4)) as i32)) <= (0i32)) as i32)) != 0) {
                            { let r_ = ((1i32) | (2i32)); let n_ = ((l_flags_7) | (r_)); l_flags_7 = n_; };
                        } else {
                            { let r_ = ((((1i32) | (2i32))) | (8i32)); let n_ = ((l_flags_7) | (r_)); l_flags_7 = n_; };
                        }
                        l_lost_8 = F::BtlMember_Damage(c, l_other_3, l_amount_6, l_flags_7);
                        if ((((!((F::BtlChar_TestFlag(c, l_chr_2, ((((316i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) {
                            F::BtlMember_AddHealth(c, l_chr_2, l_lost_8);
                        }
                        { let v_ = 30i32; let a_ = (l_other_3).wrapping_add(4068); c.m.w32(a_, (v_) as u32); };
                    }
                    if ((((((c.m.r32(((l_chr_2).wrapping_add(3476)).wrapping_add(36)) as i32)) > (0i32)) as i32)) != 0) {
                        if ((((((c.m.r32(((l_chr_2).wrapping_add(3476)).wrapping_add(40)) as i32)) < ((c.m.r32(((l_chr_2).wrapping_add(3476)).wrapping_add(36)) as i32))) as i32)) != 0) {
                            l_amount_9 = (c.m.r32(((l_chr_2).wrapping_add(3476)).wrapping_add(40)) as i32);
                            { let r_ = l_amount_9; let a_ = ((l_chr_2).wrapping_add(3476)).wrapping_add(36); let n_ = ((c.m.r32(a_) as i32)).wrapping_sub(r_); c.m.w32(a_, (n_) as u32); };
                        } else {
                            l_amount_9 = (c.m.r32(((l_chr_2).wrapping_add(3476)).wrapping_add(36)) as i32);
                            { let v_ = 0i32; let a_ = ((l_chr_2).wrapping_add(3476)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                        }
                        l_lost_10 = F::BtlMember_DrainKi(c, l_other_3, l_amount_9);
                        if ((((!((F::BtlChar_TestFlag(c, l_chr_2, ((((316i32) as i64 as u64)) as u32))) != 0)) as i32)) != 0) {
                            { let _ = F::BtlMember_AddKi(c, l_chr_2, l_lost_10); };
                        }
                        { let v_ = 30i32; let a_ = (l_other_3).wrapping_add(4068); c.m.w32(a_, (v_) as u32); };
                    }
                }
                if ((F::BtlChar_TestFlag(c, l_chr_2, ((((140i32) as i64 as u64)) as u32))) != 0) {
                    if ((((((c.m.r32(((l_chr_2).wrapping_add(3476)).wrapping_add(4)) as i32)) > (0i32)) as i32)) != 0) {
                        { let _ = { let a1_ = (c.m.r32(((l_chr_2).wrapping_add(3476)).wrapping_add(4)) as i32); let a2_ = (c.m.r32(((l_chr_2).wrapping_add(3476)).wrapping_add(16)) as i32); F::BtlMember_Damage(c, l_other_3, a1_, a2_) }; };
                    }
                    { let v_ = 0i32; let a_ = (l_chr_2).wrapping_add(3476); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = ((l_chr_2).wrapping_add(3476)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = ((l_chr_2).wrapping_add(3476)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = ((l_chr_2).wrapping_add(3476)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = ((l_chr_2).wrapping_add(3476)).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = ((l_chr_2).wrapping_add(3476)).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = ((l_chr_2).wrapping_add(3476)).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = ((l_chr_2).wrapping_add(3476)).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = ((l_chr_2).wrapping_add(3476)).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = ((l_chr_2).wrapping_add(3476)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = ((l_chr_2).wrapping_add(3476)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = ((l_chr_2).wrapping_add(3476)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
                }
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlMembers_UpdateQueuedDamage", &PROBE_N, &[], true); }
    r_
}
