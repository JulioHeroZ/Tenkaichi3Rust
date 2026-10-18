#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn vu0_b__f_bits(c: &mut Ctx, p_f_1: F32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "vu0_b__f_bits", &PROBE_N, &[(p_f_1).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    c.m.w32(fp, (p_f_1).0);
    let r_ = 'f: {
        { let _ = { rt::memcpy(c, fp.wrapping_add(4), fp, (4u32) as u32); fp.wrapping_add(4) }; };
        break 'f c.m.r32(fp.wrapping_add(4));
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "vu0_b__f_bits", &PROBE_N, &[], true); }
    r_
}

pub fn vu0_b__f_from(c: &mut Ctx, p_u_1: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "vu0_b__f_from", &PROBE_N, &[(p_u_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    c.m.w32(fp, p_u_1);
    let r_ = 'f: {
        { let _ = { rt::memcpy(c, fp.wrapping_add(4), fp, (4u32) as u32); fp.wrapping_add(4) }; };
        break 'f F32(c.m.r32(fp.wrapping_add(4)));
       F32(0)
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "vu0_b__f_from", &PROBE_N, &[], true); }
    r_
}

pub fn vu0_b__in(c: &mut Ctx, mut p_a_1: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "vu0_b__in", &PROBE_N, &[(p_a_1) as u32], false); }
    let r_ = 'f: {
    break 'f (if ((((((((p_a_1).wrapping_shr((23i32) as u32)) & (((255i32) as u32)))) == (((255i32) as u32))) as i32)) != 0) { ((((p_a_1) & (2147483648u32))) | (2139095039u32)) } else { p_a_1 });
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "vu0_b__in", &PROBE_N, &[], true); }
    r_
}

pub fn vu0_b__pack(c: &mut Ctx, mut p_sign_1: u32, mut p_e_2: i32, mut p_m_3: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "vu0_b__pack", &PROBE_N, &[(p_sign_1) as u32, (p_e_2) as u32, (p_m_3) as u32], false); }
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
    if rt::probing() { crate::host::harness::probe(c, "vu0_b__pack", &PROBE_N, &[], true); }
    r_
}

pub fn vu0_b__round_pack(c: &mut Ctx, mut p_sign_1: u32, mut p_e_2: i32, mut p_m_3: u64, mut p_sticky_4: i32, mut p_nearest_5: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "vu0_b__round_pack", &PROBE_N, &[(p_sign_1) as u32, (p_e_2) as u32, (p_m_3) as u32, (p_sticky_4) as u32, (p_nearest_5) as u32], false); }
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
    break 'f F::vu0_b__pack(c, p_sign_1, p_e_2, l_hi_6);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "vu0_b__round_pack", &PROBE_N, &[], true); }
    r_
}

pub fn vu0_b__add_core(c: &mut Ctx, mut p_a_1: u32, mut p_b_2: u32, mut p_nearest_3: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "vu0_b__add_core", &PROBE_N, &[(p_a_1) as u32, (p_b_2) as u32, (p_nearest_3) as u32], false); }
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
    p_a_1 = F::vu0_b__in(c, p_a_1);
    p_b_2 = F::vu0_b__in(c, p_b_2);
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
    break 'f F::vu0_b__round_pack(c, l_sign_6, l_e_7, l_ma_4, l_sticky_9, p_nearest_3);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "vu0_b__add_core", &PROBE_N, &[], true); }
    r_
}

pub fn vu0_b__mul_core(c: &mut Ctx, mut p_a_1: u32, mut p_b_2: u32, mut p_nearest_3: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "vu0_b__mul_core", &PROBE_N, &[(p_a_1) as u32, (p_b_2) as u32, (p_nearest_3) as u32], false); }
    let mut l_sign_4: u32 = 0;
    let mut l_m_5: u64 = 0;
    let mut l_e_6: i32 = 0;
    let r_ = 'f: {
    l_sign_4 = ((((p_a_1) ^ (p_b_2))) & (2147483648u32));
    p_a_1 = F::vu0_b__in(c, p_a_1);
    p_b_2 = F::vu0_b__in(c, p_b_2);
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
    break 'f F::vu0_b__round_pack(c, l_sign_4, l_e_6, l_m_5, 0i32, p_nearest_3);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "vu0_b__mul_core", &PROBE_N, &[], true); }
    r_
}

pub fn vu0_b__mode(c: &mut Ctx, mut p_which_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "vu0_b__mode", &PROBE_N, &[(p_which_1) as u32], false); }
    let mut l_e_2: u32 = 0;
    let r_ = 'f: {
    if ((((((c.m.r32((sym::vu0_b__mode__m).wrapping_add((((p_which_1) as u32)).wrapping_mul(4))) as i32)) < (0i32)) as i32)) != 0) {
        l_e_2 = { let a0_ = c.m.r32((sym::vu0_b__mode__name).wrapping_add((((p_which_1) as u32)).wrapping_mul(4))); F::getenv(c, a0_) };
        { let v_ = (((((((l_e_2) != (((0i32) as u32))) as i32)) != 0) && ((((((((c.m.r8((l_e_2).wrapping_add(((0i32) as u32))) as i8)) as i32)) == (49i32)) as i32)) != 0)) as i32); let a_ = (sym::vu0_b__mode__m).wrapping_add((((p_which_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    }
    break 'f (c.m.r32((sym::vu0_b__mode__m).wrapping_add((((p_which_1) as u32)).wrapping_mul(4))) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "vu0_b__mode", &PROBE_N, &[], true); }
    r_
}

pub fn vu0_b__fpu_add(c: &mut Ctx, mut p_a_1: u32, mut p_b_2: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "vu0_b__fpu_add", &PROBE_N, &[(p_a_1) as u32, (p_b_2) as u32], false); }
    let mut l_d_3: i32 = 0;
    let r_ = 'f: {
    l_d_3 = ((((((p_a_1).wrapping_shr((23i32) as u32)) & (((255i32) as u32)))) as i32)).wrapping_sub((((((p_b_2).wrapping_shr((23i32) as u32)) & (((255i32) as u32)))) as i32));
    if ((F::vu0_b__mode(c, 3i32)) != 0) {
        break 'f { let a2_ = F::vu0_b__mode(c, 1i32); F::vu0_b__add_core(c, p_a_1, p_b_2, a2_) };
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
    break 'f { let a2_ = F::vu0_b__mode(c, 1i32); F::vu0_b__add_core(c, p_a_1, p_b_2, a2_) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "vu0_b__fpu_add", &PROBE_N, &[], true); }
    r_
}

pub fn vu0_b__Sf_AddBits(c: &mut Ctx, mut p_a_1: u32, mut p_b_2: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "vu0_b__Sf_AddBits", &PROBE_N, &[(p_a_1) as u32, (p_b_2) as u32], false); }
    let r_ = 'f: {
    break 'f (if ((F::vu0_b__mode(c, 2i32)) != 0) { F::vu0_b__fpu_add(c, p_a_1, p_b_2) } else { { let a2_ = F::vu0_b__mode(c, 0i32); F::vu0_b__add_core(c, p_a_1, p_b_2, a2_) } });
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "vu0_b__Sf_AddBits", &PROBE_N, &[], true); }
    r_
}

pub fn vu0_b__Sf_MulBits(c: &mut Ctx, mut p_a_1: u32, mut p_b_2: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "vu0_b__Sf_MulBits", &PROBE_N, &[(p_a_1) as u32, (p_b_2) as u32], false); }
    let r_ = 'f: {
    break 'f { let a2_ = F::vu0_b__mode(c, 0i32); F::vu0_b__mul_core(c, p_a_1, p_b_2, a2_) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "vu0_b__Sf_MulBits", &PROBE_N, &[], true); }
    r_
}

pub fn vu0_b__f_add(c: &mut Ctx, mut p_a_1: F32, mut p_b_2: F32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "vu0_b__f_add", &PROBE_N, &[(p_a_1).0, (p_b_2).0], false); }
    let r_ = 'f: {
    break 'f { let a0_ = { let a0_ = F::vu0_b__f_bits(c, p_a_1); let a1_ = F::vu0_b__f_bits(c, p_b_2); F::vu0_b__Sf_AddBits(c, a0_, a1_) }; F::vu0_b__f_from(c, a0_) };
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "vu0_b__f_add", &PROBE_N, &[], true); }
    r_
}

pub fn vu0_b__f_sub(c: &mut Ctx, mut p_a_1: F32, mut p_b_2: F32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "vu0_b__f_sub", &PROBE_N, &[(p_a_1).0, (p_b_2).0], false); }
    let r_ = 'f: {
    break 'f { let a0_ = { let a0_ = F::vu0_b__f_bits(c, p_a_1); let a1_ = ((F::vu0_b__f_bits(c, p_b_2)) ^ (2147483648u32)); F::vu0_b__Sf_AddBits(c, a0_, a1_) }; F::vu0_b__f_from(c, a0_) };
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "vu0_b__f_sub", &PROBE_N, &[], true); }
    r_
}

pub fn vu0_b__f_mul(c: &mut Ctx, mut p_a_1: F32, mut p_b_2: F32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "vu0_b__f_mul", &PROBE_N, &[(p_a_1).0, (p_b_2).0], false); }
    let r_ = 'f: {
    break 'f { let a0_ = { let a0_ = F::vu0_b__f_bits(c, p_a_1); let a1_ = F::vu0_b__f_bits(c, p_b_2); F::vu0_b__Sf_MulBits(c, a0_, a1_) }; F::vu0_b__f_from(c, a0_) };
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "vu0_b__f_mul", &PROBE_N, &[], true); }
    r_
}

pub fn vu0_b__f_div(c: &mut Ctx, mut p_a_1: F32, mut p_b_2: F32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "vu0_b__f_div", &PROBE_N, &[(p_a_1).0, (p_b_2).0], false); }
    let r_ = 'f: {
    break 'f F::RefVu0_Div(c, p_a_1, p_b_2);
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "vu0_b__f_div", &PROBE_N, &[], true); }
    r_
}

pub fn vu0_b__f_sqrt(c: &mut Ctx, mut p_a_1: F32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "vu0_b__f_sqrt", &PROBE_N, &[(p_a_1).0], false); }
    let r_ = 'f: {
    break 'f { let a0_ = { let a0_ = F::vu0_b__f_bits(c, p_a_1); F::RefVu0_SqrtBits(c, a0_) }; F::vu0_b__f_from(c, a0_) };
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "vu0_b__f_sqrt", &PROBE_N, &[], true); }
    r_
}

pub fn vu0_b__f_lt(c: &mut Ctx, mut p_a_1: F32, mut p_b_2: F32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "vu0_b__f_lt", &PROBE_N, &[(p_a_1).0, (p_b_2).0], false); }
    let r_ = 'f: {
    break 'f { let a0_ = F::vu0_b__f_bits(c, p_a_1); let a1_ = F::vu0_b__f_bits(c, p_b_2); F::RefVu0_LtBits(c, a0_, a1_) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "vu0_b__f_lt", &PROBE_N, &[], true); }
    r_
}

pub fn vu0_b__f_toi(c: &mut Ctx, mut p_f_1: F32, mut p_shift_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "vu0_b__f_toi", &PROBE_N, &[(p_f_1).0, (p_shift_2) as u32], false); }
    let mut l_d_3: f64 = 0.0f64;
    let r_ = 'f: {
    l_d_3 = ((rt::f2d(p_f_1)) * ((((1i32).wrapping_shl((p_shift_2) as u32)) as f64)));
    if (((((l_d_3) != (l_d_3)) as i32)) != 0) {
        break 'f (if ((((F::vu0_b__f_bits(c, p_f_1)) & (2147483648u32))) != 0) { ((2147483648u32) as i32) } else { 2147483647i32 });
    }
    if (((((l_d_3) >= (f64::from_bits(0x41e0000000000000))) as i32)) != 0) {
        break 'f 2147483647i32;
    }
    if (((((l_d_3) <= (f64::from_bits(0xc1e0000000000000))) as i32)) != 0) {
        break 'f ((2147483648u32) as i32);
    }
    break 'f rt::d2i(l_d_3);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "vu0_b__f_toi", &PROBE_N, &[], true); }
    r_
}

pub fn vu0_b__f_key(c: &mut Ctx, mut p_f_1: F32) -> i64 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "vu0_b__f_key", &PROBE_N, &[(p_f_1).0], false); }
    let mut l_u_2: u32 = 0;
    let r_ = 'f: {
    l_u_2 = F::vu0_b__f_bits(c, p_f_1);
    break 'f (if ((((l_u_2) & (2147483648u32))) != 0) { ((((((l_u_2) & (2147483647u32))) as i64)).wrapping_neg()).wrapping_sub(((1i32) as i64)) } else { ((l_u_2) as i64) });
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "vu0_b__f_key", &PROBE_N, &[], true); }
    r_
}

pub fn vu0_b__f_maxf(c: &mut Ctx, mut p_a_1: F32, mut p_b_2: F32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "vu0_b__f_maxf", &PROBE_N, &[(p_a_1).0, (p_b_2).0], false); }
    let r_ = 'f: {
    break 'f (if (((((F::vu0_b__f_key(c, p_a_1)) >= (F::vu0_b__f_key(c, p_b_2))) as i32)) != 0) { p_a_1 } else { p_b_2 });
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "vu0_b__f_maxf", &PROBE_N, &[], true); }
    r_
}

pub fn vu0_b__f_minf(c: &mut Ctx, mut p_a_1: F32, mut p_b_2: F32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "vu0_b__f_minf", &PROBE_N, &[(p_a_1).0, (p_b_2).0], false); }
    let r_ = 'f: {
    break 'f (if (((((F::vu0_b__f_key(c, p_a_1)) <= (F::vu0_b__f_key(c, p_b_2))) as i32)) != 0) { p_a_1 } else { p_b_2 });
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "vu0_b__f_minf", &PROBE_N, &[], true); }
    r_
}

pub fn vu0_b__mtx_apply(c: &mut Ctx, sret: u32, mut p_r_1: u32, p_v_2: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "vu0_b__mtx_apply", &PROBE_N, &[(sret) as u32, (p_r_1) as u32, (p_v_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    rt::memcpy(c, fp, p_v_2, 16);
    let r_ = 'f: {
        {
            if ((((((((((((!((F::vu0_b__mode(c, 0i32)) != 0)) as i32)) != 0) && ((((!((F::vu0_b__mode(c, 2i32)) != 0)) as i32)) != 0)) as i32)) != 0) && ((F::Port_FastMtxApply(c, p_r_1, fp, fp.wrapping_add(32))) != 0)) as i32)) != 0) {
                break 'f { let s_ = fp.wrapping_add(32); rt::memcpy(c, sret, s_, 16); sret };
            }
        }
        { let v_ = { let a0_ = F32(c.m.r32((p_r_1).wrapping_add((((0i32) as u32)).wrapping_mul(16)))); let a1_ = F32(c.m.r32(fp)); F::vu0_b__f_mul(c, a0_, a1_) }; let a_ = fp.wrapping_add(16); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32(((p_r_1).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4))); let a1_ = F32(c.m.r32(fp)); F::vu0_b__f_mul(c, a0_, a1_) }; let a_ = (fp.wrapping_add(16)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32(((p_r_1).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8))); let a1_ = F32(c.m.r32(fp)); F::vu0_b__f_mul(c, a0_, a1_) }; let a_ = (fp.wrapping_add(16)).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32(((p_r_1).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(12))); let a1_ = F32(c.m.r32(fp)); F::vu0_b__f_mul(c, a0_, a1_) }; let a_ = (fp.wrapping_add(16)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32(fp.wrapping_add(16))); let a1_ = { let a0_ = F32(c.m.r32((p_r_1).wrapping_add((((1i32) as u32)).wrapping_mul(16)))); let a1_ = F32(c.m.r32((fp).wrapping_add(4))); F::vu0_b__f_mul(c, a0_, a1_) }; F::vu0_b__f_add(c, a0_, a1_) }; let a_ = fp.wrapping_add(16); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))); let a1_ = { let a0_ = F32(c.m.r32(((p_r_1).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(4))); let a1_ = F32(c.m.r32((fp).wrapping_add(4))); F::vu0_b__f_mul(c, a0_, a1_) }; F::vu0_b__f_add(c, a0_, a1_) }; let a_ = (fp.wrapping_add(16)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8))); let a1_ = { let a0_ = F32(c.m.r32(((p_r_1).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8))); let a1_ = F32(c.m.r32((fp).wrapping_add(4))); F::vu0_b__f_mul(c, a0_, a1_) }; F::vu0_b__f_add(c, a0_, a1_) }; let a_ = (fp.wrapping_add(16)).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(12))); let a1_ = { let a0_ = F32(c.m.r32(((p_r_1).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(12))); let a1_ = F32(c.m.r32((fp).wrapping_add(4))); F::vu0_b__f_mul(c, a0_, a1_) }; F::vu0_b__f_add(c, a0_, a1_) }; let a_ = (fp.wrapping_add(16)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32(fp.wrapping_add(16))); let a1_ = { let a0_ = F32(c.m.r32((p_r_1).wrapping_add((((2i32) as u32)).wrapping_mul(16)))); let a1_ = F32(c.m.r32((fp).wrapping_add(8))); F::vu0_b__f_mul(c, a0_, a1_) }; F::vu0_b__f_add(c, a0_, a1_) }; let a_ = fp.wrapping_add(16); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))); let a1_ = { let a0_ = F32(c.m.r32(((p_r_1).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(4))); let a1_ = F32(c.m.r32((fp).wrapping_add(8))); F::vu0_b__f_mul(c, a0_, a1_) }; F::vu0_b__f_add(c, a0_, a1_) }; let a_ = (fp.wrapping_add(16)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8))); let a1_ = { let a0_ = F32(c.m.r32(((p_r_1).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8))); let a1_ = F32(c.m.r32((fp).wrapping_add(8))); F::vu0_b__f_mul(c, a0_, a1_) }; F::vu0_b__f_add(c, a0_, a1_) }; let a_ = (fp.wrapping_add(16)).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(12))); let a1_ = { let a0_ = F32(c.m.r32(((p_r_1).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(12))); let a1_ = F32(c.m.r32((fp).wrapping_add(8))); F::vu0_b__f_mul(c, a0_, a1_) }; F::vu0_b__f_add(c, a0_, a1_) }; let a_ = (fp.wrapping_add(16)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32(fp.wrapping_add(16))); let a1_ = { let a0_ = F32(c.m.r32((p_r_1).wrapping_add((((3i32) as u32)).wrapping_mul(16)))); let a1_ = F32(c.m.r32((fp).wrapping_add(12))); F::vu0_b__f_mul(c, a0_, a1_) }; F::vu0_b__f_add(c, a0_, a1_) }; let a_ = fp.wrapping_add(32); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))); let a1_ = { let a0_ = F32(c.m.r32(((p_r_1).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(4))); let a1_ = F32(c.m.r32((fp).wrapping_add(12))); F::vu0_b__f_mul(c, a0_, a1_) }; F::vu0_b__f_add(c, a0_, a1_) }; let a_ = (fp.wrapping_add(32)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8))); let a1_ = { let a0_ = F32(c.m.r32(((p_r_1).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(8))); let a1_ = F32(c.m.r32((fp).wrapping_add(12))); F::vu0_b__f_mul(c, a0_, a1_) }; F::vu0_b__f_add(c, a0_, a1_) }; let a_ = (fp.wrapping_add(32)).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(12))); let a1_ = { let a0_ = F32(c.m.r32(((p_r_1).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(12))); let a1_ = F32(c.m.r32((fp).wrapping_add(12))); F::vu0_b__f_mul(c, a0_, a1_) }; F::vu0_b__f_add(c, a0_, a1_) }; let a_ = (fp.wrapping_add(32)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        break 'f { let s_ = fp.wrapping_add(32); rt::memcpy(c, sret, s_, 16); sret };
       sret
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "vu0_b__mtx_apply", &PROBE_N, &[], true); }
    r_
}

pub fn vu0_b__mtx_load(c: &mut Ctx, mut p_r_1: u32, mut p_m_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "vu0_b__mtx_load", &PROBE_N, &[(p_r_1) as u32, (p_m_2) as u32], false); }
    let r_ = 'f: {
    { let s_ = (p_m_2).wrapping_add((((0i32) as u32)).wrapping_mul(16)); let d_ = (p_r_1).wrapping_add((((0i32) as u32)).wrapping_mul(16)); rt::memcpy(c, d_, s_, 16); };
    { let s_ = (p_m_2).wrapping_add((((1i32) as u32)).wrapping_mul(16)); let d_ = (p_r_1).wrapping_add((((1i32) as u32)).wrapping_mul(16)); rt::memcpy(c, d_, s_, 16); };
    { let s_ = (p_m_2).wrapping_add((((2i32) as u32)).wrapping_mul(16)); let d_ = (p_r_1).wrapping_add((((2i32) as u32)).wrapping_mul(16)); rt::memcpy(c, d_, s_, 16); };
    { let s_ = (p_m_2).wrapping_add((((3i32) as u32)).wrapping_mul(16)); let d_ = (p_r_1).wrapping_add((((3i32) as u32)).wrapping_mul(16)); rt::memcpy(c, d_, s_, 16); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "vu0_b__mtx_load", &PROBE_N, &[], true); }
    r_
}

pub fn vu0_b__dot3(c: &mut Ctx, p_a_1: u32, p_b_2: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "vu0_b__dot3", &PROBE_N, &[(p_a_1) as u32, (p_b_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_tx_3: F32 = F32(0);
    let mut l_ty_4: F32 = F32(0);
    let mut l_tz_5: F32 = F32(0);
    let mut l_acc_6: F32 = F32(0);
    rt::memcpy(c, fp, p_a_1, 16);
    rt::memcpy(c, fp.wrapping_add(16), p_b_2, 16);
    let r_ = 'f: {
        l_tx_3 = { let a0_ = F32(c.m.r32(fp)); let a1_ = F32(c.m.r32(fp.wrapping_add(16))); F::vu0_b__f_mul(c, a0_, a1_) };
        l_ty_4 = { let a0_ = F32(c.m.r32((fp).wrapping_add(4))); let a1_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))); F::vu0_b__f_mul(c, a0_, a1_) };
        l_tz_5 = { let a0_ = F32(c.m.r32((fp).wrapping_add(8))); let a1_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8))); F::vu0_b__f_mul(c, a0_, a1_) };
        l_acc_6 = F::vu0_b__f_add(c, l_tx_3, l_ty_4);
        break 'f { let a1_ = { let a0_ = F32(c.m.r32((sym::gRefVu0).wrapping_add((((3i32) as u32)).wrapping_mul(16)))); F::vu0_b__f_mul(c, a0_, l_tz_5) }; F::vu0_b__f_add(c, l_acc_6, a1_) };
       F32(0)
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "vu0_b__dot3", &PROBE_N, &[], true); }
    r_
}

pub fn vu0_b__persp(c: &mut Ctx, mut p_p_1: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "vu0_b__persp", &PROBE_N, &[(p_p_1) as u32], false); }
    let mut l_q_2: F32 = F32(0);
    let r_ = 'f: {
    l_q_2 = { let a0_ = F32(c.m.r32(((sym::gRefVu0).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(12))); let a1_ = F32(c.m.r32((p_p_1).wrapping_add(12))); F::vu0_b__f_div(c, a0_, a1_) };
    { let v_ = { let a0_ = F32(c.m.r32(p_p_1)); F::vu0_b__f_mul(c, a0_, l_q_2) }; let a_ = p_p_1; c.m.w32(a_, (v_).0); };
    { let v_ = { let a0_ = F32(c.m.r32((p_p_1).wrapping_add(4))); F::vu0_b__f_mul(c, a0_, l_q_2) }; let a_ = (p_p_1).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = { let a0_ = F32(c.m.r32((p_p_1).wrapping_add(8))); F::vu0_b__f_mul(c, a0_, l_q_2) }; let a_ = (p_p_1).wrapping_add(8); c.m.w32(a_, (v_).0); };
    break 'f l_q_2;
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "vu0_b__persp", &PROBE_N, &[], true); }
    r_
}

pub fn vu0_b__on_screen(c: &mut Ctx, p_p_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "vu0_b__on_screen", &PROBE_N, &[(p_p_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_d_2: F32 = F32(0);
    let mut l_bad_3: i32 = 0;
    rt::memcpy(c, fp, p_p_1, 16);
    let r_ = 'f: {
        l_bad_3 = 0i32;
        l_d_2 = { let a0_ = F32(c.m.r32(fp)); F::vu0_b__f_sub(c, a0_, F32(0x00000000)) };
        { let r_ = (((((((F::vu0_b__f_bits(c, l_d_2)) & (2147483647u32))) == (((0i32) as u32))) as i32)) | ((((F::vu0_b__f_bits(c, l_d_2)).wrapping_shr((31i32) as u32)) as i32))); let n_ = ((l_bad_3) | (r_)); l_bad_3 = n_; };
        l_d_2 = { let a0_ = F32(c.m.r32((fp).wrapping_add(4))); F::vu0_b__f_sub(c, a0_, F32(0x00000000)) };
        { let r_ = (((((((F::vu0_b__f_bits(c, l_d_2)) & (2147483647u32))) == (((0i32) as u32))) as i32)) | ((((F::vu0_b__f_bits(c, l_d_2)).wrapping_shr((31i32) as u32)) as i32))); let n_ = ((l_bad_3) | (r_)); l_bad_3 = n_; };
        l_d_2 = { let a0_ = F32(c.m.r32((fp).wrapping_add(12))); F::vu0_b__f_sub(c, a0_, F32(0x00000000)) };
        { let r_ = (((((((F::vu0_b__f_bits(c, l_d_2)) & (2147483647u32))) == (((0i32) as u32))) as i32)) | ((((F::vu0_b__f_bits(c, l_d_2)).wrapping_shr((31i32) as u32)) as i32))); let n_ = ((l_bad_3) | (r_)); l_bad_3 = n_; };
        l_d_2 = { let a1_ = F32(c.m.r32(fp)); F::vu0_b__f_sub(c, F32(0x45800000), a1_) };
        { let r_ = (((((((F::vu0_b__f_bits(c, l_d_2)) & (2147483647u32))) == (((0i32) as u32))) as i32)) | ((((F::vu0_b__f_bits(c, l_d_2)).wrapping_shr((31i32) as u32)) as i32))); let n_ = ((l_bad_3) | (r_)); l_bad_3 = n_; };
        l_d_2 = { let a1_ = F32(c.m.r32((fp).wrapping_add(4))); F::vu0_b__f_sub(c, F32(0x45800000), a1_) };
        { let r_ = (((((((F::vu0_b__f_bits(c, l_d_2)) & (2147483647u32))) == (((0i32) as u32))) as i32)) | ((((F::vu0_b__f_bits(c, l_d_2)).wrapping_shr((31i32) as u32)) as i32))); let n_ = ((l_bad_3) | (r_)); l_bad_3 = n_; };
        break 'f (((l_bad_3) == (0i32)) as i32);
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "vu0_b__on_screen", &PROBE_N, &[], true); }
    r_
}

pub fn vu0_b__to_gs(c: &mut Ctx, sret: u32, p_p_1: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "vu0_b__to_gs", &PROBE_N, &[(sret) as u32, (p_p_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    rt::memcpy(c, fp, p_p_1, 16);
    let r_ = 'f: {
        { let v_ = { let a0_ = F32(c.m.r32(fp)); F::vu0_b__f_toi(c, a0_, 4i32) }; let a_ = fp.wrapping_add(16); c.m.w32(a_, (v_) as u32); };
        { let v_ = { let a0_ = F32(c.m.r32((fp).wrapping_add(4))); F::vu0_b__f_toi(c, a0_, 4i32) }; let a_ = (fp.wrapping_add(16)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = { let a0_ = F32(c.m.r32((fp).wrapping_add(8))); F::vu0_b__f_toi(c, a0_, 0i32) }; let a_ = (fp.wrapping_add(16)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = { let a0_ = F32(c.m.r32((fp).wrapping_add(12))); F::vu0_b__f_toi(c, a0_, 0i32) }; let a_ = (fp.wrapping_add(16)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        break 'f { let s_ = fp.wrapping_add(16); rt::memcpy(c, sret, s_, 16); sret };
       sret
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "vu0_b__to_gs", &PROBE_N, &[], true); }
    r_
}

pub fn vu0_b__stq_xyz(c: &mut Ctx, mut p_o_1: u32, p_uv_2: u32, mut p_q_3: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "vu0_b__stq_xyz", &PROBE_N, &[(p_o_1) as u32, (p_uv_2) as u32, (p_q_3).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    rt::memcpy(c, fp, p_uv_2, 16);
    let r_ = 'f: {
        { let v_ = F32(c.m.r32(((sym::gRefVu0).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8))); let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32(fp)); F::vu0_b__f_mul(c, a0_, p_q_3) }; let a_ = p_o_1; c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32((fp).wrapping_add(4))); F::vu0_b__f_mul(c, a0_, p_q_3) }; let a_ = (p_o_1).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32((fp).wrapping_add(8))); F::vu0_b__f_mul(c, a0_, p_q_3) }; let a_ = (p_o_1).wrapping_add(8); c.m.w32(a_, (v_).0); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "vu0_b__stq_xyz", &PROBE_N, &[], true); }
    r_
}

pub fn vu0_b__a_store_identity(c: &mut Ctx, mut p_m_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "vu0_b__a_store_identity", &PROBE_N, &[(p_m_1) as u32], false); }
    let r_ = 'f: {
    { let s_ = (sym::gRefVu0).wrapping_add((((3i32) as u32)).wrapping_mul(16)); let d_ = (p_m_1).wrapping_add((((0i32) as u32)).wrapping_mul(16)); rt::memcpy(c, d_, s_, 16); };
    { let s_ = (sym::gRefVu0).wrapping_add((((2i32) as u32)).wrapping_mul(16)); let d_ = (p_m_1).wrapping_add((((1i32) as u32)).wrapping_mul(16)); rt::memcpy(c, d_, s_, 16); };
    { let s_ = (sym::gRefVu0).wrapping_add((((1i32) as u32)).wrapping_mul(16)); let d_ = (p_m_1).wrapping_add((((2i32) as u32)).wrapping_mul(16)); rt::memcpy(c, d_, s_, 16); };
    { let s_ = (sym::gRefVu0).wrapping_add((((0i32) as u32)).wrapping_mul(16)); let d_ = (p_m_1).wrapping_add((((3i32) as u32)).wrapping_mul(16)); rt::memcpy(c, d_, s_, 16); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "vu0_b__a_store_identity", &PROBE_N, &[], true); }
    r_
}

pub fn vu0_b__a_cur_identity(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "vu0_b__a_cur_identity", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let s_ = (sym::gRefVu0).wrapping_add((((3i32) as u32)).wrapping_mul(16)); let d_ = (sym::gRefVu0).wrapping_add((((16i32) as u32)).wrapping_mul(16)); rt::memcpy(c, d_, s_, 16); };
    { let s_ = (sym::gRefVu0).wrapping_add((((2i32) as u32)).wrapping_mul(16)); let d_ = (sym::gRefVu0).wrapping_add((((17i32) as u32)).wrapping_mul(16)); rt::memcpy(c, d_, s_, 16); };
    { let s_ = (sym::gRefVu0).wrapping_add((((1i32) as u32)).wrapping_mul(16)); let d_ = (sym::gRefVu0).wrapping_add((((18i32) as u32)).wrapping_mul(16)); rt::memcpy(c, d_, s_, 16); };
    { let s_ = (sym::gRefVu0).wrapping_add((((0i32) as u32)).wrapping_mul(16)); let d_ = (sym::gRefVu0).wrapping_add((((19i32) as u32)).wrapping_mul(16)); rt::memcpy(c, d_, s_, 16); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "vu0_b__a_cur_identity", &PROBE_N, &[], true); }
    r_
}

pub fn vu0_b__a_cur_push(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "vu0_b__a_cur_push", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let s_ = (sym::gRefVu0).wrapping_add((((16i32) as u32)).wrapping_mul(16)); let d_ = ((sym::gRefVu0).wrapping_add(568)).wrapping_add(((((((({ let a_ = ((sym::gRefVu0).wrapping_add(532)).wrapping_add((((15i32) as u32)).wrapping_mul(2)); let o_ = c.m.r16(a_); let n_ = (o_).wrapping_add(1); c.m.w16(a_, n_); o_ }) as i32)) & (255i32))) as u32)).wrapping_mul(16)); rt::memcpy(c, d_, s_, 16); };
    { let s_ = (sym::gRefVu0).wrapping_add((((17i32) as u32)).wrapping_mul(16)); let d_ = ((sym::gRefVu0).wrapping_add(568)).wrapping_add(((((((({ let a_ = ((sym::gRefVu0).wrapping_add(532)).wrapping_add((((15i32) as u32)).wrapping_mul(2)); let o_ = c.m.r16(a_); let n_ = (o_).wrapping_add(1); c.m.w16(a_, n_); o_ }) as i32)) & (255i32))) as u32)).wrapping_mul(16)); rt::memcpy(c, d_, s_, 16); };
    { let s_ = (sym::gRefVu0).wrapping_add((((18i32) as u32)).wrapping_mul(16)); let d_ = ((sym::gRefVu0).wrapping_add(568)).wrapping_add(((((((({ let a_ = ((sym::gRefVu0).wrapping_add(532)).wrapping_add((((15i32) as u32)).wrapping_mul(2)); let o_ = c.m.r16(a_); let n_ = (o_).wrapping_add(1); c.m.w16(a_, n_); o_ }) as i32)) & (255i32))) as u32)).wrapping_mul(16)); rt::memcpy(c, d_, s_, 16); };
    { let s_ = (sym::gRefVu0).wrapping_add((((19i32) as u32)).wrapping_mul(16)); let d_ = ((sym::gRefVu0).wrapping_add(568)).wrapping_add(((((((({ let a_ = ((sym::gRefVu0).wrapping_add(532)).wrapping_add((((15i32) as u32)).wrapping_mul(2)); let o_ = c.m.r16(a_); let n_ = (o_).wrapping_add(1); c.m.w16(a_, n_); o_ }) as i32)) & (255i32))) as u32)).wrapping_mul(16)); rt::memcpy(c, d_, s_, 16); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "vu0_b__a_cur_push", &PROBE_N, &[], true); }
    r_
}

pub fn vu0_b__a_cur_pop(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "vu0_b__a_cur_pop", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let s_ = ((sym::gRefVu0).wrapping_add(568)).wrapping_add(((((((({ let a_ = ((sym::gRefVu0).wrapping_add(532)).wrapping_add((((15i32) as u32)).wrapping_mul(2)); let o_ = c.m.r16(a_); let n_ = (o_).wrapping_sub(1); c.m.w16(a_, n_); n_ }) as i32)) & (255i32))) as u32)).wrapping_mul(16)); let d_ = (sym::gRefVu0).wrapping_add((((19i32) as u32)).wrapping_mul(16)); rt::memcpy(c, d_, s_, 16); };
    { let s_ = ((sym::gRefVu0).wrapping_add(568)).wrapping_add(((((((({ let a_ = ((sym::gRefVu0).wrapping_add(532)).wrapping_add((((15i32) as u32)).wrapping_mul(2)); let o_ = c.m.r16(a_); let n_ = (o_).wrapping_sub(1); c.m.w16(a_, n_); n_ }) as i32)) & (255i32))) as u32)).wrapping_mul(16)); let d_ = (sym::gRefVu0).wrapping_add((((18i32) as u32)).wrapping_mul(16)); rt::memcpy(c, d_, s_, 16); };
    { let s_ = ((sym::gRefVu0).wrapping_add(568)).wrapping_add(((((((({ let a_ = ((sym::gRefVu0).wrapping_add(532)).wrapping_add((((15i32) as u32)).wrapping_mul(2)); let o_ = c.m.r16(a_); let n_ = (o_).wrapping_sub(1); c.m.w16(a_, n_); n_ }) as i32)) & (255i32))) as u32)).wrapping_mul(16)); let d_ = (sym::gRefVu0).wrapping_add((((17i32) as u32)).wrapping_mul(16)); rt::memcpy(c, d_, s_, 16); };
    { let s_ = ((sym::gRefVu0).wrapping_add(568)).wrapping_add(((((((({ let a_ = ((sym::gRefVu0).wrapping_add(532)).wrapping_add((((15i32) as u32)).wrapping_mul(2)); let o_ = c.m.r16(a_); let n_ = (o_).wrapping_sub(1); c.m.w16(a_, n_); n_ }) as i32)) & (255i32))) as u32)).wrapping_mul(16)); let d_ = (sym::gRefVu0).wrapping_add((((16i32) as u32)).wrapping_mul(16)); rt::memcpy(c, d_, s_, 16); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "vu0_b__a_cur_pop", &PROBE_N, &[], true); }
    r_
}

pub fn vu0_b__a_cur_load(c: &mut Ctx, mut p_m_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "vu0_b__a_cur_load", &PROBE_N, &[(p_m_1) as u32], false); }
    let r_ = 'f: {
    F::vu0_b__mtx_load(c, (sym::gRefVu0).wrapping_add((((16i32) as u32)).wrapping_mul(16)), p_m_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "vu0_b__a_cur_load", &PROBE_N, &[], true); }
    r_
}

pub fn vu0_b__a_cur_mul_vec4(c: &mut Ctx, mut p_out_1: u32, mut p_v_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "vu0_b__a_cur_mul_vec4", &PROBE_N, &[(p_out_1) as u32, (p_v_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        { let s_ = F::vu0_b__mtx_apply(c, fp, (sym::gRefVu0).wrapping_add((((16i32) as u32)).wrapping_mul(16)), p_v_2); let d_ = p_out_1; rt::memcpy(c, d_, s_, 16); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "vu0_b__a_cur_mul_vec4", &PROBE_N, &[], true); }
    r_
}

pub fn vu0_b__a_cur_mul_vec3(c: &mut Ctx, mut p_out_1: u32, mut p_v_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "vu0_b__a_cur_mul_vec3", &PROBE_N, &[(p_out_1) as u32, (p_v_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let r_ = 'f: {
        { let s_ = p_v_2; rt::memcpy(c, fp, s_, 16); }
        { let s_ = F::vu0_b__mtx_apply(c, fp.wrapping_add(32), (sym::gRefVu0).wrapping_add((((16i32) as u32)).wrapping_mul(16)), fp); rt::memcpy(c, fp.wrapping_add(16), s_, 16); }
        { let v_ = F32(c.m.r32((fp).wrapping_add(12))); let a_ = (fp.wrapping_add(16)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let s_ = fp.wrapping_add(16); let d_ = p_out_1; rt::memcpy(c, d_, s_, 16); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "vu0_b__a_cur_mul_vec3", &PROBE_N, &[], true); }
    r_
}

pub fn Vu0Cur_RotEulerMulVec4(c: &mut Ctx, mut p_out_1: u32, mut p_angles_2: u32, mut p_v_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vu0Cur_RotEulerMulVec4", &PROBE_N, &[(p_out_1) as u32, (p_angles_2) as u32, (p_v_3) as u32], false); }
    let r_ = 'f: {
    F::vu0_b__a_cur_identity(c);
    F::Vu0Cur_RotateZXY(c, p_angles_2);
    F::vu0_b__a_cur_mul_vec4(c, p_out_1, p_v_3);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Vu0Cur_RotEulerMulVec4", &PROBE_N, &[], true); }
    r_
}

pub fn Vu0Cur_RotEulerMulVec3(c: &mut Ctx, mut p_out_1: u32, mut p_angles_2: u32, mut p_v_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vu0Cur_RotEulerMulVec3", &PROBE_N, &[(p_out_1) as u32, (p_angles_2) as u32, (p_v_3) as u32], false); }
    let r_ = 'f: {
    F::vu0_b__a_cur_identity(c);
    F::Vu0Cur_RotateZXY(c, p_angles_2);
    F::vu0_b__a_cur_mul_vec3(c, p_out_1, p_v_3);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Vu0Cur_RotEulerMulVec3", &PROBE_N, &[], true); }
    r_
}

pub fn Vu0Cur_ProjectInt(c: &mut Ctx, mut p_out_1: u32, mut p_v_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vu0Cur_ProjectInt", &PROBE_N, &[(p_out_1) as u32, (p_v_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let r_ = 'f: {
        { let s_ = F::vu0_b__mtx_apply(c, fp.wrapping_add(16), (sym::gRefVu0).wrapping_add((((16i32) as u32)).wrapping_mul(16)), p_v_2); rt::memcpy(c, fp, s_, 16); }
        { let _ = F::vu0_b__persp(c, fp); };
        { let v_ = { let a0_ = F32(c.m.r32(fp)); F::vu0_b__f_toi(c, a0_, 0i32) }; let a_ = p_out_1; c.m.w32(a_, (v_) as u32); };
        { let v_ = { let a0_ = F32(c.m.r32((fp).wrapping_add(4))); F::vu0_b__f_toi(c, a0_, 0i32) }; let a_ = (p_out_1).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = { let a0_ = F32(c.m.r32((fp).wrapping_add(8))); F::vu0_b__f_toi(c, a0_, 0i32) }; let a_ = (p_out_1).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = { let a0_ = F32(c.m.r32((fp).wrapping_add(12))); F::vu0_b__f_toi(c, a0_, 0i32) }; let a_ = (p_out_1).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Vu0Cur_ProjectInt", &PROBE_N, &[], true); }
    r_
}

pub fn Vu0Cur_ProjectPoint(c: &mut Ctx, mut p_out_1: u32, mut p_v_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vu0Cur_ProjectPoint", &PROBE_N, &[(p_out_1) as u32, (p_v_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let r_ = 'f: {
        { let s_ = F::vu0_b__mtx_apply(c, fp.wrapping_add(16), (sym::gRefVu0).wrapping_add((((16i32) as u32)).wrapping_mul(16)), p_v_2); rt::memcpy(c, fp, s_, 16); }
        { let _ = F::vu0_b__persp(c, fp); };
        { let s_ = F::vu0_b__to_gs(c, fp.wrapping_add(32), fp); let d_ = p_out_1; rt::memcpy(c, d_, s_, 16); };
        break 'f F::vu0_b__on_screen(c, fp);
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Vu0Cur_ProjectPoint", &PROBE_N, &[], true); }
    r_
}

pub fn Vu0Cur_ProjectPoints(c: &mut Ctx, mut p_out_1: u32, mut p_v_2: u32, mut p_n_3: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vu0Cur_ProjectPoints", &PROBE_N, &[(p_out_1) as u32, (p_v_2) as u32, (p_n_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(64) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 64);
    let r_ = 'f: {
        { let s_ = p_v_2; rt::memcpy(c, fp, s_, 16); }
        {
            'l1: loop {
                'c2: {
                    { let s_ = F::vu0_b__mtx_apply(c, fp.wrapping_add(32), (sym::gRefVu0).wrapping_add((((16i32) as u32)).wrapping_mul(16)), fp); let d_ = fp.wrapping_add(16); rt::memcpy(c, d_, s_, 16); };
                    { let _ = F::vu0_b__persp(c, fp.wrapping_add(16)); };
                    { let o_ = p_v_2; let n_ = (o_).wrapping_add(16); p_v_2 = n_; };
                    if (((((p_n_3) != (1i32)) as i32)) != 0) {
                        { let s_ = p_v_2; let d_ = fp; rt::memcpy(c, d_, s_, 16); };
                    }
                    { let s_ = F::vu0_b__to_gs(c, fp.wrapping_add(48), fp.wrapping_add(16)); let d_ = p_out_1; rt::memcpy(c, d_, s_, 16); };
                    if ((((!((F::vu0_b__on_screen(c, fp.wrapping_add(16))) != 0)) as i32)) != 0) {
                        break 'f 0i32;
                    }
                    if ((((({ let o_ = p_n_3; let n_ = (o_).wrapping_sub(1); p_n_3 = n_; n_ }) == (0i32)) as i32)) != 0) {
                        break 'f 1i32;
                    }
                    { let o_ = p_out_1; let n_ = (o_).wrapping_add(16); p_out_1 = n_; };
                }
            }
        }
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Vu0Cur_ProjectPoints", &PROBE_N, &[], true); }
    r_
}

pub fn Vu0Cur_ProjectPointStq(c: &mut Ctx, mut p_out_1: u32, mut p_stq_2: u32, mut p_v_3: u32, mut p_uv_4: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vu0Cur_ProjectPointStq", &PROBE_N, &[(p_out_1) as u32, (p_stq_2) as u32, (p_v_3) as u32, (p_uv_4) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(64) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 64);
    let mut l_q_5: F32 = F32(0);
    let r_ = 'f: {
        { let s_ = p_uv_4; rt::memcpy(c, fp, s_, 16); }
        { let s_ = F::vu0_b__mtx_apply(c, fp.wrapping_add(32), (sym::gRefVu0).wrapping_add((((16i32) as u32)).wrapping_mul(16)), p_v_3); rt::memcpy(c, fp.wrapping_add(16), s_, 16); }
        l_q_5 = F::vu0_b__persp(c, fp.wrapping_add(16));
        F::vu0_b__stq_xyz(c, fp, fp, l_q_5);
        { let s_ = fp; let d_ = p_stq_2; rt::memcpy(c, d_, s_, 16); };
        { let s_ = F::vu0_b__to_gs(c, fp.wrapping_add(48), fp.wrapping_add(16)); let d_ = p_out_1; rt::memcpy(c, d_, s_, 16); };
        break 'f F::vu0_b__on_screen(c, fp.wrapping_add(16));
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Vu0Cur_ProjectPointStq", &PROBE_N, &[], true); }
    r_
}

pub fn Vu0Cur_ProjectPointsStq(c: &mut Ctx, mut p_out_1: u32, mut p_stq_2: u32, mut p_v_3: u32, mut p_uv_4: u32, mut p_n_5: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vu0Cur_ProjectPointsStq", &PROBE_N, &[(p_out_1) as u32, (p_stq_2) as u32, (p_v_3) as u32, (p_uv_4) as u32, (p_n_5) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(80) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 80);
    let mut l_q_6: F32 = F32(0);
    let r_ = 'f: {
        { let s_ = p_v_3; rt::memcpy(c, fp, s_, 16); }
        { let s_ = p_uv_4; rt::memcpy(c, fp.wrapping_add(16), s_, 16); }
        {
            'l1: loop {
                'c2: {
                    { let s_ = F::vu0_b__mtx_apply(c, fp.wrapping_add(48), (sym::gRefVu0).wrapping_add((((16i32) as u32)).wrapping_mul(16)), fp); let d_ = fp.wrapping_add(32); rt::memcpy(c, d_, s_, 16); };
                    l_q_6 = F::vu0_b__persp(c, fp.wrapping_add(32));
                    F::vu0_b__stq_xyz(c, (sym::gRefVu0).wrapping_add((((7i32) as u32)).wrapping_mul(16)), fp.wrapping_add(16), l_q_6);
                    { let o_ = p_v_3; let n_ = (o_).wrapping_add(16); p_v_3 = n_; };
                    { let o_ = p_uv_4; let n_ = (o_).wrapping_add(16); p_uv_4 = n_; };
                    if (((((p_n_5) != (1i32)) as i32)) != 0) {
                        { let s_ = p_v_3; let d_ = fp; rt::memcpy(c, d_, s_, 16); };
                        { let s_ = p_uv_4; let d_ = fp.wrapping_add(16); rt::memcpy(c, d_, s_, 16); };
                    }
                    { let s_ = (sym::gRefVu0).wrapping_add((((7i32) as u32)).wrapping_mul(16)); let d_ = p_stq_2; rt::memcpy(c, d_, s_, 16); };
                    { let s_ = F::vu0_b__to_gs(c, fp.wrapping_add(64), fp.wrapping_add(32)); let d_ = p_out_1; rt::memcpy(c, d_, s_, 16); };
                    if ((((!((F::vu0_b__on_screen(c, fp.wrapping_add(32))) != 0)) as i32)) != 0) {
                        break 'f 0i32;
                    }
                    if ((((({ let o_ = p_n_5; let n_ = (o_).wrapping_sub(1); p_n_5 = n_; n_ }) == (0i32)) as i32)) != 0) {
                        break 'f 1i32;
                    }
                    { let o_ = p_out_1; let n_ = (o_).wrapping_add(16); p_out_1 = n_; };
                    { let o_ = p_stq_2; let n_ = (o_).wrapping_add(16); p_stq_2 = n_; };
                }
            }
        }
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Vu0Cur_ProjectPointsStq", &PROBE_N, &[], true); }
    r_
}

pub fn vu0_b__reg_set_mul(c: &mut Ctx, mut p_reg_1: u32, mut p_a_2: u32, mut p_b_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "vu0_b__reg_set_mul", &PROBE_N, &[(p_reg_1) as u32, (p_a_2) as u32, (p_b_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(192) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 192);
    let r_ = 'f: {
        F::vu0_b__mtx_load(c, fp, p_a_2);
        F::vu0_b__mtx_load(c, fp.wrapping_add(64), p_b_3);
        { let s_ = F::vu0_b__mtx_apply(c, fp.wrapping_add(128), fp, (fp.wrapping_add(64)).wrapping_add((((0i32) as u32)).wrapping_mul(16))); let d_ = (p_reg_1).wrapping_add((((0i32) as u32)).wrapping_mul(16)); rt::memcpy(c, d_, s_, 16); };
        { let s_ = F::vu0_b__mtx_apply(c, fp.wrapping_add(144), fp, (fp.wrapping_add(64)).wrapping_add((((1i32) as u32)).wrapping_mul(16))); let d_ = (p_reg_1).wrapping_add((((1i32) as u32)).wrapping_mul(16)); rt::memcpy(c, d_, s_, 16); };
        { let s_ = F::vu0_b__mtx_apply(c, fp.wrapping_add(160), fp, (fp.wrapping_add(64)).wrapping_add((((2i32) as u32)).wrapping_mul(16))); let d_ = (p_reg_1).wrapping_add((((2i32) as u32)).wrapping_mul(16)); rt::memcpy(c, d_, s_, 16); };
        { let s_ = F::vu0_b__mtx_apply(c, fp.wrapping_add(176), fp, (fp.wrapping_add(64)).wrapping_add((((3i32) as u32)).wrapping_mul(16))); let d_ = (p_reg_1).wrapping_add((((3i32) as u32)).wrapping_mul(16)); rt::memcpy(c, d_, s_, 16); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "vu0_b__reg_set_mul", &PROBE_N, &[], true); }
    r_
}

pub fn Vu0Clip_LoadMtx(c: &mut Ctx, mut p_m_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vu0Clip_LoadMtx", &PROBE_N, &[(p_m_1) as u32], false); }
    let r_ = 'f: {
    F::vu0_b__mtx_load(c, (sym::gRefVu0).wrapping_add((((20i32) as u32)).wrapping_mul(16)), p_m_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Vu0Clip_LoadMtx", &PROBE_N, &[], true); }
    r_
}

pub fn Vu0Clip_StoreMtx(c: &mut Ctx, mut p_m_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vu0Clip_StoreMtx", &PROBE_N, &[(p_m_1) as u32], false); }
    let r_ = 'f: {
    F::vu0_b__mtx_load(c, p_m_1, (sym::gRefVu0).wrapping_add((((20i32) as u32)).wrapping_mul(16)));
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Vu0Clip_StoreMtx", &PROBE_N, &[], true); }
    r_
}

pub fn Vu0Clip_SetMulMtx(c: &mut Ctx, mut p_a_1: u32, mut p_b_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vu0Clip_SetMulMtx", &PROBE_N, &[(p_a_1) as u32, (p_b_2) as u32], false); }
    let r_ = 'f: {
    F::vu0_b__reg_set_mul(c, (sym::gRefVu0).wrapping_add((((20i32) as u32)).wrapping_mul(16)), p_a_1, p_b_2);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Vu0Clip_SetMulMtx", &PROBE_N, &[], true); }
    r_
}

pub fn Vu0Screen_LoadMtx(c: &mut Ctx, mut p_m_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vu0Screen_LoadMtx", &PROBE_N, &[(p_m_1) as u32], false); }
    let r_ = 'f: {
    F::vu0_b__mtx_load(c, (sym::gRefVu0).wrapping_add((((24i32) as u32)).wrapping_mul(16)), p_m_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Vu0Screen_LoadMtx", &PROBE_N, &[], true); }
    r_
}

pub fn Vu0Screen_StoreMtx(c: &mut Ctx, mut p_m_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vu0Screen_StoreMtx", &PROBE_N, &[(p_m_1) as u32], false); }
    let r_ = 'f: {
    F::vu0_b__mtx_load(c, p_m_1, (sym::gRefVu0).wrapping_add((((24i32) as u32)).wrapping_mul(16)));
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Vu0Screen_StoreMtx", &PROBE_N, &[], true); }
    r_
}

pub fn Vu0Screen_SetMulMtx(c: &mut Ctx, mut p_a_1: u32, mut p_b_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vu0Screen_SetMulMtx", &PROBE_N, &[(p_a_1) as u32, (p_b_2) as u32], false); }
    let r_ = 'f: {
    F::vu0_b__reg_set_mul(c, (sym::gRefVu0).wrapping_add((((24i32) as u32)).wrapping_mul(16)), p_a_1, p_b_2);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Vu0Screen_SetMulMtx", &PROBE_N, &[], true); }
    r_
}

pub fn Vu0View_LoadMtx(c: &mut Ctx, mut p_m_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vu0View_LoadMtx", &PROBE_N, &[(p_m_1) as u32], false); }
    let r_ = 'f: {
    F::vu0_b__mtx_load(c, (sym::gRefVu0).wrapping_add((((28i32) as u32)).wrapping_mul(16)), p_m_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Vu0View_LoadMtx", &PROBE_N, &[], true); }
    r_
}

pub fn Vu0View_StoreMtx(c: &mut Ctx, mut p_m_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vu0View_StoreMtx", &PROBE_N, &[(p_m_1) as u32], false); }
    let r_ = 'f: {
    F::vu0_b__mtx_load(c, p_m_1, (sym::gRefVu0).wrapping_add((((28i32) as u32)).wrapping_mul(16)));
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Vu0View_StoreMtx", &PROBE_N, &[], true); }
    r_
}

pub fn Vu0View_SetMulMtx(c: &mut Ctx, mut p_a_1: u32, mut p_b_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vu0View_SetMulMtx", &PROBE_N, &[(p_a_1) as u32, (p_b_2) as u32], false); }
    let r_ = 'f: {
    F::vu0_b__reg_set_mul(c, (sym::gRefVu0).wrapping_add((((28i32) as u32)).wrapping_mul(16)), p_a_1, p_b_2);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Vu0View_SetMulMtx", &PROBE_N, &[], true); }
    r_
}

pub fn Vu0View_SetRotTrans(c: &mut Ctx, mut p_rot_1: u32, mut p_trans_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vu0View_SetRotTrans", &PROBE_N, &[(p_rot_1) as u32, (p_trans_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(128) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 128);
    let mut l_s_3: F32 = F32(0);
    let mut l_c_4: F32 = F32(0);
    let r_ = 'f: {
        F::vu0_b__a_cur_push(c);
        F::Vu0View_StoreMtx(c, fp);
        F::vu0_b__a_store_identity(c, fp);
        F::vu0_b__a_store_identity(c, fp.wrapping_add(64));
        l_s_3 = { let a0_ = F32(c.m.r32(p_rot_1)); F::Port_sinf(c, a0_) };
        l_c_4 = { let a0_ = F32(c.m.r32(p_rot_1)); F::Port_cosf(c, a0_) };
        { let v_ = l_c_4; let a_ = ((fp.wrapping_add(64)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fneg(l_s_3); let a_ = ((fp.wrapping_add(64)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = l_s_3; let a_ = ((fp.wrapping_add(64)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = l_c_4; let a_ = ((fp.wrapping_add(64)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_).0); };
        F::vu0_b__a_cur_load(c, fp);
        F::vu0_b__a_cur_mul_vec4(c, (fp).wrapping_add((((1i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(64)).wrapping_add((((1i32) as u32)).wrapping_mul(16)));
        F::vu0_b__a_cur_mul_vec4(c, (fp).wrapping_add((((2i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(64)).wrapping_add((((2i32) as u32)).wrapping_mul(16)));
        F::vu0_b__a_store_identity(c, fp.wrapping_add(64));
        l_s_3 = { let a0_ = F32(c.m.r32((p_rot_1).wrapping_add(8))); F::Port_sinf(c, a0_) };
        l_c_4 = { let a0_ = F32(c.m.r32((p_rot_1).wrapping_add(8))); F::Port_cosf(c, a0_) };
        { let v_ = l_c_4; let a_ = (fp.wrapping_add(64)).wrapping_add((((0i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fneg(l_s_3); let a_ = ((fp.wrapping_add(64)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = l_s_3; let a_ = (fp.wrapping_add(64)).wrapping_add((((1i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
        { let v_ = l_c_4; let a_ = ((fp.wrapping_add(64)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        F::vu0_b__a_cur_load(c, fp);
        F::vu0_b__a_cur_mul_vec4(c, (fp).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(64)).wrapping_add((((0i32) as u32)).wrapping_mul(16)));
        F::vu0_b__a_cur_mul_vec4(c, (fp).wrapping_add((((1i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(64)).wrapping_add((((1i32) as u32)).wrapping_mul(16)));
        F::vu0_b__a_store_identity(c, fp.wrapping_add(64));
        l_s_3 = { let a0_ = F32(c.m.r32((p_rot_1).wrapping_add(4))); F::Port_sinf(c, a0_) };
        l_c_4 = { let a0_ = F32(c.m.r32((p_rot_1).wrapping_add(4))); F::Port_cosf(c, a0_) };
        { let v_ = l_c_4; let a_ = (fp.wrapping_add(64)).wrapping_add((((0i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
        { let v_ = l_s_3; let a_ = ((fp.wrapping_add(64)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fneg(l_s_3); let a_ = (fp.wrapping_add(64)).wrapping_add((((2i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
        { let v_ = l_c_4; let a_ = ((fp.wrapping_add(64)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_).0); };
        F::vu0_b__a_cur_load(c, fp);
        F::vu0_b__a_cur_mul_vec4(c, (fp).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(64)).wrapping_add((((0i32) as u32)).wrapping_mul(16)));
        F::vu0_b__a_cur_mul_vec4(c, (fp).wrapping_add((((2i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(64)).wrapping_add((((2i32) as u32)).wrapping_mul(16)));
        F::Mtx_MulVec3(c, (fp).wrapping_add((((3i32) as u32)).wrapping_mul(16)), fp, p_trans_2);
        F::Vu0View_LoadMtx(c, fp);
        F::vu0_b__a_cur_pop(c);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Vu0View_SetRotTrans", &PROBE_N, &[], true); }
    r_
}

pub fn Vu0View_ApplyTransRot(c: &mut Ctx, mut p_rot_1: u32, mut p_trans_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vu0View_ApplyTransRot", &PROBE_N, &[(p_rot_1) as u32, (p_trans_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(128) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 128);
    let mut l_s_3: F32 = F32(0);
    let mut l_c_4: F32 = F32(0);
    let r_ = 'f: {
        F::vu0_b__a_cur_push(c);
        F::Vu0View_StoreMtx(c, fp);
        F::Mtx_MulVec3(c, (fp).wrapping_add((((3i32) as u32)).wrapping_mul(16)), fp, p_trans_2);
        F::vu0_b__a_store_identity(c, fp.wrapping_add(64));
        l_s_3 = { let a0_ = F32(c.m.r32((p_rot_1).wrapping_add(4))); F::Port_sinf(c, a0_) };
        l_c_4 = { let a0_ = F32(c.m.r32((p_rot_1).wrapping_add(4))); F::Port_cosf(c, a0_) };
        { let v_ = l_c_4; let a_ = (fp.wrapping_add(64)).wrapping_add((((0i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
        { let v_ = l_s_3; let a_ = ((fp.wrapping_add(64)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fneg(l_s_3); let a_ = (fp.wrapping_add(64)).wrapping_add((((2i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
        { let v_ = l_c_4; let a_ = ((fp.wrapping_add(64)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_).0); };
        F::vu0_b__a_cur_load(c, fp);
        F::vu0_b__a_cur_mul_vec4(c, (fp).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(64)).wrapping_add((((0i32) as u32)).wrapping_mul(16)));
        F::vu0_b__a_cur_mul_vec4(c, (fp).wrapping_add((((2i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(64)).wrapping_add((((2i32) as u32)).wrapping_mul(16)));
        F::vu0_b__a_store_identity(c, fp.wrapping_add(64));
        l_s_3 = { let a0_ = F32(c.m.r32((p_rot_1).wrapping_add(8))); F::Port_sinf(c, a0_) };
        l_c_4 = { let a0_ = F32(c.m.r32((p_rot_1).wrapping_add(8))); F::Port_cosf(c, a0_) };
        { let v_ = l_c_4; let a_ = (fp.wrapping_add(64)).wrapping_add((((0i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fneg(l_s_3); let a_ = ((fp.wrapping_add(64)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = l_s_3; let a_ = (fp.wrapping_add(64)).wrapping_add((((1i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
        { let v_ = l_c_4; let a_ = ((fp.wrapping_add(64)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        F::vu0_b__a_cur_load(c, fp);
        F::vu0_b__a_cur_mul_vec4(c, (fp).wrapping_add((((0i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(64)).wrapping_add((((0i32) as u32)).wrapping_mul(16)));
        F::vu0_b__a_cur_mul_vec4(c, (fp).wrapping_add((((1i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(64)).wrapping_add((((1i32) as u32)).wrapping_mul(16)));
        F::vu0_b__a_store_identity(c, fp.wrapping_add(64));
        l_s_3 = { let a0_ = F32(c.m.r32(p_rot_1)); F::Port_sinf(c, a0_) };
        l_c_4 = { let a0_ = F32(c.m.r32(p_rot_1)); F::Port_cosf(c, a0_) };
        { let v_ = l_c_4; let a_ = ((fp.wrapping_add(64)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fneg(l_s_3); let a_ = ((fp.wrapping_add(64)).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = l_s_3; let a_ = ((fp.wrapping_add(64)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = l_c_4; let a_ = ((fp.wrapping_add(64)).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_).0); };
        F::vu0_b__a_cur_load(c, fp);
        F::vu0_b__a_cur_mul_vec4(c, (fp).wrapping_add((((1i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(64)).wrapping_add((((1i32) as u32)).wrapping_mul(16)));
        F::vu0_b__a_cur_mul_vec4(c, (fp).wrapping_add((((2i32) as u32)).wrapping_mul(16)), (fp.wrapping_add(64)).wrapping_add((((2i32) as u32)).wrapping_mul(16)));
        F::Vu0View_LoadMtx(c, fp);
        F::vu0_b__a_cur_pop(c);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Vu0View_ApplyTransRot", &PROBE_N, &[], true); }
    r_
}

pub fn Vu0Cur_RotateYXZ(c: &mut Ctx, mut p_rot_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vu0Cur_RotateYXZ", &PROBE_N, &[(p_rot_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(192) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 192);
    let mut l_s_2: F32 = F32(0);
    let mut l_c_3: F32 = F32(0);
    let r_ = 'f: {
        F::vu0_b__a_store_identity(c, fp);
        l_c_3 = { let a0_ = F32(c.m.r32((p_rot_1).wrapping_add(4))); F::Port_cosf(c, a0_) };
        { let v_ = l_c_3; let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
        l_s_2 = { let a0_ = F32(c.m.r32((p_rot_1).wrapping_add(4))); F::Port_sinf(c, a0_) };
        { let v_ = l_s_2; let a_ = (fp).wrapping_add((((2i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
        { let v_ = l_c_3; let a_ = ((fp).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fneg(l_s_2); let a_ = ((fp).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let s_ = F::vu0_b__mtx_apply(c, fp.wrapping_add(96), (sym::gRefVu0).wrapping_add((((16i32) as u32)).wrapping_mul(16)), (fp).wrapping_add((((0i32) as u32)).wrapping_mul(16))); let d_ = fp.wrapping_add(64); rt::memcpy(c, d_, s_, 16); };
        { let s_ = F::vu0_b__mtx_apply(c, fp.wrapping_add(112), (sym::gRefVu0).wrapping_add((((16i32) as u32)).wrapping_mul(16)), (fp).wrapping_add((((2i32) as u32)).wrapping_mul(16))); let d_ = fp.wrapping_add(80); rt::memcpy(c, d_, s_, 16); };
        { let s_ = fp.wrapping_add(64); let d_ = (sym::gRefVu0).wrapping_add((((16i32) as u32)).wrapping_mul(16)); rt::memcpy(c, d_, s_, 16); };
        { let s_ = fp.wrapping_add(80); let d_ = (sym::gRefVu0).wrapping_add((((18i32) as u32)).wrapping_mul(16)); rt::memcpy(c, d_, s_, 16); };
        F::vu0_b__a_store_identity(c, fp);
        l_c_3 = { let a0_ = F32(c.m.r32(p_rot_1)); F::Port_cosf(c, a0_) };
        { let v_ = l_c_3; let a_ = ((fp).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        l_s_2 = { let a0_ = F32(c.m.r32(p_rot_1)); F::Port_sinf(c, a0_) };
        { let v_ = l_s_2; let a_ = ((fp).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = l_c_3; let a_ = ((fp).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fneg(l_s_2); let a_ = ((fp).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let s_ = F::vu0_b__mtx_apply(c, fp.wrapping_add(128), (sym::gRefVu0).wrapping_add((((16i32) as u32)).wrapping_mul(16)), (fp).wrapping_add((((1i32) as u32)).wrapping_mul(16))); let d_ = fp.wrapping_add(64); rt::memcpy(c, d_, s_, 16); };
        { let s_ = F::vu0_b__mtx_apply(c, fp.wrapping_add(144), (sym::gRefVu0).wrapping_add((((16i32) as u32)).wrapping_mul(16)), (fp).wrapping_add((((2i32) as u32)).wrapping_mul(16))); let d_ = fp.wrapping_add(80); rt::memcpy(c, d_, s_, 16); };
        { let s_ = fp.wrapping_add(64); let d_ = (sym::gRefVu0).wrapping_add((((17i32) as u32)).wrapping_mul(16)); rt::memcpy(c, d_, s_, 16); };
        { let s_ = fp.wrapping_add(80); let d_ = (sym::gRefVu0).wrapping_add((((18i32) as u32)).wrapping_mul(16)); rt::memcpy(c, d_, s_, 16); };
        F::vu0_b__a_store_identity(c, fp);
        l_c_3 = { let a0_ = F32(c.m.r32((p_rot_1).wrapping_add(8))); F::Port_cosf(c, a0_) };
        { let v_ = l_c_3; let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
        l_s_2 = { let a0_ = F32(c.m.r32((p_rot_1).wrapping_add(8))); F::Port_sinf(c, a0_) };
        { let v_ = l_s_2; let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
        { let v_ = l_c_3; let a_ = ((fp).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fneg(l_s_2); let a_ = ((fp).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let s_ = F::vu0_b__mtx_apply(c, fp.wrapping_add(160), (sym::gRefVu0).wrapping_add((((16i32) as u32)).wrapping_mul(16)), (fp).wrapping_add((((0i32) as u32)).wrapping_mul(16))); let d_ = fp.wrapping_add(64); rt::memcpy(c, d_, s_, 16); };
        { let s_ = F::vu0_b__mtx_apply(c, fp.wrapping_add(176), (sym::gRefVu0).wrapping_add((((16i32) as u32)).wrapping_mul(16)), (fp).wrapping_add((((1i32) as u32)).wrapping_mul(16))); let d_ = fp.wrapping_add(80); rt::memcpy(c, d_, s_, 16); };
        { let s_ = fp.wrapping_add(64); let d_ = (sym::gRefVu0).wrapping_add((((16i32) as u32)).wrapping_mul(16)); rt::memcpy(c, d_, s_, 16); };
        { let s_ = fp.wrapping_add(80); let d_ = (sym::gRefVu0).wrapping_add((((17i32) as u32)).wrapping_mul(16)); rt::memcpy(c, d_, s_, 16); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Vu0Cur_RotateYXZ", &PROBE_N, &[], true); }
    r_
}

pub fn Vu0_Stub0(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vu0_Stub0", &PROBE_N, &[], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Vu0_Stub0", &PROBE_N, &[], true); }
    r_
}

pub fn Vu0_Stub1(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vu0_Stub1", &PROBE_N, &[], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Vu0_Stub1", &PROBE_N, &[], true); }
    r_
}

pub fn Vu0_Stub2(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vu0_Stub2", &PROBE_N, &[], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Vu0_Stub2", &PROBE_N, &[], true); }
    r_
}

pub fn Vu0_Stub3(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vu0_Stub3", &PROBE_N, &[], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Vu0_Stub3", &PROBE_N, &[], true); }
    r_
}

pub fn Vu0_Stub4(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vu0_Stub4", &PROBE_N, &[], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Vu0_Stub4", &PROBE_N, &[], true); }
    r_
}

pub fn Vu0_Stub5(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vu0_Stub5", &PROBE_N, &[], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Vu0_Stub5", &PROBE_N, &[], true); }
    r_
}

pub fn Vu0_Stub6(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vu0_Stub6", &PROBE_N, &[], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Vu0_Stub6", &PROBE_N, &[], true); }
    r_
}

pub fn Vu0_Stub7(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vu0_Stub7", &PROBE_N, &[], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Vu0_Stub7", &PROBE_N, &[], true); }
    r_
}

pub fn ClipVtx_Set(c: &mut Ctx, mut p_vtx_1: u32, mut p_pos_2: u32, mut p_uv_3: u32, mut p_col_4: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ClipVtx_Set", &PROBE_N, &[(p_vtx_1) as u32, (p_pos_2) as u32, (p_uv_3) as u32, (p_col_4) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let r_ = 'f: {
        { let s_ = p_pos_2; rt::memcpy(c, fp, s_, 16); }
        { let s_ = p_uv_3; rt::memcpy(c, fp.wrapping_add(16), s_, 16); }
        { let s_ = p_col_4; rt::memcpy(c, fp.wrapping_add(32), s_, 16); }
        { let s_ = fp; let d_ = p_vtx_1; rt::memcpy(c, d_, s_, 16); };
        { let s_ = fp.wrapping_add(16); let d_ = (p_vtx_1).wrapping_add(16); rt::memcpy(c, d_, s_, 16); };
        { let s_ = fp.wrapping_add(32); let d_ = (p_vtx_1).wrapping_add(32); rt::memcpy(c, d_, s_, 16); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "ClipVtx_Set", &PROBE_N, &[], true); }
    r_
}

pub fn ClipVtx_Copy(c: &mut Ctx, mut p_dst_1: u32, mut p_src_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ClipVtx_Copy", &PROBE_N, &[(p_dst_1) as u32, (p_src_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let r_ = 'f: {
        { let s_ = p_src_2; rt::memcpy(c, fp, s_, 48); }
        { let s_ = fp; let d_ = p_dst_1; rt::memcpy(c, d_, s_, 48); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "ClipVtx_Copy", &PROBE_N, &[], true); }
    r_
}

pub fn ClipVtx_SetArray(c: &mut Ctx, mut p_vtx_1: u32, mut p_pos_2: u32, mut p_uv_3: u32, mut p_col_4: u32, mut p_n_5: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ClipVtx_SetArray", &PROBE_N, &[(p_vtx_1) as u32, (p_pos_2) as u32, (p_uv_3) as u32, (p_col_4) as u32, (p_n_5) as u32], false); }
    let r_ = 'f: {
    if (((((p_n_5) == (0i32)) as i32)) != 0) {
        break 'f;
    }
    'l1: loop {
        'c2: {
            F::ClipVtx_Set(c, { let o_ = p_vtx_1; let n_ = (o_).wrapping_add(48); p_vtx_1 = n_; o_ }, { let o_ = p_pos_2; let n_ = (o_).wrapping_add(16); p_pos_2 = n_; o_ }, { let o_ = p_uv_3; let n_ = (o_).wrapping_add(16); p_uv_3 = n_; o_ }, { let o_ = p_col_4; let n_ = (o_).wrapping_add(16); p_col_4 = n_; o_ });
        }
        if !((((({ let o_ = p_n_5; let n_ = (o_).wrapping_sub(1); p_n_5 = n_; n_ }) != (0i32)) as i32)) != 0) { break 'l1; }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ClipVtx_SetArray", &PROBE_N, &[], true); }
    r_
}

pub fn ClipVtx_CopyArray(c: &mut Ctx, mut p_dst_1: u32, mut p_src_2: u32, mut p_n_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ClipVtx_CopyArray", &PROBE_N, &[(p_dst_1) as u32, (p_src_2) as u32, (p_n_3) as u32], false); }
    let r_ = 'f: {
    if (((((p_n_3) == (0i32)) as i32)) != 0) {
        break 'f;
    }
    'l1: loop {
        'c2: {
            F::ClipVtx_Copy(c, { let o_ = p_dst_1; let n_ = (o_).wrapping_add(48); p_dst_1 = n_; o_ }, { let o_ = p_src_2; let n_ = (o_).wrapping_add(48); p_src_2 = n_; o_ });
        }
        if !((((({ let o_ = p_n_3; let n_ = (o_).wrapping_sub(1); p_n_3 = n_; n_ }) != (0i32)) as i32)) != 0) { break 'l1; }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ClipVtx_CopyArray", &PROBE_N, &[], true); }
    r_
}

pub fn ClipPoly_ClipPlane(c: &mut Ctx, mut p_poly_1: u32, mut p_plane_2: u32, mut p_n_3: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ClipPoly_ClipPlane", &PROBE_N, &[(p_poly_1) as u32, (p_plane_2) as u32, (p_n_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(624) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 624);
    let mut l_dst_4: u32 = 0;
    let mut l_cur_5: u32 = 0;
    let mut l_next_6: u32 = 0;
    let mut l_i_7: i32 = 0;
    let mut l_j_8: i32 = 0;
    let mut l_nOut_9: i32 = 0;
    let mut l_t_10: F32 = F32(0);
    let r_ = 'f: {
        if (((((p_n_3) == (0i32)) as i32)) != 0) {
            break 'f 0i32;
        }
        F::ClipPlane_DistArray(c, fp.wrapping_add(432), p_poly_1, p_plane_2, p_n_3);
        l_nOut_9 = 0i32;
        {
            l_i_7 = 0i32;
            'l1: loop {
                if !(((((l_i_7) < (p_n_3)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let v_ = { let a0_ = F32(c.m.r32((fp.wrapping_add(432)).wrapping_add((((l_i_7) as u32)).wrapping_mul(16)))); F::vu0_b__f_lt(c, a0_, F32(0x00000000)) }; let a_ = (fp.wrapping_add(576)).wrapping_add((((l_i_7) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    { let r_ = (c.m.r32((fp.wrapping_add(576)).wrapping_add((((l_i_7) as u32)).wrapping_mul(4))) as i32); let n_ = (l_nOut_9).wrapping_add(r_); l_nOut_9 = n_; };
                }
                { let o_ = l_i_7; let n_ = (o_).wrapping_add(1); l_i_7 = n_; };
            }
        }
        if (((((l_nOut_9) == (0i32)) as i32)) != 0) {
            break 'f p_n_3;
        }
        if (((((p_n_3) == (l_nOut_9)) as i32)) != 0) {
            break 'f 0i32;
        }
        l_dst_4 = fp;
        l_j_8 = 1i32;
        {
            l_i_7 = 0i32;
            'l3: loop {
                if !(((((l_i_7) < (p_n_3)) as i32)) != 0) { break 'l3; }
                'c4: {
                    l_cur_5 = (p_poly_1).wrapping_add((((l_i_7) as u32)).wrapping_mul(48));
                    if (((((l_j_8) >= (p_n_3)) as i32)) != 0) {
                        l_j_8 = 0i32;
                    }
                    l_next_6 = (p_poly_1).wrapping_add((((l_j_8) as u32)).wrapping_mul(48));
                    if ((((((c.m.r32((fp.wrapping_add(576)).wrapping_add((((l_i_7) as u32)).wrapping_mul(4))) as i32)) == (0i32)) as i32)) != 0) {
                        F::ClipVtx_Copy(c, l_dst_4, l_cur_5);
                        { let o_ = l_dst_4; let n_ = (o_).wrapping_add(48); l_dst_4 = n_; };
                        if ((((((c.m.r32((fp.wrapping_add(576)).wrapping_add((((l_j_8) as u32)).wrapping_mul(4))) as i32)) != (0i32)) as i32)) != 0) {
                            l_t_10 = { let a3_ = F32(c.m.r32((fp.wrapping_add(432)).wrapping_add((((l_i_7) as u32)).wrapping_mul(16)))); F::ClipPlane_EdgeParam(c, p_plane_2, l_cur_5, l_next_6, a3_) };
                            F::ClipVtx_Lerp(c, l_dst_4, l_next_6, l_cur_5, l_t_10);
                            { let o_ = l_dst_4; let n_ = (o_).wrapping_add(48); l_dst_4 = n_; };
                        }
                    } else {
                        if ((((((c.m.r32((fp.wrapping_add(576)).wrapping_add((((l_j_8) as u32)).wrapping_mul(4))) as i32)) == (0i32)) as i32)) != 0) {
                            l_t_10 = { let a3_ = F32(c.m.r32((fp.wrapping_add(432)).wrapping_add((((l_i_7) as u32)).wrapping_mul(16)))); F::ClipPlane_EdgeParam(c, p_plane_2, l_cur_5, l_next_6, a3_) };
                            F::ClipVtx_Lerp(c, l_dst_4, l_next_6, l_cur_5, l_t_10);
                            { let o_ = l_dst_4; let n_ = (o_).wrapping_add(48); l_dst_4 = n_; };
                        }
                    }
                }
                { { let o_ = l_i_7; let n_ = (o_).wrapping_add(1); l_i_7 = n_; }; { let o_ = l_j_8; let n_ = (o_).wrapping_add(1); l_j_8 = n_; }; };
            }
        }
        l_nOut_9 = (((l_dst_4).wrapping_sub(fp) as i32)).wrapping_div(48);
        F::ClipVtx_CopyArray(c, p_poly_1, fp, l_nOut_9);
        break 'f l_nOut_9;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "ClipPoly_ClipPlane", &PROBE_N, &[], true); }
    r_
}

pub fn vu0_b__lerp4(c: &mut Ctx, sret: u32, p_a_1: u32, p_b_2: u32, mut p_t_3: F32, mut p_u_4: F32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "vu0_b__lerp4", &PROBE_N, &[(sret) as u32, (p_a_1) as u32, (p_b_2) as u32, (p_t_3).0, (p_u_4).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    rt::memcpy(c, fp, p_a_1, 16);
    rt::memcpy(c, fp.wrapping_add(16), p_b_2, 16);
    let r_ = 'f: {
        { let v_ = { let a0_ = { let a0_ = F32(c.m.r32(fp)); F::vu0_b__f_mul(c, a0_, p_t_3) }; let a1_ = { let a0_ = F32(c.m.r32(fp.wrapping_add(16))); F::vu0_b__f_mul(c, a0_, p_u_4) }; F::vu0_b__f_add(c, a0_, a1_) }; let a_ = fp.wrapping_add(32); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = { let a0_ = F32(c.m.r32((fp).wrapping_add(4))); F::vu0_b__f_mul(c, a0_, p_t_3) }; let a1_ = { let a0_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))); F::vu0_b__f_mul(c, a0_, p_u_4) }; F::vu0_b__f_add(c, a0_, a1_) }; let a_ = (fp.wrapping_add(32)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = { let a0_ = F32(c.m.r32((fp).wrapping_add(8))); F::vu0_b__f_mul(c, a0_, p_t_3) }; let a1_ = { let a0_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8))); F::vu0_b__f_mul(c, a0_, p_u_4) }; F::vu0_b__f_add(c, a0_, a1_) }; let a_ = (fp.wrapping_add(32)).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = { let a0_ = F32(c.m.r32((fp).wrapping_add(12))); F::vu0_b__f_mul(c, a0_, p_t_3) }; let a1_ = { let a0_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(12))); F::vu0_b__f_mul(c, a0_, p_u_4) }; F::vu0_b__f_add(c, a0_, a1_) }; let a_ = (fp.wrapping_add(32)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        break 'f { let s_ = fp.wrapping_add(32); rt::memcpy(c, sret, s_, 16); sret };
       sret
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "vu0_b__lerp4", &PROBE_N, &[], true); }
    r_
}

pub fn ClipVtx_Lerp(c: &mut Ctx, mut p_out_1: u32, mut p_a_2: u32, mut p_b_3: u32, mut p_t_4: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ClipVtx_Lerp", &PROBE_N, &[(p_out_1) as u32, (p_a_2) as u32, (p_b_3) as u32, (p_t_4).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(144) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 144);
    let mut l_u_5: F32 = F32(0);
    let r_ = 'f: {
        l_u_5 = { let a0_ = F32(c.m.r32(((sym::gRefVu0).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(12))); F::vu0_b__f_sub(c, a0_, p_t_4) };
        { let s_ = p_a_2; rt::memcpy(c, fp, s_, 48); }
        { let s_ = p_b_3; rt::memcpy(c, fp.wrapping_add(48), s_, 48); }
        { let s_ = F::vu0_b__lerp4(c, fp.wrapping_add(96), fp, fp.wrapping_add(48), p_t_4, l_u_5); let d_ = p_out_1; rt::memcpy(c, d_, s_, 16); };
        { let s_ = F::vu0_b__lerp4(c, fp.wrapping_add(112), (fp).wrapping_add(16), (fp.wrapping_add(48)).wrapping_add(16), p_t_4, l_u_5); let d_ = (p_out_1).wrapping_add(16); rt::memcpy(c, d_, s_, 16); };
        { let s_ = F::vu0_b__lerp4(c, fp.wrapping_add(128), (fp).wrapping_add(32), (fp.wrapping_add(48)).wrapping_add(32), p_t_4, l_u_5); let d_ = (p_out_1).wrapping_add(32); rt::memcpy(c, d_, s_, 16); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "ClipVtx_Lerp", &PROBE_N, &[], true); }
    r_
}

pub fn ClipPlane_EdgeParam(c: &mut Ctx, mut p_plane_1: u32, mut p_a_2: u32, mut p_b_3: u32, mut p_dist_4: F32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ClipPlane_EdgeParam", &PROBE_N, &[(p_plane_1) as u32, (p_a_2) as u32, (p_b_3) as u32, (p_dist_4).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        { let v_ = { let a0_ = F32(c.m.r32(p_a_2)); let a1_ = F32(c.m.r32(p_b_3)); F::vu0_b__f_sub(c, a0_, a1_) }; let a_ = fp; c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32((p_a_2).wrapping_add(4))); let a1_ = F32(c.m.r32((p_b_3).wrapping_add(4))); F::vu0_b__f_sub(c, a0_, a1_) }; let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32((p_a_2).wrapping_add(8))); let a1_ = F32(c.m.r32((p_b_3).wrapping_add(8))); F::vu0_b__f_sub(c, a0_, a1_) }; let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((p_a_2).wrapping_add(12))); let a_ = (fp).wrapping_add(12); c.m.w32(a_, (v_).0); };
        break 'f { let a1_ = F::vu0_b__dot3(c, p_plane_1, fp); F::vu0_b__f_div(c, p_dist_4, a1_) };
       F32(0)
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "ClipPlane_EdgeParam", &PROBE_N, &[], true); }
    r_
}

pub fn ClipPlane_DistArray(c: &mut Ctx, mut p_dist_1: u32, mut p_poly_2: u32, mut p_plane_3: u32, mut p_n_4: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ClipPlane_DistArray", &PROBE_N, &[(p_dist_1) as u32, (p_poly_2) as u32, (p_plane_3) as u32, (p_n_4) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_tx_5: F32 = F32(0);
    let mut l_ty_6: F32 = F32(0);
    let mut l_tz_7: F32 = F32(0);
    let mut l_acc_8: F32 = F32(0);
    let r_ = 'f: {
        if (((((p_n_4) == (0i32)) as i32)) != 0) {
            break 'f;
        }
        { let s_ = p_plane_3; let d_ = fp; rt::memcpy(c, d_, s_, 16); };
        { let s_ = p_poly_2; let d_ = fp.wrapping_add(16); rt::memcpy(c, d_, s_, 16); };
        'l1: loop {
            'c2: {
                l_tx_5 = { let a0_ = F32(c.m.r32(fp)); let a1_ = F32(c.m.r32(fp.wrapping_add(16))); F::vu0_b__f_mul(c, a0_, a1_) };
                l_ty_6 = { let a0_ = F32(c.m.r32((fp).wrapping_add(4))); let a1_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))); F::vu0_b__f_mul(c, a0_, a1_) };
                l_tz_7 = { let a0_ = F32(c.m.r32((fp).wrapping_add(8))); let a1_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8))); F::vu0_b__f_mul(c, a0_, a1_) };
                l_acc_8 = F::vu0_b__f_add(c, l_tx_5, l_ty_6);
                l_acc_8 = { let a1_ = { let a0_ = F32(c.m.r32((sym::gRefVu0).wrapping_add((((3i32) as u32)).wrapping_mul(16)))); F::vu0_b__f_mul(c, a0_, l_tz_7) }; F::vu0_b__f_add(c, l_acc_8, a1_) };
                { let v_ = { let a1_ = { let a0_ = F32(c.m.r32((sym::gRefVu0).wrapping_add((((3i32) as u32)).wrapping_mul(16)))); let a1_ = F32(c.m.r32((fp).wrapping_add(12))); F::vu0_b__f_mul(c, a0_, a1_) }; F::vu0_b__f_add(c, l_acc_8, a1_) }; let a_ = (sym::gRefVu0).wrapping_add((((6i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
                { let o_ = p_poly_2; let n_ = (o_).wrapping_add(48); p_poly_2 = n_; };
                if (((((p_n_4) != (1i32)) as i32)) != 0) {
                    { let s_ = p_poly_2; let d_ = fp.wrapping_add(16); rt::memcpy(c, d_, s_, 16); };
                }
                { let s_ = (sym::gRefVu0).wrapping_add((((6i32) as u32)).wrapping_mul(16)); let d_ = { let o_ = p_dist_1; let n_ = (o_).wrapping_add(16); p_dist_1 = n_; o_ }; rt::memcpy(c, d_, s_, 16); };
            }
            if !((((({ let o_ = p_n_4; let n_ = (o_).wrapping_sub(1); p_n_4 = n_; n_ }) != (0i32)) as i32)) != 0) { break 'l1; }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "ClipPlane_DistArray", &PROBE_N, &[], true); }
    r_
}

pub fn vu0_b__clip_project(c: &mut Ctx, mut p_scr_1: u32, mut p_stq_2: u32, mut p_rows_3: u32, mut p_poly_4: u32, mut p_n_5: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "vu0_b__clip_project", &PROBE_N, &[(p_scr_1) as u32, (p_stq_2) as u32, (p_rows_3) as u32, (p_poly_4) as u32, (p_n_5) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(80) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 80);
    let mut l_q_6: F32 = F32(0);
    let r_ = 'f: {
        { let s_ = p_poly_4; rt::memcpy(c, fp, s_, 16); }
        { let s_ = (p_poly_4).wrapping_add(16); rt::memcpy(c, fp.wrapping_add(16), s_, 16); }
        'l1: loop {
            'c2: {
                { let s_ = F::vu0_b__mtx_apply(c, fp.wrapping_add(48), p_rows_3, fp); let d_ = fp.wrapping_add(32); rt::memcpy(c, d_, s_, 16); };
                l_q_6 = F::vu0_b__persp(c, fp.wrapping_add(32));
                F::vu0_b__stq_xyz(c, (sym::gRefVu0).wrapping_add((((13i32) as u32)).wrapping_mul(16)), fp.wrapping_add(16), l_q_6);
                { let o_ = p_poly_4; let n_ = (o_).wrapping_add(48); p_poly_4 = n_; };
                if (((((p_n_5) != (1i32)) as i32)) != 0) {
                    { let s_ = p_poly_4; let d_ = fp; rt::memcpy(c, d_, s_, 16); };
                    { let s_ = (p_poly_4).wrapping_add(16); let d_ = fp.wrapping_add(16); rt::memcpy(c, d_, s_, 16); };
                }
                { let s_ = (sym::gRefVu0).wrapping_add((((13i32) as u32)).wrapping_mul(16)); let d_ = { let o_ = p_stq_2; let n_ = (o_).wrapping_add(16); p_stq_2 = n_; o_ }; rt::memcpy(c, d_, s_, 16); };
                { let s_ = F::vu0_b__to_gs(c, fp.wrapping_add(64), fp.wrapping_add(32)); let d_ = { let o_ = p_scr_1; let n_ = (o_).wrapping_add(16); p_scr_1 = n_; o_ }; rt::memcpy(c, d_, s_, 16); };
            }
            if !((((({ let o_ = p_n_5; let n_ = (o_).wrapping_sub(1); p_n_5 = n_; n_ }) != (0i32)) as i32)) != 0) { break 'l1; }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "vu0_b__clip_project", &PROBE_N, &[], true); }
    r_
}

pub fn ClipPoly_ProjectMtx(c: &mut Ctx, mut p_scr_1: u32, mut p_stq_2: u32, mut p_m_3: u32, mut p_poly_4: u32, mut p_n_5: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ClipPoly_ProjectMtx", &PROBE_N, &[(p_scr_1) as u32, (p_stq_2) as u32, (p_m_3) as u32, (p_poly_4) as u32, (p_n_5) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(64) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 64);
    let r_ = 'f: {
        F::vu0_b__mtx_load(c, fp, p_m_3);
        F::vu0_b__clip_project(c, p_scr_1, p_stq_2, fp, p_poly_4, p_n_5);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "ClipPoly_ProjectMtx", &PROBE_N, &[], true); }
    r_
}

pub fn ClipPoly_ProjectCur(c: &mut Ctx, mut p_scr_1: u32, mut p_stq_2: u32, mut p_poly_3: u32, mut p_n_4: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ClipPoly_ProjectCur", &PROBE_N, &[(p_scr_1) as u32, (p_stq_2) as u32, (p_poly_3) as u32, (p_n_4) as u32], false); }
    let r_ = 'f: {
    F::vu0_b__clip_project(c, p_scr_1, p_stq_2, (sym::gRefVu0).wrapping_add((((16i32) as u32)).wrapping_mul(16)), p_poly_3, p_n_4);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ClipPoly_ProjectCur", &PROBE_N, &[], true); }
    r_
}

pub fn Vu0_Init(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vu0_Init", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::Rand_SeedFloat(c, F32(0x3dfcc088));
    { let v_ = F32(0x00000000); let a_ = (sym::gRefVu0).wrapping_add((((0i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x00000000); let a_ = ((sym::gRefVu0).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x00000000); let a_ = ((sym::gRefVu0).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x3f800000); let a_ = ((sym::gRefVu0).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32(((sym::gRefVu0).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4))); let a_ = (sym::gRefVu0).wrapping_add((((1i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32(((sym::gRefVu0).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8))); let a_ = ((sym::gRefVu0).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32(((sym::gRefVu0).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(12))); let a_ = ((sym::gRefVu0).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((sym::gRefVu0).wrapping_add((((0i32) as u32)).wrapping_mul(16)))); let a_ = ((sym::gRefVu0).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32(((sym::gRefVu0).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(4))); let a_ = (sym::gRefVu0).wrapping_add((((2i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32(((sym::gRefVu0).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(8))); let a_ = ((sym::gRefVu0).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32(((sym::gRefVu0).wrapping_add((((1i32) as u32)).wrapping_mul(16))).wrapping_add(12))); let a_ = ((sym::gRefVu0).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((sym::gRefVu0).wrapping_add((((1i32) as u32)).wrapping_mul(16)))); let a_ = ((sym::gRefVu0).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32(((sym::gRefVu0).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(4))); let a_ = (sym::gRefVu0).wrapping_add((((3i32) as u32)).wrapping_mul(16)); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32(((sym::gRefVu0).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(8))); let a_ = ((sym::gRefVu0).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32(((sym::gRefVu0).wrapping_add((((2i32) as u32)).wrapping_mul(16))).wrapping_add(12))); let a_ = ((sym::gRefVu0).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((sym::gRefVu0).wrapping_add((((2i32) as u32)).wrapping_mul(16)))); let a_ = ((sym::gRefVu0).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add(12); c.m.w32(a_, (v_).0); };
    { let s_ = (sym::gRefVu0).wrapping_add((((0i32) as u32)).wrapping_mul(16)); let d_ = (sym::gRefVu0).wrapping_add((((19i32) as u32)).wrapping_mul(16)); rt::memcpy(c, d_, s_, 16); };
    { let s_ = (sym::gRefVu0).wrapping_add((((1i32) as u32)).wrapping_mul(16)); let d_ = (sym::gRefVu0).wrapping_add((((18i32) as u32)).wrapping_mul(16)); rt::memcpy(c, d_, s_, 16); };
    { let s_ = (sym::gRefVu0).wrapping_add((((2i32) as u32)).wrapping_mul(16)); let d_ = (sym::gRefVu0).wrapping_add((((17i32) as u32)).wrapping_mul(16)); rt::memcpy(c, d_, s_, 16); };
    { let s_ = (sym::gRefVu0).wrapping_add((((3i32) as u32)).wrapping_mul(16)); let d_ = (sym::gRefVu0).wrapping_add((((16i32) as u32)).wrapping_mul(16)); rt::memcpy(c, d_, s_, 16); };
    { let v_ = ((0i32) as u16); let a_ = ((sym::gRefVu0).wrapping_add(532)).wrapping_add((((15i32) as u32)).wrapping_mul(2)); c.m.w16(a_, v_); };
    F::Rand_Init(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Vu0_Init", &PROBE_N, &[], true); }
    r_
}

pub fn Vu0_CheckState(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vu0_CheckState", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(64) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 64);
    let r_ = 'f: {
        F::vu0_b__a_store_identity(c, fp);
        { let _ = fp; };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Vu0_CheckState", &PROBE_N, &[], true); }
    r_
}

pub fn Vec4_SetZeroW1(c: &mut Ctx, mut p_v_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vec4_SetZeroW1", &PROBE_N, &[(p_v_1) as u32], false); }
    let r_ = 'f: {
    { let s_ = (sym::gRefVu0).wrapping_add((((0i32) as u32)).wrapping_mul(16)); let d_ = p_v_1; rt::memcpy(c, d_, s_, 16); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Vec4_SetZeroW1", &PROBE_N, &[], true); }
    r_
}

pub fn Vec4_SetZero(c: &mut Ctx, mut p_v_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vec4_SetZero", &PROBE_N, &[(p_v_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = F32(0x00000000); let a_ = p_v_1; c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x00000000); let a_ = (p_v_1).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x00000000); let a_ = (p_v_1).wrapping_add(8); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x00000000); let a_ = (p_v_1).wrapping_add(12); c.m.w32(a_, (v_).0); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Vec4_SetZero", &PROBE_N, &[], true); }
    r_
}

pub fn Vec4_Set(c: &mut Ctx, mut p_v_1: u32, mut p_x_2: F32, mut p_y_3: F32, mut p_z_4: F32, mut p_w_5: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vec4_Set", &PROBE_N, &[(p_v_1) as u32, (p_x_2).0, (p_y_3).0, (p_z_4).0, (p_w_5).0], false); }
    let r_ = 'f: {
    { let v_ = p_x_2; let a_ = p_v_1; c.m.w32(a_, (v_).0); };
    { let v_ = p_y_3; let a_ = (p_v_1).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = p_z_4; let a_ = (p_v_1).wrapping_add(8); c.m.w32(a_, (v_).0); };
    { let v_ = p_w_5; let a_ = (p_v_1).wrapping_add(12); c.m.w32(a_, (v_).0); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Vec4_Set", &PROBE_N, &[], true); }
    r_
}

pub fn Vec3_Set(c: &mut Ctx, mut p_v_1: u32, mut p_x_2: F32, mut p_y_3: F32, mut p_z_4: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vec3_Set", &PROBE_N, &[(p_v_1) as u32, (p_x_2).0, (p_y_3).0, (p_z_4).0], false); }
    let r_ = 'f: {
    { let v_ = p_x_2; let a_ = p_v_1; c.m.w32(a_, (v_).0); };
    { let v_ = p_y_3; let a_ = (p_v_1).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = p_z_4; let a_ = (p_v_1).wrapping_add(8); c.m.w32(a_, (v_).0); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Vec3_Set", &PROBE_N, &[], true); }
    r_
}

pub fn Vec3_Normalize(c: &mut Ctx, mut p_out_1: u32, mut p_v_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vec3_Normalize", &PROBE_N, &[(p_out_1) as u32, (p_v_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_len_3: F32 = F32(0);
    let mut l_q_4: F32 = F32(0);
    let r_ = 'f: {
        { let s_ = p_v_2; rt::memcpy(c, fp, s_, 16); }
        l_len_3 = { let a0_ = F32(c.m.r32((sym::gRefVu0).wrapping_add((((0i32) as u32)).wrapping_mul(16)))); let a1_ = { let a0_ = F::vu0_b__dot3(c, fp, fp); F::vu0_b__f_sqrt(c, a0_) }; F::vu0_b__f_add(c, a0_, a1_) };
        l_q_4 = { let a0_ = F32(c.m.r32(((sym::gRefVu0).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(12))); F::vu0_b__f_div(c, a0_, l_len_3) };
        { let v_ = { let a0_ = F32(c.m.r32(((sym::gRefVu0).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(12))); let a1_ = F32(c.m.r32(((sym::gRefVu0).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(12))); F::vu0_b__f_sub(c, a0_, a1_) }; let a_ = (fp.wrapping_add(16)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32(fp)); F::vu0_b__f_mul(c, a0_, l_q_4) }; let a_ = fp.wrapping_add(16); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32((fp).wrapping_add(4))); F::vu0_b__f_mul(c, a0_, l_q_4) }; let a_ = (fp.wrapping_add(16)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32((fp).wrapping_add(8))); F::vu0_b__f_mul(c, a0_, l_q_4) }; let a_ = (fp.wrapping_add(16)).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let s_ = fp.wrapping_add(16); let d_ = p_out_1; rt::memcpy(c, d_, s_, 16); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Vec3_Normalize", &PROBE_N, &[], true); }
    r_
}

pub fn Vec4_Swap(c: &mut Ctx, mut p_a_1: u32, mut p_b_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vec4_Swap", &PROBE_N, &[(p_a_1) as u32, (p_b_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let r_ = 'f: {
        { let s_ = p_a_1; rt::memcpy(c, fp, s_, 16); }
        { let s_ = p_b_2; rt::memcpy(c, fp.wrapping_add(16), s_, 16); }
        { let s_ = fp; let d_ = p_b_2; rt::memcpy(c, d_, s_, 16); };
        { let s_ = fp.wrapping_add(16); let d_ = p_a_1; rt::memcpy(c, d_, s_, 16); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Vec4_Swap", &PROBE_N, &[], true); }
    r_
}

pub fn Vec4_Add(c: &mut Ctx, mut p_out_1: u32, mut p_a_2: u32, mut p_b_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vec4_Add", &PROBE_N, &[(p_out_1) as u32, (p_a_2) as u32, (p_b_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let r_ = 'f: {
        { let s_ = p_a_2; rt::memcpy(c, fp, s_, 16); }
        { let s_ = p_b_3; rt::memcpy(c, fp.wrapping_add(16), s_, 16); }
        { let v_ = { let a0_ = F32(c.m.r32(fp)); let a1_ = F32(c.m.r32(fp.wrapping_add(16))); F::vu0_b__f_add(c, a0_, a1_) }; let a_ = fp; c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32((fp).wrapping_add(4))); let a1_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))); F::vu0_b__f_add(c, a0_, a1_) }; let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32((fp).wrapping_add(8))); let a1_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8))); F::vu0_b__f_add(c, a0_, a1_) }; let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32((fp).wrapping_add(12))); let a1_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(12))); F::vu0_b__f_add(c, a0_, a1_) }; let a_ = (fp).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let s_ = fp; let d_ = p_out_1; rt::memcpy(c, d_, s_, 16); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Vec4_Add", &PROBE_N, &[], true); }
    r_
}

pub fn Vec3_Add(c: &mut Ctx, mut p_out_1: u32, mut p_a_2: u32, mut p_b_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vec3_Add", &PROBE_N, &[(p_out_1) as u32, (p_a_2) as u32, (p_b_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let r_ = 'f: {
        { let s_ = p_a_2; rt::memcpy(c, fp, s_, 16); }
        { let s_ = p_b_3; rt::memcpy(c, fp.wrapping_add(16), s_, 16); }
        { let v_ = { let a0_ = F32(c.m.r32(fp)); let a1_ = F32(c.m.r32(fp.wrapping_add(16))); F::vu0_b__f_add(c, a0_, a1_) }; let a_ = fp; c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32((fp).wrapping_add(4))); let a1_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))); F::vu0_b__f_add(c, a0_, a1_) }; let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32((fp).wrapping_add(8))); let a1_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8))); F::vu0_b__f_add(c, a0_, a1_) }; let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let s_ = fp; let d_ = p_out_1; rt::memcpy(c, d_, s_, 16); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Vec3_Add", &PROBE_N, &[], true); }
    r_
}

pub fn Vec4_Sub(c: &mut Ctx, mut p_out_1: u32, mut p_a_2: u32, mut p_b_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vec4_Sub", &PROBE_N, &[(p_out_1) as u32, (p_a_2) as u32, (p_b_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let r_ = 'f: {
        { let s_ = p_a_2; rt::memcpy(c, fp, s_, 16); }
        { let s_ = p_b_3; rt::memcpy(c, fp.wrapping_add(16), s_, 16); }
        { let v_ = { let a0_ = F32(c.m.r32(fp)); let a1_ = F32(c.m.r32(fp.wrapping_add(16))); F::vu0_b__f_sub(c, a0_, a1_) }; let a_ = fp; c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32((fp).wrapping_add(4))); let a1_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))); F::vu0_b__f_sub(c, a0_, a1_) }; let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32((fp).wrapping_add(8))); let a1_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8))); F::vu0_b__f_sub(c, a0_, a1_) }; let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32((fp).wrapping_add(12))); let a1_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(12))); F::vu0_b__f_sub(c, a0_, a1_) }; let a_ = (fp).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let s_ = fp; let d_ = p_out_1; rt::memcpy(c, d_, s_, 16); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Vec4_Sub", &PROBE_N, &[], true); }
    r_
}

pub fn Vec3_Sub(c: &mut Ctx, mut p_out_1: u32, mut p_a_2: u32, mut p_b_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vec3_Sub", &PROBE_N, &[(p_out_1) as u32, (p_a_2) as u32, (p_b_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let r_ = 'f: {
        { let s_ = p_a_2; rt::memcpy(c, fp, s_, 16); }
        { let s_ = p_b_3; rt::memcpy(c, fp.wrapping_add(16), s_, 16); }
        { let v_ = { let a0_ = F32(c.m.r32(fp)); let a1_ = F32(c.m.r32(fp.wrapping_add(16))); F::vu0_b__f_sub(c, a0_, a1_) }; let a_ = fp; c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32((fp).wrapping_add(4))); let a1_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))); F::vu0_b__f_sub(c, a0_, a1_) }; let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32((fp).wrapping_add(8))); let a1_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8))); F::vu0_b__f_sub(c, a0_, a1_) }; let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let s_ = fp; let d_ = p_out_1; rt::memcpy(c, d_, s_, 16); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Vec3_Sub", &PROBE_N, &[], true); }
    r_
}

pub fn Vec4_Mul(c: &mut Ctx, mut p_out_1: u32, mut p_a_2: u32, mut p_b_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vec4_Mul", &PROBE_N, &[(p_out_1) as u32, (p_a_2) as u32, (p_b_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let r_ = 'f: {
        { let s_ = p_a_2; rt::memcpy(c, fp, s_, 16); }
        { let s_ = p_b_3; rt::memcpy(c, fp.wrapping_add(16), s_, 16); }
        { let v_ = { let a0_ = F32(c.m.r32(fp)); let a1_ = F32(c.m.r32(fp.wrapping_add(16))); F::vu0_b__f_mul(c, a0_, a1_) }; let a_ = fp; c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32((fp).wrapping_add(4))); let a1_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))); F::vu0_b__f_mul(c, a0_, a1_) }; let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32((fp).wrapping_add(8))); let a1_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8))); F::vu0_b__f_mul(c, a0_, a1_) }; let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32((fp).wrapping_add(12))); let a1_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(12))); F::vu0_b__f_mul(c, a0_, a1_) }; let a_ = (fp).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let s_ = fp; let d_ = p_out_1; rt::memcpy(c, d_, s_, 16); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Vec4_Mul", &PROBE_N, &[], true); }
    r_
}

pub fn Vec3_Mul(c: &mut Ctx, mut p_out_1: u32, mut p_a_2: u32, mut p_b_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vec3_Mul", &PROBE_N, &[(p_out_1) as u32, (p_a_2) as u32, (p_b_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let r_ = 'f: {
        { let s_ = p_a_2; rt::memcpy(c, fp, s_, 16); }
        { let s_ = p_b_3; rt::memcpy(c, fp.wrapping_add(16), s_, 16); }
        { let v_ = { let a0_ = F32(c.m.r32(fp)); let a1_ = F32(c.m.r32(fp.wrapping_add(16))); F::vu0_b__f_mul(c, a0_, a1_) }; let a_ = fp; c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32((fp).wrapping_add(4))); let a1_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))); F::vu0_b__f_mul(c, a0_, a1_) }; let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32((fp).wrapping_add(8))); let a1_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8))); F::vu0_b__f_mul(c, a0_, a1_) }; let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let s_ = fp; let d_ = p_out_1; rt::memcpy(c, d_, s_, 16); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Vec3_Mul", &PROBE_N, &[], true); }
    r_
}

pub fn Vec4_Scale(c: &mut Ctx, mut p_out_1: u32, mut p_v_2: u32, mut p_s_3: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vec4_Scale", &PROBE_N, &[(p_out_1) as u32, (p_v_2) as u32, (p_s_3).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        { let s_ = p_v_2; rt::memcpy(c, fp, s_, 16); }
        { let v_ = { let a0_ = F32(c.m.r32(fp)); F::vu0_b__f_mul(c, a0_, p_s_3) }; let a_ = fp; c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32((fp).wrapping_add(4))); F::vu0_b__f_mul(c, a0_, p_s_3) }; let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32((fp).wrapping_add(8))); F::vu0_b__f_mul(c, a0_, p_s_3) }; let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32((fp).wrapping_add(12))); F::vu0_b__f_mul(c, a0_, p_s_3) }; let a_ = (fp).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let s_ = fp; let d_ = p_out_1; rt::memcpy(c, d_, s_, 16); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Vec4_Scale", &PROBE_N, &[], true); }
    r_
}

pub fn Vec3_Scale(c: &mut Ctx, mut p_out_1: u32, mut p_v_2: u32, mut p_s_3: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vec3_Scale", &PROBE_N, &[(p_out_1) as u32, (p_v_2) as u32, (p_s_3).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        { let s_ = p_v_2; rt::memcpy(c, fp, s_, 16); }
        { let v_ = { let a0_ = F32(c.m.r32(fp)); F::vu0_b__f_mul(c, a0_, p_s_3) }; let a_ = fp; c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32((fp).wrapping_add(4))); F::vu0_b__f_mul(c, a0_, p_s_3) }; let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32((fp).wrapping_add(8))); F::vu0_b__f_mul(c, a0_, p_s_3) }; let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let s_ = fp; let d_ = p_out_1; rt::memcpy(c, d_, s_, 16); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Vec3_Scale", &PROBE_N, &[], true); }
    r_
}

pub fn Vec4_Div(c: &mut Ctx, mut p_out_1: u32, mut p_v_2: u32, mut p_d_3: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vec4_Div", &PROBE_N, &[(p_out_1) as u32, (p_v_2) as u32, (p_d_3).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_q_4: F32 = F32(0);
    let r_ = 'f: {
        { let s_ = p_v_2; rt::memcpy(c, fp, s_, 16); }
        l_q_4 = { let a0_ = F32(c.m.r32(((sym::gRefVu0).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(12))); F::vu0_b__f_div(c, a0_, p_d_3) };
        { let v_ = { let a0_ = F32(c.m.r32(fp)); F::vu0_b__f_mul(c, a0_, l_q_4) }; let a_ = fp; c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32((fp).wrapping_add(4))); F::vu0_b__f_mul(c, a0_, l_q_4) }; let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32((fp).wrapping_add(8))); F::vu0_b__f_mul(c, a0_, l_q_4) }; let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32((fp).wrapping_add(12))); F::vu0_b__f_mul(c, a0_, l_q_4) }; let a_ = (fp).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let s_ = fp; let d_ = p_out_1; rt::memcpy(c, d_, s_, 16); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Vec4_Div", &PROBE_N, &[], true); }
    r_
}

pub fn Vec3_Div(c: &mut Ctx, mut p_out_1: u32, mut p_v_2: u32, mut p_d_3: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vec3_Div", &PROBE_N, &[(p_out_1) as u32, (p_v_2) as u32, (p_d_3).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_q_4: F32 = F32(0);
    let r_ = 'f: {
        { let s_ = p_v_2; rt::memcpy(c, fp, s_, 16); }
        l_q_4 = { let a0_ = F32(c.m.r32(((sym::gRefVu0).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(12))); F::vu0_b__f_div(c, a0_, p_d_3) };
        { let v_ = { let a0_ = F32(c.m.r32(fp)); F::vu0_b__f_mul(c, a0_, l_q_4) }; let a_ = fp; c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32((fp).wrapping_add(4))); F::vu0_b__f_mul(c, a0_, l_q_4) }; let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32((fp).wrapping_add(8))); F::vu0_b__f_mul(c, a0_, l_q_4) }; let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let s_ = fp; let d_ = p_out_1; rt::memcpy(c, d_, s_, 16); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Vec3_Div", &PROBE_N, &[], true); }
    r_
}

pub fn Vec4_Copy(c: &mut Ctx, mut p_dst_1: u32, mut p_src_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vec4_Copy", &PROBE_N, &[(p_dst_1) as u32, (p_src_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        { let s_ = p_src_2; rt::memcpy(c, fp, s_, 16); }
        { let s_ = fp; let d_ = p_dst_1; rt::memcpy(c, d_, s_, 16); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Vec4_Copy", &PROBE_N, &[], true); }
    r_
}

pub fn Vec3_Copy(c: &mut Ctx, mut p_dst_1: u32, mut p_src_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vec3_Copy", &PROBE_N, &[(p_dst_1) as u32, (p_src_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        { let s_ = p_src_2; rt::memcpy(c, fp, s_, 16); }
        { let v_ = F32(c.m.r32(fp)); let a_ = p_dst_1; c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((fp).wrapping_add(4))); let a_ = (p_dst_1).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = F32(c.m.r32((fp).wrapping_add(8))); let a_ = (p_dst_1).wrapping_add(8); c.m.w32(a_, (v_).0); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Vec3_Copy", &PROBE_N, &[], true); }
    r_
}

pub fn Mtx_MulVec4(c: &mut Ctx, mut p_out_1: u32, mut p_m_2: u32, mut p_v_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Mtx_MulVec4", &PROBE_N, &[(p_out_1) as u32, (p_m_2) as u32, (p_v_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(96) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 96);
    let r_ = 'f: {
        { let s_ = p_v_3; rt::memcpy(c, fp, s_, 16); }
        F::vu0_b__mtx_load(c, fp.wrapping_add(16), p_m_2);
        { let s_ = F::vu0_b__mtx_apply(c, fp.wrapping_add(80), fp.wrapping_add(16), fp); let d_ = p_out_1; rt::memcpy(c, d_, s_, 16); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Mtx_MulVec4", &PROBE_N, &[], true); }
    r_
}

pub fn Mtx_MulVec3(c: &mut Ctx, mut p_out_1: u32, mut p_m_2: u32, mut p_v_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Mtx_MulVec3", &PROBE_N, &[(p_out_1) as u32, (p_m_2) as u32, (p_v_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(112) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 112);
    let r_ = 'f: {
        { let s_ = p_v_3; rt::memcpy(c, fp, s_, 16); }
        F::vu0_b__mtx_load(c, fp.wrapping_add(16), p_m_2);
        { let s_ = F::vu0_b__mtx_apply(c, fp.wrapping_add(96), fp.wrapping_add(16), fp); let d_ = fp.wrapping_add(80); rt::memcpy(c, d_, s_, 16); };
        { let v_ = F32(c.m.r32((fp).wrapping_add(12))); let a_ = (fp.wrapping_add(80)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let s_ = fp.wrapping_add(80); let d_ = p_out_1; rt::memcpy(c, d_, s_, 16); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Mtx_MulVec3", &PROBE_N, &[], true); }
    r_
}

pub fn Vec4_RotateEuler(c: &mut Ctx, mut p_out_1: u32, mut p_angles_2: u32, mut p_v_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vec4_RotateEuler", &PROBE_N, &[(p_out_1) as u32, (p_angles_2) as u32, (p_v_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(64) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 64);
    let r_ = 'f: {
        F::Mtx_RotateZXY(c, fp, sym::vu0_b__Vec4_RotateEuler__identity, p_angles_2);
        F::Mtx_MulVec4(c, p_out_1, fp, p_v_3);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Vec4_RotateEuler", &PROBE_N, &[], true); }
    r_
}

pub fn Vec3_Dot(c: &mut Ctx, mut p_a_1: u32, mut p_b_2: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vec3_Dot", &PROBE_N, &[(p_a_1) as u32, (p_b_2) as u32], false); }
    let r_ = 'f: {
    break 'f F::vu0_b__dot3(c, p_a_1, p_b_2);
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "Vec3_Dot", &PROBE_N, &[], true); }
    r_
}

pub fn Vec3_Cross(c: &mut Ctx, mut p_out_1: u32, mut p_a_2: u32, mut p_b_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vec3_Cross", &PROBE_N, &[(p_out_1) as u32, (p_a_2) as u32, (p_b_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let r_ = 'f: {
        { let s_ = p_a_2; rt::memcpy(c, fp, s_, 16); }
        { let s_ = p_b_3; rt::memcpy(c, fp.wrapping_add(16), s_, 16); }
        { let v_ = { let a0_ = { let a0_ = F32(c.m.r32((fp).wrapping_add(4))); let a1_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8))); F::vu0_b__f_mul(c, a0_, a1_) }; let a1_ = { let a0_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))); let a1_ = F32(c.m.r32((fp).wrapping_add(8))); F::vu0_b__f_mul(c, a0_, a1_) }; F::vu0_b__f_sub(c, a0_, a1_) }; let a_ = fp.wrapping_add(32); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = { let a0_ = F32(c.m.r32((fp).wrapping_add(8))); let a1_ = F32(c.m.r32(fp.wrapping_add(16))); F::vu0_b__f_mul(c, a0_, a1_) }; let a1_ = { let a0_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8))); let a1_ = F32(c.m.r32(fp)); F::vu0_b__f_mul(c, a0_, a1_) }; F::vu0_b__f_sub(c, a0_, a1_) }; let a_ = (fp.wrapping_add(32)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = { let a0_ = F32(c.m.r32(fp)); let a1_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))); F::vu0_b__f_mul(c, a0_, a1_) }; let a1_ = { let a0_ = F32(c.m.r32(fp.wrapping_add(16))); let a1_ = F32(c.m.r32((fp).wrapping_add(4))); F::vu0_b__f_mul(c, a0_, a1_) }; F::vu0_b__f_sub(c, a0_, a1_) }; let a_ = (fp.wrapping_add(32)).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(12))); let a1_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(12))); F::vu0_b__f_sub(c, a0_, a1_) }; let a_ = (fp.wrapping_add(32)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let s_ = fp.wrapping_add(32); let d_ = p_out_1; rt::memcpy(c, d_, s_, 16); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Vec3_Cross", &PROBE_N, &[], true); }
    r_
}

pub fn Vec4_ToFixed12(c: &mut Ctx, mut p_out_1: u32, mut p_v_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vec4_ToFixed12", &PROBE_N, &[(p_out_1) as u32, (p_v_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        { let s_ = p_v_2; rt::memcpy(c, fp, s_, 16); }
        { let v_ = { let a0_ = F32(c.m.r32(fp)); F::vu0_b__f_toi(c, a0_, 12i32) }; let a_ = p_out_1; c.m.w32(a_, (v_) as u32); };
        { let v_ = { let a0_ = F32(c.m.r32((fp).wrapping_add(4))); F::vu0_b__f_toi(c, a0_, 12i32) }; let a_ = (p_out_1).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = { let a0_ = F32(c.m.r32((fp).wrapping_add(8))); F::vu0_b__f_toi(c, a0_, 12i32) }; let a_ = (p_out_1).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = { let a0_ = F32(c.m.r32((fp).wrapping_add(12))); F::vu0_b__f_toi(c, a0_, 12i32) }; let a_ = (p_out_1).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Vec4_ToFixed12", &PROBE_N, &[], true); }
    r_
}

pub fn Vec4_ToFixed4(c: &mut Ctx, mut p_out_1: u32, mut p_v_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vec4_ToFixed4", &PROBE_N, &[(p_out_1) as u32, (p_v_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        { let s_ = p_v_2; rt::memcpy(c, fp, s_, 16); }
        { let v_ = { let a0_ = F32(c.m.r32(fp)); F::vu0_b__f_toi(c, a0_, 4i32) }; let a_ = p_out_1; c.m.w32(a_, (v_) as u32); };
        { let v_ = { let a0_ = F32(c.m.r32((fp).wrapping_add(4))); F::vu0_b__f_toi(c, a0_, 4i32) }; let a_ = (p_out_1).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = { let a0_ = F32(c.m.r32((fp).wrapping_add(8))); F::vu0_b__f_toi(c, a0_, 4i32) }; let a_ = (p_out_1).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = { let a0_ = F32(c.m.r32((fp).wrapping_add(12))); F::vu0_b__f_toi(c, a0_, 4i32) }; let a_ = (p_out_1).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Vec4_ToFixed4", &PROBE_N, &[], true); }
    r_
}

pub fn Vec4_ToInt(c: &mut Ctx, mut p_out_1: u32, mut p_v_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vec4_ToInt", &PROBE_N, &[(p_out_1) as u32, (p_v_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        { let s_ = p_v_2; rt::memcpy(c, fp, s_, 16); }
        { let v_ = { let a0_ = F32(c.m.r32(fp)); F::vu0_b__f_toi(c, a0_, 0i32) }; let a_ = p_out_1; c.m.w32(a_, (v_) as u32); };
        { let v_ = { let a0_ = F32(c.m.r32((fp).wrapping_add(4))); F::vu0_b__f_toi(c, a0_, 0i32) }; let a_ = (p_out_1).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = { let a0_ = F32(c.m.r32((fp).wrapping_add(8))); F::vu0_b__f_toi(c, a0_, 0i32) }; let a_ = (p_out_1).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = { let a0_ = F32(c.m.r32((fp).wrapping_add(12))); F::vu0_b__f_toi(c, a0_, 0i32) }; let a_ = (p_out_1).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Vec4_ToInt", &PROBE_N, &[], true); }
    r_
}

pub fn Vec4_ToFixed4XY(c: &mut Ctx, mut p_out_1: u32, mut p_v_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vec4_ToFixed4XY", &PROBE_N, &[(p_out_1) as u32, (p_v_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        { let s_ = F::vu0_b__to_gs(c, fp, p_v_2); let d_ = p_out_1; rt::memcpy(c, d_, s_, 16); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Vec4_ToFixed4XY", &PROBE_N, &[], true); }
    r_
}

pub fn Vec4_Clamp(c: &mut Ctx, mut p_out_1: u32, mut p_v_2: u32, mut p_lo_3: F32, mut p_hi_4: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vec4_Clamp", &PROBE_N, &[(p_out_1) as u32, (p_v_2) as u32, (p_lo_3).0, (p_hi_4).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        { let s_ = p_v_2; rt::memcpy(c, fp, s_, 16); }
        { let v_ = { let a0_ = { let a0_ = F32(c.m.r32(fp)); F::vu0_b__f_maxf(c, a0_, p_lo_3) }; F::vu0_b__f_minf(c, a0_, p_hi_4) }; let a_ = fp; c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = { let a0_ = F32(c.m.r32((fp).wrapping_add(4))); F::vu0_b__f_maxf(c, a0_, p_lo_3) }; F::vu0_b__f_minf(c, a0_, p_hi_4) }; let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = { let a0_ = F32(c.m.r32((fp).wrapping_add(8))); F::vu0_b__f_maxf(c, a0_, p_lo_3) }; F::vu0_b__f_minf(c, a0_, p_hi_4) }; let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = { let a0_ = F32(c.m.r32((fp).wrapping_add(12))); F::vu0_b__f_maxf(c, a0_, p_lo_3) }; F::vu0_b__f_minf(c, a0_, p_hi_4) }; let a_ = (fp).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let s_ = fp; let d_ = p_out_1; rt::memcpy(c, d_, s_, 16); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Vec4_Clamp", &PROBE_N, &[], true); }
    r_
}

pub fn Vec3_Clamp(c: &mut Ctx, mut p_out_1: u32, mut p_v_2: u32, mut p_lo_3: F32, mut p_hi_4: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vec3_Clamp", &PROBE_N, &[(p_out_1) as u32, (p_v_2) as u32, (p_lo_3).0, (p_hi_4).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        { let s_ = p_v_2; rt::memcpy(c, fp, s_, 16); }
        { let v_ = { let a0_ = { let a0_ = F32(c.m.r32(fp)); F::vu0_b__f_maxf(c, a0_, p_lo_3) }; F::vu0_b__f_minf(c, a0_, p_hi_4) }; let a_ = fp; c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = { let a0_ = F32(c.m.r32((fp).wrapping_add(4))); F::vu0_b__f_maxf(c, a0_, p_lo_3) }; F::vu0_b__f_minf(c, a0_, p_hi_4) }; let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = { let a0_ = F32(c.m.r32((fp).wrapping_add(8))); F::vu0_b__f_maxf(c, a0_, p_lo_3) }; F::vu0_b__f_minf(c, a0_, p_hi_4) }; let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let s_ = fp; let d_ = p_out_1; rt::memcpy(c, d_, s_, 16); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Vec3_Clamp", &PROBE_N, &[], true); }
    r_
}

pub fn Vec4_Lerp(c: &mut Ctx, mut p_out_1: u32, mut p_a_2: u32, mut p_b_3: u32, mut p_t_4: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vec4_Lerp", &PROBE_N, &[(p_out_1) as u32, (p_a_2) as u32, (p_b_3) as u32, (p_t_4).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let r_ = 'f: {
        { let s_ = p_a_2; rt::memcpy(c, fp, s_, 16); }
        { let s_ = p_b_3; rt::memcpy(c, fp.wrapping_add(16), s_, 16); }
        { let s_ = { let a3_ = { let a0_ = F32(c.m.r32(((sym::gRefVu0).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(12))); F::vu0_b__f_sub(c, a0_, p_t_4) }; F::vu0_b__lerp4(c, fp.wrapping_add(32), fp, fp.wrapping_add(16), p_t_4, a3_) }; let d_ = p_out_1; rt::memcpy(c, d_, s_, 16); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Vec4_Lerp", &PROBE_N, &[], true); }
    r_
}

pub fn Vec3_Lerp(c: &mut Ctx, mut p_out_1: u32, mut p_a_2: u32, mut p_b_3: u32, mut p_t_4: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vec3_Lerp", &PROBE_N, &[(p_out_1) as u32, (p_a_2) as u32, (p_b_3) as u32, (p_t_4).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(64) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 64);
    let r_ = 'f: {
        { let s_ = p_a_2; rt::memcpy(c, fp, s_, 16); }
        { let s_ = p_b_3; rt::memcpy(c, fp.wrapping_add(16), s_, 16); }
        { let s_ = { let a3_ = { let a0_ = F32(c.m.r32(((sym::gRefVu0).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(12))); F::vu0_b__f_sub(c, a0_, p_t_4) }; F::vu0_b__lerp4(c, fp.wrapping_add(48), fp, fp.wrapping_add(16), p_t_4, a3_) }; rt::memcpy(c, fp.wrapping_add(32), s_, 16); }
        { let v_ = F32(c.m.r32((fp).wrapping_add(12))); let a_ = (fp.wrapping_add(32)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let s_ = fp.wrapping_add(32); let d_ = p_out_1; rt::memcpy(c, d_, s_, 16); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Vec3_Lerp", &PROBE_N, &[], true); }
    r_
}

pub fn Vec3_Length(c: &mut Ctx, mut p_v_1: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vec3_Length", &PROBE_N, &[(p_v_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        { let s_ = p_v_1; rt::memcpy(c, fp, s_, 16); }
        break 'f { let a0_ = F::vu0_b__dot3(c, fp, fp); F::vu0_b__f_sqrt(c, a0_) };
       F32(0)
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Vec3_Length", &PROBE_N, &[], true); }
    r_
}

pub fn Vec3_LengthSq(c: &mut Ctx, mut p_v_1: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vec3_LengthSq", &PROBE_N, &[(p_v_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        { let s_ = p_v_1; rt::memcpy(c, fp, s_, 16); }
        break 'f F::vu0_b__dot3(c, fp, fp);
       F32(0)
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Vec3_LengthSq", &PROBE_N, &[], true); }
    r_
}

pub fn Vec3_Dist(c: &mut Ctx, mut p_a_1: u32, mut p_b_2: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vec3_Dist", &PROBE_N, &[(p_a_1) as u32, (p_b_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        { let v_ = { let a0_ = F32(c.m.r32(p_a_1)); let a1_ = F32(c.m.r32(p_b_2)); F::vu0_b__f_sub(c, a0_, a1_) }; let a_ = fp; c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32((p_a_1).wrapping_add(4))); let a1_ = F32(c.m.r32((p_b_2).wrapping_add(4))); F::vu0_b__f_sub(c, a0_, a1_) }; let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32((p_a_1).wrapping_add(8))); let a1_ = F32(c.m.r32((p_b_2).wrapping_add(8))); F::vu0_b__f_sub(c, a0_, a1_) }; let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32((p_a_1).wrapping_add(12))); let a1_ = F32(c.m.r32((p_b_2).wrapping_add(12))); F::vu0_b__f_sub(c, a0_, a1_) }; let a_ = (fp).wrapping_add(12); c.m.w32(a_, (v_).0); };
        break 'f { let a0_ = F::vu0_b__dot3(c, fp, fp); F::vu0_b__f_sqrt(c, a0_) };
       F32(0)
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Vec3_Dist", &PROBE_N, &[], true); }
    r_
}

pub fn Vec3_DistSq(c: &mut Ctx, mut p_a_1: u32, mut p_b_2: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vec3_DistSq", &PROBE_N, &[(p_a_1) as u32, (p_b_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        { let v_ = { let a0_ = F32(c.m.r32(p_a_1)); let a1_ = F32(c.m.r32(p_b_2)); F::vu0_b__f_sub(c, a0_, a1_) }; let a_ = fp; c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32((p_a_1).wrapping_add(4))); let a1_ = F32(c.m.r32((p_b_2).wrapping_add(4))); F::vu0_b__f_sub(c, a0_, a1_) }; let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32((p_a_1).wrapping_add(8))); let a1_ = F32(c.m.r32((p_b_2).wrapping_add(8))); F::vu0_b__f_sub(c, a0_, a1_) }; let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32((p_a_1).wrapping_add(12))); let a1_ = F32(c.m.r32((p_b_2).wrapping_add(12))); F::vu0_b__f_sub(c, a0_, a1_) }; let a_ = (fp).wrapping_add(12); c.m.w32(a_, (v_).0); };
        break 'f F::vu0_b__dot3(c, fp, fp);
       F32(0)
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Vec3_DistSq", &PROBE_N, &[], true); }
    r_
}

pub fn Vec3_DirToEuler(c: &mut Ctx, mut p_out_1: u32, mut p_dir_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vec3_DirToEuler", &PROBE_N, &[(p_out_1) as u32, (p_dir_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        { let v_ = { let a0_ = F32(c.m.r32(p_dir_2)); let a1_ = F32(c.m.r32((p_dir_2).wrapping_add(8))); F::Port_atan2f(c, a0_, a1_) }; let a_ = (p_out_1).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (fp).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let a1_ = F32(c.m.r32(p_dir_2)); let a3_ = F32(c.m.r32((p_dir_2).wrapping_add(8))); F::Vec3_Set(c, fp, a1_, F32(0x00000000), a3_) };
        { let v_ = rt::fneg({ let a0_ = F32(c.m.r32((p_dir_2).wrapping_add(4))); let a1_ = F::Vec3_Length(c, fp); F::Port_atan2f(c, a0_, a1_) }); let a_ = p_out_1; c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (p_out_1).wrapping_add(8); c.m.w32(a_, (v_).0); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Vec3_DirToEuler", &PROBE_N, &[], true); }
    r_
}

pub fn Vec3_DiffToEuler(c: &mut Ctx, mut p_out_1: u32, mut p_a_2: u32, mut p_b_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vec3_DiffToEuler", &PROBE_N, &[(p_out_1) as u32, (p_a_2) as u32, (p_b_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        F::Vec3_Sub(c, fp, p_a_2, p_b_3);
        F::Vec3_DirToEuler(c, p_out_1, fp);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Vec3_DiffToEuler", &PROBE_N, &[], true); }
    r_
}

pub fn Mtx_ProjectInt(c: &mut Ctx, mut p_out_1: u32, mut p_m_2: u32, mut p_v_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Mtx_ProjectInt", &PROBE_N, &[(p_out_1) as u32, (p_m_2) as u32, (p_v_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(112) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 112);
    let r_ = 'f: {
        { let s_ = p_v_3; rt::memcpy(c, fp, s_, 16); }
        F::vu0_b__mtx_load(c, fp.wrapping_add(16), p_m_2);
        { let s_ = F::vu0_b__mtx_apply(c, fp.wrapping_add(96), fp.wrapping_add(16), fp); let d_ = fp.wrapping_add(80); rt::memcpy(c, d_, s_, 16); };
        { let _ = F::vu0_b__persp(c, fp.wrapping_add(80)); };
        { let v_ = { let a0_ = F32(c.m.r32(fp.wrapping_add(80))); F::vu0_b__f_toi(c, a0_, 0i32) }; let a_ = p_out_1; c.m.w32(a_, (v_) as u32); };
        { let v_ = { let a0_ = F32(c.m.r32((fp.wrapping_add(80)).wrapping_add(4))); F::vu0_b__f_toi(c, a0_, 0i32) }; let a_ = (p_out_1).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = { let a0_ = F32(c.m.r32((fp.wrapping_add(80)).wrapping_add(8))); F::vu0_b__f_toi(c, a0_, 0i32) }; let a_ = (p_out_1).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = { let a0_ = F32(c.m.r32((fp.wrapping_add(80)).wrapping_add(12))); F::vu0_b__f_toi(c, a0_, 0i32) }; let a_ = (p_out_1).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Mtx_ProjectInt", &PROBE_N, &[], true); }
    r_
}

pub fn Mtx_ProjectPoint(c: &mut Ctx, mut p_out_1: u32, mut p_m_2: u32, mut p_v_3: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Mtx_ProjectPoint", &PROBE_N, &[(p_out_1) as u32, (p_m_2) as u32, (p_v_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(128) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 128);
    let r_ = 'f: {
        { let s_ = p_v_3; rt::memcpy(c, fp, s_, 16); }
        F::vu0_b__mtx_load(c, fp.wrapping_add(16), p_m_2);
        { let s_ = F::vu0_b__mtx_apply(c, fp.wrapping_add(96), fp.wrapping_add(16), fp); let d_ = fp.wrapping_add(80); rt::memcpy(c, d_, s_, 16); };
        { let _ = F::vu0_b__persp(c, fp.wrapping_add(80)); };
        { let s_ = F::vu0_b__to_gs(c, fp.wrapping_add(112), fp.wrapping_add(80)); let d_ = p_out_1; rt::memcpy(c, d_, s_, 16); };
        break 'f F::vu0_b__on_screen(c, fp.wrapping_add(80));
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Mtx_ProjectPoint", &PROBE_N, &[], true); }
    r_
}

pub fn Mtx_ProjectPoints(c: &mut Ctx, mut p_out_1: u32, mut p_m_2: u32, mut p_v_3: u32, mut p_n_4: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Mtx_ProjectPoints", &PROBE_N, &[(p_out_1) as u32, (p_m_2) as u32, (p_v_3) as u32, (p_n_4) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(128) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 128);
    let r_ = 'f: {
        { let s_ = p_v_3; rt::memcpy(c, fp, s_, 16); }
        F::vu0_b__mtx_load(c, fp.wrapping_add(16), p_m_2);
        {
            'l1: loop {
                'c2: {
                    { let s_ = F::vu0_b__mtx_apply(c, fp.wrapping_add(96), fp.wrapping_add(16), fp); let d_ = fp.wrapping_add(80); rt::memcpy(c, d_, s_, 16); };
                    { let _ = F::vu0_b__persp(c, fp.wrapping_add(80)); };
                    { let o_ = p_v_3; let n_ = (o_).wrapping_add(16); p_v_3 = n_; };
                    if (((((p_n_4) != (1i32)) as i32)) != 0) {
                        { let s_ = p_v_3; let d_ = fp; rt::memcpy(c, d_, s_, 16); };
                    }
                    { let s_ = F::vu0_b__to_gs(c, fp.wrapping_add(112), fp.wrapping_add(80)); let d_ = p_out_1; rt::memcpy(c, d_, s_, 16); };
                    if ((((!((F::vu0_b__on_screen(c, fp.wrapping_add(80))) != 0)) as i32)) != 0) {
                        break 'f 0i32;
                    }
                    if ((((({ let o_ = p_n_4; let n_ = (o_).wrapping_sub(1); p_n_4 = n_; n_ }) == (0i32)) as i32)) != 0) {
                        break 'f 1i32;
                    }
                    { let o_ = p_out_1; let n_ = (o_).wrapping_add(16); p_out_1 = n_; };
                }
            }
        }
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Mtx_ProjectPoints", &PROBE_N, &[], true); }
    r_
}

pub fn Mtx_ProjectPointStq(c: &mut Ctx, mut p_out_1: u32, mut p_stq_2: u32, mut p_m_3: u32, mut p_v_4: u32, mut p_uv_5: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Mtx_ProjectPointStq", &PROBE_N, &[(p_out_1) as u32, (p_stq_2) as u32, (p_m_3) as u32, (p_v_4) as u32, (p_uv_5) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(144) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 144);
    let mut l_q_6: F32 = F32(0);
    let r_ = 'f: {
        { let s_ = p_v_4; rt::memcpy(c, fp, s_, 16); }
        { let s_ = p_uv_5; rt::memcpy(c, fp.wrapping_add(16), s_, 16); }
        F::vu0_b__mtx_load(c, fp.wrapping_add(32), p_m_3);
        { let s_ = F::vu0_b__mtx_apply(c, fp.wrapping_add(112), fp.wrapping_add(32), fp); let d_ = fp.wrapping_add(96); rt::memcpy(c, d_, s_, 16); };
        l_q_6 = F::vu0_b__persp(c, fp.wrapping_add(96));
        F::vu0_b__stq_xyz(c, fp.wrapping_add(16), fp.wrapping_add(16), l_q_6);
        { let s_ = fp.wrapping_add(16); let d_ = p_stq_2; rt::memcpy(c, d_, s_, 16); };
        { let s_ = F::vu0_b__to_gs(c, fp.wrapping_add(128), fp.wrapping_add(96)); let d_ = p_out_1; rt::memcpy(c, d_, s_, 16); };
        break 'f F::vu0_b__on_screen(c, fp.wrapping_add(96));
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Mtx_ProjectPointStq", &PROBE_N, &[], true); }
    r_
}

pub fn Mtx_ProjectPointsStq(c: &mut Ctx, mut p_out_1: u32, mut p_stq_2: u32, mut p_m_3: u32, mut p_v_4: u32, mut p_uv_5: u32, mut p_n_6: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Mtx_ProjectPointsStq", &PROBE_N, &[(p_out_1) as u32, (p_stq_2) as u32, (p_m_3) as u32, (p_v_4) as u32, (p_uv_5) as u32, (p_n_6) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(144) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 144);
    let mut l_q_7: F32 = F32(0);
    let r_ = 'f: {
        { let s_ = p_v_4; rt::memcpy(c, fp, s_, 16); }
        { let s_ = p_uv_5; rt::memcpy(c, fp.wrapping_add(16), s_, 16); }
        F::vu0_b__mtx_load(c, fp.wrapping_add(32), p_m_3);
        {
            'l1: loop {
                'c2: {
                    { let s_ = F::vu0_b__mtx_apply(c, fp.wrapping_add(112), fp.wrapping_add(32), fp); let d_ = fp.wrapping_add(96); rt::memcpy(c, d_, s_, 16); };
                    l_q_7 = F::vu0_b__persp(c, fp.wrapping_add(96));
                    F::vu0_b__stq_xyz(c, (sym::gRefVu0).wrapping_add((((14i32) as u32)).wrapping_mul(16)), fp.wrapping_add(16), l_q_7);
                    { let o_ = p_v_4; let n_ = (o_).wrapping_add(16); p_v_4 = n_; };
                    { let o_ = p_uv_5; let n_ = (o_).wrapping_add(16); p_uv_5 = n_; };
                    if (((((p_n_6) != (1i32)) as i32)) != 0) {
                        { let s_ = p_v_4; let d_ = fp; rt::memcpy(c, d_, s_, 16); };
                        { let s_ = p_uv_5; let d_ = fp.wrapping_add(16); rt::memcpy(c, d_, s_, 16); };
                    }
                    { let s_ = (sym::gRefVu0).wrapping_add((((14i32) as u32)).wrapping_mul(16)); let d_ = p_stq_2; rt::memcpy(c, d_, s_, 16); };
                    { let s_ = F::vu0_b__to_gs(c, fp.wrapping_add(128), fp.wrapping_add(96)); let d_ = p_out_1; rt::memcpy(c, d_, s_, 16); };
                    if ((((!((F::vu0_b__on_screen(c, fp.wrapping_add(96))) != 0)) as i32)) != 0) {
                        break 'f 0i32;
                    }
                    if ((((({ let o_ = p_n_6; let n_ = (o_).wrapping_sub(1); p_n_6 = n_; n_ }) == (0i32)) as i32)) != 0) {
                        break 'f 1i32;
                    }
                    { let o_ = p_out_1; let n_ = (o_).wrapping_add(16); p_out_1 = n_; };
                    { let o_ = p_stq_2; let n_ = (o_).wrapping_add(16); p_stq_2 = n_; };
                }
            }
        }
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Mtx_ProjectPointsStq", &PROBE_N, &[], true); }
    r_
}

pub fn Vu0_Stub8(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vu0_Stub8", &PROBE_N, &[], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Vu0_Stub8", &PROBE_N, &[], true); }
    r_
}

pub fn Vec3_AddSub(c: &mut Ctx, mut p_out_1: u32, mut p_a_2: u32, mut p_b_3: u32, mut p_c_4: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vec3_AddSub", &PROBE_N, &[(p_out_1) as u32, (p_a_2) as u32, (p_b_3) as u32, (p_c_4) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let r_ = 'f: {
        { let s_ = p_a_2; rt::memcpy(c, fp, s_, 16); }
        { let s_ = p_b_3; rt::memcpy(c, fp.wrapping_add(16), s_, 16); }
        { let s_ = p_c_4; rt::memcpy(c, fp.wrapping_add(32), s_, 16); }
        { let v_ = { let a0_ = { let a0_ = F32(c.m.r32(fp)); let a1_ = F32(c.m.r32(fp.wrapping_add(16))); F::vu0_b__f_add(c, a0_, a1_) }; let a1_ = { let a0_ = F32(c.m.r32(fp.wrapping_add(32))); let a1_ = F32(c.m.r32(((sym::gRefVu0).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(12))); F::vu0_b__f_mul(c, a0_, a1_) }; F::vu0_b__f_sub(c, a0_, a1_) }; let a_ = fp; c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = { let a0_ = F32(c.m.r32((fp).wrapping_add(4))); let a1_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))); F::vu0_b__f_add(c, a0_, a1_) }; let a1_ = { let a0_ = F32(c.m.r32((fp.wrapping_add(32)).wrapping_add(4))); let a1_ = F32(c.m.r32(((sym::gRefVu0).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(12))); F::vu0_b__f_mul(c, a0_, a1_) }; F::vu0_b__f_sub(c, a0_, a1_) }; let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = { let a0_ = F32(c.m.r32((fp).wrapping_add(8))); let a1_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8))); F::vu0_b__f_add(c, a0_, a1_) }; let a1_ = { let a0_ = F32(c.m.r32((fp.wrapping_add(32)).wrapping_add(8))); let a1_ = F32(c.m.r32(((sym::gRefVu0).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(12))); F::vu0_b__f_mul(c, a0_, a1_) }; F::vu0_b__f_sub(c, a0_, a1_) }; let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let s_ = fp; let d_ = p_out_1; rt::memcpy(c, d_, s_, 16); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Vec3_AddSub", &PROBE_N, &[], true); }
    r_
}

pub fn Vec3_SubAdd(c: &mut Ctx, mut p_out_1: u32, mut p_a_2: u32, mut p_b_3: u32, mut p_c_4: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vec3_SubAdd", &PROBE_N, &[(p_out_1) as u32, (p_a_2) as u32, (p_b_3) as u32, (p_c_4) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let r_ = 'f: {
        { let s_ = p_a_2; rt::memcpy(c, fp, s_, 16); }
        { let s_ = p_b_3; rt::memcpy(c, fp.wrapping_add(16), s_, 16); }
        { let s_ = p_c_4; rt::memcpy(c, fp.wrapping_add(32), s_, 16); }
        { let v_ = { let a0_ = { let a0_ = F32(c.m.r32(fp)); let a1_ = F32(c.m.r32(fp.wrapping_add(16))); F::vu0_b__f_sub(c, a0_, a1_) }; let a1_ = { let a0_ = F32(c.m.r32(fp.wrapping_add(32))); let a1_ = F32(c.m.r32(((sym::gRefVu0).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(12))); F::vu0_b__f_mul(c, a0_, a1_) }; F::vu0_b__f_add(c, a0_, a1_) }; let a_ = fp; c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = { let a0_ = F32(c.m.r32((fp).wrapping_add(4))); let a1_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))); F::vu0_b__f_sub(c, a0_, a1_) }; let a1_ = { let a0_ = F32(c.m.r32((fp.wrapping_add(32)).wrapping_add(4))); let a1_ = F32(c.m.r32(((sym::gRefVu0).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(12))); F::vu0_b__f_mul(c, a0_, a1_) }; F::vu0_b__f_add(c, a0_, a1_) }; let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = { let a0_ = F32(c.m.r32((fp).wrapping_add(8))); let a1_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8))); F::vu0_b__f_sub(c, a0_, a1_) }; let a1_ = { let a0_ = F32(c.m.r32((fp.wrapping_add(32)).wrapping_add(8))); let a1_ = F32(c.m.r32(((sym::gRefVu0).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(12))); F::vu0_b__f_mul(c, a0_, a1_) }; F::vu0_b__f_add(c, a0_, a1_) }; let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let s_ = fp; let d_ = p_out_1; rt::memcpy(c, d_, s_, 16); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Vec3_SubAdd", &PROBE_N, &[], true); }
    r_
}

pub fn Vec3_ScaleAdd(c: &mut Ctx, mut p_out_1: u32, mut p_dir_2: u32, mut p_base_3: u32, mut p_s_4: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vec3_ScaleAdd", &PROBE_N, &[(p_out_1) as u32, (p_dir_2) as u32, (p_base_3) as u32, (p_s_4).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let r_ = 'f: {
        { let s_ = p_dir_2; rt::memcpy(c, fp, s_, 16); }
        { let s_ = p_base_3; rt::memcpy(c, fp.wrapping_add(16), s_, 16); }
        { let v_ = { let a0_ = { let a0_ = F32(c.m.r32(fp)); F::vu0_b__f_mul(c, a0_, p_s_4) }; let a1_ = { let a0_ = F32(c.m.r32(fp.wrapping_add(16))); let a1_ = F32(c.m.r32(((sym::gRefVu0).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(12))); F::vu0_b__f_mul(c, a0_, a1_) }; F::vu0_b__f_add(c, a0_, a1_) }; let a_ = fp.wrapping_add(16); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = { let a0_ = F32(c.m.r32((fp).wrapping_add(4))); F::vu0_b__f_mul(c, a0_, p_s_4) }; let a1_ = { let a0_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))); let a1_ = F32(c.m.r32(((sym::gRefVu0).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(12))); F::vu0_b__f_mul(c, a0_, a1_) }; F::vu0_b__f_add(c, a0_, a1_) }; let a_ = (fp.wrapping_add(16)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = { let a0_ = F32(c.m.r32((fp).wrapping_add(8))); F::vu0_b__f_mul(c, a0_, p_s_4) }; let a1_ = { let a0_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8))); let a1_ = F32(c.m.r32(((sym::gRefVu0).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(12))); F::vu0_b__f_mul(c, a0_, a1_) }; F::vu0_b__f_add(c, a0_, a1_) }; let a_ = (fp.wrapping_add(16)).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let s_ = fp.wrapping_add(16); let d_ = p_out_1; rt::memcpy(c, d_, s_, 16); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Vec3_ScaleAdd", &PROBE_N, &[], true); }
    r_
}

pub fn Vec3_ScaleSub(c: &mut Ctx, mut p_out_1: u32, mut p_dir_2: u32, mut p_base_3: u32, mut p_s_4: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vec3_ScaleSub", &PROBE_N, &[(p_out_1) as u32, (p_dir_2) as u32, (p_base_3) as u32, (p_s_4).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let r_ = 'f: {
        { let s_ = p_dir_2; rt::memcpy(c, fp, s_, 16); }
        { let s_ = p_base_3; rt::memcpy(c, fp.wrapping_add(16), s_, 16); }
        { let v_ = { let a0_ = { let a0_ = F32(c.m.r32(fp)); F::vu0_b__f_mul(c, a0_, p_s_4) }; let a1_ = { let a0_ = F32(c.m.r32(fp.wrapping_add(16))); let a1_ = F32(c.m.r32(((sym::gRefVu0).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(12))); F::vu0_b__f_mul(c, a0_, a1_) }; F::vu0_b__f_sub(c, a0_, a1_) }; let a_ = fp.wrapping_add(16); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = { let a0_ = F32(c.m.r32((fp).wrapping_add(4))); F::vu0_b__f_mul(c, a0_, p_s_4) }; let a1_ = { let a0_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))); let a1_ = F32(c.m.r32(((sym::gRefVu0).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(12))); F::vu0_b__f_mul(c, a0_, a1_) }; F::vu0_b__f_sub(c, a0_, a1_) }; let a_ = (fp.wrapping_add(16)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = { let a0_ = F32(c.m.r32((fp).wrapping_add(8))); F::vu0_b__f_mul(c, a0_, p_s_4) }; let a1_ = { let a0_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8))); let a1_ = F32(c.m.r32(((sym::gRefVu0).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(12))); F::vu0_b__f_mul(c, a0_, a1_) }; F::vu0_b__f_sub(c, a0_, a1_) }; let a_ = (fp.wrapping_add(16)).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let s_ = fp.wrapping_add(16); let d_ = p_out_1; rt::memcpy(c, d_, s_, 16); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Vec3_ScaleSub", &PROBE_N, &[], true); }
    r_
}

pub fn Vec3_Add4(c: &mut Ctx, mut p_out_1: u32, mut p_a_2: u32, mut p_b_3: u32, mut p_c_4: u32, mut p_d_5: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vec3_Add4", &PROBE_N, &[(p_out_1) as u32, (p_a_2) as u32, (p_b_3) as u32, (p_c_4) as u32, (p_d_5) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(64) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 64);
    let r_ = 'f: {
        { let s_ = p_a_2; rt::memcpy(c, fp, s_, 16); }
        { let s_ = p_b_3; rt::memcpy(c, fp.wrapping_add(16), s_, 16); }
        { let s_ = p_c_4; rt::memcpy(c, fp.wrapping_add(32), s_, 16); }
        { let s_ = p_d_5; rt::memcpy(c, fp.wrapping_add(48), s_, 16); }
        { let v_ = { let a0_ = { let a0_ = { let a0_ = F32(c.m.r32(fp)); let a1_ = F32(c.m.r32(fp.wrapping_add(16))); F::vu0_b__f_add(c, a0_, a1_) }; let a1_ = { let a0_ = F32(c.m.r32(fp.wrapping_add(32))); let a1_ = F32(c.m.r32(((sym::gRefVu0).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(12))); F::vu0_b__f_mul(c, a0_, a1_) }; F::vu0_b__f_add(c, a0_, a1_) }; let a1_ = { let a0_ = F32(c.m.r32(fp.wrapping_add(48))); let a1_ = F32(c.m.r32(((sym::gRefVu0).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(12))); F::vu0_b__f_mul(c, a0_, a1_) }; F::vu0_b__f_add(c, a0_, a1_) }; let a_ = fp; c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = { let a0_ = { let a0_ = F32(c.m.r32((fp).wrapping_add(4))); let a1_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))); F::vu0_b__f_add(c, a0_, a1_) }; let a1_ = { let a0_ = F32(c.m.r32((fp.wrapping_add(32)).wrapping_add(4))); let a1_ = F32(c.m.r32(((sym::gRefVu0).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(12))); F::vu0_b__f_mul(c, a0_, a1_) }; F::vu0_b__f_add(c, a0_, a1_) }; let a1_ = { let a0_ = F32(c.m.r32((fp.wrapping_add(48)).wrapping_add(4))); let a1_ = F32(c.m.r32(((sym::gRefVu0).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(12))); F::vu0_b__f_mul(c, a0_, a1_) }; F::vu0_b__f_add(c, a0_, a1_) }; let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = { let a0_ = { let a0_ = F32(c.m.r32((fp).wrapping_add(8))); let a1_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8))); F::vu0_b__f_add(c, a0_, a1_) }; let a1_ = { let a0_ = F32(c.m.r32((fp.wrapping_add(32)).wrapping_add(8))); let a1_ = F32(c.m.r32(((sym::gRefVu0).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(12))); F::vu0_b__f_mul(c, a0_, a1_) }; F::vu0_b__f_add(c, a0_, a1_) }; let a1_ = { let a0_ = F32(c.m.r32((fp.wrapping_add(48)).wrapping_add(8))); let a1_ = F32(c.m.r32(((sym::gRefVu0).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(12))); F::vu0_b__f_mul(c, a0_, a1_) }; F::vu0_b__f_add(c, a0_, a1_) }; let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let s_ = fp; let d_ = p_out_1; rt::memcpy(c, d_, s_, 16); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Vec3_Add4", &PROBE_N, &[], true); }
    r_
}

pub fn Vec4_AddClamp(c: &mut Ctx, mut p_out_1: u32, mut p_a_2: u32, mut p_b_3: u32, mut p_lo_4: F32, mut p_hi_5: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vec4_AddClamp", &PROBE_N, &[(p_out_1) as u32, (p_a_2) as u32, (p_b_3) as u32, (p_lo_4).0, (p_hi_5).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let r_ = 'f: {
        { let s_ = p_a_2; rt::memcpy(c, fp, s_, 16); }
        { let s_ = p_b_3; rt::memcpy(c, fp.wrapping_add(16), s_, 16); }
        { let v_ = { let a0_ = { let a0_ = { let a0_ = F32(c.m.r32(fp)); let a1_ = F32(c.m.r32(fp.wrapping_add(16))); F::vu0_b__f_add(c, a0_, a1_) }; F::vu0_b__f_maxf(c, a0_, p_lo_4) }; F::vu0_b__f_minf(c, a0_, p_hi_5) }; let a_ = fp; c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = { let a0_ = { let a0_ = F32(c.m.r32((fp).wrapping_add(4))); let a1_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))); F::vu0_b__f_add(c, a0_, a1_) }; F::vu0_b__f_maxf(c, a0_, p_lo_4) }; F::vu0_b__f_minf(c, a0_, p_hi_5) }; let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = { let a0_ = { let a0_ = F32(c.m.r32((fp).wrapping_add(8))); let a1_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8))); F::vu0_b__f_add(c, a0_, a1_) }; F::vu0_b__f_maxf(c, a0_, p_lo_4) }; F::vu0_b__f_minf(c, a0_, p_hi_5) }; let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = { let a0_ = { let a0_ = F32(c.m.r32((fp).wrapping_add(12))); let a1_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(12))); F::vu0_b__f_add(c, a0_, a1_) }; F::vu0_b__f_maxf(c, a0_, p_lo_4) }; F::vu0_b__f_minf(c, a0_, p_hi_5) }; let a_ = (fp).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let s_ = fp; let d_ = p_out_1; rt::memcpy(c, d_, s_, 16); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Vec4_AddClamp", &PROBE_N, &[], true); }
    r_
}

pub fn Vec3_AddClamp(c: &mut Ctx, mut p_out_1: u32, mut p_a_2: u32, mut p_b_3: u32, mut p_lo_4: F32, mut p_hi_5: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vec3_AddClamp", &PROBE_N, &[(p_out_1) as u32, (p_a_2) as u32, (p_b_3) as u32, (p_lo_4).0, (p_hi_5).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let r_ = 'f: {
        { let s_ = p_a_2; rt::memcpy(c, fp, s_, 16); }
        { let s_ = p_b_3; rt::memcpy(c, fp.wrapping_add(16), s_, 16); }
        { let v_ = { let a0_ = { let a0_ = { let a0_ = F32(c.m.r32(fp)); let a1_ = F32(c.m.r32(fp.wrapping_add(16))); F::vu0_b__f_add(c, a0_, a1_) }; F::vu0_b__f_maxf(c, a0_, p_lo_4) }; F::vu0_b__f_minf(c, a0_, p_hi_5) }; let a_ = fp; c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = { let a0_ = { let a0_ = F32(c.m.r32((fp).wrapping_add(4))); let a1_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(4))); F::vu0_b__f_add(c, a0_, a1_) }; F::vu0_b__f_maxf(c, a0_, p_lo_4) }; F::vu0_b__f_minf(c, a0_, p_hi_5) }; let a_ = (fp).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = { let a0_ = { let a0_ = F32(c.m.r32((fp).wrapping_add(8))); let a1_ = F32(c.m.r32((fp.wrapping_add(16)).wrapping_add(8))); F::vu0_b__f_add(c, a0_, a1_) }; F::vu0_b__f_maxf(c, a0_, p_lo_4) }; F::vu0_b__f_minf(c, a0_, p_hi_5) }; let a_ = (fp).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let s_ = fp; let d_ = p_out_1; rt::memcpy(c, d_, s_, 16); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Vec3_AddClamp", &PROBE_N, &[], true); }
    r_
}

pub fn Vec3_RotateAxis(c: &mut Ctx, mut p_out_1: u32, mut p_v_2: u32, mut p_axis_3: u32, mut p_angle_4: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vec3_RotateAxis", &PROBE_N, &[(p_out_1) as u32, (p_v_2) as u32, (p_axis_3) as u32, (p_angle_4).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let mut l_cs_5: F32 = F32(0);
    let mut l_sn_6: F32 = F32(0);
    let r_ = 'f: {
        l_cs_5 = F::Mathf_Cos(c, p_angle_4);
        l_sn_6 = F::Mathf_Sin(c, p_angle_4);
        F::Vec4_Scale(c, fp, p_v_2, l_cs_5);
        F::Vec3_Cross(c, fp.wrapping_add(16), p_axis_3, p_v_2);
        F::Vec4_Scale(c, fp.wrapping_add(16), fp.wrapping_add(16), l_sn_6);
        { let a2_ = { let a0_ = F::Vec3_Dot(c, p_axis_3, p_v_2); let a1_ = F::vu0_b__f_sub(c, F32(0x3f800000), l_cs_5); F::vu0_b__f_mul(c, a0_, a1_) }; F::Vec4_Scale(c, fp.wrapping_add(32), p_axis_3, a2_) };
        F::Vec4_Add(c, p_out_1, fp, fp.wrapping_add(16));
        F::Vec4_Add(c, p_out_1, p_out_1, fp.wrapping_add(32));
        { let v_ = F32(c.m.r32((p_v_2).wrapping_add(12))); let a_ = (p_out_1).wrapping_add(12); c.m.w32(a_, (v_).0); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Vec3_RotateAxis", &PROBE_N, &[], true); }
    r_
}

pub fn Vec3_RotateX(c: &mut Ctx, mut p_out_1: u32, mut p_v_2: u32, mut p_angle_3: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vec3_RotateX", &PROBE_N, &[(p_out_1) as u32, (p_v_2) as u32, (p_angle_3).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let mut l_cs_4: F32 = F32(0);
    let mut l_sn_5: F32 = F32(0);
    let r_ = 'f: {
        l_cs_4 = F::Mathf_Cos(c, p_angle_3);
        l_sn_5 = F::Mathf_Sin(c, p_angle_3);
        F::Vec4_Scale(c, fp, p_v_2, l_cs_4);
        { let v_ = F32(0x00000000); let a_ = fp.wrapping_add(16); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = rt::fneg(F32(c.m.r32((p_v_2).wrapping_add(8)))); F::vu0_b__f_mul(c, a0_, l_sn_5) }; let a_ = (fp.wrapping_add(16)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32((p_v_2).wrapping_add(4))); F::vu0_b__f_mul(c, a0_, l_sn_5) }; let a_ = (fp.wrapping_add(16)).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (fp.wrapping_add(16)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32(p_v_2)); let a1_ = F::vu0_b__f_sub(c, F32(0x3f800000), l_cs_4); F::vu0_b__f_mul(c, a0_, a1_) }; let a_ = fp.wrapping_add(32); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (fp.wrapping_add(32)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (fp.wrapping_add(32)).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (fp.wrapping_add(32)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        F::Vec4_Add(c, p_out_1, fp, fp.wrapping_add(16));
        F::Vec4_Add(c, p_out_1, p_out_1, fp.wrapping_add(32));
        { let v_ = F32(c.m.r32((p_v_2).wrapping_add(12))); let a_ = (p_out_1).wrapping_add(12); c.m.w32(a_, (v_).0); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Vec3_RotateX", &PROBE_N, &[], true); }
    r_
}

pub fn Vec3_RotateY(c: &mut Ctx, mut p_out_1: u32, mut p_v_2: u32, mut p_angle_3: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vec3_RotateY", &PROBE_N, &[(p_out_1) as u32, (p_v_2) as u32, (p_angle_3).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let mut l_cs_4: F32 = F32(0);
    let mut l_sn_5: F32 = F32(0);
    let r_ = 'f: {
        l_cs_4 = F::Mathf_Cos(c, p_angle_3);
        l_sn_5 = F::Mathf_Sin(c, p_angle_3);
        F::Vec4_Scale(c, fp, p_v_2, l_cs_4);
        { let v_ = { let a0_ = F32(c.m.r32((p_v_2).wrapping_add(8))); F::vu0_b__f_mul(c, a0_, l_sn_5) }; let a_ = fp.wrapping_add(16); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (fp.wrapping_add(16)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = rt::fneg(F32(c.m.r32(p_v_2))); F::vu0_b__f_mul(c, a0_, l_sn_5) }; let a_ = (fp.wrapping_add(16)).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (fp.wrapping_add(16)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = fp.wrapping_add(32); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = F32(c.m.r32((p_v_2).wrapping_add(4))); let a1_ = F::vu0_b__f_sub(c, F32(0x3f800000), l_cs_4); F::vu0_b__f_mul(c, a0_, a1_) }; let a_ = (fp.wrapping_add(32)).wrapping_add(4); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (fp.wrapping_add(32)).wrapping_add(8); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (fp.wrapping_add(32)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        F::Vec4_Add(c, p_out_1, fp, fp.wrapping_add(16));
        F::Vec4_Add(c, p_out_1, p_out_1, fp.wrapping_add(32));
        { let v_ = F32(c.m.r32((p_v_2).wrapping_add(12))); let a_ = (p_out_1).wrapping_add(12); c.m.w32(a_, (v_).0); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Vec3_RotateY", &PROBE_N, &[], true); }
    r_
}

pub fn Vu0R_Init(c: &mut Ctx, mut p_r_1: u32, mut p_seed_2: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vu0R_Init", &PROBE_N, &[(p_r_1) as u32, (p_seed_2).0], false); }
    let r_ = 'f: {
    { let v_ = ((1065353216u32) | (((F::vu0_b__f_bits(c, p_seed_2)) & (8388607u32)))); let a_ = p_r_1; c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Vu0R_Init", &PROBE_N, &[], true); }
    r_
}

pub fn Vu0R_Next(c: &mut Ctx, mut p_r_1: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Vu0R_Next", &PROBE_N, &[(p_r_1) as u32], false); }
    let mut l_b_2: u32 = 0;
    let r_ = 'f: {
    l_b_2 = (((((c.m.r32(p_r_1)).wrapping_shr((4i32) as u32)) ^ ((c.m.r32(p_r_1)).wrapping_shr((22i32) as u32)))) & (1u32));
    { let v_ = ((1065353216u32) | ((((((c.m.r32(p_r_1)).wrapping_shl((1i32) as u32)) | (l_b_2))) & (8388607u32)))); let a_ = p_r_1; c.m.w32(a_, v_); };
    break 'f { let a0_ = c.m.r32(p_r_1); F::vu0_b__f_from(c, a0_) };
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "Vu0R_Next", &PROBE_N, &[], true); }
    r_
}

pub fn Rand_Float01_Model(c: &mut Ctx, mut p_r_1: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Rand_Float01_Model", &PROBE_N, &[(p_r_1) as u32], false); }
    let mut l_f_2: F32 = F32(0);
    let mut l_i_3: i32 = 0;
    let r_ = 'f: {
    l_f_2 = F32(0x00000000);
    {
        l_i_3 = 0i32;
        'l1: loop {
            if !(((((l_i_3) < (7i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                l_f_2 = F::Vu0R_Next(c, p_r_1);
            }
            { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
        }
    }
    F::Vu0R_Init(c, p_r_1, l_f_2);
    {
        l_i_3 = 0i32;
        'l3: loop {
            if !(((((l_i_3) < (7i32)) as i32)) != 0) { break 'l3; }
            'c4: {
                l_f_2 = F::Vu0R_Next(c, p_r_1);
            }
            { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
        }
    }
    break 'f { let a1_ = F32(c.m.r32(((sym::gRefVu0).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(12))); F::vu0_b__f_sub(c, l_f_2, a1_) };
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "Rand_Float01_Model", &PROBE_N, &[], true); }
    r_
}
