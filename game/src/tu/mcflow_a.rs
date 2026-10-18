#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn McFlow_ClearSlotInfo(c: &mut Ctx, mut p_kind_1: i32, mut p_slot_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "McFlow_ClearSlotInfo", &PROBE_N, &[(p_kind_1) as u32, (p_slot_2) as u32], false); }
    let mut l_i_3: i32 = 0;
    let mut l_j_4: i32 = 0;
    let r_ = 'f: {
    if (((((p_kind_1) == (0i32)) as i32)) != 0) {
        { let r_ = 1i32; let a_ = ((c.m.r32(sym::gProgress)).wrapping_add(1692)).wrapping_add((((p_slot_2) as u32)).wrapping_mul(44)); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        {
            l_i_3 = 0i32;
            'l1: loop {
                if !(((((l_i_3) < (2i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    {
                        l_j_4 = 0i32;
                        'l3: loop {
                            if !(((((l_j_4) < (5i32)) as i32)) != 0) { break 'l3; }
                            'c4: {
                                { let v_ = 164i32; let a_ = (((((c.m.r32(sym::gProgress)).wrapping_add(1692)).wrapping_add((((p_slot_2) as u32)).wrapping_mul(44))).wrapping_add(4)).wrapping_add((((l_i_3) as u32)).wrapping_mul(20))).wrapping_add((((l_j_4) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                            }
                            { let o_ = l_j_4; let n_ = (o_).wrapping_add(1); l_j_4 = n_; };
                        }
                    }
                }
                { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
            }
        }
    } else {
        if (((((p_kind_1) == (1i32)) as i32)) != 0) {
            { let r_ = 1i32; let a_ = ((c.m.r32(sym::gProgress)).wrapping_add(1692)).wrapping_add((((p_slot_2) as u32)).wrapping_mul(44)); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            {
                l_i_3 = 0i32;
                'l5: loop {
                    if !(((((l_i_3) < (2i32)) as i32)) != 0) { break 'l5; }
                    'c6: {
                        {
                            l_j_4 = 0i32;
                            'l7: loop {
                                if !(((((l_j_4) < (5i32)) as i32)) != 0) { break 'l7; }
                                'c8: {
                                    { let v_ = 164i32; let a_ = (((((c.m.r32(sym::gProgress)).wrapping_add(1692)).wrapping_add((((p_slot_2) as u32)).wrapping_mul(44))).wrapping_add(4)).wrapping_add((((l_i_3) as u32)).wrapping_mul(20))).wrapping_add((((l_j_4) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                }
                                { let o_ = l_j_4; let n_ = (o_).wrapping_add(1); l_j_4 = n_; };
                            }
                        }
                    }
                    { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
                }
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "McFlow_ClearSlotInfo", &PROBE_N, &[], true); }
    r_
}

pub fn McFlow_CopySlotInfo(c: &mut Ctx, mut p_kind_1: i32, mut p_slot_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "McFlow_CopySlotInfo", &PROBE_N, &[(p_kind_1) as u32, (p_slot_2) as u32], false); }
    let mut l_i_3: i32 = 0;
    let mut l_j_4: i32 = 0;
    let r_ = 'f: {
    if (((((p_kind_1) == (0i32)) as i32)) != 0) {
        { let r_ = 1i32; let a_ = ((c.m.r32(sym::gProgress)).wrapping_add(1692)).wrapping_add((((p_slot_2) as u32)).wrapping_mul(44)); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        {
            l_i_3 = 0i32;
            'l1: loop {
                if !(((((l_i_3) < (2i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    {
                        l_j_4 = 0i32;
                        'l3: loop {
                            if !(((((l_j_4) < (5i32)) as i32)) != 0) { break 'l3; }
                            'c4: {
                                { let v_ = { let a_ = ((((c.m.r32(sym::gMcFlow)).wrapping_add(16440)).wrapping_add(16)).wrapping_add((((l_i_3) as u32)).wrapping_mul(20))).wrapping_add((((l_j_4) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (((((c.m.r32(sym::gProgress)).wrapping_add(1692)).wrapping_add((((p_slot_2) as u32)).wrapping_mul(44))).wrapping_add(4)).wrapping_add((((l_i_3) as u32)).wrapping_mul(20))).wrapping_add((((l_j_4) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                            }
                            { let o_ = l_j_4; let n_ = (o_).wrapping_add(1); l_j_4 = n_; };
                        }
                    }
                }
                { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
            }
        }
    } else {
        if (((((p_kind_1) == (1i32)) as i32)) != 0) {
            { let r_ = 1i32; let a_ = ((c.m.r32(sym::gProgress)).wrapping_add(1692)).wrapping_add((((p_slot_2) as u32)).wrapping_mul(44)); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            {
                l_i_3 = 0i32;
                'l5: loop {
                    if !(((((l_i_3) < (2i32)) as i32)) != 0) { break 'l5; }
                    'c6: {
                        {
                            l_j_4 = 0i32;
                            'l7: loop {
                                if !(((((l_j_4) < (5i32)) as i32)) != 0) { break 'l7; }
                                'c8: {
                                    { let v_ = { let a_ = (((((c.m.r32(sym::gMcFlow)).wrapping_add(235576)).wrapping_add((((p_slot_2) as u32)).wrapping_mul(56))).wrapping_add(16)).wrapping_add((((l_i_3) as u32)).wrapping_mul(20))).wrapping_add((((l_j_4) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (((((c.m.r32(sym::gProgress)).wrapping_add(1692)).wrapping_add((((p_slot_2) as u32)).wrapping_mul(44))).wrapping_add(4)).wrapping_add((((l_i_3) as u32)).wrapping_mul(20))).wrapping_add((((l_j_4) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                }
                                { let o_ = l_j_4; let n_ = (o_).wrapping_add(1); l_j_4 = n_; };
                            }
                        }
                    }
                    { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
                }
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "McFlow_CopySlotInfo", &PROBE_N, &[], true); }
    r_
}

pub fn McFlow_SetupRequest(c: &mut Ctx, mut p_kind_1: i32, mut p_slot_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "McFlow_SetupRequest", &PROBE_N, &[(p_kind_1) as u32, (p_slot_2) as u32], false); }
    let r_ = 'f: {
    if (((((p_kind_1) == (0i32)) as i32)) != 0) {
        { let v_ = ({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((rt::div_u32({ let a_ = ({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }, ((4i32) as u32))).wrapping_mul(((4i32) as u32))); let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(235968); c.m.w32(a_, v_); };
        { let v_ = 40090i32; let a_ = ((c.m.r32(sym::gMcFlow)).wrapping_add(235968)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        { let _ = { let a0_ = ((c.m.r32(sym::gMcFlow)).wrapping_add(235968)).wrapping_add(792); ((((F::strcpy(c, a0_, sym::mcflow_a__str0)) as u64)) as i32) }; };
        { let v_ = 24i32; let a_ = ((c.m.r32(sym::gMcFlow)).wrapping_add(235968)).wrapping_add(1052); c.m.w32(a_, (v_) as u32); };
        { let a0_ = (c.m.r32(sym::gMcFlow)).wrapping_add(235968); F::McCardFile_SetNames(c, a0_, sym::mcflow_a__str1, (1i32).wrapping_neg(), (1i32).wrapping_neg(), 0i32) };
    } else {
        { let v_ = ({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((rt::div_u32({ let a_ = ({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }, ((4i32) as u32))).wrapping_mul(((4i32) as u32))); let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(235968); c.m.w32(a_, v_); };
        { let v_ = 48778i32; let a_ = ((c.m.r32(sym::gMcFlow)).wrapping_add(235968)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        { let _ = { let a0_ = ((c.m.r32(sym::gMcFlow)).wrapping_add(235968)).wrapping_add(792); ((((F::strcpy(c, a0_, sym::mcflow_a__str2)) as u64)) as i32) }; };
        { let v_ = 30i32; let a_ = ((c.m.r32(sym::gMcFlow)).wrapping_add(235968)).wrapping_add(1052); c.m.w32(a_, (v_) as u32); };
        { let r_ = (p_slot_2).wrapping_add(1i32); let a_ = (((c.m.r32(sym::gMcFlow)).wrapping_add(235968)).wrapping_add(792)).wrapping_add(((45i32) as u32)); let n_ = ((((((c.m.r8(a_) as i8)) as i32)).wrapping_add(r_)) as i8); c.m.w8(a_, (n_) as u8); };
        { let a0_ = (c.m.r32(sym::gMcFlow)).wrapping_add(235968); F::McCardFile_SetNames(c, a0_, sym::mcflow_a__str3, p_slot_2, (1i32).wrapping_neg(), 1i32) };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "McFlow_SetupRequest", &PROBE_N, &[], true); }
    r_
}

pub fn McFlow_CallDone(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "McFlow_CallDone", &PROBE_N, &[], false); }
    let mut l_arg_1: i32 = 0;
    let mut l_cb_2: u32 = 0;
    let r_ = 'f: {
    if (((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(12); (c.m.r32(a_) as i32) }) & (8i32))) != 0) {
        l_cb_2 = { let a_ = ((c.m.r32(sym::gMcFlow)).wrapping_add(237028)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) };
        l_arg_1 = { let a_ = ((c.m.r32(sym::gMcFlow)).wrapping_add(237044)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
    } else {
        if (((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(12); (c.m.r32(a_) as i32) }) & (256i32))) != 0) {
            l_cb_2 = { let a_ = ((c.m.r32(sym::gMcFlow)).wrapping_add(237028)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.r32(a_) };
            l_arg_1 = { let a_ = ((c.m.r32(sym::gMcFlow)).wrapping_add(237044)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
            { let r_ = 256i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(12); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
        } else {
            l_cb_2 = { let a_ = ((c.m.r32(sym::gMcFlow)).wrapping_add(237028)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) };
            l_arg_1 = { let a_ = ((c.m.r32(sym::gMcFlow)).wrapping_add(237044)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
        }
    }
    if (((((l_cb_2) != (((0i32) as u32))) as i32)) != 0) {
        { let _ = D::call(c, l_cb_2, &[((l_arg_1) as i64 as u64)]); };
    }
    { let r_ = (!(8i32)); let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(12); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
    { let r_ = (!(256i32)); let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(12); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "McFlow_CallDone", &PROBE_N, &[], true); }
    r_
}

pub fn McFlow_CallModeDone(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "McFlow_CallModeDone", &PROBE_N, &[], false); }
    let mut l_arg_1: i32 = 0;
    let mut l_cb_2: u32 = 0;
    let r_ = 'f: {
    if (((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(12); (c.m.r32(a_) as i32) }) & (8i32))) != 0) {
        l_cb_2 = { let a_ = (((c.m.r32(sym::gMcFlow)).wrapping_add(237060)).wrapping_add(((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(32); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) };
        l_arg_1 = { let a_ = (((c.m.r32(sym::gMcFlow)).wrapping_add(237252)).wrapping_add(((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(32); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(16))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
    } else {
        if (((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(12); (c.m.r32(a_) as i32) }) & (256i32))) != 0) {
            l_cb_2 = { let a_ = (((c.m.r32(sym::gMcFlow)).wrapping_add(237060)).wrapping_add(((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(32); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.r32(a_) };
            l_arg_1 = { let a_ = (((c.m.r32(sym::gMcFlow)).wrapping_add(237252)).wrapping_add(((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(32); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(16))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
        } else {
            l_cb_2 = { let a_ = (((c.m.r32(sym::gMcFlow)).wrapping_add(237060)).wrapping_add(((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(32); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) };
            l_arg_1 = { let a_ = (((c.m.r32(sym::gMcFlow)).wrapping_add(237252)).wrapping_add(((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(32); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
        }
    }
    if (((((l_cb_2) != (((0i32) as u32))) as i32)) != 0) {
        { let _ = D::call(c, l_cb_2, &[((l_arg_1) as i64 as u64)]); };
    }
    { let r_ = (!(8i32)); let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(12); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
    { let r_ = (!(256i32)); let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(12); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "McFlow_CallModeDone", &PROBE_N, &[], true); }
    r_
}

pub fn McFlow_CallModeRemoved(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "McFlow_CallModeRemoved", &PROBE_N, &[], false); }
    let mut l_cb_1: u32 = 0;
    let mut l_arg_2: i32 = 0;
    let r_ = 'f: {
    l_cb_1 = { let a_ = (((c.m.r32(sym::gMcFlow)).wrapping_add(237060)).wrapping_add(((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(32); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(16))).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.r32(a_) };
    l_arg_2 = { let a_ = (((c.m.r32(sym::gMcFlow)).wrapping_add(237252)).wrapping_add(((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(32); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(16))).wrapping_add((((3i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
    if (((((l_cb_1) != (((0i32) as u32))) as i32)) != 0) {
        { let _ = D::call(c, l_cb_1, &[((l_arg_2) as i64 as u64)]); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "McFlow_CallModeRemoved", &PROBE_N, &[], true); }
    r_
}

pub fn McFlow_UpdateSave(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "McFlow_UpdateSave", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
        break 'f;
    }
    's1: {
        let sv1: i32 = { let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); (c.m.r32(a_) as i32) };
        'k24: {
        'k23: {
        'k22: {
        'k21: {
        'k20: {
        'k19: {
        'k18: {
        'k17: {
        'k16: {
        'k15: {
        'k14: {
        'k13: {
        'k12: {
        'k11: {
        'k10: {
        'k9: {
        'k8: {
        'k7: {
        'k6: {
        'k5: {
        'k4: {
        'k3: {
        'k2: {
            match sv1 { 35i32 => break 'k2, 32i32 => break 'k3, 12i32 => break 'k4, 3i32 => break 'k5, 14i32 => break 'k6, 23i32 => break 'k7, 24i32 => break 'k8, 13i32 => break 'k9, 11i32 => break 'k10, 15i32 => break 'k11, 6i32 => break 'k12, 30i32 => break 'k13, 16i32 => break 'k14, 17i32 => break 'k15, 25i32 => break 'k16, 26i32 => break 'k17, 27i32 => break 'k18, 18i32 => break 'k19, 31i32 => break 'k20, 33i32 => break 'k21, 34i32 => break 'k22, 37i32 => break 'k23, 38i32 => break 'k24, _ => break 's1, }
        }
        F::Dialog_SetLayout(c, 1i32);
        F::Dialog_SetCursor(c, 1i32);
        F::Dialog_SetTitle(c, 0i32);
        F::Dialog_Start(c, 0i32);
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
        { let v_ = { let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(28); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
        break 's1;
        }
        F::Dialog_SetChoices(c, 1i32);
        { let v_ = 32i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
        if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); (c.m.r32(a_) as i32) }) > (0i32)) as i32)) != 0) {
            if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) >= (0i32)) as i32)) != 0) {
                break 's1;
            }
            { let v_ = 3i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
            { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            { let v_ = 0i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
            F::McCard_ResetStep(c);
        } else {
            if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); (c.m.r32(a_) as i32) }) < (0i32)) as i32)) != 0) {
                { let v_ = 18i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
                F::Dialog_SetCursor(c, 1i32);
            } else {
                { let v_ = F::Dialog_Input(c, 0i32); let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
                { let v_ = 12i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            }
        }
        break 's1;
        }
        F::Dialog_SetChoices(c, 1i32);
        { let v_ = 13i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
        if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); (c.m.r32(a_) as i32) }) > (0i32)) as i32)) != 0) {
            if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) >= (0i32)) as i32)) != 0) {
                break 's1;
            }
            { let v_ = 3i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
            { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            { let v_ = 0i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
            F::McCard_ResetStep(c);
        } else {
            if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); (c.m.r32(a_) as i32) }) < (0i32)) as i32)) != 0) {
                { let v_ = 18i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
                F::Dialog_SetCursor(c, 1i32);
            } else {
                { let v_ = F::Dialog_Input(c, 0i32); let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
                { let v_ = 12i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            }
        }
        break 's1;
        }
        F::Dialog_SetChoices(c, 0i32);
        { let v_ = 5i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
        if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) >= (0i32)) as i32)) != 0) {
            if (((((F::McCard_GetInfo(c, 0i32)) == (0i32)) as i32)) != 0) {
                break 's1;
            }
            if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); (c.m.r32(a_) as i32) }) > (0i32)) as i32)) != 0) {
                break 's1;
            }
            if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(12)) as i32)) != (2i32)) as i32)) != 0) {
                { let v_ = 27i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            if (({ let a0_ = (c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32); F::McCard_IsFatalError(c, a0_) }) != 0) {
                { let v_ = 27i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32)) != (0i32)) as i32)) != 0) {
                if (((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(20); (c.m.r32(a_) as i32) }) & (4i32))) != 0) {
                    { let r_ = 4i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(20); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
                    break 's1;
                }
                { let _ = F::McCard_Probe(c, 0i32); };
                if (({ let a0_ = (c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32); F::McCard_IsFatalError(c, a0_) }) != 0) {
                    { let v_ = 27i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
                    break 's1;
                }
                { let v_ = 14i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                F::Dialog_SetCursor(c, 1i32);
                break 's1;
            }
            if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(20)) as i32)) == (0i32)) as i32)) != 0) {
                { let v_ = 23i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                F::Dialog_SetCursor(c, 1i32);
            }
            break 's1;
        }
        if ((((({ let a1_ = (c.m.r32(sym::gMcFlow)).wrapping_add(235968); F::McCard_CheckFiles(c, 0i32, a1_) }) == (0i32)) as i32)) != 0) {
            if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(12)) as i32)) != (2i32)) as i32)) != 0) {
                { let v_ = 27i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(20)) as i32)) == (0i32)) as i32)) != 0) {
                { let v_ = 23i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                F::Dialog_SetCursor(c, 1i32);
                break 's1;
            }
            if (({ let a0_ = (c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32); F::McCard_IsFatalError(c, a0_) }) != 0) {
                { let v_ = 27i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32)) >= (0i32)) as i32)) != 0) {
                break 's1;
            }
            if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(16)) as i32)) < (61i32)) as i32)) != 0) {
                { let v_ = 30i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            { let v_ = 6i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
        } else {
            F::McCard_ResetStep(c);
            { let v_ = 31i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
        }
        break 's1;
        }
        F::Dialog_SetChoices(c, 1i32);
        { let v_ = 15i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
        if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); (c.m.r32(a_) as i32) }) > (0i32)) as i32)) != 0) {
            if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) >= (0i32)) as i32)) != 0) {
                break 's1;
            }
            { let v_ = 3i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
            { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            { let v_ = 0i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
            { let _ = F::McCard_Probe(c, 0i32); };
        } else {
            if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); (c.m.r32(a_) as i32) }) < ((1i32).wrapping_neg())) as i32)) != 0) {
                { let v_ = 18i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
                F::Dialog_SetCursor(c, 1i32);
            } else {
                { let v_ = F::Dialog_Input(c, 0i32); let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
                { let v_ = 12i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            }
        }
        break 's1;
        }
        F::Dialog_SetChoices(c, 1i32);
        { let v_ = 24i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
        { let _ = F::McCard_Probe(c, 0i32); };
        if (({ let a0_ = (c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32); F::McCard_IsFatalError(c, a0_) }) != 0) {
            F::Dialog_SetChoices(c, 0i32);
            { let v_ = 28i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
            { let v_ = 27i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
            { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            { let v_ = 0i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
            break 's1;
        }
        if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); (c.m.r32(a_) as i32) }) > (0i32)) as i32)) != 0) {
            if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) >= (0i32)) as i32)) != 0) {
                break 's1;
            }
            { let v_ = 24i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
            { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            { let v_ = 0i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
        } else {
            if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); (c.m.r32(a_) as i32) }) < (0i32)) as i32)) != 0) {
                { let v_ = 18i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
                F::Dialog_SetCursor(c, 1i32);
            } else {
                { let v_ = F::Dialog_Input(c, 0i32); let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
                { let v_ = 12i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            }
        }
        break 's1;
        }
        F::Dialog_SetChoices(c, 0i32);
        { let v_ = 25i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
        if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) >= (0i32)) as i32)) != 0) {
            if (((((F::McCard_GetInfo(c, 0i32)) == (0i32)) as i32)) != 0) {
                break 's1;
            }
            if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); (c.m.r32(a_) as i32) }) > (0i32)) as i32)) != 0) {
                break 's1;
            }
            if (({ let a0_ = (c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32); F::McCard_IsFatalError(c, a0_) }) != 0) {
                { let v_ = 26i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            }
            break 's1;
        }
        if (((((F::McCard_Format(c, 0i32)) == (0i32)) as i32)) != 0) {
            if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32)) < (0i32)) as i32)) != 0) {
                { let v_ = 26i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            }
        } else {
            { let v_ = 25i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
            { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
        }
        break 's1;
        }
        F::Dialog_SetChoices(c, 1i32);
        { let v_ = 14i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
        { let _ = F::McCard_Probe(c, 0i32); };
        if (({ let a0_ = (c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32); F::McCard_IsFatalError(c, a0_) }) != 0) {
            { let v_ = 27i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
            { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            { let v_ = 0i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
            break 's1;
        }
        if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); (c.m.r32(a_) as i32) }) > (0i32)) as i32)) != 0) {
            if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) >= (0i32)) as i32)) != 0) {
                break 's1;
            }
            { let v_ = 15i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
            { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            { let v_ = 0i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
        } else {
            if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); (c.m.r32(a_) as i32) }) < (0i32)) as i32)) != 0) {
                { let v_ = 18i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
                F::Dialog_SetCursor(c, 1i32);
            } else {
                { let v_ = F::Dialog_Input(c, 0i32); let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
                { let v_ = 12i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            }
        }
        break 's1;
        }
        F::Dialog_SetChoices(c, 0i32);
        { let v_ = 10i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
        if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) >= (0i32)) as i32)) != 0) {
            if (((((F::McCard_GetInfo(c, 0i32)) == (0i32)) as i32)) != 0) {
                break 's1;
            }
            if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); (c.m.r32(a_) as i32) }) > (0i32)) as i32)) != 0) {
                break 's1;
            }
            if (({ let a0_ = (c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32); F::McCard_IsFatalError(c, a0_) }) != 0) {
                { let v_ = 17i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            }
            break 's1;
        }
        if ((((({ let a1_ = (c.m.r32(sym::gMcFlow)).wrapping_add(235968); F::McCard_CreateSave(c, 0i32, a1_) }) == (0i32)) as i32)) != 0) {
            if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32)) < (0i32)) as i32)) != 0) {
                { let v_ = 17i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            }
        } else {
            { let v_ = 15i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
        }
        break 's1;
        }
        F::Dialog_SetChoices(c, 0i32);
        { let v_ = 16i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
        if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) >= (0i32)) as i32)) != 0) {
            if (((((F::McCard_GetInfo(c, 0i32)) == (0i32)) as i32)) != 0) {
                break 's1;
            }
            if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); (c.m.r32(a_) as i32) }) > (0i32)) as i32)) != 0) {
                break 's1;
            }
            if (({ let a0_ = (c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32); F::McCard_IsFatalError(c, a0_) }) != 0) {
                { let v_ = 17i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            }
            break 's1;
        }
        if ((((({ let a1_ = c.m.r32(sym::gSaveData); let a3_ = (c.m.r32(sym::gMcFlow)).wrapping_add(235968); F::McCard_WriteSave(c, 0i32, a1_, 16384i32, a3_) }) == (0i32)) as i32)) != 0) {
            if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32)) < (0i32)) as i32)) != 0) {
                { let v_ = 17i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            }
        } else {
            { let v_ = 16i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
            { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
        }
        break 's1;
        }
        F::Dialog_SetChoices(c, 0i32);
        { let v_ = 5i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
        if (((((F::McCard_FindSystemSave(c, 0i32)) == (0i32)) as i32)) != 0) {
            if (({ let a0_ = (c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32); F::McCard_IsFatalError(c, a0_) }) != 0) {
                { let v_ = 27i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            }
            break 's1;
        }
        if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32)) == (1i32)) as i32)) != 0) {
            { let v_ = 11i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
            { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
        } else {
            { let v_ = 11i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
            { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
        }
        break 's1;
        }
        F::Dialog_SetChoices(c, 0i32);
        { let v_ = 31i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
            if ((F::Dialog_IsOpen(c)) != 0) {
                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                { let v_ = 18i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
                F::Dialog_SetCursor(c, 1i32);
            }
        }
        break 's1;
        }
        F::Dialog_SetChoices(c, 0i32);
        { let v_ = 17i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
            if ((F::Dialog_IsOpen(c)) != 0) {
                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                { let r_ = 8i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(12); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 37i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
        }
        if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) >= (0i32)) as i32)) != 0) {
            break 's1;
        }
        { let r_ = 8i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(12); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        { let v_ = 37i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
        break 's1;
        }
        F::Dialog_SetChoices(c, 0i32);
        { let v_ = 18i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
            if ((F::Dialog_IsOpen(c)) != 0) {
                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                { let v_ = 18i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
                F::Dialog_SetCursor(c, 1i32);
            }
        }
        break 's1;
        }
        F::Dialog_SetChoices(c, 0i32);
        { let v_ = 26i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
        { let _ = F::McCard_Probe(c, 0i32); };
        if (({ let a0_ = (c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32); F::McCard_IsFatalError(c, a0_) }) != 0) {
            { let v_ = 27i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
            { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            { let v_ = 0i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
            break 's1;
        }
        if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) >= (0i32)) as i32)) != 0) {
            break 's1;
        }
        { let v_ = 11i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
        { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
        break 's1;
        }
        F::Dialog_SetChoices(c, 0i32);
        { let v_ = 27i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
            if ((F::Dialog_IsOpen(c)) != 0) {
                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                { let v_ = 18i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
                F::Dialog_SetCursor(c, 1i32);
            }
        }
        break 's1;
        }
        F::Dialog_SetChoices(c, 0i32);
        { let v_ = 28i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
            if ((F::Dialog_IsOpen(c)) != 0) {
                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                { let v_ = 18i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
                F::Dialog_SetCursor(c, 1i32);
            }
        }
        break 's1;
        }
        F::Dialog_SetChoices(c, 1i32);
        { let v_ = 19i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
        if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); (c.m.r32(a_) as i32) }) > (0i32)) as i32)) != 0) {
            if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) >= (0i32)) as i32)) != 0) {
                break 's1;
            }
            { let v_ = 37i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
            { let v_ = 0i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
        } else {
            if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); (c.m.r32(a_) as i32) }) < ((1i32).wrapping_neg())) as i32)) != 0) {
                { let v_ = { let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(28); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
                F::Dialog_SetCursor(c, 1i32);
                F::McCard_ResetStep(c);
            } else {
                { let v_ = F::Dialog_Input(c, 0i32); let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
                { let v_ = 12i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            }
        }
        break 's1;
        }
        F::Dialog_SetChoices(c, 0i32);
        { let v_ = 5i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
        if ((((({ let a1_ = (c.m.r32(sym::gMcFlow)).wrapping_add(235968); F::McCard_CalcNeed(c, 0i32, a1_, 0i32) }) == (0i32)) as i32)) != 0) {
            if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(12)) as i32)) != (2i32)) as i32)) != 0) {
                { let v_ = 27i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(20)) as i32)) == (0i32)) as i32)) != 0) {
                { let v_ = 23i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                F::Dialog_SetCursor(c, 1i32);
                break 's1;
            }
            if (({ let a0_ = (c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32); F::McCard_IsFatalError(c, a0_) }) != 0) {
                { let v_ = 27i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            }
            break 's1;
        }
        if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(28)) as i32)) > (0i32)) as i32)) != 0) {
            if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(16)) as i32)) < ((c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(28)) as i32))) as i32)) != 0) {
                { let v_ = 30i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            { let v_ = 33i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
            { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            F::Dialog_SetCursor(c, 1i32);
        } else {
            { let v_ = 13i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
            { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            F::Dialog_SetCursor(c, 1i32);
        }
        break 's1;
        }
        F::Dialog_SetChoices(c, 1i32);
        { let v_ = 14i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
        { let _ = F::McCard_Probe(c, 0i32); };
        if (({ let a0_ = (c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32); F::McCard_IsFatalError(c, a0_) }) != 0) {
            { let v_ = 27i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
            { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            { let v_ = 0i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
            break 's1;
        }
        if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); (c.m.r32(a_) as i32) }) > (0i32)) as i32)) != 0) {
            if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) >= (0i32)) as i32)) != 0) {
                break 's1;
            }
            { let v_ = 34i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
            { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            { let v_ = 0i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
        } else {
            if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); (c.m.r32(a_) as i32) }) < (0i32)) as i32)) != 0) {
                { let v_ = 18i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
                F::Dialog_SetCursor(c, 1i32);
            } else {
                { let v_ = F::Dialog_Input(c, 0i32); let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
                { let v_ = 12i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            }
        }
        break 's1;
        }
        F::Dialog_SetChoices(c, 0i32);
        { let v_ = 16i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
        if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) >= (0i32)) as i32)) != 0) {
            if (((((F::McCard_GetInfo(c, 0i32)) == (0i32)) as i32)) != 0) {
                break 's1;
            }
            if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); (c.m.r32(a_) as i32) }) > (0i32)) as i32)) != 0) {
                break 's1;
            }
            if (({ let a0_ = (c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32); F::McCard_IsFatalError(c, a0_) }) != 0) {
                { let v_ = 17i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            }
            break 's1;
        }
        if ((((({ let a1_ = (c.m.r32(sym::gMcFlow)).wrapping_add(235968); F::McCard_CreateSave(c, 0i32, a1_) }) == (0i32)) as i32)) != 0) {
            if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32)) < (0i32)) as i32)) != 0) {
                { let v_ = 17i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            }
        } else {
            { let v_ = 15i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
        }
        break 's1;
        }
        F::Dialog_SetChoices(c, 0i32);
        F::Dialog_Start(c, 1i32);
        { let v_ = 38i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
        }
        { let _ = F::Dialog_Input(c, 0i32); };
        if ((F::Dialog_IsClosed(c)) != 0) {
            { let v_ = 0i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
            F::McFlow_CallDone(c);
        }
        break 's1;
    }
    { let a0_ = { let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(48); (c.m.r32(a_) as i32) }; F::Dialog_SetMsg(c, a0_) };
    F::Dialog_Draw(c, 1i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "McFlow_UpdateSave", &PROBE_N, &[], true); }
    r_
}

pub fn McFlow_UpdateLoad(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "McFlow_UpdateLoad", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
        break 'f;
    }
    's1: {
        let sv1: i32 = { let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); (c.m.r32(a_) as i32) };
        'k14: {
        'k13: {
        'k12: {
        'k11: {
        'k10: {
        'k9: {
        'k8: {
        'k7: {
        'k6: {
        'k5: {
        'k4: {
        'k3: {
        'k2: {
            match sv1 { 35i32 => break 'k2, 2i32 => break 'k3, 4i32 => break 'k4, 6i32 => break 'k5, 19i32 => break 'k6, 20i32 => break 'k7, 21i32 => break 'k8, 29i32 => break 'k9, 27i32 => break 'k10, 28i32 => break 'k11, 22i32 => break 'k12, 37i32 => break 'k13, 38i32 => break 'k14, _ => break 's1, }
        }
        F::Dialog_SetLayout(c, 1i32);
        F::Dialog_SetCursor(c, 1i32);
        F::Dialog_SetTitle(c, 0i32);
        F::Dialog_Start(c, 0i32);
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
        { let v_ = 2i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
        break 's1;
        }
        F::Dialog_SetChoices(c, 1i32);
        { let v_ = 4i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
        if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); (c.m.r32(a_) as i32) }) > (0i32)) as i32)) != 0) {
            if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) >= (0i32)) as i32)) != 0) {
                break 's1;
            }
            { let v_ = 4i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
            { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            { let v_ = 0i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
            F::McCard_ResetStep(c);
        } else {
            if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); (c.m.r32(a_) as i32) }) < (0i32)) as i32)) != 0) {
                F::Dialog_SetCursor(c, 1i32);
                { let v_ = 22i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
            } else {
                { let v_ = F::Dialog_Input(c, 0i32); let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
                { let v_ = 12i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            }
        }
        break 's1;
        }
        F::Dialog_SetChoices(c, 0i32);
        { let v_ = 5i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
        if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) >= (0i32)) as i32)) != 0) {
            if (((((F::McCard_GetInfo(c, 0i32)) == (0i32)) as i32)) != 0) {
                break 's1;
            }
            if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); (c.m.r32(a_) as i32) }) > (0i32)) as i32)) != 0) {
                break 's1;
            }
            if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(12)) as i32)) != (2i32)) as i32)) != 0) {
                { let v_ = 27i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            if (({ let a0_ = (c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32); F::McCard_IsFatalError(c, a0_) }) != 0) {
                { let v_ = 27i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            }
            break 's1;
        }
        if ((((({ let a1_ = (c.m.r32(sym::gMcFlow)).wrapping_add(235968); F::McCard_CheckFiles(c, 0i32, a1_) }) == (0i32)) as i32)) != 0) {
            if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(12)) as i32)) != (2i32)) as i32)) != 0) {
                { let v_ = 27i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            if (({ let a0_ = (c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32); F::McCard_IsFatalError(c, a0_) }) != 0) {
                { let v_ = 27i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32)) >= (0i32)) as i32)) != 0) {
                break 's1;
            }
            { let v_ = 6i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
        } else {
            { let v_ = 19i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
            { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
        }
        break 's1;
        }
        F::Dialog_SetChoices(c, 0i32);
        { let v_ = 5i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
        if (((((F::McCard_FindSystemSave(c, 0i32)) == (0i32)) as i32)) != 0) {
            if (({ let a0_ = (c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32); F::McCard_IsFatalError(c, a0_) }) != 0) {
                { let v_ = 27i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            }
            break 's1;
        }
        if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32)) == (1i32)) as i32)) != 0) {
            { let v_ = 29i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
            { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
        } else {
            { let v_ = 28i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
            { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
        }
        break 's1;
        }
        F::Dialog_SetChoices(c, 0i32);
        { let v_ = 20i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
        if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) >= (0i32)) as i32)) != 0) {
            if (((((F::McCard_GetInfo(c, 0i32)) == (0i32)) as i32)) != 0) {
                break 's1;
            }
            if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); (c.m.r32(a_) as i32) }) > (0i32)) as i32)) != 0) {
                break 's1;
            }
            if (({ let a0_ = (c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32); F::McCard_IsFatalError(c, a0_) }) != 0) {
                { let v_ = 21i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            }
            break 's1;
        }
        if ((((({ let a1_ = (c.m.r32(sym::gMcFlow)).wrapping_add(56); let a3_ = (c.m.r32(sym::gMcFlow)).wrapping_add(235968); F::McCard_ReadFile(c, 0i32, a1_, 16384i32, a3_) }) == (0i32)) as i32)) != 0) {
            if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32)) < (0i32)) as i32)) != 0) {
                { let v_ = 21i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            }
        } else {
            if (((((c.m.r32((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))) as i32)) & (1i32))) != 0) {
                { let v_ = 29i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            if ((((({ let a0_ = (c.m.r32(sym::gMcFlow)).wrapping_add(56); F::McCard_Checksum(c, a0_, 16384i32, 0i32) }) != (0i32)) as i32)) != 0) {
                { let v_ = 29i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            { let s_ = (c.m.r32(sym::gMcFlow)).wrapping_add(56); let d_ = c.m.r32(sym::gSaveData); rt::memcpy(c, d_, s_, 16384); };
            F::SndOpt_Apply(c);
            F::Progress_ClearSession(c);
            { let v_ = 20i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
            { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
        }
        break 's1;
        }
        F::Dialog_SetChoices(c, 0i32);
        { let v_ = 21i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
            if ((F::Dialog_IsOpen(c)) != 0) {
                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                { let r_ = 8i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(12); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 37i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
        }
        if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) >= (0i32)) as i32)) != 0) {
            break 's1;
        }
        { let r_ = 8i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(12); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        { let v_ = 37i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
        break 's1;
        }
        F::Dialog_SetChoices(c, 0i32);
        { let v_ = 22i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
            if ((F::Dialog_IsOpen(c)) != 0) {
                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                F::Dialog_SetCursor(c, 1i32);
                { let v_ = 22i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
            }
        }
        break 's1;
        }
        F::Dialog_SetChoices(c, 0i32);
        { let v_ = 30i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
            if ((F::Dialog_IsOpen(c)) != 0) {
                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                F::Dialog_SetCursor(c, 1i32);
                { let v_ = 22i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
            }
        }
        break 's1;
        }
        F::Dialog_SetChoices(c, 0i32);
        { let v_ = 28i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
            if ((F::Dialog_IsOpen(c)) != 0) {
                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                F::Dialog_SetCursor(c, 1i32);
                { let v_ = 22i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
            }
        }
        break 's1;
        }
        F::Dialog_SetChoices(c, 0i32);
        { let v_ = 29i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
            if ((F::Dialog_IsOpen(c)) != 0) {
                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                F::Dialog_SetCursor(c, 1i32);
                { let v_ = 22i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
            }
        }
        break 's1;
        }
        F::Dialog_SetChoices(c, 1i32);
        { let v_ = 23i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
        if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); (c.m.r32(a_) as i32) }) > (0i32)) as i32)) != 0) {
            if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) >= (0i32)) as i32)) != 0) {
                break 's1;
            }
            { let v_ = 37i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
            { let v_ = 0i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
        } else {
            if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); (c.m.r32(a_) as i32) }) < ((1i32).wrapping_neg())) as i32)) != 0) {
                { let v_ = 2i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
                F::Dialog_SetCursor(c, 1i32);
                F::McCard_ResetStep(c);
            } else {
                { let v_ = F::Dialog_Input(c, 0i32); let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
                { let v_ = 12i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            }
        }
        break 's1;
        }
        F::Dialog_SetChoices(c, 0i32);
        F::Dialog_Start(c, 1i32);
        { let v_ = 38i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
        }
        { let _ = F::Dialog_Input(c, 0i32); };
        if ((F::Dialog_IsClosed(c)) != 0) {
            { let v_ = 0i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
            F::McFlow_CallDone(c);
        }
        break 's1;
    }
    { let a0_ = { let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(48); (c.m.r32(a_) as i32) }; F::Dialog_SetMsg(c, a0_) };
    F::Dialog_Draw(c, 1i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "McFlow_UpdateLoad", &PROBE_N, &[], true); }
    r_
}

pub fn McFlow_UpdateBootLoad(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "McFlow_UpdateBootLoad", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
        break 'f;
    }
    's1: {
        let sv1: i32 = { let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); (c.m.r32(a_) as i32) };
        'k14: {
        'k13: {
        'k12: {
        'k11: {
        'k10: {
        'k9: {
        'k8: {
        'k7: {
        'k6: {
        'k5: {
        'k4: {
        'k3: {
        'k2: {
            match sv1 { 35i32 => break 'k2, 5i32 => break 'k3, 4i32 => break 'k4, 7i32 => break 'k5, 8i32 => break 'k6, 6i32 => break 'k7, 19i32 => break 'k8, 20i32 => break 'k9, 21i32 => break 'k10, 29i32 => break 'k11, 22i32 => break 'k12, 37i32 => break 'k13, 38i32 => break 'k14, _ => break 's1, }
        }
        F::Dialog_SetLayout(c, 1i32);
        F::Dialog_SetCursor(c, 1i32);
        F::Dialog_SetTitle(c, 0i32);
        F::Dialog_Start(c, 0i32);
        { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
        { let v_ = 5i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
        break 's1;
        }
        F::Dialog_SetChoices(c, 0i32);
        { let v_ = 5i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
        if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) >= (0i32)) as i32)) != 0) {
            if (((((F::McCard_GetInfo(c, 0i32)) == (0i32)) as i32)) != 0) {
                break 's1;
            }
            if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); (c.m.r32(a_) as i32) }) > (0i32)) as i32)) != 0) {
                break 's1;
            }
            if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(12)) as i32)) != (2i32)) as i32)) != 0) {
                { let v_ = 7i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                F::Dialog_SetCursor(c, 1i32);
                break 's1;
            }
            if (({ let a0_ = (c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32); F::McCard_IsFatalError(c, a0_) }) != 0) {
                { let v_ = 7i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                F::Dialog_SetCursor(c, 1i32);
            }
            break 's1;
        }
        if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(20)) as i32)) == (0i32)) as i32)) != 0) {
            { let v_ = 37i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
            break 's1;
        }
        if ((((({ let a1_ = (c.m.r32(sym::gMcFlow)).wrapping_add(235968); F::McCard_CheckFiles(c, 0i32, a1_) }) == (0i32)) as i32)) != 0) {
            if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(12)) as i32)) != (2i32)) as i32)) != 0) {
                { let v_ = 7i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                F::Dialog_SetCursor(c, 1i32);
                break 's1;
            }
            if (({ let a0_ = (c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32); F::McCard_IsFatalError(c, a0_) }) != 0) {
                { let v_ = 7i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                F::Dialog_SetCursor(c, 1i32);
                break 's1;
            }
            if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32)) >= (0i32)) as i32)) != 0) {
                break 's1;
            }
            { let v_ = 6i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
        } else {
            { let v_ = 19i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
            { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
        }
        break 's1;
        }
        F::Dialog_SetChoices(c, 0i32);
        { let v_ = 5i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
        if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) >= (0i32)) as i32)) != 0) {
            if (((((F::McCard_GetInfo(c, 0i32)) == (0i32)) as i32)) != 0) {
                break 's1;
            }
            if (({ let a0_ = (c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32); F::McCard_IsFatalError(c, a0_) }) != 0) {
                { let v_ = 7i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                F::Dialog_SetCursor(c, 1i32);
            }
            break 's1;
        }
        if ((((({ let a1_ = (c.m.r32(sym::gMcFlow)).wrapping_add(235968); F::McCard_CheckFiles(c, 0i32, a1_) }) == (0i32)) as i32)) != 0) {
            if (({ let a0_ = (c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32); F::McCard_IsFatalError(c, a0_) }) != 0) {
                { let v_ = 7i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                F::Dialog_SetCursor(c, 1i32);
                break 's1;
            }
            if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32)) >= (0i32)) as i32)) != 0) {
                break 's1;
            }
            { let v_ = 6i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
        } else {
            { let v_ = 19i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
            { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
        }
        break 's1;
        }
        F::Dialog_SetChoices(c, 1i32);
        { let v_ = 6i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
        if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); (c.m.r32(a_) as i32) }) > (0i32)) as i32)) != 0) {
            if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) >= (0i32)) as i32)) != 0) {
                break 's1;
            }
            { let v_ = 37i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
            { let r_ = 4i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(20); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            break 's1;
        } else {
            if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); (c.m.r32(a_) as i32) }) < (0i32)) as i32)) != 0) {
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
                F::Dialog_SetCursor(c, 1i32);
            } else {
                { let v_ = F::Dialog_Input(c, 0i32); let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
                { let v_ = 12i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            }
        }
        if (((((F::McCard_GetInfo(c, 0i32)) == (0i32)) as i32)) != 0) {
            break 's1;
        }
        F::McCard_ResetStep(c);
        if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(12)) as i32)) == (2i32)) as i32)) != 0) {
            if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(20)) as i32)) == (0i32)) as i32)) != 0) {
                { let v_ = 37i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32)) != ((1i32).wrapping_neg())) as i32)) != 0) {
                break 's1;
            }
            { let v_ = 4i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
            { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
        } else {
            { let v_ = 1i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
            { let v_ = 0i32; let a_ = ((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
            { let v_ = 0i32; let a_ = ((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
        }
        break 's1;
        }
        F::Dialog_SetChoices(c, 1i32);
        { let v_ = 7i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
        if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); (c.m.r32(a_) as i32) }) > (0i32)) as i32)) != 0) {
            if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) >= (0i32)) as i32)) != 0) {
                break 's1;
            }
            { let v_ = 37i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
            break 's1;
        } else {
            if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); (c.m.r32(a_) as i32) }) < (0i32)) as i32)) != 0) {
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
                F::Dialog_SetCursor(c, 1i32);
            } else {
                { let v_ = F::Dialog_Input(c, 0i32); let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
                { let v_ = 12i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            }
        }
        if (((((F::McCard_GetInfo(c, 0i32)) == (0i32)) as i32)) != 0) {
            break 's1;
        }
        F::McCard_ResetStep(c);
        if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(20); (c.m.r32(a_) as i32) }) == (1i32)) as i32)) != 0) {
            { let v_ = 7i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
            F::Dialog_SetCursor(c, 1i32);
            break 's1;
        }
        if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(12)) as i32)) == (2i32)) as i32)) != 0) {
            if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(20)) as i32)) == (0i32)) as i32)) != 0) {
                { let v_ = 37i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32)) != ((1i32).wrapping_neg())) as i32)) != 0) {
                break 's1;
            }
            { let v_ = 4i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
            { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
        } else {
            { let v_ = 1i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
            { let v_ = 0i32; let a_ = ((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
            { let v_ = 0i32; let a_ = ((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
        }
        break 's1;
        }
        F::Dialog_SetChoices(c, 0i32);
        { let v_ = 5i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
        if (((((F::McCard_FindSystemSave(c, 0i32)) == (0i32)) as i32)) != 0) {
            if (({ let a0_ = (c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32); F::McCard_IsFatalError(c, a0_) }) != 0) {
                { let v_ = 7i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                F::Dialog_SetCursor(c, 1i32);
            }
            break 's1;
        }
        if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32)) == (1i32)) as i32)) != 0) {
            { let v_ = 29i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
            { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(16)) as i32)) < (61i32)) as i32)) != 0) {
                { let v_ = 4i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
                { let v_ = 8i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                F::Dialog_SetCursor(c, 1i32);
            }
        } else {
            { let v_ = 7i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
            if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(16)) as i32)) >= (61i32)) as i32)) != 0) {
                { let v_ = 37i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
            } else {
                { let v_ = 4i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
                { let v_ = 8i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                F::Dialog_SetCursor(c, 1i32);
            }
        }
        break 's1;
        }
        F::Dialog_SetChoices(c, 0i32);
        { let v_ = 20i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
        if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) >= (0i32)) as i32)) != 0) {
            if (((((F::McCard_GetInfo(c, 0i32)) == (0i32)) as i32)) != 0) {
                break 's1;
            }
            if (({ let a0_ = (c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32); F::McCard_IsFatalError(c, a0_) }) != 0) {
                { let v_ = 21i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            }
            break 's1;
        }
        if ((((({ let a1_ = (c.m.r32(sym::gMcFlow)).wrapping_add(56); let a3_ = (c.m.r32(sym::gMcFlow)).wrapping_add(235968); F::McCard_ReadFile(c, 0i32, a1_, 16384i32, a3_) }) == (0i32)) as i32)) != 0) {
            if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32)) < (0i32)) as i32)) != 0) {
                { let v_ = 21i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            }
        } else {
            if (((((c.m.r32((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))) as i32)) & (1i32))) != 0) {
                { let v_ = 29i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            if ((((({ let a0_ = (c.m.r32(sym::gMcFlow)).wrapping_add(56); F::McCard_Checksum(c, a0_, 16384i32, 0i32) }) != (0i32)) as i32)) != 0) {
                { let v_ = 29i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            { let s_ = (c.m.r32(sym::gMcFlow)).wrapping_add(56); let d_ = c.m.r32(sym::gSaveData); rt::memcpy(c, d_, s_, 16384); };
            F::SndOpt_Apply(c);
            { let v_ = 20i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
            { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
        }
        break 's1;
        }
        F::Dialog_SetChoices(c, 0i32);
        { let v_ = 21i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
        { let r_ = 8i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(20); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) >= (0i32)) as i32)) != 0) {
            break 's1;
        }
        { let v_ = 37i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
        break 's1;
        }
        F::Dialog_SetChoices(c, 0i32);
        { let v_ = 22i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
            if ((F::Dialog_IsOpen(c)) != 0) {
                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                { let v_ = 5i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            }
        }
        break 's1;
        }
        F::Dialog_SetChoices(c, 0i32);
        { let v_ = 30i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
            if ((F::Dialog_IsOpen(c)) != 0) {
                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                F::Dialog_SetCursor(c, 1i32);
                { let v_ = 22i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
            }
        }
        break 's1;
        }
        F::Dialog_SetChoices(c, 1i32);
        { let v_ = 23i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
        if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); (c.m.r32(a_) as i32) }) > (0i32)) as i32)) != 0) {
            if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) >= (0i32)) as i32)) != 0) {
                break 's1;
            }
            { let v_ = 37i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
            { let v_ = 0i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
        } else {
            if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); (c.m.r32(a_) as i32) }) < ((1i32).wrapping_neg())) as i32)) != 0) {
                { let v_ = 5i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
                F::Dialog_SetCursor(c, 1i32);
                F::McCard_ResetStep(c);
            } else {
                { let v_ = F::Dialog_Input(c, 0i32); let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
                { let v_ = 12i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            }
        }
        break 's1;
        }
        F::Dialog_SetChoices(c, 0i32);
        F::Dialog_Start(c, 1i32);
        { let v_ = 38i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
        }
        { let _ = F::Dialog_Input(c, 0i32); };
        if ((F::Dialog_IsClosed(c)) != 0) {
            { let v_ = 0i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
            { let r_ = 8i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(12); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            F::McFlow_CallDone(c);
        }
        break 's1;
    }
    { let a0_ = { let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(48); (c.m.r32(a_) as i32) }; F::Dialog_SetMsg(c, a0_) };
    F::Dialog_Draw(c, 1i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "McFlow_UpdateBootLoad", &PROBE_N, &[], true); }
    r_
}

pub fn McFlow_UpdateNewSave(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "McFlow_UpdateNewSave", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
        break 'f;
    }
    's1: {
        let sv1: i32 = { let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); (c.m.r32(a_) as i32) };
        'k23: {
        'k22: {
        'k21: {
        'k20: {
        'k19: {
        'k18: {
        'k17: {
        'k16: {
        'k15: {
        'k14: {
        'k13: {
        'k12: {
        'k11: {
        'k10: {
        'k9: {
        'k8: {
        'k7: {
        'k6: {
        'k5: {
        'k4: {
        'k3: {
        'k2: {
            match sv1 { 35i32 => break 'k2, 1i32 => break 'k3, 9i32 => break 'k4, 10i32 => break 'k5, 3i32 => break 'k6, 23i32 => break 'k7, 24i32 => break 'k8, 25i32 => break 'k9, 26i32 => break 'k10, 13i32 => break 'k11, 11i32 => break 'k12, 15i32 => break 'k13, 6i32 => break 'k14, 16i32 => break 'k15, 17i32 => break 'k16, 30i32 => break 'k17, 27i32 => break 'k18, 31i32 => break 'k19, 33i32 => break 'k20, 34i32 => break 'k21, 37i32 => break 'k22, 38i32 => break 'k23, _ => break 's1, }
        }
        F::Dialog_SetLayout(c, 1i32);
        F::Dialog_SetCursor(c, 1i32);
        F::Dialog_SetTitle(c, 0i32);
        F::Dialog_Start(c, 0i32);
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
        if (((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(20); (c.m.r32(a_) as i32) }) & (8i32))) != 0) {
            { let v_ = 1i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
        } else {
            { let v_ = 10i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
        }
        break 's1;
        }
        F::Dialog_SetChoices(c, 1i32);
        { let v_ = 3i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
        if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); (c.m.r32(a_) as i32) }) > (0i32)) as i32)) != 0) {
            if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) >= (0i32)) as i32)) != 0) {
                break 's1;
            }
            { let v_ = 10i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
            { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            { let v_ = 0i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
            F::Dialog_SetCursor(c, 1i32);
        } else {
            if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); (c.m.r32(a_) as i32) }) < (0i32)) as i32)) != 0) {
                { let v_ = 37i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
            } else {
                { let v_ = F::Dialog_Input(c, 0i32); let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
                { let v_ = 12i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            }
        }
        break 's1;
        }
        F::Dialog_SetChoices(c, 1i32);
        { let v_ = 8i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
        if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); (c.m.r32(a_) as i32) }) > (0i32)) as i32)) != 0) {
            if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) >= (0i32)) as i32)) != 0) {
                break 's1;
            }
            { let v_ = 37i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
            { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            { let v_ = 0i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
            { let r_ = 8i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(12); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            { let a0_ = c.m.r32(sym::gSaveData); F::Save_SetDefaults(c, a0_) };
            { let _ = { let a0_ = c.m.r32(sym::gSaveData); F::McCard_Checksum(c, a0_, 16384i32, 1i32) }; };
            F::SndOpt_Apply(c);
            F::Progress_ClearSession(c);
        } else {
            if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); (c.m.r32(a_) as i32) }) < (0i32)) as i32)) != 0) {
                { let v_ = 37i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
            } else {
                { let v_ = F::Dialog_Input(c, 0i32); let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
                { let v_ = 12i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            }
        }
        break 's1;
        }
        F::Dialog_SetChoices(c, 1i32);
        { let v_ = 9i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
        if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); (c.m.r32(a_) as i32) }) > (0i32)) as i32)) != 0) {
            if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) >= (0i32)) as i32)) != 0) {
                break 's1;
            }
            { let v_ = 3i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
            { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            { let v_ = 0i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
            F::McCard_ResetStep(c);
        } else {
            if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); (c.m.r32(a_) as i32) }) < (0i32)) as i32)) != 0) {
                { let v_ = 9i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
                F::Dialog_SetCursor(c, 1i32);
            } else {
                { let v_ = F::Dialog_Input(c, 0i32); let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
                { let v_ = 12i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            }
        }
        break 's1;
        }
        F::Dialog_SetChoices(c, 0i32);
        { let v_ = 5i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
        if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) >= (0i32)) as i32)) != 0) {
            if (((((F::McCard_GetInfo(c, 0i32)) == (0i32)) as i32)) != 0) {
                break 's1;
            }
            if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); (c.m.r32(a_) as i32) }) > (0i32)) as i32)) != 0) {
                break 's1;
            }
            if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(12)) as i32)) != (2i32)) as i32)) != 0) {
                { let v_ = 27i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            if (({ let a0_ = (c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32); F::McCard_IsFatalError(c, a0_) }) != 0) {
                { let v_ = 27i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(20)) as i32)) == (0i32)) as i32)) != 0) {
                { let v_ = 23i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                F::Dialog_SetCursor(c, 1i32);
            }
            break 's1;
        }
        if ((((({ let a1_ = (c.m.r32(sym::gMcFlow)).wrapping_add(235968); F::McCard_CheckFiles(c, 0i32, a1_) }) == (0i32)) as i32)) != 0) {
            if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(12)) as i32)) != (2i32)) as i32)) != 0) {
                { let v_ = 27i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(20)) as i32)) == (0i32)) as i32)) != 0) {
                { let v_ = 23i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                F::Dialog_SetCursor(c, 1i32);
                break 's1;
            }
            if (({ let a0_ = (c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32); F::McCard_IsFatalError(c, a0_) }) != 0) {
                { let v_ = 27i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32)) >= (0i32)) as i32)) != 0) {
                break 's1;
            }
            if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(16)) as i32)) < (61i32)) as i32)) != 0) {
                { let v_ = 30i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            { let v_ = 6i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
        } else {
            F::McCard_ResetStep(c);
            { let v_ = 31i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
        }
        break 's1;
        }
        F::Dialog_SetChoices(c, 1i32);
        { let v_ = 24i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
        { let _ = F::McCard_Probe(c, 0i32); };
        if (({ let a0_ = (c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32); F::McCard_IsFatalError(c, a0_) }) != 0) {
            F::Dialog_SetChoices(c, 0i32);
            { let v_ = 28i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
            { let v_ = 27i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
            { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            break 's1;
        }
        if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); (c.m.r32(a_) as i32) }) > (0i32)) as i32)) != 0) {
            if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) >= (0i32)) as i32)) != 0) {
                break 's1;
            }
            { let v_ = 24i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
            { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            { let v_ = 0i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
        } else {
            if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); (c.m.r32(a_) as i32) }) < (0i32)) as i32)) != 0) {
                F::Dialog_SetCursor(c, 1i32);
                { let v_ = 9i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
            } else {
                { let v_ = F::Dialog_Input(c, 0i32); let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
                { let v_ = 12i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            }
        }
        break 's1;
        }
        F::Dialog_SetChoices(c, 0i32);
        { let v_ = 25i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
        if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) >= (0i32)) as i32)) != 0) {
            if (((((F::McCard_GetInfo(c, 0i32)) == (0i32)) as i32)) != 0) {
                break 's1;
            }
            if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); (c.m.r32(a_) as i32) }) > (0i32)) as i32)) != 0) {
                break 's1;
            }
            if (({ let a0_ = (c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32); F::McCard_IsFatalError(c, a0_) }) != 0) {
                { let v_ = 26i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            }
            break 's1;
        }
        if (((((F::McCard_Format(c, 0i32)) == (0i32)) as i32)) != 0) {
            if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32)) < (0i32)) as i32)) != 0) {
                { let v_ = 26i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            }
        } else {
            { let v_ = 25i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
            { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
        }
        break 's1;
        }
        F::Dialog_SetChoices(c, 0i32);
        { let _ = F::McCard_Probe(c, 0i32); };
        if (({ let a0_ = (c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32); F::McCard_IsFatalError(c, a0_) }) != 0) {
            { let v_ = 27i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
            { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            break 's1;
        }
        if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) >= (0i32)) as i32)) != 0) {
            { let v_ = 26i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
            break 's1;
        }
        { let v_ = 11i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
        { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
        break 's1;
        }
        F::Dialog_SetChoices(c, 0i32);
        { let v_ = 27i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
            if ((F::Dialog_IsOpen(c)) != 0) {
                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                F::Dialog_SetCursor(c, 1i32);
                { let v_ = 9i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
            }
        }
        break 's1;
        }
        F::Dialog_SetChoices(c, 1i32);
        { let v_ = 14i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
        { let _ = F::McCard_Probe(c, 0i32); };
        if (({ let a0_ = (c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32); F::McCard_IsFatalError(c, a0_) }) != 0) {
            { let v_ = 27i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
            { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            break 's1;
        }
        if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); (c.m.r32(a_) as i32) }) > (0i32)) as i32)) != 0) {
            if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) >= (0i32)) as i32)) != 0) {
                break 's1;
            }
            { let v_ = 15i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
            { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            { let v_ = 0i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
            { let a0_ = c.m.r32(sym::gSaveData); F::Save_SetDefaults(c, a0_) };
            { let _ = { let a0_ = c.m.r32(sym::gSaveData); F::McCard_Checksum(c, a0_, 16384i32, 1i32) }; };
            F::SndOpt_Apply(c);
            F::Progress_ClearSession(c);
        } else {
            if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); (c.m.r32(a_) as i32) }) < (0i32)) as i32)) != 0) {
                F::Dialog_SetCursor(c, 1i32);
                { let v_ = 9i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
            } else {
                { let v_ = F::Dialog_Input(c, 0i32); let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
                { let v_ = 12i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            }
        }
        break 's1;
        }
        F::Dialog_SetChoices(c, 0i32);
        { let v_ = 10i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
        if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) >= (0i32)) as i32)) != 0) {
            if (((((F::McCard_GetInfo(c, 0i32)) == (0i32)) as i32)) != 0) {
                break 's1;
            }
            if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); (c.m.r32(a_) as i32) }) > (0i32)) as i32)) != 0) {
                break 's1;
            }
            if (({ let a0_ = (c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32); F::McCard_IsFatalError(c, a0_) }) != 0) {
                { let v_ = 17i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            }
            break 's1;
        }
        if ((((({ let a1_ = (c.m.r32(sym::gMcFlow)).wrapping_add(235968); F::McCard_CreateSave(c, 0i32, a1_) }) == (0i32)) as i32)) != 0) {
            if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32)) < (0i32)) as i32)) != 0) {
                { let v_ = 17i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            }
        } else {
            { let a0_ = c.m.r32(sym::gSaveData); F::Save_SetDefaults(c, a0_) };
            { let _ = { let a0_ = c.m.r32(sym::gSaveData); F::McCard_Checksum(c, a0_, 16384i32, 1i32) }; };
            F::SndOpt_Apply(c);
            F::Progress_ClearSession(c);
            { let v_ = 15i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
        }
        break 's1;
        }
        F::Dialog_SetChoices(c, 0i32);
        { let v_ = 16i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
        if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) >= (0i32)) as i32)) != 0) {
            if (((((F::McCard_GetInfo(c, 0i32)) == (0i32)) as i32)) != 0) {
                break 's1;
            }
            if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); (c.m.r32(a_) as i32) }) > (0i32)) as i32)) != 0) {
                break 's1;
            }
            if (({ let a0_ = (c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32); F::McCard_IsFatalError(c, a0_) }) != 0) {
                { let v_ = 17i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            }
            break 's1;
        }
        if ((((({ let a1_ = c.m.r32(sym::gSaveData); let a3_ = (c.m.r32(sym::gMcFlow)).wrapping_add(235968); F::McCard_WriteSave(c, 0i32, a1_, 16384i32, a3_) }) == (0i32)) as i32)) != 0) {
            if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32)) < (0i32)) as i32)) != 0) {
                { let v_ = 17i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            }
        } else {
            { let v_ = 16i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
            { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
        }
        break 's1;
        }
        F::Dialog_SetChoices(c, 0i32);
        { let v_ = 5i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
        if (((((F::McCard_FindSystemSave(c, 0i32)) == (0i32)) as i32)) != 0) {
            if (({ let a0_ = (c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32); F::McCard_IsFatalError(c, a0_) }) != 0) {
                { let v_ = 27i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            }
            break 's1;
        }
        if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32)) == (1i32)) as i32)) != 0) {
            { let v_ = 11i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
            { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
        } else {
            { let v_ = 11i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
            { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
        }
        break 's1;
        }
        F::Dialog_SetChoices(c, 0i32);
        { let v_ = 17i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
            if ((F::Dialog_IsOpen(c)) != 0) {
                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
            }
        }
        if ((((((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) <= (0i32)) as i32)) != 0) || ((((((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) && ((F::Dialog_IsOpen(c)) != 0)) as i32)) != 0)) as i32)) != 0) {
            { let v_ = 37i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
            { let r_ = 8i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(12); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        }
        break 's1;
        }
        F::Dialog_SetChoices(c, 0i32);
        { let v_ = 18i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
            if ((F::Dialog_IsOpen(c)) != 0) {
                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                F::Dialog_SetCursor(c, 1i32);
                { let v_ = 9i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
            }
        }
        break 's1;
        }
        F::Dialog_SetChoices(c, 0i32);
        { let v_ = 31i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
            if ((F::Dialog_IsOpen(c)) != 0) {
                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                F::Dialog_SetCursor(c, 1i32);
                { let v_ = 9i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
            }
        }
        break 's1;
        }
        F::Dialog_SetChoices(c, 0i32);
        { let v_ = 28i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
            if ((F::Dialog_IsOpen(c)) != 0) {
                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                F::Dialog_SetCursor(c, 1i32);
                { let v_ = 9i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
            }
        }
        break 's1;
        }
        F::Dialog_SetChoices(c, 0i32);
        { let v_ = 5i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
        if ((((({ let a1_ = (c.m.r32(sym::gMcFlow)).wrapping_add(235968); F::McCard_CalcNeed(c, 0i32, a1_, 0i32) }) == (0i32)) as i32)) != 0) {
            if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(12)) as i32)) != (2i32)) as i32)) != 0) {
                { let v_ = 27i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(20)) as i32)) == (0i32)) as i32)) != 0) {
                { let v_ = 23i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                F::Dialog_SetCursor(c, 1i32);
                break 's1;
            }
            if (({ let a0_ = (c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32); F::McCard_IsFatalError(c, a0_) }) != 0) {
                { let v_ = 27i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            }
            break 's1;
        }
        if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(28)) as i32)) > (0i32)) as i32)) != 0) {
            if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(16)) as i32)) < ((c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(28)) as i32))) as i32)) != 0) {
                { let v_ = 30i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            { let v_ = 33i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
            { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            F::Dialog_SetCursor(c, 1i32);
        } else {
            { let v_ = 13i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
            { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            F::Dialog_SetCursor(c, 1i32);
        }
        break 's1;
        }
        F::Dialog_SetChoices(c, 1i32);
        { let v_ = 14i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
        { let _ = F::McCard_Probe(c, 0i32); };
        if (({ let a0_ = (c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32); F::McCard_IsFatalError(c, a0_) }) != 0) {
            { let v_ = 27i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
            { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            { let v_ = 0i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
            break 's1;
        }
        if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); (c.m.r32(a_) as i32) }) > (0i32)) as i32)) != 0) {
            if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) >= (0i32)) as i32)) != 0) {
                break 's1;
            }
            { let v_ = 34i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
            { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            { let v_ = 0i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
        } else {
            if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); (c.m.r32(a_) as i32) }) < (0i32)) as i32)) != 0) {
                { let v_ = 9i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
                F::Dialog_SetCursor(c, 1i32);
            } else {
                { let v_ = F::Dialog_Input(c, 0i32); let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
                { let v_ = 12i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            }
        }
        break 's1;
        }
        F::Dialog_SetChoices(c, 0i32);
        { let v_ = 16i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
        if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) >= (0i32)) as i32)) != 0) {
            if (((((F::McCard_GetInfo(c, 0i32)) == (0i32)) as i32)) != 0) {
                break 's1;
            }
            if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); (c.m.r32(a_) as i32) }) > (0i32)) as i32)) != 0) {
                break 's1;
            }
            if (({ let a0_ = (c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32); F::McCard_IsFatalError(c, a0_) }) != 0) {
                { let v_ = 17i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            }
            break 's1;
        }
        if ((((({ let a1_ = (c.m.r32(sym::gMcFlow)).wrapping_add(235968); F::McCard_CreateSave(c, 0i32, a1_) }) == (0i32)) as i32)) != 0) {
            if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32)) < (0i32)) as i32)) != 0) {
                { let v_ = 17i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            }
        } else {
            { let a0_ = c.m.r32(sym::gSaveData); F::Save_SetDefaults(c, a0_) };
            { let _ = { let a0_ = c.m.r32(sym::gSaveData); F::McCard_Checksum(c, a0_, 16384i32, 1i32) }; };
            F::SndOpt_Apply(c);
            F::Progress_ClearSession(c);
            { let v_ = 15i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
        }
        break 's1;
        }
        F::Dialog_SetChoices(c, 0i32);
        F::Dialog_Start(c, 1i32);
        { let v_ = 38i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
        }
        { let _ = F::Dialog_Input(c, 0i32); };
        if ((F::Dialog_IsClosed(c)) != 0) {
            { let v_ = 0i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
            F::McFlow_CallDone(c);
        }
        break 's1;
    }
    { let a0_ = { let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(48); (c.m.r32(a_) as i32) }; F::Dialog_SetMsg(c, a0_) };
    F::Dialog_Draw(c, 1i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "McFlow_UpdateNewSave", &PROBE_N, &[], true); }
    r_
}

pub fn McFlow_UpdateReplayScan(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "McFlow_UpdateReplayScan", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
        break 'f;
    }
    's1: {
        let sv1: i32 = { let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); (c.m.r32(a_) as i32) };
        'k15: {
        'k14: {
        'k13: {
        'k12: {
        'k11: {
        'k10: {
        'k9: {
        'k8: {
        'k7: {
        'k6: {
        'k5: {
        'k4: {
        'k3: {
        'k2: {
            match sv1 { 35i32 => break 'k2, 36i32 => break 'k3, 40i32 => break 'k4, 39i32 => break 'k5, 4i32 => break 'k6, 6i32 => break 'k7, 19i32 => break 'k8, 29i32 => break 'k9, 27i32 => break 'k10, 28i32 => break 'k11, 18i32 => break 'k12, 22i32 => break 'k13, 37i32 => break 'k14, 38i32 => break 'k15, _ => break 's1, }
        }
        F::Dialog_SetLayout(c, 1i32);
        F::Dialog_SetCursor(c, 1i32);
        F::Dialog_SetTitle(c, 0i32);
        F::Dialog_Start(c, 0i32);
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(237452); c.m.w32(a_, (v_) as u32); };
        { let v_ = { let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(28); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
        break 's1;
        }
        F::Dialog_SetLayout(c, 1i32);
        F::Dialog_SetCursor(c, 1i32);
        F::Dialog_SetTitle(c, 0i32);
        F::Dialog_Start(c, 2i32);
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(237452); c.m.w32(a_, (v_) as u32); };
        { let v_ = { let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(28); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
        break 's1;
        }
        { let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(237452); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(237452); (c.m.r32(a_) as i32) }) >= (7i32)) as i32)) != 0) {
            { let r_ = 8i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(12); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            { let v_ = 37i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
            break 's1;
        }
        { let v_ = 39i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
        }
        if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(237452); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
            { let r_ = (!(112i32)); let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(12); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
        }
        { let a1_ = { let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(237452); (c.m.r32(a_) as i32) }; F::McFlow_SetupRequest(c, 1i32, a1_) };
        { let v_ = 4i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
        { let v_ = (if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(237452); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) { 12i32 } else { 120i32 }); let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
        F::McCard_ResetStep(c);
        }
        F::Dialog_SetChoices(c, 0i32);
        { let v_ = 40i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
        if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) >= (0i32)) as i32)) != 0) {
            if (((((F::McCard_GetInfo(c, 0i32)) == (0i32)) as i32)) != 0) {
                break 's1;
            }
            if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); (c.m.r32(a_) as i32) }) > (0i32)) as i32)) != 0) {
                break 's1;
            }
            if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(12)) as i32)) != (2i32)) as i32)) != 0) {
                { let v_ = 27i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            if (({ let a0_ = (c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32); F::McCard_IsFatalError(c, a0_) }) != 0) {
                { let v_ = 27i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            }
            break 's1;
        }
        if ((((({ let a1_ = (c.m.r32(sym::gMcFlow)).wrapping_add(235968); F::McCard_CheckFiles(c, 0i32, a1_) }) == (0i32)) as i32)) != 0) {
            if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(12)) as i32)) != (2i32)) as i32)) != 0) {
                { let v_ = 27i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            if (({ let a0_ = (c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32); F::McCard_IsFatalError(c, a0_) }) != 0) {
                { let v_ = 27i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32)) >= (0i32)) as i32)) != 0) {
                break 's1;
            }
            { let v_ = 6i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
        } else {
            { let v_ = 19i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
            { let v_ = 12i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
        }
        break 's1;
        }
        F::Dialog_SetChoices(c, 0i32);
        { let v_ = 40i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
        if ((((({ let a1_ = (c.m.r32(sym::gMcFlow)).wrapping_add(235968); F::McCard_FindSave(c, 0i32, a1_) }) == (0i32)) as i32)) != 0) {
            if (({ let a0_ = (c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32); F::McCard_IsFatalError(c, a0_) }) != 0) {
                { let v_ = 27i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            }
            break 's1;
        }
        if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32)) == (1i32)) as i32)) != 0) {
            { let a1_ = { let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(237452); (c.m.r32(a_) as i32) }; F::McFlow_ClearSlotInfo(c, 1i32, a1_) };
            { let r_ = 32i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(12); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            { let v_ = 40i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
        } else {
            { let r_ = 64i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(12); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            { let v_ = 40i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
        }
        break 's1;
        }
        F::Dialog_SetChoices(c, 0i32);
        { let v_ = 40i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
        if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) >= (0i32)) as i32)) != 0) {
            if (((((F::McCard_GetInfo(c, 0i32)) == (0i32)) as i32)) != 0) {
                break 's1;
            }
            if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); (c.m.r32(a_) as i32) }) > (0i32)) as i32)) != 0) {
                break 's1;
            }
            if (({ let a0_ = (c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32); F::McCard_IsFatalError(c, a0_) }) != 0) {
                { let v_ = 40i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
            }
            break 's1;
        }
        if ((((({ let a1_ = ((c.m.r32(sym::gMcFlow)).wrapping_add(235576)).wrapping_add(((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(237452); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(56)); let a3_ = (c.m.r32(sym::gMcFlow)).wrapping_add(235968); F::McCard_ReadFile(c, 0i32, a1_, 56i32, a3_) }) == (0i32)) as i32)) != 0) {
            if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32)) < (0i32)) as i32)) != 0) {
                { let v_ = 40i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
            }
        } else {
            if (((((c.m.r32((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))) as i32)) & (1i32))) != 0) {
                { let a1_ = { let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(237452); (c.m.r32(a_) as i32) }; F::McFlow_ClearSlotInfo(c, 1i32, a1_) };
                { let r_ = 32i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(12); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 40i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            if ((((({ let a0_ = (((c.m.r32(sym::gMcFlow)).wrapping_add(235576)).wrapping_add(((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(237452); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(56))).wrapping_add(8); F::McCard_Checksum(c, a0_, 48i32, 0i32) }) != (0i32)) as i32)) != 0) {
                { let a1_ = { let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(237452); (c.m.r32(a_) as i32) }; F::McFlow_ClearSlotInfo(c, 1i32, a1_) };
                { let r_ = 32i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(12); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 40i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            { let r_ = 16i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(12); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            { let a1_ = { let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(237452); (c.m.r32(a_) as i32) }; F::McFlow_CopySlotInfo(c, 1i32, a1_) };
            { let v_ = 40i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
        }
        break 's1;
        }
        F::Dialog_SetChoices(c, 0i32);
        { let v_ = 59i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
        { let _ = F::McCard_Probe(c, 0i32); };
        if (({ let a0_ = (c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32); F::McCard_IsFatalError(c, a0_) }) != 0) {
            F::Dialog_SetChoices(c, 0i32);
            { let v_ = 28i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
            { let v_ = 27i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
            { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            { let v_ = 0i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
            break 's1;
        }
        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
            if ((F::Dialog_IsOpen(c)) != 0) {
                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                F::Dialog_SetCursor(c, 1i32);
                { let v_ = (if (((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1676); (c.m.r32(a_) as i32) }) & (1i32))) != 0) { 18i32 } else { 22i32 }); let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
            }
        }
        break 's1;
        }
        F::Dialog_SetChoices(c, 0i32);
        { let v_ = 28i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
        if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); (c.m.r32(a_) as i32) }) > (0i32)) as i32)) != 0) {
            { let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
            break 's1;
        }
        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
            if ((F::Dialog_IsOpen(c)) != 0) {
                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                F::Dialog_SetCursor(c, 1i32);
                { let v_ = (if (((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1676); (c.m.r32(a_) as i32) }) & (1i32))) != 0) { 18i32 } else { 22i32 }); let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
            }
        }
        break 's1;
        }
        F::Dialog_SetChoices(c, 0i32);
        { let v_ = 58i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
        { let _ = F::McCard_Probe(c, 0i32); };
        if (({ let a0_ = (c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32); F::McCard_IsFatalError(c, a0_) }) != 0) {
            F::Dialog_SetChoices(c, 0i32);
            { let v_ = 28i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
            { let v_ = 27i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
            { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            { let v_ = 0i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
            break 's1;
        }
        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
            if ((F::Dialog_IsOpen(c)) != 0) {
                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                F::Dialog_SetCursor(c, 1i32);
                { let v_ = (if (((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1676); (c.m.r32(a_) as i32) }) & (1i32))) != 0) { 18i32 } else { 22i32 }); let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
            }
        }
        break 's1;
        }
        F::Dialog_SetChoices(c, 1i32);
        { let v_ = 19i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
        if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); (c.m.r32(a_) as i32) }) > (0i32)) as i32)) != 0) {
            if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) >= (0i32)) as i32)) != 0) {
                break 's1;
            }
            { let v_ = 37i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
            { let v_ = 0i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
        } else {
            if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); (c.m.r32(a_) as i32) }) < ((1i32).wrapping_neg())) as i32)) != 0) {
                { let v_ = 39i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
                F::Dialog_SetCursor(c, 1i32);
                F::McCard_ResetStep(c);
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(237452); c.m.w32(a_, (v_) as u32); };
            } else {
                { let v_ = F::Dialog_Input(c, 0i32); let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
                { let v_ = 12i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            }
        }
        break 's1;
        }
        F::Dialog_SetChoices(c, 1i32);
        { let v_ = 23i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
        if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); (c.m.r32(a_) as i32) }) > (0i32)) as i32)) != 0) {
            if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) >= (0i32)) as i32)) != 0) {
                break 's1;
            }
            { let v_ = 37i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
            { let v_ = 0i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
        } else {
            if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); (c.m.r32(a_) as i32) }) < ((1i32).wrapping_neg())) as i32)) != 0) {
                { let v_ = 39i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 12i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
                F::Dialog_SetCursor(c, 1i32);
                F::McCard_ResetStep(c);
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(237452); c.m.w32(a_, (v_) as u32); };
            } else {
                { let v_ = F::Dialog_Input(c, 0i32); let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
                { let v_ = 12i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            }
        }
        break 's1;
        }
        F::Dialog_SetChoices(c, 0i32);
        F::Dialog_Start(c, 1i32);
        { let v_ = 38i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
        }
        { let _ = F::Dialog_Input(c, 0i32); };
        if (((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(12); (c.m.r32(a_) as i32) }) & (8i32))) != 0) {
            { let _ = F::McCard_Probe(c, 0i32); };
            if (({ let a0_ = (c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32); F::McCard_IsFatalError(c, a0_) }) != 0) {
                { let r_ = 128i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(12); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let r_ = 8i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(12); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
                F::McFlow_CallModeRemoved(c);
                break 's1;
            }
        }
        if ((F::Dialog_IsClosed(c)) != 0) {
            if (((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(12); (c.m.r32(a_) as i32) }) & (128i32))) != 0) {
                { let v_ = 35i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 27i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(12); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
            } else {
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                F::McFlow_CallModeDone(c);
            }
            F::McCard_ResetStep(c);
        }
        break 's1;
    }
    { let a0_ = { let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(48); (c.m.r32(a_) as i32) }; F::Dialog_SetMsg(c, a0_) };
    F::Dialog_Draw(c, 1i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "McFlow_UpdateReplayScan", &PROBE_N, &[], true); }
    r_
}

pub fn McFlow_UpdateReplaySave(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "McFlow_UpdateReplaySave", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
        break 'f;
    }
    's1: {
        let sv1: i32 = { let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); (c.m.r32(a_) as i32) };
        'k22: {
        'k21: {
        'k20: {
        'k19: {
        'k18: {
        'k17: {
        'k16: {
        'k15: {
        'k14: {
        'k13: {
        'k12: {
        'k11: {
        'k10: {
        'k9: {
        'k8: {
        'k7: {
        'k6: {
        'k5: {
        'k4: {
        'k3: {
        'k2: {
            match sv1 { 35i32 => break 'k2, 12i32 => break 'k3, 3i32 => break 'k4, 23i32 => break 'k5, 24i32 => break 'k6, 13i32 => break 'k7, 11i32 => break 'k8, 15i32 => break 'k9, 6i32 => break 'k10, 30i32 => break 'k11, 16i32 => break 'k12, 17i32 => break 'k13, 25i32 => break 'k14, 26i32 => break 'k15, 27i32 => break 'k16, 18i32 => break 'k17, 31i32 => break 'k18, 33i32 => break 'k19, 34i32 => break 'k20, 37i32 => break 'k21, 38i32 => break 'k22, _ => break 's1, }
        }
        F::Dialog_SetLayout(c, 1i32);
        F::Dialog_SetCursor(c, 1i32);
        F::Dialog_SetTitle(c, 0i32);
        F::Dialog_Start(c, 0i32);
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
        F::McCard_ResetStep(c);
        { let v_ = 12i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
        break 's1;
        }
        F::Dialog_SetChoices(c, 1i32);
        { let v_ = 47i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
        { let _ = F::McCard_Probe(c, 0i32); };
        if (({ let a0_ = (c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32); F::McCard_IsFatalError(c, a0_) }) != 0) {
            F::Dialog_SetChoices(c, 0i32);
            { let v_ = 28i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
            { let v_ = 27i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
            { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            { let v_ = 0i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
            break 's1;
        }
        if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); (c.m.r32(a_) as i32) }) > (0i32)) as i32)) != 0) {
            if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) >= (0i32)) as i32)) != 0) {
                break 's1;
            }
            { let v_ = 3i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
            { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            { let v_ = 0i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
            F::McCard_ResetStep(c);
        } else {
            if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); (c.m.r32(a_) as i32) }) < (0i32)) as i32)) != 0) {
                { let v_ = 37i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
                F::Dialog_SetCursor(c, 1i32);
                { let r_ = 256i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(12); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            } else {
                { let v_ = F::Dialog_Input(c, 0i32); let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
                { let v_ = 12i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            }
        }
        break 's1;
        }
        F::Dialog_SetChoices(c, 0i32);
        { let v_ = 40i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
        if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) >= (0i32)) as i32)) != 0) {
            if (((((F::McCard_GetInfo(c, 0i32)) == (0i32)) as i32)) != 0) {
                break 's1;
            }
            if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); (c.m.r32(a_) as i32) }) > (0i32)) as i32)) != 0) {
                break 's1;
            }
            if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(12)) as i32)) != (2i32)) as i32)) != 0) {
                { let v_ = 27i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            if (({ let a0_ = (c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32); F::McCard_IsFatalError(c, a0_) }) != 0) {
                { let v_ = 27i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(20)) as i32)) == (0i32)) as i32)) != 0) {
                { let v_ = 23i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                F::Dialog_SetCursor(c, 1i32);
            }
            break 's1;
        }
        if ((((({ let a1_ = (c.m.r32(sym::gMcFlow)).wrapping_add(235968); F::McCard_CheckFiles(c, 0i32, a1_) }) == (0i32)) as i32)) != 0) {
            if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(12)) as i32)) != (2i32)) as i32)) != 0) {
                { let v_ = 27i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(20)) as i32)) == (0i32)) as i32)) != 0) {
                { let v_ = 23i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                F::Dialog_SetCursor(c, 1i32);
                break 's1;
            }
            if (({ let a0_ = (c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32); F::McCard_IsFatalError(c, a0_) }) != 0) {
                { let v_ = 27i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32)) >= (0i32)) as i32)) != 0) {
                break 's1;
            }
            if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(16)) as i32)) < (160i32)) as i32)) != 0) {
                { let v_ = 30i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            { let v_ = 6i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
        } else {
            F::McCard_ResetStep(c);
            { let v_ = 31i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
        }
        break 's1;
        }
        F::Dialog_SetChoices(c, 1i32);
        { let v_ = 24i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
        { let _ = F::McCard_Probe(c, 0i32); };
        if (({ let a0_ = (c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32); F::McCard_IsFatalError(c, a0_) }) != 0) {
            F::Dialog_SetChoices(c, 0i32);
            { let v_ = 28i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
            { let v_ = 27i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
            { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            { let v_ = 0i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
            break 's1;
        }
        if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); (c.m.r32(a_) as i32) }) > (0i32)) as i32)) != 0) {
            if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) >= (0i32)) as i32)) != 0) {
                break 's1;
            }
            { let v_ = 24i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
            { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            { let v_ = 0i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
        } else {
            if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); (c.m.r32(a_) as i32) }) < (0i32)) as i32)) != 0) {
                { let v_ = 37i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
                F::Dialog_SetCursor(c, 1i32);
                { let r_ = 256i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(12); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            } else {
                { let v_ = F::Dialog_Input(c, 0i32); let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
                { let v_ = 12i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            }
        }
        break 's1;
        }
        F::Dialog_SetChoices(c, 0i32);
        { let v_ = 25i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
        if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) >= (0i32)) as i32)) != 0) {
            if (((((F::McCard_GetInfo(c, 0i32)) == (0i32)) as i32)) != 0) {
                break 's1;
            }
            if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); (c.m.r32(a_) as i32) }) > (0i32)) as i32)) != 0) {
                break 's1;
            }
            if (({ let a0_ = (c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32); F::McCard_IsFatalError(c, a0_) }) != 0) {
                { let v_ = 26i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            }
            break 's1;
        }
        if (((((F::McCard_Format(c, 0i32)) == (0i32)) as i32)) != 0) {
            if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32)) < (0i32)) as i32)) != 0) {
                { let v_ = 26i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            }
        } else {
            { let v_ = 25i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
            { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
        }
        break 's1;
        }
        F::Dialog_SetChoices(c, 1i32);
        { let v_ = 48i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
        { let _ = F::McCard_Probe(c, 0i32); };
        if (({ let a0_ = (c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32); F::McCard_IsFatalError(c, a0_) }) != 0) {
            { let v_ = 27i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
            { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            { let v_ = 0i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
            break 's1;
        }
        if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); (c.m.r32(a_) as i32) }) > (0i32)) as i32)) != 0) {
            if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) >= (0i32)) as i32)) != 0) {
                break 's1;
            }
            { let v_ = 15i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
            { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            { let v_ = 0i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
        } else {
            if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); (c.m.r32(a_) as i32) }) < (0i32)) as i32)) != 0) {
                { let v_ = 37i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
                F::Dialog_SetCursor(c, 1i32);
                { let r_ = 256i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(12); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            } else {
                { let v_ = F::Dialog_Input(c, 0i32); let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
                { let v_ = 12i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            }
        }
        break 's1;
        }
        F::Dialog_SetChoices(c, 0i32);
        { let v_ = 44i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
        if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) >= (0i32)) as i32)) != 0) {
            if (((((F::McCard_GetInfo(c, 0i32)) == (0i32)) as i32)) != 0) {
                break 's1;
            }
            if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); (c.m.r32(a_) as i32) }) > (0i32)) as i32)) != 0) {
                break 's1;
            }
            if (({ let a0_ = (c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32); F::McCard_IsFatalError(c, a0_) }) != 0) {
                { let v_ = 17i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            }
            break 's1;
        }
        if ((((({ let a1_ = (c.m.r32(sym::gMcFlow)).wrapping_add(235968); F::McCard_CreateSave(c, 0i32, a1_) }) == (0i32)) as i32)) != 0) {
            if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32)) < (0i32)) as i32)) != 0) {
                { let v_ = 17i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            }
        } else {
            { let v_ = 15i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
        }
        break 's1;
        }
        F::Dialog_SetChoices(c, 0i32);
        { let v_ = 50i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
        if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) >= (0i32)) as i32)) != 0) {
            if (((((F::McCard_GetInfo(c, 0i32)) == (0i32)) as i32)) != 0) {
                break 's1;
            }
            if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); (c.m.r32(a_) as i32) }) > (0i32)) as i32)) != 0) {
                break 's1;
            }
            if (({ let a0_ = (c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32); F::McCard_IsFatalError(c, a0_) }) != 0) {
                { let v_ = 17i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            }
            break 's1;
        }
        if ((((({ let a1_ = (c.m.r32(sym::gMcFlow)).wrapping_add(16440); let a3_ = (c.m.r32(sym::gMcFlow)).wrapping_add(235968); F::McCard_WriteSave(c, 0i32, a1_, 109568i32, a3_) }) == (0i32)) as i32)) != 0) {
            if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32)) < (0i32)) as i32)) != 0) {
                { let v_ = 17i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            }
        } else {
            { let a1_ = { let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(237448); (c.m.r32(a_) as i32) }; F::McFlow_CopySlotInfo(c, 0i32, a1_) };
            { let v_ = 16i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
            { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
        }
        break 's1;
        }
        F::Dialog_SetChoices(c, 0i32);
        { let v_ = 40i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
        if ((((({ let a1_ = (c.m.r32(sym::gMcFlow)).wrapping_add(235968); F::McCard_FindSave(c, 0i32, a1_) }) == (0i32)) as i32)) != 0) {
            if (({ let a0_ = (c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32); F::McCard_IsFatalError(c, a0_) }) != 0) {
                { let v_ = 27i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            }
            break 's1;
        }
        if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32)) == (1i32)) as i32)) != 0) {
            { let v_ = 11i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
            { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
        } else {
            { let v_ = 11i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
            { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
        }
        break 's1;
        }
        F::Dialog_SetChoices(c, 0i32);
        { let v_ = 60i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
            if ((F::Dialog_IsOpen(c)) != 0) {
                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                { let v_ = 18i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
                F::Dialog_SetCursor(c, 1i32);
            }
        }
        break 's1;
        }
        F::Dialog_SetChoices(c, 0i32);
        { let v_ = 51i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
            if ((F::Dialog_IsOpen(c)) != 0) {
                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                { let r_ = 8i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(12); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 37i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
        }
        if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) >= (0i32)) as i32)) != 0) {
            break 's1;
        }
        { let r_ = 8i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(12); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        { let v_ = 37i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
        break 's1;
        }
        F::Dialog_SetChoices(c, 0i32);
        { let v_ = 52i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
            if ((F::Dialog_IsOpen(c)) != 0) {
                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                { let v_ = 18i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
                F::Dialog_SetCursor(c, 1i32);
            }
        }
        break 's1;
        }
        F::Dialog_SetChoices(c, 0i32);
        { let v_ = 26i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
        { let _ = F::McCard_Probe(c, 0i32); };
        if (({ let a0_ = (c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32); F::McCard_IsFatalError(c, a0_) }) != 0) {
            { let v_ = 27i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
            { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            { let v_ = 0i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
            break 's1;
        }
        if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) >= (0i32)) as i32)) != 0) {
            break 's1;
        }
        { let v_ = 11i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
        { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
        break 's1;
        }
        F::Dialog_SetChoices(c, 0i32);
        { let v_ = 27i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
            if ((F::Dialog_IsOpen(c)) != 0) {
                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                { let v_ = 18i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
                F::Dialog_SetCursor(c, 1i32);
            }
        }
        break 's1;
        }
        F::Dialog_SetChoices(c, 0i32);
        { let v_ = 28i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
        if ((((!(((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(12); (c.m.r32(a_) as i32) }) & (128i32))) != 0)) as i32)) != 0) {
            F::McFlow_CallModeRemoved(c);
            { let r_ = 128i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(12); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        }
        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
            if ((F::Dialog_IsOpen(c)) != 0) {
                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                { let v_ = 18i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
                F::Dialog_SetCursor(c, 1i32);
                { let r_ = (!(128i32)); let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(12); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
            }
        }
        break 's1;
        }
        F::Dialog_SetChoices(c, 1i32);
        { let v_ = 19i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
        if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); (c.m.r32(a_) as i32) }) > (0i32)) as i32)) != 0) {
            if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) >= (0i32)) as i32)) != 0) {
                break 's1;
            }
            { let v_ = 37i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
            { let v_ = 0i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
        } else {
            if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); (c.m.r32(a_) as i32) }) < ((1i32).wrapping_neg())) as i32)) != 0) {
                { let v_ = 7i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
                { let v_ = fid::McFlow_UpdateReplayScan; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(237024); c.m.w32(a_, v_); };
                { let v_ = 39i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
                F::Dialog_SetCursor(c, 1i32);
                F::McCard_ResetStep(c);
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(237452); c.m.w32(a_, (v_) as u32); };
            } else {
                { let v_ = F::Dialog_Input(c, 0i32); let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
                { let v_ = 12i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            }
        }
        break 's1;
        }
        F::Dialog_SetChoices(c, 0i32);
        { let v_ = 40i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
        if ((((({ let a1_ = (c.m.r32(sym::gMcFlow)).wrapping_add(235968); F::McCard_CalcNeed(c, 0i32, a1_, 1i32) }) == (0i32)) as i32)) != 0) {
            if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(12)) as i32)) != (2i32)) as i32)) != 0) {
                { let v_ = 27i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(20)) as i32)) == (0i32)) as i32)) != 0) {
                { let v_ = 23i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                F::Dialog_SetCursor(c, 1i32);
                break 's1;
            }
            if (({ let a0_ = (c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32); F::McCard_IsFatalError(c, a0_) }) != 0) {
                { let v_ = 27i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            }
            break 's1;
        }
        if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(28)) as i32)) > (0i32)) as i32)) != 0) {
            if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(16)) as i32)) < ((c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(28)) as i32))) as i32)) != 0) {
                { let v_ = 30i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            { let v_ = 33i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
            { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            F::Dialog_SetCursor(c, 1i32);
        } else {
            { let v_ = 13i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
            { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            F::Dialog_SetCursor(c, 1i32);
        }
        break 's1;
        }
        F::Dialog_SetChoices(c, 1i32);
        { let v_ = 48i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
        { let _ = F::McCard_Probe(c, 0i32); };
        if (({ let a0_ = (c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32); F::McCard_IsFatalError(c, a0_) }) != 0) {
            { let v_ = 27i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
            { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            { let v_ = 0i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
            break 's1;
        }
        if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); (c.m.r32(a_) as i32) }) > (0i32)) as i32)) != 0) {
            if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) >= (0i32)) as i32)) != 0) {
                break 's1;
            }
            { let v_ = 34i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
            { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            { let v_ = 0i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
        } else {
            if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); (c.m.r32(a_) as i32) }) < (0i32)) as i32)) != 0) {
                { let v_ = 37i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
                F::Dialog_SetCursor(c, 1i32);
                { let r_ = 256i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(12); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            } else {
                { let v_ = F::Dialog_Input(c, 0i32); let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
                { let v_ = 12i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            }
        }
        break 's1;
        }
        F::Dialog_SetChoices(c, 0i32);
        { let v_ = 50i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
        if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) >= (0i32)) as i32)) != 0) {
            if (((((F::McCard_GetInfo(c, 0i32)) == (0i32)) as i32)) != 0) {
                break 's1;
            }
            if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); (c.m.r32(a_) as i32) }) > (0i32)) as i32)) != 0) {
                break 's1;
            }
            if (({ let a0_ = (c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32); F::McCard_IsFatalError(c, a0_) }) != 0) {
                { let v_ = 17i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            }
            break 's1;
        }
        if ((((({ let a1_ = (c.m.r32(sym::gMcFlow)).wrapping_add(235968); F::McCard_CreateSave(c, 0i32, a1_) }) == (0i32)) as i32)) != 0) {
            if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32)) < (0i32)) as i32)) != 0) {
                { let v_ = 17i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            }
        } else {
            { let v_ = 15i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
        }
        break 's1;
        }
        F::Dialog_SetChoices(c, 0i32);
        F::Dialog_Start(c, 1i32);
        { let v_ = 38i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
        }
        { let _ = F::Dialog_Input(c, 0i32); };
        if (((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(12); (c.m.r32(a_) as i32) }) & (256i32))) != 0) {
            { let _ = F::McCard_Probe(c, 0i32); };
            if (({ let a0_ = (c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32); F::McCard_IsFatalError(c, a0_) }) != 0) {
                { let r_ = 128i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(12); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let r_ = 256i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(12); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
                F::McFlow_CallModeRemoved(c);
                break 's1;
            }
        }
        if ((F::Dialog_IsClosed(c)) != 0) {
            if (((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(12); (c.m.r32(a_) as i32) }) & (128i32))) != 0) {
                { let v_ = 7i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
                { let v_ = fid::McFlow_UpdateReplayScan; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(237024); c.m.w32(a_, v_); };
                { let v_ = 35i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 27i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(12); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
            } else {
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                F::McFlow_CallModeDone(c);
            }
            F::McCard_ResetStep(c);
        }
        break 's1;
    }
    { let a0_ = { let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(48); (c.m.r32(a_) as i32) }; F::Dialog_SetMsg(c, a0_) };
    F::Dialog_Draw(c, 1i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "McFlow_UpdateReplaySave", &PROBE_N, &[], true); }
    r_
}

pub fn McFlow_UpdateReplayLoad(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "McFlow_UpdateReplayLoad", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
        break 'f;
    }
    's1: {
        let sv1: i32 = { let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); (c.m.r32(a_) as i32) };
        'k14: {
        'k13: {
        'k12: {
        'k11: {
        'k10: {
        'k9: {
        'k8: {
        'k7: {
        'k6: {
        'k5: {
        'k4: {
        'k3: {
        'k2: {
            match sv1 { 35i32 => break 'k2, 2i32 => break 'k3, 4i32 => break 'k4, 6i32 => break 'k5, 19i32 => break 'k6, 20i32 => break 'k7, 21i32 => break 'k8, 29i32 => break 'k9, 27i32 => break 'k10, 28i32 => break 'k11, 22i32 => break 'k12, 37i32 => break 'k13, 38i32 => break 'k14, _ => break 's1, }
        }
        F::Dialog_SetLayout(c, 1i32);
        F::Dialog_SetCursor(c, 1i32);
        F::Dialog_SetTitle(c, 0i32);
        F::Dialog_Start(c, 0i32);
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
        F::McCard_ResetStep(c);
        { let v_ = 2i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
        }
        F::Dialog_SetChoices(c, 1i32);
        { let v_ = 54i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
        { let _ = F::McCard_Probe(c, 0i32); };
        if (({ let a0_ = (c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32); F::McCard_IsFatalError(c, a0_) }) != 0) {
            F::Dialog_SetChoices(c, 0i32);
            { let v_ = 28i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
            { let v_ = 27i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
            { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            { let v_ = 0i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
            break 's1;
        }
        if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); (c.m.r32(a_) as i32) }) > (0i32)) as i32)) != 0) {
            if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) >= (0i32)) as i32)) != 0) {
                break 's1;
            }
            { let v_ = 4i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
            { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            { let v_ = 0i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
            F::McCard_ResetStep(c);
        } else {
            if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); (c.m.r32(a_) as i32) }) < (0i32)) as i32)) != 0) {
                F::Dialog_SetCursor(c, 1i32);
                { let v_ = 37i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
                { let r_ = 256i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(12); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            } else {
                { let v_ = F::Dialog_Input(c, 0i32); let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
                { let v_ = 12i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            }
        }
        break 's1;
        }
        F::Dialog_SetChoices(c, 0i32);
        { let v_ = 40i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
        if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) >= (0i32)) as i32)) != 0) {
            if (((((F::McCard_GetInfo(c, 0i32)) == (0i32)) as i32)) != 0) {
                break 's1;
            }
            if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); (c.m.r32(a_) as i32) }) > (0i32)) as i32)) != 0) {
                break 's1;
            }
            if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(12)) as i32)) != (2i32)) as i32)) != 0) {
                { let v_ = 27i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            if (({ let a0_ = (c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32); F::McCard_IsFatalError(c, a0_) }) != 0) {
                { let v_ = 27i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            }
            break 's1;
        }
        if ((((({ let a1_ = (c.m.r32(sym::gMcFlow)).wrapping_add(235968); F::McCard_CheckFiles(c, 0i32, a1_) }) == (0i32)) as i32)) != 0) {
            if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(12)) as i32)) != (2i32)) as i32)) != 0) {
                { let v_ = 27i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            if (({ let a0_ = (c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32); F::McCard_IsFatalError(c, a0_) }) != 0) {
                { let v_ = 27i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32)) >= (0i32)) as i32)) != 0) {
                break 's1;
            }
            { let v_ = 6i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
        } else {
            { let v_ = 19i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
            { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
        }
        break 's1;
        }
        F::Dialog_SetChoices(c, 0i32);
        { let v_ = 40i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
        if ((((({ let a1_ = (c.m.r32(sym::gMcFlow)).wrapping_add(235968); F::McCard_FindSave(c, 0i32, a1_) }) == (0i32)) as i32)) != 0) {
            if (({ let a0_ = (c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32); F::McCard_IsFatalError(c, a0_) }) != 0) {
                { let v_ = 27i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            }
            break 's1;
        }
        if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32)) == (1i32)) as i32)) != 0) {
            { let v_ = 29i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
            { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
        } else {
            { let v_ = 28i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
            { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
        }
        break 's1;
        }
        F::Dialog_SetChoices(c, 0i32);
        { let v_ = 55i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
        if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) >= (0i32)) as i32)) != 0) {
            if (((((F::McCard_GetInfo(c, 0i32)) == (0i32)) as i32)) != 0) {
                break 's1;
            }
            if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); (c.m.r32(a_) as i32) }) > (0i32)) as i32)) != 0) {
                break 's1;
            }
            if (({ let a0_ = (c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32); F::McCard_IsFatalError(c, a0_) }) != 0) {
                { let v_ = 21i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            }
            break 's1;
        }
        if ((((({ let a1_ = (c.m.r32(sym::gMcFlow)).wrapping_add(126008); let a3_ = (c.m.r32(sym::gMcFlow)).wrapping_add(235968); F::McCard_ReadFile(c, 0i32, a1_, 109568i32, a3_) }) == (0i32)) as i32)) != 0) {
            if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32)) == ((3i32).wrapping_neg())) as i32)) != 0) {
                { let v_ = 29i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32)) < (0i32)) as i32)) != 0) {
                { let v_ = 21i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            }
        } else {
            if (((((c.m.r32((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))) as i32)) & (1i32))) != 0) {
                { let v_ = 29i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            if ((((({ let a0_ = (c.m.r32(sym::gMcFlow)).wrapping_add(126008); F::McCard_Checksum(c, a0_, 109568i32, 0i32) }) != (0i32)) as i32)) != 0) {
                { let v_ = 29i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            F::Battle_ClearWork(c);
            { let _ = { let a0_ = ((c.m.r32(sym::gMcFlow)).wrapping_add(126008)).wrapping_add(56); F::BattleReplay_Load(c, a0_, 109480i32) }; };
            { let v_ = 20i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
            { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
        }
        break 's1;
        }
        F::Dialog_SetChoices(c, 0i32);
        { let v_ = 56i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
            if ((F::Dialog_IsOpen(c)) != 0) {
                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                { let r_ = 8i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(12); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 37i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
        }
        if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) >= (0i32)) as i32)) != 0) {
            break 's1;
        }
        { let r_ = 8i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(12); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        { let v_ = 37i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
        break 's1;
        }
        F::Dialog_SetChoices(c, 0i32);
        { let v_ = 57i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
            if ((F::Dialog_IsOpen(c)) != 0) {
                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                F::Dialog_SetCursor(c, 1i32);
                { let v_ = 22i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
            }
        }
        break 's1;
        }
        F::Dialog_SetChoices(c, 0i32);
        { let v_ = 59i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
            if ((F::Dialog_IsOpen(c)) != 0) {
                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                F::Dialog_SetCursor(c, 1i32);
                { let v_ = 22i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
            }
        }
        break 's1;
        }
        F::Dialog_SetChoices(c, 0i32);
        { let v_ = 28i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
        if ((((!(((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(12); (c.m.r32(a_) as i32) }) & (128i32))) != 0)) as i32)) != 0) {
            F::McFlow_CallModeRemoved(c);
            { let r_ = 128i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(12); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        }
        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
            if ((F::Dialog_IsOpen(c)) != 0) {
                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                F::Dialog_SetCursor(c, 1i32);
                { let v_ = 22i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let r_ = (!(128i32)); let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(12); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
            }
        }
        break 's1;
        }
        F::Dialog_SetChoices(c, 0i32);
        { let v_ = 58i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
            if ((F::Dialog_IsOpen(c)) != 0) {
                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                F::Dialog_SetCursor(c, 1i32);
                { let v_ = 22i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
            }
        }
        break 's1;
        }
        F::Dialog_SetChoices(c, 1i32);
        { let v_ = 23i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
        if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); (c.m.r32(a_) as i32) }) > (0i32)) as i32)) != 0) {
            if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) >= (0i32)) as i32)) != 0) {
                break 's1;
            }
            { let v_ = 37i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
            { let v_ = 0i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
        } else {
            if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); (c.m.r32(a_) as i32) }) < ((1i32).wrapping_neg())) as i32)) != 0) {
                { let v_ = 7i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
                { let v_ = fid::McFlow_UpdateReplayScan; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(237024); c.m.w32(a_, v_); };
                { let v_ = 39i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
                F::Dialog_SetCursor(c, 1i32);
                F::McCard_ResetStep(c);
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(237452); c.m.w32(a_, (v_) as u32); };
            } else {
                { let v_ = F::Dialog_Input(c, 0i32); let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
                { let v_ = 12i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            }
        }
        break 's1;
        }
        F::Dialog_SetChoices(c, 0i32);
        F::Dialog_Start(c, 1i32);
        { let v_ = 38i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
        }
        { let _ = F::Dialog_Input(c, 0i32); };
        if (((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(12); (c.m.r32(a_) as i32) }) & (256i32))) != 0) {
            { let _ = F::McCard_Probe(c, 0i32); };
            if (({ let a0_ = (c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32); F::McCard_IsFatalError(c, a0_) }) != 0) {
                { let r_ = 128i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(12); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let r_ = 256i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(12); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
                F::McFlow_CallModeRemoved(c);
                break 's1;
            }
        }
        if ((F::Dialog_IsClosed(c)) != 0) {
            if (((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(12); (c.m.r32(a_) as i32) }) & (128i32))) != 0) {
                { let v_ = 7i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
                { let v_ = fid::McFlow_UpdateReplayScan; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(237024); c.m.w32(a_, v_); };
                { let v_ = 35i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 27i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(12); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
            } else {
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                F::McFlow_CallModeDone(c);
            }
            F::McCard_ResetStep(c);
        }
        break 's1;
    }
    { let a0_ = { let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(48); (c.m.r32(a_) as i32) }; F::Dialog_SetMsg(c, a0_) };
    F::Dialog_Draw(c, 1i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "McFlow_UpdateReplayLoad", &PROBE_N, &[], true); }
    r_
}

pub fn McFlow_UpdateReplayAsk(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "McFlow_UpdateReplayAsk", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
        break 'f;
    }
    's1: {
        let sv1: i32 = { let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); (c.m.r32(a_) as i32) };
        'k7: {
        'k6: {
        'k5: {
        'k4: {
        'k3: {
        'k2: {
            match sv1 { 35i32 => break 'k2, 41i32 => break 'k3, 27i32 => break 'k4, 18i32 => break 'k5, 37i32 => break 'k6, 38i32 => break 'k7, _ => break 's1, }
        }
        F::Dialog_SetLayout(c, 1i32);
        F::Dialog_SetCursor(c, 1i32);
        F::Dialog_SetTitle(c, 0i32);
        F::Dialog_Start(c, 0i32);
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
        F::McCard_ResetStep(c);
        { let v_ = 41i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
        }
        F::Dialog_SetChoices(c, 1i32);
        { let v_ = 61i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
        { let _ = F::McCard_Probe(c, 0i32); };
        if (({ let a0_ = (c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32); F::McCard_IsFatalError(c, a0_) }) != 0) {
            F::Dialog_SetChoices(c, 0i32);
            { let v_ = 28i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
            { let v_ = 27i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
            { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            { let v_ = 0i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
            break 's1;
        }
        if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); (c.m.r32(a_) as i32) }) > (0i32)) as i32)) != 0) {
            if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) >= (0i32)) as i32)) != 0) {
                break 's1;
            }
            { let v_ = 37i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
            { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            { let v_ = 0i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
            F::McCard_ResetStep(c);
            { let r_ = 8i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(12); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        } else {
            if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); (c.m.r32(a_) as i32) }) < (0i32)) as i32)) != 0) {
                { let v_ = 37i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
                F::Dialog_SetCursor(c, 1i32);
                { let r_ = 256i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(12); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            } else {
                { let v_ = F::Dialog_Input(c, 0i32); let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
                { let v_ = 12i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            }
        }
        break 's1;
        }
        F::Dialog_SetChoices(c, 0i32);
        { let v_ = 28i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
        if ((((!(((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(12); (c.m.r32(a_) as i32) }) & (128i32))) != 0)) as i32)) != 0) {
            F::McFlow_CallModeRemoved(c);
            { let r_ = 128i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(12); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        }
        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
            if ((F::Dialog_IsOpen(c)) != 0) {
                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                { let v_ = 18i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
                F::Dialog_SetCursor(c, 1i32);
                { let r_ = (!(128i32)); let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(12); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
            }
        }
        break 's1;
        }
        F::Dialog_SetChoices(c, 1i32);
        { let v_ = 19i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(48); c.m.w32(a_, (v_) as u32); };
        if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); (c.m.r32(a_) as i32) }) > (0i32)) as i32)) != 0) {
            if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) >= (0i32)) as i32)) != 0) {
                break 's1;
            }
            { let v_ = 37i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
            { let v_ = 0i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
            { let r_ = 8i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(12); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        } else {
            if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); (c.m.r32(a_) as i32) }) < ((1i32).wrapping_neg())) as i32)) != 0) {
                { let v_ = 7i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
                { let v_ = fid::McFlow_UpdateReplayScan; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(237024); c.m.w32(a_, v_); };
                { let v_ = 39i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 120i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
                F::Dialog_SetCursor(c, 1i32);
                F::McCard_ResetStep(c);
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(237452); c.m.w32(a_, (v_) as u32); };
            } else {
                { let v_ = F::Dialog_Input(c, 0i32); let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
                { let v_ = 12i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
            }
        }
        break 's1;
        }
        F::Dialog_SetChoices(c, 0i32);
        F::Dialog_Start(c, 1i32);
        { let v_ = 38i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
        }
        { let _ = F::Dialog_Input(c, 0i32); };
        if (((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(12); (c.m.r32(a_) as i32) }) & (256i32))) != 0) {
            { let _ = F::McCard_Probe(c, 0i32); };
            if (({ let a0_ = (c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32); F::McCard_IsFatalError(c, a0_) }) != 0) {
                { let r_ = 128i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(12); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let r_ = 256i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(12); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
                F::McFlow_CallModeRemoved(c);
                break 's1;
            }
        }
        if ((F::Dialog_IsClosed(c)) != 0) {
            if (((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(12); (c.m.r32(a_) as i32) }) & (128i32))) != 0) {
                { let v_ = 7i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
                { let v_ = fid::McFlow_UpdateReplayScan; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(237024); c.m.w32(a_, v_); };
                { let v_ = 35i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 27i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(12); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
            } else {
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                F::McFlow_CallModeDone(c);
            }
            F::McCard_ResetStep(c);
        }
        break 's1;
    }
    { let a0_ = { let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(48); (c.m.r32(a_) as i32) }; F::Dialog_SetMsg(c, a0_) };
    F::Dialog_Draw(c, 1i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "McFlow_UpdateReplayAsk", &PROBE_N, &[], true); }
    r_
}

pub fn McFlow_Init(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "McFlow_Init", &PROBE_N, &[], false); }
    let mut l_work_1: u32 = 0;
    let mut l_hdr_2: u32 = 0;
    let mut l_rep_3: u32 = 0;
    let mut l_i_4: i32 = 0;
    let mut l_j_5: i32 = 0;
    let r_ = 'f: {
    { let v_ = F::Heap_Alloc(c, 237456i32, ((32i32) as u32), 0i32, 2i32); let a_ = sym::gMcFlow; c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32(sym::gMcFlow); F::memset(c, a0_, 0i32, ((237456i32) as u32)) }; };
    { let v_ = { let a0_ = ({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_add(29i32); F::File_LoadSync(c, a0_, ((0i32) as u32), 0i32) }; let a_ = c.m.r32(sym::gMcFlow); c.m.w32(a_, v_); };
    { let v_ = { let a0_ = { let a_ = c.m.r32(sym::gMcFlow); c.m.r32(a_) }; F::Sprite_Unpack(c, a0_, ((0i32) as u32), ((0i32) as u32)) }; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(4); c.m.w32(a_, v_); };
    l_work_1 = c.m.r32(sym::gMcFlow);
    l_rep_3 = F::BattleReplay_GetBuffer(c, ((0i32) as u32));
    l_hdr_2 = (l_work_1).wrapping_add(16440);
    { let s_ = l_rep_3; let d_ = ((l_work_1).wrapping_add(16440)).wrapping_add(56); rt::memcpy(c, d_, s_, 109480); };
    {
        l_i_4 = 0i32;
        'l1: loop {
            if !(((((l_i_4) < (2i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                {
                    l_j_5 = 0i32;
                    'l3: loop {
                        if !(((((l_j_5) < (5i32)) as i32)) != 0) { break 'l3; }
                        'c4: {
                            if (((((((l_j_5) as u32)) < (c.m.r32(((l_rep_3).wrapping_add(192)).wrapping_add((((l_i_4) as u32)).wrapping_mul(624))))) as i32)) != 0) {
                                { let v_ = (c.m.r32(((((l_rep_3).wrapping_add(192)).wrapping_add((((l_i_4) as u32)).wrapping_mul(624))).wrapping_add(4)).wrapping_add((((l_j_5) as u32)).wrapping_mul(100))) as i32); let a_ = (((l_hdr_2).wrapping_add(16)).wrapping_add((((l_i_4) as u32)).wrapping_mul(20))).wrapping_add((((l_j_5) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                            } else {
                                { let v_ = 164i32; let a_ = (((l_hdr_2).wrapping_add(16)).wrapping_add((((l_i_4) as u32)).wrapping_mul(20))).wrapping_add((((l_j_5) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                            }
                        }
                        { let o_ = l_j_5; let n_ = (o_).wrapping_add(1); l_j_5 = n_; };
                    }
                }
            }
            { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
        }
    }
    { let _ = F::McCard_Checksum(c, (l_hdr_2).wrapping_add(8), 48i32, 1i32); };
    { let _ = F::McCard_Checksum(c, l_hdr_2, 109568i32, 1i32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "McFlow_Init", &PROBE_N, &[], true); }
    r_
}

pub fn McFlow_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "McFlow_Term", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(4); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = { let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(4); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(4); c.m.w32(a_, v_); };
    }
    if ((((({ let a_ = c.m.r32(sym::gMcFlow); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = { let a_ = c.m.r32(sym::gMcFlow); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = c.m.r32(sym::gMcFlow); c.m.w32(a_, v_); };
    }
    if (((((c.m.r32(sym::gMcFlow)) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32(sym::gMcFlow); F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = sym::gMcFlow; c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "McFlow_Term", &PROBE_N, &[], true); }
    r_
}

pub fn McFlow_Start(c: &mut Ctx, mut p_mode_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "McFlow_Start", &PROBE_N, &[(p_mode_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_mode_1; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
    's1: {
        let sv2: i32 = p_mode_1;
        match sv2 {
            0i32 => {
                { let v_ = fid::McFlow_UpdateSave; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(237024); c.m.w32(a_, v_); };
                { let v_ = 35i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 12i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                F::McFlow_SetupRequest(c, 0i32, 0i32);
                { let _ = { let a0_ = c.m.r32(sym::gSaveData); F::McCard_Checksum(c, a0_, 16384i32, 1i32) }; };
                break 's1;
            }
            1i32 => {
                { let v_ = fid::McFlow_UpdateLoad; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(237024); c.m.w32(a_, v_); };
                { let v_ = 35i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 2i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                F::McFlow_SetupRequest(c, 0i32, 0i32);
                break 's1;
            }
            2i32 => {
                { let v_ = fid::McFlow_UpdateBootLoad; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(237024); c.m.w32(a_, v_); };
                { let v_ = 35i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                F::McFlow_SetupRequest(c, 0i32, 0i32);
                break 's1;
            }
            3i32 => {
                { let v_ = fid::McFlow_UpdateNewSave; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(237024); c.m.w32(a_, v_); };
                { let v_ = 35i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                F::McFlow_SetupRequest(c, 0i32, 0i32);
                break 's1;
            }
            4i32 => {
                { let v_ = fid::McFlow_UpdateSave; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(237024); c.m.w32(a_, v_); };
                { let v_ = 35i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 32i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                F::McFlow_SetupRequest(c, 0i32, 0i32);
                { let _ = { let a0_ = c.m.r32(sym::gSaveData); F::McCard_Checksum(c, a0_, 16384i32, 1i32) }; };
                break 's1;
            }
            5i32 => {
                { let v_ = fid::McFlow_UpdateReplaySave; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(237024); c.m.w32(a_, v_); };
                { let v_ = 35i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let a1_ = { let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(237448); (c.m.r32(a_) as i32) }; F::McFlow_SetupRequest(c, 1i32, a1_) };
                break 's1;
            }
            6i32 => {
                { let v_ = fid::McFlow_UpdateReplayLoad; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(237024); c.m.w32(a_, v_); };
                { let v_ = 35i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let a1_ = { let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(237448); (c.m.r32(a_) as i32) }; F::McFlow_SetupRequest(c, 1i32, a1_) };
                break 's1;
            }
            7i32 => {
                { let v_ = fid::McFlow_UpdateReplayScan; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(237024); c.m.w32(a_, v_); };
                { let v_ = 35i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 39i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                { let r_ = (!(112i32)); let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(12); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                F::Progress_ClearTeams(c);
                break 's1;
            }
            8i32 => {
                { let v_ = fid::McFlow_UpdateReplayAsk; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(237024); c.m.w32(a_, v_); };
                { let v_ = 35i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            9i32 => {
                { let v_ = fid::McFlow_UpdateReplayScan; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(237024); c.m.w32(a_, v_); };
                { let v_ = 35i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                { let v_ = 27i32; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            _ => {
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "McFlow_Start", &PROBE_N, &[], true); }
    r_
}

pub fn McFlow_SetDoneCb(c: &mut Ctx, mut p_idx_1: i32, mut p_cb_2: u32, mut p_arg_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "McFlow_SetDoneCb", &PROBE_N, &[(p_idx_1) as u32, (p_cb_2) as u32, (p_arg_3) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_cb_2; let a_ = ((c.m.r32(sym::gMcFlow)).wrapping_add(237028)).wrapping_add((((p_idx_1) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = p_arg_3; let a_ = ((c.m.r32(sym::gMcFlow)).wrapping_add(237044)).wrapping_add((((p_idx_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "McFlow_SetDoneCb", &PROBE_N, &[], true); }
    r_
}

pub fn McFlow_SetModeCb(c: &mut Ctx, mut p_mode_1: i32, mut p_idx_2: i32, mut p_cb_3: u32, mut p_arg_4: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "McFlow_SetModeCb", &PROBE_N, &[(p_mode_1) as u32, (p_idx_2) as u32, (p_cb_3) as u32, (p_arg_4) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_cb_3; let a_ = (((c.m.r32(sym::gMcFlow)).wrapping_add(237060)).wrapping_add((((p_mode_1) as u32)).wrapping_mul(16))).wrapping_add((((p_idx_2) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = p_arg_4; let a_ = (((c.m.r32(sym::gMcFlow)).wrapping_add(237252)).wrapping_add((((p_mode_1) as u32)).wrapping_mul(16))).wrapping_add((((p_idx_2) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "McFlow_SetModeCb", &PROBE_N, &[], true); }
    r_
}

pub fn McFlow_SetSlot(c: &mut Ctx, mut p_slot_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "McFlow_SetSlot", &PROBE_N, &[(p_slot_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_slot_1; let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(237448); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "McFlow_SetSlot", &PROBE_N, &[], true); }
    r_
}

pub fn McFlow_PollCard(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "McFlow_PollCard", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if (((((F::McCard_GetInfo(c, 0i32)) == (0i32)) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(12)) as i32)) != (2i32)) as i32)) != 0) || ((((({ let a0_ = (c.m.r32(((sym::gMcCardPort).wrapping_add((((0i32) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32); F::McCard_IsFatalError(c, a0_) }) != (0i32)) as i32)) != 0)) as i32)) != 0) {
        F::McCard_ResetStep(c);
        break 'f 2i32;
    }
    F::McCard_ResetStep(c);
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "McFlow_PollCard", &PROBE_N, &[], true); }
    r_
}

pub fn McFlow_Update(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "McFlow_Update", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if (((((c.m.r32(sym::gMcFlow)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
        break 'f 0i32;
    }
    if ((((({ let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(237024); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let fp_ = { let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(237024); c.m.r32(a_) }; { let _ = D::call(c, fp_, &[]); } };
    }
    break 'f { let a_ = (c.m.r32(sym::gMcFlow)).wrapping_add(36); (c.m.r32(a_) as i32) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "McFlow_Update", &PROBE_N, &[], true); }
    r_
}
