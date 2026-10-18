#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn BtlStage_HasFeature(c: &mut Ctx, mut p_kind_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_HasFeature", &PROBE_N, &[(p_kind_1) as u32], false); }
    let mut l_d_2: u32 = 0;
    let r_ = 'f: {
    if (((((((((c.m.r32(sym::gBtlStage)) == (((0i32) as u32))) as i32)) != 0) || ((((({ l_d_2 = { let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(4); c.m.r32(a_) }; l_d_2 }) == (((0i32) as u32))) as i32)) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    's1: {
        let sv3: u32 = p_kind_1;
        match sv3 {
            0u32 => {
                if ((((((c.m.r32((l_d_2).wrapping_add(64)) as i32)) != (0i32)) as i32)) != 0) {
                    if (((({ let a_ = (c.m.r32((l_d_2).wrapping_add(68))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) & (1i32))) != 0) {
                        break 'f 1i32;
                    }
                }
                break 's1;
            }
            1u32 => {
                if ((((((c.m.r32((l_d_2).wrapping_add(64)) as i32)) > (1i32)) as i32)) != 0) {
                    if (((({ let a_ = (c.m.r32((l_d_2).wrapping_add(68))).wrapping_add((((4i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) & (1i32))) != 0) {
                        break 'f 1i32;
                    }
                }
                break 's1;
            }
            2u32 => {
                if ((((((c.m.r32((l_d_2).wrapping_add(72)) as i32)) != (0i32)) as i32)) != 0) {
                    if (((({ let a_ = (c.m.r32((l_d_2).wrapping_add(76))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) & (1i32))) != 0) {
                        break 'f 1i32;
                    }
                }
                break 's1;
            }
            3u32 => {
                if ((((((c.m.r32((l_d_2).wrapping_add(80)) as i32)) != (0i32)) as i32)) != 0) {
                    if (((({ let a_ = (c.m.r32((l_d_2).wrapping_add(84))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) & (1i32))) != 0) {
                        break 'f 1i32;
                    }
                }
                break 's1;
            }
            4u32 => {
                if ((((((c.m.r32((l_d_2).wrapping_add(152)) as i32)) != (0i32)) as i32)) != 0) {
                    if (((({ let a_ = (c.m.r32((l_d_2).wrapping_add(156))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) & (1i32))) != 0) {
                        break 'f 1i32;
                    }
                }
                break 's1;
            }
            6u32 => {
                if ((((((c.m.r32((l_d_2).wrapping_add(96)) as i32)) != (0i32)) as i32)) != 0) {
                    if (((({ let a_ = (c.m.r32((l_d_2).wrapping_add(100))).wrapping_add((((4i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) & (1i32))) != 0) {
                        break 'f 1i32;
                    }
                }
                break 's1;
            }
            7u32 => {
                if ((((((c.m.r32((l_d_2).wrapping_add(144)) as i32)) != (0i32)) as i32)) != 0) {
                    if (((({ let a_ = (c.m.r32((l_d_2).wrapping_add(148))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) & (1i32))) != 0) {
                        break 'f 1i32;
                    }
                }
                break 's1;
            }
            8u32 => {
                if ((((((c.m.r32((l_d_2).wrapping_add(88)) as i32)) != (0i32)) as i32)) != 0) {
                    if ((((((({ let a_ = (c.m.r32((l_d_2).wrapping_add(92))).wrapping_add(22); c.m.r8(a_) }) as i32)) != (0i32)) as i32)) != 0) {
                        break 'f 1i32;
                    }
                }
                break 's1;
            }
            9u32 => {
                if ((((((c.m.r32((l_d_2).wrapping_add(160)) as i32)) != (0i32)) as i32)) != 0) {
                    if (((({ let a_ = (c.m.r32((l_d_2).wrapping_add(164))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) & (1i32))) != 0) {
                        break 'f 1i32;
                    }
                }
                break 's1;
            }
            10u32 => {
                if ((((((c.m.r32((l_d_2).wrapping_add(112)) as i32)) != (0i32)) as i32)) != 0) {
                    if (((({ let a_ = (c.m.r32((l_d_2).wrapping_add(116))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) & (1i32))) != 0) {
                        break 'f 1i32;
                    }
                }
                break 's1;
            }
            11u32 => {
                if ((((((c.m.r32((l_d_2).wrapping_add(112)) as i32)) != (0i32)) as i32)) != 0) {
                    if (((({ let a_ = (c.m.r32((l_d_2).wrapping_add(116))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) & (2i32))) != 0) {
                        break 'f 1i32;
                    }
                }
                break 's1;
            }
            12u32 => {
                if ((((((c.m.r32((l_d_2).wrapping_add(112)) as i32)) != (0i32)) as i32)) != 0) {
                    if (((({ let a_ = (c.m.r32((l_d_2).wrapping_add(116))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) & (4i32))) != 0) {
                        break 'f 1i32;
                    }
                }
                break 's1;
            }
            13u32 => {
                break 'f F::BtlStage_HasFlag8(c);
            }
            _ => {
            }
        }
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_HasFeature", &PROBE_N, &[], true); }
    r_
}

pub fn BtlStage_GetFileMember(c: &mut Ctx, mut p_index_1: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_GetFileMember", &PROBE_N, &[(p_index_1) as u32], false); }
    let mut l_file_2: u32 = 0;
    let mut l_ret_3: u32 = 0;
    let r_ = 'f: {
    l_file_2 = { let a_ = ((c.m.r32(sym::gCommonRes)).wrapping_add(32)).wrapping_add(4); c.m.r32(a_) };
    l_ret_3 = ((0i32) as u32);
    if (((((c.m.r32(((l_file_2).wrapping_add((((p_index_1) as u32)).wrapping_mul(4))).wrapping_add((((4i32) as u32)).wrapping_mul(4)))) != (c.m.r32(((l_file_2).wrapping_add((((p_index_1) as u32)).wrapping_mul(4))).wrapping_add((((3i32) as u32)).wrapping_mul(4))))) as i32)) != 0) {
        l_ret_3 = (l_file_2).wrapping_add(((c.m.r32(((l_file_2).wrapping_add((((p_index_1) as u32)).wrapping_mul(4))).wrapping_add((((3i32) as u32)).wrapping_mul(4)))).wrapping_shr((2i32) as u32)).wrapping_mul(4));
    }
    break 'f l_ret_3;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_GetFileMember", &PROBE_N, &[], true); }
    r_
}

pub fn BtlStage_GetFxResA(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_GetFxResA", &PROBE_N, &[], false); }
    let mut l_d_1: u32 = 0;
    let mut l_ret_2: u32 = 0;
    let mut bb_: u32 = 0;
    let r_ = 'f: {
    'fl1: loop {
        match bb_ {
            0 => {
                l_ret_2 = ((0i32) as u32);
                if (((((c.m.r32(sym::gBtlStage)) != (((0i32) as u32))) as i32)) != 0) { bb_ = 1; continue 'fl1; } else { bb_ = 2; continue 'fl1; }
            }
            1 => {
                l_d_1 = { let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(4); c.m.r32(a_) };
                if (((((l_d_1) != (((0i32) as u32))) as i32)) != 0) { bb_ = 3; continue 'fl1; } else { bb_ = 4; continue 'fl1; }
            }
            2 => {
                break 'f l_ret_2;
            }
            3 => {
                if ((((!(((({ let a_ = (c.m.r32((l_d_1).wrapping_add(116))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) & (1i32))) != 0)) as i32)) != 0) { bb_ = 5; continue 'fl1; } else { bb_ = 6; continue 'fl1; }
            }
            4 => {
                bb_ = 2; continue 'fl1;
            }
            5 => {
                bb_ = 7; continue 'fl1;
            }
            6 => {
                if (((((c.m.r32(sym::gBtlStageFxRes)) == (((0i32) as u32))) as i32)) != 0) { bb_ = 9; continue 'fl1; } else { bb_ = 10; continue 'fl1; }
            }
            7 => {
                break 'f ((0i32) as u32);
            }
            8 => {
                bb_ = 6; continue 'fl1;
            }
            9 => {
                bb_ = 7; continue 'fl1;
            }
            10 => {
                l_ret_2 = { let a_ = (c.m.r32(sym::gBtlStageFxRes)).wrapping_add(8); c.m.r32(a_) };
                bb_ = 4; continue 'fl1;
            }
            11 => {
                bb_ = 10; continue 'fl1;
            }
            12 => {
                bb_ = 7; continue 'fl1;
            }
            13 => {
                break 'fl1;
            }
            _ => unreachable!(),
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_GetFxResA", &PROBE_N, &[], true); }
    r_
}

pub fn BtlStage_GetFxResB(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_GetFxResB", &PROBE_N, &[], false); }
    let mut l_d_1: u32 = 0;
    let mut l_ret_2: u32 = 0;
    let mut bb_: u32 = 0;
    let r_ = 'f: {
    'fl1: loop {
        match bb_ {
            0 => {
                l_ret_2 = ((0i32) as u32);
                if (((((c.m.r32(sym::gBtlStage)) != (((0i32) as u32))) as i32)) != 0) { bb_ = 1; continue 'fl1; } else { bb_ = 2; continue 'fl1; }
            }
            1 => {
                l_d_1 = { let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(4); c.m.r32(a_) };
                if (((((l_d_1) != (((0i32) as u32))) as i32)) != 0) { bb_ = 3; continue 'fl1; } else { bb_ = 4; continue 'fl1; }
            }
            2 => {
                break 'f l_ret_2;
            }
            3 => {
                if ((((!(((({ let a_ = (c.m.r32((l_d_1).wrapping_add(116))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) & (2i32))) != 0)) as i32)) != 0) { bb_ = 5; continue 'fl1; } else { bb_ = 6; continue 'fl1; }
            }
            4 => {
                bb_ = 2; continue 'fl1;
            }
            5 => {
                bb_ = 7; continue 'fl1;
            }
            6 => {
                if (((((c.m.r32(sym::gBtlStageFxRes)) == (((0i32) as u32))) as i32)) != 0) { bb_ = 9; continue 'fl1; } else { bb_ = 10; continue 'fl1; }
            }
            7 => {
                break 'f ((0i32) as u32);
            }
            8 => {
                bb_ = 6; continue 'fl1;
            }
            9 => {
                bb_ = 7; continue 'fl1;
            }
            10 => {
                l_ret_2 = { let a_ = (c.m.r32(sym::gBtlStageFxRes)).wrapping_add(16); c.m.r32(a_) };
                bb_ = 4; continue 'fl1;
            }
            11 => {
                bb_ = 10; continue 'fl1;
            }
            12 => {
                bb_ = 7; continue 'fl1;
            }
            13 => {
                break 'fl1;
            }
            _ => unreachable!(),
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_GetFxResB", &PROBE_N, &[], true); }
    r_
}

pub fn BtlStage_GetFxResC(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_GetFxResC", &PROBE_N, &[], false); }
    let mut l_d_1: u32 = 0;
    let mut l_ret_2: u32 = 0;
    let mut bb_: u32 = 0;
    let r_ = 'f: {
    'fl1: loop {
        match bb_ {
            0 => {
                l_ret_2 = ((0i32) as u32);
                if (((((c.m.r32(sym::gBtlStage)) != (((0i32) as u32))) as i32)) != 0) { bb_ = 1; continue 'fl1; } else { bb_ = 2; continue 'fl1; }
            }
            1 => {
                l_d_1 = { let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(4); c.m.r32(a_) };
                if (((((l_d_1) != (((0i32) as u32))) as i32)) != 0) { bb_ = 3; continue 'fl1; } else { bb_ = 4; continue 'fl1; }
            }
            2 => {
                break 'f l_ret_2;
            }
            3 => {
                if ((((!(((({ let a_ = (c.m.r32((l_d_1).wrapping_add(116))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) & (4i32))) != 0)) as i32)) != 0) { bb_ = 5; continue 'fl1; } else { bb_ = 6; continue 'fl1; }
            }
            4 => {
                bb_ = 2; continue 'fl1;
            }
            5 => {
                bb_ = 7; continue 'fl1;
            }
            6 => {
                if (((((c.m.r32(sym::gBtlStageFxRes)) == (((0i32) as u32))) as i32)) != 0) { bb_ = 9; continue 'fl1; } else { bb_ = 10; continue 'fl1; }
            }
            7 => {
                break 'f ((0i32) as u32);
            }
            8 => {
                bb_ = 6; continue 'fl1;
            }
            9 => {
                bb_ = 7; continue 'fl1;
            }
            10 => {
                l_ret_2 = { let a_ = (c.m.r32(sym::gBtlStageFxRes)).wrapping_add(24); c.m.r32(a_) };
                bb_ = 4; continue 'fl1;
            }
            11 => {
                bb_ = 10; continue 'fl1;
            }
            12 => {
                bb_ = 7; continue 'fl1;
            }
            13 => {
                break 'fl1;
            }
            _ => unreachable!(),
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_GetFxResC", &PROBE_N, &[], true); }
    r_
}

pub fn BtlStage_GetFxResA2(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_GetFxResA2", &PROBE_N, &[], false); }
    let mut l_d_1: u32 = 0;
    let mut l_res_2: u32 = 0;
    let mut l_ret_3: u32 = 0;
    let mut bb_: u32 = 0;
    let r_ = 'f: {
    'fl1: loop {
        match bb_ {
            0 => {
                l_ret_3 = ((0i32) as u32);
                if (((((c.m.r32(sym::gBtlStage)) != (((0i32) as u32))) as i32)) != 0) { bb_ = 1; continue 'fl1; } else { bb_ = 2; continue 'fl1; }
            }
            1 => {
                l_d_1 = { let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(4); c.m.r32(a_) };
                if (((((l_d_1) != (((0i32) as u32))) as i32)) != 0) { bb_ = 3; continue 'fl1; } else { bb_ = 4; continue 'fl1; }
            }
            2 => {
                break 'f l_ret_3;
            }
            3 => {
                if ((((!(((({ let a_ = (c.m.r32((l_d_1).wrapping_add(116))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) & (1i32))) != 0)) as i32)) != 0) { bb_ = 5; continue 'fl1; } else { bb_ = 6; continue 'fl1; }
            }
            4 => {
                bb_ = 2; continue 'fl1;
            }
            5 => {
                bb_ = 7; continue 'fl1;
            }
            6 => {
                l_res_2 = c.m.r32(sym::gBtlStageFxRes);
                if (((((l_res_2) == (((0i32) as u32))) as i32)) != 0) { bb_ = 9; continue 'fl1; } else { bb_ = 10; continue 'fl1; }
            }
            7 => {
                break 'f ((0i32) as u32);
            }
            8 => {
                bb_ = 6; continue 'fl1;
            }
            9 => {
                bb_ = 7; continue 'fl1;
            }
            10 => {
                if (((((c.m.r32((l_res_2).wrapping_add(8))) == (((0i32) as u32))) as i32)) != 0) { bb_ = 12; continue 'fl1; } else { bb_ = 13; continue 'fl1; }
            }
            11 => {
                bb_ = 10; continue 'fl1;
            }
            12 => {
                bb_ = 7; continue 'fl1;
            }
            13 => {
                l_ret_3 = c.m.r32((l_res_2).wrapping_add(12));
                bb_ = 4; continue 'fl1;
            }
            14 => {
                bb_ = 13; continue 'fl1;
            }
            15 => {
                bb_ = 7; continue 'fl1;
            }
            16 => {
                break 'fl1;
            }
            _ => unreachable!(),
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_GetFxResA2", &PROBE_N, &[], true); }
    r_
}

pub fn BtlStage_GetFxResB2(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_GetFxResB2", &PROBE_N, &[], false); }
    let mut l_d_1: u32 = 0;
    let mut l_res_2: u32 = 0;
    let mut l_ret_3: u32 = 0;
    let mut bb_: u32 = 0;
    let r_ = 'f: {
    'fl1: loop {
        match bb_ {
            0 => {
                l_ret_3 = ((0i32) as u32);
                if (((((c.m.r32(sym::gBtlStage)) != (((0i32) as u32))) as i32)) != 0) { bb_ = 1; continue 'fl1; } else { bb_ = 2; continue 'fl1; }
            }
            1 => {
                l_d_1 = { let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(4); c.m.r32(a_) };
                if (((((l_d_1) != (((0i32) as u32))) as i32)) != 0) { bb_ = 3; continue 'fl1; } else { bb_ = 4; continue 'fl1; }
            }
            2 => {
                break 'f l_ret_3;
            }
            3 => {
                if ((((!(((({ let a_ = (c.m.r32((l_d_1).wrapping_add(116))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) & (2i32))) != 0)) as i32)) != 0) { bb_ = 5; continue 'fl1; } else { bb_ = 6; continue 'fl1; }
            }
            4 => {
                bb_ = 2; continue 'fl1;
            }
            5 => {
                bb_ = 7; continue 'fl1;
            }
            6 => {
                l_res_2 = c.m.r32(sym::gBtlStageFxRes);
                if (((((l_res_2) == (((0i32) as u32))) as i32)) != 0) { bb_ = 9; continue 'fl1; } else { bb_ = 10; continue 'fl1; }
            }
            7 => {
                break 'f ((0i32) as u32);
            }
            8 => {
                bb_ = 6; continue 'fl1;
            }
            9 => {
                bb_ = 7; continue 'fl1;
            }
            10 => {
                if (((((c.m.r32((l_res_2).wrapping_add(16))) == (((0i32) as u32))) as i32)) != 0) { bb_ = 12; continue 'fl1; } else { bb_ = 13; continue 'fl1; }
            }
            11 => {
                bb_ = 10; continue 'fl1;
            }
            12 => {
                bb_ = 7; continue 'fl1;
            }
            13 => {
                l_ret_3 = c.m.r32((l_res_2).wrapping_add(20));
                bb_ = 4; continue 'fl1;
            }
            14 => {
                bb_ = 13; continue 'fl1;
            }
            15 => {
                bb_ = 7; continue 'fl1;
            }
            16 => {
                break 'fl1;
            }
            _ => unreachable!(),
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_GetFxResB2", &PROBE_N, &[], true); }
    r_
}

pub fn BtlStage_GetFxResC2(c: &mut Ctx) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_GetFxResC2", &PROBE_N, &[], false); }
    let mut l_d_1: u32 = 0;
    let mut l_res_2: u32 = 0;
    let mut l_ret_3: u32 = 0;
    let mut bb_: u32 = 0;
    let r_ = 'f: {
    'fl1: loop {
        match bb_ {
            0 => {
                l_ret_3 = ((0i32) as u32);
                if (((((c.m.r32(sym::gBtlStage)) != (((0i32) as u32))) as i32)) != 0) { bb_ = 1; continue 'fl1; } else { bb_ = 2; continue 'fl1; }
            }
            1 => {
                l_d_1 = { let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(4); c.m.r32(a_) };
                if (((((l_d_1) != (((0i32) as u32))) as i32)) != 0) { bb_ = 3; continue 'fl1; } else { bb_ = 4; continue 'fl1; }
            }
            2 => {
                break 'f l_ret_3;
            }
            3 => {
                if ((((!(((({ let a_ = (c.m.r32((l_d_1).wrapping_add(116))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) & (4i32))) != 0)) as i32)) != 0) { bb_ = 5; continue 'fl1; } else { bb_ = 6; continue 'fl1; }
            }
            4 => {
                bb_ = 2; continue 'fl1;
            }
            5 => {
                bb_ = 7; continue 'fl1;
            }
            6 => {
                l_res_2 = c.m.r32(sym::gBtlStageFxRes);
                if (((((l_res_2) == (((0i32) as u32))) as i32)) != 0) { bb_ = 9; continue 'fl1; } else { bb_ = 10; continue 'fl1; }
            }
            7 => {
                break 'f ((0i32) as u32);
            }
            8 => {
                bb_ = 6; continue 'fl1;
            }
            9 => {
                bb_ = 7; continue 'fl1;
            }
            10 => {
                if (((((c.m.r32((l_res_2).wrapping_add(24))) == (((0i32) as u32))) as i32)) != 0) { bb_ = 12; continue 'fl1; } else { bb_ = 13; continue 'fl1; }
            }
            11 => {
                bb_ = 10; continue 'fl1;
            }
            12 => {
                bb_ = 7; continue 'fl1;
            }
            13 => {
                l_ret_3 = c.m.r32((l_res_2).wrapping_add(28));
                bb_ = 4; continue 'fl1;
            }
            14 => {
                bb_ = 13; continue 'fl1;
            }
            15 => {
                bb_ = 7; continue 'fl1;
            }
            16 => {
                break 'fl1;
            }
            _ => unreachable!(),
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_GetFxResC2", &PROBE_N, &[], true); }
    r_
}

pub fn BtlStage_HasFlag8(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_HasFlag8", &PROBE_N, &[], false); }
    let mut l_d_1: u32 = 0;
    let mut l_ret_2: i32 = 0;
    let mut l_bits_3: i32 = 0;
    let r_ = 'f: {
    l_ret_2 = 0i32;
    if (((((c.m.r32(sym::gBtlStage)) != (((0i32) as u32))) as i32)) != 0) {
        l_d_1 = { let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(4); c.m.r32(a_) };
        if (((((l_d_1) != (((0i32) as u32))) as i32)) != 0) {
            l_bits_3 = (({ let a_ = (c.m.r32((l_d_1).wrapping_add(116))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) & (8i32));
            l_ret_2 = (((l_bits_3) != (0i32)) as i32);
        }
    }
    break 'f l_ret_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_HasFlag8", &PROBE_N, &[], true); }
    r_
}

pub fn BtlStage_HasMoon(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_HasMoon", &PROBE_N, &[], false); }
    let mut l_d_1: u32 = 0;
    let mut l_ret_2: i32 = 0;
    let mut l_bits_3: i32 = 0;
    let r_ = 'f: {
    l_ret_2 = 0i32;
    if (((((c.m.r32(sym::gBtlStage)) != (((0i32) as u32))) as i32)) != 0) {
        l_d_1 = { let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(4); c.m.r32(a_) };
        if (((((l_d_1) != (((0i32) as u32))) as i32)) != 0) {
            l_bits_3 = (({ let a_ = (c.m.r32((l_d_1).wrapping_add(116))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) & (16i32));
            l_ret_2 = (((l_bits_3) != (0i32)) as i32);
        }
    }
    break 'f l_ret_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_HasMoon", &PROBE_N, &[], true); }
    r_
}

pub fn BtlStage_GetAmbient(c: &mut Ctx, mut p_rgba_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_GetAmbient", &PROBE_N, &[(p_rgba_1) as u32], false); }
    let mut l_d_2: u32 = 0;
    let mut l_c_3: u32 = 0;
    let r_ = 'f: {
    if (((((c.m.r32(sym::gBtlStage)) == (((0i32) as u32))) as i32)) != 0) {
        { let v_ = 128i32; let a_ = (p_rgba_1).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = 128i32; let a_ = (p_rgba_1).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = 128i32; let a_ = (p_rgba_1).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = 128i32; let a_ = (p_rgba_1).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        break 'f 0i32;
    }
    if (((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((8192i32) as u64)))) != 0) {
        { let v_ = 128i32; let a_ = (p_rgba_1).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = 128i32; let a_ = (p_rgba_1).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = 128i32; let a_ = (p_rgba_1).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = 128i32; let a_ = (p_rgba_1).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        break 'f 0i32;
    }
    l_d_2 = { let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(4); c.m.r32(a_) };
    if (((((l_d_2) == (((0i32) as u32))) as i32)) != 0) {
        { let v_ = 128i32; let a_ = (p_rgba_1).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = 128i32; let a_ = (p_rgba_1).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = 128i32; let a_ = (p_rgba_1).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = 128i32; let a_ = (p_rgba_1).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        break 'f 0i32;
    }
    l_c_3 = c.m.r32((l_d_2).wrapping_add(140));
    { let v_ = ((c.m.r8((l_c_3).wrapping_add(((0i32) as u32)))) as i32); let a_ = (p_rgba_1).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    { let v_ = ((c.m.r8((l_c_3).wrapping_add(((1i32) as u32)))) as i32); let a_ = (p_rgba_1).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    { let v_ = ((c.m.r8((l_c_3).wrapping_add(((2i32) as u32)))) as i32); let a_ = (p_rgba_1).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_GetAmbient", &PROBE_N, &[], true); }
    r_
}

pub fn BtlStage_SetAmbient(c: &mut Ctx, mut p_rgba_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_SetAmbient", &PROBE_N, &[(p_rgba_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_d_2: u32 = 0;
    let mut l_c_3: u32 = 0;
    let r_ = 'f: {
        { let v_ = c.m.r64((p_rgba_1).wrapping_add((((0i32) as u32)).wrapping_mul(4))); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w64(a_, v_); };
        { let v_ = c.m.r64((p_rgba_1).wrapping_add((((2i32) as u32)).wrapping_mul(4))); let a_ = (fp).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w64(a_, v_); };
        if (((((c.m.r32(sym::gBtlStage)) == (((0i32) as u32))) as i32)) != 0) {
            break 'f 0i32;
        }
        l_d_2 = { let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(4); c.m.r32(a_) };
        if (((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((8192i32) as u64)))) != 0) {
            break 'f 0i32;
        }
        if (((((l_d_2) == (((0i32) as u32))) as i32)) != 0) {
            break 'f 0i32;
        }
        l_c_3 = c.m.r32((l_d_2).wrapping_add(140));
        { let v_ = (((c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) as u8); let a_ = (l_c_3).wrapping_add(((0i32) as u32)); c.m.w8(a_, v_); };
        { let v_ = (((c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) as u8); let a_ = (l_c_3).wrapping_add(((1i32) as u32)); c.m.w8(a_, v_); };
        { let v_ = (((c.m.r32((fp).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32)) as u8); let a_ = (l_c_3).wrapping_add(((2i32) as u32)); c.m.w8(a_, v_); };
        break 'f 1i32;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_SetAmbient", &PROBE_N, &[], true); }
    r_
}

pub fn BtlStage_GetId(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_GetId", &PROBE_N, &[], false); }
    let mut l_d_1: u32 = 0;
    let r_ = 'f: {
    if (((((c.m.r32(sym::gBtlStage)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f (1i32).wrapping_neg();
    }
    if (((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((8192i32) as u64)))) != 0) {
        break 'f (1i32).wrapping_neg();
    }
    l_d_1 = { let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(4); c.m.r32(a_) };
    if (((((l_d_1) == (((0i32) as u32))) as i32)) != 0) {
        break 'f (1i32).wrapping_neg();
    }
    break 'f { let a_ = c.m.r32((l_d_1).wrapping_add(20)); (c.m.r32(a_) as i32) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_GetId", &PROBE_N, &[], true); }
    r_
}

pub fn BtlStage_GetLightDir(c: &mut Ctx, mut p_out_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_GetLightDir", &PROBE_N, &[(p_out_1) as u32], false); }
    let mut l_d_2: u32 = 0;
    let mut l_dir_3: u32 = 0;
    let r_ = 'f: {
    if (((((c.m.r32(sym::gBtlStage)) == (((0i32) as u32))) as i32)) != 0) {
        F::Vec4_Copy(c, p_out_1, sym::D_002C6C10);
        break 'f (1i32).wrapping_neg();
    }
    if (((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((8192i32) as u64)))) != 0) {
        F::Vec4_Copy(c, p_out_1, sym::D_002C6C10);
        break 'f (1i32).wrapping_neg();
    }
    l_d_2 = { let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(4); c.m.r32(a_) };
    if (((((l_d_2) == (((0i32) as u32))) as i32)) != 0) {
        F::Vec4_Copy(c, p_out_1, sym::D_002C6C10);
        break 'f (1i32).wrapping_neg();
    }
    l_dir_3 = c.m.r32((l_d_2).wrapping_add(36));
    F::Vec4_Copy(c, p_out_1, l_dir_3);
    F::Vec4_Copy(c, sym::D_002C6C10, l_dir_3);
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_GetLightDir", &PROBE_N, &[], true); }
    r_
}

pub fn BtlStage_GetPlace2C(c: &mut Ctx, mut p_pos_1: u32, mut p_rot_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_GetPlace2C", &PROBE_N, &[(p_pos_1) as u32, (p_rot_2) as u32], false); }
    let mut l_d_3: u32 = 0;
    let mut l_b_4: u32 = 0;
    let mut l_yaw_5: F32 = F32(0);
    let r_ = 'f: {
    if (((((c.m.r32(sym::gBtlStage)) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    if (((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((8192i32) as u64)))) != 0) {
        break 'f 0i32;
    }
    l_d_3 = { let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(4); c.m.r32(a_) };
    if (((((l_d_3) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    l_b_4 = c.m.r32((l_d_3).wrapping_add(44));
    l_yaw_5 = { let a0_ = F32(c.m.r32((l_b_4).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a1_ = F32(c.m.r32((l_b_4).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); let a2_ = F32(c.m.r32((l_b_4).wrapping_add((((4i32) as u32)).wrapping_mul(4)))); let a3_ = F32(c.m.r32((l_b_4).wrapping_add((((6i32) as u32)).wrapping_mul(4)))); F::Stg_CalcYaw(c, a0_, a1_, a2_, a3_) };
    { let v_ = F32(c.m.r32((l_b_4).wrapping_add((((0i32) as u32)).wrapping_mul(4)))); let a_ = p_pos_1; c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x00000000); let a_ = (p_pos_1).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((l_b_4).wrapping_add((((2i32) as u32)).wrapping_mul(4)))); let a_ = (p_pos_1).wrapping_add(8); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x3f800000); let a_ = (p_pos_1).wrapping_add(12); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x3f800000); let a_ = (p_rot_2).wrapping_add(12); c.m.w32(a_, (v_).0); };
    { let v_ = l_yaw_5; let a_ = (p_rot_2).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x00000000); let a_ = (p_rot_2).wrapping_add(8); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x00000000); let a_ = p_rot_2; c.m.w32(a_, (v_).0); };
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_GetPlace2C", &PROBE_N, &[], true); }
    r_
}

pub fn BtlStage_GetChangeTarget(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_GetChangeTarget", &PROBE_N, &[], false); }
    let mut l_kind_1: i32 = 0;
    let r_ = 'f: {
    if ((((!((F::BtlStage_IsReady(c)) != 0)) as i32)) != 0) {
        break 'f (1i32).wrapping_neg();
    }
    l_kind_1 = { let a_ = ({ let a_ = ({ let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(4); c.m.r32(a_) }).wrapping_add(20); c.m.r32(a_) }).wrapping_add(12); (c.m.r32(a_) as i32) };
    if (((((l_kind_1) == (1i32)) as i32)) != 0) {
        break 'f 15i32;
    }
    if (((((l_kind_1) == (2i32)) as i32)) != 0) {
        break 'f 3i32;
    }
    break 'f (1i32).wrapping_neg();
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_GetChangeTarget", &PROBE_N, &[], true); }
    r_
}

pub fn BtlStage_UpdateTimers(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_UpdateTimers", &PROBE_N, &[], false); }
    let mut l_i_1: u32 = 0;
    let mut l_base_2: u32 = 0;
    let mut l_t_3: u32 = 0;
    let r_ = 'f: {
    if (((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0) {
        break 'f;
    }
    if ((((({ let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(16); c.m.r32(a_) }) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    l_base_2 = { let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(20); c.m.r32(a_) };
    {
        l_i_1 = ((0i32) as u32);
        'l1: loop {
            if !(((((l_i_1) < ({ let a_ = (c.m.r32(sym::gBtlStage)).wrapping_add(16); c.m.r32(a_) })) as i32)) != 0) { break 'l1; }
            'c2: {
                l_t_3 = (l_base_2).wrapping_add((l_i_1).wrapping_mul(16));
                { let r_ = F32(0x3e088888); let a_ = (l_t_3).wrapping_add(8); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                { let r_ = (!(4i32)); let a_ = l_t_3; let n_ = ((((((c.m.r16(a_)) as i32)) & (r_))) as u16); c.m.w16(a_, n_); };
                if (((rt::flt(F32(c.m.r32((l_t_3).wrapping_add(12))), F32(c.m.r32((l_t_3).wrapping_add(8)))) as i32)) != 0) {
                    { let r_ = 4i32; let a_ = l_t_3; let n_ = ((((((c.m.r16(a_)) as i32)) | (r_))) as u16); c.m.w16(a_, n_); };
                    { let v_ = F32(0x00000000); let a_ = (l_t_3).wrapping_add(8); c.m.w32(a_, (v_).0); };
                }
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_UpdateTimers", &PROBE_N, &[], true); }
    r_
}

pub fn BtlStage_Update(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_Update", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if (((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((8192i32) as u64)))) != 0) {
        break 'f;
    }
    F::BtlStage_UpdateTimers(c);
    F::BtlStage_UpdateObjs(c);
    F::StgAmb_Update(c);
    F::StgModel_UpdateAnims(c);
    F::StgRigid_Update(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlStage_Update", &PROBE_N, &[], true); }
    r_
}

pub fn StgAmb_SetBgmVolume(c: &mut Ctx, mut p_volume_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgAmb_SetBgmVolume", &PROBE_N, &[(p_volume_1) as u32], false); }
    let r_ = 'f: {
    { let a0_ = rt::f2i(rt::fmul(rt::fmul(rt::i2f(p_volume_1), F32(c.m.r32(sym::gStgAmb))), F32(0x3dcccccc))); F::MapBgm_SetVolume(c, a0_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgAmb_SetBgmVolume", &PROBE_N, &[], true); }
    r_
}

pub fn StgAmb_PlaySe(c: &mut Ctx, mut p_mask_1: u32, mut p_id_2: i32, mut p_volume_3: i32, mut p_pan_4: i32, mut p_pitch_5: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgAmb_PlaySe", &PROBE_N, &[(p_mask_1) as u32, (p_id_2) as u32, (p_volume_3) as u32, (p_pan_4) as u32, (p_pitch_5) as u32], false); }
    let r_ = 'f: {
    break 'f { let a2_ = rt::f2i(rt::fmul(rt::i2f(p_volume_3), F32(c.m.r32((sym::gStgAmb).wrapping_add(8))))); F::Snd_PlaySeEx(c, p_mask_1, p_id_2, a2_, p_pan_4, p_pitch_5) };
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "StgAmb_PlaySe", &PROBE_N, &[], true); }
    r_
}

pub fn StgAmb_Falloff(c: &mut Ctx, mut p_x_1: F32, mut p_near_2: F32, mut p_far_3: F32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgAmb_Falloff", &PROBE_N, &[(p_x_1).0, (p_near_2).0, (p_far_3).0], false); }
    let mut l_ret_4: F32 = F32(0);
    let r_ = 'f: {
    if (((rt::flt(p_far_3, p_x_1) as i32)) != 0) {
        l_ret_4 = F32(0x00000000);
    } else {
        if (((rt::flt(p_near_2, p_x_1) as i32)) != 0) {
            l_ret_4 = rt::fsub(F32(0x3f800000), rt::fdiv(rt::fsub(p_x_1, p_near_2), rt::fsub(p_far_3, p_near_2)));
        } else {
            l_ret_4 = F32(0x3f800000);
        }
    }
    break 'f l_ret_4;
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "StgAmb_Falloff", &PROBE_N, &[], true); }
    r_
}

pub fn StgAmb_Update00(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgAmb_Update00", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::StgAmb_SetBgmVolume(c, 80i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgAmb_Update00", &PROBE_N, &[], true); }
    r_
}

pub fn StgAmb_Update01(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgAmb_Update01", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::StgAmb_SetBgmVolume(c, 80i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgAmb_Update01", &PROBE_N, &[], true); }
    r_
}

pub fn StgAmb_Update02(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgAmb_Update02", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(80) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 80);
    let mut l_view_1: u32 = 0;
    let mut l_vol_2: F32 = F32(0);
    let r_ = 'f: {
        l_view_1 = c.m.r32(sym::gBtlCamView);
        if (((((l_view_1) != (((0i32) as u32))) as i32)) != 0) {
            F::Mtx_InverseRT(c, fp, (l_view_1).wrapping_add(64));
            { let _ = F::BtlStage_GetWaterLevel(c, fp.wrapping_add(64)); };
            if (((rt::flt(F32(c.m.r32(((fp).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((fp.wrapping_add(64)).wrapping_add((((0i32) as u32)).wrapping_mul(4))))) as i32)) != 0) {
                l_vol_2 = rt::fmul({ let a0_ = rt::fsub(F32(c.m.r32((fp.wrapping_add(64)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((fp).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); F::StgAmb_Falloff(c, a0_, F32(0x41a00000), F32(0x43340000)) }, F32(0x43000000));
            } else {
                l_vol_2 = rt::fmul({ let a0_ = rt::fsub(F32(c.m.r32(((fp).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((fp.wrapping_add(64)).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); F::StgAmb_Falloff(c, a0_, F32(0x41a00000), F32(0x437a0000)) }, F32(0x42c00000));
            }
            F::StgAmb_SetBgmVolume(c, rt::f2i(l_vol_2));
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "StgAmb_Update02", &PROBE_N, &[], true); }
    r_
}

pub fn StgAmb_Update03(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgAmb_Update03", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(80) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 80);
    let mut l_amb_1: u32 = 0;
    let mut l_view_2: u32 = 0;
    let mut l_vol_3: i32 = 0;
    let mut l_pan_4: i32 = 0;
    let mut l_x_5: F32 = F32(0);
    let r_ = 'f: {
        l_amb_1 = sym::gStgAmb;
        l_view_2 = c.m.r32(sym::gBtlCamView);
        if ((((((c.m.r32((l_amb_1).wrapping_add(12)) as i32)) == (0i32)) as i32)) != 0) {
            { let v_ = (rt::rem_i32(F::Rand_IntRange(c, 0i32, 65535i32), 90i32)).wrapping_add(90i32); let a_ = (l_amb_1).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        }
        { let a_ = (l_amb_1).wrapping_add(12); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
        if ((((((c.m.r32((l_amb_1).wrapping_add(12)) as i32)) <= (0i32)) as i32)) != 0) {
            { let v_ = (rt::rem_i32(F::Rand_IntRange(c, 0i32, 65535i32), 90i32)).wrapping_add(90i32); let a_ = (l_amb_1).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
            l_vol_3 = F::Rand_IntRange(c, 63i32, 127i32);
            l_pan_4 = F::Rand_IntRange(c, (15i32).wrapping_neg(), 15i32);
            { let _ = F::StgAmb_PlaySe(c, ((8i32) as u32), 10i32, l_vol_3, l_pan_4, 0i32); };
        }
        if (((((l_view_2) != (((0i32) as u32))) as i32)) != 0) {
            F::Mtx_InverseRT(c, fp, (l_view_2).wrapping_add(64));
            { let _ = F::BtlStage_GetWaterLevel(c, fp.wrapping_add(64)); };
            if (((rt::flt(F32(c.m.r32(((fp).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((fp.wrapping_add(64)).wrapping_add((((0i32) as u32)).wrapping_mul(4))))) as i32)) != 0) {
                l_x_5 = rt::fsub(F32(c.m.r32((fp.wrapping_add(64)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((fp).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))));
            } else {
                l_x_5 = rt::fsub(F32(c.m.r32(((fp).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((fp.wrapping_add(64)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))));
            }
            { let a0_ = rt::f2i(rt::fmul(F::StgAmb_Falloff(c, l_x_5, F32(0x00000000), F32(0x442f0000)), F32(0x43000000))); F::StgAmb_SetBgmVolume(c, a0_) };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "StgAmb_Update03", &PROBE_N, &[], true); }
    r_
}

pub fn StgAmb_Update04(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgAmb_Update04", &PROBE_N, &[], false); }
    let mut l_amb_1: u32 = 0;
    let mut l_vol_2: i32 = 0;
    let mut l_pan_3: i32 = 0;
    let r_ = 'f: {
    l_amb_1 = sym::gStgAmb;
    if ((((((c.m.r32((l_amb_1).wrapping_add(12)) as i32)) == (0i32)) as i32)) != 0) {
        { let v_ = (rt::rem_i32(F::Rand_IntRange(c, 0i32, 65535i32), 90i32)).wrapping_add(90i32); let a_ = (l_amb_1).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
    }
    { let a_ = (l_amb_1).wrapping_add(12); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
    if ((((((c.m.r32((l_amb_1).wrapping_add(12)) as i32)) <= (0i32)) as i32)) != 0) {
        { let v_ = (rt::rem_i32(F::Rand_IntRange(c, 0i32, 65535i32), 90i32)).wrapping_add(90i32); let a_ = (l_amb_1).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        l_vol_2 = F::Rand_IntRange(c, 63i32, 127i32);
        l_pan_3 = F::Rand_IntRange(c, (15i32).wrapping_neg(), 15i32);
        { let _ = F::StgAmb_PlaySe(c, ((8i32) as u32), 10i32, (l_vol_2).wrapping_shr((2i32) as u32), l_pan_3, 0i32); };
    }
    F::StgAmb_SetBgmVolume(c, 63i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgAmb_Update04", &PROBE_N, &[], true); }
    r_
}

pub fn StgAmb_Update05(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgAmb_Update05", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::StgAmb_SetBgmVolume(c, 80i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgAmb_Update05", &PROBE_N, &[], true); }
    r_
}

pub fn StgAmb_Update06(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgAmb_Update06", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::StgAmb_SetBgmVolume(c, 80i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgAmb_Update06", &PROBE_N, &[], true); }
    r_
}

pub fn StgAmb_Update07(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgAmb_Update07", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::StgAmb_SetBgmVolume(c, 64i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgAmb_Update07", &PROBE_N, &[], true); }
    r_
}

pub fn StgAmb_Update08(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgAmb_Update08", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::StgAmb_SetBgmVolume(c, 80i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgAmb_Update08", &PROBE_N, &[], true); }
    r_
}

pub fn StgAmb_Update09(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgAmb_Update09", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::StgAmb_SetBgmVolume(c, 128i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgAmb_Update09", &PROBE_N, &[], true); }
    r_
}

pub fn StgAmb_Update10(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgAmb_Update10", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(224) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 224);
    let mut l_amb_1: u32 = 0;
    let mut l_view_2: u32 = 0;
    let mut l_k_3: F32 = F32(0);
    let mut l_best_4: F32 = F32(0);
    let mut l_d_5: F32 = F32(0);
    let mut l_i_6: i32 = 0;
    let mut l_vol_7: i32 = 0;
    let mut l_pan_8: i32 = 0;
    let r_ = 'f: {
        l_amb_1 = sym::gStgAmb;
        l_view_2 = c.m.r32(sym::gBtlCamView);
        if ((((((c.m.r32((l_amb_1).wrapping_add(12)) as i32)) == (0i32)) as i32)) != 0) {
            { let v_ = (rt::rem_i32(F::Rand_IntRange(c, 0i32, 65535i32), 90i32)).wrapping_add(90i32); let a_ = (l_amb_1).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        }
        { let a_ = (l_amb_1).wrapping_add(12); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
        if ((((((c.m.r32((l_amb_1).wrapping_add(12)) as i32)) <= (0i32)) as i32)) != 0) {
            { let v_ = (rt::rem_i32(F::Rand_IntRange(c, 0i32, 65535i32), 90i32)).wrapping_add(90i32); let a_ = (l_amb_1).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
            l_vol_7 = F::Rand_IntRange(c, 63i32, 127i32);
            l_pan_8 = F::Rand_IntRange(c, (15i32).wrapping_neg(), 15i32);
            { let _ = F::StgAmb_PlaySe(c, ((8i32) as u32), 10i32, (l_vol_7).wrapping_shr((2i32) as u32), l_pan_8, 0i32); };
        }
        if (((((l_view_2) != (((0i32) as u32))) as i32)) != 0) {
            F::Mtx_InverseRT(c, fp, (l_view_2).wrapping_add(64));
            { let _ = F::BtlStage_GetWaterLevel(c, fp.wrapping_add(208)); };
            if (((rt::flt(F32(c.m.r32(((fp).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((fp.wrapping_add(208)).wrapping_add((((0i32) as u32)).wrapping_mul(4))))) as i32)) != 0) {
                l_k_3 = { let a0_ = rt::fsub(F32(c.m.r32((fp.wrapping_add(208)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((fp).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); F::StgAmb_Falloff(c, a0_, F32(0x41a00000), F32(0x43340000)) };
            } else {
                l_k_3 = { let a0_ = rt::fsub(F32(c.m.r32(((fp).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((fp.wrapping_add(208)).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); F::StgAmb_Falloff(c, a0_, F32(0x41a00000), F32(0x437a0000)) };
            }
            { let s_ = sym::stg_b__StgAmb_Update10__sPath; let d_ = fp.wrapping_add(64); rt::memcpy(c, d_, s_, 128); };
            l_best_4 = F32(0x00000000);
            F::Vec4_Copy(c, fp.wrapping_add(192), (fp).wrapping_add((((3i32) as u32)).wrapping_mul(16)));
            { let v_ = F32(0x00000000); let a_ = (fp.wrapping_add(192)).wrapping_add(4); c.m.w32(a_, (v_).0); };
            {
                l_i_6 = 0i32;
                'l1: loop {
                    if !(((((l_i_6) < (8i32)) as i32)) != 0) { break 'l1; }
                    'c2: {
                        l_d_5 = F::Vec3_Dist(c, (fp.wrapping_add(64)).wrapping_add((((l_i_6) as u32)).wrapping_mul(16)), fp.wrapping_add(192));
                        if (((((l_i_6) == (0i32)) as i32)) != 0) {
                            l_best_4 = l_d_5;
                        } else {
                            if (((rt::flt(l_d_5, l_best_4) as i32)) != 0) {
                                l_best_4 = l_d_5;
                            }
                        }
                    }
                    { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; };
                }
            }
            { let r_ = F::StgAmb_Falloff(c, l_best_4, F32(0x438c0000), F32(0x44098000)); let n_ = rt::fmul(l_k_3, r_); l_k_3 = n_; };
            F::StgAmb_SetBgmVolume(c, rt::f2i(rt::fmul(l_k_3, F32(0x42a00000))));
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "StgAmb_Update10", &PROBE_N, &[], true); }
    r_
}

pub fn StgAmb_Update11(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgAmb_Update11", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(80) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 80);
    let mut l_view_1: u32 = 0;
    let mut l_x_2: F32 = F32(0);
    let r_ = 'f: {
        l_view_1 = c.m.r32(sym::gBtlCamView);
        if (((((l_view_1) != (((0i32) as u32))) as i32)) != 0) {
            F::Mtx_InverseRT(c, fp, (l_view_1).wrapping_add(64));
            { let _ = F::BtlStage_GetWaterLevel(c, fp.wrapping_add(64)); };
            if (((rt::flt(F32(c.m.r32(((fp).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((fp.wrapping_add(64)).wrapping_add((((0i32) as u32)).wrapping_mul(4))))) as i32)) != 0) {
                l_x_2 = rt::fsub(F32(c.m.r32((fp.wrapping_add(64)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((fp).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))));
            } else {
                l_x_2 = rt::fsub(F32(c.m.r32(((fp).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((fp.wrapping_add(64)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))));
            }
            { let a0_ = rt::f2i(rt::fmul(F::StgAmb_Falloff(c, l_x_2, F32(0x41a00000), F32(0x43160000)), F32(0x43000000))); F::StgAmb_SetBgmVolume(c, a0_) };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "StgAmb_Update11", &PROBE_N, &[], true); }
    r_
}

pub fn StgAmb_Update12(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgAmb_Update12", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(80) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 80);
    let mut l_view_1: u32 = 0;
    let mut l_x_2: F32 = F32(0);
    let r_ = 'f: {
        l_view_1 = c.m.r32(sym::gBtlCamView);
        if (((((l_view_1) != (((0i32) as u32))) as i32)) != 0) {
            F::Mtx_InverseRT(c, fp, (l_view_1).wrapping_add(64));
            { let _ = F::BtlStage_GetWaterLevel(c, fp.wrapping_add(64)); };
            if (((rt::flt(F32(c.m.r32(((fp).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((fp.wrapping_add(64)).wrapping_add((((0i32) as u32)).wrapping_mul(4))))) as i32)) != 0) {
                l_x_2 = rt::fsub(F32(c.m.r32((fp.wrapping_add(64)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((fp).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))));
            } else {
                l_x_2 = rt::fsub(F32(c.m.r32(((fp).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((fp.wrapping_add(64)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))));
            }
            { let a0_ = rt::f2i(rt::fmul(F::StgAmb_Falloff(c, l_x_2, F32(0x41a00000), F32(0x43960000)), F32(0x42c00000))); F::StgAmb_SetBgmVolume(c, a0_) };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "StgAmb_Update12", &PROBE_N, &[], true); }
    r_
}

pub fn StgAmb_Update13(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgAmb_Update13", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::StgAmb_SetBgmVolume(c, 0i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgAmb_Update13", &PROBE_N, &[], true); }
    r_
}

pub fn StgAmb_Update14(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgAmb_Update14", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(80) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 80);
    let mut l_view_1: u32 = 0;
    let mut l_x_2: F32 = F32(0);
    let r_ = 'f: {
        l_view_1 = c.m.r32(sym::gBtlCamView);
        if (((((l_view_1) != (((0i32) as u32))) as i32)) != 0) {
            F::Mtx_InverseRT(c, fp, (l_view_1).wrapping_add(64));
            { let _ = F::BtlStage_GetWaterLevel(c, fp.wrapping_add(64)); };
            if (((rt::flt(F32(c.m.r32(((fp).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((fp.wrapping_add(64)).wrapping_add((((0i32) as u32)).wrapping_mul(4))))) as i32)) != 0) {
                l_x_2 = rt::fsub(F32(c.m.r32((fp.wrapping_add(64)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32(((fp).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))));
            } else {
                l_x_2 = rt::fsub(F32(c.m.r32(((fp).wrapping_add((((3i32) as u32)).wrapping_mul(16))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((fp.wrapping_add(64)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))));
            }
            { let a0_ = rt::f2i(rt::fmul(F::StgAmb_Falloff(c, l_x_2, F32(0x41a00000), F32(0x44960000)), F32(0x42c00000))); F::StgAmb_SetBgmVolume(c, a0_) };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "StgAmb_Update14", &PROBE_N, &[], true); }
    r_
}

pub fn StgAmb_Update15(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgAmb_Update15", &PROBE_N, &[], false); }
    let mut l_amb_1: u32 = 0;
    let mut l_vol_2: i32 = 0;
    let mut l_pan_3: i32 = 0;
    let r_ = 'f: {
    l_amb_1 = sym::gStgAmb;
    if ((((((c.m.r32((l_amb_1).wrapping_add(12)) as i32)) == (0i32)) as i32)) != 0) {
        { let v_ = (rt::rem_i32(F::Rand_IntRange(c, 0i32, 65536i32), 90i32)).wrapping_add(90i32); let a_ = (l_amb_1).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
    }
    { let a_ = (l_amb_1).wrapping_add(12); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
    if ((((((c.m.r32((l_amb_1).wrapping_add(12)) as i32)) <= (0i32)) as i32)) != 0) {
        { let v_ = (rt::rem_i32(F::Rand_IntRange(c, 0i32, 65536i32), 90i32)).wrapping_add(90i32); let a_ = (l_amb_1).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        l_vol_2 = F::Rand_IntRange(c, 95i32, 127i32);
        l_pan_3 = F::Rand_IntRange(c, (15i32).wrapping_neg(), 15i32);
        { let _ = F::StgAmb_PlaySe(c, ((8i32) as u32), 10i32, l_vol_2, l_pan_3, 0i32); };
    }
    F::StgAmb_SetBgmVolume(c, 96i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgAmb_Update15", &PROBE_N, &[], true); }
    r_
}

pub fn StgAmb_Update16(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgAmb_Update16", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::StgAmb_SetBgmVolume(c, 127i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgAmb_Update16", &PROBE_N, &[], true); }
    r_
}

pub fn StgAmb_Update17(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgAmb_Update17", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::StgAmb_SetBgmVolume(c, 127i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgAmb_Update17", &PROBE_N, &[], true); }
    r_
}

pub fn StgAmb_Update18(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgAmb_Update18", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::StgAmb_SetBgmVolume(c, 127i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgAmb_Update18", &PROBE_N, &[], true); }
    r_
}

pub fn StgAmb_Update19(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgAmb_Update19", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::StgAmb_SetBgmVolume(c, 127i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgAmb_Update19", &PROBE_N, &[], true); }
    r_
}

pub fn StgAmb_Update20(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgAmb_Update20", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::StgAmb_SetBgmVolume(c, 127i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgAmb_Update20", &PROBE_N, &[], true); }
    r_
}

pub fn StgAmb_Update21(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgAmb_Update21", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::StgAmb_SetBgmVolume(c, 127i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgAmb_Update21", &PROBE_N, &[], true); }
    r_
}

pub fn StgAmb_Update22(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgAmb_Update22", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::StgAmb_SetBgmVolume(c, 127i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgAmb_Update22", &PROBE_N, &[], true); }
    r_
}

pub fn StgAmb_Start(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgAmb_Start", &PROBE_N, &[], false); }
    let mut l_id_1: i32 = 0;
    let r_ = 'f: {
    { let _ = F::memset(c, sym::gStgAmb, 0i32, 28u32); };
    { let v_ = F32(0x3f800000); let a_ = sym::gStgAmb; c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x3f800000); let a_ = (sym::gStgAmb).wrapping_add(8); c.m.w32(a_, (v_).0); };
    l_id_1 = F::BtlStage_GetId(c);
    's1: {
        let sv2: i32 = l_id_1;
        match sv2 {
            0i32 | 23i32 | 24i32 => {
                { let v_ = fid::StgAmb_Update00; let a_ = (sym::gStgAmb).wrapping_add(24); c.m.w32(a_, v_); };
                break 's1;
            }
            1i32 | 25i32 | 26i32 => {
                { let v_ = fid::StgAmb_Update01; let a_ = (sym::gStgAmb).wrapping_add(24); c.m.w32(a_, v_); };
                break 's1;
            }
            2i32 => {
                { let v_ = fid::StgAmb_Update02; let a_ = (sym::gStgAmb).wrapping_add(24); c.m.w32(a_, v_); };
                break 's1;
            }
            3i32 => {
                { let v_ = fid::StgAmb_Update03; let a_ = (sym::gStgAmb).wrapping_add(24); c.m.w32(a_, v_); };
                break 's1;
            }
            4i32 | 27i32 => {
                { let v_ = fid::StgAmb_Update04; let a_ = (sym::gStgAmb).wrapping_add(24); c.m.w32(a_, v_); };
                break 's1;
            }
            5i32 => {
                { let v_ = fid::StgAmb_Update05; let a_ = (sym::gStgAmb).wrapping_add(24); c.m.w32(a_, v_); };
                break 's1;
            }
            6i32 | 28i32 => {
                { let v_ = fid::StgAmb_Update06; let a_ = (sym::gStgAmb).wrapping_add(24); c.m.w32(a_, v_); };
                break 's1;
            }
            7i32 => {
                { let v_ = fid::StgAmb_Update07; let a_ = (sym::gStgAmb).wrapping_add(24); c.m.w32(a_, v_); };
                break 's1;
            }
            8i32 => {
                { let v_ = fid::StgAmb_Update08; let a_ = (sym::gStgAmb).wrapping_add(24); c.m.w32(a_, v_); };
                break 's1;
            }
            9i32 | 29i32 | 30i32 => {
                { let v_ = fid::StgAmb_Update09; let a_ = (sym::gStgAmb).wrapping_add(24); c.m.w32(a_, v_); };
                break 's1;
            }
            10i32 => {
                { let v_ = fid::StgAmb_Update10; let a_ = (sym::gStgAmb).wrapping_add(24); c.m.w32(a_, v_); };
                break 's1;
            }
            11i32 => {
                { let v_ = fid::StgAmb_Update11; let a_ = (sym::gStgAmb).wrapping_add(24); c.m.w32(a_, v_); };
                break 's1;
            }
            12i32 => {
                { let v_ = fid::StgAmb_Update12; let a_ = (sym::gStgAmb).wrapping_add(24); c.m.w32(a_, v_); };
                break 's1;
            }
            13i32 => {
                { let v_ = fid::StgAmb_Update13; let a_ = (sym::gStgAmb).wrapping_add(24); c.m.w32(a_, v_); };
                break 's1;
            }
            14i32 => {
                { let v_ = fid::StgAmb_Update14; let a_ = (sym::gStgAmb).wrapping_add(24); c.m.w32(a_, v_); };
                break 's1;
            }
            15i32 => {
                { let v_ = fid::StgAmb_Update15; let a_ = (sym::gStgAmb).wrapping_add(24); c.m.w32(a_, v_); };
                break 's1;
            }
            16i32 => {
                { let v_ = fid::StgAmb_Update16; let a_ = (sym::gStgAmb).wrapping_add(24); c.m.w32(a_, v_); };
                break 's1;
            }
            17i32 => {
                { let v_ = fid::StgAmb_Update17; let a_ = (sym::gStgAmb).wrapping_add(24); c.m.w32(a_, v_); };
                break 's1;
            }
            18i32 => {
                { let v_ = fid::StgAmb_Update18; let a_ = (sym::gStgAmb).wrapping_add(24); c.m.w32(a_, v_); };
                break 's1;
            }
            19i32 => {
                { let v_ = fid::StgAmb_Update19; let a_ = (sym::gStgAmb).wrapping_add(24); c.m.w32(a_, v_); };
                break 's1;
            }
            20i32 => {
                { let v_ = fid::StgAmb_Update20; let a_ = (sym::gStgAmb).wrapping_add(24); c.m.w32(a_, v_); };
                break 's1;
            }
            21i32 => {
                { let v_ = fid::StgAmb_Update21; let a_ = (sym::gStgAmb).wrapping_add(24); c.m.w32(a_, v_); };
                break 's1;
            }
            22i32 => {
                { let v_ = fid::StgAmb_Update22; let a_ = (sym::gStgAmb).wrapping_add(24); c.m.w32(a_, v_); };
                break 's1;
            }
            _ => {
            }
        }
    }
    F::MapBgm_Play(c, (l_id_1).wrapping_add(68516i32));
    F::StgAmb_SetBgmVolume(c, 0i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgAmb_Start", &PROBE_N, &[], true); }
    r_
}

pub fn StgAmb_Update(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgAmb_Update", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if (((((c.m.r32((sym::gStgAmb).wrapping_add(24))) != (((0i32) as u32))) as i32)) != 0) {
        { let fp_ = c.m.r32((sym::gStgAmb).wrapping_add(24)); { let _ = D::call(c, fp_, &[]); } };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgAmb_Update", &PROBE_N, &[], true); }
    r_
}

pub fn StgAmb_Stop(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgAmb_Stop", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::StgAmb_SetBgmVolume(c, 0i32);
    F::MapBgm_Stop(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgAmb_Stop", &PROBE_N, &[], true); }
    r_
}

pub fn StgAmb_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgAmb_Term", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let v_ = ((0i32) as u32); let a_ = (sym::gStgAmb).wrapping_add(24); c.m.w32(a_, v_); };
    F::StgAmb_SetBgmVolume(c, 0i32);
    F::MapBgm_Stop(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgAmb_Term", &PROBE_N, &[], true); }
    r_
}

pub fn StgAmb_Init(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgAmb_Init", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::StgAmb_Start(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgAmb_Init", &PROBE_N, &[], true); }
    r_
}

pub fn StgAmb_SetBgmScale(c: &mut Ctx, mut p_scale_1: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgAmb_SetBgmScale", &PROBE_N, &[(p_scale_1).0], false); }
    let r_ = 'f: {
    { let v_ = p_scale_1; let a_ = sym::gStgAmb; c.m.w32(a_, (v_).0); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgAmb_SetBgmScale", &PROBE_N, &[], true); }
    r_
}

pub fn StgAmb_SetSeScale(c: &mut Ctx, mut p_scale_1: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgAmb_SetSeScale", &PROBE_N, &[(p_scale_1).0], false); }
    let r_ = 'f: {
    { let v_ = p_scale_1; let a_ = (sym::gStgAmb).wrapping_add(8); c.m.w32(a_, (v_).0); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgAmb_SetSeScale", &PROBE_N, &[], true); }
    r_
}

pub fn ScrXfade_AllocBuffers(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ScrXfade_AllocBuffers", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if ((((({ let a_ = (c.m.r32(sym::gScrXfade)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) == (((0i32) as u32))) as i32)) != 0) {
        { let v_ = F::Heap_Alloc(c, 393360i32, ((32i32) as u32), 0i32, 2i32); let a_ = (c.m.r32(sym::gScrXfade)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = F::Heap_Alloc(c, 393360i32, ((32i32) as u32), 0i32, 2i32); let a_ = (c.m.r32(sym::gScrXfade)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ScrXfade_AllocBuffers", &PROBE_N, &[], true); }
    r_
}

pub fn ScrXfade_FreeBuffers(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ScrXfade_FreeBuffers", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if ((((({ let a_ = (c.m.r32(sym::gScrXfade)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = { let a_ = (c.m.r32(sym::gScrXfade)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
        { let a0_ = { let a_ = (c.m.r32(sym::gScrXfade)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = (c.m.r32(sym::gScrXfade)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = (c.m.r32(sym::gScrXfade)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ScrXfade_FreeBuffers", &PROBE_N, &[], true); }
    r_
}

pub fn ScrXfade_Capture(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ScrXfade_Capture", &PROBE_N, &[], false); }
    let mut l_fbp_1: i32 = 0;
    let r_ = 'f: {
    l_fbp_1 = 3584i32;
    if ((((!(((((c.m.r32((sym::gGfx).wrapping_add(564)) as i32)) & (1i32))) != 0)) as i32)) != 0) {
        l_fbp_1 = 0i32;
    }
    { let _ = F::sceGsSyncPath(c, 0i32, ((0i32) as u16)); };
    F::ScrXfade_StoreHalf(c, ((l_fbp_1) as i16), ((512i32) as u16), ((224i32) as u16), 0i32);
    F::ScrXfade_StoreHalf(c, (((l_fbp_1).wrapping_add(1792i32)) as i16), ((512i32) as u16), ((224i32) as u16), 1i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ScrXfade_Capture", &PROBE_N, &[], true); }
    r_
}

pub fn ScrXfade_StoreHalf(c: &mut Ctx, mut p_sbp_1: i16, mut p_w_2: u16, mut p_h_3: u16, mut p_index_4: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ScrXfade_StoreHalf", &PROBE_N, &[(p_sbp_1) as u32, (p_w_2) as u32, (p_h_3) as u32, (p_index_4) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(112) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 112);
    let mut l_xf_5: u32 = 0;
    let mut l_p_6: u32 = 0;
    let mut l_e_7: u32 = 0;
    let mut l_tag_8: u32 = 0;
    let mut l_qwc_9: i32 = 0;
    let r_ = 'f: {
        l_xf_5 = c.m.r32(sym::gScrXfade);
        l_p_6 = c.m.r32((l_xf_5).wrapping_add((((p_index_4) as u32)).wrapping_mul(4)));
        l_e_7 = (l_xf_5).wrapping_add((((p_index_4) as u32)).wrapping_mul(32u32));
        l_tag_8 = (l_e_7).wrapping_add(((16i32) as u32));
        F::sceGsSetDefStoreImage(c, fp, p_sbp_1, (((((p_w_2) as i32)).wrapping_shr((6i32) as u32)) as i16), ((1i32) as i16), ((0i32) as i16), ((0i32) as i16), ((p_w_2) as i16), ((p_h_3) as i16));
        F::FlushCache(c, 0i32);
        { let _ = F::sceGsExecStoreImage(c, fp, (l_p_6).wrapping_add(((112i32) as u32))); };
        { let _ = F::sceGsSyncPath(c, 0i32, ((0i32) as u16)); };
        F::Dbg_AfterStoreImage(c);
        l_qwc_9 = rt::div_i32(((((p_w_2) as i32)).wrapping_mul(((p_h_3) as i32))).wrapping_mul(3i32), 16i32);
        { let v_ = (((((l_qwc_9).wrapping_add(9i32)) | (805306368i32))) as u32); let a_ = (l_tag_8).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = l_p_6; let a_ = (l_tag_8).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = (l_tag_8).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = (l_tag_8).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        l_tag_8 = (l_e_7).wrapping_add(((32i32) as u32));
        { let v_ = ((1879048192i32) as u32); let a_ = (l_tag_8).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = (l_tag_8).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = (l_tag_8).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = (l_tag_8).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = (((((l_qwc_9).wrapping_add(8i32)) | (1342177280i32))) as u32); let a_ = (l_p_6).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = (l_p_6).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = (l_p_6).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = (l_p_6).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_6).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_6 = n_; };
        { let v_ = ((((((((4i32) as u64)) | ((((1i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (l_p_6).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((14i32) as u64); let a_ = (l_p_6).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_6).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_6 = n_; };
        { let v_ = ((80i32) as u64); let a_ = (l_p_6).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((0i32) as u64); let a_ = (l_p_6).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_6).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_6 = n_; };
        { let v_ = ((81i32) as u64); let a_ = (l_p_6).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((0i32) as u64); let a_ = (l_p_6).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_6).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_6 = n_; };
        { let v_ = ((((p_w_2) as u64)) | ((((p_h_3) as u64)).wrapping_shl((32i32) as u32))); let a_ = (l_p_6).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((82i32) as u64); let a_ = (l_p_6).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_6).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_6 = n_; };
        { let v_ = ((83i32) as u64); let a_ = (l_p_6).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((0i32) as u64); let a_ = (l_p_6).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_6).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_6 = n_; };
        { let v_ = ((((((((l_qwc_9) | (32768i32))) as u32)) as u64)) | ((2u64).wrapping_shl((58i32) as u32))); let a_ = (l_p_6).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((0i32) as u64); let a_ = (l_p_6).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_6).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_6 = n_; };
        { let r_ = (l_qwc_9).wrapping_mul(2i32); let n_ = (l_p_6).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_6 = n_; };
        { let v_ = ((((((((1i32) as u64)) | ((((1i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (l_p_6).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((14i32) as u64); let a_ = (l_p_6).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (l_p_6).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_6 = n_; };
        { let v_ = ((63i32) as u64); let a_ = (l_p_6).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((0i32) as u64); let a_ = (l_p_6).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "ScrXfade_StoreHalf", &PROBE_N, &[], true); }
    r_
}

pub fn ScrXfade_Draw(c: &mut Ctx, mut p_unused_1: i32, mut p_tbp_2: i32, mut p_alpha_3: u8) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ScrXfade_Draw", &PROBE_N, &[(p_unused_1) as u32, (p_tbp_2) as u32, (p_alpha_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_i_4: i32 = 0;
    let mut l_dbp_5: i32 = 0;
    let mut l_bw_6: i32 = 0;
    let mut l_w_7: i32 = 0;
    let mut l_h_8: i32 = 0;
    let r_ = 'f: {
        l_w_7 = 512i32;
        l_h_8 = 224i32;
        l_dbp_5 = 0i32;
        l_bw_6 = (l_w_7).wrapping_shr((6i32) as u32);
        {
            l_i_4 = 0i32;
            'l1: loop {
                if !(((((l_i_4) < (2i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    's3: {
                        let sv9: i32 = l_i_4;
                        match sv9 {
                            0i32 => {
                                l_dbp_5 = p_tbp_2;
                                break 's3;
                            }
                            1i32 => {
                                l_dbp_5 = (p_tbp_2).wrapping_add(rt::div_i32((l_w_7).wrapping_mul(l_h_8), 64i32));
                                break 's3;
                            }
                            _ => {
                            }
                        }
                    }
                    { let v_ = (((((((l_dbp_5) as u64)).wrapping_shl((32i32) as u32)) | ((((l_bw_6) as u64)).wrapping_shl((48i32) as u32)))) | (72057594054705152u64)); let a_ = ({ let a_ = (c.m.r32(sym::gScrXfade)).wrapping_add((((l_i_4) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add((((4i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
                    { let a0_ = ((c.m.r32(sym::gScrXfade)).wrapping_add(16)).wrapping_add((((l_i_4) as u32)).wrapping_mul(32)); F::Dma_AddData(c, a0_, 16i32) };
                }
                { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
            }
        }
        { let v_ = F::Dma_BeginDirect(c); let a_ = fp; c.m.w32(a_, v_); };
        F::Gfx_PutDefaultEnv(c, fp);
        { let v_ = ((((((((5i32) as u64)) | ((((1i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (c.m.r32(fp)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((14i32) as u64); let a_ = (c.m.r32(fp)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let a_ = fp; let n_ = (c.m.r32(a_)).wrapping_add((((r_) as u32)).wrapping_mul(8)); c.m.w32(a_, n_); };
        { let v_ = ((196608i32) as u64); let a_ = (c.m.r32(fp)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((71i32) as u64); let a_ = (c.m.r32(fp)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let a_ = fp; let n_ = (c.m.r32(a_)).wrapping_add((((r_) as u32)).wrapping_mul(8)); c.m.w32(a_, n_); };
        { let v_ = ((((((224i32) as u64)) | ((((49i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((32i32) as u32))); let a_ = (c.m.r32(fp)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((78i32) as u64); let a_ = (c.m.r32(fp)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let a_ = fp; let n_ = (c.m.r32(a_)).wrapping_add((((r_) as u32)).wrapping_mul(8)); c.m.w32(a_, n_); };
        { let v_ = ((0i32) as u64); let a_ = (c.m.r32(fp)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((20i32) as u64); let a_ = (c.m.r32(fp)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let a_ = fp; let n_ = (c.m.r32(a_)).wrapping_add((((r_) as u32)).wrapping_mul(8)); c.m.w32(a_, n_); };
        { let v_ = ((0i32) as u64); let a_ = (c.m.r32(fp)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((63i32) as u64); let a_ = (c.m.r32(fp)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let a_ = fp; let n_ = (c.m.r32(a_)).wrapping_add((((r_) as u32)).wrapping_mul(8)); c.m.w32(a_, n_); };
        { let v_ = ((((((((p_tbp_2) as u32)) as u64)) | ((((l_bw_6) as u64)).wrapping_shl((14i32) as u32)))) | ((42561u64).wrapping_shl((20i32) as u32))); let a_ = (c.m.r32(fp)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((6i32) as u64); let a_ = (c.m.r32(fp)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let a_ = fp; let n_ = (c.m.r32(a_)).wrapping_add((((r_) as u32)).wrapping_mul(8)); c.m.w32(a_, n_); };
        F::Dma_PutTexStrips(c, fp, 0i32, 0i32, 512i32, 448i32, 0i32, 0i32, 0i32, 0i32, 512i32, 448i32, 8i32, 8i32, (((((((p_alpha_3) as i32)).wrapping_shl((24i32) as u32)) | (8421504i32))) as u32), 1i32);
        { let a0_ = c.m.r32(fp); F::Dma_EndDirect(c, a0_) };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "ScrXfade_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn ScrXfade_Init(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ScrXfade_Init", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let v_ = F::Heap_Alloc(c, ((112u32) as i32), ((32i32) as u32), 0i32, 2i32); let a_ = sym::gScrXfade; c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32(sym::gScrXfade); F::memset(c, a0_, 0i32, 112u32) }; };
    F::ScrXfade_Reset(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ScrXfade_Init", &PROBE_N, &[], true); }
    r_
}

pub fn ScrXfade_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ScrXfade_Term", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if (((((c.m.r32(sym::gScrXfade)) != (((0i32) as u32))) as i32)) != 0) {
        F::ScrXfade_FreeBuffers(c);
        { let a0_ = c.m.r32(sym::gScrXfade); F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = sym::gScrXfade; c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ScrXfade_Term", &PROBE_N, &[], true); }
    r_
}

pub fn ScrXfade_Reset(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ScrXfade_Reset", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let v_ = 0i32; let a_ = (c.m.r32(sym::gScrXfade)).wrapping_add(80); c.m.w32(a_, (v_) as u32); };
    { let v_ = 0i32; let a_ = (c.m.r32(sym::gScrXfade)).wrapping_add(84); c.m.w32(a_, (v_) as u32); };
    { let a0_ = (c.m.r32(sym::gScrXfade)).wrapping_add(88); F::Ramp_Stop(c, a0_) };
    F::ScrXfade_FreeBuffers(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ScrXfade_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn ScrXfade_Update(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ScrXfade_Update", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if ((((!(((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0)) as i32)) != 0) {
        if (({ let a0_ = (c.m.r32(sym::gScrXfade)).wrapping_add(88); F::Ramp_Step(c, a0_) }) != 0) {
            { let v_ = 0i32; let a_ = (c.m.r32(sym::gScrXfade)).wrapping_add(80); c.m.w32(a_, (v_) as u32); };
            { let v_ = 0i32; let a_ = (c.m.r32(sym::gScrXfade)).wrapping_add(84); c.m.w32(a_, (v_) as u32); };
            F::ScrXfade_FreeBuffers(c);
        } else {
            { let v_ = rt::f2i(rt::fsub(F32(0x43000000), rt::fmul({ let a_ = ((c.m.r32(sym::gScrXfade)).wrapping_add(88)).wrapping_add(16); F32(c.m.r32(a_)) }, F32(0x43000000)))); let a_ = (c.m.r32(sym::gScrXfade)).wrapping_add(80); c.m.w32(a_, (v_) as u32); };
        }
    }
    if ((((({ let a_ = (c.m.r32(sym::gScrXfade)).wrapping_add(80); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
        { let a2_ = (({ let a_ = (c.m.r32(sym::gScrXfade)).wrapping_add(80); (c.m.r32(a_) as i32) }) as u8); F::ScrXfade_Draw(c, 0i32, 7168i32, a2_) };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ScrXfade_Update", &PROBE_N, &[], true); }
    r_
}

pub fn ScrXfade_PostDraw(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ScrXfade_PostDraw", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if ((((({ let a_ = (c.m.r32(sym::gScrXfade)).wrapping_add(84); (c.m.r32(a_) as i32) }) == (1i32)) as i32)) != 0) {
        F::ScrXfade_AllocBuffers(c);
        F::ScrXfade_Capture(c);
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gScrXfade)).wrapping_add(84); c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ScrXfade_PostDraw", &PROBE_N, &[], true); }
    r_
}

pub fn ScrXfade_Start(c: &mut Ctx, mut p_request_1: i32, mut p_seconds_2: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ScrXfade_Start", &PROBE_N, &[(p_request_1) as u32, (p_seconds_2).0], false); }
    let r_ = 'f: {
    { let a0_ = (c.m.r32(sym::gScrXfade)).wrapping_add(88); F::Ramp_Start(c, a0_, p_seconds_2, F32(0x00000000), F32(0x3f800000)) };
    { let v_ = p_request_1; let a_ = (c.m.r32(sym::gScrXfade)).wrapping_add(84); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ScrXfade_Start", &PROBE_N, &[], true); }
    r_
}

pub fn ScrXfade_RequestCapture(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ScrXfade_RequestCapture", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let v_ = 1i32; let a_ = (c.m.r32(sym::gScrXfade)).wrapping_add(84); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ScrXfade_RequestCapture", &PROBE_N, &[], true); }
    r_
}

pub fn ScrXfade_IsActive(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ScrXfade_IsActive", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f ((({ let a_ = (c.m.r32(sym::gScrXfade)).wrapping_add(80); (c.m.r32(a_) as i32) }) != (0i32)) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "ScrXfade_IsActive", &PROBE_N, &[], true); }
    r_
}

pub fn ScrWarp_BeginPacket(c: &mut Ctx, mut p_pp_1: u32, mut p_x0_2: i32, mut p_x1_3: i32, mut p_w_4: i32, mut p_h_5: i32, mut p_tw_6: i32, mut p_th_7: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ScrWarp_BeginPacket", &PROBE_N, &[(p_pp_1) as u32, (p_x0_2) as u32, (p_x1_3) as u32, (p_w_4) as u32, (p_h_5) as u32, (p_tw_6) as u32, (p_th_7) as u32], false); }
    let r_ = 'f: {
    { let v_ = F::Dma_BeginDirect(c); let a_ = p_pp_1; c.m.w32(a_, v_); };
    F::Gfx_PutDefaultEnv(c, p_pp_1);
    { let v_ = ((((((((7i32) as u64)) | ((((0i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (c.m.r32(p_pp_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((14i32) as u64); let a_ = (c.m.r32(p_pp_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let a_ = p_pp_1; let n_ = (c.m.r32(a_)).wrapping_add((((r_) as u32)).wrapping_mul(8)); c.m.w32(a_, n_); };
    { let v_ = ((196608i32) as u64); let a_ = (c.m.r32(p_pp_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((71i32) as u64); let a_ = (c.m.r32(p_pp_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let a_ = p_pp_1; let n_ = (c.m.r32(a_)).wrapping_add((((r_) as u32)).wrapping_mul(8)); c.m.w32(a_, n_); };
    { let v_ = ((((((((0i32) as u64)) | (((((p_tw_6).wrapping_sub(1i32)) as u64)).wrapping_shl((16i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((32i32) as u32)))) | (((((p_th_7).wrapping_sub(1i32)) as u64)).wrapping_shl((48i32) as u32))); let a_ = (c.m.r32(p_pp_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((64i32) as u64); let a_ = (c.m.r32(p_pp_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let a_ = p_pp_1; let n_ = (c.m.r32(a_)).wrapping_add((((r_) as u32)).wrapping_mul(8)); c.m.w32(a_, n_); };
    { let v_ = ((((((224i32) as u64)) | ((((49i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((32i32) as u32))); let a_ = (c.m.r32(p_pp_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((78i32) as u64); let a_ = (c.m.r32(p_pp_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let a_ = p_pp_1; let n_ = (c.m.r32(a_)).wrapping_add((((r_) as u32)).wrapping_mul(8)); c.m.w32(a_, n_); };
    { let v_ = ((((((((336i32) as u64)) | ((((4i32) as u64)).wrapping_shl((16i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((32i32) as u32))); let a_ = (c.m.r32(p_pp_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((76i32) as u64); let a_ = (c.m.r32(p_pp_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let a_ = p_pp_1; let n_ = (c.m.r32(a_)).wrapping_add((((r_) as u32)).wrapping_mul(8)); c.m.w32(a_, n_); };
    { let v_ = ((((((((((((2i32) as u64)) | ((((2i32) as u64)).wrapping_shl((2i32) as u32)))) | ((((p_x0_2) as u64)).wrapping_shl((4i32) as u32)))) | (((((p_x1_3).wrapping_sub(1i32)) as u64)).wrapping_shl((14i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((24i32) as u32)))) | (((((p_h_5).wrapping_sub(1i32)) as u64)).wrapping_shl((34i32) as u32))); let a_ = (c.m.r32(p_pp_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((8i32) as u64); let a_ = (c.m.r32(p_pp_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let a_ = p_pp_1; let n_ = (c.m.r32(a_)).wrapping_add((((r_) as u32)).wrapping_mul(8)); c.m.w32(a_, n_); };
    { let v_ = ((96i32) as u64); let a_ = (c.m.r32(p_pp_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((20i32) as u64); let a_ = (c.m.r32(p_pp_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let a_ = p_pp_1; let n_ = (c.m.r32(a_)).wrapping_add((((r_) as u32)).wrapping_mul(8)); c.m.w32(a_, n_); };
    { let v_ = (if ((((!(((((c.m.r32((sym::gGfx).wrapping_add(564)) as i32)) & (1i32))) != 0)) as i32)) != 0) { 10267790848u64 } else { 10267787264u64 }); let a_ = (c.m.r32(p_pp_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((6i32) as u64); let a_ = (c.m.r32(p_pp_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let a_ = p_pp_1; let n_ = (c.m.r32(a_)).wrapping_add((((r_) as u32)).wrapping_mul(8)); c.m.w32(a_, n_); };
    F::Dma_PutTexStrips(c, p_pp_1, 0i32, 0i32, p_tw_6, p_th_7, 0i32, 0i32, p_x0_2, 0i32, (p_w_4).wrapping_add(p_x0_2), p_h_5, 8i32, 8i32, 2155905152u32, 0i32);
    { let v_ = ((((((((4i32) as u64)) | ((((0i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (c.m.r32(p_pp_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((14i32) as u64); let a_ = (c.m.r32(p_pp_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let a_ = p_pp_1; let n_ = (c.m.r32(a_)).wrapping_add((((r_) as u32)).wrapping_mul(8)); c.m.w32(a_, n_); };
    { let v_ = (((if ((((!(((((c.m.r32((sym::gGfx).wrapping_add(564)) as i32)) & (1i32))) != 0)) as i32)) != 0) { 524400i32 } else { 524288i32 })) as u64); let a_ = (c.m.r32(p_pp_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((76i32) as u64); let a_ = (c.m.r32(p_pp_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let a_ = p_pp_1; let n_ = (c.m.r32(a_)).wrapping_add((((r_) as u32)).wrapping_mul(8)); c.m.w32(a_, n_); };
    { let v_ = ((((((((p_x0_2) as u64)) | (((((p_x1_3).wrapping_sub(1i32)) as u64)).wrapping_shl((16i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((32i32) as u32)))) | (((((p_h_5).wrapping_sub(1i32)) as u64)).wrapping_shl((48i32) as u32))); let a_ = (c.m.r32(p_pp_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((64i32) as u64); let a_ = (c.m.r32(p_pp_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let a_ = p_pp_1; let n_ = (c.m.r32(a_)).wrapping_add((((r_) as u32)).wrapping_mul(8)); c.m.w32(a_, n_); };
    { let v_ = ((((((((((((2i32) as u64)) | ((((2i32) as u64)).wrapping_shl((2i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((4i32) as u32)))) | (((((p_tw_6).wrapping_sub(1i32)) as u64)).wrapping_shl((14i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((24i32) as u32)))) | (((((p_th_7).wrapping_sub(1i32)) as u64)).wrapping_shl((34i32) as u32))); let a_ = (c.m.r32(p_pp_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((8i32) as u64); let a_ = (c.m.r32(p_pp_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let a_ = p_pp_1; let n_ = (c.m.r32(a_)).wrapping_add((((r_) as u32)).wrapping_mul(8)); c.m.w32(a_, n_); };
    { let v_ = 9126881792u64; let a_ = (c.m.r32(p_pp_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((6i32) as u64); let a_ = (c.m.r32(p_pp_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let a_ = p_pp_1; let n_ = (c.m.r32(a_)).wrapping_add((((r_) as u32)).wrapping_mul(8)); c.m.w32(a_, n_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ScrWarp_BeginPacket", &PROBE_N, &[], true); }
    r_
}

pub fn ScrWarp_EndPacket(c: &mut Ctx, mut p_pp_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ScrWarp_EndPacket", &PROBE_N, &[(p_pp_1) as u32], false); }
    let r_ = 'f: {
    { let a0_ = c.m.r32(p_pp_1); F::Dma_EndDirect(c, a0_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ScrWarp_EndPacket", &PROBE_N, &[], true); }
    r_
}

pub fn ScrWarp_BeginStrip(c: &mut Ctx, mut p_pp_1: u32, mut p_count_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ScrWarp_BeginStrip", &PROBE_N, &[(p_pp_1) as u32, (p_count_2) as u32], false); }
    let r_ = 'f: {
    { let v_ = ((((((((1i32) as u64)) | ((((0i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (c.m.r32(p_pp_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((14i32) as u64); let a_ = (c.m.r32(p_pp_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let a_ = p_pp_1; let n_ = (c.m.r32(a_)).wrapping_add((((r_) as u32)).wrapping_mul(8)); c.m.w32(a_, n_); };
    { let v_ = ((348i32) as u64); let a_ = (c.m.r32(p_pp_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((0i32) as u64); let a_ = (c.m.r32(p_pp_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let a_ = p_pp_1; let n_ = (c.m.r32(a_)).wrapping_add((((r_) as u32)).wrapping_mul(8)); c.m.w32(a_, n_); };
    { let v_ = ((((((((p_count_2) as u64)) | ((((1i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((4i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (c.m.r32(p_pp_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((21279i32) as u64); let a_ = (c.m.r32(p_pp_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let a_ = p_pp_1; let n_ = (c.m.r32(a_)).wrapping_add((((r_) as u32)).wrapping_mul(8)); c.m.w32(a_, n_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ScrWarp_BeginStrip", &PROBE_N, &[], true); }
    r_
}

pub fn ScrWarp_EndStrip(c: &mut Ctx, mut p_pp_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ScrWarp_EndStrip", &PROBE_N, &[(p_pp_1) as u32], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ScrWarp_EndStrip", &PROBE_N, &[], true); }
    r_
}

pub fn ScrWarp_PutVertex(c: &mut Ctx, mut p_pp_1: u32, mut p_x_2: i32, mut p_y_3: i32, mut p_u_4: i32, mut p_v_5: i32, mut p_rgba_6: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ScrWarp_PutVertex", &PROBE_N, &[(p_pp_1) as u32, (p_x_2) as u32, (p_y_3) as u32, (p_u_4) as u32, (p_v_5) as u32, (p_rgba_6) as u32], false); }
    let r_ = 'f: {
    { let v_ = ((0i32) as u64); let a_ = (c.m.r32(p_pp_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((((p_rgba_6) as u64)) | ((1065353216u64).wrapping_shl((32i32) as u32))); let a_ = (c.m.r32(p_pp_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let a_ = p_pp_1; let n_ = (c.m.r32(a_)).wrapping_add((((r_) as u32)).wrapping_mul(8)); c.m.w32(a_, n_); };
    { let v_ = (((((p_u_4).wrapping_shl((4i32) as u32)) as u64)) | (((((p_v_5).wrapping_shl((4i32) as u32)) as u64)).wrapping_shl((16i32) as u32))); let a_ = (c.m.r32(p_pp_1)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((((((p_x_2).wrapping_shl((4i32) as u32)).wrapping_add(28672i32)) as u64)) | ((((((p_y_3).wrapping_shl((4i32) as u32)).wrapping_add(29184i32)) as u64)).wrapping_shl((16i32) as u32))); let a_ = (c.m.r32(p_pp_1)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let a_ = p_pp_1; let n_ = (c.m.r32(a_)).wrapping_add((((r_) as u32)).wrapping_mul(8)); c.m.w32(a_, n_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ScrWarp_PutVertex", &PROBE_N, &[], true); }
    r_
}

pub fn ScrWarp_Alloc(c: &mut Ctx, mut p_view_1: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ScrWarp_Alloc", &PROBE_N, &[(p_view_1) as u32], false); }
    let mut l_mgr_2: u32 = 0;
    let mut l_found_3: u32 = 0;
    let mut l_o_4: u32 = 0;
    let mut l_i_5: i32 = 0;
    let r_ = 'f: {
    l_mgr_2 = c.m.r32(sym::gScrWarp);
    l_found_3 = ((0i32) as u32);
    l_o_4 = c.m.r32(l_mgr_2);
    {
        l_i_5 = 0i32;
        'l1: loop {
            if !(((((l_i_5) < (10i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((((((c.m.r32(l_o_4) as i32)) == (0i32)) as i32)) != 0) {
                    l_found_3 = l_o_4;
                    break 'l1;
                }
            }
            { { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; }; { let o_ = l_o_4; let n_ = (o_).wrapping_add(1136); l_o_4 = n_; }; };
        }
    }
    if (((((l_found_3) != (((0i32) as u32))) as i32)) != 0) {
        if (((((p_view_1) == (0i32)) as i32)) != 0) {
            { let a_ = ((l_mgr_2).wrapping_add(164)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        } else {
            { let a_ = ((l_mgr_2).wrapping_add(164)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
        }
    }
    break 'f l_found_3;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "ScrWarp_Alloc", &PROBE_N, &[], true); }
    r_
}

pub fn ScrWarp_Free(c: &mut Ctx, mut p_o_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ScrWarp_Free", &PROBE_N, &[(p_o_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = 0i32; let a_ = p_o_1; c.m.w32(a_, (v_) as u32); };
    if ((((((c.m.r32((p_o_1).wrapping_add(4)) as i32)) == (0i32)) as i32)) != 0) {
        { let a_ = ((c.m.r32(sym::gScrWarp)).wrapping_add(164)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
    } else {
        { let a_ = ((c.m.r32(sym::gScrWarp)).wrapping_add(164)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ScrWarp_Free", &PROBE_N, &[], true); }
    r_
}

pub fn ScrWarp_IsOnScreen(c: &mut Ctx, mut p_screen_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ScrWarp_IsOnScreen", &PROBE_N, &[(p_screen_1) as u32], false); }
    let r_ = 'f: {
    if (((rt::fle(F32(c.m.r32((p_screen_1).wrapping_add(8))), F32(0x00000000)) as i32)) != 0) {
        break 'f 0i32;
    }
    if (((rt::fle(F32(0x45100000), F32(c.m.r32(p_screen_1))) as i32)) != 0) {
        break 'f 0i32;
    }
    if (((rt::fle(F32(c.m.r32(p_screen_1)), F32(0x44e00000)) as i32)) != 0) {
        break 'f 0i32;
    }
    if (((rt::fle(F32(0x450e0000), F32(c.m.r32((p_screen_1).wrapping_add(4)))) as i32)) != 0) {
        break 'f 0i32;
    }
    if (((rt::fle(F32(c.m.r32((p_screen_1).wrapping_add(4))), F32(0x44e40000)) as i32)) != 0) {
        break 'f 0i32;
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "ScrWarp_IsOnScreen", &PROBE_N, &[], true); }
    r_
}

pub fn ScrWarp_Step(c: &mut Ctx, mut p_o_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ScrWarp_Step", &PROBE_N, &[(p_o_1) as u32], false); }
    let mut l_r0_2: F32 = F32(0);
    let mut l_r1_3: F32 = F32(0);
    let mut l_r2_4: F32 = F32(0);
    let mut l_rj_5: F32 = F32(0);
    let mut l_dir_6: u32 = 0;
    let mut l_i_7: i32 = 0;
    let r_ = 'f: {
    { let r_ = F32(0x3f800000); let a_ = (p_o_1).wrapping_add(8); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
    if (((rt::fle(F32(c.m.r32((p_o_1).wrapping_add(8))), F32(0x00000000)) as i32)) != 0) {
        F::ScrWarp_Free(c, p_o_1);
        break 'f;
    }
    if (((rt::flt(F32(0x44160000), F32(c.m.r32((p_o_1).wrapping_add(12)))) as i32)) != 0) {
        F::ScrWarp_Free(c, p_o_1);
        break 'f;
    }
    l_r0_2 = rt::fadd(F32(c.m.r32((p_o_1).wrapping_add(12))), F32(c.m.r32((p_o_1).wrapping_add(16))));
    { let v_ = l_r0_2; let a_ = (p_o_1).wrapping_add(12); c.m.w32(a_, (v_).0); };
    l_r2_4 = rt::fadd(l_r0_2, F32(c.m.r32((p_o_1).wrapping_add(24))));
    l_r1_3 = rt::fadd(l_r0_2, rt::fmul(F32(c.m.r32((p_o_1).wrapping_add(24))), F32(0x3f000000)));
    l_dir_6 = ((c.m.r32(sym::gScrWarp)).wrapping_add(4)).wrapping_add((((0i32) as u32)).wrapping_mul(8));
    {
        l_i_7 = 0i32;
        'l1: loop {
            if !(((((l_i_7) < (20i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let v_ = rt::fmul(rt::fadd(F32(c.m.r32((p_o_1).wrapping_add(64))), rt::fmul(F32(c.m.r32((l_dir_6).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), l_r0_2)), F32(0x3f000000)); let a_ = ((((p_o_1).wrapping_add(656)).wrapping_add((((l_i_7) as u32)).wrapping_mul(24))).wrapping_add((((0i32) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fmul(rt::fadd(F32(c.m.r32(((p_o_1).wrapping_add(64)).wrapping_add(4))), rt::fmul(F32(c.m.r32((l_dir_6).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), l_r0_2)), F32(0x3f000000)); let a_ = ((((p_o_1).wrapping_add(656)).wrapping_add((((l_i_7) as u32)).wrapping_mul(24))).wrapping_add((((0i32) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fmul(rt::fadd(F32(c.m.r32((p_o_1).wrapping_add(64))), rt::fmul(F32(c.m.r32((l_dir_6).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), l_r1_3)), F32(0x3f000000)); let a_ = ((((p_o_1).wrapping_add(656)).wrapping_add((((l_i_7) as u32)).wrapping_mul(24))).wrapping_add((((1i32) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fmul(rt::fadd(F32(c.m.r32(((p_o_1).wrapping_add(64)).wrapping_add(4))), rt::fmul(F32(c.m.r32((l_dir_6).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), l_r1_3)), F32(0x3f000000)); let a_ = ((((p_o_1).wrapping_add(656)).wrapping_add((((l_i_7) as u32)).wrapping_mul(24))).wrapping_add((((1i32) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fmul(rt::fadd(F32(c.m.r32((p_o_1).wrapping_add(64))), rt::fmul(F32(c.m.r32((l_dir_6).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), l_r2_4)), F32(0x3f000000)); let a_ = ((((p_o_1).wrapping_add(656)).wrapping_add((((l_i_7) as u32)).wrapping_mul(24))).wrapping_add((((2i32) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fmul(rt::fadd(F32(c.m.r32(((p_o_1).wrapping_add(64)).wrapping_add(4))), rt::fmul(F32(c.m.r32((l_dir_6).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), l_r2_4)), F32(0x3f000000)); let a_ = ((((p_o_1).wrapping_add(656)).wrapping_add((((l_i_7) as u32)).wrapping_mul(24))).wrapping_add((((2i32) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                l_rj_5 = rt::fadd(l_r0_2, rt::fmul(F32(c.m.r32((p_o_1).wrapping_add(24))), F32(c.m.r32(((p_o_1).wrapping_add(96)).wrapping_add((((l_i_7) as u32)).wrapping_mul(4))))));
                { let v_ = rt::fadd(F32(c.m.r32((p_o_1).wrapping_add(80))), rt::fmul(F32(c.m.r32((l_dir_6).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), l_r0_2)); let a_ = ((((p_o_1).wrapping_add(176)).wrapping_add((((l_i_7) as u32)).wrapping_mul(24))).wrapping_add((((0i32) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fadd(F32(c.m.r32((p_o_1).wrapping_add(84))), rt::fmul(F32(c.m.r32((l_dir_6).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), l_r0_2)); let a_ = ((((p_o_1).wrapping_add(176)).wrapping_add((((l_i_7) as u32)).wrapping_mul(24))).wrapping_add((((0i32) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fadd(F32(c.m.r32((p_o_1).wrapping_add(80))), rt::fmul(F32(c.m.r32((l_dir_6).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), l_rj_5)); let a_ = ((((p_o_1).wrapping_add(176)).wrapping_add((((l_i_7) as u32)).wrapping_mul(24))).wrapping_add((((1i32) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fadd(F32(c.m.r32((p_o_1).wrapping_add(84))), rt::fmul(F32(c.m.r32((l_dir_6).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), l_rj_5)); let a_ = ((((p_o_1).wrapping_add(176)).wrapping_add((((l_i_7) as u32)).wrapping_mul(24))).wrapping_add((((1i32) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fadd(F32(c.m.r32((p_o_1).wrapping_add(80))), rt::fmul(F32(c.m.r32((l_dir_6).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), l_r2_4)); let a_ = ((((p_o_1).wrapping_add(176)).wrapping_add((((l_i_7) as u32)).wrapping_mul(24))).wrapping_add((((2i32) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                { let v_ = rt::fadd(F32(c.m.r32((p_o_1).wrapping_add(84))), rt::fmul(F32(c.m.r32((l_dir_6).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), l_r2_4)); let a_ = ((((p_o_1).wrapping_add(176)).wrapping_add((((l_i_7) as u32)).wrapping_mul(24))).wrapping_add((((2i32) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                if (((rt::flt(F32(c.m.r32(((((p_o_1).wrapping_add(656)).wrapping_add((((l_i_7) as u32)).wrapping_mul(24))).wrapping_add((((0i32) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(0x40000000)) as i32)) != 0) {
                    { let v_ = F32(0x40000000); let a_ = ((((p_o_1).wrapping_add(656)).wrapping_add((((l_i_7) as u32)).wrapping_mul(24))).wrapping_add((((0i32) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                }
                if (((rt::flt(F32(c.m.r32(((((p_o_1).wrapping_add(656)).wrapping_add((((l_i_7) as u32)).wrapping_mul(24))).wrapping_add((((0i32) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(0x00000000)) as i32)) != 0) {
                    { let v_ = F32(0x00000000); let a_ = ((((p_o_1).wrapping_add(656)).wrapping_add((((l_i_7) as u32)).wrapping_mul(24))).wrapping_add((((0i32) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                }
                if (((rt::flt(F32(c.m.r32(((((p_o_1).wrapping_add(656)).wrapping_add((((l_i_7) as u32)).wrapping_mul(24))).wrapping_add((((1i32) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(0x40000000)) as i32)) != 0) {
                    { let v_ = F32(0x40000000); let a_ = ((((p_o_1).wrapping_add(656)).wrapping_add((((l_i_7) as u32)).wrapping_mul(24))).wrapping_add((((1i32) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                }
                if (((rt::flt(F32(c.m.r32(((((p_o_1).wrapping_add(656)).wrapping_add((((l_i_7) as u32)).wrapping_mul(24))).wrapping_add((((1i32) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(0x00000000)) as i32)) != 0) {
                    { let v_ = F32(0x00000000); let a_ = ((((p_o_1).wrapping_add(656)).wrapping_add((((l_i_7) as u32)).wrapping_mul(24))).wrapping_add((((1i32) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                }
                if (((rt::flt(F32(c.m.r32(((((p_o_1).wrapping_add(656)).wrapping_add((((l_i_7) as u32)).wrapping_mul(24))).wrapping_add((((2i32) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(0x40000000)) as i32)) != 0) {
                    { let v_ = F32(0x40000000); let a_ = ((((p_o_1).wrapping_add(656)).wrapping_add((((l_i_7) as u32)).wrapping_mul(24))).wrapping_add((((2i32) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                }
                if (((rt::flt(F32(c.m.r32(((((p_o_1).wrapping_add(656)).wrapping_add((((l_i_7) as u32)).wrapping_mul(24))).wrapping_add((((2i32) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(0x00000000)) as i32)) != 0) {
                    { let v_ = F32(0x00000000); let a_ = ((((p_o_1).wrapping_add(656)).wrapping_add((((l_i_7) as u32)).wrapping_mul(24))).wrapping_add((((2i32) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                }
            }
            { { let o_ = l_i_7; let n_ = (o_).wrapping_add(1); l_i_7 = n_; }; { let r_ = 2i32; let n_ = (l_dir_6).wrapping_add((((r_) as u32)).wrapping_mul(4)); l_dir_6 = n_; }; };
        }
    }
    { let r_ = F32(c.m.r32((p_o_1).wrapping_add(20))); let a_ = (p_o_1).wrapping_add(16); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
    { let r_ = F32(c.m.r32((p_o_1).wrapping_add(28))); let a_ = (p_o_1).wrapping_add(24); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
    if (((rt::fle(F32(c.m.r32((p_o_1).wrapping_add(8))), F32(0x40c00000)) as i32)) != 0) {
        { let r_ = F32(c.m.r32((p_o_1).wrapping_add(36))); let a_ = (p_o_1).wrapping_add(32); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
        if (((rt::flt(F32(c.m.r32((p_o_1).wrapping_add(32))), F32(0x00000000)) as i32)) != 0) {
            { let v_ = F32(0x00000000); let a_ = (p_o_1).wrapping_add(32); c.m.w32(a_, (v_).0); };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ScrWarp_Step", &PROBE_N, &[], true); }
    r_
}

pub fn ScrWarp_Draw(c: &mut Ctx, mut p_split_1: i32, mut p_view_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ScrWarp_Draw", &PROBE_N, &[(p_split_1) as u32, (p_view_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_o_3: u32 = 0;
    let mut l_x0_4: i32 = 0;
    let mut l_x1_5: i32 = 0;
    let mut l_w_6: i32 = 0;
    let mut l_tw_7: i32 = 0;
    let mut l_left_8: i32 = 0;
    let mut l_rgb_9: i32 = 0;
    let mut l_j_10: i32 = 0;
    let mut l_jn_11: i32 = 0;
    let mut l_k_12: i32 = 0;
    let mut l_m_13: i32 = 0;
    let mut l_j1_14: i32 = 0;
    let mut l_n_15: i32 = 0;
    let mut l_a0_16: u32 = 0;
    let mut l_a1_17: u32 = 0;
    let r_ = 'f: {
        l_j1_14 = 0i32;
        l_x0_4 = 0i32;
        l_w_6 = 512i32;
        l_tw_7 = 256i32;
        l_x1_5 = 512i32;
        if ((p_split_1) != 0) {
            { let r_ = 2i32; let n_ = rt::div_i32(l_w_6, r_); l_w_6 = n_; };
            l_x0_4 = l_w_6;
            { let r_ = 2i32; let n_ = rt::div_i32(l_tw_7, r_); l_tw_7 = n_; };
            if (((((p_view_2) != (1i32)) as i32)) != 0) {
                { let r_ = l_w_6; let n_ = (l_x0_4).wrapping_sub(r_); l_x0_4 = n_; };
            }
            l_x1_5 = (l_x0_4).wrapping_add(l_w_6);
        }
        F::ScrWarp_BeginPacket(c, fp.wrapping_add(12), l_x0_4, l_x1_5, l_w_6, 448i32, l_tw_7, 224i32);
        l_o_3 = { let a_ = c.m.r32(sym::gScrWarp); c.m.r32(a_) };
        {
            l_left_8 = (10i32).wrapping_sub(1i32);
            'l1: loop {
                if !(((((l_left_8) >= (0i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    if ((((((c.m.r32(l_o_3) as i32)) == (0i32)) as i32)) != 0) {
                        break 'c2;
                    }
                    if (((((((((p_split_1) != (0i32)) as i32)) != 0) && ((((((c.m.r32((l_o_3).wrapping_add(4)) as i32)) != (p_view_2)) as i32)) != 0)) as i32)) != 0) {
                        break 'c2;
                    }
                    l_j_10 = 0i32;
                    l_n_15 = 20i32;
                    l_rgb_9 = 8421504i32;
                    { let v_ = rt::f2i(rt::fmul(F32(c.m.r32((l_o_3).wrapping_add(32))), F32(0x437f0000))); let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    { let v_ = rt::div_i32((c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32), 8i32); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    { let v_ = rt::div_i32((c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32), 8i32); let a_ = (fp).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    {
                        'l3: loop {
                            if !(((((l_j_10) < (2i32)) as i32)) != 0) { break 'l3; }
                            'c4: {
                                l_jn_11 = (l_j_10).wrapping_add(1i32);
                                F::ScrWarp_BeginStrip(c, fp.wrapping_add(12), ((20i32).wrapping_add(1i32)).wrapping_mul(2i32));
                                {
                                    l_k_12 = 0i32;
                                    'l5: loop {
                                        if !(((((l_k_12) < ((20i32).wrapping_add(1i32))) as i32)) != 0) { break 'l5; }
                                        'c6: {
                                            l_a0_16 = (fp).wrapping_add((((l_j_10) as u32)).wrapping_mul(4));
                                            l_a1_17 = (fp).wrapping_add((((l_jn_11) as u32)).wrapping_mul(4));
                                            l_j1_14 = l_jn_11;
                                            l_m_13 = rt::rem_i32(l_k_12, l_n_15);
                                            { let a1_ = rt::f2i(F32(c.m.r32(((((l_o_3).wrapping_add(176)).wrapping_add((((l_m_13) as u32)).wrapping_mul(24))).wrapping_add((((l_j_10) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a2_ = rt::f2i(F32(c.m.r32(((((l_o_3).wrapping_add(176)).wrapping_add((((l_m_13) as u32)).wrapping_mul(24))).wrapping_add((((l_j_10) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); let a3_ = rt::f2i(F32(c.m.r32(((((l_o_3).wrapping_add(656)).wrapping_add((((l_m_13) as u32)).wrapping_mul(24))).wrapping_add((((l_j_10) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a4_ = rt::f2i(F32(c.m.r32(((((l_o_3).wrapping_add(656)).wrapping_add((((l_m_13) as u32)).wrapping_mul(24))).wrapping_add((((l_j_10) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); let a5_ = ((((c.m.r32(l_a0_16) as i32)).wrapping_shl((24i32) as u32)) | (l_rgb_9)); F::ScrWarp_PutVertex(c, fp.wrapping_add(12), a1_, a2_, a3_, a4_, a5_) };
                                            { let a1_ = rt::f2i(F32(c.m.r32(((((l_o_3).wrapping_add(176)).wrapping_add((((l_m_13) as u32)).wrapping_mul(24))).wrapping_add((((l_j1_14) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a2_ = rt::f2i(F32(c.m.r32(((((l_o_3).wrapping_add(176)).wrapping_add((((l_m_13) as u32)).wrapping_mul(24))).wrapping_add((((l_j1_14) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); let a3_ = rt::f2i(F32(c.m.r32(((((l_o_3).wrapping_add(656)).wrapping_add((((l_m_13) as u32)).wrapping_mul(24))).wrapping_add((((l_j1_14) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a4_ = rt::f2i(F32(c.m.r32(((((l_o_3).wrapping_add(656)).wrapping_add((((l_m_13) as u32)).wrapping_mul(24))).wrapping_add((((l_j1_14) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); let a5_ = ((((c.m.r32(l_a1_17) as i32)).wrapping_shl((24i32) as u32)) | (l_rgb_9)); F::ScrWarp_PutVertex(c, fp.wrapping_add(12), a1_, a2_, a3_, a4_, a5_) };
                                        }
                                        { let o_ = l_k_12; let n_ = (o_).wrapping_add(1); l_k_12 = n_; };
                                    }
                                }
                                F::ScrWarp_EndStrip(c, fp.wrapping_add(12));
                            }
                            { let o_ = l_j_10; let n_ = (o_).wrapping_add(1); l_j_10 = n_; };
                        }
                    }
                }
                { { let o_ = l_left_8; let n_ = (o_).wrapping_sub(1); l_left_8 = n_; }; { let o_ = l_o_3; let n_ = (o_).wrapping_add(1136); l_o_3 = n_; }; };
            }
        }
        F::ScrWarp_EndPacket(c, fp.wrapping_add(12));
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "ScrWarp_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn ScrWarp_Init(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ScrWarp_Init", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let mut l_a_2: F32 = F32(0);
    let r_ = 'f: {
    { let v_ = F::Heap_Alloc(c, ((172u32) as i32), ((32i32) as u32), 0i32, 2i32); let a_ = sym::gScrWarp; c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32(sym::gScrWarp); F::memset(c, a0_, 0i32, 172u32) }; };
    l_a_2 = F32(0xc0490fda);
    { let v_ = F::Heap_Alloc(c, (((((10i32) as u32)).wrapping_mul(1136u32)) as i32), ((32i32) as u32), 0i32, 2i32); let a_ = c.m.r32(sym::gScrWarp); c.m.w32(a_, v_); };
    { let _ = { let a0_ = { let a_ = c.m.r32(sym::gScrWarp); c.m.r32(a_) }; F::memset(c, a0_, 0i32, (((10i32) as u32)).wrapping_mul(1136u32)) }; };
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (20i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let v_ = F::Mathf_Sin(c, l_a_2); let a_ = (((c.m.r32(sym::gScrWarp)).wrapping_add(4)).wrapping_add((((l_i_1) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                { let v_ = F::Mathf_Cos(c, l_a_2); let a_ = (((c.m.r32(sym::gScrWarp)).wrapping_add(4)).wrapping_add((((l_i_1) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                { let r_ = F32(0x3ea0d97b); let n_ = rt::fadd(l_a_2, r_); l_a_2 = n_; };
                if (((rt::fle(F32(0x40490fda), l_a_2) as i32)) != 0) {
                    { let r_ = F32(0x40c90fda); let n_ = rt::fsub(l_a_2, r_); l_a_2 = n_; };
                }
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ScrWarp_Init", &PROBE_N, &[], true); }
    r_
}

pub fn ScrWarp_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ScrWarp_Term", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if (((((c.m.r32(sym::gScrWarp)) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = { let a_ = c.m.r32(sym::gScrWarp); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
        { let a0_ = c.m.r32(sym::gScrWarp); F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = sym::gScrWarp; c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ScrWarp_Term", &PROBE_N, &[], true); }
    r_
}

pub fn ScrWarp_Reset(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ScrWarp_Reset", &PROBE_N, &[], false); }
    let mut l_o_1: u32 = 0;
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
    l_o_1 = { let a_ = c.m.r32(sym::gScrWarp); c.m.r32(a_) };
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < (10i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let v_ = 0i32; let a_ = l_o_1; c.m.w32(a_, (v_) as u32); };
            }
            { { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; }; { let o_ = l_o_1; let n_ = (o_).wrapping_add(1136); l_o_1 = n_; }; };
        }
    }
    { let v_ = 0i32; let a_ = ((c.m.r32(sym::gScrWarp)).wrapping_add(164)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    { let v_ = 0i32; let a_ = ((c.m.r32(sym::gScrWarp)).wrapping_add(164)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ScrWarp_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn ScrWarp_UpdateAll(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ScrWarp_UpdateAll", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(80) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 80);
    let mut l_o_1: u32 = 0;
    let mut l_i_2: i32 = 0;
    let mut l_x_3: F32 = F32(0);
    let mut l_y_4: F32 = F32(0);
    let r_ = 'f: {
        l_o_1 = { let a_ = c.m.r32(sym::gScrWarp); c.m.r32(a_) };
        {
            l_i_2 = 0i32;
            'l1: loop {
                if !(((((l_i_2) < (10i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    if ((((((c.m.r32(l_o_1) as i32)) == (0i32)) as i32)) != 0) {
                        break 'c2;
                    }
                    if ((((!(((((c.m.r32(l_o_1) as i32)) & (2i32))) != 0)) as i32)) != 0) {
                        if ((((((F::Battle_IsSplitScreen(c)) != 0) && ((((!((F::BtlScene_IsSingleView(c)) != 0)) as i32)) != 0)) as i32)) != 0) {
                            { let a1_ = (((c.m.r32(sym::gBtlCam)).wrapping_add(1824)).wrapping_add(((((c.m.r32((l_o_1).wrapping_add(4)) as i32)) as u32)).wrapping_mul(656))).wrapping_add(320); F::Mtx_Copy(c, fp, a1_) };
                        } else {
                            F::Vu0Screen_StoreMtx(c, fp);
                        }
                        F::Mtx_ProjectInt(c, fp.wrapping_add(64), fp, (l_o_1).wrapping_add(48));
                        F::IVec4_ToFloat(c, (l_o_1).wrapping_add(64), fp.wrapping_add(64));
                        if ((((!((F::ScrWarp_IsOnScreen(c, (l_o_1).wrapping_add(64))) != 0)) as i32)) != 0) {
                            F::ScrWarp_Free(c, l_o_1);
                            break 'c2;
                        }
                        { let r_ = F32(0x44e00000); let a_ = (l_o_1).wrapping_add(64); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                        l_y_4 = rt::fsub(F32(c.m.r32(((l_o_1).wrapping_add(64)).wrapping_add(4))), F32(0x44e40000));
                        { let v_ = l_y_4; let a_ = ((l_o_1).wrapping_add(64)).wrapping_add(4); c.m.w32(a_, (v_).0); };
                        l_x_3 = F32(c.m.r32((l_o_1).wrapping_add(64)));
                        { let v_ = l_y_4; let a_ = (l_o_1).wrapping_add(84); c.m.w32(a_, (v_).0); };
                        { let v_ = l_x_3; let a_ = (l_o_1).wrapping_add(80); c.m.w32(a_, (v_).0); };
                        if ((((((((((F::Battle_IsSplitScreen(c)) != 0) && ((((!((F::BtlScene_IsSingleView(c)) != 0)) as i32)) != 0)) as i32)) != 0) && ((((((c.m.r32((l_o_1).wrapping_add(4)) as i32)) == (1i32)) as i32)) != 0)) as i32)) != 0) {
                            { let r_ = F32(0x43800000); let a_ = (l_o_1).wrapping_add(64); let n_ = rt::fsub(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                        }
                        { let r_ = 2i32; let a_ = l_o_1; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    }
                    F::ScrWarp_Step(c, l_o_1);
                }
                { { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; }; { let o_ = l_o_1; let n_ = (o_).wrapping_add(1136); l_o_1 = n_; }; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "ScrWarp_UpdateAll", &PROBE_N, &[], true); }
    r_
}

pub fn ScrWarp_DrawAll(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ScrWarp_DrawAll", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if ((((((((!((F::Battle_IsSplitScreen(c)) != 0)) as i32)) != 0) || ((F::BtlScene_IsSingleView(c)) != 0)) as i32)) != 0) {
        F::ScrWarp_Draw(c, 0i32, 0i32);
    } else {
        if ((((({ let a_ = ((c.m.r32(sym::gScrWarp)).wrapping_add(164)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) > (0i32)) as i32)) != 0) {
            F::ScrWarp_Draw(c, 1i32, 0i32);
        }
        if ((((({ let a_ = ((c.m.r32(sym::gScrWarp)).wrapping_add(164)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) > (0i32)) as i32)) != 0) {
            F::ScrWarp_Draw(c, 1i32, 1i32);
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ScrWarp_DrawAll", &PROBE_N, &[], true); }
    r_
}

pub fn ScrWarp_Update(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ScrWarp_Update", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if ((((((((({ let a_ = ((c.m.r32(sym::gScrWarp)).wrapping_add(164)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) > (0i32)) as i32)) != 0) || ((((({ let a_ = ((c.m.r32(sym::gScrWarp)).wrapping_add(164)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) > (0i32)) as i32)) != 0)) as i32)) != 0) {
        if ((((!(((({ let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); c.m.r64(a_) }) & (((256i32) as u64)))) != 0)) as i32)) != 0) {
            F::ScrWarp_UpdateAll(c);
        }
        F::ScrWarp_DrawAll(c);
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "ScrWarp_Update", &PROBE_N, &[], true); }
    r_
}

pub fn ScrWarp_Spawn(c: &mut Ctx, mut p_view_1: i32, mut p_pos_2: u32, mut p_seconds_3: F32, mut p_radius_4: F32, mut p_width_5: F32, mut p_speed_6: F32, mut p_jitter_7: F32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ScrWarp_Spawn", &PROBE_N, &[(p_view_1) as u32, (p_pos_2) as u32, (p_seconds_3).0, (p_radius_4).0, (p_width_5).0, (p_speed_6).0, (p_jitter_7).0], false); }
    let mut l_o_8: u32 = 0;
    let mut l_i_9: i32 = 0;
    let mut l_base_10: F32 = F32(0);
    let r_ = 'f: {
    l_o_8 = F::ScrWarp_Alloc(c, p_view_1);
    if (((((l_o_8) == (((0i32) as u32))) as i32)) != 0) {
        break 'f 0i32;
    }
    { let v_ = p_view_1; let a_ = (l_o_8).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
    { let r_ = 1i32; let a_ = l_o_8; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    { let v_ = rt::fmul(p_seconds_3, F32(0x41f00000)); let a_ = (l_o_8).wrapping_add(8); c.m.w32(a_, (v_).0); };
    F::Vec4_Copy(c, (l_o_8).wrapping_add(48), p_pos_2);
    { let v_ = p_radius_4; let a_ = (l_o_8).wrapping_add(12); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x3f008080); let a_ = (l_o_8).wrapping_add(32); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0xbdab5600); let a_ = (l_o_8).wrapping_add(36); c.m.w32(a_, (v_).0); };
    { let v_ = p_width_5; let a_ = (l_o_8).wrapping_add(24); c.m.w32(a_, (v_).0); };
    { let v_ = p_speed_6; let a_ = (l_o_8).wrapping_add(16); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fneg(rt::fdiv(rt::fmul(p_speed_6, F32(0x3e999999)), F32(c.m.r32((l_o_8).wrapping_add(8))))); let a_ = (l_o_8).wrapping_add(20); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fdiv(rt::fmul(p_width_5, F32(0x404ccccc)), F32(c.m.r32((l_o_8).wrapping_add(8)))); let a_ = (l_o_8).wrapping_add(28); c.m.w32(a_, (v_).0); };
    l_base_10 = rt::fsub(F32(0x3f000000), rt::fmul(p_jitter_7, F32(0x3f000000)));
    {
        l_i_9 = 0i32;
        'l1: loop {
            if !(((((l_i_9) < (20i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let v_ = rt::fadd(l_base_10, rt::fmul(F::Rand_Float01(c), p_jitter_7)); let a_ = ((l_o_8).wrapping_add(96)).wrapping_add((((l_i_9) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            }
            { let o_ = l_i_9; let n_ = (o_).wrapping_add(1); l_i_9 = n_; };
        }
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "ScrWarp_Spawn", &PROBE_N, &[], true); }
    r_
}

pub fn StgFog_SetupTex(c: &mut Ctx, mut p_tone_1: u32, mut p_arg_2: u16) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgFog_SetupTex", &PROBE_N, &[(p_tone_1) as u32, (p_arg_2) as u32], false); }
    let r_ = 'f: {
    F::GfxClut_InitPacket(c, p_tone_1, p_arg_2);
    { let v_ = c.m.r32((p_tone_1).wrapping_add(1168)); let a_ = (p_tone_1).wrapping_add(1216); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgFog_SetupTex", &PROBE_N, &[], true); }
    r_
}

pub fn StgFog_BuildClut(c: &mut Ctx, mut p_tone_1: u32, mut p_pts_2: u32, mut p_clearLast_3: i32, mut p_scale_4: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgFog_BuildClut", &PROBE_N, &[(p_tone_1) as u32, (p_pts_2) as u32, (p_clearLast_3) as u32, (p_scale_4).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(1024) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 1024);
    let mut l_clut_5: u32 = 0;
    let mut l_i_6: i32 = 0;
    let mut l_idx_7: i32 = 0;
    let r_ = 'f: {
        l_clut_5 = c.m.r32((p_tone_1).wrapping_add(1216));
        { let _ = F::memset(c, fp, 0i32, 1024u32); };
        F::StgCurve_Build(c, p_pts_2, fp);
        {
            l_i_6 = 0i32;
            'l1: loop {
                if !(((((l_i_6) < (256i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_idx_7 = ((((((l_i_6) & (231i32))) | ((((l_i_6) & (8i32))).wrapping_shl((1i32) as u32)))) | ((((l_i_6) & (16i32))).wrapping_shr((1i32) as u32)));
                    { let v_ = ((0i32) as u8); let a_ = (l_clut_5).wrapping_add(((((l_idx_7).wrapping_shl((2i32) as u32)).wrapping_add(0i32)) as u32)); c.m.w8(a_, v_); };
                    { let v_ = ((0i32) as u8); let a_ = (l_clut_5).wrapping_add(((((l_idx_7).wrapping_shl((2i32) as u32)).wrapping_add(1i32)) as u32)); c.m.w8(a_, v_); };
                    { let v_ = ((0i32) as u8); let a_ = (l_clut_5).wrapping_add(((((l_idx_7).wrapping_shl((2i32) as u32)).wrapping_add(2i32)) as u32)); c.m.w8(a_, v_); };
                    { let v_ = ((rt::f2u(rt::fmul(rt::fmul(F32(c.m.r32((fp).wrapping_add((((l_i_6) as u32)).wrapping_mul(4)))), F32(0x43000000)), p_scale_4))) as u8); let a_ = (l_clut_5).wrapping_add(((((l_idx_7).wrapping_shl((2i32) as u32)).wrapping_add(3i32)) as u32)); c.m.w8(a_, v_); };
                }
                { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; };
            }
        }
        if ((p_clearLast_3) != 0) {
            { let v_ = ((0i32) as u8); let a_ = (l_clut_5).wrapping_add(((1020i32) as u32)); c.m.w8(a_, v_); };
            { let v_ = ((0i32) as u8); let a_ = (l_clut_5).wrapping_add(((1021i32) as u32)); c.m.w8(a_, v_); };
            { let v_ = ((0i32) as u8); let a_ = (l_clut_5).wrapping_add(((1022i32) as u32)); c.m.w8(a_, v_); };
            { let v_ = ((0i32) as u8); let a_ = (l_clut_5).wrapping_add(((1023i32) as u32)); c.m.w8(a_, v_); };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "StgFog_BuildClut", &PROBE_N, &[], true); }
    r_
}

pub fn StgFog_Init(c: &mut Ctx, mut p_arg_1: u16) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgFog_Init", &PROBE_N, &[(p_arg_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = F::Heap_Alloc(c, ((1536u32) as i32), ((32i32) as u32), 0i32, 2i32); let a_ = sym::gStgFog; c.m.w32(a_, v_); };
    { let _ = { let a0_ = c.m.r32(sym::gStgFog); F::memset(c, a0_, 0i32, 1536u32) }; };
    { let a0_ = c.m.r32(sym::gStgFog); F::StgFog_SetupTex(c, a0_, p_arg_1) };
    F::StgFog_ResetColor(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgFog_Init", &PROBE_N, &[], true); }
    r_
}

pub fn StgFog_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgFog_Term", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if (((((c.m.r32(sym::gStgFog)) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32(sym::gStgFog); F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = sym::gStgFog; c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgFog_Term", &PROBE_N, &[], true); }
    r_
}

pub fn StgFog_ResetColor(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgFog_ResetColor", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let v_ = 2155905152u32; let a_ = (c.m.r32(sym::gStgFog)).wrapping_add(1488); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgFog_ResetColor", &PROBE_N, &[], true); }
    r_
}

pub fn StgFog_Draw(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgFog_Draw", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_tone_1: u32 = 0;
    let mut l_width_2: i32 = 0;
    let mut l_half_3: i32 = 0;
    let mut l_srcH_4: i32 = 0;
    let mut l_h_5: i32 = 0;
    let mut l_tw_6: i32 = 0;
    let mut l_th_7: i32 = 0;
    let mut l_fbw_8: u64 = 0;
    let r_ = 'f: {
        l_tone_1 = c.m.r32(sym::gStgFog);
        l_width_2 = 512i32;
        l_half_3 = 256i32;
        l_srcH_4 = 448i32;
        l_h_5 = 224i32;
        F::Dma_AddData(c, (l_tone_1).wrapping_add(1172), 16i32);
        F::Dma_AddTexFlush(c);
        { let a0_ = (if ((((!(((((c.m.r32((sym::gGfx).wrapping_add(564)) as i32)) & (1i32))) != 0)) as i32)) != 0) { 112i32 } else { 0i32 }); F::Dma_AddFrame(c, a0_, 8i32, 16777215i32) };
        F::Dma_AddZbuf(c, 224i32, 1i32);
        { let a2_ = ((c.m.r16((l_tone_1).wrapping_add(1204))) as i32); F::GfxPost_DrawDepthClut(c, 0i32, 7168i32, a2_, (((128u64).wrapping_shl((32i32) as u32)) | (((72i32) as u64)))) };
        F::Dma_AddZbuf(c, 224i32, 0i32);
        {
            rt::memset(c, fp.wrapping_add(4), 0, 8);
            { let v_ = ((10752i32) as u32); c.m.w32(fp.wrapping_add(4).wrapping_add(0), v_); }
            { let v_ = ((11648i32) as u32); c.m.w32(fp.wrapping_add(4).wrapping_add(4), v_); }
            rt::memset(c, fp.wrapping_add(12), 0, 8);
            { let v_ = ((336i32) as u32); c.m.w32(fp.wrapping_add(12).wrapping_add(0), v_); }
            { let v_ = ((364i32) as u32); c.m.w32(fp.wrapping_add(12).wrapping_add(4), v_); }
            l_fbw_8 = ((4i32) as u64);
            l_tw_6 = F::Tex_Log2Size(c, 256i32);
            l_th_7 = F::Tex_Log2Size(c, 224i32);
            { let v_ = F::Dma_BeginDirect(c); let a_ = fp; c.m.w32(a_, v_); };
            F::Gfx_PutDefaultEnv(c, fp);
            { let v_ = ((((((((4i32) as u64)) | ((((0i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (c.m.r32(fp)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let v_ = ((14i32) as u64); let a_ = (c.m.r32(fp)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let r_ = 2i32; let a_ = fp; let n_ = (c.m.r32(a_)).wrapping_add((((r_) as u32)).wrapping_mul(8)); c.m.w32(a_, n_); };
            { let v_ = ((((c.m.r32((fp.wrapping_add(12)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))) as u64)) | ((l_fbw_8).wrapping_shl((16i32) as u32))); let a_ = (c.m.r32(fp)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let v_ = ((76i32) as u64); let a_ = (c.m.r32(fp)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let r_ = 2i32; let a_ = fp; let n_ = (c.m.r32(a_)).wrapping_add((((r_) as u32)).wrapping_mul(8)); c.m.w32(a_, n_); };
            { let v_ = ((((((224i32) as u64)) | ((((49i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((32i32) as u32))); let a_ = (c.m.r32(fp)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let v_ = ((78i32) as u64); let a_ = (c.m.r32(fp)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let r_ = 2i32; let a_ = fp; let n_ = (c.m.r32(a_)).wrapping_add((((r_) as u32)).wrapping_mul(8)); c.m.w32(a_, n_); };
            { let v_ = ((196608i32) as u64); let a_ = (c.m.r32(fp)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let v_ = ((71i32) as u64); let a_ = (c.m.r32(fp)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let r_ = 2i32; let a_ = fp; let n_ = (c.m.r32(a_)).wrapping_add((((r_) as u32)).wrapping_mul(8)); c.m.w32(a_, n_); };
            { let v_ = (if ((((!(((((c.m.r32((sym::gGfx).wrapping_add(564)) as i32)) & (1i32))) != 0)) as i32)) != 0) { 44627529216u64 } else { 44627525632u64 }); let a_ = (c.m.r32(fp)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let v_ = ((6i32) as u64); let a_ = (c.m.r32(fp)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let r_ = 2i32; let a_ = fp; let n_ = (c.m.r32(a_)).wrapping_add((((r_) as u32)).wrapping_mul(8)); c.m.w32(a_, n_); };
            F::Dma_PutTexStrips(c, fp, 0i32, 0i32, l_half_3, l_h_5, 0i32, 0i32, 0i32, 0i32, l_width_2, l_srcH_4, 0i32, 0i32, 2155905152u32, 1i32);
            { let v_ = ((((((((3i32) as u64)) | ((((0i32) as u64)).wrapping_shl((15i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((58i32) as u32)))) | ((((1i32) as u64)).wrapping_shl((60i32) as u32))); let a_ = (c.m.r32(fp)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let v_ = ((14i32) as u64); let a_ = (c.m.r32(fp)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let r_ = 2i32; let a_ = fp; let n_ = (c.m.r32(a_)).wrapping_add((((r_) as u32)).wrapping_mul(8)); c.m.w32(a_, n_); };
            { let v_ = (if ((((!(((((c.m.r32((sym::gGfx).wrapping_add(564)) as i32)) & (1i32))) != 0)) as i32)) != 0) { 18374686479672148080u64 } else { 18374686479672147968u64 }); let a_ = (c.m.r32(fp)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let v_ = ((76i32) as u64); let a_ = (c.m.r32(fp)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let r_ = 2i32; let a_ = fp; let n_ = (c.m.r32(a_)).wrapping_add((((r_) as u32)).wrapping_mul(8)); c.m.w32(a_, n_); };
            { let v_ = (((128u64).wrapping_shl((32i32) as u32)) | (((84i32) as u64))); let a_ = (c.m.r32(fp)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let v_ = ((66i32) as u64); let a_ = (c.m.r32(fp)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let r_ = 2i32; let a_ = fp; let n_ = (c.m.r32(a_)).wrapping_add((((r_) as u32)).wrapping_mul(8)); c.m.w32(a_, n_); };
            { let v_ = ((((((((c.m.r32((fp.wrapping_add(4)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))) as u64)) | ((l_fbw_8).wrapping_shl((14i32) as u32)))) | ((((l_tw_6) as u64)).wrapping_shl((26i32) as u32)))) | ((((l_th_7) as u64)).wrapping_shl((30i32) as u32))); let a_ = (c.m.r32(fp)).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let v_ = ((6i32) as u64); let a_ = (c.m.r32(fp)).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
            { let r_ = 2i32; let a_ = fp; let n_ = (c.m.r32(a_)).wrapping_add((((r_) as u32)).wrapping_mul(8)); c.m.w32(a_, n_); };
            F::Dma_PutTexStrips(c, fp, 0i32, 0i32, l_width_2, l_srcH_4, 0i32, 0i32, 0i32, 0i32, l_half_3, l_h_5, 8i32, 8i32, 2155905152u32, 1i32);
            { let a0_ = c.m.r32(fp); F::Dma_EndDirect(c, a0_) };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "StgFog_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn StgFog_SetParams(c: &mut Ctx, mut p_prm_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "StgFog_SetParams", &PROBE_N, &[(p_prm_1) as u32], false); }
    let mut l_pt_2: u32 = 0;
    let r_ = 'f: {
    l_pt_2 = (p_prm_1).wrapping_add(4);
    if (((((p_prm_1) == (((0i32) as u32))) as i32)) != 0) {
        { let v_ = 1i32; let a_ = (c.m.r32(sym::gStgFog)).wrapping_add(1492); c.m.w32(a_, (v_) as u32); };
        { let a0_ = ((c.m.r32(sym::gStgFog)).wrapping_add(1280)).wrapping_add((((0i32) as u32)).wrapping_mul(64)); F::Vec4_Set(c, a0_, F32(0x00000000), F32(0x00000000), F32(0x00000000), F32(0x3f800000)) };
        { let a0_ = ((c.m.r32(sym::gStgFog)).wrapping_add(1280)).wrapping_add((((1i32) as u32)).wrapping_mul(64)); F::Vec4_Set(c, a0_, F32(0x435e0000), F32(0x00000000), F32(0x41b80000), F32(0x3f800000)) };
        { let a0_ = ((c.m.r32(sym::gStgFog)).wrapping_add(1280)).wrapping_add((((2i32) as u32)).wrapping_mul(64)); F::Vec4_Set(c, a0_, F32(0x437f0000), F32(0x00000000), F32(0x437f0000), F32(0x3f800000)) };
    } else {
        { let v_ = ((((c.m.r32(p_prm_1) as i32)).wrapping_shr((1i32) as u32)) & (1i32)); let a_ = (c.m.r32(sym::gStgFog)).wrapping_add(1492); c.m.w32(a_, (v_) as u32); };
        { let a0_ = ((c.m.r32(sym::gStgFog)).wrapping_add(1280)).wrapping_add((((0i32) as u32)).wrapping_mul(64)); let a1_ = rt::i2f((c.m.r8((l_pt_2).wrapping_add(((0i32) as u32)))) as i32); let a3_ = rt::i2f((c.m.r8((l_pt_2).wrapping_add(((1i32) as u32)))) as i32); F::Vec4_Set(c, a0_, a1_, F32(0x00000000), a3_, F32(0x3f800000)) };
        { let a0_ = ((c.m.r32(sym::gStgFog)).wrapping_add(1280)).wrapping_add((((1i32) as u32)).wrapping_mul(64)); let a1_ = rt::i2f((c.m.r8((l_pt_2).wrapping_add(((2i32) as u32)))) as i32); let a3_ = rt::i2f((c.m.r8((l_pt_2).wrapping_add(((3i32) as u32)))) as i32); F::Vec4_Set(c, a0_, a1_, F32(0x00000000), a3_, F32(0x3f800000)) };
        { let a0_ = ((c.m.r32(sym::gStgFog)).wrapping_add(1280)).wrapping_add((((2i32) as u32)).wrapping_mul(64)); let a1_ = rt::i2f((c.m.r8((l_pt_2).wrapping_add(((4i32) as u32)))) as i32); let a3_ = rt::i2f((c.m.r8((l_pt_2).wrapping_add(((5i32) as u32)))) as i32); F::Vec4_Set(c, a0_, a1_, F32(0x00000000), a3_, F32(0x3f800000)) };
    }
    { let a0_ = c.m.r32(sym::gStgFog); let a1_ = (c.m.r32(sym::gStgFog)).wrapping_add(1280); let a2_ = { let a_ = (c.m.r32(sym::gStgFog)).wrapping_add(1492); (c.m.r32(a_) as i32) }; F::StgFog_BuildClut(c, a0_, a1_, a2_, F32(0x3f800000)) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "StgFog_SetParams", &PROBE_N, &[], true); }
    r_
}
