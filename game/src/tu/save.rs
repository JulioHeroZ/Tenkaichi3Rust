#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn Save_UnlockAll(c: &mut Ctx, mut p_opt_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Save_UnlockAll", &PROBE_N, &[(p_opt_1) as u32], false); }
    let mut l_slot_2: u32 = 0;
    let mut l_i_3: i32 = 0;
    let mut l_v_4: u32 = 0;
    let r_ = 'f: {
    l_slot_2 = (p_opt_1).wrapping_add(16);
    {
        l_i_3 = 0i32;
        'l1: loop {
            if !(((((l_i_3) < (3i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let v_ = ((0i32) as u64); let a_ = ((p_opt_1).wrapping_add(3088)).wrapping_add((((l_i_3) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            }
            { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
        }
    }
    {
        l_i_3 = 0i32;
        'l3: loop {
            if !(((((l_i_3) < (161i32)) as i32)) != 0) { break 'l3; }
            'c4: {
                { let r_ = (((1i64).wrapping_shl((rt::rem_i32(l_i_3, 64i32)) as u32)) as u64); let a_ = ((c.m.r32(sym::gSaveData)).wrapping_add(3088)).wrapping_add((((rt::div_i32(l_i_3, 64i32)) as u32)).wrapping_mul(8)); let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
            }
            { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
        }
    }
    { let v_ = ((0i32) as u64); let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(3112); c.m.w64(a_, v_); };
    {
        l_i_3 = 0i32;
        'l5: loop {
            if !(((((l_i_3) < (35i32)) as i32)) != 0) { break 'l5; }
            'c6: {
                { let r_ = (((1i64).wrapping_shl((l_i_3) as u32)) as u64); let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(3112); let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
            }
            { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
        }
    }
    {
        l_i_3 = 0i32;
        'l7: loop {
            if !(((((l_i_3) < (25i32)) as i32)) != 0) { break 'l7; }
            'c8: {
                if (((((l_i_3) >= (20i32)) as i32)) != 0) {
                    break 'l7;
                }
                { let r_ = (((1i32).wrapping_shl((l_i_3) as u32)) as u32); let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(3120); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
            }
            { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
        }
    }
    {
        l_i_3 = 0i32;
        'l9: loop {
            if !(((((l_i_3) < (7i32)) as i32)) != 0) { break 'l9; }
            'c10: {
                { let r_ = (1i32).wrapping_shl((l_i_3) as u32); let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(8); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            }
            { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
        }
    }
    {
        l_i_3 = 0i32;
        'l11: loop {
            if !(((((l_i_3) < (350i32)) as i32)) != 0) { break 'l11; }
            'c12: {
                { let r_ = 1i32; let a_ = ((p_opt_1).wrapping_add(11976)).wrapping_add(((l_i_3) as u32)); let n_ = ((((((c.m.r8(a_)) as i32)) | (r_))) as u8); c.m.w8(a_, n_); };
                if ((((F::Rand_Range(c, ((100i32) as u32))) & (((1i32) as u32)))) != 0) {
                    { let r_ = 2i32; let a_ = ((p_opt_1).wrapping_add(11976)).wrapping_add(((l_i_3) as u32)); let n_ = ((((((c.m.r8(a_)) as i32)) | (r_))) as u8); c.m.w8(a_, n_); };
                }
            }
            { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
        }
    }
    { let v_ = 4850000i32; let a_ = (p_opt_1).wrapping_add(12328); c.m.w32(a_, (v_) as u32); };
    {
        l_i_3 = 0i32;
        'l13: loop {
            if !(((((l_i_3) < (14i32)) as i32)) != 0) { break 'l13; }
            'c14: {
                { let v_ = (1i32).wrapping_neg(); let a_ = (((p_opt_1).wrapping_add(11584)).wrapping_add((((l_i_3) as u32)).wrapping_mul(28))).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
            }
            { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
        }
    }
    { let v_ = 20i32; let a_ = (p_opt_1).wrapping_add(524); c.m.w32(a_, (v_) as u32); };
    { let r_ = 1i32; let a_ = (p_opt_1).wrapping_add(520); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    {
        l_i_3 = 0i32;
        'l15: loop {
            if !(((((l_i_3) < (9i32)) as i32)) != 0) { break 'l15; }
            'c16: {
                { let r_ = 3i32; let a_ = (l_slot_2).wrapping_add((((l_i_3) as u32)).wrapping_mul(16)); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                l_v_4 = (((p_opt_1).wrapping_add(16)).wrapping_add((((l_i_3) as u32)).wrapping_mul(16))).wrapping_add(4);
                { let v_ = 65535i32; let a_ = (l_v_4).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = 65535i32; let a_ = (l_v_4).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = 65535i32; let a_ = (l_v_4).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            }
            { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Save_UnlockAll", &PROBE_N, &[], true); }
    r_
}

pub fn Save_SetDefaults(c: &mut Ctx, mut p_opt_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Save_SetDefaults", &PROBE_N, &[(p_opt_1) as u32], false); }
    let mut l_i_2: i32 = 0;
    let mut l_info_3: u32 = 0;
    let r_ = 'f: {
    l_info_3 = ({ let a_ = ((c.m.r32(sym::gCommonRes)).wrapping_add(4)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add((rt::div_u32({ let a_ = ({ let a_ = ((c.m.r32(sym::gCommonRes)).wrapping_add(4)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(8); c.m.r32(a_) }, ((4i32) as u32))).wrapping_mul(((4i32) as u32)));
    F::Progress_ClearSession(c);
    { let _ = F::memset(c, p_opt_1, 0i32, ((16384i32) as u32)); };
    { let r_ = 3i32; let a_ = ((p_opt_1).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(16)); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    { let r_ = 1i32; let a_ = ((((p_opt_1).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    { let r_ = 1i32; let a_ = ((((p_opt_1).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(4)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    { let v_ = 1i32; let a_ = (p_opt_1).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
    F::Save_ResetRules(c);
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < (161i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                's3: {
                    let sv4: i32 = l_i_2;
                    match sv4 {
                        7i32 | 8i32 | 9i32 | 10i32 | 24i32 | 38i32 | 54i32 | 69i32 | 70i32 | 71i32 | 72i32 | 75i32 | 76i32 | 97i32 | 110i32 | 120i32 | 145i32 | 149i32 | 150i32 | 151i32 | 152i32 | 153i32 | 154i32 | 155i32 | 156i32 | 157i32 | 158i32 | 159i32 | 160i32 => {
                            break 's3;
                        }
                        _ => {
                            { let r_ = (((1i64).wrapping_shl((rt::rem_i32(l_i_2, 64i32)) as u32)) as u64); let a_ = ((c.m.r32(sym::gSaveData)).wrapping_add(3088)).wrapping_add((((rt::div_i32(l_i_2, 64i32)) as u32)).wrapping_mul(8)); let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
                            break 's3;
                        }
                    }
                }
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
    {
        l_i_2 = 0i32;
        'l4: loop {
            if !(((((l_i_2) < (35i32)) as i32)) != 0) { break 'l4; }
            'c5: {
                's6: {
                    let sv5: i32 = l_i_2;
                    match sv5 {
                        17i32 | 18i32 | 19i32 | 20i32 | 21i32 | 22i32 | 31i32 | 32i32 => {
                            break 's6;
                        }
                        _ => {
                            { let r_ = (((1i64).wrapping_shl((l_i_2) as u32)) as u64); let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(3112); let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
                            break 's6;
                        }
                    }
                }
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
    {
        l_i_2 = 0i32;
        'l7: loop {
            if !(((((l_i_2) < (25i32)) as i32)) != 0) { break 'l7; }
            'c8: {
                if (((((l_i_2) >= (20i32)) as i32)) != 0) {
                    break 'l7;
                }
                { let r_ = (((1i32).wrapping_shl((l_i_2) as u32)) as u32); let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(3120); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
    {
        l_i_2 = 0i32;
        'l9: loop {
            if !(((((l_i_2) < (350i32)) as i32)) != 0) { break 'l9; }
            'c10: {
                if (((((c.m.r32(((l_info_3).wrapping_add((((l_i_2) as u32)).wrapping_mul(40))).wrapping_add(20)) as i32)) & (2i32))) != 0) {
                    F::Save_AddItem(c, l_i_2);
                }
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
    {
        l_i_2 = 0i32;
        'l11: loop {
            if !(((((l_i_2) < (14i32)) as i32)) != 0) { break 'l11; }
            'c12: {
                { let v_ = (1i32).wrapping_neg(); let a_ = (((p_opt_1).wrapping_add(11584)).wrapping_add((((l_i_2) as u32)).wrapping_mul(28))).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
    { let v_ = 5i32; let a_ = (p_opt_1).wrapping_add(524); c.m.w32(a_, (v_) as u32); };
    { let v_ = 99i32; let a_ = (p_opt_1).wrapping_add(1916); c.m.w32(a_, (v_) as u32); };
    {
        l_i_2 = 0i32;
        'l13: loop {
            if !(((((l_i_2) < (2i32)) as i32)) != 0) { break 'l13; }
            'c14: {
                { let v_ = 2i32; let a_ = (((p_opt_1).wrapping_add(5644)).wrapping_add((((l_i_2) as u32)).wrapping_mul(32))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = 1i32; let a_ = (((p_opt_1).wrapping_add(5644)).wrapping_add((((l_i_2) as u32)).wrapping_mul(32))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (((p_opt_1).wrapping_add(5644)).wrapping_add((((l_i_2) as u32)).wrapping_mul(32))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = 3i32; let a_ = (((p_opt_1).wrapping_add(5644)).wrapping_add((((l_i_2) as u32)).wrapping_mul(32))).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = 4i32; let a_ = (((p_opt_1).wrapping_add(5644)).wrapping_add((((l_i_2) as u32)).wrapping_mul(32))).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = 5i32; let a_ = (((p_opt_1).wrapping_add(5644)).wrapping_add((((l_i_2) as u32)).wrapping_mul(32))).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = 6i32; let a_ = (((p_opt_1).wrapping_add(5644)).wrapping_add((((l_i_2) as u32)).wrapping_mul(32))).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = 7i32; let a_ = (((p_opt_1).wrapping_add(5644)).wrapping_add((((l_i_2) as u32)).wrapping_mul(32))).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = 2i32; let a_ = (((p_opt_1).wrapping_add(5708)).wrapping_add((((l_i_2) as u32)).wrapping_mul(32))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = 1i32; let a_ = (((p_opt_1).wrapping_add(5708)).wrapping_add((((l_i_2) as u32)).wrapping_mul(32))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (((p_opt_1).wrapping_add(5708)).wrapping_add((((l_i_2) as u32)).wrapping_mul(32))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = 3i32; let a_ = (((p_opt_1).wrapping_add(5708)).wrapping_add((((l_i_2) as u32)).wrapping_mul(32))).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = 4i32; let a_ = (((p_opt_1).wrapping_add(5708)).wrapping_add((((l_i_2) as u32)).wrapping_mul(32))).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = 5i32; let a_ = (((p_opt_1).wrapping_add(5708)).wrapping_add((((l_i_2) as u32)).wrapping_mul(32))).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = 6i32; let a_ = (((p_opt_1).wrapping_add(5708)).wrapping_add((((l_i_2) as u32)).wrapping_mul(32))).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = 7i32; let a_ = (((p_opt_1).wrapping_add(5708)).wrapping_add((((l_i_2) as u32)).wrapping_mul(32))).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
    { let r_ = 7i32; let a_ = (p_opt_1).wrapping_add(5640); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    { let v_ = 0i32; let a_ = (p_opt_1).wrapping_add(5780); c.m.w32(a_, (v_) as u32); };
    { let v_ = 9i32; let a_ = (p_opt_1).wrapping_add(5800); c.m.w32(a_, (v_) as u32); };
    { let v_ = 9i32; let a_ = (p_opt_1).wrapping_add(5804); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Save_SetDefaults", &PROBE_N, &[], true); }
    r_
}

pub fn Save_Stub266600(c: &mut Ctx, mut p_opt_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Save_Stub266600", &PROBE_N, &[(p_opt_1) as u32], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Save_Stub266600", &PROBE_N, &[], true); }
    r_
}

pub fn Save_Init(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Save_Init", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let v_ = F::Heap_Alloc(c, 16384i32, ((32i32) as u32), 0i32, 2i32); let a_ = sym::gSaveData; c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32(sym::gSaveData); F::memset(c, a0_, 0i32, ((16384i32) as u32)) }; };
    { let a0_ = c.m.r32(sym::gSaveData); F::Save_SetDefaults(c, a0_) };
    { let a0_ = c.m.r32(sym::gSaveData); F::Save_Stub266600(c, a0_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Save_Init", &PROBE_N, &[], true); }
    r_
}

pub fn Save_AddItem(c: &mut Ctx, mut p_idx_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Save_AddItem", &PROBE_N, &[(p_idx_1) as u32], false); }
    let r_ = 'f: {
    if ((((!(((((({ let a_ = ((c.m.r32(sym::gSaveData)).wrapping_add(11976)).wrapping_add(((p_idx_1) as u32)); c.m.r8(a_) }) as i32)) & (1i32))) != 0)) as i32)) != 0) {
        { let r_ = 1i32; let a_ = ((c.m.r32(sym::gSaveData)).wrapping_add(11976)).wrapping_add(((p_idx_1) as u32)); let n_ = ((((((c.m.r8(a_)) as i32)) | (r_))) as u8); c.m.w8(a_, n_); };
        { let r_ = 2i32; let a_ = ((c.m.r32(sym::gSaveData)).wrapping_add(11976)).wrapping_add(((p_idx_1) as u32)); let n_ = ((((((c.m.r8(a_)) as i32)) | (r_))) as u8); c.m.w8(a_, n_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Save_AddItem", &PROBE_N, &[], true); }
    r_
}

pub fn Save_AddMoney(c: &mut Ctx, mut p_amount_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Save_AddMoney", &PROBE_N, &[(p_amount_1) as u32], false); }
    let r_ = 'f: {
    { let r_ = p_amount_1; let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(12328); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
    if ((((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(12328); (c.m.r32(a_) as i32) }) > (9999999i32)) as i32)) != 0) {
        { let v_ = 9999999i32; let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(12328); c.m.w32(a_, (v_) as u32); };
    } else {
        if ((((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(12328); (c.m.r32(a_) as i32) }) < (0i32)) as i32)) != 0) {
            { let v_ = 0i32; let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(12328); c.m.w32(a_, (v_) as u32); };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Save_AddMoney", &PROBE_N, &[], true); }
    r_
}

pub fn Save_ResetRules(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Save_ResetRules", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let v_ = 3i32; let a_ = ((c.m.r32(sym::gSaveData)).wrapping_add(3124)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    { let v_ = 2i32; let a_ = ((c.m.r32(sym::gSaveData)).wrapping_add(3124)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    { let v_ = 2i32; let a_ = ((c.m.r32(sym::gSaveData)).wrapping_add(3124)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    { let v_ = 0i32; let a_ = ((c.m.r32(sym::gSaveData)).wrapping_add(3124)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    { let v_ = 0i32; let a_ = ((c.m.r32(sym::gSaveData)).wrapping_add(3124)).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    { let v_ = 0i32; let a_ = ((c.m.r32(sym::gSaveData)).wrapping_add(3124)).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Save_ResetRules", &PROBE_N, &[], true); }
    r_
}
