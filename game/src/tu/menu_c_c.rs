#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn HistResult_SplitEpisode(c: &mut Ctx, mut p_id_1: i32, mut p_menu_2: u32, mut p_item_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HistResult_SplitEpisode", &PROBE_N, &[(p_id_1) as u32, (p_menu_2) as u32, (p_item_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_i_4: i32 = 0;
    let r_ = 'f: {
        rt::memset(c, fp, 0, 32);
        { let v_ = 3i32; c.m.w32(fp.wrapping_add(0), (v_) as u32); }
        { let v_ = 4i32; c.m.w32(fp.wrapping_add(4), (v_) as u32); }
        { let v_ = 7i32; c.m.w32(fp.wrapping_add(8), (v_) as u32); }
        { let v_ = 5i32; c.m.w32(fp.wrapping_add(12), (v_) as u32); }
        { let v_ = 16i32; c.m.w32(fp.wrapping_add(16), (v_) as u32); }
        { let v_ = 5i32; c.m.w32(fp.wrapping_add(20), (v_) as u32); }
        { let v_ = 4i32; c.m.w32(fp.wrapping_add(24), (v_) as u32); }
        { let v_ = 4i32; c.m.w32(fp.wrapping_add(28), (v_) as u32); }
        's1: {
            let sv5: i32 = p_id_1;
            match sv5 {
                20i32 => {
                    p_id_1 = 23i32;
                    break 's1;
                }
                21i32 => {
                    p_id_1 = 20i32;
                    break 's1;
                }
                22i32 => {
                    p_id_1 = 21i32;
                    break 's1;
                }
                23i32 => {
                    p_id_1 = 22i32;
                    break 's1;
                }
                _ => {
                }
            }
        }
        { let v_ = 0i32; let a_ = p_menu_2; c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = p_item_3; c.m.w32(a_, (v_) as u32); };
        {
            l_i_4 = 0i32;
            'l2: loop {
                if !(((((l_i_4) < (8i32)) as i32)) != 0) { break 'l2; }
                'c3: {
                    if (((((p_id_1) < ((c.m.r32((fp).wrapping_add((((l_i_4) as u32)).wrapping_mul(4))) as i32))) as i32)) != 0) {
                        break 'l2;
                    }
                    { let a_ = p_menu_2; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    { let r_ = (c.m.r32((fp).wrapping_add((((l_i_4) as u32)).wrapping_mul(4))) as i32); let n_ = (p_id_1).wrapping_sub(r_); p_id_1 = n_; };
                }
                { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
            }
        }
        { let v_ = p_id_1; let a_ = p_item_3; c.m.w32(a_, (v_) as u32); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "HistResult_SplitEpisode", &PROBE_N, &[], true); }
    r_
}

pub fn HistResult_BuildRewards(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HistResult_BuildRewards", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_prog_1: u32 = 0;
    let mut l_rw_2: u32 = 0;
    let mut l_i_3: i32 = 0;
    let mut l_j_4: i32 = 0;
    let mut l_id_5: i32 = 0;
    let r_ = 'f: {
        l_prog_1 = c.m.r32(sym::gProgress);
        l_rw_2 = (l_prog_1).wrapping_add(64);
        { let _ = { let a0_ = (c.m.r32(sym::gHistResult)).wrapping_add(176); F::memset(c, a0_, 0i32, ((136i32) as u32)) }; };
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(312); c.m.w32(a_, (v_) as u32); };
        {
            l_i_3 = 0i32;
            'l1: loop {
                if !(((((l_i_3) < (3i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let v_ = (1i32).wrapping_neg(); let a_ = (fp).wrapping_add((((l_i_3) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    l_id_5 = (c.m.r32((((l_prog_1).wrapping_add(64)).wrapping_add(48)).wrapping_add((((l_i_3) as u32)).wrapping_mul(4))) as i32);
                    if (((((l_id_5) >= (0i32)) as i32)) != 0) {
                        F::HistResult_SplitEpisode(c, l_id_5, fp.wrapping_add(12), fp.wrapping_add(16));
                        if ((((!(((({ let a_ = (((c.m.r32(sym::gSaveData)).wrapping_add(8)).wrapping_add(8)).wrapping_add(((((c.m.r32(fp.wrapping_add(12)) as i32)) as u32)).wrapping_mul(16)); (c.m.r32(a_) as i32) }) & (1i32))) != 0)) as i32)) != 0) {
                            {
                                l_j_4 = 0i32;
                                'l3: loop {
                                    if !(((((l_j_4) < (3i32)) as i32)) != 0) { break 'l3; }
                                    'c4: {
                                        if ((((((c.m.r32((fp).wrapping_add((((l_j_4) as u32)).wrapping_mul(4))) as i32)) == ((c.m.r32(fp.wrapping_add(12)) as i32))) as i32)) != 0) {
                                            break 'l3;
                                        }
                                    }
                                    { let o_ = l_j_4; let n_ = (o_).wrapping_add(1); l_j_4 = n_; };
                                }
                            }
                            if (((((l_j_4) == (3i32)) as i32)) != 0) {
                                { let v_ = (c.m.r32(fp.wrapping_add(12)) as i32); let a_ = (fp).wrapping_add((((l_i_3) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                            }
                        }
                    }
                }
                { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
            }
        }
        if (((((({ let a_ = (((c.m.r32(sym::gSaveData)).wrapping_add(8)).wrapping_add(11968)).wrapping_add(((136i32) as u32)); c.m.r8(a_) }) as i32)) & (1i32))) != 0) {
            { let v_ = ({ let a_ = (l_rw_2).wrapping_add(((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(60); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_mul(2i32); let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(152); c.m.w32(a_, (v_) as u32); };
        } else {
            { let v_ = { let a_ = (l_rw_2).wrapping_add(((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(60); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(152); c.m.w32(a_, (v_) as u32); };
        }
        {
            l_i_3 = 0i32;
            'l5: loop {
                if !(((((l_i_3) < (3i32)) as i32)) != 0) { break 'l5; }
                'c6: {
                    l_id_5 = (c.m.r32(((l_rw_2).wrapping_add(12)).wrapping_add((((l_i_3) as u32)).wrapping_mul(4))) as i32);
                    if (((((((((l_id_5) >= (0i32)) as i32)) != 0) && ((((!(((((({ let a_ = (((c.m.r32(sym::gSaveData)).wrapping_add(8)).wrapping_add(11968)).wrapping_add(((l_id_5) as u32)); c.m.r8(a_) }) as i32)) & (1i32))) != 0)) as i32)) != 0)) as i32)) != 0) {
                        { let v_ = 2i32; let a_ = ((c.m.r32(sym::gHistResult)).wrapping_add(176)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(312); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(8)); c.m.w32(a_, (v_) as u32); };
                        { let v_ = l_id_5; let a_ = (((c.m.r32(sym::gHistResult)).wrapping_add(176)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(312); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(8))).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                        { let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(312); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        F::Save_AddItem(c, l_id_5);
                    }
                }
                { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
            }
        }
        {
            l_i_3 = 0i32;
            'l7: loop {
                if !(((((l_i_3) < (3i32)) as i32)) != 0) { break 'l7; }
                'c8: {
                    l_id_5 = (c.m.r32(((l_rw_2).wrapping_add(24)).wrapping_add((((l_i_3) as u32)).wrapping_mul(4))) as i32);
                    if (((((((((l_id_5) >= (0i32)) as i32)) != 0) && ((((!((((((({ let a_ = (((c.m.r32(sym::gSaveData)).wrapping_add(8)).wrapping_add(3080)).wrapping_add(((((l_id_5).wrapping_shr((6i32) as u32)) as u32)).wrapping_mul(8)); c.m.r64(a_) }).wrapping_shr(((l_id_5).wrapping_sub(((l_id_5).wrapping_shr((6i32) as u32)).wrapping_shl((6i32) as u32))) as u32)) & (((1i32) as u64)))) as i32)) != 0)) as i32)) != 0)) as i32)) != 0) {
                        { let v_ = 0i32; let a_ = ((c.m.r32(sym::gHistResult)).wrapping_add(176)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(312); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(8)); c.m.w32(a_, (v_) as u32); };
                        { let v_ = l_id_5; let a_ = (((c.m.r32(sym::gHistResult)).wrapping_add(176)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(312); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(8))).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                        { let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(312); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        { let r_ = (((1i64).wrapping_shl(((l_id_5).wrapping_sub(((l_id_5).wrapping_shr((6i32) as u32)).wrapping_shl((6i32) as u32))) as u32)) as u64); let a_ = (((c.m.r32(sym::gSaveData)).wrapping_add(8)).wrapping_add(3080)).wrapping_add(((((l_id_5).wrapping_shr((6i32) as u32)) as u32)).wrapping_mul(8)); let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
                    }
                }
                { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
            }
        }
        {
            l_i_3 = 0i32;
            'l9: loop {
                if !(((((l_i_3) < (3i32)) as i32)) != 0) { break 'l9; }
                'c10: {
                    l_id_5 = (c.m.r32(((l_rw_2).wrapping_add(36)).wrapping_add((((l_i_3) as u32)).wrapping_mul(4))) as i32);
                    if (((((((((l_id_5) >= (0i32)) as i32)) != 0) && ((((!((((((({ let a_ = ((c.m.r32(sym::gSaveData)).wrapping_add(8)).wrapping_add(3104); c.m.r64(a_) }).wrapping_shr((l_id_5) as u32)) & (((1i32) as u64)))) as i32)) != 0)) as i32)) != 0)) as i32)) != 0) {
                        { let v_ = 1i32; let a_ = ((c.m.r32(sym::gHistResult)).wrapping_add(176)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(312); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(8)); c.m.w32(a_, (v_) as u32); };
                        { let v_ = l_id_5; let a_ = (((c.m.r32(sym::gHistResult)).wrapping_add(176)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(312); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(8))).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                        { let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(312); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        { let r_ = (((1i64).wrapping_shl((l_id_5) as u32)) as u64); let a_ = ((c.m.r32(sym::gSaveData)).wrapping_add(8)).wrapping_add(3104); let n_ = ((c.m.r64(a_)) | (r_)); c.m.w64(a_, n_); };
                    }
                }
                { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
            }
        }
        {
            l_i_3 = 0i32;
            'l11: loop {
                if !(((((l_i_3) < (3i32)) as i32)) != 0) { break 'l11; }
                'c12: {
                    l_id_5 = (c.m.r32((fp).wrapping_add((((l_i_3) as u32)).wrapping_mul(4))) as i32);
                    if (((((((((l_id_5) >= (0i32)) as i32)) != 0) && ((((!(((({ let a_ = (((c.m.r32(sym::gSaveData)).wrapping_add(8)).wrapping_add(8)).wrapping_add((((l_id_5) as u32)).wrapping_mul(16)); (c.m.r32(a_) as i32) }) & (1i32))) != 0)) as i32)) != 0)) as i32)) != 0) {
                        { let v_ = 7i32; let a_ = ((c.m.r32(sym::gHistResult)).wrapping_add(176)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(312); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(8)); c.m.w32(a_, (v_) as u32); };
                        { let v_ = l_id_5; let a_ = (((c.m.r32(sym::gHistResult)).wrapping_add(176)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(312); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(8))).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                        { let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(312); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        { let r_ = 1i32; let a_ = (((c.m.r32(sym::gSaveData)).wrapping_add(8)).wrapping_add(8)).wrapping_add((((l_id_5) as u32)).wrapping_mul(16)); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                        { let r_ = 2i32; let a_ = (((c.m.r32(sym::gSaveData)).wrapping_add(8)).wrapping_add(8)).wrapping_add((((l_id_5) as u32)).wrapping_mul(16)); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                        { let r_ = 512i32; let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(8); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    }
                }
                { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
            }
        }
        {
            l_i_3 = 0i32;
            'l13: loop {
                if !(((((l_i_3) < (3i32)) as i32)) != 0) { break 'l13; }
                'c14: {
                    l_id_5 = (c.m.r32(((l_rw_2).wrapping_add(48)).wrapping_add((((l_i_3) as u32)).wrapping_mul(4))) as i32);
                    if (((((l_id_5) >= (0i32)) as i32)) != 0) {
                        F::HistResult_SplitEpisode(c, l_id_5, fp.wrapping_add(12), fp.wrapping_add(16));
                        if ((((!(((({ let a_ = (((((c.m.r32(sym::gSaveData)).wrapping_add(8)).wrapping_add(8)).wrapping_add(((((c.m.r32(fp.wrapping_add(12)) as i32)) as u32)).wrapping_mul(16))).wrapping_add(4)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) & ((((1u32).wrapping_shl(((c.m.r32(fp.wrapping_add(16)) as i32)) as u32)) as i32)))) != 0)) as i32)) != 0) {
                            { let v_ = 8i32; let a_ = ((c.m.r32(sym::gHistResult)).wrapping_add(176)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(312); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(8)); c.m.w32(a_, (v_) as u32); };
                            { let v_ = l_id_5; let a_ = (((c.m.r32(sym::gHistResult)).wrapping_add(176)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(312); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(8))).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                            { let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(312); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                            { let r_ = (((1u32).wrapping_shl(((c.m.r32(fp.wrapping_add(16)) as i32)) as u32)) as i32); let a_ = (((((c.m.r32(sym::gSaveData)).wrapping_add(8)).wrapping_add(8)).wrapping_add(((((c.m.r32(fp.wrapping_add(12)) as i32)) as u32)).wrapping_mul(16))).wrapping_add(4)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                            { let r_ = (((1u32).wrapping_shl(((c.m.r32(fp.wrapping_add(16)) as i32)) as u32)) as i32); let a_ = (((((c.m.r32(sym::gSaveData)).wrapping_add(8)).wrapping_add(8)).wrapping_add(((((c.m.r32(fp.wrapping_add(12)) as i32)) as u32)).wrapping_mul(16))).wrapping_add(4)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                            { let r_ = 64i32; let a_ = (((c.m.r32(sym::gSaveData)).wrapping_add(8)).wrapping_add(8)).wrapping_add(((((c.m.r32(fp.wrapping_add(12)) as i32)) as u32)).wrapping_mul(16)); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                        }
                    }
                }
                { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
            }
        }
        l_id_5 = ({ let a_ = (F::BattleResult_GetPtr(c)).wrapping_add(68); (c.m.r32(a_) as i32) }).wrapping_sub(1i32);
        if (((((l_id_5) >= (0i32)) as i32)) != 0) {
            { let v_ = 9i32; let a_ = ((c.m.r32(sym::gHistResult)).wrapping_add(176)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(312); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(8)); c.m.w32(a_, (v_) as u32); };
            { let v_ = l_id_5; let a_ = (((c.m.r32(sym::gHistResult)).wrapping_add(176)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(312); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(8))).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
            { let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(312); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
            { let r_ = (((1u32).wrapping_shl((l_id_5) as u32)) as i32); let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(8); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "HistResult_BuildRewards", &PROBE_N, &[], true); }
    r_
}

pub fn HistResult_IsMenuCleared(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HistResult_IsMenuCleared", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_result_1: i32 = 0;
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
        rt::memset(c, fp, 0, 32);
        { let v_ = 3i32; c.m.w32(fp.wrapping_add(0), (v_) as u32); }
        { let v_ = 4i32; c.m.w32(fp.wrapping_add(4), (v_) as u32); }
        { let v_ = 7i32; c.m.w32(fp.wrapping_add(8), (v_) as u32); }
        { let v_ = 5i32; c.m.w32(fp.wrapping_add(12), (v_) as u32); }
        { let v_ = 16i32; c.m.w32(fp.wrapping_add(16), (v_) as u32); }
        { let v_ = 5i32; c.m.w32(fp.wrapping_add(20), (v_) as u32); }
        { let v_ = 4i32; c.m.w32(fp.wrapping_add(24), (v_) as u32); }
        { let v_ = 4i32; c.m.w32(fp.wrapping_add(28), (v_) as u32); }
        l_result_1 = 1i32;
        {
            l_i_2 = 0i32;
            'l1: loop {
                if !(((((l_i_2) < ({ let a_ = (fp).wrapping_add(((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(52); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) })) as i32)) != 0) { break 'l1; }
                'c2: {
                    if ((((!(((({ let a_ = (((((c.m.r32(sym::gSaveData)).wrapping_add(8)).wrapping_add(8)).wrapping_add(((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(52); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(16))).wrapping_add(4)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) & ((((1u32).wrapping_shl((l_i_2) as u32)) as i32)))) != 0)) as i32)) != 0) {
                        l_result_1 = 0i32;
                        break 'l1;
                    }
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        break 'f l_result_1;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "HistResult_IsMenuCleared", &PROBE_N, &[], true); }
    r_
}

pub fn HistResult_Init(c: &mut Ctx, mut p_section_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HistResult_Init", &PROBE_N, &[(p_section_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_flags_2: i32 = 0;
    let mut l_bg_3: i32 = 0;
    let r_ = 'f: {
        { let v_ = ((0i32) as u32); c.m.w32(fp, v_); }
        l_flags_2 = F::BattleResult_GetFlags(c);
        { let v_ = F::Heap_Alloc(c, 320i32, ((32i32) as u32), 0i32, 2i32); let a_ = sym::gHistResult; c.m.w32(a_, v_); };
        { let _ = { let a0_ = c.m.r32(sym::gHistResult); F::memset(c, a0_, 0i32, ((320i32) as u32)) }; };
        { let v_ = (c.m.r32(sym::gMenuArc2)).wrapping_add((({ let a_ = (c.m.r32(sym::gMenuArc2)).wrapping_add((((p_section_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = c.m.r32(sym::gHistResult); c.m.w32(a_, v_); };
        { let v_ = { let a0_ = { let a_ = c.m.r32(sym::gHistResult); c.m.r32(a_) }; F::Sprite_Unpack(c, a0_, ((0i32) as u32), ((0i32) as u32)) }; let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(4); c.m.w32(a_, v_); };
        { let v_ = { let a0_ = (({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_add({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(52); (c.m.r32(a_) as i32) })).wrapping_add(16i32); F::File_LoadSync(c, a0_, ((0i32) as u32), 0i32) }; let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(8); c.m.w32(a_, v_); };
        { let v_ = { let a0_ = { let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(8); c.m.r32(a_) }; F::Sprite_Unpack(c, a0_, ((0i32) as u32), ((0i32) as u32)) }; let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(12); c.m.w32(a_, v_); };
        if ((((l_flags_2) & (1i32))) != 0) {
            l_bg_3 = ((F::Rand_Range(c, ((3i32) as u32))) as i32);
            { let r_ = (((1u32).wrapping_shl(({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(56); (c.m.r32(a_) as i32) }) as u32)) as i32); let a_ = (((((c.m.r32(sym::gSaveData)).wrapping_add(8)).wrapping_add(8)).wrapping_add(((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(52); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(16))).wrapping_add(4)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        } else {
            l_bg_3 = 3i32;
            { let r_ = 32i32; let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(128); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        }
        { let v_ = F::File_LoadSync(c, (l_bg_3).wrapping_add(1022i32), ((0i32) as u32), 0i32); let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(16); c.m.w32(a_, v_); };
        { let v_ = { let a0_ = { let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(16); c.m.r32(a_) }; F::Sprite_Unpack(c, a0_, ((0i32) as u32), ((0i32) as u32)) }; let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(20); c.m.w32(a_, v_); };
        { let v_ = { let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(20); c.m.r32(a_) }; let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        's1: {
            let sv4: i32 = l_bg_3;
            match sv4 {
                0i32 => {
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistResult)).wrapping_add(72)).wrapping_add((((10i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistResult)).wrapping_add(72)).wrapping_add((((9i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    break 's1;
                }
                1i32 => {
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistResult)).wrapping_add(72)).wrapping_add((((11i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    break 's1;
                }
                2i32 => {
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistResult)).wrapping_add(72)).wrapping_add((((12i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    break 's1;
                }
                3i32 => {
                    { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistResult)).wrapping_add(72)).wrapping_add((((13i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    break 's1;
                }
                _ => {
                }
            }
        }
        { let v_ = ({ let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = c.m.r32(fp); let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(68); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(12); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(12); c.m.r32(a_) }).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistResult)).wrapping_add(72)).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistResult)).wrapping_add(72)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistResult)).wrapping_add(72)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistResult)).wrapping_add(72)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistResult)).wrapping_add(72)).wrapping_add((((8i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((4i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistResult)).wrapping_add(72)).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((5i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistResult)).wrapping_add(72)).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistResult)).wrapping_add(72)).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistResult)).wrapping_add(72)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let a0_ = ((c.m.r32(sym::gHistResult)).wrapping_add(24)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); let a1_ = ({ let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a2_ = (c.m.r32(sym::gHistResult)).wrapping_add(72); F::Flash_Create(c, a0_, a1_, a2_) };
        { let a0_ = ((c.m.r32(sym::gHistResult)).wrapping_add(24)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_Play(c, a0_, 1i32) };
        { let a0_ = ({ let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); F::GetWin_Init(c, a0_, 1i32) };
        if (((({ let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(128); (c.m.r32(a_) as i32) }) & (32i32))) != 0) {
            { let v_ = 0i32; let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(152); c.m.w32(a_, (v_) as u32); };
            F::StreamSe_PlayPausedDefault(c, 0i32, 68552i32);
        } else {
            if ((((((F::HistResult_IsMenuCleared(c)) != 0) && ((((!(((({ let a_ = (((c.m.r32(sym::gSaveData)).wrapping_add(8)).wrapping_add(8)).wrapping_add(((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(52); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(16)); (c.m.r32(a_) as i32) }) & (4i32))) != 0)) as i32)) != 0)) as i32)) != 0) {
                F::StreamSe_PlayPausedDefault(c, 0i32, 68553i32);
            } else {
                F::StreamSe_PlayPausedDefault(c, 0i32, 68551i32);
            }
            F::HistResult_BuildRewards(c);
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "HistResult_Init", &PROBE_N, &[], true); }
    r_
}

pub fn HistResult_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HistResult_Term", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let r_ = 'f: {
    F::GetWin_Term(c);
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (1i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let a0_ = ((c.m.r32(sym::gHistResult)).wrapping_add(24)).wrapping_add((((l_i_1) as u32)).wrapping_mul(44)); F::Flash_Destroy(c, a0_) };
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
    if ((((({ let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(20); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = { let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(20); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(20); c.m.w32(a_, v_); };
    }
    if ((((({ let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(16); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = { let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(16); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(16); c.m.w32(a_, v_); };
    }
    if ((((({ let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(12); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = { let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(12); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(12); c.m.w32(a_, v_); };
    }
    if ((((({ let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(8); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = { let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(8); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(8); c.m.w32(a_, v_); };
    }
    if ((((({ let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(4); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = { let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(4); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(4); c.m.w32(a_, v_); };
    }
    if (((((c.m.r32(sym::gHistResult)) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32(sym::gHistResult); F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = sym::gHistResult; c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HistResult_Term", &PROBE_N, &[], true); }
    r_
}

pub fn HistResult_Draw(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HistResult_Draw", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(112) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 112);
    let mut l_i_1: i32 = 0;
    let mut l_flash_2: u32 = 0;
    let r_ = 'f: {
        { let a0_ = { let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(68); c.m.r32(a_) }; F::Sprite_DrawPicture(c, a0_, 0i32, 0i32, 128i32) };
        l_flash_2 = ((c.m.r32(sym::gHistResult)).wrapping_add(24)).wrapping_add((((0i32) as u32)).wrapping_mul(44));
        {
            l_i_1 = 0i32;
            'l1: loop {
                if !(((((l_i_1) < (2i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let _ = F::sprintf(c, fp.wrapping_add(28), sym::menu_c_c__str0, &[(((l_i_1).wrapping_add(1i32)) as i64 as u64)]); };
                    F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), fp.wrapping_add(28), fp);
                    if (((((l_i_1) == (0i32)) as i32)) != 0) {
                        if (((({ let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(128); (c.m.r32(a_) as i32) }) & (32i32))) != 0) {
                            { let _ = F::Flash_ClipSetFlags(c, l_flash_2, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
                        } else {
                            { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                            { let v_ = 0i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                            { let v_ = 512i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                            { let v_ = 128i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                            F::Flash_ClipSetUv(c, l_flash_2, fp, fp.wrapping_add(8));
                            { let _ = F::Flash_ClipSetFlags(c, l_flash_2, fp, 2i32, ((((1i32) as i64 as u64)) as u8)); };
                        }
                    } else {
                        if (((({ let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(128); (c.m.r32(a_) as i32) }) & (32i32))) != 0) {
                            { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                            { let v_ = 128i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                            { let v_ = 512i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                            { let v_ = 256i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                            F::Flash_ClipSetUv(c, l_flash_2, fp, fp.wrapping_add(8));
                            { let _ = F::Flash_ClipSetFlags(c, l_flash_2, fp, 2i32, ((((1i32) as i64 as u64)) as u8)); };
                        } else {
                            { let _ = F::Flash_ClipSetFlags(c, l_flash_2, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
                        }
                    }
                }
                { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
            }
        }
        {
            l_i_1 = 0i32;
            'l3: loop {
                if !(((((l_i_1) < (2i32)) as i32)) != 0) { break 'l3; }
                'c4: {
                    { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = (l_i_1).wrapping_mul(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 512i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = ((c.m.r32((fp.wrapping_add(8)).wrapping_add(4)) as i32)).wrapping_add(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                    { let _ = F::sprintf(c, fp.wrapping_add(28), sym::menu_c_c__str1, &[(((l_i_1).wrapping_add(1i32)) as i64 as u64)]); };
                    F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), fp.wrapping_add(28), fp);
                    F::Flash_ClipSetUv(c, l_flash_2, fp, fp.wrapping_add(8));
                }
                { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
            }
        }
        { let a4_ = { let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(148); (c.m.r32(a_) as i32) }; F::Num_Draw(c, l_flash_2, sym::menu_c_c__str2, 0i32, 4i32, a4_, 64i32, 64i32, 0i32) };
        { let a4_ = { let a_ = ((c.m.r32(sym::gSaveData)).wrapping_add(8)).wrapping_add(12320); (c.m.r32(a_) as i32) }; F::Num_Draw(c, l_flash_2, sym::menu_c_c__str3, 0i32, 7i32, a4_, 64i32, 64i32, 0i32) };
        {
            rt::memset(c, fp.wrapping_add(92), 0, 16);
            { let v_ = F32(0x41888888); c.m.w32(fp.wrapping_add(92).wrapping_add(0), (v_).0); }
            { let v_ = F32(0x41888888); c.m.w32(fp.wrapping_add(92).wrapping_add(4), (v_).0); }
            { let v_ = F32(0x41088888); c.m.w32(fp.wrapping_add(92).wrapping_add(8), (v_).0); }
            { let v_ = F32(0x41088888); c.m.w32(fp.wrapping_add(92).wrapping_add(12), (v_).0); }
            { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
            { let v_ = 0i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
            { let v_ = 512i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
            { let v_ = 256i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
            {
                l_i_1 = 0i32;
                'l5: loop {
                    if !(((((l_i_1) < (4i32)) as i32)) != 0) { break 'l5; }
                    'c6: {
                        { let _ = F::sprintf(c, fp.wrapping_add(28), sym::menu_c_c__str4, &[(((l_i_1).wrapping_add(1i32)) as i64 as u64)]); };
                        F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), fp.wrapping_add(28), fp);
                        { let _ = F::Flash_ClipSetFlags(c, l_flash_2, fp, 32i32, ((((1i32) as i64 as u64)) as u8)); };
                        { let a3_ = ((c.m.r32(sym::gHistResult)).wrapping_add(160)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); let a5_ = rt::fneg(F32(c.m.r32((fp.wrapping_add(92)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4))))); F::FlashAnim_Scroll(c, l_flash_2, fp, fp.wrapping_add(8), a3_, ((0i32) as u32), a5_, F32(0x00000000)) };
                    }
                    { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
                }
            }
        }
        { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = (rt::rem_i32({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(56); (c.m.r32(a_) as i32) }, 4i32)).wrapping_mul(64i32); let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = 512i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = ((c.m.r32((fp.wrapping_add(8)).wrapping_add(4)) as i32)).wrapping_add(64i32); let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), sym::menu_c_c__str5, fp);
        F::Flash_ClipSetUv(c, l_flash_2, fp, fp.wrapping_add(8));
        { let a2_ = rt::div_i32({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(56); (c.m.r32(a_) as i32) }, 4i32); F::Flash_ClipSetTex(c, l_flash_2, fp, a2_) };
        {
            l_i_1 = 0i32;
            'l7: loop {
                if !(((((l_i_1) < (1i32)) as i32)) != 0) { break 'l7; }
                'c8: {
                    { let a0_ = ((c.m.r32(sym::gHistResult)).wrapping_add(24)).wrapping_add((((l_i_1) as u32)).wrapping_mul(44)); F::Flash_Draw(c, a0_) };
                }
                { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
            }
        }
        F::GetWin_Draw(c);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "HistResult_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn HistResult_Update(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HistResult_Update", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let r_ = 'f: {
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (1i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let a0_ = ((c.m.r32(sym::gHistResult)).wrapping_add(24)).wrapping_add((((l_i_1) as u32)).wrapping_mul(44)); F::Flash_Advance(c, a0_) };
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
    if (((({ let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(128); (c.m.r32(a_) as i32) }) & (64i32))) != 0) {
        if (((({ let a_ = (((c.m.r32(sym::gHistResult)).wrapping_add(24)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add(12); c.m.r32(a_) }) & (((1i32) as u32)))) != 0) {
            { let r_ = 1i32; let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(128); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            { let r_ = 2i32; let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(128); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            { let v_ = 1i32; let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(136); c.m.w32(a_, (v_) as u32); };
            { let r_ = 64i32; let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(128); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HistResult_Update", &PROBE_N, &[], true); }
    r_
}

pub fn HistResult_Input(c: &mut Ctx, mut p_result_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HistResult_Input", &PROBE_N, &[(p_result_1) as u32], false); }
    let r_ = 'f: {
    if ((((!(((({ let a_ = (((c.m.r32(sym::gHistResult)).wrapping_add(24)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add(8); c.m.r32(a_) }) & (((2i32) as u32)))) != 0)) as i32)) != 0) {
        break 'f;
    }
    if ((((!(((({ let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(128); (c.m.r32(a_) as i32) }) & (4i32))) != 0)) as i32)) != 0) {
        { let r_ = 4i32; let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(128); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        if ((((!(((({ let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(128); (c.m.r32(a_) as i32) }) & (32i32))) != 0)) as i32)) != 0) {
            { let v_ = 1i32; let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(144); c.m.w32(a_, (v_) as u32); };
            break 'f;
        }
    }
    if ((((({ let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(140); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
        break 'f;
    }
    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
        if (((({ let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(128); (c.m.r32(a_) as i32) }) & (32i32))) != 0) {
            { let v_ = 0i32; let a_ = p_result_1; c.m.w32(a_, (v_) as u32); };
        } else {
            if ((F::HistResult_IsMenuCleared(c)) != 0) {
                if (((({ let a_ = (((c.m.r32(sym::gSaveData)).wrapping_add(8)).wrapping_add(8)).wrapping_add(((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(52); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(16)); (c.m.r32(a_) as i32) }) & (4i32))) != 0) {
                    { let v_ = 1i32; let a_ = p_result_1; c.m.w32(a_, (v_) as u32); };
                } else {
                    { let v_ = 2i32; let a_ = p_result_1; c.m.w32(a_, (v_) as u32); };
                }
            } else {
                { let v_ = 1i32; let a_ = p_result_1; c.m.w32(a_, (v_) as u32); };
            }
        }
        { let v_ = 51i32; let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(144); c.m.w32(a_, (v_) as u32); };
        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HistResult_Input", &PROBE_N, &[], true); }
    r_
}

pub fn HistResult_UpdateState(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HistResult_UpdateState", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if ((((({ let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(144); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
        break 'f;
    }
    if ((((({ let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(156); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
        break 'f;
    }
    's1: {
        let sv1: i32 = { let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(144); (c.m.r32(a_) as i32) };
        match sv1 {
            1i32 => {
                { let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(144); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            2i32 => {
                if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                    { let v_ = { let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(152); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(148); c.m.w32(a_, (v_) as u32); };
                    { let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(144); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                } else {
                    { let r_ = 33i32; let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(148); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                    if ((((({ let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(148); (c.m.r32(a_) as i32) }) >= ({ let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(152); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                        { let v_ = { let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(152); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(148); c.m.w32(a_, (v_) as u32); };
                        { let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(144); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    }
                }
                break 's1;
            }
            4i32 => {
                if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                    { let a0_ = { let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(148); (c.m.r32(a_) as i32) }; F::Save_AddMoney(c, a0_) };
                    { let v_ = 0i32; let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(148); c.m.w32(a_, (v_) as u32); };
                    { let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(144); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                } else {
                    if ((((({ let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(148); (c.m.r32(a_) as i32) }) <= (32i32)) as i32)) != 0) {
                        { let a0_ = { let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(148); (c.m.r32(a_) as i32) }; F::Save_AddMoney(c, a0_) };
                        { let v_ = 0i32; let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(148); c.m.w32(a_, (v_) as u32); };
                        { let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(144); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    } else {
                        { let r_ = 33i32; let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(148); let n_ = ((c.m.r32(a_) as i32)).wrapping_sub(r_); c.m.w32(a_, (n_) as u32); };
                        F::Save_AddMoney(c, 33i32);
                    }
                }
                break 's1;
            }
            3i32 => {
                if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                    { let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(144); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                }
                break 's1;
            }
            5i32 => {
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(144); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            51i32 => {
                if ((((({ let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(312); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
                    { let v_ = 57i32; let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(144); c.m.w32(a_, (v_) as u32); };
                } else {
                    { let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(144); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    { let v_ = 0i32; let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(316); c.m.w32(a_, (v_) as u32); };
                }
                break 's1;
            }
            52i32 => {
                { let a0_ = { let a_ = ((c.m.r32(sym::gHistResult)).wrapping_add(176)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(316); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(8)); (c.m.r32(a_) as i32) }; let a1_ = { let a_ = (((c.m.r32(sym::gHistResult)).wrapping_add(176)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(316); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(8))).wrapping_add(4); (c.m.r32(a_) as i32) }; F::GetWin_Setup(c, a0_, a1_) };
                if ((((({ let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(316); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
                    F::GetWin_Next(c);
                } else {
                    F::GetWin_Open(c);
                }
                { let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(316); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(144); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            53i32 | 56i32 => {
                if ((F::GetWin_IsAnimating(c)) != 0) {
                    { let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(144); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                }
                break 's1;
            }
            54i32 => {
                if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                    if ((((({ let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(316); (c.m.r32(a_) as i32) }) == ({ let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(312); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                        { let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(144); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    } else {
                        { let v_ = 52i32; let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(144); c.m.w32(a_, (v_) as u32); };
                    }
                    { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                }
                break 's1;
            }
            55i32 => {
                { let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(144); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                F::GetWin_Close(c);
                break 's1;
            }
            57i32 => {
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(144); c.m.w32(a_, (v_) as u32); };
                { let r_ = 64i32; let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(128); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a0_ = ((c.m.r32(sym::gHistResult)).wrapping_add(24)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_c_c__str6, ((((1i32) as i64 as u64)) as u8)) };
                break 's1;
            }
            _ => {
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HistResult_UpdateState", &PROBE_N, &[], true); }
    r_
}

pub fn HistResult_UpdateSe(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HistResult_UpdateSe", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if ((((({ let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(156); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
        break 'f;
    }
    's1: {
        let sv1: i32 = { let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(156); (c.m.r32(a_) as i32) };
        match sv1 {
            1i32 => {
                { let v_ = 2i32; let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(156); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            2i32 => {
                if (((((F::StreamSe_GetStat(c, 0i32)) == (5i32)) as i32)) != 0) {
                    { let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(156); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                }
                break 's1;
            }
            3i32 => {
                F::Bgm_Play(c, 68376i32);
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(156); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            _ => {
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HistResult_UpdateSe", &PROBE_N, &[], true); }
    r_
}

pub fn HistResult_Run(c: &mut Ctx, mut p_section_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HistResult_Run", &PROBE_N, &[(p_section_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        { let v_ = 1i32; c.m.w32(fp, (v_) as u32); }
        F::HistResult_Init(c, p_section_1);
        F::ColorFade_StartIn(c, 0i32, 0i32, 0i32, 20i32);
        'l1: while ((1i32) != 0) {
            F::Gfx_BeginFrame(c);
            F::Pad_Update(c);
            F::Snd_Update(c);
            F::ColorFade_Update(c);
            if ((((!(((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(20); (c.m.r32(a_) as i32) }) & (256i32))) != 0)) as i32)) != 0) {
                F::HistResult_Update(c);
                F::HistResult_UpdateSe(c);
                F::HistResult_UpdateState(c);
            }
            F::HistResult_Draw(c);
            F::ColorFade_Draw(c);
            F::Gfx_EndFrame(c, 1i32);
            F::Dma_Flush(c);
            F::File_Stub264D90(c);
            if (((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(20); (c.m.r32(a_) as i32) }) & (256i32))) != 0) {
                continue 'l1;
            }
            if ((F::ColorFade_IsInDone(c)) != 0) {
                if ((((!(((({ let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(128); (c.m.r32(a_) as i32) }) & (8i32))) != 0)) as i32)) != 0) {
                    { let r_ = 8i32; let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(128); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    { let v_ = 1i32; let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(156); c.m.w32(a_, (v_) as u32); };
                    F::StreamSe_Resume(c, 0i32);
                }
            }
            if ((F::ColorFade_IsFadingOut(c)) != 0) {
                F::Bgm_FadeOutStep(c);
                continue 'l1;
            }
            if ((F::ColorFade_IsOutDone(c)) != 0) {
                break 'l1;
            }
            if (((({ let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(128); (c.m.r32(a_) as i32) }) & (2i32))) != 0) {
                if ((((({ let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(136); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) == ((1i32).wrapping_neg())) as i32)) != 0) {
                    F::ColorFade_StartOut(c, 0i32, 0i32, 0i32, 20i32);
                }
            } else {
                if ((((((((((((({ let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(156); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) && ((((({ let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(144); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0)) as i32)) != 0) && ((((!(((({ let a_ = (c.m.r32(sym::gHistResult)).wrapping_add(128); (c.m.r32(a_) as i32) }) & (64i32))) != 0)) as i32)) != 0)) as i32)) != 0) {
                    F::HistResult_Input(c, fp);
                }
            }
        }
        F::HistResult_Term(c);
        F::Dma_ResetBuffers(c);
        break 'f (c.m.r32(fp) as i32);
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "HistResult_Run", &PROBE_N, &[], true); }
    r_
}
