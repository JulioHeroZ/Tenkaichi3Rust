#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn vu0_a__f2u(c: &mut Ctx, p_f_1: F32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__f2u", &PROBE_N, &[(p_f_1).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    c.m.w32(fp, (p_f_1).0);
    let r_ = 'f: {
        { let _ = { rt::memcpy(c, fp.wrapping_add(4), fp, (((4i32) as u32)) as u32); fp.wrapping_add(4) }; };
        break 'f c.m.r32(fp.wrapping_add(4));
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__f2u", &PROBE_N, &[], true); }
    r_
}

pub fn vu0_a__u2f(c: &mut Ctx, p_u_1: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__u2f", &PROBE_N, &[(p_u_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    c.m.w32(fp, p_u_1);
    let r_ = 'f: {
        { let _ = { rt::memcpy(c, fp.wrapping_add(4), fp, (((4i32) as u32)) as u32); fp.wrapping_add(4) }; };
        break 'f F32(c.m.r32(fp.wrapping_add(4)));
       F32(0)
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__u2f", &PROBE_N, &[], true); }
    r_
}

pub fn vu0_a__in(c: &mut Ctx, mut p_a_1: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__in", &PROBE_N, &[(p_a_1) as u32], false); }
    let r_ = 'f: {
    break 'f (if ((((((((p_a_1).wrapping_shr((23i32) as u32)) & (((255i32) as u32)))) == (((255i32) as u32))) as i32)) != 0) { ((((p_a_1) & (2147483648u32))) | (2139095039u32)) } else { p_a_1 });
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__in", &PROBE_N, &[], true); }
    r_
}

pub fn vu0_a__pack(c: &mut Ctx, mut p_sign_1: u32, mut p_e_2: i32, mut p_m_3: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__pack", &PROBE_N, &[(p_sign_1) as u32, (p_e_2) as u32, (p_m_3) as u32], false); }
    let r_ = 'f: {
    if (((((p_e_2) >= (255i32)) as i32)) != 0) {
        break 'f ((p_sign_1) | (2139095039u32));
    }
    if (((((p_e_2) <= (0i32)) as i32)) != 0) {
        break 'f p_sign_1;
    }
    break 'f ((((p_sign_1) | ((((p_e_2) as u32)).wrapping_shl((23i32) as u32)))) | (((p_m_3) & (((8388607i32) as u32)))));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__pack", &PROBE_N, &[], true); }
    r_
}

pub fn vu0_a__round_pack(c: &mut Ctx, mut p_sign_1: u32, mut p_e_2: i32, mut p_m_3: u64, mut p_sticky_4: i32, mut p_nearest_5: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__round_pack", &PROBE_N, &[(p_sign_1) as u32, (p_e_2) as u32, (p_m_3) as u32, (p_sticky_4) as u32, (p_nearest_5) as u32], false); }
    let mut l_hi_6: u32 = 0;
    let mut l_lo_7: u32 = 0;
    let r_ = 'f: {
    l_hi_6 = (((p_m_3).wrapping_shr((32i32) as u32)) as u32);
    l_lo_7 = ((p_m_3) as u32);
    if ((((((((((p_nearest_5) != 0) && ((((l_lo_7) & (2147483648u32))) != 0)) as i32)) != 0) && ((((((((((((l_lo_7) & (2147483647u32))) != 0) || ((p_sticky_4) != 0)) as i32)) != 0) || ((((l_hi_6) & (((1i32) as u32)))) != 0)) as i32)) != 0)) as i32)) != 0) {
        { let o_ = l_hi_6; let n_ = (o_).wrapping_add(1); l_hi_6 = n_; };
        if (((((l_hi_6) == (((16777216i32) as u32))) as i32)) != 0) {
            { let r_ = 1i32; let n_ = (l_hi_6).wrapping_shr((r_) as u32); l_hi_6 = n_; };
            { let o_ = p_e_2; let n_ = (o_).wrapping_add(1); p_e_2 = n_; };
        }
    }
    break 'f F::vu0_a__pack(c, p_sign_1, p_e_2, l_hi_6);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__round_pack", &PROBE_N, &[], true); }
    r_
}

pub fn vu0_a__add_core(c: &mut Ctx, mut p_a_1: u32, mut p_b_2: u32, mut p_nearest_3: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__add_core", &PROBE_N, &[(p_a_1) as u32, (p_b_2) as u32, (p_nearest_3) as u32], false); }
    let mut l_ma_4: u64 = 0;
    let mut l_mb_5: u64 = 0;
    let mut l_sign_6: u32 = 0;
    let mut l_e_7: i32 = 0;
    let mut l_d_8: i32 = 0;
    let mut l_sticky_9: i32 = 0;
    let mut l_t_10: u32 = 0;
    let mut l_lost_11: u64 = 0;
    let r_ = 'f: {
    l_sticky_9 = 0i32;
    p_a_1 = F::vu0_a__in(c, p_a_1);
    p_b_2 = F::vu0_a__in(c, p_b_2);
    if ((((((((((((p_a_1).wrapping_shr((23i32) as u32)) & (((255i32) as u32)))) == (((0i32) as u32))) as i32)) != 0) && ((((((((p_b_2).wrapping_shr((23i32) as u32)) & (((255i32) as u32)))) == (((0i32) as u32))) as i32)) != 0)) as i32)) != 0) {
        break 'f ((((p_a_1) & (p_b_2))) & (2147483648u32));
    }
    if ((((((((p_b_2).wrapping_shr((23i32) as u32)) & (((255i32) as u32)))) == (((0i32) as u32))) as i32)) != 0) {
        break 'f p_a_1;
    }
    if ((((((((p_a_1).wrapping_shr((23i32) as u32)) & (((255i32) as u32)))) == (((0i32) as u32))) as i32)) != 0) {
        break 'f p_b_2;
    }
    if (((((((p_a_1) & (((2147483647i32) as u32)))) < (((p_b_2) & (((2147483647i32) as u32))))) as i32)) != 0) {
        l_t_10 = p_a_1;
        p_a_1 = p_b_2;
        p_b_2 = l_t_10;
    }
    l_sign_6 = ((p_a_1) & (2147483648u32));
    l_e_7 = (((((p_a_1).wrapping_shr((23i32) as u32)) & (((255i32) as u32)))) as i32);
    l_d_8 = (l_e_7).wrapping_sub((((((p_b_2).wrapping_shr((23i32) as u32)) & (((255i32) as u32)))) as i32));
    l_ma_4 = (((((((p_a_1) & (((8388607i32) as u32)))) | (((8388608i32) as u32)))) as u64)).wrapping_shl((32i32) as u32);
    l_mb_5 = (((((((p_b_2) & (((8388607i32) as u32)))) | (((8388608i32) as u32)))) as u64)).wrapping_shl((32i32) as u32);
    if ((((((p_a_1) ^ (p_b_2))) & (2147483648u32))) != 0) {
        l_lost_11 = (((((l_d_8) > (32i32)) as i32)) as u64);
        l_sticky_9 = ((l_lost_11) as i32);
        l_mb_5 = (if (((((l_d_8) >= (64i32)) as i32)) != 0) { ((0i32) as u64) } else { (l_mb_5).wrapping_shr((l_d_8) as u32) });
        { let r_ = (l_mb_5).wrapping_add(l_lost_11); let n_ = (l_ma_4).wrapping_sub(r_); l_ma_4 = n_; };
        if (((((l_ma_4) == (((0i32) as u64))) as i32)) != 0) {
            break 'f ((0i32) as u32);
        }
        'l1: while ((((!((((l_ma_4) & ((((1i32) as u64)).wrapping_shl((55i32) as u32)))) != 0)) as i32)) != 0) {
            { let r_ = 1i32; let n_ = (l_ma_4).wrapping_shl((r_) as u32); l_ma_4 = n_; };
            { let o_ = l_e_7; let n_ = (o_).wrapping_sub(1); l_e_7 = n_; };
        }
    } else {
        l_sticky_9 = (((l_d_8) > (32i32)) as i32);
        { let r_ = (if (((((l_d_8) >= (64i32)) as i32)) != 0) { ((0i32) as u64) } else { (l_mb_5).wrapping_shr((l_d_8) as u32) }); let n_ = (l_ma_4).wrapping_add(r_); l_ma_4 = n_; };
        if ((((l_ma_4) & ((((1i32) as u64)).wrapping_shl((56i32) as u32)))) != 0) {
            { let r_ = ((((l_ma_4) & (((1i32) as u64)))) as i32); let n_ = ((l_sticky_9) | (r_)); l_sticky_9 = n_; };
            { let r_ = 1i32; let n_ = (l_ma_4).wrapping_shr((r_) as u32); l_ma_4 = n_; };
            { let o_ = l_e_7; let n_ = (o_).wrapping_add(1); l_e_7 = n_; };
        }
    }
    break 'f F::vu0_a__round_pack(c, l_sign_6, l_e_7, l_ma_4, l_sticky_9, p_nearest_3);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__add_core", &PROBE_N, &[], true); }
    r_
}

pub fn vu0_a__mul_core(c: &mut Ctx, mut p_a_1: u32, mut p_b_2: u32, mut p_nearest_3: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__mul_core", &PROBE_N, &[(p_a_1) as u32, (p_b_2) as u32, (p_nearest_3) as u32], false); }
    let mut l_sign_4: u32 = 0;
    let mut l_m_5: u64 = 0;
    let mut l_e_6: i32 = 0;
    let r_ = 'f: {
    l_sign_4 = ((((p_a_1) ^ (p_b_2))) & (2147483648u32));
    p_a_1 = F::vu0_a__in(c, p_a_1);
    p_b_2 = F::vu0_a__in(c, p_b_2);
    if ((((((((((((p_a_1).wrapping_shr((23i32) as u32)) & (((255i32) as u32)))) == (((0i32) as u32))) as i32)) != 0) || ((((((((p_b_2).wrapping_shr((23i32) as u32)) & (((255i32) as u32)))) == (((0i32) as u32))) as i32)) != 0)) as i32)) != 0) {
        break 'f l_sign_4;
    }
    l_m_5 = (((((((p_a_1) & (((8388607i32) as u32)))) | (((8388608i32) as u32)))) as u64)).wrapping_mul(((((((p_b_2) & (((8388607i32) as u32)))) | (((8388608i32) as u32)))) as u64));
    l_e_6 = (((((((p_a_1).wrapping_shr((23i32) as u32)) & (((255i32) as u32)))) as i32)).wrapping_add((((((p_b_2).wrapping_shr((23i32) as u32)) & (((255i32) as u32)))) as i32))).wrapping_sub(127i32);
    if ((((l_m_5) & ((((1i32) as u64)).wrapping_shl((47i32) as u32)))) != 0) {
        { let o_ = l_e_6; let n_ = (o_).wrapping_add(1); l_e_6 = n_; };
        { let r_ = 8i32; let n_ = (l_m_5).wrapping_shl((r_) as u32); l_m_5 = n_; };
    } else {
        { let r_ = 9i32; let n_ = (l_m_5).wrapping_shl((r_) as u32); l_m_5 = n_; };
    }
    break 'f F::vu0_a__round_pack(c, l_sign_4, l_e_6, l_m_5, 0i32, p_nearest_3);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__mul_core", &PROBE_N, &[], true); }
    r_
}

pub fn vu0_a__mode(c: &mut Ctx, mut p_which_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__mode", &PROBE_N, &[(p_which_1) as u32], false); }
    let mut l_e_2: u32 = 0;
    let r_ = 'f: {
    if ((((((c.m.r32((sym::vu0_a__mode__m).wrapping_add((((p_which_1) as u32)).wrapping_mul(4))) as i32)) < (0i32)) as i32)) != 0) {
        l_e_2 = { let a0_ = c.m.r32((sym::vu0_a__mode__name).wrapping_add((((p_which_1) as u32)).wrapping_mul(4))); F::getenv(c, a0_) };
        { let v_ = (((((((l_e_2) != (((0i32) as u32))) as i32)) != 0) && ((((((((c.m.r8((l_e_2).wrapping_add(((0i32) as u32))) as i8)) as i32)) == (49i32)) as i32)) != 0)) as i32); let a_ = (sym::vu0_a__mode__m).wrapping_add((((p_which_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    }
    break 'f (c.m.r32((sym::vu0_a__mode__m).wrapping_add((((p_which_1) as u32)).wrapping_mul(4))) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__mode", &PROBE_N, &[], true); }
    r_
}

pub fn vu0_a__fpu_add(c: &mut Ctx, mut p_a_1: u32, mut p_b_2: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__fpu_add", &PROBE_N, &[(p_a_1) as u32, (p_b_2) as u32], false); }
    let mut l_d_3: i32 = 0;
    let r_ = 'f: {
    l_d_3 = ((((((p_a_1).wrapping_shr((23i32) as u32)) & (((255i32) as u32)))) as i32)).wrapping_sub((((((p_b_2).wrapping_shr((23i32) as u32)) & (((255i32) as u32)))) as i32));
    if ((F::vu0_a__mode(c, 3i32)) != 0) {
        break 'f { let a2_ = F::vu0_a__mode(c, 1i32); F::vu0_a__add_core(c, p_a_1, p_b_2, a2_) };
    }
    if (((((l_d_3) >= (25i32)) as i32)) != 0) {
        { let r_ = 2147483648u32; let n_ = ((p_b_2) & (r_)); p_b_2 = n_; };
    } else {
        if (((((l_d_3) > (0i32)) as i32)) != 0) {
            { let r_ = (4294967295u32).wrapping_shl(((l_d_3).wrapping_sub(1i32)) as u32); let n_ = ((p_b_2) & (r_)); p_b_2 = n_; };
        } else {
            if (((((l_d_3) <= ((25i32).wrapping_neg())) as i32)) != 0) {
                { let r_ = 2147483648u32; let n_ = ((p_a_1) & (r_)); p_a_1 = n_; };
            } else {
                if (((((l_d_3) < (0i32)) as i32)) != 0) {
                    { let r_ = (4294967295u32).wrapping_shl((((l_d_3).wrapping_neg()).wrapping_sub(1i32)) as u32); let n_ = ((p_a_1) & (r_)); p_a_1 = n_; };
                }
            }
        }
    }
    break 'f { let a2_ = F::vu0_a__mode(c, 1i32); F::vu0_a__add_core(c, p_a_1, p_b_2, a2_) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__fpu_add", &PROBE_N, &[], true); }
    r_
}

pub fn vu0_a__Sf_AddBits(c: &mut Ctx, mut p_a_1: u32, mut p_b_2: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__Sf_AddBits", &PROBE_N, &[(p_a_1) as u32, (p_b_2) as u32], false); }
    let r_ = 'f: {
    break 'f (if ((F::vu0_a__mode(c, 2i32)) != 0) { F::vu0_a__fpu_add(c, p_a_1, p_b_2) } else { { let a2_ = F::vu0_a__mode(c, 0i32); F::vu0_a__add_core(c, p_a_1, p_b_2, a2_) } });
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__Sf_AddBits", &PROBE_N, &[], true); }
    r_
}

pub fn vu0_a__Sf_MulBits(c: &mut Ctx, mut p_a_1: u32, mut p_b_2: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__Sf_MulBits", &PROBE_N, &[(p_a_1) as u32, (p_b_2) as u32], false); }
    let r_ = 'f: {
    break 'f { let a2_ = F::vu0_a__mode(c, 0i32); F::vu0_a__mul_core(c, p_a_1, p_b_2, a2_) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__Sf_MulBits", &PROBE_N, &[], true); }
    r_
}

pub fn RefVu0_SqrtBits(c: &mut Ctx, mut p_a_1: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "RefVu0_SqrtBits", &PROBE_N, &[(p_a_1) as u32], false); }
    let mut l_ea_2: u32 = 0;
    let mut l_n_3: u64 = 0;
    let mut l_r_4: u64 = 0;
    let mut l_bit_5: u64 = 0;
    let mut l_e_6: i32 = 0;
    let r_ = 'f: {
    l_ea_2 = (((p_a_1).wrapping_shr((23i32) as u32)) & (((255i32) as u32)));
    if (((((l_ea_2) == (((0i32) as u32))) as i32)) != 0) {
        break 'f ((0i32) as u32);
    }
    l_e_6 = (((l_ea_2) as i32)).wrapping_sub(127i32);
    l_n_3 = ((((((p_a_1) & (((8388607i32) as u32)))) | (((8388608i32) as u32)))) as u64);
    if ((((l_e_6) & (1i32))) != 0) {
        { let r_ = 1i32; let n_ = (l_n_3).wrapping_shl((r_) as u32); l_n_3 = n_; };
        { let r_ = 1i32; let n_ = (l_e_6).wrapping_sub(r_); l_e_6 = n_; };
    }
    { let r_ = 23i32; let n_ = (l_n_3).wrapping_shl((r_) as u32); l_n_3 = n_; };
    l_r_4 = ((0i32) as u64);
    {
        l_bit_5 = (((1i32) as u64)).wrapping_shl((46i32) as u32);
        'l1: loop {
            if !(((((l_bit_5) != (((0i32) as u64))) as i32)) != 0) { break 'l1; }
            'c2: {
                if (((((l_n_3) >= ((l_r_4).wrapping_add(l_bit_5))) as i32)) != 0) {
                    { let r_ = (l_r_4).wrapping_add(l_bit_5); let n_ = (l_n_3).wrapping_sub(r_); l_n_3 = n_; };
                    l_r_4 = ((l_r_4).wrapping_shr((1i32) as u32)).wrapping_add(l_bit_5);
                } else {
                    { let r_ = 1i32; let n_ = (l_r_4).wrapping_shr((r_) as u32); l_r_4 = n_; };
                }
            }
            { let r_ = 2i32; let n_ = (l_bit_5).wrapping_shr((r_) as u32); l_bit_5 = n_; };
        }
    }
    break 'f ((((((rt::div_i32(l_e_6, 2i32)).wrapping_add(127i32)) as u32)).wrapping_shl((23i32) as u32)) | (((((l_r_4) as u32)) & (((8388607i32) as u32)))));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "RefVu0_SqrtBits", &PROBE_N, &[], true); }
    r_
}

pub fn RefVu0_ItofBits(c: &mut Ctx, mut p_v_1: i32, mut p_fracBits_2: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "RefVu0_ItofBits", &PROBE_N, &[(p_v_1) as u32, (p_fracBits_2) as u32], false); }
    let mut l_sign_3: u32 = 0;
    let mut l_u_4: u32 = 0;
    let mut l_e_5: i32 = 0;
    let r_ = 'f: {
    l_sign_3 = (if (((((p_v_1) < (0i32)) as i32)) != 0) { 2147483648u32 } else { ((0i32) as u32) });
    l_u_4 = (if (((((p_v_1) < (0i32)) as i32)) != 0) { (0u32).wrapping_sub(((p_v_1) as u32)) } else { ((p_v_1) as u32) });
    l_e_5 = 31i32;
    if (((((l_u_4) == (((0i32) as u32))) as i32)) != 0) {
        break 'f ((0i32) as u32);
    }
    'l1: while ((((!((((l_u_4) & (2147483648u32))) != 0)) as i32)) != 0) {
        { let r_ = 1i32; let n_ = (l_u_4).wrapping_shl((r_) as u32); l_u_4 = n_; };
        { let o_ = l_e_5; let n_ = (o_).wrapping_sub(1); l_e_5 = n_; };
    }
    break 'f ((((l_sign_3) | ((((((l_e_5).wrapping_add(127i32)).wrapping_sub(p_fracBits_2)) as u32)).wrapping_shl((23i32) as u32)))) | ((((l_u_4).wrapping_shr((8i32) as u32)) & (((8388607i32) as u32)))));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "RefVu0_ItofBits", &PROBE_N, &[], true); }
    r_
}

pub fn RefVu0_SubBits(c: &mut Ctx, mut p_a_1: u32, mut p_b_2: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "RefVu0_SubBits", &PROBE_N, &[(p_a_1) as u32, (p_b_2) as u32], false); }
    let r_ = 'f: {
    break 'f F::vu0_a__Sf_AddBits(c, p_a_1, ((p_b_2) ^ (2147483648u32)));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "RefVu0_SubBits", &PROBE_N, &[], true); }
    r_
}

pub fn RefVu0_LtBits(c: &mut Ctx, mut p_a_1: u32, mut p_b_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "RefVu0_LtBits", &PROBE_N, &[(p_a_1) as u32, (p_b_2) as u32], false); }
    let mut l_x_3: i32 = 0;
    let mut l_y_4: i32 = 0;
    let r_ = 'f: {
    l_x_3 = (if ((((p_a_1) & (2147483648u32))) != 0) { (((((p_a_1) & (((2147483647i32) as u32)))) as i32)).wrapping_neg() } else { ((((p_a_1) & (((2147483647i32) as u32)))) as i32) });
    l_y_4 = (if ((((p_b_2) & (2147483648u32))) != 0) { (((((p_b_2) & (((2147483647i32) as u32)))) as i32)).wrapping_neg() } else { ((((p_b_2) & (((2147483647i32) as u32)))) as i32) });
    break 'f (((l_x_3) < (l_y_4)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "RefVu0_LtBits", &PROBE_N, &[], true); }
    r_
}

pub fn RefVu0_Add(c: &mut Ctx, mut p_a_1: F32, mut p_b_2: F32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "RefVu0_Add", &PROBE_N, &[(p_a_1).0, (p_b_2).0], false); }
    let r_ = 'f: {
    break 'f { let a0_ = { let a0_ = F::vu0_a__f2u(c, p_a_1); let a1_ = F::vu0_a__f2u(c, p_b_2); F::vu0_a__Sf_AddBits(c, a0_, a1_) }; F::vu0_a__u2f(c, a0_) };
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "RefVu0_Add", &PROBE_N, &[], true); }
    r_
}

pub fn RefVu0_Sub(c: &mut Ctx, mut p_a_1: F32, mut p_b_2: F32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "RefVu0_Sub", &PROBE_N, &[(p_a_1).0, (p_b_2).0], false); }
    let r_ = 'f: {
    break 'f { let a0_ = { let a0_ = F::vu0_a__f2u(c, p_a_1); let a1_ = F::vu0_a__f2u(c, p_b_2); F::RefVu0_SubBits(c, a0_, a1_) }; F::vu0_a__u2f(c, a0_) };
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "RefVu0_Sub", &PROBE_N, &[], true); }
    r_
}

pub fn RefVu0_Mul(c: &mut Ctx, mut p_a_1: F32, mut p_b_2: F32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "RefVu0_Mul", &PROBE_N, &[(p_a_1).0, (p_b_2).0], false); }
    let r_ = 'f: {
    break 'f { let a0_ = { let a0_ = F::vu0_a__f2u(c, p_a_1); let a1_ = F::vu0_a__f2u(c, p_b_2); F::vu0_a__Sf_MulBits(c, a0_, a1_) }; F::vu0_a__u2f(c, a0_) };
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "RefVu0_Mul", &PROBE_N, &[], true); }
    r_
}

pub fn RefVu0_Div(c: &mut Ctx, mut p_a_1: F32, mut p_b_2: F32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "RefVu0_Div", &PROBE_N, &[(p_a_1).0, (p_b_2).0], false); }
    let r_ = 'f: {
    break 'f { let a0_ = { let a0_ = F::vu0_a__f2u(c, p_a_1); let a1_ = F::vu0_a__f2u(c, p_b_2); F::RefVu0_DivBits(c, a0_, a1_) }; F::vu0_a__u2f(c, a0_) };
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "RefVu0_Div", &PROBE_N, &[], true); }
    r_
}

pub fn vu0_a__setFlags(c: &mut Ctx, mut p_r_1: u32, mut p_m_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__setFlags", &PROBE_N, &[(p_r_1) as u32, (p_m_2) as u32], false); }
    let mut l_cur_3: u32 = 0;
    let mut l_i_4: i32 = 0;
    let r_ = 'f: {
    l_cur_3 = ((0i32) as u32);
    {
        l_i_4 = 0i32;
        'l1: loop {
            if !(((((l_i_4) < (4i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((((p_m_2) & ((8i32).wrapping_shr((l_i_4) as u32)))) != 0) {
                    if (((((((c.m.r32((p_r_1).wrapping_add((((l_i_4) as u32)).wrapping_mul(4)))) & (((2147483647i32) as u32)))) == (((0i32) as u32))) as i32)) != 0) {
                        { let r_ = ((1i32) as u32); let n_ = ((l_cur_3) | (r_)); l_cur_3 = n_; };
                    }
                    if ((((c.m.r32((p_r_1).wrapping_add((((l_i_4) as u32)).wrapping_mul(4)))) & (2147483648u32))) != 0) {
                        { let r_ = ((2i32) as u32); let n_ = ((l_cur_3) | (r_)); l_cur_3 = n_; };
                    }
                }
            }
            { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
        }
    }
    { let v_ = ((((((c.m.r32((sym::gRefVu0).wrapping_add(564))) & (((4032i32) as u32)))) | (l_cur_3))) | ((l_cur_3).wrapping_shl((6i32) as u32))); let a_ = (sym::gRefVu0).wrapping_add(564); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__setFlags", &PROBE_N, &[], true); }
    r_
}

pub fn vu0_a__put(c: &mut Ctx, mut p_fd_1: i32, mut p_r_2: u32, mut p_m_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__put", &PROBE_N, &[(p_fd_1) as u32, (p_r_2) as u32, (p_m_3) as u32], false); }
    let mut l_i_4: i32 = 0;
    let r_ = 'f: {
    if (((((p_fd_1) == (0i32)) as i32)) != 0) {
        break 'f;
    }
    {
        l_i_4 = 0i32;
        'l1: loop {
            if !(((((l_i_4) < (4i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((((p_m_3) & ((8i32).wrapping_shr((l_i_4) as u32)))) != 0) {
                    { let v_ = c.m.r32((p_r_2).wrapping_add((((l_i_4) as u32)).wrapping_mul(4))); let a_ = ((sym::gRefVu0).wrapping_add((((p_fd_1) as u32)).wrapping_mul(16))).wrapping_add((((l_i_4) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                }
            }
            { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__put", &PROBE_N, &[], true); }
    r_
}

pub fn vu0_a__putAcc(c: &mut Ctx, mut p_r_1: u32, mut p_m_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__putAcc", &PROBE_N, &[(p_r_1) as u32, (p_m_2) as u32], false); }
    let mut l_i_3: i32 = 0;
    let r_ = 'f: {
    {
        l_i_3 = 0i32;
        'l1: loop {
            if !(((((l_i_3) < (4i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((((p_m_2) & ((8i32).wrapping_shr((l_i_3) as u32)))) != 0) {
                    { let v_ = c.m.r32((p_r_1).wrapping_add((((l_i_3) as u32)).wrapping_mul(4))); let a_ = ((sym::gRefVu0).wrapping_add(512)).wrapping_add((((l_i_3) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                }
            }
            { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__putAcc", &PROBE_N, &[], true); }
    r_
}

pub fn vu0_a__op_lqc2(c: &mut Ctx, mut p_ft_1: i32, mut p_p_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__op_lqc2", &PROBE_N, &[(p_ft_1) as u32, (p_p_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        { let _ = { rt::memcpy(c, fp, p_p_2, (((16i32) as u32)) as u32); fp }; };
        F::vu0_a__put(c, p_ft_1, fp, 15i32);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__op_lqc2", &PROBE_N, &[], true); }
    r_
}

pub fn vu0_a__op_sqc2(c: &mut Ctx, mut p_ft_1: i32, mut p_p_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__op_sqc2", &PROBE_N, &[(p_ft_1) as u32, (p_p_2) as u32], false); }
    let r_ = 'f: {
    { let _ = { rt::memcpy(c, p_p_2, (sym::gRefVu0).wrapping_add((((p_ft_1) as u32)).wrapping_mul(16)), (((16i32) as u32)) as u32); p_p_2 }; };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__op_sqc2", &PROBE_N, &[], true); }
    r_
}

pub fn vu0_a__op_vadd(c: &mut Ctx, mut p_m_1: i32, mut p_fd_2: i32, mut p_fs_3: i32, mut p_ft_4: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__op_vadd", &PROBE_N, &[(p_m_1) as u32, (p_fd_2) as u32, (p_fs_3) as u32, (p_ft_4) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_i_5: i32 = 0;
    let r_ = 'f: {
        {
            l_i_5 = 0i32;
            'l1: loop {
                if !(((((l_i_5) < (4i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let v_ = { let a0_ = c.m.r32(((sym::gRefVu0).wrapping_add((((p_fs_3) as u32)).wrapping_mul(16))).wrapping_add((((l_i_5) as u32)).wrapping_mul(4))); let a1_ = c.m.r32(((sym::gRefVu0).wrapping_add((((p_ft_4) as u32)).wrapping_mul(16))).wrapping_add((((l_i_5) as u32)).wrapping_mul(4))); F::vu0_a__Sf_AddBits(c, a0_, a1_) }; let a_ = (fp).wrapping_add((((l_i_5) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                }
                { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
            }
        }
        F::vu0_a__setFlags(c, fp, p_m_1);
        F::vu0_a__put(c, p_fd_2, fp, p_m_1);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__op_vadd", &PROBE_N, &[], true); }
    r_
}

pub fn vu0_a__op_vsub(c: &mut Ctx, mut p_m_1: i32, mut p_fd_2: i32, mut p_fs_3: i32, mut p_ft_4: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__op_vsub", &PROBE_N, &[(p_m_1) as u32, (p_fd_2) as u32, (p_fs_3) as u32, (p_ft_4) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_i_5: i32 = 0;
    let r_ = 'f: {
        {
            l_i_5 = 0i32;
            'l1: loop {
                if !(((((l_i_5) < (4i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let v_ = { let a0_ = c.m.r32(((sym::gRefVu0).wrapping_add((((p_fs_3) as u32)).wrapping_mul(16))).wrapping_add((((l_i_5) as u32)).wrapping_mul(4))); let a1_ = c.m.r32(((sym::gRefVu0).wrapping_add((((p_ft_4) as u32)).wrapping_mul(16))).wrapping_add((((l_i_5) as u32)).wrapping_mul(4))); F::RefVu0_SubBits(c, a0_, a1_) }; let a_ = (fp).wrapping_add((((l_i_5) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                }
                { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
            }
        }
        F::vu0_a__setFlags(c, fp, p_m_1);
        F::vu0_a__put(c, p_fd_2, fp, p_m_1);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__op_vsub", &PROBE_N, &[], true); }
    r_
}

pub fn vu0_a__op_vmul(c: &mut Ctx, mut p_m_1: i32, mut p_fd_2: i32, mut p_fs_3: i32, mut p_ft_4: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__op_vmul", &PROBE_N, &[(p_m_1) as u32, (p_fd_2) as u32, (p_fs_3) as u32, (p_ft_4) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_i_5: i32 = 0;
    let r_ = 'f: {
        {
            l_i_5 = 0i32;
            'l1: loop {
                if !(((((l_i_5) < (4i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let v_ = { let a0_ = c.m.r32(((sym::gRefVu0).wrapping_add((((p_fs_3) as u32)).wrapping_mul(16))).wrapping_add((((l_i_5) as u32)).wrapping_mul(4))); let a1_ = c.m.r32(((sym::gRefVu0).wrapping_add((((p_ft_4) as u32)).wrapping_mul(16))).wrapping_add((((l_i_5) as u32)).wrapping_mul(4))); F::vu0_a__Sf_MulBits(c, a0_, a1_) }; let a_ = (fp).wrapping_add((((l_i_5) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                }
                { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
            }
        }
        F::vu0_a__setFlags(c, fp, p_m_1);
        F::vu0_a__put(c, p_fd_2, fp, p_m_1);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__op_vmul", &PROBE_N, &[], true); }
    r_
}

pub fn vu0_a__op_vaddbc(c: &mut Ctx, mut p_m_1: i32, mut p_fd_2: i32, mut p_fs_3: i32, mut p_ft_4: i32, mut p_c_5: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__op_vaddbc", &PROBE_N, &[(p_m_1) as u32, (p_fd_2) as u32, (p_fs_3) as u32, (p_ft_4) as u32, (p_c_5) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_b_6: u32 = 0;
    let mut l_i_7: i32 = 0;
    let r_ = 'f: {
        l_b_6 = c.m.r32(((sym::gRefVu0).wrapping_add((((p_ft_4) as u32)).wrapping_mul(16))).wrapping_add((((p_c_5) as u32)).wrapping_mul(4)));
        {
            l_i_7 = 0i32;
            'l1: loop {
                if !(((((l_i_7) < (4i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let v_ = { let a0_ = c.m.r32(((sym::gRefVu0).wrapping_add((((p_fs_3) as u32)).wrapping_mul(16))).wrapping_add((((l_i_7) as u32)).wrapping_mul(4))); F::vu0_a__Sf_AddBits(c, a0_, l_b_6) }; let a_ = (fp).wrapping_add((((l_i_7) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                }
                { let o_ = l_i_7; let n_ = (o_).wrapping_add(1); l_i_7 = n_; };
            }
        }
        F::vu0_a__setFlags(c, fp, p_m_1);
        F::vu0_a__put(c, p_fd_2, fp, p_m_1);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__op_vaddbc", &PROBE_N, &[], true); }
    r_
}

pub fn vu0_a__op_vsubbc(c: &mut Ctx, mut p_m_1: i32, mut p_fd_2: i32, mut p_fs_3: i32, mut p_ft_4: i32, mut p_c_5: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__op_vsubbc", &PROBE_N, &[(p_m_1) as u32, (p_fd_2) as u32, (p_fs_3) as u32, (p_ft_4) as u32, (p_c_5) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_b_6: u32 = 0;
    let mut l_i_7: i32 = 0;
    let r_ = 'f: {
        l_b_6 = c.m.r32(((sym::gRefVu0).wrapping_add((((p_ft_4) as u32)).wrapping_mul(16))).wrapping_add((((p_c_5) as u32)).wrapping_mul(4)));
        {
            l_i_7 = 0i32;
            'l1: loop {
                if !(((((l_i_7) < (4i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let v_ = { let a0_ = c.m.r32(((sym::gRefVu0).wrapping_add((((p_fs_3) as u32)).wrapping_mul(16))).wrapping_add((((l_i_7) as u32)).wrapping_mul(4))); F::RefVu0_SubBits(c, a0_, l_b_6) }; let a_ = (fp).wrapping_add((((l_i_7) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                }
                { let o_ = l_i_7; let n_ = (o_).wrapping_add(1); l_i_7 = n_; };
            }
        }
        F::vu0_a__setFlags(c, fp, p_m_1);
        F::vu0_a__put(c, p_fd_2, fp, p_m_1);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__op_vsubbc", &PROBE_N, &[], true); }
    r_
}

pub fn vu0_a__op_vmulbc(c: &mut Ctx, mut p_m_1: i32, mut p_fd_2: i32, mut p_fs_3: i32, mut p_ft_4: i32, mut p_c_5: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__op_vmulbc", &PROBE_N, &[(p_m_1) as u32, (p_fd_2) as u32, (p_fs_3) as u32, (p_ft_4) as u32, (p_c_5) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_b_6: u32 = 0;
    let mut l_i_7: i32 = 0;
    let r_ = 'f: {
        l_b_6 = c.m.r32(((sym::gRefVu0).wrapping_add((((p_ft_4) as u32)).wrapping_mul(16))).wrapping_add((((p_c_5) as u32)).wrapping_mul(4)));
        {
            l_i_7 = 0i32;
            'l1: loop {
                if !(((((l_i_7) < (4i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let v_ = { let a0_ = c.m.r32(((sym::gRefVu0).wrapping_add((((p_fs_3) as u32)).wrapping_mul(16))).wrapping_add((((l_i_7) as u32)).wrapping_mul(4))); F::vu0_a__Sf_MulBits(c, a0_, l_b_6) }; let a_ = (fp).wrapping_add((((l_i_7) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                }
                { let o_ = l_i_7; let n_ = (o_).wrapping_add(1); l_i_7 = n_; };
            }
        }
        F::vu0_a__setFlags(c, fp, p_m_1);
        F::vu0_a__put(c, p_fd_2, fp, p_m_1);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__op_vmulbc", &PROBE_N, &[], true); }
    r_
}

pub fn vu0_a__op_vaddabc(c: &mut Ctx, mut p_m_1: i32, mut p_fs_2: i32, mut p_ft_3: i32, mut p_c_4: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__op_vaddabc", &PROBE_N, &[(p_m_1) as u32, (p_fs_2) as u32, (p_ft_3) as u32, (p_c_4) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_b_5: u32 = 0;
    let mut l_i_6: i32 = 0;
    let r_ = 'f: {
        l_b_5 = c.m.r32(((sym::gRefVu0).wrapping_add((((p_ft_3) as u32)).wrapping_mul(16))).wrapping_add((((p_c_4) as u32)).wrapping_mul(4)));
        {
            l_i_6 = 0i32;
            'l1: loop {
                if !(((((l_i_6) < (4i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let v_ = { let a0_ = c.m.r32(((sym::gRefVu0).wrapping_add((((p_fs_2) as u32)).wrapping_mul(16))).wrapping_add((((l_i_6) as u32)).wrapping_mul(4))); F::vu0_a__Sf_AddBits(c, a0_, l_b_5) }; let a_ = (fp).wrapping_add((((l_i_6) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                }
                { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; };
            }
        }
        F::vu0_a__setFlags(c, fp, p_m_1);
        F::vu0_a__putAcc(c, fp, p_m_1);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__op_vaddabc", &PROBE_N, &[], true); }
    r_
}

pub fn vu0_a__op_vmulabc(c: &mut Ctx, mut p_m_1: i32, mut p_fs_2: i32, mut p_ft_3: i32, mut p_c_4: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__op_vmulabc", &PROBE_N, &[(p_m_1) as u32, (p_fs_2) as u32, (p_ft_3) as u32, (p_c_4) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_b_5: u32 = 0;
    let mut l_i_6: i32 = 0;
    let r_ = 'f: {
        l_b_5 = c.m.r32(((sym::gRefVu0).wrapping_add((((p_ft_3) as u32)).wrapping_mul(16))).wrapping_add((((p_c_4) as u32)).wrapping_mul(4)));
        {
            l_i_6 = 0i32;
            'l1: loop {
                if !(((((l_i_6) < (4i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let v_ = { let a0_ = c.m.r32(((sym::gRefVu0).wrapping_add((((p_fs_2) as u32)).wrapping_mul(16))).wrapping_add((((l_i_6) as u32)).wrapping_mul(4))); F::vu0_a__Sf_MulBits(c, a0_, l_b_5) }; let a_ = (fp).wrapping_add((((l_i_6) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                }
                { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; };
            }
        }
        F::vu0_a__setFlags(c, fp, p_m_1);
        F::vu0_a__putAcc(c, fp, p_m_1);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__op_vmulabc", &PROBE_N, &[], true); }
    r_
}

pub fn vu0_a__op_vmaddabc(c: &mut Ctx, mut p_m_1: i32, mut p_fs_2: i32, mut p_ft_3: i32, mut p_c_4: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__op_vmaddabc", &PROBE_N, &[(p_m_1) as u32, (p_fs_2) as u32, (p_ft_3) as u32, (p_c_4) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_b_5: u32 = 0;
    let mut l_i_6: i32 = 0;
    let r_ = 'f: {
        l_b_5 = c.m.r32(((sym::gRefVu0).wrapping_add((((p_ft_3) as u32)).wrapping_mul(16))).wrapping_add((((p_c_4) as u32)).wrapping_mul(4)));
        {
            l_i_6 = 0i32;
            'l1: loop {
                if !(((((l_i_6) < (4i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let v_ = { let a0_ = c.m.r32(((sym::gRefVu0).wrapping_add(512)).wrapping_add((((l_i_6) as u32)).wrapping_mul(4))); let a1_ = { let a0_ = c.m.r32(((sym::gRefVu0).wrapping_add((((p_fs_2) as u32)).wrapping_mul(16))).wrapping_add((((l_i_6) as u32)).wrapping_mul(4))); F::vu0_a__Sf_MulBits(c, a0_, l_b_5) }; F::vu0_a__Sf_AddBits(c, a0_, a1_) }; let a_ = (fp).wrapping_add((((l_i_6) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                }
                { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; };
            }
        }
        F::vu0_a__setFlags(c, fp, p_m_1);
        F::vu0_a__putAcc(c, fp, p_m_1);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__op_vmaddabc", &PROBE_N, &[], true); }
    r_
}

pub fn vu0_a__op_vmaddbc(c: &mut Ctx, mut p_m_1: i32, mut p_fd_2: i32, mut p_fs_3: i32, mut p_ft_4: i32, mut p_c_5: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__op_vmaddbc", &PROBE_N, &[(p_m_1) as u32, (p_fd_2) as u32, (p_fs_3) as u32, (p_ft_4) as u32, (p_c_5) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_b_6: u32 = 0;
    let mut l_i_7: i32 = 0;
    let r_ = 'f: {
        l_b_6 = c.m.r32(((sym::gRefVu0).wrapping_add((((p_ft_4) as u32)).wrapping_mul(16))).wrapping_add((((p_c_5) as u32)).wrapping_mul(4)));
        {
            l_i_7 = 0i32;
            'l1: loop {
                if !(((((l_i_7) < (4i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let v_ = { let a0_ = c.m.r32(((sym::gRefVu0).wrapping_add(512)).wrapping_add((((l_i_7) as u32)).wrapping_mul(4))); let a1_ = { let a0_ = c.m.r32(((sym::gRefVu0).wrapping_add((((p_fs_3) as u32)).wrapping_mul(16))).wrapping_add((((l_i_7) as u32)).wrapping_mul(4))); F::vu0_a__Sf_MulBits(c, a0_, l_b_6) }; F::vu0_a__Sf_AddBits(c, a0_, a1_) }; let a_ = (fp).wrapping_add((((l_i_7) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                }
                { let o_ = l_i_7; let n_ = (o_).wrapping_add(1); l_i_7 = n_; };
            }
        }
        F::vu0_a__setFlags(c, fp, p_m_1);
        F::vu0_a__put(c, p_fd_2, fp, p_m_1);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__op_vmaddbc", &PROBE_N, &[], true); }
    r_
}

pub fn vu0_a__op_vmulq(c: &mut Ctx, mut p_m_1: i32, mut p_fd_2: i32, mut p_fs_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__op_vmulq", &PROBE_N, &[(p_m_1) as u32, (p_fd_2) as u32, (p_fs_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_i_4: i32 = 0;
    let r_ = 'f: {
        {
            l_i_4 = 0i32;
            'l1: loop {
                if !(((((l_i_4) < (4i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let v_ = { let a0_ = c.m.r32(((sym::gRefVu0).wrapping_add((((p_fs_3) as u32)).wrapping_mul(16))).wrapping_add((((l_i_4) as u32)).wrapping_mul(4))); let a1_ = c.m.r32((sym::gRefVu0).wrapping_add(528)); F::vu0_a__Sf_MulBits(c, a0_, a1_) }; let a_ = (fp).wrapping_add((((l_i_4) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                }
                { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
            }
        }
        F::vu0_a__setFlags(c, fp, p_m_1);
        F::vu0_a__put(c, p_fd_2, fp, p_m_1);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__op_vmulq", &PROBE_N, &[], true); }
    r_
}

pub fn vu0_a__op_vaddq(c: &mut Ctx, mut p_m_1: i32, mut p_fd_2: i32, mut p_fs_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__op_vaddq", &PROBE_N, &[(p_m_1) as u32, (p_fd_2) as u32, (p_fs_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_i_4: i32 = 0;
    let r_ = 'f: {
        {
            l_i_4 = 0i32;
            'l1: loop {
                if !(((((l_i_4) < (4i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let v_ = { let a0_ = c.m.r32(((sym::gRefVu0).wrapping_add((((p_fs_3) as u32)).wrapping_mul(16))).wrapping_add((((l_i_4) as u32)).wrapping_mul(4))); let a1_ = c.m.r32((sym::gRefVu0).wrapping_add(528)); F::vu0_a__Sf_AddBits(c, a0_, a1_) }; let a_ = (fp).wrapping_add((((l_i_4) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                }
                { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
            }
        }
        F::vu0_a__setFlags(c, fp, p_m_1);
        F::vu0_a__put(c, p_fd_2, fp, p_m_1);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__op_vaddq", &PROBE_N, &[], true); }
    r_
}

pub fn vu0_a__op_vopmula(c: &mut Ctx, mut p_fs_1: i32, mut p_ft_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__op_vopmula", &PROBE_N, &[(p_fs_1) as u32, (p_ft_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        { let v_ = { let a0_ = c.m.r32(((sym::gRefVu0).wrapping_add((((p_fs_1) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4))); let a1_ = c.m.r32(((sym::gRefVu0).wrapping_add((((p_ft_2) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4))); F::vu0_a__Sf_MulBits(c, a0_, a1_) }; let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = { let a0_ = c.m.r32(((sym::gRefVu0).wrapping_add((((p_fs_1) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4))); let a1_ = c.m.r32(((sym::gRefVu0).wrapping_add((((p_ft_2) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4))); F::vu0_a__Sf_MulBits(c, a0_, a1_) }; let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = { let a0_ = c.m.r32(((sym::gRefVu0).wrapping_add((((p_fs_1) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4))); let a1_ = c.m.r32(((sym::gRefVu0).wrapping_add((((p_ft_2) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4))); F::vu0_a__Sf_MulBits(c, a0_, a1_) }; let a_ = (fp).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = (fp).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        F::vu0_a__setFlags(c, fp, 14i32);
        F::vu0_a__putAcc(c, fp, 14i32);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__op_vopmula", &PROBE_N, &[], true); }
    r_
}

pub fn vu0_a__op_vopmsub(c: &mut Ctx, mut p_fd_1: i32, mut p_fs_2: i32, mut p_ft_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__op_vopmsub", &PROBE_N, &[(p_fd_1) as u32, (p_fs_2) as u32, (p_ft_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        { let v_ = { let a0_ = c.m.r32(((sym::gRefVu0).wrapping_add(512)).wrapping_add((((0i32) as u32)).wrapping_mul(4))); let a1_ = { let a0_ = c.m.r32(((sym::gRefVu0).wrapping_add((((p_fs_2) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4))); let a1_ = c.m.r32(((sym::gRefVu0).wrapping_add((((p_ft_3) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4))); F::vu0_a__Sf_MulBits(c, a0_, a1_) }; F::RefVu0_SubBits(c, a0_, a1_) }; let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = { let a0_ = c.m.r32(((sym::gRefVu0).wrapping_add(512)).wrapping_add((((1i32) as u32)).wrapping_mul(4))); let a1_ = { let a0_ = c.m.r32(((sym::gRefVu0).wrapping_add((((p_fs_2) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4))); let a1_ = c.m.r32(((sym::gRefVu0).wrapping_add((((p_ft_3) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4))); F::vu0_a__Sf_MulBits(c, a0_, a1_) }; F::RefVu0_SubBits(c, a0_, a1_) }; let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = { let a0_ = c.m.r32(((sym::gRefVu0).wrapping_add(512)).wrapping_add((((2i32) as u32)).wrapping_mul(4))); let a1_ = { let a0_ = c.m.r32(((sym::gRefVu0).wrapping_add((((p_fs_2) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4))); let a1_ = c.m.r32(((sym::gRefVu0).wrapping_add((((p_ft_3) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4))); F::vu0_a__Sf_MulBits(c, a0_, a1_) }; F::RefVu0_SubBits(c, a0_, a1_) }; let a_ = (fp).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = (fp).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        F::vu0_a__setFlags(c, fp, 14i32);
        F::vu0_a__put(c, p_fd_1, fp, 14i32);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__op_vopmsub", &PROBE_N, &[], true); }
    r_
}

pub fn vu0_a__op_vmr32(c: &mut Ctx, mut p_m_1: i32, mut p_ft_2: i32, mut p_fs_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__op_vmr32", &PROBE_N, &[(p_m_1) as u32, (p_ft_2) as u32, (p_fs_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        { let v_ = c.m.r32(((sym::gRefVu0).wrapping_add((((p_fs_3) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4))); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = c.m.r32(((sym::gRefVu0).wrapping_add((((p_fs_3) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4))); let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = c.m.r32(((sym::gRefVu0).wrapping_add((((p_fs_3) as u32)).wrapping_mul(16))).wrapping_add((((3i32) as u32)).wrapping_mul(4))); let a_ = (fp).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = c.m.r32(((sym::gRefVu0).wrapping_add((((p_fs_3) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4))); let a_ = (fp).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        F::vu0_a__put(c, p_ft_2, fp, p_m_1);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__op_vmr32", &PROBE_N, &[], true); }
    r_
}

pub fn vu0_a__op_vmove(c: &mut Ctx, mut p_m_1: i32, mut p_ft_2: i32, mut p_fs_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__op_vmove", &PROBE_N, &[(p_m_1) as u32, (p_ft_2) as u32, (p_fs_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        { let _ = { rt::memcpy(c, fp, (sym::gRefVu0).wrapping_add((((p_fs_3) as u32)).wrapping_mul(16)), (((16i32) as u32)) as u32); fp }; };
        F::vu0_a__put(c, p_ft_2, fp, p_m_1);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__op_vmove", &PROBE_N, &[], true); }
    r_
}

pub fn vu0_a__itof(c: &mut Ctx, mut p_m_1: i32, mut p_ft_2: i32, mut p_fs_3: i32, mut p_frac_4: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__itof", &PROBE_N, &[(p_m_1) as u32, (p_ft_2) as u32, (p_fs_3) as u32, (p_frac_4) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_i_5: i32 = 0;
    let r_ = 'f: {
        {
            l_i_5 = 0i32;
            'l1: loop {
                if !(((((l_i_5) < (4i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let v_ = { let a0_ = (c.m.r32(((sym::gRefVu0).wrapping_add((((p_fs_3) as u32)).wrapping_mul(16))).wrapping_add((((l_i_5) as u32)).wrapping_mul(4))) as i32); F::RefVu0_ItofBits(c, a0_, p_frac_4) }; let a_ = (fp).wrapping_add((((l_i_5) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                }
                { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
            }
        }
        F::vu0_a__put(c, p_ft_2, fp, p_m_1);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__itof", &PROBE_N, &[], true); }
    r_
}

pub fn vu0_a__op_vitof0(c: &mut Ctx, mut p_m_1: i32, mut p_ft_2: i32, mut p_fs_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__op_vitof0", &PROBE_N, &[(p_m_1) as u32, (p_ft_2) as u32, (p_fs_3) as u32], false); }
    let r_ = 'f: {
    F::vu0_a__itof(c, p_m_1, p_ft_2, p_fs_3, 0i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__op_vitof0", &PROBE_N, &[], true); }
    r_
}

pub fn vu0_a__op_vitof4(c: &mut Ctx, mut p_m_1: i32, mut p_ft_2: i32, mut p_fs_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__op_vitof4", &PROBE_N, &[(p_m_1) as u32, (p_ft_2) as u32, (p_fs_3) as u32], false); }
    let r_ = 'f: {
    F::vu0_a__itof(c, p_m_1, p_ft_2, p_fs_3, 4i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__op_vitof4", &PROBE_N, &[], true); }
    r_
}

pub fn vu0_a__op_vitof12(c: &mut Ctx, mut p_m_1: i32, mut p_ft_2: i32, mut p_fs_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__op_vitof12", &PROBE_N, &[(p_m_1) as u32, (p_ft_2) as u32, (p_fs_3) as u32], false); }
    let r_ = 'f: {
    F::vu0_a__itof(c, p_m_1, p_ft_2, p_fs_3, 12i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__op_vitof12", &PROBE_N, &[], true); }
    r_
}

pub fn vu0_a__op_vsqrt(c: &mut Ctx, mut p_ft_1: i32, mut p_c_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__op_vsqrt", &PROBE_N, &[(p_ft_1) as u32, (p_c_2) as u32], false); }
    let r_ = 'f: {
    { let v_ = { let a0_ = c.m.r32(((sym::gRefVu0).wrapping_add((((p_ft_1) as u32)).wrapping_mul(16))).wrapping_add((((p_c_2) as u32)).wrapping_mul(4))); F::RefVu0_SqrtBits(c, a0_) }; let a_ = (sym::gRefVu0).wrapping_add(528); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__op_vsqrt", &PROBE_N, &[], true); }
    r_
}

pub fn vu0_a__op_vdiv(c: &mut Ctx, mut p_fs_1: i32, mut p_a_2: i32, mut p_ft_3: i32, mut p_b_4: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__op_vdiv", &PROBE_N, &[(p_fs_1) as u32, (p_a_2) as u32, (p_ft_3) as u32, (p_b_4) as u32], false); }
    let r_ = 'f: {
    { let v_ = { let a0_ = c.m.r32(((sym::gRefVu0).wrapping_add((((p_fs_1) as u32)).wrapping_mul(16))).wrapping_add((((p_a_2) as u32)).wrapping_mul(4))); let a1_ = c.m.r32(((sym::gRefVu0).wrapping_add((((p_ft_3) as u32)).wrapping_mul(16))).wrapping_add((((p_b_4) as u32)).wrapping_mul(4))); F::RefVu0_DivBits(c, a0_, a1_) }; let a_ = (sym::gRefVu0).wrapping_add(528); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__op_vdiv", &PROBE_N, &[], true); }
    r_
}

pub fn vu0_a__op_vsqi(c: &mut Ctx, mut p_fs_1: i32, mut p_vi_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__op_vsqi", &PROBE_N, &[(p_fs_1) as u32, (p_vi_2) as u32], false); }
    let r_ = 'f: {
    { let s_ = (sym::gRefVu0).wrapping_add((((p_fs_1) as u32)).wrapping_mul(16)); let d_ = ((sym::gRefVu0).wrapping_add(568)).wrapping_add((((((((c.m.r16(((sym::gRefVu0).wrapping_add(532)).wrapping_add((((p_vi_2) as u32)).wrapping_mul(2)))) as i32)) & (255i32))) as u32)).wrapping_mul(16)); rt::memcpy(c, d_, s_, 16); };
    { let a_ = ((sym::gRefVu0).wrapping_add(532)).wrapping_add((((p_vi_2) as u32)).wrapping_mul(2)); let o_ = c.m.r16(a_); let n_ = (o_).wrapping_add(1); c.m.w16(a_, n_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__op_vsqi", &PROBE_N, &[], true); }
    r_
}

pub fn vu0_a__op_vlqd(c: &mut Ctx, mut p_ft_1: i32, mut p_vi_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__op_vlqd", &PROBE_N, &[(p_ft_1) as u32, (p_vi_2) as u32], false); }
    let r_ = 'f: {
    { let a_ = ((sym::gRefVu0).wrapping_add(532)).wrapping_add((((p_vi_2) as u32)).wrapping_mul(2)); let o_ = c.m.r16(a_); let n_ = (o_).wrapping_sub(1); c.m.w16(a_, n_); };
    { let a1_ = ((sym::gRefVu0).wrapping_add(568)).wrapping_add((((((((c.m.r16(((sym::gRefVu0).wrapping_add(532)).wrapping_add((((p_vi_2) as u32)).wrapping_mul(2)))) as i32)) & (255i32))) as u32)).wrapping_mul(16)); F::vu0_a__put(c, p_ft_1, a1_, 15i32) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__op_vlqd", &PROBE_N, &[], true); }
    r_
}

pub fn vu0_a__ee_lq(c: &mut Ctx, mut p_t_1: i32, mut p_p_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__ee_lq", &PROBE_N, &[(p_t_1) as u32, (p_p_2) as u32], false); }
    let r_ = 'f: {
    { let _ = { rt::memcpy(c, ((sym::gRefVu0).wrapping_add(4664)).wrapping_add((((p_t_1) as u32)).wrapping_mul(16)), p_p_2, (((16i32) as u32)) as u32); ((sym::gRefVu0).wrapping_add(4664)).wrapping_add((((p_t_1) as u32)).wrapping_mul(16)) }; };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__ee_lq", &PROBE_N, &[], true); }
    r_
}

pub fn vu0_a__ee_sq(c: &mut Ctx, mut p_t_1: i32, mut p_p_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__ee_sq", &PROBE_N, &[(p_t_1) as u32, (p_p_2) as u32], false); }
    let r_ = 'f: {
    { let _ = { rt::memcpy(c, p_p_2, ((sym::gRefVu0).wrapping_add(4664)).wrapping_add((((p_t_1) as u32)).wrapping_mul(16)), (((16i32) as u32)) as u32); p_p_2 }; };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__ee_sq", &PROBE_N, &[], true); }
    r_
}

pub fn vu0_a__ee_mfc1(c: &mut Ctx, mut p_t_1: i32, mut p_bits_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__ee_mfc1", &PROBE_N, &[(p_t_1) as u32, (p_bits_2) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_bits_2; let a_ = (((sym::gRefVu0).wrapping_add(4664)).wrapping_add((((p_t_1) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = (if ((((p_bits_2) & (2147483648u32))) != 0) { 4294967295u32 } else { ((0i32) as u32) }); let a_ = (((sym::gRefVu0).wrapping_add(4664)).wrapping_add((((p_t_1) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__ee_mfc1", &PROBE_N, &[], true); }
    r_
}

pub fn vu0_a__ee_set64(c: &mut Ctx, mut p_t_1: i32, mut p_lo_2: u32, mut p_hi_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__ee_set64", &PROBE_N, &[(p_t_1) as u32, (p_lo_2) as u32, (p_hi_3) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_lo_2; let a_ = (((sym::gRefVu0).wrapping_add(4664)).wrapping_add((((p_t_1) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = p_hi_3; let a_ = (((sym::gRefVu0).wrapping_add(4664)).wrapping_add((((p_t_1) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__ee_set64", &PROBE_N, &[], true); }
    r_
}

pub fn vu0_a__ee_pextlw(c: &mut Ctx, mut p_d_1: i32, mut p_s_2: i32, mut p_t_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__ee_pextlw", &PROBE_N, &[(p_d_1) as u32, (p_s_2) as u32, (p_t_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        { let v_ = c.m.r32((((sym::gRefVu0).wrapping_add(4664)).wrapping_add((((p_t_3) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4))); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = c.m.r32((((sym::gRefVu0).wrapping_add(4664)).wrapping_add((((p_s_2) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4))); let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = c.m.r32((((sym::gRefVu0).wrapping_add(4664)).wrapping_add((((p_t_3) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4))); let a_ = (fp).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = c.m.r32((((sym::gRefVu0).wrapping_add(4664)).wrapping_add((((p_s_2) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4))); let a_ = (fp).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let s_ = fp; let d_ = ((sym::gRefVu0).wrapping_add(4664)).wrapping_add((((p_d_1) as u32)).wrapping_mul(16)); rt::memcpy(c, d_, s_, 16); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__ee_pextlw", &PROBE_N, &[], true); }
    r_
}

pub fn vu0_a__ee_pextuw(c: &mut Ctx, mut p_d_1: i32, mut p_s_2: i32, mut p_t_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__ee_pextuw", &PROBE_N, &[(p_d_1) as u32, (p_s_2) as u32, (p_t_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        { let v_ = c.m.r32((((sym::gRefVu0).wrapping_add(4664)).wrapping_add((((p_t_3) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4))); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = c.m.r32((((sym::gRefVu0).wrapping_add(4664)).wrapping_add((((p_s_2) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4))); let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = c.m.r32((((sym::gRefVu0).wrapping_add(4664)).wrapping_add((((p_t_3) as u32)).wrapping_mul(16))).wrapping_add((((3i32) as u32)).wrapping_mul(4))); let a_ = (fp).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = c.m.r32((((sym::gRefVu0).wrapping_add(4664)).wrapping_add((((p_s_2) as u32)).wrapping_mul(16))).wrapping_add((((3i32) as u32)).wrapping_mul(4))); let a_ = (fp).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let s_ = fp; let d_ = ((sym::gRefVu0).wrapping_add(4664)).wrapping_add((((p_d_1) as u32)).wrapping_mul(16)); rt::memcpy(c, d_, s_, 16); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__ee_pextuw", &PROBE_N, &[], true); }
    r_
}

pub fn vu0_a__ee_pcpyld(c: &mut Ctx, mut p_d_1: i32, mut p_s_2: i32, mut p_t_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__ee_pcpyld", &PROBE_N, &[(p_d_1) as u32, (p_s_2) as u32, (p_t_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        { let v_ = c.m.r32((((sym::gRefVu0).wrapping_add(4664)).wrapping_add((((p_t_3) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4))); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = c.m.r32((((sym::gRefVu0).wrapping_add(4664)).wrapping_add((((p_t_3) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4))); let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = c.m.r32((((sym::gRefVu0).wrapping_add(4664)).wrapping_add((((p_s_2) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4))); let a_ = (fp).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = c.m.r32((((sym::gRefVu0).wrapping_add(4664)).wrapping_add((((p_s_2) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4))); let a_ = (fp).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let s_ = fp; let d_ = ((sym::gRefVu0).wrapping_add(4664)).wrapping_add((((p_d_1) as u32)).wrapping_mul(16)); rt::memcpy(c, d_, s_, 16); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__ee_pcpyld", &PROBE_N, &[], true); }
    r_
}

pub fn vu0_a__ee_pcpyud(c: &mut Ctx, mut p_d_1: i32, mut p_s_2: i32, mut p_t_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__ee_pcpyud", &PROBE_N, &[(p_d_1) as u32, (p_s_2) as u32, (p_t_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        { let v_ = c.m.r32((((sym::gRefVu0).wrapping_add(4664)).wrapping_add((((p_s_2) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4))); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = c.m.r32((((sym::gRefVu0).wrapping_add(4664)).wrapping_add((((p_s_2) as u32)).wrapping_mul(16))).wrapping_add((((3i32) as u32)).wrapping_mul(4))); let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = c.m.r32((((sym::gRefVu0).wrapping_add(4664)).wrapping_add((((p_t_3) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4))); let a_ = (fp).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = c.m.r32((((sym::gRefVu0).wrapping_add(4664)).wrapping_add((((p_t_3) as u32)).wrapping_mul(16))).wrapping_add((((3i32) as u32)).wrapping_mul(4))); let a_ = (fp).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let s_ = fp; let d_ = ((sym::gRefVu0).wrapping_add(4664)).wrapping_add((((p_d_1) as u32)).wrapping_mul(16)); rt::memcpy(c, d_, s_, 16); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__ee_pcpyud", &PROBE_N, &[], true); }
    r_
}

pub fn vu0_a__op_qmtc2(c: &mut Ctx, mut p_t_1: i32, mut p_fd_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__op_qmtc2", &PROBE_N, &[(p_t_1) as u32, (p_fd_2) as u32], false); }
    let r_ = 'f: {
    F::vu0_a__put(c, p_fd_2, ((sym::gRefVu0).wrapping_add(4664)).wrapping_add((((p_t_1) as u32)).wrapping_mul(16)), 15i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__op_qmtc2", &PROBE_N, &[], true); }
    r_
}

pub fn vu0_a__op_qmfc2(c: &mut Ctx, mut p_t_1: i32, mut p_fs_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__op_qmfc2", &PROBE_N, &[(p_t_1) as u32, (p_fs_2) as u32], false); }
    let r_ = 'f: {
    { let s_ = (sym::gRefVu0).wrapping_add((((p_fs_2) as u32)).wrapping_mul(16)); let d_ = ((sym::gRefVu0).wrapping_add(4664)).wrapping_add((((p_t_1) as u32)).wrapping_mul(16)); rt::memcpy(c, d_, s_, 16); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__op_qmfc2", &PROBE_N, &[], true); }
    r_
}

pub fn vu0_a__ftoiBits(c: &mut Ctx, mut p_a_1: u32, mut p_frac_2: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__ftoiBits", &PROBE_N, &[(p_a_1) as u32, (p_frac_2) as u32], false); }
    let mut l_m_3: u32 = 0;
    let mut l_v_4: u32 = 0;
    let mut l_e_5: i32 = 0;
    let r_ = 'f: {
    l_m_3 = ((((p_a_1) & (((8388607i32) as u32)))) | (((8388608i32) as u32)));
    l_e_5 = (((((((p_a_1).wrapping_shr((23i32) as u32)) & (((255i32) as u32)))) as i32)).wrapping_sub(127i32)).wrapping_add(p_frac_2);
    if ((((((((((((p_a_1).wrapping_shr((23i32) as u32)) & (((255i32) as u32)))) == (((0i32) as u32))) as i32)) != 0) || (((((l_e_5) < (0i32)) as i32)) != 0)) as i32)) != 0) {
        break 'f ((0i32) as u32);
    }
    if (((((l_e_5) >= (31i32)) as i32)) != 0) {
        break 'f (if ((((p_a_1) & (2147483648u32))) != 0) { 2147483648u32 } else { 2147483647u32 });
    }
    l_v_4 = (if (((((l_e_5) >= (23i32)) as i32)) != 0) { (l_m_3).wrapping_shl(((l_e_5).wrapping_sub(23i32)) as u32) } else { (l_m_3).wrapping_shr(((23i32).wrapping_sub(l_e_5)) as u32) });
    break 'f (if ((((p_a_1) & (2147483648u32))) != 0) { (0u32).wrapping_sub(l_v_4) } else { l_v_4 });
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__ftoiBits", &PROBE_N, &[], true); }
    r_
}

pub fn vu0_a__op_vftoi(c: &mut Ctx, mut p_m_1: i32, mut p_ft_2: i32, mut p_fs_3: i32, mut p_frac_4: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__op_vftoi", &PROBE_N, &[(p_m_1) as u32, (p_ft_2) as u32, (p_fs_3) as u32, (p_frac_4) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_i_5: i32 = 0;
    let r_ = 'f: {
        {
            l_i_5 = 0i32;
            'l1: loop {
                if !(((((l_i_5) < (4i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let v_ = { let a0_ = c.m.r32(((sym::gRefVu0).wrapping_add((((p_fs_3) as u32)).wrapping_mul(16))).wrapping_add((((l_i_5) as u32)).wrapping_mul(4))); F::vu0_a__ftoiBits(c, a0_, p_frac_4) }; let a_ = (fp).wrapping_add((((l_i_5) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                }
                { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
            }
        }
        F::vu0_a__put(c, p_ft_2, fp, p_m_1);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__op_vftoi", &PROBE_N, &[], true); }
    r_
}

pub fn vu0_a__op_vmfir(c: &mut Ctx, mut p_m_1: i32, mut p_ft_2: i32, mut p_vi_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__op_vmfir", &PROBE_N, &[(p_m_1) as u32, (p_ft_2) as u32, (p_vi_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_i_4: i32 = 0;
    let r_ = 'f: {
        {
            l_i_4 = 0i32;
            'l1: loop {
                if !(((((l_i_4) < (4i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let v_ = ((((((c.m.r16(((sym::gRefVu0).wrapping_add(532)).wrapping_add((((p_vi_3) as u32)).wrapping_mul(2)))) as i16)) as i32)) as u32); let a_ = (fp).wrapping_add((((l_i_4) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                }
                { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
            }
        }
        F::vu0_a__put(c, p_ft_2, fp, p_m_1);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__op_vmfir", &PROBE_N, &[], true); }
    r_
}

pub fn vu0_a__seam_c(c: &mut Ctx, mut p_a0_1: u32, mut p_a1_2: u32, mut p_a2_3: u32, mut p_a3_4: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__seam_c", &PROBE_N, &[(p_a0_1) as u32, (p_a1_2) as u32, (p_a2_3) as u32, (p_a3_4) as u32], false); }
    let mut l_i_5: i32 = 0;
    let mut l_out_6: i32 = 0;
    let r_ = 'f: {
    F::vu0_a__put(c, 19i32, sym::vu0_a__seam_c__k4096, 15i32);
    { let v_ = ((32768i32) as u16); let a_ = ((sym::gRefVu0).wrapping_add(532)).wrapping_add((((1i32) as u32)).wrapping_mul(2)); c.m.w16(a_, v_); };
    {
        l_i_5 = 0i32;
        'l1: loop {
            if !(((((l_i_5) < (8i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                F::vu0_a__op_lqc2(c, (1i32).wrapping_add(l_i_5), (p_a2_3).wrapping_add((((l_i_5).wrapping_mul(16i32)) as u32)));
            }
            { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
        }
    }
    {
        l_i_5 = 0i32;
        'l3: loop {
            if !(((((l_i_5) < (4i32)) as i32)) != 0) { break 'l3; }
            'c4: {
                F::vu0_a__op_lqc2(c, (9i32).wrapping_add(l_i_5), (p_a1_2).wrapping_add((((128i32).wrapping_add((l_i_5).wrapping_mul(16i32))) as u32)));
            }
            { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
        }
    }
    {
        l_i_5 = 0i32;
        'l5: loop {
            if !(((((l_i_5) < (3i32)) as i32)) != 0) { break 'l5; }
            'c6: {
                F::vu0_a__op_vmulabc(c, 15i32, 9i32, (1i32).wrapping_add(l_i_5), 0i32);
                F::vu0_a__op_vmaddabc(c, 15i32, 10i32, (1i32).wrapping_add(l_i_5), 1i32);
                F::vu0_a__op_vmaddabc(c, 15i32, 11i32, (1i32).wrapping_add(l_i_5), 2i32);
                F::vu0_a__op_vmaddbc(c, 15i32, (13i32).wrapping_add(l_i_5), 12i32, (1i32).wrapping_add(l_i_5), 3i32);
            }
            { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
        }
    }
    F::vu0_a__op_sqc2(c, 13i32, (p_a1_2).wrapping_add(((0i32) as u32)));
    F::vu0_a__op_sqc2(c, 14i32, (p_a1_2).wrapping_add(((16i32) as u32)));
    F::vu0_a__op_sqc2(c, 15i32, (p_a1_2).wrapping_add(((32i32) as u32)));
    F::vu0_a__op_sqc2(c, 12i32, (p_a1_2).wrapping_add(((48i32) as u32)));
    {
        l_i_5 = 0i32;
        'l7: loop {
            if !(((((l_i_5) < (3i32)) as i32)) != 0) { break 'l7; }
            'c8: {
                F::vu0_a__op_vmulabc(c, 15i32, 9i32, (5i32).wrapping_add(l_i_5), 0i32);
                F::vu0_a__op_vmaddabc(c, 15i32, 10i32, (5i32).wrapping_add(l_i_5), 1i32);
                F::vu0_a__op_vmaddabc(c, 15i32, 11i32, (5i32).wrapping_add(l_i_5), 2i32);
                F::vu0_a__op_vmaddbc(c, 15i32, (13i32).wrapping_add(l_i_5), 12i32, (5i32).wrapping_add(l_i_5), 3i32);
            }
            { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
        }
    }
    F::vu0_a__op_sqc2(c, 13i32, (p_a1_2).wrapping_add(((64i32) as u32)));
    F::vu0_a__op_sqc2(c, 14i32, (p_a1_2).wrapping_add(((80i32) as u32)));
    F::vu0_a__op_sqc2(c, 15i32, (p_a1_2).wrapping_add(((96i32) as u32)));
    F::vu0_a__op_sqc2(c, 12i32, (p_a1_2).wrapping_add(((112i32) as u32)));
    F::vu0_a__op_lqc2(c, 13i32, (p_a0_1).wrapping_add(((16i32) as u32)));
    F::vu0_a__op_lqc2(c, 16i32, (p_a0_1).wrapping_add(((0i32) as u32)));
    {
        l_i_5 = 0i32;
        'l9: loop {
            if !(((((l_i_5) < (4i32)) as i32)) != 0) { break 'l9; }
            'c10: {
                F::vu0_a__op_lqc2(c, (9i32).wrapping_add(l_i_5), (p_a1_2).wrapping_add((((l_i_5).wrapping_mul(16i32)) as u32)));
            }
            { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
        }
    }
    F::vu0_a__op_vmulabc(c, 15i32, 9i32, 13i32, 0i32);
    F::vu0_a__op_vmaddabc(c, 15i32, 10i32, 13i32, 1i32);
    F::vu0_a__op_vmaddabc(c, 15i32, 11i32, 13i32, 2i32);
    F::vu0_a__op_vmaddbc(c, 15i32, 14i32, 12i32, 0i32, 3i32);
    {
        l_i_5 = 0i32;
        'l11: loop {
            if !(((((l_i_5) < (4i32)) as i32)) != 0) { break 'l11; }
            'c12: {
                F::vu0_a__op_lqc2(c, (9i32).wrapping_add(l_i_5), (p_a1_2).wrapping_add((((64i32).wrapping_add((l_i_5).wrapping_mul(16i32))) as u32)));
            }
            { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
        }
    }
    F::vu0_a__op_vmulabc(c, 15i32, 9i32, 13i32, 0i32);
    F::vu0_a__op_vmaddabc(c, 15i32, 10i32, 13i32, 1i32);
    F::vu0_a__op_vmaddabc(c, 15i32, 11i32, 13i32, 2i32);
    F::vu0_a__op_vmaddbc(c, 15i32, 15i32, 12i32, 0i32, 3i32);
    F::vu0_a__op_vsub(c, 15i32, 14i32, 14i32, 15i32);
    F::vu0_a__op_vmulbc(c, 15i32, 14i32, 14i32, 16i32, 3i32);
    F::vu0_a__op_vadd(c, 14i32, 14i32, 15i32, 14i32);
    F::vu0_a__op_lqc2(c, 13i32, (p_a1_2).wrapping_add(((192i32) as u32)));
    F::vu0_a__op_vmulbc(c, 14i32, 14i32, 14i32, 13i32, 3i32);
    F::vu0_a__op_vaddbc(c, 14i32, 14i32, 14i32, 13i32, 3i32);
    F::vu0_a__op_vadd(c, 8i32, 15i32, 0i32, 14i32);
    F::vu0_a__op_vadd(c, 4i32, 15i32, 0i32, 0i32);
    F::vu0_a__op_vaddbc(c, 2i32, 15i32, 0i32, 0i32, 3i32);
    F::vu0_a__op_lqc2(c, 9i32, (p_a3_4).wrapping_add(((0i32) as u32)));
    F::vu0_a__op_lqc2(c, 10i32, (p_a3_4).wrapping_add(((16i32) as u32)));
    F::vu0_a__op_vsub(c, 15i32, 11i32, 16i32, 9i32);
    F::vu0_a__op_vsub(c, 15i32, 12i32, 16i32, 10i32);
    F::vu0_a__op_vmulabc(c, 15i32, 1i32, 11i32, 0i32);
    F::vu0_a__op_vmaddabc(c, 15i32, 2i32, 11i32, 1i32);
    F::vu0_a__op_vmaddabc(c, 15i32, 3i32, 11i32, 2i32);
    F::vu0_a__op_vmaddbc(c, 15i32, 13i32, 4i32, 0i32, 3i32);
    F::vu0_a__op_vmulabc(c, 15i32, 5i32, 12i32, 0i32);
    F::vu0_a__op_vmaddabc(c, 15i32, 6i32, 12i32, 1i32);
    F::vu0_a__op_vmaddabc(c, 15i32, 7i32, 12i32, 2i32);
    F::vu0_a__op_vmaddbc(c, 15i32, 14i32, 8i32, 0i32, 3i32);
    F::vu0_a__op_vsub(c, 15i32, 9i32, 13i32, 14i32);
    F::vu0_a__op_vmulbc(c, 15i32, 9i32, 9i32, 16i32, 3i32);
    F::vu0_a__op_vadd(c, 15i32, 9i32, 14i32, 9i32);
    F::vu0_a__op_vmulabc(c, 15i32, 24i32, 9i32, 0i32);
    F::vu0_a__op_vmaddabc(c, 15i32, 25i32, 9i32, 1i32);
    F::vu0_a__op_vmaddabc(c, 15i32, 26i32, 9i32, 2i32);
    F::vu0_a__op_vmaddbc(c, 15i32, 9i32, 27i32, 9i32, 3i32);
    F::vu0_a__op_vdiv(c, 0i32, 3i32, 9i32, 3i32);
    F::vu0_a__op_vmulq(c, 14i32, 9i32, 9i32);
    F::vu0_a__op_vftoi(c, 15i32, 10i32, 9i32, 4i32);
    { let v_ = ((0i32) as u32); let a_ = (sym::gRefVu0).wrapping_add(564); c.m.w32(a_, v_); };
    F::vu0_a__op_vsub(c, 13i32, 0i32, 9i32, 0i32);
    F::vu0_a__op_vsub(c, 12i32, 0i32, 19i32, 9i32);
    l_out_6 = (((((c.m.r32((sym::gRefVu0).wrapping_add(564))) & (((192i32) as u32)))) != (((0i32) as u32))) as i32);
    if ((l_out_6) != 0) {
        F::vu0_a__op_vmfir(c, 1i32, 10i32, 1i32);
    }
    F::vu0_a__op_lqc2(c, 11i32, (p_a0_1).wrapping_add(((32i32) as u32)));
    F::vu0_a__op_vmulq(c, 14i32, 12i32, 11i32);
    F::vu0_a__op_vmulq(c, 14i32, 14i32, 15i32);
    F::vu0_a__op_sqc2(c, 12i32, (p_a0_1).wrapping_add(((80i32) as u32)));
    F::vu0_a__op_sqc2(c, 14i32, (p_a0_1).wrapping_add(((96i32) as u32)));
    F::vu0_a__op_sqc2(c, 10i32, (p_a0_1).wrapping_add(((48i32) as u32)));
       
    };
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__seam_c", &PROBE_N, &[], true); }
    r_
}

pub fn vu0_a__seam_code_load(c: &mut Ctx, mut p_path_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__seam_code_load", &PROBE_N, &[(p_path_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(112) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 112);
    let mut l_fp_2: u32 = 0;
    let mut l_i_3: u32 = 0;
    let mut l_n_4: u32 = 0;
    let mut l_ok_5: i32 = 0;
    let r_ = 'f: {
        l_fp_2 = F::fopen(c, p_path_1, sym::vu0_a__str4);
        l_ok_5 = 0i32;
        if (((((l_fp_2) == (((0i32) as u32))) as i32)) != 0) {
            break 'f 0i32;
        }
        if (((((((((((((F::fread(c, fp, ((1i32) as u32), 64u32, l_fp_2)) == (64u32)) as i32)) != 0) && (((((((c.m.r8((fp).wrapping_add(((0i32) as u32)))) as i32)) == (127i32)) as i32)) != 0)) as i32)) != 0) && (((((((c.m.r8((fp).wrapping_add(((1i32) as u32)))) as i32)) == (69i32)) as i32)) != 0)) as i32)) != 0) {
            { let _ = { rt::memcpy(c, fp.wrapping_add(96), (fp).wrapping_add(((28i32) as u32)), (((4i32) as u32)) as u32); fp.wrapping_add(96) }; };
            l_n_4 = ((((((c.m.r8((fp).wrapping_add(((44i32) as u32)))) as i32)) | ((((c.m.r8((fp).wrapping_add(((45i32) as u32)))) as i32)).wrapping_shl((8i32) as u32)))) as u32);
            {
                l_i_3 = ((0i32) as u32);
                'l1: loop {
                    if !(((((((((l_i_3) < (l_n_4)) as i32)) != 0) && ((((!((l_ok_5) != 0)) as i32)) != 0)) as i32)) != 0) { break 'l1; }
                    'c2: {
                        { let _ = { let a1_ = (((c.m.r32(fp.wrapping_add(96))).wrapping_add((l_i_3).wrapping_mul(((32i32) as u32)))) as i32); F::fseek(c, l_fp_2, a1_, 0i32) }; };
                        if (((((F::fread(c, fp.wrapping_add(64), ((1i32) as u32), 32u32, l_fp_2)) != (32u32)) as i32)) != 0) {
                            break 'l1;
                        }
                        { let _ = { rt::memcpy(c, fp.wrapping_add(100), (fp.wrapping_add(64)).wrapping_add(((4i32) as u32)), (((4i32) as u32)) as u32); fp.wrapping_add(100) }; };
                        { let _ = { rt::memcpy(c, fp.wrapping_add(104), (fp.wrapping_add(64)).wrapping_add(((8i32) as u32)), (((4i32) as u32)) as u32); fp.wrapping_add(104) }; };
                        { let _ = { rt::memcpy(c, fp.wrapping_add(108), (fp.wrapping_add(64)).wrapping_add(((16i32) as u32)), (((4i32) as u32)) as u32); fp.wrapping_add(108) }; };
                        if (((((((((c.m.r32(fp.wrapping_add(104))) <= (((1114064i32) as u32))) as i32)) != 0) && ((((((((1114064i32) as u32)).wrapping_add(512u32)) <= ((c.m.r32(fp.wrapping_add(104))).wrapping_add(c.m.r32(fp.wrapping_add(108))))) as i32)) != 0)) as i32)) != 0) {
                            { let _ = { let a1_ = (((c.m.r32(fp.wrapping_add(100))).wrapping_add((((1114064i32) as u32)).wrapping_sub(c.m.r32(fp.wrapping_add(104))))) as i32); F::fseek(c, l_fp_2, a1_, 0i32) }; };
                            l_ok_5 = (((F::fread(c, sym::vu0_a__sSeamCode, ((4i32) as u32), ((128i32) as u32), l_fp_2)) == (((128i32) as u32))) as i32);
                        }
                    }
                    { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
                }
            }
        }
        { let _ = F::fclose(c, l_fp_2); };
        break 'f l_ok_5;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__seam_code_load", &PROBE_N, &[], true); }
    r_
}

pub fn vu0_a__seam_interp(c: &mut Ctx, mut p_a0_1: u32, mut p_a1_2: u32, mut p_a2_3: u32, mut p_a3_4: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__seam_interp", &PROBE_N, &[(p_a0_1) as u32, (p_a1_2) as u32, (p_a2_3) as u32, (p_a3_4) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(400) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 400);
    let mut l_pc_5: i32 = 0;
    let mut l_next_6: i32 = 0;
    let mut l_steps_7: i32 = 0;
    let mut l_w_8: u32 = 0;
    let mut l_op_9: u32 = 0;
    let mut l_rs_10: u32 = 0;
    let mut l_rt_11: u32 = 0;
    let mut l_rd_12: u32 = 0;
    let mut l_sa_13: u32 = 0;
    let mut l_imm_14: i16 = 0;
    let mut l_slot_15: i32 = 0;
    let mut l_target_16: i32 = 0;
    let mut l_m_17: i32 = 0;
    let mut l_ft_18: i32 = 0;
    let mut l_fs_19: i32 = 0;
    let mut l_fd_20: i32 = 0;
    let mut l_bc_21: i32 = 0;
    let mut l_fn_22: i32 = 0;
    let mut l_idx_23: i32 = 0;
    let mut l_d_24: u32 = 0;
    let r_ = 'f: {
        l_pc_5 = 0i32;
        { let _ = F::memset(c, fp, 0i32, 128u32); };
        { let _ = F::memset(c, fp.wrapping_add(128), 0i32, 256u32); };
        { let v_ = p_a0_1; let a_ = (fp).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = p_a1_2; let a_ = (fp).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = p_a2_3; let a_ = (fp).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = p_a3_4; let a_ = (fp).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        {
            l_steps_7 = 0i32;
            'l1: loop {
                if !(((((l_steps_7) < (1000i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_w_8 = c.m.r32((sym::vu0_a__sSeamCode).wrapping_add((((l_pc_5) as u32)).wrapping_mul(4)));
                    l_op_9 = (l_w_8).wrapping_shr((26i32) as u32);
                    l_rs_10 = (((l_w_8).wrapping_shr((21i32) as u32)) & (((31i32) as u32)));
                    l_rt_11 = (((l_w_8).wrapping_shr((16i32) as u32)) & (((31i32) as u32)));
                    l_rd_12 = (((l_w_8).wrapping_shr((11i32) as u32)) & (((31i32) as u32)));
                    l_sa_13 = (((l_w_8).wrapping_shr((6i32) as u32)) & (((31i32) as u32)));
                    l_imm_14 = ((((l_w_8) & (((65535i32) as u32)))) as i16);
                    l_slot_15 = 0i32;
                    l_target_16 = (1i32).wrapping_neg();
                    l_next_6 = (l_pc_5).wrapping_add(1i32);
                    if (((((l_w_8) == (((0i32) as u32))) as i32)) != 0) {
                    } else {
                        if (((((l_op_9) == (((15i32) as u32))) as i32)) != 0) {
                            { let v_ = (((((((((((l_imm_14) as u16)) as u32)).wrapping_shl((16i32) as u32)) as i32)) as i64)) as u64); let a_ = (fp.wrapping_add(128)).wrapping_add((l_rt_11).wrapping_mul(8)); c.m.w64(a_, v_); };
                        } else {
                            if (((((l_op_9) == (((13i32) as u32))) as i32)) != 0) {
                                { let v_ = ((c.m.r64((fp.wrapping_add(128)).wrapping_add((l_rs_10).wrapping_mul(8)))) | (((((l_imm_14) as u16)) as u64))); let a_ = (fp.wrapping_add(128)).wrapping_add((l_rt_11).wrapping_mul(8)); c.m.w64(a_, v_); };
                            } else {
                                if (((((l_op_9) == (((12i32) as u32))) as i32)) != 0) {
                                    { let v_ = ((c.m.r64((fp.wrapping_add(128)).wrapping_add((l_rs_10).wrapping_mul(8)))) & (((((l_imm_14) as u16)) as u64))); let a_ = (fp.wrapping_add(128)).wrapping_add((l_rt_11).wrapping_mul(8)); c.m.w64(a_, v_); };
                                } else {
                                    if (((((l_op_9) == (((9i32) as u32))) as i32)) != 0) {
                                        { let v_ = (((((((((c.m.r64((fp.wrapping_add(128)).wrapping_add((l_rs_10).wrapping_mul(8)))) as u32)).wrapping_add(((((l_imm_14) as i32)) as u32))) as i32)) as i64)) as u64); let a_ = (fp.wrapping_add(128)).wrapping_add((l_rt_11).wrapping_mul(8)); c.m.w64(a_, v_); };
                                    } else {
                                        if (((((((((l_op_9) == (((0i32) as u32))) as i32)) != 0) && (((((((l_w_8) & (((63i32) as u32)))) == (((56i32) as u32))) as i32)) != 0)) as i32)) != 0) {
                                            { let v_ = (c.m.r64((fp.wrapping_add(128)).wrapping_add((l_rt_11).wrapping_mul(8)))).wrapping_shl((l_sa_13) as u32); let a_ = (fp.wrapping_add(128)).wrapping_add((l_rd_12).wrapping_mul(8)); c.m.w64(a_, v_); };
                                        } else {
                                            if (((((((((l_op_9) == (((0i32) as u32))) as i32)) != 0) && (((((((l_w_8) & (((63i32) as u32)))) == (((0i32) as u32))) as i32)) != 0)) as i32)) != 0) {
                                                { let v_ = (((((((((c.m.r64((fp.wrapping_add(128)).wrapping_add((l_rt_11).wrapping_mul(8)))) as u32)).wrapping_shl((l_sa_13) as u32)) as i32)) as i64)) as u64); let a_ = (fp.wrapping_add(128)).wrapping_add((l_rd_12).wrapping_mul(8)); c.m.w64(a_, v_); };
                                            } else {
                                                if (((((((((l_op_9) == (((0i32) as u32))) as i32)) != 0) && (((((((l_w_8) & (((63i32) as u32)))) == (((8i32) as u32))) as i32)) != 0)) as i32)) != 0) {
                                                    l_slot_15 = 1i32;
                                                    l_target_16 = 128i32;
                                                } else {
                                                    if (((((l_op_9) == (((4i32) as u32))) as i32)) != 0) {
                                                        l_slot_15 = 1i32;
                                                        l_target_16 = (if (((((c.m.r64((fp.wrapping_add(128)).wrapping_add((l_rs_10).wrapping_mul(8)))) == (c.m.r64((fp.wrapping_add(128)).wrapping_add((l_rt_11).wrapping_mul(8))))) as i32)) != 0) { ((l_pc_5).wrapping_add(1i32)).wrapping_add(((l_imm_14) as i32)) } else { (l_pc_5).wrapping_add(2i32) });
                                                    } else {
                                                        if (((((l_op_9) == (((54i32) as u32))) as i32)) != 0) {
                                                            { let a1_ = (c.m.r32((fp).wrapping_add((l_rs_10).wrapping_mul(4)))).wrapping_add(((((l_imm_14) as i32)) as u32)); F::vu0_a__op_lqc2(c, ((l_rt_11) as i32), a1_) };
                                                        } else {
                                                            if (((((l_op_9) == (((62i32) as u32))) as i32)) != 0) {
                                                                { let a1_ = (c.m.r32((fp).wrapping_add((l_rs_10).wrapping_mul(4)))).wrapping_add(((((l_imm_14) as i32)) as u32)); F::vu0_a__op_sqc2(c, ((l_rt_11) as i32), a1_) };
                                                            } else {
                                                                if (((((((((l_op_9) == (((18i32) as u32))) as i32)) != 0) && (((((l_rs_10) == (((5i32) as u32))) as i32)) != 0)) as i32)) != 0) {
                                                                    rt::memset(c, fp.wrapping_add(384), 0, 16);
                                                                    { let v_ = ((c.m.r64((fp.wrapping_add(128)).wrapping_add((l_rt_11).wrapping_mul(8)))) as u32); c.m.w32(fp.wrapping_add(384).wrapping_add(0), v_); }
                                                                    { let v_ = (((c.m.r64((fp.wrapping_add(128)).wrapping_add((l_rt_11).wrapping_mul(8)))).wrapping_shr((32i32) as u32)) as u32); c.m.w32(fp.wrapping_add(384).wrapping_add(4), v_); }
                                                                    { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(384).wrapping_add(8), v_); }
                                                                    { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(384).wrapping_add(12), v_); }
                                                                    F::vu0_a__put(c, ((l_rd_12) as i32), fp.wrapping_add(384), 15i32);
                                                                } else {
                                                                    if (((((((((l_op_9) == (((18i32) as u32))) as i32)) != 0) && (((((l_rs_10) == (((6i32) as u32))) as i32)) != 0)) as i32)) != 0) {
                                                                        if (((((l_rd_12) == (((16i32) as u32))) as i32)) != 0) {
                                                                            { let v_ = ((c.m.r64((fp.wrapping_add(128)).wrapping_add((l_rt_11).wrapping_mul(8)))) as u32); let a_ = (sym::gRefVu0).wrapping_add(564); c.m.w32(a_, v_); };
                                                                        } else {
                                                                            if (((((l_rd_12) < (((16i32) as u32))) as i32)) != 0) {
                                                                                { let v_ = ((c.m.r64((fp.wrapping_add(128)).wrapping_add((l_rt_11).wrapping_mul(8)))) as u16); let a_ = ((sym::gRefVu0).wrapping_add(532)).wrapping_add((l_rd_12).wrapping_mul(2)); c.m.w16(a_, v_); };
                                                                            } else {
                                                                                break 'f 0i32;
                                                                            }
                                                                        }
                                                                    } else {
                                                                        if (((((((((l_op_9) == (((18i32) as u32))) as i32)) != 0) && (((((l_rs_10) == (((2i32) as u32))) as i32)) != 0)) as i32)) != 0) {
                                                                            if (((((l_rd_12) != (((16i32) as u32))) as i32)) != 0) {
                                                                                break 'f 0i32;
                                                                            }
                                                                            { let v_ = ((c.m.r32((sym::gRefVu0).wrapping_add(564))) as u64); let a_ = (fp.wrapping_add(128)).wrapping_add((l_rt_11).wrapping_mul(8)); c.m.w64(a_, v_); };
                                                                        } else {
                                                                            if (((((((((l_op_9) == (((18i32) as u32))) as i32)) != 0) && ((((l_w_8) & (((33554432i32) as u32)))) != 0)) as i32)) != 0) {
                                                                                l_m_17 = (((((l_w_8).wrapping_shr((21i32) as u32)) & (((15i32) as u32)))) as i32);
                                                                                l_ft_18 = ((l_rt_11) as i32);
                                                                                l_fs_19 = ((l_rd_12) as i32);
                                                                                l_fd_20 = ((l_sa_13) as i32);
                                                                                l_bc_21 = ((((l_w_8) & (((3i32) as u32)))) as i32);
                                                                                l_fn_22 = ((((l_w_8) & (((63i32) as u32)))) as i32);
                                                                                if (((((l_fn_22) < (60i32)) as i32)) != 0) {
                                                                                    if (((((l_fn_22) < (4i32)) as i32)) != 0) {
                                                                                        F::vu0_a__op_vaddbc(c, l_m_17, l_fd_20, l_fs_19, l_ft_18, l_bc_21);
                                                                                    } else {
                                                                                        if (((((((((l_fn_22) >= (8i32)) as i32)) != 0) && (((((l_fn_22) < (12i32)) as i32)) != 0)) as i32)) != 0) {
                                                                                            F::vu0_a__op_vmaddbc(c, l_m_17, l_fd_20, l_fs_19, l_ft_18, l_bc_21);
                                                                                        } else {
                                                                                            if (((((((((l_fn_22) >= (24i32)) as i32)) != 0) && (((((l_fn_22) < (28i32)) as i32)) != 0)) as i32)) != 0) {
                                                                                                F::vu0_a__op_vmulbc(c, l_m_17, l_fd_20, l_fs_19, l_ft_18, l_bc_21);
                                                                                            } else {
                                                                                                if (((((l_fn_22) == (28i32)) as i32)) != 0) {
                                                                                                    F::vu0_a__op_vmulq(c, l_m_17, l_fd_20, l_fs_19);
                                                                                                } else {
                                                                                                    if (((((l_fn_22) == (40i32)) as i32)) != 0) {
                                                                                                        F::vu0_a__op_vadd(c, l_m_17, l_fd_20, l_fs_19, l_ft_18);
                                                                                                    } else {
                                                                                                        if (((((l_fn_22) == (44i32)) as i32)) != 0) {
                                                                                                            F::vu0_a__op_vsub(c, l_m_17, l_fd_20, l_fs_19, l_ft_18);
                                                                                                        } else {
                                                                                                            break 'f 0i32;
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                } else {
                                                                                    l_idx_23 = (((((l_sa_13).wrapping_shl((2i32) as u32)) as i32)) | (l_bc_21));
                                                                                    if (((((((((l_idx_23) >= (8i32)) as i32)) != 0) && (((((l_idx_23) < (12i32)) as i32)) != 0)) as i32)) != 0) {
                                                                                        F::vu0_a__op_vmaddabc(c, l_m_17, l_fs_19, l_ft_18, l_bc_21);
                                                                                    } else {
                                                                                        if (((((((((l_idx_23) >= (24i32)) as i32)) != 0) && (((((l_idx_23) < (28i32)) as i32)) != 0)) as i32)) != 0) {
                                                                                            F::vu0_a__op_vmulabc(c, l_m_17, l_fs_19, l_ft_18, l_bc_21);
                                                                                        } else {
                                                                                            if (((((l_idx_23) == (21i32)) as i32)) != 0) {
                                                                                                F::vu0_a__op_vftoi(c, l_m_17, l_ft_18, l_fs_19, 4i32);
                                                                                            } else {
                                                                                                if (((((l_idx_23) == (56i32)) as i32)) != 0) {
                                                                                                    F::vu0_a__op_vdiv(c, l_fs_19, (((((l_w_8).wrapping_shr((21i32) as u32)) & (((3i32) as u32)))) as i32), l_ft_18, (((((l_w_8).wrapping_shr((23i32) as u32)) & (((3i32) as u32)))) as i32));
                                                                                                } else {
                                                                                                    if (((((l_idx_23) == (61i32)) as i32)) != 0) {
                                                                                                        F::vu0_a__op_vmfir(c, l_m_17, l_ft_18, l_fs_19);
                                                                                                    } else {
                                                                                                        if (((((((((l_idx_23) == (59i32)) as i32)) != 0) || (((((l_idx_23) == (47i32)) as i32)) != 0)) as i32)) != 0) {
                                                                                                        } else {
                                                                                                            break 'f 0i32;
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            } else {
                                                                                break 'f 0i32;
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    if ((l_slot_15) != 0) {
                        l_d_24 = c.m.r32((sym::vu0_a__sSeamCode).wrapping_add(((((l_pc_5).wrapping_add(1i32)) as u32)).wrapping_mul(4)));
                        if (((((l_d_24) == (((0i32) as u32))) as i32)) != 0) {
                        } else {
                            if ((((((l_d_24).wrapping_shr((26i32) as u32)) == (((62i32) as u32))) as i32)) != 0) {
                                { let a1_ = (c.m.r32((fp).wrapping_add(((((l_d_24).wrapping_shr((21i32) as u32)) & (((31i32) as u32)))).wrapping_mul(4)))).wrapping_add(((((((((l_d_24) & (((65535i32) as u32)))) as i16)) as i32)) as u32)); F::vu0_a__op_sqc2(c, (((((l_d_24).wrapping_shr((16i32) as u32)) & (((31i32) as u32)))) as i32), a1_) };
                            } else {
                                break 'f 0i32;
                            }
                        }
                        l_next_6 = l_target_16;
                    }
                    if (((((l_next_6) >= (128i32)) as i32)) != 0) {
                        break 'f 1i32;
                    }
                    l_pc_5 = l_next_6;
                }
                { let o_ = l_steps_7; let n_ = (o_).wrapping_add(1); l_steps_7 = n_; };
            }
        }
        break 'f 0i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__seam_interp", &PROBE_N, &[], true); }
    r_
}

pub fn ObjSeam_TransformVtx(c: &mut Ctx, mut p_vtx_1: u32, mut p_work_2: u32, mut p_mtx_3: u32, mut p_ofs_4: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ObjSeam_TransformVtx", &PROBE_N, &[(p_vtx_1) as u32, (p_work_2) as u32, (p_mtx_3) as u32, (p_ofs_4) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(640) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 640);
    let mut l_path_5: u32 = 0;
    let mut l_ran_6: i32 = 0;
    let mut l_same_7: i32 = 0;
    let r_ = 'f: {
        if ((((((c.m.r32(sym::vu0_a__ObjSeam_TransformVtx__mode) as i32)) < (0i32)) as i32)) != 0) {
            l_path_5 = F::getenv(c, sym::vu0_a__str5);
            { let v_ = (if (((((l_path_5) == (((0i32) as u32))) as i32)) != 0) { 0i32 } else { (if ((F::vu0_a__seam_code_load(c, l_path_5)) != 0) { 1i32 } else { 2i32 }) }); let a_ = sym::vu0_a__ObjSeam_TransformVtx__mode; c.m.w32(a_, (v_) as u32); };
            if ((((((c.m.r32(sym::vu0_a__ObjSeam_TransformVtx__mode) as i32)) == (2i32)) as i32)) != 0) {
                { let _ = { let a0_ = c.m.r32(sym::stderr); F::fprintf(c, a0_, sym::vu0_a__str6, &[((l_path_5) as u64)]) }; };
            }
        }
        if ((((((c.m.r32(sym::vu0_a__ObjSeam_TransformVtx__mode) as i32)) == (1i32)) as i32)) != 0) {
            { let s_ = sym::gRefVu0; let d_ = sym::vu0_a__ObjSeam_TransformVtx__before; rt::memcpy(c, d_, s_, 4792); };
            { let _ = { rt::memcpy(c, fp, p_vtx_1, (112u32) as u32); fp }; };
            { let _ = { rt::memcpy(c, fp.wrapping_add(112), p_work_2, (208u32) as u32); fp.wrapping_add(112) }; };
            F::vu0_a__seam_c(c, p_vtx_1, p_work_2, p_mtx_3, p_ofs_4);
            { let s_ = sym::gRefVu0; let d_ = sym::vu0_a__ObjSeam_TransformVtx__afterC; rt::memcpy(c, d_, s_, 4792); };
            { let _ = { rt::memcpy(c, fp.wrapping_add(320), p_vtx_1, (112u32) as u32); fp.wrapping_add(320) }; };
            { let _ = { rt::memcpy(c, fp.wrapping_add(432), p_work_2, (208u32) as u32); fp.wrapping_add(432) }; };
            { let s_ = sym::vu0_a__ObjSeam_TransformVtx__before; let d_ = sym::gRefVu0; rt::memcpy(c, d_, s_, 4792); };
            { let _ = { rt::memcpy(c, p_vtx_1, fp, (112u32) as u32); p_vtx_1 }; };
            { let _ = { rt::memcpy(c, p_work_2, fp.wrapping_add(112), (208u32) as u32); p_work_2 }; };
            l_ran_6 = F::vu0_a__seam_interp(c, p_vtx_1, p_work_2, p_mtx_3, p_ofs_4);
            l_same_7 = ((((((((((((((((((((((((((((l_ran_6) != 0) && (((((F::memcmp(c, fp.wrapping_add(320), p_vtx_1, 112u32)) == (0i32)) as i32)) != 0)) as i32)) != 0) && (((((F::memcmp(c, fp.wrapping_add(432), p_work_2, 208u32)) == (0i32)) as i32)) != 0)) as i32)) != 0) && (((((F::memcmp(c, sym::vu0_a__ObjSeam_TransformVtx__afterC, sym::gRefVu0, 512u32)) == (0i32)) as i32)) != 0)) as i32)) != 0) && (((((F::memcmp(c, (sym::vu0_a__ObjSeam_TransformVtx__afterC).wrapping_add(512), (sym::gRefVu0).wrapping_add(512), 16u32)) == (0i32)) as i32)) != 0)) as i32)) != 0) && (((((c.m.r32((sym::vu0_a__ObjSeam_TransformVtx__afterC).wrapping_add(528))) == (c.m.r32((sym::gRefVu0).wrapping_add(528)))) as i32)) != 0)) as i32)) != 0) && (((((c.m.r32((sym::vu0_a__ObjSeam_TransformVtx__afterC).wrapping_add(564))) == (c.m.r32((sym::gRefVu0).wrapping_add(564)))) as i32)) != 0)) as i32)) != 0) && (((((F::memcmp(c, (sym::vu0_a__ObjSeam_TransformVtx__afterC).wrapping_add(532), (sym::gRefVu0).wrapping_add(532), 32u32)) == (0i32)) as i32)) != 0)) as i32);
            { let a_ = sym::vu0_a__ObjSeam_TransformVtx__calls; let o_ = c.m.r32(a_); let n_ = (o_).wrapping_add(1); c.m.w32(a_, n_); };
            if ((((!((l_same_7) != 0)) as i32)) != 0) {
                { let a_ = sym::vu0_a__ObjSeam_TransformVtx__bad; let o_ = c.m.r32(a_); let n_ = (o_).wrapping_add(1); c.m.w32(a_, n_); };
                if (((((c.m.r32(sym::vu0_a__ObjSeam_TransformVtx__bad)) <= (((5i32) as u32))) as i32)) != 0) {
                    { let _ = { let a0_ = c.m.r32(sym::stderr); let a2_ = c.m.r32(sym::vu0_a__ObjSeam_TransformVtx__calls); F::fprintf(c, a0_, sym::vu0_a__str7, &[((a2_) as u64), (((if ((l_ran_6) != 0) { sym::vu0_a__str8 } else { sym::vu0_a__str9 })) as u64)]) }; };
                }
            }
            if (((((((((c.m.r32(sym::vu0_a__ObjSeam_TransformVtx__calls)) == (((1i32) as u32))) as i32)) != 0) || (((((rt::rem_u32(c.m.r32(sym::vu0_a__ObjSeam_TransformVtx__calls), ((20000i32) as u32))) == (((0i32) as u32))) as i32)) != 0)) as i32)) != 0) {
                { let _ = { let a0_ = c.m.r32(sym::stderr); let a2_ = c.m.r32(sym::vu0_a__ObjSeam_TransformVtx__calls); let a3_ = c.m.r32(sym::vu0_a__ObjSeam_TransformVtx__bad); F::fprintf(c, a0_, sym::vu0_a__str10, &[((a2_) as u64), ((a3_) as u64)]) }; };
            }
            break 'f;
        }
        F::vu0_a__seam_c(c, p_vtx_1, p_work_2, p_mtx_3, p_ofs_4);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "ObjSeam_TransformVtx", &PROBE_N, &[], true); }
    r_
}

pub fn Vu0_ResetState(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vu0_ResetState", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let _ = F::memset(c, sym::gRefVu0, 0i32, 4792u32); };
    { let v_ = 1065353216u32; let a_ = ((sym::gRefVu0).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Vu0_ResetState", &PROBE_N, &[], true); }
    r_
}

pub fn vu0_a__b_Vec4_SetZeroW1(c: &mut Ctx, mut p_a0_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__b_Vec4_SetZeroW1", &PROBE_N, &[(p_a0_1) as u32], false); }
    let r_ = 'f: {
    F::vu0_a__op_sqc2(c, 0i32, p_a0_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__b_Vec4_SetZeroW1", &PROBE_N, &[], true); }
    r_
}

pub fn vu0_a__b_Vec3_Set(c: &mut Ctx, mut p_v_1: u32, mut p_x_2: u32, mut p_y_3: u32, mut p_z_4: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__b_Vec3_Set", &PROBE_N, &[(p_v_1) as u32, (p_x_2) as u32, (p_y_3) as u32, (p_z_4) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_z_4; let a_ = (p_v_1).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = p_x_2; let a_ = (p_v_1).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = p_y_3; let a_ = (p_v_1).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__b_Vec3_Set", &PROBE_N, &[], true); }
    r_
}

pub fn vu0_a__b_Vec3_Normalize(c: &mut Ctx, mut p_a0_1: u32, mut p_a1_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__b_Vec3_Normalize", &PROBE_N, &[(p_a0_1) as u32, (p_a1_2) as u32], false); }
    let r_ = 'f: {
    F::vu0_a__op_lqc2(c, 4i32, p_a1_2);
    F::vu0_a__op_vmul(c, 14i32, 5i32, 4i32, 4i32);
    F::vu0_a__op_vaddabc(c, 8i32, 5i32, 5i32, 1i32);
    F::vu0_a__op_vmaddbc(c, 8i32, 5i32, 3i32, 5i32, 2i32);
    F::vu0_a__op_vsqrt(c, 5i32, 0i32);
    F::vu0_a__op_vaddq(c, 8i32, 5i32, 0i32);
    F::vu0_a__op_vdiv(c, 0i32, 3i32, 5i32, 0i32);
    F::vu0_a__op_vsub(c, 15i32, 6i32, 0i32, 0i32);
    F::vu0_a__op_vmulq(c, 14i32, 6i32, 4i32);
    F::vu0_a__op_sqc2(c, 6i32, p_a0_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__b_Vec3_Normalize", &PROBE_N, &[], true); }
    r_
}

pub fn vu0_a__b_Vec4_Scale(c: &mut Ctx, mut p_a0_1: u32, mut p_a1_2: u32, mut p_f12_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__b_Vec4_Scale", &PROBE_N, &[(p_a0_1) as u32, (p_a1_2) as u32, (p_f12_3) as u32], false); }
    let r_ = 'f: {
    F::vu0_a__op_lqc2(c, 4i32, p_a1_2);
    F::vu0_a__ee_mfc1(c, 0i32, p_f12_3);
    F::vu0_a__op_qmtc2(c, 0i32, 5i32);
    F::vu0_a__op_vmulbc(c, 15i32, 4i32, 4i32, 5i32, 0i32);
    F::vu0_a__op_sqc2(c, 4i32, p_a0_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__b_Vec4_Scale", &PROBE_N, &[], true); }
    r_
}

pub fn vu0_a__b_Vec4_Copy(c: &mut Ctx, mut p_a0_1: u32, mut p_a1_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__b_Vec4_Copy", &PROBE_N, &[(p_a0_1) as u32, (p_a1_2) as u32], false); }
    let r_ = 'f: {
    F::vu0_a__ee_lq(c, 0i32, p_a1_2);
    F::vu0_a__ee_sq(c, 0i32, p_a0_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__b_Vec4_Copy", &PROBE_N, &[], true); }
    r_
}

pub fn vu0_a__b_Mtx_MulVec4(c: &mut Ctx, mut p_a0_1: u32, mut p_a1_2: u32, mut p_a2_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__b_Mtx_MulVec4", &PROBE_N, &[(p_a0_1) as u32, (p_a1_2) as u32, (p_a2_3) as u32], false); }
    let r_ = 'f: {
    F::vu0_a__op_lqc2(c, 8i32, p_a2_3);
    F::vu0_a__op_lqc2(c, 4i32, p_a1_2);
    F::vu0_a__op_lqc2(c, 5i32, (p_a1_2).wrapping_add(((16i32) as u32)));
    F::vu0_a__op_lqc2(c, 6i32, (p_a1_2).wrapping_add(((32i32) as u32)));
    F::vu0_a__op_lqc2(c, 7i32, (p_a1_2).wrapping_add(((48i32) as u32)));
    F::vu0_a__op_vmulabc(c, 15i32, 4i32, 8i32, 0i32);
    F::vu0_a__op_vmaddabc(c, 15i32, 5i32, 8i32, 1i32);
    F::vu0_a__op_vmaddabc(c, 15i32, 6i32, 8i32, 2i32);
    F::vu0_a__op_vmaddbc(c, 15i32, 8i32, 7i32, 8i32, 3i32);
    F::vu0_a__op_sqc2(c, 8i32, p_a0_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__b_Mtx_MulVec4", &PROBE_N, &[], true); }
    r_
}

pub fn vu0_a__b_Vec3_Cross(c: &mut Ctx, mut p_a0_1: u32, mut p_a1_2: u32, mut p_a2_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__b_Vec3_Cross", &PROBE_N, &[(p_a0_1) as u32, (p_a1_2) as u32, (p_a2_3) as u32], false); }
    let r_ = 'f: {
    F::vu0_a__op_lqc2(c, 4i32, p_a1_2);
    F::vu0_a__op_lqc2(c, 5i32, p_a2_3);
    F::vu0_a__op_vopmula(c, 4i32, 5i32);
    F::vu0_a__op_vopmsub(c, 5i32, 5i32, 4i32);
    F::vu0_a__op_vsub(c, 1i32, 5i32, 5i32, 5i32);
    F::vu0_a__op_sqc2(c, 5i32, p_a0_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__b_Vec3_Cross", &PROBE_N, &[], true); }
    r_
}

pub fn vu0_a__b_Vec3_Length(c: &mut Ctx, mut p_a0_1: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__b_Vec3_Length", &PROBE_N, &[(p_a0_1) as u32], false); }
    let r_ = 'f: {
    F::vu0_a__op_lqc2(c, 4i32, p_a0_1);
    F::vu0_a__op_vmul(c, 14i32, 4i32, 4i32, 4i32);
    F::vu0_a__op_vaddabc(c, 8i32, 4i32, 4i32, 1i32);
    F::vu0_a__op_vmaddbc(c, 8i32, 4i32, 3i32, 4i32, 2i32);
    F::vu0_a__op_vsqrt(c, 4i32, 0i32);
    break 'f c.m.r32((sym::gRefVu0).wrapping_add(528));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__b_Vec3_Length", &PROBE_N, &[], true); }
    r_
}

pub fn vu0_a__b_Vec3_DirToEuler(c: &mut Ctx, mut p_out_1: u32, mut p_dir_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__b_Vec3_DirToEuler", &PROBE_N, &[(p_out_1) as u32, (p_dir_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_len_3: u32 = 0;
    let r_ = 'f: {
        { let _ = F::memset(c, fp, 0i32, 16u32); };
        { let v_ = { let a0_ = F32(c.m.r32((p_dir_2).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a1_ = F32(c.m.r32((p_dir_2).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); F::Port_atan2f(c, a0_, a1_) }; let a_ = (p_out_1).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let a1_ = c.m.r32((p_dir_2).wrapping_add((((0i32) as u32)).wrapping_mul(4))); let a3_ = c.m.r32((p_dir_2).wrapping_add((((2i32) as u32)).wrapping_mul(4))); F::vu0_a__b_Vec3_Set(c, fp, a1_, ((0i32) as u32), a3_) };
        l_len_3 = F::vu0_a__b_Vec3_Length(c, fp);
        { let v_ = ((0i32) as u32); let a_ = (p_out_1).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = (({ let a0_ = { let a0_ = F32(c.m.r32((p_dir_2).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a1_ = F::vu0_a__u2f(c, l_len_3); F::Port_atan2f(c, a0_, a1_) }; F::vu0_a__f2u(c, a0_) }) ^ (2147483648u32)); let a_ = (p_out_1).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__b_Vec3_DirToEuler", &PROBE_N, &[], true); }
    r_
}

pub fn IVec4_SetZeroW1(c: &mut Ctx, mut p_v_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "IVec4_SetZeroW1", &PROBE_N, &[(p_v_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = 0i32; let a_ = (p_v_1).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    { let v_ = 1i32; let a_ = (p_v_1).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    { let v_ = 0i32; let a_ = (p_v_1).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    { let v_ = 0i32; let a_ = (p_v_1).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "IVec4_SetZeroW1", &PROBE_N, &[], true); }
    r_
}

pub fn IVec4_SetZero(c: &mut Ctx, mut p_v_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "IVec4_SetZero", &PROBE_N, &[(p_v_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = 0i32; let a_ = (p_v_1).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    { let v_ = 0i32; let a_ = (p_v_1).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    { let v_ = 0i32; let a_ = (p_v_1).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    { let v_ = 0i32; let a_ = (p_v_1).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "IVec4_SetZero", &PROBE_N, &[], true); }
    r_
}

pub fn IVec4_Set(c: &mut Ctx, mut p_v_1: u32, mut p_x_2: i32, mut p_y_3: i32, mut p_z_4: i32, mut p_w_5: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "IVec4_Set", &PROBE_N, &[(p_v_1) as u32, (p_x_2) as u32, (p_y_3) as u32, (p_z_4) as u32, (p_w_5) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_w_5; let a_ = (p_v_1).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    { let v_ = p_x_2; let a_ = (p_v_1).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    { let v_ = p_y_3; let a_ = (p_v_1).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    { let v_ = p_z_4; let a_ = (p_v_1).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "IVec4_Set", &PROBE_N, &[], true); }
    r_
}

pub fn IVec3_Set(c: &mut Ctx, mut p_v_1: u32, mut p_x_2: i32, mut p_y_3: i32, mut p_z_4: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "IVec3_Set", &PROBE_N, &[(p_v_1) as u32, (p_x_2) as u32, (p_y_3) as u32, (p_z_4) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_z_4; let a_ = (p_v_1).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    { let v_ = p_x_2; let a_ = (p_v_1).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    { let v_ = p_y_3; let a_ = (p_v_1).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "IVec3_Set", &PROBE_N, &[], true); }
    r_
}

pub fn IVec4_Swap(c: &mut Ctx, mut p_a_1: u32, mut p_b_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "IVec4_Swap", &PROBE_N, &[(p_a_1) as u32, (p_b_2) as u32], false); }
    let r_ = 'f: {
    F::vu0_a__ee_lq(c, 0i32, p_a_1);
    F::vu0_a__ee_lq(c, 1i32, p_b_2);
    F::vu0_a__ee_sq(c, 0i32, p_b_2);
    F::vu0_a__ee_sq(c, 1i32, p_a_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "IVec4_Swap", &PROBE_N, &[], true); }
    r_
}

pub fn IVec4_Add(c: &mut Ctx, mut p_out_1: u32, mut p_a_2: u32, mut p_b_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "IVec4_Add", &PROBE_N, &[(p_out_1) as u32, (p_a_2) as u32, (p_b_3) as u32], false); }
    let r_ = 'f: {
    { let v_ = (c.m.r32((p_a_2).wrapping_add((((0i32) as u32)).wrapping_mul(4)))).wrapping_add(c.m.r32((p_b_3).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = (p_out_1).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = (c.m.r32((p_a_2).wrapping_add((((1i32) as u32)).wrapping_mul(4)))).wrapping_add(c.m.r32((p_b_3).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a_ = (p_out_1).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = (c.m.r32((p_a_2).wrapping_add((((2i32) as u32)).wrapping_mul(4)))).wrapping_add(c.m.r32((p_b_3).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); let a_ = (p_out_1).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = (c.m.r32((p_a_2).wrapping_add((((3i32) as u32)).wrapping_mul(4)))).wrapping_add(c.m.r32((p_b_3).wrapping_add((((3i32) as u32)).wrapping_mul(4)))); let a_ = (p_out_1).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "IVec4_Add", &PROBE_N, &[], true); }
    r_
}

pub fn IVec3_Add(c: &mut Ctx, mut p_out_1: u32, mut p_a_2: u32, mut p_b_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "IVec3_Add", &PROBE_N, &[(p_out_1) as u32, (p_a_2) as u32, (p_b_3) as u32], false); }
    let r_ = 'f: {
    { let v_ = (c.m.r32((p_a_2).wrapping_add((((0i32) as u32)).wrapping_mul(4)))).wrapping_add(c.m.r32((p_b_3).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = (p_out_1).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = (c.m.r32((p_a_2).wrapping_add((((1i32) as u32)).wrapping_mul(4)))).wrapping_add(c.m.r32((p_b_3).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a_ = (p_out_1).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = (c.m.r32((p_a_2).wrapping_add((((2i32) as u32)).wrapping_mul(4)))).wrapping_add(c.m.r32((p_b_3).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); let a_ = (p_out_1).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "IVec3_Add", &PROBE_N, &[], true); }
    r_
}

pub fn IVec4_Sub(c: &mut Ctx, mut p_out_1: u32, mut p_a_2: u32, mut p_b_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "IVec4_Sub", &PROBE_N, &[(p_out_1) as u32, (p_a_2) as u32, (p_b_3) as u32], false); }
    let r_ = 'f: {
    { let v_ = (c.m.r32((p_a_2).wrapping_add((((0i32) as u32)).wrapping_mul(4)))).wrapping_sub(c.m.r32((p_b_3).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = (p_out_1).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = (c.m.r32((p_a_2).wrapping_add((((1i32) as u32)).wrapping_mul(4)))).wrapping_sub(c.m.r32((p_b_3).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a_ = (p_out_1).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = (c.m.r32((p_a_2).wrapping_add((((2i32) as u32)).wrapping_mul(4)))).wrapping_sub(c.m.r32((p_b_3).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); let a_ = (p_out_1).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = (c.m.r32((p_a_2).wrapping_add((((3i32) as u32)).wrapping_mul(4)))).wrapping_sub(c.m.r32((p_b_3).wrapping_add((((3i32) as u32)).wrapping_mul(4)))); let a_ = (p_out_1).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "IVec4_Sub", &PROBE_N, &[], true); }
    r_
}

pub fn IVec3_Sub(c: &mut Ctx, mut p_out_1: u32, mut p_a_2: u32, mut p_b_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "IVec3_Sub", &PROBE_N, &[(p_out_1) as u32, (p_a_2) as u32, (p_b_3) as u32], false); }
    let r_ = 'f: {
    { let v_ = (c.m.r32((p_a_2).wrapping_add((((0i32) as u32)).wrapping_mul(4)))).wrapping_sub(c.m.r32((p_b_3).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = (p_out_1).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = (c.m.r32((p_a_2).wrapping_add((((1i32) as u32)).wrapping_mul(4)))).wrapping_sub(c.m.r32((p_b_3).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a_ = (p_out_1).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = (c.m.r32((p_a_2).wrapping_add((((2i32) as u32)).wrapping_mul(4)))).wrapping_sub(c.m.r32((p_b_3).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); let a_ = (p_out_1).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "IVec3_Sub", &PROBE_N, &[], true); }
    r_
}

pub fn IVec4_Mul(c: &mut Ctx, mut p_out_1: u32, mut p_a_2: u32, mut p_b_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "IVec4_Mul", &PROBE_N, &[(p_out_1) as u32, (p_a_2) as u32, (p_b_3) as u32], false); }
    let r_ = 'f: {
    { let v_ = (c.m.r32((p_a_2).wrapping_add((((0i32) as u32)).wrapping_mul(4)))).wrapping_mul(c.m.r32((p_b_3).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = (p_out_1).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = (c.m.r32((p_a_2).wrapping_add((((1i32) as u32)).wrapping_mul(4)))).wrapping_mul(c.m.r32((p_b_3).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a_ = (p_out_1).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = (c.m.r32((p_a_2).wrapping_add((((2i32) as u32)).wrapping_mul(4)))).wrapping_mul(c.m.r32((p_b_3).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); let a_ = (p_out_1).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = (c.m.r32((p_a_2).wrapping_add((((3i32) as u32)).wrapping_mul(4)))).wrapping_mul(c.m.r32((p_b_3).wrapping_add((((3i32) as u32)).wrapping_mul(4)))); let a_ = (p_out_1).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "IVec4_Mul", &PROBE_N, &[], true); }
    r_
}

pub fn IVec3_Mul(c: &mut Ctx, mut p_out_1: u32, mut p_a_2: u32, mut p_b_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "IVec3_Mul", &PROBE_N, &[(p_out_1) as u32, (p_a_2) as u32, (p_b_3) as u32], false); }
    let r_ = 'f: {
    { let v_ = (c.m.r32((p_a_2).wrapping_add((((0i32) as u32)).wrapping_mul(4)))).wrapping_mul(c.m.r32((p_b_3).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = (p_out_1).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = (c.m.r32((p_a_2).wrapping_add((((1i32) as u32)).wrapping_mul(4)))).wrapping_mul(c.m.r32((p_b_3).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); let a_ = (p_out_1).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = (c.m.r32((p_a_2).wrapping_add((((2i32) as u32)).wrapping_mul(4)))).wrapping_mul(c.m.r32((p_b_3).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); let a_ = (p_out_1).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "IVec3_Mul", &PROBE_N, &[], true); }
    r_
}

pub fn IVec4_Scale(c: &mut Ctx, mut p_out_1: u32, mut p_a_2: u32, mut p_s_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "IVec4_Scale", &PROBE_N, &[(p_out_1) as u32, (p_a_2) as u32, (p_s_3) as u32], false); }
    let r_ = 'f: {
    { let v_ = (c.m.r32((p_a_2).wrapping_add((((0i32) as u32)).wrapping_mul(4)))).wrapping_mul(((p_s_3) as u32)); let a_ = (p_out_1).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = (c.m.r32((p_a_2).wrapping_add((((1i32) as u32)).wrapping_mul(4)))).wrapping_mul(((p_s_3) as u32)); let a_ = (p_out_1).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = (c.m.r32((p_a_2).wrapping_add((((2i32) as u32)).wrapping_mul(4)))).wrapping_mul(((p_s_3) as u32)); let a_ = (p_out_1).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = (c.m.r32((p_a_2).wrapping_add((((3i32) as u32)).wrapping_mul(4)))).wrapping_mul(((p_s_3) as u32)); let a_ = (p_out_1).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "IVec4_Scale", &PROBE_N, &[], true); }
    r_
}

pub fn IVec3_Scale(c: &mut Ctx, mut p_out_1: u32, mut p_a_2: u32, mut p_s_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "IVec3_Scale", &PROBE_N, &[(p_out_1) as u32, (p_a_2) as u32, (p_s_3) as u32], false); }
    let r_ = 'f: {
    { let v_ = (c.m.r32((p_a_2).wrapping_add((((0i32) as u32)).wrapping_mul(4)))).wrapping_mul(((p_s_3) as u32)); let a_ = (p_out_1).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = (c.m.r32((p_a_2).wrapping_add((((1i32) as u32)).wrapping_mul(4)))).wrapping_mul(((p_s_3) as u32)); let a_ = (p_out_1).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = (c.m.r32((p_a_2).wrapping_add((((2i32) as u32)).wrapping_mul(4)))).wrapping_mul(((p_s_3) as u32)); let a_ = (p_out_1).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "IVec3_Scale", &PROBE_N, &[], true); }
    r_
}

pub fn vu0_a__idiv(c: &mut Ctx, mut p_a_1: i32, mut p_s_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__idiv", &PROBE_N, &[(p_a_1) as u32, (p_s_2) as u32], false); }
    let r_ = 'f: {
    if (((((((((p_a_1) == (((2147483647i32).wrapping_neg()).wrapping_sub(1i32))) as i32)) != 0) && (((((p_s_2) == ((1i32).wrapping_neg())) as i32)) != 0)) as i32)) != 0) {
        break 'f ((2147483647i32).wrapping_neg()).wrapping_sub(1i32);
    }
    break 'f rt::div_i32(p_a_1, p_s_2);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__idiv", &PROBE_N, &[], true); }
    r_
}

pub fn IVec4_Div(c: &mut Ctx, mut p_out_1: u32, mut p_a_2: u32, mut p_s_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "IVec4_Div", &PROBE_N, &[(p_out_1) as u32, (p_a_2) as u32, (p_s_3) as u32], false); }
    let r_ = 'f: {
    if (((((p_s_3) == (0i32)) as i32)) != 0) {
        break 'f;
    }
    { let v_ = { let a0_ = (c.m.r32((p_a_2).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32); F::vu0_a__idiv(c, a0_, p_s_3) }; let a_ = (p_out_1).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    { let v_ = { let a0_ = (c.m.r32((p_a_2).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32); F::vu0_a__idiv(c, a0_, p_s_3) }; let a_ = (p_out_1).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    { let v_ = { let a0_ = (c.m.r32((p_a_2).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32); F::vu0_a__idiv(c, a0_, p_s_3) }; let a_ = (p_out_1).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    { let v_ = { let a0_ = (c.m.r32((p_a_2).wrapping_add((((3i32) as u32)).wrapping_mul(4))) as i32); F::vu0_a__idiv(c, a0_, p_s_3) }; let a_ = (p_out_1).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "IVec4_Div", &PROBE_N, &[], true); }
    r_
}

pub fn IVec3_Div(c: &mut Ctx, mut p_out_1: u32, mut p_a_2: u32, mut p_s_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "IVec3_Div", &PROBE_N, &[(p_out_1) as u32, (p_a_2) as u32, (p_s_3) as u32], false); }
    let r_ = 'f: {
    if (((((p_s_3) == (0i32)) as i32)) != 0) {
        break 'f;
    }
    { let v_ = { let a0_ = (c.m.r32((p_a_2).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32); F::vu0_a__idiv(c, a0_, p_s_3) }; let a_ = (p_out_1).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    { let v_ = { let a0_ = (c.m.r32((p_a_2).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32); F::vu0_a__idiv(c, a0_, p_s_3) }; let a_ = (p_out_1).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    { let v_ = { let a0_ = (c.m.r32((p_a_2).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32); F::vu0_a__idiv(c, a0_, p_s_3) }; let a_ = (p_out_1).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "IVec3_Div", &PROBE_N, &[], true); }
    r_
}

pub fn IVec4_Copy(c: &mut Ctx, mut p_out_1: u32, mut p_in_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "IVec4_Copy", &PROBE_N, &[(p_out_1) as u32, (p_in_2) as u32], false); }
    let r_ = 'f: {
    F::vu0_a__ee_lq(c, 0i32, p_in_2);
    F::vu0_a__ee_sq(c, 0i32, p_out_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "IVec4_Copy", &PROBE_N, &[], true); }
    r_
}

pub fn IVec3_Copy(c: &mut Ctx, mut p_out_1: u32, mut p_in_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "IVec3_Copy", &PROBE_N, &[(p_out_1) as u32, (p_in_2) as u32], false); }
    let r_ = 'f: {
    { let v_ = c.m.r32((p_in_2).wrapping_add((((0i32) as u32)).wrapping_mul(4))); let a_ = (p_out_1).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = c.m.r32((p_in_2).wrapping_add((((1i32) as u32)).wrapping_mul(4))); let a_ = (p_out_1).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = c.m.r32((p_in_2).wrapping_add((((2i32) as u32)).wrapping_mul(4))); let a_ = (p_out_1).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "IVec3_Copy", &PROBE_N, &[], true); }
    r_
}

pub fn IVec4_ToFloat12(c: &mut Ctx, mut p_out_1: u32, mut p_in_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "IVec4_ToFloat12", &PROBE_N, &[(p_out_1) as u32, (p_in_2) as u32], false); }
    let r_ = 'f: {
    F::vu0_a__op_lqc2(c, 4i32, p_in_2);
    F::vu0_a__op_vitof12(c, 15i32, 5i32, 4i32);
    F::vu0_a__op_sqc2(c, 5i32, p_out_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "IVec4_ToFloat12", &PROBE_N, &[], true); }
    r_
}

pub fn IVec4_ToFloat4(c: &mut Ctx, mut p_out_1: u32, mut p_in_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "IVec4_ToFloat4", &PROBE_N, &[(p_out_1) as u32, (p_in_2) as u32], false); }
    let r_ = 'f: {
    F::vu0_a__op_lqc2(c, 4i32, p_in_2);
    F::vu0_a__op_vitof4(c, 15i32, 5i32, 4i32);
    F::vu0_a__op_sqc2(c, 5i32, p_out_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "IVec4_ToFloat4", &PROBE_N, &[], true); }
    r_
}

pub fn IVec4_ToFloat(c: &mut Ctx, mut p_out_1: u32, mut p_in_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "IVec4_ToFloat", &PROBE_N, &[(p_out_1) as u32, (p_in_2) as u32], false); }
    let r_ = 'f: {
    F::vu0_a__op_lqc2(c, 4i32, p_in_2);
    F::vu0_a__op_vitof0(c, 15i32, 5i32, 4i32);
    F::vu0_a__op_sqc2(c, 5i32, p_out_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "IVec4_ToFloat", &PROBE_N, &[], true); }
    r_
}

pub fn vu0_a__iclamp(c: &mut Ctx, mut p_v_1: i32, mut p_lo_2: i32, mut p_hi_3: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__iclamp", &PROBE_N, &[(p_v_1) as u32, (p_lo_2) as u32, (p_hi_3) as u32], false); }
    let r_ = 'f: {
    break 'f (if (((((p_v_1) < (p_lo_2)) as i32)) != 0) { p_lo_2 } else { (if (((((p_hi_3) < (p_v_1)) as i32)) != 0) { p_hi_3 } else { p_v_1 }) });
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "vu0_a__iclamp", &PROBE_N, &[], true); }
    r_
}

pub fn IVec4_Clamp(c: &mut Ctx, mut p_out_1: u32, mut p_in_2: u32, mut p_lo_3: i32, mut p_hi_4: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "IVec4_Clamp", &PROBE_N, &[(p_out_1) as u32, (p_in_2) as u32, (p_lo_3) as u32, (p_hi_4) as u32], false); }
    let r_ = 'f: {
    { let v_ = { let a0_ = (c.m.r32((p_in_2).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32); F::vu0_a__iclamp(c, a0_, p_lo_3, p_hi_4) }; let a_ = (p_out_1).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    { let v_ = { let a0_ = (c.m.r32((p_in_2).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32); F::vu0_a__iclamp(c, a0_, p_lo_3, p_hi_4) }; let a_ = (p_out_1).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    { let v_ = { let a0_ = (c.m.r32((p_in_2).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32); F::vu0_a__iclamp(c, a0_, p_lo_3, p_hi_4) }; let a_ = (p_out_1).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    { let v_ = { let a0_ = (c.m.r32((p_in_2).wrapping_add((((3i32) as u32)).wrapping_mul(4))) as i32); F::vu0_a__iclamp(c, a0_, p_lo_3, p_hi_4) }; let a_ = (p_out_1).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "IVec4_Clamp", &PROBE_N, &[], true); }
    r_
}

pub fn IVec3_Clamp(c: &mut Ctx, mut p_out_1: u32, mut p_in_2: u32, mut p_lo_3: i32, mut p_hi_4: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "IVec3_Clamp", &PROBE_N, &[(p_out_1) as u32, (p_in_2) as u32, (p_lo_3) as u32, (p_hi_4) as u32], false); }
    let r_ = 'f: {
    { let v_ = { let a0_ = (c.m.r32((p_in_2).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32); F::vu0_a__iclamp(c, a0_, p_lo_3, p_hi_4) }; let a_ = (p_out_1).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    { let v_ = { let a0_ = (c.m.r32((p_in_2).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32); F::vu0_a__iclamp(c, a0_, p_lo_3, p_hi_4) }; let a_ = (p_out_1).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    { let v_ = { let a0_ = (c.m.r32((p_in_2).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32); F::vu0_a__iclamp(c, a0_, p_lo_3, p_hi_4) }; let a_ = (p_out_1).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    { let v_ = (c.m.r32((p_in_2).wrapping_add((((3i32) as u32)).wrapping_mul(4))) as i32); let a_ = (p_out_1).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "IVec3_Clamp", &PROBE_N, &[], true); }
    r_
}

pub fn IVec_Stub(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "IVec_Stub", &PROBE_N, &[], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "IVec_Stub", &PROBE_N, &[], true); }
    r_
}

pub fn IVec4_InGsRange(c: &mut Ctx, mut p_p_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "IVec4_InGsRange", &PROBE_N, &[(p_p_1) as u32], false); }
    let mut l_v0_2: u32 = 0;
    let r_ = 'f: {
    F::vu0_a__op_lqc2(c, 7i32, p_p_1);
    F::vu0_a__op_vsub(c, 15i32, 4i32, 0i32, 0i32);
    F::vu0_a__ee_set64(c, 0i32, 1166016512u32, 1166016512u32);
    F::vu0_a__op_qmtc2(c, 0i32, 5i32);
    F::vu0_a__op_vitof4(c, 12i32, 7i32, 7i32);
    F::vu0_a__op_vitof0(c, 3i32, 7i32, 7i32);
    { let v_ = ((0i32) as u32); let a_ = (sym::gRefVu0).wrapping_add(564); c.m.w32(a_, v_); };
    F::vu0_a__op_vsub(c, 13i32, 0i32, 7i32, 4i32);
    F::vu0_a__op_vsub(c, 12i32, 0i32, 5i32, 7i32);
    l_v0_2 = c.m.r32((sym::gRefVu0).wrapping_add(564));
    { let r_ = ((192i32) as u32); let n_ = ((l_v0_2) & (r_)); l_v0_2 = n_; };
    break 'f (((l_v0_2) < (((1i32) as u32))) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "IVec4_InGsRange", &PROBE_N, &[], true); }
    r_
}

pub fn IVec4_InGsRange3(c: &mut Ctx, mut p_a_1: u32, mut p_b_2: u32, mut p_c_3: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "IVec4_InGsRange3", &PROBE_N, &[(p_a_1) as u32, (p_b_2) as u32, (p_c_3) as u32], false); }
    let mut l_v0_4: u32 = 0;
    let r_ = 'f: {
    F::vu0_a__op_lqc2(c, 7i32, p_a_1);
    F::vu0_a__op_lqc2(c, 8i32, p_b_2);
    F::vu0_a__op_lqc2(c, 9i32, p_c_3);
    F::vu0_a__op_vsub(c, 15i32, 4i32, 0i32, 0i32);
    F::vu0_a__ee_set64(c, 0i32, 1166016512u32, 1166016512u32);
    F::vu0_a__op_qmtc2(c, 0i32, 5i32);
    F::vu0_a__op_vitof4(c, 12i32, 7i32, 7i32);
    F::vu0_a__op_vitof0(c, 3i32, 7i32, 7i32);
    F::vu0_a__op_vitof4(c, 12i32, 8i32, 8i32);
    F::vu0_a__op_vitof0(c, 3i32, 8i32, 8i32);
    F::vu0_a__op_vitof4(c, 12i32, 9i32, 9i32);
    F::vu0_a__op_vitof0(c, 3i32, 9i32, 9i32);
    { let v_ = ((0i32) as u32); let a_ = (sym::gRefVu0).wrapping_add(564); c.m.w32(a_, v_); };
    F::vu0_a__op_vsub(c, 13i32, 0i32, 7i32, 4i32);
    F::vu0_a__op_vsub(c, 12i32, 0i32, 5i32, 7i32);
    F::vu0_a__op_vsub(c, 13i32, 0i32, 8i32, 4i32);
    F::vu0_a__op_vsub(c, 12i32, 0i32, 5i32, 8i32);
    F::vu0_a__op_vsub(c, 13i32, 0i32, 9i32, 4i32);
    F::vu0_a__op_vsub(c, 12i32, 0i32, 5i32, 9i32);
    l_v0_4 = c.m.r32((sym::gRefVu0).wrapping_add(564));
    { let r_ = ((192i32) as u32); let n_ = ((l_v0_4) & (r_)); l_v0_4 = n_; };
    break 'f (((l_v0_4) < (((1i32) as u32))) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "IVec4_InGsRange3", &PROBE_N, &[], true); }
    r_
}

pub fn IVec4_InGsRange4(c: &mut Ctx, mut p_a_1: u32, mut p_b_2: u32, mut p_c_3: u32, mut p_d_4: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "IVec4_InGsRange4", &PROBE_N, &[(p_a_1) as u32, (p_b_2) as u32, (p_c_3) as u32, (p_d_4) as u32], false); }
    let mut l_v0_5: u32 = 0;
    let r_ = 'f: {
    { let _ = p_d_4; };
    F::vu0_a__op_lqc2(c, 7i32, p_a_1);
    F::vu0_a__op_lqc2(c, 8i32, p_b_2);
    F::vu0_a__op_lqc2(c, 9i32, p_c_3);
    F::vu0_a__op_lqc2(c, 10i32, p_c_3);
    F::vu0_a__op_vsub(c, 15i32, 4i32, 0i32, 0i32);
    F::vu0_a__ee_set64(c, 0i32, 1166016512u32, 1166016512u32);
    F::vu0_a__op_qmtc2(c, 0i32, 5i32);
    F::vu0_a__op_vitof4(c, 12i32, 7i32, 7i32);
    F::vu0_a__op_vitof0(c, 3i32, 7i32, 7i32);
    F::vu0_a__op_vitof4(c, 12i32, 8i32, 8i32);
    F::vu0_a__op_vitof0(c, 3i32, 8i32, 8i32);
    F::vu0_a__op_vitof4(c, 12i32, 9i32, 9i32);
    F::vu0_a__op_vitof0(c, 3i32, 9i32, 9i32);
    F::vu0_a__op_vitof4(c, 12i32, 10i32, 10i32);
    F::vu0_a__op_vitof0(c, 3i32, 10i32, 10i32);
    { let v_ = ((0i32) as u32); let a_ = (sym::gRefVu0).wrapping_add(564); c.m.w32(a_, v_); };
    F::vu0_a__op_vsub(c, 13i32, 0i32, 7i32, 4i32);
    F::vu0_a__op_vsub(c, 12i32, 0i32, 5i32, 7i32);
    F::vu0_a__op_vsub(c, 13i32, 0i32, 8i32, 4i32);
    F::vu0_a__op_vsub(c, 12i32, 0i32, 5i32, 8i32);
    F::vu0_a__op_vsub(c, 13i32, 0i32, 9i32, 4i32);
    F::vu0_a__op_vsub(c, 12i32, 0i32, 5i32, 9i32);
    F::vu0_a__op_vsub(c, 13i32, 0i32, 10i32, 4i32);
    F::vu0_a__op_vsub(c, 12i32, 0i32, 5i32, 10i32);
    l_v0_5 = c.m.r32((sym::gRefVu0).wrapping_add(564));
    { let r_ = ((192i32) as u32); let n_ = ((l_v0_5) & (r_)); l_v0_5 = n_; };
    break 'f (((l_v0_5) < (((1i32) as u32))) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "IVec4_InGsRange4", &PROBE_N, &[], true); }
    r_
}

pub fn Vu0_SinCos(c: &mut Ctx, mut p_angle_1: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vu0_SinCos", &PROBE_N, &[(p_angle_1).0], false); }
    let mut l_f12_2: u32 = 0;
    let mut l_f0_3: u32 = 0;
    let mut l_neg_4: i32 = 0;
    let r_ = 'f: {
    l_f12_2 = F::vu0_a__f2u(c, p_angle_1);
    l_f0_3 = ((0i32) as u32);
    l_neg_4 = F::RefVu0_LtBits(c, l_f12_2, l_f0_3);
    l_f0_3 = 1070141403u32;
    if ((l_neg_4) != 0) {
        l_f12_2 = F::vu0_a__Sf_AddBits(c, l_f0_3, l_f12_2);
    } else {
        l_f12_2 = F::RefVu0_SubBits(c, l_f0_3, l_f12_2);
    }
    F::vu0_a__ee_mfc1(c, 0i32, l_f12_2);
    F::vu0_a__op_qmtc2(c, 0i32, 4i32);
    F::vu0_a__op_lqc2(c, 5i32, sym::vu0_a__sSinCoef);
    F::vu0_a__op_vmr32(c, 1i32, 4i32, 4i32);
    F::vu0_a__op_vaddbc(c, 8i32, 12i32, 0i32, 4i32, 0i32);
    F::vu0_a__op_vmul(c, 8i32, 4i32, 4i32, 4i32);
    F::vu0_a__op_vmulbc(c, 7i32, 12i32, 12i32, 0i32, 0i32);
    F::vu0_a__op_vmulbc(c, 15i32, 6i32, 5i32, 4i32, 3i32);
    F::vu0_a__op_vmulbc(c, 15i32, 6i32, 6i32, 4i32, 0i32);
    F::vu0_a__op_vmulbc(c, 14i32, 6i32, 6i32, 4i32, 0i32);
    F::vu0_a__op_vaddbc(c, 8i32, 12i32, 12i32, 6i32, 3i32);
    F::vu0_a__op_vmulbc(c, 12i32, 6i32, 6i32, 4i32, 0i32);
    F::vu0_a__op_vaddbc(c, 8i32, 12i32, 12i32, 6i32, 2i32);
    F::vu0_a__op_vmulbc(c, 8i32, 6i32, 6i32, 4i32, 0i32);
    F::vu0_a__op_vaddbc(c, 8i32, 12i32, 12i32, 6i32, 1i32);
    F::vu0_a__op_vaddbc(c, 8i32, 12i32, 12i32, 6i32, 0i32);
    F::vu0_a__op_vaddbc(c, 12i32, 12i32, 0i32, 12i32, 0i32);
    F::vu0_a__op_vmul(c, 8i32, 7i32, 12i32, 12i32);
    F::vu0_a__op_vsubbc(c, 1i32, 7i32, 0i32, 7i32, 0i32);
    F::vu0_a__op_vsqrt(c, 7i32, 3i32);
    F::vu0_a__op_vaddq(c, 8i32, 7i32, 0i32);
    if ((l_neg_4) != 0) {
        F::vu0_a__op_vsubbc(c, 8i32, 12i32, 0i32, 7i32, 0i32);
    } else {
        F::vu0_a__op_vaddbc(c, 8i32, 12i32, 0i32, 7i32, 0i32);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Vu0_SinCos", &PROBE_N, &[], true); }
    r_
}

pub fn Vu0_InitAxisRegs(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vu0_InitAxisRegs", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::vu0_a__op_vmr32(c, 15i32, 1i32, 0i32);
    F::vu0_a__op_vmr32(c, 15i32, 2i32, 1i32);
    F::vu0_a__op_vmr32(c, 15i32, 3i32, 2i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Vu0_InitAxisRegs", &PROBE_N, &[], true); }
    r_
}

pub fn Mtx_StoreIdentity(c: &mut Ctx, mut p_m_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Mtx_StoreIdentity", &PROBE_N, &[(p_m_1) as u32], false); }
    let r_ = 'f: {
    F::vu0_a__op_sqc2(c, 3i32, p_m_1);
    F::vu0_a__op_sqc2(c, 2i32, (p_m_1).wrapping_add(((16i32) as u32)));
    F::vu0_a__op_sqc2(c, 1i32, (p_m_1).wrapping_add(((32i32) as u32)));
    F::vu0_a__op_sqc2(c, 0i32, (p_m_1).wrapping_add(((48i32) as u32)));
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Mtx_StoreIdentity", &PROBE_N, &[], true); }
    r_
}

pub fn Mtx_StoreIdentityRot(c: &mut Ctx, mut p_m_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Mtx_StoreIdentityRot", &PROBE_N, &[(p_m_1) as u32], false); }
    let r_ = 'f: {
    F::vu0_a__op_sqc2(c, 3i32, p_m_1);
    F::vu0_a__op_sqc2(c, 2i32, (p_m_1).wrapping_add(((16i32) as u32)));
    F::vu0_a__op_sqc2(c, 1i32, (p_m_1).wrapping_add(((32i32) as u32)));
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Mtx_StoreIdentityRot", &PROBE_N, &[], true); }
    r_
}

pub fn Mtx_ClearTrans(c: &mut Ctx, mut p_m_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Mtx_ClearTrans", &PROBE_N, &[(p_m_1) as u32], false); }
    let r_ = 'f: {
    F::vu0_a__op_sqc2(c, 0i32, (p_m_1).wrapping_add(((48i32) as u32)));
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Mtx_ClearTrans", &PROBE_N, &[], true); }
    r_
}

pub fn Mtx_GetTrans(c: &mut Ctx, mut p_v_1: u32, mut p_m_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Mtx_GetTrans", &PROBE_N, &[(p_v_1) as u32, (p_m_2) as u32], false); }
    let r_ = 'f: {
    F::vu0_a__ee_lq(c, 0i32, (p_m_2).wrapping_add(((48i32) as u32)));
    F::vu0_a__ee_sq(c, 0i32, p_v_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Mtx_GetTrans", &PROBE_N, &[], true); }
    r_
}

pub fn Mtx_AxisZToEuler(c: &mut Ctx, mut p_angles_1: u32, mut p_m_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Mtx_AxisZToEuler", &PROBE_N, &[(p_angles_1) as u32, (p_m_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(96) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 96);
    let r_ = 'f: {
        { let s_ = sym::vu0_a__Mtx_AxisZToEuler__axisZW; rt::memcpy(c, fp.wrapping_add(80), s_, 16); }
        F::vu0_a__ee_lq(c, 0i32, p_m_2);
        F::vu0_a__ee_lq(c, 1i32, (p_m_2).wrapping_add(((16i32) as u32)));
        F::vu0_a__ee_lq(c, 2i32, (p_m_2).wrapping_add(((32i32) as u32)));
        F::vu0_a__ee_sq(c, 0i32, fp);
        F::vu0_a__ee_sq(c, 1i32, (fp).wrapping_add(((16i32) as u32)));
        F::vu0_a__ee_sq(c, 2i32, (fp).wrapping_add(((32i32) as u32)));
        F::vu0_a__op_sqc2(c, 0i32, (fp).wrapping_add(((48i32) as u32)));
        F::vu0_a__b_Mtx_MulVec4(c, fp.wrapping_add(64), fp, fp.wrapping_add(80));
        F::vu0_a__b_Vec3_DirToEuler(c, p_angles_1, fp.wrapping_add(64));
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Mtx_AxisZToEuler", &PROBE_N, &[], true); }
    r_
}

pub fn Mtx_SetTrans(c: &mut Ctx, mut p_m_1: u32, mut p_v_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Mtx_SetTrans", &PROBE_N, &[(p_m_1) as u32, (p_v_2) as u32], false); }
    let r_ = 'f: {
    F::vu0_a__ee_lq(c, 0i32, p_v_2);
    F::vu0_a__ee_sq(c, 0i32, (p_m_1).wrapping_add(((48i32) as u32)));
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Mtx_SetTrans", &PROBE_N, &[], true); }
    r_
}

pub fn Mtx_Translate(c: &mut Ctx, mut p_dst_1: u32, mut p_src_2: u32, mut p_v_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Mtx_Translate", &PROBE_N, &[(p_dst_1) as u32, (p_src_2) as u32, (p_v_3) as u32], false); }
    let r_ = 'f: {
    F::vu0_a__ee_lq(c, 0i32, p_src_2);
    F::vu0_a__ee_lq(c, 1i32, (p_src_2).wrapping_add(((16i32) as u32)));
    F::vu0_a__ee_lq(c, 2i32, (p_src_2).wrapping_add(((32i32) as u32)));
    F::vu0_a__ee_sq(c, 0i32, p_dst_1);
    F::vu0_a__ee_sq(c, 1i32, (p_dst_1).wrapping_add(((16i32) as u32)));
    F::vu0_a__ee_sq(c, 2i32, (p_dst_1).wrapping_add(((32i32) as u32)));
    F::vu0_a__op_lqc2(c, 4i32, p_v_3);
    F::vu0_a__op_lqc2(c, 5i32, (p_src_2).wrapping_add(((48i32) as u32)));
    F::vu0_a__op_vadd(c, 14i32, 5i32, 5i32, 4i32);
    F::vu0_a__op_sqc2(c, 5i32, (p_dst_1).wrapping_add(((48i32) as u32)));
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Mtx_Translate", &PROBE_N, &[], true); }
    r_
}

pub fn Mtx_TranslateLocal(c: &mut Ctx, mut p_dst_1: u32, mut p_src_2: u32, mut p_v_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Mtx_TranslateLocal", &PROBE_N, &[(p_dst_1) as u32, (p_src_2) as u32, (p_v_3) as u32], false); }
    let r_ = 'f: {
    F::vu0_a__op_lqc2(c, 8i32, p_v_3);
    F::vu0_a__op_lqc2(c, 4i32, p_src_2);
    F::vu0_a__op_lqc2(c, 5i32, (p_src_2).wrapping_add(((16i32) as u32)));
    F::vu0_a__op_lqc2(c, 6i32, (p_src_2).wrapping_add(((32i32) as u32)));
    F::vu0_a__op_lqc2(c, 7i32, (p_src_2).wrapping_add(((48i32) as u32)));
    F::vu0_a__op_vmulabc(c, 15i32, 4i32, 8i32, 0i32);
    F::vu0_a__op_vmaddabc(c, 15i32, 5i32, 8i32, 1i32);
    F::vu0_a__op_vmaddabc(c, 15i32, 6i32, 8i32, 2i32);
    F::vu0_a__op_vmaddbc(c, 15i32, 7i32, 7i32, 8i32, 3i32);
    F::vu0_a__op_sqc2(c, 4i32, p_dst_1);
    F::vu0_a__op_sqc2(c, 5i32, (p_dst_1).wrapping_add(((16i32) as u32)));
    F::vu0_a__op_sqc2(c, 6i32, (p_dst_1).wrapping_add(((32i32) as u32)));
    F::vu0_a__op_sqc2(c, 7i32, (p_dst_1).wrapping_add(((48i32) as u32)));
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Mtx_TranslateLocal", &PROBE_N, &[], true); }
    r_
}

pub fn Mtx_Mul(c: &mut Ctx, mut p_dst_1: u32, mut p_a_2: u32, mut p_b_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Mtx_Mul", &PROBE_N, &[(p_dst_1) as u32, (p_a_2) as u32, (p_b_3) as u32], false); }
    let r_ = 'f: {
    F::vu0_a__op_lqc2(c, 4i32, p_a_2);
    F::vu0_a__op_lqc2(c, 5i32, (p_a_2).wrapping_add(((16i32) as u32)));
    F::vu0_a__op_lqc2(c, 6i32, (p_a_2).wrapping_add(((32i32) as u32)));
    F::vu0_a__op_lqc2(c, 7i32, (p_a_2).wrapping_add(((48i32) as u32)));
    F::vu0_a__op_lqc2(c, 8i32, p_b_3);
    F::vu0_a__op_lqc2(c, 9i32, (p_b_3).wrapping_add(((16i32) as u32)));
    F::vu0_a__op_lqc2(c, 10i32, (p_b_3).wrapping_add(((32i32) as u32)));
    F::vu0_a__op_lqc2(c, 11i32, (p_b_3).wrapping_add(((48i32) as u32)));
    F::vu0_a__op_vmulabc(c, 15i32, 4i32, 8i32, 0i32);
    F::vu0_a__op_vmaddabc(c, 15i32, 5i32, 8i32, 1i32);
    F::vu0_a__op_vmaddabc(c, 15i32, 6i32, 8i32, 2i32);
    F::vu0_a__op_vmaddbc(c, 15i32, 8i32, 7i32, 8i32, 3i32);
    F::vu0_a__op_vmulabc(c, 15i32, 4i32, 9i32, 0i32);
    F::vu0_a__op_vmaddabc(c, 15i32, 5i32, 9i32, 1i32);
    F::vu0_a__op_vmaddabc(c, 15i32, 6i32, 9i32, 2i32);
    F::vu0_a__op_vmaddbc(c, 15i32, 9i32, 7i32, 9i32, 3i32);
    F::vu0_a__op_vmulabc(c, 15i32, 4i32, 10i32, 0i32);
    F::vu0_a__op_vmaddabc(c, 15i32, 5i32, 10i32, 1i32);
    F::vu0_a__op_vmaddabc(c, 15i32, 6i32, 10i32, 2i32);
    F::vu0_a__op_vmaddbc(c, 15i32, 10i32, 7i32, 10i32, 3i32);
    F::vu0_a__op_vmulabc(c, 15i32, 4i32, 11i32, 0i32);
    F::vu0_a__op_vmaddabc(c, 15i32, 5i32, 11i32, 1i32);
    F::vu0_a__op_vmaddabc(c, 15i32, 6i32, 11i32, 2i32);
    F::vu0_a__op_vmaddbc(c, 15i32, 11i32, 7i32, 11i32, 3i32);
    F::vu0_a__op_sqc2(c, 8i32, p_dst_1);
    F::vu0_a__op_sqc2(c, 9i32, (p_dst_1).wrapping_add(((16i32) as u32)));
    F::vu0_a__op_sqc2(c, 10i32, (p_dst_1).wrapping_add(((32i32) as u32)));
    F::vu0_a__op_sqc2(c, 11i32, (p_dst_1).wrapping_add(((48i32) as u32)));
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Mtx_Mul", &PROBE_N, &[], true); }
    r_
}

pub fn Mtx_Copy(c: &mut Ctx, mut p_dst_1: u32, mut p_src_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Mtx_Copy", &PROBE_N, &[(p_dst_1) as u32, (p_src_2) as u32], false); }
    let r_ = 'f: {
    F::vu0_a__ee_lq(c, 0i32, p_src_2);
    F::vu0_a__ee_lq(c, 1i32, (p_src_2).wrapping_add(((16i32) as u32)));
    F::vu0_a__ee_lq(c, 2i32, (p_src_2).wrapping_add(((32i32) as u32)));
    F::vu0_a__ee_lq(c, 3i32, (p_src_2).wrapping_add(((48i32) as u32)));
    F::vu0_a__ee_sq(c, 0i32, p_dst_1);
    F::vu0_a__ee_sq(c, 1i32, (p_dst_1).wrapping_add(((16i32) as u32)));
    F::vu0_a__ee_sq(c, 2i32, (p_dst_1).wrapping_add(((32i32) as u32)));
    F::vu0_a__ee_sq(c, 3i32, (p_dst_1).wrapping_add(((48i32) as u32)));
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Mtx_Copy", &PROBE_N, &[], true); }
    r_
}

pub fn Mtx_Transpose(c: &mut Ctx, mut p_dst_1: u32, mut p_src_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Mtx_Transpose", &PROBE_N, &[(p_dst_1) as u32, (p_src_2) as u32], false); }
    let r_ = 'f: {
    F::vu0_a__ee_lq(c, 0i32, p_src_2);
    F::vu0_a__ee_lq(c, 1i32, (p_src_2).wrapping_add(((16i32) as u32)));
    F::vu0_a__ee_lq(c, 2i32, (p_src_2).wrapping_add(((32i32) as u32)));
    F::vu0_a__ee_lq(c, 3i32, (p_src_2).wrapping_add(((48i32) as u32)));
    F::vu0_a__ee_pextlw(c, 4i32, 1i32, 0i32);
    F::vu0_a__ee_pextuw(c, 5i32, 1i32, 0i32);
    F::vu0_a__ee_pextlw(c, 6i32, 3i32, 2i32);
    F::vu0_a__ee_pextuw(c, 7i32, 3i32, 2i32);
    F::vu0_a__ee_pcpyld(c, 0i32, 6i32, 4i32);
    F::vu0_a__ee_pcpyud(c, 1i32, 4i32, 6i32);
    F::vu0_a__ee_pcpyld(c, 2i32, 7i32, 5i32);
    F::vu0_a__ee_pcpyud(c, 3i32, 5i32, 7i32);
    F::vu0_a__ee_sq(c, 0i32, p_dst_1);
    F::vu0_a__ee_sq(c, 1i32, (p_dst_1).wrapping_add(((16i32) as u32)));
    F::vu0_a__ee_sq(c, 2i32, (p_dst_1).wrapping_add(((32i32) as u32)));
    F::vu0_a__ee_sq(c, 3i32, (p_dst_1).wrapping_add(((48i32) as u32)));
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Mtx_Transpose", &PROBE_N, &[], true); }
    r_
}

pub fn Mtx_InverseRT(c: &mut Ctx, mut p_dst_1: u32, mut p_src_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Mtx_InverseRT", &PROBE_N, &[(p_dst_1) as u32, (p_src_2) as u32], false); }
    let r_ = 'f: {
    F::vu0_a__ee_lq(c, 0i32, p_src_2);
    F::vu0_a__ee_lq(c, 1i32, (p_src_2).wrapping_add(((16i32) as u32)));
    F::vu0_a__ee_lq(c, 2i32, (p_src_2).wrapping_add(((32i32) as u32)));
    F::vu0_a__op_lqc2(c, 4i32, (p_src_2).wrapping_add(((48i32) as u32)));
    F::vu0_a__op_vmove(c, 15i32, 5i32, 4i32);
    F::vu0_a__op_vmove(c, 14i32, 4i32, 0i32);
    F::vu0_a__op_qmfc2(c, 3i32, 4i32);
    F::vu0_a__ee_pextlw(c, 4i32, 1i32, 0i32);
    F::vu0_a__ee_pextuw(c, 5i32, 1i32, 0i32);
    F::vu0_a__ee_pextlw(c, 6i32, 3i32, 2i32);
    F::vu0_a__ee_pextuw(c, 7i32, 3i32, 2i32);
    F::vu0_a__ee_pcpyld(c, 0i32, 6i32, 4i32);
    F::vu0_a__ee_pcpyud(c, 1i32, 4i32, 6i32);
    F::vu0_a__ee_pcpyld(c, 2i32, 7i32, 5i32);
    F::vu0_a__op_qmtc2(c, 0i32, 6i32);
    F::vu0_a__op_qmtc2(c, 1i32, 7i32);
    F::vu0_a__op_qmtc2(c, 2i32, 8i32);
    F::vu0_a__op_vmulabc(c, 14i32, 6i32, 5i32, 0i32);
    F::vu0_a__op_vmaddabc(c, 14i32, 7i32, 5i32, 1i32);
    F::vu0_a__op_vmaddbc(c, 14i32, 4i32, 8i32, 5i32, 2i32);
    F::vu0_a__op_vsub(c, 14i32, 4i32, 0i32, 4i32);
    F::vu0_a__ee_sq(c, 0i32, p_dst_1);
    F::vu0_a__ee_sq(c, 1i32, (p_dst_1).wrapping_add(((16i32) as u32)));
    F::vu0_a__ee_sq(c, 2i32, (p_dst_1).wrapping_add(((32i32) as u32)));
    F::vu0_a__op_sqc2(c, 4i32, (p_dst_1).wrapping_add(((48i32) as u32)));
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Mtx_InverseRT", &PROBE_N, &[], true); }
    r_
}

pub fn Mtx_RotateZ(c: &mut Ctx, mut p_dst_1: u32, mut p_src_2: u32, mut p_angle_3: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Mtx_RotateZ", &PROBE_N, &[(p_dst_1) as u32, (p_src_2) as u32, (p_angle_3).0], false); }
    let r_ = 'f: {
    F::Vu0_SinCos(c, p_angle_3);
    F::vu0_a__op_lqc2(c, 4i32, p_src_2);
    F::vu0_a__op_lqc2(c, 5i32, (p_src_2).wrapping_add(((16i32) as u32)));
    F::vu0_a__op_lqc2(c, 6i32, (p_src_2).wrapping_add(((32i32) as u32)));
    F::vu0_a__op_lqc2(c, 7i32, (p_src_2).wrapping_add(((48i32) as u32)));
    F::vu0_a__op_vsub(c, 3i32, 8i32, 0i32, 0i32);
    F::vu0_a__op_vaddbc(c, 8i32, 8i32, 0i32, 12i32, 1i32);
    F::vu0_a__op_vaddbc(c, 4i32, 8i32, 0i32, 12i32, 0i32);
    F::vu0_a__op_vsub(c, 3i32, 9i32, 0i32, 0i32);
    F::vu0_a__op_vsubbc(c, 8i32, 9i32, 0i32, 12i32, 0i32);
    F::vu0_a__op_vaddbc(c, 4i32, 9i32, 0i32, 12i32, 1i32);
    F::vu0_a__op_vmulabc(c, 15i32, 8i32, 4i32, 0i32);
    F::vu0_a__op_vmaddabc(c, 15i32, 9i32, 4i32, 1i32);
    F::vu0_a__op_vmaddabc(c, 15i32, 1i32, 4i32, 2i32);
    F::vu0_a__op_vmaddbc(c, 15i32, 4i32, 0i32, 4i32, 3i32);
    F::vu0_a__op_vmulabc(c, 15i32, 8i32, 5i32, 0i32);
    F::vu0_a__op_vmaddabc(c, 15i32, 9i32, 5i32, 1i32);
    F::vu0_a__op_vmaddabc(c, 15i32, 1i32, 5i32, 2i32);
    F::vu0_a__op_vmaddbc(c, 15i32, 5i32, 0i32, 5i32, 3i32);
    F::vu0_a__op_vmulabc(c, 15i32, 8i32, 6i32, 0i32);
    F::vu0_a__op_vmaddabc(c, 15i32, 9i32, 6i32, 1i32);
    F::vu0_a__op_vmaddabc(c, 15i32, 1i32, 6i32, 2i32);
    F::vu0_a__op_vmaddbc(c, 15i32, 6i32, 0i32, 6i32, 3i32);
    F::vu0_a__op_vmulabc(c, 15i32, 8i32, 7i32, 0i32);
    F::vu0_a__op_vmaddabc(c, 15i32, 9i32, 7i32, 1i32);
    F::vu0_a__op_vmaddabc(c, 15i32, 1i32, 7i32, 2i32);
    F::vu0_a__op_vmaddbc(c, 15i32, 7i32, 0i32, 7i32, 3i32);
    F::vu0_a__op_sqc2(c, 4i32, p_dst_1);
    F::vu0_a__op_sqc2(c, 5i32, (p_dst_1).wrapping_add(((16i32) as u32)));
    F::vu0_a__op_sqc2(c, 6i32, (p_dst_1).wrapping_add(((32i32) as u32)));
    F::vu0_a__op_sqc2(c, 7i32, (p_dst_1).wrapping_add(((48i32) as u32)));
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Mtx_RotateZ", &PROBE_N, &[], true); }
    r_
}

pub fn Mtx_RotateX(c: &mut Ctx, mut p_dst_1: u32, mut p_src_2: u32, mut p_angle_3: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Mtx_RotateX", &PROBE_N, &[(p_dst_1) as u32, (p_src_2) as u32, (p_angle_3).0], false); }
    let r_ = 'f: {
    F::Vu0_SinCos(c, p_angle_3);
    F::vu0_a__op_lqc2(c, 4i32, p_src_2);
    F::vu0_a__op_lqc2(c, 5i32, (p_src_2).wrapping_add(((16i32) as u32)));
    F::vu0_a__op_lqc2(c, 6i32, (p_src_2).wrapping_add(((32i32) as u32)));
    F::vu0_a__op_lqc2(c, 7i32, (p_src_2).wrapping_add(((48i32) as u32)));
    F::vu0_a__op_vsub(c, 9i32, 8i32, 0i32, 0i32);
    F::vu0_a__op_vaddbc(c, 4i32, 8i32, 0i32, 12i32, 1i32);
    F::vu0_a__op_vaddbc(c, 2i32, 8i32, 0i32, 12i32, 0i32);
    F::vu0_a__op_vsub(c, 9i32, 9i32, 0i32, 0i32);
    F::vu0_a__op_vsubbc(c, 4i32, 9i32, 0i32, 12i32, 0i32);
    F::vu0_a__op_vaddbc(c, 2i32, 9i32, 0i32, 12i32, 1i32);
    F::vu0_a__op_vmulabc(c, 15i32, 3i32, 4i32, 0i32);
    F::vu0_a__op_vmaddabc(c, 15i32, 8i32, 4i32, 1i32);
    F::vu0_a__op_vmaddabc(c, 15i32, 9i32, 4i32, 2i32);
    F::vu0_a__op_vmaddbc(c, 15i32, 4i32, 0i32, 4i32, 3i32);
    F::vu0_a__op_vmulabc(c, 15i32, 3i32, 5i32, 0i32);
    F::vu0_a__op_vmaddabc(c, 15i32, 8i32, 5i32, 1i32);
    F::vu0_a__op_vmaddabc(c, 15i32, 9i32, 5i32, 2i32);
    F::vu0_a__op_vmaddbc(c, 15i32, 5i32, 0i32, 5i32, 3i32);
    F::vu0_a__op_vmulabc(c, 15i32, 3i32, 6i32, 0i32);
    F::vu0_a__op_vmaddabc(c, 15i32, 8i32, 6i32, 1i32);
    F::vu0_a__op_vmaddabc(c, 15i32, 9i32, 6i32, 2i32);
    F::vu0_a__op_vmaddbc(c, 15i32, 6i32, 0i32, 6i32, 3i32);
    F::vu0_a__op_vmulabc(c, 15i32, 3i32, 7i32, 0i32);
    F::vu0_a__op_vmaddabc(c, 15i32, 8i32, 7i32, 1i32);
    F::vu0_a__op_vmaddabc(c, 15i32, 9i32, 7i32, 2i32);
    F::vu0_a__op_vmaddbc(c, 15i32, 7i32, 0i32, 7i32, 3i32);
    F::vu0_a__op_sqc2(c, 4i32, p_dst_1);
    F::vu0_a__op_sqc2(c, 5i32, (p_dst_1).wrapping_add(((16i32) as u32)));
    F::vu0_a__op_sqc2(c, 6i32, (p_dst_1).wrapping_add(((32i32) as u32)));
    F::vu0_a__op_sqc2(c, 7i32, (p_dst_1).wrapping_add(((48i32) as u32)));
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Mtx_RotateX", &PROBE_N, &[], true); }
    r_
}

pub fn Mtx_RotateY(c: &mut Ctx, mut p_dst_1: u32, mut p_src_2: u32, mut p_angle_3: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Mtx_RotateY", &PROBE_N, &[(p_dst_1) as u32, (p_src_2) as u32, (p_angle_3).0], false); }
    let r_ = 'f: {
    F::Vu0_SinCos(c, p_angle_3);
    F::vu0_a__op_lqc2(c, 4i32, p_src_2);
    F::vu0_a__op_lqc2(c, 5i32, (p_src_2).wrapping_add(((16i32) as u32)));
    F::vu0_a__op_lqc2(c, 6i32, (p_src_2).wrapping_add(((32i32) as u32)));
    F::vu0_a__op_lqc2(c, 7i32, (p_src_2).wrapping_add(((48i32) as u32)));
    F::vu0_a__op_vsub(c, 5i32, 8i32, 0i32, 0i32);
    F::vu0_a__op_vaddbc(c, 8i32, 8i32, 0i32, 12i32, 1i32);
    F::vu0_a__op_vsubbc(c, 2i32, 8i32, 0i32, 12i32, 0i32);
    F::vu0_a__op_vsub(c, 5i32, 9i32, 0i32, 0i32);
    F::vu0_a__op_vaddbc(c, 8i32, 9i32, 0i32, 12i32, 0i32);
    F::vu0_a__op_vaddbc(c, 2i32, 9i32, 0i32, 12i32, 1i32);
    F::vu0_a__op_vmulabc(c, 15i32, 8i32, 4i32, 0i32);
    F::vu0_a__op_vmaddabc(c, 15i32, 2i32, 4i32, 1i32);
    F::vu0_a__op_vmaddabc(c, 15i32, 9i32, 4i32, 2i32);
    F::vu0_a__op_vmaddbc(c, 15i32, 4i32, 0i32, 4i32, 3i32);
    F::vu0_a__op_vmulabc(c, 15i32, 8i32, 5i32, 0i32);
    F::vu0_a__op_vmaddabc(c, 15i32, 2i32, 5i32, 1i32);
    F::vu0_a__op_vmaddabc(c, 15i32, 9i32, 5i32, 2i32);
    F::vu0_a__op_vmaddbc(c, 15i32, 5i32, 0i32, 5i32, 3i32);
    F::vu0_a__op_vmulabc(c, 15i32, 8i32, 6i32, 0i32);
    F::vu0_a__op_vmaddabc(c, 15i32, 2i32, 6i32, 1i32);
    F::vu0_a__op_vmaddabc(c, 15i32, 9i32, 6i32, 2i32);
    F::vu0_a__op_vmaddbc(c, 15i32, 6i32, 0i32, 6i32, 3i32);
    F::vu0_a__op_vmulabc(c, 15i32, 8i32, 7i32, 0i32);
    F::vu0_a__op_vmaddabc(c, 15i32, 2i32, 7i32, 1i32);
    F::vu0_a__op_vmaddabc(c, 15i32, 9i32, 7i32, 2i32);
    F::vu0_a__op_vmaddbc(c, 15i32, 7i32, 0i32, 7i32, 3i32);
    F::vu0_a__op_sqc2(c, 4i32, p_dst_1);
    F::vu0_a__op_sqc2(c, 5i32, (p_dst_1).wrapping_add(((16i32) as u32)));
    F::vu0_a__op_sqc2(c, 6i32, (p_dst_1).wrapping_add(((32i32) as u32)));
    F::vu0_a__op_sqc2(c, 7i32, (p_dst_1).wrapping_add(((48i32) as u32)));
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Mtx_RotateY", &PROBE_N, &[], true); }
    r_
}

pub fn Mtx_RotateZXY(c: &mut Ctx, mut p_dst_1: u32, mut p_src_2: u32, mut p_angles_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Mtx_RotateZXY", &PROBE_N, &[(p_dst_1) as u32, (p_src_2) as u32, (p_angles_3) as u32], false); }
    let r_ = 'f: {
    { let a2_ = F32(c.m.r32((p_angles_3).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); F::Mtx_RotateZ(c, p_dst_1, p_src_2, a2_) };
    { let a2_ = F32(c.m.r32((p_angles_3).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); F::Mtx_RotateX(c, p_dst_1, p_dst_1, a2_) };
    { let a2_ = F32(c.m.r32((p_angles_3).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); F::Mtx_RotateY(c, p_dst_1, p_dst_1, a2_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Mtx_RotateZXY", &PROBE_N, &[], true); }
    r_
}

pub fn Mtx_RotateXYZ(c: &mut Ctx, mut p_dst_1: u32, mut p_src_2: u32, mut p_angles_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Mtx_RotateXYZ", &PROBE_N, &[(p_dst_1) as u32, (p_src_2) as u32, (p_angles_3) as u32], false); }
    let r_ = 'f: {
    { let a2_ = F32(c.m.r32((p_angles_3).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); F::Mtx_RotateX(c, p_dst_1, p_src_2, a2_) };
    { let a2_ = F32(c.m.r32((p_angles_3).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); F::Mtx_RotateY(c, p_dst_1, p_dst_1, a2_) };
    { let a2_ = F32(c.m.r32((p_angles_3).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); F::Mtx_RotateZ(c, p_dst_1, p_dst_1, a2_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Mtx_RotateXYZ", &PROBE_N, &[], true); }
    r_
}

pub fn Mtx_ScaleDiag(c: &mut Ctx, mut p_dst_1: u32, mut p_src_2: u32, mut p_v_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Mtx_ScaleDiag", &PROBE_N, &[(p_dst_1) as u32, (p_src_2) as u32, (p_v_3) as u32], false); }
    let r_ = 'f: {
    F::vu0_a__op_lqc2(c, 8i32, p_v_3);
    F::vu0_a__op_lqc2(c, 4i32, p_src_2);
    F::vu0_a__op_lqc2(c, 5i32, (p_src_2).wrapping_add(((16i32) as u32)));
    F::vu0_a__op_lqc2(c, 6i32, (p_src_2).wrapping_add(((32i32) as u32)));
    F::vu0_a__op_lqc2(c, 7i32, (p_src_2).wrapping_add(((48i32) as u32)));
    F::vu0_a__op_vmulbc(c, 8i32, 4i32, 4i32, 8i32, 0i32);
    F::vu0_a__op_vmulbc(c, 4i32, 5i32, 5i32, 8i32, 1i32);
    F::vu0_a__op_vmulbc(c, 2i32, 6i32, 6i32, 8i32, 2i32);
    F::vu0_a__op_sqc2(c, 4i32, p_dst_1);
    F::vu0_a__op_sqc2(c, 5i32, (p_dst_1).wrapping_add(((16i32) as u32)));
    F::vu0_a__op_sqc2(c, 6i32, (p_dst_1).wrapping_add(((32i32) as u32)));
    F::vu0_a__op_sqc2(c, 7i32, (p_dst_1).wrapping_add(((48i32) as u32)));
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Mtx_ScaleDiag", &PROBE_N, &[], true); }
    r_
}

pub fn Mtx_ScaleDiagUniform(c: &mut Ctx, mut p_dst_1: u32, mut p_src_2: u32, mut p_s_3: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Mtx_ScaleDiagUniform", &PROBE_N, &[(p_dst_1) as u32, (p_src_2) as u32, (p_s_3).0], false); }
    let mut l_f12_4: u32 = 0;
    let r_ = 'f: {
    l_f12_4 = F::vu0_a__f2u(c, p_s_3);
    F::vu0_a__ee_mfc1(c, 0i32, l_f12_4);
    F::vu0_a__op_qmtc2(c, 0i32, 8i32);
    F::vu0_a__op_lqc2(c, 4i32, p_src_2);
    F::vu0_a__op_lqc2(c, 5i32, (p_src_2).wrapping_add(((16i32) as u32)));
    F::vu0_a__op_lqc2(c, 6i32, (p_src_2).wrapping_add(((32i32) as u32)));
    F::vu0_a__op_lqc2(c, 7i32, (p_src_2).wrapping_add(((48i32) as u32)));
    F::vu0_a__op_vmulbc(c, 8i32, 4i32, 4i32, 8i32, 0i32);
    F::vu0_a__op_vmulbc(c, 4i32, 5i32, 5i32, 8i32, 0i32);
    F::vu0_a__op_vmulbc(c, 2i32, 6i32, 6i32, 8i32, 0i32);
    F::vu0_a__op_sqc2(c, 4i32, p_dst_1);
    F::vu0_a__op_sqc2(c, 5i32, (p_dst_1).wrapping_add(((16i32) as u32)));
    F::vu0_a__op_sqc2(c, 6i32, (p_dst_1).wrapping_add(((32i32) as u32)));
    F::vu0_a__op_sqc2(c, 7i32, (p_dst_1).wrapping_add(((48i32) as u32)));
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Mtx_ScaleDiagUniform", &PROBE_N, &[], true); }
    r_
}

pub fn Mtx_RotateZXYTranslate(c: &mut Ctx, mut p_dst_1: u32, mut p_src_2: u32, mut p_angles_3: u32, mut p_pos_4: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Mtx_RotateZXYTranslate", &PROBE_N, &[(p_dst_1) as u32, (p_src_2) as u32, (p_angles_3) as u32, (p_pos_4) as u32], false); }
    let r_ = 'f: {
    F::Mtx_RotateZXY(c, p_dst_1, p_src_2, p_angles_3);
    F::Mtx_Translate(c, p_dst_1, p_src_2, p_pos_4);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Mtx_RotateZXYTranslate", &PROBE_N, &[], true); }
    r_
}

pub fn Mtx_MakeCamera(c: &mut Ctx, mut p_m_1: u32, mut p_pos_2: u32, mut p_zdir_3: u32, mut p_ydir_4: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Mtx_MakeCamera", &PROBE_N, &[(p_m_1) as u32, (p_pos_2) as u32, (p_zdir_3) as u32, (p_ydir_4) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(80) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 80);
    let r_ = 'f: {
        F::Mtx_StoreIdentity(c, fp);
        F::vu0_a__b_Vec3_Cross(c, fp.wrapping_add(64), p_ydir_4, p_zdir_3);
        F::vu0_a__b_Vec3_Normalize(c, (fp).wrapping_add((((0i32) as u32)).wrapping_mul(16)), fp.wrapping_add(64));
        F::vu0_a__b_Vec3_Normalize(c, (fp).wrapping_add((((2i32) as u32)).wrapping_mul(16)), p_zdir_3);
        F::vu0_a__b_Vec3_Cross(c, (fp).wrapping_add((((1i32) as u32)).wrapping_mul(16)), (fp).wrapping_add((((2i32) as u32)).wrapping_mul(16)), (fp).wrapping_add((((0i32) as u32)).wrapping_mul(16)));
        F::Mtx_Translate(c, fp, fp, p_pos_2);
        F::Mtx_InverseRT(c, p_m_1, fp);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Mtx_MakeCamera", &PROBE_N, &[], true); }
    r_
}

pub fn Mtx_SetRows(c: &mut Ctx, mut p_m_1: u32, mut p_r0_2: u32, mut p_r1_3: u32, mut p_r2_4: u32, mut p_r3_5: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Mtx_SetRows", &PROBE_N, &[(p_m_1) as u32, (p_r0_2) as u32, (p_r1_3) as u32, (p_r2_4) as u32, (p_r3_5) as u32], false); }
    let r_ = 'f: {
    F::vu0_a__b_Vec4_Copy(c, (p_m_1).wrapping_add((((0i32) as u32)).wrapping_mul(16)), p_r0_2);
    F::vu0_a__b_Vec4_Copy(c, (p_m_1).wrapping_add((((1i32) as u32)).wrapping_mul(16)), p_r1_3);
    F::vu0_a__b_Vec4_Copy(c, (p_m_1).wrapping_add((((2i32) as u32)).wrapping_mul(16)), p_r2_4);
    F::vu0_a__b_Vec4_Copy(c, (p_m_1).wrapping_add((((3i32) as u32)).wrapping_mul(16)), p_r3_5);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Mtx_SetRows", &PROBE_N, &[], true); }
    r_
}

pub fn Mtx_MakeNormalLight(c: &mut Ctx, mut p_m_1: u32, mut p_l0_2: u32, mut p_l1_3: u32, mut p_l2_4: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Mtx_MakeNormalLight", &PROBE_N, &[(p_m_1) as u32, (p_l0_2) as u32, (p_l1_3) as u32, (p_l2_4) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(80) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 80);
    let mut l_minusOne_5: u32 = 0;
    let r_ = 'f: {
        l_minusOne_5 = 3212836864u32;
        F::vu0_a__b_Vec4_Scale(c, fp.wrapping_add(64), p_l0_2, l_minusOne_5);
        F::vu0_a__b_Vec3_Normalize(c, (fp).wrapping_add((((0i32) as u32)).wrapping_mul(16)), fp.wrapping_add(64));
        F::vu0_a__b_Vec4_Scale(c, fp.wrapping_add(64), p_l1_3, l_minusOne_5);
        F::vu0_a__b_Vec3_Normalize(c, (fp).wrapping_add((((1i32) as u32)).wrapping_mul(16)), fp.wrapping_add(64));
        F::vu0_a__b_Vec4_Scale(c, fp.wrapping_add(64), p_l2_4, l_minusOne_5);
        F::vu0_a__b_Vec3_Normalize(c, (fp).wrapping_add((((2i32) as u32)).wrapping_mul(16)), fp.wrapping_add(64));
        F::vu0_a__b_Vec4_SetZeroW1(c, (fp).wrapping_add((((3i32) as u32)).wrapping_mul(16)));
        F::Mtx_Transpose(c, p_m_1, fp);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Mtx_MakeNormalLight", &PROBE_N, &[], true); }
    r_
}

pub fn Mtx_MakeViewScreen(c: &mut Ctx, mut p_m_1: u32, mut p_scrz_2: F32, mut p_ax_3: F32, mut p_ay_4: F32, mut p_cx_5: F32, mut p_cy_6: F32, mut p_zmin_7: F32, mut p_zmax_8: F32, mut p_nearz_9: F32, mut p_farz_10: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Mtx_MakeViewScreen", &PROBE_N, &[(p_m_1) as u32, (p_scrz_2).0, (p_ax_3).0, (p_ay_4).0, (p_cx_5).0, (p_cy_6).0, (p_zmin_7).0, (p_zmax_8).0, (p_nearz_9).0, (p_farz_10).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(128) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 128);
    let mut l_f0_11: u32 = 0;
    let mut l_f17_12: u32 = 0;
    let mut l_f18_13: u32 = 0;
    let mut l_f19_14: u32 = 0;
    let mut l_f20_15: u32 = 0;
    let mut l_f21_16: u32 = 0;
    let r_ = 'f: {
        l_f17_12 = F::vu0_a__f2u(c, p_zmin_7);
        l_f18_13 = F::vu0_a__f2u(c, p_zmax_8);
        l_f19_14 = F::vu0_a__f2u(c, p_nearz_9);
        l_f20_15 = ((l_f18_13) ^ (2147483648u32));
        l_f0_11 = F::vu0_a__f2u(c, p_farz_10);
        l_f18_13 = F::RefVu0_SubBits(c, l_f18_13, l_f17_12);
        l_f17_12 = F::vu0_a__Sf_MulBits(c, l_f17_12, l_f0_11);
        l_f21_16 = F::vu0_a__Sf_MulBits(c, l_f0_11, l_f19_14);
        l_f20_15 = F::vu0_a__Sf_MulBits(c, l_f20_15, l_f19_14);
        l_f0_11 = F::RefVu0_SubBits(c, l_f0_11, l_f19_14);
        l_f21_16 = F::vu0_a__Sf_MulBits(c, l_f21_16, l_f18_13);
        l_f20_15 = F::vu0_a__Sf_AddBits(c, l_f20_15, l_f17_12);
        l_f21_16 = F::RefVu0_DivBits(c, l_f21_16, l_f0_11);
        l_f20_15 = F::RefVu0_DivBits(c, l_f20_15, l_f0_11);
        F::Mtx_StoreIdentity(c, fp);
        { let v_ = p_scrz_2; let a_ = ((fp).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = p_scrz_2; let a_ = ((fp).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = 1065353216u32; let a_ = ((fp).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = ((fp).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = ((fp).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = 1065353216u32; let a_ = ((fp).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        F::Mtx_StoreIdentity(c, fp.wrapping_add(64));
        { let v_ = p_ax_3; let a_ = ((fp.wrapping_add(64)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = p_ay_4; let a_ = ((fp.wrapping_add(64)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = l_f21_16; let a_ = ((fp.wrapping_add(64)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = p_cx_5; let a_ = ((fp.wrapping_add(64)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = p_cy_6; let a_ = ((fp.wrapping_add(64)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = l_f20_15; let a_ = ((fp.wrapping_add(64)).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        F::Mtx_Mul(c, p_m_1, fp.wrapping_add(64), fp);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Mtx_MakeViewScreen", &PROBE_N, &[], true); }
    r_
}

pub fn Mtx_MakeDropShadow(c: &mut Ctx, mut p_m_1: u32, mut p_lp_2: u32, mut p_fa_3: F32, mut p_fb_4: F32, mut p_fc_5: F32, mut p_mode_6: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Mtx_MakeDropShadow", &PROBE_N, &[(p_m_1) as u32, (p_lp_2) as u32, (p_fa_3).0, (p_fb_4).0, (p_fc_5).0, (p_mode_6) as u32], false); }
    let mut l_a_7: u32 = 0;
    let mut l_b_8: u32 = 0;
    let mut l_c_9: u32 = 0;
    let mut l_one_10: u32 = 0;
    let mut l_x_11: u32 = 0;
    let mut l_y_12: u32 = 0;
    let mut l_z_13: u32 = 0;
    let mut l_ax_14: u32 = 0;
    let mut l_by_15: u32 = 0;
    let mut l_cz_16: u32 = 0;
    let mut l_d_17: u32 = 0;
    let mut l_bx_18: u32 = 0;
    let mut l_cx_19: u32 = 0;
    let mut l_ay_20: u32 = 0;
    let mut l_az_21: u32 = 0;
    let mut l_cy_22: u32 = 0;
    let mut l_bz_23: u32 = 0;
    let mut l_p_24: u32 = 0;
    let mut l_q_25: u32 = 0;
    let mut l_r_26: u32 = 0;
    let mut l_ap_27: u32 = 0;
    let mut l_bq_28: u32 = 0;
    let mut l_cr_29: u32 = 0;
    let mut l_cq_30: u32 = 0;
    let mut l_n_31: u32 = 0;
    let mut l_ar_32: u32 = 0;
    let mut l_br_33: u32 = 0;
    let mut l_bp_34: u32 = 0;
    let mut l_cp_35: u32 = 0;
    let mut l_nr_36: u32 = 0;
    let mut l_aq_37: u32 = 0;
    let r_ = 'f: {
    l_a_7 = F::vu0_a__f2u(c, p_fa_3);
    l_b_8 = F::vu0_a__f2u(c, p_fb_4);
    l_c_9 = F::vu0_a__f2u(c, p_fc_5);
    l_one_10 = 1065353216u32;
    if (((((p_mode_6) != (0i32)) as i32)) != 0) {
        l_x_11 = c.m.r32((p_lp_2).wrapping_add((((0i32) as u32)).wrapping_mul(4)));
        l_y_12 = c.m.r32((p_lp_2).wrapping_add((((1i32) as u32)).wrapping_mul(4)));
        l_z_13 = c.m.r32((p_lp_2).wrapping_add((((2i32) as u32)).wrapping_mul(4)));
        l_ax_14 = F::vu0_a__Sf_MulBits(c, l_a_7, l_x_11);
        l_by_15 = F::vu0_a__Sf_MulBits(c, l_b_8, l_y_12);
        l_cz_16 = F::vu0_a__Sf_MulBits(c, l_c_9, l_z_13);
        { let v_ = l_a_7; let a_ = ((p_m_1).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = l_b_8; let a_ = ((p_m_1).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = l_c_9; let a_ = ((p_m_1).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        l_d_17 = F::vu0_a__Sf_AddBits(c, l_ax_14, l_by_15);
        l_bx_18 = F::vu0_a__Sf_MulBits(c, l_b_8, l_x_11);
        { let v_ = ((l_x_11) ^ (2147483648u32)); let a_ = ((p_m_1).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        l_cx_19 = F::vu0_a__Sf_MulBits(c, l_c_9, l_x_11);
        { let v_ = ((l_y_12) ^ (2147483648u32)); let a_ = ((p_m_1).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        l_d_17 = F::vu0_a__Sf_AddBits(c, l_d_17, l_cz_16);
        l_ay_20 = F::vu0_a__Sf_MulBits(c, l_a_7, l_y_12);
        { let v_ = ((l_z_13) ^ (2147483648u32)); let a_ = ((p_m_1).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        l_az_21 = F::vu0_a__Sf_MulBits(c, l_a_7, l_z_13);
        { let v_ = l_bx_18; let a_ = ((p_m_1).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        l_cy_22 = F::vu0_a__Sf_MulBits(c, l_c_9, l_y_12);
        { let v_ = l_cx_19; let a_ = ((p_m_1).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        l_d_17 = F::RefVu0_SubBits(c, l_one_10, l_d_17);
        l_bz_23 = F::vu0_a__Sf_MulBits(c, l_b_8, l_z_13);
        { let v_ = l_ay_20; let a_ = ((p_m_1).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = l_az_21; let a_ = ((p_m_1).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = l_cy_22; let a_ = ((p_m_1).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = l_bz_23; let a_ = ((p_m_1).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = F::RefVu0_SubBits(c, l_d_17, l_one_10); let a_ = ((p_m_1).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = F::vu0_a__Sf_AddBits(c, l_ax_14, l_d_17); let a_ = ((p_m_1).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = F::vu0_a__Sf_AddBits(c, l_by_15, l_d_17); let a_ = ((p_m_1).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = F::vu0_a__Sf_AddBits(c, l_cz_16, l_d_17); let a_ = ((p_m_1).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    } else {
        l_p_24 = c.m.r32((p_lp_2).wrapping_add((((0i32) as u32)).wrapping_mul(4)));
        l_q_25 = c.m.r32((p_lp_2).wrapping_add((((1i32) as u32)).wrapping_mul(4)));
        l_r_26 = c.m.r32((p_lp_2).wrapping_add((((2i32) as u32)).wrapping_mul(4)));
        l_ap_27 = F::vu0_a__Sf_MulBits(c, l_a_7, l_p_24);
        l_bq_28 = F::vu0_a__Sf_MulBits(c, l_b_8, l_q_25);
        l_cr_29 = F::vu0_a__Sf_MulBits(c, l_c_9, l_r_26);
        { let v_ = ((0i32) as u32); let a_ = ((p_m_1).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = ((p_m_1).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        l_cq_30 = F::vu0_a__Sf_MulBits(c, l_c_9, l_q_25);
        { let v_ = ((0i32) as u32); let a_ = ((p_m_1).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        l_n_31 = F::vu0_a__Sf_AddBits(c, l_ap_27, l_bq_28);
        l_ar_32 = F::vu0_a__Sf_MulBits(c, l_a_7, l_r_26);
        l_br_33 = F::vu0_a__Sf_MulBits(c, l_b_8, l_r_26);
        l_n_31 = F::vu0_a__Sf_AddBits(c, l_n_31, l_cr_29);
        l_bp_34 = F::vu0_a__Sf_MulBits(c, l_b_8, l_p_24);
        l_cp_35 = F::vu0_a__Sf_MulBits(c, l_c_9, l_p_24);
        l_nr_36 = F::RefVu0_DivBits(c, ((l_one_10) ^ (2147483648u32)), l_n_31);
        l_ap_27 = F::RefVu0_SubBits(c, l_ap_27, l_n_31);
        l_bq_28 = F::RefVu0_SubBits(c, l_bq_28, l_n_31);
        l_cr_29 = F::RefVu0_SubBits(c, l_cr_29, l_n_31);
        l_aq_37 = F::vu0_a__Sf_MulBits(c, l_a_7, l_q_25);
        { let v_ = F::vu0_a__Sf_MulBits(c, l_nr_36, l_ap_27); let a_ = ((p_m_1).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = F::vu0_a__Sf_MulBits(c, l_nr_36, ((l_n_31) ^ (2147483648u32))); let a_ = ((p_m_1).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = F::vu0_a__Sf_MulBits(c, l_nr_36, l_bp_34); let a_ = ((p_m_1).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = F::vu0_a__Sf_MulBits(c, l_nr_36, l_cp_35); let a_ = ((p_m_1).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = F::vu0_a__Sf_MulBits(c, l_nr_36, ((l_p_24) ^ (2147483648u32))); let a_ = ((p_m_1).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = F::vu0_a__Sf_MulBits(c, l_nr_36, l_aq_37); let a_ = ((p_m_1).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = F::vu0_a__Sf_MulBits(c, l_nr_36, l_bq_28); let a_ = ((p_m_1).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = F::vu0_a__Sf_MulBits(c, l_nr_36, l_cq_30); let a_ = ((p_m_1).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = F::vu0_a__Sf_MulBits(c, l_nr_36, ((l_q_25) ^ (2147483648u32))); let a_ = ((p_m_1).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = F::vu0_a__Sf_MulBits(c, l_nr_36, l_ar_32); let a_ = ((p_m_1).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = F::vu0_a__Sf_MulBits(c, l_nr_36, l_br_33); let a_ = ((p_m_1).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = F::vu0_a__Sf_MulBits(c, l_nr_36, l_cr_29); let a_ = ((p_m_1).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = F::vu0_a__Sf_MulBits(c, l_nr_36, ((l_r_26) ^ (2147483648u32))); let a_ = ((p_m_1).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Mtx_MakeDropShadow", &PROBE_N, &[], true); }
    r_
}

pub fn Vu0Cur_Init(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vu0Cur_Init", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::vu0_a__op_vmove(c, 15i32, 19i32, 0i32);
    F::vu0_a__op_vmr32(c, 15i32, 18i32, 19i32);
    F::vu0_a__op_vmr32(c, 15i32, 17i32, 18i32);
    F::vu0_a__op_vmr32(c, 15i32, 16i32, 17i32);
    { let v_ = ((0i32) as u16); let a_ = ((sym::gRefVu0).wrapping_add(532)).wrapping_add((((15i32) as u32)).wrapping_mul(2)); c.m.w16(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Vu0Cur_Init", &PROBE_N, &[], true); }
    r_
}

pub fn Vu0Cur_LoadIdentity(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vu0Cur_LoadIdentity", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::vu0_a__op_vmove(c, 15i32, 16i32, 3i32);
    F::vu0_a__op_vmove(c, 15i32, 17i32, 2i32);
    F::vu0_a__op_vmove(c, 15i32, 18i32, 1i32);
    F::vu0_a__op_vmove(c, 15i32, 19i32, 0i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Vu0Cur_LoadIdentity", &PROBE_N, &[], true); }
    r_
}

pub fn Vu0Cur_Push(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vu0Cur_Push", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::vu0_a__op_vsqi(c, 16i32, 15i32);
    F::vu0_a__op_vsqi(c, 17i32, 15i32);
    F::vu0_a__op_vsqi(c, 18i32, 15i32);
    F::vu0_a__op_vsqi(c, 19i32, 15i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Vu0Cur_Push", &PROBE_N, &[], true); }
    r_
}

pub fn Vu0Cur_Pop(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vu0Cur_Pop", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::vu0_a__op_vlqd(c, 19i32, 15i32);
    F::vu0_a__op_vlqd(c, 18i32, 15i32);
    F::vu0_a__op_vlqd(c, 17i32, 15i32);
    F::vu0_a__op_vlqd(c, 16i32, 15i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Vu0Cur_Pop", &PROBE_N, &[], true); }
    r_
}

pub fn Vu0Cur_PopN(c: &mut Ctx, mut p_n_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vu0Cur_PopN", &PROBE_N, &[(p_n_1) as u32], false); }
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < (p_n_1)) as i32)) != 0) { break 'l1; }
            'c2: {
                F::Vu0Cur_Pop(c);
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Vu0Cur_PopN", &PROBE_N, &[], true); }
    r_
}

pub fn Vu0Cur_ResetStack(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vu0Cur_ResetStack", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let v_ = ((0i32) as u16); let a_ = ((sym::gRefVu0).wrapping_add(532)).wrapping_add((((15i32) as u32)).wrapping_mul(2)); c.m.w16(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Vu0Cur_ResetStack", &PROBE_N, &[], true); }
    r_
}

pub fn Vu0Cur_GetStackDepth(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vu0Cur_GetStackDepth", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f rt::div_i32(((((c.m.r16(((sym::gRefVu0).wrapping_add(532)).wrapping_add((((15i32) as u32)).wrapping_mul(2)))) as i16)) as i32), 4i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Vu0Cur_GetStackDepth", &PROBE_N, &[], true); }
    r_
}

pub fn Vu0Cur_IsStackUsed(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vu0Cur_IsStackUsed", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f (((F::Vu0Cur_GetStackDepth(c)) != (0i32)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Vu0Cur_IsStackUsed", &PROBE_N, &[], true); }
    r_
}

pub fn Vu0Cur_LoadIdentityRot(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vu0Cur_LoadIdentityRot", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::vu0_a__op_vmove(c, 15i32, 16i32, 3i32);
    F::vu0_a__op_vmove(c, 15i32, 17i32, 2i32);
    F::vu0_a__op_vmove(c, 15i32, 18i32, 1i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Vu0Cur_LoadIdentityRot", &PROBE_N, &[], true); }
    r_
}

pub fn Vu0Cur_ClearTrans(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vu0Cur_ClearTrans", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::vu0_a__op_vmove(c, 15i32, 19i32, 0i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Vu0Cur_ClearTrans", &PROBE_N, &[], true); }
    r_
}

pub fn Vu0Cur_LoadMtx(c: &mut Ctx, mut p_m_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vu0Cur_LoadMtx", &PROBE_N, &[(p_m_1) as u32], false); }
    let r_ = 'f: {
    F::vu0_a__op_lqc2(c, 16i32, p_m_1);
    F::vu0_a__op_lqc2(c, 17i32, (p_m_1).wrapping_add(((16i32) as u32)));
    F::vu0_a__op_lqc2(c, 18i32, (p_m_1).wrapping_add(((32i32) as u32)));
    F::vu0_a__op_lqc2(c, 19i32, (p_m_1).wrapping_add(((48i32) as u32)));
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Vu0Cur_LoadMtx", &PROBE_N, &[], true); }
    r_
}

pub fn Vu0Cur_StoreMtx(c: &mut Ctx, mut p_m_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vu0Cur_StoreMtx", &PROBE_N, &[(p_m_1) as u32], false); }
    let r_ = 'f: {
    F::vu0_a__op_sqc2(c, 16i32, p_m_1);
    F::vu0_a__op_sqc2(c, 17i32, (p_m_1).wrapping_add(((16i32) as u32)));
    F::vu0_a__op_sqc2(c, 18i32, (p_m_1).wrapping_add(((32i32) as u32)));
    F::vu0_a__op_sqc2(c, 19i32, (p_m_1).wrapping_add(((48i32) as u32)));
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Vu0Cur_StoreMtx", &PROBE_N, &[], true); }
    r_
}

pub fn Vu0Cur_GetTrans(c: &mut Ctx, mut p_v_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vu0Cur_GetTrans", &PROBE_N, &[(p_v_1) as u32], false); }
    let r_ = 'f: {
    F::vu0_a__op_sqc2(c, 19i32, p_v_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Vu0Cur_GetTrans", &PROBE_N, &[], true); }
    r_
}

pub fn Vu0Cur_AxisZToEuler(c: &mut Ctx, mut p_angles_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vu0Cur_AxisZToEuler", &PROBE_N, &[(p_angles_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(96) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 96);
    let r_ = 'f: {
        { let s_ = sym::vu0_a__Vu0Cur_AxisZToEuler__axisZW; rt::memcpy(c, fp.wrapping_add(80), s_, 16); }
        F::Vu0Cur_StoreMtx(c, fp.wrapping_add(16));
        F::vu0_a__b_Mtx_MulVec4(c, fp, fp.wrapping_add(16), fp.wrapping_add(80));
        F::vu0_a__b_Vec3_DirToEuler(c, p_angles_1, fp);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Vu0Cur_AxisZToEuler", &PROBE_N, &[], true); }
    r_
}

pub fn Vu0Cur_SetTrans(c: &mut Ctx, mut p_v_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vu0Cur_SetTrans", &PROBE_N, &[(p_v_1) as u32], false); }
    let r_ = 'f: {
    F::vu0_a__op_lqc2(c, 19i32, p_v_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Vu0Cur_SetTrans", &PROBE_N, &[], true); }
    r_
}

pub fn Vu0Cur_Translate(c: &mut Ctx, mut p_v_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vu0Cur_Translate", &PROBE_N, &[(p_v_1) as u32], false); }
    let r_ = 'f: {
    F::vu0_a__op_lqc2(c, 4i32, p_v_1);
    F::vu0_a__op_vadd(c, 14i32, 19i32, 19i32, 4i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Vu0Cur_Translate", &PROBE_N, &[], true); }
    r_
}

pub fn Vu0Cur_TranslateLocal(c: &mut Ctx, mut p_v_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vu0Cur_TranslateLocal", &PROBE_N, &[(p_v_1) as u32], false); }
    let r_ = 'f: {
    F::vu0_a__op_lqc2(c, 4i32, p_v_1);
    F::vu0_a__op_vmulabc(c, 15i32, 16i32, 4i32, 0i32);
    F::vu0_a__op_vmaddabc(c, 15i32, 17i32, 4i32, 1i32);
    F::vu0_a__op_vmaddabc(c, 15i32, 18i32, 4i32, 2i32);
    F::vu0_a__op_vmaddbc(c, 15i32, 19i32, 19i32, 4i32, 3i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Vu0Cur_TranslateLocal", &PROBE_N, &[], true); }
    r_
}

pub fn Vu0Cur_MulMtx(c: &mut Ctx, mut p_m_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vu0Cur_MulMtx", &PROBE_N, &[(p_m_1) as u32], false); }
    let r_ = 'f: {
    F::vu0_a__op_lqc2(c, 4i32, p_m_1);
    F::vu0_a__op_lqc2(c, 5i32, (p_m_1).wrapping_add(((16i32) as u32)));
    F::vu0_a__op_lqc2(c, 6i32, (p_m_1).wrapping_add(((32i32) as u32)));
    F::vu0_a__op_lqc2(c, 7i32, (p_m_1).wrapping_add(((48i32) as u32)));
    F::vu0_a__op_vmove(c, 15i32, 8i32, 16i32);
    F::vu0_a__op_vmove(c, 15i32, 9i32, 17i32);
    F::vu0_a__op_vmove(c, 15i32, 10i32, 18i32);
    F::vu0_a__op_vmove(c, 15i32, 11i32, 19i32);
    F::vu0_a__op_vmulabc(c, 15i32, 8i32, 4i32, 0i32);
    F::vu0_a__op_vmaddabc(c, 15i32, 9i32, 4i32, 1i32);
    F::vu0_a__op_vmaddabc(c, 15i32, 10i32, 4i32, 2i32);
    F::vu0_a__op_vmaddbc(c, 15i32, 16i32, 11i32, 4i32, 3i32);
    F::vu0_a__op_vmulabc(c, 15i32, 8i32, 5i32, 0i32);
    F::vu0_a__op_vmaddabc(c, 15i32, 9i32, 5i32, 1i32);
    F::vu0_a__op_vmaddabc(c, 15i32, 10i32, 5i32, 2i32);
    F::vu0_a__op_vmaddbc(c, 15i32, 17i32, 11i32, 5i32, 3i32);
    F::vu0_a__op_vmulabc(c, 15i32, 8i32, 6i32, 0i32);
    F::vu0_a__op_vmaddabc(c, 15i32, 9i32, 6i32, 1i32);
    F::vu0_a__op_vmaddabc(c, 15i32, 10i32, 6i32, 2i32);
    F::vu0_a__op_vmaddbc(c, 15i32, 18i32, 11i32, 6i32, 3i32);
    F::vu0_a__op_vmulabc(c, 15i32, 8i32, 7i32, 0i32);
    F::vu0_a__op_vmaddabc(c, 15i32, 9i32, 7i32, 1i32);
    F::vu0_a__op_vmaddabc(c, 15i32, 10i32, 7i32, 2i32);
    F::vu0_a__op_vmaddbc(c, 15i32, 19i32, 11i32, 7i32, 3i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Vu0Cur_MulMtx", &PROBE_N, &[], true); }
    r_
}

pub fn Vu0Cur_MulMtxRev(c: &mut Ctx, mut p_m_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vu0Cur_MulMtxRev", &PROBE_N, &[(p_m_1) as u32], false); }
    let r_ = 'f: {
    F::vu0_a__op_lqc2(c, 4i32, p_m_1);
    F::vu0_a__op_lqc2(c, 5i32, (p_m_1).wrapping_add(((16i32) as u32)));
    F::vu0_a__op_lqc2(c, 6i32, (p_m_1).wrapping_add(((32i32) as u32)));
    F::vu0_a__op_lqc2(c, 7i32, (p_m_1).wrapping_add(((48i32) as u32)));
    F::vu0_a__op_vmulabc(c, 15i32, 4i32, 16i32, 0i32);
    F::vu0_a__op_vmaddabc(c, 15i32, 5i32, 16i32, 1i32);
    F::vu0_a__op_vmaddabc(c, 15i32, 6i32, 16i32, 2i32);
    F::vu0_a__op_vmaddbc(c, 15i32, 16i32, 7i32, 16i32, 3i32);
    F::vu0_a__op_vmulabc(c, 15i32, 4i32, 17i32, 0i32);
    F::vu0_a__op_vmaddabc(c, 15i32, 5i32, 17i32, 1i32);
    F::vu0_a__op_vmaddabc(c, 15i32, 6i32, 17i32, 2i32);
    F::vu0_a__op_vmaddbc(c, 15i32, 17i32, 7i32, 17i32, 3i32);
    F::vu0_a__op_vmulabc(c, 15i32, 4i32, 18i32, 0i32);
    F::vu0_a__op_vmaddabc(c, 15i32, 5i32, 18i32, 1i32);
    F::vu0_a__op_vmaddabc(c, 15i32, 6i32, 18i32, 2i32);
    F::vu0_a__op_vmaddbc(c, 15i32, 18i32, 7i32, 18i32, 3i32);
    F::vu0_a__op_vmulabc(c, 15i32, 4i32, 19i32, 0i32);
    F::vu0_a__op_vmaddabc(c, 15i32, 5i32, 19i32, 1i32);
    F::vu0_a__op_vmaddabc(c, 15i32, 6i32, 19i32, 2i32);
    F::vu0_a__op_vmaddbc(c, 15i32, 19i32, 7i32, 19i32, 3i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Vu0Cur_MulMtxRev", &PROBE_N, &[], true); }
    r_
}

pub fn Vu0Cur_Transpose(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vu0Cur_Transpose", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::vu0_a__op_qmfc2(c, 0i32, 16i32);
    F::vu0_a__op_qmfc2(c, 1i32, 17i32);
    F::vu0_a__op_qmfc2(c, 2i32, 18i32);
    F::vu0_a__op_qmfc2(c, 3i32, 19i32);
    F::vu0_a__ee_pextlw(c, 4i32, 1i32, 0i32);
    F::vu0_a__ee_pextuw(c, 5i32, 1i32, 0i32);
    F::vu0_a__ee_pextlw(c, 6i32, 3i32, 2i32);
    F::vu0_a__ee_pextuw(c, 7i32, 3i32, 2i32);
    F::vu0_a__ee_pcpyld(c, 0i32, 6i32, 4i32);
    F::vu0_a__ee_pcpyud(c, 1i32, 4i32, 6i32);
    F::vu0_a__ee_pcpyld(c, 2i32, 7i32, 5i32);
    F::vu0_a__ee_pcpyud(c, 3i32, 5i32, 7i32);
    F::vu0_a__op_qmtc2(c, 0i32, 16i32);
    F::vu0_a__op_qmtc2(c, 1i32, 17i32);
    F::vu0_a__op_qmtc2(c, 2i32, 18i32);
    F::vu0_a__op_qmtc2(c, 3i32, 19i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Vu0Cur_Transpose", &PROBE_N, &[], true); }
    r_
}

pub fn Vu0Cur_InverseRT(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vu0Cur_InverseRT", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::vu0_a__op_qmfc2(c, 0i32, 16i32);
    F::vu0_a__op_qmfc2(c, 1i32, 17i32);
    F::vu0_a__op_qmfc2(c, 2i32, 18i32);
    F::vu0_a__op_vmove(c, 15i32, 4i32, 19i32);
    F::vu0_a__op_vmove(c, 14i32, 4i32, 0i32);
    F::vu0_a__op_qmfc2(c, 3i32, 4i32);
    F::vu0_a__ee_pextlw(c, 4i32, 1i32, 0i32);
    F::vu0_a__ee_pextuw(c, 5i32, 1i32, 0i32);
    F::vu0_a__ee_pextlw(c, 6i32, 3i32, 2i32);
    F::vu0_a__ee_pextuw(c, 7i32, 3i32, 2i32);
    F::vu0_a__ee_pcpyld(c, 0i32, 6i32, 4i32);
    F::vu0_a__ee_pcpyud(c, 1i32, 4i32, 6i32);
    F::vu0_a__ee_pcpyld(c, 2i32, 7i32, 5i32);
    F::vu0_a__op_qmtc2(c, 0i32, 6i32);
    F::vu0_a__op_qmtc2(c, 1i32, 7i32);
    F::vu0_a__op_qmtc2(c, 2i32, 8i32);
    F::vu0_a__op_vmulabc(c, 14i32, 6i32, 19i32, 0i32);
    F::vu0_a__op_vmaddabc(c, 14i32, 7i32, 19i32, 1i32);
    F::vu0_a__op_vmaddbc(c, 14i32, 4i32, 8i32, 19i32, 2i32);
    F::vu0_a__op_vsub(c, 14i32, 4i32, 0i32, 4i32);
    F::vu0_a__op_qmtc2(c, 0i32, 16i32);
    F::vu0_a__op_qmtc2(c, 1i32, 17i32);
    F::vu0_a__op_qmtc2(c, 2i32, 18i32);
    F::vu0_a__op_vmove(c, 15i32, 19i32, 4i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Vu0Cur_InverseRT", &PROBE_N, &[], true); }
    r_
}

pub fn Vu0Cur_RotateZ(c: &mut Ctx, mut p_angle_1: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vu0Cur_RotateZ", &PROBE_N, &[(p_angle_1).0], false); }
    let r_ = 'f: {
    F::Vu0_SinCos(c, p_angle_1);
    F::vu0_a__op_vsub(c, 3i32, 8i32, 0i32, 0i32);
    F::vu0_a__op_vaddbc(c, 8i32, 8i32, 0i32, 12i32, 1i32);
    F::vu0_a__op_vaddbc(c, 4i32, 8i32, 0i32, 12i32, 0i32);
    F::vu0_a__op_vsub(c, 3i32, 9i32, 0i32, 0i32);
    F::vu0_a__op_vsubbc(c, 8i32, 9i32, 0i32, 12i32, 0i32);
    F::vu0_a__op_vaddbc(c, 4i32, 9i32, 0i32, 12i32, 1i32);
    F::vu0_a__op_vmulabc(c, 15i32, 8i32, 16i32, 0i32);
    F::vu0_a__op_vmaddabc(c, 15i32, 9i32, 16i32, 1i32);
    F::vu0_a__op_vmaddabc(c, 15i32, 1i32, 16i32, 2i32);
    F::vu0_a__op_vmaddbc(c, 15i32, 16i32, 0i32, 16i32, 3i32);
    F::vu0_a__op_vmulabc(c, 15i32, 8i32, 17i32, 0i32);
    F::vu0_a__op_vmaddabc(c, 15i32, 9i32, 17i32, 1i32);
    F::vu0_a__op_vmaddabc(c, 15i32, 1i32, 17i32, 2i32);
    F::vu0_a__op_vmaddbc(c, 15i32, 17i32, 0i32, 17i32, 3i32);
    F::vu0_a__op_vmulabc(c, 15i32, 8i32, 18i32, 0i32);
    F::vu0_a__op_vmaddabc(c, 15i32, 9i32, 18i32, 1i32);
    F::vu0_a__op_vmaddabc(c, 15i32, 1i32, 18i32, 2i32);
    F::vu0_a__op_vmaddbc(c, 15i32, 18i32, 0i32, 18i32, 3i32);
    F::vu0_a__op_vmulabc(c, 15i32, 8i32, 19i32, 0i32);
    F::vu0_a__op_vmaddabc(c, 15i32, 9i32, 19i32, 1i32);
    F::vu0_a__op_vmaddabc(c, 15i32, 1i32, 19i32, 2i32);
    F::vu0_a__op_vmaddbc(c, 15i32, 19i32, 0i32, 19i32, 3i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Vu0Cur_RotateZ", &PROBE_N, &[], true); }
    r_
}

pub fn Vu0Cur_RotateX(c: &mut Ctx, mut p_angle_1: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vu0Cur_RotateX", &PROBE_N, &[(p_angle_1).0], false); }
    let r_ = 'f: {
    F::Vu0_SinCos(c, p_angle_1);
    F::vu0_a__op_vsub(c, 9i32, 8i32, 0i32, 0i32);
    F::vu0_a__op_vaddbc(c, 4i32, 8i32, 0i32, 12i32, 1i32);
    F::vu0_a__op_vaddbc(c, 2i32, 8i32, 0i32, 12i32, 0i32);
    F::vu0_a__op_vsub(c, 9i32, 9i32, 0i32, 0i32);
    F::vu0_a__op_vsubbc(c, 4i32, 9i32, 0i32, 12i32, 0i32);
    F::vu0_a__op_vaddbc(c, 2i32, 9i32, 0i32, 12i32, 1i32);
    F::vu0_a__op_vmulabc(c, 15i32, 3i32, 16i32, 0i32);
    F::vu0_a__op_vmaddabc(c, 15i32, 8i32, 16i32, 1i32);
    F::vu0_a__op_vmaddabc(c, 15i32, 9i32, 16i32, 2i32);
    F::vu0_a__op_vmaddbc(c, 15i32, 16i32, 0i32, 16i32, 3i32);
    F::vu0_a__op_vmulabc(c, 15i32, 3i32, 17i32, 0i32);
    F::vu0_a__op_vmaddabc(c, 15i32, 8i32, 17i32, 1i32);
    F::vu0_a__op_vmaddabc(c, 15i32, 9i32, 17i32, 2i32);
    F::vu0_a__op_vmaddbc(c, 15i32, 17i32, 0i32, 17i32, 3i32);
    F::vu0_a__op_vmulabc(c, 15i32, 3i32, 18i32, 0i32);
    F::vu0_a__op_vmaddabc(c, 15i32, 8i32, 18i32, 1i32);
    F::vu0_a__op_vmaddabc(c, 15i32, 9i32, 18i32, 2i32);
    F::vu0_a__op_vmaddbc(c, 15i32, 18i32, 0i32, 18i32, 3i32);
    F::vu0_a__op_vmulabc(c, 15i32, 3i32, 19i32, 0i32);
    F::vu0_a__op_vmaddabc(c, 15i32, 8i32, 19i32, 1i32);
    F::vu0_a__op_vmaddabc(c, 15i32, 9i32, 19i32, 2i32);
    F::vu0_a__op_vmaddbc(c, 15i32, 19i32, 0i32, 19i32, 3i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Vu0Cur_RotateX", &PROBE_N, &[], true); }
    r_
}

pub fn Vu0Cur_RotateY(c: &mut Ctx, mut p_angle_1: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vu0Cur_RotateY", &PROBE_N, &[(p_angle_1).0], false); }
    let r_ = 'f: {
    F::Vu0_SinCos(c, p_angle_1);
    F::vu0_a__op_vsub(c, 5i32, 8i32, 0i32, 0i32);
    F::vu0_a__op_vaddbc(c, 8i32, 8i32, 0i32, 12i32, 1i32);
    F::vu0_a__op_vsubbc(c, 2i32, 8i32, 0i32, 12i32, 0i32);
    F::vu0_a__op_vsub(c, 5i32, 9i32, 0i32, 0i32);
    F::vu0_a__op_vaddbc(c, 8i32, 9i32, 0i32, 12i32, 0i32);
    F::vu0_a__op_vaddbc(c, 2i32, 9i32, 0i32, 12i32, 1i32);
    F::vu0_a__op_vmulabc(c, 15i32, 8i32, 16i32, 0i32);
    F::vu0_a__op_vmaddabc(c, 15i32, 2i32, 16i32, 1i32);
    F::vu0_a__op_vmaddabc(c, 15i32, 9i32, 16i32, 2i32);
    F::vu0_a__op_vmaddbc(c, 15i32, 16i32, 0i32, 16i32, 3i32);
    F::vu0_a__op_vmulabc(c, 15i32, 8i32, 17i32, 0i32);
    F::vu0_a__op_vmaddabc(c, 15i32, 2i32, 17i32, 1i32);
    F::vu0_a__op_vmaddabc(c, 15i32, 9i32, 17i32, 2i32);
    F::vu0_a__op_vmaddbc(c, 15i32, 17i32, 0i32, 17i32, 3i32);
    F::vu0_a__op_vmulabc(c, 15i32, 8i32, 18i32, 0i32);
    F::vu0_a__op_vmaddabc(c, 15i32, 2i32, 18i32, 1i32);
    F::vu0_a__op_vmaddabc(c, 15i32, 9i32, 18i32, 2i32);
    F::vu0_a__op_vmaddbc(c, 15i32, 18i32, 0i32, 18i32, 3i32);
    F::vu0_a__op_vmulabc(c, 15i32, 8i32, 19i32, 0i32);
    F::vu0_a__op_vmaddabc(c, 15i32, 2i32, 19i32, 1i32);
    F::vu0_a__op_vmaddabc(c, 15i32, 9i32, 19i32, 2i32);
    F::vu0_a__op_vmaddbc(c, 15i32, 19i32, 0i32, 19i32, 3i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Vu0Cur_RotateY", &PROBE_N, &[], true); }
    r_
}

pub fn Vu0Cur_RotateZXY(c: &mut Ctx, mut p_angles_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vu0Cur_RotateZXY", &PROBE_N, &[(p_angles_1) as u32], false); }
    let r_ = 'f: {
    { let a0_ = F32(c.m.r32((p_angles_1).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); F::Vu0Cur_RotateZ(c, a0_) };
    { let a0_ = F32(c.m.r32((p_angles_1).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); F::Vu0Cur_RotateX(c, a0_) };
    { let a0_ = F32(c.m.r32((p_angles_1).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); F::Vu0Cur_RotateY(c, a0_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Vu0Cur_RotateZXY", &PROBE_N, &[], true); }
    r_
}

pub fn Vu0Cur_RotateXYZ(c: &mut Ctx, mut p_angles_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vu0Cur_RotateXYZ", &PROBE_N, &[(p_angles_1) as u32], false); }
    let r_ = 'f: {
    { let a0_ = F32(c.m.r32((p_angles_1).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); F::Vu0Cur_RotateX(c, a0_) };
    { let a0_ = F32(c.m.r32((p_angles_1).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); F::Vu0Cur_RotateY(c, a0_) };
    { let a0_ = F32(c.m.r32((p_angles_1).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); F::Vu0Cur_RotateZ(c, a0_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Vu0Cur_RotateXYZ", &PROBE_N, &[], true); }
    r_
}

pub fn Vu0Cur_ScaleDiag(c: &mut Ctx, mut p_v_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vu0Cur_ScaleDiag", &PROBE_N, &[(p_v_1) as u32], false); }
    let r_ = 'f: {
    F::vu0_a__op_lqc2(c, 4i32, p_v_1);
    F::vu0_a__op_vmulbc(c, 8i32, 16i32, 16i32, 4i32, 0i32);
    F::vu0_a__op_vmulbc(c, 4i32, 17i32, 17i32, 4i32, 1i32);
    F::vu0_a__op_vmulbc(c, 2i32, 18i32, 18i32, 4i32, 2i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Vu0Cur_ScaleDiag", &PROBE_N, &[], true); }
    r_
}

pub fn Vu0Cur_ScaleDiagUniform(c: &mut Ctx, mut p_s_1: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vu0Cur_ScaleDiagUniform", &PROBE_N, &[(p_s_1).0], false); }
    let mut l_f12_2: u32 = 0;
    let r_ = 'f: {
    l_f12_2 = F::vu0_a__f2u(c, p_s_1);
    F::vu0_a__ee_mfc1(c, 0i32, l_f12_2);
    F::vu0_a__op_qmtc2(c, 0i32, 4i32);
    F::vu0_a__op_vmulbc(c, 8i32, 16i32, 16i32, 4i32, 0i32);
    F::vu0_a__op_vmulbc(c, 4i32, 17i32, 17i32, 4i32, 0i32);
    F::vu0_a__op_vmulbc(c, 2i32, 18i32, 18i32, 4i32, 0i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Vu0Cur_ScaleDiagUniform", &PROBE_N, &[], true); }
    r_
}

pub fn Vu0Cur_RotateZXYTranslate(c: &mut Ctx, mut p_angles_1: u32, mut p_pos_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vu0Cur_RotateZXYTranslate", &PROBE_N, &[(p_angles_1) as u32, (p_pos_2) as u32], false); }
    let r_ = 'f: {
    F::Vu0Cur_RotateZXY(c, p_angles_1);
    F::Vu0Cur_Translate(c, p_pos_2);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Vu0Cur_RotateZXYTranslate", &PROBE_N, &[], true); }
    r_
}

pub fn Vu0Cur_MulVec4(c: &mut Ctx, mut p_out_1: u32, mut p_v_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vu0Cur_MulVec4", &PROBE_N, &[(p_out_1) as u32, (p_v_2) as u32], false); }
    let r_ = 'f: {
    F::vu0_a__op_lqc2(c, 4i32, p_v_2);
    F::vu0_a__op_vmulabc(c, 15i32, 16i32, 4i32, 0i32);
    F::vu0_a__op_vmaddabc(c, 15i32, 17i32, 4i32, 1i32);
    F::vu0_a__op_vmaddabc(c, 15i32, 18i32, 4i32, 2i32);
    F::vu0_a__op_vmaddbc(c, 15i32, 4i32, 19i32, 4i32, 3i32);
    F::vu0_a__op_sqc2(c, 4i32, p_out_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Vu0Cur_MulVec4", &PROBE_N, &[], true); }
    r_
}

pub fn Vu0Cur_MulVec3(c: &mut Ctx, mut p_out_1: u32, mut p_v_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vu0Cur_MulVec3", &PROBE_N, &[(p_out_1) as u32, (p_v_2) as u32], false); }
    let r_ = 'f: {
    F::vu0_a__op_lqc2(c, 4i32, p_v_2);
    F::vu0_a__op_vmulabc(c, 15i32, 16i32, 4i32, 0i32);
    F::vu0_a__op_vmaddabc(c, 15i32, 17i32, 4i32, 1i32);
    F::vu0_a__op_vmaddabc(c, 15i32, 18i32, 4i32, 2i32);
    F::vu0_a__op_vmaddbc(c, 14i32, 4i32, 19i32, 4i32, 3i32);
    F::vu0_a__op_sqc2(c, 4i32, p_out_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Vu0Cur_MulVec3", &PROBE_N, &[], true); }
    r_
}

pub fn StgVu_RotateZ(c: &mut Ctx, mut p_angle_1: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgVu_RotateZ", &PROBE_N, &[(p_angle_1).0], false); }
    let mut l_r_2: i32 = 0;
    let r_ = 'f: {
    F::Vu0_SinCos(c, p_angle_1);
    F::vu0_a__op_vsub(c, 3i32, 8i32, 0i32, 0i32);
    F::vu0_a__op_vaddbc(c, 8i32, 8i32, 0i32, 12i32, 1i32);
    F::vu0_a__op_vsubbc(c, 4i32, 8i32, 0i32, 12i32, 0i32);
    F::vu0_a__op_vsub(c, 3i32, 9i32, 0i32, 0i32);
    F::vu0_a__op_vaddbc(c, 8i32, 9i32, 0i32, 12i32, 0i32);
    F::vu0_a__op_vaddbc(c, 4i32, 9i32, 0i32, 12i32, 1i32);
    {
        l_r_2 = 16i32;
        'l1: loop {
            if !(((((l_r_2) < (20i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                F::vu0_a__op_vmulabc(c, 15i32, 8i32, l_r_2, 0i32);
                F::vu0_a__op_vmaddabc(c, 15i32, 9i32, l_r_2, 1i32);
                F::vu0_a__op_vmaddabc(c, 15i32, 1i32, l_r_2, 2i32);
                F::vu0_a__op_vmaddbc(c, 15i32, l_r_2, 0i32, l_r_2, 3i32);
            }
            { let o_ = l_r_2; let n_ = (o_).wrapping_add(1); l_r_2 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgVu_RotateZ", &PROBE_N, &[], true); }
    r_
}

pub fn StgVu_RotateX(c: &mut Ctx, mut p_angle_1: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgVu_RotateX", &PROBE_N, &[(p_angle_1).0], false); }
    let mut l_r_2: i32 = 0;
    let r_ = 'f: {
    F::Vu0_SinCos(c, p_angle_1);
    F::vu0_a__op_vsub(c, 9i32, 8i32, 0i32, 0i32);
    F::vu0_a__op_vaddbc(c, 4i32, 8i32, 0i32, 12i32, 1i32);
    F::vu0_a__op_vsubbc(c, 2i32, 8i32, 0i32, 12i32, 0i32);
    F::vu0_a__op_vsub(c, 9i32, 9i32, 0i32, 0i32);
    F::vu0_a__op_vaddbc(c, 4i32, 9i32, 0i32, 12i32, 0i32);
    F::vu0_a__op_vaddbc(c, 2i32, 9i32, 0i32, 12i32, 1i32);
    {
        l_r_2 = 16i32;
        'l1: loop {
            if !(((((l_r_2) < (20i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                F::vu0_a__op_vmulabc(c, 15i32, 3i32, l_r_2, 0i32);
                F::vu0_a__op_vmaddabc(c, 15i32, 8i32, l_r_2, 1i32);
                F::vu0_a__op_vmaddabc(c, 15i32, 9i32, l_r_2, 2i32);
                F::vu0_a__op_vmaddbc(c, 15i32, l_r_2, 0i32, l_r_2, 3i32);
            }
            { let o_ = l_r_2; let n_ = (o_).wrapping_add(1); l_r_2 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgVu_RotateX", &PROBE_N, &[], true); }
    r_
}

pub fn StgVu_RotateY(c: &mut Ctx, mut p_angle_1: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgVu_RotateY", &PROBE_N, &[(p_angle_1).0], false); }
    let mut l_r_2: i32 = 0;
    let r_ = 'f: {
    F::Vu0_SinCos(c, p_angle_1);
    F::vu0_a__op_vsub(c, 5i32, 8i32, 0i32, 0i32);
    F::vu0_a__op_vaddbc(c, 8i32, 8i32, 0i32, 12i32, 1i32);
    F::vu0_a__op_vsubbc(c, 2i32, 8i32, 0i32, 12i32, 0i32);
    F::vu0_a__op_vsub(c, 5i32, 9i32, 0i32, 0i32);
    F::vu0_a__op_vaddbc(c, 8i32, 9i32, 0i32, 12i32, 0i32);
    F::vu0_a__op_vaddbc(c, 2i32, 9i32, 0i32, 12i32, 1i32);
    {
        l_r_2 = 16i32;
        'l1: loop {
            if !(((((l_r_2) < (20i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                F::vu0_a__op_vmulabc(c, 15i32, 8i32, l_r_2, 0i32);
                F::vu0_a__op_vmaddabc(c, 15i32, 2i32, l_r_2, 1i32);
                F::vu0_a__op_vmaddabc(c, 15i32, 9i32, l_r_2, 2i32);
                F::vu0_a__op_vmaddbc(c, 15i32, l_r_2, 0i32, l_r_2, 3i32);
            }
            { let o_ = l_r_2; let n_ = (o_).wrapping_add(1); l_r_2 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgVu_RotateY", &PROBE_N, &[], true); }
    r_
}
