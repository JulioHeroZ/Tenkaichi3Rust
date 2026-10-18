#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn mathf_pc__bits(c: &mut Ctx, p_f_1: F32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "mathf_pc__bits", &PROBE_N, &[(p_f_1).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    c.m.w32(fp, (p_f_1).0);
    let r_ = 'f: {
        { let _ = F::memcpy(c, fp.wrapping_add(4), fp, ((4i32) as u32)); };
        break 'f c.m.r32(fp.wrapping_add(4));
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "mathf_pc__bits", &PROBE_N, &[], true); }
    r_
}

pub fn mathf_pc__from(c: &mut Ctx, p_u_1: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "mathf_pc__from", &PROBE_N, &[(p_u_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    c.m.w32(fp, p_u_1);
    let r_ = 'f: {
        { let _ = F::memcpy(c, fp.wrapping_add(4), fp, ((4i32) as u32)); };
        break 'f F32(c.m.r32(fp.wrapping_add(4)));
       F32(0)
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "mathf_pc__from", &PROBE_N, &[], true); }
    r_
}

pub fn Mathf_WrapAngle(c: &mut Ctx, mut p_angle_1: F32, mut p_half_2: F32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Mathf_WrapAngle", &PROBE_N, &[(p_angle_1).0, (p_half_2).0], false); }
    let mut l_a_3: u32 = 0;
    let mut l_h_4: u32 = 0;
    let mut l_range_5: u32 = 0;
    let r_ = 'f: {
    l_a_3 = F::mathf_pc__bits(c, p_angle_1);
    l_h_4 = F::mathf_pc__bits(c, p_half_2);
    l_range_5 = F::RefVu0_AddBits(c, l_h_4, l_h_4);
    'l1: while ((F::RefVu0_LtBits(c, l_a_3, l_range_5)) != 0) {
        l_a_3 = F::RefVu0_AddBits(c, l_a_3, l_range_5);
    }
    'l2: while ((F::RefVu0_LtBits(c, l_h_4, l_a_3)) != 0) {
        l_a_3 = F::RefVu0_SubBits(c, l_a_3, l_range_5);
    }
    break 'f F::mathf_pc__from(c, l_a_3);
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "Mathf_WrapAngle", &PROBE_N, &[], true); }
    r_
}

pub fn Mathf_SinFast(c: &mut Ctx, mut p_angle_1: F32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Mathf_SinFast", &PROBE_N, &[(p_angle_1).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_x_2: u32 = 0;
    let mut l_x2_3: u32 = 0;
    let mut l_r_4: u32 = 0;
    let mut l_i_5: i32 = 0;
    let r_ = 'f: {
        l_x_2 = { let a0_ = F::Mathf_WrapAngle(c, p_angle_1, F32(0x40490fda)); F::mathf_pc__bits(c, a0_) };
        l_x2_3 = F::RefVu0_MulBits(c, l_x_2, l_x_2);
        l_r_4 = l_x_2;
        {
            l_i_5 = 0i32;
            'l1: loop {
                if !(((((l_i_5) < (4i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let v_ = { let a0_ = { let a0_ = F32(c.m.r32((sym::gMathfSinCoef).wrapping_add((((l_i_5) as u32)).wrapping_mul(4)))); F::mathf_pc__bits(c, a0_) }; F::RefVu0_MulBits(c, a0_, l_x_2) }; let a_ = (fp).wrapping_add((((l_i_5) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = { let a0_ = c.m.r32((fp).wrapping_add((((l_i_5) as u32)).wrapping_mul(4))); F::RefVu0_MulBits(c, a0_, l_x2_3) }; let a_ = (fp).wrapping_add((((l_i_5) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                }
                { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
            }
        }
        {
            l_i_5 = 0i32;
            'l3: loop {
                if !(((((l_i_5) < (3i32)) as i32)) != 0) { break 'l3; }
                'c4: {
                    { let v_ = { let a0_ = c.m.r32((fp).wrapping_add((((l_i_5) as u32)).wrapping_mul(4))); F::RefVu0_MulBits(c, a0_, l_x2_3) }; let a_ = (fp).wrapping_add((((l_i_5) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                }
                { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
            }
        }
        l_r_4 = { let a1_ = c.m.r32((fp).wrapping_add((((3i32) as u32)).wrapping_mul(4))); F::RefVu0_AddBits(c, l_r_4, a1_) };
        {
            l_i_5 = 0i32;
            'l5: loop {
                if !(((((l_i_5) < (2i32)) as i32)) != 0) { break 'l5; }
                'c6: {
                    { let v_ = { let a0_ = c.m.r32((fp).wrapping_add((((l_i_5) as u32)).wrapping_mul(4))); F::RefVu0_MulBits(c, a0_, l_x2_3) }; let a_ = (fp).wrapping_add((((l_i_5) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                }
                { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
            }
        }
        l_r_4 = { let a1_ = c.m.r32((fp).wrapping_add((((2i32) as u32)).wrapping_mul(4))); F::RefVu0_AddBits(c, l_r_4, a1_) };
        { let v_ = { let a0_ = c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4))); F::RefVu0_MulBits(c, a0_, l_x2_3) }; let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        l_r_4 = { let a1_ = c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4))); F::RefVu0_AddBits(c, l_r_4, a1_) };
        l_r_4 = { let a1_ = c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4))); F::RefVu0_AddBits(c, l_r_4, a1_) };
        break 'f F::mathf_pc__from(c, l_r_4);
       F32(0)
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Mathf_SinFast", &PROBE_N, &[], true); }
    r_
}

pub fn Mathf_Sqrt(c: &mut Ctx, mut p_x_1: F32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Mathf_Sqrt", &PROBE_N, &[(p_x_1).0], false); }
    let r_ = 'f: {
    break 'f { let a0_ = { let a0_ = F::mathf_pc__bits(c, p_x_1); F::RefVu0_SqrtBits(c, a0_) }; F::mathf_pc__from(c, a0_) };
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "Mathf_Sqrt", &PROBE_N, &[], true); }
    r_
}

pub fn Rand_SeedFloat(c: &mut Ctx, mut p_seed_1: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Rand_SeedFloat", &PROBE_N, &[(p_seed_1).0], false); }
    let mut l_scaled_2: F32 = F32(0);
    let r_ = 'f: {
    l_scaled_2 = { let a0_ = { let a0_ = F::mathf_pc__bits(c, p_seed_1); let a1_ = F::mathf_pc__bits(c, F32(0x4b189680)); F::RefVu0_MulBits(c, a0_, a1_) }; F::mathf_pc__from(c, a0_) };
    { let v_ = ((1065353216u32) | (((F::mathf_pc__bits(c, p_seed_1)) & (8388607u32)))); let a_ = sym::gPortVu0R; c.m.w32(a_, v_); };
    F::Port_Srand(c, (if (((rt::fge(l_scaled_2, F32(0x4f800000)) as i32)) != 0) { 4294967295u32 } else { (if (((rt::fle(l_scaled_2, F32(0x00000000)) as i32)) != 0) { 0u32 } else { rt::f2u(l_scaled_2) }) }));
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Rand_SeedFloat", &PROBE_N, &[], true); }
    r_
}

pub fn mathf_pc__rnext(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "mathf_pc__rnext", &PROBE_N, &[], false); }
    let mut l_b_1: u32 = 0;
    let r_ = 'f: {
    l_b_1 = (((((c.m.r32(sym::gPortVu0R)).wrapping_shr((4i32) as u32)) ^ ((c.m.r32(sym::gPortVu0R)).wrapping_shr((22i32) as u32)))) & (1u32));
    { let v_ = ((1065353216u32) | ((((((c.m.r32(sym::gPortVu0R)).wrapping_shl((1i32) as u32)) | (l_b_1))) & (8388607u32)))); let a_ = sym::gPortVu0R; c.m.w32(a_, v_); };
    break 'f c.m.r32(sym::gPortVu0R);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "mathf_pc__rnext", &PROBE_N, &[], true); }
    r_
}

pub fn Rand_Float01(c: &mut Ctx) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Rand_Float01", &PROBE_N, &[], false); }
    let mut l_f_1: u32 = 0;
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
    l_f_1 = ((0i32) as u32);
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < (7i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                l_f_1 = F::mathf_pc__rnext(c);
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
    { let v_ = ((1065353216u32) | (((l_f_1) & (8388607u32)))); let a_ = sym::gPortVu0R; c.m.w32(a_, v_); };
    {
        l_i_2 = 0i32;
        'l3: loop {
            if !(((((l_i_2) < (7i32)) as i32)) != 0) { break 'l3; }
            'c4: {
                l_f_1 = F::mathf_pc__rnext(c);
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
    break 'f { let a0_ = F::RefVu0_SubBits(c, l_f_1, 1065353216u32); F::mathf_pc__from(c, a0_) };
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "Rand_Float01", &PROBE_N, &[], true); }
    r_
}
