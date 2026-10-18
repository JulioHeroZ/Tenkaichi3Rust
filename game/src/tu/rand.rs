#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn Rand_Seed(c: &mut Ctx, mut p_seed_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Rand_Seed", &PROBE_N, &[(p_seed_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_seed_1; let a_ = (sym::gRandState).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    {
        { let v_ = 1i32; let a_ = sym::gRandIndex; c.m.w32(a_, (v_) as u32); };
        'l1: loop {
            if !((((((c.m.r32(sym::gRandIndex) as i32)) < (624i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let v_ = ((((1812433253u64).wrapping_mul((((({ let a_ = (sym::gRandState).wrapping_add((((((c.m.r32(sym::gRandIndex) as i32)).wrapping_sub(1i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) }) ^ (({ let a_ = (sym::gRandState).wrapping_add((((((c.m.r32(sym::gRandIndex) as i32)).wrapping_sub(1i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((30i32) as u32)))) as u64))).wrapping_add((((c.m.r32(sym::gRandIndex) as i32)) as u64))) as u32); let a_ = (sym::gRandState).wrapping_add(((((c.m.r32(sym::gRandIndex) as i32)) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
            }
            { let a_ = sym::gRandIndex; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Rand_Seed", &PROBE_N, &[], true); }
    r_
}

pub fn Rand_SeedByArray(c: &mut Ctx, mut p_key_1: u32, mut p_keyLen_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Rand_SeedByArray", &PROBE_N, &[(p_key_1) as u32, (p_keyLen_2) as u32], false); }
    let mut l_i_3: i32 = 0;
    let mut l_j_4: i32 = 0;
    let mut l_k_5: i32 = 0;
    let r_ = 'f: {
    F::Rand_Seed(c, ((19650218i32) as u32));
    l_i_3 = 1i32;
    l_j_4 = 0i32;
    l_k_5 = (if (((((624i32) > (p_keyLen_2)) as i32)) != 0) { 624i32 } else { p_keyLen_2 });
    {
        'l1: loop {
            if !((l_k_5) != 0) { break 'l1; }
            'c2: {
                { let v_ = ((((((((c.m.r32((sym::gRandState).wrapping_add((((l_i_3) as u32)).wrapping_mul(4)))) as u64)) ^ ((((((c.m.r32((sym::gRandState).wrapping_add(((((l_i_3).wrapping_sub(1i32)) as u32)).wrapping_mul(4)))) ^ ((c.m.r32((sym::gRandState).wrapping_add(((((l_i_3).wrapping_sub(1i32)) as u32)).wrapping_mul(4)))).wrapping_shr((30i32) as u32)))) as u64)).wrapping_mul(1664525u64)))).wrapping_add(((c.m.r32((p_key_1).wrapping_add((((l_j_4) as u32)).wrapping_mul(4)))) as u64))).wrapping_add(((l_j_4) as u64))) as u32); let a_ = (sym::gRandState).wrapping_add((((l_i_3) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
                { let o_ = l_j_4; let n_ = (o_).wrapping_add(1); l_j_4 = n_; };
                if (((((l_i_3) >= (624i32)) as i32)) != 0) {
                    { let v_ = c.m.r32((sym::gRandState).wrapping_add(((((624i32).wrapping_sub(1i32)) as u32)).wrapping_mul(4))); let a_ = (sym::gRandState).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    l_i_3 = 1i32;
                }
                if (((((l_j_4) >= (p_keyLen_2)) as i32)) != 0) {
                    l_j_4 = 0i32;
                }
            }
            { let o_ = l_k_5; let n_ = (o_).wrapping_sub(1); l_k_5 = n_; };
        }
    }
    {
        l_k_5 = (624i32).wrapping_sub(1i32);
        'l3: loop {
            if !((l_k_5) != 0) { break 'l3; }
            'c4: {
                { let v_ = (((((((c.m.r32((sym::gRandState).wrapping_add((((l_i_3) as u32)).wrapping_mul(4)))) as u64)) ^ ((((((c.m.r32((sym::gRandState).wrapping_add(((((l_i_3).wrapping_sub(1i32)) as u32)).wrapping_mul(4)))) ^ ((c.m.r32((sym::gRandState).wrapping_add(((((l_i_3).wrapping_sub(1i32)) as u32)).wrapping_mul(4)))).wrapping_shr((30i32) as u32)))) as u64)).wrapping_mul(1566083941u64)))).wrapping_sub(((l_i_3) as u64))) as u32); let a_ = (sym::gRandState).wrapping_add((((l_i_3) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
                if (((((l_i_3) >= (624i32)) as i32)) != 0) {
                    { let v_ = c.m.r32((sym::gRandState).wrapping_add(((((624i32).wrapping_sub(1i32)) as u32)).wrapping_mul(4))); let a_ = (sym::gRandState).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    l_i_3 = 1i32;
                }
            }
            { let o_ = l_k_5; let n_ = (o_).wrapping_sub(1); l_k_5 = n_; };
        }
    }
    { let v_ = 2147483648u32; let a_ = (sym::gRandState).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Rand_SeedByArray", &PROBE_N, &[], true); }
    r_
}

pub fn Rand_Next(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Rand_Next", &PROBE_N, &[], false); }
    let mut l_y_1: u32 = 0;
    let mut l_kk_2: i32 = 0;
    let r_ = 'f: {
    if ((((((c.m.r32(sym::gRandIndex) as i32)) >= (624i32)) as i32)) != 0) {
        if ((((((c.m.r32(sym::gRandIndex) as i32)) == ((624i32).wrapping_add(1i32))) as i32)) != 0) {
            F::Rand_Seed(c, ((5489i32) as u32));
        }
        {
            l_kk_2 = 0i32;
            'l1: loop {
                if !(((((l_kk_2) < ((624i32).wrapping_sub(397i32))) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_y_1 = ((((((((c.m.r32((sym::gRandState).wrapping_add((((l_kk_2) as u32)).wrapping_mul(4)))) as u64)) & (2147483648u64))) | (((((c.m.r32((sym::gRandState).wrapping_add(((((l_kk_2).wrapping_add(1i32)) as u32)).wrapping_mul(4)))) as u64)) & (2147483647u64))))) as u32);
                    { let v_ = ((((c.m.r32((sym::gRandState).wrapping_add(((((l_kk_2).wrapping_add(397i32)) as u32)).wrapping_mul(4)))) ^ ((l_y_1).wrapping_shr((1i32) as u32)))) ^ (c.m.r32((sym::gRandMag01).wrapping_add((((l_y_1) & (((1i32) as u32)))).wrapping_mul(4))))); let a_ = (sym::gRandState).wrapping_add((((l_kk_2) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                }
                { let o_ = l_kk_2; let n_ = (o_).wrapping_add(1); l_kk_2 = n_; };
            }
        }
        l_y_1 = ((((((((c.m.r32((sym::gRandState).wrapping_add(((((624i32).wrapping_sub(1i32)) as u32)).wrapping_mul(4)))) as u64)) & (2147483648u64))) | (((((c.m.r32((sym::gRandState).wrapping_add((((0i32) as u32)).wrapping_mul(4)))) as u64)) & (2147483647u64))))) as u32);
        { let v_ = ((((c.m.r32((sym::gRandState).wrapping_add(((((397i32).wrapping_sub(1i32)) as u32)).wrapping_mul(4)))) ^ ((l_y_1).wrapping_shr((1i32) as u32)))) ^ (c.m.r32((sym::gRandMag01).wrapping_add((((l_y_1) & (((1i32) as u32)))).wrapping_mul(4))))); let a_ = (sym::gRandState).wrapping_add(((((624i32).wrapping_sub(1i32)) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = 0i32; let a_ = sym::gRandIndex; c.m.w32(a_, (v_) as u32); };
    }
    l_y_1 = { let a_ = (sym::gRandState).wrapping_add(((({ let a_ = sym::gRandIndex; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); o_ }) as u32)).wrapping_mul(4)); c.m.r32(a_) };
    { let r_ = (l_y_1).wrapping_shr((11i32) as u32); let n_ = ((l_y_1) ^ (r_)); l_y_1 = n_; };
    { let r_ = (((l_y_1).wrapping_shl((7i32) as u32)) & (2636928640u32)); let n_ = ((l_y_1) ^ (r_)); l_y_1 = n_; };
    { let r_ = (((l_y_1).wrapping_shl((15i32) as u32)) & (4022730752u32)); let n_ = ((l_y_1) ^ (r_)); l_y_1 = n_; };
    { let r_ = (l_y_1).wrapping_shr((18i32) as u32); let n_ = ((l_y_1) ^ (r_)); l_y_1 = n_; };
    break 'f l_y_1;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Rand_Next", &PROBE_N, &[], true); }
    r_
}

pub fn Rand_Init(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Rand_Init", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_i_1: i32 = 0;
    let r_ = 'f: {
        { let _ = F::memset(c, fp, 0i32, 16u32); };
        { let a0_ = ((F::func_002BB390(c)) as u32); F::Port_Srand(c, a0_) };
        {
            l_i_1 = 0i32;
            'l1: loop {
                if !(((((l_i_1) < (4i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let v_ = ((F::Port_Rand(c)) as u32); let a_ = (fp).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                }
                { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
            }
        }
        F::Rand_SeedByArray(c, fp, 4i32);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Rand_Init", &PROBE_N, &[], true); }
    r_
}

pub fn Rand_Range(c: &mut Ctx, mut p_n_1: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Rand_Range", &PROBE_N, &[(p_n_1) as u32], false); }
    let r_ = 'f: {
    if (((((p_n_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f ((0i32) as u32);
    }
    break 'f rt::rem_u32(F::Rand_Next(c), p_n_1);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Rand_Range", &PROBE_N, &[], true); }
    r_
}
