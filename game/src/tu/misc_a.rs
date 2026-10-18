#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn OldPass_MtSeed(c: &mut Ctx, mut p_seed_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "OldPass_MtSeed", &PROBE_N, &[(p_seed_1) as u32], false); }
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
    { let v_ = p_seed_1; let a_ = (sym::gOldPassMtState).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    {
        l_i_2 = 1i32;
        'l1: loop {
            if !(((((l_i_2) < (624i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let v_ = ((((1812433253i32) as u32)).wrapping_mul(((c.m.r32((sym::gOldPassMtState).wrapping_add(((((l_i_2).wrapping_sub(1i32)) as u32)).wrapping_mul(4)))) ^ ((c.m.r32((sym::gOldPassMtState).wrapping_add(((((l_i_2).wrapping_sub(1i32)) as u32)).wrapping_mul(4)))).wrapping_shr((30i32) as u32))))).wrapping_add(((l_i_2) as u32)); let a_ = (sym::gOldPassMtState).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
    { let v_ = 1i32; let a_ = sym::gOldPassMtIndex; c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "OldPass_MtSeed", &PROBE_N, &[], true); }
    r_
}

pub fn OldPass_MtSeedByArray(c: &mut Ctx, mut p_key_1: u32, mut p_keyLen_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "OldPass_MtSeedByArray", &PROBE_N, &[(p_key_1) as u32, (p_keyLen_2) as u32], false); }
    let mut l_i_3: i32 = 0;
    let mut l_j_4: i32 = 0;
    let mut l_k_5: i32 = 0;
    let r_ = 'f: {
    l_i_3 = 1i32;
    l_j_4 = 0i32;
    l_k_5 = (if (((((624i32) > (p_keyLen_2)) as i32)) != 0) { 624i32 } else { p_keyLen_2 });
    F::OldPass_MtSeed(c, ((19650218i32) as u32));
    {
        'l1: loop {
            if !((l_k_5) != 0) { break 'l1; }
            'c2: {
                { let v_ = ((((c.m.r32((sym::gOldPassMtState).wrapping_add((((l_i_3) as u32)).wrapping_mul(4)))) ^ ((((c.m.r32((sym::gOldPassMtState).wrapping_add(((((l_i_3).wrapping_sub(1i32)) as u32)).wrapping_mul(4)))) ^ ((c.m.r32((sym::gOldPassMtState).wrapping_add(((((l_i_3).wrapping_sub(1i32)) as u32)).wrapping_mul(4)))).wrapping_shr((30i32) as u32)))).wrapping_mul(((1664525i32) as u32))))).wrapping_add(c.m.r32((p_key_1).wrapping_add((((l_j_4) as u32)).wrapping_mul(4))))).wrapping_add(((l_j_4) as u32)); let a_ = (sym::gOldPassMtState).wrapping_add((((l_i_3) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
                { let o_ = l_j_4; let n_ = (o_).wrapping_add(1); l_j_4 = n_; };
                if (((((l_i_3) >= (624i32)) as i32)) != 0) {
                    { let v_ = c.m.r32((sym::gOldPassMtState).wrapping_add(((((624i32).wrapping_sub(1i32)) as u32)).wrapping_mul(4))); let a_ = (sym::gOldPassMtState).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
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
                { let v_ = (((((((c.m.r32((sym::gOldPassMtState).wrapping_add((((l_i_3) as u32)).wrapping_mul(4)))) as u64)) ^ ((((((c.m.r32((sym::gOldPassMtState).wrapping_add(((((l_i_3).wrapping_sub(1i32)) as u32)).wrapping_mul(4)))) ^ ((c.m.r32((sym::gOldPassMtState).wrapping_add(((((l_i_3).wrapping_sub(1i32)) as u32)).wrapping_mul(4)))).wrapping_shr((30i32) as u32)))) as u64)).wrapping_mul(1566083941u64)))).wrapping_sub(((l_i_3) as u64))) as u32); let a_ = (sym::gOldPassMtState).wrapping_add((((l_i_3) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
                if (((((l_i_3) >= (624i32)) as i32)) != 0) {
                    { let v_ = c.m.r32((sym::gOldPassMtState).wrapping_add(((((624i32).wrapping_sub(1i32)) as u32)).wrapping_mul(4))); let a_ = (sym::gOldPassMtState).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    l_i_3 = 1i32;
                }
            }
            { let o_ = l_k_5; let n_ = (o_).wrapping_sub(1); l_k_5 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "OldPass_MtSeedByArray", &PROBE_N, &[], true); }
    r_
}

pub fn OldPass_MtNext(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "OldPass_MtNext", &PROBE_N, &[], false); }
    let mut l_y_1: u32 = 0;
    let r_ = 'f: {
    l_y_1 = { let a_ = (sym::gOldPassMtState).wrapping_add(((({ let a_ = sym::gOldPassMtIndex; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); o_ }) as u32)).wrapping_mul(4)); c.m.r32(a_) };
    { let r_ = (l_y_1).wrapping_shr((11i32) as u32); let n_ = ((l_y_1) ^ (r_)); l_y_1 = n_; };
    { let r_ = (((l_y_1).wrapping_shl((7i32) as u32)) & (2636928640u32)); let n_ = ((l_y_1) ^ (r_)); l_y_1 = n_; };
    { let r_ = (((l_y_1).wrapping_shl((15i32) as u32)) & (4022730752u32)); let n_ = ((l_y_1) ^ (r_)); l_y_1 = n_; };
    { let r_ = (l_y_1).wrapping_shr((18i32) as u32); let n_ = ((l_y_1) ^ (r_)); l_y_1 = n_; };
    break 'f l_y_1;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "OldPass_MtNext", &PROBE_N, &[], true); }
    r_
}

pub fn OldPass_ClearBits(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "OldPass_ClearBits", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let r_ = 'f: {
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (32i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let v_ = ((0i32) as u8); let a_ = (sym::gOldPassBits).wrapping_add(((l_i_1) as u32)); c.m.w8(a_, v_); };
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "OldPass_ClearBits", &PROBE_N, &[], true); }
    r_
}

pub fn OldPass_PutBits(c: &mut Ctx, mut p_value_1: u32, mut p_pos_2: i32, mut p_count_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "OldPass_PutBits", &PROBE_N, &[(p_value_1) as u32, (p_pos_2) as u32, (p_count_3) as u32], false); }
    let mut l_i_4: i32 = 0;
    let mut l_bit_5: i32 = 0;
    let mut l_p_6: i32 = 0;
    let r_ = 'f: {
    {
        l_i_4 = 0i32;
        'l1: loop {
            if !(((((l_i_4) < (p_count_3)) as i32)) != 0) { break 'l1; }
            'c2: {
                l_bit_5 = (((((p_value_1).wrapping_shr((l_i_4) as u32)) & (((1i32) as u32)))) as i32);
                l_p_6 = (l_i_4).wrapping_add(p_pos_2);
                if ((l_bit_5) != 0) {
                    { let r_ = (1i32).wrapping_shl((rt::rem_i32(l_p_6, 8i32)) as u32); let a_ = (sym::gOldPassBits).wrapping_add(((rt::div_i32(l_p_6, 8i32)) as u32)); let n_ = ((((((c.m.r8(a_)) as i32)) | (r_))) as u8); c.m.w8(a_, n_); };
                } else {
                    { let r_ = (!((1i32).wrapping_shl((rt::rem_i32(l_p_6, 8i32)) as u32))); let a_ = (sym::gOldPassBits).wrapping_add(((rt::div_i32(l_p_6, 8i32)) as u32)); let n_ = ((((((c.m.r8(a_)) as i32)) & (r_))) as u8); c.m.w8(a_, n_); };
                }
            }
            { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "OldPass_PutBits", &PROBE_N, &[], true); }
    r_
}

pub fn OldPass_GetBit(c: &mut Ctx, mut p_pos_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "OldPass_GetBit", &PROBE_N, &[(p_pos_1) as u32], false); }
    let r_ = 'f: {
    break 'f (((((c.m.r8((sym::gOldPassBits).wrapping_add(((rt::div_i32(p_pos_1, 8i32)) as u32)))) as i32)).wrapping_shr((rt::rem_i32(p_pos_1, 8i32)) as u32)) & (1i32));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "OldPass_GetBit", &PROBE_N, &[], true); }
    r_
}

pub fn OldPass_GetBits(c: &mut Ctx, mut p_pos_1: i32, mut p_count_2: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "OldPass_GetBits", &PROBE_N, &[(p_pos_1) as u32, (p_count_2) as u32], false); }
    let mut l_value_3: u32 = 0;
    let mut l_i_4: i32 = 0;
    let r_ = 'f: {
    l_value_3 = ((0i32) as u32);
    {
        l_i_4 = 0i32;
        'l1: loop {
            if !(((((l_i_4) < (p_count_2)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let r_ = (((F::OldPass_GetBit(c, (l_i_4).wrapping_add(p_pos_1))).wrapping_shl((l_i_4) as u32)) as u32); let n_ = ((l_value_3) | (r_)); l_value_3 = n_; };
            }
            { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
        }
    }
    break 'f l_value_3;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "OldPass_GetBits", &PROBE_N, &[], true); }
    r_
}

pub fn misc_a__OldPass_Div(c: &mut Ctx, mut p_a_1: i32, mut p_b_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "misc_a__OldPass_Div", &PROBE_N, &[(p_a_1) as u32, (p_b_2) as u32], false); }
    let r_ = 'f: {
    break 'f rt::div_i32(p_a_1, p_b_2);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "misc_a__OldPass_Div", &PROBE_N, &[], true); }
    r_
}

pub fn OldPass_BitsToText(c: &mut Ctx, mut p_bitCount_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "OldPass_BitsToText", &PROBE_N, &[(p_bitCount_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(256) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 256);
    let mut l_n_2: i32 = 0;
    let mut l_i_3: i32 = 0;
    let mut l_value_4: u32 = 0;
    let r_ = 'f: {
        rt::memset(c, fp, 0, 256);
        { let v_ = sym::misc_a__str0; c.m.w32(fp.wrapping_add(0), v_); }
        { let v_ = sym::misc_a__str1; c.m.w32(fp.wrapping_add(4), v_); }
        { let v_ = sym::misc_a__str2; c.m.w32(fp.wrapping_add(8), v_); }
        { let v_ = sym::misc_a__str3; c.m.w32(fp.wrapping_add(12), v_); }
        { let v_ = sym::misc_a__str4; c.m.w32(fp.wrapping_add(16), v_); }
        { let v_ = sym::misc_a__str5; c.m.w32(fp.wrapping_add(20), v_); }
        { let v_ = sym::misc_a__str6; c.m.w32(fp.wrapping_add(24), v_); }
        { let v_ = sym::misc_a__str7; c.m.w32(fp.wrapping_add(28), v_); }
        { let v_ = sym::misc_a__str8; c.m.w32(fp.wrapping_add(32), v_); }
        { let v_ = sym::misc_a__str9; c.m.w32(fp.wrapping_add(36), v_); }
        { let v_ = sym::misc_a__str10; c.m.w32(fp.wrapping_add(40), v_); }
        { let v_ = sym::misc_a__str11; c.m.w32(fp.wrapping_add(44), v_); }
        { let v_ = sym::misc_a__str12; c.m.w32(fp.wrapping_add(48), v_); }
        { let v_ = sym::misc_a__str13; c.m.w32(fp.wrapping_add(52), v_); }
        { let v_ = sym::misc_a__str14; c.m.w32(fp.wrapping_add(56), v_); }
        { let v_ = sym::misc_a__str15; c.m.w32(fp.wrapping_add(60), v_); }
        { let v_ = sym::misc_a__str16; c.m.w32(fp.wrapping_add(64), v_); }
        { let v_ = sym::misc_a__str17; c.m.w32(fp.wrapping_add(68), v_); }
        { let v_ = sym::misc_a__str18; c.m.w32(fp.wrapping_add(72), v_); }
        { let v_ = sym::misc_a__str19; c.m.w32(fp.wrapping_add(76), v_); }
        { let v_ = sym::misc_a__str20; c.m.w32(fp.wrapping_add(80), v_); }
        { let v_ = sym::misc_a__str21; c.m.w32(fp.wrapping_add(84), v_); }
        { let v_ = sym::misc_a__str22; c.m.w32(fp.wrapping_add(88), v_); }
        { let v_ = sym::misc_a__str23; c.m.w32(fp.wrapping_add(92), v_); }
        { let v_ = sym::misc_a__str24; c.m.w32(fp.wrapping_add(96), v_); }
        { let v_ = sym::misc_a__str25; c.m.w32(fp.wrapping_add(100), v_); }
        { let v_ = sym::misc_a__str26; c.m.w32(fp.wrapping_add(104), v_); }
        { let v_ = sym::misc_a__str27; c.m.w32(fp.wrapping_add(108), v_); }
        { let v_ = sym::misc_a__str28; c.m.w32(fp.wrapping_add(112), v_); }
        { let v_ = sym::misc_a__str29; c.m.w32(fp.wrapping_add(116), v_); }
        { let v_ = sym::misc_a__str30; c.m.w32(fp.wrapping_add(120), v_); }
        { let v_ = sym::misc_a__str31; c.m.w32(fp.wrapping_add(124), v_); }
        { let v_ = sym::misc_a__str32; c.m.w32(fp.wrapping_add(128), v_); }
        { let v_ = sym::misc_a__str33; c.m.w32(fp.wrapping_add(132), v_); }
        { let v_ = sym::misc_a__str34; c.m.w32(fp.wrapping_add(136), v_); }
        { let v_ = sym::misc_a__str35; c.m.w32(fp.wrapping_add(140), v_); }
        { let v_ = sym::misc_a__str36; c.m.w32(fp.wrapping_add(144), v_); }
        { let v_ = sym::misc_a__str37; c.m.w32(fp.wrapping_add(148), v_); }
        { let v_ = sym::misc_a__str38; c.m.w32(fp.wrapping_add(152), v_); }
        { let v_ = sym::misc_a__str39; c.m.w32(fp.wrapping_add(156), v_); }
        { let v_ = sym::misc_a__str40; c.m.w32(fp.wrapping_add(160), v_); }
        { let v_ = sym::misc_a__str41; c.m.w32(fp.wrapping_add(164), v_); }
        { let v_ = sym::misc_a__str42; c.m.w32(fp.wrapping_add(168), v_); }
        { let v_ = sym::misc_a__str43; c.m.w32(fp.wrapping_add(172), v_); }
        { let v_ = sym::misc_a__str44; c.m.w32(fp.wrapping_add(176), v_); }
        { let v_ = sym::misc_a__str45; c.m.w32(fp.wrapping_add(180), v_); }
        { let v_ = sym::misc_a__str46; c.m.w32(fp.wrapping_add(184), v_); }
        { let v_ = sym::misc_a__str47; c.m.w32(fp.wrapping_add(188), v_); }
        { let v_ = sym::misc_a__str48; c.m.w32(fp.wrapping_add(192), v_); }
        { let v_ = sym::misc_a__str49; c.m.w32(fp.wrapping_add(196), v_); }
        { let v_ = sym::misc_a__str50; c.m.w32(fp.wrapping_add(200), v_); }
        { let v_ = sym::misc_a__str51; c.m.w32(fp.wrapping_add(204), v_); }
        { let v_ = sym::misc_a__str52; c.m.w32(fp.wrapping_add(208), v_); }
        { let v_ = sym::misc_a__str53; c.m.w32(fp.wrapping_add(212), v_); }
        { let v_ = sym::misc_a__str54; c.m.w32(fp.wrapping_add(216), v_); }
        { let v_ = sym::misc_a__str55; c.m.w32(fp.wrapping_add(220), v_); }
        { let v_ = sym::misc_a__str56; c.m.w32(fp.wrapping_add(224), v_); }
        { let v_ = sym::misc_a__str57; c.m.w32(fp.wrapping_add(228), v_); }
        { let v_ = sym::misc_a__str58; c.m.w32(fp.wrapping_add(232), v_); }
        { let v_ = sym::misc_a__str59; c.m.w32(fp.wrapping_add(236), v_); }
        { let v_ = sym::misc_a__str60; c.m.w32(fp.wrapping_add(240), v_); }
        { let v_ = sym::misc_a__str61; c.m.w32(fp.wrapping_add(244), v_); }
        { let v_ = sym::misc_a__str62; c.m.w32(fp.wrapping_add(248), v_); }
        { let v_ = sym::misc_a__str63; c.m.w32(fp.wrapping_add(252), v_); }
        {
            l_n_2 = 0i32;
            'l1: loop {
                if !(((((l_n_2) < (F::misc_a__OldPass_Div(c, p_bitCount_1, 6i32))) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_value_4 = ((0i32) as u32);
                    {
                        l_i_3 = 0i32;
                        'l3: loop {
                            if !(((((l_i_3) < (6i32)) as i32)) != 0) { break 'l3; }
                            'c4: {
                                { let r_ = (((F::OldPass_GetBit(c, ((l_n_2).wrapping_mul(6i32)).wrapping_add(l_i_3))).wrapping_shl((l_i_3) as u32)) as u32); let n_ = ((l_value_4) | (r_)); l_value_4 = n_; };
                            }
                            { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
                        }
                    }
                    { let v_ = { let a_ = c.m.r32((fp).wrapping_add((l_value_4).wrapping_mul(4))); (c.m.r8(a_) as i8) }; let a_ = (sym::gOldPassText).wrapping_add(((l_n_2) as u32)); c.m.w8(a_, (v_) as u8); };
                    { let v_ = ((0i32) as i8); let a_ = (sym::gOldPassText).wrapping_add((((l_n_2).wrapping_add(1i32)) as u32)); c.m.w8(a_, (v_) as u8); };
                }
                { let o_ = l_n_2; let n_ = (o_).wrapping_add(1); l_n_2 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "OldPass_BitsToText", &PROBE_N, &[], true); }
    r_
}

pub fn OldPass_TextToBits(c: &mut Ctx, mut p_text_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "OldPass_TextToBits", &PROBE_N, &[(p_text_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(256) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 256);
    let mut l_p_2: u32 = 0;
    let mut l_pos_3: i32 = 0;
    let mut l_i_4: i32 = 0;
    let r_ = 'f: {
        rt::memset(c, fp, 0, 256);
        { let v_ = sym::misc_a__str64; c.m.w32(fp.wrapping_add(0), v_); }
        { let v_ = sym::misc_a__str65; c.m.w32(fp.wrapping_add(4), v_); }
        { let v_ = sym::misc_a__str66; c.m.w32(fp.wrapping_add(8), v_); }
        { let v_ = sym::misc_a__str67; c.m.w32(fp.wrapping_add(12), v_); }
        { let v_ = sym::misc_a__str68; c.m.w32(fp.wrapping_add(16), v_); }
        { let v_ = sym::misc_a__str69; c.m.w32(fp.wrapping_add(20), v_); }
        { let v_ = sym::misc_a__str70; c.m.w32(fp.wrapping_add(24), v_); }
        { let v_ = sym::misc_a__str71; c.m.w32(fp.wrapping_add(28), v_); }
        { let v_ = sym::misc_a__str72; c.m.w32(fp.wrapping_add(32), v_); }
        { let v_ = sym::misc_a__str73; c.m.w32(fp.wrapping_add(36), v_); }
        { let v_ = sym::misc_a__str74; c.m.w32(fp.wrapping_add(40), v_); }
        { let v_ = sym::misc_a__str75; c.m.w32(fp.wrapping_add(44), v_); }
        { let v_ = sym::misc_a__str76; c.m.w32(fp.wrapping_add(48), v_); }
        { let v_ = sym::misc_a__str77; c.m.w32(fp.wrapping_add(52), v_); }
        { let v_ = sym::misc_a__str78; c.m.w32(fp.wrapping_add(56), v_); }
        { let v_ = sym::misc_a__str79; c.m.w32(fp.wrapping_add(60), v_); }
        { let v_ = sym::misc_a__str80; c.m.w32(fp.wrapping_add(64), v_); }
        { let v_ = sym::misc_a__str81; c.m.w32(fp.wrapping_add(68), v_); }
        { let v_ = sym::misc_a__str82; c.m.w32(fp.wrapping_add(72), v_); }
        { let v_ = sym::misc_a__str83; c.m.w32(fp.wrapping_add(76), v_); }
        { let v_ = sym::misc_a__str84; c.m.w32(fp.wrapping_add(80), v_); }
        { let v_ = sym::misc_a__str85; c.m.w32(fp.wrapping_add(84), v_); }
        { let v_ = sym::misc_a__str86; c.m.w32(fp.wrapping_add(88), v_); }
        { let v_ = sym::misc_a__str87; c.m.w32(fp.wrapping_add(92), v_); }
        { let v_ = sym::misc_a__str88; c.m.w32(fp.wrapping_add(96), v_); }
        { let v_ = sym::misc_a__str89; c.m.w32(fp.wrapping_add(100), v_); }
        { let v_ = sym::misc_a__str90; c.m.w32(fp.wrapping_add(104), v_); }
        { let v_ = sym::misc_a__str91; c.m.w32(fp.wrapping_add(108), v_); }
        { let v_ = sym::misc_a__str92; c.m.w32(fp.wrapping_add(112), v_); }
        { let v_ = sym::misc_a__str93; c.m.w32(fp.wrapping_add(116), v_); }
        { let v_ = sym::misc_a__str94; c.m.w32(fp.wrapping_add(120), v_); }
        { let v_ = sym::misc_a__str95; c.m.w32(fp.wrapping_add(124), v_); }
        { let v_ = sym::misc_a__str96; c.m.w32(fp.wrapping_add(128), v_); }
        { let v_ = sym::misc_a__str97; c.m.w32(fp.wrapping_add(132), v_); }
        { let v_ = sym::misc_a__str98; c.m.w32(fp.wrapping_add(136), v_); }
        { let v_ = sym::misc_a__str99; c.m.w32(fp.wrapping_add(140), v_); }
        { let v_ = sym::misc_a__str100; c.m.w32(fp.wrapping_add(144), v_); }
        { let v_ = sym::misc_a__str101; c.m.w32(fp.wrapping_add(148), v_); }
        { let v_ = sym::misc_a__str102; c.m.w32(fp.wrapping_add(152), v_); }
        { let v_ = sym::misc_a__str103; c.m.w32(fp.wrapping_add(156), v_); }
        { let v_ = sym::misc_a__str104; c.m.w32(fp.wrapping_add(160), v_); }
        { let v_ = sym::misc_a__str105; c.m.w32(fp.wrapping_add(164), v_); }
        { let v_ = sym::misc_a__str106; c.m.w32(fp.wrapping_add(168), v_); }
        { let v_ = sym::misc_a__str107; c.m.w32(fp.wrapping_add(172), v_); }
        { let v_ = sym::misc_a__str108; c.m.w32(fp.wrapping_add(176), v_); }
        { let v_ = sym::misc_a__str109; c.m.w32(fp.wrapping_add(180), v_); }
        { let v_ = sym::misc_a__str110; c.m.w32(fp.wrapping_add(184), v_); }
        { let v_ = sym::misc_a__str111; c.m.w32(fp.wrapping_add(188), v_); }
        { let v_ = sym::misc_a__str112; c.m.w32(fp.wrapping_add(192), v_); }
        { let v_ = sym::misc_a__str113; c.m.w32(fp.wrapping_add(196), v_); }
        { let v_ = sym::misc_a__str114; c.m.w32(fp.wrapping_add(200), v_); }
        { let v_ = sym::misc_a__str115; c.m.w32(fp.wrapping_add(204), v_); }
        { let v_ = sym::misc_a__str116; c.m.w32(fp.wrapping_add(208), v_); }
        { let v_ = sym::misc_a__str117; c.m.w32(fp.wrapping_add(212), v_); }
        { let v_ = sym::misc_a__str118; c.m.w32(fp.wrapping_add(216), v_); }
        { let v_ = sym::misc_a__str119; c.m.w32(fp.wrapping_add(220), v_); }
        { let v_ = sym::misc_a__str120; c.m.w32(fp.wrapping_add(224), v_); }
        { let v_ = sym::misc_a__str121; c.m.w32(fp.wrapping_add(228), v_); }
        { let v_ = sym::misc_a__str122; c.m.w32(fp.wrapping_add(232), v_); }
        { let v_ = sym::misc_a__str123; c.m.w32(fp.wrapping_add(236), v_); }
        { let v_ = sym::misc_a__str124; c.m.w32(fp.wrapping_add(240), v_); }
        { let v_ = sym::misc_a__str125; c.m.w32(fp.wrapping_add(244), v_); }
        { let v_ = sym::misc_a__str126; c.m.w32(fp.wrapping_add(248), v_); }
        { let v_ = sym::misc_a__str127; c.m.w32(fp.wrapping_add(252), v_); }
        l_p_2 = p_text_1;
        l_pos_3 = 0i32;
        'l1: while ((((((((c.m.r8(l_p_2) as i8)) as i32)) != (0i32)) as i32)) != 0) {
            {
                l_i_4 = 0i32;
                'l2: loop {
                    if !(((((l_i_4) < (64i32)) as i32)) != 0) { break 'l2; }
                    'c3: {
                        if ((((((((c.m.r8(l_p_2) as i8)) as i32)) == ((({ let a_ = c.m.r32((fp).wrapping_add((((l_i_4) as u32)).wrapping_mul(4))); (c.m.r8(a_) as i8) }) as i32))) as i32)) != 0) {
                            F::OldPass_PutBits(c, ((l_i_4) as u32), l_pos_3, 6i32);
                            { let r_ = 6i32; let n_ = (l_pos_3).wrapping_add(r_); l_pos_3 = n_; };
                            break 'l2;
                        }
                    }
                    { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
                }
            }
            { let o_ = l_p_2; let n_ = (o_).wrapping_add(1); l_p_2 = n_; };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "OldPass_TextToBits", &PROBE_N, &[], true); }
    r_
}

pub fn OldPass_Nop(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "OldPass_Nop", &PROBE_N, &[], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "OldPass_Nop", &PROBE_N, &[], true); }
    r_
}

pub fn misc_a__OldPass_SetKey(c: &mut Ctx, mut p_seed_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "misc_a__OldPass_SetKey", &PROBE_N, &[(p_seed_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        rt::memset(c, fp, 0, 16);
        { let v_ = p_seed_1; c.m.w32(fp.wrapping_add(0), v_); }
        { let v_ = (p_seed_1).wrapping_mul(((2i32) as u32)); c.m.w32(fp.wrapping_add(4), v_); }
        { let v_ = (p_seed_1).wrapping_mul(((3i32) as u32)); c.m.w32(fp.wrapping_add(8), v_); }
        { let v_ = (p_seed_1).wrapping_mul(((4i32) as u32)); c.m.w32(fp.wrapping_add(12), v_); }
        F::OldPass_MtSeedByArray(c, fp, 4i32);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "misc_a__OldPass_SetKey", &PROBE_N, &[], true); }
    r_
}

pub fn OldPass_DecodeChar(c: &mut Ctx, mut p_out_1: u32, mut p_text_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "OldPass_DecodeChar", &PROBE_N, &[(p_out_1) as u32, (p_text_2) as u32], false); }
    let mut l_i_3: i32 = 0;
    let mut l_sum_4: i32 = 0;
    let mut l_seed_5: u32 = 0;
    let mut l_n_6: i32 = 0;
    let r_ = 'f: {
    if (((((F::strlen(c, p_text_2)) != (((32i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    F::OldPass_TextToBits(c, p_text_2);
    l_seed_5 = F::OldPass_GetBits(c, 0i32, 32i32);
    F::misc_a__OldPass_SetKey(c, l_seed_5);
    {
        l_i_3 = 4i32;
        'l1: loop {
            if !(((((l_i_3) < (24i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let r_ = F::OldPass_MtNext(c); let a_ = (sym::gOldPassBits).wrapping_add(((l_i_3) as u32)); let n_ = ((((((c.m.r8(a_)) as u32)) ^ (r_))) as u8); c.m.w8(a_, n_); };
            }
            { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
        }
    }
    l_sum_4 = 0i32;
    {
        l_i_3 = 0i32;
        'l3: loop {
            if !(((((l_i_3) < (119i32)) as i32)) != 0) { break 'l3; }
            'c4: {
                { let r_ = F::OldPass_GetBit(c, l_i_3); let n_ = (l_sum_4).wrapping_add(r_); l_sum_4 = n_; };
            }
            { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
        }
    }
    if (((((((l_sum_4) as u32)) != (F::OldPass_GetBits(c, 119i32, 8i32))) as i32)) != 0) {
        break 'f 0i32;
    }
    {
        l_i_3 = 0i32;
        'l5: loop {
            if !(((((l_i_3) < (8i32)) as i32)) != 0) { break 'l5; }
            'c6: {
                if (((((((c.m.r8((sym::gOldPassBits).wrapping_add((((l_i_3).wrapping_add(16i32)) as u32)))) as i32)) != (((((c.m.r8((sym::gOldPassBits).wrapping_add(((l_i_3) as u32)))) as i32)) ^ (((c.m.r8((sym::gOldPassBits).wrapping_add((((l_i_3).wrapping_add(8i32)) as u32)))) as i32))))) as i32)) != 0) {
                    break 'f 0i32;
                }
            }
            { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
        }
    }
    { let v_ = ((F::OldPass_GetBits(c, 32i32, 8i32)) as i32); let a_ = p_out_1; c.m.w32(a_, (v_) as u32); };
    {
        l_n_6 = 0i32;
        'l7: loop {
            if !(((((l_n_6) < (7i32)) as i32)) != 0) { break 'l7; }
            'c8: {
                { let v_ = ((F::OldPass_GetBits(c, (40i32).wrapping_add((l_n_6).wrapping_mul(10i32)), 10i32)) as i32); let a_ = ((p_out_1).wrapping_add(4)).wrapping_add((((l_n_6) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            }
            { let o_ = l_n_6; let n_ = (o_).wrapping_add(1); l_n_6 = n_; };
        }
    }
    { let v_ = ((F::OldPass_GetBits(c, 110i32, 3i32)) as i32); let a_ = (p_out_1).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
    { let v_ = ((F::OldPass_GetBits(c, 113i32, 1i32)) as i32); let a_ = (p_out_1).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
    { let v_ = ((F::OldPass_GetBits(c, 114i32, 4i32)) as i32); let a_ = (p_out_1).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
    { let v_ = ((F::OldPass_GetBits(c, 118i32, 1i32)) as i32); let a_ = (p_out_1).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "OldPass_DecodeChar", &PROBE_N, &[], true); }
    r_
}

pub fn OldPass_DecodeShort(c: &mut Ctx, mut p_out_1: u32, mut p_text_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "OldPass_DecodeShort", &PROBE_N, &[(p_out_1) as u32, (p_text_2) as u32], false); }
    let mut l_i_3: i32 = 0;
    let mut l_sum_4: i32 = 0;
    let mut l_seed_5: u32 = 0;
    let r_ = 'f: {
    if (((((F::strlen(c, p_text_2)) != (((20i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    F::OldPass_TextToBits(c, p_text_2);
    l_seed_5 = F::OldPass_GetBits(c, 0i32, 32i32);
    F::misc_a__OldPass_SetKey(c, l_seed_5);
    {
        l_i_3 = 4i32;
        'l1: loop {
            if !(((((l_i_3) < (15i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let r_ = F::OldPass_MtNext(c); let a_ = (sym::gOldPassBits).wrapping_add(((l_i_3) as u32)); let n_ = ((((((c.m.r8(a_)) as u32)) ^ (r_))) as u8); c.m.w8(a_, n_); };
            }
            { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
        }
    }
    l_sum_4 = 0i32;
    {
        l_i_3 = 0i32;
        'l3: loop {
            if !(((((l_i_3) < (68i32)) as i32)) != 0) { break 'l3; }
            'c4: {
                { let r_ = F::OldPass_GetBit(c, l_i_3); let n_ = (l_sum_4).wrapping_add(r_); l_sum_4 = n_; };
            }
            { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
        }
    }
    if (((((((l_sum_4) as u32)) != (F::OldPass_GetBits(c, 68i32, 8i32))) as i32)) != 0) {
        break 'f 0i32;
    }
    {
        l_i_3 = 0i32;
        'l5: loop {
            if !(((((l_i_3) < (5i32)) as i32)) != 0) { break 'l5; }
            'c6: {
                if (((((((c.m.r8((sym::gOldPassBits).wrapping_add((((l_i_3).wrapping_add(10i32)) as u32)))) as i32)) != (((((c.m.r8((sym::gOldPassBits).wrapping_add(((l_i_3) as u32)))) as i32)) ^ (((c.m.r8((sym::gOldPassBits).wrapping_add((((l_i_3).wrapping_add(5i32)) as u32)))) as i32))))) as i32)) != 0) {
                    break 'f 0i32;
                }
            }
            { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
        }
    }
    { let v_ = ((F::OldPass_GetBits(c, 32i32, 8i32)) as i32); let a_ = p_out_1; c.m.w32(a_, (v_) as u32); };
    { let v_ = 0i32; let a_ = (p_out_1).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
    { let v_ = (((F::OldPass_GetBits(c, 40i32, 20i32)).wrapping_mul(((100i32) as u32))) as i32); let a_ = (p_out_1).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
    { let v_ = ((F::OldPass_GetBits(c, 60i32, 2i32)) as i32); let a_ = (p_out_1).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
    { let v_ = ((F::OldPass_GetBits(c, 62i32, 1i32)) as i32); let a_ = (p_out_1).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
    { let v_ = ((F::OldPass_GetBits(c, 63i32, 4i32)) as i32); let a_ = (p_out_1).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
    { let v_ = ((F::OldPass_GetBits(c, 67i32, 1i32)) as i32); let a_ = (p_out_1).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "OldPass_DecodeShort", &PROBE_N, &[], true); }
    r_
}

pub fn OldPass_EncodeChar(c: &mut Ctx, mut p_in_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "OldPass_EncodeChar", &PROBE_N, &[(p_in_1) as u32], false); }
    let mut l_i_2: i32 = 0;
    let mut l_sum_3: i32 = 0;
    let mut l_seed_4: u32 = 0;
    let mut l_hi_5: i32 = 0;
    let mut l_lo_6: i32 = 0;
    let mut l_n_7: i32 = 0;
    let r_ = 'f: {
    l_hi_5 = F::Port_Rand(c);
    l_lo_6 = F::Port_Rand(c);
    l_seed_4 = (((((l_hi_5).wrapping_shl((16i32) as u32)) | (((l_lo_6) & (65535i32))))) as u32);
    F::misc_a__OldPass_SetKey(c, l_seed_4);
    F::OldPass_ClearBits(c);
    F::OldPass_PutBits(c, l_seed_4, 0i32, 32i32);
    { let a0_ = (((c.m.r32(p_in_1) as i32)) as u32); F::OldPass_PutBits(c, a0_, 32i32, 8i32) };
    {
        l_n_7 = 0i32;
        'l1: loop {
            if !(((((l_n_7) < (7i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let a0_ = (((c.m.r32(((p_in_1).wrapping_add(4)).wrapping_add((((l_n_7) as u32)).wrapping_mul(4))) as i32)) as u32); F::OldPass_PutBits(c, a0_, (40i32).wrapping_add((l_n_7).wrapping_mul(10i32)), 10i32) };
            }
            { let o_ = l_n_7; let n_ = (o_).wrapping_add(1); l_n_7 = n_; };
        }
    }
    { let a0_ = (((c.m.r32((p_in_1).wrapping_add(32)) as i32)) as u32); F::OldPass_PutBits(c, a0_, 110i32, 3i32) };
    { let a0_ = (((c.m.r32((p_in_1).wrapping_add(36)) as i32)) as u32); F::OldPass_PutBits(c, a0_, 113i32, 1i32) };
    { let a0_ = (((c.m.r32((p_in_1).wrapping_add(40)) as i32)) as u32); F::OldPass_PutBits(c, a0_, 114i32, 4i32) };
    { let a0_ = (((c.m.r32((p_in_1).wrapping_add(44)) as i32)) as u32); F::OldPass_PutBits(c, a0_, 118i32, 1i32) };
    l_sum_3 = 0i32;
    {
        l_i_2 = 0i32;
        'l3: loop {
            if !(((((l_i_2) < (128i32)) as i32)) != 0) { break 'l3; }
            'c4: {
                { let r_ = F::OldPass_GetBit(c, l_i_2); let n_ = (l_sum_3).wrapping_add(r_); l_sum_3 = n_; };
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
    F::OldPass_PutBits(c, ((l_sum_3) as u32), 119i32, 8i32);
    {
        l_i_2 = 0i32;
        'l5: loop {
            if !(((((l_i_2) < (8i32)) as i32)) != 0) { break 'l5; }
            'c6: {
                { let v_ = ((((((c.m.r8((sym::gOldPassBits).wrapping_add(((l_i_2) as u32)))) as i32)) ^ (((c.m.r8((sym::gOldPassBits).wrapping_add((((l_i_2).wrapping_add(8i32)) as u32)))) as i32)))) as u8); let a_ = (sym::gOldPassBits).wrapping_add((((l_i_2).wrapping_add(16i32)) as u32)); c.m.w8(a_, v_); };
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
    {
        l_i_2 = 4i32;
        'l7: loop {
            if !(((((l_i_2) < (24i32)) as i32)) != 0) { break 'l7; }
            'c8: {
                { let r_ = F::OldPass_MtNext(c); let a_ = (sym::gOldPassBits).wrapping_add(((l_i_2) as u32)); let n_ = ((((((c.m.r8(a_)) as u32)) ^ (r_))) as u8); c.m.w8(a_, n_); };
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
    F::OldPass_BitsToText(c, 192i32);
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "OldPass_EncodeChar", &PROBE_N, &[], true); }
    r_
}

pub fn OldPass_EncodeShort(c: &mut Ctx, mut p_in_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "OldPass_EncodeShort", &PROBE_N, &[(p_in_1) as u32], false); }
    let mut l_i_2: i32 = 0;
    let mut l_sum_3: i32 = 0;
    let mut l_seed_4: u32 = 0;
    let mut l_hi_5: i32 = 0;
    let mut l_lo_6: i32 = 0;
    let mut l_n_7: i32 = 0;
    let r_ = 'f: {
    l_hi_5 = F::Port_Rand(c);
    l_lo_6 = F::Port_Rand(c);
    l_seed_4 = (((((l_hi_5).wrapping_shl((16i32) as u32)) | (((l_lo_6) & (65535i32))))) as u32);
    F::misc_a__OldPass_SetKey(c, l_seed_4);
    F::OldPass_ClearBits(c);
    F::OldPass_PutBits(c, l_seed_4, 0i32, 32i32);
    { let a0_ = (((c.m.r32(p_in_1) as i32)) as u32); F::OldPass_PutBits(c, a0_, 32i32, 8i32) };
    if ((((((c.m.r32((p_in_1).wrapping_add(8)) as i32)) > (104857500i32)) as i32)) != 0) {
        { let v_ = 104857500i32; let a_ = (p_in_1).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
    }
    { let a0_ = ((rt::div_i32((c.m.r32((p_in_1).wrapping_add(8)) as i32), 100i32)) as u32); F::OldPass_PutBits(c, a0_, 40i32, 20i32) };
    { let a0_ = (((c.m.r32((p_in_1).wrapping_add(12)) as i32)) as u32); F::OldPass_PutBits(c, a0_, 60i32, 2i32) };
    { let a0_ = (((c.m.r32((p_in_1).wrapping_add(16)) as i32)) as u32); F::OldPass_PutBits(c, a0_, 62i32, 1i32) };
    { let a0_ = (((c.m.r32((p_in_1).wrapping_add(20)) as i32)) as u32); F::OldPass_PutBits(c, a0_, 63i32, 4i32) };
    { let a0_ = (((c.m.r32((p_in_1).wrapping_add(24)) as i32)) as u32); F::OldPass_PutBits(c, a0_, 67i32, 1i32) };
    l_sum_3 = 0i32;
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < (68i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let r_ = F::OldPass_GetBit(c, l_i_2); let n_ = (l_sum_3).wrapping_add(r_); l_sum_3 = n_; };
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
    F::OldPass_PutBits(c, ((l_sum_3) as u32), 68i32, 8i32);
    {
        l_i_2 = 0i32;
        'l3: loop {
            if !(((((l_i_2) < (5i32)) as i32)) != 0) { break 'l3; }
            'c4: {
                { let v_ = ((((((c.m.r8((sym::gOldPassBits).wrapping_add(((l_i_2) as u32)))) as i32)) ^ (((c.m.r8((sym::gOldPassBits).wrapping_add((((l_i_2).wrapping_add(5i32)) as u32)))) as i32)))) as u8); let a_ = (sym::gOldPassBits).wrapping_add((((l_i_2).wrapping_add(10i32)) as u32)); c.m.w8(a_, v_); };
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
    {
        l_i_2 = 4i32;
        'l5: loop {
            if !(((((l_i_2) < (15i32)) as i32)) != 0) { break 'l5; }
            'c6: {
                { let r_ = F::OldPass_MtNext(c); let a_ = (sym::gOldPassBits).wrapping_add(((l_i_2) as u32)); let n_ = ((((((c.m.r8(a_)) as u32)) ^ (r_))) as u8); c.m.w8(a_, n_); };
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
    F::OldPass_BitsToText(c, 120i32);
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "OldPass_EncodeShort", &PROBE_N, &[], true); }
    r_
}

pub fn OldPass_GetText(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "OldPass_GetText", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f sym::gOldPassText;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "OldPass_GetText", &PROBE_N, &[], true); }
    r_
}

pub fn OldPass_RandomChar(c: &mut Ctx, mut p_out_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "OldPass_RandomChar", &PROBE_N, &[(p_out_1) as u32], false); }
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
    { let v_ = rt::rem_i32(F::Port_Rand(c), 129i32); let a_ = p_out_1; c.m.w32(a_, (v_) as u32); };
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < (7i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let v_ = rt::rem_i32(F::Port_Rand(c), 566i32); let a_ = ((p_out_1).wrapping_add(4)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
    { let v_ = rt::rem_i32(F::Port_Rand(c), 3i32); let a_ = (p_out_1).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
    { let v_ = rt::rem_i32(F::Port_Rand(c), 7i32); let a_ = (p_out_1).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
    { let v_ = rt::rem_i32(F::Port_Rand(c), 2i32); let a_ = (p_out_1).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "OldPass_RandomChar", &PROBE_N, &[], true); }
    r_
}

pub fn OldPass_RandomShort(c: &mut Ctx, mut p_out_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "OldPass_RandomShort", &PROBE_N, &[(p_out_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = rt::rem_i32(F::Port_Rand(c), 129i32); let a_ = p_out_1; c.m.w32(a_, (v_) as u32); };
    { let v_ = rt::rem_i32(F::Port_Rand(c), 129i32); let a_ = (p_out_1).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
    { let v_ = rt::rem_i32(F::Port_Rand(c), 3i32); let a_ = (p_out_1).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
    { let v_ = (rt::rem_i32(F::Port_Rand(c), 1048575i32)).wrapping_mul(100i32); let a_ = (p_out_1).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
    { let v_ = rt::rem_i32(F::Port_Rand(c), 7i32); let a_ = (p_out_1).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
    { let v_ = rt::rem_i32(F::Port_Rand(c), 2i32); let a_ = (p_out_1).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "OldPass_RandomShort", &PROBE_N, &[], true); }
    r_
}

pub fn OldPass_GetDebugTextA(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "OldPass_GetDebugTextA", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f sym::misc_a__str128;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "OldPass_GetDebugTextA", &PROBE_N, &[], true); }
    r_
}

pub fn OldPass_GetDebugTextB(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "OldPass_GetDebugTextB", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f sym::misc_a__str129;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "OldPass_GetDebugTextB", &PROBE_N, &[], true); }
    r_
}
