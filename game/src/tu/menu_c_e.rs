#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn CharSel_SetStageChips(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "CharSel_SetStageChips", &PROBE_N, &[], false); }
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
                    l_id_2 = { let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14340); c.m.r32(a_) }).wrapping_add(((((l_i_1).wrapping_add(({ let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(1364); c.m.r32(a_) }).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(6i32))) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
                    { let v_ = { let a_ = ((({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(1364); c.m.r32(a_) }).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ((({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(1364); c.m.r32(a_) }).wrapping_add(8)).wrapping_add((((1i32) as u32)).wrapping_mul(24))).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    { let v_ = l_id_2; let a_ = ((({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(1364); c.m.r32(a_) }).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(24))).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    {
                        if (((((l_id_2) == (62i32)) as i32)) != 0) {
                            l_icon_4 = 37i32;
                        } else {
                            if (((((l_id_2) == (63i32)) as i32)) != 0) {
                                l_icon_4 = 38i32;
                            } else {
                                if (((((l_id_2) >= (36i32)) as i32)) != 0) {
                                    l_icon_4 = (1i32).wrapping_add(rt::rem_i32((l_id_2).wrapping_sub(36i32), 36i32));
                                } else {
                                    l_icon_4 = (l_id_2).wrapping_add(1i32);
                                }
                            }
                        }
                        l_res_3 = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(48); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(48); c.m.r32(a_) }).wrapping_add((((l_icon_4) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32));
                    }
                    { let v_ = { let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(280)).wrapping_add(((((c.m.r32((fp).wrapping_add((((l_i_1) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) }; let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(280)).wrapping_add(((((41i32).wrapping_add(l_i_1)) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = c.m.r32((l_res_3).wrapping_add(16)); let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(280)).wrapping_add(((((c.m.r32((fp).wrapping_add((((l_i_1) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                }
                { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "CharSel_SetStageChips", &PROBE_N, &[], true); }
    r_
}

pub fn CharSel_SetRowChips(c: &mut Ctx, mut p_side_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "CharSel_SetRowChips", &PROBE_N, &[(p_side_1) as u32], false); }
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
                    l_id_3 = { let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1376)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(((((l_i_2).wrapping_add(({ let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(7i32))) as u32)).wrapping_mul(36)); (c.m.r32(a_) as i32) };
                    { let v_ = { let a_ = (({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(48)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(76)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    { let v_ = l_id_3; let a_ = (({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(48)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    l_res_4 = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(36); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(36); c.m.r32(a_) }).wrapping_add(((((l_id_3).wrapping_add(1i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32));
                    { let v_ = { let a_ = (((c.m.r32(sym::gCharSel)).wrapping_add(564)).wrapping_add((((p_side_1) as u32)).wrapping_mul(100))).wrapping_add(((((c.m.r32((fp).wrapping_add((((l_i_2) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) }; let a_ = (((c.m.r32(sym::gCharSel)).wrapping_add(564)).wrapping_add((((p_side_1) as u32)).wrapping_mul(100))).wrapping_add(((((18i32).wrapping_add(l_i_2)) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = c.m.r32((l_res_4).wrapping_add(16)); let a_ = (((c.m.r32(sym::gCharSel)).wrapping_add(564)).wrapping_add((((p_side_1) as u32)).wrapping_mul(100))).wrapping_add(((((c.m.r32((fp).wrapping_add((((l_i_2) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "CharSel_SetRowChips", &PROBE_N, &[], true); }
    r_
}

pub fn CharSel_SetCellFormChips(c: &mut Ctx, mut p_side_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "CharSel_SetCellFormChips", &PROBE_N, &[(p_side_1) as u32], false); }
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
                    if (((((l_i_2) < ({ let a_ = (({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1376)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add((((({ let a_ = { let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }; (c.m.r32(a_) as i32) }).wrapping_add(({ let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(7i32))) as u32)).wrapping_mul(36))).wrapping_add(4); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                        l_id_3 = { let a_ = ((({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1376)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add((((({ let a_ = { let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }; (c.m.r32(a_) as i32) }).wrapping_add(({ let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(7i32))) as u32)).wrapping_mul(36))).wrapping_add(8)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
                    } else {
                        l_id_3 = 164i32;
                    }
                    { let v_ = { let a_ = (({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(48)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(76)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    { let v_ = l_id_3; let a_ = (({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(48)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    l_res_4 = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(36); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(36); c.m.r32(a_) }).wrapping_add(((((l_id_3).wrapping_add(1i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32));
                    { let v_ = { let a_ = (((c.m.r32(sym::gCharSel)).wrapping_add(564)).wrapping_add((((p_side_1) as u32)).wrapping_mul(100))).wrapping_add(((((c.m.r32((fp).wrapping_add((((l_i_2) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) }; let a_ = (((c.m.r32(sym::gCharSel)).wrapping_add(564)).wrapping_add((((p_side_1) as u32)).wrapping_mul(100))).wrapping_add(((((18i32).wrapping_add(l_i_2)) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = c.m.r32((l_res_4).wrapping_add(16)); let a_ = (((c.m.r32(sym::gCharSel)).wrapping_add(564)).wrapping_add((((p_side_1) as u32)).wrapping_mul(100))).wrapping_add(((((c.m.r32((fp).wrapping_add((((l_i_2) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "CharSel_SetCellFormChips", &PROBE_N, &[], true); }
    r_
}

pub fn CharSel_SetCustomChips(c: &mut Ctx, mut p_side_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "CharSel_SetCustomChips", &PROBE_N, &[(p_side_1) as u32], false); }
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
                    l_id_3 = { let a_ = (((c.m.r32(sym::gCharSel)).wrapping_add(13280)).wrapping_add((((p_side_1) as u32)).wrapping_mul(504))).wrapping_add(((((l_i_2).wrapping_add(({ let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(16); (c.m.r32(a_) as i32) }).wrapping_mul(7i32))) as u32)).wrapping_mul(36)); (c.m.r32(a_) as i32) };
                    { let v_ = { let a_ = (({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(48)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(76)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    { let v_ = l_id_3; let a_ = (({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(48)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    l_res_4 = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(36); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(36); c.m.r32(a_) }).wrapping_add(((((l_id_3).wrapping_add(1i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32));
                    { let v_ = { let a_ = (((c.m.r32(sym::gCharSel)).wrapping_add(564)).wrapping_add((((p_side_1) as u32)).wrapping_mul(100))).wrapping_add(((((c.m.r32((fp).wrapping_add((((l_i_2) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) }; let a_ = (((c.m.r32(sym::gCharSel)).wrapping_add(564)).wrapping_add((((p_side_1) as u32)).wrapping_mul(100))).wrapping_add(((((18i32).wrapping_add(l_i_2)) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = c.m.r32((l_res_4).wrapping_add(16)); let a_ = (((c.m.r32(sym::gCharSel)).wrapping_add(564)).wrapping_add((((p_side_1) as u32)).wrapping_mul(100))).wrapping_add(((((c.m.r32((fp).wrapping_add((((l_i_2) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "CharSel_SetCustomChips", &PROBE_N, &[], true); }
    r_
}

pub fn CharSel_SetFormChips(c: &mut Ctx, mut p_side_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "CharSel_SetFormChips", &PROBE_N, &[(p_side_1) as u32], false); }
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
                    if (((((l_i_2) < ({ let a_ = ((((c.m.r32(sym::gCharSel)).wrapping_add(13280)).wrapping_add((((p_side_1) as u32)).wrapping_mul(504))).wrapping_add((((({ let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(12); (c.m.r32(a_) as i32) }).wrapping_add(({ let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(16); (c.m.r32(a_) as i32) }).wrapping_mul(7i32))) as u32)).wrapping_mul(36))).wrapping_add(4); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                        l_id_3 = { let a_ = (((((c.m.r32(sym::gCharSel)).wrapping_add(13280)).wrapping_add((((p_side_1) as u32)).wrapping_mul(504))).wrapping_add((((({ let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(12); (c.m.r32(a_) as i32) }).wrapping_add(({ let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(16); (c.m.r32(a_) as i32) }).wrapping_mul(7i32))) as u32)).wrapping_mul(36))).wrapping_add(8)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
                    } else {
                        l_id_3 = 164i32;
                    }
                    { let v_ = { let a_ = (({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(48)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(76)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    { let v_ = l_id_3; let a_ = (({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(48)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    l_res_4 = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(36); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(36); c.m.r32(a_) }).wrapping_add(((((l_id_3).wrapping_add(1i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32));
                    { let v_ = { let a_ = (((c.m.r32(sym::gCharSel)).wrapping_add(564)).wrapping_add((((p_side_1) as u32)).wrapping_mul(100))).wrapping_add(((((c.m.r32((fp).wrapping_add((((l_i_2) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) }; let a_ = (((c.m.r32(sym::gCharSel)).wrapping_add(564)).wrapping_add((((p_side_1) as u32)).wrapping_mul(100))).wrapping_add(((((18i32).wrapping_add(l_i_2)) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = c.m.r32((l_res_4).wrapping_add(16)); let a_ = (((c.m.r32(sym::gCharSel)).wrapping_add(564)).wrapping_add((((p_side_1) as u32)).wrapping_mul(100))).wrapping_add(((((c.m.r32((fp).wrapping_add((((l_i_2) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "CharSel_SetFormChips", &PROBE_N, &[], true); }
    r_
}

pub fn CharSel_UpdateFaceLoad(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "CharSel_UpdateFaceLoad", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        's1: {
            let sv1: i32 = { let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(768); (c.m.r32(a_) as i32) };
            match sv1 {
                5i32 => {
                    if (((({ let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(776); (c.m.r32(a_) as i32) }) ^ (1i32))) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(104); (c.m.r32(a_) as i32) }) & (1i32))) != 0) {
                        { let r_ = 1i32; let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(776); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
                    }
                    { let v_ = 6i32; let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(768); c.m.w32(a_, (v_) as u32); };
                    break 's1;
                }
                6i32 => {
                    if (((({ let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(776); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(104); (c.m.r32(a_) as i32) }) & (1i32))) != 0) {
                        { let r_ = 1i32; let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(776); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(104); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
                    }
                    { let v_ = 1i32; let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(768); c.m.w32(a_, (v_) as u32); };
                    break 's1;
                }
                1i32 => {
                    F::File_CancelRequests(c);
                    { let _ = { let a0_ = ({ let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(776); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(108); (c.m.r32(a_) as i32) }).wrapping_add(761i32); let a1_ = { let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(8)).wrapping_add(((({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(776); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.r32(a_) }; let _ = 92160i32; F::File_Request(c, a0_, a1_) }; };
                    { let v_ = 2i32; let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(768); c.m.w32(a_, (v_) as u32); };
                    break 's1;
                }
                2i32 => {
                    if ((F::File_UpdateRequests(c)) != 0) {
                        { let v_ = 3i32; let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(768); c.m.w32(a_, (v_) as u32); };
                    }
                    break 's1;
                }
                3i32 => {
                    { let _ = { let a0_ = { let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(8)).wrapping_add(((({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(776); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.r32(a_) }; let a1_ = { let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(16)).wrapping_add(((({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(776); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.r32(a_) }; F::Sprite_Unpack(c, a0_, a1_, ((0i32) as u32)) }; };
                    { let v_ = { let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(16)).wrapping_add(((({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(776); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.r32(a_) }; let a_ = fp; c.m.w32(a_, v_); };
                    { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
                    if ((((({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(776); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
                        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(280)).wrapping_add((((23i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    } else {
                        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(280)).wrapping_add((((22i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    }
                    { let r_ = 2i32; let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(776); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(104); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    if (((({ let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(776); (c.m.r32(a_) as i32) }) ^ (1i32))) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(104); (c.m.r32(a_) as i32) }) & (1i32))) != 0) {
                        { let r_ = 1i32; let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(776); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
                        { let v_ = 6i32; let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(768); c.m.w32(a_, (v_) as u32); };
                    } else {
                        { let v_ = 4i32; let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(768); c.m.w32(a_, (v_) as u32); };
                    }
                    break 's1;
                }
                4i32 => {
                    if ((((({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(772); (c.m.r32(a_) as i32) }) != (4i32)) as i32)) != 0) {
                        break 's1;
                    }
                    if (((({ let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(104); (c.m.r32(a_) as i32) }) & (1i32))) != 0) {
                        { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(280)).wrapping_add((((23i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                        { let v_ = 0i32; let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(776); c.m.w32(a_, (v_) as u32); };
                        { let v_ = 6i32; let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(768); c.m.w32(a_, (v_) as u32); };
                    } else {
                        if (((({ let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(104); (c.m.r32(a_) as i32) }) & (1i32))) != 0) {
                            { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(280)).wrapping_add((((22i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                            { let v_ = 1i32; let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(776); c.m.w32(a_, (v_) as u32); };
                            { let v_ = 6i32; let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(768); c.m.w32(a_, (v_) as u32); };
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
    if rt::probing() { crate::host::harness::probe(c, "CharSel_UpdateFaceLoad", &PROBE_N, &[], true); }
    r_
}

pub fn CharSel_RequestFace(c: &mut Ctx, mut p_side_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "CharSel_RequestFace", &PROBE_N, &[(p_side_1) as u32], false); }
    let r_ = 'f: {
    if (((({ let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(104); (c.m.r32(a_) as i32) }) & (2i32))) != 0) {
        { let r_ = 2i32; let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(104); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
    } else {
        if ((((({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(776); (c.m.r32(a_) as i32) }) == (p_side_1)) as i32)) != 0) {
            { let v_ = 5i32; let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(768); c.m.w32(a_, (v_) as u32); };
        }
    }
    { let r_ = 1i32; let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add((((p_side_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(104); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "CharSel_RequestFace", &PROBE_N, &[], true); }
    r_
}

pub fn CharSel_UpdateStageLoad(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "CharSel_UpdateStageLoad", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        's1: {
            let sv1: i32 = { let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(772); (c.m.r32(a_) as i32) };
            match sv1 {
                5i32 => {
                    { let v_ = 6i32; let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(772); c.m.w32(a_, (v_) as u32); };
                    break 's1;
                }
                6i32 => {
                    if (((({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(764); (c.m.r32(a_) as i32) }) & (32i32))) != 0) {
                        { let r_ = 32i32; let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(764); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
                    }
                    { let v_ = 1i32; let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(772); c.m.w32(a_, (v_) as u32); };
                    break 's1;
                }
                1i32 => {
                    F::File_CancelRequests(c);
                    { let _ = { let a0_ = (925i32).wrapping_add((if ((((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(1364); c.m.r32(a_) }).wrapping_add(60); (c.m.r32(a_) as i32) }) < (36i32)) as i32)) != 0) { { let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(1364); c.m.r32(a_) }).wrapping_add(60); (c.m.r32(a_) as i32) } } else { ({ let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(1364); c.m.r32(a_) }).wrapping_add(60); (c.m.r32(a_) as i32) }).wrapping_sub(36i32) })); let a1_ = { let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(24); c.m.r32(a_) }; let _ = 243712i32; F::File_Request(c, a0_, a1_) }; };
                    { let v_ = 2i32; let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(772); c.m.w32(a_, (v_) as u32); };
                    break 's1;
                }
                2i32 => {
                    if ((F::File_UpdateRequests(c)) != 0) {
                        { let v_ = 3i32; let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(772); c.m.w32(a_, (v_) as u32); };
                    }
                    break 's1;
                }
                3i32 => {
                    { let _ = { let a0_ = { let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(24); c.m.r32(a_) }; let a1_ = { let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(28)).wrapping_add(((({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(780); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.r32(a_) }; F::Sprite_Unpack(c, a0_, a1_, ((0i32) as u32)) }; };
                    { let v_ = { let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(28)).wrapping_add(((({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(780); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.r32(a_) }; let a_ = fp; c.m.w32(a_, v_); };
                    { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
                    { let v_ = c.m.r32(fp); let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(272)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let r_ = 1i32; let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(780); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
                    { let r_ = 64i32; let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(764); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    { let v_ = 4i32; let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(772); c.m.w32(a_, (v_) as u32); };
                    break 's1;
                }
                4i32 => {
                    if ((((({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(768); (c.m.r32(a_) as i32) }) != (4i32)) as i32)) != 0) {
                        break 's1;
                    }
                    if (((({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(764); (c.m.r32(a_) as i32) }) & (32i32))) != 0) {
                        { let v_ = 6i32; let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(772); c.m.w32(a_, (v_) as u32); };
                    }
                    break 's1;
                }
                _ => {
                }
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "CharSel_UpdateStageLoad", &PROBE_N, &[], true); }
    r_
}

pub fn CharSel_RequestStage(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "CharSel_RequestStage", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if (((({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(764); (c.m.r32(a_) as i32) }) & (64i32))) != 0) {
        { let r_ = 64i32; let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(764); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
    } else {
        { let v_ = 5i32; let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(772); c.m.w32(a_, (v_) as u32); };
    }
    { let r_ = 32i32; let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(764); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    { let v_ = { let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(28)).wrapping_add(((({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(780); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.r32(a_) }; let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(272)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    { let v_ = { let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(28)).wrapping_add(((((({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(780); (c.m.r32(a_) as i32) }) ^ (1i32))) as u32)).wrapping_mul(4)); c.m.r32(a_) }; let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(272)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "CharSel_RequestStage", &PROBE_N, &[], true); }
    r_
}

pub fn CharSel_ClipGoto(c: &mut Ctx, mut p_flash_1: i32, mut p_side_2: i32, mut p_kind_3: i32, mut p_label_4: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "CharSel_ClipGoto", &PROBE_N, &[(p_flash_1) as u32, (p_side_2) as u32, (p_kind_3) as u32, (p_label_4) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(80) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 80);
    let mut l_f_5: u32 = 0;
    let r_ = 'f: {
        l_f_5 = ((c.m.r32(sym::gCharSel)).wrapping_add(52)).wrapping_add((((p_flash_1) as u32)).wrapping_mul(44));
        's1: {
            let sv6: i32 = p_kind_3;
            match sv6 {
                0i32 => {
                    { let _ = { let a2_ = { let a_ = { let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add((((p_side_2) as u32)).wrapping_mul(4)); c.m.r32(a_) }; (c.m.r32(a_) as i32) }; F::sprintf(c, fp.wrapping_add(8), sym::menu_c_e__str0, &[((a2_) as i64 as u64)]) }; };
                    break 's1;
                }
                1i32 => {
                    { let _ = { let a2_ = { let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add((((p_side_2) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(8); (c.m.r32(a_) as i32) }; F::sprintf(c, fp.wrapping_add(8), sym::menu_c_e__str1, &[((a2_) as i64 as u64)]) }; };
                    break 's1;
                }
                2i32 => {
                    { let _ = { let a2_ = ({ let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add((((p_side_2) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(20); (c.m.r32(a_) as i32) }).wrapping_add(1i32); F::sprintf(c, fp.wrapping_add(8), sym::menu_c_e__str2, &[((a2_) as i64 as u64)]) }; };
                    break 's1;
                }
                5i32 => {
                    { let _ = { let a2_ = ({ let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add((((p_side_2) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(24); (c.m.r32(a_) as i32) }).wrapping_add(1i32); F::sprintf(c, fp.wrapping_add(8), sym::menu_c_e__str3, &[((a2_) as i64 as u64)]) }; };
                    break 's1;
                }
                6i32 => {
                    { let _ = { let a2_ = { let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add((((p_side_2) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(12); (c.m.r32(a_) as i32) }; F::sprintf(c, fp.wrapping_add(8), sym::menu_c_e__str4, &[((a2_) as i64 as u64)]) }; };
                    break 's1;
                }
                8i32 => {
                    if ((((({ let a_ = { let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(1364); c.m.r32(a_) }; (c.m.r32(a_) as i32) }) == (6i32)) as i32)) != 0) {
                        { let _ = F::sprintf(c, fp.wrapping_add(8), sym::menu_c_e__str5, &[]); };
                    } else {
                        { let _ = { let a2_ = { let a_ = { let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(1364); c.m.r32(a_) }; (c.m.r32(a_) as i32) }; F::sprintf(c, fp.wrapping_add(8), sym::menu_c_e__str6, &[((a2_) as i64 as u64)]) }; };
                    }
                    break 's1;
                }
                3i32 | 7i32 | 9i32 => {
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
    if rt::probing() { crate::host::harness::probe(c, "CharSel_ClipGoto", &PROBE_N, &[], true); }
    r_
}

pub fn CharSel_Init(c: &mut Ctx, mut p_section_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "CharSel_Init", &PROBE_N, &[(p_section_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_i_2: i32 = 0;
    let mut l_cols_3: i32 = 0;
    let mut l_stageCols_4: i32 = 0;
    let mut l_stageRows_5: i32 = 0;
    let mut l_data_6: u32 = 0;
    let mut l_extra_7: i32 = 0;
    let mut l_swaps_8: i32 = 0;
    let mut l_n_9: i32 = 0;
    let mut l_i_10: i32 = 0;
    let mut l_rows_11: i32 = 0;
    let mut l_r_12: i32 = 0;
    let mut l_k_13: i32 = 0;
    let mut l_dbg_14: i32 = 0;
    let mut l_dbg_15: i32 = 0;
    let mut l_added_16: i32 = 0;
    let mut l_n_17: i32 = 0;
    let mut l_k_18: i32 = 0;
    let mut l_last_19: i32 = 0;
    let mut l_dbg_20: i32 = 0;
    let r_ = 'f: {
        { let v_ = ((0i32) as u32); c.m.w32(fp, v_); }
        { let v_ = F::Heap_Alloc(c, 14924i32, ((32i32) as u32), 0i32, 2i32); let a_ = sym::gCharSel; c.m.w32(a_, v_); };
        { let _ = { let a0_ = c.m.r32(sym::gCharSel); F::memset(c, a0_, 0i32, ((14924i32) as u32)) }; };
        if ((((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); (c.m.r32(a_) as i32) }) == (45i32)) as i32)) != 0) {
            { let v_ = (c.m.r32(sym::gMenuArc6)).wrapping_add((({ let a_ = (c.m.r32(sym::gMenuArc6)).wrapping_add((((p_section_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = c.m.r32(sym::gCharSel); c.m.w32(a_, v_); };
            { let v_ = { let a0_ = { let a_ = c.m.r32(sym::gCharSel); c.m.r32(a_) }; F::Sprite_Unpack(c, a0_, ((0i32) as u32), ((0i32) as u32)) }; let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(4); c.m.w32(a_, v_); };
        } else {
            { let v_ = (c.m.r32(sym::gMenuArc5)).wrapping_add((({ let a_ = (c.m.r32(sym::gMenuArc5)).wrapping_add((((p_section_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = c.m.r32(sym::gCharSel); c.m.w32(a_, v_); };
            { let v_ = { let a0_ = { let a_ = c.m.r32(sym::gCharSel); c.m.r32(a_) }; F::Sprite_Unpack(c, a0_, ((0i32) as u32), ((0i32) as u32)) }; let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(4); c.m.w32(a_, v_); };
        }
        if ((((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); (c.m.r32(a_) as i32) }) == (45i32)) as i32)) != 0) {
            { let v_ = 0i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1568); c.m.w32(a_, (v_) as u32); };
        }
        { let v_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((47i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(280)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(280)).wrapping_add((((17i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(280)).wrapping_add((((16i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(280)).wrapping_add((((18i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(280)).wrapping_add((((20i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(280)).wrapping_add((((19i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(280)).wrapping_add((((9i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(280)).wrapping_add((((40i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        if ((((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1568); (c.m.r32(a_) as i32) }) == (1i32)) as i32)) != 0) {
            { let v_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((50i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
            { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
            { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(280)).wrapping_add((((10i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        }
        { let v_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(280)).wrapping_add((((39i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(280)).wrapping_add((((38i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((8i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(280)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(280)).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((4i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(280)).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(280)).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((9i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(280)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(280)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((10i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(280)).wrapping_add((((21i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((11i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(280)).wrapping_add((((24i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(280)).wrapping_add((((26i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(280)).wrapping_add((((25i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(280)).wrapping_add((((27i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((12i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(280)).wrapping_add((((47i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(280)).wrapping_add((((48i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        {
            { let a0_ = (({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(280)).wrapping_add((((39i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) as i32); let a1_ = (({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(280)).wrapping_add((((19i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) as i32); let a2_ = (({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(280)).wrapping_add((((20i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) as i32); F::Port_NameFontSheets(c, a0_, a1_, a2_) };
        }
        { let a0_ = ((c.m.r32(sym::gCharSel)).wrapping_add(52)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); let a1_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((15i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a2_ = (c.m.r32(sym::gCharSel)).wrapping_add(280); F::Flash_Create(c, a0_, a1_, a2_) };
        { let a0_ = ((c.m.r32(sym::gCharSel)).wrapping_add(52)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_Play(c, a0_, 1i32) };
        { let v_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((17i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = (((c.m.r32(sym::gCharSel)).wrapping_add(564)).wrapping_add((((0i32) as u32)).wrapping_mul(100))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = (((c.m.r32(sym::gCharSel)).wrapping_add(564)).wrapping_add((((0i32) as u32)).wrapping_mul(100))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((18i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = (((c.m.r32(sym::gCharSel)).wrapping_add(564)).wrapping_add((((1i32) as u32)).wrapping_mul(100))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = (((c.m.r32(sym::gCharSel)).wrapping_add(564)).wrapping_add((((1i32) as u32)).wrapping_mul(100))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((19i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = (((c.m.r32(sym::gCharSel)).wrapping_add(564)).wrapping_add((((0i32) as u32)).wrapping_mul(100))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = (((c.m.r32(sym::gCharSel)).wrapping_add(564)).wrapping_add((((0i32) as u32)).wrapping_mul(100))).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((4i32).wrapping_mul(64i32)) as u32)); let a_ = (((c.m.r32(sym::gCharSel)).wrapping_add(564)).wrapping_add((((0i32) as u32)).wrapping_mul(100))).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = (((c.m.r32(sym::gCharSel)).wrapping_add(564)).wrapping_add((((0i32) as u32)).wrapping_mul(100))).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((20i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = (((c.m.r32(sym::gCharSel)).wrapping_add(564)).wrapping_add((((1i32) as u32)).wrapping_mul(100))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = (((c.m.r32(sym::gCharSel)).wrapping_add(564)).wrapping_add((((1i32) as u32)).wrapping_mul(100))).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((4i32).wrapping_mul(64i32)) as u32)); let a_ = (((c.m.r32(sym::gCharSel)).wrapping_add(564)).wrapping_add((((1i32) as u32)).wrapping_mul(100))).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = (((c.m.r32(sym::gCharSel)).wrapping_add(564)).wrapping_add((((1i32) as u32)).wrapping_mul(100))).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((21i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = (((c.m.r32(sym::gCharSel)).wrapping_add(564)).wrapping_add((((0i32) as u32)).wrapping_mul(100))).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = (((c.m.r32(sym::gCharSel)).wrapping_add(564)).wrapping_add((((1i32) as u32)).wrapping_mul(100))).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(280)).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((22i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = (((c.m.r32(sym::gCharSel)).wrapping_add(564)).wrapping_add((((0i32) as u32)).wrapping_mul(100))).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = (((c.m.r32(sym::gCharSel)).wrapping_add(564)).wrapping_add((((1i32) as u32)).wrapping_mul(100))).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((23i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = (((c.m.r32(sym::gCharSel)).wrapping_add(564)).wrapping_add((((0i32) as u32)).wrapping_mul(100))).wrapping_add((((10i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = (((c.m.r32(sym::gCharSel)).wrapping_add(564)).wrapping_add((((0i32) as u32)).wrapping_mul(100))).wrapping_add((((17i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = (((c.m.r32(sym::gCharSel)).wrapping_add(564)).wrapping_add((((1i32) as u32)).wrapping_mul(100))).wrapping_add((((10i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = (((c.m.r32(sym::gCharSel)).wrapping_add(564)).wrapping_add((((1i32) as u32)).wrapping_mul(100))).wrapping_add((((17i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = (((c.m.r32(sym::gCharSel)).wrapping_add(476)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = (((c.m.r32(sym::gCharSel)).wrapping_add(476)).wrapping_add((((1i32) as u32)).wrapping_mul(44))).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let a0_ = ((c.m.r32(sym::gCharSel)).wrapping_add(52)).wrapping_add((((1i32) as u32)).wrapping_mul(44)); let a1_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((24i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a2_ = ((c.m.r32(sym::gCharSel)).wrapping_add(564)).wrapping_add((((0i32) as u32)).wrapping_mul(100)); F::Flash_Create(c, a0_, a1_, a2_) };
        { let a0_ = ((c.m.r32(sym::gCharSel)).wrapping_add(52)).wrapping_add((((1i32) as u32)).wrapping_mul(44)); F::Flash_Play(c, a0_, 1i32) };
        { let a0_ = ((c.m.r32(sym::gCharSel)).wrapping_add(52)).wrapping_add((((2i32) as u32)).wrapping_mul(44)); let a1_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((25i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a2_ = ((c.m.r32(sym::gCharSel)).wrapping_add(564)).wrapping_add((((1i32) as u32)).wrapping_mul(100)); F::Flash_Create(c, a0_, a1_, a2_) };
        { let a0_ = ((c.m.r32(sym::gCharSel)).wrapping_add(52)).wrapping_add((((2i32) as u32)).wrapping_mul(44)); F::Flash_Play(c, a0_, 1i32) };
        { let v_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((48i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = (((c.m.r32(sym::gCharSel)).wrapping_add(476)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add((((9i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = (((c.m.r32(sym::gCharSel)).wrapping_add(476)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((49i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = (((c.m.r32(sym::gCharSel)).wrapping_add(476)).wrapping_add((((1i32) as u32)).wrapping_mul(44))).wrapping_add((((9i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = (((c.m.r32(sym::gCharSel)).wrapping_add(476)).wrapping_add((((1i32) as u32)).wrapping_mul(44))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((26i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = (((c.m.r32(sym::gCharSel)).wrapping_add(476)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = (((c.m.r32(sym::gCharSel)).wrapping_add(476)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((27i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = (((c.m.r32(sym::gCharSel)).wrapping_add(476)).wrapping_add((((1i32) as u32)).wrapping_mul(44))).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = (((c.m.r32(sym::gCharSel)).wrapping_add(476)).wrapping_add((((1i32) as u32)).wrapping_mul(44))).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((28i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = (((c.m.r32(sym::gCharSel)).wrapping_add(476)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = (((c.m.r32(sym::gCharSel)).wrapping_add(476)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = (((c.m.r32(sym::gCharSel)).wrapping_add(476)).wrapping_add((((1i32) as u32)).wrapping_mul(44))).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = (((c.m.r32(sym::gCharSel)).wrapping_add(476)).wrapping_add((((1i32) as u32)).wrapping_mul(44))).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((29i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = (((c.m.r32(sym::gCharSel)).wrapping_add(476)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = (((c.m.r32(sym::gCharSel)).wrapping_add(476)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add((((8i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = (((c.m.r32(sym::gCharSel)).wrapping_add(476)).wrapping_add((((1i32) as u32)).wrapping_mul(44))).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = (((c.m.r32(sym::gCharSel)).wrapping_add(476)).wrapping_add((((1i32) as u32)).wrapping_mul(44))).wrapping_add((((8i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((30i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = (((c.m.r32(sym::gCharSel)).wrapping_add(476)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add((((10i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = (((c.m.r32(sym::gCharSel)).wrapping_add(476)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((31i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = (((c.m.r32(sym::gCharSel)).wrapping_add(476)).wrapping_add((((1i32) as u32)).wrapping_mul(44))).wrapping_add((((10i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = (((c.m.r32(sym::gCharSel)).wrapping_add(476)).wrapping_add((((1i32) as u32)).wrapping_mul(44))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let a0_ = ((c.m.r32(sym::gCharSel)).wrapping_add(52)).wrapping_add((((3i32) as u32)).wrapping_mul(44)); let a1_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((32i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a2_ = ((c.m.r32(sym::gCharSel)).wrapping_add(476)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_Create(c, a0_, a1_, a2_) };
        { let a0_ = ((c.m.r32(sym::gCharSel)).wrapping_add(52)).wrapping_add((((3i32) as u32)).wrapping_mul(44)); F::Flash_Play(c, a0_, 1i32) };
        { let a0_ = ((c.m.r32(sym::gCharSel)).wrapping_add(52)).wrapping_add((((4i32) as u32)).wrapping_mul(44)); let a1_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((33i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a2_ = ((c.m.r32(sym::gCharSel)).wrapping_add(476)).wrapping_add((((1i32) as u32)).wrapping_mul(44)); F::Flash_Create(c, a0_, a1_, a2_) };
        { let a0_ = ((c.m.r32(sym::gCharSel)).wrapping_add(52)).wrapping_add((((4i32) as u32)).wrapping_mul(44)); F::Flash_Play(c, a0_, 1i32) };
        l_data_6 = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((53i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32));
        F::ItemPanel_Init(c, l_data_6, 0i32);
        F::ItemPanel_Init(c, l_data_6, 1i32);
        { let a0_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((51i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); F::ItemHelp_Init(c, a0_) };
        { let v_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((55i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let a0_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((54i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a1_ = c.m.r32(fp); F::IconWin_Init(c, a0_, a1_) };
        { let v_ = ({ let a_ = ((c.m.r32(sym::gCommonRes)).wrapping_add(4)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = ((c.m.r32(sym::gCommonRes)).wrapping_add(4)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14920); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((43i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(40); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((44i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(44); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((45i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(36); c.m.w32(a_, v_); };
        {
            l_i_2 = 0i32;
            'l1: loop {
                if !(((((l_i_2) < (165i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let v_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(36); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(36); c.m.r32(a_) }).wrapping_add(((((l_i_2).wrapping_add(1i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
                    { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        { let v_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((46i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(48); c.m.w32(a_, v_); };
        {
            l_i_2 = 0i32;
            'l3: loop {
                if !(((((l_i_2) < (38i32)) as i32)) != 0) { break 'l3; }
                'c4: {
                    { let v_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(48); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(48); c.m.r32(a_) }).wrapping_add(((((l_i_2).wrapping_add(1i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
                    { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        { let v_ = (({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((39i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32))).wrapping_add(((16i32) as u32)); let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14340); c.m.w32(a_, v_); };
        { let v_ = { let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((39i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14348); c.m.w32(a_, (v_) as u32); };
        { let a0_ = (c.m.r32(sym::gCharSel)).wrapping_add(14348); let a1_ = { let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14340); c.m.r32(a_) }; F::StgGrid_ApplyUnlocks(c, a0_, a1_) };
        {
            l_extra_7 = (if ((F::Port_NetSession(c)) != 0) { 0i32 } else { (c.m.r32(sym::gPortExtraStageCount) as i32) });
            l_swaps_8 = (if ((F::Port_NetSession(c)) != 0) { 0i32 } else { (c.m.r32(sym::gPortStageReplaceCount) as i32) });
            l_n_9 = { let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14348); (c.m.r32(a_) as i32) };
            if (((((l_n_9) > (64i32)) as i32)) != 0) {
                l_n_9 = 64i32;
            }
            {
                l_i_10 = 0i32;
                'l5: loop {
                    if !(((((l_i_10) < (l_n_9)) as i32)) != 0) { break 'l5; }
                    'c6: {
                        { let v_ = { let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14340); c.m.r32(a_) }).wrapping_add((((l_i_10) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (sym::menu_c_e__CharSel_Init__sAllIds).wrapping_add((((l_i_10) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    }
                    { let o_ = l_i_10; let n_ = (o_).wrapping_add(1); l_i_10 = n_; };
                }
            }
            {
                l_i_10 = 0i32;
                'l7: loop {
                    if !(((((((((l_i_10) < (l_extra_7)) as i32)) != 0) && (((((l_n_9) < (90i32)) as i32)) != 0)) as i32)) != 0) { break 'l7; }
                    'c8: {
                        { let v_ = (c.m.r32((sym::gPortExtraStages).wrapping_add((((l_i_10) as u32)).wrapping_mul(4))) as i32); let a_ = (sym::menu_c_e__CharSel_Init__sAllIds).wrapping_add(((({ let o_ = l_n_9; let n_ = (o_).wrapping_add(1); l_n_9 = n_; o_ }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    }
                    { let o_ = l_i_10; let n_ = (o_).wrapping_add(1); l_i_10 = n_; };
                }
            }
            {
                l_r_12 = 0i32;
                'l9: loop {
                    if !(((((l_r_12) < (l_swaps_8)) as i32)) != 0) { break 'l9; }
                    'c10: {
                        {
                            l_k_13 = 0i32;
                            'l11: loop {
                                if !(((((l_k_13) < (l_n_9)) as i32)) != 0) { break 'l11; }
                                'c12: {
                                    if ((((((c.m.r32((sym::menu_c_e__CharSel_Init__sAllIds).wrapping_add((((l_k_13) as u32)).wrapping_mul(4))) as i32)) == ((c.m.r32((sym::gPortReplaceOld).wrapping_add((((l_r_12) as u32)).wrapping_mul(4))) as i32))) as i32)) != 0) {
                                        { let v_ = (c.m.r32((sym::gPortReplaceNew).wrapping_add((((l_r_12) as u32)).wrapping_mul(4))) as i32); let a_ = (sym::menu_c_e__CharSel_Init__sAllIds).wrapping_add((((l_k_13) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                        break 'l11;
                                    }
                                }
                                { let o_ = l_k_13; let n_ = (o_).wrapping_add(1); l_k_13 = n_; };
                            }
                        }
                    }
                    { let o_ = l_r_12; let n_ = (o_).wrapping_add(1); l_r_12 = n_; };
                }
            }
            { let v_ = (((((((l_extra_7) > (0i32)) as i32)) != 0) || (((((l_swaps_8) > (0i32)) as i32)) != 0)) as i32); let a_ = sym::menu_c_e__sPortStagesAdded; c.m.w32(a_, (v_) as u32); };
            if (((c.m.r32(sym::menu_c_e__sPortStagesAdded) as i32)) != 0) {
                l_rows_11 = rt::div_i32(((l_n_9).wrapping_add(6i32)).wrapping_sub(1i32), 6i32);
                {
                    l_i_10 = l_n_9;
                    'l13: loop {
                        if !(((((((((l_i_10) < ((l_rows_11).wrapping_mul(6i32))) as i32)) != 0) && (((((l_i_10) < (96i32)) as i32)) != 0)) as i32)) != 0) { break 'l13; }
                        'c14: {
                            { let v_ = 63i32; let a_ = (sym::menu_c_e__CharSel_Init__sAllIds).wrapping_add((((l_i_10) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                        }
                        { let o_ = l_i_10; let n_ = (o_).wrapping_add(1); l_i_10 = n_; };
                    }
                }
                { let v_ = sym::menu_c_e__CharSel_Init__sAllIds; let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14340); c.m.w32(a_, v_); };
                { let v_ = l_n_9; let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14348); c.m.w32(a_, (v_) as u32); };
            }
        }
        if (((((F::getenv(c, sym::menu_c_e__str7)) != (((0i32) as u32))) as i32)) != 0) {
            { let _ = { let a0_ = c.m.r32(sym::stderr); let a2_ = { let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14348); (c.m.r32(a_) as i32) }; F::fprintf(c, a0_, sym::menu_c_e__str8, &[((a2_) as i64 as u64)]) }; };
            {
                l_dbg_14 = 0i32;
                'l15: loop {
                    if !(((((l_dbg_14) < ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14348); (c.m.r32(a_) as i32) })) as i32)) != 0) { break 'l15; }
                    'c16: {
                        { let _ = { let a0_ = c.m.r32(sym::stderr); let a2_ = (((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14340); c.m.r32(a_) }).wrapping_add((((l_dbg_14) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)) as i32); F::fprintf(c, a0_, sym::menu_c_e__str9, &[((a2_) as i64 as u64)]) }; };
                    }
                    { let o_ = l_dbg_14; let n_ = (o_).wrapping_add(1); l_dbg_14 = n_; };
                }
            }
            { let _ = { let a0_ = c.m.r32(sym::stderr); F::fprintf(c, a0_, sym::menu_c_e__str10, &[]) }; };
        }
        { let v_ = 6i32; let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14312); c.m.w32(a_, (v_) as u32); };
        if (((c.m.r32(sym::menu_c_e__sPortStagesAdded) as i32)) != 0) {
            { let v_ = rt::div_i32((({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14348); (c.m.r32(a_) as i32) }).wrapping_add(6i32)).wrapping_sub(1i32), 6i32); let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14312); c.m.w32(a_, (v_) as u32); };
        }
        { let v_ = (({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((56i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32))).wrapping_add(((16i32) as u32)); let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14344); c.m.w32(a_, v_); };
        { let v_ = { let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((56i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14352); c.m.w32(a_, (v_) as u32); };
        { let a0_ = (c.m.r32(sym::gCharSel)).wrapping_add(14352); let a1_ = { let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14344); c.m.r32(a_) }; F::BgmList_ApplyUnlocks(c, a0_, a1_) };
        if (((((F::getenv(c, sym::menu_c_e__str11)) != (((0i32) as u32))) as i32)) != 0) {
            { let _ = { let a0_ = c.m.r32(sym::stderr); let a2_ = { let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14352); (c.m.r32(a_) as i32) }; F::fprintf(c, a0_, sym::menu_c_e__str12, &[((a2_) as i64 as u64)]) }; };
            {
                l_dbg_15 = 0i32;
                'l17: loop {
                    if !(((((l_dbg_15) < ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14352); (c.m.r32(a_) as i32) })) as i32)) != 0) { break 'l17; }
                    'c18: {
                        { let _ = { let a0_ = c.m.r32(sym::stderr); let a2_ = (((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14344); c.m.r32(a_) }).wrapping_add((((l_dbg_15) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)) as i32); F::fprintf(c, a0_, sym::menu_c_e__str13, &[((a2_) as i64 as u64)]) }; };
                    }
                    { let o_ = l_dbg_15; let n_ = (o_).wrapping_add(1); l_dbg_15 = n_; };
                }
            }
            { let _ = { let a0_ = c.m.r32(sym::stderr); F::fprintf(c, a0_, sym::menu_c_e__str14, &[]) }; };
        }
        { let r_ = 4i32; let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14352); let n_ = ((c.m.r32(a_) as i32)).wrapping_sub(r_); c.m.w32(a_, (n_) as u32); };
        { let v_ = 24i32; let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14344); c.m.r32(a_) }).wrapping_add((((({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14352); (c.m.r32(a_) as i32) }).wrapping_sub(1i32)) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        {
            l_added_16 = (if ((F::Port_NetSession(c)) != 0) { 0i32 } else { (c.m.r32(sym::gPortSongCount) as i32) });
            if (((((l_added_16) > (0i32)) as i32)) != 0) {
                l_n_17 = { let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14352); (c.m.r32(a_) as i32) };
                l_last_19 = { let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14344); c.m.r32(a_) }).wrapping_add(((((l_n_17).wrapping_sub(1i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
                if (((((l_n_17) > (30i32)) as i32)) != 0) {
                    l_n_17 = 30i32;
                }
                if (((((l_added_16) > (40i32)) as i32)) != 0) {
                    l_added_16 = 40i32;
                }
                {
                    l_k_18 = 0i32;
                    'l19: loop {
                        if !(((((l_k_18) < ((l_n_17).wrapping_sub(1i32))) as i32)) != 0) { break 'l19; }
                        'c20: {
                            { let v_ = { let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14344); c.m.r32(a_) }).wrapping_add((((l_k_18) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (sym::menu_c_e__CharSel_Init__sBgmIds).wrapping_add((((l_k_18) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                        }
                        { let o_ = l_k_18; let n_ = (o_).wrapping_add(1); l_k_18 = n_; };
                    }
                }
                {
                    l_k_18 = 0i32;
                    'l21: loop {
                        if !(((((l_k_18) < (l_added_16)) as i32)) != 0) { break 'l21; }
                        'c22: {
                            { let v_ = (c.m.r32((sym::gPortSongOffsets).wrapping_add((((l_k_18) as u32)).wrapping_mul(4))) as i32); let a_ = (sym::menu_c_e__CharSel_Init__sBgmIds).wrapping_add((((((l_n_17).wrapping_sub(1i32)).wrapping_add(l_k_18)) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                        }
                        { let o_ = l_k_18; let n_ = (o_).wrapping_add(1); l_k_18 = n_; };
                    }
                }
                l_n_17 = (((l_n_17).wrapping_sub(1i32)).wrapping_add(l_added_16)).wrapping_add(1i32);
                { let v_ = l_last_19; let a_ = (sym::menu_c_e__CharSel_Init__sBgmIds).wrapping_add(((((l_n_17).wrapping_sub(1i32)) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = sym::menu_c_e__CharSel_Init__sBgmIds; let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14344); c.m.w32(a_, v_); };
                { let v_ = l_n_17; let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14352); c.m.w32(a_, (v_) as u32); };
            }
        }
        if (((((F::getenv(c, sym::menu_c_e__str15)) != (((0i32) as u32))) as i32)) != 0) {
            { let _ = { let a0_ = c.m.r32(sym::stderr); let a2_ = { let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14352); (c.m.r32(a_) as i32) }; F::fprintf(c, a0_, sym::menu_c_e__str16, &[((a2_) as i64 as u64)]) }; };
            {
                l_dbg_20 = 0i32;
                'l23: loop {
                    if !(((((l_dbg_20) < ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14352); (c.m.r32(a_) as i32) })) as i32)) != 0) { break 'l23; }
                    'c24: {
                        { let _ = { let a0_ = c.m.r32(sym::stderr); let a2_ = (((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14344); c.m.r32(a_) }).wrapping_add((((l_dbg_20) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)) as i32); F::fprintf(c, a0_, sym::menu_c_e__str17, &[((a2_) as i64 as u64)]) }; };
                    }
                    { let o_ = l_dbg_20; let n_ = (o_).wrapping_add(1); l_dbg_20 = n_; };
                }
            }
            { let _ = { let a0_ = c.m.r32(sym::stderr); F::fprintf(c, a0_, sym::menu_c_e__str18, &[]) }; };
        }
        { let v_ = (({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((42i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32))).wrapping_add(((16i32) as u32)); let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1376)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = { let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((42i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1368)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = { let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1376)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }; let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1376)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = { let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1368)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1368)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        {
            { l_i_2 = 0i32; l_cols_3 = 7i32; };
            'l25: loop {
                if !(((((l_i_2) < (2i32)) as i32)) != 0) { break 'l25; }
                'c26: {
                    { let a0_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1384)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); let a1_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1392)).wrapping_add((((l_i_2) as u32)).wrapping_mul(5940)); let a2_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1368)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); let a3_ = { let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1376)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); c.m.r32(a_) }; let a4_ = ((c.m.r32(sym::gCharSel)).wrapping_add(13272)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); let a5_ = ((c.m.r32(sym::gCharSel)).wrapping_add(13280)).wrapping_add((((l_i_2) as u32)).wrapping_mul(504)); F::ChrGrid_Build(c, a0_, a1_, a2_, a3_, a4_, a5_) };
                    { let v_ = { let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1384)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1368)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    { let v_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1392)).wrapping_add((((l_i_2) as u32)).wrapping_mul(5940)); let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1376)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = rt::div_i32({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1368)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }, l_cols_3); let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(14288)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    if (((((rt::rem_i32({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1368)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }, l_cols_3)) != (0i32)) as i32)) != 0) {
                        { let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(14288)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    }
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        { let v_ = F::Heap_Alloc(c, 92160i32, ((64i32) as u32), 0i32, 2i32); let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = F::Heap_Alloc(c, 92160i32, ((64i32) as u32), 0i32, 2i32); let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(8)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = F::Heap_Alloc(c, 133120i32, ((32i32) as u32), 0i32, 2i32); let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = F::Heap_Alloc(c, 133120i32, ((32i32) as u32), 0i32, 2i32); let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = F::Heap_Alloc(c, 243712i32, ((64i32) as u32), 0i32, 2i32); let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(24); c.m.w32(a_, v_); };
        { let v_ = F::Heap_Alloc(c, 274432i32, ((32i32) as u32), 0i32, 2i32); let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(28)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = F::Heap_Alloc(c, 274432i32, ((32i32) as u32), 0i32, 2i32); let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(28)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ((c.m.r32(sym::gCharSel)).wrapping_add(792)).wrapping_add((((0i32) as u32)).wrapping_mul(240)); let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ((c.m.r32(sym::gCharSel)).wrapping_add(792)).wrapping_add((((1i32) as u32)).wrapping_mul(240)); let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = (c.m.r32(sym::gCharSel)).wrapping_add(1280); let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(1364); c.m.w32(a_, v_); };
        { let s_ = ((c.m.r32(sym::gProgress)).wrapping_add(1088)).wrapping_add((((0i32) as u32)).wrapping_mul(240)); let d_ = { let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }; rt::memcpy(c, d_, s_, 48); };
        { let s_ = ((c.m.r32(sym::gProgress)).wrapping_add(1088)).wrapping_add((((1i32) as u32)).wrapping_mul(240)); let d_ = { let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }; rt::memcpy(c, d_, s_, 48); };
        { let v_ = { let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1568); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14316); c.m.w32(a_, (v_) as u32); };
        l_stageCols_4 = 6i32;
        l_stageRows_5 = 6i32;
        { let v_ = rt::rem_i32({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1576); (c.m.r32(a_) as i32) }, l_stageCols_4); let a_ = { let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(1364); c.m.r32(a_) }; c.m.w32(a_, (v_) as u32); };
        { let v_ = rt::div_i32({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1576); (c.m.r32(a_) as i32) }, l_stageRows_5); let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(1364); c.m.r32(a_) }).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        {
            l_i_2 = 0i32;
            'l27: loop {
                if !(((((l_i_2) < ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14352); (c.m.r32(a_) as i32) })) as i32)) != 0) { break 'l27; }
                'c28: {
                    if ((((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14344); c.m.r32(a_) }).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) == ({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1580); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                        { let v_ = l_i_2; let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(1364); c.m.r32(a_) }).wrapping_add(76); c.m.w32(a_, (v_) as u32); };
                    }
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        { let v_ = 4i32; let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(768); c.m.w32(a_, (v_) as u32); };
        {
            l_i_2 = 0i32;
            'l29: loop {
                if !(((((l_i_2) < (2i32)) as i32)) != 0) { break 'l29; }
                'c30: {
                    if ((((!(({ let a0_ = { let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1376)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); c.m.r32(a_) }; let a1_ = ({ let a_ = { let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); c.m.r32(a_) }; (c.m.r32(a_) as i32) }).wrapping_add(({ let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(7i32)); F::ChrGrid_IsSelectable(c, a0_, a1_) }) != 0)) as i32)) != 0) {
                        { let _ = { let a0_ = { let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); c.m.r32(a_) }; F::memset(c, a0_, 0i32, ((48i32) as u32)) }; };
                        { let v_ = l_i_2; let a_ = { let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); c.m.r32(a_) }; c.m.w32(a_, (v_) as u32); };
                        { let s_ = { let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); c.m.r32(a_) }; let d_ = ((c.m.r32(sym::gProgress)).wrapping_add(1088)).wrapping_add((((l_i_2) as u32)).wrapping_mul(240)); rt::memcpy(c, d_, s_, 48); };
                    }
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        if ((((!(({ let a0_ = { let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14340); c.m.r32(a_) }; let a1_ = { let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1576); (c.m.r32(a_) as i32) }; F::StgGrid_IsSelectable(c, a0_, a1_) }) != 0)) as i32)) != 0) {
            { let v_ = 0i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1576); c.m.w32(a_, (v_) as u32); };
            { let v_ = 0i32; let a_ = { let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(1364); c.m.r32(a_) }; c.m.w32(a_, (v_) as u32); };
            { let v_ = 0i32; let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(1364); c.m.r32(a_) }).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        }
        F::CharSel_SetRowChips(c, 0i32);
        F::CharSel_SetRowChips(c, 1i32);
        F::CharSel_SetStageChips(c);
        { let v_ = { let a_ = (({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(48)).wrapping_add(((({ let a_ = { let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }; (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(108); c.m.w32(a_, (v_) as u32); };
        { let v_ = { let a_ = (({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(48)).wrapping_add(((({ let a_ = { let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }; (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(108); c.m.w32(a_, (v_) as u32); };
        { let v_ = { let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14340); c.m.r32(a_) }).wrapping_add((((({ let a_ = { let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(1364); c.m.r32(a_) }; (c.m.r32(a_) as i32) }).wrapping_add(({ let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(1364); c.m.r32(a_) }).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(6i32))) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(1364); c.m.r32(a_) }).wrapping_add(60); c.m.w32(a_, (v_) as u32); };
        { let v_ = { let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(1364); c.m.r32(a_) }).wrapping_add(76); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(1364); c.m.r32(a_) }).wrapping_add(80); c.m.w32(a_, (v_) as u32); };
        {
            l_i_2 = 0i32;
            'l31: loop {
                if !(((((l_i_2) < (2i32)) as i32)) != 0) { break 'l31; }
                'c32: {
                    { let _ = { let a0_ = ({ let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(108); (c.m.r32(a_) as i32) }).wrapping_add(761i32); let a1_ = { let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(8)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); c.m.r32(a_) }; F::File_LoadSync(c, a0_, a1_, 92160i32) }; };
                    { let _ = { let a0_ = { let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(8)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); c.m.r32(a_) }; let a1_ = { let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(16)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); c.m.r32(a_) }; F::Sprite_Unpack(c, a0_, a1_, ((0i32) as u32)) }; };
                    { let v_ = { let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(16)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); c.m.r32(a_) }; let a_ = fp; c.m.w32(a_, v_); };
                    { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(280)).wrapping_add(((((if (((((l_i_2) != (0i32)) as i32)) != 0) { 22i32 } else { 23i32 })) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let r_ = 2i32; let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(104); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        { let _ = { let a0_ = (925i32).wrapping_add((if ((((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(1364); c.m.r32(a_) }).wrapping_add(60); (c.m.r32(a_) as i32) }) < (36i32)) as i32)) != 0) { { let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(1364); c.m.r32(a_) }).wrapping_add(60); (c.m.r32(a_) as i32) } } else { ({ let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(1364); c.m.r32(a_) }).wrapping_add(60); (c.m.r32(a_) as i32) }).wrapping_sub(36i32) })); let a1_ = { let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(24); c.m.r32(a_) }; F::File_LoadSync(c, a0_, a1_, 243712i32) }; };
        { let _ = { let a0_ = { let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(24); c.m.r32(a_) }; let a1_ = { let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(28)).wrapping_add(((({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(780); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.r32(a_) }; F::Sprite_Unpack(c, a0_, a1_, ((0i32) as u32)) }; };
        { let v_ = { let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(28)).wrapping_add(((({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(780); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.r32(a_) }; let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = c.m.r32(fp); let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(272)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = c.m.r32(fp); let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(272)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = 4i32; let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(772); c.m.w32(a_, (v_) as u32); };
        { let r_ = 64i32; let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(764); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        { let v_ = 128i32; let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(14320)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(14320)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = 1i32; let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(780); c.m.w32(a_, (v_) as u32); };
        if ((((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14344); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(1364); c.m.r32(a_) }).wrapping_add(80); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) == (24i32)) as i32)) != 0) {
            { let a0_ = (((((68382i32) as u32)).wrapping_add(F::Rand_Range(c, ((9i32) as u32)))) as i32); F::Bgm_Play(c, a0_) };
        } else {
            { let a0_ = (if ((((((((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14344); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(1364); c.m.r32(a_) }).wrapping_add(80); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) >= (26i32)) as i32)) != 0) && ((((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14344); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(1364); c.m.r32(a_) }).wrapping_add(80); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) < ((26i32).wrapping_add(40i32))) as i32)) != 0)) as i32)) != 0) { ((3400i32).wrapping_add(73000i32)).wrapping_add(({ let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14344); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(1364); c.m.r32(a_) }).wrapping_add(80); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_sub(26i32)) } else { (68374i32).wrapping_add({ let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14344); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(1364); c.m.r32(a_) }).wrapping_add(80); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) }); F::Bgm_Play(c, a0_) };
        }
        {
            l_i_2 = 0i32;
            'l33: loop {
                if !(((((l_i_2) < (2i32)) as i32)) != 0) { break 'l33; }
                'c34: {
                    { let a0_ = ((c.m.r32(sym::gCharSel)).wrapping_add(14360)).wrapping_add((((l_i_2) as u32)).wrapping_mul(140)); let a1_ = { let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(40); c.m.r32(a_) }; F::TextBox_Init(c, a0_, a1_, (((l_i_2).wrapping_add(1i32)) as u32)) };
                    { let a0_ = ((c.m.r32(sym::gCharSel)).wrapping_add(14360)).wrapping_add((((l_i_2) as u32)).wrapping_mul(140)); F::TextBox_SetUnk80(c, a0_, 1i32) };
                    { let a0_ = ((c.m.r32(sym::gCharSel)).wrapping_add(14640)).wrapping_add((((l_i_2) as u32)).wrapping_mul(140)); let a1_ = { let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(44); c.m.r32(a_) }; F::TextBox_Init(c, a0_, a1_, (((l_i_2).wrapping_add(3i32)) as u32)) };
                    { let a0_ = ((c.m.r32(sym::gCharSel)).wrapping_add(14640)).wrapping_add((((l_i_2) as u32)).wrapping_mul(140)); F::TextBox_SetUnk80(c, a0_, 1i32) };
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "CharSel_Init", &PROBE_N, &[], true); }
    r_
}

pub fn CharSel_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "CharSel_Term", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let r_ = 'f: {
    { let v_ = (1i32).wrapping_neg(); let a_ = sym::gUiNameIdx; c.m.w32(a_, (v_) as u32); };
    { let v_ = (1i32).wrapping_neg(); let a_ = sym::gUiSongIdx; c.m.w32(a_, (v_) as u32); };
    F::IconWin_Term(c);
    F::ItemHelp_Term(c);
    F::ItemPanel_Term(c, 1i32);
    F::ItemPanel_Term(c, 0i32);
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (5i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let a0_ = ((c.m.r32(sym::gCharSel)).wrapping_add(52)).wrapping_add((((l_i_1) as u32)).wrapping_mul(44)); F::Flash_Destroy(c, a0_) };
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
    if ((((({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(28)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = { let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(28)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(28)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    }
    if ((((({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(28)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = { let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(28)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(28)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    }
    if ((((({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(24); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = { let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(24); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(24); c.m.w32(a_, v_); };
    }
    if ((((({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = { let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    }
    if ((((({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = { let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    }
    if ((((({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(8)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = { let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(8)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(8)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    }
    if ((((({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = { let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    }
    if ((((({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(4); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = { let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(4); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(4); c.m.w32(a_, v_); };
    }
    if (((((c.m.r32(sym::gCharSel)) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32(sym::gCharSel); F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = sym::gCharSel; c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "CharSel_Term", &PROBE_N, &[], true); }
    r_
}

pub fn CharSel_Draw(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "CharSel_Draw", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(272) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 272);
    let mut l_i_1: i32 = 0;
    let mut l_j_2: i32 = 0;
    let mut l_f_3: u32 = 0;
    let mut l_nameId_4: i32 = 0;
    let mut l_idx_5: i32 = 0;
    let mut l_bgm_6: i32 = 0;
    let mut l_idx_7: i32 = 0;
    let mut l_have_8: i32 = 0;
    let mut l_shown_9: i32 = 0;
    let mut l_plate_10: u32 = 0;
    let r_ = 'f: {
        { let v_ = (1i32).wrapping_neg(); let a_ = sym::gUiNameIdx; c.m.w32(a_, (v_) as u32); };
        { let v_ = (1i32).wrapping_neg(); let a_ = sym::gUiSongIdx; c.m.w32(a_, (v_) as u32); };
        {
            l_i_1 = 0i32;
            'l1: loop {
                if !(((((l_i_1) < (2i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    if (((({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(764); (c.m.r32(a_) as i32) }) & (64i32))) != 0) {
                        's3: {
                            let sv11: i32 = l_i_1;
                            match sv11 {
                                0i32 => {
                                    { let r_ = 5i32; let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(14320)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                                    if ((((({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(14320)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) >= (128i32)) as i32)) != 0) {
                                        { let v_ = 128i32; let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(14320)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                    }
                                    break 's3;
                                }
                                1i32 => {
                                    { let r_ = 5i32; let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(14320)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); let n_ = ((c.m.r32(a_) as i32)).wrapping_sub(r_); c.m.w32(a_, (n_) as u32); };
                                    if ((((({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(14320)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) < (0i32)) as i32)) != 0) {
                                        { let v_ = 0i32; let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(14320)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                    }
                                    break 's3;
                                }
                                _ => {
                                }
                            }
                        }
                    } else {
                        if (((((l_i_1) == (0i32)) as i32)) != 0) {
                            { let v_ = 0i32; let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(14320)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                            break 'c2;
                        } else {
                            if (((((l_i_1) == (1i32)) as i32)) != 0) {
                                { let v_ = 128i32; let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(14320)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                            }
                        }
                    }
                    { let a0_ = { let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(272)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }; let a3_ = { let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(14320)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; F::Sprite_DrawPicture(c, a0_, 0i32, 0i32, a3_) };
                }
                { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
            }
        }
        l_f_3 = ((c.m.r32(sym::gCharSel)).wrapping_add(52)).wrapping_add((((0i32) as u32)).wrapping_mul(44));
        {
            l_i_1 = 0i32;
            'l4: loop {
                if !(((((l_i_1) < (2i32)) as i32)) != 0) { break 'l4; }
                'c5: {
                    F::Flash_FindLabel(c, l_f_3, ((0i32) as u32), (if ((l_i_1) != 0) { sym::menu_c_e__str19 } else { sym::menu_c_e__str20 }), fp);
                    if ((((({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14356); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
                        { let _ = F::Flash_ClipSetFlags(c, l_f_3, fp, 258i32, ((((1i32) as i64 as u64)) as u8)); };
                    } else {
                        { let _ = F::Flash_ClipSetFlags(c, l_f_3, fp, 258i32, ((((0i32) as i64 as u64)) as u8)); };
                    }
                    if (((({ let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(104); (c.m.r32(a_) as i32) }) & (2i32))) != 0) {
                        { let r_ = F32(0x3d999999); let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(14328)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                        if (((rt::fge({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(14328)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }, F32(0x3f800000)) as i32)) != 0) {
                            { let v_ = F32(0x3f800000); let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(14328)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                        }
                    } else {
                        { let v_ = F32(0x00000000); let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(14328)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    }
                    F::Flash_FindLabel(c, l_f_3, ((0i32) as u32), (if ((l_i_1) != 0) { sym::menu_c_e__str21 } else { sym::menu_c_e__str22 }), fp);
                    { let a2_ = { let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(14328)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); F32(c.m.r32(a_)) }; F::Flash_ClipSetAlpha(c, l_f_3, fp, a2_) };
                    { let a3_ = (((({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14356); (c.m.r32(a_) as i32) }) as u8)) as i32); { let _ = F::Flash_ClipSetFlags(c, l_f_3, fp, 128i32, ((((a3_) as i64 as u64)) as u8)); } };
                    F::Flash_FindLabel(c, l_f_3, ((0i32) as u32), (if ((l_i_1) != 0) { sym::menu_c_e__str23 } else { sym::menu_c_e__str24 }), fp);
                    { let a4_ = { let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(108); (c.m.r32(a_) as i32) }; let a5_ = ((c.m.r32(sym::gCharSel)).wrapping_add(14360)).wrapping_add((((l_i_1) as u32)).wrapping_mul(140)); F::TextBox_AttachLine(c, l_f_3, fp, 0i32, 0i32, a4_, a5_) };
                    F::Flash_FindLabel(c, l_f_3, ((0i32) as u32), (if ((l_i_1) != 0) { sym::menu_c_e__str25 } else { sym::menu_c_e__str26 }), fp);
                    { let a4_ = { let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(108); (c.m.r32(a_) as i32) }; let a5_ = ((c.m.r32(sym::gCharSel)).wrapping_add(14640)).wrapping_add((((l_i_1) as u32)).wrapping_mul(140)); F::TextBox_AttachLine(c, l_f_3, fp, 0i32, 0i32, a4_, a5_) };
                }
                { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
            }
        }
        { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 512i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        F::Flash_FindLabel(c, l_f_3, ((0i32) as u32), sym::menu_c_e__str27, fp);
        {
            l_nameId_4 = { let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(1364); c.m.r32(a_) }).wrapping_add(60); (c.m.r32(a_) as i32) };
            if (((((l_nameId_4) >= (36i32)) as i32)) != 0) {
                { let v_ = 0i32; c.m.w32(fp.wrapping_add(92), (v_) as u32); }
                { let v_ = 0i32; c.m.w32(fp.wrapping_add(96), (v_) as u32); }
                l_idx_5 = (l_nameId_4).wrapping_sub(36i32);
                { let _ = F::Flash_ClipSetFlags(c, l_f_3, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
                { let v_ = 1i32; let a_ = sym::menu_c_e__sPortStageNameHidden; c.m.w32(a_, (v_) as u32); };
                F::Flash_ClipGetPos(c, l_f_3, fp, fp.wrapping_add(92), fp.wrapping_add(96));
                if ((((((((((((((c.m.r32(fp) as i32)) >= (0i32)) as i32)) != 0) && (((((l_idx_5) >= (0i32)) as i32)) != 0)) as i32)) != 0) && (((((l_idx_5) < ((c.m.r32(sym::gPortExtraStageCount) as i32))) as i32)) != 0)) as i32)) != 0) {
                    if (((c.m.r32(sym::gUiNameReady) as i32)) != 0) {
                        { let v_ = ((c.m.r32(fp.wrapping_add(92)) as i32)).wrapping_add(64i32); let a_ = sym::gUiNameX; c.m.w32(a_, (v_) as u32); };
                        { let v_ = ((c.m.r32(fp.wrapping_add(96)) as i32)).wrapping_add(14i32); let a_ = sym::gUiNameY; c.m.w32(a_, (v_) as u32); };
                        { let v_ = 384i32; let a_ = sym::gUiNameW; c.m.w32(a_, (v_) as u32); };
                        { let v_ = 48i32; let a_ = sym::gUiNameH; c.m.w32(a_, (v_) as u32); };
                        { let v_ = l_idx_5; let a_ = sym::gUiNameIdx; c.m.w32(a_, (v_) as u32); };
                    } else {
                        { let s_ = sym::gFontStyle; rt::memcpy(c, fp.wrapping_add(100), s_, 72); }
                        { let v_ = ((1i32) as u32); let a_ = (sym::gFontStyle).wrapping_add(28); c.m.w32(a_, v_); };
                        { let v_ = 4294967295u32; let a_ = (sym::gFontStyle).wrapping_add(12); c.m.w32(a_, v_); };
                        { let v_ = ((1i32) as u32); let a_ = (sym::gFontStyle).wrapping_add(36); c.m.w32(a_, v_); };
                        { let v_ = 3221225472u32; let a_ = (sym::gFontStyle).wrapping_add(40); c.m.w32(a_, v_); };
                        { let v_ = ((1i32) as u16); let a_ = (sym::gFontStyle).wrapping_add(44); c.m.w16(a_, v_); };
                        { let v_ = ((1i32) as u16); let a_ = (sym::gFontStyle).wrapping_add(46); c.m.w16(a_, v_); };
                        { let a0_ = ((c.m.r32(fp.wrapping_add(92)) as i32)).wrapping_add(256i32); let a1_ = ((c.m.r32(fp.wrapping_add(96)) as i32)).wrapping_add(rt::div_i32((64i32).wrapping_sub(F::Font_GetGlyphHeight(c)), 2i32)); F::Font_PrintAsciiAt(c, a0_, a1_, (sym::gPortStageNames).wrapping_add((((l_idx_5) as u32)).wrapping_mul(64))) };
                        { let s_ = fp.wrapping_add(100); let d_ = sym::gFontStyle; rt::memcpy(c, d_, s_, 72); };
                    }
                }
            } else {
                { let v_ = (rt::rem_i32(l_nameId_4, 4i32)).wrapping_mul(64i32); let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                { let v_ = ((c.m.r32((fp.wrapping_add(8)).wrapping_add(4)) as i32)).wrapping_add(64i32); let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                { let v_ = rt::div_i32(l_nameId_4, 4i32); let a_ = (fp.wrapping_add(8)).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
                F::Flash_ClipSetUv(c, l_f_3, fp, fp.wrapping_add(8));
                { let a2_ = (c.m.r32((fp.wrapping_add(8)).wrapping_add(16)) as i32); F::Flash_ClipSetTex(c, l_f_3, fp, a2_) };
                if (((c.m.r32(sym::menu_c_e__sPortStageNameHidden) as i32)) != 0) {
                    { let _ = F::Flash_ClipSetFlags(c, l_f_3, fp, 2i32, ((((1i32) as i64 as u64)) as u8)); };
                    { let v_ = 0i32; let a_ = sym::menu_c_e__sPortStageNameHidden; c.m.w32(a_, (v_) as u32); };
                }
            }
        }
        F::Flash_FindLabel(c, l_f_3, ((0i32) as u32), sym::menu_c_e__str28, fp);
        if ((((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(1364); c.m.r32(a_) }).wrapping_add(68); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
            { let _ = F::Flash_ClipSetFlags(c, l_f_3, fp, 258i32, ((((1i32) as i64 as u64)) as u8)); };
        } else {
            { let _ = F::Flash_ClipSetFlags(c, l_f_3, fp, 258i32, ((((0i32) as i64 as u64)) as u8)); };
        }
        {
            l_i_1 = 0i32;
            'l6: loop {
                if !(((((l_i_1) < (12i32)) as i32)) != 0) { break 'l6; }
                'c7: {
                    { let _ = F::sprintf(c, fp.wrapping_add(28), sym::menu_c_e__str29, &[((l_i_1) as i64 as u64)]); };
                    F::Flash_FindLabel(c, l_f_3, ((0i32) as u32), fp.wrapping_add(28), fp);
                    { let a3_ = (((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(1364); c.m.r32(a_) }).wrapping_add(68); (c.m.r32(a_) as i32) }) as u8)) as i32); { let _ = F::Flash_ClipSetFlags(c, l_f_3, fp, 128i32, ((((a3_) as i64 as u64)) as u8)); } };
                }
                { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
            }
        }
        {
            l_i_1 = 0i32;
            'l8: loop {
                if !(((((l_i_1) < (2i32)) as i32)) != 0) { break 'l8; }
                'c9: {
                    F::Flash_FindLabel(c, l_f_3, (if ((l_i_1) != 0) { sym::menu_c_e__str30 } else { sym::menu_c_e__str31 }), (if ((l_i_1) != 0) { sym::menu_c_e__str32 } else { sym::menu_c_e__str33 }), fp);
                    if ((((((((({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14312); (c.m.r32(a_) as i32) }) >= (2i32)) as i32)) != 0) || ((((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(1364); c.m.r32(a_) }).wrapping_add(64); (c.m.r32(a_) as i32) }) == (9i32)) as i32)) != 0)) as i32)) != 0) {
                        { let v_ = (l_i_1).wrapping_mul(32i32); let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                        { let v_ = ((l_i_1).wrapping_mul(32i32)).wrapping_add(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                        { let v_ = 32i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                        { let v_ = 64i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                        F::Flash_ClipSetUv(c, l_f_3, fp, fp.wrapping_add(8));
                        { let _ = F::Flash_ClipSetFlags(c, l_f_3, fp, 2i32, ((((1i32) as i64 as u64)) as u8)); };
                    } else {
                        { let _ = F::Flash_ClipSetFlags(c, l_f_3, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
                    }
                }
                { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
            }
        }
        { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 512i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        {
            l_bgm_6 = { let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14344); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(1364); c.m.r32(a_) }).wrapping_add(76); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
            if (((((l_bgm_6) >= (26i32)) as i32)) != 0) {
                l_idx_7 = (l_bgm_6).wrapping_sub(26i32);
                l_have_8 = 0i32;
                l_shown_9 = 0i32;
                { let v_ = 0i32; let a_ = fp.wrapping_add(172); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = fp.wrapping_add(176); c.m.w32(a_, (v_) as u32); };
                F::Flash_FindLabel(c, l_f_3, sym::menu_c_e__str34, sym::menu_c_e__str35, fp);
                if ((((((c.m.r32(fp) as i32)) >= (0i32)) as i32)) != 0) {
                    F::Flash_ClipGetPos(c, l_f_3, fp, fp.wrapping_add(172), fp.wrapping_add(176));
                    l_have_8 = 1i32;
                }
                { let _ = F::Flash_ClipSetFlags(c, l_f_3, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
                F::Flash_FindLabel(c, l_f_3, sym::menu_c_e__str36, sym::menu_c_e__str37, fp);
                if ((((((((!((l_have_8) != 0)) as i32)) != 0) && ((((((c.m.r32(fp) as i32)) >= (0i32)) as i32)) != 0)) as i32)) != 0) {
                    F::Flash_ClipGetPos(c, l_f_3, fp, fp.wrapping_add(172), fp.wrapping_add(176));
                    l_shown_9 = 1i32;
                }
                { let r_ = l_have_8; let n_ = ((l_shown_9) | (r_)); l_shown_9 = n_; };
                { let _ = F::Flash_ClipSetFlags(c, l_f_3, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
                { let v_ = 1i32; let a_ = sym::menu_c_e__sPortSongNameHidden; c.m.w32(a_, (v_) as u32); };
                F::Flash_FindLabel(c, l_f_3, ((0i32) as u32), sym::menu_c_e__str38, fp);
                F::Flash_ClipGetPos(c, l_f_3, fp, fp.wrapping_add(180), fp.wrapping_add(184));
                if ((((!((l_shown_9) != 0)) as i32)) != 0) {
                } else {
                    if (((c.m.r32(sym::gUiSongReady) as i32)) != 0) {
                        { let v_ = ((c.m.r32(fp.wrapping_add(180)) as i32)).wrapping_add((c.m.r32(fp.wrapping_add(172)) as i32)); let a_ = sym::gUiSongX; c.m.w32(a_, (v_) as u32); };
                        { let v_ = ((c.m.r32(fp.wrapping_add(184)) as i32)).wrapping_add((c.m.r32(fp.wrapping_add(176)) as i32)); let a_ = sym::gUiSongY; c.m.w32(a_, (v_) as u32); };
                        { let v_ = 512i32; let a_ = sym::gUiSongW; c.m.w32(a_, (v_) as u32); };
                        { let v_ = 32i32; let a_ = sym::gUiSongH; c.m.w32(a_, (v_) as u32); };
                        { let v_ = ((!((l_have_8) != 0)) as i32); let a_ = sym::gUiSongLit; c.m.w32(a_, (v_) as u32); };
                        { let v_ = l_idx_7; let a_ = sym::gUiSongIdx; c.m.w32(a_, (v_) as u32); };
                        {
                            { let a0_ = { let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(1364); c.m.r32(a_) }).wrapping_add(64); (c.m.r32(a_) as i32) }; let a2_ = (c.m.r32(fp.wrapping_add(180)) as i32); let a3_ = (c.m.r32(fp.wrapping_add(184)) as i32); let a4_ = (c.m.r32(fp.wrapping_add(172)) as i32); let a5_ = (c.m.r32(fp.wrapping_add(176)) as i32); F::Port_SongDebug(c, a0_, l_idx_7, a2_, a3_, a4_, a5_) };
                        }
                    } else {
                        if (((((((((l_idx_7) >= (0i32)) as i32)) != 0) && (((((l_idx_7) < ((c.m.r32(sym::gPortSongCount) as i32))) as i32)) != 0)) as i32)) != 0) {
                            { let s_ = sym::gFontStyle; rt::memcpy(c, fp.wrapping_add(188), s_, 72); }
                            { let v_ = ((0i32) as u32); let a_ = (sym::gFontStyle).wrapping_add(28); c.m.w32(a_, v_); };
                            { let v_ = 4294967295u32; let a_ = (sym::gFontStyle).wrapping_add(12); c.m.w32(a_, v_); };
                            { let v_ = ((1i32) as u32); let a_ = (sym::gFontStyle).wrapping_add(36); c.m.w32(a_, v_); };
                            { let v_ = 3221225472u32; let a_ = (sym::gFontStyle).wrapping_add(40); c.m.w32(a_, v_); };
                            { let a0_ = ((c.m.r32(fp.wrapping_add(180)) as i32)).wrapping_add((c.m.r32(fp.wrapping_add(172)) as i32)); let a1_ = (((c.m.r32(fp.wrapping_add(184)) as i32)).wrapping_add((c.m.r32(fp.wrapping_add(176)) as i32))).wrapping_add(rt::div_i32((32i32).wrapping_sub(F::Font_GetGlyphHeight(c)), 2i32)); F::Font_PrintAsciiAt(c, a0_, a1_, (sym::gPortSongNames).wrapping_add((((l_idx_7) as u32)).wrapping_mul(64))) };
                            { let s_ = fp.wrapping_add(188); let d_ = sym::gFontStyle; rt::memcpy(c, d_, s_, 72); };
                        }
                    }
                }
            } else {
                { let v_ = (rt::rem_i32(l_bgm_6, 8i32)).wrapping_mul(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                { let v_ = ((c.m.r32((fp.wrapping_add(8)).wrapping_add(4)) as i32)).wrapping_add(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                { let v_ = rt::div_i32(l_bgm_6, 8i32); let a_ = (fp.wrapping_add(8)).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
                F::Flash_FindLabel(c, l_f_3, sym::menu_c_e__str39, sym::menu_c_e__str40, fp);
                F::Flash_ClipSetUv(c, l_f_3, fp, fp.wrapping_add(8));
                { let a2_ = (c.m.r32((fp.wrapping_add(8)).wrapping_add(16)) as i32); F::Flash_ClipSetTex(c, l_f_3, fp, a2_) };
                if (((c.m.r32(sym::menu_c_e__sPortSongNameHidden) as i32)) != 0) {
                    { let _ = F::Flash_ClipSetFlags(c, l_f_3, fp, 2i32, ((((1i32) as i64 as u64)) as u8)); };
                }
                F::Flash_FindLabel(c, l_f_3, sym::menu_c_e__str41, sym::menu_c_e__str42, fp);
                F::Flash_ClipSetUv(c, l_f_3, fp, fp.wrapping_add(8));
                { let a2_ = (c.m.r32((fp.wrapping_add(8)).wrapping_add(16)) as i32); F::Flash_ClipSetTex(c, l_f_3, fp, a2_) };
                if (((c.m.r32(sym::menu_c_e__sPortSongNameHidden) as i32)) != 0) {
                    { let _ = F::Flash_ClipSetFlags(c, l_f_3, fp, 2i32, ((((1i32) as i64 as u64)) as u8)); };
                    { let v_ = 0i32; let a_ = sym::menu_c_e__sPortSongNameHidden; c.m.w32(a_, (v_) as u32); };
                }
            }
        }
        {
            l_i_1 = 0i32;
            'l10: loop {
                if !(((((l_i_1) < (2i32)) as i32)) != 0) { break 'l10; }
                'c11: {
                    l_f_3 = ((c.m.r32(sym::gCharSel)).wrapping_add(52)).wrapping_add(((((3i32).wrapping_add(l_i_1)) as u32)).wrapping_mul(44));
                    { let v_ = (((l_i_1) ^ (1i32))).wrapping_mul(32i32); let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = ((c.m.r32(fp.wrapping_add(8)) as i32)).wrapping_add(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 32i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                    F::Flash_FindLabel(c, l_f_3, ((0i32) as u32), sym::menu_c_e__str43, fp);
                    F::Flash_ClipSetUv(c, l_f_3, fp, fp.wrapping_add(8));
                    {
                        l_j_2 = 0i32;
                        'l12: loop {
                            if !(((((l_j_2) < (4i32)) as i32)) != 0) { break 'l12; }
                            'c13: {
                                { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                                { let v_ = (l_j_2).wrapping_mul(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                                { let v_ = 256i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                                { let v_ = ((l_j_2).wrapping_mul(32i32)).wrapping_add(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                                { let _ = F::sprintf(c, fp.wrapping_add(28), sym::menu_c_e__str44, &[(((l_j_2).wrapping_add(1i32)) as i64 as u64)]); };
                                F::Flash_FindLabel(c, l_f_3, ((0i32) as u32), fp.wrapping_add(28), fp);
                                { let a2_ = (if (((((l_j_2) < ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(14296)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) })) as i32)) != 0) { F32(0x3f800000) } else { F32(0x3e999999) }); F::Flash_ClipSetColor(c, l_f_3, fp, a2_) };
                                F::Flash_FindLabel(c, l_f_3, fp.wrapping_add(28), sym::menu_c_e__str45, fp);
                                F::Flash_ClipSetUv(c, l_f_3, fp, fp.wrapping_add(8));
                                F::Flash_FindLabel(c, l_f_3, fp.wrapping_add(28), sym::menu_c_e__str46, fp);
                                F::Flash_ClipSetUv(c, l_f_3, fp, fp.wrapping_add(8));
                                if (((((l_j_2) == (0i32)) as i32)) != 0) {
                                    F::Flash_FindLabel(c, l_f_3, fp.wrapping_add(28), sym::menu_c_e__str47, fp);
                                    { let _ = F::Flash_ClipSetFlags(c, l_f_3, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
                                } else {
                                    F::Flash_FindLabel(c, l_f_3, fp.wrapping_add(28), sym::menu_c_e__str48, fp);
                                    { let a3_ = ((({ let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1376)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add((((({ let a_ = { let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }; (c.m.r32(a_) as i32) }).wrapping_add(({ let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(7i32))) as u32)).wrapping_mul(36)); (c.m.r32(a_) as i32) }) != (161i32)) as i32); { let _ = F::Flash_ClipSetFlags(c, l_f_3, fp, 2i32, ((((a3_) as i64 as u64)) as u8)); } };
                                }
                            }
                            { let o_ = l_j_2; let n_ = (o_).wrapping_add(1); l_j_2 = n_; };
                        }
                    }
                    {
                        l_j_2 = 0i32;
                        'l14: loop {
                            if !(((((l_j_2) < (4i32)) as i32)) != 0) { break 'l14; }
                            'c15: {
                                { let v_ = (rt::rem_i32(l_j_2, 2i32)).wrapping_mul(64i32); let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                                { let v_ = (rt::div_i32(l_j_2, 2i32)).wrapping_mul(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                                { let v_ = ((c.m.r32(fp.wrapping_add(8)) as i32)).wrapping_add(64i32); let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                                { let v_ = ((c.m.r32((fp.wrapping_add(8)).wrapping_add(4)) as i32)).wrapping_add(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                                { let _ = F::sprintf(c, fp.wrapping_add(28), sym::menu_c_e__str49, &[(((l_j_2).wrapping_add(1i32)) as i64 as u64)]); };
                                F::Flash_FindLabel(c, l_f_3, ((0i32) as u32), fp.wrapping_add(28), fp);
                                if (((((l_j_2) < ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(14304)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                                    { let _ = F::Flash_ClipSetFlags(c, l_f_3, fp, 2i32, ((((1i32) as i64 as u64)) as u8)); };
                                    's16: {
                                        let sv12: i32 = { let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(14304)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
                                        match sv12 {
                                            2i32 => {
                                                F::Flash_ClipSetOffset(c, l_f_3, fp, 45i32, 0i32);
                                                break 's16;
                                            }
                                            3i32 => {
                                                F::Flash_ClipSetOffset(c, l_f_3, fp, 22i32, 0i32);
                                                break 's16;
                                            }
                                            _ => {
                                            }
                                        }
                                    }
                                    F::Flash_FindLabel(c, l_f_3, fp.wrapping_add(28), sym::menu_c_e__str50, fp);
                                    F::Flash_ClipSetUv(c, l_f_3, fp, fp.wrapping_add(8));
                                    F::Flash_FindLabel(c, l_f_3, fp.wrapping_add(28), sym::menu_c_e__str51, fp);
                                    F::Flash_ClipSetUv(c, l_f_3, fp, fp.wrapping_add(8));
                                } else {
                                    { let _ = F::Flash_ClipSetFlags(c, l_f_3, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
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
            'l17: loop {
                if !(((((l_i_1) < (2i32)) as i32)) != 0) { break 'l17; }
                'c18: {
                    l_plate_10 = ((0i32) as u32);
                    l_f_3 = ((c.m.r32(sym::gCharSel)).wrapping_add(52)).wrapping_add(((((1i32).wrapping_add(l_i_1)) as u32)).wrapping_mul(44));
                    's19: {
                        let sv13: i32 = { let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14316); (c.m.r32(a_) as i32) };
                        match sv13 {
                            0i32 => {
                                l_plate_10 = (((if (((((l_i_1) == (0i32)) as i32)) != 0) { 0i32 } else { 2i32 })) as u32);
                                break 's19;
                            }
                            1i32 => {
                                l_plate_10 = (((((l_i_1) != (0i32)) as i32)) as u32);
                                break 's19;
                            }
                            2i32 => {
                                l_plate_10 = ((2i32) as u32);
                                break 's19;
                            }
                            _ => {
                            }
                        }
                    }
                    { let v_ = ((((l_plate_10).wrapping_sub((rt::div_u32(l_plate_10, ((2i32) as u32))).wrapping_mul(((2i32) as u32)))).wrapping_mul(((64i32) as u32))) as i32); let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = ((((rt::div_u32(l_plate_10, ((2i32) as u32))).wrapping_mul(((32i32) as u32))).wrapping_add(((32i32) as u32))) as i32); let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                    { let v_ = (((((l_plate_10).wrapping_sub((rt::div_u32(l_plate_10, ((2i32) as u32))).wrapping_mul(((2i32) as u32)))).wrapping_mul(((64i32) as u32))).wrapping_add(((64i32) as u32))) as i32); let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = (((rt::div_u32(l_plate_10, ((2i32) as u32))).wrapping_mul(((32i32) as u32))) as i32); let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    F::Flash_FindLabel(c, l_f_3, ((0i32) as u32), sym::menu_c_e__str52, fp);
                    F::Flash_ClipSetUv(c, l_f_3, fp, fp.wrapping_add(8));
                    if ((((({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(14288)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) >= (2i32)) as i32)) != 0) {
                        {
                            l_j_2 = 0i32;
                            'l20: loop {
                                if !(((((l_j_2) < (2i32)) as i32)) != 0) { break 'l20; }
                                'c21: {
                                    { let v_ = 32i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                                    { let v_ = ((l_j_2).wrapping_mul(32i32)).wrapping_add(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                                    { let v_ = 64i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                                    { let v_ = (l_j_2).wrapping_mul(32i32); let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                                    F::Flash_FindLabel(c, l_f_3, (if ((l_j_2) != 0) { sym::menu_c_e__str53 } else { sym::menu_c_e__str54 }), (if ((l_j_2) != 0) { sym::menu_c_e__str55 } else { sym::menu_c_e__str56 }), fp);
                                    F::Flash_ClipSetUv(c, l_f_3, fp, fp.wrapping_add(8));
                                    F::Flash_FindLabel(c, l_f_3, ((0i32) as u32), (if ((l_j_2) != 0) { sym::menu_c_e__str57 } else { sym::menu_c_e__str58 }), fp);
                                    if (((({ let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(104); (c.m.r32(a_) as i32) }) & (64i32))) != 0) {
                                        { let _ = F::Flash_ClipSetFlags(c, l_f_3, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
                                    } else {
                                        if (((((((((l_i_1) == (1i32)) as i32)) != 0) && ((((({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14316); (c.m.r32(a_) as i32) }) != (l_i_1)) as i32)) != 0)) as i32)) != 0) {
                                            if ((((({ let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(112); (c.m.r32(a_) as i32) }) != (7i32)) as i32)) != 0) {
                                                { let _ = F::Flash_ClipSetFlags(c, l_f_3, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
                                            } else {
                                                { let _ = F::Flash_ClipSetFlags(c, l_f_3, fp, 2i32, ((((1i32) as i64 as u64)) as u8)); };
                                            }
                                        } else {
                                            { let _ = F::Flash_ClipSetFlags(c, l_f_3, fp, 2i32, ((((1i32) as i64 as u64)) as u8)); };
                                        }
                                    }
                                }
                                { let o_ = l_j_2; let n_ = (o_).wrapping_add(1); l_j_2 = n_; };
                            }
                        }
                    } else {
                        {
                            l_j_2 = 0i32;
                            'l22: loop {
                                if !(((((l_j_2) < (2i32)) as i32)) != 0) { break 'l22; }
                                'c23: {
                                    F::Flash_FindLabel(c, l_f_3, ((0i32) as u32), (if ((l_j_2) != 0) { sym::menu_c_e__str59 } else { sym::menu_c_e__str60 }), fp);
                                    { let _ = F::Flash_ClipSetFlags(c, l_f_3, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
                                }
                                { let o_ = l_j_2; let n_ = (o_).wrapping_add(1); l_j_2 = n_; };
                            }
                        }
                    }
                    F::Flash_FindLabel(c, l_f_3, ((0i32) as u32), sym::menu_c_e__str61, fp);
                    if ((((({ let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(116); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
                        { let _ = F::Flash_ClipSetFlags(c, l_f_3, fp, 258i32, ((((1i32) as i64 as u64)) as u8)); };
                    } else {
                        { let _ = F::Flash_ClipSetFlags(c, l_f_3, fp, 258i32, ((((0i32) as i64 as u64)) as u8)); };
                    }
                    {
                        l_j_2 = 0i32;
                        'l24: loop {
                            if !(((((l_j_2) < (14i32)) as i32)) != 0) { break 'l24; }
                            'c25: {
                                { let _ = F::sprintf(c, fp.wrapping_add(28), sym::menu_c_e__str62, &[((l_j_2) as i64 as u64)]); };
                                F::Flash_FindLabel(c, l_f_3, ((0i32) as u32), fp.wrapping_add(28), fp);
                                { let a3_ = (((({ let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(116); (c.m.r32(a_) as i32) }) as u8)) as i32); { let _ = F::Flash_ClipSetFlags(c, l_f_3, fp, 128i32, ((((a3_) as i64 as u64)) as u8)); } };
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
            'l26: loop {
                if !(((((l_i_1) < (5i32)) as i32)) != 0) { break 'l26; }
                'c27: {
                    { let a0_ = ((c.m.r32(sym::gCharSel)).wrapping_add(52)).wrapping_add((((l_i_1) as u32)).wrapping_mul(44)); F::Flash_Draw(c, a0_) };
                }
                { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
            }
        }
        F::Font_FlushAll(c);
        F::IconWin_Draw(c);
        F::ItemPanel_Draw(c, 0i32);
        F::ItemPanel_Draw(c, 1i32);
        { let a0_ = { let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(788); (c.m.r32(a_) as i32) }; F::ItemHelp_Draw(c, a0_) };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "CharSel_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn CharSel_Update(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "CharSel_Update", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let r_ = 'f: {
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (5i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let a0_ = ((c.m.r32(sym::gCharSel)).wrapping_add(52)).wrapping_add((((l_i_1) as u32)).wrapping_mul(44)); F::Flash_Advance(c, a0_) };
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
                if ((((({ let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(116); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
                    if (((({ let a_ = ((((c.m.r32(sym::gCharSel)).wrapping_add(52)).wrapping_add((((1i32) as u32)).wrapping_mul(44))).wrapping_add((((l_i_1) as u32)).wrapping_mul(44))).wrapping_add(12); c.m.r32(a_) }) & (((1i32) as u32)))) != 0) {
                        { let v_ = 0i32; let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(116); c.m.w32(a_, (v_) as u32); };
                    }
                }
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
    if ((((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(1364); c.m.r32(a_) }).wrapping_add(68); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
        if (((({ let a_ = (((c.m.r32(sym::gCharSel)).wrapping_add(52)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add(12); c.m.r32(a_) }) & (((1i32) as u32)))) != 0) {
            { let v_ = 0i32; let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(1364); c.m.r32(a_) }).wrapping_add(68); c.m.w32(a_, (v_) as u32); };
        }
    }
    if ((((({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14356); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
        if ((((((({ let a_ = (((c.m.r32(sym::gCharSel)).wrapping_add(52)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add(12); c.m.r32(a_) }) as i32)) < (0i32)) as i32)) != 0) {
            { let v_ = 0i32; let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14356); c.m.w32(a_, (v_) as u32); };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "CharSel_Update", &PROBE_N, &[], true); }
    r_
}

pub fn CharSel_Input(c: &mut Ctx, mut p_running_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "CharSel_Input", &PROBE_N, &[(p_running_1) as u32], false); }
    let mut l_count_2: i32 = 0;
    let mut l_i_3: i32 = 0;
    let mut l_j_4: i32 = 0;
    let mut l_n_5: i32 = 0;
    let mut l_id_6: i32 = 0;
    let mut l_chara_7: i32 = 0;
    let mut l_r_8: i32 = 0;
    let mut l_turn_9: i32 = 0;
    let mut l_pressed_10: u32 = 0;
    let r_ = 'f: {
    l_count_2 = 0i32;
    if ((((!(((({ let a_ = (((c.m.r32(sym::gCharSel)).wrapping_add(52)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add(8); c.m.r32(a_) }) & (((2i32) as u32)))) != 0)) as i32)) != 0) {
        break 'f;
    }
    's1: {
        let sv11: i32 = { let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14316); (c.m.r32(a_) as i32) };
        match sv11 {
            0i32 | 2i32 => {
                l_count_2 = 1i32;
                break 's1;
            }
            1i32 => {
                l_count_2 = 2i32;
                break 's1;
            }
            _ => {
            }
        }
    }
    if ((((!(((({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(764); (c.m.r32(a_) as i32) }) & (2i32))) != 0)) as i32)) != 0) {
        {
            l_i_3 = 0i32;
            'l2: loop {
                if !(((((l_i_3) < (l_count_2)) as i32)) != 0) { break 'l2; }
                'c3: {
                    F::CharSel_ClipGoto(c, (l_i_3).wrapping_add(1i32), l_i_3, 0i32, sym::menu_c_e__str63);
                }
                { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
            }
        }
        { let r_ = 2i32; let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(764); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
    if (((({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(764); (c.m.r32(a_) as i32) }) & (4i32))) != 0) {
        's4: {
            let sv12: i32 = { let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(1364); c.m.r32(a_) }).wrapping_add(64); (c.m.r32(a_) as i32) };
            match sv12 {
                8i32 => {
                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((1i32) as u32)))) != 0) {
                        F::CharSel_ClipGoto(c, 0i32, 0i32, 8i32, sym::menu_c_e__str64);
                        { let a0_ = { let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14340); c.m.r32(a_) }; let a1_ = { let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(1364); c.m.r32(a_) }; let a2_ = { let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(1364); c.m.r32(a_) }).wrapping_add(4); (c.m.r32(a_) as i32) }; F::StgGrid_MoveLeft(c, a0_, a1_, a2_) };
                        F::CharSel_ClipGoto(c, 0i32, 0i32, 8i32, sym::menu_c_e__str65);
                        if ((((({ let a_ = { let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(1364); c.m.r32(a_) }; (c.m.r32(a_) as i32) }) != (6i32)) as i32)) != 0) {
                            { let v_ = { let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14340); c.m.r32(a_) }).wrapping_add((((({ let a_ = { let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(1364); c.m.r32(a_) }; (c.m.r32(a_) as i32) }).wrapping_add(({ let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(1364); c.m.r32(a_) }).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(6i32))) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(1364); c.m.r32(a_) }).wrapping_add(60); c.m.w32(a_, (v_) as u32); };
                            F::CharSel_RequestStage(c);
                        }
                        { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 0i32); };
                    } else {
                        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((2i32) as u32)))) != 0) {
                            F::CharSel_ClipGoto(c, 0i32, 0i32, 8i32, sym::menu_c_e__str66);
                            { let a0_ = { let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14340); c.m.r32(a_) }; let a1_ = { let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(1364); c.m.r32(a_) }; let a2_ = { let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(1364); c.m.r32(a_) }).wrapping_add(4); (c.m.r32(a_) as i32) }; F::StgGrid_MoveRight(c, a0_, a1_, a2_) };
                            F::CharSel_ClipGoto(c, 0i32, 0i32, 8i32, sym::menu_c_e__str67);
                            if ((((({ let a_ = { let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(1364); c.m.r32(a_) }; (c.m.r32(a_) as i32) }) != (6i32)) as i32)) != 0) {
                                { let v_ = { let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14340); c.m.r32(a_) }).wrapping_add((((({ let a_ = { let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(1364); c.m.r32(a_) }; (c.m.r32(a_) as i32) }).wrapping_add(({ let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(1364); c.m.r32(a_) }).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(6i32))) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(1364); c.m.r32(a_) }).wrapping_add(60); c.m.w32(a_, (v_) as u32); };
                                F::CharSel_RequestStage(c);
                            }
                            { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 0i32); };
                        } else {
                            if ((((((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((8i32) as u32)))) != 0) && ((((({ let a_ = { let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(1364); c.m.r32(a_) }; (c.m.r32(a_) as i32) }) != (6i32)) as i32)) != 0)) as i32)) != 0) {
                                if ((((({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14312); (c.m.r32(a_) as i32) }) < (2i32)) as i32)) != 0) {
                                    break 'f;
                                }
                                { let a0_ = ((c.m.r32(sym::gCharSel)).wrapping_add(52)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_c_e__str68, ((((1i32) as i64 as u64)) as u8)) };
                                { let v_ = 1i32; let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(1364); c.m.r32(a_) }).wrapping_add(68); c.m.w32(a_, (v_) as u32); };
                                F::CharSel_ClipGoto(c, 0i32, 0i32, 8i32, sym::menu_c_e__str69);
                                { let a0_ = { let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14340); c.m.r32(a_) }; let a1_ = { let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(1364); c.m.r32(a_) }; let a2_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(1364); c.m.r32(a_) }).wrapping_add(4); let a3_ = { let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14312); (c.m.r32(a_) as i32) }; F::StgGrid_MoveUp(c, a0_, a1_, a2_, a3_) };
                                F::CharSel_ClipGoto(c, 0i32, 0i32, 8i32, sym::menu_c_e__str70);
                                F::CharSel_SetStageChips(c);
                                { let v_ = { let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14340); c.m.r32(a_) }).wrapping_add((((({ let a_ = { let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(1364); c.m.r32(a_) }; (c.m.r32(a_) as i32) }).wrapping_add(({ let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(1364); c.m.r32(a_) }).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(6i32))) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(1364); c.m.r32(a_) }).wrapping_add(60); c.m.w32(a_, (v_) as u32); };
                                F::CharSel_RequestStage(c);
                                { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 2i32); };
                            } else {
                                if ((((((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((4i32) as u32)))) != 0) && ((((({ let a_ = { let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(1364); c.m.r32(a_) }; (c.m.r32(a_) as i32) }) != (6i32)) as i32)) != 0)) as i32)) != 0) {
                                    if ((((({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14312); (c.m.r32(a_) as i32) }) < (2i32)) as i32)) != 0) {
                                        break 'f;
                                    }
                                    { let a0_ = ((c.m.r32(sym::gCharSel)).wrapping_add(52)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_c_e__str71, ((((1i32) as i64 as u64)) as u8)) };
                                    { let v_ = 1i32; let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(1364); c.m.r32(a_) }).wrapping_add(68); c.m.w32(a_, (v_) as u32); };
                                    F::CharSel_ClipGoto(c, 0i32, 0i32, 8i32, sym::menu_c_e__str72);
                                    { let a0_ = { let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14340); c.m.r32(a_) }; let a1_ = { let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(1364); c.m.r32(a_) }; let a2_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(1364); c.m.r32(a_) }).wrapping_add(4); let a3_ = { let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14312); (c.m.r32(a_) as i32) }; F::StgGrid_MoveDown(c, a0_, a1_, a2_, a3_) };
                                    F::CharSel_ClipGoto(c, 0i32, 0i32, 8i32, sym::menu_c_e__str73);
                                    F::CharSel_SetStageChips(c);
                                    { let v_ = { let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14340); c.m.r32(a_) }).wrapping_add((((({ let a_ = { let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(1364); c.m.r32(a_) }; (c.m.r32(a_) as i32) }).wrapping_add(({ let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(1364); c.m.r32(a_) }).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(6i32))) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(1364); c.m.r32(a_) }).wrapping_add(60); c.m.w32(a_, (v_) as u32); };
                                    F::CharSel_RequestStage(c);
                                    { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 1i32); };
                                } else {
                                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                                        { let v_ = 0i32; let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(1364); c.m.r32(a_) }).wrapping_add(68); c.m.w32(a_, (v_) as u32); };
                                        F::CharSel_ClipGoto(c, 0i32, 0i32, 8i32, sym::menu_c_e__str74);
                                        if ((((({ let a_ = { let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(1364); c.m.r32(a_) }; (c.m.r32(a_) as i32) }) == (6i32)) as i32)) != 0) {
                                            { let a0_ = ((c.m.r32(sym::gCharSel)).wrapping_add(52)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_c_e__str75, ((((1i32) as i64 as u64)) as u8)) };
                                            F::IconWin_SetIcon(c, 1i32);
                                            F::CharSel_ClipGoto(c, 0i32, 0i32, 9i32, sym::menu_c_e__str76);
                                            { let v_ = { let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(1364); c.m.r32(a_) }).wrapping_add(80); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(1364); c.m.r32(a_) }).wrapping_add(76); c.m.w32(a_, (v_) as u32); };
                                            { let v_ = 9i32; let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(1364); c.m.r32(a_) }).wrapping_add(64); c.m.w32(a_, (v_) as u32); };
                                        } else {
                                            { let a0_ = ((c.m.r32(sym::gCharSel)).wrapping_add(52)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_c_e__str77, ((((1i32) as i64 as u64)) as u8)) };
                                            F::IconWin_Close(c);
                                            { let v_ = 0i32; let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14356); c.m.w32(a_, (v_) as u32); };
                                            { let r_ = 8i32; let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(764); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                                            { let r_ = 16i32; let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(764); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                                            { let v_ = 60i32; let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(784); c.m.w32(a_, (v_) as u32); };
                                        }
                                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                                    } else {
                                        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1024i32) as u32)))) != 0) {
                                            { let v_ = 0i32; let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(1364); c.m.r32(a_) }).wrapping_add(68); c.m.w32(a_, (v_) as u32); };
                                            { let r_ = 4i32; let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(764); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
                                            { let a0_ = ((c.m.r32(sym::gCharSel)).wrapping_add(52)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_c_e__str78, ((((1i32) as i64 as u64)) as u8)) };
                                            F::IconWin_Close(c);
                                            F::CharSel_ClipGoto(c, 0i32, 0i32, 8i32, sym::menu_c_e__str79);
                                            if ((((({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14316); (c.m.r32(a_) as i32) }) != (1i32)) as i32)) != 0) {
                                                { let a0_ = ((c.m.r32(sym::gCharSel)).wrapping_add(52)).wrapping_add((((2i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_c_e__str80, ((((1i32) as i64 as u64)) as u8)) };
                                                if (((({ let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(104); (c.m.r32(a_) as i32) }) & (64i32))) != 0) {
                                                    F::CharSel_ClipGoto(c, 2i32, 1i32, 1i32, sym::menu_c_e__str81);
                                                    { let v_ = 1i32; let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(112); c.m.w32(a_, (v_) as u32); };
                                                } else {
                                                    if (((({ let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(104); (c.m.r32(a_) as i32) }) & (128i32))) != 0) {
                                                        F::CharSel_ClipGoto(c, 2i32, 1i32, 6i32, sym::menu_c_e__str82);
                                                        { let v_ = 1i32; let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(112); c.m.w32(a_, (v_) as u32); };
                                                    } else {
                                                        F::CharSel_ClipGoto(c, 2i32, 1i32, 0i32, sym::menu_c_e__str83);
                                                        { let v_ = 0i32; let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(112); c.m.w32(a_, (v_) as u32); };
                                                    }
                                                }
                                            } else {
                                                {
                                                    l_j_4 = 0i32;
                                                    'l5: loop {
                                                        if !(((((l_j_4) < (2i32)) as i32)) != 0) { break 'l5; }
                                                        'c6: {
                                                            { let a0_ = ((c.m.r32(sym::gCharSel)).wrapping_add(52)).wrapping_add(((((1i32).wrapping_add(l_j_4)) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_c_e__str84, ((((1i32) as i64 as u64)) as u8)) };
                                                            if (((({ let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add((((l_j_4) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(104); (c.m.r32(a_) as i32) }) & (64i32))) != 0) {
                                                                F::CharSel_ClipGoto(c, (l_j_4).wrapping_add(1i32), l_j_4, 1i32, sym::menu_c_e__str85);
                                                                { let v_ = 1i32; let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add((((l_j_4) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(112); c.m.w32(a_, (v_) as u32); };
                                                            } else {
                                                                if (((({ let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add((((l_j_4) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(104); (c.m.r32(a_) as i32) }) & (128i32))) != 0) {
                                                                    F::CharSel_ClipGoto(c, (l_j_4).wrapping_add(1i32), l_j_4, 6i32, sym::menu_c_e__str86);
                                                                    { let v_ = 1i32; let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add((((l_j_4) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(112); c.m.w32(a_, (v_) as u32); };
                                                                } else {
                                                                    F::CharSel_ClipGoto(c, (l_j_4).wrapping_add(1i32), l_j_4, 0i32, sym::menu_c_e__str87);
                                                                    { let v_ = 0i32; let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add((((l_j_4) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(112); c.m.w32(a_, (v_) as u32); };
                                                                }
                                                            }
                                                        }
                                                        { let o_ = l_j_4; let n_ = (o_).wrapping_add(1); l_j_4 = n_; };
                                                    }
                                                }
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
                9i32 => {
                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((8i32) as u32)))) != 0) {
                        { let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(1364); c.m.r32(a_) }).wrapping_add(76); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                        if ((((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(1364); c.m.r32(a_) }).wrapping_add(76); (c.m.r32(a_) as i32) }) < (0i32)) as i32)) != 0) {
                            { let v_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14352); (c.m.r32(a_) as i32) }).wrapping_sub(1i32); let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(1364); c.m.r32(a_) }).wrapping_add(76); c.m.w32(a_, (v_) as u32); };
                        }
                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                    } else {
                        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((4i32) as u32)))) != 0) {
                            { let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(1364); c.m.r32(a_) }).wrapping_add(76); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                            if ((((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(1364); c.m.r32(a_) }).wrapping_add(76); (c.m.r32(a_) as i32) }) > (({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14352); (c.m.r32(a_) as i32) }).wrapping_sub(1i32))) as i32)) != 0) {
                                { let v_ = 0i32; let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(1364); c.m.r32(a_) }).wrapping_add(76); c.m.w32(a_, (v_) as u32); };
                            }
                            { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                        } else {
                            if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                                if ((((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14344); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(1364); c.m.r32(a_) }).wrapping_add(76); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) != (25i32)) as i32)) != 0) {
                                    { let a0_ = ((c.m.r32(sym::gCharSel)).wrapping_add(52)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_c_e__str88, ((((1i32) as i64 as u64)) as u8)) };
                                    F::IconWin_SetIcon(c, 0i32);
                                    F::CharSel_ClipGoto(c, 0i32, 0i32, 9i32, sym::menu_c_e__str89);
                                    F::CharSel_ClipGoto(c, 0i32, 0i32, 8i32, sym::menu_c_e__str90);
                                    { let v_ = { let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(1364); c.m.r32(a_) }).wrapping_add(76); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(1364); c.m.r32(a_) }).wrapping_add(80); c.m.w32(a_, (v_) as u32); };
                                    if ((((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14344); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(1364); c.m.r32(a_) }).wrapping_add(80); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) != (24i32)) as i32)) != 0) {
                                        { let a0_ = (if ((((((((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14344); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(1364); c.m.r32(a_) }).wrapping_add(80); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) >= (26i32)) as i32)) != 0) && ((((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14344); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(1364); c.m.r32(a_) }).wrapping_add(80); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) < ((26i32).wrapping_add(40i32))) as i32)) != 0)) as i32)) != 0) { ((3400i32).wrapping_add(73000i32)).wrapping_add(({ let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14344); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(1364); c.m.r32(a_) }).wrapping_add(80); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_sub(26i32)) } else { (68374i32).wrapping_add({ let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14344); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(1364); c.m.r32(a_) }).wrapping_add(80); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) }); F::Bgm_Play(c, a0_) };
                                    }
                                    { let v_ = 8i32; let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(1364); c.m.r32(a_) }).wrapping_add(64); c.m.w32(a_, (v_) as u32); };
                                    { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                                } else {
                                    { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 7i32); };
                                }
                            } else {
                                if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1024i32) as u32)))) != 0) {
                                    { let a0_ = ((c.m.r32(sym::gCharSel)).wrapping_add(52)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_c_e__str91, ((((1i32) as i64 as u64)) as u8)) };
                                    F::IconWin_SetIcon(c, 0i32);
                                    F::CharSel_ClipGoto(c, 0i32, 0i32, 9i32, sym::menu_c_e__str92);
                                    F::CharSel_ClipGoto(c, 0i32, 0i32, 8i32, sym::menu_c_e__str93);
                                    { let v_ = { let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(1364); c.m.r32(a_) }).wrapping_add(80); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(1364); c.m.r32(a_) }).wrapping_add(76); c.m.w32(a_, (v_) as u32); };
                                    { let v_ = 8i32; let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(1364); c.m.r32(a_) }).wrapping_add(64); c.m.w32(a_, (v_) as u32); };
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
            l_i_3 = 0i32;
            'l7: loop {
                if !(((((l_i_3) < (l_count_2)) as i32)) != 0) { break 'l7; }
                'c8: {
                    if (((({ let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) ^ (1i32))) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(104); (c.m.r32(a_) as i32) }) & (2048i32))) != 0) {
                        break 'c8;
                    }
                    's9: {
                        let sv13: i32 = { let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(112); (c.m.r32(a_) as i32) };
                        match sv13 {
                            0i32 => {
                                if ((((c.m.r32(((sym::gPad).wrapping_add((((l_i_3) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((1i32) as u32)))) != 0) {
                                    { let a0_ = ((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })).wrapping_add(1i32); let a1_ = (l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }); F::CharSel_ClipGoto(c, a0_, a1_, 0i32, sym::menu_c_e__str94) };
                                    { let _ = { let a0_ = { let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1376)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }; let a1_ = { let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }; let a2_ = { let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(4); (c.m.r32(a_) as i32) }; F::ChrGrid_MoveLeft(c, a0_, a1_, a2_) }; };
                                    { let a0_ = ((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })).wrapping_add(1i32); let a1_ = (l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }); F::CharSel_ClipGoto(c, a0_, a1_, 0i32, sym::menu_c_e__str95) };
                                    { let v_ = { let a_ = (({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(48)).wrapping_add(((({ let a_ = { let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }; (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(108); c.m.w32(a_, (v_) as u32); };
                                    { let a0_ = (l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }); F::CharSel_RequestFace(c, a0_) };
                                    { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 0i32); };
                                } else {
                                    if ((((c.m.r32(((sym::gPad).wrapping_add((((l_i_3) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((2i32) as u32)))) != 0) {
                                        { let a0_ = ((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })).wrapping_add(1i32); let a1_ = (l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }); F::CharSel_ClipGoto(c, a0_, a1_, 0i32, sym::menu_c_e__str96) };
                                        { let _ = { let a0_ = { let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1376)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }; let a1_ = { let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }; let a2_ = { let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(4); (c.m.r32(a_) as i32) }; F::ChrGrid_MoveRight(c, a0_, a1_, a2_) }; };
                                        { let a0_ = ((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })).wrapping_add(1i32); let a1_ = (l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }); F::CharSel_ClipGoto(c, a0_, a1_, 0i32, sym::menu_c_e__str97) };
                                        { let v_ = { let a_ = (({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(48)).wrapping_add(((({ let a_ = { let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }; (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(108); c.m.w32(a_, (v_) as u32); };
                                        { let a0_ = (l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }); F::CharSel_RequestFace(c, a0_) };
                                        { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 0i32); };
                                    } else {
                                        if ((((c.m.r32(((sym::gPad).wrapping_add((((l_i_3) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((8i32) as u32)))) != 0) {
                                            if ((((({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(14288)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) < (2i32)) as i32)) != 0) {
                                                break 'c8;
                                            }
                                            { let a0_ = ((c.m.r32(sym::gCharSel)).wrapping_add(52)).wrapping_add(((((1i32).wrapping_add((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }))) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_c_e__str98, ((((1i32) as i64 as u64)) as u8)) };
                                            { let v_ = 1i32; let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(116); c.m.w32(a_, (v_) as u32); };
                                            { let a0_ = ((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })).wrapping_add(1i32); let a1_ = (l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }); F::CharSel_ClipGoto(c, a0_, a1_, 0i32, sym::menu_c_e__str99) };
                                            { let a0_ = { let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1376)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }; let a1_ = { let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }; let a2_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(4); let a3_ = { let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(14288)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; F::ChrGrid_MoveUp(c, a0_, a1_, a2_, a3_) };
                                            { let a0_ = ((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })).wrapping_add(1i32); let a1_ = (l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }); F::CharSel_ClipGoto(c, a0_, a1_, 0i32, sym::menu_c_e__str100) };
                                            { let a0_ = (l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }); F::CharSel_SetRowChips(c, a0_) };
                                            { let v_ = { let a_ = (({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(48)).wrapping_add(((({ let a_ = { let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }; (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(108); c.m.w32(a_, (v_) as u32); };
                                            { let a0_ = (l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }); F::CharSel_RequestFace(c, a0_) };
                                            { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 2i32); };
                                        } else {
                                            if ((((c.m.r32(((sym::gPad).wrapping_add((((l_i_3) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((4i32) as u32)))) != 0) {
                                                if ((((({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(14288)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) < (2i32)) as i32)) != 0) {
                                                    break 'c8;
                                                }
                                                { let a0_ = ((c.m.r32(sym::gCharSel)).wrapping_add(52)).wrapping_add(((((1i32).wrapping_add((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }))) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_c_e__str101, ((((1i32) as i64 as u64)) as u8)) };
                                                { let v_ = 1i32; let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(116); c.m.w32(a_, (v_) as u32); };
                                                { let a0_ = ((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })).wrapping_add(1i32); let a1_ = (l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }); F::CharSel_ClipGoto(c, a0_, a1_, 0i32, sym::menu_c_e__str102) };
                                                { let a0_ = { let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1376)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }; let a1_ = { let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }; let a2_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(4); let a3_ = { let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(14288)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; F::ChrGrid_MoveDown(c, a0_, a1_, a2_, a3_) };
                                                { let a0_ = ((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })).wrapping_add(1i32); let a1_ = (l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }); F::CharSel_ClipGoto(c, a0_, a1_, 0i32, sym::menu_c_e__str103) };
                                                { let a0_ = (l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }); F::CharSel_SetRowChips(c, a0_) };
                                                { let v_ = { let a_ = (({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(48)).wrapping_add(((({ let a_ = { let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }; (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(108); c.m.w32(a_, (v_) as u32); };
                                                { let a0_ = (l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }); F::CharSel_RequestFace(c, a0_) };
                                                { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 1i32); };
                                            } else {
                                                if ((((c.m.r32(((sym::gPad).wrapping_add((((l_i_3) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                                                    if ((((({ let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1376)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add((((({ let a_ = { let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }; (c.m.r32(a_) as i32) }).wrapping_add(({ let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(7i32))) as u32)).wrapping_mul(36)); (c.m.r32(a_) as i32) }) == (161i32)) as i32)) != 0) {
                                                        { let a0_ = ((c.m.r32(sym::gCharSel)).wrapping_add(52)).wrapping_add(((((3i32).wrapping_add((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }))) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_c_e__str104, ((((1i32) as i64 as u64)) as u8)) };
                                                        { let a0_ = ((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })).wrapping_add(1i32); let a1_ = (l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }); F::CharSel_ClipGoto(c, a0_, a1_, 0i32, sym::menu_c_e__str105) };
                                                        { let v_ = 4i32; let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(14296)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                                        { let v_ = 2i32; let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(14304)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                                        if ((((!((((({ let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(24); (c.m.r32(a_) as i32) }) < ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(14304)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) })) as i32)) != 0)) as i32)) != 0) {
                                                            { let v_ = 0i32; let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                                                        }
                                                        { let a0_ = ((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })).wrapping_add(3i32); let a1_ = (l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }); F::CharSel_ClipGoto(c, a0_, a1_, 2i32, sym::menu_c_e__str106) };
                                                        { let v_ = 2i32; let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(112); c.m.w32(a_, (v_) as u32); };
                                                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                                                    } else {
                                                        if ((((({ let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1376)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add((((({ let a_ = { let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }; (c.m.r32(a_) as i32) }).wrapping_add(({ let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(7i32))) as u32)).wrapping_mul(36)); (c.m.r32(a_) as i32) }) == (163i32)) as i32)) != 0) {
                                                            if ((((({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(13272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
                                                                { let _ = { let a0_ = ((c.m.r32(sym::gCharSel)).wrapping_add(13280)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(504)); let a1_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(12); let a2_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(16); F::ChrGrid_FixCursor(c, a0_, a1_, a2_, 2i32) }; };
                                                                { let r_ = 128i32; let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(104); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                                                                { let a0_ = ((c.m.r32(sym::gCharSel)).wrapping_add(52)).wrapping_add(((((1i32).wrapping_add((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }))) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_c_e__str107, ((((1i32) as i64 as u64)) as u8)) };
                                                                { let v_ = 1i32; let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(116); c.m.w32(a_, (v_) as u32); };
                                                                { let a0_ = ((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })).wrapping_add(1i32); let a1_ = (l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }); F::CharSel_ClipGoto(c, a0_, a1_, 0i32, sym::menu_c_e__str108) };
                                                                { let a0_ = ((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })).wrapping_add(1i32); let a1_ = (l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }); F::CharSel_ClipGoto(c, a0_, a1_, 6i32, sym::menu_c_e__str109) };
                                                                { let a0_ = (l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }); F::CharSel_SetCustomChips(c, a0_) };
                                                                { let v_ = { let a_ = (({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(48)).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(12); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(108); c.m.w32(a_, (v_) as u32); };
                                                                { let a0_ = (l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }); F::CharSel_RequestFace(c, a0_) };
                                                                { let v_ = 6i32; let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(112); c.m.w32(a_, (v_) as u32); };
                                                                { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 41i32); };
                                                            } else {
                                                                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 7i32); };
                                                            }
                                                        } else {
                                                            if ((((({ let a_ = (({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1376)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add((((({ let a_ = { let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }; (c.m.r32(a_) as i32) }).wrapping_add(({ let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(7i32))) as u32)).wrapping_mul(36))).wrapping_add(4); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
                                                                { let r_ = 64i32; let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(104); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                                                                if ((((!((((({ let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(8); (c.m.r32(a_) as i32) }) < ({ let a_ = (({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1376)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add((((({ let a_ = { let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }; (c.m.r32(a_) as i32) }).wrapping_add(({ let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(7i32))) as u32)).wrapping_mul(36))).wrapping_add(4); (c.m.r32(a_) as i32) })) as i32)) != 0)) as i32)) != 0) {
                                                                    { let v_ = 0i32; let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                                                                }
                                                                { let a0_ = ((c.m.r32(sym::gCharSel)).wrapping_add(52)).wrapping_add(((((1i32).wrapping_add((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }))) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_c_e__str110, ((((1i32) as i64 as u64)) as u8)) };
                                                                { let v_ = 1i32; let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(116); c.m.w32(a_, (v_) as u32); };
                                                                { let a0_ = ((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })).wrapping_add(1i32); let a1_ = (l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }); F::CharSel_ClipGoto(c, a0_, a1_, 0i32, sym::menu_c_e__str111) };
                                                                { let a0_ = ((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })).wrapping_add(1i32); let a1_ = (l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }); F::CharSel_ClipGoto(c, a0_, a1_, 1i32, sym::menu_c_e__str112) };
                                                                { let a0_ = (l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }); F::CharSel_SetCellFormChips(c, a0_) };
                                                                { let v_ = { let a_ = (({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(48)).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(8); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(108); c.m.w32(a_, (v_) as u32); };
                                                                { let a0_ = (l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }); F::CharSel_RequestFace(c, a0_) };
                                                                { let v_ = 1i32; let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(112); c.m.w32(a_, (v_) as u32); };
                                                                { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 41i32); };
                                                            } else {
                                                                { let a0_ = ((c.m.r32(sym::gCharSel)).wrapping_add(52)).wrapping_add(((((3i32).wrapping_add((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }))) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_c_e__str113, ((((1i32) as i64 as u64)) as u8)) };
                                                                { let a0_ = ((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })).wrapping_add(1i32); let a1_ = (l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }); F::CharSel_ClipGoto(c, a0_, a1_, 0i32, sym::menu_c_e__str114) };
                                                                { let v_ = 4i32; let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(14296)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                                                { let v_ = { let a0_ = { let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(108); (c.m.r32(a_) as i32) }; let a1_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(24); F::ChrTbl_WrapCostume(c, a0_, a1_) }; let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(14304)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                                                { let a0_ = ((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })).wrapping_add(3i32); let a1_ = (l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }); F::CharSel_ClipGoto(c, a0_, a1_, 2i32, sym::menu_c_e__str115) };
                                                                { let v_ = 2i32; let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(112); c.m.w32(a_, (v_) as u32); };
                                                                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    if ((((c.m.r32(((sym::gPad).wrapping_add((((l_i_3) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1024i32) as u32)))) != 0) {
                                                        if ((((((((({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14316); (c.m.r32(a_) as i32) }) != (1i32)) as i32)) != 0) && ((((({ l_turn_9 = { let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }; l_turn_9 }) != (0i32)) as i32)) != 0)) as i32)) != 0) {
                                                            if (((((l_turn_9) == (1i32)) as i32)) != 0) {
                                                                F::CharSel_ClipGoto(c, (l_i_3).wrapping_add(2i32), (l_i_3).wrapping_add(l_turn_9), 0i32, sym::menu_c_e__str116);
                                                                { let v_ = 0i32; let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); c.m.w32(a_, (v_) as u32); };
                                                                { let a0_ = ((c.m.r32(sym::gCharSel)).wrapping_add(52)).wrapping_add(((((1i32).wrapping_add((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }))) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_c_e__str117, ((((1i32) as i64 as u64)) as u8)) };
                                                                if (((({ let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(104); (c.m.r32(a_) as i32) }) & (64i32))) != 0) {
                                                                    { let a0_ = ((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })).wrapping_add(1i32); let a1_ = (l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }); F::CharSel_ClipGoto(c, a0_, a1_, 1i32, sym::menu_c_e__str118) };
                                                                    { let v_ = l_turn_9; let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(112); c.m.w32(a_, (v_) as u32); };
                                                                } else {
                                                                    if (((({ let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(104); (c.m.r32(a_) as i32) }) & (128i32))) != 0) {
                                                                        { let a0_ = ((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })).wrapping_add(1i32); let a1_ = (l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }); F::CharSel_ClipGoto(c, a0_, a1_, 6i32, sym::menu_c_e__str119) };
                                                                        { let v_ = 6i32; let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(112); c.m.w32(a_, (v_) as u32); };
                                                                    } else {
                                                                        { let a0_ = ((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })).wrapping_add(1i32); let a1_ = (l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }); F::CharSel_ClipGoto(c, a0_, a1_, 0i32, sym::menu_c_e__str120) };
                                                                        { let v_ = 0i32; let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(112); c.m.w32(a_, (v_) as u32); };
                                                                    }
                                                                }
                                                            }
                                                        } else {
                                                            F::ColorFade_StartOut(c, 0i32, 0i32, 0i32, 20i32);
                                                            { let v_ = 0i32; let a_ = p_running_1; c.m.w32(a_, (v_) as u32); };
                                                        }
                                                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 2i32); };
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                break 's9;
                            }
                            1i32 => {
                                if ((((c.m.r32(((sym::gPad).wrapping_add((((l_i_3) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((1i32) as u32)))) != 0) {
                                    { let a0_ = ((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })).wrapping_add(1i32); let a1_ = (l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }); F::CharSel_ClipGoto(c, a0_, a1_, 1i32, sym::menu_c_e__str121) };
                                    { let a0_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(48); let a1_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(8); F::ChrGrid_PrevForm(c, a0_, a1_) };
                                    { let a0_ = ((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })).wrapping_add(1i32); let a1_ = (l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }); F::CharSel_ClipGoto(c, a0_, a1_, 1i32, sym::menu_c_e__str122) };
                                    { let v_ = { let a_ = (({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(48)).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(8); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(108); c.m.w32(a_, (v_) as u32); };
                                    { let a0_ = (l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }); F::CharSel_RequestFace(c, a0_) };
                                    { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 0i32); };
                                } else {
                                    if ((((c.m.r32(((sym::gPad).wrapping_add((((l_i_3) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((2i32) as u32)))) != 0) {
                                        { let a0_ = ((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })).wrapping_add(1i32); let a1_ = (l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }); F::CharSel_ClipGoto(c, a0_, a1_, 1i32, sym::menu_c_e__str123) };
                                        { let a0_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(48); let a1_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(8); F::ChrGrid_NextForm(c, a0_, a1_) };
                                        { let a0_ = ((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })).wrapping_add(1i32); let a1_ = (l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }); F::CharSel_ClipGoto(c, a0_, a1_, 1i32, sym::menu_c_e__str124) };
                                        { let v_ = { let a_ = (({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(48)).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(8); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(108); c.m.w32(a_, (v_) as u32); };
                                        { let a0_ = (l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }); F::CharSel_RequestFace(c, a0_) };
                                        { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 0i32); };
                                    } else {
                                        if ((((c.m.r32(((sym::gPad).wrapping_add((((l_i_3) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                                            { let a0_ = ((c.m.r32(sym::gCharSel)).wrapping_add(52)).wrapping_add(((((3i32).wrapping_add((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }))) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_c_e__str125, ((((1i32) as i64 as u64)) as u8)) };
                                            { let a0_ = ((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })).wrapping_add(1i32); let a1_ = (l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }); F::CharSel_ClipGoto(c, a0_, a1_, 1i32, sym::menu_c_e__str126) };
                                            if (((({ let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(104); (c.m.r32(a_) as i32) }) & (128i32))) != 0) {
                                                { let v_ = 2i32; let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(14296)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                                if ((((!((((({ let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(20); (c.m.r32(a_) as i32) }) < ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(14296)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) })) as i32)) != 0)) as i32)) != 0) {
                                                    { let v_ = 0i32; let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
                                                }
                                            } else {
                                                { let v_ = 4i32; let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(14296)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                            }
                                            { let a0_ = ((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })).wrapping_add(3i32); let a1_ = (l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }); F::CharSel_ClipGoto(c, a0_, a1_, 2i32, sym::menu_c_e__str127) };
                                            { let v_ = { let a0_ = { let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(108); (c.m.r32(a_) as i32) }; let a1_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(24); F::ChrTbl_WrapCostume(c, a0_, a1_) }; let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(14304)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                            { let v_ = 2i32; let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(112); c.m.w32(a_, (v_) as u32); };
                                            { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                                        } else {
                                            if ((((c.m.r32(((sym::gPad).wrapping_add((((l_i_3) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1024i32) as u32)))) != 0) {
                                                { let r_ = 64i32; let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(104); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
                                                { let a0_ = ((c.m.r32(sym::gCharSel)).wrapping_add(52)).wrapping_add(((((1i32).wrapping_add((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }))) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_c_e__str128, ((((1i32) as i64 as u64)) as u8)) };
                                                { let v_ = 1i32; let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(116); c.m.w32(a_, (v_) as u32); };
                                                if (((({ let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(104); (c.m.r32(a_) as i32) }) & (128i32))) != 0) {
                                                    { let a0_ = ((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })).wrapping_add(1i32); let a1_ = (l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }); F::CharSel_ClipGoto(c, a0_, a1_, 1i32, sym::menu_c_e__str129) };
                                                    { let a0_ = ((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })).wrapping_add(1i32); let a1_ = (l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }); F::CharSel_ClipGoto(c, a0_, a1_, 6i32, sym::menu_c_e__str130) };
                                                    { let a0_ = (l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }); F::CharSel_SetCustomChips(c, a0_) };
                                                    { let v_ = { let a_ = (({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(48)).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(12); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(108); c.m.w32(a_, (v_) as u32); };
                                                    { let a0_ = (l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }); F::CharSel_RequestFace(c, a0_) };
                                                    { let v_ = 6i32; let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(112); c.m.w32(a_, (v_) as u32); };
                                                } else {
                                                    { let a0_ = ((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })).wrapping_add(1i32); let a1_ = (l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }); F::CharSel_ClipGoto(c, a0_, a1_, 1i32, sym::menu_c_e__str131) };
                                                    { let a0_ = ((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })).wrapping_add(1i32); let a1_ = (l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }); F::CharSel_ClipGoto(c, a0_, a1_, 0i32, sym::menu_c_e__str132) };
                                                    { let a0_ = (l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }); F::CharSel_SetRowChips(c, a0_) };
                                                    { let v_ = { let a_ = (({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(48)).wrapping_add(((({ let a_ = { let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }; (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(108); c.m.w32(a_, (v_) as u32); };
                                                    { let a0_ = (l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }); F::CharSel_RequestFace(c, a0_) };
                                                    { let v_ = 0i32; let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(112); c.m.w32(a_, (v_) as u32); };
                                                }
                                                { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 41i32); };
                                            }
                                        }
                                    }
                                }
                                break 's9;
                            }
                            2i32 => {
                                if ((((c.m.r32(((sym::gPad).wrapping_add((((l_i_3) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((8i32) as u32)))) != 0) {
                                    { let a0_ = ((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })).wrapping_add(3i32); let a1_ = (l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }); F::CharSel_ClipGoto(c, a0_, a1_, 2i32, sym::menu_c_e__str133) };
                                    { let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(20); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                                    if ((((({ let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(20); (c.m.r32(a_) as i32) }) < (0i32)) as i32)) != 0) {
                                        { let v_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(14296)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_sub(1i32); let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
                                    }
                                    { let a0_ = ((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })).wrapping_add(3i32); let a1_ = (l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }); F::CharSel_ClipGoto(c, a0_, a1_, 2i32, sym::menu_c_e__str134) };
                                    { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                                } else {
                                    if ((((c.m.r32(((sym::gPad).wrapping_add((((l_i_3) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((4i32) as u32)))) != 0) {
                                        { let a0_ = ((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })).wrapping_add(3i32); let a1_ = (l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }); F::CharSel_ClipGoto(c, a0_, a1_, 2i32, sym::menu_c_e__str135) };
                                        { let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(20); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                                        if ((((!((((({ let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(20); (c.m.r32(a_) as i32) }) < ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(14296)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) })) as i32)) != 0)) as i32)) != 0) {
                                            { let v_ = 0i32; let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
                                        }
                                        { let a0_ = ((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })).wrapping_add(3i32); let a1_ = (l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }); F::CharSel_ClipGoto(c, a0_, a1_, 2i32, sym::menu_c_e__str136) };
                                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                                    } else {
                                        l_pressed_10 = c.m.r32(((sym::gPad).wrapping_add((((l_i_3) as u32)).wrapping_mul(448))).wrapping_add(396));
                                        if (((if ((((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) == (0i32)) as i32)) != 0) { ((l_pressed_10) & (((2i32) as u32))) } else { ((l_pressed_10) & (((1i32) as u32))) })) != 0) {
                                            if ((((({ let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(20); (c.m.r32(a_) as i32) }) <= (0i32)) as i32)) != 0) {
                                                break 'c8;
                                            }
                                            if ((((({ let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1376)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add((((({ let a_ = { let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }; (c.m.r32(a_) as i32) }).wrapping_add(({ let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(7i32))) as u32)).wrapping_mul(36)); (c.m.r32(a_) as i32) }) == (161i32)) as i32)) != 0) {
                                                break 'c8;
                                            }
                                            { let a0_ = ((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })).wrapping_add(3i32); let a1_ = (l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }); F::CharSel_ClipGoto(c, a0_, a1_, 2i32, sym::menu_c_e__str137) };
                                            if ((((({ let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1376)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add((((({ let a_ = { let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }; (c.m.r32(a_) as i32) }).wrapping_add(({ let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(7i32))) as u32)).wrapping_mul(36)); (c.m.r32(a_) as i32) }) == (163i32)) as i32)) != 0) {
                                                { let a0_ = (l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }); let a1_ = { let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(108); (c.m.r32(a_) as i32) }; let a2_ = ({ let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(12); (c.m.r32(a_) as i32) }).wrapping_add(({ let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(16); (c.m.r32(a_) as i32) }).wrapping_mul(7i32)); F::ItemPanel_SetChara(c, a0_, a1_, a2_, 0i32, 1i32) };
                                            } else {
                                                { let a0_ = (l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }); let a1_ = { let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(108); (c.m.r32(a_) as i32) }; let a2_ = ({ let a_ = { let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }; (c.m.r32(a_) as i32) }).wrapping_add(({ let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(7i32)); let a3_ = ({ let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(20); (c.m.r32(a_) as i32) }).wrapping_sub(1i32); F::ItemPanel_SetChara(c, a0_, a1_, a2_, a3_, 0i32) };
                                            }
                                            { let a0_ = (l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }); F::ItemPanel_Show(c, a0_) };
                                            { let r_ = 2048i32; let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(104); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                                            { let v_ = 3i32; let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(112); c.m.w32(a_, (v_) as u32); };
                                        } else {
                                            if ((((c.m.r32(((sym::gPad).wrapping_add((((l_i_3) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                                                { let a0_ = ((c.m.r32(sym::gCharSel)).wrapping_add(52)).wrapping_add(((((3i32).wrapping_add((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }))) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_c_e__str138, ((((1i32) as i64 as u64)) as u8)) };
                                                { let a0_ = ((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })).wrapping_add(3i32); let a1_ = (l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }); F::CharSel_ClipGoto(c, a0_, a1_, 2i32, sym::menu_c_e__str139) };
                                                { let a0_ = ((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })).wrapping_add(3i32); let a1_ = (l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }); F::CharSel_ClipGoto(c, a0_, a1_, 5i32, sym::menu_c_e__str140) };
                                                { let v_ = 5i32; let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(112); c.m.w32(a_, (v_) as u32); };
                                                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                                            } else {
                                                if ((((c.m.r32(((sym::gPad).wrapping_add((((l_i_3) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1024i32) as u32)))) != 0) {
                                                    { let a0_ = ((c.m.r32(sym::gCharSel)).wrapping_add(52)).wrapping_add(((((3i32).wrapping_add((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }))) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_c_e__str141, ((((1i32) as i64 as u64)) as u8)) };
                                                    { let a0_ = ((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })).wrapping_add(3i32); let a1_ = (l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }); F::CharSel_ClipGoto(c, a0_, a1_, 2i32, sym::menu_c_e__str142) };
                                                    if (((({ let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(104); (c.m.r32(a_) as i32) }) & (64i32))) != 0) {
                                                        { let a0_ = ((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })).wrapping_add(1i32); let a1_ = (l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }); F::CharSel_ClipGoto(c, a0_, a1_, 1i32, sym::menu_c_e__str143) };
                                                        { let v_ = 1i32; let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(112); c.m.w32(a_, (v_) as u32); };
                                                    } else {
                                                        if (((({ let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(104); (c.m.r32(a_) as i32) }) & (128i32))) != 0) {
                                                            { let a0_ = ((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })).wrapping_add(1i32); let a1_ = (l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }); F::CharSel_ClipGoto(c, a0_, a1_, 6i32, sym::menu_c_e__str144) };
                                                            { let v_ = 6i32; let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(112); c.m.w32(a_, (v_) as u32); };
                                                        } else {
                                                            { let a0_ = ((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })).wrapping_add(1i32); let a1_ = (l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }); F::CharSel_ClipGoto(c, a0_, a1_, 0i32, sym::menu_c_e__str145) };
                                                            { let v_ = 0i32; let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(112); c.m.w32(a_, (v_) as u32); };
                                                        }
                                                    }
                                                    { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 2i32); };
                                                }
                                            }
                                        }
                                    }
                                }
                                break 's9;
                            }
                            3i32 => {
                                l_r_8 = { let a0_ = (l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }); F::ItemPanel_Input(c, a0_, l_i_3) };
                                if (((((l_r_8) > (0i32)) as i32)) != 0) {
                                    F::ItemHelp_Open(c);
                                    { let v_ = (l_r_8).wrapping_sub(1i32); let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(788); c.m.w32(a_, (v_) as u32); };
                                    { let v_ = 4i32; let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(112); c.m.w32(a_, (v_) as u32); };
                                } else {
                                    if (((((l_r_8) < (0i32)) as i32)) != 0) {
                                        { let a0_ = (l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }); F::ItemPanel_Hide(c, a0_) };
                                        { let r_ = (!(2048i32)); let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(104); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                                        { let a0_ = ((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })).wrapping_add(3i32); let a1_ = (l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }); F::CharSel_ClipGoto(c, a0_, a1_, 2i32, sym::menu_c_e__str146) };
                                        { let v_ = 2i32; let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(112); c.m.w32(a_, (v_) as u32); };
                                    }
                                }
                                break 's9;
                            }
                            4i32 => {
                                if ((((c.m.r32(((sym::gPad).wrapping_add((((l_i_3) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1536i32) as u32)))) != 0) {
                                    F::ItemHelp_Close(c);
                                    { let v_ = 3i32; let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(112); c.m.w32(a_, (v_) as u32); };
                                    { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 2i32); };
                                }
                                break 's9;
                            }
                            5i32 => {
                                if ((((c.m.r32(((sym::gPad).wrapping_add((((l_i_3) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((1i32) as u32)))) != 0) {
                                    { let a0_ = ((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })).wrapping_add(3i32); let a1_ = (l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }); F::CharSel_ClipGoto(c, a0_, a1_, 5i32, sym::menu_c_e__str147) };
                                    { let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(24); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                                    if ((((({ let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(24); (c.m.r32(a_) as i32) }) < (0i32)) as i32)) != 0) {
                                        { let v_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(14304)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_sub(1i32); let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                                    }
                                    { let a0_ = ((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })).wrapping_add(3i32); let a1_ = (l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }); F::CharSel_ClipGoto(c, a0_, a1_, 5i32, sym::menu_c_e__str148) };
                                    { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                                } else {
                                    if ((((c.m.r32(((sym::gPad).wrapping_add((((l_i_3) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((2i32) as u32)))) != 0) {
                                        { let a0_ = ((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })).wrapping_add(3i32); let a1_ = (l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }); F::CharSel_ClipGoto(c, a0_, a1_, 5i32, sym::menu_c_e__str149) };
                                        { let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(24); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                                        if ((((!((((({ let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(24); (c.m.r32(a_) as i32) }) < ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(14304)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) })) as i32)) != 0)) as i32)) != 0) {
                                            { let v_ = 0i32; let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                                        }
                                        { let a0_ = ((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })).wrapping_add(3i32); let a1_ = (l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }); F::CharSel_ClipGoto(c, a0_, a1_, 5i32, sym::menu_c_e__str150) };
                                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                                    } else {
                                        if ((((c.m.r32(((sym::gPad).wrapping_add((((l_i_3) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                                            { let v_ = 0i32; let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(116); c.m.w32(a_, (v_) as u32); };
                                            { let a0_ = ((c.m.r32(sym::gCharSel)).wrapping_add(52)).wrapping_add(((((3i32).wrapping_add((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }))) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_c_e__str151, ((((1i32) as i64 as u64)) as u8)) };
                                            { let a0_ = ((c.m.r32(sym::gCharSel)).wrapping_add(52)).wrapping_add(((((1i32).wrapping_add((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }))) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_c_e__str152, ((((1i32) as i64 as u64)) as u8)) };
                                            { let a0_ = ((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })).wrapping_add(3i32); let a1_ = (l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }); F::CharSel_ClipGoto(c, a0_, a1_, 5i32, sym::menu_c_e__str153) };
                                            { let a0_ = ((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })).wrapping_add(3i32); let a1_ = (l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }); F::CharSel_ClipGoto(c, a0_, a1_, 2i32, sym::menu_c_e__str154) };
                                            if (((({ let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(104); (c.m.r32(a_) as i32) }) & (64i32))) != 0) {
                                                { let v_ = { let a_ = (({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(48)).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(8); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                                                if ((((({ let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(20); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
                                                    if ((((({ let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1376)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add((((({ let a_ = { let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }; (c.m.r32(a_) as i32) }).wrapping_add(({ let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(7i32))) as u32)).wrapping_mul(36)); (c.m.r32(a_) as i32) }) == (163i32)) as i32)) != 0) {
                                                        { let s_ = ((c.m.r32(sym::gSaveData)).wrapping_add(11584)).wrapping_add((((({ let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(12); (c.m.r32(a_) as i32) }).wrapping_add(({ let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(16); (c.m.r32(a_) as i32) }).wrapping_mul(7i32))) as u32)).wrapping_mul(28)); let d_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(32); rt::memcpy(c, d_, s_, 16); };
                                                    } else {
                                                        { let s_ = (((c.m.r32(sym::gSaveData)).wrapping_add(6152)).wrapping_add((((({ let a_ = { let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }; (c.m.r32(a_) as i32) }).wrapping_add(({ let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(7i32))) as u32)).wrapping_mul(56))).wrapping_add((((({ let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(20); (c.m.r32(a_) as i32) }).wrapping_sub(1i32)) as u32)).wrapping_mul(16)); let d_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(32); rt::memcpy(c, d_, s_, 16); };
                                                    }
                                                } else {
                                                    { let _ = { let a0_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(32); F::memset(c, a0_, 0i32, ((16i32) as u32)) }; };
                                                }
                                                { let a0_ = ((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })).wrapping_add(1i32); let a1_ = (l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }); F::CharSel_ClipGoto(c, a0_, a1_, 1i32, sym::menu_c_e__str155) };
                                            } else {
                                                if (((({ let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(104); (c.m.r32(a_) as i32) }) & (128i32))) != 0) {
                                                    { let v_ = { let a_ = (({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(48)).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(12); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                                                    if ((((({ let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(20); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
                                                        { let s_ = ((c.m.r32(sym::gSaveData)).wrapping_add(11584)).wrapping_add((((({ let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(12); (c.m.r32(a_) as i32) }).wrapping_add(({ let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(16); (c.m.r32(a_) as i32) }).wrapping_mul(7i32))) as u32)).wrapping_mul(28)); let d_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(32); rt::memcpy(c, d_, s_, 16); };
                                                    } else {
                                                        { let _ = { let a0_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(32); F::memset(c, a0_, 0i32, ((16i32) as u32)) }; };
                                                    }
                                                    { let a0_ = ((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })).wrapping_add(1i32); let a1_ = (l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }); F::CharSel_ClipGoto(c, a0_, a1_, 6i32, sym::menu_c_e__str156) };
                                                } else {
                                                    if ((((({ let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1376)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add((((({ let a_ = { let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }; (c.m.r32(a_) as i32) }).wrapping_add(({ let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(7i32))) as u32)).wrapping_mul(36)); (c.m.r32(a_) as i32) }) == (161i32)) as i32)) != 0) {
                                                        {
                                                            'l10: loop {
                                                                'c11: {
                                                                    l_n_5 = (({ let a0_ = (({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1368)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32); F::Rand_Range(c, a0_) }) as i32);
                                                                    l_id_6 = { let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1376)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add((((l_n_5) as u32)).wrapping_mul(36)); (c.m.r32(a_) as i32) };
                                                                    if (((((l_id_6) < (161i32)) as i32)) != 0) {
                                                                        l_chara_7 = l_id_6;
                                                                        break 'l10;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        if ((((({ let a_ = (({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1376)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add((((l_n_5) as u32)).wrapping_mul(36))).wrapping_add(4); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
                                                            l_chara_7 = { let a_ = ((({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1376)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add((((l_n_5) as u32)).wrapping_mul(36))).wrapping_add(8)).wrapping_add(({ let a0_ = (({ let a_ = (({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1376)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add((((l_n_5) as u32)).wrapping_mul(36))).wrapping_add(4); (c.m.r32(a_) as i32) }) as u32); F::Rand_Range(c, a0_) }).wrapping_mul(4)); (c.m.r32(a_) as i32) };
                                                        }
                                                        { let v_ = l_chara_7; let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                                                    } else {
                                                        { let v_ = { let a_ = (({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(48)).wrapping_add(((({ let a_ = { let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }; (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                                                        l_n_5 = ({ let a_ = { let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }; (c.m.r32(a_) as i32) }).wrapping_add(({ let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(7i32));
                                                    }
                                                    if ((((({ let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(20); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
                                                        { let s_ = (((c.m.r32(sym::gSaveData)).wrapping_add(6152)).wrapping_add((((l_n_5) as u32)).wrapping_mul(56))).wrapping_add((((({ let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(20); (c.m.r32(a_) as i32) }).wrapping_sub(1i32)) as u32)).wrapping_mul(16)); let d_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(32); rt::memcpy(c, d_, s_, 16); };
                                                    } else {
                                                        { let _ = { let a0_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(32); F::memset(c, a0_, 0i32, ((16i32) as u32)) }; };
                                                    }
                                                    { let a0_ = ((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })).wrapping_add(1i32); let a1_ = (l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }); F::CharSel_ClipGoto(c, a0_, a1_, 0i32, sym::menu_c_e__str157) };
                                                }
                                            }
                                            { let v_ = 7i32; let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(112); c.m.w32(a_, (v_) as u32); };
                                            if ((((((((({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14316); (c.m.r32(a_) as i32) }) != (1i32)) as i32)) != 0) && ((((({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0)) as i32)) != 0) {
                                                { let v_ = 1i32; let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); c.m.w32(a_, (v_) as u32); };
                                                { let a0_ = ((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })).wrapping_add(1i32); let a1_ = (l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }); F::CharSel_ClipGoto(c, a0_, a1_, 0i32, sym::menu_c_e__str158) };
                                                { let v_ = 0i32; let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(112); c.m.w32(a_, (v_) as u32); };
                                            }
                                            { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                                        } else {
                                            if ((((c.m.r32(((sym::gPad).wrapping_add((((l_i_3) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1024i32) as u32)))) != 0) {
                                                { let a0_ = ((c.m.r32(sym::gCharSel)).wrapping_add(52)).wrapping_add(((((3i32).wrapping_add((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }))) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_c_e__str159, ((((1i32) as i64 as u64)) as u8)) };
                                                { let a0_ = ((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })).wrapping_add(3i32); let a1_ = (l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }); F::CharSel_ClipGoto(c, a0_, a1_, 5i32, sym::menu_c_e__str160) };
                                                { let a0_ = ((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })).wrapping_add(3i32); let a1_ = (l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }); F::CharSel_ClipGoto(c, a0_, a1_, 2i32, sym::menu_c_e__str161) };
                                                { let v_ = 2i32; let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(112); c.m.w32(a_, (v_) as u32); };
                                                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 2i32); };
                                            }
                                        }
                                    }
                                }
                                break 's9;
                            }
                            6i32 => {
                                if ((((c.m.r32(((sym::gPad).wrapping_add((((l_i_3) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((1i32) as u32)))) != 0) {
                                    { let a0_ = ((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })).wrapping_add(1i32); let a1_ = (l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }); F::CharSel_ClipGoto(c, a0_, a1_, 6i32, sym::menu_c_e__str162) };
                                    { let _ = { let a0_ = ((c.m.r32(sym::gCharSel)).wrapping_add(13280)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(504)); let a1_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(12); let a2_ = { let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(16); (c.m.r32(a_) as i32) }; F::ChrGrid_MoveLeft(c, a0_, a1_, a2_) }; };
                                    { let a0_ = ((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })).wrapping_add(1i32); let a1_ = (l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }); F::CharSel_ClipGoto(c, a0_, a1_, 6i32, sym::menu_c_e__str163) };
                                    { let v_ = { let a_ = (({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(48)).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(12); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(108); c.m.w32(a_, (v_) as u32); };
                                    { let a0_ = (l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }); F::CharSel_RequestFace(c, a0_) };
                                    { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 0i32); };
                                } else {
                                    if ((((c.m.r32(((sym::gPad).wrapping_add((((l_i_3) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((2i32) as u32)))) != 0) {
                                        { let a0_ = ((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })).wrapping_add(1i32); let a1_ = (l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }); F::CharSel_ClipGoto(c, a0_, a1_, 6i32, sym::menu_c_e__str164) };
                                        { let _ = { let a0_ = ((c.m.r32(sym::gCharSel)).wrapping_add(13280)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(504)); let a1_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(12); let a2_ = { let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(16); (c.m.r32(a_) as i32) }; F::ChrGrid_MoveRight(c, a0_, a1_, a2_) }; };
                                        { let a0_ = ((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })).wrapping_add(1i32); let a1_ = (l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }); F::CharSel_ClipGoto(c, a0_, a1_, 6i32, sym::menu_c_e__str165) };
                                        { let v_ = { let a_ = (({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(48)).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(12); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(108); c.m.w32(a_, (v_) as u32); };
                                        { let a0_ = (l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }); F::CharSel_RequestFace(c, a0_) };
                                        { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 0i32); };
                                    } else {
                                        if ((((c.m.r32(((sym::gPad).wrapping_add((((l_i_3) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((8i32) as u32)))) != 0) {
                                            { let a0_ = ((c.m.r32(sym::gCharSel)).wrapping_add(52)).wrapping_add(((((1i32).wrapping_add((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }))) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_c_e__str166, ((((1i32) as i64 as u64)) as u8)) };
                                            { let v_ = 1i32; let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(116); c.m.w32(a_, (v_) as u32); };
                                            { let a0_ = ((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })).wrapping_add(1i32); let a1_ = (l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }); F::CharSel_ClipGoto(c, a0_, a1_, 6i32, sym::menu_c_e__str167) };
                                            { let a0_ = ((c.m.r32(sym::gCharSel)).wrapping_add(13280)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(504)); let a1_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(12); let a2_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(16); F::ChrGrid_MoveUp(c, a0_, a1_, a2_, 2i32) };
                                            { let a0_ = ((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })).wrapping_add(1i32); let a1_ = (l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }); F::CharSel_ClipGoto(c, a0_, a1_, 6i32, sym::menu_c_e__str168) };
                                            { let a0_ = (l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }); F::CharSel_SetCustomChips(c, a0_) };
                                            { let v_ = { let a_ = (({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(48)).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(12); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(108); c.m.w32(a_, (v_) as u32); };
                                            { let a0_ = (l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }); F::CharSel_RequestFace(c, a0_) };
                                            { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 2i32); };
                                        } else {
                                            if ((((c.m.r32(((sym::gPad).wrapping_add((((l_i_3) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((4i32) as u32)))) != 0) {
                                                { let a0_ = ((c.m.r32(sym::gCharSel)).wrapping_add(52)).wrapping_add(((((1i32).wrapping_add((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }))) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_c_e__str169, ((((1i32) as i64 as u64)) as u8)) };
                                                { let v_ = 1i32; let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(116); c.m.w32(a_, (v_) as u32); };
                                                { let a0_ = ((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })).wrapping_add(1i32); let a1_ = (l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }); F::CharSel_ClipGoto(c, a0_, a1_, 6i32, sym::menu_c_e__str170) };
                                                { let a0_ = ((c.m.r32(sym::gCharSel)).wrapping_add(13280)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(504)); let a1_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(12); let a2_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(16); F::ChrGrid_MoveDown(c, a0_, a1_, a2_, 2i32) };
                                                { let a0_ = ((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })).wrapping_add(1i32); let a1_ = (l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }); F::CharSel_ClipGoto(c, a0_, a1_, 6i32, sym::menu_c_e__str171) };
                                                { let a0_ = (l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }); F::CharSel_SetCustomChips(c, a0_) };
                                                { let v_ = { let a_ = (({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(48)).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(12); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(108); c.m.w32(a_, (v_) as u32); };
                                                { let a0_ = (l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }); F::CharSel_RequestFace(c, a0_) };
                                                { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 1i32); };
                                            } else {
                                                if ((((c.m.r32(((sym::gPad).wrapping_add((((l_i_3) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                                                    if ((((({ let a_ = ((((c.m.r32(sym::gCharSel)).wrapping_add(13280)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(504))).wrapping_add((((({ let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(12); (c.m.r32(a_) as i32) }).wrapping_add(({ let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(16); (c.m.r32(a_) as i32) }).wrapping_mul(7i32))) as u32)).wrapping_mul(36))).wrapping_add(4); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
                                                        { let r_ = 64i32; let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(104); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                                                        if ((((!((((({ let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(8); (c.m.r32(a_) as i32) }) < ({ let a_ = ((((c.m.r32(sym::gCharSel)).wrapping_add(13280)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(504))).wrapping_add((((({ let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(12); (c.m.r32(a_) as i32) }).wrapping_add(({ let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(16); (c.m.r32(a_) as i32) }).wrapping_mul(7i32))) as u32)).wrapping_mul(36))).wrapping_add(4); (c.m.r32(a_) as i32) })) as i32)) != 0)) as i32)) != 0) {
                                                            { let v_ = 0i32; let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                                                        }
                                                        { let a0_ = ((c.m.r32(sym::gCharSel)).wrapping_add(52)).wrapping_add(((((1i32).wrapping_add((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }))) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_c_e__str172, ((((1i32) as i64 as u64)) as u8)) };
                                                        { let v_ = 1i32; let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(116); c.m.w32(a_, (v_) as u32); };
                                                        { let a0_ = ((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })).wrapping_add(1i32); let a1_ = (l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }); F::CharSel_ClipGoto(c, a0_, a1_, 6i32, sym::menu_c_e__str173) };
                                                        { let a0_ = ((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })).wrapping_add(1i32); let a1_ = (l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }); F::CharSel_ClipGoto(c, a0_, a1_, 1i32, sym::menu_c_e__str174) };
                                                        { let a0_ = (l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }); F::CharSel_SetFormChips(c, a0_) };
                                                        { let v_ = { let a_ = (({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(48)).wrapping_add(((({ let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(8); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(108); c.m.w32(a_, (v_) as u32); };
                                                        { let a0_ = (l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }); F::CharSel_RequestFace(c, a0_) };
                                                        { let v_ = 1i32; let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(112); c.m.w32(a_, (v_) as u32); };
                                                        { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 41i32); };
                                                    } else {
                                                        { let a0_ = ((c.m.r32(sym::gCharSel)).wrapping_add(52)).wrapping_add(((((3i32).wrapping_add((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }))) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_c_e__str175, ((((1i32) as i64 as u64)) as u8)) };
                                                        { let a0_ = ((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })).wrapping_add(1i32); let a1_ = (l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }); F::CharSel_ClipGoto(c, a0_, a1_, 6i32, sym::menu_c_e__str176) };
                                                        { let v_ = 2i32; let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(14296)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                                        if ((((!((((({ let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(20); (c.m.r32(a_) as i32) }) < ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(14296)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) })) as i32)) != 0)) as i32)) != 0) {
                                                            { let v_ = 0i32; let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
                                                        }
                                                        { let a0_ = ((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })).wrapping_add(3i32); let a1_ = (l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }); F::CharSel_ClipGoto(c, a0_, a1_, 2i32, sym::menu_c_e__str177) };
                                                        { let v_ = { let a0_ = { let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(108); (c.m.r32(a_) as i32) }; let a1_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(24); F::ChrTbl_WrapCostume(c, a0_, a1_) }; let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(14304)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                                        { let v_ = 2i32; let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(112); c.m.w32(a_, (v_) as u32); };
                                                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                                                    }
                                                } else {
                                                    if ((((c.m.r32(((sym::gPad).wrapping_add((((l_i_3) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1024i32) as u32)))) != 0) {
                                                        { let r_ = 128i32; let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(104); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
                                                        { let a0_ = ((c.m.r32(sym::gCharSel)).wrapping_add(52)).wrapping_add(((((1i32).wrapping_add((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }))) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_c_e__str178, ((((1i32) as i64 as u64)) as u8)) };
                                                        { let v_ = 1i32; let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(116); c.m.w32(a_, (v_) as u32); };
                                                        { let a0_ = ((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })).wrapping_add(1i32); let a1_ = (l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }); F::CharSel_ClipGoto(c, a0_, a1_, 6i32, sym::menu_c_e__str179) };
                                                        { let a0_ = ((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })).wrapping_add(1i32); let a1_ = (l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }); F::CharSel_ClipGoto(c, a0_, a1_, 0i32, sym::menu_c_e__str180) };
                                                        { let a0_ = (l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }); F::CharSel_SetRowChips(c, a0_) };
                                                        { let v_ = { let a_ = (({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(48)).wrapping_add(((({ let a_ = { let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }; (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(108); c.m.w32(a_, (v_) as u32); };
                                                        { let a0_ = (l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }); F::CharSel_RequestFace(c, a0_) };
                                                        { let v_ = 0i32; let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(112); c.m.w32(a_, (v_) as u32); };
                                                        { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 41i32); };
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                break 's9;
                            }
                            7i32 => {
                                if (((({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(764); (c.m.r32(a_) as i32) }) & (4i32))) != 0) {
                                    break 'c8;
                                }
                                if ((((({ let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) ^ (1i32))) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(112); (c.m.r32(a_) as i32) }) == (7i32)) as i32)) != 0) {
                                    { let a0_ = ((c.m.r32(sym::gCharSel)).wrapping_add(52)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_c_e__str181, ((((1i32) as i64 as u64)) as u8)) };
                                    F::IconWin_Open(c);
                                    F::CharSel_ClipGoto(c, 0i32, 0i32, 8i32, sym::menu_c_e__str182);
                                    { let r_ = 4i32; let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(764); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                                    { let v_ = 8i32; let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(1364); c.m.r32(a_) }).wrapping_add(64); c.m.w32(a_, (v_) as u32); };
                                    { let v_ = 1i32; let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14356); c.m.w32(a_, (v_) as u32); };
                                } else {
                                    if ((((c.m.r32(((sym::gPad).wrapping_add((((l_i_3) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1024i32) as u32)))) != 0) {
                                        { let a0_ = ((c.m.r32(sym::gCharSel)).wrapping_add(52)).wrapping_add(((((1i32).wrapping_add((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }))) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_c_e__str183, ((((1i32) as i64 as u64)) as u8)) };
                                        { let a0_ = ((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })).wrapping_add(1i32); let a1_ = (l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }); F::CharSel_ClipGoto(c, a0_, a1_, 0i32, sym::menu_c_e__str184) };
                                        { let a0_ = (l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }); F::CharSel_SetRowChips(c, a0_) };
                                        { let v_ = { let a_ = (({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(48)).wrapping_add(((({ let a_ = { let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }; (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(108); c.m.w32(a_, (v_) as u32); };
                                        { let a0_ = (l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) }); F::CharSel_RequestFace(c, a0_) };
                                        { let v_ = 0i32; let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add(((((l_i_3).wrapping_add({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14336); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(112); c.m.w32(a_, (v_) as u32); };
                                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 2i32); };
                                    }
                                }
                                break 's9;
                            }
                            _ => {
                            }
                        }
                    }
                }
                { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "CharSel_Input", &PROBE_N, &[], true); }
    r_
}

pub fn CharSel_Run(c: &mut Ctx, mut p_section_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "CharSel_Run", &PROBE_N, &[(p_section_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_screenMode_2: i32 = 0;
    let mut l_bgm_3: i32 = 0;
    let mut l_timeLimit_4: i32 = 0;
    let mut l_announcer_5: i32 = 0;
    let mut l_stage_6: i32 = 0;
    let mut l_unk10_7: i32 = 0;
    let mut l_cpuLevel_8: i32 = 0;
    let mut l_i_9: i32 = 0;
    let mut l_mode_10: i32 = 0;
    let r_ = 'f: {
        { let v_ = 1i32; c.m.w32(fp, (v_) as u32); }
        F::CharSel_Init(c, p_section_1);
        F::ColorFade_StartIn(c, 0i32, 0i32, 0i32, 20i32);
        'l1: while ((1i32) != 0) {
            F::Gfx_BeginFrame(c);
            F::CharSel_UpdateFaceLoad(c);
            F::CharSel_UpdateStageLoad(c);
            F::Pad_Update(c);
            F::Snd_Update(c);
            F::ColorFade_Update(c);
            if ((((!(((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(20); (c.m.r32(a_) as i32) }) & (256i32))) != 0)) as i32)) != 0) {
                F::CharSel_Update(c);
            }
            F::CharSel_Draw(c);
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
                if ((((((((({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(768); (c.m.r32(a_) as i32) }) == (4i32)) as i32)) != 0) || ((((({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(772); (c.m.r32(a_) as i32) }) == (4i32)) as i32)) != 0)) as i32)) != 0) {
                    break 'l1;
                }
                continue 'l1;
            }
            if (((({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(764); (c.m.r32(a_) as i32) }) & (16i32))) != 0) {
                if ((((({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(784); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) == ((1i32).wrapping_neg())) as i32)) != 0) {
                    F::ColorFade_StartOut(c, 0i32, 0i32, 0i32, 20i32);
                    { let s_ = { let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }; let d_ = ((c.m.r32(sym::gProgress)).wrapping_add(1088)).wrapping_add((((0i32) as u32)).wrapping_mul(240)); rt::memcpy(c, d_, s_, 48); };
                    { let s_ = { let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }; let d_ = ((c.m.r32(sym::gProgress)).wrapping_add(1088)).wrapping_add((((1i32) as u32)).wrapping_mul(240)); rt::memcpy(c, d_, s_, 48); };
                    { let v_ = ({ let a_ = { let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(1364); c.m.r32(a_) }; (c.m.r32(a_) as i32) }).wrapping_add(({ let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(1364); c.m.r32(a_) }).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(6i32)); let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1576); c.m.w32(a_, (v_) as u32); };
                    { let v_ = { let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14344); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(1364); c.m.r32(a_) }).wrapping_add(80); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1580); c.m.w32(a_, (v_) as u32); };
                }
            } else {
                F::CharSel_Input(c, fp);
            }
        }
        if ((((((c.m.r32(fp) as i32)) != (0i32)) as i32)) != 0) {
            rt::memset(c, fp.wrapping_add(4), 0, 8);
            { let v_ = 1i32; c.m.w32(fp.wrapping_add(4).wrapping_add(0), (v_) as u32); }
            { let v_ = 1i32; c.m.w32(fp.wrapping_add(4).wrapping_add(4), (v_) as u32); }
            if ((((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(1364); c.m.r32(a_) }).wrapping_add(60); (c.m.r32(a_) as i32) }) == (35i32)) as i32)) != 0) {
                'l2: loop {
                    'c3: {
                        { let v_ = { let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14340); c.m.r32(a_) }).wrapping_add(({ let a0_ = ((({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14348); (c.m.r32(a_) as i32) }).wrapping_sub(1i32)) as u32); F::Rand_Range(c, a0_) }).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(1364); c.m.r32(a_) }).wrapping_add(60); c.m.w32(a_, (v_) as u32); };
                    }
                    if !((((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(1364); c.m.r32(a_) }).wrapping_add(60); (c.m.r32(a_) as i32) }) >= (35i32)) as i32)) != 0) { break 'l2; }
                }
            }
            if ((((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); (c.m.r32(a_) as i32) }) == (45i32)) as i32)) != 0) {
                l_mode_10 = 6i32;
                l_bgm_3 = { let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14344); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(1364); c.m.r32(a_) }).wrapping_add(80); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
                l_screenMode_2 = 0i32;
                if (((((l_bgm_3) == (24i32)) as i32)) != 0) {
                    l_bgm_3 = (((F::Rand_Range(c, ((9i32) as u32))).wrapping_add(((8i32) as u32))) as i32);
                }
                l_announcer_5 = 1i32;
                l_timeLimit_4 = 0i32;
                l_unk10_7 = 0i32;
                l_stage_6 = { let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(1364); c.m.r32(a_) }).wrapping_add(60); (c.m.r32(a_) as i32) };
                l_cpuLevel_8 = (1i32).wrapping_neg();
                { let v_ = 1i32; let a_ = (fp.wrapping_add(4)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = 1i32; let a_ = (fp.wrapping_add(4)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            } else {
                l_mode_10 = 0i32;
                l_bgm_3 = { let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14344); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(1364); c.m.r32(a_) }).wrapping_add(80); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
                l_screenMode_2 = ((({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14316); (c.m.r32(a_) as i32) }) == (1i32)) as i32);
                if (((((l_bgm_3) == (24i32)) as i32)) != 0) {
                    l_bgm_3 = (((F::Rand_Range(c, ((9i32) as u32))).wrapping_add(((8i32) as u32))) as i32);
                }
                's4: {
                    let sv11: i32 = { let a_ = ((c.m.r32(sym::gSaveData)).wrapping_add(3124)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
                    match sv11 {
                        0i32 => {
                            l_timeLimit_4 = 1i32;
                            break 's4;
                        }
                        1i32 => {
                            l_timeLimit_4 = 2i32;
                            break 's4;
                        }
                        2i32 => {
                            l_timeLimit_4 = 3i32;
                            break 's4;
                        }
                        3i32 => {
                            l_timeLimit_4 = 4i32;
                            break 's4;
                        }
                        4i32 => {
                            l_timeLimit_4 = 0i32;
                            break 's4;
                        }
                        _ => {
                            l_timeLimit_4 = 5i32;
                            break 's4;
                        }
                    }
                }
                l_stage_6 = { let a_ = ({ let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(1364); c.m.r32(a_) }).wrapping_add(60); (c.m.r32(a_) as i32) };
                l_unk10_7 = (({ let a_ = ((c.m.r32(sym::gSaveData)).wrapping_add(3124)).wrapping_add((((5i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) ^ (1i32));
                l_announcer_5 = { let a_ = ((c.m.r32(sym::gSaveData)).wrapping_add(3124)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
                l_cpuLevel_8 = { let a0_ = (({ let a_ = ((c.m.r32(sym::gSaveData)).wrapping_add(3124)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32); F::CpuLevel_FromSetting(c, a0_) };
                { let v_ = (({ let a_ = ((c.m.r32(sym::gSaveData)).wrapping_add(3124)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) ^ (1i32)); let a_ = (fp.wrapping_add(4)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = (({ let a_ = ((c.m.r32(sym::gSaveData)).wrapping_add(3124)).wrapping_add((((4i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) ^ (1i32)); let a_ = (fp.wrapping_add(4)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            }
            F::Battle_ClearWork(c);
            F::BattleSetup_SetRule(c, l_screenMode_2, l_mode_10, l_bgm_3, l_timeLimit_4, l_announcer_5, l_stage_6, l_unk10_7);
            's5: {
                let sv12: i32 = { let a_ = (c.m.r32(sym::gCharSel)).wrapping_add(14316); (c.m.r32(a_) as i32) };
                match sv12 {
                    0i32 => {
                        F::BattleSetup_SetSide(c, 0i32, 0i32, 0i32, 1i32, 1i32, 1i32, 0i32, ((0i32) as u32));
                        { let a4_ = (c.m.r32((fp.wrapping_add(4)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32); F::BattleSetup_SetSide(c, 1i32, 2i32, 1i32, 1i32, a4_, 1i32, 0i32, ((0i32) as u32)) };
                        break 's5;
                    }
                    1i32 => {
                        F::BattleSetup_SetSide(c, 0i32, 0i32, 0i32, 1i32, 1i32, 1i32, 0i32, ((0i32) as u32));
                        F::BattleSetup_SetSide(c, 1i32, 0i32, 1i32, 1i32, 1i32, 1i32, 0i32, ((0i32) as u32));
                        break 's5;
                    }
                    2i32 => {
                        { let a4_ = (c.m.r32((fp.wrapping_add(4)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32); F::BattleSetup_SetSide(c, 0i32, 2i32, 0i32, 1i32, a4_, 1i32, 0i32, ((0i32) as u32)) };
                        { let a4_ = (c.m.r32((fp.wrapping_add(4)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32); F::BattleSetup_SetSide(c, 1i32, 2i32, 1i32, 1i32, a4_, 1i32, 0i32, ((0i32) as u32)) };
                        break 's5;
                    }
                    _ => {
                    }
                }
            }
            {
                l_i_9 = 0i32;
                'l6: loop {
                    if !(((((l_i_9) < (2i32)) as i32)) != 0) { break 'l6; }
                    'c7: {
                        { let a2_ = { let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add((((l_i_9) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(28); (c.m.r32(a_) as i32) }; let a3_ = { let a_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add((((l_i_9) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(24); (c.m.r32(a_) as i32) }; let a7_ = ({ let a_ = ((c.m.r32(sym::gCharSel)).wrapping_add(1272)).wrapping_add((((l_i_9) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_add(32); F::BattleSetup_SetMember(c, l_i_9, 0i32, a2_, a3_, 0i32, l_cpuLevel_8, F32(0x42c80000), a7_) };
                    }
                    { let o_ = l_i_9; let n_ = (o_).wrapping_add(1); l_i_9 = n_; };
                }
            }
            F::BattleSetup_Finish(c);
        }
        F::CharSel_Term(c);
        F::Dma_ResetBuffers(c);
        break 'f (c.m.r32(fp) as i32);
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "CharSel_Run", &PROBE_N, &[], true); }
    r_
}
