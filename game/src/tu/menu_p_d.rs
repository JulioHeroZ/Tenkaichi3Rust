#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn UbScore_Fill(c: &mut Ctx, mut p_kind_1: i32, mut p_score_2: u32, mut p_pages_3: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "UbScore_Fill", &PROBE_N, &[(p_kind_1) as u32, (p_score_2) as u32, (p_pages_3) as u32], false); }
    let mut l_outcome_4: i32 = 0;
    let mut l_result_5: u32 = 0;
    let mut l_i_6: i32 = 0;
    let mut l_n_7: i32 = 0;
    let mut l_q_8: i32 = 0;
    let mut l_m_9: i32 = 0;
    let r_ = 'f: {
    l_outcome_4 = 2i32;
    l_result_5 = F::BattleResult_GetPtr(c);
    if ((((((c.m.r32((l_result_5).wrapping_add(8)) as i32)) == (0i32)) as i32)) != 0) {
        if (((((c.m.r32(l_result_5) as i32)) & (1i32))) != 0) {
            l_outcome_4 = 0i32;
        } else {
            l_outcome_4 = 1i32;
        }
    }
    { let _ = F::memset(c, p_score_2, 0i32, 680u32); };
    { let v_ = 0i32; let a_ = p_pages_3; c.m.w32(a_, (v_) as u32); };
    if (((((l_outcome_4) != (2i32)) as i32)) != 0) {
        if (((((((rt::flt(F32(0x00000000), F32(c.m.r32(((l_result_5).wrapping_add(44)).wrapping_add((((0i32) as u32)).wrapping_mul(4))))) as i32)) != 0) && (((rt::flt(F32(c.m.r32(((l_result_5).wrapping_add(44)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(0x3f800000)) as i32)) != 0)) as i32)) != 0) {
            { let v_ = 1i32; let a_ = ((p_score_2).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(12)); c.m.w32(a_, (v_) as u32); };
        } else {
            { let v_ = rt::f2i(F32(c.m.r32(((l_result_5).wrapping_add(44)).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a_ = ((p_score_2).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(12)); c.m.w32(a_, (v_) as u32); };
        }
        { let v_ = (c.m.r32(((l_result_5).wrapping_add(36)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32); let a_ = ((p_score_2).wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(12)); c.m.w32(a_, (v_) as u32); };
        { let v_ = (c.m.r32(((l_result_5).wrapping_add(28)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32); let a_ = ((p_score_2).wrapping_add(16)).wrapping_add((((2i32) as u32)).wrapping_mul(12)); c.m.w32(a_, (v_) as u32); };
        if (((((p_kind_1) == (2i32)) as i32)) != 0) {
            { let v_ = (c.m.r32((l_result_5).wrapping_add(24)) as i32); let a_ = ((p_score_2).wrapping_add(16)).wrapping_add((((3i32) as u32)).wrapping_mul(12)); c.m.w32(a_, (v_) as u32); };
            { let v_ = 4i32; let a_ = (p_score_2).wrapping_add(652); c.m.w32(a_, (v_) as u32); };
        } else {
            { let v_ = 0i32; let a_ = ((p_score_2).wrapping_add(16)).wrapping_add((((3i32) as u32)).wrapping_mul(12)); c.m.w32(a_, (v_) as u32); };
            { let v_ = 3i32; let a_ = (p_score_2).wrapping_add(652); c.m.w32(a_, (v_) as u32); };
        }
        if ((((((c.m.r32(((p_score_2).wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(12))) as i32)) > (100i32)) as i32)) != 0) {
            { let v_ = 100i32; let a_ = ((p_score_2).wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(12)); c.m.w32(a_, (v_) as u32); };
        }
        if ((((((c.m.r32(((p_score_2).wrapping_add(16)).wrapping_add((((2i32) as u32)).wrapping_mul(12))) as i32)) > (99990i32)) as i32)) != 0) {
            { let v_ = 99990i32; let a_ = ((p_score_2).wrapping_add(16)).wrapping_add((((2i32) as u32)).wrapping_mul(12)); c.m.w32(a_, (v_) as u32); };
        }
        {
            l_i_6 = 0i32;
            'l1: loop {
                if !(((((l_i_6) < (48i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    if (((((c.m.r64((l_result_5).wrapping_add(16))).wrapping_shr((l_i_6) as u32)) & (((1i32) as u64)))) != 0) {
                        { let v_ = l_i_6; let a_ = ((p_score_2).wrapping_add(64)).wrapping_add(((({ let a_ = (p_score_2).wrapping_add(656); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); o_ }) as u32)).wrapping_mul(12)); c.m.w32(a_, (v_) as u32); };
                    }
                }
                { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; };
            }
        }
        l_n_7 = 3i32;
        l_m_9 = 3i32;
        l_q_8 = rt::div_i32((c.m.r32((p_score_2).wrapping_add(656)) as i32), l_n_7);
        if ((rt::rem_i32((c.m.r32((p_score_2).wrapping_add(656)) as i32), l_m_9)) != 0) {
            { let o_ = l_q_8; let n_ = (o_).wrapping_add(1); l_q_8 = n_; };
        }
        { let v_ = l_q_8; let a_ = p_pages_3; c.m.w32(a_, (v_) as u32); };
        { let s_ = (l_result_5).wrapping_add(52); let d_ = (p_score_2).wrapping_add(660); rt::memcpy(c, d_, s_, 16); };
        if ((((((((c.m.r16(((p_score_2).wrapping_add(660)).wrapping_add(4)) as i16)) as i32)) >= (100i32)) as i32)) != 0) {
            { let v_ = ((99i32) as i16); let a_ = ((p_score_2).wrapping_add(660)).wrapping_add(4); c.m.w16(a_, (v_) as u16); };
        }
    }
    break 'f l_outcome_4;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "UbScore_Fill", &PROBE_N, &[], true); }
    r_
}

pub fn UbScore_CalcPoints(c: &mut Ctx, mut p_price_1: u32, mut p_score_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "UbScore_CalcPoints", &PROBE_N, &[(p_price_1) as u32, (p_score_2) as u32], false); }
    let mut l_i_3: i32 = 0;
    let mut l_j_4: i32 = 0;
    let r_ = 'f: {
    {
        l_i_3 = 0i32;
        'l1: loop {
            if !(((((l_i_3) < (4i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if (((((l_i_3) != (2i32)) as i32)) != 0) {
                    { let v_ = { let a_ = ((p_price_1).wrapping_add(((((c.m.r32(((p_score_2).wrapping_add(16)).wrapping_add((((l_i_3) as u32)).wrapping_mul(12))) as i32)) as u32)).wrapping_mul(24))).wrapping_add((((l_i_3) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (((p_score_2).wrapping_add(16)).wrapping_add((((l_i_3) as u32)).wrapping_mul(12))).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                } else {
                    {
                        l_j_4 = 0i32;
                        'l3: loop {
                            if !(((((l_j_4) < (100i32)) as i32)) != 0) { break 'l3; }
                            'c4: {
                                if ((((!((((((c.m.r32((sym::gUbScoreTimeSteps).wrapping_add((((l_j_4) as u32)).wrapping_mul(4))) as i32)) < ((c.m.r32(((p_score_2).wrapping_add(16)).wrapping_add((((l_i_3) as u32)).wrapping_mul(12))) as i32))) as i32)) != 0)) as i32)) != 0) {
                                    { let v_ = (c.m.r32(((p_price_1).wrapping_add((((l_j_4) as u32)).wrapping_mul(24))).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32); let a_ = (((p_score_2).wrapping_add(16)).wrapping_add((((l_i_3) as u32)).wrapping_mul(12))).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                                    break 'l3;
                                }
                                if (((((l_j_4) == (100i32)) as i32)) != 0) {
                                    { let v_ = (c.m.r32(((p_price_1).wrapping_add((((100i32) as u32)).wrapping_mul(24))).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32); let a_ = (((p_score_2).wrapping_add(16)).wrapping_add((((l_i_3) as u32)).wrapping_mul(12))).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                                }
                            }
                            { let o_ = l_j_4; let n_ = (o_).wrapping_add(1); l_j_4 = n_; };
                        }
                    }
                }
                { let v_ = (c.m.r32((((p_score_2).wrapping_add(16)).wrapping_add((((l_i_3) as u32)).wrapping_mul(12))).wrapping_add(4)) as i32); let a_ = (((p_score_2).wrapping_add(16)).wrapping_add((((l_i_3) as u32)).wrapping_mul(12))).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
            }
            { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
        }
    }
    {
        l_i_3 = 0i32;
        'l5: loop {
            if !(((((l_i_3) < ((c.m.r32((p_score_2).wrapping_add(656)) as i32))) as i32)) != 0) { break 'l5; }
            'c6: {
                { let v_ = { let a_ = ((p_price_1).wrapping_add(((((c.m.r32(((p_score_2).wrapping_add(64)).wrapping_add((((l_i_3) as u32)).wrapping_mul(12))) as i32)) as u32)).wrapping_mul(24))).wrapping_add(16); (c.m.r32(a_) as i32) }; let a_ = (((p_score_2).wrapping_add(64)).wrapping_add((((l_i_3) as u32)).wrapping_mul(12))).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                { let v_ = (c.m.r32((((p_score_2).wrapping_add(64)).wrapping_add((((l_i_3) as u32)).wrapping_mul(12))).wrapping_add(4)) as i32); let a_ = (((p_score_2).wrapping_add(64)).wrapping_add((((l_i_3) as u32)).wrapping_mul(12))).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
            }
            { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "UbScore_CalcPoints", &PROBE_N, &[], true); }
    r_
}

pub fn UbScore_CalcRank(c: &mut Ctx, mut p_kind_1: i32, mut p_score_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "UbScore_CalcRank", &PROBE_N, &[(p_kind_1) as u32, (p_score_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_row_3: i32 = 0;
    let mut l_total_4: i32 = 0;
    let mut l_i_5: i32 = 0;
    let r_ = 'f: {
        rt::memset(c, fp, 0, 32);
        rt::memset(c, fp.wrapping_add(0), 0, 16);
        { let v_ = 1599i32; c.m.w32(fp.wrapping_add(0).wrapping_add(0), (v_) as u32); }
        { let v_ = 2199i32; c.m.w32(fp.wrapping_add(0).wrapping_add(4), (v_) as u32); }
        { let v_ = 2799i32; c.m.w32(fp.wrapping_add(0).wrapping_add(8), (v_) as u32); }
        { let v_ = 9999i32; c.m.w32(fp.wrapping_add(0).wrapping_add(12), (v_) as u32); }
        rt::memset(c, fp.wrapping_add(16), 0, 16);
        { let v_ = 2099i32; c.m.w32(fp.wrapping_add(16).wrapping_add(0), (v_) as u32); }
        { let v_ = 2499i32; c.m.w32(fp.wrapping_add(16).wrapping_add(4), (v_) as u32); }
        { let v_ = 3199i32; c.m.w32(fp.wrapping_add(16).wrapping_add(8), (v_) as u32); }
        { let v_ = 9999i32; c.m.w32(fp.wrapping_add(16).wrapping_add(12), (v_) as u32); }
        if (((((((((p_kind_1) == (2i32)) as i32)) != 0) || (((((p_kind_1) == (4i32)) as i32)) != 0)) as i32)) != 0) {
            l_row_3 = 1i32;
        } else {
            l_row_3 = 0i32;
        }
        l_total_4 = 0i32;
        { let v_ = 0i32; let a_ = (p_score_2).wrapping_add(676); c.m.w32(a_, (v_) as u32); };
        {
            l_i_5 = 0i32;
            'l1: loop {
                if !(((((l_i_5) < ((c.m.r32((p_score_2).wrapping_add(652)) as i32))) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let r_ = (c.m.r32((((p_score_2).wrapping_add(16)).wrapping_add((((l_i_5) as u32)).wrapping_mul(12))).wrapping_add(4)) as i32); let n_ = (l_total_4).wrapping_add(r_); l_total_4 = n_; };
                }
                { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
            }
        }
        {
            l_i_5 = 0i32;
            'l3: loop {
                if !(((((l_i_5) < ((c.m.r32((p_score_2).wrapping_add(656)) as i32))) as i32)) != 0) { break 'l3; }
                'c4: {
                    { let r_ = (c.m.r32((((p_score_2).wrapping_add(64)).wrapping_add((((l_i_5) as u32)).wrapping_mul(12))).wrapping_add(4)) as i32); let n_ = (l_total_4).wrapping_add(r_); l_total_4 = n_; };
                }
                { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
            }
        }
        {
            l_i_5 = 0i32;
            'l5: loop {
                if !(((((l_i_5) < (4i32)) as i32)) != 0) { break 'l5; }
                'c6: {
                    if ((((!((((((c.m.r32(((fp).wrapping_add((((l_row_3) as u32)).wrapping_mul(16))).wrapping_add((((l_i_5) as u32)).wrapping_mul(4))) as i32)) < (l_total_4)) as i32)) != 0)) as i32)) != 0) {
                        { let v_ = (l_i_5).wrapping_add(1i32); let a_ = (p_score_2).wrapping_add(676); c.m.w32(a_, (v_) as u32); };
                        break 'l5;
                    }
                }
                { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
            }
        }
        break 'f l_total_4;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "UbScore_CalcRank", &PROBE_N, &[], true); }
    r_
}

pub fn UbScore_Transfer(c: &mut Ctx, mut p_from_1: u32, mut p_to_2: u32, mut p_step_3: i32, mut p_rate_4: F32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "UbScore_Transfer", &PROBE_N, &[(p_from_1) as u32, (p_to_2) as u32, (p_step_3) as u32, (p_rate_4).0], false); }
    let r_ = 'f: {
    if ((((((c.m.r32(p_from_1) as i32)) <= (0i32)) as i32)) != 0) {
        break 'f 1i32;
    }
    { let _ = ((D::call(c, fid::Snd_PlaySe, &[((((2i32) as u32)) as u64), ((5i32) as i64 as u64)])) as i32); };
    if ((((((c.m.r32(p_from_1) as i32)) >= (p_step_3)) as i32)) != 0) {
        { let r_ = rt::f2i(rt::fmul(rt::i2f(p_step_3), p_rate_4)); let a_ = p_to_2; let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
        { let r_ = p_step_3; let a_ = p_from_1; let n_ = ((c.m.r32(a_) as i32)).wrapping_sub(r_); c.m.w32(a_, (n_) as u32); };
    } else {
        { let r_ = rt::f2i(rt::fmul(rt::i2f((c.m.r32(p_from_1) as i32)), p_rate_4)); let a_ = p_to_2; let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
        { let v_ = 0i32; let a_ = p_from_1; c.m.w32(a_, (v_) as u32); };
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "UbScore_Transfer", &PROBE_N, &[], true); }
    r_
}

pub fn UbScore_CountLine(c: &mut Ctx, mut p_score_1: u32, mut p_isBonus_2: i32, mut p_index_3: i32, mut p_step_4: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "UbScore_CountLine", &PROBE_N, &[(p_score_1) as u32, (p_isBonus_2) as u32, (p_index_3) as u32, (p_step_4) as u32], false); }
    let mut l_line_5: u32 = 0;
    let mut l_done_6: i32 = 0;
    let r_ = 'f: {
    l_line_5 = ((0i32) as u32);
    's1: {
        let sv7: i32 = p_isBonus_2;
        match sv7 {
            0i32 => {
                l_line_5 = ((p_score_1).wrapping_add(16)).wrapping_add((((p_index_3) as u32)).wrapping_mul(12));
                break 's1;
            }
            1i32 => {
                l_line_5 = ((p_score_1).wrapping_add(64)).wrapping_add((((p_index_3) as u32)).wrapping_mul(12));
                break 's1;
            }
            _ => {
            }
        }
    }
    l_done_6 = F::UbScore_Transfer(c, (l_line_5).wrapping_add(4), p_score_1, p_step_4, F32(0x3f800000));
    if ((((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1592); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
        if ((((((c.m.r32(p_score_1) as i32)) > (65535i32)) as i32)) != 0) {
            { let v_ = 65535i32; let a_ = p_score_1; c.m.w32(a_, (v_) as u32); };
        }
    }
    break 'f l_done_6;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "UbScore_CountLine", &PROBE_N, &[], true); }
    r_
}

pub fn UbScore_ConvertStep(c: &mut Ctx, mut p_score_1: u32, mut p_step_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "UbScore_ConvertStep", &PROBE_N, &[(p_score_1) as u32, (p_step_2) as u32], false); }
    let r_ = 'f: {
    break 'f F::UbScore_Transfer(c, p_score_1, (p_score_1).wrapping_add(8), p_step_2, F32(0x3dcccccc));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "UbScore_ConvertStep", &PROBE_N, &[], true); }
    r_
}

pub fn UbScore_SetPage(c: &mut Ctx, mut p_score_1: u32, mut p_page_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "UbScore_SetPage", &PROBE_N, &[(p_score_1) as u32, (p_page_2) as u32], false); }
    let mut l_i_3: i32 = 0;
    let mut l_first_4: i32 = 0;
    let r_ = 'f: {
    if (((((p_page_2) > (0i32)) as i32)) != 0) {
        l_first_4 = ((p_page_2).wrapping_mul(3i32)).wrapping_sub(3i32);
        {
            l_i_3 = 0i32;
            'l1: loop {
                if !(((((((((l_i_3) < (3i32)) as i32)) != 0) && ((((((l_first_4).wrapping_add(l_i_3)) < ((c.m.r32((p_score_1).wrapping_add(656)) as i32))) as i32)) != 0)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let v_ = (l_first_4).wrapping_add(l_i_3); let a_ = ((p_score_1).wrapping_add(640)).wrapping_add((((l_i_3) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                }
                { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "UbScore_SetPage", &PROBE_N, &[], true); }
    r_
}

pub fn UbScore_PlateGoto(c: &mut Ctx, mut p_flash_1: u32, mut p_plate_2: i32, mut p_on_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "UbScore_PlateGoto", &PROBE_N, &[(p_flash_1) as u32, (p_plate_2) as u32, (p_on_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(80) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 80);
    let mut l_plateFmt_4: u32 = 0;
    let mut l_arrowFmt_5: u32 = 0;
    let r_ = 'f: {
        l_plateFmt_4 = sym::menu_p_d__str0;
        l_arrowFmt_5 = sym::menu_p_d__str1;
        if ((p_on_3) != 0) {
            { let _ = F::sprintf(c, fp.wrapping_add(8), l_plateFmt_4, &[((p_plate_2) as i64 as u64)]); };
            F::Flash_FindLabel(c, p_flash_1, ((0i32) as u32), fp.wrapping_add(8), fp);
            F::Flash_ClipGotoLabel(c, p_flash_1, fp, sym::menu_p_d__str2);
            { let _ = F::sprintf(c, fp.wrapping_add(8), l_arrowFmt_5, &[((p_plate_2) as i64 as u64)]); };
            F::Flash_FindLabel(c, p_flash_1, ((0i32) as u32), fp.wrapping_add(8), fp);
            F::Flash_ClipGotoLabel(c, p_flash_1, fp, sym::menu_p_d__str3);
        } else {
            { let _ = F::sprintf(c, fp.wrapping_add(8), l_plateFmt_4, &[((p_plate_2) as i64 as u64)]); };
            F::Flash_FindLabel(c, p_flash_1, ((0i32) as u32), fp.wrapping_add(8), fp);
            F::Flash_ClipGotoLabel(c, p_flash_1, fp, sym::menu_p_d__str4);
            { let _ = F::sprintf(c, fp.wrapping_add(8), l_arrowFmt_5, &[((p_plate_2) as i64 as u64)]); };
            F::Flash_FindLabel(c, p_flash_1, ((0i32) as u32), fp.wrapping_add(8), fp);
            F::Flash_ClipGotoLabel(c, p_flash_1, fp, sym::menu_p_d__str5);
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "UbScore_PlateGoto", &PROBE_N, &[], true); }
    r_
}

pub fn UbScore_AddRanking(c: &mut Ctx, mut p_score_1: i32, mut p_chara_2: i32, mut p_cleared_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "UbScore_AddRanking", &PROBE_N, &[(p_score_1) as u32, (p_chara_2) as u32, (p_cleared_3) as u32], false); }
    let mut l_rank_4: u32 = 0;
    let mut l_i_5: i32 = 0;
    let mut l_j_6: i32 = 0;
    let r_ = 'f: {
    l_rank_4 = ((c.m.r32(sym::gSaveData)).wrapping_add(8)).wrapping_add(520);
    {
        l_i_5 = 0i32;
        'l1: loop {
            if !(((((l_i_5) < (10i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((((((c.m.r32(((l_rank_4).wrapping_add((((l_i_5) as u32)).wrapping_mul(12))).wrapping_add(4)) as i32)) < (p_score_1)) as i32)) != 0) {
                    {
                        l_j_6 = (10i32).wrapping_sub(2i32);
                        'l3: loop {
                            if !(((((l_j_6) >= (l_i_5)) as i32)) != 0) { break 'l3; }
                            'c4: {
                                { let s_ = (l_rank_4).wrapping_add((((l_j_6) as u32)).wrapping_mul(12)); let d_ = (l_rank_4).wrapping_add(((((l_j_6).wrapping_add(1i32)) as u32)).wrapping_mul(12)); rt::memcpy(c, d_, s_, 12); };
                            }
                            { let o_ = l_j_6; let n_ = (o_).wrapping_sub(1); l_j_6 = n_; };
                        }
                    }
                    { let v_ = p_score_1; let a_ = ((l_rank_4).wrapping_add((((l_i_5) as u32)).wrapping_mul(12))).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    { let v_ = p_chara_2; let a_ = (l_rank_4).wrapping_add((((l_i_5) as u32)).wrapping_mul(12)); c.m.w32(a_, (v_) as u32); };
                    { let v_ = (((((p_cleared_3) != (0i32)) as i32)) as u8); let a_ = ((l_rank_4).wrapping_add((((l_i_5) as u32)).wrapping_mul(12))).wrapping_add(8); c.m.w8(a_, v_); };
                    break 'f;
                }
            }
            { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "UbScore_AddRanking", &PROBE_N, &[], true); }
    r_
}

pub fn UbScore_SaveMissionBest(c: &mut Ctx, mut p_mission_1: i32, mut p_total_2: i32, mut p_score_3: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "UbScore_SaveMissionBest", &PROBE_N, &[(p_mission_1) as u32, (p_total_2) as u32, (p_score_3) as u32], false); }
    let mut l_record_4: i32 = 0;
    let r_ = 'f: {
    l_record_4 = 0i32;
    if ((((({ let a_ = ((((c.m.r32(sym::gSaveData)).wrapping_add(8)).wrapping_add(644)).wrapping_add((((p_mission_1) as u32)).wrapping_mul(12))).wrapping_add(8); (c.m.r32(a_) as i32) }) < (p_total_2)) as i32)) != 0) {
        { let v_ = p_total_2; let a_ = ((((c.m.r32(sym::gSaveData)).wrapping_add(8)).wrapping_add(644)).wrapping_add((((p_mission_1) as u32)).wrapping_mul(12))).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        l_record_4 = 1i32;
        { let v_ = (((c.m.r32((p_score_3).wrapping_add(676)) as i32)) as u8); let a_ = ((((c.m.r32(sym::gSaveData)).wrapping_add(8)).wrapping_add(644)).wrapping_add((((p_mission_1) as u32)).wrapping_mul(12))).wrapping_add(1); c.m.w8(a_, v_); };
        { let v_ = (((c.m.r16(((p_score_3).wrapping_add(660)).wrapping_add(4)) as i16)) as u8); let a_ = ((((c.m.r32(sym::gSaveData)).wrapping_add(8)).wrapping_add(644)).wrapping_add((((p_mission_1) as u32)).wrapping_mul(12))).wrapping_add(2); c.m.w8(a_, v_); };
        { let v_ = (((c.m.r16(((p_score_3).wrapping_add(660)).wrapping_add(6)) as i16)) as u8); let a_ = ((((c.m.r32(sym::gSaveData)).wrapping_add(8)).wrapping_add(644)).wrapping_add((((p_mission_1) as u32)).wrapping_mul(12))).wrapping_add(3); c.m.w8(a_, v_); };
        { let v_ = (((c.m.r16(((p_score_3).wrapping_add(660)).wrapping_add(8)) as i16)) as u8); let a_ = ((((c.m.r32(sym::gSaveData)).wrapping_add(8)).wrapping_add(644)).wrapping_add((((p_mission_1) as u32)).wrapping_mul(12))).wrapping_add(4); c.m.w8(a_, v_); };
    }
    break 'f l_record_4;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "UbScore_SaveMissionBest", &PROBE_N, &[], true); }
    r_
}

pub fn UbScore_SaveBestB(c: &mut Ctx, mut p_course_1: i32, mut p_total_2: i32, mut p_score_3: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "UbScore_SaveBestB", &PROBE_N, &[(p_course_1) as u32, (p_total_2) as u32, (p_score_3) as u32], false); }
    let mut l_record_4: i32 = 0;
    let r_ = 'f: {
    l_record_4 = 0i32;
    if ((((({ let a_ = ((((c.m.r32(sym::gSaveData)).wrapping_add(8)).wrapping_add(1844)).wrapping_add((((p_course_1) as u32)).wrapping_mul(12))).wrapping_add(4); (c.m.r32(a_) as i32) }) < (p_total_2)) as i32)) != 0) {
        { let v_ = p_total_2; let a_ = ((((c.m.r32(sym::gSaveData)).wrapping_add(8)).wrapping_add(1844)).wrapping_add((((p_course_1) as u32)).wrapping_mul(12))).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        l_record_4 = 1i32;
        { let v_ = (((c.m.r32((p_score_3).wrapping_add(676)) as i32)) as u8); let a_ = ((((c.m.r32(sym::gSaveData)).wrapping_add(8)).wrapping_add(1844)).wrapping_add((((p_course_1) as u32)).wrapping_mul(12))).wrapping_add(8); c.m.w8(a_, v_); };
        { let v_ = (((c.m.r16(((p_score_3).wrapping_add(660)).wrapping_add(4)) as i16)) as u8); let a_ = ((((c.m.r32(sym::gSaveData)).wrapping_add(8)).wrapping_add(1844)).wrapping_add((((p_course_1) as u32)).wrapping_mul(12))).wrapping_add(9); c.m.w8(a_, v_); };
        { let v_ = (((c.m.r16(((p_score_3).wrapping_add(660)).wrapping_add(6)) as i16)) as u8); let a_ = ((((c.m.r32(sym::gSaveData)).wrapping_add(8)).wrapping_add(1844)).wrapping_add((((p_course_1) as u32)).wrapping_mul(12))).wrapping_add(10); c.m.w8(a_, v_); };
        { let v_ = (((c.m.r16(((p_score_3).wrapping_add(660)).wrapping_add(8)) as i16)) as u8); let a_ = ((((c.m.r32(sym::gSaveData)).wrapping_add(8)).wrapping_add(1844)).wrapping_add((((p_course_1) as u32)).wrapping_mul(12))).wrapping_add(11); c.m.w8(a_, v_); };
        { let v_ = (c.m.r32(((p_score_3).wrapping_add(16)).wrapping_add((((3i32) as u32)).wrapping_mul(12))) as i32); let a_ = (((c.m.r32(sym::gSaveData)).wrapping_add(8)).wrapping_add(1844)).wrapping_add((((p_course_1) as u32)).wrapping_mul(12)); c.m.w32(a_, (v_) as u32); };
    }
    break 'f l_record_4;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "UbScore_SaveBestB", &PROBE_N, &[], true); }
    r_
}

pub fn UbScore_SaveBestC(c: &mut Ctx, mut p_course_1: i32, mut p_total_2: i32, mut p_score_3: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "UbScore_SaveBestC", &PROBE_N, &[(p_course_1) as u32, (p_total_2) as u32, (p_score_3) as u32], false); }
    let mut l_record_4: i32 = 0;
    let r_ = 'f: {
    l_record_4 = 0i32;
    if ((((({ let a_ = ((((c.m.r32(sym::gSaveData)).wrapping_add(8)).wrapping_add(1912)).wrapping_add((((p_course_1) as u32)).wrapping_mul(12))).wrapping_add(8); (c.m.r32(a_) as i32) }) < (p_total_2)) as i32)) != 0) {
        { let v_ = p_total_2; let a_ = ((((c.m.r32(sym::gSaveData)).wrapping_add(8)).wrapping_add(1912)).wrapping_add((((p_course_1) as u32)).wrapping_mul(12))).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        l_record_4 = 1i32;
        { let v_ = (((c.m.r32((p_score_3).wrapping_add(676)) as i32)) as u8); let a_ = ((((c.m.r32(sym::gSaveData)).wrapping_add(8)).wrapping_add(1912)).wrapping_add((((p_course_1) as u32)).wrapping_mul(12))).wrapping_add(1); c.m.w8(a_, v_); };
        { let v_ = (((c.m.r16(((p_score_3).wrapping_add(660)).wrapping_add(4)) as i16)) as u8); let a_ = ((((c.m.r32(sym::gSaveData)).wrapping_add(8)).wrapping_add(1912)).wrapping_add((((p_course_1) as u32)).wrapping_mul(12))).wrapping_add(2); c.m.w8(a_, v_); };
        { let v_ = (((c.m.r16(((p_score_3).wrapping_add(660)).wrapping_add(6)) as i16)) as u8); let a_ = ((((c.m.r32(sym::gSaveData)).wrapping_add(8)).wrapping_add(1912)).wrapping_add((((p_course_1) as u32)).wrapping_mul(12))).wrapping_add(3); c.m.w8(a_, v_); };
        { let v_ = (((c.m.r16(((p_score_3).wrapping_add(660)).wrapping_add(8)) as i16)) as u8); let a_ = ((((c.m.r32(sym::gSaveData)).wrapping_add(8)).wrapping_add(1912)).wrapping_add((((p_course_1) as u32)).wrapping_mul(12))).wrapping_add(4); c.m.w8(a_, v_); };
    }
    break 'f l_record_4;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "UbScore_SaveBestC", &PROBE_N, &[], true); }
    r_
}

pub fn UbScore_GetRewardItem(c: &mut Ctx, mut p_idx_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "UbScore_GetRewardItem", &PROBE_N, &[(p_idx_1) as u32], false); }
    let r_ = 'f: {
    break 'f (c.m.r32((sym::gUbRewardItems).wrapping_add((((p_idx_1) as u32)).wrapping_mul(4))) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "UbScore_GetRewardItem", &PROBE_N, &[], true); }
    r_
}
