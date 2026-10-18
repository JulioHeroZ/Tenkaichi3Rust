#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn UbTeamSel_SumCost(c: &mut Ctx, mut p_skipCur_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "UbTeamSel_SumCost", &PROBE_N, &[(p_skipCur_1) as u32], false); }
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
    if (((({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(492); (c.m.r32(a_) as i32) }) & (32i32))) != 0) {
        { let v_ = 0i32; let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(304); c.m.w32(a_, (v_) as u32); };
        {
            l_i_2 = 0i32;
            'l1: loop {
                if !(((((l_i_2) < ({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(308); (c.m.r32(a_) as i32) })) as i32)) != 0) { break 'l1; }
                'c2: {
                    if (((((((((l_i_2) != ({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) })) as i32)) != 0) || ((((!((p_skipCur_1) != 0)) as i32)) != 0)) as i32)) != 0) {
                        { let r_ = { let a0_ = { let a_ = (({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add((((l_i_2) as u32)).wrapping_mul(48))).wrapping_add(28); (c.m.r32(a_) as i32) }; F::ChrTbl_GetCost(c, a0_) }; let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(304); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                    }
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "UbTeamSel_SumCost", &PROBE_N, &[], true); }
    r_
}

pub fn UbTeamSel_IsCharaFree(c: &mut Ctx, mut p_chara_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "UbTeamSel_IsCharaFree", &PROBE_N, &[(p_chara_1) as u32], false); }
    let mut l_ok_2: i32 = 0;
    let mut l_i_3: i32 = 0;
    let r_ = 'f: {
    l_ok_2 = 1i32;
    {
        l_i_3 = 0i32;
        'l1: loop {
            if !(((((l_i_3) < ({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(308); (c.m.r32(a_) as i32) })) as i32)) != 0) { break 'l1; }
            'c2: {
                if (((((l_i_3) != ({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                    if (({ let a1_ = { let a_ = (({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add((((l_i_3) as u32)).wrapping_mul(48))).wrapping_add(28); (c.m.r32(a_) as i32) }; F::ChrTbl_IsRelated(c, p_chara_1, a1_) }) != 0) {
                        l_ok_2 = 0i32;
                    }
                }
            }
            { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
        }
    }
    break 'f l_ok_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "UbTeamSel_IsCharaFree", &PROBE_N, &[], true); }
    r_
}

pub fn UbTeamSel_FitsDp(c: &mut Ctx, mut p_chara_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "UbTeamSel_FitsDp", &PROBE_N, &[(p_chara_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32(sym::gUbTeamSel);
    if ((((!(((((c.m.r32((l_w_2).wrapping_add(492)) as i32)) & (32i32))) != 0)) as i32)) != 0) {
        break 'f 1i32;
    }
    if (((((((((p_chara_1) == (161i32)) as i32)) != 0) || (((((p_chara_1) == (163i32)) as i32)) != 0)) as i32)) != 0) {
        break 'f 1i32;
    }
    break 'f (((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(304); (c.m.r32(a_) as i32) }).wrapping_add(F::ChrTbl_GetCost(c, p_chara_1))) <= ((c.m.r32((l_w_2).wrapping_add(6860)) as i32))) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "UbTeamSel_FitsDp", &PROBE_N, &[], true); }
    r_
}

pub fn UbTeamSel_RemoveMember(c: &mut Ctx, mut p_idx_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "UbTeamSel_RemoveMember", &PROBE_N, &[(p_idx_1) as u32], false); }
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
    { let _ = { let a0_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add((((p_idx_1) as u32)).wrapping_mul(48)); F::memset(c, a0_, 0i32, 48u32) }; };
    { let v_ = (1i32).wrapping_neg(); let a_ = (({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add((((p_idx_1) as u32)).wrapping_mul(48))).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
    if (((((p_idx_1) < ({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(308); (c.m.r32(a_) as i32) })) as i32)) != 0) {
        {
            l_i_2 = p_idx_1;
            'l1: loop {
                if !(((((l_i_2) < (({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(308); (c.m.r32(a_) as i32) }).wrapping_sub(1i32))) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let s_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(((((l_i_2).wrapping_add(1i32)) as u32)).wrapping_mul(48)); let d_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add((((l_i_2) as u32)).wrapping_mul(48)); rt::memcpy(c, d_, s_, 48); };
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        { let _ = { let a0_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add((((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(308); (c.m.r32(a_) as i32) }).wrapping_sub(1i32)) as u32)).wrapping_mul(48)); F::memset(c, a0_, 0i32, 48u32) }; };
        { let v_ = (1i32).wrapping_neg(); let a_ = (({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add((((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(308); (c.m.r32(a_) as i32) }).wrapping_sub(1i32)) as u32)).wrapping_mul(48))).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "UbTeamSel_RemoveMember", &PROBE_N, &[], true); }
    r_
}

pub fn UbTeamSel_SetChips(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "UbTeamSel_SetChips", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_i_1: i32 = 0;
    let mut l_id_2: i32 = 0;
    let mut l_res_3: u32 = 0;
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
            l_i_1 = 0i32;
            'l1: loop {
                if !(((((l_i_1) < (7i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_id_2 = { let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(892); c.m.r32(a_) }).wrapping_add(((((({ let a_ = (({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(7i32)).wrapping_add(l_i_1)) as u32)).wrapping_mul(36)); (c.m.r32(a_) as i32) };
                    { let v_ = { let a_ = ((({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(240)).wrapping_add((((0i32) as u32)).wrapping_mul(28))).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ((({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(240)).wrapping_add((((1i32) as u32)).wrapping_mul(28))).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    { let v_ = l_id_2; let a_ = ((({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(240)).wrapping_add((((0i32) as u32)).wrapping_mul(28))).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    l_res_3 = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(16); c.m.r32(a_) }).wrapping_add(((((l_id_2).wrapping_add(1i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32));
                    { let v_ = { let a_ = ((c.m.r32(sym::gUbTeamSel)).wrapping_add(340)).wrapping_add(((((c.m.r32((fp).wrapping_add((((l_i_1) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) }; let a_ = ((c.m.r32(sym::gUbTeamSel)).wrapping_add(340)).wrapping_add(((((18i32).wrapping_add(l_i_1)) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = c.m.r32((l_res_3).wrapping_add(16)); let a_ = ((c.m.r32(sym::gUbTeamSel)).wrapping_add(340)).wrapping_add(((((c.m.r32((fp).wrapping_add((((l_i_1) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                }
                { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "UbTeamSel_SetChips", &PROBE_N, &[], true); }
    r_
}

pub fn UbTeamSel_SetFormChips(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "UbTeamSel_SetFormChips", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_i_1: i32 = 0;
    let mut l_id_2: i32 = 0;
    let mut l_res_3: u32 = 0;
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
            l_i_1 = 0i32;
            'l1: loop {
                if !(((((l_i_1) < (7i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    if (((((l_i_1) < ({ let a_ = (({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(892); c.m.r32(a_) }).wrapping_add(((((({ let a_ = (({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(7i32)).wrapping_add({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48)); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(36))).wrapping_add(4); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                        l_id_2 = { let a_ = ((({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(892); c.m.r32(a_) }).wrapping_add(((((({ let a_ = (({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(7i32)).wrapping_add({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48)); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(36))).wrapping_add(8)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
                    } else {
                        l_id_2 = 164i32;
                    }
                    { let v_ = { let a_ = ((({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(240)).wrapping_add((((0i32) as u32)).wrapping_mul(28))).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ((({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(240)).wrapping_add((((1i32) as u32)).wrapping_mul(28))).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    { let v_ = l_id_2; let a_ = ((({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(240)).wrapping_add((((0i32) as u32)).wrapping_mul(28))).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    l_res_3 = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(16); c.m.r32(a_) }).wrapping_add(((((l_id_2).wrapping_add(1i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32));
                    { let v_ = { let a_ = ((c.m.r32(sym::gUbTeamSel)).wrapping_add(340)).wrapping_add(((((c.m.r32((fp).wrapping_add((((l_i_1) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) }; let a_ = ((c.m.r32(sym::gUbTeamSel)).wrapping_add(340)).wrapping_add(((((18i32).wrapping_add(l_i_1)) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = c.m.r32((l_res_3).wrapping_add(16)); let a_ = ((c.m.r32(sym::gUbTeamSel)).wrapping_add(340)).wrapping_add(((((c.m.r32((fp).wrapping_add((((l_i_1) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                }
                { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "UbTeamSel_SetFormChips", &PROBE_N, &[], true); }
    r_
}

pub fn UbTeamSel_SetTeamTex(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "UbTeamSel_SetTeamTex", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_i_1: i32 = 0;
    let mut l_id_2: i32 = 0;
    let mut l_res_3: u32 = 0;
    let mut l_res_4: u32 = 0;
    let r_ = 'f: {
        rt::memset(c, fp, 0, 20);
        { let v_ = 1i32; c.m.w32(fp.wrapping_add(0), (v_) as u32); }
        { let v_ = 4i32; c.m.w32(fp.wrapping_add(4), (v_) as u32); }
        { let v_ = 5i32; c.m.w32(fp.wrapping_add(8), (v_) as u32); }
        { let v_ = 6i32; c.m.w32(fp.wrapping_add(12), (v_) as u32); }
        { let v_ = 7i32; c.m.w32(fp.wrapping_add(16), (v_) as u32); }
        if (((({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(492); (c.m.r32(a_) as i32) }) & (32i32))) != 0) {
            { let v_ = 0i32; let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(304); c.m.w32(a_, (v_) as u32); };
        }
        {
            l_i_1 = 0i32;
            'l1: loop {
                if !(((((l_i_1) < (5i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gUbTeamSel)).wrapping_add(440)).wrapping_add(((((c.m.r32((fp).wrapping_add((((l_i_1) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    if (((((l_i_1) < ({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(308); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                        l_id_2 = { let a_ = (({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add((((l_i_1) as u32)).wrapping_mul(48))).wrapping_add(28); (c.m.r32(a_) as i32) };
                        l_res_3 = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(16); c.m.r32(a_) }).wrapping_add(((((l_id_2).wrapping_add(1i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32));
                        { let v_ = c.m.r32((l_res_3).wrapping_add(16)); let a_ = ((c.m.r32(sym::gUbTeamSel)).wrapping_add(440)).wrapping_add(((((c.m.r32((fp).wrapping_add((((l_i_1) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                        if (((({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(492); (c.m.r32(a_) as i32) }) & (32i32))) != 0) {
                            { let r_ = F::ChrTbl_GetCost(c, l_id_2); let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(304); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                        }
                    } else {
                        if (((((l_i_1) >= ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(6852); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                            l_res_4 = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(16); c.m.r32(a_) }).wrapping_add(((((164i32).wrapping_add(1i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32));
                            { let v_ = c.m.r32((l_res_4).wrapping_add(16)); let a_ = ((c.m.r32(sym::gUbTeamSel)).wrapping_add(440)).wrapping_add(((((c.m.r32((fp).wrapping_add((((l_i_1) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                        }
                    }
                }
                { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "UbTeamSel_SetTeamTex", &PROBE_N, &[], true); }
    r_
}

pub fn UbTeamSel_UpdateFaceLoad(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "UbTeamSel_UpdateFaceLoad", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        's1: {
            let sv1: i32 = { let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(500); (c.m.r32(a_) as i32) };
            match sv1 {
                5i32 => {
                    { let v_ = 6i32; let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(500); c.m.w32(a_, (v_) as u32); };
                    break 's1;
                }
                6i32 => {
                    if (((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(296); (c.m.r32(a_) as i32) }) & (1i32))) != 0) {
                        { let r_ = 1i32; let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(296); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
                    }
                    { let v_ = 1i32; let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(500); c.m.w32(a_, (v_) as u32); };
                    break 's1;
                }
                1i32 => {
                    F::File_CancelRequests(c);
                    if ((((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(312); (c.m.r32(a_) as i32) }) < (0i32)) as i32)) != 0) {
                        { let r_ = 1024i32; let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(296); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                        { let r_ = 2i32; let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(296); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                        { let v_ = 4i32; let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(500); c.m.w32(a_, (v_) as u32); };
                    } else {
                        { let _ = { let a0_ = ({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(312); (c.m.r32(a_) as i32) }).wrapping_add(761i32); let a1_ = { let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(8); c.m.r32(a_) }; let _ = 92160i32; F::File_Request(c, a0_, a1_) }; };
                        { let v_ = 2i32; let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(500); c.m.w32(a_, (v_) as u32); };
                    }
                    break 's1;
                }
                2i32 => {
                    if ((F::File_UpdateRequests(c)) != 0) {
                        { let v_ = 3i32; let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(500); c.m.w32(a_, (v_) as u32); };
                    }
                    break 's1;
                }
                3i32 => {
                    { let _ = { let a0_ = { let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(8); c.m.r32(a_) }; let a1_ = { let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(12); c.m.r32(a_) }; F::Sprite_Unpack(c, a0_, a1_, ((0i32) as u32)) }; };
                    { let v_ = { let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(12); c.m.r32(a_) }; let a_ = fp; c.m.w32(a_, v_); };
                    { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbTeamSel)).wrapping_add(216)).wrapping_add((((17i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let r_ = 2i32; let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(296); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    { let v_ = 4i32; let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(500); c.m.w32(a_, (v_) as u32); };
                    break 's1;
                }
                4i32 => {
                    if (((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(296); (c.m.r32(a_) as i32) }) & (1i32))) != 0) {
                        { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gUbTeamSel)).wrapping_add(216)).wrapping_add((((17i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                        { let v_ = 6i32; let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(500); c.m.w32(a_, (v_) as u32); };
                    }
                    break 's1;
                }
                _ => {
                }
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "UbTeamSel_UpdateFaceLoad", &PROBE_N, &[], true); }
    r_
}

pub fn UbTeamSel_RequestFace(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "UbTeamSel_RequestFace", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if (((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(296); (c.m.r32(a_) as i32) }) & (2i32))) != 0) {
        { let r_ = 2i32; let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(296); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
    } else {
        { let v_ = 5i32; let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(500); c.m.w32(a_, (v_) as u32); };
    }
    if (((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(296); (c.m.r32(a_) as i32) }) & (1024i32))) != 0) {
        { let r_ = 1024i32; let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(296); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
    }
    { let r_ = 1i32; let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(296); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "UbTeamSel_RequestFace", &PROBE_N, &[], true); }
    r_
}

pub fn UbTeamSel_ClipGoto(c: &mut Ctx, mut p_movie_1: i32, mut p_kind_2: i32, mut p_label_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "UbTeamSel_ClipGoto", &PROBE_N, &[(p_movie_1) as u32, (p_kind_2) as u32, (p_label_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(80) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 80);
    let mut l_flash_4: u32 = 0;
    let r_ = 'f: {
        l_flash_4 = ((c.m.r32(sym::gUbTeamSel)).wrapping_add(36)).wrapping_add((((p_movie_1) as u32)).wrapping_mul(44));
        's1: {
            let sv5: i32 = p_kind_2;
            match sv5 {
                0i32 => {
                    { let _ = { let a2_ = { let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48)); (c.m.r32(a_) as i32) }; F::sprintf(c, fp.wrapping_add(8), sym::menu_m_b__str0, &[((a2_) as i64 as u64)]) }; };
                    break 's1;
                }
                1i32 => {
                    { let _ = { let a2_ = { let a_ = (({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(8); (c.m.r32(a_) as i32) }; F::sprintf(c, fp.wrapping_add(8), sym::menu_m_b__str1, &[((a2_) as i64 as u64)]) }; };
                    break 's1;
                }
                2i32 => {
                    { let _ = { let a2_ = ({ let a_ = (({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(20); (c.m.r32(a_) as i32) }).wrapping_add(1i32); F::sprintf(c, fp.wrapping_add(8), sym::menu_m_b__str2, &[((a2_) as i64 as u64)]) }; };
                    break 's1;
                }
                5i32 => {
                    { let _ = { let a2_ = ({ let a_ = (({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(24); (c.m.r32(a_) as i32) }).wrapping_add(1i32); F::sprintf(c, fp.wrapping_add(8), sym::menu_m_b__str3, &[((a2_) as i64 as u64)]) }; };
                    break 's1;
                }
                6i32 => {
                    's2: {
                        let sv6: i32 = { let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) };
                        match sv6 {
                            5i32 => {
                                { let _ = F::sprintf(c, fp.wrapping_add(8), sym::menu_m_b__str4, &[]); };
                                break 's2;
                            }
                            _ => {
                                { let _ = { let a2_ = { let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }; F::sprintf(c, fp.wrapping_add(8), sym::menu_m_b__str5, &[((a2_) as i64 as u64)]) }; };
                                break 's2;
                            }
                        }
                    }
                    break 's1;
                }
                3i32 => {
                    break 'f;
                }
                _ => {
                    break 's1;
                }
            }
        }
        F::Flash_FindLabel(c, l_flash_4, ((0i32) as u32), fp.wrapping_add(8), fp);
        F::Flash_ClipGotoLabel(c, l_flash_4, fp, p_label_3);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "UbTeamSel_ClipGoto", &PROBE_N, &[], true); }
    r_
}

pub fn UbTeamSel_Init(c: &mut Ctx, mut p_section_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "UbTeamSel_Init", &PROBE_N, &[(p_section_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_i_2: i32 = 0;
    let mut l_j_3: i32 = 0;
    let mut l_cols_4: i32 = 0;
    let mut l_cols2_5: i32 = 0;
    let r_ = 'f: {
        { let v_ = ((0i32) as u32); c.m.w32(fp, v_); }
        { let v_ = F::Heap_Alloc(c, ((7176u32) as i32), ((32i32) as u32), 0i32, 2i32); let a_ = sym::gUbTeamSel; c.m.w32(a_, v_); };
        { let _ = { let a0_ = c.m.r32(sym::gUbTeamSel); F::memset(c, a0_, 0i32, 7176u32) }; };
        { let v_ = (c.m.r32(sym::gMenuArc3)).wrapping_add((({ let a_ = (c.m.r32(sym::gMenuArc3)).wrapping_add((((p_section_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = c.m.r32(sym::gUbTeamSel); c.m.w32(a_, v_); };
        { let v_ = { let a0_ = { let a_ = c.m.r32(sym::gUbTeamSel); c.m.r32(a_) }; F::Sprite_Unpack(c, a0_, ((0i32) as u32), ((0i32) as u32)) }; let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(4); c.m.w32(a_, v_); };
        's1: {
            let sv6: i32 = { let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1608); (c.m.r32(a_) as i32) };
            match sv6 {
                0i32 => {
                    break 's1;
                }
                1i32 => {
                    { let r_ = 32i32; let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(492); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    { let v_ = ({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1608); (c.m.r32(a_) as i32) }).wrapping_sub(1i32); let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(6856); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 10i32; let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(6860); c.m.w32(a_, (v_) as u32); };
                    break 's1;
                }
                2i32 => {
                    { let r_ = 32i32; let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(492); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    { let v_ = ({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1608); (c.m.r32(a_) as i32) }).wrapping_sub(1i32); let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(6856); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 15i32; let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(6860); c.m.w32(a_, (v_) as u32); };
                    break 's1;
                }
                3i32 => {
                    { let r_ = 32i32; let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(492); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    { let v_ = ({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1608); (c.m.r32(a_) as i32) }).wrapping_sub(1i32); let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(6856); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 20i32; let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(6860); c.m.w32(a_, (v_) as u32); };
                    break 's1;
                }
                _ => {
                }
            }
        }
        { let v_ = { let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1604); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(6852); c.m.w32(a_, (v_) as u32); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((32i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = c.m.r32(fp); let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(212); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((33i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbTeamSel)).wrapping_add(216)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbTeamSel)).wrapping_add(216)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbTeamSel)).wrapping_add(216)).wrapping_add((((11i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbTeamSel)).wrapping_add(216)).wrapping_add((((12i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((4i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbTeamSel)).wrapping_add(216)).wrapping_add((((13i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbTeamSel)).wrapping_add(216)).wrapping_add((((8i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbTeamSel)).wrapping_add(216)).wrapping_add((((9i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbTeamSel)).wrapping_add(216)).wrapping_add((((18i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((34i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbTeamSel)).wrapping_add(216)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbTeamSel)).wrapping_add(216)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbTeamSel)).wrapping_add(216)).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((5i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbTeamSel)).wrapping_add(216)).wrapping_add((((14i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((6i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbTeamSel)).wrapping_add(216)).wrapping_add((((15i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((8i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbTeamSel)).wrapping_add(216)).wrapping_add((((16i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        if (((({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(492); (c.m.r32(a_) as i32) }) & (32i32))) != 0) {
            { let v_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
            { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
            { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbTeamSel)).wrapping_add(216)).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
            { let v_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
            { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
            { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbTeamSel)).wrapping_add(216)).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
            { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbTeamSel)).wrapping_add(216)).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        }
        { let a0_ = ((c.m.r32(sym::gUbTeamSel)).wrapping_add(36)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); let a1_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((31i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a2_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(216); F::Flash_Create(c, a0_, a1_, a2_) };
        { let a0_ = ((c.m.r32(sym::gUbTeamSel)).wrapping_add(36)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_Play(c, a0_, 1i32) };
        if (((({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(492); (c.m.r32(a_) as i32) }) & (32i32))) != 0) {
            { let v_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
            { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
            { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbTeamSel)).wrapping_add(340)).wrapping_add((((9i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
            { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbTeamSel)).wrapping_add(440)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        }
        { let v_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbTeamSel)).wrapping_add(340)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbTeamSel)).wrapping_add(340)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbTeamSel)).wrapping_add(340)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbTeamSel)).wrapping_add(340)).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((4i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbTeamSel)).wrapping_add(340)).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbTeamSel)).wrapping_add(340)).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((8i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbTeamSel)).wrapping_add(340)).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((9i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbTeamSel)).wrapping_add(340)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((10i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbTeamSel)).wrapping_add(340)).wrapping_add((((10i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbTeamSel)).wrapping_add(340)).wrapping_add((((17i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbTeamSel)).wrapping_add(296)).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbTeamSel)).wrapping_add(440)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbTeamSel)).wrapping_add(440)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let a0_ = ((c.m.r32(sym::gUbTeamSel)).wrapping_add(36)).wrapping_add((((2i32) as u32)).wrapping_mul(44)); let a1_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((11i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a2_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(340); F::Flash_Create(c, a0_, a1_, a2_) };
        { let a0_ = ((c.m.r32(sym::gUbTeamSel)).wrapping_add(36)).wrapping_add((((2i32) as u32)).wrapping_mul(44)); F::Flash_Play(c, a0_, 1i32) };
        { let v_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((12i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbTeamSel)).wrapping_add(296)).wrapping_add((((9i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbTeamSel)).wrapping_add(296)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((13i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbTeamSel)).wrapping_add(296)).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbTeamSel)).wrapping_add(296)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((14i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbTeamSel)).wrapping_add(296)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbTeamSel)).wrapping_add(296)).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((15i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbTeamSel)).wrapping_add(296)).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbTeamSel)).wrapping_add(296)).wrapping_add((((8i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((16i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbTeamSel)).wrapping_add(296)).wrapping_add((((10i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbTeamSel)).wrapping_add(296)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let a0_ = ((c.m.r32(sym::gUbTeamSel)).wrapping_add(36)).wrapping_add((((3i32) as u32)).wrapping_mul(44)); let a1_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((17i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a2_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(296); F::Flash_Create(c, a0_, a1_, a2_) };
        { let a0_ = ((c.m.r32(sym::gUbTeamSel)).wrapping_add(36)).wrapping_add((((3i32) as u32)).wrapping_mul(44)); F::Flash_Play(c, a0_, 1i32) };
        { let v_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((18i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbTeamSel)).wrapping_add(440)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((19i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbTeamSel)).wrapping_add(440)).wrapping_add((((8i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbTeamSel)).wrapping_add(440)).wrapping_add((((11i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbTeamSel)).wrapping_add(440)).wrapping_add((((10i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((20i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbTeamSel)).wrapping_add(440)).wrapping_add((((9i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbTeamSel)).wrapping_add(440)).wrapping_add((((12i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let a0_ = ((c.m.r32(sym::gUbTeamSel)).wrapping_add(36)).wrapping_add((((1i32) as u32)).wrapping_mul(44)); let a1_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((21i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a2_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(440); F::Flash_Create(c, a0_, a1_, a2_) };
        { let a0_ = ((c.m.r32(sym::gUbTeamSel)).wrapping_add(36)).wrapping_add((((1i32) as u32)).wrapping_mul(44)); F::Flash_Play(c, a0_, 1i32) };
        { let a0_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((22i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); F::ItemPanel_Init(c, a0_, 0i32) };
        { let v_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((25i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(28); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((26i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(32); c.m.w32(a_, v_); };
        { let a0_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((23i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a1_ = { let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(28); c.m.r32(a_) }; let _ = 0i32; F::MsgWin_Init(c, a0_, a1_, 1i32) };
        F::MsgWin_Open(c);
        { let a0_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((24i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); F::ItemHelp_Init(c, a0_) };
        { let v_ = ({ let a_ = ((c.m.r32(sym::gCommonRes)).wrapping_add(4)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = ((c.m.r32(sym::gCommonRes)).wrapping_add(4)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(7172); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((28i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(20); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((29i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(24); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((27i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(16); c.m.w32(a_, v_); };
        {
            l_i_2 = 0i32;
            'l2: loop {
                if !(((((l_i_2) < (165i32)) as i32)) != 0) { break 'l2; }
                'c3: {
                    { let v_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(16); c.m.r32(a_) }).wrapping_add(((((l_i_2).wrapping_add(1i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
                    { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        { let v_ = (({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((30i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32))).wrapping_add(((16i32) as u32)); let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(892); c.m.w32(a_, v_); };
        { let v_ = (({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((30i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) as i32); let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(888); c.m.w32(a_, (v_) as u32); };
        { let a0_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(896); let a1_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(900); let a2_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(888); let a3_ = { let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(892); c.m.r32(a_) }; F::ChrGrid_Build(c, a0_, a1_, a2_, a3_, ((0i32) as u32), ((0i32) as u32)) };
        { let v_ = { let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(896); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(888); c.m.w32(a_, (v_) as u32); };
        { let v_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(900); let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(892); c.m.w32(a_, v_); };
        {
            l_cols_4 = 7i32;
            l_cols2_5 = 7i32;
            { let v_ = rt::div_i32({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(888); (c.m.r32(a_) as i32) }, l_cols_4); let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(6840); c.m.w32(a_, (v_) as u32); };
            if ((rt::rem_i32({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(888); (c.m.r32(a_) as i32) }, l_cols2_5)) != 0) {
                { let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(6840); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
            }
        }
        { let v_ = F::Heap_Alloc(c, 92160i32, ((64i32) as u32), 0i32, 2i32); let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(8); c.m.w32(a_, v_); };
        { let v_ = F::Heap_Alloc(c, 133120i32, ((32i32) as u32), 0i32, 2i32); let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(12); c.m.w32(a_, v_); };
        { let v_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(512); let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.w32(a_, v_); };
        { let s_ = (c.m.r32(sym::gProgress)).wrapping_add(1088); let d_ = { let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }; rt::memcpy(c, d_, s_, 240); };
        { let v_ = 4i32; let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(500); c.m.w32(a_, (v_) as u32); };
        {
            l_j_3 = 0i32;
            'l4: loop {
                if !(((((l_j_3) < (5i32)) as i32)) != 0) { break 'l4; }
                'c5: {
                    if ((((!(({ let a0_ = { let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(892); c.m.r32(a_) }; let a1_ = ({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add((((l_j_3) as u32)).wrapping_mul(48)); (c.m.r32(a_) as i32) }).wrapping_add(({ let a_ = (({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add((((l_j_3) as u32)).wrapping_mul(48))).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(7i32)); F::ChrGrid_IsSelectable(c, a0_, a1_) }) != 0)) as i32)) != 0) {
                        { let _ = { let a0_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add((((l_j_3) as u32)).wrapping_mul(48)); F::memset(c, a0_, 0i32, 48u32) }; };
                        { let v_ = 0i32; let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add((((l_j_3) as u32)).wrapping_mul(48)); c.m.w32(a_, (v_) as u32); };
                        { let s_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add((((l_j_3) as u32)).wrapping_mul(48)); let d_ = ((c.m.r32(sym::gProgress)).wrapping_add(1088)).wrapping_add((((l_j_3) as u32)).wrapping_mul(48)); rt::memcpy(c, d_, s_, 48); };
                    }
                }
                { let o_ = l_j_3; let n_ = (o_).wrapping_add(1); l_j_3 = n_; };
            }
        }
        F::UbTeamSel_SetChips(c);
        { let v_ = { let a_ = ((({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(240)).wrapping_add((((0i32) as u32)).wrapping_mul(28))).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(312); c.m.w32(a_, (v_) as u32); };
        {
            l_j_3 = 0i32;
            'l6: loop {
                if !(((((l_j_3) < (5i32)) as i32)) != 0) { break 'l6; }
                'c7: {
                    { let v_ = (1i32).wrapping_neg(); let a_ = (({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add((((l_j_3) as u32)).wrapping_mul(48))).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                }
                { let o_ = l_j_3; let n_ = (o_).wrapping_add(1); l_j_3 = n_; };
            }
        }
        {
            l_i_2 = 0i32;
            'l8: loop {
                if !(((((l_i_2) < (2i32)) as i32)) != 0) { break 'l8; }
                'c9: {
                    { let v_ = ((F::Rand_Range(c, ((32i32) as u32))) as i32); let a_ = ((c.m.r32(sym::gUbTeamSel)).wrapping_add(6876)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        { let v_ = (1i32).wrapping_neg(); let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(496); c.m.w32(a_, (v_) as u32); };
        { let _ = { let a0_ = ({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(312); (c.m.r32(a_) as i32) }).wrapping_add(761i32); let a1_ = { let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(8); c.m.r32(a_) }; F::File_LoadSync(c, a0_, a1_, 92160i32) }; };
        { let _ = { let a0_ = { let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(8); c.m.r32(a_) }; let a1_ = { let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(12); c.m.r32(a_) }; F::Sprite_Unpack(c, a0_, a1_, ((0i32) as u32)) }; };
        { let v_ = { let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(12); c.m.r32(a_) }; let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbTeamSel)).wrapping_add(216)).wrapping_add((((17i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let r_ = 2i32; let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(296); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        { let a0_ = ((c.m.r32(sym::gUbTeamSel)).wrapping_add(6892)).wrapping_add((((0i32) as u32)).wrapping_mul(140)); let a1_ = { let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(20); c.m.r32(a_) }; F::TextBox_Init(c, a0_, a1_, ((1i32) as u32)) };
        { let a0_ = ((c.m.r32(sym::gUbTeamSel)).wrapping_add(6892)).wrapping_add((((0i32) as u32)).wrapping_mul(140)); F::TextBox_SetUnk80(c, a0_, 1i32) };
        { let a0_ = ((c.m.r32(sym::gUbTeamSel)).wrapping_add(6892)).wrapping_add((((1i32) as u32)).wrapping_mul(140)); let a1_ = { let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(24); c.m.r32(a_) }; F::TextBox_Init(c, a0_, a1_, ((3i32) as u32)) };
        { let a0_ = ((c.m.r32(sym::gUbTeamSel)).wrapping_add(6892)).wrapping_add((((1i32) as u32)).wrapping_mul(140)); F::TextBox_SetUnk80(c, a0_, 1i32) };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "UbTeamSel_Init", &PROBE_N, &[], true); }
    r_
}

pub fn UbTeamSel_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "UbTeamSel_Term", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let r_ = 'f: {
    F::ItemHelp_Term(c);
    F::MsgWin_Term(c);
    F::ItemPanel_Term(c, 0i32);
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (4i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let a0_ = ((c.m.r32(sym::gUbTeamSel)).wrapping_add(36)).wrapping_add((((l_i_1) as u32)).wrapping_mul(44)); F::Flash_Destroy(c, a0_) };
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
    if ((((({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(12); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = { let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(12); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(12); c.m.w32(a_, v_); };
    }
    if ((((({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(8); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = { let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(8); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(8); c.m.w32(a_, v_); };
    }
    if ((((({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(4); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = { let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(4); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(4); c.m.w32(a_, v_); };
    }
    if (((((c.m.r32(sym::gUbTeamSel)) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32(sym::gUbTeamSel); F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = sym::gUbTeamSel; c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "UbTeamSel_Term", &PROBE_N, &[], true); }
    r_
}

pub fn UbTeamSel_Draw(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "UbTeamSel_Draw", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(160) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 160);
    let mut l_i_1: i32 = 0;
    let mut l_flash_2: u32 = 0;
    let mut l_chara_3: i32 = 0;
    let mut l_cost_4: i32 = 0;
    let mut l_cols_5: i32 = 0;
    let r_ = 'f: {
        { let a0_ = { let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(212); c.m.r32(a_) }; F::Sprite_DrawPicture(c, a0_, 0i32, 0i32, 128i32) };
        l_flash_2 = ((c.m.r32(sym::gUbTeamSel)).wrapping_add(36)).wrapping_add((((0i32) as u32)).wrapping_mul(44));
        {
            l_i_1 = 0i32;
            'l1: loop {
                if !(((((l_i_1) < (2i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let _ = F::sprintf(c, fp.wrapping_add(28), sym::menu_m_b__str6, &[(((l_i_1).wrapping_add(17i32)) as i64 as u64)]); };
                    { let _ = F::sprintf(c, fp.wrapping_add(92), sym::menu_m_b__str7, &[(((l_i_1).wrapping_add(17i32)) as i64 as u64)]); };
                    F::Flash_FindLabel(c, l_flash_2, fp.wrapping_add(28), fp.wrapping_add(92), fp);
                    { let a2_ = ((c.m.r32(sym::gUbTeamSel)).wrapping_add(6876)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); F::FlashAnim_Blink(c, l_flash_2, fp, a2_, 0i32) };
                    { let _ = F::sprintf(c, fp.wrapping_add(92), sym::menu_m_b__str8, &[(((l_i_1).wrapping_add(17i32)) as i64 as u64)]); };
                    F::Flash_FindLabel(c, l_flash_2, fp.wrapping_add(28), fp.wrapping_add(92), fp);
                    if ((((({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(6872); (c.m.r32(a_) as i32) }) == (l_i_1)) as i32)) != 0) {
                        { let a2_ = ((c.m.r32(sym::gUbTeamSel)).wrapping_add(6884)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); F::FlashAnim_Talk(c, l_flash_2, fp, a2_, 0i32) };
                    } else {
                        F::FlashAnim_ShowNext2(c, l_flash_2, fp, 0i32);
                    }
                }
                { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
            }
        }
        if ((((((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(296); (c.m.r32(a_) as i32) }) & (((1024i32) | (2i32))))) == (2i32)) as i32)) != 0) {
            { let r_ = F32(0x3d999999); let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(6864); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
            if (((rt::fge({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(6864); F32(c.m.r32(a_)) }, F32(0x3f800000)) as i32)) != 0) {
                { let v_ = F32(0x3f800000); let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(6864); c.m.w32(a_, (v_).0); };
            }
        } else {
            { let v_ = F32(0x00000000); let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(6864); c.m.w32(a_, (v_).0); };
        }
        F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), sym::menu_m_b__str9, fp);
        { let a2_ = { let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(6864); F32(c.m.r32(a_)) }; F::Flash_ClipSetAlpha(c, l_flash_2, fp, a2_) };
        F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), sym::menu_m_b__str10, fp);
        { let a4_ = { let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(312); (c.m.r32(a_) as i32) }; let a5_ = ((c.m.r32(sym::gUbTeamSel)).wrapping_add(6892)).wrapping_add((((0i32) as u32)).wrapping_mul(140)); F::TextBox_AttachLine(c, l_flash_2, fp, 0i32, 0i32, a4_, a5_) };
        F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), sym::menu_m_b__str11, fp);
        { let a4_ = { let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(312); (c.m.r32(a_) as i32) }; let a5_ = ((c.m.r32(sym::gUbTeamSel)).wrapping_add(6892)).wrapping_add((((1i32) as u32)).wrapping_mul(140)); F::TextBox_AttachLine(c, l_flash_2, fp, 0i32, 0i32, a4_, a5_) };
        if (((({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(492); (c.m.r32(a_) as i32) }) & (32i32))) != 0) {
            { let v_ = (rt::rem_i32({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(6856); (c.m.r32(a_) as i32) }, 2i32)).wrapping_mul(64i32); let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
            { let v_ = (rt::div_i32({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(6856); (c.m.r32(a_) as i32) }, 2i32)).wrapping_mul(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
            { let v_ = ((c.m.r32(fp.wrapping_add(8)) as i32)).wrapping_add(64i32); let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
            { let v_ = ((c.m.r32((fp.wrapping_add(8)).wrapping_add(4)) as i32)).wrapping_add(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
            F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), sym::menu_m_b__str12, fp);
            F::Flash_ClipSetUv(c, l_flash_2, fp, fp.wrapping_add(8));
            { let a4_ = { let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(304); (c.m.r32(a_) as i32) }; F::Num_Draw(c, l_flash_2, sym::menu_m_b__str13, 0i32, 2i32, a4_, 32i32, 64i32, 1i32) };
        }
        l_flash_2 = ((c.m.r32(sym::gUbTeamSel)).wrapping_add(36)).wrapping_add((((3i32) as u32)).wrapping_mul(44));
        { let v_ = 32i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 64i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = 32i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), sym::menu_m_b__str14, fp);
        F::Flash_ClipSetUv(c, l_flash_2, fp, fp.wrapping_add(8));
        {
            l_i_1 = 0i32;
            'l3: loop {
                if !(((((l_i_1) < (4i32)) as i32)) != 0) { break 'l3; }
                'c4: {
                    { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 256i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = (l_i_1).wrapping_mul(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    { let v_ = ((c.m.r32((fp.wrapping_add(8)).wrapping_add(4)) as i32)).wrapping_add(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                    { let _ = F::sprintf(c, fp.wrapping_add(28), sym::menu_m_b__str15, &[(((l_i_1).wrapping_add(1i32)) as i64 as u64)]); };
                    F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), fp.wrapping_add(28), fp);
                    { let a2_ = (if (((((l_i_1) < ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(6844); (c.m.r32(a_) as i32) })) as i32)) != 0) { F32(0x3f800000) } else { F32(0x3e999999) }); F::Flash_ClipSetColor(c, l_flash_2, fp, a2_) };
                    F::Flash_FindLabel(c, l_flash_2, fp.wrapping_add(28), sym::menu_m_b__str16, fp);
                    F::Flash_ClipSetUv(c, l_flash_2, fp, fp.wrapping_add(8));
                    F::Flash_FindLabel(c, l_flash_2, fp.wrapping_add(28), sym::menu_m_b__str17, fp);
                    F::Flash_ClipSetUv(c, l_flash_2, fp, fp.wrapping_add(8));
                    if (((((l_i_1) == (0i32)) as i32)) != 0) {
                        F::Flash_FindLabel(c, l_flash_2, fp.wrapping_add(28), sym::menu_m_b__str18, fp);
                        { let _ = F::Flash_ClipSetFlags(c, l_flash_2, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
                    }
                }
                { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
            }
        }
        {
            l_i_1 = 0i32;
            'l5: loop {
                if !(((((l_i_1) < (4i32)) as i32)) != 0) { break 'l5; }
                'c6: {
                    { let v_ = (rt::rem_i32(l_i_1, 2i32)).wrapping_mul(64i32); let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = ((c.m.r32(fp.wrapping_add(8)) as i32)).wrapping_add(64i32); let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = (rt::div_i32(l_i_1, 2i32)).wrapping_mul(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    { let v_ = ((c.m.r32((fp.wrapping_add(8)).wrapping_add(4)) as i32)).wrapping_add(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                    { let _ = F::sprintf(c, fp.wrapping_add(28), sym::menu_m_b__str19, &[(((l_i_1).wrapping_add(1i32)) as i64 as u64)]); };
                    F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), fp.wrapping_add(28), fp);
                    if (((((l_i_1) < ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(6848); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                        { let _ = F::Flash_ClipSetFlags(c, l_flash_2, fp, 2i32, ((((1i32) as i64 as u64)) as u8)); };
                        's7: {
                            let sv6: i32 = { let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(6848); (c.m.r32(a_) as i32) };
                            match sv6 {
                                2i32 => {
                                    F::Flash_ClipSetOffset(c, l_flash_2, fp, 45i32, 0i32);
                                    break 's7;
                                }
                                3i32 => {
                                    F::Flash_ClipSetOffset(c, l_flash_2, fp, 22i32, 0i32);
                                    break 's7;
                                }
                                _ => {
                                }
                            }
                        }
                        F::Flash_FindLabel(c, l_flash_2, fp.wrapping_add(28), sym::menu_m_b__str20, fp);
                        F::Flash_ClipSetUv(c, l_flash_2, fp, fp.wrapping_add(8));
                        F::Flash_FindLabel(c, l_flash_2, fp.wrapping_add(28), sym::menu_m_b__str21, fp);
                        F::Flash_ClipSetUv(c, l_flash_2, fp, fp.wrapping_add(8));
                    } else {
                        { let _ = F::Flash_ClipSetFlags(c, l_flash_2, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
                    }
                }
                { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
            }
        }
        l_flash_2 = ((c.m.r32(sym::gUbTeamSel)).wrapping_add(36)).wrapping_add((((2i32) as u32)).wrapping_mul(44));
        { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 64i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = 32i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), sym::menu_m_b__str22, fp);
        F::Flash_ClipSetUv(c, l_flash_2, fp, fp.wrapping_add(8));
        if ((((({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(6840); (c.m.r32(a_) as i32) }) >= (2i32)) as i32)) != 0) {
            {
                l_i_1 = 0i32;
                'l8: loop {
                    if !(((((l_i_1) < (2i32)) as i32)) != 0) { break 'l8; }
                    'c9: {
                        { let v_ = (l_i_1).wrapping_mul(32i32); let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                        { let v_ = 32i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                        { let v_ = ((c.m.r32(fp.wrapping_add(8)) as i32)).wrapping_add(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                        { let v_ = 64i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                        F::Flash_FindLabel(c, l_flash_2, (if ((l_i_1) != 0) { sym::menu_m_b__str23 } else { sym::menu_m_b__str24 }), (if ((l_i_1) != 0) { sym::menu_m_b__str25 } else { sym::menu_m_b__str26 }), fp);
                        F::Flash_ClipSetUv(c, l_flash_2, fp, fp.wrapping_add(8));
                        F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), (if ((l_i_1) != 0) { sym::menu_m_b__str27 } else { sym::menu_m_b__str28 }), fp);
                        if (((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(296); (c.m.r32(a_) as i32) }) & (64i32))) != 0) {
                            { let _ = F::Flash_ClipSetFlags(c, l_flash_2, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
                        } else {
                            { let _ = F::Flash_ClipSetFlags(c, l_flash_2, fp, 2i32, ((((1i32) as i64 as u64)) as u8)); };
                        }
                    }
                    { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
                }
            }
        } else {
            {
                l_i_1 = 0i32;
                'l10: loop {
                    if !(((((l_i_1) < (2i32)) as i32)) != 0) { break 'l10; }
                    'c11: {
                        F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), (if ((l_i_1) != 0) { sym::menu_m_b__str29 } else { sym::menu_m_b__str30 }), fp);
                        { let _ = F::Flash_ClipSetFlags(c, l_flash_2, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
                    }
                    { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
                }
            }
        }
        F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), sym::menu_m_b__str31, fp);
        if ((((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(320); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
            { let _ = F::Flash_ClipSetFlags(c, l_flash_2, fp, 258i32, ((((1i32) as i64 as u64)) as u8)); };
        } else {
            { let _ = F::Flash_ClipSetFlags(c, l_flash_2, fp, 258i32, ((((0i32) as i64 as u64)) as u8)); };
        }
        {
            l_i_1 = 0i32;
            'l12: loop {
                if !(((((l_i_1) < (14i32)) as i32)) != 0) { break 'l12; }
                'c13: {
                    { let _ = F::sprintf(c, fp.wrapping_add(28), sym::menu_m_b__str32, &[((l_i_1) as i64 as u64)]); };
                    F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), fp.wrapping_add(28), fp);
                    { let a3_ = (((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(320); (c.m.r32(a_) as i32) }) as u8)) as i32); { let _ = F::Flash_ClipSetFlags(c, l_flash_2, fp, 128i32, ((((a3_) as i64 as u64)) as u8)); } };
                    {
                        l_cols_5 = 7i32;
                        l_chara_3 = { let a_ = ((({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(240)).wrapping_add((((rt::div_i32(l_i_1, l_cols_5)) as u32)).wrapping_mul(28))).wrapping_add((((rt::rem_i32(l_i_1, l_cols_5)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
                    }
                    if (((({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(492); (c.m.r32(a_) as i32) }) & (32i32))) != 0) {
                        if (((((l_chara_3) < (161i32)) as i32)) != 0) {
                            l_cost_4 = F::ChrTbl_GetCost(c, l_chara_3);
                        } else {
                            l_cost_4 = 0i32;
                        }
                        { let _ = F::sprintf(c, fp.wrapping_add(92), sym::menu_m_b__str33, &[((l_i_1) as i64 as u64)]); };
                        F::Flash_FindLabel(c, l_flash_2, fp.wrapping_add(28), fp.wrapping_add(92), fp);
                        if (((((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(304); (c.m.r32(a_) as i32) }).wrapping_add(l_cost_4)) > ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(6860); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                            F::Flash_ClipSetColor(c, l_flash_2, fp, F32(0x3ecccccc));
                        } else {
                            if (((((l_chara_3) < (161i32)) as i32)) != 0) {
                                { let a2_ = (if ((F::UbTeamSel_IsCharaFree(c, l_chara_3)) != 0) { F32(0x3f800000) } else { F32(0x3ecccccc) }); F::Flash_ClipSetColor(c, l_flash_2, fp, a2_) };
                            } else {
                                F::Flash_ClipSetColor(c, l_flash_2, fp, F32(0x3f800000));
                            }
                        }
                        { let _ = F::sprintf(c, fp.wrapping_add(92), sym::menu_m_b__str34, &[((l_i_1) as i64 as u64)]); };
                        F::Flash_FindLabel(c, l_flash_2, fp.wrapping_add(28), fp.wrapping_add(92), fp);
                        if (((((l_chara_3) < (161i32)) as i32)) != 0) {
                            { let v_ = (rt::rem_i32(l_cost_4, 4i32)).wrapping_mul(32i32); let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                            { let v_ = ((c.m.r32(fp.wrapping_add(8)) as i32)).wrapping_add(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                            { let v_ = (rt::div_i32(l_cost_4, 4i32)).wrapping_mul(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                            { let v_ = ((c.m.r32((fp.wrapping_add(8)).wrapping_add(4)) as i32)).wrapping_add(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                            F::Flash_ClipSetUv(c, l_flash_2, fp, fp.wrapping_add(8));
                            { let _ = F::Flash_ClipSetFlags(c, l_flash_2, fp, 2i32, ((((1i32) as i64 as u64)) as u8)); };
                        } else {
                            { let _ = F::Flash_ClipSetFlags(c, l_flash_2, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
                        }
                    } else {
                        { let _ = F::sprintf(c, fp.wrapping_add(92), sym::menu_m_b__str35, &[((l_i_1) as i64 as u64)]); };
                        F::Flash_FindLabel(c, l_flash_2, fp.wrapping_add(28), fp.wrapping_add(92), fp);
                        if (((((l_chara_3) < (161i32)) as i32)) != 0) {
                            { let a2_ = (if ((F::UbTeamSel_IsCharaFree(c, l_chara_3)) != 0) { F32(0x3f800000) } else { F32(0x3ecccccc) }); F::Flash_ClipSetColor(c, l_flash_2, fp, a2_) };
                        } else {
                            F::Flash_ClipSetColor(c, l_flash_2, fp, F32(0x3f800000));
                        }
                    }
                }
                { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
            }
        }
        l_flash_2 = ((c.m.r32(sym::gUbTeamSel)).wrapping_add(36)).wrapping_add((((1i32) as u32)).wrapping_mul(44));
        {
            l_i_1 = 0i32;
            'l14: loop {
                if !(((((l_i_1) < (5i32)) as i32)) != 0) { break 'l14; }
                'c15: {
                    { let _ = F::sprintf(c, fp.wrapping_add(28), sym::menu_m_b__str36, &[((l_i_1) as i64 as u64)]); };
                    { let v_ = (rt::rem_i32(l_i_1, 4i32)).wrapping_mul(64i32); let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = ((c.m.r32(fp.wrapping_add(8)) as i32)).wrapping_add(64i32); let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = (rt::div_i32(l_i_1, 4i32)).wrapping_mul(72i32); let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    { let v_ = ((c.m.r32((fp.wrapping_add(8)).wrapping_add(4)) as i32)).wrapping_add(72i32); let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                    { let _ = F::sprintf(c, fp.wrapping_add(92), sym::menu_m_b__str37, &[((l_i_1) as i64 as u64)]); };
                    F::Flash_FindLabel(c, l_flash_2, fp.wrapping_add(28), fp.wrapping_add(92), fp);
                    F::Flash_ClipSetUv(c, l_flash_2, fp, fp.wrapping_add(8));
                    { let _ = F::sprintf(c, fp.wrapping_add(92), sym::menu_m_b__str38, &[((l_i_1) as i64 as u64)]); };
                    F::Flash_FindLabel(c, l_flash_2, fp.wrapping_add(28), fp.wrapping_add(92), fp);
                    if (((((l_i_1) < ({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(308); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                        l_cost_4 = { let a0_ = { let a_ = (({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add((((l_i_1) as u32)).wrapping_mul(48))).wrapping_add(28); (c.m.r32(a_) as i32) }; F::ChrTbl_GetCost(c, a0_) };
                        { let v_ = (rt::rem_i32(l_cost_4, 4i32)).wrapping_mul(32i32); let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                        { let v_ = ((c.m.r32(fp.wrapping_add(8)) as i32)).wrapping_add(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                        { let v_ = (rt::div_i32(l_cost_4, 4i32)).wrapping_mul(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                        { let v_ = ((c.m.r32((fp.wrapping_add(8)).wrapping_add(4)) as i32)).wrapping_add(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                        { let _ = F::Flash_ClipSetFlags(c, l_flash_2, fp, 2i32, ((((1i32) as i64 as u64)) as u8)); };
                        F::Flash_ClipSetUv(c, l_flash_2, fp, fp.wrapping_add(8));
                    } else {
                        { let _ = F::Flash_ClipSetFlags(c, l_flash_2, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
                    }
                }
                { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
            }
        }
        { let a0_ = ((c.m.r32(sym::gUbTeamSel)).wrapping_add(36)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_Draw(c, a0_) };
        { let a0_ = ((c.m.r32(sym::gUbTeamSel)).wrapping_add(36)).wrapping_add((((1i32) as u32)).wrapping_mul(44)); F::Flash_Draw(c, a0_) };
        { let a0_ = ((c.m.r32(sym::gUbTeamSel)).wrapping_add(36)).wrapping_add((((2i32) as u32)).wrapping_mul(44)); F::Flash_Draw(c, a0_) };
        F::Font_FlushAll(c);
        { let a0_ = ((c.m.r32(sym::gUbTeamSel)).wrapping_add(36)).wrapping_add((((3i32) as u32)).wrapping_mul(44)); F::Flash_Draw(c, a0_) };
        { let a2_ = { let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(496); (c.m.r32(a_) as i32) }; F::MsgWin_Draw(c, 0i32, 0i32, a2_) };
        F::ItemPanel_Draw(c, 0i32);
        { let a0_ = { let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(508); (c.m.r32(a_) as i32) }; F::ItemHelp_Draw(c, a0_) };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "UbTeamSel_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn UbTeamSel_Update(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "UbTeamSel_Update", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let r_ = 'f: {
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (4i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let a0_ = ((c.m.r32(sym::gUbTeamSel)).wrapping_add(36)).wrapping_add((((l_i_1) as u32)).wrapping_mul(44)); F::Flash_Advance(c, a0_) };
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
    F::ItemPanel_Update(c, 0i32);
    if ((((((((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(320); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) && (((({ let a_ = (((c.m.r32(sym::gUbTeamSel)).wrapping_add(36)).wrapping_add((((2i32) as u32)).wrapping_mul(44))).wrapping_add(12); c.m.r32(a_) }) & (((1i32) as u32)))) != 0)) as i32)) != 0) {
        { let v_ = 0i32; let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(320); c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "UbTeamSel_Update", &PROBE_N, &[], true); }
    r_
}

pub fn UbTeamSel_Input(c: &mut Ctx, mut p_result_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "UbTeamSel_Input", &PROBE_N, &[(p_result_1) as u32], false); }
    let mut l_item_2: i32 = 0;
    let r_ = 'f: {
    if ((((!(((({ let a_ = (((c.m.r32(sym::gUbTeamSel)).wrapping_add(36)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add(8); c.m.r32(a_) }) & (((2i32) as u32)))) != 0)) as i32)) != 0) {
        break 'f;
    }
    if ((((!(((({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(492); (c.m.r32(a_) as i32) }) & (2i32))) != 0)) as i32)) != 0) {
        F::UbTeamSel_ClipGoto(c, 2i32, 0i32, sym::menu_m_b__str39);
        { let r_ = 2i32; let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(492); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
    's1: {
        let sv3: i32 = { let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(316); (c.m.r32(a_) as i32) };
        match sv3 {
            0i32 => {
                if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((1i32) as u32)))) != 0) {
                    F::UbTeamSel_ClipGoto(c, 2i32, 0i32, sym::menu_m_b__str40);
                    { let a0_ = { let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(892); c.m.r32(a_) }; let a1_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48)); let a2_ = { let a_ = (({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(4); (c.m.r32(a_) as i32) }; { let _ = F::ChrGrid_MoveLeft(c, a0_, a1_, a2_); } };
                    F::UbTeamSel_ClipGoto(c, 2i32, 0i32, sym::menu_m_b__str41);
                    { let v_ = { let a_ = ((({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(240)).wrapping_add((((0i32) as u32)).wrapping_mul(28))).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(312); c.m.w32(a_, (v_) as u32); };
                    F::UbTeamSel_RequestFace(c);
                    { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 0i32); };
                } else {
                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((2i32) as u32)))) != 0) {
                        F::UbTeamSel_ClipGoto(c, 2i32, 0i32, sym::menu_m_b__str42);
                        { let a0_ = { let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(892); c.m.r32(a_) }; let a1_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48)); let a2_ = { let a_ = (({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(4); (c.m.r32(a_) as i32) }; { let _ = F::ChrGrid_MoveRight(c, a0_, a1_, a2_); } };
                        F::UbTeamSel_ClipGoto(c, 2i32, 0i32, sym::menu_m_b__str43);
                        { let v_ = { let a_ = ((({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(240)).wrapping_add((((0i32) as u32)).wrapping_mul(28))).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(312); c.m.w32(a_, (v_) as u32); };
                        F::UbTeamSel_RequestFace(c);
                        { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 0i32); };
                    } else {
                        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((8i32) as u32)))) != 0) {
                            if ((((({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(6840); (c.m.r32(a_) as i32) }) >= (2i32)) as i32)) != 0) {
                                { let a0_ = ((c.m.r32(sym::gUbTeamSel)).wrapping_add(36)).wrapping_add((((2i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_m_b__str44, ((((1i32) as i64 as u64)) as u8)) };
                                { let v_ = 1i32; let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(320); c.m.w32(a_, (v_) as u32); };
                                F::UbTeamSel_ClipGoto(c, 2i32, 0i32, sym::menu_m_b__str45);
                                { let a0_ = { let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(892); c.m.r32(a_) }; let a1_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48)); let a2_ = (({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(4); let a3_ = { let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(6840); (c.m.r32(a_) as i32) }; F::ChrGrid_MoveUp(c, a0_, a1_, a2_, a3_) };
                                F::UbTeamSel_ClipGoto(c, 2i32, 0i32, sym::menu_m_b__str46);
                                F::UbTeamSel_SetChips(c);
                                { let v_ = { let a_ = ((({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(240)).wrapping_add((((0i32) as u32)).wrapping_mul(28))).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(312); c.m.w32(a_, (v_) as u32); };
                                F::UbTeamSel_RequestFace(c);
                                { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 2i32); };
                            }
                        } else {
                            if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((4i32) as u32)))) != 0) {
                                if ((((({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(6840); (c.m.r32(a_) as i32) }) >= (2i32)) as i32)) != 0) {
                                    { let a0_ = ((c.m.r32(sym::gUbTeamSel)).wrapping_add(36)).wrapping_add((((2i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_m_b__str47, ((((1i32) as i64 as u64)) as u8)) };
                                    { let v_ = 1i32; let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(320); c.m.w32(a_, (v_) as u32); };
                                    F::UbTeamSel_ClipGoto(c, 2i32, 0i32, sym::menu_m_b__str48);
                                    { let a0_ = { let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(892); c.m.r32(a_) }; let a1_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48)); let a2_ = (({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(4); let a3_ = { let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(6840); (c.m.r32(a_) as i32) }; F::ChrGrid_MoveDown(c, a0_, a1_, a2_, a3_) };
                                    F::UbTeamSel_ClipGoto(c, 2i32, 0i32, sym::menu_m_b__str49);
                                    F::UbTeamSel_SetChips(c);
                                    { let v_ = { let a_ = ((({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(240)).wrapping_add((((0i32) as u32)).wrapping_mul(28))).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(312); c.m.w32(a_, (v_) as u32); };
                                    F::UbTeamSel_RequestFace(c);
                                    { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 1i32); };
                                }
                            } else {
                                if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                                    if (({ let a0_ = { let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(312); (c.m.r32(a_) as i32) }; F::UbTeamSel_FitsDp(c, a0_) }) != 0) {
                                        if ((((({ let a_ = (({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(892); c.m.r32(a_) }).wrapping_add(((((({ let a_ = (({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(7i32)).wrapping_add({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48)); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(36))).wrapping_add(4); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
                                            { let r_ = 64i32; let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(296); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                                            if ((((({ let a_ = (({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(8); (c.m.r32(a_) as i32) }) >= ({ let a_ = (({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(892); c.m.r32(a_) }).wrapping_add(((((({ let a_ = (({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(7i32)).wrapping_add({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48)); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(36))).wrapping_add(4); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                                                { let v_ = 0i32; let a_ = (({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                                            }
                                            { let a0_ = ((c.m.r32(sym::gUbTeamSel)).wrapping_add(36)).wrapping_add((((2i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_m_b__str50, ((((1i32) as i64 as u64)) as u8)) };
                                            { let v_ = 1i32; let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(320); c.m.w32(a_, (v_) as u32); };
                                            F::UbTeamSel_ClipGoto(c, 2i32, 0i32, sym::menu_m_b__str51);
                                            F::UbTeamSel_ClipGoto(c, 2i32, 1i32, sym::menu_m_b__str52);
                                            F::UbTeamSel_SetFormChips(c);
                                            { let v_ = { let a_ = ((({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(240)).wrapping_add((((0i32) as u32)).wrapping_mul(28))).wrapping_add(((({ let a_ = (({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(8); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(312); c.m.w32(a_, (v_) as u32); };
                                            F::UbTeamSel_RequestFace(c);
                                            { let v_ = 1i32; let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(316); c.m.w32(a_, (v_) as u32); };
                                            { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 41i32); };
                                        } else {
                                            if (({ let a0_ = { let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(312); (c.m.r32(a_) as i32) }; F::UbTeamSel_IsCharaFree(c, a0_) }) != 0) {
                                                { let a0_ = ((c.m.r32(sym::gUbTeamSel)).wrapping_add(36)).wrapping_add((((3i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_m_b__str53, ((((1i32) as i64 as u64)) as u8)) };
                                                F::UbTeamSel_ClipGoto(c, 2i32, 0i32, sym::menu_m_b__str54);
                                                { let v_ = (if (((({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(492); (c.m.r32(a_) as i32) }) & (64i32))) != 0) { 1i32 } else { 4i32 }); let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(6844); c.m.w32(a_, (v_) as u32); };
                                                if ((((({ let a_ = (({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(20); (c.m.r32(a_) as i32) }) >= ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(6844); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                                                    { let v_ = 0i32; let a_ = (({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
                                                }
                                                F::UbTeamSel_ClipGoto(c, 3i32, 2i32, sym::menu_m_b__str55);
                                                { let v_ = { let a0_ = { let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(312); (c.m.r32(a_) as i32) }; let a1_ = (({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(24); F::ChrTbl_WrapCostume(c, a0_, a1_) }; let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(6848); c.m.w32(a_, (v_) as u32); };
                                                { let v_ = 2i32; let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(316); c.m.w32(a_, (v_) as u32); };
                                                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                                            } else {
                                                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 7i32); };
                                            }
                                        }
                                    } else {
                                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 7i32); };
                                    }
                                } else {
                                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1024i32) as u32)))) != 0) {
                                        { let v_ = 0i32; let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(320); c.m.w32(a_, (v_) as u32); };
                                        if ((((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(308); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
                                            { let a0_ = ((c.m.r32(sym::gUbTeamSel)).wrapping_add(36)).wrapping_add((((2i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_m_b__str56, ((((1i32) as i64 as u64)) as u8)) };
                                            { let a0_ = ((c.m.r32(sym::gUbTeamSel)).wrapping_add(36)).wrapping_add((((1i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_m_b__str57, ((((1i32) as i64 as u64)) as u8)) };
                                            F::UbTeamSel_ClipGoto(c, 2i32, 0i32, sym::menu_m_b__str58);
                                            F::UbTeamSel_ClipGoto(c, 1i32, 6i32, sym::menu_m_b__str59);
                                            { let s_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(840); let d_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48)); rt::memcpy(c, d_, s_, 48); };
                                            { let v_ = { let a_ = (({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(28); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(312); c.m.w32(a_, (v_) as u32); };
                                            F::UbTeamSel_RequestFace(c);
                                            F::UbTeamSel_SumCost(c, 0i32);
                                            { let v_ = 6i32; let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(316); c.m.w32(a_, (v_) as u32); };
                                        } else {
                                            F::ColorFade_StartOut(c, 0i32, 0i32, 0i32, 20i32);
                                            { let v_ = 0i32; let a_ = p_result_1; c.m.w32(a_, (v_) as u32); };
                                        }
                                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 2i32); };
                                    }
                                }
                            }
                        }
                    }
                }
                break 's1;
            }
            1i32 => {
                if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((1i32) as u32)))) != 0) {
                    F::UbTeamSel_ClipGoto(c, 2i32, 1i32, sym::menu_m_b__str60);
                    { let a0_ = (({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(240)).wrapping_add((((0i32) as u32)).wrapping_mul(28)); let a1_ = (({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(8); F::ChrGrid_PrevForm(c, a0_, a1_) };
                    F::UbTeamSel_ClipGoto(c, 2i32, 1i32, sym::menu_m_b__str61);
                    { let v_ = { let a_ = ((({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(240)).wrapping_add((((0i32) as u32)).wrapping_mul(28))).wrapping_add(((({ let a_ = (({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(8); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(312); c.m.w32(a_, (v_) as u32); };
                    F::UbTeamSel_RequestFace(c);
                    { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 0i32); };
                } else {
                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((2i32) as u32)))) != 0) {
                        F::UbTeamSel_ClipGoto(c, 2i32, 1i32, sym::menu_m_b__str62);
                        { let a0_ = (({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(240)).wrapping_add((((0i32) as u32)).wrapping_mul(28)); let a1_ = (({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(8); F::ChrGrid_NextForm(c, a0_, a1_) };
                        F::UbTeamSel_ClipGoto(c, 2i32, 1i32, sym::menu_m_b__str63);
                        { let v_ = { let a_ = ((({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(240)).wrapping_add((((0i32) as u32)).wrapping_mul(28))).wrapping_add(((({ let a_ = (({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(8); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(312); c.m.w32(a_, (v_) as u32); };
                        F::UbTeamSel_RequestFace(c);
                        { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 0i32); };
                    } else {
                        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                            if (({ let a0_ = { let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(312); (c.m.r32(a_) as i32) }; F::UbTeamSel_FitsDp(c, a0_) }) != 0) {
                                if (({ let a0_ = { let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(312); (c.m.r32(a_) as i32) }; F::UbTeamSel_IsCharaFree(c, a0_) }) != 0) {
                                    { let a0_ = ((c.m.r32(sym::gUbTeamSel)).wrapping_add(36)).wrapping_add((((3i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_m_b__str64, ((((1i32) as i64 as u64)) as u8)) };
                                    F::UbTeamSel_ClipGoto(c, 2i32, 1i32, sym::menu_m_b__str65);
                                    { let v_ = (if (((({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(492); (c.m.r32(a_) as i32) }) & (64i32))) != 0) { 1i32 } else { 4i32 }); let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(6844); c.m.w32(a_, (v_) as u32); };
                                    if ((((({ let a_ = (({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(20); (c.m.r32(a_) as i32) }) >= ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(6844); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                                        { let v_ = 0i32; let a_ = (({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
                                    }
                                    F::UbTeamSel_ClipGoto(c, 3i32, 2i32, sym::menu_m_b__str66);
                                    { let v_ = { let a0_ = { let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(312); (c.m.r32(a_) as i32) }; let a1_ = (({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(24); F::ChrTbl_WrapCostume(c, a0_, a1_) }; let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(6848); c.m.w32(a_, (v_) as u32); };
                                    { let v_ = 2i32; let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(316); c.m.w32(a_, (v_) as u32); };
                                    { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                                } else {
                                    { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 7i32); };
                                }
                            } else {
                                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 7i32); };
                            }
                        } else {
                            if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1024i32) as u32)))) != 0) {
                                { let r_ = 64i32; let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(296); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
                                { let a0_ = ((c.m.r32(sym::gUbTeamSel)).wrapping_add(36)).wrapping_add((((2i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_m_b__str67, ((((1i32) as i64 as u64)) as u8)) };
                                { let v_ = 1i32; let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(320); c.m.w32(a_, (v_) as u32); };
                                F::UbTeamSel_ClipGoto(c, 2i32, 1i32, sym::menu_m_b__str68);
                                F::UbTeamSel_ClipGoto(c, 2i32, 0i32, sym::menu_m_b__str69);
                                F::UbTeamSel_SetChips(c);
                                { let v_ = { let a_ = ((({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(240)).wrapping_add((((0i32) as u32)).wrapping_mul(28))).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(312); c.m.w32(a_, (v_) as u32); };
                                F::UbTeamSel_RequestFace(c);
                                { let v_ = 0i32; let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(316); c.m.w32(a_, (v_) as u32); };
                                { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 41i32); };
                            }
                        }
                    }
                }
                break 's1;
            }
            2i32 => {
                if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((8i32) as u32)))) != 0) {
                    if ((((({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(6844); (c.m.r32(a_) as i32) }) >= (2i32)) as i32)) != 0) {
                        F::UbTeamSel_ClipGoto(c, 3i32, 2i32, sym::menu_m_b__str70);
                        { let a_ = (({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(20); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                        if ((((({ let a_ = (({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(20); (c.m.r32(a_) as i32) }) < (0i32)) as i32)) != 0) {
                            { let v_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(6844); (c.m.r32(a_) as i32) }).wrapping_sub(1i32); let a_ = (({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
                        }
                        F::UbTeamSel_ClipGoto(c, 3i32, 2i32, sym::menu_m_b__str71);
                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                    }
                } else {
                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((4i32) as u32)))) != 0) {
                        if ((((({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(6844); (c.m.r32(a_) as i32) }) >= (2i32)) as i32)) != 0) {
                            F::UbTeamSel_ClipGoto(c, 3i32, 2i32, sym::menu_m_b__str72);
                            { let a_ = (({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(20); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                            if ((((({ let a_ = (({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(20); (c.m.r32(a_) as i32) }) >= ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(6844); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                                { let v_ = 0i32; let a_ = (({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
                            }
                            F::UbTeamSel_ClipGoto(c, 3i32, 2i32, sym::menu_m_b__str73);
                            { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                        }
                    } else {
                        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((2i32) as u32)))) != 0) {
                            if ((((({ let a_ = (({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(20); (c.m.r32(a_) as i32) }) > (0i32)) as i32)) != 0) {
                                F::UbTeamSel_ClipGoto(c, 3i32, 2i32, sym::menu_m_b__str74);
                                { let a1_ = { let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(312); (c.m.r32(a_) as i32) }; let a2_ = ({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48)); (c.m.r32(a_) as i32) }).wrapping_add(({ let a_ = (({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(7i32)); let a3_ = ({ let a_ = (({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(20); (c.m.r32(a_) as i32) }).wrapping_sub(1i32); F::ItemPanel_SetChara(c, 0i32, a1_, a2_, a3_, 0i32) };
                                F::ItemPanel_Show(c, 0i32);
                                { let r_ = 2048i32; let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(296); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                                { let v_ = 3i32; let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(316); c.m.w32(a_, (v_) as u32); };
                                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                            }
                        } else {
                            if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                                { let a0_ = ((c.m.r32(sym::gUbTeamSel)).wrapping_add(36)).wrapping_add((((3i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_m_b__str75, ((((1i32) as i64 as u64)) as u8)) };
                                F::UbTeamSel_ClipGoto(c, 3i32, 2i32, sym::menu_m_b__str76);
                                F::UbTeamSel_ClipGoto(c, 3i32, 5i32, sym::menu_m_b__str77);
                                { let v_ = 5i32; let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(316); c.m.w32(a_, (v_) as u32); };
                                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                            } else {
                                if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1024i32) as u32)))) != 0) {
                                    { let a0_ = ((c.m.r32(sym::gUbTeamSel)).wrapping_add(36)).wrapping_add((((3i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_m_b__str78, ((((1i32) as i64 as u64)) as u8)) };
                                    F::UbTeamSel_ClipGoto(c, 3i32, 2i32, sym::menu_m_b__str79);
                                    if (((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(296); (c.m.r32(a_) as i32) }) & (64i32))) != 0) {
                                        F::UbTeamSel_ClipGoto(c, 2i32, 1i32, sym::menu_m_b__str80);
                                        { let v_ = 1i32; let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(316); c.m.w32(a_, (v_) as u32); };
                                    } else {
                                        F::UbTeamSel_ClipGoto(c, 2i32, 0i32, sym::menu_m_b__str81);
                                        { let v_ = 0i32; let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(316); c.m.w32(a_, (v_) as u32); };
                                    }
                                    { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 2i32); };
                                }
                            }
                        }
                    }
                }
                break 's1;
            }
            3i32 => {
                {
                    l_item_2 = F::ItemPanel_Input(c, 0i32, 0i32);
                    if (((((l_item_2) > (0i32)) as i32)) != 0) {
                        F::ItemHelp_Open(c);
                        { let v_ = (l_item_2).wrapping_sub(1i32); let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(508); c.m.w32(a_, (v_) as u32); };
                        { let v_ = 4i32; let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(316); c.m.w32(a_, (v_) as u32); };
                    } else {
                        if (((((l_item_2) < (0i32)) as i32)) != 0) {
                            F::ItemPanel_Hide(c, 0i32);
                            F::UbTeamSel_ClipGoto(c, 3i32, 2i32, sym::menu_m_b__str82);
                            { let v_ = 2i32; let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(316); c.m.w32(a_, (v_) as u32); };
                        }
                    }
                    break 's1;
                }
            }
            4i32 => {
                if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1536i32) as u32)))) != 0) {
                    F::ItemHelp_Close(c);
                    { let v_ = 3i32; let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(316); c.m.w32(a_, (v_) as u32); };
                    { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 2i32); };
                }
                break 's1;
            }
            5i32 => {
                if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((1i32) as u32)))) != 0) {
                    F::UbTeamSel_ClipGoto(c, 3i32, 5i32, sym::menu_m_b__str83);
                    { let a_ = (({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(24); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                    if ((((({ let a_ = (({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(24); (c.m.r32(a_) as i32) }) < (0i32)) as i32)) != 0) {
                        { let v_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(6848); (c.m.r32(a_) as i32) }).wrapping_sub(1i32); let a_ = (({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                    }
                    F::UbTeamSel_ClipGoto(c, 3i32, 5i32, sym::menu_m_b__str84);
                    { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                } else {
                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((2i32) as u32)))) != 0) {
                        F::UbTeamSel_ClipGoto(c, 3i32, 5i32, sym::menu_m_b__str85);
                        { let a_ = (({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(24); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        if ((((({ let a_ = (({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(24); (c.m.r32(a_) as i32) }) >= ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(6848); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                            { let v_ = 0i32; let a_ = (({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                        }
                        F::UbTeamSel_ClipGoto(c, 3i32, 5i32, sym::menu_m_b__str86);
                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                    } else {
                        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                            { let v_ = 0i32; let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(320); c.m.w32(a_, (v_) as u32); };
                            { let a0_ = ((c.m.r32(sym::gUbTeamSel)).wrapping_add(36)).wrapping_add((((3i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_m_b__str87, ((((1i32) as i64 as u64)) as u8)) };
                            { let a0_ = ((c.m.r32(sym::gUbTeamSel)).wrapping_add(36)).wrapping_add((((2i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_m_b__str88, ((((1i32) as i64 as u64)) as u8)) };
                            { let a0_ = ((c.m.r32(sym::gUbTeamSel)).wrapping_add(36)).wrapping_add((((1i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_m_b__str89, ((((1i32) as i64 as u64)) as u8)) };
                            F::UbTeamSel_ClipGoto(c, 3i32, 5i32, sym::menu_m_b__str90);
                            F::UbTeamSel_ClipGoto(c, 3i32, 2i32, sym::menu_m_b__str91);
                            if (((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(296); (c.m.r32(a_) as i32) }) & (64i32))) != 0) {
                                { let v_ = { let a_ = ((({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(240)).wrapping_add((((0i32) as u32)).wrapping_mul(28))).wrapping_add(((({ let a_ = (({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(8); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                                if ((((({ let a_ = (({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(20); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
                                    { let s_ = (((c.m.r32(sym::gSaveData)).wrapping_add(6152)).wrapping_add((((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48)); (c.m.r32(a_) as i32) }).wrapping_add(({ let a_ = (({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(7i32))) as u32)).wrapping_mul(56))).wrapping_add((((({ let a_ = (({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(20); (c.m.r32(a_) as i32) }).wrapping_sub(1i32)) as u32)).wrapping_mul(16)); let d_ = (({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(32); rt::memcpy(c, d_, s_, 16); };
                                } else {
                                    { let _ = { let a0_ = (({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(32); F::memset(c, a0_, 0i32, 16u32) }; };
                                }
                                F::UbTeamSel_ClipGoto(c, 2i32, 1i32, sym::menu_m_b__str92);
                            } else {
                                { let v_ = { let a_ = ((({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(240)).wrapping_add((((0i32) as u32)).wrapping_mul(28))).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                                if ((((({ let a_ = (({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(20); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
                                    { let s_ = (((c.m.r32(sym::gSaveData)).wrapping_add(6152)).wrapping_add((((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48)); (c.m.r32(a_) as i32) }).wrapping_add(({ let a_ = (({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(7i32))) as u32)).wrapping_mul(56))).wrapping_add((((({ let a_ = (({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(20); (c.m.r32(a_) as i32) }).wrapping_sub(1i32)) as u32)).wrapping_mul(16)); let d_ = (({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(32); rt::memcpy(c, d_, s_, 16); };
                                } else {
                                    { let _ = { let a0_ = (({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(32); F::memset(c, a0_, 0i32, 16u32) }; };
                                }
                                F::UbTeamSel_ClipGoto(c, 2i32, 0i32, sym::menu_m_b__str93);
                            }
                            F::UbTeamSel_ClipGoto(c, 1i32, 6i32, sym::menu_m_b__str94);
                            if ((((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) == ({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(308); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                                { let v_ = ({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }).wrapping_add(1i32); let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(308); c.m.w32(a_, (v_) as u32); };
                                { let v_ = { let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(308); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(300); c.m.w32(a_, (v_) as u32); };
                                if ((((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) >= ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(6852); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                                    { let v_ = 5i32; let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(300); c.m.w32(a_, (v_) as u32); };
                                }
                            }
                            F::UbTeamSel_SetTeamTex(c);
                            F::UbTeamSel_ClipGoto(c, 1i32, 6i32, sym::menu_m_b__str95);
                            if ((((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) == (5i32)) as i32)) != 0) {
                                { let v_ = { let a_ = (({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add((((0i32) as u32)).wrapping_mul(48))).wrapping_add(28); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(312); c.m.w32(a_, (v_) as u32); };
                                F::UbTeamSel_RequestFace(c);
                            }
                            if (((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(296); (c.m.r32(a_) as i32) }) & (64i32))) != 0) {
                                { let r_ = 64i32; let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(296); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
                            }
                            if (((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(296); (c.m.r32(a_) as i32) }) & (128i32))) != 0) {
                                { let r_ = 128i32; let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(296); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
                            }
                            { let v_ = 6i32; let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(316); c.m.w32(a_, (v_) as u32); };
                            { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                        } else {
                            if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1024i32) as u32)))) != 0) {
                                { let a0_ = ((c.m.r32(sym::gUbTeamSel)).wrapping_add(36)).wrapping_add((((3i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_m_b__str96, ((((1i32) as i64 as u64)) as u8)) };
                                F::UbTeamSel_ClipGoto(c, 3i32, 5i32, sym::menu_m_b__str97);
                                F::UbTeamSel_ClipGoto(c, 3i32, 2i32, sym::menu_m_b__str98);
                                { let v_ = 2i32; let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(316); c.m.w32(a_, (v_) as u32); };
                                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 2i32); };
                            }
                        }
                    }
                }
                break 's1;
            }
            6i32 => {
                if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((1i32) as u32)))) != 0) {
                    F::UbTeamSel_ClipGoto(c, 1i32, 6i32, sym::menu_m_b__str99);
                    { let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(300); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                    if ((((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) < (0i32)) as i32)) != 0) {
                        { let v_ = 5i32; let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(300); c.m.w32(a_, (v_) as u32); };
                    } else {
                        if ((((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) > ({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(308); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                            { let v_ = { let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(308); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(300); c.m.w32(a_, (v_) as u32); };
                            if ((((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) >= ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(6852); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                                { let v_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(6852); (c.m.r32(a_) as i32) }).wrapping_sub(1i32); let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(300); c.m.w32(a_, (v_) as u32); };
                            }
                        }
                    }
                    F::UbTeamSel_ClipGoto(c, 1i32, 6i32, sym::menu_m_b__str100);
                    if ((((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) == (5i32)) as i32)) != 0) {
                        { let v_ = { let a_ = (({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add((((0i32) as u32)).wrapping_mul(48))).wrapping_add(28); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(312); c.m.w32(a_, (v_) as u32); };
                    } else {
                        { let v_ = { let a_ = (({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(28); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(312); c.m.w32(a_, (v_) as u32); };
                    }
                    F::UbTeamSel_RequestFace(c);
                    { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                } else {
                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((2i32) as u32)))) != 0) {
                        F::UbTeamSel_ClipGoto(c, 1i32, 6i32, sym::menu_m_b__str101);
                        { let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(300); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        if ((((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) > (5i32)) as i32)) != 0) {
                            { let v_ = 0i32; let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(300); c.m.w32(a_, (v_) as u32); };
                        } else {
                            if ((((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) > ({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(308); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                                { let v_ = 5i32; let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(300); c.m.w32(a_, (v_) as u32); };
                            } else {
                                if ((((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) >= ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(6852); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                                    { let v_ = 5i32; let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(300); c.m.w32(a_, (v_) as u32); };
                                }
                            }
                        }
                        F::UbTeamSel_ClipGoto(c, 1i32, 6i32, sym::menu_m_b__str102);
                        if ((((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) == (5i32)) as i32)) != 0) {
                            { let v_ = { let a_ = (({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add((((0i32) as u32)).wrapping_mul(48))).wrapping_add(28); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(312); c.m.w32(a_, (v_) as u32); };
                        } else {
                            { let v_ = { let a_ = (({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(28); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(312); c.m.w32(a_, (v_) as u32); };
                        }
                        F::UbTeamSel_RequestFace(c);
                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                    } else {
                        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                            if ((((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) != (5i32)) as i32)) != 0) {
                                F::UbTeamSel_SumCost(c, 1i32);
                                { let a0_ = ((c.m.r32(sym::gUbTeamSel)).wrapping_add(36)).wrapping_add((((1i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_m_b__str103, ((((1i32) as i64 as u64)) as u8)) };
                                { let a0_ = ((c.m.r32(sym::gUbTeamSel)).wrapping_add(36)).wrapping_add((((2i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_m_b__str104, ((((1i32) as i64 as u64)) as u8)) };
                                F::UbTeamSel_ClipGoto(c, 1i32, 6i32, sym::menu_m_b__str105);
                                F::UbTeamSel_ClipGoto(c, 2i32, 0i32, sym::menu_m_b__str106);
                                F::UbTeamSel_SetChips(c);
                                { let v_ = { let a_ = ((({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(240)).wrapping_add((((0i32) as u32)).wrapping_mul(28))).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(312); c.m.w32(a_, (v_) as u32); };
                                F::UbTeamSel_RequestFace(c);
                                { let s_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48)); let d_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(840); rt::memcpy(c, d_, s_, 48); };
                                { let v_ = 0i32; let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(316); c.m.w32(a_, (v_) as u32); };
                                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                            } else {
                                if ((((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(308); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
                                    { let a0_ = ((c.m.r32(sym::gUbTeamSel)).wrapping_add(36)).wrapping_add((((1i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_m_b__str107, ((((1i32) as i64 as u64)) as u8)) };
                                    F::UbTeamSel_ClipGoto(c, 1i32, 6i32, sym::menu_m_b__str108);
                                    { let v_ = 1i32; let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(6868); c.m.w32(a_, (v_) as u32); };
                                    { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                                } else {
                                    { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 7i32); };
                                }
                            }
                        } else {
                            if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1024i32) as u32)))) != 0) {
                                if ((((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(308); (c.m.r32(a_) as i32) }) == (1i32)) as i32)) != 0) {
                                    { let a0_ = ((c.m.r32(sym::gUbTeamSel)).wrapping_add(36)).wrapping_add((((1i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_m_b__str109, ((((1i32) as i64 as u64)) as u8)) };
                                    { let a0_ = ((c.m.r32(sym::gUbTeamSel)).wrapping_add(36)).wrapping_add((((2i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_m_b__str110, ((((1i32) as i64 as u64)) as u8)) };
                                    F::UbTeamSel_ClipGoto(c, 1i32, 6i32, sym::menu_m_b__str111);
                                    { let v_ = 0i32; let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(300); c.m.w32(a_, (v_) as u32); };
                                    F::UbTeamSel_ClipGoto(c, 2i32, 0i32, sym::menu_m_b__str112);
                                    F::UbTeamSel_SetChips(c);
                                    { let v_ = { let a_ = ((({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(240)).wrapping_add((((0i32) as u32)).wrapping_mul(28))).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(312); c.m.w32(a_, (v_) as u32); };
                                    F::UbTeamSel_RequestFace(c);
                                    { let v_ = 0i32; let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(316); c.m.w32(a_, (v_) as u32); };
                                } else {
                                    if ((((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) == (5i32)) as i32)) != 0) {
                                        { let a0_ = ({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(308); (c.m.r32(a_) as i32) }).wrapping_sub(1i32); F::UbTeamSel_RemoveMember(c, a0_) };
                                    } else {
                                        if ((((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) == ({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(308); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                                            { let a0_ = ({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }).wrapping_sub(1i32); F::UbTeamSel_RemoveMember(c, a0_) };
                                            F::UbTeamSel_ClipGoto(c, 1i32, 6i32, sym::menu_m_b__str113);
                                            { let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(300); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                                            F::UbTeamSel_ClipGoto(c, 1i32, 6i32, sym::menu_m_b__str114);
                                            { let v_ = { let a_ = (({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(28); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(312); c.m.w32(a_, (v_) as u32); };
                                            F::UbTeamSel_RequestFace(c);
                                        } else {
                                            { let a0_ = { let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }; F::UbTeamSel_RemoveMember(c, a0_) };
                                            { let v_ = { let a_ = (({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(300); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(48))).wrapping_add(28); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(312); c.m.w32(a_, (v_) as u32); };
                                            F::UbTeamSel_RequestFace(c);
                                        }
                                    }
                                }
                                { let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(308); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                                F::UbTeamSel_SetTeamTex(c);
                                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 2i32); };
                            }
                        }
                    }
                }
                break 's1;
            }
            _ => {
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "UbTeamSel_Input", &PROBE_N, &[], true); }
    r_
}

pub fn UbTeamSel_UpdateEnd(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "UbTeamSel_UpdateEnd", &PROBE_N, &[], false); }
    let mut l_step_1: i32 = 0;
    let r_ = 'f: {
    l_step_1 = { let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(6868); (c.m.r32(a_) as i32) };
    if (((((l_step_1) == (0i32)) as i32)) != 0) {
        break 'f;
    }
    's1: {
        let sv2: i32 = l_step_1;
        match sv2 {
            1i32 => {
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(6872); c.m.w32(a_, (v_) as u32); };
                { let v_ = 55i32; let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(496); c.m.w32(a_, (v_) as u32); };
                { let a0_ = { let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(32); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(496); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34661i32, a2_) };
                { let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(6868); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            2i32 => {
                if (((((F::Voice_GetStat(c, 0i32)) == (5i32)) as i32)) != 0) {
                    { let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(6868); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                } else {
                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                        { let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(6868); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                    }
                }
                break 's1;
            }
            3i32 => {
                { let r_ = 8i32; let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(492); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let r_ = 16i32; let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(492); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 15i32; let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(504); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(6868); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            _ => {
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "UbTeamSel_UpdateEnd", &PROBE_N, &[], true); }
    r_
}

pub fn UbTeamSel_Run(c: &mut Ctx, mut p_section_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "UbTeamSel_Run", &PROBE_N, &[(p_section_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        { let v_ = 1i32; c.m.w32(fp, (v_) as u32); }
        F::UbTeamSel_Init(c, p_section_1);
        F::ColorFade_StartIn(c, 0i32, 0i32, 0i32, 20i32);
        'l1: while ((1i32) != 0) {
            F::Gfx_BeginFrame(c);
            F::UbTeamSel_UpdateFaceLoad(c);
            F::Pad_Update(c);
            F::Snd_Update(c);
            F::ColorFade_Update(c);
            if ((((!(((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(20); (c.m.r32(a_) as i32) }) & (256i32))) != 0)) as i32)) != 0) {
                F::UbTeamSel_Update(c);
                F::UbTeamSel_UpdateEnd(c);
            }
            F::UbTeamSel_Draw(c);
            F::Font_FlushAll(c);
            F::ColorFade_Draw(c);
            F::Gfx_EndFrame(c, 1i32);
            F::Dma_Flush(c);
            F::File_Stub264D90(c);
            if (((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(20); (c.m.r32(a_) as i32) }) & (256i32))) != 0) {
                continue 'l1;
            }
            if ((F::ColorFade_IsInDone(c)) != 0) {
                if ((((((((!(((({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(492); (c.m.r32(a_) as i32) }) & (128i32))) != 0)) as i32)) != 0) && (((({ let a_ = (((c.m.r32(sym::gUbTeamSel)).wrapping_add(36)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add(8); c.m.r32(a_) }) & (((2i32) as u32)))) != 0)) as i32)) != 0) {
                    { let r_ = 128i32; let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(492); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    { let v_ = 0i32; let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(6872); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 54i32; let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(496); c.m.w32(a_, (v_) as u32); };
                    { let a0_ = { let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(32); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(496); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34661i32, a2_) };
                }
            }
            if ((F::ColorFade_IsFadingOut(c)) != 0) {
                F::Voice_FadeOutStep(c, 0i32);
                F::Bgm_FadeOutStep(c);
                continue 'l1;
            }
            if ((F::ColorFade_IsOutDone(c)) != 0) {
                if ((((({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(500); (c.m.r32(a_) as i32) }) != (4i32)) as i32)) != 0) {
                    continue 'l1;
                }
                break 'l1;
            }
            if (((({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(492); (c.m.r32(a_) as i32) }) & (16i32))) != 0) {
                if ((((({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(504); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) == ((1i32).wrapping_neg())) as i32)) != 0) {
                    F::ColorFade_StartOut(c, 0i32, 0i32, 0i32, 20i32);
                    { let s_ = { let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }; let d_ = (c.m.r32(sym::gProgress)).wrapping_add(1088); rt::memcpy(c, d_, s_, 240); };
                }
            } else {
                if ((((({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(6868); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
                    F::UbTeamSel_Input(c, fp);
                }
            }
        }
        if (((c.m.r32(fp) as i32)) != 0) {
            { let v_ = { let a_ = ({ let a_ = (c.m.r32(sym::gUbTeamSel)).wrapping_add(836); c.m.r32(a_) }).wrapping_add(308); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1604); c.m.w32(a_, (v_) as u32); };
        }
        F::UbTeamSel_Term(c);
        F::Dma_ResetBuffers(c);
        break 'f (c.m.r32(fp) as i32);
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "UbTeamSel_Run", &PROBE_N, &[], true); }
    r_
}
