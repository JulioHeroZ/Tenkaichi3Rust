#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn BtlGame_Draw(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlGame_Draw", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::Hud_Draw(c);
    {
        F::Port_GsMarker(c, 19i32);
    }
    F::BtlScript_UpdateView(c);
    F::PauseMenu_Draw(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlGame_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn BtlGame_IsFighting(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlGame_IsFighting", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f F::BtlSeq_IsFighting(c);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlGame_IsFighting", &PROBE_N, &[], true); }
    r_
}

pub fn BtlGame_IsReplay(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "BtlGame_IsReplay", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f (c.m.r32(sym::gBtlGameReplayActive) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "BtlGame_IsReplay", &PROBE_N, &[], true); }
    r_
}

pub fn PauseMenu_GetMenu(c: &mut Ctx, mut p_which_1: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "PauseMenu_GetMenu", &PROBE_N, &[(p_which_1) as u32], false); }
    let mut l_menu_2: u32 = 0;
    let r_ = 'f: {
    l_menu_2 = ((0i32) as u32);
    's1: {
        let sv3: i32 = p_which_1;
        match sv3 {
            0i32 => {
                's2: {
                    let sv4: i32 = F::Battle_GetMode(c);
                    match sv4 {
                        0i32 => {
                            if ((((!((F::BattleReplay_IsActive(c)) != 0)) as i32)) != 0) {
                                l_menu_2 = sym::gPauseMenuVersus;
                            } else {
                                if ((((!((F::BattleReplay_IsLoaded(c)) != 0)) as i32)) != 0) {
                                    l_menu_2 = sym::gPauseMenuReplayRec;
                                } else {
                                    l_menu_2 = sym::gPauseMenuReplayPlay;
                                }
                            }
                            break 's2;
                        }
                        1i32 => {
                            l_menu_2 = sym::gPauseMenuMode1;
                            break 's2;
                        }
                        2i32 => {
                            l_menu_2 = sym::gPauseMenuMode2;
                            break 's2;
                        }
                        3i32 => {
                            l_menu_2 = sym::gPauseMenuMode3;
                            break 's2;
                        }
                        4i32 => {
                            l_menu_2 = sym::gPauseMenuMode4;
                            break 's2;
                        }
                        5i32 => {
                            l_menu_2 = sym::gPauseMenuMode5;
                            break 's2;
                        }
                        6i32 => {
                            l_menu_2 = sym::gPauseMenuMode6;
                            break 's2;
                        }
                        7i32 => {
                            l_menu_2 = sym::gPauseMenuMode7;
                            break 's2;
                        }
                        _ => {
                        }
                    }
                }
                break 's1;
            }
            1i32 => {
                's3: {
                    let sv5: i32 = F::Battle_GetMode(c);
                    match sv5 {
                        0i32 => {
                            if ((((!((F::BattleReplay_IsActive(c)) != 0)) as i32)) != 0) {
                                l_menu_2 = sym::gPauseMenuResultVersus;
                            } else {
                                if ((((!((F::BattleReplay_IsLoaded(c)) != 0)) as i32)) != 0) {
                                    l_menu_2 = sym::gPauseMenuResultReplayRec;
                                } else {
                                    l_menu_2 = sym::gPauseMenuResultReplayPlay;
                                }
                            }
                            break 's3;
                        }
                        1i32 => {
                            l_menu_2 = sym::gPauseMenuResultMode1;
                            break 's3;
                        }
                        _ => {
                        }
                    }
                }
                break 's1;
            }
            _ => {
            }
        }
    }
    break 'f l_menu_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "PauseMenu_GetMenu", &PROBE_N, &[], true); }
    r_
}

pub fn PauseMenu_ConfirmFunc(c: &mut Ctx, mut p_ev_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "PauseMenu_ConfirmFunc", &PROBE_N, &[(p_ev_1) as u32], false); }
    let mut l_menu_2: u32 = 0;
    let r_ = 'f: {
    if (((c.m.r32((p_ev_1).wrapping_add(16)) as i32)) != 0) {
        l_menu_2 = { let a0_ = (c.m.r32(sym::gPauseMenuWhich) as i32); F::PauseMenu_GetMenu(c, a0_) };
        F::BtlMenu_SetValue(c, l_menu_2, 1i32, 0i32);
        F::BtlMenu_SetValue(c, l_menu_2, 0i32, 0i32);
        { let v_ = 2i32; let a_ = (sym::gPauseMenuConfirmCursor).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "PauseMenu_ConfirmFunc", &PROBE_N, &[], true); }
    r_
}

pub fn PauseMenu_SkillListFunc(c: &mut Ctx, mut p_ev_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "PauseMenu_SkillListFunc", &PROBE_N, &[(p_ev_1) as u32], false); }
    let mut l_work_2: u32 = 0;
    let mut l_list_3: u32 = 0;
    let mut l_pad_4: u32 = 0;
    let mut l_last_5: i32 = 0;
    let mut l_page_6: i32 = 0;
    let mut l_move_7: i32 = 0;
    let mut l_max_8: i32 = 0;
    let mut l_scroll_9: i32 = 0;
    let mut l_cursor_10: i32 = 0;
    let mut l_old_11: i32 = 0;
    let mut l_lim_12: i32 = 0;
    let r_ = 'f: {
    l_work_2 = F::BtlMenu_GetWork(c);
    if ((((({ let a_ = (c.m.r32(p_ev_1)).wrapping_add(28); (c.m.r32(a_) as i32) }) == (4i32)) as i32)) != 0) {
        l_list_3 = ((l_work_2).wrapping_add(32)).wrapping_add(((((c.m.r32((l_work_2).wrapping_add(12)) as i32)) as u32)).wrapping_mul(128));
        l_pad_4 = { let a_ = ((sym::gPad).wrapping_add(((((c.m.r32((l_work_2).wrapping_add(12)) as i32)) as u32)).wrapping_mul(448))).wrapping_add(400); c.m.r32(a_) };
        l_last_5 = ((c.m.r32(l_list_3) as i32)).wrapping_sub(1i32);
        l_page_6 = (c.m.r32((l_list_3).wrapping_add(44)) as i32);
        if ((((l_pad_4) & (((2i32) as u32)))) != 0) {
            { let o_ = l_page_6; let n_ = (o_).wrapping_add(1); l_page_6 = n_; };
            { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
        }
        if ((((l_pad_4) & (((1i32) as u32)))) != 0) {
            { let o_ = l_page_6; let n_ = (o_).wrapping_sub(1); l_page_6 = n_; };
            { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
        }
        if (((((l_page_6) > (l_last_5)) as i32)) != 0) {
            l_page_6 = 0i32;
        }
        if (((((l_page_6) < (0i32)) as i32)) != 0) {
            l_page_6 = l_last_5;
        }
        l_move_7 = 0i32;
        if ((((l_pad_4) & (((8i32) as u32)))) != 0) {
            l_move_7 = (1i32).wrapping_neg();
        }
        if ((((l_pad_4) & (((4i32) as u32)))) != 0) {
            { let o_ = l_move_7; let n_ = (o_).wrapping_add(1); l_move_7 = n_; };
        }
        l_scroll_9 = (c.m.r32(((l_list_3).wrapping_add(48)).wrapping_add((((l_page_6) as u32)).wrapping_mul(4))) as i32);
        l_cursor_10 = (c.m.r32(((l_list_3).wrapping_add(88)).wrapping_add((((l_page_6) as u32)).wrapping_mul(4))) as i32);
        l_max_8 = ((c.m.r32(((l_list_3).wrapping_add(4)).wrapping_add((((l_page_6) as u32)).wrapping_mul(4))) as i32)).wrapping_sub(1i32);
        l_lim_12 = (l_scroll_9).wrapping_add(6i32);
        l_old_11 = l_cursor_10;
        if (((((l_move_7) != (0i32)) as i32)) != 0) {
            if (((((l_move_7) < (0i32)) as i32)) != 0) {
                if (((((l_cursor_10) != (0i32)) as i32)) != 0) {
                    { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                }
            } else {
                if (((((l_cursor_10) != (l_max_8)) as i32)) != 0) {
                    { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                }
            }
            { let r_ = l_move_7; let n_ = (l_cursor_10).wrapping_add(r_); l_cursor_10 = n_; };
        }
        if (((((l_cursor_10) > (l_max_8)) as i32)) != 0) {
            l_cursor_10 = l_max_8;
        }
        if (((((l_cursor_10) < (0i32)) as i32)) != 0) {
            l_cursor_10 = 0i32;
        }
        if (((((l_cursor_10) < (l_scroll_9)) as i32)) != 0) {
            l_scroll_9 = l_cursor_10;
        }
        if (((((l_cursor_10) > (l_lim_12)) as i32)) != 0) {
            l_scroll_9 = (l_cursor_10).wrapping_sub(6i32);
        }
        { let v_ = l_scroll_9; let a_ = ((l_list_3).wrapping_add(48)).wrapping_add((((l_page_6) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = l_cursor_10; let a_ = ((l_list_3).wrapping_add(88)).wrapping_add((((l_page_6) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = l_page_6; let a_ = (l_list_3).wrapping_add(44); c.m.w32(a_, (v_) as u32); };
        if (((((l_cursor_10) != (l_old_11)) as i32)) != 0) {
            F::FontIcon_ResetAnim(c);
        }
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "PauseMenu_SkillListFunc", &PROBE_N, &[], true); }
    r_
}

pub fn PauseMenu_CpuLevelFunc(c: &mut Ctx, mut p_ev_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "PauseMenu_CpuLevelFunc", &PROBE_N, &[(p_ev_1) as u32], false); }
    let mut l_level_2: i32 = 0;
    let mut l_menu_3: u32 = 0;
    let mut l_menu_4: u32 = 0;
    let mut l_cursor_5: i32 = 0;
    let r_ = 'f: {
    if (((c.m.r32((p_ev_1).wrapping_add(16)) as i32)) != 0) {
        l_menu_3 = { let a0_ = (c.m.r32(sym::gPauseMenuWhich) as i32); F::PauseMenu_GetMenu(c, a0_) };
        F::BtlMenu_SetValue(c, l_menu_3, 2i32, 0i32);
        F::BtlMenu_SetValue(c, l_menu_3, 3i32, 0i32);
        F::BtlMenu_SetValue(c, l_menu_3, 4i32, 0i32);
        F::BtlMenu_SetValue(c, l_menu_3, 5i32, 0i32);
        F::BtlMenu_SetValue(c, l_menu_3, 6i32, 0i32);
        F::BtlMenu_SetValue(c, l_menu_3, 7i32, 0i32);
        F::BtlMenu_SetPicked(c, l_menu_3, 2i32, 0i32);
        F::BtlMenu_SetPicked(c, l_menu_3, 3i32, 0i32);
        F::BtlMenu_SetPicked(c, l_menu_3, 4i32, 0i32);
        F::BtlMenu_SetPicked(c, l_menu_3, 5i32, 0i32);
        F::BtlMenu_SetPicked(c, l_menu_3, 6i32, 0i32);
        F::BtlMenu_SetPicked(c, l_menu_3, 7i32, 0i32);
        l_level_2 = { let a0_ = F::BtlAiMgr_GetLevel(c, 1i32); F::CpuLevel_ToSetting(c, a0_) };
        if (((((l_level_2) == ((1i32).wrapping_neg())) as i32)) != 0) {
            { let v_ = 1i32; let a_ = (sym::gPauseMenuCpuLevelCursor).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        } else {
            { let v_ = (l_level_2).wrapping_add(2i32); let a_ = (sym::gPauseMenuCpuLevelCursor).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        }
    }
    if ((((({ let a_ = (c.m.r32(p_ev_1)).wrapping_add(28); (c.m.r32(a_) as i32) }) == (4i32)) as i32)) != 0) {
        l_menu_4 = { let a0_ = (c.m.r32(sym::gPauseMenuWhich) as i32); F::PauseMenu_GetMenu(c, a0_) };
        { let _ = F::BtlMenu_GetPicked(c, l_menu_4, 2i32); };
        { let _ = F::BtlMenu_GetPicked(c, l_menu_4, 3i32); };
        { let _ = F::BtlMenu_GetPicked(c, l_menu_4, 4i32); };
        { let _ = F::BtlMenu_GetPicked(c, l_menu_4, 5i32); };
        { let _ = F::BtlMenu_GetPicked(c, l_menu_4, 6i32); };
        { let _ = F::BtlMenu_GetPicked(c, l_menu_4, 7i32); };
        l_cursor_5 = (c.m.r32((sym::gPauseMenuCpuLevelCursor).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32);
        if (((((l_cursor_5) == (1i32)) as i32)) != 0) {
            l_level_2 = (1i32).wrapping_neg();
        } else {
            l_level_2 = F::CpuLevel_FromSetting(c, (((((l_cursor_5).wrapping_sub(2i32)) as i64 as u64)) as u32));
        }
        F::BtlAiMgr_SetLevel(c, 1i32, l_level_2);
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "PauseMenu_CpuLevelFunc", &PROBE_N, &[], true); }
    r_
}

pub fn PauseMenu_Reset(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "PauseMenu_Reset", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let a0_ = F::PauseMenu_GetMenu(c, 0i32); F::BtlMenu_Reset(c, a0_) };
    { let a0_ = F::PauseMenu_GetMenu(c, 1i32); F::BtlMenu_Reset(c, a0_) };
    { let v_ = 0i32; let a_ = sym::gPauseMenuWhich; c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "PauseMenu_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn PauseMenu_Init(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "PauseMenu_Init", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::BtlMenu_Init(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "PauseMenu_Init", &PROBE_N, &[], true); }
    r_
}

pub fn PauseMenu_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "PauseMenu_Term", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::BtlMenu_Term(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "PauseMenu_Term", &PROBE_N, &[], true); }
    r_
}

pub fn PauseMenu_Update(c: &mut Ctx, mut p_pad_1: i32, mut p_which_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "PauseMenu_Update", &PROBE_N, &[(p_pad_1) as u32, (p_which_2) as u32], false); }
    let mut l_set_3: u32 = 0;
    let mut l_menu_4: u32 = 0;
    let mut l_item_5: u32 = 0;
    let r_ = 'f: {
    { let v_ = p_which_2; let a_ = sym::gPauseMenuWhich; c.m.w32(a_, (v_) as u32); };
    l_menu_4 = F::PauseMenu_GetMenu(c, p_which_2);
    { let a0_ = { let a_ = ({ let a0_ = F::BattleSide_GetObjId(c, p_pad_1); F::BtlObj_Get(c, a0_) }).wrapping_add(188); c.m.r32(a_) }; F::BtlMenu_SetScript(c, a0_) };
    l_set_3 = fid::BattleResult_Set;
    if ((F::BtlMenu_Update(c, p_which_2, ((0i32) as u32), l_menu_4, p_pad_1)) != 0) {
        { let r_ = (((!(256i32))) as u64); let a_ = (F::Battle_GetWork(c)).wrapping_add(6640); let n_ = ((c.m.r64(a_)) & (r_)); c.m.w64(a_, n_); };
        F::BtlMenu_ClearActive(c);
        F::Adx_ResumeSeVoice(c);
        break 'f 0i32;
    }
    F::BtlMenu_SetTop(c, l_menu_4);
    l_item_5 = F::BtlMenu_FindResult(c, ((0i32) as u32), l_menu_4);
    if (((((l_item_5) != (((0i32) as u32))) as i32)) != 0) {
        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
        's1: {
            let sv6: i32 = (c.m.r32((l_item_5).wrapping_add(4)) as i32);
            match sv6 {
                12i32 => {
                    { let _ = D::call(c, l_set_3, &[((8i32) as i64 as u64), ((32768i32) as i64 as u64)]); };
                    break 's1;
                }
                19i32 => {
                    { let _ = D::call(c, l_set_3, &[((8i32) as i64 as u64), ((32768i32) as i64 as u64)]); };
                    break 's1;
                }
                13i32 => {
                    { let _ = D::call(c, l_set_3, &[((8i32) as i64 as u64), ((16i32) as i64 as u64)]); };
                    break 's1;
                }
                14i32 => {
                    { let _ = D::call(c, l_set_3, &[((8i32) as i64 as u64), ((128i32) as i64 as u64)]); };
                    break 's1;
                }
                15i32 => {
                    { let _ = D::call(c, l_set_3, &[((8i32) as i64 as u64), ((8i32) as i64 as u64)]); };
                    break 's1;
                }
                16i32 => {
                    { let _ = D::call(c, l_set_3, &[((8i32) as i64 as u64), ((256i32) as i64 as u64)]); };
                    break 's1;
                }
                18i32 => {
                    { let _ = D::call(c, l_set_3, &[((8i32) as i64 as u64), ((512i32) as i64 as u64)]); };
                    break 's1;
                }
                17i32 => {
                    { let _ = D::call(c, l_set_3, &[((8i32) as i64 as u64), ((1024i32) as i64 as u64)]); };
                    break 's1;
                }
                20i32 => {
                    if (((((p_pad_1) == (0i32)) as i32)) != 0) {
                        { let _ = D::call(c, l_set_3, &[((8i32) as i64 as u64), ((32i32) as i64 as u64)]); };
                    } else {
                        { let _ = D::call(c, l_set_3, &[((8i32) as i64 as u64), ((64i32) as i64 as u64)]); };
                    }
                    break 's1;
                }
                23i32 => {
                    { let _ = D::call(c, l_set_3, &[((8i32) as i64 as u64), ((4096i32) as i64 as u64)]); };
                    break 's1;
                }
                24i32 => {
                    { let _ = D::call(c, l_set_3, &[((8i32) as i64 as u64), ((65536i32) as i64 as u64)]); };
                    break 's1;
                }
                25i32 => {
                    { let _ = D::call(c, l_set_3, &[((8i32) as i64 as u64), ((65536i32) as i64 as u64)]); };
                    break 's1;
                }
                26i32 => {
                    { let _ = D::call(c, l_set_3, &[((8i32) as i64 as u64), ((32i32) as i64 as u64)]); };
                    break 's1;
                }
                21i32 => {
                    { let _ = D::call(c, l_set_3, &[((8i32) as i64 as u64), ((32i32) as i64 as u64)]); };
                    break 's1;
                }
                22i32 => {
                    break 's1;
                }
                _ => {
                }
            }
        }
    }
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "PauseMenu_Update", &PROBE_N, &[], true); }
    r_
}

pub fn PauseMenu_Draw(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "PauseMenu_Draw", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::BtlMenu_DrawAll(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "PauseMenu_Draw", &PROBE_N, &[], true); }
    r_
}
