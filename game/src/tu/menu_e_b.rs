#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn TeamSel_SumCost(c: &mut Ctx, mut p_side_1: i32, mut p_skipCur_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "TeamSel_SumCost", &PROBE_N, &[(p_side_1) as u32, (p_skipCur_2) as u32], false); }
    let mut l_i_3: i32 = 0;
    let r_ = 'f: {
    if ((((({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15420); (c.m.r32(a_) as i32) }) == (2i32)) as i32)) != 0) {
        { let v_ = 0i32; let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(304); c.m.w32(a_, (v_) as u32); };
        {
            l_i_3 = 0i32;
            'l1: loop {
                if !(((((l_i_3) < ({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(308); (c.m.r32(a_) as i32) })) as i32)) != 0) { break 'l1; }
                'c2: {
                    if (((((((((l_i_3) != ({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) })) as i32)) != 0) || ((((!((p_skipCur_2) != 0)) as i32)) != 0)) as i32)) != 0) {
                        { let r_ = { let a0_ = { let a_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add((((l_i_3) as u32)).wrapping_mul(48))).wrapping_add(28); (c.m.r32(a_) as i32) }; F::ChrTbl_GetCost(c, a0_) }; let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(304); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                    }
                }
                { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "TeamSel_SumCost", &PROBE_N, &[], true); }
    r_
}

pub fn TeamSel_IsCharaFree(c: &mut Ctx, mut p_side_1: i32, mut p_chara_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "TeamSel_IsCharaFree", &PROBE_N, &[(p_side_1) as u32, (p_chara_2) as u32], false); }
    let mut l_ok_3: i32 = 0;
    let mut l_i_4: i32 = 0;
    let r_ = 'f: {
    l_ok_3 = 1i32;
    {
        l_i_4 = 0i32;
        'l1: loop {
            if !(((((l_i_4) < ({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(308); (c.m.r32(a_) as i32) })) as i32)) != 0) { break 'l1; }
            'c2: {
                if (((((l_i_4) != ({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                    if (({ let a1_ = { let a_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add((((l_i_4) as u32)).wrapping_mul(48))).wrapping_add(28); (c.m.r32(a_) as i32) }; F::ChrTbl_IsRelated(c, p_chara_2, a1_) }) != 0) {
                        l_ok_3 = 0i32;
                    }
                }
            }
            { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
        }
    }
    break 'f l_ok_3;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "TeamSel_IsCharaFree", &PROBE_N, &[], true); }
    r_
}

pub fn TeamSel_FitsDp(c: &mut Ctx, mut p_side_1: i32, mut p_chara_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "TeamSel_FitsDp", &PROBE_N, &[(p_side_1) as u32, (p_chara_2) as u32], false); }
    let mut l_w_3: u32 = 0;
    let r_ = 'f: {
    l_w_3 = c.m.r32(sym::gTeamSel);
    if ((((((c.m.r32((l_w_3).wrapping_add(15420)) as i32)) != (2i32)) as i32)) != 0) {
        break 'f 1i32;
    }
    if (((((((((p_chara_2) == (161i32)) as i32)) != 0) || (((((p_chara_2) == (163i32)) as i32)) != 0)) as i32)) != 0) {
        break 'f 1i32;
    }
    break 'f (((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(304); (c.m.r32(a_) as i32) }).wrapping_add(F::ChrTbl_GetCost(c, p_chara_2))) <= ((c.m.r32((l_w_3).wrapping_add(15428)) as i32))) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "TeamSel_FitsDp", &PROBE_N, &[], true); }
    r_
}

pub fn TeamSel_RemoveMember(c: &mut Ctx, mut p_side_1: i32, mut p_idx_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "TeamSel_RemoveMember", &PROBE_N, &[(p_side_1) as u32, (p_idx_2) as u32], false); }
    let mut l_i_3: i32 = 0;
    let r_ = 'f: {
    { let _ = { let a0_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add((((p_idx_2) as u32)).wrapping_mul(48)); F::memset(c, a0_, 0i32, 48u32) }; };
    { let v_ = (1i32).wrapping_neg(); let a_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add((((p_idx_2) as u32)).wrapping_mul(48))).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
    if (((((p_idx_2) < ({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(308); (c.m.r32(a_) as i32) })) as i32)) != 0) {
        {
            l_i_3 = p_idx_2;
            'l1: loop {
                if !(((((l_i_3) < (({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(308); (c.m.r32(a_) as i32) }).wrapping_sub(1i32))) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let s_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((((l_i_3).wrapping_add(1i32)) as u32)).wrapping_mul(48)); let d_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add((((l_i_3) as u32)).wrapping_mul(48)); rt::memcpy(c, d_, s_, 48); };
                }
                { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
            }
        }
        { let _ = { let a0_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add((((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(308); (c.m.r32(a_) as i32) }).wrapping_sub(1i32)) as u32)).wrapping_mul(48)); F::memset(c, a0_, 0i32, 48u32) }; };
        { let v_ = (1i32).wrapping_neg(); let a_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add((((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(308); (c.m.r32(a_) as i32) }).wrapping_sub(1i32)) as u32)).wrapping_mul(48))).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "TeamSel_RemoveMember", &PROBE_N, &[], true); }
    r_
}

pub fn TeamSel_SetStageChips(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "TeamSel_SetStageChips", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_i_1: i32 = 0;
    let mut l_id_2: i32 = 0;
    let mut l_res_3: u32 = 0;
    let mut l_icon_4: i32 = 0;
    let r_ = 'f: {
        rt::memset(c, fp, 0, 24);
        { let v_ = 8i32; c.m.w32(fp.wrapping_add(0), (v_) as u32); }
        { let v_ = 11i32; c.m.w32(fp.wrapping_add(4), (v_) as u32); }
        { let v_ = 12i32; c.m.w32(fp.wrapping_add(8), (v_) as u32); }
        { let v_ = 13i32; c.m.w32(fp.wrapping_add(12), (v_) as u32); }
        { let v_ = 14i32; c.m.w32(fp.wrapping_add(16), (v_) as u32); }
        { let v_ = 15i32; c.m.w32(fp.wrapping_add(20), (v_) as u32); }
        {
            l_i_1 = 0i32;
            'l1: loop {
                if !(((((l_i_1) < (6i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_id_2 = { let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15452); c.m.r32(a_) }).wrapping_add(((((({ let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(2468); c.m.r32(a_) }).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(6i32)).wrapping_add(l_i_1)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
                    { let v_ = { let a_ = ((({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(2468); c.m.r32(a_) }).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ((({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(2468); c.m.r32(a_) }).wrapping_add(8)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    { let v_ = l_id_2; let a_ = ((({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(2468); c.m.r32(a_) }).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    {
                        l_icon_4 = (if (((((l_id_2) == (62i32)) as i32)) != 0) { 37i32 } else { (if (((((l_id_2) == (63i32)) as i32)) != 0) { 38i32 } else { (l_id_2).wrapping_add(1i32) }) });
                        l_res_3 = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(48); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(48); c.m.r32(a_) }).wrapping_add((((l_icon_4) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32));
                    }
                    { let v_ = { let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(368)).wrapping_add(((((c.m.r32((fp).wrapping_add((((l_i_1) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) }; let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(368)).wrapping_add(((((41i32).wrapping_add(l_i_1)) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = c.m.r32((l_res_3).wrapping_add(16)); let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(368)).wrapping_add(((((c.m.r32((fp).wrapping_add((((l_i_1) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                }
                { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "TeamSel_SetStageChips", &PROBE_N, &[], true); }
    r_
}

pub fn TeamSel_SetChips(c: &mut Ctx, mut p_side_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "TeamSel_SetChips", &PROBE_N, &[(p_side_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_i_2: i32 = 0;
    let mut l_id_3: i32 = 0;
    let mut l_res_4: u32 = 0;
    let r_ = 'f: {
        rt::memset(c, fp, 0, 28);
        { let v_ = 8i32; c.m.w32(fp.wrapping_add(0), (v_) as u32); }
        { let v_ = 11i32; c.m.w32(fp.wrapping_add(4), (v_) as u32); }
        { let v_ = 12i32; c.m.w32(fp.wrapping_add(8), (v_) as u32); }
        { let v_ = 13i32; c.m.w32(fp.wrapping_add(12), (v_) as u32); }
        { let v_ = 14i32; c.m.w32(fp.wrapping_add(16), (v_) as u32); }
        { let v_ = 15i32; c.m.w32(fp.wrapping_add(20), (v_) as u32); }
        { let v_ = 16i32; c.m.w32(fp.wrapping_add(24), (v_) as u32); }
        {
            l_i_2 = 0i32;
            'l1: loop {
                if !(((((l_i_2) < (7i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_id_3 = { let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2480)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((((({ let a_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(7i32)).wrapping_add(l_i_2)) as u32)).wrapping_mul(36)); (c.m.r32(a_) as i32) };
                    { let v_ = { let a_ = ((({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(240)).wrapping_add((((0i32) as u32)).wrapping_mul(28))).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ((({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(240)).wrapping_add((((1i32) as u32)).wrapping_mul(28))).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    { let v_ = l_id_3; let a_ = ((({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(240)).wrapping_add((((0i32) as u32)).wrapping_mul(28))).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    l_res_4 = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(36); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(36); c.m.r32(a_) }).wrapping_add(((((l_id_3).wrapping_add(1i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32));
                    { let v_ = { let a_ = (((c.m.r32(sym::gTeamSel)).wrapping_add(652)).wrapping_add((((p_side_1) as u32)).wrapping_mul(100))).wrapping_add(((((c.m.r32((fp).wrapping_add((((l_i_2) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) }; let a_ = (((c.m.r32(sym::gTeamSel)).wrapping_add(652)).wrapping_add((((p_side_1) as u32)).wrapping_mul(100))).wrapping_add(((((18i32).wrapping_add(l_i_2)) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = c.m.r32((l_res_4).wrapping_add(16)); let a_ = (((c.m.r32(sym::gTeamSel)).wrapping_add(652)).wrapping_add((((p_side_1) as u32)).wrapping_mul(100))).wrapping_add(((((c.m.r32((fp).wrapping_add((((l_i_2) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "TeamSel_SetChips", &PROBE_N, &[], true); }
    r_
}

pub fn TeamSel_SetGridFormChips(c: &mut Ctx, mut p_side_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "TeamSel_SetGridFormChips", &PROBE_N, &[(p_side_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_i_2: i32 = 0;
    let mut l_id_3: i32 = 0;
    let mut l_res_4: u32 = 0;
    let r_ = 'f: {
        rt::memset(c, fp, 0, 28);
        { let v_ = 8i32; c.m.w32(fp.wrapping_add(0), (v_) as u32); }
        { let v_ = 11i32; c.m.w32(fp.wrapping_add(4), (v_) as u32); }
        { let v_ = 12i32; c.m.w32(fp.wrapping_add(8), (v_) as u32); }
        { let v_ = 13i32; c.m.w32(fp.wrapping_add(12), (v_) as u32); }
        { let v_ = 14i32; c.m.w32(fp.wrapping_add(16), (v_) as u32); }
        { let v_ = 15i32; c.m.w32(fp.wrapping_add(20), (v_) as u32); }
        { let v_ = 16i32; c.m.w32(fp.wrapping_add(24), (v_) as u32); }
        {
            l_i_2 = 0i32;
            'l1: loop {
                if !(((((l_i_2) < (7i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    if (((((l_i_2) < ({ let a_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2480)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add((((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48)); (c.m.r32(a_) as i32) }).wrapping_add(({ let a_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(7i32))) as u32)).wrapping_mul(36))).wrapping_add(4); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                        l_id_3 = { let a_ = ((({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2480)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add((((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48)); (c.m.r32(a_) as i32) }).wrapping_add(({ let a_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(7i32))) as u32)).wrapping_mul(36))).wrapping_add(8)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
                    } else {
                        l_id_3 = 164i32;
                    }
                    { let v_ = { let a_ = ((({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(240)).wrapping_add((((0i32) as u32)).wrapping_mul(28))).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ((({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(240)).wrapping_add((((1i32) as u32)).wrapping_mul(28))).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    { let v_ = l_id_3; let a_ = ((({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(240)).wrapping_add((((0i32) as u32)).wrapping_mul(28))).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    l_res_4 = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(36); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(36); c.m.r32(a_) }).wrapping_add(((((l_id_3).wrapping_add(1i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32));
                    { let v_ = { let a_ = (((c.m.r32(sym::gTeamSel)).wrapping_add(652)).wrapping_add((((p_side_1) as u32)).wrapping_mul(100))).wrapping_add(((((c.m.r32((fp).wrapping_add((((l_i_2) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) }; let a_ = (((c.m.r32(sym::gTeamSel)).wrapping_add(652)).wrapping_add((((p_side_1) as u32)).wrapping_mul(100))).wrapping_add(((((18i32).wrapping_add(l_i_2)) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = c.m.r32((l_res_4).wrapping_add(16)); let a_ = (((c.m.r32(sym::gTeamSel)).wrapping_add(652)).wrapping_add((((p_side_1) as u32)).wrapping_mul(100))).wrapping_add(((((c.m.r32((fp).wrapping_add((((l_i_2) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "TeamSel_SetGridFormChips", &PROBE_N, &[], true); }
    r_
}

pub fn TeamSel_SetCustomChips(c: &mut Ctx, mut p_side_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "TeamSel_SetCustomChips", &PROBE_N, &[(p_side_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_i_2: i32 = 0;
    let mut l_id_3: i32 = 0;
    let mut l_res_4: u32 = 0;
    let r_ = 'f: {
        rt::memset(c, fp, 0, 28);
        { let v_ = 8i32; c.m.w32(fp.wrapping_add(0), (v_) as u32); }
        { let v_ = 11i32; c.m.w32(fp.wrapping_add(4), (v_) as u32); }
        { let v_ = 12i32; c.m.w32(fp.wrapping_add(8), (v_) as u32); }
        { let v_ = 13i32; c.m.w32(fp.wrapping_add(12), (v_) as u32); }
        { let v_ = 14i32; c.m.w32(fp.wrapping_add(16), (v_) as u32); }
        { let v_ = 15i32; c.m.w32(fp.wrapping_add(20), (v_) as u32); }
        { let v_ = 16i32; c.m.w32(fp.wrapping_add(24), (v_) as u32); }
        {
            l_i_2 = 0i32;
            'l1: loop {
                if !(((((l_i_2) < (7i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_id_3 = { let a_ = (((c.m.r32(sym::gTeamSel)).wrapping_add(14384)).wrapping_add((((p_side_1) as u32)).wrapping_mul(504))).wrapping_add(((((l_i_2).wrapping_add(({ let a_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(16); (c.m.r32(a_) as i32) }).wrapping_mul(7i32))) as u32)).wrapping_mul(36)); (c.m.r32(a_) as i32) };
                    { let v_ = { let a_ = ((({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(240)).wrapping_add((((0i32) as u32)).wrapping_mul(28))).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ((({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(240)).wrapping_add((((1i32) as u32)).wrapping_mul(28))).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    { let v_ = l_id_3; let a_ = ((({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(240)).wrapping_add((((0i32) as u32)).wrapping_mul(28))).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    l_res_4 = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(36); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(36); c.m.r32(a_) }).wrapping_add(((((l_id_3).wrapping_add(1i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32));
                    { let v_ = { let a_ = (((c.m.r32(sym::gTeamSel)).wrapping_add(652)).wrapping_add((((p_side_1) as u32)).wrapping_mul(100))).wrapping_add(((((c.m.r32((fp).wrapping_add((((l_i_2) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) }; let a_ = (((c.m.r32(sym::gTeamSel)).wrapping_add(652)).wrapping_add((((p_side_1) as u32)).wrapping_mul(100))).wrapping_add(((((18i32).wrapping_add(l_i_2)) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = c.m.r32((l_res_4).wrapping_add(16)); let a_ = (((c.m.r32(sym::gTeamSel)).wrapping_add(652)).wrapping_add((((p_side_1) as u32)).wrapping_mul(100))).wrapping_add(((((c.m.r32((fp).wrapping_add((((l_i_2) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "TeamSel_SetCustomChips", &PROBE_N, &[], true); }
    r_
}

pub fn TeamSel_SetFormChips(c: &mut Ctx, mut p_side_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "TeamSel_SetFormChips", &PROBE_N, &[(p_side_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_i_2: i32 = 0;
    let mut l_id_3: i32 = 0;
    let mut l_res_4: u32 = 0;
    let r_ = 'f: {
        rt::memset(c, fp, 0, 28);
        { let v_ = 8i32; c.m.w32(fp.wrapping_add(0), (v_) as u32); }
        { let v_ = 11i32; c.m.w32(fp.wrapping_add(4), (v_) as u32); }
        { let v_ = 12i32; c.m.w32(fp.wrapping_add(8), (v_) as u32); }
        { let v_ = 13i32; c.m.w32(fp.wrapping_add(12), (v_) as u32); }
        { let v_ = 14i32; c.m.w32(fp.wrapping_add(16), (v_) as u32); }
        { let v_ = 15i32; c.m.w32(fp.wrapping_add(20), (v_) as u32); }
        { let v_ = 16i32; c.m.w32(fp.wrapping_add(24), (v_) as u32); }
        {
            l_i_2 = 0i32;
            'l1: loop {
                if !(((((l_i_2) < (7i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    if (((((l_i_2) < ({ let a_ = ((((c.m.r32(sym::gTeamSel)).wrapping_add(14384)).wrapping_add((((p_side_1) as u32)).wrapping_mul(504))).wrapping_add((((({ let a_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(12); (c.m.r32(a_) as i32) }).wrapping_add(({ let a_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(16); (c.m.r32(a_) as i32) }).wrapping_mul(7i32))) as u32)).wrapping_mul(36))).wrapping_add(4); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                        l_id_3 = { let a_ = (((((c.m.r32(sym::gTeamSel)).wrapping_add(14384)).wrapping_add((((p_side_1) as u32)).wrapping_mul(504))).wrapping_add((((({ let a_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(12); (c.m.r32(a_) as i32) }).wrapping_add(({ let a_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(16); (c.m.r32(a_) as i32) }).wrapping_mul(7i32))) as u32)).wrapping_mul(36))).wrapping_add(8)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
                    } else {
                        l_id_3 = 164i32;
                    }
                    { let v_ = { let a_ = ((({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(240)).wrapping_add((((0i32) as u32)).wrapping_mul(28))).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ((({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(240)).wrapping_add((((1i32) as u32)).wrapping_mul(28))).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    { let v_ = l_id_3; let a_ = ((({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(240)).wrapping_add((((0i32) as u32)).wrapping_mul(28))).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    l_res_4 = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(36); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(36); c.m.r32(a_) }).wrapping_add(((((l_id_3).wrapping_add(1i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32));
                    { let v_ = { let a_ = (((c.m.r32(sym::gTeamSel)).wrapping_add(652)).wrapping_add((((p_side_1) as u32)).wrapping_mul(100))).wrapping_add(((((c.m.r32((fp).wrapping_add((((l_i_2) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) }; let a_ = (((c.m.r32(sym::gTeamSel)).wrapping_add(652)).wrapping_add((((p_side_1) as u32)).wrapping_mul(100))).wrapping_add(((((18i32).wrapping_add(l_i_2)) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = c.m.r32((l_res_4).wrapping_add(16)); let a_ = (((c.m.r32(sym::gTeamSel)).wrapping_add(652)).wrapping_add((((p_side_1) as u32)).wrapping_mul(100))).wrapping_add(((((c.m.r32((fp).wrapping_add((((l_i_2) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "TeamSel_SetFormChips", &PROBE_N, &[], true); }
    r_
}

pub fn TeamSel_SetTeamTex(c: &mut Ctx, mut p_side_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "TeamSel_SetTeamTex", &PROBE_N, &[(p_side_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_i_2: i32 = 0;
    let mut l_id_3: i32 = 0;
    let mut l_res_4: u32 = 0;
    let r_ = 'f: {
        rt::memset(c, fp, 0, 20);
        { let v_ = 1i32; c.m.w32(fp.wrapping_add(0), (v_) as u32); }
        { let v_ = 4i32; c.m.w32(fp.wrapping_add(4), (v_) as u32); }
        { let v_ = 5i32; c.m.w32(fp.wrapping_add(8), (v_) as u32); }
        { let v_ = 6i32; c.m.w32(fp.wrapping_add(12), (v_) as u32); }
        { let v_ = 7i32; c.m.w32(fp.wrapping_add(16), (v_) as u32); }
        if ((((({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15420); (c.m.r32(a_) as i32) }) == (2i32)) as i32)) != 0) {
            { let v_ = 0i32; let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(304); c.m.w32(a_, (v_) as u32); };
        }
        {
            l_i_2 = 0i32;
            'l1: loop {
                if !(((((l_i_2) < (5i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let v_ = ((0i32) as u32); let a_ = (((c.m.r32(sym::gTeamSel)).wrapping_add(852)).wrapping_add((((p_side_1) as u32)).wrapping_mul(52))).wrapping_add(((((c.m.r32((fp).wrapping_add((((l_i_2) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    if (((((l_i_2) < ({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(308); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                        l_id_3 = { let a_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add((((l_i_2) as u32)).wrapping_mul(48))).wrapping_add(28); (c.m.r32(a_) as i32) };
                        l_res_4 = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(36); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(36); c.m.r32(a_) }).wrapping_add(((((l_id_3).wrapping_add(1i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32));
                        { let v_ = c.m.r32((l_res_4).wrapping_add(16)); let a_ = (((c.m.r32(sym::gTeamSel)).wrapping_add(852)).wrapping_add((((p_side_1) as u32)).wrapping_mul(52))).wrapping_add(((((c.m.r32((fp).wrapping_add((((l_i_2) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                        if ((((({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15420); (c.m.r32(a_) as i32) }) == (2i32)) as i32)) != 0) {
                            { let r_ = F::ChrTbl_GetCost(c, l_id_3); let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(304); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                        }
                    }
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "TeamSel_SetTeamTex", &PROBE_N, &[], true); }
    r_
}

pub fn TeamSel_UpdateFaceLoad(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "TeamSel_UpdateFaceLoad", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        's1: {
            let sv1: i32 = { let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(960); (c.m.r32(a_) as i32) };
            match sv1 {
                5i32 => {
                    if (((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(968); (c.m.r32(a_) as i32) }) ^ (1i32))) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(296); (c.m.r32(a_) as i32) }) & (1i32))) != 0) {
                        { let r_ = 1i32; let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(968); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
                    }
                    { let v_ = 6i32; let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(960); c.m.w32(a_, (v_) as u32); };
                    break 's1;
                }
                6i32 => {
                    if (((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(968); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(296); (c.m.r32(a_) as i32) }) & (1i32))) != 0) {
                        { let r_ = 1i32; let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(968); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(296); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
                    }
                    { let v_ = 1i32; let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(960); c.m.w32(a_, (v_) as u32); };
                    break 's1;
                }
                1i32 => {
                    F::File_CancelRequests(c);
                    if ((((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(968); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(312); (c.m.r32(a_) as i32) }) < (0i32)) as i32)) != 0) {
                        { let r_ = 1024i32; let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(968); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(296); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                        { let r_ = 2i32; let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(968); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(296); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                        if (((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(968); (c.m.r32(a_) as i32) }) ^ (1i32))) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(296); (c.m.r32(a_) as i32) }) & (1i32))) != 0) {
                            { let r_ = 1i32; let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(968); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
                            { let v_ = 6i32; let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(960); c.m.w32(a_, (v_) as u32); };
                        } else {
                            { let v_ = 4i32; let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(960); c.m.w32(a_, (v_) as u32); };
                        }
                    } else {
                        { let _ = { let a0_ = ({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(968); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(312); (c.m.r32(a_) as i32) }).wrapping_add(761i32); let a1_ = { let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(8)).wrapping_add(((({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(968); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.r32(a_) }; let _ = 92160i32; F::File_Request(c, a0_, a1_) }; };
                        { let v_ = 2i32; let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(960); c.m.w32(a_, (v_) as u32); };
                    }
                    break 's1;
                }
                2i32 => {
                    if ((F::File_UpdateRequests(c)) != 0) {
                        { let v_ = 3i32; let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(960); c.m.w32(a_, (v_) as u32); };
                    }
                    break 's1;
                }
                3i32 => {
                    { let _ = { let a0_ = { let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(8)).wrapping_add(((({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(968); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.r32(a_) }; let a1_ = { let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(16)).wrapping_add(((({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(968); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.r32(a_) }; F::Sprite_Unpack(c, a0_, a1_, ((0i32) as u32)) }; };
                    { let v_ = { let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(16)).wrapping_add(((({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(968); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.r32(a_) }; let a_ = fp; c.m.w32(a_, v_); };
                    { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
                    if ((((({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(968); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
                        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(368)).wrapping_add((((23i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    } else {
                        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(368)).wrapping_add((((22i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    }
                    { let r_ = 2i32; let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(968); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(296); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    if (((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(968); (c.m.r32(a_) as i32) }) ^ (1i32))) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(296); (c.m.r32(a_) as i32) }) & (1i32))) != 0) {
                        { let r_ = 1i32; let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(968); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
                        { let v_ = 6i32; let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(960); c.m.w32(a_, (v_) as u32); };
                    } else {
                        { let v_ = 4i32; let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(960); c.m.w32(a_, (v_) as u32); };
                    }
                    break 's1;
                }
                4i32 => {
                    if ((((({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(964); (c.m.r32(a_) as i32) }) != (4i32)) as i32)) != 0) {
                        break 's1;
                    }
                    if (((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(296); (c.m.r32(a_) as i32) }) & (1i32))) != 0) {
                        { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(368)).wrapping_add((((23i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                        { let v_ = 0i32; let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(968); c.m.w32(a_, (v_) as u32); };
                        { let v_ = 6i32; let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(960); c.m.w32(a_, (v_) as u32); };
                    } else {
                        if (((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(296); (c.m.r32(a_) as i32) }) & (1i32))) != 0) {
                            { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(368)).wrapping_add((((22i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                            { let v_ = 1i32; let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(968); c.m.w32(a_, (v_) as u32); };
                            { let v_ = 6i32; let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(960); c.m.w32(a_, (v_) as u32); };
                        }
                    }
                    break 's1;
                }
                _ => {
                }
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "TeamSel_UpdateFaceLoad", &PROBE_N, &[], true); }
    r_
}

pub fn TeamSel_RequestFace(c: &mut Ctx, mut p_side_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "TeamSel_RequestFace", &PROBE_N, &[(p_side_1) as u32], false); }
    let r_ = 'f: {
    if (((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(296); (c.m.r32(a_) as i32) }) & (2i32))) != 0) {
        { let r_ = 2i32; let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(296); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
    } else {
        if ((((({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(968); (c.m.r32(a_) as i32) }) == (p_side_1)) as i32)) != 0) {
            { let v_ = 5i32; let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(960); c.m.w32(a_, (v_) as u32); };
        }
    }
    if (((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(296); (c.m.r32(a_) as i32) }) & (1024i32))) != 0) {
        { let r_ = 1024i32; let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(296); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
    }
    { let r_ = 1i32; let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(296); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "TeamSel_RequestFace", &PROBE_N, &[], true); }
    r_
}

pub fn TeamSel_UpdateStageLoad(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "TeamSel_UpdateStageLoad", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        's1: {
            let sv1: i32 = { let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(964); (c.m.r32(a_) as i32) };
            match sv1 {
                5i32 => {
                    { let v_ = 6i32; let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(964); c.m.w32(a_, (v_) as u32); };
                    break 's1;
                }
                6i32 => {
                    if (((({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(956); (c.m.r32(a_) as i32) }) & (32i32))) != 0) {
                        { let r_ = 32i32; let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(956); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
                    }
                    { let v_ = 1i32; let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(964); c.m.w32(a_, (v_) as u32); };
                    break 's1;
                }
                1i32 => {
                    F::File_CancelRequests(c);
                    { let _ = { let a0_ = ({ let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(2468); c.m.r32(a_) }).wrapping_add(60); (c.m.r32(a_) as i32) }).wrapping_add(925i32); let a1_ = { let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(24); c.m.r32(a_) }; let _ = 243712i32; F::File_Request(c, a0_, a1_) }; };
                    { let v_ = 2i32; let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(964); c.m.w32(a_, (v_) as u32); };
                    break 's1;
                }
                2i32 => {
                    if ((F::File_UpdateRequests(c)) != 0) {
                        { let v_ = 3i32; let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(964); c.m.w32(a_, (v_) as u32); };
                    }
                    break 's1;
                }
                3i32 => {
                    { let _ = { let a0_ = { let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(24); c.m.r32(a_) }; let a1_ = { let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(28)).wrapping_add(((({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(972); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.r32(a_) }; F::Sprite_Unpack(c, a0_, a1_, ((0i32) as u32)) }; };
                    { let v_ = { let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(28)).wrapping_add(((({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(972); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.r32(a_) }; let a_ = fp; c.m.w32(a_, v_); };
                    { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
                    { let v_ = c.m.r32(fp); let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(360)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let r_ = 1i32; let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(972); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
                    { let r_ = 64i32; let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(956); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    { let v_ = 4i32; let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(964); c.m.w32(a_, (v_) as u32); };
                    break 's1;
                }
                4i32 => {
                    if ((((({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(960); (c.m.r32(a_) as i32) }) != (4i32)) as i32)) != 0) {
                        break 's1;
                    }
                    if (((({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(956); (c.m.r32(a_) as i32) }) & (32i32))) != 0) {
                        { let v_ = 6i32; let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(964); c.m.w32(a_, (v_) as u32); };
                    }
                    break 's1;
                }
                _ => {
                }
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "TeamSel_UpdateStageLoad", &PROBE_N, &[], true); }
    r_
}

pub fn TeamSel_RequestStage(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "TeamSel_RequestStage", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if (((({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(956); (c.m.r32(a_) as i32) }) & (64i32))) != 0) {
        { let r_ = 64i32; let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(956); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
    } else {
        { let v_ = 5i32; let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(964); c.m.w32(a_, (v_) as u32); };
    }
    { let r_ = 32i32; let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(956); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    { let v_ = { let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(28)).wrapping_add(((({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(972); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.r32(a_) }; let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(360)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = { let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(28)).wrapping_add(((((({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(972); (c.m.r32(a_) as i32) }) ^ (1i32))) as u32)).wrapping_mul(4)); c.m.r32(a_) }; let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(360)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "TeamSel_RequestStage", &PROBE_N, &[], true); }
    r_
}

pub fn TeamSel_ClipGoto(c: &mut Ctx, mut p_flash_1: i32, mut p_side_2: i32, mut p_kind_3: i32, mut p_label_4: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "TeamSel_ClipGoto", &PROBE_N, &[(p_flash_1) as u32, (p_side_2) as u32, (p_kind_3) as u32, (p_label_4) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(80) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 80);
    let mut l_f_5: u32 = 0;
    let r_ = 'f: {
        l_f_5 = ((c.m.r32(sym::gTeamSel)).wrapping_add(52)).wrapping_add((((p_flash_1) as u32)).wrapping_mul(44));
        's1: {
            let sv6: i32 = p_kind_3;
            match sv6 {
                0i32 => {
                    { let _ = { let a2_ = { let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((p_side_2) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((p_side_2) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48)); (c.m.r32(a_) as i32) }; F::sprintf(c, fp.wrapping_add(8), sym::menu_e_b__str0, &[((a2_) as i64 as u64)]) }; };
                    break 's1;
                }
                1i32 => {
                    { let _ = { let a2_ = { let a_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((p_side_2) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((p_side_2) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(8); (c.m.r32(a_) as i32) }; F::sprintf(c, fp.wrapping_add(8), sym::menu_e_b__str1, &[((a2_) as i64 as u64)]) }; };
                    break 's1;
                }
                2i32 => {
                    { let _ = { let a2_ = ({ let a_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((p_side_2) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((p_side_2) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(20); (c.m.r32(a_) as i32) }).wrapping_add(1i32); F::sprintf(c, fp.wrapping_add(8), sym::menu_e_b__str2, &[((a2_) as i64 as u64)]) }; };
                    break 's1;
                }
                5i32 => {
                    { let _ = { let a2_ = ({ let a_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((p_side_2) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((p_side_2) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(24); (c.m.r32(a_) as i32) }).wrapping_add(1i32); F::sprintf(c, fp.wrapping_add(8), sym::menu_e_b__str3, &[((a2_) as i64 as u64)]) }; };
                    break 's1;
                }
                6i32 => {
                    's2: {
                        let sv7: i32 = { let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((p_side_2) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) };
                        match sv7 {
                            5i32 => {
                                { let _ = F::sprintf(c, fp.wrapping_add(8), sym::menu_e_b__str4, &[]); };
                                break 's2;
                            }
                            _ => {
                                { let _ = { let a2_ = { let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((p_side_2) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }; F::sprintf(c, fp.wrapping_add(8), sym::menu_e_b__str5, &[((a2_) as i64 as u64)]) }; };
                                break 's2;
                            }
                        }
                    }
                    break 's1;
                }
                7i32 => {
                    { let _ = { let a2_ = { let a_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((p_side_2) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((p_side_2) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(12); (c.m.r32(a_) as i32) }; F::sprintf(c, fp.wrapping_add(8), sym::menu_e_b__str6, &[((a2_) as i64 as u64)]) }; };
                    break 's1;
                }
                9i32 => {
                    if ((((({ let a_ = { let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(2468); c.m.r32(a_) }; (c.m.r32(a_) as i32) }) == (6i32)) as i32)) != 0) {
                        { let _ = F::sprintf(c, fp.wrapping_add(8), sym::menu_e_b__str7, &[]); };
                    } else {
                        { let _ = { let a2_ = { let a_ = { let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(2468); c.m.r32(a_) }; (c.m.r32(a_) as i32) }; F::sprintf(c, fp.wrapping_add(8), sym::menu_e_b__str8, &[((a2_) as i64 as u64)]) }; };
                    }
                    break 's1;
                }
                3i32 | 10i32 => {
                    break 'f;
                }
                _ => {
                    break 's1;
                }
            }
        }
        F::Flash_FindLabel(c, l_f_5, ((0i32) as u32), fp.wrapping_add(8), fp);
        F::Flash_ClipGotoLabel(c, l_f_5, fp, p_label_4);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "TeamSel_ClipGoto", &PROBE_N, &[], true); }
    r_
}

pub fn TeamSel_Init(c: &mut Ctx, mut p_section_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "TeamSel_Init", &PROBE_N, &[(p_section_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_i_2: i32 = 0;
    let mut l_j_3: i32 = 0;
    let mut l_panel_4: u32 = 0;
    let mut l_cols_5: i32 = 0;
    let mut l_cols_6: i32 = 0;
    let mut l_rows_7: i32 = 0;
    let r_ = 'f: {
        { let v_ = ((0i32) as u32); c.m.w32(fp, v_); }
        { let v_ = F::Heap_Alloc(c, ((16036u32) as i32), ((32i32) as u32), 0i32, 2i32); let a_ = sym::gTeamSel; c.m.w32(a_, v_); };
        { let _ = { let a0_ = c.m.r32(sym::gTeamSel); F::memset(c, a0_, 0i32, 16036u32) }; };
        { let v_ = (c.m.r32(sym::gMenuArc5)).wrapping_add((({ let a_ = (c.m.r32(sym::gMenuArc5)).wrapping_add((((p_section_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = c.m.r32(sym::gTeamSel); c.m.w32(a_, v_); };
        { let v_ = { let a0_ = { let a_ = c.m.r32(sym::gTeamSel); c.m.r32(a_) }; F::Sprite_Unpack(c, a0_, ((0i32) as u32), ((0i32) as u32)) }; let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((47i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(368)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(368)).wrapping_add((((17i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(368)).wrapping_add((((16i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(368)).wrapping_add((((18i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(368)).wrapping_add((((20i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(368)).wrapping_add((((19i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(368)).wrapping_add((((9i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(368)).wrapping_add((((40i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        if ((((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1568); (c.m.r32(a_) as i32) }) == (1i32)) as i32)) != 0) {
            { let v_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((50i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
            { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
            { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(368)).wrapping_add((((10i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        }
        { let v_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(368)).wrapping_add((((39i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(368)).wrapping_add((((38i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((8i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(368)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(368)).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((4i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(368)).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(368)).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((9i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(368)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(368)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((10i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(368)).wrapping_add((((21i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((11i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(368)).wrapping_add((((24i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(368)).wrapping_add((((26i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(368)).wrapping_add((((25i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(368)).wrapping_add((((27i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((12i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(368)).wrapping_add((((47i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(368)).wrapping_add((((48i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        if ((((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1572); (c.m.r32(a_) as i32) }) == (2i32)) as i32)) != 0) {
            { let v_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((13i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
            { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
            { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(368)).wrapping_add((((28i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
            { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(368)).wrapping_add((((31i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
            { let v_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((14i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
            { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
            { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(368)).wrapping_add((((29i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
            { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(368)).wrapping_add((((30i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        }
        { let a0_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(52)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); let a1_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((15i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a2_ = (c.m.r32(sym::gTeamSel)).wrapping_add(368); F::Flash_Create(c, a0_, a1_, a2_) };
        { let a0_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(52)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_Play(c, a0_, 1i32) };
        if ((((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1572); (c.m.r32(a_) as i32) }) == (2i32)) as i32)) != 0) {
            { let v_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((16i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
            { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
            { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = (((c.m.r32(sym::gTeamSel)).wrapping_add(652)).wrapping_add((((0i32) as u32)).wrapping_mul(100))).wrapping_add((((9i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
            { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = (((c.m.r32(sym::gTeamSel)).wrapping_add(652)).wrapping_add((((1i32) as u32)).wrapping_mul(100))).wrapping_add((((9i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
            { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = (((c.m.r32(sym::gTeamSel)).wrapping_add(852)).wrapping_add((((0i32) as u32)).wrapping_mul(52))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
            { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = (((c.m.r32(sym::gTeamSel)).wrapping_add(852)).wrapping_add((((1i32) as u32)).wrapping_mul(52))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        }
        { let v_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((17i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = (((c.m.r32(sym::gTeamSel)).wrapping_add(652)).wrapping_add((((0i32) as u32)).wrapping_mul(100))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = (((c.m.r32(sym::gTeamSel)).wrapping_add(652)).wrapping_add((((0i32) as u32)).wrapping_mul(100))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((18i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = (((c.m.r32(sym::gTeamSel)).wrapping_add(652)).wrapping_add((((1i32) as u32)).wrapping_mul(100))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = (((c.m.r32(sym::gTeamSel)).wrapping_add(652)).wrapping_add((((1i32) as u32)).wrapping_mul(100))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((19i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = (((c.m.r32(sym::gTeamSel)).wrapping_add(652)).wrapping_add((((0i32) as u32)).wrapping_mul(100))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = (((c.m.r32(sym::gTeamSel)).wrapping_add(652)).wrapping_add((((0i32) as u32)).wrapping_mul(100))).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((4i32).wrapping_mul(64i32)) as u32)); let a_ = (((c.m.r32(sym::gTeamSel)).wrapping_add(652)).wrapping_add((((0i32) as u32)).wrapping_mul(100))).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = (((c.m.r32(sym::gTeamSel)).wrapping_add(652)).wrapping_add((((0i32) as u32)).wrapping_mul(100))).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((20i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = (((c.m.r32(sym::gTeamSel)).wrapping_add(652)).wrapping_add((((1i32) as u32)).wrapping_mul(100))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = (((c.m.r32(sym::gTeamSel)).wrapping_add(652)).wrapping_add((((1i32) as u32)).wrapping_mul(100))).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((4i32).wrapping_mul(64i32)) as u32)); let a_ = (((c.m.r32(sym::gTeamSel)).wrapping_add(652)).wrapping_add((((1i32) as u32)).wrapping_mul(100))).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = (((c.m.r32(sym::gTeamSel)).wrapping_add(652)).wrapping_add((((1i32) as u32)).wrapping_mul(100))).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((21i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = (((c.m.r32(sym::gTeamSel)).wrapping_add(652)).wrapping_add((((0i32) as u32)).wrapping_mul(100))).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = (((c.m.r32(sym::gTeamSel)).wrapping_add(652)).wrapping_add((((1i32) as u32)).wrapping_mul(100))).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(368)).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((22i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = (((c.m.r32(sym::gTeamSel)).wrapping_add(652)).wrapping_add((((0i32) as u32)).wrapping_mul(100))).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = (((c.m.r32(sym::gTeamSel)).wrapping_add(652)).wrapping_add((((1i32) as u32)).wrapping_mul(100))).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((23i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = (((c.m.r32(sym::gTeamSel)).wrapping_add(652)).wrapping_add((((0i32) as u32)).wrapping_mul(100))).wrapping_add((((10i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = (((c.m.r32(sym::gTeamSel)).wrapping_add(652)).wrapping_add((((0i32) as u32)).wrapping_mul(100))).wrapping_add((((17i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = (((c.m.r32(sym::gTeamSel)).wrapping_add(652)).wrapping_add((((1i32) as u32)).wrapping_mul(100))).wrapping_add((((10i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = (((c.m.r32(sym::gTeamSel)).wrapping_add(652)).wrapping_add((((1i32) as u32)).wrapping_mul(100))).wrapping_add((((17i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = (((c.m.r32(sym::gTeamSel)).wrapping_add(564)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = (((c.m.r32(sym::gTeamSel)).wrapping_add(564)).wrapping_add((((1i32) as u32)).wrapping_mul(44))).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = (((c.m.r32(sym::gTeamSel)).wrapping_add(852)).wrapping_add((((0i32) as u32)).wrapping_mul(52))).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = (((c.m.r32(sym::gTeamSel)).wrapping_add(852)).wrapping_add((((1i32) as u32)).wrapping_mul(52))).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let a0_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(52)).wrapping_add((((3i32) as u32)).wrapping_mul(44)); let a1_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((24i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a2_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(652)).wrapping_add((((0i32) as u32)).wrapping_mul(100)); F::Flash_Create(c, a0_, a1_, a2_) };
        { let a0_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(52)).wrapping_add((((3i32) as u32)).wrapping_mul(44)); F::Flash_Play(c, a0_, 1i32) };
        { let a0_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(52)).wrapping_add((((4i32) as u32)).wrapping_mul(44)); let a1_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((25i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a2_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(652)).wrapping_add((((1i32) as u32)).wrapping_mul(100)); F::Flash_Create(c, a0_, a1_, a2_) };
        { let a0_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(52)).wrapping_add((((4i32) as u32)).wrapping_mul(44)); F::Flash_Play(c, a0_, 1i32) };
        { let v_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((48i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = (((c.m.r32(sym::gTeamSel)).wrapping_add(564)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add((((9i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = (((c.m.r32(sym::gTeamSel)).wrapping_add(564)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((49i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = (((c.m.r32(sym::gTeamSel)).wrapping_add(564)).wrapping_add((((1i32) as u32)).wrapping_mul(44))).wrapping_add((((9i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = (((c.m.r32(sym::gTeamSel)).wrapping_add(564)).wrapping_add((((1i32) as u32)).wrapping_mul(44))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((26i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = (((c.m.r32(sym::gTeamSel)).wrapping_add(564)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = (((c.m.r32(sym::gTeamSel)).wrapping_add(564)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((27i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = (((c.m.r32(sym::gTeamSel)).wrapping_add(564)).wrapping_add((((1i32) as u32)).wrapping_mul(44))).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = (((c.m.r32(sym::gTeamSel)).wrapping_add(564)).wrapping_add((((1i32) as u32)).wrapping_mul(44))).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((28i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = (((c.m.r32(sym::gTeamSel)).wrapping_add(564)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = (((c.m.r32(sym::gTeamSel)).wrapping_add(564)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = (((c.m.r32(sym::gTeamSel)).wrapping_add(564)).wrapping_add((((1i32) as u32)).wrapping_mul(44))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = (((c.m.r32(sym::gTeamSel)).wrapping_add(564)).wrapping_add((((1i32) as u32)).wrapping_mul(44))).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((29i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = (((c.m.r32(sym::gTeamSel)).wrapping_add(564)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = (((c.m.r32(sym::gTeamSel)).wrapping_add(564)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add((((8i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = (((c.m.r32(sym::gTeamSel)).wrapping_add(564)).wrapping_add((((1i32) as u32)).wrapping_mul(44))).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = (((c.m.r32(sym::gTeamSel)).wrapping_add(564)).wrapping_add((((1i32) as u32)).wrapping_mul(44))).wrapping_add((((8i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((30i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = (((c.m.r32(sym::gTeamSel)).wrapping_add(564)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add((((10i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = (((c.m.r32(sym::gTeamSel)).wrapping_add(564)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((31i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = (((c.m.r32(sym::gTeamSel)).wrapping_add(564)).wrapping_add((((1i32) as u32)).wrapping_mul(44))).wrapping_add((((10i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = (((c.m.r32(sym::gTeamSel)).wrapping_add(564)).wrapping_add((((1i32) as u32)).wrapping_mul(44))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let a0_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(52)).wrapping_add((((5i32) as u32)).wrapping_mul(44)); let a1_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((32i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a2_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(564)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_Create(c, a0_, a1_, a2_) };
        { let a0_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(52)).wrapping_add((((5i32) as u32)).wrapping_mul(44)); F::Flash_Play(c, a0_, 1i32) };
        { let a0_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(52)).wrapping_add((((6i32) as u32)).wrapping_mul(44)); let a1_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((33i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a2_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(564)).wrapping_add((((1i32) as u32)).wrapping_mul(44)); F::Flash_Create(c, a0_, a1_, a2_) };
        { let a0_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(52)).wrapping_add((((6i32) as u32)).wrapping_mul(44)); F::Flash_Play(c, a0_, 1i32) };
        { let v_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((34i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = (((c.m.r32(sym::gTeamSel)).wrapping_add(852)).wrapping_add((((0i32) as u32)).wrapping_mul(52))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = (((c.m.r32(sym::gTeamSel)).wrapping_add(852)).wrapping_add((((1i32) as u32)).wrapping_mul(52))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((35i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = (((c.m.r32(sym::gTeamSel)).wrapping_add(852)).wrapping_add((((0i32) as u32)).wrapping_mul(52))).wrapping_add((((8i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = (((c.m.r32(sym::gTeamSel)).wrapping_add(852)).wrapping_add((((0i32) as u32)).wrapping_mul(52))).wrapping_add((((11i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = (((c.m.r32(sym::gTeamSel)).wrapping_add(852)).wrapping_add((((0i32) as u32)).wrapping_mul(52))).wrapping_add((((10i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = (((c.m.r32(sym::gTeamSel)).wrapping_add(852)).wrapping_add((((1i32) as u32)).wrapping_mul(52))).wrapping_add((((8i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = (((c.m.r32(sym::gTeamSel)).wrapping_add(852)).wrapping_add((((1i32) as u32)).wrapping_mul(52))).wrapping_add((((11i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = (((c.m.r32(sym::gTeamSel)).wrapping_add(852)).wrapping_add((((1i32) as u32)).wrapping_mul(52))).wrapping_add((((10i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((36i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = (((c.m.r32(sym::gTeamSel)).wrapping_add(852)).wrapping_add((((0i32) as u32)).wrapping_mul(52))).wrapping_add((((9i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = (((c.m.r32(sym::gTeamSel)).wrapping_add(852)).wrapping_add((((0i32) as u32)).wrapping_mul(52))).wrapping_add((((12i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = (((c.m.r32(sym::gTeamSel)).wrapping_add(852)).wrapping_add((((1i32) as u32)).wrapping_mul(52))).wrapping_add((((9i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = (((c.m.r32(sym::gTeamSel)).wrapping_add(852)).wrapping_add((((1i32) as u32)).wrapping_mul(52))).wrapping_add((((12i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let a0_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(52)).wrapping_add((((1i32) as u32)).wrapping_mul(44)); let a1_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((37i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a2_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(852)).wrapping_add((((0i32) as u32)).wrapping_mul(52)); F::Flash_Create(c, a0_, a1_, a2_) };
        { let a0_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(52)).wrapping_add((((1i32) as u32)).wrapping_mul(44)); F::Flash_Play(c, a0_, 1i32) };
        { let a0_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(52)).wrapping_add((((2i32) as u32)).wrapping_mul(44)); let a1_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((38i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a2_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(852)).wrapping_add((((1i32) as u32)).wrapping_mul(52)); F::Flash_Create(c, a0_, a1_, a2_) };
        { let a0_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(52)).wrapping_add((((2i32) as u32)).wrapping_mul(44)); F::Flash_Play(c, a0_, 1i32) };
        {
            l_panel_4 = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((53i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32));
            F::ItemPanel_Init(c, l_panel_4, 0i32);
            F::ItemPanel_Init(c, l_panel_4, 1i32);
        }
        { let a0_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((51i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); F::ItemHelp_Init(c, a0_) };
        { let v_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((55i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let a0_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((54i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a1_ = c.m.r32(fp); F::IconWin_Init(c, a0_, a1_) };
        { let v_ = ({ let a_ = ((c.m.r32(sym::gCommonRes)).wrapping_add(4)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = ((c.m.r32(sym::gCommonRes)).wrapping_add(4)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(16032); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((43i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(40); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((44i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(44); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((45i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(36); c.m.w32(a_, v_); };
        {
            l_i_2 = 0i32;
            'l1: loop {
                if !(((((l_i_2) < (165i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let v_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(36); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(36); c.m.r32(a_) }).wrapping_add(((((l_i_2).wrapping_add(1i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
                    { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        { let v_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((46i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(48); c.m.w32(a_, v_); };
        {
            l_i_2 = 0i32;
            'l3: loop {
                if !(((((l_i_2) < (38i32)) as i32)) != 0) { break 'l3; }
                'c4: {
                    { let v_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(48); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(48); c.m.r32(a_) }).wrapping_add(((((l_i_2).wrapping_add(1i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
                    { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        { let v_ = (({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((39i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32))).wrapping_add(((16i32) as u32)); let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15452); c.m.w32(a_, v_); };
        { let v_ = { let a_ = ((({ let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((39i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.r32(a_) }); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15460); c.m.w32(a_, (v_) as u32); };
        { let a0_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15460); let a1_ = { let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15452); c.m.r32(a_) }; F::StgGrid_ApplyUnlocks(c, a0_, a1_) };
        { let v_ = (({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((56i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32))).wrapping_add(((16i32) as u32)); let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15456); c.m.w32(a_, v_); };
        { let v_ = { let a_ = ((({ let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((56i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.r32(a_) }); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15464); c.m.w32(a_, (v_) as u32); };
        { let a0_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15464); let a1_ = { let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15456); c.m.r32(a_) }; F::BgmList_ApplyUnlocks(c, a0_, a1_) };
        { let r_ = 4i32; let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15464); let n_ = ((c.m.r32(a_) as i32)).wrapping_sub(r_); c.m.w32(a_, (n_) as u32); };
        { let v_ = 24i32; let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15456); c.m.r32(a_) }).wrapping_add((((({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15464); (c.m.r32(a_) as i32) }).wrapping_sub(1i32)) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = (({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((42i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32))).wrapping_add(((16i32) as u32)); let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2480)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = { let a_ = ((({ let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((42i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.r32(a_) }); (c.m.r32(a_) as i32) }; let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2472)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = { let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2480)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }; let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2480)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = { let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2472)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2472)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        {
            l_i_2 = 0i32;
            'l5: loop {
                if !(((((l_i_2) < (2i32)) as i32)) != 0) { break 'l5; }
                'c6: {
                    l_cols_5 = 7i32;
                    { let a0_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2488)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); let a1_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2496)).wrapping_add((((l_i_2) as u32)).wrapping_mul(5940)); let a2_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2472)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); let a3_ = { let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2480)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); c.m.r32(a_) }; let a4_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(14376)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); let a5_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(14384)).wrapping_add((((l_i_2) as u32)).wrapping_mul(504)); F::ChrGrid_Build(c, a0_, a1_, a2_, a3_, a4_, a5_) };
                    { let v_ = { let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2488)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2472)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    { let v_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2496)).wrapping_add((((l_i_2) as u32)).wrapping_mul(5940)); let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2480)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = rt::div_i32({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2472)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }, l_cols_5); let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(15392)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    if ((rt::rem_i32({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2472)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }, l_cols_5)) != 0) {
                        { let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(15392)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    }
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        { let v_ = F::Heap_Alloc(c, 92160i32, ((64i32) as u32), 0i32, 2i32); let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = F::Heap_Alloc(c, 92160i32, ((64i32) as u32), 0i32, 2i32); let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(8)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = F::Heap_Alloc(c, 133120i32, ((32i32) as u32), 0i32, 2i32); let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = F::Heap_Alloc(c, 133120i32, ((32i32) as u32), 0i32, 2i32); let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = F::Heap_Alloc(c, 243712i32, ((64i32) as u32), 0i32, 2i32); let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(24); c.m.w32(a_, v_); };
        { let v_ = F::Heap_Alloc(c, 274432i32, ((32i32) as u32), 0i32, 2i32); let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(28)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = F::Heap_Alloc(c, 274432i32, ((32i32) as u32), 0i32, 2i32); let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(28)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(984)).wrapping_add((((0i32) as u32)).wrapping_mul(648)); let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(984)).wrapping_add((((1i32) as u32)).wrapping_mul(648)); let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = (c.m.r32(sym::gTeamSel)).wrapping_add(2384); let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(2468); c.m.w32(a_, v_); };
        { let s_ = ((c.m.r32(sym::gProgress)).wrapping_add(1088)).wrapping_add((((0i32) as u32)).wrapping_mul(240)); let d_ = { let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }; rt::memcpy(c, d_, s_, 240); };
        { let s_ = ((c.m.r32(sym::gProgress)).wrapping_add(1088)).wrapping_add((((1i32) as u32)).wrapping_mul(240)); let d_ = { let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }; rt::memcpy(c, d_, s_, 240); };
        { let v_ = { let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1568); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15416); c.m.w32(a_, (v_) as u32); };
        { let v_ = { let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1572); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15420); c.m.w32(a_, (v_) as u32); };
        { let v_ = { let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1584); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15424); c.m.w32(a_, (v_) as u32); };
        {
            l_cols_6 = 6i32;
            l_rows_7 = 6i32;
            { let v_ = rt::rem_i32({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1576); (c.m.r32(a_) as i32) }, l_cols_6); let a_ = { let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(2468); c.m.r32(a_) }; c.m.w32(a_, (v_) as u32); };
            { let v_ = rt::div_i32({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1576); (c.m.r32(a_) as i32) }, l_rows_7); let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(2468); c.m.r32(a_) }).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        }
        {
            l_i_2 = 0i32;
            'l7: loop {
                if !(((((l_i_2) < ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15464); (c.m.r32(a_) as i32) })) as i32)) != 0) { break 'l7; }
                'c8: {
                    if ((((({ let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15456); c.m.r32(a_) }).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) == ({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1580); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                        { let v_ = l_i_2; let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(2468); c.m.r32(a_) }).wrapping_add(76); c.m.w32(a_, (v_) as u32); };
                    }
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        { let v_ = 4i32; let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(960); c.m.w32(a_, (v_) as u32); };
        if ((((({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15420); (c.m.r32(a_) as i32) }) == (2i32)) as i32)) != 0) {
            's9: {
                let sv8: i32 = { let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15424); (c.m.r32(a_) as i32) };
                match sv8 {
                    0i32 => {
                        { let v_ = 10i32; let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15428); c.m.w32(a_, (v_) as u32); };
                        break 's9;
                    }
                    1i32 => {
                        { let v_ = 15i32; let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15428); c.m.w32(a_, (v_) as u32); };
                        break 's9;
                    }
                    2i32 => {
                        { let v_ = 20i32; let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15428); c.m.w32(a_, (v_) as u32); };
                        break 's9;
                    }
                    _ => {
                    }
                }
            }
        }
        {
            l_i_2 = 0i32;
            'l10: loop {
                if !(((((l_i_2) < (2i32)) as i32)) != 0) { break 'l10; }
                'c11: {
                    {
                        l_j_3 = 0i32;
                        'l12: loop {
                            if !(((((l_j_3) < (5i32)) as i32)) != 0) { break 'l12; }
                            'c13: {
                                if ((((!(({ let a0_ = { let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2480)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); c.m.r32(a_) }; let a1_ = ({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add((((l_j_3) as u32)).wrapping_mul(48)); (c.m.r32(a_) as i32) }).wrapping_add(({ let a_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add((((l_j_3) as u32)).wrapping_mul(48))).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(7i32)); F::ChrGrid_IsSelectable(c, a0_, a1_) }) != 0)) as i32)) != 0) {
                                    { let _ = { let a0_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add((((l_j_3) as u32)).wrapping_mul(48)); F::memset(c, a0_, 0i32, 48u32) }; };
                                    { let v_ = l_i_2; let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add((((l_j_3) as u32)).wrapping_mul(48)); c.m.w32(a_, (v_) as u32); };
                                    { let s_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add((((l_j_3) as u32)).wrapping_mul(48)); let d_ = (((c.m.r32(sym::gProgress)).wrapping_add(1088)).wrapping_add((((l_i_2) as u32)).wrapping_mul(240))).wrapping_add((((l_j_3) as u32)).wrapping_mul(48)); rt::memcpy(c, d_, s_, 48); };
                                }
                            }
                            { let o_ = l_j_3; let n_ = (o_).wrapping_add(1); l_j_3 = n_; };
                        }
                    }
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        if ((((!(({ let a0_ = { let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15452); c.m.r32(a_) }; let a1_ = { let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1576); (c.m.r32(a_) as i32) }; F::StgGrid_IsSelectable(c, a0_, a1_) }) != 0)) as i32)) != 0) {
            { let v_ = 0i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1576); c.m.w32(a_, (v_) as u32); };
            { let v_ = 0i32; let a_ = { let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(2468); c.m.r32(a_) }; c.m.w32(a_, (v_) as u32); };
            { let v_ = 0i32; let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(2468); c.m.r32(a_) }).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        }
        F::TeamSel_SetChips(c, 0i32);
        F::TeamSel_SetChips(c, 1i32);
        F::TeamSel_SetStageChips(c);
        { let v_ = { let a_ = ((({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(240)).wrapping_add((((0i32) as u32)).wrapping_mul(28))).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(312); c.m.w32(a_, (v_) as u32); };
        { let v_ = { let a_ = ((({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(240)).wrapping_add((((0i32) as u32)).wrapping_mul(28))).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(312); c.m.w32(a_, (v_) as u32); };
        { let v_ = { let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15452); c.m.r32(a_) }).wrapping_add(((((({ let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(2468); c.m.r32(a_) }).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(6i32)).wrapping_add({ let a_ = { let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(2468); c.m.r32(a_) }; (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(2468); c.m.r32(a_) }).wrapping_add(60); c.m.w32(a_, (v_) as u32); };
        { let v_ = { let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(2468); c.m.r32(a_) }).wrapping_add(76); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(2468); c.m.r32(a_) }).wrapping_add(80); c.m.w32(a_, (v_) as u32); };
        {
            l_i_2 = 0i32;
            'l14: loop {
                if !(((((l_i_2) < (2i32)) as i32)) != 0) { break 'l14; }
                'c15: {
                    {
                        l_j_3 = 0i32;
                        'l16: loop {
                            if !(((((l_j_3) < (5i32)) as i32)) != 0) { break 'l16; }
                            'c17: {
                                { let v_ = (1i32).wrapping_neg(); let a_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add((((l_j_3) as u32)).wrapping_mul(48))).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                            }
                            { let o_ = l_j_3; let n_ = (o_).wrapping_add(1); l_j_3 = n_; };
                        }
                    }
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        {
            l_i_2 = 0i32;
            'l18: loop {
                if !(((((l_i_2) < (2i32)) as i32)) != 0) { break 'l18; }
                'c19: {
                    { let _ = { let a0_ = ({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(312); (c.m.r32(a_) as i32) }).wrapping_add(761i32); let a1_ = { let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(8)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); c.m.r32(a_) }; F::File_LoadSync(c, a0_, a1_, 92160i32) }; };
                    { let _ = { let a0_ = { let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(8)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); c.m.r32(a_) }; let a1_ = { let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(16)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); c.m.r32(a_) }; F::Sprite_Unpack(c, a0_, a1_, ((0i32) as u32)) }; };
                    { let v_ = { let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(16)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); c.m.r32(a_) }; let a_ = fp; c.m.w32(a_, v_); };
                    { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(368)).wrapping_add(((((if ((l_i_2) != 0) { 22i32 } else { 23i32 })) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let r_ = 2i32; let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(296); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        { let _ = { let a0_ = ({ let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(2468); c.m.r32(a_) }).wrapping_add(60); (c.m.r32(a_) as i32) }).wrapping_add(925i32); let a1_ = { let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(24); c.m.r32(a_) }; F::File_LoadSync(c, a0_, a1_, 243712i32) }; };
        { let _ = { let a0_ = { let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(24); c.m.r32(a_) }; let a1_ = { let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(28)).wrapping_add(((({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(972); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.r32(a_) }; F::Sprite_Unpack(c, a0_, a1_, ((0i32) as u32)) }; };
        { let v_ = { let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(28)).wrapping_add(((({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(972); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.r32(a_) }; let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = c.m.r32(fp); let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(360)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = c.m.r32(fp); let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(360)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = 4i32; let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(964); c.m.w32(a_, (v_) as u32); };
        { let r_ = 64i32; let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(956); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        { let v_ = 128i32; let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(15432)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(15432)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = 1i32; let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(972); c.m.w32(a_, (v_) as u32); };
        if ((((({ let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15456); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(2468); c.m.r32(a_) }).wrapping_add(80); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) == (24i32)) as i32)) != 0) {
            { let a0_ = (((F::Rand_Range(c, ((9i32) as u32))).wrapping_add(((68382i32) as u32))) as i32); F::Bgm_Play(c, a0_) };
        } else {
            { let a0_ = ({ let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15456); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(2468); c.m.r32(a_) }).wrapping_add(80); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(68374i32); F::Bgm_Play(c, a0_) };
        }
        {
            l_i_2 = 0i32;
            'l20: loop {
                if !(((((l_i_2) < (2i32)) as i32)) != 0) { break 'l20; }
                'c21: {
                    { let a0_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(15472)).wrapping_add((((l_i_2) as u32)).wrapping_mul(140)); let a1_ = { let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(40); c.m.r32(a_) }; F::TextBox_Init(c, a0_, a1_, (((l_i_2).wrapping_add(1i32)) as u32)) };
                    { let a0_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(15472)).wrapping_add((((l_i_2) as u32)).wrapping_mul(140)); F::TextBox_SetUnk80(c, a0_, 1i32) };
                    { let a0_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(15752)).wrapping_add((((l_i_2) as u32)).wrapping_mul(140)); let a1_ = { let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(44); c.m.r32(a_) }; F::TextBox_Init(c, a0_, a1_, (((l_i_2).wrapping_add(3i32)) as u32)) };
                    { let a0_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(15752)).wrapping_add((((l_i_2) as u32)).wrapping_mul(140)); F::TextBox_SetUnk80(c, a0_, 1i32) };
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "TeamSel_Init", &PROBE_N, &[], true); }
    r_
}

pub fn TeamSel_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "TeamSel_Term", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let r_ = 'f: {
    F::IconWin_Term(c);
    F::ItemHelp_Term(c);
    F::ItemPanel_Term(c, 1i32);
    F::ItemPanel_Term(c, 0i32);
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (7i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let a0_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(52)).wrapping_add((((l_i_1) as u32)).wrapping_mul(44)); F::Flash_Destroy(c, a0_) };
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
    if ((((({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(28)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = { let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(28)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(28)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    }
    if ((((({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(28)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = { let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(28)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(28)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    }
    if ((((({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(24); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = { let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(24); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(24); c.m.w32(a_, v_); };
    }
    if ((((({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = { let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    }
    if ((((({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = { let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    }
    if ((((({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(8)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = { let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(8)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(8)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    }
    if ((((({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = { let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    }
    if ((((({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = { let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(4); c.m.w32(a_, v_); };
    }
    if (((((c.m.r32(sym::gTeamSel)) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32(sym::gTeamSel); F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = sym::gTeamSel; c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "TeamSel_Term", &PROBE_N, &[], true); }
    r_
}

pub fn TeamSel_Draw(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "TeamSel_Draw", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(160) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 160);
    let mut l_i_1: i32 = 0;
    let mut l_j_2: i32 = 0;
    let mut l_flash_3: u32 = 0;
    let mut l_chara_4: i32 = 0;
    let mut l_cost_5: i32 = 0;
    let mut l_n_6: u32 = 0;
    let mut l_cols_7: i32 = 0;
    let r_ = 'f: {
        {
            l_i_1 = 0i32;
            'l1: loop {
                if !(((((l_i_1) < (2i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    if (((({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(956); (c.m.r32(a_) as i32) }) & (64i32))) != 0) {
                        's3: {
                            let sv8: i32 = l_i_1;
                            match sv8 {
                                0i32 => {
                                    { let r_ = 5i32; let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(15432)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                                    if ((((({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(15432)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) >= (128i32)) as i32)) != 0) {
                                        { let v_ = 128i32; let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(15432)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                    }
                                    break 's3;
                                }
                                1i32 => {
                                    { let r_ = 5i32; let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(15432)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); let n_ = ((c.m.r32(a_) as i32)).wrapping_sub(r_); c.m.w32(a_, (n_) as u32); };
                                    if ((((({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(15432)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) < (0i32)) as i32)) != 0) {
                                        { let v_ = 0i32; let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(15432)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                    }
                                    break 's3;
                                }
                                _ => {
                                }
                            }
                        }
                    } else {
                        if (((((l_i_1) == (0i32)) as i32)) != 0) {
                            { let v_ = 0i32; let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(15432)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                            break 'c2;
                        } else {
                            if (((((l_i_1) == (1i32)) as i32)) != 0) {
                                { let v_ = 128i32; let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(15432)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                            }
                        }
                    }
                    { let a0_ = { let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(360)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }; let a3_ = { let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(15432)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; F::Sprite_DrawPicture(c, a0_, 0i32, 0i32, a3_) };
                }
                { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
            }
        }
        l_flash_3 = ((c.m.r32(sym::gTeamSel)).wrapping_add(52)).wrapping_add((((0i32) as u32)).wrapping_mul(44));
        {
            l_i_1 = 0i32;
            'l4: loop {
                if !(((((l_i_1) < (2i32)) as i32)) != 0) { break 'l4; }
                'c5: {
                    F::Flash_FindLabel(c, l_flash_3, ((0i32) as u32), (if ((l_i_1) != 0) { sym::menu_e_b__str9 } else { sym::menu_e_b__str10 }), fp);
                    if ((((({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15468); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
                        { let _ = F::Flash_ClipSetFlags(c, l_flash_3, fp, 258i32, ((((1i32) as i64 as u64)) as u8)); };
                    } else {
                        { let _ = F::Flash_ClipSetFlags(c, l_flash_3, fp, 258i32, ((((0i32) as i64 as u64)) as u8)); };
                    }
                    if ((((((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(296); (c.m.r32(a_) as i32) }) & (((1024i32) | (2i32))))) == (2i32)) as i32)) != 0) {
                        { let r_ = F32(0x3d999999); let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(15440)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                        if (((rt::fge({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(15440)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }, F32(0x3f800000)) as i32)) != 0) {
                            { let v_ = F32(0x3f800000); let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(15440)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                        }
                    } else {
                        { let v_ = F32(0x00000000); let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(15440)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    }
                    F::Flash_FindLabel(c, l_flash_3, ((0i32) as u32), (if ((l_i_1) != 0) { sym::menu_e_b__str11 } else { sym::menu_e_b__str12 }), fp);
                    { let a2_ = { let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(15440)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }; F::Flash_ClipSetAlpha(c, l_flash_3, fp, a2_) };
                    { let a3_ = (((({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15468); (c.m.r32(a_) as i32) }) as u8)) as i32); { let _ = F::Flash_ClipSetFlags(c, l_flash_3, fp, 128i32, ((((a3_) as i64 as u64)) as u8)); } };
                    F::Flash_FindLabel(c, l_flash_3, ((0i32) as u32), (if ((l_i_1) != 0) { sym::menu_e_b__str13 } else { sym::menu_e_b__str14 }), fp);
                    { let a4_ = { let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(312); (c.m.r32(a_) as i32) }; let a5_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(15472)).wrapping_add((((l_i_1) as u32)).wrapping_mul(140)); F::TextBox_AttachLine(c, l_flash_3, fp, 0i32, 0i32, a4_, a5_) };
                    F::Flash_FindLabel(c, l_flash_3, ((0i32) as u32), (if ((l_i_1) != 0) { sym::menu_e_b__str15 } else { sym::menu_e_b__str16 }), fp);
                    { let a4_ = { let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(312); (c.m.r32(a_) as i32) }; let a5_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(15752)).wrapping_add((((l_i_1) as u32)).wrapping_mul(140)); F::TextBox_AttachLine(c, l_flash_3, fp, 0i32, 0i32, a4_, a5_) };
                }
                { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
            }
        }
        { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 512i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = (rt::rem_i32({ let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(2468); c.m.r32(a_) }).wrapping_add(60); (c.m.r32(a_) as i32) }, 4i32)).wrapping_mul(64i32); let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = ((c.m.r32((fp.wrapping_add(8)).wrapping_add(4)) as i32)).wrapping_add(64i32); let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        { let v_ = rt::div_i32({ let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(2468); c.m.r32(a_) }).wrapping_add(60); (c.m.r32(a_) as i32) }, 4i32); let a_ = (fp.wrapping_add(8)).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
        F::Flash_FindLabel(c, l_flash_3, ((0i32) as u32), sym::menu_e_b__str17, fp);
        F::Flash_ClipSetUv(c, l_flash_3, fp, fp.wrapping_add(8));
        { let a2_ = (c.m.r32((fp.wrapping_add(8)).wrapping_add(16)) as i32); F::Flash_ClipSetTex(c, l_flash_3, fp, a2_) };
        if ((((({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15420); (c.m.r32(a_) as i32) }) == (2i32)) as i32)) != 0) {
            {
                l_i_1 = 0i32;
                'l6: loop {
                    if !(((((l_i_1) < (2i32)) as i32)) != 0) { break 'l6; }
                    'c7: {
                        { let v_ = (rt::rem_i32({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15424); (c.m.r32(a_) as i32) }, 2i32)).wrapping_mul(64i32); let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                        { let v_ = (rt::div_i32({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15424); (c.m.r32(a_) as i32) }, 2i32)).wrapping_mul(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                        { let v_ = ((c.m.r32(fp.wrapping_add(8)) as i32)).wrapping_add(64i32); let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                        { let v_ = ((c.m.r32((fp.wrapping_add(8)).wrapping_add(4)) as i32)).wrapping_add(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                        F::Flash_FindLabel(c, l_flash_3, ((0i32) as u32), (if ((l_i_1) != 0) { sym::menu_e_b__str18 } else { sym::menu_e_b__str19 }), fp);
                        F::Flash_ClipSetUv(c, l_flash_3, fp, fp.wrapping_add(8));
                        { let a4_ = { let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(304); (c.m.r32(a_) as i32) }; F::Num_Draw(c, l_flash_3, (if ((l_i_1) != 0) { sym::menu_e_b__str20 } else { sym::menu_e_b__str21 }), 0i32, 2i32, a4_, 32i32, 64i32, 1i32) };
                    }
                    { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
                }
            }
        }
        F::Flash_FindLabel(c, l_flash_3, ((0i32) as u32), sym::menu_e_b__str22, fp);
        if ((((({ let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(2468); c.m.r32(a_) }).wrapping_add(68); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
            { let _ = F::Flash_ClipSetFlags(c, l_flash_3, fp, 258i32, ((((1i32) as i64 as u64)) as u8)); };
        } else {
            { let _ = F::Flash_ClipSetFlags(c, l_flash_3, fp, 258i32, ((((0i32) as i64 as u64)) as u8)); };
        }
        {
            l_i_1 = 0i32;
            'l8: loop {
                if !(((((l_i_1) < (12i32)) as i32)) != 0) { break 'l8; }
                'c9: {
                    { let _ = F::sprintf(c, fp.wrapping_add(28), sym::menu_e_b__str23, &[((l_i_1) as i64 as u64)]); };
                    F::Flash_FindLabel(c, l_flash_3, ((0i32) as u32), fp.wrapping_add(28), fp);
                    { let a3_ = (((({ let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(2468); c.m.r32(a_) }).wrapping_add(68); (c.m.r32(a_) as i32) }) as u8)) as i32); { let _ = F::Flash_ClipSetFlags(c, l_flash_3, fp, 128i32, ((((a3_) as i64 as u64)) as u8)); } };
                }
                { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
            }
        }
        {
            l_i_1 = 0i32;
            'l10: loop {
                if !(((((l_i_1) < (2i32)) as i32)) != 0) { break 'l10; }
                'c11: {
                    { let v_ = (l_i_1).wrapping_mul(32i32); let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = ((c.m.r32(fp.wrapping_add(8)) as i32)).wrapping_add(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 32i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 64i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                    F::Flash_FindLabel(c, l_flash_3, (if ((l_i_1) != 0) { sym::menu_e_b__str24 } else { sym::menu_e_b__str25 }), (if ((l_i_1) != 0) { sym::menu_e_b__str26 } else { sym::menu_e_b__str27 }), fp);
                    F::Flash_ClipSetUv(c, l_flash_3, fp, fp.wrapping_add(8));
                }
                { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
            }
        }
        { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 512i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = (rt::rem_i32({ let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15456); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(2468); c.m.r32(a_) }).wrapping_add(76); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }, 8i32)).wrapping_mul(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = ((c.m.r32((fp.wrapping_add(8)).wrapping_add(4)) as i32)).wrapping_add(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        { let v_ = rt::div_i32({ let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15456); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(2468); c.m.r32(a_) }).wrapping_add(76); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }, 8i32); let a_ = (fp.wrapping_add(8)).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
        F::Flash_FindLabel(c, l_flash_3, sym::menu_e_b__str28, sym::menu_e_b__str29, fp);
        F::Flash_ClipSetUv(c, l_flash_3, fp, fp.wrapping_add(8));
        { let a2_ = (c.m.r32((fp.wrapping_add(8)).wrapping_add(16)) as i32); F::Flash_ClipSetTex(c, l_flash_3, fp, a2_) };
        F::Flash_FindLabel(c, l_flash_3, sym::menu_e_b__str30, sym::menu_e_b__str31, fp);
        F::Flash_ClipSetUv(c, l_flash_3, fp, fp.wrapping_add(8));
        { let a2_ = (c.m.r32((fp.wrapping_add(8)).wrapping_add(16)) as i32); F::Flash_ClipSetTex(c, l_flash_3, fp, a2_) };
        {
            l_i_1 = 0i32;
            'l12: loop {
                if !(((((l_i_1) < (2i32)) as i32)) != 0) { break 'l12; }
                'c13: {
                    l_flash_3 = ((c.m.r32(sym::gTeamSel)).wrapping_add(52)).wrapping_add(((((5i32).wrapping_add(l_i_1)) as u32)).wrapping_mul(44));
                    { let v_ = (((l_i_1) ^ (1i32))).wrapping_mul(32i32); let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = ((c.m.r32(fp.wrapping_add(8)) as i32)).wrapping_add(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 32i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                    F::Flash_FindLabel(c, l_flash_3, ((0i32) as u32), sym::menu_e_b__str32, fp);
                    F::Flash_ClipSetUv(c, l_flash_3, fp, fp.wrapping_add(8));
                    {
                        l_j_2 = 0i32;
                        'l14: loop {
                            if !(((((l_j_2) < (4i32)) as i32)) != 0) { break 'l14; }
                            'c15: {
                                { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                                { let v_ = 256i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                                { let v_ = (l_j_2).wrapping_mul(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                                { let v_ = ((c.m.r32((fp.wrapping_add(8)).wrapping_add(4)) as i32)).wrapping_add(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                                { let _ = F::sprintf(c, fp.wrapping_add(28), sym::menu_e_b__str33, &[(((l_j_2).wrapping_add(1i32)) as i64 as u64)]); };
                                F::Flash_FindLabel(c, l_flash_3, ((0i32) as u32), fp.wrapping_add(28), fp);
                                { let a2_ = (if (((((l_j_2) < ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(15400)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) })) as i32)) != 0) { F32(0x3f800000) } else { F32(0x3e999999) }); F::Flash_ClipSetColor(c, l_flash_3, fp, a2_) };
                                F::Flash_FindLabel(c, l_flash_3, fp.wrapping_add(28), sym::menu_e_b__str34, fp);
                                F::Flash_ClipSetUv(c, l_flash_3, fp, fp.wrapping_add(8));
                                F::Flash_FindLabel(c, l_flash_3, fp.wrapping_add(28), sym::menu_e_b__str35, fp);
                                F::Flash_ClipSetUv(c, l_flash_3, fp, fp.wrapping_add(8));
                                if (((((l_j_2) == (0i32)) as i32)) != 0) {
                                    F::Flash_FindLabel(c, l_flash_3, fp.wrapping_add(28), sym::menu_e_b__str36, fp);
                                    { let _ = F::Flash_ClipSetFlags(c, l_flash_3, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
                                } else {
                                    F::Flash_FindLabel(c, l_flash_3, fp.wrapping_add(28), sym::menu_e_b__str37, fp);
                                    { let a3_ = ((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2480)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add((((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48)); (c.m.r32(a_) as i32) }).wrapping_add(({ let a_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(7i32))) as u32)).wrapping_mul(36)); (c.m.r32(a_) as i32) }) != (161i32)) as i32); { let _ = F::Flash_ClipSetFlags(c, l_flash_3, fp, 2i32, ((((a3_) as i64 as u64)) as u8)); } };
                                }
                            }
                            { let o_ = l_j_2; let n_ = (o_).wrapping_add(1); l_j_2 = n_; };
                        }
                    }
                    {
                        l_j_2 = 0i32;
                        'l16: loop {
                            if !(((((l_j_2) < (4i32)) as i32)) != 0) { break 'l16; }
                            'c17: {
                                { let v_ = (rt::rem_i32(l_j_2, 2i32)).wrapping_mul(64i32); let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                                { let v_ = ((c.m.r32(fp.wrapping_add(8)) as i32)).wrapping_add(64i32); let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                                { let v_ = (rt::div_i32(l_j_2, 2i32)).wrapping_mul(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                                { let v_ = ((c.m.r32((fp.wrapping_add(8)).wrapping_add(4)) as i32)).wrapping_add(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                                { let _ = F::sprintf(c, fp.wrapping_add(28), sym::menu_e_b__str38, &[(((l_j_2).wrapping_add(1i32)) as i64 as u64)]); };
                                F::Flash_FindLabel(c, l_flash_3, ((0i32) as u32), fp.wrapping_add(28), fp);
                                if (((((l_j_2) < ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(15408)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                                    { let _ = F::Flash_ClipSetFlags(c, l_flash_3, fp, 2i32, ((((1i32) as i64 as u64)) as u8)); };
                                    's18: {
                                        let sv9: i32 = { let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(15408)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
                                        match sv9 {
                                            2i32 => {
                                                F::Flash_ClipSetOffset(c, l_flash_3, fp, 45i32, 0i32);
                                                break 's18;
                                            }
                                            3i32 => {
                                                F::Flash_ClipSetOffset(c, l_flash_3, fp, 22i32, 0i32);
                                                break 's18;
                                            }
                                            _ => {
                                            }
                                        }
                                    }
                                    F::Flash_FindLabel(c, l_flash_3, fp.wrapping_add(28), sym::menu_e_b__str39, fp);
                                    F::Flash_ClipSetUv(c, l_flash_3, fp, fp.wrapping_add(8));
                                    F::Flash_FindLabel(c, l_flash_3, fp.wrapping_add(28), sym::menu_e_b__str40, fp);
                                    F::Flash_ClipSetUv(c, l_flash_3, fp, fp.wrapping_add(8));
                                } else {
                                    { let _ = F::Flash_ClipSetFlags(c, l_flash_3, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
                                }
                            }
                            { let o_ = l_j_2; let n_ = (o_).wrapping_add(1); l_j_2 = n_; };
                        }
                    }
                }
                { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
            }
        }
        {
            l_i_1 = 0i32;
            'l19: loop {
                if !(((((l_i_1) < (2i32)) as i32)) != 0) { break 'l19; }
                'c20: {
                    l_flash_3 = ((c.m.r32(sym::gTeamSel)).wrapping_add(52)).wrapping_add(((((3i32).wrapping_add(l_i_1)) as u32)).wrapping_mul(44));
                    l_n_6 = ((0i32) as u32);
                    's21: {
                        let sv10: i32 = { let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15416); (c.m.r32(a_) as i32) };
                        match sv10 {
                            0i32 => {
                                l_n_6 = (((if ((l_i_1) != 0) { 2i32 } else { 0i32 })) as u32);
                                break 's21;
                            }
                            1i32 => {
                                l_n_6 = (((if ((l_i_1) != 0) { 1i32 } else { 0i32 })) as u32);
                                break 's21;
                            }
                            2i32 => {
                                l_n_6 = ((2i32) as u32);
                                break 's21;
                            }
                            _ => {
                            }
                        }
                    }
                    { let v_ = (((rt::div_u32(l_n_6, ((2i32) as u32))).wrapping_mul(((32i32) as u32))) as i32); let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    { let v_ = ((((l_n_6).wrapping_sub((rt::div_u32(l_n_6, ((2i32) as u32))).wrapping_mul(((2i32) as u32)))).wrapping_mul(((64i32) as u32))) as i32); let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = ((c.m.r32((fp.wrapping_add(8)).wrapping_add(4)) as i32)).wrapping_add(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                    { let v_ = ((c.m.r32(fp.wrapping_add(8)) as i32)).wrapping_add(64i32); let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    F::Flash_FindLabel(c, l_flash_3, ((0i32) as u32), sym::menu_e_b__str41, fp);
                    F::Flash_ClipSetUv(c, l_flash_3, fp, fp.wrapping_add(8));
                    if ((((({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(15392)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) >= (2i32)) as i32)) != 0) {
                        {
                            l_j_2 = 0i32;
                            'l22: loop {
                                if !(((((l_j_2) < (2i32)) as i32)) != 0) { break 'l22; }
                                'c23: {
                                    { let v_ = (l_j_2).wrapping_mul(32i32); let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                                    { let v_ = 32i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                                    { let v_ = ((c.m.r32(fp.wrapping_add(8)) as i32)).wrapping_add(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                                    { let v_ = 64i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                                    F::Flash_FindLabel(c, l_flash_3, (if ((l_j_2) != 0) { sym::menu_e_b__str42 } else { sym::menu_e_b__str43 }), (if ((l_j_2) != 0) { sym::menu_e_b__str44 } else { sym::menu_e_b__str45 }), fp);
                                    F::Flash_ClipSetUv(c, l_flash_3, fp, fp.wrapping_add(8));
                                    F::Flash_FindLabel(c, l_flash_3, ((0i32) as u32), (if ((l_j_2) != 0) { sym::menu_e_b__str46 } else { sym::menu_e_b__str47 }), fp);
                                    if (((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(296); (c.m.r32(a_) as i32) }) & (64i32))) != 0) {
                                        { let _ = F::Flash_ClipSetFlags(c, l_flash_3, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
                                    } else {
                                        if (((((((((l_i_1) == (1i32)) as i32)) != 0) && ((((({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15416); (c.m.r32(a_) as i32) }) != (1i32)) as i32)) != 0)) as i32)) != 0) {
                                            if ((((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(316); (c.m.r32(a_) as i32) }) != (8i32)) as i32)) != 0) {
                                                { let _ = F::Flash_ClipSetFlags(c, l_flash_3, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
                                            } else {
                                                { let _ = F::Flash_ClipSetFlags(c, l_flash_3, fp, 2i32, ((((1i32) as i64 as u64)) as u8)); };
                                            }
                                        } else {
                                            { let _ = F::Flash_ClipSetFlags(c, l_flash_3, fp, 2i32, ((((1i32) as i64 as u64)) as u8)); };
                                        }
                                    }
                                }
                                { let o_ = l_j_2; let n_ = (o_).wrapping_add(1); l_j_2 = n_; };
                            }
                        }
                    } else {
                        {
                            l_j_2 = 0i32;
                            'l24: loop {
                                if !(((((l_j_2) < (2i32)) as i32)) != 0) { break 'l24; }
                                'c25: {
                                    F::Flash_FindLabel(c, l_flash_3, ((0i32) as u32), (if ((l_j_2) != 0) { sym::menu_e_b__str48 } else { sym::menu_e_b__str49 }), fp);
                                    { let _ = F::Flash_ClipSetFlags(c, l_flash_3, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
                                }
                                { let o_ = l_j_2; let n_ = (o_).wrapping_add(1); l_j_2 = n_; };
                            }
                        }
                    }
                    F::Flash_FindLabel(c, l_flash_3, ((0i32) as u32), sym::menu_e_b__str50, fp);
                    if ((((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(320); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
                        { let _ = F::Flash_ClipSetFlags(c, l_flash_3, fp, 258i32, ((((1i32) as i64 as u64)) as u8)); };
                    } else {
                        { let _ = F::Flash_ClipSetFlags(c, l_flash_3, fp, 258i32, ((((0i32) as i64 as u64)) as u8)); };
                    }
                    {
                        l_j_2 = 0i32;
                        'l26: loop {
                            if !(((((l_j_2) < (14i32)) as i32)) != 0) { break 'l26; }
                            'c27: {
                                l_cols_7 = 7i32;
                                { let _ = F::sprintf(c, fp.wrapping_add(28), sym::menu_e_b__str51, &[((l_j_2) as i64 as u64)]); };
                                F::Flash_FindLabel(c, l_flash_3, ((0i32) as u32), fp.wrapping_add(28), fp);
                                { let a3_ = (((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(320); (c.m.r32(a_) as i32) }) as u8)) as i32); { let _ = F::Flash_ClipSetFlags(c, l_flash_3, fp, 128i32, ((((a3_) as i64 as u64)) as u8)); } };
                                l_chara_4 = { let a_ = ((({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(240)).wrapping_add((((rt::div_i32(l_j_2, l_cols_7)) as u32)).wrapping_mul(28))).wrapping_add((((rt::rem_i32(l_j_2, l_cols_7)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
                                if ((((({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15420); (c.m.r32(a_) as i32) }) == (2i32)) as i32)) != 0) {
                                    if (((((l_chara_4) < (161i32)) as i32)) != 0) {
                                        l_cost_5 = F::ChrTbl_GetCost(c, l_chara_4);
                                    } else {
                                        l_cost_5 = 0i32;
                                    }
                                    { let _ = F::sprintf(c, fp.wrapping_add(92), sym::menu_e_b__str52, &[((l_j_2) as i64 as u64)]); };
                                    F::Flash_FindLabel(c, l_flash_3, fp.wrapping_add(28), fp.wrapping_add(92), fp);
                                    if (((((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(304); (c.m.r32(a_) as i32) }).wrapping_add(l_cost_5)) > ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15428); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                                        F::Flash_ClipSetColor(c, l_flash_3, fp, F32(0x3ecccccc));
                                    } else {
                                        if (((((l_chara_4) < (161i32)) as i32)) != 0) {
                                            { let a2_ = (if ((F::TeamSel_IsCharaFree(c, l_i_1, l_chara_4)) != 0) { F32(0x3f800000) } else { F32(0x3ecccccc) }); F::Flash_ClipSetColor(c, l_flash_3, fp, a2_) };
                                        } else {
                                            F::Flash_ClipSetColor(c, l_flash_3, fp, F32(0x3f800000));
                                        }
                                    }
                                    { let _ = F::sprintf(c, fp.wrapping_add(92), sym::menu_e_b__str53, &[((l_j_2) as i64 as u64)]); };
                                    F::Flash_FindLabel(c, l_flash_3, fp.wrapping_add(28), fp.wrapping_add(92), fp);
                                    if (((((l_chara_4) < (161i32)) as i32)) != 0) {
                                        { let v_ = (rt::rem_i32(l_cost_5, 4i32)).wrapping_mul(32i32); let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                                        { let v_ = ((c.m.r32(fp.wrapping_add(8)) as i32)).wrapping_add(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                                        { let v_ = (rt::div_i32(l_cost_5, 4i32)).wrapping_mul(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                                        { let v_ = ((c.m.r32((fp.wrapping_add(8)).wrapping_add(4)) as i32)).wrapping_add(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                                        F::Flash_ClipSetUv(c, l_flash_3, fp, fp.wrapping_add(8));
                                        { let _ = F::Flash_ClipSetFlags(c, l_flash_3, fp, 2i32, ((((1i32) as i64 as u64)) as u8)); };
                                    } else {
                                        { let _ = F::Flash_ClipSetFlags(c, l_flash_3, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
                                    }
                                } else {
                                    { let _ = F::sprintf(c, fp.wrapping_add(92), sym::menu_e_b__str54, &[((l_j_2) as i64 as u64)]); };
                                    F::Flash_FindLabel(c, l_flash_3, fp.wrapping_add(28), fp.wrapping_add(92), fp);
                                    if (((((l_chara_4) < (161i32)) as i32)) != 0) {
                                        { let a2_ = (if ((F::TeamSel_IsCharaFree(c, l_i_1, l_chara_4)) != 0) { F32(0x3f800000) } else { F32(0x3ecccccc) }); F::Flash_ClipSetColor(c, l_flash_3, fp, a2_) };
                                    } else {
                                        F::Flash_ClipSetColor(c, l_flash_3, fp, F32(0x3f800000));
                                    }
                                }
                            }
                            { let o_ = l_j_2; let n_ = (o_).wrapping_add(1); l_j_2 = n_; };
                        }
                    }
                }
                { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
            }
        }
        {
            l_i_1 = 0i32;
            'l28: loop {
                if !(((((l_i_1) < (2i32)) as i32)) != 0) { break 'l28; }
                'c29: {
                    l_flash_3 = ((c.m.r32(sym::gTeamSel)).wrapping_add(52)).wrapping_add(((((1i32).wrapping_add(l_i_1)) as u32)).wrapping_mul(44));
                    {
                        l_j_2 = 0i32;
                        'l30: loop {
                            if !(((((l_j_2) < (5i32)) as i32)) != 0) { break 'l30; }
                            'c31: {
                                { let _ = F::sprintf(c, fp.wrapping_add(28), sym::menu_e_b__str55, &[((l_j_2) as i64 as u64)]); };
                                if (((((l_i_1) == (0i32)) as i32)) != 0) {
                                    { let v_ = (rt::rem_i32(l_j_2, 4i32)).wrapping_mul(64i32); let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                                    { let v_ = ((c.m.r32(fp.wrapping_add(8)) as i32)).wrapping_add(64i32); let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                                    { let v_ = (rt::div_i32(l_j_2, 4i32)).wrapping_mul(72i32); let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                                    { let v_ = ((c.m.r32((fp.wrapping_add(8)).wrapping_add(4)) as i32)).wrapping_add(72i32); let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                                } else {
                                    { let v_ = (rt::rem_i32((l_j_2).wrapping_add(5i32), 4i32)).wrapping_mul(64i32); let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                                    { let v_ = ((c.m.r32(fp.wrapping_add(8)) as i32)).wrapping_add(64i32); let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                                    { let v_ = (rt::div_i32((l_j_2).wrapping_add(5i32), 4i32)).wrapping_mul(72i32); let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                                    { let v_ = ((c.m.r32((fp.wrapping_add(8)).wrapping_add(4)) as i32)).wrapping_add(72i32); let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                                }
                                { let _ = F::sprintf(c, fp.wrapping_add(92), sym::menu_e_b__str56, &[((l_j_2) as i64 as u64)]); };
                                F::Flash_FindLabel(c, l_flash_3, fp.wrapping_add(28), fp.wrapping_add(92), fp);
                                F::Flash_ClipSetUv(c, l_flash_3, fp, fp.wrapping_add(8));
                                { let _ = F::sprintf(c, fp.wrapping_add(92), sym::menu_e_b__str57, &[((l_j_2) as i64 as u64)]); };
                                F::Flash_FindLabel(c, l_flash_3, fp.wrapping_add(28), fp.wrapping_add(92), fp);
                                if (((((l_j_2) < ({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(308); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                                    l_cost_5 = { let a0_ = { let a_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add((((l_j_2) as u32)).wrapping_mul(48))).wrapping_add(28); (c.m.r32(a_) as i32) }; F::ChrTbl_GetCost(c, a0_) };
                                    { let v_ = (rt::rem_i32(l_cost_5, 4i32)).wrapping_mul(32i32); let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                                    { let v_ = ((c.m.r32(fp.wrapping_add(8)) as i32)).wrapping_add(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                                    { let v_ = (rt::div_i32(l_cost_5, 4i32)).wrapping_mul(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                                    { let v_ = ((c.m.r32((fp.wrapping_add(8)).wrapping_add(4)) as i32)).wrapping_add(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                                    { let _ = F::Flash_ClipSetFlags(c, l_flash_3, fp, 2i32, ((((1i32) as i64 as u64)) as u8)); };
                                    F::Flash_ClipSetUv(c, l_flash_3, fp, fp.wrapping_add(8));
                                } else {
                                    { let _ = F::Flash_ClipSetFlags(c, l_flash_3, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
                                }
                                { let _ = F::sprintf(c, fp.wrapping_add(92), sym::menu_e_b__str58, &[((l_j_2) as i64 as u64)]); };
                                F::Flash_FindLabel(c, l_flash_3, fp.wrapping_add(28), fp.wrapping_add(92), fp);
                                if (((((l_j_2) < ({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(308); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                                    { let _ = F::Flash_ClipSetFlags(c, l_flash_3, fp, 2i32, ((((1i32) as i64 as u64)) as u8)); };
                                } else {
                                    { let _ = F::Flash_ClipSetFlags(c, l_flash_3, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
                                }
                            }
                            { let o_ = l_j_2; let n_ = (o_).wrapping_add(1); l_j_2 = n_; };
                        }
                    }
                }
                { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
            }
        }
        {
            l_i_1 = 0i32;
            'l32: loop {
                if !(((((l_i_1) < (7i32)) as i32)) != 0) { break 'l32; }
                'c33: {
                    { let a0_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(52)).wrapping_add((((l_i_1) as u32)).wrapping_mul(44)); F::Flash_Draw(c, a0_) };
                }
                { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
            }
        }
        F::Font_FlushAll(c);
        F::IconWin_Draw(c);
        F::ItemPanel_Draw(c, 0i32);
        F::ItemPanel_Draw(c, 1i32);
        { let a0_ = { let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(980); (c.m.r32(a_) as i32) }; F::ItemHelp_Draw(c, a0_) };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "TeamSel_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn TeamSel_Update(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "TeamSel_Update", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let r_ = 'f: {
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (7i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let a0_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(52)).wrapping_add((((l_i_1) as u32)).wrapping_mul(44)); F::Flash_Advance(c, a0_) };
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
    F::ItemPanel_Update(c, 0i32);
    F::ItemPanel_Update(c, 1i32);
    {
        l_i_1 = 0i32;
        'l3: loop {
            if !(((((l_i_1) < (2i32)) as i32)) != 0) { break 'l3; }
            'c4: {
                if ((((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(320); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
                    if (((({ let a_ = ((((c.m.r32(sym::gTeamSel)).wrapping_add(52)).wrapping_add((((3i32) as u32)).wrapping_mul(44))).wrapping_add((((l_i_1) as u32)).wrapping_mul(44))).wrapping_add(12); c.m.r32(a_) }) & (((1i32) as u32)))) != 0) {
                        { let v_ = 0i32; let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(320); c.m.w32(a_, (v_) as u32); };
                    }
                }
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
    if ((((({ let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(2468); c.m.r32(a_) }).wrapping_add(68); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
        if (((({ let a_ = (((c.m.r32(sym::gTeamSel)).wrapping_add(52)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add(12); c.m.r32(a_) }) & (((1i32) as u32)))) != 0) {
            { let v_ = 0i32; let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(2468); c.m.r32(a_) }).wrapping_add(68); c.m.w32(a_, (v_) as u32); };
        }
    }
    if ((((({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15468); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
        if ((((((({ let a_ = (((c.m.r32(sym::gTeamSel)).wrapping_add(52)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add(12); c.m.r32(a_) }) as i32)) < (0i32)) as i32)) != 0) {
            { let v_ = 0i32; let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15468); c.m.w32(a_, (v_) as u32); };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "TeamSel_Update", &PROBE_N, &[], true); }
    r_
}

pub fn TeamSel_Input(c: &mut Ctx, mut p_result_1: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "TeamSel_Input", &PROBE_N, &[(p_result_1) as u32], false); }
    let mut l_i_2: i32 = 0;
    let mut l_pads_3: i32 = 0;
    let mut l_r_4: i32 = 0;
    let mut l_chara_5: i32 = 0;
    let mut l_idx_6: i32 = 0;
    let r_ = 'f: {
    l_pads_3 = 0i32;
    if ((((!(((({ let a_ = (((c.m.r32(sym::gTeamSel)).wrapping_add(52)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add(8); c.m.r32(a_) }) & (((2i32) as u32)))) != 0)) as i32)) != 0) {
        break 'f F32(0);
    }
    's1: {
        let sv7: i32 = { let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15416); (c.m.r32(a_) as i32) };
        match sv7 {
            0i32 | 2i32 => {
                l_pads_3 = 1i32;
                break 's1;
            }
            1i32 => {
                l_pads_3 = 2i32;
                break 's1;
            }
            _ => {
            }
        }
    }
    if ((((!(((({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(956); (c.m.r32(a_) as i32) }) & (2i32))) != 0)) as i32)) != 0) {
        {
            l_i_2 = 0i32;
            'l2: loop {
                if !(((((l_i_2) < (l_pads_3)) as i32)) != 0) { break 'l2; }
                'c3: {
                    F::TeamSel_ClipGoto(c, (l_i_2).wrapping_add(3i32), l_i_2, 0i32, sym::menu_e_b__str59);
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        { let r_ = 2i32; let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(956); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
    if (((({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(956); (c.m.r32(a_) as i32) }) & (4i32))) != 0) {
        's4: {
            let sv8: i32 = { let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(2468); c.m.r32(a_) }).wrapping_add(64); (c.m.r32(a_) as i32) };
            match sv8 {
                9i32 => {
                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((1i32) as u32)))) != 0) {
                        F::TeamSel_ClipGoto(c, 0i32, 0i32, 9i32, sym::menu_e_b__str60);
                        { let a0_ = { let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15452); c.m.r32(a_) }; let a1_ = { let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(2468); c.m.r32(a_) }; let a2_ = { let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(2468); c.m.r32(a_) }).wrapping_add(4); (c.m.r32(a_) as i32) }; F::StgGrid_MoveLeft(c, a0_, a1_, a2_) };
                        F::TeamSel_ClipGoto(c, 0i32, 0i32, 9i32, sym::menu_e_b__str61);
                        if ((((({ let a_ = { let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(2468); c.m.r32(a_) }; (c.m.r32(a_) as i32) }) != (6i32)) as i32)) != 0) {
                            { let v_ = { let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15452); c.m.r32(a_) }).wrapping_add(((((({ let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(2468); c.m.r32(a_) }).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(6i32)).wrapping_add({ let a_ = { let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(2468); c.m.r32(a_) }; (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(2468); c.m.r32(a_) }).wrapping_add(60); c.m.w32(a_, (v_) as u32); };
                            F::TeamSel_RequestStage(c);
                        }
                        { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 0i32); };
                    } else {
                        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((2i32) as u32)))) != 0) {
                            F::TeamSel_ClipGoto(c, 0i32, 0i32, 9i32, sym::menu_e_b__str62);
                            { let a0_ = { let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15452); c.m.r32(a_) }; let a1_ = { let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(2468); c.m.r32(a_) }; let a2_ = { let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(2468); c.m.r32(a_) }).wrapping_add(4); (c.m.r32(a_) as i32) }; F::StgGrid_MoveRight(c, a0_, a1_, a2_) };
                            F::TeamSel_ClipGoto(c, 0i32, 0i32, 9i32, sym::menu_e_b__str63);
                            if ((((({ let a_ = { let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(2468); c.m.r32(a_) }; (c.m.r32(a_) as i32) }) != (6i32)) as i32)) != 0) {
                                { let v_ = { let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15452); c.m.r32(a_) }).wrapping_add(((((({ let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(2468); c.m.r32(a_) }).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(6i32)).wrapping_add({ let a_ = { let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(2468); c.m.r32(a_) }; (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(2468); c.m.r32(a_) }).wrapping_add(60); c.m.w32(a_, (v_) as u32); };
                                F::TeamSel_RequestStage(c);
                            }
                            { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 0i32); };
                        } else {
                            if ((((((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((8i32) as u32)))) != 0) && ((((({ let a_ = { let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(2468); c.m.r32(a_) }; (c.m.r32(a_) as i32) }) != (6i32)) as i32)) != 0)) as i32)) != 0) {
                                { let a0_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(52)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_e_b__str64, ((((1i32) as i64 as u64)) as u8)) };
                                { let v_ = 1i32; let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(2468); c.m.r32(a_) }).wrapping_add(68); c.m.w32(a_, (v_) as u32); };
                                F::TeamSel_ClipGoto(c, 0i32, 0i32, 9i32, sym::menu_e_b__str65);
                                { let a0_ = { let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15452); c.m.r32(a_) }; let a1_ = { let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(2468); c.m.r32(a_) }; let a2_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(2468); c.m.r32(a_) }).wrapping_add(4); F::StgGrid_MoveUp(c, a0_, a1_, a2_, 6i32) };
                                F::TeamSel_ClipGoto(c, 0i32, 0i32, 9i32, sym::menu_e_b__str66);
                                F::TeamSel_SetStageChips(c);
                                { let v_ = { let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15452); c.m.r32(a_) }).wrapping_add(((((({ let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(2468); c.m.r32(a_) }).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(6i32)).wrapping_add({ let a_ = { let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(2468); c.m.r32(a_) }; (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(2468); c.m.r32(a_) }).wrapping_add(60); c.m.w32(a_, (v_) as u32); };
                                F::TeamSel_RequestStage(c);
                                { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 2i32); };
                            } else {
                                if ((((((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((4i32) as u32)))) != 0) && ((((({ let a_ = { let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(2468); c.m.r32(a_) }; (c.m.r32(a_) as i32) }) != (6i32)) as i32)) != 0)) as i32)) != 0) {
                                    { let a0_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(52)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_e_b__str67, ((((1i32) as i64 as u64)) as u8)) };
                                    { let v_ = 1i32; let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(2468); c.m.r32(a_) }).wrapping_add(68); c.m.w32(a_, (v_) as u32); };
                                    F::TeamSel_ClipGoto(c, 0i32, 0i32, 9i32, sym::menu_e_b__str68);
                                    { let a0_ = { let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15452); c.m.r32(a_) }; let a1_ = { let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(2468); c.m.r32(a_) }; let a2_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(2468); c.m.r32(a_) }).wrapping_add(4); F::StgGrid_MoveDown(c, a0_, a1_, a2_, 6i32) };
                                    F::TeamSel_ClipGoto(c, 0i32, 0i32, 9i32, sym::menu_e_b__str69);
                                    F::TeamSel_SetStageChips(c);
                                    { let v_ = { let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15452); c.m.r32(a_) }).wrapping_add(((((({ let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(2468); c.m.r32(a_) }).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(6i32)).wrapping_add({ let a_ = { let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(2468); c.m.r32(a_) }; (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(2468); c.m.r32(a_) }).wrapping_add(60); c.m.w32(a_, (v_) as u32); };
                                    F::TeamSel_RequestStage(c);
                                    { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 1i32); };
                                } else {
                                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                                        { let v_ = 0i32; let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(2468); c.m.r32(a_) }).wrapping_add(68); c.m.w32(a_, (v_) as u32); };
                                        F::TeamSel_ClipGoto(c, 0i32, 0i32, 9i32, sym::menu_e_b__str70);
                                        if ((((({ let a_ = { let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(2468); c.m.r32(a_) }; (c.m.r32(a_) as i32) }) == (6i32)) as i32)) != 0) {
                                            { let a0_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(52)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_e_b__str71, ((((1i32) as i64 as u64)) as u8)) };
                                            F::IconWin_SetIcon(c, 1i32);
                                            F::TeamSel_ClipGoto(c, 0i32, 0i32, 10i32, sym::menu_e_b__str72);
                                            { let v_ = { let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(2468); c.m.r32(a_) }).wrapping_add(80); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(2468); c.m.r32(a_) }).wrapping_add(76); c.m.w32(a_, (v_) as u32); };
                                            { let v_ = 10i32; let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(2468); c.m.r32(a_) }).wrapping_add(64); c.m.w32(a_, (v_) as u32); };
                                        } else {
                                            { let a0_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(52)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_e_b__str73, ((((1i32) as i64 as u64)) as u8)) };
                                            F::IconWin_Close(c);
                                            { let v_ = 0i32; let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15468); c.m.w32(a_, (v_) as u32); };
                                            { let r_ = 8i32; let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(956); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                                            { let r_ = 16i32; let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(956); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                                            { let v_ = 60i32; let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(976); c.m.w32(a_, (v_) as u32); };
                                        }
                                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                                    } else {
                                        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1024i32) as u32)))) != 0) {
                                            { let v_ = 0i32; let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(2468); c.m.r32(a_) }).wrapping_add(68); c.m.w32(a_, (v_) as u32); };
                                            { let r_ = 4i32; let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(956); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
                                            { let a0_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(52)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_e_b__str74, ((((1i32) as i64 as u64)) as u8)) };
                                            F::IconWin_Close(c);
                                            F::TeamSel_ClipGoto(c, 0i32, 0i32, 9i32, sym::menu_e_b__str75);
                                            if ((((({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15416); (c.m.r32(a_) as i32) }) != (1i32)) as i32)) != 0) {
                                                { let a0_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(52)).wrapping_add((((2i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_e_b__str76, ((((1i32) as i64 as u64)) as u8)) };
                                                F::TeamSel_ClipGoto(c, 2i32, 1i32, 6i32, sym::menu_e_b__str77);
                                                { let v_ = 6i32; let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(316); c.m.w32(a_, (v_) as u32); };
                                            } else {
                                                { let a0_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(52)).wrapping_add((((1i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_e_b__str78, ((((1i32) as i64 as u64)) as u8)) };
                                                { let a0_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(52)).wrapping_add((((2i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_e_b__str79, ((((1i32) as i64 as u64)) as u8)) };
                                                F::TeamSel_ClipGoto(c, 1i32, 0i32, 6i32, sym::menu_e_b__str80);
                                                F::TeamSel_ClipGoto(c, 2i32, 1i32, 6i32, sym::menu_e_b__str81);
                                                { let v_ = 6i32; let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(316); c.m.w32(a_, (v_) as u32); };
                                                { let v_ = 6i32; let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(316); c.m.w32(a_, (v_) as u32); };
                                            }
                                            { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 2i32); };
                                        }
                                    }
                                }
                            }
                        }
                    }
                    break 's4;
                }
                10i32 => {
                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((8i32) as u32)))) != 0) {
                        { let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(2468); c.m.r32(a_) }).wrapping_add(76); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                        if ((((({ let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(2468); c.m.r32(a_) }).wrapping_add(76); (c.m.r32(a_) as i32) }) < (0i32)) as i32)) != 0) {
                            { let v_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15464); (c.m.r32(a_) as i32) }).wrapping_sub(1i32); let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(2468); c.m.r32(a_) }).wrapping_add(76); c.m.w32(a_, (v_) as u32); };
                        }
                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                    } else {
                        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((4i32) as u32)))) != 0) {
                            { let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(2468); c.m.r32(a_) }).wrapping_add(76); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                            if ((((({ let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(2468); c.m.r32(a_) }).wrapping_add(76); (c.m.r32(a_) as i32) }) > (({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15464); (c.m.r32(a_) as i32) }).wrapping_sub(1i32))) as i32)) != 0) {
                                { let v_ = 0i32; let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(2468); c.m.r32(a_) }).wrapping_add(76); c.m.w32(a_, (v_) as u32); };
                            }
                            { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                        } else {
                            if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                                if ((((({ let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15456); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(2468); c.m.r32(a_) }).wrapping_add(76); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) != (25i32)) as i32)) != 0) {
                                    { let a0_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(52)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_e_b__str82, ((((1i32) as i64 as u64)) as u8)) };
                                    F::IconWin_SetIcon(c, 0i32);
                                    F::TeamSel_ClipGoto(c, 0i32, 0i32, 10i32, sym::menu_e_b__str83);
                                    F::TeamSel_ClipGoto(c, 0i32, 0i32, 9i32, sym::menu_e_b__str84);
                                    { let v_ = { let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(2468); c.m.r32(a_) }).wrapping_add(76); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(2468); c.m.r32(a_) }).wrapping_add(80); c.m.w32(a_, (v_) as u32); };
                                    if ((((({ let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15456); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(2468); c.m.r32(a_) }).wrapping_add(80); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) != (24i32)) as i32)) != 0) {
                                        { let a0_ = ({ let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15456); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(2468); c.m.r32(a_) }).wrapping_add(80); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(68374i32); F::Bgm_Play(c, a0_) };
                                    }
                                    { let v_ = 9i32; let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(2468); c.m.r32(a_) }).wrapping_add(64); c.m.w32(a_, (v_) as u32); };
                                    { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                                } else {
                                    { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 7i32); };
                                }
                            } else {
                                if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1024i32) as u32)))) != 0) {
                                    { let a0_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(52)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_e_b__str85, ((((1i32) as i64 as u64)) as u8)) };
                                    F::IconWin_SetIcon(c, 0i32);
                                    F::TeamSel_ClipGoto(c, 0i32, 0i32, 10i32, sym::menu_e_b__str86);
                                    F::TeamSel_ClipGoto(c, 0i32, 0i32, 9i32, sym::menu_e_b__str87);
                                    { let v_ = { let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(2468); c.m.r32(a_) }).wrapping_add(80); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(2468); c.m.r32(a_) }).wrapping_add(76); c.m.w32(a_, (v_) as u32); };
                                    { let v_ = 9i32; let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(2468); c.m.r32(a_) }).wrapping_add(64); c.m.w32(a_, (v_) as u32); };
                                    { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 2i32); };
                                }
                            }
                        }
                    }
                    break 's4;
                }
                _ => {
                }
            }
        }
    } else {
        {
            l_i_2 = 0i32;
            'l5: loop {
                if !(((((l_i_2) < (l_pads_3)) as i32)) != 0) { break 'l5; }
                'c6: {
                    if (((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) ^ (1i32))) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(296); (c.m.r32(a_) as i32) }) & (2048i32))) != 0) {
                        break 'c6;
                    }
                    's7: {
                        let sv9: i32 = { let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(316); (c.m.r32(a_) as i32) };
                        match sv9 {
                            0i32 => {
                                if ((((c.m.r32(((sym::gPad).wrapping_add((((l_i_2) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((1i32) as u32)))) != 0) {
                                    { let a0_ = ((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })).wrapping_add(3i32); let a1_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_ClipGoto(c, a0_, a1_, 0i32, sym::menu_e_b__str88) };
                                    { let _ = { let a0_ = { let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2480)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }; let a1_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48)); let a2_ = { let a_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(4); (c.m.r32(a_) as i32) }; F::ChrGrid_MoveLeft(c, a0_, a1_, a2_) }; };
                                    { let a0_ = ((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })).wrapping_add(3i32); let a1_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_ClipGoto(c, a0_, a1_, 0i32, sym::menu_e_b__str89) };
                                    { let v_ = { let a_ = ((({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(240)).wrapping_add((((0i32) as u32)).wrapping_mul(28))).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(312); c.m.w32(a_, (v_) as u32); };
                                    { let a0_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_RequestFace(c, a0_) };
                                    { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 0i32); };
                                } else {
                                    if ((((c.m.r32(((sym::gPad).wrapping_add((((l_i_2) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((2i32) as u32)))) != 0) {
                                        { let a0_ = ((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })).wrapping_add(3i32); let a1_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_ClipGoto(c, a0_, a1_, 0i32, sym::menu_e_b__str90) };
                                        { let _ = { let a0_ = { let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2480)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }; let a1_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48)); let a2_ = { let a_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(4); (c.m.r32(a_) as i32) }; F::ChrGrid_MoveRight(c, a0_, a1_, a2_) }; };
                                        { let a0_ = ((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })).wrapping_add(3i32); let a1_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_ClipGoto(c, a0_, a1_, 0i32, sym::menu_e_b__str91) };
                                        { let v_ = { let a_ = ((({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(240)).wrapping_add((((0i32) as u32)).wrapping_mul(28))).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(312); c.m.w32(a_, (v_) as u32); };
                                        { let a0_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_RequestFace(c, a0_) };
                                        { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 0i32); };
                                    } else {
                                        if ((((c.m.r32(((sym::gPad).wrapping_add((((l_i_2) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((8i32) as u32)))) != 0) {
                                            if ((((({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(15392)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) < (2i32)) as i32)) != 0) {
                                                break 'c6;
                                            }
                                            { let a0_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(52)).wrapping_add((((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })).wrapping_add(3i32)) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_e_b__str92, ((((1i32) as i64 as u64)) as u8)) };
                                            { let v_ = 1i32; let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(320); c.m.w32(a_, (v_) as u32); };
                                            { let a0_ = ((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })).wrapping_add(3i32); let a1_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_ClipGoto(c, a0_, a1_, 0i32, sym::menu_e_b__str93) };
                                            { let a0_ = { let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2480)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }; let a1_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48)); let a2_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(4); let a3_ = { let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(15392)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; F::ChrGrid_MoveUp(c, a0_, a1_, a2_, a3_) };
                                            { let a0_ = ((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })).wrapping_add(3i32); let a1_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_ClipGoto(c, a0_, a1_, 0i32, sym::menu_e_b__str94) };
                                            { let a0_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_SetChips(c, a0_) };
                                            { let v_ = { let a_ = ((({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(240)).wrapping_add((((0i32) as u32)).wrapping_mul(28))).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(312); c.m.w32(a_, (v_) as u32); };
                                            { let a0_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_RequestFace(c, a0_) };
                                            { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 2i32); };
                                        } else {
                                            if ((((c.m.r32(((sym::gPad).wrapping_add((((l_i_2) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((4i32) as u32)))) != 0) {
                                                if ((((({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(15392)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) < (2i32)) as i32)) != 0) {
                                                    break 'c6;
                                                }
                                                { let a0_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(52)).wrapping_add((((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })).wrapping_add(3i32)) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_e_b__str95, ((((1i32) as i64 as u64)) as u8)) };
                                                { let v_ = 1i32; let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(320); c.m.w32(a_, (v_) as u32); };
                                                { let a0_ = ((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })).wrapping_add(3i32); let a1_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_ClipGoto(c, a0_, a1_, 0i32, sym::menu_e_b__str96) };
                                                { let a0_ = { let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2480)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }; let a1_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48)); let a2_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(4); let a3_ = { let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(15392)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; F::ChrGrid_MoveDown(c, a0_, a1_, a2_, a3_) };
                                                { let a0_ = ((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })).wrapping_add(3i32); let a1_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_ClipGoto(c, a0_, a1_, 0i32, sym::menu_e_b__str97) };
                                                { let a0_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_SetChips(c, a0_) };
                                                { let v_ = { let a_ = ((({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(240)).wrapping_add((((0i32) as u32)).wrapping_mul(28))).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(312); c.m.w32(a_, (v_) as u32); };
                                                { let a0_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_RequestFace(c, a0_) };
                                                { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 1i32); };
                                            } else {
                                                if ((((c.m.r32(((sym::gPad).wrapping_add((((l_i_2) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                                                    if ((((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2480)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((((({ let a_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(7i32)).wrapping_add({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48)); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(36)); (c.m.r32(a_) as i32) }) == (161i32)) as i32)) != 0) {
                                                        { let a0_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(52)).wrapping_add((((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })).wrapping_add(5i32)) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_e_b__str98, ((((1i32) as i64 as u64)) as u8)) };
                                                        { let a0_ = ((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })).wrapping_add(3i32); let a1_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_ClipGoto(c, a0_, a1_, 0i32, sym::menu_e_b__str99) };
                                                        { let v_ = (if ((((({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15420); (c.m.r32(a_) as i32) }) == (2i32)) as i32)) != 0) { 1i32 } else { 4i32 }); let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(15400)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                                        if ((((!((((({ let a_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(20); (c.m.r32(a_) as i32) }) < ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(15400)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) })) as i32)) != 0)) as i32)) != 0) {
                                                            { let v_ = 0i32; let a_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
                                                        }
                                                        { let a0_ = ((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })).wrapping_add(5i32); let a1_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_ClipGoto(c, a0_, a1_, 2i32, sym::menu_e_b__str100) };
                                                        { let v_ = 2i32; let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(15408)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                                        if ((((!((((({ let a_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(24); (c.m.r32(a_) as i32) }) < ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(15408)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) })) as i32)) != 0)) as i32)) != 0) {
                                                            { let v_ = 0i32; let a_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                                                        }
                                                        { let v_ = 2i32; let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(316); c.m.w32(a_, (v_) as u32); };
                                                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                                                    } else {
                                                        if ((((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2480)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((((({ let a_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(7i32)).wrapping_add({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48)); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(36)); (c.m.r32(a_) as i32) }) == (163i32)) as i32)) != 0) {
                                                            if ((((({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(14376)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
                                                                { let _ = { let a0_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(14384)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(504)); let a1_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(12); let a2_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(16); F::ChrGrid_FixCursor(c, a0_, a1_, a2_, 2i32) }; };
                                                                { let r_ = 128i32; let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(296); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                                                                { let a0_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(52)).wrapping_add((((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })).wrapping_add(3i32)) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_e_b__str101, ((((1i32) as i64 as u64)) as u8)) };
                                                                { let v_ = 1i32; let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(320); c.m.w32(a_, (v_) as u32); };
                                                                { let a0_ = ((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })).wrapping_add(3i32); let a1_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_ClipGoto(c, a0_, a1_, 0i32, sym::menu_e_b__str102) };
                                                                { let a0_ = ((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })).wrapping_add(3i32); let a1_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_ClipGoto(c, a0_, a1_, 7i32, sym::menu_e_b__str103) };
                                                                { let a0_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_SetCustomChips(c, a0_) };
                                                                { let v_ = { let a_ = ((({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(240)).wrapping_add((((0i32) as u32)).wrapping_mul(28))).wrapping_add(((({ let a_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(12); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(312); c.m.w32(a_, (v_) as u32); };
                                                                { let a0_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_RequestFace(c, a0_) };
                                                                { let v_ = 7i32; let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(316); c.m.w32(a_, (v_) as u32); };
                                                                { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 41i32); };
                                                            } else {
                                                                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 7i32); };
                                                            }
                                                        } else {
                                                            if (({ let a0_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); let a1_ = { let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(312); (c.m.r32(a_) as i32) }; F::TeamSel_FitsDp(c, a0_, a1_) }) != 0) {
                                                                if ((((({ let a_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2480)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((((({ let a_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(7i32)).wrapping_add({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48)); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(36))).wrapping_add(4); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
                                                                    { let r_ = 64i32; let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(296); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                                                                    if ((((!((((({ let a_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(8); (c.m.r32(a_) as i32) }) < ({ let a_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2480)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((((({ let a_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(7i32)).wrapping_add({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48)); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(36))).wrapping_add(4); (c.m.r32(a_) as i32) })) as i32)) != 0)) as i32)) != 0) {
                                                                        { let v_ = 0i32; let a_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                                                                    }
                                                                    { let a0_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(52)).wrapping_add((((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })).wrapping_add(3i32)) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_e_b__str104, ((((1i32) as i64 as u64)) as u8)) };
                                                                    { let v_ = 1i32; let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(320); c.m.w32(a_, (v_) as u32); };
                                                                    { let a0_ = ((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })).wrapping_add(3i32); let a1_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_ClipGoto(c, a0_, a1_, 0i32, sym::menu_e_b__str105) };
                                                                    { let a0_ = ((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })).wrapping_add(3i32); let a1_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_ClipGoto(c, a0_, a1_, 1i32, sym::menu_e_b__str106) };
                                                                    { let a0_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_SetGridFormChips(c, a0_) };
                                                                    { let v_ = { let a_ = ((({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(240)).wrapping_add((((0i32) as u32)).wrapping_mul(28))).wrapping_add(((({ let a_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(8); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(312); c.m.w32(a_, (v_) as u32); };
                                                                    { let a0_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_RequestFace(c, a0_) };
                                                                    { let v_ = 1i32; let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(316); c.m.w32(a_, (v_) as u32); };
                                                                    { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 41i32); };
                                                                } else {
                                                                    if (({ let a0_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); let a1_ = { let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(312); (c.m.r32(a_) as i32) }; F::TeamSel_IsCharaFree(c, a0_, a1_) }) != 0) {
                                                                        { let a0_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(52)).wrapping_add((((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })).wrapping_add(5i32)) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_e_b__str107, ((((1i32) as i64 as u64)) as u8)) };
                                                                        { let a0_ = ((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })).wrapping_add(3i32); let a1_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_ClipGoto(c, a0_, a1_, 0i32, sym::menu_e_b__str108) };
                                                                        { let v_ = (if ((((({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15420); (c.m.r32(a_) as i32) }) == (2i32)) as i32)) != 0) { 1i32 } else { 4i32 }); let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(15400)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                                                        if ((((!((((({ let a_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(20); (c.m.r32(a_) as i32) }) < ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(15400)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) })) as i32)) != 0)) as i32)) != 0) {
                                                                            { let v_ = 0i32; let a_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
                                                                        }
                                                                        { let a0_ = ((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })).wrapping_add(5i32); let a1_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_ClipGoto(c, a0_, a1_, 2i32, sym::menu_e_b__str109) };
                                                                        { let v_ = { let a0_ = { let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(312); (c.m.r32(a_) as i32) }; let a1_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(24); F::ChrTbl_WrapCostume(c, a0_, a1_) }; let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(15408)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                                                        { let v_ = 2i32; let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(316); c.m.w32(a_, (v_) as u32); };
                                                                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                                                                    } else {
                                                                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 7i32); };
                                                                    }
                                                                }
                                                            } else {
                                                                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 7i32); };
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    if ((((c.m.r32(((sym::gPad).wrapping_add((((l_i_2) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1024i32) as u32)))) != 0) {
                                                        { let v_ = 0i32; let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(320); c.m.w32(a_, (v_) as u32); };
                                                        if ((((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(308); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
                                                            { let a0_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(52)).wrapping_add((((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })).wrapping_add(3i32)) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_e_b__str110, ((((1i32) as i64 as u64)) as u8)) };
                                                            { let a0_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(52)).wrapping_add((((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })).wrapping_add(1i32)) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_e_b__str111, ((((1i32) as i64 as u64)) as u8)) };
                                                            { let a0_ = ((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })).wrapping_add(3i32); let a1_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_ClipGoto(c, a0_, a1_, 0i32, sym::menu_e_b__str112) };
                                                            { let a0_ = ((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })).wrapping_add(1i32); let a1_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_ClipGoto(c, a0_, a1_, 6i32, sym::menu_e_b__str113) };
                                                            { let s_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2288)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(48)); let d_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48)); rt::memcpy(c, d_, s_, 48); };
                                                            { let v_ = { let a_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(28); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(312); c.m.w32(a_, (v_) as u32); };
                                                            { let a0_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_RequestFace(c, a0_) };
                                                            { let a0_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_SumCost(c, a0_, 0i32) };
                                                            { let v_ = 6i32; let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(316); c.m.w32(a_, (v_) as u32); };
                                                        } else {
                                                            if ((((({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15416); (c.m.r32(a_) as i32) }) != (1i32)) as i32)) != 0) {
                                                                's8: {
                                                                    let sv10: i32 = { let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) };
                                                                    match sv10 {
                                                                        0i32 => {
                                                                            F::ColorFade_StartOut(c, 0i32, 0i32, 0i32, 20i32);
                                                                            { let v_ = 0i32; let a_ = p_result_1; c.m.w32(a_, (v_) as u32); };
                                                                            break 's8;
                                                                        }
                                                                        1i32 => {
                                                                            { let a1_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_ClipGoto(c, (l_i_2).wrapping_add(4i32), a1_, 0i32, sym::menu_e_b__str114) };
                                                                            { let v_ = 0i32; let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); c.m.w32(a_, (v_) as u32); };
                                                                            { let a0_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(52)).wrapping_add((((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })).wrapping_add(1i32)) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_e_b__str115, ((((1i32) as i64 as u64)) as u8)) };
                                                                            { let a0_ = ((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })).wrapping_add(1i32); let a1_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_ClipGoto(c, a0_, a1_, 6i32, sym::menu_e_b__str116) };
                                                                            { let v_ = 6i32; let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(316); c.m.w32(a_, (v_) as u32); };
                                                                            break 's8;
                                                                        }
                                                                        _ => {
                                                                        }
                                                                    }
                                                                }
                                                            } else {
                                                                F::ColorFade_StartOut(c, 0i32, 0i32, 0i32, 20i32);
                                                                { let v_ = 0i32; let a_ = p_result_1; c.m.w32(a_, (v_) as u32); };
                                                            }
                                                        }
                                                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 2i32); };
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                break 's7;
                            }
                            1i32 => {
                                if ((((c.m.r32(((sym::gPad).wrapping_add((((l_i_2) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((1i32) as u32)))) != 0) {
                                    { let a0_ = ((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })).wrapping_add(3i32); let a1_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_ClipGoto(c, a0_, a1_, 1i32, sym::menu_e_b__str117) };
                                    { let a0_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(240)).wrapping_add((((0i32) as u32)).wrapping_mul(28)); let a1_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(8); F::ChrGrid_PrevForm(c, a0_, a1_) };
                                    { let a0_ = ((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })).wrapping_add(3i32); let a1_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_ClipGoto(c, a0_, a1_, 1i32, sym::menu_e_b__str118) };
                                    { let v_ = { let a_ = ((({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(240)).wrapping_add((((0i32) as u32)).wrapping_mul(28))).wrapping_add(((({ let a_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(8); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(312); c.m.w32(a_, (v_) as u32); };
                                    { let a0_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_RequestFace(c, a0_) };
                                    { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 0i32); };
                                } else {
                                    if ((((c.m.r32(((sym::gPad).wrapping_add((((l_i_2) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((2i32) as u32)))) != 0) {
                                        { let a0_ = ((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })).wrapping_add(3i32); let a1_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_ClipGoto(c, a0_, a1_, 1i32, sym::menu_e_b__str119) };
                                        { let a0_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(240)).wrapping_add((((0i32) as u32)).wrapping_mul(28)); let a1_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(8); F::ChrGrid_NextForm(c, a0_, a1_) };
                                        { let a0_ = ((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })).wrapping_add(3i32); let a1_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_ClipGoto(c, a0_, a1_, 1i32, sym::menu_e_b__str120) };
                                        { let v_ = { let a_ = ((({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(240)).wrapping_add((((0i32) as u32)).wrapping_mul(28))).wrapping_add(((({ let a_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(8); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(312); c.m.w32(a_, (v_) as u32); };
                                        { let a0_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_RequestFace(c, a0_) };
                                        { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 0i32); };
                                    } else {
                                        if ((((c.m.r32(((sym::gPad).wrapping_add((((l_i_2) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                                            if ((((!(({ let a0_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); let a1_ = { let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(312); (c.m.r32(a_) as i32) }; F::TeamSel_FitsDp(c, a0_, a1_) }) != 0)) as i32)) != 0) {
                                                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 7i32); };
                                            } else {
                                                if ((((!(({ let a0_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); let a1_ = { let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(312); (c.m.r32(a_) as i32) }; F::TeamSel_IsCharaFree(c, a0_, a1_) }) != 0)) as i32)) != 0) {
                                                    { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 7i32); };
                                                } else {
                                                    { let a0_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(52)).wrapping_add((((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })).wrapping_add(5i32)) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_e_b__str121, ((((1i32) as i64 as u64)) as u8)) };
                                                    { let a0_ = ((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })).wrapping_add(3i32); let a1_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_ClipGoto(c, a0_, a1_, 1i32, sym::menu_e_b__str122) };
                                                    if (((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(296); (c.m.r32(a_) as i32) }) & (128i32))) != 0) {
                                                        { let v_ = (if ((((({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15420); (c.m.r32(a_) as i32) }) == (2i32)) as i32)) != 0) { 1i32 } else { 2i32 }); let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(15400)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                                        if ((((!((((({ let a_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(20); (c.m.r32(a_) as i32) }) < ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(15400)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) })) as i32)) != 0)) as i32)) != 0) {
                                                            { let v_ = 0i32; let a_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
                                                        }
                                                    } else {
                                                        { let v_ = (if ((((({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15420); (c.m.r32(a_) as i32) }) == (2i32)) as i32)) != 0) { 1i32 } else { 4i32 }); let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(15400)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                                        if ((((!((((({ let a_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(20); (c.m.r32(a_) as i32) }) < ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(15400)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) })) as i32)) != 0)) as i32)) != 0) {
                                                            { let v_ = 0i32; let a_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
                                                        }
                                                    }
                                                    { let a0_ = ((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })).wrapping_add(5i32); let a1_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_ClipGoto(c, a0_, a1_, 2i32, sym::menu_e_b__str123) };
                                                    { let v_ = { let a0_ = { let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(312); (c.m.r32(a_) as i32) }; let a1_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(24); F::ChrTbl_WrapCostume(c, a0_, a1_) }; let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(15408)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                                    { let v_ = 2i32; let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(316); c.m.w32(a_, (v_) as u32); };
                                                    { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                                                }
                                            }
                                        } else {
                                            if ((((c.m.r32(((sym::gPad).wrapping_add((((l_i_2) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1024i32) as u32)))) != 0) {
                                                { let r_ = 64i32; let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(296); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
                                                { let a0_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(52)).wrapping_add((((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })).wrapping_add(3i32)) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_e_b__str124, ((((1i32) as i64 as u64)) as u8)) };
                                                { let v_ = 1i32; let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(320); c.m.w32(a_, (v_) as u32); };
                                                if (((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(296); (c.m.r32(a_) as i32) }) & (128i32))) != 0) {
                                                    { let a0_ = ((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })).wrapping_add(3i32); let a1_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_ClipGoto(c, a0_, a1_, 1i32, sym::menu_e_b__str125) };
                                                    { let a0_ = ((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })).wrapping_add(3i32); let a1_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_ClipGoto(c, a0_, a1_, 7i32, sym::menu_e_b__str126) };
                                                    { let a0_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_SetCustomChips(c, a0_) };
                                                    { let v_ = { let a_ = ((({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(240)).wrapping_add((((0i32) as u32)).wrapping_mul(28))).wrapping_add(((({ let a_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(12); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(312); c.m.w32(a_, (v_) as u32); };
                                                    { let a0_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_RequestFace(c, a0_) };
                                                    { let v_ = 7i32; let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(316); c.m.w32(a_, (v_) as u32); };
                                                } else {
                                                    { let a0_ = ((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })).wrapping_add(3i32); let a1_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_ClipGoto(c, a0_, a1_, 1i32, sym::menu_e_b__str127) };
                                                    { let a0_ = ((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })).wrapping_add(3i32); let a1_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_ClipGoto(c, a0_, a1_, 0i32, sym::menu_e_b__str128) };
                                                    { let a0_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_SetChips(c, a0_) };
                                                    { let v_ = { let a_ = ((({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(240)).wrapping_add((((0i32) as u32)).wrapping_mul(28))).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(312); c.m.w32(a_, (v_) as u32); };
                                                    { let a0_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_RequestFace(c, a0_) };
                                                    { let v_ = 0i32; let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(316); c.m.w32(a_, (v_) as u32); };
                                                }
                                                { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 41i32); };
                                            }
                                        }
                                    }
                                }
                                break 's7;
                            }
                            2i32 => {
                                if ((((c.m.r32(((sym::gPad).wrapping_add((((l_i_2) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((8i32) as u32)))) != 0) {
                                    if ((((({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(15400)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) < (2i32)) as i32)) != 0) {
                                        break 'c6;
                                    }
                                    { let a0_ = ((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })).wrapping_add(5i32); let a1_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_ClipGoto(c, a0_, a1_, 2i32, sym::menu_e_b__str129) };
                                    { let a_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(20); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                                    if ((((({ let a_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(20); (c.m.r32(a_) as i32) }) < (0i32)) as i32)) != 0) {
                                        { let v_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(15400)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_sub(1i32); let a_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
                                    }
                                    { let a0_ = ((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })).wrapping_add(5i32); let a1_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_ClipGoto(c, a0_, a1_, 2i32, sym::menu_e_b__str130) };
                                    { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                                } else {
                                    if ((((c.m.r32(((sym::gPad).wrapping_add((((l_i_2) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((4i32) as u32)))) != 0) {
                                        if ((((({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(15400)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) < (2i32)) as i32)) != 0) {
                                            break 'c6;
                                        }
                                        { let a0_ = ((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })).wrapping_add(5i32); let a1_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_ClipGoto(c, a0_, a1_, 2i32, sym::menu_e_b__str131) };
                                        { let a_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(20); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                                        if ((((!((((({ let a_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(20); (c.m.r32(a_) as i32) }) < ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(15400)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) })) as i32)) != 0)) as i32)) != 0) {
                                            { let v_ = 0i32; let a_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
                                        }
                                        { let a0_ = ((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })).wrapping_add(5i32); let a1_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_ClipGoto(c, a0_, a1_, 2i32, sym::menu_e_b__str132) };
                                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                                    } else {
                                        if ((((c.m.r32(((sym::gPad).wrapping_add((((l_i_2) as u32)).wrapping_mul(448))).wrapping_add(396))) & ((((if ((((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) == (0i32)) as i32)) != 0) { 2i32 } else { 1i32 })) as u32)))) != 0) {
                                            if ((((({ let a_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(20); (c.m.r32(a_) as i32) }) <= (0i32)) as i32)) != 0) {
                                                break 'c6;
                                            }
                                            if ((((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2480)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((((({ let a_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(7i32)).wrapping_add({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48)); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(36)); (c.m.r32(a_) as i32) }) != (161i32)) as i32)) != 0) {
                                                { let a0_ = ((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })).wrapping_add(5i32); let a1_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_ClipGoto(c, a0_, a1_, 2i32, sym::menu_e_b__str133) };
                                                if ((((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2480)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((((({ let a_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(7i32)).wrapping_add({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48)); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(36)); (c.m.r32(a_) as i32) }) == (163i32)) as i32)) != 0) {
                                                    { let a0_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); let a1_ = { let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(312); (c.m.r32(a_) as i32) }; let a2_ = ({ let a_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(12); (c.m.r32(a_) as i32) }).wrapping_add(({ let a_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(16); (c.m.r32(a_) as i32) }).wrapping_mul(7i32)); F::ItemPanel_SetChara(c, a0_, a1_, a2_, 0i32, 1i32) };
                                                } else {
                                                    { let a0_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); let a1_ = { let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(312); (c.m.r32(a_) as i32) }; let a2_ = (({ let a_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(7i32)).wrapping_add({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48)); (c.m.r32(a_) as i32) }); let a3_ = ({ let a_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(20); (c.m.r32(a_) as i32) }).wrapping_sub(1i32); F::ItemPanel_SetChara(c, a0_, a1_, a2_, a3_, 0i32) };
                                                }
                                                { let a0_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::ItemPanel_Show(c, a0_) };
                                                { let r_ = 2048i32; let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(296); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                                                { let v_ = 3i32; let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(316); c.m.w32(a_, (v_) as u32); };
                                            }
                                        } else {
                                            if ((((c.m.r32(((sym::gPad).wrapping_add((((l_i_2) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                                                { let a0_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(52)).wrapping_add((((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })).wrapping_add(5i32)) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_e_b__str134, ((((1i32) as i64 as u64)) as u8)) };
                                                { let a0_ = ((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })).wrapping_add(5i32); let a1_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_ClipGoto(c, a0_, a1_, 2i32, sym::menu_e_b__str135) };
                                                { let a0_ = ((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })).wrapping_add(5i32); let a1_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_ClipGoto(c, a0_, a1_, 5i32, sym::menu_e_b__str136) };
                                                { let v_ = 5i32; let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(316); c.m.w32(a_, (v_) as u32); };
                                                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                                            } else {
                                                if ((((c.m.r32(((sym::gPad).wrapping_add((((l_i_2) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1024i32) as u32)))) != 0) {
                                                    { let a0_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(52)).wrapping_add((((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })).wrapping_add(5i32)) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_e_b__str137, ((((1i32) as i64 as u64)) as u8)) };
                                                    { let a0_ = ((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })).wrapping_add(5i32); let a1_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_ClipGoto(c, a0_, a1_, 2i32, sym::menu_e_b__str138) };
                                                    if (((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(296); (c.m.r32(a_) as i32) }) & (64i32))) != 0) {
                                                        { let a0_ = ((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })).wrapping_add(3i32); let a1_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_ClipGoto(c, a0_, a1_, 1i32, sym::menu_e_b__str139) };
                                                        { let v_ = 1i32; let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(316); c.m.w32(a_, (v_) as u32); };
                                                    } else {
                                                        if (((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(296); (c.m.r32(a_) as i32) }) & (128i32))) != 0) {
                                                            { let a0_ = ((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })).wrapping_add(3i32); let a1_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_ClipGoto(c, a0_, a1_, 7i32, sym::menu_e_b__str140) };
                                                            { let v_ = 7i32; let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(316); c.m.w32(a_, (v_) as u32); };
                                                        } else {
                                                            { let a0_ = ((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })).wrapping_add(3i32); let a1_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_ClipGoto(c, a0_, a1_, 0i32, sym::menu_e_b__str141) };
                                                            { let v_ = 0i32; let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(316); c.m.w32(a_, (v_) as u32); };
                                                        }
                                                    }
                                                    { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 2i32); };
                                                }
                                            }
                                        }
                                    }
                                }
                                break 's7;
                            }
                            3i32 => {
                                l_r_4 = { let a0_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::ItemPanel_Input(c, a0_, l_i_2) };
                                if (((((l_r_4) > (0i32)) as i32)) != 0) {
                                    F::ItemHelp_Open(c);
                                    { let v_ = (l_r_4).wrapping_sub(1i32); let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(980); c.m.w32(a_, (v_) as u32); };
                                    { let v_ = 4i32; let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(316); c.m.w32(a_, (v_) as u32); };
                                } else {
                                    if (((((l_r_4) < (0i32)) as i32)) != 0) {
                                        { let a0_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::ItemPanel_Hide(c, a0_) };
                                        { let r_ = (!(2048i32)); let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(296); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                                        { let a0_ = ((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })).wrapping_add(5i32); let a1_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_ClipGoto(c, a0_, a1_, 2i32, sym::menu_e_b__str142) };
                                        { let v_ = 2i32; let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(316); c.m.w32(a_, (v_) as u32); };
                                    }
                                }
                                break 's7;
                            }
                            4i32 => {
                                if ((((c.m.r32(((sym::gPad).wrapping_add((((l_i_2) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1536i32) as u32)))) != 0) {
                                    F::ItemHelp_Close(c);
                                    { let v_ = 3i32; let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(316); c.m.w32(a_, (v_) as u32); };
                                    { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 2i32); };
                                }
                                break 's7;
                            }
                            5i32 => {
                                if ((((c.m.r32(((sym::gPad).wrapping_add((((l_i_2) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((1i32) as u32)))) != 0) {
                                    { let a0_ = ((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })).wrapping_add(5i32); let a1_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_ClipGoto(c, a0_, a1_, 5i32, sym::menu_e_b__str143) };
                                    { let a_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(24); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                                    if ((((({ let a_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(24); (c.m.r32(a_) as i32) }) < (0i32)) as i32)) != 0) {
                                        { let v_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(15408)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_sub(1i32); let a_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                                    }
                                    { let a0_ = ((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })).wrapping_add(5i32); let a1_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_ClipGoto(c, a0_, a1_, 5i32, sym::menu_e_b__str144) };
                                    { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                                } else {
                                    if ((((c.m.r32(((sym::gPad).wrapping_add((((l_i_2) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((2i32) as u32)))) != 0) {
                                        { let a0_ = ((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })).wrapping_add(5i32); let a1_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_ClipGoto(c, a0_, a1_, 5i32, sym::menu_e_b__str145) };
                                        { let a_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(24); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                                        if ((((!((((({ let a_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(24); (c.m.r32(a_) as i32) }) < ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(15408)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) })) as i32)) != 0)) as i32)) != 0) {
                                            { let v_ = 0i32; let a_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                                        }
                                        { let a0_ = ((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })).wrapping_add(5i32); let a1_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_ClipGoto(c, a0_, a1_, 5i32, sym::menu_e_b__str146) };
                                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                                    } else {
                                        if ((((c.m.r32(((sym::gPad).wrapping_add((((l_i_2) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                                            { let v_ = 0i32; let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(320); c.m.w32(a_, (v_) as u32); };
                                            { let a0_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(52)).wrapping_add((((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })).wrapping_add(5i32)) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_e_b__str147, ((((1i32) as i64 as u64)) as u8)) };
                                            { let a0_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(52)).wrapping_add((((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })).wrapping_add(3i32)) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_e_b__str148, ((((1i32) as i64 as u64)) as u8)) };
                                            { let a0_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(52)).wrapping_add((((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })).wrapping_add(1i32)) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_e_b__str149, ((((1i32) as i64 as u64)) as u8)) };
                                            { let a0_ = ((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })).wrapping_add(5i32); let a1_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_ClipGoto(c, a0_, a1_, 5i32, sym::menu_e_b__str150) };
                                            { let a0_ = ((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })).wrapping_add(5i32); let a1_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_ClipGoto(c, a0_, a1_, 2i32, sym::menu_e_b__str151) };
                                            if (((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(296); (c.m.r32(a_) as i32) }) & (64i32))) != 0) {
                                                { let v_ = { let a_ = ((({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(240)).wrapping_add((((0i32) as u32)).wrapping_mul(28))).wrapping_add(((({ let a_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(8); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                                                if ((((({ let a_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(20); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
                                                    if ((((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2480)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((((({ let a_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(7i32)).wrapping_add({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48)); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(36)); (c.m.r32(a_) as i32) }) == (163i32)) as i32)) != 0) {
                                                        { let s_ = ((c.m.r32(sym::gSaveData)).wrapping_add(11584)).wrapping_add((((({ let a_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(12); (c.m.r32(a_) as i32) }).wrapping_add(({ let a_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(16); (c.m.r32(a_) as i32) }).wrapping_mul(7i32))) as u32)).wrapping_mul(28)); let d_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(32); rt::memcpy(c, d_, s_, 16); };
                                                    } else {
                                                        { let s_ = (((c.m.r32(sym::gSaveData)).wrapping_add(6152)).wrapping_add(((((({ let a_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(7i32)).wrapping_add({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48)); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(56))).wrapping_add((((({ let a_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(20); (c.m.r32(a_) as i32) }).wrapping_sub(1i32)) as u32)).wrapping_mul(16)); let d_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(32); rt::memcpy(c, d_, s_, 16); };
                                                    }
                                                } else {
                                                    { let _ = { let a0_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(32); F::memset(c, a0_, 0i32, 16u32) }; };
                                                }
                                                { let a0_ = ((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })).wrapping_add(3i32); let a1_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_ClipGoto(c, a0_, a1_, 1i32, sym::menu_e_b__str152) };
                                            } else {
                                                if (((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(296); (c.m.r32(a_) as i32) }) & (128i32))) != 0) {
                                                    { let v_ = { let a_ = ((({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(240)).wrapping_add((((0i32) as u32)).wrapping_mul(28))).wrapping_add(((({ let a_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(12); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                                                    if ((((({ let a_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(20); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
                                                        { let s_ = ((c.m.r32(sym::gSaveData)).wrapping_add(11584)).wrapping_add((((({ let a_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(12); (c.m.r32(a_) as i32) }).wrapping_add(({ let a_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(16); (c.m.r32(a_) as i32) }).wrapping_mul(7i32))) as u32)).wrapping_mul(28)); let d_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(32); rt::memcpy(c, d_, s_, 16); };
                                                    } else {
                                                        { let _ = { let a0_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(32); F::memset(c, a0_, 0i32, 16u32) }; };
                                                    }
                                                    { let a0_ = ((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })).wrapping_add(3i32); let a1_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_ClipGoto(c, a0_, a1_, 7i32, sym::menu_e_b__str153) };
                                                } else {
                                                    if ((((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2480)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((((({ let a_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(7i32)).wrapping_add({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48)); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(36)); (c.m.r32(a_) as i32) }) == (161i32)) as i32)) != 0) {
                                                        'l9: loop {
                                                            'c10: {
                                                                {
                                                                    'l11: loop {
                                                                        'c12: {
                                                                            l_idx_6 = (({ let a0_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2472)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32); F::Rand_Range(c, a0_) }) as i32);
                                                                            if ((((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2480)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add((((l_idx_6) as u32)).wrapping_mul(36)); (c.m.r32(a_) as i32) }) < (161i32)) as i32)) != 0) {
                                                                                l_chara_5 = { let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2480)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add((((l_idx_6) as u32)).wrapping_mul(36)); (c.m.r32(a_) as i32) };
                                                                                break 'l11;
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                if ((((({ let a_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2480)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add((((l_idx_6) as u32)).wrapping_mul(36))).wrapping_add(4); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
                                                                    l_chara_5 = { let a_ = ((({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2480)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add((((l_idx_6) as u32)).wrapping_mul(36))).wrapping_add(8)).wrapping_add(({ let a0_ = (({ let a_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2480)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add((((l_idx_6) as u32)).wrapping_mul(36))).wrapping_add(4); (c.m.r32(a_) as i32) }) as u32); F::Rand_Range(c, a0_) }).wrapping_mul(4)); (c.m.r32(a_) as i32) };
                                                                }
                                                            }
                                                            if !((((!(({ let a0_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_IsCharaFree(c, a0_, l_chara_5) }) != 0)) as i32)) != 0) { break 'l9; }
                                                        }
                                                        { let v_ = l_chara_5; let a_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                                                    } else {
                                                        { let v_ = { let a_ = ((({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(240)).wrapping_add((((0i32) as u32)).wrapping_mul(28))).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                                                        l_idx_6 = ({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48)); (c.m.r32(a_) as i32) }).wrapping_add(({ let a_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(7i32));
                                                    }
                                                    if ((((({ let a_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(20); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
                                                        { let s_ = (((c.m.r32(sym::gSaveData)).wrapping_add(6152)).wrapping_add((((l_idx_6) as u32)).wrapping_mul(56))).wrapping_add((((({ let a_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(20); (c.m.r32(a_) as i32) }).wrapping_sub(1i32)) as u32)).wrapping_mul(16)); let d_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(32); rt::memcpy(c, d_, s_, 16); };
                                                    } else {
                                                        { let _ = { let a0_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(32); F::memset(c, a0_, 0i32, 16u32) }; };
                                                    }
                                                    { let a0_ = ((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })).wrapping_add(3i32); let a1_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_ClipGoto(c, a0_, a1_, 0i32, sym::menu_e_b__str154) };
                                                }
                                            }
                                            { let a0_ = ((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })).wrapping_add(1i32); let a1_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_ClipGoto(c, a0_, a1_, 6i32, sym::menu_e_b__str155) };
                                            if ((((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) == ({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(308); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                                                { let v_ = ({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }).wrapping_add(1i32); let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(308); c.m.w32(a_, (v_) as u32); };
                                                { let v_ = { let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(308); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); c.m.w32(a_, (v_) as u32); };
                                            }
                                            { let a0_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_SetTeamTex(c, a0_) };
                                            { let a0_ = ((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })).wrapping_add(1i32); let a1_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_ClipGoto(c, a0_, a1_, 6i32, sym::menu_e_b__str156) };
                                            if ((((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) == (5i32)) as i32)) != 0) {
                                                { let v_ = { let a_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add((((0i32) as u32)).wrapping_mul(48))).wrapping_add(28); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(312); c.m.w32(a_, (v_) as u32); };
                                                { let a0_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_RequestFace(c, a0_) };
                                            }
                                            if (((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(296); (c.m.r32(a_) as i32) }) & (64i32))) != 0) {
                                                { let r_ = 64i32; let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(296); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
                                            }
                                            if (((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(296); (c.m.r32(a_) as i32) }) & (128i32))) != 0) {
                                                { let r_ = 128i32; let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(296); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
                                            }
                                            { let v_ = 6i32; let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(316); c.m.w32(a_, (v_) as u32); };
                                            { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                                        } else {
                                            if ((((c.m.r32(((sym::gPad).wrapping_add((((l_i_2) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1024i32) as u32)))) != 0) {
                                                { let a0_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(52)).wrapping_add((((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })).wrapping_add(5i32)) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_e_b__str157, ((((1i32) as i64 as u64)) as u8)) };
                                                { let a0_ = ((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })).wrapping_add(5i32); let a1_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_ClipGoto(c, a0_, a1_, 5i32, sym::menu_e_b__str158) };
                                                { let a0_ = ((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })).wrapping_add(5i32); let a1_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_ClipGoto(c, a0_, a1_, 2i32, sym::menu_e_b__str159) };
                                                { let v_ = 2i32; let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(316); c.m.w32(a_, (v_) as u32); };
                                                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 2i32); };
                                            }
                                        }
                                    }
                                }
                                break 's7;
                            }
                            6i32 => {
                                if ((((c.m.r32(((sym::gPad).wrapping_add((((l_i_2) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((1i32) as u32)))) != 0) {
                                    { let a0_ = ((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })).wrapping_add(1i32); let a1_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_ClipGoto(c, a0_, a1_, 6i32, sym::menu_e_b__str160) };
                                    { let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                                    if ((((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) < (0i32)) as i32)) != 0) {
                                        { let v_ = 5i32; let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); c.m.w32(a_, (v_) as u32); };
                                    } else {
                                        if ((((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) > ({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(308); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                                            { let v_ = { let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(308); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); c.m.w32(a_, (v_) as u32); };
                                        }
                                    }
                                    { let a0_ = ((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })).wrapping_add(1i32); let a1_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_ClipGoto(c, a0_, a1_, 6i32, sym::menu_e_b__str161) };
                                    if ((((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) == (5i32)) as i32)) != 0) {
                                        { let v_ = { let a_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add((((0i32) as u32)).wrapping_mul(48))).wrapping_add(28); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(312); c.m.w32(a_, (v_) as u32); };
                                    } else {
                                        { let v_ = { let a_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(28); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(312); c.m.w32(a_, (v_) as u32); };
                                    }
                                    { let a0_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_RequestFace(c, a0_) };
                                    { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                                } else {
                                    if ((((c.m.r32(((sym::gPad).wrapping_add((((l_i_2) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((2i32) as u32)))) != 0) {
                                        { let a0_ = ((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })).wrapping_add(1i32); let a1_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_ClipGoto(c, a0_, a1_, 6i32, sym::menu_e_b__str162) };
                                        { let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                                        if ((((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) > (5i32)) as i32)) != 0) {
                                            { let v_ = 0i32; let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); c.m.w32(a_, (v_) as u32); };
                                        } else {
                                            if ((((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) > ({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(308); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                                                { let v_ = 5i32; let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); c.m.w32(a_, (v_) as u32); };
                                            }
                                        }
                                        { let a0_ = ((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })).wrapping_add(1i32); let a1_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_ClipGoto(c, a0_, a1_, 6i32, sym::menu_e_b__str163) };
                                        if ((((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) == (5i32)) as i32)) != 0) {
                                            { let v_ = { let a_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add((((0i32) as u32)).wrapping_mul(48))).wrapping_add(28); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(312); c.m.w32(a_, (v_) as u32); };
                                        } else {
                                            { let v_ = { let a_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(28); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(312); c.m.w32(a_, (v_) as u32); };
                                        }
                                        { let a0_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_RequestFace(c, a0_) };
                                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                                    } else {
                                        if ((((c.m.r32(((sym::gPad).wrapping_add((((l_i_2) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                                            if ((((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) != (5i32)) as i32)) != 0) {
                                                { let a0_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_SumCost(c, a0_, 1i32) };
                                                { let a0_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(52)).wrapping_add((((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })).wrapping_add(1i32)) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_e_b__str164, ((((1i32) as i64 as u64)) as u8)) };
                                                { let a0_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(52)).wrapping_add((((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })).wrapping_add(3i32)) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_e_b__str165, ((((1i32) as i64 as u64)) as u8)) };
                                                { let a0_ = ((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })).wrapping_add(1i32); let a1_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_ClipGoto(c, a0_, a1_, 6i32, sym::menu_e_b__str166) };
                                                { let a0_ = ((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })).wrapping_add(3i32); let a1_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_ClipGoto(c, a0_, a1_, 0i32, sym::menu_e_b__str167) };
                                                { let a0_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_SetChips(c, a0_) };
                                                { let v_ = { let a_ = ((({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(240)).wrapping_add((((0i32) as u32)).wrapping_mul(28))).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(312); c.m.w32(a_, (v_) as u32); };
                                                { let a0_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_RequestFace(c, a0_) };
                                                { let s_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48)); let d_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2288)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(48)); rt::memcpy(c, d_, s_, 48); };
                                                { let v_ = 0i32; let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(316); c.m.w32(a_, (v_) as u32); };
                                                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                                            } else {
                                                if ((((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(308); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
                                                    { let a0_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(52)).wrapping_add((((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })).wrapping_add(1i32)) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_e_b__str168, ((((1i32) as i64 as u64)) as u8)) };
                                                    { let a0_ = ((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })).wrapping_add(1i32); let a1_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_ClipGoto(c, a0_, a1_, 6i32, sym::menu_e_b__str169) };
                                                    { let v_ = 8i32; let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(316); c.m.w32(a_, (v_) as u32); };
                                                    if ((((((((({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15416); (c.m.r32(a_) as i32) }) != (1i32)) as i32)) != 0) && ((((({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0)) as i32)) != 0) {
                                                        { let v_ = 1i32; let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); c.m.w32(a_, (v_) as u32); };
                                                        { let a0_ = ((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })).wrapping_add(3i32); let a1_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_ClipGoto(c, a0_, a1_, 0i32, sym::menu_e_b__str170) };
                                                        { let v_ = 0i32; let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(316); c.m.w32(a_, (v_) as u32); };
                                                    }
                                                    { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                                                } else {
                                                    { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 7i32); };
                                                }
                                            }
                                        } else {
                                            if ((((c.m.r32(((sym::gPad).wrapping_add((((l_i_2) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1024i32) as u32)))) != 0) {
                                                if ((((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(308); (c.m.r32(a_) as i32) }) == (1i32)) as i32)) != 0) {
                                                    { let a0_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(52)).wrapping_add((((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })).wrapping_add(1i32)) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_e_b__str171, ((((1i32) as i64 as u64)) as u8)) };
                                                    { let a0_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(52)).wrapping_add((((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })).wrapping_add(3i32)) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_e_b__str172, ((((1i32) as i64 as u64)) as u8)) };
                                                    { let a0_ = ((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })).wrapping_add(1i32); let a1_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_ClipGoto(c, a0_, a1_, 6i32, sym::menu_e_b__str173) };
                                                    { let v_ = 0i32; let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); c.m.w32(a_, (v_) as u32); };
                                                    { let a0_ = ((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })).wrapping_add(3i32); let a1_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_ClipGoto(c, a0_, a1_, 0i32, sym::menu_e_b__str174) };
                                                    { let a0_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_SetChips(c, a0_) };
                                                    { let v_ = { let a_ = ((({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(240)).wrapping_add((((0i32) as u32)).wrapping_mul(28))).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(312); c.m.w32(a_, (v_) as u32); };
                                                    { let a0_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_RequestFace(c, a0_) };
                                                    { let v_ = 0i32; let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(316); c.m.w32(a_, (v_) as u32); };
                                                } else {
                                                    if ((((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) == (5i32)) as i32)) != 0) {
                                                        { let a0_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); let a1_ = ({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(308); (c.m.r32(a_) as i32) }).wrapping_sub(1i32); F::TeamSel_RemoveMember(c, a0_, a1_) };
                                                    } else {
                                                        if ((((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) == ({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(308); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                                                            { let a0_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); let a1_ = ({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }).wrapping_sub(1i32); F::TeamSel_RemoveMember(c, a0_, a1_) };
                                                            { let a0_ = ((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })).wrapping_add(1i32); let a1_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_ClipGoto(c, a0_, a1_, 6i32, sym::menu_e_b__str175) };
                                                            { let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                                                            { let a0_ = ((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })).wrapping_add(1i32); let a1_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_ClipGoto(c, a0_, a1_, 6i32, sym::menu_e_b__str176) };
                                                            { let v_ = { let a_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(28); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(312); c.m.w32(a_, (v_) as u32); };
                                                            { let a0_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_RequestFace(c, a0_) };
                                                        } else {
                                                            { let a0_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); let a1_ = { let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }; F::TeamSel_RemoveMember(c, a0_, a1_) };
                                                            { let v_ = { let a_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(28); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(312); c.m.w32(a_, (v_) as u32); };
                                                            { let a0_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_RequestFace(c, a0_) };
                                                        }
                                                    }
                                                }
                                                { let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(308); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                                                { let a0_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_SetTeamTex(c, a0_) };
                                                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 2i32); };
                                            }
                                        }
                                    }
                                }
                                break 's7;
                            }
                            7i32 => {
                                if ((((c.m.r32(((sym::gPad).wrapping_add((((l_i_2) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((1i32) as u32)))) != 0) {
                                    { let a0_ = ((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })).wrapping_add(3i32); let a1_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_ClipGoto(c, a0_, a1_, 7i32, sym::menu_e_b__str177) };
                                    { let _ = { let a0_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(14384)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(504)); let a1_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(12); let a2_ = { let a_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(16); (c.m.r32(a_) as i32) }; F::ChrGrid_MoveLeft(c, a0_, a1_, a2_) }; };
                                    { let a0_ = ((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })).wrapping_add(3i32); let a1_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_ClipGoto(c, a0_, a1_, 7i32, sym::menu_e_b__str178) };
                                    { let v_ = { let a_ = ((({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(240)).wrapping_add((((0i32) as u32)).wrapping_mul(28))).wrapping_add(((({ let a_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(12); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(312); c.m.w32(a_, (v_) as u32); };
                                    { let a0_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_RequestFace(c, a0_) };
                                    { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 0i32); };
                                } else {
                                    if ((((c.m.r32(((sym::gPad).wrapping_add((((l_i_2) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((2i32) as u32)))) != 0) {
                                        { let a0_ = ((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })).wrapping_add(3i32); let a1_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_ClipGoto(c, a0_, a1_, 7i32, sym::menu_e_b__str179) };
                                        { let _ = { let a0_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(14384)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(504)); let a1_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(12); let a2_ = { let a_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(16); (c.m.r32(a_) as i32) }; F::ChrGrid_MoveRight(c, a0_, a1_, a2_) }; };
                                        { let a0_ = ((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })).wrapping_add(3i32); let a1_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_ClipGoto(c, a0_, a1_, 7i32, sym::menu_e_b__str180) };
                                        { let v_ = { let a_ = ((({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(240)).wrapping_add((((0i32) as u32)).wrapping_mul(28))).wrapping_add(((({ let a_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(12); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(312); c.m.w32(a_, (v_) as u32); };
                                        { let a0_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_RequestFace(c, a0_) };
                                        { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 0i32); };
                                    } else {
                                        if ((((c.m.r32(((sym::gPad).wrapping_add((((l_i_2) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((8i32) as u32)))) != 0) {
                                            { let a0_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(52)).wrapping_add((((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })).wrapping_add(3i32)) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_e_b__str181, ((((1i32) as i64 as u64)) as u8)) };
                                            { let v_ = 1i32; let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(320); c.m.w32(a_, (v_) as u32); };
                                            { let a0_ = ((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })).wrapping_add(3i32); let a1_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_ClipGoto(c, a0_, a1_, 7i32, sym::menu_e_b__str182) };
                                            { let a0_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(14384)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(504)); let a1_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(12); let a2_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(16); F::ChrGrid_MoveUp(c, a0_, a1_, a2_, 2i32) };
                                            { let a0_ = ((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })).wrapping_add(3i32); let a1_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_ClipGoto(c, a0_, a1_, 7i32, sym::menu_e_b__str183) };
                                            { let a0_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_SetCustomChips(c, a0_) };
                                            { let v_ = { let a_ = ((({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(240)).wrapping_add((((0i32) as u32)).wrapping_mul(28))).wrapping_add(((({ let a_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(12); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(312); c.m.w32(a_, (v_) as u32); };
                                            { let a0_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_RequestFace(c, a0_) };
                                            { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 2i32); };
                                        } else {
                                            if ((((c.m.r32(((sym::gPad).wrapping_add((((l_i_2) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((4i32) as u32)))) != 0) {
                                                { let a0_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(52)).wrapping_add((((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })).wrapping_add(3i32)) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_e_b__str184, ((((1i32) as i64 as u64)) as u8)) };
                                                { let v_ = 1i32; let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(320); c.m.w32(a_, (v_) as u32); };
                                                { let a0_ = ((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })).wrapping_add(3i32); let a1_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_ClipGoto(c, a0_, a1_, 7i32, sym::menu_e_b__str185) };
                                                { let a0_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(14384)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(504)); let a1_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(12); let a2_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(16); F::ChrGrid_MoveDown(c, a0_, a1_, a2_, 2i32) };
                                                { let a0_ = ((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })).wrapping_add(3i32); let a1_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_ClipGoto(c, a0_, a1_, 7i32, sym::menu_e_b__str186) };
                                                { let a0_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_SetCustomChips(c, a0_) };
                                                { let v_ = { let a_ = ((({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(240)).wrapping_add((((0i32) as u32)).wrapping_mul(28))).wrapping_add(((({ let a_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(12); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(312); c.m.w32(a_, (v_) as u32); };
                                                { let a0_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_RequestFace(c, a0_) };
                                                { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 1i32); };
                                            } else {
                                                if ((((c.m.r32(((sym::gPad).wrapping_add((((l_i_2) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                                                    if ((((!(({ let a0_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); let a1_ = { let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(312); (c.m.r32(a_) as i32) }; F::TeamSel_FitsDp(c, a0_, a1_) }) != 0)) as i32)) != 0) {
                                                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 7i32); };
                                                    } else {
                                                        if ((((({ let a_ = ((((c.m.r32(sym::gTeamSel)).wrapping_add(14384)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(504))).wrapping_add((((({ let a_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(12); (c.m.r32(a_) as i32) }).wrapping_add(({ let a_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(16); (c.m.r32(a_) as i32) }).wrapping_mul(7i32))) as u32)).wrapping_mul(36))).wrapping_add(4); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
                                                            { let r_ = 64i32; let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(296); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                                                            if ((((!((((({ let a_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(8); (c.m.r32(a_) as i32) }) < ({ let a_ = ((((c.m.r32(sym::gTeamSel)).wrapping_add(14384)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(504))).wrapping_add((((({ let a_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(12); (c.m.r32(a_) as i32) }).wrapping_add(({ let a_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(16); (c.m.r32(a_) as i32) }).wrapping_mul(7i32))) as u32)).wrapping_mul(36))).wrapping_add(4); (c.m.r32(a_) as i32) })) as i32)) != 0)) as i32)) != 0) {
                                                                { let v_ = 0i32; let a_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                                                            }
                                                            { let a0_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(52)).wrapping_add((((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })).wrapping_add(3i32)) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_e_b__str187, ((((1i32) as i64 as u64)) as u8)) };
                                                            { let v_ = 1i32; let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(320); c.m.w32(a_, (v_) as u32); };
                                                            { let a0_ = ((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })).wrapping_add(3i32); let a1_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_ClipGoto(c, a0_, a1_, 7i32, sym::menu_e_b__str188) };
                                                            { let a0_ = ((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })).wrapping_add(3i32); let a1_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_ClipGoto(c, a0_, a1_, 1i32, sym::menu_e_b__str189) };
                                                            { let a0_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_SetFormChips(c, a0_) };
                                                            { let v_ = { let a_ = ((({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(240)).wrapping_add((((0i32) as u32)).wrapping_mul(28))).wrapping_add(((({ let a_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(8); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(312); c.m.w32(a_, (v_) as u32); };
                                                            { let a0_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_RequestFace(c, a0_) };
                                                            { let v_ = 1i32; let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(316); c.m.w32(a_, (v_) as u32); };
                                                            { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 41i32); };
                                                        } else {
                                                            if (({ let a0_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); let a1_ = { let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(312); (c.m.r32(a_) as i32) }; F::TeamSel_IsCharaFree(c, a0_, a1_) }) != 0) {
                                                                { let a0_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(52)).wrapping_add((((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })).wrapping_add(5i32)) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_e_b__str190, ((((1i32) as i64 as u64)) as u8)) };
                                                                { let a0_ = ((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })).wrapping_add(3i32); let a1_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_ClipGoto(c, a0_, a1_, 7i32, sym::menu_e_b__str191) };
                                                                { let v_ = (if ((((({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15420); (c.m.r32(a_) as i32) }) == (2i32)) as i32)) != 0) { 1i32 } else { 2i32 }); let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(15400)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                                                if ((((!((((({ let a_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(20); (c.m.r32(a_) as i32) }) < ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(15400)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) })) as i32)) != 0)) as i32)) != 0) {
                                                                    { let v_ = 0i32; let a_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
                                                                }
                                                                { let a0_ = ((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })).wrapping_add(5i32); let a1_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_ClipGoto(c, a0_, a1_, 2i32, sym::menu_e_b__str192) };
                                                                { let v_ = { let a0_ = { let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(312); (c.m.r32(a_) as i32) }; let a1_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(24); F::ChrTbl_WrapCostume(c, a0_, a1_) }; let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(15408)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                                                { let v_ = 2i32; let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(316); c.m.w32(a_, (v_) as u32); };
                                                                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                                                            } else {
                                                                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 7i32); };
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    if ((((c.m.r32(((sym::gPad).wrapping_add((((l_i_2) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1024i32) as u32)))) != 0) {
                                                        { let r_ = 128i32; let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(296); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
                                                        { let a0_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(52)).wrapping_add((((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })).wrapping_add(3i32)) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_e_b__str193, ((((1i32) as i64 as u64)) as u8)) };
                                                        { let v_ = 1i32; let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(320); c.m.w32(a_, (v_) as u32); };
                                                        { let a0_ = ((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })).wrapping_add(3i32); let a1_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_ClipGoto(c, a0_, a1_, 7i32, sym::menu_e_b__str194) };
                                                        { let a0_ = ((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })).wrapping_add(3i32); let a1_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_ClipGoto(c, a0_, a1_, 0i32, sym::menu_e_b__str195) };
                                                        { let a0_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_SetChips(c, a0_) };
                                                        { let v_ = { let a_ = ((({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(240)).wrapping_add((((0i32) as u32)).wrapping_mul(28))).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(312); c.m.w32(a_, (v_) as u32); };
                                                        { let a0_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_RequestFace(c, a0_) };
                                                        { let v_ = 0i32; let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(316); c.m.w32(a_, (v_) as u32); };
                                                        { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 41i32); };
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                break 's7;
                            }
                            8i32 => {
                                if (((({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(956); (c.m.r32(a_) as i32) }) & (4i32))) != 0) {
                                    break 'c6;
                                }
                                if ((((({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) ^ (1i32))) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(316); (c.m.r32(a_) as i32) }) == (8i32)) as i32)) != 0) {
                                    { let a0_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(52)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_e_b__str196, ((((1i32) as i64 as u64)) as u8)) };
                                    F::IconWin_Open(c);
                                    F::TeamSel_ClipGoto(c, 0i32, 0i32, 9i32, sym::menu_e_b__str197);
                                    { let r_ = 4i32; let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(956); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                                    { let v_ = 1i32; let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15468); c.m.w32(a_, (v_) as u32); };
                                    { let v_ = 9i32; let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(2468); c.m.r32(a_) }).wrapping_add(64); c.m.w32(a_, (v_) as u32); };
                                } else {
                                    if ((((c.m.r32(((sym::gPad).wrapping_add((((l_i_2) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1024i32) as u32)))) != 0) {
                                        { let a0_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(52)).wrapping_add((((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })).wrapping_add(1i32)) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_e_b__str198, ((((1i32) as i64 as u64)) as u8)) };
                                        { let a0_ = ((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })).wrapping_add(1i32); let a1_ = (l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) }); F::TeamSel_ClipGoto(c, a0_, a1_, 6i32, sym::menu_e_b__str199) };
                                        { let v_ = 6i32; let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add(((((l_i_2).wrapping_add({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15448); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(316); c.m.w32(a_, (v_) as u32); };
                                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 2i32); };
                                    }
                                }
                                break 's7;
                            }
                            _ => {
                            }
                        }
                    }
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
    }
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "TeamSel_Input", &PROBE_N, &[], true); }
    r_
}

pub fn TeamSel_Run(c: &mut Ctx, mut p_section_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "TeamSel_Run", &PROBE_N, &[(p_section_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_i_2: i32 = 0;
    let mut l_j_3: i32 = 0;
    let mut l_bgm_4: i32 = 0;
    let mut l_timeLimit_5: i32 = 0;
    let mut l_cpuLevel_6: i32 = 0;
    let r_ = 'f: {
        { let v_ = 1i32; c.m.w32(fp, (v_) as u32); }
        F::TeamSel_Init(c, p_section_1);
        F::ColorFade_StartIn(c, 0i32, 0i32, 0i32, 20i32);
        'l1: while ((1i32) != 0) {
            F::Gfx_BeginFrame(c);
            F::TeamSel_UpdateFaceLoad(c);
            F::TeamSel_UpdateStageLoad(c);
            F::Pad_Update(c);
            F::Snd_Update(c);
            F::ColorFade_Update(c);
            if ((((!(((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(20); (c.m.r32(a_) as i32) }) & (256i32))) != 0)) as i32)) != 0) {
                F::TeamSel_Update(c);
            }
            F::TeamSel_Draw(c);
            F::Font_FlushAll(c);
            F::ColorFade_Draw(c);
            F::Gfx_EndFrame(c, 1i32);
            F::Dma_Flush(c);
            F::File_Stub264D90(c);
            if (((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(20); (c.m.r32(a_) as i32) }) & (256i32))) != 0) {
                continue 'l1;
            }
            if ((F::ColorFade_IsFadingOut(c)) != 0) {
                F::Bgm_FadeOutStep(c);
                continue 'l1;
            }
            if ((F::ColorFade_IsOutDone(c)) != 0) {
                if ((((((((({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(960); (c.m.r32(a_) as i32) }) == (4i32)) as i32)) != 0) || ((((({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(964); (c.m.r32(a_) as i32) }) == (4i32)) as i32)) != 0)) as i32)) != 0) {
                    break 'l1;
                }
                continue 'l1;
            }
            if (((({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(956); (c.m.r32(a_) as i32) }) & (16i32))) != 0) {
                if ((((({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(976); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) == ((1i32).wrapping_neg())) as i32)) != 0) {
                    F::ColorFade_StartOut(c, 0i32, 0i32, 0i32, 20i32);
                    { let s_ = { let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }; let d_ = ((c.m.r32(sym::gProgress)).wrapping_add(1088)).wrapping_add((((0i32) as u32)).wrapping_mul(240)); rt::memcpy(c, d_, s_, 240); };
                    { let s_ = { let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }; let d_ = ((c.m.r32(sym::gProgress)).wrapping_add(1088)).wrapping_add((((1i32) as u32)).wrapping_mul(240)); rt::memcpy(c, d_, s_, 240); };
                    { let v_ = ({ let a_ = { let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(2468); c.m.r32(a_) }; (c.m.r32(a_) as i32) }).wrapping_add(({ let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(2468); c.m.r32(a_) }).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(6i32)); let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1576); c.m.w32(a_, (v_) as u32); };
                    { let v_ = { let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15456); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(2468); c.m.r32(a_) }).wrapping_add(80); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1580); c.m.w32(a_, (v_) as u32); };
                }
                continue 'l1;
            }
            { let _ = F::TeamSel_Input(c, fp); };
        }
        if ((((((c.m.r32(fp) as i32)) != (0i32)) as i32)) != 0) {
            rt::memset(c, fp.wrapping_add(4), 0, 8);
            { let v_ = 1i32; c.m.w32(fp.wrapping_add(4).wrapping_add(0), (v_) as u32); }
            { let v_ = 1i32; c.m.w32(fp.wrapping_add(4).wrapping_add(4), (v_) as u32); }
            if ((((({ let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(2468); c.m.r32(a_) }).wrapping_add(60); (c.m.r32(a_) as i32) }) == (35i32)) as i32)) != 0) {
                'l2: loop {
                    'c3: {
                        { let v_ = { let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15452); c.m.r32(a_) }).wrapping_add(({ let a0_ = ((({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15460); (c.m.r32(a_) as i32) }).wrapping_sub(1i32)) as u32); F::Rand_Range(c, a0_) }).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(2468); c.m.r32(a_) }).wrapping_add(60); c.m.w32(a_, (v_) as u32); };
                    }
                    if !((((({ let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(2468); c.m.r32(a_) }).wrapping_add(60); (c.m.r32(a_) as i32) }) >= (35i32)) as i32)) != 0) { break 'l2; }
                }
            }
            l_bgm_4 = { let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15456); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(2468); c.m.r32(a_) }).wrapping_add(80); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
            if (((((l_bgm_4) == (24i32)) as i32)) != 0) {
                l_bgm_4 = (((F::Rand_Range(c, ((9i32) as u32))).wrapping_add(((8i32) as u32))) as i32);
            }
            's4: {
                let sv7: i32 = { let a_ = ((c.m.r32(sym::gSaveData)).wrapping_add(3124)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
                match sv7 {
                    0i32 => {
                        l_timeLimit_5 = 1i32;
                        break 's4;
                    }
                    1i32 => {
                        l_timeLimit_5 = 2i32;
                        break 's4;
                    }
                    2i32 => {
                        l_timeLimit_5 = 3i32;
                        break 's4;
                    }
                    3i32 => {
                        l_timeLimit_5 = 4i32;
                        break 's4;
                    }
                    4i32 => {
                        l_timeLimit_5 = 0i32;
                        break 's4;
                    }
                    _ => {
                        l_timeLimit_5 = 5i32;
                        break 's4;
                    }
                }
            }
            { let v_ = (({ let a_ = ((c.m.r32(sym::gSaveData)).wrapping_add(3124)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) ^ (1i32)); let a_ = (fp.wrapping_add(4)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            { let v_ = (({ let a_ = ((c.m.r32(sym::gSaveData)).wrapping_add(3124)).wrapping_add((((4i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) ^ (1i32)); let a_ = (fp.wrapping_add(4)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            l_cpuLevel_6 = { let a0_ = (({ let a_ = ((c.m.r32(sym::gSaveData)).wrapping_add(3124)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32); F::CpuLevel_FromSetting(c, a0_) };
            F::Battle_ClearWork(c);
            { let a0_ = ((({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15416); (c.m.r32(a_) as i32) }) == (1i32)) as i32); let a4_ = { let a_ = ((c.m.r32(sym::gSaveData)).wrapping_add(3124)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a5_ = { let a_ = ({ let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(2468); c.m.r32(a_) }).wrapping_add(60); (c.m.r32(a_) as i32) }; let a6_ = (({ let a_ = ((c.m.r32(sym::gSaveData)).wrapping_add(3124)).wrapping_add((((5i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) ^ (1i32)); F::BattleSetup_SetRule(c, a0_, 0i32, l_bgm_4, l_timeLimit_5, a4_, a5_, a6_) };
            's5: {
                let sv8: i32 = { let a_ = (c.m.r32(sym::gTeamSel)).wrapping_add(15416); (c.m.r32(a_) as i32) };
                match sv8 {
                    0i32 => {
                        { let a3_ = { let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(308); (c.m.r32(a_) as i32) }; F::BattleSetup_SetSide(c, 0i32, 0i32, 0i32, a3_, 1i32, 1i32, 0i32, ((0i32) as u32)) };
                        { let a3_ = { let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(308); (c.m.r32(a_) as i32) }; let a4_ = (c.m.r32((fp.wrapping_add(4)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32); F::BattleSetup_SetSide(c, 1i32, 2i32, 1i32, a3_, a4_, 1i32, 0i32, ((0i32) as u32)) };
                        break 's5;
                    }
                    1i32 => {
                        { let a3_ = { let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(308); (c.m.r32(a_) as i32) }; F::BattleSetup_SetSide(c, 0i32, 0i32, 0i32, a3_, 1i32, 1i32, 0i32, ((0i32) as u32)) };
                        { let a3_ = { let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(308); (c.m.r32(a_) as i32) }; F::BattleSetup_SetSide(c, 1i32, 0i32, 1i32, a3_, 1i32, 1i32, 0i32, ((0i32) as u32)) };
                        break 's5;
                    }
                    2i32 => {
                        { let a3_ = { let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(308); (c.m.r32(a_) as i32) }; let a4_ = (c.m.r32((fp.wrapping_add(4)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32); F::BattleSetup_SetSide(c, 0i32, 2i32, 0i32, a3_, a4_, 1i32, 0i32, ((0i32) as u32)) };
                        { let a3_ = { let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(308); (c.m.r32(a_) as i32) }; let a4_ = (c.m.r32((fp.wrapping_add(4)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32); F::BattleSetup_SetSide(c, 1i32, 2i32, 1i32, a3_, a4_, 1i32, 0i32, ((0i32) as u32)) };
                        break 's5;
                    }
                    _ => {
                    }
                }
            }
            {
                l_i_2 = 0i32;
                'l6: loop {
                    if !(((((l_i_2) < (2i32)) as i32)) != 0) { break 'l6; }
                    'c7: {
                        {
                            l_j_3 = 0i32;
                            'l8: loop {
                                if !(((((l_j_3) < ({ let a_ = ({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(308); (c.m.r32(a_) as i32) })) as i32)) != 0) { break 'l8; }
                                'c9: {
                                    { let a2_ = { let a_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add((((l_j_3) as u32)).wrapping_mul(48))).wrapping_add(28); (c.m.r32(a_) as i32) }; let a3_ = { let a_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add((((l_j_3) as u32)).wrapping_mul(48))).wrapping_add(24); (c.m.r32(a_) as i32) }; let a7_ = (({ let a_ = ((c.m.r32(sym::gTeamSel)).wrapping_add(2280)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add((((l_j_3) as u32)).wrapping_mul(48))).wrapping_add(32); F::BattleSetup_SetMember(c, l_i_2, l_j_3, a2_, a3_, 0i32, l_cpuLevel_6, F32(0x42c80000), a7_) };
                                }
                                { let o_ = l_j_3; let n_ = (o_).wrapping_add(1); l_j_3 = n_; };
                            }
                        }
                    }
                    { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
                }
            }
            F::BattleSetup_Finish(c);
        }
        F::TeamSel_Term(c);
        F::Dma_ResetBuffers(c);
        break 'f (c.m.r32(fp) as i32);
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "TeamSel_Run", &PROBE_N, &[], true); }
    r_
}
