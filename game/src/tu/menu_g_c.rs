#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn DuelMenu_Idle(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DuelMenu_Idle", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(304); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
    if ((((({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(304); (c.m.r32(a_) as i32) }) == (3600i32)) as i32)) != 0) {
        { let v_ = 1i32; let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(332); c.m.w32(a_, (v_) as u32); };
        { let v_ = 8i32; let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(300); c.m.w32(a_, (v_) as u32); };
        { let a0_ = { let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(12); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(300); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34247i32, a2_) };
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(304); c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "DuelMenu_Idle", &PROBE_N, &[], true); }
    r_
}

pub fn DuelMenu_ShowValues(c: &mut Ctx, mut p_item_1: i32, mut p_show_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DuelMenu_ShowValues", &PROBE_N, &[(p_item_1) as u32, (p_show_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(160) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 160);
    let mut l_flash_3: u32 = 0;
    let mut l_i_4: i32 = 0;
    let mut l_j_5: i32 = 0;
    let r_ = 'f: {
        l_flash_3 = ((c.m.r32(sym::gDuelMenu)).wrapping_add(20)).wrapping_add((((0i32) as u32)).wrapping_mul(44));
        's1: {
            let sv6: i32 = p_item_1;
            match sv6 {
                0i32 => {
                    {
                        l_i_4 = 0i32;
                        'l2: loop {
                            if !(((((l_i_4) < (5i32)) as i32)) != 0) { break 'l2; }
                            'c3: {
                                { let _ = F::sprintf(c, fp.wrapping_add(28), sym::menu_g_c__str0, &[(((p_item_1).wrapping_add(1i32)) as i64 as u64), (((l_i_4).wrapping_add(1i32)) as i64 as u64)]); };
                                F::Flash_FindLabel(c, l_flash_3, ((0i32) as u32), fp.wrapping_add(28), fp);
                                { let _ = F::Flash_ClipSetFlags(c, l_flash_3, fp, 2i32, ((((((((p_show_2) as u8)) as i32)) as i64 as u64)) as u8)); };
                                if ((p_show_2) != 0) {
                                    { let v_ = (l_i_4).wrapping_mul(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                                    { let v_ = ((c.m.r32((fp.wrapping_add(8)).wrapping_add(4)) as i32)).wrapping_add(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                                    { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                                    { let v_ = 256i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                                    { let _ = F::sprintf(c, fp.wrapping_add(92), sym::menu_g_c__str1, &[(((p_item_1).wrapping_add(1i32)) as i64 as u64)]); };
                                    F::Flash_FindLabel(c, l_flash_3, fp.wrapping_add(28), fp.wrapping_add(92), fp);
                                    F::Flash_ClipSetUv(c, l_flash_3, fp, fp.wrapping_add(8));
                                    { let _ = F::sprintf(c, fp.wrapping_add(92), sym::menu_g_c__str2, &[(((p_item_1).wrapping_add(1i32)) as i64 as u64)]); };
                                    F::Flash_FindLabel(c, l_flash_3, fp.wrapping_add(28), fp.wrapping_add(92), fp);
                                    F::Flash_ClipSetUv(c, l_flash_3, fp, fp.wrapping_add(8));
                                }
                            }
                            { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
                        }
                    }
                    break 's1;
                }
                1i32 => {
                    {
                        l_i_4 = 0i32;
                        'l4: loop {
                            if !(((((l_i_4) < (5i32)) as i32)) != 0) { break 'l4; }
                            'c5: {
                                { let _ = F::sprintf(c, fp.wrapping_add(28), sym::menu_g_c__str3, &[(((p_item_1).wrapping_add(1i32)) as i64 as u64), (((l_i_4).wrapping_add(1i32)) as i64 as u64)]); };
                                F::Flash_FindLabel(c, l_flash_3, ((0i32) as u32), fp.wrapping_add(28), fp);
                                { let _ = F::Flash_ClipSetFlags(c, l_flash_3, fp, 2i32, ((((((((p_show_2) as u8)) as i32)) as i64 as u64)) as u8)); };
                                if ((p_show_2) != 0) {
                                    { let v_ = (l_i_4).wrapping_mul(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                                    { let v_ = ((c.m.r32((fp.wrapping_add(8)).wrapping_add(4)) as i32)).wrapping_add(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                                    { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                                    { let v_ = 256i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                                    { let _ = F::sprintf(c, fp.wrapping_add(92), sym::menu_g_c__str4, &[(((p_item_1).wrapping_add(1i32)) as i64 as u64)]); };
                                    F::Flash_FindLabel(c, l_flash_3, fp.wrapping_add(28), fp.wrapping_add(92), fp);
                                    F::Flash_ClipSetUv(c, l_flash_3, fp, fp.wrapping_add(8));
                                    { let _ = F::sprintf(c, fp.wrapping_add(92), sym::menu_g_c__str5, &[(((p_item_1).wrapping_add(1i32)) as i64 as u64)]); };
                                    F::Flash_FindLabel(c, l_flash_3, fp.wrapping_add(28), fp.wrapping_add(92), fp);
                                    F::Flash_ClipSetUv(c, l_flash_3, fp, fp.wrapping_add(8));
                                }
                            }
                            { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
                        }
                    }
                    break 's1;
                }
                2i32 => {
                    {
                        l_i_4 = 0i32;
                        'l6: loop {
                            if !(((((l_i_4) < (7i32)) as i32)) != 0) { break 'l6; }
                            'c7: {
                                { let _ = F::sprintf(c, fp.wrapping_add(28), sym::menu_g_c__str6, &[(((p_item_1).wrapping_add(1i32)) as i64 as u64), (((l_i_4).wrapping_add(1i32)) as i64 as u64)]); };
                                F::Flash_FindLabel(c, l_flash_3, ((0i32) as u32), fp.wrapping_add(28), fp);
                                { let _ = F::Flash_ClipSetFlags(c, l_flash_3, fp, 2i32, ((((((((p_show_2) as u8)) as i32)) as i64 as u64)) as u8)); };
                                if ((p_show_2) != 0) {
                                    { let v_ = (l_i_4).wrapping_mul(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                                    { let v_ = ((c.m.r32((fp.wrapping_add(8)).wrapping_add(4)) as i32)).wrapping_add(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                                    { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                                    { let v_ = 256i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                                    { let _ = F::sprintf(c, fp.wrapping_add(92), sym::menu_g_c__str7, &[(((p_item_1).wrapping_add(1i32)) as i64 as u64)]); };
                                    F::Flash_FindLabel(c, l_flash_3, fp.wrapping_add(28), fp.wrapping_add(92), fp);
                                    F::Flash_ClipSetUv(c, l_flash_3, fp, fp.wrapping_add(8));
                                    { let _ = F::sprintf(c, fp.wrapping_add(92), sym::menu_g_c__str8, &[(((p_item_1).wrapping_add(1i32)) as i64 as u64)]); };
                                    F::Flash_FindLabel(c, l_flash_3, fp.wrapping_add(28), fp.wrapping_add(92), fp);
                                    F::Flash_ClipSetUv(c, l_flash_3, fp, fp.wrapping_add(8));
                                }
                            }
                            { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
                        }
                    }
                    break 's1;
                }
                3i32 => {
                    F::Flash_FindLabel(c, l_flash_3, ((0i32) as u32), sym::menu_g_c__str9, fp);
                    { let _ = F::Flash_ClipSetFlags(c, l_flash_3, fp, 2i32, ((((((((p_show_2) as u8)) as i32)) as i64 as u64)) as u8)); };
                    {
                        l_i_4 = 0i32;
                        'l8: loop {
                            if !(((((l_i_4) < (2i32)) as i32)) != 0) { break 'l8; }
                            'c9: {
                                {
                                    l_j_5 = 0i32;
                                    'l10: loop {
                                        if !(((((l_j_5) < (2i32)) as i32)) != 0) { break 'l10; }
                                        'c11: {
                                            { let _ = F::sprintf(c, fp.wrapping_add(28), (if ((l_i_4) != 0) { sym::menu_g_c__str10 } else { sym::menu_g_c__str11 }), &[(((p_item_1).wrapping_add(1i32)) as i64 as u64), (((l_j_5).wrapping_add(1i32)) as i64 as u64)]); };
                                            F::Flash_FindLabel(c, l_flash_3, ((0i32) as u32), fp.wrapping_add(28), fp);
                                            { let _ = F::Flash_ClipSetFlags(c, l_flash_3, fp, 2i32, ((((((((p_show_2) as u8)) as i32)) as i64 as u64)) as u8)); };
                                            if ((p_show_2) != 0) {
                                                { let v_ = (l_j_5).wrapping_mul(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                                                { let v_ = ((c.m.r32((fp.wrapping_add(8)).wrapping_add(4)) as i32)).wrapping_add(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                                                { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                                                { let v_ = 256i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                                                F::Flash_FindLabel(c, l_flash_3, fp.wrapping_add(28), sym::menu_g_c__str12, fp);
                                                F::Flash_ClipSetUv(c, l_flash_3, fp, fp.wrapping_add(8));
                                                F::Flash_FindLabel(c, l_flash_3, fp.wrapping_add(28), sym::menu_g_c__str13, fp);
                                                F::Flash_ClipSetUv(c, l_flash_3, fp, fp.wrapping_add(8));
                                            }
                                        }
                                        { let o_ = l_j_5; let n_ = (o_).wrapping_add(1); l_j_5 = n_; };
                                    }
                                }
                            }
                            { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
                        }
                    }
                    break 's1;
                }
                4i32 => {
                    {
                        l_i_4 = 0i32;
                        'l12: loop {
                            if !(((((l_i_4) < (2i32)) as i32)) != 0) { break 'l12; }
                            'c13: {
                                { let _ = F::sprintf(c, fp.wrapping_add(28), sym::menu_g_c__str14, &[(((p_item_1).wrapping_add(1i32)) as i64 as u64), (((l_i_4).wrapping_add(1i32)) as i64 as u64)]); };
                                F::Flash_FindLabel(c, l_flash_3, ((0i32) as u32), fp.wrapping_add(28), fp);
                                { let _ = F::Flash_ClipSetFlags(c, l_flash_3, fp, 2i32, ((((((((p_show_2) as u8)) as i32)) as i64 as u64)) as u8)); };
                                if ((p_show_2) != 0) {
                                    { let v_ = (l_i_4).wrapping_mul(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                                    { let v_ = ((c.m.r32((fp.wrapping_add(8)).wrapping_add(4)) as i32)).wrapping_add(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                                    { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                                    { let v_ = 256i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                                    F::Flash_FindLabel(c, l_flash_3, fp.wrapping_add(28), sym::menu_g_c__str15, fp);
                                    F::Flash_ClipSetUv(c, l_flash_3, fp, fp.wrapping_add(8));
                                    F::Flash_FindLabel(c, l_flash_3, fp.wrapping_add(28), sym::menu_g_c__str16, fp);
                                    F::Flash_ClipSetUv(c, l_flash_3, fp, fp.wrapping_add(8));
                                }
                            }
                            { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
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
    if rt::probing() { crate::host::harness::probe(c, "DuelMenu_ShowValues", &PROBE_N, &[], true); }
    r_
}

pub fn DuelMenu_SetupValue(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DuelMenu_SetupValue", &PROBE_N, &[], false); }
    let r_ = 'f: {
    's1: {
        let sv1: i32 = { let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(272)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
        match sv1 {
            0i32 => {
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(308); c.m.w32(a_, (v_) as u32); };
                { let v_ = 5i32; let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(324); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(312); c.m.w32(a_, (v_) as u32); };
                { let v_ = { let a_ = ((c.m.r32(sym::gSaveData)).wrapping_add(3124)).wrapping_add(((({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(308); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(316)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(316)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = ({ let a_ = ({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(264); c.m.r32(a_) }).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(68)).wrapping_add((((47i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(68)).wrapping_add((((48i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                break 's1;
            }
            1i32 => {
                { let v_ = 1i32; let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(308); c.m.w32(a_, (v_) as u32); };
                { let v_ = 5i32; let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(324); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(312); c.m.w32(a_, (v_) as u32); };
                { let v_ = { let a_ = ((c.m.r32(sym::gSaveData)).wrapping_add(3124)).wrapping_add(((({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(308); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(316)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(316)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = ({ let a_ = ({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(264); c.m.r32(a_) }).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(68)).wrapping_add((((47i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(68)).wrapping_add((((48i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                break 's1;
            }
            2i32 => {
                { let v_ = 2i32; let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(308); c.m.w32(a_, (v_) as u32); };
                { let v_ = 7i32; let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(324); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(312); c.m.w32(a_, (v_) as u32); };
                { let v_ = { let a_ = ((c.m.r32(sym::gSaveData)).wrapping_add(3124)).wrapping_add(((({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(308); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(316)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(316)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = ({ let a_ = ({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(264); c.m.r32(a_) }).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(68)).wrapping_add((((47i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(68)).wrapping_add((((48i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                break 's1;
            }
            3i32 => {
                { let v_ = 3i32; let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(308); c.m.w32(a_, (v_) as u32); };
                { let v_ = 2i32; let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(324); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(312); c.m.w32(a_, (v_) as u32); };
                { let v_ = { let a_ = ((c.m.r32(sym::gSaveData)).wrapping_add(3124)).wrapping_add(((({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(308); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(316)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = { let a_ = ((c.m.r32(sym::gSaveData)).wrapping_add(3124)).wrapping_add((((({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(308); (c.m.r32(a_) as i32) }).wrapping_add(1i32)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(316)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(68)).wrapping_add((((47i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                { let v_ = ({ let a_ = ({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(264); c.m.r32(a_) }).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(68)).wrapping_add((((48i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                break 's1;
            }
            4i32 => {
                { let v_ = 5i32; let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(308); c.m.w32(a_, (v_) as u32); };
                { let v_ = 2i32; let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(324); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(312); c.m.w32(a_, (v_) as u32); };
                { let v_ = { let a_ = ((c.m.r32(sym::gSaveData)).wrapping_add(3124)).wrapping_add(((({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(308); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(316)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(316)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(68)).wrapping_add((((47i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                { let v_ = ({ let a_ = ({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(264); c.m.r32(a_) }).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(68)).wrapping_add((((48i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                break 's1;
            }
            _ => {
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "DuelMenu_SetupValue", &PROBE_N, &[], true); }
    r_
}

pub fn DuelMenu_Init(c: &mut Ctx, mut p_section_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DuelMenu_Init", &PROBE_N, &[(p_section_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
        { let v_ = ((0i32) as u32); c.m.w32(fp, v_); }
        { let v_ = F::Heap_Alloc(c, 404i32, ((32i32) as u32), 0i32, 2i32); let a_ = sym::gDuelMenu; c.m.w32(a_, v_); };
        { let _ = { let a0_ = c.m.r32(sym::gDuelMenu); F::memset(c, a0_, 0i32, ((404i32) as u32)) }; };
        { let v_ = (c.m.r32(sym::gMenuArc5)).wrapping_add((({ let a_ = (c.m.r32(sym::gMenuArc5)).wrapping_add((((p_section_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = c.m.r32(sym::gDuelMenu); c.m.w32(a_, v_); };
        { let v_ = { let a0_ = { let a_ = c.m.r32(sym::gDuelMenu); c.m.r32(a_) }; F::Sprite_Unpack(c, a0_, ((0i32) as u32), ((0i32) as u32)) }; let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(4); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = c.m.r32(fp); let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(64); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(68)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(68)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(68)).wrapping_add((((8i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(68)).wrapping_add((((38i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(68)).wrapping_add((((39i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(68)).wrapping_add((((40i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(68)).wrapping_add((((35i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(68)).wrapping_add((((36i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(68)).wrapping_add((((37i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(68)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(68)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(68)).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(68)).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((4i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(68)).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((5i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(68)).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(68)).wrapping_add((((12i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(68)).wrapping_add((((15i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(68)).wrapping_add((((14i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(68)).wrapping_add((((10i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(68)).wrapping_add((((17i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((8i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(68)).wrapping_add((((13i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(68)).wrapping_add((((16i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(68)).wrapping_add((((9i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(68)).wrapping_add((((11i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((9i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(68)).wrapping_add((((41i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((10i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(68)).wrapping_add((((42i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(68)).wrapping_add((((47i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(68)).wrapping_add((((48i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = c.m.r32(fp); let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(264); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((11i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(68)).wrapping_add((((46i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(68)).wrapping_add((((45i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((12i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(68)).wrapping_add((((43i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(68)).wrapping_add((((18i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(68)).wrapping_add((((19i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((13i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(68)).wrapping_add((((20i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(68)).wrapping_add((((21i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((14i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(68)).wrapping_add((((34i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(68)).wrapping_add((((44i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((15i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(68)).wrapping_add((((22i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(68)).wrapping_add((((23i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((16i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(68)).wrapping_add((((24i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(68)).wrapping_add((((25i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((17i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(68)).wrapping_add((((32i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(68)).wrapping_add((((33i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(68)).wrapping_add((((28i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(68)).wrapping_add((((29i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((4i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(68)).wrapping_add((((30i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((5i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(68)).wrapping_add((((31i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((18i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(68)).wrapping_add((((26i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(68)).wrapping_add((((27i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let a0_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(20)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); let a1_ = ({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((19i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a2_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(68); F::Flash_Create(c, a0_, a1_, a2_) };
        { let a0_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(20)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_Play(c, a0_, 1i32) };
        { let v_ = ({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((24i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let a0_ = ({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((23i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a1_ = c.m.r32(fp); F::IconWin_Init(c, a0_, a1_) };
        F::IconWin_Open(c);
        { let v_ = ({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((21i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(8); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((26i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(12); c.m.w32(a_, v_); };
        { let a0_ = ({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((22i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a1_ = { let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(8); c.m.r32(a_) }; let _ = 0i32; F::MsgWin_Init(c, a0_, a1_, 1i32) };
        F::MsgWin_Open(c);
        { let v_ = ({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((27i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(16); c.m.w32(a_, v_); };
        { let a0_ = ({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((25i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a1_ = { let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(16); c.m.r32(a_) }; F::Dialog_Init(c, a0_, a1_, 0i32) };
        F::Dialog_SetLayout(c, 0i32);
        {
            l_i_2 = 0i32;
            'l1: loop {
                if !(((((l_i_2) < (2i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let v_ = ((F::Rand_Range(c, ((32i32) as u32))) as i32); let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(336)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        { let v_ = (1i32).wrapping_neg(); let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(300); c.m.w32(a_, (v_) as u32); };
        { let v_ = { let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1568); (c.m.r32(a_) as i32) }; let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(272)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = { let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1572); (c.m.r32(a_) as i32) }; let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(272)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = { let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1584); (c.m.r32(a_) as i32) }; let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(272)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "DuelMenu_Init", &PROBE_N, &[], true); }
    r_
}

pub fn DuelMenu_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DuelMenu_Term", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let r_ = 'f: {
    F::Dialog_Term(c);
    F::MsgWin_Term(c);
    F::IconWin_Term(c);
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (1i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let a0_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(20)).wrapping_add((((l_i_1) as u32)).wrapping_mul(44)); F::Flash_Destroy(c, a0_) };
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
    if ((((({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(4); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = { let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(4); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(4); c.m.w32(a_, v_); };
    }
    if (((((c.m.r32(sym::gDuelMenu)) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32(sym::gDuelMenu); F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = sym::gDuelMenu; c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "DuelMenu_Term", &PROBE_N, &[], true); }
    r_
}

pub fn DuelMenu_Draw(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DuelMenu_Draw", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(256) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 256);
    let mut l_i_1: i32 = 0;
    let mut l_j_2: i32 = 0;
    let mut l_rule_3: i32 = 0;
    let mut l_flash_4: u32 = 0;
    let r_ = 'f: {
        { let a0_ = { let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(64); c.m.r32(a_) }; F::Sprite_DrawPicture(c, a0_, 0i32, 0i32, 128i32) };
        l_flash_4 = ((c.m.r32(sym::gDuelMenu)).wrapping_add(20)).wrapping_add((((0i32) as u32)).wrapping_mul(44));
        {
            l_i_1 = 0i32;
            'l1: loop {
                if !(((((l_i_1) < (2i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    F::Flash_FindLabel(c, l_flash_4, (if ((l_i_1) != 0) { sym::menu_g_c__str17 } else { sym::menu_g_c__str18 }), (if ((l_i_1) != 0) { sym::menu_g_c__str19 } else { sym::menu_g_c__str20 }), fp);
                    { let a2_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(336)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); F::FlashAnim_Blink(c, l_flash_4, fp, a2_, 0i32) };
                    F::Flash_FindLabel(c, l_flash_4, (if ((l_i_1) != 0) { sym::menu_g_c__str21 } else { sym::menu_g_c__str22 }), (if ((l_i_1) != 0) { sym::menu_g_c__str23 } else { sym::menu_g_c__str24 }), fp);
                    if ((((({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(332); (c.m.r32(a_) as i32) }) == (l_i_1)) as i32)) != 0) {
                        { let a2_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(344)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); F::FlashAnim_Talk(c, l_flash_4, fp, a2_, 0i32) };
                    } else {
                        F::FlashAnim_ShowNext2(c, l_flash_4, fp, 0i32);
                    }
                }
                { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
            }
        }
        l_rule_3 = 0i32;
        {
            l_i_1 = 0i32;
            'l3: loop {
                if !(((((l_i_1) < (4i32)) as i32)) != 0) { break 'l3; }
                'c4: {
                    { let v_ = (l_i_1).wrapping_mul(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    { let v_ = ((c.m.r32((fp.wrapping_add(8)).wrapping_add(4)) as i32)).wrapping_add(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 512i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let _ = F::sprintf(c, fp.wrapping_add(28), sym::menu_g_c__str25, &[(((l_i_1).wrapping_add(1i32)) as i64 as u64)]); };
                    if (((((l_i_1) == (1i32)) as i32)) != 0) {
                        F::Flash_FindLabel(c, l_flash_4, ((0i32) as u32), fp.wrapping_add(28), fp);
                        { let a2_ = (if (((({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(268); (c.m.r32(a_) as i32) }) & (16i32))) != 0) { F32(0x3ecccccc) } else { F32(0x3f800000) }); F::Flash_ClipSetColor(c, l_flash_4, fp, a2_) };
                    }
                    F::Flash_FindLabel(c, l_flash_4, fp.wrapping_add(28), sym::menu_g_c__str26, fp);
                    F::Flash_ClipSetUv(c, l_flash_4, fp, fp.wrapping_add(8));
                    F::Flash_FindLabel(c, l_flash_4, fp.wrapping_add(28), sym::menu_g_c__str27, fp);
                    F::Flash_ClipSetUv(c, l_flash_4, fp, fp.wrapping_add(8));
                }
                { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
            }
        }
        {
            l_i_1 = 0i32;
            'l5: loop {
                if !(((((l_i_1) < (3i32)) as i32)) != 0) { break 'l5; }
                'c6: {
                    { let v_ = (l_i_1).wrapping_mul(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    { let v_ = ((c.m.r32((fp.wrapping_add(8)).wrapping_add(4)) as i32)).wrapping_add(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 512i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let _ = F::sprintf(c, fp.wrapping_add(28), sym::menu_g_c__str28, &[(((l_i_1).wrapping_add(1i32)) as i64 as u64)]); };
                    F::Flash_FindLabel(c, l_flash_4, fp.wrapping_add(28), sym::menu_g_c__str29, fp);
                    F::Flash_ClipSetUv(c, l_flash_4, fp, fp.wrapping_add(8));
                    F::Flash_FindLabel(c, l_flash_4, fp.wrapping_add(28), sym::menu_g_c__str30, fp);
                    F::Flash_ClipSetUv(c, l_flash_4, fp, fp.wrapping_add(8));
                }
                { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
            }
        }
        { let v_ = 16i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        { let v_ = 64i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        F::Flash_FindLabel(c, l_flash_4, ((0i32) as u32), sym::menu_g_c__str31, fp);
        { let a2_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(352); let a3_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(356); F::FlashAnim_Sheet(c, l_flash_4, fp, a2_, a3_, fp.wrapping_add(8), 1i32, 4i32, 4i32) };
        {
            l_i_1 = 0i32;
            'l7: loop {
                if !(((((l_i_1) < (3i32)) as i32)) != 0) { break 'l7; }
                'c8: {
                    { let v_ = (l_i_1).wrapping_mul(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    { let v_ = ((c.m.r32((fp.wrapping_add(8)).wrapping_add(4)) as i32)).wrapping_add(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 256i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let _ = F::sprintf(c, fp.wrapping_add(28), sym::menu_g_c__str32, &[(((l_i_1).wrapping_add(1i32)) as i64 as u64)]); };
                    F::Flash_FindLabel(c, l_flash_4, fp.wrapping_add(28), sym::menu_g_c__str33, fp);
                    F::Flash_ClipSetUv(c, l_flash_4, fp, fp.wrapping_add(8));
                    F::Flash_FindLabel(c, l_flash_4, fp.wrapping_add(28), sym::menu_g_c__str34, fp);
                    F::Flash_ClipSetUv(c, l_flash_4, fp, fp.wrapping_add(8));
                }
                { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
            }
        }
        {
            l_i_1 = 0i32;
            'l9: loop {
                if !(((((l_i_1) < (6i32)) as i32)) != 0) { break 'l9; }
                'c10: {
                    { let v_ = (l_i_1).wrapping_mul(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    { let v_ = ((c.m.r32((fp.wrapping_add(8)).wrapping_add(4)) as i32)).wrapping_add(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 256i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let _ = F::sprintf(c, fp.wrapping_add(28), sym::menu_g_c__str35, &[(((l_i_1).wrapping_add(1i32)) as i64 as u64)]); };
                    F::Flash_FindLabel(c, l_flash_4, fp.wrapping_add(28), sym::menu_g_c__str36, fp);
                    F::Flash_ClipSetUv(c, l_flash_4, fp, fp.wrapping_add(8));
                    F::Flash_FindLabel(c, l_flash_4, fp.wrapping_add(28), sym::menu_g_c__str37, fp);
                    F::Flash_ClipSetUv(c, l_flash_4, fp, fp.wrapping_add(8));
                }
                { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
            }
        }
        {
            l_i_1 = 0i32;
            'l11: loop {
                if !(((((l_i_1) < (5i32)) as i32)) != 0) { break 'l11; }
                'c12: {
                    's13: {
                        let sv5: i32 = l_i_1;
                        match sv5 {
                            0i32 => {
                                l_rule_3 = 0i32;
                                break 's13;
                            }
                            1i32 => {
                                l_rule_3 = 1i32;
                                break 's13;
                            }
                            2i32 => {
                                l_rule_3 = 2i32;
                                break 's13;
                            }
                            3i32 => {
                                l_rule_3 = 3i32;
                                break 's13;
                            }
                            4i32 => {
                                l_rule_3 = 5i32;
                                break 's13;
                            }
                            _ => {
                            }
                        }
                    }
                    's14: {
                        let sv6: i32 = l_i_1;
                        match sv6 {
                            0i32 | 1i32 | 2i32 => {
                                { let v_ = ({ let a_ = ((c.m.r32(sym::gSaveData)).wrapping_add(3124)).wrapping_add((((l_rule_3) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_mul(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                                { let v_ = ((c.m.r32((fp.wrapping_add(8)).wrapping_add(4)) as i32)).wrapping_add(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                                { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                                { let v_ = 256i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                                { let _ = F::sprintf(c, fp.wrapping_add(28), sym::menu_g_c__str38, &[(((l_i_1).wrapping_add(1i32)) as i64 as u64)]); };
                                { let _ = F::sprintf(c, fp.wrapping_add(92), sym::menu_g_c__str39, &[(((l_i_1).wrapping_add(1i32)) as i64 as u64)]); };
                                F::Flash_FindLabel(c, l_flash_4, fp.wrapping_add(28), fp.wrapping_add(92), fp);
                                F::Flash_ClipSetUv(c, l_flash_4, fp, fp.wrapping_add(8));
                                { let _ = F::sprintf(c, fp.wrapping_add(92), sym::menu_g_c__str40, &[(((l_i_1).wrapping_add(1i32)) as i64 as u64)]); };
                                F::Flash_FindLabel(c, l_flash_4, fp.wrapping_add(28), fp.wrapping_add(92), fp);
                                F::Flash_ClipSetUv(c, l_flash_4, fp, fp.wrapping_add(8));
                                break 's14;
                            }
                            4i32 => {
                                { let v_ = ({ let a_ = ((c.m.r32(sym::gSaveData)).wrapping_add(3124)).wrapping_add((((l_rule_3) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_mul(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                                { let v_ = ((c.m.r32((fp.wrapping_add(8)).wrapping_add(4)) as i32)).wrapping_add(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                                { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                                { let v_ = 256i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                                { let _ = F::sprintf(c, fp.wrapping_add(28), sym::menu_g_c__str41, &[(((l_i_1).wrapping_add(1i32)) as i64 as u64)]); };
                                F::Flash_FindLabel(c, l_flash_4, fp.wrapping_add(28), sym::menu_g_c__str42, fp);
                                F::Flash_ClipSetUv(c, l_flash_4, fp, fp.wrapping_add(8));
                                F::Flash_FindLabel(c, l_flash_4, fp.wrapping_add(28), sym::menu_g_c__str43, fp);
                                F::Flash_ClipSetUv(c, l_flash_4, fp, fp.wrapping_add(8));
                                break 's14;
                            }
                            3i32 => {
                                {
                                    l_j_2 = 0i32;
                                    'l15: loop {
                                        if !(((((l_j_2) < (2i32)) as i32)) != 0) { break 'l15; }
                                        'c16: {
                                            { let v_ = ({ let a_ = (((c.m.r32(sym::gSaveData)).wrapping_add(3112)).wrapping_add(12)).wrapping_add(((((l_rule_3).wrapping_add(l_j_2)) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_mul(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                                            { let v_ = ((c.m.r32((fp.wrapping_add(8)).wrapping_add(4)) as i32)).wrapping_add(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                                            { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                                            { let v_ = 128i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                                            { let _ = F::sprintf(c, fp.wrapping_add(28), (if ((l_j_2) != 0) { sym::menu_g_c__str44 } else { sym::menu_g_c__str45 }), &[(((l_i_1).wrapping_add(1i32)) as i64 as u64)]); };
                                            { let _ = F::sprintf(c, fp.wrapping_add(92), (if ((l_j_2) != 0) { sym::menu_g_c__str46 } else { sym::menu_g_c__str47 }), &[(((l_i_1).wrapping_add(1i32)) as i64 as u64)]); };
                                            F::Flash_FindLabel(c, l_flash_4, fp.wrapping_add(28), fp.wrapping_add(92), fp);
                                            F::Flash_ClipSetUv(c, l_flash_4, fp, fp.wrapping_add(8));
                                            { let _ = F::sprintf(c, fp.wrapping_add(92), (if ((l_j_2) != 0) { sym::menu_g_c__str48 } else { sym::menu_g_c__str49 }), &[(((l_i_1).wrapping_add(1i32)) as i64 as u64)]); };
                                            F::Flash_FindLabel(c, l_flash_4, fp.wrapping_add(28), fp.wrapping_add(92), fp);
                                            F::Flash_ClipSetUv(c, l_flash_4, fp, fp.wrapping_add(8));
                                        }
                                        { let o_ = l_j_2; let n_ = (o_).wrapping_add(1); l_j_2 = n_; };
                                    }
                                }
                                break 's14;
                            }
                            _ => {
                            }
                        }
                    }
                }
                { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
            }
        }
        {
            l_i_1 = 0i32;
            'l17: loop {
                if !(((((l_i_1) < (5i32)) as i32)) != 0) { break 'l17; }
                'c18: {
                    { let a1_ = (((l_i_1) == ({ let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(272)).wrapping_add((((4i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) })) as i32); F::DuelMenu_ShowValues(c, l_i_1, a1_) };
                }
                { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
            }
        }
        { let v_ = 0i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = 256i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 512i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        F::Flash_FindLabel(c, l_flash_4, ((0i32) as u32), sym::menu_g_c__str50, fp);
        { let a3_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(384); F::FlashAnim_Scroll(c, l_flash_4, fp, fp.wrapping_add(8), a3_, ((0i32) as u32), F32(0xbe91a2b3), F32(0x00000000)) };
        {
            rt::memset(c, fp.wrapping_add(220), 0, 24);
            { let v_ = F32(0xbf9c09c0); c.m.w32(fp.wrapping_add(220).wrapping_add(0), (v_).0); }
            { let v_ = F32(0x3fb60b60); c.m.w32(fp.wrapping_add(220).wrapping_add(4), (v_).0); }
            { let v_ = F32(0x3fb60b60); c.m.w32(fp.wrapping_add(220).wrapping_add(8), (v_).0); }
            { let v_ = F32(0x3fda740d); c.m.w32(fp.wrapping_add(220).wrapping_add(12), (v_).0); }
            { let v_ = F32(0x3fb60b60); c.m.w32(fp.wrapping_add(220).wrapping_add(16), (v_).0); }
            { let v_ = F32(0x3fb60b60); c.m.w32(fp.wrapping_add(220).wrapping_add(20), (v_).0); }
            { let v_ = 256i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
            { let v_ = 512i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
            { let v_ = 0i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
            { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
            {
                l_i_1 = 0i32;
                'l19: loop {
                    if !(((((l_i_1) < (2i32)) as i32)) != 0) { break 'l19; }
                    'c20: {
                        { let _ = F::sprintf(c, fp.wrapping_add(156), sym::menu_g_c__str51, &[(((l_i_1).wrapping_add(1i32)) as i64 as u64)]); };
                        F::Flash_FindLabel(c, l_flash_4, ((0i32) as u32), fp.wrapping_add(156), fp);
                        { let a3_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(360)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); let a5_ = F32(c.m.r32((fp.wrapping_add(220)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)))); F::FlashAnim_Scroll(c, l_flash_4, fp, fp.wrapping_add(8), a3_, ((0i32) as u32), a5_, F32(0x00000000)) };
                    }
                    { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
                }
            }
            { let v_ = 512i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
            { let v_ = 256i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
            { let v_ = 0i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
            { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
            {
                l_i_1 = 2i32;
                'l21: loop {
                    if !(((((l_i_1) < (6i32)) as i32)) != 0) { break 'l21; }
                    'c22: {
                        { let _ = F::sprintf(c, fp.wrapping_add(156), sym::menu_g_c__str52, &[(((l_i_1).wrapping_add(1i32)) as i64 as u64)]); };
                        F::Flash_FindLabel(c, l_flash_4, ((0i32) as u32), fp.wrapping_add(156), fp);
                        { let a4_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(360)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); let a6_ = F32(c.m.r32((fp.wrapping_add(220)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)))); F::FlashAnim_Scroll(c, l_flash_4, fp, fp.wrapping_add(8), ((0i32) as u32), a4_, F32(0x00000000), a6_) };
                    }
                    { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
                }
            }
        }
        {
            l_i_1 = 0i32;
            'l23: loop {
                if !(((((l_i_1) < (1i32)) as i32)) != 0) { break 'l23; }
                'c24: {
                    { let a0_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(20)).wrapping_add((((l_i_1) as u32)).wrapping_mul(44)); F::Flash_Draw(c, a0_) };
                }
                { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
            }
        }
        F::IconWin_Draw(c);
        { let a2_ = { let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(300); (c.m.r32(a_) as i32) }; F::MsgWin_Draw(c, 0i32, 0i32, a2_) };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "DuelMenu_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn DuelMenu_ClipGoto(c: &mut Ctx, mut p_level_1: i32, mut p_label_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DuelMenu_ClipGoto", &PROBE_N, &[(p_level_1) as u32, (p_label_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(80) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 80);
    let mut l_flash_3: u32 = 0;
    let mut l_i_4: i32 = 0;
    let r_ = 'f: {
        l_flash_3 = ((c.m.r32(sym::gDuelMenu)).wrapping_add(20)).wrapping_add((((0i32) as u32)).wrapping_mul(44));
        's1: {
            let sv5: i32 = p_level_1;
            match sv5 {
                0i32 => {
                    { let _ = { let a2_ = ({ let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(272)).wrapping_add((((p_level_1) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(1i32); F::sprintf(c, fp.wrapping_add(8), sym::menu_g_c__str53, &[((a2_) as i64 as u64)]) }; };
                    break 's1;
                }
                1i32 => {
                    { let _ = { let a2_ = ({ let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(272)).wrapping_add((((p_level_1) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(1i32); F::sprintf(c, fp.wrapping_add(8), sym::menu_g_c__str54, &[((a2_) as i64 as u64)]) }; };
                    break 's1;
                }
                2i32 => {
                    { let _ = { let a2_ = ({ let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(272)).wrapping_add((((p_level_1) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(1i32); F::sprintf(c, fp.wrapping_add(8), sym::menu_g_c__str55, &[((a2_) as i64 as u64)]) }; };
                    break 's1;
                }
                3i32 => {
                    { let _ = { let a2_ = ({ let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(272)).wrapping_add((((p_level_1) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(1i32); F::sprintf(c, fp.wrapping_add(8), sym::menu_g_c__str56, &[((a2_) as i64 as u64)]) }; };
                    F::Flash_FindLabel(c, l_flash_3, ((0i32) as u32), fp.wrapping_add(8), fp);
                    F::Flash_ClipGotoLabel(c, l_flash_3, fp, p_label_2);
                    's2: {
                        let sv6: i32 = { let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(272)).wrapping_add((((p_level_1) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
                        match sv6 {
                            3i32 => {
                                {
                                    l_i_4 = 0i32;
                                    'l3: loop {
                                        if !(((((l_i_4) < (2i32)) as i32)) != 0) { break 'l3; }
                                        'c4: {
                                            { let _ = { let a2_ = ({ let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(272)).wrapping_add((((p_level_1) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(1i32); F::sprintf(c, fp.wrapping_add(8), (if ((l_i_4) != 0) { sym::menu_g_c__str57 } else { sym::menu_g_c__str58 }), &[((a2_) as i64 as u64)]) }; };
                                            F::Flash_FindLabel(c, l_flash_3, ((0i32) as u32), fp.wrapping_add(8), fp);
                                            F::Flash_ClipGotoLabel(c, l_flash_3, fp, p_label_2);
                                        }
                                        { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
                                    }
                                }
                                break 's2;
                            }
                            5i32 => {
                                break 's2;
                            }
                            _ => {
                                { let _ = { let a2_ = ({ let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(272)).wrapping_add((((p_level_1) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(1i32); F::sprintf(c, fp.wrapping_add(8), sym::menu_g_c__str59, &[((a2_) as i64 as u64)]) }; };
                                F::Flash_FindLabel(c, l_flash_3, ((0i32) as u32), fp.wrapping_add(8), fp);
                                F::Flash_ClipGotoLabel(c, l_flash_3, fp, p_label_2);
                                break 's2;
                            }
                        }
                    }
                    break 'f;
                }
                4i32 => {
                    's5: {
                        let sv7: i32 = { let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(272)).wrapping_add((((p_level_1) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
                        match sv7 {
                            3i32 => {
                                { let _ = { let a1_ = (if (({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(312); (c.m.r32(a_) as i32) }) != 0) { sym::menu_g_c__str60 } else { sym::menu_g_c__str61 }); let a2_ = ({ let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(272)).wrapping_add((((p_level_1) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(1i32); let a3_ = ({ let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(316)).wrapping_add(((({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(312); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(1i32); F::sprintf(c, fp.wrapping_add(8), a1_, &[((a2_) as i64 as u64), ((a3_) as i64 as u64)]) }; };
                                break 's5;
                            }
                            5i32 => {
                                break 's5;
                            }
                            _ => {
                                { let _ = { let a2_ = ({ let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(272)).wrapping_add((((p_level_1) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(1i32); let a3_ = ({ let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(316)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(1i32); F::sprintf(c, fp.wrapping_add(8), sym::menu_g_c__str62, &[((a2_) as i64 as u64), ((a3_) as i64 as u64)]) }; };
                                break 's5;
                            }
                        }
                    }
                    break 's1;
                }
                _ => {
                }
            }
        }
        F::Flash_FindLabel(c, l_flash_3, ((0i32) as u32), fp.wrapping_add(8), fp);
        F::Flash_ClipGotoLabel(c, l_flash_3, fp, p_label_2);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "DuelMenu_ClipGoto", &PROBE_N, &[], true); }
    r_
}

pub fn DuelMenu_Update(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DuelMenu_Update", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let r_ = 'f: {
    if (((({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(268); (c.m.r32(a_) as i32) }) & (16i32))) != 0) {
        if ((((((c.m.r32(((sym::gPad).wrapping_add((((1i32) as u32)).wrapping_mul(448))).wrapping_add(384)) as i32)) != (255i32)) as i32)) != 0) {
            { let r_ = 16i32; let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(268); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
        }
    } else {
        if ((((((c.m.r32(((sym::gPad).wrapping_add((((1i32) as u32)).wrapping_mul(448))).wrapping_add(384)) as i32)) == (255i32)) as i32)) != 0) {
            { let r_ = 16i32; let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(268); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        }
    }
    if (((((((({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(268); (c.m.r32(a_) as i32) }) & (8i32))) != 0) && ((((({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(300); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0)) as i32)) != 0) {
        if (((((F::Voice_GetStat(c, 0i32)) == (5i32)) as i32)) != 0) {
            { let v_ = 0i32; let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(332); c.m.w32(a_, (v_) as u32); };
            { let v_ = ({ let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(272)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(1i32); let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(300); c.m.w32(a_, (v_) as u32); };
            { let a0_ = { let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(12); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(300); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34247i32, a2_) };
        }
    }
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (1i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let a0_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(20)).wrapping_add((((l_i_1) as u32)).wrapping_mul(44)); F::Flash_Advance(c, a0_) };
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "DuelMenu_Update", &PROBE_N, &[], true); }
    r_
}

pub fn DuelMenu_Input(c: &mut Ctx, mut p_result_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DuelMenu_Input", &PROBE_N, &[(p_result_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(384) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 384);
    let r_ = 'f: {
        if ((((!(((({ let a_ = (((c.m.r32(sym::gDuelMenu)).wrapping_add(20)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add(8); c.m.r32(a_) }) & (((2i32) as u32)))) != 0)) as i32)) != 0) {
            break 'f;
        }
        if ((((!(((({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(268); (c.m.r32(a_) as i32) }) & (4i32))) != 0)) as i32)) != 0) {
            F::DuelMenu_ClipGoto(c, 0i32, sym::menu_g_c__str63);
            { let r_ = 4i32; let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(268); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        }
        's1: {
            let sv2: i32 = { let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(296); (c.m.r32(a_) as i32) };
            match sv2 {
                0i32 => {
                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((8i32) as u32)))) != 0) {
                        { let v_ = 0i32; let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(304); c.m.w32(a_, (v_) as u32); };
                        { let a0_ = { let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(296); (c.m.r32(a_) as i32) }; F::DuelMenu_ClipGoto(c, a0_, sym::menu_g_c__str64) };
                        { let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(272)).wrapping_add(((({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(296); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                        if (((({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(268); (c.m.r32(a_) as i32) }) & (16i32))) != 0) {
                            if ((((({ let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(272)).wrapping_add(((({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(296); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) == (1i32)) as i32)) != 0) {
                                { let v_ = 0i32; let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(272)).wrapping_add(((({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(296); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                            }
                        }
                        if ((((({ let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(272)).wrapping_add(((({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(296); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) < (0i32)) as i32)) != 0) {
                            { let v_ = 3i32; let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(272)).wrapping_add(((({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(296); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                        }
                        { let a0_ = { let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(296); (c.m.r32(a_) as i32) }; F::DuelMenu_ClipGoto(c, a0_, sym::menu_g_c__str65) };
                        { let v_ = 0i32; let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(332); c.m.w32(a_, (v_) as u32); };
                        { let v_ = ({ let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(272)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(1i32); let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(300); c.m.w32(a_, (v_) as u32); };
                        { let a0_ = { let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(12); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(300); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34247i32, a2_) };
                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                    } else {
                        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((4i32) as u32)))) != 0) {
                            { let v_ = 0i32; let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(304); c.m.w32(a_, (v_) as u32); };
                            { let a0_ = { let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(296); (c.m.r32(a_) as i32) }; F::DuelMenu_ClipGoto(c, a0_, sym::menu_g_c__str66) };
                            { let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(272)).wrapping_add(((({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(296); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                            if (((({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(268); (c.m.r32(a_) as i32) }) & (16i32))) != 0) {
                                if ((((({ let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(272)).wrapping_add(((({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(296); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) == (1i32)) as i32)) != 0) {
                                    { let v_ = 2i32; let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(272)).wrapping_add(((({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(296); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                }
                            }
                            if ((((({ let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(272)).wrapping_add(((({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(296); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) >= (4i32)) as i32)) != 0) {
                                { let v_ = 0i32; let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(272)).wrapping_add(((({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(296); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                            }
                            { let a0_ = { let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(296); (c.m.r32(a_) as i32) }; F::DuelMenu_ClipGoto(c, a0_, sym::menu_g_c__str67) };
                            { let v_ = 0i32; let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(332); c.m.w32(a_, (v_) as u32); };
                            { let v_ = ({ let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(272)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(1i32); let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(300); c.m.w32(a_, (v_) as u32); };
                            { let a0_ = { let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(12); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(300); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34247i32, a2_) };
                            { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                        } else {
                            if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                                { let v_ = 0i32; let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(304); c.m.w32(a_, (v_) as u32); };
                                's2: {
                                    let sv3: i32 = { let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(272)).wrapping_add(((({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(296); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
                                    'k5: {
                                    'k4: {
                                    'k3: {
                                        match sv3 { 3i32 => break 'k3, 1i32 => break 'k4, _ => break 'k5, }
                                    }
                                    {
                                        { let a0_ = { let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(296); (c.m.r32(a_) as i32) }; F::DuelMenu_ClipGoto(c, a0_, sym::menu_g_c__str68) };
                                        { let _ = { let a2_ = ({ let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(272)).wrapping_add(((({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(296); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(1i32); F::sprintf(c, fp, sym::menu_g_c__str69, &[((a2_) as i64 as u64)]) }; };
                                        { let a0_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(20)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, fp, ((((1i32) as i64 as u64)) as u8)) };
                                        { let v_ = 3i32; let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(296); c.m.w32(a_, (v_) as u32); };
                                        F::IconWin_Close(c);
                                        { let a0_ = { let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(16); c.m.r32(a_) }; F::MsgWin_SetText(c, a0_) };
                                        { let v_ = ({ let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(272)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(5i32); let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(300); c.m.w32(a_, (v_) as u32); };
                                        F::Voice_StopWithLip(c);
                                        { let a0_ = { let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(296); (c.m.r32(a_) as i32) }; F::DuelMenu_ClipGoto(c, a0_, sym::menu_g_c__str70) };
                                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                                        break 's2;
                                    }
                                    }
                                    if (((({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(268); (c.m.r32(a_) as i32) }) & (16i32))) != 0) {
                                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 7i32); };
                                        break 's2;
                                    }
                                    }
                                    {
                                        { let a0_ = { let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(296); (c.m.r32(a_) as i32) }; F::DuelMenu_ClipGoto(c, a0_, sym::menu_g_c__str71) };
                                        { let _ = { let a2_ = ({ let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(272)).wrapping_add(((({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(296); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(1i32); F::sprintf(c, fp.wrapping_add(64), sym::menu_g_c__str72, &[((a2_) as i64 as u64)]) }; };
                                        { let a0_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(20)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, fp.wrapping_add(64), ((((1i32) as i64 as u64)) as u8)) };
                                        { let v_ = 1i32; let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(296); c.m.w32(a_, (v_) as u32); };
                                        { let v_ = 0i32; let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(332); c.m.w32(a_, (v_) as u32); };
                                        { let v_ = ({ let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(272)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(5i32); let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(300); c.m.w32(a_, (v_) as u32); };
                                        { let a0_ = { let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(12); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(300); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34247i32, a2_) };
                                        { let a0_ = { let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(296); (c.m.r32(a_) as i32) }; F::DuelMenu_ClipGoto(c, a0_, sym::menu_g_c__str73) };
                                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                                        break 's2;
                                    }
                                }
                            } else {
                                if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1024i32) as u32)))) != 0) {
                                    { let v_ = 0i32; let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(304); c.m.w32(a_, (v_) as u32); };
                                    F::ColorFade_StartOut(c, 0i32, 0i32, 0i32, 20i32);
                                    { let v_ = 0i32; let a_ = p_result_1; c.m.w32(a_, (v_) as u32); };
                                    { let a0_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(20)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_g_c__str74, ((((1i32) as i64 as u64)) as u8)) };
                                    { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 2i32); };
                                } else {
                                    F::DuelMenu_Idle(c);
                                }
                            }
                        }
                    }
                    break 's1;
                }
                1i32 => {
                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((8i32) as u32)))) != 0) {
                        { let v_ = 0i32; let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(304); c.m.w32(a_, (v_) as u32); };
                        { let a0_ = { let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(296); (c.m.r32(a_) as i32) }; F::DuelMenu_ClipGoto(c, a0_, sym::menu_g_c__str75) };
                        { let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(272)).wrapping_add(((({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(296); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                        if ((((({ let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(272)).wrapping_add(((({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(296); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) < (0i32)) as i32)) != 0) {
                            { let v_ = 2i32; let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(272)).wrapping_add(((({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(296); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                        }
                        { let a0_ = { let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(296); (c.m.r32(a_) as i32) }; F::DuelMenu_ClipGoto(c, a0_, sym::menu_g_c__str76) };
                        { let v_ = 0i32; let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(332); c.m.w32(a_, (v_) as u32); };
                        { let v_ = ({ let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(272)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(5i32); let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(300); c.m.w32(a_, (v_) as u32); };
                        { let a0_ = { let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(12); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(300); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34247i32, a2_) };
                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                    } else {
                        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((4i32) as u32)))) != 0) {
                            { let v_ = 0i32; let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(304); c.m.w32(a_, (v_) as u32); };
                            { let a0_ = { let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(296); (c.m.r32(a_) as i32) }; F::DuelMenu_ClipGoto(c, a0_, sym::menu_g_c__str77) };
                            { let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(272)).wrapping_add(((({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(296); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                            if ((((({ let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(272)).wrapping_add(((({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(296); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) >= (3i32)) as i32)) != 0) {
                                { let v_ = 0i32; let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(272)).wrapping_add(((({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(296); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                            }
                            { let a0_ = { let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(296); (c.m.r32(a_) as i32) }; F::DuelMenu_ClipGoto(c, a0_, sym::menu_g_c__str78) };
                            { let v_ = 0i32; let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(332); c.m.w32(a_, (v_) as u32); };
                            { let v_ = ({ let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(272)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(5i32); let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(300); c.m.w32(a_, (v_) as u32); };
                            { let a0_ = { let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(12); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(300); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34247i32, a2_) };
                            { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                        } else {
                            if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                                { let v_ = 0i32; let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(304); c.m.w32(a_, (v_) as u32); };
                                if ((((((((({ let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(272)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) == (1i32)) as i32)) != 0) && (((({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(268); (c.m.r32(a_) as i32) }) & (16i32))) != 0)) as i32)) != 0) {
                                    { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 7i32); };
                                    break 's1;
                                }
                                { let a0_ = { let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(296); (c.m.r32(a_) as i32) }; F::DuelMenu_ClipGoto(c, a0_, sym::menu_g_c__str79) };
                                if ((((({ let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(272)).wrapping_add(((({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(296); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) == (2i32)) as i32)) != 0) {
                                    { let v_ = 2i32; let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(296); c.m.w32(a_, (v_) as u32); };
                                    { let a0_ = { let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(296); (c.m.r32(a_) as i32) }; F::DuelMenu_ClipGoto(c, a0_, sym::menu_g_c__str80) };
                                    { let _ = { let a2_ = ({ let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(272)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(1i32); F::sprintf(c, fp.wrapping_add(128), sym::menu_g_c__str81, &[((a2_) as i64 as u64)]) }; };
                                    { let a0_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(20)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, fp.wrapping_add(128), ((((1i32) as i64 as u64)) as u8)) };
                                } else {
                                    { let v_ = 1i32; let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(328); c.m.w32(a_, (v_) as u32); };
                                }
                                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                            } else {
                                if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1024i32) as u32)))) != 0) {
                                    { let v_ = 0i32; let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(304); c.m.w32(a_, (v_) as u32); };
                                    { let a0_ = { let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(296); (c.m.r32(a_) as i32) }; F::DuelMenu_ClipGoto(c, a0_, sym::menu_g_c__str82) };
                                    { let v_ = 0i32; let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(296); c.m.w32(a_, (v_) as u32); };
                                    { let a0_ = { let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(296); (c.m.r32(a_) as i32) }; F::DuelMenu_ClipGoto(c, a0_, sym::menu_g_c__str83) };
                                    { let _ = { let a2_ = ({ let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(272)).wrapping_add(((({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(296); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(1i32); F::sprintf(c, fp.wrapping_add(192), sym::menu_g_c__str84, &[((a2_) as i64 as u64)]) }; };
                                    { let a0_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(20)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, fp.wrapping_add(192), ((((1i32) as i64 as u64)) as u8)) };
                                    { let v_ = 0i32; let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(332); c.m.w32(a_, (v_) as u32); };
                                    { let v_ = ({ let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(272)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(1i32); let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(300); c.m.w32(a_, (v_) as u32); };
                                    { let a0_ = { let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(12); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(300); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34247i32, a2_) };
                                    { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 2i32); };
                                } else {
                                    F::DuelMenu_Idle(c);
                                }
                            }
                        }
                    }
                    break 's1;
                }
                2i32 => {
                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((8i32) as u32)))) != 0) {
                        { let v_ = 0i32; let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(304); c.m.w32(a_, (v_) as u32); };
                        { let a0_ = { let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(296); (c.m.r32(a_) as i32) }; F::DuelMenu_ClipGoto(c, a0_, sym::menu_g_c__str85) };
                        { let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(272)).wrapping_add(((({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(296); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                        if ((((({ let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(272)).wrapping_add(((({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(296); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) < (0i32)) as i32)) != 0) {
                            { let v_ = 2i32; let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(272)).wrapping_add(((({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(296); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                        }
                        { let a0_ = { let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(296); (c.m.r32(a_) as i32) }; F::DuelMenu_ClipGoto(c, a0_, sym::menu_g_c__str86) };
                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                    } else {
                        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((4i32) as u32)))) != 0) {
                            { let v_ = 0i32; let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(304); c.m.w32(a_, (v_) as u32); };
                            { let a0_ = { let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(296); (c.m.r32(a_) as i32) }; F::DuelMenu_ClipGoto(c, a0_, sym::menu_g_c__str87) };
                            { let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(272)).wrapping_add(((({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(296); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                            if ((((({ let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(272)).wrapping_add(((({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(296); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) >= (3i32)) as i32)) != 0) {
                                { let v_ = 0i32; let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(272)).wrapping_add(((({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(296); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                            }
                            { let a0_ = { let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(296); (c.m.r32(a_) as i32) }; F::DuelMenu_ClipGoto(c, a0_, sym::menu_g_c__str88) };
                            { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                        } else {
                            if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                                { let v_ = 0i32; let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(304); c.m.w32(a_, (v_) as u32); };
                                if ((((((((({ let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(272)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) == (1i32)) as i32)) != 0) && (((({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(268); (c.m.r32(a_) as i32) }) & (16i32))) != 0)) as i32)) != 0) {
                                    { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 7i32); };
                                    break 's1;
                                }
                                { let a0_ = { let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(296); (c.m.r32(a_) as i32) }; F::DuelMenu_ClipGoto(c, a0_, sym::menu_g_c__str89) };
                                { let v_ = 1i32; let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(328); c.m.w32(a_, (v_) as u32); };
                                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                            } else {
                                if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1024i32) as u32)))) != 0) {
                                    { let v_ = 0i32; let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(304); c.m.w32(a_, (v_) as u32); };
                                    { let a0_ = { let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(296); (c.m.r32(a_) as i32) }; F::DuelMenu_ClipGoto(c, a0_, sym::menu_g_c__str90) };
                                    { let v_ = 1i32; let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(296); c.m.w32(a_, (v_) as u32); };
                                    { let a0_ = { let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(296); (c.m.r32(a_) as i32) }; F::DuelMenu_ClipGoto(c, a0_, sym::menu_g_c__str91) };
                                    { let _ = { let a2_ = ({ let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(272)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(1i32); F::sprintf(c, fp.wrapping_add(256), sym::menu_g_c__str92, &[((a2_) as i64 as u64)]) }; };
                                    { let a0_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(20)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, fp.wrapping_add(256), ((((1i32) as i64 as u64)) as u8)) };
                                    { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 2i32); };
                                } else {
                                    F::DuelMenu_Idle(c);
                                }
                            }
                        }
                    }
                    break 's1;
                }
                3i32 => {
                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((8i32) as u32)))) != 0) {
                        { let v_ = 0i32; let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(304); c.m.w32(a_, (v_) as u32); };
                        { let a0_ = { let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(296); (c.m.r32(a_) as i32) }; F::DuelMenu_ClipGoto(c, a0_, sym::menu_g_c__str93) };
                        { let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(272)).wrapping_add(((({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(296); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                        if ((((({ let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(272)).wrapping_add(((({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(296); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) < (0i32)) as i32)) != 0) {
                            { let v_ = 5i32; let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(272)).wrapping_add(((({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(296); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                        }
                        { let a0_ = { let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(296); (c.m.r32(a_) as i32) }; F::DuelMenu_ClipGoto(c, a0_, sym::menu_g_c__str94) };
                        { let v_ = ({ let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(272)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(5i32); let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(300); c.m.w32(a_, (v_) as u32); };
                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                    } else {
                        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((4i32) as u32)))) != 0) {
                            { let v_ = 0i32; let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(304); c.m.w32(a_, (v_) as u32); };
                            { let a0_ = { let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(296); (c.m.r32(a_) as i32) }; F::DuelMenu_ClipGoto(c, a0_, sym::menu_g_c__str95) };
                            { let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(272)).wrapping_add(((({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(296); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                            if ((((({ let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(272)).wrapping_add(((({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(296); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) >= (6i32)) as i32)) != 0) {
                                { let v_ = 0i32; let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(272)).wrapping_add(((({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(296); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                            }
                            { let a0_ = { let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(296); (c.m.r32(a_) as i32) }; F::DuelMenu_ClipGoto(c, a0_, sym::menu_g_c__str96) };
                            { let v_ = ({ let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(272)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(5i32); let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(300); c.m.w32(a_, (v_) as u32); };
                            { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                        } else {
                            if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                                { let v_ = 0i32; let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(304); c.m.w32(a_, (v_) as u32); };
                                { let a0_ = { let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(296); (c.m.r32(a_) as i32) }; F::DuelMenu_ClipGoto(c, a0_, sym::menu_g_c__str97) };
                                if ((((({ let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(272)).wrapping_add(((({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(296); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) == (5i32)) as i32)) != 0) {
                                    { let v_ = 1i32; let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(388); c.m.w32(a_, (v_) as u32); };
                                } else {
                                    F::DuelMenu_SetupValue(c);
                                    { let v_ = 4i32; let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(296); c.m.w32(a_, (v_) as u32); };
                                    { let v_ = { let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(272)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(272)).wrapping_add(((({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(296); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                    { let a0_ = { let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(296); (c.m.r32(a_) as i32) }; F::DuelMenu_ClipGoto(c, a0_, sym::menu_g_c__str98) };
                                    if ((((({ let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(272)).wrapping_add(((({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(296); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) == (3i32)) as i32)) != 0) {
                                        { let r_ = 1i32; let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(312); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
                                        { let a0_ = { let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(296); (c.m.r32(a_) as i32) }; F::DuelMenu_ClipGoto(c, a0_, sym::menu_g_c__str99) };
                                        { let r_ = 1i32; let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(312); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
                                    }
                                    { let a0_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(20)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_g_c__str100, ((((1i32) as i64 as u64)) as u8)) };
                                }
                                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                            } else {
                                if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1024i32) as u32)))) != 0) {
                                    { let v_ = 0i32; let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(304); c.m.w32(a_, (v_) as u32); };
                                    { let a0_ = { let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(296); (c.m.r32(a_) as i32) }; F::DuelMenu_ClipGoto(c, a0_, sym::menu_g_c__str101) };
                                    { let v_ = 0i32; let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(296); c.m.w32(a_, (v_) as u32); };
                                    { let a0_ = { let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(296); (c.m.r32(a_) as i32) }; F::DuelMenu_ClipGoto(c, a0_, sym::menu_g_c__str102) };
                                    { let _ = { let a2_ = ({ let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(272)).wrapping_add(((({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(296); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(1i32); F::sprintf(c, fp.wrapping_add(320), sym::menu_g_c__str103, &[((a2_) as i64 as u64)]) }; };
                                    { let a0_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(20)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, fp.wrapping_add(320), ((((1i32) as i64 as u64)) as u8)) };
                                    F::IconWin_Open(c);
                                    { let a0_ = { let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(8); c.m.r32(a_) }; F::MsgWin_SetText(c, a0_) };
                                    { let v_ = 0i32; let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(332); c.m.w32(a_, (v_) as u32); };
                                    { let v_ = ({ let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(272)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(1i32); let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(300); c.m.w32(a_, (v_) as u32); };
                                    { let a0_ = { let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(12); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(300); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34247i32, a2_) };
                                    { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 2i32); };
                                }
                            }
                        }
                    }
                    break 's1;
                }
                4i32 => {
                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((8i32) as u32)))) != 0) {
                        { let v_ = 0i32; let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(304); c.m.w32(a_, (v_) as u32); };
                        { let a0_ = { let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(296); (c.m.r32(a_) as i32) }; F::DuelMenu_ClipGoto(c, a0_, sym::menu_g_c__str104) };
                        { let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(316)).wrapping_add(((({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(312); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                        if ((((({ let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(316)).wrapping_add(((({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(312); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) < (0i32)) as i32)) != 0) {
                            { let v_ = ({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(324); (c.m.r32(a_) as i32) }).wrapping_sub(1i32); let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(316)).wrapping_add(((({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(312); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                        }
                        { let a0_ = { let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(296); (c.m.r32(a_) as i32) }; F::DuelMenu_ClipGoto(c, a0_, sym::menu_g_c__str105) };
                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                    } else {
                        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((4i32) as u32)))) != 0) {
                            { let v_ = 0i32; let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(304); c.m.w32(a_, (v_) as u32); };
                            { let a0_ = { let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(296); (c.m.r32(a_) as i32) }; F::DuelMenu_ClipGoto(c, a0_, sym::menu_g_c__str106) };
                            { let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(316)).wrapping_add(((({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(312); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                            if ((((({ let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(316)).wrapping_add(((({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(312); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) >= ({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(324); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                                { let v_ = 0i32; let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(316)).wrapping_add(((({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(312); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                            }
                            { let a0_ = { let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(296); (c.m.r32(a_) as i32) }; F::DuelMenu_ClipGoto(c, a0_, sym::menu_g_c__str107) };
                            { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                        } else {
                            if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((((1i32) | (2i32))) as u32)))) != 0) {
                                { let v_ = 0i32; let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(304); c.m.w32(a_, (v_) as u32); };
                                if ((((({ let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(272)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) == (3i32)) as i32)) != 0) {
                                    { let a0_ = { let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(296); (c.m.r32(a_) as i32) }; F::DuelMenu_ClipGoto(c, a0_, sym::menu_g_c__str108) };
                                    { let r_ = 1i32; let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(312); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
                                    { let a0_ = { let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(296); (c.m.r32(a_) as i32) }; F::DuelMenu_ClipGoto(c, a0_, sym::menu_g_c__str109) };
                                    { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                                }
                            } else {
                                if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                                    { let v_ = 0i32; let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(304); c.m.w32(a_, (v_) as u32); };
                                    { let a0_ = { let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(296); (c.m.r32(a_) as i32) }; F::DuelMenu_ClipGoto(c, a0_, sym::menu_g_c__str110) };
                                    if ((((({ let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(272)).wrapping_add(((({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(296); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) == (3i32)) as i32)) != 0) {
                                        { let r_ = 1i32; let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(312); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
                                        { let a0_ = { let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(296); (c.m.r32(a_) as i32) }; F::DuelMenu_ClipGoto(c, a0_, sym::menu_g_c__str111) };
                                        { let r_ = 1i32; let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(312); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
                                        { let v_ = { let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(316)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ((c.m.r32(sym::gSaveData)).wrapping_add(3124)).wrapping_add((((({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(308); (c.m.r32(a_) as i32) }).wrapping_add(1i32)) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                    }
                                    { let v_ = { let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(316)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ((c.m.r32(sym::gSaveData)).wrapping_add(3124)).wrapping_add(((({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(308); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                    { let v_ = 3i32; let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(296); c.m.w32(a_, (v_) as u32); };
                                    { let a0_ = { let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(296); (c.m.r32(a_) as i32) }; F::DuelMenu_ClipGoto(c, a0_, sym::menu_g_c__str112) };
                                    { let a0_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(20)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_g_c__str113, ((((1i32) as i64 as u64)) as u8)) };
                                    { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                                } else {
                                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1024i32) as u32)))) != 0) {
                                        { let v_ = 0i32; let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(304); c.m.w32(a_, (v_) as u32); };
                                        { let a0_ = { let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(296); (c.m.r32(a_) as i32) }; F::DuelMenu_ClipGoto(c, a0_, sym::menu_g_c__str114) };
                                        if ((((({ let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(272)).wrapping_add(((({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(296); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) == (3i32)) as i32)) != 0) {
                                            { let r_ = 1i32; let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(312); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
                                            { let a0_ = { let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(296); (c.m.r32(a_) as i32) }; F::DuelMenu_ClipGoto(c, a0_, sym::menu_g_c__str115) };
                                            { let r_ = 1i32; let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(312); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
                                        }
                                        { let v_ = 3i32; let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(296); c.m.w32(a_, (v_) as u32); };
                                        { let a0_ = { let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(296); (c.m.r32(a_) as i32) }; F::DuelMenu_ClipGoto(c, a0_, sym::menu_g_c__str116) };
                                        { let a0_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(20)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_g_c__str117, ((((1i32) as i64 as u64)) as u8)) };
                                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 2i32); };
                                    }
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
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "DuelMenu_Input", &PROBE_N, &[], true); }
    r_
}

pub fn DuelMenu_UpdateStart(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DuelMenu_UpdateStart", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if ((((({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(328); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
        break 'f;
    }
    's1: {
        let sv1: i32 = { let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(328); (c.m.r32(a_) as i32) };
        match sv1 {
            1i32 => {
                's2: {
                    let sv2: i32 = { let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(296); (c.m.r32(a_) as i32) };
                    match sv2 {
                        1i32 => {
                            's3: {
                                let sv3: i32 = { let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(272)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
                                match sv3 {
                                    0i32 => {
                                        { let v_ = 0i32; let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(332); c.m.w32(a_, (v_) as u32); };
                                        { let v_ = 9i32; let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(300); c.m.w32(a_, (v_) as u32); };
                                        { let a0_ = { let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(12); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(300); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34247i32, a2_) };
                                        break 's3;
                                    }
                                    1i32 => {
                                        { let v_ = 1i32; let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(332); c.m.w32(a_, (v_) as u32); };
                                        { let v_ = 10i32; let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(300); c.m.w32(a_, (v_) as u32); };
                                        { let a0_ = { let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(12); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(300); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34247i32, a2_) };
                                        break 's3;
                                    }
                                    _ => {
                                    }
                                }
                            }
                            break 's2;
                        }
                        2i32 => {
                            { let v_ = 0i32; let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(332); c.m.w32(a_, (v_) as u32); };
                            { let v_ = 11i32; let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(300); c.m.w32(a_, (v_) as u32); };
                            { let a0_ = { let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(12); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(300); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34247i32, a2_) };
                            break 's2;
                        }
                        _ => {
                        }
                    }
                }
                { let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(328); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            2i32 => {
                if (((((F::Voice_GetStat(c, 0i32)) == (5i32)) as i32)) != 0) {
                    { let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(328); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                } else {
                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                        { let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(328); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                    }
                }
                break 's1;
            }
            3i32 => {
                { let r_ = 1i32; let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(268); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let r_ = 2i32; let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(268); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 15i32; let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(292); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(328); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            _ => {
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "DuelMenu_UpdateStart", &PROBE_N, &[], true); }
    r_
}

pub fn DuelMenu_UpdateReset(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DuelMenu_UpdateReset", &PROBE_N, &[], false); }
    let mut l_r_1: u32 = 0;
    let r_ = 'f: {
    l_r_1 = (c.m.r32(sym::gDuelMenu)).wrapping_add(388);
    if ((((((c.m.r32(l_r_1) as i32)) == (0i32)) as i32)) != 0) {
        break 'f;
    }
    's1: {
        let sv2: i32 = (c.m.r32(l_r_1) as i32);
        'k5: {
        'k4: {
        'k3: {
        'k2: {
            match sv2 { 1i32 => break 'k2, 2i32 => break 'k3, 3i32 => break 'k4, 4i32 => break 'k5, _ => break 's1, }
        }
        { let v_ = 2i32; let a_ = l_r_1; c.m.w32(a_, (v_) as u32); };
        F::Dialog_SetLayout(c, 0i32);
        F::Dialog_Start(c, 0i32);
        F::Dialog_SetCursor(c, 1i32);
        { let v_ = 0i32; let a_ = (l_r_1).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        }
        F::Dialog_SetChoices(c, 1i32);
        F::Dialog_Start(c, 0i32);
        { let v_ = 1i32; let a_ = (l_r_1).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        if ((((((c.m.r32((l_r_1).wrapping_add(4)) as i32)) > (0i32)) as i32)) != 0) {
            { let a_ = (l_r_1).wrapping_add(8); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
            if ((((((c.m.r32((l_r_1).wrapping_add(8)) as i32)) < (0i32)) as i32)) != 0) {
                F::Save_ResetRules(c);
                { let v_ = 120i32; let a_ = (l_r_1).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                { let v_ = 3i32; let a_ = l_r_1; c.m.w32(a_, (v_) as u32); };
            }
        } else {
            if ((((((c.m.r32((l_r_1).wrapping_add(4)) as i32)) < (0i32)) as i32)) != 0) {
                F::Dialog_SetCursor(c, 1i32);
                { let v_ = 120i32; let a_ = (l_r_1).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                { let v_ = 3i32; let a_ = l_r_1; c.m.w32(a_, (v_) as u32); };
            } else {
                { let v_ = F::Dialog_Input(c, 1i32); let a_ = (l_r_1).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                { let v_ = 12i32; let a_ = (l_r_1).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
            }
        }
        break 's1;
        }
        F::Dialog_SetChoices(c, 0i32);
        { let a0_ = { let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(296); (c.m.r32(a_) as i32) }; F::DuelMenu_ClipGoto(c, a0_, sym::menu_g_c__str118) };
        F::Dialog_Start(c, 1i32);
        { let _ = F::Dialog_Input(c, 0i32); };
        { let v_ = 4i32; let a_ = l_r_1; c.m.w32(a_, (v_) as u32); };
        break 's1;
        }
        { let _ = F::Dialog_Input(c, 0i32); };
        if ((F::Dialog_IsClosed(c)) != 0) {
            { let v_ = 0i32; let a_ = l_r_1; c.m.w32(a_, (v_) as u32); };
        }
        break 's1;
    }
    { let a0_ = (c.m.r32((l_r_1).wrapping_add(12)) as i32); F::Dialog_SetMsg(c, a0_) };
    F::Dialog_Draw(c, 1i32);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "DuelMenu_UpdateReset", &PROBE_N, &[], true); }
    r_
}

pub fn DuelMenu_Run(c: &mut Ctx, mut p_section_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DuelMenu_Run", &PROBE_N, &[(p_section_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        { let v_ = 1i32; c.m.w32(fp, (v_) as u32); }
        F::DuelMenu_Init(c, p_section_1);
        F::ColorFade_StartIn(c, 0i32, 0i32, 0i32, 20i32);
        'l1: while ((1i32) != 0) {
            F::Gfx_BeginFrame(c);
            F::Pad_Update(c);
            F::Snd_Update(c);
            F::ColorFade_Update(c);
            if ((((!(((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(20); (c.m.r32(a_) as i32) }) & (256i32))) != 0)) as i32)) != 0) {
                F::DuelMenu_Update(c);
                F::DuelMenu_UpdateStart(c);
            }
            F::DuelMenu_Draw(c);
            F::DuelMenu_UpdateReset(c);
            F::ColorFade_Draw(c);
            F::Gfx_EndFrame(c, 1i32);
            F::Dma_Flush(c);
            F::File_Stub264D90(c);
            if (((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(20); (c.m.r32(a_) as i32) }) & (256i32))) != 0) {
                continue 'l1;
            }
            if ((F::ColorFade_IsInDone(c)) != 0) {
                if ((((((((!(((({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(268); (c.m.r32(a_) as i32) }) & (8i32))) != 0)) as i32)) != 0) && (((({ let a_ = (((c.m.r32(sym::gDuelMenu)).wrapping_add(20)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add(8); c.m.r32(a_) }) & (((2i32) as u32)))) != 0)) as i32)) != 0) {
                    { let r_ = 8i32; let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(268); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    { let v_ = 0i32; let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(332); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(300); c.m.w32(a_, (v_) as u32); };
                    { let a0_ = { let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(12); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(300); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34247i32, a2_) };
                }
            }
            if ((F::ColorFade_IsFadingOut(c)) != 0) {
                F::Voice_FadeOutStep(c, 0i32);
                F::Bgm_FadeOutStep(c);
                continue 'l1;
            }
            if ((F::ColorFade_IsOutDone(c)) != 0) {
                break 'l1;
            }
            if (((({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(268); (c.m.r32(a_) as i32) }) & (2i32))) != 0) {
                if ((((({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(292); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) == ((1i32).wrapping_neg())) as i32)) != 0) {
                    F::ColorFade_StartOut(c, 0i32, 0i32, 0i32, 20i32);
                    { let v_ = { let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(272)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1568); c.m.w32(a_, (v_) as u32); };
                    { let v_ = { let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(272)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1572); c.m.w32(a_, (v_) as u32); };
                    { let v_ = { let a_ = ((c.m.r32(sym::gDuelMenu)).wrapping_add(272)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1584); c.m.w32(a_, (v_) as u32); };
                }
            } else {
                if ((((((((({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(328); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) && ((((({ let a_ = (c.m.r32(sym::gDuelMenu)).wrapping_add(388); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0)) as i32)) != 0) {
                    F::DuelMenu_Input(c, fp);
                }
            }
        }
        F::DuelMenu_Term(c);
        F::Dma_ResetBuffers(c);
        break 'f (c.m.r32(fp) as i32);
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "DuelMenu_Run", &PROBE_N, &[], true); }
    r_
}
